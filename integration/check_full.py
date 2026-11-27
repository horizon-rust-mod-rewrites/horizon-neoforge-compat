#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Full pinned Horizon + real companion check in a NEW disposable scratch tree.

This is deliberately not the dependency-free endpoint checker. It needs the
complete public source, native build dependencies and several GiB of free disk.
Never edits the supplied Horizon or compatibility sources. Keeps its scratch
outputs/logs on failure; never calls git commit/push or starts a game/server.
"""
import argparse
import hashlib
import json
import math
import os
import re
from pathlib import Path
import shutil
import subprocess
import sys
import tomllib

from check import verify

HERE = Path(__file__).resolve().parent
COMPAT = HERE.parent


def disk_preflight(parent, minimum_gib, source_bytes=0):
    """Read-only lower-bound check; not a promise that a full build will fit."""
    if not math.isfinite(minimum_gib) or minimum_gib < 0.1:
        raise RuntimeError('minimum build reservation must be finite and at least 0.1 GiB')
    free = shutil.disk_usage(parent).free
    reservation = int(minimum_gib * 1024**3)
    print(f'Disk preflight: {free:,} free bytes; reservation {reservation:,} bytes; '
          f'additional source copy {source_bytes:,} bytes', flush=True)
    if free < source_bytes + reservation:
        raise RuntimeError('full build NOT STARTED: insufficient disk '
                           f'(reservation {reservation:,} bytes + source copy '
                           f'{source_bytes:,} bytes); do not download large build inputs here')


def tracked_sources(source):
    """Verify every materialized blob against the pinned Git tree, without fetches."""
    raw = subprocess.check_output(['git', '-C', str(source), 'ls-tree', '-rz', 'HEAD'])
    files, missing = [], []
    for entry in raw.split(b'\0'):
        if not entry:
            continue
        meta, raw_name = entry.split(b'\t', 1)
        mode, kind, digest = meta.decode().split()
        name = os.fsdecode(raw_name)
        if kind != 'blob':
            raise RuntimeError(f'unsupported gitlink/tree entry: {name}')
        path = source / name
        if not path.exists() and not path.is_symlink():
            missing.append(name)
            continue
        data = os.fsencode(os.readlink(path)) if mode == '120000' else path.read_bytes()
        actual = hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()
        if actual != digest:
            raise RuntimeError(f'upstream working blob differs from pinned tree: {name}')
        files.append((name, mode, len(data)))
    return files, missing


def wire_companion(scratch):
    """Compile the actual companion as a module inside horizon-client-lib."""
    companion = scratch / '_neoforge_companion'
    companion.mkdir()
    for name in ['compat_bridge.rs', 'bridge_contract_tests.rs']:
        shutil.copyfile(HERE / name, companion / name)
    manifest = scratch / 'crates/horizon-client-lib/Cargo.toml'
    text = manifest.read_text()
    if text.count('[dependencies]\n') != 1:
        raise RuntimeError('unexpected pinned client manifest')
    text = text.replace('[dependencies]\n', '[dependencies]\nhorizon-neoforge-compat = { path = '
                        + json.dumps(str(COMPAT)) + ' }\n', 1)
    manifest.write_text(text)
    root = scratch / 'crates/horizon-client/src/client_lib.rs'
    with root.open('a') as out:
        out.write('\n#[path = ' + json.dumps(str(companion / 'compat_bridge.rs'))
                  + ']\npub mod neoforge_compat_bridge;\n')


def check_lock(before, after):
    """Adding the local dependency must not change any existing pinned dependency."""
    if ({k: v for k, v in before.items() if k != 'package'}
            != {k: v for k, v in after.items() if k != 'package'}):
        raise RuntimeError('companion changed pinned lock metadata')

    def indexed(packages):
        entries = {}
        for package in packages:
            key = package['name'] + '@' + package['version'] + '@' + package.get('source', '')
            if key in entries:
                raise RuntimeError('duplicate pinned lock identity: ' + key)
            entries[key] = package
        return entries

    old = indexed(before['package'])
    new = indexed(after['package'])
    for key, original in old.items():
        expected = dict(original)
        if original['name'] == 'horizon-client-lib':
            expected['dependencies'] = sorted(original['dependencies'] + ['horizon-neoforge-compat'])
        if new.get(key) != expected:
            raise RuntimeError('companion changed an existing pinned lock entry: ' + key)
    added = [p for key, p in new.items() if key not in old]
    core = tomllib.loads((COMPAT / 'Cargo.toml').read_text())['package']
    if added != [{'name': core['name'], 'version': core['version']}]:
        raise RuntimeError('unexpected added lock entries (expected dependency-free local core only)')


def verify_test_summary(output, test_filter, minimum_passed):
    """Fail closed on empty or unexpectedly shrunken selected regression suites."""
    counts = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', output)
    if (not counts or any(int(failed) for _, failed, _ in counts)
            or sum(int(passed) for passed, _, _ in counts) < minimum_passed):
        raise RuntimeError('test filter selected fewer passing tests than required '
                           f'({minimum_passed}); not validation: {test_filter}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path, nargs='?',
                        help='complete clean pinned checkout; required except for disk-only preflight')
    parser.add_argument('--scratch', type=Path, required=True, help='new directory; retained, not cleaned')
    parser.add_argument('--target', help='optional Cargo target; host toolchain still required')
    parser.add_argument('--minimum-build-free-gib', type=float, default=3,
                        help='disk reservation after source copy, not a measured build-size promise')
    parser.add_argument('--disk-preflight-only', action='store_true',
                        help='check scratch filesystem before downloading inputs; no source/build validation')
    args = parser.parse_args()
    if args.source is None and not args.disk_preflight_only:
        parser.error('source is required unless --disk-preflight-only is used')
    scratch = args.scratch.resolve()
    if scratch.exists() or not scratch.parent.is_dir():
        raise RuntimeError('scratch must not exist; its parent must already exist')
    # Cheap refusal before source hashing, source retrieval by the caller, or Cargo.
    # Repeat with the real copy size below; the first check has no source estimate.
    disk_preflight(scratch.parent, args.minimum_build_free_gib)
    if args.disk_preflight_only:
        print('Disk-only preflight passed lower bound. NO source, toolchain, dependency, '
              'compilation, engine/TCP or join checks executed; build fit is NOT guaranteed.')
        return
    source = args.source.resolve()
    if source == scratch or source in scratch.parents or scratch in source.parents:
        raise RuntimeError('scratch and source must be separate trees')
    verify(source)
    files, missing = tracked_sources(source)
    size = sum(size for _, _, size in files)
    print(f'Preflight: {len(files)} verified source blobs, {len(missing)} missing; '
          f'{size:,} materialized source bytes', flush=True)
    if missing:
        raise RuntimeError('full build NOT STARTED: incomplete source; '
                           f'first missing paths: {missing[:3]}')
    disk_preflight(scratch.parent, args.minimum_build_free_gib, size)
    scratch.mkdir()
    for name, mode, _ in files:
        src, dst = source / name, scratch / name
        dst.parent.mkdir(parents=True, exist_ok=True)
        if mode == '120000':
            dst.symlink_to(os.readlink(src))
        else:
            shutil.copy2(src, dst)
    subprocess.run(['git', 'apply', '--check', str(HERE / 'horizon-early-configuration.patch')],
                   cwd=scratch, check=True)
    subprocess.run(['git', 'apply', str(HERE / 'horizon-early-configuration.patch')],
                   cwd=scratch, check=True)
    env = os.environ.copy()
    for key in ['HORIZON_HOME', 'HORIZON_LAUNCHER_HOME', 'XDG_DATA_HOME', 'XDG_CONFIG_HOME',
                'HORIZON_LAUNCHER_TRASH', 'HORIZON_LAUNCHER_CLIPBOARD',
                'HORIZON_LAUNCHER_DESKTOP', 'TMPDIR']:
        path = scratch / '_runtime' / key.lower()
        path.mkdir(parents=True, exist_ok=True)
        env[key] = str(path)
    env.update(HORIZON_LAUNCHER_NO_SELF_UPDATE='1', HORIZON_NO_AUDIO='1',
               CARGO_TARGET_DIR=str(scratch / 'target'), CARGO_INCREMENTAL='0',
               CARGO_PROFILE_DEV_DEBUG='0', CARGO_PROFILE_TEST_DEBUG='0')
    env.setdefault('CARGO_HOME', str(scratch / '_cargo_home'))
    cargo = ['cargo']
    target = ['--target', args.target] if args.target else []

    def execute(arguments, minimum_passed=1):
        command = cargo + arguments[:1] + target + arguments[1:]
        print('+ ' + ' '.join(command), flush=True)
        logfile = scratch / 'full-check.log'
        with logfile.open('a') as log:
            log.write('+ ' + ' '.join(command) + '\n')
            log.flush()
            start = log.tell()
            subprocess.run(command, cwd=scratch, env=env, stdout=log,
                           stderr=subprocess.STDOUT, check=True)
        if arguments[0] == 'test':
            with logfile.open() as log:
                log.seek(start)
                output = log.read()
            verify_test_summary(output, arguments[-1], minimum_passed)

    # Establish the exact upstream locked graph with the patch before adding glue.
    execute(['check', '--locked', '-p', 'horizon-client-lib', '--no-default-features'])
    before = tomllib.loads((scratch / 'Cargo.lock').read_text())
    wire_companion(scratch)
    # Deliberate lock update ONLY for the new dependency-free local package.
    # Offline resolution prevents opportunistic updates; then audit every entry.
    execute(['check', '--offline', '-p', 'horizon-client-lib', '--no-default-features'])
    check_lock(before, tomllib.loads((scratch / 'Cargo.lock').read_text()))
    for package, test_filter, minimum_passed in [
        ('horizon-client-lib', 'neoforge_compat_bridge::tests::', 9),
        ('horizon-client-lib', 'early_configuration', 9),
        ('horizon-protocol', 'neoforge_configuration_framing_tests', 1),
        ('horizon-client-lib', 'net::tests::', 81),
    ]:
        execute(['test', '--locked', '--offline', '-p', package, '--no-default-features',
                 '--lib', test_filter], minimum_passed)
    verify(source)
    print('PASS: full patched client + actual companion typecheck, engine/TCP/vanilla-Horizon tests. '
          'NOT a NeoForge server join or semantic content validation.')


if __name__ == '__main__':
    try:
        main()
    except (RuntimeError, subprocess.CalledProcessError) as error:
        print(str(error), file=sys.stderr)
        sys.exit(1)
