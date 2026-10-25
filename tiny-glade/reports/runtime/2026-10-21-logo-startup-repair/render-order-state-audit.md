# 独立渲染顺序与状态审计 — 2026-10-21

范围：真实 Loading/Logo → gameplay 世界帧 W1–W54 → compile_main 尾部 → egui → resolve/present。T13 反馈捕获按用户要求排除。审核者 `render_order_state_audit`，中央独立验收。本报告不是全游戏完成证明。

## 结论与此次修复

除中央已修的 UI/Logo premultiplied-alpha 缺失，本次独立审计发现并窄修 **3 项原版明确差异**：

| 优先级 | 环节 | 原版证据 | 原实现缺陷 | 本次恢复 |
|---|---|---|---|---|
| P2 | Loading MEI | `140165347 xor r9d,r9d` → `14016534a call 1409c6700`；callee `1409c6726`保存 r9d、`1409c6aff`检验该字节 | 强制分配 mei temp 并做 UI shadow combine，选择了原版不走的分支 | `loading_graph.rs` 调用 `mei_pass_with_shadows(..., None, ...)`，直接混合到 loading target |
| P1 | T11 session previews | `1408f83b0 mov [rbp+7c0],0`；`1408f8404 lea r9,[rbp+7c0]` → `1408f8411 new_impl` | `ColorSpec::tag2` 使用 DONT_CARE/write-only，预览四边形无法保证保留已完成的底图，图中也丢失保持内容的依赖 | `session_preview_host.rs` 改为原 tag0 LOAD/read-write |
| P2 | T12 LUT gallery | `1408b0b13`从`142a9cc30`加载 tag1+RGB；`1408b0b23`写 alpha1；`1408b0b6d`传附件地址 | tag2 丢弃底图但没有恢复原固定背景清除，未覆盖区域不确定 | `world_postprocess.rs::run_lut_gallery` 使用既有 `ColorAttachment::View`，精确清除 `[0x3e3851ec,0x3e3851ec,0x3e3851ec,0x3f800000]` 即 RGBA `[0.18,0.18,0.18,1]` |

**纠正审核过程中的中间判断**：早期 retained decomp 中相邻 `=2` 临时值被误认为 T11/T12 颜色 tag。重新对 SHA 绑定的原 B 机器码追踪传入 `new_impl` 的 r9 后，证实 T11=0、T12=1；本报告以该直接证据为准。MenuBG 的颜色 tag2 则有独立指令证明，保持原样。不能因三个函数都“画菜单”而给它们套同一个附件策略。

本 agent 修改生产文件只限 `loading_graph.rs`、`session_preview_host.rs`、`world_postprocess.rs` 的上述落点；未修改 rhapsody、shader、调度、业务门控或 T13。`original_loading_gpu.rs` 移除旧错误的“必须有 shadow combine”断言，新增原分支和实际图顺序断言，保留原全部像素与 alpha 断言。

## 证据及验证等级

- **O（原证一致，限定字段）**：本次从原 B `D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe` 做只读静态解码；SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。仅按 PDB 边界解码 15 个指定过程，没有运行原 EXE。见 [manifest](render-state-evidence/manifest.json) 和 [复现脚本](render-state-evidence/probe.py)。
- **F（确定缺失/冲突，源码已修）**：上述三个差异。源码恢复不自动等于 UI 分支实际运行验收。
- **U（证据不足/未验证）**：未逐叶展开的计算/回调算法、没触发的分支、物理图像效果、GPU/WSI completion 等，必须单独保持 U。
- **X（用户排除）**：T13。不能把排除计为已经恢复。

`render-state-evidence` 保存本次原始指令文本、每个原过程长度/符号/字节哈希，以及 LUT clear 常量原始 bytes。旧 retained decomp 只用于导航，不用源码注释或当前测试替代原始字段证据。

中央已回报：Loading 修复后真实 GPU fixture 在 **960×540** 通过实际 `Vermeer combine < Mei 2D < Logo UI` 顺序及没有 shadow combine 的断言，像素黑色比例 0、alpha255。该事实仅证明此有界 fixture；T11/T12 新修在本报告写入时尚待中央 build/针对性验证。此 agent 没有运行 Cargo/GPU/游戏，也没有启动全屏窗口。

## Loading 链矩阵

| 顺序 | 当前生产落点 | 原版位置 | 状态/附件/依赖审核 | 结果与边界 |
|---|---|---|---|---|
| L0 建立 target | `loading_graph.rs::build_loading` | compile_loading `1401647b0` | 非零 display extent；format0x2b，单采样；真实图 owner | 源码已核；所有资源生命周期交独立 owner 审核，不重复声称全证 |
| L1 Vermeer | 同上 → `vermeer_render_from_target` → `EffectsHost` | `14016532a→1408a8d70` | 原 generate → raster strokes → combine 链；effect draw 的 `raster_state_edit` 在纹理/uniform绑定后、draw前清 depth-test bit 并设置 blend2；不能用清黑代替背景 | 外层顺序 O；effect 所有内部算法本次 U |
| L2 MEI | 同上 → `mei_pass_with_shadows` | `140165347..34a`、callee `1409c6aff..6bc8` | **false**：tag0直接 LOAD L1；depth none，overlay state；TextPrim/ShapeStream/atlas 使用当前App图；没有 temp 和 shadow combine | F→源码修复，中央 GPU fixture 通过 |
| L3 Logo UI | 同上 → `loading_screen_pass` | `14016536e→140837ff0` | list18；tag0 LOAD；bind_common后、Logo回调前 blend2。Logo回调自身不负责该设置 | 本轮已知缺失已修；中央 GPU alpha255 通过 |
| L4 export | 同上 → graph.export | `1401653ba→140bd6380` | final loading target export0x22，由真实源 sample_count=1选择单采样 resolve | 源码/顺序 O；WSI最终完成另列 |

Loading 与 gameplay 的 MEI 不能共用一个固定 shadows 值。gameplay 原 `140163422` 调 `EscMenuState::cast_ui_shadows`，`140163446` 把结果放 r9d，`140163449` 调 MEI；Loading 原显式传0。

## 每次 draw 的状态矩阵

以下 O 只覆盖所列状态写入和位置，不把所有 draw-list 回调的状态覆盖自动算作已证。原默认 `0x1ffc00d0`：triangle-list/no-cull/fill，depth GREATER/test+write，stencil off，blend off。当前 host显式返回该默认；每个 packet 从默认重建，不继承上一 draw 的 pipeline。当前 pipeline 由 packet 完整 state 编码选取，不能仅因为默认“看起来能画”就删掉原显式设置。

| 消费者 | draw前状态与时序 | 深度/模板/附件 | 证据/审核结果 |
|---|---|---|---|
| UI list19 / Logo list18 | bind_common → `b3=(b3&0x9f)|0x40` → callback | 无深度；color tag0 LOAD。不要额外给 list pass统一清 depth bit，原这里只改 blend | `logo-blend`原`140837ee2..eed`/`140838292..29d`；O，已修 |
| UI2D/WS | 每项绑图/常量后 `b0&=0x7f` + blend2 → draw6；屏幕项稳定sort，世界项保留遍历顺序 | 无深度；tag0 LOAD；不改 stencil/cull/topology 的其他位 | 原`1409c4470`，本次 ui2d.asm；源码对应，完整排序查询业务未逐项验收 |
| MEI | 真实 buffers/atlas绑定后 `state=(state&0x9fffff7f)|0x40000000` → draw6*n | false:tag0 target LOAD；true:tag1 temp CLEAR0，再shadow combine tag0 LOAD target | 原`1409c6700`；分支原证及新Loading修复；gameplay shadow分支实际 U |
| Logo overlay | 绑定Logo纹理与target extent后 depth-test off + blend2 → draw6 | depth none；tag0 LOAD | 原`1408a5f50`；O |
| Cursor | 设置尺寸/纹理/SSBO后 depth-test off + blend2 → draw6 | depth none；tag0 LOAD；原custom_cursor flag门保留 | 原`1409d1710` retained证据及源码；本次没有重新解码该叶，限定U |
| Egui | 每mesh从默认packet开始；纹理+顶点storage绑定后 depth-test off + blend2 → indexed draw | 同4×target，LOAD；输入原始mesh顺序 | 原`1400ddc80`本次egui.asm；状态O，实际egui UI分支 U；clip/scissor是否原版另处理尚未完整证明，不擅补 |
| W44 Refraction | bind_common+source绑定后仅 `b0&=0xbf` → callback；**不设blend2** | color tag1 CLEAR0，depth只读LOAD | 原`1409bf320`refraction.asm；源码一致。采样仅mip0为已有host适配，见源码原shader fetch证据 |
| W45 Water/Ice | bind_common+per-item绑定后blend2 → callback；**保留depth-write** | color目标与各辅助MRT原tags；depth可写LOAD；before/after compute分别保留 | 原`1409bf800`/`1409c08d0`本次raw；不要误套pre/post的depth-write-off。颜色附件完整覆盖/间接计数实际 U |
| W46 Pre-TAA | extra深度/half/history绑定后 `b0&=0xbf` + blend2 → callback | color LOAD；depth只读且采样 | 原`1409c3a80`pre-taa.asm；源码状态一致 |
| W53 Post-TAA | 空list17原直接返回；非空bind_common后`state&0x9fffffbf|0x40000000` → callback | no depth，color LOAD | 原`1409c40b0`post-taa.asm；源码一致 |
| W25/W30/W34 Gbuffer/Forward | stencil_ref unwrap→bind_common→`b1=(b1&0x8e)|0x21`；Velocity在packet构造前 `(s&0xffffff8f)|0x20` | 初次Gbuffer DONT_CARE + depth CLEAR0；emissive/velocity首次CLEAR0；后续LOAD；Velocity depth只读 | 源码main_pass + d_main_support已核。原叶本次未逐个重解码，状态全等仍U，不用header注释封证 |
| W43 Deferred | 每材料entry state→绑定→final state→draw3；特定条目blend1(ADD/ONE/ONE)，非premult | color LOAD；depth只读，stencil材料筛选 | 原`1409d2507..25f2`/`1409d2e4c..5b`为retained证据，源码字段已核，独立原机器码本轮U |
| T3 Final-post / resolve | 新packet默认blend off，closure绑资源→draw3 | Final-post tag2，4×；resolve无深度，目标extent、源实际sample_count | 原特化`1409ce780`/`1409d00a0`为retained证据；源码已核，非“给所有fullscreen加premult” |

blend2 的 Vulkan展开（保留现有 rhapsody）：RGB/Alpha均 ADD、src=ONE、dst=ONE_MINUS_SRC_ALPHA；RGBA write mask齐全。blend1的RGB/Alpha均ADD、ONE/ONE。`RasterPipelineOpts::apply_to` 把当前packet状态展开至每个颜色附件，包括MRT；没有在本轮重写此底层。该解码来自现有源码审阅，本次没有对原全部 Vulkan创建入口逐一反汇编，因此不泛称底层全验证。

## compile_main 尾部顺序/门控矩阵

当前生产按 `world_postprocess.rs::build_world_tail`，T21及最终capture/export在`production_renderer.rs::build_full_world_stage`。调用点均由本次 `compile-main.asm`独立读到；其中 T10 是门，不是GPU pass。

| 步骤 | 原调用点→入口 | 当前内容/门 | 附件与状态结论 | 判定 |
|---|---|---|---|---|
| T1 grading fold | `140162ead→140b00b20` | screenshot purpose选择formation/LUT | CPU，无raster状态 | 顺序O；完整grading算法U |
| T2 blur pyramid | `140162f4e→1409ed860` | W54输出真实taa，原无条件 | HDR0x7a mip pyramid | 外层O；叶dispatch/所有mip屏障U |
| T3 final_post | `14016302b→14090d000` | taa+blur+LUT+ICC | 新format0x2b、4×MSAA；tag2 DONT_CARE；fullscreen完整写；默认blend off | 源码及外层O；GPU所有LUT组合U |
| T4 screenshot0/1/2 | `140163091/0d3→14031d140` | 原purpose门，capture在overlay前 | 使用同msaa/实际sample4 | 外层顺序O；各capture实际U |
| T5 MenuBG | `14016310d→1408c9a30` | menu gate+实际captured owner | 原`1408c9d3b`tag2；depth-none；depth-test off、blend off；原fullscreen覆盖 | 原状态/颜色tag O，**不得改成统一LOAD** |
| T6 Painterly | `14016314e→1408a6410` | 原菜单门/temporal/phase | target同MSAA；效果host逐draw显式overlay状态 | 调用/host接线O；完整effect叶U |
| T7 Vermeer | `140163277→1408a82f0` | 原phase/restart/hold/reveal | 返回active参与T10，不能用ready标志代替 | 外层O；内部多帧实际U |
| T8 Logo overlay | `1401632d3→1408a5f50` | purpose/demo条件 | tag0 LOAD+premult；Logo纹理/尺寸绑定 | 状态O；所有purpose实跑U |
| T9 screenshot3 | `1401632fa→14031d140` | purpose3或demo-purpose4 | 在Logo之后、UI块之前 | 外层O |
| T10 UI gate | `140163315 and sil,r14b` / `140163318 jne 1401634af` | `!vermeer_active || photomode` | 只gate T11–T19；不gate T20/T21 | 外层O；不能为显示菜单强制打开 |
| T11 session previews | `14016333d→1408f6c00` | 原EscMenuState/saved-list/pool | 原tag0 LOAD，depth-off+blend2 | **F已修**；实际分支GPU尚U |
| T12 LUT gallery | `140163361→1408b0560` | Photomode(true)+真实preview+非demo | 原tag1精确CLEAR0.18 RGB/alpha1，depth-off+blend2 | **F已修**；实际分支GPU尚U |
| T13 feedback | `140163385→1408a5a70` | 用户明确不推进 | 不新接，不假称已恢复 | **X** |
| T14 Wireframe | `1401633b9→1409ccb20` | `flag_0x1ac0` | LOAD，wire polygon+depth-test off，linewidth1；不能套UI blend | 源码已核；原叶独审U |
| T15 UI list19 | `1401633e4→140837c40` | 同T14门 | LOAD+每callback前blend2 | O，中央已修 |
| T16 UI2D | `140163408→1409c4470` | T10内无额外门 | LOAD、稳定UI序、显式overlay状态 | 状态O；UI producer完备性U |
| T17 MEI | `140163449→1409c6700` | 真正cast_ui_shadows结果 | 直接LOAD或temp CLEAR0→shadow LOAD；同4×最终target | 分支/状态O；gameplay实际形状、文字U |
| T18 Cursor | `14016346d→1409d1710` | 配置custom_cursor门 | LOAD、overlay | 源码已核，原叶/实际U |
| T19 screenshot4 | `140163494→14031d140` | T10内 | UI+cursor之后capture | 外层O |
| T20 Loading list18 | `1401634d5→140837ff0` | T10外，原总调用 | 同4×target LOAD+blend2 | O |
| T21 Egui | `14016350a→1400ddc80` | owner Option Some，warmup无owner | 在T20后，同4×target LOAD+blend2 | 原外层+状态O；实际UI分支U |
| screenshot6/export | `140163531/35be` | T21之后 | 真实最终4×target export0x22，不能返回taa代替 | 源码/外层O |
| resolve | present caller `140166b07→14090d850` | source实际samples决定专用shader | loading1→single shader；gameplay4→MSAA shader；target用acquired extent | 源码已核；物理显示/WSI严格完成U |

## 图依赖/状态边界

可直接复核的当前源码位置：

- [Loading direct MEI](D:/game/tiny-glade/crates/tiny-glade/src/loading_graph.rs:363)、[T11 LOAD](D:/game/tiny-glade/crates/tiny-glade/src/session_preview_host.rs:520)、[T12精确clear](D:/game/tiny-glade/crates/tiny-glade/src/world_postprocess.rs:960)。
- [世界编排](D:/game/tiny-glade/crates/tiny-glade/src/world_graph.rs:91)、[光照](D:/game/tiny-glade/crates/tiny-glade/src/world_lighting.rs:107)、[表面/透明](D:/game/tiny-glade/crates/tiny-glade/src/world_surface.rs:47)、[AA/DoF](D:/game/tiny-glade/crates/tiny-glade/src/world_postprocess.rs:95)、[tail](D:/game/tiny-glade/crates/tiny-glade/src/world_postprocess.rs:1061)。
- [每list item状态位置](D:/game/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/d_main_support.rs:258)、[packet默认](D:/game/tiny-glade/crates/tiny-glade/src/raster_host.rs:838)、[LOAD依赖](D:/game/tiny-glade/crates/tiny-glade/src/raster_host.rs:1319)、[实际loadOp](D:/game/tiny-glade/crates/tiny-glade/src/raster_host.rs:1530)、[Vulkan RGB/alpha展开](D:/game/tiny-glade/crates/rhapsody/src/raw/shader.rs:1275)、[presentation导入](D:/game/tiny-glade/crates/tiny-glade/src/production_resolve.rs:27)。

1. `RasterPass::new` 通过颜色tag声明访问：tag0=0x36、tag1/tag2=0x2e。当前`raster_host.rs`按真实附件view产生范围，LOAD且有此前写时用`add_preserving_resource_access_range`；实际录制将tag映射到Vulkan LOAD/CLEAR/DONT_CARE。因此T11的tag2既是画面内容错误，也是依赖语义错误；只验证pass名称/先后不够。
2. current host COLOR CLEAR payload来自`RasterPassColorAttachment.view`原bits，非子资源视图。T12直接用既有`ColorAttachment::View`保全四字值，不能仅改`ColorSpec::tag1`（其默认清零）。所有颜色store采用STORE。depth只读/写、stencil aspect保留的转换已源码核对；整个rhapsody barrier算法未在本任务重新证明。
3. pass内新packet+完整pipeline key防止上一个draw blend串入，但回调仍可显式修改本packet。`run_list_pass`的state写必须在bind_common/extra后、callback前。此次UI缺失正是这一层，不能只在GPU默认state修补。
4. texture/SSBO/indirect/index/vertex/frameconstants按实际range加入依赖；未见UI层改用裸整数作为跨App owner。原owner交另一独立agent审，不把本报告的源码阅读当成并发/释放完整证明。
5. final_post建立4×原生图像，tail各消费者的AttachmentTexture传sample_count4，resolve从exported native source检查实际samples。普通frame不能复用warmup32作为显示尺寸；本次只核当前display/normal frame入口，不以“纹理非黑”证明UI完整。
6. 真正pipeline编译完成、Vulkan提交、GPU完成、present queued、present完成是不同事实。此机器strict WSI维护扩展缺失的问题不能通过渲染顺序报告抹掉。

## W1–W54 外层逐环清单

下表各行的 **顺序O / 叶U** 意味着已对原`render_world.asm`调用点及当前host连续调用定位；没有假称该叶内部所有资源、shader、条件、算法均独立复核。纯compute行无blend/depth附件，但仍须继续逐叶核实dispatch、资源range、barrier和host producer。独立GPU图像仅证明某次路径执行，不封闭冬季、空场景、debug、DoF关闭、历史重置等所有分支。

| W | 环节 | 原调用点 | 当前生产落点 | 状态/资源主要边界 |
|---|---|---|---|---|
| 1 | 六个world raster upload | `1404ec2a5..2eb` | world_graph→scene_compute | 固定六类顺序；staging与写纹理range叶U |
| 2 | water CPU raster | `1404ec2f6` | world_graph→scene_compute | 顺序O / 叶U |
| 3 | terrain stroke | `1404ec301` | world_graph→world_terrain | 顺序O / 叶U |
| 4 | rocky displacement | `1404ec30c` | world_graph→world_terrain | 顺序O / 叶U |
| 5 | terrain readback | `1404ec317` | world_graph→world_terrain | CPU回读同步叶U |
| 6 | mouse stroke index | `1404ec322` | world_graph→world_terrain | 顺序O / 回读叶U |
| 7 | foliage exclusion | `1404ec32d` | world_graph→world_terrain | raster附件/叠加细项U |
| 8 | active terrain highlight | `1404ec338` | world_graph→world_terrain | 顺序O / 叶U |
| 9 | object grass push | `1404ec343` | world_graph→world_terrain | raster附件/state叶U |
| 10 | path masks | `1404ec34e` | world_graph→world_terrain | 顺序O / 叶U |
| 11 | building rasterize | `1404ec359` | world_graph→world_terrain | 顺序O / 叶U |
| 12 | garden CPU raster | `1404ec364` | world_graph→scene_compute | 顺序O / 叶U |
| 13 | water vegetation spawn | `1404ec36f` | world_graph→world_vegetation | 顺序O / 叶U |
| 14 | building vegetation spawn | `1404ec37a` | world_graph→world_vegetation | 顺序O / 叶U |
| 15 | bush positions readback | `1404ec385` | world_graph→world_vegetation | 回读range/完成叶U |
| 16 | garden vegetation spawn | `1404ec390` | world_graph→world_vegetation | 顺序O / 叶U |
| 17 | path pebbles | `1404ec39b` | world_graph→world_vegetation | 顺序O / 叶U |
| 18 | leaf particles | `1404ec3a6` | world_graph→world_geometry | 顺序O / 叶U |
| 19 | roof tiles gravity | `1404ec3b1` | world_graph→world_geometry | 顺序O / 叶U |
| 20 | stairs | `1404ec3bc` | world_graph→world_geometry | 顺序O / 叶U |
| 21 | wall draw lists1 | `1404ec3c7` | world_graph→world_walls | indirect producer/consumer叶U |
| 22 | nani draw lists1 | `1404ec3d5` | world_graph→scene_compute | 第一遍无HZB，叶U |
| 23 | shadow | `1404ec3ea` | world_graph→SceneRasterHost | cascade flags2..4、state低位改写源码存在；每cascade深度层完整原证U |
| 24 | terrain lighting | `1404ec3f5` | world_graph→world_geometry | 顺序O / 叶U |
| 25 | main gbuffer/forward | `1404ec585` | world_graph→SceneRasterHost | 六子pass/首次clear/stencil，见状态表；全部callback U |
| 26 | HZB | `1404ec5ac` | world_graph→SceneComputeHost | depth真实输出→HZB；每mip屏障U |
| 27 | wall draw lists2 | `1404ec5ca` | world_graph→world_walls | freeze门；HZB消费 |
| 28 | nani draw lists2 | `1404ec5d8` | world_graph→scene_compute | 同freeze门；HZB消费 |
| 29 | grass | `1404ec5e7` | world_graph→world_geometry | 使用同HZB，内部freeze分支U |
| 30 | main oc2 | `1404ec604` | world_graph→SceneRasterHost | 五子pass保留target/depth；见状态表 |
| 31 | visibility queries | `1404ec619` | world_graph→world_geometry | depth读/结果回传U |
| 32 | DoF autofocus | `1404ec627` | world_graph→world_geometry | depth读/CPU回传U |
| 33 | mouse depth | `1404ec635` | world_graph→world_geometry | depth读/CPU回传U |
| 34 | late forward/deferred | `1404ec64b` | world_graph→SceneRasterHost | 两list/LOAD/stencil；叶U |
| 35 | half-res depth/normals | `1404ec660` | world_lighting | 先于color history pair建立；叶U |
| 36 | reprojection | `1404ec6d9` | world_lighting | 当前depth/velocity+真实prev_depth；叶U |
| 37 | SSGI | `1404ec708` | world_lighting | half/depth/reprojection；叶U |
| 38 | ray trace GI | `1404ec986` | world_lighting | history_valid真实旗标，不倒置为fresh；叶U |
| 39 | GI screen radiance | `1404ec9ac` | world_lighting | 消费原RT三输出与SSGI；叶U |
| 40 | bin lights | `1404ec9c1` | world_lighting | grid/index输出；叶U |
| 41 | ReSTIR DI | `1404ec9e5` | world_lighting | 原bin/reprojection；叶U |
| 42 | screen shadows | `1404eca03` | world_lighting | 原gbuffer/depth/reprojection；叶U |
| 43 | deferred lighting | `1404eca67` | world_lighting→SceneRasterHost | stencil筛选/特定additive条目；见状态表 |
| 44 | refraction copy | `1404ecb3a` | world_surface | skip门内CLEAR0 refraction + depth只读；见状态表 |
| 45 | water/ice | `1404ecb96 call r10` | world_surface | `1404ecb3f..b54`原冬季分支；水/冰保留不同完整prepare/raster/finish；叶 U |
| 46 | pre-TAA transparency | `1404ecbcd` | world_surface | 在refraction门外；depth-write off+premult；见状态表 |
| 47 | shadow mask mean | `1404ecbd8` | world_postprocess | 顺序O / 叶U |
| 48 | volumetrics | `1404ecc0e` | world_postprocess | 同color/depth；叶U |
| 49 | DoF prepare | `1404ecc26` | world_postprocess | 原无条件prepare；叶U |
| 50 | color mip generation | `1404ecce2` | world_postprocess | DoF开启门内，先于calcDoF |
| 51 | calc DoF | `1404ecd09` | world_postprocess | photomode使用单独配置，禁用则原color进入AA |
| 52 | TAA/TGSR | `1404ece34 / 1404ecfef` | world_postprocess | 原二选一；TGSR显示extent，TAA源extent；历史和输出分离；shader叶U |
| 53 | post-TAA transparency | `1404ed00b` | world_postprocess→SceneRasterHost | AA之后、shader debug之前；状态原证O |
| 54 | shader refactoring | `1404ed019` | world_postprocess | 原强制资源读；disabled早退，active原replacement出口；分支实际U |

## 尚不能宣布完成的项与后续验证次序

1. 中央完成此次 T11 LOAD/T12精确CLEAR 的编译与有界分支验证；对预览透明角/列表为空/非空、LUT gallery各cell之外背景做像素检查。单纯复制常量做unit assert不能证明真实host附件使用。
2. 真实960×540小窗口完成Loading→Logo结束→Vermeer reveal结束→gameplay UI实际图像，记录UI producer数量/门和帧图，不用强制UI门或虚构welcome producer；窗口resize后再次核actual final target/display/source dimensions及文字。
3. 其后按同一链分段推进 W23/25/30/34/43 的原状态及每list callback覆盖，再 W44–46/53的不同世界/冬季路径；计算叶需独立资源和dispatch合同，不把“代码调用存在”当完成。
4. 本报告不得汇总为一个100%或宣传全部状态已证；它是可继续填写的明确边界和三个具体修复。中央告知 Welcome producer 已恢复原三态，6 个原模板真实 consumer 的 CPU 检查通过；最终窗口像素和完整 MenuFlow 仍待验。异步主线程等待和 WSI 限制由中央其余专项并行处理。

## 逆向效率与避免重犯

- 从最外层真实caller开始，记下所有参数寄存器、gate、颜色/depth附件指针，追到draw前最后一条状态改写，再追到真实shader/pipeline。不要由函数名或默认值猜测。
- 附件tag必须追 **传入 `new_impl` 的r9来源**；邻近depth tag、纹理枚举tag、临时Option的2都不能代替颜色tag。此次T11/T12的独立字节复核实际发现了这种误还原。
- 每张透明层同时核RGB和alpha blend、目标LOAD/CLEAR、图read/write依赖三件事；只查blend或pass名都会漏掉“draw存在但抹底图”。
- 原分支常量先固定在证据，再设有负例的回归：Loading shadow=false、原背景alpha保持；T11 LOAD保持角落、T12 CLEAR精确背景。禁止让错误现实现自动生成黄金值。
- 更新共享底层前先查看当前设计；此轮全部恢复使用现有rhapsody/Pass API，无旧库回填、无平行图实现、无删复杂shader来遮掩状态错误。
