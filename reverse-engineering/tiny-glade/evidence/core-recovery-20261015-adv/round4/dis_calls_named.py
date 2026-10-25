#!/usr/bin/env python3
"""Disassemble a VA range from B (RNE) binary; print direct CALL targets with procs.tsv names."""
import sys, struct
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
    vsize, va, rawsize, rawptr = struct.unpack_from("<IIII", data, o + 8)
    sections.append((va, vsize, rawptr, rawsize))

def va2off(va):
    rva = va - BASE
    for va0, vsize, rawptr, rawsize in sections:
        if va0 <= rva < va0 + vsize:
            return rawptr + (rva - va0)
    raise ValueError("not mapped")

name = {}
for line in open(r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv",
                 encoding="utf-8", errors="replace"):
    parts = line.rstrip("\n").split("\t")
    if len(parts) >= 4:
        try:
            a = int(parts[0], 16)
        except ValueError:
            continue
        name[a] = parts[3]

va = int(sys.argv[1], 16)
n = int(sys.argv[2], 16)
off = va2off(va)
md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True
allins = list(md.disasm(data[off:off + n], va))
print("total insns", len(allins))
seen = set()
for i in allins:
    if i.mnemonic == "call":
        op = i.operands[0]
        tgt = None
        if op.type == 3:
            tgt = i.address + i.size + op.imm
        if tgt is None:
            continue
        if tgt in seen:
            continue
        seen.add(tgt)
        nm = name.get(tgt, "")
        short = nm.split("::")[-1][:80] if nm else ""
        print("%#x <- %#x  %s" % (tgt, i.address, short))
