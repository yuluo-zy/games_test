# M01 公共提示：真实组件身份与生命周期复核（2026-10-26）

本轮修复两个实际接口断点：三个条件消费者从不存在业务生产者的 `FocusSubjectByte` 改读原版 `HintType`；动画和删除冷却从 `GameTime` 改读原版 `Time<()>`。保留 B 版目录与已有动画、模板、布局、持久化结构。

“HintAnimation 少了属性数组”是旧记录误报，现撤回。没有增加数组，也没有增加伪组件注入系统。`WorldPoint`/`Bounds` 的零载荷也不是本轮缺口：其真实消费者读取已有预览资源，未重复制造几何。

## 原版证据与判定

执行原件：`B版本内容/tiny-glade.rne`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。PDB SHA-256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。全部原指令 oracle 使用该 RNE，不把分析副本当执行原件。

| 原函数 | 原接口/原指令 | 结论与修复 |
|---|---|---|
| `hint_focused_edit_condition` `0x14227da70` | PDB 参数候选 `15deff16641845a7bf3529aa` 明确 `Query<&HintType>`；首项字节比较 1 | 当前提示为 FocusedEdit 才发完成音效。查询改读 orchestrator 真实组件 |
| `hint_clutter_focused_edit_condition` `0x141c3d890` | 候选 `cf177bf3f95a1f181c27219d` 明确 `Query<&HintType>`；比较 9 | 当前提示为 ClutterFocus 才发完成音效，不把工具模式等同提示类型 |
| `hint_undo` `0x142284220` | 候选 `9b737df0df38582643b7f5a2` 明确 `Query<&HintType>` 与 `Res<GameTime>`；比较 2 | 真实 Undo 展示期间累计游戏时间，严格 `>10` 完成；redo 仅此分支完成，保留原行为 |
| `hint_animation_system` `0x141c4f400` | 候选 `b485aa43425abc76d7f0f8bb`：`Query<(&mut HintAnimation, Option<&HintMarkedForDeletion>)>, Res<Time<()>>` | 真实帧时间驱动淡入、淡出、图帧时间，暂停游戏仍前进 |
| `hint_deletion_cooldown_system` `0x141c4f790` | 候选 `e9be85bc5a7c25b81ac324ec`：`Query<(Entity,&mut HintDeleteAfterCooldown)>, Res<Time<()>>, Commands` | 真实帧时间减少冷却；`<=0` 删除冷却并加入删除标记 |
| `HintAnimation::register_required_components` `0x141c53530` | `0x141c537f9` 写 align=4；`0x141c53804` 写 size=12；循环行偏移 `row*12`，分别读写 `+0/+4/+8` | 单个组件三个 f32；无“属性数组漏实现”证据。增加 `repr(C)` 与大小/对齐断言说明当前确切布局 |

原参数摘录在 [m01-public-hints-system-params.json](m01-public-hints-system-params.json)，同时记录摘录来源与 SHA。实时 Ghidra 完整响应在 [m01-public-hints-contract-native.json](m01-public-hints-contract-native.json)。该后端元数据为 `tiny-glade.exe`、131517 个函数，保留了旧安装路径；其 metadata 不返回文件 SHA，故不以该路径单独宣称原件身份。执行与静态指令补充均直接绑定上述 RNE。

原件汇编分别在 [动画](m01-public-hints-animation.asm) 与 [组件 descriptor](m01-public-hints-animation-descriptor.asm)。`MAXSS/MINSS` 的源操作数顺序会传播 NaN，不能依反编译伪码改成吞 NaN 的 maxNum。当前 Rust `clamp` 已经匹配，无须修改数学算法。

## 实际业务链与验收

1. `hint_undo` 消费真实擦除事件，输出真实 `DisplayHintRequest`；`hint_orchestrator` 创建含 `HintType::Undo`、文字和绑定显示树的实体。再次执行 Undo 消费者，真实提示实体触发计时；10 秒不完成，15 秒完成。
2. `hint_orchestrator` 创建 FocusedEdit/ClutterFocus 实体，原 `ModeJustChanged` 触发两个条件系统；对应 `HintType` 才发 `PlayUiSound(0x13)`。原杂物 hint 复用 FocusedEdit 展示文本时不发 ClutterFocus 音效，按原版判定保留，未为让测试通过而改 id。
3. `TutorialHintComplete` 进入生产 `tutorial_completion`，置真实 `GameProfile` 标志并发 `SaveGameProfileCmd`。使用现有 GameProfile RON schema 序列化/恢复，在新 World 执行真实 `hints_startup`，恢复对应完成条件。本轮没有改教程 schema。
4. 游戏 `GameTime.delta=0`，真实 `Time` 前进：开始音效 20 → 淡入完成 → 冷却到期 → 删除标记 → 结束音效 21 → 淡出归零 → 实体删除。
5. 原资产测试让真实动画在 150ms 产生 eased=0.5，进入原模板 `hint_draw`，核对帧缩放、锚点、布局记忆、提示进度图；通过真实删除系统卸载实体，MEI 下一帧节点清空。
6. 递归组合显示加载原模板与原字体，真实 `LabelContent` 准备文字并生成 `tg_text` 图元，核对类型化键、鼠标、WASD、图帧、分隔组合与颜色透明度。

第 3 项覆盖生产完成消费者、保存请求、原 schema 和重新进入初始化，未在此测试调用生产磁盘目录写入。生产存档文件原子写入与宿主保存调度仍由中央联合验收负责，不能把 schema 往返描述成真实文件落盘验证。

## 原指令差分

[probe_m01_hints_animation_native.py](probe_m01_hints_animation_native.py) 执行 `0x141c4f58c` 至 `0x141c4f4e6`，包含除法、删除方向、原 SSE clamp、原 `0x140c95ef0` easing 调用和 `time+=dt`。仅提供已解析 ECS 行、Time 帧时间、删除标记、change tick 存储，跳过 ECS archetype 查找/调度；没有外部 helper 桥。

2720 例覆盖开启/删除、阈值、负零、越界值、正负无穷、两个 NaN payload、2048 个固定种子随机样本。全部 `value/eased/time` 的 f32 位型精确匹配。原夹具 [m01-hints-animation.ron](../../tests/fixtures/native/m01-hints-animation.ron) 与 [原执行记录](m01-hints-animation-native.json) 分别绑定 SHA。

首次候选测试在第 682 例的帧时间输入断言失败：任意 f32 经 `Duration::from_secs_f32` 后再 `as_secs_f32` 可能改变 1 ULP，尚未进入候选动画差分。已保留 [首次失败](m01-public-hints-tests-first-failure.log)。修正夹具边界为真实整数纳秒 Duration，再执行原版算法；仍精确比较输入与全部三个输出位型，未放宽任何数值误差，也未修改 easing 库来掩盖问题。

## 已执行命令与范围

```sh
/Users/yuluo/Desktop/game/tools/venv/bin/python tiny-glade/reports/module-recovery-2026-10-23/probe_m01_hints_animation_native.py
cd tiny-glade
cargo test --offline -p system-onboarding --lib
TG_INSTALL_ROOT='/Users/yuluo/Desktop/game/B版本内容' cargo test --offline -p system-onboarding --lib -- --ignored
```

结果：普通 **77 通过，0 失败，2 ignored**；原资产 CPU **2 通过，0 失败**；上述原指令夹具 **2720/2720**。候选为 `aarch64-apple-darwin`、Rust 1.98.1、debug/unoptimized。源码、Cargo、fixture 与候选可执行文件 SHA 在 [m01-public-hints-verification.json](m01-public-hints-verification.json)；日志分别为 [普通测试](m01-public-hints-tests.log)、[原资产测试](m01-public-hints-original-assets.log)。首次构建曾被 rectangle owner 尚未整合的 WallId 类型阻断，待其修复后重测通过，未修改其源文件。

本轮没有进行 OS 输入、GPU 窗口、原版调度器或全游戏性能验收。已有生产宿主注册 TimePlugin，未新增零时间注入或修改渲染引擎。

## 下一轮复用经验

- 原系统 Query/Resource 身份先看 PDB 全部类型化参数，再追真实 producer/consumer；物理 ICF callee 名不能证明组件身份。把未知字节壳改成另一个自动注入壳，依旧不能恢复真实链。
- `entity_row * component_size` 不是属性数组。先核 descriptor、query item 与三处字段消费，再列复杂度缺口；证伪旧报告需要撤回，不能把撤回误报计为新增实现。
- UI 生命周期时间与游戏时间必须逐系统确认；动画/冷却是 Time，Undo 累计和位置平滑是 GameTime，不能全局替换。
- f32 差分入口必须区分原始浮点载荷与 Duration 真实可表示域。保留失败证据，修正输入边界，再跑原指令，不能用 ULP 容差掩盖接口转换。
- 验收必须包含真实组件生产者、事件消费、状态权威、保存请求、原 schema、模板绘制和卸载；同时明确各测试排除的原 ECS、磁盘调度、GPU 与 OS 输入边界。
