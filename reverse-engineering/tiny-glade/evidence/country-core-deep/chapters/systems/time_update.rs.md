# systems/time_update.rs

> 文件作用：**游戏时钟系统**——推进 56B 时钟 {secs,nanos,flag@+0x6,total f64@+0x10,
> 60ths@+0x18,frac@+0x20,frame_count@+0x2c,flag@+0x30}；重放时 delta 取自录制帧
> @+0xc8（0xd0B 帧，InputRecordReplayState+0x40==i64::MIN+2 判 Replaying），前两帧 clamp 1/60。

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `time_update` | 0x140922c30 | 10 | 见文件作用；Duration 进位 panic（"overflow when adding durations"）+ f32→Duration 转换（std 内联） | O：实证（total 为 f64，纠正合同 f32 说） | camera-time\decomp\decomp-0x140922c30.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `calculate_delta_seconds`（行 45）— 查询/计算：GameTime Duration(secs@+0x10,nanos@+0x18)→f32 秒（内联的 f32→Duration 位运算族，并入 time_update 主体）
