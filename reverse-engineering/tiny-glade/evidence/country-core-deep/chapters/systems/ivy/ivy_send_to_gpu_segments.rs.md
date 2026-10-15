# systems/ivy/ivy_send_to_gpu_segments.rs

> 文件作用：**段 GPU 上传（SSBO）**

函数总计 4：业务 1 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments` | 0x140a9b4e0 | 33 | 系统:上传 | O：ivy_send_to_gpu_segments——段 SSBO 上传 | collision-eco\decomp\decomp-0x140a9b4e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项

## 实现方式（算法级）

- ivy_send_to_gpu_segments（0x140a9b4e0）：全量段收集（IvyStorage::total_segment_count 预算）→ 段属性打包 → SSBO 上传（ivy_segments 系，render 域交叉）。
