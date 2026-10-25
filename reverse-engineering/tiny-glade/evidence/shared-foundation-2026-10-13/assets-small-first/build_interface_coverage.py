"""Conservative original-private API inventory including skeleton inlinee declarations."""
import collections,hashlib,json,pathlib,re
OUT=pathlib.Path(__file__).resolve().parent;ROOT=OUT.parents[2]
SKEL=ROOT/'skeleton/B/crates'
native=json.loads((OUT.parent/'assets/functions.json').read_text(encoding='utf8'))
public=json.loads((ROOT/'evidence/render-architecture/public-symbols.json').read_text(encoding='utf8'))
def canonical(name):return re.sub(r'::h[0-9a-f]{16}.*$','',name.split('<',1)[0]).strip()
by_native=collections.defaultdict(list)
for r in native:by_native[canonical(r['name'])].append(r)
inventory={};excluded=[];schemas=collections.defaultdict(list);inputs=[]
for crate in ['asset','asset-pipe']:
    for p in sorted((SKEL/crate/'src').rglob('*.rs')):
        text=p.read_text(encoding='utf8');inputs.append({'path':p.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
        original=re.search(r'^// Original source: (.+)$',text,re.M)[1]
        for m in re.finditer(r'^// 函数符号: (.+)$',text,re.M):
            end=text.find('// 函数符号:',m.end());block=text[m.end():end if end!=-1 else len(text)]
            name=m[1];key=canonical(name)
            forms=re.search(r'^// 记录形式: (.+)$',block,re.M)
            loc=re.search(r'^// 首条位置: (.+)$',block,re.M)
            line=re.search(r'^// 源码行: (.+)$',block,re.M)
            item={'symbol':name,'metadata_file':p.relative_to(ROOT).as_posix(),'original_source':original,
                'record_forms':forms[1] if forms else None,'source_line':line[1] if line else None,
                'first_location':json.loads(loc[1]) if loc else None}
            # Source headers can be compilation attribution (root lib contains RON/DDS
            # bodies). Capture actual PDB definition-source references, and separately
            # identify the defining namespace using the original Cargo package boundary.
            item['definition_source_references']=sorted(set(re.findall(r'^//\s+definition_source=(.+)$',block,re.M)))
            item['definition_owner_namespace']=name.split('::',1)[0]
            item['compilation_attribution_is_not_definition_ownership']=True
            if not name.startswith(('asset::','asset_pipe::')):
                excluded.append({**item,'reason':'Cargo/PDB definition namespace belongs to confirmed external/std library; source references retained; not game function merely because compiled into asset root'});continue
            if '::_::' in name:
                schemas[original].append(item)
                excluded.append({**item,'reason':'derive/macro visitor/serializer implementation glue; associated private schema rules retained separately'});continue
            if key.rsplit('::',1)[-1] in ['clone','eq','hash','fmt','expecting']:
                excluded.append({**item,'reason':'derived representation/trait glue; delegates library container/hash/format primitives; no independently recovered private algorithm claimed'});continue
            row=inventory.setdefault(key,{'id':key,'original_owner':key.rsplit('::',1)[0],'original_method':key.rsplit('::',1)[-1],
                'original_sources':[],'PDB_records':[],'kind':'closure' if '::closure$' in key else 'private_interface'})
            if original not in row['original_sources']:row['original_sources'].append(original)
            row['PDB_records'].append(item)

# These public aliases add APIs omitted by module procedure inventories. Do not merge
# distinct methods/trait targets merely because identical-code folding shares an address.
aliases={
 'asset::prefab::PrefabName::as_str':('asset::prefab::PrefabName','as_str','0x141556130'),
 'asset::prefab::PrefabNameRef::as_str':('asset::prefab::PrefabNameRef','as_str','0x14067dd80'),
 'asset::prefab::PrefabName::Deref::deref':('asset::prefab::PrefabName [Deref]','deref','0x141556130'),
 'asset::prefab::PrefabName::From<&str>::from':('asset::prefab::PrefabName [From<&str>]','from','0x141556080'),
 'asset::prefab::PrefabName::AsRef<str>::as_ref':('asset::prefab::PrefabName [AsRef<str>]','as_ref','0x141556130')}
for key,(owner,method,address) in aliases.items():
    def matching(r):
        n=r['demangled']
        if key.endswith('::Deref::deref'):return n.startswith('_$LT$asset..prefab..PrefabName$u20$as$u20$core..ops..deref..Deref$GT$::deref')
        if 'From<&str>'in key:return n.startswith('_$LT$asset..prefab..PrefabName$u20$as$u20$core..convert..From$LT$$RF$str$GT$$GT$::from')
        if 'AsRef<str>'in key:return n.startswith('_$LT$asset..prefab..PrefabName$u20$as$u20$core..convert..AsRef$LT$str$GT$$GT$::as_ref')
        return n.startswith(key+'::h')
    matches=[r for r in public if matching(r)];assert matches,key
    inventory[key]={'id':key,'original_owner':owner,'original_method':method,'kind':'private_interface',
        'original_sources':['crates/asset/src/prefab.rs'],'PDB_records':[],
        'public_aliases':[{'symbol':r['demangled'],'address':r['va']} for r in matches]}

# Original implementation indices are preserved, and resolved only where public PDB
# trait names confirm their owner. Unknown indexed concrete_default types stay unknown.
owners={
 'asset::inheritable::impl$1':'Inheritable<T> [original InheritFrom]',
 'asset::inheritable::impl$5':'HashMap<K,V> [original private InheritFrom implementation]',
 'asset::prefab::impl$4':'PrefabNameRef [AsRef<PrefabNameRef>]',
 'asset::prefab::impl$5':'PrefabName [AsRef<PrefabNameRef>; public AsRef<str> separate]',
 'asset::prefab::config::impl$5':'PrefabRenderConfig [InheritFrom]',
 'asset::prefab::config::impl$6':'PrefabRtConfig [InheritFrom]'}

# Emitted functions below were read against original disassembly/producer calls in the
# delivered scope. Inline-only names do not automatically inherit analyzed status.
known={canonical(r['name']) for r in native if r['name'].startswith(('asset::','asset_pipe::'))
    and '::_::' not in r['name'] and not r['name'].endswith('::fmt') and '::closure$'not in r['name']}
implemented=set(known)-{'asset::prefab::impl$4::as_ref','asset::prefab::impl$5::as_ref'}
ports={
 'asset::inheritable::Inheritable::into_concrete':('asset/src/inheritable.rs','Inheritable::into_concrete','original texture resolver panic/value cases; not all generic default branches'),
 'asset::texture::config::InheritableTextureOpts::inherit_from':('asset/src/texture/config.rs','InheritableTextureOpts::inherit_from','original scanner parent/default field resolution plus actual input fixtures'),
 'asset::texture::scan::load_texture_group_config':('asset/src/texture/scan.rs','scan_asset_tree_walker RON loading branch','delegates original RON; private selection/error context policy partially compared'),
 'asset_pipe::texture::resize_image':('asset-pipe/src/texture.rs','process_decoded_image/next_level resize_exact branches','original resize call sites 0x1417bc03b..0x1417bc096 and closure3'),
 'asset_pipe::texture::pad_image':('asset-pipe/src/texture.rs','pad_edges/next_level','original closure3 edge copy loops 0x1417bc930'),
 'asset_pipe::texture::apply_transfer_function':('asset-pipe/src/texture.rs','with_rgba8 and process_decoded_image RGB loops','original RGB-only OETF/EOTF loops; alpha untouched'),
 'asset_pipe::texture::srgb_eotf':('asset-pipe/src/texture.rs','srgb_eotf','original inlined EOTF branch in process_regular_image; no independent scalar oracle'),
 'asset_pipe::texture::process_dds_image':('asset-pipe/src/texture.rs','process_dds_image/process_dds','original inlined DDS route 0x1417ba565..0x1417bafb7'),
 'asset_pipe::texture::process_image_array':('asset-pipe/src/texture.rs','process_texture_asset/assemble_array','original array route in compile_texture_asset; invariants and layer-major data'),
}
known.update(ports);implemented.update(ports)
typed_impl={
 'asset::prefab::PrefabName::new':('asset/src/prefab.rs','PrefabName::new'),
 'asset::prefab::PrefabName::as_name_ref':('asset/src/prefab.rs','PrefabName::as_name_ref'),
 'asset::prefab::PrefabName::AsRef<str>::as_ref':('asset/src/prefab.rs','impl AsRef<str> for PrefabName'),
 'asset::inheritable::impl$1::inherit_from':('asset/src/inheritable.rs','impl Inherit for Inheritable<T>'),
 'asset::inheritable::impl$5::inherit_from':('asset/src/inheritable.rs','impl Inherit for HashMap<K,V>'),
 'asset::inheritable::deserialize_inheritable_option':('asset/src/inheritable.rs','deserialize_inheritable_option'),
 'asset::prefab::config::impl$5::inherit_from':('asset/src/prefab/config.rs','PrefabRenderConfig::inherit_from'),
 'asset::prefab::config::impl$6::inherit_from':('asset/src/prefab/config.rs','PrefabRtConfig::inherit_from'),
 'asset::texture::config::InheritableTextureOpts::into_concrete':('asset/src/texture/config.rs','InheritableTextureOpts::into_concrete')}
known.update(aliases) # Shared bodies already read; typed aliases remain not implemented unless declared.
implemented.add('asset::prefab::PrefabName::AsRef<str>::as_ref')
for key in inventory:
    if key in typed_impl:continue
    if key.startswith('asset::prefab::scan::') and key in implemented:typed_impl[key]=('asset/src/prefab/scan.rs',key.rsplit('::',1)[-1])
    if key.startswith('asset::texture::scan::') and key in implemented:typed_impl[key]=('asset/src/texture/scan.rs',key.rsplit('::',1)[-1])
    if key.startswith('asset_pipe::texture::') and key in implemented:typed_impl[key]=('asset-pipe/src/texture.rs',key.rsplit('::',1)[-1])
for key,(path,api,note) in ports.items():typed_impl[key]=(path,api)

# Later batches may advance a row only through an explicit business comparison,
# implementation and bounded verification record. Names alone never advance it.
batch_updates={}
for path in sorted(OUT.glob('interface-batch-*/coverage-additions.json')):
    batch=json.loads(path.read_text(encoding='utf8'));assert batch['baseline']=='B'
    for update in batch['interfaces']:
        assert update['id'] in inventory,update['id']
        known.add(update['id']);implemented.add(update['id'])
        typed_impl[update['id']]=(update['path'],update['api'])
        batch_updates[update['id']]=(update,batch)

closure_known={'asset_pipe::texture::process_regular_image::closure$3':('next_level','original standalone closure disassembly'),
    'asset_pipe::texture::compile_texture_assets::closure$4':('Parallel::each job closure','original each→run caller with per-asset callback; direct original dependency'),
    'asset_pipe::texture::process_regular_image::closure$2':('encode_level base','original with_rgba8 specialization and base output'),
    'asset_pipe::texture::process_regular_image::closure$4':('encode_level appended mips','original with_rgba8 specialization and mip concatenation')}
rows=[]
for key,row in sorted(inventory.items()):
    row['original_owner']=owners.get(row['original_owner'],row['original_owner'])
    row['addresses']=sorted({r['va'] for r in by_native.get(key,[])})
    row['addresses']+=sorted({r['address'] for r in row.get('public_aliases',[]) if r['address'] not in row['addresses']})
    row['record_scope']='native procedure and/or inlinee declarations; not reconstructed source text'
    row['actual_business_analysis']=key in known or key in closure_known
    row['implemented']=key in implemented or key in closure_known
    row['static_compared']=row['actual_business_analysis']
    row['native_compared']=key=='asset_pipe::texture::srgb_oetf'
    row['artifact_compared']=False
    row['consumer_integrated']=False # Original country-core runtime consumer is not restored.
    row['restoration_internal_consumers_integrated']=row['implemented'] and not key.startswith('asset::prefab::')
    row['native_comparison_evidence']='gamma-verification.json; 4364 cases' if row['native_compared'] else None
    row['full_interface_accepted']=False
    if row['native_compared']:row['status']='native_compared'
    elif row['implemented']:row['status']='implemented_static'
    elif row['actual_business_analysis']:row['status']='analyzed_only'
    else:row['status']='untouched'
    mapped=typed_impl.get(key)
    if mapped:
        p=ROOT/'reconstruction/shared-foundation/crates'/mapped[0]
        row['recovered_api']={'path':p.relative_to(ROOT).as_posix(),'API':mapped[1],'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'source_signature_identical':False}
    elif key in closure_known:
        row['recovered_api']={'path':'reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs','API':closure_known[key][0],'source_signature_identical':False}
    else:row['recovered_api']=None
    row['evidence']=sorted({r['metadata_file'] for r in row['PDB_records']})
    if row['actual_business_analysis']:
        row['evidence']+=['delivery.json','cargo-test.log']
        row['evidence']+=[f'../assets/asm-{r["va"]}.txt' for r in by_native.get(key,[]) if (OUT.parent/'assets'/f'asm-{r["va"]}.txt').exists()]
    row['analysis_scope']=ports.get(key,(None,None,None))[2] if key in ports else ('original emitted body/call-site comparison; typed consumer/native ABI remains partial' if row['actual_business_analysis'] else 'Metadata only; no original business/control-flow analysis accepted')
    row['gaps']=(['full native input/output and original game consumer integration pending'] if row['implemented'] else ['original typed method/behavior must be traced before claiming implementation'])
    if 'concrete_default'in key:row['gaps'].append('indexed owner/default values and producer paths not recovered; do not equate candidate Default with original concrete_default')
    if 'as_ref'in key and not row['implemented']:row['gaps'].append('original PrefabNameRef nominal wrapper/API not declared; current port exposes &str')
    if key in batch_updates:
        update,batch=batch_updates[key]
        row['analysis_scope']=update['scope']
        row['evidence']+=batch['common_evidence']
        row['native_compared']=update.get('native_compared',False)
        row['native_comparison_evidence']=update.get('native_comparison_evidence')
        row['restoration_internal_consumers_integrated']=update.get('internal_integrated',False)
        row['status']='native_compared'if row['native_compared']else'implemented_static'
        row['gaps']=batch['gaps']
    rows.append(row)

interfaces=[r for r in rows if r['kind']=='private_interface'];closures=[r for r in rows if r['kind']=='closure']
schema_rows=[]
for source,records in sorted(schemas.items()):
    schema_rows.append({'original_source':source,'generated_records':len(records),
        'classification':'private_schema_rules; generated derive visitors/serialize/resolve bodies excluded from authored API denominator',
        'actual_business_analysis':True,'implemented':True,'native_compared':False,
        'artifact_compared':source=='crates/asset/src/texture/blob.rs',
        'status':'implemented_static','verification':'388 archive byte-identical roundtrips' if source=='crates/asset/src/texture/blob.rs' else 'shipped RON/schema field inventories and typed input/compile fixtures; not every bad-input branch',
        'PDB_records':records,'gaps':['full native field/type and error contracts not exhaustively verified']})
counts={
 'observed_original_private_interfaces':len(interfaces),'analyzed':sum(r['actual_business_analysis'] for r in interfaces),
 'untouched':sum(r['status']=='untouched' for r in interfaces),'implemented':sum(r['implemented'] for r in interfaces),
 'native_compared':sum(r['native_compared'] for r in interfaces),'consumer_integrated':0,
 'status_counts':dict(collections.Counter(r['status'] for r in interfaces)),
 'closures_separate':len(closures),'closures_analyzed':sum(r['actual_business_analysis']for r in closures),
 'schema_rule_groups_separate':len(schema_rows),'excluded_records':len(excluded)}
report={'schema_version':1,'baseline':'B','scope':'asset B9 original paths + asset-pipe B2 original paths',
 'denominator':'observed original-private named APIs including inlinee declarations; generic duplicates merged; closures/schema rules/compiler/library glue separately recorded',
 'counts':counts,'interfaces':interfaces,'closures':closures,'schema_rules':schema_rows,'excluded':excluded,
 'inventory_inputs':inputs,'source_text_identical_required':False,
 'boolean_dimensions_independent':True,'full_module_accepted':False,
 'untouched_interfaces':[r['id'] for r in interfaces if r['status']=='untouched'],
 'C_new_interfaces':{'count':None,'status':'not inventoried/unknown; no B-address transfer','files':['asset-pipe/src/mip_flood.rs','asset-pipe/src/store.rs','asset-pipe/src/texture_processing.rs','asset/src/foliage.rs','asset/src/mesh.rs','asset/src/legacy_mod.rs','asset/src/texture.rs']},
 'limits':['Inventory is observed PDB lower bound, not all author source declarations; missing/non-recorded inline APIs unknown',
    'PDB void placeholders never ABI; indexed impl owners only resolved with public trait/source evidence',
    'implemented_static means listed behavior port exists, not full original method equality; native comparison OETF and four shallow texture specializations only',
    'original country-core consumer integration absent; internal asset-pipe→asset calls separate',
    'skeleton records are metadata, never implementation recovery']}
(OUT/'interface-coverage.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
md=['# B asset / asset-pipe 接口覆盖对照','',
    '本表来自 B 原 11 个文件的 PDB 骨架函数/inlinee 声明、原 LLVM procedure 库存及 public alias。骨架不是源码实现；PDB `void()` 不是 ABI。按原私有定义归属去 hash、合并同一泛型重复，不把 1013 个含库 procedure 当接口分母，也不把同地址的不同原 trait 接口合并。','',
    f'已观察私有接口 **{len(interfaces)}**：已业务分析 **{counts["analyzed"]}**、尚未触及 **{counts["untouched"]}**、已有对应行为实现 **{counts["implemented"]}**、原指令差分 **{counts["native_compared"]}**、原 country-core 消费者接线 **0**。另列 {len(closures)} 个 closure（{counts["closures_analyzed"]} 已分析）及 {len(schema_rows)} 组 typed schema 规则。库存仍是已观察下界，不计算全包完成率。','',
    '`implemented_static` 是已列行为的 Rust 实现/静态对照，不代表完整原方法等价；`native_compared` 仅 OETF 与浅继承的四种纹理字段实例。布尔维度独立：内部 asset-pipe→asset 已编译调用与原游戏消费者接线分开，原类型/错误边界/未记录接口仍可能缺失。','',
    '| 原接口（同泛型已合并） | 状态 | 原地址/形式 | 恢复 API |','|---|---|---|---|']
for r in interfaces:
    address=', '.join(r['addresses']) or 'inlinee 声明；地址未提取'
    api=r['recovered_api']['API'] if r['recovered_api'] else '未恢复；已有共享 body 分析也不补造 typed API'
    md.append(f'| `{r["id"]}` | {r["status"]} | {address} | `{api}` |')
md+=['','## 尚未触及的接口','']
md.extend('- `'+name+'`' for name in report['untouched_interfaces'])
md+=['','尚未触及的 indexed concrete_default/default/部分 inherit_from、PrefabScanResult 依赖联接和 BcMode 字节数接口保持单独缺口。仅从其他 caller 见到相似规则或写了候选 Default，不算这些原接口已分析。新增继承/借用名称/纹理 concrete_default 的对照和测试见 interface-batch-02。','',
    '## Closure 和 schema 分开记录','',
    '| Closure | 状态 | 对应 port |','|---|---|---|']
for r in closures:md.append(f'| `{r["id"]}` | {r["status"]} | '+(r['recovered_api']['API'] if r['recovered_api'] else '未分析')+' |')
md+=['','Serde/RKYV derive 的 field visitor、serialize/resolve 胶水不计作者私有接口，但其承载的字段、enum、Option、默认值、28B texture root 等业务交换规则没有被删除：JSON 的 schema_rules 单独记录原 source、PDB 条目和验证范围。Blob schema 有 388 个原文件 byte-identical roundtrip；其余 schema 为发行 RON 和 typed fixture 对照，完整错误/原 native 类型没有因此通过。','',
    '## 版本与未验收范围','',
    '- C 的 mip_flood/store/texture_processing/foliage/mesh/legacy_mod/texture 是独立新路径；C 接口尚未盘点，数量 unknown，不拿 B 地址推断。',
    '- 原 PNG/JPG/DDS 输入缺失，完整原 pipeline/compression 入口差分未完成。',
    '- PrefabNameRef 已恢复为无分配 UTF-8 借用视图；原 ABI/layout 完全一致、全部 unresolved-default/坏输入副作用、原日志/CLI、country-core 热重载消费者仍未验收。',
    '- 原 easy-parallel 3.2.0 的每资产 each→run 已接线；14 个测试覆盖正常编译、全部任务完成后错误返回、panic先join后传播、失败不持久化 meta/list。',
    '', '逐记录/source/evidence/hash 见 [interface-coverage.json](interface-coverage.json)。']
(OUT/'interface-coverage.md').write_text('\n'.join(md)+'\n',encoding='utf8')
print(json.dumps(counts));print('untouched:\n'+'\n'.join(report['untouched_interfaces']))
