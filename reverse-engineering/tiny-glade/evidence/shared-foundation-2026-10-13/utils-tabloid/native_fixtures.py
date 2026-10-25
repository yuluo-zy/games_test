"""Execute original B RNE CPU instructions; emit fixtures for actual Rust tests."""
from pathlib import Path
import ctypes,json,hashlib,struct,random
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
out=Path(__file__).parent
binary=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw=binary.read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095)
u.mem_write(base,p.get_memory_mapped_image())
stack=0x200000000;scratch=0x210000000;stop=0x220000000
u.mem_map(stack,0x20000);u.mem_map(scratch,0x20000);u.mem_map(stop,0x1000)
crt=ctypes.CDLL('ucrtbase')
functions={0x142923f10:('fmodf',2),0x142923e80:('cosf',1),0x142923fd0:('sinf',1),0x142923eb0:('exp2f',1)}
for name,n in functions.values():
 f=getattr(crt,name);f.argtypes=[ctypes.c_float]*n;f.restype=ctypes.c_float
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def value(b):return struct.unpack('<f',struct.pack('<I',b&0xffffffff))[0]
def xf(reg,f):u.reg_write(reg,bits(f))
def hook(uc,addr,size,data):
 if addr in functions:
  name,n=functions[addr];args=[value(u.reg_read(UC_X86_REG_XMM0))]
  if n==2:args.append(value(u.reg_read(UC_X86_REG_XMM1)))
  xf(UC_X86_REG_XMM0,getattr(crt,name)(*args))
  sp=u.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',u.mem_read(sp,8))[0]
  u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ret)
u.hook_add(UC_HOOK_CODE,hook)
def reset():
 sp=stack+0x10008;u.reg_write(UC_X86_REG_RSP,sp);u.mem_write(sp,struct.pack('<Q',stop))
 for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_XMM0,UC_X86_REG_XMM1,UC_X86_REG_XMM2,UC_X86_REG_XMM3]:u.reg_write(r,0)
def run(addr):u.emu_start(addr,stop,count=200000)
rng=random.Random(13013);fixtures={}
fixtures['rng']=[(seed,0)for seed in [0,1,0xffffffff,0xffffffffffffffff]+[rng.getrandbits(64)for _ in range(40)]]
for i,(seed,_)in enumerate(fixtures['rng']):
 reset();u.reg_write(UC_X86_REG_RCX,seed);run(0x140c96270);fixtures['rng'][i]=(seed,u.reg_read(UC_X86_REG_RAX))
fixtures['loop_index']=[]
for _ in range(50):
 index=rng.randrange(-100000,100000);period=rng.randrange(1,1000);reset();u.reg_write(UC_X86_REG_RCX,index&0xffffffff);u.reg_write(UC_X86_REG_RDX,period);run(0x141200280)
 fixtures['loop_index'].append((index,period,u.reg_read(UC_X86_REG_RAX)))
fixtures['circular_lerp']=[]
for _ in range(50):
 a,b,factor=[rng.uniform(-20,20),rng.uniform(-20,20),rng.uniform(-2,2)];reset();u.mem_write(scratch,struct.pack('<f',a));u.reg_write(UC_X86_REG_RCX,scratch);xf(UC_X86_REG_XMM1,b);xf(UC_X86_REG_XMM2,factor);run(0x1416f3330)
 fixtures['circular_lerp'].append((bits(a),bits(b),bits(factor),u.reg_read(UC_X86_REG_XMM0)&0xffffffff))
fixtures['angle_clamp']=[]
for _ in range(50):
 a=rng.uniform(0,6.283185);length=rng.uniform(0,6.283185);angle=rng.uniform(-20,20);reset();xf(UC_X86_REG_XMM0,a);xf(UC_X86_REG_XMM1,length);xf(UC_X86_REG_XMM2,angle);run(0x1425f24d0)
 fixtures['angle_clamp'].append((bits(a),bits(length),bits(angle),u.reg_read(UC_X86_REG_XMM0)&0xffffffff))
fixtures['easing']=[]
for tag in range(7):
 for t in [0.0,0.25,0.5,0.75,1.0]+[rng.random()for _ in range(12)]:
  bias=0.3;reset();u.mem_write(scratch,struct.pack('<If',tag,bias));u.reg_write(UC_X86_REG_RCX,scratch);xf(UC_X86_REG_XMM1,t);run(0x140c95ef0)
  fixtures['easing'].append((tag,bits(t),bits(bias),u.reg_read(UC_X86_REG_XMM0)&0xffffffff))
fixtures['rgb9e5']=[]
for channels in [[0,0,0],[1,1,1],[65408,65408,65408],[-1,1,0]]+[[rng.uniform(-1,1e5)for _ in range(3)]for _ in range(50)]:
 reset();u.mem_write(scratch,struct.pack('<3f',*channels));u.reg_write(UC_X86_REG_RCX,scratch);run(0x1417c66b0)
 fixtures['rgb9e5'].append((*[bits(c)for c in channels],u.reg_read(UC_X86_REG_RAX)&0xffffffff))
report={'binary':str(binary),'sha256':hashlib.sha256(raw).hexdigest(),'library_hooks':functions,'fixtures':fixtures}
(out/'native-fixtures.json').write_text(json.dumps(report,indent=2),encoding='utf8')
print({k:len(v)for k,v in fixtures.items()})
