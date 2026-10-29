"""原两完整297B斜率闭包。只桥接graph/endpoint/邻居表/缓存查询资源边界。"""
from pathlib import Path
import hashlib,json,random,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S=0x20000000,0x30000000;u.mem_map(D,0x10000);u.mem_map(S,0x10000);ret=D+0xf000
pack=lambda *x:struct.pack('<'+'f'*len(x),*x)
neighbors=[];visited=[]
def finish():
 sp=u.reg_read(UC_X86_REG_RSP);pc=struct.unpack('<Q',u.mem_read(sp,8))[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,pc)
def hook(uc,pc,size,data):
 if pc==0x1408fe8b0:u.reg_write(UC_X86_REG_RAX,D+0x3000);finish()
 elif pc==0x1408fe890:
  u.mem_write(u.reg_read(UC_X86_REG_RCX),struct.pack('<QQII',1,0,55,0));finish()
 elif pc==0x14125de50:
  assert struct.unpack('<I',u.mem_read(u.reg_read(UC_X86_REG_RDX),4))[0]==55
  u.reg_write(UC_X86_REG_RAX,D+0x4000 if neighbors else 0);finish()
 elif pc==0x140a1ab80:
  node=u.reg_read(UC_X86_REG_R8)&0xffffffff;visited.append(node)
  position=neighbors[node-1];out=u.reg_read(UC_X86_REG_RCX)
  u.mem_write(out,struct.pack('<I',0) if position is None else struct.pack('<I',1)+pack(*position));finish()
u.hook_add(UC_HOOK_CODE,hook)
rng=random.Random(0x1407ad8c0);cases=[]
for _ in range(512):
 anchor=[rng.uniform(-70,70)for _ in range(3)]
 cases.append((anchor,[None if rng.random()<.2 else [rng.uniform(-70,70)for _ in range(3)] for _ in range(rng.randrange(6))]))
for d in [0.,1e-8,1e-5,1.,4.]:
 for dy in [-.250001,-.25,-.249999,0.,.249999,.25,.250001]:cases.append(([0.,0.,0.],[[d,dy*max(d,1e-5),-0.]]))
rows=[]
for anchor,neighbors in cases:
 u.mem_write(D,b'\0'*0x10000)
 u.mem_write(D+0x1000,struct.pack('<QQQQ',D+0x2000,D+0x2100,D+0x2200,D+0x2300))
 u.mem_write(D+0x2000,struct.pack('<Q',D+0x2400));u.mem_write(D+0x2100,struct.pack('<Q',D+0x2500))
 u.mem_write(D+0x2200,pack(anchor[1]));u.mem_write(D+0x2300,pack(anchor[0],anchor[2]))
 u.mem_write(D+0x4000,struct.pack('<QQQ',len(neighbors),D+0x5000,len(neighbors)))
 if neighbors:u.mem_write(D+0x5000,b''.join(struct.pack('<II',i+1,0)for i in range(len(neighbors))))
 outputs=[]
 for entry in [0x1407ad8c0,0x1407ad9f0]:
  sp=S+0xe008;u.mem_write(sp,struct.pack('<Q',ret));u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RCX,D+0x1000)
  visited=[];u.emu_start(entry,ret,count=10000);outputs.append(u.reg_read(UC_X86_REG_RAX)&0xff)
 rawinputs=pack(*anchor)+b''.join(struct.pack('<I',v is not None)+(pack(*v)if v is not None else b'\0'*12)for v in neighbors)
 rows.append(' '.join([str(len(neighbors)),str(outputs[0]),str(outputs[1]),rawinputs.hex()]))
out=Path(__file__).with_suffix('.txt');out.write_text('\n'.join(rows)+'\n')
Path(__file__).with_suffix('.json').write_text(json.dumps({'binary_sha256':sha,'cases':len(rows),'executions':len(rows)*2,'ranges':['1407ad8c0 complete297B','1407ad9f0 complete297B'],'bridges':['1408fe8b0 graph owner','1408fe890 endpoint value','14125de50 resolved adjacency vector','140a1ab80 actual cached query value/None'],'fixture_sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'boundary':'resolved-resource slope closures only; private arithmetic and predicates execute unchanged'},indent=2))
print(len(rows))

# 同一原资源桥接继续执行主函数的整个高度选择块及真实两个闭包，不注入坡度结果。
height_cases=[]
for _ in range(512):
 anchor=[rng.uniform(-30,30),rng.uniform(-2,100),rng.uniform(-30,30)]
 target=[rng.uniform(-30,30),rng.uniform(-2,100),rng.uniform(-30,30)]
 floor=rng.uniform(-2,12)
 height_cases.append((anchor,target,floor,rng.randrange(2),rng.randrange(2),
     [None if rng.random()<.2 else [rng.uniform(-30,30),rng.uniform(-2,100),rng.uniform(-30,30)]for _ in range(rng.randrange(4))]))
for anchor_y in [0.,1.,1.000001,4.,100001.]:
 for target_y in [-1.,0.,2.]:
  for distance in [0.,.8,1.,3.2,4.,8.]:height_cases.append(([0.,anchor_y,0.],[distance,target_y,0.],0.,0,0,[]))
height_rows=[]
for anchor,target,floor,disabled,fixed,neighbors in height_cases:
 u.mem_write(D,b'\0'*0x10000);bp=D+0x1000
 u.mem_write(bp+0x5b0,struct.pack('<Q',D+0x2400));u.mem_write(bp+0x300,struct.pack('<Q',D+0x2500))
 u.mem_write(bp+0x3f8,pack(anchor[1]));u.mem_write(bp+0x488,pack(anchor[0],anchor[2]))
 u.mem_write(bp+0x698,struct.pack('<Q',D+0x7000));u.mem_write(D+0x7000,bytes([disabled]))
 u.mem_write(bp+0x6c8,struct.pack('<Q',D+0x2600));u.mem_write(D+0x2611,bytes([fixed]))
 u.mem_write(D+0x4000,struct.pack('<QQQ',len(neighbors),D+0x5000,len(neighbors)))
 if neighbors:u.mem_write(D+0x5000,b''.join(struct.pack('<II',i+1,0)for i in range(len(neighbors))))
 u.reg_write(UC_X86_REG_RBP,bp);u.reg_write(UC_X86_REG_RSP,S+0xe000)
 for reg,value in [(UC_X86_REG_XMM0,floor),(UC_X86_REG_XMM6,target[1]),(UC_X86_REG_XMM7,target[0]),(UC_X86_REG_XMM8,target[2])]:
  u.reg_write(reg,int.from_bytes(pack(value),'little'))
 visited=[];u.emu_start(0x1407acc79,0x1407acdf5,count=100000)
 result=bytes(u.mem_read(bp+0x4e0,4))
 input_bytes=pack(*anchor,*target,floor)+b''.join(struct.pack('<I',v is not None)+(pack(*v)if v is not None else b'\0'*12)for v in neighbors)
 height_rows.append(' '.join([str(disabled),str(fixed),str(len(neighbors)),input_bytes.hex(),result.hex()]))
height_out=Path(__file__).with_name('1407acc79-stair-height-native.txt');height_out.write_text('\n'.join(height_rows)+'\n')
height_out.with_suffix('.json').write_text(json.dumps({'binary_sha256':sha,'cases':len(height_rows),'ranges':['1407acc79..1407acdf5 including original out-of-line continuation chunks','1407ad8c0 full','1407ad9f0 full'],'bridges':['same four resolved resource queries as slope pack; no slope/math result injection'],'inputs':'actual anchor/target world positions, resolved source floor and two canonical flags; source floor lookup and prior candidate acceptance excluded','fixture_sha256':hashlib.sha256(height_out.read_bytes()).hexdigest(),'boundary':'complete post-lookup staircase height algorithm; not whole Place main or GPU'},indent=2))
print('height',len(height_rows))
