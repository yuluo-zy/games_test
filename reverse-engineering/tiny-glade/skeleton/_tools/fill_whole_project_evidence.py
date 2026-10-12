"""Attach retained project-wide evidence without fabricating source definitions.

Numeric layout facts below are bounded existing observations. Other evidence links
and asset JSON schemas never become native Rust fields or cross-version claims.
"""
from __future__ import annotations
import collections,gzip,hashlib,json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];PROJECT=ROOT.parent
START='// === WHOLE PROJECT EVIDENCE V1 BEGIN ===';END='// === WHOLE PROJECT EVIDENCE V1 END ==='
PDB_START='// === PDB STRUCTURES V1 BEGIN ===';PDB_END='// === PDB STRUCTURES V1 END ==='
NAME=re.compile(r'(?<![\w:])([a-z_][a-z_0-9]*(?:::[A-Za-z_][A-Za-z_0-9$]*)*::[A-Z][A-Za-z_0-9]*)(?![A-Za-z_0-9])')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write_json(p,x):
 p.parent.mkdir(parents=True,exist_ok=True);temp=p.with_name(p.name+'.tmp');temp.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n');temp.replace(p)
def write_gz(p,x):
 p.parent.mkdir(parents=True,exist_ok=True);temp=p.with_name(p.name+'.tmp')
 with gzip.open(temp,'wt',compresslevel=1) as f:json.dump(x,f,ensure_ascii=False,separators=(',',':'))
 temp.replace(p)
def load(p):return json.load(gzip.open(p,'rt'))
def evidence(rel):return {'path':rel,'sha256':sha(PROJECT/rel)}
def snapshots():return {p.relative_to(PROJECT).as_posix():sha(p) for p in (PROJECT/'reconstruction').rglob('*') if p.is_file() and (p.suffix in ('.rs','.toml') or p.name=='Cargo.lock')}
class Version:
 def __init__(self,v):
  self.v=v;self.names={x['name']:x for x in load(ROOT/'_pdb'/v/'structures/game-named-symbols.json.gz')};self.files={};self.report=json.loads((ROOT/'_pdb'/v/'structures/report.json').read_text());self.actual={p.relative_to(ROOT/v).as_posix() for p in (ROOT/v).rglob('*.rs')};self.catalog={};self.cargo=[]
  for path in self.actual:self.files[path]={'version':v,'path':path,'native_partial_layouts':[],'serialized_schema_observations':[],'evidence_links':[],'unknowns':['unrecorded original member names/types','complete original struct size/alignment/repr','author source text','runtime system behavior not proved by metadata']}
  for p in (ROOT/v).rglob('Cargo.toml'):
   text=p.read_text();m=re.search(r'(?m)^name\s*=\s*"([^"]+)"',text)
   self.cargo.append({'path':p.parent.relative_to(ROOT/v).as_posix(),'package':m[1] if m else 'unknown_workspace_or_empty_manifest'})
 def add(self,path,name,fields,sources,**extra):
  if path not in self.files:raise RuntimeError(f'Missing corresponding source {self.v}/{path}')
  x={'baseline':self.v,'name':name,'status':'retained_native_partial_observation','fields':fields,'complete_size':'unknown','alignment':'unknown','rust_repr':'unknown','evidence':[evidence(p) for p in sources],**extra};self.files[path]['native_partial_layouts'].append(x)
 def observations(self):
  dec='evidence/building-decorator-info/findings.md';offset={'A':0x21,'B':0x22,'C':0x32}[self.v]
  self.add('crates/country-core/src/resources/walls/decorator.rs','DecoratorInfo destination accessor (version specific)',[{'offset':offset,'read_bytes':1,'original_name':'unknown','type':'destination niche/tag byte; full enum ABI unknown'}],[dec,'evidence/building-decorator-info/accessor-three-version-symbols.json'],wall_accessor_return_offset=0x10 if self.v=='C' else 0,limitations=['B full destination payload does not transfer to A/C','shared address preferred aliases are not definition ownership'])
  if self.v=='C':
   sources=['reconstruction/critters/sheep_pet.rs','evidence/toolchain-pet-pilot-2026-10-11/critters-findings.md','evidence/toolchain-pet-pilot-2026-10-11/critters-C-pet-verification.json']
   fields=[{'offset':off,'read_bytes':width,'type':ty,'original_name':'unknown','observed_role':role} for off,width,ty,role in [(0x18,4,'u32','pet phase'),(0x1c,4,'f32','pet progress'),(0x20,8,'u64','pet event membership target'),(0x58,4,'f32','pet amplitude; being_petted ordered > 0.01'),(0x60,1,'u8','observed control byte; other consumers unknown')]]
   for path in ['crates/country-core/src/resources/critters/sheep.rs','crates/systems/critters/src/sheep/sheep_pet_animation.rs']:
    self.add(path,'Sheep C pet state projection',fields,sources,observed_storage_span=104,phase_observations=[{'value':i,'original_variant_name':'unknown','semantic_label':n} for i,n in enumerate(['idle','rise','hold','fade','complete'])],source_mapping='original Sheep source / pet consumer; projection is not complete original ABI',limitations=['original startup/ID constructor, UI event sender and full walk/renderer unknown','semantic phase labels are reconstructed labels, not original enum variant names'])
  if self.v!='B':return
  # Exact serialized anchor names are established in the retained native serializers.
  fields=[{'offset':off,'read_bytes':width,'type':ty,'original_name':name,'name_basis':basis} for off,width,ty,name,basis in [(0,8,'two f32 observed values','coord','native WallAttachment serializer'),(8,8,'WallId u64','anchor.platform','native anchor serializer'),(0x10,4,'f32','anchor.wall_height_relative_y','native anchor serializer'),(0x14,12,'Vec3 observed values','anchor.anchor_ws','native anchor serializer'),(0x20,1,'WallCornerAttachment observed byte','anchor.corner','native serializer/consumer'),(0x21,1,'bool observed byte','anchor.is_bottom_door','native serializer'),(0x22,1,'WallAttachmentSide observed byte','anchor.side','native serializer; destination niche overlap'),(0x2c,4,'u32 copied insertion key','DecoratorId','placement native consumer'),(0x44,1,'u8 read','rank','retained field observation'),(0x45,1,'u8 store','unknown','DEV demo flag false5/true0'),(0x46,1,'DecoratorType observed byte','unknown','type tag consumer'),(0x47,1,'u8 store','unknown','caller R8 byte; semantics unknown')]]
  self.add('crates/country-core/src/resources/walls/decorator.rs','DecoratorInfo B anchor/placement fields',fields,[dec],observed_destination_tags={'wall_side_0':0,'wall_side_1':1,'roof':2,'terrain':3,'stair':4},observed_subtype_tags={'cottage_window':0x38,'gothic_window':0x39,'arrow_slit':0x3a},tag_name_status='consumer semantics; original enum variant spellings unknown',uninterpreted_stores=['+0x28','+0x30..+0x43'])
  terrain='evidence/terrain-editing/findings.md';second='evidence/terrain-editing/second-batch-findings.md'
  self.add('crates/country-core/src/resources/terrain_height.rs','TerrainHeightsData B readback projection',[{'offset':0x50,'read_bytes':4,'type':'f32','original_name':'unknown','observed_role':'lower height boundary'}, {'offset':0x54,'read_bytes':4,'type':'f32','original_name':'unknown','observed_role':'upper height boundary'}],[terrain,'evidence/terrain-editing/native-provenance.json'],observed_data_bytes=0x80000,dirty_mask={'words':64,'word_type':'u64','span_bytes':0x200,'original_definition_layout':'unknown'},limitations=['GPU/CPU readback contract, not invented CPU brush/GPU implementation','tile-to-world spatial mapping and full resource size unknown'])
  self.add('crates/country-core/src/resources/terrain_editing.rs','TerrainEditingData B new stroke counter',[{'offset':0x38,'read_bytes':4,'type':'u32 read','original_name':'unknown','observed_role':'next stroke counter; inserted key is prior value'}],[terrain],upper_bound=0x3fffff)
  candidates=[p for p in self.actual if p.endswith('/edit_terrain_stroke.rs')]
  for path in candidates:
   self.add(path,'EditTerrainStrokeCmd B observed channel record',[{'offset':off,'read_bytes':width,'type':ty,'original_name':'unknown','observed_role':role} for off,width,ty,role in [(0,4,'operation code','falloff operation'),(4,4,'f32','falloff value'),(8,4,'operation code','height operation'),(12,4,'f32','height value'),(16,12,'three zeroed f32-sized words','position field span; full variant semantics unknown'),(28,4,'u32','stroke index'),(32,8,'u64 event metadata','EventInstance id; library channel observation only')]],[second,'evidence/terrain-editing/B-edit-replay-verification.json'],observed_event_instance_span=40,operation_values={'keep':0,'absolute':1,'delta':2},operation_names='consumer semantic labels; original enum variant names unknown',limitations=['channel-specific EventInstance observation; no Bevy storage reimplementation'])
  camera='evidence/parallel-camera/B-constructed-semantic-state.json';native='evidence/parallel-camera/B-constructor-native.json'
  self.add('crates/country-core/src/systems/main_camera.rs','MainCamera B constructor observed state regions',[],[camera,native,'evidence/parallel-camera/integration.md'],regions=load_json(camera),limitations=['observed initialized regions/bytes; complete field names and camera semantics unknown','allocator and host-libm bridges retained in original report','B data do not establish A/C layouts'])
 def retained_evidence(self):
  # Retained type/schema provenance is mapped by explicit source paths or exact
  # PDB names. Links do not silently promote textual mentions into member fields.
  selected=[]
  for p in (PROJECT/'evidence').rglob('*'):
   if not p.is_file() or p.suffix not in ('.json','.md'):continue
   rel=p.relative_to(PROJECT).as_posix()
   if '/skills-snapshot/' in rel or '/toolchain-revalidation/' in rel or '/pack-' in rel or '/payloads/' in rel:continue
   if p.suffix=='.md' and any(k in p.name for k in ('findings','integration')):selected.append(p)
   elif p.suffix=='.json' and (p.name=='report.json' or any(k in p.name for k in ('schema','layout','field','owner-map','source-ownership','source-provenance','registration-types','component-query-types'))):selected.append(p)
  original_sources=set(self.actual)
  for p in sorted(selected):
   if p.stat().st_size>2*1024*1024:continue
   rel=p.relative_to(PROJECT).as_posix();text=p.read_text();version_parts={x for x in re.findall(r'(?:^|/)([ABC])(?:[-_/]|\.)',rel)}
   if version_parts and self.v not in version_parts:continue
   mapping=collections.defaultdict(list)
   for name in {m.group(1) for m in NAME.finditer(text)}:
    if name not in self.names:continue
    for target in self.names[name]['target_files']:
     if target in original_sources and not target.startswith('_unmapped/'):mapping[target].append({'kind':'exact_PDB_named_entity_mention','name':name,'status':'evidence_association; original field ownership unknown'})
   for candidate in re.findall(r'(?:crates|rhapsody2|shabriri)/[A-Za-z0-9_./-]+\.rs',text):
    if candidate in original_sources:mapping[candidate].append({'kind':'explicit_source_path_mention','path':candidate,'status':'source mention; exact assertion applicability remains version-scoped'})
   if not mapping:continue
   eid=hashlib.sha256(rel.encode()).hexdigest()[:20]
   self.catalog[eid]={'id':eid,**evidence(rel),'version_scope':'explicit '+','.join(sorted(version_parts)) if version_parts else 'mixed_or_unspecified; applicability requires per-version original evidence','kind':'existing_schema/type/source_evidence','content_format':p.suffix,'field_promotion':'none; links are not native layout assertions','historical_or_failed':any(x in rel for x in ['failure','initial','rejected','pre-lock'])}
   for path,basis in mapping.items():self.files[path]['evidence_links'].append({'evidence_id':eid,'basis':basis})
 def resources(self):
  if self.v!='B':return
  commands=load_json('evidence/resources/command-schemas.json')
  ownership=load_json('reconstruction/history/observed-history-schema.json')
  tag_files={'WallNewSystem':'wall_compound_edit','Path':'path_edit','Terrain':'terrain_edit','Roof':'roof_edit','DecoratorV5':'decorator_edit','Garden':'garden_edit','Color':'color_edit','Tree':'tree_edit','Wall':'wall_edit','Water':'water_edit','Stairs':'stairs_history_edit','Cobblestone2':'cobblestone_edit','Cobblestone1':'cobblestone_edit'}
  for tag,data in commands['command_examples'].items():
   stem=tag_files[tag];targets=[p for p in self.actual if p.startswith('crates/country-core/src/resources/history/') and p.endswith('/'+stem+'.rs')]
   if not targets:targets=['crates/country-core/src/resources/history/mod.rs']
   for path in targets:
    if path not in self.files:continue
    schemas={k:v for k,v in ownership['path_schema'].items() if k=='/commands/'+tag or k.startswith('/commands/'+tag+'/')}
    self.files[path]['serialized_schema_observations'].append({'baseline':'B','name':tag+' history serialized observations','status':'shipped_asset_JSON_schema; not native struct layout','original_type_definition':'unknown','field_offsets':'unknown','Rust_numeric_types':'unknown','data':data,'observed_paths':schemas,'evidence':[evidence('evidence/resources/command-schemas.json'),evidence('evidence/resources/resources-summary.json'),evidence('reconstruction/history/observed-history-schema.json')],'limits':commands['limitations']})
  resources=load_json('evidence/resources/json-schemas.json')
  routes=[('assets/data/rocky_terrain.json',['crates/country-core/src/resources/rocky_terrain.rs']),('assets/colors.json',['crates/country-core/src/resources/color.rs']),('assets/glade/',['crates/country-core/src/resources/glade_settings.rs']),('assets/tod/',['crates/country-core/src/resources/time_of_day.rs']),('assets/tree/',['crates/country-core/src/resources/tree_storage.rs'])]
  for item in resources['files']:
   asset=item['path'];targets=[]
   for prefix,paths in routes:
    if asset.startswith(prefix):targets+=paths
   # Mesh metadata belongs to resource schema references, not generated game vertex structs.
   if 'attributes' in item.get('keys',[]):targets+=['crates/country-core/src/resources/render/mesh_atlas_library.rs']
   for path in sorted(set(targets)):
    if path not in self.files:continue
    self.files[path]['serialized_schema_observations'].append({'baseline':'B','name':asset,'status':'observed_shipped_resource_JSON_schema; native type/file binding candidate','original_type_definition':'unknown','field_offsets':'unknown','Rust_numeric_types':'unknown','data':item,'evidence':[evidence('evidence/resources/json-schemas.json'),evidence('evidence/resources/resources-summary.json')],'limits':['JSON key/type observations do not prove Rust members, enum discriminants, missing defaults, or A/C resource compatibility']})
 def run(self):
  self.observations();self.retained_evidence();self.resources();index=[]
  struct_files={x['path']:x for x in self.report['files_index']}
  for path,p in sorted(self.files.items()):
   source=ROOT/self.v/path;old=source.read_text()
   if START in old:
    assert END in old;head,tail=old.split(START,1);_,after=tail.split(END,1);old=head.rstrip()+'\n'+after.lstrip('\n')
   lines=[START,'// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。','// 完整附档: '+source.name+'.project-evidence.json.gz','// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。']
   for native in p['native_partial_layouts']:
    lines += ['// [版本 '+self.v+' 原指令局部结构] '+native['name']]
    for k,value in native.items():
     if k!='fields':lines.append('// '+k+': '+json.dumps(value,ensure_ascii=False))
    for field in native['fields']:lines.append('// [观察字段，未确认的作者字段名=unknown] '+json.dumps(field,ensure_ascii=False))
   for s in p['serialized_schema_observations']:
    lines += ['// [资源/历史 JSON schema，不是内存布局] '+s['name'],'// 全部实际键和数据形态: '+json.dumps(s.get('observed_paths',s['data']),ensure_ascii=False),'// offsets / Rust numeric types / missing defaults = unknown。']
   for link in p['evidence_links']:
    e=self.catalog[link['evidence_id']];lines.append('// [已有模块证据关联；不自动成为字段事实] '+e['path']+'; '+e['version_scope']+'; '+json.dumps(link['basis'],ensure_ascii=False))
   if not p['native_partial_layouts']:lines.append('// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。')
   lines.append('// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。');lines.append(END)
   block='\n'.join(lines)+'\n';marker='// === PDB ENRICHMENT V1 BEGIN ==='
   if marker in old:at=old.index(marker);new=old[:at].rstrip()+'\n\n'+block+'\n'+old[at:]
   else:new=old.rstrip()+'\n\n'+block
   tmp=source.with_name(source.name+'.project-evidence.tmp');tmp.write_text(new);tmp.replace(source)
   p['evidence_catalog']=f'_pdb/{self.v}/structures/project-evidence-catalog.json.gz';write_gz(source.with_suffix('.rs.project-evidence.json.gz'),p)
   meta=struct_files[path];index.append({'path':path,'PDB_definition_records':meta['definitions'],'PDB_named_entities':meta['named_symbol_entities'],'prior_native_observations':meta['observed_structures'],'new_native_partial_layouts':len(p['native_partial_layouts']),'new_native_field_observations':sum(len(x['fields']) for x in p['native_partial_layouts']),'serialized_schemas':len(p['serialized_schema_observations']),'evidence_links':len(p['evidence_links']),'unresolved_layout':True,'metadata_file':path+'.project-evidence.json.gz','companion_sha256':sha(source.with_suffix('.rs.project-evidence.json.gz')),'block_sha256':hashlib.sha256(block.encode()).hexdigest()})
  # Package grouping follows the deepest existing manifest; no roof-only filter.
  packages=collections.defaultdict(lambda:collections.Counter());file_groups=[]
  for f in index:
   owners=[c for c in self.cargo if f['path'].startswith(c['path']+'/')];owner=max(owners,key=lambda x:len(x['path'])) if owners else {'path':f['path'].split('/')[0],'package':'metadata_or_unknown_owner'};group=owner['path'];c=packages[group];c['files']+=1
   for k in ['PDB_definition_records','PDB_named_entities','prior_native_observations','new_native_partial_layouts','new_native_field_observations','serialized_schemas','evidence_links']:c[k]+=f[k]
   file_groups.append({**f,'package_path':group,'package_name':owner['package']})
  result={'version':self.v,'scope':'Entire existing version project, all current Rust files and original/local/generated/reference/unmapped modules','source_files':len(index),'package_groups':len(packages),'all_current_files_checked':set(struct_files)==self.actual,'new_native_partial_layouts':sum(x['new_native_partial_layouts'] for x in index),'new_native_field_observations':sum(x['new_native_field_observations'] for x in index),'serialized_schema_observations':sum(x['serialized_schemas'] for x in index),'evidence_links':sum(x['evidence_links'] for x in index),'unknown_is_not_complete_recovery':True,'packages':{k:dict(v) for k,v in sorted(packages.items())},'files':file_groups}
  assert result['all_current_files_checked']
  write_json(ROOT/'_pdb'/self.v/'structures/project-coverage.json',result);write_gz(ROOT/'_pdb'/self.v/'structures/project-evidence-catalog.json.gz',self.catalog)
  print(json.dumps({k:v for k,v in result.items() if k not in ['packages','files']},ensure_ascii=False),flush=True);return result

def load_json(rel):return json.loads((PROJECT/rel).read_text())
def main():
 before=snapshots();results=[Version(v).run() for v in 'ABC'];assert before==snapshots()
 write_json(ROOT/'pdb-whole-project-coverage.json',{'scope':'whole A/B/C project metadata + retained evidence','versions':results,'production_project_modified':False,'new_delivery_directory':False,'full_game_recovered':False})
 rows=['# A/B/C 全项目结构覆盖核对','','此清单按整个现有项目逐文件核对。`unknown` 为证据缺口；文件经过检查不代表其类型布局或游戏实现已经恢复。','', '| 版本 | 现有 .rs 文件 | 模块分组 | 新增局部布局 | 新增字段观察 | 资源/历史 schema | 已有证据关联 |','|---|---:|---:|---:|---:|---:|---:|']
 for r in results:rows.append('| '+r['version']+' | '+' | '.join(str(r[k]) for k in ['source_files','package_groups','new_native_partial_layouts','new_native_field_observations','serialized_schema_observations','evidence_links'])+' |')
 for r in results:
  rows+=['','## '+r['version'],'','| 项目模块 | 文件数 | PDB 命名实体 | PDB 定义 | 新增局部布局 | Schema | 证据关联 |','|---|---:|---:|---:|---:|---:|---:|']
  for name,c in r['packages'].items():rows.append('| '+name+' | '+' | '.join(str(c.get(k,0)) for k in ['files','PDB_named_entities','PDB_definition_records','new_native_partial_layouts','serialized_schemas','evidence_links'])+' |')
 rows+=['','逐文件完整状态在 `pdb-whole-project-coverage.json` 和各版本 `_pdb/<版本>/structures/project-coverage.json`。可确认的 PDB 结构字段/枚举在 `PDB STRUCTURES V1`，已有模块证据、资源 schema 和新增局部原指令字段在 `WHOLE PROJECT EVIDENCE V1`。','', 'PDB C/C++ 定义记录含第三方依赖，不能当作已恢复的游戏结构。通用库实现、完整 Rust ABI、缺失字段/默认值和原作者源码仍没有补造。B 原指令与资源信息未套用到 A/C。']
 (ROOT/'WHOLE_PROJECT_COVERAGE.md').write_text('\n'.join(rows)+'\n')
if __name__=='__main__':main()
