# meta_commander.rs

> 文件作用：**元命令队列（每帧处理一个 MetaCommand）**。`MetaCommander` = 80 B
> `{commands: VecDeque<MetaCommand> (cap-first 4 qwords), profiler_sinks HashMap (4 qwords),
> RandomState (2 qwords)}`；`MetaCommand` element = 0x38 B（7 qword：tag@+0 + 6 载荷 qword），tag 0..0x13，空哨兵
> `0x8000000000000014`（0x8000000000000000 = 变体 0 会话加载判别）。`commander_update` niche `^0x8000000000000000` 判别，case 0..0x13：
> 会话加载（NextSession+fence）/复队/TimeUpdateStrategy/InputRecordReplayState/手动时间/设备列表/
> 建目录/puffin profiling/GPU+内存报告/despawn/写设置/ShutdownGame/exit。
> `MetaCommandFence` = Arc<Mutex<(bool,bool)>>（`new_pair_blocked` 分配 0x18 B，strong=2，两 flag true）。
> 证据：commander_update/visit_str/visit_enum/new_pair_blocked/new/peek/consume/serialize 反编译 +
> peek/consume/is_profiling 原字节。

函数总计 52：业务 9 / 内联无独立符号 5 / 派生胶水 38

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `commander_update` | 0x14091f180 | 0 | 处理一个队列命令 | **O** | switch case 0..0x13（0xd 合并/共享）；串 No such profiling ID / writing profiler output / GPU peak memory report not available / called `Result::unwrap()` |
| `deserialize::visit_str<ron>` | 0x140168e00 | 24 | ron 变体名匹配 | **O** | 全局动作/设置名表（0x14293d960+：input_space_cursor_move/camera_pan/tool_*/ui_*/ToolbarAlignment/ImageFormationConfig/InnerOuter 等） |
| `deserialize::visit_enum<ron>` | 0x140168b60 | 24 | ron enum 匹配 | **O** | 多分支变体名 |
| `MetaCommandFence::new_pair_blocked` | 0x14091ef90 | 102 | 阻塞 fence | **O** | 分配 0x18 B（Arc 头 + Mutex<(bool,bool)>），strong=2，两 flag=true |
| `MetaCommander::new` | 0x14091eff0 | 136 | 构造空元命令器 | **O** | 80 B：空 commands + 空 sinks + RandomState 种子（FUN_140d4d740） |
| `MetaCommander::peek` | 0x14091f0c0 | 151 | 队首命令 | **O** | VecDeque front()：`ptr+(head-(head>=cap?cap:0))*0x38`，len==0→null（原字节） |
| `MetaCommander::is_profiling` | 0x14091f0f0 | 156 | 是否 profiling | **O** | `[+0x38]!=0`（字段身份存疑，DIGEST §7） |
| `MetaCommander::consume` | 0x14091f100 | 159 | 弹出队首命令 | **O** | pop_front（head 环绕 + len-1），拷贝 0x38 B（原字节） |
| `serialize (MemoryLog Pretty)` | 0x1413fc1b0 | 377 | MemoryLog JSON 序列化 | **O** | PrettyFormatter |

## 实现方式（算法级）（O，commander_update/peek/consume/is_profiling 原字节）

**commander_update @0x14091f180**：每帧处理一个 MetaCommand——`niche ^0x8000000000000000`
判别（空哨兵），case 0..0x13：会话加载（NextSession+fence）/复队/TimeUpdateStrategy/
InputRecordReplayState/手动时间/设备列表/建目录/puffin profiling/GPU+内存报告/despawn/
写设置/ShutdownGame/exit（0xd 合并/共享）；串 `No such profiling ID`/`writing profiler
output`/`GPU peak memory report not available`。（O）
**MetaCommander 结构（80B）**：`{commands: VecDeque<MetaCommand>（cap-first 4 qwords）,
profiler_sinks HashMap（4 qwords）, RandomState（2 qwords）}`；MetaCommand element=0x38B =
**7 qword {tag@+0, 载荷 qword1@+8, qword2@+0x10, qword3@+0x18, code*@+0x20, qword5@+0x28,
qword6@+0x30}**（consume/commander_update 读 7 qword），tag 0..0x13，**空哨兵
0x8000000000000014**（旧"空哨兵 0x8000000000000000"修正：0x8000000000000000 是变体 0=会话加载
判别）。commander_update 分派 case 0..0x13：会话加载（NextSession+fence）/复队/TimeUpdateStrategy/
InputRecordReplayState/手动时间/设备列表/建目录/puffin profiling/GPU+内存报告/despawn/写设置/
ShutdownGame/exit（0xd 合并/共享）；case 0（会话加载）流程=AppMode 门控→新 fence(FUN_14091ef90)
→建临时 on_load Vec→append_on_load<Fence>(0x141328ac0)→FUN_1412e0b90 应用→命令带 fence 重入队；
case 9=建目录（FUN_140d76ed0→140d61cd0→140d5a150）；case 6=float→十进制格式化推 32B 事件；
case 2/3=profiler_sinks hashbrown 迭代。（O）
- `peek @0x14091f0c0`（O，fresh 反编译）：39B 短函数——`[+0x18](VecDeque len)!=0` 判空；
  front 值内联（旧"ptr+(head-...)*0x38 计算"→新证据 len 判空，值计算内联）。
- `consume @0x14091f100`（O，fresh）：pop_front——环 {cap@+0,ptr@+8,head@+0x10,len@+0x18}，
  `head=(head-(cap<=head+1?cap:0))+1` 环绕、len--、`ptr+head*0x38` 读 7 qword 拷出；
  空 → 返回 0x8000000000000014（=空哨兵）。
- `is_profiling @0x14091f0f0`：`[+0x38]!=0`（该偏移在 hashbrown 表语义未钉死 **U**，
  DIGEST §7）。（O）
- `new_pair_blocked @0x14091ef90`（O，RNE 原字节 + reanalyst 卡2）：MetaCommandFence=Arc，alloc 0x18，
  `[+0]=1`(strong) `[+8]=1`(weak) `[+0x10]=1`(标志字节) → LOCK inc strong=2 返回。
  **unblock 标志偏移 = +0x10**（commander_update 0x14091f450 原字节 `movzx eax,[rax+0x10]` 互证；
  旧"两 flag true/+2"修正）。FUN_1412e0b90 = Commands::insert_resource<NextSession>（会话切换核心）。
- `new @0x14091eff0`：空 commands + 空 sinks + RandomState 种子（FUN_140d4d740）。（O）

## 内联/无独立符号函数

- `MetaCommandFence::unblock`（行 117）— **I**（清除阻塞标志，名称推断）
- `MetaCommandFence::is_blocked`（行 122）— **I**（读阻塞标志，名称推断）
- `MetaCommander::extend_script`（行 147）— **I**（追加命令列表）
- `MetaCommander::is_empty`（行 163）— **O**（队列空判定）
- `MetaCommander::undo_consume`（行 171）— **I**（推回队首）

## 派生胶水（38 项：closure、default、deserialize、visit_map、visit_str 等）

- `commander_update::closure$0`（0x140922670）— 会话加载闭包（fence/NextSession）**O**
- 其余为 ron serde 派生/闭包胶水
