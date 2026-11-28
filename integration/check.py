#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Verify the pin, apply in isolation, check structure, run dependency-free Rust tests.
Never writes the supplied source checkout. Full engine tests are a separate step.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
REPO = HERE.parent
PATCH = HERE / 'horizon-early-configuration.patch'
NET = 'crates/horizon-client/src/net.rs'
EARLY = 'crates/horizon-client/src/net/early_configuration.rs'
TABLE = 'crates/horizon-protocol/src/adapter/v1_21_1.rs'


def run(args, cwd=None, success=True):
    result = subprocess.run(list(map(str, args)), cwd=cwd, text=True,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    if success and result.returncode:
        raise RuntimeError(f'{args}:\n{result.stdout}')
    return result


def verify(source):
    pin = json.loads((REPO / 'provenance/horizon.json').read_text())
    assert run(['git', '-C', source, 'rev-parse', 'HEAD']).stdout.strip() == pin['commit']
    for name, expected in pin['read_files'].items():
        assert hashlib.sha256((source / name).read_bytes()).hexdigest() == expected, name
    # Files used by this patch beyond the previous audit's manifest.
    extra = json.loads((HERE / 'source-hashes.json').read_text())
    for name, expected in extra.items():
        assert hashlib.sha256((source / name).read_bytes()).hexdigest() == expected, name
    print(f'PASS: HEAD {pin["commit"]}, {len(pin["read_files"])} provenance hashes + {len(extra)} integration hashes')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path)
    parser.add_argument('--rustc', default=shutil.which('rustc'))
    parser.add_argument('--target')
    parser.add_argument('--linker')
    args = parser.parse_args()
    source = args.source.resolve()
    verify(source)
    assert args.rustc, 'supply --rustc or install Rust'
    with tempfile.TemporaryDirectory(prefix='horizon-integration-') as directory:
        root = Path(directory)
        # Only existing files required for application/testing, not a modified shared tree.
        patch_files = [line.split(' b/', 1)[1] for line in PATCH.read_text().splitlines()
                       if line.startswith('diff --git ')]
        for name in set(patch_files + [TABLE]):
            if (source / name).exists():
                (root / name).parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(source / name, root / name)
        run(['git', 'apply', '--check', PATCH], root)
        run(['git', 'apply', PATCH], root)
        assert run(['git', 'apply', '--check', PATCH], root, False).returncode != 0
        run(['git', 'apply', '--reverse', '--check', PATCH], root)
        print('PASS: clean application, reverse check, duplicate application refused')
        net = (root / NET).read_text()
        early = (root / EARLY).read_text()
        class Structure(unittest.TestCase):
            # These are deliberately external patch/ordering contract checks, not
            # substitutes for the included (full-dependency) engine runtime tests.
            def test_route_precedes_horizon_special_cases_and_generic_dispatch(self):
                body = net[net.index('fn configure_with_profile('):]
                positions = [body.index(x) for x in [
                    'ordered_owner::OrderedOwner::new(epoch, profile, None)',
                    'negotiation.initial()',
                    'cfg::Clientbound::CustomPayload(p) if owner.is_some()',
                    'p.channel == crate::build_mismatch::CHANNEL',
                    'let Some(p) = receive_application(']]
                self.assertEqual(positions, sorted(positions))
            def test_opt_in_and_offer_suppression(self):
                self.assertIn('if early_profile.is_none()\n        && payloads.local.has_mod_declarations()', net)
                self.assertIn('early_profile: Option<early_configuration::Profile>', net)
                self.assertIn('c.adapt(horizon_protocol::adapter::ServerVersion::V1_21_1)', net)
                branch = net.split('if owner.is_some() => {', 1)[1].split('p.channel == crate::build_mismatch::CHANNEL')[0]
                self.assertNotIn('send_control(', branch)
                self.assertNotIn('receive_application(', branch)
            def test_finish_is_before_horizon_accept_and_writer_transition(self):
                body = net.split('cfg::Clientbound::FinishConfiguration(_) => {', 1)[1]
                self.assertLess(body.index('owner.finish()'), body.index('negotiation.finish_allowing_silent_peer()'))
                self.assertLess(body.index('owner.finish()'), body.index('w.transition_protocol('))
                self.assertIn('return match owner.finish()', body)
                self.assertIn('modded_play_boundary_not_implemented', early)
                self.assertIn('self.bridge.reset()', early)
            def test_default_reconfiguration_still_uses_original_path(self):
                body = net.split('play::Clientbound::StartConfiguration(_) => {', 1)[1]
                self.assertIn('let registries = configure(', body)
                self.assertIn('resources,\n        payloads,\n        initial,\n        None,', net)
        suite = unittest.defaultTestLoader.loadTestsFromTestCase(Structure)
        assert unittest.TextTestRunner(verbosity=2).run(suite).wasSuccessful()
        flags = []
        if args.target:
            flags += ['--target', args.target]
        if args.linker:
            flags += ['-C', 'linker=' + args.linker]
        print(run([args.rustc, '--version']).stdout.strip())
        executable = root / 'endpoint-tests'
        run([args.rustc, '--edition', '2021', '--test', root / EARLY, *flags, '-o', executable])
        print(run([executable]).stdout)
        # Compile the ACTUAL pinned generated packet table, not a Python model.
        table_test = root / 'table_test.rs'
        table_test.write_text('''#[derive(Clone, Copy)]
pub enum State { Handshake, Status, Login, Configuration, Play }
#[path = "crates/horizon-protocol/src/adapter/v1_21_1.rs"] mod table;
#[test] fn directions_767() {
    assert_eq!(table::PROTOCOL, 767);
    assert_eq!(table::NAME, "1.21.1");
    assert_eq!(table::clientbound(State::Configuration)[1], Some(1));
    assert_eq!(table::serverbound(State::Configuration)[2], Some(2));
    assert_eq!(table::clientbound(State::Configuration)[14], Some(15));
    assert_eq!(table::serverbound(State::Configuration)[7], Some(7));
}
''')
        run([args.rustc, '--edition', '2021', '--test', table_test, *flags, '-o', executable])
        print(run([executable]).stdout)
        print('NOT EXECUTED here: full Horizon configure/queued-writer and TCP framing tests; see README.')
        run(['git', 'apply', '--reverse', PATCH], root)
        for name in patch_files:
            if (source / name).exists():
                assert (source / name).read_bytes() == (root / name).read_bytes(), name
        print('PASS: exact restoration after reverse application')
    verify(source)


if __name__ == '__main__':
    main()
