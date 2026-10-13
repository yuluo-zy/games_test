"""Original B camera entry-point fixtures; glam stays original native code."""
from pathlib import Path
import ctypes, hashlib, json, math, random, struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

OUT = Path(__file__).parent
ROOT = OUT.parents[2]
BINARY = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw = BINARY.read_bytes()
assert hashlib.sha256(raw).hexdigest() == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
u = Uc(UC_ARCH_X86, UC_MODE_64)
u.mem_map(base, (pe.OPTIONAL_HEADER.SizeOfImage + 4095) & ~4095)
u.mem_write(base, pe.get_memory_mapped_image())
STACK, INPUT, OUTPUT, STOP = 0x200000000, 0x210000000, 0x210001000, 0x220000000
u.mem_map(STACK, 0x20000); u.mem_map(INPUT, 0x20000); u.mem_map(STOP, 0x1000)
crt = ctypes.CDLL('ucrtbase')
for name in ['cosf', 'sinf']:
    fn = getattr(crt, name); fn.argtypes = [ctypes.c_float]; fn.restype = ctypes.c_float
def bits(v): return struct.unpack('<I', struct.pack('<f', v))[0]
def value(v): return struct.unpack('<f', struct.pack('<I', v & 0xffffffff))[0]
def hook(uc, address, size, user):
    name = {0x142923e80: 'cosf', 0x142923fd0: 'sinf'}.get(address)
    if name:
        result = getattr(crt, name)(value(u.reg_read(UC_X86_REG_XMM0)))
        u.reg_write(UC_X86_REG_XMM0, bits(result))
        sp = u.reg_read(UC_X86_REG_RSP)
        ret = struct.unpack('<Q', u.mem_read(sp, 8))[0]
        u.reg_write(UC_X86_REG_RSP, sp + 8); u.reg_write(UC_X86_REG_RIP, ret)
u.hook_add(UC_HOOK_CODE, hook)
def run(address, values, count):
    sp = STACK + 0x10008
    u.reg_write(UC_X86_REG_RSP, sp); u.mem_write(sp, struct.pack('<Q', STOP))
    u.mem_write(INPUT, struct.pack('<' + 'f' * len(values), *values))
    u.mem_write(OUTPUT, bytes(256))
    u.reg_write(UC_X86_REG_RCX, OUTPUT); u.reg_write(UC_X86_REG_RDX, INPUT)
    u.emu_start(address, STOP, count=100000)
    assert u.reg_read(UC_X86_REG_RIP) == STOP
    return list(struct.unpack('<' + 'I' * count, u.mem_read(OUTPUT, count * 4)))

rng = random.Random(1301322)
fixtures = {'xz_rotation': [], 'projection': []}
matrices = []
# Three quaternion-to-matrix chart branches, all quadrants, near-vertical
# projected-right fallback, threshold neighbors, zero and non-unit bases.
for yaw in [0, .2, math.pi/2, math.pi, -math.pi/2, -2.8] + [rng.uniform(-math.pi, math.pi) for _ in range(64)]:
    for roll in [0, math.pi/2]:
        c, s, cr, sr = math.cos(yaw), math.sin(yaw), math.cos(roll), math.sin(roll)
        matrices.append([c*cr, sr, -s*cr, 0, -c*sr, cr, s*sr, 0, s, 0, c, 0, 2, 3, -4, 1])
for x in [0, 0.00000999, 0.00001, 0.00001001, -0.00001001]:
    matrices.append([x, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1])
matrices.append([0]*15 + [1])
for m in matrices:
    inputs = [0]*48 + m  # inverse_view at +0xc0 in original CameraMatrices.
    fixtures['xz_rotation'].append({'inverse_view': [bits(v) for v in m], 'quat': run(0x140c96e70, inputs, 4)})
for _ in range(80):
    inputs = [rng.uniform(.2, 4), rng.uniform(.001, 1), rng.uniform(5, 170)]
    fixtures['projection'].append({'params': [bits(v) for v in inputs], 'matrices': run(0x140c97390, inputs, 32)})
report = {'binary': str(BINARY), 'sha256': hashlib.sha256(raw).hexdigest(), 'method': 'Unicorn executes original entry points; CRT cosf/sinf only hooks; glam original instructions execute unchanged', 'functions': {'xz_rotation': '0x140c96e70', 'projection': '0x140c97390'}, 'fixtures': fixtures}
(OUT/'camera-native-fixtures.json').write_text(json.dumps(report, indent=2)+'\n', encoding='utf8')
source = ROOT/'reconstruction/shared-foundation/crates/utils/src/camera.rs'
text = source.read_text(encoding='utf8').split('\n#[cfg(test)]\nmod native_oracle')[0]
test = '''
#[cfg(test)]
mod native_oracle {
    use super::*;
    fn compare(actual: &[f32], expected: &[u32]) {
        for (a, bits) in actual.iter().zip(expected) {
            let e = f32::from_bits(*bits);
            assert!((a-e).abs() <= 0.000002 * e.abs().max(1.0) || (a.is_nan() && e.is_nan()), "actual={a:?}, original={e:?}");
        }
    }
    #[test] fn original_xz_rotation() {
        let fixtures: &[([u32;16],[u32;4])] = &[
'''
for row in fixtures['xz_rotation']:
    test += f"            ({row['inverse_view']!r}, {row['quat']!r}),\n"
test += '''        ];
        for (matrix, expected) in fixtures {
            let inverse_view = Mat4::from_cols_array(&matrix.map(f32::from_bits));
            let camera = CameraMatrices { projection:Mat4::IDENTITY, inverse_view };
            compare(&camera.xz_projective_rotation().to_array(), expected);
        }
    }
    #[test] fn original_projection() {
        let fixtures: &[([u32;3],[u32;32])] = &[
'''
for row in fixtures['projection']:
    test += f"            ({row['params']!r}, {row['matrices']!r}),\n"
test += '''        ];
        for (params, expected) in fixtures {
            let [aspect, near, vertical_fov_degrees] = params.map(f32::from_bits);
            let result = CameraProjectionParams { aspect, near, vertical_fov_degrees }.calc_matrices();
            compare(&result.projection.to_cols_array(), &expected[..16]);
            compare(&result.inverse_projection.to_cols_array(), &expected[16..]);
        }
    }
}
'''
source.write_text(text + test, encoding='utf8')
print({name:len(rows) for name, rows in fixtures.items()})
