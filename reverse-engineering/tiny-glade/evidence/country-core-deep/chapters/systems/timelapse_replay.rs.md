# systems/timelapse_replay.rs

> 文件作用：**延时摄影录制/回放状态机**——`TimelapseState` 字节 0=idle/1=recording/2=finished；
> start：消费 StartTimelapseReplayCmd→修剪输入流→有未消费帧→mode_manager::check_if_can_switch；
> finish：未完成时推 MetaCommand（i64::MIN 哨兵，meta_commander.rs）→state=1，命令队列空后推
> {tag,byte} 事件（TimelapseReplayFinished/Interrupt 族）；cleanup：有剩余帧→日志→state=2。

函数总计 5：业务 4 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `TimelapseState::is_active` | 0x14092e6b0 | 44 | 谓词：**`state != 2`**（"未结束"；骨架写 ==1 是错的） | O：`cmp byte[rcx],2; setne al` | camera-time\decomp\decomp-0x14092e6b0.txt（hex 手解） |
| `start_timelapse` | 0x140a8a310 | 51 | 见文件作用（EventReader 修剪 + check_if_can_switch<closure> 0x140af9e00） | O：实证 | camera-time\decomp\decomp-0x140a8a310.txt |
| `finish_timelapse` | 0x140a8a460 | 95 | 见文件作用（MetaCommand 推入 FUN_140b59900；事件 stride 0x10 {tag:i64::MIN, byte}） | O：实证 | camera-time\decomp\decomp-0x140a8a460.txt |
| `cleanup_timelapse` | 0x140a8a5d0 | 138 | 有未消费帧→日志（level 3）"The timelapse has finished"→state=2 | O：实证 | camera-time\decomp\decomp-0x140a8a5d0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）

`TimelapseState` 字节 0=idle/1=recording/2=finished；`is_active` 0x14092e6b0 = `state != 2`（骨架写 ==1 错）。
```
start_timelapse（0x140a8a310）：消费 StartTimelapseReplayCmd → 修剪输入流 → 有未消费帧 → mode_manager::check_if_can_switch<closure>
finish_timelapse（0x140a8a460）：state!=2 时：未消费帧且 state 偶数 → 推 MetaCommand（i64::MIN 哨兵 + 16B@+0x60..0x6c，FUN_140b59900）→ state=1
  命令队列空（+0x18==0）→ 推 {tag:i64::MIN, byte: state} 事件（stride 0x10）并清另一计数器
cleanup_timelapse（0x140a8a5d0）：有未消费帧 → 日志（level 3）"The timelapse has finished" → state=2
```
