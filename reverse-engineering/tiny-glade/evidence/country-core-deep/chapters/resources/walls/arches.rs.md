# resources/walls/arches.rs

> 文件作用：拱段/过梁段的跨帧状态容器（ArchSegment/ArchSegments/LintelSegment/LintelSegments/BuildingHadArchesLastFrame）：由 system_wall_constructor::construct_arches 写回，供跨帧差分，无独立业务函数。

函数总计 3：业务 0 / 内联无独立符号 0 / 派生胶水 3

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 3 项
## 实现方式（算法级）

### 该文件无独立业务函数（依据：sysres-procedures.json 不含 arches.rs）

sysres-procedures.json 无 `resources/walls/arches.rs` 键（仅 3 条 derive 胶水）。ArchSegment/ArchSegments/LintelSegment/LintelSegments/BuildingHadArchesLastFrame 为跨帧状态容器，由 system_wall_constructor::construct_arches 写回、material_stats/构造系统内联读取；无独立可恢复业务过程。

### 遗留 U

- ArchSegments 容器布局（构造域消费点另行恢复）。
