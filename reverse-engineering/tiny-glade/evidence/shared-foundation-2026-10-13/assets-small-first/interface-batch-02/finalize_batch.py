"""Record rule -> implementation -> original comparison -> fix -> recheck."""
import hashlib,json,pathlib,re,struct
from original_code import OUT,SHA,read
ROOT=OUT.parents[3];CRATES=ROOT/'reconstruction/shared-foundation/crates'
for name in ['cargo-test.log','cargo-clippy.log']:
    p=OUT/name;raw=p.read_bytes()
    if raw[:2]in[b'\xff\xfe',b'\xfe\xff']:p.write_text(raw.decode('utf-16'),encoding='utf8')
log=(OUT/'cargo-test.log').read_text(encoding='utf8');clippy=(OUT/'cargo-clippy.log').read_text(encoding='utf8')
passed=sum(map(int,re.findall(r'test result: ok\. (\d+) passed',log)))
assert passed==19,passed
assert not re.search(r'test result: FAILED|error:',clippy)
assert 'Finished' in clippy
coverage=json.loads((OUT/'coverage-additions.json').read_text())
for row in coverage['interfaces']:
    p=CRATES/row['path'];row['implementation_sha256']=hashlib.sha256(p.read_bytes()).hexdigest()
    row['implementation_path']=p.relative_to(ROOT).as_posix()
    row['evidence_kind']='original instructions/static caller comparison plus Rust contract tests'
    row['original_consumer_integrated']=False
native=json.loads((OUT/'shallow-native-comparison.json').read_text());assert native['ok'] and native['cases']==160000
fmt_ptr,fmt_len=struct.unpack('<QQ',read(0x142ad7880,16))
missing_ptr,n,tail_ptr,n2=struct.unpack('<QQQQ',read(0x142ad78c8,32))
assert read(fmt_ptr,fmt_len).decode()=='as_concrete() called on an Inheritable value without a prior call to inherit()'
assert read(missing_ptr,n).decode()=='Unable to resolve: a default value '
report={'schema_version':1,'baseline':'B','original_RNE_sha256':SHA,
 'scope':'New private API behavior only in B asset9/asset-pipe2 inventory; 11 additional implementations, five formerly untouched APIs',
 'interfaces':coverage['interfaces'],'native_comparison':native,
 'rules':{
   'inherit_shallow':'Copy parent only for Inherit, preserving parent concrete/missing-default/inherit state; concrete child and explicit None are atomic. T only needs original std Clone; deep inherit_from recursively merges concrete objects.',
   'as_concrete':'Borrow concrete value; Inherit and MissingDefault panic with separate original formatting, without consuming or cloning the value.',
   'deserialize_inheritable':'Call original typed serde/RON deserialization; explicit valid T becomes Value(T), typed errors propagate, missing fields use Inherit. Separate permissive option callback unchanged.',
   'PrefabName':'Original shared Arc getter and borrowed identity getter; byte length/content remain unchanged; Borrow lookup performs exact length and runtime memcmp. New/From use std Arc, no name normalization.',
   'concrete_texture_default':'Original root inline writes zero tags: resize None, Gamma Linear, mips None, compression Color; distinct from serde missing-input Default.'},
 'original_static_evidence':{
   'as_concrete':{'inlinee':'0x7bf7','consumer':'country_core::startup::load_prefabs::load_prefabs_impl','tag_gate':'0x1412a23c2..0x1412a23e2','panic_state_branch':'0x1412a5528..0x1412a5608','panic_fmt':'0x1428d9390','inherit_string':read(fmt_ptr,fmt_len).decode(),'missing_default_format':[read(missing_ptr,n).decode(),read(tail_ptr,n2).decode()]},
   'deserialize':{'inlinee':'0x20bb7','bool_call':'0x142687c88 -> ron::parse::Parser::bool 0x140bc81c0','typed_enum_call':'0x142682b52 -> original RON deserialize_enum 0x142681f40','result_branches':['0x142687ca6 -> 0x1426884b3 error path','0x142682b5e -> 0x142683061 error path'],'success':'typed bool/enum result stored unchanged as concrete field; missing markers are default path, not parser sentinels'},
   'borrow':{'inlinee':'0x7872','call_sites':['0x141552e40','0x14155309b'],'callee':'0x141556130','comparison':'returned length compared, then external memcmp 0x142923bf0','borrowed_getter':'0x14067dd80 returns input data pointer unchanged'},
   'concrete_default':{'inlinee':'0x1a65f','caller':'asset::texture::scan::scan_asset_tree','range':'0x1420d8c24..0x1420d8c38','stores':'resize tag u32=0 at rbp-0x48; gamma+compression u16=0 at rbp-0x34; mip tag u32=0 at rbp-0x3c'}},
 'differences_and_fixes':[
   {'difference':'Recovered texture opts originally invoked deep generic inheritance; leaf no-op stubs made outputs coincide but did not express original atomic policy.','fix':'Added Clone-only inherit_shallow and wired all four texture fields to it.','recheck':'160000 original instruction cases and composite deep/shallow contrast test.'},
   {'difference':'Borrowed concrete API and named ordinary Deserialize callback absent.','fix':'Added as_concrete with exact original two panic formats; added typed callback and made serde impl delegate.','recheck':'borrow pointer identity; original failure texts; direct RON bool errors unchanged; ordinary Option vs permissive option adapter distinguished.'},
   {'difference':'PrefabName exposed str only; nominal borrowed view and original aliases/trait APIs absent.','fix':'Added transparent UTF-8 PrefabNameRef adapter and original borrowed/name methods/traits using Arc, Borrow, Deref.','recheck':'empty/case/path/nonASCII/embedded NUL names preserve bytes; same pointers/hash; standard HashMap Borrow lookup succeeds without allocating a key.'},
   {'difference':'Texture resolved defaults existed only through generic from_concrete(Default), independent original method absent.','fix':'Explicit concrete_default implementing original four field tags; scanner calls it.','recheck':'resolved defaults differ from Inherit markers; explicit None remains concrete after default merge.'},
   {'difference':'Strict Clippy identified intentional identity AsRef test as useless method syntax.','fix':'Test uses explicit UFCS to exercise the recovered trait API without suppressing warnings.','recheck':'strict Clippy all targets passes.'}],
 'verification':{'cargo_test_exit':0,'tests_passed':passed,'strict_all_targets_clippy_exit':0,'fmt_packages':['asset','asset-pipe'],'original_texture_archives':388,'original_prefab_input_files':28,
   'new_test_names':['shallow_inheritance_preserves_concrete_children_and_unresolved_markers','concrete_borrow_preserves_identity_and_original_unresolved_failures','explicit_deserializer_delegates_typed_values_and_preserves_errors','concrete_texture_defaults_resolve_missing_fields_without_replacing_explicit_none','borrowed_name_views_share_bytes_and_map_hash_without_normalizing']},
 'dependency_reuse':'No new dependency; serde/RON typed parsing, Arc/Clone/Borrow/HashMap/memcmp library behavior directly reused, private wrappers only. Original Cargo identities unchanged.',
 'gaps':coverage['gaps']+['Generic indexed impl$2/impl$6 concrete_default ownership/bounds still pending; one inline None specialization does not prove full generic rule','Nineteen observed APIs still untouched; previous C/whole native pipeline/logging/CLI gaps retained']}
(OUT/'business-comparison.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
md=['# B 资产接口第二批恢复','',
 '本批补 11 个对应实现，其中 5 个此前未触及：浅继承、借用具体值、普通 Deserialize 回调、PrefabName Borrow、纹理 concrete_default。原库存仍为 61：已分析/有实现 42、尚未触及 19、原指令比较 2、原游戏消费者接线 0。','',
 '| 规则 | 恢复实现与原作对照 | 复验 |','|---|---|---|',
 '| 浅继承 | 仅 Inherit 复制父状态；具体值、None、MissingDefault 不递归覆盖。已接纹理配置。 | 原 RNE 4 种纹理实例，全部有效标签联合组合 160,000 组，0 差异；复合对象与深继承区别测试。 |',
 '| as_concrete | 直接借用具体值，未继承与缺少默认值分别使用原 panic 文本。 | LLVM inlinee/原 tag gate/panic_fmt 对照；借用身份与失败文本测试。未执行任意泛型原 ABI。 |',
 '| deserialize_inheritable | 普通 typed serde/RON 结果映射 Value；错误保留。与 permissive Option 字段回调分开。 | 原 bool/enum 调用和 Result 分支；缺字段/false/true/坏输入/普通 Option 测试。 |',
 '| PrefabName 借用接口 | 标准 Arc + 借用 UTF-8 名称视图，AsRef/Borrow/Deref/From/as_str；原调用方比长度并复用 memcmp。 | 原两种 getter + 两个 Borrow 内联调用；空名/大小写/路径/Unicode/NUL/pointer/hash/HashMap 借用查询测试。 |',
 '| 纹理 concrete_default | 原零标签对应 None/Linear/MipNone/Color，scanner 已接线。 | 原 0x1420d8c24..38 四字段初始化；缺失字段和显式 None 保留测试。 |','',
 '全部 19 项 asset/asset-pipe 测试与 strict Clippy all targets 通过；其中发行物回归继续覆盖 388 个纹理档案、28 个原 prefab 输入文件。原图片输入缺失，完整像素/压缩原入口差分没有因此完成。没有新增依赖或库算法重写。','',
 'indexed generic concrete_default 的类型/trait 边界仍未知，不用一个内联 None 初始化推断全部泛型实现。其余 19 个未触及接口和 C 新路径保持独立缺口；原 country-core 运行时接线仍未完成。','',
 '规则、原地址、差异修复、源码 hash、机器指令比较范围见 [business-comparison.json](business-comparison.json)；统一接口台账见 [interface-coverage.json](../interface-coverage.json)。']
(OUT/'DELIVERY.md').write_text('\n'.join(md)+'\n',encoding='utf8')
print(json.dumps({'tests':passed,'additional_implementations':len(coverage['interfaces']),'native_cases':native['cases']}))
