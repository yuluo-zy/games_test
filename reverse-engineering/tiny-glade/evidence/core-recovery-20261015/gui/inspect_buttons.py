from pathlib import Path
import sys,struct,json,hashlib,re
sys.path.insert(0,'D:/game/reverse-engineering/tiny-glade/evidence/utils-deep-followup-2026-10-14')
from pdb_reader import Pdb,z
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
from capstone.x86 import X86_OP_MEM,X86_REG_RIP,X86_OP_IMM
ROOT=Path(__file__).parent
p=Pdb('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
procs=[]
for mi,name,ss,sb in p.modules():
 if ss==65535 or not sb:continue
 for pos,k,b in p.symbols(ss,sb):
  if k not in (0x1110,0x110f,0x1147,0x1146):continue
  size=struct.unpack_from('<I',b,12)[0];off,seg=struct.unpack_from('<IH',b,28)
  name=z(b[35:])
  if seg==1 and off+0x140001000 in (0x142341fe0,0x1423427f0,0x142342f30,0x140870570,0x1408705b0,0x140b596b0,0x140b59720,0x14090e500):
   procs.append(dict(va=hex(0x140001000+off),size=size,name=name,module=mi))
(ROOT/'toolbar-buttons-procedures.json').write_text(json.dumps(procs,indent=2))
data=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes()
assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',data,60)[0];ns=struct.unpack_from('<H',data,pe+6)[0];start=pe+24+struct.unpack_from('<H',data,pe+20)[0]
def offset(va):
 for i in range(ns):
  vs,rva,rawsize,raw=struct.unpack_from('<IIII',data,start+i*40+8)
  if rva<=va-0x140000000<rva+vs:return raw+va-0x140000000-rva
 raise ValueError(hex(va))
pubs=json.load(open('D:/game/reverse-engineering/tiny-glade/evidence/render-architecture/public-symbols.json'))
names={}
for x in pubs:
 n=x['demangled']
 if len(n)<350 and x.get('va'):names.setdefault(int(x['va'],16),[]).append(n)
md=Cs(CS_ARCH_X86,CS_MODE_64);md.detail=True
for proc in procs:
 va=int(proc['va'],16);lines=[]
 for ins in md.disasm(data[offset(va):offset(va)+proc['size']],va):
  line=f'{ins.address:#x} {ins.mnemonic:8} {ins.op_str}'
  for op in ins.operands:
   if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
    target=ins.address+ins.size+op.mem.disp
    try:
     st=re.match(rb'[ -~]{3,}',data[offset(target):offset(target)+100])
     if st:line+=f' ; @{target:#x} {st[0].decode()!r}'
    except ValueError:pass
   elif op.type==X86_OP_IMM and ins.mnemonic in ('call','jmp'):
    line+=' ; '+ ' / '.join(names.get(op.imm,[])[:2])
  lines.append(line)
 (ROOT/f'{va:#x}.asm').write_text('\n'.join(lines))
print(f'saved {len(procs)} bounded procedure records')
