"""Disassemble bytes from the B (RNE) binary by VA using capstone (x64).
Usage:
  python scripts/rne-disasm.py <va> [<nbytes>]
"""
import sys, struct
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

BIN = r"D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe"
data = open(BIN, "rb").read()
BASE = 0x140000000

e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
nsec = struct.unpack_from("<H", data, e_lfanew + 6)[0]
opt_size = struct.unpack_from("<H", data, e_lfanew + 20)[0]
sec_off = e_lfanew + 24 + opt_size
sections = []
for i in range(nsec):
    o = sec_off + i * 40
    name = data[o:o+8].rstrip(b"\0").decode("ascii", "replace")
    vsize, va, rawsize, rawptr = struct.unpack_from("<IIII", data, o + 8)
    sections.append((name, va, vsize, rawptr, rawsize))

def va2off(va):
    rva = va - BASE
    for name, va0, vsize, rawptr, rawsize in sections:
        if va0 <= rva < va0 + vsize:
            return rawptr + (rva - va0)
    raise ValueError(f"VA {va:#x} not mapped")

va = int(sys.argv[1], 16)
n = int(sys.argv[2], 16) if len(sys.argv) > 2 else 256
off = va2off(va)
code = data[off:off + n]
md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = False
for ins in md.disasm(code, va):
    print(f"0x{ins.address:x}  {ins.mnemonic:<8s} {ins.op_str}")
