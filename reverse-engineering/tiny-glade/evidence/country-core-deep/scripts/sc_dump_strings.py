"""Compact UNK string resolver for decomp files (startup-components domain).

Usage: python sc_dump_strings.py <decomp_file.txt> [max_per_line]
Reads a decomp file, finds all UNK_142xxxx references, resolves each address:
  - If it points at a plausible fat pointer {ptr,len} -> prints as &str '...'
  - Else prints as C-string (truncated to 96 chars, printable).
Prints sorted unique results.
"""
import sys, re, struct

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
            off = rawptr + (rva - va0)
            if off < 0 or off >= len(data):
                raise ValueError
            return off
    raise ValueError


def printable(b):
    return "".join(chr(c) if 32 <= c < 127 else "." for c in b)


def resolve(va):
    try:
        off = va2off(va)
    except ValueError:
        return None
    # try fat pointer {ptr,len} with len in sane range and ptr mapped
    try:
        ptr, ln = struct.unpack_from("<QQ", data, off)
        if 0 < ln < 400 and 0x140000000 <= ptr < 0x141000000:
            soff = va2off(ptr)
            s = data[soff:soff+ln]
            if all(c < 128 for c in s):
                return "&str " + repr(s.decode("utf-8", "replace"))
    except Exception:
        pass
    # C string (up to 96 bytes)
    end = data.find(b"\0", off, off + 200)
    if end < 0:
        end = off + 96
    raw = data[off:end]
    if all(c < 128 for c in raw) and 1 <= len(raw) <= 96:
        return "cstr " + repr(raw.decode("utf-8", "replace"))
    return None


def main():
    path = sys.argv[1]
    txt = open(path, encoding="utf-8", errors="replace").read()
    addrs = sorted(set(int(m, 16) for m in re.findall(r"UNK_([0-9a-fA-F]{9,16})", txt)))
    out = {}
    for va in addrs:
        r = resolve(va)
        if r is not None:
            out[va] = r
    for va in sorted(out):
        print(f"0x{va:x}  {out[va]}")


if __name__ == "__main__":
    main()
