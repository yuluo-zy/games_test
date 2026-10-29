"""完整执行原B相机XZ投影及其四元数helper，无函数桥接。"""
from pathlib import Path
import struct,hashlib,json,math,random
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import UC_X86_REG_RSP,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RIP
root=Path("/Users/yuluo/Desktop/game")
raw=(root/"B版本内容/tiny-glade.rne").read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=="f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
pe=pefile.PE(data=raw,fast_load=True)
base=pe.OPTIONAL_HEADER.ImageBase
blob=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64)
u.mem_map(base,(len(blob)+4095)&~4095);u.mem_write(base,blob)
DATA,STACK,STOP=0x20000000,0x30000000,0x200ff000
u.mem_map(DATA,0x100000);u.mem_map(STACK,0x100000)
def bits(x):
    try:return struct.unpack("<I",struct.pack("<f",x))[0]
    except OverflowError:return 0x7f800000 if x>0 else 0xff800000
def execute(words):
    u.mem_write(DATA,bytes(0x1000))
    u.mem_write(DATA+0x1c0,struct.pack("<16I",*words))
    sp=STACK+0xf0008;u.mem_write(sp,struct.pack("<Q",STOP))
    u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RCX,DATA)
    u.reg_write(UC_X86_REG_RDX,DATA+0x100)
    u.emu_start(0x140c96e70,STOP,count=10000)
    assert u.reg_read(UC_X86_REG_RIP)==STOP
    return list(struct.unpack("<4I",u.mem_read(DATA,16)))
identity=[bits(float(i//4==i%4)) for i in range(16)]
cases=[]
def add(name,words):cases.append((name,list(words),execute(words)))
add("identity",identity)
# 相机正规基与滚转/投影阈值两侧；只输入已量化矩阵，不桥接原算法。
for yaw in [0,.1,.7,1.5707963267948966,2.3,3.141592653589793,-.7]:
 for pitch in [0,.3,1.2,1.5707963267948966,-1.2]:
  for roll in [0,.5,1.5707963267948966]:
   cy,sy=math.cos(yaw),math.sin(yaw);cp,sp=math.cos(pitch),math.sin(pitch);cr,sr=math.cos(roll),math.sin(roll)
   x=[cy*cr+sy*sp*sr,cp*sr,-sy*cr+cy*sp*sr,0]
   y=[-cy*sr+sy*sp*cr,cp*cr,sy*sr+cy*sp*cr,0]
   z=[sy*cp,-sp,cy*cp,0]
   add("camera",list(map(bits,x+y+z+[13,-7,41,1])))
for epsilon in [0,1e-8,9.999999e-6,1e-5,1.0000001e-5,1e-4,-1e-5]:
 w=identity.copy();w[0]=bits(epsilon);w[1]=bits(1);w[2]=bits(epsilon)
 add("project_threshold",w)
for i in range(16):
 for value in [0,0x80000000,1,0x80000001,0x00800000,0x7f7fffff,0x7f800000,0xff800000,0x7fc12345,0xffc54321]:
  w=identity.copy();w[i]=value;add("exception",w)
rng=random.Random(0x140c96e70)
for _ in range(512):
 add("matrix",[bits(rng.uniform(-9,9)) for i in range(16)])
out=root/"tiny-glade/reports/module-recovery-2026-10-23"
report={"binary_sha256":sha,"entries":["0x140c96e70","0x140c913e0"],"bridges":[],"case_count":len(cases),"rows":[{"name":n,"matrix_bits":m,"quaternion_bits":q} for n,m,q in cases]}
with (out/"m01-camera-xz-native.json").open("x") as f:json.dump(report,f,indent=2);f.write("\n")
lines=["// 原140c96e70及140c913e0完整执行，无桥接；input为inverse_view列序16word。","const ORIGINAL_CAMERA_XZ: &[([u32;16],[u32;4])] = &["]
lines += ["(["+",".join(hex(x) for x in m)+"],["+",".join(hex(x) for x in q)+"])," for _,m,q in cases]
lines+=["];"]
with (out/"m01-camera-xz-native.rs").open("x") as f:f.write("\n".join(lines)+"\n")
print(json.dumps({"cases":len(cases),"bridges":[],"binary_sha256":sha}))

