"""Prepare bounded private B asset functions; PDB procedures supply the lexical ranges."""
import hashlib,json,pathlib,struct
OUT=pathlib.Path(__file__).resolve().parent
ROOT=OUT.parents[2]
raw=pathlib.Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,0x3c)[0]
n=struct.unpack_from('<H',raw,pe+6)[0];op=struct.unpack_from('<H',raw,pe+20)[0]
base=struct.unpack_from('<Q',raw,pe+48)[0]
sections=[]
for i in range(n):
    p=pe+24+op+40*i
    vs,rva,size,off=struct.unpack_from('<IIII',raw,p+8)
    sections.append((base+rva,size,off))
def read(va,size):
    for start,length,offset in sections:
        if start<=va and va+size<=start+length:
            return raw[offset+va-start:offset+va-start+size]
    raise ValueError(hex(va))
rows=json.loads((ROOT/'evidence/shared-foundation-2026-10-13/assets/functions.json').read_text())
targets=[]
for r in rows:
    if not r['name'].startswith('asset_pipe::texture::') or 'serialize' in r['name'] or 'fmt' in r['name']:continue
    a=int(r['va'],16)
    targets.append({'address':r['va'],'size':r['size'],'bytes_sha256':hashlib.sha256(read(a,r['size'])).hexdigest(),
        'range_basis':'LLVM B PDB S_GPROC32 procedure code_size; includes EH funclets; source identity in prior assets/identity.json','name':r['name']})
(OUT/'targets.json').write_text(json.dumps(targets,indent=2)+'\n',encoding='utf8')
job=[{'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':[{k:v for k,v in r.items() if k!='name'} for r in targets]}},
     {'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':','.join(r['address'] for r in targets),
        'fields':'name,body,signature,decompiled_code,callees'}}]
(OUT/'bounded-job.json').write_text(json.dumps(job,indent=2)+'\n',encoding='utf8')
print(len(targets))
