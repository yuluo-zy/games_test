"""Bounded raw-RNE/typed caller evidence for canvas and its missing owners."""
from pathlib import Path
import bisect,hashlib,json,re,struct,subprocess

HERE=Path(__file__).resolve().parent
TARGET=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
data=TARGET.read_bytes();sha=hashlib.sha256(data).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',data,60)[0];n=struct.unpack_from('<H',data,pe+6)[0]
so=pe+24+struct.unpack_from('<H',data,pe+20)[0]
sections=[struct.unpack_from('<IIII',data,so+40*i+8) for i in range(n)]
def read(va,size):
    rva=va-0x140000000
    for vs,start,rs,raw in sections:
        if start<=rva and rva+size<=start+max(vs,rs):return data[raw+rva-start:raw+rva-start+size]
    raise ValueError(hex(va))

procs={};query_records=[];functions={0x140a7abd0,0x140aa9380,0x1407706d0,0x1411fcad0,0x14075b810,0x1403770b0,0x1400d25f0}
for line in Path('D:/game/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').open(encoding='utf8'):
    f=line.rstrip().split('\t')
    if len(f)<5:continue
    address=int(f[0],16);size=int(f[1]);name=f[-1]
    procs.setdefault(address,[]).append((size,name))
    if name.startswith('country_core::render::nani::systems::nani_update_plaster'):
        functions.add(address)
    if 'ClearCanvasCmd' in name and 'AnimatedPlasterPeel' in name and ('run_unsafe<' in name or 'into_configs<' in name):
        query_records.append({'va':f[0],'size':size,'name':name})

result={'original_sha256':sha,'functions':{},'typed_query_records':query_records}
with (HERE/'clear-canvas-original.asm').open('w',encoding='utf8') as out:
    for address in sorted(functions):
        size=max(row[0] for row in procs[address])
        asm=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',
            f'--start-address={hex(address)}',f'--stop-address={hex(address+size)}',str(TARGET)],
            stdout=subprocess.PIPE,check=True).stdout.decode('utf8')
        out.write(asm)
        calls=[]
        for line in asm.splitlines():
            m=re.search(r'^([0-9a-f]+):.*callq\s+0x([0-9a-f]+)',line)
            if m:
                target=int(m.group(2),16)
                calls.append({'site':'0x'+m.group(1),'target':hex(target),
                    'pdb_aliases':[name for _,name in procs.get(target,[])]})
        result['functions'][hex(address)]={'size':size,'sha256':hashlib.sha256(read(address,size)).hexdigest(),
            'pdb_aliases':[name for _,name in procs[address]],'calls':calls}
starts=sorted(procs)
references=[]
for vs,start,rs,raw in sections:
    if start!=0x1000:continue
    code=data[raw:raw+rs]
    for match in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]....',code,re.S):
        va=0x140000000+start+match.start()
        target=va+7+struct.unpack_from('<i',match.group(),3)[0]
        if target not in [0x140a7abd0,0x140375310]:continue
        owner=starts[bisect.bisect_right(starts,va)-1]
        references.append({'site':hex(va),'target':hex(target),'containing_proc':hex(owner),
            'pdb_aliases':[name for _,name in procs[owner]]})
result['code_references']=references
raw_pointers=[]
for target in [0x140375310,0x140a7abd0]:
    needle=struct.pack('<Q',target)
    at=data.find(needle)
    while at>=0:
        for vs,start,rs,raw in sections:
            if raw<=at<raw+rs:
                site=0x140000000+start+at-raw
                values=[]
                for offset in range(-0x40,0x61,8):
                    value=struct.unpack('<Q',read(site+offset,8))[0]
                    values.append({'offset':offset,'value':hex(value),'proc_names':[n for _,n in procs.get(value,[])]})
                raw_pointers.append({'site':hex(site),'target':hex(target),'adjacent_qwords':values})
        at=data.find(needle,at+1)
result['raw_function_pointers']=raw_pointers
vtable_references=[]
candidate_bases=[int(p['site'],16)-0x50 for p in raw_pointers if p['target']=='0x140375310']
for vs,start,rs,raw in sections:
    if start!=0x1000:continue
    code=data[raw:raw+rs]
    for match in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]....',code,re.S):
        va=0x140000000+start+match.start();target=va+7+struct.unpack_from('<i',match.group(),3)[0]
        if target not in candidate_bases:continue
        owner=starts[bisect.bisect_right(starts,va)-1]
        vtable_references.append({'site':hex(va),'target':hex(target),'containing_proc':hex(owner),
            'pdb_aliases':[name for _,name in procs[owner]]})
result['candidate_vtable_base_references']=vtable_references
direct_callers=[]
for vs,start,rs,raw in sections:
    if start!=0x1000:continue
    code=data[raw:raw+rs]
    for match in re.finditer(rb'\xe8....',code,re.S):
        va=0x140000000+start+match.start();target=va+5+struct.unpack_from('<i',match.group(),1)[0]
        if target!=0x1403770b0:continue
        owner=starts[bisect.bisect_right(starts,va)-1]
        direct_callers.append({'site':hex(va),'target':hex(target),'containing_proc':hex(owner),
            'pdb_aliases':[name for _,name in procs[owner]]})
result['run_if_direct_callers']=direct_callers
result['registration_contract']={
    'run_unsafe':'0x140375310','system_vtable':'0x1429c8a80','run_unsafe_slot':'0x1429c8ad0',
    'run_if_builder':'0x1403770b0','run_if_call':'0x1405a0237',
    'set_value_site':'0x1405a023c','set_value':0x21,'in_set_call':'0x1405a026a',
    'schedule_call':'0x1405a031a','schedule_helper':'0x1403276f0',
    'schedule_pdb_aliases':[name for _,name in procs[0x1403276f0]],
    'condition_name_site':'0x1429a6b05',
    'condition_name':read(0x1429a6b05,150).split(b'>')[0].decode('ascii')+'>',
    'condition_vtable':'0x142939fa0','condition_run_unsafe':'0x1400d1c00',
    'condition_pdb_aliases':[name for _,name in procs[0x1400d1c00]],
    'code_range_start':'0x1405a0230','code_range_end_exclusive':'0x1405a031f',
    'code_range_sha256':hashlib.sha256(read(0x1405a0230,0xef)).hexdigest(),
}
registration=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',
    '--start-address=0x1405a0230','--stop-address=0x1405a031f',str(TARGET)],
    stdout=subprocess.PIPE,check=True).stdout.decode('utf8')
(HERE/'clear-canvas-registration-original.asm').write_text(registration,encoding='utf8')
(HERE/'clear-canvas-original.json').write_text(json.dumps(result,indent=2),encoding='utf8')
print(json.dumps({'function_ranges':{a:d['size'] for a,d in result['functions'].items()},
    'typed_query_records':len(query_records)},indent=2))
