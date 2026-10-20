from pathlib import Path
import sys, struct, json
sys.path.insert(0,str(Path(__file__).parents[2]/'utils-deep-followup-2026-10-14'))
from pdb_reader import Pdb,z
out=Path(__file__).parent;p=Pdb('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
db=p.stream(3);ss=struct.unpack_from('<H',db,20)[0];s=p.stream(ss);q=0;n=0;hits=[]
while q<len(s):
 ln,k=struct.unpack_from('<HH',s,q);b=s[q+4:q+ln+2];n+=1
 if k==0x110e:
  name=z(b[10:])
  if 'roof_support_beam' in name.lower() or 'roofsupportbeam' in name.lower():
   off,seg=struct.unpack_from('<IH',b,4);hits.append(dict(name=name,section=seg,offset=off,va=hex(0x140001000+off) if seg==1 else None))
 q+=ln+2
(out/'public-symbols.json').write_text(json.dumps(dict(symbol_stream=ss,total_records=n,matches=hits),indent=2),encoding='utf8')
print('records',n,'matches',len(hits))
for x in hits:print(x['va'],x['name'])
