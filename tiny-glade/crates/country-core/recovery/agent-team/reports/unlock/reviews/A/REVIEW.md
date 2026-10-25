# REVIEW — unlock A 清单 12 项独立复核（reviewer-A-unlock，2026-10-17）

> 角色：reviewer-A-unlock（只读）。硬标准：**fresh 反编译/反汇编 → 独立复述 → 逐语句/逐字段判等**。
> 未改 `src`；未改任何 restorer 证据文件。身份：B patched `tiny-glade.exe`
> （sha256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`）。
> 工具：Ghidra `--worker shared`（只读）+ radare2 `--worker reviewer-a-unlock`
> + 独立 taffy 0.3.13 编译探针。所有原始结果在本目录 `evidence/`（jobs/results/decomp）。

## 0. 三态回执

- **主清单（unlock 轮 12 项）：判等 11 / 需修 1 / 未实现 0。**
- **任务点名的附加 6 项（跨轮/缺口项）：判等 6 / 需修 0 / 未实现 0。**
- **总计 17 判等 / 1 需修 / 0 未实现。FIX 清单：FIX-A1（见 §4）。**
- 冷却表长度复核：0x143038ae8 处 **19×f32（id 0..18）= 76B**，其后 4B 对齐 padding
  至 +0x50，再接 u64 表 `[8,9,4,15,16,12]`。`[f32;19]` 与原生数组长度一致
  （consume/hint 复核亦记 19 项）；0x3e99999a=0.3 的 id 集合与直读逐项一致。无需调整。

## 1. 方法与证据

| 证据 | 内容 |
|---|---|
| `evidence/jobs/ghidra-b1..b9.json` + `evidence/results/*.json` | 38 个函数 fresh 反编译/callees（含 `SignifierEntityBuilder::{new,build}` 0x14089d9b0/0x14089dd50） |
| `evidence/jobs/r2-setters.json` | 8 个 taffy setter 单态逐指令（immediate 写面） |
| `evidence/jobs/r2-geo.json` | geo_src 两跳表原始字节 + 两函数指令流 + legacy_raycast + DebugColor 跳表 |
| `evidence/jobs/r2-closure.json` | `DebugLine::show` 闭包 0x140b055a0 全部指令（builder 6-qword payload 构造点） |
| `evidence/jobs/r2-colors.json` | DebugLine 颜色跳表 6 目标常量 |
| `evidence/jobs/ghidra-mem.json` | 冷却表 0x143038ae8 直读 + 颜色字符串 |
| `evidence/decomp/` | 每个地址的独立反编译文本 |
| `/tmp/taffy-probe`（本机临时 crate） | 独立编译 taffy 0.3.13：Style 偏移/枚举判别（非复用 restorer 探针） |

## 2. 主清单 12 项（审计 A2 6 + A4 6）

| # | 审计项 | 独立复核证据（fresh） | 判等要点 / 差异 | verdict |
|---|---|---|---|---|
| 1 | `DebugColor` `Debug`/`as_array`（A2，0x140b05c70） | 跳表 0x142ae7334 原始 24B 解码 → 6 目标 0x140b05c89/ccd/c9b/cb4/c92/ce6；字符串 0x142ae71c1 = `White/Red/Blue/Green/Grey`；Custom 目标 `add rcx,4` + `lea rdx,"Custom"` + call 0x140d872d0（`debug_tuple_field1_finish`） | 五单元变体原名；`Custom([r,g,b])`。Rust 手写 `Debug` + `as_array()` 逐字一致。 | **判等** |
| 2 | `SignifierFactory::new`（A2，0x14089d8b0） | 全函数反编译 | `memcpy(name)` 后写 5 qword：`+0 = len, +8 = ptr, +0x10 = len, +0x18 = inner, +0x20 = marker`。Rust `new` 同序同值（Box 接管载荷）。 | **判等** |
| 3 | `UiSignifierStream` 承接/唤醒（A2，0x140b2cf90 / 0x140b2cfa0） | `write`（0x140b2cf90）逐指令；`signifier_create` 全反编译 | `write`：`next@+0x168` 原子预留 → 槽 stride **0x28**（5 qword）→ `committed@+0x160` 提交。`signifier_create`：`memcpy/memset 0x170`（mem::take）+ String clone + Arc `LOCK +1` + `UiState+0x20` indexmap **0x28B/条** + `calculate_hash<(WidgetId,Option<&str>)>` + vtable `+0x28` 调用 + `data_ptr = stored + 0x10 + ((align-1)&!0xf)` + 逐实体 `insert<SignifierId>`。Rust Raw/Factory 双槽、take_entries、wake、hash、insert 全对应。**既有 GAP（restorer 已登记）**：UiState+0x20 持久配置表克隆未建模，当前按帧流 spawn，同帧可观察行为一致。 | **判等（限定）** |
| 4 | `mei_pre_update` 9 参签名（A2，0x1406842e0） | `procs.tsv` 0x140f968d0 `run_unsafe<FunctionSystem<...>>` 单态元组独立读取 | 9 参逐一：`ResMut<MeiContext>, EventReader<MouseWheel>, ResMut<UiNavState>, Res<WindowSize>, Res<Time>, Res<ActionSetStates>, Option<ResMut<UiState>>, Option<Res<CursorPositionSS>>, EventWriter<PlayMenuUiSound>`。Rust 签名同序。主体未实现（restorer 已登记）。 | **判等（签名）** |
| 5 | `mei_post_update` 9 参签名（A2，0x1406884c0） | `procs.tsv` 0x140f971e0 同法 | `ResMut<MeiContext>, ResMut<TextRenderer>, ResMut<UiNavState>, ResMut<AssetTextureLibrary>, Res<GraphicsDeviceResource>, Res<WindowSize>, Option<Res<SteamClientResource>>, Option<Res<SteamInput>>, Res<ActiveInputModeTracker>`。Rust 同序。 | **判等（签名）** |
| 6 | `render_taffy_ui_to_widgets` 13 参（A2，0x142338990） | `procs.tsv` 0x141ea4010 同法 | Query 7 元组 + Res<UiState>/ResMut<UiNavState>/UiClickEvent/UiDragEvent/Res<AppMode>/Res<UiSignifierStream>/Res<CursorPositionSS>/Res<GameConfig>/Res<ActionSetStates>/UiHistoryButtonPressed/Commands/Res<ToolbarSizeMult>。Rust 同序同类型（`UiHistoryButtonPressed` 路径 `backend::input::calculate_input_delta` 与原 PDB 一致）。 | **判等（签名）** |
| 7 | `DisplayHintRequest` 布局（A4，0x14227fc30 等） | orchestrator 全反编译 + 12 个写面 fresh 反编译/反汇编 + `hint_position_update` 0x141c50510 + 冷却表直读 | 见 §2.1。**T=0x50、EventInstance stride=0x58**；`placement@+0`（24B）、`display@+0x18`（0x30B）、`flags@+0x48`（bit0）、`id@+0x49`、event_id@+0x50。写面 0x1200 / 0x0a01-0c01 / 0x0300 / 0x0000 / 0x0200 / 0x0700 / 0x0600 / 0x0100 / 0x0400 / 0x0800 / 0xd00/0xe00/100/1000/1100 / clutter 0x100+placement3 / deselect 0+6+shape 全部逐值命中。 | **判等** |
| 8 | `sort_display_hint_requests`（A4，0x1425e5b70 / 0x1425e57e0） | 两排序实例全反编译 | 所有比较均为 `*(u8*)(a+0x49) < *(u8*)(b+0x49)`；`insertion_sort_shift_left` 与 `sort8_stable` 同键。Rust `sort_by(|a,b| a.id.cmp(&b.id))`＝稳定升序。 | **判等** |
| 9 | `HintType::Rollercoaster = 18`（A4） | `prompt_rollercoaster` 0x14227bb70 fresh | 事件 +0x48 u16 = **0x1200** → flags 0 / id 0x12。Rust 枚举 18 + `from_id(18)`。 | **判等** |
| 10 | `DebugLine::show`（A4，0x140b04f70） | show 全反编译 + 闭包体 0x140b055a0（含 build 调用点指令流）+ 颜色跳表 0x142ae731c + 常量区 + `DebugLine::new` 0x140b04e70 + `SignifierEntityBuilder::build` 默认（line）分支 0x14089dd50 | `name = format!("{:?}{}{:?}{}", points, colors.is_some(), color, line_width)`（4 空片段）；`colors=None` 时按 tag 0..5 填 **(1,1,1)/(1,0,0)/(0,0,1)/(0,1,0)/(0.1,0.1,0.1)/Custom@+0x34**，再 `repeat(len)`；长度 = 相邻距离和（`JZ` 复用 `CMP len,2` 的 ZF，len==2 走单段）；ArcInner 数据 `{points Vec, colors Vec, length, width, ui_pass, dashed}`；闭包 shader `dashed_line`(11) / `vertex_color_shader`(19)、`ui_pass^1`、`animation_duration 0.0`、dashed 先 `DashedLineLength(len*4.0)` 再 `DrawTopology::Alt(width)`、返回 1 元素。**发现 FIX-A1**：闭包向 builder 写的 6 qword 是两段 `Vec` 的 `{cap,ptr,len}`：`{points.cap@+0x10, points.ptr@+0x18, points.len@+0x20, colors.cap@+0x28, colors.ptr@+0x30, colors.len@+0x38}`（指令 0x140b05739..0x140b0574d 从两条新 Vec 记录整块搬运；build 默认分支以 `in_RDX[5]=payload[3]` 作 colors Vec 的 cap）。Rust 第 4 qword 误用 `points.capacity()`，应为 `colors.capacity()`。其余逐项一致。 | **需修（FIX-A1）** |
| 11 | `NaniBundle<T>` 0x48B（A4） | `component_ids` 0x14267a9e0 + `write_components` 0x141a07f20 | 组件集/声明序 `NaniInstanceDesc → Transform → InstanceDataMarker<T>`；写面 desc 读 `+0x30..0x48`（0x18B）、transform 读 `+0x00..0x30`（0x30B）、marker ZST ⇒ **0x48B**。Rust 声明序同；布局测试本次已跑 **4/4 通过**（原 ENOSPC 已解除）。 | **判等** |
| 12 | `DrawableMeshBundle` 0x40B（A4） | `component_ids` 0x14267c2a0 + `write_components` 0x141fe7aa0 | 组件集/序 `Handle<Mesh> → Handle<RasterShaderProgram> → Transform`；写面 `+0x30 = Mesh`、`+0x38 = RasterShaderProgram`、`+0x00..0x30 = Transform` ⇒ **0x40B**。Rust 同；布局测试通过。 | **判等** |

### 2.1 `DisplayHintRequest` 逐字段独立复述与对照

fresh `hint_orchestrator` 0x14227fc30 事件迭代：
`uVar13 * 0x58 + base` → **步长 0x58**；`*(int*)(req+0x00) == 10` → insert
`InteractionPrompt`；`+0x00..+0x18` 24B 整体作为 `HintPlacement`；
`*(int*)(req+0x18) != 0xc4` → clone+insert `BindingHintDisplay`；
`*(char*)(req+0x48) != 0` → insert `HintProgressBar`；`bVar2 = *(u8*)(req+0x49)` 走
`<0x12` / `==0x12` / `==2` / else 四个文本分支；event_id 写 `+0x50`。

写面（fresh 逐值）与 Rust call site 对照：

| 写面 | 原生 +0 / +0x48 u16 | flags/id | Rust call site |
|---|---|---|---|
| prompt_rollercoaster 0x14227bb70 | 10(ScreenCenter) / 0x1200 | 0 / 18 | `HintType::Rollercoaster` + ScreenCenter |
| hint_camera_display_condition 0x14227c910 | 6 / {0x0a01,0x0b01,0x0c01} | 1 / 10,11,12 | CameraPan/Rotate/Zoom + flag 1 |
| hint_disable_snapping 0x14227e140 | `bVar2+6` / 0x0300 | 0 / 3 | DisableSnapping + `wall_shape_placement` |
| hint_deselect_tool 0x1422853c0 | `6` 或 `shape+6` / **0x0000** | 0 / 0 | DeselectTool + `wall_shape_placement` |
| hint_undo 0x142284220 | 6 / 0x0200 | 0 / 2 | Undo |
| hint_roof_shape 0x142284ed0 | 6 / 0x0700（`word [rsp+0xf0]`、`dword [rsp+0xa8]`） | 0 / 7 | RoofShape |
| hint_terrain_layering 0x142283cc0 | 6 / 0x0600 | 0 / 6 | Terrain |
| hint_terrain_focus 0x142283980 | 6 / 0x0100 | 0 / 1 | FocusedEdit |
| hint_terrain_steepness 0x142283640 | 6 / 0x0400 | 0 / 4 | TerrainSteepness |
| hint_unlink_windows 0x14227d760 | 6 / 0x0800 | 0 / 8 | UnlinkWindows |
| hint_stairs 0x142285a20 ×5 | 6 / 0xd00,0xe00,0x100,0x1000,0x1100 | 0 / 13,14,1,16,17 | StairsPlaced/Focus/FocusedEdit/Detach/CanDetach |
| hint_clutter_focused_edit_hint 0x141c3db80 | **3** / 0x0100 | 0 / 1 | Subject(DecoClutter) / FocusedEdit |

consumers：`hint_position_update` 0x141c50510 组件步长 **0x18**，`switch(u32@+0)` 显式
case **6/7/8/10**：6 = 光标 + `2*margin`；7 = `CameraMatrices::eye_right` +
`world_space_to_screen_space`；8 = `Rectangle2d::as_points3` + 四角投影；10 = `0.5*w,0.5*h`；
其它走 0x142f14844（restorer 已登记 U）。

冷却表 0x143038ae8 直读（128B）：前 **19×f32 = [0,0.3,0,0,0.3,0.3,0.3,0.3,0,0,0.3,0.3,0.3,0,0,0,0,0,0]**，
其后 4B padding 到 +0x50 接 u64 表 `[8,9,4,15,16,12]` ⇒ id 1/4/5/6/7/10/11/12 = 0.3，
id 18 = 0；与报告及 `[f32;19]` 一致。

## 3. 任务点名的附加 6 项

| # | 项 | 独立复核证据（fresh） | 判等要点 / 差异 | verdict |
|---|---|---|---|---|
| 13 | `geo_src_to_edit_subject` / `_w_deco_clutter` 跳表链（0x140977f50 / 0x140978000） | radare2 原始指令 70/130 条；两跳表 17×i32 原始字节手工解码 | plain：tag 0/1/8/9→Wall(1)@+8；2→Roof(2)@+8；13/14→StairPoint(4)@+4；4/5/6/7/10/11→None(0)；3/12/15/16→panic。`_w_deco` 仅 tag 4/5 改道：`r8==0→None` → `DecoratorStorage::get` → `[+0x48]==0→None` → hashbrown 条目 **stride 0x50**、键 `[+0]`、ty `byte[+0x4E]` → `al=ty+0xC8`：`al==7→StairPoint(4)`，`al==6 || al>=8→DecoClutter(3)`，否则 None。Rust 逐 tag/逐门一致（`storage_addr` 0/1/2 映射沿用 DecoratorStorage 合同）。 | **判等** |
| 14 | `legacy_raycast` 命中码（0x1408cc680） | radare2 指令流 + `Rapier::raycast` 0x1408cdc60 fresh | `mask = 0xFB`、`max_toi = 0x7F7FFFFF(f32::MAX)`；miss 判据 `cmp dword [rsp+0xb4], 2`（结果 +0x34），miss 仅写 `+0x30 = u32 2`；hit 全量 0x40B 拷贝。Rust `Rapier::raycast(self,o,d,0xfb)`；`is_miss()` 读 discriminant(+0x34)；miss 写 `toi = f32::from_bits(2)`（`+0x30` 位型 0x00000002）；`#[repr(C)]` 偏移与机器码一致。未初始化尾字节差异为既登记 I。 | **判等** |
| 15 | `UiNav` 0x40 / `HistoryButton` 1B 布局 | `register_component<UiNav>` 0x141e36510 / `<HistoryButton>` 0x141e34b90 全反编译 | UiNav：size 立即数 **0x40**、align **8**；HistoryButton：size **0x1**、align **1**。Rust `UiNav{[UiNavLink;4]}`（repr(C)，size assert 0x40）、`HistoryButton{kind:u8}`（size assert 1）。 | **判等** |
| 16 | `link_up_nav`（0x141e4d880） | 全函数反编译 | n==0 直接返回；否则取元素 stride **0x20** 的 `+0x8` 得 ids；对第 i 项 `prev=ids[(i+n-1)%n]`、`next=ids[(i+1)%n]`；横向 slot2={3,next}、slot3={3,prev}，slot0/1 tag=0；纵向 slot0={3,prev}、slot1={3,next}，slot2/3 tag=0；未连接槽 value qword 原生未显式写（Rust 取 0，已登记 I）。Rust `link_up_nav`/`UiNav::horizontal/vertical` 同。输入为 0x20 字节元素取 `+8` vs Rust `&[u64]` 为表示差异（重建选择）。 | **判等（重建）** |
| 17 | 8× taffy setter closure（0x142375b40..0x142376300） | 8 单态逐指令 + 独立 taffy 0.3.13 编译探针 | 写面 immediate：env0 `[rsp+0x1c2]=2`、env2 `[0x1bb]=1`、env10 `[0x1bb]=0`、env11 `[0x1bd]=3`、env12 `[0x1c2]=4`、env13 `[0x1bd]=2`、undo0 `[0x1c2]=3`、undo1 `[0x1bd]=4`。Style clone 基址 `rsp+0x70` ⇒ 字段偏移 **0x152 justify_content / 0x14b flex_direction / 0x14d align_items**（独立探针：Style size 0x158，offset_of 同值）。判别：Row=0/Column=1；`Option<AlignItems>`：None=0xff、FlexStart=2、FlexEnd=3、Center=4；JustifyContent 同 niche。8 个 Rust setter 的字段/值逐项一致。 | **判等** |
| 18 | `RoofTileSsbo` 的 `SparseSsbo`（0x1412e4a90 / 0x1416191a0 / 0x141a3c070） | 三个泛型单态 fresh 反编译 + `shaders/roof_tiles_gravity.cs.hlsl` 字节偏移 | `new_storage_buffer`：`memset(map,0,0x2000)` + sentinel `[0x2000]=0x20` ⇒ **CHUNK_COUNT=0x2000**；影子 `len=0x40000`；active/dirty 位图各 **0x80** 字；空闲栈降序 `0x1fff..0`。`TypedStorageBuffer::new` 上限 **0x40000**（`0x40000 < len` panic）。`push`：槽 stride **0x40**、整块 8 qword 拷贝。着色器 `RoofShingle` 偏移 0/12/16/24/32/40/44/48/52/56/60 与 Rust `#[repr(C)]` 11 字段逐一对应。3 个布局/常量测试本次运行 **3/3 通过**。**原 wave3“RoofTileSsbo 非 SparseSsbo”core-type-gap 已闭合。** | **判等** |

## 4. 差异、GAP、FIX 与未实现

### FIX-A1（唯一需修项，latent 但属字段级不等）

- 位置：`crates/country-core/src/utils/debug_viz.rs::DebugLine::show` 的闭包 payload。
- 原生（fresh 0x140b055a0，指令 0x140b05739..0x140b0574d + build 0x14089dd50 默认分支）：
  6 qword 是两段新 `Vec` 的 `{cap, ptr, len}`：
  ```
  +0x10 payload[0] = points.cap   +0x28 payload[3] = colors.cap
  +0x18 payload[1] = points.ptr   +0x30 payload[4] = colors.ptr
  +0x20 payload[2] = points.len   +0x38 payload[5] = colors.len
  ```
  build 默认（line）分支用 `in_RDX[5]`（= payload[3]）作 `set_attribute<Vec<[f32;3]>>` 的
  cap，用 `in_RDX[6]/[7]` 作 ptr/len。
- Rust 现状：第 4 qword 写 `points.capacity()`；应为 `colors.capacity()`。
- 影响：`colors = Some(vec)` 且其长度/容量 ≠ points 时，colors Vec 的 cap 被写错
  （未来恢复 build 后会以错误 cap 重建/释放该 Vec）。`colors = None` 填充路径与
  points/colors 同长时数值恰好相等，故当前测试与既有调用点不暴露。
- 建议改法（restorer 执行，reviewer 未改 src）：
  ```rust
  let payload = [
      points.capacity() as u64,
      points.as_ptr() as u64,
      points.len() as u64,
      colors.capacity() as u64, // FIX-A1: native payload[3] = colors Vec cap
      colors.as_ptr() as u64,
      colors.len() as u64,
  ];
  ```
  并在 payload 注释中把 `{len1, ptr1, len1, cap1, ptr2, len2}` 更正为
  `{cap1,ptr1,len1,cap2,ptr2,len2}`。

### 其余差异 / GAP（均为既登记，不新增 FIX）

| 项 | 状态 | 说明 |
|---|---|---|
| `signifier_create` 的 `UiState+0x20` 配置表克隆 | **既有 GAP（restorer 登记）** | 原生逐条 clone 进持久 indexmap 再唤醒；crate 按帧流 spawn。同帧 spawn 等价，跨帧持久化语义缺。 |
| `SignifierEntityBuilder::build` mesh/shader 解析面 | **既有 GAP** | 与本 12 项无关的既有 build 缺口；show 的闭包载荷/组件插入除 FIX-A1 外已判等。 |
| placement 7/8 payload 世界点/矩形来源 | **既有 U** | 判别值与消费者 case 判等；payload 来源仍占位。 |
| `DELETION_COOLDOWN_TABLE` 19 项 | **与原生一致** | 0x143038ae8 为 19×f32 + 4B padding（+0x50 起 u64 表）；id 0..18 全覆盖。 |
| mei 三函数主体 / render_taffy 主体 | **既有未实现（本复核只判签名）** | 签名独立判等；主体按 RESTORE 诚实登记未实现。 |

## 5. 测试复跑（本次，非实现凭据，仅交叉校验）

```
cargo test -p country-core --lib components::drawable::tests     -> 4 passed
cargo test -p country-core --lib resources::roofs::roof_tile_ssbo-> 3 passed
cargo test -p country-core --lib utils::debug_viz                -> 6 passed
cargo test -p country-core --lib ui_edit_focus                   -> 5 passed
cargo test -p system-onboarding --lib                            -> 29 passed
cargo test -p toolbar --lib                                      -> 10 passed
```

## 6. 结论

unlock 轮 12 项 A 级条目（6×A2 + 6×A4）经 fresh 机器码逐项独立复核：**11 项升 C（判等）、
1 项需修（`DebugLine::show` 闭包 payload 第 4 qword，FIX-A1）**；
任务点名附加的 geo_src 跳表链、legacy_raycast 命中码、UiNav/HistoryButton、link_up_nav、
8× taffy setter、RoofTileSsbo SparseSsbo 全部判等。**FIX 清单：FIX-A1 1 条。**
