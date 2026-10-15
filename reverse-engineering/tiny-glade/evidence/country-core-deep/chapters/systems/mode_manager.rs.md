# systems/mode_manager.rs

> 文件作用：AppMode 模式状态机中枢：哈希 esc_menu_canvas 阻塞早退、排空 RequestAppModeChange 双缓冲队列、check_if_can_switch 校验/延后（pending=0xd Replay）、按模式 id 跳表分发到工具栏类别闭包 env_3..14、send_mode_just_changed 事件（prev/current AppMode 对）。AppMode 判别：2=Timelapse、6..9=Wall 形状、10+5k=Tool(类别 k，视觉序)、0xd=Replay；事件队列 = bevy 0.16 Events 双 VecDeque（EventInstance {id u64,event}，官方 registry 布局）。

函数总计 68：业务 21 / 内联无独立符号 2 / 派生胶水 45

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::mode_manager::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x140afbe10 | 15 | 序列化/serde | O：AppMode serde_json deserialize visit_str（变体名 All/Wall/Path/Bulldoze/Water/Garden/Tree → 0..6） | ui-input\decomp\decomp-0x140afbe10.txt（已补） |
| `country_core::systems::mode_manager::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >` | 0x140afbba0 | 15 | 序列化/serde | O：serde_json deserialize（AppMode 反序列化入口，Compact/Pretty 对应序列化侧） |  |
| `country_core::systems::mode_manager::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1400368a0 | 54 | 序列化/serde | O：AppMode CompactFormatter 序列化（u8 判别-0xfa 界 0x10 的跳表分发，变体名在跳表外） | ui-input\decomp\decomp-0x1400368a0.txt |
| `country_core::systems::mode_manager::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x140afc7c0 | 54 | 序列化/serde | O：AppMode PrettyFormatter 序列化（同上，跳表 at rip+0x1fe99fc 区） | ui-input\decomp\decomp-0x140afc7c0.txt |
| `country_core::systems::mode_manager::AppMode::is_tool_selected` | 0x140a86b40 | 87 | 谓词/校验 | O：AppMode::is_tool_selected 真身（SOURCE_MAP 行 87；重建代码写的 0x140a8a5d0 是 cleanup_timelapse，伪） |  |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$0>` | 0x140afa8a0 | 114 | check_if_can_switch 分发器 | O（round2 capstone 全解）：force→push {u32 4, 计数} 0x20B 记录；非 force→*模式状态=0xd(Replay)、push {id, 请求mode}、byte@1<2 再 push {id,byte}；byte@1>=2 = unreachable!() panic（toolbar.rs:40，串 0x142ae5650） | ui-input\decomp\decomp-0x140afa8a0.txt（已补） |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$10>` | 0x140afa190 | 114 | 类别闭包 env_10 | O：工具栏类别 idx6(Water) 的模式索引推进（current+1 % len、Toolbar.current=N、拷 ModeEntry 0x10B 入 pending、cat==0x15 哨兵跳过）；force 段 push {4,计数} | ui-input\decomp\decomp-0x140afa190.txt |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$11>` | 0x140afa640 | 114 | 类别闭包 env_11 | O：idx9(Animals) 同 env_10 模式（entry@0x130） | ui-input\decomp\decomp-0x140afa640.txt |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$12>` | 0x140afa510 | 114 | 类别闭包 env_12 | O：idx5(Trees) 同 env_10 模式（entry@0xb0） | ui-input\decomp\decomp-0x140afa510.txt |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$13>` | 0x140afa060 | 114 | 类别闭包 env_13 | O：idx8(Meadow) 同 env_10 模式（entry@0x110） | ui-input\decomp\decomp-0x140afa060.txt |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$14>` | 0x140afa9f0 | 114 | 类别闭包 env_14 | O：idx10(Terrain)：请求.cat==0x13 时清零 cat 直调 switch_mode；否则同 env_10（entry@0x150）；越界 panic FUN_1428d9518 | ui-input\decomp\decomp-0x140afa9f0.txt |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$3>` | 0x140afa2c0 | 114 | 类别闭包 env_3 | O：idx7(cat0 空类别)：switch_mode 直调路径 + 同模式推进 | ui-input\decomp\decomp-0x140afa2c0.txt |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$4>` | 0x140afa770 | 114 | 类别闭包 env_4 | O：动态类别变体（类别 id 取自参数 bVar13），索引推进同族 | ui-input\decomp\decomp-0x140afa770.txt |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$5>` | 0x140afab70 | 114 | 类别闭包 env_5 | O：idx0(Buildings)：`current != 0` 判定的 idx0 变体（entry@+0x10） | ui-input\decomp\decomp-0x140afab70.txt |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$6>` | 0x140afa3f0 | 114 | 类别闭包 env_6 | O：idx1(Walls)（byte==1 判定） | ui-input\decomp\decomp-0x140afa3f0.txt |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$7>` | 0x140afadb0 | 114 | 类别闭包 env_7 | O：idx2(Decor)（byte==2） | ui-input\decomp\decomp-0x140afadb0.txt |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$8>` | 0x140afac90 | 114 | 类别闭包 env_8 | O：idx3(Misc)（byte==3） | ui-input\decomp\decomp-0x140afac90.txt |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$9>` | 0x140af9f30 | 114 | 类别闭包 env_9 | O：idx4(Paths)（byte==4） | ui-input\decomp\decomp-0x140af9f30.txt |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::timelapse_replay::start_timelapse::closure_env$0>` | 0x140af9e00 | 114 | start_timelapse 的检查闭包 env_0 | O：非 force：写 pending AppMode 对、组 40B 记录调 FUN_140b59be0（timelapse 元命令，meta_commander.rs）、推 {u32 1, f32 时标} 事件、清标志字节 | ui-input\decomp\decomp-0x140af9e00.txt |
| `country_core::systems::mode_manager::mode_manager` | 0x140a86b60 | 154 | 系统:周期性处理（mode_manager） | O：hash('esc_menu_canvas') 早退（hover+id+交点 tag!=2 且 mode!=2）→ 排空请求队列 → check_if_can_switch → 按状态@+0x80 模式 id 走跳表 @0x142ad890c（0..64 → 函数体各块；mode=10+5k 直调 env 闭包） | ui-input\decomp\decomp-0x140a86b60.txt |
| `country_core::systems::mode_manager::send_mode_just_changed_event` | 0x140a875e0 | 352 | 系统:事件发射（send_mode_just_changed_event，PDB 名 closure$0） | O：prev/current AppMode ne → push 0x28B {prev 16B, current 16B, id}，current:=prev。载荷是双 AppMode 事件（非 RequestAppModeChange） | ui-input\decomp\decomp-0x140a875e0.txt |

## 实现方式（算法级）（O，decomp + round2 capstone 全解）

**mode_manager @0x140a86b60**（主系统）：
1. **esc_menu_canvas 早退**：`hash("esc_menu_canvas", len 0xf)`（FUN_1405bdda0，串 0x142ad8501）；
   若 `hover字节@+0x18 != 0 && widget id@+0x38 == hash && 交点载荷@+0x28 != 2(None) && mode@+0x28 != 2`
   → 直接 return（ESC 菜单画布悬停时阻塞模式机）。
2. **排空请求双队列**：param = Events<RequestAppModeChange> 双 VecDeque
   `{a:{cap,ptr,len,head}@+0x10/+0x30 区, 计数@+0x40}`；计算未消费数
   `counter - head_a - head_b`（bevy 0.16 Events 语义）；非零 → 取最后一条
   `{id u64@0, 请求mode u32@8}`，调 `check_if_can_switch(force = 双队列读数差为 0, pending,
   &closure_env)`（env 来自 mode_manager 捕获）。
3. **模式分发**：读 `状态@+0x80` 的 u32 模式 id → jumptable @0x142ad890c
   `(*(code *)(&table + (u32)id*4))()` —— 0..64 项全落函数体 0x140a86cec..0x140a874e8 各模式块；
   干净映射（capstone 消中指令假跳转）：**mode = 10+5k**，k=工具栏视觉序
   `{Buildings:0, Walls:1, Decor:2, Misc:3, Paths:4, Water:5, Animals:6, Trees:7, Meadow:8,
   Terrain:9}` → 直调 `check_if_can_switch<closure_env_5..14>`；mode 5/8 额外调 env_3/env_4
   （cat0 复位路径）；mode 0/5 同形块（`cmp [state+0x30],0 → jno` 分流）。

**check_if_can_switch @0x140afa8a0（分发器主体，round2 capstone 0x140afa8a0..0x140afa9e8）**：
- `force(cl != 0)`：push 0x20B 记录 `{u32 4@0, 计数@+0x18}` → 返回（立即执行路径）。
- `非 force`：① 闭包环境[0]（mode_manager param_1 = ResMut<模式状态>）tick 回写后
  `mov dword [*env0], 0xd` —— **pending 置 Replay(0xd)**（历史回放联动）；
  ② push 0x10B `{计数, u32 *env[2]（请求 mode）}` 入环境[1] 队列；
  ③ 读 `byte [env[2]+1]`：
  - `< 2` → 环境[3] 队列 push `{计数, byte}` 后返回；
  - `>= 2` → `lea rcx,0x142ae5650; mov edx,0x28; lea r8,0x142ae5658; call panic_str` =
    `unreachable!()`（"internal error: entered unreachable code"，**文件 toolbar.rs line 40**）。

**15 个 closure_env_N 类别推进器（env_3..14，两段式，O）**：
- force 段：push `{u32 4, 计数}` 0x20B。
- 类别段（以 env_10=Water idx6 @0x140afa190 为例）：
  `Toolbar.categories.len(+0x10) < N → 返回`；
  `categories[cat N].modes.len(+0xd0) == 0 → 返回`；
  `Toolbar.current(+0x18) == N` → `idx = (current+1) % len`（`(uVar7|uVar12)>>0x20==0` 判 32/64
  位双路径）；
  否则 `current >= len → FUN_1428d9518()`（bounds panic，toolbar.rs 0x142ae5638）；
  写回 `categories[cat].current(+0xd8)`；`Toolbar.current = N`；
  取 `modes[idx]`（0x10B），`cat != 0x15`（空哨兵）→ 0x10B `{cat u32@0, arg u64@4, mode u32@0xc}`
  拷入 pending 请求槽。
- **闭包↔类别映射**（O）：env_3=idx7(cat0 空类, switch_mode 直调)；env_6=idx1(Walls byte==1)；
  env_7=idx2(Decor)；env_8=idx3(Misc)；env_9=idx4(Paths)；env_12=idx5(Trees, entry@0xb0)；
  env_10=idx6(Water)；env_13=idx8(Meadow, entry@0x110)；env_11=idx9(Animals, entry@0x130)；
  env_14=idx10(Terrain, entry@0x150，`request.cat==0x13` 时清零 cat 直调 switch_mode)；
  env_5=idx0(Buildings, current!=0 变体, entry@0x10)；env_4=动态类别变体（类别取自参数）。

**check_if_can_switch<start_timelapse::closure_env_0> @0x140af9e00（env_0，O）**：
非 force 时 (a) 环境[1]（Res<AppMode>）2 qword AppMode 对 → 写环境[0]（ResMut，pending=当前）；
(b) 读相机资源 f32@+0x60/u32@+0x68/u32@+0x6c → 组 40B 栈记录
`{i64::MIN, f32, u32, u32, 0, byte=1, ..., i64::MIN+1}` → FUN_140b59be0（MetaCommand 入队，
meta_commander.rs —— timelapse 起始元命令）；(c) `time_scale = f32 * _UNK_142925988`（全局乘数）
→ push 事件 `{u32=1, f32=time_scale}`；(d) 置环境[6] ResMut 字节=0。

**send_mode_just_changed_event @0x140a875e0（O）**：
`func_0x000140a825e0(prev, current)`（= AppMode PartialEq::ne，2×qword）→ ne 则：
`Events push 0x28B`（元素 stride 0x28）`{prev qword×2@0, current qword×2@0x10, 计数@0x20}`；
随后 `current := prev`。**事件载荷是 {prev, current} 双 AppMode**（非 RequestAppModeChange）。

**timelapse 联调（wp20 复核 O）**：start_timelapse @0x140a8a310 = 排空→check_if_can_switch<env_0>；
finish_timelapse @0x140a8a460 = 排空→`MetaCommander::extend`(1 条)+标志字节=1，且窗口焦点@+0x18==0
→ push WindowFocusEvent `{id, u8=0}`（失焦）；cleanup_timelapse @0x140a8a5d0 = 排空→log +
`*(byte*)*state = 2`（回 Timelapse 模式）。

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::mode_manager::check_if_can_switch`（行 114）— 谓词/校验
- `country_core::systems::mode_manager::switch_app_mode`（行 339）— I：按名推断 = 写入 AppMode 状态 + PrevAppMode 更新的内联核心（逻辑并入 mode_manager 跳表块，U 独立地址）

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_enum`、`visit_map`、`visit_newtype_struct`、`visit_seq`、`visit_str` 等 45 项
