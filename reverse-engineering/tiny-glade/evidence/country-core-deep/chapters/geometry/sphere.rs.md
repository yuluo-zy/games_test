# geometry/sphere.rs

> 文件作用：**程序化球体几何（icosphere）**（MODULE_GAP 判 0% 覆盖，本节补齐）。`sphere::try_from` 从序列化数据构建细分球体，依赖 `hexasphere-12.0.0`（公开 crate，直接复用不逆向库实现）。

函数总计 2：业务 1 / 派生胶水 1。

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$2::try_from` | 0x140a06b00 | 19 | 球体反序列化/构建 | O（8891B）：解析球体定义（细分级等）→ 调用 hexasphere 构建细分球 → 写顶点缓冲；panic 串 `Cannot create an icosphere of …`（0x142ac6758）、源 `geometry/sphere.rs`；hexasphere 来自 crates.io registry（`hexasphere-12.0.0/src/lib.rs`）；参数化构建 + 大量 f32/长度处理 | geometry-gap\decomp\decomp-0x140a06b00.txt |

## 派生胶水

`impl$0::from`（1 项，From 转换）。

## 关键结论

- `sphere::try_from` 是 **icosphere 构建入口**：游戏侧解析参数（细分级别/半径）并调 hexasphere 库生成顶点；库实现不逆向（公开依赖）。
- 位置 `geometry/sphere.rs`（panic 串证实）；与 `cube.rs`/`plane.rs`（纯胶水）不同，这是唯一真实业务。
