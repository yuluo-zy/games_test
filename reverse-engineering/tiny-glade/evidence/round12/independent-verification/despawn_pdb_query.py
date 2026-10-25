from pathlib import Path
import json,struct,sys
sys.path.insert(0,'D:/game/reverse-engineering/tiny-glade/evidence/utils-deep-followup-2026-10-14')
from pdb_reader import Pdb,z
p=Pdb('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
modules=list(p.modules()); _,name,stream,size=modules[490]
symbols=list(p.symbols(stream,size)); active=False; output=[]; types=set()
for offset,kind,b in symbols:
    if kind in (0x110f,0x1110,0x1146,0x1147):
        active=z(b[35:])=='country_core::systems::despawn::despawn_glade_entities'
        if active: types.add(struct.unpack_from('<I',b,24)[0])
    if active:
        output.append({'offset':offset,'kind':hex(kind),'payload':b.hex(),'name':z(b[35:]) if kind in (0x110f,0x1110,0x1146,0x1147) else None})
        if kind==0x113e: types.add(struct.unpack_from('<I',b)[0])
        if kind in (0x0006,0x114f):active=False
result={'module':name,'symbols':output,'types':{}}
for index,kind,b in p.types(2):
    # Names and immediate arglist candidates only; no guessed ABI.
    if index in types:
        result['types'][hex(index)]={'kind':hex(kind),'payload':b.hex()}
        if kind==0x1008:types.add(struct.unpack_from('<I',b,8)[0])
        if kind==0x1201:
            n=struct.unpack_from('<I',b)[0]
            result['types'][hex(index)]['arg_ids']=[hex(v) for v in struct.unpack_from('<'+'I'*n,b,4)]
Path(__file__).with_name('despawn-pdb.json').write_text(json.dumps(result,indent=2),encoding='utf8')
ids={i:(k,b) for i,k,b in p.types(4)}
selected={struct.unpack_from('<I',bytes.fromhex(row['payload']),8)[0] for row in output if row['kind']=='0x114d'}
names={}
for i in selected:
    k,b=ids[i]
    parent=struct.unpack_from('<I',b)[0]
    sk,sb=ids.get(parent,(0,b''))
    names[hex(i)]=(z(sb[4:])+'::' if sk==0x1605 else '')+z(b[8:])
Path(__file__).with_name('despawn-inline-methods.json').write_text(json.dumps(names,indent=2),encoding='utf8')
print(json.dumps({k:v for k,v in names.items() if any(s in v for s in ['Query','despawn','With<','Without<','iter_inner'])},indent=2))
