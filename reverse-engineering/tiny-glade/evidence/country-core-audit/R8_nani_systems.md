# R8：render::nani 系统层（maintain / 20 × update / flush）

日期：2026-10-14。落点：`tiny-glade/crates/country-core/src/render/nani/systems.rs`。

## 做了什么
`nani_maintain_instances`（0x14206f300）、20 个 `nani_update_*`（0x142070ac0..0x14207fcc0）、`nani_flush_instance_updates`（0x142080640）。
工作区不用真实 bevy `Query`，系统写成"行视图 + 纯函数"（与 `systems/render_loop` 一致）；载荷数值仍全部来自 R2 已核对的 `instance_updates::build_*`，本文件只负责：选 kind、取 owned subset 写锁、`update_instance_data`、把字载荷装进对应实例数据类型。

## EXE 核对（本次）
| 项 | 证据 |
|---|---|
| 解锁后 `total_instance_count += 1` 无条件 | 0x14206fc31 `INC qword [rax+0x1440]`，在 `extend_no_data`（0x140992280）+ 解锁之后、记录检查（0x14206fc44 `cmp r14b,0x1b`，0x1b = 27 = "无记录"哨兵）之前 |
| 无记录 → warn-once | 0x14206fc59 起构造 `Arguments`，字符串 0x142fd3580 = "Instance limit reached while trying to spawn "，日志目标串 "country_core::render::nani::systems"（0x142fd34c0 附近实读） |
| `Named` 找不到 → panic 文案 | 数据区 0x142fd3430 起三段：`"NaniInstanceDesc requested mesh "`、`" and shader "`、`",\nbut this mesh is not registered for that shader in nani_meshes.ron"` |
| 先删后建再改 | 汇编里三段循环的先后顺序与 `B_n_sys.md §2` 一致 |
| `Mutex was poisonned` + `$crate :: Level :: Warn` | 0x142fd35xx 实读，去重键沿用 `nani_warn_once!` |

## 偏差与不确定
* DIGEST/旧 wp6 `systems.rs` 的 `nani_maintain_instances` 是带 `todo!()` 的骨架，**未采用**；这里按机器码和 `B_n_sys.md §2` 重写。
* kind ↔ 系统 ↔ 数据类型的对应取自 DIGEST（B 阶段 O）；"StoneFloor = 9" 在本工作区的枚举里是 `RoofFloor`。
* 19 个 gap 组件的真实字段布局仍是 U：行视图只带 `build_*` 已需要的原始字。`NaniInstanceDesc` 用枚举表达，不复刻 24 B 内存布局。
* `Or<(Changed<…>, Changed<NaniActiveInstance>)>`（plaster / stairs_floor）的变更过滤由调用方完成。
* `RemovedComponents` 里"generation == 0 的无效实体跳过"没有对应物（`Entity` 在 bevy 0.16 里不会为 0）。
* `startup_nani`、`load_mesh_atlases`、`reload_mesh_src`：未做（依赖启动流程与 `AssetMeshLibrary` 的 `reload_mesh_src`，归 F 包）。

测试 +7（maintain 三段、Named 解析/错 shader panic、删除未知实体 panic、update 写入载荷、实例类型不符 panic、flush 滚动计数）。
