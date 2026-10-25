import json,struct
from pathlib import Path
root=Path(r'D:/game/reverse-engineering/tiny-glade')
symbols=json.loads((root/'evidence/render-architecture/public-symbols.json').read_text())
for s in symbols:
 n=s.get('demangled','')
 if ('delete_affected_leaves' in n and 'retain' in n) or ('IvyDirectionProposer' in n and 'default' in n): print(s.get('va'),n[:400])
d=(root/'workspace/input/tiny-glade.exe').read_bytes(); pe=struct.unpack_from('<I',d,0x3c)[0]; ns=struct.unpack_from('<H',d,pe+6)[0]; sz=struct.unpack_from('<H',d,pe+20)[0]
secs=[struct.unpack_from('<IIII',d,pe+24+sz+40*i+8) for i in range(ns)]
def read(va,n):
 for vs,v,rs,r in secs:
  rva=va-0x140000000
  if v<=rva<v+vs:
   delta=rva-v
   available=max(0,min(n,rs-delta))
   return d[r+delta:r+delta+available]+bytes(n-available)
 raise ValueError(hex(va))
for va,n in [(0x142a16d10,4),(0x142925870,4),(0x142925904,4),(0x142aa4550,16),(0x1429258e0,16),(0x142a96720,32),(0x142aa9130,32)]:
 b=read(va,n); print(hex(va),b.hex(),struct.unpack('<'+'f'*(n//4),b))
# Import names identify the compiler's CRT floating-point calls.
opt=pe+24; import_rva=struct.unpack_from('<I',d,opt+120)[0]
for at in range(0,10000,20):
 row=struct.unpack('<IIIII',read(0x140000000+import_rva+at,20))
 if not any(row):break
 oft,_,_,name,ft=row
 for i in range(10000):
  val=struct.unpack('<Q',read(0x140000000+oft+i*8,8))[0]
  if not val:break
  va=0x140000000+ft+i*8
  if va in [0x143193130,0x1431931a8,0x143193118]: print(hex(va),read(0x140000000+val+2,40).split(b'\0')[0])
