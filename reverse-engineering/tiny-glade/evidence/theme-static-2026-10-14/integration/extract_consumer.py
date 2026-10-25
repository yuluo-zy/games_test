import json
from pathlib import Path
out=Path('D:/game/reverse-engineering/tiny-glade/evidence/theme-static-2026-10-14/integration')
j=json.loads((out/'consumer-prepare.artifacts/001.stdout.txt').read_text())
for f in j['functions']:
 (out/(f['address']+'.c')).write_text(f.get('decompiled_code',''),encoding='utf-8')
