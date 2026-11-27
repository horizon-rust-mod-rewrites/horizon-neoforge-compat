#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Synthetic lock/wiring safety checks, never full Cargo or engine evidence."""
import copy
from pathlib import Path
import tempfile
import tomllib
import unittest

import check_full


class LockAuditTests(unittest.TestCase):
    def setUp(self):
        # Deliberately tiny synthetic lock graph, not a substitute for the actual pin.
        self.before = {'version': 4, 'package': [
            {'name': 'horizon-client-lib', 'version': '0.1.0', 'dependencies': ['dep']},
            {'name': 'dep', 'version': '1.0.0', 'source': 'registry+https://example.invalid',
             'checksum': 'a' * 64},
        ]}
        self.after = copy.deepcopy(self.before)
        self.after['package'][0]['dependencies'] = ['dep', 'horizon-neoforge-compat']
        core = tomllib.loads((check_full.COMPAT / 'Cargo.toml').read_text())['package']
        self.after['package'].append({'name': core['name'], 'version': core['version']})

    def audit(self):
        check_full.check_lock(self.before, self.after)

    def test_only_local_core_addition_passes(self):
        self.audit()

    def test_existing_version_change_refuses(self):
        self.after['package'][1]['version'] = '1.0.1'
        with self.assertRaisesRegex(RuntimeError, 'existing pinned lock entry'):
            self.audit()

    def test_existing_checksum_change_refuses(self):
        self.after['package'][1]['checksum'] = 'b' * 64
        with self.assertRaisesRegex(RuntimeError, 'existing pinned lock entry'):
            self.audit()

    def test_existing_source_change_refuses(self):
        self.after['package'][1]['source'] = 'registry+https://different.invalid'
        with self.assertRaisesRegex(RuntimeError, 'existing pinned lock entry'):
            self.audit()

    def test_removed_existing_package_refuses(self):
        del self.after['package'][1]
        with self.assertRaisesRegex(RuntimeError, 'existing pinned lock entry'):
            self.audit()

    def test_extra_unexpected_package_refuses(self):
        self.after['package'].append({'name': 'unexpected', 'version': '1.0.0'})
        with self.assertRaisesRegex(RuntimeError, 'unexpected added lock entries'):
            self.audit()

    def test_core_from_registry_refuses(self):
        self.after['package'][-1]['source'] = 'registry+https://example.invalid'
        with self.assertRaisesRegex(RuntimeError, 'unexpected added lock entries'):
            self.audit()

    def test_new_transitive_dependency_on_core_refuses(self):
        self.after['package'][-1]['dependencies'] = ['dep']
        with self.assertRaisesRegex(RuntimeError, 'unexpected added lock entries'):
            self.audit()

    def test_core_not_added_refuses(self):
        self.after['package'].pop()
        with self.assertRaisesRegex(RuntimeError, 'unexpected added lock entries'):
            self.audit()

    def test_client_did_not_link_core_refuses(self):
        self.after['package'][0]['dependencies'] = ['dep']
        with self.assertRaisesRegex(RuntimeError, 'existing pinned lock entry'):
            self.audit()

    def test_unexpected_client_dependency_refuses(self):
        self.after['package'][0]['dependencies'].append('unexpected')
        with self.assertRaisesRegex(RuntimeError, 'existing pinned lock entry'):
            self.audit()


class WiringTests(unittest.TestCase):
    def test_actual_glue_bytes_and_explicit_dependency_are_wired_only_in_scratch(self):
        with tempfile.TemporaryDirectory() as temporary:
            scratch = Path(temporary)
            manifest = scratch / 'crates/horizon-client-lib/Cargo.toml'
            root = scratch / 'crates/horizon-client/src/client_lib.rs'
            manifest.parent.mkdir(parents=True)
            root.parent.mkdir(parents=True)
            manifest.write_text('[package]\nname = "horizon-client-lib"\nversion = "0.1.0"\n'
                                '[dependencies]\nexisting = "1"\n')
            original = '// synthetic client root\npub mod net;\n'
            root.write_text(original)
            check_full.wire_companion(scratch)
            parsed = tomllib.loads(manifest.read_text())
            self.assertEqual(parsed['dependencies']['existing'], '1')
            self.assertEqual(parsed['dependencies']['horizon-neoforge-compat'],
                             {'path': str(check_full.COMPAT)})
            for name in ['compat_bridge.rs', 'bridge_contract_tests.rs']:
                self.assertEqual((scratch / '_neoforge_companion' / name).read_bytes(),
                                 (check_full.HERE / name).read_bytes())
            self.assertTrue(root.read_text().startswith(original))
            self.assertIn('pub mod neoforge_compat_bridge;', root.read_text())
            self.assertIn(str(scratch / '_neoforge_companion/compat_bridge.rs'), root.read_text())

    def test_unexpected_manifest_shape_refuses(self):
        with tempfile.TemporaryDirectory() as temporary:
            scratch = Path(temporary)
            manifest = scratch / 'crates/horizon-client-lib/Cargo.toml'
            manifest.parent.mkdir(parents=True)
            original = '[package]\nname = "unexpected"\n'
            manifest.write_text(original)
            with self.assertRaisesRegex(RuntimeError, 'unexpected pinned client manifest'):
                check_full.wire_companion(scratch)
            self.assertEqual(manifest.read_text(), original)


if __name__ == '__main__':
    unittest.main(verbosity=2)
