# B 阶段 d_cb_a：`build_draw_lists` 回调前一半（gap 序号 1..=35）

O = 机器码观测，I = 推断，U = 未知。未 commit。

## 1. 结论

- `draw_lists.rs::DrawFnId::func()` 里 **前 35 个 typed gap 全部换成真实本体**（`super::draw_callbacks_a::*`），`is_recovered()` 同步登记；
  d_cb_b 负责其余 34 个（已在同一个 `draw_lists.rs` 里替换各自的行）。本范围无 `todo!/unimplemented!`。
- 新文件（全部 `rustfmt --edition 2024 --config max_width=120` 通过）：
  `draw_passes/draw_callbacks_a.rs`（回调 + 读取 + 本体，1112 行）、`draw_callbacks_a_views.rs`（视图类型，182 行）、
  `draw_callbacks_a_tests.rs`（47 个测试，1059 行）。`draw_passes/mod.rs` 追加 `pub mod draw_callbacks_a;`。
- 测试：隔离测试台全树 **634 passed / 0 failed**（含本范围 47 个；d_cb_b 的回调在台上用空桩替代，见 §7）。
  测试台改动与启动脚本：`evidence/country-core-audit/B_d_cb_a_harness.diff`。

## 2. 取值路径（O，类型名不是猜的）

回调签名 `fn(DrawBuilder, &EntityDrawParams, &mut World)`；`rcx` = builder 指针，`rdx` = `&EntityDrawParams`（`[rdx+8]` 实体），`r8` = World。
- **资源**：`Components::get_resource_id(TypeId)` → `[world+0x210/0x218]` 稀疏表 → `[world+0x1e0]` 资源槽；任一步失败共用一个 `Option::unwrap` panic
  ＝ `world.get_resource::<T>().unwrap()`（`res`）。
- **必需组件**：`world.entities` 取位置（失败 → `Result::unwrap` panic，43 B 文案）+ `Components::get_id` + `Table::get_component`
  （失败 → `Option::unwrap` panic）＝ `world.get_entity(e).unwrap().get::<T>().unwrap()`（`comp`）；**可选组件**失败 → `xor reg,reg`（`comp_opt`）；
  `get_id` + 稀疏集 `cmp [..+8*id],0; setne` ＝ `EntityRef::contains::<T>()`（`has`）。
- **TypeId → 类型名**：`get_id`/`get_resource_id` 的两个 u64 立即数在 EXE 里同时出现在 `init_state<T>` / `register_component<T>` /
  `insert_resource<T>` / `register_resource_with<…T…>` / `component_clone_via_clone<T>` / `register_event<T>` 等泛型实例体内。方法：对每个 TypeId 低 64 位
  在 `.text` 字节里搜索 → 按 `procs.tsv` 的地址区间定位包含它的过程 → 取过程名里的 `<T>`。不是靠 `type_id` 符号（PDB 里没有组件的 `type_id`）。

TypeId → 类型（本范围用到的 33 个）：

| TypeId（低 64 位） | 类型 | 种类 | 真实类型可用？ |
|---|---|---|---|
| 0xed4361819ec3a729 | `components::drawable::AlbedoMap` | 组件 | `(pub Gap)` → 视图 |
| 0x028b8ca353f65821 | `AlphaMap`（**I**：bundle `[DrawableMeshBundle, AlbedoMap, AlphaMap, NormalMap]` 剔除法） | 组件 | `(pub Gap)` → 视图 |
| 0x83098c3ec284caf1 | `components::NormalMap` | 组件 | 视图 |
| 0x0e2c5c63fb76ee91 | `components::WallFlagCloth` | 组件 | 视图 |
| 0x983a5c6eded1c4e5 | `components::RoofFlagCloth` | 组件 | 视图 |
| 0x2ff65f567dea6ff9 | `resources::color_id::ColorId` | 组件 | **真实类型** |
| 0x3b633fbe5a39b5b4 | `components::WoodenGate` | 组件 | 视图 |
| 0x73873257d5f05e11 | `components::SplashParticle` | 组件 | 视图 |
| 0xf26b1f0dd0e37102 | `components::ArrowGizmo` | 组件 | 视图 |
| 0x4782fcee8a8c578b | `components::DashedLineLength` | 组件 | 视图 |
| 0x877acce28b84e29f | `asset_libraries::Handle<Mesh>` | 组件 | **真实类型** |
| 0xe8fca894de8d2f65 | `components::SemiTransparent` | 组件（`contains`） | **真实类型** |
| 0x60556123d75da34f | `resources::critters::sheep::Sheep` | 组件 | crate 无 → 视图 |
| 0xa0da022fc84ca3d9 | `components::drawable::PrevTransform` | 组件 | 视图 |
| 0x775c6361f3425a3a | `components::StairsStep` | 组件 | 视图 |
| 0x66476a13bbcc5e41 | **U**（`components::impl$66`，候选 `BackgroundTerrain`，I） | 组件（`contains`） | stub `MarkerU66476a13` |
| 0xa6c0fc01ec67ef4c | **U**（`components::impl$25`，候选 `DontReceiveContactShadows`，I） | 组件（`Option`） | stub `MarkerUa6c0fc01` |
| 0x192600af2434483b | **U**（`components::impl$29`，f32 缺省 0.8，候选 `OverwriteRoughness`，I） | 组件 | stub `FloatU192600af` |
| 0x98df08dcfa7ea5ba | `resources::logo_animation::LogoAnimation` | 资源 | 无 `Resource` 派生 → 视图 |
| 0x5628183cc22697bc | `resources::loading_screen::LoadingScreenProgress` | 资源 | 视图 |
| 0x5eaaca67868e0174 | `Events<glade_border::ShowDemoBorderOverlay>` | 资源 | 事件类型 crate 无 → stub |
| 0x74430dfcc6d8719b | `resources::session::ActiveSession` | 资源 | 视图 |
| 0xd2f519263066413b | `resources::weather::Weather` | 资源 | 视图 |
| 0xc2b8e941c91a6428 | `systems::mode_manager::AppMode` | 资源 | 视图（判别域对不上，见 §5） |
| 0xbba451ee6a08a3ff | `systems::camera_rig::MainCamera` | 资源 | 视图 |
| 0x6efcda1c77e106b4 | `resources::editing_highlight::EditingHighlight` | 资源 | 视图 |
| 0x62dfe8f62b6fcc5 | `resources::cursor_terrain_raycast::CursorTerrainRaycast` | 资源 | **真实类型** |
| 0xae77a86a2c6cefc | `systems::wall::eraser::EraserSizeMultiplier` | 资源 | crate 无 → stub |
| 0x48b321ef779dc301 | `resources::gui::DebugGui` | 资源 | 视图 |
| 0x9311cf2688353878 | `resources::roofs::roof_highlight::RoofHighlight` | 资源 | crate 无 → stub |
| 0x850dbd51eafef756 | `resources::walls::WallHighlight` | 资源 | **真实类型**（`.ty` = `u8 @+0x10`，I） |
| 0x8226cc786e679355 | `asset_libraries::ssbo_library::AssetSsboLibrary` | 资源 | **真实类型** |
| 0xd455e4bbfd4f066e | `asset_libraries::mesh_library::AssetMeshLibrary` | 资源 | **真实类型** |
| 0xdf356893cc4e98a8 | `resources::ivy::ivy_storage::IvyStorage` | 资源 | **真实类型** |

**视图类型**：crate 里这些类型大多是 `(pub Gap)` 骨架，读不出字段，所以 `draw_callbacks_a_views.rs` 里每个类型只含被读字段（偏移写在文档里）。
与 `draw_callbacks_b` 里同名/同语义的视图（`ActiveSessionView`、`EditingHighlightView`、`DebugGuiView`、`EraserSizeMultiplier`、`RoofHighlight`、
`RoofFlagClothView`、`PrevTransformView`、`LogoAnimationView`、`MainCameraView`、`WeatherView`）字段**不是同一个子集**（例：我读 `LogoAnimation +0/+0x10`，
b 读 `+8/+0x10`；我读 `Weather +0xc`，b 读 `+4`；`MainCameraView` b 多 `season_params`），**集成者需要取并集**后让两边 `pub use` 同一份，
并让真实类型实现这些字段（或把读取点换到真实类型）。

## 3. 恢复清单（gap 序号 → 地址 → Rust 项）

每个 `read_*` 从 World 取纯数据，`*_body(b, &in, end)` 是纯逻辑（终结操作由调用者给，测试才能在 `draw()` 前读回 uniform），回调把二者接起来。

| # | 地址 | 原符号 | Rust 项 | 要点（O） |
|---|---|---|---|---|
| 1 | 0x14086a780 | `stairs_draw` | `stairs_draw_cb` | `StairsStep` 首字节 → `StairsEntityData{flags, aux:0}` |
| 2 | 0x1408a4ca0 | `indirect_draw_bricks<1>` | `bricks_lod1_cb` | 代理 3 字节 → `gate` → **先 discard 再读资源**；常规路径才读 `WallHighlight.ty` 与四个擦除高亮资源（`read_erase_highlight`） |
| 3 | 0x1408a53d0 | `indirect_draw_bricks<0>` | `bricks_lod0_cb` | 同上 |
| 4 | 0x1408dbc50 | ivy 枝 | `ivy_branch_cb` | `IvyStorage.ivies.values().points.len()` → 现有 `draw_instanced_ivy_branch_shader` |
| 5/6 | 0x1408dc180/a0 | ivy 叶 / 花（21 B 薄壳） | `ivy_leaf_cb` / `ivy_flower_cb` | `MainCamera.eye` + `AssetSsboLibrary`（`LibView`） |
| 7 | 0x1409ae900 | `closure_env$6` | `env6` | `AlbedoMap`（必需）、`AlphaMap`/`NormalMap`（可选，缺省键 0x4b2c611c0a80f895 / 0xe410c6c6901dbb51）；绑定序 4 磁盘 + 预设 0xf/5/0xe/0x13/9/10；`bool(normal.is_some())` |
| 8 | 0x1409af390 | `$27` | `env27` | `contains::<SemiTransparent>`、可选 `AlbedoMap`、`contains` U 标记（存在 → 磁盘键 0xbd39551337285fb6）、`f32` 缺省 `0.8`（0x3f4ccccd）、`Option` U 组件 → `bool(!has)` |
| 9 | 0x1409afcc0 | `$2`（羊） | `env2` | `Sheep` + `PrevTransform` → `i32/f32/bool` + `Mat4::from_scale_rotation_translation` + `SheepAnimationSsbo` |
| 10 | 0x1409b0570 | `$48` | `env48` | 四磁盘 → `draw_instanced(0x1f4)` |
| 11 | 0x1409b07f0 | `$52` | `env52` | `Events<ShowDemoBorderOverlay>` 空（`[+0x30]\|[+0x10]==0`）→ `discard`，否则磁盘 + 预设 9/10 → `draw` |
| 12 | 0x1409b0b20 | `$57` | `env57` | 拓扑 tag 1 → `state.byte0=(b&0xfc)\|2`/`blend`；`Handle<Mesh>` → `AssetMeshLibrary::get` → 网格 `+0x10`（I：`indices.len()`）写 `i32`；`DashedLineLength` 写 `f32` |
| 13 | 0x1409b0fa0 | `$45` | `env45` | 五磁盘 |
| 14 | 0x1409b12b0 | `$0` | `env0` | `state.byte0=(b&0xf3)\|4`；预设 9/10 → `WoodenPlankSsbo` → 磁盘 → `HalftimberColorIdSsbo` → 磁盘×2 → `bool/Vec3/f32`（擦除高亮三项，**不含** `DebugGui.suppress`）→ `nani_draw(WoodenPlank=0xd)` |
| 15 | 0x1409b1b00 | `$37` | `env37` | `LogoAnimation +0/+0x10` 两个 `f32`，2 磁盘 |
| 16 | 0x1409b1e90 | `$39` | `env39` | `nani_draw(LanternEmissive=0x12)` |
| 17 | 0x1409b1ee0 | `$19` | `env19` | `SplashParticle +8` → `f32` |
| 18 | 0x1409b2120 | `$32` | `env32` | 2 磁盘 → `nani_draw(BalconyDoor=0x16)` |
| 19 | 0x1409b2290 | `$41` | `env41` | `state.byte3=(b&0x9f)\|0x40`；`LoadingScreenProgress +0/+4` 两个 `f32`；1 磁盘 |
| 20 | 0x1409b2580 | `$53` | `env53` | 与 `$48` 同 4 磁盘 → `draw_instanced(0x3c)` |
| 21 | 0x1409b28c0 | `$23`（屋顶旗） | `env23` | `RoofFlagCloth.u32`、`ColorId` → `i32`×2；3 磁盘，第三张键按 `ActiveSession` 字节选（见 §4） |
| 22 | 0x1409b2f40 | `$8` | `env8` | 预设 9/10/0xf → 磁盘 → 预设 0x16 → `nani_draw(PreplacedFlower=0xe)` |
| 23 | 0x1409b3220 | `$28` | `env28` | `nani_draw(Uber=0x17)` |
| 24 | 0x1409b3270 | `$16` | `env16` | `state.byte0=(b&0xf3)\|8`；Dormer SSBO×2、`bind_wall_color_customization`、屋顶磁盘、`RoofColorIdSsbo`、`RoofHighlight` 两个 `i32`、石/木地板、`HalftimberColorIdSsbo`、磁盘 → `nani_draw(RoofFloor=9)` |
| 25 | 0x1409b3780 | `$11` | `env11` | 第二张磁盘键按 `ActiveSession` 变体字节：`==3`→0x34c237b9c8c9523d，`==4`→0x85351b1262eaaf04，否则 0xdb21398409f34377；预设 9/10/0xf、`BushSsbo` → `nani_draw(Bush=0xc)` |
| 26 | 0x1409b3be0 | `$20` | `env20` | 预设 0x13 → `draw` |
| 27 | 0x1409b3cc0 | `$10` | `env10` | 预设 9/10/0xf、`FlowerSsbo` → `nani_draw(Flower=0xb)` |
| 28 | 0x1409b3eb0 | `$55` | `env55` | `state.byte3` 同 `$41`；`ArrowGizmo` **+4 先、+0 后** 两个 `f32` |
| 29 | 0x1409b4180 | `$33` | `env33` | 1 磁盘 → `nani_draw(Cloth=0x14)` |
| 30 | 0x1409b4270 | `$17` | `env17` | 同 `$16` 的绑定序（无 `RoofHighlight` uniform）→ `nani_draw(StairsFloor=0x1a)` |
| 31 | 0x1409b4600 | `$40` | `env40` | 预设 0x13/5 → 磁盘 0x84a0…（同 stairs/rocky）→ 预设 0x15 |
| 32 | 0x1409b4860 | `$22`（墙旗） | `env22` | `WallFlagCloth`（`f32@4/8/0xc`、`bool@0x10`、`i32@0`）+ `ColorId` → 6 个 uniform；3 磁盘 |
| 33 | 0x1409b5140 | `$3`（天气粒子） | `env3` | `Weather+0xc`：`!(x > 0.0)`（`ucomiss; jbe`，含 NaN）→ `discard`（此时**不再读** `AppMode`/`MainCamera`）；否则 `f32`、`bool(AppMode tag==0x13)`、`bool(MainCamera+0x4d0==4)`、`DofAutofocusSsbo` → `draw_instanced(0x2ee0)` |
| 34 | 0x1409b55e0 | `$30` | `env30` | 预设 9/10、磁盘 → `nani_draw(SolidAlbedoMapEmissive=0x10)` |
| 35 | 0x1409b57c0 | `$14`（木门） | `env14` | `WoodenGate`：磁盘 → `i32(q18 低 32 位)` → `GateColorIdSsbo` → 磁盘×2 → `i32(+0x30)` |

## 4. 值得注意的机器码细节（O）

- **会话字节选键（`$22`/`$23`）**：`ActiveSession` 的字节序列（`ptr@+8, len@+0x10`）上 `cmp byte,1; ja` 循环——**存在 `<= 1` 的字节**选 `0x4c58d4d61ec450d2`，
  否则（含空序列）选缺省 `0xe52375bc32a964a4`。这与 `water_planes_draw` 的「无 0 字节」扫描是**不同**的判定（旧文档把二者混为一谈的风险，请注意）。
- **`$16`/`$17` 的 `RoofHighlight`**：`edi = tag != 0 ? value : 0`，第二个 `i32` = `tag`；两者的绑定调用序列完全相同（测试断言），`$16` 只多两个 uniform。
- **砖回调的 discard 先于资源读取**：LOD0 `mode>=2 || b1==1`、LOD1 的 `(bit, sel)` 判定都在 `AssetSsboLibrary` 取值之前，所以回调先 `gate` 再读资源（否则会把原版的 discard 变成 panic）。
- **`state` 位**：byte0 `(b&0xf3)|v`（`$0` v=4；`$16/$17` v=8）；byte3 `(b&0x9f)|0x40`（`$41/$55`，即 `RasterState::set_byte3_field_2`）；拓扑 tag 1（`$57`）与 `topology_draw` 同。
- **`$17` 的 ABI**：builder 被分段拷贝（qword + 0xa0 B + byte `+0xa8` + 余下），那只是编译器对 `state.byte0` 改写的展开，参数仍是 `rcx = &builder`。
- **`$52`** 的资源是 `Events<ShowDemoBorderOverlay>`（`Vec` 长度在 `+0x10/+0x30` ＝ `events_a`/`events_b`），不是事件类型本身；`Events::is_empty()` 等价。
- 带符号 `movabs` 立即数全部用测试按 `-0x… as u64` 复核（`negative_movabs_constants_match_the_machine_code`）。

## 5. O / I / U

**O**：全部绑定序、预设号、磁盘键/名、uniform 名与类型、SSBO 绑定名与 `SsboId = xxh3(槽名)`（测试复算）、`nani_draw` 的 kind 立即数、`state` 位运算、discard 条件、
panic 位置（Result/Option unwrap）、`Mat4` 由 `Transform{rot@0, trans@0x10, scale@0x1c}` 组成（SIMD 展开逐项核对为 `from_scale_rotation_translation`）。

**I**：`AlphaMap` 类型名（bundle 剔除）；`WallHighlight.ty` 即 `u8 @+0x10`；`Mesh +0x10` 即 `indices.len()`（`$57`）；`Ivy` 的 `usize@+0x20` 即 `points.len()`；`Sheep +0x28`
是 `u64` 截断成 `i32`（机器码 `mov r9, qword`）；三个 U 组件的候选名；`WoodenGate +0x18` 同理。

**U**（未编造，留 stub / 视图并在此列出）：
1. 三个 TypeId 的类型名：`0x66476a13…`、`0xa6c0fc01…`、`0x192600af…`（只知同属 bundle `[KeepOnLevelUnload, DrawableMeshBundle, BackgroundTerrain, DontReceiveContactShadows, OverwriteRoughness, …]`）。
2. 各视图字段的**语义名**（偏移已知，名字 U），以及 `AppMode` 的 `tag == 0x13` 对应哪个变体：crate 里的 `AppMode` 只有 5 个变体，判别域对不上，故用 `AppModeView{tag:u32}`。
3. `Weather +0xc` 的语义（我叫 `intensity`，只凭 `> 0` 才画粒子）；`MainCamera +0x4d0 == 4` 的语义（b 称 season，I）。
4. `WallBrickInstancedDrawProxy` 三个字节与真实类型 `{kind: u16, sub: u8}` 的对应关系（字节 `b0@0,b1@1,b2@2` 是机器码事实，对应方式 U）。
5. `StairsEntityData.aux`（`stairs_draw` 里 `aux = 0` 仅覆盖组件在表里的路径；稀疏集路径的 `aux` 语义 U，沿用 d_shaders 的结论）。
6. `nani_draw` 的 `RenderGraph`：crate 里是占位类型，`nani_draw_world` 用一次性 `RenderGraph::default()`（I：`get_rg_buffer` 命中已导入句柄时不碰它；与 d_cb_b 的实现同形）。

## 6. 对公共 API / 共享文件的改动

- `draw_passes/mod.rs`：追加 `pub mod draw_callbacks_a;`。
- `draw_passes/draw_lists.rs`：只改我负责的 35 行 `DrawFnId::X => gap::<0x…>` → `super::draw_callbacks_a::…`；`is_recovered` 的 `matches!` 里补 35 个变体；
  测试 `recovered_bodies_flag` 里 `!Env6.is_recovered()` 改为 `Env6.is_recovered()`（该断言的前提「Env6 是 gap」已不成立）。`gap::<ADDR>` 现在没有调用者了（两个 agent 都替换完），
  集成者可删；模块头文档里「其余是 typed gap」的措辞也已过时。
- 没有改 `lib.rs` / `rhapsody_types.rs` / `rhapsody_contract` / `draw_builder.rs`（本轮没有追加需求）。
- 与 d_cb_b 重复的小件：`nani_draw_world`、`LibView`、`res`/`comp`、`cull_state`、六个 `*View`（见 §2 末尾）——**集成者去重**。

## 7. 测试

隔离测试台 `/tmp/h_d_cb_a`（以 `/tmp/h_d_gap2` 为底；本轮已删，改动见 `B_d_cb_a_harness.diff`：加挂 `draw_callbacks_a`、`indirect_params.rs`，
给 `Gap` 补 derive，`render::nani` 里放 `GpuWorld`/`NaniDrawExt` 的**桩**（记录 kind，不是真实实现），`components::SemiTransparent`、`resources::{ivy_storage,walls::WallHighlight}`
的形状相同的桩；d_cb_b 的 `draw_callbacks_b` 在台上是 34 个空函数）。台上 `draw_bricks.rs` 用一份补了 `DrawIndexedIndirectBuffer` 导入的拷贝（真实文件的 tests 模块缺这条导入，不属于本范围）。
```
/tmp/h_d_cb_a/go.sh          # 全树 → 634 passed; 0 failed
/tmp/h_d_cb_a/go.sh draw_callbacks_a   # 本范围 47 个
```
覆盖：
- **每个回调**至少一条：绑定调用序（`RecHost` 记录预设/磁盘/SSBO 顺序）、终结操作种类（`draw` / `discard` / `draw_instanced(n)` / `multi_draw_indexed_indirect`）、`state` 位、缺资源/缺组件 panic 文案；
- **uniform 值**在 `draw()` 前读回（`Env2` 的 `Mat4` 逐元素对 `from_scale_rotation_translation`；`Sheep`/`WoodenGate` 的 qword→`i32` 截断；`Env27` 的缺省 `0.8`；`Env3` 的 NaN/负值/零都 discard 且不读后续资源；`Env16` 的 `tag==0`）；
- 反面分支：砖回调 discard 先于资源读取、`Env52` 无事件、`Env6` 无可选贴图的缺省键、`Env11` 的变体 3/4/其它、`cloth_key_c` 的空序列/全 `>1`/含 0/含 1；
- nani 小闭包：`kind` 常量逐一对立即数、`*_binds` 前缀序列、**缺 `GpuWorld` 时 11 个回调全部在 `nani_draw` 处 `Option::unwrap` panic**（说明绑定先于 nani，且资源访问路径通到了）；
- SSBO 标签：6 个新标签 `SsboId == xxh3_64(槽名)`；
- 接线：35 个 `DrawFnId` 的 `func()` 指针与本文件函数一致且 `is_recovered()`。

**未覆盖**（诚实列出）：真实 `NaniDrawExt::nani_draw` 在台上是桩，nani 之后的间接绘制路径由 `draw_ext.rs` 自己的测试负责；真实 `GpuWorld` 在台上不存在，所以没有「`GpuWorld` 在场时成功走通」的集成用例；
`ivy_leaf_cb`/`ivy_flower_cb`/砖回调的成功路径依赖 `AssetSsboLibrary` 里注册实例 SSBO（需要 `GraphicsDevice` 替身），只测了 discard/缺资源分支，成功路径由 `draw_bricks`/`instanced_ivy` 自己的测试覆盖。

## 8. 遗留 / 建议

1. 集成者：合并两份视图类型（并集）、去重 `nani_draw_world`/`LibView`/`res`/`comp`；让真实 `ActiveSession`/`Weather`/`EditingHighlight`/`DebugGui`/`MainCamera`/`LogoAnimation`/`LoadingScreenProgress`/`RoofFlagCloth`/`WallFlagCloth`/`WoodenGate`/… 暴露被读字段后，把 `res::<*View>` 换成真实类型。
2. `resources::roofs::roof_highlight::RoofHighlight`、`systems::wall::eraser::EraserSizeMultiplier`、`glade_border::ShowDemoBorderOverlay`、`critters::sheep::Sheep` 在 crate 里不存在（有 TypeId 与 PDB 名，证据确凿），需要由各自 owner 建立。
3. 三个 U TypeId 的类型名：可从 `BackgroundTerrain`/`DontReceiveContactShadows`/`OverwriteRoughness` 三个候选里，用各自 `register_required_components` 的实现体（`components::impl$25/$29/$66`）里的 `type_name`/注册顺序进一步证实。
4. `draw_callbacks_a.rs` 1112 行（超过 800 行建议值）：已把视图与测试拆出；剩余是 35 个回调的常量表，不再拆是为了让「地址 → 常量」同处便于对照。
