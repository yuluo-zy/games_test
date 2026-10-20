from pathlib import Path
import json,struct,sys
sys.path.insert(0,'D:/game/reverse-engineering/tiny-glade/evidence/utils-deep-followup-2026-10-14')
from pdb_reader import Pdb,z
p=Pdb('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
types={i:(kind,b) for i,kind,b in p.types(2)}
def num(b,at):
    v=struct.unpack_from('<H',b,at)[0]
    if v<0x8000:return v,at+2
    sizes={0x8000:1,0x8001:2,0x8002:2,0x8003:4,0x8004:4,0x8009:8,0x800a:8}
    n=sizes[v];return int.from_bytes(b[at+2:at+2+n],'little'),at+2+n
def typename(index):
    kind,b=types.get(index,(0,b''))
    if kind in (0x1504,0x1505):return z(b[num(b,16)[1]:])
    return hex(index)
rows=[]
for index,(kind,b) in types.items():
    if kind not in (0x1504,0x1505):continue
    name=typename(index)
    if name not in ('country_core::resources::walls::decorator_archivist::DecoratorArchivalResources',):continue
    size,_=num(b,16);field=struct.unpack_from('<I',b,4)[0]
    row={'id':hex(index),'name':name,'size':size,'fieldlist':hex(field),'members':[]}
    _,body=types.get(field,(0,b''));at=0
    while at<len(body):
        leaf=struct.unpack_from('<H',body,at)[0]
        if leaf!=0x150d:
            row['unparsed_tail']=body[at:].hex();break
        flags,type_id=struct.unpack_from('<HI',body,at+2)
        offset,end=num(body,at+8);name=z(body[end:]);end=body.index(0,end)+1
        row['members'].append({'name':name,'offset':offset,'type_id':hex(type_id),'type':typename(type_id)})
        at=(end+3)&~3
    rows.append(row)
out=Path(__file__).with_name('archival-original-type.json');out.write_text(json.dumps(rows,indent=2),encoding='utf8')
print(json.dumps(rows,indent=2))
