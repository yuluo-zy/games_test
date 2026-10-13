# DrawBuilder 声明层业务契约 — draw_builder.rs

标注：**O**=反编译直接支持；**I**=调用链/行号组合；**U**=未恢复。

## 1. 结构体 `DrawBuilder`（O，尺寸 0xf8 = 248 字节）

所有方法按值消费并返回 `self`（builder 模式，`memcpy(dst, src, 0xf8)` 回写）。

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | 第一个字段（0x48B 记录快照判别/占位；`draw()` 置 2 后并入记录） | draw* 快照 push 模式 |
| +0x10 | `access_ranges: Vec<…>` {cap,ptr,len}（8B/元素） | `read_rg_storage_buffer` 0x140906150 中 in_RDX+0x10/+0x18/+0x20 |
| +0x18 | 外部/属性绑定 Vec {cap,ptr,len} | 同上；`bind_external` |
| +0x40 | `mode: u32`（draw 命令判别值，见 §3） | `multi_draw_indexed_indirect` 置 7 等 |
| +0x44 | draw 参数区（+0x44/+0x48/+0x50/+0x58/+0x60/+0x64，随 mode 解释） | 各 draw* 写入 |
| +0xb0 | `variant: u8`（raster 变体索引） | `new` 写 in_R9B |
| +0xb8 | uniform 数据缓冲 ptr；+0xc0 len | `optional_uniform` 0x141629a20 读 +0xb8/+0xc0 |
| +0xde | 状态字节（==8 判“无 pass”） | `optional_uniform` 读 +0xde |
| +0xe0 | 游戏上下文指针（含 placement map@+0x1c8、uniform 绑定表@+0x118/+0x120、注册表） | `new`/`read_rg_storage_buffer`/`draw_instanced_from_placement_data` |
| +0xe8 | 资源注册表 / shader 库指针（hashbrown@+0x98/+0xa0/+0xb0） | `read_rg_storage_buffer` 读 +0xe8 |
| +0xf0 | 0（新对象占位，可能为“已分配动态常量”标记） | `new` 写 0 |

typed gap：+0x60..+0xb0、+0xc0..+0xe0、+0xf0 之间字段语义未逐项恢复（HashMap<BindingIndex, DescriptorSetBinding>、shader 句柄 Arc、PassResourceSlot 表等）。

## 2. 生命周期：`new`（O，行 73；0x140905be0）

`DrawBuilder::new(world: &World, shader: Handle<RasterShaderProgram>, variant: u8) -> DrawBuilder`

1. `FrozenMap::get<RasterShaderProgram>(shader_handle)`（AssetShaderLibrary 中 raster 程序表；`unwrap_failed` 若缺）→ 引用计数 clone（per-variant 槽 `+0x58 + variant*0x40`）。
2. `Components::get_resource_id()`（hash 0x2ed189581b92decf → GraphicsDeviceResource；缺则 `unwrap_failed`）。
3. `rhapsody::raw::dynamic_constants::DynamicConstants::allocate()` → `DynamicConstantsChunk::into_uniform_buffer_binding_and_data()` 分配动态常量段。
4. `HashMap<shader_pipeline::reflection::BindingIndex, GpuOwned<DescriptorSetBinding>>::insert(...)` 登记动态常量绑定。
5. 写 variant@+0xb0、world 指针@+0xe0、shader 句柄@+0xe8、+0xf0=0。

## 3. 资源访问声明（O，行 139/166/210）

- `read_rg_storage_buffer`（0x140906150）/ `write_rg_storage_buffer`（0x1409067e0）：注册表 hashbrown 查找（+0xe8）→ `debug_assert!(set_index==0)` → `PassBuilder::add_resource_access<PassResources>` → `pass_api::bind()` → 追加 0x44B `RenderPassBinding`（到 *(self+0xe0) 结构 +0x18/+0x20/+0x28 的 Vec）→ 追加 8B access_range。
- `bind_external_storage_buffer`（0x140906400）：注册表查找 → 追加外部绑定记录。

## 4. Draw 命令家族（O，行 278/286/294/303/308/324/336/388/409/432/476/484）

每个 `draw_*` 方法：写 mode@+0x40 与参数区 → 把整个 builder 快照为 0xb8B 记录（判别 2 + payload[+8..+0xb8]）追加到 draw 命令 Vec（*(self+0xe0) +0x00/+0x08/+0x10）→ `drop_in_place<DrawBuilder>`（消费 self）。

| mode | 入口 | 地址 | 参数区语义 |
|---|---|---|---|
| 1 | （间接句柄路径的清理分支，见 panic/drop 代码） | — | 存句柄；重建时 `managed::handle::impl$6::drop` |
| 3 | `draw_instanced_w_gl_draw_flag(flag)` | 0x140907490 | +0x44 = gl_draw_flag |
| 4 | `draw_multi_instanced(base,count)` | 0x140907050 | +0x44 起点/+0x4c 数 |
| 5 | `multi_draw_indirect(buffer,offset,draw_count)` | 0x1409076b0 | `AssetSsboLibrary::get_rg_buffer<DrawIndirectBuffer>`；+0x44/+0x50/+0x58 |
| 7 | `multi_draw_indexed_indirect(idx, offset, count)` | 0x140907990 | `get_rg_buffer<DrawIndexedIndirectBuffer>`；`DAT_142aaaa78 + idx*0x14` 读 4B 常量（间接段表）；+0x48=idx*0x14 偏移、+0x60=1 |
| 7 | `draw_indexed_indirect_raw` | 0x140907cc0 | 从间接缓冲记录 +0x10 读 resource_index；`add_resource_access` |
| 8 | `draw_indexed_indirect_count` | 0x140907f90 | 两个间接缓冲（draw + count 各 `add_resource_access`）；+0x44/+0x48/+0x50/+0x58/+0x60/+0x68 |
| — | `draw`（0x140906f40）/`draw_no_vao`（0x140906a90）/`draw_no_vao_indexed`（0x140906cb0）/`draw_no_index_buffer`（0x1409083f0） | — | 直接快照 push，不写间接参数 |
| — | `discard`（0x1409082c0） | — | 丢弃 builder（不 push 记录） |

间接绘制统一先 `AssetSsboLibrary::get_rg_buffer<...>()` + `add_resource_access`；`multi_draw_indexed_indirect` 额外从 `DAT_142aaaa78` 常量表按间接段索引读 4B（推断为每段 draw count 或 stride 配置）。

## 5. 绘制参数与 uniform（O/I）

- `draw`（0x140906f40）仅快照；具体 `Draw`/`DrawIndexed` 参数由闭包在录制期解释。
- `impl$2::optional_uniform<T>`（0x141629a20：f32；0x141629b90：Mat4；0x141629d20：[u32;2]）：`(*(self+0xde) != 8)` 且上下文有效时，按 uniform 名 hash（+0xe8→+0x118/+0x120 表）取偏移，写 `T` 到 uniform 缓冲（+0xb8，越界 `slice_start_index_len_fail`）。
- `impl$2::uniform<T>`（inline，155 宿主）：同 optional_uniform 但必选（无 `==8` 守卫）。
- `impl$2::read_storage_buffer<T>`（行 553；0x141629e90 等 43 个实例）：`pass_resource_access::read_access_buffer<T>()`。
- `push_constants`（inline 1）/ `stencil_ref`（inline 11）/ `draw_indexed_indirect`（inline 11）/ `draw_indirect`（inline 1）/ `draw_instanced`（inline 13）：均无独立编译实例。
- `bind_dummy_shadowmap`（inline 1）：绑定占位 shadowmap。
- `get_indirect_args_buffer`（inline 2）：取间接参数缓冲句柄。
- `NoDropOnlyTake::{take,try_take}`（I）：取走型辅助，防止误 drop。

## 6. `bind_texture_to_draw_builder<T>`（impl$3，O，行 601）

`0x1408ee800`（ref Handle<Texture>）/ `0x1408fd6c0`（ref Texture）/ `0x140908610`（TextureFromDisk）/ `0x140908c50`（TexturePreset）/ `0x14096f650`（ReadOnlyHandle<Texture>）。语义：把纹理绑定到 DrawBuilder 的纹理槽，内部经 `TextureBindingKey`/`optional_uniform` 登记尺寸。

## 7. Drop（O，行 55；0x14089a140）

`impl$0::drop`：释放 mode 相关的句柄/堆（mode==1 → managed handle drop；mode==4 且 +0x48 非空 → dealloc 间接参数堆）。
