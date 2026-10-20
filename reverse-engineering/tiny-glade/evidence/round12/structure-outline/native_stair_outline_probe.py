from pathlib import Path
import struct,json,hashlib
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).resolve().parent;raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x50000000;c.mem_map(D,0x100000);c.mem_map(S,0x10000);c.mem_map(STOP,0x1000)
q=lambda a,v:c.mem_write(a,struct.pack('<Q',v&0xffffffffffffffff));d=lambda a,v:c.mem_write(a,struct.pack('<I',v&0xffffffff));u=lambda a:struct.unpack('<Q',c.mem_read(a,8))[0];u32=lambda a:struct.unpack('<I',c.mem_read(a,4))[0]
heap=0;outputs=[];panic=None

def ret():
 sp=c.reg_read(UC_X86_REG_RSP);target=u(sp);c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,target)
def hook(cpu,pc,sz,_):
 global heap,panic
 rcx=cpu.reg_read(UC_X86_REG_RCX);rdx=cpu.reg_read(UC_X86_REG_RDX)
 if pc==STOP:cpu.emu_stop();return
 if pc==0x140613c10:
  ptr=heap;heap=(heap+rcx+15)&~15;cpu.reg_write(UC_X86_REG_RAX,ptr);ret()
 elif pc==0x140613c20:ret()
 elif pc==0x140b9f180:
  oldcap=u(rcx);oldptr=u(rcx+8);n=u(rcx+16);cap=max(4,oldcap*2,n+1);ptr=heap;heap+=cap*12
  if n:cpu.mem_write(ptr,bytes(cpu.mem_read(oldptr,n*12)))
  q(rcx,cap);q(rcx+8,ptr);ret()
 elif pc==0x140b04f70:
  length=u(rcx+16);data=bytes(cpu.mem_read(u(rcx+8),length*12)) if length else b''
  outputs.append({'point_bits':list(struct.iter_unpack('<3I',data)),'color':u32(rcx+48),'width_bits':u32(rcx+64),'flag44':cpu.mem_read(rcx+68,1)[0],'flag45':cpu.mem_read(rcx+69,1)[0],'colors_discriminant':hex(u(rcx+24))});ret()
 elif pc in [0x1428d9518,0x1428d8fe3]:panic=hex(pc);cpu.emu_stop()
c.hook_add(UC_HOOK_CODE,hook)
def run(name,kind,path,heights,widths,dirs):
 global heap,outputs,panic
 heap=D+0x10000;outputs=[];panic=None;c.mem_write(D,bytes(0x100000));c.mem_write(S,bytes(0x10000));ctx,record=D+0x100,D+0x200
 q(ctx,D+0x500);q(ctx+8,D+0x508);q(ctx+16,D+0x510);d(D+0x500,kind);d(D+0x508,0x40000000);q(D+0x510,D+0x600)
 for off,values,width in [(0,path,2),(0x38,heights,1),(0x50,widths,1),(0x68,dirs,2)]:
  ptr=heap;packed=b''.join(struct.pack('<'+str(width)+'f',*(v if width>1 else [v])) for v in values);heap=(heap+len(packed)+15)&~15;cpu_data=packed if packed else b'\x00';c.mem_write(ptr,cpu_data);q(record+off,len(values));q(record+off+8,ptr);q(record+off+16,len(values))
 for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:c.reg_write(reg,0)
 sp=S+0x8000-8;q(sp,STOP);c.reg_write(UC_X86_REG_RSP,sp);c.reg_write(UC_X86_REG_RCX,ctx);c.reg_write(UC_X86_REG_RDX,record);c.reg_write(UC_X86_REG_MXCSR,0x1f80);c.reg_write(UC_X86_REG_RFLAGS,0x202)
 try:c.emu_start(0x14095fca0,STOP,count=1000000)
 except Exception as e:raise RuntimeError(name,hex(c.reg_read(UC_X86_REG_RIP)),e)
 return {'name':name,'kind':kind,'path':path,'heights':heights,'widths':widths,'directions':dirs,'outputs':outputs,'panic':panic}
cases=[]
for kind in [0,1,2,3,255]:
 for n in [0,1,2,4]:
  path=[(float(i),float(i*i)) for i in range(n)];heights=[1.+i*.25 for i in range(n)];widths=[.1+i*.33 for i in range(n)];dirs=[(1.,0.) if i%2 else (0.,1.) for i in range(n)]
  cases.append(run(f'kind{kind}-count{n}',kind,path,heights,widths,dirs))
for kind in [0,1,2]:
 for field in ['heights','widths','directions']:
  data={'path':[(0.,0.),(1.,2.),(2.,3.)],'heights':[2.,3.,4.],'widths':[1.,2.,3.],'dirs':[(0.,0.),(.6,.8),(-1.,-0.)]};data[{'directions':'dirs'}.get(field,field)]=data[{'directions':'dirs'}.get(field,field)][:1];cases.append(run(f'truncate-{field}-kind{kind}',kind,**data))
(OUT/'native-stair-outline-fixtures.json').write_text(json.dumps({'original_sha256':hashlib.sha256(raw).hexdigest(),'function':{'address':'0x14095fca0','size':2474,'sha256':hashlib.sha256(image[0x14095fca0-base:0x14095fca0-base+2474]).hexdigest()},'scope':'Entire original2474B game callback, captured at each actual DebugLine.show owner boundary. Native path extrusion, safe-normalize, reverse traversal, closure, kind1 additional centerline and empty panic all execute. Only allocator/Vec growth/dealloc are dependency shims; DebugLine rendering owner is intentionally outside this geometry contract.','cases':cases},indent=2)+'\n')
print({'native_stair_outline_cases':len(cases),'panics':sum(c['panic'] is not None for c in cases)})
