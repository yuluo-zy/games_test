from pathlib import Path
import struct,json,hashlib
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();p=pefile.PE(data=raw)
base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(base,image)
D,S,STOP,GEN=0x20000000,0x30000000,0x50000000,0x50000100
c.mem_map(D,0x40000);c.mem_map(S,0x20000);c.mem_map(STOP,0x1000)
def q(a,v):c.mem_write(a,struct.pack('<Q',v&0xffffffffffffffff))
def u(a):return struct.unpack('<Q',c.mem_read(a,8))[0]
def d(a,v):c.mem_write(a,struct.pack('<I',v&0xffffffff))
def ret():
 sp=c.reg_read(UC_X86_REG_RSP);pc=u(sp);c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,pc)
trace=[]
def hook(cpu,pc,sz,_):
 global trace
 rcx=cpu.reg_read(UC_X86_REG_RCX);rdx=cpu.reg_read(UC_X86_REG_RDX);r8=cpu.reg_read(UC_X86_REG_R8)
 if pc==STOP:cpu.emu_stop();return
 if pc==0x14092d570:trace.append('named_lookup');cpu.reg_write(UC_X86_REG_RAX,1);cpu.reg_write(UC_X86_REG_RDX,0x11111111)
 elif pc==0x14092cc60:trace.append('register_mesh');cpu.reg_write(UC_X86_REG_RAX,0x22222222)
 elif pc==0x140a6c120:
  trace.append('mesh_from_pts:'+bytes(cpu.mem_read(rdx,r8)).decode());cpu.mem_write(rcx,bytes(120))
 elif pc==0x140a34c90:trace.append('set_colors')
 elif pc==0x1400d9b20:cpu.reg_write(UC_X86_REG_RAX,D+0x2800)
 elif pc==0x140d146a0:
  trace.append('reserve_entity')
  for i,v in enumerate([0,D+0x3000,D+0x3000,0,0x1234]):q(rcx+8*i,v)
 elif pc==0x140937aa0:trace.append('shader_lookup');cpu.reg_write(UC_X86_REG_RAX,1);cpu.reg_write(UC_X86_REG_RDX,0x33333333)
 elif pc==0x1412de950:trace.append('transient_receipt')
 elif pc==0x140613c20:pass # Existing allocation boundary; ownership drops recorded by native branches.
 elif pc==GEN:
  trace.append('generate_points');q(rcx,2);q(rcx+8,D+0x2000);q(rcx+16,2)
 else:return
 ret()
c.hook_add(UC_HOOK_CODE,hook)
def run(kind,disabled,wireframe,ui_disabled,lifespan,parent):
 global trace
 trace=[];c.mem_write(D,bytes(0x40000));c.mem_write(S,bytes(0x20000))
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RBP,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:c.reg_write(r,0)
 for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):c.reg_write(r,0)
 b=D+0x100;c.mem_write(b,struct.pack('<4f',0,0,0,1));q(b+0x10,0x8000000000000000+kind)
 if kind==0:q(b+0x18,5);q(b+0x20,D+0x1800);q(b+0x28,5);c.mem_write(D+0x1800,b'brush')
 elif kind==2:q(b+0x18,2);q(b+0x20,D+0x2000);q(b+0x28,2)
 elif kind==3:
  q(b+0x10,2);q(b+0x18,D+0x2000);q(b+0x20,2);q(b+0x28,2);q(b+0x30,D+0x2100);q(b+0x38,2)
 elif kind==4:q(b+0x18,D+0x2200);q(b+0x20,D+0x2300);q(D+0x2300+0x28,GEN)
 c.mem_write(b+0x40,bytes([disabled,1]));d(b+0x44,0x3f800000);d(b+0x48,1)
 d(b+0x50,1);d(b+0x54,0x40200000);d(b+0x58,lifespan);d(b+0x5c,0x3f333333)
 q(b+0x60,19);q(b+0x68,D+0x1900);q(b+0x70,19);c.mem_write(D+0x1900,b'vertex_color_shader')
 d(b+0x78,0);c.mem_write(b+0x7c,struct.pack('<3f',1,2,3));c.mem_write(b+0x88,bytes([parent,wireframe,ui_disabled]))
 meshes=D+0x1000;q(meshes+0x18,D+0x2700);q(D+0x2700,1)
 q(D+0x3000,0x10000);q(D+0x3008,D+0x4000);q(D+0x3010,0)
 sp=S+0x10000-8;q(sp,STOP);q(sp+0x28,D+0x2a00)
 c.reg_write(UC_X86_REG_RSP,sp);c.reg_write(UC_X86_REG_RCX,D+0x800);c.reg_write(UC_X86_REG_RDX,b);c.reg_write(UC_X86_REG_R8,meshes);c.reg_write(UC_X86_REG_R9,D+0x1100)
 c.reg_write(UC_X86_REG_MXCSR,0x1f80);c.reg_write(UC_X86_REG_RFLAGS,0x202)
 try:c.emu_start(0x14089dd50,STOP,count=50000)
 except Exception as e:raise RuntimeError(f'{e} pc={c.reg_read(UC_X86_REG_RIP):x} trace={trace}')
 assert c.reg_read(UC_X86_REG_RIP)==STOP
 data=D+0x4000;n=u(D+0x3010);payload=bytes(c.mem_read(data,n))
 assert u(data)==0x1412db690
 # Native queue callback followed by actual DrawableBundle + Animation.
 scale=struct.unpack('<3f',payload[0x24:0x30]);value=struct.unpack_from('<f',payload,0x54)[0]
 assert scale==((2.5,)*3 if disabled else (0.,)*3)
 assert value==(1.0 if disabled else 2.5)
 callbacks=[hex(u(data+0x98))]
 assert u(data+0x98)==(0x1412d6ba0 if wireframe else (0x1412d8160 if ui_disabled else 0x1412d96a0))
 assert n==0x98+(0x20 if wireframe or not ui_disabled else 0)+0x20+lifespan*0x28+parent*0x20
 assert payload[8:24]==struct.pack('<4f',0,0,0,1)
 assert payload[0x18:0x24]==struct.pack('<3f',1,2,3)
 assert ('set_colors' in trace)==(kind==3)
 assert ('generate_points' in trace)==(kind==4)
 assert ('transient_receipt' in trace)==(kind!=0)
 assert ('named_lookup' in trace)==(kind==0)
 return {'mesh_branch':kind,'disable_animation':bool(disabled),'wireframe':bool(wireframe),'ui_disabled':bool(ui_disabled),'lifespan':bool(lifespan),'parent':bool(parent),'trace':trace.copy(),'native_initial_scale':scale,'native_animation_value':value,'command_queue_bytes':n,'first_pass_callback':callbacks[0],'queue_hex':payload.hex()}
rows=[]
for kind in range(5):
 for mask in range(32):rows.append(run(kind,*[(mask>>i)&1 for i in range(5)]))
(OUT/'original-builder-fixtures.json').write_text(json.dumps({'original_sha256':sha,'original_function':{'va':'0x14089dd50','bytes':3840,'sha256':hashlib.sha256(image[0x14089dd50-base:0x14089dd50-base+3840]).hexdigest()},'scope':'Entire original builder executes its business switch, scale/animation, pass/lifespan/parent branches and native deferred payload writes. Existing library lookups, geometry constructor, std allocation/TLS, Bevy spawn and transient receipt insertion are isolated boundary stubs; this is not an end-to-end runtime proof.','cases':rows},indent=2)+'\n')
print(json.dumps({'native_builder_cases':len(rows)}))
