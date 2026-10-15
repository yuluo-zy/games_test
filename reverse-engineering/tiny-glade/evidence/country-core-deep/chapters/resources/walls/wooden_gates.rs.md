# resources/walls/wooden_gates.rs

> 文件作用：木门资源：木门/栅栏门相关状态（无独立业务函数，渲染/构造域消费）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::wooden_gates::Gates::clear` | 0x140aaae80 | 14 | 删除/清理 | I | wall\decomp\decomp-0x140aaae80.txt |
## 实现方式（算法级）

### 该文件无独立业务函数（依据：sysres-procedures.json 不含 wooden_gates.rs）

sysres-procedures.json 无 `resources/walls/wooden_gates.rs` 键。Gates 状态由木门/栅栏门构造与渲染系统内联消费。索引外补充确认：

- `Gates::clear` 0x140aaae80（O，索引外补充）：hashbrown **整集清空**（读 ctrl 位图 → 逐槽释放 → len 归 0），帧间重置木门计数。

### 遗留 U

- Gates 字段布局（木门计数/状态）U。
