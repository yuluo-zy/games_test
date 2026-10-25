from pathlib import Path
import sys,struct,json
sys.path.insert(0,'D:/game/reverse-engineering/tiny-glade/evidence/utils-deep-followup-2026-10-14')
from pdb_reader import Pdb,z
p=Pdb('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
wanted={0x140b883f0,0x140d597e0,0x140d5a150,0x1415ce710,0x1415cc100,0x1415bcdd0,0x141597c00,0x141595fe0,0x141596d90};out=[]
for mi,mod,ss,sb in p.modules():
 if ss==65535 or not sb:continue
 for pos,k,b in p.symbols(ss,sb):
  if k not in (0x1110,0x110f,0x1147,0x1146):continue
  off,seg=struct.unpack_from('<IH',b,28);name=z(b[35:]);va=0x140001000+off
  if seg==1 and (any(va<=a<va+struct.unpack_from('<I',b,12)[0] for a in (0x141ebb5ad,0x1405a3316,0x1405a39c0)) or va in wanted or 'NextSessionKind' in name or 'MetaCommandFence' in name and len(name)<250):out.append(dict(va=hex(va),size=struct.unpack_from('<I',b,12)[0],name=name,module=mi))
Path(__file__).with_name('helper-procedures.json').write_text(json.dumps(out,indent=1))
print('\n'.join(x['va']+' '+x['name'] for x in out if any(int(x['va'],16)<=a<int(x['va'],16)+x['size'] for a in (0x141ebb5ad,0x1405a3316,0x1405a39c0))))
