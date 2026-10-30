"""执行原RMB heading的全部SSE输入计算，止于CRT atan2f边界，不桥接运算。"""
from pathlib import Path
import struct, hashlib, json, random
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import *

root = Path(__file__).resolve().parents[3]
raw = (root / 'B版本内容/tiny-glade.rne').read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
image = pe.get_memory_mapped_image()
u = Uc(UC_ARCH_X86, UC_MODE_64)
u.mem_map(base, (len(image)+4095)&~4095)
u.mem_write(base, image)
D = 0x20000000
u.mem_map(D, 0x10000)
rng = random.Random(0x99ca60)
quats = [[0, 0, 0, 0x3f800000], [0x80000000, 0, 0x80000000, 0x3f800000],
    [0, 0x3f3504f3, 0, 0x3f3504f3]]
for _ in range(512):
    quats.append(list(struct.unpack('<4I', struct.pack('<4f',
        *(rng.uniform(-10, 10) for _ in range(4))))))
rows = []
for q in quats:
    u.reg_write(UC_X86_REG_XMM0, int.from_bytes(struct.pack('<4I', *q), 'little'))
    u.emu_start(0x140976fc9, 0x14097705b, count=64)
    assert u.reg_read(UC_X86_REG_RIP) == 0x14097705b
    rows.append([q, [u.reg_read(UC_X86_REG_XMM0)&0xffffffff,
                     u.reg_read(UC_X86_REG_XMM1)&0xffffffff]])
target = Path(__file__).with_name('m01-ui-convert-heading-native-cases.rs')
target.write_text('&[\n' + ''.join('(' + str(q) + ',' + str(v) + '),\n'
    for q, v in rows) + ']\n')
normals, rotations = [], []
for i, q in enumerate(quats):
    u.mem_write(D+0x2000, struct.pack('<4I', *q))
    u.mem_write(D+0x8008, struct.pack('<Q', D+0xff00))
    u.reg_write(UC_X86_REG_RSP, D+0x8008)
    u.reg_write(UC_X86_REG_RCX, D+0x1000)
    u.reg_write(UC_X86_REG_RDX, D+0x2000)
    u.emu_start(0x14099c420, D+0xff00, count=256)
    assert u.reg_read(UC_X86_REG_RIP) == D+0xff00
    normals.append([q, list(struct.unpack('<3I', u.mem_read(D+0x1000, 12)))])
    camera = quats[(i*73+1) % len(quats)]
    u.reg_write(UC_X86_REG_RBP, D+0x2000)
    u.mem_write(D+0x21a0, struct.pack('<4I', *camera))
    u.reg_write(UC_X86_REG_XMM6, int.from_bytes(struct.pack('<4I', *q), 'little'))
    u.emu_start(0x14099e61d, 0x14099e67d, count=64)
    assert u.reg_read(UC_X86_REG_RIP) == 0x14099e67d
    rotation = list(struct.unpack('<4I', u.reg_read(UC_X86_REG_XMM0).to_bytes(16, 'little')))
    rotations.append([q, camera, rotation])
pose_target = target.with_name('m01-ui-convert-pose-native-cases.rs')
pose_target.write_text('(&[\n'+''.join('('+str(q)+','+str(v)+'),\n' for q,v in normals)
    +'],&[\n'+''.join('('+str(q)+','+str(c)+','+str(v)+'),\n' for q,c,v in rotations)+'])\n')
meta = dict(binary_sha256=sha, range=['0x140976fc9', '0x14097705b'],
    cases=len(rows), arithmetic_bridges=[], scope='atan2f argument pair; CRT body excluded',
    fixture_sha256=hashlib.sha256(target.read_bytes()).hexdigest(),
    pose_ranges=['full 0x14099c420', '0x14099e61d..0x14099e67d'],
    normal_cases=len(normals), rotation_cases=len(rotations),
    pose_fixture_sha256=hashlib.sha256(pose_target.read_bytes()).hexdigest())
Path(__file__).with_suffix('.json').write_text(json.dumps(meta, indent=2)+'\n')
print(json.dumps(meta))
