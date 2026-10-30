"""原 B 灯槽 callback：真实无桥接执行，覆盖同ID更新/原半精度fallback及新槽分配。
音频调用被原声音计数=4门关闭；只测该明确边界，不声称执行ECS或GPU驱动。
"""
from pathlib import Path
import struct, hashlib, itertools, json, random, pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import *
root = Path(__file__).resolve().parents[3]
raw = (root/'B版本内容/tiny-glade.rne').read_bytes()
digest = hashlib.sha256(raw).hexdigest()
assert digest == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
img = pe.get_memory_mapped_image()
u = Uc(UC_ARCH_X86, UC_MODE_64)
u.mem_map(base, (len(img)+4095)&~4095)
u.mem_write(base,img)
D = 0x20000000
u.mem_map(D, 0x400000)
def q(at,*values): u.mem_write(at, struct.pack('<'+'Q'*len(values),*values))
def words(at,values):u.mem_write(at,struct.pack('<'+'I'*len(values),*values))
# 原std_detect CACHE选择非F16C的原软件分支；不替换任何半精度指令或数学。
q(0x1436307b0,D+0x100)
q(D+0x100,1)
env, variant, oldp, newp = [D+x for x in [0x200,0x300,0x400,0x440]]
count, first, last, free, gpu, audio, snd = [D+x for x in [0x500,0x508,0x510,0x520,0x540,0x560,0x580]]
gpudata = D+0x10000
q(env,audio,free,count,gpu,first,last)
q(audio,snd)
words(snd,[4])
q(gpu,0x8000,gpudata,0x8000)
u.mem_write(D+0x600,struct.pack('<H',5))
rand = random.Random(0x140790400)
rows=[]
ints = [0,0x80000000,0x3f800000,0x3e800000,0x477fe000,0x47800000,0x33800000,0x33000000,0x7f800000,0xff800000,0x7fc01234,0x7fa01234]
ints += [rand.getrandbits(32) for _ in range(32)]
for intensity,newonly,change in itertools.product(ints,[False,True],['same','color','position']):
    old=[17,0x12345678,intensity,5,0x3f800000,0xc0000000,0x40400000]
    new=old.copy()
    new[3]=0
    if change=='color':new[1]^=0x80000001
    if change=='position':new[5]=0x7fc01234
    words(oldp,old);words(newp,new)
    q(variant,*([0,newp,0] if newonly else [2,oldp,newp]))
    q(count,0);q(first,0xffffffffffffffff);q(last,0)
    q(free,1,D+0x600,1)
    u.mem_write(gpudata+5*16,bytes(16))
    rsp=D+0x300008
    q(rsp,D+0x3ff000)
    u.reg_write(UC_X86_REG_RSP,rsp)
    u.reg_write(UC_X86_REG_RCX,env)
    u.reg_write(UC_X86_REG_RDX,variant)
    u.emu_start(0x141257c50,D+0x3ff000,count=1000)
    assert u.reg_read(UC_X86_REG_RIP)==D+0x3ff000
    result=list(struct.unpack('<4I',u.mem_read(gpudata+5*16,16)))
    endcount=struct.unpack('<Q',u.mem_read(count,8))[0]
    bounds=list(struct.unpack('<QQ',u.mem_read(first,16)))
    slot=struct.unpack('<H',u.mem_read(newp+12,2))[0]
    rows.append([old,new,int(newonly),result,endcount,bounds,slot])
p=Path(__file__).with_name('m01-lantern-slots-native-cases.rs')
p.write_text('&[\n'+''.join('('+','.join(str(v).replace(' ','') for v in row)+'),\n' for row in rows)+']\n')
meta={'binary_sha256':digest,'function':'0x141257c50','cases':len(rows),'bridges':[],
      'boundary':'sound counter already4; software f16 original branch; allocator excluded by nonempty free list',
      'fixture_sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
Path(__file__).with_suffix('.json').write_text(json.dumps(meta,indent=2)+'\n')
print(json.dumps(meta))
