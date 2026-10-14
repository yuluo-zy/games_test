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
