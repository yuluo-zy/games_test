"""Bound original B symbol/inline ranges and persist focused SDK job inputs."""
import bisect,concurrent.futures,hashlib,json,re,subprocess
from pathlib import Path
import pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
OUT=Path(__file__).parent;ROOT=OUT.parents[2]
PDB=Path('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
def dump(module):
    cmd=['C:/Program Files/LLVM/bin/llvm-pdbutil.exe','dump','--symbols',f'--modi={module}',str(PDB)]
    result=subprocess.run(cmd,capture_output=True,text=True,encoding='utf-8',errors='replace')
    assert result.returncode==0,result.stderr
    path=OUT/f'B-module-{module}.txt';path.write_text(result.stdout,encoding='utf-8')
    return {'module':module,'command':cmd,'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'bytes':path.stat().st_size,'exit_code':result.returncode}
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
    reports=list(pool.map(dump,[183,390,432,567,762,1419,1452,3064,757,761]))
(OUT/'pdb-focus.json').write_text(json.dumps(reports,indent=2)+'\n',encoding='utf-8')
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase
symbols=json.loads((ROOT/'evidence/render-architecture/public-symbols.json').read_text(encoding='utf-8'))
starts=sorted({int(s['va'],16) for s in symbols if s.get('va')})
targets=[]
for a in [0x140ad1fe0,0x141208e10]:
    ranges=[e.struct for e in pe.DIRECTORY_ENTRY_EXCEPTION if e.struct.BeginAddress==a-base]
    size=ranges[0].EndAddress-ranges[0].BeginAddress if ranges else starts[bisect.bisect_right(starts,a)]-a
    targets.append({'address':hex(a),'size':size,'range_basis':'original B .pdata RuntimeFunction' if ranges else 'next-public upper bound; not exact author body','bytes_sha256':hashlib.sha256(pe.get_data(a-base,size)).hexdigest()})
job=[{'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':targets}},
     {'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':','.join(t['address'] for t in targets),'fields':'decompiled_code,callees,body,name'}}]
(OUT/'targets.json').write_text(json.dumps(targets,indent=2)+'\n',encoding='utf-8')
(OUT/'job.json').write_text(json.dumps(job,indent=2)+'\n',encoding='utf-8')
md=Cs(CS_ARCH_X86,CS_MODE_64);result=[]
desired={0x3739:'ComparativeSpace-lerp_clamp',0x19F18:'classify_side',0x6242:'regions_intersect',0x5F61:'map_slice',0x5F63:'LoopSlice-len',0x5F68:'LoopSlice-index',0xAF37:'Spline2D-find_segment',0x7EBF:'Rdp-stitch_with',0x114C0:'LoopSlice-next_back',0xAEEF:'BoundSpace-distance',0xAEF1:'BoundSpace-project',0xAEF3:'BoundSpace-project_clamp',0xAEF4:'BoundSpace-subtract',0xAEF0:'LoopSpace-project',0xAEF2:'LoopSpace-project_clamp',0x3A2A:'LoopSpace-project_unbounded',0x7B78:'LoopSpace-subtract'}
for module in [r['module'] for r in reports]:
    text=(OUT/f'B-module-{module}.txt').read_text(encoding='utf-8')
    matches=list(re.finditer(r'^\s*(\d+) \| (S_[A-Z0-9_]+)[^\n]*',text,re.M))
    records={int(m[1]):(m[2],text[m.start():matches[i+1].start() if i+1<len(matches) else len(text)]) for i,m in enumerate(matches)}
    for record,(kind,body) in records.items():
        if kind!='S_INLINESITE':continue
        inline=re.search(r'inlinee = (0x[0-9a-fA-F]+)',body)
        if not inline or int(inline[1],16) not in desired:continue
        identity=int(inline[1],16);parent=int(re.search(r'parent = (\d+)',body)[1]);chain=[record]
        while records[parent][0] not in ['S_GPROC32','S_LPROC32']:
            chain.append(parent);parent=int(re.search(r'parent = (\d+)',records[parent][1])[1])
        chain.append(parent);proc=records[parent][1]
        section,offset,size=map(int,re.search(r'addr = (\d+):(\d+), code size = (\d+)',proc).groups())
        address=base+pe.sections[section-1].VirtualAddress+offset
        offsets=[int(x,16) for x in re.findall(r'code (?:end )?0x([0-9A-Fa-f]+)',body)]
        if not offsets:continue
        first,last=min(offsets),max(offsets);data=pe.get_data(address-base+first,last-first)
        name=desired[identity]+'-'+str(module)+'-'+str(record)
        (OUT/(name+'.asm.txt')).write_text('\n'.join(f'{i.address:#x}: {i.mnemonic} {i.op_str}' for i in md.disasm(data,address+first))+'\n',encoding='utf-8')
        result.append({'interface':desired[identity],'ipi':hex(identity),'module':module,'inline_record':record,'ancestor_records':chain,'physical_procedure_record':parent,'physical_name':re.search(r'`([^`]+)`',proc)[1],'physical_address':hex(address),'physical_size':size,'inline_annotation':body.strip(),'enclosing_code_range':[hex(address+first),hex(address+last)],'enclosing_bytes_sha256':hashlib.sha256(data).hexdigest(),'assembly':name+'.asm.txt','range_limit':'annotations may be discontinuous; enclosing range contains dependency/consumer instructions; derive business from exact record/caller only'})
(OUT/'inline-bindings.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print('modules',[(r['module'],r['bytes']) for r in reports]);print('inline bindings',[(r['interface'],r['physical_address'],r['assembly']) for r in result])
