# 主分支合并与 M01 完成情况复核（2026-10-28）

**结论：最新主分支已同步并与本地恢复工作合并，M01 公共交互／toolbar 尚未完成。** 本文按此前模块编号解释用户的“m0”，以当前源码、真实调用和本轮实际测试判断，不从统计条目或旧完成声明推算。

## 合并基线与保留范围

主分支由 `8408f140f` 快进到 `origin/main` 的 `9028e2db4`，新增87个提交。本地原有170个已跟踪文件修改，19个路径与远端重叠。全部修改及未跟踪文件先完整保存到 stash `a296a5ca14ab4d42de4897642c807de5c6f49956`，保留该备份；恢复后核对2874个未跟踪文件条目，没有缺失或内容差异。目录形式的原始status条目数与逐文件计数不同。

- 沿用远端新渲染目录，将旧 `rg/vulkan.rs` 的常量页退役修复迁至 `rg/vulkan/session/frame_imports.rs`；没有重建旧聚合文件。对应回归改用新的公开 `resource` API。
- 远端 `bb6c51087` 的形状移动／圆编辑仍有明确TODO；保留本地已有原证、真实消费者和差分支持的完整实现。双方原始版本分别保留在主分支历史与stash，未按文件长度或提交日期裁定算法正确性。
- 高度系统保留原B组的位置，消除双方整合造成的重复注册；边界插值和模块声明也消除重复定义。`OnShapeMoved` 使用真实生产者与消费者的同一事件类型，删除另造的单WallId占位事件。
- 保留远端新增的输入适配→高度→历史→undo/redo集成测试。其原夹具缺发布阶段路径高度，并瞄准未抬高的位置；首次运行两项失败已保留。夹具补真实路径准备并按原聚焦控件+0.6的几何位置瞄准，生产几何／断言均未放宽，最终高度组52项通过。
- 工具指南保留远端迁移后的 `docs/current/AI_REVERSE_TOOLS.md` 与Mac入口，合入本地工具失败／CFG／SSE经验，不恢复旧入口文件。

## 本轮验证

| 实际执行 | 结果 | 边界 |
| --- | --- | --- |
| `cargo check --offline -p tiny-glade --tests` | 通过 | 含测试目标的类型／API整合，不证明业务验收 |
| `toolbar --lib`，显式原B资产目录 | 109通过、9忽略 | 第一次缺资产目录的3项失败单独保存；忽略项未当通过 |
| core `systems::wall::shapes::`，native-physics | 54通过 | 形状叶子与真实World链，不证明窗口输入 |
| core `adjust_wall_height`，native-physics | 52通过 | 含保留的远端真实输入适配及undo/redo测试 |
| 原25叶形状系统／五消费者顺序 | 1通过 | 测试手工添加cache目标，不能冒充生产视觉父链安装 |
| `toolbar_module_chain`／`toolbar_focus_chain` | 8／2通过 | 含保存文件和新进程恢复；子进程测试不重复累计 |
| 常量页外部Arc／Weak释放回归 | 本机真实Vulkan 1通过 | 强制GPU、不新增idle等待；不等同完整游戏present或性能验收 |

命令、原失败、源码指纹、日志SHA及范围详见 [机器可读复核](main-sync-review-2026-10-28.json)。本轮不发送真实OS鼠标键盘输入。

## 当前仍未闭合的实际代码

1. **装饰交互与视觉父链没有进入生产App。** `run_game.rs`调用的是`decorator_plugin::build_startup_resources`；`build_recovered_visual_systems`没有调用点，`register_ui_systems`也仅存在于未调用的聚合build内。恢复了Place／Move／旋转、生成器和validator的函数，不等于这些函数在实际游戏中运行。原完整父链仍需按原顺序装配，不能简单挂旧子集替代。
2. **楼梯装配主函数仍为空。** `decorator_visual/stairs_assemble.rs::playermade_stairs_assemble<const X: bool>`没有实现体；两个原实例都没有完成。梯步、栏杆、楼板等局部函数通过不能替代这一入口。
3. **结构图变形仍为空结果。** 同文件`calculate_distortion_from_structural_graph`仍直接返回`Vec::new()`，不具备原结构图变形业务。
4. **完整场景另有自动杂物所有权缺口。** `manage_autoclutter_entities::resolved`对丢失selected mesh/source的记录显式panic，并拒绝尚未恢复的selector。此项是当前源码检查结论；旧构建的GPU失败仅作历史线索，本轮没有声称最新合并构建已完成整场景GPU复跑。

前3项足以否定M01已经完成；第4项另外阻碍完整游戏联合验收。真实Steam SDK、Credits和许可证仍按用户决定排除，不重新作为阻断项。

接下来先补楼梯主流程与结构图变形，再装配完整交互／缓存／生成／碰撞／验证父链，最后在同一生产App验证编辑、历史、保存恢复、Reload和真实GPU画面。窗口OS输入仍遵守此前暂缓要求。未提交、未推送本轮合并工作区。

## 复用经验

Git无文本冲突不代表语义合并成功：重复模块声明、同名算法、独立注册加原tuple内注册，都可能自动合并后才暴露。新旧目录应迁移修复的行为，而不是恢复被重构删除的大文件。输入整合测试须使用原控件所需的权威发布数据与真实几何位置；保留失败反例后补夹具，不为测试通过回退原算法。
