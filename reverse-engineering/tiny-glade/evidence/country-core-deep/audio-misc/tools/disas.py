"""Local x64 disassembler over workspace/input/tiny-glade.exe (base 0x140000000).
Usage: python disas.py <va> <size> [va size ...]
Prints address: bytes  mnemonic operands. Read-only, no Ghidra backend.
"""
import sys
import struct
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

BIN = r"D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe"
BASE = 0x140000000
data = open(BIN, "rb").read()

e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
nsec = struct.unpack_from("<H", data, e_lfanew + 6)[0]
opt_size = struct.unpack_from("<H", data, e_lfanew + 20)[0]
sec_off = e_lfanew + 24 + opt_size
sections = []
for i in range(nsec):
    o = sec_off + i * 40
    name = data[o:o + 8].rstrip(b"\0").decode("ascii", "replace")
    vsize, va, rawsize, rawptr = struct.unpack_from("<IIII", data, o + 8)
    sections.append((name, va, vsize, rawptr, rawsize))


def va2off(va):
    rva = va - BASE
    for name, va0, vsize, rawptr, rawsize in sections:
        if va0 <= rva < va0 + vsize:
            if rva - va0 >= rawsize:
                raise ValueError(f"VA {va:#x} in .bss (no raw data)")
            return rawptr + (rva - va0)
    raise ValueError(f"VA {va:#x} not mapped")


md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True

args = sys.argv[1:]
while args:
    va = int(args.pop(0), 16)
    n = int(args.pop(0), 16)
    off = va2off(va)
    code = data[off:off + n]
    for ins in md.disasm(code, va):
        print(f"{ins.address:012x}  {ins.bytes.hex():<20} {ins.mnemonic:8} {ins.op_str}")
    print("----")
