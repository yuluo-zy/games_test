from pathlib import Path
import json,re,subprocess
ROOT=Path('D:/game/ljxsj_92385/Tiny Glade');OUT=Path('D:/game/reverse-engineering/tiny-glade/evidence/pdb-structure')
LLVM=Path('C:/Program Files/LLVM/bin'); base=0x140001000
procedures=[];debug=[]
for file in OUT.glob('*-module*.txt'):
    text=file.read_text(encoding='utf-8-sig');lines=text.splitlines()
    for i,line in enumerate(lines):
        if 'S_GPROC32 ' in line or 'S_LPROC32 ' in line:
            m=re.search(r'`([^`]+)`',line);address=re.search(r'addr = (\d+):(\d+), code size = (\d+)',lines[i+1])
            if m and address:
                sec,off,size=map(int,address.groups())
                if any(k in m.group(1) for k in ['country_core','system_roof','system_wall_constructor','system_decorator']):
                    procedures.append({'name':m.group(1),'section':sec,'section_offset':off,'va':hex(base+off) if sec==1 else None,'code_size':size,'end_exclusive':hex(base+off+size) if sec==1 else None,'evidence':str(file),'line':i+1,'pdb_type_record':lines[i+2].strip()})
    debug.append({'file':str(file),'procedure_count':sum('S_GPROC32 ' in line or 'S_LPROC32 ' in line for line in lines),
                  'local_variable_record_count':sum('S_LOCAL ' in line for line in lines),'data_record_count':sum('S_GDATA32 ' in line or 'S_LDATA32 ' in line for line in lines),
                  'source_paths':[line for line in lines if 'country-slice-private/crates/' in line]})
(OUT/'procedures.json').write_text(json.dumps({'procedures':procedures,'module_debug_coverage':debug,'limitations':['CodeView code_size is an observed compiled code range; inlined routines may have no separate boundary','A void() CodeView type is not a recovered Rust signature','No field name or offset is inferred from stack size or object memory accesses']},ensure_ascii=False,indent=2),encoding='utf-8')
calls=[]
for scope in ['roof-create','history-finish']:
    dis=(OUT/(scope+'-full-disassembly.txt')).read_text(encoding='utf-8-sig')
    for i,line in enumerate(dis.splitlines()):
        m=re.search(r'^([0-9a-f]+):.*callq\s+(0x[0-9a-f]+)',line)
        if m:calls.append({'scope':scope,'callsite':hex(int(m.group(1),16)),'destination':m.group(2),'evidence_line':i+1,'public_symbols':[]})
destinations={int(x['destination'],16) for x in calls};roof_symbols=[]; active=None
p=subprocess.Popen([str(LLVM/'llvm-pdbutil.exe'),'dump','--publics',str(ROOT/'tiny_glade.pdb')],stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,encoding='utf-8',errors='replace')
for line in p.stdout:
    if 'S_PUB32' in line:active={'record':line.strip()}
    elif active is not None and 'addr =' in line:
        m=re.search(r'addr = (\d+):(\d+)',line)
        if m and int(m.group(1))==1:
            va=base+int(m.group(2));active['va']=hex(va);active['address_record']=line.strip()
            if va in destinations:
                for call in calls:
                    if int(call['destination'],16)==va:call['public_symbols'].append(active)
            if 'system_roof' in active['record'] and 'drop_in_place' not in active['record'] and len(roof_symbols)<100:roof_symbols.append(active)
        active=None
p.wait()
if p.returncode:raise RuntimeError(p.stderr.read())
for c in calls:
    symbols=c['public_symbols'];c['total_aliases']=len(symbols)
    symbols.sort(key=lambda s:0 if any(k in s['record'] for k in ['country_core','system_roof','system_decorator','system_wall_constructor']) else 1)
    c['public_symbols']=symbols[:8];c['omitted_aliases']=max(0,len(symbols)-8)
for scope,label,entry,size,end in [('roof-create','system_roof::create_roof::create_roof','0x1421aaea0',2075,'0x1421ab6bb'),('history-finish','country_core::resources::history::History::finish_potential_mutation','0x140a86980',420,'0x140a86b24')]:
    (OUT/(scope.replace('-create','').replace('-finish','')+'-call-candidates.json')).write_text(json.dumps({'function':label,'entry':entry,'code_size':size,'end_exclusive':end,'calls':[c for c in calls if c['scope']==scope],'limitations':['Static direct call sites only, not runtime call trace','Some sites are cleanup/unwind paths; graph sequencing and semantic roles unproven','Inlinees are not separately emitted calls','At most 8 aliases retained per target, game names prioritized; omitted count preserved','Symbol aliases at the same address do not establish unique semantics']},ensure_ascii=False,indent=2),encoding='utf-8')
(OUT/'roof-symbol-seeds.json').write_text(json.dumps({'symbols':roof_symbols,'scope':'Up to 100 non-drop system_roof public symbol records; no broad source reconstruction'},ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'procedures':len(procedures),'direct_calls':len(calls),'mapped_sites':sum(bool(c['public_symbols']) for c in calls),'module_coverage':[{k:v for k,v in x.items() if k!='source_paths'} for x in debug]},indent=2))
