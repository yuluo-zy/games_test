"""Per-original-file business delivery; lexical declarations are not acceptance."""
from pathlib import Path
import hashlib,json,re
OUT=Path(__file__).parent;ROOT=OUT.parents[2];SRC=ROOT/'reconstruction/shared-foundation/crates/utils/src'
old=OUT.parent/'utils-tabloid'
manifest=json.loads((old/'native-manifest.json').read_text(encoding='utf8'))
verification=json.loads((OUT/'verification.json').read_text(encoding='utf8'))
# kind, verification, gaps. Every module remains partial; no source-text claim.
info={
'angle_range':('private_algorithm','50 B native clamp fixtures; compiled tests pass','Serde/wrap constructor edge policies outside fixture scope; A/C untested'),
'camera':('private_strategy_with_original_dependency','146 B xz rotations +80 B projection fixtures; compiled tests pass','Lens smoothing/update/input producer and camera resource assembly not recovered; body matrix constructor only static review'),
'circular_f32':('private_algorithm','50 B native lerp fixtures; compiled tests pass','Constructor serde normalization, NaN/tie endpoint policies not exhaustive'),
'comparative_space':('private_algorithm','Original B static policy review; compiled','Complete consumers and period=0/negative/nonfinite cases lack native differential'),
'easing_functions':('private_algorithm','119 B native easing fixtures; compiled tests pass','Out-of-range/NaN input and Bias constructors not exhaustively compared'),
'file':('original_dependency_wrapper_with_private_policy','B static formatting/path/copy review; candidate path test pass','Filesystem failure/result/logging and non-UTF8 paths not native differential'),
'float_grid':('private_algorithm_with_storage_adapter','B static two sampler policies; compiled','Fixed/dynamic grid consumer flags, NaN/edge extrapolation not original differential; ByteFloatGrid is explicit adapter'),
'halver':('original_dependency_wrapper','Uses original half2.4.1 f16 conversion and glam0.29.3 arrays; compiled','Generic arrays/Vec2 are safe adapters; original all-instance producer graph unverified'),
'index_condenser':('private_algorithm_using_std_containers','B remove static;72 original get inline-prefix cases exact; Cell cache/getter and clear statically bound to original PDB consumer; candidate reset tests pass','Insert/default producer key policy and full GPUWorld/startup consumer integration not established; clear/getter no independent original entry differential'),
'local_time':('original_dependency_wrapper_with_private_cache_policy','B init overwrites cache each call and UTC fallback static review; compiled','Startup call frequency and actual platform offset failure/refresh workflow not native runtime'),
'loop_range':('private_algorithm','50 B native LoopIdx fixtures; compiled tests pass','Full iterator ABI and exceptional/zero period paths incomplete'),
'name_hash':('original_dependency_wrapper','Uses xxhash-rust0.8.10 xxh3_64(seed0/default secret); B boundary static review; compiled','Actual asset-name producer and hash consumer integration unverified'),
'natural_cubic':('private_algorithm_using_original_glam','44 B constructors;120 B position +120 B tangent outputs; compiled comparisons pass','Repeated/degenerate/nonfinite/too-few controls and full periodic boundary stress untested; A/C untested'),
'rdp':('private_algorithm_using_original_glam','108 B original new_with_opts indices exactly match; compiled comparisons pass','Vec2 adapter not native differential; nonfinite/negative epsilon/extreme magnitudes/duplicate control inputs untested'),
'rgb9e5':('private_packing_policy','54 B native packing fixtures; compiled tests pass','NaN/infinity/signed-zero and complete serialized field consumers not exhaustive'),
'rng':('private_seed_transform_with_original_dependency_producer','44 B transformed states exact; typed producer calls original fastrand1.8.0 with_seed','Native typed ABI not claimed; downstream generation distributions/shuffle consumers remain external library'),
'selective_deserialize':('private_serde_visitor_strategy_with_original_dependency','B History/snapshots field constants and wrapper swallow-error/default static review;12 candidate nested/partial-success JSON cases pass','Actual snapshot payload type and original full visitor runtime differential unresolved; caller must use direct Deserialize, no synthetic EOF gate'),
'subtractive_1d_boolean':('private_algorithm_using_std_vec','158 B original interval results bitwise exact; compiled comparisons pass','Generated old upper_exclusive states, multiple sequential cuts and nonfinite/reversed ranges not exhaustive'),
'swizzlers':('original_dependency_type_adapter','4 original coordinate interfaces with256 payload-bit cases exact;6 original borrowed pointer/length passthrough cases; compiled','Vec3SliceAs2 original return wrapper/projected axis/element stride/borrowing aliases and actual consumer remain unresolved; safe adapter preserves source storage only'),
'threshold_range':('private_algorithm','Three B monomorphizations statically reviewed; compiled','NaN singleton high policy and callback retention behavior lack complete original-entry differential'),
'timer':('private_filter_policy_using_original_libraries','B init_state warmup1/dt1/60/deque4 restored;140 original delta transitions all bits exact','Standalone state/Default expresses original Local values; Bevy SystemParam integration and Time resource production not yet wired'),
'wall_space':('private_coordinate_policy_with_original_curve_bridge','B static methods; compiled; parent-owned Curve2 CurveSampling bridge now exists','Wall consumers, extrapolation/tangent/nearest-u coordinate chain needs original differential'),
}
files=[]
for name,(kind,checked,gap) in info.items():
    p=SRC/(name+'.rs');text=p.read_text(encoding='utf8');prod=text.split('#[cfg(test)]')[0]
    declarations=re.findall(r'\bfn\s+([a-zA-Z_][a-zA-Z_0-9]*)',prod)
    originals=[r for r in manifest['functions'] if f'utils::{name}::' in r['name'] or f'utils..{name}..' in r['name']]
    files.append({'original_file':f'crates/utils/src/{name}.rs','recovered_file':p.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'classification':kind,'candidate_functions':declarations,'function_count_is_lexical_not_original_coverage':True,'original_function_evidence':originals,'verification':checked,'status':'verified_partial' if 'fixtures' in checked or 'outputs' in checked or 'transitions' in checked or 'bitwise' in checked else 'implemented_partial','gaps':[gap]})
assert len(files)==22
differences=[
 {'module':'camera','rule':'Project normalized camera-right onto XZ; threshold fallback uses view forward; normalize_or(+X), upright basis, original glam Quat::from_mat3','difference':'Previous atan2/from_rotation_y used different quaternion chart/sign and degenerate fallback','fix':'Delegate normalized basis conversion to glam0.29.3; use fov.to_radians() matching original coefficient','recheck':'146 original rotation+80 projection cases pass'},
 {'module':'natural_cubic','rule':'Chord-length knots, scalar tridiagonal curvature solve, natural or clamped endpoint derivative, cyclic periodic boundary, polynomial position/tangent','difference':'Previous candidate only accepted caller coefficients; control-point producer and cyclic closure missing','fix':'Implement game scalar solve and constructors; preserve knot span difference in diagonal; use original glam only for vectors','recheck':'44 native constructor coefficients +120 position +120 tangent outputs pass; measured errors in verification.json'},
 {'module':'timer','rule':'Original Local starts warmup=1, previous=1/60, deque capacity4 head0 len0','difference':'Previous only from_observed_state permitted arbitrary external initial state','fix':'Default now supplies actual original init_state values; retain explicit resumed-state adapter','recheck':'140 original stateful steps exact returns/previous/warmup/sample bits'},
 {'module':'selective_deserialize','rule':'Select History then snapshots; ignored errors retain any completed selected result; missing result defaults','difference':'Previous generic field adapter had no concrete original nesting; from_slice EOF validation would impose an extra consumer gate','fix':'Add exact B marker types and composed SaveHistorySnapshots<T>; test direct Deserialize consumer','recheck':'12 candidate nested/error/duplicate cases pass; original full visitor runtime NOT executed'},
 {'module':'local_time','rule':'init_local_time refreshes cache on every invocation','difference':'Previous comment incorrectly claimed one-time guard','fix':'Correct contract; preserve refresh behavior; startup frequency left separate','recheck':'Original0x140c91d60 directly writes global cache without guard'},
 {'module':'rng','rule':'Private transform creates initial state; original fastrand1.8.0 with_seed stores that u64 directly','difference':'Previous comment incorrectly warned that with_seed would transform again','fix':'Read installed original dependency source; correct comment and provide typed original-library adapter','recheck':'44 native state fixtures unchanged; RNG generation remains original dependency'},
]
batch_path=OUT/'interface-batch-02/interface-state-updates.json'
if batch_path.exists():
    differences += [
        {'module':'swizzlers','rule':'Vec2 x0y/xvy and Vec3 x0z/xvz keep original lanes; Vec3 xvz replaces middle y with its parameter','difference':'Previously only free xvy adapter existed; remaining original trait interfaces had no restored business comparison','fix':'Add original receiver trait names and lane policies with original glam Vec3::new','recheck':'256 original coordinate cases bitwise match; no synthetic adapters counted as new original interfaces'},
        {'module':'swizzlers','rule':'[Vec3] as_vec2 transports original borrowed pointer and length without repacking','difference':'The common identity native address has141 aliases and a misleading416-byte next-public upper bound; source consumer semantics are not derivable from that body','fix':'Bind B PDB procedure size4 and restrict Ghidra body; provide safe borrowed transport without unsafe Vec2 slice casting','recheck':'6 original transport cases; projected consumer type/axis/stride remains an explicit gap'},
        {'module':'index_condenser','rule':'Shared Condex borrow refreshes Cell cached index only when fallback finds key; missing key preserves cached value','difference':'Previous cached_index:u32 forced mutable handle borrow and did not expose original get/last_known_index APIs','fix':'Use original std Cell<u32>; implement get(&Condex) and last_known_index, route recovered remove through get','recheck':'72 original inline-prefix cases exact returns/cache; PDB IPI0x74E3/0x74F0 binds original consumers'},
        {'module':'index_condenser','rule':'clear empties dense keys and map through original std clear while preserving capacities','difference':'Previous candidate lacked clear, leaving original reset consumer interface untouched','fix':'Call Vec::clear and HashMap::clear; old Condex keys fail get after clearing','recheck':'Original startup PDB IPI0x74EB clear region statically compared; candidate reset/capacity test pass; whole original startup not executed'},
    ]
report={'schema':'shared-foundation-delivery-v2','owner':'foundation_utils_base','date':'2026-10-13','baseline':'B','status':'verified_partial','source_original_claimed':False,'modified_global_progress':False,'original_binary_sha256':manifest['sha256'],'files':files,'business_difference_repair_records':differences,'verification':verification,'native_fixture_cases':{'inherited_six_families':367,'camera_xz':146,'camera_projection':80,'rdp_indices':108,'spline_constructors':44,'spline_position_outputs':120,'spline_tangent_outputs':120,'timer_transitions':140,'interval_subtractions':158},'candidate_only_cases':{'selective_nested':12},'library_boundary_audit':{'reimplemented_external_libraries':[],'original_dependencies_reused':['glam0.29.3','fastrand1.8.0','half2.4.1','serde1.0.200','serde_json1.0.85(validation only)','xxhash-rust0.8.10','time0.3.36','std collections/filesystem/locks/format/math'],'rdp_ownership':'utils::rdp::Rdp::new_with_opts original private entry0x141fe0b90; callees Vec/iterator/allocation/panic only; no external simplifier restored','spline_ownership':'utils::natural_cubic original private entries0x140c926b0/0x140c930f0/0x140c93970 -> private scalar0x140c94a10/0x140c95200 and game closures141941180/141941510; bumpalo allocation and std Fn forwarding not reconstructed','synthetic_validation_tools':['native_camera.py','native_spline_rdp.py','native_small.py','query_native.py','embed_spline_rdp.py','validate_final.py','validate_selective.rs','finalize_delivery.py'],'tools_do_not_count_as_game_code':True},'evidence':['native-query.json','extras-query.json','camera-native-fixtures.json','spline-rdp-native-fixtures.json','small-native-fixtures.json','verification.json','../utils-tabloid/native-manifest.json','../utils-tabloid/B-symbols.json'],'full_original_game_runtime':False,'ownership_released_on_final':True}
if batch_path.exists():
    report['native_fixture_cases'].update(interface_batch_02_coordinates=256,interface_batch_02_borrowed_transport=6,interface_batch_02_original_inline_get=72)
    report['interface_coverage']='interface-coverage.json'
    report['evidence']+=['interface-batch-02/interface-state-updates.json','interface-batch-02/inline-bindings.json','interface-batch-02/query-bounded.json']
    report['library_boundary_audit']['original_dependencies_reused'].append('std::cell::Cell (shared Condex cache)')
    report['library_boundary_audit']['validation_only_raw_table']='Original HashMap/SipHash/native lookup executes in oracle; valid raw buckets are supplied as fixture storage, not a delivered hash/container algorithm.'
(OUT/'delivery.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
md='''# utils 顶层 22 文件交付（B，业务行为恢复）

实现范围按原源码文件，不宣称作者文本或全部 API/ABI 复刻。lib/curve/geometry/components 由其他 owner 负责。本轮发现并修复 camera 四元数 chart/fallback、Spline 控制点生产者与周期闭合、Timer 启动 Local、History→snapshots marker、local_time 初始化说明与 RNG 依赖说明。RDP/区间候选经过原入口复查，没有在已验证域发现输出差异。

源码仅恢复游戏策略与私有算法。glam/fastrand/half/Serde/xxhash/time/std 均调用原依赖；没有新写向量、四元数、RNG 生成器、hash、IEEE half 转换器、JSON parser 或通用容器。RDP/Spline 自有算法归属由原 utils PDB 定义与原入口/私有 scalar callees 支持；其 bumpalo/std 胶水不计恢复。Unicorn 分配与 memcpy hooks 仅为验证工具，不作为游戏实现。

## 真实原码与候选比较

继承六族原码 367 案例，本轮新增原入口 916 个输入/状态/输出记录（camera226、RDP108、Spline构造44/位置120/切线120、Timer140、区间158）。这些输入域内规则比较通过。Spline 比较已收紧到 1e-5 的相对/单位绝对门限；实际最大系数误差 4.7683716e-7、位置 2.861023e-6、切线 4.7683716e-7，来自 f32 等价代数和求解顺序变化；测试覆盖非均匀弦长、自然/单端/双端导数、周期闭合及 t clamp，未借容差放宽任何枚举/索引/端点/分支选择。代表性控制点坐标在 ±10；退化/重复点/巨大坐标与完整周期压力尚未验收，不能外推全数值域保证。

Timer 初值是0x14179a010直接写入的 warmup=1、previous=1/60、容量4空队列；140 连续步骤对返回值及全部 Local 状态逐bit比较。Camera xz通过原glam基变换而非自写数学；146旋转样例含四象限、near-vertical fallback及1e-5阈值邻域。Serde候选12场景直接调用 Deserialize，保留完成值后出错的 partial-success；原完整嵌套 visitor 差分尚未运行，Snapshot 实际类型由消费者提供。

## 按原文件列出的功能、验证和缺口

| 原文件 | 分类 | 候选函数 | 验证 | 缺口 |
|---|---|---|---|---|
'''
for f in files:md+=f"| `{Path(f['original_file']).name}` | {f['classification']} | {', '.join(f['candidate_functions'])} | {f['verification']} | {f['gaps'][0]} |\n"
md+='\n当前真实检查和源码 hash 以 [verification.json](verification.json) 为准；全 utils 测试数包含其他 owner 的 geometry/curve 测试，不能计作本 owner 的原函数数。完整游戏消费者/Bevy资源/ECS/场景均未验收。后续优先闭合 FloatGrid、WallSpace、IndexCondenser 真实消费者以及存档GC payload，随后补退化输入域。\n'
if batch_path.exists():md+='\n接口批次02新增256坐标原入口/6借用运输属性/72原get内联片段案例，按8个原接口更新分析和实现状态；运输属性对照不等于Vec2 projected consumer已恢复。详见 [接口表](interface-coverage.md) 和 [批次02合同](interface-batch-02/CONTRACT.md)。\n'
(OUT/'CONTRACT.md').write_text(md,encoding='utf8')
print('delivery files',len(files),'whole_utils_checks_passed',verification['all_passed'])
