#!/usr/bin/env python3
"""Dev-only regeneration from an already edited scratch tree (not application)."""
import difflib
from pathlib import Path
import sys
base, edited = map(Path, sys.argv[1:])
files = ['crates/horizon-client/src/net.rs',
         'crates/horizon-client/src/net/early_configuration.rs',
         'crates/horizon-client/src/net/early_configuration_tests.rs',
         'crates/horizon-client/src/net/early_configuration_net_tests.rs',
         'crates/horizon-protocol/src/adapter/session.rs',
         'crates/horizon-protocol/src/connection.rs']
parts = []
for name in files:
    old = (base / name).read_text() if (base / name).exists() else ''
    new = (edited / name).read_text()
    parts.append('diff --git a/{0} b/{0}\n'.format(name))
    if not old:
        parts.append('new file mode 100644\n')
    parts.extend(difflib.unified_diff(old.splitlines(True), new.splitlines(True),
                                    fromfile='a/'+name if old else '/dev/null',
                                    tofile='b/'+name))
Path(__file__).with_name('horizon-early-configuration.patch').write_text(''.join(parts).replace('\n \n', '\n\n'))
