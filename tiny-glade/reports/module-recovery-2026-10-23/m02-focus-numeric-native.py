from pathlib import Path
import struct,json,hashlib,random,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path('/Users/yuluo/Desktop/game');raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);u.mem_map(0x20000000,0x40000)
def words(xs):return struct.pack('<'+'I'*len(xs),*xs)
def bits(xs):return list(struct.unpack('<'+'I'*len(xs),struct.pack('<'+'f'*len(xs),*xs)))
def rwords(addr,n):return list(struct.unpack('<'+'I'*n,u.mem_read(addr,n*4)))
rng=random.Random(52719776);rows=[]
for i in range(192):
 count=1+i%13;points=[[rng.uniform(-100,100),rng.uniform(-3,3),rng.uniform(-100,100)] for _ in range(count)]
 if i%17==0: points=[[(-0. if k%2 else 0.),0.,(-0. if k%3 else 0.)] for k in range(count)]
 points_bits=[bits(x) for x in points];ray_bits=bits([rng.uniform(-100,100),rng.uniform(-10,80),rng.uniform(-100,100),rng.uniform(-1,1),rng.choice([-1.,-.01,.01,1.]),rng.uniform(-1,1)]);height_bits=bits([rng.uniform(-20,20)])[0]
 frame,entry,ray,pts=0x20008000,0x20010000,0x20020000,0x20030000
 u.mem_write(pts,words(sum(points_bits,[])));u.mem_write(entry-0x20,struct.pack('<QQ',pts,count));u.mem_write(entry-8,words([height_bits]));u.mem_write(ray,words(ray_bits));u.mem_write(0x20000000,struct.pack('<Q',ray))
 for reg,val in [(UC_X86_REG_RBP,frame),(UC_X86_REG_RCX,entry),(UC_X86_REG_R11,0),(UC_X86_REG_R15,entry),(UC_X86_REG_R14,0x20000000),(UC_X86_REG_RDX,count)]:u.reg_write(reg,val)
 u.emu_start(0x1409a51de,0x1409a5304,count=10000);assert u.reg_read(UC_X86_REG_RIP)==0x1409a5304
 rows.append(dict(points=points_bits,ray=ray_bits,height=height_bits,anchor=rwords(frame+0x178,2),cursor=list(struct.unpack('<2I',u.reg_read(UC_X86_REG_XMM6).to_bytes(16,'little')[:8]))))
# 直接执行原坡度投影与clamp数值块；无外部call/桥接。
falloff=[]
for i in range(256):
 cursor=bits([rng.uniform(-200,200),rng.uniform(-200,200)]);anchor=bits([rng.uniform(-100,100),rng.uniform(-100,100)]);direction=bits([rng.uniform(-2,2),rng.uniform(-2,2)]);offset=bits([rng.uniform(-2,2)])[0]
 if i%19==0:direction=[0,0]
 # 原此块起点已计算start/end；将构造start/end也交由原5840块计算。
 def fv(x):return struct.unpack('<f',words([x]))[0]
 def f(x):return struct.unpack('<f',struct.pack('<f',x))[0]
 start=bits([f(f(fv(direction[j])*2.5)+fv(anchor[j])) for j in range(2)]);end=bits([f(f(fv(direction[j])*7.)+fv(anchor[j])) for j in range(2)])
 for reg,vals in [(UC_X86_REG_XMM10,start),(UC_X86_REG_XMM11,end),(UC_X86_REG_XMM6,cursor),(UC_X86_REG_XMM9,bits([1.]))]:u.reg_write(reg,int.from_bytes(words(vals)+bytes(16-len(vals)*4),'little'))
 u.reg_write(UC_X86_REG_RBP,0x20008000);u.mem_write(0x20008000+0x440,struct.pack('<Q',0x20010000));u.mem_write(0x20010028,words([offset]));u.emu_start(0x1409a59f4,0x1409a5a3d,count=1000)
 falloff.append(dict(cursor=cursor,anchor=anchor,direction=direction,offset=offset,result=u.reg_read(UC_X86_REG_XMM9)&0xffffffff))
rotations=[]
for i in range(256):
 q=bits([rng.uniform(-2,2) for _ in range(4)]);position=bits([rng.uniform(-100,100) for _ in range(3)])
 u.reg_write(UC_X86_REG_XMM9,int.from_bytes(words(q),'little'));u.reg_write(UC_X86_REG_RBP,0x20008000);u.mem_write(0x20008000+0x240,words(position))
 u.emu_start(0x1409a5c8b,0x1409a5d36,count=1000)
 rotations.append(dict(quaternion=q,position=position,result=rwords(0x20008000+0x108,3)))
out=dict(binary_sha256=sha,function='1409a4fb0',scope='原连续数值块：51de..5304锚点/平面投影；59f4..5a3d坡度投影/clamp。无桥接，不宣称执行整个UI系统。坡度两个端点在输入准备处按f32构造，Rust需同时验证。',anchors=rows,falloff=falloff,rotations=rotations)
path=root/'tiny-glade/reports/module-recovery-2026-10-23/m02-focus-numeric-native.json';path.write_text(json.dumps(out,indent=2));print(json.dumps({'anchors':len(rows),'falloff':len(falloff),'output':str(path)}))
