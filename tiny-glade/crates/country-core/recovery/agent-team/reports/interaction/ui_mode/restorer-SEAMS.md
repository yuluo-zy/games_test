# UI 域 7 项 seam 收口 — restorer-SEAMS

日期: 2026-10-17
依据: fresh ghidra shared 只读（`get_functions fields=signature,decompiled_code,callees` +
`disassemble_function` + `read_memory`）与既有 fresh 证据（`work/ui-restorer-remainder/`）。
本轮证据落盘 `recovery/agent-team/work/ui-restorer-seams/`：
`fresh-1..11-result.json`、`disasm-producer-result.json`、`producer-callees-result.json`、
`callees-result.json`。
身份 B patched EXE `tiny-glade.exe`（sha256 cf048e27…6323）。
协议: `planning/2026-10-16-logic-verify-protocol.md`、`2026-10-17-recovery-sop.md`。

## 汇总

| # | seam | 状态 | 落点 |
|---|---|---|---|
| 1 | RtMesh 矩阵 / RT Resource | **落地**（0x60 布局订正 + Resource + tag-2 句柄） | `resources/rt.rs`、`systems/prefab.rs` |
| 2 | atlas cull 来源 | **锚定+落地** | `systems/prefab.rs`（`CullInstanceSource`） |
| 3 | PrefabWatcher | **落地**（hotwatch 100 ms，集合语义） | 新 `startup/load_prefabs.rs` |
| 4 | PrefabPart 生产者 | **锚定+部分落地**（mesh 载入 seam） | `startup/load_prefabs.rs`、`resources/prefab.rs` |
| 5 | GradingLuts 纹理 0x38 | **落地**（stride/搬移锚定，内部字段登记） | `resources/grading_lut.rs` |
| 6 | 跨文件 PrefabInstanceData | **统一**（re-export 0x58 语义） | `components/trees.rs` |
| 7 | debug_display / debug_set_history_start | **落地**（逐字节格式串） | `resources/history/mod.rs`、`history_transactions.rs` |

编译: `cargo check -p country-core --lib` **PASS**（0 errors）。
单测: 相关过滤 `prefab:: grading_lut:: resources::rt:: startup::load_prefabs:: debug_display`
**43 passed / 0 failed**；全库 `cargo test -p country-core --lib` **2068 passed / 0 failed**。

## 1. RtMesh 矩阵 / RT Resource

fresh 订正（0x1415626f0 `DirtyCow<RtMesh>::get_mut` 按 **0x60** value layout 分配；
0x141562aa0 `RtWorld::get_mut` 默认写序列；0x140841550 / 0x140841d60 写面）：

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00..0x40 | `matrix: [f32;16]` | create 写 `[RDI+0x00..0x3c]`；update 比较 `+0x10..0x4c` 后写全 16 |
| +0x40..0x58 | `positions: Vec<Vec3>` | create 调 `Mesh::positions`（0x140a07b30）+ `0x1415634c0` 按 u32 索引映射 Vec3；元素 0xc |
| +0x58 | `color: u32` | `MOV [RDI+0x58],ECX` |

* 默认值（0x141562aa0）：`m[0]=m[5]=m[10]=m[11]=1`（与 create 的 [10]/[11] 重复 lane 一致）、
  空 positions（cap0/ptr dangling 4/len0）、color 0。
* `RtWorld` 现 **derive(Resource)**（mangled `hot_reload_prefabs` 系统签名含 `ResMut<RtWorld>`）。
* 句柄：`create_prefab_rt` `MOV EDX,0x2` + `RtWorld+0x30` 计数器 → `RtMeshHandle{tag:2,payload}`；
  新增 `RtMeshHandle::from_prefab` / `RtWorld::alloc_prefab_handle` / `write_prefab_mesh`。
* `RtBvhSsbo` 元素 0x50（upload 断言）**不是** 0x60 的 `RtMesh`；拆为 `RtBvhRecord([u8;0x50])`，
  载荷字段登记（不编造）。
* `update_prefabs`：`RtWorld::get` 读矩阵，仅比较 `matrix[4..16]`，不等才 `get_mut` 写全 16。
* 仍登记：0x48 B 延迟命令记录（`10.0`、panic handler）与 `{2, handle}` 事件的宿主队列；
  `build_prefab_contents` 以 `(handle, radius)` 返回值暴露。

## 2. atlas cull 来源

`create_prefab_gpu_instances` 0x140841b00 disasm：`base_cull` 为 **16 B**：
前 12 B 逐字节复制到输出 `InstanceCullData.position`；`+0xc` **f32** 经
`half::binary16::f16::from_f32`（0x1408b9f20）写输出 `radius`（f16 @+0xc）；
`+0xe` 被 `part_index`（`PrefabMeshAtlasPart+0` 低 16 位）覆盖为 `mesh_index`；
`kind = part+4`（subset 字节）。已落地为 `CullInstanceSource { position: Vec3, radius: f32 }`。
来源寄存器（spawn 的 `SpawnPrefabCmd` 载荷）仍未锚定，登记。

## 3. PrefabWatcher

fresh：`new` 0x1412a8e00 `MOV R8D,0x5f5e100` = **100 ms**；hotwatch
`new_with_custom_delay`；结构 +0x30/+0x38 两个 `Arc<Mutex<HashSet<…>>>`；未监视判别 3。
0x140922830 / 0x140922a80 closure：命中事件时锁集合，把捕获的 `Vec<PrefabName>` 全部
insert（**不是**从路径解析名），或插入捕获的 `GpuWorldSubsetKind`。
已落地 `startup::load_prefabs::{PrefabWatcher, watch_prefab, watch_atlas, take_changed,
reload_prefabs}`（`hotwatch::Hotwatch`；O 为 Windows watcher，平台别名登记）。
登记：closure 原始事件谓词 `*event >> 1 == 0x4000000000000001` 未映射到具名
`DebouncedEvent` 变体（暂以 `Write` 近似）；`PrefabWatchPlugin::build` 仍为 bevy_app 胶水。

## 4. PrefabPart 生产者

* 生产者系统 = `startup::load_prefabs::load_prefabs_impl`（0x1412a1df0）；调用链 fresh：
  `asset::prefab::scan` RON 配置 → `set_attribute<Vec<[f32;3]>>` 0x140a34c90 /
  `set_attribute<Vec<f32>>` 0x140a35b80 → `filter_mesh_by_vertex_color` 0x140a08680/0x140a08d30
  → `Mesh::aabb_bounds` 0x140a07870 → `PrefabLibrary::add` 0x1408e2540（每 0x28 元素
  `(PrefabName, Prefab)`），另 `PrefabDependencies` insert 0x14152b320。
* 载荷 `Prefab { parts: Vec<PrefabPart> }`；PrefabPart 0x98 偏移表（fresh）：
  +0x18 audio Option、+0x20/+0x28 kind 名/tag、+0x30/+0x38 name、+0x40 rank、
  **+0x48..0x60 Aabb3 缓存**、+0x60/+0x64/+0x68 / +0x6c/0x70/0x74 / +0x78/0x7c/0x80
  三套 albedo（read_memory 三表 r={0x60,0x60,0x60,0x6c,0x78}、g={…0x64…0x70,0x7c}、
  b={…0x68…0x74,0x80} → 季节 0..2→[0]、3→[1]、4→[2]）、+0x84 rt_kind。
* 已落地 `load_prefabs_impl`（配置→运行时 Prefab，albedo/rt_kind/audio kind）；
  登记 mesh 载入（`load_prefab_mesh`）、bounds 计算、audio name/rank、rt.semantic 映射。

## 5. GradingLuts 纹理 0x38

`GradingLuts::load` 0x140b01df0 双 Vec：meta stride **0x28**、texture stride **0x38**
（两段独立 `grow_one`）；`load_cube` 0x140b01050 `MOVUPS [RDX+0x28..0x60]` 原样搬
`create_texture` sret 的 0x38 B。已落地 `TextureRecord`（`#[repr(C)]` 0x38：
语义 `Texture` 0x18 + `_gap [u8;0x20]`）与 `META_RECORD_SIZE`/`TEXTURE_RECORD_SIZE`；
记录内部字段属 `rhapsody::raw::texture::Texture`，未锚定部分保留 gap，登记不编造。

## 6. 跨文件 PrefabInstanceData

`components::trees::PrefabInstanceData` 改为 `pub use crate::systems::prefab::PrefabInstanceData`
（0x58 语义：+0x08 GPU Vec / +0x20 RT Vec / +0x38 audio Vec / +0x48 PrefabName）。
`components/mod.rs` 既有 re-export 不变；无调用方使用旧 `position/scale/rotation_degrees` 字段。

## 7. 库内小项

* `History::debug_display`（0x140a84ba0）：格式片段逐字节 `"History: "` / `"/"` /
  `". Start at "`（read_memory 0x142ad80b8/c1/c2），参数为 `History+0x68`（history_chain
  Display，读 `chain+0x30`）、`cursor-1`、`History+0x6c`。已落地
  `format!("History: {}/{}. Start at {}")`；+0x68/+0x6c 以 `debug_chain` /
  `debug_history_start` 字段表出（chain 内部语义登记）。
* `History::debug_set_history_start`（0x140a84c40，4 B）：`*(u32*)(self+0x6c) = arg`，已落地。
* `missing_prefab` 0x1409e6210 已在 `resources/history/tree_edit.rs` 实现（复核保留）。
* 其余 ad-hoc 登记（`add_decorator_diff`、`send_state_diff_events`、
  `convert_before_after_states_to_changes` 等）仍缺跨域类型/载荷，维持登记。

## 文件改动

`src/resources/rt.rs`、`src/systems/prefab.rs`、`src/resources/prefab.rs`、
`src/resources/grading_lut.rs`、`src/components/trees.rs`、`src/resources/history/mod.rs`、
`src/recovered_building/history_transactions.rs`、`src/startup/load_prefabs.rs`（新）、
`src/startup/mod.rs`、`Cargo.toml`（+`asset` path 依赖）。
未改 `src/geometry/**`、`src/asset_libraries/ssbo_library.rs`、
`src/render/sparse_instance_buffer.rs`、`src/systems/collision/**`。

## 纪律声明

- 逐条以 fresh 反编译/机器码为准；未锚定字段用 `_gap`/登记，不编造。
- 宏胶水（Bevy 插件注册、derive）未重建；`PrefabWatchPlugin::build` 保持胶水标记。
- 环境：首次 `cargo test` 遇 ENOSPC（磁盘 100%），仅删除
  `tiny-glade/target/debug/incremental`（构建缓存）后复跑；未触碰源码其他树。
