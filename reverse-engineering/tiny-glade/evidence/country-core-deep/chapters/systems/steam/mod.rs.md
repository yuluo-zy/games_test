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
| country_core::systems::steam::_::impl$0::apply | 0x14003bb80 | 449 | bevy 系统参数状态 init/apply/queue 胶水（steam 系统参数面，非游戏业务） | O（bevy 胶水） | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| country_core::systems::steam::_::impl$0::init_state | 0x140af0460 | 449 | steam Input 参数初始化（get_param 系） | I（胶水） | audio-misc\decomp\decomp-0x140af0460.txt |
| country_core::systems::steam::_::impl$0::new_archetype | 0x14003bb80 | 449 | bevy 系统参数状态 init/apply/queue 胶水（steam 系统参数面，非游戏业务） | O（bevy 胶水） | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| country_core::systems::steam::_::impl$0::queue | 0x14003bb80 | 449 | bevy 系统参数状态 init/apply/queue 胶水（steam 系统参数面，非游戏业务） | O（bevy 胶水） | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| country_core::systems::steam::SteamOverlayOpenResources::can_use_overlay | 0x140aef1a0 | 456 | 0x140aef1a0（validate_param 系）：overlay 可用性校验 | O | audio-misc\decomp\decomp-0x140aef1a0.txt |
| country_core::systems::steam::SteamOverlayOpenResources::open_overlay_or_browser_to_webpage<alloc::string::String> | 0x14026d580 | 466 | open_overlay_or_browser_to_webpage<String> 泛型实例（overlay 可用→开 overlay，否则浏览器） | O |  |
| country_core::systems::steam::SteamOverlayOpenResources::open_overlay_or_browser_to_webpage<ref$<str$> > | 0x14026d3c0 | 466 | open_overlay_or_browser_to_webpage<&str> 泛型实例 | O | audio-misc\decomp\decomp-0x14026d3c0.txt |
| country_core::systems::steam::SteamOverlayOpenResources::open_browser_to_webpage<ref$<str$> > | 0x14026d080 | 480 | open_browser_to_webpage<&str>：直开浏览器 | O |  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::steam::_::impl$0::get_param`（行 449）— 查询/计算
- `country_core::systems::steam::_::impl$0::validate_param`（行 449）— 谓词/校验

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 5 项
