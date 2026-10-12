# B 屋顶 lifecycle 待恢复入口

本批只读导出复用既有 Ghidra 数据库；[report.json](report.json)绑定数据库来源 patched EXE。原指令执行需继续使用 tracking/baselines.json 登记的 B RNE。反编译 void() / 错误的泛型函数名仍可能是 Ghidra类型占位，不能直接当作完整ABI或源码。

| 原入口 | 地址 | 已观察的游戏逻辑 | 尚待实现 |
|---|---|---|---|
| create_roof | 0x1421aaea0 | 读取PublicWalls及缓存高度；约束wall shape tag<2；Roof::new/ty/is_gable后组装含RoofTileSimulation/TriggerVolume/Animation等bundle；发送屋顶/音频事件 | 权威wall schema、presence默认值、各初始化器、实体注册与任务时序 |
| mark_roof_for_deletion | 0x14062f3d0 | 收集删除请求，遍历相关roof查询；构造删除动画/通知的后段 | 全部过滤条件、animation schema和移除时机 |
| delete_roofs | 0x14062f8c0 | 已导出完整观察入口 | ECS移除、公开wall关联与资源生命周期 |
| update_roof_spatial | 0x1421a7710 | 从wall缓存更新Roof+0x4c高度及+0x54半木构标志；wall shape+0x258..0x274映射Roof+8..0x24；ordered float相等决定是否发送tag2重建通知；Roof::ty为false时更新附属对象pivot y加bits0xbf30a3d7，为true时写零 | 查询成功/跳过路径、shape权威字段、附属对象布局及完整原指令差分 |

以上是调查证据，尚不计为Rust实现或系统验收。回放生产者已实现的创建/删除事件也不等于这些消费者已完成。下一步应按原类型与调用点恢复wall→roof关联，避免用猜测的场景数据补齐未知值。

2026-10-11：创建初始化与已解析空间更新现已有Rust实现及原指令验证，详见 [建筑增量](../../building-state/findings.md)。上表剩余消费者仍未计为完成。
