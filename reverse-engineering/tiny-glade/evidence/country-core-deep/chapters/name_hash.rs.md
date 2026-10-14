# name_hash.rs（utils::name_hash）

> 文件作用：**NameHash —— xxh3_64 名字哈希 newtype（资源名→64 位 id）。Debug 格式 NameHash { hash: N }。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `NameHash::compute` | 0x140c9cd80 | 9 | xxh3_64_with_secret(默认 secret, 0xc0, seed=0) = xxhash-rust xxh3_64 默认路径（46B） | O | audio-misc round1 手解 |
| `NameHash::eq` | 0x140c9cd70 | 15 | u64 比较 | O | 同上 |
