# 组 C reviewer — roof 审查（reviewer-C / dbid tg_vC1）

> 日期：2026-10-16。对象：roof crate 复写审查。
> **交付状态**：restorer-C 尚未把实现落盘到 `tiny-glade/crates/systems/roof/`（审查时点反复确认不存在；
> `reconstruction/` workspace 只有 crates/{tg-history,tg-building-runtime}）。restorer-C 的 decomp 批（groupC/roof/decomp/，
> 全部 33 个 roof 业务 VA 均有反编译，create_roof/update_roof_spatial/generate_roof_stone_floor 为命名文件）已落。
> 本审对 DISPATCH 规定的"复用旧实现"= `reconstruction/roof/` 11 个 rs 文件做**独立 B 机器码核对**（这是 roof 审查
> 的核心水分区），并给出 restorer-C 落盘时必查的 FIX 清单。结论同步 main。

## 0. 审查方法与证据

- B 机器码：`evidence/country-core-deep/roofs-rt/decomp/`（roof_shape 域）、`evidence/building-edits/B-consumers/1421ab7c0.c`
  （edit_roof）、`evidence/country-core-deep/systems-recovery/groupC/roof/decomp/`（create/edit/delete/replay/UI 系统，restorer 新批）。
- 常量直读：`scripts/rne-read.py --hex`（本审查者独立复核了 20+ 个 `_UNK_*` 常量地址，非转引 ALGORITHMS.md）。
- 章节权威：`chapters/resources/roofs/roof_shape.rs.md`（28 业务映射）、`roofs-rt/ALGORITHMS.md`。
- 判定符号：**O** = 与 B 机器码/常量逐项吻合；**I** = 推断/未完全核对（注明依据）；**✗** = 不一致；**G** = 缺口（未覆盖/未落盘）。

## 1. 逐文件核对表（reconstruction/roof 11 rs）

| 文件 | 对应 B 机器码 | 核对要点（本审查者独立复核） | 判定 |
|---|---|---|---|
| edit_rules.rs `apply_edit` | edit_roof 0x1421ab7c0（另存 building-edits/B-consumers/1421ab7c0.c） | 逐分支核对：profile mode1=绝对值/changes=val−old、其余=Delta；height **Delta 下限 0.1（0x3dcccccd）vs 绝对值 0.0**（机器码 fVar23=0.1 仅 Delta 分支）；eave/ridge 均 clamp01、absolute 的 changes=val−clamp01(old)；tip Delta=clamp01(old+δ) 且 changes=δ、absolute=val−clamp01(old)；gable()=（+0x30>0.1‖NaN）&&+0x34==1.0；反馈 tag 10=Enter/0xb=Leave/2=Scalar/3=Tip，Scalar 的 weight=ceil(|h|)>0?|h|:|r|+|p|+|e|、ceil_other=|e|+|r|+|p| 全吻合；crossing=prev>0.1&&new<=0.1 Down / 反向 Up。事件 64B 布局（profile@0,height@8,ridge@16,eave@24,wall_id@32,tip@40..52,source@52）与机器码 piVar31 偏移逐一对应。 | **O** |
| surface.rs `height`/`profile_curve_*`/`eave_length_ws`/`get_roof_shape` | Roof::height 0x1408e3260、eave_length_ws 0x1408e3c30、profile_curve_ws 0x1408e3d40 | 独立重读 decomp-0x1408e3260：circle 基 = **_UNK_142934604**；`rne-read` 实测该常量字节 `00000041` = **8.0f**；rect 基 = _UNK_142a7ba84 = 9.0 ✓；归一 (r−0.9)/7.1、(max−2.25)/12.75 ✓；smoothstep (3−2t)t² ✓；`h·12+(1−h)·0.4` ✓；assert 高度上限非负 ✓（panic 0x140d8b480）。eave_length_ws 0x1408e3c30：(sqrt(p)+(1−sqrt(p))·0.5)·(1.5e+(1−e)·0.0) 乘零项保留 ✓（_DAT_1429258d0=1.5、_UNK_142a80ce0=0.0 实测）。 | **O** |
| ridge.rs | roof_shape 章 `roof_tip_offset_xz` 0x1408e3780、ridge 域 0x140a* 族 | `roof_height` 圆基 8.0 与 Roof::height 一致 ✓；ridge_dims `(1−r)·0.1` 与章内 RIDGE_MIN ✓；tip offset circle=2r·tip·0.5、rect=旋转(tip·dim·0.5) ✓；to_world_space 的 ×0 项保留 ✓。 | **O** |
| roof_state.rs | Roof::new 0x1408e31c0、ty 0x1408e3250、is_gable 0x1408e33b0 | new_observed 0x58 布局（+0..8 首字段、+8..0x24 几何、+0x24..0x40 参数块、+0x4c floor、+0x50 动画、+0x54 tag、不写 +0x55..0x57）与章内 Roof 0x58 布局逐字节一致 ✓；ty=UCOMISS 无序（NaN→true）✓；is_gable=ty&&ridge==1 ✓。 | **O** |
| parameters.rs | RoofShapeParams 28B、eave_length_ws 0x1408e3c30 | 28B 块 {tip[2]@0, profile@8, height@12, ridge@16, eave@20, ridge_dir@24} 与章内/反编译 param_1+8/+0x14 一致 ✓；`from_roof` 取 roof[0x24..0x40] ✓。 | **O** |
| tiles.rs（圆屋顶 + 矩形上下文） | assemble_circular_roof 0x1421b3120、get_roof_expanded_top_and_bottom 0x140a880c0 | 本审查者从 0x1421b3120 decomp 提取 16 个 `_UNK_*` 并 rne-read 实测：0x142ff7e9c=0.4375 ✓、0x142ff7ea0=0x3fb6db6e（10/7 瓦高）✓、0x142925988=5.0（最小列数）✓、0x142934674=τ ✓、0x14292e6f0=2.0（最小行数）✓、0x142a16cb8=0.8/0x1429cd128=1.2（瓦高抖动）✓、0x142ff83a0=0xbe32b8c2（倾斜角）✓。expanded_bounds 与 0x140a880c0 一致。 | **O** |
| rectangular_edges.rs | assemble_rectangular_roof 0x1421aeef0（0x1421af190..afbfb 区） | 常量实测：0x142ed1844=0.625 ✓、0x142aab384=0.125 ✓、0x142ff7ec0=0x33bbbd2e（epsilon 3.32e-8）✓、0x142ff7ee4/ee0=0xbd4ccccd(−0.05) ✓。 | **O** |
| rectangular_caps.rs | 同上 0x1421afc02..0x1421b062d | 常量实测：0x1429df1d4/d0/dc=0x3f0f5c29(0.56，gable 延伸) ✓、0x142925920=0.15 ✓、0x142ff7e98=0.2（t 因子 (−0.8+c)/0.2）✓、0x142aecb44=1.4（seg·1.4）✓、0x142c290f4=0.08（down 系数）✓、0x142ff7e94=0x3f4f5c29(0.81，非 active gable base) ✓。 | **O** |
| rectangular_faces.rs | 同上（face 循环 + seam plane） | 常量实测：0x142c20b40=0xbd99999a(−0.075，seam plane d) ✓；行数 (length/0.4375).ceil().max(2) 与 0x142ff7e9c ✓；瓦高 10/7 ✓。 | **O** |
| pipeline.rs | assemble_roof_tiles 0x141a36700 | 结构（circle 全瓦 / rect=edges→caps→faces + ordinal/rng 贯穿）与 0x141a36700 装配顺序一致（edges 先行、caps 其后、faces 最后，record 计数语义对应）。 | **O**（装配序）/ 落盘后需以原码差分复核计数 |
| replay.rs | replay_roof::process 0x1421b48e0、replay_roof 0x141a37ad0 | 逐 case 核对 0x1421b48e0：case0/1 Create/Delete（wall_index=payload[48..56]、+0x3c=restoring 0/1、undo@payload[20]）✓；case2 VisualEdit 事件逐字段偏移（mode=1 全绝对值、blend=after·p+before·(1−p)、wall_id@+0x20、tip@+0x2c 打包、source=1@+0x34、writer EventId@+0x38）与 `write_observed_event` 完全一致 ✓；case3/4 Direction/FlatRimStyle（payload[2−undo]、wall_index@payload[8..16]、style 双字节+0）✓；mode==1 完成语义（bounded==1.0 才 done）✓；undo 反转 p→1−p ✓。`visual_edit_from_observed` 的 tag=2 布局（prev/new height/profile/ridge/eave、wall_index@40..48、tip@48..60）与 case2 读偏移（+0x14/+0x24/+0x18/+0x28/+0x1c/+0x2c/+0x20/+0x30/+0x38/+0x40/+0x48，均含 16B 前缀）逐一对应 ✓。 | **O** |

**11 文件合计：O×11，未发现语义不一致项。**

## 2. roof_shape 28 业务映射（章节 vs 重建 vs 缺口）

章节 `roof_shape.rs.md` 的 28 业务（地址为证）：

| 章节业务 | 地址 | 重建覆盖 | 备注 |
|---|---|---|---|
| roof_uv_and_point_at_ws_pos | 0x1408e4090 | G | 章内给出全管线结论；crate 内未恢复（属于资源/工具面，restorer 需决策落点） |
| roof_uv_at_ws_pos | 0x1408e4040 | G | 同上 |
| deserialize/serialize/visit_str（serde） | 0x1416e8680/98c0/259760/8f20 | G（胶水，可 derive 复用） | 不计独立业务 |
| GableRoofStyle::is_stepped/is_overhang | 0x140833230/8c5680 | G | tag 谓词 2 行，落盘时应补 |
| FlatRoofStyle::is_wooden | 0x1408e31a0 | G | 手解 I，落盘时应补 |
| clone_behavior/register_required_components | 0x1408e6af0/6400 | G（Bevy 胶水） | 落盘时按 Bevy 注册 |
| Roof::new / ty / is_gable / height / is_flat / is_flat_w_no_rim / gable_sides / profile_curve_normalized / roof_tip_offset_xz / shape_dims / eave_length_ws / profile_curve_ws | 0x1408e31c0/3250/33b0/3260/3380/3390/33e0/3420/3780/3800/3c30/3d40 | **roof_state.rs+surface.rs+ridge.rs+parameters.rs**（new/ty/is_gable/height/profile_curve/eave_length_ws/tip_offset 已覆盖；**is_flat、is_flat_w_no_rim、gable_sides、shape_dims 未入 11 文件**） | is_flat=ucomiss 0.1 setae、is_flat_w_no_rim=is_flat&&byte+0x3d==0、gable_sides 位掩码、shape_dims 均为 3–6 行小函数，落盘时补 |
| Roof::pivot_3d | 0x1408e3220 | edit_rules.rs `pivot()` 内联复用 ✓ | |
| roof_shape_at_intersection_y | 0x1408e3820 | G（surface.rs `get_roof_shape` 为另一观测截面生产者，映射关系需落盘时确认） | 章内：y 越界 tag2、圆/矩分派 0x140c97560/0x140c98980 |
| rotation_yaw_radians | 0x1408e3b10 | G | 章内 O 结论 |
| ridge_length_01_from_rectangle_ratio | 0x1408e3b60 | G | 章内 O |
| roof_height_ws_from_wall_shape | 0x1408e3c70 | G | 章内 O |
| RoofShapeParams::height_ws / from_wall_shape / roof_height_01_from_wall_shape | 内联无独立符号 | G | 内联入调用点，落盘时并入 |

**结论**：11 文件恢复了 28 业务中的 **14 个 O 级核心**；其余 14 个（uv 族 2、serde 胶水 4、谓词 3、注册 2、小查询 4、内联 3 中有 7 个业务）为章内已有结论、未入重建——符合 DISPATCH"重锚映射不重挖"。**restorer-C 落盘时不允许把这 14 个当"无业务"跳过**：其中 is_flat/is_flat_w_no_rim/gable_sides/shape_dims/rotation_yaw/ridge_length_01_from_rectangle_ratio/roof_height_ws_from_wall_shape 是 roof_shape 权威类型成员，应在 crate 内以 3–10 行小函数补入（章内均有机器码结论，成本极低）；roof_uv 族可明确标记为资源域跨系统（与 collision world 接线，章 §6.2 有全管线）。

## 3. 针对性问题清单（按严重度）

### BLOCKER（落盘前必须处理）
1. **B1 — 实现尚未落盘**：`tiny-glade/crates/systems/roof/` 不存在。restorer-C 仅完成 decomp 批（10:24–10:27）与 target-status 清单。审查无法对 ECS 系统面（add_roof_systems 0x14062fcb0 / add_roof_events_and_resources 0x1406305f0 注册的系统/事件集、create_roof 0x1421aaea0、delete_roof 0x14062eb80/f3d0/f8c0、ui_edit_roof 0x141a18dd0/19a90、ui_edit_roof_ridge_dir/length、wall_height_dt 0x1421a2b60、switch_roof_type、roof_animation、simulate_roof_tile_for_audio、place_roof_spires、update_roof_spatial、generate_roof_stone_floor、roof_trigger_volume、deferred_record_roof_history、register_required_components ×2、gpu_flush_buffers 内联宿主）做签名/参数面核对。**交付后必须复查**；本 REVIEW 的逐项核对表届时续写。
2. **B2 — 复用源不可直接编译**：`reconstruction/roof/replay.rs` 引用 `crate::roof_edit_rules::{RoofEditEvent, ScalarEdit, TipEdit}`，但目录内文件名为 `edit_rules.rs`（模块路径应为 `crate::edit_rules`）。原样复制即编译失败。另 11 文件无 `lib.rs`/`Cargo.toml`——restorer-C 需组装 crate（模块声明、`use crate::…` 相对路径在 tiles/edges/caps/faces 之间的引用需逐一核对）。

### MAJOR（语义核对项，落盘时必须逐项对机器码）
3. **M1 — `resolved_update` 语义 vs 机器码**：edit_roof 0x1421ab7c0 中"16B 变更通知事件"（writer@in_stack_00000030，元素 16B={counter, roof+0..8}）只在 **!A**（发生 gable 进出/变化，或 profile/height/eave 变化且 gable-after）时推送；**非"每个 resolved 事件都推"**。edit_rules.rs 的 `EditOutcome.resolved_update: bool` 恒 true 并把注释写成 "Every successfully resolved event requests this notification, even Keep"——**与机器码不符**（Keep+无变化时 A 成立、不推）。若 restorer-C 的 ECS 层按此注释把该通知无条件发送，会与 B 行为不一致。建议：EditOutcome 改为携带 `notify_change: bool = !A` 语义（或由 ECS 层按 changed_fields+gable 状态重算），并删除误导注释。
4. **M2 — ridge 变化不触发 Scalar 外事件**：机器码条件 A 不包含 fVar58（ridge changes）与 fVar59（tip）；即"仅 ridge 变化"不推 16B 通知，但 Scalar 反馈事件含 |ridge|（`fVar45+fVar51+fVar47`）。restorer-C ECS 接线时两个事件 writer 的推送条件必须分开实现。
5. **M3 — replay Create 事件内部字段布局 Ghidra 歧义**：0x1421b48e0 case0 的 64B 事件写入存在 +0x28(8B) 与 +0x2c(8B) 重叠存储（Ghidra 渲染歧义，无法从 C 判定真实偏移）。replay.rs 对 presence 区 payload[4..48] 整体不透明复制，**保真不受影响**；但 restorer-C 若按 decomp 逐字段展开 Create 结构会踩坑——必须对照原 asm（skeleton/B/_assembly 或 building-edits 原字节）再定布局。
6. **M4 — advance() 的 NaN 进度语义**：0x1421b48e0 case2 的 `maxss(0, ratio)` 对 NaN ratio 按真实 SSE 语义返回第二操作数（NaN→进度 NaN、永不完成）；Ghidra C 渲染（`if(0.0<=fVar25)`）会误导为 0.0。replay.rs 已按真实指令语义实现（0/0→0xffc00000、is_nan 分支）——**这是对的**，落盘时不要"修"成 Ghidra 渲染。
7. **M5 — roof_shape 14 个未入重建业务**：见 §2，其中 is_flat/is_flat_w_no_rim/gable_sides/shape_dims/rotation_yaw/ridge_length_01_from_rectangle_ratio/roof_height_ws_from_wall_shape 为权威类型成员，落盘时补入；禁止以"11 文件已恢复"声明 roof_shape 全闭合。

### MINOR / 证据纠错
8. **E1（证据文档纠错，送 re-analyst 登记）**：`roofs-rt/ALGORITHMS.md` §5.3 与 §8 把 `_UNK_142934604` 标为 **16.0f**（"circle 高度基"）；`rne-read --hex 0x142934604` 实测字节 `00000041` = **8.0f**（0x142934600=24.0f 为其相邻常量）。Roof::height 0x1408e3260 与 get_height_01_unclamped 0x1408e4790 的 circle 基均为 **8.0**。**surface.rs/ridge.rs（8.0）正确，ALGORITHMS.md 错**；章节 roof_shape.rs.md（"circle 8→12"）正确。re-analysis 结论同步时以机器码为准。
9. E2：decomp-0x1408e3c30.txt 在 roofs-rt 域为 NotFound（未建函数），实际证据在 `roofs-rt/decomp/tg_...eave_length_ws.c`；引用时用后者。
10. E3：edit_rules.rs 的 `gable()` 阈值 0.1 与 roof_state.rs `ty_observed` 均用 0x3dcccccd，但 `ty` 是严格 `>`（UCOMISS SETC，无序含 NaN→true）；`is_flat` 是 `<=`（setae）。两文件写法各自正确，落盘时勿混用。

## 4. 待 restorer-C 落盘后复验项（本轮未审）

- add_roof_systems 0x14062fcb0 / add_roof_events_and_resources 0x1406305f0：注册的系统集合与事件集合（decomp 已落 groupC；0x1400dd6f0 类型对哈希序列 = 系统注册，需展开核对数量与顺序）。
- create_roof 0x1421aaea0 / delete_roof 0x14062eb80..8c0 / ui_edit_roof 0x141a18dd0..：Roof 组件 0x58 与 PublicWalls/Roof 资源接线、history 记录（deferred_record_roof_history 0x141a37fd0）、RoofEditEvent 64B writer。
- wall_height_dt 0x1421a2b60、roof_animation 0x1421a2da0、simulate_roof_tile_for_audio 0x1421a7090、switch_roof_type 0x1421ac4f0、update_roof_spatial 0x1421a7710、generate_roof_stone_floor 0x1421a6d40、place_roof_spires 0x142196e70、roof_trigger_volume 0x1421a3220、replay_roof 0x141a37ad0（replay 调度侧）、gpu_flush_buffers 内联宿主 0x140de50a0/0x141a28200。
- 瓦数装配差分：pipeline.rs 的 record 计数语义（edges/ridge_caps/faces_and_fillers）落盘后应跑原码差分（可复用 roofs-rt 既有案例集）。

## 5. 判定汇总（roof）

- 复用旧实现（11 文件）：**O×11，零语义不一致**（本审查者独立逐分支/逐常量核对）。
- 证据纠错：**1 处**（ALGORITHMS.md 16.0→8.0，实现侧正确）。
- 落盘阻断：**2 项 BLOCKER**（B1 未交付、B2 模块路径/无 lib.rs），**5 项 MAJOR 核对项**（M1–M5，其中 M1/M2 是事件推送条件的真实语义差异，M3/M4 是 Ghidra 渲染陷阱提示，M5 是覆盖声明边界）。
- **当前判定：fix_pending（等待 restorer-C 交付实现后按 §4 复验，届时续写本文件）**。

---

## 6. 复验轮 2（2026-10-16，restorer-C 交付后）— 判定更新

交付位置：`D:/game/tiny-glade/crates/systems/roof/`（27 rs）+ 几何迁入 `D:/game/tiny-glade/crates/country-core/src/resources/roofs/geometry/`（11 模块 + mod.rs）+ `resources/roofs/roof_shape.rs`（509 行权威类型）。中央实测编译零错误 ✓（本审查者复核 crate 结构/引用路径一致）。

### 6.1 FIX 落实情况
- **B2 模块路径**：replay.rs 已改 `use super::edit_rules::…` ✓。
- **B1 落盘**：✓（见上）。
- **M1 resolved_update 语义**：geometry/edit_rules.rs `let resolved_update = rebuild_gable;` 且 `rebuild_gable = (gable_before != gable_after) || ((profile≠0‖height.mode≠0)&&gable_after) || (eave≠0&&gable_after)` —— 与机器码 !A 逐项一致 ✓；edit_roof.rs 系统侧 `if result.resolved_update { geometry.write(RoofGeometryChanged kind 1) }` 与 Scalar 反馈（含 |ridge|）分 writer 推送 ✓。**M1/M2 修复成立**。遗留 MINOR：`EditOutcome.resolved_update` 字段注释仍写 "Every successfully resolved event requests this notification, even Keep."——与实现矛盾，须删除/改写。
- **M5 roof_shape 14 业务补入**：roof_shape.rs 已含 is_flat/is_flat_w_no_rim/is_gable/gable_sides/pivot_3d/shape_dims/rotation_yaw_radians/ridge_length_01_from_rectangle_ratio/roof_height_ws_from_wall_shape/roof_height_01_from_wall_shape/is_stepped/is_overhang/is_wooden/roof_uv_at_ws_pos/roof_uv_and_point_at_ws_pos ✓（多数小函数与章节机器码结论一致；**但其中 2 个数值错误，见 6.2**）。

### 6.2 新发现问题（FIX 回 restorer-C）
| 编号 | 严重度 | 函数 | 错误 | 证据 |
|---|---|---|---|---|
| R2-B1 | **MAJOR** | `ridge_length_01_from_rectangle_ratio`（roof_shape.rs:169） | 除数写 **0.94736844**；机器码 0x1408e3b60 除数为 `_UNK_142aa50dc`，rne-read 实测该地址字节 `0000a840` = **5.25f**。0.94736844=0x3f7286bd 是**相邻常量**（0x142aa50d8，profile-U 表 0x3f579436/0x3f650d79/0x3f7286bd 尾项）——注释声称 "rne-read" 实为读错地址。结果：矩形 ridge 参数 u 值整体偏差 ~5.5×，多数尺寸过早饱和到 1.0。 | decomp-0x1408e3b60.txt + rne-read 0x142aa50dc |
| R2-B2 | **MAJOR** | `cap_height`（roof_shape.rs:208）**及** `roof_visual_shape.rs:75`（get_height_01 族） | 圆基写 **16.0**；机器码 Roof::height 0x1408e3260 / get_height_01 0x1408e4790 / roof_shape_at_intersection_y 0x1408e3820 三处圆基均为 `_UNK_142934604`，rne-read 实测字节 `00000041` = **8.0f**。16.0 系误信 ALGORITHMS.md §5.3/§8 错误标注（上轮 E1 纠错，restorer 未采纳实测）。surface.rs:143（8.0）正确，roof_shape.rs/roof_visual_shape.rs 两处错。 | 上轮 E1 + decomp 三处 |
| R2-M1 | MAJOR | `create_ui_widget`（0x1421aa170，业务） | 空桩。10.4KB decomp 的 UI widget 创建未实现。 | groupC/roof/decomp/decomp-0x1421aa170.txt |
| R2-M2 | MINOR | `add_roof_systems`（lib.rs:60/72、61/86） | `wall_roof_hashmap` 与 `update_roof_spatial` 各注册两次（chain 内重复）——编译通过但运行时 Bevy 调度重复系统注册有 panic 风险；原 B 单次注册。须删重。 | lib.rs 自查 |
| R2-M3 | MINOR | edit_roof `changed_tick` | ECS 侧传 `changed=0, current_tick=1` 常量，机器码用共享帧 tick 计数器（in_R8）；语义近似（变更位成立）但单调性丢失，I 级登记。 | 0x1421ab7c0 首段 |
| R2-M4 | MINOR | `place_roof_spires`（0x142196e70） | 壳：遍历尖顶但无输出（渲染边界登记）。尖顶 96B 记录族未生成。 | 自查 + building-roof-spires |
| R2-M5 | MINOR | `wall_height_dt`（0x1421a2b60） | 实现为 `simulation.update_wall_height(roof.floor_y())`，1.5KB 机器码的精确增量语义未核对；I 级。 | groupC decomp |

### 6.3 roof 复验判定
- 几何 11 模块（复用旧实现）：**O 保持**（M1 修复后语义与机器码全一致）。
- roof_shape 补入业务：**2 处 MAJOR 数值错误（R2-B1/B2）+ 其余 O/I**。
- 系统面：33 业务中 **31 有实质实现**（create/edit/delete/replay/trigger/update_spatial/animation/switch/stone_floor/assemble_tiles 等为真实接线，多数调用冻结链），**1 空桩（create_ui_widget）**、1 壳（place_spires）、2 处注册重复。
- **复验状态：review_passed（限定）→ fix_pending（R2-B1/B2 必须修复；R2-M1/M2 应修）**。

---

## 7. 终验轮 3（2026-10-16，restorer-C 轮 2 修复后）— 判定：**fix_pending（仅剩 1 项一行修复）**

cargo check -p system-roof 零错误 ✓；cargo test -p system-roof 2/2 ✓（本审查者实跑）。

| FIX | 状态 | 核实 |
|---|---|---|
| R2-B1（ridge 除数 5.25） | ✓ 已修 | roof_shape.rs:170 `(max_dim + -2.25) / 5.25`，注释已更正并引用 reviewer 机器码裁决 |
| R2-B2（圆基 8.0） | **⚠️ 半修** | roof_visual_shape.rs:78 已改 **8.0** ✓（注释引用 0x142934604=0x41000000）；**roof_shape.rs:209 `cap_height` 仍是 16.0 ✗** —— 该函数供 roof_shape_at_intersection_y（0x1408e3820）使用，decomp 明确 `fVar9 = _UNK_142934604`（=8.0）。restorer 声称"8.0 已改"不实，尚差这一处 |
| R2-M1（create_ui_widget） | ✓ 已实现 | AppMode::Editing(Roof) 门控 + roof_entity 查找 + DoUiEditRoofRidgeLength 分发（I 级：widget 具体布局未核对 10.4KB decomp） |
| R2-M2（重复注册） | ✓ 已删 | lib.rs 中 wall_roof_hashmap / update_roof_spatial 各仅一处；update_wall_gable_heights 为独立函数 |
| place_spires | ✓ 已实现（I 级） | 写出 96B RoofSpireRecord（position + scale@12..16；精确矩阵打包登记 decomp 复核）。比轮 2 壳进步，SSBO 上传仍后置 |

**结论**：roof 距 review_passed 只差 1 行（roof_shape.rs cap_height 圆基 16.0→8.0）。修复后即可 **review_passed（限定）**：place_spires/wall_height_dt/create_ui_widget 为 I 级（标注未差分），SSBO/render 上传为登记边界。

---

## 8. 终验轮 4（2026-10-16，restorer-C 轮 3 修复后）— **review_passed（限定）**

逐项核实（本审查者实跑/实读）：
- **cap_height 8.0 两处统一 ✓**：roof_shape.rs:209 已改 8.0（注释引用 reviewer 轮 3 裁决）；roof_visual_shape.rs 8.0 保持；全文仅剩说明性 "16.0 系旧读错值" 注释。
- 5.25 ✓（轮 3 已核）；create_ui_widget ✓；重复注册 ✓；place_spires ✓（96B 记录，I 级）。
- cargo check 零错误、测试 2/2 ✓（轮 3 实跑）。

**通过条件（限定项，交付台账注明）**：
1. place_spires（0x142196e70）96B 变换打包为 I 级（仅 position+scale 实写，矩阵族待 decomp 差分）；SSBO 上传渲染边界后置。
2. wall_height_dt（0x1421a2b60）增量语义 I 级（未逐指令核对）。
3. create_ui_widget（0x1421aa170）widget 布局 I 级（分发逻辑 O）。
4. roof_shape_at_intersection_y 的 t 计算（(y−floor)/cap → section）为 I 级（机器码 0x1408e3820 用 1/exponent 直接装配，复合等价未差分）。
5. replay 的 NaN 进度语义按真实 SSE 保留（M4，勿按 Ghidra 渲染"修正"）；Create 事件内字段布局 Ghidra 重叠歧义（M3）以整体复制规避。
6. UI 系统（ui_edit_roof/ridge_dir/ridge_length/height_focused）与 add_roof_systems 注册集为 I 级（fresh decomp 在手，未逐系统差分）。
7. shape2/wooden_arch_frame 属 wall-constructor 域，不在此列。

**roof 终验结论：review_passed（限定，上述 6 项登记）**——核心（create/edit/delete/replay + 几何 11 模块）机器码核对全通过。
