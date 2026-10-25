"""Read existing B LLVM/source skeleton + publics; no decompilation or writes to source."""
from pathlib import Path
from collections import Counter
import hashlib,json,re
OUT=Path(__file__).parent;ROOT=OUT.parents[2]
PDB_SRC=ROOT/'skeleton/B/crates/utils/src'
DELIVERY=json.loads((OUT/'delivery.json').read_text(encoding='utf8'))
MODULES={Path(f['original_file']).stem:f for f in DELIVERY['files']}
PUBLICS=[p for p in json.loads((ROOT/'evidence/render-architecture/public-symbols.json').read_text(encoding='utf8')) if 'utils::' in p['demangled'] or 'utils..' in p['demangled']]
MANIFEST=json.loads((OUT.parent/'utils-tabloid/native-manifest.json').read_text(encoding='utf8'))
BASE=0x140000000
def clean(name):
    name=re.sub(r'::h[0-9a-f]{16}.*$','',name)
    for a,b in {'$LT$':'<','$GT$':'>','$C$':',','$u20$':' ','$RF$':'&','$u5b$':'[','$u5d$':']','$u3b$':';','$u27$':"'"}.items():name=name.replace(a,b)
    name=name.replace('..','::').replace('_$','').strip('_')
    return name
def erase_generics(name):
    if name.startswith('<'):
        # Trait implementation's outer angle brackets are ownership, not a
        # generic argument. Preserve concrete type and trait with nested args erased.
        tail=name.rsplit('>::',1)
        if len(tail)==2:
            return '<'+erase_generics(tail[0][1:])+'>::'+erase_generics(tail[1])
    result=[];depth=0
    for c in name:
        if c=='<':depth+=1
        elif c=='>':depth=max(0,depth-1)
        elif not depth:result.append(c)
    return ''.join(result)
def canonical(name):return erase_generics(clean(name))
by_address={}
for p in PUBLICS:
    if p.get('va'):by_address.setdefault(int(p['va'],16),[]).append(p)
def own_public(name,module):
    n=clean(name)
    return (n.startswith(f'utils::{module}::') or n.startswith(f'<utils::{module}::') or f'as utils::{module}::' in n) and not n.startswith(('core::ptr::','core::ops::'))
records={};exclusions=[];closures=[];inputs=[]
for module in MODULES:
    p=PDB_SRC/(module+'.rs');text=p.read_text(encoding='utf8')
    inputs.append({'path':p.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
    parts=re.split(r'^// 函数符号: ',text,flags=re.M)
    for block in parts[1:]:
        name=block.splitlines()[0];owner_name=canonical(name)
        entries=[hex(BASE+int(a,16)) for a in re.findall(r'0x[0-9a-f]+',next(iter(re.findall(r'^// 编译实例 RVAs: (.*)$',block,re.M)),''))]
        aliases=[]
        for address in entries:
            aliases.extend(x for x in by_address.get(int(address,16),[]) if own_public(x['demangled'],module))
        alias_keys=sorted({canonical(x['demangled']) for x in aliases})
        # Use exact address bindings only. impl$N whose owner is unresolved stays
        # a distinct opaque owner; never guess it from a method's spelling.
        key=alias_keys[0] if len(alias_keys)==1 else owner_name
        provenance={'name':name,'source':p.relative_to(ROOT).as_posix(),'source_line_in_inventory':text[:text.find('// 函数符号: '+name)].count('\n')+1,'entries':entries,'record_kinds':re.findall(r'(compiled_procedure|inline_instance|inlinee_declaration)',block.split('// BEGIN GENERATED DEPENDENCY ANNOTATIONS')[0]),'ipi_ids':next(iter(re.findall(r'^// IPI IDs: (.*)$',block,re.M)),None),'original_source_line':next(iter(re.findall(r'^// 源码行: (.*)$',block,re.M)),None),'first_record_location':next(iter(re.findall(r'^// 首条位置: (.*)$',block,re.M)),None),'public_aliases':[{'name':a['demangled'],'address':a['va'],'symbol_record':a['symbol']} for a in aliases]}
        if 'closure$' in name:
            closures.append({'key':owner_name,'module':module,'reason':'private closure; tracked separately, not an API interface','provenance':provenance});continue
        leaf=owner_name.rsplit('::',1)[-1]
        generated_serde=(module in ['circular_f32','wall_space'] and '::_::impl$' in name)
        boilerplate=leaf in ['fmt','eq','hash','clone']
        system_glue=module=='timer' and '::_::impl$' in name and leaf!='init_state'
        # Do not discard authored/default/clone/schema behavior merely because
        # its method implements a trait. Their individual bodies remain triage
        # work until compiler-only forwarding is established.
        if system_glue:
            exclusions.append({'key':owner_name,'module':module,'reason':'generated Serde structural/field visitor glue' if generated_serde else 'Bevy SystemParam library/compiler forwarding (init_state game values retained separately)' if system_glue else 'compiler-derived formatting/equality/hash/clone container glue','provenance':provenance});continue
        row=records.setdefault(key,{'canonical_key':key,'module':module,'original_names':[],'original_entries':[],'pdb_provenance':[],'owner_type_resolved':'impl$' not in key,'generic_instances_merged':True})
        row['original_names'].append(name);row['original_entries']+=entries;row['pdb_provenance'].append(provenance)
        row['original_names'] += [a['demangled'] for a in aliases]

# Inline declarations and compiled instances can carry the same PDB original
# owner/method name even when only the compiled row has a public type binding.
# Merge that exact declaration identity; method spelling alone is not a binding.
declaration_groups={}
for key,row in records.items():
    for provenance in row['pdb_provenance']:
        identity=(row['module'],canonical(provenance['name']))
        declaration_groups.setdefault(identity,set()).add(key)
declaration_merges=[]
for identity,keys in declaration_groups.items():
    keys=[k for k in keys if k in records]
    if len(keys)<2:continue
    target=sorted(keys,key=lambda k:('impl$' in k,k))[0]
    for key in keys:
        if key==target:continue
        original=records.pop(key)
        for field in ['original_names','original_entries','pdb_provenance']:records[target][field]+=original[field]
        declaration_merges.append({'module':identity[0],'original_declaration':identity[1],'merged_key':key,'canonical_key':target,'basis':'same original PDB owner/method declaration after generic argument erasure; compiled public binding supplies canonical type'})

# Public entries not represented by an LLVM file binding are listed as additional
# observed interfaces, but are not merged with unresolved impl$N by name alone.
represented_addresses={a for r in records.values() for a in r['original_entries']}
for p in PUBLICS:
    if not p.get('va') or p['va'] in represented_addresses:continue
    for module in MODULES:
        if not own_public(p['demangled'],module):continue
        key=canonical(p['demangled'])
        if any(key in [canonical(n) for n in r['original_names']] for r in records.values()):continue
        # Public trait instances lacking any file binding may be the same
        # interface as an opaque inline impl. List as unresolved supplement,
        # not silently double count the observed denominator.
        exclusions.append({'key':key,'module':module,'reason':'public-only implementation defined by country_core storage owner, outside the 22 utils source files; tracked as consumer dependency, not utils API ownership','public_record':p})

for row in records.values():
    row['original_names']=sorted(set(row['original_names']));row['original_entries']=sorted(set(row['original_entries']))
    row['record_kinds']=sorted({x for p in row['pdb_provenance'] for x in p['record_kinds']})
    row['abi_verified']=False
    row['analysis_status']='untouched'
    row['candidate_implementation']=None
    row['business_analysis_evidence']=[]
    row['static_comparison']=False
    row['original_instruction_differential']=False
    row['consumer_wired']=False
    row['gaps']=['No per-interface business analysis recorded; PDB name/location is inventory only.']

def match(module,fragment):
    return [r for r in records.values() if r['module']==module and any(fragment in canonical(n) for n in r['original_names'])]
def mark(module,fragment,implementation,evidence,static=True,oracle=False,wired=False,gap=None):
    rows=match(module,fragment)
    if not rows:raise ValueError((module,fragment))
    for r in rows:
        r.update(analysis_status='analyzed_partial',candidate_implementation=implementation,business_analysis_evidence=evidence,static_comparison=static,original_instruction_differential=oracle,consumer_wired=wired,gaps=[gap or MODULES[module]['gaps'][0]])

def old_evidence(module,method):
    return ['../utils-tabloid/CONTRACT.md']+[f"../utils-tabloid/{r['assembly']}" for r in MANIFEST['functions'] if f'utils::{module}::' in r['name'] and canonical(r['name']).endswith('::'+method)]

for module,methods in {
 'angle_range':['AngleRange::clamp','AngleRange::from_min_and_len'],
 'camera':['CameraMatrices::eye_position','CameraMatrices::eye_direction','CameraMatrices::eye_right','CameraMatrices::aspect_ratio','CameraMatrices::xz_projective_rotation','CameraBodyMatrices::from_position_rotation','CameraProjectionParams::calc_matrices','CameraLens::new','CameraLens::immediately_set_vertical_fov_degrees'],
 'circular_f32':['CircularF32::new','CircularF32::set_value','CircularF32::as_f32','CircularF32::distance','CircularF32::lerp'],
 'easing_functions':['EasingFunction::apply','cubic_in_out'],
 'file':['date_time_string_for_filename','generate_backup_file_path','copy_file_to_backup'],
 'local_time':['init_local_time','system_time_to_local_time'],
 'loop_range':['LoopIdx::index','LoopIdx::raw_idx','LoopRange::len','LoopRange::is_empty','LoopRange::start','LoopRange::end','LoopRange::new_loop','LoopRange::new_line','LoopRange::subslice'],
 'name_hash':['NameHash::compute','NameHash::debug_name'],
 'rdp':['Rdp::new_with_opts'],
 'rgb9e5':['pack_rgb9e5'],
 'rng':['init_rng_u64','init_rng'],
 'subtractive_1d_boolean':['SubtractiveBoolean1D::new_range','SubtractiveBoolean1D::result','SubtractiveBoolean1D::subtract'],
 'threshold_range':['threshold_range'],
 'wall_space':['WallSpace::space_length','WallSpaceCoord::from_world_space','WallSpaceCoord::from_curve_internal_coord','WallSpaceCoord::to_world_space','WallSpaceCoord::get_pos_xz','WallSpaceCoord::get_tangent','WallSpaceCoord::with_y_offset','WallSpaceCoord::with_y'],
}.items():
    for name in methods:
        method=name.rsplit('::',1)[-1]
        oracle=(module,method) in {('angle_range','clamp'),('circular_f32','lerp'),('easing_functions','apply'),('loop_range','index'),('rgb9e5','pack_rgb9e5'),('rng','init_rng_u64'),('rdp','new_with_opts'),('subtractive_1d_boolean','subtract'),('camera','xz_projective_rotation'),('camera','calc_matrices')}
        mark(module,name,{'file':MODULES[module]['recovered_file'],'method':name},old_evidence(module,method)+(['verification.json'] if oracle else []),oracle=oracle)

# These mappings are backed by prior body/consumer analysis, not guessed by
# same-name declarations. Remaining inline-only helpers stay untouched.
mark('index_condenser','IndexCondenser::remove',{'file':MODULES['index_condenser']['recovered_file'],'method':'IndexCondenser::remove'},['../utils-tabloid/index_condenser-reva.json','../utils-tabloid/140a19f50.asm.txt'])
mark('float_grid','bilinear_sample_at_world_xz',{'file':MODULES['float_grid']['recovered_file'],'method':'FloatGridSampler::bilinear_sample_at_world_xz'},['../utils-tabloid/CONTRACT.md','../utils-tabloid/native-manifest.json'],gap='Two policies reviewed, 11 public instances merge by interface; preserve dynamic world/512byte-grid and sampling_fix vs legacy strategies; input-format specialization and full differential unresolved.')
mark('float_grid','FloatGrid::load_unchecked',{'file':MODULES['float_grid']['recovered_file'],'method':'FloatGrid::load_index_unchecked via explicit storage adapter'},['../utils-tabloid/CONTRACT.md'],static=False,gap='Trait delegates to real terrain/raster storage; candidate name/interface differs; original producer not wired.')
for r in match('halver','to_f16'):
    mark('halver',r['canonical_key'],{'file':MODULES['halver']['recovered_file'],'method':'Halver::to_f16'},['../utils-tabloid/CONTRACT.md','../utils-tabloid/native-manifest.json'],static=bool(r['original_entries']),gap='Original half conversion reused; exact inline instance type and producer unresolved where no native/public binding.')
for name,implementation in {
 'SplineSegment2D::position_at_local_t':'SplineSegment2D::position_at_local_t',
 'SplineSegment2D::tangent_at_local_t':'SplineSegment2D::tangent_at_local_t',
 'SplineSegment3D::position_at_local_t':'SplineSegment3D::position_at_local_t',
 'SplineSegment3D::tangent_at_local_t':'SplineSegment3D::tangent_at_local_t',
 'Spline2D::from_control_points_with_tangents':'Spline2D::from_control_points_with_tangents',
 'Spline2D::from_periodic_loop':'Spline2D::from_periodic_loop',
 'Spline3D::from_control_points_with_tangents':'Spline3D::from_control_points_with_tangents',
 'Spline3D::position_at_t':'Spline3D::position_at_t','Spline3D::tangent_at_t':'Spline3D::tangent_at_t','Spline3D::to_owned':'Spline3D::to_owned',
 'solve_clamped_spline':'second_derivatives + coefficients',
 'compute_periodic_spline':'periodic_second_derivatives + coefficients',
 'build_knots_open':'knots(periodic=false)',
 'build_knots_open3':'knots(periodic=false)',
 'build_knots_closed':'knots(periodic=true)',
 'eval_cubic':'SplineSegment2D/3D::position_at_local_t',
 'eval_cubic_deriv':'SplineSegment2D/3D::tangent_at_local_t',
 'Spline3D::find_segment_and_dt':'Spline3D::segment_at_t',
}.items():
    # Internal helper observations validated through original constructor/output
    # chain are distinguished from direct entry-point differential below.
    direct=name in ['Spline2D::from_control_points_with_tangents','Spline2D::from_periodic_loop','Spline3D::from_control_points_with_tangents','Spline3D::position_at_t','Spline3D::tangent_at_t']
    mark('natural_cubic',name,{'file':MODULES['natural_cubic']['recovered_file'],'method':implementation},['native-query.json','extras-query.json','spline-rdp-native-fixtures.json','verification.json'],oracle=direct,wired='find_segment_and_dt' in name,gap='Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown.')
mark('timer','FilteredWallTime::delta_secs',{'file':MODULES['timer']['recovered_file'],'method':'FilteredWallTime::delta_secs'},['../utils-tabloid/141799e70.asm.txt','small-native-fixtures.json','verification.json'],oracle=True)
mark('timer','TimeFilter::update',{'file':MODULES['timer']['recovered_file'],'method':'FilteredWallTime::delta_secs private filter body'},['../utils-tabloid/141799e70.asm.txt','small-native-fixtures.json'],oracle=False,gap='Inline TimeFilter update represented inside standalone filter; original 140-step caller oracle covers behavior indirectly; Bevy integration missing.')
mark('timer','impl$2::default',{'file':MODULES['timer']['recovered_file'],'method':'FilteredWallTime::default'},['extras-query.json','0x14179a010.c'],gap='Original Local values established by init_state; impl$2 exact Rust owner/ABI unresolved. Values not library Default reimplementation.')
mark('timer','init_state',{'file':MODULES['timer']['recovered_file'],'method':'FilteredWallTime::default game Local values only'},['extras-query.json','0x14179a010.c'],gap='Recover only warmup=1/dt1/60/empty deque-cap4; original SystemParam callable registration deliberately delegated to original Bevy and not wired.')
for name,method in [('impl$1::deserialize','SelectiveDeserialize::deserialize'),('impl$0::visit_map','nested SelectField::visit_map')]:
    mark('selective_deserialize',name,{'file':MODULES['selective_deserialize']['recovered_file'],'method':method},['native-query.json','../utils-tabloid/selective-persistent.json','selective-policy.log','CONTRACT.md'],static=name.endswith('deserialize'),gap='Outer wrapper original error/default policy compared; inner map policy inferred from exact History/snapshots constants and tested candidate only; full visitor/runtime and payload type unresolved.')
mark('wall_space','WallSpaceCoord::new_unchecked',{'file':MODULES['wall_space']['recovered_file'],'method':'WallSpaceCoord::new_unchecked'},['../utils-tabloid/CONTRACT.md','../utils-tabloid/native-manifest.json'])
for module in ['wall_space','comparative_space']:
    for r in list(records.values()):
        if r['module']!=module:continue
        n=r['canonical_key'];leaf=n.rsplit('::',1)[-1]
        if ('WallSpace as ' in n or 'CircularComparativeSpace as ' in n) and leaf in ['distance','project','project_clamp','project_unbounded','subtract']:
            mark(module,n,{'file':MODULES[module]['recovered_file'],'method':leaf},['../utils-tabloid/CONTRACT.md','../utils-tabloid/native-manifest.json'],gap='Concrete trait owner bound by original compiled address; static game policy reviewed, full interface/consumer differential remains incomplete.')

# Known implementation adapters without original interface proof are not counted
# as recovered by matching method spelling. They are listed separately below.
ADAPTERS=[]
for name,method,gap in [
 ('index_condenser','IndexCondenser::insert','Candidate invariant producer exists; original inline insert policy not compared.'),
 ('index_condenser','IndexCondenser::get','Candidate resolve exists under another API; original get key/handle/borrow semantics unknown.'),
 ('index_condenser','Condex::last_known_index','Candidate IndexHandle cached_index field exists; no original method analysis.'),
 ('swizzlers','impl$1::xvy','Candidate xvy(Vec2,y) exists; prior same-layout assumption, no per-original interface native/inline rule comparison.'),
 ('loop_range','impl$1::sub','Candidate LoopIdx::sub exists; trait receiver/type binding and independent method review unresolved.'),
 ('loop_range','impl$2::next','Candidate Iterator::next exists; original specialized iterator policy not individually verified.'),
 ('loop_range','impl$3::next_back','Candidate Iterator::next_back exists; exact iterator owner binding/edge behavior unresolved.'),
]:
    for r in match(name,method):
        r['candidate_implementation']={'file':MODULES[name]['recovered_file'],'method':'candidate adapter; original method binding unverified'}
        r['gaps']=[gap]
        ADAPTERS.append({'original_interface':r['canonical_key'],'gap':gap})

batch_updates=[]
for update_file in sorted(OUT.glob('interface-batch-*/interface-state-updates.json')):
    update_document=json.loads(update_file.read_text(encoding='utf8'))
    assert update_document['baseline']=='B'
    for update in update_document['updates']:
        key=update['canonical_key'];assert key in records,(str(update_file),key)
        previous=records[key]['analysis_status']
        records[key].update(update)
        batch_updates.append({'key':key,'before':previous,'after':update['analysis_status'],'evidence':update_file.relative_to(OUT).as_posix()})
ADAPTERS=[x for x in ADAPTERS if records[x['original_interface']]['analysis_status']=='untouched']
rows=sorted(records.values(),key=lambda r:(r['module'],r['canonical_key']))
stats={'observed_private_interfaces':len(rows),'business_analyzed_partial':sum(r['analysis_status']=='analyzed_partial' for r in rows),'untouched':sum(r['analysis_status']=='untouched' for r in rows),'candidate_implementation_present':sum(r['candidate_implementation'] is not None for r in rows),'static_business_compared':sum(r['static_comparison'] for r in rows),'direct_original_entry_differential':sum(r['original_instruction_differential'] for r in rows),'consumer_wired':sum(r['consumer_wired'] for r in rows),'excluded_boilerplate_or_unresolved_public_aliases':len(exclusions),'private_closures_separate':len(closures)}
stats['known_without_candidate_implementation']=sum(r['candidate_implementation'] is None for r in rows)
stats['candidate_present_but_original_business_untouched']=sum(r['analysis_status']=='untouched' and r['candidate_implementation'] is not None for r in rows)
stats['original_inline_differential']=sum(r.get('original_inline_differential',False) for r in rows)
report={'schema':'original-interface-coverage-v1','owner':'foundation_utils_base','baseline':'B','scope':'22 original utils top-level files only; lib.rs/curve/geometry/components excluded','denominator':'Observed B compiled/inline/declaration interfaces in existing LLVM-generated per-file inventory; not all original source APIs','canonicalization':'Remove Rust hashes/generic instance args; merge exact LLVM/public address ownership aliases only. Preserve unresolved impl$N owners and separate interface methods/strategies. FloatGrid specializations retain input/policy detail. Compiler glue/drop/library definitions and closures excluded from API count.','statistics':stats,'interfaces':rows,'untouched_interfaces':[r['canonical_key'] for r in rows if r['analysis_status']=='untouched'],'analyzed_without_candidate_implementation':[r['canonical_key'] for r in rows if r['analysis_status']!='untouched' and r['candidate_implementation'] is None],'candidate_adapters_without_original_analysis':ADAPTERS,'excluded':exclusions,'closures':closures,'inputs':inputs,'unknown_limits':['Existing per-file .pdb.jsonl.gz and full _pdb/B archives referenced by skeleton are missing locally; inventory comments retain record locations and IPI references but this audit does not invent missing parameter/inline-body data.','PDB void() is not Rust ABI. Some impl$N owners cannot be resolved; they remain distinct rather than merged by method name.','Untouched means no recorded interface-level business analysis, even if an unrelated/self-designed adapter shares a name. Located PDB/bytes alone is not analysis.','No exhaustive API denominator: fully optimized-away/unobserved methods can be missing. Observed denominator is a lower bound, not completion percentage.','Direct native comparison flags exclude indirect constructor-chain coverage and candidate-only tests. consumer_wired only true for actual recovered in-crate original call links, never full original game.'],'source_modified_by_audit':False,'new_decompilation_by_audit':False}
report['known_without_candidate_implementation']=[r['canonical_key'] for r in rows if r['candidate_implementation'] is None]
report['batch_interface_state_changes']=batch_updates
report['exact_declaration_alias_merges']=declaration_merges
report['module_summary']={module:{'observed':sum(r['module']==module for r in rows),'analyzed':sum(r['module']==module and r['analysis_status']!='untouched' for r in rows),'untouched':sum(r['module']==module and r['analysis_status']=='untouched' for r in rows),'with_candidate':sum(r['module']==module and r['candidate_implementation'] is not None for r in rows)} for module in MODULES}
report['inputs'] += [{'path':'evidence/render-architecture/public-symbols.json','sha256':hashlib.sha256((ROOT/'evidence/render-architecture/public-symbols.json').read_bytes()).hexdigest()},{'path':(OUT/'delivery.json').relative_to(ROOT).as_posix(),'sha256':hashlib.sha256((OUT/'delivery.json').read_bytes()).hexdigest()}]
report['untouched_policy_note']='Conservative interface-specific audit: related algorithm logic in another method does not prove this observed original interface; constructor/getter/adaptor names are not counted solely by resemblance. Unbound helpers remain pending original-interface rule correspondence.'
report['unknown_limits'].append('Private-owned Clone/Hash/fmt/Serde schema methods whose generated-versus-authored role is not independently established are retained for triage, not claimed as algorithms requiring hand implementation. Do not exclude them merely by trait name, or reimplement Serde/std to close them.')
(OUT/'interface-coverage.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
md='# utils 顶层 22 文件：原 B 接口覆盖审计\n\n'
md+=f"已观察私有接口 **{stats['observed_private_interfaces']}** 项；有逐接口业务分析 **{stats['business_analyzed_partial']}** 项；严格未触及 **{stats['untouched']}** 项。候选实现存在 {stats['candidate_implementation_present']} 项，静态业务比较 {stats['static_business_compared']} 项，直接原入口对照 {stats['direct_original_entry_differential']} 项。这些状态交叉，不能相加，也不是完整游戏百分比。\n\n"
md+=f"独立记录原内联片段差分 **{stats['original_inline_differential']}** 项；它不混入直接原入口数。\n\n"
md+=f"已知但无对应候选实现 **{stats['known_without_candidate_implementation']}** 项；另有 **{stats['candidate_present_but_original_business_untouched']}** 项仅存在适配器、缺原接口业务分析，仍列未触及。\n\n"
md+='分母来自已有 B LLVM compiled/inline/declaration 元数据，不是 Rust 当前函数声明数。仅定位、metadata 或字节转储不算业务分析；有候选适配器但未验证原接口的仍列未触及。私有闭包另列，库/派生/Bevy forwarding 排除；业务 Local init_state 保留。Rust hash和泛型实例合并，但未解析 impl$N owner 保留区分；无证据不猜类型。\n\n'
md+='| 原 B 接口 | 业务分析 | 候选实现 | 静态比较 | 原码对照 | 接线 | 原入口/PDB | 缺口 |\n|---|---|---|---|---|---|---|---|\n'
for r in rows:
    impl=r['candidate_implementation']['method'] if r['candidate_implementation'] else '无'
    markv=lambda v:'是' if v else '否'
    oracle='直接原入口' if r['original_instruction_differential'] else '原内联片段' if r.get('original_inline_differential') else '否'
    md+=f"| `{r['canonical_key']}` | {'部分已分析' if r['analysis_status']!='untouched' else '**未触及**'} | {impl} | {markv(r['static_comparison'])} | {oracle} | {markv(r['consumer_wired'])} | {', '.join(r['original_entries']) or 'inline/IPI；无独立入口'} | {'; '.join(r['gaps'])} |\n"
md+='\n## 严格未触及接口\n\n'
md+='\n'.join('- `'+k+'`' for k in report['untouched_interfaces'])+'\n'
md+='\n## 分母和证据限制\n\n'+'\n'.join('- '+x for x in report['unknown_limits'])+'\n'
md+='\n完整来源行号/入口、排除项和闭包见 [interface-coverage.json](interface-coverage.json)。本轮只审计已有证据，未改源码、未新增反编译。\n'
(OUT/'interface-coverage.md').write_text(md,encoding='utf8')
print(json.dumps(stats))
