# Toolbar / decorator / 存档本轮收尾记录

日期：2026-10-20。五个 agent 并行取证和实现，中央负责公共类型、宿主装配及独立验证。原版 B 镜像 SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。T13 反馈捕获按用户要求排除。

## 本轮结果

| 模块 | 已恢复并通过本轮验证 | 仍未闭合 |
| --- | --- | --- |
| toolbar | owned 工具栏树、普通及摄影 full/demo 装配、13 类滑块真实 owner 与拖动/重置/绘制/旋钮回收、撤销重做布局/自动隐藏、焦点传播/开始事件/动画/声音/排序 | 完整键盘与手柄导航、菜单遮挡隐藏、鼠标抢占、demo policy，以及真实游戏联合验收 |
| decorator 交互 | 真实请求字段；focused dispatcher 的已取证分支；rotate→typed change→Info/目的地/历史标记/音频/指示符子链；修复 ExtraInfo 布局和零时长动画 | 完整 tag8 控件命中入口、其余 move/place/dispatcher variants、跨帧拖动→提交→历史回放、完整音频载荷消费 |
| decorator 视觉 | 当前帧缓存与 Local 旧帧存储；地形支持/屋顶 UV 与朝向；地形 clutter→缓存→蓝图→碰撞/视觉→Nani 实际行；楼梯结构碰撞叶节点与注册表操作 | stair-attached 缓存、完整楼梯请求 producer、蓝图 validator、完整 unloading，以及实际 GPU 渲染验收 |
| 存档 | 保留第一阶段真实命令/owner/错误终态/后台文件重载修复；补充当前 owner API 兼容；现有 decorator owner 的有限卸载与修改跟踪生命周期 | 非空 Roof/Tree/颜色消费者的整游戏回载、stairs checksum/格式语料、真实设备上传与最终画面 |

现有用户口径 toolbar≈60%、decorator≈27%没有重新全量计分。新增叶节点、原码案例和 CPU 测试不直接折算整链完成率。三个主流程仍为部分完成。

## 关键正确性修复

- `DecoratorInfo` 的 ExtraInfo 是 `+0x30..+0x43` 的 20 字节，niche/tag 在 `+0x41`，与目的地 `+0x10` 独立；保留未写 padding 与 Stair niche。576 个原码转换案例覆盖完整 Info 与读回。
- 缓存当前 map 与系统 Local 旧 map 分离；插入后立即可读。修改跟踪清理只清 modified/removed，不能销毁 authoritative Info/d2s。原清理调用位于 validate_blueprints 内，不冒充通用帧尾系统。
- 预览音频用 DecoratorType 判别而非 subtype；65536 个 type/subtype 对照。旋转 typed patch 不再把 112 字节 change payload 当作 72 字节 Info。
- SignifierAnimation 按原 28 字节状态和 IEEE 运算恢复，正确处理 duration=0、标记删除和 MinimumLifespan；原码 1728 案例加真实 ECS 验证。
- 工具栏保留 FnOnce/drop 和延迟插入顺序、keep 的任意 u8、焦点 BFS/visited、跨帧 Local 集合；滑块遵循真实实体 generation niche。两个旧重置默认值按原码修正。
- `RaycastMeshData::new_convex` 使用实际 Vec3 浮点顶点，哈希仍按原始位表示；楼梯凸包失败后的原球体回退算法完整保留。

## 宿主接线与边界

工具栏真实启动入口接 OnEnter(Game)；Update/set7、set8 保留原嵌套 tuple 边界与无 deferred flush 的链顺序。沿用当前 MEI 的文本/Taffy 及已拆分的摄影系统，没有恢复旧的重复实现。正常启动入口调用 ToolbarPlugin build；剩余导航/隐藏/抢占等系统没有放空占位消费者。独立只读审查确认事件维护、Taffy/TextRenderer/EraserSizeMultiplier/BrushSizes 的真实启动路径；PhotomodeScreenshotFolder 尚无宿主 producer，打开截图文件夹分支仍未闭合。

decorator `build_recovered_visual_systems` 为候选接线入口，**尚未在 main 安装**：stair-attached 缓存未恢复时全类型运行仍可能触发 expect。现有 owner 的卸载函数是明确有限范围的实现，不能当成完整原卸载入口。ClutterMeshes/GraphicsDevice 等工程资源条件并非已证明的原作 guard。

保留现有 rhapsody 和底层工具重写；本轮无 rhapsody/utils 实质源码 diff。SharedAssetShaderLibrary 使用当前 canonical owner，未构造空假 shader/GPU 资源。

## 独立验证

最终纳入验证的 **364 项 Rust tests 通过，0 ignored**。命令均固定 stable 工具链，使用 `--locked --offline -j2`；由中央一个 Cargo 顺序执行。

| 验证组 | 通过数 |
| --- | ---: |
| toolbar lib + slider/undo/presentation/既有回归 | 87 |
| system-decorator lib + core/audio/unload/visual/stairs integration | 249 |
| country-core ExtraInfo/tracking/signifier/extra unit/focused request | 17 |
| snapshot_native_restoration / worker_reload / save_load_gap_state | 11 |

decorator 首次 lib 为 215 通过、1 失败：旧 fixture 用非法 ExtraInfo tag8。窄修 fixture 为 Stair niche0后，整个 lib 216/216 通过，业务断言未弱化。摄影树 fixture 只修 CRLF 比较。同步最新代码后，第一阶段记录的六个缺失 lib fixture 阻断已消除；本轮实际编译 country-core 2740 项 lib tests，只运行列出的过滤目标，**不是 2740 全量执行**。

宿主正常 `cargo check -p tiny-glade --lib --bins` 被本机旧安装路径缺失 `fmod.dll` 阻断。显式 FMOD_LIB_DIR 空目录的 metadata-only 检查随后也失败：system-critters/startup_sheep.rs 的 bare CR doc-comment（11 处诊断）。没有链接、原生音频、GPU 或整游戏通过结论。该类型检查在用户要求停止测试前已结束；收到“先提交别测了”后不再启动任何测试。

提交前只读审查发现旧 camera-only 和完整摄影按钮 consumer 重复消费同一点击。已删除旧 camera-only 注册，把完整按钮与 slider reset 安排在 restore_after_timelapse 后、finish 前。此宿主接线窄修在用户要求停止测试后完成，未重新测试；不能用此前 364 项通过替代其运行验证。

实际命令、返回码、失败与复验摘要见 `validation-results.json`；最终源码身份见 `source-manifest.json`。原码 probe 的 hooks 仅证明对应调用者边界，不能推导物理、渲染或完整游戏等价。第一阶段原报告保留为 `PHASE1-REPORT.md`。

## Git 同步与交接

远程 `git@github.com:yuluo-zy/games_test.git`，分支 main。先以本地来源 f0c3ee0736f88518d2a28d958258b0b4a7a0d3cd 对照，将已有改动合并至远程 a6ef0b5e6e5261851252db0483b3a5491397fab4。恢复缺失文件并保留手工架构；冲突、备份和收据在忽略的 tmp/games-test-pre-sync-20261020。Windows 无法表达的 146 个研究文件名仅 sparse 排除，未从 Git 删除。研究 probe 的 exe/pdb/rlib 不提交，保留源码与 native fixture。

下一轮优先恢复 toolbar 导航/隐藏/抢占，随后 decorator 命中入口、楼梯请求/缓存及 validator；各步独立验收后再安装通用宿主视觉链。完整性和原算法正确性优先，不能用空资源或省略业务分支换取“跑通”。
