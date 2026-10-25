# TB-R1 键位设置修复与独立复审（2026-10-24）

本轮恢复 `gui_controls` 键位业务链和它必需的权威键位／输入消费者。按 B 版原 owner 在 `country-core/src/game_key_bindings.rs` 补齐缺失模块，继续使用现有 `GameProfile.key_bindings`；没有第二套配置。`settings.rs` 顶层接线由中央 owner 完成，本轮只审查该调用方。输入 OS 适配只修改 `event_loop.rs` 键盘／鼠标两个分支，未修改窗口或渲染生命周期。

## 恢复内容与原证

| 链路 | 原入口／证据 | 恢复及复审结论 |
| --- | --- | --- |
| 权威动作身份 | `get_keycode 0x14089d300`、`action_desc 0x14089cb60` | 37 动作、3 鼠标动作；前 6 个动作枚举和档案字段顺序不同，明确投影；描述保留本地化、方向后缀、真实图标三类。 |
| 改键冲突算法 | `assign_key 0x14089b2a0`、`assign_mouse_button 0x14089b280` | 键盘只清 35 字段的相同键，water/experimental_camera 不参与清重，然后写目标；原非对称规则完整保留。鼠标只写目标，允许共享。 |
| 显示文字 | `keycode_to_str 0x14089d490`、静态表／鼠标 helper | 194 命名键逐项核对，保留原 `AltRight -> Right Right`；None 显示 `?`，不支持显示名但合法的键显示 `???`。鼠标标准／Other 文字分别恢复。 |
| 浏览分组／行 | `key_bindings_sub_menu 0x1400854a0`、原 slots | 保留 7 分组、相机多键行、鼠标 Pan/Rotate/Focus、滚轮说明、demo 隐藏 Tree/Water、工具图标、UI 分组、Reset 和 Experimental。按用户范围删除 Feedback 编辑行，保留旧档 slot。 |
| 采集／确认 | 顶部 `0x140245790` 中 `0x140246304..0x140246cc0`、modal slots | 原 tag Keyboard0/Mouse1/Reset2/Browse3；Keyboard 先 last 后过滤，帧计数至少 2，排除 Escape，未过滤 repeat；Mouse 先 last，仅排除 Left，包含释放事件且可更新候选。确认才写档案，取消不改值，Reset 必须显式二次确认。 |
| 页面结构与输入 | 原 header/category/row/modal 单态化 slots | 标题 90/Weight800；行文字 24/图标高48；modal 32/64；分类宽350/字号60。数值按钮身份独立于显示文字，声音0/2，Back 热键18和声音9。Browse 读取后立即清真实 TabSwitcher.delta，modal 不消费该 delta。 |
| 输入权威消费者 | `translate_input 0x1417cd0a0..0x1417ce33e`、Bevy0.16类型 | 稀疏 typed 集合完整保留 Unidentified 的 5 平台 variant/payload，以及与标准按钮不同身份的 Other(0..4)。OS事件经同一个 typed apply_event 进入 facade。已覆盖权威改键、动作边沿、保存重载与 SessionLoaded 清理。 |
| 菜单门 | `0x1417cd20b..21e` 分支、菜单 `0x1417cd224`、世界 `0x1417cd4e0`、公共尾 | 修正旧实现把全部数字动作和历史事件放入 world 门的错误。菜单仍处理 UI/公共快捷键／历史；Escape 在菜单发 ui_cancel18、世界发 main_menu27；相机／工具／手势仍受原 world 门控制。 |
| 顶部生产接线 | 当前 `settings.rs::do_gui_escape_menu_settings` | Closed 重置 local；只在对应采集态推进 EventReader；传真实 settings TemplateView、权威 GameProfile、原 demo 标志；确认／重置发布 SaveGameProfileCmd；返回 Controls。中央拥有顶层真实 system adapter 测试。 |

原 `show_gui_controls()` TODO 经核对没有对应 B 骨架符号或调用方，历史地址 `0x1402ef5e0` 也不是函数入口；经中央确认删除该伪 API，作为误报纠正，不当作原业务删除。现存 `show_control_tweakers` 保留。

## 验证与有限域

- 权威模块 4 测试通过：原完整 `assign_key` 无桥执行 14,430 命名键／None 冲突样本；另 1,628 原生载荷样本；原完整 `keycode_to_str` 194 样本；鼠标共享与序列化往返。全部 37 字段比较语义值，不比较 padding。
- 输入 backend 20 测试通过：含原完整 `translate_input` 1,320 样本；生产枚举保留的 1,080 样本逐 40 位动作 bitmap 一致。原已按用户删除的页面 tag6/9 保留在原证、不强加回生产枚举。另有 typed 事件→真实 facade→GameProfile→translate_input/calculate_input_delta 的系统测试，覆盖平台载荷、Other 身份、帧清理、按住／释放、实时改键、序列化重载、SessionLoaded。
- UI 4 测试通过，含 2 个显式启用的原模板／字体 CPU 测试：浏览、真实行按钮、候选和确认、取消、重置 No/Yes、返回、真实工具 ImageContent、鼠标 Other Released 候选／确认、RON 往返。其余 2 个纯测试覆盖采集边界和完整分组／demo／极值 Tab。
- 最终 `tiny-glade --lib --no-run` 构建成功。后续遵从中央磁盘约束，直接运行已有 binary，不再排新 Cargo 构建。精确源码和日志 hash 见 `toolbar-keybindings-final-review.json`；3 份最终小测试日志已同目录保存。
- `translate_input` oracle 执行完整原函数，但 4 个桥只替代锁定 Bevy/hashbrown 的按键集合查找；输入固定无 Steam、无鼠标／滚轮／光标。当前 fixture 比较数字动作 bitmap，不宣称 Hashbrown、Steam、原生 OS 或完整模拟轴的全域差分。正常模拟轴仍由既有 backend 测试覆盖。
- CPU 原模板验证不代表 GPU 绘制／present 或真实 OS 鼠标键盘验收。用户明确暂不进行真实输入验收；Steam SDK 按用户要求排除。渲染、顶层系统验收由中央报告统一说明。

## 性能和结构

绑定表由 `Local` 缓存，仅档案键位变化时重编译。普通键继续数字快路；只为数字接口不能表达的原生键／低值 Other 维护额外集合，没有每帧重建完整 typed 输入集合。分类 delta 用有界 O(1) 同余实现，保留 i32 极值语义。未宣称未测量的 FPS 或帧时提升。

## 下一轮可复用经验

1. **先区分动作枚举、序列化字段和动作槽三种顺序。** 前六个动作的命名相近不能据此直连，必须读取跳表；逐字段完整差分比只测目标字段更容易发现非对称清重规则。
2. **事件 last 的位置是业务语义。** `read().last().filter(...)` 与 `filter(...).last()` 不等价。不能凭字段附近的 0/1 猜测 repeat／state 或鼠标按下／释放，先结合锁定类型布局与原指令读取宽度。
3. **保留 UI 控件的原身份。** 动态键名不能用作 stable node ID，否则改键后焦点／输入消费状态可能迁移；还需复核匿名子节点、声音、导航和字号，而非只核对页面文字。
4. **页面能改键不代表业务闭环。** 合法 Unidentified/Other 若在下一帧数字投影中 panic 或别名，UI 测试仍可通过。必须沿原生产事件→权威档案→实际消费者→保存重载→重置继续查。
5. **菜单不能用“整个输入系统禁用”近似。** 读取分支汇合点才能区分 world 路和 UI／公共尾。仅测 Closed 状态会掩盖设置页面键盘完全失效的问题。
6. **Ghidra size 不是连续地址跨度。** 非连续 body 的总字节数不能直接作为 stop address；坏 CFG 的 C 输出可能夹入别的函数。采用有界原反汇编、jump table、slot 和完整正常返回路径互证。
7. **差分需要写清桥和有限域。** 依赖容器查询可桥接到已验证的输入集合边界，但不能跳过业务分支再宣称原函数一致；原函数完整返回、输入域、排除范围及证据 hash 一起保存。
8. **独立 review 后再收口。** 本轮二次审查补出标题 Weight800、Tab delta 消费和菜单公共尾；先前“函数已有实现”不等于该子链完成。原 B owner 文件确实缺失时才补同路径模块，不扩散新目录。
