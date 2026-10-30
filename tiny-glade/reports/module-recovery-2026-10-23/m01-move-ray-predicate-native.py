"""执行原153B射线过滤器；只隔离既有ColliderSet查找边界。"""
from pathlib import Path
import hashlib
import json
import struct

import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
raw = (ROOT / 'B版本内容/tiny-glade.rne').read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
cpu.mem_map(base, (pe.OPTIONAL_HEADER.SizeOfImage + 4095) & ~4095)
cpu.mem_write(base, pe.get_memory_mapped_image())
D, S, STOP = 0x20000000, 0x30000000, 0x50000000
cpu.mem_map(D, 0x10000)
cpu.mem_map(S, 0x10000)
cpu.mem_map(STOP, 0x1000)


def q(address, value):
    cpu.mem_write(address, struct.pack('<Q', value))


def d(address, value):
    cpu.mem_write(address, struct.pack('<I', value))


def hook(cpu, pc, size, _):
    if pc == STOP:
        cpu.emu_stop()
    elif pc == 0x140b93940:
        # 原ColliderSet::get_mut_internal依赖已由实际Rapier CPU测试覆盖。
        # 这里只返回真实native布局的userdata入口，不替代过滤/receipt校验算法。
        cpu.reg_write(UC_X86_REG_RAX, D + 0x400)
        sp = cpu.reg_read(UC_X86_REG_RSP)
        target = struct.unpack('<Q', cpu.mem_read(sp, 8))[0]
        cpu.reg_write(UC_X86_REG_RSP, sp + 8)
        cpu.reg_write(UC_X86_REG_RIP, target)


cpu.hook_add(UC_HOOK_CODE, hook)
wall = 0x123456789abcdef0
rows = []
for kind in range(17):
    for excluded in (None, wall, wall + 1):
        cpu.mem_write(D, bytes(0x1000))
        q(D + 8, D + 0x100)
        q(D + 0x10, 1)
        d(D + 0x100, kind)
        q(D + 0x108, wall)
        d(D + 0x120, 3)
        q(D + 0x200, D)
        q(D + 0x208, D + 0x500)
        q(D + 0x210, D + 0x300)
        q(D + 0x300, int(excluded is not None))
        q(D + 0x308, excluded or 0)
        q(D + 0x400, 3 << 32)
        sp = S + 0x8000 - 8
        q(sp, STOP)
        cpu.reg_write(UC_X86_REG_RSP, sp)
        cpu.reg_write(UC_X86_REG_RCX, D + 0x200)
        cpu.reg_write(UC_X86_REG_RDX, 0)
        cpu.reg_write(UC_X86_REG_R8, 3)
        cpu.emu_start(0x1408ca9a0, STOP, count=500)
        assert cpu.reg_read(UC_X86_REG_RIP) == STOP
        actual = bool(cpu.reg_read(UC_X86_REG_RAX) & 0xff)
        expected = not (kind in (0, 1, 2, 8) and excluded == wall)
        assert actual == expected
        rows.append({'kind': kind, 'excluded': excluded, 'accepted': actual})
(OUT / 'm01-move-ray-predicate-native.json').write_text(json.dumps({
    'binary_sha256': sha,
    'function': '0x1408ca9a0',
    'pdb_bytes': 153,
    'isolated_boundary': 'ColliderSet::get_mut_internal returns supplied collider userdata; original registry generation and every predicate branch execute unchanged',
    'cases': rows,
}, indent=2) + '\n')
print(json.dumps({'original_predicate_cases': len(rows)}))
