# B 阶段 d_shaders：`draw_passes/draw_shaders/*` 与 `render_loop/resources/critters.rs`

范围：`reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/` 全部文件 + `systems/render_loop/resources/`。
PDB 前缀 `country_core::systems::render_loop::draw_passes::draw_shaders`（共 104 个过程，归并后 ~32 个函数体）。
O = 机器码观测，I = 推断，U = 未知（未编造）。方法：`tools/country-core-audit/dis.py` 反汇编 + 自写的“立即数/调用实参模拟器”
`dsh_emu.py`（逐调用点打印 `rcx/rdx/r8/r9/[rsp+0x20]` 的立即数，所有哈希、预设号、文件键都是脚本抽取，非手抄）+ `dsh_ssbo_info.py`
（从 25 个 `read_storage_buffer<T>` 实例抽槽名与 `SsboId`）+ `dsh_xxh3_name_search.rs.txt`（把 EXE 全部可打印串按 `xxh3_64` 反查名字）。

## 1. 过程 → Rust 项

| 地址 | 原符号 | Rust 项 | 要点（O） |
|---|---|---|---|
| 0x1409cacb0 | `floor_shader::draw_floor_shader` | `floor_shader::draw_floor_shader(b, flag)` | `state byte0 = (b&0xf3)\|8`；预设 9/0xa/6/0x13 → 磁盘 0x4a2b… → `bind_stone_floor_textures` → `bind_wall_color_customization` → 3 磁盘 → 预设 0x15/7/0xc/0xe/0x14 → 磁盘 0x954b… → `write_uniform<bool>(0x0be4740b…)` → `draw()`；ABI 第 4 参数 `r9b`=bool，`rdx/r8` 未用 |
| 0x1408e5a10 | `path_pebble_draw` | `path_pebble_shader::path_pebble_draw` | `read<PebblePlacement>` → 磁盘 0x84b5…（与 floor 同文件）→ `multi_draw_indexed_indirect(3,1,0)` |
| 0x1409cb4e0 | `rocky_terrain_shader` | `rocky_terrain_draw::rocky_terrain_shader(b, flag, vertex_count)` | `state byte0 = (b&0xf3)\|4`；Vertex/Triangle SSBO → 磁盘 0x84a0… → 预设 0xc → bool uniform（与 floor 同名）→ `draw_no_index_buffer(u32 资源)` |
| 0x140a8a090 | `leaf_particle_draw` | `leaf_particle_draw::leaf_particle_draw(b, count)` | 资源 `u64@+0`=粒子数；SSBO → 磁盘×2 → `draw_instanced(count)` |
| 0x140b4af70 / 0x140b4b1d0 | `draw_grass_shader` / `draw_grass_lod1_shader` | `grass_shader::{draw_common, draw_grass_shader, draw_grass_lod1_shader}` | 预设 9/10/8 → GrassBlades、AtomicCounter → `write_uniform<i32>(0xb06e…, 0x90000)`；LOD0 `multi_draw_indexed_indirect(0,1,0)`，LOD1 **非索引** `multi_draw_indirect(1,0)` |
| 0x1409d9bc0 | `water_planes_draw` | `water_planes_draw::water_planes_draw` | 进程布尔守卫→`discard`；`state byte3 &= 0x9f`；绑定序见模块注释（9/10 与磁盘 0x579f… **各绑两次**，O）；`write_uniform<Vec3>(0x6f71…, water_color)`；`bind_ray_tracing`（闭包）；`variant==4` 加雪纹理 0xc555…；`draw()` |
| 0x1409da580 / 0x1409da760 / 0x1409da940 | `lilypad_draw` / `lilypad_flower_draw` / `reed_draw` | 同名 | SSBO → 9/10 → `bind_water_object_push` → 磁盘（reed 无）→ `multi_draw_indexed_indirect(8 / 0xf / 9, 1, 0)` |
| 0x14086a780 | `stairs_draw` | `stairs_draw::stairs_draw(b, StairsEntityData)` | SSBO → 磁盘 0x84a0… → `i32 0x9c40` → `bool flag` → 预设 0xc → `multi_draw_indexed_indirect(flag+1,1,0)` |
| 0x1408a53d0 / 0x1408a4ca0 | `indirect_draw_bricks<0/1>` | `draw_bricks::indirect_draw_bricks<LOD>` + `gate` / `bricks_mode` / `lod1_mode_bits` | 见 §3 |
| 0x1408dbc50 | `draw_instanced_ivy_branch_shader` | `instanced_ivy::draw_instanced_ivy_branch_shader` | 实例数 = Σ `saturating_sub(段数,1)`（对 0x98 B 桶的 hashbrown 扫描） |
| 0x1408dc1c0 / 0x1408dc180 / 0x1408dc1a0 | `ivy_leaf_shader` + 两个 21 B 薄壳 | `ivy_leaf_shader(b,is_flower,eye,key,lib)` / `draw_instanced_ivy_{leaf,flower}_shader` | 薄壳只塞 `(is_flower, 文件键)` 后 `jmp`；Vec3 eye → 预设 9/10/5 → 磁盘 → bool → SSBO → 库里取实例数 |
| 0x1415547e0 | `draw_instanced_butterfly_shader` | 同名 | 9/10 → Butterfly → 磁盘 → `draw_instanced(0x18)` |
| 0x1415549b0 / 0x141554cf0 | firefly / glow | 同名（`intensity` 为 0 → `discard`） | `write_uniform<f32>(0x075a…)`；glow 另读 Visibility、Dof；`draw_instanced(0x60)` |
| 0x141555080 | `draw_duck_shader` | `draw_duck_shader(b,&DuckInputs)` | Mat4 + 4 个标量 + 两个 `i32`（`state<2`、`state==3`）→ `draw()` |
| 0x141555ec0 | `draw_wren_shader` | `draw_wren_shader(b, nani)` | 9/10 → 磁盘 → `nani_draw(Wren=0x11)` |
| 0x1409c9830 / 0x1409ca350 | `tree_trunk` / `tree_canopy` | `clearing_shaders::{tree_trunk, tree_canopy}` | 模式字节≠0 → `state=(b&0xf3)\|4`；6/5 张磁盘纹理；`canopy_factor`、`blend_value` 公式；`nani_draw(TreeTrunk=1 / Canopy=0)` |
| 0x1409d9070 / 0x1409d96a0 / 0x1409d98a0 | `instanced_roof::*`（**上一版缺这个文件**） | 新建 `instanced_roof.rs` | 瓦：Dormer×2、RoofColor、4 磁盘、RoofTile、两个 `i32` uniform、`nani_draw(RoofTile=8)`；尖顶/梁：库里取实例数后 `draw_instanced` |
| 0x141631f90 | `bind_water_object_push<DrawBuilder>` | `common_includes::bind_water_object_push` | 预设 0xf → Ripple SSBO → 预设 0x18 |
| 0x14160e330 / 0x14160e630 | `bind_water_object_push<ComputePass>` / `bind_garden_exclusion<ComputePass>` | `common_includes_compute::{bind_water_object_push_compute, bind_garden_exclusion_compute}` | 访问码 0x19；哈希与 `c2_support.rs` 另一份实现逐一相同 |
| 0x140ae8070/80, 0x14085b8a0/b0, 0x140903c90/a0 | 三组 `build` + `startup_*` | `billboard_forest` / `clouds` / `stars` + 共用 `startup_quad` | 索引 `[0,1,2,2,1,3]`；常量见 §4 |
| 0x1412fa8e0 | `from_iter<u32, FlatMap<TupleWindows<RangeInclusive<u32>…>>>` | `startup_quad::{quad_window_indices, quad_indices}` | 闭包 `(a,b)→[2a,2a+1,2b,2b,2a+1,2b+1]`，范围 `0..=1`；ICF 后三者共用 |
| 0x14161b0f0 / 0x14161f1e0 / 0x14161fea0 / 0x14160f920 | `new_storage_buffer<Butterfly/Firefly/FireflyVisibility/SheepAnimation>` | `resources::critters::*` | 元素布局、`NAME`、`BUFFER_LEN`、缺省元素，见 §2 |
| 0x141dbcb20 / 0x141dbd070 | `Map<QueryIter<(Transform, Butterfly/Firefly)>>::next` | `ButterflySsbo::from_components` / `FireflySsbo::from_translation` | `forward = rotation * -Z`，其余按 `Transform` 0x30 B 布局 |
| `Plugin::{name,ready,finish,is_unique}`、`as_any*`、`type_id`、`add_to_app<FeedbackReporter…>` | bevy 胶水 | 不恢复 | 无业务逻辑；`FeedbackReporterPlugin::add_to_app`（0x1405b0140）不属本模块 |
| `read_access_buffer/write_access_buffer/read_storage_buffer/write_storage_buffer<critters::*>`、`AssetSsboLibrary::{add_new,get_rg_buffer}<critters::*>`、`TypedStorageBuffer::new_storage_buffer`、`cpu_write`、`from_iter<ButterflySsbo/FireflySsbo>` | 泛型单态化 | 由 A3/A4/ssbo 的泛型实现覆盖；本模块提供类型 | `cpu_write<Butterfly/Firefly>`（0x14230edb0/0x14230f1e0）未单独核对（I：同 `MappedStorageBuffer::cpu_write` 泛型） |

## 2. SSBO 类型标签与 critter 元素（`ssbo_tags.rs`、`resources/critters.rs`）

- `read_storage_buffer<T>` 有 25 个实例；每个只含两样常量：槽名（`rustc_entry` 键）与 `SsboId`（`get_rg_buffer<T>` 立即数）。
  `ssbo_tags.rs` 为每个 `T` 给出零大小标签：槽名/类型名/`SsboId` 由脚本从 EXE 抽取，**测试用真实 `xxhash-rust` 复算
  `xxh3_64(槽名) == SsboId`，22 个标签 + 两个间接缓冲全部通过**（因此“槽名 = `StorageElement::NAME`、`SsboId = xxh3(NAME)`”不再是推断）。
  `DrawIndirectBuffer`/`DrawIndexedIndirectBuffer` 的 id 也等于 `xxh3("DrawIndirectBuffer"/"DrawIndexedIndirectBuffer")`。
  `BrickTransformSsbo` 的槽名是 "BrickTransforms"（全串反查所得），不是类型名。注意 `RoofSpireSsbo` 的槽名在原版里拼成 "RoofSpearTransforms"。
- critter 元素（O）：
  | 类型 | 槽名 / `SsboId` | 元素 | `BUFFER_LEN` | 缺省 |
  |---|---|---|---|---|
  | `ButterflySsbo` | "Butterfly Buffer" / 0x61619a2e1d85da94 | 32 B：`position`、`anim_t`、`forward`、`pad` | 24（0x300 B） | 位置 0、`forward=(-0,-0,-1)` |
  | `FireflySsbo` | "Firefly Buffer" / 0x29d4a64d5ea39871 | 16 B：`position`、`pad` | 96（0x600 B） | 全 0 |
  | `FireflyVisibilitySsbo` | "Firefly Vis Buffer" / 0x9248897309ea4fea | `f32` | 96（0x180 B，`memory=4`） | 全 0 |
  | `SheepAnimationSsbo` | "Sheep Animation" / 0xc70858c12c64d9ba | 0x3e0 B（62×float4，`imul 0x3e0`） | 0（随数据，`TypedStorageBuffer`） | — |
  `draw_instanced_*` 的实例数正是 `BUFFER_LEN`（0x18 / 0x60）。`SheepAnimationSsbo` 完整实现了库侧 `SsboType`；
  三个映射缓冲只给 `StorageElement` + A4 `SsboType` + `DrawSsbo`（原因见 §5 A3 缺口）。

## 3. 关键发现（对 A3/A4 的纠正）

1. **`DrawBuilder +0xf0` 不是 `bool`**：`indirect_draw_bricks` 里 `cmp byte ptr [rcx+0xf0], 2`、`0x1c >> v`、`0x201000000 >> 8v`
   说明它是 0..=4 的小枚举（`bind_common` 写 `CommonDrawParams +4`，其 `test r14d, r14d` 只是“非 0”）。
   A4 的 `common_flag: bool` 会把 2..4 截成 `true`。我没有改 A4：`common_includes::DrawModeExt::draw_mode()` 暂时取 `common_flag() as u8`，
   `draw_bricks` 的判定表已按完整字节写成纯函数 `gate::<LOD>(mode, proxy)` 并单独测试，`indirect_draw_bricks_with_mode` 可显式传 mode。
   建议 A4 把字段改成 `u8`（并让 `CommonDrawParams.flag` 同步），届时 `draw_mode()` 一行即可对正；`bind_common` 里 `write_uniform<u32>` 的值也应是该字节而非 0/1。
2. **`AssetSsboLibrary` 虚表槽 +0x18 不是 `buffer()`**：`ivy_leaf_shader`/`roof_spire`/`roof_support_beam` 把它的返回值 `[rax]` 当实例数，
   `indirect_draw_bricks` 把同一返回值 `+0x68` 当 `&ManagedHandle`；两者都吻合 `InstanceBuffer<T>{instance_count, buffer}`（缓冲内句柄在 `+0x60`）。
   即该槽 = “取出 `InstanceBuffer`，非实例缓冲则为空”。A3 `Ssbo::buffer()` 返回 `&self.buffer`（在 `+8`），对不上。
   我在 `ssbo_tags` 里把这个语义放成边界 `SsboInstanceView::lookup(ssbo_id) -> Option<(state.name, Option<InstanceBufferView>)>`，
   并写了 `instance_buffer/instance_count`（名字不符 → `assert_failed`，未注册或非实例缓冲 → 0 / None）。A3 需要给 `AssetSsboLibrary` 实现该 trait。
3. **`multi_draw_indexed_indirect` 的静态表有 24 项**（0x142aaaa78，第 24 项起是别的数据），A4 只列了 10 项，而本模块调用点用到下标 3/8/9/0xf/0x11..0x17。
   我已**直接改了 `draw_builder.rs` 里的常量**（数据-only，O：`INDEXED_INDIRECT_INDEX_COUNTS: [u32; 24]`，第 10、11 项在 EXE 里就是 0）——这是对 A4 文件的唯一一处写入，请集成者确认。
4. **`bind_resources` 的载体 `rg_buffer_hash` 用的是绑定名哈希**（`carrier!` 宏），而 `read_storage_buffer` 需要的是 `SsboId = xxh3(NAME)`；
   例如 `WallColorIdSsbo` 现在返回 0xe9701a257aa8828c（那是 `bind_ray_tracing` 的 uniform/绑定名）。`floor_shader`/`draw_bricks` 的测试因此用 `rg::<WallColorIdSsbo>()` 取值而不写死。
5. 原 `crate::resources::critters::{ButterflySsbo, FireflySsbo, FireflyVisibilitySsbo}`（WP18 的占位）与本模块重复；PDB 里 `read_storage_buffer<…>` 的类型路径是
   `systems::render_loop::resources::critters::*`，应以本模块为准，`resources/critters.rs` 里的同名占位可删。

## 4. O / I / U 清单

**O（机器码观测）**：上表全部绑定顺序、预设号、文件键、名字哈希、访问码 0x1d、`state` 位运算（floor `&0xf3|8`；rocky/tree/roof/bricks LOD0 `&0xf3|4`；LOD1 `&0xf3`；water byte3 `&0x9f`）、
`write_uniform` 的类型（i32/bool/f32/Vec3/Mat4）与名字、`multi_draw*` 的下标与 `arg_50=1,arg_58=0`、`draw_instanced` 的实例数、`discard` 的触发条件、`nani_draw` 的 kind（Canopy 0 / TreeTrunk 1 / RoofTile 8 / Wren 0x11）、
三个 startup 的名字/路径/长度/半径常量/`DesignatedRenderPass=0x10`（billboard 没有）、网格名“billboard forest”（空格）≠ 库名“billboard_forest”（下划线）。

**I（推断）**：`bind_terrain_heightmap/lighting` = 预设 9/10；预设 8、5 等的语义；`state byte0` 的两位是“剔除模式”；`stairs_draw` 里实体取值的 `aux`（实体侧 hashbrown/稀疏集路径）；
`water_planes_draw` 的进程布尔来源；`WATER_WINTER_VARIANT = 4` 是冬季；`0x90000` 的含义。

**U（未知，未编造）**：各世界资源的类型名（只有 TypeId 双 u64，写在各文件模块注释里：0xd2f519…、0xa21277…、0x850dbd…、0x3dce36…、0x9311cf…、0xfa4453… 等）；
预设与磁盘纹理的真实名字；`DuckInputs` 三个组件各自是什么类型；`StartupBundle.flag`（一个恒 false 的 bool）；mesh 32 B 常量的第一字语义（半径 85.28/84.64/87.09，I：远裁剪）；
`indirect_draw_bricks` 的 `brick_mode` 资源、代理字节 `b0/b1/b2` 的名字。

## 5. stub / 缺口清单

- `ssbo_tags.rs`：`DrawSsbo`、`SsboInstanceView`、`InstanceBufferView`（见 §3.2）；标签类型替代尚无 `SsboType` 实现的真实 SSBO 类型。真实类型就绪后只需把调用点的标签换成真实类型（`rg_buffer_hash` 必须相同，测试保证）。
- `startup_quad.rs`：`StartupHost`（`load_mesh_into_library` 0x140b1aca0 / `load_shader_into_library` 0x140b3fe40 / `Commands::spawn_empty`）、`AddStartupSystem`（`SubApp::add_systems<Startup,…>`）。
  crate 没有 `bevy_app`，所以 `Plugin::build` 以 `build(&self, &mut impl AddStartupSystem)` 表达；`startup/mod.rs` 的 `load_mesh_into_library` 仍是 `todo!`（不在本范围）。
- `nani_draw`：`clearing_shaders` / `instanced_roof` / `draw_wren_shader` 以闭包 `FnOnce(DrawBuilder, GpuWorldSubsetKind)` 接 `NaniDrawExt::nani_draw`（它需要 `&GpuWorld/&AssetSsboLibrary/&mut RenderGraph`，由调用方闭包捕获）。
- `water_planes_draw`：`bind_ray_tracing<DrawBuilder>`（0x1416289d0，4167 B，`bind_resources` 的 DrawBuilder 实例）A4 尚未提供，以闭包参数传入。
- `bind_brick_erase_highlight`：A4 现已实现，`draw_bricks` 直接调用（四个世界资源值经 `BrickEraseHighlightInputs` 传入）。
- A3 缺口：`render::ssbo::MappedStorageBuffer` 固定 0x40000 B / 对齐 1；而 `new_storage_buffer<Butterfly/Firefly/FireflyVisibility>` 是 `BUFFER_LEN×size_of::<T>()` 字节、对齐 4、`memory` 2（Vis 为 4）、
  `None` 时写缺省元素（蝴蝶是常量展开，萤火虫 `memset 0`）；`>BUFFER_LEN×size+0x1f` 的初始数据 panic（`cmp rbx, 0x320`）。`MappedStorageBuffer` 的 `_marker` 私有，外部无法按此构造，所以没给这三个类型库侧 `SsboType`。
- 世界取值：函数以参数接收“从 World 读出的值”（`vertex_count`、`intensity`、`DuckInputs`、`TreeTrunkInputs`、`BrickEntity` 等），其来源（`get_resource_id` 的 TypeId）写在各函数文档里；
  `EntityDrawParams`（A4 `draw_list.rs`）没有 `entity` 访问器（O：`[rdx+8]` 是实体），所以 `stairs_draw`/`indirect_draw_bricks` 取已解析的数据而不是 `&EntityDrawParams`。

## 6. 对公共 API 的改动（旧 → 新）

旧骨架里全部是 `unimplemented!` 且返回 `DrawBuilder`；机器码里 `draw_*` 是终结操作，所以都改成返回 `()`、参数为 `DrawBuilder<'p>`（A4 新签名）。具体：
`draw_floor_shader(b)->DrawBuilder` → `(b, flag: bool)`；`path_pebble_draw(b, &Ssbo)->DrawBuilder` → `(b)`；`draw_grass_*`、`draw_common`、`water_planes_draw`、`lilypad_*`、`reed_draw` 同理去掉 SSBO 引用参数
（SSBO 由类型标签表达）；`indirect_draw_bricks<LOD>(b,&proxy,&drawlist,&roofcolors,&Gap)->DrawBuilder` → `(b, BrickEntity, &dyn SsboInstanceView, brick_mode, &BrickEraseHighlightInputs)`；
`common_includes::bind_*<T>(pass)->T` → 具体 `DrawBuilder` 版 + `*_compute` 版；三个 plugin：`BillboardForestPlugin`→`BillboardForest`、`CloudsPlugin`→`CloudPlugin`（与 PDB 名一致），`build(&mut self,&mut Gap)`→`build(&self,&mut impl AddStartupSystem)`；
`startup_*` 的参数 `(gpu, meshes, mesh_watch, shaders, shader_watch, commands)` → `startup_*<H: StartupHost>(&mut H)`。`resources/critters.rs` 的四个 `(Gap)` 载体 → 真实元素结构。以上这些符号当前在 crate 里**没有其它使用者**（grep 过）。
新增文件：`ssbo_tags.rs`、`startup_quad.rs`、`common_includes_compute.rs`、`instanced_roof.rs`、`test_support.rs`（`cfg(test)`）。`mod.rs` 已登记；没有改 `lib.rs` / `rhapsody_types.rs` / `rhapsody_contract`。

## 7. 测试

隔离台 `/tmp/h_d_shaders`（以 `h_a4_compute` 为底，挂载真实的 `draw_stream/draw_builder/framebuffer_binding/pass_resource_access/compute_pass_builder/ssbo/ssbo_library/name_hash/…`，
把假的 xxhash 换成真实 `xxhash-rust 0.8.17`、加 `itertools 0.14`、`GpuWorldSubsetKind` 换成与真实枚举同名同值的 27 项）：

```
export PATH=$HOME/.cargo/bin:$PATH CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0
L0H_REAL=$PWD/reconstruction/country-core/src L0H_WORK=/tmp/w_d_shaders CARGO_TARGET_DIR=/tmp/l0h-target /tmp/h_d_shaders/run.sh
```
结果：**481 passed / 0 failed**（其中本范围 92 个：billboard 3、clearing 8、clouds 2、common_includes 4、draw_bricks 13、floor 4、grass 5、critters 7、ivy 5、roof 5、leaf_particle 2、pebble 2、rocky 3、ssbo_tags 5、stairs 3、stars 2、startup_quad 3、water 11、`resources::critters` 5）。
`common_includes_compute.rs` 在同一台里编译通过（真实 `ComputePass`），未做行为测试（`ComputePass::new` 需要完整设备替身，c2 的 `c2_support` 里有同形实现与测试）。
所有文件 `rustfmt --check --edition 2024` 通过；本范围内无 `todo!/unimplemented!`。

测试方式：`test_support::RecHost` 包住 A4 的 `MockHost`，记录每次 `rg_texture`（预设）/`texture_by_name`（磁盘）/`rg_buffer_resource`（SSBO）调用，逐条对照机器码里的绑定序；
uniform 值在 `draw()` 之前通过 `DrawBuilder::constants_data()` 读回（`uniform_at`）；`draw` 种类、`state` 位、间接缓冲资源下标、`discard` 路径、panic 文案（`Result::unwrap`/`Option::unwrap`/`assert_failed`）均有断言。
覆盖的反面分支：`skip` 守卫、`intensity==0` 丢弃、LOD0 `mode>=2`/`b1==1`、LOD1 四种 `(b1, bit)` 组合与跳表三项、`brick_mode==0 && b0!=0` 在绑定两个缓冲之后才 `discard`、库里缺条目→实例数 0、名字不符→`assert_failed`、无常量块时 uniform 静默跳过。

## 8. 遗留

1. A4：`common_flag` 改 `u8`（§3.1）；`bind_ray_tracing<DrawBuilder>`；`EntityDrawParams.entity`。
2. A3：`AssetSsboLibrary` 实现 `SsboInstanceView`（§3.2）；`MappedStorageBuffer` 按 `BUFFER_LEN` 建模（§5）；`bind_resources` 载体的 `rg_buffer_hash`（§3.4）。
3. 世界资源的类型名（见 U）；`DuckInputs` 的三个组件类型；预设/磁盘纹理的名字。
4. `draw_list`/`common_draw_params` 里调用这些函数的闭包尚未接线（本范围没有调用者）。
5. `cpu_write<ButterflySsbo/FireflySsbo>`（0x14230edb0/0x14230f1e0，各 1057 B）与 `from_iter<ButterflySsbo/FireflySsbo>`（0x141dbe3c0/0x141dbed60）只核对了元素闭包，未逐条核对 CPU 写入循环（I：与 `MappedStorageBuffer::cpu_write` 泛型一致）。
6. 磁盘上 `/tmp` 剩余空间很小（<500 MB），harness 用了共享的 `/tmp/l0h-target`。
