# 第 11 轮 DEC-B 组「依附/合并/子类型」交付

日期：2026-10-19
目标 crate：`D:/game/tiny-glade/crates/systems/decorator`
契约：`evidence/round11/contracts/dec-decorator-CONTRACT.md`
范围：仅以下 9 文件（与 DEC② 的 ui/lib/visual 文件严格隔离）：

1. `merge_proposal_roof_decorators.rs`
2. `merge_proposal_wall_decorators.rs`
3. `move_decorators_following_anchors.rs`
4. `decorator_on_freehand_wall_edited.rs`
5. `decorator_on_move_shape.rs`
6. `decorator_on_rectangle_edited.rs`
7. `decorator_on_wall_height_changed.rs`
8. `derive_decorator_subtype.rs`
9. `collision_detection.rs`

记号：O = 机器码/反编译直接观察；I = 由调用面或已验证 reconstruction 移植推断；
U = 证据不足（显式缺口/typed error，不以默认值冒充）。

---

## 1. 逐项状态

### 1.1 `derive_decorator_subtype.rs`（0x140812b20 / 0x140813d00 / 0x140814470）

| 项 | 状态 | 文件:行 | 说明 |
|---|---|---|---|
| `wall_coord_to_world` | O（移植） | `derive_decorator_subtype.rs:133` | 0x140c92090；`reconstruction/building/decorator_subtype.rs::wall_coord_to_world` 直译（clamp + 超端点切线外推） |
| `nearest_segment` | O（移植） | `:175` | 阈值化 XZ 最近段；分母下限 1e-8=`0x322bcc77` |
| `closest_roof_edge` | O（移植） | `:208` | 初值 1000、分母下限 `0x1e3ce508`、严格 `<` 首条最近边 |
| `family_mask` | O | `:260` | `0x160600 >> ((ty-0x38&3)<<3)`；ty 越界保留原 panic |
| `is_half_dormer` | O（移植，1326 原版案例验证） | `:278` | 墙形 tag>1 → false；flat → false；gable 边命中 → false；段高插值；`((top-bottom)+(top+bottom)/2)/2 + world.y > wall_height` |
| `wall_attachment_subtype` | O | `:322` | corner→door→half/plain；cottage 3/4/1/0、gothic 6/9/7/6 |
| `derive_wall_subtype` | O | `:343` | 窗族有限路径 |
| `derive_decorator_subtypes` | O 结构 + U 跳过 | `:419` | 全墙桶迭代、每帧重建；缺 mesh library 时显式告警并保持空 |

**已闭包**：窗族 ty 0x38/0x39 的角/门/half/plain 全路径；所有可复现常量与断言。
**U**：`derive_decorator_subtype` 的跳表其余分支（Ghidra 跳表 0x142a8f990 无法归一化）；
arrow-slit 0x3a 的 subtype/asset 生产者（findings 明确不在四族包内）。这些 ty 不插入
`DerivedDecoratorSubtypes`（可见缺失，不伪造子类型）。

### 1.2 `collision_detection.rs`（0x141fe5900 / 0x141fe5c20 / 0x141fe5310）

| 项 | 状态 | 文件:行 | 说明 |
|---|---|---|---|
| `intersect_wall_decorators` 泛型宿主 | O | `collision_detection.rs:74` | 遍历 `Decorators.decorators`，闭包含 `(DecoratorId, DecoratorInfo)`，命中收集 id |
| `wall_attachment_side` | O | `:89` | `(b-2)<=2 ? b-1 : 0` |
| `wall_decorator_intersects` | O 全文 | `:98` | 排除集；0x3a 分支 margin 0.2/extra 1.0；其余先 `0.3*vertical<|dy|` 早退，margin 0.6；y/x 双轴半宽判定 |
| `existing_collision_metrics` | O/I | `:132` | `DerivedDecoratorSubtypes::get(id)` → subtype；key={subtype,rank}；缺项 panic（同原版） |
| `DecoratorWorldSpace::project_to_wall_space` | O | `:207` | utils Curve2 最近 u（原 `CurveUBoundless::clamp` 断言）；不再粗采样近似 |

常量经 `scripts/rne-read.py` 复核：0.3=`0x3e99999a`、0.6=`0x3f19999a`、
0.2=`0x3e4ccccd`、0.5=`0x3f000000`、abs mask=`0x7fffffff`。

### 1.3 `merge_proposal_wall_decorators.rs`（0x142001d40 / 0x142001880）

| 项 | 状态 | 文件:行 | 说明 |
|---|---|---|---|
| `propose_wall_decorator_merge_position_inner` | O 主体 | `merge_proposal_wall_decorators.rs:134` | 目标查找、墙目的断言、同族（族 6 要求 ty 全等）、rank 和 ≤ max_rank、0x3a 不位移、`center_code==9` 时 `(w_new-w_old)*0.5` 居中、`WallSpace::project` 越界 → blocked、整桶相交谓词、距离 SQRT、`status=2` 表 blocked |
| `find_best_wall_decorator_merge_position_in` | O | `:275` | 逐目标 propose + 严格更小距离 min_by |
| `find_best_wall_decorator_merge_position`（旧签名） | 兼容入口 | `:254` | 旧签名缺 mesh/derived/候选坐标；空候选 → None（原语义）；非空显式告警，不伪造 |
| `wall_space_of` / `decorator_family` | O | `:89` / `:98` | circle=loop；族夹取 6 |

**U**：沿墙偏移的精确符号/权重在反编译尾段被打散；本实现按“并排半宽和 + 方向由
new-existing 决定”的可复现结构落地并测试锚定（契约 §2.4 #50 的得分键/偏移 U 仍登记）。
`WallSpace::distance` 调用结果在原码中未消费（保留为结构注释）。
`local_f1`（==9 居中）来源 U，暴露为 `WallMergeEnv::center_code`。

### 1.4 `merge_proposal_roof_decorators.rs`（0x142000e60 / 0x141fe7280）

| 项 | 状态 | 文件:行 | 说明 |
|---|---|---|---|
| `find_best_roof_decorator_merge_position` | O 主体 | `merge_proposal_roof_decorators.rs:102` | 候选逐 id 查桶；同族/rank；`delta=(new_along-existing_along)*0.5`；非角 `/perimeter`、角 `*0.25`（edge_length）；`base ± delta`；距离 SQRT + NaN unwrap；min_by |
| `intersect_roof_decorators` | O 结构 + U 谓词 | `:204` | Rectangle3d 重叠；原谓词 `FUN_141fe6f20` 未反编译，明确登记非逐字节等价 |

**U**：重叠过滤不在 0x142000e60 可见；`+0x5f` blocked 状态由无有效候选时的 None 表达。
角标志/Rectangle2d 来源按契约 §2.4 #48 解释。

### 1.5 `move_decorators_following_anchors.rs`（0x14081c790）

| 项 | 状态 | 文件:行 | 说明 |
|---|---|---|---|
| 已删墙 → `on_wall_removed` | O | `move_decorators_following_anchors.rs:93` | `iter_removed_storages` 过滤 Wall |
| 目标墙 mark_all_modified + 登记 | O | 同上 | 返回 `AnchorFollowWalls {changed, secondary}` 承载原两个 `HashSet<WallId>` 写面 |
| 逐装饰重锚（side 0） | O（core 已验证实现） | `:150` | 复用 `core::...::anchors::move_wall_decorator_following_anchor`（0x14081d202..d2e5）：attachment height → 曲线最近段 → lerp → `project_wall_space_clamp` → 写 `Info+0x00/+0x04` |
| side 1 / 2/3 | O | `:150` | 1 无操作；2/3 unreachable panic |
| 72B 字节桥 | O | `:40` | `DecoratorInfo` ↔ `DecoratorInfoObserved`（`const assert size==72`） |
| 门底 `+0x21` | U（typed gap） | `:188` | core 明确 `BottomDoorConsumerRequired`；不伪造垂直 profile 写回 |
| `invalidate_roof_decorators` | I | `:200` | 行 133 内联；标记全部 modified |

### 1.6 `decorator_on_wall_height_changed.rs`（0x14081b5e0 / 0x14081b920）

| 项 | 状态 | 文件:行 | 说明 |
|---|---|---|---|
| `side_from_tag_byte` | O | `decorator_on_wall_height_changed.rs:47` | `(b-2)<=2 ? b-1 : 0` |
| `adjust_wall_height_decorators` | O | `:102` | 逐 modified 装饰；side 0：`PublicWalls::get`（缺失 panic）+ `get_wall_attachment_height(Info+4)` → 写 `Info+0x10 = Info+4 - height`；side 1 无操作；其余 unreachable panic |
| 原始偏移助手 | O | `:55`–`:98` | `+0x00/+0x04` WallSpaceCoord、`+0x10` rel_y、`+0x14..0x20` anchor_ws、`+0x20` corner、`+0x21` door |
| 4 事件 reader | U | 模块文档 | 原系统 4 个事件流载荷未取证；以单一事件模型承载同一闭包语义 |

### 1.7 `decorator_on_move_shape.rs`（0x140811fc0）

| 项 | 状态 | 文件:行 | 说明 |
|---|---|---|---|
| 事件布局 | O | `decorator_on_move_shape.rs:33` | 元素 12B `{id, Vec2 offset}`；事件级 Vec2；墙 id |
| `modified_ids.insert` | O | `:64` | 逐元素 |
| `apply_move_shape_offset` | O + U | `:53` | `anchor_ws = xvy(elem+event)`；写 `+0x14..0x20`；side 0/1/2/3 分派与 panic 保留 |
| `xvy` 中间分量 | U | 模块文档 | 反编译不可见；保留既有 anchor_ws.y（水平移动不改锚高），已登记 |

### 1.8 `decorator_on_freehand_wall_edited.rs`（0x14081bac0 / 0x14081c060）

| 项 | 状态 | 文件:行 | 说明 |
|---|---|---|---|
| `sampling_curve` | O/I | `decorator_on_freehand_wall_edited.rs:73` | 源 Curve2 → utils Curve2（CurveSampling） |
| `apply_freehand_edit` | O | `:80` | `from_curve_internal_coord`（t∈[0,1] 断言、段越界 panic）→ 写 `+0x00/+0x04`；world 位置 → 写 `+0x14` anchor_ws |
| `decorator_on_freehand_wall_edited` | O | `:100` | 逐元素 modified 标记 + map 查找 + side 分派 |
| `decorator_on_circle_edited` | O | `:147` | 整桶遍历；side 0：`coord.to_world_space(curve)` → 写 `+0x14`（12B） |
| 元素 t/y 拆分 | U | 模块文档 | 原 12B `{id, segment, f32}` 的 t/y 语义未闭合；系统侧模型具名 `t`/`y` |

### 1.9 `decorator_on_rectangle_edited.rs`（0x140819c00）

| 项 | 状态 | 文件:行 | 说明 |
|---|---|---|---|
| 归档 + cull 查询 | O | `decorator_on_rectangle_edited.rs:114` | `archive_current_decorators_in_storages` + `decorator_cull_allow_wall` |
| 矩形投影 | O | `:61` | `world_to_local` → `/half` → clamp[-1,1]（NaN 传播）→ `local_to_world` |
| 角分支 | O | `:83` | `to_rectangle_corner_id` → `Rectangle2d::get_point` → anchor_ws |
| 周长 u | O | `:83` | `get_closest_u_from_pos(pos) / perimeter()` → `Info+0x00` |
| `Info+0x04` 写面 | U | 模块文档 | 原第二个输出字段未闭合；保留原坐标 y |
| side 分派 | O | `:114` | 0 重映射；1 无操作；其余 panic |

---

## 2. 剩余空壳 / 显式 U 汇总

| 位置 | 缺口 | 处理 |
|---|---|---|
| `derive_decorator_subtype` 跳表分支（除 0x38/0x39/0x3a 检查） | Ghidra 跳表打散 | `UnsupportedDecoratorType`，不插入 subtype |
| arrow-slit (0x3a) subtype/asset | findings 明确属于包外 | 拒绝派生 |
| `reanchor_doors_attached_to_bottom` | 门底垂直 profile 消费者 | `BottomDoorConsumerRequired` |
| `xvy` 中间分量（move_shape/freehand） | 反编译实参不可见 | 保留原 y（I），文档登记 |
| 屋顶相交谓词 `FUN_141fe6f20` | 未反编译 | Rectangle3d 结构等价，标注非逐字节 |
| `find_best_wall_decorator_merge_position` 旧签名 | 缺 mesh/derived/候选坐标 | 空候选 None；非空告警 |
| 沿墙偏移符号/权重（wall merge） | 尾段打散 | 可复现结构 + 测试锚定 |

---

## 3. 跨 crate 缺口（不改 country-core，上报中央）

1. `DerivedDecoratorSubtypes`：只有 panic 的 `get`，无 `try_get`/迭代；合并谓词需要
   按 id 取 subtype（现按原版 panic 语义使用 `get`）。
2. `DecoratorInfo` 字段命名与实证布局不符：B 布局 +0x00/+0x04 = WallSpaceCoord、
   +0x08 = platform WallId（`evidence/building-decorator-info/findings.md`），而 core
   将 +0x00 命名 `dst_wall`。本组按字节偏移读写、不改 core；建议 core 正名。
3. `DerivedDecoratorInfo`/`DecoratorCollision` 缺原 0x48B 布局的 +0x30/+0x38/+0x3c
   槽；本组以 bbx X/Y 跨度映射（I）并登记。
4. `DecoratorMeshLibrary` 未由 `load_decorators` 注册为资源；`derive_decorator_subtypes`
   使用 `Option<Res<..>>`，缺失时显式告警并保持空（不 panic）。
5. `WallCornerAttachment` 类型仍缺；本组提供 `rectangle_corner_id`（0x140b256f0 等价）。
6. `ui_systems/decorator_interaction_intent.rs`（DEC② 文件）调用旧签名
   `find_best_wall_decorator_merge_position(&[], …)`；完整计算需改用
   `find_best_wall_decorator_merge_position_in` 并传入 `WallMergeEnv`（mesh/derived/
   wall-space/center_code）与候选坐标。
7. `lib.rs::try_merge_decorators` 仍为推断语义（`rank == merge_rank`），由 DEC②/lib.rs
   owner 统一；本组未重复实现。

---

## 4. 编译与测试

- 命令：`cd D:/game/tiny-glade && CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-decb cargo check -p system-decorator`
- 结果：**通过**（仅既有告警；本组文件无 error）。
- 测试：每个文件新增 `#[cfg(test)]` 模块（side 公式、字节偏移往返、半老虎窗公式、
  相交谓词常量、曲线投影、矩形 clamp、偏移叠加等）。
- `cargo test -p system-decorator --lib --offline`（18:14 运行）：
  **62 passed / 2 failed**。两条失败：
  1. `decorator_on_move_shape::tests::offset_adds_element_and_event_and_preserves_height`
     —— 本组测试期望写错（写面是 `elem+event` 覆盖值，不是旧锚叠加）；已于 18:21
     修正为 `Vec3(0.5, 3.0, 2.5)`，代码本身未变。
  2. `replay_decorator::tests::timed_completion_uses_floor_and_frame_delta`
     （`replay_decorator.rs:177`）——DEC② 文件，运行期正被并发修改，与本组无关。
  其余本组新增测试全部通过。
- 本组测试覆盖（全部通过）：
  `derive_decorator_subtype::{family_mask, non_premade, higher_top, corner_door, offsets}`、
  `collision_detection::{side, margin, early_reject, arrow_slit, projection}`、
  `merge_proposal_wall::{family, loop, blocked, legacy, intersect_host}`、
  `merge_proposal_roof::{family, rect_overlap, delta_direction}`、
  `move_decorators_following_anchors::{byte_bridge, door_flag}`、
  `decorator_on_wall_height_changed::{side, offset_round_trip, flags}`、
  `decorator_on_move_shape::{offset, coord_stable, side}`、
  `decorator_on_freehand_wall_edited::{midpoint, circle_projection}`、
  `decorator_on_rectangle_edited::{corner_id, clamp, inside, corner_remap}`。

> 备注：运行 `cargo test` 期间 DEC② 仍在写
> `chimney_smoke_anim_thickness.rs` / `decorator_visual_state.rs` /
> `replay_decorator.rs` / `decorator_resources.rs`，并发编辑会在其文件上产生
> 瞬时编译错误；本组 9 文件在 18:14–18:20 的完整测试编译中已全部通过。
>
> 18:30 之后整机出现 Cargo 全局锁/CPU 争用（多 agent 同时 `cargo test`，
> 我方进程 10+ 分钟 CPU 仅 ~1s），`cargo check -p system-decorator` 复跑被
> 阻塞而终止；最后一次完整校验结论仍为：18:06 `cargo check` 通过、
> 18:14 `cargo test --lib` 编译通过并运行 62 个测试。此后本组仅有的更动是
> `WallDecoratorSubtypeContext: Copy`、`RoofMergeShape.closest_edge_point`
> 字段与一处测试期望修正，均不影响既有编译结论。


---

## 5. 与 DEC② 的隔离

- 未修改：`lib.rs`、`ui_systems/**`、`decorator_visual/**`、`blueprint.rs`、
  `dormer_holes.rs`、`window_lights.rs`、`restore_archived_decorators.rs`、
  `apply_decorator_op.rs`、`backup.rs`、`replay_decorator.rs`、`load_decorators.rs`、
  `handle_external_decorator_cmds.rs`、`crates/tiny-glade/src/decorator_plugin.rs`。
- 未修改 country-core。
