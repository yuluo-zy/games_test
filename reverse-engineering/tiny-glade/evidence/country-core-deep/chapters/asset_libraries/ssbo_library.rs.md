# asset_libraries/ssbo_library.rs

> 文件作用：**类型化 SSBO 注册表**——`AssetSsboLibrary = FrozenMap<SsboId, Box<BufferState>>` + 按类型擦除的
> `Ssbo` trait 虚表族（78 虚表 + sparse 4 = 82）；`SsboId = NameHash(xxh3_64(StorageElement::NAME))`；入口：
> `add_new`/`add_new_with_data`（注册）、`get_rg_buffer`（渲染面导入句柄）、`get_mut`（CPU 面）、`export_from_rg`
> （批量导出）。334 业务 = 12 个模板族单态化，不逐条反编译（本任务按类型族归并标注）。
> 证据：wp4 asset_libraries 合同 §ssbo_library + `evidence/country-core-audit/L0_NOTES.md` "A3 ssbo_library" +
> `reconstruction/country-core/src/asset_libraries/ssbo_library.rs`（O 全量核对）+ B_x_out（get_cpu_shadow_words 追加）+ d_cb_a/b（SsboId 复算）。

函数总计 343：业务 334 / 内联无独立符号 5 / 派生胶水 4

## 布局表（O）

| 结构 | 布局 | 证据 |
|---|---|---|
| `AssetSsboLibrary` | `FrozenMap<SsboId, Box<BufferState>>`（+8 借用字节、+0x10 hashbrown：ctrl/mask/items/RandomState）+ storage-buffer 描述符（per-T vtable）+ name 串（断言用） | impl$6::default 0x14091eb60（空表）；wp4 |
| `SsboId` | u64 = `NameHash(xxh3_64(T::NAME))`——**短名**（StorageElement::NAME，如 `"Cobblestone3RasterStagingSsbo"`→0x802ff23dee42ab47、`"NaniNextVisibilityBitmaskSsbo"`→0x69cdc275a605bad6，verify_ssbo_id.py 用原 EXE XXH3 secret 复算；`"ColorLibrary"`→0xef2a68c12cde3f0d、`"WallColorIds"`→0xaf42315d263aa22a） | L0_NOTES / A3_color / render-gap |
| `BufferState` | 0x40 B：`data: Box<dyn Ssbo>` @0x00（胖指针）、`name: &'static str` @0x10（**完整** `type_name::<T>()`，如 ColorLibrarySsbo 长 0x3e）、`rg: RwLock<Option<ImportedBuffer>>` @0x20（锁字 @0x20、20 B 句柄 @0x28、**+0x36 字节 = 5 表示 None**） | BufferState::new 0x14091eb30；L0_NOTES |
| `Ssbo` trait 虚表 | `[drop, size, align, +0x18 as_any, +0x20 as_any_mut, +0x28 buffer, +0x30 handle]`；四组实现：`impl$0`=TypedStorageBuffer(0x28)、`impl$1`=MappedStorageBuffer(0x38)、`impl$2`=InstanceBuffer(0x30)、`impl$12`=SparseInstanceBuffer(0x90) | L0_NOTES / i_api / wp2 sparse |

## 类型族归并总表（334 业务 → 12 族）

| 族 | 实例数 | 代表地址 | 逆向结论 | 证据 |
|---|---|---|---|---|
| `impl$0::as_any` / `as_any_mut<T>`（TypedStorageBuffer<T>） | 38 + 38 | 0x14160f470..0x14285a3e0 | **O**：`mov rax,rcx; lea rdx,[Any 虚表]`（as_any 与 as_any_mut **同址 ICF 合并**——两者均返回自身指针 + Any vtable） | L0_NOTES / i_api |
| `impl$1::as_any` / `as_any_mut<T>`（MappedStorageBuffer<T>） | 21 + 21 | 0x14160f690..0x14160f7d0 | **O**：同上 | L0_NOTES |
| `impl$2::as_any` / `as_any_mut<T>`（InstanceBuffer<T>） | 19 + 19 | 0x14160f7e0..0x14160f900 | **O**：同上 | L0_NOTES |
| `impl$0::buffer` / `handle<T>` | 4（2 buffer + 2 handle，1 个唯一地址） | 0x14067dd80（ICF 合并） | **O**：`mov rax,rcx; ret`——缓冲在 +0（与 `AppSteamIntegration::client` 同址 ICF） | 本批 rne-read 字节 + audio-misc §2.11 |
| `impl$1::buffer` / `handle<T>` | 2（1 + 1，1 个唯一地址） | 0x140a439a0 | **O**：`lea rax,[rcx+0x10]`（内嵌 Buffer @+0x10） | L0_NOTES |
| `impl$2::buffer` / `handle<T>` | 2（1 + 1，1 个唯一地址） | 0x14160f910 | **O**：`lea rax,[rcx+8]`（内嵌 Buffer @+8） | L0_NOTES |
| `BufferState::new` | 1 | 0x14091eb30 | **O**（36 B）：`(ptr, size, data, len)` 填充 0x40 B 状态 | wp4 |
| `AssetSsboLibrary::add_new_with_data<T>` | 9 | 0x1412badf0 起 | **O**：`T::new_storage_buffer(device, Some(data))`（`len*size_of::<T>()` 字节）→ BufferState → FrozenMap::insert | wp4 / L0_NOTES |
| `AssetSsboLibrary::add_new<T>` | 73 | 0x1412be270..0x1425f1640 | **O**：`T::new_storage_buffer(device, None)` → BufferState → insert；撞键 panic（同名/同 id 两文案） | wp4 / L0_NOTES |
| `AssetSsboLibrary::get_mut<T>` | 5 | 0x1412098a0 起 | **O**：表空/键缺失 → None；命中 `assert_eq!(name, type_name::<T>())` 经 vtable +0x20 as_any_mut 取回具体缓冲 | wp4 / L0_NOTES |
| `AssetSsboLibrary::get_rg_buffer<T>` | 81 | 0x1402b31a0 起 | **O**：查表（缺失 panic `"Buffer {NAME} … not available in the library. Forgot to register it?"`）→ 读锁 rg Some 直接返回 → None：放锁**再查一次表**（unwrap）+ 名字断言 + `rg.import_unowned(state.data.handle(), 0x37)` + 写锁存 Some | wp4 / L0_NOTES |
| `AssetSsboLibrary::export_from_rg` | 1 | 0x14091ebb0 | **O**：逐 BufferState 取写锁 `rg.take()`；原先 Some 才 `rg.export(handle, 0x37)` | wp4 / L0_NOTES |

### 族明细 1：`impl$0`（TypedStorageBuffer<T>，38 类型；as_any/as_any_mut 同址）

```
ColorLibrarySsbo@0x14160f660  BrickDrawListSsbo@0x14160f480  BrickNextVisibilityBitmaskSsbo@0x14160f650
BrickPrevVisibilityBitmaskSsbo@0x14160f5d0  NaniDrawListLengthSsbo@0x14160f4c0  NaniDrawListSsbo@0x14160f620
NaniFilteredInstanceAllocatorSsbo@0x14160f600  NaniInstanceIndirectionAllocatorSsbo@0x14160f4b0
NaniInstanceIndirectionSsbo@0x14160f570  NaniListDrawCmdOffsets@0x14160f5b0  NaniNextVisibilityBitmaskSsbo@0x14160f590
NaniPrevVisibilityBitmaskSsbo@0x14160f5c0  GateColorIdSsbo@0x14285a3c0  HalftimberColorIdSsbo@0x14285a3e0
WallColorIdSsbo@0x14285a3b0  RoofColorIdSsbo@0x14285a3d0  SpatialResolveOffsetsSsbo@0x14160f670
RockyTerrainTriangleSSBO@0x14160f470  RockyTerrainVertexSSBO@0x14160f4d0  RippleSourceSsbo@0x14160f550
ShadowMaskHistogramSsbo@0x14160f510  ShadowMaskMeanSsbo@0x14160f490  BushSsbo@0x14160f4e0  FlowerSsbo@0x14160f610
DofAutofocusSsbo@0x14160f530  GrassBladeDataSsbo@0x14160f680  LeafParticleSsbo@0x14160f640  ParticleEmitterSsbo@0x14160f4a0
PebblePlacementSsbo@0x14160f5f0  StairsTransformSsbo@0x14160f580  CustomCursorSsbo@0x14160f520  ShapeStreamSsbo@0x14160f500
TextPrimSsbo@0x14160f5e0  AtomicCounterBuffer@0x14160f5a0  DispatchIndirectBuffer@0x14160f560
DrawIndexedIndirectBuffer@0x14160f4f0  DrawIndirectBuffer@0x14160f540  SheepAnimationSsbo@0x14160f630
```

### 族明细 2：`impl$1`（MappedStorageBuffer<T>，21 类型）

```
NormalDebugSsbo@0x14160f710  MouseDepthValueSsbo@0x14160f6f0  GardenMaskSsbo@0x14160f7a0  TerrainHeightSsbo@0x14160f6c0
VisualTerrainHeightSsbo@0x14160f790  TerrainRockMaskSsbo@0x14160f7b0  WaterExcludeClutterMaskSsbo@0x14160f690
WaterMaskSsbo@0x14160f720  Cobblestone1RasterStagingSsbo@0x14160f6d0  Cobblestone2RasterStagingSsbo@0x14160f700
Cobblestone3RasterStagingSsbo@0x14160f740  GardenRasterStagingSsbo@0x14160f770  PathRasterStagingSsbo@0x14160f7d0
WaterRasterStagingSsbo@0x14160f730  StairsCounterSsbo@0x14160f6a0  DofFocusReadbackSsbo@0x14160f760
MouseTerrainRaycastSsbo@0x14160f780  BushCpuSsbo@0x14160f6b0  ButterflySsbo@0x14160f6e0  FireflySsbo@0x14160f750
FireflyVisibilitySsbo@0x14160f7c0
```

### 族明细 3：`impl$2`（InstanceBuffer<T>，19 类型）

```
NaniDrawListOffsetDefaults@0x14160f8c0  LanternSsbo@0x14160f860  RoofSpireSsbo@0x14160f8d0
RoofSupportBeamSsbo@0x14160f8f0  RtBvhSsbo@0x14160f880  RtTriangleMetaSsbo@0x14160f8b0  RtTrianglesSsbo@0x14160f7f0
DormerHoleCountPerCellSsbo@0x14160f890  DormerHoleSsbo@0x14160f820  IvyFlowerSsbo@0x14160f8a0  IvyLeafSsbo@0x14160f850
IvySegmentSsbo@0x14160f7e0  Flower03Ssbo@0x14160f900  GardenStoneSsbo@0x14160f800  LilypadFlowerSsbo@0x14160f870
LilypadSsbo@0x14160f830  ReedSsbo@0x14160f810  GroundedBuildingsSsbo@0x14160f8e0  BoatSsbo@0x14160f840
```

### 族明细 4：`add_new_with_data<T>`（9 类型，按值初始化数据）

```
ColorLibrarySsbo@0x1412bbcf0  NaniDrawListSsbo@0x1412bb2f0  NaniInstanceIndirectionSsbo@0x1412bb070
NaniListDrawCmdOffsets@0x1412bbf70  NaniNextVisibilityBitmaskSsbo@0x1412badf0  NaniPrevVisibilityBitmaskSsbo@0x1412bb570
RockyTerrainTriangleSSBO@0x1412bba70  RockyTerrainVertexSSBO@0x1412bc1f0  SheepAnimationSsbo@0x1412bb7f0
```

### 族明细 5：`add_new<T>`（73 类型；除 add_new_with_data 9 型 + get_rg_buffer 的 81 型之外按域分列）

```
墙/屋顶域:  BrickDrawListSsbo@0x1412c3050  BrickNextVisibilityBitmaskSsbo@0x1412c3d00  BrickPrevVisibilityBitmaskSsbo@0x1412bc6f0
  BrickTransformSsbo@0x1412bf670  RoofTileSsbo@0x1412bfba0  RoofSpireSsbo@0x1412bdd70  RoofSupportBeamSsbo@0x1412c2650
  WoodenPlankSsbo@0x1412c6000  DormerHoleSsbo@0x1412bf170  DormerHoleCountPerCellSsbo@0x1412c3f80
nani 域:    NaniDrawListLengthSsbo@0x1412bd0f0  NaniDrawListOffsetDefaults@0x1412bdff0  NaniFilteredInstanceAllocatorSsbo@0x1412bd870
  NaniInstanceIndirectionAllocatorSsbo@0x1412c5380
地形/栅格域: NormalDebugSsbo@0x1412c05d0  MouseDepthValueSsbo@0x1412c4c00  GardenMaskSsbo@0x1412c5600
  SpatialResolveOffsetsSsbo@0x1412bd370  TerrainHeightSsbo@0x1412c6a30  VisualTerrainHeightSsbo@0x1412c5b00
  TerrainRockMaskSsbo@0x1412be9f0  WaterMaskSsbo@0x1412c2dd0  WaterExcludeClutterMaskSsbo@0x1412c1750  RippleSourceSsbo@0x1412c4200
  Cobblestone1RasterStagingSsbo@0x1412bf3f0  Cobblestone2RasterStagingSsbo@0x1412c4e80  Cobblestone3RasterStagingSsbo@0x1412bdaf0
  GardenRasterStagingSsbo@0x1412c6cb0  PathRasterStagingSsbo@0x1412c6f30  WaterRasterStagingSsbo@0x1412be770
  StairsCounterSsbo@0x1412c0850
rt/常春藤域: RtBvhSsbo@0x1412c0d50  RtTriangleMetaSsbo@0x1412c00d0  RtTrianglesSsbo@0x1412c3550  LanternSsbo@0x1412c14d0
  IvyFlowerSsbo@0x1412bf920  IvyLeafSsbo@0x1412bc970  IvySegmentSsbo@0x1412c28d0
compute/draw 域: ShadowMaskHistogramSsbo@0x1412c5100  ShadowMaskMeanSsbo@0x1412c0fd0  BushSsbo@0x1412bce70
  FlowerSsbo@0x1412c1250  Flower03Ssbo@0x1412c4480  GardenStoneSsbo@0x1412c4980  LilypadSsbo@0x1412c32d0
  LilypadFlowerSsbo@0x1412c4700  ReedSsbo@0x1412c5d80  DofAutofocusSsbo@0x1412bec70  DofFocusReadbackSsbo@0x1412bcbf0
  GrassBladeDataSsbo@0x1412c19d0  LeafParticleSsbo@0x1412c67b0  ParticleEmitterSsbo@0x1412c62b0
  MouseTerrainRaycastSsbo@0x1412bfe50  PebblePlacementSsbo@0x1412c5880  GroundedBuildingsSsbo@0x1412c0350
  BushCpuSsbo@0x1412c1ed0  BoatSsbo@0x1412c23d0  StairsTransformSsbo@0x1412c37d0  CustomCursorSsbo@0x1412beef0
  ShapeStreamSsbo@0x1412be4f0  TextPrimSsbo@0x1412c2b50  AtomicCounterBuffer@0x1412bd5f0  DispatchIndirectBuffer@0x1412c2150
  DrawIndexedIndirectBuffer@0x1412c6530  DrawIndirectBuffer@0x1412be270  ButterflySsbo@0x1412c0ad0  FireflySsbo@0x1412c1c50
  FireflyVisibilitySsbo@0x1412bc470  SparkleSsbo@0x1412c3a50
color_id 域: GateColorIdSsbo@0x1425f0ef0  HalftimberColorIdSsbo@0x1425f1160  WallColorIdSsbo@0x1425f1640  RoofColorIdSsbo@0x1425f13d0
```

### 族明细 6：`get_mut<T>`（5 类型）

```
BrickTransformSsbo@0x1412c71b0  RoofTileSsbo@0x141a3adb0  WoodenPlankSsbo@0x141209a70  SparkleSsbo@0x1412098a0
ParticleEmitterSsbo@0x1412c7380
```

### 族明细 7：`get_rg_buffer<T>`（81 类型；与 add_new/add_new_with_data 的类型集一致，仅少 NormalDebugSsbo 外的未注册型）

```
ColorLibrarySsbo@0x1402b3e60  BrickDrawListSsbo@0x1412cbea0  BrickNextVisibilityBitmaskSsbo@0x1412d04c0
BrickPrevVisibilityBitmaskSsbo@0x1412d0e50  BrickTransformSsbo@0x1412c9530  NaniDrawListLengthSsbo@0x1412d0b20
NaniDrawListOffsetDefaults@0x1412d17e0  NaniDrawListSsbo@0x1412cf1a0  NaniFilteredInstanceAllocatorSsbo@0x1412ca1f0
NaniInstanceIndirectionAllocatorSsbo@0x1412d3490  NaniInstanceIndirectionSsbo@0x1412ccb60  NaniNextVisibilityBitmaskSsbo@0x1412d1180
NaniPrevVisibilityBitmaskSsbo@0x1412ce810  NormalDebugSsbo@0x1412d2e30  GateColorIdSsbo@0x1412c7ee0
HalftimberColorIdSsbo@0x1412ca520  WallColorIdSsbo@0x1412cc500  RoofColorIdSsbo@0x1412cfe60  MouseDepthValueSsbo@0x1412d5e00
GardenMaskSsbo@0x1412cfb30  LanternSsbo@0x1412d5470  SpatialResolveOffsetsSsbo@0x1412d57a0  RockyTerrainTriangleSSBO@0x1412cc1d0
RockyTerrainVertexSSBO@0x1412cb840  RoofSpireSsbo@0x1412cdb50  RoofTileSsbo@0x1412c9b90  RoofSupportBeamSsbo@0x1412cb510
WoodenPlankSsbo@0x1412cce90  RtBvhSsbo@0x1412d4150  RtTriangleMetaSsbo@0x1412d1e40  RtTrianglesSsbo@0x1412c7bb0
SparkleSsbo@0x1412d2b00  TerrainHeightSsbo@0x1412cd4f0  VisualTerrainHeightSsbo@0x1412d37c0  TerrainRockMaskSsbo@0x1412d4480
DormerHoleCountPerCellSsbo@0x1412cee70  DormerHoleSsbo@0x1412d4ae0  RippleSourceSsbo@0x1412c8210  WaterExcludeClutterMaskSsbo@0x1412d3e20
WaterMaskSsbo@0x1412cc830  Cobblestone1RasterStagingSsbo@0x1402b3b30  Cobblestone2RasterStagingSsbo@0x1402b4190
Cobblestone3RasterStagingSsbo@0x1402b31a0  GardenRasterStagingSsbo@0x1402b44c0  PathRasterStagingSsbo@0x1402b3800
WaterRasterStagingSsbo@0x1402b34d0  StairsCounterSsbo@0x1412c9ec0  IvyFlowerSsbo@0x1412c9860  IvyLeafSsbo@0x1412c8ba0
IvySegmentSsbo@0x1412cd1c0  ShadowMaskHistogramSsbo@0x1412ca850  ShadowMaskMeanSsbo@0x1412cab80  BushSsbo@0x1412d07f0
Flower03Ssbo@0x1412c8ed0  FlowerSsbo@0x1412cd820  GardenStoneSsbo@0x1412caeb0  LilypadFlowerSsbo@0x1412d24a0
LilypadSsbo@0x1412c7880  ReedSsbo@0x1412cf4d0  DofAutofocusSsbo@0x1412d0190  DofFocusReadbackSsbo@0x1412cf800
GrassBladeDataSsbo@0x1412d5140  LeafParticleSsbo@0x1412d14b0  ParticleEmitterSsbo@0x1412d47b0  MouseTerrainRaycastSsbo@0x1412d27d0
PebblePlacementSsbo@0x1412c8870  GroundedBuildingsSsbo@0x1412ce1b0  BushCpuSsbo@0x1412c8540  BoatSsbo@0x1412d3af0
StairsTransformSsbo@0x1412c9200  CustomCursorSsbo@0x1412c7550  ShapeStreamSsbo@0x1412cbb70  TextPrimSsbo@0x1412d1b10
AtomicCounterBuffer@0x1412cb1e0  DispatchIndirectBuffer@0x1412d2170  DrawIndexedIndirectBuffer@0x1412d5ad0
DrawIndirectBuffer@0x1412d6130  ButterflySsbo@0x1412d4e10  FireflySsbo@0x1412ceb40  FireflyVisibilitySsbo@0x1412d3160
SheepAnimationSsbo@0x1412ce4e0
```

## 内联/无独立符号函数

- `AssetSsboLibrary::new`（行 148）— **I**：Default 同形（空表构造）
- `AssetSsboLibrary::add_existing`（行 174）— **I**：登记既有 BufferState（无独立符号）
- `AssetSsboLibrary::get_dyn`（行 206）— **I**：FrozenMap 按 SsboId 查 + vtable 取（内联于 get<T> 族）
- `AssetSsboLibrary::get_mut_dyn`（行 214）— **I**：同上可变版
- `AssetSsboLibrary::get`（行 242）— **I**：FrozenMap 查找 + vtable +0x18 as_any（内联）
- `Ssbo::cpu_shadow_words`（B_x_out 追加）— **O**（调用点证据）：只有 4 字节元素 `ShadowedBuffer<T>` 返回 `Some(&[u32])`（TerrainRockMaskSsbo 影子）；`nani_update_preplaced_flower` 经 vtable +0x18 取回 `(ptr,len)` u32 数组，NULL → unwrap panic

## 派生胶水（4 项：impl$10::hash、impl$12::eq、impl$6::default 0x14091eb60、impl$7::get_rg_buffer::closure$0）

- `impl$6::default`（0x14091eb60）= derive(Default) 展开（空 FrozenMap + name，O，L0_NOTES）。
- hash/eq = `SsboId`/`BufferState` 的 NoHashHasher 与相等比较（derive/外部库，不补）。

## 纠错/差异

- **wp4 合同与 L0_NOTES 的 BufferState 状态字节**：wp4 记 `+0x36 = 5 表示 exported-to-CPU`；L0_NOTES 更正为 **5 = rg None（未导入）**，`get_rg_buffer` 在 None 时 re-import——以 L0_NOTES 为准（reconstruction 已按此落地）。
- **A3 与 i_api 虚表槽序**：wp4 记 `+0x18 = get ptr`；L0_NOTES 确认 `+0x18 = as_any`（返回 `(自身指针, Any 虚表)`）——两者一致，无冲突。
- 旧版 `get_rg_buffer` 的 "state==5 → re-import" 与 wp4 描述一致（O）。
- **0x14067dd80**：`impl$0::buffer`/`handle` = `mov rax,rcx; ret`（本批字节复核），与 `AppSteamIntegration::client`（audio-misc §2.11）ICF 合并；`impl$1` 0x140a439a0 = `lea rax,[rcx+0x10]`（session-save 证据）；`impl$2` 0x14160f910 = `lea rax,[rcx+8]`。
- **SsboId 断言文案**：`add_new` 撞同名 → `"Tried to register the same storage buffer twice: {NAME} (type: {full})"`；撞短名哈希 → `"Tried to register two storage buffers with the same \`NAME\`…"`；`get_rg_buffer` 缺失 → `"Buffer {NAME} (type: {full}) is not available in the library. Forgot to register it?"`（O，wp4/L0_NOTES）。
- 调试标签（add_new 的 BufferState 名）：`"Firefly_Vis_Buffer"`、`"NaniNextVisibilityBitmaskSsbo"` 等（wp4 O）；`SsboId = xxh3_64(槽名)` 由 d_cb_a/b 测试复算（`"Boats"`、`"Small lowpoly flowers"`、`"Garden Stone"`、`"HalftimberColorId"`、`"Sparkle"`）。

## 跨领域线索

- 渲染面消费者：`pass_resource_access::read/write_access_buffer<T>`（A4）经 `get_rg_buffer<T>` 取句柄；d_cb_a/b 的 69 个 draw 回调、`indirect_draw_bricks`（`SparseInstanceBuffer<BrickTransformSsbo>` 经 +0x68 内嵌 Buffer）、`ivy_leaf_shader`/`roof_spire`/`roof_support_beam`（`as_any` 后读具体对象）、`env36`（SparkleSsbo `to_draw_multi_instanced`）、`update_color_library`（ColorLibrarySsbo，键 0xef2a68c12cde3f0d）。
- `AssetSsboLibrary` TypeId = 0x8226cc786e679355（d_cb_a §2，真实资源在场）。
- `get_cpu_shadow_words<T>`：B_x_out 追加的只读影子入口（TerrainRockMaskSsbo 4 字节元素），flower 系统使用。
