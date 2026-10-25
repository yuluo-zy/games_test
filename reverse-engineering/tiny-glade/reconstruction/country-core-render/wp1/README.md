# WP1 — country-core pass 声明层恢复（B 版本）

日期：2026-10-13。目标：B（`tiny-glade.exe` cf048e27…；`tiny_glade.pdb` 2d9686b8…）。
本包是**业务契约 + 骨架对齐 Rust**，不是作者原文；rhapsody / bevy / ash / shader_pipeline / vk_sync_fork 为复用依赖（参照 `reconstruction/rhapsody-rebuild` 的候选 API 边界），不重实现。

## 交付内容

| 目录 | 内容 |
|---|---|
| `contracts/` | 10 个模块的逐函数组业务契约（O/I/U 标注、真实 producer/consumer、字段+布局证据源、失败分支、typed gap） |
| `src/pass_builder/` | 与 skeleton 模块树对齐的 Rust（compute_pass_builder / pass_resource_access / draw_builder / bind_resources / pass_resources / uniform_binder / framebuffer_binding / draw_list / draw_stream / shader_uri） |
| `evidence/evidence.json` | 反编译地址表、skeleton bundle 路径+SHA256、函数实例计数、判定说明、gap 清单 |
| `evidence/` 其余 | 解析 bundle 的辅助脚本 |

## 覆盖的 native 过程范围

原任务口径 ~342 native procs。真实结构是**少量泛型函数的大量单态化实例**（enriched.json 实测）：
- compute_pass_builder：142 functions（117 有编译实例）
- pass_resource_access：124（117）
- draw_builder：87（68）
- uniform_binder：18（15）
- bind_resources：5（4）
- draw_stream：27（5 有实例）
- shader_uri：52（30 有实例）
- framebuffer_binding：9（1）
- pass_resources：1（1）；draw_list：1（0）

实现策略：**每个泛型核心实现一次**，单态化实例由 Rust 泛型自然产生；实例名单与地址见 `evidence.json` 与 contracts（如 pass_resource_access §4 的 ~80 个 SSBO 实例全集）。

## 关键业务逻辑（已验证自机器码，非推测）

1. `ComputePass`（400B，builder 式按值返回）：`new<T: AsComputeShaderSource>` 经 `AssetShaderLibrary::get_compute_by_name` 缓存 shader → `PassResources::new_from` → `UniformBinder::new` → `bind_color_library`。`read/write(_concurrent)_rg_storage_buffer` 在 hashbrown 注册表（键=句柄 hash）解析 `(set_index, resource_index)`，断言 set==0，`PassBuilder::add_resource_access` + `pass_api::bind` 追加 0x44B `RenderPassBinding` + 8B access_range。dispatch 家族（`dispatch_1d/2d/groups/with/indirect_raw/indirect_with`，模式 2/3/4）全部落到 `PassBuilder::render<PassResources, dispatch_impl::closure_env_0>`；`ComputeDispatchApi::dispatch_3d` 是立即模式（div_ceil + 可选 full barrier + 锁内计数 + vkCmdDispatch）。
2. `DrawBuilder`（0xf8B）：`new` 经 FrozenMap 取 raster 程序 + DynamicConstants 分配；draw 家族把整个 builder 快照为 0xb8B 记录（判别 2）追加到 draw 命令 Vec 并消费 self；模式判别 1/3/4/5/7/8；间接绘制统一 `AssetSsboLibrary::get_rg_buffer<*>` + `add_resource_access`。
3. `RasterPass`（~0x3e8B）：`new_impl` 把 ≤5 个 `ColorAttachment` 转 `RasterPassColorAttachment`；`render` 构造 `RenderPassAttachmentDesc` 列表（≤0x80）、按深度标志加附件、`HashMap<RenderPassDesc,RenderPass>` 去重、`GraphicsDevice::create_render_pass`、`PassBuilder::render`，`DrawPassRenderCheck` drop 断言已渲染。
4. `ShaderUri::parse`：按最后 `:` `/` `.` 切成 3 个 &str（48B）；`AsComputeShaderSource` 的 18 个游戏实现 = compute pass 清单（URI 见表）。
5. `PassResources::new_from`：7 个 hash → 7 资源指针（56B）。
6. `UniformBinder`：`write_uniform<T>`/`write_optional_uniform<T>` 按绑定偏移写 uniform 数据缓冲。

## 复用边界（dependencies.json 实证）

imports 归属：`alloc/core/std`（std）、`ash`、`bevy_ecs`、`hashbrown`、`parking_lot`、`rhapsody`、`shader_pipeline`、`vk_sync_fork`、`arrayvec`、`equivalent`、`log` = 复用依赖；`country_core` = 游戏自有。游戏 SSBO 结构体（`*Ssbo`）由 WP2/WP3/WP4 实现 `SsboType` trait（本包定义边界 trait），WP1 不重定义。

## 跨包 consumer 接线主张（contract）

- 上游消费：`tiny_glade::render_loop::render`（0x1404e8010）与 `render_world`（0x1404ec110）经 `DrawLists` → `DrawBuilder`/`RasterPass`/`ComputePass` 声明，`PassBuilder::render<PassResources,_>` 注册（INDEX.md R2/R4-R8）。
- 下游依赖：WP1 的 `PassResources` 是 rhapsody `PassBuilder::render` 的 UserData；rhapsody `pass_api::{RenderPassBinding,bind}`、`ResourceAccessIndex`、`binding::PassResourceSlot` 由 rhapsody-rebuild 提供。
- WP1 拥有的共享类型：`ComputePass`、`DrawBuilder`、`PassResources`、`UniformBinder`、`RasterPass`/`DrawPacket`/`DrawStream`、`ShaderUri`、`DrawKind`、`TextureBindingKey`、`ColorAttachment`/`DepthAttachment`/`RasterFramebufferBinding`、pass 资源访问绑定类型（`AccessRange`/`ExternalBinding`）。

## 主要剩余 gap

- `ComputePass` +0x48..+0x130、`DrawBuilder` +0x60..+0xb0、`UniformBinder` 尺寸的逐字段语义。
- `PassResources` 资源名映射（7 hash → 名称）。
- dispatch/draw 模式判别值 → Rust 枚举名；`RenderPassBinding` 字段语义（归 rhapsody）。
- `ColorAttachment::desc` 真实逻辑（反编译空占位、被内联）。
- `ShaderUri::source_hash` 算法；`AsShaderDefValue` 编码与两 16-hex 变体对选择规则。
- Rust 编译接入：需与 rhapsody-rebuild 的候选 `PassBuilder`/`pass_api` API 及 WP2/WP3/WP4 的 `SsboType` 实现联调后 `cargo check`。
