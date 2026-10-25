"""Audit known original business interfaces; do not treat decompilation as work."""
from pathlib import Path
import json,re,hashlib,collections
OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[2]
PUBLIC=ROOT/'evidence/render-architecture/public-symbols.json'
SRC=ROOT/'reconstruction/shared-foundation/crates/utils/src'
delivery=json.loads((OUT/'delivery.json').read_text(encoding='utf-8'))
symbols=json.loads(PUBLIC.read_text(encoding='utf-8'))
raw=[s for s in symbols if s.get('va') and s.get('demangled','').startswith(('utils::geometry::','utils::curve::','utils::components::transform::'))]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def canon(name):
 name=re.sub(r'::h[0-9a-f]{16}.*$','',name)
 name=name.replace('Curve$LT$T$C$CSem$GT$','Curve<T,CSem>').replace('Curve$LT$glam..f32..vec2..Vec2$C$CSem$GT$','Curve2<CSem>').replace('Curve$LT$glam..f32..vec3..Vec3$C$utils..curve..CurveCoordSemantic2D$GT$','Curve3XZ')
 name=name.replace('CurveInternalCoord$LT$CSem$GT$','CurveInternalCoord<CSem>').replace('CurveU$LT$CSem$GT$','CurveU<CSem>').replace('CurveSlice$LT$T$C$Points$C$CSem$C$RemapT$C$RemapF$GT$','CurveSlice<T,Points,CSem,RemapT,RemapF>')
 # Skeleton IPI names place real type arguments after the method; public names
 # place escaped arguments after the owner. Strip both, then restore a stable
 # generic owner key while retaining every typed instance as evidence.
 depth=0;out=[]
 for char in name:
  if char=='<':depth+=1
  elif char=='>':depth=max(0,depth-1)
  elif depth==0:out.append(char)
 name=''.join(out)
 name=name.replace('utils::curve::Curve::','utils::curve::Curve<T,CSem>::')
 name=name.replace('utils::curve::CurveInternalCoord::','utils::curve::CurveInternalCoord<CSem>::')
 name=name.replace('utils::curve::curve_u::CurveU::','utils::curve::curve_u::CurveU<CSem>::')
 name=name.replace('utils::curve::curve_slice::CurveSlice::','utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::')
 if name in {'utils::curve::Curve<T,CSem>::project_point','utils::curve::Curve<T,CSem>::to_curve3'}:name=name.replace('Curve<T,CSem>','Curve2<CSem>')
 if name=='utils::curve::Curve<T,CSem>::into_curve3':name=name.replace('Curve<T,CSem>','Curve3XZ')
 name=name.replace('utils::curve::Curve2::','utils::curve::Curve2<CSem>::')
 # Public trait spelling and IPI impl spelling refer to the same derived
 # serialization interface; retain policy uncertainty but merge this alias.
 if '_::_$LT$impl$u20$serde..ser..Serialize$u20$for$u20$'in name:
  name=name.split('_::_$LT$',1)[0]+'_::impl$0::serialize'
 return name
def source_for(name):
 if '::components::'in name:return 'components/transform.rs'
 if '::curve::curve_slice::'in name:return 'curve/curve_slice.rs'
 if '::curve::curve_u_circular::'in name:return 'curve/curve_u_circular.rs'
 if '::curve::curve_u::'in name:return 'curve/curve_u.rs'
 if '::curve::'in name:return 'curve/mod.rs'
 suffix=name.split('utils::geometry::',1)[1]
 if '::'not in suffix:return 'geometry/mod.rs'
 parts=suffix.split('::')
 if parts[0]=='triangle_triangle':return 'geometry/triangle_triangle/'+parts[1]+'.rs'
 return 'geometry/'+parts[0]+'.rs'
def method(name):return name.rsplit('::',1)[1]
# These are explicitly reviewed against the original (not all names decompiled).
analyzed_only={
 'utils::curve::Curve<T,CSem>::try_extend':'Original mutates/appends the owned point vector then rebuilds with duplicate rejection; no recovered mutating API yet.',
 'utils::curve::Curve<T,CSem>::try_resample_approx_pt_count':'Reviewed spacing/retained-feature and endpoint append paths; final algorithm not reconstructed.',
 'utils::curve::Curve<T,CSem>::is_on_right_side':'Reviewed nearest-segment and normalized cross-side query; no implementation.',
 'utils::geometry::circle::Circle2d::as_curve2':'Reviewed collect→curve build call; whether collection closes the final segment remains unresolved.',
 'utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners':'Reviewed exact-corner/edge dispatch and transformed corner normal sums; no implementation.',
 'utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge':'Reviewed inverse footprint and hypot branch; not implemented.',
 'utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge':'Reviewed inverse footprint/hypot/inside branch; not implemented.',
}
# Exact emitted entries exercised by fixtures. Generic variants are not all
# validated merely because the shared interface has one exercised instance.
native={
 '0x140c9ce00':('native-fixtures.json','look_at',100,'bits','complete_declared_business_input_case'),
 '0x140c9d0d0':('edge-fixtures.json','transform_abs_diff_eq',90,'bool','component/tolerance/NaN cases'),
 '0x141469430':('native-fixtures.json','curves2',41,'bits','Curve2 constructor'),
 '0x141269ce0':('native-fixtures.json','curves3',40,'bits','Curve3XYZ constructor'),
 '0x1412699d0':('native-fixtures.json','curves3',40,'bits','Curve3XZ constructor'),
 '0x141467ce0':('native-fixtures.json','samples',328,'bits','Curve2 coord'),
 '0x141467270':('native-fixtures.json','samples',328,'bits','Curve2 position'),
 '0x141468f90':('native-fixtures.json','samples',328,'bits','Curve2 tangent'),
 '0x14146a060':('native-fixtures.json','nearest',328,'bits','Curve2 nearest coord'),
 '0x141469a20':('native-fixtures.json','nearest',328,'bits','Curve2 nearest u'),
 '0x140154b60':('geometry-fixtures.json','slice2',40,'bits','Curve2 try_slice'),
 '0x1411f8750':('geometry-fixtures.json','circular_slice2',20,'bits','Curve2 circular_slice'),
 '0x14123afa0':('geometry-fixtures.json','ray_triangle',82,'bits','2-sided boundaries and epsilon'),
 '0x14123b230':('geometry-fixtures.json','ray_sphere',94,'bits','tangent/negative root/non-unit ray'),
 '0x1416f34a0':('geometry-fixtures.json','uv',66,'bits','first UV triangle and boundaries'),
 '0x140c985a0':('geometry-fixtures.json','rect3',50,'bits','Rectangle3d contains'),
 '0x140c98660':('geometry-fixtures.json','rect3',50,'bits','Rectangle3d extent'),
 '0x140c98690':('geometry-fixtures.json','rect3',50,'bits','Rectangle3d points'),
 '0x140c98620':('geometry-fixtures.json','rect3',50,'bits','Rectangle3d expand'),
 '0x140c975a0':('boundary-fixtures.json','circle',80,'bits','position'),
 '0x140c978e0':('boundary-fixtures.json','circle',80,'bits','normal'),
 '0x140c97660':('boundary-fixtures.json','circle',80,'bits','tangent'),
 '0x140c976f0':('boundary-fixtures.json','circle',80,'bits','closest u / unnormalizable 0'),
 '0x140c99800':('boundary-fixtures.json','rectangle',80,'bits','position'),
 '0x140c9b420':('boundary-fixtures.json','rectangle',80,'bool','SAT overlaps'),
 '0x141fe0750':('edge-fixtures.json','threshold_crossings',30,'event_projection','RowTrimmerSink callback bodies intercepted; full spans unvalidated'),
 '0x141240860':('linear-fixtures.json','aabb3',70,'bits','classify ray'),
 '0x141240570':('linear-fixtures.json','aabb3',70,'bits','intersect segment'),
 '0x1412803f0':('linear-fixtures.json','plane',70,'bits','plane min_t'),
 '0x14261f470':('linear-fixtures.json','line',70,'bits','closed line intervals'),
 '0x1416c2f20':('linear-fixtures.json','barycentric',70,'bits','3 weights'),
 '0x141fdfd80':('triangle-fixtures.json','triangle_triangle',257,'ordered_float_tolerance_3e-5','Point/Segment/Coplanar/Disjoint; ordered endpoints'),
}
# Private cross/helpers are reached inside the original root entry; record that
# separately rather than pretending to have a direct standalone input corpus.
indirect={
 'utils::geometry::triangle_triangle::cross_intersect::cross_intersect':('triangle-fixtures.json',257,'ordered_float_tolerance_3e-5','indirect private callee of triangles_intersect'),
}
exclude=[];grouped={};closures=[]
for s in raw:
 n=canon(s['demangled'])
 if 'panic_cold_display'in n:reason='compiler-generated panic formatter; not private business interface'
 elif '_::_'in n and ('Serialize'in n or 'serialize'in n):reason=None # Keep generated private schema policy until ownership is resolved.
 elif any(c in n for c in ('closure$','{{closure}}')):reason='private closure; retained separately, not public/method denominator'
 else:reason=None
 if reason:exclude.append({'original_name':s['demangled'],'address':s['va'],'reason':reason});continue
 grouped.setdefault(n,[]).append(s)

# Original skeleton is only PDB metadata, never implementation evidence. Read
# only function headers assigned to each original definition source; inline
# dependencies printed underneath are not APIs of this file.
SKEL=ROOT/'skeleton/B/crates/utils/src'
skeleton_count=0;skeleton_files=[]
for f in delivery['source_files']:
 relative=f['original_source'].removeprefix('crates/utils/src/');p=SKEL/relative
 text=p.read_text(encoding='utf-8');skeleton_files.append({'path':str(p.relative_to(ROOT)).replace('\\','/'),'sha256':sha(p)})
 matches=list(re.finditer(r'^// 函数符号: (.+)$',text,re.M))
 for i,match in enumerate(matches):
  header=text[match.end():text.find('// BEGIN GENERATED DEPENDENCY ANNOTATIONS',match.end())];full=match.group(1);n=canon(full);skeleton_count+=1
  # Exclude only actual function-identity closures, not callbacks appearing in
  # the type argument list of a legitimate generic business method.
  identity=full.split('<',1)[0]
  metadata={'skeleton_name':full,'definition_source':f['original_source'],'artifact':str(p.relative_to(ROOT)).replace('\\','/'),'line':text.count('\n',0,match.start())+1,'record_forms':re.search(r'^// 记录形式: (.+)$',header,re.M).group(1)if re.search(r'^// 记录形式: (.+)$',header,re.M)else None,'source_line':re.search(r'^// 源码行: (.+)$',header,re.M).group(1)if re.search(r'^// 源码行: (.+)$',header,re.M)else None}
  location=re.search(r'^// 首条位置: (\{.*\})$',header,re.M)
  if location:
   metadata['first_location']=json.loads(location.group(1))
  rvas=re.search(r'^// 编译实例 RVAs: (.+)$',header,re.M)
  metadata['addresses']=[hex(0x140000000+int(v,16))for v in re.findall(r'0x[0-9a-f]+',rvas.group(1))]if rvas else[]
  if not identity.startswith('utils::'):exclude.append({'original_name':full,'reason':'definition namespace is library/other module despite occurrence under source dependency inventory','metadata':metadata});continue
  if '::closure$'in identity:
   closures.append({'identifier':n,'original_name':full,'business_analysis_started':False,'has_recovered_implementation':False,'status':'untouched_private_closure','metadata':metadata});continue
  if 'panic_cold_display'in identity:exclude.append({'original_name':full,'reason':'compiler panic formatter','metadata':metadata});continue
  row={'demangled':full,'symbol':None,'va':metadata['addresses'][0]if metadata['addresses']else None,'skeleton_metadata':metadata}
  grouped.setdefault(n,[]).append(row)

def declarations(path,m):
 code=path.read_text(encoding='utf-8');result=[]
 for match in re.finditer(r'pub\s+fn\s+'+re.escape(m)+r'\s*(?:<[^\n]*>)?\s*\([^)]*\)(?:\s*->[^\{;]+)?',code):
  prefix=code[:match.start()];impls=list(re.finditer(r'\bimpl[^\{]*\{',prefix));owner=impls[-1].group().rstrip('{').strip()if impls and prefix.count('{')>prefix.count('}')else 'free function'
  result.append({'declaration':' '.join(match.group().split()),'line':code.count('\n',0,match.start())+1,'owner_context':owner})
 return result

rows=[]
for name,originals in sorted(grouped.items()):
 metadata_sources={s['skeleton_metadata']['definition_source']for s in originals if s.get('skeleton_metadata')}
 relative=next(iter(metadata_sources)).removeprefix('crates/utils/src/')if len(metadata_sources)==1 else source_for(name)
 recovered_relative='curve/curve_slice.rs'if name.startswith('utils::curve::Curve<T,CSem>::')and method(name)in {'circular_slice','crashy_slice','slice_into','try_slice'}else relative
 path=SRC/recovered_relative;implementation=declarations(path,method(name))if path.exists()else[]
 owner_type=name.rsplit('::',1)[0].rsplit('::',1)[-1].split('<',1)[0]
 if re.match(r'^[A-Z]',owner_type):implementation=[d for d in implementation if re.search(r'\b'+re.escape(owner_type)+r'\b',d['owner_context'])]
 # Same address as Circle constructor is not evidence for the different
 # Segment2d owner, and a scalar field representation is not the private
 # serialized-as-curve3 adapter behavior.
 if 'Segment2d::'in name or 'curve2_serialized_as_curve3::'in name:implementation=[]
 if '::impl$'in name or 'CurveUBoundless::'in name:implementation=[] # Same method spelling is not evidence for another owner.
 helper_alias={'utils::geometry::triangle_triangle::utils::is_tri_degenerated':'degenerate','utils::geometry::triangle_triangle::utils::orient3d':'side','utils::geometry::triangle_triangle::utils::permute_tri_right':'canonicalize'}
 if name in helper_alias:
  code=path.read_text(encoding='utf-8');pattern=re.search(r'(?:pub\(crate\)\s+)?fn\s+'+helper_alias[name]+r'\s*\(',code)
  implementation=[{'declaration':'Private helper business rules are recovered under '+helper_alias[name]+'; standalone original signature not claimed','line':code.count('\n',0,pattern.start())+1,'owner_context':'private game helper','partial_surface_mapping':True}]if pattern else[]
 is_implemented=bool(implementation);analysed=is_implemented or name in analyzed_only
 matches=[(s,native[s['va']])for s in originals if s['va']in native]
 comparisons=[]
 seen_native=set()
 for s,(fixture,group,count,comparison,scope)in matches:
  if s['va']in seen_native:continue
  seen_native.add(s['va'])
  comparisons.append({'address':s['va'],'artifact':str((OUT/fixture).relative_to(ROOT)).replace('\\','/'),'cases':count,'comparison':comparison,'scope':scope,'direct_original_entry':True})
 if name in indirect:
  fixture,count,comparison,scope=indirect[name];comparisons.append({'artifact':str((OUT/fixture).relative_to(ROOT)).replace('\\','/'),'cases':count,'comparison':comparison,'scope':scope,'direct_original_entry':False})
 compared=bool(comparisons)
 status='native_compared'if compared else 'implemented_static'if is_implemented else 'analyzed_only'if analysed else 'untouched'
 addresses=sorted({a for s in originals for a in ([s['va']]if s['va']else[])+s.get('skeleton_metadata',{}).get('addresses',[])},key=lambda v:int(v,16))
 decompiled=[]
 for address in addresses:
  for directory in (OUT,OUT.parent/'geometry'):
   p=directory/(address[2:]+'.c')
   if p.exists():decompiled.append(str(p.relative_to(ROOT)).replace('\\','/'));break
 gaps=[]
 if not analysed:gaps.append('Only located/PDB/decompiled evidence; no business rule analysis or implementation.')
 elif name in analyzed_only:gaps.append(analyzed_only[name])
 elif not compared:gaps.append('Business candidate/static review exists; original entry differential not yet executed.')
 if '::curve::Curve<'in name:gaps.append('This merged generic interface has additional instantiations; raw PDB names do not resolve all T/CSem variants. Curve2-only comparison must not count Curve3 query variants as verified.')
 if any(c['comparison']=='event_projection'for c in comparisons):gaps.append('Native event projection only; full receiver/consumer behavior and span output are not validated.')
 if any('tolerance'in c['comparison']for c in comparisons):gaps.append('Ordered geometric endpoint comparison allows 3e-5 distance residual; not bit-identical.')
 if 'curve2_serialized_as_curve3::'in name:gaps.append('Private serialization conversion policy is genuine game work; not excluded as serde glue. Exact source-line mapping and wire behavior remain unanalysed.')
 if '::to_curve3'in name:gaps.append('Candidate exposes height mapping convenience; original generic mapping callback surface not fully matched.')
 if '::crashy_'in name and is_implemented:gaps.append('Underlying result-returning rule may be compared; this panic wrapper/error contract has no direct original fixture.')
 if name in helper_alias:gaps.append('Private helper strategy is implemented and root-callsite checked; original standalone inline signature is not exposed 1:1, no separate helper differential corpus.')
 file_record=next((f for f in delivery['source_files']if f['restored_file'].endswith('/'+relative)),None)
 schema_policy=('_::_'in name or '::_::'in name or '::impl$'in name)
 rows.append({'interface_id':name,'canonical_owner':name.rsplit('::',1)[0],'method':method(name),'original_source':'crates/utils/src/'+relative,'source_mapping_basis':'Definition header in original B skeleton; native public owner/callee evidence merged','baseline':'B','ownership':'private_or_generated_schema_trait_policy_requires_classification'if schema_policy else'private_game_business_interface','ownership_evidence':['tracking/original-Cargo.lock:8100','evidence/render-architecture/public-symbols.json','Original skeleton B function headers; dependency annotations excluded'], 'original_addresses':addresses,'pdb_records':[{'original_name':s['demangled'],'address':s['va'],'public_record':s.get('symbol'),'section':s.get('section'),'offset':s.get('offset'),'skeleton_metadata':s.get('skeleton_metadata')}for s in originals],'original_typed_signature_verified':False,'status':status,'business_analysis_started':analysed,'has_recovered_implementation':is_implemented,'has_original_comparison':compared,'decompiled_evidence_present':bool(decompiled),'decompiled_evidence':decompiled,'all_observed_instantiations_native_compared':False,'candidate_crate_exported':is_implemented,'candidate_consumer_wired':name in {'utils::curve::Curve<T,CSem>::get_coord_at_u','utils::curve::Curve<T,CSem>::get_pos_at_u','utils::curve::Curve<T,CSem>::get_tangent_at_u','utils::curve::Curve<T,CSem>::get_approx_u_from_pos','utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect'},'original_consumer_chain_verified':False,'integrated':False,'restored_file':str(path.relative_to(ROOT)).replace('\\','/')if is_implemented else None,'restored_declarations':implementation,'original_comparison':comparisons,'business_rules':file_record['business_rules']if analysed and file_record else[],'gaps':gaps})

counts=collections.Counter(r['status']for r in rows)
files=[]
for f in delivery['source_files']:
 rr=[r for r in rows if r['original_source']==f['original_source']]
 files.append({'original_source':f['original_source'],'known_interfaces':len(rr),'status_counts':dict(collections.Counter(r['status']for r in rr)),'unknown_inline_interfaces':True,'note':'No countable standalone public business API in current inventory; only private inline/callee or original source path is known'if not rr else None})
variants={
 'Curve2':{'point_type':'glam Vec2','arc_length':'XY','duplicate_check':'full XY','closest_projection':'XY','constructor_original_addresses':['0x141469430'],'native_constructor_compared':True,'query_comparison_scope':'coord/position/tangent/nearest/slice/circular_slice compared; resample/smooth/remap unimplemented'},
 'Curve3XYZ':{'point_type':'glam Vec3','arc_length':'XYZ','duplicate_check':'full XYZ','closest_projection':'XZ for inspected emitted nearest helper 0x141469e00','constructor_original_addresses':['0x141269ce0'],'native_constructor_compared':True,'query_comparison_scope':'constructor/points_u only; generic Vec3 query implementations compiled but not original-entry compared'},
 'Curve3XZ':{'point_type':'glam Vec3','arc_length':'XZ','duplicate_check':'full XYZ','closest_projection':'XZ','constructor_original_addresses':['0x1412699d0'],'native_constructor_compared':True,'query_comparison_scope':'constructor/points_u only; queries/semantic conversion not original-entry compared'},
 'Unresolved_other_instantiations':{'known':False,'note':'Other emitted generic hashes, including different strides, require TPI/inline/caller type resolution; do not infer ABI or completeness.'}}
report={'schema_version':1,'baseline':'B','audit_kind':'known_private_business_interface_inventory','scope':'components/transform.rs + curve4 files + geometry14 files','denominator_complete':False,'full_inline_interface_inventory_available':False,'public_input':{'artifact':str(PUBLIC.relative_to(ROOT)).replace('\\','/'),'sha256':sha(PUBLIC),'raw_matching_public_records':len(raw),'original_names_with_hash_removed':True,'generic_repetitions_merged':True,'different_owner_same_address_not_merged':True},'exclusions':exclude,'known_interface_count':len(rows),'status_counts':{k:counts[k]for k in ['untouched','analyzed_only','implemented_static','native_compared','integrated']},'independent_counts':{'business_analysis_started':sum(r['business_analysis_started']for r in rows),'has_recovered_implementation':sum(r['has_recovered_implementation']for r in rows),'has_original_comparison':sum(r['has_original_comparison']for r in rows),'original_consumer_chain_verified':sum(r['original_consumer_chain_verified']for r in rows),'decompiled_evidence_only_and_untouched':sum(r['status']=='untouched'and r['decompiled_evidence_present']for r in rows)},'strategy_variants':variants,'interfaces':rows,'files':files,'private_helpers_not_public_denominator':[{'identifier':'unnamed private triangle orientation helper','address':'0x142418dc0','inferred_source':'geometry/triangle_triangle/utils.rs','source_attribution_exact_line_verified':False,'analyzed':True,'implemented_as':'orient','native_compared_indirect':True,'evidence':['orient-result.json','142418dc0.c','triangle-fixtures.json'],'integrated':False}], 'private_closures_not_public_denominator':[],'inline_unknowns':[{'source':'curve/curve_u_circular.rs','reason':'PDB source path exists, no complete LLVM inline method inventory in this group; documentation stub is not interface recovery.'},{'source':'all19files','reason':'Current public-symbol inventory is not a complete source API inventory. Existing evidence/pdb-structure/procedures.json contains no utils rows and must not be treated as a full utils DBI/inline scan.'}],'excluded_other_versions':delivery['excluded_other_versions'],'count_policy':['PDB name or saved decompilation alone is untouched business work.','native_compared may cover only selected strategy variants; see per-interface evidence.','event projection and float tolerance are explicit separate validation qualifications.','integrated requires a verified original consumer chain; crate compile/export and local wrapper links do not suffice.','No overall recovery percentage is derived from this incomplete known API denominator.']}
report['skeleton_input']={'function_header_records':skeleton_count,'files':skeleton_files,'definition_source_filter':'Own original-source function headers only; underlying dependency annotations not counted','raw_jsonl_gz_archives_available_in_current_workspace':False}
report['private_closures_not_public_denominator']=closures
report['inline_unknowns']=[{'source':'all19files','reason':'407 original B function headers include observed inlinee declarations, but omitted/full DBI archives are unavailable; completeness of original source API still not established.'}]
report['strategy_variants']['Curve3Thresholded']={'point_type':'glam Vec3','semantic_type':'CurveCoordSemanticThresholded3D','known_from':'Original B skeleton typed instantiations (constructor/query/slice)','arc_length_policy':'not recovered; distinct from XYZ/XZ','native_constructor_compared':False,'status':'untouched_strategy; cannot substitute Curve3XYZ by assumption'}
report['ownership_counts']={'private_game_business_interfaces':sum(r['ownership']=='private_game_business_interface'for r in rows),'preserved_trait_schema_items_pending_ownership':sum(r['ownership']!='private_game_business_interface'for r in rows)}
report['status_counts_by_ownership']={ownership:dict(collections.Counter(r['status']for r in rows if r['ownership']==ownership))for ownership in {r['ownership']for r in rows}}
for r in rows:
 if 'utils::curve::Curve<'not in r['interface_id'] and 'utils::curve::Curve2<'not in r['interface_id'] and 'utils::curve::Curve3XZ'not in r['interface_id']:continue
 vv={}
 for record in r['pdb_records']:
  full=record['original_name'];params=full.split('<',1)[1].split(',',2)if '<'in full else[]
  variant=None
  if '::Curve2<'in r['interface_id']:variant='Curve2'
  elif '::Curve3XZ'in r['interface_id']:variant='Curve3XZ'
  elif len(params)>=2:
   if 'vec2::Vec2'in params[0]:variant='Curve2'
   elif 'vec3::Vec3'in params[0]:
    variant='Curve3Thresholded'if 'Thresholded3D'in params[1]else'Curve3XYZ'if 'Semantic3D'in params[1]else'Curve3XZ'if 'Semantic2D'in params[1]else None
  if variant:
   metadata=record.get('skeleton_metadata')or{};rr=vv.setdefault(variant,{'known_typed_records':[],'addresses':set()});rr['known_typed_records'].append({'name':full,'evidence':metadata.get('artifact'),'line':metadata.get('line'),'coordinate_argument':('CurveUBoundless'if 'CurveUBoundless<'in full else'CurveU'if'CurveU<'in full else'not_resolved')});rr['addresses'].update(metadata.get('addresses',[]));
   if record.get('address'):rr['addresses'].add(record['address'])
 r['strategy_variants']=[]
 for variant,v in vv.items():
  comparisons=[c for c in r['original_comparison']if c.get('address')in v['addresses']]
  implementation=r['has_recovered_implementation']and variant!='Curve3Thresholded'
  r['strategy_variants'].append({'strategy':variant,'known_typed_records':v['known_typed_records'],'addresses':sorted(v['addresses']),'has_recovered_implementation':implementation,'business_analysis_started':r['business_analysis_started']if variant!='Curve3Thresholded'else False,'has_original_comparison':bool(comparisons),'comparison':comparisons,'status':'native_compared'if comparisons else'implemented_static'if implementation else'untouched','gap':'Thresholded3D policy not analyzed or implemented'if variant=='Curve3Thresholded'else'This variant is not independently original-compared'if not comparisons else'Selected cases; not exhaustive source API acceptance'})
 r['generic_interface_variants_complete']=False
(OUT/'interface-coverage.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
md=['# components / curve / geometry：按原 B 接口核对','',f'合并 {len(raw)} 条 public 和原 B 骨架 **{skeleton_count} 条函数定义元数据**，目前已知 **{len(rows)} 个不同接口/待归属 trait-schema 策略项**。泛型 hash 实例合并；不同 owner 即使同地址仍分开。**inline 全集未知，不能作为完整分母。**','', '| 状态 | 已知接口数 | 定义 |','|---|---:|---|']
labels={'untouched':'完全未开始业务分析（只有名字/反编译也算这里）','analyzed_only':'已分析部分业务，尚无实现','implemented_static':'实现并静态核对，尚无对应原入口对照','native_compared':'已有原指令对照；变体、投影、容差仍分别限定','integrated':'原游戏消费者闭包已验证'}
for k in ['untouched','analyzed_only','implemented_static','native_compared','integrated']:md.append(f'| `{k}` | {counts[k]} | {labels[k]} |')
md.extend(['',f'其中 {report["independent_counts"]["decompiled_evidence_only_and_untouched"]} 个 untouched 接口虽有伪代码，但未形成业务分析，不能计已完成。','', '## 目录分布','', '| 原文件 | 已知接口 | 未触及 | 只分析 | 静态实现 | 原码对照 | 原消费者接线 |','|---|---:|---:|---:|---:|---:|---:|'])
for f in files:
 c=f['status_counts'];md.append('| `'+f['original_source']+'` | '+str(f['known_interfaces'])+' | '+' | '.join(str(c.get(k,0))for k in ['untouched','analyzed_only','implemented_static','native_compared','integrated'])+' |')
md.extend(['',f'326项中，明确私有函数/方法 {report["ownership_counts"]["private_game_business_interfaces"]} 项；{report["ownership_counts"]["preserved_trait_schema_items_pending_ownership"]} 项 trait/schema 策略尚待确认归属或生成方式，保留在分母，不以 trait 名字排除。','', '`curve_u_circular.rs` 已纳入9条 inline元数据（含待归属trait）；不能因没有public地址忽略。私有closure单列，不算公共/方法接口分母。C 专有 brace 不计入 B。','', '原 `curve_slice.rs` 的17项 CurveSlice/CurvePoints API 仍完全未触及；候选文件里实现的4个裁切方法原本属于 `Curve`，故记在原 `curve/mod.rs`，不能从候选文件同名推定原CurveSlice类已恢复。','','## 策略不能混算','','- Curve2：XY 长度、XY 重复点和 XY 近点；构造、查询、裁切已有原入口对照。','- Curve3XYZ：XYZ 长度、全 XYZ 重复点；本次原入口对照仅构造及 points_u，3D 查询不能借 Curve2 测试算完成。','- Curve3XZ：XZ 长度仍检查全 XYZ 重复点；同样仅构造及 points_u 已原入口对照。','- **Curve3Thresholded**：原骨架明确存在 CurveCoordSemanticThresholded3D，阈值度量策略尚未恢复，不能当XYZ或XZ处理。','- 其他 generic hash 的 T/CSem 尚未全部解出，逐地址保留，不能把全部泛型实例自动视为已验收。','- JSON每个合并Curve接口有单独strategy_variants子表；以具体变体的实施/对照状态为准，不把一次Curve2测试复制给3D。','','## 每个接口','', '| 原接口（合并 hash） | 状态 | 原地址 | 恢复实现 | 对照限定 / 缺口 |','|---|---|---|---|---|'])
for r in rows:
 compar='; '.join(c['scope']+' / '+c['comparison']for c in r['original_comparison'])
 detail=compar or ('; '.join(r['gaps'][:1]))
 md.append('| `'+r['interface_id']+'` | `'+r['status']+'` | '+', '.join('`'+v+'`'for v in r['original_addresses'])+' | '+('`'+r['restored_file']+'`'if r['restored_file']else'—')+' | '+detail.replace('|','/')+' |')
md.extend(['','## 真正尚未触及的接口',''])
for r in rows:
 if r['status']=='untouched':md.append('- `'+r['interface_id']+'`')
md.extend(['','## 库存边界与补充','',f'{len(raw)} 条原 public + {skeleton_count} 条骨架函数头中，仅确证panic/外部库胶水排除；私有closures {len(closures)} 条单列。trait/default/drop/serde若缺乏自动生成或库归属证据，保留为待归属项，避免漏掉策略。`curve2_serialized_as_curve3::serialize/deserialize` 是真正的私有转换策略，记为 untouched。','', '详细机器表包含独立 analysis/implementation/comparison/integration 布尔值、原完整名称/地址/原PDB记录、候选声明与行号、伪代码证据、比较限定和逐接口缺口。`get_functions` 或符号定位不等于分析完成。','', '当前没有原游戏消费者闭包通过验收的接口；WallSpace 的候选 bridge、Triangle 的候选 wrapper 和 crate 出口分别记录，不能代替原消费链验证。'])
(OUT/'interface-coverage.md').write_text('\n'.join(md)+'\n',encoding='utf-8')
print(json.dumps({'raw_public_records':len(raw),'excluded_records':len(exclude),'known_interfaces':len(rows),'status_counts':report['status_counts'],'independent_counts':report['independent_counts']},ensure_ascii=False))
