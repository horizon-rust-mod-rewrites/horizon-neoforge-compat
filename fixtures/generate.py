#!/usr/bin/env python3
"""Synthetic SPECIFICATION vectors, NOT captures or Java differential results.
Independent literal fields/hex, not output from Rust under test. --check makes no writes.
"""
from pathlib import Path
import sys
vectors = {
    'common-version-v1': '0101',
    'common-version-signed': '03017fffffff ff0f'.replace(' ', ''), # [1,127,-1]
    'common-register-empty-play': '0104706c617900',
    'common-register-demo': '0104706c6179010964656d6f3a70696e67',
    'dinnerbone-demo': '64656d6f3a70696e670064656d6f3a706f6e6700',
    # registry demo:items, IDs 0 -> demo:a and 300 -> demo:b, alias demo:old -> demo:a
    'frozen-items': '0a64656d6f3a6974656d7302000664656d6f3a61ac020664656d6f3a62010864656d6f3a6f6c640664656d6f3a61',
}
for name, value in vectors.items():
    p = Path(__file__).parent / (name + '.bin')
    b = bytes.fromhex(value)
    if '--check' in sys.argv:
        assert p.read_bytes() == b, name
    else:
        p.write_bytes(b)
print(f'{len(vectors)} synthetic fixtures verified')
