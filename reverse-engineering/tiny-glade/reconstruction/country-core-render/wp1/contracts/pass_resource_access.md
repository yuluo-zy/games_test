# pass_resource_access.rs 业务契约

标注：**O**=反编译直接支持；**I**=调用链/类型关系组合；**U**=未恢复。

## 1. `read_access_buffer<T>` / `write_access_buffer<T>` / `write_access_buffer_concurrent<T>`（O）

通用辅助：把游戏 SSBO 类型 T 解析为 RG 资源句柄并登记读/写/并发写访问。实例地址：
- read：0x140970620（AtomicCounterBuffer）… 0x140973620（RockyTerrainTriangleSSBO）共 53 个；
- write：0x140973720 … 0x140975b20 共 37 个；
- write_concurrent：0x140975c20 … 0x1409764a0 共 9 个。

流程（read 例，0x140970620；write 0x140975920 仅访问类型不同，size 差 14B 因返回 access_index 与 concurrent 分支）：

1. `AssetSsboLibrary::get_rg_buffer<T>()` — 按 T 类型取 RG buffer 句柄。
2. `PassBuilder::add_resource_access<PassResources>(resource_index, access_type, sync_group)`。
3. `uVar2 = bindings.len()`；若 ≥ 0x100 → `unwrap_failed`（绑定上限 256）。
4. `rhapsody::rg::pass_api::impl$5::bind()` → 0x44B `RenderPassBinding`。
5. 追加到 bindings Vec（{cap,ptr,len}，元素 0x44B）。
6. write 变体返回 access_index（u8），供调用方继续追加 access_range。

`write_access_buffer_concurrent`（0x1409764a0）：同 write，但 `add_resource_access` 使用并发写访问类型（sync_group 不同）。

这些辅助函数不是 `ComputePass::read_storage_buffer` 的直接别名：ComputePass 的 `read_rg_storage_buffer`（RG 句柄路径）自行解析注册表；`read_access_buffer<T>` 经 SSBO 库按类型解析。

## 2. `ssbo_to_rg_handle`（I，inline，56 个宿主）

内联辅助：把 `&T`（游戏 SSBO 包装）转成 `rhapsody::rg::resource::Handle`。宿主为 `read_access_buffer<T>` 与 ComputePass/DrawBuilder 的 `write_storage_buffer<T>` 等。真实签名（取句柄/取 hash）U。

## 3. 纹理访问 trait：`impl$0..impl$6` 的 `as_texture` / `dims`（O）

行号锚（enriched source_line）：impl$0 as_texture@37/dims@43（module 549，DoF 通道）；impl$2 as_texture@66/dims@76（module 500，`Texture` 引用）；impl$3 as_texture@83/dims@91（module 589）；impl$6 as_texture@133/dims@141（module 507）。各 impl 对应不同“可当纹理访问”的类型（`&Texture`、`Handle<Texture>`、`ReadOnlyHandle<Texture>`、`WriteTextureUnchecked` 等）。

语义：把 pass 资源声明为纹理访问，供 `bind_texture_to_*` 使用；`as_texture` 返回 `rhapsody::raw::texture::Texture` 引用，`dims` 返回 `Extent`。值/实现类型映射部分 U（PDB 只给 `void()` 占位）。

## 4. 已知 SSBO 类型实例全集（O，PDB 单态化名单，见 evidence）

`AtomicCounterBuffer`、`DrawIndexedIndirectBuffer`、`DrawIndirectBuffer`、`NaniDrawListLengthSsbo`、`NaniNextVisibilityBitmaskSsbo`、`NaniPrevVisibilityBitmaskSsbo`、`NaniDrawListSsbo`、`NaniInstanceIndirectionSsbo`、`NaniInstanceIndirectionAllocatorSsbo`、`NaniFilteredInstanceAllocatorSsbo`、`NaniDrawListOffsetDefaults`、`ColorLibrarySsbo`、`WallColorIdSsbo`、`GateColorIdSsbo`、`RoofColorIdSsbo`、`HalftimberColorIdSsbo`、`RtTrianglesSsbo`、`RtTriangleMetaSsbo`、`RtBvhSsbo`、`Cobblestone{1,2,3}RasterStagingSsbo`、`PathRasterStagingSsbo`、`WaterRasterStagingSsbo`、`GardenRasterStagingSsbo`、`TerrainHeightSsbo`、`VisualTerrainHeightSsbo`、`TerrainRockMaskSsbo`、`WaterMaskSsbo`、`WaterExcludeClutterMaskSsbo`、`GardenMaskSsbo`、`RoofTileSsbo`、`RoofSpireSsbo`、`WoodenPlankSsbo`、`RoofSupportBeamSsbo`、`RockyTerrainVertexSSBO`、`RockyTerrainTriangleSSBO`、`MouseDepthValueSsbo`、`MouseTerrainRaycastSsbo`、`BushCpuSsbo`、`DofAutofocusSsbo`、`DofFocusReadbackSsbo`、`ShadowMaskMeanSsbo`、`ShadowMaskHistogramSsbo`、`PebblePlacementSsbo`、`GrassBladeDataSsbo`、`StairsTransformSsbo`、`LilypadSsbo`、`LilypadFlowerSsbo`、`ReedSsbo`、`LeafParticleSsbo`、`ParticleEmitterSsbo`、`FireflySsbo`、`FireflyVisibilitySsbo`、`ButterflySsbo`、`SheepAnimationSsbo`、`LanternSsbo`、`SpatialResolveOffsetsSsbo`、`GroundedBuildingsSsbo`、`FlowerSsbo`、`Flower03Ssbo`、`BushSsbo`、`GardenStoneSsbo`、`BoatSsbo`、`SparkleSsbo`、`IvyLeafSsbo`、`IvyFlowerSsbo`、`IvySegmentSsbo`、`BrickTransformSsbo`、`BrickDrawListSsbo`、`BrickNextVisibilityBitmaskSsbo`、`BrickPrevVisibilityBitmaskSsbo`、`DormerHoleSsbo`、`DormerHoleCountPerCellSsbo`、`ShapeStreamSsbo`、`TextPrimSsbo`、`CustomCursorSsbo`、`StairsCounterSsbo`、`NormalDebugSsbo`、`WaterExcludeClutterMaskSsbo`。

这些结构体归属其它包（WP2 缓冲 / WP3 nani / WP4），WP1 仅引用为泛型参数，不重定义。
