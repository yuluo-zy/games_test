import json, struct, hashlib, sys

EXE = r"D:/game/ljxsj_92385/Tiny Glade/tiny-glade.exe"
PROCS = r"D:/game/reverse-engineering/tiny-glade/evidence/toolbar-origin-2026-10-16/toolbar-procedures.json"
OUT = r"D:/game/reverse-engineering/tiny-glade/evidence/round11/tbd-jobs/prepare-targets.json"

WANT = [
    "0x142343a10", "0x142344560", "0x1423448a0", "0x1423449f0",
    "0x142340890", "0x142340d20", "0x141eaec50", "0x141e4d1e0",
    "0x141e4ccb0", "0x141e4dc40",
]

data = open(EXE, "rb").read()
e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
assert data[e_lfanew:e_lfanew+4] == b"PE\0\0"
coff = e_lfanew + 4
nsec = struct.unpack_from("<H", data, coff + 2)[0]
opt_size = struct.unpack_from("<H", data, coff + 16)[0]
opt = coff + 20
magic = struct.unpack_from("<H", data, opt)[0]
assert magic == 0x20B, hex(magic)
image_base = struct.unpack_from("<Q", data, opt + 24)[0]
sec = opt + opt_size
sections = []
for i in range(nsec):
    off = sec + i * 40
    name = data[off:off+8].rstrip(b"\0").decode("ascii", "replace")
    vsize, va, rawsize, rawptr = struct.unpack_from("<IIII", data, off + 8)
    sections.append((name, va, vsize, rawsize, rawptr))
print("image_base", hex(image_base))
for s in sections:
    print(s)

def va_to_off(va):
    rva = va - image_base
    for name, sva, vsize, rawsize, rawptr in sections:
        if sva <= rva < sva + max(vsize, rawsize):
            delta = rva - sva
            if delta >= rawsize:
                raise ValueError("RVA in virtual-only tail: " + hex(va))
            return rawptr + delta
    raise ValueError("VA outside sections: " + hex(va))

procs = {e["address"]: e for e in json.load(open(PROCS, encoding="utf-8"))}
targets = []
for a in WANT:
    e = procs[a]
    off = va_to_off(int(a, 16))
    raw = data[off:off + e["size"]]
    targets.append({
        "address": a,
        "size": e["size"],
        "bytes_sha256": hashlib.sha256(raw).hexdigest(),
        "range_basis": "PDB compiled_procedure size %d; name=%s; B patched EXE sha256 cf048e27" % (e["size"], e["name"]),
    })
json.dump(targets, open(OUT, "w", encoding="utf-8"), indent=2)
print(json.dumps(targets, indent=2))
