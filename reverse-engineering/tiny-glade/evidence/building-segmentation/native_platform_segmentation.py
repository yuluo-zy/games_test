"""Original resolved per-mask platform business; actualoriginal calc masks."""
import struct
from native_segmentation import produce
from native_fixture import *
def call(entry,args,xmm=None,stack_args=()):
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop))
 for i,v in enumerate(stack_args):cpu.mem_write(sp+0x28+i*8,struct.pack('<Q',v))
 cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 for reg,v in args:cpu.reg_write(reg,v)
 if xmm is not None:cpu.reg_write(UC_X86_REG_XMM2,xmm)
 try:cpu.emu_start(entry,stop,count=5000000)
 except Exception as e:raise RuntimeError(f'{e} rip={cpu.reg_read(UC_X86_REG_RIP):x} last={last[-10:]}')from e
 assert cpu.reg_read(UC_X86_REG_RIP)==stop

def result_masks(vec):
 cap,ptr,n=struct.unpack('<3Q',cpu.mem_read(vec,24));out=[]
 for i in range(n):
  raw=bytes(cpu.mem_read(ptr+i*136,136));fc,fp,fn,cc,cp,cn=struct.unpack_from('<6Q',raw);out.append({'flags':bytes(cpu.mem_read(fp,fn)).hex(),'coords':bytes(cpu.mem_read(cp,cn*8)).hex(),'range':raw[120:132].hex(),'length':raw[132:136].hex()})
 return out

def process(points,values,shape,special,maximum,roof_word=None,shape_fields=(.2,2.,3.)):
 original=produce(points,values,shape,special,shape_fields)
 cpu.mem_write(data+0x108,struct.pack('<IfIf',1,maximum,1,99.));n=len(points)
 flat=False
 if roof_word is not None:
  cpu.mem_write(data+0x9000+0x30,struct.pack('<I',roof_word));call(0x1408e3380,[(UC_X86_REG_RCX,data+0x9000)]);flat=bool(cpu.reg_read(UC_X86_REG_RAX)&0xff)
 if shape>=2 or not flat:return original,[]
 collected=[];steps=[]
 for i,mask in enumerate(original):
  raw=bytes.fromhex(mask['raw']);mp=data+0x10000+i*136;cpu.mem_write(mp,raw);rangebytes=raw[120:132]
  view=data+0x6000;cpu.mem_write(view,struct.pack('<QQ',data+0x2000,n)+rangebytes+bytes(4)+struct.pack('<Q',data));hv=data+0x5000
  call(0x141202370,[(UC_X86_REG_RCX,hv),(UC_X86_REG_RDX,view),(UC_X86_REG_R8,data+0x8000)])
  mapper=data+0x6100;cpu.mem_write(mapper,struct.pack('<8Q',data+0x2000,data+0x2000+n*12,data+0x3000,data+0x3000+n*4,0,n,n,data));lift=data+0x5020
  call(0x141205db0,[(UC_X86_REG_RCX,lift),(UC_X86_REG_RDX,mapper),(UC_X86_REG_R8,data+0x8000)])
  masks=data+0x5040;sv=data+0x5060;cpu.mem_write(masks,struct.pack('<3Q',0,8,0));cpu.mem_write(sv,struct.pack('<3Q',0,8,0));callback=data+0x6200;cpu.mem_write(callback,struct.pack('<4Q',mp,masks,data,lift));_,hp,hn=struct.unpack('<3Q',cpu.mem_read(hv,24))
  call(0x1411ffbd0,[(UC_X86_REG_RCX,hp),(UC_X86_REG_RDX,hn),(UC_X86_REG_R9,callback)],bits(3.),(sv,mp))
  collected.extend(result_masks(masks));_,sp,sn=struct.unpack('<3Q',cpu.mem_read(sv,24))
  for j in range(sn):
   bts=bytes(cpu.mem_read(sp+j*32,28));start,end=struct.unpack_from('<2Q',bts);steps.append({'start':start,'end':end,'range':bts[16:28].hex()})
 return collected,steps
if __name__=='__main__':print(process([[0.,0.,0.],[1.,.2,0.],[2.,.4,0.]],[0.,.5,0.],0,0,4.,0))
