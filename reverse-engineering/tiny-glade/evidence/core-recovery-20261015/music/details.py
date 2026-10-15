from pathlib import Path
import sys,json,struct
sys.path.insert(0,str(Path(__file__).parent))
import inspect_music as m
from capstone.x86 import X86_OP_MEM,X86_REG_RIP
out=[]
for va in (0x1408b5500,0x140a72810):
 p=next(x for x in m.procs if int(x['va'],16)==va)
 for i in m.md.disasm(m.data[m.offset(va):m.offset(va)+p['size']],va):
  for o in i.operands:
   if o.type==X86_OP_MEM and o.mem.base==X86_REG_RIP and i.mnemonic in ('ucomiss','movss','divss','mulss','subss','addss'):
    a=i.address+i.size+o.mem.disp
    out.append({'instruction':hex(i.address),'va':hex(a),'float':struct.unpack_from('<f',m.data,m.offset(a))[0]})
print(json.dumps(out,indent=1))
for a in (0x142a9fa24,0x142a9fa34):
 print(hex(a),[hex(a+x) for x in struct.unpack_from('<4i',m.data,m.offset(a))])
n=[]
for mi,mod,ss,sb in m.p.modules():
 if ss==65535 or not sb:continue
 for pos,k,b in m.p.symbols(ss,sb):
  if k not in (0x1110,0x110f,0x1147,0x1146):continue
  s=m.z(b[35:])
  if 'FunctionSystem<' in s and 'Music' in s and len(s)>250:n.append(s)
Path(__file__).with_name('parameter-types.json').write_text(json.dumps(n,indent=2))
print('\n'.join(n[:2]))
