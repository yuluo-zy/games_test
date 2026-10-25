# Toolbar 配套公共提示链修复与复核（2026-10-24）

本轮修复的是 toolbar 的提示可视消费者、关闭吸附教程通知及其持久化。它沿用 B 版 `systems/onboarding`、`country-core`、`toolbar/mei` 的目录和权威资源。**不把本轮 CPU 验证写成整个 toolbar、真实 OS 输入或 GPU 画面验收完成。** Steam SDK 按用户范围排除。

## 原证与恢复内容

| 链路 | 原入口／关键指令 | 当前恢复 |
| --- | --- | --- |
| 形状吸附通知 | `14097f970`；producer 由 daily_schema 独立恢复 | 有真实候选正常吸附发 false，修饰键禁用发 true；无候选不虚发通知 |
| 关闭吸附提示 | `14227e140..14227e3d4` | 消费双缓冲整批最后事件；true 仅首次 Complete(3)；false 在原模式／profile／local 门控下展示；键来自权威 slot24，Steam query `0x0f01` |
| 实体到界面 | `141c51a60` | 原查询、枚举身份、窗口高度缩放、opacity、交互提示存在性、帧布局记忆、原文本字体、Beta 分支、progress 四图及忽略输入 |
| 递归组合 | `141c52700..141c5309c` | Empty、Graphic、完整 typed Key/Mouse/Row、Steam 前置解析、Alternate/Adjacent/Together；原模板、数字节点身份、分隔符、递归深度 |
| 静态／动画图片 | `141c52700` Graphic 分支、原 constructor/conversion | 静态单图与 Vec 动画明确分开；原 floor→usize 饱和→取模，保留空动画错误；原 height 而非虚构颜色字段 |
| 字形解析 | `14227e5e0`、`14228dd10` | 沿现有 SteamInput 解析 owner，每个字形恢复原高64；不声称真实 Steam SDK 接入 |
| 完成保存 | `140a658a0`、`140a65fe0`、`140a65c30`、`140a65e20` | 原5个顶层字段、19项 TutorialCompletionState、嵌套 ExpBackOff；保留现有公有字段作为唯一权威 |
| 初始档案 | `140916480` 三条 fallback | 原计时／冷却和完成区全零；720仅由 backoff 首次消费惰性初始化 |

`hint_draw` 继续写入生产 `MeiContext`；完整插件将它注册在 `GameSystemSet(0x24)`。无另建提示渲染层。真实字体排版与图元测试调用现有 `LabelContent::draw` 和 `TextRenderer`，不是自绘替代品。实体离开后由同一 MEI 帧回收卸载。

camera 构造器、position/highlight/animation/lifecycle 前半由 new_daily 同步恢复。本轮统一测试包含其稳定版源码；其原证和剩余 camera 条件由该 owner 独立报告。本报告不代其宣称全部 camera 条件已恢复。

## 纠正的隐蔽失败

1. 原 `NotifyHintSystemSnapping` 错作 unit，consumer 又读取恒零 seam：关闭吸附从不完成。现恢复 bool 并连接真实 shape producer。
2. 原 `GameProfile` 将 tutorial 当 bool，其他完成位跳过序列化：完成后重启丢失。真实保存／重启测试首先失败，随后追原嵌套 serializer 修复；没有放宽测试。
3. 原顶层 UUID 名为 `game_installation_id`，不是旧文声称的 `player_id`。旧工程文件作为迁移 alias 读取，新保存写原名。
4. welcome 字节 `+206` 同属原19字段持久化区；不得再写作进程内标志。
5. MEI 新节点缺前帧布局时不能把原保留锚点清零；现 memo 恢复原数据流，并分别测首帧与回收后旧布局。
6. InteractionPrompt 原分支看组件存在性，不读取人为的 `active` bool；测试以存在且 false 证明这一点。
7. Ghidra function size 为非连续 body 字节总量，不能用于截断 asm 范围。递归 render 的106字节报告并不等于函数只有106字节。

## 验证

- `cargo test -p country-core --lib game_profile::tests --offline`：**9 passed**。
- `cargo test -p system-onboarding --lib --no-run --offline`：通过。
- `TG_INSTALL_ROOT=<B版本内容> target/debug/deps/system_onboarding-fd42554164d20c61 --include-ignored --nocapture`：**50 passed，0 failed，0 ignored**。
- 关闭吸附原完整 consumer **4896** 例：双缓冲全部切分、事件顺序、模式／shape、档案与 local 完成位、命名键／None／完整平台原生键。唯一原执行桥为标准 allocator。
- Graphic 原分支 **1320** 例：单图、多帧、f32 特殊值、周期边界、饱和取模及 height bits。桥仅 libm floorf 与最终 MEI 图片输出；不将它夸称完整 MEI 指令差分。
- 教程原 serializer **279** mask：19 one-hot、全零／全一／交错与确定随机，比较原字段名、顺序、布尔指针来源。RON 内部格式化和分配失败分支在桥外。
- 实际墙权威→发布→空间索引→`shape_placement_params`→bool 通知→提示→Complete(3)→`SaveGameProfileCmd`→RON 重载；无测试碰撞命中注入，正常、禁用、重复、无候选四条路径均验证。
- 原模板／字体：typed 特殊键与 MouseOther、Row、所有组合子、真实文字 primitives 及颜色透明度；draw 的 text、图片、progress、anchor、缩放、首帧／旧布局、Beta 批消费、实体卸载。
- 真实生产临时文件原子替换写盘与读回；旧工程 bool tutorial／扁平 cooldowns／player_id 能读取，写回原嵌套格式。

日志中空动画的 should_panic 与未先 resolve Steam 的 catch_unwind 为原错误边界断言，不是未解决测试失败。编译仍有项目既有 warning，不扩大本轮范围做无关清理。

## 尚未在此轮验收的范围

- 用户暂停真实 OS 鼠标／键盘验收；未操作游戏窗口。
- 本轮没有完成 GPU 窗口截图、完整生产绘制尾部或长期资源稳定性验证；CPU 原模板、字体和文字图元不是 GPU 验收替身。
- camera display condition 由 new_daily 继续按原 `14227c910` 深查；`hint_clutter_focus_edit::make_focus` 仍存在旧 glyph_mode／Left fallback，已上报中央，不能因为本轮50测试通过而认定它完整。
- Beta warning 消费分支及事件注册已恢复；当前没有凭空制造 beta 业务 producer。
- 旧文件当年未保存的教程标志无法恢复，不推断为已完成。未知／缺字段／畸形档案的全部原反序列化失败策略未做穷尽差分。

## 下轮复用经验

先用真实 producer 与保存重启链暴露缺口，再反查被传对象、serializer 泛型单态化和静态字段常量。将原执行桥限制为依赖边界并逐项写明；保留失败断言，不用“结构已存在”代替接线。先稳定共享文件边界，再统一一次 no-run 和 CPU 原资产测试；共享文件按函数 marker 修改，不整体格式化覆盖别人代码。

原证 JSON、有限 fixture、探针脚本、静态 asm、最终日志与源码 hash 位于同目录及 `toolbar-public-hints-final-review.json`；源程序／PDB 身份沿该清单绑定。
