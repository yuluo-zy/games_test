# render/shader.rs

> 文件作用：**着色器程序加载/缓存/变体面**——`ComputeShaderProgram`（0xa0 B）、`RasterShaderProgram`（0xd8 B）、
> `RasterShaderProgramVariant`（`RwLock<HashMap<RasterPipelineDesc, Box<RasterPipeline>>>` 变体→pipeline 缓存）。
> 编译本身（shader-compiler/shader-pipeline/shader-variants）是复用依赖，本模块只做加载/缓存/接线。证据：
> `reconstruction/country-core-render/wp4/contracts/shader.md`（O）+ A3_shader（消费者侧）。

函数总计 7：业务 6 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `compile_shader_variants` | 0x1409383b0 | 0 | 猜测：编译变体；证实：`shader_variants::get_raster_shader_variants()` + 逐变体 load_compiled | **O**（1054 B）：复用库取 raster 变体清单，每个变体 `ShaderCompilerArtifacts::load_compiled<RasterPipelineDesc>()`，产出编译变体产物容器（每 16-hex 变体一个；见 render-passes/shaders/catalog.md 两变体模式） | wp4 shader |
| `bevy_ecs::…::is_exclusive<…meshwatch 系统>` / `impl$10::clone_behavior` | 0x140002370 | 24 | 猜测：bevy 包装；证实：**is_exclusive/clone_behavior 胶水（ICF 合并）** | **O**：`mov rax,rcx; ret` 类 5 B 桩；PDB 泛型名揭示 meshwatch 后端系统参数面（与 0x140002450 同族，见 mesh_library 章）。`impl$10::clone_behavior` 为 bevy FunctionSystem 胶水，非本模块业务 | audio-misc/camera-time decomp（5 B 桩） |
| `ComputeShaderProgram::new` | 0x140937e10 | 55 | 猜测：构造计算程序；证实：load_compiled + create_compute_pipeline | **O**：`shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<ComputePipelineDesc>()`（reuse，0x140 B 产物）→ debug 日志 "Creating pipeline"（module `country_core::render::shader`）→ Box 程序（0x150 分配）→ `GraphicsDevice::create_compute_pipeline<Arc<CompiledShaderPipeline>>` → 0xa0 B 结构填充（+0x90/+0x98 源路径/入口 String） | wp4 shader |
| `RasterShaderProgram::new_draw` | 0x140938240 | 95 | 猜测：构造 raster；证实：compile_shader_variants + 填充 0xd8 B | **O**：调 `compile_shader_variants()`；Ok → 拷 64 B 参数 + variants + 2 参数进结构（0xd0+）；Err → unwrap | wp4 shader |
| `RasterShaderProgramVariant::get_or_generate` | 0x140938b00 | 173 | 猜测：变体→pipeline 缓存；证实：读锁查 + 写锁生成 | **O**（1395 B）：(1) 共享锁按 desc 哈希 + 字段 +0x18 + handle-eq 查，命中返回缓存 handle；(2) miss → 排他锁 `HashMap::rustc_entry<RasterPipelineDesc, Box<RasterPipeline>>` get-or-insert：vacant → bump `Arc<CompiledShaderPipeline>` 引用、`create_raster_pipeline<Arc<CompiledShaderPipeline>>`、Box 0x38 B `RasterPipeline` 入 entry；(3) 放锁返回 handle。**两个 16-hex 变体编译产物 → 运行期 pipeline 的映射缓存** | wp4 shader |

## 内联/无独立符号函数

- `compile_shader_variants::get_compiled`（行 138）— **I**：变体产物取用助手（无独立符号）
- PDB 名存在无独立过程：`RasterShaderProgram::desc()` / `recompile()`（I：访问器/重上传包装，typed gaps）

## 纠错/差异（wp4 shader 合同）

- 类型布局：`ComputeShaderProgram` 0xa0 B（add_compute Box memcpy O）；`RasterShaderProgram` 0xd8 B（add_raster Box memcpy O）；`RasterShaderProgramVariant` 缓存 = `RwLock<HashMap<RasterPipelineDesc, Box<RasterPipeline>>>`（+0x08 锁、+0x10 buckets、+0x18 mask、+0x28 items）。
- U：`RasterPipelineDesc` +0x18 字段语义（feature/binding 选择子）；`ComputeShaderProgram` 字段级布局细节。
- 消费者：`AssetShaderLibrary::add_compute/add_raster`（A3_shader）、`ComputePass::new`（0x1405876a0，get_compute_by_name 命中复用）、`DrawBuilder::new`（0x140905be0）。
