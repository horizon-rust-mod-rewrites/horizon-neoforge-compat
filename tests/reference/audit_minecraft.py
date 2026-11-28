#!/usr/bin/env python3
"""Audit enum initializer bytecode from official Minecraft 1.21.1 (no JVM).
Usage: python3 audit_minecraft.py [scratch-directory]
Downloads public version metadata and official server/mappings into the supplied
scratch directory; verifies publisher SHA-1. No game binaries enter the repository.
This is NOT Java reference execution or an independent codec equivalence test.
"""
import hashlib
import io
import json
from pathlib import Path
import sys
import urllib.request
import zipfile


def class_data(b):
    p = 8
    def u2():
        nonlocal p
        value = int.from_bytes(b[p:p+2], 'big')
        p += 2
        return value
    cp = [None] * u2()
    i = 1
    while i < len(cp):
        tag = b[p]
        p += 1
        if tag == 1:
            n = u2()
            cp[i] = b[p:p+n].decode()
            p += n
        elif tag in (7, 8, 16, 19, 20):
            cp[i] = (tag, u2())
        elif tag in (9, 10, 11, 12, 17, 18):
            cp[i] = (tag, u2(), u2())
        elif tag in (3, 4):
            cp[i] = (tag, b[p:p+4].hex())
            p += 4
        elif tag in (5, 6):
            cp[i] = (tag, b[p:p+8].hex())
            p += 8
            i += 1
        elif tag == 15:
            cp[i] = (tag, b[p], int.from_bytes(b[p+1:p+3], 'big'))
            p += 3
        else:
            raise ValueError(tag)
        i += 1
    def resolve(index):
        v = cp[index]
        if isinstance(v, str):
            return v
        if v[0] in (7, 8):
            return resolve(v[1])
        if v[0] in (9, 10, 11, 12):
            return ':'.join(resolve(x) for x in v[1:])
        return str(v)
    p += 6
    count = u2()
    p += 2 * count
    methods = {}
    for group in range(2):
        for _ in range(u2()):
            u2()
            name, desc = resolve(u2()), resolve(u2())
            for _ in range(u2()):
                an = resolve(u2())
                n = int.from_bytes(b[p:p+4], 'big')
                p += 4
                data = b[p:p+n]
                p += n
                if group == 1 and an == 'Code':
                    length = int.from_bytes(data[4:8], 'big')
                    methods[(name, desc)] = data[8:8+length]
    return resolve, methods


def audit_enum(b, class_name, expected):
    resolve, methods = class_data(b)
    code = methods[('<clinit>', '()V')]
    # These exact enum initializers consist of NEW/DUP/LDC/ICONST/LDC/
    # INVOKESPECIAL/PUTSTATIC for each constant, then the synthetic values array.
    p = 0
    for ordinal, (field, textual_id) in enumerate(expected):
        assert code[p] == 0xbb and resolve(int.from_bytes(code[p+1:p+3], 'big')) == class_name
        p += 3
        assert code[p] == 0x59  # dup
        p += 1
        assert code[p] == 0x12 and resolve(code[p+1]) == field
        p += 2
        assert code[p] == 0x03 + ordinal  # iconst_0 ... iconst_4
        p += 1
        assert code[p] == 0x12 and resolve(code[p+1]) == textual_id
        p += 2
        assert code[p] == 0xb7 and resolve(int.from_bytes(code[p+1:p+3], 'big')) == class_name + ':<init>:(Ljava/lang/String;ILjava/lang/String;)V'
        p += 3
        assert code[p] == 0xb3 and resolve(int.from_bytes(code[p+1:p+3], 'big')) == class_name + ':' + chr(ord('a')+ordinal) + ':L' + class_name + ';'
        p += 3
        print(class_name, field, 'ordinal', ordinal, 'textual ID', textual_id)
    # Verify the synthetic values array also uses the same declaration order.
    values = methods[('$values' if ('$values', '()[L'+class_name+';') in methods else ('b' if class_name == 'vu' else 'c'), '()[L'+class_name+';')]
    assert values[0] == 0x03 + len(expected) and values[1] == 0xbd
    assert resolve(int.from_bytes(values[2:4], 'big')) == class_name
    q = 4
    for ordinal in range(len(expected)):
        assert values[q:q+3] == bytes([0x59, 0x03+ordinal, 0xb2])
        assert resolve(int.from_bytes(values[q+3:q+5], 'big')) == class_name + ':' + chr(ord('a')+ordinal) + ':L' + class_name + ';'
        assert values[q+5] == 0x53
        q += 6
    assert values[q:] == b'\xb0'


def main():
    p = Path(sys.argv[1]) if len(sys.argv) > 1 else Path('/tmp/mc-1.21.1-codec-audit')
    p.mkdir(parents=True, exist_ok=True)
    def fetch(url, name):
        f = p / name
        if not f.exists():
            f.write_bytes(urllib.request.urlopen(url, timeout=90).read())
        return f.read_bytes()
    m = json.loads(fetch('https://piston-meta.mojang.com/mc/game/version_manifest_v2.json', 'manifest.json'))
    v = next(x for x in m['versions'] if x['id'] == '1.21.1')
    data = fetch(v['url'], 'version.json')
    assert hashlib.sha1(data).hexdigest() == v['sha1']
    d = json.loads(data)
    for key in ('server', 'server_mappings'):
        name = key + ('.jar' if key == 'server' else '.txt')
        b = fetch(d['downloads'][key]['url'], name)
        assert hashlib.sha1(b).hexdigest() == d['downloads'][key]['sha1']
        print(key, 'SHA256', hashlib.sha256(b).hexdigest())
        if key == 'server':
            with zipfile.ZipFile(io.BytesIO(b)) as z:
                inner = next(n for n in z.namelist() if n.startswith('META-INF/versions/') and n.endswith('.jar'))
                server = z.read(inner)
    with zipfile.ZipFile(io.BytesIO(server)) as z:
        for name, expected in [('vu', [('HANDSHAKING', 'handshake'), ('PLAY', 'play'), ('STATUS', 'status'), ('LOGIN', 'login'), ('CONFIGURATION', 'configuration')]), ('zh', [('SERVERBOUND', 'serverbound'), ('CLIENTBOUND', 'clientbound')])]:
            b = z.read(name+'.class')
            print(name+'.class', 'SHA256', hashlib.sha256(b).hexdigest())
            audit_enum(b, name, expected)
    mappings = (p/'server_mappings.txt').read_text()
    assert 'net.minecraft.network.ConnectionProtocol -> vu:' in mappings
    assert 'net.minecraft.network.protocol.PacketFlow -> zh:' in mappings
    print('Exact official 1.21.1 enum initializer and values-array bytecode audited. No Java was executed.')

if __name__ == '__main__':
    main()
