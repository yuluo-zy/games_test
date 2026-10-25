from pathlib import Path
import sys,struct,json
sys.path.insert(0,'D:/game/reverse-engineering/tiny-glade/evidence/utils-deep-followup-2026-10-14')
from pdb_reader import Pdb,z
p=Pdb('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb');out=[]
for mi,mod,ss,sb in p.modules():
 if ss==65535 or not sb:continue
 for pos,k,b in p.symbols(ss,sb):
  if k not in (0x1110,0x110f,0x1147,0x1146):continue
  size=struct.unpack_from('<I',b,12)[0];off,seg=struct.unpack_from('<IH',b,28);name=z(b[35:])
  if seg==1 and ('resources::session::' in name and len(name)<350 or off+0x140001000 in (0x141329ca0,0x14159ed30,0x141598750,0x1415cc860,0x1415b1eb0,0x14159b6c0)):
   out.append(dict(va=hex(0x140001000+off),size=size,name=name,module=mi))
Path('D:/game/reverse-engineering/tiny-glade/evidence/core-recovery-20261015/session/procedures.json').write_text(json.dumps(out,indent=1))
print('\n'.join(f"{x['va']} {x['name']}" for x in out if 'ActiveSession' in x['name'] or 'SessionMeta' in x['name'] or x['va'] in ('0x141329ca0','0x14159ed30')))
