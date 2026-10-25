import hashlib,json,struct
from pathlib import Path
B=Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne'); data=B.read_bytes()
assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
nt=struct.unpack_from('<I',data,0x3c)[0];n=struct.unpack_from('<H',data,nt+6)[0];opt=struct.unpack_from('<H',data,nt+20)[0];base=struct.unpack_from('<Q',data,nt+24+24)[0]
sections=[struct.unpack_from('<8sIIII',data,nt+24+opt+40*i) for i in range(n)]
def read(va,length):
 rva=va-base
 for name,vs,addr,rs,off in sections:
  if addr<=rva and rva+length<=addr+rs:return data[off+rva-addr:off+rva-addr+length]
 raise ValueError(hex(va))
out={'binary':str(B),'binary_sha256':hashlib.sha256(data).hexdigest(),'scope':'Four finite original location tables and only RIP literals referenced by bounded 8143B producer/1203B helper. No execution.','tables':[],'f32':[]}
for va,count,first in [(0x142a8eab0,4,0),(0x142a8eac0,22,4),(0x142a8eb18,22,4),(0x142a8eb70,22,4)]:
 raw=read(va,count*4);offsets=struct.unpack('<'+'i'*count,raw)
 out['tables'].append({'va':hex(va),'raw_hex':raw.hex(),'sha256':hashlib.sha256(raw).hexdigest(),'entries':[{'index':i+first,'offset':v,'target':hex(va+v)} for i,v in enumerate(offsets)]})
for va in [0x142925870,0x1429258d8,0x142925904,0x142925908,0x142925950,0x142925984,0x14295a0e8,0x1429a0764,0x1429cd068,0x1429eb6f8,0x142a7c87c,0x142a82c10,0x142a867a8,0x142a867ac,0x142a867b0,0x142a8e4f8,0x142a8e4fc,0x142a8e500,0x142a8e504]:
 raw=read(va,4);out['f32'].append({'va':hex(va),'bits':hex(struct.unpack('<I',raw)[0]),'value':struct.unpack('<f',raw)[0]})
Path(__file__).with_name('finite-original-tables.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({'tables':out['tables'],'f32':out['f32']},indent=2))
