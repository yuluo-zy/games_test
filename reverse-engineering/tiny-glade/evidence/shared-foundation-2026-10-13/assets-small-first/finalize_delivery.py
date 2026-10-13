import hashlib,json,pathlib,re
OUT=pathlib.Path(__file__).resolve().parent;ROOT=OUT.parents[2]
CRATES=ROOT/'reconstruction/shared-foundation/crates'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
for name in ['cargo-test.log','clippy.log']:
    p=OUT/name;raw=p.read_bytes();text=raw.decode('utf-16') if raw[:2] in [b'\xff\xfe',b'\xfe\xff'] else raw.decode('utf8')
    p.write_text(text,encoding='utf8',newline='\n')
test_log=(OUT/'cargo-test.log').read_text(encoding='utf8');clippy_log=(OUT/'clippy.log').read_text(encoding='utf8')
passed=sum(int(x) for x in re.findall(r'test result: ok\. (\d+) passed',test_log))
assert passed==19 and 'test result: FAILED' not in test_log
assert 'Finished `dev`' in clippy_log and '\nerror:' not in clippy_log
gamma=json.loads((OUT/'gamma-verification.json').read_text());assert gamma['ok'] and gamma['cases']==4364
bounded=json.loads((OUT/'bounded-report.json').read_text());assert bounded['ok'] and bounded['actual_mcp_connection']
functions={
 'asset-pipe/src/lib.rs':[],
 'asset-pipe/src/texture.rs':[
   ('compile_or_load_texture_assets','0x1417b7300','index loading/conditional build; modern root argument; outer flags/logs not recreated'),
   ('compile_texture_assets','0x1417b7790','original easy-parallel 3.2.0 each→run; one descriptor per task; join before collecting Results/persisting; normal/error/panic fixtures'),
   ('compile_texture_asset','0x1417b8f50','descriptor+mtime+magic/version cache and output; synthetic actual-file fixture'),
   ('process_regular_image / process_decoded_image','0x1417bbc10','gamma, resize, mips, format; instruction/control-flow comparison, contracts; no full original entry differential'),
   ('next_level','0x1417bc930','private mip closure; base resize/padding/type conversion; original address is closure, new API is own port'),
   ('with_rgba8','0x1420e2bd0,0x1420e3080','RGB-only output transfer; original generic specializations; wrapper differs in API'),
   ('srgb_oetf','0x1417bcd20','4364 original private instruction cases bit-identical'),
   ('compress / bc7_settings','0x1417bcd60','original channel selection/basic presets; dependency codecs reused; contract only, not compressed-pixel native differential'),
   ('process_dds_image / process_dds','0x1417ba565..0x1417bafb7','inlined DDS route in compile_texture_asset; format/layer-zero/dimensions clauses checked'),
   ('assemble_array','0x1417b99bc..0x1417ba6e4','inlined array route in compile_texture_asset; invariants and layer-major join checked')],
 'asset/src/lib.rs':[],
 'asset/src/inheritable.rs':[
   ('Inheritable::inherit_from / Option::inherit_from / HashMap::inherit_from','0x142689b90,0x142689ed0,0x14268a2e0,0x14268a700,0x142424d90','control-flow comparison; own semantic data types; not native ABI'),
   ('deserialize_inheritable_option','0x14268aa60,0x14268aee0,0x14268b260,0x14268b770','typed RON thin adaptation; actual prefab plus bare/Some/None/unit/error contracts')],
 'asset/src/prefab.rs':[
   ('PrefabName::new','0x141556080','Arc<str> wrapper, dependency allocation semantics reused'),
   ('PrefabName::as_name_ref / AsRef<str>::as_ref','0x141556130','Arc-backed string/nominal borrowed view; new exact-byte Borrow/AsRef/Deref adapters and lookup contracts in interface-batch-02')],
 'asset/src/prefab/config.rs':[
   ('PrefabConfig::inherit_from / PrefabPartConfig::inherit_from','0x142424d90','map merge and default resolution candidate; unresolved/default paths not exhaustive'),
   ('PrefabRtConfig::inherit_from / PrefabRenderConfig::inherit_from / AudioTriggerConfig::inherit_from','0x142879c20,0x142879de0,0x142689ed0,0x142689b90','field-level override candidates; semantic mappings, not native structure declarations'),
   ('deserialize_float3',None,'owned serde adapter for original three-element RON sequence syntax; source type name unknown')],
 'asset/src/prefab/scan.rs':[
   ('PrefabDependencies::add_file_to_prefab_dependency','0x142009df0','file→prefab vector relationship; duplicate registrations retained'),
   ('scan_asset_tree','0x14200a1b0','25 original shipped prefabs/28 dependency files parse+merge'),
   ('scan_asset_tree_walker','0x14200a5f0','default.ron inheritance and recursion; consumer native differential remains'),
   ('load_prefab_config','0x14200ba50','original RON delegation; modern compiler adaptation')],
 'asset/src/texture/blob.rs':[
   ('TextureAssetBlob transport / load / to_file_bytes','0x1417b55c0','28-byte archive root inferred from producer and validated against all 388 originals; loader/writer APIs are own safe adapters')],
 'asset/src/texture/config.rs':[
   ('InheritableTextureOpts::inherit_from / into_concrete','0x142456250','field resolution candidate; defaults/source file syntax validated; all panic/default cases not exhaustive')],
 'asset/src/texture/desc.rs':[
   ('TextureOpts / TextureAssetDesc / Gamma / MipChain / CompressionMethod',None,'original serialization fields/variants; Color default corrected from original scanner'),
   ('TextureAssetDesc::source_files',None,'owned helper; not counted as an identified original function')],
 'asset/src/texture/scan.rs':[
   ('scan_asset_tree','0x1420d8ba0','root defaults and mapping; original configs/fixture scan'),
   ('scan_asset_tree_walker','0x1420d8d00','configuration inheritance, extension selection, consecutive array indexing; fixtures')]
}
files=[]
for rel,items in functions.items():
    path=CRATES/rel
    files.append({'original_path':'crates/'+rel,'implementation_path':path.relative_to(ROOT).as_posix(),
        'sha256':sha(path),'version':'B','functions':[{'restored_function':f,'original_address':a,'validation':v} for f,a,v in items],
        'status':'module_glue_only' if not items else 'verified_scopes_partial',
        'original_text_identical':False,'full_original_file_accepted':False})
extra=CRATES/'asset/src/texture.rs'
batch=json.loads((OUT/'interface-batch-02/business-comparison.json').read_text(encoding='utf8'))
for item in batch['interfaces']:
    file=next(f for f in files if f['implementation_path']==item['implementation_path'])
    file['functions'].append({'restored_function':item['api'],'original_address':None,
        'original_inline_or_shared_body_evidence':'interface-batch-02/business-comparison.json and inline-sites.json',
        'validation':item['scope'],'native_compared':item.get('native_compared',False)})
report={
 'schema_version':1,'baseline':'B','original_binary_sha256':gamma['original_binary_sha256'],
 'patched_program_sha256':'cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323',
 'policy':'business rules and algorithm equivalence; independent source/API wrapping; original external dependencies reused',
 'worker':'foundation-assets-small-first','current_program':'tiny-glade.exe',
 'scope':{'asset_pipe_B_original_files':2,'asset_B_original_files':9,'union_inventory_not_B_denominator':True,
    'C_only_unrestored':['crates/asset-pipe/src/mip_flood.rs','crates/asset-pipe/src/store.rs','crates/asset-pipe/src/texture_processing.rs',
        'crates/asset/src/foliage.rs','crates/asset/src/mesh.rs','crates/asset/src/legacy_mod.rs','crates/asset/src/texture.rs']},
 'files':files,'synthetic_glue':[{'path':extra.relative_to(ROOT).as_posix(),'sha256':sha(extra),'not_C_original_file_recovery':True}],
 'differences_fixed':[
    {'old':'CompressionMethod default None','original':'Color','fix':'Default Color; scanner/compile fixture'},
    {'old':'BC7 slow presets','original':'basic presets','fix':'original dependency opaque_basic/alpha_basic; original literal constants checked'},
    {'old':'untagged option loses Code enum on Rust 1.97','original':'bare/Some/None/unit accepted preserving enums','fix':'original RON RawValue and typed parsing thin adapter'},
    {'old':'prefab vector fields expected RON tuples','original':'shipped input uses three-element sequences','fix':'serde Vec boundary validates length and stores fixed array'},
    {'old':'later RKYV archive(check_bytes) macro','original':'fork API archive_attr + bytecheck derive','fix':'compile/format compatibility only, not algorithm recovery'},
    {'old':'EOTF NaN takes powf branch','original':'unordered comparison takes linear division','fix':'strict-greater nonlinear branch'},
    {'old':'sequential caller loop with manually delayed error','original':'easy_parallel each→run joins every per-asset task, preserves added result order, propagates panic after join','fix':'reuse exact easy-parallel3.2.0 each→run; first error while collecting results; no meta/list on error or panic'}],
 'verification':{'cargo_tests':{'passed':passed,'command':'cargo test -p asset -p asset-pipe --locked --offline','log':'cargo-test.log'},
    'clippy':{'passed':True,'command':'cargo clippy -p asset -p asset-pipe --all-targets --locked --offline -- -D warnings','log':'clippy.log'},
    'fmt':{'passed':True,'command':'cargo fmt -p asset -p asset-pipe'},
    'shipped_archives':{'count':388,'bytes':311679016,'byte_identical_roundtrip':True,'not_image_recompilation':True},
    'shipped_prefabs':{'prefabs':25,'dependency_files':28,'parse_and_inherit':True,'not_native_consumer_equivalence':True},
    'original_scalar':{'report':'gamma-verification.json','cases':4364,'mismatches':0,'scope':'private OETF 64-byte entry; reused public UCRT powf'},
    'tool_query':{'report':'bounded-report.json','ok':True,'private_functions':9,'not_business_acceptance':True},
    'shallow_native':{'report':'interface-batch-02/shallow-native-comparison.json','cases':160000,'mismatches':0,'scope':'Four scalar/array texture shallow specializations; not arbitrary generic ABI'}},
 'gaps':['No original PNG/JPG/DDS source texture corpus: shipped assets-src/textures has 16 RON files only; full input→pixel/compressed archive original-entry differential pending',
    'Prefab original field/native type names and all unresolved-default/error/side-effect paths not exhaustively proven',
    'Original print/log controls and outer CLI flags not reproduced; easy_parallel per-asset each→run has now been restored from original dependency',
    'Country-core live asset library/hot reload integration not yet recovered',
    'A/C independent identities and C new files need independent analysis; no B address transfer'],
 'interface_inventory':'interface-coverage.json','dependency_reuse_audit':'dependency-reuse-audit.json',
 'parallel_policy':'parallel-policy.json','interface_batches':['interface-batch-02/business-comparison.json'],
 'full_module_accepted':False,'full_original_game_executed':False,'report':'DELIVERY.md'}
(OUT/'delivery.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print(json.dumps({'files':len(files),'tests':passed,'archives':388,'scalar_cases':4364,'full_module_accepted':False}))
