# garden fresh 复核报告（restorer-eco-fresh，2026-10-17）

> 只以 fresh 反编译为准（Ghidra `--worker shared` 2026-10-17 19:32：
> `0x1415f87c0` decomp + 反汇编；`0x141695e10`、`0x140ae4620` 维持既有判等证据）。
> 范围：`startup_garden_brush`（本轮 11 项中的 garden 项）。

## 函数表

| VA | 函数 | verdict | 说明 |
|---|---|---|---|
| 0x1415f87c0 | startup_garden_brush | **判等（host 边界）** | fresh 反编译确认 **2 次迭代**，每次严格按序：`load_json_as_mesh<&str>`（失败 unwrap_failed）→ `load_mesh_into_library` → `load_shader_into_library<&str>` → `Commands::spawn_empty` → push 一条 0x78B insert 命令；第 1 次 `insert<DrawableMeshBundle>`，第 2 次 `insert<(DrawableMeshBundle, DontCastShadow)>`（0x1415f88eb / 0x1415f8a63 两处 closure_env_0 单态名逐字）。 |
| 0x140ae4620 | draw_garden | 判等（不动） | 红线：已判等函数不改。 |
| 0x141695e10 | record_garden_history | 判等（不动） | 红线：已判等函数不改。 |

## startup_garden_brush 现状核对

`src/startup/startup_garden_brush.rs` 已具备完整原生控制流：
- `GARDEN_BRUSH_MESHES` / `GARDEN_BRUSH_SHADERS`：8 个串均来自 `read_memory`
  （0x142c7d468…0x142c7d531），顺序与直接调用序一致；
- `run_garden_brush(host, load_mesh)`：逐资产 mesh→shader→spawn/insert，
  `i == 1` 时 `DontCastShadow`；bundle 载荷为 `Transform::IDENTITY` + mesh
  handle + shader handle + false（0x1415f88f0…8960）；
- ECS 系统 `startup_garden_brush` 负责 G2 网格注册；shader/spawn 依赖渲染
  管线，经 `GardenBrushHost` 边界表达（与 `startup_quad.rs::StartupHost`
  同一解耦方式），符合“渲染跨域登记接口不冒顶”。

与 fresh 反编译逐条对应，**无新增分歧**；未改代码。

## 测试

- `cargo check -p country-core --lib` 通过。
- `cargo test -p country-core --lib startup_garden_brush`：**1 passed**
  （`two_iterations_run_mesh_shader_spawn_and_second_insert_casts_no_shadow`：
  断言 mesh→shader→spawn 两轮序、8 串常量、第 2 轮 DontCastShadow）。
- 全库 `cargo test -p country-core --lib`：1974 passed / 0 failed。

## 登记

- 实际 shader 缓冲注册 / spawn 命令落地属 render 域；host 边界为库内接口，
  接线条件 = render 侧提供 `GardenBrushHost` 实现。
