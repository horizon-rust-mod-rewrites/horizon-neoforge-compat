#!/usr/bin/env python3
"""Bounded actual-Java/Rust query/setup differential tests, never a join test."""
import argparse
import hashlib
import json
from pathlib import Path
import random
import subprocess
import zipfile

SEED = 211248
ROOT = Path(__file__).resolve().parents[2]


def hx(value):
    return value.encode('utf-8').hex()


def model(kind, phases):
    result = []
    for phase, records in sorted(phases.items()):
        result.append(f'P:{phase}')
        for identifier, version, flow, optional in sorted(records):
            result.append(f'Q:{hx(identifier)}:{hx(version)}:{flow}:{int(optional)}' if kind == 'query'
                          else f'S:{hx(identifier)}:{hx(version)}')
    return ';'.join(result)


def corpus():
    cases = []
    for kind in ('query', 'setup'):
        cases.append((kind, ''))
        for phase in range(5):
            cases.append((kind, model(kind, {phase: []})))
            for flow in (-1, 0, 1):
                for optional in (False, True):
                    cases.append((kind, model(kind, {phase: [('demo:channel', '1', flow, optional)]})))
        cases.append((kind, model(kind, {p: [('a:same', '', -1, False)] for p in range(5)})))
        for version in ('', 'λ雪🦫', '\x00\t\n;:', 'a' * 127, 'b' * 128, 'c' * 1024, '🦫' * 256):
            cases.append((kind, model(kind, {4: [('a:v', version, 1, True)]})))
        cases.append((kind, model(kind, {1: [('a:' + 'z' * 254, 'v', -1, False)]})))
        # Aggregate entry bound (across phases), and multi-byte collection counts.
        cases.append((kind, model(kind, {p: [(f'a:{i:03x}', '', i % 3 - 1, i % 2 == 0)
                                                 for i in range(p * 200, (p + 1) * 200)] for p in range(5)})))
        cases.append((kind, model(kind, {4: [(f'a:{i:03x}', '', -1, False) for i in range(1024)]})))
        cases.append((kind, model(kind, {4: [(f'a:{i:03x}', 'v' * 1024, -1, False) for i in range(31)]})))
        rng = random.Random(SEED)
        for index in range(80):
            phases = {}
            for p in rng.sample(range(5), rng.randrange(6)):
                phases[p] = [(f'demo:p{p}/v{i}', rng.choice(['', '1', '2.0-β', '🦫', str(index)]),
                              rng.choice([-1, 0, 1]), bool(rng.getrandbits(1))) for i in range(rng.randrange(9))]
            cases.append((kind, model(kind, phases)))
    return cases


def batch(command, requests, cwd):
    result = subprocess.run(command, input=''.join('\t'.join(r) + '\n' for r in requests),
                            capture_output=True, text=True, encoding='utf-8', timeout=90, cwd=cwd)
    if result.returncode:
        raise AssertionError(f'process failed {result.returncode}: {result.stderr[-3000:]}')
    lines = [line.split('\t', 2)[1:] for line in result.stdout.splitlines() if line.startswith('REF\t')]
    assert len(lines) == len(requests), (len(lines), len(requests), result.stderr[-3000:])
    return lines


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--scratch', type=Path, required=True)
    parser.add_argument('--java-home', type=Path, required=True)
    parser.add_argument('--rust-bin', type=Path, required=True)
    parser.add_argument('--report', type=Path, help='optional JSON result file')
    args = parser.parse_args()
    scratch = args.scratch.resolve()
    assert ROOT not in scratch.parents and scratch != ROOT
    # Assert the tested Minecraft class bytes are exactly from the pinned game jar.
    assert hashlib.sha256((scratch / 'inner.jar').read_bytes()).hexdigest() == 'c301de10f575027d13eac18c7f34409d60648cf56a35d566aa1f530ff617840a'
    with zipfile.ZipFile(scratch / 'inner.jar') as official, zipfile.ZipFile(scratch / 'runtime.jar') as runtime:
        official_classes = {name: hashlib.sha256(official.read(name)).hexdigest() for name in official.namelist() if name.endswith('.class')}
        assert set(official_classes) == {name for name in runtime.namelist() if name.endswith('.class')}
        assert all(hashlib.sha256(runtime.read(name)).hexdigest() == digest for name, digest in official_classes.items())
    java = [str(args.java_home.resolve() / 'bin/java'), '-Xmx128m', '-cp',
            f'{scratch}/classes:{scratch}/runtime.jar:{scratch}/libraries/*', 'CodecReference']
    rust = [str(args.rust_bin.resolve())]
    cases = corpus()
    encodes = [('encode', kind, value) for kind, value in cases]
    j_encoded = batch(java, encodes, scratch)
    r_encoded = batch(rust, encodes, scratch)
    for source, encoded in [('Java', j_encoded), ('Rust', r_encoded)]:
        assert all(status == 'OK' for status, _ in encoded), (source, [x for x in encoded if x[0] != 'OK'])
        requests = [('decode', kind, encoded[i][1]) for i, (kind, _) in enumerate(cases)]
        for target, command in [('Java', java), ('Rust', rust)]:
            decoded = batch(command, requests, scratch)
            for i, (_, expected) in enumerate(cases):
                assert decoded[i] == ['OK', expected], (source, target, i, cases[i], decoded[i])
    print(f'PASS {len(cases)} valid models, both encoders -> both decoders ({4 * len(cases)} semantic comparisons)')

    # Independent literal anchor; actual upstream Java and Rust must both emit it.
    anchors = [('query', model('query', {4: [('demo:x', '1', -1, False)]}), '0104010664656d6f3a7801310000'),
               ('setup', model('setup', {4: [('demo:x', '1', -1, False)]}), '0104010664656d6f3a780664656d6f3a780131')]
    for command in (java, rust):
        assert batch(command, [('encode', k, v) for k, v, _ in anchors], scratch) == [['OK', h] for _, _, h in anchors]
    print('PASS 2 literal anchors in both independent encoders')

    # Every proper byte prefix of selected populated upstream AND Rust outputs.
    malformed = set()
    for encoded in (j_encoded, r_encoded):
        for kind in ('query', 'setup'):
            selected = [i for i, (k, v) in enumerate(cases) if k == kind and v and 10 <= len(encoded[i][1]) // 2 <= 120][:12]
            selected += [i for i, (k, _) in enumerate(cases) if k == kind and 10 <= len(encoded[i][1]) <= 240][-8:]
            for i in selected:
                body = encoded[i][1]
                for end in range(0, len(body), 2):
                    malformed.add((kind, body[:end]))
    malformed.update((kind, body) for kind in ('query', 'setup') for body in ('', '0105', '01ffffffff0f', '010401', '0000', '808080808000'))
    requests = [('decode', k, body) for k, body in sorted(malformed)]
    for command in (java, rust):
        results = batch(command, requests, scratch)
        assert all(status == 'ERR' for status, _ in results), [(requests[i], value) for i, value in enumerate(results) if value[0] != 'ERR'][:3]
    print(f'PASS {len(requests)} shared malformed/truncation cases rejected by both')

    # Known deliberate safety-policy differences: do not demand false equivalence.
    # Java accepts nonzero booleans, duplicate map phases, duplicate records, and
    # version strings exceeding Rust's byte limit. Sizes remain small and bounded.
    canonical = anchors[0][2]
    divergences = [
        ('query', canonical[:-2] + '02'),
        ('query', canonical[:-4] + '020000'),
        ('query', '0204000400'),
        ('query', '010402' + canonical[6:] * 2),
        ('setup', '0204000400'),
        ('setup', anchors[1][2].replace('780131', '790131')),
        ('setup', '010402' + anchors[1][2][6:] * 2),
    ]
    for kind in ('query', 'setup'):
        value = model(kind, {4: [('a:v', 'é' * 513, -1, False)]})
        upstream = batch(java, [('encode', kind, value)], scratch)[0]
        assert upstream[0] == 'OK'
        divergences.append((kind, upstream[1]))
        assert batch(rust, [('encode', kind, value)], scratch)[0][0] == 'ERR'
    for kind in ('query', 'setup'):
        for records in ([('a:long', 'x' * 1025, -1, False)],
                        [(f'a:{i:03x}', '', -1, False) for i in range(1025)],
                        [(f'a:{i:03x}', 'v' * 1024, -1, False) for i in range(32)]):
            value = model(kind, {4: records})
            assert batch(java, [('encode', kind, value)], scratch)[0][0] == 'OK'
            assert batch(rust, [('encode', kind, value)], scratch)[0][0] == 'ERR'
    print('PASS 6 Java-encodable/Rust-writer-refused boundary models (string, entry, body limits)')
    requests = [('decode', kind, body) for kind, body in divergences]
    assert all(status == 'OK' for status, _ in batch(java, requests, scratch))
    assert all(status == 'ERR' for status, _ in batch(rust, requests, scratch))
    print(f'PASS {len(divergences)} explicitly classified upstream-accepts/Rust-rejects cases')

    # Bounded deterministic smoke corpus, no random multi-gigabyte collection sizes.
    rng = random.Random(SEED)
    requests = [('decode', kind, bytes([rng.randrange(4), rng.randrange(5), rng.randrange(4)] +
                 [rng.randrange(256) for _ in range(rng.randrange(32))]).hex())
                for kind in ('query', 'setup') for _ in range(128)]
    j_random, r_random = batch(java, requests, scratch), batch(rust, requests, scratch)
    shared_accept = 0
    for request, j, r in zip(requests, j_random, r_random):
        if r[0] == 'OK':
            assert j == r, (request, j, r)
            shared_accept += 1
    print(f'PASS {len(requests)} bounded smoke inputs: every Rust acceptance agrees ({shared_accept} acceptances)')
    report = {'baseline': {'minecraft': '1.21.1', 'protocol': 767, 'neoforge': '21.1.248'},
              'seed': SEED, 'valid_models': len(cases), 'semantic_comparisons': 4 * len(cases),
              'literal_anchors': 2, 'writer_policy_boundaries': 6, 'shared_malformed': len(malformed), 'classified_policy_divergences': len(divergences),
              'bounded_smoke_inputs': len(requests), 'smoke_shared_acceptances': shared_accept,
              'java_version': subprocess.run(java[:1] + ['-version'], capture_output=True, text=True, check=True).stderr.strip(),
              'rust_driver_sha256': hashlib.sha256(args.rust_bin.read_bytes()).hexdigest(),
              'source_adaptation': json.loads((scratch / 'source-adaptation.json').read_text()),
              'runtime_jar_sha256': hashlib.sha256((scratch / 'runtime.jar').read_bytes()).hexdigest(),
              'unchanged_official_classfiles': len(official_classes),
              'official_codec_class_sha256': {name: official_classes[name] for name in ('vw.class', 'yv.class', 'yx.class', 'akr.class', 'vu.class', 'zh.class')},
              'test_driver_sha256': {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in
                  (Path(__file__).resolve(), Path(__file__).with_name('prepare.py').resolve(), Path(__file__).with_name('CodecReference.java').resolve(), ROOT / 'src/bin/codec-reference.rs')},
              'limitations': ['Source-extracted NeoForge codec compositions, not a full NeoForge installation.',
                              'Query/setup only; no failure Component codec execution.',
                              'No server connection, framing, live join, game content or individual mods tested.',
                              'Bounded deterministic suite, not exhaustive or coverage-guided fuzzing.']}
    if args.report:
        args.report.write_text(json.dumps(report, indent=2) + '\n')
    print('PASS actual Java 21 / official MC 1.21.1 / source-extracted NeoForge 21.1.248 differential suite')


if __name__ == '__main__':
    main()
