# systems/steam/mod.rs

> 文件作用：**Steam 集成：客户端资源/覆盖层/截图/输入映射。**

函数总计 23：业务 16 / 内联无独立符号 2 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::steam::SteamClientResource::show_floating_gamepad_text_input | 0x140ae9390 | 30 | 0x140ae9390：SteamAPI_ISteamUtils_ShowFloatingGamepadTextInput+Dismissed 回调 | O | audio-misc\decomp\decomp-0x140ae9390.txt |
| country_core::systems::steam::SteamClientResource::is_floating_gamepad_text_input_open | 0x140ae9470 | 52 | 0x140ae9470：进程全局字节 0x143634d94（回调缓存，非 API 调用；重建模型错） | O | audio-misc\decomp\decomp-0x140ae9470.txt |
| country_core::systems::steam::SteamClientResource::input_show_binding_panel | 0x140ae9480 | 55 | 0x140ae9480：客户端非空→ShowBindingPanel(handle) | O | audio-misc\decomp\decomp-0x140ae9480.txt |
| country_core::systems::steam::AppSteamIntegration::try_new | 0x140ae94b0 | 102 | 0x140ae94b0：创建 steamworks 客户端 | I |  |
| country_core::systems::steam::AppSteamIntegration::client | 0x14067dd80 | 119 | 0x14067dd80：mov rax,rcx（client 字段@+0） | O | audio-misc\decomp\decomp-0x14067dd80.txt, collision-eco\decomp\decomp-0x14067dd80.txt, terrain-water\decomp\decomp-0x14067dd80.txt |
| country_core::systems::steam::AppSteamIntegration::user_id | 0x140ae96f0 | 123 | 0x140ae96f0：每次现算 client.user().steam_id()（Arc 临时 clone+drop；重建存字段模型错） | O | audio-misc\decomp\decomp-0x140ae96f0.txt |
| country_core::systems::steam::impl$4::add_steam_integration | 0x140945320 | 133 | 0x140945320：bevy 插件 build——0x30B 系统配置{1,1,0,0,8,0}（8 系统批量 add_systems 链） | O | audio-misc\decomp\decomp-0x140945320.txt |
| country_core::systems::steam::update_steam_inputs | 0x140ae9740 | 171 | 0x140ae9740：14 个同构动作块：读动作状态→沿变化→游戏输入计数(+0x170 减/+0x178 增)→压 u16 输入码(0x200..0xf00)入环形队列+PrevSteamInputState 帧差；码→动作名配对需 ui-input 域枚举 | O（结构） | audio-misc\decomp\decomp-0x140ae9740.txt |
| country_core::systems::steam::_::impl$0::apply | 0x14003bb80 | 449 | bevy 系统参数状态 init/apply/queue 胶水（steam 系统参数面，非游戏业务） | O（round3 round2 复查修正）：0x14003bb80=ICF 折叠区（S_PUB32 437 符号），该址含 SteamOverlayOpenResources SystemParam apply 符号 → 身份成立；机器码 1B ret = 无状态胶水 no-op（round1 曾误判“该址唯一符号=Plugin::finish”，身份未证实结论作废） | rne-read 0x14003bb80；S_PUB32 public-symbols.json |
| country_core::systems::steam::_::impl$0::init_state | 0x140af0460 | 449 | steam Input 参数初始化（get_param 系） | I（胶水） | audio-misc\decomp\decomp-0x140af0460.txt |
| country_core::systems::steam::_::impl$0::new_archetype | 0x14003bb80 | 449 | bevy 系统参数状态 init/apply/queue 胶水（steam 系统参数面，非游戏业务） | O（round3 round2 复查修正）：0x14003bb80=ICF 折叠区（S_PUB32 437 符号），该址含 SteamOverlayOpenResources SystemParam new_archetype 符号 → 身份成立；机器码 1B ret = 无状态胶水 no-op（round1 曾误判“该址唯一符号=Plugin::finish”，身份未证实结论作废） | rne-read 0x14003bb80；S_PUB32 public-symbols.json |
| country_core::systems::steam::_::impl$0::queue | 0x14003bb80 | 449 | bevy 系统参数状态 init/apply/queue 胶水（steam 系统参数面，非游戏业务） | O（round3 round2 复查修正）：0x14003bb80=ICF 折叠区（S_PUB32 437 符号），该址含 SteamOverlayOpenResources SystemParam queue 符号 → 身份成立；机器码 1B ret = 无状态胶水 no-op（round1 曾误判“该址唯一符号=Plugin::finish”，身份未证实结论作废） | rne-read 0x14003bb80；S_PUB32 public-symbols.json |
| country_core::systems::steam::SteamOverlayOpenResources::can_use_overlay | 0x140aef1a0 | 456 | 0x140aef1a0（validate_param 系）：overlay 可用性校验 | O | audio-misc\decomp\decomp-0x140aef1a0.txt |
| country_core::systems::steam::SteamOverlayOpenResources::open_overlay_or_browser_to_webpage<alloc::string::String> | 0x14026d580 | 466 | open_overlay_or_browser_to_webpage<String> 泛型实例（overlay 可用→开 overlay，否则浏览器） | O |  |
| country_core::systems::steam::SteamOverlayOpenResources::open_overlay_or_browser_to_webpage<ref$<str$> > | 0x14026d3c0 | 466 | open_overlay_or_browser_to_webpage<&str> 泛型实例 | O | audio-misc\decomp\decomp-0x14026d3c0.txt |
| country_core::systems::steam::SteamOverlayOpenResources::open_browser_to_webpage<ref$<str$> > | 0x14026d080 | 480 | open_browser_to_webpage<&str>：直开浏览器 | O |  |

## 实现方式（算法级）（O，decomp-0x140ae9740 结构全解）

**update_steam_inputs @0x140ae9740（445KB/7947 行，22 个同构块）**：
- 开头：帧计数推进（`param_2[6] += 1`）；`pbVar20 = FUN_140d4d740()`（帧计数）、
  `FUN_140d64e00()`（时间）；param_3 = ResMut<SteamInputResource>（tick 回写 +0x1c）。
- **每块（~222 行）**：
  1. **找本帧变化动作**：PrevSteamInputState 表（16B 槽 SIMD lane 匹配：每槽 16 字节
     位图比较 → `SUB161(auVar>>7,0)&1` 等 16 位汇聚 → `while(bit!=0)` 取最低位）——
     找到本帧有沿变化的 Steam 动作；digital 位（`bVar52` 提取）/analog 值（fVar 浮点）。
  2. **沿变化簿记**：`游戏输入资源 +0x170 -= (pressed & 1)`（释放计数）、
     `+0x178 += 1`（按住计数）；写 Steam handle 槽位 `{[-2][0]=4/…, [-1][0]=1, [-1][8]=0}`。
  3. **压游戏输入码**：`*(u16*)(ring_ptr + len*2) = 输入码`（**Vec<u16> 2B 元素 cap-first**）；
     `len==cap → FUN_140b5fff0`（grow，Location 0x142ae39a0 = crates/…/steam/mod.rs）；`len++`。
  4. 每块尾 `*(byte*)(lVar33 + 0x8d) |= …`（联动字节）。
- **22 个输入码（代码序，O）**：`0x400,0x500,0x600,0x700,0x900,0x800,0xa00,0xb00,0xc00,
  0xe00,0xd00,0x200,0x300, 0x1300,0x1400,0x1600,0x1700,0x1800,0x1500,0x1200,0xf00, 0x101(尾块)`
  （round2 只列了 0x200..0xf00 14 个；**实际 21 个动作码 + 尾 0x101**）。
- **码→动作名配对**需 ui-input 域游戏 InputEvent 枚举（跨域依赖，本域不臆造）。动作候选表
  （SteamMainActionSet::action_handle 0x1417c3720 表序 O）：CameraPan/cursor_move/camera_height/
  left_click/right_click/undo/redo/tool_building/tool_path/tool_decorator/tool_customize/tool_eraser/
  tool_terrain/tool_garden/tool_tree/tool_water/time_of_day/photomode/toggle_snapping/zoom_in/
  zoom_out/main_menu/ui_cancel/ui_confirm/ui_up/ui_left/ui_right/ui_down。

**其余函数（O）**：
- `user_id @0x140ae96f0`：每次现算 `self.client.user().steam_id()`（Arc 临时 clone+drop）u64。
- `client @0x14067dd80`：`mov rax,rcx`（client 字段@+0）。
- `is_floating_gamepad_text_input_open @0x140ae9470`：读进程全局字节 **0x143634d94**（回调缓存）。
- `input_show_binding_panel @0x140ae9480`：客户端非空 → ShowBindingPanel(handle)。
- `add_steam_integration @0x140945320`：插件 build，0x30B 系统配置 {1,1,0,0,8,0}（8 系统注册）。
- `open_overlay_or_browser_to_webpage @0x14026d3c0/<String>@0x14026d580`：overlay 可用 → 开 overlay，
  否则 open_browser_to_webpage（@0x14026d080）。`can_use_overlay/validate_param @0x140aef1a0`。

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::steam::_::impl$0::get_param`（行 449）— 查询/计算
- `country_core::systems::steam::_::impl$0::validate_param`（行 449）— 谓词/校验

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 5 项
