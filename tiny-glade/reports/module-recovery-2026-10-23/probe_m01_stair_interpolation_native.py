"""仅执行 B 原插值内核，不调用或替换任何业务 callee。完整发行身份见相邻 evidence manifest。"""
from pathlib import Path
import pefile,struct,json,random
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path(__file__).resolve().parent
p=pefile.PE(str(root.parents[2]/'B版本内容/tiny-glade.rne'),fast_load=True)
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
rng=random.Random(0x7cfe5a)
cases=[]
for a,b in [([0.,0.,0.],[0.,0.,0.]),([-0.,0.,-0.],[0.,-0.,0.]),([1.,2.,3.],[-4.,5.,-6.])]:
 for t0,t1 in [([0.,0.],[0.,0.]),([1.,0.],[-1.,0.]),([0.,1.],[0.,-1.])]:
  for f in [0.,0.5,1.]:cases.append((a,b,t0,t1,f))
for i in range(101):
 cases.append(([rng.uniform(-10,10) for _ in range(3)],[rng.uniform(-10,10) for _ in range(3)], [rng.uniform(-2,2) for _ in range(2)],[rng.uniform(-2,2) for _ in range(2)],rng.uniform(0,1)))
rows=[]
for old,new,ot,nt,f in cases:
 old=list(map(bits,old));new=list(map(bits,new));ot=list(map(bits,ot));nt=list(map(bits,nt));f=bits(f)
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(0x1407cf000,0x2000);u.mem_write(0x1407cf000,p.get_data(0x7cf000,0x2000));u.mem_map(0x200000,0x1000)
 u.mem_write(0x200000,struct.pack('<3I',*new));u.mem_write(0x200100,struct.pack('<2I',*nt))
 for r,v in [(UC_X86_REG_R8,0x200000),(UC_X86_REG_R13,0),(UC_X86_REG_R14,0x200100),(UC_X86_REG_R12,0),(UC_X86_REG_R15,0),(UC_X86_REG_RBX,0)]:u.reg_write(r,v)
 def xmm(r,v):u.reg_write(r,int.from_bytes(struct.pack('<4I',*v),'little'))
 xmm(UC_X86_REG_XMM1,[old[2],0,0,0]);xmm(UC_X86_REG_XMM2,old[:2]+[0,0]);xmm(UC_X86_REG_XMM3,ot+[0,0]);xmm(UC_X86_REG_XMM6,[f]*4);xmm(UC_X86_REG_XMM8,[0]*4);xmm(UC_X86_REG_XMM9,[0x80000000]*4);xmm(UC_X86_REG_XMM10,[bits(1.)]*4);xmm(UC_X86_REG_XMM11,[bits(1.),0,0,0]);xmm(UC_X86_REG_XMM12,[0x3851b717]*4);xmm(UC_X86_REG_XMM13,[f]*4)
 u.emu_start(0x1407cfe5a,0x1407cff72,count=1000)
 out=list(struct.unpack('<3I',u.mem_read(0x200000,12)))+list(struct.unpack('<2I',u.mem_read(0x200100,8)))
 rows.append((old,new,ot,nt,f,out,bool(u.reg_read(UC_X86_REG_R15)&1)))
(root/'m01-stair-interpolation-native-vectors.json').write_text(json.dumps({'range':'1407cfe5a..1407cff72','rows':rows},indent=2))
lines=['const STAIR_INTERPOLATION_NATIVE: &[([u32;3],[u32;3],[u32;2],[u32;2],u32,[u32;5],bool)] = &[']
for old,new,ot,nt,f,out,moved in rows:
 arr=lambda xs:'['+','.join(hex(x) for x in xs)+']'
 lines.append('    ('+','.join([arr(old),arr(new),arr(ot),arr(nt),hex(f),arr(out),str(moved).lower()])+'),')
lines.append('];');(root/'m01-stair-interpolation-native-vectors.rs').write_text('\n'.join(lines)+'\n');print(len(rows))
