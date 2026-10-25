# 2026-10-21 history / save 只读进度审计

基线：中央已同步 main f72938c81。仅读现有源码和记录；没有 Cargo、测试、原码执行或生产修改。以下百分比是“完整业务行为恢复”的工程估计区间，不是测试覆盖率、函数数比例或已验收完成率；置信度中等。

| 模块 | 保守估计 | 当前已实现 / 接线 | 未闭合的主要业务边界 |
|---|---:|---|---|
| History 账本 / 录制 / cursor / Undo-Redo / 分桶 / Pop | 80–90% | canonical HistoryTransactions、entry-owned decorator/stairs aux、持久 pending queue、方向/定时/单 Pop 与真实 owner 恢复；宿主 set14/35/40/41 | 全部用户编辑→多次 undo/redo→场景重建的整游戏验收；CPU 适配器 optional physics/archive 的成功不可充当完整副作用验收 |
| Wall 历史保存与回放 | 75–85% | typed 保存、17-way 身份纠错、native9 ChangeStyle、16 bool ChangeSkirt、10 plaster、12 supports、premade/freehand morph；真实 InnerWalls authority | 所有非有限/旧兼容分支、权威→构造→GPU 实例/删除完整闭合；旧报告的9/16错误已有独立修正，不能沿用 |
| Roof / FlatRoof 历史 | 65–75% | Create/Delete/Visual/Ridge/Rim 类型、原风格事件与 set40 replay、RoofSystems 实际安装 | 完整非空 roof consumer→tile/spire/support/stone-floor/稀疏上传的联合验收；部分原调度点仍明确 OPEN |
| Color 历史 | 75–85% | 实际 replay_color set40→18项颜色消费者、真实 typed SSBO owner；snapshot 三色表请求 | 完整菜单操作→历史→帧上传联合验收；独立 write GPU 的工程接线不等于所有原注册点已确定 |
| Terrain / 六种 Raster 历史 | 70–80% | Terrain typed edits、六 raster variants 与统一 replay_rasters、真实 dirty/staging request、set40 消费 | 真实地形 readback/渲染帧与多步 undo、所有 raster GPU owner 联合验收；部分 hover/focus/replay 原注册点尚未定位 |
| Tree 历史 | 65–75% | TreePlugin set40 replay、set27真实 insert/move/delete/convert 链、保留 storage ID/variant/name/旋转等 | prefab/forest/预置树真实非空场景和二次会话生命周期联合验收；prefab runtime 原注册点未定位，宿主现有工程 guard 必须另记 |
| Decorator / Stairs / AutoClutter 历史 | 60–75% | markers 与 entry-owned aux、Pop 恢复 canonical storage/graph；支持移除事件回放；AutoClutter Delete 保存与双向展开 | 原控制器→完整 diff→跨帧 commit→undo/redo→碰撞与最终 visual；stairs 音频仍有未解码载荷置零，不能称等价完成 |
| 保存文件 / 后台 worker / 备份 / snapshot GC | 80–90% | 真实文件改写、deferred JSON、snapshot 内容身份、worker flush/close/join、错误与 unsaved owner | 完整多会话/真实原存档长期回归，以及当前基线验证；不能由旧单例通过推断完整一致 |
| Snapshot 捕获 / 读档与域恢复 | 70–80% | 14节捕获/解析、真实 Roof/Tree/颜色请求、terrain invalidation、六 raster staging、AutoClutter replacement、Stairs restore | 非空全域→事件消费→实体几何→GPU 联合验收；Stairs JSON/checksum仍保留不一致 gap；gap != success，真实 load 会终止 |
| 旧版本迁移 / Load session 状态机 | 70–80% | settings-aware解析、拥有型 Parsed source move、兼容元数据4位、原 height迁移、两侧颜色映射、history identity、snapshot cursor偏移、backup gates、加载 marker/counter | 原始多版本非空 snapshot+完整 replay+二次会话实际验收；源码存在和静态原证不能替代本基线测试 |

综合：编辑→历史→回放约65–75%；保存→snapshot→worker→重载约70–80%。主要缺口已从“有没有回放 consumer”转为“真实消费者和完整场景副作用是否闭合”。保留原版 deprecated panic 是正确边界，不应当作为缺失 variant 或简化失败来计分。

## 当前源码位置

- EditType 全15外层变体：D:/game/tiny-glade/crates/country-core/src/resources/history/mod.rs:95；实际展开 dispatcher：.../resources/history/replay_cmd_buffer.rs:220。旧 Decorator(tag11)按原保留 unreachable；六 raster 不是缺失。
- History 实际宿主 set14/35/40/41 与单 Pop：D:/game/tiny-glade/crates/tiny-glade/src/run_game.rs:965；真实 Pop 对 aux→storage/stairs/archive/raycast：D:/game/tiny-glade/crates/country-core/src/systems/history.rs:343，ECS wrapper:399。
- 跨crate decorator/stairs consumer：D:/game/tiny-glade/crates/tiny-glade/src/history_runtime.rs:89；Prefab runtime OPEN:106；Roof若干原注册点OPEN:134；Terrain/Raster安装:164。
- 原 TreePlugin replay与实际命令链：D:/game/tiny-glade/crates/systems/tree-tool/src/lib.rs:121。Color实际插件: D:/game/tiny-glade/crates/systems/color/src/lib.rs:267；宿主安装 D:/game/tiny-glade/crates/tiny-glade/src/run_game.rs:101 和414。
- Stairs replay音频未解码字段：D:/game/tiny-glade/crates/country-core/src/systems/history/replay_stairs.rs:85–112。
- save_game set43：D:/game/tiny-glade/crates/tiny-glade/src/run_game.rs:304；load set12、completion set36：同文件696–712。
- Snapshot 捕获真实 Query/resource：D:/game/tiny-glade/crates/country-core/src/systems/save/save_game.rs:304、384；14节序：.../systems/save/snapshot.rs:162；域恢复:1638。
- snapshot report明确区分排队请求和场景完成：snapshot.rs:1630；Stairs checksum mismatch gap:1985；load要求report.gaps全空: D:/game/tiny-glade/crates/country-core/src/systems/save/load_game.rs:1082。
- snapshot Stairs格式：snapshot.rs:1176；原新版格式仍需保守复核，不能把无Stairs节的旧v17语料当现代格式覆盖。
- worker capture: D:/game/tiny-glade/crates/country-core/src/systems/save/save_thread.rs:384；shutdown:760。
- 空的旧 add_save_load_systems helper 位于 .../systems/save/mod.rs:23，但实际宿主已明确安装，不应凭旧stub注释把整条存档链判空。

## 现有验证证据边界

- progress-20261020/CENTRAL-REPORT.md：该历史检查点11项 snapshot_native_restoration / worker_reload / save_load_gap_state通过。只能证明该源码检查点及测试范围，不能替代f72938c81的新基线验收。
- evidence/round12/history/REPORT.md 与 HISTORY-WIRE-CORRECTION.md：明确后者覆盖前者旧9/16误归属；18原MultiEdit/13域fixture和原33history/2120条是待验证或外部语料数量，不可当实际通过数。
- evidence/round12/validation-v20-result.json：exit_code=1；validation-v20.log:701 显示 original_load_menu 编译失败，因此该次未执行成功的history wire也不能记通过。
- 最新 stairs-history-serde-readonly-triage-20261021/NATIVE-SERDE-RECHECK.json：820字节/187条指令静态覆盖，original_target_executed=false；不是原程序运行或Rust测试结果。
- history_runtime.rs内5项真实App回归存在，original_history_wire.rs四项分支回归存在；未在本次执行，不升级为“通过”。

审计判断不应继续使用“FlatRoof/tree-tool/stairs还未跨crate接线”这一旧缺口描述。现在主缺口是完整原调度合同和真实多域生命周期、实体/碰撞/上传/画面；当前研究原码/CPU leaf通过也不能折算整游戏完成。
