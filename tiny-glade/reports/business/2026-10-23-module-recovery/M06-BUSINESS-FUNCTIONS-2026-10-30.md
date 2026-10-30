# M06 待核业务函数清单（2026-10-30）

## 2026-10-30 合并墙装饰碰撞归档复核

高度命令的 `todo!` 已核为原版失败语义而非缺失算法：B `adjust_wall_height_impl` @140ac1f20 的反编译 `decomp-0x140ac1f20.txt` 在读取事件 `+0x10` 后循环跳过 0，非 1 立即调用 1428d9430（字符串 142adf330、长度 0x13，`not yet implemented`），之后才进入墙查找。现有 UI 生产者 `ui_adjust_wall_height_mode.rs` 写模式 1；不能凭此断言其他值不可达。源码改为明确 `panic!("not yet implemented")`，并将现有回归改成无效模式加缺墙，验证失败次序；尚待中央 Cargo 运行。

B `merge_walls` @140999920 的反编译在 14099a 的归档段依次调用 1413e7230（`merge a`）、140b2bc00、1413e7230（`merge b`）、140b2bc00、1413e7230（`merge target`），随后迁移装饰存储；见 `reverse-engineering/tiny-glade/evidence/country-core-deep/wall/decomp/decomp-0x140999920.txt`。当前真实 App set 19 包含 `finish_wall → merge_walls → telegraph_walls_are_merged`，历史记录消费者在 set 34。此前合并路径只归档装饰数据桶，未转交活碰撞资源，无法保留两张碰撞表的历史载荷。

`country-core/src/systems/wall/merge_walls.rs` 现按原调用次序使用既有 `DecoratorArchivalResources` 和单地址归档 owner，保留空桶及首次归档语义，并读取两张活碰撞表。新增 native-physics 回归检查两张活网格以及合并后地址；尚未由中央 Cargo 构建运行，真实 App 合并→Undo/Redo、碰撞恢复与持久化消费者仍未验收。未声称整个合并算法、失败路径或 M6 完成。

图查询使用 README 的默认 published snapshot 协议及 B 地址搜索，首轮沙箱阻止本地 socket；受审查执行后返回 `Snapshot is absent or not complete; unpublished batches cannot be queried`。保留该错误作为本次查询结果；既有 `tools/code-map-runtime/verification-full.json` 记录快照 `053060f0c5d388b353567d8b` 曾通过完整验证，当前发布状态由图工具 owner 核查，不能据此认定图无证据或自行重建。

后续以 `query search --args '{"version":"B","text":"0x140999920"}'` 重试成功，发布快照 `5ac3dab66b4ac8c04a89025c` 将该地址映射到 `country_core::systems::wall::merge_walls::merge_walls` 和原 B 路径；B 二进制 SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`、PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。图报告 `collision/world.rs` 的源码绑定过期，图静态映射不证明运行时接线或语义。

历史消费者盘点：set 34 的 `history_record_merge_walls` 消费 pending，先记录可选高度调整，再写两个 Delete 和一个 Create；现有有界测试检查这四项的顺序和颜色。`ReplayBuffer` 在墙恢复前检查 `source_66`，`replay_wall` 将分组回放到 `InnerWalls`，真实 App 的 `history_runtime` 再将颜色请求路由到实际颜色 owner；持久化使用 `resources/history/serialized.rs` 的 Wall 编辑序列化。以上为静态接线与既有局部测试，不证明合并记录在真实 App 中经保存、重载、Undo/Redo 与碰撞恢复完整闭合。中央首轮构建 `/private/tmp/m6-merge-archive-20261030.log` 因测试夹具对 `DecoratorToStorageMap` 使用索引触发 E0608；已改用 `lookup_storage`，待中央重跑。

中央后续确认合并组12/12、双碰撞归档及模式失败各1项通过。原 `restore_archived_decorators` @140817a40 的 PDB set42 合同和保留机器码见 `evidence/round12/archival-owner/RESTORE-INTEGRATION.md`、`restore-registration.json` 与 `restore-original.asm`：八参包括 AppMode 和 ModeJustChanged，而非历史游标；墙擦除且无模式事件时跳过桶与碰撞恢复，楼梯始终恢复，模式事件时提取并立即丢弃差分。正常提交差分另由 `history_finish_recording_system` 负责。当前 `run_game.rs` 调用 `WallSystems::add_wall_systems`，其 set42 注册此函数；`system-decorator` 的另一个通用安装器不注册，不能把两个入口混同。

候选修复恢复原八参、桶清理及墙存在性、两张碰撞表的逐桶落位和事件控制；新增两个系统级回归。归档的 collider 位姿已包含网格局部位姿，新的现有碰撞 owner 入口直接使用最终位姿，避免重复叠加。尚未中央编译或验证；本地归档载荷为活 mesh 与位姿，原生独立 shape 的极端差异仍须证实，完整合并→历史回放→保存/重载/卸载仍未验收。

复核清桶：原 `141257900.asm` 的 `141257a20..a89` 以剩余计数和 hashbrown control 字节遍历 0x50 字节活桶项，`141257a78` 直接读项末字节（Info +0x47）并在值2时退出，而非仅从空迭代器构造 None；当前实现保留该分支。原与当前 HashMap 的桶扫描顺序不能假定跨平台一致，含状态2与其他项的清理前缀尚无动态差分。进一步将归档载荷补为活 collider shape Arc、最终位姿和注册 mesh/source；恢复用归档 shape 而非重建 mesh shape，同位姿已有 collider 的 shape 不同时也须替换。新增回归使用与 mesh 不同的 shape。中央首轮日志 `/private/tmp/m6-restore-collision-20261030.log` 的6处类型/夹具错误已修，修后仍待重新构建及运行；实际 App 和原侧差分尚未验证。

本清单从最新源码的 M06 词法库存中提取仍具有业务含义、但尚未唯一映射到当前实现的函数组。它是人工语义审计的输入，不是原版算法等价、生产接线或完成率的声明。

- 源码提交：`aa37abb24`
- B 函数索引 SHA256：`61a747815e31f7f080bf88b95d307c15750ea2090aec2226c0198673f4a9f711`
- 生成入口：[`tools/audit_m06_business_functions.py`](../../../tools/audit_m06_business_functions.py)
- 基础扫描：[`tools/audit_b_module_inventory.py`](../../../tools/audit_b_module_inventory.py)
- 机器可读结果：[`M06-BUSINESS-FUNCTIONS-2026-10-30.json`](M06-BUSINESS-FUNCTIONS-2026-10-30.json)

## 口径

库存的 `ambiguous_candidates` 表示同名实现不止一个；`unmatched_review` 表示没有同名实现，可能因为改名、跨 crate 迁移、泛型/宏展开，也可能真的缺失。两类都不能直接判为未实现。本清单排除 Rust trait、serde visitor、Bevy SystemParam 等机械名称，并清除仅凭 new/get/insert/remove 同名生成的关联。通用接口需另以类型 owner 和原证关联，不能据此判为无业务或已恢复。排除记录只留在基础 JSON 的 excluded_associations 中。

过滤前 M06 共 900 组，已清除 324 组名称关联：generic_method_requires_owner_evidence=46、mechanical_name_no_business_association=278。

| 指标 | 数量 | 比例 |
|---|---:|---:|
| M06 全部函数组 | 576 | 100.0% |
| 唯一非空体候选 | 315 | 54.7% |
| 多候选 | 69 | 12.0% |
| 未匹配 | 189 | 32.8% |
| 空体/部分占位 | 3 | 0.5% |

排除机械实现后：

| 指标 | 数量 | 比例 |
|---|---:|---:|
| 业务函数组 | 576 | 100.0% |
| 唯一非空体候选 | 315 | 54.7% |
| 唯一或同名候选 | 384 | 66.7% |
| 待人工核对的业务函数 | 258 | 44.8% |

## 待核业务函数分类

| 功能域 | 多候选 | 未匹配 | 合计 |
|---|---:|---:|---:|
| 墙权威、空间索引、形状、分段、吸附与合并 | 24 | 61 | 85 |
| 墙体装饰物：ID、存储、变换、网格、碰撞与归档 | 17 | 33 | 50 |
| 洞口、门、窗、栅栏与屋顶窗 | 11 | 33 | 44 |
| 墙体编辑 UI、手柄、高度、移动与预览 | 6 | 15 | 21 |
| 抹灰、半木与曲面半木 | 2 | 10 | 12 |
| 柱、塔柱、托架与高差支撑 | 0 | 12 | 12 |
| 擦除、删除归档与历史 | 4 | 6 | 10 |
| 材料统计与颜色 | 1 | 8 | 9 |
| 过梁与承重梁 | 0 | 6 | 6 |
| 拱形生成与拱曲线 | 2 | 3 | 5 |
| 其他构造辅助 | 2 | 2 | 4 |

## 明细

“候选源码”仅供定位；多候选中的硬编码数量上限只影响本报告展示，JSON 保留全部候选。

### 其他构造辅助

- **多候选** `country_core::resources::walls::WallId::get_inner`
  - 原路径：`crates/country-core/src/resources/walls/mod.rs`
  - B ID：`77b92ea39180934fe9`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator.rs:93`<br>`tiny-glade/crates/country-core/src/resources/walls/mod.rs:58`
- **多候选** `country_core::resources::walls::WallId::is_valid`
  - 原路径：`crates/country-core/src/resources/walls/mod.rs`
  - B ID：`8f3fc44e86f1dd84ba`
  - 候选源码：`tiny-glade/crates/rhapsody/src/sync/generation/mod.rs:34`<br>`tiny-glade/crates/country-core/src/asset_libraries/mod.rs:66`<br>`tiny-glade/crates/country-core/src/resources/walls/mod.rs:61`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_trigger_volume.rs:72`
- **未匹配** `country_core::systems::wall::shapes::impl$0::add_wall_shape_ux`
  - 原路径：`crates/country-core/src/systems/wall/shapes/mod.rs`
  - B ID：`d3e3f866a069d0f079`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::rectangle::get_rectangle_orientation`
  - 原路径：`crates/country-core/src/systems/wall/shapes/rectangle/mod.rs`
  - B ID：`576702aed34432a231`
  - 候选源码：—

### 墙体编辑 UI、手柄、高度、移动与预览

- **多候选** `country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier`
  - 原路径：`crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs`
  - B ID：`2dbff7ea49929e0a0f`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height.rs:88`<br>`tiny-glade/crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs:121`<br>`tiny-glade/crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs:95`
- **多候选** `country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget`
  - 原路径：`crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs`
  - B ID：`a9380ed01ee4926ff3`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height.rs:59`<br>`tiny-glade/crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs:45`<br>`tiny-glade/crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs:46`
- **多候选** `country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::signifier`
  - 原路径：`crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs`
  - B ID：`a8589fc04198cec61e`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height.rs:88`<br>`tiny-glade/crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs:121`<br>`tiny-glade/crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs:95`
- **多候选** `country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::widget`
  - 原路径：`crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs`
  - B ID：`ec6d828bb2ea16694a`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height.rs:59`<br>`tiny-glade/crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs:45`<br>`tiny-glade/crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs:46`
- **多候选** `country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier`
  - 原路径：`crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs`
  - B ID：`aece28308ab0412210`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height.rs:88`<br>`tiny-glade/crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs:121`<br>`tiny-glade/crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs:95`
- **多候选** `country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget`
  - 原路径：`crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs`
  - B ID：`4ce93a6240d2ad9ab9`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height.rs:59`<br>`tiny-glade/crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs:45`<br>`tiny-glade/crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs:46`
- **未匹配** `country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls`
  - 原路径：`crates/country-core/src/systems/ui_move_freehand_walls.rs`
  - B ID：`8684a06f7009b4f1ab`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve`
  - 原路径：`crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs`
  - B ID：`4809b6fcfb8ce6ee99`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::init_mode`
  - 原路径：`crates/country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height_mode.rs`
  - B ID：`66ca97a397b7b5b734`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::circle::ui_edit_circle::finish_circle_edit`
  - 原路径：`crates/country-core/src/systems/wall/shapes/circle/ui_edit_circle.rs`
  - B ID：`b4f2a32429e5e43a11`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::circle::ui_edit_circle::get_trigger_vol`
  - 原路径：`crates/country-core/src/systems/wall/shapes/circle/ui_edit_circle.rs`
  - B ID：`c120e226dc7740bf9f`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::circle::ui_edit_circle::update_circle_radius`
  - 原路径：`crates/country-core/src/systems/wall/shapes/circle/ui_edit_circle.rs`
  - B ID：`d6f9417f2219c19f6e`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_dims_from_cursor_pos`
  - 原路径：`crates/country-core/src/systems/wall/shapes/rectangle/ui_create_rectangle.rs`
  - B ID：`90d5937b7052544e69`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::get_distance_to_edge`
  - 原路径：`crates/country-core/src/systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs`
  - B ID：`f7144fcacb72b22104`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::skewed_box_trigger_vol`
  - 原路径：`crates/country-core/src/systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs`
  - B ID：`41ba3a3b8554e242ef`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate`
  - 原路径：`crates/country-core/src/systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs`
  - B ID：`47c19a9ca3103ac2d4`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::trigger_volume`
  - 原路径：`crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs`
  - B ID：`31c47e6f83a48ac103`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::impl$0::to_widget_name_hash`
  - 原路径：`crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs`
  - B ID：`18f5cb2b03cd0d7689`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$0::to_widget_name_hash`
  - 原路径：`crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs`
  - B ID：`a54306aa16f8f08359`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeSignifierLerp::lerp_radius`
  - 原路径：`crates/country-core/src/systems/wall/shapes/utils/shape_signifier_lerp.rs`
  - B ID：`a4b06a715e5a56cacb`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeVal::interpolate`
  - 原路径：`crates/country-core/src/systems/wall/shapes/utils/shape_signifier_lerp.rs`
  - B ID：`435602414db0ec4b99`
  - 候选源码：—

### 墙体装饰物：ID、存储、变换、网格、碰撞与归档

- **多候选** `country_core::resources::walls::decorator::DecoratorDst::as_wall`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`88e749928799027c11`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator.rs:532`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:49`
- **多候选** `country_core::resources::walls::decorator::DecoratorDst::is_wall`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`68228211ad46f6d4c3`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator.rs:519`<br>`tiny-glade/crates/country-core/src/resources/walls/inner_wall_state.rs:43`
- **多候选** `country_core::resources::walls::decorator::DecoratorId::get_inner`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`f4b92642a6876f9a95`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator.rs:93`<br>`tiny-glade/crates/country-core/src/resources/walls/mod.rs:58`
- **多候选** `country_core::resources::walls::decorator::DecoratorIdGen::generate`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`00e105ff58a3e62026`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator.rs:138`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator.rs:170`
- **多候选** `country_core::resources::walls::decorator::TransientDecoratorIdGen::generate`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`02ec23aa7d9cb76821`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator.rs:138`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator.rs:170`
- **多候选** `country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut`
  - 原路径：`crates/country-core/src/resources/walls/decorator_storage.rs`
  - B ID：`1ba3cb8026cddc5518`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/public_walls.rs:83`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:140`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:233`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:512`
- **多候选** `country_core::resources::walls::decorator_storage::DecoratorStorage::clear_modification_tracking`
  - 原路径：`crates/country-core/src/resources/walls/decorator_storage.rs`
  - B ID：`f0b661455b44ef4076`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:211`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:727`
- **多候选** `country_core::resources::walls::decorator_storage::DecoratorStorage::get_mut`
  - 原路径：`crates/country-core/src/resources/walls/decorator_storage.rs`
  - B ID：`dcdf73a99dcb2f962f`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/public_walls.rs:83`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:140`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:233`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:512`
- **多候选** `country_core::resources::walls::decorator_storage::DecoratorStorage::iter_mut`
  - 原路径：`crates/country-core/src/resources/walls/decorator_storage.rs`
  - B ID：`7960372696d555e2c4`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:249`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:687`
- **多候选** `country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall`
  - 原路径：`crates/country-core/src/resources/walls/decorator_storage.rs`
  - B ID：`35c213f0f4a696ca4d`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator.rs:532`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:49`
- **多候选** `country_core::resources::walls::decorator_storage::Decorators::clear`
  - 原路径：`crates/country-core/src/resources/walls/decorator_storage.rs`
  - B ID：`173cbeb38da6a5ccc1`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/curved_halftimber.rs:24`<br>`tiny-glade/crates/country-core/src/resources/walls/wooden_gates.rs:20`<br>`tiny-glade/crates/country-core/src/resources/walls/plaster.rs:42`
- **多候选** `country_core::resources::walls::decorator_storage::Decorators::clear_modification_tracking`
  - 原路径：`crates/country-core/src/resources/walls/decorator_storage.rs`
  - B ID：`e997ae6a10e0dbfcfb`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:211`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:727`
- **多候选** `country_core::resources::walls::decorator_storage::Decorators::get_mut`
  - 原路径：`crates/country-core/src/resources/walls/decorator_storage.rs`
  - B ID：`57720fb3eea8f9968b`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/public_walls.rs:83`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:140`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:233`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:512`
- **多候选** `country_core::resources::walls::decorator_storage::Decorators::iter_mut`
  - 原路径：`crates/country-core/src/resources/walls/decorator_storage.rs`
  - B ID：`3f00ad0aefebf2f0ac`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:249`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:687`
- **多候选** `country_core::resources::walls::decorator_storage::impl$8::update`
  - 原路径：`crates/country-core/src/resources/walls/decorator_storage.rs`
  - B ID：`b53f2aa068e0e9d053`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/ui_focus_mode_shape_height.rs:87`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:101`
- **多候选** `country_core::resources::walls::decorator_subtype::impl$6::into_decorator_subtype`
  - 原路径：`crates/country-core/src/resources/walls/decorator_subtype.rs`
  - B ID：`ac6ac2d70aa0d605d0`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator_subtype.rs:55`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_subtype.rs:193`
- **多候选** `country_core::resources::walls::decorator_subtype::impl$7::into_decorator_subtype`
  - 原路径：`crates/country-core/src/resources/walls/decorator_subtype.rs`
  - B ID：`7f3cbd19bdc09bd9f1`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator_subtype.rs:55`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_subtype.rs:193`
- **未匹配** `country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::swap_buffers_and_clear`
  - 原路径：`crates/country-core/src/resources/walls/cached_decorator_transforms.rs`
  - B ID：`92a65542fef1c2baae`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform`
  - 原路径：`crates/country-core/src/resources/walls/cached_decorator_transforms.rs`
  - B ID：`7b366ec2ea12dbfdf9`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::try_position`
  - 原路径：`crates/country-core/src/resources/walls/cached_decorator_transforms.rs`
  - B ID：`ee0d1709ec78eeb8f5`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::try_transform`
  - 原路径：`crates/country-core/src/resources/walls/cached_decorator_transforms.rs`
  - B ID：`4163f570244434bc2f`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator::DecoratorOpEventKind::id`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`fbded5d516db7ae57c`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator::DecoratorRotationXZ::from_quat`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`cfe8cb2212d5c3e625`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator::DecoratorRotationXZ::from_xz_forward_vector`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`47876a4d3919081d4a`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator::DecoratorRotationXZ::offset`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`ebfb2cdd6e7dcf4c24`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator::DecoratorRotationXZ::to_projective_xz_mat2`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`c2e5d86d4cfe8158fa`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator::DecoratorRotationXZ::to_quat`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`17673d190593002328`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator::DecoratorRotationXZ::to_xz_forward_vector`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`8042727c03a2102c85`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator::DeferredDecoratorOpEvents::events`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`21ba8c74db9004afe5`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator::DeferredDecoratorOpEvents::send`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`f7e4d09d39d7199802`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator::DeferredDecoratorOpEvents::unleash`
  - 原路径：`crates/country-core/src/resources/walls/decorator.rs`
  - B ID：`752be8882241fd483c`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archived_walls`
  - 原路径：`crates/country-core/src/resources/walls/decorator_archivist.rs`
  - B ID：`62fe0c86fbdf4e89d3`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_colliders`
  - 原路径：`crates/country-core/src/resources/walls/decorator_archivist.rs`
  - B ID：`c6241b8bfc5bb8f572`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_clear`
  - 原路径：`crates/country-core/src/resources/walls/decorator_archivist.rs`
  - B ID：`b48bfddcd51608cf97`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_stairs`
  - 原路径：`crates/country-core/src/resources/walls/decorator_archivist.rs`
  - B ID：`0dbde77070c8f9f819`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_terrain`
  - 原路径：`crates/country-core/src/resources/walls/decorator_archivist.rs`
  - B ID：`f7bd34410b445a673a`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_interactions`
  - 原路径：`crates/country-core/src/resources/walls/decorator_archivist.rs`
  - B ID：`ab2d05d11d3ead8907`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_archived_decorators`
  - 原路径：`crates/country-core/src/resources/walls/decorator_archivist.rs`
  - B ID：`8bdfb7ca317704f88a`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_before_after_all`
  - 原路径：`crates/country-core/src/resources/walls/decorator_archivist.rs`
  - B ID：`3772678e07311cd920`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_before_after_impl`
  - 原路径：`crates/country-core/src/resources/walls/decorator_archivist.rs`
  - B ID：`7b74425f00761b4f83`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh`
  - 原路径：`crates/country-core/src/resources/walls/decorator_subtype.rs`
  - B ID：`cd2dafb07e6d62509b`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls`
  - 原路径：`crates/country-core/src/resources/walls/decorator_subtype.rs`
  - B ID：`78f92eb36c5452656b`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_outline`
  - 原路径：`crates/country-core/src/resources/walls/decorator_subtype.rs`
  - B ID：`c397e903bedcb3dd6b`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_outline`
  - 原路径：`crates/country-core/src/resources/walls/decorator_subtype.rs`
  - B ID：`b715831afcfc1ee124`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert_collision_and_interaction_as_same`
  - 原路径：`crates/country-core/src/resources/walls/decorator_subtype.rs`
  - B ID：`df624793c6001d25fb`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh`
  - 原路径：`crates/country-core/src/resources/walls/decorator_subtype.rs`
  - B ID：`8b4e81f3e6664500ec`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh`
  - 原路径：`crates/country-core/src/resources/walls/decorator_subtype.rs`
  - B ID：`2ac46aa33880233023`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh`
  - 原路径：`crates/country-core/src/resources/walls/decorator_subtype.rs`
  - B ID：`3b3fee8410ab468792`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::try_get_subtype`
  - 原路径：`crates/country-core/src/resources/walls/decorator_subtype.rs`
  - B ID：`8afa7d88d9519b005f`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::eraser::ui_bulldozer::handle_decorators`
  - 原路径：`crates/country-core/src/systems/wall/eraser/ui_bulldozer.rs`
  - B ID：`1674881c884f11962d`
  - 候选源码：—

### 墙权威、空间索引、形状、分段、吸附与合并

- **多候选** `country_core::resources::walls::inner_wall_state::StructureType::is_wall`
  - 原路径：`crates/country-core/src/resources/walls/inner_wall_state.rs`
  - B ID：`0a450e2d77658483fc`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/decorator.rs:519`<br>`tiny-glade/crates/country-core/src/resources/walls/inner_wall_state.rs:43`
- **多候选** `country_core::resources::walls::inner_wall_state::WallState::height`
  - 原路径：`crates/country-core/src/resources/walls/inner_wall_state.rs`
  - B ID：`cb031def9d55499834`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/shapes/shape_placement.rs:92`<br>`tiny-glade/crates/country-core/src/resources/walls/inner_walls.rs:51`<br>`tiny-glade/crates/country-core/src/resources/walls/inner_wall_state.rs:227`
- **多候选** `country_core::resources::walls::inner_wall_state::WallState::is_premade_shape`
  - 原路径：`crates/country-core/src/resources/walls/inner_wall_state.rs`
  - B ID：`42cc9f03f265a4562c`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/inner_wall_state.rs:265`<br>`tiny-glade/crates/country-core/src/resources/walls/public_wall_state.rs:150`
- **多候选** `country_core::resources::walls::inner_walls::InnerWalls::all`
  - 原路径：`crates/country-core/src/resources/walls/inner_walls.rs`
  - B ID：`2f5febd5d9b72be69a`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/inner_walls.rs:103`<br>`tiny-glade/crates/country-core/src/resources/walls/public_walls.rs:88`
- **多候选** `country_core::resources::walls::inner_walls::InnerWalls::contains`
  - 原路径：`crates/country-core/src/resources/walls/inner_walls.rs`
  - B ID：`1915bfe696bcd6e5f9`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs:52`<br>`tiny-glade/crates/country-core/src/resources/walls/inner_walls.rs:99`
- **多候选** `country_core::resources::walls::inner_walls::InnerWalls::new_wall`
  - 原路径：`crates/country-core/src/resources/walls/inner_walls.rs`
  - B ID：`5391baf8129a0cce4f`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/history_recording.rs:35`<br>`tiny-glade/crates/country-core/src/resources/walls/inner_walls.rs:121`
- **多候选** `country_core::resources::walls::inner_walls::InnerWalls::wall_ids`
  - 原路径：`crates/country-core/src/resources/walls/inner_walls.rs`
  - B ID：`5388bce9c8524fc909`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/inner_walls.rs:107`<br>`tiny-glade/crates/country-core/src/resources/walls/public_walls.rs:91`
- **多候选** `country_core::resources::walls::public_wall_state::PublicWallState::flat_roof_y`
  - 原路径：`crates/country-core/src/resources/walls/public_wall_state.rs`
  - B ID：`0806b27b506794a0a1`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/public_wall_state.rs:212`<br>`tiny-glade/crates/country-core/src/resources/walls/height_rules.rs:64`
- **多候选** `country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape`
  - 原路径：`crates/country-core/src/resources/walls/public_wall_state.rs`
  - B ID：`649d97fc49e17beae9`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/inner_wall_state.rs:265`<br>`tiny-glade/crates/country-core/src/resources/walls/public_wall_state.rs:150`
- **多候选** `country_core::resources::walls::public_wall_state::PublicWallState::max_y`
  - 原路径：`crates/country-core/src/resources/walls/public_wall_state.rs`
  - B ID：`f5cafe2d3284e0358e`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/publication_fields.rs:97`<br>`tiny-glade/crates/country-core/src/resources/walls/public_wall_state.rs:204`
- **多候选** `country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable`
  - 原路径：`crates/country-core/src/resources/walls/public_wall_state.rs`
  - B ID：`a35c0608333afabcd8`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/publication_fields.rs:100`<br>`tiny-glade/crates/country-core/src/resources/walls/public_wall_state.rs:209`
- **多候选** `country_core::resources::walls::public_walls::PublicWalls::wall_ids`
  - 原路径：`crates/country-core/src/resources/walls/public_walls.rs`
  - B ID：`1329c4eb8d547dccf6`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/inner_walls.rs:107`<br>`tiny-glade/crates/country-core/src/resources/walls/public_walls.rs:91`
- **多候选** `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::is_valid`
  - 原路径：`crates/country-core/src/resources/walls/wall_trigger_volume.rs`
  - B ID：`a6793f5cc1f3ad51c4`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/mod.rs:61`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_trigger_volume.rs:72`
- **多候选** `country_core::systems::wall::draw_wall::draw_wall`
  - 原路径：`crates/country-core/src/systems/wall/draw_wall.rs`
  - B ID：`0dc789d7cbc7816a5a`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/draw_wall.rs:108`<br>`tiny-glade/crates/country-core/src/systems/wall/structure_outline.rs:464`
- **多候选** `country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2`
  - 原路径：`crates/country-core/src/systems/wall/valid_enclosure/mod.rs`
  - B ID：`7a6d4b8957a81de6fe`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:115`<br>`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:252`
- **多候选** `country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3`
  - 原路径：`crates/country-core/src/systems/wall/valid_enclosure/mod.rs`
  - B ID：`39fbaf7e95bfff6baa`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:123`<br>`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:256`
- **多候选** `country_core::systems::wall::valid_enclosure::ShapeParameters::center`
  - 原路径：`crates/country-core/src/systems/wall/valid_enclosure/mod.rs`
  - B ID：`8ee1796c5b980f5ef7`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:44`<br>`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:218`
- **多候选** `country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point`
  - 原路径：`crates/country-core/src/systems/wall/valid_enclosure/mod.rs`
  - B ID：`fa1f7b1a0a5dbc7460`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:83`<br>`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:241`
- **多候选** `country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape`
  - 原路径：`crates/country-core/src/systems/wall/valid_enclosure/mod.rs`
  - B ID：`898f4b40549abaa2bb`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:97`<br>`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:245`
- **多候选** `country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos`
  - 原路径：`crates/country-core/src/systems/wall/valid_enclosure/mod.rs`
  - B ID：`70b54687c16729a522`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:75`<br>`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:235`
- **多候选** `country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u`
  - 原路径：`crates/country-core/src/systems/wall/valid_enclosure/mod.rs`
  - B ID：`b1d46a47a29481fca5`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:67`<br>`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:231`
- **多候选** `country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u`
  - 原路径：`crates/country-core/src/systems/wall/valid_enclosure/mod.rs`
  - B ID：`806829daf78e5fd347`
  - 候选源码：`tiny-glade/crates/utils/src/curve/mod.rs:341`<br>`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:59`<br>`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:227`
- **多候选** `country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter`
  - 原路径：`crates/country-core/src/systems/wall/valid_enclosure/mod.rs`
  - B ID：`9aa135117c84d951a9`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:51`<br>`tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs:223`
- **多候选** `country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::add`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`56358839cd2ba9491d`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/eraser/bulldoze.rs:84`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes.rs:28`<br>`tiny-glade/crates/country-core/src/resources/walls/freeform_height_cache.rs:28`<br>`tiny-glade/crates/country-core/src/resources/walls/height_cache.rs:50`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes/geometry.rs:58`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes/geometry.rs:161`
- **未匹配** `country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height`
  - 原路径：`crates/country-core/src/resources/walls/inner_wall_state.rs`
  - B ID：`977df54799a3df69c6`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::inner_wall_state::WallState::is_enclosed`
  - 原路径：`crates/country-core/src/resources/walls/inner_wall_state.rs`
  - B ID：`63b78d4b5ab51d3790`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version`
  - 原路径：`crates/country-core/src/resources/walls/inner_wall_state.rs`
  - B ID：`f738f71aaacbfb8283`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::inner_wall_state::deserialize_wall_style`
  - 原路径：`crates/country-core/src/resources/walls/inner_wall_state.rs`
  - B ID：`51fc19cea3459882a6`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::inner_walls::InnerWalls::_deprecated_update_skirt`
  - 原路径：`crates/country-core/src/resources/walls/inner_walls.rs`
  - B ID：`275a4f338812e3567f`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed`
  - 原路径：`crates/country-core/src/resources/walls/public_wall_state.rs`
  - B ID：`06be6544b8a24b473b`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_attachment::WallCornerAttachment::from_rectangle_corner_id`
  - 原路径：`crates/country-core/src/resources/walls/wall_attachment.rs`
  - B ID：`e35d61ff9bc81d4345`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached`
  - 原路径：`crates/country-core/src/resources/walls/wall_attachment.rs`
  - B ID：`91452cd9a0f447e4ed`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id`
  - 原路径：`crates/country-core/src/resources/walls/wall_attachment.rs`
  - B ID：`756a63f7549bc2a8b7`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_trigger_volume::Quad::apply_params`
  - 原路径：`crates/country-core/src/resources/walls/wall_trigger_volume.rs`
  - B ID：`92bc0fc03e730aaf42`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_trigger_volume::Quad::as_triangles_w_normals_n_uvs`
  - 原路径：`crates/country-core/src/resources/walls/wall_trigger_volume.rs`
  - B ID：`5625c2dea53e6cded2`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback`
  - 原路径：`crates/country-core/src/resources/walls/wall_trigger_volume.rs`
  - B ID：`810e32004e1438a55d`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_position_from_mesh_uv`
  - 原路径：`crates/country-core/src/resources/walls/wall_trigger_volume.rs`
  - B ID：`407675086060c6aa41`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute_quads`
  - 原路径：`crates/country-core/src/resources/walls/wall_trigger_volume.rs`
  - B ID：`8cd8c6d2b56fcaf082`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_trigger_volume::WallTriggerVolumeParams::offset_from_the_top_ws`
  - 原路径：`crates/country-core/src/resources/walls/wall_trigger_volume.rs`
  - B ID：`17dd4d07ad7be8ae5a`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_trigger_volume::WallTriggerVolumeParams::u_range`
  - 原路径：`crates/country-core/src/resources/walls/wall_trigger_volume.rs`
  - B ID：`76005c22d1b038fd1b`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to`
  - 原路径：`crates/country-core/src/systems/wall/shapes/shape_placement.rs`
  - B ID：`9d7f04d4adf787a429`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::snapping::snapping_to_ends::find_closest_end_to_snap_to`
  - 原路径：`crates/country-core/src/systems/wall/snapping/snapping_to_ends.rs`
  - B ID：`09195650ccc6c5ad23`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to`
  - 原路径：`crates/country-core/src/systems/wall/snapping/snapping_to_segments.rs`
  - B ID：`df7101cf2ec2658220`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::ui_continue_wall::WallContinueState::display_signifier`
  - 原路径：`crates/country-core/src/systems/wall/ui_continue_wall.rs`
  - B ID：`ab4edde5152e8af58f`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs`
  - B ID：`067ef0eb47d91919f3`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::publish_public_wall_state::process`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs`
  - B ID：`5c78d81fb03586e4df`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_roof_hashmap::wall_roof_hashmap`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_roof_hashmap.rs`
  - B ID：`8f1c90e134b20d3406`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::remove_segments`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`b7c8134700d729f55e`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::walls`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`d3473c75da80b495eb`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment::terrain_height_closest_to_ws_pos`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`5275df1dc59fae6eab`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::bounds_ws`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`7e7f197e69147faedc`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_index_to_grid_coord`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`5318069b633f6b3773`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`9acf723712d865c71e`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::dims`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`0e75f47c891c04957f`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cell_at_index`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`e8fc25a7c5e02180d8`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_aabb`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`83f66470ac4790f280`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`54aaeedeb197aa28f4`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`43ede893f6e2fff486`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_aabb_clamped`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`09909063cb2c0ff22a`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_segment`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`71e44511b9f56cbf0d`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`5353e8df30db764f77`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord_unclamped`
  - 原路径：`crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs`
  - B ID：`ef6c5f399e0b0059f4`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_walls::create_wall`
  - 原路径：`crates/systems/wall-constructor/src/construct_walls.rs`
  - B ID：`f84afcdda06feab939`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_platform_stairs::construct_platform_stairs`
  - 原路径：`crates/systems/wall-constructor/src/lib.rs`
  - B ID：`4698137d1f48cfc0e0`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_playermade_stairs::preprocess_curve`
  - 原路径：`crates/systems/wall-constructor/src/lib.rs`
  - B ID：`e4f08bae84d74b075b`
  - 候选源码：—
- **未匹配** `system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split`
  - 原路径：`crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs`
  - B ID：`ad32f70231e3468f70`
  - 候选源码：—
- **未匹配** `system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split_with_offset`
  - 原路径：`crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs`
  - B ID：`074d9da0f39c6c4202`
  - 候选源码：—
- **未匹配** `system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_splits_in_empty_range`
  - 原路径：`crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs`
  - B ID：`b2f89dc554e95e5cd2`
  - 候选源码：—
- **未匹配** `system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation`
  - 原路径：`crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs`
  - B ID：`c786d2100e1db2ebbe`
  - 候选源码：—
- **未匹配** `system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation`
  - 原路径：`crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs`
  - B ID：`0e06660b9c5a6ecd1f`
  - 候选源码：—
- **未匹配** `system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment`
  - 原路径：`crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs`
  - B ID：`bee32d44960d20b464`
  - 候选源码：—
- **未匹配** `system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate`
  - 原路径：`crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs`
  - B ID：`46ad62f1f6f0a6d1f8`
  - 候选源码：—
- **未匹配** `system_wall_constructor::segmentation::sample_wall_path_intersections::add_point_with_mask_to_output`
  - 原路径：`crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs`
  - B ID：`7bae423b466a9727c8`
  - 候选源码：—
- **未匹配** `system_wall_constructor::segmentation::sample_wall_path_intersections::arch_function`
  - 原路径：`crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs`
  - B ID：`5ef9fdddbca9f2ca86`
  - 候选源码：—
- **未匹配** `system_wall_constructor::segmentation::sample_wall_path_intersections::threshold_curve_path_mask`
  - 原路径：`crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs`
  - B ID：`e6462593e27195c92f`
  - 候选源码：—
- **未匹配** `system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments`
  - 原路径：`crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs`
  - B ID：`a93ce14267bf1fd1fd`
  - 候选源码：—
- **未匹配** `system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl`
  - 原路径：`crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs`
  - B ID：`ef3220135206b6775d`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::perturb_row_ends`
  - 原路径：`crates/systems/wall-constructor/src/utils/mod.rs`
  - B ID：`ffe6db982a7102d268`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::split_horizontally`
  - 原路径：`crates/systems/wall-constructor/src/utils/mod.rs`
  - B ID：`06f9c4b0575084c74c`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::trim_rows::impl$0::above`
  - 原路径：`crates/systems/wall-constructor/src/utils/trim_rows.rs`
  - B ID：`16cf1449d89069aa0d`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::trim_rows::impl$0::above_to_below`
  - 原路径：`crates/systems/wall-constructor/src/utils/trim_rows.rs`
  - B ID：`74fce7c764fa08281c`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::trim_rows::impl$0::below_to_above`
  - 原路径：`crates/systems/wall-constructor/src/utils/trim_rows.rs`
  - B ID：`d0ba1cafd384eab2b5`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::wall_corners::add_wall_corners`
  - 原路径：`crates/systems/wall-constructor/src/utils/wall_corners.rs`
  - B ID：`78b7b605e7e039bb01`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::wall_skirts::add_skirts`
  - 原路径：`crates/systems/wall-constructor/src/utils/wall_skirts.rs`
  - B ID：`d58fb52c2d2def35ac`
  - 候选源码：—
- **未匹配** `system_wall_constructor::wall_constructor::WallConstructor::from_curve`
  - 原路径：`crates/systems/wall-constructor/src/wall_constructor.rs`
  - B ID：`78caa6273535e9f1f6`
  - 候选源码：—

### 抹灰、半木与曲面半木

- **多候选** `country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables::clear`
  - 原路径：`crates/country-core/src/resources/walls/curved_halftimber.rs`
  - B ID：`2c553ab5042a0cd887`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/curved_halftimber.rs:24`<br>`tiny-glade/crates/country-core/src/resources/walls/wooden_gates.rs:20`<br>`tiny-glade/crates/country-core/src/resources/walls/plaster.rs:42`
- **多候选** `country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered`
  - 原路径：`crates/country-core/src/resources/walls/inner_wall_state.rs`
  - B ID：`1fe8258ca0c3b8448c`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/publication_fields.rs:110`<br>`tiny-glade/crates/country-core/src/resources/walls/inner_wall_state.rs:78`
- **未匹配** `country_core::resources::walls::inner_walls::InnerWalls::update_plaster_peel`
  - 原路径：`crates/country-core/src/resources/walls/inner_walls.rs`
  - B ID：`8b9fcfdaddc46b2c22`
  - 候选源码：—
- **未匹配** `system_wall_constructor::plaster_systems::create_plaster`
  - 原路径：`crates/systems/wall-constructor/src/plaster_systems.rs`
  - B ID：`0864c8f36c35decc11`
  - 候选源码：—
- **未匹配** `system_wall_constructor::plaster_systems::generate_plaster_mesh`
  - 原路径：`crates/systems/wall-constructor/src/plaster_systems.rs`
  - B ID：`27754d08f10352be0a`
  - 候选源码：—
- **未匹配** `system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal`
  - 原路径：`crates/systems/wall-constructor/src/plaster_systems.rs`
  - B ID：`d576a1092d6fc32c2e`
  - 候选源码：—
- **未匹配** `system_wall_constructor::plaster_systems::maintain_plaster`
  - 原路径：`crates/systems/wall-constructor/src/plaster_systems.rs`
  - B ID：`506097572453da6092`
  - 候选源码：—
- **未匹配** `system_wall_constructor::plaster_systems::sd_box`
  - 原路径：`crates/systems/wall-constructor/src/plaster_systems.rs`
  - B ID：`a0d0ce9f07fa30e75c`
  - 候选源码：—
- **未匹配** `system_wall_constructor::plaster_systems::split_wall_into_sides`
  - 原路径：`crates/systems/wall-constructor/src/plaster_systems.rs`
  - B ID：`65a84feda6204c2e2f`
  - 候选源码：—
- **未匹配** `system_wall_constructor::plaster_systems::vec_push_within_capacity_assert`
  - 原路径：`crates/systems/wall-constructor/src/plaster_systems.rs`
  - B ID：`fbf5f03d4611ecc594`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::half_timber::roof_outline`
  - 原路径：`crates/systems/wall-constructor/src/utils/half_timber.rs`
  - B ID：`6fcbaada077cba086c`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::half_timber::wrap_around`
  - 原路径：`crates/systems/wall-constructor/src/utils/half_timber.rs`
  - B ID：`dde253bc798edee077`
  - 候选源码：—

### 拱形生成与拱曲线

- **多候选** `system_wall_constructor::construct_arches::construct_arches`
  - 原路径：`crates/systems/wall-constructor/src/construct_arches/mod.rs`
  - B ID：`89e9c257999bc1b340`
  - 候选源码：`tiny-glade/crates/systems/wall-constructor/src/construct_arches/mod.rs:56`<br>`tiny-glade/crates/country-core/src/resources/walls/public_wall_state.rs:239`
- **多候选** `system_wall_constructor::construct_arches::profile::ArchFunction::remap_t`
  - 原路径：`crates/systems/wall-constructor/src/construct_arches/profile.rs`
  - B ID：`9b2c14a4f5807b9a99`
  - 候选源码：`tiny-glade/crates/systems/wall-constructor/src/construct_arches/profile.rs:77`<br>`tiny-glade/crates/systems/wall-constructor/src/construct_arches/profile.rs:103`
- **未匹配** `system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks`
  - 原路径：`crates/systems/wall-constructor/src/arch_walker.rs`
  - B ID：`98bd1489c9328f5760`
  - 候选源码：—
- **未匹配** `system_wall_constructor::arch_walker::curve_to_arch_ws`
  - 原路径：`crates/systems/wall-constructor/src/arch_walker.rs`
  - B ID：`85752a74963b34303b`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl`
  - 原路径：`crates/systems/wall-constructor/src/construct_arches/profile.rs`
  - B ID：`3d17976d7098f2d43c`
  - 候选源码：—

### 擦除、删除归档与历史

- **多候选** `country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer::contains`
  - 原路径：`crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs`
  - B ID：`370457b88d2667efc3`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs:52`<br>`tiny-glade/crates/country-core/src/resources/walls/inner_walls.rs:99`
- **多候选** `country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer::is_empty`
  - 原路径：`crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs`
  - B ID：`4c1422749c3083fa8f`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs:49`<br>`tiny-glade/crates/country-core/src/systems/wall/eraser/archivist.rs:114`<br>`tiny-glade/crates/country-core/src/systems/wall/wall_state/path_preparation.rs:56`<br>`tiny-glade/crates/country-core/src/systems/wall/construction/segmentation.rs:80`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:111`
- **多候选** `country_core::systems::wall::eraser::archivist::ChildWalls::extend`
  - 原路径：`crates/country-core/src/systems/wall/eraser/archivist.rs`
  - B ID：`be023c030aa34f8e51`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/eraser/archivist.rs:109`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes.rs:32`
- **多候选** `country_core::systems::wall::eraser::archivist::ChildWalls::is_empty`
  - 原路径：`crates/country-core/src/systems/wall/eraser/archivist.rs`
  - B ID：`a436144460e39c926d`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs:49`<br>`tiny-glade/crates/country-core/src/systems/wall/eraser/archivist.rs:114`<br>`tiny-glade/crates/country-core/src/systems/wall/wall_state/path_preparation.rs:56`<br>`tiny-glade/crates/country-core/src/systems/wall/construction/segmentation.rs:80`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:111`
- **未匹配** `country_core::resources::history::wall_compound_edit::CompoundEditBuilder::wall_deleted`
  - 原路径：`crates/country-core/src/systems/wall/deferred_wall_history.rs`
  - B ID：`f1c33841bed9883364`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::eraser::archivist::WallEraserArchivist::record_erase_original_wall`
  - 原路径：`crates/country-core/src/systems/wall/eraser/archivist.rs`
  - B ID：`60b521ddcbc88c3830`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::eraser::ui_bulldozer::can_reconnect_stairs_if_midpoint_deleted`
  - 原路径：`crates/country-core/src/systems/wall/eraser/ui_bulldozer.rs`
  - B ID：`d4a6ecfe740d456057`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims`
  - 原路径：`crates/country-core/src/systems/wall/eraser/wall_eraser.rs`
  - B ID：`ba35a317c0742e6705`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty`
  - 原路径：`crates/country-core/src/systems/wall/eraser/wall_eraser.rs`
  - B ID：`12c4527ccbdb6ee4da`
  - 候选源码：—
- **未匹配** `country_core::systems::wall::eraser::wall_eraser::wall_eraser`
  - 原路径：`crates/country-core/src/systems/wall/eraser/wall_eraser.rs`
  - B ID：`6bf4fe7d666f619c93`
  - 候选源码：—

### 材料统计与颜色

- **多候选** `country_core::resources::walls::material_stats::MaterialStats::add_stairs_length`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`eda0ce3b507829c691`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/construction/segmentation.rs:437`<br>`tiny-glade/crates/country-core/src/resources/walls/material_stats.rs:81`
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::arches_length_delta`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`6eb5f4df21952e4a63`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::bricks_length_delta`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`4f565b605b52e0d59c`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::has_material_switched`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`d95859a28808aa8c31`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::latest_update_material_proportions`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`6ca0d2c9f871bfb3e0`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::path_segmentation_delta`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`745f75c2c1b4784ba7`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::sewer_length_delta`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`0b02a4bf95c1996def`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::stairs_length_delta`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`0eb36eff416c6b8062`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::HighlightColor::to_rgb`
  - 原路径：`crates/country-core/src/resources/walls/mod.rs`
  - B ID：`371ca8c2aa866ed80e`
  - 候选源码：—

### 柱、塔柱、托架与高差支撑

- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::arches_pillar_delta`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`ba587975fd72eab0b8`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::pillar_count`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`b9dc003ec1b8a64f35`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::set_pillar_placements`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`9c167867184269436b`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer`
  - 原路径：`crates/systems/wall-constructor/src/construct_elevation_supports/circle_bottom_layer.rs`
  - B ID：`7ebd74758ca35e2ca8`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret`
  - 原路径：`crates/systems/wall-constructor/src/construct_elevation_supports/circle_turret.rs`
  - B ID：`952d7dd024017bdd4a`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets`
  - 原路径：`crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_large_brackets.rs`
  - B ID：`0611fd513550291845`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars`
  - 原路径：`crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_pillars.rs`
  - B ID：`d43518ec769f542a40`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets`
  - 原路径：`crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_small_brackets.rs`
  - B ID：`6286525e16ed95c42f`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns`
  - 原路径：`crates/systems/wall-constructor/src/construct_elevation_supports/util_pillar_construction.rs`
  - B ID：`503cd5821d6e0df9f1`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals`
  - 原路径：`crates/systems/wall-constructor/src/lib.rs`
  - B ID：`3604408386f1c69919`
  - 候选源码：—
- **未匹配** `system_wall_constructor::place_pillars::emit_pillar_at_point`
  - 原路径：`crates/systems/wall-constructor/src/place_pillars.rs`
  - B ID：`cb22d43b35c0796a72`
  - 候选源码：—
- **未匹配** `system_wall_constructor::place_pillars::place_pillars`
  - 原路径：`crates/systems/wall-constructor/src/place_pillars.rs`
  - B ID：`b5e7a555605c2db822`
  - 候选源码：—

### 洞口、门、窗、栅栏与屋顶窗

- **多候选** `country_core::resources::walls::wall_holes::FinalizedWallHoles::clear`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`7690445bcca6f9dec9`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/curved_halftimber.rs:24`<br>`tiny-glade/crates/country-core/src/resources/walls/wooden_gates.rs:20`<br>`tiny-glade/crates/country-core/src/resources/walls/plaster.rs:42`
- **多候选** `country_core::resources::walls::wall_holes::FinalizedWallHoles::is_empty`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`ccecd83353b6925c73`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs:49`<br>`tiny-glade/crates/country-core/src/systems/wall/eraser/archivist.rs:114`<br>`tiny-glade/crates/country-core/src/systems/wall/wall_state/path_preparation.rs:56`<br>`tiny-glade/crates/country-core/src/systems/wall/construction/segmentation.rs:80`<br>`tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:111`
- **多候选** `country_core::resources::walls::wall_holes::HoleChecksum::add`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`0c5bdc44cc7e9835e8`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/eraser/bulldoze.rs:84`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes.rs:28`<br>`tiny-glade/crates/country-core/src/resources/walls/freeform_height_cache.rs:28`<br>`tiny-glade/crates/country-core/src/resources/walls/height_cache.rs:50`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes/geometry.rs:58`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes/geometry.rs:161`
- **多候选** `country_core::resources::walls::wall_holes::InProgressHoleStorage::add`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`e5311ccd0436ea7a90`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/eraser/bulldoze.rs:84`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes.rs:28`<br>`tiny-glade/crates/country-core/src/resources/walls/freeform_height_cache.rs:28`<br>`tiny-glade/crates/country-core/src/resources/walls/height_cache.rs:50`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes/geometry.rs:58`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes/geometry.rs:161`
- **多候选** `country_core::resources::walls::wall_holes::InProgressHoleStorage::extend`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`d9cc9e6046d774e7e8, e2a6dc2abd6019f78f`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/eraser/archivist.rs:109`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes.rs:32`
- **多候选** `country_core::resources::walls::wall_holes::WallHoleStorage::clear`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`ea8ba56ff30bf5038f`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/curved_halftimber.rs:24`<br>`tiny-glade/crates/country-core/src/resources/walls/wooden_gates.rs:20`<br>`tiny-glade/crates/country-core/src/resources/walls/plaster.rs:42`
- **多候选** `country_core::resources::walls::wall_holes::WallHoleStorage::finalize`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`6221445b10a67cca00`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/wall_holes.rs:47`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes/geometry.rs:173`
- **多候选** `country_core::resources::walls::wall_holes::WallHoles::add`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`dcdb4c8309d9a096d9`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/eraser/bulldoze.rs:84`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes.rs:28`<br>`tiny-glade/crates/country-core/src/resources/walls/freeform_height_cache.rs:28`<br>`tiny-glade/crates/country-core/src/resources/walls/height_cache.rs:50`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes/geometry.rs:58`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes/geometry.rs:161`
- **多候选** `country_core::resources::walls::wall_holes::WallHoles::extend`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`3f1eaf1450a580849d`
  - 候选源码：`tiny-glade/crates/country-core/src/systems/wall/eraser/archivist.rs:109`<br>`tiny-glade/crates/country-core/src/resources/walls/wall_holes.rs:32`
- **多候选** `country_core::resources::walls::wooden_gates::Gates::clear`
  - 原路径：`crates/country-core/src/resources/walls/wooden_gates.rs`
  - B ID：`27b3443b0973d734f7`
  - 候选源码：`tiny-glade/crates/country-core/src/resources/walls/curved_halftimber.rs:24`<br>`tiny-glade/crates/country-core/src/resources/walls/wooden_gates.rs:20`<br>`tiny-glade/crates/country-core/src/resources/walls/plaster.rs:42`
- **多候选** `system_wall_constructor::construct_gates::segment_length`
  - 原路径：`crates/systems/wall-constructor/src/construct_gates/mod.rs`
  - B ID：`a65ddf739d0e79ff59`
  - 候选源码：`tiny-glade/crates/systems/timberframe/src/timberframe_rectangle.rs:32`<br>`tiny-glade/crates/utils/src/curve/mod.rs:174`<br>`tiny-glade/crates/utils/src/curve/mod.rs:187`<br>`tiny-glade/crates/utils/src/curve/mod.rs:197`
- **未匹配** `country_core::resources::walls::dormers::DormerHoleSsbo::cottage`
  - 原路径：`crates/country-core/src/resources/walls/dormers.rs`
  - B ID：`6efa38d176370c6769`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::dormers::DormerHoleSsbo::gothic`
  - 原路径：`crates/country-core/src/resources/walls/dormers.rs`
  - B ID：`26530848398bea5245`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::fence_length_delta`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`1d65548837bb7ac740`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::gates_length_delta`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`4d44ecbe9b94ade22c`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::has_fence_seed_changed`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`eb87b843c302f40c7f`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`cf39f25ca2f8cbf00a`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`6281545332dcb42584`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_holes::FinalizedHoleStorage::inner`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`2fbdb9c36e0d361afb`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_holes::FinalizedWallHoles::finalize_from`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`858584ad5fec38cd55`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_holes::FinalizedWallHoles::holes_no_duplicates`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`23bf685a6f128fd290`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_holes::InProgressHoleStorage::holes`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`0a612c5ad41944f89e`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`dede0169df4ece54ee`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::wall_holes::WallHoleStorage::as_wip_mut`
  - 原路径：`crates/country-core/src/resources/walls/wall_holes.rs`
  - B ID：`5fd6d5e92e33943110`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_fences::process_segment::place_fence_piece`
  - 原路径：`crates/systems/wall-constructor/src/construct_fences/process_segment.rs`
  - B ID：`25b110a15fe8bfbc0a`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_gates::DoorMeshInProgress::add_quad`
  - 原路径：`crates/systems/wall-constructor/src/construct_gates/mod.rs`
  - B ID：`f5fd9ecf8c94a99f2a`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_gates::DoorMeshInProgress::into_mesh`
  - 原路径：`crates/systems/wall-constructor/src/construct_gates/mod.rs`
  - B ID：`d4b5bf6c7d75101cfb`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_gates::door_arch_heights`
  - 原路径：`crates/systems/wall-constructor/src/construct_gates/mod.rs`
  - B ID：`e321cce130a81ec864`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_gates::door_bottom_curve`
  - 原路径：`crates/systems/wall-constructor/src/construct_gates/mod.rs`
  - B ID：`e008aa95052aa07352`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_gates::door_mesh`
  - 原路径：`crates/systems/wall-constructor/src/construct_gates/mod.rs`
  - B ID：`52a200989390c0da8e`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_gates::door_mesh_gap`
  - 原路径：`crates/systems/wall-constructor/src/construct_gates/mod.rs`
  - B ID：`88fe7c713ec25383e7`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_gates::offset_door_bottom_curve`
  - 原路径：`crates/systems/wall-constructor/src/construct_gates/mod.rs`
  - B ID：`31bda0440fff505d0c`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_gates::place_brick_steps`
  - 原路径：`crates/systems/wall-constructor/src/construct_gates/mod.rs`
  - B ID：`b9d56e79dbd56b0bb0`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_gates::place_ladder`
  - 原路径：`crates/systems/wall-constructor/src/construct_gates/mod.rs`
  - B ID：`ba404ad1f3f90d07a5`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_gates::systems::construct_sewer_bars`
  - 原路径：`crates/systems/wall-constructor/src/construct_gates/systems.rs`
  - B ID：`1a6ad7bdef29c21d99`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_gates::systems::is_sewer_gate`
  - 原路径：`crates/systems/wall-constructor/src/construct_gates/systems.rs`
  - B ID：`eb9fc533dc9e94211c`
  - 候选源码：—
- **未匹配** `system_wall_constructor::plaster_systems::mirror_holes_as_needed`
  - 原路径：`crates/systems/wall-constructor/src/plaster_systems.rs`
  - B ID：`7f6d4a9b57b0ae81e5`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows`
  - 原路径：`crates/systems/wall-constructor/src/utils/half_timber.rs`
  - B ID：`2a41cdfa3fa1b7d580`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::half_timber::get_window_aabb`
  - 原路径：`crates/systems/wall-constructor/src/utils/half_timber.rs`
  - B ID：`652cf4798b221941dd`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::half_timber::overlapping_windows`
  - 原路径：`crates/systems/wall-constructor/src/utils/half_timber.rs`
  - B ID：`9a0c3f3e29e91b758c`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::resolve_hole_overlap::check_for_hole_overlap`
  - 原路径：`crates/systems/wall-constructor/src/utils/resolve_hole_overlap.rs`
  - B ID：`44314835a166698187`
  - 候选源码：—
- **未匹配** `system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap`
  - 原路径：`crates/systems/wall-constructor/src/utils/resolve_hole_overlap.rs`
  - B ID：`24256256bb7e71bedc`
  - 候选源码：—
- **未匹配** `system_wall_constructor::wall_constructor::WallConstructor::normalize_holes`
  - 原路径：`crates/systems/wall-constructor/src/wall_constructor.rs`
  - B ID：`1c0ce01caf2e72a149`
  - 候选源码：—
- **未匹配** `system_wall_constructor::wall_constructor::split_brick_by_holes`
  - 原路径：`crates/systems/wall-constructor/src/wall_constructor.rs`
  - B ID：`26967357b0d7ba9e63`
  - 候选源码：—

### 过梁与承重梁

- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::lintels_length_delta`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`44b8190bd102024e36`
  - 候选源码：—
- **未匹配** `country_core::resources::walls::material_stats::MaterialStats::lintels_pillar_delta`
  - 原路径：`crates/country-core/src/resources/walls/material_stats.rs`
  - B ID：`c0110b283dfc81becc`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::accept`
  - 原路径：`crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs`
  - B ID：`ed62dcae79b5d5e718`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::propose`
  - 原路径：`crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs`
  - B ID：`65eb996ff81a572bc4`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams`
  - 原路径：`crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs`
  - B ID：`38ae67c36ed64d859a`
  - 候选源码：—
- **未匹配** `system_wall_constructor::construct_lintels::profile::wooden_lintel_profile`
  - 原路径：`crates/systems/wall-constructor/src/construct_lintels/profile.rs`
  - B ID：`f9439a3c0e1b476cc7`
  - 候选源码：—

## 显式空体或部分占位

| 函数 | 状态 | 原路径 | 处理要求 |
|---|---|---|---|
| `country_core::resources::walls::decorator_storage::DecoratorStorage::verify_integrity` | `empty_body_review` | `crates/country-core/src/resources/walls/decorator_storage.rs` | 空体需原证确认是否原版无操作。 |
| `country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl` | `partial_stub` | `crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_height_impl.rs` | 含显式未完成分支，必须对照原算法补全。 |
| `system_wall_constructor::debug_force_update_arches` | `empty_body_review` | `crates/systems/wall-constructor/src/lib.rs` | 台账记录原版为 1B 返回；仍需保留原证再裁定，不能只按空体判缺失。 |

## 审计要求

1. 逐项确认当前实现、原版函数、输入输出与失败路径，不凭同名或非空函数体判完成。
2. 多候选必须按类型 owner、原路径、调用者和 ABI/schema 确定唯一映射。
3. 未匹配先检查改名、跨 crate 迁移、trait/generic 单态化；确认缺失后再补算法。
4. 算法恢复后仍须单独验证生产接线、历史、碰撞、GPU、保存恢复和卸载。
5. 本清单的分类与数量不能替代调用链工程完成度。

## 2026-10-30 保存与装饰 ID 链静态复核（未验收）

本机未发现状态台账所引 `tmp-merge` 留存产物，不能复用其历史运行结果。当前源码的墙色检查在父进程完成真实保存后，以完整 `SavedMultiEdit` wire 比较运行态与磁盘；子进程 Reload 后再比较运行态、同一磁盘和父 wire，并分别核对 cursor、history_start。另有基于 `serde_json::Value` 的完整父子比较；该路径经过数值扩展和重新解析，不能仅凭旧日志认定真实 f32 位模式不同。须保留实际首差和两侧 f32 位型后才可归因；不得更改期望或放宽断言。

快照加载实际通过 `restore_snapshot_with_id_gen` 逐个 `reserve` ID（`decorator.rs` 注释记录 B `0x14155c4e1` 的 64 位加比较后截断）；完整生产 `decorators_on_glade_unload` 注册于 Game 的 OnExit 并重置存储和生成器。`system-decorator` 中未注册的 scoped helper 不是生产入口。真实冲突仍待通过已有 `M6_ID` 输出核对加载、模式切换及放置各阶段的 next 与存储，不以静态阅读宣称修复。

中央串行独立运行入口（从 `tiny-glade/`，`TG_INSTALL_ROOT` 指向本机完整 B 安装目录，Vulkan/GPU 和原版资产须可用）：

```sh
TG_INSTALL_ROOT='<B安装目录>' TG_M6_DUMP_DIR='/private/tmp/m6-persistence-diagnostics' CARGO_BUILD_JOBS=1 cargo test -p tiny-glade --test original_gameplay_chain m06_real_wall_color_picker_history_save_new_process -- --exact --nocapture --test-threads=1
```

装饰场景相同命令将测试名替换为 `m06_real_decorator_collision_picker_opens_matching_menu`。每项独立进程，保留父存档、预期和首差文件；本次未执行 Cargo、未做原侧动态差分，也未证明真实 OS 输入或完整卸载重入。

中央随后实际运行 `m06_real_wall_color_picker_history_save_new_process`（`/private/tmp/m6-color-fullchain-20261030.log`，159.72 秒，失败）：父运行态与磁盘 typed history 检查通过，子运行态第 0 项 `/edits/0` 缺失父侧 `Tree::Place`。此次构建早于新 early 事件维护修复；日志未留存 `TG_M6_DUMP_DIR` 快照，不能据此比较后续项。

静态追踪发现唯一直接首插历史的生产者 `convert_preplaced_trees` 即便接受树集合为空，也调用原 `History::insert_starting_edit`，插入空 `MultiEdit` 并增 cursor；旧存档迁移发出 `SpawnPreplacedTrees`。这一机制能解释首差，但本次未记录调用，尚不能确认因果。现给子测试补充复制目的文件与加载状态、首项编辑数诊断，并比较实际目的文件；待中央使用新源码及 early 事件修复串行复验，不更改原插入语义。

共享 B Ghidra 只读定点复核 `convert_preplaced_trees@14226be70`：反编译行 122–128 显示事件触发设置 Local 标志，计数超过 2 时先清标志，再读取 mask（行 152–169）。当前重构在新 GPU readback 尚不可用时保留标志并等待；两者生命周期有明确差异。该差异可能使旧会话转换跨 Reload 延迟到新历史，但本次尚无实际调用时间线，不能直接当作根因；涉及 tree-tool 的修复须由对应 owner 按 mapped buffer 原语义处理，不能简单去掉必要数据或伪造 readback。

v4 `/private/tmp/m6-color-fullchain-v4-20261030.log` 实际确认候选：子 intro 在 submitted=0 请求旧转换；目标 v27 存档 Reload 后 cursor=14、15 项、首项 281 条，随后旧转换等待101帧、readback_frame=1 时写入新 ledger，首项变空。完整父存档及参数保留 `/private/tmp/m6-retained-20261030/wall-color/run-NZoce9`。当前候选把异步等待绑定到请求时的 session 和 Game 状态变更 tick：跨退出／重入取消过期任务，同代真实请求仍按已恢复路径读取真实 mask、执行转换；保留必需 mask owner 的失败检查。该代际边界是异步宿主适配，不冒称原 ABI 字段；原版两帧后直接取映射及 missing map panic 的跨线程等价仍需另证。新增同 session Reload 期间延迟回读回归，尚待中央运行。

中央对 v4 保留存档重跑 child：`/private/tmp/m6-color-child-epoch-v1-20261030.log` 1/1 通过（30.83 秒），Reload 后完整 typed history、cursor、颜色/GPU和恢复后撤销重做通过；旧转换不再写入新 ledger。原始 JSON 报告的一个 f64 末位差经过现有完整 typed 比较一致，未放宽断言。此结果是旧父存档+新 child 的组合验证，不代表新源码完整父子链或同 session 回归已运行。

已补 `original_preplaced_startup` 两项回归，分别覆盖同一 session 的 Reload 和切换 session；均检查旧等待不写新 ledger，以及新事件仍触发一次转换。测试使用现有真实系统和回读发布接口，尚待中央运行。`git diff --check` 通过，不替代 Cargo 或完整父链验收。
