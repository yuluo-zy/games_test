import struct,hashlib,json,pefile
from unicorn import *
from unicorn.x86_const import *
raw=open('B版本内容/tiny-glade.rne','rb').read();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase

def run(segments):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x500000)
 w=lambda a,b:u.mem_write(a,b); q=lambda a,v:w(a,struct.pack('<Q',v)); read=lambda a,n:bytes(u.mem_read(a,n)); get=lambda a:struct.unpack('<Q',read(a,8))[0]; rbp=0x30100000;mask=0x30001000;last=[]
 for offset,ptr in [(0x30,0x30002000),(0x48,0x30004000),(0x60,0x30005000)]:q(mask+offset,128);q(mask+offset+8,ptr);q(mask+offset+16,0)
 for i,(positions,heights,us,first,lastidx,range_) in enumerate(segments):
  a=0x30010000+i*104; h=0x30020000+i*0x1000;pnt=0x30030000+i*0x1000;pu=0x30040000+i*0x1000;n=len(positions)
  for off,ptr,vals,fmt in [(0,h,heights,'f'),(0x18,pnt,positions,'3f'),(0x30,pu,us,'f')]:
   q(a+off,n);q(a+off+8,ptr);q(a+off+16,n)
   for j,v in enumerate(vals):w(ptr+j*(12 if fmt=='3f' else 4),struct.pack('<'+fmt,*(v if fmt=='3f' else [v])))
  q(a+0x48,first);q(a+0x50,lastidx);w(a+0x58,struct.pack('<iii',*range_))
 q(rbp+0x2d8,mask+0x60);q(rbp+0x2e8,mask)
 for reg,val in [(UC_X86_REG_RBP,rbp),(UC_X86_REG_RSP,rbp-0x1000),(UC_X86_REG_RBX,mask+0x30),(UC_X86_REG_RSI,mask+0x48),(UC_X86_REG_R15,0x30010000),(UC_X86_REG_R12,0x30010000+len(segments)*104),(UC_X86_REG_XMM6,0x3f800000),(UC_X86_REG_XMM12,0x3e99999a),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(reg,val)
 def hook(c,pc,size,_):
  last.append(pc);del last[:-20]
  if pc==0x140760190:c.emu_stop()
 u.hook_add(UC_HOOK_CODE,hook)
 try:u.emu_start(0x140760c20,0x140760191,count=1000000)
 except Exception as e:raise Exception((str(e),hex(u.reg_read(UC_X86_REG_RIP)),list(map(hex,last))))
 assert u.reg_read(UC_X86_REG_RIP)==0x140760190,list(map(hex,last))
 n=get(mask+0x40);return {'proposals':read(0x30002000,n*28).hex(),'scores':read(0x30004000,get(mask+0x58)*4).hex(),'keys':list(struct.unpack('<'+'Q'*get(mask+0x70),read(0x30005000,get(mask+0x70)*8)))}
cases=[ [([[0.,0.,0.],[1.,0.,0.]],[1.,2.],[0.,.5],0,2,[0,2,4])], [([[0.,0.,0.],[.3,0.,0.]],[.3,1.],[0.,.5],0,2,[0,2,4])], [([[0.,0.,0.],[1.,0.,0.]],[1.,2.],[0.,.5],0,2,[0,2,4]),([[1.,0.,0.],[2.,0.,0.]],[3.,1.],[.5,1.],1,3,[1,3,4])]]
import random
rng=random.Random(0x75f680)
for _ in range(48):
 segs=[]; current=0
 for j in range(rng.randrange(1,5)):
  n=rng.randrange(2,5); coords=[[float(current+k),rng.randrange(-2,3)/4.,rng.randrange(-2,3)/4.] for k in range(n)]
  segs.append((coords,[rng.randrange(0,9)/4 for _ in range(n)],[rng.randrange(0,9)/8 for _ in range(n)],current,current+n,[current,current+n,16]));current+=n-1
 cases.append(segs)
for i,c in enumerate(cases):print(json.dumps({'case':i,'input':c,'original':run(c)}))
