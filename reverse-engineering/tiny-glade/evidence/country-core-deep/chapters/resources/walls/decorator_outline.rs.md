# resources/walls/decorator_outline.rs

> 文件作用：装饰描边数据（DecoratorOutline{尺寸+0x20..,位置+0x30..,判别}/DecoratorCollisionOutline{Lines(Vec<Vec3>)/Sphere}）与调试显示（Mat3+平移逐顶点变换）。

函数总计 6：业务 2 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display` | 0x140b33920 | 24 | DecoratorCollisionOutline::display（Mat3+平移逐顶点） | O | wall\decomp\decomp-0x140b33920.txt |
| `country_core::resources::walls::decorator_outline::DecoratorOutline::display` | 0x140b33b00 | 61 | DecoratorOutline::display 闭包 | O | wall\decomp\decomp-0x140b33b00.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
