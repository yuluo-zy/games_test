# RESTORE — systems/timberframe（restorer-state 重建，2026-10-17）

> 角色：restorer-state-timberframe · 硬标准（真实逻辑非形状）：每业务函数 fresh 反编译 → 重建完整控制流 → Rust body 逐语句判等；重建不了标 `未实现`（分歧点），不以近似通过。

## 逐函数判定

| 函数（VA） | 判定 | 说明 |
|---|---|---|
| `construct_timberframe_rectangle`（0x141a41ba0） | **重建（结构/常量/相位/计数判等）＋ 未实现（梁实例写路径/朝向装配）** | 见 §1 |
| `construct_timberframe_circle`（0x1421b67a0） | **重建（同上）＋ 未实现（写路径/裙层/屋顶环）** | 见 §2 |
| `timberframe_beam_count_change_audio`（0x1421b6610） | **判等（主体；帧读面按机器码 +0x2c 裸读，布局 GAP 登记）** | 见 §3 |
| `add_timberframe`（0x140630a20） | **glue-legitimate**（bevy App 注册壳） | 见 §4 |
| `timberframe_on_glade_unload`（行 50 内联） | **glue**（卸载清理，占位 clear；宿主 OnExit 接线 GAP） | §4 |

## FIX 落实（reviewer-state 复核 4 条，2026-10-17）

1. **[FIX-1] 每事件计数写回**：`lVar19/lVar15=0` 于每事件起点、`BeamCount+8=…` 于每事件末尾
   （最后事件胜，batch>1 语义）。Rust 改为每事件内 `let mut event_total=0; …; beams.count=event_total;`
   ——已修（rect/circle）。
2. **[FIX-2] bit0 门控方向**：rect 处理 **bit0=1**、circle 处理 **bit0=0**（方向相反，O）。bevy `read()`
   无法复现两方向过滤 → 已在源码标注 O 方向 + `get_shape()` 形状过滤为「bit0=形状类型标签」的
   未判等代理；契约暴露首字节转集成 owner（GAP 登记）。
3. **[FIX-3] GameClock 布局**：`frame_count` 字段实际 @+0x28（reviewer 核验；章节证据 @0x2c 待校正），
   二进制读 GameTime+0x2c。audio 改为**按机器码面裸读 +0x2c**（`game_time_int_at_0x2c`），不依赖字段名；
   原结构 +0x2c 字段语义 U，country-core 布局校正转集成（GAP 登记）。
4. **[FIX-4] lib.rs 多余事件注册**：`Events<PlayTimberframeSound>` 原 add_timberframe 不注册
   （country-core `add_audio_events` 已注册）→ 已删（仅注册 `ConstructTimberframeCmd`）。

## fresh 反编译证据索引

- 本仓证据：`evidence/restorer-state-timberframe/`
  - `2026-10-17-result-decomp.json`（get_functions：signature,decompiled_code,locals ×4）
  - `decomp-construct_timberframe_rectangle.txt`（37,872 B，全读）
  - `decomp-construct_timberframe_circle.txt`（23,169 B，全读）
  - `decomp-timberframe_beam_count_change_audio.txt`（3,050 B，全读）
  - `decomp-add_timberframe.txt`（4,674 B，全读）
  - `2026-10-17-result-constants.json`（read_memory 常量核验）
- 常量（read_memory，O）：`0x1429258d0 = {1.5, 0.63, …}`——circle 梁数除数 **1.5**（`fVar22/1.5`），
  RE-ANALYSIS-2 读 0.63 是错 lane（0.63 @+4 为另一常量），已纠正；`0x142ebc300 = {1.51, "crates/…"}`
  ——rect 等距除数 **1.51**；`0x142a8c6f0 = {0.2, 0.2}` 截面；`0x142925870 = {0.5, 0.4923077(≈0.4922), …}`。
- 既有分析（指引，已 fresh 核）：`upper-modules-plan/systems/timberframe/{TASKS,ANALYSIS}.md`、
  `evidence/country-core-deep/systems-recovery/groupA/timberframe/{RE-ANALYSIS,RE-ANALYSIS-2,REVIEW,RESTORE}.md`。

## §1 矩形（0x141a41ba0）

**签名（O，寄存器面）**：`(EventReader<ConstructTimberframeCmd>, ResMut<TimberframeInstanceBuffer>,
Res<AssetSsboLibrary>, ResMut<TimberframeBeamCount>, Res<TimberframeConstructionParamsCache>)`
（RDX tick、R9 tick、栈+0x28 = Cache 读 +8/+0xc）。与机器码一致（reviewer FIX 已落）。

**逐事件相位（O）**：① 槽空 → SSBO 查找 + 创建持久 allocation 存入槽；二次查找 + allocation +0x18
互斥 → `writer().clear()`。② 事件 0x28 步长，`do{…}while((*e & 1)==0)`——**处理 bit0 置位、跳过
bit0 清零条目**（与 circle 相反，O）。③ **垂直梁**：`as_curve3` 每顶点 `orientation_mat3` → push（截面 0.2），
`lVar19 += 顶点数`（**每事件起点 lVar19=0**）。④ **等距梁**：相邻顶点对 `d`、`n=⌊d/1.51⌋`，
`for i in 1..n: t=i/n` lerp → push（截面 0x3e3851eb=0.18），`lVar19 += n-1`。⑤ **裙层**：
`split_height_into_skirts` + 栈上 {0.0, cache+0xc} 两固定环；每环每边中点一根，`lVar19 += 2×边数`。
⑥ **屋脊**：`if(event[0x1c]==0)` → roof_height / ridge_length_01 / RoofRidgeDims::calc →
`as_curve2`×2 → `lVar19 += min(lenA,lenB)`；`perimeter>0.5` → +1。⑦ **每事件末尾**
`*(BeamCount+8)=lVar19`（最后事件胜）。

**已重建（判等）**：相位结构与顺序、`n=⌊d/1.51⌋`+`t=i/n` 等距算法、截面 0.2/0.18 常量、垂直/等距/裙层
两固定环计数、**每事件计数重置 + 每事件写回**（FIX-1）、`BeamCount.count`（i64 @+8）写回面、
`+0x1c` 屋脊门控的存在与位置、bit0 门控方向（rect 处理 bit0=1）。

**未实现（分歧点）**：
- 梁实例写路径：`WoodenPlankSsbo` 非 `SparseSsbo`/`SsboType`、`render::RenderInstance` 记录 country-core
  未提供、`utils::rotation_from_normal`（0x140c90a40）/`orientation_mat3`（0x140c99570）未提供
  → 逐梁 `SparseInstanceBufferWriter::push` 与朝向/实例装配未落地。
- `split_height_into_skirts`（0x140b34f80，country_core::resources::walls）未提供 → 裙层环数 N 未知，
  `count` 只计两固定环（写回值为真实计数**下界**，如实登记）。
- 事件首字节 bit0 门控与 `event[0x1c]` 门控字节不在 `ConstructTimberframeCmd` 契约内（shape =
  ShapeParameters 28B + `_gap`；真实 payload 自 event+4 起，偏移 I/GAP）→ 门控未执行，按全部事件处理。
- `as_curve3(height)` 的 height 实参（寄存器面）I；屋脊 `ridge_dir` 字节 U。

## §2 圆形（0x1421b67a0）

**签名（O）**：`(EventReader, Res<TimberframeInstanceBuffer>, Res<AssetSsboLibrary>,
Res<TimberframeConstructionParamsCache>, ResMut<TimberframeBeamCount>)`（末两参与矩形相反；RDX 只读
无 tick、栈+0x28 = BeamCount tick）。与机器码一致。

**逐事件相位（O）**：① SSBO 查找 WoodenPlankSsbo（类型名字串 0x142ff8cf0）+ 名字断言；资源槽
首 qword==0 → `unwrap_failed`（**panic，非 return**——纠正旧 `return` 近似）；`[1]` allocation +0x18
互斥。② 事件迭代，`do{…}while((*e & 1)!=0)`——**处理首字节 bit0 清零、跳过 bit0 置位**条目
（方向与矩形相反，O）。③ **主环**：`count=max(6,⌊perimeter/1.5⌋)`（1.5 常量已核）；`lVar15=0`
（**每事件起点**）；`for i in 0..count: u=i/(count-1)`（越界/NaN → utils panic）、`get_pos_at_u`、
高度 cache+0xc、截面 0.2 → push。④ **裙层**：`split_height_into_skirts` + 栈上 {0.0, cache+0xc}
两固定环；每环 `u=i/(count-1)` + `rotation_from_normal` → 高度 h+cache+8 → push。⑤ **屋顶环**：
`if(event[0x1c]==0)` → `roof_height_ws_from_wall_shape` → push。⑥ **每事件末尾**
`*(BeamCount+8)=lVar15`（最后事件胜）。

**已重建（判等）**：`count=max(6,⌊perimeter/1.5⌋)`（纠正旧 `(p/0.2).ceil()` 与 RE-ANALYSIS-2 的 0.63）、
`u=i/(count-1)` 采样（纠正旧 `i/count`）、相位结构与顺序、裙层两固定环计数、**每事件计数重置 + 每事件
写回**（FIX-1）、`BeamCount.count` 写回、空槽 panic 面（纠正旧 return）、`param_2 = Res` 只读面、
bit0 门控方向（circle 处理 bit0=0）。

**未实现（分歧点）**：同 §1——WoodenPlankSsbo/RenderInstance/rotation_from_normal/split_height_into_skirts
缺失 → 逐梁写路径、裙层环数、屋顶环（门控字节 U）未落地；事件 bit0/+0x1c 门控字节不在契约内。

## §3 音频（0x1421b6610）——判等（主体）

**签名（O）**：`(EventReader, ResMut<TimberframeBeamCount>, Res<GameTime>, EventWriter<PlayTimberframeSound>)`
（RDX tick = ResMut、R8 = GameTime 只读、R9 = EventWriter）。

**控制流（O，逐语句）**：无新事件 → 返回；`frame = *(int*)(GameTime+0x2c)`（机器码读面，O；
reviewer 核验 country-core `GameClock` 结构体 `frame_count` 字段实际 @+0x28、章节证据 @0x2c 待校正——
本实现按机器码面**裸读 +0x2c**，不依赖字段名，见 FIX-3）；`if ((frame - BeamCount.last_frame == 1) &&
(BeamCount.last_count != BeamCount.count))` → `ShapeParameters::center()` + `x0y()` + 构造
`PlayTimberframeSound{kind=6, pos=x0y(center)}` → push；回写 `last_frame=frame`、`last_count=count`、tick。

**已判等**：事件门控、frame 读面（+0x2c，按机器码裸读）、`(Δframe==1) && (last_count≠count)` 触发、
kind=6、x0y 位置、`last_frame`/`last_count` 回写位置与序。

**分歧点（GAP）**：`get_building_size_01`（0x1417d3f10，audio/building_audio 域）未在 country-core 提供
→ 事件 size 字段（+0x10）未写出；core `PlayTimberframeSound` 缺 size/id 字段（真实 0x20B 事件）；触发用
事件索引取首个未读（I）；`GameClock` +0x2c 字段语义 U（原结构 frame_count @0x28，布局校正转 country-core）。

**BeamCount 布局修正（O，fresh）**：`{last_count i64 @+0x00, count i64 @+0x08, last_frame i32 @+0x10}`，
非旧 `{current i32, previous i32, non_empty bool}` 近似。

## §4 插件（0x140630a20）——glue-legitimate

`App::insert_resource<TimberframeInstanceBuffer>` → `insert_resource<TimberframeBeamCount>` →
条件 `register_event<ConstructTimberframeCmd>`（= add_event）→ `chain_ignore_deferred().in_set(…)` →
`add_systems(Update, (rect, circle, audio))` → `Schedules::add_systems(OnExit<AppLoadingState>,
unload(ResMut<TimberframeInstanceBuffer>))`（Schedules 缺失 → bevy 标准 panic 串）。纯注册，无业务 →
**glue-legitimate，不重建**。

**GAP**：`in_set` 的 `dyn SystemSet` 具体标签 U（本处以 chain 表达，I）；`OnExit<AppLoadingState>` 依赖
country-core 未提供的 `AppLoadingState` 与 bevy_state → 暴露 `timberframe_on_glade_unload` 供宿主接线。
**FIX-4**：`Events<PlayTimberframeSound>` 多余注册已删（原 add_timberframe 不注册；country-core
`add_audio_events` 已注册）。`init_resource`（仅缺则插）vs 原 `insert_resource`（无条件覆盖）为插件
构建语义近似，登记轻微差。

## 交付面

- 修改：`crates/systems/timberframe/src/{lib.rs, timberframe_beam_count_change_audio.rs,
  timberframe_circle.rs, timberframe_rectangle.rs}`（函数体按真实控制流重建）。
- 编译：`cargo check -p timberframe` **通过**（workspace 根下；验证窗口内 Finished dev，仅本 crate 0 warning）。
  注：country-core/water 正被并行 agent 编辑，最终复跑偶见其瞬态错误（ivy/ui_clutter/decorator_storage），
  非本 crate 引入；另见下节。
- 协作登记：验证中观测到 country-core `ivy_send_to_gpu_leaf`/`ivy_pruner` 曾短暂编译错误（另一 agent
  修复中）；本任务未修改 country-core。

## 跨域交接（GAP 汇总，转集成 owner / 对应 restorer）

1. **country-core**：提供 `WoodenPlankSsbo` 的 `SparseSsbo`/`SsboType` 与 `render::RenderInstance` 记录、
   `resources::walls::split_height_into_skirts`、`systems::audio::building_audio::get_building_size_01`、
   `PlayTimberframeSound` 补 size/id 字段 → 解锁两构造函数写路径与音频 size。
   `GameClock` 布局校正（FIX-3：`frame_count` 字段 @0x28、二进制读 +0x2c，章节证据待改）。
2. **utils**：提供 `rotation_from_normal`（0x140c90a40）、`Rectangle2d::orientation_mat3`（0x140c99570）。
3. **事件契约**：`ConstructTimberframeCmd` 需暴露事件首字节 bit0（rect 处理 bit0=1 / circle 处理 bit0=0，
   FIX-2）与 `+0x1c` 字节（真实 payload 自 event+4 起，当前 shape/gap 布局 I）→ 解锁两门控与
   rect/circle 跳条目语义。
