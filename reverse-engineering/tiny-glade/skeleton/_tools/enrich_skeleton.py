"""Enrich isolated PDB skeletons. Facts, candidates and unknowns stay distinct."""
from __future__ import annotations
from collections import defaultdict, Counter
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path, PurePosixPath
import re
import pickle
import subprocess
import sys
import yaml

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parent
LLVM = Path('/opt/homebrew/opt/llvm/bin')
START = '// === PDB ENRICHMENT V1 BEGIN ==='
END = '// === PDB ENRICHMENT V1 END ==='
MARKER = '// Generated from PDB metadata'

def sha(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
    return h.hexdigest()

def write_json(p,x):
    p.parent.mkdir(parents=True,exist_ok=True);tmp=p.with_suffix(p.suffix+'.tmp')
    tmp.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n');tmp.replace(p)

def write_gz(p,x):
    p.parent.mkdir(parents=True,exist_ok=True);tmp=p.with_suffix(p.suffix+'.tmp')
    with gzip.open(tmp,'wt',compresslevel=1)as f:json.dump(x,f,ensure_ascii=False,separators=(',',':'));f.write('\n')
    tmp.replace(p)

def canonical(s):
    out=[];depth=0
    for c in s:
        if c=='<':depth+=1
        elif c=='>':depth=max(0,depth-1)
        elif not depth:out.append(c)
    return ''.join(out)

def split_types(s):
    out=[];start=0;angle=paren=bracket=0
    for i,c in enumerate(s):
        if c=='<':angle+=1
        elif c=='>':angle-=1
        elif c=='(':paren+=1
        elif c==')':paren-=1
        elif c=='[':bracket+=1
        elif c==']':bracket-=1
        elif c==',' and angle==paren==bracket==0:out.append(s[start:i].strip());start=i+1
    if s[start:].strip():out.append(s[start:].strip())
    return out

def generic_nodes(s):
    # Preserve compiler spelling, including tuple$/enum2$/ref$ wrappers.
    result=[];stack=[]
    for i,c in enumerate(s):
        if c=='<':
            j=i-1
            while j>=0 and (s[j].isalnum()or s[j]in '_$:'):j-=1
            stack.append((j+1,i,s[j+1:i]))
        elif c=='>'and stack:
            begin,opened,base=stack.pop()
            if base:result.append({'type':s[begin:i+1],'constructor':base,'arguments':split_types(s[opened+1:i]),'evidence':'pdb_symbol_type_spelling','layout':'unknown'})
    return result

def function_pointers(s):
    result=[];pos=0
    while True:
        at=s.find('(*)(',pos)
        if at<0:break
        left=at-1
        while left>=0 and s[left].isspace():left-=1
        endreturn=left+1;angle=0
        while left>=0:
            c=s[left]
            if c=='>':angle+=1
            elif c=='<':
                if angle:angle-=1
                else:break
            elif c in ',('and angle==0:break
            left-=1
        start=left+1;ret=s[start:endreturn].strip();end=at+4;depth=1
        while end<len(s)and depth:
            if s[end]=='(':depth+=1
            elif s[end]==')':depth-=1
            end+=1
        if depth:break
        args=split_types(s[at+4:end-1])
        result.append({'spelling':s[start:end].strip(),'return_type_spelling':ret,'parameter_type_spellings':args,
            'rust_parameter_names':'unknown','rust_abi':'unknown','binding_to_game_function':'unknown','evidence':'function_pointer_type_embedded_in_generic_symbol'})
        pos=end
    return result

SYSTEM = {'Res':'resource_read','ResMut':'resource_write_capability','Query':'component_query','EventReader':'event_reader','EventWriter':'event_writer',
          'MessageReader':'message_reader','MessageWriter':'message_writer','With':'query_with_filter','Without':'query_without_filter',
          'In':'system_input','Local':'system_local','Commands':'deferred_command_interface','ParamSet':'parameter_set'}
def interfaces(s):
    result=[]
    for node in generic_nodes(s):
        leaf=node['constructor'].split('::')[-1]
        if leaf in SYSTEM:
            result.append({'kind':SYSTEM[leaf],'type':node['type'],'type_arguments':node['arguments'],'actual_runtime_usage':'unknown'})
    if 'bevy_ecs::system::commands::Commands'in s:result.append({'kind':'deferred_command_interface','type':'bevy_ecs::system::commands::Commands','actual_runtime_usage':'unknown'})
    return result

def body(d):return next((v for k,v in d.items()if k!='Kind'and isinstance(v,dict)),{})
def doc(row):
    d=row.get('decoded')
    if d is None:d=yaml.load(row.get('yaml',''),Loader=yaml.CSafeLoader)or{}
    # LLVM scalar strings such as "true" must not become YAML booleans.
    b=body(d)
    if any(k in b and not isinstance(b[k],str)for k in('Name','String','UniqueName')):
        raw=yaml.load(row.get('yaml',''),Loader=yaml.BaseLoader)or{};r=body(raw)
        for k in('Name','String','UniqueName'):
            if k in r:b[k]=r[k]
    return d

def safe_path(s):
    p=PurePosixPath(s)
    if p.is_absolute()or'..'in p.parts:raise ValueError(s)
    return p.as_posix()

spec=importlib.util.spec_from_file_location('evidence_pack','/Users/yuluo/.codex/skills/native-evidence-pack/scripts/collect.py')
pack=importlib.util.module_from_spec(spec);spec.loader.exec_module(pack)

class Enrichment:
    def __init__(self,v,baseline):
        self.v=v;self.base=baseline;self.out=ROOT/'_pdb'/v/'enrichment';self.out.mkdir(exist_ok=True)
        self.sources={r['path']:r for r in json.loads((ROOT/'_pdb'/v/'source-files.json').read_text())}
        self.original_sources=set(self.sources);self.procs={};self.by_rva=defaultdict(list);self.functions=defaultdict(dict)
        self.assigned=defaultdict(set);self.canonical_sources=defaultdict(set);self.file_candidates=defaultdict(dict)
        self.wrapper_types={};self.wrappers_by_module=defaultdict(list);self.references=defaultdict(list)
        self.type_mentions=defaultdict(list);self.files_by_module=defaultdict(set);self.checks_by_module={}
        self.stats=Counter();self.mapping=[];self.types={'IPI':{},'TPI':{}}
        self.closure_refs=defaultdict(list);self.inline_host_ids=set();self.symbolizer_raw=[]

    def ipi_name(self,idx):
        d=self.types['IPI'].get(idx,{});b=body(d);name=b.get('Name',f'IPI_0x{idx:X}')
        def string(index,seen=None):
            seen=set()if seen is None else seen
            if not index or index in seen:return ''
            seen.add(index);x=self.types['IPI'].get(index,{});y=body(x)
            if x.get('Kind')=='LF_SUBSTR_LIST':return ''.join(string(i,seen.copy())for i in y.get('StringIndices',[]))
            return string(y.get('Id',0),seen)+y.get('String','')
        scope=string(b.get('ParentScope',0));return (scope+'::'if scope else'')+str(name),b.get('FunctionType')

    def fid(self,name,tid):return hashlib.sha256(f'{self.v}\0{name}\0{tid}'.encode()).hexdigest()[:24]
    def fn(self,path,name,tid):
        key=self.fid(name,tid)
        if key not in self.functions[path]:
            self.functions[path][key]={'id':key,'name':name,'type_index':tid,'compiled_instances':[], 'inline_hosts':[], 'reference_callers':[],
              'candidate_interface_ids':set(),'closure_environment_references':[], 'source_mapping_evidence':[],
              'unknowns':{'author_parameter_names':'unknown','complete_rust_signature':'unknown','rust_abi':'unknown','body':'unknown','capture_field_names':'unknown','capture_offsets':'unknown','type_layout':'unknown','indirect_callers':'unknown'},
              'generic_type_relations':generic_nodes(name),'parameter_type_candidates':function_pointers(name),'system_interfaces_from_own_symbol':interfaces(name)}
            f=self.functions[path][key];can=canonical(name)
            m=re.search(r'::closure\$(\d+)',can)
            f['closure']={'kind':'closure','index':int(m[1]),'lexical_owner':can[:m.start()],'captures':'unknown'}if m else None
            self.canonical_sources[can].add(path)
        return self.functions[path][key]

    def load_sources(self):
        for path,r in self.sources.items():
            for mi in r.get('modules',[]):self.files_by_module[mi].add(path)
            with gzip.open(ROOT/self.v/r['metadata_file'],'rb')as f:
                for line in f:
                    if b'"class":"procedure"'in line:
                        x=json.loads(line);tid=x.get('resolved_tpi',x['type_index']);fn=self.fn(path,x['name'],tid)
                        pid=f"{x['module']}:{x['record_offset']}";self.assigned[pid].add(path)
                        fn['compiled_instances'].append({'procedure_id':pid,'rva':x['rva'],'va':x['va'],'source_line':x.get('source_line'),'mapping':'pdb_line_table_overlap'})
                    elif b'"class":"inline_site"'in line:
                        x=json.loads(line);fn=self.fn(path,x['name'],x.get('type_index'))
                        host=x.get('physical_procedure');hostid=f"{x['module']}:{host['record_offset']}"if host else None
                        if hostid:self.inline_host_ids.add(hostid)
                        fn['inline_hosts'].append({'kind':'inlined_into','host_procedure_id':hostid,'host_name':host.get('name')if host else None,
                            'host_rva':host.get('rva')if host else None,'module':x['module'],'inline_record_offset':x['record_offset'],'inlinee':x['inlinee'],
                            'source_line':x.get('source_line'),'annotation_data':x['annotations_hex'],'evidence':'pdb_inlinesite','is_standalone_call':False})
                    elif b'"event":"inlinee"'in line:
                        x=json.loads(line);self.fn(path,x['name'],x.get('type_index'))
        with gzip.open(ROOT/'_pdb'/self.v/'types.jsonl.gz','rt')as f:
            for line in f:
                x=json.loads(line);self.types[x['namespace']][x['index']]=doc(x)

    def load_procedures(self):
        for p in sorted((ROOT/'_pdb'/self.v/'modules').glob('*.jsonl.gz')):
            checks=[];mi=None
            with gzip.open(p,'rb')as f:
                for line in f:
                    if b'"event":"checksum"'in line:checks.append(json.loads(line))
                    elif b'"class":"procedure"'in line:
                        x=json.loads(line);mi=x['module'];pid=f"{mi}:{x['record_offset']}";sections=self.base['pe']['sections']
                        if not 1<=x['section']<=len(sections):continue
                        x['rva']=sections[x['section']-1]['rva']+x['section_offset'];x['procedure_id']=pid
                        x['source_files']=sorted(self.assigned[pid]);self.procs[pid]=x;self.by_rva[x['rva']].append(pid)
                        # Extract every candidate, including bool/float/aggregate return spellings.
                        pointers=function_pointers(x['name'])
                        if pointers:
                            ids=[]
                            for pointer in pointers:
                                cid=hashlib.sha256(pointer['spelling'].encode()).hexdigest()[:24];ids.append(cid)
                                if cid not in self.wrapper_types:self.wrapper_types[cid]={**pointer,'id':cid,'system_interfaces':interfaces(pointer['spelling']),'type_relations':generic_nodes(pointer['spelling']),'wrapper_procedures':[]}
                                self.wrapper_types[cid]['wrapper_procedures'].append({'procedure_id':pid,'module':mi,'name':x['name'],'rva':x['rva']})
                            x['candidate_interface_ids']=ids;self.wrappers_by_module[mi].append(pid)
                        for match in re.finditer(r'([A-Za-z_$][A-Za-z0-9_$]*(?:::[A-Za-z0-9_$]+)+)::closure_env\$(\d+)',x['name']):
                            self.closure_refs[match[1]+'::closure$'+match[2]].append({'kind':'closure_environment_type_reference','procedure_id':pid,'name':x['name'],'rva':x['rva'],'environment_type':match[0],'capture_layout':'unknown','is_call_edge':False})
            if mi is None:
                with gzip.open(p,'rt')as f:mi=json.loads(next(f))['index']
            self.checks_by_module[mi]=checks
        print(json.dumps({'version':self.v,'phase':'indexed','procedures':len(self.procs),'parameter_type_candidates':len(self.wrapper_types)},ensure_ascii=False),flush=True)

    def bind_artifacts(self):
        binary=Path(self.base['binary']['path']);debug=Path(self.base['pdb']['path'])
        assert sha(binary)==self.base['binary']['sha256'];assert sha(debug)==self.base['pdb']['sha256']
        pe=pack.PE(binary);identity=pack.pdb_identity(debug)
        assert any(x['guid']==identity['guid']and x['age']==identity['age']for x in pe.rsds)
        assert debug.parent==binary.parent and debug.name in {Path(x['pdb_name'].replace('\\','/')).name for x in pe.rsds}
        self.pe=pe;self.binding={'binary':self.base['binary'],'pdb':self.base['pdb'],'matching_rsds':True,'target_executed':False}

    def source_for_original(self,path,checks,allow_dependency=False):
        normalized=path.replace('\\','/')
        if 'country-slice-private/'in normalized:
            rel=normalized.split('country-slice-private/',1)[1]
            target='generated/'+rel if rel.startswith('target/')else rel
            kind='generated_source'if rel.startswith('target/')else'private_source'
        else:
            target='_references/sources/'+hashlib.sha256(path.encode()).hexdigest()[:16]+'/'+PurePosixPath(normalized).name
            kind='dependency_or_external_source_reference'
            if not allow_dependency:return None
        if not target.endswith('.rs'):return None
        target=safe_path(target)
        if target not in self.sources:
            matched=[c for c in checks if c.get('path')==path]
            self.sources[target]={'version':self.v,'path':target,'source_sha256':[c['checksum']for c in matched],
                'original_paths':[path],'modules':[],'kind':kind,'new_source_metadata_only':True}
        return target

    def namespace_candidates(self,name):
        can=canonical(name);hits=[]
        for path in self.original_sources:
            parts=path.split('/');src=parts.index('src')if'src'in parts else None
            if src is None:continue
            crate=parts[src-1]
            exceptions={'toolbar':'toolbar','forest':'forest','timberframe':'timberframe','tree-tool':'system_tree'}
            root=exceptions.get(crate,'system_'+crate.replace('-','_')if len(parts)>2 and parts[1]=='systems'else crate.replace('-','_'))
            tail=parts[src+1:];stem=tail[-1][:-3]
            namespace='::'.join([root]+tail[:-1]+([]if stem in('mod','lib','main')else[stem]))
            if can==namespace or can.startswith(namespace+'::'):hits.append((len(namespace),path))
        if not hits:return []
        length=max(x[0]for x in hits);return sorted(p for n,p in hits if n==length)

    def fill_missing_mappings(self):
        audit=json.loads((ROOT/'_pdb'/self.v/'unfilled-functions-audit.json').read_text())
        original_missing=audit.get('all_unplaced_private_namespace_procedures',audit.get('pre_mapping_fix_all_unplaced_private_namespace_procedures',audit['procedure_names_not_present_in_any_rs']))
        wanted={f"{x['module']}:{x['record_offset']}"for x in original_missing}
        queries=[]
        for pid in sorted(wanted):
            x=self.procs[pid]
            for off in {0,min(4,x['code_size']-1)}:queries.append((pid,x['rva']+off))
        argv=[str(LLVM/'llvm-symbolizer'),'--obj='+self.base['binary']['path'],'--relative-address','--output-style=JSON','--inlines','--no-debuginfod','--disable-gsym','--fallback-debug-path='+str(Path(self.base['pdb']['path']).parent)]
        run=subprocess.run(argv,input=''.join(hex(rva)+'\n'for _,rva in queries),capture_output=True,text=True,check=True)
        try:
            docs=json.loads(run.stdout)
            if isinstance(docs,dict):docs=[docs]
        except json.JSONDecodeError:
            docs=[json.loads(line)for line in run.stdout.splitlines()if line.strip()]
        assert len(docs)==len(queries)
        frames=defaultdict(list)
        for (pid,rva),d in zip(queries,docs):
            assert int(d['Address'],16)==rva;frames[pid].append(d);self.symbolizer_raw.append({'procedure_id':pid,'result':d})
        write_json(self.out/'symbolizer-missing-mappings.json',{'binding':self.binding,'argv':argv,'stderr':run.stderr,'records':self.symbolizer_raw})
        for pid in sorted(wanted):
            x=self.procs[pid];chosen=[];candidates=[];evidence=[];status='unknown'
            # ICF aliases can symbolize as a different function: never promote their file.
            for d in frames[pid]:
                syms=d.get('Symbol',[])
                if not syms:continue
                frame=syms[-1];symbol=frame.get('FunctionName','');file=frame.get('FileName','')
                stripped=re.sub(r'::h[0-9a-f]{16}(?: .*)?$','',symbol)
                compatible=canonical(stripped)==canonical(x['name'])
                if compatible and file and frame.get('Line',0)>0:
                    target=self.source_for_original(file,self.checks_by_module[x['module']],True)
                    if target:chosen.append(target);evidence.append({'kind':'symbolizer_physical_frame','file':file,'line':frame['Line'],'symbol':symbol,'address':d['Address']})
            chosen=sorted(set(chosen))
            if len(chosen)==1:status='pdb_symbolizer_source_attribution'
            else:
                chosen=[];candidates=self.namespace_candidates(x['name'])
                if len(candidates)==1:chosen=candidates;status='namespace_unique_source_candidate'
                else:
                    path=f"_unmapped/module-{x['module']:05d}.rs";chosen=[path]
                    self.sources.setdefault(path,{'version':self.v,'path':path,'source_sha256':[],'original_paths':[], 'kind':'unknown_source_module_bucket','new_source_metadata_only':True,'modules':[x['module']]})
                    status='unknown_source_preserved_in_module_bucket'
            tid=x['type_index']
            if x['type_namespace']=='IPI':_,tid=self.ipi_name(tid)
            x['source_files']=chosen;x['mapping_status']=status;self.assigned[pid].update(chosen)
            mapping={'procedure_id':pid,'name':x['name'],'source_files':chosen,'status':status,'candidate_files':candidates,'evidence':evidence,
                'author_definition_file_confirmed':False if status!='pdb_symbolizer_source_attribution'else None,
                'original_definition_file':'unknown'if status!='pdb_symbolizer_source_attribution'else'PDB code attribution; ICF caveats preserved'}
            self.mapping.append(mapping)
            for path in chosen:
                fn=self.fn(path,x['name'],tid);fn['compiled_instances'].append({'procedure_id':pid,'rva':x['rva'],'mapping':status})
                fn['source_mapping_evidence'].append(mapping);self.files_by_module[x['module']].add(path)
            self.stats[status]+=1
        write_json(self.out/'missing-function-mappings.json',self.mapping)

    def revise_weak_mappings(self):
        # Improve generated-source routing without treating escaped Rust public aliases
        # as exact author-definition names. Unambiguous symbolizer matches keep their status.
        frames=defaultdict(list)
        for row in self.symbolizer_raw:frames[row['procedure_id']].append(row['result'])
        for mapping in self.mapping:
            if mapping['status']not in('namespace_unique_source_candidate','unknown_source_preserved_in_module_bucket'):continue
            pid=mapping['procedure_id'];x=self.procs[pid];can=canonical(x['name'])
            prefix=can.split('::impl$',1)[0];method=can.split('::')[-1]
            candidates=[];ev=[]
            for record in frames[pid]:
                symbols=record.get('Symbol',[])
                if not symbols:continue
                frame=symbols[-1];display=frame.get('FunctionName','').replace('..','::')
                stripped=re.sub(r'::h[0-9a-f]{16}(?: .*)?$','',display)
                if prefix in stripped and stripped.endswith('::'+method)and frame.get('FileName')and frame.get('Line',0)>0:
                    path=self.source_for_original(frame['FileName'],self.checks_by_module[x['module']],True)
                    if path:
                        candidates.append(path);ev.append({'kind':'symbolizer_namespace_and_method_candidate','symbol':frame['FunctionName'],'file':frame['FileName'],'line':frame['Line'],'address':record['Address']})
            candidates=sorted(set(candidates));new_status=None;chosen=None
            if len(candidates)==1:chosen=candidates;new_status='pdb_symbolizer_namespace_source_candidate'
            elif all(p.endswith('/src/lib.rs')or p.endswith('/src/main.rs')for p in mapping['source_files']):
                path=f"_unmapped/module-{x['module']:05d}.rs";chosen=[path];new_status='unknown_source_preserved_in_module_bucket'
                self.sources.setdefault(path,{'version':self.v,'path':path,'source_sha256':[],'original_paths':[],'kind':'unknown_source_module_bucket','new_source_metadata_only':True,'modules':[x['module']]})
            if chosen is None:continue
            old_files=list(mapping['source_files'])
            for path in old_files:
                for key,fn in list(self.functions[path].items()):
                    if any(r['procedure_id']==pid for r in fn['source_mapping_evidence']):
                        fn['source_mapping_evidence']=[r for r in fn['source_mapping_evidence']if r['procedure_id']!=pid]
                        fn['compiled_instances']=[r for r in fn['compiled_instances']if r['procedure_id']!=pid]
                        if not fn['compiled_instances']and not fn['inline_hosts']:del self.functions[path][key]
            mapping.update({'source_files':chosen,'status':new_status,'evidence':ev,'author_definition_file_confirmed':False,'original_definition_file':'unknown; bounded source attribution candidate'})
            x['source_files']=chosen;x['mapping_status']=new_status;self.assigned[pid]=set(chosen)
            tid=x['type_index']
            if x['type_namespace']=='IPI':_,tid=self.ipi_name(tid)
            for path in chosen:
                fn=self.fn(path,x['name'],tid);fn['source_mapping_evidence'].append(mapping);fn['compiled_instances'].append({'procedure_id':pid,'rva':x['rva'],'mapping':new_status})
                self.files_by_module[x['module']].add(path)
        write_json(self.out/'missing-function-mappings.json',self.mapping)
        for path in self.sources:self.functions.setdefault(path,{})

    def collect_native_references(self):
        ranges={}
        for x in self.procs.values():
            if x['code_size']==0:continue
            try:s=self.pe.section_for(x['rva'],x['code_size'],executable=True)
            except pack.PackError:continue
            # Same folded entry is scanned once at its largest PDB size; all aliases remain.
            old=ranges.get(x['rva'])
            if old is None or old[2]<x['code_size']:ranges[x['rva']]=(x['rva'],s['offset']+x['rva']-s['rva'],x['code_size'])
        rangefile=self.out/'native-ranges.txt';rangefile.write_text(''.join(f'{a} {b} {c}\n'for a,b,c in sorted(ranges.values())))
        coverage={};completion=None
        recordfile=self.out/'native-reference-records.jsonl.gz';reportfile=self.out/'native-reference-report.json'
        reuse=False
        if recordfile.exists()and reportfile.exists():
            saved=json.loads(reportfile.read_text())
            reuse=saved['binding']==self.binding and saved['scanner_source_sha256']==sha(ROOT/'_tools/scan_refs.cpp')and saved['scanner_binary_sha256']==sha(ROOT/'_tools/scan_refs')and saved.get('records_sha256')==sha(recordfile)
        archive=gzip.open(recordfile,'wt',compresslevel=1)if not reuse else None
        with (self.out/'native-reader.log').open('ab')as log:
            p=subprocess.Popen([str(ROOT/'_tools/scan_refs'),self.base['binary']['path'],str(rangefile)],stdout=subprocess.PIPE,stderr=log,text=True)if not reuse else None
            stream=p.stdout if p else gzip.open(recordfile,'rt')
            try:
                for line in stream:
                    row=json.loads(line)
                    if archive:archive.write(line)
                    if row['event']=='coverage':coverage[row['entry_rva']]=row
                    elif row['event']=='complete':completion=row
                    else:
                        # Re-check emitted bytes and relative operands against the original PE.
                        raw=bytes.fromhex(row['bytes']);assert self.pe.read_rva(row['instruction_rva'],len(raw))==raw
                        if row['target_rva'] is not None:
                            width=1 if row['kind']=='direct_jump_outside_procedure'and row['llvm_opcode'].startswith('JMP_1')else 4
                            delta=int.from_bytes(raw[-width:],'little',signed=True)
                            assert row['instruction_rva']+len(raw)+delta==row['target_rva']
                            if row['target_rva']in self.by_rva:
                                callers=self.by_rva.get(row['caller_entry_rva'],[])
                                row['caller_alias_group_rva']=row['caller_entry_rva']
                                row['evidence']='original_binary_decoded_reachable_transfer';row['runtime_observed']=False
                                row['target_alias_group_rva']=row['target_rva']
                                self.references[row['target_rva']].append(row)
                        self.stats[row['kind']]+=1
                if p and p.wait():raise RuntimeError(f'native reference scan failed {p.returncode}')
            except BaseException:
                if p:p.terminate();p.wait()
                raise
            finally:
                if archive:archive.close()
                if not p:stream.close()
        assert completion and completion['ranges']==len(ranges)
        self.native_coverage=coverage
        write_json(self.out/'native-reference-report.json',{'binding':self.binding,'complete':completion,'kind_counts':dict(self.stats),
            'scanner_source_sha256':sha(ROOT/'_tools/scan_refs.cpp'),'scanner_binary_sha256':sha(ROOT/'_tools/scan_refs'),
            'records_sha256':sha(recordfile),'native_record_cache_reused':reuse,
            'coverage_scope':'CFG traversal from PDB procedure entries; returns and unconditional transfers end blocks; unresolved switch/indirect jumps are not guessed',
            'caller_identity':'All PDB aliases retained. A static decoded transfer is not runtime-observed execution.',
            'ranges_with_decode_gaps':sum(bool(x['decode_gaps'])for x in coverage.values()),'ranges_with_indirect_jumps':sum(bool(x['unresolved_indirect_jumps'])for x in coverage.values())})
        print(json.dumps({'version':self.v,'phase':'references','ranges':len(ranges),'direct_calls':self.stats['direct_call'],'indirect_calls_unknown':self.stats['indirect_call_unknown']},ensure_ascii=False),flush=True)

    def connect(self):
        for path,fs in self.functions.items():
            mods={mi for mi,files in self.files_by_module.items()if path in files}
            for mi in mods:
                for pid in self.wrappers_by_module.get(mi,[]):
                    x=self.procs[pid]
                    for cid in x['candidate_interface_ids']:
                        self.file_candidates[path][cid]={'candidate_id':cid,'association':'same_compiland_source_context_only','binding_to_specific_game_function':'unknown'}
            for fn in fs.values():
                fn['candidate_interface_evidence']=[]
                seen=set()
                for compiled in fn['compiled_instances']:
                    for ref in self.references.get(compiled['rva'],[]):
                        key=(ref['instruction_rva'],ref['target_rva'],ref['kind'])
                        if key not in seen:fn['reference_callers'].append(ref);seen.add(key)
                        if ref['kind']=='direct_call':
                            for pid in self.by_rva.get(ref['caller_alias_group_rva'],[]):
                                caller=self.procs[pid]
                                for cid in caller.get('candidate_interface_ids',[]):
                                    fn['candidate_interface_ids'].add(cid)
                                    evidence={'candidate_id':cid,'association':'native_direct_generic_caller','caller_procedure_id':pid,
                                        'call_instruction_rva':ref['instruction_rva'],'callee_entry_rva':ref['target_rva'],
                                        'binding_to_complete_function_signature':'candidate_only','caller_alias_group_rva':ref['caller_alias_group_rva'],
                                        'callee_alias_group_rva':ref['target_alias_group_rva'],'same_address_aliases_not_disambiguated':True}
                                    fn['candidate_interface_evidence'].append(evidence)
                                    self.file_candidates[path][cid]={'candidate_id':cid,'association':'native_direct_generic_caller','binding_to_specific_game_function':'candidate_only'}
                for inline in fn['inline_hosts']:
                    host=self.procs.get(inline['host_procedure_id'])
                    if host:
                        inline['host_source_files']=host['source_files']
                        for cid in host.get('candidate_interface_ids',[]):
                            fn['candidate_interface_ids'].add(cid)
                            self.file_candidates[path][cid]={'candidate_id':cid,'association':'physical_generic_wrapper_contains_game_inline','binding_to_complete_function_signature':'candidate_only'}
                fn['closure_environment_references']=self.closure_refs.get(canonical(fn['name']),[])
                for ref in fn['closure_environment_references']:
                    wrapper=self.procs[ref['procedure_id']]
                    for cid in wrapper.get('candidate_interface_ids',[]):fn['candidate_interface_ids'].add(cid)
                fn['candidate_interface_ids']=sorted(fn['candidate_interface_ids'])
                fn['tpi_type_record']=self.types['TPI'].get(fn['type_index'])
                fn['complete_signature_status']='unknown; PDB process type placeholder does not establish Rust parameters or ABI'
        write_gz(self.out/'candidate-interfaces.json.gz',self.wrapper_types)
        write_gz(self.out/'procedure-index.json.gz',list(self.procs.values()))
        write_gz(self.out/'address-aliases.json.gz',{str(rva):[{'procedure_id':pid,'name':self.procs[pid]['name'],'source_files':self.procs[pid]['source_files']}for pid in ids]for rva,ids in self.by_rva.items()})

    def render(self):
        report_files=[]
        for path,fs in sorted(self.functions.items()):
            source=ROOT/self.v/path;source.parent.mkdir(parents=True,exist_ok=True)
            suffix=''
            if source.exists():
                text=source.read_text()
                if text and not text.startswith(MARKER):raise RuntimeError(f'Refuse overwrite manual source {source}')
                if START in text:
                    prefix,old=text.split(START,1)
                    if END not in old:raise RuntimeError(f'Unclosed enrichment block {source}')
                    _,suffix=old.split(END,1)
                    text=prefix
                text=text.rstrip()+'\n\n'
            else:
                text=f'// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.\n// Version: {self.v}\n// Skeleton path: {path}\n// Original source: '+', '.join(self.sources[path].get('original_paths',[]))+'\n// 函数体、未记录参数名和布局 = unknown。\n\n'
            blocks=[START,'// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。',
                '// 完整信息: '+source.name+'.enriched.json.gz',
                '// 引用方全部同地址别名: _pdb/'+self.v+'/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。',
                '// 文件级接口候选来自同一编译单元，不等于具体函数签名。',
                f'// 文件级候选接口: {len(self.file_candidates[path])}; 完整候选见附档及 _pdb/{self.v}/enrichment/candidate-interfaces.json.gz', '']
            file_candidate_data=[]
            for association in self.file_candidates[path].values():
                cid=association['candidate_id'];c=self.wrapper_types[cid]
                item={**association,'function_type_spelling':c['spelling'],'parameter_type_spellings':c['parameter_type_spellings'],
                    'return_type_spelling':c['return_type_spelling'],'system_interfaces':c['system_interfaces'],'type_relations':c['type_relations'],
                    'parameter_names':'unknown','actual_function_binding':'unknown','layout':'unknown'}
                file_candidate_data.append(item)
            for c in file_candidate_data[:8]:
                blocks.append('// [文件级接口候选，具体函数归属 unknown] '+c['function_type_spelling'])
                blocks.append('// 候选参数类型: '+json.dumps(c['parameter_type_spellings'],ensure_ascii=False))
                for i in c['system_interfaces']:blocks.append('// [候选系统接口] '+i['kind']+': '+i['type'])
            if len(file_candidate_data)>8:blocks.append('// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。')
            blocks.append('')
            for fn in sorted(fs.values(),key=lambda x:x['name']):
                blocks += ['// ------------------------------------------------------------------','// 函数: '+fn['name'],'// 信息 ID: '+fn['id'],
                    '// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。']
                for pointer in fn['parameter_type_candidates']:
                    blocks+=['// [符号内函数类型候选] '+pointer['spelling'],'// 参数类型拼写: '+json.dumps(pointer['parameter_type_spellings'],ensure_ascii=False)]
                if fn['candidate_interface_ids']:
                    for cid in fn['candidate_interface_ids'][:8]:
                        candidate=self.wrapper_types[cid]
                        blocks.append('// [泛型宿主/闭包/直接引用方参数候选] '+cid+': '+candidate['spelling'])
                    if len(fn['candidate_interface_ids'])>8:blocks.append('// 其余候选接口见完整附档，共 '+str(len(fn['candidate_interface_ids']))+' 条。')
                elif not fn['parameter_type_candidates']:blocks.append('// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。')
                for interface in fn['system_interfaces_from_own_symbol']:blocks.append('// [系统接口类型拼写] '+interface['kind']+': '+interface['type'])
                for rel in fn['generic_type_relations']:blocks.append('// [类型关系] '+rel['constructor']+' -> '+json.dumps(rel['arguments'],ensure_ascii=False)+'; 字段布局 = unknown。')
                if fn['closure']:blocks.append('// [闭包] '+json.dumps(fn['closure'],ensure_ascii=False))
                for ref in fn['closure_environment_references'][:8]:blocks.append('// [闭包环境类型引用] '+ref['environment_type']+' <- '+ref['name']+'; 捕获字段/偏移 = unknown。')
                refs=fn['reference_callers'];inline=fn['inline_hosts'];closure=fn['closure_environment_references']
                blocks.append(f'// 引用方: 原程序直接转移 {len(refs)} 条；内联宿主 {len(inline)} 条；闭包环境类型引用 {len(closure)} 条。')
                for ref in refs[:16]:
                    ids=self.by_rva.get(ref['caller_alias_group_rva'],[])
                    aliases=' | '.join(self.procs[i]['name']for i in ids[:4])or'unknown'
                    if len(ids)>4:aliases+=f' | 其余 {len(ids)-4} 个同地址别名见共享索引'
                    blocks.append('// [静态直接引用] '+aliases+' @ '+hex(ref['instruction_rva'])+' -> '+hex(ref['target_rva'])+' ('+ref['kind']+'); 同地址别名未裁定。')
                for ref in inline[:8]:blocks.append('// [PDB 内联宿主] '+str(ref['host_name'])+f"; module={ref['module']}, inline_record={ref['inline_record_offset']}; 不是独立调用边。")
                if len(refs)>16 or len(inline)>8:blocks.append('// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。')
                if not refs:blocks.append('// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。')
                blocks.append('// 间接调用方、运行时实际调用和完整调度关系 = unknown。')
                for mapping in fn['source_mapping_evidence']:blocks.append('// [文件映射] '+mapping['status']+'; 作者定义文件是否精确确认仍按证据状态。')
                blocks.append('')
            blocks.append(END);source.write_text(text+'\n'.join(blocks)+'\n'+suffix.lstrip('\n'))
            payload={'version':self.v,'binding':self.binding,'source':self.sources[path],'functions':list(fs.values()),'file_interface_candidates':file_candidate_data,
                'interface_catalog':f'_pdb/{self.v}/enrichment/candidate-interfaces.json.gz','scope':'Metadata and static references only; no production integration or inferred implementation'}
            payload['caller_alias_catalog']=f'_pdb/{self.v}/enrichment/address-aliases.json.gz'
            write_gz(source.with_suffix('.rs.enriched.json.gz'),payload)
            summary={'path':path,'functions':len(fs),'candidate_interfaces':len(self.file_candidates[path]),'direct_reference_records':sum(len(x['reference_callers'])for x in fs.values()),
                'inline_host_records':sum(len(x['inline_hosts'])for x in fs.values()),'metadata_file':source.relative_to(ROOT).as_posix()+'.enriched.json.gz'}
            write_json(source.with_suffix('.rs.enrichment-summary.json'),summary);report_files.append(summary)
        write_json(self.out/'source-files.json',report_files)
        write_json(self.out/'report.json',{'version':self.v,'source_files_enriched':len(report_files),'function_entries':sum(x['functions']for x in report_files),
            'new_metadata_source_files':len(set(self.functions)-self.original_sources),'missing_function_records_routed':len(self.mapping),'mapping_status_counts':dict(Counter(x['status']for x in self.mapping)),
            'all_unknown_mappings_have_explicit_module_bucket':True,'candidate_interface_types':len(self.wrapper_types),
            'direct_reference_records_attached':sum(x['direct_reference_records']for x in report_files),'inline_hosts_attached':sum(x['inline_host_records']for x in report_files),
            'production_project_modified':False,'references_include_indirect_unknowns':True,'target_executed':False})

    def run(self):
        cache=self.out/'metadata-stage.pickle';meta=self.out/'metadata-stage.json'
        inputs={'source_files_sha256':sha(ROOT/'_pdb'/self.v/'source-files.json'),'pdb_sha256':self.base['pdb']['sha256'],'schema_version':1}
        if cache.exists()and meta.exists():
            known=json.loads(meta.read_text());assert known['inputs']==inputs;assert sha(cache)==known['cache_sha256']
            with cache.open('rb')as f:self.__dict__.update(pickle.load(f))
            # A delivery copy must never reuse the original directory as its write target.
            self.out=ROOT/'_pdb'/self.v/'enrichment'
            self.bind_artifacts()
            print(json.dumps({'version':self.v,'phase':'metadata_cache_reused'}),flush=True)
        else:
            self.load_sources();self.load_procedures();self.bind_artifacts();self.fill_missing_mappings()
            state={k:v for k,v in self.__dict__.items()if k!='pe'}
            with cache.open('wb')as f:pickle.dump(state,f,protocol=5)
            write_json(meta,{'inputs':inputs,'cache_sha256':sha(cache),'scope':'Trusted local intermediate only, do not load external pickle files'})
        self.revise_weak_mappings();self.collect_native_references();self.connect();self.render()
        assert sha(self.base['binary']['path'])==self.base['binary']['sha256'];assert sha(self.base['pdb']['path'])==self.base['pdb']['sha256']
        print(json.dumps(json.loads((self.out/'report.json').read_text()),ensure_ascii=False),flush=True)

def main():
    baselines=json.loads((PROJECT/'tracking/baselines.json').read_text())['baselines'];reports=[]
    for v in sys.argv[1:]or['A','B','C']:
        e=Enrichment(v,baselines[v]);e.run();reports.append(json.loads((e.out/'report.json').read_text()));del e
    reports=[json.loads((ROOT/'_pdb'/v/'enrichment/report.json').read_text())for v in 'ABC'if(ROOT/'_pdb'/v/'enrichment/report.json').exists()]
    write_json(ROOT/'pdb-enrichment-report.json',{'versions':reports,'production_project_modified':False,'scope':'Candidate type interfaces, PDB relationships, explicit file mapping gaps and original-binary static references'})

if __name__=='__main__':main()
