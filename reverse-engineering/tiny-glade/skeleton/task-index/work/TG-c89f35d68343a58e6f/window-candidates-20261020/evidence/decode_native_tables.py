from pathlib import Path
import json,struct,hashlib
p=Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne');raw=p.read_bytes()
layout=json.loads(Path('reverse-engineering/tiny-glade/skeleton/_pdb/B/input.json').read_text());base=0x140000000

def read(va,size):
 for s in layout['sections']:
  d=va-base-s['rva']
  if 0<=d and d+size<=s['raw_size']:return raw[s['raw_pointer']+d:s['raw_pointer']+d+size]
 raise ValueError(hex(va))
results=[]
for name,va,n in [('fullvisual-candidate-copy',0x142a91c18,6),('on-windows-kind',0x143042370,6)]:
 b=read(va,n*4);results.append({'name':name,'va':hex(va),'raw_hex':b.hex(),'sha256':hashlib.sha256(b).hexdigest(),'entries':[{'tag':i,'signed_relative':d,'target':hex(va+d)} for i,d in enumerate(struct.unpack('<'+'i'*n,b))]})
for va,n in [(0x1429cd0c0,4),(0x142925904,1)]:
 b=read(va,n*4);results.append({'name':'consumer-constant','va':hex(va),'raw_hex':b.hex(),'f32':list(struct.unpack('<'+'f'*n,b))})
w=Path(__file__).parent;(w/'tables-native-B.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'read_only':True,'tables':results},indent=2)+'\n')
print(json.dumps(results,indent=2))
