"""执行 B 滚动及 lerp 原指令；唯一依赖桥是宿主 libSystem.expf。"""
from pathlib import Path
import ctypes
import hashlib
import json
import struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
ROOT = Path(__file__).resolve().parents[4]
raw = (ROOT / 'B版本内容/tiny-glade.rne').read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
image = pe.get_memory_mapped_image()
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
cpu.mem_map(base, (len(image) + 4095) & ~4095)
cpu.mem_write(base, image)
data, stack, stop = 0x20000000, 0x30000000, 0x20002000
cpu.mem_map(data, 0x3000)
cpu.mem_map(stack, 0x10000)
libm = ctypes.CDLL('/usr/lib/libSystem.B.dylib')
libm.expf.argtypes, libm.expf.restype = [ctypes.c_float], ctypes.c_float
exp_address = 0x142923ec0
cpu.mem_write(exp_address, b'\xc3')
def exp_bridge(uc, address, size, _):
    if address == exp_address:
        value = struct.unpack('<f', uc.reg_read(UC_X86_REG_XMM0).to_bytes(16, 'little')[:4])[0]
        uc.reg_write(UC_X86_REG_XMM0, int.from_bytes(struct.pack('<f', libm.expf(value)), 'little'))
cpu.hook_add(UC_HOOK_CODE, exp_bridge, begin=exp_address, end=exp_address)
def bits(value):
    return struct.unpack('<I', struct.pack('<f', value))[0]
rows = []
for state in [0, 2, 4, 6, 8]:
    for dt in [0., 1e-6, 1e-5, 0.016, 0.1]:
        for scroll, velocity, previous, target, low, high in [
            (-400., 0., 200., 90., -400., 550.),
            (45., 100., 200., 90., -400., 50.),
            (-405., -100., 180., 90., -400., 550.),
            (10., 5., 180., -20., 30., -30.),
        ]:
            payload = [scroll, velocity, previous, target, low, high]
            cursor = [10., 190.]
            cpu.mem_write(data, struct.pack('<II6f', 0, 0x12345678, *payload))
            cpu.mem_write(data+0x100, bytes(0x100))
            cpu.mem_write(data+0x118, struct.pack('<Q', data+0x300))
            cpu.mem_write(data+0x144, struct.pack('<3f', *cursor, dt))
            cpu.mem_write(data+0x300, bytes([state]))
            rsp = stack + 0xf008
            cpu.mem_write(rsp, struct.pack('<Q', stop))
            for register, value in [(UC_X86_REG_RCX, data+0x200), (UC_X86_REG_RDX, data),
                (UC_X86_REG_R8, data+0x100), (UC_X86_REG_RSP, rsp), (UC_X86_REG_MXCSR, 0x1f80)]:
                cpu.reg_write(register, value)
            cpu.emu_start(0x141dfb700, stop, count=1000)
            assert cpu.reg_read(UC_X86_REG_RIP) == stop
            result = list(struct.unpack('<6I', cpu.mem_read(data+8, 24)))
            assert bytes(cpu.mem_read(data, 8)) == struct.pack('<II', 0, 0x12345678)
            rows.append(dict(state=state, dt=bits(dt), cursor=list(map(bits,cursor)),
                             before=list(map(bits,payload)), after=result))
out = ROOT/'tiny-glade/tests/fixtures/native/toolbar-scroll.ron'
lines = ['[']
for row in rows:
    lines.append('  ('+', '.join(k+': '+(str(tuple(v)) if isinstance(v,list) else str(v)) for k,v in row.items())+'),')
lines.append(']')
out.write_text('\n'.join(lines)+'\n')
metadata = dict(binary_sha256=sha, entry='0x141dfb700', exit='0x141dfb7f1 ret',
    ranges=['0x141dfb700..0x141dfb7f2','0x140c90670..0x140c906cb'],
    bridge='仅将原 expf 依赖调用 0x142923ec0 桥接至宿主 libSystem.expf；原滚动分支、算术及三个lerp调用全部执行。不是原Windows CRT全域判等，也不包含页面/OS/GPU。',
    cases=len(rows), fixture=str(out.relative_to(ROOT)), fixture_sha256=hashlib.sha256(out.read_bytes()).hexdigest())
Path(__file__).with_name('toolbar-scroll-native.json').write_text(json.dumps(metadata,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(metadata,ensure_ascii=False))
