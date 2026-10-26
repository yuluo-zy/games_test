"""执行 B 发行 RNE 的相机焦点→UV 指令；不替代运算或依赖调用。"""
from pathlib import Path
import hashlib
import json
import struct

import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import (
    UC_X86_REG_RAX, UC_X86_REG_RDX, UC_X86_REG_RBP,
    UC_X86_REG_R12, UC_X86_REG_R15, UC_X86_REG_RIP,
    UC_X86_REG_XMM7, UC_X86_REG_MXCSR,
)

ROOT = Path(__file__).resolve().parents[4]
BINARY = ROOT / "B版本内容/tiny-glade.rne"
OUTPUT = ROOT / "tiny-glade/tests/fixtures/native/toolbar-autofocus.json"
EXPECTED_SHA = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
ENTRY, EXIT = 0x140A17AD7, 0x140A17B5C

raw = BINARY.read_bytes()
assert hashlib.sha256(raw).hexdigest() == EXPECTED_SHA
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
image = pe.get_memory_mapped_image()
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
cpu.mem_map(base, (len(image) + 4095) & ~4095)
cpu.mem_write(base, image)
data = 0x20000000
cpu.mem_map(data, 0x3000)

def bits(value):
    return struct.unpack("<I", struct.pack("<f", value))[0]

vectors = [
    [bits(x) for x in [0.2, 0.4, 0.6, 0.8]],
    [bits(x) for x in [-1.25, 0.75, 0.125, -0.5]],
    [0, 0x80000000, 0x80000000, 0],
    [1, 0x80000001, 0x007fffff, 0x807fffff],
    [0x7f7fffff, 0xff7fffff, 0x3f800001, 0xbf800001],
    [0x7f800000, 0xff800000, 0xff800000, 0x7f800000],
]
rows = []
for mode in [0, 1, 3, 4, 99, 0xffffffff]:
    for width, height in [(1600, 800), (800, 1600), (960, 540), (1, 1), (0xffffffff, 1)]:
        for focus in vectors:
            cpu.mem_write(data, bytes(0x3000))
            cpu.mem_write(data, struct.pack("<II", width, height))
            payload = struct.pack("<I", mode) + struct.pack("<II", *focus[:2]) + bytes(4)
            payload += struct.pack("<II", *focus[2:])
            cpu.mem_write(data + 0x100, payload)
            # 入口前资源解析与 lock gate 不属于本段，使用已解析的真实布局输入。
            for register, value in [
                (UC_X86_REG_RAX, mode), (UC_X86_REG_RDX, 0xffffffff),
                (UC_X86_REG_R12, data), (UC_X86_REG_R15, data + 0x100),
                (UC_X86_REG_RBP, data + 0x1000), (UC_X86_REG_MXCSR, 0x1f80),
            ]:
                cpu.reg_write(register, value)
            cpu.emu_start(ENTRY, EXIT, count=100)
            assert cpu.reg_read(UC_X86_REG_RIP) == EXIT
            result = cpu.reg_read(UC_X86_REG_XMM7).to_bytes(16, "little")
            rows.append({"mode": mode, "window": [width, height], "focus_bits": focus,
                         "uv_bits": list(struct.unpack("<II", result[:8]))})

report = {
    "binary_sha256": EXPECTED_SHA,
    "instruction_range": [hex(ENTRY), hex(EXIT)],
    "instruction_sha256": hashlib.sha256(image[ENTRY-base:EXIT-base]).hexdigest(),
    "scope": "原 mode 选择及相机坐标分支完整执行，无运算 hook；不含 mode2 鼠标资源查找、ECS lock gate、GPU shader 或窗口验收。",
    "mxcsr": "0x1f80",
    "cases": rows,
}
OUTPUT.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n")
print(json.dumps({"cases": len(rows), "output": str(OUTPUT)}))
