"""Collect B math definitions and bounded original byte ranges, not ABI claims."""
import bisect
import hashlib
import json
from pathlib import Path
import re
import struct

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'evidence/shared-foundation-2026-10-13/geometry'
OUT.mkdir(parents=True,exist_ok=True)
binary=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw=binary.read_bytes()
peoff=struct.unpack_from('<I',raw,0x3c)[0]
nsections=struct.unpack_from('<H',raw,peoff+6)[0]
opt_size=struct.unpack_from('<H',raw,peoff+20)[0]
base=struct.unpack_from('<Q',raw,peoff+24+24)[0]
sections=[]
for i in range(nsections):
    o=peoff+24+opt_size+i*40
    name=raw[o:o+8].split(b'\0')[0].decode()
    virtual_size,rva,raw_size,offset=struct.unpack_from('<IIII',raw,o+8)
    sections.append((name,rva,virtual_size,offset,raw_size))
def read_rva(rva,size):
    for _,start,virtual_size,offset,raw_size in sections:
        if start<=rva<start+max(virtual_size,raw_size):
            return raw[offset+rva-start:offset+rva-start+size]
    raise ValueError(rva)
pdata=next(x for x in sections if x[0]=='.pdata')
unwind={}
for o in range(pdata[3],pdata[3]+pdata[4]-11,12):
    begin,end,_=struct.unpack_from('<III',raw,o)
    if begin and end>begin:unwind[base+begin]=end-begin
all_symbols=json.loads((ROOT/'evidence/render-architecture/public-symbols.json').read_text(encoding='utf-8'))
starts=sorted({int(s['va'],16) for s in all_symbols if s.get('va')})
targets={}
for s in all_symbols:
    name=s['demangled']
    if not s.get('va') or not name.startswith('utils::'):continue
    if not any(x in name for x in ('utils::geometry::','utils::curve::','utils::components::transform::')):continue
    if any(x in name for x in ('Serialize','Deserialize','serialize','deserialize','drop_in_place')):continue
    a=int(s['va'],16)
    canonical=re.sub(r'::h[0-9a-f]{16}.*$','',name)
    size=unwind.get(a)
    source='pe_unwind_contiguous_range' if size else 'next_public_symbol_upper_bound; not_PDB_body'
    if size is None:
        i=bisect.bisect_right(starts,a)
        size=(starts[i]-a) if i<len(starts) else 256
    size=min(size,24000)
    row=targets.setdefault(a,{'address':hex(a),'size':size,'range_basis':source,'names':[],
                              'bytes_sha256':hashlib.sha256(read_rva(a-base,size)).hexdigest()})
    if canonical not in row['names']:row['names'].append(canonical)
original_sha=hashlib.sha256(raw).hexdigest()
assert original_sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
(OUT/'targets.json').write_text(json.dumps({'original_binary_sha256':original_sha,'image_base':hex(base),
    'targets':sorted(targets.values(),key=lambda r:int(r['address'],16)),
    'limits':['范围只用于限定取证；无unwind时是symbol上界，不等于作者完整函数body','仅私有定义名字，不恢复serde/std/glam实现']},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'targets':len(targets),'range_basis_counts':{k:sum(r['range_basis']==k for r in targets.values()) for k in {r['range_basis'] for r in targets.values()}}}))
import_rva,import_size=struct.unpack_from('<II',raw,peoff+24+112+8)
imports={}
def string_rva(rva):
    return read_rva(rva,512).split(b'\0',1)[0].decode(errors='replace')
for i in range(import_size//20):
    desc=read_rva(import_rva+i*20,20)
    original_first,_,_,name_rva,first=struct.unpack('<IIIII',desc)
    if not name_rva:break
    dll=string_rva(name_rva)
    thunk=original_first or first
    j=0
    while True:
        value=struct.unpack('<Q',read_rva(thunk+j*8,8))[0]
        if not value:break
        symbol='ordinal:'+str(value&0xffff) if value>>63 else string_rva(value+2)
        imports[hex(base+first+j*8)]={'dll':dll,'name':symbol}
        j+=1
(OUT/'imports.json').write_text(json.dumps(imports,ensure_ascii=False,indent=2),encoding='utf-8')
index=OUT/'decompilation-index.json'
if index.exists():
    code='\n'.join(r['code'] for r in json.loads(index.read_text(encoding='utf-8')))
    addresses=sorted({int(a,16) for a in re.findall(r'(?:DAT|UNK)_([0-9a-f]{9})',code)})
    constants={}
    for a in addresses:
        data=read_rva(a-base,32)
        constants[hex(a)]={'hex':data.hex(),'f32':list(struct.unpack('<8f',data))}
    (OUT/'constants.json').write_text(json.dumps(constants,ensure_ascii=False,indent=2),encoding='utf-8')
