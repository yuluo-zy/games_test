# 证据链缺口调查与重锚方案(2026-10-15,coordinator)

> 触发:reviewer-state 发现 contracts.json 引用 `evidence/core-recovery-20261015/<dom>/REPORT.md`
> 共 10 份在 games_test 全树不存在;recovery/README.md 另有 4 份(gui/walls/roofs-stairs/raster)。
> 本文件为调查结论 + 可执行重锚方案。追踪条目已加入 DISPATCH.md「证据链缺口」。

## ① 调查结论:报告从未存在(非丢失)

- `find games_test -type d -name core-recovery-20261015` → 无目录;任何 `REPORT.md` 均不在该路径。
- git(games_test 仓库,HEAD 6845e43f7):`git ls-files "*/evidence/core-recovery-20261015/*"` 空;
  `git log --all -- <路径>` 无任何提交;全库 `git grep -l core-recovery-20261015` 仅命中
  README.md(HEAD 提交本身)——即引用已提交,目录从未提交/创建。
- 结论:**这批 REPORT.md 是规划中的落盘位置,实际从未写入**。各模块"限定核验"结论已内联在
  contracts.json 的 `verified_scope`/`remaining` 字段(这是规范内容);详细逐函数核验此前落盘在
  `skeleton/task-index/work/TG-*/core-{gui,walls}-20261014/REPORT.md`,本轮落在
  `recovery/agent-team/reports/<组>/<模块>/`(REVIEW/RESTORE/RE-ANALYSIS.md,landing 中)。

## 受影响引用清单

**contracts.json 10 条 → 19 个合同条目**(路径相对 reverse-engineering/tiny-glade 根):

| 缺失 REPORT | 引用它的合同条目 |
|---|---|
| `core-recovery-20261015/critters/REPORT.md` | resources/critters/sheep.rs、resources/critters/sheep/control.rs |
| `core-recovery-20261015/garden/REPORT.md` | systems/garden/draw_garden.rs、systems/garden/record_garden_history.rs |
| `core-recovery-20261015/glade-settings/REPORT.md` | resources/glade_settings.rs |
| `core-recovery-20261015/input/REPORT.md` | systems/backend/input/calculate_input_delta.rs、systems/backend/input/translate_input_to_cursor_moved.rs |
| `core-recovery-20261015/ivy/REPORT.md` | systems/ivy/ivy_leaf_spawner.rs |
| `core-recovery-20261015/music/REPORT.md` | systems/audio/music/music_player.rs |
| `core-recovery-20261015/save/REPORT.md` | systems/save/shadow_file.rs、save_thread.rs、save_backup_metronome.rs、snapshot_gc.rs |
| `core-recovery-20261015/session/REPORT.md` | resources/session.rs、resources/image_formation.rs |
| `core-recovery-20261015/terrain/REPORT.md` | systems/terrain/edit_terrain_stroke.rs、delayed_terrain_changed_msg.rs |
| `core-recovery-20261015/ui-clutter/REPORT.md` | systems/ui_clutter/defered_autoclutter_history.rs、systems/history/replay_auto_clutter.rs |

**recovery/README.md 4 条**:`evidence/core-recovery-20261015/{gui,walls,roofs-stairs,raster}/REPORT.md`。

## 真实存在、可作重锚目标的证据

- **逐文件章节**(19 个受影响合同文件全部在位,逐一核对):
  `evidence/country-core-deep/chapters/<file>.rs.md`(例:`chapters/resources/critters/sheep.rs.md`)。
- 域级分析:`evidence/country-core-deep/{session-save,audio-misc,input,terrain-water,ui-input,
  collision-eco}/DIGEST.md`+`ALGORITHMS.md`+`HIGHLIGHTS.md`;round3/MODULE-DEEP-DIG.md。
- 本轮 worker 报告(实时):`recovery/agent-team/reports/<组>/<模块>/{RESTORE,REVIEW,RE-ANALYSIS}.md`
  ——save/music/session/sheep 的 REVIEW 已用 fresh 反编译重新锚到 DIGEST/round3,并**同诉缺失**。
- gui/walls 已存在真实 REPORT.md:`skeleton/task-index/work/TG-cfac1f498c427933d2/core-gui-20261014/
  REPORT.md` 与 `TG-9f411268a571c6bce1/core-walls-20261014/REPORT.md`(contracts.json 的
  gui.rs 条目已正确引用前者,可作样例)。
- roofs-stairs/raster 无 REPORT.md:最近真证据为 `country-core-deep/roofs-rt/ALGORITHMS.md`、
  `country-core-audit/R5_world_raster_terrain.md` 与 world_raster/roof 章节。

## 重锚方案

**方案 A(推荐):contracts.json evidence 改指真实文件,不重建目录。**
对每条缺失引用,在对应合同条目 `evidence` 数组追加/替换为:
```
evidence/country-core-deep/chapters/<file>.md
evidence/country-core-deep/round3/MODULE-DEEP-DIG.md
```
(如该条目另有真实域证据则保留,如 session-save/DIGEST.md 等;`verified_scope`/`remaining`
不动,仍是规范核验结论)。README 4 条改为:
- gui → skeleton/task-index/work/TG-cfac1f498c427933d2/core-gui-20261014/REPORT.md
- walls → skeleton/task-index/work/TG-9f411268a571c6bce1/core-walls-20261014/REPORT.md
- roofs-stairs → country-core-deep/roofs-rt/ALGORITHMS.md(+roof/stairs 章节)
- raster → country-core-audit/R5_world_raster_terrain.md(+world_raster 章节)

**方案 B(备选,仅当要保路径稳定):重建目录做"重锚收集器"。**在
`reverse-engineering/tiny-glade/evidence/core-recovery-20261015/<dom>/REPORT.md` 生成最小
REPORT,内容仅为"重锚声明 + 真实证据指针(章节/DIGEST/decomp)+ 该条目 verified_scope 摘录",
明确标注"非新增核验、仅收集既有证据",不写新结论。适用:README 的 roofs-stairs/raster
(无 REPORT.md)若希望路径不变。

**不建议**:直接编造含新核验结论的 REPORT.md——违反"证据须原生可复验"纪律。

## 执行步骤(待主会话批准)

1. 备份 contracts.json → 按上表替换 evidence 条目 → 重算 `contracts_sha256` 并更新
   inventory.json(README 更新清单流程:rustdoc 导出 + tools/audit_core_foundation.py 重生成;
   不重生成则需显式登记 hash 差异)。
2. 同步更新 README.md 4 处 evidence 引用。
3. 每模块 RESTORE/REVIEW/RE-ANALYSIS.md 落盘后,contracts 可再追加指向
   `recovery/agent-team/reports/<组>/<模块>/<role>.md` 作为"本轮实时核验"。
4. 更新 DISPATCH.md「证据链缺口」条目状态(见该节)。

## 附带发现(供本批使用)

- reviewer-state 已在 save REVIEW 纠错:DIGEST/ALGORITHMS 第 9 节键应为 `snapshots`(9B),
  "snapshot" 过期;consider_backing_up clamp 上界 8.0/步进 -1.0;new_from_session 计数器对
  每个入选条目解析数字后缀(非仅 history-auto)。这些 fresh 反编译纠错应回填 contracts/章节。
- session REVIEW 结论:已实现范围可 `review_passed`(限定),完整切换生命周期 blocked
  (NextSession/GameSaveData/MetaCommand)——与 DISPATCH 跨组裁决一致。
