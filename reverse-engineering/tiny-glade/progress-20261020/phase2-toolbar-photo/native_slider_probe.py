from pathlib import Path
import hashlib,json,struct,itertools,math
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[3]
binary=ROOT/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
raw=binary.read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);image=p.get_memory_mapped_image();base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
u.mem_map(0x100000,0x80000)
stack,extent,slider,tick,cursor,alloc=0x110000,0x120000,0x121000,0x122000,0x123000,0x124000
fraction=0.;stop_nav=False;stop_drag=False;stop_category=False;stop_reset=False
def f32(x):return struct.unpack('<f',struct.pack('<f',x))[0]
def xf(n,x):u.reg_write(UC_X86_REG_XMM0+n,int.from_bytes(struct.pack('<4f',*(x+[0.]*(4-len(x)))),'little'))
def xget(n):return struct.unpack('<f',u.reg_read(UC_X86_REG_XMM0+n).to_bytes(16,'little')[:4])[0]
def writeq(a,v):u.mem_write(a,struct.pack('<Q',v))
def readq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
def hook(u,addr,size,_):
 if stop_nav and addr in (0x142348893,0x14234888b):u.emu_stop();return
 if stop_drag and addr==0x142348fa1:u.emu_stop();return
 if stop_category and addr==0x142349b40:u.emu_stop();return
 if stop_reset and addr in (0x14234811e,0x142348130):u.emu_stop();return
 if stop_reset and addr==0x142923eb0:
  xf(0,[2.0**xget(0)]);rsp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,readq(rsp));u.reg_write(UC_X86_REG_RSP,rsp+8);return
 if addr in (0x142349de0,0x140613c10):
  if addr==0x142349de0:xf(0,[fraction]) # parameter getter separately validated: 1,638 cases
  else:u.reg_write(UC_X86_REG_RAX,alloc)
  rsp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,readq(rsp));u.reg_write(UC_X86_REG_RSP,rsp+8)
u.hook_add(UC_HOOK_CODE,hook)
visual=bytearray()
dimensions=[(400.,40.),(100.,20.),(0.,40.),(-100.,40.),(400.,-40.),(.1,.1)]
for dims,frac,hover,dt,point in itertools.product(dimensions,[-.25,0.,.25,.5,1.,1.25],[0.,.5,1.],[0.,.01,.1],[0,1,2]):
 fraction=frac;center=(100.,75.);side=f32(f32(dims[1])*.3);travel=f32(f32(f32(side*-2.5)+f32(dims[0]))*.5)
 lo=f32(center[0]-travel);hi=f32(center[0]+travel);knob=f32(f32(frac*hi)+f32(f32(1.-frac)*lo))
 pointx=knob if point==0 else (f32(knob+f32(side*.2)) if point==1 else knob+abs(side)*2+10)
 u.mem_write(extent,struct.pack('<4f',*center,*dims));u.mem_write(slider,bytes(24));u.mem_write(slider+8,struct.pack('<f',hover));u.mem_write(cursor,struct.pack('<2f',pointx,75.))
 u.reg_write(UC_X86_REG_RSP,stack);u.reg_write(UC_X86_REG_R15,extent);u.reg_write(UC_X86_REG_R12,slider);u.reg_write(UC_X86_REG_R13,tick)
 writeq(stack+0xd0,cursor);writeq(stack+0xf8,tick);writeq(stack+0xb0,tick);writeq(stack+0x50,tick);writeq(stack+0xb8,tick)
 for n in range(16):xf(n,[0.])
 xf(0,list(dims));xf(10,[f32(f32(dt)/f32(.2))]);xf(11,[1.]);xf(14,[-f32(f32(dt)/f32(.2))])
 u.emu_start(0x142348b15,0x142348c57,count=5000)
 out=(struct.unpack('<f',u.mem_read(slider+8,4))[0],xget(7),xget(8),xget(12))
 visual+=struct.pack('<13f',*center,*dims,frac,hover,dt,pointx,75.,*out)
(HERE/'visual-native.bin').write_bytes(visual)

stop_nav=True;nav=bytearray();entity=0x200000001
navptr,prevptr,actionptr,boolptr=0x125000,0x126000,0x127000,0x128000
for lp,rp,timer,armed,focus_case,frac in itertools.product(range(5),range(5),[-.1,.1],[0,1],range(4),[0.,.5,1.]):
 dt=.05;focus=(1,0,entity);previous=focus
 if focus_case==1:previous=(0,0,0)
 if focus_case==2:focus=(0,0,0)
 if focus_case==3:focus=(1,0,0x200000002)
 u.mem_write(stack,bytes(0x400));u.mem_write(navptr,bytes(64));u.mem_write(navptr+0x10,struct.pack('<IIQ',*focus));u.mem_write(prevptr,struct.pack('<IIQ',*previous));u.mem_write(boolptr,bytes([armed]));u.mem_write(actionptr,bytes(400))
 for offset,phase in [(0xa0,lp),(0xa8,rp)]:u.mem_write(actionptr+offset,struct.pack('<If',phase,timer))
 for off,pointer in [(0xe8,navptr),(0x100,navptr+0x10),(0x110,actionptr),(0x290,boolptr),(0x298,prevptr)]:writeq(stack+off,pointer)
 writeq(stack+0x60,entity);u.mem_write(stack+0x5c,struct.pack('<I',armed))
 u.reg_write(UC_X86_REG_RSP,stack);u.reg_write(UC_X86_REG_R15,0)
 xf(6,[frac]);xf(13,[dt]);xf(11,[1.])
 u.emu_start(0x142348cfc,0,count=1000)
 has=int(readq(stack+0xa8)!=0);candidate=struct.unpack('<I',u.mem_read(stack+0x120,4))[0]
 category=int(u.reg_read(UC_X86_REG_R15)!=0)
 nav+=struct.pack('<IIfBB2xffIIII',lp,rp,timer,armed,focus_case,frac,dt,u.mem_read(boolptr,1)[0],has,candidate,category)
(HERE/'navigation-native.bin').write_bytes(nav)
stop_nav=False;stop_drag=True;drag_data=bytearray();event=0x129000;grabptr=0x12a000
for state,has_grab,offset,frac,cursor_x in itertools.product(range(5),[0,1],[-4.,0.,4.],[0.,.5,1.],[-100.,96.,100.,104.,300.,float('nan')]):
 fraction=frac
 u.mem_write(stack,bytes(0x400));u.mem_write(extent,struct.pack('<4f',100.,100.,400.,40.));u.mem_write(slider,bytes(24));u.mem_write(cursor,struct.pack('<2f',cursor_x,100.));u.mem_write(event,struct.pack('<QQB7x',0,entity,state));u.mem_write(grabptr,struct.pack('<If',has_grab,offset))
 for off,pointer in [(0xd8,extent),(0x40,tick),(0x60,slider),(0x270,grabptr),(0xd0,cursor),(0xf8,tick),(0xb0,tick),(0x50,tick),(0xb8,tick)]:writeq(stack+off,pointer)
 u.mem_write(stack+0x4c,struct.pack('<I',has_grab))
 u.reg_write(UC_X86_REG_RSP,stack);u.reg_write(UC_X86_REG_R8,extent);u.reg_write(UC_X86_REG_R9,0);u.reg_write(UC_X86_REG_R10,slider);u.reg_write(UC_X86_REG_RCX,0);u.reg_write(UC_X86_REG_R13,event)
 xf(10,[offset]);xf(11,[1.]);xf(12,[0.])
 u.emu_start(0x1423490fd,0,count=2000)
 current_grab=struct.unpack('<If',u.mem_read(grabptr,8));has=int(readq(stack+0xa8)!=0);category=int(readq(stack+0xf0)!=0)
 drag_data+=struct.pack('<IIf f f I f I I I',state,has_grab,offset,frac,cursor_x,current_grab[0],current_grab[1],has,u.reg_read(UC_X86_REG_XMM12)&0xffffffff,category)
(HERE/'drag-native.bin').write_bytes(drag_data)

stop_drag=False;stop_category=True;category_data=bytearray()
mode_view,mode_value,writer,queue,queue_data=0x12b000,0x12c000,0x12d000,0x12e000,0x130000
for kind,mode_tag in itertools.product(range(13),range(21)):
 u.mem_write(stack,bytes(0x400));u.mem_write(mode_view,bytes(32));u.mem_write(writer,bytes(32));u.mem_write(queue,bytes(128))
 writeq(stack+0x2c8,mode_view);writeq(mode_view,mode_value);u.mem_write(mode_value,struct.pack('<I',mode_tag))
 writeq(stack+0x2d0,writer);writeq(writer,queue);writeq(writer+0x10,tick);u.mem_write(writer+0x1c,struct.pack('<I',123))
 writeq(queue+0x20,100);writeq(queue+0x28,queue_data)
 u.reg_write(UC_X86_REG_RSP,stack);u.reg_write(UC_X86_REG_RAX,kind)
 u.emu_start(0x142349a85,0,count=300)
 count=readq(queue+0x30);category=u.mem_read(queue_data+8,1)[0] if count else 0
 category_data+=struct.pack('<IIII',kind,mode_tag,int(count),category)
(HERE/'category-native.bin').write_bytes(category_data)
stop_category=False;stop_reset=True;reset_data=[]
configmem,cameramem,brushmem,kindmem=0x140000,0x141000,0x142000,0x143000
lenses=[0x18c,0x244,0x2c4,0x364,0x41c,0x4a4];focuses=[0x170,0x228,0x2a8,0x348,0x400,0x488]
for kind,mode,focus_mode in itertools.product(range(13),range(6),range(4)):
 u.mem_write(stack,bytes(0x400));u.mem_write(configmem,bytes(0xc8));u.mem_write(cameramem,bytes(0x4e0));u.mem_write(brushmem,struct.pack('<3f',.1,.2,.3));u.mem_write(kindmem,bytes([kind]))
 for row,values in enumerate([[-.75,1.25,.85,.6,.07,.005],[10.,20.,30.,40.,50.,60.],[11.,21.,31.,41.,51.,61.]]):
  for offset,value in zip([0x1c,0x20,0x24,0x30,0x38,0x34],values):u.mem_write(configmem+row*64+offset,struct.pack('<f',value))
 u.mem_write(cameramem+0x4d0,bytes([mode]));lo=lenses[mode];fo=focuses[mode]
 for offset,value in [(8,12.),(0x14,73.),(0x18,2.5),(0x24,7.5)]:u.mem_write(cameramem+lo+offset,struct.pack('<f',value))
 u.mem_write(cameramem+fo,struct.pack('<I',focus_mode))
 for offset,value in [(4,-10.),(8,25.),(0x10,17.),(0x14,-23.),(0x18,.125)]:u.mem_write(cameramem+fo+offset,struct.pack('<f',value))
 for offset,pointer in [(0x58,tick),(0x60,configmem),(0x70,tick),(0x78,cameramem),(0x80,tick),(0x88,brushmem)]:writeq(stack+offset,pointer)
 u.reg_write(UC_X86_REG_RSP,stack);u.reg_write(UC_X86_REG_R15,kindmem);u.reg_write(UC_X86_REG_RBP,0)
 xf(9,[1.]);xf(10,[-2.]);xf(11,[2.]);xf(12,[-3.])
 u.emu_start(0x14234824a,0,count=2000)
 reset_data.append({'kind':kind,'mode':mode,'focus_mode':focus_mode,'camera':bytes(u.mem_read(cameramem,0x4e0)).hex(),
  'config':bytes(u.mem_read(configmem,0xc8)).hex(),'brushes':bytes(u.mem_read(brushmem,12)).hex()})
(HERE/'reset-native.json').write_text(json.dumps(reset_data,indent=2))
report={'binary_sha256':sha,'visual_cases':len(visual)//52,'navigation_cases':len(nav)//40,
 'drag_cases':len(drag_data)//40,'category_cases':len(category_data)//16,
 'reset_parameter_cases':len(reset_data),
 'visual_range':['142348b15','142348c57'],'navigation_range':['142348cfc','142348ef9'],
 'hooks':'visual only: separately-validated getter returns fixture fraction; original allocator returns test storage. Original geometry/AABB/easing and all digital-state routines execute original instructions. Navigation has no dependency hooks.',
 'fixtures':{name:hashlib.sha256((HERE/name).read_bytes()).hexdigest() for name in ['visual-native.bin','navigation-native.bin','drag-native.bin','category-native.bin']}}
(HERE/'native-probe.json').write_text(json.dumps(report,indent=2));print(json.dumps(report))
