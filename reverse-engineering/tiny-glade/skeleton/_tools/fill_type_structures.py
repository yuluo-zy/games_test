"""Expand exact PDB types into existing A/B/C metadata sources. No invented ABI."""
from __future__ import annotations
import collections, copy, gzip, hashlib, json, re, sys
from pathlib import Path, PurePosixPath
import yaml
from enrich_skeleton import generic_nodes
ROOT=Path(__file__).resolve().parents[1]
PROJECT=ROOT.parent
START='// === PDB STRUCTURES V1 BEGIN ==='
END='// === PDB STRUCTURES V1 END ==='
UDT={'LF_STRUCTURE','LF_CLASS','LF_UNION','LF_ENUM'}
REFS={'ReturnType','ArgumentList','ReferentType','ModifiedType','ClassType','ThisType','FieldList','DerivationList','VTableShape','ElementType','IndexType','UnderlyingType','Type','TypeIndex','MemberType','MethodList','ContinuationIndex','ArgIndices','BaseType','VFPtrType','ContainingType','OverloadedMethodIndex'}
TYPE_NAME=re.compile(r'(?<![\w:])([a-z_][a-z_0-9]*(?:::[A-Za-z_][A-Za-z_0-9$]*)*::[A-Z][A-Za-z_0-9]*)(?![A-Za-z_0-9])')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def js(p,x):
 p.parent.mkdir(parents=True,exist_ok=True); t=p.with_name(p.name+'.tmp'); t.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n');t.replace(p)
def gz(p,x):
 p.parent.mkdir(parents=True,exist_ok=True);t=p.with_name(p.name+'.tmp')
 with gzip.GzipFile(filename='',mode='wb',fileobj=t.open('wb'),mtime=0) as f:f.write(json.dumps(x,ensure_ascii=False,separators=(',',':')).encode())
 t.replace(p)
def body(d):return next((v for k,v in d.items() if k!='Kind' and isinstance(v,dict)),{})
def refs(x,key=''):
 if isinstance(x,dict):
  for k,v in x.items():yield from refs(v,k)
 elif isinstance(x,list):
  for v in x:yield from refs(v,key)
 elif key in REFS and type(x) is int and x:yield x

def fix_strings(row):
 d=copy.deepcopy(row.get('decoded',{}))
 # YAML 1.1 interprets unquoted true/false names as bool. Preserve original spelling.
 def needs(x):
  if isinstance(x,dict):return any((k in ('Name','String','UniqueName') and not isinstance(v,str)) or needs(v) for k,v in x.items())
  if isinstance(x,list):return any(needs(v) for v in x)
  return False
 if needs(d):
  raw=yaml.load(row['yaml'],Loader=yaml.CBaseLoader)
  def repair(a,b):
   if isinstance(a,dict):
    for k,v in a.items():
     if k in ('Name','String','UniqueName'):a[k]=b[k]
     else:repair(v,b[k])
   elif isinstance(a,list):
    for v,w in zip(a,b):repair(v,w)
  repair(d,raw)
 return d

class Recovery:
 def __init__(self,v):
  self.v=v;self.out=ROOT/'_pdb'/v/'structures';self.tpi={};self.ipi={};self.files=collections.defaultdict(lambda:{'definitions':[],'named_symbol_entities':[],'observed_game_structures':[],'unowned_type_records':[]})
  primary=json.loads((ROOT/'_pdb'/v/'source-files.json').read_text())
  sources={s['path']:s for s in primary}
  # Audit the entire current version tree, including later restored packages,
  # generated files, explicit unknown buckets and manifest source stubs.
  for source in (ROOT/v).rglob('*.rs'):
   path=source.relative_to(ROOT/v).as_posix();meta=source.with_suffix('.rs.pdb.json')
   if meta.exists():
    entry=json.loads(meta.read_text());entry['path']=path;sources[path]=entry
   elif path not in sources:sources[path]={'version':v,'path':path,'original_paths':[],'source_kind':'existing_source; original ownership unknown'}
  self.sources=list(sources.values());self.source_by_original={o:s['path'] for s in self.sources for o in s.get('original_paths',[])}
  self.owners={};self.scope_map=collections.defaultdict(set);self.names={};self.bind=json.loads((ROOT/'_pdb'/v/'enrichment'/'metadata-stage.json').read_text())['inputs']
  self.simple={};header=Path('/opt/homebrew/opt/llvm/include/llvm/DebugInfo/CodeView/TypeIndex.h')
  h=header.read_text();enum=h.split('enum class SimpleTypeKind',1)[1].split('};',1)[0]
  for name,value in re.findall(r'(\w+)\s*=\s*(0x[0-9a-fA-F]+)',enum):self.simple[int(value,16)]=name
  self.simple_header={'path':str(header),'sha256':sha(header)}
  for l in gzip.open(ROOT/'_pdb'/v/'types.jsonl.gz','rt'):
   r=json.loads(l);r['decoded']=fix_strings(r);(self.tpi if r['namespace']=='TPI' else self.ipi)[r['index']]=r
  for cargo in (ROOT/v).rglob('Cargo.toml'):
   m=re.search(r'(?m)^name\s*=\s*"([^"]+)"',cargo.read_text())
   if m:self.owners[m[1].replace('-','_')]=cargo.parent.relative_to(ROOT/v).as_posix()+'/src/'
  for s in self.sources:
   path=s['path']
   for crate,prefix in self.owners.items():
    if not path.startswith(prefix):continue
    tail=path[len(prefix):].removesuffix('.rs').split('/')
    if tail[-1] in ('mod','lib','main'):tail=tail[:-1]
    self.scope_map['::'.join([crate]+tail)].add(path)
  # Fallback files that were mapped in enrichment are not treated as original definitions.
  self.defs={};self.source_records=collections.defaultdict(list)
  for s in self.sources:
   self.files[s['path']]
  for idx,r in self.ipi.items():
   if r['decoded'].get('Kind') in ('LF_UDT_SRC_LINE','LF_UDT_MOD_SRC_LINE'):
    b=body(r['decoded']);original=self.string(b.get('SourceFile',0));self.source_records[b['UDT']].append({'ipi_index':idx,'original_path':original,'line':b.get('LineNumber'),'module':b.get('Module'),'source_string_id':b.get('SourceFile')})
 def string(self,idx,seen=None):
  seen=set() if seen is None else seen
  if not idx:return ''
  if idx in seen:return 'unknown: cyclic IPI string'
  seen.add(idx);r=self.ipi.get(idx,{})['decoded'] if idx in self.ipi else {};b=body(r)
  if r.get('Kind')=='LF_STRING_ID':return self.string(b.get('Id',0),seen)+b.get('String','')
  if r.get('Kind')=='LF_SUBSTR_LIST':return ''.join(self.string(i,seen.copy()) for i in b.get('StringIndices',[]))
  return 'unknown: IPI string '+hex(idx)
 def reference_path(self,original):
  if original in self.source_by_original:return self.source_by_original[original]
  if not original or original.startswith('unknown:'):return '_unmapped/pdb-data-types.rs'
  name=PurePosixPath(original.replace('\\','/')).name
  if not name.endswith('.rs'):name+='.rs'
  return '_references/sources/'+hashlib.sha256(original.encode()).hexdigest()[:16]+'/'+name
 def map_name(self,name):
  parts=name.split('::')[:-1]
  while parts:
   scope='::'.join(parts)
   if scope in self.scope_map:return sorted(self.scope_map[scope]),'namespace_module_candidate; author definition file unknown'
   parts.pop()
  return ['_unmapped/game-named-types.rs'],'unknown_author_source'
 def type_ref(self,idx,seen=None):
  seen=set() if seen is None else seen
  if not idx:return {'index':idx,'spelling':'none / sentinel','status':'recorded_sentinel'}
  if idx<0x1000:
   simple=self.simple.get(idx&0xff,'unknown');mode=(idx>>8)&0xf
   return {'index':idx,'spelling':simple+(' / CodeView pointer mode '+str(mode) if mode else ''),'status':'CodeView_simple_type; not Rust source spelling'}
  r=self.tpi.get(idx);d=r.get('decoded',{}) if r else {};b=body(d);k=d.get('Kind','unknown')
  if idx in seen:return {'index':idx,'kind':k,'spelling':b.get('Name','recursive reference'),'status':'recorded_recursive_reference'}
  seen.add(idx);name=b.get('Name')
  if name:return {'index':idx,'kind':k,'spelling':name,'status':'PDB_record_name'}
  inner={key:self.type_ref(b[key],seen.copy()) for key in ('ReferentType','ModifiedType','ElementType','UnderlyingType','Type') if type(b.get(key)) is int}
  return {'index':idx,'kind':k,'status':'PDB_record' if r else 'unknown_missing_record','record':d,'nested_types':inner}
 def graph(self,seeds):
  todo=list(seeds);seen=set()
  while todo:
   idx=todo.pop()
   if idx in seen or idx<0x1000:continue
   if idx not in self.tpi:raise RuntimeError('Missing TPI '+hex(idx))
   seen.add(idx);todo.extend(refs(self.tpi[idx]['decoded']))
  return seen
 def fields(self,idx,seen=None):
  seen=set() if seen is None else seen
  if not idx:return []
  if idx in seen:raise RuntimeError('FieldList cycle '+hex(idx))
  seen.add(idx);r=self.tpi[idx];assert r['decoded']['Kind']=='LF_FIELDLIST'
  result=[]
  for order,x in enumerate(r['decoded']['FieldList']):
   b=body(x)
   if x['Kind']=='LF_INDEX':result.extend(self.fields(b['ContinuationIndex'],seen));continue
   result.append({'field_list_index':idx,'order':order,'kind':x['Kind'],'record':x,'type_references':[self.type_ref(i) for i in sorted(set(refs(x)))]})
  return result
 def definitions(self):
  full_by_key=collections.defaultdict(list)
  for idx,r in self.tpi.items():
   d=r['decoded'];b=body(d)
   if d.get('Kind') in UDT and 'ForwardReference' not in b.get('Options',[]):full_by_key[(d['Kind'],b.get('UniqueName') or b.get('Name'))].append(idx)
  for idx,r in self.tpi.items():
   d=r['decoded'];k=d.get('Kind');b=body(d)
   if k not in UDT:continue
   forward='ForwardReference' in b.get('Options',[]);key=(k,b.get('UniqueName') or b.get('Name'));candidates=full_by_key[key] if forward and key[1] and key[1]!='<unnamed-tag>' else []
   own=self.source_records.get(idx,[]);inherited=[q for i in candidates for q in self.source_records.get(i,[])] if not own else []
   targets=sorted({self.reference_path(q['original_path']) for q in own or inherited}) or ['_unmapped/pdb-data-types.rs']
   mapping='PDB_UDT_source_line' if own else 'forward_definition_name_candidate; original declaration file unknown' if inherited else 'unknown_original_file'
   item={'id':f'{self.v}:TPI:{idx:X}','index':idx,'kind':k,'name':b.get('Name','unknown'),'unique_name':b.get('UniqueName'),'definition_status':'forward_declaration_only' if forward else 'PDB_definition_record','size_bytes':None if forward else b.get('Size'),'alignment':'unknown','packing':'unknown','rust_repr':'unknown','record':d,'fields_and_relations':self.fields(b.get('FieldList',0)),'source_records':own,'candidate_definition_indices':candidates,'source_mapping':mapping,'target_files':targets,'unknowns':['original source text','unrecorded visibility/attributes','Rust ABI and layout for unrelated Rust names']}
   self.defs[idx]=item
   for path in targets:self.files[path]['definitions'].append(item)
 def named(self):
  found=collections.defaultdict(lambda:{'ipi_occurrences':[],'procedure_occurrences':[],'relationship_records':{}})
  for idx,r in self.ipi.items():
   d=r['decoded'];b=body(d)
   text=b.get('String','') if d.get('Kind')=='LF_STRING_ID' else b.get('Name','')
   if d.get('Kind')=='LF_FUNC_ID':text=self.string(b.get('ParentScope',0))+'::'+text
   if not text:continue
   for match in TYPE_NAME.finditer(text):
    name=match.group(1)
    if name.split('::',1)[0] not in self.owners:continue
    # Uppercase path segments may be enum variants; never assume they are struct definitions.
    data=found[name];data['ipi_occurrences'].append({'index':idx,'kind':d['Kind'],'spelling':text,'role':'symbol_name_or_type_scope; exact entity kind unknown'})
  procedures=json.load(gzip.open(ROOT/'_pdb'/self.v/'enrichment'/'procedure-index.json.gz','rt'))
  for proc in procedures:
   text=proc['name'];matches={m.group(1) for m in TYPE_NAME.finditer(text) if m.group(1).split('::',1)[0] in self.owners}
   if not matches:continue
   relation_nodes=generic_nodes(text)
   for name in sorted(matches):
    found[name]['procedure_occurrences'].append({'module':proc['module'],'record_offset':proc.get('record_offset'),'rva':proc.get('rva'),'name':text,'role':'PDB procedure generic type / associated symbol occurrence'})
   for rel in relation_nodes:
    spelling=json.dumps(rel,ensure_ascii=False)
    for name in matches:
     if name in rel['type']:found[name]['relationship_records'][spelling]=rel
  # All generic relation records from available system/interface catalog, verbatim.
  catalog=json.load(gzip.open(ROOT/'_pdb'/self.v/'enrichment'/'candidate-interfaces.json.gz','rt'))
  candidates=catalog.values() if isinstance(catalog,dict) else catalog
  for c in candidates:
   if not isinstance(c,dict):continue
   for rel in c.get('type_relations',[]):
    spelling=json.dumps(rel,ensure_ascii=False)
    for m in TYPE_NAME.finditer(spelling):
     name=m.group(1)
     if name.split('::',1)[0] not in self.owners:continue
     found[name]['relationship_records'][spelling]=rel
  for name,data in sorted(found.items()):
   targets,status=self.map_name(name)
   item={'name':name,'kind':'unknown (type / enum variant / associated symbol)','definition_status':'name_and_relations_only','fields':'unknown','size_bytes':'unknown','alignment':'unknown','enum_values':'unknown','source_mapping':status,'target_files':targets,'ipi_occurrences':data['ipi_occurrences'],'procedure_occurrences':data['procedure_occurrences'],'type_relationships':list(data['relationship_records'].values()),'layout_relation':'generic nesting or symbol scope does not prove member fields, inheritance, or runtime scheduling'}
   self.names[name]=item
   for path in targets:self.files[path]['named_symbol_entities'].append(item)
 def observed(self):
  touched=set()
  def ensure(path):
   if path in touched:return
   if getattr(self,'observe_only',False):
    companion=ROOT/self.v/(path+'.structures.json.gz')
    if companion.exists():self.files[path]=json.load(gzip.open(companion,'rt'))
   self.files[path]['observed_game_structures']=[]
   touched.add(path)
  def evidence(rel):
   p=PROJECT/rel;return {'path':rel,'sha256':sha(p)}
  def add(path,name,fields,basis,**more):
   assert (ROOT/self.v/path).exists(),path
   ensure(path)
   item={'name':name,'baseline':self.v,'status':'native_observed_partial_layout; not full original struct declaration','fields':fields,'alignment':'unknown','complete_size':'unknown','unobserved_fields':'unknown','original_field_names':'unknown unless explicitly established','evidence':basis,**more}
   self.files[path]['observed_game_structures'].append(item)
  constructor='evidence/building-state/constructor-version-comparison.json';doc=json.loads((PROJECT/constructor).read_text());version=next(x for x in doc['versions'] if x['baseline']==self.v)
  baseline=json.loads((PROJECT/'tracking/baselines.json').read_text())['baselines'][self.v]
  assert version['source_sha256']==baseline['binary']['sha256']
  writes=[{'offset':0,'width':8,'original_name':'unknown','type':'unknown; qword copied from RDX'}, {'offset':8,'width':28,'original_name':'unknown','type':'unknown; aggregate copied from R9'}, {'offset':0x24,'width':28,'original_name':'unknown','type':'unknown; aggregate copied from R8'}]
  offset=0x40 if self.v=='A' else 0x4c
  writes += [{'offset':offset,'width':4,'original_name':'unknown','type':'f32 store from XMM0'}, {'offset':offset+4,'width':4,'original_name':'unknown','type':'unknown; dword bits copied'}, {'offset':offset+8,'width':1,'original_name':'unknown','type':'unknown; byte copied'}]
  if self.v!='A':writes.append({'offset':0x40,'width':12,'original_name':'unknown','type':'unknown; aggregate copied from pointer'})
  add(doc['source_path'],'country_core::resources::roofs::roof_shape::Roof',writes,[evidence(constructor)],native_constructor=version,limitations=doc['limitations'])
  nani='evidence/building-nani/findings.md'
  subset=9 if self.v in ('A','B') else 7
  add('crates/country-core/src/resources/roofs/roof_shape.rs','Roof Nani creation relationship (version specific)',[],[evidence(nani)],relationship={'bundle':'NaniBundle<StoneFloorInstanceData>','mesh_selection':'AtlasPart(0)','subset_kind':subset,'subset_kind_status':'subset classification, not lifecycle','full_component_id_mapping':'B explicit; A/C incomplete'},enum_values={'StoneFloor_subset_kind':subset,'scope':'observed descriptor assignment, not full enum definition'})
  if self.v!='B':return
  params='evidence/multi-version/B-parameter-findings.md'
  params_fields=[{'offset':0,'width':8,'type':'[f32;2] observed components','original_name':'tip_offset_01','name_basis':'serialized field'}, *[{'offset':off,'width':4,'type':'f32','original_name':name,'name_basis':'serialized field'} for off,name in [(8,'profile_01'),(12,'height_01'),(16,'ridge_length_01'),(20,'eave_length_01')]], {'offset':24,'width':1,'type':'RoofRidgeDirection observed byte','original_name':'ridge_dir','name_basis':'serialized field'}, {'offset':25,'width':1,'type':'unknown; rim byte','original_name':'unknown','observed_semantics':'flat rim; missing default unknown'}]
  add(doc['source_path'],'country_core::resources::roofs::roof_shape::RoofShapeParams',params_fields,[evidence(params),evidence('reconstruction/roof/parameters.rs'),evidence('evidence/multi-version/B-parameters-verification.json')],observed_span=28,unobserved_tail='bytes 26/27 preserved, original padding/field status unknown')
  add(doc['source_path'],'country_core::resources::roofs::roof_shape::RoofRidgeDirection',[],[evidence(params)],enum_values={'Width':0,'Length':1},discriminant_representation='observed byte in B; original repr attribute unknown')
  add('crates/country-core/src/resources/history/roof_edit.rs','RoofEdit native replay variants',[],[evidence(params),evidence('reconstruction/history/observed-history-schema.json'),evidence('evidence/multi-version/B-discrete-replay-verification.json')],native_tags=[{'value':0,'serialized_name':'CreateRoof'},{'value':1,'serialized_name':'DeleteRoof'},{'value':2,'serialized_name':'VisualEdit'},{'value':3,'serialized_name':'SwitchRoofRidgeDir'},{'value':4,'serialized_name':'unknown','observed_branch_semantics':'flat rim style change'}],discriminant_representation='B replay payload tags; complete Rust enum ABI unknown')
  for name,preferred,fields,span in [('RoofAnimation','crates/systems/roof/src/visual/roof_animation.rs',[{'offset':0,'width':4,'type':'f32','original_name':'unknown','observed_semantics':'primary animation channel'},{'offset':4,'width':4,'type':'f32','original_name':'unknown','observed_semantics':'secondary animation channel'}],8),('RoofTileSimulation','crates/country-core/src/resources/roofs/roof_tile_simulation.rs',[{'offset':4,'width':4,'type':'f32','original_name':'unknown','observed_semantics':'initialized from max_y; other bytes opaque'}],24)]:
   candidates={p for n in self.names if n.endswith('::'+name) for p in self.names[n]['target_files'] if not p.startswith('_unmapped/')}
   path=preferred if (ROOT/self.v/preferred).exists() else next(iter(candidates)) if len(candidates)==1 else None
   if path:add(path,name,fields,[evidence('reconstruction/building/roof_components.rs'),evidence('evidence/building-lifecycle/B-verification.json')],observed_storage_span=span,source_mapping='PDB namespace candidate; original definition file unknown')
  add('crates/systems/roof/src/create_roof.rs','B Roof creation Bundle component graph',[],[evidence(nani)],explicit_components=['Roof','RoofTileSimulation','RoofTriggerVolume','RoofAnimation','RoofTiles','RoofSpire','RoofSupportBeams','NaniInstanceDesc','utils::components::transform::Transform','InstanceDataMarker<StoneFloorInstanceData>'],required_component_edges=[],zst_markers=['RoofSpire','RoofSupportBeams','InstanceDataMarker<StoneFloorInstanceData>'],known_component_storage={'NaniInstanceDesc_stride':24,'Transform_stride':48,'NaniBundle_copy_span':80},limitations=['ZST markers do not prove generated support/spire geometry','self registration does not form require-self edges','Bundle copy span includes alignment/padding; original Rust repr unknown'])
  wall='crates/country-core/src/resources/walls/public_wall_state.rs'
  field=lambda off,width,ty,semantic: {'offset':off,'width':width,'type':ty,'original_name':'unknown','observed_semantics':semantic}
  fields=[field(0x10,8,'u64 read','raw Curve2 point count'),field(0x30,4,'f32','raw Curve2 length'),field(0x108,1,'u8 bit test; full field type unknown','max_y cache presence bit 0'),field(0x10c,4,'f32','max_y cache value'),field(0x110,1,'u8 bit test; full field type unknown','gable max_y cache presence bit 0'),field(0x114,4,'f32','gable max_y cache value'),field(0x258,28,'unknown; raw shape record','shape tag at first four bytes; payload incomplete'),field(0x28c,4,'f32','published raw Curve2 length'),field(0x299,1,'u8 read; original enum unknown','style; half-timber consumer checks >= 2')]
  add(wall,'country_core::resources::walls::public_wall_state::PublicWallState',fields,[evidence('reconstruction/building/wall_state.rs'),evidence('evidence/building-gather/findings.md'),evidence('evidence/building-state/B-spatial-verification.json')],observed_storage_span=0x2a0,span_status='B observed storage projection, not recovered Rust ABI')
  publication='evidence/building-production-authority/B-publication-source-fields.json';pub=json.loads((PROJECT/publication).read_text());assert pub['binary_sha256']==baseline['binary']['sha256']
  bindings=pub['bindings']+pub['additional_bindings']
  public_fields=[{'offset':int(x['public_offset'],16),'original_name':'unknown','type':'unknown','observed_semantics':x.get('name',x.get('semantic_name')),'raw_source_offset':int(x['raw_offset'],16),'binding':x} for x in bindings]
  add(wall,'PublicWallState publication source bindings (partial)',public_fields,[evidence(publication)],evidence_scope=pub['scope'],limitations=pub['excluded'])
  # Raw offsets are kept with their matching source projection; no invented Rust container internals.
  rawpaths=[p for p in self.scope_map.get('country_core::resources::walls::raw_walls',[])];rawpath=rawpaths[0] if len(rawpaths)==1 else 'crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs'
  if (ROOT/self.v/rawpath).exists():add(rawpath,'RawInnerWall source projection',[{'offset':int(x['raw_offset'],16),'original_name':'unknown','type':'unknown','observed_semantics':x.get('name',x.get('semantic_name')),'public_destination_offset':int(x['public_offset'],16)} for x in bindings],[evidence(publication)],source_mapping='native publisher evidence; full type definition ownership unknown')
  coverage='evidence/pdb-structure/type-layout-coverage.json';win=json.loads((PROJECT/coverage).read_text())['window_aspect_ratio']
  add('crates/country-core/src/resources/window.rs','country_core::resources::window::WindowSize',[field(x['offset'],x['read_bytes'],'u32 read', 'aspect ratio numerator' if x['offset']==0 else 'aspect ratio denominator') for x in win['reads']],[evidence(coverage)],native_observation=win)
  visual='evidence/multi-version/B-roof-replay-schema/visual-field-map.json';visualdoc=json.loads((PROJECT/visual).read_text());assert visualdoc['source_sha256']==baseline['binary']['sha256']
  add('crates/country-core/src/resources/history/roof_edit.rs','Roof history VisualEdit serialized fields',[{'offset':x['offset_from_enum'],'original_name':x['field'],'type':'unknown','width':'unknown','name_basis':'native serializer key','binding':x} for x in visualdoc['fields']],[evidence(visual)],offset_base='VisualEdit enum record; history offsets retained separately',enum_discriminant='unknown')
  terrain='evidence/terrain-editing/B-history-schema.json';td=json.loads((PROJECT/terrain).read_text())
  add('crates/country-core/src/resources/history/terrain_edit.rs','TerrainEdit native variants (partial)',[],[evidence(terrain)],native_variant_schema=td,enum_discriminant='native niche; not Rust repr u32')
  # Evidence-only component/query graph: names remain the observed symbols; no require-self edges inferred.
  for rel in ['evidence/building-nani/B-solid-vertex-color-owner/component-query-types.json','evidence/building-lifecycle/B-types/trait-candidates.json','evidence/building-wall-history/B-rectangle-cancel-helpers/decorator-types.json','evidence/building-planning/roof-decorator-library-and-function-schemas-B.json']:
   p=PROJECT/rel
   if not p.exists():continue
   data=json.loads(p.read_text());text=json.dumps(data,ensure_ascii=False);targets=set()
   for m in TYPE_NAME.finditer(text):
    if m.group(1) in self.names:targets.update(self.names[m.group(1)]['target_files'])
   for path in sorted(targets):
    if path.startswith('_unmapped/'):continue
    ensure(path)
    self.files[path]['observed_game_structures'].append({'name':'existing native type/interface evidence','baseline':'B','status':'evidence_symbol_relationships; layout unknown','fields':'unknown','enum_values':'unknown','original_definition_file':'unknown','evidence':[evidence(rel)],'evidence_data':data,'limitations':['Query/trait/generic symbols are type relations, not member fields','self registration does not establish a required-component edge','B analysis database identity may be patched; no A/C transfer']})
 def render(self):
  all_covered=set();report=[]
  for path,payload in list(self.files.items()):
   indices=self.graph([x['index'] for x in payload['definitions']]);payload['type_graph']={str(i):self.tpi[i] for i in sorted(indices)};all_covered.update(indices)
  orphan=[] if getattr(self,'observe_only',False) else sorted(set(self.tpi)-all_covered)
  if orphan:
   self.files['_unmapped/pdb-type-records.rs']['unowned_type_records']=[self.tpi[i] for i in orphan]
  for path,payload in sorted(self.files.items()):
   source=ROOT/self.v/path;source.parent.mkdir(parents=True,exist_ok=True)
   original=source.read_text() if source.exists() else f'// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.\n// Version: {self.v}\n// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。\n'
   if START in original:
    assert END in original;prefix,rest=original.split(START,1);_,suffix=rest.split(END,1);original=prefix.rstrip()+'\n'+suffix.lstrip('\n')
   rows=[START,'// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。','// 完整结构及原始类型记录: '+source.name+'.structures.json.gz','// 未记录的字段、布局、枚举值及源码定义归属 = unknown。']
   for d in payload['definitions']:
    rows+=['// ------------------------------------------------------------------','// [PDB '+d['definition_status']+'] '+d['name']+'; TPI '+hex(d['index']),'// 原始定义文件映射: '+d['source_mapping'],'// size = '+str(d['size_bytes'] if d['size_bytes'] is not None else 'unknown')+'; alignment / packing / Rust repr = unknown','// 定义记录: '+json.dumps(d['record'],ensure_ascii=False)]
    for x in d['fields_and_relations']:
     rows.append('// [字段 / 枚举值 / 类型关系] '+json.dumps(x,ensure_ascii=False))
    if d['candidate_definition_indices']:rows.append('// 前向定义候选（仅名称关联）: '+str(d['candidate_definition_indices']))
    for x in d['source_records']:rows.append('// [PDB UDT 源码行] '+json.dumps(x,ensure_ascii=False))
   if not any(payload[k] for k in ('definitions','named_symbol_entities','observed_game_structures','unowned_type_records')):
    rows.append('// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。')
   for n in payload['named_symbol_entities']:
    rows+=['// ------------------------------------------------------------------','// [PDB 命名实体；类型/枚举变体身份待确认] '+n['name'],'// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。','// 原始定义文件: '+n['source_mapping'],'// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: '+str(len(n['ipi_occurrences']))+'; 过程引用数: '+str(len(n['procedure_occurrences']))]
    for rel in n['type_relationships']:rows.append('// [PDB 泛型关系，非字段关系] '+json.dumps(rel,ensure_ascii=False))
   for o in payload['observed_game_structures']:
    rows+=['// ------------------------------------------------------------------','// [已有逆向证据 '+o['baseline']+' / '+o['status']+'] '+o['name']]
    # Full evidence snapshots are in companion; every accepted field is also readable inline.
    for k,value in o.items():
     if k not in ('fields','evidence_data','native_constructor'):rows.append('// '+k+': '+json.dumps(value,ensure_ascii=False))
    for f in o.get('fields',[]) if isinstance(o.get('fields'),list) else []:rows.append('// [观察字段；未命名者 original_name=unknown] '+json.dumps(f,ensure_ascii=False))
   # Every referenced/remaining TPI record is printed, including bitfields, modifiers, pointers and methods.
   records=list(payload.get('type_graph',{}).values())+payload['unowned_type_records']
   if records:rows.append('// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===')
   for r in records:rows.append('// [TPI '+hex(r['index'])+'] '+json.dumps(r['decoded'],ensure_ascii=False))
   rows.append(END)
   # Place before enrichment so legacy enrichment refresh cannot discard structures.
   block='\n'.join(rows)+'\n';marker='// === PDB ENRICHMENT V1 BEGIN ==='
   if marker in original:pos=original.index(marker);new=original[:pos].rstrip()+'\n\n'+block+'\n'+original[pos:]
   else:new=original.rstrip()+'\n\n'+block
   temp=source.with_name(source.name+'.structures.tmp');temp.write_text(new);temp.replace(source)
   payload.update({'version':self.v,'pdb_binding':self.bind,'source':path,'simple_type_names_source':self.simple_header,'facts_policy':'No guessed fields or Rust ABI. PDB and native observations remain separate.'});gz(source.with_suffix('.rs.structures.json.gz'),payload)
   report.append({'path':path,'definitions':len(payload['definitions']),'named_symbol_entities':len(payload['named_symbol_entities']),'observed_structures':len(payload['observed_game_structures']),'type_graph_records':len(records),'metadata_file':path+'.structures.json.gz','source_sha256':sha(source),'structures_block_sha256':hashlib.sha256(block.encode()).hexdigest(),'companion_sha256':sha(source.with_suffix('.rs.structures.json.gz'))})
  if getattr(self,'observe_only',False):
   prior=json.loads((self.out/'report.json').read_text());index={x['path']:x for x in prior['files_index']};index.update({x['path']:x for x in report});prior['files_index']=list(index.values());prior['files']=len(index);prior['native_structure_attachments']=sum(x['observed_structures'] for x in index.values());js(self.out/'report.json',prior);print(json.dumps({'version':self.v,'observed_files_refreshed':len(report),'native_structure_attachments':prior['native_structure_attachments']}),flush=True);return prior
  data={'version':self.v,'pdb_binding':self.bind,'UDT_records':len(self.defs),'definition_records':sum(x['definition_status']=='PDB_definition_record' for x in self.defs.values()),'forward_records':sum(x['definition_status']=='forward_declaration_only' for x in self.defs.values()),'member_records':sum(f['kind']=='LF_MEMBER' for d in self.defs.values() for f in d['fields_and_relations']),'enum_values':sum(f['kind']=='LF_ENUMERATE' for d in self.defs.values() for f in d['fields_and_relations']),'named_symbol_entities':len(self.names),'files':len(report),'all_TPI_records_retained_in_files':len(all_covered)+len(orphan),'TPI_records':len(self.tpi),'native_structure_attachments':sum(x['observed_structures'] for x in report),'source_mapping_counts':dict(collections.Counter(x['source_mapping'] for x in self.defs.values())),'no_guessed_fields':True,'new_delivery_directory':False,'files_index':report}
  gz(self.out/'type-definitions.json.gz',list(self.defs.values()));gz(self.out/'game-named-symbols.json.gz',list(self.names.values()));js(self.out/'report.json',data)
  print(json.dumps({k:v for k,v in data.items() if k!='files_index'},ensure_ascii=False),flush=True);return data
 def run(self):self.definitions();self.named();self.observed();return self.render()

def snapshot():return {p.relative_to(PROJECT).as_posix():sha(p) for p in (PROJECT/'reconstruction').rglob('*') if p.is_file() and (p.suffix in ('.rs','.toml') or p.name=='Cargo.lock')}
def main():
 before=snapshot();js(ROOT/'_pdb'/'production-source-snapshot-before-structures.json',before)
 args=sys.argv[1:];observe_only='--observe-only' in args;versions=[v for v in args if v!='--observe-only'] or ['A','B','C'];results=[]
 for v in versions:
  recovery=Recovery(v)
  if observe_only:
   recovery.observe_only=True;recovery.files.clear();recovery.names={x['name']:x for x in json.load(gzip.open(recovery.out/'game-named-symbols.json.gz','rt'))};recovery.observed();results.append(recovery.render())
  else:results.append(recovery.run())
 assert before==snapshot(),'Production sources changed during recovery'
 published=[json.loads((ROOT/'_pdb'/v/'structures'/'report.json').read_text()) for v in ['A','B','C'] if (ROOT/'_pdb'/v/'structures'/'report.json').exists()]
 js(ROOT/'pdb-structures-report.json',{'versions':published,'production_project_modified':False,'existing_ABC_only':True,'target_executed':False})
if __name__=='__main__':main()
