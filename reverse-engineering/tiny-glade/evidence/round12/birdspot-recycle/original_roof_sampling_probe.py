from pathlib import Path
import struct,json,hashlib,math
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).resolve().parent;raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x50000000;c.mem_map(D,0x100000);c.mem_map(S,0x10000);c.mem_map(STOP,0x1000)
q=lambda a,v:c.mem_write(a,struct.pack('<Q',v&0xffffffffffffffff));d=lambda a,v:c.mem_write(a,struct.pack('<I',v&0xffffffff));f=lambda a,v:c.mem_write(a,struct.pack('<f',v));u=lambda a:struct.unpack('<Q',c.mem_read(a,8))[0];uf=lambda a:struct.unpack('<f',c.mem_read(a,4))[0]
heap=D+0x10000;bp=S+0x8000;trace=[];captured=None;params={}
def ret():
 sp=c.reg_read(UC_X86_REG_RSP);target=u(sp);c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,target)
def hook(cpu,pc,sz,_):
 global heap,captured,params
 rcx=cpu.reg_read(UC_X86_REG_RCX);rdx=cpu.reg_read(UC_X86_REG_RDX)
 trace.append(pc)
 if len(trace)>64:del trace[0]
 if pc==0x140f86340:
  vec=cpu.reg_read(UC_X86_REG_R8);captured=bytes(cpu.mem_read(u(vec+8),u(vec+16)*24));cpu.emu_stop();return
 if pc==0x140613c10:
  ptr=heap;heap=(heap+rcx+15)&~15;cpu.reg_write(UC_X86_REG_RAX,ptr);ret()
 elif pc==0x142923ee0:
  x=struct.unpack('<f',struct.pack('<I',cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff))[0];v=float(math.floor(x)) if math.isfinite(x) and x!=0 else x
  cpu.reg_write(UC_X86_REG_XMM0,struct.unpack('<I',struct.pack('<f',v))[0]);ret()
 elif pc==0x140562db0:
  ptr=heap;heap+=16*24;q(rcx,16);q(rcx+8,ptr);ret()
 elif pc in [0x142923c00,0x142923c20]:
  r8=cpu.reg_read(UC_X86_REG_R8);cpu.mem_write(rcx,bytes(cpu.mem_read(rdx,r8)) if pc==0x142923c00 else bytes([rdx&255])*r8);ret()
 elif pc==0x140c90350:params={'count':rdx,'amplitude_bits':cpu.reg_read(UC_X86_REG_XMM2)&0xffffffff}
 elif pc==0x14231de1d:
  params['boundaries_bits']=list(struct.unpack('<'+'I'*u(bp+0x20),cpu.mem_read(u(bp+0x18),u(bp+0x20)*4)))
c.hook_add(UC_HOOK_CODE,hook)
def run(name,dims=(4,6),direction=0,gable=0,ridge=.8,height=.6,frame=77,tag=1,axis=(1,0),center=(3,-5),tip=(.25,-.4)):
 global heap,captured,params
 heap=D+0x10000;captured=None;params={};c.mem_write(D,bytes(0x100000));c.mem_write(S,bytes(0x10000));roof=D+0x100;time=D+0x200;manager=D+0x300
 d(roof+8,tag)
 for off,val in [(0xc,axis[0]),(0x10,axis[1]),(0x14,center[0]),(0x18,center[1]),(0x1c,dims[0]),(0x20,dims[1]),(0x24,tip[0]),(0x28,tip[1]),(0x2c,.5),(0x30,height),(0x34,ridge),(0x4c,2)]:f(roof+off,val)
 d(roof+0x3c,direction);c.mem_write(roof+0x54,bytes([gable]));d(time+0x2c,frame);q(bp+0x50,time);q(bp+0x108,manager);q(bp+0x70,D+0x400)
 for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:c.reg_write(reg,0)
 for reg,val in [(UC_X86_REG_XMM6,1.),(UC_X86_REG_XMM12,.1),(UC_X86_REG_XMM13,.5),(UC_X86_REG_XMM14,.3),(UC_X86_REG_XMM11,-.1),(UC_X86_REG_XMM15,.7)]:c.reg_write(reg,struct.unpack('<I',struct.pack('<f',val))[0])
 c.reg_write(UC_X86_REG_RCX,roof);c.reg_write(UC_X86_REG_RBP,bp);c.reg_write(UC_X86_REG_RSP,bp-0x80);c.reg_write(UC_X86_REG_MXCSR,0x1f80);c.reg_write(UC_X86_REG_RFLAGS,0x202)
 try:c.emu_start(0x14231db27,STOP,count=5000000)
 except Exception as e:raise RuntimeError((name,hex(c.reg_read(UC_X86_REG_RIP)),e,[hex(a) for a in trace[-12:]]))
 assert captured is not None,(name,hex(c.reg_read(UC_X86_REG_RIP)))
 assert len(captured)%24==0
 return {'name':name,'roof_hex':bytes(c.mem_read(roof,88)).hex(),'frame_count':frame,'parameters':params,'native_points':[{'occupant_bits':u0,'position_bits':[a,b,c0],'id':i} for u0,a,b,c0,i in struct.iter_unpack('<Q4I',captured)]}
cases=[]
for direction in [0,1]:
 for gable in [0,1]:
  for ridge in [0,.05,.2,.5,1.]:
   for frame in [0,77,0xffffffff]:
    cases.append(run(f'dir{direction}-style{gable}-ridge{ridge}-frame{frame}',direction=direction,gable=gable,ridge=ridge,frame=frame))
for direction in [0,1]:
 for width in [.49,.5,.500001,.69,.7,.700001,.89,.9,1.2,1.5,2.1,3.,15.]:
  dims=(width,width)
  cases.append(run(f'length{width}-dir{direction}',dims=dims,ridge=1.,direction=direction,gable=0))
for name,kwargs in [('circle',{'tag':0}),('flat',{'height':.1}),('flat-below',{'height':.099999}),('pitched-next',{'height':.100001}),('rotated',{'axis':(.6,.8),'center':(-2,7)}),('signed-zero',{'axis':(1,-0.),'center':(-0.,0.),'tip':(-0.,0.)})]:cases.append(run(name,**kwargs))
(OUT/'original-roof-sampling-fixtures.json').write_text(json.dumps({'original_sha256':hashlib.sha256(raw).hexdigest(),'range':['0x14231db27','0x14231e0a6'],'scope':'Original complete shape gate, ridge/edge/clipping, frame-count seed, split planning, original random_splits/fastrand, interval sampling, world transform, height and roof-noise game instructions. Stops at original create_spots owner boundary. Only CRT floorf/memcpy/memset, allocation and generic Vec growth are dependency shims; no game helper is replaced.','cases':cases},indent=2)+'\n')
print({'native_roof_cases':len(cases),'nonempty':sum(bool(c['native_points']) for c in cases)})
