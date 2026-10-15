# systems/ivy/ivy_send_to_gpu_leaf.rs

> 文件作用：**叶 GPU 上传（SSBO）**

函数总计 4：业务 1 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf` | 0x1408c1580 | 13 | 系统:上传 | O：ivy_send_to_gpu_leaf——叶 SSBO 上传 | collision-eco\decomp\decomp-0x1408c1580.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项

## 实现方式（算法级）

- ivy_send_to_gpu_leaf（0x1408c1580）：全量叶迭代（IvyLeafStorage::all_leaves）→ 104B 叶实例参数打包 → 叶 SSBO 上传（render 域交叉）。
