# REVIEW — X4+X6（组 state：save_game 系统主体 + commander_update 余 case）终验

> 审查员：reviewer-state，2026-10-16。复核 restorer X4（save_game_system）+ X6（commander_update
> case 2/3/7/8/0xc）落地。
> 编译基线实测：`cargo check -p country-core --offline` **零错误**（6 警告，含他组在途）；全 crate 测试
> **1709/1**（唯一败 geometry::sphere，既知）；systems::save + meta_commander 测试 15 通过 0 败。

## 逐项复核

### X4 save_game_system（save_game.rs 0x140a3f470 可接核心）
| 项 | verdict | 说明 |
|---|---|---|
| ① 备份节拍器推进 | **OK** | `AutoSaveState.metronome.add_active_game_time(time.delta)`；阈值门（+0x148<60.0）在 metronome 累计块内（save_backup_metronome，reviewer 复核）。 |
| ② 触发备份 → log "Capturing a game snapshot" + `CaptureSnapshotRequested` 事件 | **OK** | `consider_backing_up().is_some()` → log + EventWriter<CaptureSnapshotRequested>。宿主构建快照 Arc 后经 AsyncSaveCmd::CaptureSnapshot 发送（接线留宿主，诚实）。 |
| ③ 同步保存 OpenFile/Flush 由宿主经 `auto.send_command` 响应 | OK | send_command 存在（313 行）；宿主事件响应完成发送。 |
| register_save_game_events + add_save_load_systems_impl | **OK** | mod.rs 41 行 register_save_game_events（含 CaptureSnapshotRequested）；52 行 add_save_load_systems_impl。 |
| 仍阻塞：20+ SystemParam 域资源收集 | GAP（登记） | InnerWalls/Roof/TimeOfDay/Terrain/DecoratorStorage/三色 ID/Prefab…由宿主收集；**不伪造节载荷**。 |

### X6 commander_update 余 case（meta_commander.rs）
| case | verdict | 说明 |
|---|---|---|
| 2/3（profiler_sinks HashMap 迭代，control-byte 位扫描；载荷非 0 → 重排 {tag3, countdown 0x12c}） | **OK（登记）** | Handled + 机器码语义注释；sink 载荷由宿主。 |
| 7（清零 + TimeUpdateStrategy 应用 FUN_1412e1400） | **OK（登记）** | Handled + 语义注释。 |
| 8（追加 0x30B 条目到 Vec） | **OK（登记）** | Handled + 语义注释。 |
| 0xc（GPU/内存报告 alloc 0x68B → Arc → hashbrown insert） | **OK（登记）** | Handled + 语义注释。 |

## 结论与 DISPATCH 建议
- **X4+X6 → 闭合（库内系统主体）**。save_game 系统核心（节拍器/备份触发/事件）+ commander_update
  全 case 语义登记，与 RE-SESSION-SWITCH §1.1 机器码一致；sink 载荷/域资源面由宿主，诚实登记不伪造。
- 残留登记（不阻塞）：save_game 域资源收集（20+ SystemParam 组装 GameSaveData）、commander_update
  case 2/3/7/8/0xc 的 sink 载荷/资源面接线。
- save/session → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked（域资源收集/组装 +
  全局调度 + 各域系统批量并入）。
