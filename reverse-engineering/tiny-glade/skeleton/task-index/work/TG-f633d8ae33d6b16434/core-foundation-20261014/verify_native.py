"""B DirtyTiles cell entry, with a public fmaf bridge (not game logic)."""
import argparse
import ctypes
import hashlib
import json
import random
import struct
from pathlib import Path

import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import (
    UC_X86_REG_RAX, UC_X86_REG_RCX, UC_X86_REG_RSP,
    UC_X86_REG_XMM0, UC_X86_REG_XMM1, UC_X86_REG_XMM2, UC_X86_REG_MXCSR,
)

SHA = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
BASE, DATA, STACK, STOP = 0x140000000, 0x20000000, 0x30000000, 0x40000000


def bits(value):
    return struct.unpack("<I", struct.pack("<f", value))[0]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("binary", type=Path)
    ap.add_argument("output", type=Path)
    args = ap.parse_args()
    raw = args.binary.read_bytes()
    assert hashlib.sha256(raw).hexdigest() == SHA
    pe = pefile.PE(data=raw)
    uc = Uc(UC_ARCH_X86, UC_MODE_64)
    for section in pe.sections:
        size = (max(section.Misc_VirtualSize, section.SizeOfRawData) + 4095) & ~4095
        if size:
            uc.mem_map(BASE + section.VirtualAddress, size)
            uc.mem_write(BASE + section.VirtualAddress, section.get_data())
    uc.mem_map(DATA, 4096)
    uc.mem_map(STACK, 0x10000)
    uc.mem_map(STOP, 4096)
    uc.reg_write(UC_X86_REG_MXCSR, 0x1F80)
    fmaf = ctypes.CDLL(None).fmaf
    fmaf.argtypes = [ctypes.c_float, ctypes.c_float, ctypes.c_float]
    fmaf.restype = ctypes.c_float
    uc.mem_write(0x142923EF0, b"\xc3")

    def bridge(machine, address, size, context):
        if address != 0x142923EF0:
            return
        values = [struct.unpack("<f", struct.pack("<I", machine.reg_read(reg) & 0xFFFFFFFF))[0]
                  for reg in [UC_X86_REG_XMM0, UC_X86_REG_XMM1, UC_X86_REG_XMM2]]
        machine.reg_write(UC_X86_REG_XMM0, bits(fmaf(*values)))

    uc.hook_add(UC_HOOK_CODE, bridge)
    values = [bits(v) for v in [-1000, -68, -67.03125, -66, -65, -1, 0, 1, 64, 65, 66, 1000]]
    values += [0x80000000, 0x7FC00001, 0xFFC00001, 0x7F800000, 0xFF800000,
               0x7F7FFFFF, 0xFF7FFFFF]
    for value in [-67.03125, -65, 65]:
        values += [bits(value) - 1, bits(value) + 1]
    cases = [(x, y) for x in values for y in values]
    rng = random.Random(20261014)
    cases += [(rng.getrandbits(32), rng.getrandbits(32)) for _ in range(256)]
    result = {"binary_sha256": SHA, "entry": "0x140960b20", "exit": "ret@0x140960c15",
              "bridge": "0x142923ef0 UCRT fmaf -> host C fmaf; excludes Windows CRT implementation",
              "cases": []}
    for x, y in cases:
        sp = STACK + 0x8008
        uc.mem_write(sp, struct.pack("<Q", STOP))
        uc.mem_write(DATA, b"\xa5" * 16)
        uc.reg_write(UC_X86_REG_RSP, sp)
        uc.reg_write(UC_X86_REG_RCX, DATA)
        uc.reg_write(UC_X86_REG_XMM1, x)
        uc.reg_write(UC_X86_REG_XMM2, y)
        uc.emu_start(0x140960B20, STOP, count=1000)
        assert uc.reg_read(UC_X86_REG_RSP) == sp + 8
        assert uc.reg_read(UC_X86_REG_RAX) == DATA
        out = bytes(uc.mem_read(DATA, 16))
        tag, column, row = struct.unpack("<III", out[:12])
        assert tag in (0, 1)
        assert out[12:] == b"\xa5" * 4
        if tag == 0:
            assert out[4:] == b"\xa5" * 12
        result["cases"].append({"input_bits": [x, y], "cell": [column, row] if tag else None})
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({"cases": len(cases), "output": str(args.output)}))


if __name__ == "__main__":
    main()
