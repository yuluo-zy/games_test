from pathlib import Path
import json
p=Path('reverse-engineering/tiny-glade/evidence/round12/wall-gpu/sheep-startup-query.json');r=json.loads(p.read_text());f=json.loads(r['results'][1]['stdout'])['functions'][0];p.with_suffix('.c').write_text(f['decompiled_code'],encoding='utf-8');print(f['signature']);print(f['body_bytes'])
