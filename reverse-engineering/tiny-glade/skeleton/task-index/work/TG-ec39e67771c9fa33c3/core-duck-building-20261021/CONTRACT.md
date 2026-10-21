# 鸭子建筑反应合同（待独立复核）

范围：TG-ec39e67771c9fa33c3，仅 ducks_react_to_being_inside_buildings.rs。基线 fa53d49a61dd7496cddc9af054c6fb1899fa1676；不修改生产源。B 原 RNE/PDB 身份由 native-evidence-v3/report.json 绑定；完整主函数 0x14231b140..0x14231b711，PDB procedure code_size=1489，331 条指令字节全部核验。只作为 B 还原，C 与 A 等价未验证。

## 真实输入与分支

- 两个直接 wrapper（RVA 0x1d9e857 与 0x1da7402）类型一致：EventReader<CmdCheckForSheepInsideBuilding>、Query<(&Duck,&Transform)>、Res<PublicWalls>、Query<&Roof>。原系统不是每帧鸭状态扫描；无 DuckState、无 GladeBorder 资源。
- 逐事件（原 EventInstance stride16、wall@+8），PublicWalls::get(event.wall).unwrap()；即使没有鸭子，缺 wall 仍 panic。已有 row 的合法性断言保留。
- +0x258 <=1 是 premade ShapeParameters tag（圆0、矩形1），不是墙组计数；非预制形状直接消费事件跳过。
- 原 +0x280 为现 authority.elevation()（源 +0x58 经 publication 复制的 canonical terrain_floor）。若 elevation>1.5 跳过；NaN 此门不过滤，但后续有序高度比较排除。
- get_wall_shape 只获取该事件建筑的真实28B形状。逐 Query 中所有 Duck（包括没有 DuckState 的鸭子），不要求错误的 DuckInsideBuildingAudio 组件。
- 逐鸭先执行 max_y（未算高度时 panic；没有鸭子不触发），再 roof_entity→真实Query<&Roof>拿取height；缺 association、过期entity、没有Roof组件全部取 +0.0。这是原版明确的零分支，不能把 max_y 也回退零。
- 随后 contains_point(x,z)，下界 elevation<=duck.y，上界 duck.y<=max_y+roof.height，两端闭合；NaN 任一比较失败。使用 canonical ShapeParameters::contains_point。0x140ad2950 的别名之一才是本调用的 ShapeParameters::contains_point；其RCX来自get_wall_shape，并非全局边界。
- 满足后触发 Duck 内首个音频实例（原Duck+0）start_if_not_playing；对应现有 Duck.audio.quack。保留每个事件重复触发调用，AudioInstance负责播放状态去重；无新音频、无stop/set_position。
- 保持原调用/失败顺序：事件→get/unwrap→shape/elevation门→每鸭max_y→roof→shape包含→高度→音频。

## 当前架构适配

原Query为只读Duck，因为原音频对象内部可执行副作用；当前手工重构AudioHandle需要&mut。候选使用 Query<(&mut Duck,&Transform)>，仅在播放分支通过 Mut::bypass_change_detection().audio.quack 修改后端句柄，不写Duck行为字段，不误触发Changed<Duck>。不修改现组件/音频API，不增加镜像组件；删除本源中无producer的DuckInsideBuildingAudio。

不遍历所有墙，不分配轮廓、不做射线法、不使用可选全局border。复杂度为O(本帧检查事件×活鸭)且无每鸭分配；保持原roof/max_y逐鸭求值以保持失败顺序。

## 生产链现存缺口与最窄协调

当前源码没有 CmdCheckForSheepInsideBuilding 的事件生产者。B finish_edit_rectangle 0x141464a60 wrapper（源TG-121b45a0b18d48b9f1）确实携带EventWriter<CmdCheckForSheepInsideBuilding>；当前 finish_edit_rectangle 在 decision.wall_notification 处 `let _ = wall` 丢弃通知。需上层按原完整事件顺序恢复，不能改用OnWallChanged通用转发（语义不同）。候选测试会使用真实Events队列，但这不代表实际编辑producer已恢复。

当前Duck::new_planned 仅创建Default DuckAudio（均Disabled），全生产源码无Ducks/Quack/TakeOff/Land实际instantiate。B Duck::new 0x140f85bb0（PDB code_size1920）已采原指令；三个事件实际存在，Quack设置名为pitch的参数（精确字符串待原数据摘取），首实例放入Duck+0。音频构造属于duck.rs/components.rs/core critters原source任务，需要另行授权/claim；消费候选使用现有Active sink测试，不以Disabled默认为完成。

系统已被CritterPlugin原0x24链注册，CmdCheck事件由SheepPlugin注册；core ivy亦注册共享事件。root若允许仅CritterPlugin单独宿主，须在其共享初始化中幂等init真实Events类型（不得另造事件）。

## 验收用例（同World）

1. InnerWalls真实new_wall→PublicWalls.consume→真实事件→本系统→同实体Duck.audio.quack Active记录器；无事件不播放，事件只消费一次，两个同wall事件各调用一次。
2. Duck缺少DuckState仍可触发；Changed<Duck>在仅后端音频调用后仍false。
3. 非事件墙不参与判断；圆/旋转矩形形状边界走canonical函数；freehand tag跳过；elevation=1.5通过、next_up拒绝，NaN/上下界有序比较。
4. Y上下界闭合，低于elevation与高于max_y+roof不播；真实roof、missing roof、stale Entity与无Roof组件都覆盖。
5. 缺wall即使query空也panic；高度未ready且有鸭子即使鸭在shape外也panic；无鸭子不读max_y。

Cargo/rustc与运行全部由root唯一owner执行；当前只产候选，不宣称原进程或C验证完成。
