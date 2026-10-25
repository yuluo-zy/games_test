"""Read strings/bytes from the B (RNE) binary by VA.
  python scripts/rne-read.py <va...>           -> NUL-terminated C strings
  python scripts/rne-read.py --fat <va...>     -> &str fat pointers ({ptr,len})
  python scripts/rne-read.py --hex <va> <n>    -> hex dump of n bytes
  python scripts/rne-read.py --x64 <va...>     -> dump u64 values
"""
import sys, struct
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
            if rva - va0 >= rawsize:
                raise ValueError(f"VA {va:#x} is zero-filled virtual data in {name}, not a raw-file offset")
            return rawptr + (rva - va0)
    raise ValueError(f"VA {va:#x} not mapped")

mode = "cstr"
args = sys.argv[1:]
if args and args[0].startswith("--"):
    mode = args.pop(0)

for a in args:
    if mode == "--unks":
        break
    va = int(a, 16)
    try:
        off = va2off(va)
        if mode == "--fat":
            ptr, ln = struct.unpack_from("<QQ", data, off)
            s = data[va2off(ptr):va2off(ptr)+ln].decode("utf-8", "replace")
            print(f"{a}  -> {s!r}")
        elif mode == "--hex":
            n = int(args[args.index(a)+1], 16)
            print(f"{a}  {data[off:off+n].hex()}")
            break
        elif mode == "--x64":
            vals = struct.unpack_from("<" + "Q"*min(8, (len(data)-off)//8), data, off)
            print(f"{a}  " + " ".join(f"{v:#x}" for v in vals))
        else:
            end = data.find(b"\0", off, off + 256)
            print(f"{a}  {data[off:end].decode('utf-8','replace')!r}")
    except Exception as e:
        print(f"{a}  ERR {e}")

if mode == "--unks":
    # Extract all UNK_142xxxxx refs from decomp file(s), resolve fat-ptr or cstr.
    import re as _re
    unks = set()
    for fn in args:
        text = open(fn, encoding="utf-8", errors="replace").read()
        unks.update(_re.findall(r"UNK_142[a-f0-9]{5,6}", text))
    for u in sorted(unks):
        va = int(u[4:], 16)
        try:
            off = va2off(va)
            ptr, ln = struct.unpack_from("<QQ", data, off)
            if 0x140000000 <= ptr < 0x144000000 and 0 < ln < 4096:
                s = data[va2off(ptr):va2off(ptr)+ln].decode("utf-8", "replace")
                if any(c.isprintable() or c in "\n\t" for c in s) and "\\" not in s[:40]:
                    print(f"{u}  fat -> {s!r}")
                    continue
        except Exception:
            pass
        try:
            off = va2off(va)
            end = data.find(b"\0", off, off + 256)
            s = data[off:end].decode("utf-8", "replace")
            if 4 <= len(s) <= 200 and all(32 <= ord(c) < 127 or c in "\n" for c in s):
                print(f"{u}  cstr -> {s!r}")
        except Exception:
            pass
