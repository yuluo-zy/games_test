"""Research harness: original full B WallConstructor business body, explicit inputs.
Not an accepted verifier until output, bridges and input provenance are closed.
"""
import json,struct,ctypes,ctypes.util,math,hashlib
from pathlib import Path
from unicorn import *
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parents[2]
b=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']['B'];raw=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256'];base=b['pe']['image_base'];cpu=Uc(UC_ARCH_X86,UC_MODE_64);end=max(s['rva']+max(s['raw_size'],s['virtual_size']) for s in b['pe']['sections']);cpu.mem_map(base,(end+4095)&~4095)
for s in b['pe']['sections']:
 if s['raw_size']:cpu.mem_write(base+s['rva'],raw[s['raw_pointer']:s['raw_pointer']+s['raw_size']])
data,stack,heap,stop=0x20000000,0x30000000,0x40000000,0x50000000
cpu.mem_map(data,0x100000);cpu.mem_map(stack,0x200000);cpu.mem_map(heap,0x1000000);cpu.mem_map(stop,0x1000);allocation=[heap];last=[]
def u64(p):return struct.unpack('<Q',cpu.mem_read(p,8))[0]
def wf(p,v):cpu.mem_write(p,struct.pack('<f',v))
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def fl(v):return struct.unpack('<f',struct.pack('<I',v&0xffffffff))[0]
def ret(v=None):
 if v is not None:cpu.reg_write(UC_X86_REG_RAX,v)
 sp=cpu.reg_read(UC_X86_REG_RSP);ra=u64(sp);cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,ra)
def alloc(n):
 p=allocation[0];allocation[0]+=max((n+15)&~15,0x1000);assert allocation[0]<heap+0x1000000;return p
lib=ctypes.CDLL(ctypes.util.find_library('m') or ctypes.util.find_library('c'))
functions={0x142923ee0:('floorf',1),0x142923e60:('ceilf',1),0x142923fd0:('sinf',1),0x142924060:('truncf',1),0x142923fa0:('roundf',1)}
for pc,(name,count)in functions.items():f=getattr(lib,name);f.argtypes=[ctypes.c_float]*count;f.restype=ctypes.c_float

def hook(uc,pc,size,user):
 if len(last)>100:last.pop(0)
 last.append(hex(pc))
 if pc==0x140c16a00:ret(0)
 elif pc in [0x140613c10,0x140613c40]:
  n=cpu.reg_read(UC_X86_REG_RCX);p=alloc(n)
  if pc==0x140613c40 and n:cpu.mem_write(p,bytes(n))
  ret(p)
 elif pc in [0x140613c20,0x14289aa40]:ret()
 elif pc==0x140613c30:
  old=cpu.reg_read(UC_X86_REG_RCX);oldn=cpu.reg_read(UC_X86_REG_RDX);newn=cpu.reg_read(UC_X86_REG_R9);p=alloc(newn);cpu.mem_write(p,bytes(cpu.mem_read(old,min(oldn,newn))));ret(p)
 elif pc in [0x142923c00,0x142923c10]:
  dst,src,n=(cpu.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);
  if n:
   try:cpu.mem_write(dst,bytes(cpu.mem_read(src,n)))
   except Exception as e:raise RuntimeError(f'CRT memory bridge pc={pc:x} dst={dst:x} src={src:x} count={n}')from e
  ret(dst)
 elif pc==0x142923c20:
  dst,value,n=(cpu.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);
  if n:cpu.mem_write(dst,bytes([value&255])*n)
  ret(dst)
 elif pc==0x142923bf0:
  a,z,n=(cpu.reg_read(r)for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);aa=bytes(cpu.mem_read(a,n))if n else b'';zz=bytes(cpu.mem_read(z,n))if n else b'';ret((aa>zz)-(aa<zz))
 elif pc in functions:
  name,count=functions[pc];f=getattr(lib,name);cpu.reg_write(UC_X86_REG_XMM0,bits(f(fl(cpu.reg_read(UC_X86_REG_XMM0)))));ret()
 elif 0x1428d8f00<=pc<=0x1428f1800:raise RuntimeError('panic '+hex(pc))
cpu.hook_add(UC_HOOK_CODE,hook)

def call(seed=27,points=None,us=None,length=12.,height=2.,holes=None,closed=True,authority_heights=None,verbose=False):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000))
 points=points or [[-2.,-1.],[-2.,1.],[2.,1.],[2.,-1.],[-2.,-1.]];us=us or [0.,1/6,.5,2/3,1.]
 n=len(points);curve=data+0x100;cpu.mem_write(curve,struct.pack('<QQQQQQf',n,data+0x1000,n,n,data+0x2000,n,length))
 cpu.mem_write(data+0x1000,b''.join(struct.pack('<2f',*p) for p in points));cpu.mem_write(data+0x2000,struct.pack('<'+'f'*n,*us))
 cpu.mem_write(data+0x3000,struct.pack('<'+'f'*n,*(authority_heights if authority_heights is not None else [height]*n)));cpu.mem_write(data+0x3100,struct.pack('<'+'f'*n,*[0.]*n));cpu.mem_write(data+0x3200,struct.pack('<'+'f'*n,*[0.]*n));cpu.mem_write(data+0x3300,struct.pack('<Q',1))
 # Original FinalizedWallHoles view: cap/ptr/count at0/8/16 length24 closed28.
 holes=holes or [];cpu.mem_write(data+0x4100,b''.join(struct.pack('<4fBBxx',*a,kind,origin)for a,kind,origin in holes));cpu.mem_write(data+0x4000,struct.pack('<QQQfB',len(holes),data+0x4100,len(holes),length,int(closed)));cpu.mem_write(data,struct.pack('<QQQ',4096,data+0x10000,0));cpu.mem_write(data+0x5000,struct.pack('<QQQ',8192,data+0x30000,0))
 cpu.mem_write(data+0x6000,struct.pack('<5fB',0.,0.,0.,0.,0.,2))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,curve);cpu.reg_write(UC_X86_REG_R8,23);cpu.reg_write(UC_X86_REG_R9,data+0x3000);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 args={0x28:n,0x30:data+0x3100,0x38:n,0x40:data+0x3200,0x48:n,0x50:2,0x58:seed,0x60:data+0x4000,0x68:0,0x70:data+0x5000,0x78:0,0x80:data+0x6000,0x88:0,0x90:0,0x98:0,0xa0:0,0xa8:bits(1.),0xb0:0}
 for o,v in args.items():cpu.mem_write(sp+o,struct.pack('<Q',v))
 try:cpu.emu_start(0x140769760,stop,count=10000000)
 except Exception as e:
  print('FAIL',e,'rip',hex(cpu.reg_read(UC_X86_REG_RIP)),'last',last[-12:]);raise
 count=u64(data+0x5010);records=[bytes(cpu.mem_read(data+0x30000+i*96,96)).hex()for i in range(count)];
 if verbose:print('records',count,'normalized_holes',u64(data+16))
 return records
if __name__=='__main__':call()
