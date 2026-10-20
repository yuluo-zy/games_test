from pathlib import Path
import json,re
O=Path(__file__).resolve().parent;R=Path('D:/game')
for name in ['140b2cb30','140b2be70']:
 p=R/f'reverse-engineering/tiny-glade/evidence/country-core-deep/wall/decomp/decomp-0x{name}.txt'
 raw=p.read_text();s=json.loads(raw)['decompiled_code'] if raw.lstrip().startswith('{') else raw
 s=s.replace('\r\n','\n')
 while '\n\n' in s:s=s.replace('\n\n','\n')
 (O/f'extract-{name}.c').write_text(s)
 print(name,s[:200] if name=='140b2be70' else s)
