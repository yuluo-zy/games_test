"""骑乘之后原羊地形消费者：保留原分支/插值体，桥接外部栅格采样与libm。"""
from pathlib import Path
import ctypes, hashlib, itertools, json, struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

root = Path(__file__).resolve().parents[4]
raw = (root / 'B版本内容/tiny-glade.rne').read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
im = pe.get_memory_mapped_image()
u = Uc(UC_ARCH_X86, UC_MODE_64)
u.mem_map(base, (len(im)+4095)&~4095)
u.mem_write(base, im)
data, stack = 0x20000000, 0x30000000
u.mem_map(data, 0x10000)
u.mem_map(stack, 0x10000)
rsp = stack+0x7000
libm = ctypes.CDLL('/usr/lib/libSystem.B.dylib')
libm.expf.argtypes = [ctypes.c_float]
libm.expf.restype = ctypes.c_float
state = {}
def put(a, fmt, *v): u.mem_write(a, struct.pack('<'+fmt, *v))
def get(a, fmt): return struct.unpack('<'+fmt, u.mem_read(a, struct.calcsize('<'+fmt)))
def bits(v): return struct.unpack('<I', struct.pack('<f', v))[0]
def ret():
    sp=u.reg_read(UC_X86_REG_RSP)
    ra=get(sp, 'Q')[0]
    u.reg_write(UC_X86_REG_RSP, sp+8)
    u.reg_write(UC_X86_REG_RIP, ra)
def hook(uc, a, size, user):
    if a == 0x142923ec0:
        val=struct.unpack('<f', struct.pack('<I', uc.reg_read(UC_X86_REG_XMM0)&0xffffffff))[0]
        uc.reg_write(UC_X86_REG_XMM0, bits(libm.expf(val)))
        ret()
    elif a == 0x140a14df0:
        assert uc.reg_read(UC_X86_REG_XMM3)&0xffffffff == bits(130.)
        assert get(uc.reg_read(UC_X86_REG_RSP)+0x28, 'I')[0] == bits(130.)
        state['water_calls'] += 1
        uc.reg_write(UC_X86_REG_XMM0, state['water'])
        ret()
    elif a == 0x14231c37b:
        state['panic'] = True
        uc.emu_stop()
u.hook_add(UC_HOOK_CODE, hook)
cases = list(itertools.product([0,1,2,3,4,5,6,7,255], [-1.,0.,2.], [-.5,0.,3.], [-2.,2.], [-1.,0.,2.], [0.,.016,.25], [-1.,0.,1.]))
# 非有限样本保持原payload。这里只把每个输入单独改NaN，不混入平台invalid-operation的新生NaN。
inputs = [[loc,*map(bits,[h,y,cache,v,dt,water])] for loc,h,y,cache,v,dt,water in cases]
for loc in [0,1,2,4,6]:
    for column in [1,2,3,4,6]:
        row=[loc,*map(bits,[0.,.1,-.2,.3,.016,0.])]
        row[column]=0x7fc01234
        inputs.append(row)
rows=[]
for row in inputs:
    loc,h,y,cache,v,dt,water=row
    u.mem_write(data, b'\0'*0x10000)
    u.mem_write(stack, b'\0'*0x10000)
    put(data+0x14,'I',y)
    put(data+0x100,'II',loc,cache)
    put(data+0x204,'I',v)
    put(data+0x328,'I',dt)
    for offset,p in [(0x38,data+0x200),(0x40,data+0x404),(0x48,data+0x408),(0x90,data+0x300)]:put(rsp+offset,'Q',p)
    put(rsp+0x34,'I',123)
    for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RBP,data),(UC_X86_REG_RSI,data+0x100),(UC_X86_REG_R14,data+0x400),(UC_X86_REG_RAX,data+0x300)]:u.reg_write(reg,val)
    state.update(water=water,water_calls=0,panic=False)
    u.emu_start(0x14231be3c,0x14231be5c,count=100)
    for reg,value in [(UC_X86_REG_XMM0,h),(UC_X86_REG_XMM6,bits(4.)),(UC_X86_REG_XMM7,bits(130.)),(UC_X86_REG_XMM9,bits(-.5)),(UC_X86_REG_XMM10,0),(UC_X86_REG_XMM11,bits(-.3)),(UC_X86_REG_XMM12,bits(.3)),(UC_X86_REG_XMM13,bits(10.)),(UC_X86_REG_XMM14,bits(-.05)),(UC_X86_REG_XMM15,bits(20.))]:u.reg_write(reg,value)
    u.emu_start(0x14231c162,0x14231bf10,count=2000)
    rows.append((row,[get(data+0x14,'I')[0],get(data+0x204,'I')[0],*get(data+0x100,'II'),state['water_calls'],int(state['panic'])]))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-coaster-sheep-terrain.ron'
fixture.write_text('[\n'+''.join('('+str(a).replace(' ','')+','+str(b).replace(' ','')+'),\n' for a,b in rows)+']\n')
out=dict(binary_sha256=sha,cases=len(rows),entry='0x14231c162',exit='0x14231bf10',smoothing_entry='0x14231be3c..be5c',fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),columns=['input(state,h_bits,y_bits,cache_bits,vy_bits,dt_bits,water_bits)','output(y_bits,vy_bits,state,cache_bits,water_calls,panic)'],bridges=['external height sample is supplied h; all following native branches execute','water bilinear supplies controlled value and verifies both130 arguments','Darwin libc expf for original UCRT expf import: cross-library boundary, no claim of all-domain platform equality','state3 original panic entry recorded; panic runtime/logging not executed'],scope='ground2/transition4/panic3/floating6/default bit0 cached payload, velocity and tick side effects originate in original code; ECS query setup excluded')
Path(__file__).with_name('toolbar-coaster-sheep-terrain-native.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(out,ensure_ascii=False))
