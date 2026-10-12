# 建筑系统历史、持久化、输入和排程缺口审计

日期：2026-10-11。只读规划审计；未修改 Rust、现有报告或进度台账。应用 native-rust-reconstruction 的依赖筛除、原模块映射和连续 Bevy 链验收规则。B 是已实现基线，C v1.16.0 是最终目标；PDB 源路径/校验码不是作者源码。

## 当前可确认的交付边界

- 实际 World 中已有 raw Wall→publication/cache→Roof create/association/edit/delete→开口 raw/finalized/cache，以及显式配置 Rim 账本→录制/提交→instant undo/redo→分叉截断→开口更新。24 个 workspace / 20 个 runtime 测试和 12 帧新场景验证这一限定路径。
- History start/add/finish/nuke_future/cursor、token policy 已有 B 462 序列原码验收。Undo 原行为是**正序展开全部 edit 到 ReplayCmd，再反转整个命令数组**；Rim 一个 edit 恰好一个 command，不能把该特例泛化成反转任意 raw edit。
- Roof VisualEdit process 1600 例、B JSON 118 个实际 VisualEdit×3进度×2方向=708 例联接已过，但属于 resolved/probe 验收。真实 runtime `ObservedReplayEvent` 只有 Create/Delete/Direction/FlatRimStyle；`route_replay` 无 VisualEdit，也没有实际 `RoofEditEvent` World consumer。
- `BuildingHistory.transactions` 默认 None，需明确导入 observed ledger 或调用已验证 future-clear reset；它没有恢复原作 startup History。原 snapshot `S` 泛型占位不等于游戏 snapshot decode/import。
- `instant=false` 在 World 中明确 fault；原码历史验收该 flag 的选择/传递，不证明 timed scheduler。辅助 owner 三个 counts 只记录存在性，非零 cursor replay 直接返回 `AuxiliaryReplayOwnerRequired`。
- 现 `BuildingStage` 的 19 阶段 `.chain()` 是重建工程显式依赖顺序。`original_scheduler_verified=false`，尚无原 fork registration/run_if/system-set graph 全证据。

## 缺口矩阵

| 模块 / 原拥有者 | 当前状态 | 必须恢复的游戏内容 | 不计作核心恢复的依赖 |
|---|---|---|---|
| `country-core/resources/history/wall_compound_edit`、`systems/history/replay_wall` | 实际墙输入/采集已通，Wall compound history 未通 | Create/Delete、形状 morph、height/elevation/style/arch/闭合变化的分组顺序、前后状态、派生 roof/decorator/stair side-effect 归组 | Vec/Arc、ECS query、曲线公开依赖 |
| `resources/history/decorator_edit`、`walls/decorator_storage` | 观察 schema 已知，四类开口使用 resolved placement/collision | 稀疏 before/after diff、owner Wall/Terrain/Stairs 与 dst Wall/Roof/Terrain/Stair 两层关系、host变更/删除/restoration、事件顺序 | HashMap、Serde、Arc |
| `systems/history/replay_stairs`、`stairs_state` | stairs edge/hash 样本，暂无业务应用 | 楼梯 before/after 边图、支持件删除/重建、hash policy、门窗派生楼梯是否合入同一事务 | 图容器、哈希库、序列化胶水 |
| `system-roof/ui_edit_roof*` 和 `resources/history` | Rim 录制事务通，其他离散事件可直接分发 | Roof create/delete/direction/full VisualEdit 的真实记录生产者；鼠标开始/持续/结束/cancel 到事务；无递归记录策略 | Bevy input/event/query |
| `systems/history/replay_mode/replay_pop_bucket/finish_replay`、`replay_cmd_buffer` | timed flag 核心已知，World 不支持 | Bucket 组织、顺序/并发、elapsed/duration、pop条件、flush、interrupt/instant化、完成阶段恢复和未完成命令保留 | 通用队列、调度器、计时底层 |
| `systems/save/snapshot`、`resources/history` | 样本身份清单，snapshot payload/selection 未通 | capture/closest selection/validation/import，cursor/history_start/undo_anchor，缺失引用 fallback，ID和跨对象重建 | rkyv/ron 原fork、压缩/Serde |
| `systems/save/load_game/save_game/save_thread/snapshot_gc` | 原 source paths 和函数候选 | 游戏 schema 迁移、原 section 格式、存档 identity、incremental history追加、dirty/nonce、autosave/backup/GC消费者 | 文件IO、线程、通用压缩/JSON/RON解析 |
| 建筑输入/拾取/UI工具 | CLI合成事件是adapter | selection、工具mode、pointer capture、drag边界、snapping、参数限制、取消/删改策略和history start/finish ownership | winit/Bevy input、GUI库、dolly |
| 游戏注册与排程 | B组件创建tuple局部已确认，19阶段人工chain | 原资源初始化、事件reader/writer、run_if/AppMode guard、changed filters、ApplyDeferred、dirty propagation、异步重建/取消 | Bevy scheduler/archetype/task内部 |
| C迁移和最终验收 | B主链；C只有部分leaf差分 | 每个模块真实C ABI/enum/API/schema差异与World整链验证；最终建筑 save-load-edit-undo-delete 生命周期 | 更换依赖身份不能代替业务验收 |

## 三版本约束和语料新核查

`evidence/multi-version/*-sources.json` 显示 history_process、history_recording、finish_replay、replay_mode、replay_pop_bucket、replay_wall、snapshot、load_game 源 checksum 三版不同。它们都来自私有 `crates/country-core`，保留为业务目标，不能因为复用容器/相似符号就筛掉。B/C replay_raster、replay_auto_clutter、snapshot_gc 等有相同 private source checksum，也不自动变成库。

History A MultiEdit32/cursor+50；B/C MultiEdit40/cursor+68；A两项 empty checks，B/C四项（edit+3 aux）。C Replay byte tag9，B dword tag13。B Bevy0.16.0、C0.17.3及 Message API 差异需按三版锁文件/PDB适配；原fork出处未核实，不能宣称 registry 即无补丁。

语料 `corpus-index.json`：98 history / 101 snapshot 文件，内容去重50 / 52；缺失引用**A18/B22/C22=62**。B22项只有2项有其他版本同ID候选，均 `replacement_verified=false`。不能补空 snapshot、用相同ID直接替换或把缺失解码成默认世界。已有旧 findings 的22数字只指B。

## 五个攻关包（按收益与依赖）

### H1：完整 Roof 记录→VisualEdit 实际 World→timed replay

入口：B `RoofReplayCmd::process` 1421b48e0；`ui_edit_roof` 141a18dd0 / `do_ui_edit_roof` 141a19a90；direction记录141a3ac20；history_finish_recording1409dd790；replay_mode140a7c550；replay_pop_bucket140b38250；finish_replay1408b36a0。C候选分别 ui_edit_roof141f34460、direction记录141f3a810、history_finish_recording140ba0880、replay_mode140b96ef0、replay_pop_bucket140b29920、finish_replay140c20eb0。

输入/输出：真实 Roof编辑输入和既有 ledger→MultiEdit记录及 ReplayBuffer→真实Query写Roof、gable/hole/CPU/Nani变化→bucket完成和AppMode恢复。前置：既有 edit_rules/replay numerical core；补真正事件consumer与原scheduler边。先做instant完整Roof owners，再计时。

验收：录制create→多次VisualEdit→direction→delete一组；正序redo/整个展开命令数组逆序undo；跨若干dt，dt0/duration0/完成后重复帧；marked/missing owner行为；interrupt与instant化；无history递归；删后undo重建和派生几何一致。风险：mode low-bit与mode==1是不同条件，0/0保留NaN分支，record previous来源不能猜当前值。

### H2：Wall compound + Decorator/stairs auxiliary owners 的完整事务

入口：B CompoundEditBuilder new_edit_sequence140a82b20、next140a82b40、build140a82d50；wall_deleted140a876a0；history_record_adjust_wall_height140ac79e0；replay_wall140951b80 / WallReplayEditCmd::process140abfb30；decorator diff conversion140b3fab0、send_state_diff_events1417bdae0、DecoratorStorage::restore_snapshot140a20d70；StairsState::restore_snapshot1408ff590、replay_stairs140af75b0。C对应build140b8f6f0、height history140bebcc0、replay_wall1419c0d80 / process1419e0090、diff conversion140c7a3d0、send diff140a71200、decorator restore140a47f50、stairs restore140a56090 / replay140b73700。

输入/输出：一项权威墙变化→compound操作组+真实decorator差异+stairs边副记录→统一commit→重新执行整组对象/派生创建删除。前置：墙形态/实体所有权、原门窗宿主解析和stairs基础消费者（需与几何agent协作）；H1通用command expansion合同。三项未知aux不能凭JSON键名一一强配，需要每项writer、entry descriptor、replay reader证据。

验收：同一组中墙缩放/height修改→roof调整→开口迁移→楼梯边变化；compound一edit展开多cmd，undo逆转展开后数组而非raw edits；host删除与restoration、不存在对象、重复ID/墙owner区分；撤销后新编辑截所有future关联snapshot；空组保持redo。风险：本批恢复的是建筑域owner，不必顺带实现树/水等全部游戏领域；Color Wall/Roof/Decorator/Stair共用建筑domain仍应纳入。

### H3：真实快照选择→装载→建筑复原→保存闭环

入口：B parse_saved_game_data141470d00、load_game14146d030；validate_and_load_closest_snapshot_data1417a6e50、init_game_state_from_snapshot1417a9c50；capture_game_snapshot1417a1d00 / capture_save_data1417a0430；current_snapshot140a84650、closest_resolved_snapshot140a84a50、set_current_snapshot140a84760；save_game140a3f470、add_history_edit_to_save_file140a439b0、save_thread141720680。C parse140b21aa0、load140b24fa0、validate141bfd120、init141bf9a00、capture141bf40e0 / capture_save141bfb6a0、save140acdab0、addhistory140acd750、thread141b6df50。

输入/输出：真实发行history/snapshot→经过游戏schema与迁移的权威建筑状态+ledger→继续编辑并save→重新load相同业务状态；明确缺失引用策略。前置：H2对象owner schema/ID与主要消费者；文件容器可与H1/H2并行调查。复用原rkyv/ron/Serde/压缩，恢复字段、调用参数、选择和版本政策。

验收：从可用snapshot恢复到cursor，只回放需要的区间；尾部redo保留；老WallNewSystem→新Wall实际迁移；save→load ID/host关系/几何/hash/历史cursor一致；损坏/缺失snapshot保持原fallback或error；B22未知项单列。风险：同ID不是兼容性证据、nonce随机并非time、unknown扩展字段必须保留、rkyv ABI或原fork不可替换成猜测serializer。

### H4：真实建筑工具输入与history ownership

入口：B UI height mode140ac6d60及get_closest_point_u_to_cursor140ac7400；roof UI入口见H1；PDB中country-core `systems/ui_edit_freehand_walls` 140ac98e0、resources/mouse_capture140b17830；需要继续索引premade create/delete/morph及decorator编辑工具writer，不把当前CLI当作原UI。

输入/输出：拾取/选择、mouse capture、pointer begin/move/end/cancel、toolmode→权威墙/屋顶/门窗命令→正确事务边界+反馈。前置：H1/H2 state/replay contracts，可先恢复生成业务命令的CPU规则，交互窗口/UI壳按用户当前源码优先顺序后置。

验收：一次drag一个commit、重复move不重复结束、cancel/切工具/丢失host、focus编辑、等值input的tick/sound/history分离；同一command可由UI或replay产生但record flag政策正确。风险：slider3d/GUI泛型包含业务类型并不使其库实现归属游戏；工具可见名称不能代替真实payload类型。

### H5：原业务排程图 + C迁移 + 系统完成门槛

入口：三版country-core `systems/history/mod.rs`、`systems/save/mod.rs`、system-roof/system-decorator/system-wall-constructor plugin/lib source paths；从真实registration调用和FunctionSystem参数取descriptor、resource/query/event edges，未定位地址不能编造。B现重建入口 `crates/tg-building-runtime/src/lib.rs` BuildingPlugin只用于对比。

输入/输出：原registration和阶段guard→有证据的业务图（含组件/required graph、resources、events、run_if、deferred边、dirty/task cancellation）→以依赖API注册，不重做scheduler。前置：可先并行登记，执行迁移和最终验收需H1-H4及墙/门窗/楼梯几何基本完整。C差分每条模块重新验收，A作追溯而非全量兼容目标。

验收门槛：create墙/roof/门窗/stairs→unchanged帧→形状/高度/terrain变更→开口重算与墙体裁切→commit→timed undo/redo→host删除/渐隐/flush→undo重建→save/reload→干净帧，无悬挂ID/旧cache/task/backend意图；真实样本有逐项结果，覆盖不支持命令不能静默skip。GPU后置但要最终验证allocation/upload/activehandle的消费者，CPU intention不算backend成功。原ARM瓦片case14 1ULP失败保持，不能因24测试通过改写为全数值一致。

## 最小完整建筑 History 扩展包：owner顺序与输入/输出合同

下表的“推进顺序”是解锁依赖的工程计划，**不是尚未恢复的原MultiEdit运行顺序**。真实组内次序以writer、compound builder和generate_replay_cmds_from_edit分支为准，不能按表顺序强排。首次接受真实非Rim数据前必须实现typed owner dispatch；unknown owner/aux返回明确不支持，不能落一个“有效但空”的history group。

| 推进顺序 / owner | 原producer目标 | 原consumer目标 | 最小闭环及前置 |
|---|---|---|---|
| 1 Roof | `ui_edit_roof`/focused/ridge_length记录VisualEdit的before/after；`record_switch_roof_ridge_history` B141a3ac20；create/delete路径的History::add调用点尚需定位，不借用replay事件当producer | RoofEdit展开→原 `RoofReplayCmd::process` B1421b48e0→实际edit_roof/create/delete/direction消费者 | 已有core与实体查询；增加真实 RoofEdit World事件后先恢复Create/Delete/Direction/VisualEdit完整录制和instant，再接timed；FullFlatStyle的history owner可能为Color，禁止与Rim混用 |
| 2 Wall | `CompoundEditBuilder::{new_edit_sequence,next,build}` B140a82b20/140a82b40/140a82d50；`wall_deleted`140a876a0；`history_record_adjust_wall_height`140ac79e0；create/morph/style/elevation/arch生产者需按各UI/权威mutation的add调用链定位 | `generate_replay_cmds_from_edit`多个实例按真实tag分支；`replay_wall`140951b80、`WallReplayEditCmd::process`140abfb30→InnerWalls权威创建/删改→publish/roof关系 | 支持Wall/Create/Delete/Edit各独立分支，compound一次展开多命令；恢复before/after的curve、shape、height/elevation/style/arch字段；Delete undo恢复原WallId及roof/门窗owner关系，不重新分配domain ID |
| 3 Decorator | Place/Change/Remove工具到DecoratorStorage差异收集；B diff conversion140b3fab0；原author mutation进入token、owner target及原HistoryV5分支的writer仍需定位 | `send_state_diff_events`1417bdae0、`DecoratorStorage::restore_snapshot`140a20d70与原diff apply调用点→各family/dst实际entity/holes消费者 | JSON中的DecoratorV5空命令不是空变更；实质diff在同entry.decorators；Place/Move(Remapped Change)/Remove必须落before/after和目标；owner Wall与dst Roof不得合并；删除/恢复保持原decorator domain ID与rank/seed/anchor/UV |
| 4 Color | system-color中Wall/Roof/Halftimber/Decorator/Stair颜色命令记录History::add的caller；当前FlatStyle/Rim消费者可作为入口但ColorID首循环仍未恢复 | 建筑Color Replay owner→真实system-color各命令reader；flat/rim仍分清tag，ColorID与亮度需另合同 | Color和移动/创建可共组；同值赋值tick、sound、semantic通知、history记录需分别验证；染色撤销不重新生成或丢失ID |
| 5 Stairs / composite auxiliaries | stairs graph及support变化捕获before/after/hash；门窗/墙删除导致图改变是否写到同一MultiEdit需恢复真实writer | `StairsState::restore_snapshot`1408ff590、`replay_stairs`140af75b0→支持件删/建及DecoratorStair宿主重算 | 复合组含墙+roof+door+stairs+color时必须完整undo/redo；hash_before/after不是相同数组的普通container hash猜测；第三aux owner继续以字段descriptor和writer确认，不凭排除法命名 |

交易合同先统一：domain ID、可复原before/after、owner dispatch、ReplayCmd expansion、录制标志、deferred删除完成、派生dirty集合和aux数据。不得把未知binary padding填成有语义默认。跨owner重复ID需按真实owner域区分，实体Entity可变，稳定domain ID必须恢复。

**cancel和redo截断**：已证明first actual add才截redo；仅打开/取消空组不能丢redo。现 start可替换pending，非原取消API。下一调查需定位pointer cancel/HistoryCmd::interrupt B140af5b20及结束输入的token调用：分别确认取消是否撤销实时预览、丢弃pending或提交已改变状态；cancel发生于首add后是否能保留旧redo不能猜。验收包含开始拖动无move、move后cancel、丢失focus、切工具、undo处理中再输入、连续多begin/end，同一组只一次finish。

**恢复ID与异步取消**：现CPU瓦片重建同步，不能凭“dirty”和Nani intention推断原异步生命周期完成。针对delete/morph/undo/import沿真实spawn-task→generation/cache key→完成consumer→cancel/removal路径调查。最小验收必须让编辑前的延迟任务在删除或undo后才完成，确认它不会回写旧宿主/旧几何；原generation/token规则未知先列调查项，不臆造并宣称原行为。保存/装载要清理当前任务、按恢复ID重建关系，再放行新任务，flush与原阶段证据一起确定。

**可并行研究现有snapshot，62缺失引用不是研究阻塞**：101已发行snapshot/52去重内容可立即按A/B/C格式和schema路径聚类，选各版已有完整snapshot+history对，执行原validate/load/init路径，恢复container section与建筑对象schema。缺失引用单独调查closest fallback和兼容政策；不得要求补齐所有缺失才开始schema和现有样本的可执行load验收。完整世界snapshot内含树/水等未实现owner时，建筑域读取必须保留其opaque或显式scope，不丢弃后宣称可原样全世界重存。

## 核心证据路径

- `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-history/findings.md`
- `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-history/version-comparison.json`
- `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/multi-version/B-parameter-findings.md`
- `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/multi-version/corpus-index.json`
- `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/multi-version/A-sources.json`（同目录 B/C-sources.json，*-functions.json）
- `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/history/observed-history-schema.json`
- `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/history/transaction.rs`
- `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/roof/replay.rs`
- `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/crates/tg-building-runtime/src/history.rs`
- `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/crates/tg-building-runtime/src/lib.rs`
- `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/tracking/project.json`

函数地址来自现有聚焦PDB候选索引，未执行的新函数不写作行为已确认；shared-address aliases需descriptor/caller再判定。完整性按上述业务场景门槛判断，不能将24测试、33局部算法组、函数量、源码path数量换算成全建筑完成率。
