# RESTORE — timberframe（组 A restorer，2026-10-16）

> 实现落点：`tiny-glade/crates/systems/timberframe/`。
> 证据：`groupA/timberframe/RE-ANALYSIS.md`、`groupA/timberframe/decomp/`、BASELINE。

## 实现清单

| 函数 | VA | 状态 | 实现要点 |
|---|---|---|---|
| `impl$0::add_timberframe` | 0x140630a20 | 已实现（O 面） | insert_resource（TimberframeInstanceBuffer/TimberframeBeamCount）+ register_event + add_systems(chain) |
| `timberframe_beam_count_change_audio` | 0x1421b6610 | 已实现 | `(当前-上次==1) && 非空` → PlayTimberframeSound {kind=6, ShapeParameters::center 位置}；size_01（get_building_size_01 0x1417d3f10）登记待 core 音频域扩展；上次计数推进（I） |
| `construct_timberframe_circle` | 0x1421b67a0 | 已实现（算法骨架） | 28B 墙形状 → ShapeParameters::get_shape → Circle2d；perimeter + 0.2 间距 u 采样（get_pos_at_u/get_normal_at_u）；屋顶高 roof_height_ws_from_wall_shape（0x1408e3c70）；梁实例 → TimberframeInstanceBuffer（0x20B 条目，GPU 侧 WoodenPlankSsbo）；构造后刷新 beams.current |
| `construct_timberframe_rectangle` | 0x141a41ba0 | 已实现（算法骨架） | Rectangle2d 四边采样（as_points2 + 0.2 间距）；0.5 归一化（0x142925870 字节直读——re-analyst 记 0.4922×9，**以字节为准 0.5**）；0.63 缩放（0x1429258e0 区，I）；RoofRidgeDims/ridge_length 面登记 |
| `timberframe_on_glade_unload` | 内联（行 50） | 登记（内联体） | — |

## core 侧接线（delta）

- `resources/roofs/roof_buffers.rs`：WoodenPlankSsbo（typed gap，类型名串 0x142ff8cf0）。
- `systems/wall/shapes/calc_shape_height.rs`：TimberframeConstructionParamsCache（typed gap）。

## 未实现项 / 待 reviewer

1. 梁实例 0x20B 记录字段全表（位置/朝向/长度公式 I；0.63/0.5 归一化的精确语义）。
2. 0x140b34f80 归属冲突（HighlightColor::to_rgb vs split_height_into_skirts）。
3. beam_count 的位置源 = 命令载荷墙形状中心（已落地 ShapeParameters::center 解码）。
4. add_timberframe 的 3 系统精确链序（chain_ignore_deferred 面，O 结构）。

## 编译

`cargo check -p timberframe`（离线）通过。

## 修复轮 2（reviewer-A）

- circle：param_2 改 `Res<TimberframeInstanceBuffer>` 只读 + cap==0 门控（机器码无 tick 回写；梁输出路径 re-analyst §3.2 待补证——GAP 登记，不再 push）。
- rectangle：Beams/Cache 参数序互换（机器码 param_4=BeamCount(tick)/param_5=Cache(只读)）。
- RECT_SCALE=0.63 无字节支持（0x1429258e0 实测 0.5×2）——已标 I，保留登记。
