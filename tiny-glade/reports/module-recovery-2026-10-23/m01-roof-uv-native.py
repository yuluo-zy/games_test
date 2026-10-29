"""原B屋顶单形状查询的转换算法：整段原函数执行，未桥接浮点运算。"""
from pathlib import Path
import struct, hashlib, json, random
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import *
ROOT = Path(__file__).resolve().parents[3]
raw = (ROOT/'B版本内容/tiny-glade.rne').read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
image, base = pe.get_memory_mapped_image(), pe.OPTIONAL_HEADER.ImageBase
u = Uc(UC_ARCH_X86, UC_MODE_64)
u.mem_map(base, (len(image)+4095)&~4095); u.mem_write(base,image)
D=0x20000000; u.mem_map(D,0x10000)
def fpack(values): return struct.pack('<'+'f'*len(values),*values)
def bits(values): return list(struct.unpack('<'+'I'*len(values),fpack(values)))
def put(offset,values): u.mem_write(D+offset,fpack(values))
rng=random.Random(0x8e4090)
rows=[]
for i in range(259):
    q = [[0.,0.,0.,1.],[0.,0.7071067690849304,0.,0.7071067690849304],[-0.,0.,-0.,1.]][i] if i<3 else [rng.uniform(-1.,1.) for _ in range(4)]
    translation=[rng.uniform(-20.,20.) for _ in range(3)]
    ray=[rng.uniform(-20.,20.) for _ in range(6)]
    toi=rng.uniform(-5.,15.)
    vertices=[rng.uniform(-4.,4.) for _ in range(9)]
    normals=[rng.uniform(-1.,1.) for _ in range(9)]
    uvs=[rng.uniform(-2.,2.) for _ in range(6)]
    fallback=[rng.uniform(-1.,1.) for _ in range(3)]
    # 原mesh枚举的完整字段偏移由1408ce4a0读取；不强行套当前Rust布局。
    u.mem_write(D+0x1000,bytes(0x100)); u.mem_write(D+0x2000,bytes(0x100))
    u.mem_write(D+0x1018,struct.pack('<Q',D+0x2000))
    for offset, value in [(0x10,D+0x4000),(0x18,3),(0x20,D+0x4100),(0x28,1),(0x40,D+0x4200),(0x48,3),(0x50,D+0x4300),(0x58,3)]:
        u.mem_write(D+0x2000+offset,struct.pack('<Q',value))
    put(0x4000,vertices); u.mem_write(D+0x4100,struct.pack('<3I',0,1,2))
    put(0x4200,normals); put(0x4300,uvs); put(0x3000,ray); put(0x3200,q+translation)
    u.mem_write(D+0x3100,struct.pack('<II4f',2,i%2,toi,*fallback))
    u.mem_write(D+0x3300,bytes(0x80)); u.mem_write(D+0x8008,struct.pack('<Q',D+0xff00))
    u.mem_write(D+0x8030,struct.pack('<Q',D+0x1000))
    for reg, value in [(UC_X86_REG_RSP,D+0x8008),(UC_X86_REG_RCX,D+0x3300),(UC_X86_REG_RDX,D+0x3000),(UC_X86_REG_R8,D+0x3100),(UC_X86_REG_R9,D+0x3200)]: u.reg_write(reg,value)
    u.emu_start(0x1408ce4a0,D+0xff00,count=4096)
    assert u.reg_read(UC_X86_REG_RIP)==D+0xff00
    result=bytes(u.mem_read(D+0x3300,0x40))
    assert struct.unpack_from('<I',result,0x34)[0]==1
    rows.append((bits(ray),bits(q),bits(translation),bits([toi]),bits(vertices),bits(normals),bits(uvs),list(struct.unpack_from('<6I',result,0x18)),list(struct.unpack_from('<2I',result,0x38))))
target=Path(__file__).with_name('m01-roof-uv-native-cases.rs')
target.write_text('&[\n'+''.join('('+','.join(str(v) for v in row)+'),\n' for row in rows)+']\n')
meta=dict(binary_sha256=sha,entry='0x1408ce4a0',cases=len(rows),arithmetic_bridges=[],scope='full original converter, including native barycentric 1416c2f20; single shape itself tested separately in actual Rapier World',fixture_sha256=hashlib.sha256(target.read_bytes()).hexdigest())
Path(__file__).with_suffix('.json').write_text(json.dumps(meta,indent=2)+'\n'); print(json.dumps(meta))
