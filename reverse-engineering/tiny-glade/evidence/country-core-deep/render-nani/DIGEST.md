# render-nani 域 DIGEST（tg_rn1）

> 覆盖 `country_core::render/pass_builder/**`（10 文件）与 `render/nani/**`（7 文件），即全 crate 最后一批未章节化文件。
> 章节：`chapters/render/pass_builder/**` 与 `chapters/render/nani/**`（17 章全部就地填充，占位清零）。
> **新反编译数 = 0**：所有函数均已在 B 阶段（B_i_api / B_i_infra / B_i_nani / B_n_world / B_n_sys / B_a4_compute /
> B_a4_draw / B_d_cb_a / B_d_cb_b / B_d_gap / B_x_out）、wp1/wp3 合同与 reconstruction 源码（带地址标注）中有证据，
> 本域全部映射进章节（PLAYBOOK §1/§5：不重复反编译、批量优先）。O=机器码观测，I=推断，U=未知（不编造）。

## 1. 模块清单（SOURCE_MAP-render.json 统计）

| 文件 | 业务 | 内联 | 胶水 | 合计 | 文件作用（详见章节） |
|---|---|---|---|---|---|
| pass_builder/bind_resources.rs | 4 | 1 | 0 | 5 | BindGameResources（bind_color_library / bind_ray_tracing） |
| pass_builder/compute_pass_builder.rs | 117 | 19 | 6 | 142 | ComputePass 400B 声明层 + dispatch 家族 + bind_texture |
| pass_builder/draw_builder.rs | 67 | 13 | 7 | 87 | DrawBuilder 0xf8B draw 声明器（种类 0/1/2/3/4/5/7/8） |
| pass_builder/draw_list.rs | 0 | 1 | 0 | 1 | DrawListItem / EntityDrawParams（0x80B 元组元素） |
| pass_builder/draw_stream.rs | 5 | 7 | 14 | 27 | DrawPacket / RasterPass / DrawStream / TextureBindingKey |
| pass_builder/framebuffer_binding.rs | 1 | 8 | 0 | 9 | ColorAttachment 0x20B / DepthAttachment |
| pass_builder/pass_resource_access.rs | 117 | 7 | 0 | 124 | read/write/concurrent_access_buffer<T>（53/37/9）+ TextureAccess |
| pass_builder/pass_resources.rs | 1 | 0 | 0 | 1 | PassResources::new_from（7 TypeId → 56B） |
| pass_builder/shader_uri.rs | 29 | 17 | 6 | 52 | ShaderUri 解析 + AsComputeShaderSource（18 实现） |
| pass_builder/uniform_binder.rs | 15 | 2 | 1 | 18 | UniformBinder（守卫四道 + 写宽表） |
| nani/buffers.rs | 2 | 4 | 0 | 6 | MAX_DRAWS_PER_LIST 27 项表 + NaniDrawListParams + 载体 |
| nani/draw_ext.rs | 1 | 0 | 0 | 1 | nani_draw（subset→间接绘制分派） |
| nani/gpu_world.rs | 152 | 31 | 19 | 202 | GpuWorld / OwnedGpuWorldSubset / InstanceKey / NaniInstanceData |
| nani/instance_data.rs | 6 | 15 | 1 | 22 | 每类实例 GPU 布局（0x30/0x40/0x60/0x70）+ CurvedHalfTimber 构造器 |
| nani/mesh_atlas.rs | 3 | 1 | 6 | 10 | MeshAtlas::add（≤0x40 part 图集合并） |
| nani/systems.rs | 22 | 0 | 7 | 30 | maintain + 20×nani_update_* + flush（21 系统 CPU 侧） |
| nani/trimesh.rs | 5 | 0 | 3 | 8 | TrimeshBuilder（64 三角形 meshlet） |
| **合计** | **547** | **126** | **71** | **744** | 17 文件 |

## 2. 布局表（O，地址为 B patched EXE cf048e27…）

### 2.1 ComputePass（400B，O）
+0x00 bindings `Vec<RenderPassBinding(0x44)>`；+0x18 access_ranges `Vec<{u32,u8}(8)>`；+0x30 external_bindings `Vec<(0x48)>`；
+0x60 PassBuilder；+0x110/+0x120/+0x136 UniformBinder（+0x136==8 = 无 uniform 块）；+0x138 &着色器反射（hashbrown：键=绑定名哈希 u64，值=(SetIndex,BindingIndex)）；
+0x140 &GraphicsDevice；+0x148 PassResources(56B)。方法按值消费 self（memcpy 0x190 回传）。

### 2.2 DrawBuilder（0xf8B，O）
+0x00 `NoDropOnlyTake<DrawPacket>`（首 u64 niche，2=已取走）；+0x10 access_ranges Vec；+0x18 外部绑定 Vec；
+0x40 种类 u32 + 参数区（+0x44..0x68 随种类解释）；+0xb0 variant u8；+0xb8 uniform 缓冲 ptr/+0xc0 len；
+0xde 状态字节（==8 无 pass）；+0xe0 `&mut RasterPass`（+0x00 draws Vec、+0x18 bindings Vec、+0x60 名字→槽表、
+0x90 PassBuilder、+0x1a8 PassResources、+0x1b8 ssbo 库、+0x1c8 placement 表、+0x1d0 Mutex<DynamicConstants>、
+0x1e0 设备、+0x1f0 world、+0x1f8 视口——旧版 "GameContext" 是误读）；+0xe8 `Arc<CompiledShaderPipeline>`；+0xf0 bind_common 布尔。
DrawPacket 0xb8B：head[2]@0、槽 Vec@0x10/0x28、种类 u32+载荷@0x40、HashMap@0x70、shader@0xa0、光栅状态@0xa8、blend 1.0@0xac、variant@0xb0。

### 2.3 GpuWorld（≥0x14e8B，O，B_n_world §3 推翻旧骨架）
+0x00..+0x1440 = 27×0xc0 `RwLock<Option<GpuWorldSubset>>`（每槽 +0x98/+0xa0 trait 对象 {data_ptr, vtable}）；
+0x1440 `total_instance_count: u64`；+0x1448 保留（roll_instance_counts 写入上帧计数）；+0x1450 `Box<NaniDrawListParams>`(0x70B)；
+0x1458 `TypedStorageBuffer<NaniMeshAtlasPartInfo>`；+0x1480 `TypedStorageBuffer<NaniDrawListInfo>`（4B 元素，初始数据=max_instance_offsets）。
旧骨架的"7 个主 subset 锁数组 / +0x14a0 params 数据"**不存在**。
OwnedGpuWorldSubset（u64 单位索引）：[0..2] Vec<ChunkId>、[3]/[4]/[6] HashMap<InstanceKey,u32>、[9]/[10] AtomicU64 min/max、
[0xb]/[0xc] Vec<InstanceCullData>、[0x12]/[0x13] NaniInstanceData trait 对象、[0x14] instance_count、[0x16] next_instance_index（单调无 freelist）。

### 2.4 关键类型
- **InstanceKey = 12B** `{id:u32, index:u32, kind:u8}`（旧 16B 假设错）；**InstanceCullData = 16B** `{Vec3, f16 radius, u16 mesh_index}`。
- **GpuWorldSubsetKind = 27 变体**（O，0x142aba868/0x142aba940）：Canopy, TreeTrunk, BackgroundTree, SolidVertexColor,
  SolidVertexColorWindow, SolidVertexColorWindowGlass, SolidVertexColorBricks, GothicWindowBricks, RoofTile, RoofFloor,
  Plant, Flower, Bush, WoodenPlank, PreplacedFlower, SolidAlbedoMap, SolidAlbedoMapEmissive, Wren, LanternEmissive,
  Plaster, Cloth, Smoke, BalconyDoor, Uber, CurvedHalfTimber, VelocityOnly, StairsFloor；旧的 Main/Bush/Unknown9..26 全是编造。
- **NaniInstanceData vtable**（O）：+0x18 type_name、+0x20 as_any、+0x28 as_any_mut、+0x30 gpu_flush、+0x38 rg_handle、
  +0x40 import_to_rg、+0x48 export_from_rg、+0x50 copy（copy(dst,src) 目的在前）。
- **NaniDrawListParams 0x70B**：+0x00/+0x18/+0x30/+0x48 四个 Vec<u32>（cmd_offsets/max_instance_offsets/fine_grained_offsets/batch_offsets，各 108 项 = 27×4），
  +0x60/+0x64/+0x68/+0x6c 四个 u32 汇总（= 3788800/71936/2048/8，O 互证）。
- **MAX_DRAWS_PER_LIST 27 值**（O，0x142aa14c0；与 max_instances 两张表逐值相同）：8192,8192,4096,32768,8192,8192,8192,8192,
  262144,40960,1024,53120,9600,131072,512,8192,4096,1024,32768,65536,2048,1024,2048,131072,81920,256,32768。
- **PassResources 56B**（O，0x140909310）：+0x00 AssetShaderLibrary（A）、+0x08 AssetTextureLibrary（C）、+0x10 AssetSsboLibrary（D）、
  +0x18/+0x20 帧级资源（类型 U）、+0x28 Mutex<DynamicConstants>（F）、+0x30 可选资源（B）；查找顺序 A,B,C,D,E,F,G，B 可选。
- **TrimeshBuilder 0x50B**（O）：positions_index Option<usize>@0、chunks Vec@0x10、attribs Vec@0x28（0x18B 记录 {data, elem_size u16, align u8}，无属性名）、
  input_max_index@0x40、vertex_count@0x48；TrimeshChunk 0x20B（address@0、index_count u16@8）。
- **MeshAtlas**（O）：mesh@0（0x78B）、layouts Vec<MeshAtlasPart(16B)>@0x78、parts HashMap@0x90；MeshAtlasPart `{base_index, count, radius, pad}`。

## 3. 方法/系统行为摘要（逐函数详见章节）

- **ComputePass::new<T>**（14 实例，O）：ShaderUri hash → PassResources::new_from → get_compute_by_name（未命中 as_compute_pipeline+ComputeShaderProgram::new+add_compute，命中 clone）→ UniformBinder::new → bind_color_library；2311B（带 PERM_* 源）与 1943B（&str）两种形态。
- **dispatch 模式**（O）：0 组数 / 1 线程总数 / 2 间接 / 3 闭包 / 4 间接+闭包；渲染回调体 0x14169af70；`dispatch_3d` 立即模式 local size 0 → div_by_zero **先于屏障**；模式→Rust 枚举名 U。
- **访问码**（O，B 阶段纠错后）：0x1a compute 读 / 0x37 写 / 0x32 cpu-mapped 写 / 0x02 间接读 / 0x34 传输写 / 0x1f DrawBuilder 读 / 0x1d 采样图像 / 0x36·0x2e 颜色附件 / 0x2f·0x17 深度；`is_write = (a-0x28)<0x10`（0x140c10e10）。同步组：读 1 / 写 0 / 并发写=调用方。
- **read/write/concurrent_access_buffer<T>**（53/37/9 实例 = 3 泛型，O）：get_rg_buffer（资源号=句柄+0x10）→ add_resource_access → 绑定数≥0x100 unwrap → 0x44B RenderPassBinding（read 标签 3 / write 标签 4）；**SSBO 访问不经注册表**（旧版臆造已删）。
- **nani_draw**（O，9 步）：GpuWorld 读锁 → draw_count（Owned/External 变体）→ list_index=kind*4|PASS_VARIANT_TABLE[+0xf0]（[0,0,1,2,3]）→ list_index_to_cmd_offset（max_instance_offsets 表，B_i_nani §2.2 纠错）→ 三间接 SSBO → cmd_slot*20 → count 路径（FORCE_FINE 0x143634d9c || (kind<=1 && !FORCE_NO_FINE 0x143634d9d)）或 raw。
- **systems**：maintain 先删→后建→再改（NaniInstanceDesc 24B {mesh bit0 判别, subset_kind u8@+0x10}；Named 需 part.subset==kind 否则 panic）；20 个 nani_update_* 的 kind 数字与载荷尺寸全部 O（如 PreplacedFlower=0xe/0x30、CurvedHalfTimber=0x18/&mut 直写、VelocityOnly=0x19/0x60 双仿射）；flush 27 槽（stride 0xc0）Owned 才 gpu_flush，+0x1448=+0x1440; +0x1440=0。

## 4. panic / 日志串清单（原文 + 地址，均 O）

| 串 | 地址 | 位置 |
|---|---|---|
| "The instance data for {kind:?} is not {T} but {actual}" | 0x142c76920（三片段） | gpu_world 类型断言 |
| "Subset {:?} is not owned" | — | owned_subset_mut External |
| "NaniInstanceDesc requested mesh {} and shader {:?},\nbut this mesh is not registered for that shader in nani_meshes.ron" | 行 63:21 | maintain Named |
| "Instance limit reached while trying to spawn {desc:?}"（warn-once） | 行 98:13 | maintain 容量满 |
| "Failed to remove GPU instance {:?}"（Trace 级） | 行 0x311 | remove_instance 未命中 |
| "Mutex was poisonned" | — | warn-once 表锁 |
| "The DrawBuilder for {name} was not drawn or discarded." | 0x14089a140 | DrawBuilder::drop |
| "RasterPass dropped without a call to render()" | 0x140896d80 | DrawPassRenderCheck::drop |
| "Raw textures must not be used as rg resources"（unreachable!） | draw_stream.rs:204:9 | &Texture 键转换 |
| "Trying to bind {:?} for writing, but it's not writable" | 0x142aaa898（两段） | bind_texture 只读类型写访问 |
| "TODO?" | 0x142aaa7c8 | bind_texture 外部路径带视图 |
| "assertion failed: !access.is_write()" | — | 外部路径 |
| "Resource does not exist" | — | dispatch 间接模式注册表缺缓冲 |
| "This Handle for PlacementDataSSBO doesn't have a corresponding buffer" | — | draw_instanced_from_placement_data |
| "Max instance count reached for {kind}"（旧合同文本；机器码实为 warn-once 跳过） | — | extend_no_data/create_instances |
| "invalid mesh: {:?}" / "Duplicate atlas mesh name: {:?}" / "Mesh attribute mismatch…" | — | MeshAtlas::add |
| "cast_slice>…" | — | trimesh bytemuck |
| "value out of bounds for field WindowGlassFlags.color_id" | — | glass 系统 bit31 |
| "Oops! This shader handle doesn't have a named shader"（draw_lists 侧） | — | 跨域 |

## 5. 与现有合同/代码的差异（B 阶段对 wp 合同的纠错，均已落章节）

1. **gpu_world 布局**：wp3/contracts/gpu_world.md 的 "+0x1450 7 锁数组 / +0x14a0 params" 错 → B_n_world：27×0xc0 槽 + Box<NaniDrawListParams>@+0x1450 + 两个 TypedStorageBuffer（+0x1458/+0x1480）。
2. **InstanceKey**：wp3 合同 16B {chunk_id, instance_index, sort_key} → 12B {id, index, kind}；16B 的是 InstanceCullData。
3. **buffers 两函数**：wp3 称 `nani_calculate_draw_list_params` 与 `default_value` 反编译重叠（同一代码体）→ B_n_world：两个独立函数（地址区间不重叠），Ghidra 跳表误读。
4. **shader_uri parse**：wp1 合同的 whole/name/entry 三段语义全错 → path/stem/entry（entry 可选、无 '.' → unwrap）。
5. **NaniMeshAtlasPartInfo**：BUFFER_LEN=0x6c0 确认，真实元素大小 U（wp3 未提）；NaniDrawListInfo 4B、初始数据 = max_instance_offsets。
6. **访问码**：wp1 旧值 0x17/0x1a 当"写"、0x02 当"读" → 0x02 是间接读、0x1a 是 compute 读、0x37 写（is_write = 0x28..=0x37）。
7. **draw_builder**：wp1 的 mode/draw_args/GameContext 全误读 → NoDropOnlyTake + &mut RasterPass；draw_* 终结操作无返回值。
8. **bind_resources**：wp1 槽名 "WallColorId"（单数）→ "WallColorIds"（复数，xxh3=0xaf42315d263aa22a）；rt 三槽名 xxh3 复算吻合。
9. **preplaced_flower 载荷**：旧 0x40 → **0x30**；**curved_halftimber 无 cap 掩码**（旧文档错）；隐藏载荷 = y=-100 单位变换（不是 scale=0）。
10. **mesh_atlas**：PDB 的 `ComputedLayout` = 链接器别名（元素就是 {index_offset, index_count, radius, 0}）。
11. **reconstruction 源码侧**：`RenderGraph` 作参数传 nani_draw（A4 PassHost 缺 World 资源访问器，B_i_nani §2）；`impl ResourceLookup for World` = typed gap（恒 None）；`DrawMode(u32)` 保留原判别值（枚举名 U）。

## 6. 跨领域线索（供中央索引关联）

- **gpu_generate_draw_lists**（render_loop 域）消费 `GpuWorld::{mesh_atlas_info_buf, draw_list_info_buf, try_get_subset_mut}`、`GpuWorldSubsetKind::all()`、`ComputePass::dispatch_with<cull_and_bucket::closure_env$1>`（0x14160a2e0）、`dispatch_indirect_with<group_draw_cmd_prefix_sum::closure_env$0>`（0x14160b850）。
- **render_world**（0x1404ec110）消费 `GpuWorld::import_to_rg/export_from_rg`。
- **draw_callbacks_a/b**（draw_passes 域）经 `nani_draw(kind)` 大量调用（kind 立即数见 B_d_cb_a/B_d_cb_b 表：WoodenPlank=0xd、PreplacedFlower=0xe、RoofFloor=9、Bush=0xc、Flower=0xb、Cloth=0x14、StairsFloor=0x1a、LanternEmissive=0x12、BalconyDoor=0x16、Uber=0x17、SolidAlbedoMapEmissive=0x10、Plaster 等）；缺 GpuWorld 时全部在 nani_draw 处 unwrap panic。
- **PassHost**（draw_stream.rs）：`ssbo_buffer_word`（deferred_raster/final_post 用 LanternSsbo）、`rg_resources()`（RG 纹理创建）；真实实现属集成层（world→PassResources/GraphicsDeviceResource）。
- **资源 TypeId 交叉**：`CursorTerrainRaycast`（0x062dfe8f62b6fcc5）、`EraserSizeMultiplier`（0x0ae77a86a2c6cefc）、`EditingHighlight`（0x6efcda1c77e106b4）、`DebugGui`（0x48b321ef779dc301）= bind_brick_erase_highlight 的 A/B/C/D（B_d_cb_b §3.1 证实）。
- **SSBO 类型全集**：read 53 / write 37 / write_concurrent 9 名单在 wp1/contracts/pass_resource_access.md §4；SsboId = xxh3_64(槽名) 全测试复算。
- **共享原语**：`inclusive_prefix_scan_u32_1m`（grass/nani）、`blur_pyramid/rev_blur_pyramid`、`rg_buffer_copy_from_slice`（render_loop mod.rs）、`warn_once`（nani 模块内，游戏自有宏）。

## 7. 错位/纠错（PDB 名与行为不符）

1. **`impl$44::fmt`（0x140a241d0）**：PDB 归 gpu_world，机器码是 `debug_tuple_field1_finish("InstanceKey", …)`（单字段元组派生 Debug），与 12B InstanceKey 对不上——疑为 `IndexCondenser::Condex<InstanceKey>` 的 Debug 或 ICF 别名；**U，未恢复**。
2. **`GpuWorldSubsetKind::shader` / `try_from_shader`**（骨架行 173/270）：B_n_world 证实无机器码依据，reconstruction 已删除；章节标 U（残留）。
3. **`curved_halftimber_mesh_atlas_part_idx`**（instance_data.rs 行 443）：仅 PDB LF_FUNC_ID（内联残留），无机器码依据，已删除。
4. **`impl$14::as_any` 与 `as_any_mut` 同址**（0x141572280 系）：ICF 合并，槽位序号可更正。
5. **`mesh_atlas::impl$10::clone_behavior`（0x140002370）**与 bevy `is_exclusive<FunctionSystem>` 同址（ICF）；render-loop DIGEST 已记，本域复证。
6. **`bind_color_library`**：wp1 曾描述 ComputePass 版带槽表——机器码无槽表（B_i_api §3）；DrawBuilder 版才有。
7. **`nani_draw` 的 `+0xf0` 标志**：B_n_sys 曾以 `self.variant`（+0xb0）代"pass 变体"——正确来源是 `DrawBuilder+0xf0`（bind_common 写的布尔），B_i_nani/i_api 已对齐。

## 8. U 清单（最大项，typed gap，不编造）

1. **`PassResources` +0x18/+0x20 两个资源真名**（E/G；候选 frame_constants/camera/Globals/sampler set，I）与 7 个 TypeId→资源名映射。
2. **`GpuWorldSubsetDesc` 字段**（owned/external/owned_custom 已内联）；**External subset 的载荷与 draw_count 来源**。
3. **除 CurvedHalfTimber 外 19 个实例数据类型的字段布局**（只保证尺寸/对齐/StorageElement 名；字段在 nani_update_* 与 shader 侧）。
4. **`NaniMeshAtlasPartInfo` 真实元素大小**（只确认 BUFFER_LEN=0x6c0）；**`GpuInstance` 字段**。
5. **`NaniUberInstanceFlags` / `WindowGlassFlags` / `CurvedHalfTimberCaps` 位域布局**（modular_bitfield 派生名 O）。
6. **dispatch 模式判别 → Rust 枚举名**（DrawMode(u32) 保留）；**`RenderPassBinding` 判别 2 的含义**；`vk_sync_fork::AccessType` 数值名（只知 is_write 范围）。
7. **`bind_texture` 受管 / TextureFromDisk 外部绑定路径**（记录布局 O，ComputePass 侧未实现）；`bind_texture<&mut Handle>`（0x1408edd00）无对应 Rust 类型。
8. **`DrawPacket.head[2]`（+0/+8）与 +0x28 Vec<u8> 语义**；multi_draw_indirect 各 u64 参数语义；`RasterState` 位名。
9. **`nani_flush_instance_updates` 参数顺序**（I：Res<GraphicsDeviceResource> 位置）；各 gap 组件（BrickColor/NaniSolidTexture/WindowLight/AlbedoMap/PrevTransform/Wren/GothicWindowBrickDeformation/Roof/RoofAnimation/Floor/LanternLightParams/Plaster/ClothlineCloth/NormalMap/Smoke/StairsFloorMeshChunk）真实布局（systems 注释含机器码偏移）。
10. **`startup_nani`**（27 个 subset 的创建/assign_subset 接线，未恢复）；`NaniDrawListParams::list_draw_merge_window_range/subtotal_range` 语义（I）；GPU 侧 cull/bucket 算法（场景后置，wave-2 HLSL 域）。

## 9. 验证说明

- 全部结论来源于：B 阶段 20 份报告（785/634/628/508/325/277… 隔离测试台全过，机器码常量/文案/分支断言）、wp1/wp3 合同、reconstruction 源码（业务逻辑与地址标注一致）。
- 本域未新增反编译（0 个）；未修改 reconstruction 源码（文档与证据 only）。
- 17 个章节文件已就地更新：文件作用、逐函数 O/I/U + 证据链接、内联/胶水注释、每章证据注记（布局/常量/U/纠错）。
