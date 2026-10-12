"""Original B mask ->104B segment owner; generic allocation/CRT bridges only.
All inputs own disjoint buffers. Normalize owned Vec fields to contents, never
compare allocator addresses to reconstructed Rust fields.
"""
import sys,ctypes,json,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-production'))
from native_production import *
import pefile
thunk=0x142923f10
assert bytes(cpu.mem_read(thunk,2))==b'\xff\x25'
iat=thunk+6+struct.unpack('<i',cpu.mem_read(thunk+2,4))[0]
pe=pefile.PE(data=raw)
name=next((x.name.decode() for d in pe.DIRECTORY_ENTRY_IMPORT for x in d.imports if x.address==iat),None)
assert name=='fmodf',(hex(iat),name)
fmodf=lib.fmodf;fmodf.argtypes=[ctypes.c_float,ctypes.c_float];fmodf.restype=ctypes.c_float

# These functions are pinned Cargo dependency implementations; do not port TLS
# deadlock bookkeeping into the game. Their single-thread observer side effects
# do not change the original SparseWriter pool/count/data/dirty instructions.
for va,symbol in [(0x140c81c70,'acquire_resource'),(0x140c81e10,'release_resource')]:
 evidence=ROOT/'evidence/building-segmentation/B-process'/f'{va:x}.c'
 assert f'parking_lot_core::parking_lot::deadlock_impl::{symbol}' in evidence.read_text()

def crt_hook(uc,pc,size,user):
 if pc in (0x140c81c70,0x140c81e10) and cpu.reg_read(UC_X86_REG_RIP)==pc:ret()
 if pc==thunk and cpu.reg_read(UC_X86_REG_RIP)==pc:
  x=fmodf(fl(cpu.reg_read(UC_X86_REG_XMM0)),fl(cpu.reg_read(UC_X86_REG_XMM1)))
  cpu.reg_write(UC_X86_REG_XMM0,bits(x));ret()
cpu.hook_add(UC_HOOK_CODE,crt_hook)

def _block(payload):
 p=alloc(len(payload));cpu.mem_write(p,payload);return p

def materialize(flags,coords,loop_range,segment_values,arch,points,us,regular=True,closed=True):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000))
 assert len(flags)==len(coords) and 0<len(flags)<=65536
 assert loop_range[1]-loop_range[0]==len(flags) and loop_range[2]>0
 assert len(points)==len(us) and len(points)>1
 flagptr=_block(bytes(flags));coordptr=_block(b''.join(struct.pack('<If',i,t)for i,t in coords))
 valueptr=_block(struct.pack('<'+'f'*len(segment_values),*segment_values));archptr=_block(struct.pack('<'+'f'*len(arch),*arch));pointptr=_block(b''.join(struct.pack('<3f',*p)for p in points));uptr=_block(struct.pack('<'+'f'*len(us),*us))
 mask=data+0x100;cpu.mem_write(mask,struct.pack('<QQQQQQ',len(flags),flagptr,len(flags),len(coords),coordptr,len(coords)));cpu.mem_write(mask+0x78,struct.pack('<3i',*loop_range));cpu.mem_write(mask+0x84,struct.pack('<f',0.))
 space=data+0x200;cpu.mem_write(space,struct.pack('<If',int(closed),1.))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,mask);cpu.reg_write(UC_X86_REG_R8,valueptr);cpu.reg_write(UC_X86_REG_R9,len(segment_values));cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 args={0x28:archptr,0x30:len(arch),0x38:pointptr,0x40:len(points),0x48:uptr,0x50:len(us),0x58:int(regular),0x60:space}
 for offset,v in args.items():cpu.mem_write(sp+offset,struct.pack('<Q',v))
 cpu.emu_start(0x14122ff90,stop,count=10000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop,'segment owner did not return'
 cap,ptr,n=struct.unpack('<3Q',cpu.mem_read(data,24));assert n<=cap
 if n:assert heap<=ptr and ptr+cap*104<=heap+0x1000000
 global last_materialized_views
 last_materialized_views=[ptr+104*i for i in range(n)]
 output=[]
 for index in range(n):
  p=ptr+104*index;vectors=[]
  for offset,width in [(0,4),(24,12),(48,4)]:
   cap,arr,count=struct.unpack('<3Q',cpu.mem_read(p+offset,24));assert count<=cap
   payload=bytes(cpu.mem_read(arr,count*width)) if count else b'';vectors.append(struct.pack('<Q',count)+payload)
  output.append(b''.join(vectors)+bytes(cpu.mem_read(p+72,28)))
 return output

if __name__=='__main__':
 case=dict(flags=[8,0,0,8],coords=[(0,0),(1,0),(2,0),(2,1)],loop_range=(0,4,4),segment_values=[.3,.6,.8,.4],arch=[2.,2.,2.,2.],points=[[0.,0.,0.],[1.,0.,0.],[2.,0.,0.],[3.,0.,0.]],us=[0.,1/3,2/3,1.],regular=True,closed=False)
 result=materialize(**case);print(json.dumps({'count':len(result),'canonical_hex':[x.hex()for x in result],'CRT_bridge':{'thunk':hex(thunk),'iat':hex(iat),'name':name}}))

def create_sparse_writer(pages=64):
 """Actual64B writer state:32 slots/page, real free-pool and dirty bitsets.
 Returned views read actual original writes; no game push function is bridged.
 """
 assert 1<=pages<=4096
 counts=_block(bytes(pages));dirty=_block(bytes(((pages+63)//64)*8));allocation_dirty=_block(bytes(((pages+63)//64)*8));records=alloc(pages*32*64)
 owned_ptr=_block(struct.pack('<I',0)+bytes((pages-1)*4));owned=_block(struct.pack('<QQQ',pages,owned_ptr,1))
 freeptr=_block(b''.join(struct.pack('<I',i)for i in range(1,pages)));pool=_block(struct.pack('<QQQQ',0,pages-1,freeptr,pages-1))
 writer=_block(struct.pack('<QQQQQQQQQQI',counts,pages,allocation_dirty,(pages+63)//64,owned,records,pages*32,pool,dirty,(pages+63)//64,0))
 return dict(writer=writer,counts=counts,records=records,owned=owned,dirty=dirty,allocation_dirty=allocation_dirty,pool=pool,pages=pages)

def sparse_records(view):
 cap,p,n=struct.unpack('<3Q',cpu.mem_read(view['owned'],24));assert n<=cap
 pages=struct.unpack('<'+'I'*n,cpu.mem_read(p,n*4));out=[]
 for page in pages:
  count=bytes(cpu.mem_read(view['counts']+page,1))[0];assert count<=32
  out.extend(bytes(cpu.mem_read(view['records']+(page*32+i)*64,64))for i in range(count))
 return out

def produce_support_instances(heights,ground,raised,flags,coords,wall_id,ordinal,closed=True):
 """Entire original141242530 -> actual original64B SparseWriter storage.
 Heights/ground/raised/flags/coords are its true caller ABI views. Their raw
 source preparation is verified separately; do not expose derived views toWorld.
 """
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000))
 n=len(heights);assert 2<=n<=65536 and len(ground)==len(raised)==len(flags)==len(coords)==n
 hptr=_block(struct.pack('<'+'f'*n,*heights));gptr=_block(b''.join(struct.pack('<3f',*p)for p in ground));rptr=_block(b''.join(struct.pack('<3f',*p)for p in raised));fptr=_block(bytes(flags));cptr=_block(b''.join(struct.pack('<If',i,t)for i,t in coords))
 view=create_sparse_writer(64);public=data+0x100;cpu.mem_write(public+0x258,struct.pack('<I',0 if closed else 2));growth=data+0x400;env=_block(struct.pack('<QQ',view['writer'],growth))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,wall_id);cpu.reg_write(UC_X86_REG_RDX,public);cpu.reg_write(UC_X86_REG_R8,hptr);cpu.reg_write(UC_X86_REG_R9,n);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 args={0x28:gptr,0x30:n,0x38:rptr,0x40:n,0x48:env,0x50:fptr,0x58:n,0x60:cptr,0x68:n,0x70:ordinal}
 for offset,v in args.items():cpu.mem_write(sp+offset,struct.pack('<Q',v))
 cpu.emu_start(0x141242530,stop,count=10000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop,'support owner did not return'
 return sparse_records(view),bytes(cpu.mem_read(growth,1))[0]

def rdp_indices(points):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000))
 p=_block(b''.join(struct.pack('<3f',*row)for row in points));sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.mem_write(sp+0x28,struct.pack('<f',1.));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,p);cpu.reg_write(UC_X86_REG_R8,len(points));cpu.reg_write(UC_X86_REG_XMM3,bits(.2));cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 cpu.emu_start(0x141fe0b90,stop,count=10000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop,'privateRdp did not return'
 cap,p,n=struct.unpack('<3Q',cpu.mem_read(data,24));assert n<=cap
 return list(struct.unpack('<'+'Q'*n,cpu.mem_read(p,n*8)))if n else[]

def prepare_current_lintel(segment,prepared_points,public_284,public_288):
 """Actual contiguous caller14076e65c..86e including original141204400.
 Runs on the real materializer's segment allocation; it replaces height Vec.
 """
 public=data+0x800;pp=_block(b''.join(struct.pack('<3f',*p)for p in prepared_points));cpu.mem_write(public+0x90,struct.pack('<QQ',pp,len(prepared_points)))
 cap,hp,n=struct.unpack('<3Q',cpu.mem_read(segment,24));cap,gp,gn=struct.unpack('<3Q',cpu.mem_read(segment+24,24));assert n==gn and n>=2
 bp=stack+0x100800;sp=stack+0x100000;cpu.reg_write(UC_X86_REG_RBP,bp);cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_R15,segment);cpu.reg_write(UC_X86_REG_RSI,hp);cpu.reg_write(UC_X86_REG_RDX,gp);cpu.reg_write(UC_X86_REG_RCX,gp+4);cpu.reg_write(UC_X86_REG_R13,gp);cpu.reg_write(UC_X86_REG_R14,n);cpu.reg_write(UC_X86_REG_RAX,n);cpu.reg_write(UC_X86_REG_RDI,data+0xa00)
 cpu.mem_write(bp+0x1e8,struct.pack('<Q',public));cpu.reg_write(UC_X86_REG_XMM8,bits(public_284));cpu.reg_write(UC_X86_REG_XMM9,bits(public_288));cpu.reg_write(UC_X86_REG_XMM11,bits(-.2));cpu.reg_write(UC_X86_REG_XMM12,0);cpu.reg_write(UC_X86_REG_XMM13,0);cpu.reg_write(UC_X86_REG_XMM14,int.from_bytes(struct.pack('<4f',*([-.2]*4)),'little'));cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 cpu.emu_start(0x14076e65c,0x14076e86e,count=10000000);assert cpu.reg_read(UC_X86_REG_RIP)==0x14076e86e,'caller preparation did not reach edge'
 _,hp,n=struct.unpack('<3Q',cpu.mem_read(segment,24));_,rp,rn=struct.unpack('<3Q',cpu.mem_read(bp+0xc0,24));assert n==rn
 heights=list(struct.unpack('<'+'f'*n,cpu.mem_read(hp,n*4)));ground=[list(struct.unpack('<3f',cpu.mem_read(gp+i*12,12)))for i in range(n)];raised=[list(struct.unpack('<3f',cpu.mem_read(rp+i*12,12)))for i in range(n)]
 return heights,ground,raised

def current_third_accumulation(segments,third):
 """Actual caller14076f1f4..2e7 loop and LoopRange indexing helpers.
 Prepared segment height Vec must have already been replaced by callerprepare.
 """
 thirdptr=_block(struct.pack('<'+'f'*len(third),*third));pubend=data+0xc00;cpu.mem_write(pubend-0x1a8,struct.pack('<QQ',thirdptr,len(third)))
 bp=stack+0x100800;cpu.reg_write(UC_X86_REG_RBP,bp);cpu.reg_write(UC_X86_REG_RSP,stack+0x100000);cpu.reg_write(UC_X86_REG_R15,pubend);cpu.reg_write(UC_X86_REG_XMM15,bits(-.11250000447034836));cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 # Actual output materializer allocated an ordered contiguous104B vector.
 assert all(b==segments[0]+104*i for i,b in enumerate(segments))
 if not segments:return third
 cpu.reg_write(UC_X86_REG_RDI,segments[0]);cpu.mem_write(bp+0x240,struct.pack('<Q',segments[-1]+104))
 cpu.emu_start(0x14076f1f4,0x14076f2e7,count=10000000);assert cpu.reg_read(UC_X86_REG_RIP)==0x14076f2e7,'third writer did not complete'
 return list(struct.unpack('<'+'f'*len(third),cpu.mem_read(thirdptr,len(third)*4)))

def produce_support_batch(sources,wall_id,closed=True):
 """Multiple original CPU calls share one real SparseWriter/pool/count/dirty.
 Source tuples are actual preparation outputs with their per-mask ordinal.
 """
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));view=create_sparse_writer(256);public=data+0x100;cpu.mem_write(public+0x258,struct.pack('<I',0 if closed else 2));growth=data+0x400;env=_block(struct.pack('<QQ',view['writer'],growth))
 for heights,ground,raised,ordinal in sources:
  n=len(heights);flags=[8]+[0]*(n-2)+[8];coords=[(0,0)]*(n-1)+[(n-2,1)]
  hptr=_block(struct.pack('<'+'f'*n,*heights));gptr=_block(b''.join(struct.pack('<3f',*p)for p in ground));rptr=_block(b''.join(struct.pack('<3f',*p)for p in raised));fptr=_block(bytes(flags));cptr=_block(b''.join(struct.pack('<If',i,t)for i,t in coords))
  sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,wall_id);cpu.reg_write(UC_X86_REG_RDX,public);cpu.reg_write(UC_X86_REG_R8,hptr);cpu.reg_write(UC_X86_REG_R9,n);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
  for offset,v in {0x28:gptr,0x30:n,0x38:rptr,0x40:n,0x48:env,0x50:fptr,0x58:n,0x60:cptr,0x68:n,0x70:ordinal}.items():cpu.mem_write(sp+offset,struct.pack('<Q',v))
  cpu.emu_start(0x141242530,stop,count=10000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop,'support batch owner did not return'
 assert bytes(cpu.mem_read(growth,1))[0]==0,'native batch exceeded fixture pool'
 return sparse_records(view)
