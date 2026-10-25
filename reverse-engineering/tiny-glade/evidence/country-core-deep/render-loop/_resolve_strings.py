import struct
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
            return rawptr + (rva - va0)
    return None
def cstr(va, maxlen=512):
    off = va2off(va)
    if off is None: return None
    end = data.find(b"\0", off, off + maxlen)
    if end < 0: end = off + maxlen
    try: return data[off:end].decode("utf-8", "replace")
    except: return None
addrs = []
for line in open('evidence/country-core-deep/render-loop/_unks.txt'):
    addrs.append(int(line.strip()[4:], 16))
with open('evidence/country-core-deep/render-loop/_string_table.txt', 'w', encoding='utf-8') as f:
    for va in addrs:
        s = cstr(va)
        if s and 1 <= len(s) <= 200:
            f.write(f"{va:08x}  {s!r}\n")
print('done', len(addrs))
