# G04 主墙构造阶段证据（B 已验证，A/C 身份对照）

## 已闭合的 CPU 链

- 原 `split_into_random_rows` 14123e110 → 游戏 `utils::random_splits{,_into}` 140c90350/140c90050。Rust 复用 fastrand 1.8.0，保持游戏分段数量、0.495 上限、0.21/span jitter、0.046/height 去近边规则。160 条原指令随机输入通过。
- 原 `trim_rows` 14123eef0 → 私有 `split_heightmap_by_threshold<RowTrimmerSink>` 141fe0750 → 阈值穿越、横向量化、窄条删除/特殊扩宽、相邻 row 引用范围。200 条全部 32 字节原输出通过；Ghidra C 错误删除核心 blocks，因此 B-trim-native.asm 是原 PE 完整 Capstone，而非只依赖 C。
- 原 `FinalizedWallHoles` iterator 14123fc70 + from_curve stable sort：保留 primary、闭合墙周期候选、归一化、按 max_x 稳定排序。120 条原 owner 前段指令输出通过；padding 只影响周期候选，不扩张实际 bounds。
- 原 `resolve_hole_overlap` 1412411c0 全业务，包含中心拆分递归、角分类、seed、deformation、两种 split flag 和 deque 顺序。320 个场景（后100个多洞）全部有序 38 字节通过；40 字节记录最后2字节是 padding，未伪造。
- 原 from_curve 14076c0ed..14076c092 输出消费者：真实 96 字节 CPU `BrickTransformSsbo`（48 affine +16 deformation +12 opaque surface +4 seed +4 material +8 wall_distance +4 flags）。从原完整 from_curve 的矩形/圆墙、空洞/单洞调用捕获880条实际记录，Rust全部96字节相同。

验证命令：在 reconstruction 执行 `cargo build --release --bin wall-constructor-probe`，在项目根执行 `/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_constructor.py`。
最新报告 `B-wall-constructor-verification.json`：3516/3516（9阶段），0差异。其中实际 proposal owner 712组、已闭合 plain proposal→cuts/split→96B consumer 707组；完整96B instance 880条。所有原game helpers执行原指令，没有用 Rust candidate 替代 oracle helper。

## 不能宣告完整的部分

Rust 还未恢复 from_curve 的 row→proposal 中段：相邻行匹配、横向 partition owner、noise 和 seed 来源、authority/支撑高度采样、surface/depth选择与 special/fence/roof 分支。`WallEmissionContext` 明确要求这些已 resolved 的源值，**不是**完整 from_curve API；不得用 uniform proposal、零deformation、默认seed或默认surface补齐后标为完整。

full native fixture 已在矩形/圆墙、空洞/单洞/多洞场景执行至 return。此前多洞 bridge 问题已按原 PE IAT 纠正：142923bf0=memcmp（IAT143193020）、142923c00=memcpy（IAT143193028）、142923c10=memmove（IAT143193030）、142923c20=memset（IAT143193038）。完整多洞原指令现已可运行。

## 原 caller 参数（B construct_walls 14075925f → from_curve 140769760）

Windows x64 caller：RCX normalized-hole output Vec owner，RDX Curve2，R8D material/id opaque，R9 arch/height array。
call 前 stack +20 arch count、+28 第二个 authority height ptr、+30 count、+38 第三个 support/elevation ptr、+40 count、+48 construction kind byte、+50 **u64 seed值**、+58 FinalizedWallHoles ref、+60 row-end perturbation f32、+68 output Vec<96B> ptr、+70 lower/base f32、+78 [5f32 +tag byte20] top/rim bounds ref、+80 option flag/+88 f32、+90 option flag/+98 f32、+A0 wall float、+A8 bool（construct_walls真实调用为0）。from_curve 入口各 stack offset 增加8；RBP=entry RSP-0x5a8。

Curve2布局 cap0/points_ptr8/count16/U_cap24/U_ptr32/U_count40/length48。FinalizedWallHoles cap0/ptr8/count16/length24/closed28；单hole20B，aabb0..15/kind16/origin17。MyBrick40B：aabb0..15/deformation16..31/seed32..35/split_y36/split_x37/padding38..39。WallRow32B：next start0/end8/normalized y16,20/U24,28。

## 模块归属与依赖筛除

`three-version-provenance.json` 包含 A/B/C PDB source 文件身份、源MD5及六个 canonical候选。三个版本皆归原 `crates/systems/wall-constructor/src/{wall_constructor.rs,utils/*.rs}`；utils/geometry/split_heightmap 和 utils/random_splits 是私有游戏几何，不能当 cargo dependency替换。标准 Vec/VecDeque/sort、fastrand、CRT数学与puffin为依赖，Rust复用并在oracle bridge列表声明。A/C仅提供身份/版本对照，**本报告不声称 A/C 行为等价**。

原源码导出：`../building-integrate/B-core/{140757800,140769760}.c`；`../building-integrate/B-consumers/`；`../building-integrate/B-wall-owners-next/{140c90350,140c90050,141fe0750}.c`。C normalize_holes 1408105d0、split_brick_by_holes 140810670 尚待导出，不阻碍B阶段验证。

## 稳定 Rust API 给父 agent

`WallBrick`、`WallRow`、`WallBrickInstance`、`WallEmissionContext`；`split_into_random_rows`、`trim_rows`、`normalize_wall_holes`、`resolve_hole_overlap`、`cut_wall_bricks`、`emit_resolved_wall_brick`。
父接World时只能声明已恢复 raw authority→rows/holes normalization 与 resolved proposal→hole cutting→actual CPU instances；尚缺 row→proposal 桥。删除消费者可清理这些真实输出实体，但不能使未恢复 upstream 变成完整消费者。

## 新增安全 consumer 闭环

`consume_wall_proposal` 恢复 original from_curve 的外围 seed modulation、原洞 intersects 采样与 stochastic horizontal split，并保持完整 ordered proposals。`emit_plain_wall_brick` 只针对 construction kind2/top tag2/fence-fit false：各切割 piece 的 world Y center 重新计算 flags（abs(y-height_scale)<0.95 →0x14，否则4）。原 curve_ends/row_u/depth_axis/surface 为 proposal 级共享源值，原完整调用每组所有pieces已断言相同；其上游采样仍未恢复。

稳定安全 API：`construct_resolved_plain_wall_proposal(brick, normalized_holes, context, row_height, split_draw)` 直达真实96B CPU输出，调用者明确提供已resolved原 proposal/context/RNG draw。不得称完整 from_curve，也不得自行生成这些未恢复的源值。父可接真实 Bevy proposal→CPU instance owner 与删改清理；raw authority rows 尚缺到proposal的中段生产边。

B-proposal-cases.json现为重新采集的712组实际original proposals/consumer输出，B-emission-cases.json为880条actual实例。旧B-resolved-plain-chain-verification.json/B-emission-verification.json已标记superseded；当前统一报告707组consumer闭环通过。

Consumer `holes` 的严格契约是原 row collector 选出的 normalized candidate slice（不是任意全局洞列表）。当前 plain 样本没有 row-end perturbation，完整全局洞与 candidate 对输出相同；有扰动/复杂authority branch不能泛化该事实。World 必须明确给出已resolved候选源，或公开保留collector未恢复边。

## 原 row-hole collector 已恢复

`select_native_row_holes(row_bounds, normalized_holes, length, height_scale)` 返回 `NativeRowHoleCandidates { source_indices, holes, suffix_min_x }`。B原 from_curve 14076a293..14076a498整段指令及原geometryhelpers执行验证372例（320随机、48 ULP/tolerance边界与重复、4次序/NaN）。row_bounds是normalized [u_min,y_min,u_max,y_max]；扩张half=.46/length*.5、.46/height*.5；边界相等inclusive；保留输入normalize的stable max_x order，不在collector重新排序。反向min(min_x-half_padding)生成实际suffix阈值，World可从真实 storage 改动重新选该 `.holes`。

追加CRT IAT审计修正：142924060→IAT1431931d0→truncf，旧研究fixture将它误认cosf，影响raw proposal noise。native_fixture已改正，全部proposal/emission来源重新采集，当前统一验收使用真实truncf。B-CRT-IAT.json保存所有桥身份；不把研究fixture的旧错误解释成原程序行为。

## 下批中段攻关入口（本批冻结后只读分析，尚非 Rust acceptance）

原 B body 可继续按以下 source owners 收敛；不要用本批末段捕获值代替这些 owner。

1. **已证明 raw row horizontal partition**：`split_plain_row_horizontal(row_u,length,source_seed,boundary_count,row_index)`。kind2/tag2 每个 row span `(u1-u0)*length`，边界数 `ceil(span/.46)+1`，jitter `.32/span`；RNG seed 为原 u64 wrapping `source_seed*0x154df + vertical_boundary_count*0x159 + trimmed_row_index*0x96a3dd`。vertical boundary count 是 split_into_random_rows 输出长度，不是 curve point count；row index 是 trim_rows 真输出32B行序号。45个完整原from_curve owner样本（多seed/高度、非均匀authority裁短U）位相/边界全f32一致，已并入统一报告。
2. **逐 proposal 两条 RNG owner**，原14076a239..255和14076b15c..19a：split_draw stream 候选seed `source_seed*0x91e + boundary_count*0xdf + row_index*0x91fd`；depth stream候选seed `source_seed*0x15b3 + boundary_count² +0x2a6 + row_index`。depth取draw后原式 `max(depth_base+(draw-.5)*.07,.05)`。当前仅指令表达式/局部source指针已识别；必须追被跳过proposal是否也消耗draw，以及复杂row/chunk顺序后才可写Rust。
3. **noise owner**，14076acb6..14076ae93：对源 normalized y 乘height_scale得到物理高度，`hash(y)=fract(sinf((y*10 +seed_as_f32)*12.989800453186035)*43758.546875)`，这里fract实现是真实truncf subtraction，不能误用cosf。下一hash阶段对两倍hash再做sinf/乘constant/truncf subtraction，并乘1000获得phase。14076b746..8d9按物理横坐标、`.2` envelope、下/上两层hash/phase生成`.06`变形；是否将upper置0来自下一行实际U覆盖，并非固定每row都有noise。需整组proposal原40B输出验证，不接受只验证hash叶函数。
4. **原 MyBrick seed owner**：14076a0f3..fa先建 `boundary_count*0x924 + source_seed_u32 +10`，14076af61..6e加 `trimmed_row_index*0x54`，14076b576..586再加 `column_window_index*0x171`。原40B seed是u32位图，不是float；0x480这个stack slot在emitter阶段被surface值复用，不能按末段同槽位推raw类型。后续围绕相邻边界window顺序/跳过分支恢复后再验收。
5. **Curve与authority采样**：父补导出 `evidence/building-integrate/B-row-curve/1411f8c10.{c,asm.txt}` 已确认原private CurveU右偏二分/finite/assert/inverse_lerp，与现有 `reconstruction/roof/surface.rs::Curve2::coord_at_u`契约相同，下一优先复用已恢复模块。from_curve还自己从row_u进入forward coordinate walker，跨点取arch/terrain/support数组的端点/中心高度；roof-tag、nonzero support/terrain与扰动分支不可当已完成。先恢复 kind2/tag2/bool_A8false、原输入两个附加authority数组确实为0的合法分支，随后扩非零数组；不得把0数组作为未证明世界默认值。

本批生产模块/probe已冻结，父可重跑global绑定。剩余验收标准：真实 World authority直接产生proposal/RNG/context，撤除手工resolved input requirement，并完成原完整from_curve输出数组逐字节对照；当前collector与CPU实例链的真实Bevy接入不自动满足这条标准。
