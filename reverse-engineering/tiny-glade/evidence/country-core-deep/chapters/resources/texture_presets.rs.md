# resources/texture_presets.rs

> 文件作用：**纹理预设绑定类型（TexturePreset 数值槽/TextureFromDisk 名字哈希）。**

函数总计 6：业务 1 / 内联无独立符号 0 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::texture_presets::impl$1::from | 0x140a59890 | 104 | 0xa59890：From<u8> 判别映射→rhapsody TextureDesc（usage: 8=mip/10-12=depth/15-16=color） | O |  |

## 派生胶水（derive/宏生成；不计独立业务）

`eq`、`fmt`、`hash`、`next` 等 5 项
