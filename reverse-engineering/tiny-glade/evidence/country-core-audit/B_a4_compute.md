# B 阶段报告：a4_compute（compute pass 声明层 + 帧 pass 辅助）

范围：`reconstruction/country-core/src/render/pass_builder/{compute_pass_builder,pass_resource_access,pass_resources,bind_resources,uniform_binder,mod}.rs`、
`render/rg_helpers.rs`、`systems/render_loop/mod.rs`。没有碰 draw_builder / draw_stream / draw_list / framebuffer_binding / raster_single。
标注：O = 机器码观测，I = 推断，U = 未知。

## 1. 结果摘要
- 6 处重点 `todo!`（pass_resource_access 4 + compute_pass_builder 2）全部清除；范围内文件已无 `todo!` / `unimplemented!`
  （render_loop/mod.rs 原有 5 个 `unimplemented!` 也已替换）。
- 旧版有一批**与机器码不符的臆造**，已纠正（见 §4）：访问码、同步组、注册表键值含义、SSBO 访问不经注册表、
  `dispatch_*` 的模式编号与返回值、`UniformBinder` 的键类型、`PassResources` 的字段顺序等。
- 测试：隔离测试台 325 通过 / 0 失败（本范围 64 个：compute_pass_builder 30、pass_resource_access 9、
  pass_resources 5、uniform_binder 9、bind_resources 5、rg_helpers 3、render_loop RenderContext 3）。

## 2. 函数 → Rust 项（地址 → 项）
| 地址 | PDB 过程 | Rust 项 | 结论 |
|---|---|---|---|
| 0x140909310 | `PassResources::new_from` | `PassResources::new_from` + `type_ids::*` | O：7 个 128 位 TypeId，查找顺序 A,B,C,D,E,F,G；B 可选其余缺失 `Option::unwrap`；内存布局 +0 A,+8 C,+0x10 D,+0x18 E,+0x20 G,+0x28 F,+0x30 B |
| 0x140587eb0 / 0x1405876a0 | `ComputePass::new<&str / String>`（共 12 实例） | `ComputePass::new` | O：`calculate_hash((src,&()))`；`get_compute_by_name` 命中复用，否则 `as_compute_pipeline`→`ComputeShaderProgram::new`→`add_compute`；设备缺失 `Option::unwrap`；`UniformBinder::new`；末尾 `bind_color_library` |
| 0x14096dac0 / dd60 / e000 | `read/write/write_concurrent_rg_storage_buffer` | `ComputePass::{read,write,write_concurrent}_rg_storage_buffer` | O：access 0x1a/0x37/0x37，同步组 1/0/参数，`binding` 标签 4；注册表未命中**整体跳过**；`assert_eq!(set,0)`；绑定数 ≥0x100 → `unwrap` 失败（在 `add_resource_access` 之后） |
| 0x14096e2a0 | `bind_external_storage_buffer` | 同名 | O：assert set → `get_vk_buffer`（None → `Option::unwrap`）→ 0x48 B 记录 `{binding, tag 5, vk, 0, !0, 句柄克隆, 0}` |
| 0x14096e540 | `bind_external_raw` | 同名 | O：命中覆写绑定号收入槽；未命中丢弃 |
| 0x14096e880 / 0x14160b850 | `dispatch_indirect_raw / _with` | 同名 | O：`add_resource_access(buf, 0x02, 1)`；模式 2 / 4 |
| 0x14096eaf0 / ec80 / ee10 | `dispatch_1d / 2d / groups` | 同名 | O：模式 1（线程总数，缺省维=1）/ 1 / 0（线程组）|
| 0x14160a2e0 | `dispatch_with<F>` | `dispatch_with` | O：模式 3，闭包装箱 |
| 0x14169af70 | dispatch 渲染回调体（`boxed::call_once`） | `RecordedComputePass::execute` | O/I：push constants → 按模式；组模式 `[dev+0xaba]==1` 则屏障、计数+1、`vkCmdDispatch`；间接模式 `ResourceRegistry::buffer`（"Resource does not exist"）+ `vkCmdDispatchIndirect`；闭包模式拿 `ComputeDispatchApi` |
| 0x14096efa0 | `ComputeDispatchApi::dispatch_3d` | `ImmediateDispatch::dispatch_threads` | O：local size 三项为 0 → `div_by_zero`（**先于屏障**）；屏障；`ceil` 除法；计数；`vkCmdDispatch(cmd,gx,gy,gz)` |
| 0x140588650 起（×6）/ 0x14160c0c0 | `impl$3::read_storage_buffer<T>` | `ComputePass::read_storage_buffer<T>` | O：`read_access_buffer<T>(access 0x1a)`，再查注册表追加范围 |
| 0x1416044c0 起 / 0x141609770 | `write_storage_buffer<T>` / `write_to_cpu_mapped_storage_buffer<T>` | 同名（后者委托前者） | O：两者同形，`write_access_buffer<T>(access 0x32)`；`write_storage_buffer_concurrent` 无实例（I） |
| 0x140905260 等 ×8 | `impl$4::bind_texture_to_compute_pass<A>` | `ComputePass::bind_texture<A: ComputeTexture>` | O：见 §3.4。仅覆盖 RG 路径（Preset/RgTexture/WriteTextureUnchecked/RG Handle）；`TextureFromDisk` 与受管句柄的外部绑定路径为 U（见 §6） |
| 0x1405896e0 / 0x1415fc070 | `bind_color_library<ComputePass>` | `bind_color_library_compute` | O：无槽表；access 0x1a；哈希 0x52ec6a17beb8fef0 |
| 0x1415fc230 | `bind_ray_tracing<ComputePass>` | `bind_ray_tracing_compute<Tri,Meta,Bvh,Wall>` | O：取 Tri/Bvh 缓冲（缺失 unwrap）→ 4 个 `read_access_buffer`（Tri/Meta/Bvh/WallColorId，哈希见代码）→ 2 个 `write_uniform<u32>`；ComputePass 版不写 CPU 缓冲 |
| 0x1402b00e0 / 0x140973720 / 0x140975c20 | `read/write/write_concurrent_access_buffer<T>` | 同名 | O：`get_rg_buffer<T>`（资源号 = 句柄+0x10）→ `add_resource_access` → 绑定数检查 → 追加（读标签 3 / 写标签 4），返回追加前下标；读组 1、写组 0、并发写组=参数 |
| 0x1408ef010/ef170、0x1409248b0/248f0、0x140a59800/59860 | `impl$2/$3,$6/$0::as_texture / dims` | `TextureAccess for &Handle<Texture> / &RgTexture,WriteTextureUnchecked / TexturePreset` | O：返回枚举（标签 0 RG 纹理 / 标签 1 受管：`get_vk_image_view` unwrap + 句柄克隆）；`dims`=`extent_2d(desc)` |
| 0x140c10e10 | `vk_sync_fork::AccessType::is_write` | `access_is_write` | O：`(a-0x28)<0x10` |
| 0x14096b420 | `UniformBinder::new` | `UniformBinder::new` | O：无 uniform 块 → `None`；否则按反射大小 `DynamicConstants::allocate` |
| 0x14096b570..bc10（×9） | `write_uniform<C,T>` | `UniformBinder::write_uniform` + `UniformBytes` | O：四道守卫静默返回；起点越界 `slice_start_index_len_fail`；写宽 u32/i32/f32/bool→u32 4B、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B |
| 0x14096bd20 / be40 / bf50 | `write_optional_uniform<C,T>` | `UniformBinder::write_optional_uniform` | O：同守卫；值由闭包**惰性**求（命中后才调用）；返回 bool |
| 0x140c87650 | `BindingNameHash::with_suffix_dims` | `with_suffix_dims` | O：`hash + 1` |
| 0x1409ddbb0 / dde90 / 0x14090d9a0 | `clear_rg_buffer / clear_rg_color_image / rg_buffer_copy_from_slice<u32>` | `render::rg_helpers::*`（已存在）+ `systems::render_loop::{clear_rg_buffer,…}` | O：pass 名 "Clear buffer"/"Clear image"/"Buffer memcpy"、访问 0x34、组 0；本次核对并**修正断言文案**为原版 `assertion failed: data.len() <= buffer.desc().size`（50 B，读自 0x142aab1e0）|
| 0x1409dda50 | `RenderContext::finish` | `RenderContext::finish`（消费 self） | O：仅 `ManagedHandle::drop`，无逻辑 |

## 3. 关键观测（O）
1. **访问码**：`0x1a` compute 读 SSBO；`0x37` 写；`0x32` `write_*_storage_buffer`/cpu-mapped 写；`0x02` 间接参数读；`0x34` 传输写；
   DrawBuilder 版 `bind_color_library` 用 `0x1f`。`is_write` = `0x28..=0x37`。（旧版把 0x02 当“读”、0x17/0x1a 当“写”，均不符。）
2. **同步组**：读 = 1，写 = 0，并发写 = 调用方给的 u32，间接参数读 = 1，纹理读 1 / 写 0。
3. **注册表**：`ComputePass+0x138` 指向的是**着色器反射**，键 = 绑定名哈希（u64），值 = `(SetIndex, BindingIndex)`；
   每次命中 `assert_eq!(set, 0)`；`AccessRange.resource_index` 存的是 BindingIndex，不是资源号。`*_access_buffer<T>` 完全不经注册表。
4. **bind_texture**：未命中 → 只写维度 uniform（键 `name+1`，值 `[w,h]`，闭包惰性）；命中 → assert → 取纹理 → 绑定数检查 →
   `add_resource_access(res, access, write?0:1)` → 视图绑定（读/写两个构造函数）→ `AccessRange` → 维度 uniform。
5. **RenderPassBinding 变体**：`read_access_buffer` 标签 3（0x140bd6c70），`write_*`/`*_rg_storage_buffer` 标签 4（0x140bd6c60），
   图像视图另一变体（+0x3c 资源号，+0x40 写标志）。
6. **PassResources 角色（O）**：+0 `AssetShaderLibrary`、+8 `AssetTextureLibrary`、+0x10 `AssetSsboLibrary`、+0x28 `Mutex<DynamicConstants>`
   （`rg_buffer_copy_from_slice` 与 `ComputePass::new` 都对它 lock）。+0x18/+0x20/+0x30 类型 U。
7. **dispatch 返回 `()`**（不返回 ComputePass），把闭包状态交给 `PassBuilder::render`。

## 4. 对公共 API 的改动（旧 → 新，原因）
- `ComputePass::new(uri, &mut World, &GraphicsDeviceResource)` → `new(name, &source, ComputePassInit{world, backend, uniforms})`：原签名无法表达 O 的 name/world/设备/uniform 池。无外部调用者。
- `read/write_rg_storage_buffer(hash)` → `(hash, &RgBuffer)`（`+concurrent` 多 `group`）：原版需要资源号（`handle+0x10`）。
- `bind_external_storage_buffer(hash, ExternalBinding)` → `(hash, &ManagedHandle)`；`bind_external_raw(hash, [u64;8])` → `(hash, ExternalSlot)`；`ExternalBinding` 重定义为 `{binding, slot}`。
- `read/write_storage_buffer<T>()` → `<T: asset_libraries::ssbo_library::SsboType>(hash)`（原版泛型是库的 SsboType，而不是 `pass_resource_access::SsboType`）。
- `dispatch_*` 返回 `RecordedComputePass`（原 `ComputePass`）；`dispatch_with/indirect_with` 取 `impl FnOnce(&mut dyn ComputeDispatchApi)`；`dispatch_indirect_raw(&RgBuffer, offset)`；`DispatchArgs` 保留。
- 删除：`dispatch_3d_immediate`（依赖 `ash::vk::CommandBuffer`，改为 `ImmediateDispatch` + `ComputeExec` 接缝）、`read/write/read_write_texture(name)`、`TextureAccess` 枚举（改 `bind_texture`）。
- `ResourceRegistry`：原 `{buckets,mask,count}` 裸指针字段 → `{bindings: HashMap, uniform_block, uniform}`；`get(hash)` 与 `Default` 保持（draw_builder 仅用这两个）。
- `AccessRange`、`PassResourceSlot`、`BindGameResources` 全部**既有方法签名未改**（a4_draw 的实现不受影响）；新增 provided 方法 `resolve_binding`（默认 `None`）。
- `pass_resource_access::SsboType`（旧 `rg_buffer_hash` trait）保留，供 `resources/*`、`water*`、`transparency_pass` 等现有 impl 使用。`ACCESS_READ/WRITE/WRITE_CONCURRENT/SYNC_GROUP_DEFAULT` 常量保留但**取值已纠正**（0x1a/0x37/0x37/0）。
- `UniformBinder`：`write_uniform(binding: BindingIndex, &T)` → `write_uniform(&UniformReflection, key: u64, T)`；`write_optional_uniform` 取闭包；键由 `BindingIndex(u32)` 改 u64 名字哈希。`render/uniform.rs` 里的 `todo!`（不在我范围）现在可以直接用该签名落实。
- `PassResources`：字段 `r0..r6` → 按内存顺序的具名字段；`new_from(&World)` → `new_from(&(impl ResourceLookup + ?Sized))`，`World` 的实现为 typed gap（见 §5）。
- `systems::render_loop`：`RenderContext(pub Gap)` → 结构体（pass 登记表）；删除无 PDB 对应的 `clear_buffer`；`clear_rg_*`/`rg_buffer_copy_from_slice` 参数改为 `&mut RenderContext, &RgBufferInfo/&RgImageInfo`（泛型 `<T>` 去掉：只观测到 `u32` 实例）；`finish<T>` → `RenderContext::finish(self)`。

## 5. 共享文件改动（仅追加，请集成者合并）
- `src/lib.rs`，`rhapsody_contract` 内追加一个 `impl<'a, UD> PassBuilder<'a, UD> { pub fn detached() -> Self }`（`BindGameResources::as_pass_builder` 的 ComputePass 视图用）。
- 未改 `rhapsody_types.rs` / `frozen_map.rs` / 其他 contract。

## 6. Stub / 接缝清单（不发明行为）
- `ResourceLookup`（`Components::get_resource_id` 的 128 位 TypeId 查找）；`impl for World` = typed gap（恒 `None`，会走原版 `unwrap_failed` 路径）。
- `WorldAccess`（`get_resource<GraphicsDeviceResource>`）、`ComputePassDevice`（`get_vk_buffer`、管线反射）、`TextureDevice`（`get_vk_image_view` 与受管纹理 desc）、
  `UniformAllocator`（`DynamicConstants::allocate`+`into_uniform_buffer`）、`ComputeExec`（`vkCmdDispatch[Indirect]`、屏障、计数、local size、push constants）、
  `RecordingPassBuilder`（`PassBuilder` 的 `add_resource_access` 与 `+0x40` 渲染图，复用 `rhapsody_types::RenderGraph`）。
- `bind_resources` 的 Rt*/WallColorId SSBO 载体（真实 `resources::rt::*` 含 `Vec`，不满足 `StorageElement: Copy`；`bind_ray_tracing_compute` 对类型泛型，真实类型就绪后直接换）。
- `RenderContext::allocate_dynamic`：外部库行为 U，按 4 字节推进游标。

## 7. 未恢复 / 不确定（U / I）
- U：`bind_texture` 的**受管句柄 / `TextureFromDisk` 外部绑定路径**（0x1409056d0、0x1408ee240 中 `get_vk_image_view`+句柄克隆分支）只实现了 RG 路径；`TextureAccess` 已给出标签 1 的数据；`Handle<Texture>` 经 `bind_texture` 时资源号取 `handle()`、extent 取受管 desc（I）。
- U：`bind_ray_tracing` 两个 u32 uniform 的取值来源（I：`Buffer` 首个字的低 32 位；Tri→键 0x1c9f…，Bvh→键 0xc2b7…）。
- U：PassResources +0x18/+0x20/+0x30 三个资源的类型；`ComputePass+0x140` 的确切类型（I：设备）。
- U：push constants 的原始承载字段（I：自行保存字节，回调前推送）；`write_storage_buffer_concurrent` 无原实例。
- I：`TexturePreset::as_texture` 的 extent 取自库里 `PresetTextureState.texture`（占位 `ImportedTexture` 不含 desc）。
- 未恢复：`PassBuilder::render` 内部的图录制与 `RenderGraph` 编译（rhapsody 库，范围外）。

## 8. 验证
```
/tmp/a4c_run.sh   # 等价于 B_BRIEF 的流程：复制 tools/country-core-audit/l0-harness → /tmp/h_a4_compute，挂载本范围真实源文件，
                  # 用真实 lib.rs 的 rhapsody_contract 与 render_loop/mod.rs 的 RenderContext 片段，离线 cargo test
test result: ok. 325 passed; 0 failed
```
- `rustfmt --edition 2024 --config max_width=120` 已应用于 compute_pass_builder / pass_resource_access / pass_resources / bind_resources / uniform_binder / rg_helpers；
  `mod.rs`、`render_loop/mod.rs` 因 rustfmt 会递归格式化他人子模块，未运行（语法由测试台编译覆盖）。
- 测试覆盖：常量与立即数（访问码、哈希、TypeId）、各分支（注册表未命中静默、set≠0 断言、绑定数 0x100 溢出且 access 已先登记、vk 缓冲缺失 unwrap、设备缺失 unwrap、
  local size=0 在屏障之前 panic、间接缓冲缺失 "Resource does not exist"）、惰性闭包、着色器程序只编译一次、各 dispatch 模式与执行序。
- harness 偏差：把真实 `rhapsody_contract` 中的 `ash::vk::CommandBuffer` 在测试台里替换为 `u64`（harness 无 ash）。
