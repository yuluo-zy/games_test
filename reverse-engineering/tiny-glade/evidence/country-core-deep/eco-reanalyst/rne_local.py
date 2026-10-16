#!/usr/bin/env python3
"""Read strings/bytes from the B RNE binary by VA (local mac variant).
  python3 rne_local.py <va...>          -> NUL-terminated C strings
  python3 rne_local.py --fat <va...>    -> &str fat pointers ({ptr,len})
  python3 rne_local.py --hex <va> <n>   -> hex dump of n bytes
  python3 rne_local.py --x64 <va...>    -> dump u64 values
"""
import sys, struct

BIN = "/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne"
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
            if rva - va0 >= rawsize:
                raise ValueError(f"VA {va:#x} is zero-filled virtual data in {name}, not a raw-file offset")
            return rawptr + (rva - va0)
    raise ValueError(f"VA {va:#x} not mapped")

mode = "cstr"
args = sys.argv[1:]
if args and args[0].startswith("--"):
    mode = args.pop(0)

if mode == "--hex":
    va = int(args[0], 16)
    n = int(args[1])
    off = va2off(va)
    for i in range(0, n, 16):
        chunk = data[off+i:off+min(i+16, n)]
        hexs = " ".join(f"{b:02x}" for b in chunk)
        asc = "".join(chr(b) if 32 <= b < 127 else "." for b in chunk)
        print(f"{va+i:016x}  {hexs:<48}  {asc}")
    sys.exit(0)

for a in args:
    va = int(a, 16)
    if mode == "--fat":
        off = va2off(va)
        ptr, ln = struct.unpack_from("<QQ", data, off)
        if ptr == 0:
            print(f"{va:#x}: NULL")
            continue
        try:
            poff = va2off(ptr)
        except ValueError:
            print(f"{va:#x}: ptr {ptr:#x} not mapped")
            continue
        s = data[poff:poff+ln]
        print(f"{va:#x}: {s!r}")
    elif mode == "--f32":
        off = va2off(va)
        f = struct.unpack_from("<f", data, off)[0]
        print(f"{va:#x}: {f:.9g}")
    elif mode == "--x64":
        off = va2off(va)
        v = struct.unpack_from("<Q", data, off)[0]
        print(f"{va:#x}: {v:#x} ({v})")
    else:
        off = va2off(va)
        end = data.find(b"\0", off)
        s = data[off:end]
        try:
            print(f"{va:#x}: {s.decode('utf-8')!r}")
        except UnicodeDecodeError:
            print(f"{va:#x}: {s!r}")
