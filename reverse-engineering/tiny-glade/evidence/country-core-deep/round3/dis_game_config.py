# -*- coding: utf-8 -*-
"""Disassemble GameConfig::upgrade 0x14090df90..0x14090e000 and resolve rip-relative targets."""
import capstone

BIN = r"D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe"
BASE = 0x140000000
data = open(BIN, "rb").read()

va0 = 0x14090df90
n = 0x110
off = va0 - BASE
blob = data[off:off + n]
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
md.detail = True

import struct

for insn in md.disasm(blob, va0):
    mnem = insn.mnemonic + " " + insn.op_str
    rip = insn.address + insn.size
    if "rip" in insn.op_str:
        # extract displacement
        disp = insn.operands[-1].mem.disp if insn.operands else 0
        target = rip + disp
        mnem += f"   ; -> {target:#x}"
        if 0x140000000 <= target < 0x144000000:
            try:
                val = struct.unpack("<f", data[(target - BASE):(target - BASE) + 4])[0]
                mnem += f"  (f32={val!r})"
            except Exception:
                pass
    print(f"{insn.address:#08x}: {mnem}")
