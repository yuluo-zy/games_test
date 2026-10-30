import hashlib,json,struct,pefile
from unicorn import *
from unicorn.x86_const import *
raw=open('B版本内容/tiny-glade.rne','rb').read();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase

def run(points,range_,key,flag,coord):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x500000);rbp=0x30100000;mask=0x30001000;last=[]
 w=lambda a,b:u.mem_write(a,b);q=lambda a,v:w(a,struct.pack('<Q',v));read=lambda a,n:bytes(u.mem_read(a,n))
 n=max(key+1,2);q(mask+8,0x30002000);q(mask+0x10,n);q(mask+0x20,0x30003000);q(mask+0x28,n);q(mask+0x38,0x30004000);q(mask+0x40,1);q(mask+0x50,0x30005000);q(mask+0x58,1);q(mask+0x68,0x30006000);q(mask+0x70,1);w(mask+0x78,struct.pack('<iii',*range_));w(0x30002000+key,bytes([flag]));w(0x30003000+key*8,struct.pack('<if',*coord));q(0x30006000,key)
 w(0x30010000,b''.join(struct.pack('<3f',*v) for v in points));q(0x30008000-0x210,0x30010000);q(0x30008000-0x208,len(points));q(rbp+0x1c8,0x30008000);q(rbp-8,len(points));w(rbp+0x90,struct.pack('<I',2))
 for r,v in [(UC_X86_REG_RSP,rbp-0x1000),(UC_X86_REG_RBP,rbp),(UC_X86_REG_R13,mask),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(r,v)
 def hook(c,pc,size,_):
  last.append(pc);del last[:-25]
  if pc==0x1407542bd:c.emu_stop()
 u.hook_add(UC_HOOK_CODE,hook)
 try:u.emu_start(0x140753f66,0x1407542be,count=500000)
 except Exception as e:raise Exception((str(e),hex(u.reg_read(UC_X86_REG_RIP)),list(map(hex,last))))
 assert u.reg_read(UC_X86_REG_RIP)==0x1407542bd,list(map(hex,last));return read(rbp+0x124,12).hex()
pts=[[0.,0.,0.],[4.,0.,0.],[8.,0.,0.]]
cases=[(pts,[0,3,4],0,2,[0,0.]),(pts,[0,3,4],2,4,[1,1.]),(pts,[0,3,4],0,0,[0,0.]),(pts,[0,3,3],2,4,[1,1.]),(pts,[0,3,4],1,2,[1,.5])]
short=[[0.,0.,0.],[.1,0.,0.]]
cases += [(short,[0,2,2],0,2,[0,0.]),(short,[0,2,2],1,4,[0,1.]),(short,[1,2,2],0,2,[0,0.]),(short,[-2,0,2],0,2,[0,0.])]
for i,c in enumerate(cases):print(json.dumps({'case':i,'input':c,'original':run(*c)}))
