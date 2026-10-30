# M01 独立业务复核：真实注册、拒绝恢复与生命周期

日期：2026-10-27。审计基线 main@858da5891 的工作树；其他 owner 仍在同步修改。仅复核本轮 M01，未按统计资料推算覆盖率，未重复 Cargo、未进行 OS 输入或 GPU 验收。本审计不编辑其他 owner 业务源。

## 依据与范围

先对照 `reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/systems`：freehand/move 的真实归属是 `ui_edit_freehand_walls.rs`、`ui_move_freehand_walls.rs`、`ui_move_shapes.rs`，高度属于 `wall/adjust_wall_height/`，不是另造 `shapes/freehand` 等目录。复核事件 producer → canonical event → authority consumer → 历史/缓存/可视化 → 卸载；源文件存在或单元测试手动调用不等于生产注册。

原行为来源是发行 `B版本内容/tiny-glade.rne`，SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。Ghidra 用于 caller/构造器/SystemParam 身份，跳表和函数范围以发行机器码、PDB 范围交叉检查。旧工具的物理 callee 名不能单独作为系统身份。

## 发现与处理状态

### P1：生产缓存/可视化注册缺失，排序目标只是未注册系统的集合

审计快照：`tiny-glade/src/run_game.rs:427` 仅调用 `decorator_plugin::build_startup_resources`；`build_recovered_visual_systems` 存在但无生产 caller。手动把 `cache_decorator_transforms` 注册在测试内不能证明生产链闭合。五个新增消费者 `.before(cache_decorator_transforms)` 在 cache 未注册时只形成集合约束，不能产生缓存。

最小触发：现有墙上有装饰物，拖动形状后下一帧进入 pin/anchor/蓝图消费者；authority 已更新而 CachedDecoratorTransforms 没有真实 producer。本问题已交中央修复。旧“stairs cache 未恢复所以禁止挂载”的注释已失效：真实 `compute_stair_transform` 已有 request 匹配、缺 request 原 sentinel、quantization 和 ladder kernel；但 request assemble 整族仍不能据此宣称完成。

### P1：validate_blueprints 未生产注册，修改追踪无法按原帧边界清理

审计快照：`systems/decorator/src/blueprint.rs:814` 本体已恢复，生产宿主没有调用。该函数入口 `:839` 无条件清 Storage modification tracking，之后才判断 deferred.records。若只挂 cache/clear/generator/instantiate/copy，modified/removed 留在下一帧，会反复清空、生成 Arc mesh 并重建可视化/碰撞；不能靠另写一个清 tracking 的系统掩盖缺 validator。

本体复核：14 组 SystemParam（Derived 组内部两个资源）；原主入口 `0x140827130` 完整 539 B。Ghidra 错认 488 B，在 `1408272b5` 把间接跳转误认调用，丢掉图标分支；直接 LLVM 已补全。六项图标表对应 default@0x69、Move@0x5c、Rotate@0x60、tag4@0x5c、tag5@0x5c、exact clear 跳过。实际短路回调 `0x1412b84f0` 是 PDB 8161 B，Ghidra 2093 B body 标签和坏 CFG 都不能作为截止依据。

现源码具有 locator addr/id 查找、缺 blueprint 拒绝、真实位置验证、required subtype/cache、同墙物理 overlap、56..63 排除62 的特殊源继续、blocking 与 intended outline、全部 11 类反馈、backup.created 门、fallback 图标和 mute 事件，未发现被压成 clear-only 桩。Hash 容器迭代保留实际容器顺序，不排序反馈或预先折叠 locator。Move/Rotate 类型由 owned canonical request 投影；尚未恢复的其他 legacy dispatcher writer 不能借 accessor 声称业务已完成。

### P1：备份仍为 5 参数近似，无法支撑真实验证拒绝后的完整回滚

位置：`systems/decorator/src/backup.rs:95`、`:121`、`:187`。现 CREATE 只保存 Storage 与 deferred.records；RESTORE 只写回部分桶/调用 stairs.restore_backup 并恢复 records。三个 auxiliary 仍为无实际 owner 的字节模型；events Vec 也没有来自真实 DeferredDecoratorOpEvents 的复制。`restore_backup` 中 archivist 被 `let _ = archivist` 丢弃。

原构造器 **`0x140489a80`** SystemParam 完整明确九个 owner：

1. `Res<DecoratorArchivist>`
2. `ResMut<DecoratorStorage>`
3. `ResMut<DerivedDecoratorSubtypes>`
4. `ResMut<DecoratorBlueprints>`
5. `ResMut<CachedDecoratorTransforms>`
6. `ResMut<DeferredDecoratorOpEvents>`
7. `ResMut<DeferredDecoratorEditHistory>`
8. `ResMut<DecoratorBackup>`
9. `ResMut<StairsState>`

因此原三个 map 不再是未知：subtypes / blueprints / transforms；0x88B 事件 Vec 对应真实 DeferredDecoratorOpEvents。原 `0x1408238a0` CREATE 与 RESTORE 都有对应 map 的 clone_from 和事件 Vec 克隆；详见原反编译 `evidence/toolchain/decor-trunk/decompiled/1408238a0_create_or_restore_decorator_backup.c` 的 167/175/183/198、400/408/416/431 行。不能继续以 U 注释跳过。

原 CREATE 沿 archivist.all_decorators 定位桶并调用 tracked_clone_from；RESTORE 先 all_storages 清 tracking，再沿 all_decorators 反拷，并恢复各 map 后的两个元数据字段。不得改成无差别整 World/整 Storage clone 来替代。状态分支也有边界差异：原非0、非1在 created 门后进入 RESTORE，现 match 仅2而丢弃其他字节值；正常 validator 生产2，但仍须保留原完整门控。

最小触发：开始合法装饰编辑创建 backup，产生位置/蓝图/派生变化，validator 拒绝并写 state=2；现恢复器仅恢复部分 Storage，后续 instantiate/copy 却读到拒绝操作的蓝图和变换，事件也未回退。**即使验证器本体完整，闭环仍不完整。** 已同步中央和 move_shapes owner；本审计不自行改其源。

### P1：卸载缺失，挂可视化前须同步恢复原 owner 释放

原 `decorators_on_glade_unload@0x140347090` 完整 16+4 个资源及 Commands。早先只有 todo/无注册；move_shapes owner 现已落盘真实释放函数，中央仍需确认生产 OnExit 注册及独立通用实体卸载。重置容器、释放 Arc/稀疏实例与移除 StairsTopDownMesh 不等于只清长度；本函数不接 RaycastWorld，不能自造逐 collider 删除替代其原卸载 owner。

### P2：矩形旋转 producer 发事件后未结束枚举（已由 owner 修复）

位置：`country-core/src/systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs:251`。原 `140ad898c..8993` 写事件后直接 `140ad8997` 进入 epilogue，不继续枚举。旧源码继续下一个 corner，使非末尾 active corner 后的 inactive corner 覆盖 `ClosestCornerCache.recalculate=false`，并额外创建 widgets。owner 已在 `:259` 补 return，中文注释绑定原地址。尺寸 producer 的原循环不同，不应机械照抄 return。

## 验证器与画面链的真实顺序

原插件 `0x140342c10` 在 `1403451af` 调 parent19 构造器 `0x1400d52f0`，随后加入 **Update / GameSystemSet(25)**。parent 配置数组每项 0x70 B；原是普通 `.chain()`，不是整组 `.chain_ignore_deferred()`。索引与输出写入能从 `m01-decorator-set25-parent.asm` 逐字节回溯：

| parent slot | 已核实内容 | 构造器 / 关键地址 |
| --- | --- | --- |
| 2 | cache_decorator_transforms | 1400c6200 → 输出+0xe0 |
| 4 | derive_decorator_subtypes | 1403e9f50 → +0x1c0 |
| 5 | 普通 tuple(clear_blueprints, clear_out_visual_entities) | 1403d6580 → +0x230 |
| 6 | 普通 15 generator 组 | 140529c20 → +0x2a0 |
| 7 | instantiate_blueprint_colliders | 1403d6a80 → +0x310 |
| 8 | flush_rapier_colliders | 140254d90 → +0x380 |
| 9 | validate_blueprints | 1403d68d0 → +0x3f0 |
| 10 | cull_oob_decorators | 140104f60 → +0x460 |
| 11 | maintain_stair_graph | 140285ee0 |
| 12 | cull_stair_attached_decorators | 140105110 |
| 13 | create_or_restore_decorator_backup | 140489a80 |
| 14 | 普通 tuple(stairs 组, fullvisual/copy 链) | +0x620 |
| 15 | unleash_decorator_op_events | 1402de9d0 |
| 16 | flush_rapier_colliders | 140254d90 |
| 17 | maintain_lantern_lights | 1400e5010 |
| 18 | visualize_ui_stairs | 1403c3860 |

slot14 的第二个成员来自 **1403d6ce0**：`instantiate_blueprints → 普通 tuple4 → copy_blueprints_to_prev`，仅这三阶段用 `chain_ignore_deferred`。tuple4 包含 chimney_assemble_bricks、door_stairs_assemble_bricks、dormer roof tiles 链、upload_dormer_holes_to_gpu；内部不是强行串行四条。caller 先构造暂存在 RBP+320，随后搬到第5个输入配置；parent 再将输入尾部两个配置组成普通 tuple2 写 slot14。不能因为临时栈位置或 ctor 调用时间而错放到 slot1。slot1 实际是 ui_convert_autoclutter_to_decoclutter + apply_stairs_op_events 链。

安全投影必须保留 **cache → derive → clear → generators → collider → flush → validate → 原剔除/恢复阶段 → instantiate → tuple4 → copy → unleash → flush** 的关系。生产条件原构造器和 parent set25 添加路径均未发现额外 run_if；不得加 `resource_exists<ClutterMeshes>` 默默跳过。真实资产缺失应由现有 startup owner/require_clutter_catalog 明确暴露。record_history 的独立注册阶段不能由本19组推断，应另循 caller；本审计不臆定其必须位于 copy 前。

五 consumer 之前所属普通 tuple 与 anchors/cache 组，应依中央已核原树保留；25 个 Update 业务叶子 + 1 个 is_gameplay_active 条件不能误写成26业务系统，也不能以数量相等代替逐组顺序。

## 已排除的误报

- 同帧两个同名 signifier factory 并非必然被去重丢失：create 对已有 entity 使用帧开始快照，同帧重复 factory 原可调用两次。
- freehand move 的 OnShapeMoved 使用移动前发布曲线的 midpoint 是原 `1409781d0` 行为；不能“优化”成移动后 midpoint。
- freehand 相机用 `MainCamera.eye_forward` 对应原 CameraMatrices::eye_direction；不能替成 eye_position。
- height mode2 的 not-yet-implemented panic 是原版分支，不能当作候选空实现随意补业务。
- 原矩形边扩展、lerp、border 搜索的逐位差分已单独完成，见同目录 `m01-shared-geometry-independent-review.md`；不在这里重复跑数或放宽 NaN 契约。

## 经验

1. **恢复接线时先追 schedule config 的实际搬运，再猜执行先后。** 构造器创建时间、临时栈槽和父数组序号不相同；跟踪每个0x70B配置才能避免把 visual/copy 放在validator之前。
2. **完整 validator 仍需完整 rollback owner。** 审查不止到 state=2；继续追 backup 消费者、被保存的真实map和延迟事件，再追instantiate/copy。
3. **工具 bad CFG 不是业务缺失。** 539B与488B、8161B与2093B的差异都须以PDB范围、LLVM跳表回溯，不能截断反编译照写简化实现。
4. **生产注册不能由测试手动资源或手动cache证明。** before未注册系统只创建约束，不产生更新。
5. **已排除的异常要沉淀。** 原frame midpoint、原todo panic、同帧factory语义都应保留，避免下一轮重复“修正”成偏离原版的代码。

## 原始复核包

同目录 `m01-independent-review-rectangle-native.json`；`m01-independent-review-validator-registration-native.json`、`m01-independent-review-validator-body-neighbors-native.json`、`m01-independent-review-validator-extra-configs-native.json`、`m01-independent-review-validator-final-configs-native.json`、`m01-independent-review-validator-neighbor-configs-native.json`、`m01-independent-review-validator-predicate-native.json`；`m01-independent-review-validator-main.asm`、`m01-independent-review-validator-config.asm`、`m01-independent-review-visual-copy-config.asm`。原父表及插件caller已有 `m01-decorator-set25-parent.asm`、`m01-decorator-plugin-registration.asm`。其中Ghidra坏CFG原样保留，不能把它单独当完整oracle。

另应 height owner 请求，补存 `m01-independent-review-generators15-config-native.json` 与 `m01-independent-review-generators15-members-native.json`，明确 slot6 的普通15成员组及14个typed构造器（cottage_wall_windows在父构造器内直接inline），用于独立核生成器，不代表15个算法已经通过验收。
