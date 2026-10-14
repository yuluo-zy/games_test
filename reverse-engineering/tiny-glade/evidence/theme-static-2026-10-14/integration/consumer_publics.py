import json
from pathlib import Path
root=Path('D:/game/reverse-engineering/tiny-glade')
functions=json.loads((root/'evidence/multi-version/B-functions.json').read_text())['functions']
addresses={'0x1400570b0','0x140b46480','0x140310740','0x1403d0f70','0x1403d1750'}
rows=[f for f in functions if f['va'] in addresses]
(root/'evidence/theme-static-2026-10-14/integration/consumer-callee-publics.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
for f in rows:print(f['va'],f['name'])
