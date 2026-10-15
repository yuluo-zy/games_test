#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Simulate the Affine3Packed pack instruction sequences from rne-read bytes."""
import sys, struct, subprocess, os

sys.path.insert(0, os.environ.get("W8_CAP", r"C:\Users\liyu\AppData\Local\Temp\w8_capstone"))
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

def rd(va, n):
    out = subprocess.run([sys.executable, "-I",
                          "D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py",
                          "--hex", hex(va), str(n)],
                         capture_output=True, text=True,
                         env={**os.environ, "PYTHONIOENCODING": "utf-8"}).stdout.strip()
    return bytes.fromhex(out.split()[-1])

class Sim:
    def __init__(self, mem, regs=None):
        self.xmm = {i: [0.0]*4 for i in range(16)}
        self.regs = {"rax": 0, "rcx": 0, "rdx": 0, "r8": 0, "r9": 0, "rsp": 0x7000}
        if regs:
            self.regs.update(regs)
        self.mem = mem  # addr -> 4 bytes float

    def m32(self, a):
        b = self.mem.get(a)
        return struct.unpack("<f", b)[0] if b else 0.0

    def addr(self, expr):
        # expr like 'qword ptr [rsp + 0x28]' or 'xmmword ptr [rdx + 0x10]' or '[r8]'
        inner = expr.split("[")[1].rstrip("]").strip()
        parts = inner.split("+")
        base = self.regs.get(parts[0].strip(), 0)
        off = 0
        if len(parts) > 1:
            off = int(parts[1].strip(), 0)
        return base + off

    def run(self, code, base):
        md = Cs(CS_ARCH_X86, CS_MODE_64)
        for i in md.disasm(code, base):
            m, o = i.mnemonic, i.op_str
            if m == "mov":
                dst, src = [p.strip() for p in o.split(",")]
                if src == "rcx":
                    self.regs[dst] = self.regs[src]
                elif src.startswith("qword ptr"):
                    self.regs[dst] = struct.unpack("<Q", self.mem[self.addr(src)])[0]
            elif m == "movss":
                dst, src = [p.strip() for p in o.split(",")]
                if src.startswith("dword ptr"):
                    v = self.m32(self.addr(src))
                else:
                    v = self.xmm[int(src[3:])][0]
                self.xmm[int(dst[3:])] = [v, 0.0, 0.0, 0.0]
            elif m == "movsd":
                dst, src = [p.strip() for p in o.split(",")]
                if src.startswith("qword ptr"):
                    a = self.addr(src)
                    lo, hi = self.m32(a), self.m32(a + 4)
                else:
                    lo, hi = self.xmm[int(src[3:])][0], self.xmm[int(src[3:])][1]
                d = int(dst[3:])
                self.xmm[d] = [lo, hi, 0.0, 0.0]
            elif m == "movlhps":
                dst, src = [p.strip() for p in o.split(",")]
                d, s = int(dst[3:]), int(src[3:])
                self.xmm[d] = [self.xmm[d][0], self.xmm[d][1], self.xmm[s][0], self.xmm[s][1]]
            elif m == "movhps":
                dst, src = [p.strip() for p in o.split(",")]
                if src.startswith("qword ptr"):
                    a = self.addr(src)
                    lo, hi = self.m32(a), self.m32(a + 4)
                else:
                    lo, hi = self.xmm[int(src[3:])][0], self.xmm[int(src[3:])][1]
                self.xmm[int(dst[3:])] = [self.xmm[int(dst[3:])][0], self.xmm[int(dst[3:])][1], lo, hi]
            elif m == "shufps":
                # REAL semantics (empirically verified): result dword k for k<2
                # comes from operand1 (dst) dword imm[2k+1:2k]; k>=2 from
                # operand2 (src) dword imm[2k+1:2k]; index 0..3 within operand.
                dst, src, imm = [p.strip() for p in o.split(",")]
                d, s = int(dst[3:]), int(src[3:])
                imm = int(imm, 16)
                out = []
                for k in range(4):
                    sel = (imm >> (2*k)) & 3
                    out.append(self.xmm[d if k < 2 else s][sel])
                self.xmm[d] = out
            elif m == "unpcklps":
                dst, src = [p.strip() for p in o.split(",")]
                d, s = int(dst[3:]), int(src[3:])
                self.xmm[d] = [self.xmm[d][0], self.xmm[s][0], self.xmm[d][1], self.xmm[s][1]]
            elif m == "unpcklpd":
                dst, src = [p.strip() for p in o.split(",")]
                d, s = int(dst[3:]), int(src[3:])
                self.xmm[d] = [self.xmm[d][0], self.xmm[d][1], self.xmm[s][0], self.xmm[s][1]]
            elif m == "movups":
                dst, src = [p.strip() for p in o.split(",")]
                if dst.startswith("xmmword ptr"):
                    a = self.addr(dst)
                    vals = self.xmm[int(src[3:])]
                    for k in range(4):
                        self.mem[a + 4*k] = struct.pack("<f", vals[k])
                else:
                    a = self.addr(src)
                    for k in range(4):
                        self.xmm[int(dst[3:])][k] = self.m32(a + 4*k)
            elif m == "ret":
                break
        return self

def show(addr, mem, n=12):
    return [struct.unpack("<f", mem[addr + 4*k])[0] for k in range(n)]

# ---------- from_cols 0x14089d500 ----------
mem = {}
for base, v in ((0x1000, [1, 2, 3]), (0x2000, [4, 5, 6]), (0x3000, [7, 8, 9]), (0x4000, [10, 11, 12])):
    for k, f in enumerate(v):
        mem[base + 4*k] = struct.pack("<f", f)
s = Sim(mem, regs={"rax": 0x5000, "rcx": 0x5000, "rdx": 0x1000, "r8": 0x2000, "r9": 0x3000})
s.mem[0x7028] = struct.pack("<Q", 0x4000)  # [rsp+0x28] = &w
s.run(rd(0x14089d500, 72), 0x14089d500)
print("from_cols out:", show(0x5000, s.mem))

# ---------- from_mat3_translation 0x14089d4d0 ----------
mem = {}
for k, f in enumerate([1, 2, 3, 4, 5, 6, 7, 8, 9]):
    mem[0x1000 + 4*k] = struct.pack("<f", f)
for k, f in enumerate([10.0, 11.0, 12.0]):
    mem[0x2000 + 4*k] = struct.pack("<f", f)
s = Sim(mem, regs={"rax": 0x5000, "rcx": 0x5000, "rdx": 0x1000, "r8": 0x2000})
s.run(rd(0x14089d4d0, 64), 0x14089d4d0)
print("from_mat3_translation out:", show(0x5000, s.mem))

# ---------- impl$1::from (From<Mat4>) 0x14089d560 ----------
mem = {}
for k, f in enumerate([1, 2, 3, 13, 4, 5, 6, 14, 7, 8, 9, 15, 16, 17, 18, 19]):
    mem[0x1000 + 4*k] = struct.pack("<f", f)
s = Sim(mem, regs={"rax": 0x5000, "rcx": 0x5000, "rdx": 0x1000})
s.run(rd(0x14089d560, 128), 0x14089d560)
print("impl from(Mat4) out:", show(0x5000, s.mem))
