# 方向、平屋顶外观与檐边消费者接入实际建筑链

## 技能沉淀

native-rust-reconstruction已创建在用户skills目录，由短SKILL.md和三个按需reference组成。三版依赖/原模块映射、原指令验证、Bevy业务链及token控制为通用流程，具体地址/字段只保留项目evidence。quick_validate通过，实际独立调查按该技能识别聚焦索引漏掉的system-color消费者、修正FlatStyle/rim混淆并验证B/C行为；不以格式通过代替业务判断验证。项目AGENTS链接该技能。

## B方向事件消费者

原system_roof::ui_edit_roof_ridge_dir::switch_roof_ridge_direction入口141a3a730，事件reader不是只处理UI按钮的生产者。PublicWalls::roof_entity不存在会unwrap失败，关联存在但Query失效同样失败；原已解析成功row范围141a3aa2b→141a3a8e1写Roof3c、标Changed、发geometry kind1（Entity/WallId）、gable WallId，以及sound kind7（pivot+Roof::height）。同值方向也执行所有写入和通知，重复消息不合并。

1024例原指令对照两种形状、方向0/1、等值输入、完整Roof88保留字节、tick和三个输出；private pivot/height原函数实际执行，标准event输出容量预分配。lookup/Query成功row由入口提供，原query存储和扩容不重写。三版源/静态方向写入见three-version-direction-comparison.json；A/C方向完整消费者未以B报告代验。

Runtime SwitchRoofDirection通过实际WallRoofMap及Query<&mut Roof>解析，没有Without<MarkedForDeletion>，此过滤范围由完整PDB FunctionSystem参数确认。失败转换为BuildingReport fault并停止该consumer剩余命令，CLI在本次update后返回错误；这是可诊断适配边界，非复制原panic/unwind或fork的异常调度。场景测试确认后续方向命令不继续执行。标记删除但实体仍存活时仍接受方向写入。

## FlatRoofStyle和RimStyle

原消费者属于system-color，是confirmed private business。B142299dd0 / C1428ac4f0后两个循环分别恢复，详见flat-findings.md、flat-version-comparison.json。完整外观12bytes写Roof40..4c；replay tag4实际上SetFlatRoofRimStyleCmd，仅写Roof3d，不能用一个FlatStyle事件混写。

外观语义比较忽略inactive payload/padding，但12字节照样复制并标tick；tag2比较指定active字段原bits。檐边也无条件写目标和tick。两个消费者仅状态变化才通知RecalculateWallHoles；不是RecalculateRoofCmd。Style sound字节严格1才发kind11；Rim history字节严格1才记录tag4/from/to/WallId，即使新旧rim相同也记录，from来自命令。Missing关联/无效query均skip，已标记/动画/斜屋顶没有额外过滤。

B/C各1536例全指令核心通过，另各4个missing/invalidquery guard实际执行确认skip。原History::add只截Mutation入参，未执行原事务。A完整相关符号与源范围未找到匹配，记录未匹配，不宣称A没有该能力或照搬B实现。ColorID首循环不在本轮范围。

## 真实World和回放联接

新roof_direction.rs / roof_customization.rs接Stage::Edit；后续gable、CPU瓦片、动画和Nani仍使用既有真实Bevy链。原离散replay输出经ReplayRoofEvent适配分发Create/Delete/Direction/FlatRimStyle；显式presence44作为创建输入，消息ID/padding不填造业务默认值。五个编辑World测试覆盖方向→gable/tiles/sound、重复/同值消息、missing失败、marked实体、Flat style/rim/union padding、Nani更新、history requests、方向/rim撤销选择、Delete回放→真正despawn→undo重建和Nani登记。

RoofRimHistoryRequests为待历史owner处理的adapter队列，持续保存请求，删除实体不丢失已经产生的mutation。RecalculateWallHoles为确认通知，实际开口几何消费者尚未恢复。该分发并非完整history cursor/commit/snapshot/undo事务恢复；UI造命令与原fork完整排程仍需独立取证。

CLI新增roof-edit-cycle.json八帧场景，原raw六帧/resolved五帧均重验。整workspace22测试，runtime18测试；registry113身份仍匹配B。功能链通过不改变ARM矩形瓦片case14单Y差1ULP的未关闭状态。

下一批优先：RecalculateWallHoles消费与墙/门窗状态联接，原History::add/commit/cursor事务及VisualEdit实际World消费；再向freeform缓存、墙砖列/支撑/开口推进。GPU与原fork排程、C Nani迁移仍不记为完成。
