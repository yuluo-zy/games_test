import bisect,hashlib,json,re
from pathlib import Path
import pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
OUT=Path(__file__).parent;ROOT=OUT.parents[2]
pe=pefile.PE('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne');base=pe.OPTIONAL_HEADER.ImageBase
symbols=json.loads((ROOT/'evidence/render-architecture/public-symbols.json').read_text(encoding='utf-8'))
starts=sorted({int(p['va'],16) for p in symbols if p.get('va')});targets=[]
for a in [0x142879de0,0x142879c20,0x142689ed0,0x142689b90]:
    unwind=[e.struct for e in pe.DIRECTORY_ENTRY_EXCEPTION if e.struct.BeginAddress==a-base]
    size=unwind[0].EndAddress-unwind[0].BeginAddress if unwind else starts[bisect.bisect_right(starts,a)]-a
    targets.append({'address':hex(a),'size':size,'range_basis':'B original .pdata runtime function' if unwind else 'B next-public bound','bytes_sha256':hashlib.sha256(pe.get_data(a-base,size)).hexdigest()})
job=[{'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':targets}},
 {'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':','.join(t['address'] for t in targets),'fields':'decompiled_code,callees,body,name'}}]
(OUT/'asset-job.json').write_text(json.dumps(job,indent=2)+'\n',encoding='utf-8')
desired={0x20B9E:'PrefabAudio-concrete_default',0x20BA0:'AudioTrigger-concrete_default',0x1E84A:'PrefabPart-concrete_default',0x20BD0:'Render-concrete_default',0x1A661:'Inheritable-concrete_default',0x20BA2:'Option-audio-finalization',0x20B9C:'PrefabAudio-inherit',0x20B9A:'PrefabRt-inherit'}
md=Cs(CS_ARCH_X86,CS_MODE_64);rows=[]
for module in [4091,4590]:
    text=(OUT/f'B-module-{module}.txt').read_text(encoding='utf-8');matches=list(re.finditer(r'^\s*(\d+) \| (S_[A-Z0-9_]+)[^\n]*',text,re.M))
    records={int(m[1]):(m[2],text[m.start():matches[i+1].start() if i+1<len(matches) else len(text)]) for i,m in enumerate(matches)}
    for record,(kind,body) in records.items():
        if kind!='S_INLINESITE':continue
        inline=re.search(r'inlinee = (0x[0-9a-fA-F]+)',body)
        if not inline or int(inline[1],16) not in desired:continue
        identity=int(inline[1],16);parent=int(re.search(r'parent = (\d+)',body)[1]);chain=[record]
        while records[parent][0] not in ['S_GPROC32','S_LPROC32']:
            chain.append(parent);parent=int(re.search(r'parent = (\d+)',records[parent][1])[1])
        chain.append(parent);proc=records[parent][1];section,offset,size=map(int,re.search(r'addr = (\d+):(\d+), code size = (\d+)',proc).groups());address=base+pe.sections[section-1].VirtualAddress+offset
        offsets=[int(x,16) for x in re.findall(r'code (?:end )?0x([0-9A-Fa-f]+)',body)];first,last=min(offsets),max(offsets)
        data=pe.get_data(address-base+first,last-first);name=desired[identity]+'-'+str(module)+'-'+str(record)
        (OUT/(name+'.asm.txt')).write_text('\n'.join(f'{i.address:#x}: {i.mnemonic} {i.op_str}' for i in md.disasm(data,address+first))+'\n',encoding='utf-8')
        rows.append({'interface':desired[identity],'ipi':hex(identity),'module':module,'record':record,'physical_name':re.search(r'`([^`]+)`',proc)[1],'address':hex(address),'range':[hex(address+first),hex(address+last)],'bytes_sha256':hashlib.sha256(data).hexdigest(),'assembly':name+'.asm.txt','annotation':body})
(OUT/'asset-inline-bindings.json').write_text(json.dumps(rows,indent=2)+'\n',encoding='utf-8')
print([(r['interface'],r['range']) for r in rows])
