"""有界执行原烟囱算术/完整builder写面与40B载荷；不模拟World或资产查询结果。

容器/缓存值是显式原ABI输入，lower placement的布尔分支另由真实World验证。
厚度39B原片段另由Rosetta直接执行，防止Unicorn双NaN传播失真。
"""
from pathlib import Path
import hashlib, json, struct, subprocess, tempfile, random
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import *

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
raw = (ROOT / 'B版本内容/tiny-glade.rne').read_bytes()
SHA = hashlib.sha256(raw).hexdigest()
assert SHA == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
u = Uc(UC_ARCH_X86, UC_MODE_64)
u.mem_map(base, (pe.OPTIONAL_HEADER.SizeOfImage + 4095) & ~4095)
u.mem_write(base, pe.get_memory_mapped_image())
u.mem_map(0x20000000, 0x10000)
FRAME, BLUE, BUILDERS, EXTRA, PARAMS = 0x20008000, 0x20001000, 0x20002000, 0x20003000, 0x20004000

def bits(x): return struct.unpack('<I', struct.pack('<f', x))[0]
def f(x): return struct.unpack('<f', struct.pack('<I', x))[0]
def reg(n, *values):
    values = list(values) + [0.0] * (4 - len(values))
    u.reg_write(globals()['UC_X86_REG_XMM' + str(n)], int.from_bytes(struct.pack('<4f', *values), 'little'))
def wordreg(n): return u.reg_read(globals()['UC_X86_REG_XMM' + str(n)]) & 0xffffffff
def put(addr, fmt, *values): u.mem_write(addr, struct.pack(fmt, *values))
def run(start, end):
    u.emu_start(start, end, count=20000)
    assert u.reg_read(UC_X86_REG_RIP) == end
def words(addr, n): return list(struct.unpack('<' + 'I' * n, u.mem_read(addr, n * 4)))
def transform(addr, q, p, s): put(addr, '<10f8x', *q, *p, *s)
def reset(q, p, s, extra, smoke, state, material):
    u.mem_write(0x20000000, bytes(0x10000))
    for name in ['RAX','RBX','RCX','RDX','RSI','RDI','R8','R9','R10','R11','R12','R13','R14','R15']:
        u.reg_write(globals()['UC_X86_REG_' + name], 0)
    for n in range(16): reg(n)
    u.reg_write(UC_X86_REG_RBP, FRAME)
    u.reg_write(UC_X86_REG_RSP, FRAME - 0x700)
    u.reg_write(UC_X86_REG_MXCSR, 0x1f80)
    put(BLUE + 0xb0, '<QQQ', 8, BUILDERS, 0)
    put(EXTRA, '<fB', extra, smoke)
    put(FRAME + 0x1b8, '<Q', EXTRA)
    put(FRAME + 0x328, '<Q', BLUE)
    put(FRAME + 0x1f8, '<I', material)
    put(FRAME + 0x21c, '<I', material)
    put(FRAME + 0x355, '<B', state)
    put(FRAME + 0x267, '<B', state)

def builder(index):
    addr = BUILDERS + index * 0x160
    ptr, n = struct.unpack('<QQ', u.mem_read(addr + 0xb0, 16))
    name = bytes(u.mem_read(ptr, n)).decode()
    payload = words(addr + 0xc0, 10)
    smoke = list(u.mem_read(addr + 0x14c, 3))
    return dict(transform=payload, smoke=smoke, nani=name)

roof, wall, pipe = [], [], []
quats = [(0.,0.,0.,1.), (0.,0.6,0.,0.8), (0.,-1.,0.,0.), (0.6,0.,0.8,0.)]
for qi, q in enumerate(quats):
    for pi, position in enumerate([(4., 2., -3.), (-11.125, 1.000001, 7.25)]):
        for ei, extra in enumerate([-2.5, 0., 1.4, 17.125]):
            scale = (1.0 + pi, 0.5 + qi * 0.25, 1.5)
            smoke, state, material = ei, pi, 0x10203040 + qi
            inp = [bits(v) for v in (*q, *position, *scale, extra)] + [smoke, state, material]
            # 原roof：烟雾builder；半高/中心；collider之前的max_y查询作为显式边界。
            reset(q, position, scale, extra, smoke, state, material)
            transform(FRAME + 0xe0, q, position, scale)
            u.reg_write(UC_X86_REG_RAX, EXTRA)
            u.reg_write(UC_X86_REG_R13, BLUE)
            reg(8, 3.0)
            run(0x1407c122f, 0x1407c150c)
            visual = builder(0)
            run(0x1407c150c, 0x1407c1559)
            center = [wordreg(9), wordreg(13), wordreg(10)]
            reg(11, position[1]); reg(6, 8.0); reg(12, 1.0)
            u.reg_write(UC_X86_REG_RAX, PARAMS); u.reg_write(UC_X86_REG_RDI, PARAMS)
            run(0x1407c16bc, 0x1407c1791)
            roof.append(dict(input=inp, builder=visual, center=center, params=words(PARAMS, 10)))
            # 原wall：完整主体变换、烟顶和40B装配写面；lower真假两路均保留。
            for lower in [False, True]:
                reset(q, position, scale, extra, smoke, state, material)
                transform(FRAME + 0x200, q, position, scale)
                reg(7, 8.0); reg(14, 2.0)
                run(0x1407c2c00, 0x1407c2d6b)
                body = words(FRAME + 0x200, 10)
                half, height, without_extra = wordreg(11), wordreg(15), wordreg(7)
                transform(FRAME - 16, q, position, scale)
                u.reg_write(UC_X86_REG_R12, BLUE)
                run(0x1407c3b80, 0x1407c3ed6)
                visual = builder(0)
                run(0x1407c3ed6, 0x1407c3ef8)
                reg(11, f(half)); reg(12, 2.0); reg(7, f(without_extra))
                u.reg_write(UC_X86_REG_RDI, int(lower))
                u.reg_write(UC_X86_REG_RAX, PARAMS)
                run(0x1407c3f17, 0x1407c4017)
                wall.append(dict(input=inp, lower=lower, body=body, builder=visual, params=words(PARAMS, 10)))
            # 原vent：两个完整builder写面，资产/三角形收集边界不参与位姿计算。
            reset(q, position, scale, extra, smoke, state, material)
            transform(FRAME + 0x200, q, position, scale)
            u.reg_write(UC_X86_REG_RDX, BLUE)
            run(0x1407c2ffd, 0x1407c32a2)
            first = builder(0)
            u.reg_write(UC_X86_REG_R12, BLUE)
            run(0x1407c34a0, 0x1407c4017)
            pipe.append(dict(input=inp, builders=[first, builder(1)]))

# 同一原39B片段，独立x86原执行，不用Unicorn浮点结果作最终oracle。
values = [0,0x80000000,0x3f000000,0x3f800000,0x7f800000,0xff800000,0x7fc12345,0xffc54321,0x7f812345]
steps = []
for old in values:
    for delta in values:
        for sign in [0x3f800000,0xbf800000]:
            for target in [0x3f800000,0x40200000]: steps.append([old,sign,target,delta])
with tempfile.TemporaryDirectory(prefix='m01-original-smoke-') as td:
    td = Path(td); fragment = td/'fragment.bin'; inputs = td/'inputs.bin'; outputs = td/'outputs.bin'; binary = td/'step-x86'
    native = pe.get_data(0x8197f7, 39)
    fragment.write_bytes(native)
    inputs.write_bytes(b''.join(struct.pack('<4I', *row) for row in steps))
    source = OUT/'m01-hints-smoke-step-original.c'
    subprocess.run(['clang','-arch','x86_64','-O2',str(source),'-o',str(binary)], check=True)
    subprocess.run(['arch','-x86_64',str(binary),str(fragment),str(inputs),str(outputs)], check=True)
    results = struct.unpack('<'+'I'*len(steps), outputs.read_bytes())
    smoke_step = [dict(input=row, output=result) for row,result in zip(steps,results)]
    execution = dict(fragment_sha256=hashlib.sha256(native).hexdigest(), source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(), binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest())

report = dict(original_sha256=SHA, scope=__doc__, execution=execution, roof=roof, wall=wall, pipe=pipe, smoke_step=smoke_step)
path = OUT/'m01-hints-chimney-original-fixture.json'
with path.open('x') as dst: json.dump(report, dst, indent=2)
lines = ['// 原RNE执行夹具；见同名JSON的SHA与有界执行合同。']
for name, rows, formatter in [
    ('ROOF',roof,lambda r: (r['input'],r['builder']['transform'],r['center'],r['params'])),
    ('WALL',wall,lambda r: (r['input'],r['lower'],r['body'],r['builder']['transform'],r['params'])),
    ('PIPE',pipe,lambda r: (r['input'],r['builders'][0]['transform'],r['builders'][1]['transform'])),
    ('SMOKE_STEP',smoke_step,lambda r: (r['input'],r['output'])),
]:
    types = {'ROOF':'([u32;14],[u32;10],[u32;3],[u32;10])','WALL':'([u32;14],bool,[u32;10],[u32;10],[u32;10])','PIPE':'([u32;14],[u32;10],[u32;10])','SMOKE_STEP':'([u32;4],u32)'}
    lines.append(f'pub const {name}_CASES: &[{types[name]}] = &[')
    for row in rows:
        values = formatter(row)
        lines.append('    (' + ','.join(json.dumps(x).lower() for x in values) + '),')
    lines.append('];')
with (OUT/'m01-hints-chimney-original-cases.rs').open('x') as dst: dst.write('\n'.join(lines)+'\n')
print(json.dumps(dict(roof=len(roof),wall=len(wall),pipe=len(pipe),smoke_step=len(smoke_step),original_sha256=SHA)))
