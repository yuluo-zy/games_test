# systems/save/load_game.rs

> 文件作用：读档主系统：LoadGameManager 状态机（LoadGameState::LoadGameData）→ 写 loading.tmp → NextSessionKind 解码（1=parse_saved_game_data 含 upgrade 备份、2=on_load 命令回放+内存快照）→ PostGameLoadCommand 回放（SwitchAppMode 忽略）→ 旧版颜色 ID 升级（{1→2,2→5,4→8,5→3,8→4}）→ extract_max_wall_id_from_history。

函数总计 14：业务 2 / 内联无独立符号 5 / 派生胶水 7

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::save::load_game::load_game`` | 0x14146d030 | 0 | IO/解析 | O：主系统（13+ 段 SystemParam）：状态机+loading.tmp+kind 解码+命令回放+升级+wall id 恢复（详见 DIGEST §3） | session-save\decomp\decomp-0x14146d030.txt  |
| ``country_core::systems::save::load_game::parse_saved_game_data`` | 0x141470d00 | 413 | IO/解析 | O：parse_saved_game_data（内联于 load_game）：Making save file backup for upgrade→Reading file data→Parsing JSON→Version→BackwardCompatibility→Camera→parsing camera→Time of day（串块 0x142c64450） | session-save\decomp\decomp-0x14146d030.txt  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::save::load_game::LoadGameManager::new`（行 78）— 构造
- `country_core::systems::save::load_game::apply_parsed_game_data`（行 744）— 待推断
- `country_core::systems::save::load_game::apply_backward_compatibility_overrides`（行 870）— 待推断
- `country_core::systems::save::load_game::upgrade_window_color`（行 880）— 待推断
- `country_core::systems::save::load_game::extract_max_wall_id_from_history`（行 899）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 7 项

## 实现方式（算法级，2026-10-15 深挖；全文见 session-save/ALGORITHMS.md）

- **load_game 0x14146d030**（13+ 段 SystemParam）：
  1. 门控 `*app_mode == 0xe`（LoadGameData 态）→ 否则 return；写 loading.tmp；
  2. **状态字节 +9**：0 → 按 +0x4c 计数（可负）发 16B 加载进度记录 {计数器, tag
     0x3010001/0x3010101} 入 Vec<16B>，置 2，return（多帧节拍）；2 → 主流程；其他
     → panic @0x142c641d0；
  3. **NextSessionKind 解码**：`decoded = kind ^ 0x8000000000000000; sel =
     min(decoded, 2)`；==1 → parse_saved_game_data（FUN_141470d00 内联：节序
     "Making save file backup for upgrade"→"Reading file data"→"Parsing JSON"→
     "Version"→"BackwardCompatibility"→"Camera"→"parsing camera"→"Time of day"
     （缺失 → "This save file had no 'Time of day' data"）；失败 → "Couldn't load
     save file from "）；==2 → on_load 命令回放 + 内存快照路径；
  4. **MetaCommand 回放**（on_load 4B 条目，fence 0x8000000000000003 终止）：变体 0
     → 写 manager 状态 + push 24B 事件；变体 1 → FUN_140b5a180 push MetaCommand
     （meta_commander.rs @0x142c5ae38）；变体 2+ → Arc LOCK dec drop；
  5. **PostGameLoadCommand 回放**（第二轮遍历）：变体 0（SwitchAppMode）→ log
     "Ignoring PostGameLoadCommand::SwitchAppMode"（@0x142c642e0）跳过；变体 1 →
     push 24B {tag, 载荷×2}；断言 "Command skipping must not be used together with
     snapshots" / "Can't skip "（@0x142c64378/0x142c643d8）→ FUN_1412e0b90 应用；
  6. **版本 → 全局开关**：`0x14362c308 = (v > 0x19)`；`0x14362c011 = (v > 0x1a)`
     （后者 = **history 域楼梯 checksum 校验开关**，跨域互证）；
  7. **旧版颜色 ID 升级**（"Detected old version of window color IDs. Upgrading..." /
     "Detected old version of arrow slits color IDs. Upgrading..."）：+0x52 ∈
     {0x38..0x3a} 的 0x58 步长记录，+0x30 字段按 {0,3,6,7→原值; 1→2; 2→5; 4→8;
     5→3; 8→4; 999→999; default→1} 改写；
  8. **extract_max_wall_id_from_history**：遍历 MultiEdit 取最大 wall id → 计数器
     资源 +0xc0；
  9. **init_game_state_from_snapshot**（FUN_1417a9c50）：拷贝 0x1b0B 数据应用 →
     清理（快照 Arc 列表 dec 循环）；事件 LoadingGameComplete/StartupReveal…；
     "Cloning game state for save rewrite took "（@0x142c64b48）。
