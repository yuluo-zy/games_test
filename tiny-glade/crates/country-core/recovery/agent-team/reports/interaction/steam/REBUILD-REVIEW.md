# REVIEW — steam 域真实逻辑重建(rebuild-steam 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: rebuild-steam REBUILD.md(resources/steam.rs + systems/steam/{mod,startup_steam}.rs
991 行,LOGIC/REBUILD 落盘);steam 测试 7/7。
编译注:现全仓 check 唯一 error 在 **rhapsody** crate(transient_resources.rs/buffer.rs/
texture.rs E0061,**rhapsody 组在途**,非 steam 域文件;REBUILD 已预告此暂断,settle 后复验
通过,以最终为准)。

## 逐项复验

### ① 分类(49 = 27 胶水 + 22 重建)— PASS
27 胶水-非业务:内联访问器/谓词(10,含 is_any_active Menu 与 Main 同构)、内联校验包装(3)、
bevy SystemParam(7,ICF 1B ret)、derive Default(2)、回调/迭代闭包(5)——不重建 ✓。
22 重建:19 纯业务 + 3 `from` 转换(impl$30/31/37::from = packed_set/packed_set_plus_one/
packed_pair,RNE 机器码,非 derive)✓。

### ② 重点 — PASS
- `is_any_active`(0x1409e8aa0):任一 analog `max(|x|,|y|)>=0.25`(阈值 0x142925950=0000803e
  =0.25 f32,NaN 不触发 ucomiss 无序遍历)✓。
- `update_steam_inputs` 8 参签名(add_steam_integration in_set PDB 全型):Arc 克隆门→
  frame_count++→frame_actions 清空→RunFrame/GetConnectedControllers→try_load 首帧→
  **EscMenuState 门 tag==2‖(tag==10&&flag!=1)**(同 translate_input)→逐手柄 ActivateActionSet
  +read_state+is_any_active→**controller_deadline=now+250ms**→模拟→analog[2]camera_pan/
  analog[3]height×0.1/analog[4].x zoom×dt×30×1.5→**22 同构数字块**(压入序 0x400,0x500,
  0x600,0x700,0x900,0x800,0xa00,0xb00,0xc00,0xe00,0xd00,0x200,0x300,0x1300,0x1400,
  0x1600,0x1700,0x1800,0x1500,0x1200,0xf00,0x101)→Menu 集→current.digital[18]→尾块
  InputCursorDelta 最末事件长度 + 右键松开<3.0→current.digital[1] ✓。

### ③ 登记诚实 — PASS
- SteamSdk trait 注入边界(库外 steamworks/webbrowser 只审调用语义)、Instant::now
  now_hint 占位——注入/占位,不冒充 ✓。
- update_steam_inputs **算法级重建**(16B/槽 SIMD 位图→逐动作 prev_digital:[u64;25] 标志),
  可观察行为(码压入/计数/模式截止)一致,但不算逐指令判等——LOGIC 标"重建(算法级)"诚实 ✓。
- 22 码→动作名配对跨域依赖(SteamActionSetDigitalAction 变体名按 try_load 表序**候选**标注,
  待 ui-input 游戏 InputEvent 枚举确认,未臆造定论)✓。
- OsWindowCmd 本地承载(原 resources::window::OsWindowCmd 未落库),消费端接线前并入
  登记 ✓。

### ④ 测试 — PASS(协调器 7/7)
resources::steam 5(码序唯一性、名表序、0.25 阈值+NaN、打包转换机器码)+ systems::steam 2
(DIGITAL_BLOCKS/DIGITAL_CODES 22 码压入序)。

## 结论与 DISPATCH 建议

- steam 域重建复验通过:**实现层面闭合**(分类/重点/登记全部对得上);库外 SDK 注入边界、
  动作名跨域确认、OsWindowCmd 并入为登记待接。
- 建议:steam 域 → 闭合(实现层面);rhapsody E0061 为他组在途,不属 steam。库外项
  (steamworks 实参/动作名配对/窗口消费)由跨组接线时闭合。
