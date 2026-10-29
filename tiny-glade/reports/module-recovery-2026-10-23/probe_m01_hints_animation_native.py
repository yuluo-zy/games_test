"""执行 B 原 HintAnimation 算术与原 CubicInOut；仅提供已解析 ECS 行和帧时间。"""
from pathlib import Path
import hashlib, itertools, json, random, struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import *

ROOT = Path(__file__).resolve().parents[3]
raw = (ROOT / 'B版本内容/tiny-glade.rne').read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
image = pe.get_memory_mapped_image()
u = Uc(UC_ARCH_X86, UC_MODE_64)
u.mem_map(base, (len(image)+4095)&~4095)
u.mem_write(base, image)
data, stack = 0x20000000, 0x30000000
u.mem_map(data, 0x10000)
u.mem_map(stack, 0x10000)
sp = stack + 0x8000

def put(addr, fmt, *xs):
    u.mem_write(addr, struct.pack('<'+fmt, *xs))

def bits(x):
    return struct.unpack('<I', struct.pack('<f', x))[0]

values = [bits(x) for x in [-1., -0., 0., .1, .49, .5, .9, 1., 2., float('inf'), -float('inf')]] + [0x7fc00000, 0x7fc12345, 1]
times = [bits(x) for x in [-0., 0., .7, 12.]]
deltas = [0, 1_000_000, 16_666_667, 100_000_000, 300_000_000, 3_000_000_000]
cases = list(itertools.product(values, times, deltas, [0, 1]))
rng = random.Random(0x141c4f400)
cases += [(bits(rng.uniform(-.5, 1.5)), bits(rng.uniform(0, 60)), rng.randrange(500_000_001), rng.randrange(2)) for _ in range(2048)]
rows = []
for value, age, duration_ns, deleting in cases:
    # 与 Duration::as_secs_f32 相同的两次 f32 转换和运算，提供 Time 可真实表示的帧时间。
    f32 = lambda x: struct.unpack('<f', struct.pack('<f', x))[0]
    delta = bits(f32(f32(duration_ns // 1_000_000_000) + f32(f32(duration_ns % 1_000_000_000) / f32(1_000_000_000))))
    u.mem_write(data, bytes(0x200))
    u.mem_write(sp-0x100, bytes(0x200))
    put(data, '3I', value, 0xdeadbeef, age)
    put(data+0x158, 'I', delta)
    put(sp+0x20, 'Q', deleting)
    put(sp+0x48, 'Q', data+0x100)
    u.reg_write(UC_X86_REG_RSP, sp)
    u.reg_write(UC_X86_REG_RBX, 0)
    u.reg_write(UC_X86_REG_RDI, data)
    u.reg_write(UC_X86_REG_RSI, data+0x80)
    u.reg_write(UC_X86_REG_R11, data+0x100)
    u.reg_write(UC_X86_REG_XMM6, bits(.3))
    u.reg_write(UC_X86_REG_XMM7, 0x80000000)
    u.reg_write(UC_X86_REG_XMM8, bits(1.))
    # 包含除法、删除方向、SSE clamp、原 easing 调用以及 time+=dt；停在 ECS 下一行迭代前。
    u.emu_start(0x141c4f58c, 0x141c4f4e6, count=1000)
    assert u.reg_read(UC_X86_REG_RIP) == 0x141c4f4e6
    output = struct.unpack('<3I', u.mem_read(data, 12))
    rows.append((value, age, duration_ns, delta, bool(deleting), *output))
fixture = ROOT / 'tiny-glade/tests/fixtures/native/m01-hints-animation.ron'
fixture.write_text('[\n'+''.join('('+','.join(str(x).lower() for x in row)+'),\n' for row in rows)+']\n')
report = {
    'binary_sha256': sha, 'cases': len(rows), 'seed': '0x141c4f400',
    'fixture': str(fixture.relative_to(ROOT)), 'fixture_sha256': hashlib.sha256(fixture.read_bytes()).hexdigest(),
    'ranges': [['0x141c4f58c', '0x141c4f4e6']],
    'native_helpers': ['0x140c95ef0 EasingFunction::apply(selector=1 CubicInOut)'],
    'bridges': [],
    'supplied_boundaries': ['原 ECS 已解析的一行 12B HintAnimation、删除标记、Time<()>.delta_seconds、change tick 存储；不含 ECS archetype 查找/调度'],
    'columns': ['old_value_bits', 'old_time_bits', 'frame_duration_ns', 'delta_bits', 'deleting', 'value_bits', 'eased_bits', 'time_bits'],
    'comparison': '全部三个 f32 逐位比较；NaN payload 不放宽',
}
Path(__file__).with_name('m01-hints-animation-native.json').write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n')
print(json.dumps({'cases': len(rows), 'fixture_sha256': report['fixture_sha256']}))
