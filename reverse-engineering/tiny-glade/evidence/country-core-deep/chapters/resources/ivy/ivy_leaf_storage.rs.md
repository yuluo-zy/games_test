# resources/ivy/ivy_leaf_storage.rs

> 文件作用：**常春藤叶存储：IvyLeaf 104B GPU 实例参数集**

函数总计 7：业务 1 / 内联无独立符号 5 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage::add` | 0x1408fe280 | 53 | 系统:写入 | O：IvyLeafStorage::add——push 104B IvyLeaf 到对应 ivy | collision-eco\decomp\decomp-0x1408fe280.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage::new`（行 47）— 构造
- `country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage::all_leaves`（行 88）— O：全量叶迭代（内联）
- `country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage::all_leaves_mut`（行 92）— O：全量叶迭代（内联）
- `country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage::delete_affected_leaves`（行 96）— 删除/清理
- `country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage::delete_all_leaves`（行 104）— 删除/清理

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）

- `IvyLeafStorage::add`（0x1408fe280）：按 ivy_id 查 HashMap<usize, Vec<IvyLeaf>>（hashbrown，`FUN_1405b0d70` 探测；键=ivy id）→ 未命中建新 Vec（cap 1）`FUN_141527390` 插入，命中 push 104B IvyLeaf。
- **IvyLeaf = 0x68B（104B，O 机器码逐字段）**：`{u64@0, u64@8, ivy_id u64@0x10, u64@0x18, pos Vec3@0x20 (x,y@0x20, z@0x28), 法向? Vec3@0x2c, 上向? Vec3@0x38, u32@0x44, u64=0@0x48, u32@0x50, u32@0x54, Vec3@0x58}`。GPU 打包后每叶 0x30=48B（ivy_send_to_gpu_leaf 章）。
- 内联面：`all_leaves` / `all_leaves_mut`（全量叶迭代，GPU 上传/动画用）；`delete_affected_leaves` 按 ivy id 过滤删除（pruner 联动）；`delete_all_leaves` 全清。
