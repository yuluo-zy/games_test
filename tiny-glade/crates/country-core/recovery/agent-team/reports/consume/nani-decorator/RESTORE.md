# RESTORE — consume/nani-decorator（restorer-consume-nani-decorator，2026-10-17）

> 任务：`NaniBundle<T>` / `DrawableMeshBundle` 落库后，重建 decorator 的视觉构建
> `WindowVisualEntityBuilder::build<Transient/Persistent>`（0x1412ab270 /
> 0x1412a9b30），并核查 `decorator_visual/*` 族是否还有依赖 NaniBundle 的 spawn 面。
> 方法：每函数 fresh 反编译 + `disassemble_function`/r2 逐指令判等（硬标准 +
> 分歧检查表 8 条）。工具：ghidra `--worker shared`（只读）+ radare2（RNE 指令
> oracle，唯一 worker `restorer-consume-nani-decorator`）。身份：B patched EXE
> `cf048e27…`（ghidra `program=tiny-glade.exe`）。
>
> 原始输出：`work/restorer-consume-nani-decorator/jobs/job1..job14-result.json`、
> `disasm_1412ab270.txt`；复用 `reports/wave3/decorator/evidence/
> decomp_0x1412ab270.c` / `decomp_0x1412a9b30.c`（本轮 diff 逐字节一致）。

## 0. 结论速览

| 函数 / 子项 | 判定 | 交付 |
|---|---|---|
| `build<DecoratorTransientVisualState>`（0x1412ab270，3281 B） | **判等**（12 类分派 + 14 位条件链 + impl$5） | `src/utils/visual_entity_builder.rs` |
| `build<DecoratorPersistentVisualState>`（0x1412a9b30，3281 B，与上同体） | **判等**（仅尾部 impl$4） | 同上 |
| `impl$4::add_decorator_visual_entity`（0x1408119e0） | 判等（既有 `decorator_visual_state.rs` 已实现，fresh 复核） | — |
| `impl$5::add_decorator_visual_entity`（0x140811c30） | 判等（瞬态实体表 push；**更正 wave3 的 autoclutter 目标结论**） | 同上（sink impl） |
| `decorator_visual/*` 其他 NaniBundle spawn 面 | 无（仅 build；调用点为 clothline / generate_clothline_visuals / instantiate_blueprints） | 登记 |

`cargo check -p system-decorator`：**通过**（`Finished dev profile`，0 error；6 个
warning 均为既有 `LOG_TARGET`/`mut storage` 项）。首轮曾观察到 4 个他模块错误
（`apply_decorator_op.rs` ×3、`deferred_decorator_history.rs` ×1，来自 country-core
并发改动），在其他 owner 修复后已全绿——本批未改他模块。
`cargo test -p system-decorator --lib utils::visual_entity_builder`：**4 passed**。
`rustfmt --check`：通过。

## 1. 证据（本轮 fresh）

- 反编译：`reports/unlock/nani/job1-0x1412ab270.c` / `job1-0x1412a9b30.c`
  （各 1113 行）。两文件仅泛型名/标签地址不同（`diff` O）。
- 反汇编：ghidra `disassemble_function 0x1412ab270`（696 条）。Ghidra 流分析漏掉
  跳表 case 体（0x1412ab3fc..0x1412abb1c），由 r2 `disassemble` 在 RNE 上补齐
  （job6/job7，逐 case 独立核对）。
- 跳表：`read_memory 0x142c2b058`（31×i32，len 9..0x27）→ case 目标。
- 字符串：`read_memory` 0x142c2ae10 / 0x142c2ae33 → `nani_clutter_SolidAlbedoMap`
  `Emissive` / `nani_solid_vertex_color_window_glass`；其余名字由逐字节比较还原
  （见 §2.1）。
- `caller Location` 表 0x142c2aeb0（file=`.../visual_entity_builder.rs`，len 59；
  line/col）：各 spawn 调用点行号 270（gothic）/278（lantern）/286（cloth）/
  294/302（balcony）/310（uber）/318（default）——补强 build 的方法行号面。
- 构建器大小 0x158：`instantiate_blueprints` 0x1408289e2 `MOV R8D,0x158` +
  `clone<WindowVisualEntityBuilder>`（0x14125f250）0x1408289ec 调用。
- 调用点 ABI（**wave4 FIX-4 更正**）：0x1407e29c8（clothline）/
  0x1408289fb（instantiate_blueprints）：
  `RCX=&builder, EDX=id, R8=&AssetMeshLibrary, R9=&AssetShaderLibrary`；
  栈参 `[RSP+0x178]` = **arg6 = &mut Commands**（不是实体载荷），
  `[RSP+0x180]` = arg7 = &mut 状态表，arg5（`[RSP+0x170]`）**build 从不读取**。
  **实体由 build 内部创建**：12 个 case 各自 `Commands::spawn<NaniBundle<T>>`
  / `spawn_empty`（helper 0x14126dfe0 等内部走 `spawn_empty`），返回的
  EntityCommands 的实体字段（`[RSP+0x60]` 等）重载入 RBX，供后续 insert 与
  impl$4/impl$5 使用。此前 RESTORE 的"预分配实体 = arg6"论证作废。
- `impl$5` 调用点 0x1412ac662：`MOV EDX,ESI; MOV R8,RBX; CALL 0x140811c30`，
  `RBX` = Entity；函数体 `*(u64*)(vec_ptr + len*8) = in_R8`（步长 8）。

## 2. 判等细节

### 2.1 mesh 名 12 类分派（O）

分派键 = 构建器 `+0xb0/+0xb8`（`&str`）的长度 switch（跳表 0x142c2b058）+ 逐字节
比较。名称、实例数据类型、`NaniInstanceDesc.subset_kind`、selector：

| nani 名（len） | `NaniBundle<T>` | kind（disasm） | selector |
|---|---|---|---|
| `nani_uber` (9) | `UberInstanceData` | 23（`0x17`） | `Named(NameHash(mesh 名 +0xa0))` |
| `nani_cloth` (10) | `ClothInstanceData` | 20（`0x14`） | Named |
| `nani_smoke` (10) | `SmokeInstanceData` | 21（`0x15`） | **`AtlasPart(0)`**（`DecoratorRank::new(0)` 0x140833250 → `impl$1::from` 0x14082bd60） |
| `nani_balcony_door` (17) | `BalconyDoorInstanceData` | 22（`0x16`） | Named |
| `nani_lantern_emissive` (21) | `LanternEmissiveInstanceData` | 18（`0x12`） | Named |
| `nani_solid_vertex_color` (23) | `SolidVertexColorInstanceData` | 3 | Named |
| `nani_gothic_window_bricks` (25) | `GothicWindowBricksInstanceData` | 7 | Named |
| `nani_clutter_SolidAlbedoMap` (27) | `SolidAlbedoMapInstanceData` | 15（`0xf`） | Named + `NaniSolidTexture` |
| `nani_solid_vertex_color_window` (30) | `SolidVertexColorWindowInstanceData` | 4 | Named |
| `nani_solid_vertex_color_bricks` (30 第二支) | `SolidVertexColorBricksInstanceData` | 6 | Named |
| `nani_clutter_SolidAlbedoMapEmissive` (35) | `SolidAlbedoMapEmissiveInstanceData` | 16（`0x10`） | Named + `NaniSolidTexture` |
| `nani_solid_vertex_color_window_glass` (36) | `SolidVertexColorWindowGlassInstanceData` | 5 | Named |
| 默认 | `DrawableMeshBundle` | — | `AssetMeshLibrary::get_handle_by_name(mesh 名)`（内部 panic）+ `AssetShaderLibrary::get_raster_handle_by_name(nani 名).expect(nani 名)`（None → `expect_failed` 0x1412ac982；msg 由 RCX/RDX = 名字 ptr/len 确认） |

两个 albedo case 对 `+0x150` 做 `Option::unwrap`（r2：`movzx r12d,[rdi+0x150];
cmp r12b,9; je <unwrap_failed>`）⇒ **None 判别 = 9**，变体值域 0..=8。

### 2.2 transform（O）

- `rotation` = `+0xc0`（Quat 16 B）；`translation` = `+0xd0`（Vec3）；
  `scale` = `+0xdc`（Vec3）。
- 若 `|+0x140 Vec3| > 0`（`UCOMISS/JBE`，NaN 跳过）：
  `translation += rotation * offset`（标准 q 旋转向量公式，逐分量对照 0x1412ab317..3a6）。
- 若 `+0x30 == 1`：`scale = (+0x34,+0x38,+0x3c)`（覆盖）。
- 写入 `NaniBundle.transform` / `DrawableMeshBundle.transform` 的布局与
  `unlock/nani` 的 `write_components` 证据一致（rotation@0/trans@0x10/scale@0x1c；
  尾部 8 B padding 原样拷贝：`+0xe8` → bundle+0x28）。

### 2.3 条件组件链（14 位，O 顺序）

| # | 组件 | 条件（原版 tag） | 载荷（builder 偏移） |
|---|---|---|---|
| 1 | `ColorId`（core） | `+0xf0` bit0 | u32 @+0xf4 |
| 2 | `GothicWindowBrickDeformation` | `+0x40 == 1` | 0x3c B @+0x44..0x80 |
| 3 | `BrickColor` | **`+0x114 != 2`** | 0x18 B @+0x100..0x118 |
| 4 | `ClothlineCloth` | `+0x80 == 1` | u64@+0x84 + u32@+0x8c |
| 5 | `SemiTransparent`（core，ZST） | `+0x14e != 0` | — |
| 6 | `WindowLight` | `+0x10` bit0 | u64@+0x18 + id |
| 7 | `BalconyDoor` | `+0x14f != 0` | translation(3f32) + `rotation*Vec3::Z`(3f32) |
| 8 | `Smoke` | `+0x14c != 2` | u64(id) + u32 0 + u8@+0x14c + u8@+0x14d |
| 9 | `AlbedoMap` | `+0x20 == 1` | u64@+0x28 |
| 10 | `NormalMap` | `+0x00` bit0 | u64@+0x08 |
| 11 | `RoofFlagCloth` | `+0xf8 == 1` | u32@+0xfc |
| 12 | `WallFlagCloth` | `+0x13c != 2` | u64@+0x130 + u64@+0x134 + u32@+0x13c |
| 13 | `LanternLightParams` | `+0x118 == 1` | `rotation*dir` 3f32 + 原始 4f32@+0x11c + id + u8 0 |
| 14 | `SittableForBirds` | `+0x90 == 1` | id + u64@+0x94 + u32@+0x9c |

**分歧更正（对照 wave3 RESTORE）**：wave3 写 `BrickColor（+0x114==2）`；fresh
反汇编 0x1412abe2c 为 `CMP byte [RDI+0x114],0x2; JNZ <insert block>`，即
**!= 2 才插入**。已按机器码实现。其余 13 条与 wave3 一致。

### 2.4 状态落点（impl$4 / impl$5）

- `build<Persistent>` 尾部 = `impl$4::add_decorator_visual_entity`（0x1408119e0）：
  `HashMap<u32, PersistentVisualEntities>`，entry `+0x08` seal 位 bit0 ==1 →
  `panic_fmt`（"The persistent visuals are already sealed"）；否则 push。
  既有 `DecoratorPersistentVisualState::add_decorator_visual_entity` 已判等。
- `build<Transient>` 尾部 = `impl$5::add_decorator_visual_entity`（0x140811c30）。
  **wave3「autoclutter 表」结论不成立**（fresh 机器码）：
  - 调用点 0x1412ac662：`EDX = id, R8 = RBX = Entity`；
  - 函数体新条目初始化 + `*(u64*)(vec_ptr + len*8) = in_R8` → Vec 元素 8 B；
  - Ghidra 类型串 `Vec<DisplayChildAutoClutterInfo>` 与机器码矛盾：
    `grow_one<DisplayChildAutoClutterInfo>` 在 0x14099ca60 的实际步长是 **0x60**
    （`imul 0x60`），同一 Vec 不可能以 *8 写入；
  - 该 map 形状 = `HashMap<DecoratorId, Vec<Entity>>`，与
    `clear_out_visual_entities` 的 `Drain<DecoratorId, Vec<Entity>>` 完全一致。
  ⇒ impl$5 = `DecoratorTransientVisualState::add_decorator_visual_entity(id, entity)`；
  D1 原 `build_transient` 落瞬态表是**正确**的。Ghidra 的类型应用判为错误，
  已在本批报告中更正（按协议：不采信 reviewer 描述，只以 fresh 反编译/机器码为准）。

## 3. 实现

`crates/systems/decorator/src/utils/visual_entity_builder.rs` 重写：

- `WindowVisualEntityBuilder`：按 §2 表重建字段（transform / 法线偏移 / scale
  覆盖 / mesh 名 / nani 名 / 14 位条件字段 / `Option<u8>` solid texture）。
  `Default` 为恒等变换（scale=1）。
- `DecoratorVisualStateSink` trait：`DecoratorPersistentVisualState`（impl$4，
  seal panic）与 `DecoratorTransientVisualState`（impl$5，entry push）两个 impl。
- `build<S>`：transform 计算 → 12 类 `NaniBundle<T>` / 默认 `DrawableMeshBundle`
  → 14 位条件链 → `state.add_decorator_visual_entity(id, entity)`。
  **实体在 build 内部分派分支 `spawn_empty` 创建**（与 B 的
  `Commands::spawn`/`spawn_empty` 等价），参数面按原调用点：
  `(Commands, id, &AssetMeshLibrary, &AssetShaderLibrary, &mut State) -> Entity`；
  默认分支先做两查表（panic 先于 spawn，顺序与 B 一致）。
  `build_transient` / `build_persistent` 为薄包装（固定 id 来源，不额外 spawn）。
- 方法 `new/mark_as_preview/has_birdspot/color_id/lantern_light/custom_shader/
  albedo_map/clothline/flag_roof_cloth/flag_wall_cloth/smoke/
  mark_as_window_glass` 保留；与链字段的映射有证据者直接接线，其余标 U。
- 3 个单元测试：transform 原样 / scale 覆盖 / 法线偏移旋转叠加与零偏移跳过。

## 4. typed-gap（不编造）

| 项 | 证据 | 缺口 / 解锁 |
|---|---|---|
| 12 个条件组件类型 | PDB 名（registry：ClothlineCloth/GothicWindowBrickDeformation/BrickColor/BalconyDoor/Smoke/RoofFlagCloth/WallFlagCloth/LanternLightParams/SittableForBirds/NormalMap/AlbedoMap）+ 载荷字节数 O | country-core 未落类型；本批以 `pending_core::*` stand-in（原 PDB 名、原载荷字节数）承载。core 落库后必须删除 stand-in 并改 core 类型（TypeId 才与原版一致） |
| `NaniSolidTexture` 变体名 | 1 B enum；`Option` 以 9 为 None（r2 `cmp r12b,9`）⇒ 变体值域 0..=8 | 变体名/顺序无证据；以 `NaniSolidTexture(pub u8)` 承载裸判别，不命名变体 |
| `NaniMeshSelector` 判别序 | O：1=AtlasPart、0=Named（`impl$1::from` `mov [rcx],1`；Named 分支 `mov dword [...],0`）；core 源码枚举顺序相反（`render/nani/systems.rs:46`） | core owner 显式判别 `AtlasPart=1, Named=0`（与 `reports/consume/nani-roof-toolbar` 同一登记） |
| `NaniInstanceDesc` 模块/字段类型 | PDB `country_core::components::drawable::NaniInstanceDesc`；core 现定义在 `render::nani::systems`（重导出）；`NaniInstanceMeshDesc`（drawable）与 `NaniMeshSelector`（systems）重复 | core owner 合并；本批用 `NaniMeshSelector`（同布局语义），`impl$1::from` 以 `AtlasPart(0)` 等价表达 |
| 构建器字段名/方法到字段的映射 | 偏移与用途 O；名 U | `mesh_name/nani_name/normal_offset` 等均为语义名；`mark_as_preview`/`mark_as_window_glass`/`custom_shader` 的 build 外消费面 U |
| 包装面 `build_transient/build_persistent` 签名 | 原 build 自建实体（arg6=Commands/arg7=状态表）；D1 API 无库参数 | 已改为 `(Commands, &mut State, &AssetMeshLibrary, &AssetShaderLibrary)`，build 内部 `spawn_empty`；无外部消费者（workspace 检索确认） |

## 5. `decorator_visual/*` 其他 NaniBundle spawn 面

`get_xrefs_to` 复核：`build<Transient>` 调用点仅
`clothline`（0x1407e29e7）与 `generate_clothline_visuals`（0x1407e7006）；
`build<Persistent>` 调用点仅 `instantiate_blueprints`（0x140828a30，不在本 crate）。
这些是 build 的**消费面**，无独立 `spawn<NaniBundle<…>>`/`insert<NaniBundle<…>>`
指令。`clothline` 的 10 参面与布绳几何、`instantiate_blueprints` 的蓝图主流程均
U，本批不重写（登记）。

## 5.1 wave4 reviewer FIX 落实

| # | 处理 | 说明 |
|---|---|---|
| FIX-4（高） | **已修（文档 + 代码）** | §1 ABI 更正：`[RSP+0x178]`=arg6=&mut Commands、`[RSP+0x180]`=arg7=&mut 状态表、arg5 未读；实体由 build 内部分派分支 `spawn_empty`/`spawn` 创建。代码去掉 `entity` 入参：`build(commands, id, mesh_lib, shader_lib, state) -> Entity`，各分派分支内部建实体；默认分支两查表先于 spawn（panic 顺序与 B 一致）；包装面不再额外 `spawn_empty`。 |
| FIX-5（高） | **已修（注释/台账 + 方法落地）** | `decorator_visual_state.rs`：impl$5 说明改为瞬态实体表；删除 `_AUTOCLUTTER_IMPL5_UNIMPLEMENTED`，新增 `DecoratorTransientVisualState::add_decorator_visual_entity(id, entity)`（8 B push），sink impl 改为委托该法。 |
| FIX-1（中） | **不涉及** | 本批无容器/子项缓冲分配；装饰侧无裸指针/子项 Vec 释放点（工具容器在 toolbar 域）。 |
| FIX-2/3/6 | **不涉及** | toolbar/roof 域。 |

## 6. 编译

`cargo check -p system-decorator`：**通过**（`Finished dev profile`，0 error）。
`cargo test -p system-decorator --lib utils::visual_entity_builder`：4 passed。
`rustfmt --check`：通过。
（过程：首轮本文件 2 个 `InstanceDataMarker::default` 约束错误已修；期间另有 4 个
他模块错误来自 country-core 并发改动，按红线未改，其他 owner 修复后全绿。）

## 7. 三态回执

**完成**：`build<Transient/Persistent>` 两实例逐语句判等（12 类分派、14 位条件链、
transform、impl$4/impl$5）；`decorator_visual` 无其他 NaniBundle spawn 面。
**登记**：12 个 core 组件 stand-in + `NaniSolidTexture` 变体名 + `NaniMeshSelector`
判别序 + `NaniInstanceDesc` 模块合并 + 构建器字段名 U。
**阻塞**：无（`cargo check -p system-decorator` 通过，4 个单测通过）。

计数：**判等 4**（build×2 + impl$4/impl$5 语义）、**重建 2**（build 两实例）
+ 13 typed-gap stand-in 类型、**未实现 0**（本批范围）。
