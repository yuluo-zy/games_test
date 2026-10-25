# asset-lib-render 域 DIGEST（country-core asset_libraries/** + render/ 其余文件）

- 日期：2026-10-14。版本：B patched（`workspace/input/tiny-glade.exe`，基址 0x140000000）。专属 database id：`tg_al1`。
- 范围：`chapters/asset_libraries/**`（12 章）+ `chapters/render/{ssbo,shadowed_buffer,sparse_instance_buffer,mesh,shader,
  atmosphere,directional_light,frame_constants,fsr_plugin,load_texture,mod,vao,view_constants,indirect}.rs.md`（14 章）=
  **26 章全部就地更新，占位清零**。
- 方法：**零新反编译**——全部 26 章基于既有证据映射（B 阶段 A3_*/B_d_cb_a/b/B_x_out 审计 + wp2/wp4 合同 + render-gap/
  audio-misc/backend-misc decomp）；仅 4 处 rne-read 字节复核（0x140987cb0、0x140976880、0x14067dd80、0x14003bb80），
  无 MCP 新反编译。GPU 后置：只恢复 CPU 侧。
- 结论标注：O（机器码/反编译实测）/ I（推断，注明依据）/ U（未知，不编造）。

## 1. 模块清单与业务过程数（与 SOURCE_MAP-render.json 一致）

| 文件 | 业务 | 内联 | 胶水 | 主要结论来源 |
|---|---|---|---|---|
| asset_libraries/audio_trigger_library.rs | 2 | 5 | 3 | A3_color §三 |
| asset_libraries/color_library.rs | 5 | 0 | 7 | A3_color §一 |
| asset_libraries/configwatch.rs | 2 | 0 | 2 | A3_shader |
| asset_libraries/mesh_library.rs | 12 | 0 | 4 | A3_mesh |
| asset_libraries/meshwatch.rs | 2 | 1 | 2 | A3_mesh |
| asset_libraries/mod.rs | 2 | 6 | 4 | wp4 asset_libraries + render-gap §1 |
| asset_libraries/placement_data_library.rs | 3 | 2 | 1 | A3_color §二 |
| asset_libraries/shader_library.rs | 6 | 2 | 0 | A3_shader |
| asset_libraries/ssbo_library.rs | **334** | 5 | 4 | wp4 + L0_NOTES + B_x_out + d_cb_a/b |
| asset_libraries/texture_library.rs | 13 | 6 | 8 | A3_texture + render-gap §4.1 |
| asset_libraries/texture_watch.rs | 2 | 0 | 2 | A3_texture |
| asset_libraries/vao_library.rs | 4 | 3 | 3 | A3_mesh |
| render/ssbo.rs | **113** | 14 | 5 | wp2 ssbo + render-gap §2.2/§4.2 |
| render/shadowed_buffer.rs | **39** | 4 | 0 | wp2 shadowed_buffer |
| render/sparse_instance_buffer.rs | **31** | 13 | 9 | wp2 sparse_instance_buffer |
| render/mesh.rs | 26 | 12 | 8 | wp4 mesh |
| render/shader.rs | 6 | 1 | 0 | wp4 shader |
| render/atmosphere.rs | 2 | 0 | 8 | wp4 + render-gap §1 #5 |
| render/directional_light.rs | 2 | 2 | 0 | wp4 |
| render/frame_constants.rs | 1 | 1 | 2 | wp4 + 本批字节复核 |
| render/fsr_plugin.rs | 1 | 0 | 0 | B-focused + 本批字节复核 |
| render/load_texture.rs | 1 | 0 | 1 | wp4 |
| render/mod.rs | 1 | 0 | 3 | reconstruction + 本批字节复核 |
| render/vao.rs | 1 | 2 | 1 | wp2 vao |
| render/view_constants.rs | 2 | 1 | 1 | wp4 |
| render/indirect.rs | 0 | 5 | 0 | wp2 indirect |
| **合计** | **613** | 85 | 78 |（776 总函数） |

泛型单态化归并：`ssbo_library` 334 业务 = **12 个模板族**（as_any/as_any_mut ×38+21+19 同址对、buffer/handle ×8
（3 个唯一地址）、
add_new ×73、add_new_with_data ×9、get_mut ×5、get_rg_buffer ×81、export_from_rg、BufferState::new、Default）；
`render/ssbo` 113 业务 = **6 个模板族**（impl$0 new_storage_buffer ×38、TypedStorageBuffer::new ×47、
gpu_write_ranges_nonoverlapping ×4、impl$4 ×21、cpu_write ×2、impl$7 ×1）；`shadowed_buffer` 39 = new ×22 + gpu_flush ×17；
`sparse_instance_buffer` 31 = 4 类型 × 模板族 + is_null。**全部按族标注，未逐条反编译**。

## 2. 布局表（O 摘要；详见各章）

- **AssetSsboLibrary** = `FrozenMap<SsboId, Box<BufferState>>`；`SsboId = xxh3_64(StorageElement::NAME)`（短名）；
  `BufferState` 0x40 B = `{data: Box<dyn Ssbo> @0, name: &'static str @0x10, rg: RwLock<Option<ImportedBuffer>> @0x20,
  +0x36 字节 5 = None}`；`Ssbo` 虚表 = `[drop,size,align, +0x18 as_any, +0x20 as_any_mut, +0x28 buffer, +0x30 handle]`
  四组实现（TypedStorageBuffer 0x28 / InstanceBuffer 0x30 / MappedStorageBuffer 0x38 / SparseInstanceBuffer 0x90）。
- **TypedStorageBuffer<T>** 40 B：buffer 16 B + length@0x10 + byte_size@0x18（守卫）+ f5@0x20 U；
  **InstanceBuffer<T>** 48 B：instance_count@0 + TSB@8；**MappedStorageBuffer<T>** 56 B：ptr@0 + len@8 + buffer@0x10 + f4..6 U。
- **ShadowedBuffer<T>** 72 B：dirty(start,end AtomicUsize 含端点，MAX 空哨兵) + data(ptr,len) + TSB@0x20。
- **SparseInstanceBuffer<T>** 0x90 B：pending handles RwLock + chunk_count + free_chunks + byte_map（块活跃计数，
  sentinel 0x20）+ active/dirty bitset + slot_meta u32（U 角色）+ allocations Arc + TSB@0x68；
  `SparseInstanceBufferWriter<T>` 0x58 B + current_chunk@0x60；块 = 32 实例；4 类型常量见章。
- **AssetMeshLibrary** 0xd0：Vec<Handle> modified@0 + Arc<Mutex<Vec>>@0x18 + FrozenMap@0x20 + 名字表@0x60 + 路径表@0x98；
  **AssetShaderLibrary** 0xe0：raster FrozenMap@0 + compute FrozenMap@0x40 + raster_names@0x80 + compute_names@0xb8（NoHash）；
  **AssetTextureLibrary** 0xc8：textures Vec@0 + dset@0x18 + by_name@0x30 + presets@0x50 + RandomState@0x70 +
  `std::sync::RwLock<DescriptorMap>`@0x80（pending@0x90、indices@0xa8、next@0xc0，上限 0x1fe0）；
  **AssetPlacementDataLibrary** 0x60 双表；**AssetVAOLibrary** 0x60 双表；**MeshWatch** 0x40 / **ConfigWatch** 0x38 /
  **TextureWatch**：hotwatch + 共享集合（100 ms 延迟）。
- **ColorLibrarySsbo** = `[[f32;4]; 37]` 0x250 B（默认品红 (1,0,1,1)）；37 槽名全表见 color_library 章。
- **Mesh** 0x78：indices Vec@0 + 哨兵@0x18 + attributes HashMap@0x28 + 源文件 u64×3@0x60..0x70；
  `VertexAttributeValues` 判别 0..3（Float/Float2/Float3/Float4/…）。
- **DirectionalLight** ~0x78：view-basis Mat4@0 + position@0x40 + target@0x50 + distance@0x70；
  **SunSkyColors** 48 B 3×Vec4；**DerivedAtmosphere** 4×Vec3；**ViewConstants** ~0x320 矩阵链 + 像素偏移@0x200 + 视口@0x210。

## 3. 方法/系统行为（O/I/U）

- 全部 26 章逐函数 O/I/U 标注完成；**O 覆盖 613 业务中的绝大部分**（族级 O 对单态化），U 项集中在：
  `TypedStorageBuffer::new_arbitrary`/`gpu_write_element` 体（U/I）、`SparseInstanceBuffer::slot_meta` 角色、
  `Allocation.field0`、`Vao` f6–f8、`Mesh` +0x18/+0x20 语义、`VertexAttributeValues` 变体数、`RasterPipelineDesc`
  +0x18、`ComputeShaderProgram` 字段级、`BufferWriteError` 变体、`ViewConstants` 逐列映射、`FrameConstants` 全字段、
  `CompiledTexture` 头字段语义、`TexturePreset` 0x19..0x1f 动态路径、`PlacementDataSsbo` JSON schema、`ActiveSession`
  字节键判定（跨域）等。
- 系统函数（bevy 面）：`update_color_library`（设置/`&AssetSsboLibrary`/`&Arc<GraphicsDevice>`/`&mut ColorLibrarySsbo`，
  ResMut 判别 O）；`transient_meshes` 的 bevy 包装揭示 meshwatch 后端系统参数面
  `(Res<GraphicsDeviceResource>, Res<AssetMeshLibrary>, Res<MeshWatch>, Res<AssetShaderLibrary>, Res<ShaderWatch>,
  Commands)`（O，PDB 泛型名）；`register_required_components<Handle<Vao>>` = derive(Component) 胶水（O，非业务）。

## 4. panic / 日志串清单（原文 + 地址，O；详见各章）

| 地址 | 原文 |
|---|---|
| 0x142ab2208 | `Init Color Library failed: {} entry is missing from ColorLibrarySSBO`（color_library:66） |
| 0x142ab10d0 | `AssetShaderLibrary: the shader name {} is already taken`（shader_library:62:21） |
| 0x142ab1b60 | `AssetTextureLibrary: the name {} is already taken`（texture_library） |
| 0x142ab1c16 区 | `assert!(by_name.insert(..{handle,is_virtual:false}).is_none())` 全文案（texture add） |
| 0x142ab1fa0 | bindless 上限断言全文（REQUIRED_BINDLESS_DESCRIPTOR_COUNT − RESERVED_DESCRIPTOR_COUNT） |
| 0x142aafcd8 | `Mesh {:?} does not exist in the library`（mesh_library get_handle_by_name） |
| 0x142aafe20 | `Asset Mesh library was requested to reload a non-existing mesh path {:?}`（mesh_library） |
| 0x142aae830 / 0x142aae800 | `Mesh is not in the  mesh library`（双空格）/ `Mesh is invalid`（vao_library add） |
| 0x142aae898 / 0x142aae8c0 | `The requested mesh doesn't exist` / `VAO handle is invalid`（vao_library rebuild_vao） |
| — | `AssetPlacementDataLibrary: the placement data from {} has already been loaded`（placement load:93） |
| — | `Couldn't find {}`（placement load:100） |
| — | `AssetTextureLibrary: …` / `Onoz, {name} already exists, and is a real texture…`（create_name_mapping） |
| 0x142aadc80 | `Configwatch: {} has changed`（configwatch closure，info 级别 3） |
| — | `Tried to register the same storage buffer twice: {NAME} (type: {full})` / `…two storage buffers with the same \`NAME\`…`（ssbo add_new） |
| — | `Buffer {NAME} (type: {full}) is not available in the library. Forgot to register it?`（ssbo get_rg_buffer） |
| — | `Excessive data size supplied for buffer {type}`（render/ssbo impl$0） |
| — | `No such texture: {hash}. Available: …` / `Texture not loaded: {hash}`（texture get_by_name） |

## 5. 与现有合同/代码的差异（“代码写 X，证据为 Y”）

1. **BufferState +0x36**：wp4 记 `5 = exported-to-CPU`；L0_NOTES 更正为 **5 = rg None（未导入）**，`get_rg_buffer` None 时 re-import——以 L0_NOTES 为准（reconstruction 已按此落地）。
2. **`Ssbo` 虚表 +0x18**：wp4 记 "get ptr"；L0_NOTES 确认 = `as_any`（返回 `(自身指针, Any 虚表)`）；`update_color_library`/`ivy_leaf_shader`/`indirect_draw_bricks` 调用点与之一致，非冲突。
3. **`frame_constants::should_reset_temporal`**（0x140976880）：wp4 记"空体/寄存器透传"；本批字节复核 = `cmp [rcx+0x80],0; sete al`（11 B）——重标志在 +0x80。
4. **`atmosphere` serialize**：宿主 = `DerivedAtmosphere`（4×Vec3）非 `SunSkyColors`（3×Vec4）——不同型（render-gap §1 #5）。
5. **`fsr_plugin::build`**（0x14003bb80）：1 字节 RET（ICF 与 bevy `Plugin::finish<MainCameraPlugin>` 空实现合并）——骨架"构造/注册/生成待推断"不成立，无独立业务。
6. **`AvailableGraphicsDevices::from_preference_order`**（0x140987cb0）：字节复核 = 0xa0 B 元素成对交换 `reverse()`，reconstruction 已正确。
7. **旧版臆造（A3 阶段已纠，逐条登记在章）**：mesh_library `add(name, mesh)` 签名、reload 顺序、disposed 标志；shader_library 形参与文案；texture_library bindless 结构/上限/分配；color_library 槽偏移与 RON 格式；placement load 不读文件；configwatch/meshwatch/texture_watch 50 ms 与"读文件字节"；audio_trigger 二级 slotmap 与 Option 返回；vao_library add 无设备无校验——全部已按 O 纠正并落码。
8. **`TypedStorageBuffer::new` 0xa59890 归位**：render-gap 纠错 `From<TexturePreset> for TextureDesc`（texture_presets.rs），`create_preset_textures` 落码应替换 `desc_of` 闭包。
9. **shadowed_buffer/sparse_instance_buffer**：与 wp2 合同逐条一致（脏区间含端点、byte_size 守卫、(0,4,count) 三元组、slot_meta 不编造）。
10. **reconstruction 现状**：26 个源文件均无 `todo!/unimplemented!`（grep 证实）；本章仅文档登记，**未改源码**。

## 6. 跨领域线索

- **全局 HANDLE_COUNT** `0x143634cd0`：mesh_library/vao_library/shader_library/ssbo_library/placement_data 共用（`lock xadd`）。
- **SsboId = xxh3_64(槽名)** 由 d_cb_a/b 测试复算（"Boats"/"Small lowpoly flowers"/"Garden Stone"/"HalftimberColorId"/"Sparkle"/"Firefly_Vis_Buffer" 等标签）；`"ColorLibrary"`→0xef2a68c12cde3f0d、`"WallColorIds"`→0xaf42315d263aa22a。
- 消费链：`pass_resource_access::read/write_access_buffer<T>`（A4）→ `get_rg_buffer<T>`；d_cb_a/b 69 回调（SSBO/预设/磁盘绑定序）；`update_color_library` ← `systems::backend::configwatch`（0x14085b1c0）；`rebuild_vaos`（0x140864bf0）← mesh_library.modified；`trigger_audio_system`（0x1409eafa0）← AudioTriggerLibrary + `Aabb3::intersect_segment`。
- TypeId（d_cb_a §2）：`Handle<Mesh>` 组件 0x877acce28b84e29f、`AssetSsboLibrary` 资源 0x8226cc786e679355、`AssetMeshLibrary` 资源 0xd455e4bbfd4f066e（均真实类型在场）。
- `0x14067dd80` 与 `AppSteamIntegration::client`（audio-misc）ICF 合并（`mov rax,rcx`）；`0x140949170` 与 `sync_point_count` 同址。
- 进程级布尔（render-gap §3.4）：`*0x14362c0d8` = freeze_occlusion_culling、`*0x14362c098` = demo 开关——本域缓冲创建/刷新路径的守卫。

## 7. 错位 / 纠错汇总（机器码为准）

| 地址 | PDB/sysres 名 | 实际（O） |
|---|---|---|
| 0x14003bb80 | fsr_plugin `plugin_impl::impl$0::build` | 1 B RET，与 bevy `Plugin::finish<MainCameraPlugin>` ICF 合并；无独立业务 |
| 0x140002450 | mesh_library `transient_meshes`（及 bevy `component_access`、`UiState::active_widget_state`） | `lea rax,[rcx+0x18]; ret`——三 PDB 名 ICF 共址；transient_meshes = `&Arc<Mutex<Vec<_>>>` 访问器 |
| 0x140002370 | shader `impl$10::clone_behavior` / bevy `is_exclusive<meshwatch 系统>` | 5 B 桩（ICF），bevy 系统包装胶水 |
| 0x140976880 | `should_reset_temporal` | `[+0x80]==0 → bool`（非空体透传） |
| 0x140949170 | `VertexAttributeValues::array_length` | 5 B 返回 Vec len；别名 `sync_point_count`（ICF） |
| 0x14067dd80 | ssbo `impl$0::buffer`/`handle` | `mov rax,rcx; ret`（缓冲 @+0）；与 `AppSteamIntegration::client` ICF |
| 0x140322a10 | atmosphere `serialize` | 宿主 `DerivedAtmosphere`（4×Vec3），非 `SunSkyColors` |
| 0x140a59890 | texture_presets `impl$1::from` | `From<TexturePreset> for TextureDesc`（render-gap 纠错；非 `From<u8>`） |

## 反编译清单（本域）

- 新反编译：**0**（MCP）。字节复核（rne-read）：0x140987cb0（reverse 循环）、0x140976880（重标志）、0x14067dd80（buffer 桩）、0x14003bb80（RET 桩）。
- 复用的既有 decomp：audio-misc/backend-misc/camera-time/ui-input/session-save/collision-eco 各域 decomp（0x14067dd80、0x140a439a0、0x14093b530、0x14092d8d0、0x140b18f60、0x140002370、0x140002450、0x14003bb80）+ B 阶段反汇编（A3_*、B_d_cb_a/b、B_x_out、B-focused 14003bb80）。
