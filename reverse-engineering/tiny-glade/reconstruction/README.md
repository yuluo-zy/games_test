# Tiny Glade 原作 Rust 还原工程

最终目标是完整复现给定发行构建的游戏行为和画面。当前工程只包含已由原作证据恢复的模块，不能称为完整游戏。

与正式工程 `game/tiny-glade` 的逐文件差异与移植清单见 [PORTING-CHECKLIST.md](PORTING-CHECKLIST.md)。

建筑计划第一批已接真实 Bevy：原碰撞资产与普通/半窗/门定位、墙高度命令、Roof 参数/类型切换、五类屋顶历史和计时回放、原 row 洞候选与 resolved proposal → 96B 墙砖实例。当前 33 个 workspace 测试通过；原 rows → proposal/noise/context 生产边、结构家族、快照/C迁移/原调度与GPU仍待恢复。完整范围见[实施证据](../evidence/building-integrate/findings.md)。

```sh
cargo run --release --locked -p tg-building-runtime --bin building-run -- crates/tg-building-runtime/fixtures/asset-parameter-history-cycle.json
```

这个 14 帧场景使用项目现有 B 发行 assets 的显式路径，包含真实资源放置、计时撤销/重做、平斜切换、门窗移动/删除与宿主清理；不是原游戏交互/画面验收。

当前工作优先逆向源码。已核查发行目录/PDB嵌入源，目前未取得作者原始Rust文本；这里是按原作机器码恢复并验证的可编译实现源码。GPU与场景回放后置。每个原函数族的骨架与证据见 [`../skeleton/README.md`](../skeleton/README.md) 与 `../evidence/`。

研究输入为 A/B/C 三个发行版本，身份见 [baselines.json](../tracking/baselines.json)。当前机器码实现以 B 为参照，最终目标版本按 [三版本计划](../docs/current/THREE_VERSION_RUST_PLAN.md)登记。Rust 工程与发行目录分离；没有读取或改动用户之前的 demo。

```sh
cargo build --release --workspace --locked
cargo run --release --locked --bin tg-reconstruction -- identity
cargo run --release --locked --bin tg-reconstruction -- aspect 1920 1080
cargo run --release --locked --bin tg-reconstruction -- roof-state 0.5 1.0
cargo run --release --locked --bin tg-reconstruction -- roof-profile 0.5 6.0 0.1 5.04
```

当前源码工作流程：真实过程定位 → 数据/调用链证据 → Rust类型与完整函数实现 → 原作机器码校验 → 源码模块联接。之后再做场景与GPU。普通 Cargo 构建只能证明工程可编译，不能代替机器码差分或最终游戏集成。

`roof_pipeline::assemble_roof_tiles_observed`组合完整矩形/圆形CPU瓦片生成。矩形依次调用四边条瓦、脊瓦、面瓦/补片，保留同一RNG/seed序列。CLI `roof-tiles <roof88hex> <rectangle24hex或-> <mode0/1> [输出文件]`可以独立生成64字节瓦片记录；无需原游戏进程或渲染环境。

机器码差分脚本位于 `../scripts`，报告位于 `../evidence`。当前覆盖范围与下一步见 [STATUS](../../../tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md) 及 [机器可读追踪](../tracking/project.json)。

新增 `crates/tg-history`，可以独立解析三个版本的打包历史，保留未知命令与大整数ID。`roof/parameters.rs` 恢复 B 参数布局与檐口计算；`roof/replay.rs` 恢复 VisualEdit 插值、创建/删除/方向/样式事件生产者。创建 payload 暂以观察字节接口保留，未确认的缺失字段不补默认值。

```sh
cargo test --workspace --locked
cargo run --release --locked --bin tg-reconstruction -- history-summary "/绝对路径/history.json"
```

`roof-history-step <B history.json> <entry> <edit> <roof88hex> <duration> <elapsed> <dt> <undo0/1>` 可执行实际 VisualEdit 到已解析屋顶的更新。roof88hex 必须由调用者提供；接口不选择历史游标，不负责 ECS 查询或建立场景。

当前验证入口（在本目录上一级运行，Python 3.11+ 环境需安装 Unicorn 2.1.4）：

```sh
python scripts/multi-version/verify_build.py
python scripts/multi-version/verify_history.py
python scripts/multi-version/verify_parameters.py
python scripts/multi-version/verify_replay.py
python scripts/multi-version/verify_discrete_replay.py
python scripts/multi-version/verify_history_chain.py
python scripts/source-edits/verify.py
python scripts/update-progress.py
```

先 `cargo build --release --workspace --locked`；差分脚本依赖当前 release 探针和基线登记的原 RNE 路径。报告绑定源码与 Cargo.lock 哈希。完整证据与未恢复边界见 [本轮发现](../evidence/multi-version/B-parameter-findings.md)。旧 Windows 构建报告不会自动计为当前扩展 workspace 的全链验收。

建筑业务新增 `building/wall_state.rs`、`building/roof_creation.rs`、`building/roof_spatial.rs`，联接墙状态、创建事件与屋顶更新。保持原依赖复用，不恢复hash probing、Bevy查询及事件容器。

```sh
# 从已解析的观察输入初始化Roof（不会spawn实体）
./target/release/tg-reconstruction building-roof-create <wall672hex> <create-event64hex> <initial-roof88hex>
# 更新已解析Roof和related组件，输出有序重建通知
./target/release/tg-reconstruction building-roof-update <wall672hex> <roof88hex> <related48hex> <wall-id-hex> <entity-hex>
```

输入布局限定B，`wall672hex`是672字节原墙状态投影，初始化Roof/related padding由调用者保留；未确认字段不构造默认值。接口不替代实体注册或完整场景。新差分入口为 `verify_building_state.py`、`verify_roof_creation.py`、`verify_building_chain.py`（同在 `scripts/multi-version`），分别2659/848/512例通过。见 [建筑发现](../evidence/building-state/findings.md)。每次选取新业务实现前执行 [依赖归属指导](../docs/current/DEPENDENCY_FILTERING.md)。

生命周期新增 `building/wall_publication.rs`、`roof_associations.rs`、`roof_components.rs`、`roof_deletion.rs`、`roof_lifecycle.rs`。公开库API `ResolvedRoofLifecycle::prepare/consume_delete_requests/deletion_mark_committed/advance`连接创建、删除意图、已提交标记、动画和移除意图；engine entity由调用者提供，不自行生成或重写Bevy调度。`RoofAssociations::rebuild`用标准HashMap处理已解析rows及presence变化，缺失墙按原清理规则处理。

发布字段投影清除缓存flag，resolved-only接口仍要求调用方填充缓存。下方新增真实Bevy runtime已接circle/rectangle缓存生产、typed空TriggerVolume与实际spawn/despawn；原始发布采集已接下方真实runtime；GPU消费者仍待恢复。

新增验证入口：`verify_lifecycle.py`（2316个B案例）、`verify_lifecycle_cycle.py`（128个组合）、`verify_component_versions.py`（三版各128个TileSim原码比较）与可复现CLI验证`verify_building_cli.py`。源码/ABI边界详见[本批记录](../evidence/building-lifecycle/findings.md)。

## 真实 Bevy headless 建筑运行时

`crates/tg-building-runtime` 已将墙发布字段→基础缓存→Roof组件创建→墙编辑传播→CPU瓦片→标记/动画/despawn→关联清理接入真实Bevy0.16 App。此前resolved-only CLI仍可用于差分，实际运行链使用：

```sh
cargo test --workspace --locked
cargo run --release --locked -p tg-building-runtime --bin building-run -- crates/tg-building-runtime/fixtures/building-cycle.json
```

该JSON是显式集成fixture；ID使用字符串。原始输入入口使用 `raw-building-cycle.json`：提供 raw Curve2、shape 和真实512×512 u16地形（fixture为显式常量网格），自动采集发布。`terrain_dirty_tiles` 接64×64空间脏格候选，墙移动/删除同步清理索引。circle/rectangle缓存、B逐拱段山墙、10个创建组件注册、atlas选择和Roof StoneFloor Nani CPU请求已接完整链。runtime包含20项连续World/注册测试，整workspace24项；registry依赖113项匹配B身份。GPU backend、freeform几何和原作fork完整排程仍待恢复。

```sh
cargo run --release --locked -p tg-building-runtime --bin building-run -- crates/tg-building-runtime/fixtures/raw-building-cycle.json
```

`publish_wall`仍保留resolved差分接口。字段 `source_point_count/source_curve_length` 必须来自原始Curve2，不能填重采样计数或墙高。见[本轮采集/山墙/Nani记录](../evidence/building-gather/findings.md)、[Nani边界](../evidence/building-nani/findings.md)。新增验证入口 `verify_wall_collection.py`、`verify_wall_terrain_versions.py`、`verify_wall_collection_dirty.py`、`verify_wall_spatial_index.py`、`verify_gable_arch.py`、`verify_nani.py`。

当前ARM矩形CPU整链33组有1组的一处坐标差1ULP，原始报告保持失败，不宣称本机全瓦片位精确。完整范围、注册证据与下一步见[运行时记录](../evidence/building-runtime/findings.md)。验证入口为 `scripts/multi-version/verify_wall_cache.py`、`verify_runtime.py`、`verify_build.py`（从调查根目录运行）。


方向切换、完整平屋顶外观与檐边消费者新增 `building/roof_direction.rs`、`roof_customization.rs`。它们经真实World传播到gable、瓦片、Nani和删除链，离散replay创建/删除/方向/檐边与删除撤销重建已接入。tag4原事件为FlatRimStyle，完整外观是独立12byte类型；通知、等值Changed写入与历史record flag分开处理。

```sh
cargo run --release --locked -p tg-building-runtime --bin building-run -- crates/tg-building-runtime/fixtures/roof-edit-cycle.json
```

八帧场景输出方向、rim/style、开口通知与待历史mutation。`RoofRimHistoryRequests`是适配队列，尚未执行完整history commit；`RecalculateWallHoles`实际几何消费者仍待恢复。原码入口 `verify_roof_direction.py`、`verify_roof_customization.py`，B方向1024、B/C外观和檐边各1536及各4失败guard通过。见[编辑消费者证据](../evidence/building-edits/findings.md)。


已将RecalculateWallHoles的实际publisher读者接到四类门窗raw生产、周期副本、finalized AABB和跨帧缓存。新增 `openings::UpsertOpening/RemoveOpening` 使用已解析墙位置和collision metadata，生成实际开口记录；原资产/UV解析和三维墙砖裁切未完成。Rim事务通过显式BuildingHistory账本配置运行start/add/commit/instant undo/redo与分叉截redo，不猜startup默认账本。

```sh
cargo run --release --locked -p tg-building-runtime --bin building-run -- crates/tg-building-runtime/fixtures/openings-history-cycle.json
```

十二帧场景和两个新World链测试通过；最新整workspace24项。新增验证 `verify_hole_invalidation.py`、`verify_wall_holes.py`、`verify_opening_rules.py`、`verify_history_transaction.py`、`verify_openings_history_runtime.py`。本轮边界见[开口/历史闭环](../evidence/building-holes/findings.md)；其他history owners、完整snapshot/保存和timed replay仍待恢复。
