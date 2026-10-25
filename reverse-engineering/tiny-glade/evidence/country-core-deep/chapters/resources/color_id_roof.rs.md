# resources/color_id_roof.rs

> 文件作用：**屋顶颜色 ID SSBO。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::color_id_roof::impl$0::default_value | 0x14099ca50 | 29 | 0x14099ca50：返回 1（屋顶默认 ColorId=1；wp16 合同写 0 错） | O | audio-misc\decomp\decomp-0x14099ca50.txt |

## 实现方式（算法级）（O，decomp-0x14099ca50）

**impl$0::default_value @0x14099ca50**：`eax=1` —— 屋顶默认 `ColorId=1`，非 0。
（wp16 合同 `default_value→ColorId(0)` 与之冲突，按机器码=1 重审；默认色索引语义
以屋顶侧=1 对齐。§4 #2 / §7 错位表。）O
