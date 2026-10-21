# 矩形编辑完成 producer 合同（需中央协调后实施）

源任务 TG-121b45a0b18d48b9f1，owner core-duck-building-20261021；当前生产源仍未修改。B 原函数0x141464a60，PDB module1606 record106788，code_size2014，完整440条指令及所有内联源定位见native-evidence/report.json（ok，RNE/PDB全SHA绑定）。只证明B，不称A/C行为等价。

## 原参数与真实时序

两个静态直接 wrapper 的9参数一致：Res<InnerWalls>、ResMut<WipRectangle>、ResMut<History>、ResMut<DecoratorStorage>、ResMut<DecoratorArchivist>、EventReader<FinishEditRectangleCmd>、EventWriter<OnRectangleEdited>、EventWriter<NotifyHintSystemShapeEditWasPerformed>、EventWriter<CmdCheckForSheepInsideBuilding>。不是 OnWallChanged 转发。

1. 先读当前WipRectangle：非Idle立即克隆为OnRectangleEdited事件（80B载荷，含原EventId后实例stride88）。即使无finish事件、后续失败也已经写出。tag1的+12..16闲置word置0；tag2复制实际载荷；其余字段不能造默认覆盖。
2. finish.read().last()只判断有无，整队已读到末端。一帧多个finish仅提交一次。事件payload本身不取wall；wall从WipRectangle当前variant取。
3. 无事件直接返回，不改Wip/History/Archivist/Storage变更tick。若有事件但当前Idle(0)，panic；非法tag也不可默认成功。
4. 计算上一矩形与当前矩形的axis点积→acosf→abs→度，阈值原bits0x3c23d70a(0.01)。角度有序大于阈值才changed；否则比较尺寸差平方和sqrt。尺寸有序大于阈值才changed；NaN的JA/JBE保持与普通大于比较一致。只改center不算changed。不得新增normalize/clamp，否则错误state会被掩盖。
5. changed：仅此分支取InnerWalls对应墙unwrap，shape tag>1有panic_fmt，tag0圆也panic；tag1矩形的after读取真实墙源。before读取Wip里的previous rectangle，形成tag5 MorphPremadeShape复合history一组一项，立即History.add(EditType::Wall)。当前RecordWallHistory异步转发改变后续事件/错误顺序，不满足原合同；应由中央确认接真正的History资源（默认snapshot身份MaybePendingSnapshotId已在启动注册）。
6. History.add完成之后依次写NotifyHintSystemShapeEditWasPerformed(unit)、CmdCheckForSheepInsideBuilding{wall}。不得把OnRectangleEdited写到此处或仅changed分支。
7. unchanged：不查InnerWalls，不添加history/通知，立即调用真实LiveDecoratorArchivist::extract_and_restore_to_storage(&mut LiveDecoratorStorage)。真实三桶归档、collision map清理与tracked_clone_into已存在，不可继续空块。
8. 两个成功分支最后仅把Wip tag清0；其余payload保留。发生panic时保留原state与既有OnRectangleEdited副作用。

## 原80B记录布局（由finish读取和克隆指令逐项证明）

| 字段 | tag1 尺寸编辑 | tag2 朝向编辑 |
|---|---:|---:|
| wall u64 | +16 | +8 |
| previous.axis Vec2 | +32,+36 | +24,+28 |
| previous.center Vec2 | +40,+44 | +32,+36 |
| previous.size Vec2 | +48,+52 | +40,+44 |
| current.axis Vec2 | +56,+60 | +48,+52 |
| current.center Vec2 | +64,+68 | +56,+60 |
| current.size Vec2 | +72,+76 | +64,+68 |

非上述字段（如+4、tag1 +8/+24、tag2尾）仍需上游原始类型/创建指令辨认；finish只能保留原来真实payload，不能假造值。当前RectangleEditUiState([u8;80])可保留为手工重构owner，不要求复制原ABI；需要可靠typed读写previous/current rectangle。

## 当前跨源缺口与live owner

- dims writer：TG-74cbaa67617ca2c217，crates/country-core/src/systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs，2026-10-20T15:47 live claim NULL。当前每次fill(0)，wall写+16，moved.size写+48/+52（是previous.size位置），不保留previous.axis/center，不填current矩形，不发Finish。需要完整原UI算法恢复，不能只调偏移掩盖旧值保存/释放时序。
- orientation writer：TG-b0d4797db2453f6794，同目录ui_edit_rectangle_orientation.rs，live owner clutter-startup-20261020。当前fill(0)，wall写+8，signed angle写+24（原previous.axis.x），其它矩形字段皆0；每submitting调用直接发Finish。不可抢foreign。
- 原Wip模块：TG-b0756addcbd847f00a，同目录mod.rs，live claim NULL。当前只有子模块/角度helper，无原WipRectangle类型；现RectangleEditUiState位于本finish源。中央需裁定保留现owner并补typed view/真实事件类型，不能创建另一份权威状态。
- resources/events.rs生产已有，但未找到OnRectangleEdited。需中央最窄原类型补充并注册，不由本claim越界改。onboarding的NotifyHintSystemShapeEditWasPerformed类型已有；CmdCheck类型已有。
- finish_edit_rectangle当前在全生产源码无任何schedule引用。register_shape_resources已初始化RectangleEditUiState和FinishEditRectangleCmd；新事件需中央既有注册点加入。尚未请求把工具成功算闭包。
- edit_decision.rs已有B对应helper，可复用其previous/after与阈值算法，但当前调用绕过active_state_snapshot/empty_notification/restore且吞error，多个event for循环错误；helper默认把shape errors作为Result，宿主需明确panic并保留原副作用顺序。

## 验收闭包（实施后）

真实UI事件与原state producer→真实InnerWalls变更→真实PublicWalls发布→finish的History.add与三类事件→鸭建筑反应→原Duck持有Quack。保存/撤销共享实际History，不另建ledger。

同World测试至少覆盖：active无Finish每帧快照且不改state tick；多个Finish只提交一次；使用state墙而非event.wall；changed记录真实before/after且notify顺序；unchanged真实归档恢复并清三桶；缺wall/错误shape/Idle失败及active快照先发；先失败不reset；NaN/阈值/center-only比较；完整UI释放后consumer播放。所有运行由root唯一Cargo owner。

当前只完成原证、合同和缺口审计；未落部分write候选以避免伪装整链。
