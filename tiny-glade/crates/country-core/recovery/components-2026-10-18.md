# components 审查与字段恢复（B，2026-10-18）

本轮处理现有 `src/components/` 的结构、注释、三个 `_gap` 载荷及其直接调用方。
三个占位已全部替换为有生产者/消费者证据的字段。类型/字段名中未由 PDB
保留的部分采用语义命名；不宣称取回作者原始 Rust 文本。

## 结构与使用

| 文件 | 职责 |
|---|---|
| `mod.rs` | 模块说明和兼容的公共导出 |
| `rendering.rs` | 渲染标记、虚线长度与粗糙度参数 |
| `drawable.rs` | 共用网格选择器和两种可绘制 Bundle |
| `trees.rs` | 树身份、预置树原名称、动画和移动弯曲状态 |
| `ui.rs` | UI 范围、拥有资源名的图像和 64 位绘制顺序 |
| `registry.rs` | 原作 57 个组件的逆向元数据清单，不承担运行时注册 |

旧的 `components::*` 公共入口继续导出相同类型。旧 `components::trees::*`
中的跨域导出保留为兼容入口；新增代码从各自模块或 `components` 导入。

## 修复的审查问题

| 问题 | 修改及影响 |
|---|---|
| `TreeAnimation` 把时长误命名为 alpha，缺少真正的逐树进度 | 恢复 duration@0、progress@4、canopy_visibility@8、hidden_this_frame@12；大小 16、对齐 4 |
| 删除系统读取时长 | `despawn_trees` 改读 progress，保留严格 `<0.01` 和 NaN 不删除 |
| 动画进度按查询序放在 `Local<Vec<_>>` | 进度随实体保存，删除其他实体或切换 archetype 后不会错配 |
| `_gap: u64` 被用作浮点/标志的手工位打包 | 树动画和移动弯曲改用有类型的字段；原运算顺序、隐藏标志消费/清除保留 |
| `TreeComponent` 和 `ConvertedPreplacedTree` 被写成空标记 | 恢复树 ID/预置标志，以及拥有 `PrefabName` 的 16 字节载荷 |
| `Ui2dImage` 只有占位，工具栏克隆名称后丢弃 | 恢复 String + u32 layer；`new` 移入名称并将 layer 设为 0，工具栏改为真正使用 label |
| `UiDrawOrder` 为 u32，绘制端按 u64 排序 | 恢复为 u64，避免高 32 位丢失 |
| 全量 Clone/Copy 派生改变 ECS 克隆策略 | 移除当前组件上不符合 B 的派生，恢复 Bevy Default；载荷值类型仍可 Clone/Copy |
| 网格选择器存在两份定义，并被误作独立组件 | render::nani 的旧入口改为同一选择器的别名，选择器不再派生 Component |
| Bundle 注释把有效复制至 0x48 当成本机分配大小 | 明确本机 16 字节对齐后的大小为 0x50；不以 Rust ABI 跨平台推断原二进制布局 |
| registry 把选择策略入口叫 handler 地址，且排序描述不符表顺序 | 改为 clone_behavior_va；说明按文件分组，区分证据库存与已恢复类型 |

`OverwriteRoughness` 未挂载时由绘制端使用 0.8；显式构造其零值 Default
是另一种情况。注释已区分这两个行为，不凭回退值改造构造策略。

## 原作证据

本轮复用 B 数据库，共享 Ghidra 只读；没有修改数据库或发行文件。

- patched EXE SHA-256：`cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`。
- 发行 RNE SHA-256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 注册描述符：TreeAnimation `0x140846ae0`、TreeMoveBend `0x1408471d0`、
  TreeComponent `0x140845d00`、ConvertedPreplacedTree `0x1408463f0`、
  Ui2dImage `0x14084f5d0`、UiDrawOrder `0x140850aa0`。
- 创建/读取：process_tree_cmds `0x142267430`、despawn_trees `0x1422686d0`、
  spawn_preplaced_trees `0x140862270`、convert_preplaced_trees `0x14226be70`、
  decorator_cursor_icon 的内联 Ui2dImage::new `0x1407adda7..0x1407addb6`。
- 原逐树动画指令 `0x142279cdf..0x142279deb`；移动弯曲 `0x1422689c0`。
- 26 段分析输入的指令字节与发行 RNE 相同。静态字节一致不等于候选运行结果等价。

原始调用、提取片段、字段合同和字节对照保存在
[本轮工作目录](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-abf1305389c44cc264/components-review-20261018/CONTRACT.md)。
已有的动画汇编证据复用
`recovery/agent-team/work/restorer-tree-forest-remaining/evidence/handplaced_hp_step0.txt`；
57 项克隆/required 清单依据原 `country-core-deep/components-geometry/ALGORITHMS.md`。

## 验证与边界

执行结果、当前源码指纹和日志见
[components-2026-10-18.json](components-2026-10-18.json)。
限定验证包括组件布局/描述符、实体进度归属、UI 名称/图层与 64 位排序键，
以及真实 `despawn_trees` 系统的事件输出。未执行原进程动画 oracle 或完整游戏/GPU 链。

初次 all-targets 检查被已有渲染 `pass_kit/test_env.rs` 的接口不匹配阻塞：
辅助实现仍用 `import_unowned_texture`，当前 trait 要求 `import_texture`。
该文件和 trait 不属于本轮修改；失败日志保留，组件检查采用独立集成测试。

工作区库检查另外被 `system-water/src/water_audio_grid.rs:173–174` 的既有
字段不匹配阻塞：WaterGridCell 没有 water_level，position 期望 [f32; 2] 却被写入 Vec3。
本轮未修改水模块；不能把相关四库检查通过解释为全工作区编译通过。

复验期间共享 rhapsody 源码曾短暂缺少 GraphImageLease，随后该类型已补入，
最终组件集成测试重试通过。相关依赖仍有并行改动；本轮 CPU 检查不验收这些后端改动。

以下是本轮范围外的既有缺口，不能用组件检查通过来升级验收：

- 原放置记录向 GPU 的 `+0x20/+0x24/+0x28` 写面、全局 canopy 状态和实际原调度。
- 当前 InsertTreeCmd 未承载原来的完整 flags/rotation 等；现有手放入口的
  TreeComponent.is_preplaced 显式为 false，尚不代表原预置树命令全链已接入。
- 原 cache_prev_object_transforms 系统和 registry 其余未定义的组件载荷。
- 其他目录的 `_gap`、完整历史/删除传播和真实渲染后端。

旧报告中 `{alpha, _gap}`、查询序 Local 进度、UI 占位、树空标记的结论仅作历史；
旧编译/测试快照不能验收当前新类型。当前状态为限定恢复和候选行为检查通过，
不标记整个 components 或整个 B 游戏完成。审查由本轮实现者对照原始证据完成，
未声称独立 reviewer 或跨版本验收。
