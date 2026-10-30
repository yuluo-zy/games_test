# M02 地形焦点编辑：原版核验与恢复

本轮收口 `Mode5 / ui_terrain_focus_mode`，没有把整个地形模块标为完成。真实 OS 输入仍按用户要求暂缓；本轮通过实际 Bevy World 的合成输入验证生产系统，GPU 双高度图、宿主读回、保存重启由中央与对应 owner 联合验收。

## 归属与原件

恢复文件保持 B 骨架原路径 `crates/country-core/src/systems/terrain/ui_terrain_focus_mode.rs`，没有新增业务目录，也没有重建另一套 terrain。

- 原函数：`1409a4fb0`，B PDB `527:19776`，`code_size=13202`。
- 原源文件 SHA：`d716af5feb5a8fe951efd8bcf7a79927007b2fc541bf7bb20b7f9fd7d213edc8`。
- 发行 RNE SHA：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- [13 组真实 SystemParam](m02-terrain-focus-typed-params.json)、[完整原指令](m02-terrain-focus-full.asm)、[修复 CFG 后的主体](m02-terrain-focus-complete.c)。
- 四个工厂原件由中央独立核验：[C/PDB 边界](m02-root-focus-factories-original.json)、[含 EH 尾部的完整 ASM](m02-root-focus-factories-original.asm)。
- 最终源码及证据 SHA 见 [验证收据](m02-focus-final-validation.json)。

## 首个真实缺口

原实现使用 `TerrainFocusRaycast/Hover/Selection` 三个临时资源，全仓只有初始化与测试写入，没有生产者；实际选中 ID 被错误替换为 `HashMap::values().last()`。画面仅发 Raw 数字记录，尾部拖拽与历史逻辑没有恢复。原始说明还把这些记录叫成窗口焦点事件。

独立读取发现 shared Ghidra 将函数识别为 2837B，120KB C 越界包含 `1409adxxx` 的邻近墙体函数。旧说明中的 CircularF32、相机朝向和 RequestAppModeChange 都来自该错误 CFG。此次先在独立 `m02-focus-20261027` worker 修正函数体与三处四路跳表，再恢复业务，不按错误 C 或旧完成统计推断功能。

三跳表在 `1409a58be / 1409a63f7 / 1409a6e68`，表 `142abdf98 / fa8 / fb8` 的实际目标由 RNE 字节解析。主体反编译不再串入相邻墙体函数；PDB 范围内独立 EH funclet 保持独立，没有人为删除真实业务分支。

## 生产调用与状态

实际宿主 `run_game::add_original_plugins → TerrainSystems::add_terrain_systems` 已挂本函数。原 13 组参数恢复为：模式、唯一 `TerrainEditingData`、真实 `UiState`、真实指示符流、Local 缓存、相机光标射线、`PendingTerrainHistory`、实际装饰归档器、点集命令 writer、边界/反馈/高亮组合、动作状态、参数命令 writer，以及窗口/屏幕光标/相机/物理世界/hoverID/地形射线组合。

指示符流使用安全 `ResMut` 接口。没有额外镜像、默认 History 或替代射线。Mode5 与原版相同先标记 terrain changed tick；不存在该 ID 时进入真实 hover 比较，不回退到另一条笔画。

Local 持有原点集、原首点、两个拖拽偏移、XZ 方向和旧高度/坡度。原初始化 wrapper `141752c50` 的内联 Default 与后续字段复制已核：[原 Default 证据](m02-focus-cache-default-native.json)、[初始化 ASM](m02-focus-cache-initialize.asm)。None 不活跃载荷在安全 Rust 中初始化，不宣称与原未定义 padding 逐字节相等。

## 三个完整操作

### 坡度

锚点为最靠近笔画 XZ 包围盒中心的实际笔画点，严格保留首次平局、NaN 的原 min/max 语义；不是包围盒中心。实际高度用于射线平面，轮廓半径为 `falloff*7+(1-falloff)*2.5`，没有错误地夹紧高度后当成 falloff。

原球云采样与拾取半径、活动线宽 5/普通线宽 2、方向捕获、Started 偏移、拖拽投影、0..1 夹紧、参数命令及结束历史全部恢复。max/min 的 NaN 与有符号零操作数规则保持原行为。

### 高度

原盒体 0.75×1×0.5、垂直位置、比例屏幕坐标和真实相机矩阵均复用现有原算法。Started 投影距离相对当前笔画高度捕获；Dragging 距离相对地面计算后减原偏移，再限制到 0..20。没有改成简单屏幕 Y 增量。开始时允许真实装饰剔除，结束写旧/新高度与坡度。

### 平移

原盒体 1.5×0.5×1.5。Started 只捕获一次点集与首点；后续基于原点集计算新点，不逐帧累积漂移。Y 也执行原 `+0.0`，保留符号零语义。原边界测试是当前射线平面 XZ、半径 2 的圆，失败依次发两个真实边界事件，不提交点集。结束历史使用实际已接受点集的首点差，越界尝试不会混入历史。

没有原捕获时按原 unwrap 失败；空点集按原索引失败；非空 pending 按原 replace 后断言失败，没有吞错或默认补点。

## 真实画面与生命周期

四类 closure 均为真实 SignifierFactory：程序化圆线、坡度箭头、高度箭头、两层移动网格。程序化圆线写 canonical `DrawTopology::Alt(width)`，进入现有提取/绘制组件链。箭头保持完整四元数、缩放与平移，不简化成 yaw。

- 原资源拼写 `brush_arrow_flat_single_foward` 的拼写错误也必须保留。
- 高度网格为 `brush_arrow_flat_single_up`；移动网格为 `move_shape/move_shape_outline`。
- 工厂动画时长为 0，不能沿用构建器默认 0.2。
- 原坡度箭头局部偏移向量实读为 `(0,0,0.2)`；保留原四元数多项式、求和次序及世界平移。
- Widget 与 Factory 名称保留 `TerrainStrokeId(n)` 的原 Debug；没有以现代 `StrokeId` 或裸数字改变 WidgetId 哈希。
- 临时圆线网格随实际实体析构进入原 TransientMesh 队列，并经 drain 释放，测试没有只检查类型存在。

## 验收

[最终定向日志](m02-focus-final-tests.log) 7/7 通过；[native-physics cargo check](m02-focus-final-check.log) 通过。

[原数值 oracle](m02-focus-numeric-native.json) 与[可重放脚本](m02-focus-numeric-native.py) 覆盖 704 输入、1792 个 f32 输出 word，全部逐 bit 相等，没有放宽 epsilon：

1. 192 组原 `51de..5304` 连续指令：非规则多点、平局、正负零的锚点与相机射线平面投影。
2. 256 组原 `59f4..5a3d`：坡度投影与夹紧，包括退化方向。该块输入的两个端点在夹具准备处按 f32 构造，明确不把它说成整个 UI 原函数执行。
3. 256 组原 `5c8b..5d36`：任意非单位四元数的箭头完整偏移与平移。没有归一化输入来隐藏复杂分支。

实际 World 测试验证：明确选择 ID 与最后插入 ID 不同；三个真实 widget；原 factory 实体/组件；坡度与高度 Started→Dragging→Finished；真实编辑消费者、音效与重算事件；平移跨帧、边界拒绝和实际接受值历史；Pending→实际 History recorder；历史 undo/redo 叶子产出命令并由同一实际消费者修改权威点集；空点、缺捕获、pending 冲突失败顺序；hover 高亮被活动 widget 抢占；实体销毁与临时网格回收。

这不是 OS 输入验收，也不是整个 HistoryCmd 宿主/磁盘保存/GPU 全链验收。

完整 terrain 首次回归实际为 **81/82**，见[保留失败日志](m02-terrain-module-first-regression.log)。唯一失败为旧 `release_without_a_last_stroke_still_clears_the_plane` 夹具以 `next_index=0` 调用新版恢复的原 get_last_mut counter 断言。已报中央及 draw/counter owner，不能为了绿灯删除该原失败语义；本文件 7 项保持通过。中央负责最终统一回归结果。

## 下一轮复用经验

1. 先比对 PDB code_size、原机器码与间接跳表，再读大反编译；输出被截断、越界或别的业务名混入时停止业务推断。
2. `grow_one` 的折叠泛型别名不能确定事件身份。此次 0x10 stride 的旧 WindowFocus 命名，实际是 typed SystemParam 中的 AddTerrainHighlightCmd。
3. 同时核 writer、原 FunctionSystem 与 consumer；没有生产写入方的临时资源不能作为已接线证据。
4. 场景锚点、滑块几何、原浮点求和次序和字符串哈希均属于业务合同；常量的坐标轴必须读字节，不能凭画面猜。
5. 原算法数值对照、实际 World、真实 factory 生命周期、GPU/OS 验收分开记录。一个通过不代替其它层。
6. 共享文件先发合同，独占原归属；本轮中央负责 host/mod 去假资源，Move owner 负责真实 u64 counter，避免各自重写数据体系。
