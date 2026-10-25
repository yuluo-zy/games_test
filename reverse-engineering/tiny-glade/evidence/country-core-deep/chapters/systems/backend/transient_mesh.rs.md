# systems/backend/transient_mesh.rs

> 文件作用：**瞬时网格后端系统。`delete_dropped_transient_meshes` 全内联（无独立物理符号），为 `TransientMeshRegistry::drain_meshes_to_delete`（0x1412e9e70）的消费闭包，排空待删网格句柄并释放 GPU 网格；组件在 backend 每帧运行。**

函数总计 2：业务 0 / 内联无独立符号 1 / 派生胶水 1

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes`（行 10）— **I**：全内联，无独立地址；从名称+PDB inline 记录推断为 `TransientMeshRegistry::drain_meshes_to_delete`（0x1412e9e70）的消费闭包，逐网格释放 GPU 网格。行为并入调用点（drain 消费处），无独立机器码归属。

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项（行 16 的消费闭包）

## 备注

- 重构 `DeleteDroppedTransientMeshes { registry: Arc<TransientMeshRegistry> }` 与证据方向一致（Arc 共享 registry），但为推断映射。

## 实现方式（算法级）

**delete_dropped_transient_meshes（全内联，I）**：
- 无独立地址；为 `TransientMeshRegistry::drain_meshes_to_delete`（0x1412e9e70，O 地址）的消费闭包，逻辑并入 drain 消费处。（I）
- 处理链：drain 排空待删网格句柄 → 逐网格释放 GPU 网格；组件在 backend 每帧运行。（I）
- 重构 `DeleteDroppedTransientMeshes { registry: Arc<TransientMeshRegistry> }` 与证据方向一致（Arc 共享 registry），为推断映射。（I）
