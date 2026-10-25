# resources/shutdown.rs

> 文件作用：**游戏关闭命令（bevy 系统）。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::shutdown::initiate_game_shutdown | 0x140b2ba40 | 11 | 0xb2ba40：bevy 系统（EventReader<ShutdownGame> 双缓冲+Commands::insert_resource；重建代码 Vec 参数模型错） | O | audio-misc\decomp\decomp-0x140b2ba40.txt |
