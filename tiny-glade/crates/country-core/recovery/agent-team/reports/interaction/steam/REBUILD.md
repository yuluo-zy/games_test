# steam 域 重建记录（rebuild-steam, 2026-10-17）

> 协议：2026-10-16-logic-verify-protocol.md「重建前必做:宏生成函数 vs 真实业务函数 分类」。
> 本域 49 原函数全无 Rust body → 全部进入分类+重建流程。分类结论记入下表，
> 重建后的 verdict 见 `LOGIC.md`。

## 分类结论（每函数：宏胶水 / 真业务 / 存疑）

### 宏生成胶水（glue-legitimate，不重建为业务）

判定依据：src 对应 derive 属性 / inventory kind（glue, inline）/ 反编译体为 fmt 分派
或空体/单指令。共 27 项，不逐个重建：

- **内联访问器/谓词（10）**：`SteamMainActionSet::action_handle`、
  `SteamMenuActionSet::action_handle`、`SteamMainActionSetAnalogAction::name_c_str`、
  `SteamMainActionSetDigitalAction::name_c_str`、`SteamMainActionSetAction::name_c_str`、
  `SteamMenuActionSetAction::name_c_str`、`SteamActionSetAction::is_analog`、
  `impl$0::into_action_value`、`SteamMenuActionSetState::is_any_active`（与 Main 同构）。
- **内联校验包装（3）**：`startup_steam::impl$0::check_get_action_set_handle` /
  `check_get_analog_action_handle` / `check_get_digital_action_handle`。
- **bevy SystemParam（7）**：`_::impl$0::{apply, new_archetype, queue}`（0x14003bb80
  ICF 折叠，1B ret no-op）、`get_param`/`validate_param`（内联）、`init_state`
  （0x140af0460）。init_state 的字段揭示已并入重建（SteamClientResource +
  Events<OsWindowCmd>）。
- **derive Default（2）**：`impl$8::default`（SteamClientResource）、`impl$9::default`
  （SteamScreenshotRequests）。
- **回调/迭代闭包（5）**：`show_floating_gamepad_text_input::closure$0`（Dismissed）、
  `add_steam_integration::closure$0`（ScreenshotRequested）、`try_load::closure$0`×2（log）、
  `get_action_origin_glyphs::closure$0/$1`。回调注册语义在库外 SDK 边界。

### 真实业务函数（重建，22：纯业务 19 + from×3 转换胶水）

按 fresh 反编译逐语句重建，全部落位 `src/resources/steam.rs` +
`src/systems/steam/{mod.rs, startup_steam.rs}`：

| 函数 | 重建位置 | 关键控制流 |
|---|---|---|
| SteamMainActionSet::try_load | resources/steam.rs | GameControls 28 动作逐名查句柄 |
| SteamMenuActionSet::try_load | resources/steam.rs | Menu 10 动作 |
| SteamMainActionSet::read_state | resources/steam.rs | 3 analog + 25 digital |
| SteamMenuActionSet::read_state | resources/steam.rs | 1 analog + 9 digital |
| SteamMainActionSetState::is_any_active | resources/steam.rs | 0.25 阈值 + 任一 digital |
| SteamInput::get_action_origin_glyphs | resources/steam.rs | origins→glyph 映射 |
| impl$30/31/37::from | resources/steam.rs | `packed_set`/`+1`/`packed_pair`（RNE 机器码） |
| SteamClientResource::show_floating_gamepad_text_input | systems/steam/mod.rs | 全局标志门 + 浮窗 |
| SteamClientResource::is_floating_gamepad_text_input_open | systems/steam/mod.rs | 读全局字节 |
| SteamClientResource::input_show_binding_panel | systems/steam/mod.rs | 客户端非空→ShowBindingPanel |
| AppSteamIntegration::try_new | systems/steam/mod.rs | app id 2198150 → init_app（注入） |
| AppSteamIntegration::client | systems/steam/mod.rs | 字段访问器 |
| AppSteamIntegration::user_id | systems/steam/mod.rs | 每次现算 steam_id |
| impl$4::add_steam_integration | systems/steam/mod.rs | 资源+系统注册（库内部分） |
| update_steam_inputs | systems/steam/mod.rs | 见下 |
| SteamOverlayOpenResources::can_use_overlay | systems/steam/mod.rs | Deck 检查 |
| SteamOverlayOpenResources::open_overlay_or_browser_to_webpage×2 | systems/steam/mod.rs | 覆盖层/浏览器分派 |
| SteamOverlayOpenResources::open_browser_to_webpage | systems/steam/mod.rs | webbrowser + OsWindowCmd 事件 |
| startup_steam | systems/steam/startup_steam.rs | init + insert SteamInput |

### 存疑（半胶水半业务）

`impl$30/31/37::from`：手写 From 转换（单指令/双指令机器码），非 derive 生成——
按机器码重建为 `packed_set`/`packed_set_plus_one`/`packed_pair`（转换胶水，保留语义）。

## update_steam_inputs（0x140ae9740）重建说明

- **签名**（O：add_steam_integration in_set PDB 全型）：
  `(Res<SteamClientResource>, Local<PrevSteamInputState>, ResMut<SteamInput>,
  ResMut<ActionSetValues>, ResMut<ActiveInputModeTracker>,
  EventReader<InputCursorDelta>, Res<GameTime>, Res<EscMenuState>)`。
- **控制流**（O，ReVa fresh 反编译 8158 行逐段）：
  1. Arc 克隆门（无客户端→return）。
  2. `PrevSteamInputState.frame_count += 1`。
  3. `SteamInput.frame_actions` 清空；released/pressed 复位。
  4. `RunFrame` + `GetConnectedControllers`；原手柄不在连接表→清 0。
  5. 动作集 try_load（Main/Menu）首帧一次性，成功 info 日志
     （"Loaded the main/Menu action set from Steam"）。
  6. **EscMenuState 门**（行 887）：`tag==2 || (tag==10 && flag!=1)`（同 translate_input）。
  7. 逐手柄：`ActivateActionSet(Main)` → `read_state` → `is_any_active`
     （模拟 max(|x|,|y|)>=0.25）→ `controller_deadline = now + 250ms`。
  8. 模拟→`ActionSetValues.current.analog`：camera_pan[2]+=左摇杆、camera_height[3]
     +=右摇杆×0.1、camera_zoom[4].x+=delta×30×1.5（行 907..920 偏移核对）。
  9. **22 同构数字块**：沿变化簿记（`released -= prev&1`、`pressed += 1`）→压 u16 码
     入 `frame_actions`（cap-first 2B 元素，grow_one<SteamActionSetDigitalAction>）。
     22 码压入序与反编译逐条核对：`0x400,0x500,0x600,0x700,0x900,0x800,0xa00,0xb00,
     0xc00,0xe00,0xd00,0x200,0x300,0x1300,0x1400,0x1600,0x1700,0x1800,0x1500,0x1200,
     0xf00,0x101`。
  10. Menu 集 `ActivateActionSet` + `read_state` + `is_any_active`；菜单数字→
      current.digital[18]（行 5999 `*(lVar16+0x9a)|=...`）。
  11. 尾块：`InputCursorDelta` 最末事件长度累加到 `PrevSteamInputState.mouse_moved`；
      右键松开且 <3.0 → `current.digital[1]`（行 8060..8136）。
- **建模简化（诚实登记）**：原 16B/槽位图的上帧状态表（SIMD 位收敛 + 槽位 tag 编码）
  简化为 `SteamInput.prev_digital: [u64; 25]` 逐动作标志。码→动作名配对为跨域依赖
  （需 ui-input 域游戏 InputEvent 枚举确认），`SteamActionSetDigitalAction` 变体名
  按 try_load 表序**候选**标注，未臆造为定论。该简化不改可观察行为（压入码/计数/
  模式截止），但不算逐指令判等——见 LOGIC.md「重建（算法级）」标注。

## 其他重建注

- **库外边界（注入）**：steamworks（`SteamSdk` trait）、webbrowser（open_browser
  回调）、Instant::now（now_hint 占位）均经注入/占位；只审调用语义。
- **OsWindowCmd**：原 `country_core::resources::window::OsWindowCmd` 未落库，本域
  本地承载 `systems::steam::OsWindowCmd`；消费端（窗口/UI）接线前并入 resources::window。
- **GameSystemSet**：`add_steam_integration` 原 in_set(GameSystemSet) 接线由宿主调度。

## 编译与测试

- `cargo check -p country-core`：**通过**（exit 0；11 warnings，均为他模块既有）。
- `cargo test -p country-core --lib steam`：**7 passed / 0 failed**
  （resources::steam 5 + systems::steam 2：码序唯一性、名表序、0.25 阈值+NaN、
  打包转换机器码）。
- 注：全 crate check 曾因 rhapsody（他域在途）E0369 暂断，settle 后复验通过；
  现场以最终通过为准。
