# 合同：resources::rt（软件 BVH 光线追踪世界族）

源 SHA256（B skeleton）：`resources/rt/{mod,bvh,bvh_builder}.rs`
PDB 实体：`RtWorld` / `RtMesh` / `RtMeshHandle` / `DirtyCow` / `GpuRtTriangle` / `GpuBlBvh` / `InputTriangle` / `RtBvhBuilderGlue(+Resource)` / `RtBvhSsbo` / `RtTrianglesSsbo` / `RtTriangleMetaSsbo` / `UpdateRtWorldEvent`
Rust：`src/resources/rt.rs`

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。

## 已观察（O）

- **`RtMeshHandle`** 24B 枚举：`impl$2::from`（0x1408bc510）置 tag=1 + payload@0x10；
  `impl$5::from`（0x1408bc520）空。`RtWorld::get_mut<WallId>`（0x1412259c0）经
  `rt::impl$5::from` 建墙句柄，hashbrown get-or-insert（缺省建默认 `RtMesh`）。
- **`RtMesh`**（ArcInner 0x60B）：`impl$1::from`（0x1408bc460）线性 RGB→打包 u32
  （`sqrt(c)*255+0.5` 各通道 clamp，`g<<8|r|b<<16`；`set_albedo` 墙/楼梯构造内联）；
  顶点 `Vec<Vec3>`（+0x58 ptr/len/cap，`get_mut` 默认路径 12B/项）。
- **`DirtyCow<T>`**：COW Arc + 脏标志；`get_mut`（0x1415626f0 / 0x15626f0）
  `Arc::make_mut` 语义。
- **`RtWorld`**：`HashMap<RtMeshHandle, DirtyCow<RtMesh>>` + 移除/脏计数簿记
  （+0x10..+0x40，U）。`update_rt_world`（0x1408bc650）：puffin scope 内
  清"已移除网格"位图、`HashMap::from_iter<…, Map<IterMut, consume>>` 把脏网格汇入
  工作队列并写脏标志；`upload_rt_world`（0x1408bcff0）锁 glue，把三个输出缓冲
  经 `AssetSsboLibrary::get` 上传：
  `RtTrianglesSsbo`（count×0x18=24B）、`RtTriangleMetaSsbo`（count×4=4B）、
  `RtBvhSsbo`（count×0x50=80B）。超限日志 "Out of pre-allocated buffer space for
  ray tracing-data"。
- **`RtBvhBuilderGlue`**（`upload_rt_world` 读取）：+0x48/+0x50 三角形列表、
  +0x60 三角形元、+0x30 BVH 节点、busy/idle 标志 +0x14/+0x15。
  **`RtBvhBuilderGlueResource`** = `Arc<RwLock<RtBvhBuilderGlue>>`；
  `is_idle`（0x1408bc530）读锁判空闲。
- **`bvh_builder_worker`**（0x140b24d50）：后台线程，写锁 glue；状态==2 空闲
  sleep，否则收集全部脏网格三角形（`HashMap::values<RtMeshHandle,DirtyCow<RtMesh>>`
  → `flat_map(chunks_exact<Vec3> zip Repeat<u32>)` → `filter_map` 成
  `InputTriangle`，stride 0x28=40B），调 `bvh::build_gpu_bvh`（0x140b0da50）——
  三角形重排为 3×16B 顶点行（`[x,y,z,z]`）→ `obvhs::cwbvh::builder::
  build_cwbvh_from_tris`（复用库）→ 收集叶序三角形 + `Vec<u32>` 索引 → 存回 glue，
  清 busy 标志。
- 三个 SSBO 为游戏 SSBO 类型（O：AssetSsboLibrary `add_new`/`get_rg_buffer` +
  pass_resource_access 读写），与 render-passes 观察一致：GI/水"deferred RT"
  无硬件光追，共用软件 BVH（`rt_bl_bvh`/`rt_triangles`）。

## 输入 / 输出 / 未知

- 输入：墙/装饰/楼梯构造写 `RtMesh`（`set_albedo`/顶点）；`UpdateRtWorldEvent`。
- 输出：三个 RT SSBO（三角形/元/BVH 节点），供 GI/水 pass 采样。
- 未知（U）：`RtMesh` 变换/材质字段；`RtWorld` 计数簿记布局；`GpuRtTriangle`
  上传紧凑布局（24B vs 构建 48B 两套）；glue 内部 Vec 槽位与 `bvh_builder_worker`
  事件唤醒细节。

## 复用边界

- `obvhs::cwbvh::builder::build_cwbvh_from_tris`（本地 crate）→ `reused::bvh`。
- `frozen::FrozenMap` + `nohash_hasher`（AssetSsboLibrary 容器）→ `reused::map`。
- rhapsody `update_buffer_bytes` / `import_unowned` 属 rhapsody 边界（wp1-4）。
