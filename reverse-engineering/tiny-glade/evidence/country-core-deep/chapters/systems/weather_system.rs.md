# systems/weather_system.rs

> 文件作用：**冬季雪强振荡**——`glade_type@+0x58==4`（冬）时，10 分钟正弦
> `sin(game_time/300·π)` 正负驱动 SnowIntensity@+0xc ±0.1·dt（GameTime+0x28），clamp [0,1]。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `weather_system` | 0x1408e5b00 | 0 | 见文件作用；非冬季不写（连 tick 都不回写）；常量 300@142aa5140、π@1429edba4、0.1@142925984、1e9@1429a7640 | O：实证 | camera-time\decomp\decomp-0x1408e5b00.txt |
