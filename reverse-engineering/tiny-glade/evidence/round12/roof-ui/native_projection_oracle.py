"""Execute complete original PSS constructors/transform/projection and lerp.

No game-call emulation and no byte patching. Input matrices are real affine
view / perspective records; fixtures freeze complete arguments and output.
"""
from native_focused_helpers_oracle import c,setup,DATA,STACK,bits,UC_HOOK_CODE
from unicorn.x86_const import *
from pathlib import Path
import json,struct,random,math
OUT=Path(__file__).parent
IDENTITY=[1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.]
def pack(a,values):c.mem_write(a,struct.pack('<%df'%len(values),*values))
def run_project(screen,xy,view,projection,ray,limits):
 setup();sp=STACK+0x80000
 pack(DATA+0x1000,projection);pack(DATA+0x1080,view)
 pack(DATA+0x2000,ray);pack(DATA+0x2100,limits);pack(DATA+0x2200,screen)
 for off,p in [(0x28,DATA+0x1000),(0x30,DATA+0x2000),(0x38,DATA+0x2100)]:c.mem_write(sp+off,struct.pack('<Q',p))
 c.reg_write(UC_X86_REG_R9,DATA+0x2200);c.reg_write(UC_X86_REG_XMM1,bits(xy[0]));c.reg_write(UC_X86_REG_XMM2,bits(xy[1]))
 c.emu_start(0x140ac84a0,0,count=10000)
 assert c.reg_read(UC_X86_REG_RIP)==DATA+0x100
 values=[*screen,*xy,*view,*projection,*ray,*limits]
 return dict(input_bits=[bits(v) for v in values],output_bits=struct.unpack('<3I',c.mem_read(DATA,12)))
cases=[]
views=[IDENTITY,IDENTITY[:12]+[2.,-1.,-10.,1.]]
projections=[IDENTITY,[1.25,0.,0.,0.,0.,1.75,0.,0.,0.,0.,0.,-1.,0.,0.,0.1,0.]]
for view in views:
 for proj in projections:
  for origin in [(0.,0.,0.),(3.,4.,-8.),(0.,0.,1.)]:
   for direction in [(0.,1.,0.),(1.,0.,0.),(0.,0.,0.),(1.,2.,3.)]:
    for xy in [(0.,0.),(.25,-.1),(100.,-100.)]:
     cases.append(run_project([1920.,1080.,1920./1080.,1.],xy,view,proj,[*origin,*direction],[0.,3.4028234663852886e38]))
for xy in [(float('nan'),0.),(float('inf'),1.)]:
 cases.append(run_project([0.,0.,float('nan'),1.],xy,IDENTITY,IDENTITY,[0.,0.,0.,0.,1.,0.],[0.,1.]))
transforms=[]
for size in [(1920,1080),(200,100),(0,0),(10,0)]:
 for xy in [(0.,0.),(100.,50.),(-2.,-4.)]:
  setup();c.mem_write(DATA+0x1000,struct.pack('<2I',*size));c.reg_write(UC_X86_REG_RDX,DATA+0x1000)
  c.emu_start(0x140ac8420,0,count=1000);screen=bytes(c.mem_read(DATA,16))
  # Fresh CPU call-frame only; retain constructed PSS in DATA+0x2000.
  setup();c.mem_write(DATA+0x2000,screen);c.reg_write(UC_X86_REG_RCX,DATA+0x2000)
  c.reg_write(UC_X86_REG_XMM1,bits(xy[0]));c.reg_write(UC_X86_REG_XMM2,bits(xy[1]))
  c.emu_start(0x140ac8450,0,count=1000)
  transforms.append(dict(size=size,xy_bits=[bits(v) for v in xy],screen_bits=struct.unpack('<4I',screen),output_bits=[c.reg_read(UC_X86_REG_XMM0)&0xffffffff,c.reg_read(UC_X86_REG_XMM1)&0xffffffff]))
report=dict(source='unchanged B original RNE',projection='complete140ac84a0/723B, calls original lerp140c904d0',cases=cases,transform_screen=transforms)
(OUT/'native-projection-fixtures.json').write_text(json.dumps(report,indent=2),encoding='utf8')
print('projection',len(cases),'constructor+transform',len(transforms))
