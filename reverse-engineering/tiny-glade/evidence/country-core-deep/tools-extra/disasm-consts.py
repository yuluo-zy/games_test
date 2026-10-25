"""Disassemble and resolve all rip-relative operands for a VA range."""
import sys, struct
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
BIN = r"D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe"
BASE = 0x140000000
data = open(BIN, "rb").read()
e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
nsec = struct.unpack_from("<H", data, e_lfanew+6)[0]
opt = struct.unpack_from("<H", data, e_lfanew+20)[0]
sec_off = e_lfanew+24+opt
sections = []
for i in range(nsec):
    o = sec_off + i*40
    name = data[o:o+8].rstrip(b"\0").decode("ascii","replace")
    vsize, va, rawsize, rawptr = struct.unpack_from("<IIII", data, o+8)
    sections.append((name, va, vsize, rawptr, rawsize))
def va2off(va):
    rva = va - BASE
    for name, va0, vsize, rawptr, rawsize in sections:
        if va0 <= rva < va0 + vsize:
            return rawptr + (rva - va0)
    raise ValueError(f"VA {va:#x} not mapped")
va = int(sys.argv[1], 16)
n = int(sys.argv[2], 16)
off = va2off(va)
code = data[off:off+n]
md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True
for i in md.disasm(code, va):
    rip_rel = None
    for op in i.operands:
        if op.type == 3 and op.imm > 0x140000000:  # MEM
            rip_rel = op.mem.disp
    line = f"{i.address:#x}  {i.mnemonic}\t{i.op_str}"
    if rip_rel is not None:
        tgt = i.address + i.size + rip_rel
        b = data[va2off(tgt):va2off(tgt)+16]
        s = b.split(b'\0')[0].decode('utf-8','replace')
        line += f"   <<< const@{tgt:#x} = {b.hex()}  {s!r}"
    print(line)
