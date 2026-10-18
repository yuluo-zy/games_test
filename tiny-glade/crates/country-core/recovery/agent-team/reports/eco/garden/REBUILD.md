# REBUILD 报告 — systems/garden（rebuild-gardensheep，2026-10-17）

> 依据：`2026-10-16-logic-verify-protocol.md`。只以 fresh 反编译为准（本会话重新
> get_functions 0x141695e10/0x1415f87c0，disassemble_function 0x1415f87c0，
> read_memory 0x142c7d468…0x142c7d531 逐串核实）。重建对象 = LOGIC.md 判出的
> `record_garden_history`（分歧）与 `startup_garden_brush`（未实现-需重做-部分）。

## 一、函数分类（宏胶水 vs 真业务）

| VA | 函数 | verdict | 类别 | 本批处理 |
|---|---|---|---|---|
| 0x140ae4620 | `draw_garden` | **判等** | 真业务 | 不动（红线：不删除判等函数） |
| 0x141695e10 | `record_garden_history` | 判等（本体）/ **分歧注记** | 真业务 | **重建**：删自造 failures.write 分支 |
| 0x1415f87c0 | `startup_garden_brush` | 未实现-需重做（部分） | 真业务 | **重建**：补 2×load_shader + 2×spawn/insert |
| 0x140b6d290 | `impl$0::build`（garden 插件） | 胶水-非业务 | 宏胶水 | 不动 |
| — | draw_garden 3 闭包（on_draw/on_erase/on_record） | 胶水-非业务 | 宏胶水 | 不动 |
| — | `GardenMaskSsbo` | 无业务函数 | storage element 胶水 | 不动 |

## 二、record_garden_history（0x141695e10）— 重建

fresh 反编译（2026-10-17）：
- 事件读取：EventReader 未读计数推进，未读=0 → `return`。`events.read().last().is_none()` 等价。
- 记录：hardcode 槽 5（+0x1678 Vec / +0x1680 cap / +0x1688 len / +0x1690 / +0x1694），
  take_history_stroke 语义；Arc<Vec<Vec3>> + Arc<RasterStroke> + `History::add` 0x140a84ec0。
- **关键**：`History::add` 调用后直接 epilogue（0x14169601a→0x14169601f），**无 RAX 测试、
  无失败事件写**。原生 `History::add` 失败走 `panic_fmt`（"Tried writing to History when th…"，
  无 `Err` 返回）。LOGIC §2 已注明 Rust 的
  `if let Err → failures.write(HistoryRecordingFailed)` 是共享 history 记录族
  （roof/wall/ui_clutter 同模式）的自造防御分支，非原生 garden 控制流。

重建结果：`record_garden_history` 移除 `failures: EventWriter<HistoryRecordingFailed>` 参数与
`failures.write` 分支；`record_raster_history` 的 `Result` 按原生"调用即返回"丢弃（`let _ =`）。
- Rust 位置：`src/systems/garden/record_garden_history.rs`
- 分歧点：原 `if let Err` failures 分支 → **删除**（判等至原生 epilogue）。
- 未动：`garden/mod.rs` 的 `register_garden_events`（HistoryRecordingFailed 资源注册保留，
  LOGIC 判定为胶水等价注册；garden/tests.rs 仍断言该事件为空）。

## 三、startup_garden_brush（0x1415f87c0）— 重建（补 shader + spawn）

fresh 反编译 + 反汇编（2026-10-17）证实原生按顺序执行 **2 次**：
1. `load_json_as_mesh<&str>`（0x140b0eeb0）+ unwrap
2. `load_mesh_into_library`（0x140b1aca0）
3. `load_shader_into_library<&str>`（0x140b3fe40）
4. `Commands::spawn_empty`（0x140d146a0）+ push 0x78B 命令：第 1 次
   `insert<DrawableMeshBundle>`；第 2 次 `insert<(DrawableMeshBundle, DontCastShadow)>`
   （closure_env_0 类型名含 `country_core::components::drawable::{DrawableMeshBundle,DontCastShadow}`）。

bundle 载荷（反汇编 0x1415f88f0…）：Transform::IDENTITY（旋转(0,0,0,1)、平移0、缩放1）+
网格句柄（+0x38）+ shader 句柄（+0x40）+ bool=false（+0x48）——与 `startup_quad.rs` 骨架同形。

**字符串 read_memory 逐串核实（0x142c7d468…）**：
| 用途 | 地址 | 长度 | 串 |
|---|---|---|---|
| mesh file 1 | 0x142c7d468 | 31 | `assets/meshes/garden_stone.json` |
| mesh name 1 | 0x142c7d487 | 12 | `garden_stone` |
| shader path 1 | 0x142c7d493 | 44 | `/solid_vertex_color_garden_stone.raster.hlsl` |
| shader name 1 | 0x142c7d4bf | 31 | `solid_vertex_color_garden_stone` |
| mesh file 2 | 0x142c7d4de | 33 | `assets/meshes/lowpoly_flower.json` |
| mesh name 2 | 0x142c7d4ff | 16 | `garden_flower_03` |
| shader path 2 | 0x142c7d50f | 34 | `/garden_lowpoly_flower.raster.hlsl` |
| shader name 2 | 0x142c7d531 | 21 | `garden_lowpoly_flower` |

> **纠偏（2026-10-17）**：RESTORE.md 的 shader 路径 `garden_stone/…` / `garden_flower_03/…`
> 前缀**不在镜像**——路径以 `/` 开头，shader 程序名 = 文件主名（非网格名）。

重建结构：
- `register_brush_mesh(meshes, path, name) -> Handle<Mesh>`：步骤 1-2（load_json + add），
  被 ECS 系统复用。
- `startup_garden_brush(meshes: Res<AssetMeshLibrary>)`：ECS G2 网格载入组接线（保持既有注册）。
- `GardenBrushHost` trait + `run_garden_brush(host, load_mesh)`：**完整原生控制流**
  （每资产 mesh→shader→spawn；第 2 次 spawn 带 DontCastShadow）。shader 载入与 spawn/insert
  依赖渲染管线，经 host 边界表达——同 `startup_quad.rs::StartupHost` 的既有解耦方式。
  `load_mesh` 可注入（ECS 用真 `load_json_as_mesh`；测试用假网格避免读盘）。
- `GARDEN_BRUSH_SHADERS` 常量（O read_memory）。
- Rust 位置：`src/startup/startup_garden_brush.rs`（+ 新增 2 元素测试）。

## 四、编译检查

- `cargo check -p country-core`：**通过**（14 warnings 全部为既有其他模块，本模块 0 警告）。
- 前置修复（登记，非 garden 业务）：
  - `recovered_building/building_history_transactions.rs`：补 `HistoryRecordingValidation` import
    （既有工作树 E0405，遮蔽全部后续编译）。
  - `systems/world_raster.rs::record_raster_history`：补 `M: HistoryRecordingValidation` 泛型界
    （共享 history 重构给 `HistoryTransactions::add` 加的界，garden 调用链依赖）。
- `cargo test --lib`：**测试二进制因既有重构损坏无法构建**（55 个既有错误全来自
  render/rhapsody 域 `Device`/`GraphicsDevice` 元组构造重构，其中 garden/tests.rs:66 的
  `GraphicsDevice(0)` 属同一既有原因，非本批 garden 改动所致）。本批新文件
  （record_garden_history.rs / startup_garden_brush.rs）不在错误清单。
- **行为验证**：临时 integration test（`cargo test -p country-core --test <tmp>`
  已跑后删除）编译 country-core lib 并对 `run_garden_brush` 全序列断言
  mesh→shader→spawn 两轮序 + 第 2 轮 DontCastShadow + 8 串 shader/mesh 常量 —— **3 项全过**。

## 五、自检

- record_garden_history 逐分支：events 空→return；非空→slot 5 take + `History::add`；
  无 failures 分支（与 fresh 反编译 epilogue 判等）。✓
- startup_garden_brush 两次迭代序（mesh→shader→spawn、第 2 次 DontCastShadow）由测试断言
  `two_iterations_run_mesh_shader_spawn_and_second_insert_casts_no_shadow`。✓
- shader/mesh 全部 8 串 read_memory 实证，长度与反汇编立即数一致。✓
- 未删除判等函数（draw_garden / plugin build 胶水不动）。✓
- 未改他模块业务逻辑（仅 2 处既有编译阻塞的最小修复，见上）。✓
