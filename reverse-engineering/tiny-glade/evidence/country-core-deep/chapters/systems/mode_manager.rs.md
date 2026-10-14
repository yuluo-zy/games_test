# systems/mode_manager.rs

> 文件作用：AppMode 模式状态机中枢：哈希 esc_menu_canvas 阻塞早退、排空 RequestAppModeChange 双缓冲队列、check_if_can_switch 校验/延后（pending=0xd Replay）、按模式 id 跳表分发到工具栏类别闭包 env_3..14、send_mode_just_changed 事件（prev/current AppMode 对）。AppMode 判别：2=Timelapse、6..9=Wall 形状、10+5k=Tool(类别 k，视觉序)、0xd=Replay；事件队列 = bevy 0.16 Events 双 VecDeque（EventInstance {id u64,event}，官方 registry 布局）。

函数总计 68：业务 21 / 内联无独立符号 2 / 派生胶水 45

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::mode_manager::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x140afbe10 | 15 | 序列化/serde | O：AppMode serde_json deserialize visit_str（派生胶水过滤线内的业务锚，U 细节） | ui-input\decomp\decomp-0x140afbe10.txt |
| `country_core::systems::mode_manager::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >` | 0x140afbba0 | 15 | 序列化/serde | O：serde_json deserialize（AppMode 反序列化入口，Compact/Pretty 对应序列化侧） |  |
| `country_core::systems::mode_manager::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1400368a0 | 54 | 序列化/serde | O：AppMode CompactFormatter 序列化（u8 判别-0xfa 界 0x10 的跳表分发，变体名在跳表外） | ui-input\decomp\decomp-0x1400368a0.txt |
| `country_core::systems::mode_manager::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x140afc7c0 | 54 | 序列化/serde | O：AppMode PrettyFormatter 序列化（同上，跳表 at rip+0x1fe99fc 区） | ui-input\decomp\decomp-0x140afc7c0.txt |
| `country_core::systems::mode_manager::AppMode::is_tool_selected` | 0x140a86b40 | 87 | 谓词/校验 | O：AppMode::is_tool_selected 真身（SOURCE_MAP 行 87；重建代码写的 0x140a8a5d0 是 cleanup_timelapse，伪） |  |
| `country_core::systems::mode_manager::check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env$0>` | 0x140afa8a0 | 114 | check_if_can_switch 分发器 | O（round2 capstone 全解）：force→push {u32 4, 计数} 0x20B 记录；非 force→*模式状态=0xd(Replay)、push {id, 请求mode}、byte@1<2 再 push {id,byte}；byte@1>=2 = unreachable!() panic（toolbar.rs:40，串 0x142ae5650）。分发在 mode_manager 跳表块内 | ui-input\decomp\decomp-0x140afa8a0.txt |
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

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::mode_manager::check_if_can_switch`（行 114）— 谓词/校验
- `country_core::systems::mode_manager::switch_app_mode`（行 339）— I：按名推断 = 写入 AppMode 状态 + PrevAppMode 更新的内联核心（逻辑并入 mode_manager 跳表块，U 独立地址）

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_enum`、`visit_map`、`visit_newtype_struct`、`visit_seq`、`visit_str` 等 45 项
