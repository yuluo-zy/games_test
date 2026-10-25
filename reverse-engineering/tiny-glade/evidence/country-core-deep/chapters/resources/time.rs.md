# resources/time.rs

> 文件作用：**游戏时间资源门面**——`GameTime` 实测布局：{game_time 累计 Duration（secs@+0x0/
> nanos@+0x8）、delta Duration（secs@+0x10/nanos@+0x18）、delta f32@+0x28；+0x20..0x27 与尾部 U}；
> 消费方：weather（game_time 正弦）、time_update（delta）、time_of_day（dt@+0x28）。
> `Time` = bevy_time 0.16 离线门面。
>
> 补证：`GameTime::default` 业务面仅 `impl$0::default`（0x140b64d00，VA 已确认），零化构造；
> 布局 +0x28 delta_f32 由 systems/time_update / systems/time_of_day 写入。

函数总计 1：业务 0 / 内联无独立符号 0 / 派生胶水 1

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项（`impl$0::default`，0x140b64d00）

## 实现方式（算法级）

`GameTime` 实测布局：{game_time 累计 Duration（secs@+0x0/nanos@+0x8）、delta Duration（secs@+0x10/nanos@+0x18）、delta f32@+0x28}；消费方：weather（game_time 正弦）、time_update（delta）、time_of_day（dt@+0x28）。+0x20..0x27 与尾部 U。`GameTime::default`（0x140b64d00）零化构造；`Time` = bevy_time 0.16 离线门面。
