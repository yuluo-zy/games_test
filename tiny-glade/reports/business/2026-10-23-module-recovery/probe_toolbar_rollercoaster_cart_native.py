"""执行原版每车 1ms 动态指令，不桥接候选算法。

边界：调用方提供 Spline 的前后未归一化切线、dolly look_at 四元数和速度。
J/B/C/peak 与 A 两个原指令片段之间跳过羊随机事件发送；不执行 ECS 查询。
"""
from pathlib import Path
import hashlib, itertools, json, math, struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

ROOT = Path(__file__).resolve().parents[4]
RAW = (ROOT / 'B版本内容/tiny-glade.rne').read_bytes()
SHA = hashlib.sha256(RAW).hexdigest()
assert SHA == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
PE = pefile.PE(data=RAW, fast_load=True)
BASE = PE.OPTIONAL_HEADER.ImageBase
IMAGE = PE.get_memory_mapped_image()
U = Uc(UC_ARCH_X86, UC_MODE_64)
U.mem_map(BASE, (len(IMAGE) + 4095) & ~4095)
U.mem_write(BASE, IMAGE)
DATA, STACK = 0x20000000, 0x30000000
U.mem_map(DATA, 0x10000)
U.mem_map(STACK, 0x10000)
BP = STACK + 0x8000
CART, STATE, SHEEP, FLAGS = DATA, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000

def f32(x): return struct.unpack('<f', struct.pack('<f', x))[0]
def bits(x): return struct.unpack('<I', struct.pack('<f', x))[0]
def put(addr, fmt, *xs): U.mem_write(addr, struct.pack('<' + fmt, *xs))
def xmm(reg, xs): U.reg_write(reg, int.from_bytes(struct.pack('<4f', *xs), 'little'))
def outputs(addr, count): return list(struct.unpack('<' + 'I' * count, U.mem_read(addr, count * 4)))
def run(start, end):
    def stop(uc, address, size, data):
        if address == end: uc.emu_stop()
    h = U.hook_add(UC_HOOK_CODE, stop)
    try: U.emu_start(start, BASE + len(IMAGE), count=2000)
    finally: U.hook_del(h)
    assert U.reg_read(UC_X86_REG_RIP) == end, hex(U.reg_read(UC_X86_REG_RIP))

# 四元数允许直接作为输入边界；同时覆盖单位与略偏离单位值，避免隐含 normalize。
profiles = [
    ([1., 0., 0.], [1., 0., 0.], [0., 0., 0., 1.]),
    ([.8, .3, -.2], [.7, -.1, .5], [0., .70710677, 0., .70710677]),
    ([1.3, -.4, .2], [.9, .8, -.3], [.2, -.3, .1, math.sqrt(.86)]),
    ([-.1, 1.2, -.5], [.4, -.9, .6], [.5, .5, .5, .5]),
    ([0., -0., 0.], [0., 0., -0.], [0., 0., 0., 1.]),
    ([.5, .5, .5], [-.5, -.5, -.5], [.15, -.25, .35, .88]),
]
initials = [
    ([0., 0., 0.], [0., 0., 0.], [0., 0., 0.], 0.),
    ([.2, -.3, .4], [.01, -.02, .03], [.4, -.5, .6], .026),
    ([-.2, .3, -.4], [.999, -1., .999], [-.4, .5, -.6], .024),
    ([1., -1., .5], [-.999, 1., -1.], [1., -1., 1.], .5),
    ([0., 0., 0.], [.001, -.001, .001], [0., 0., 0.], .026),
    ([-0., 0., -0.], [-0., 0., -0.], [-0., 0., -0.], -0.),
]
rows = []
cases = list(itertools.product(range(len(profiles)), [(2., 2.), (15., 14.5), (49., 50.)], [0, 1], [0, 1], [-1., -0., 0., 1.], range(len(initials))))
main_count = len(cases)
# 原稳定有限主集保持不变，非有限增补单独落盘，每次只改变一个字段。
specials = [struct.unpack('<f', struct.pack('<I', b))[0] for b in [0x7fc01234, 0x7f800000, 0xff800000]]
for special in specials:
    for field in range(10):
        vectors = [[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]]
        peak = 0.
        if field < 9: vectors[field // 3][field % 3] = special
        else: peak = special
        initials.append((*vectors, peak))
        for selected in [0, 1]: cases.append((0, (2., 2.), selected, 0, 1., len(initials)-1))
    for field in range(10):
        old_tangent, tangent, quat = [x.copy() for x in profiles[0]]
        if field < 3: old_tangent[field] = special
        elif field < 6: tangent[field-3] = special
        else: quat[field-6] = special
        profiles.append((old_tangent, tangent, quat))
        for selected in [0, 1]: cases.append((len(profiles)-1, (2., 2.), selected, 0, 1., 0))
for profile, (old_speed, speed), selected, sheep, steer, initial in cases:
    old_tangent, tangent, quat = profiles[profile]
    old_tangent, tangent, quat = [list(map(f32, x)) for x in [old_tangent, tangent, quat]]
    inertia, lean, acceleration, peak = initials[initial]
    sheep_scale = [.3, .5, .4] if sheep else [0., 0., 0.]
    U.mem_write(CART, bytes(0x1b0)); U.mem_write(STATE, bytes(0x50)); U.mem_write(FLAGS, bytes(8))
    U.mem_write(BP, bytes(0x800))
    for reg in range(UC_X86_REG_XMM0, UC_X86_REG_XMM15 + 1): U.reg_write(reg, 0)
    U.reg_write(UC_X86_REG_RBP, BP); U.reg_write(UC_X86_REG_RSP, STACK + 0x7000)
    U.reg_write(UC_X86_REG_R15, CART); U.reg_write(UC_X86_REG_R14, STATE)
    U.reg_write(UC_X86_REG_RSI, 0 if selected else 1)
    put(STATE + 0x44, 'f', speed)
    put(CART + 0x17c, '3f', *inertia); put(CART + 0x188, '3f', *lean)
    put(CART + 0x194, '3f', *acceleration); put(CART + 0x1a0, 'f', peak)
    put(SHEEP + 0x3c, '3f', *sheep_scale)
    put(BP + 0x318, 'Q', SHEEP); put(BP + 0x768, 'Q', SHEEP if sheep else 0)
    put(BP + 0x738, 'Q', 1); put(BP + 0x718, 'Q', FLAGS); put(BP + 0x770, 'Q', 0)
    put(BP + 0x530, '4f', *quat); put(BP + 0x2d0, '4f', old_speed, old_speed, 0., 0.)
    put(BP + 0x700, '4f', tangent[0], 0., 0., 0.)
    put(BP + 0x740, '4f', tangent[1], tangent[2], 0., 0.)
    put(BP + 0xa0, 'f', steer); put(BP + 0x40, '4f', steer, steer, 0., 0.)
    signed_zero = math.copysign(0., steer)
    put(BP + 0x20, '4f', f32(f32(steer * f32(.001)) * 5.), signed_zero, 0., 0.)
    put(BP + 0x30, '4f', signed_zero, signed_zero, 0., 0.)
    xmm(UC_X86_REG_XMM12, [f32(old_tangent[0] * old_speed), 0., 0., 0.])
    xmm(UC_X86_REG_XMM14, [old_tangent[1], old_tangent[2], 0., 0.])
    run(0x1403b8d68, 0x1403b9260)
    # 原9263从常量重载1。9260..94b0是羊触发事件，不属于此夹具边界。
    xmm(UC_X86_REG_XMM8, [1., 0., 0., 0.])
    run(0x1403b94b0, 0x1403b95d9)
    rows.append([selected, sheep, bits(old_speed), bits(speed), list(map(bits, old_tangent)), list(map(bits, tangent)), list(map(bits, quat)), bits(steer), list(map(bits, inertia)), list(map(bits, lean)), list(map(bits, acceleration)), bits(peak), list(map(bits, sheep_scale)), outputs(CART + 0x17c, 3), outputs(CART + 0x188, 3), outputs(CART + 0x194, 3), outputs(CART + 0x1a0, 1)[0], U.mem_read(FLAGS, 1)[0]])

def ron(x):
    if isinstance(x, list): return '[' + ','.join(map(ron, x)) + ']'
    return str(x)
fixture = ROOT / 'tiny-glade/tests/fixtures/native/toolbar-rollercoaster-cart-dynamics.ron'
def save(path, records):
    path.write_text('[\n' + ''.join('((' + ','.join(map(ron, row[:13])) + '),(' + ','.join(map(ron, row[13:])) + ')),\n' for row in records) + ']\n')
save(fixture, rows[:main_count])
extra_fixture = fixture.with_name('toolbar-rollercoaster-cart-dynamics-nonfinite.ron')
save(extra_fixture, rows[main_count:])
report = {
    'binary_sha256': SHA, 'pdb_sha256': '2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911',
    'cases': main_count, 'fixture': str(fixture.relative_to(ROOT)),
    'fixture_sha256': hashlib.sha256(fixture.read_bytes()).hexdigest(),
    'nonfinite_cases': len(rows)-main_count,
    'nonfinite_fixture': str(extra_fixture.relative_to(ROOT)),
    'nonfinite_fixture_sha256': hashlib.sha256(extra_fixture.read_bytes()).hexdigest(),
    'executed_ranges': [['0x1403b8d68', '0x1403b9260'], ['0x1403b94b0', '0x1403b95d9']],
    'bridges': [],
    'supplied_boundaries': ['调用方提供前后未归一化Spline切线、前后速度、dolly look_at四元数', '私有ABI Cart/State/羊形状字段、选择索引和横向输入', '跳过羊随机事件发送，原9263常量1在进入A片段前重载'],
    'columns': ['selected_u32', 'sheep_u32', 'old_speed_bits', 'speed_bits', 'old_tangent_bits3', 'tangent_bits3', 'look_quat_bits4', 'steer_bits', 'old_J_bits3', 'old_BC_bits3', 'old_A_bits3', 'old_peak_bits', 'sheep_scale_bits3', 'J_bits3', 'BC_bits3', 'A_bits3', 'peak_bits', 'cross_flag_u8'],
    'serialization': '((前13输入列),(后5输出列))；向量是RON序列[]，可用Vec<u32>读取并检查维度，避免serde大于16 tuple限制。',
    'scope': '原每车1ms J/B/C/peak/A指令；不覆盖速度/t、Spline求值、look_at、随机羊事件、ECS/摄像机/音频消费。尚需Rust候选对照。',
}
Path(__file__).with_name('toolbar-rollercoaster-cart-dynamics-native.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')
print(json.dumps({'cases': main_count, 'nonfinite_cases': len(rows)-main_count, 'fixture_sha256': report['fixture_sha256'], 'nonfinite_fixture_sha256': report['nonfinite_fixture_sha256']}))
