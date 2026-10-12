"""Attach version-bound render-pass metadata to existing sources only.
PDB/API facts, SPIR-V declarations, mapping candidates and unknowns stay separate.
"""
from __future__ import annotations
import collections,gzip,hashlib,json,re,struct,sys
from pathlib import Path
from enrich_skeleton import generic_nodes
ROOT=Path(__file__).resolve().parents[1];PROJECT=ROOT.parent
START='// === RENDER PASSES V1 BEGIN ===';END='// === RENDER PASSES V1 END ==='
GRAMMAR=ROOT/'_tools/spirv.core.grammar.json'
G=json.loads(GRAMMAR.read_text());OPS={x['opcode']:x['opname'] for x in G['instructions']}
ENUMS={x['kind']:{int(e['value'],0) if isinstance(e['value'],str) else e['value']:e['enumerant'] for e in x.get('enumerants',[])} for x in G['operand_kinds']}
BASES=json.loads((PROJECT/'tracking/baselines.json').read_text())['baselines']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def jwrite(p,x):
 t=p.with_name(p.name+'.tmp');t.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n');t.replace(p)
def gzwrite(p,x):
 t=p.with_name(p.name+'.tmp')
 with gzip.open(t,'wt',compresslevel=1) as f:json.dump(x,f,ensure_ascii=False,separators=(',',':'))
 t.replace(p)
def gzload(p):return json.load(gzip.open(p,'rt'))
def string(words):
 b=struct.pack('<'+'I'*len(words),*words);pos=b.find(b'\0')
 if pos<0:raise ValueError('Unterminated SPIR-V string')
 return b[:pos].decode('utf-8'),(pos+4)//4

def reflect(payload):
 ws=struct.unpack('<'+'I'*(len(payload)//4),payload);assert ws[0]==0x07230203 and ws[4]==0
 names={};member_names=collections.defaultdict(dict);dec=collections.defaultdict(list);member_dec=collections.defaultdict(lambda:collections.defaultdict(list));types={};variables={};constants={};entries=[];modes=[];groups=[];inst=[];i=5
 while i<len(ws):
  n,op=ws[i]>>16,ws[i]&0xffff
  if not n or i+n>len(ws):raise ValueError(f'Instruction boundary {i}/{len(ws)}')
  args=list(ws[i+1:i+n]);name=OPS.get(op,'unknown_opcode_'+str(op));inst.append((name,args,i));i+=n
  if name=='OpName':names[args[0]]=string(args[1:])[0]
  elif name=='OpMemberName':member_names[args[0]][args[1]]=string(args[2:])[0]
  elif name=='OpDecorate':dec[args[0]].append({'decoration':ENUMS['Decoration'].get(args[1],str(args[1])),'operands':args[2:]})
  elif name=='OpMemberDecorate':member_dec[args[0]][args[1]].append({'decoration':ENUMS['Decoration'].get(args[2],str(args[2])),'operands':args[3:]})
  elif name in ('OpGroupDecorate','OpGroupMemberDecorate'):groups.append((name,args))
  elif name.startswith('OpType'):types[args[0]]={'opcode':name,'operands':args[1:]}
  elif name in ('OpConstant','OpSpecConstant'):
   constants[args[1]]={'type':args[0],'words':args[2:],'specialization':name=='OpSpecConstant'}
  elif name=='OpVariable':variables[args[1]]={'type':args[0],'storage_class':ENUMS['StorageClass'].get(args[2],str(args[2])),'initializer_id':args[3] if len(args)>3 else None}
  elif name=='OpEntryPoint':
   entry,used=string(args[2:]);entries.append({'execution_model':ENUMS['ExecutionModel'].get(args[0],str(args[0])),'function_id':args[1],'entry_name':entry,'interface_ids':args[2+used:]})
  elif name in ('OpExecutionMode','OpExecutionModeId'):modes.append({'entry_id':args[0],'mode':ENUMS['ExecutionMode'].get(args[1],str(args[1])),'operands':args[2:],'operand_kind':'id' if name.endswith('Id') else 'literal'})
 for name,args in groups:
  if name=='OpGroupDecorate':
   for target in args[1:]:dec[target]+=dec[args[0]]
  else:
   for target,member in zip(args[1::2],args[2::2]):member_dec[target][member]+=dec[args[0]]
 def type_doc(idx,seen=None):
  seen=set() if seen is None else seen
  if idx in seen:return {'id':idx,'status':'recursive_reference'}
  seen.add(idx);t=types.get(idx,{})
  result={'id':idx,'name':names.get(idx,'unknown'),'record':t,'decorations':dec[idx]}
  if t.get('opcode')=='OpTypeStruct':result['members']=[{'index':n,'name':member_names[idx].get(n,'unknown'),'type_id':tid,'decorations':member_dec[idx][n]} for n,tid in enumerate(t['operands'])]
  elif t.get('opcode')=='OpTypePointer':result['pointee']=type_doc(t['operands'][1],seen)
  elif t.get('opcode') in ('OpTypeArray','OpTypeRuntimeArray','OpTypeSampledImage'):result['element']=type_doc(t['operands'][0],seen)
  return result
 def deco_value(idx,key):
  return next((x['operands'][0] for x in dec[idx] if x['decoration']==key and x['operands']), 'unknown')
 def direction(var_id):
  v=variables[var_id];storage=v['storage_class'];t=types.get(v['type'],{});pointee=t.get('operands',[None,None])[1] if t.get('opcode')=='OpTypePointer' else v['type'];pt=types.get(pointee,{})
  visited=set()
  while pt.get('opcode') in ('OpTypeArray','OpTypeRuntimeArray') and pointee not in visited:
   visited.add(pointee);pointee=pt['operands'][0];pt=types.get(pointee,{})
  quals={x['decoration'] for x in dec[var_id]+dec[pointee]};raw_quals=sorted(quals)
  if 'NonWritable' in quals and 'NonReadable' not in quals:return 'declared_read_only',raw_quals
  if 'NonReadable' in quals and 'NonWritable' not in quals:return 'declared_write_only',raw_quals
  if storage=='Input':return 'stage_input',raw_quals
  if storage=='Output':return 'stage_output',raw_quals
  if storage=='PushConstant':return 'push_constant_input',raw_quals
  if storage=='Uniform' and 'BufferBlock' not in quals:return 'uniform_input',raw_quals
  if storage=='UniformConstant' and (pt.get('opcode') in ('OpTypeSampler','OpTypeSampledImage') or pt.get('opcode')=='OpTypeImage' and len(pt['operands'])>5 and pt['operands'][5]==1):return 'sampled_input',raw_quals
  if pt.get('opcode')=='OpTypeStruct':
   counts=[{x['decoration'] for x in member_dec[pointee][n]} for n in range(len(pt['operands']))]
   if counts and all('NonWritable' in c for c in counts):return 'all_members_declared_nonwritable',raw_quals
   if counts and all('NonReadable' in c for c in counts):return 'all_members_declared_nonreadable',raw_quals
  return 'unknown_read_write_direction',raw_quals
 descriptors=[];interfaces=[]
 for vid,v in variables.items():
  access,qualifiers=direction(vid);record={'variable_id':vid,'name':names.get(vid,'unknown'),'storage_class':v['storage_class'],'type':type_doc(v['type']),'descriptor_set':deco_value(vid,'DescriptorSet'),'binding':deco_value(vid,'Binding'),'location':deco_value(vid,'Location'),'builtin':deco_value(vid,'BuiltIn'),'declared_direction':access,'decorations':dec[vid],'access_qualifiers':qualifiers,'runtime_resource_handle':'unknown','actually_used_by_entry':'unknown; declaration only'}
  if record['binding']!='unknown' or v['storage_class'] in ('Uniform','UniformConstant','StorageBuffer','PushConstant'):descriptors.append(record)
  if v['storage_class'] in ('Input','Output'):interfaces.append(record)
 return {'spirv_version':hex(ws[1]),'generator':ws[2],'id_bound':ws[3],'instruction_count':len(inst),'entries':entries,'execution_modes':modes,'declared_resources':descriptors,'stage_interfaces':interfaces,'type_records':{str(k):{**v,'name':names.get(k,'unknown'),'decorations':dec[k],'member_names':member_names[k],'member_decorations':dict(member_dec[k])} for k,v in types.items()},'constants':constants,'limitations':['Declarations/access qualifiers do not prove actual entry usage','Descriptor slots do not identify same runtime resource across passes','Shader output variables are not recovered render-graph outputs','Specialization values/runtime dimensions and image layouts unknown']}

def shader_catalog(version):
 root=Path(BASES[version]['binary']['path']).parent/'compiled-shaders';records=[]
 for p in sorted(root.rglob('*')):
  if p.suffix not in ('.bin','.precomp') or not p.is_file():continue
  raw=p.read_bytes();cursor=0;stages=[];gaps=[];header_uri=None
  if p.suffix=='.precomp' and len(raw)>8:
   n=struct.unpack_from('<I',raw,4)[0]
   if n<len(raw)-8:
    try:
     candidate=raw[8:8+n].decode('utf-8')
     if '/' in candidate and ':' in candidate and candidate.isprintable():header_uri=candidate
    except UnicodeDecodeError:pass
  while True:
   at=raw.find(b'\x03\x02\x23\x07',cursor)
   if at<0:break
   cursor=at+4
   try:
    if at<4:raise ValueError('Missing word-count prefix')
    words=struct.unpack_from('<I',raw,at-4)[0]
    if words<5 or at+words*4>len(raw):raise ValueError('Invalid payload boundary')
    data=raw[at:at+words*4];stage=reflect(data);stage.update({'payload_offset':at,'payload_words':words,'payload_sha256':hashlib.sha256(data).hexdigest()});stages.append(stage);cursor=at+words*4
   except (ValueError,AssertionError,UnicodeDecodeError,IndexError,struct.error) as error:gaps.append({'offset':at,'error':str(error)})
  ident=hashlib.sha256((version+':'+p.relative_to(root).as_posix()).encode()).hexdigest()[:24]
  records.append({'id':ident,'version':version,'path':str(p),'relative_path':p.relative_to(root).as_posix(),'sha256':sha(p),'header_shader_uri':header_uri,'stages':stages,'parse_gaps':gaps,'uninterpreted_container_bytes':'retained in original file; wrapper binding metadata not inferred','mapping_status':'physical version-bound artifact; pass binding assessed separately'})
 return records

API_LEAF={'read_texture':'texture_read_api','write_texture':'texture_write_api','read_write_texture':'texture_read_write_api','read_buffer':'buffer_read_api','write_buffer':'buffer_write_api','read_write_buffer':'buffer_read_write_api','uniform':'uniform_input_api','uniform_buffer':'uniform_input_api','bind_shadowmap':'named_shadowmap_binding_api','bind_terrain_heightmap':'named_terrain_heightmap_binding_api','bind_terrain_lighting':'named_terrain_lighting_binding_api','get_or_create_temporal':'temporal_resource_api','get_or_create_temporal_pair':'temporal_pair_api','export_temporal_and_finish':'temporal_export_api','raster_read':'raster_read_api'}
FAMILIES={'gpu_generate_draw_lists':['nani_','cull_','prefix_scan_','draw_list_'],'generate_hzb':['hzb_'],'ssgi':['gtao_'],'ray_trace_gi':['gi_'],'eval_gi_screen_radiance':['gi_'],'reprojection':['reprojection_','reproject_'],'bin_lights':['light_binning_'],'restir_di':['restir_'],'deferred_raster':['deferred_'],'taa':['taa','tgsr'],'transparency_pass':['water_','clouds','smoke','window_glass'],'grass':['grass_'],'dof':['dof_'],'dof_autofocus':['dof_autofocus'],'apply_volumetrics':['volumetric_','apply_volumetrics'],'shadow_pass':['shadow_'],'instanced_roof':['roof_tile','roof'],'main_pass':[]}

def shader_hints(name):
 hints=[]
 if '::shader::' not in name:return hints
 # Compiler-generated typed shader entries and permutation keys are retained exactly.
 m=re.match(r'([\w:]+)::impl\$\d+::to_shader_permutation_key',name)
 if m:
  family=m[1].replace('::shader::','/').replace('::','/');hints.append({'shader_family':family,'entry':'unknown','basis':'PDB_shader_permutation_type','symbol':name})
 for node in generic_nodes(name):
  if node['constructor'].endswith('::bind_entry_point'):
   value=node['arguments'][0] if node['arguments'] else ''
   head=value.split('<',1)[0]
   if '::shader::' in head and '::' in head:
    parent,entry=head.rsplit('::',1);family=parent.replace('::shader::','/').replace('::','/');hints.append({'shader_family':family,'entry':entry,'basis':'PDB_typed_bind_entry_point','symbol':name,'type_argument':value})
 return hints

class Version:
 def __init__(self,v):
  self.v=v;self.shaders=shader_catalog(v);self.by_shader={s['id']:s for s in self.shaders};self.procs=gzload(ROOT/'_pdb'/v/'enrichment/procedure-index.json.gz');self.by_name=collections.defaultdict(list);self.by_rva=collections.defaultdict(list)
  for p in self.procs:self.by_name[p['name']].append(p);self.by_rva[p.get('rva')].append(p)
  files=set((ROOT/v/'crates/country-core/src/systems/render_loop').rglob('*.rs'))
  files.update(p for p in [(ROOT/v/'crates/tiny-glade/src/render_loop.rs'),(ROOT/v/'crates/tiny-glade/src/run_game.rs')] if p.exists())
  if v=='C':files.update((ROOT/v/'rhapsody2/render-std').rglob('*.rs'))
  self.files={p.relative_to(ROOT/v).as_posix():p for p in sorted(files)};self.payloads={};self.rva_files=collections.defaultdict(set);self.incoming=collections.defaultdict(list);self.outgoing=collections.defaultdict(list);self.sequences=collections.defaultdict(list)
  self.common_inputs=[{'path':str(ROOT/'_pdb'/v/'enrichment'/name),'sha256':sha(ROOT/'_pdb'/v/'enrichment'/name)} for name in ['procedure-index.json.gz','native-reference-records.jsonl.gz']]
  self.alias_cache={}
 def aliases(self,rva):
  if rva in self.alias_cache:return self.alias_cache[rva]
  names=list(dict.fromkeys(p['name'] for p in self.by_rva.get(rva,[])))
  result={'alias_group_rva':rva,'count':len(names),'display_examples':[n.split('<',1)[0] for n in names[:3]],'complete_catalog':f'_pdb/{self.v}/render-address-aliases.json.gz','exclusive_symbol':'unknown'}
  self.alias_cache[rva]=result;return result
 def collect(self):
  for rel,p in self.files.items():
   dep_path=p.with_suffix('.rs.dependencies.json');deps=json.loads(dep_path.read_text()) if dep_path.exists() else {'functions':[]};dep_by={f['name']:f for f in deps['functions']};text=p.read_text();names=list(dict.fromkeys(re.findall(r'^// 函数符号: (.*)',text,re.M)));items=[]
   for name in names:
    procs=self.by_name[name];rawdeps=dep_by.get(name,{}).get('dependencies',[]);resource_ops=[];hints=[]
    for dep in rawdeps:
     symbol=dep['function'];head=symbol.split('<',1)[0];leaf=head.rsplit('::',1)[-1];action=API_LEAF.get(leaf)
     if action or head.startswith(('rhapsody::rg::node_builder::','rhapsody::rg::pass_builder::')) and leaf in ('read','write','access'):
      action=action or 'graph_resource_'+leaf+'_api'
      resource_ops.append({'role':action,'symbol':symbol,'relationship':dep['relationship'],'evidence':dep['first_evidence'],'resource_instance':'unknown','source_level_arguments':'unknown','scope':'PDB inline or physical native dependency; node identity/dataflow not inferred'})
     for hint in shader_hints(symbol):hints.append({**hint,'relationship':dep['relationship'],'evidence':dep['first_evidence']})
    # Typed helpers physically associated to a source file are context evidence, not exact binding to other functions.
    for hint in shader_hints(name):hints.append({**hint,'relationship':'PDB_source_symbol_context','evidence':{'source':rel}})
    procdata=[{'module':q['module'],'record_offset':q.get('record_offset'),'rva':q.get('rva'),'code_size':q.get('code_size'),'name':q['name']} for q in procs]
    # Generic/library symbols appearing in a source context do not become Pass entrypoints.
    own=name.startswith(('country_core::systems::render_loop::','tiny_glade::render_loop::','tiny_glade::run_game::','rhapsody_gi::','rhapsody_taa::','rhapsody_dof::','rhapsody_shadow::','rhapsody_post::','rhapsody_core::'))
    leaf=name.split('<',1)[0].rsplit('::',1)[-1]
    if own and leaf not in ('default','fmt','hash','eq','clone','as_compute_pipeline','as_raster_pipeline'):
     for q in procs:
      if q.get('rva') is not None:self.rva_files[q['rva']].add(rel)
    items.append({'symbol':name,'physical_procedures':procdata,'resource_api_evidence':resource_ops,'shader_type_hints':hints,'parameters':'unknown unless explicitly recorded by PDB; void() placeholder is not a signature','inputs':'unknown concrete pass arguments/resource handles','outputs':'unknown concrete returned resources','gpu_execution_order':'unknown'})
   file_kind='game_compute_pass_context' if '/compute_passes/' in rel else 'game_raster_pass_context' if '/draw_passes/' in rel else 'renderer_library_pass_context' if '/render-std/' in rel else 'frame_or_auxiliary_context'
   self.payloads[rel]={'version':self.v,'source':rel,'file_role':file_kind,'functions':items,'input_output_status':'API action evidence and shader declaration interfaces below; complete runtime input/output unknown','resource_dependency_edges':'unknown runtime resource identity/producer-consumer edges','shader_mappings':[],'shader_interfaces':[],'native_incoming':[],'native_outgoing':[],'parent_callsite_order':[],'unknowns':['exact graph-node instances and resource versions','runtime branch enable conditions','unresolved indirect calls','GPU execution schedule and queue ownership','formats/dimensions/layout transitions of runtime images','shader permutations selected per frame']}
  for line in gzip.open(ROOT/'_pdb'/self.v/'enrichment/native-reference-records.jsonl.gz','rt'):
   ref=json.loads(line);src=ref.get('caller_entry_rva');dst=ref.get('target_rva');source_files=self.rva_files.get(src,set());target_files=self.rva_files.get(dst,set())
   if not source_files and not target_files:continue
   record={**ref,'caller_aliases':self.aliases(src),'target_aliases':self.aliases(dst),'caller_source_candidates':sorted(source_files),'target_source_candidates':sorted(target_files),'source_mapping_status':'PDB source attribution and same-address aliases; exclusive ownership unknown','runtime_executed':False}
   for path in source_files:self.outgoing[path].append(record)
   for path in target_files:self.incoming[path].append(record)
   # Only decoded callsites, grouped within their physical parent; no fabricated global ordering.
   # A folded address can be shared with thousands of unrelated RET helpers.
   # Keep every transfer in incoming/outgoing, but do not assert a Pass ordering edge.
   record['pass_target_attribution']='unique_PDB_target_name' if record['target_aliases']['count']==1 else 'unknown_same_address_alias_target; not a confirmed call to this Pass'
   if target_files and record['target_aliases']['count']==1:self.sequences[src].append(record)
  for src,records in self.sequences.items():
   records.sort(key=lambda r:r['instruction_rva'])
   for rank,record in enumerate(records):
    for path in record['target_source_candidates']:
     position={'caller_entry_rva':src,'caller_aliases':record['caller_aliases'],'callsite_rva':record['instruction_rva'],'position_among_mapped_render_targets':rank+1,'mapped_render_target_count':len(records),'previous_callsite':records[rank-1]['instruction_rva'] if rank else None,'next_callsite':records[rank+1]['instruction_rva'] if rank+1<len(records) else None,'previous_target_files':records[rank-1]['target_source_candidates'] if rank else [],'next_target_files':records[rank+1]['target_source_candidates'] if rank+1<len(records) else [],'ordering_status':'instruction_address_order_only; branches/loops/GPU schedule unknown','kind':record['kind']}
     self.payloads[path]['parent_callsite_order'].append(position)
  for rel,payload in self.payloads.items():
   payload['native_incoming']=self.incoming[rel];payload['native_outgoing']=self.outgoing[rel];mapping={};stem=Path(rel).stem
   for fn in payload['functions']:
    for hint in fn['shader_type_hints']:
     for shader in self.shaders:
      uri=shader['header_shader_uri']
      if uri and uri.split(':',1)[0]==hint['shader_family'] and (hint['entry']=='unknown' or uri.rsplit(':',1)[-1]==hint['entry']):
       key=(shader['id'],hint['symbol'],fn['symbol']);mapping[key]={'shader_id':shader['id'],'relative_path':shader['relative_path'],'sha256':shader['sha256'],'status':'PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown','host_symbol':fn['symbol'],'hint':hint,'shader_uri':uri}
   # A/B binaries lack typed identity links. Filename correspondence is only a candidate.
   if self.v in ('A','B'):
    prefixes=[stem]+FAMILIES.get(stem,[])
    for shader in self.shaders:
     name=Path(shader['relative_path']).name.lstrip('_').split('.hlsl',1)[0]
     if any(prefix and (name==prefix or name.startswith(prefix+'_') or name.startswith(prefix+'.') or prefix.endswith('_') and name.startswith(prefix)) for prefix in prefixes):
      mapping[(shader['id'],'candidate','')]= {'shader_id':shader['id'],'relative_path':shader['relative_path'],'sha256':shader['sha256'],'status':'filename_family_candidate_only; exact Pass binding unknown','basis':{'file_stem':stem,'prefix_candidates':prefixes},'actual_binding':'unknown'}
   payload['shader_mappings']=list(mapping.values());ids=sorted({m['shader_id'] for m in mapping.values()});payload['shader_interfaces']=[{'shader_id':i,'relative_path':self.by_shader[i]['relative_path'],'mapping_to_pass':'see individual mapping evidence; not confirmed runtime resource binding','stages':[{'entries':s['entries'],'execution_modes':s['execution_modes'],'declared_resources':s['declared_resources'],'stage_interfaces':s['stage_interfaces'],'payload_offset':s['payload_offset'],'payload_sha256':s['payload_sha256']} for s in self.by_shader[i]['stages']],'parse_gaps':self.by_shader[i]['parse_gaps']} for i in ids]
   payload['binding']={'binary':BASES[self.v]['binary'],'pdb':BASES[self.v]['pdb'],'target_executed':False};payload['input_evidence']=list(self.common_inputs)
   d=self.files[rel].with_suffix('.rs.dependencies.json')
   if d.exists():payload['input_evidence'].append({'path':str(d),'sha256':sha(d)})
 def render(self):
  reports=[]
  for rel,payload in self.payloads.items():
   source=self.files[rel];old=source.read_text();clean=old
   if START in old:
    assert old.count(START)==old.count(END)==1;head,tail=old.split(START,1);_,suffix=tail.split(END,1)
    suffix=suffix[2:] if suffix.startswith('\n\n// === PDB ENRICHMENT V1 BEGIN ===') else suffix.removeprefix('\n');clean=head+suffix
   ambiguous=sum(ref['target_aliases']['count']!=1 for ref in payload['native_incoming'])
   rows=[START,'// 渲染 Pass: '+rel,'// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。','// 完整记录: '+source.name+'.render-pass.json.gz','// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。','// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。','// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。',f'// 同地址别名目标候选 {ambiguous} 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。']
   for fn in payload['functions']:
    if not(fn['resource_api_evidence'] or fn['shader_type_hints']):continue
    rows+=['// ------------------------------------------------------------------','// [PDB 函数上下文] '+fn['symbol']]
    for e in fn['resource_api_evidence']:rows.append('// [资源接口动作；资源实例 unknown] '+json.dumps(e,ensure_ascii=False,separators=(',',':')))
    for h in fn['shader_type_hints']:rows.append('// [PDB Shader 类型/入口] '+json.dumps(h,ensure_ascii=False,separators=(',',':')))
   for m in payload['shader_mappings']:rows.append('// [Shader 映射] '+json.dumps(m,ensure_ascii=False,separators=(',',':')))
   if not payload['shader_mappings']:rows.append('// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。')
   for s in payload['shader_interfaces']:
    rows.append('// [Shader 声明接口；Pass 实际绑定见映射状态] '+s['relative_path'])
    for stage in s['stages']:
     rows.append('// Shader stages / entries / workgroups: '+json.dumps({'entries':stage['entries'],'execution_modes':stage['execution_modes']},ensure_ascii=False,separators=(',',':')))
     for io in stage['declared_resources']+stage['stage_interfaces']:rows.append('// [声明 I/O；不是已确认的渲染图资源边] '+json.dumps(io,ensure_ascii=False,separators=(',',':')))
   for order in payload['parent_callsite_order']:rows.append('// [父函数静态调用位置] '+json.dumps(order,ensure_ascii=False,separators=(',',':')))
   if not payload['parent_callsite_order']:rows.append('// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。')
   rows+=['// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。','// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。',END]
   block='\n'.join(rows)+'\n';marker='// === PDB ENRICHMENT V1 BEGIN ==='
   if marker in clean:
    pos=clean.index(marker);text=clean[:pos]+block+'\n'+clean[pos:]
   else:text=clean+('' if clean.endswith('\n') else'\n')+block
   temp=source.with_name(source.name+'.render.tmp');temp.write_text(text);temp.replace(source)
   gzwrite(source.with_suffix('.rs.render-pass.json.gz'),payload)
   reports.append({'path':rel,'file_role':payload['file_role'],'function_contexts':len(payload['functions']),'resource_api_records':sum(len(f['resource_api_evidence']) for f in payload['functions']),'shader_mappings':len(payload['shader_mappings']),'typed_shader_mappings':sum(m['status'].startswith('PDB_typed') for m in payload['shader_mappings']),'shader_interface_declarations':sum(len(s['declared_resources'])+len(s['stage_interfaces']) for f in payload['shader_interfaces'] for s in f['stages']),'static_call_positions':len(payload['parent_callsite_order']),'ambiguous_alias_incoming':ambiguous,'block_sha256':hashlib.sha256(block.encode()).hexdigest(),'preserved_before_sha256':hashlib.sha256(clean.encode()).hexdigest(),'metadata_file':rel+'.render-pass.json.gz','metadata_sha256':sha(source.with_suffix('.rs.render-pass.json.gz'))})
  gzwrite(ROOT/'_pdb'/self.v/'render-shader-catalog.json.gz',{'version':self.v,'grammar_sha256':sha(GRAMMAR),'grammar_source':'https://raw.githubusercontent.com/KhronosGroup/SPIRV-Headers/main/include/spirv/unified1/spirv.core.grammar.json','shaders':self.shaders,'identity_source':BASES[self.v]})
  gzwrite(ROOT/'_pdb'/self.v/'render-address-aliases.json.gz',{str(rva):[{'name':p['name'],'module':p['module'],'record_offset':p.get('record_offset')} for p in procs] for rva,procs in self.by_rva.items() if rva is not None})
  jwrite(ROOT/'_pdb'/self.v/'render-callsite-sequences.json',{'version':self.v,'order_policy':'CPU instruction address order within each parent; runtime branches and GPU resource dependency schedule unknown','parent_functions':[{'entry_rva':src,'aliases':self.aliases(src),'calls':records} for src,records in sorted(self.sequences.items())]})
  report={'version':self.v,'files':len(reports),'shader_files':len(self.shaders),'spirv_modules':sum(len(s['stages']) for s in self.shaders),'shader_parse_gaps':sum(len(s['parse_gaps']) for s in self.shaders),'resource_api_records':sum(f['resource_api_records'] for f in reports),'shader_mappings':sum(f['shader_mappings'] for f in reports),'typed_shader_mappings':sum(f['typed_shader_mappings'] for f in reports),'shader_interface_declarations':sum(f['shader_interface_declarations'] for f in reports),'static_call_positions':sum(f['static_call_positions'] for f in reports),'target_executed':False,'only_existing_directories':True,'files_index':reports}
  jwrite(ROOT/'_pdb'/self.v/'render-pass-report.json',report);print(json.dumps({k:v for k,v in report.items() if k!='files_index'}),flush=True);return report

def snapshot():return {p.relative_to(PROJECT).as_posix():sha(p) for p in (PROJECT/'reconstruction').rglob('*') if p.is_file() and (p.suffix in ('.rs','.toml') or p.name=='Cargo.lock')}
def main():
 before=snapshot();jwrite(ROOT/'_pdb/production-source-snapshot-before-render-passes.json',before)
 reports=[]
 for v in sys.argv[1:] or ['A','B','C']:
  version=Version(v);version.collect();reports.append(version.render())
 assert before==snapshot(),'Production source changed'
 jwrite(ROOT/'render-pass-report.json',{'versions':reports,'scope':'Metadata and shader declarations only; no renderer implementation or GPU execution','production_project_modified':False,'unknowns_explicit':True,'no_new_directory':True})
if __name__=='__main__':main()
