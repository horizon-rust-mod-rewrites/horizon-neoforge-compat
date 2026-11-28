#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Dependency-free driver safety tests, NOT Horizon compilation/runtime tests."""
import contextlib
import io
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

import check_full

GIB = 1024**3


class DiskPreflightTests(unittest.TestCase):
    def check_space(self, free, minimum=3, source_bytes=0):
        with patch.object(check_full.shutil, 'disk_usage', return_value=SimpleNamespace(free=free)), \
                contextlib.redirect_stdout(io.StringIO()):
            check_full.disk_preflight(Path('.'), minimum, source_bytes)

    def test_below_reservation_refuses(self):
        with self.assertRaisesRegex(RuntimeError, 'NOT STARTED: insufficient disk'):
            self.check_space(3 * GIB - 1)

    def test_exact_reservation_passes(self):
        self.check_space(3 * GIB)

    def test_source_copy_is_additional(self):
        with self.assertRaisesRegex(RuntimeError, 'insufficient disk'):
            self.check_space(3 * GIB + 100, source_bytes=101)
        self.check_space(3 * GIB + 101, source_bytes=101)

    def test_invalid_reservations_refuse_before_disk_access(self):
        for value in [float('nan'), float('inf'), -float('inf'), -1, 0, 0.099]:
            with self.subTest(value=value), patch.object(check_full.shutil, 'disk_usage') as disk:
                with self.assertRaisesRegex(RuntimeError, 'finite and at least 0.1 GiB'):
                    check_full.disk_preflight(Path('.'), value)
                disk.assert_not_called()

    def test_minimum_allowed_reservation(self):
        self.check_space(int(0.1 * GIB), minimum=0.1)


class DriverPreflightTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.scratch = self.root / 'new-build'
        self.output = io.StringIO()
        self.enterContext(contextlib.redirect_stdout(self.output))
        self.enterContext(contextlib.redirect_stderr(io.StringIO()))
        # Any subprocess is a test failure; no Git, Cargo or network is executed.
        self.process = self.enterContext(patch.object(
            check_full.subprocess, 'run', side_effect=AssertionError('unexpected subprocess')))
        self.verify = self.enterContext(patch.object(check_full, 'verify'))
        self.tracked = self.enterContext(patch.object(check_full, 'tracked_sources'))
        self.disk = self.enterContext(patch.object(
            check_full.shutil, 'disk_usage', return_value=SimpleNamespace(free=8 * GIB)))

    def invoke(self, *args):
        with patch('sys.argv', ['check_full.py', '--scratch', str(self.scratch), *map(str, args)]):
            check_full.main()

    def assert_untouched(self):
        self.assertFalse(self.scratch.exists())
        self.verify.assert_not_called()
        self.tracked.assert_not_called()
        self.process.assert_not_called()

    def test_disk_only_success_does_not_claim_build_or_touch_inputs(self):
        self.invoke('--disk-preflight-only')
        self.assertIn('NO source, toolchain, dependency, compilation, engine/TCP or join checks',
                      self.output.getvalue())
        self.assertIn('NOT guaranteed', self.output.getvalue())
        self.assert_untouched()

    def test_disk_only_ignores_optional_nonexistent_source(self):
        self.invoke(self.root / 'not-downloaded', '--disk-preflight-only')
        self.assert_untouched()

    def test_disk_only_low_space_refuses(self):
        self.disk.return_value.free = GIB
        with self.assertRaisesRegex(RuntimeError, 'NOT STARTED: insufficient disk'):
            self.invoke('--disk-preflight-only')
        self.assert_untouched()

    def test_normal_mode_refuses_disk_before_reading_source(self):
        self.disk.return_value.free = GIB
        with self.assertRaisesRegex(RuntimeError, 'NOT STARTED: insufficient disk'):
            self.invoke(self.root / 'not-downloaded')
        self.assert_untouched()

    def test_normal_mode_still_requires_source(self):
        with self.assertRaises(SystemExit) as error:
            self.invoke()
        self.assertEqual(error.exception.code, 2)
        self.assert_untouched()

    def test_existing_scratch_refuses_without_modification(self):
        self.scratch.mkdir()
        sentinel = self.scratch / 'keep.txt'
        sentinel.write_text('preserve existing work')
        with self.assertRaisesRegex(RuntimeError, 'scratch must not exist'):
            self.invoke('--disk-preflight-only')
        self.assertEqual(sentinel.read_text(), 'preserve existing work')
        self.disk.assert_not_called()
        self.process.assert_not_called()

    def test_missing_parent_refuses_without_creation(self):
        self.scratch = self.root / 'missing-parent' / 'new-build'
        with self.assertRaisesRegex(RuntimeError, 'parent must already exist'):
            self.invoke('--disk-preflight-only')
        self.assertFalse(self.scratch.parent.exists())
        self.assert_untouched()

    def test_normal_mode_missing_source_blobs_refuse(self):
        self.tracked.return_value = ([], ['absent.rs'])
        with self.assertRaisesRegex(RuntimeError, 'incomplete source.*absent.rs'):
            self.invoke(self.root / 'source')
        self.assertFalse(self.scratch.exists())
        self.process.assert_not_called()

    def test_normal_mode_rechecks_disk_with_source_copy_size(self):
        self.disk.return_value.free = 3 * GIB
        self.tracked.return_value = ([('large.rs', '100644', 1)], [])
        with self.assertRaisesRegex(RuntimeError, 'insufficient disk'):
            self.invoke(self.root / 'source')
        self.assertEqual(self.disk.call_count, 2)
        self.assertFalse(self.scratch.exists())
        self.process.assert_not_called()

    def test_normal_mode_still_rejects_overlapping_trees(self):
        with self.assertRaisesRegex(RuntimeError, 'separate trees'):
            self.invoke(self.root)
        self.assert_untouched()


if __name__ == '__main__':
    unittest.main(verbosity=2)
