#!/usr/bin/env python3
# Read-only: read C strings at given VAs.
import struct, sys
sys.stdout.reconfigure(errors='replace')
d=open(sys.argv[1],'rb').read()
e=struct.unpack_from('<I',d,0x3c)[0]; nsec=struct.unpack_from('<H',d,e+6)[0]
optsz=struct.unpack_from('<H',d,e+20)[0]; secs=[]; off=e+24+optsz
for i in range(nsec):
    nm=d[off:off+8].rstrip(b'\x00').decode('latin1')
    vsz,va,rsz,ra=struct.unpack_from('<IIII',d,off+8); secs.append((nm,va,vsz,ra,rsz)); off+=40
IB=0x140000000
def off_of(rva):
    for nm,va,vsz,ra,rsz in secs:
        if va<=rva<va+max(vsz,rsz): return ra+(rva-va)
for vs in sys.argv[2].split(','):
    va=int(vs,16); fo=off_of(va-IB)
    if fo is None: print(vs,'no section'); continue
    end=d.find(b'\x00',fo,fo+200)
    print(vs, 'off=0x%x'%fo, repr(d[fo:end].decode('utf-8','replace')))
