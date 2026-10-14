# systems/wall/upload_instanced_wall_ssbo.rs

> 文件作用：实例化墙砖 SSBO 上载：AssetSsboLibrary::get_mut<BrickTransformSsbo>（缺失 unwrap panic）；逐 InstancedWall 脏区间（+0x108 compact 标志双路径）upload_to_storage_buffer；SparseInstanceBuffer::gpu_flush。GPU 前段。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::upload_instanced_wall_ssbo::gpu_flush_brick_buffers` | 0x14082c170 | 9 | upload_instanced_wall_ssbo（get_mut<BrickTransformSsbo> unwrap + 脏区间双路径 + gpu_flush） | O | wall\decomp\decomp-0x14082c170.txt |
