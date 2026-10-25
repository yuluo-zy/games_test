"""For each RIP-relative load site of a pointer slot, disassemble a window and
report whether a store through the loaded register follows (candidate writers).

Usage: python -P check_pointer_writers.py <slot_va> [<slot_va> ...]
"""
import bisect, re, struct, subprocess, sys

BASE = 0x140000000
EXE = r"D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe"
TSV = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv"
OBJDUMP = r"C:/Program Files/LLVM/bin/llvm-objdump"

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
        return P[i][2]
    return None


def disp32_scan(target):
    """Byte-level scan for disp32 windows referencing target; return list of (insn_start_hint, disp_pos)."""
    data = open(EXE, "rb").read()
    e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
    nsec = struct.unpack_from("<H", data, e_lfanew + 6)[0]
    opt = struct.unpack_from("<H", data, e_lfanew + 20)[0]
    secs = [struct.unpack_from("<IIII", data, e_lfanew + 24 + opt + 40 * i + 8) for i in range(nsec)]
    text = next(s for s in secs if True)
    for vs, v, rs, rp in secs:
        pass
    # find .text
    tname = tva = tvsize = trawptr = trawsize = None
    for i in range(nsec):
        nm = data[e_lfanew + 24 + opt + 40 * i:e_lfanew + 24 + opt + 40 * i + 8].rstrip(b"\0")
        vs, v, rs, rp = struct.unpack_from("<IIII", data, e_lfanew + 24 + opt + 40 * i + 8)
        if nm == b".text":
            tname, tva, tvsize, trawptr, trawsize = nm, v, vs, rp, rs
    tb = data[trawptr:trawptr + trawsize]
    hits = []
    n = len(tb)
    for p in range(1, n - 4):
        if (tb[p - 1] & 0xC7) == 0x05:
            d = struct.unpack_from("<i", tb, p)[0]
            end = BASE + tva + p + 4
            if end + d == target:
                hits.append(p)
    return hits, (tva, tvsize)


def run(slot, target):
    hits, (tva, tvsize) = disp32_scan(slot)
    out = []
    for p in hits:
        site = BASE + tva + p - 4  # instruction end = disp end; start unknown; disassemble backwards window
        # disassemble a window starting a bit before
        win = subprocess.run(
            [OBJDUMP, "-d", "--no-show-raw-insn", "-M", "intel",
             f"--start-address={site - 0x10:#x}", f"--stop-address={site + 0x30:#x}", EXE],
            capture_output=True, text=True).stdout
        insns = []
        for line in win.splitlines():
            m = re.match(r"\s*([0-9a-f]+):\s+(.*)", line)
            if m:
                insns.append((int(m.group(1), 16), m.group(2)))
        if not insns:
            continue
        # find index of the rip load: instruction at or after site-16 containing our slot comment
        idx = None
        for i, (a, ins) in enumerate(insns):
            if f"# {slot:#x}" in ins:
                idx = i
                break
        if idx is None:
            continue
        # register loaded
        m = re.search(r"mov\s+(r[a-z0-9]+), qword ptr \[rip", insns[idx][1])
        reg = m.group(1) if m else "?"
        writes = []
        for a, ins in insns[idx + 1:idx + 7]:
            if re.search(rf"\[{reg}\]", ins) and ("mov" in ins or "set" in ins or "xchg" in ins or "inc" in ins or "dec" in ins or "add" in ins or "or" in ins or "and" in ins or "cmp" in ins or "test" in ins):
                writes.append(ins)
            if re.search(r"\[{reg}", ins) and ("mov" in ins or "set" in ins):
                writes.append(ins)
        o = owner(site)
        out.append((site, reg, writes, o[:120] if o else "?"))
    return out


for slot_hex in sys.argv[1:]:
    slot = int(slot_hex, 16)
    print(f"## slot {slot:#x}")
    for site, reg, writes, nm in run(slot, None):
        w = "; ".join(writes) if writes else "(no store through reg)"
        print(f"  {site:#x} reg={reg} {w}  in {nm}")
