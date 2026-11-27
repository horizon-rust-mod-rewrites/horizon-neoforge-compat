#!/usr/bin/env python3
"""Download only the public NeoForge SOURCE artifact in memory; no Java or scripts run.
No Minecraft server/client jar, mappings bundle, proprietary assets, or mod jars fetched.
"""
from pathlib import Path
import hashlib, io, json, urllib.request, zipfile
m = json.loads((Path(__file__).parent / 'neoforge-21.1.248.json').read_text())
b = urllib.request.urlopen(m['artifact'], timeout=60).read(4 * 1024 * 1024)
assert hashlib.sha256(b).hexdigest() == m['sha256'], 'artifact hash mismatch'
official = urllib.request.urlopen(m['publisher_checksum_url'], timeout=30).read(256).decode().strip()
assert official == m['sha256'], 'publisher checksum changed'
with zipfile.ZipFile(io.BytesIO(b)) as z:
    for name, expected in m['source_files'].items():
        assert hashlib.sha256(z.read(name)).hexdigest() == expected, name
print(f"Verified NeoForge 21.1.248 source artifact and {len(m['source_files'])} inspected Java files")
