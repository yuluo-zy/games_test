"""Scan .text of tiny-glade.exe for RIP-relative instructions referencing given VAs.

Usage: python -I rip_scan.py <va_target> [<va_target> ...]
  - finds every instruction whose disp32 (T - end_of_insn) matches a target
  - annotates containing PDB proc (from data/procs.tsv)
  - prints instruction class: LEA / MOV-load / MOV-store / CMP / TEST / movss/movsd / other
"""
import bisect, os, struct, sys
import numpy as np

BASE = 0x140000000
EXE = r"D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe"
TSV = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv"

data = open(EXE, "rb").read()
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

text = next(s for s in sections if s[0] == ".text")
tname, tva, tvsize, trawptr, trawsize = text
text_bytes = data[trawptr:trawptr + trawsize]
T = np.frombuffer(text_bytes, dtype=np.uint8)
n = len(T)

# load procs
P = []
for l in open(TSV, encoding="utf8"):
    p = l.rstrip("\n").split("\t")
    if len(p) >= 5:
        P.append((int(p[0], 16), int(p[1]), p[4]))
P.sort()
starts = [x[0] for x in P]


def owner(va):
    i = bisect.bisect_right(starts, va) - 1
    if i >= 0 and va < starts[i] + P[i][1]:
        return P[i]
    return None


def va2off(va):
    rva = va - BASE
    for name, va0, vsize, rawptr, rawsize in sections:
        if va0 <= rva < va0 + max(vsize, rawsize):
            return rawptr + (rva - va0)
    return None


# class of instruction given opcode bytes and modrm
def classify(bs):
    # bs = bytes of the instruction (raw). We only care about direction of mem access.
    if len(bs) < 4:
        return "?"
    op = bs[-5] if len(bs) >= 5 else bs[0]  # byte before modrm
    modrm = bs[-4]
    if op == 0x8D:
        return "LEA"
    if op in (0x89, 0x88, 0xC7, 0xC6, 0x0B, 0x09, 0x11, 0x13):
        return "STORE"
    if op in (0x8B, 0x8A, 0x0F, 0x10, 0x12):
        return "LOAD"
    if op in (0x3B, 0x39, 0x38, 0x85, 0x84):
        return "CMP/TEST"
    if op in (0x0F, 0xF3, 0xF2, 0x66):
        return "SIMD?" + ("L" if modrm >> 6 == 1 else "S") if False else "SIMD"
    return f"op{op:02x}"


def scan(target, out=sys.stdout):
    """target: VA. Return list of (insn_va, disp_off, kind) writes stores first."""
    # disp32 position P; end = P+4; disp = target - end (must fit int32 and be negative => target > end)
    d = target - (BASE + tva)
    hits = []
    # vectorized: for each P, expected disp32 = target - (BASE+tva + P + 4)
    # We scan positions where byte before P is a modrm with r/m==101 (mod==00)
    arr = T[:n - 8]
    # candidate P: arr[P-1] & 0xC7 == 0x05 ; also handle 4-byte disp where previous byte is modrm
    modrm_ok = (arr[:-1] & 0xC7) == 0x05  # arr[P-1] for P in 1..n
    pidx = np.nonzero(modrm_ok)[0] + 1
    pidx = pidx[pidx < n - 5]  # need 4 disp bytes + instruction start beyond
    # expected disp32 for each p
    exp = (target - (BASE + tva + pidx + 4)) & 0xFFFFFFFF
    got = np.frombuffer(text_bytes, dtype=np.uint8, count=4 * len(pidx)) if False else None
    # gather 4 bytes at pidx (vectorized via stride tricks)
    strides = np.lib.stride_tricks.as_strided
    w = strides(T, shape=(n - 3, 4), strides=(1, 1))
    got = w[pidx]
    val = got[:, 0].astype(np.uint32) | (got[:, 1].astype(np.uint32) << 8) | (got[:, 2].astype(np.uint32) << 16) | (got[:, 3].astype(np.uint32) << 24)
    m = np.nonzero(val == exp)[0]
    for i in m:
        p = int(pidx[i])
        insn_va = BASE + tva + p - 4
        if p < 4:
            continue
        # find instruction start: look back up to 15 bytes for plausible opcode start
        # simple heuristic: bytes before modrm; we print raw context instead
        raw = text_bytes[max(p - 6, 0):p + 4]
        kind = "?"
        if p >= 5:
            op = text_bytes[p - 5]
            if op == 0x8D:
                kind = "LEA"
            elif op in (0x89, 0x88, 0xC7, 0xC6, 0x09, 0x0B, 0x11, 0x13, 0x01, 0x03):
                kind = "STORE"
            elif op in (0x8B, 0x8A, 0x10, 0x12, 0x0F):
                kind = "LOAD"
            elif op in (0x3B, 0x39, 0x38, 0x85, 0x84, 0x33):
                kind = "CMP/TEST"
            elif op in (0xF3, 0xF2, 0x66):
                kind = "SIMD"
            elif op == 0x48 or op == 0x40 or op == 0x41:
                # REX prefix - look one more byte
                op2 = text_bytes[p - 6] if p >= 6 else 0
                if op2 == 0x8D:
                    kind = "LEA"
                elif op2 in (0x89, 0x88, 0xC7, 0xC6, 0x09, 0x0B, 0x11, 0x13, 0x01, 0x03):
                    kind = "STORE"
                elif op2 in (0x8B, 0x8A, 0x10, 0x12, 0x0F):
                    kind = "LOAD"
                elif op2 in (0x3B, 0x39, 0x38, 0x85, 0x84, 0x33):
                    kind = "CMP/TEST"
                else:
                    kind = f"rex-op{op2:02x}"
            else:
                kind = f"op{op:02x}"
        o = owner(insn_va)
        nm = o[2][:150] if o else "?"
        hits.append((kind, insn_va, hex(target), nm, raw.hex()))
    return hits


for t in sys.argv[1:]:
    target = int(t, 16)
    hits = scan(target)
    print(f"## target {target:#x}")
    if not hits:
        print("  (no RIP-relative references found)")
    for kind, va, tg, nm, raw in sorted(hits, key=lambda h: h[1]):
        mark = "WRITE?" if kind == "STORE" else " "
        print(f"  {mark} {kind:8s} {va:#x}  raw={raw}  in {nm}")
