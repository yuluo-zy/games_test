import runpy,struct,json
from pathlib import Path
r=Path(__file__).parent
m=runpy.run_path(str(r/'inspect_modes.py'));d=m['data'];off=m['offset']
def strings(va,n):
 a=[]
 for i in range(n):
  ptr,sz=struct.unpack_from('<QQ',d,off(va)+i*16);a.append(d[off(ptr):off(ptr)+sz].decode())
 return a
categories=strings(0x142ae5888,22)
(r/'category-names.json').write_text(json.dumps(categories,indent=2));print(categories)
clutter=strings(0x142cda7c8,56)
(r/'clutter-names.json').write_text(json.dumps(clutter,indent=2));print(clutter)
for va,n in [(0x142ae61ec,16),(0x142ad890c,5),(0x142ad8920,5),(0x142ad8934,5),(0x142ad8948,5),(0x142ad895c,5),(0x142ad8970,5),(0x142ad8984,5),(0x142ad8998,5),(0x142ad89ac,5),(0x142ad89c0,5),(0x142ad89d4,5),(0x142ad89e8,5),(0x142ad89fc,5)]:
 print(hex(va),[hex(va+struct.unpack_from('<i',d,off(va)+i*4)[0]) for i in range(n)])
