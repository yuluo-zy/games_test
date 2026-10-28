# B 历史目录的当前去向

原路径基准为 `skeleton/B/crates/country-core/src/`；当前路径基准为 `tiny-glade/crates/country-core/src/`。这里只比较目录与已有源码/证据，不把同名存在当作原行为已验收。主结论与证据边界见 [README](README.md)。

## resources/history：12 个原目录项

| B 原文件 | 当前实现去向 | 覆盖与缺口 |
| --- | --- | --- |
| `mod.rs` | `resources/history/mod.rs`；`recovered_building/history_transactions.rs`、`history_snapshots.rs` | History/Entry/EditType/Identity、录制、截断、游标、快照有实现。完整 typed MultiEdit 导入和待定快照 ID 生命周期仍不完整或需补证。 |
| `auto_clutter_edit.rs` | 同名模块及 `recovered_systems/clutter.rs` | 类型 re-export、展开及回放存在；完整生成/显示消费者另核。 |
| `color_edit.rs` | 同名模块 | target 分派和 Serde 存在；楼梯栏杆/支撑类型仍不透明，应用事件消费未接全。 |
| `decorator_edit.rs` | 同名模块及墙装饰 owner | 状态、diff、转换、录制和事件已有；地址 wire 键占位和部分反映射仍需补齐。 |
| `path_edit.rs` | 同名模块、RasterStroke | 序列化、From 入口存在；From 仍用默认值填补部分源字段，不能认定原转换完整。 |
| `replay_cmd_buffer.rs` | 同名模块 | 桶、命令、即时化和展开存在；Terrain 统一展开拒绝，部分 merge/计数/当前命令等原接口未见完整等价接线。 |
| `roof_edit.rs` | `tg-history::RoofCommand`；`recovered_building/building_history_transactions.rs` 及屋顶 replay | 本包四种投影；core 还有 tag 4 风格变更；其 JSON 标签名未确定。 |
| `stairs_history_edit.rs` | 同名模块及 stairs owner | 支撑边编辑、Serde 和回放存在；辅助差分链仍有限定。 |
| `terrain_edit.rs` | `systems/terrain/deferred_terrain_history.rs`、`replay_terrain.rs` | 有局部模型与回放；CurveU 渐进 revert/reapply 及 edit→命令转换未闭合。 |
| `tree_edit.rs` | 同名模块；tree-tool/forest | 数据和回放存在；完整插入命令载荷仍有缺口。 |
| `wall_compound_edit.rs` | `recovered_building/wall_history_edit.rs`、`wall_history_replay.rs` | 组构建和部分子编辑存在；11 种子载荷仍 unresolved，部分操作不可回放。 |
| `wall_history.rs` | `systems/wall/history_recording.rs` 及 recovered_building 的墙/建筑历史实现 | 新墙、墙高事件有对应，覆盖受子编辑载荷和消费者限制。 |

HistoryChainIndex/HistoryEditChain 的基础包装由 i32、Vec 和事务方法合并承接，未保留同名 wrapper 不单算业务缺失。`MaybePendingSnapshotId` 等状态模型尚缺完整证据；快照文件名生成和 GC stem 解析已在 save thread / snapshot_gc 存在，不能说整个 ID 域全缺。

## systems/history：15 个原目录项

| B 原文件 | 当前实现去向 | 覆盖与缺口 |
| --- | --- | --- |
| `mod.rs` | `systems/history.rs`，startup 调用 | 已有局部 CPU 注册和调度；原完整引擎顺序未验收。 |
| `history_process.rs` | 同模块的 cursor/control 处理与桶展开 | undo/redo 已有；第二流仅部分分支，原 producer/载荷尚未闭合。 |
| `history_recording.rs` | `systems/history/recording.rs` 及各域 producer | 有录制/提交正文；原 owner 计数与状态字仍有模型替代。 |
| `finish_replay.rs` | 同名模块 | 模式、工具栏及哨兵处理存在；完整域清理仍有限定。 |
| `replay_mode.rs` | 同名模块 | 长按、速度和模式转换存在；完整 HistoryCmd 载荷对应段仍未恢复。 |
| `replay_auto_clutter.rs` | 同名模块 | 有对应实现，不能沿用早期“无历史回放”的旧结论。 |
| `replay_garden.rs` | `replay_raster.rs` 的 Garden 域 | 合并实现，独立文件缺失不计功能丢失。 |
| `replay_path.rs` | `replay_raster.rs` 的 Path/Cobblestone1–3 域 | 合并实现。 |
| `replay_water.rs` | `replay_raster.rs` 的 Water 域 | 合并实现。 |
| `replay_raster.rs` | 同名模块 | 时长、切片、撤销重绘及多域提交已有；GPU/整调度未在此次验收。 |
| `replay_stairs.rs` | 同名模块与 stairs resource | 有回放，完整跨域链仍有限定。 |
| `replay_terrain.rs` | `systems/terrain/replay_terrain.rs` | 有 CPU 回放；`terrain_replay_from_edit` 返回 None，统一桶扩展未闭合。 |
| `replay_wall.rs` | 同名模块及 `recovered_building/wall_history_replay.rs` | 多条实际分支已有；未恢复操作返回 UnrestoredOperation。 |
| `replay_pop_bucket.rs` | `systems/history.rs::replay_pop_bucket` | 完整函数存在，但当前 CPU 调度使用简化 pop，aux 恢复/碰撞 flush 生产接线未见。 |
| `signifier_replay_active.rs` | 未找到对应实现/替代映射 | 原系统和相关类型未映射；不能将 Bevy 注册胶水算作恢复了指示器业务。 |

当前额外 `replay_color`、`replay_decorator` 等模块用于承接原其他位置/共享分派的业务，不因 B 此目录没有同名文件就判为额外或冗余。此表不恢复缺失函数、不升级原行为验收状态。
