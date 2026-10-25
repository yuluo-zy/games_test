# REVIEW — 库内可闭合缺口 闭合复验（组 state）

> 审查员：reviewer-state，2026-10-16。复核 restorer 对 completeness-audit 库内可闭合项的闭合。
> 编译基线实测：`cargo check -p country-core --offline` **零错误**（6 警告，含他组在途）；
> `systems::save` 测试 **11/11**；全 crate **1711/1**（唯一败 geometry::sphere，既知，非 state）。

## 逐项复核

| 项 | 修复 | verdict |
|---|---|---|
| SaveBackupList 计数器口径 | `trailing_counter` 剥已知前缀后取末尾连续数字，对**每个入选条目** `max(n+1)`（注释引 0x140ae8470 机器码 uStack_74=max(prev,parsed+1)，不限 history-auto-）——与 completeness-audit 库内可闭合 #1 一致。 | **OK** |
| extend_on_load tag=3 | 卡2 已用 OnLoadEntry::Batch（机器码 tag=3 + Box<0xa8B>）对齐；本轮确认无回归（save 11/11、全 crate 仅 geometry）。 | **OK（无回归）** |
| GameProfile 字段序 | 重排为 tutorial→tutorial_cooldowns→game_installation_id→key_bindings→new_glade_count（player_id skip）——与原 serialize 序（RE-DOMAIN-TYPES §4②）一致；无回归。 | **OK** |
| terrain 历史机制登记 | EditType::Terrain 原版无此名（visit_str 权威 15 变体），地形记录机制待 E 补挖，保留 tag=15 不伪造——维持。 | OK（登记） |
| history 40-op 并入交接 | WallHistoryOperationTag 17 名已登记；并入需 serde/MultiEdit 跳表 0x142aee31c 对齐 + wall replay 通配核对——交接后续 restorer，登记。 | OK（交接登记） |

## 结论与 DISPATCH 建议
- **库内可闭合缺口 → 全部闭合**（SaveBackupList 计数器 / extend_on_load tag / GameProfile 字段序），
  其余（terrain 机制 / 40-op 并入）为登记交接项，非库内缺失。
- **DISPATCH 建议**：save/session/music/history/startup/roofs/stairs/meta_commander/域类型 →
  **review_passed（限定已实现范围）**，库内完整性已由 completeness-audit 判定（无假实现）。
  完整 core 验收仍 blocked（外部依赖：域资源收集/组装、snapshot 域序列化器、commander_update sink 应用、
  GladeAudio 生产、27 startup 缺、replay_garden/path/water/terrain、视觉 RtWorld/mesh、camera_rig 控制器、
  全局调度）。
