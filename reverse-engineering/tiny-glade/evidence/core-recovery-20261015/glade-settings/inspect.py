import struct,json
from pathlib import Path
p=Path('reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes()
pe=struct.unpack_from('<I',p,0x3c)[0];ns=struct.unpack_from('<H',p,pe+6)[0];sz=struct.unpack_from('<H',p,pe+20)[0]
sects=[struct.unpack_from('<IIII',p,pe+24+sz+40*i+8) for i in range(ns)]
def read(va,n):
 rva=va-0x140000000
 for vs,v,rs,r in sects:
  if v<=rva<v+vs:
   d=rva-v; k=max(0,min(n,rs-d));return p[r+d:r+d+k]+bytes(n-k)
 raise ValueError(hex(va))
for i in range(11):
 a,n=struct.unpack('<QQ',read(0x142aceb88+i*16,16))
 print(i,hex(a),read(a,n).decode(),hex(0x142c5dac8+struct.unpack('<i',read(0x142c5dac8+i*4,4))[0]))
