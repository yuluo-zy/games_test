# meta_commander.rs

> 文件作用：**MetaCommander 元命令队列（每帧消费一条）+ profiler sink 表 + MetaCommandFence 装载栅栏。commander_update 按标签 0..0x13 分派（装载/回放/性能/内存/退出）。**

函数总计 9：业务 9 / 内联无独立符号 2 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `MetaCommander::new` | 0x14091eff0 | 35 | 空命令队列+空 sink 表（80B 资源） | O | wp15 app-shell.md |
| `MetaCommander::peek` | 0x14091f0c0 | 41 | VecDeque 队首元素指针/None（ring 计算，元素 stride 0x38） | O | audio-misc round1 手解 |
| `MetaCommander::consume` | 0x14091f100 | 47 | 弹出队首（空哨兵 0x8000000000000014） | O | wp15 app-shell.md |
| `MetaCommander::is_profiling` | 0x14091f0f0 | 55 | [+0x38]!=0（该偏移字段语义 U：sink 表/命令队列区） | O（偏移）/U（语义） | audio-misc round1 手解 |
| `commander_update` | 0x14091f180 | 62 | 标签分派：0=装载(NextSession+Fence)、1=重排队、2=TimeUpdateStrategy、3=InputRecordReplayState、6=手动时间、7=默认回放、8=GPU 设备列表、9=建存档目录、0xc=开 profiling、0xd=写 profile、0xe=GPU 性能、0xf=MemoryLog JSON、0x10=despawn、0x11=WriteSettingsFileCmd、0x12=ShutdownGame、0x13=process::exit | O | wp15 app-shell.md |
| `MetaCommandFence::new_pair_blocked` | 0x14091ef90 | 75 | Arc<Mutex<(bool,bool)>> 双置位（装载阻塞栅栏） | O | 同上 |
| `MetaCommandFence::is_blocked/unblock` | (内联) | — | 栅栏查询/解除（commander_update/load_game 内联） | O | 同上 |
