import struct,hashlib,json,pefile,random
from unicorn import *
from unicorn.x86_const import *
raw=open('B版本内容/tiny-glade.rne','rb').read();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase

def run(points, wrapped=False):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x500000)
 w=lambda a,b:u.mem_write(a,b);q=lambda a,v:w(a,struct.pack('<Q',v));get=lambda a,n:bytes(u.mem_read(a,n));rbp=0x30100000;last=[];bricks=[];end={}
 def ret():
  sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',get(sp,8))[0]);u.reg_write(UC_X86_REG_RSP,sp+8)
 def hook(c,pc,size,_):
  last.append(pc);del last[:-20]
  if pc==0x1412e4450:
   bricks.append(get(c.reg_read(UC_X86_REG_RDX),96).hex());ret()
  elif pc==0x140a99133:
   end['best']=get(rbp+0x308,4).hex();end['pose_xy']=get(rbp+0x140,8).hex();end['pose_z']=get(rbp+0x290,4).hex();end['direction_xy']=get(rbp-0x40,8).hex();end['direction_z']=get(rbp+0x208,4).hex();c.emu_stop()
 u.hook_add(UC_HOOK_CODE,hook)
 q(rbp+0x130,1);q(rbp+0x2c0,0x30002000);q(rbp+0x2f8,100001);cap=len(points)+3; head=cap-2 if wrapped else 0; q(0x30002000,cap);q(0x30002008,0x30003000);q(0x30002010,head);q(0x30002018,len(points));
 for i,point in enumerate(points):w(0x30003000+24*((head+i)%cap),struct.pack('<6f',*point))
 for r,v in [(UC_X86_REG_RBP,rbp),(UC_X86_REG_RSP,0x30100000-0x1000),(UC_X86_REG_RDI,100002),(UC_X86_REG_R14,0),(UC_X86_REG_XMM14,0x3f800000),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(r,v)
 try:u.emu_start(0x140a98ada,0x140a99134,count=1000000)
 except Exception as e:raise Exception((str(e),hex(u.reg_read(UC_X86_REG_RIP)),list(map(hex,last))))
 assert end, list(map(hex,last));return {'bricks':bricks,**end}
cases=[[[0.,0.,0.,0.,1.,0.],[2.,0.,0.,0.,1.,0.]],[[0.,0.,0.,0.,1.,0.],[.5,0.,0.,0.,1.,0.]],[[0.,0.,0.,0.,1.,0.],[.3,0.,0.,0.,1.,0.],[1.2,0.,0.,0.,1.,0.]],[[0.,0.,0.,0.,1.,0.],[1.,0.,0.,0.,1.,0.],[1.,0.,1.,0.,1.,0.]]]
rng=random.Random(0x6b98ad)
for _ in range(64):
 points=[]; x=y=z=0.0
 for i in range(rng.randrange(2,6)):
  if i: x+=rng.randrange(-4,5)/4; y+=rng.randrange(-2,3)/4; z+=rng.randrange(-4,5)/4
  points.append([x,y,z,rng.randrange(-4,5)/4,rng.randrange(-4,5)/4,rng.randrange(-4,5)/4])
 cases.append(points)
for i,c in enumerate(cases):print(json.dumps({'case':i,'input':c,'original':run(c),'wrapped':run(c,True)}))
