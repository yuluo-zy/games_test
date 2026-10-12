import struct, sys

EXE = 'workspace/input/tiny-glade.exe'
IB = 0x140000000
d = open(EXE,'rb').read()

e = struct.unpack_from('<I', d, 0x3c)[0]
nsec = struct.unpack_from('<H', d, e+6)[0]
optsz = struct.unpack_from('<H', d, e+20)[0]
secs = []
off = e + 24 + optsz
for i in range(nsec):
    nm = d[off:off+8].rstrip(b'\x00').decode('latin1')
    vsz, va, rsz, ra = struct.unpack_from('<IIII', d, off+8)
    secs.append((nm, va, vsz, ra, rsz))
    off += 40

def off_of(rva):
    for nm, va, vsz, ra, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            return ra + (rva - va)
    return None

def read_bytes(va, n):
    rva = va - IB
    o = off_of(rva)
    return d[o:o+n]

def hexdump(va, n):
    b = read_bytes(va, n)
    out = []
    for i in range(0, len(b), 16):
        chunk = b[i:i+16]
        hexs = ' '.join('%02X' % x for x in chunk)
        out.append('%08X  %-47s' % (va+i, hexs))
    return '\n'.join(out)

if __name__ == '__main__':
    cmd = sys.argv[1]
    if cmd == 'hex':
        va = int(sys.argv[2], 16)
        n = int(sys.argv[3], 16)
        print(hexdump(va, n))
    elif cmd == 'secs':
        for s in secs: print(s)
