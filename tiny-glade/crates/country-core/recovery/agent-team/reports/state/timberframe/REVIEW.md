# REVIEW — systems/timberframe（reviewer-state-timberframe 复核，2026-10-17）

> 角色：reviewer-state-timberframe · 硬标准（真实逻辑非形状）：逐函数 **fresh 反编译** → 独立复述完整控制流
> → 与 Rust body 逐语句对照 → verdict。本复核 fresh 反编译的代码区与 restorer 证据
> `evidence/restorer-state-timberframe/decomp-*.txt` 逐字节一致（diff 验证），下述复述与判定均基于
> 本 reviewer 自己的 fresh decomp（`evidence/reviewer-state-timberframe/review-decomp-*.txt`）。

## 逐函数判定

| 函数（VA） | 判定 | 要点 |
|---|---|---|
| `construct_timberframe_rectangle`（0x141a41ba0） | **重建（结构/常量/相位/计数判等）＋ 未实现（梁写路径）＋ 1 新分歧** | §1 |
| `construct_timberframe_circle`（0x1421b67a0） | **重建（同上）＋ 未实现（梁写路径）＋ 1 新分歧** | §2 |
| `timberframe_beam_count_change_audio`（0x1421b6610） | **判等（主体）** | §3 |
| `add_timberframe`（0x140630a20） | **glue-legitimate**（bevy App 注册壳；1 处多余注册 + init/insert 语义差） | §4 |
| `timberframe_on_glade_unload`（行 50 内联） | **glue**（body U，clear 为 I 占位） | §4 |

## 方法

- 工具：`ghidra --worker shared`，`get_functions(signature,decompiled_code,locals)` × 4 函数 + `read_memory` × 6 常量点。
- 常量（O，本复核 read_memory）：`0x1429258d0 = {1.5, 0.63, 0.3, 0}` 首 lane **1.5**（`DIVSS dword[0x1429258d0]`）；
  `0x142ebc300 = 1.51`（紧邻模块路径串）；`0x142a8c6f0 = {0.2, 0.2}`；`0x142925870 = {0.5, 0.4923077, …}`。
  `PTR_s_Timberframe_constructor_ran_out_o_142ff8f40` → @0x142ff8f08 54B 串（circle 侧）；rect 侧 0x142ebc3c0 表同串。**restorer 全部常量主张独立核验通过**。
- 计数下限 6：decomp `if (local_190 < 0x7) local_190 = 0x6;`（O）→ 等价 `max(6,⌊perimeter/1.5⌋)`。**核验通过**。
- 空槽 panic：circle `if (*(longlong*)*in_RDX == 0) unwrap_failed();`（noreturn）—— **panic 非 return，核验通过**。
- `u = i/(count-1)`：circle 三循环均 `(float)iter / ((float)count + -1.0)`（O）。**核验通过（纠正旧 i/count）**。
- BeamCount 布局（audio decomp，O）：`plVar3[0]` i64 last_count @+0、`plVar3[1]` i64 count @+8、`plVar3[2]` i32 last_frame @+0x10；
  回写 `*(int*)(plVar3+2)=frame`、`*plVar3=plVar3[1]`。**与 Rust 结构体字段/偏移一致**。

---

## §1 矩形（0x141a41ba0）—— 独立复述

**签名（寄存器面，O）**：RCX=`EventReader<ConstructTimberframeCmd>`、RDX=`ResMut<TimberframeInstanceBuffer>`
（`*(undefined4*)in_RDX[2]=*(undefined4*)(in_RDX+0x1c)` tick 写，+0x1c 触发）、R8=`Res<AssetSsboLibrary>`、
R9=`ResMut<TimberframeBeamCount>`（`*(longlong*)(*in_R9+8)=lVar19` 计数写 + tick）、
栈+0x28=`Res<TimberframeConstructionParamsCache>`（读 +8、+0xc 两 f32）。**与 restorer 面一致**。

**逐事件控制流（O）**：
1. **槽空创建**：`pbVar18=*in_RDX`；`if (*(longlong*)pbVar18==0)` → `FrozenMap::get<WoodenPlankSsbo>` +
   名字断言（`[3]==0x3d` && `memcmp("country_core::resources::roofs::roof_buffers::WoodenPlankSsbo/…/sparse_instance_buffer.rs")`）
   → vtable +0x18 → 缓冲；`__rust_alloc` 新 `SparseInstanceBufferAllocation`（6 qword + 1 byte 初值）；
   遍历 `*(buffer+0x60)` 计数器取 id；`drop_in_place` 旧 allocation；存 `{buffer@0, allocation@8}` 入槽；`goto LAB_141a41d5b`。
   空槽 + 名字失败 → `assert_failed`；缓冲空 → `unwrap_failed`（noreturn）。
2. **二查 SSBO + clear**：`FrozenMap::get` 复查 → 名字断言 → vtable；`if (slot qword==0) unwrap_failed`；
   `lVar11=*(slot+8)`（allocation），空 → unwrap_failed；互斥 `lVar11+0x18`（LOCK/INC try_lock→lock_slow）+`acquire_resource`；
   校验 `((*(all+0x10)*4+*(all+8)==4 || *(all+0x10)==0) && (*(buf+0x28)==0)) → unwrap_failed`；
   **`SparseInstanceBufferWriter::clear<WoodenPlankSsbo>()`**。
3. **事件迭代**：cursor 算术（0x28 步长）；`do {取下一事件; *cursor++} while ((*event & 1)==0)` ——
   **跳过首字节 bit0 清零条目，处理 bit0 置位条目**（与 circle 相反，O）。payload 自 `event+4`。
4. **垂直梁**：`as_curve3()` → lVar8 顶点；`lVar19=0`（**每事件重置**）；每顶点 `orientation_mat3`（0x140c99570）
   → `RenderInstance::from` → `push<WoodenPlankSsbo>`；`lVar19++`（= 顶点数）。
5. **等距梁**：`as_curve3()` 二取；相邻顶点对 `fVar27=floorf(√d² / 1.51)`，负/NaN → 0，>2^63 → 上限；
   `if (1 < n) { lVar19 += n-1; for i in 1..n: t=i/n; lerp; orientation_mat3; 截面 0x3e3851eb=0.18; push; }`。
6. **裙层**：`split_height_into_skirts()` → Vec<f32> + 栈 `{0.0, cache+0xc}` 两固定环；
   每环每边（`lVar8>=2` 时 `lVar8-1` 根）：中点 + `rotation_from_normal`（0x140c90a40）+ 高度 `h+cache+8`
   + 截面 0.2 → push；`lVar19 += (lVar8-1)`。环耗尽两次后 `goto LAB_141a42fd0`。
7. **屋脊**：`if (event[0x1c]==0)` 门控 → `roof_height_ws_from_wall_shape`（0x1408e3c70）、
   `ridge_length_01_from_rectangle_ratio`（0x1408e3b60）、`RoofRidgeDims::calc`（0x1408e2700）→
   `as_curve2()` ×2 → `min(lenA,lenB)` 根（rotation_from_normal + 截面 0.2）push；`perimeter()>0.5` →
   `get_point()` ×2 → +1 根。`lVar19 += min + (perimeter>0.5 ? 1 : 0)`。
8. **回写**：`*puVar7=local_29c`（tick）；`*(longlong*)(*in_R9+8)=lVar19`；`goto LAB_141a41f69`（下一事件）。
   循环尾部互斥释放 + return。

### 与 Rust 逐语句对照（`timberframe_rectangle.rs`）

判等面：相位顺序、`n=⌊d/1.51⌋`+`t=i/n`（`if 1<n` 门）、截面 0.2/0.18、裙层两固定环 `2×edges`（edges=lVar8-1）、
`BeamCount+8` 写回、`event[0x1c]` 门控的存在/位置、计数即写回面。**这些与 decomp 一致**。

分歧点（本复核新增，restorer 未登记）：
- **[FIX-1] 计数写回语义：跨事件累加 vs 每事件重置**。decomp 在每事件的垂直梁起点 `lVar19=0`（L331/L335），
  `BeamCount+8=lVar19` 在每事件末尾写回（L899）——**最终值 = 最后一个事件的梁数**（batch 多事件时）。
  Rust `total` 在 `for cmd in constructs.read()` **之外**累加、循环后单次写 `beams.count=total` —— **多事件帧下写回真实计数的求和，与原语义分歧**（影响 audio 触发 `last_count!=count`）。
  FIX：每事件内重置并写回 `beams.count`（`let mut per_event=0; …per_event+=…; beams.count=per_event;`）。
- **[FIX-2] 事件 bit0 门控方向**：decomp 矩形处理 **bit0=1** 条目、circle 处理 **bit0=0**（两方向相反，O）；
  Rust 两函数均依赖 bevy `read()`，无法同时复现两方向。若 bit0=形状类型标签（1=矩形/0=圆），则 Rust 的
  `get_shape()` 形状过滤是合理代理但 **未验证等价**；若 bit0 为 bevy 内部读标记则必有一方分歧。需 country-core
  事件契约暴露首字节（当前 `shape: Vec<u8>` 占位偏移 I/GAP）。→ 转集成 owner。
- `as_curve3` 高度实参（寄存器面 I，restorer 已登记）：Rust 以 `0.0` 一调用点供垂直/等距/裙三相位复用，
  原函数独立调 3 次；仅影响梁 y 位、不影响点数（计数面等价）。

已登记 GAP（核验为真）：WoodenPlankSsbo 非 SparseSsbo/SsboType、`render::RenderInstance`/`rotation_from_normal`/
`orientation_mat3`/`split_height_into_skirts` 未提供 → 槽空创建、writer.clear、逐梁 push、裙层环数 N、
屋脊门控（+0x1c 字节契约外）未落地；`total` 为真实计数下界（缺 N×edges + roof）。

---

## §2 圆形（0x1421b67a0）—— 独立复述

**签名（寄存器面，O）**：RCX=`EventReader`、RDX=`Res<TimberframeInstanceBuffer>`（**只读**：无 RDX tick 写）、
R8=`Res<AssetSsboLibrary>`、R9=`Res<TimberframeConstructionParamsCache>`（读 +8=offset、+0xc=主高两 f32）、
栈+0x28=`ResMut<TimberframeBeamCount>`（+8 计数写 + tick）。**与 restorer 面一致（纠正旧 ResMut）**。

**逐事件控制流（O）**：
1. `FrozenMap::get<WoodenPlankSsbo>` → 名字断言 → vtable +0x18 → 缓冲 lVar11；空 → 不进入 → 末尾 unwrap_failed。
   `if (*(longlong*)*in_RDX==0) unwrap_failed`（**panic 非 return**）；`lVar3=((*in_RDX))[1]` allocation，空 → panic；
   互斥 `lVar3+0x18` + acquire_resource；校验 → unwrap_failed。
2. **事件迭代**：`do{取下一; *cursor++} while ((*event & 1)!=0)` —— **跳过 bit0 置位、处理 bit0 清零条目**
   （与矩形相反，O）。payload 自 `event+4`。
3. **主环**：`perimeter()/1.5` → trunc（负/NaN→0）；`if (count<7) count=6`；`as_points2_ptcount`；
   `lVar15=0`（**每事件重置**）；`for i in 0..count: u=i/(count-1)`（0≤u≤1 + NaN panic）→ `get_pos_at_u`
   → `xvy()` → 高度 `cache+0xc`、截面 0.2 → push；`lVar15++`。
4. **裙层**：`split_height_into_skirts()` + 栈 `{0.0, cache+0xc}` 两固定环；每环 `u=i/(count-1)` +
   `rotation_from_normal` → 高度 `h+cache+8`、截面 0.2 → push（每环 count 根）；`lVar15+=count`/环。
5. **屋顶环**：`if (event[0x1c]==0)` → `roof_height_ws_from_wall_shape`（0x1408e3c70）→ 高度
   `cache+8+cache+0xc`；同采样 + `rotation_from_normal` → push（count 根）；`lVar15++`/根。
6. **回写**：tick + `*(longlong*)(*stack0x28+8)=lVar15`；`goto LAB_1421b6a1a`（下一事件）。

### 与 Rust 逐语句对照（`timberframe_circle.rs`）

判等面：`max(6,⌊perimeter/1.5⌋)`、`u=i/(count-1)`、相位结构与顺序、裙层两固定环 `2×count`、空槽 panic 面、
RDX 只读面、BeamCount+8 写回。**与 decomp 一致**。

分歧点（本复核新增，restorer 未登记）：
- **[FIX-1]（同矩形）计数写回语义**：decomp `lVar15=0` 在每事件主环前（L256）、每事件末尾写回（L584）——
  **最终值 = 最后事件梁数**；Rust 跨事件累加后单次写 `beams.count=total`。多事件帧语义分歧。FIX 同矩形。
- **[FIX-2]（同矩形）bit0 门控方向**：circle 处理 bit0=0、rect 处理 bit0=1。Rust 依赖 bevy `read()`。
  需事件契约暴露首字节 → 转集成 owner。

已登记 GAP（核验为真）：写路径（WoodenPlankSsbo/`rotation_from_normal`/`split_height_into_skirts`）、
裙层环数 N、屋顶环（门控字节契约外）；`total` 为下界。**附加注**：Rust `if buffer.0.is_empty() { panic!(…) }`
  是 CPU Vec 占位门控——因写路径未落地、`TimberframeInstanceBuffer.0` 永不填充，circle 在任一事件到达时
  **恒 panic**；原函数仅在 SSBO allocation 缺失时 panic（rect 先建槽后不 panic）。占位偏差已登记但运行期
  后果（system 不可达）建议转集成 owner 评估。

---

## §3 音频（0x1421b6610）—— 独立复述 → 判等（主体）

**签名（寄存器面，O）**：RCX=`EventReader`、RDX=`ResMut<TimberframeBeamCount>`（tick 写）、
R8=`Res<GameTime>`（只读）、R9=`EventWriter<PlayTimberframeSound>`。

**逐语句控制流（O）**：
1. 双缓冲 cursor 算术求「新事件数」（uVar13/uVar15/wrap 处理）；`*(*RCX)=*(reader+0x40)-uVar14` 推进游标；
   `if (!wrap && 有事件)` 进入触发块 —— **无新事件 → 不触发、不回写，直接 return**。
2. `lVar2=*R8`（GameTime 数据）；`iVar10=*(int*)(lVar2+0x2c)`（帧计数 @+0x2c）。
3. `plVar3=*RDX`；`if ((iVar10-(int)plVar3[2]==1) && (*plVar3 != plVar3[1]))` ——
   **`(frame-last_frame==1) && (last_count!=count)`**。
4. 触发：`ShapeParameters::center()` → `x0y()` → `get_building_size_01()`（0x1417d3f10）→
   构造 `PlayTimberframeSound`（低字节 **kind=6**；0x20 步长；+0x40 事件 id 递增）→ `Vec::push`
   （满时 `RawVec::grow_one`）。事件 0x20B：`{kind@+0, pos 8B@+8, size 8B@+0x10, id@+0x18}`。
5. 回写：`*(int*)(plVar3+2)=iVar10`（last_frame=frame）；`*plVar3=plVar3[1]`（last_count=count）；
   ResMut tick 写。**last_frame 先、last_count 后**。

### 与 Rust 对照（`timberframe_beam_count_change_audio.rs`）

判等：事件门控（`read().next() else return`）、frame 读面 +0x2c 定位、`(Δframe==1)&&(last_count!=count)`、
kind=6、`x0y(center)` 位置、last_frame/last_count 回写位置与序、`wrapping_sub` 等价 int 减法。**核心触发逻辑判等**。

分歧/GAP：
- **[FIX-3] GameClock 布局偏移**：decomp 读 `GameTime+0x2c`；章节证据（`chapters/systems/time_update.rs.md`、
  `camera-time/DIGEST.md`）一致 `frame_count@+0x2c`；但 country-core `GameClock` 结构体实际布局
  `{elapsed Duration 16B@0, total_secs f64@16, ticks_60 u64@24, frac f32@32, delta f32@0x24, frame_count u32@0x28, ran@0x2c}` ——
  **`frame_count` 字段 @0x28，与二进制读 +0x2c 差 4 字节**（结构体自身 doc 亦写 @0x2c，内部不一致）。
  Rust `time.frame_count` 读取 @0x28 → **非二进制 frame_count 位置**（跨域 FIX：country-core GameClock 布局须核对）。
- `ShapeParameters::center()`/`get_building_size_01()` 的自变量来源：decomp 无事件 payload 读（void() ABI 隐藏实参）；
  Rust 以首事件 `cmd.shape[..28]` 反解析 → **I 级**（restorer 已登记）。`get_building_size_01` 的 size 写 +0x10：
  core `PlayTimberframeSound{kind,position}` 缺 size/id 字段，事件结构 16B vs 原 0x20B —— **GAP（已登记）**。

---

## §4 插件（0x140630a20）—— glue-legitimate（1 处多余注册）

**独立复述（O）**：`insert_resource<TimberframeInstanceBuffer>` → `insert_resource<TimberframeBeamCount>`（default 内联）
→ `Components::get_resource_id`(hash 0x94f6…) 判未注册则 `EventRegistry::register_event<ConstructTimberframeCmd>`（= add_event）
→ `IntoScheduleConfigs::chain_ignore_deferred()` + `Interner::intern<dyn SystemSet>`(0x1b) + `in_set_inner` →
`SubApp::add_systems<…,Update,…>`（rect+circle+audio chain）→ `get_resource_id`(hash 0x8e1b…) 判已注册则
`Schedules::add_systems<…,OnExit<AppLoadingState>, void(ResMut<TimberframeInstanceBuffer>)>`；缺 Schedules → bevy panic 串。
**纯注册，无业务 → glue-legitimate 成立**。

对照（`lib.rs`）：
- **[FIX-4] 多余事件注册**：Rust `register_timberframe_events` 额外 `init_resource::<Events<PlayTimberframeSound>>()`，
  **原 add_timberframe 只注册 ConstructTimberframeCmd**（PlayTimberframeSound 由 country-core `add_audio_events` 注册，
  已核）。效果幂等（重复注册无害）但超出原函数体 → 轻微分歧，删或注释。
- `init_resource`（仅缺则插）vs 原 `insert_resource`（无条件覆盖）：插件构建语义近似，登记轻微差。
- `in_set` 具体 SystemSet 标签 U（0x1b intern）；OnExit<AppLoadingState> 依赖 bevy_state/AppLoadingState → 已登记
  GAP，`timberframe_on_glade_unload` 暴露供宿主接线。

`timberframe_on_glade_unload`：inline_instance，body U（无独立反编译体）；Rust `buffer.0.clear()` 为 **I 占位**
（deref 触发 tick 与注册面一致）。glue，语义 U 登记。

---

## FIX 清单（本复核新增，区别于 restorer 已登记的 GAP；2026-10-17 复验状态）

1. **[FIX-1] rect/circle `beams.count` 写回语义** → **fix_verified**。restorer 已改为每事件 `let mut event_total=0` +
   每事件末尾 `beams.count=event_total`（rect `timberframe_rectangle.rs` L86/L130；circle `timberframe_circle.rs`
   L110/L142），与 decomp 每事件重置（L331/L335/L256）+ 每事件写回（L899/L584）一致，最后事件胜。
2. **[FIX-2] 事件首字节 bit0 门控** → **fix_verified（登记面）**。rect/circle 代码注释已准确登记方向相反
   （rect 处理 bit0=1/跳过 bit0=0；circle 处理 bit0=0/跳过 bit0=1），`get_shape()` 过滤注明「合理代理但未逐字节判等」
   ——无过度断言；契约暴露仍转集成 owner。**残余**：`timberframe_rectangle.rs` 头部注释 L12 仍写
   「跳过 `(*event & 1) != 0` 条目」（=跳过 bit0 置位），与函数内正确注释（处理 bit0=1）及 RESTORE.md 矛盾 —— 应改
   为「跳过首字节 bit0 清零条目」。circle 头部 L14 正确。
3. **[FIX-3] audio 帧读 +0x2c** → **fix_verified**。`game_time_int_at_0x2c`（`timberframe_beam_count_change_audio.rs`
   L50-56）按机器码面 `read_unaligned(+0x2c)` 裸读，不依赖字段名；`GameClock.frame_count` @0x28 与章节 @0x2c 的
   布局差已登记为跨域 GAP（待 country-core 校正结构体）。`cargo check -p timberframe` 通过。
4. **[FIX-4] `lib.rs` 多余 `Events<PlayTimberframeSound>` 注册** → **fix_verified**。`register_timberframe_events`
   仅注册 `ConstructTimberframeCmd`；PlayTimberframeSound 注册已删（country-core `add_audio_events` 负责，核验存在）。

**复验判定**：4 条 FIX 全部落实；仅 FIX-2 留 1 条头部注释方向笔误（文档级，不改行为）。rect/circle 计数写回语义
现与原一致；audio 帧读按机器码面判等；glue 注册面收敛到原函数体。

## 转集成 owner（跨域交接，含 restorer 原列）

- country-core：`WoodenPlankSsbo` SparseSsbo/SsboType、`render::RenderInstance`、`resources::walls::split_height_into_skirts`、
  `systems::audio::building_audio::get_building_size_01`、`PlayTimberframeSound` 补 size/id 字段、`GameClock` 布局（FIX-3）、
  `ConstructTimberframeCmd` 暴露首字节 bit0 与 +0x1c（FIX-2）。
- utils：`rotation_from_normal`（0x140c90a40）、`Rectangle2d::orientation_mat3`（0x140c99570）。

## 过度断言核验

- 无「无未实现功能」类断言。restorer 对两构造的 `未实现`、audio 的 `判等（主体）`、add_timberframe 的
  `glue-legitimate` 均如实；audio 的 `判等` 因 FIX-3（帧偏移）与 center 来源 I 而**不宜按绝对判等计**。
- `split_height_into_skirts` 归属冲突（RE-ANALYSIS-2 登记）为跨域，不影响本 crate 判定。
