# 第 0 层（A1 渲染基元）逆向进度与证据

范围：`AUDIT_REPORT.md` 里依赖分层的 L0（A1）。规则：先挖业务逻辑，库依赖（rhapsody / puffin / bumpalo）只占位。
方法：`tools/country-core-audit/dis.py <地址>` 逐条核对原机器码；数学/语义写成单测，在隔离测试台
`tools/country-core-audit/l0-harness`（只依赖 glam，离线可跑）里编译执行：

```
cd tools/country-core-audit/l0-harness && CARGO_TARGET_DIR=/tmp/l0h-target cargo test --offline   # 72 passed
```

> 范围声明：测试台编译 16 个文件（view_constants、frame_constants、dirty_index_range、ssbo、shadowed_buffer、
> mesh、vao、utils、shader_uri、directional_light、atmosphere、rg_helpers、shader、render_config、load_texture，
> 外加 utils_contract / parking_lot 两个占位）。**整个 crate 仍然没有在本机编译过**（依赖 `dolly` 等包无法离线/联网获取）；
> 未挂进测试台的改动文件（`render/mod.rs`、`draw_stream.rs`、`lib.rs`）只做了 `rustfmt` 语法解析。

## 一、本轮完成（O＝与原机器码逐项核对，I＝推断）

| 单元 | 地址 | 结果 |
|---|---|---|
| `ViewConstants::set_pixel_offset` | 0x140b18af0 | 重写。TAA 抖动：`j = 2·offset/viewport`；`clip_from_view_jittered = T(−j)·P`，`view_from_clip_jittered = P⁻¹·T(+j)`，`clip_from_world_jittered = Pj·V`；同时存 prev 偏移与 prev jitter。参数 `(offset, prev_offset, viewport:u32×2)` 来自 Win64 寄存器/栈 |
| `ViewportConstantBuilder::build` | 0x140b18f60 | 重写。builder 是 `[当前4矩阵][上一帧4矩阵] + viewport + 两个偏移`（0x220 B）；输出 0x320 B = 12 个 4×4 + 4 个 Vec2；`prev_clip_from_clip = ((E·G)·D)·B`（重投影） |
| `ViewConstants` 布局 | — | 偏移 0x000…0x318 全部对上；旧版字段数量、顺序、含义均错 |
| `ViewConstants::default` | 0x1400ef490 | 12 个单位矩阵 + Vec2 全 0（O）。该函数是 `[ViewConstants;5]` 的默认值 |
| `FrameConstants` | 0x14012d450 / 0x1401c42b0 | 总大小 **0x1090**；含 5 个 `ViewConstants`（4 次 `build` + 1 次 `set_pixel_offset`，见 `prepare_frame_constants` 0x140af40b0） |
| `FrameConstants::should_reset_temporal` | 0x140976880 | `cmp [self+0x80],0; sete al`，即 u32 **等于 0** 为真。旧版读一个 bool，语义相反 |
| `TypedStorageBuffer::new<T>` | 0x1416152f0 ×45 / 0x1416154c0 / 0x141616c00 | 重写。形态是 `(device, Option<&[T]>)`：`BUFFER_LEN` 三种取值 0 / 1 / 0x6c0；用途 0x22(STORAGE\|DST) 或 0x12(UNIFORM\|DST)；无初始数据时：0→panic 并带完整文案、1→`default_value()`、>1→整块清零；数据过长 panic "Excessive data size supplied for buffer {type}" |
| `InstanceBuffer::new_storage_buffer` | 0x1416207e0 | 重写。真实断言是 `len % size_of::<T>() == 0` 与 4 字节对齐；**旧版的 `count % 4 == 0` 在原函数中不存在** |
| `MappedStorageBuffer::new_storage_buffer` | 0x14161ad70 | 重写。缓冲恒为 0x40000 B，memory=2，对齐 **1**；数据 >0x40000 panic；断言 `mapped.len() >= desc.size` |
| `gpu_write_ranges_nonoverlapping` | 0x14161ab00 | 元素区间 ×es 换成字节区间；空区间直接返回；多出的第 5 个参数是目的字节偏移 |
| `ShadowedBuffer::new` | 0x14168f870 | **按值取 `Vec<T>`**，用其切片建 GPU 缓冲，再 `shrink_to_fit` 成 Box。旧版会得到空影子数组（数据损坏缺陷，已修） |
| `ShadowedBuffer::gpu_flush` | 0x141691b30 | 闭区间；传给设备的是 `data[start..=end]` 的字节、范围 `0..n·es`、目的偏移 `start·es`。旧版把范围写成绝对位置，造成双重偏移 |
| `DirtyIndexRange` | — | `add` 改为闭区间语义（旧版多标 1 个元素，窗口触及末尾时 `gpu_flush` 越界 panic） |
| `build_vao` | 0x140909690 + 0x141309060 + 0x1408ed0e0 | 重写。入参是 `(device,&Mesh)`；先建索引缓冲（usage 0x40，名 "Index buffer"），再逐属性建顶点缓冲（usage 0x80，名 "Vertex buffer"）；`Vao{ bindings @0, index_buffer @0x18 }`=0x40 B，`VaoBinding{Arc<str>, Buffer}`=56 B。旧版结构与流程都不对 |
| rhapsody 占位 | — | usage 立即数与 `VkBufferUsageFlagBits` 逐位吻合（0x02/0x10/0x20/0x40/0x80）；`update_buffer_bytes(buffer, data, ranges, offset)` 与 `create_buffer(desc, name, data)` 的形状由调用点实参反推 |

## 一（续）、第二批完成（O＝逐条核对机器码）

| 单元 | 地址 | 结果与旧实现的差异 |
|---|---|---|
| `ShaderUri::parse` | 0x140968960 | 重写。三个反向搜索（`:` → `/` → `.`），输出是 `{path, stem, entry: Option<&str>}`（48 B）；**找不到 `.` 才 panic，没有 `:` 不 panic**（entry=None，指针 0 编码）。旧版把 entry 当必选、`whole/name/entry` 含义全错 |
| `source_hash<&str>` | 0x1408a4300 | `calculate_hash(&(s, &()))`；`calculate_hash`(0x1405bdda0) 就是 `DefaultHasher::new()`（SipHash-1-3, key=0, `str` 写字节后补 0xff），现有 `utils.rs` 的 std 写法已逐位一致，加了测试固定 |
| `as_compute_pipeline<&str>` | 0x1408a4330 | 原函数返回 `ComputePipelineDesc{path: String, entry: String("cs_main"), defines: []}.with_entry(uri.entry)`（0x48 B）；旧 trait 返回的 `ComputeShaderSource{uri, defs}` 并不存在。**未改 trait 返回类型**——它牵连 8 个 `impl AsComputeShaderSource`（taa/dof/bin_lights/…），在无法整 crate 编译时批量改风险大；新增 `compute_pipeline_from_uri` 承载真实逻辑 |
| `impl$0::new_storage_buffer<T>` | 0x14160f920 起 38 个 | 落在 `TypedStorageBuffer::new_storage_buffer(device, Option<&[u8]>)`：`None` → 同款 `BUFFER_LEN of zero` panic；断言 `len % size_of::<T>()==0`、`ptr%4==0`；`BufferDesc::new(len, 0x22, memory=2).alignment(4)`。与 `new` 的差别是 memory=2 |
| `DirectionalLight::look_at` / `set_direction_towards_light` | 0x140b18550 / 0x140b18830 | **`look_at` 是构造函数**（sret），不是 `&mut self` 方法。矩阵 = glam `Mat4::look_at_rh(eye, target, Y)`（逐步对照：`f·eye` 取正、第三列取 −f）。布局 view@0、forward@0x40、position@0x4c、(1,1,1)@0x58、target@0x64、distance@0x70，size 0x80（旧版字段与偏移全错）。`set_direction`：`position = target + normalize(dir)*distance`，target/distance/(1,1,1) 不动 |
| `SunSkyColors::lerp` | 0x140a97650 | `other*t + self*(1-t)`，不是 glam 的 `a+(b-a)*t`（t=1 的舍入不同）；测试锁定端点精确 |
| `rg_buffer_copy_from_slice<u32>` / `clear_rg_buffer` / `clear_rg_color_image` | 0x14090d9a0 / 0x1409ddbb0 / 0x1409dde90 | 新文件 `rg_helpers.rs`。pass 名 "Buffer memcpy"/"Clear buffer"/"Clear image"，访问码 0x34；断言 `len*4 <= buffer.size`；copy 经 `DynamicConstants::allocate` 暂存；clear 回调是 `fill_buffer(buf, None, 0)` / `clear_color_image`。`RenderContext::finish`(0x1409dda50) 只做句柄 drop，不还原 |
| `Mesh` 全家（20 个） | 0x140a07390…0x140a36110 | 重写。布局：indices@0、`source_file: Option<(String,8B)>`@0x18、属性表@0x38（hashbrown+SipHash 密钥）、label@0x68。**`VertexAttributeValues` 判别式 Int=0/Float=1/Float2=2/Float3=3/Float4=4，没有第 6 个变体**。`From<Vec<T>>` 与 `set_attribute<Vec<T>>` 带 `assert!(!v.is_empty())`；取属性缺失 → `unwrap` panic（旧版返回空切片）；类型不符 → "incorrect attribute type"；`add_color` 的 n 是位置属性 len 且不检查类型；**`add_color_self` 是按值 builder，不是“按位置着色”**；`aabb_bounds` 空 → `{+MAX,-MAX}`、否则以首点为初值；`collision_bbx` 返回 `Rectangle2d`(xz) + y 范围而不是 `Aabb3`；`collision_radius_xz` 遍历**索引**；`retain_vertices` 四步顺序与越界 panic、`remapped != u32::MAX` 断言；`append` 不匹配的 panic 消息带 `Discriminant(n)` |
| `ComputeShaderProgram::new` | 0x140937e10 | 参数 `(name, desc 按值, device)`；加载编译产物失败 → unwrap panic；**trace** 级日志 "Creating pipeline {name}"（旧版 debug）；布局 desc@0/pipeline@0x48/name@0x90 = 0xa0 |
| `compile_shader_variants` | 0x1409383b0 | 返回 `Result<[Variant;2], anyhow::Error>`：`get_raster_shader_variants` 给**两份** desc，依次 `load_compiled`，失败返回 Err（第二份失败时丢弃第一份）。旧版“第二个变体可选”是错的 |
| `RasterShaderProgram::new_draw` | 0x140938240 | 失败 unwrap；布局 `{desc@0, name@0x48, variants[2]@0x58}` = 0xd8；第二个形参未被使用 |
| `RasterShaderProgramVariant::get_or_generate` | 0x140938b00 | 变体 = `{Arc<Compiled>@0, RwLock<HashMap<Key(0x20B), Box<RasterPipeline 0x38B>>>@8}`；读锁命中 → 写锁 entry → `Arc` 强引用 +1 → `create_raster_pipeline` 失败 unwrap；命中时传入的 key（按值）被丢弃；返回指向 Box 内的引用。键 = 托管句柄 + u32@0x18 |
| `load_compiled_texture_image` | 0x140917860 | 重写。路径 `compiled-assets/textures/<name>.texture`（旧版 `.bin`）；`ptr%4`（旧版写 8）；魔数/版本断言带路径消息；**文件尾 0x1c 字节**是描述（数据相对偏移、数据长度、extent×3、mip u16(<256)、层数 u16、两个 u8）；旧版的“长度 %8”“mip<0xff”等检查不存在。原版对短文件读裸指针（UB），这里改成带边界检查的 panic |
| `render_config`（小函数） | 0x140968ad0 / 0x140968bb0 / 0x140968f40 / fmt×3 | `FullscreenConfig` 是**枚举**（判别式 1 携带分辨率），不是结构体；`UpscalingPreset` 5 档 = UltraPerformance/Performance/Balanced/Quality/UltraQuality，比例表 1/3、1/2、25/43、2/3、10/13（旧版 `TgsrHalf…` 与数值均为编造）；`ImageQualityPreset` = Performance/Balanced/Quality/UltraQuality/Custom（旧版 Low/Medium/High/Ultra）；`previous` = 判别式−1，0 → None |
| `GraphicsDeviceResource::deref` / `from_preference_order` | 0x140987ca0 / 0x140987cb0 | deref 是 `[self]+0x10`（ArcInner 数据偏移）→ 资源里是 `Arc<GraphicsDevice>`；`from_preference_order` 就是原地 `reverse()` |

**第 0 层 PDB 过程覆盖**：`render::` 下（排除 nani/pass_builder/raster_single/A2）逐一对照 PDB 过程表，全部命中上面两批；
剩余的只有 serde(ron/json) 序列化、`Debug`/`Clone`、drop、bevy 注册样板，属于库/样板，不还原。
`uniform.rs` / `indirect.rs` / `debug_info_buffers.rs` / `adaptive_render_state.rs` / `fsr_plugin.rs` / `mesh_atlas.rs`
在 PDB 里**没有对应的独立过程**（内联或仅单态化），因此没有可核对的机器码，保持原状且在 `AUDIT_REPORT` 里标注为“未验证”。

## 二、对上层的影响（已处理 / 待处理）

- 已按新接口修了 `render/sparse_instance_buffer.rs` 的 3 处调用点（A2 层，**未**重新核对其内部逻辑）：
  `SparseSsbo: StorageElement`，`BUFFER_LEN == CHUNK_COUNT×32`。证据：`MeshAtlasPartInfoGpuInstance` 的 `new` 单态化 BUFFER_LEN=0x6c0=54×32。
- 仍有三个互不一致的 `GraphicsDevice` 占位（`lib.rs`、`rhapsody_contract`、`ssbo::rhapsody_boundary`），并且
  `systems/backend/prepare_frame_constants.rs` 与 `boundary.rs` 另有各自的空壳 `ViewConstants`/`ViewportConstantBuilder`。
  统一它们是 step 0 的一部分，本轮没有动。
- `prepare_frame_constants`（0x140af40b0，3989 B）是 `FrameConstants` 的真实填充者；它还给出 Halton 抖动序列、
  “首帧 prev=当前”等逻辑，属于更高层，已记录入口，未展开。

## 三、第 0 层仍未做 / 范围外

| 项目 | 原因 |
|---|---|
| 把 `AsComputeShaderSource::as_compute_pipeline` 改成返回 `ComputePipelineDesc` 并同步 8 个实现 | 需要整 crate 可编译后再做，属 A4/上层改动 |
| 默认值结构 `render_config::impl$23/impl$39::default`（0x48 / 0xc8 B，含 1920×1080、60.0） | 整块结构的字段名未恢复，只记录了偏移值，未落地 |
| `FrameConstants` 头部 0xf0 字节的字段 | 填充者是 `prepare_frame_constants`（0x140af40b0），属更高层 |
| step 0：统一三个 `GraphicsDevice` 占位、两份空壳 `ViewConstants` | 需整 crate 编译验证 |
| A2 `sparse_instance_buffer.rs` 内部逻辑 | **已重写并核对**（见文末 “A2 第一批”） |
| `raster_single_object`（2.2k/6.4k/2.1k B）| 属 A4，依赖 pass_builder |

## 四、本轮的两个结论性发现

1. **契约里的 O 不等于代码里的 O。** 本轮重写的每一个单元，旧实现都在接口形状上与原函数不符
   （`ShadowedBuffer::new` 的 Vec 所有权、`gpu_flush` 的范围语义、`TypedStorageBuffer::new` 的三种 `BUFFER_LEN`、
   `build_vao` 的整个输入）。仅“编译通过”无法发现这些。
2. **两个独立反编译互相印证是有效的自检手段**：`build` 与 `set_pixel_offset` 分别解码后，
   `set_pixel_offset_agrees_with_build` 用 `assert_eq!` 逐位比较通过，说明矩阵流水线和字段偏移是对的。
3. **“库调用顺序 + 所有权 + 失败处理”才是游戏自有逻辑。** shader/texture/mesh 这几处，旧版的错误几乎都在
   这三件事上：谁 unwrap、谁返回 Err、日志级别、字节顺序、按值还是按引用（例：`get_or_generate` 命中时丢弃传入的 key）。
   这些在“能编译”的前提下完全看不出来，只能靠机器码。
4. **PDB 名字不是证据。** `add_color_self`、`look_at`、`FullscreenConfig`、`UpscalingPreset` 的旧实现都是按名字想象的，
   机器码给出的形态（builder / 构造函数 / 枚举 / 比例表）与名字联想完全不同。

## Step 0 完成：设备与类型占位统一

- `rhapsody_contract::GraphicsDevice` 现在是全 crate 唯一的设备类型，并实现三个边界 trait：
  `ssbo::GraphicsDevice`、`load_texture::TextureDevice`、`shader::ShaderBackend`（均为占位实现，不触 GPU）。
- `Texture` / `TextureDesc` / `CompiledShaderPipeline` / `RasterPipelineDesc` / `RasterPipelineKey` /
  `RasterPipeline` / `ComputePipeline` 只在 `src/rhapsody_types.rs` 定义一份；`lib.rs`、`shader.rs`、
  `load_texture.rs` 再导出。`ComputePipelineDesc` 以 `shader_uri.rs` 为准。
- 删除了 contract 里与 trait 同名但签名不同的内联 `create_*pipeline` / `create_texture`（无外部调用者）。
- `boundary::{FrameConstants, ViewConstants}`、`ViewportConstantBuilder` 改为再导出真实实现。
- 验证：测试台 75 个单测通过；整 crate 仍未编译（缺 `dolly`），`lib.rs` 只过 rustfmt 语法检查。


## A2 第一批：`sparse_instance_buffer.rs`（逐条核对机器码，测试台 88 个单测通过）

核对的符号：`new_storage_buffer` 0x1412e4a90、`take_out_waste` 0x1412e3d00、`gpu_flush` 0x1412e2f10、`hide`/`show`、
`allocation_to_draw_multi_instanced` 0x1412e3a60、`to_draw_multi_instanced` 0x1412e4250、`Writer::push` 0x1412e4450、
`Writer::clear` 0x1412e4760、`drop_in_place<SparseInstanceBufferAllocation>` 0x1405de840、`drop_in_place<Writer>` 0x1405dec30。

| 项 | 旧实现 | 机器码 |
|---|---|---|
| 缓冲布局 | `pending_resource_handles`、`chunk_count`、`slot_meta` 等 9 个字段 | 0x90 B：`RwLock<Vec<u32>>`(空闲栈)@0、`Box<[u8]>`@0x20、active@0x30、**CPU 影子 `Box<[T]>`**@0x40、dirty@0x50、`Arc<Mutex<Vec<AllocationHandle>>>`@0x60、`TypedStorageBuffer`@0x68 |
| `gpu_flush` 数据源 | “持久映射 CPU 指针”（`buffer_cpu_ptr` 占位） | CPU 影子 `self.data`（@0x40/0x48）；原版**不**在 runs 为空时提前返回 |
| `Allocation` | `{ _field0 指针, ids ptr, ids len }`，释放时 `Box::from_raw(_field0)` | 0x20 B `Mutex<Vec<ChunkId>>`（Vec@0、RawMutex@0x18）；`_field0` 实为 Vec 的 capacity |
| 句柄 | `{word, Option<NonNull>}` | `AllocationHandle = Option<Box<Allocation>>`；`SparseInstanceBufferAllocation<T> = {Weak<waste>@0, handle@8}`，`Drop` 时 `upgrade` → 锁 → push |
| `take_out_waste` | 遍历 `allocations: Arc<RwLock<Vec<Allocation>>>` | 遍历 `waste`（`Arc<Mutex<Vec<AllocationHandle>>>`）；空句柄 → `Option::unwrap` panic；ids 逆序压回空闲栈 |
| `Writer` | 裸指针 + 自有 `allocated_chunks: Vec`、`free_chunks: RwLock` | 0x58 B 全借用：`&mut [u8]`、`&[AtomicUsize]`、`MutexGuard<Vec<ChunkId>>`、`&mut [T]`、`&RwLock<Vec>`、`&[AtomicUsize]`、`u32` |
| `push` | 无返回值，空闲栈空时静默 | 返回 bool（true = 无空闲 chunk）；按值整块拷贝 T；越界均为 `index out of bounds` |
| `clear` | 同上（逆序压回、不清 dirty） | 吻合；另外 `byte_map` 为空时 `Option::unwrap` panic |
| draw 区间 | 返回 `(0, 4, len)` 三元组 | 返回 `Vec<(first_instance, instance_count)>`，长度为 0 的 run 不输出 |
| `hide/show` | `expect("hide: null allocation")` + `assert!` | `Option::unwrap` + 数组下标（panic 文案 `index out of bounds`） |

仍为 I（原版内联、无独立符号）：`new_allocation`（建空分配）与 `writer`（建写入器，起始 chunk = 哨兵）。
`SparseSsbo` 改为 `unsafe trait`（影子数组全零初始化要求全零合法），去掉了不存在的 `ELEMENT_SIZE`。
puffin 作用域（函数名、区间数）是库行为，未复刻。

### A2 尚未做：`render/nani/mesh_atlas.rs`
`MeshAtlas::add` 0x140a36200（4040 B）当前仍是 `todo!`。反汇编已确认的骨架：先取 `Vertex_Position`，按名字哈希（RandomState）
查属性表；复制 mesh 属性表（`hashbrown clone`）；比对属性名集合（排序后 memcmp）不一致时 `anyhow!("Mesh attribute mismatch: …")`；
对每个属性 `VertexAttributeValues::append`；`assertion failed: mesh_idx_within_atlas`；`HashMap::insert`（重复名 panic）。
这是下一步的单独工作项。


## A2 完成：`MeshAtlas::add`（0x140a36200，4040 B）

逐条核对机器码（O）：
- 布局：`mesh: Mesh` @0x00（0x78 B）、`layouts: Vec<16B>` @0x78、`parts: HashMap<AtlassedMeshName,u32>` @0x90（items @0xa8）。PDB 里的 `toolbar::mei::layout::ComputedLayout` 是 ICF 别名，元素 = `{index_offset, index_count, radius, 0}`。
- 签名 `add(&mut self, name, &Mesh) -> Result<u32, anyhow::Error>`；调用方 `MeshAtlasLibrary::add`（0x140a371d0 / 0x140a37760）在 Err 上加 "invalid mesh: " 上下文。
- 顺序：空索引 panic `invalid mesh: {name:?}` → 半径（表非空才取 `Vertex_Position`，缺失 expect、类型不符 "incorrect attribute type"）→ `assert!(mesh_idx_within_atlas)`（`parts.len() < 0x40`）→ 顶点基址 = 任一现有属性长度 → 首个 mesh 整体克隆，否则索引 `+base`（先追加）→ 排序属性名比对，不同则 `Mesh attribute mismatch.\n    Existing: {:?}\n    Incoming: {:?}`（索引不回滚）→ 逐属性 `append` → push 布局 → `parts.insert`，重复 panic `Duplicate atlas mesh name: {name:?}`。
- 连带：`Mesh` 增加 `Clone`（原版首个 mesh 走 `hashbrown::clone` + `source_file` 克隆）。
- 测试台 96 个单测通过（新增 8 个 mesh_atlas 测试）。
- 至此 **A1 + A2 全部完成**。仍未核对项见上文 "第 0 层里还没落地的项"。

## A3：`asset_libraries/ssbo_library.rs`（253 个符号 = 6 个泛型函数的单态化）

逐条核对（O）：`BufferState::new` 0x14091eb30、`impl$6::default` 0x14091eb60、`export_from_rg` 0x14091ebb0、
`get_mut<T>` 0x1412098a0、`add_new<T>` 0x1412bc470、`add_new_with_data<T>` 0x1412badf0、`get_rg_buffer<T>` 0x1402b31a0、
`FrozenMap::insert/get`（0x140969bf0 / 0x1405ca870）、`NameHash::compute` 0x140c9cd80、`Ssbo` 三组桩 impl$0/1/2。

- **库 = `FrozenMap<SsboId, Box<BufferState>>`，所有入口取 `&self`**（`get_mut` 取 `&mut`）。
- **`SsboId = NameHash(xxh3_64(T::NAME 短名))`**，不是完整类型名。`tools/country-core-audit/verify_ssbo_id.py` 用原 EXE 的
  xxh3 secret 复算两个立即数（`Cobblestone3RasterStagingSsbo → 0x802ff23dee42ab47`、`NaniNextVisibilityBitmaskSsbo → 0x69cdc275a605bad6`）一致。
- **`BufferState` 0x40 B**：`Box<dyn Ssbo>`@0、完整 `type_name`@0x10（len 0x42 等）、`RwLock<Option<20B 句柄>>`@0x20，`+0x36 == 5` = **None（未导入）**。
  旧版把 5 当“已导出”、把句柄当 u64、把状态当枚举，全错；`export_from_rg` 实际是 `take()` 后 `rg.export(h, 0x37)`。
- `get_rg_buffer`：读锁命中直接返回；None 时**重新查表**、`assert_eq!(name, type_name::<T>())`、`rg.import_unowned(handle, 0x37)`、写锁存入。
- 注册：缓冲先创建、后查重；重复分“同类型”与“短名撞键”两种 panic 文案。
- `Ssbo` trait 4 个方法（vtable +0x18/+0x20/+0x28/+0x30），三组实现分别对应 `TypedStorageBuffer`(+0) / `InstanceBuffer`(+8) / `MappedStorageBuffer`(+0x10) 的缓冲偏移。`sparse_instance_buffer` 改为引用该 trait（补 `buffer()`）。
- 新增 `frozen_map.rs`（`frozen::FrozenMap` 占位，能工作）、`name_hash.rs`（`xxhash_rust::xxh3_64`，依赖 `xxhash-rust =0.8.10`，harness 用 FNV 占位 shim）。
- 删除旧版的臆造项：`StorageBufferType`、`get_dyn`、`add_existing`、`BufferStateKind`、`MappedStorageBuffer<T>` 空壳。
- 测试台 108 个单测通过（新增 12 个 ssbo_library 测试）。

## A3 完成与集成（5 个并行组）

证据分文件：`A3_color.md`、`A3_texture.md`、`A3_mesh.md`、`A3_shader.md`、`A3_res_render.md`；`ssbo_library` 见上节。
- 统一测试台 `tools/country-core-audit/l0-harness`：源文件里的路径用 `@SRC@` 占位，`./run.sh` 生成临时副本并离线运行（不再把本机绝对路径写进仓库）。**264 个单测全部通过**（A1 + A2 + A3）。依赖：真实 `bevy_ecs =0.16.0`（离线缓存可用）、`serde_json =1.0.85`、`slotmap` 与 `xxhash-rust` 的占位 shim。
- 集成时修正：`NameHash` 的 Debug 改为 `NameHash { hash: N }`（原版 `debug_struct_field1_finish`，由 `AtlassedMeshName` 的 panic 文案互证）；`resources/mod.rs` 的 `SpatialResolveOffsetsSsbo` 改为 re-export 真实类型；`Ssbo` vtable 槽序文档降级（+0x20/+0x30 为 O，+0x18/+0x28 为 I）。
- 已知遗留（均为 U/I，不影响已核对函数）：`render/mesh.rs` 的 `source_file` 原版是 Windows `PathBuf`（当前 `SourceFile{path,stamp}` 兼容）；`boundary.rs` 的 `AssetMeshLibrary` 占位待换成真实类型；`PrefabName/PrefabNameRef` 在两处各有占位；`TextureDesc::from(preset)`（0xa59890）属于 `resources/texture_presets.rs`，尚未恢复；`rhapsody_contract::GraphicsDevice` 缺 `get_vk_image_view` 等调用面；`hotwatch`/`notify`/`nohash-hasher`/`relative-path` 仍是占位；整个 crate 仍未整体编译（缺 `dolly`）。
