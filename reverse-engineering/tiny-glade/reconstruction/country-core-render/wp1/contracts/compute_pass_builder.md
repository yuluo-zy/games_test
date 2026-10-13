# ComputePass 声明层业务契约 — compute_pass_builder.rs

研究日期：2026-10-13。仅 B patched EXE（`tiny-glade.exe` sha256 cf048e27…；PDB `tiny_glade.pdb` sha256 2d9686b8…）。
标注：**O**=反编译/机器码直接支持；**I**=名称+调用链+行号组合；**U**=未恢复。
PDB `void()` 占位不构成真实 ABI；函数签名由调用点机器码与结构体布局恢复，字段布局中未恢复的部分记为 typed gap，不臆造。

## 1. 结构体 `ComputePass`（O，尺寸 400 字节 = 0x190）

所有方法按值消费并返回 `self`（builder 模式，`memcpy(dst, src, 400)` 回写）。

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | `bindings: Vec<rhapsody::rg::pass_api::RenderPassBinding>`（{cap,ptr,len}，元素 0x44B） | `read_rg_storage_buffer` 0x14096dac0 中 in_RDX[0]/[1]/[2]，`grow_one<RenderPassBinding>` |
| +0x18 | `access_ranges: Vec<…>`（{cap,ptr,len}，元素 8B：u32 resource_index + u8 access_index） | 同上 in_RDX[3]/[4]/[5]；`grow_one<Range<ResourceAccessIndex>>` |
| +0x30 | 外部绑定 Vec（{cap,ptr,len}，元素 0x48B） | `bind_external_*` 0x14096e2a0/0x14096e540 中 in_RDX+0x30/+0x38/+0x40 |
| +0x138 | RG 资源注册表指针（hashbrown，桶@+0x98、掩码@+0xa0、计数@+0xb0；键=句柄 hash u64，值={u32 set_index, u32 resource_index}） | 三处读 +0x138 的 hashbrown 查找；`assert_failed<SetIndex>` |
| +0x136 | 1 字节“已渲染/类型”标记（`dispatch_1d` 读 +0x136==8 判 drop） | dispatch 薄封装 |

typed gap：+0x48..+0x130 之间字段（shader 程序句柄、UniformBinder、PassResources 引用、mode/closure 载体）未逐项恢复。

## 2. 生命周期：`new`（O，行 146；0x1405876a0 及 11 个单态化实例）

`ComputePass::new<T: AsComputeShaderSource>(uri: T, world: &mut World) -> ComputePass`

1. clone 2 个 `DescriptorSetBinding` 句柄（`rhapsody::raw::managed::handle::impl$21::clone<[DescriptorSetBinding;2]>`）。
2. `ShaderUri` 计算 hash（`utils::calculate_hash`），`PassResources::new_from()`（0x140909310）按 hash 从 ECS world 取 7 资源。
3. `get_resource<GraphicsDeviceResource>`（world 资源表，`unwrap_failed` 若缺）。
4. `AssetShaderLibrary::get_compute_by_name(name_hash)`：
   - 未命中 → `shader_uri::impl$1::as_compute_pipeline::<T>()` + `rhapsody::raw::shader::ComputeShaderProgram::new` + 引用计数 clone + `AssetShaderLibrary::add_compute()` 缓存（推断：compute 程序按名称缓存于 AssetShaderLibrary）。
   - 命中 → 直接引用计数 clone。
5. `UniformBinder::new()`（0x14096b420）创建 uniform 数据缓冲 + 绑定表。
6. `bind_resources::bind_color_library<ComputePass>()`（0x1415fc070）登记 `ColorLibrarySsbo` 读访问。
7. 返回 400B 结构体。

源行号锚：new@146；write_storage_buffer@185；write_storage_buffer_concurrent@211（enriched.json compiled_instances source_line）。

## 3. 资源访问声明（RG 句柄路径）

三个 RG 函数（读/写/并发写）结构相同（0x14096dac0 / 0x14096dd60 / 0x14096e000，差异仅在传给 `add_resource_access` 与 `bind` 的访问类型）：

1. 按句柄 hash（in_R8）在注册表 hashbrown 中查找 → `(set_index, resource_index)`；`debug_assert!(set_index == 0)` 否则 `assert_failed<SetIndex>`。
2. `PassBuilder::add_resource_access<PassResources>(resource_index, access_type, sync_group)`（rhapsody 边界）。
3. `pass_api::impl$5::bind()` 产出 0x44B `RenderPassBinding`（4×16B + 4B），追加到 `self.bindings`；绑定数 ≤ 0xff，否则 `unwrap_failed`。
4. 向 `self.access_ranges` 追加 8B 记录 {u32 resource_index, u8 access_index}。

调用方传句柄 hash：`read_rg_storage_buffer(&mut self, handle: …)`。源码行号：read@239 / write@264 / write_concurrent@287。

## 4. 外部绑定（`bind_external_*`，O，行 313/341）

- `bind_external_storage_buffer`（0x14096e2a0）：注册表查找 → `GraphicsDevice::get_vk_buffer()`（`unwrap_failed` 若缺）→ clone handle → 向外部绑定 Vec 追加 0x48B 记录（含 resource_index 与访问类型 5）。
- `bind_external_raw`（0x14096e540）：注册表查找 → 复制调用方传入的 8×u64 原始描述符数据到 0x48B 记录。
- 都不调用 `PassBuilder::add_resource_access`；只登记外部绑定。

## 5. Dispatch 家族（O，行 440/448/452/464/469/473/640）

所有 dispatch 末端调用 `rhapsody::rg::pass_builder::PassBuilder::render<PassResources, dispatch_impl::closure_env_0>`（0x14096eaf0 等），注册模式（+0x40 处 u32 判别）：

| 入口 | 地址 | 模式值 | 说明 |
|---|---|---|---|
| `dispatch_1d(x)` | 0x14096eaf0 | — | 置 1D 组数后 render |
| `dispatch_2d(x,y)` | 0x14096ec80 | — | 置 2D 组数 |
| `dispatch_groups(g)` | 0x14096ee10 | — | 置组数 |
| `dispatch_indirect_raw(&buffer)` | 0x14096e880 | 2 | 从 buffer 记录 +0x10 读 resource_index；`add_resource_access` 后再 render |
| `dispatch_with<F>(args, closure)` | 0x14160a2e0 | 3 | 复制 400B closure env；closure 函数指针存入记录 |
| `dispatch_indirect_with<F>(args, closure)` | 0x14160b850 | 4 | 同 3 + 从 args +0x10 读 indirect resource_index；`add_resource_access` |
| `ComputeDispatchApi::dispatch_3d` | 0x14096efa0 | — | 立即模式：读 pipeline 状态，`div_ceil` 换算组数，若 `device+0xaba` 为 1 先 `issue_full_pipeline_barrier`，锁内递增 device+0xaa0 计数，经 device+0x360 函数指针 `vkCmdDispatch` |

- 模式判别 2/3/4 与 DrawBuilder 的 draw 模式（1/3/4/5/7/8）分属不同枚举；尚未映射回 Rust 枚举名（U）。
- `dispatch_impl`（inline，132 个宿主）是共享实现：把 (mode, args, closure) 包装进 `closure_env_0`（400B）并调用 `PassBuilder::render`。
- closure 实例（PDB 单态化）：`cull_and_bucket::closure_env$1`、`group_draw_cmd_prefix_sum::closure_env$0<RenderSublistsIter>` 等（见 evidence）。

## 6. `read_storage_buffer` / `write_storage_buffer`（trait impl，O）

- `impl$3::read_storage_buffer<T: ?Sized>`（0x14160c0c0 等，行 733）：`pass_resource_access::read_access_buffer<T>()` + 返回 self。行 733 与 `bind_texture_to_compute_pass` 等相邻，属 `BindReadOnlyResources` trait（见 draw_builder 同 trait）。
- `ComputePass::write_storage_buffer<T>`（行 185）：`pass_resource_access::write_access_buffer<T>()`。
- `ComputePass::write_storage_buffer_concurrent<T>`（行 211）：`pass_resource_access::write_access_buffer_concurrent<T>()`。
- `write_to_cpu_mapped_storage_buffer<T>`（0x141609770，行 362）：`write_access_buffer<T>()` 后再按 hash 解析 resource_index 并追加 access_range。

## 7. 其它方法（I/U）

- `read_texture` / `write_texture` / `read_write_texture` / `read_texture_view` / `push_constants` / `read_draw_indirect_cmd_buffer` / `write_draw_indirect_cmd_buffer`：无独立编译实例（全部内联到 44/102/238/500/502/503 等宿主，如 `generate_mips`/`generate_hzb`/`blur_pyramid`/`bind_texture_to_compute_pass`）。`read_write_texture` 行号 161；语义为按资源名绑定纹理访问。
- `ComputeDispatchApi` / `ComputeDispatchIndirectApi` trait：提供 `dispatch_1d/2d/3d`、`push_constants`、`track_gpu_lifetime`；dispatch_3d 的立即模式见 §5。

## 8. 失败分支汇总

- 注册表缺句柄：静默跳过（无绑定追加）。
- set_index != 0：`panic!(assert_failed)`。
- 绑定数 > 0xff：`unwrap_failed`。
- 注册表计数（+0xb0）为 0：跳过整个查找。
- `dispatch_*` 消费后 self 标记为已渲染（+0x136），重复 render 由 rhapsody `PassBuilder::render` 断言。
