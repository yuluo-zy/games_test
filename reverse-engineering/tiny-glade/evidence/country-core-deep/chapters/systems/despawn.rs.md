# systems/despawn.rs

> 文件作用：**关卡拆除实体清理**——Query 迭代，world 字节 @+0x108 二分：A 支（stride 0xa0、
> count@+0x58、实体数组@+0x50、0x10B 元素）与 B 支（stride 0x48、count@+0x10、数组@+0x8、
> 8B 元素）；逐槽逐实体 Commands::despawn（0x140d14770）；u64 实体 id 高 32 位==0 提前返回。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `despawn_glade_entities` | 0x1408b3510 | 5 | 见文件作用 | O：实证 | camera-time\decomp\decomp-0x1408b3510.txt |
