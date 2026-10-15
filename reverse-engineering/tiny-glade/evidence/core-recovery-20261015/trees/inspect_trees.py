from pathlib import Path
import sys,struct,json
sys.path.insert(0,'D:/game/reverse-engineering/tiny-glade/evidence/utils-deep-followup-2026-10-14')
from pdb_reader import Pdb,z
p=Pdb('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
wanted={0x140eec970,0x140a38560,0x140a385a0};out=[]
for mi,mod,ss,sb in p.modules():
 if ss==65535 or not sb:continue
 for pos,k,b in p.symbols(ss,sb):
  if k not in (0x1110,0x110f,0x1147,0x1146):continue
  off,seg=struct.unpack_from('<IH',b,28);name=z(b[35:]);va=0x140001000+off
  if seg==1 and (va in wanted or name.startswith('system_tree::') or 'tree_storage::' in name and ('FunctionSystem' in name or len(name)<300)):out.append(dict(va=hex(va),size=struct.unpack_from('<I',b,12)[0],name=name,module=mi))
Path(__file__).with_name('helper-procedures.json').write_text(json.dumps(out,indent=1))
print('\n'.join(x['va']+' '+x['name'] for x in out if x['name'].startswith('system_tree::') or int(x['va'],16) in wanted))
