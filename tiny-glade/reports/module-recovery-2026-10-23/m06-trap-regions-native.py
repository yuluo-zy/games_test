import pefile,hashlib,struct,random
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;img=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(img)+4095)&~4095);u.mem_write(base,img);D=0x20000000;u.mem_map(D,0x10000);outputs=[]
def hook(u,a,size,data):
 if a in (0x141cf6d60,0x140ee3fd0):
  if a==0x140ee3fd0:outputs.append(bytes(u.mem_read(u.reg_read(UC_X86_REG_R8),0x48)))
  else:u.reg_write(UC_X86_REG_RAX,0xabcdef)
  rsp=u.reg_read(UC_X86_REG_RSP);dest=struct.unpack('<Q',u.mem_read(rsp,8))[0];u.reg_write(UC_X86_REG_RSP,rsp+8);u.reg_write(UC_X86_REG_RIP,dest)
u.hook_add(UC_HOOK_CODE,hook);r=random.Random(500);rows=[]
for i in range(128):
 rbp=D+0x1000;record=D+0x7000;tick=D+0x8000
 q=[r.uniform(-1,1) for _ in range(4)];xyz=[r.uniform(-20,20) for _ in range(3)];outputs.clear()
 u.mem_write(rbp-0x20,struct.pack('<8f',*(q+xyz+[1.0])));u.mem_write(rbp+0x128,struct.pack('<Q',record));u.mem_write(record-0x50,struct.pack('<I',37));u.mem_write(record-0x1c,struct.pack('<I',19));u.mem_write(record-3,b'\x03');u.mem_write(rbp+0xd0,struct.pack('<Q',99));u.mem_write(rbp+0x90,struct.pack('<Q',tick));u.mem_write(rbp+0x98,struct.pack('<Q',D+0xa000));u.reg_write(UC_X86_REG_RBP,rbp);u.reg_write(UC_X86_REG_RSP,D+0xf008);u.reg_write(UC_X86_REG_XMM7,int.from_bytes(struct.pack('<4f',-.5,-.5,0,0),'little'));u.reg_write(UC_X86_REG_XMM11,int.from_bytes(struct.pack('<4f',1.5,0,0,0),'little'))
 u.emu_start(0x1422cf521,0x1422cf7c3,count=3000);assert u.reg_read(UC_X86_REG_RIP)==0x1422cf7c3 and len(outputs)==2
 inputs=struct.unpack('<7I',struct.pack('<7f',*(q+xyz)));regions=[list(struct.unpack('<4I3I',out[0x18:0x34])) for out in outputs];rows.append((list(inputs),regions))
Path('/private/tmp/m6-trap-cases.rs').write_text('&[\n'+''.join('('+str(a).replace(' ','')+','+str(o).replace(' ','')+'),\n' for a,o in rows)+']\n');print(len(rows))
