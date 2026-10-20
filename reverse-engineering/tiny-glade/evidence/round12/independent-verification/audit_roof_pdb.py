"""Read-only independent physical/inline evidence audit, not a reachability proof."""
from pathlib import Path
import hashlib, json, struct, sys

OUT = Path(__file__).resolve().parent
sys.path.insert(0, 'D:/game/reverse-engineering/tiny-glade/evidence/utils-deep-followup-2026-10-14')
from pdb_reader import Pdb, z

pdb = Pdb('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
records = dict((i,(kind,payload)) for i,kind,payload in pdb.types(4))
ids = {}
for index,(kind,payload) in records.items():
    if kind != 0x1601: continue
    parent = struct.unpack_from('<I', payload)[0]
    scope_kind,scope = records.get(parent,(0,b''))
    name = (z(scope[4:])+'::' if scope_kind == 0x1605 else '') + z(payload[8:])
    if 'place_roof_support_beams' in name: ids[index] = name

report = {'pdb_sha256':hashlib.sha256(pdb.raw).hexdigest(), 'ipi_records':len(records),
          'producer_method_ids':{hex(k):v for k,v in ids.items()}, 'physical_producers':[],
          'producer_inline_sites':[], 'source_c13':[], 'module_count':0, 'symbol_count':0}
dbi = pdb.stream(3)
string_stream = pdb.stream(4886)
strings_size = struct.unpack_from('<I',string_stream,8)[0]
strings = string_stream[12:12+strings_size]
at = 64
limit = at + struct.unpack_from('<I',dbi,24)[0]
while at < limit:
    module = report['module_count']; report['module_count'] += 1
    stream_index,symbol_bytes,c11,c13 = struct.unpack_from('<HIII', dbi, at+34)
    first = dbi.index(0,at+64); second = dbi.index(0,first+1); at = (second+4)&~3
    if stream_index == 0xffff: continue
    stream = pdb.stream(stream_index)
    offset = 4
    while offset < symbol_bytes:
        size,kind = struct.unpack_from('<HH',stream,offset)
        payload = stream[offset+4:offset+size+2]
        report['symbol_count'] += 1
        if kind in (0x110f,0x1110,0x1146,0x1147):
            name = z(payload[35:])
            if 'system_roof::visual::place_support_beams::place_roof_support_beams' in name:
                report['physical_producers'].append({'module':module,'name':name,'record':offset})
        if kind in (0x114d,0x115d):
            identity = struct.unpack_from('<I',payload,8)[0]
            if identity in ids: report['producer_inline_sites'].append({'module':module,'identity':hex(identity),'record':offset})
        offset += size+2
    chunks = stream[symbol_bytes+c11:symbol_bytes+c11+c13]
    sub_at = 0; sub = []
    while sub_at < len(chunks):
        kind,size = struct.unpack_from('<II',chunks,sub_at)
        sub.append((kind,chunks[sub_at+8:sub_at+8+size])); sub_at = (sub_at+8+size+3)&~3
    paths = {}
    for kind,payload in sub:
        if kind != 0xf4: continue
        sub_at = 0
        while sub_at < len(payload):
            string_offset,n,_ = struct.unpack_from('<IBB',payload,sub_at)
            path = z(strings[string_offset:])
            if path.endswith('/roof/src/visual/place_support_beams.rs'):
                paths[sub_at] = path
                report['source_c13'].append({'module':module,'kind':'checksum','path':path,'checksum':payload[sub_at+6:sub_at+6+n].hex()})
            sub_at = (sub_at+6+n+3)&~3
    for kind,payload in sub:
        if kind == 0xf2:
            sub_at = 12
            while sub_at < len(payload):
                file_index,lines,size = struct.unpack_from('<III',payload,sub_at)
                if file_index in paths: report['source_c13'].append({'module':module,'kind':'physical_lines','line_count':lines})
                sub_at += size
        elif kind == 0xf6:
            extended = struct.unpack_from('<I',payload)[0]; sub_at = 4
            while sub_at < len(payload):
                identity,file_index,line = struct.unpack_from('<III',payload,sub_at); sub_at += 12
                if extended:
                    count = struct.unpack_from('<I',payload,sub_at)[0]; sub_at += 4+count*4
                if identity in ids or file_index in paths:
                    report['source_c13'].append({'module':module,'kind':'inline_declaration','identity':hex(identity),'line':line})

report['limitation'] = 'CodeView body/inline evidence absence does not prove absence of anonymous or indirect dynamic writers.'
(OUT/'roof-pdb-independent.json').write_text(json.dumps(report,indent=2),encoding='utf8')
print(json.dumps(report,indent=2))
