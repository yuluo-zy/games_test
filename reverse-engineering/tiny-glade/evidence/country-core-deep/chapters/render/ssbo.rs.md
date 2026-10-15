# render/ssbo.rs

> 文件作用：**GPU 存储缓冲包装原语**——`TypedStorageBuffer<T>`（40 B，GPU-only 元素缓冲）、`InstanceBuffer<T>`（48 B =
> instance_count + TSB）、`MappedStorageBuffer<T>`（56 B，持久 CPU 映射）+ `BufferWriteError`。全部为
> `StorageElement` 泛型单态化（113 业务 = 6 个模板族 × N 类型）。消费者：`AssetSsboLibrary::get_rg_buffer<T>`、
> `pass_builder::pass_resource_access`（绑定）、`SparseInstanceBuffer::gpu_flush`、critters/path 系统 `cpu_write`。
> 证据：`reconstruction/country-core-render/wp2/contracts/ssbo.md`（O 全量）+ `L0_NOTES.md` "A3 ssbo_library" +
> render-gap DIGEST §2.2（元素形态）。

函数总计 132：业务 113 / 内联无独立符号 14 / 派生胶水 5

## 布局表（O，wp2 合同）

| 结构 | 布局 | 证据 |
|---|---|---|
| `TypedStorageBuffer<T>` | 40 B（5 qword）：`buffer: Buffer`（rhapsody，16 B 句柄）+ `length: usize` @0x10（元素数）+ `byte_size: usize` @0x18（字节容量，**gpu_flush 守卫用**）+ `field5` @0x20（U，无反编译引用） | render-top/decomp/decomp-0x141618e00.txt（原 TypedStorageBuffer::new 0x141618e00；ShadowedBuffer::gpu_flush） |
| `InstanceBuffer<T>` | 48 B：`instance_count: usize` @0x00 + `buffer: TypedStorageBuffer<T>` @0x08 | render-top/decomp/decomp-0x1416207e0.txt（原 impl$7::new_storage_buffer 0x1416207e0） |
| `MappedStorageBuffer<T>` | 56 B：`mapped_ptr: *mut T` @0x00 + `length: usize` @0x08 + `buffer: Buffer` @0x10 + f4..f6 @0x20..0x38（U） | render-top/decomp/decomp-0x14161ad70.txt（原 impl$4::new_storage_buffer 0x14161ad70） |
| `BufferWriteError` | 枚举（O drop_in_place；变体 U）。Debug fmt（impl$22::fmt）tag 0xb → 单态单元变体，否则 1 字段元组变体 | render-gap §1 #9（0x1402b15c0：`"OutOfBounds"`/`"BackendError"` 两串 O） |

## 类型族归并总表（113 业务 → 6 族）

| 族 | 实例数 | 代表地址 | 逆向结论 | 证据 |
|---|---|---|---|---|
| `impl$0::new_storage_buffer<T>`（按字节数据的创建器，BUFFER_LEN>0 固定大小形态） | 38 | 0x14160f920 起（+ color_id 4 个 0x142859xxx） | **O**：`BUFFER_LEN>0` 时固定 `BUFFER_LEN×size_of::<T>()`、usage 0x22、memory 1、align 4；有数据先断言 `len%es==0`+4 对齐、`len>=size+es` → panic `"Excessive data size supplied for buffer {type}"`；无数据（r8==0）**不上传** | render-top/decomp/decomp-0x14160f920.txt（原 wp2 合同 + render-gap §4.2 + L0_NOTES） |
| `TypedStorageBuffer::new<T>` | 47 | 0x1416152f0..0x14161a760 | **O**：`BufferDesc::new` + `alignment` + `create_buffer<Option<&[u8]>>`；size==0 → panic；非 Ok → unwrap_failed（tag 0xb） | render-top/decomp/decomp-0x1416152f0.txt（原 wp2 合同） |
| `TypedStorageBuffer::gpu_write_ranges_nonoverlapping<T>` | 4 | 0x14161ab00 / 0x141fd3700 / 0x141fd3960 / 0x1421b4de0 | **O**：ranges 空 → 早退；bumpalo Bump 临时区（ranges.len()*0x10 容量断言）；Range 端点 ×size_of::<T>() 转字节范围；`update_buffer_bytes`；非 Ok → unwrap_failed | render-top/decomp/decomp-0x14161ab00.txt（原 wp2 合同） |
| `impl$4::new_storage_buffer<T>`（MappedStorageBuffer，256 KiB 上限） | 21 | 0x14161ad70 起 | **O**：`initial: Option<&[T]>`；None → 分配 0x40000 零化 CPU 块；Some → 断言字节长 ≤0x40000；create_buffer + `Buffer::mapped_slice_mut()`（None → unwrap）；写 [0]=ptr、[1]=len、[2..3]=buffer；None 路径的临时块释放；映射长 < 请求长 → panic | render-top/decomp/decomp-0x14161ad70.txt（原 wp2 合同） |
| `impl$6::cpu_write<T>`（映射缓冲 CPU 写） | 2 | 0x14230edb0（Butterfly）/ 0x14230f1e0（Firefly） | **O**：puffin scope；`assert!(data.len() <= self.length)`（len_mismatch_fail）；memcpy `min(data.len(), length)` 元素 | render-top/decomp/decomp-0x14230edb0.txt（原 wp2 合同） |
| `impl$7::new_storage_buffer<T>`（InstanceBuffer） | 1 | 0x1416207e0（NaniDrawListOffsetDefaults） | **O**：`count==0` → `T::default_value()`（Option）：Some → 分配 1 槽默认实例副本 + TSB，instance_count=1；None → 0。否则断言 `byte_size % size_of::<T>()==0` 与 `count % 4==0`；instance_count = byte_size/es | render-top/decomp/decomp-0x1416207e0.txt（原 wp2 合同） |

## 族明细

### `impl$0::new_storage_buffer<T>`（38 类型）

```
SheepAnimationSsbo@0x14160f920  ShapeStreamSsbo@0x14160fb70  NaniListDrawCmdOffsets@0x14160ffc0
NaniPrevVisibilityBitmaskSsbo@0x14160fd90  DofAutofocusSsbo@0x1416101f0  ParticleEmitterSsbo@0x141610600
DrawIndirectBuffer@0x141610820  BushSsbo@0x141610b60  PebblePlacementSsbo@0x141610db0  TextPrimSsbo@0x141611730
SpatialResolveOffsetsSsbo@0x141611380  GrassBladeDataSsbo@0x141611980  BrickPrevVisibilityBitmaskSsbo@0x141611000
BrickNextVisibilityBitmaskSsbo@0x141611df0  RockyTerrainVertexSSBO@0x141611ba0  ShadowMaskHistogramSsbo@0x141612170
BrickDrawListSsbo@0x141612a90  StairsTransformSsbo@0x141612cb0  ColorLibrarySsbo@0x141612f00
NaniDrawListLengthSsbo@0x141612870  NaniFilteredInstanceAllocatorSsbo@0x141612650  NaniInstanceIndirectionSsbo@0x1416132d0
NaniInstanceIndirectionAllocatorSsbo@0x141613500  NaniDrawListSsbo@0x141613940  ShadowMaskMeanSsbo@0x141613720
FlowerSsbo@0x141613b90  DispatchIndirectBuffer@0x141613db0  RockyTerrainTriangleSSBO@0x1416144a0
CustomCursorSsbo@0x1416146c0  AtomicCounterBuffer@0x1416148e0  NaniNextVisibilityBitmaskSsbo@0x141614b00
DrawIndexedIndirectBuffer@0x141614d30  RippleSourceSsbo@0x1416150a0  LeafParticleSsbo@0x141614110
GateColorIdSsbo@0x142859fe0  HalftimberColorIdSsbo@0x142859470  WallColorIdSsbo@0x142859840  RoofColorIdSsbo@0x142859c10
```

### `TypedStorageBuffer::new<T>`（47 类型）

```
VelocityOnlyInstanceData@0x1416152f0  NaniDrawListOffsetDefaults@0x1416154c0  NaniDrawListInfo@0x141615810
LanternSsbo@0x141615bb0  IvyFlowerSsbo@0x141615f50  LilypadSsbo@0x141615d80  DormerHoleCountPerCellSsbo@0x1416159e0
IvyLeafSsbo@0x141616120  RtTrianglesSsbo@0x1416162f0  PlacementDataSsbo@0x1416164c0
GothicWindowBricksInstanceData@0x141616690  RoofSpireSsbo@0x141616860  NaniMeshAtlasPartInfo@0x141616c00
GpuInstance@0x141616f30  StoneFloorInstanceData@0x141617100  SolidVertexColorWindowInstanceData@0x1416172d0
SolidVertexColorInstanceData@0x1416174a0  SolidVertexColorWindowGlassInstanceData@0x141617670
GroundedBuildingsSsbo@0x141617840  SparkleSsbo@0x141617a10  ReedSsbo@0x141617be0  StairsFloorInstanceData@0x141617db0
RtTriangleMetaSsbo@0x141617f80  CurvedHalfTimberInstanceData@0x141618150  BalconyDoorInstanceData@0x141618320
SolidAlbedoMapInstanceData@0x141618c30  PlantInstanceData@0x141618e00  Flower03Ssbo@0x141618fd0
SolidAlbedoMapEmissiveInstanceData@0x141619370  WrenInstanceData@0x141619540  PreplacedFlowerInstanceData@0x141619710
LanternEmissiveInstanceData@0x1416198e0  GardenStoneSsbo@0x141619ab0  WoodenPlankSsbo@0x141619c80
RoofTileSsbo@0x1416191a0  ClothInstanceData@0x141619e50  BoatSsbo@0x14161a020  IvySegmentSsbo@0x14161a3c0
RoofSupportBeamSsbo@0x14161a590  DormerHoleSsbo@0x14161a760  LilypadFlowerSsbo@0x14161a1f0
PlasterInstanceData@0x14161a930  BrickTransformSsbo@0x141618890  UberInstanceData@0x1416186c0
SmokeInstanceData@0x1416184f0  RtBvhSsbo@0x141618a60
```

### `impl$4::new_storage_buffer<T>`（MappedStorageBuffer，21 类型）

```
NormalDebugSsbo@0x14161e5a0  MouseDepthValueSsbo@0x14161e8f0  GardenMaskSsbo@0x14161f660
TerrainHeightSsbo@0x14161b870  VisualTerrainHeightSsbo@0x14161c540  TerrainRockMaskSsbo@0x141620320
WaterExcludeClutterMaskSsbo@0x14161c080  WaterMaskSsbo@0x14161d570  Cobblestone1RasterStagingSsbo@0x14161da30
Cobblestone2RasterStagingSsbo@0x14161ad70  Cobblestone3RasterStagingSsbo@0x14161d1f0  GardenRasterStagingSsbo@0x14161ca00
PathRasterStagingSsbo@0x14161fb20  WaterRasterStagingSsbo@0x14161e220  StairsCounterSsbo@0x14161ddb0
DofFocusReadbackSsbo@0x14161ed70  MouseTerrainRaycastSsbo@0x14161cd80  BushCpuSsbo@0x14161bd30
ButterflySsbo@0x14161b0f0  FireflySsbo@0x14161f1e0  FireflyVisibilitySsbo@0x14161fea0
```

### `gpu_write_ranges_nonoverlapping<T>` / `cpu_write<T>` / `impl$7::new_storage_buffer<T>`

```
gpu_write_ranges_nonoverlapping:  BrickTransformSsbo@0x14161ab00  RoofTileSsbo@0x1421b4de0
  WoodenPlankSsbo@0x141fd3700  SparkleSsbo@0x141fd3960
cpu_write:  ButterflySsbo@0x14230edb0  FireflySsbo@0x14230f1e0
impl$7::new_storage_buffer:  NaniDrawListOffsetDefaults@0x1416207e0（元素 0x1b0=432 B；T::default_value 有默认槽）
```

## 内联/无独立符号函数

- `TypedStorageBuffer::new`（行 63）— **I**：泛型入口（O 行为见族表）
- `TypedStorageBuffer::new_arbitrary`（行 118）— **U/I**：任意字节布局构造器（无独立机器码，语义未恢复）
- `TypedStorageBuffer::gpu_write`（行 156）— **I**：委托 `gpu_write_with_offset(data, 0)`（PDB 依赖 O：slice_as_bytes、size_of_val、Result::map_err）
- `TypedStorageBuffer::gpu_write_element`（行 199）— **U/I**：单元素写（无独立代码）
- `TypedStorageBuffer::gpu_write_with_offset`（行 208）— **I**：边界检查 `(offset+len)*es <= byte_size` → `update_buffer_bytes`（契约 I）
- `MappedStorageBuffer::new`（行 276）— **I**：构造入口（O 行为见族表）
- `impl$6::mapped_data` / `mapped_data_mut`（行 297/301）— **I**：`&[T]`/`&mut [T]` 访问器（`mapped_ptr..+length`）
- `impl$7::new_storage_buffer`（行 321）— 泛型入口（O 行为见族表）
- `InstanceBuffer::new`（行 335）— **I**：构造器
- `InstanceBuffer::gpu_write`（行 357）— **I**：写内嵌 TSB
- `InstanceBuffer::gpu_write_subset`（行 369）— **I**：下标/计数子集写
- `InstanceBuffer::set_instance_count` / `instance_count`（行 383/392）— **I**：±0x00 字段访问（O 布局）

## 派生胶水（5 项：impl$22::fmt ×2、closure$0 ×2、deref）

- `impl$22::fmt`（BufferWriteError Debug，render-gap O 0x1402b15c0）：tag 0xb → `"OutOfBounds"`，否则 `"BackendError"`。
- closure$0（gpu_write_ranges 的映射闭包）、deref = 标准胶水，不补。

## 纠错/差异（wp2 合同 + render-gap）

- **Ok tag 0xb** 是 Rust 枚举布局产物，非语义常量（wp2 合同注）。
- reconstruction `pass_kit::fixed_len_typed_storage`（pass_kit.rs:371）已按 `BUFFER_LEN>0` 形态落地；差异已注明：`BUFFER_LEN>1` 无数据时传全零而原版不上传（U：设备未初始化内容；render-gap §4.2 结论，可选整合）。
- 元素形态（render-gap §2.2）：`DofAutofocusSsbo` = `DofFocus`（64 B，inc/coc.hlsl）、`DofFocusReadbackSsbo` = `DofFocusReadback`（16 B，成员原拼写 `lock_reponse`）、`ShadowMaskHistogramSsbo` = u32[256]、`ShadowMaskMeanSsbo` = 1×f32——载体可补真实 `StorageElement`（NAME/BUFFER_LEN 机器码核对 U，留各 SSBO owner）。
- `TypedStorageBuffer::new` 的 `SpatialResolveOffsetsSsbo` 实例（0x141611380）：大小 0x2000、用途 0x12、对齐 4、名 `"SpatialResolveOffsets"`（A3_res_render O，512 个 int4 默认值）。

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. 三形态创建器（O，wp2 合同 + render-gap §4.2）
- **`impl$0::new_storage_buffer<T>`（BUFFER_LEN>0 固定大小形态，38 类型）**：固定 `BUFFER_LEN×size_of::<T>()`、usage 0x22、memory 1、align 4；有数据 → 断言 `len%es==0` + 4 对齐 + `len>=size+es`（**`"Excessive data size supplied for buffer {type}"` panic**）；**无数据不上传**（r8==0 分支）。
- **`TypedStorageBuffer::new<T>`（47 类型）**：`BufferDesc::new + alignment + create_buffer<Option<&[u8]>>`；size==0 → panic；非 Ok → unwrap_failed（tag 0xb）。40B = `buffer: Buffer`(16B) + length@0x10 + byte_size@0x18（**gpu_flush 守卫**）+ field5@0x20(U)。
- **`impl$4::new_storage_buffer<T>`（MappedStorageBuffer，21 类型）**：`initial: Option<&[T]>`；None → 分配 0x40000 零化 CPU 块；Some → 断言字节长 ≤0x40000；`create_buffer` + `Buffer::mapped_slice_mut()`（None → unwrap）；写 [0]=mapped_ptr、[1]=len、[2..3]=buffer；映射长 < 请求长 → panic。56B 布局。**大小 = BUFFER_LEN×size_of::<T>()**（0x40000 只是 6 个 u8 staging 的巧合）。
- **`impl$7::new_storage_buffer<T>`（InstanceBuffer，1 类型）**：`count==0` → `T::default_value()`（Some → 1 槽默认实例 + TSB、instance_count=1；None → 0）；否则断言 `byte_size%es==0` 与 `count%4==0`；instance_count = byte_size/es。48B = instance_count@0 + TSB@8。

### 2. 写路径
- `gpu_write_ranges_nonoverlapping<T>`（4 类型）：ranges 空 → 早退；bumpalo Bump 临时区（ranges.len()×0x10 容量断言）；Range 端点 ×es 转字节范围；`update_buffer_bytes`；非 Ok → unwrap_failed。
- `cpu_write<T>`（Butterfly/Firefly）：puffin scope；`assert!(data.len() <= self.length)`（**len_mismatch_fail**）；memcpy `min(data.len(), length)` 元素。
- 消费链：`AssetSsboLibrary::get_rg_buffer<T>` → `BufferState.data` vtable+0x18 → 取 TSB → 绑定（pass_resource_access 章）；`ShadowedBuffer::gpu_flush`/`SparseInstanceBuffer::gpu_flush` 以内嵌 TSB 的 byte_size 守卫上传。

### 3. 纠错
Ok tag 0xb 是 Rust 枚举布局产物；`pass_kit::fixed_len_typed_storage` 对 BUFFER_LEN>1 无数据时传全零而原版不上传（U）；元素形态：DofAutofocusSsbo=DofFocus(64B)、DofFocusReadbackSsbo=16B（成员原拼写 `lock_reponse`）、ShadowMaskHistogram=u32[256]、ShadowMaskMean=1×f32。

## 跨领域线索

- `update_color_library`（color_library）经 `AssetSsboLibrary` vtable +0x18 取 `TypedStorageBuffer<ColorLibrarySsbo>` 后读 `byte_size@+0x18` 守卫 + update_buffer_bytes（A3_color O）。
- `ShadowedBuffer::gpu_flush`/`SparseInstanceBuffer::gpu_flush` 以内嵌 TSB 的 `byte_size` 守卫上传（wp2 合同集成注）。
- `pass_kit.rs` 的 `SsboType`/`StorageElement` 契约与 `impl$0/4/7` 形态对应（render-gap §2.3）。
