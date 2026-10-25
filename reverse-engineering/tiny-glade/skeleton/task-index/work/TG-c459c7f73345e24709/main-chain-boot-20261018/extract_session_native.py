import hashlib,bisect
from pathlib import Path
import capstone,pefile
root=Path('/Users/yuluo/Desktop/game'); out=Path(__file__).parent
raw=(root/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
for name,a,n in [('save-resource-producer',0x1409447e0,1739),('startup-session',0x14166e7d0,18791)]:
 offset=p.get_offset_from_rva(a-p.OPTIONAL_HEADER.ImageBase)
 (out/(name+'-rne.asm')).write_text('\n'.join(f'{i.address:016x}: {i.mnemonic} {i.op_str}' for i in md.disasm(raw[offset:offset+n],a))+'\n')
 print(name,hex(a),n)
md.detail=True
lines=[]
a=0x14166e7d0;offset=p.get_offset_from_rva(a-p.OPTIONAL_HEADER.ImageBase)
for i in md.disasm(raw[offset:offset+18791],a):
 for op in i.operands:
  if op.type==capstone.x86.X86_OP_MEM and op.mem.base==capstone.x86.X86_REG_RIP:
   addr=i.address+i.size+op.mem.disp
   try: o=p.get_offset_from_rva(addr-p.OPTIONAL_HEADER.ImageBase)
   except: continue
   data=raw[o:o+96]
   text=data.split(b'\0',1)[0]
   if len(text)>2 and all(32<=c<127 for c in text):
    lines.append(f'{i.address:x}: {i.mnemonic} {i.op_str} ; {addr:x} {text.decode()!r}')
(out/'startup-session-rdata.txt').write_text('\n'.join(lines)+'\n')
print('\n'.join(lines))
