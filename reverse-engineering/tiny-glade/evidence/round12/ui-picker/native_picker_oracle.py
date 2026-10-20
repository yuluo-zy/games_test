"""Unchanged B async host: executes game closure plus original private geometry.
Bridges only Rust allocation/Vec reserve and CRT memory; no picker branches.
"""
from pathlib import Path
import pefile,struct,hashlib,json
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);cpu.mem_write(base,pe.get_memory_mapped_image())
DATA,STACK,HEAP,STOP=0x20000000,0x30000000,0x40000000,0x50000000
for p,n in [(DATA,0x100000),(STACK,0x200000),(HEAP,0x1000000),(STOP,0x1000)]:cpu.mem_map(p,n)
heap=HEAP
def u64(a):return struct.unpack('<Q',cpu.mem_read(a,8))[0]
def put(a,fmt,*v):cpu.mem_write(a,struct.pack(fmt,*v))
def alloc(n):
 global heap
 p=heap;heap+=max((n+15)&~15,0x100);assert heap<HEAP+0x1000000;return p
def ret(v=None):
 if v is not None:cpu.reg_write(UC_X86_REG_RAX,v)
 sp=cpu.reg_read(UC_X86_REG_RSP);pc=u64(sp);cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,pc)
trace=[]
def hook(c,pc,size,user):
 trace.append(pc)
 if pc==STOP:c.emu_stop();return
 if pc==0x140613c10:ret(alloc(c.reg_read(UC_X86_REG_RCX)))
 elif pc in (0x140613c20,0x14289aa40):ret()
 elif pc in (0x1406319d0,0x1405627b0):
  ptr=c.reg_read(UC_X86_REG_RCX);stride=56 if pc==0x1406319d0 else 64;cap=max(4,u64(ptr)*2);p=alloc(cap*stride);old=u64(ptr+8);n=u64(ptr+16)
  if n:c.mem_write(p,bytes(c.mem_read(old,n*stride)))
  put(ptr,'<QQ',cap,p);ret()
 elif pc in (0x142923c00,0x142923c10):
  dst,src,n=[c.reg_read(r) for r in (UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8)]
  if n:c.mem_write(dst,bytes(c.mem_read(src,n)))
  ret(dst)
cpu.hook_add(UC_HOOK_CODE,hook)
def run(widgets,origin=(0,0,0),direction=(0,0,1),screen=(0,0),terrain=None):
 global heap
 heap=HEAP;trace.clear();cpu.mem_write(DATA,bytes(0x10000));cpu.mem_write(STACK,bytes(0x200000))
 out,state,env,ray,rayrefs,ssref,ss,trref,tr,items= [DATA+i*0x100 for i in range(10)]
 put(ray,'<6f',*origin,*direction);put(rayrefs,'<QQ',ray,ray+12);put(ssref,'<Q',ss);put(ss,'<2f',*screen);put(trref,'<Q',tr)
 if terrain is not None:put(tr,'<I3f',1,*terrain)
 put(env,'<4Q',ray,rayrefs,ssref,trref);put(state,'<3Q',env,items,len(widgets))
 for i,(wid,b,extra) in enumerate(widgets):
  wp=DATA+0x2000+i*0x200;cpu.mem_write(wp,b);put(items+i*16,'<QQ',wid,wp)
  if extra:
   arr=DATA+0x8000+i*0x400;put(wp+0x90,'<3Q',len(extra),arr,len(extra));cpu.mem_write(arr,b''.join(struct.pack('<'+'f'*len(x),*x)for x in extra))
 for r in (UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15):cpu.reg_write(r,0)
 for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(r,0)
 sp=STACK+0x100008;put(sp,'<Q',STOP)
 for r,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,out),(UC_X86_REG_RDX,state),(UC_X86_REG_MXCSR,0x1f80),(UC_X86_REG_RFLAGS,0x202)]:cpu.reg_write(r,v)
 try:cpu.emu_start(0x140ac3f50,STOP,count=1000000)
 except Exception as e:raise RuntimeError(f'{e} @ {cpu.reg_read(UC_X86_REG_RIP):x} trace '+str([hex(t)for t in trace[-15:]]))
 assert cpu.reg_read(UC_X86_REG_RIP)==STOP
 result={}
 for name,voff,stride in [('ordinary',0,56),('canvas',24,64)]:
  p,n=u64(out+voff+8),u64(out+voff+16)
  result[name]=[bytes(cpu.mem_read(p+i*stride,stride)).hex()for i in range(n)]
 return result
def widget(tag,payload,layer=0,occlude=True,flags=1):
 b=bytearray(176);b[0x88]=tag;b[0x89]=layer;b[0xa8]=occlude;b[0xa9]=flags
 for off,values in payload.items():struct.pack_into('<'+'f'*len(values),b,off,*values)
 return bytes(b)
if __name__=='__main__':
 cases=[]
 def add(name,w,**kw):
  result=run(w,**kw);cases.append(dict(name=name,widgets=[dict(id=i,bytes=b.hex(),extra=e)for i,b,e in w],inputs=kw,result=result));print(name,[len(result[x])for x in ['ordinary','canvas']],flush=True)
 add('sphere',[(1,widget(0,{0x8c:[1],0x90:[0,0,3]}),None)])
 add('sphere-cloud',[(2,widget(1,{0x8c:[1]}),[(0,0,5),(0,0,2),(10,0,1)])])
 add('triangles',[(3,widget(2,{}),[(-1,-1,5,1,-1,5,0,1,5),(-1,-1,2,1,-1,2,0,1,2)])])
 add('screen-aabb',[(4,widget(3,{0x8c:[.25],0x90:[-1,-1,1,1]}),None)])
 add('world-canvas',[(5,widget(4,{0x8c:[10,20]},layer=2),None)],terrain=(1,9,2))
 add('terrain-canvas-no-terrain',[(6,widget(5,{0x8c:[99]},layer=3),None)])
 add('terrain-canvas',[(6,widget(5,{0x8c:[99]},layer=3),None)],terrain=(1,9,2))
 add('plane-disc',[(7,widget(6,{0x8c:[0,0,3,0,0,-1,2]}),None)])
 add('empty',[(8,widget(7,{}),None)])
 (ROOT/'native-picker-fixtures.json').write_text(json.dumps(cases,indent=2),encoding='utf8')
