"""原7d7600映射与7d75c8相邻高度min；不替换业务callee，保留逐位输入/输出。"""
from pathlib import Path
import hashlib, json, random, struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import *

root = Path(__file__).resolve().parents[3]
out = Path(__file__).resolve().parent
raw = (root / 'B版本内容/tiny-glade.rne').read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
u = Uc(UC_ARCH_X86, UC_MODE_64)
u.mem_map(0x1407d7000, 0x1000)
u.mem_write(0x1407d7000, pe.get_data(0x7d7000, 0x1000))
u.mem_map(0x200000, 0x1000)
def bits(x): return struct.unpack('<I', struct.pack('<f', x))[0]
def xmm(reg, x): u.reg_write(reg, x)
for reg, value in [(UC_X86_REG_XMM6, 1.), (UC_X86_REG_XMM9, .45),
    (UC_X86_REG_XMM10, -.45), (UC_X86_REG_XMM11, .55),
    (UC_X86_REG_XMM12, 3.), (UC_X86_REG_XMM13, .3), (UC_X86_REG_XMM14, .7)]:
    xmm(reg, bits(value))
values = [0, 0x80000000, 1, 0x80000001, bits(.45)-1, bits(.45), bits(.45)+1,
    bits(1.)-1, bits(1.), bits(1.)+1, 0x7f800000, 0xff800000,
    0x7fc00000, 0xffc12345, 0x7f800001]
rng = random.Random(0x7d7600)
values += [bits(rng.uniform(-.5, 1.5)) for _ in range(256)]
factors = []
for value in values:
    u.mem_write(0x200000, struct.pack('<I', value))
    u.reg_write(UC_X86_REG_R14, 0x200000)
    u.emu_start(0x1407d7600, 0x1407d7140, count=100)
    assert u.reg_read(UC_X86_REG_RIP) == 0x1407d7140
    factors.append([value, u.reg_read(UC_X86_REG_XMM0) & 0xffffffff])
mins = []
for sample in values[:15]:
    for height in values[:15]:
        u.mem_write(0x200000, struct.pack('<2I', sample, height))
        u.reg_write(UC_X86_REG_R14, 0x200000)
        u.reg_write(UC_X86_REG_RCX, 0x200008)
        u.reg_write(UC_X86_REG_RBX, 0)
        u.emu_start(0x1407d75c8, 0x1407d75f0, count=100)
        mins.append([sample, height, struct.unpack('<I', u.mem_read(0x200000, 4))[0]])
report = dict(binary_sha256=sha, pdb_sha256='2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911',
    scope='two original scalar blocks only; graph/diff/request parent is not covered', factors=factors, mins=mins)
(out / 'm01-stair-distortion-scalar-native.json').write_text(json.dumps(report, indent=2))
text = 'const ORIGINAL_DISTORTION_FACTORS: &[[u32; 2]] = &[\n' + ''.join(f'    [{a}, {b}],\n' for a,b in factors) + '];\n'
text += 'const ORIGINAL_DISTORTION_MIN: &[[u32; 3]] = &[\n' + ''.join(f'    [{a}, {b}, {c}],\n' for a,b,c in mins) + '];\n'
(out / 'm01-stair-distortion-scalar-native-cases.rs').write_text(text)
print(json.dumps(dict(factors=len(factors), mins=len(mins), binary_sha256=sha)))
