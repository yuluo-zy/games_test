# steam 域 逐函数逻辑判等表（rebuild-steam, 2026-10-17）

> 范围：`src/systems/steam/`（mod.rs 23 + startup_steam.rs 4 = 27 原函数）+
> `src/resources/steam.rs`（22 原函数）。本域此前从未实现（src 无对应文件）——
> 无 Rust body 可对，全部原函数按控制流 fresh 反编译后重建。判等口径 =
> 重建体与 fresh 反编译的控制流逐条对应；无任何函数存在旧 body 可判"已实现"。

verdict 定义（2026-10-16-logic-verify-protocol.md）：
- `重建`：fresh 反编译（ReVa/Ghidra shared）+ RNE 机器码核验后按控制流实现。
- `胶水-非业务`：derive/serde/注册/SystemParam/内联访问器/closure，按 AGENTS.md 不审计，
  标记 glue-legitimate 不按业务重建（derive 缺失才修）。

## resources/steam.rs（22 原函数）

| 函数 | VA | 源码行 | verdict | 分歧点 / 重建依据 |
|---|---|---|---|---|
| SteamMainActionSet::read_state | 0x1409e86b0 | 67 | 重建 | 3 analog(Vec2) + 25 digital(bool)；Arc clone 门 + drop_slow。重建于 `SteamMainActionSet::read_state` |
| SteamMenuActionSet::read_state | 0x1409e8bf0 | 67 | 重建 | 1 analog + 9 digital。`SteamMenuActionSet::read_state` |
| SteamMainActionSet::action_handle | （内联） | 89 | 胶水-非业务 | 句柄数组访问器，内联无独立符号 |
| SteamMenuActionSet::action_handle | （内联） | 89 | 胶水-非业务 | 同上 |
| SteamMainActionSet::try_load<Input> | 0x1417c3720 | 102 | 重建 | 动作集 "GameControls" + 3 analog + 25 digital 逐名查句柄；集句柄失败→loaded=0，句柄单项失败仅 error 日志不回滚。动作名串长按反编译 local_f8 逐条核对 |
| SteamMenuActionSet::try_load<Input> | 0x1417c5770 | 102 | 重建 | Menu 集 1 analog(cursor_move) + 9 digital；同上语义 |
| impl$14::try_load::closure$0 | — | 105 | 胶水-非业务 | log 闭包 |
| impl$21::try_load::closure$0 | — | 105 | 胶水-非业务 | log 闭包 |
| SteamMenuActionSetState::is_any_active | （内联） | 126 | 胶水-非业务 | 谓词与 Main 同构，重建为共享方法 |
| SteamMainActionSetState::is_any_active | 0x1409e8aa0 | 128 | 重建 | **RNE 机器码定案**：任一 analog `max(|x|,|y|)>=0.25`（0x142925950=`0000803e`；NaN 不触发）+ 任一 digital。`mov al,1`+逐字节 cmp+jne |
| SteamMainActionSetAnalogAction::name_c_str | （内联） | 154 | 胶水-非业务 | 名字访问器 |
| SteamMainActionSetDigitalAction::name_c_str | （内联） | 168 | 胶水-非业务 | 名字访问器 |
| SteamMainActionSetAction::name_c_str | （内联） | 194 | 胶水-非业务 | 名字访问器 |
| SteamMenuActionSetAction::name_c_str | （内联） | 194 | 胶水-非业务 | 名字访问器 |
| SteamActionSetAction::is_analog | （内联） | 231 | 胶水-非业务 | kind 谓词 |
| impl$30::from | 0x1409e8d80 | 250 | 重建(转换胶水) | RNE：`movzx eax,cl; shl eax,8; ret` = `x<<8`。重建为 `packed_set` |
| impl$31::from | 0x1409e8d90 | 256 | 重建(转换胶水) | RNE：`(x<<8)+1`。`packed_set_plus_one` |
| impl$37::from | 0x1409e8da0 | 280 | 重建(转换胶水) | RNE：`a+(b<<8)+1`。`packed_pair` |
| impl$0::into_action_value | （内联） | 354 | 胶水-非业务 | 值转换内联 |
| SteamInput::get_action_origin_glyphs | 0x1409e6280 | 385 | 重建 | 动作可用位→Get*ActionOrigins→GetGlyphForActionOrigin_Legacy→to_string_lossy；origin!=0 才映射；结果 32B/元素（Cow 复制对）。返回 `Option<Vec<String>>`（tag 0/1/2 合并） |
| impl$2::get_action_origin_glyphs::closure$0 | — | 402 | 胶水-非业务 | 迭代闭包 |
| impl$2::get_action_origin_glyphs::closure$1 | — | 406 | 胶水-非业务 | 迭代闭包 |

## systems/steam/mod.rs（23 原函数）

| 函数 | VA | 源码行 | verdict | 分歧点 / 重建依据 |
|---|---|---|---|---|
| SteamClientResource::show_floating_gamepad_text_input | 0x140ae9390 | 30 | 重建 | 全局字节 0x143634d94 未开才弹浮窗；`ShowFloatingGamepadTextInput(1,0,0,0x500,400)`+Dismissed 回调注册；结果缓存全局 |
| impl$0::show_floating_gamepad_text_input::closure$0 | — | 41 | 胶水-非业务 | Dismissed 回调闭包（库外 SDK 职责） |
| SteamClientResource::is_floating_gamepad_text_input_open | 0x140ae9470 | 52 | 重建 | RNE：`movzx eax,byte[0x143634d94]; ret` |
| SteamClientResource::input_show_binding_panel | 0x140ae9480 | 55 | 重建 | 客户端非空→ShowBindingPanel(handle)；`lVar1!=0` 门 |
| impl$8::default | — | 96 | 胶水-非业务 | Default derive |
| AppSteamIntegration::try_new | 0x140ae94b0 | 102 | 重建 | 常量 app id 2198150（串"2198150"解析）→ info 日志 → `Client::init_app`（库外经 sdk_factory 注入）；返回 Result 判别 0=Ok |
| AppSteamIntegration::client | 0x14067dd80 | 119 | 重建 | RNE：`mov rax,rcx`（client 字段 @+0 按引用返回） |
| AppSteamIntegration::user_id | 0x140ae96f0 | 123 | 重建 | 每次现算 `client.user().steam_id()`（Arc 临时 clone+drop），不缓存字段 |
| impl$4::add_steam_integration | 0x140945320 | 133 | 重建 | 插件 build：insert 资源 x2、HookScreenshots(1)、ScreenshotRequested 回调、PreStartup startup_steam、Update update_steam_inputs.in_set(GameSystemSet)。库内重建为 `add_steam_integration(world)` |
| impl$4::add_steam_integration::closure$0 | — | 145 | 胶水-非业务 | 截图回调闭包（库外） |
| impl$9::default | — | 163 | 胶水-非业务 | Default derive |
| update_steam_inputs | 0x140ae9740 | 171 | 重建（算法级） | **8 参数签名**（in_set PDB 全型）。Arc 克隆门→帧计数→RunFrame+GetConnectedControllers→动作集 try_load→EscMenuState 门(tag==2‖(tag==10&&flag!=1))→逐手柄 ActivateActionSet+read_state+is_any_active→controller_deadline=now+250ms→analog→current.analog→22 同构块（沿变化簿记 +0x170 减/+0x178 增 + 压码）→Menu 集→尾块右键手势（InputCursorDelta 阈值 3.0→current.digital[1]）。**重建注**：16B/槽 SIMD 位图表简化为逐动作 `prev_digital` 标志；22 码序（0x400..0x101）与反编译逐条核对 |
| update_steam_inputs::closure$0 | — | 422 | 胶水-非业务 | 块闭包 |
| _::impl$0::apply | 0x14003bb80 | 449 | 胶水-非业务 | bevy SystemParam（ICF 折叠，1B ret no-op） |
| _::impl$0::get_param | （内联） | 449 | 胶水-非业务 | SystemParam 查询 |
| _::impl$0::init_state | 0x140af0460 | 449 | 胶水-非业务 | SystemParam init（SteamClientResource + Events<OsWindowCmd>）；已并入 add_steam_integration |
| _::impl$0::new_archetype | 0x14003bb80 | 449 | 胶水-非业务 | SystemParam no-op |
| _::impl$0::queue | 0x14003bb80 | 449 | 胶水-非业务 | SystemParam no-op |
| _::impl$0::validate_param | （内联） | 449 | 胶水-非业务 | SystemParam 校验 |
| SteamOverlayOpenResources::can_use_overlay | 0x140aef1a0 | 456 | 重建 | 客户端 @+0x20 非空→IsSteamRunningOnSteamDeck；空→false |
| SteamOverlayOpenResources::open_overlay_or_browser_to_webpage<&str> | 0x14026d3c0 | 466 | 重建 | 客户端空→开浏览器；非空→Deck 检查（结果不参与分支）→activate_game_overlay_to_web_page |
| SteamOverlayOpenResources::open_overlay_or_browser_to_webpage<String> | 0x14026d580 | 466 | 重建 | String 变体：消费 URL，末尾释放 String |
| SteamOverlayOpenResources::open_browser_to_webpage<&str> | 0x14026d080 | 480 | 重建 | webbrowser::open（库外注入）；成功→发 `OsWindowCmd` 事件（原 Events<OsWindowCmd>）；失败→"Failed opening browser window" 日志 |

## systems/steam/startup_steam.rs（4 原函数）

| 函数 | VA | 源码行 | verdict | 分歧点 / 重建依据 |
|---|---|---|---|---|
| impl$0::check_get_action_set_handle | （内联） | 12 | 胶水-非业务 | 句柄校验包装 |
| impl$0::check_get_analog_action_handle | （内联） | 22 | 胶水-非业务 | 句柄校验包装 |
| impl$0::check_get_digital_action_handle | （内联） | 32 | 胶水-非业务 | 句柄校验包装 |
| startup_steam | 0x140a9bbf0 | 42 | 重建 | 客户端 Arc 门→`input.init()`→构造 SteamInput（HashMap RandomState）→Commands::insert_resource。原注册于 PreStartup 首帧一次性 |

## 汇总

- 原函数总数：49（resources 22 + systems/mod 23 + startup 4）。
- `重建`（业务，fresh 反编译逐条对应）：**22**（纯业务 19 + `from×3` 转换胶水 3）。
- `胶水-非业务`（glue-legitimate，不按业务重建）：**27**（内联访问器/谓词 10 +
  SystemParam 7 + closure 5 + Default 2 + 内联校验 3）。
- `判等`（存在旧 body 且逐条判等）：**0**（本域此前无实现）。
- `未实现-需重做`（有 body 但不对）：**0**。

fresh 反编译证据：`recovery/agent-team/work/rebuild-steam/decomp/`（ReVa/Ghidra shared）；
RNE 机器码：`work/rebuild-steam/results/radare-small.json`（is_any_active/from×3/
is_floating_gamepad_text_input_open 尾部指令核验）。
