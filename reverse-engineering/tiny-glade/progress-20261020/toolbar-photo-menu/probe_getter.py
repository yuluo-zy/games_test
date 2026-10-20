from pathlib import Path
import hashlib, json, math, struct, subprocess
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
binary = ROOT / 'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
data = binary.read_bytes()
assert hashlib.sha256(data).hexdigest() == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=data, fast_load=True)
uc = Uc(UC_ARCH_X86, UC_MODE_64)
base = pe.OPTIONAL_HEADER.ImageBase
size = (pe.OPTIONAL_HEADER.SizeOfImage + 4095) & ~4095
uc.mem_map(base, size)
uc.mem_write(base, pe.get_memory_mapped_image())
stack, configs, camera, brushes, stop = 0x100000, 0x200000, 0x210000, 0x220000, 0x230000
for addr in (stack, configs, camera, brushes, stop): uc.mem_map(addr, 0x10000)
def pack(v): return struct.pack('<f',v)
def bits(v): return struct.unpack('<I',pack(v))[0]
def hook(uc, addr, size, _):
    if addr in (0x142923f50, 0x142923eb0): # original log2f/exp2f dependencies
        x=struct.unpack('<f',struct.pack('<I',uc.reg_read(UC_X86_REG_XMM0)&0xffffffff))[0]
        v=(math.log2(x) if x>0 else (-math.inf if x==0 else math.nan)) if addr==0x142923f50 else 2.0**x
        uc.reg_write(UC_X86_REG_XMM0,bits(v))
        rsp=uc.reg_read(UC_X86_REG_RSP)
        uc.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',uc.mem_read(rsp,8))[0])
        uc.reg_write(UC_X86_REG_RSP,rsp+8)
uc.hook_add(UC_HOOK_CODE, hook)
focus_offsets=[0x170,0x228,0x2a8,0x348,0x400,0x488]
lens_offsets=[0x18c,0x244,0x2c4,0x364,0x41c,0x4a4]
fixtures=[]
for mode in range(6):
 for sample in range(21):
  values=[(sample-5)/4, .6+sample/20, sample/10, sample/20, sample/100, sample/500]
  lens=[10.,100.,sample*8.,.25,8.,sample/2.]
  distance=2.0**(sample-10)
  tool=[sample/20,sample/4,sample/3]
  uc.mem_write(configs,bytes(0x100)); uc.mem_write(configs+0xc0,b'\x01')
  for off,value in zip([0x1c,0x20,0x24,0x30,0x38,0x34],values):uc.mem_write(configs+0x40+off,pack(value))
  uc.mem_write(camera,bytes(0x500));uc.mem_write(camera+0x4d0,bytes([mode]))
  for off,value in zip([0xc,0x10,8,0x1c,0x20,0x18],lens):uc.mem_write(camera+lens_offsets[mode]+off,pack(value))
  uc.mem_write(camera+focus_offsets[mode]+0x18,pack(distance))
  uc.mem_write(brushes,struct.pack('<3f',*tool))
  for kind in range(13):
   rsp=stack+0x8008
   uc.mem_write(rsp,struct.pack('<Q',stop));uc.mem_write(rsp+0x28,struct.pack('<Q',brushes))
   uc.reg_write(UC_X86_REG_RSP,rsp);uc.reg_write(UC_X86_REG_RCX,kind)
   uc.reg_write(UC_X86_REG_RDX,configs);uc.reg_write(UC_X86_REG_R8,camera)
   uc.reg_write(UC_X86_REG_XMM3,bits(sample/10))
   uc.emu_start(0x142349de0,stop,count=500)
   fixtures.append(dict(mode=mode,kind=kind,values=values,lens=lens,distance=distance,tool=tool,terrain=sample/10,expected=uc.reg_read(UC_X86_REG_XMM0)&0xffffffff))
(HERE/'getter-native-fixtures.json').write_text(json.dumps(fixtures,indent=2))
print(json.dumps({'cases':len(fixtures),'native_getter':'142349de0..142349fd4','dependency_hook':'log2f only','sha256':hashlib.sha256(data).hexdigest()}))

# Execute the real camera-write basic blocks, retaining their ResMut change
# tick writes and selected-camera dispatch, until the original loop tail.
write_fixtures=[]
for mode in range(6):
 for focus_mode in range(4):
  for sample in range(5):
   for kind,entry in [(6,0x142349c4b),(7,0x14234983e),(8,0x142349ce7)]:
    uc.mem_write(camera,bytes(0x500));uc.mem_write(camera+0x4d0,bytes([mode]))
    lo=lens_offsets[mode];fo=focus_offsets[mode]
    for off,value in [(0xc,10.),(0x10,100.),(8,73.),(0x1c,.25),(0x20,8.),(0x18,3.5)]:uc.mem_write(camera+lo+off,pack(value))
    uc.mem_write(camera+fo,struct.pack('<I',focus_mode))
    for off,value in [(4,-10.),(8,25.),(0x10,17.),(0x14,-23.),(0x18,.125)]:uc.mem_write(camera+fo+off,pack(value))
    rsp=stack+0x8000; menu_view=stack+0x1000; menu=stack+0x1100; camera_view=stack+0x1200; tick=stack+0x1300
    uc.mem_write(rsp+0x2c8,struct.pack('<Q',menu_view));uc.mem_write(menu_view,struct.pack('<Q',menu));uc.mem_write(menu,struct.pack('<I',0x13))
    uc.mem_write(rsp+0x2a8,struct.pack('<Q',camera_view));uc.mem_write(camera_view+0x10,struct.pack('<Q',tick));uc.mem_write(camera_view+0x1c,struct.pack('<I',123))
    uc.mem_write(rsp+0xb8,struct.pack('<Q',camera));uc.mem_write(rsp+0x30,struct.pack('<Q',0))
    value=(sample-2)/2
    uc.reg_write(UC_X86_REG_RSP,rsp);uc.reg_write(UC_X86_REG_XMM6,bits(value));uc.reg_write(UC_X86_REG_XMM11,bits(1.0))
    uc.emu_start(entry,0x1423499ca,count=2000)
    write_fixtures.append(dict(mode=mode,kind=kind,focus_mode=focus_mode,value=value,camera=bytes(uc.mem_read(camera,0x4e0)).hex(),tick=struct.unpack('<I',uc.mem_read(tick,4))[0]))
(HERE/'camera-write-native-fixtures.json').write_text(json.dumps(write_fixtures,indent=2))
print(json.dumps({'camera_write_cases':len(write_fixtures),'hook':'original log2f/exp2f dependency only'}))
