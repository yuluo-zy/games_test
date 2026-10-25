import json,struct
from pathlib import Path
p=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne');raw=p.read_bytes();pe=struct.unpack_from('<I',raw,60)[0];n=struct.unpack_from('<H',raw,pe+6)[0];opt=struct.unpack_from('<H',raw,pe+20)[0]
def at(va,size):
 rva=va-0x140000000
 for i in range(n):
  off=pe+24+opt+40*i;vsz,vaddr,sz,fpos=struct.unpack_from('<IIII',raw,off+8)
  if vaddr<=rva<vaddr+max(vsz,sz):return raw[fpos+rva-vaddr:fpos+rva-vaddr+size]
 raise ValueError('unmapped')
print(json.dumps([{'va':hex(va),'hex':at(va,size).hex(),'utf8':at(va,size).decode('utf8')} for va,size in [(0x142cdc5fe,8),(0x142cdc5ba,7)]]))
