from pathlib import Path
import json, hashlib, collections, datetime
OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
BASE = ROOT/'evidence/shared-foundation-2026-10-13/geometry-curves'
CRATE = ROOT/'reconstruction/shared-foundation/crates/utils'
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def relative(p): return p.relative_to(ROOT).as_posix()
audit = json.loads((BASE/'interface-coverage.json').read_text(encoding='utf-8'))
extra = json.loads((OUT/'extra-triangle-fixtures.json').read_text(encoding='utf-8'))
old = json.loads((BASE/'triangle-fixtures.json').read_text(encoding='utf-8'))
files = sorted(p for group in ['components','curve','geometry'] for p in (CRATE/'src'/group).rglob('*.rs'))
hashes = [{'path':relative(p),'sha256':sha(p)} for p in files]
findings = [
 {'id':'GEO-01','severity':'P1','kind':'business_logic_defect','status':'fixed',
  'location':'crates/utils/src/geometry/triangle_triangle/cross_intersect.rs:9',
  'original_interface':'utils::geometry::triangle_triangle::cross_intersect::cross_intersect',
  'evidence_kind':'original B entry execution plus retained private decompilation',
  'original_entry':'0x141fdfd80','private_callee':'0x1424181a0',
  'before':'Plane-side tolerance was incorrectly treated as vertex snapping, followed by sorted interval clipping; original chooses edges using tolerant tetrahedron signs and allows edge-line extrapolation.',
  'reproduction':{'artifact':relative(OUT/'extra-triangle-fixtures.json'),'case_indices':[38,48,55],'before_candidate_tag':3,'original_tag':1,'input_scale':0.001},
  'fix':'Use original tolerant edge selection, glam Mat4 determinant and edge-plane arithmetic; retain emitted endpoint order and finite checks.',
  'validation':'304 new original-entry cases and 257 prior cases pass classification and ordered endpoint checks; bounds on numerical residual remain explicit.'},
 {'id':'GEO-02','severity':'P2','kind':'dependency_reimplementation','status':'fixed',
  'location':'crates/utils/src/curve/mod.rs:166','original_interface':'utils::curve::Curve::get_coord_at_u',
  'evidence_kind':'original native query fixture and source dependency audit',
  'before':'Candidate hand-coded sorted-slice binary search, duplicating std.',
  'fix':'Use slice::partition_point(value <= u); preserve original right bias, clamps and segment interpolation.',
  'validation':'41 original Curve2 inputs / 328 coord-position-tangent samples, including repeated and short segments, still pass exact bit checks.'},
 {'id':'GEO-03','severity':'P1','kind':'functional_completeness_gap','status':'open',
  'location':'crates/utils/src/curve/mod.rs:52','original_interface':'CurveCoordSemanticThresholded3D and generic Curve/CurveSlice APIs',
  'evidence_kind':'PDB typed instantiations and current authoritative interface audit',
  'gap':'Thresholded3D length/sampling policy is not implemented. Curve3 query policies, transform required-component wiring, source schema and caller closure policies are not fully recovered. Candidate source existence is not full original interface validation.',
  'fix':'Requires original type/consumer evidence; no invented substitution or placeholders added.'},
 {'id':'GEO-04','severity':'P2','kind':'performance_risk','status':'open',
  'location':'crates/utils/src/curve/mod.rs:229','original_interface':'utils::curve::Curve::reverse',
  'evidence_kind':'static candidate source and original 0x14146c190 retained decompilation',
  'gap':'reverse reverses then clones all points and reallocates cumulative lengths, adding O(n) copy and two allocations. Original observed entry also clones; therefore this is a potential optimization, not an original behavior mismatch.',
  'benchmark_verified':False,'fix':'Kept unchanged pending actual consumer hot-path evidence; no speedup claim.'}
]
missing = [{k:r.get(k) for k in ['interface_id','original_source','ownership','status','method','original_addresses','gap']} for r in audit['interfaces'] if not r.get('has_recovered_implementation')]
slice_rows=[]
for r in audit['interfaces']:
    if r['original_source'].endswith('/curve_slice.rs'):
        method=r['method']
        slice_rows.append({'interface_id':r['interface_id'],'current_audit_status':r['status'],
          'candidate_method':{'points_len':'CurvePoints::len','points_iter':'CurvePoints::iter','loop_points_iter':'LoopCurvePoints::iter','is_empty':'CurvePoints::is_empty','len':'CurvePoints::len (receiver ownership requires confirmation)','iter':'CurvePoints::iter (slice/loop receiver ownership requires confirmation)'}.get(method,'CurveSlice::'+method),
          'review_scope':'Borrowed ordinary and LoopCurvePoints views implemented; candidate does not establish original semantic strategy, caller closure, generic specialization or consumer acceptance.',
          'native_fixture_scope':'Extension Vec3-to-XZ donor projection; coordinate constructors and inclusive point-center ranges only; no blanket acceptance of all 17 interfaces.'})
validation={
 'schema_version':1,'recorded_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'source_hashes':hashes,'cargo_lock_sha256':sha(CRATE.parents[1]/'Cargo.lock'),
 'commands':[{'command':'cargo test -p utils --locked --offline','exit_code':0,'tests_passed':57,'scope':'Full utils crate at validation time; includes two concurrent sibling-review tests.'},
             {'command':'cargo clippy -p utils --all-targets --locked --offline -- -D warnings','exit_code':0}],
 'original_binary_sha256':extra['original_sha256'],'whole_game_execution':False,
 'new_original_entry_cases':extra['case_count'],'new_categories':dict(collections.Counter(r['expected']['tag'] for r in extra['cases'])),
 'new_input_categories':{'random_scale_sweep_1e_minus4_to_1e8':245,'thin_triangles_and_endpoint_shifts':32,'nan_positive_negative_infinity':27},
 'prior_original_entry_cases':len(old['cases']),'prior_categories':dict(collections.Counter(r['expected']['tag'] for r in old['cases'])),
 'comparison':{'categories':'exact result category','new_endpoints':'ordered endpoint distances <= max(abs(inputs),abs(expected))*3e-5+1e-10','prior_endpoints':'ordered endpoint distances <= 3e-5','bit_identical_endpoints_claimed':False},
 'execution':'Original B 0x141fdfd80 and original private callees executed in Unicorn; no source model replacing private instructions. Existing allocator/CRT hooks only. RNE SHA checked on import.',
 'artifacts':[{'path':relative(p),'sha256':sha(p)} for p in [OUT/'extra_triangle_oracle.py',OUT/'extra-triangle-fixtures.json',OUT/'extra_triangle_tests.rs',BASE/'triangle-fixtures.json',BASE/'triangle_tests.rs',BASE/'141fdfd80.c',BASE/'1424181a0.c',BASE/'142418dc0.c']],
 'performance':{'triangle_candidate_heap_allocations_static_before':2,'triangle_candidate_heap_allocations_static_after':0,'triangle_candidate_sort_operations_static_before':2,'triangle_candidate_sort_operations_static_after':0,'latency_speedup_measured':False,'whole_game_performance_measured':False},
 'limitations':['PDB/interface inventory is incomplete.','No acceptance promotion for CurveSlice 17 APIs; unresolved strategy/caller variants persist.','The original full consumer chain and gameplay behavior have not been verified.','Original game-private control policy is reconstructed; generic math/search/container/iterator dependencies are reused.']}
review={'schema_version':1,'scope':[relative(CRATE/'src'/g) for g in ['components','curve','geometry']],
 'findings':findings,'source_hashes':hashes,'native_review_validation':relative(OUT/'validation.json'),
 'authoritative_audit_input':{'path':relative(BASE/'interface-coverage.json'),'sha256':sha(BASE/'interface-coverage.json'),'known_interfaces':audit['known_interface_count'],'status_counts':audit['status_counts'],'counts_are_prior_authoritative_status_not_review_promotions':True},
 'function_implementation_gaps':missing,'curve_slice_interface_review':slice_rows,
 'dependency_audit':{'reused':['glam 0.29.3 vectors/cross/dot/normalize/Quat/Mat4 determinant','lerp 0.4.0 circular interpolation','itertools 0.14.0 tuple_windows','take-until 0.2.0 inclusive stop policy','std Vec/slice/iterators/partition_point'],
   'removed_duplicate_implementation':['hand-coded binary search'],
   'retained_private_rules':['XY/XZ/XYZ metric choice','curve cumulative length and repeated point policy','directed circular interval','triangle tolerant edge selection','ray/rectangle/circle boundary and float operand order'],
   'custom_iterator_wrapper':'CirclePoints2 is a thin private circle sampling policy wrapper around iteration; no independent math/container library implementation.',
   'scope_limit':'These owned directories only; does not certify all historic restorations or resolve engine-fork commit.'}}
for filename, data in [('review.json',review),('validation.json',validation)]:
    (OUT/filename).write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
lines=['# components / curve / geometry 独立代码 Review','',
 '发现并修复 1 项 P1 业务缺陷和 1 项 P2 重复实现。功能仍未完善，原游戏调用链尚未验收。', '',
 '| 编号 | 严重度 / 类型 | 结论 |','|---|---|---|',
 '| GEO-01 | P1：三角交叠漏判 | 已修复。3 组 0.001 尺度输入原作返回 Segment，候选返回 Disjoint。 |',
 '| GEO-02 | P2：重复实现 std 搜索 | 已改为 slice::partition_point，保留右偏规则。 |',
 '| GEO-03 | P1：功能缺口 | Thresholded3D / 泛型策略 / 原消费者链仍未闭合。 |',
 '| GEO-04 | P2：性能风险 | reverse 全点 clone 和重新分配，未基准测量。原作入口也有 clone，未擅改。 |','',
 '## 业务缺陷与修复证据','',
 '原 B `0x141fdfd80 → 0x1424181a0` 先按 ±1e-10 的四点 determinant 选两条边，再计算边所在直线与平面的交点。容差用于选择边，不能把容差内顶点直接吸附到平面，也不能把求交参数夹到线段内。候选原来的 plane_cut + 两区间裁剪在小尺度下漏判。修复直接表达该业务策略，调用 glam 的 determinant / cross / normalize，保持原输出的端点顺序。', '',
 '新增 **304 个真实原入口案例**：245 个 1e-4～1e8 尺度随机三角、32 个细三角/端点移动、27 个 NaN/±Inf；旧 257 案例继续通过。分类精确一致；新增有序端点残差界为 max(abs(inputs), abs(expected)) × 3e-5 + 1e-10，旧有序端点保持绝对 3e-5。没有声明端点逐位一致。新增案例 38 / 48 / 55 是实际失败复现。原指令在 Unicorn 执行，发行 RNE SHA 固定；未启动完整游戏。', '',
 '## 性能与依赖','',
 '修正后的三角交叠热路径全部使用固定大小的栈上值，静态去掉每次调用两次 Vec 分配及两次排序。未执行延迟对比，不由此宣称整体提速。最近曲线查询为 O(log n)，近点搜索、裁切测长和延伸均为 O(n)；审查范围内没有发现新的 O(n²) 路径。reverse 的 O(n) clone/重建风险单列，原消费者调用频率未知。', '',
 'glam / lerp / itertools / take-until / std 直接复用。唯一确认的通用轮子是 Curve::get_coord_at_u 中的手写二分，已删除；328 个原 Curve2 采样继续逐位通过。标量乘加顺序、投影选择、循环区间、碰撞边界是有原入口证据的私有规则，不能为消除表面算术代码而任意更改。CirclePoints2 是采样策略包装，不是重写迭代器库。', '',
 '## CurveSlice 审查范围','',
 '现有状态对象、端点、坐标迭代、remap、延伸源码已经编译并通过现有 fixture；普通借用点集及逻辑 LoopCurvePoints 不复制底层点集。延伸以 tuple_windows 和原 take-until 依赖实现，停止顶点包含在输入中。零长度边导致 NaN、零跨度保留双端点等行为来自原指令，不能当成随意修复的缺陷。', '',
 '但原 Thresholded3D 采样策略、所有 remap/stop 具体调用闭包、所有 RangeBounds 变体和原游戏消费者未验收。四组 fixture 不能代表 17 个原 API 全面完成。逐接口记录如下：', '',
 '| 原接口 | 当前候选对应 | 仍缺 |','|---|---|---|']
for r in slice_rows:
    lines.append(f"| `{r['interface_id']}` | `{r['candidate_method']}` | 泛型/调用方策略及原消费者闭合；当前权威状态 `{r['current_audit_status']}` 保留 |")
lines+=['','## 原接口未实现清单','',
 '以下来自当前权威接口审计。Review 不借源码存在或局部测试自动提升接口状态；CurveSlice 的新候选映射已在上表单列。源表的完整 inline 分母仍未知。', '',
 '| 原文件 | 未实现原接口 | 原状态 / 归属 |','|---|---|---|']
for r in missing:
    lines.append(f"| `{r['original_source']}` | `{r['interface_id']}` | `{r['status']}` / `{r['ownership']}` |")
lines+=['','## 验证与输入快照','',
 '当前 `cargo test -p utils --locked --offline`：57/57；`cargo clippy -p utils --all-targets --locked --offline -- -D warnings` 通过。测试总数包含并行其他 owner 的新增测试，不全部记为本 Review 新增。精确源文件哈希、原入口类别、fixture 哈希与功能限制见 [validation.json](validation.json)，机器可读逐接口缺口见 [review.json](review.json)。本报告没有做原游戏 consumer/GPU/整帧/整游戏性能验收。']
(OUT/'REVIEW.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
print(json.dumps({'reviewed_source_files':len(hashes),'findings':len(findings),'function_gap_rows':len(missing),'slice_api_rows':len(slice_rows),'native_cases':extra['case_count']+len(old['cases'])}))
