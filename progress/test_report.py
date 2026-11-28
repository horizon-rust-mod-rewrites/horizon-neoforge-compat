#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Dependency-free regression tests for progress metadata, never engine tests."""
import copy
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

import report


class ReportTests(unittest.TestCase):
    def setUp(self):
        self.data = json.loads(report.DATA.read_text())

    def task(self, tid):
        return next(t for s in self.data['sections'] for t in s['tasks'] if t['id'] == tid)

    def rejects(self, message):
        with self.assertRaisesRegex(ValueError, message):
            report.validate(self.data)

    def test_baseline_valid_and_thirty_percent(self):
        self.assertEqual(report.score(report.validate(self.data)), 30)

    def test_non_object_metadata_root_refuses(self):
        for value in (None, [], 'metadata'):
            with self.subTest(value=value):
                with self.assertRaisesRegex(ValueError, 'metadata root must be an object'):
                    report.validate(value)

    def test_non_object_validation_records_refuse(self):
        mutations = [
            ('last_engine_validation must be an object',
             lambda data: data.__setitem__('last_engine_validation', None)),
            ('section must be an object',
             lambda data: data['sections'].__setitem__(0, None)),
            ('task must be an object',
             lambda data: data['sections'][0]['tasks'].__setitem__(0, None)),
            ('evidence entry must be an object',
             lambda data: self.task_in(data, 'pins').__setitem__('evidence', [None])),
        ]
        for message, mutate in mutations:
            with self.subTest(message=message):
                data = copy.deepcopy(self.data)
                mutate(data)
                with self.assertRaisesRegex(ValueError, message):
                    report.validate(data)

    @staticmethod
    def task_in(data, tid):
        return next(t for s in data['sections'] for t in s['tasks'] if t['id'] == tid)

    def test_partial_and_blocked_rows_earn_zero(self):
        tasks = report.validate(self.data)
        for status in ('blocked', 'in_progress', 'not_started'):
            changed = copy.deepcopy(tasks)
            changed['owner']['status'] = status
            self.assertEqual(report.score(changed), 30)

    def test_weights_must_total_one_hundred(self):
        self.task('pins')['weight'] += 1
        self.rejects('total exactly 100')

    def test_boolean_weight_is_not_integer(self):
        self.task('pins')['weight'] = True
        self.rejects('positive integer')

    def test_zero_and_negative_weights_refuse(self):
        for value in (0, -1):
            with self.subTest(value=value):
                self.task('pins')['weight'] = value
                self.rejects('positive integer')

    def test_duplicate_task_ids_refuse(self):
        self.task('licenses')['id'] = 'pins'
        self.rejects('duplicate or invalid task')

    def test_duplicate_section_ids_refuse(self):
        self.data['sections'][1]['id'] = self.data['sections'][0]['id']
        self.rejects('duplicate or invalid section')

    def test_unknown_status_refuses(self):
        self.task('pins')['status'] = 'almost_complete'
        self.rejects('invalid status')

    def test_completed_row_needs_evidence(self):
        self.task('pins')['evidence'] = []
        self.rejects('requires retained evidence')

    def test_missing_evidence_refuses(self):
        self.task('pins')['evidence'][0]['path'] = 'progress/does-not-exist.txt'
        self.rejects('missing or escaping evidence')

    def test_absolute_traversal_and_backslash_paths_refuse(self):
        for value in ('/etc/passwd', '../README.md', 'progress/../README.md', 'progress\\report.py'):
            with self.subTest(value=value):
                self.task('pins')['evidence'][0]['path'] = value
                self.rejects('repository-relative')

    def test_external_symlink_refuses(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d) / 'root'
            root.mkdir()
            outside = Path(d) / 'outside.txt'
            outside.write_text('not repository evidence')
            (root / 'link').symlink_to(outside)
            with self.assertRaisesRegex(ValueError, 'missing or escaping evidence'):
                report.evidence_path('link', root)

    def test_invalid_checkpoint_sha_refuses(self):
        self.task('pins')['evidence'][0]['checkpoint_sha'] = 'main'
        self.rejects('checkpoint SHA')

    def test_unknown_or_self_dependency_refuses(self):
        for dep in ('unknown', 'pins'):
            with self.subTest(dep=dep):
                self.task('pins')['depends_on'] = [dep]
                self.rejects('unknown/self dependency')

    def test_duplicate_dependencies_refuse(self):
        self.task('owner')['depends_on'] = ['selection', 'selection']
        self.rejects('invalid dependencies')

    def test_cycle_refuses(self):
        self.task('owner')['depends_on'] = ['split-authority']
        self.rejects('dependency cycle')

    def test_complete_cannot_depend_on_incomplete(self):
        self.task('engine-build')['depends_on'] = ['owner']
        self.rejects('incomplete prerequisite')

    def test_required_gate_cannot_disappear(self):
        self.task('owner')['id'] = 'renamed-owner'
        self.rejects('missing mandatory readiness gates')

    def test_exact_baseline_is_required(self):
        self.data['baseline']['neoforge'] = '21.1.x'
        self.rejects('exact pinned target')

    def test_invalid_execution_identity_refuses(self):
        self.data['last_engine_validation']['tested_sha'] = 'HEAD'
        self.rejects('tested SHA')

    def test_invalid_run_url_refuses(self):
        self.data['last_engine_validation']['run_url'] = 'https://example.invalid/run/1'
        self.rejects('run URL')

    def test_report_is_deterministic_and_contains_no_ready_claim(self):
        first = report.render(self.data)
        self.assertEqual(first, report.render(self.data))
        self.assertIn('30% (30/100 points)', first)
        self.assertIn('mod rewrites: NO', first)
        self.assertIn('CANNOT JOIN', first)
        self.assertIn('both finish refusals remain', first)
        self.assertEqual(first, report.REPORT.read_text())

    def test_invalid_json_cli_fails_without_rewriting(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root / 'progress').mkdir()
            (root / 'progress/report.py').write_text(Path(report.__file__).read_text())
            (root / 'progress/compatibility.json').write_text('{')
            output = root / 'COMPATIBILITY_PROGRESS.md'
            output.write_text('sentinel')
            run = subprocess.run([sys.executable, str(root / 'progress/report.py')],
                                 capture_output=True, text=True)
            self.assertEqual(run.returncode, 1)
            self.assertIn('FAIL:', run.stderr)
            self.assertEqual(output.read_text(), 'sentinel')

    def test_non_object_json_cli_fails_without_traceback_or_rewriting(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root / 'progress').mkdir()
            (root / 'progress/report.py').write_text(Path(report.__file__).read_text())
            (root / 'progress/compatibility.json').write_text('null')
            output = root / 'COMPATIBILITY_PROGRESS.md'
            output.write_text('sentinel')
            run = subprocess.run([sys.executable, str(root / 'progress/report.py')],
                                 capture_output=True, text=True)
            self.assertEqual(run.returncode, 1)
            self.assertIn('FAIL: metadata root must be an object', run.stderr)
            self.assertNotIn('Traceback', run.stderr)
            self.assertEqual(output.read_text(), 'sentinel')

    def test_check_mode_passes_without_writing(self):
        before = report.REPORT.stat().st_mtime_ns
        run = subprocess.run([sys.executable, str(Path(report.__file__)), '--check'],
                             capture_output=True, text=True)
        self.assertEqual(run.returncode, 0, run.stderr)
        self.assertEqual(before, report.REPORT.stat().st_mtime_ns)

    def test_check_mode_refuses_stale_output_without_writing(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root / 'progress').mkdir()
            # Copy just renderer plus its referenced evidence, not game/build data.
            (root / 'progress/report.py').write_text(Path(report.__file__).read_text())
            (root / 'progress/compatibility.json').write_text(json.dumps(self.data))
            paths = {self.data['last_engine_validation']['retained_evidence']}
            paths.update(e['path'] for s in self.data['sections'] for t in s['tasks'] for e in t['evidence'])
            for value in paths:
                p = root / value
                p.parent.mkdir(parents=True, exist_ok=True)
                p.write_text('synthetic file-presence fixture, not test evidence')
            output = root / 'COMPATIBILITY_PROGRESS.md'
            output.write_text('stale')
            run = subprocess.run([sys.executable, str(root / 'progress/report.py'), '--check'],
                                 capture_output=True, text=True)
            self.assertEqual(run.returncode, 1)
            self.assertIn('stale COMPATIBILITY_PROGRESS.md', run.stderr)
            self.assertEqual(output.read_text(), 'stale')


if __name__ == '__main__':
    unittest.main()
