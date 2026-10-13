# bind_resources.rs 业务契约（BindGameResources trait）

标注：**O**=反编译直接支持；**I**=调用链组合；**U**=未恢复。

## 1. trait `BindGameResources`（impl$0，I）

为 `ComputePass`（module 1632）与 `DrawBuilder`（module 1638）提供 `bind_color_library` / `bind_ray_tracing` / `bind_shadowmap`。

## 2. `bind_color_library<Pass>`（O，行 55）

实例：`<ComputePass>` 0x1415fc070；`<DrawBuilder>` 0x1416286f0。

1. `HashMap<&str, rhapsody::rg::binding::PassResourceSlot>::rustc_entry(&"ColorLibrarySsbo" 名)`（pass 的资源槽表）。
2. 槽为空 → `pass_resource_access::read_access_buffer<ColorLibrarySsbo>()` + 插入槽（记录 access_index）。
3. 按资源 hash `0x52ec6a17beb8fef0` 在注册表查找 → 追加 8B access_range {u32 resource_index, u8 access_index}（若注册表计数非 0）。
4. 返回消费后的 self（`memcpy(dst, src, size)`）。

## 3. `bind_ray_tracing<Pass>`（O，行 59）

实例：`<ComputePass>` 0x1415fc230；`<DrawBuilder>` 0x1416289d0（4167B）。

1. `FrozenMap<SsboId, Box<BufferState>, NoHashHasher>` 查 `RtTrianglesSsbo` 的 `SsboId`（hash `0x65cfda6a55a841ac`，名称字面量 0x2c 字节，不符 `assert_failed<&str,&str>`）→ 经 BufferState vtable 取 buffer 值；`RtTriangleMetaSsbo` 同（0x26 字面量）。
2. 对 4 个 RT SSBO 依次：`rustc_entry<&str, PassResourceSlot>` → 空槽则 `read_access_buffer<RtTrianglesSsbo/RtTriangleMetaSsbo/RtBvhSsbo>()` + 插入 → 按 hash（`0x168ddb2c3b5f38a9`/`0x47ddebfb264383d0`/`0x339e76846042a271`）解析 resource_index → 追加 8B access_range。
3. `draw_builder::impl$2::read_storage_buffer<WallColorIdSsbo>()` 登记墙色读访问；按 hash `0x1c9f2eef0ac739ac`/`0xc2b7f2fdf5e21521` 把读取的 buffer 值写入 DrawBuilder 内 CPU 缓冲（+0x198/+0xa0 目标，越界 `slice_start_index_len_fail`）——墙色 ID 回读供 CPU 侧使用。
4. 返回消费后的 self。

ComputePass 变体（0x1415fc230，3287B）结构与 DrawBuilder 相同，但末尾的 `WallColorIdSsbo` 回读路径不同（ComputePass 无 CPU 缓冲写）。

## 4. `bind_shadowmap`（I，inline 12）

占位绑定 shadowmap（DrawBuilder 的 `bind_dummy_shadowmap` 相关）。

## 5. 资源槽 hash 常量

| 资源 | 注册表 hash | 用途 |
|---|---|---|
| ColorLibrarySsbo | 0x52ec6a17beb8fef0 | 颜色库读访问 |
| RtTrianglesSsbo | 0x168ddb2c3b5f38a9 | 软件 BVH 三角形 |
| RtTriangleMetaSsbo | 0x47ddebfb264383d0 | 三角形元数据 |
| RtBvhSsbo | 0x339e76846042a271 | BVH 节点 |
| WallColorIdSsbo（写入侧） | 0x1c9f2eef0ac739ac / 0xc2b7f2fdf5e21521 | 墙色 ID 回读 |
