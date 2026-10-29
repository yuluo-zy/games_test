import pefile,hashlib,struct,random
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);D=0x20000000;u.mem_map(D,0x10000)
def f32(v):return struct.unpack('<f',struct.pack('<f',v))[0]
def bits(v):return struct.unpack('<I',struct.pack('<f',v))[0]
r=random.Random(602);cases=[]
for i in range(256):
 ws=[f32(r.choice([0.,.25,.5,1.,1.2,2.5,4.])) for _ in range(9)];random=f32(r.choice([0.,0.25,0.5,0.75,0.99999994]))
 if i==0:ws=[0.]*9
 total=0.
 for w in ws:total=f32(total+w)
 rbp=D+0x1000; table=D+0x4000
 for j,w in enumerate(ws):u.mem_write(table+j*16,struct.pack('<QfI',D+0x6000+j*16,w,0))
 u.mem_write(rbp+0x680,struct.pack('<Q',table));u.mem_write(rbp+0x698,struct.pack('<Q',9))
 u.reg_write(UC_X86_REG_RBP,rbp);u.reg_write(UC_X86_REG_RSP,D+0xf008);u.reg_write(UC_X86_REG_RDX,table);u.reg_write(UC_X86_REG_RAX,144);u.reg_write(UC_X86_REG_RCX,0xffffffffffffffff)
 for reg,value in [(UC_X86_REG_XMM6,total),(UC_X86_REG_XMM0,random),(UC_X86_REG_XMM1,0.)]:u.reg_write(reg,int.from_bytes(struct.pack('<f',value)+bytes(12),'little'))
 u.emu_start(0x1422d7ef0,0x1422d7f32,count=400);assert u.reg_read(UC_X86_REG_RIP)==0x1422d7f32
 idx=(u.reg_read(UC_X86_REG_RAX)-(D+0x6000))//16;assert 0<=idx<9
 cases.append(([bits(w) for w in ws],bits(random),idx))
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-clutter-pick-native-cases.rs').write_text('&[\n'+''.join('('+str(ws).replace(' ','')+','+str(r)+','+str(i)+'),\n'for ws,r,i in cases)+']\n')
print(len(cases))
