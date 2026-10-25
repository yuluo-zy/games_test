# Tiny Glade 原作历史命令结构还原

证据编号：`TG-HISTORY-OBSERVED-20261010`。范围：指定发行目录下全部 33 份 `assets/starting-builds/**/history.json`。原文件只读；没有启动原作、重放编辑或创建游戏实现。以下“已观察”指实际 JSON 数据；“候选解释”需要原作消费者函数验证。

## 已还原范围和验证

已解析 2,120 个 `History.edit_chain` 条目、2,774 个外层编辑命令、13 类外层 tag，生成 689 个规范化路径的字段/类型结构及 31 个嵌套 tag 路径。每类结构保留实际来源文件和 JSON pointer，完整结果在 [observed-history-schema.json](./observed-history-schema.json)。`[]` 表示数组项，`{numeric_key}` 表示数值字符串索引的映射项。这是开放的观察模型，不能当作原作所有命令的封闭枚举。

[sample-verification.json](./sample-verification.json) 已通过：33 份文件身份复核、无损 JSON 往返（保留大整数）、1,348 个来源 pointer 解析、所有记录的 singleton tag 校验、墙创建/变化成对状态、2D 自由墙曲线、装饰差异与楼梯副记录样本。大整数不能经 JavaScript `Number` 后再保存；现有 Python 解析保留它们。验证没有证明几何生成、撤销执行或运行时结果。

## 文档和版本

33 份文件都含 `Camera`、`History`、`Sheep`、`Time of day`、`Version`。部分还含 `Atmosphere`、`Grading`、`Postprocess`；11 份 Version 27 文档含 `Identity`、`BackwardCompatibility`、`Colors`，其 `BackwardCompatibility` 均为 27。

`History` 均有 `edit_chain`、`chain_index`、`snapshots`；28 份有 `history_start`。版本统计：16 为 2 份，17 为 3 份，18 为 16 份，20 为 1 份，27 为 11 份。此字段不是产品发行版本，也不是 Cargo 包版本。

**版本适配不能按命令名猜新旧。** 本样本 Version 16/17/18/20 的墙命令都叫 `WallNewSystem`，Version 27 的墙命令都叫 `Wall`。前者状态可见 `structure_ty`、`arch_ty`、`has_skirts`；后者可见 `style` 以及 `ChangeFreehandHeightType`、`ChangeArchType` 等不同操作。实际迁移代码尚未还原。

6 份文档在 `chain_index` 之后仍有条目，数量为 1、1、1、1、2、3，故不得直接把整个数组视作当前状态。已观察 cursor 是有效数组索引；它很可能表示当前历史位置，但“执行到此位置”的语义仍要检查加载/撤销函数。

`snapshots` 有 54 条引用，32 条可在本 history 同目录的 `snapshots/<snapshot_id>.snapshot` 找到文件，22 条找不到。整个资源目录实际有 36 个 snapshot，额外 4 个不属于这 32 条存在引用的闭包。缺失引用未被当成解析失败，也不能据此确认安装损坏；可能是历史引用保留或发布裁剪，需读取加载策略。文件身份与缺失 pointer 已完整记入 schema。

## 外层命令及嵌套变体

| 外层 tag | 次数 | 已观察嵌套结构 |
|---|---:|---|
| `WallNewSystem` | 682 | 操作组数组，见下节 |
| `Wall` | 242 | 操作组数组，见下节 |
| `Roof` | 241 | CreateRoof 92 / VisualEdit 118 / DeleteRoof 26 / SwitchRoofRidgeDir 5 |
| `DecoratorV5` | 345 | Place 144 / Change 190 / Remove 11；负载在这些样本中为空对象，实质记录在同 entry 的 decorators |
| `Terrain` | 102 | NewTerrainStroke 42 / EditTerrainStroke 56 / MoveTerrainStroke 4 |
| `Tree` | 609 | Place 598 / Move 11 |
| `Color` | 114 | Wall 53 / Roof 23 / Halftimber 13 / Decorator 10 / StairRailing 7 / FlatRoof 4 / StairLadder 1 / Brightness 3 |
| `Stairs` | 9 | SupportsDelete 7 / SupportsRecreate 2 |
| `Path` | 234 | brush_size / is_additive / points；228 次还含 blend_mode |
| `Water` | 140 | brush_size / is_additive / points / blend_mode |
| `Garden` | 53 | 同类笔刷字段；40 次含 blend_mode |
| `Cobblestone1` | 1 | 同类笔刷字段，含 blend_mode |
| `Cobblestone2` | 2 | 同类笔刷字段，含 blend_mode |

这里列出的次数为外层命令出现次数，不是操作组中单个操作数量，更不是可见玩家操作次数。一个 entry 可以包含多类命令及派生副记录。

## 墙：稳定标识、分组操作、形态与参数

已观察负载形式为 `[[{Operation: payload}, ...], ...]`，而非固定两项 before/after 元组。`WallNewSystem` 有 1/2/3/5 个组，出现 398/132/9/143 次；`Wall` 有 1/2/5 个组，出现 190/44/8 次。各组原始顺序完整保留。组 0 常含直接编辑，后组可含删除/创建/变化，但不能仅凭此赋予固定执行阶段含义。

`WallNewSystem` 的全部已观察内层操作及次数：CreateWall 311、WallChanged 459、DeleteWall 179、AdjustHeight 221、MorphPremadeShape 80、ChangeStyle 5、ChangeElevation 16、ChangeArchHeight 12、MorphFreehandWall 49、ChangeIsEnclosedState 2。

`Wall` 的全部已观察内层操作及次数：CreateWall 40、WallChanged 58、MorphPremadeShape 85、ChangeElevation 14、AdjustHeight 69、ChangeStyle 9、DeleteWall 12、MorphFreehandWall 7、ChangeArchHeight 6、ChangeFreehandHeightType 1、ChangeArchType 1。

CreateWall 有 `wall_index`、`wall_height`、`elevation`、`color_id`、`after`；DeleteWall 有 `wall_index`、`wall_height`、`color_id`、`before`。变化操作通常有 `wall_index` 和 `before`/`after`；WallChanged 另有 `operation_ty`，可为 `ExpandFrom`、`ShrinkTo` 或字符串形式的无负载值。闭合状态可为字符串，也可为 `{PremadeShape:{params:{Rectangle:...}}}` 或 Circle 变体。

可验证样本：`assets/starting-builds/demo/00-demo2/history.json` 的 `/History/edit_chain/0/edits/0/WallNewSystem/0/0/CreateWall` 与 `/History/edit_chain/0/edits/0/WallNewSystem/1/0/WallChanged` 都指向 `wall_index:25`，前者 `after` 精确等于后者 `before`。这证明同一命令组中记录了创建后的状态再变化；不能代替原作调用顺序证明。

已采样 1,688 个带 `points`/`points_u` 的曲线对象：点数与参数数全部相等，`points_u` 全部单调非递减且位于 `[0,1]`。其中 1,576 个曲线对象的所有点为 3D，112 个为 2D；2D 样本位于 MorphFreehandWall 的 before/after。例：`full/04-small-bridge/history.json#/History/edit_chain/28/edits/0/Wall/0/0/MorphFreehandWall/before`。因此不能把所有曲线都直接读取为 Vec3。`length`、`points_u` 的具体计算方法、2D 轴约定和轮廓求解尚待反编译。

## 屋顶：墙标识及可编辑参数

CreateRoof/DeleteRoof 都含 `wall_index`、`color_id`、`params`，部分有 `flat_roof_style`。params 可见 `height_01`、`profile_01`、`eave_length_01`、`ridge_length_01`、`ridge_dir`、`tip_offset_01`。VisualEdit 的 118 例均包含 `prev_*`/`new_*` 形式的 eave、height、profile、ridge、tip_offset 和 `wall_index`；SwitchRoofRidgeDir 为 `wall_index`/`from`/`to`。

直接结论是历史屋顶记录使用 `wall_index`，这些样本没有独立 `roof_id` 字段。候选解释是屋顶由对应墙/闭合轮廓对象索引；是否真正同一 ID 类型、是否一墙一顶及平顶的对象归属，需要消费者函数确认。01 参数名不能证明完整归一化方程或视觉阈值。

## 装饰：owner 目标与附着位置是两层信息

408 个 entry 有 `decorators`，其 `diffs` 共 427 个批次，每批形式为 `[target,{before:{id:record,...},after:{id:record,...}}]`。target 观察为 `{Wall:数字}` 243 批、`"Terrain"` 174 批、`"Stairs"` 10 批。id 为 JSON 数值字符串键；不应假定不同 target 中相同 id 是同一对象。

以每批内键集比较统计，新增 166 次、删除 38 次、修改 319 次，共 523 个装饰 ID 的变化出现次数。**427 是批次数，523 是批内记录变化次数；同一批可变更多个 ID，同一 ID 可在多条历史中反复变化。** before 共 357 条记录=319 修改+38 删除；after 共 485 条记录=319 修改+166 新增；未变共享 ID 为 0。结合字段名 `diffs`，这强支持“只记录变化项”的候选解释，仍需应用函数确认是否属于稀疏映射和删除约定。

每条装饰记录均可见 `dst`、`rank`、`seed`、`ty`，另有可选 `color_id`、`allow_connection`、`chimney_h_offset`、`h_offset`、`clv`、`stair_width`、`railing`、`ladder_color`。dst 为带负载枚举：

| dst | 直接观察字段 |
|---|---|
| Wall | anchor（anchor_ws / corner / platform / side / wall_height_relative_y）、coord（x/y） |
| Roof | mesh_uv，二维数组 |
| Terrain | pos_xz、yaw_radians |
| Stair | graph_edge（二整数）、uv、yaw_radians |

关键原始样本：`assets/starting-builds/demo/02-demo4/history.json#/History/edit_chain/41/decorators/diffs/0/0` 为 `{Wall:5}`，同批 `/History/edit_chain/41/decorators/diffs/0/1/after/0/dst` 为 `{Roof:{mesh_uv:[0.3183494210243225,0.7705391049385071]}}`。**已观察是 Wall:5 目标批次内有 Roof 附着记录。** “墙 ID 是屋顶装饰宿主索引”属于高置信候选，不能据此宣称完整内部宿主模型已经还原。

## 楼梯副记录不同于装饰差异

63 个 entry 带 `stairs`，其四个字段为 `before`、`after`、`hash_before`、`hash_after`。before/after 是整数边对数组，边对每项恰为两整数；它们不是 decorators 的 `diffs` 字典。独立 Stairs 命令的 SupportsDelete/SupportsRecreate 也含 edge（二整数）。这提供楼梯关系图的线索，但边的节点类型、hash 算法、支持件生成及是否为完整拓扑快照尚未知。

## 其他 ID 和状态记录

Terrain NewTerrainStroke 用 `index`、`height_ws`、`stroke_pts`；EditTerrainStroke 用 `index`、`prev_height_ws/new_height_ws`、`prev_falloff/new_falloff`；MoveTerrainStroke 用 `id`、`original_stroke_pts`、`move_vec`。index/id 是否可互换不能仅从命名认定。

Tree Place 中同时存在外层 `index` 和 `tree.id`，tree 另含 position、scale、rotation_degrees，部分有 prefab_name、is_preplaced；Tree Move 用 `index` 和 `from`/`to`。因此 index 与 tree.id 必须保留为不同字段，ID 分配、prefab 查找与预置树处理待定位。

Color Wall/Roof 等变体记录 wall_id 与 from/to；Decorator 另含 decorator_id。它与 wall_index、装饰 target 数值的关系是消费者函数应首先验证的标识连接点。

## 下一步原作代码核验

1. 原作 history 的 Deserialize/版本迁移→加载 chain_index/history_start/snapshot 选择，解释缺失旧快照如何处理。
2. Wall/WallNewSystem 的各操作组消费顺序→领域状态改变→派生创建/删除→几何构造，确认是否有中间帧或仅最终提交。
3. 装饰 diff 的 target 查询和 before/after 应用→dst Wall/Roof/Terrain/Stair 解析→局部坐标与几何适配，尤其核验 Wall:5 + Roof UV 样本。
4. Roof params/VisualEdit→屋面求解→屋瓦实例写入与 shader 过渡参数；当前历史数据本身不包含动画方程。
5. stairs 的边/hash 和 Terrain index/id 的消费者，之后才考虑 snapshot 容器解码。

这轮完成的是原作持久化数据与编辑记录的结构还原。实际状态应用、撤销/重做、版本迁移、几何算法及连续视觉变化仍未还原，不宣称已有可运行复刻。

运行方法（输出位于独立还原目录）：

```powershell
python D:\game\reverse-engineering\tiny-glade\reconstruction\history\observed_history_model.py
python D:\game\reverse-engineering\tiny-glade\reconstruction\history\verify_observed_samples.py
```
