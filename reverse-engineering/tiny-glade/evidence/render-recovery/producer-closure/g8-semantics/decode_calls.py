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
    secs.append((nm, va, vsz, ra, rsz)); off += 40
def off_of(rva):
    for nm, va, vsz, ra, rsz in secs:
        if va <= rva < va + max(vsz, rsz): return ra + (rva - va)
    return None
def read_bytes(va, n):
    o = off_of(va - IB); return d[o:o+n]

def calls(va, n):
    b = read_bytes(va, n); out=[]; i=0
    while i < len(b):
        if b[i]==0xE8 and i+5<=len(b):
            rel=struct.unpack_from('<i',b,i+1)[0]
            site=va+i; out.append((site,site+5+rel)); i+=5
        else: i+=1
    return out

if __name__=='__main__':
    va=int(sys.argv[1],16); n=int(sys.argv[2],16)
    for s,t in calls(va,n): print(hex(s),'->',hex(t))
