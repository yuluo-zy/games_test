# systems/timberframe 业务逻辑分析（B 版）

> 范围：仅本 crate（`crates/systems/timberframe`）内部实现。不展开 caller/callee 接线、不分析与其他 crate 的联动。
> 依赖库（std / glam / fastrand / utils / bevy_ecs / parking_lot / itertools / log）直接复用，不逆向其内部。

## 0. 证据与方法

- 目标：B patched EXE，`tiny-glade.exe`，sha256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`。
- 工具：`ghidra --worker shared`（`get_functions` 取 `signature,decompiled_code`；`disassemble_function`；`read_memory`）。只读，未写库。
- 骨架元数据：`skeleton/B/crates/systems/timberframe/src/*.rs`（PDB `compiled_procedure` 符号、源码行、依赖注解、PDB 泛型关系、内联点）。
- 符号标记：**O** = 原指令/反编译/常量字节直接观察；**I** = 由多证据推断；**U** = unknown。

### 0.1 已核实的关键常量（O，read_memory + disassemble 双重确认）

| 常量 | 浮点值 | 位模式 | 数据地址 / 指令 | 用途 |
|---|---|---|---|---|
| 矩形木梁间距除数 | 1.51 | `0x3FC147AE` | `0x142ebc300`；`141a42498 DIVSS XMM0,dword[0x142ebc300]` | 沿矩形墙边分段木梁数 |
| 圆形木梁间距除数 | 1.5 | `0x3FC00000` | `0x1429258d0`；`1421b6a89 DIVSS XMM0,dword[0x1429258d0]` | 沿圆周木梁数 |
| 木梁截面（大） | 0.2 | `0x3E4CCCCD` | 立即数（多处 `MOV dword[...],0x3e4ccccd`） | 木梁宽/厚 |
| 木梁截面（小） | ≈0.18 | `0x3E3851EB` | 立即数（`141a4273f/141a42751` 等） | 中段插值木梁截面 |
| 圆最少分段 | 6 | `CMP R8,0x7; MOV EAX,0x6; CMOVC R8,RAX` | `1421b6ad6` | 圆周木梁数下限 |

调试日志字符串（O，`read_memory@0x142ebc388`）：`"Timberframe constructor ran out of wooden plank pieces"`（矩形与圆共用，圆侧符号 `PTR_s_Timberframe_constructor_ran_out_o_142ff8f40`）。模块路径串：`"timberframe::timberframe_rectangle/rustc..."`、`"timberframe::timberframe_circle/..."`。

### 0.2 系统签名（O，PDB 泛型关系 `*.structures.json.gz` 与反编译 prologue 交叉确认）

插件注册的三个 `FunctionSystem`（`crates/systems/timberframe` 命名空间）：

```
A(矩形) void(EventReader<ConstructTimberframeCmd>, ResMut<TimberframeInstanceBuffer>,
              Res<AssetSsboLibrary>, ResMut<TimberframeBeamCount>,
              Res<TimberframeConstructionParamsCache>)
B(圆形) void(EventReader<ConstructTimberframeCmd>, ResMut<TimberframeInstanceBuffer>,
              Res<AssetSsboLibrary>, Res<TimberframeConstructionParamsCache>,
              ResMut<TimberframeBeamCount>)        // 后两个参数顺序与 A 相反
C(音频) void(EventReader<ConstructTimberframeCmd>, ResMut<TimberframeBeamCount>,
              Res<GameTime>, EventWriter<PlayTimberframeSound>)
```

反编译 prologue 印证（O）：
- 矩形 `141a41c14 MOV RSI,R9` / `141a41c17 MOV RDI,RCX` / `141a41c1a MOV R12,[RDX]` / `141a41c22 MOV RBX,[R8]` → RCX=EventReader，RDX=实例缓冲，R8=AssetSsboLibrary，R9=BeamCount。
- 圆形 `1421b6814 MOV RSI,R9` / `1421b6817 MOV R14,RDX` / `1421b681a MOV RDI,RCX`，且反编译里 `*R9` 被当作 ParamsCache（读 +8/+0xc 两个 f32），第 5 栈参当 BeamCount → 与 A 顺序相反。

---

## 1. 模块概览

| 文件 | 本 ns 函数 | 角色 |
|---|---|---|
| `src/lib.rs` | `impl$0::add_timberframe`、`timberframe_on_glade_unload`（内联） | 插件注册 / 卸载清理 |
| `src/timberframe_rectangle.rs` | `construct_timberframe_rectangle`（+`closure$0`） | 矩形墙面木梁生成（system A） |
| `src/timberframe_circle.rs` | `construct_timberframe_circle`（+`closure$0/1/2`） | 圆形墙面木梁生成（system B） |
| `src/timberframe_beam_count_change_audio.rs` | `timberframe_beam_count_change_audio`、`TimberframeBeamCount::default` | 木梁数量变化音效（system C） |

外部依赖（仅列出被调用的，均**复用不逆向**）：`utils::geometry::rectangle::Rectangle2d`、`utils::geometry::circle::Circle2d`、`utils::rotation_from_normal`、`utils::swizzlers`、`utils::components::transform::Transform`、`utils::curve::Curve*`；`country_core::asset_libraries::ssbo_library::AssetSsboLibrary`、`country_core::render::sparse_instance_buffer::{SparseInstanceBufferWriter, SparseInstanceBufferAllocation}`、`country_core::resources::roofs::roof_buffers::WoodenPlankSsbo`、`country_core::resources::roofs::roof_{ridge::RoofRidgeDims, shape::*}`、`country_core::resources::walls::split_height_into_skirts`、`country_core::resources::events::ConstructTimberframeCmd`、`country_core::systems::wall::valid_enclosure::ShapeParameters`、`country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache`、`country_core::systems::audio::{building_audio::get_building_size_01, timberframe_audio::PlayTimberframeSound}`。

---

## 2. `src/lib.rs` — 插件注册

### 2.1 `impl$0::add_timberframe`（源码行 26）

- **地址**：VA `0x140630a20`（RVA `0x630a20`），size 511（O）。
- **记录形式**：`compiled_procedure`，签名占位 `void()`（真实为 `Plugin::build` 方法，ABI unknown）。

**内部逻辑（O，逐调用）**：
1. `App::insert_resource::<timberframe::TimberframeInstanceBuffer>()`。
2. `App::insert_resource::<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>()`（其 `default` 被内联调用，见 §5.1）。
3. 条件式 `add_event`：`Components::get_resource_id()` 做「已注册?」判定；若未注册则 `EventRegistry::register_event::<country_core::resources::events::ConstructTimberframeCmd>()`。等价于 `app.add_event::<ConstructTimberframeCmd>()`。
4. `IntoScheduleConfigs::chain_ignore_deferred()` + `Interner::intern::<dyn SystemSet>()` + `ScheduleConfigs::in_set_inner(...)`：把一组 system 串联（chain）并放入某个 `SystemSet`。
5. `SubApp::add_systems::<..., Update, ...>(...)`：在 **`Update`** 调度注册三个 system（system A/B/C，chain 顺序 A→B→C）。
6. 二次 `Components::get_resource_id()` + `Schedules::add_systems::<..., OnExit<AppLoadingState>, void(ResMut<TimberframeInstanceBuffer>)>(...)`：在 **`OnExit(AppLoadingState)`** 注册清理 system（即 `timberframe_on_glade_unload`，参数 `ResMut<TimberframeInstanceBuffer>`）。
   - 该分支带「resource 不存在」的 panic 字符串（Bevy `Schedules` 标准报错文案）。

**算法**：无（注册/接线）。

**数据结构/常量**：
- 本 crate 资源：`TimberframeInstanceBuffer`（lib.rs 骨架命名实体，字段 U）、`TimberframeBeamCount`。
- 事件：`ConstructTimberframeCmd`（country_core），`PlayTimberframeSound`（country_core）。
- 调度：`Update`、`OnExit(AppLoadingState)`；`SystemSet`（具体集合 U）。

**未知**：`SystemSet` 的具体类型/标签 U；chain 与 in_set 的确切先后与集合名 U；三个 system 是否严格 A→B→C（由 chain 决定，顺序 O，但 B 的插入点仅由 PDB tuple 推断为 I）。

### 2.2 `timberframe_on_glade_unload`（源码行 50）

- **记录形式**：`inline_instance`（无独立符号地址；仅内联出现，instances=5）。
- **签名（PDB 泛型）**：`void(ResMut<timberframe::TimberframeInstanceBuffer>)`，注册在 `OnExit<AppLoadingState>`。
- **内部逻辑**：U（无独立反编译体）。由签名与注册点推断：场景卸载时清理/重置 `TimberframeInstanceBuffer` 资源（I）。
- **未知**：函数体逐指令行为 U（内联，未单独恢复）。

---

## 3. `src/timberframe_rectangle.rs` — 矩形木梁

### 3.1 `construct_timberframe_rectangle`（源码行 21）

- **地址**：VA `0x141a41ba0`（RVA `0x1a41ba0`），size 7340（O）。
- **记录形式**：`compiled_procedure`；真实签名见 §0.2 system A（O）。

**内部逻辑（输入→处理→输出）**：

输入：`EventReader<ConstructTimberframeCmd>`（每个 cmd 描述一面矩形墙 + 形状参数）、`ResMut<TimberframeInstanceBuffer>`、`Res<AssetSsboLibrary>`、`ResMut<TimberframeBeamCount>`、`Res<TimberframeConstructionParamsCache>`。

处理（按反编译阶段，O）：

1. **取 SSBO / 加锁**（约 137–241 行）：`AssetSsboLibrary::get(SsboId::WoodenPlankSsbo)`（`FrozenMap::get`，key 串 `"country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"`）；对缓冲区互斥锁（`parking_lot`）加锁；`SparseInstanceBufferWriter::clear::<WoodenPlankSsbo>()` 清空写入器。
2. **事件迭代准备**（242–272）：按 `Events` 双缓冲游标（偏移 `+0x18/+0x38` 两组）计算新事件；事件步长 `0x28`（40 字节）；`ParamsCache` 两参数入 `local_10c(+8)`、`fVar33(+0xc)`。
3. **垂直木梁（沿墙脊线顶点）**（273–421）：对每个有效事件，`Rectangle2d::as_curve3()` 取墙面 3D 折线；对每个顶点做 `orientation_mat3()` → 由法线求旋转四元数（见 §7）→ 组装 `RenderInstance`（`country_core::resources::render::impl$1::from()`）→ `SparseInstanceBufferWriter::push::<WoodenPlankSsbo>()`。截面用 `0x3E4CCCCD`(0.2)。
4. **等距木梁（沿墙边插值）**（428–559）：再次 `as_curve3()`；对相邻顶点对，取 3D 距离 `d`，梁数 `n = floor(d / 1.51)`；对 `i in 1..n` 以 `t = i/n` 线性插值（`lerp`）出位置，重复四元数/朝向计算并 push。截面用 `0x3E3851EB`(≈0.18)。
5. **横向裙板/腰带（高度分层）**（567–731）：`country_core::resources::walls::split_height_into_skirts()` 把墙高切成若干"裙"层；每层 `as_curve3()` 后沿边 push 木梁（截面 0.2）。
6. **屋脊/山墙**（732–897）：条件 `event[0x1c]==0` 时，`roof_height_ws_from_wall_shape()`、`ridge_length_01_from_rectangle_ratio()`、`RoofRidgeDims::calc()`；`as_curve2()` 取屋脊线并 push 木梁；再 `perimeter()`/`get_point()` 在 `perimeter>0.5` 时 push 一根（山墙顶？）。
7. **回写**（898–900）：`*TimberframeBeamCount.0(+8) = lVar19`（本次生成木梁总数）；回写 `ResMut` 的 change tick。

输出：把木梁实例写入 `WoodenPlankSsbo` 的稀疏实例缓冲；把木梁计数写回 `TimberframeBeamCount`。

**算法**：
- 事件驱动实例生成（Bevy system）。
- **弧长均匀采样**：`Rectangle2d::as_curve2/as_curve3`、`get_point`、`perimeter`（utils，复用）。
- **等距布梁**：`floor(len / 1.51)` 段 + 线性插值（`t=i/n`）。已知算法：均匀分段 / lerp。
- **法线→旋转**：`utils::rotation_from_normal`（四元数），配 `orientation_mat3()` 局部基。
- **高度分层**：`split_height_into_skirts`（country_core）。
- **屋脊几何**：`RoofRidgeDims::calc`（country_core）。

**数据结构/常量**：
- `WoodenPlankSsbo`（"wooden plank pieces" 实例池）；`SparseInstanceBufferWriter/Allocation<WoodenPlankSsbo>`。
- `ConstructTimberframeCmd` 事件：步长 0x28(40B)；偏移 `+0x1c` 一个字节门控屋脊段（U 语义）；payload 从 `+4` 起。
- `TimberframeConstructionParamsCache`（country_core）：观测到 `+8`(f32)、`+0xc`(f32) 被读；矩形里 `+8` 用于裙层位置偏移、`+0xc` 用作木梁厚度缩放（I）。
- 推送实例带类型/标签字段（观测 `local_198=0`、`uStack_190=0x200000000`）语义 U。
- 常量：`1.51`（间距）、`0.2`/`0.18`（截面）、`0.5`（perimeter 阈值）。

**未知**：
- 推送实例中 `0x0000000200000000` 字段的语义/枚举 U。
- `ParamsCache` `+8/+0xc` 的业务名（"距墙偏移/梁厚"?）U。
- 事件 `+0x1c` 字节语义（是否 roof/gable 开关）U。
- `closure$0`（源码行 40）逐语句细节（内联，未单独成函数）U。
- `ResMut` tick 回写与 `TimberframeBeamCount` 完整布局 U。

### 3.2 `construct_timberframe_rectangle::closure$0`（源码行 40）

- **记录形式**：`inline_instance, inlinee_declaration`，instances=6，无独立地址（O）。
- **内部逻辑**：U（内联）。由父函数上下文推断：位于「垂直木梁/等距木梁」循环内，做单根木梁的朝向/实例组装（I）。
- **未知**：闭包捕获与逐指令行为 U。

---

## 4. `src/timberframe_circle.rs` — 圆形木梁

### 4.1 `construct_timberframe_circle`（源码行 24）

- **地址**：VA `0x1421b67a0`（RVA `0x21b67a0`），size 4542（O）。
- **记录形式**：`compiled_procedure`；真实签名见 §0.2 system B（O，注意末两参顺序与矩形相反）。

**内部逻辑（输入→处理→输出）**：

输入同矩形（参数顺序不同）。

处理（O）：
1. **取 SSBO / 加锁 / clear**：同矩形（`AssetSsboLibrary::get(WoodenPlankSsbo)` → 加锁 → `SparseInstanceBufferWriter::clear`）。
2. **事件迭代准备**：同矩形；`ParamsCache` 从 `*R9` 读 `+8/+0xc`。
3. **垂直木梁（沿圆周）**：`Circle2d::perimeter() / 1.5` → 取整得圆周木梁数 `n`；`if n < 7 { n = 6 }`（下限 6）；`Circle2d::as_points2_ptcount(n)` 采样 `n` 点；对 `i in 0..n`：`u = i / (n-1)`，`get_pos_at_u(u)` 得位置，组装并 push（截面 0.2）。`u` 越界 `[0,1]` 或 NaN 时 panic（utils 内联断言）。
4. **横向裙板（高度分层）**：`split_height_into_skirts()` 得各裙层；每层 `as_points2_ptcount` + 沿圆周 push 木梁（用 `itertools` + `chain(once, map(...))` 收集点，见 `closure$1/2`）。
5. **顶部/屋脊环**：`roof_height_ws_from_wall_shape()`；再次 `as_points2_ptcount` 并在抬高的环上 push 木梁。
6. **回写**：`*TimberframeBeamCount.0(+8) = lVar15`；回写 tick。

输出：同矩形（写 `WoodenPlankSsbo` + `TimberframeBeamCount`）。

**算法**：
- **圆周均匀采样**：`Circle2d::as_points2_ptcount` + `get_pos_at_u`（utils）。
- **计数**：`floor(perimeter / 1.5)`，下限 6。已知算法：按弧长估算分段数。
- 法线→旋转、高度分层同矩形。

**数据结构/常量**：同矩形；常量 `1.5`、`0.2`、下限 `6`。

**未知**：
- 与矩形共享规则的**显式复用方式** U（两函数各自实现，未见公共 helper 符号；是否有源级共享函数 U）。
- `closure$1`（行 84）、`closure$2`（行 109）细节 U。
- 事件 `+0x1c` 门控（同矩形）U。

### 4.2 圆形闭包（源码行 38 / 84 / 109）

- `closure$0`（行 38，instances=4）：圆周点 map（`as_points2_ptcount::closure_env_0` 之上再 map），生成单点木梁。
- `closure$1`（行 84，instances=5）、`closure$2`（行 109，instances=3）：裙板/屋脊环的点收集（`chain` + `map`）。
- **内部逻辑**：U（均内联，无独立地址）。

---

## 5. `src/timberframe_beam_count_change_audio.rs` — 木梁数量变化音效

### 5.1 `TimberframeBeamCount`（+ `impl$1::default`，源码行 11）

- **记录形式**：`inline_instance`（`default`，instances=2，无独立地址）。
- **内部逻辑**：U（无独立体）。由 `insert_resource` 注册与矩形/圆形对其 `+8`(i64 计数) 的写入推断：资源持「当前木梁数」等状态，`default` 初始化为零/默认值（I）。
- **未知**：字段布局、`default` 各字段初值 U。

### 5.2 `timberframe_beam_count_change_audio`（源码行 18）

- **地址**：VA `0x1421b6610`（RVA `0x21b6610`），size 395（O）。
- **记录形式**：`compiled_procedure`；签名见 §0.2 system C（O）。

**内部逻辑（O）**：
1. **事件迭代**：`EventReader<ConstructTimberframeCmd>` 游标算术（`*reader` 与 `+0x18/+0x38` 两组），求新事件数 `uVar14`；若为 0 直接返回（无变化不发音）。
2. **取两处计数**：`ResMut<TimberframeBeamCount>` 与 `Res<GameTime>`（寄存器 `R8/RDX`，映射见 §0.2 备注）：
   - `iVar10 = *(int*)(game_time + 0x2c)`（一个 int，I 推断为新木梁数）。
   - `plVar3 = *beam_count_res`；`plVar3[2]` 为旧木梁数（int），`plVar3[0]/[1]` 为切片指针。
3. **判定**：`if (iVar10 - old_count == 1) && (非空) { 发音 }` —— **木梁数增加 1 时**触发。
4. **发音**：`ShapeParameters::center()` 取建筑中心，`utils::swizzlers::x0y()`、`building_audio::get_building_size_01()` 取尺寸；构造 `PlayTimberframeSound` 事件（观测 variant 标签 = `6`，字段含 size/pos）并 `Vec::push`（必要时 `RawVec::grow_one`）写入 `EventWriter<PlayTimberframeSound>`。
5. **回写**：`*(int*)(plVar3+2) = iVar10`（更新旧计数），游标/切片推进（`*plVar3 = plVar3[1]`），回写 tick。

输出：向 `PlayTimberframeSound` 事件队列投递一条音效事件。

**算法**：计数差分（`new - old == 1`）；无几何算法。

**数据结构/常量**：
- `PlayTimberframeSound`（country_core）：观测事件大小 0x20(32B)，variant 标签 `6`，含建筑尺寸/位置。
- `TimberframeBeamCount`：`plVar3[2]`（偏移 +0x10 起 int）存旧计数（I）。
- `ShapeParameters::center`、`get_building_size_01`（country_core）。

**未知**：
- 参数寄存器确切映射（`R8`=GameTime? 还是 BeamCount?）：PDB 顺序为 `(EventReader, ResMut<BeamCount>, Res<GameTime>, EventWriter)`，但反编译中 `R8` 的 `+0x2c` int 与 `RDX` 的切片并存，二者语义未完全钉死 → U。
- `PlayTimberframeSound` 各字段语义、variant `6` 含义 U。
- 触发是否还需其它条件（如时间间隔）U。

---

## 6. 逻辑总览

```
lib.rs: add_timberframe
  ├─ insert_resource::<TimberframeInstanceBuffer>
  ├─ insert_resource::<TimberframeBeamCount::default>
  ├─ add_event::<ConstructTimberframeCmd>
  ├─ Update: add_systems((rect, circle, audio).chain().in_set(..))
  └─ OnExit(AppLoadingState): add_systems(timberframe_on_glade_unload)

每帧 Update：
  construct_timberframe_rectangle (system A)  ─┐  读 ConstructTimberframeCmd 事件，
  construct_timberframe_circle   (system B)  ─┼─ 生成木梁实例写入 WoodenPlankSsbo；
                                              │  把本帧木梁总数写入 TimberframeBeamCount
  timberframe_beam_count_change_audio (C)  ───┘  比较木梁数变化，+1 时投递 PlayTimberframeSound
```

- 矩形与圆形是**两个独立 system**（同名参数、末两参顺序相反），各按形状生成木梁；都在 `Update` 内 chain，音频在其后。
- 二者结构高度同构：取 SSBO → 清写 → 逐事件 → 垂直梁 → 高度分层裙板 → （矩形另有屋脊；圆形有顶环）→ 回写计数。

## 7. 算法清单

| 算法 | 位置 | 说明 | 证据 |
|---|---|---|---|
| 事件驱动实例生成 | A/B/C | Bevy system 读 `ConstructTimberframeCmd` | O |
| 弧长均匀采样（矩形） | A | `Rectangle2d::{as_curve2,as_curve3,get_point,perimeter}` | O |
| 弧长均匀采样（圆） | B | `Circle2d::{as_points2_ptcount,get_pos_at_u,perimeter}` | O |
| 等距布梁 | A | `n=floor(edge_len/1.51)` + lerp(`t=i/n`) | O（常量已核） |
| 圆周布梁计数 | B | `n=floor(perimeter/1.5)`，下限 6 | O（常量已核） |
| 法线→四元数旋转 | A/B | `utils::rotation_from_normal` + `orientation_mat3` | O |
| 高度分层（裙） | A/B | `split_height_into_skirts`（country_core） | O |
| 屋脊几何 | A | `RoofRidgeDims::calc` + `ridge_length_01_from_rectangle_ratio` + `roof_height_ws_from_wall_shape` | O |
| 计数差分触发音效 | C | `new - old == 1` | O |
| 线性插值 / 均匀分段 | A | lerp + floor 分段（标准） | O |

## 8. 未知清单（诚实登记）

1. 推送木梁实例中字段 `0x0000000200000000`（可能材质/类型标签）语义 —— **U**。
2. `TimberframeConstructionParamsCache` 偏移 `+8`/`+0xc` 两 f32 的业务名 —— **U**。
3. 事件 `ConstructTimberframeCmd` 偏移 `+0x1c` 字节的门控语义（是否 roof/gable）—— **U**。
4. `TimberframeInstanceBuffer` / `TimberframeBeamCount` 完整字段布局与 `default` 初值 —— **U**。
5. `PlayTimberframeSound` 字段语义与 variant `6` —— **U**；音频系统参数寄存器映射 —— **U**。
6. 矩形与圆形是否有源级共享 helper（两函数各自展开，未见公共符号）—— **U**。
7. 内联闭包 `rectangle::closure$0`、`circle::closure$0/1/2`、`TimberframeBeamCount::default`、`timberframe_on_glade_unload` 的逐指令体 —— **U**（均无独立地址）。
8. `1.51`（矩形）与 `1.5`（圆）间距常量不一致是否有意 —— **U**。
9. `SystemSet` 具体标签、`chain`/`in_set` 精确顺序 —— **I/U**。

## 9. 最小 Rust 实现草案（仅供参照，未写入工程）

> 仅当对应函数内部逻辑已**完全确定**且不引入新 core 接口时给出；因 §8 尚有多项 U，以下为**骨架草案**，带 `// U` 处需原侧复核。假设 `country-core`/`utils` 契约冻结。

```rust
// crates/systems/timberframe/src/lib.rs
impl Plugin for TimberframePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimberframeInstanceBuffer::default());       // U: 具体 Default
        app.insert_resource(TimberframeBeamCount::default());
        app.add_event::<ConstructTimberframeCmd>();
        app.add_systems(
            Update,
            (
                construct_timberframe_rectangle,
                construct_timberframe_circle,
                timberframe_beam_count_change_audio,
            )
                .chain_ignore_deferred()
                .in_set(TimberframeSet /* U: 集合名 */),
        );
        app.add_systems(OnExit(AppLoadingState), timberframe_on_glade_unload);
    }
}
```

```rust
// crates/systems/timberframe/src/timberframe_rectangle.rs（核心布梁，节选）
const PLANK_SPACING_RECT: f32 = 1.51;   // O
const BEAM_SECTION: f32 = 0.2;          // O
const BEAM_SECTION_MID: f32 = 0.18;     // O
// 等距布梁：
let n = (seg_len / PLANK_SPACING_RECT).floor() as u32;
for i in 1..n {
    let t = i as f32 / n as f32;
    let pos = a.lerp(b, t);              // lerp
    let rot = rotation_from_normal(/* 法线 */); // utils
    writer.push::<WoodenPlankSsbo>(instance(pos, rot, BEAM_SECTION_MID));
}
// 回写计数：
beam_count.0 = total_planks;             // U: 字段名
```

```rust
// crates/systems/timberframe/src/timberframe_circle.rs（核心布梁，节选）
const PLANK_SPACING_CIRCLE: f32 = 1.5;  // O
const MIN_SEGMENTS: usize = 6;          // O
let mut n = (circle.perimeter() / PLANK_SPACING_CIRCLE) as usize;
if n < MIN_SEGMENTS { n = MIN_SEGMENTS; }
let pts = circle.as_points2_ptcount(n);
for (i, _) in pts.iter().enumerate() {
    let u = i as f32 / (n as f32 - 1.0);
    let pos = circle.get_pos_at_u(u);
    writer.push::<WoodenPlankSsbo>(instance(pos, /* rot */, BEAM_SECTION));
}
```

```rust
// crates/systems/timberframe/src/timberframe_beam_count_change_audio.rs（节选）
for _cmd in reader.read() {
    let new = /* 当前木梁数 */;          // U: 具体来源
    if new - beam_count.prev == 1 {
        let size = get_building_size_01(/* ... */);
        let center = ShapeParameters::center(/* ... */);
        sounds.write(PlayTimberframeSound { /* variant 6, pos=center, size */ }); // U: 字段
    }
    beam_count.prev = new;
}
```

---

## 附：逐函数地址速查

| 函数 | VA | 形式 | 源码行 |
|---|---|---|---|
| `construct_timberframe_rectangle` | `0x141a41ba0` | compiled_procedure | 21 |
| `construct_timberframe_rectangle::closure$0` | —（内联） | inline | 40 |
| `construct_timberframe_circle` | `0x1421b67a0` | compiled_procedure | 24 |
| `construct_timberframe_circle::closure$0/1/2` | —（内联） | inline | 38/84/109 |
| `timberframe_beam_count_change_audio` | `0x1421b6610` | compiled_procedure | 18 |
| `TimberframeBeamCount::default` | —（内联） | inline | 11 |
| `impl$0::add_timberframe` | `0x140630a20` | compiled_procedure | 26 |
| `timberframe_on_glade_unload` | —（内联） | inline | 50 |
