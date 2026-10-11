# RecalculateWallHoles → 门窗开口状态 → 历史闭环

## 实际消费者和原模块

B完整PDB FunctionSystem确认 RecalculateWallHoles 由 country-core 的 publish_public_wall_state读取，不是 wall_roof_hashmap自身消费。原reader 140a8bbc0..140a8bccd对InnerWalls.contains检查，存在才发OnWallChanged；缺失skip，重复消息保持顺序。只桥接std lookup的512例原指令验证已落B-invalidation-verification.json。原publisher先处理gable reader再处理holes reader；两者都会到OnWallChanged。A未在本次完整符号范围找到对应消息，C已切换Message API且本次仅定位到writer，不能从符号负结果推断功能不存在。三版private源和方法映射详见three-version-reader-provenance及opening/storage各comparison。

本轮没有停在转发通知：OnWallChanged进入真实Bevy的门窗输入/派生状态重算，再贡献实际RawWallHole，进入跨帧存储和FinalizedWallHole几何。兼容resolved场景以已有合法PublicWalls投影作为authority presence接口；原始输入路径来自实际采集后的墙。原InnerWalls存储/哈希内部仍使用库边界。

## 门窗实际生产规则

opening_rules.rs恢复Cottage窗/阳台门及Gothic窗/阳台门四类完整已解析raw贡献。输入是DecoratorInfo::dst_as_wall位置、尺寸变体及资产碰撞metadata，不接受预计算hole数组。Cottage单矩形，Cottage窗的宽度只对circle宿主修正；Gothic各size由三个矩形组成，不合并。B3072例原record生产/count/字段比较通过。精确literal修复保留首次8例差异报告，不弱化bit比较。原WindowVisualEntityBuilder::build只spawn视觉组件，不是贡献生产者。范围与三版源/常量证据见opening-findings.md。

未恢复的输入前段包括资产库实际解码/查找、屋顶感知定位、宿主UV投影、骨骼形变与dormer；目前这些metadata/位置由显式resolved输入提供，未伪造资源或作者默认尺寸。OpeningAuthority/OpeningState为重建适配组件，真实Bevy管理实体/Changed/Commands；不宣称原visual Bundle或GPU实例已完成。

## 开口存储和实际finalizer调用

wall_holes.rs恢复raw20bytes和Finalized20bytes、std DefaultHasher调用schema、sum/xor指纹、WIP/Finalized/Prev跨帧转换、周期端点复制和按maxX稳定排序。B768原码例强比较两份fingerprint、几何、AL返回值和缓存参数；详情storage-findings.md。finalize不推进previous指纹；advance_frame才推进。原数据无merge/去重。

额外通过原caller140757800的两处实际调用确认：140757e14读PublicWall+28c的原始Curve2.length，140757e1c按shape tag!=2给闭合参数，140757e2e调用finalize并用AL判定几何更新。另一处140758cff..140758d1a同义。调用上下文见B-finalizer-callers/references.json。runtime按此传参，没有猜3D周长或仅以circle/rectangle枚举判断闭合。

实际World每帧advance_frame，保留上帧缓存并重收集所有存活门窗的cached raw贡献。同值会重算被通知的门窗状态，最终指纹相同则复用几何；metadata/宿主变化更新实际AABB；删除门窗不再贡献raw并移除实体；宿主消失清理derived host/holes和wall storage，仍存在的显式authority在宿主恢复后可重新解析。原整体装饰对象删除/视觉实体级联与其他hole生产者不在这批范围。

## 真实历史事务联接

新增history/transaction.rs恢复B start/try-start/add/finish、首个edit截redo、快照引用按cursor保留、nonce更新、cursor选择与原Rim命令展开/逆序重放。462组原指令序列通过，另三个原Token Drop guard场景证明析构仅遗忘mutation检查，不commit。完整证据见../building-history/findings.md。

BuildingHistory默认不猜原startup账本。必须显式导入observed ledger，或选择已确认的nuke_future_states清除后状态。本轮fixture显式选择后者。已配置后，Rim consumer产mutation → 原录制组add → finish真实写entry和cursor → InputReplayRecordingFinished通知 → instant undo/redo生成ReplayRoofEvent → Rim写入且record_history=0 → OnWallChanged → 门窗raw/缓存更新。未配置时保持原adapter pending队列；未start不自动录制。组内undo逆序，redo正序，撤销后新edit才截redo，空commit不毁redo。

目前闭环只涵盖Rim-only组及instant replay；其他MultiEdit owners/完整快照装载/原保存schema、VisualEdit world调度和timed replay仍未完成。instant=false明确报接口缺口，未猜等待时序。C Replay判别和A ledger布局不同，不能把B事务参数直接套三版。

## 整链验收

openings-history-cycle.json十二帧连续执行：创建两类门窗、生成4 raw/5 finalized（含跨端副本）、缓存复用、两条Rim组commit、undo、redo、undo后分叉、metadata变化改AABB、删门窗、删宿主。实际World两个测试另外覆盖跨两个wall的三条mutation逆序、圆墙缩宽、缺失Recalc跳过、宿主恢复。整workspace24测试（runtime20）及锁定构建通过，113 registry身份仍匹配B。

B-world-chain-verification.json保存逐帧实际结果和源码/binary/fixture哈希。原raw六帧、edit八帧和resolved五帧继续重验。没有完整游戏验收；原fork完整排程、墙砖三维裁切/墙mesh、门窗视觉资源/GPU、其他hole贡献及完整历史owner仍需要恢复。既有ARM瓦片单Y 1ULP失败保持未关闭。
