"""Follow typed projection/conversion consumers, IPI method kinds and TPI."""
from pathlib import Path
import hashlib,json,struct,sys
from pdb_reader import Pdb,z
sys.path.insert(0,'D:/game/tools/reverse-runtime/utils-gap-20261014/python-deps')
import pefile
OUT=Path(__file__).parent;ROOT=OUT.parents[3]
target=ROOT/'ljxsj_92385/Tiny Glade'
pdb=Pdb(target/'tiny_glade.pdb');pe=pefile.PE(str(target/'tiny-glade.rne'),fast_load=True)
base=pe.OPTIONAL_HEADER.ImageBase
ids={i:(k,b) for i,k,b in pdb.types(4)}
def scope(i):
    k,b=ids.get(i,(0,b''))
    return z(b[4:]) if k==0x1605 else f'ipi:{i:#x}'
names={}
for i,(k,b) in ids.items():
    if k==0x1601:
        parent,typ=struct.unpack_from('<II',b);name=z(b[8:]);namespace=scope(parent)
        if namespace.startswith('utils::') and (
            name in ['as_vec2','reinterpret_as_curve2','try_to_curve2','crashy_to_curve2','to_affine3','bound','is_bounded','find_segment_and_dt','position_at_t','tangent_at_t']
            or 'swizzlers' in namespace):
            names[i]={'id':hex(i),'kind':'LF_FUNC_ID','scope':namespace,'name':name,'type':hex(typ),'hosts':[]}
    elif k==0x1602:
        if any(x in b for x in [b'bound',b'affine',b'vec2',b'segment_and_dt']):
            names[i]={'id':hex(i),'kind':'LF_MFUNC_ID','raw_hex':b.hex(),'hosts':[]}
hits=[]
for i,k,b in pdb.types(2):
    if any(x in b for x in [b'utils::swizzlers',b'Vec3Slice',b'Vec3As2',b'Spline2D',b'WallSpaceCoord']):
        hits.append({'index':hex(i),'kind':hex(k),'size':len(b),'strings':[z(v) for v in b.split(b'\0') if len(v)>5 and any(x in v for x in [b'utils',b'Slice',b'As2',b'Spline',b'Wall'])],'raw_hex':b.hex()})
procs=[];count=0
for mi,module,ss,sb in pdb.modules():
    count+=1
    if ss==0xffff or not sb:continue
    syms=dict((q,(k,b)) for q,k,b in pdb.symbols(ss,sb))
    for rec,(k,b) in syms.items():
        if k in (0x1110,0x110f,0x1147,0x1146):
            name=z(b[35:])
            if any(x in name for x in ['swizzlers','Spline2D','WallSpaceCoord','Vec3As2','Vec3Slice','reinterpret_as_curve2','try_to_curve2']):
                off,seg=struct.unpack_from('<IH',b,28)
                procs.append({'module':mi,'record':rec,'name':name,'address':hex(base+pe.sections[seg-1].VirtualAddress+off),'size':struct.unpack_from('<I',b,12)[0]})
        if k not in (0x114d,0x115d):continue
        parent,_,identity=struct.unpack_from('<III',b)
        if identity not in names:continue
        root=parent;seen=set()
        while root in syms and syms[root][0] not in (0x1110,0x110f,0x1147,0x1146):
            if root in seen:raise RuntimeError('cyclic PDB scope')
            seen.add(root);root=struct.unpack_from('<I',syms[root][1])[0]
        if root not in syms:continue
        pb=syms[root][1];off,seg=struct.unpack_from('<IH',pb,28)
        names[identity]['hosts'].append({'module':mi,'record':rec,'procedure_record':root,'name':z(pb[35:]),'address':hex(base+pe.sections[seg-1].VirtualAddress+off),'size':struct.unpack_from('<I',pb,12)[0],'annotations_hex':b[12:].hex()})
r={'baseline':'B','pdb_sha256':hashlib.sha256(pdb.raw).hexdigest(),'modules_scanned':count,'methods':list(names.values()),'tpi_hits':hits,'procedures':procs,'scope':'Includes LF_FUNC_ID and LF_MFUNC_ID, all compiled procedure scopes, linked inline hosts and TPI type records. Types/absence do not by themselves establish emitted semantics.'}
(OUT/'deep-inventory.json').write_text(json.dumps(r,indent=2)+'\n',encoding='utf8')
print(json.dumps({'methods':[(v.get('scope'),v.get('name'),v['id'],len(v['hosts'])) for v in names.values()],'tpi_hits':len(hits),'modules':count},indent=2))
