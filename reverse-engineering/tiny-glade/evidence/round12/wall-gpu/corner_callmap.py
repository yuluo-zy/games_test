from pathlib import Path
import re,json,hashlib
r=Path('D:/game');e=r/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu'
asm=(e/'corners-full-original.asm').read_text(encoding='utf-8');targets={int(x,16) for x in re.findall(r'callq\s+0x([0-9a-f]+)',asm)};names={x:[] for x in targets}
for line in (r/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').open(encoding='utf-8'):
 parts=line.rstrip().split('\t',4)
 if len(parts)<5:continue
 va=int(parts[0],16)
 if va in names:names[va].append({'bytes':int(parts[1]),'module':int(parts[2]),'object':parts[3],'symbol':parts[4]})
(e/'corners-private-call-map.json').write_text(json.dumps({hex(k):v for k,v in sorted(names.items())},indent=2),encoding='utf-8')
for k,v in sorted(names.items()):
 keep=[t for t in v if t['module'] in range(1400,1500)]
 if not keep:keep=[t for t in v if any(n in t['symbol'] for n in ['wall_space','random_split','InProgressHole','Curve::','Triangle','WallStyle'])]
 if keep:print(hex(k),'; '.join(t['symbol'] for t in keep))
