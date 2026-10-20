from pathlib import Path
import pefile,struct,itertools,json,hashlib,random
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
H=Path('reverse-engineering/tiny-glade/progress-20261020/phase2-undo-systems');R=Path('.')
data=(R/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes();assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=data,fast_load=True);img=p.get_memory_mapped_image();base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(img)+4095)&~4095);u.mem_write(base,img);u.mem_map(0x100000,0x100000)
def q(a,v):u.mem_write(a,struct.pack('<Q',v))
def f(a,v):u.mem_write(a,struct.pack('<f',v))
def getq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
blob=bytearray();n=0;rng=random.Random(0x3449f0)
for alignment,enabled,old,menu,phase in itertools.product(range(4),(False,True),(0.,-0.,.5,1.,float("nan")),(2,3,10),range(5)):
 for cursor,dt in [((0.,0.),.06),((640.,55.),.06),((640.,720.),.06),((55.,360.),.06),((1225.,360.),.06),((640.,665.),.06),((640.,360.),-.06)]:
  icon=rng.choice([.5,1.,1.25,2.]);window=(1280,720)
  for wrapper,target in [(0x110000,0x120000),(0x110100,0x120100),(0x110200,0x120200),(0x110300,0x120400),(0x110400,0x120500),(0x110500,0x120600),(0x110600,0x120800),(0x110700,0x120900),(0x110800,0x121000)]:
   u.mem_write(wrapper,bytes(32));q(wrapper,target);q(wrapper+16,0x122000)
  f(0x120000,cursor[0]);f(0x120004,cursor[1]);q(0x120100,menu);u.mem_write(0x120108,bytes([0]))
  f(0x1202e8,icon);u.mem_write(0x1202f0,bytes([alignment,enabled]));f(0x120458,dt);u.mem_write(0x120500,struct.pack('<II',*window));u.mem_write(0x120600,struct.pack('<If',phase,0.))
  q(0x120800,0x2200000001);q(0x120808,0x3300000002);f(0x120810,old)
  q(0x1209a0,0x123000);q(0x1209a8,1);u.mem_write(0x123000,bytes(40));q(0x123010,0x2200000001)
  q(0x121020,64);q(0x121028,0x124000);q(0x121030,0);q(0x121040,7)
  stack=0x180008;q(stack,0x1f0000)
  for a,v in [(0x28,0x110400),(0x30,0x110500),(0x38,0x110600),(0x40,0x110700),(0x48,0x110800)]:q(stack+a,v)
  for reg,v in [(UC_X86_REG_RSP,stack),(UC_X86_REG_RCX,0x110000),(UC_X86_REG_RDX,0x110100),(UC_X86_REG_R8,0x110200),(UC_X86_REG_R9,0x110300)]:u.reg_write(reg,v)
  u.emu_start(0x1423449f0,0x1f0000,count=3000)
  assert u.reg_read(UC_X86_REG_RIP)==0x1f0000
  progress=bytes(u.mem_read(0x120810,4));offset=bytes(u.mem_read(0x123008,8));event_count=getq(0x121030);assert event_count in (0,1)
  sound=struct.unpack('<I',u.mem_read(0x124000,4))[0] if event_count else 0xffffffff
  blob+=struct.pack('<4BI2I5f',alignment,enabled,menu,0,phase,*window,icon,old,dt,*cursor)+progress+offset+struct.pack('<I',sound);n+=1
(H/'native-auto-hide.bin').write_bytes(blob);(H/'native-auto-hide-manifest.json').write_text(json.dumps({'cases':n,'stride':52,'hooks':0,'original':['1423449f0','142344de8'],'binary_sha256':hashlib.sha256(data).hexdigest(),'fixtures_sha256':hashlib.sha256(blob).hexdigest()},indent=2));print(n,'full native system cases; zero hooks')
