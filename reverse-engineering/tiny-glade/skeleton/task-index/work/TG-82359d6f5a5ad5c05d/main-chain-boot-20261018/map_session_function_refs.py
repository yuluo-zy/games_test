from pathlib import Path
import re,struct,pefile,bisect,json
root=Path('/Users/yuluo/Desktop/game');raw=(root/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();p=pefile.PE(data=raw,fast_load=True)
procs=[]
for line in (root/'games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text().splitlines():
 c=line.split('\t')
 try:procs.append((int(c[0],16),int(c[1]),c[4]))
 except:pass
procs.sort();starts=[i[0] for i in procs];targets={0x14166e7d0,0x14173e110,0x1414d5490,0x142ccf9d0};hits=[]
for m in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d](....)',raw,re.S):
 try:a=p.OPTIONAL_HEADER.ImageBase+p.get_rva_from_offset(m.start())
 except:continue
 target=a+7+struct.unpack('<i',m[1])[0]
 if target in targets:
  owner=procs[bisect.bisect_right(starts,a)-1];hits.append({'site':hex(a),'target':hex(target),'owner':hex(owner[0]),'name':owner[2]})
for target in targets:
 for m in re.finditer(re.escape(struct.pack('<Q',target)),raw):
  try:a=p.OPTIONAL_HEADER.ImageBase+p.get_rva_from_offset(m.start())
  except:continue
  hits.append({'site':hex(a),'target':hex(target),'kind':'absolute-pointer'})
Path(__file__).with_suffix('.json').write_text(json.dumps(hits,indent=2));print(json.dumps(hits,indent=2))
