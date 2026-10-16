"""Bind a small original PDB procedure selection to exact distribution bytes."""
import hashlib,json,struct
from pathlib import Path
OUT=Path(__file__).resolve().parent
GAME=OUT.parents[3]/'ljxsj_92385/Tiny Glade'
raw=(GAME/'tiny-glade.rne').read_bytes()
patched=(GAME/'tiny-glade.exe').read_bytes()
pe=struct.unpack_from('<I',raw,0x3c)[0]
nsec=struct.unpack_from('<H',raw,pe+6)[0]
opt=struct.unpack_from('<H',raw,pe+20)[0]
sections=[struct.unpack_from('<IIII',raw,pe+24+opt+40*i+8) for i in range(nsec)]
def code(va,size):
 rva=va-0x140000000
 for vs,rv,rs,off in sections:
  if rv<=rva<rv+rs:
   pos=off+rva-rv
   return raw[pos:pos+size],patched[pos:pos+size]
 raise ValueError(hex(va))
addresses={0x141e4c650,0x141e4c6c0,0x140683ec0,0x140fe0e10,0x142374e10,0x141e9df60,0x140fe7cb0,0x140fe7e40}
rows=[]
all_toolbar=[]
for line in (OUT/'procs.tsv').read_text(encoding='utf-8').splitlines():
 va,size,mi,obj,name=line.split('\t',4)
 if name.startswith('toolbar::'):
  all_toolbar.append({'address':va,'size':int(size),'module':int(mi),'object':obj,'name':name})
 if int(va,16) not in addresses:continue
 original,modified=code(int(va,16),int(size))
 assert original==modified,va
 rows.append({'address':va,'size':int(size),'bytes_sha256':hashlib.sha256(original).hexdigest(),
              'range_basis':f'original PDB compiled procedure S_[GL]PROC32: module={mi}; {name}',
              'original_matches_patched':True,'name':name})
assert len({r['address'] for r in rows})==len(addresses)
(OUT/'bounded-procedures.json').write_text(json.dumps(rows,indent=2)+'\n',encoding='utf-8')
(OUT/'toolbar-procedures.json').write_text(json.dumps(all_toolbar,indent=2)+'\n',encoding='utf-8')
targets=[{k:r[k] for k in ['address','size','bytes_sha256','range_basis']} for r in rows]
job=[{'tool':'get_metadata','args':{'program':'tiny-glade.exe'}},
     {'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':targets}},
     {'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':','.join(r['address'] for r in rows),'fields':'signature,decompiled_code,callees'}},
     {'tool':'save_program','args':{'program':'tiny-glade.exe'}}]
(OUT/'bounded-query.json').write_text(json.dumps(job,indent=2),encoding='utf-8')
print(json.dumps({'toolbar_compiled_procedures':len(all_toolbar),'selected':rows},indent=2))
