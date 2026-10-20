"""Execute the original pure geometry leaf with explicit shape-boundary hooks.

The original swizzler and all original scalar/SIMD arithmetic run unmodified.
Only convex/sphere constructors are hooked: we capture their actual arguments,
not execute Parry or claim that an entire original game/physics world ran.
"""
from pathlib import Path
import hashlib
import json
import struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
path = ROOT / "reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe"
raw = path.read_bytes()
assert hashlib.sha256(raw).hexdigest() == "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
image = pe.get_memory_mapped_image()

def run(case, convex_success):
    uc = Uc(UC_ARCH_X86, UC_MODE_64)
    uc.mem_map(base, (len(image) + 0xfff) & ~0xfff)
    uc.mem_write(base, image)
    stack, data, stop = 0x100000, 0x300000, 0x400000
    uc.mem_map(stack, 0x100000)
    uc.mem_map(data, 0x10000)
    uc.mem_map(stop, 0x1000)
    sp = stack + 0x80008
    uc.mem_write(sp, struct.pack("<Q", stop))
    addresses = [data + 0x100 + i * 0x40 for i in range(6)]
    for address, value in zip(addresses, case):
        values = value if isinstance(value, list) else [value]
        uc.mem_write(address, struct.pack("<" + "f" * len(values), *values))
    uc.reg_write(UC_X86_REG_RSP, sp)
    uc.reg_write(UC_X86_REG_RCX, data)
    uc.reg_write(UC_X86_REG_RDX, addresses[0])
    uc.reg_write(UC_X86_REG_R8, addresses[1])
    uc.reg_write(UC_X86_REG_R9, addresses[2])
    uc.mem_write(sp + 0x28, struct.pack("<QQQ", *addresses[3:]))
    captured = {}

    def return_to_caller():
        current_sp = uc.reg_read(UC_X86_REG_RSP)
        target = struct.unpack("<Q", uc.mem_read(current_sp, 8))[0]
        uc.reg_write(UC_X86_REG_RSP, current_sp + 8)
        uc.reg_write(UC_X86_REG_RIP, target)

    def hook(uc, address, size, _):
        if address == stop:
            uc.emu_stop()
        elif address == 0x1408CEC10:
            count = uc.reg_read(UC_X86_REG_R8)
            pointer = uc.reg_read(UC_X86_REG_RDX)
            points = bytes(uc.mem_read(pointer, count * 12))
            captured["convex_points_bits"] = list(struct.unpack("<" + "I" * count * 3, points))
            captured["convex_points"] = [list(p) for p in struct.iter_unpack("<fff", points)]
            uc.mem_write(uc.reg_read(UC_X86_REG_RCX), bytes([1 if convex_success else 0]) + bytes(127))
            return_to_caller()
        elif address == 0x1408D0120:
            radius = uc.reg_read(UC_X86_REG_XMM1) & 0xffffffff
            center = bytes(uc.mem_read(uc.reg_read(UC_X86_REG_R8), 12))
            captured["sphere_radius_bits"] = radius
            captured["sphere_center_bits"] = list(struct.unpack("<III", center))
            captured["sphere_center"] = list(struct.unpack("<fff", center))
            uc.mem_write(uc.reg_read(UC_X86_REG_RCX), bytes([2]) + bytes(127))
            return_to_caller()

    uc.hook_add(UC_HOOK_CODE, hook)
    uc.emu_start(0x1407DEC90, stop + 1, count=2000)
    captured["convex_success_hook"] = convex_success
    assert "convex_points_bits" in captured
    assert ("sphere_center_bits" in captured) == (not convex_success)
    return captured

cases = [
    [[1.25, 2.5, -3.0], [4.0, -1.25, 7.5], [0.6, 0.8], [-0.8, 0.6], 1.2, 0.7],
    [[-2.0, 0.0, 3.0], [-2.0, 0.0, 3.0], [1.0, 0.0], [1.0, 0.0], 0.0, 0.0],
]
result = {"scope": __doc__, "cases": [{"input": c, "success": run(c, True), "fallback": run(c, False)} for c in cases]}
(OUT / "native-structural-geometry-fixtures.json").write_text(json.dumps(result, indent=2), encoding="utf8")
print(json.dumps(result["cases"], indent=2))
