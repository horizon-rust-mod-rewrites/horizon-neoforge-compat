#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Compile actual core + companion glue + patched endpoint together in isolation.
This is not full Horizon compilation, network framing, or live join validation.
Run after integration/check.py so source pin/application checks have passed.
"""
import argparse
import os
from pathlib import Path
import subprocess
import tempfile

HERE = Path(__file__).resolve().parent
REPO = HERE.parent

def run(command, cwd=None):
    subprocess.run(list(map(str, command)), cwd=cwd, check=True)

def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--target', default='x86_64-unknown-linux-musl')
    args = p.parse_args()
    run(['cargo', 'build', '--offline', '--locked', '--lib', '--target', args.target], REPO)
    (REPO / 'target').mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='compat-bridge-', dir=REPO / 'target') as temp:
        root = Path(temp)
        patch = (HERE / 'horizon-early-configuration.patch').read_text()
        name = 'crates/horizon-client/src/net/early_configuration.rs'
        section = patch.split('diff --git a/' + name + ' b/' + name + '\n', 1)[1].split('diff --git ', 1)[0]
        lines = [line[1:] for line in section.splitlines() if line.startswith('+') and not line.startswith('+++')]
        # Test module file is provided too; rustc --test compiles the endpoint's own tests.
        for filename in ['early_configuration_tests.rs']:
            path = 'crates/horizon-client/src/net/' + filename
            s = patch.split('diff --git a/' + path + ' b/' + path + '\n', 1)[1].split('diff --git ', 1)[0]
            (root / filename).write_text('\n'.join(line[1:] for line in s.splitlines() if line.startswith('+') and not line.startswith('+++')) + '\n')
        (root / 'early_configuration.rs').write_text('\n'.join(lines) + '\n')
        source = '''extern crate horizon_neoforge_compat;
pub mod net { pub mod early_configuration { include!(concat!(env!("BRIDGE_TEST_ROOT"), "/endpoint_include.rs")); } }
#[path = "COMPANION"] mod compat_bridge;
use net::early_configuration::*;
use std::sync::Arc;
fn profile() -> Profile { Profile::neoforge("1.21.1",767,"21.1.248",[7;32],vec![],Arc::new(compat_bridge::CompatFactory {pack_sha256:[7;32],registrations:Vec::new})).unwrap() }
#[test] fn actual_wire_query_setup_common_then_finish_refusal() {
 let p=profile();let mut e=p.begin().unwrap();
 let query=e.receive("neoforge:register", &[0]).unwrap(); assert_eq!(query.len(),1);assert_eq!(query[0].body,[0]);
 let setup=e.receive("neoforge:network", &[0]).unwrap();assert_eq!(setup[0].channel,"minecraft:register");
 assert_eq!(e.receive("c:version", &[1,1]).unwrap()[0].body,[1,1]);
 assert!(e.finish().is_err());assert!(e.receive("c:version", &[1,1]).is_err());
}
#[test] fn unknown_required_task_and_reconnect_cleanup() {
 let p=profile();let mut e=p.begin().unwrap();e.receive("neoforge:register", &[0]).unwrap();e.receive("neoforge:network", &[0]).unwrap();
 assert!(e.receive("neoforge:extensible_enum_data", &[0]).is_err());drop(e);
 let mut fresh=p.begin().unwrap(); assert!(fresh.receive("neoforge:network", &[0]).is_err());
}
#[test] fn failure_payload_never_becomes_success() {let p=profile();let mut e=p.begin().unwrap();assert!(e.receive("neoforge:modded_network_setup_failed", &[0]).is_err());}
'''.replace('COMPANION', str(HERE / 'compat_bridge.rs'))
        # include! cannot contain inner module documentation; preserve source otherwise.
        (root / 'endpoint_include.rs').write_text('\n'.join(line for line in lines if not line.startswith('//!'))+'\n')
        (root / 'test.rs').write_text(source)
        env = os.environ.copy(); env['BRIDGE_TEST_ROOT'] = str(root)
        command = ['rustc','--edition','2021','--test',root/'test.rs','--target',args.target,'--extern','horizon_neoforge_compat='+str(REPO/'target'/args.target/'debug/libhorizon_neoforge_compat.rlib'),'-o',root/'test']
        linker = os.environ.get('CARGO_TARGET_'+args.target.upper().replace('-','_')+'_LINKER')
        if linker: command += ['-C','linker='+linker]
        subprocess.run(list(map(str,command)), check=True, env=env)
        run([root/'test'])
    print('PASS: actual library/companion/patched-endpoint contract; NOT full Horizon or sockets')

if __name__ == '__main__':
    main()
