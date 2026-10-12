"""Audit retained render-pass annotations against source inputs and original PE/SPIR-V."""
import collections,gzip,hashlib,json,struct
from pathlib import Path
from enrich_skeleton import pack
ROOT=Path(__file__).resolve().parents[1];PROJECT=ROOT.parent
START='// === RENDER PASSES V1 BEGIN ===';END='// === RENDER PASSES V1 END ==='
BASES=json.loads((PROJECT/'tracking/baselines.json').read_text())['baselines']
GRAMMAR=json.loads((ROOT/'_tools/spirv.core.grammar.json').read_text());OPS={i['opname']:i['opcode'] for i in GRAMMAR['instructions']}
DEC={e['enumerant']:int(e['value'],0) if isinstance(e['value'],str) else e['value'] for k in GRAMMAR['operand_kinds'] if k['kind']=='Decoration' for e in k['enumerants']}
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def gz(p):return json.load(gzip.open(p,'rt'))
def cleaned(text):
 assert text.count(START)==text.count(END)==1
 head,rest=text.split(START,1);content,suffix=rest.split(END,1);block=START+content+END+'\n'
 suffix=suffix[2:] if suffix.startswith('\n\n// === PDB ENRICHMENT V1 BEGIN ===') else suffix.removeprefix('\n')
 return head+suffix,block
results=[];verified_inputs={};direct_byte_checks=0
for version in 'ABC':
 report=json.loads((ROOT/'_pdb'/version/'render-pass-report.json').read_text());baseline=BASES[version];binary=Path(baseline['binary']['path']);assert sha(binary)==baseline['binary']['sha256'];pe=pack.PE(binary)
 catalog=gz(ROOT/'_pdb'/version/'render-shader-catalog.json.gz');assert catalog['version']==version;shaders={s['id']:s for s in catalog['shaders']};aliases=gz(ROOT/'_pdb'/version/'render-address-aliases.json.gz')
 stages=0;descriptors=0;unqualified=0
 for shader in shaders.values():
  path=Path(shader['path']);assert path.is_relative_to(binary.parent/'compiled-shaders');assert sha(path)==shader['sha256'];raw=path.read_bytes();assert shader['version']==version
  for stage in shader['stages']:
   stages+=1;at=stage['payload_offset'];words=stage['payload_words'];assert struct.unpack_from('<I',raw,at-4)[0]==words;data=raw[at:at+words*4];assert hashlib.sha256(data).hexdigest()==stage['payload_sha256'];ws=struct.unpack('<'+'I'*words,data);assert ws[0]==0x07230203
   i=5;count=0;variables={};decorations=collections.defaultdict(dict);entries=[]
   while i<words:
    length=ws[i]>>16;opcode=ws[i]&65535;assert length>0 and i+length<=words;args=ws[i+1:i+length];count+=1
    if opcode==OPS['OpVariable']:variables[args[1]]=args
    if opcode==OPS['OpDecorate']:decorations[args[0]][args[1]]=list(args[2:])
    if opcode==OPS['OpEntryPoint']:entries.append(args[1])
    i+=length
   assert count==stage['instruction_count'] and i==words
   assert [e['function_id'] for e in stage['entries']]==entries
   for io in stage['declared_resources']+stage['stage_interfaces']:
    descriptors+=1;vid=io['variable_id'];assert vid in variables and variables[vid][0]==io['type']['id'];assert io['runtime_resource_handle']=='unknown';assert io['actually_used_by_entry'].startswith('unknown')
    if io['declared_direction']=='unknown_read_write_direction':unqualified+=1
    for key,name in [('descriptor_set','DescriptorSet'),('binding','Binding'),('location','Location'),('builtin','BuiltIn')]:
     if DEC[name] in decorations[vid]:assert io[key]==decorations[vid][DEC[name]][0]
 seq=json.loads((ROOT/'_pdb'/version/'render-callsite-sequences.json').read_text());orders={p['entry_rva']:p['calls'] for p in seq['parent_functions']};byte_cache=set();source_count=api_count=map_count=typed_count=positions=0
 for row in report['files_index']:
  source_count+=1;p=ROOT/version/row['path'];clean,block=cleaned(p.read_text());assert hashlib.sha256(block.encode()).hexdigest()==row['block_sha256'];assert hashlib.sha256(clean.encode()).hexdigest()==row['preserved_before_sha256'],('Outside-block content changed',p)
  assert all(not line or line.startswith('//') for line in block.splitlines());companion=ROOT/version/row['metadata_file'];assert sha(companion)==row['metadata_sha256'];payload=gz(companion);assert payload['version']==version and payload['source']==row['path'] and payload['binding']['binary']==baseline['binary'] and payload['binding']['pdb']==baseline['pdb']
  assert payload['resource_dependency_edges'].startswith('unknown');assert 'GPU' in block and 'unknown' in block
  for evidence in payload['input_evidence']:
   if evidence['path'] not in verified_inputs:verified_inputs[evidence['path']]=sha(Path(evidence['path']))
   assert verified_inputs[evidence['path']]==evidence['sha256']
  dep_path=p.with_suffix('.rs.dependencies.json');deps=json.loads(dep_path.read_text())['functions'] if dep_path.exists() else [];dep_by={d['name']:d['dependencies'] for d in deps}
  for fn in payload['functions']:
   for api in fn['resource_api_evidence']:
    api_count+=1;assert api['resource_instance']=='unknown';assert any(d['function']==api['symbol'] and d['relationship']==api['relationship'] and d['first_evidence']==api['evidence'] for d in dep_by.get(fn['symbol'],[]));assert api['symbol'] in block
  for mapping in payload['shader_mappings']:
   map_count+=1;shader=shaders[mapping['shader_id']];assert mapping['sha256']==shader['sha256'] and mapping['relative_path']==shader['relative_path'];assert 'unknown' in mapping['status'];assert mapping['relative_path'] in block
   if mapping['status'].startswith('PDB_typed'):
    typed_count+=1;assert version=='C';hint=mapping['hint'];assert shader['header_shader_uri'].split(':',1)[0]==hint['shader_family'];assert hint['entry']=='unknown' or shader['header_shader_uri'].rsplit(':',1)[-1]==hint['entry']
  for reflection in payload['shader_interfaces']:
   shader=shaders[reflection['shader_id']];assert len(reflection['stages'])==len(shader['stages'])
   for actual,expected in zip(reflection['stages'],shader['stages']):
    for key in ['entries','execution_modes','declared_resources','stage_interfaces','payload_offset','payload_sha256']:assert actual[key]==expected[key]
  for transfer in payload['native_incoming']+payload['native_outgoing']:
   assert transfer['runtime_executed'] is False;inst=transfer['instruction_rva'];data=bytes.fromhex(transfer['bytes']);key=(inst,transfer['bytes'])
   if key not in byte_cache:assert pe.read_rva(inst,len(data))==data;byte_cache.add(key);direct_byte_checks+=1
   for group in [transfer['caller_aliases'],transfer['target_aliases']]:
    full=aliases.get(str(group['alias_group_rva']),[]);assert group['count']==len(set(x['name'] for x in full));assert group['exclusive_symbol']=='unknown'
  for position in payload['parent_callsite_order']:
   positions+=1;calls=orders[position['caller_entry_rva']];rank=position['position_among_mapped_render_targets']-1;assert len(calls)==position['mapped_render_target_count'];assert calls[rank]['instruction_rva']==position['callsite_rva'];assert row['path'] in calls[rank]['target_source_candidates'];assert 'unknown' in position['ordering_status'];assert calls[rank]['target_aliases']['count']==1
  assert row['ambiguous_alias_incoming']==sum(t['target_aliases']['count']!=1 for t in payload['native_incoming'])
 assert stages==report['spirv_modules'] and source_count==report['files'] and api_count==report['resource_api_records'] and map_count==report['shader_mappings'] and typed_count==report['typed_shader_mappings'] and positions==report['static_call_positions']
 results.append({'version':version,'source_files_verified':source_count,'resource_api_records_verified':api_count,'shader_files_sha256_verified':len(shaders),'spirv_modules_boundary_verified':stages,'declared_interfaces_verified':descriptors,'unknown_resource_access_declarations':unqualified,'shader_mappings_verified':map_count,'typed_shader_mappings_verified':typed_count,'static_call_positions_verified':positions,'unique_native_instruction_bytes_verified':len(byte_cache),'preserved_existing_source_content':True,'unknown_runtime_graph_edges_preserved':True,'shader_parse_gaps':report['shader_parse_gaps']})
 print(json.dumps(results[-1]),flush=True)
before=json.loads((ROOT/'_pdb/production-source-snapshot-before-render-passes.json').read_text());after={p.relative_to(PROJECT).as_posix():sha(p) for p in (PROJECT/'reconstruction').rglob('*') if p.is_file() and (p.suffix in ('.rs','.toml') or p.name=='Cargo.lock')};assert before==after
result={'ok':True,'versions':results,'production_source_files_unchanged':len(before),'original_instruction_byte_checks':direct_byte_checks,'no_new_delivery_directory':True,'scope':'Metadata and static artifact audit; not GPU execution or complete renderer verification'}
(ROOT/'render-pass-verification.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
