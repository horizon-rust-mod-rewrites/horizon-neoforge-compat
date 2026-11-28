#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Validate an evidence-backed rubric and deterministically render its report.

Offline metadata checks only: does not verify test truth, execute protocol code,
fetch a CI result, grant runtime authority or authorize configuration completion.
"""
import argparse
from datetime import date
import json
from pathlib import Path, PurePosixPath
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'progress/compatibility.json'
REPORT = ROOT / 'COMPATIBILITY_PROGRESS.md'
STATUSES = {'complete', 'in_progress', 'blocked', 'not_started'}
BASELINE = {'minecraft': '1.21.1', 'protocol': 767, 'neoforge': '21.1.248',
            'horizon': 'ae632edf48f6cae3ba52a5c81523ec712488c9cc'}
REQUIRED_GATES = {'owner', 'split-authority', 'transport-runtime', 'task-order',
                  'content-transaction', 'baseline-join', 'fixture-join',
                  'live-lifecycle', 'finish-proof', 'scope-review'}
SHA = re.compile(r'[0-9a-f]{40}\Z')
IDENTIFIER = re.compile(r'[a-z][a-z0-9-]*\Z')


def require(condition, message):
    if not condition:
        raise ValueError(message)


def nonempty(value):
    return isinstance(value, str) and bool(value.strip()) and '\n' not in value


def evidence_path(value, root):
    require(nonempty(value), 'evidence path must be a nonempty single line')
    p = PurePosixPath(value)
    require(not p.is_absolute() and '..' not in p.parts and '\\' not in value,
            'evidence path must be repository-relative')
    resolved = (root / value).resolve()
    require(resolved.is_relative_to(root.resolve()) and resolved.is_file(),
            f'missing or escaping evidence: {value}')


def validate(data, root=ROOT):
    require(isinstance(data, dict), 'metadata root must be an object')
    require(type(data.get('schema_version')) is int and data['schema_version'] == 1,
            'unsupported schema_version')
    require(type(data.get('rubric_version')) is int and data['rubric_version'] >= 1,
            'rubric_version must be positive')
    require(data.get('baseline') == BASELINE, 'baseline must match exact pinned target')
    require(nonempty(data.get('updated')), 'updated date required')
    date.fromisoformat(data['updated'])
    run = data.get('last_engine_validation', {})
    require(isinstance(run, dict), 'last_engine_validation must be an object')
    require(bool(SHA.fullmatch(str(run.get('tested_sha', '')))), 'invalid tested SHA')
    require(bool(re.fullmatch(
        r'https://github\.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/[0-9]+',
        str(run.get('run_url', '')))), 'invalid exact-engine run URL')
    evidence_path(run.get('retained_evidence'), root)
    sections = data.get('sections')
    require(isinstance(sections, list) and sections, 'sections must be nonempty')
    tasks = {}
    section_ids = set()
    for section in sections:
        require(isinstance(section, dict), 'section must be an object')
        sid = section.get('id', '')
        require(isinstance(sid, str) and bool(IDENTIFIER.fullmatch(sid)) and sid not in section_ids,
                'duplicate or invalid section ID')
        section_ids.add(sid)
        require(nonempty(section.get('title')), 'section title required')
        require(isinstance(section.get('tasks'), list) and section['tasks'], 'empty section')
        for task in section['tasks']:
            require(isinstance(task, dict), 'task must be an object')
            tid = task.get('id', '')
            require(isinstance(tid, str) and bool(IDENTIFIER.fullmatch(tid)) and tid not in tasks,
                    'duplicate or invalid task ID')
            tasks[tid] = task
            require(nonempty(task.get('title')) and nonempty(task.get('acceptance')),
                    f'{tid}: title and acceptance required')
            require(type(task.get('weight')) is int and 0 < task['weight'] <= 100,
                    f'{tid}: weight must be positive integer')
            require(task.get('status') in STATUSES, f'{tid}: invalid status')
            deps = task.get('depends_on')
            require(isinstance(deps, list) and all(isinstance(x, str) for x in deps)
                    and len(set(deps)) == len(deps), f'{tid}: invalid dependencies')
            evidence = task.get('evidence')
            require(isinstance(evidence, list), f'{tid}: evidence list required')
            require(task['status'] != 'complete' or bool(evidence),
                    f'{tid}: complete task requires retained evidence')
            for item in evidence:
                require(isinstance(item, dict), f'{tid}: evidence entry must be an object')
                evidence_path(item.get('path'), root)
                require(bool(SHA.fullmatch(str(item.get('checkpoint_sha', '')))),
                        f'{tid}: invalid evidence checkpoint SHA')
    require(sum(t['weight'] for t in tasks.values()) == 100, 'weights must total exactly 100')
    require(REQUIRED_GATES <= tasks.keys(), 'missing mandatory readiness gates')
    for tid, task in tasks.items():
        for dep in task['depends_on']:
            require(dep in tasks and dep != tid, f'{tid}: unknown/self dependency')
            require(task['status'] != 'complete' or tasks[dep]['status'] == 'complete',
                    f'{tid}: completed task has incomplete prerequisite {dep}')
    visiting, visited = set(), set()

    def visit(tid):
        require(tid not in visiting, 'dependency cycle')
        if tid in visited:
            return
        visiting.add(tid)
        for dep in tasks[tid]['depends_on']:
            visit(dep)
        visiting.remove(tid)
        visited.add(tid)

    for tid in tasks:
        visit(tid)
    return tasks


def score(tasks):
    return sum(t['weight'] for t in tasks.values() if t['status'] == 'complete')


def render(data, root=ROOT):
    tasks = validate(data, root)
    earned = score(tasks)
    ready = all(t['status'] == 'complete' for t in tasks.values())
    run = data['last_engine_validation']
    lines = [
        '# Compatibility layer completion tracker', '',
        f'**Evidence-backed checklist completion: {earned}% ({earned}/100 points).**', '',
        f'**Ready to begin individual mod rewrites: {"YES — subject to evidence review" if ready else "NO"}.**', '',
        f'Updated: {data["updated"]}. Rubric: v{data["rubric_version"]}.', '',
        '## What this percentage means', '',
        '- A conservative, human-weighted acceptance checklist, NOT measured engineering effort, elapsed time, ETA, test coverage or percentage of mods supported.',
        '- Only fully evidenced **complete** rows earn their fixed points. Partial/in-progress/blocked rows earn zero. Tracker/documentation work itself earns no points.',
        '- The denominator is the full reusable baseline layer, including semantic content and isolated live proof, not just negotiation. Scope: MC **1.21.1**, protocol **767**, NeoForge **21.1.248** only.',
        '- 100% means ready to BEGIN separate mod rewrites against the documented supported contract; it does not implement or guarantee support for roughly 70 mods or every NeoForge API.',
        '- New requirements must be added, not hidden as exclusions to reach 100%. Rebalance weights to 100 with a rubric-version bump and explain the change in the ledger; percentages across rubric versions are not directly comparable.',
        '- A green metadata check validates arithmetic, evidence-file presence and prerequisite consistency only. Humans must review evidence truth, scope and freshness; older passing SHAs do not validate newer code.', '',
        '## Latest executed engine checkpoint', '',
        f'- Tested source SHA: `{run["tested_sha"]}`; distinct from newer tracking/docs commits.',
        f'- [Successful hosted run]({run["run_url"]}); [retained selected-engine evidence]({run["retained_evidence"]}).',
        '- 172 core tests EACH debug/release, including 37 split tests; 16 companion / 9 early / 14 protocol / 81 network-filter passed, 3 ignored. Selected suites only; overlapping isolated tests are not additional coverage.',
        '- **Current runtime: CANNOT JOIN.** Production split routing, complete tasks, real semantic content publication, launcher/UI and live interoperability remain unproven. Default split/feature-flag opt-ins remain disabled; both finish refusals remain.', '',
        '## Milestone percentages', '',
        '| Milestone | Earned / allocated points | Completion |',
        '| --- | ---: | ---: |',
    ]
    for section in data['sections']:
        total = sum(t['weight'] for t in section['tasks'])
        done = score({t['id']: t for t in section['tasks']})
        lines.append(f'| {section["title"]} | {done} / {total} | {done / total * 100:.1f}% |')
    lines += ['', '## Acceptance checklist', '',
              'Evidence checkpoint SHAs identify where the referenced record/source was reviewed, not necessarily the original execution SHA. Read each retained transcript for what actually ran. Source/test files support inspection but do not by themselves prove execution.', '']
    def cell(text):
        return text.replace('|', '&#124;')
    for section in data['sections']:
        lines += [f'### {section["title"]}', '',
                  '| ID / deliverable | Points | Status | Acceptance / retained evidence |',
                  '| --- | ---: | --- | --- |']
        for t in section['tasks']:
            refs = '; '.join(f'[{e["path"]}]({e["path"]}) (checkpoint `{e["checkpoint_sha"][:7]}`)' for e in t['evidence'])
            dep = ' Prerequisites: ' + ', '.join(f'`{d}`' for d in t['depends_on']) + '.' if t['depends_on'] else ''
            lines.append(f'| `{t["id"]}` — {cell(t["title"])} | {t["weight"]} | {t["status"].replace("_", " ")} | {cell(t["acceptance"])}{dep}' + (f' Evidence: {refs}' if refs else ' No completion evidence yet.') + ' |')
        lines.append('')
    lines += [
        '## Immediate next implementation gate', '',
        'Implement `owner` first. See [ordered connection acceptance plan](docs/ordered-connection-owner.md). A new timer, phase snapshot or split opt-in alone is not that owner. Then prove split authority, admission/deadlines and awaited production dispatch/write completion. Keep both finish refusals throughout this work.', '',
        '## Updating this tracker', '',
        '1. Edit `progress/compatibility.json`; attach retained repository-relative evidence and its reviewed checkpoint SHA. Complete prerequisites first. Regressions must downgrade affected rows and any dependent completed rows.',
        '2. Record the exact tested source SHA in every new execution record. Keep implementation, tests, CI results and live-server proof distinct. Update `last_engine_validation` only after inspecting that exact run.',
        '3. Run `python3 progress/report.py` then `python3 progress/report.py --check` and `python3 -m unittest discover -s progress -p "test_*.py" -v`.',
        '4. Update the change ledger and NEXT_AGENT.md; refresh SHA256SUMS. Use the same draft PR with the required AI attribution footer.',
        '5. Before reporting 100%, review ALL acceptance criteria and final exact-source runtime/live evidence. This file never grants Loader authority or toggles a finish gate.', '',
        '## Change ledger', '',
        '- 2026-11-28, rubric v1: initial 100-point baseline, **30% complete**. Credits existing exact-version codec/infrastructure and selected hosted engine evidence only. New tracker/owner plan adds **0 compatibility points**. No new protocol implementation or live execution is claimed.', '',
        '<!-- Generated by progress/report.py; edit JSON and renderer, then regenerate. -->', '',
    ]
    return '\n'.join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true', help='refuse stale generated report without writing')
    args = parser.parse_args()
    try:
        text = render(json.loads(DATA.read_text()))
        if args.check:
            require(REPORT.is_file() and REPORT.read_text() == text,
                    'stale COMPATIBILITY_PROGRESS.md; run python3 progress/report.py')
        else:
            REPORT.write_text(text)
        print('PASS: progress metadata/report consistent (not runtime validation)')
    except (ValueError, KeyError, TypeError, OSError) as error:
        print(f'FAIL: {error}', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
