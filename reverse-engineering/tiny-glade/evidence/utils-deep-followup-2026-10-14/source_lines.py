"""Inspect every B C13 physical source-line record, independent of symbol names.

Inline annotations are separately inventoried by deep_inventory.py. Physical
line absence is not evidence that author source was unreachable or dead.
"""
from pathlib import Path
import json,struct
from pdb_reader import Pdb,z
OUT=Path(__file__).parent;ROOT=OUT.parents[3]
p=Pdb(ROOT/'ljxsj_92385/Tiny Glade/tiny_glade.pdb')
# B /names, independently identified with llvm-pdbutil --named-streams.
names=p.stream(4886);sig,version,size=struct.unpack_from('<III',names)
assert sig==0xeffeeffe and version==1
strings=names[12:12+size]
d=p.stream(3);total=struct.unpack_from('<I',d,24)[0];pos=64;mi=0
rows=[];inline=[];modules=0;line_subsections=0
ids={i:(k,b) for i,k,b in p.types(4)}
def method(identity):
    kind,b=ids.get(identity,(0,b''))
    if kind!=0x1601:return hex(identity)
    parent=struct.unpack_from('<I',b)[0];sk,sb=ids.get(parent,(0,b''))
    return (z(sb[4:])+'::' if sk==0x1605 else '')+z(b[8:])
while pos<64+total:
    stream,sym,c11,c13=struct.unpack_from('<HIII',d,pos+34)
    a=pos+64;b=d.index(b'\0',a);c=d.index(b'\0',b+1)
    module=z(d[a:b]);pos=(c+4)&~3
    if stream!=0xffff and c13:
        data=p.stream(stream)[sym+c11:sym+c11+c13];q=0;subs=[];checks={}
        while q<len(data):
            kind,n=struct.unpack_from('<II',data,q);body=data[q+8:q+8+n]
            assert len(body)==n
            subs.append((kind,body));q=(q+8+n+3)&~3
        for kind,body in subs:
            if kind!=0xf4:continue
            q=0
            while q<len(body):
                offset,n,ckind=struct.unpack_from('<IBB',body,q)
                checks[q]={'path':z(strings[offset:strings.index(b'\0',offset)]),'checksum_kind':ckind,'checksum':body[q+6:q+6+n].hex()}
                q=(q+6+n+3)&~3
        for kind,body in subs:
            if kind==0xf6:
                signature=struct.unpack_from('<I',body)[0];r=4
                assert signature in (0,1)
                while r<len(body):
                    identity,file_id,line=struct.unpack_from('<III',body,r);r+=12
                    info=checks.get(file_id,{'path':f'unknown:{file_id}'})
                    if signature:
                        extras=struct.unpack_from('<I',body,r)[0];r+=4+4*extras
                    if any(info['path'].endswith('/'+x) for x in ['wall_space.rs','natural_cubic.rs','swizzlers.rs']):
                        inline.append({'module':mi,'identity':hex(identity),'method':method(identity),'line':line,**info})
            if kind!=0xf2:continue
            line_subsections+=1
            off,section,flags,length=struct.unpack_from('<IHHI',body);q=12
            while q<len(body):
                file_id,count,blocksize=struct.unpack_from('<III',body,q)
                info=checks.get(file_id,{'path':f'unknown:{file_id}'})
                if any(info['path'].endswith('/'+x) for x in ['wall_space.rs','natural_cubic.rs','swizzlers.rs']):
                    entries=[]
                    for j in range(count):
                        delta,lf=struct.unpack_from('<II',body,q+12+j*8)
                        entries.append({'line':lf&0xffffff,'end_line':(lf&0xffffff)+((lf>>24)&0x7f),'section_offset':hex(off+delta)})
                    rows.append({'module':mi,'module_name':module,**info,'section':section,'range_offset':hex(off),'range_size':length,'entries':entries})
                assert blocksize>=12+count*(12 if flags&1 else 8)
                q+=blocksize
        modules+=1
    mi+=1
result={'modules_total':mi,'modules_with_c13':modules,'line_subsections_total':line_subsections,'source_ranges':rows,'inline_declarations':inline}
(OUT/'source-line-inventory.json').write_text(json.dumps(result,indent=2),encoding='utf8')
for fn in ['wall_space.rs','natural_cubic.rs','swizzlers.rs']:
    selected=[r for r in rows if r['path'].endswith('/'+fn)]
    lines=sorted({e['line'] for r in selected for e in r['entries']})
    print(fn,len(selected),'ranges','source lines:',lines)
print('total',mi,'modules;',line_subsections,'line subsections')
