# resources/walls/elevation_support_stats.rs

> 文件作用：升降支撑统计（ElevationSupportStats/ElevationSupports）：墙高调整的支撑统计，供 audio elevation_supports_audio 消费；无独立业务函数。

函数总计 2：业务 0 / 内联无独立符号 0 / 派生胶水 2

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 2 项
## 实现方式（算法级）

### 该文件无独立业务函数（依据：sysres-procedures.json 不含 elevation_support_stats.rs）

sysres-procedures.json 无 `resources/walls/elevation_support_stats.rs` 键（仅 2 条 derive 胶水）。ElevationSupportStats/ElevationSupports 为纯状态容器，由墙高调整系统（adjust_wall_height_impl 0x140ac1f20 等）内联读写，供 audio elevation_supports_audio 消费；无独立可恢复业务过程。

### 遗留 U

- ElevationSupportStats 字段布局 U（音频域消费点另行恢复）。
