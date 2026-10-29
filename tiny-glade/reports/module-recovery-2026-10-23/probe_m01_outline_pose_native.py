"""发行原140b33b56..d6c，独立执行四分支矩阵转Quat及forward偏移。"""
from pathlib import Path
import pefile,struct,json,random
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
r=Path(__file__).resolve().parent;p=pefile.PE(str(r.parents[2]/'B版本内容/tiny-glade.rne'),fast_load=True)
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
rng=random.Random(0xb33b00)
matrices=[]
for x,y,z in [(1,1,1),(1,-1,-1),(-1,1,-1),(-1,-1,1),(2,3,4),(-2,-3,4),(-2,3,-4),(2,-3,-4)]:
 matrices.append([x,0,0,0,0,y,0,0,0,0,z,0,1.,2.,3.,0])
for i in range(56):matrices.append([rng.uniform(-4,4) if j%4!=3 else 0. for j in range(16)])
rows=[]
for m in matrices:
 for offset in [False,True]:
  words=list(map(bits,m));u=Uc(UC_ARCH_X86,UC_MODE_64)
  for va in [0x140b33000,0x142925000,0x142aec000,0x142a8c000]:u.mem_map(va,0x1000);u.mem_write(va,p.get_data(va-p.OPTIONAL_HEADER.ImageBase,0x1000))
  u.mem_map(0x200000,0x2000);u.reg_write(UC_X86_REG_RBP,0x200800);u.reg_write(UC_X86_REG_RDX,0x201000);u.mem_write(0x201000,struct.pack('<16I',*words));u.mem_write(0x200938,bytes([int(offset)]))
  u.emu_start(0x140b33b56,0x140b33d6c,count=1000)
  q=list(struct.unpack('<4I',u.reg_read(UC_X86_REG_XMM6).to_bytes(16,'little')));pos=list(struct.unpack('<3I',u.mem_read(0x200800,12)))
  rows.append((words,offset,q,pos))
(r/'m01-outline-pose-native-vectors.json').write_text(json.dumps({'range':'140b33b56..140b33d6c','rows':rows},indent=2))
lines=['const OUTLINE_POSE_NATIVE: &[([u32;16],bool,[u32;4],[u32;3])] = &[']
for m,flag,q,pos in rows:
 arr=lambda xs:'['+','.join(hex(v) for v in xs)+']'
 lines.append('    ('+','.join([arr(m),str(flag).lower(),arr(q),arr(pos)])+'),')
lines.append('];');(r/'m01-outline-pose-native-vectors.rs').write_text('\n'.join(lines)+'\n');print(len(rows))
