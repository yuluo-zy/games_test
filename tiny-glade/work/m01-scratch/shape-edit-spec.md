# Tiny Glade B 版形状编辑 UX：实现规格（M01，15 叶子）

只读逆向规格。**未修改任何生产源码、未运行 cargo/build、未 commit。**
二进制身份：`workspace/input/tiny-glade.exe`（= B patched，SHA256 `cf048e27…6323`）与
`D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne`（B 原发行，`f507a6ce…7d03`）；本文常量均以该 exe 的
`.rdata` 实读（`scripts/rne-read.py --hex`，UTF-8 强制输出）。

证据基线与方法：
- **参数真值（权威）**：`reports/module-recovery-2026-10-23/m01-system-params-from-pdb.md`
  （`FunctionSystem<fn(..)>` PDB 拼写 + `drop_in_place<..FunctionSystem<fn(..),name>>`）。**不得沿用旧 Rust 签名。**
- **控制流**：磁盘反编译 `evidence/country-core-deep/{wall,ui-input}/decomp/decomp-0x*.txt`；已转 C 的
  `work/m01-scratch/{adjh,dd,dr,sh_dims,sh_rot,sigdims,fer}_body.c` 及 `work/m01-scratch/spec/*.md`（详细草稿，作为附录证据）。
- **常量**：本文 §0.2 实读表覆盖草稿里标 UNKNOWN 的项；未实读者仍标 UNKNOWN。
- 机器码→ABI 的解释由主分析复核；小模型结论不作为 ABI 真值。

状态汇总（详见各节末）：本文件先写全部 14 节（15 叶子），每节独立 READY/PENDING。
分节附录草稿（只读、未并入本文件）：`spec/ui_edit_circle.md`、`spec/ui_move_shapes.md`、
`spec/freehand.md`、`spec/rectangle_and_adjust_height.md`、`spec/circle_rotate_and_signifiers.md`；
审计：`audit/{adjust_height,circle,move_freehand,rectangle,toolbar_host}.md`。

---

## 0. 全局约定与实读常量

### 0.1 通用映射约定
- PDB `enum2$<AppMode>`：反编译 `*(u32*)*param`；`> 5` 直接 return；`1/2` 为 Move/Edit 子态（见各叶）。
- `ResMut<InnerWalls>` 为 hashbrown SwissTable（bucket stride **0x2a8**，负偏移访问：键 `-0x2a8`，`state u64@-0x290`，`f32 activation@-0x270`，`curve pts@-0x53`(ptr)/`-0x4f`(len,u32)，`aux@-0x50`，`height@-0x4`）。命中后统一断言
  `w.simplified_curve_2d.is_valid()`（assert 串 `@0x142ada193` len 0x32）。
- `EventWriter<E>` 底层 `Events<E>`；`EventInstance<T>` 布局 = `{ event, event_id: usize }`（一般 8B 事件 + 8B id）。
- `UiSignifierStream::write`（VA `0x14146c880`）= `AppendOnlyVec::push`；原生 40B 记录。**当前仓库的
  `UiAppendSignifier`(5×qword) 是早期数字适配，`SIGNIFIER-CONTRACT.md` 明令不得作为真实工厂；恢复时须用
  `SignifierFactory::from_closure` / `write_factory`，禁止继续 `stream.write(UiAppendSignifier{..})`。**
- `UiState::add`（VA `0x1409231f0`）返回 widget 上下文，字段 `+0x18 category u8 / +0x19 stage u8 /
  +0x38 identity ptr`。入口判定通常 `if (*(u8*)(ctx+0x18) > 1 && *(ptr*)(ctx+0x38)==self) stage=*(u8*)(ctx+0x19); else stage=2;`。

### 0.2 实读常量表（本文修正草稿 UNKNOWN）
| 符号/VA | 原始字节 | 值 | 语义 / 出处 |
|---|---|---|---|
| 0x142960674 | `0ad7233c` | 0.0099999998 (≈0.01) | 变更阈值 |
| 0x142925984 | `cdcccc3d` | 0.1 | roof 平顶阈（history_record）；亦作吸附/去重步长 |
| 0x142925870 | `0000003f` | 0.5 | NDC→屏幕偏移 / 下限 |
| 0x142925904 | `0000803f` | 1.0 | 上界 / 归一 |
| 0x1429258d8 | `9a99993e` | 0.30000001 | 无屋顶下限 / 吸附 |
| 0x1429cd064 | `00002040` | 2.5 | adjust 上限（预置形状） |
| 0x142a7de00 | `00006041` | 14.0 | adjust 上限（用户形状） |
| 0x1429cd068 | `cdcc4c3e` | 0.2 | Circle/Rect as_curve2 采样步 |
| 0x142a7baa0 | `08e53c1e` | **9.9999997e-21 (≈1e-20)** | do_dims 边长 L² 除零下限（`0x1e3ce508`） |
| 0x142a13848 | `00004843` | **200.0** | edit_rectangle ease 率（dims 源） |
| 0x142a7fa30 | `0000a041` | **20.0** | edit_rectangle ease 率（rotate 源） |
| 0x142a89600 | `1f856b3e` | 0.23 | edit_line / rotate 角基 |
| 0x1429edb90 | `0000403f` | 0.75 | edit_line / rotate 角系数 |
| 0x1429edb94 | `0000b442` | 90.0 | 角标尺（度） |
| 0x142a16d10 | `000020c1` | -10.0 | rotate gizmo ease 速率 |
| 0x142a16cb8 | `cdcc4c3f` | 0.80000001 | rotate fit 常数 |
| 0x142abcb68 | `8096184b` | 1.0e7 | rotate 最近点初值 |
| 0x142adf490[0] | `3333333f` | 0.7 | adjust 下限（slope>0.1，非平顶） |
| 0x142adf490[1] | `cdcccc3f` | 1.6 | adjust 下限（slope<=0.1，平顶） |
| 0x142ae0d44..0d60 | 见下 | 8×f32 | **fast-acos 8 系数（实读）** |

**fast-acos 系数（实读，`0x142ae0d44..0x142ae0d60`，LE u32）**：
```
[0] 0xbaa57a2c [1] 0x3bda90c5 [2] 0xbc8bfc66 [3] 0x3cfd10f8
[4] 0xbd4d8392 [5] 0x3db63a9e [6] 0xbe5bbfca [7] 0x3fc90fda(≈π/2)
```
算法：`x=|cos|; s=max(0,1-x); p=Horner(x; c0..c7); a=p*sqrt(s); result = if cos<0 { π-a } else { a }`，
符号再由 `cross = b.y*a.x - a.y*b.x` 的符号位决定（NaN 位型合成：`(cross & sign)|nanbits` 或 `nanbits`）。
**当前 `ui_edit_rectangle_orientation.rs:19` 的 `ACOS_COEFFS` 前 4 项错误**（`0x3bda90c5,0x3cd90a90,0x3c8bfc66,0x3dfd10f8`），
且 `fast_acos` 缺 `π-a` 负半支与符号选择——**必须按上表重写**。

跳表（实读 int32 相对偏移，`target = table_VA + (int32)entry`）：
- adjust `0x142aeba90`: case0→`0x140b30577`(跳过) case1→`0x140b304a1` case2→`0x140b30582` case3→`0x140b305e9`
- rotate-rect `0x142ae19b8`: case0→`0x140ad9b12` case1→`0x140ad919b` case2→`0x140ad93c3` case3→`0x140ad9a35`
- move `0x142ab7514`: case0→`0x14097c31e`(return) 1→`0x14097ac5b` 2→`0x14097ae9d` 3→`0x14097af78` 4→`0x14097ffb0`(halt/Debug)
- move 二级 `0x142ab7504`: 0→`0x14097983e` 1→`0x14097863a` 2→`0x140978861` 3→`0x140978917`
- move roof-snap `0x142ab7524`: 0→`0x14097ffc0` 1→`0x14097fb43` 2→`0x14098007d` 3/4/5→`0x14098094a` 6/7→`0x14097fb43`
- edit-freehand `0x142ae0cb8`: 0→`0x140ac9dc5` 1→`0x140aca18b` 2→`0x140aca3e0` 3→`0x140aca360` 4→`0x140acfded` 5→`0x140accd8a` 6→`0x140acce45` 7→`0x140accedb`
- edit-freehand 二级 `0x142ae0cc8`: 0→`0x140acfdfd` 1→`0x140accd9a` 2→`0x140acce55` 3→`0x140acceeb`

### 0.3 当前仓库缺失类型（grep 零定义；必须新建）
`OnShapeMoved`、`OnFreehandWallEdited`、`OnCircleRotated`、`OnRectangleEdited`、`RecalculateRoofCmd`、
`AnimatedCircleSignifierResource`、`UiEditRectangleActiveWidgetResource`、`WipRectangle`、`Slider3d`、
`DoUiEditRectangleDims`、`DoUiEditRectangleRotate`、`ClosestCornerCache`、`CircleGrabParams`、
`CircleSignifierParams`、`MoveShapeCache`、`MoveWallCache`、`EditFreehandWallCache`、`RotateCircleCache`、
`CurrentAngleGizmo`、`EditCircleSignifier`、`SignifierEditCircleShapeLine`、`EditRectangleLinePreview`、
`UiWallHeight::widget`/`signifier` 的 widget 状态类型。
已存在：`ShowPinnedDecoClutter`、`PlayWallUiSound`、`RaycastWorld`、`ShapeIntersections`、`Modifiers`、
`SignifierAnimation`、`UiState`(含 `add`/`dragged`/`intersecting`)、`FinishAdjWallHeightMode`、`InitAdjWallHeightMode`、
`AdjustWallHeightUi`、`GladeBorder`、`TerrainHeightsData`、`CursorTerrainRaycast`、`CameraCursorRay`、
`CursorPositionSS`、`MainCamera`、`GameTime`。

### 0.4 各叶输入源速查（ActionSetStates / UiState / 光标 / Local）
| 叶 | ActionSetStates | UiState | 光标/相机射线 | Local |
|---|---|---|---|---|
| ui_adjust_wall_height | 无（用 GameTime + func_0x140c16a00 拖动门） | widget 命中 ctx | 无 | 无（资源 UiWallHeight） |
| ui_adjust_wall_height_mode | 无 | 无 | CursorPositionSS+MainCamera+WindowSize+AudioCursorMoved | 无（资源 Cache） |
| history_record_adjust_wall_height | 无 | 无 | 无 | 无（资源 Cache） |
| move_gizmo_and_trigger_vol | 由调用者传 | 内部 `UiState::add` | 参数传入 anchor/dir | 无 |
| ui_move_shapes | 有（`digital[2]`=camera_rotate_mode，经 `move_gizmo` 读） | 选中 WallId | CameraCursorRay+RaycastWorld+CursorTerrainRaycast+AudioCursorMoved | `MoveShapeCache` |
| ui_move_freehand_walls | 有（`digital[2]`=camera_rotate_mode，经 `move_gizmo` 读） | widget epoch | CursorTerrainRaycast+AudioCursorMoved+MainCamera | `MoveWallCache` |
| ui_edit_freehand_walls | 无（PDB 无） | widget epoch | CursorTerrainRaycast+AudioCursorMoved | `EditFreehandWallCache` |
| ui_edit_circle | 无（PDB 无） | dragged/intersecting | CursorPositionSS+WindowSize+MainCamera+MainCameraMoved | `CircleGrabParams` `CircleSignifierParams` |
| signifier_edit_circle_arrow | 无 | 无 | 无（Query+资源驱动） | 无 |
| signifier_edit_circle_line | 无 | 无 | 无（Query+资源驱动） | 无 |
| ui_rotate_circle | 无（PDB 无） | 编辑 WallId | CameraCursorRay+WindowSize+MainCamera | `RotateCircleCache` `CurrentAngleGizmo` |
| ui_edit_rectangle_dims | 无 | 命中边 ctx | CameraCursorRay+MainCameraMoved | 资源 `UiEditRectangleActiveWidgetResource` |
| do_ui_edit_rectangle_dims | 无 | 无 | Slider3d 投影 | 无（`WipRectangle` 资源） |
| ui_edit_rectangle_rotate | 无 | 命中角 ctx | CameraCursorRay+MainCameraMoved | `ClosestCornerCache` |
| do_ui_edit_rectangle_rotate | 无 | 无 | CameraCursorRay+GameTime | 无 |
| signifier_edit_dims | 无 | 无 | 无（Query+资源） | 无 |
| edit_rectangle | 无 | 无 | 无 | 无 |
| finish_edit_rectangle | 无 | 无 | 无 | 无（`WipRectangle` 资源） |

> **ActionSetStates 槽号（已解）**：`ui_move_shapes`/`ui_move_freehand_walls` 的 `Res<ActionSetStates>` 由 `move_gizmo_and_trigger_vol` 读取 `digital[2].pressed()`（`+0x10`=digital[2]，`camera_rotate_mode`；pressed=`phase<3`）；§3/§4 只把它传给 helper。指令锚见 §2。其余叶子无该参数。

### 0.5 当前壳签名 ↔ PDB 参数映射（现状与缺口）
| 叶 | 当前 shell（文件:函数）参数 | PDB 参数量 | 关键缺失/错配 |
|---|---|---|---|
| ui_adjust_wall_height | 无系统函数（仅 `adjust_wall_height.rs: struct UiWallHeight` + 语义面方法） | 11 | 系统本体 + widget/signifier |
| ui_adjust_wall_height_mode | 12 参匹配（`ui_adjust_wall_height_mode.rs:191`） | 12 | 无（`LiveDecoratorArchivist` 同型别名） |
| history_record_adjust_wall_height | 7 参匹配（`...:310`，泛型 `<S>`） | 7 | 常量外置、`Result` 包装 |
| move_gizmo_and_trigger_vol | 不存在 | (helper) | 全缺 |
| ui_move_shapes | 不存在 | 30 | 全缺 |
| ui_move_freehand_walls | 不存在 | 22 | 全缺 |
| ui_edit_freehand_walls | 不存在 | 20 | 全缺 |
| ui_edit_circle | `(EventReader<EditCircleCmd>, EventReader<FinishEditCircleCmd>, ResMut<InnerWalls>, ResMut<CircleEditState>, EventWriter<RecordWallHistory>)` = 5 | 30 | 两事件为臆造；缺 25 个真实参数 |
| signifier_edit_circle_arrow | `(Res<PublicWalls>, ResMut<UiSignifierStream>)` = 2 | 3 | 数据源应为 `Query<(&EditCircleSignifier,&mut Transform)>` + `AnimatedCircleSignifierResource` |
| signifier_edit_circle_line | `(Res<PreviewCircle>, ResMut<UiSignifierStream>)` = 2 | 4 | 应为 Query + `AnimatedCircleSignifierResource` + `AssetMeshLibrary` |
| ui_rotate_circle | `(EventReader<RotateCircleCmd>, ResMut<InnerWalls>)` = 2 | 21 | `RotateCircleCmd` 臆造；缺 19 参 |
| ui_edit_rectangle_dims | `(Res<RectangleDimsEditInput>, Res<InnerWalls>, ResMut<RectangleEditUiState>, ResMut<PreviewRectangle>, EventWriter<EditRectangleCmd>)` = 5 | 11 | 输入源应 UiState/CameraCursorRay；缺 `DoUiEditRectangleDims`/`WallEntities`/`WallTriggerVolume`/`WallEntities`/`Commands` |
| do_ui_edit_rectangle_dims | 7 平铺实参（wall,edge,cursor,inner,state,preview,commands） | 12 | 缺 `PublicWalls`/`DecoratorArchivist`/`WipRectangle`/`Slider3d`/`GladeBorder`/4 事件 writer / reader |
| ui_edit_rectangle_rotate | 6 参（`RectangleOrientationEditInput`+`InnerWalls`+`RectangleEditUiState`+`PreviewRectangle`+2 writer） | 10 | 输入源/`ClosestCornerCache`/`DoUiEditRectangleRotate` 缺失 |
| do_ui_edit_rectangle_rotate | 7 平铺实参 | 12 | 缺 `WipRectangle`/`GameTime`/`GladeBorder`/4 writer/reader；acos 系数与 π 补错 |
| box_trigger_volume | 现有 `box_trigger_volume`/`skewed_box_trigger_vol` | (helper) | EQUAL（保留） |
| signifier_edit_dims | `(Res<PreviewRectangle>, ResMut<UiSignifierStream>)` = 2 | 4 | 应 Query + `UiEditRectangleActiveWidgetResource` + `AssetMeshLibrary` |
| edit_rectangle | `(EventReader<EditRectangleCmd>, ResMut<InnerWalls>)` = 2 | 5 | 缺两个音频 writer、`Res<Time>`、ease |
| finish_edit_rectangle | `(EventReader<FinishEditRectangleCmd>, ResMut<RectangleEditUiState>, Res<InnerWalls>, EventWriter<RecordWallHistory>)` = 4 | 9 | 缺 `WipRectangle`/`DecoratorStorage`/`DecoratorArchivist`/3 writer |

### 0.6 语义 vs 布局 标注约定（实现线必读）
- **`[SEMANTICS-DECODED]`**：该字段/分支的含义来自指令（用于分支/算术/状态转移/事件选择）。实现线可据此写业务逻辑。
- **`[FIELD-NAMES-SYNTHESIZED]`**：仅类型/偏移/大小已知，字段只被搬运、未被解释；名字是实现层合成，**不得当作语义证据**。实现线只能搬运，不得据此写判断逻辑。
- 每个类型/事件块标其一。READY 仍需控制流、槽位、事件载荷、写读者齐全；类型可建国 ≠ 叶子 READY。

---

## 1. adjust_wall_height 三件套

### 1.1 `ui_adjust_wall_height` — VA `0x140b2fc00` / 3680B（11 参）
PDB 11 参（权威）：
1 `Res<AppMode>` · 2 `Res<PublicWalls>` · 3 `Query<&Roof>` · 4 `Res<UiState>` ·
5 `EventWriter<FinishAdjWallHeightMode>` · 6 `EventWriter<AdjustWallHeightUi>` ·
7 `EventWriter<InitAdjWallHeightMode>` · 8 `Query<&RoofAnimation>` · 9 `Res<UiSignifierStream>` ·
10 `EventReader<MainCameraMoved>` · 11 `Res<GameTime>`。

> **纠正草稿**：param_10 是 `EventReader<MainCameraMoved>`（PDB 权威），不是"墙选 pick 事件"；反编译中
> param_10 的 reader 游标机制（`events[+0x10/+0x18/+0x30/+0x38/+0x40]`）确认它是 Bevy `EventReader`。
> 该 reader 不产生墙选择，只用来算"相机是否有新移动"布尔 `has_new`。

**反编译形参 → PDB 映射（逐项确证）**：
| PDB# | 类型 | 反编译锚点 |
|---|---|---|
| 1 | `Res<AppMode>` | `mode = *(u32*)*param_1`；`wall = *((u64*)*param_1 + 2)`（载荷=当前墙 WallId） |
| 2 | `Res<PublicWalls>` | `plVar33 = *param_2`（SwissTable 0x2a8 探针；`FUN_140a923a0(plVar33,..)` self） |
| 3 | `Query<&Roof>` | 作为第 3 实参传入 `PublicWalls::roof_component(plVar33, wall, param_3)` |
| 4 | `Res<UiState>` | `param_4 = *param_4`，作为 `UiWallHeight::widget` 的 out 实参 |
| 5 | `EventWriter<FinishAdjWallHeightMode>` | `lVar10=*param_5; tick=*(param_5+0x1c); grow=param_5[2]` |
| 6 | `EventWriter<AdjustWallHeightUi>` | `puVar39=*param_6; tick=*(param_6+0x1c); grow=param_6[2]` |
| 7 | `EventWriter<InitAdjWallHeightMode>` | `lVar12=*param_7; tick=*(param_7+0x1c); grow=param_7[2]` |
| 8 | `Query<&RoofAnimation>` | `lVar27=*param_8; lVar7=param_8[1]`（`+0x1a8` 表 0x48 步长、`+0x38/+0x110/+0x28` 索引/位图） |
| 9 | `Res<UiSignifierStream>` | `uVar9 = *param_9` → `FUN_140956cd0(...,uVar9)` 末参 |
| 10 | `EventReader<MainCameraMoved>` | `uVar29=*(u64*)*param_10; lVar27=param_10[1]`（cursor/events 双缓冲） |
| 11 | `Res<GameTime>` | `lVar8=*param_11; *(u32*)(lVar8+0x2c)` → `FUN_140956cd0` 第 4 实参 |

**输入**：`AppMode`（Editing(Wall(wall))，载荷 wall id）；`MainCameraMoved` reader（重算触发）；`UiState`（widget 命中上下文）；`GameTime`（+0x2c 传 signifier）。
**输出**：`UiSignifierStream`（抬起预览 + widget）；三事件 writer；`UiState::add` 注册 widget。
**完整控制流（`adjh_body.c` + `decomp-0x140b2fc00.txt` 实读）**：
```
src = func_0x140c16a00()                     // TLS once / 源路径；src!=0 → .rdata 扫描分支
if src != 0 { /* UNK_142aeb2d4/142aeb8d8 UTF-8 续位找 '/' '\'; Events next/refresh */ return }
mode = *(u32*)*param_1
if mode >= 6 { goto tail }
// MainCameraMoved reader 游标推进：
ev = param_10[1]; cur = *(*param_10)
new_a = cur - ev[0x18] (clamp>=0); new_b = cur - ev[0x38] (clamp>=0)
has_a = new_a <= ev[0x10] && (ev[0x10]-new_a)!=0
has_b = new_b <= ev[0x30] && (ev[0x30]-new_b)!=0
has_new = has_a || has_b                       // = uStack_9a（widget 第 4 实参）
*(u64*)*param_10 = ev[0x40] - (has_new ? 0 : new_a+new_b)
wall = *((u64*)*param_1 + 2)
// PublicWalls 探针(0x2a8) 找到 key==wall 的 entry；断言 entry[-0x290]>=2 && entry[-0x270]>0
//   gate1: entry[-9] (byte)==0      // 未锁定（草稿旧记 value[+0x296]==0）
//   gate2: entry[-0x48] (u32)>=2    // shape count（草稿旧记 value[+0x258] tag）
anim = RoofAnimation 查询值 (fVar3，经 param_8 的 +0x1a8/+0x38 表)
roof = PublicWalls::roof_component(param_2, wall, param_3)     // FUN_140a923a0；可能为 0
flat_w_no_rim = roof != 0 && roof[+0x3d] != 0 && roof[+0x30] <= 0.1
if !(anim != 1.0 && entry[-9]==0 && entry[-0x48]>=2) { goto tail }
state = { wall@+0:u64, flag8@+8:u8=bVar25, flag9@+9:u8=bVar25^1, ... }   // 正常路径 bVar25=0 → +9=1
widget = UiWallHeight::widget(state, ui_state_out, wall, has_new, flat_w_no_rim)   // FUN_1409566d0
if (bVar25==0 && !has_new) && (widget[+0x18]<2 || widget[+0x38]!=self || widget[+0x19]==2) { goto tail }
UiWallHeight::signifier(state, wall, screen_ctx, *(u32*)(*param_11+0x2c), stream)   // FUN_140956cd0
stage = (widget[+0x18]>1 && widget[+0x38]==self) ? widget[+0x19] : 2
b = min(stage-2, 3)                            // 跳表 0x142aeba90
switch b {
  0 => {}                                      // 无写入
  1 => { AdjustWallHeightUi.push16(WallId); InitAdjWallHeightMode.push8(ZST-id) }
  2 => { AdjustWallHeightUi.push16(WallId) }
  3 => { FinishAdjWallHeightMode.push16(WallId) }
}
tail: return
```
> `code_0x140b30796/307d0/307e0/307c4/3081f` 为各断言/未命中早退；`code_0x140b306a0` 是 `.rdata` 分支的计数自增返回。
> `entry[-0x48]` 同时用于 gate2 与 case3 的 skip（`bVar25=1`），故"预置形状"与"shape count"是同一字段。

> **交叉核验（2026-10-27 实现线独立机器码解码，与本节一致）**：跳表 `0x142aeba90` = `[0→0x140b30577(空), 1→0x140b304a1, 2→0x140b30582, 3→0x140b305e9]`；
> 语义 **case1=AdjustWallHeightUi+InitAdjWallHeightMode，case2=AdjustWallHeightUi，case3=FinishAdjWallHeightMode，case0=空**；
> 索引 `clamp(hover_tag-2,0,3)`，`hover_tag` 仅当 `UiState.byte[+0x18]>=2 && qword[+0x38]==wall` 取 `byte[+0x19]`，否则默认 2。
> 门控：roof 浮点须 ordered 等于 1.0；`roof.height_01(+0x30) <= 0.1`；`roof.byte[+0x3d] != 0`。
> **旧草稿 `rectangle_and_adjust_height.md §3.6` 的"1=finish,2=adjust,3=init"顺序是错的，以本节为准。**

> **PublicWalls 内部偏移（`+0x48` 二级表、`+0x1a8`、`+0x110`、entry stride `0x2a8`、entry 字段 `-9`/`-0x48`/`-0x290`/`-0x270`）不要求照抄字节偏移。**
> 实现线应使用 `PublicWalls` 的公开 API 实现等价语义：`PublicWalls::get(wall)` 取墙记录、`simplified_curve_2d` 的有效性等价于 `curve.is_valid()`（`-0x290`/`-0x270` 断言），
> `shape count >= 2`（`-0x48`）对应 `PublicWallState` 的形状计数/预置形状判定，`-9` 对应该墙"是否处于可编辑/未锁定态"的布尔。若无法从现有公开方法直接取到某一字段，**需补充哪条原写入点**：
> `PublicWalls` 发布点（`publish_public_wall_state` 写 `simplified_curve_2d`）与 `WallState` 的 shape-count 写入点；不得按偏移强转。

**事件载荷（producer=本系统；reader=`ui_adjust_wall_height_mode` / `history_record_adjust_wall_height`）**：
| 事件 | 行宽 | 载荷 |
|---|---|---|
| `FinishAdjWallHeightMode(pub WallId)` | 16B | `EventInstance`: `+0 WallId u64, +8 event_id u64` |
| `AdjustWallHeightUi(pub WallId)` | 16B | `+0 WallId u64, +8 event_id u64` |
| `InitAdjWallHeightMode`（ZST/unit） | 8B | 仅 `event_id u64` |
三类型当前仓库均已存在且正确（`resources/events.rs`、`ui_adjust_wall_height_mode.rs`）。

**两个被调用 helper 的合同（在本模块内实现）**：
- `UiWallHeight::widget`（`0x1409566d0`，参数 `(state, out_ui_state, wall, has_new:bool, flat_w_no_rim:bool)`）：
  - 由 `wall` 的曲线点（`+0x90` ptr / `+0x98` len，0xc B/点）与高度（`+0xe0` ptr / `+0xe8` len）生成抬起点数组，`y = point.y + lift`；
    lift：`state+9==0 → +0.6(0x142925908)`；`state+9!=0 && flat==0 → +0`；`state+9!=0 && flat!=0 → -0.29(0x142a94920)`。
  - 颜色参数：表 `0x142ab3c08=[0.7,0.5]`，下标 `(shape_tag<2) as usize`（`shape_tag=*(u32*)(wall_state+600)`）。
  - 名字前缀 `0x142ab47a8="edit_height_"`（format 闭包 `FUN_140b34ec0`）；`FUN_140d79f90`(format)→`FUN_1409231f0`(`UiState::add`) 注册 widget，写回 `out_ui_state`。
- `UiWallHeight::signifier`（`0x140956cd0`，参数 `(state, wall, screen_ctx, game_time_scalar:u32, stream)`）`[SEMANTICS-DECODED]`：
  - `state+9==1`（正常路径，ui_adjust_wall_height 走此支）：`if state+8==0` 先门控 `ctx=*screen_ctx`；
    `ctx[+0x18]==0 || ctx[+0x38]!=ctx_ref || ctx[+0x28]==2 → return`。
    否则建 **0x20 记录**（`FUN_140613c10(0x20,8)`）：`{ +0x00 u64=1, +0x08 u64=1, +0x10 u64=*(state+0), +0x18 u32=angle }`（+0x1c padding）。
    - 字段3 = `*(state+0)` = **WallId**（adjust 调用者令 `state+0 = lVar14` = PublicWalls 命中的 key=WallId），**不是 curve_ptr**。
    - `angle = if *(state+8)==0 { 0.2(0x1429cd068) } else { 0.0 }`；adjust 正常路径 state+8=0 → **angle=0.2**。
    - 名字 `0x142ab4820="wall_height_edit_"`；push marker `0x142ab4830`（`FUN_14146c880`）。
  - 另一路径（`state+9!=1`，其它 height 调用者）：由 `wall+0x90/+0x98/+0xe0/+0xe8` 生成 lift `+0.6` 点数组，建 **0x28 记录** `{1,1,len,points_ptr,len}`（点指针在此支，不是 0x20 支）。
  - 分配失败 → `FUN_1428d9000` panic。
  > **回答实现线**：0x20 记录不是 `{1,1,curve_ptr,angle}`；正确为 `{1,1,WallId,0.2}`（正常路径）。`SignifierFactory::new(name,null,0)` 应保留载荷 `{kind=1, kind2=1, WallId, angle=0.2}`（按 SIGNIFIER-CONTRACT 走真实工厂）。
- `UiWallHeight::widget`（`0x1409566d0`）写入 WidgetParams 的标量 `uStack_b4` `[SEMANTICS-DECODED]`：
  - 正常路径（`state+9==1`）→ `uVar10 = color_tbl[shape_tag<2]`，`color_tbl@0x142ab3c08 = [0.7, 0.5]`；**tag>=2→0.7，tag<2→0.5**。
  - 仅 `state+9==0`（其它调用者）才用固定 `_UNK_142925870 = 0.5`。
  > **回答实现线**：实现假设的"半径常量 0.5"**不成立**——该标量是颜色/env 值（audit 亦标 `0x142ab3c08` 为颜色参数），widget 命中几何来自抬起的曲线点（`wall+0x90/0x98` 点 + `+0xe0/0xe8` 高度），无独立半径常量；正常路径应取 0.7/0.5（按 `shape_tag<2`），而非固定 0.5。
- **gate1 `entry[-9]`（可编辑/未锁定布尔）** `[SEMANTICS-DECODED / RESOLVED]`：
  - 用途实读：`*(char*)(entry-0x9) != 0 → 跳过（不编辑）`；`entry-9` = 值记录末尾附近 1 字节。
  - **字段路径（已闭合）**：`entry-9` = 值起始 + **0x297** = `PublicWallState.being_bulldozed: bool`。依据 `public_wall_state.rs:363`：`bytes[0x297] = self.being_bulldozed as u8;`（紧邻 0x295=freehand_ty、0x296=source_flag_65）。即 gate1 = `!being_bulldozed`。
  - **写入者**：`publish_public_wall_state`（原 VA **`0x140a8b530`**，`public-symbols.json` 具名；原 publisher 投影写区间 `140a8d825..140a8d914`）。当前 repo `publish_public_wall_state.rs:161`：`state.set_being_bulldozed(bulldozing.contains(id))`——每次 public 行重建时按 bulldoze 动画队列成员资格写入。
  - 结论：实现线直接用 `PublicWallState::is_being_bulldozed()`（等价 `entry-9 != 0`），无需按偏移强转。

**批次 1 合同问题答复（实现线 51 passed 后）`[SEMANTICS-DECODED]`**：
1. **widget 命中几何 = `SphereCloud{centers=lifted 点, radius=env}`（审查线正确；我此前"TriangleMesh"判断错误，逐指令如下）** `[SEMANTICS-DECODED]`：
   - **载荷 stride 12B（Vec<Vec3>，非 Vec<Triangle>36B）**：抬起点循环 `0x1409568c0`/`0x140956990` 按 `movss [rax+r8*4-0x14]=x / [-0x10]=y+lift / [-0xc]=z`、`r8+=6`（2 点=24B → 12B/点）。故命中体是点云，不可能是 `TriangleMesh(Vec<Triangle>)`。
   - **体积判别 byte = 1（SphereCloud）**：`0x140956b11` `mov byte [rbp+0x70],1`；半径 `0x140956b15` `movss [rbp+0x74],xmm6`；centers Vec `[rbp+0x78]=len,[rbp+0x80]=ptr,[rbp+0x88]=len`（结构基址 `[rbp+0x70]`：tag@+0、radius@+4、centers@+8/+0x10/+0x18）。tag=1 对应 SphereCloud（TriangleMesh=2）。
   - **`0x202` 在 `WidgetParams+0x78`（extra_78），不是 volume tag**：`0x140956bed` `mov word [rbp+0x20],0x202`；add 调用 `0x140956c1c/0x140956c18`：`rcx=rsi(self)`、`rdx=&[rbp+0x90]`(name String)、`r8=&[rbp-0x58]`(params 基址) → `[rbp+0x20]-[rbp-0x58]=+0x78`。即 extra_78（brush-preview enum tag2），与几何无关。
   - **env 值 `0x142ab3c08[shape_tag<2]=[0.7,0.5]` 就是半径**：读取点 `0x140956b05` `lea rdx,[rip+0x215d0fc]`(=0x142ab3c08) + `movss xmm6,[rdx+rcx*4]`（`rcx=(*(u32*)(wall+0x258)<2)`），随即 `0x140956b15` 存入半径 lane。**全 evidence 反编译语料中 `142ab3c08` 仅出现在 `decomp-0x1409566d0`（无其它读者）**，故 audit 的"颜色"标签无支持；它只流入 `SphereCloud.radius`，即半径/env 值。
   - **`FUN_140b34ec0` 不是网格工厂**：仅 0x60B，`call 0x140d85ad0`（fmt），产出名字 String，无三角化；全 `.text` 无直接 caller。
   - 点源：`wall+0x90`(ptr)/`+0x98`(len) + `wall+0xe0`(ptr)/`+0xe8`(len)；`y += lift`（`state+9==0`→+0.6；否则 `flat`→−0.29 / 非 flat→0）；Transform=单位（点已世界坐标）。
   - **结论**：`SphereCloud{centers=lifted, radius=env}` 是原版语义；当前实现方向正确，env 就应在 radius 通道。
2. **signifier 所有权：按本项目 `SIGNIFIER-CONTRACT`，必须走 `SignifierFactory::from_closure`/`write_factory`，删除数字 Raw 与 5-qword `write_raw`；原版数值记录不构成豁免** `[SEMANTICS-DECODED]`：
   - 原版 `0x140956cd0` 正常路径的语义（供闭包复刻）：记录 `{+0x10 WallId, +0x18 angle}`，`angle = if state+8==0 { 0.2(0x1429cd068) } else { 0.0 }`（adjust 正常路径 state+8=0 → 0.2）；名字前缀 `0x142ab4820="wall_height_edit_"`（`FUN_140d79f90` 格式化），marker `0x142ab4830`；前置门 `ctx[+0x18]!=0 && ctx[+0x38]==ref && ctx[+0x28]!=2`。
   - **闭包合同**：`SignifierFactory::from_closure( name = format!("wall_height_edit_{wall}"), capture = { wall: WallId, angle: 0.2, curve = PublicWalls[wall].simplified_curve_2d 的点 } )`；marker/kind = `0x142ab4830`；闭包生成的实体 = 沿墙抬起曲线、在拖拽角度处的"高度编辑"signifier（线/网格本体）。**精确 mesh/线的顶点构造需 `0x142ab4830` marker 的消费者反编译**（当前未取）——在此之前按 §7/§8 的 signifier 实体模式构造并对齐可见几何，标 FIELD-NAMES-SYNTHESIZED。
3. **`UiState[+0x18]/[+0x38]` 写入者 = `ui_update`**（`systems/ui/ui_update.rs:128`；原 set6 在 `hover_intersection` 之后）：末尾 `state.replace_interaction_full(owner.interaction,...)` 写 `+0x18 native_tag / +0x19 native_phase / +0x38 widget id`。`hover_intersection`(`hover_intersection.rs:242`) 先用相机射线对 widget 命中体（本例为 `SphereCloud` 点云/体积）求交写入 `hover_intersections`；`ui_update` 取首个命中→`enter_hover`(tag=1)→点击 `digital[0].just_pressed`→tag=2/phase=3(capture)→phase=4(drag)→release 时 phase=|hit==widget|。
   - **测试最小前置**：① 每帧由 producer 经 `UiState::add` 注册 widget（本例 `SphereCloud` 命中体）；② `hover_intersection` 产生**真实 intersect**（payload[0]!=2）；③ `ActionSetStates.digital[0]` 走 just_pressed→pressed→just_released；④ `WidgetParams.sounds.slots` 就绪（`ui_update` 读它发 PlayUiSound）；⑤ `ui_update` 末尾 `clear_widgets()` → producer 必须**每帧先注册**。

**批次 2 合同答复（marker 消费者 + 双 producer）`[SEMANTICS-DECODED 部分]`**：
1. **`0x142ab4830` marker = signifier 载荷的类型描述符**（不是字符串）：表 `0x142ab4830` x64 = `{0x0, 0x10(size), 0x8(align), 0x14094f4b0(drop), 0x1409576d0, 0x1409576d0}`；其后 `0x142ab4850` 为源路径 fat→**`crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs`**。即该 signifier 载荷类型定义在 `adjust_wall_ui_mode` 模块（`UiWallHeight`）。`0x14094f4b0` 是小包装，实际 worker=`0x1409576d0`（分配 8B + 0x13B 缓冲、`0x8000000000000001` 哨兵）——**实体构造在这里，但完整 mesh/Transform/动画需 `0x1409576d0` 全文反编译**（当前未取）。
   - producer 在 `UiWallHeight::signifier`(0x140956cd0) **内联**生成载荷（非独立闭包）；通用 signifier 流消费者按此 descriptor 生成实体。实体 = `adjust_wall_ui_mode.rs` 定义的墙高编辑 signifier。
2. **双 producer 仲裁（原版确有双 producer，不是我们的命名错误）**：`ui_adjust_wall_height`(0x140b2fc00) 与 `ui_focus_mode_shape_height`(0x140a623e0) **前缀相同**——`0x142ab47a8` 与 `0x142ad4210` 的 fat 都指向 `"edit_height_"`（len 12），且都用 format 闭包 `FUN_140b34ec0`。
   - **focus 的 PDB SystemParam（12 参，`skeleton/B/.../ui_focus_mode_shape_height.rs:1124`）确证含**：
     `Res<AppMode>, PublicWalls, WindowSize, MainCamera, UiState, EventWriter<FinishAdjWallHeightMode>, EventWriter<AdjustWallHeightUi>, EventWriter<InitAdjWallHeightMode>, EventWriter<HandleRaiseShapeUiInteraction>, UiSignifierStream, GameTime, Local<LerpedGizmoPosition>`。
     → focus **也写 Init/Adjust/Finish 三事件**（另加 HandleRaiseShapeUiInteraction）。故"Init 双发音频"是**原版行为**，不是重建引入。
   - 两者 gate 相同：`mode<=5 && mode∈{1,2}`（`0x140a623e0` 行 172）。
   - 几何不同：adjust=`SphereCloud` radius `0x142ab3c08[shape_tag<2]`；focus=radius **0.5**(`0x3f000000`)+tag `0x100`+`xvy` swizzle 的 0x18 载荷（`uStack_1f8=2`，行 490-522），并有 `Local<LerpedGizmoPosition>`（gizmo 位置缓动）。
   - **运行期仲裁（待定项）**：因前缀同、事件同，区分靠 format 捕获后缀 + widget 命中体 + 各自 `UiWallHeight::signifier` 前置门（`ctx[+0x18]!=0 && ctx[+0x38]==widget_ref`）。**需要哪条证据**：① focus format 捕获值（`pcStack_108`）是否=WallId（决定后缀是否同名冲突）；② 两系统体内对 AppMode 具体值（1 vs 2）的进一步分支（若各处理不同子态则天然互斥）；③ focus 的 `Local<LerpedGizmoPosition>` 与 adjust 的 `UiWallHeight` 是否注册到**同一 widget id**（若是，则原版按"最后一个注册者"仲裁）。

**批次 3（marker 消费者全文 + focus 仲裁，Ghidra 反编译）`[SEMANTICS-DECODED 部分]`**：
1. `FUN_1409576d0`（117 行）是 signifier 工厂的**实体构造体**，产出 **1 个**实体：
   - 分配字符串 **`"vertex_color_shader"`**（串基 `0x142a9948c`）+ 源路径 `crates/country-core/src/start...`。
   - 构建 0x28 材质记录：`{ +0x00 = FUN_1412dc8f0, +0x08 = (uStack_4c,uStack_50), +0x10 = 0x4040000000000001, +0x18 = 0, +0x20 = FUN_141564d30 }`（两个 fn 指针=材质/绘制回调；`0x40400000`=3.0）。
   - `FUN_14089dd50`（builder_1，16015B decomp）按载荷 `param_2[2]` tag **switch** 出四类通用 signifier 网格：串 `"signifier empty"`/`"signifier custom"`/`"signifier custom w colors"`/`"signifier custom fn"`（`0x142a99570/b3/c3/e8`），属性 `"Vertex_Color"`（`0x142a995dc`）——即**通用顶点色 signifier 网格**，非墙高专属几何。`FUN_1412ddf90`（builder_2）构 0x80 材质/绘制描述符（fn `FUN_1412d73f0`/`FUN_141564d30` + f32 对）。
   - **构造合同（已由 worker `0x1409576d0` 反编译确证，取代此前"传抬起点"的错误合同）**：worker 组装的 builder 载荷（`&uStack_108`）为**常量**：`[0..1]` = 四元数常量 `0x14293e0c0 = {(0,0,0), 1.0}`（identity 旋转 + 1.0 缩放）、`[2]` = tag `0x8000000000000001`（builder switch tag = `param_2[2]^0x8000…` = **1**）、`uStack_c8=0x101`、`uStack_c4=0x13f800000`(=1.5)、计数 `0x13`；随后追加 0x28 材质元素 `{+0x00 FUN_1412dc8f0, +0x08 f32 对, +0x10 0x4040000000000001(=3.0/1), +0x18 0, +0x20 FUN_141564d30}`，再 `FUN_1412ddf90(...,&0x142ab48b0)`。worker 只读 `record[0]/[1]`(=1,1)，**不读 WallId/angle，也不转发墙抬起点/墙色**。→ **墙高 signifier = 固定常量 handle 指示物**（identity Transform、scale 1.0、常量材质回调），**不是抬起的墙曲线、也不逐顶点取墙色**；实现应改为固定常量 handle，`PointsWithColors(points=lifted)` 是错误做法。
   - **builder 分支（tag→branch，已判）**：builder switch = `switch(param_2[2] ^ 0x8000000000000000)`；wall worker 的 `param_2[2]=0x8000000000000001` → **tag=1 → `case 1` = `"signifier empty"`**（字符串 `0x142a99570`，名长 `0xf`=15）。case 1 的 **mesh 源 = 内置资产 `FUN_1400d9b20(0)`**（asset lookup，带 refcount）+ 常量块 `0x142ac64c0..0xd8`，再 `FUN_14092cc60` 构建；外层的 `0x13`(=19) 是 builder 外层计数，非 case1 几何。⇒ **墙高 signifier = "signifier empty"（内置 empty 资产，无墙几何/无墙色）**。
   - **结案（已证，非占位）**：墙高 signifier = **`"signifier empty"`（内置 empty 资产，无墙几何、无墙色）**。原版**不使用**颜色数组、不使用 `angle` 参与几何 —— 颜色/angle 的"占位"说明作废；实现用 `SignifierMesh::Empty` + identity Transform（scale 1.0）即**与原版一致**，不是占位缺陷。record `{WallId,angle}` 仅作身份（`wall_height_edit_{..}` 名/查找键）。
   - 返回 `Vec<Entity>`：`*param_1=1; param_1[1]=puStack_40; param_1[2]=1`（= 1 个实体）。
2. **focus `pcStack_108` = AppMode 载荷 = 当前 wall id**（`0x140a623e0` 行 171-179）：`uVar7=*(u32*)*param_1`（mode）；`pcVar12 = *(u64*)((u64*)*param_1 + 8)`（AppMode 第 2 qword）；`pcStack_108=pcVar12`；`hash_one(PublicWalls+0x20,&pcStack_108)` 用它查 PublicWalls。
   - ⇒ adjust 与 focus 对**同一 wall** 都格式化为 **`edit_height_{wall}`（同前缀+同 wall id）→ 同一 widget 名/WidgetId**。原版即"同名覆盖/按注册顺序"，几何取最后注册者（adjust=SphereCloud，focus=Sphere）。**"双 producer + 同名"是原版行为，不是重建命名错误**；实现保持即可，但需知 hover 命中体取决于最后注册的 geometry。

**批次 4：阶段/phase 机器关键裁决（阻塞 finish 腿）`[SEMANTICS-DECODED]`**：
1. **`stage` = `UiState+0x19` 字节**（指令级确证）：`UiState::add`（`0x1409231f0`）= 加锁 RwLock@`UiState+0x60`、把 0xb0B `WidgetParams` memcpy 后插入 `UiState+0x68` 的 widgets 表、**返回 `param_1`（UiState 指针本身）**；`UiWallHeight::widget`(0x1409566d0) 把 add 的返回值（=UiState）回传；adjust 读 `[ret+0x19]` → **stage = UiState+0x19 = DragState 字节**（repo `into_drag_state 0x1428df10` 直取该字节）。
2. **adjust 的 `b=clamp(stage-2,max3)` 与 `ui_update` 的 `bVar15=clamp(pauStack_78[1][9]-2,max3)` 逐字相同**（`uiupd.c` 554-557：`bVar10=phase-2; bVar15=3; if(bVar10<3)bVar15=bVar10;`）。二者共用同一阶段机；阶段字节仅由 drag 机器写。
3. **`ui_update`（`0x140b2de60`）PDB 8 参**：`ResMut<UiState>, Res<ActionSetStates>, EventReader<WindowFocusEvent/EscapeMenuJustOpened/ShutdownGameCmd>, EventWriter<OverwriteBrushPreview/OsCursorVisibilityCmd/PlayUiSound>` —— **无 `FinishAdjWallHeightMode` writer**。故 finish **不是** ui_update 发的事件，而是 adjust 直接读 `+0x19`。
4. **阻塞点（审查线更正：无符号回绕，finish 可达）**：adjust `bVar25 = cVar54 - 2; if (2 < bVar25) bVar25 = 3;`（`bVar25`/`cVar54` 为 **byte**，比较**无符号**）：stage 0→`0xFE`、1→`0xFF` 均 `>2` → b=3 → **case3(Finish)**；stage2→0(case0 skip)、3→1(case1 = AdjustUi+Init)、4→2(case2 = AdjustUi)、>=5→3(Finish)。即 **stage {0,1,>=5} → Finish**。**repo 用 `saturating_sub`（把 -2 钳成 0）是错误**；应改为无符号回绕减法 + 无符号 `>2` 比较。**5 从不被任何系统写入**（`.text` 常量扫描 `mov byte[+0x19],5` 为负；124 个 `[x+0x19]` 写点均为**寄存器源 → 调用者计算值**），故 `≥5→finish` 只是回绕数学结果，不是独立状态。
   - **3/4（drag start/continuing）写入者**：`ui_update` 全函数**仅一处** phase 写（0/1，`0x140b2e7d7`）。3/4 由 drag 拥有的**工具系统**算好后经 commit helper（`0x140b2fad5`：`mov byte[rcx+0x18],2; mov byte[rcx+0x19],al(=caller local)`；**无直接 caller → 内联进各工具**）提交。repo 把 phase 计算收敛进 `ui_update` 是结构性简化，但产出 {0,1,3,4} 经无符号 clamp 后行为等价（3→init/adjust、4→adjust、0/1→finish）。
   - **需确认（非阻塞）**：`DragState` 是否恰为 {0=CANCELLED,1=FINISHED,2=NONE,3=STARTED}（repo 假定）；若原版另有 4（continuing），其写入点=工具侧 commit 内联体。

**批次 6：signifier 消费者链现状（signifier_create + descriptor.call）`[SEMANTICS-DECODED 部分]`**：
- `signifier_create` = **`0x140b2cfa0`**（`public-symbols` 具名；556 行）——流驱动系统：迭代 stream 项，按 0x28 步长写结果（`sc_1.c` 340-344），对每项调用其 **type-descriptor 的 `call`**（间接）。`UiSignifierStream::write`=`0x140b2cf90`（无独立函数体，已内联）。`SignifierFactory::new`=`0x14089d8b0`。
- **producers 不建 Factory**：`UiWallHeight::signifier`(0x140956cd0) 与 arrow(0x140b35120) **都不调用** `0x14089d8b0`——它们 push **raw 记录 + type-descriptor marker**（`FUN_14146c880` = AppendOnlyVec::push）；marker 的 `call` = worker（wall_height `0x1409576d0`，arrow `0x140b35650`）。
- **wall_height worker `0x1409576d0` 的实体构造**：builder 载荷主要由**常量**组成——Transform/四元数常量块 `0x14293e0c0 = {0,0,0,1.0}`（identity 旋转 + 1.0 缩放）、tag `0x101`、标量 `0x13f800000`(=1.5)、计数 `0x13`；并转发 `param_2[0]/[1]`（=记录 {1,1,…}）。**它并不把 wall 的抬起点/颜色表转发给 builder** → 修正此前"传抬起曲线点"的合同：**墙高 signifier 实体是一个由常量定义的固定小 handle 指示物，不是抬起的墙曲线，也非逐顶点取墙色**。
- **4 条载荷问题的现状**：
  1. **颜色**：builder 的 `Vertex_Color` 用 payload 颜色数组（`param_2[5]/[6]/[7]`）；但 wall_height worker 未从墙色表填该数组（其载荷为常量），故 **墙高 signifier 很可能无真实逐顶点色（走常量/无 colors 支）**——实现"白占位"可能本就可接受，**需确认 worker 选中 builder 的哪个 branch（tag）**。
  2. **angle(0.2)**：builder 不读；worker 只读 `param_2[0..1]`（=1,1）。→ angle 在 worker 里**未被使用**（或用于未展开的 branch）；倾向"仅记录/身份，未参与几何"。
  3. **§7 points**：arrow worker 转发 `param_2[3]/[4]`；但 wall_height worker 用常量。两者不同；arrow 的 [3]/[4] 仍需确认来源。
  4. **WidgetId 捕获**：captured=`*param_2`（资源）；仍待确认偏移。
- **需要哪条证据（统一）**：`signifier_create` 对 descriptor 的**间接 call 点**（哪个寄存器传 record 指针、record↔builder payload 的字段映射）——当前 `sc_1.c` 的 grep 未见直接 `FUN_1409576d0/140b35650/14089dd50` 调用，说明经 vtable 间接；需对 `sc_1.c` 的间接 `(*(code*))(...)` 调用点与 record 布局逐字段对读，才能定 angle/points/color 的最终来源。

**常量（RNE 实读，见 §0.2）**：0.6(`0x142925908`=`0x3f19999a`)、-0.29(`0x142a94920`=`0xbe947ae2`)、0.2(`0x1429cd068`)、1.0(`0x142925904`)、0.1(`0x142925984`)、半径/env 表 `0x142ab3c08=[0.7,0.5]`。

**当前仓库映射/差异**：系统函数**缺失**（`adjust_wall_height.rs` 只有 `struct UiWallHeight`+`lifted_points/widget_name/signifier_name/env_scalar/guard` 语义面方法；`adjust_wall_height_impl.rs` 是另一系统）。需删除/降级语义面方法；已可复用 `PublicWalls`/`Query<&Roof>`/`PublicWalls::roof_component`/三事件/`UiState::add`/`UiSignifierStream`。
**测试**：mode>=6 早退；无新相机事件且 widget 非活动→不写事件；stage 2/3/4/5→对应 writer（1→AdjustUi+Init，2→AdjustUi，3→Finish）；`flat_w_no_rim` 三态 lift；颜色表 tag<2 分支；断言失败早退；真实链需 host 相机。
**STATUS: READY**（系统本体/helper 合同/常量/事件/门控/字节布局均已自证；widget 命中=`SphereCloud{centers=lifted 12B/pt, radius=env 0x142ab3c08[shape_tag<2]}`（体积 tag=1；`0x202` 在 WidgetParams+0x78 extra_78，非几何 tag）、signifier 语义记录 `{WallId,angle=0.2}`+name `wall_height_edit_`+marker `0x142ab4830`（实现须走 `SignifierFactory::from_closure`/`write_factory`，删数字 Raw）；gate1=`!being_bulldozed`（值+0x297，writer=`publish_public_wall_state` VA `0x140a8b530`）已闭合。）

### 1.2 `ui_adjust_wall_height_mode` — VA `0x140ac6d60` / 12 参
PDB 12 参：
1 `Res<CursorPositionSS>` · 2 `Res<MainCamera>` · 3 `Res<WindowSize>` · 4 `Res<PublicWalls>` ·
5 `DecoratorArchivalResources` · 6 `ResMut<AdjWallHeightModeCache>` · 7 `EventReader<AudioCursorMoved>` ·
8 `EventReader<AdjustWallHeightUi>` · 9 `EventWriter<AdjustWallHeightCmd>` ·
10 `Query<(&TestCube,&mut Transform)>` · 11 `Res<DecoratorStorage>` · 12 `ResMut<DecoratorArchivist>`。
控制流：取 `AdjustWallHeightUi` 最后事件；屏幕 SS→世界（`mouse_ss_to_cs`）→ grab offset；init/update 两帧；装饰归档 + cull。
当前仓库：`ui_adjust_wall_height_mode.rs:191` 已实现（PARTIAL；12 参已对上；#12 用 `LiveDecoratorArchivist` 代 `DecoratorArchivist`）。缺失：`AdjustWallHeightUi` 已有。
差异：事件取最后、两帧语义齐全；数值 OK；`LiveDecoratorArchivist` 与 `DecoratorArchivist` 的别名需确认。
测试：已有；补 `AudioCursorMoved`/`AdjustWallHeightUi` 双帧。
**STATUS: READY**（现值已在位；`LiveDecoratorArchivist = DecoratorArchivist<LiveColliderPayload>` 已确认为同型别名）

### 1.3 `history_record_adjust_wall_height` — VA `0x140ac79e0` / 1360B（7 参）
PDB 7 参：1 `ResMut<AdjWallHeightModeCache>` · 2 `EventReader<FinishAdjWallHeightMode>` ·
3 `Query<(&TestCube,&mut Transform)>` · 4 `ResMut<History>` · 5 `Res<PublicWalls>` · 6 `Query<&Roof>` ·
7 `EventWriter<NotifyHintSystemShapeEditWasPerformed>`。
控制流（实读）：游标推进→无事件早退；记录门 `cache[+0x20]==1 && cache[+0x28]&1 && cache[+0]&1`；
h1=`[+0x24]`, h2=`[+0x2c]`, wall=`[+8]`；两次钳制：
`lo = roof? (roof.slope<=0.1 ? 1.6 : 0.7) : (cVar==0 ? 0.5 : 0.3)`，`hi = cVar==0 ? 14.0 : 2.5`，`clamp=min(max(h,lo),hi)`；
`h1!=h2 || NaN` → `EditType::new_height_changed` + `History::add`；再查 PublicWalls，若 `shape count(+0x48)<2` → hint（`NotifyHintSystemShapeEditWasPerformed` 是 **ZST**，8B = `EventInstance` id）。
尾部必达：cache `+0,+0x10,+0x18,+0x20,+0x28` 各 4B 清零。
常量：0.1/0.7/1.6/0.5/0.3/2.5/14.0（§0.2）。
当前仓库：`ui_adjust_wall_height_mode.rs:310` 已实现（PARTIAL：7 参对上；钳制外置 `height_rules`；`history.add()->Result` 包装；`new_height_changed` 私有 fn）。
差异：把钳制常量内联回本函数（严格一致）；`new_height_changed` 应为 `EditType` 关联函数。
测试：三条件门；平顶/斜顶/无屋顶下限；NaN 相等判据；尾部清零。
**STATUS: READY**

---

## 2. `move_gizmo_and_trigger_vol` — VA `0x14097d280` / 4832B

非独立 Bevy system；由 `ui_move_shapes` / `ui_move_freehand_walls` 调用（PDB
`country_core::systems::ui_move_shapes::move_gizmo_and_trigger_vol`）。无 FunctionSystem 签名（普通 fn）。

**签名（`mg.asm` 实读）**：
```
fn move_gizmo_and_trigger_vol(
    anchor:  &Vec3,        // RCX：*rcx = x,y ; *(u32)(rcx+8) = z
    rotation:&Quat,        // RDX：16 字节（Quat；非 Vec3）
    focus:   &FocusRec,    // R8：*r8 -> 记录，record[+0x10] = count:u32
    actions: &ActionSetStates,  // 由调用者传入的 Res<ActionSetStates>（rbx=指针；读 digital[2].pressed()）
    shape:   &ShapeRec,    // stack：+0x30 height:f32, +0x280 activation:f32
) -> ()                    // 返回 Vec<Triangle> 由 sret（RCX）写回调用者缓冲
```
> **ActionSetStates 读取（已解，指令锚）**：`0x14097D382 mov rax,[rbx]`；`0x14097D385 mov ebx,[rax+0x10]`（`+0x10`=**digital[2]**，8B/槽）；`0x14097D3EF xor eax,eax; 0x14097D3F1 cmp ebx,3; 0x14097D3F4 setae al` → `released = phase>=3`（`pressed = phase<3`）。`digital[2]`=`camera_rotate_mode`（`DIGITAL_TARGETS (37,2)`=鼠标 Rotate 绑定）。用途：camera_rotate_mode 按住/松开决定 gizmo 轴取向（相机相对 vs 固定）。PDB `S_INLINESITE inlinee=0x6A2A(pressed)` VA `0x14097D3EF..0x14097D3FD`。
> **纠正草稿**：第 2 参是 16 字节 **Quat**（`movaps xmm0,[rdx]` 后整块拷入 Transform.rotation），不是"方向 Vec3"。

**控制流（逐指令）**：
```
h = shape[+0x30]
t = clamp((h + (-0.5))/2.5, 0, 1)            // -0.5@0x14295a0e8, 2.5@0x1429cd064
smooth = (3.0 - 2.0*t) * t * t               // 3.0@0x1429cd078
base   = (1.0 - smooth) * 0.5 + smooth       // 0.5@0x142925870
Box3 = box_trigger_volume(                   // call 0x140ada0e0（§11 的同一 helper）
        width  = base * 1.5,                 // 1.5@0x1429258d0
        height = 0.5,                        // xmm2 = 0.5
        depth  = base * 1.5,                 // xmm3 = xmm1（x/z 对称）
        transform = &Transform {
            rotation:    *rotation,          // Quat
            translation: *anchor,            // Vec3
            scale:       Vec3::ONE,          // 三个 1.0
        })
count = focus.record[+0x10]
has_many = count > 2
ctx = UiState::add::<String>(format(/* 0x142ab6d30, fmt 0x140981660=Vet3 fmt */))   // FUN_140d79f90 + 1409231f0
if shape[+0x280] > 0.0 {                      // activation
    xvy(anchor)                               // 0x140c9cde0
    rec5 = {1,1, q0,q1, q2,q3}                // 0x28 B 记录（名字+Vec3），kind 0x42ab6d90 variant 1
    UiSignifierStream.push(rec5)
}
hover = 1.0
if ctx[+0x18] != 0 {                          // 命中类别
    if ctx[+0x18] == 1 { matched = (ctx[+0x38]==self && ctx[+0x19] != 0) }
    else               { matched = (ctx[+0x38]==self && ctx[+0x19] != 2) }
    if matched { hover = 1.2 }                // 1.2 个常立即数
}
scale = base * hover                          // local_94
for i in 0..2 {                               // 0x18 步长表，base 0x142ab6c90
    tag = axis_table[i].byte; if tag == 2 { return }
    axis_rec = {1,1, a0,a1, dir0,dir1, scale@+0x30, anchor Vec3@+0x34, tag u8@+0x40}  // 0x44 B
    UiSignifierStream.push(axis_rec)          // kind 0x42ab6e28 variant 1
}
```
- 轴表两项：`0x142ab6c90 = "move_shape"`（len 10）、`0x142ab6dc0 = "move_shape_outline"`（len 0x12）；`tag==2` 终止。轴记录名格式参数表 `0x142ab6dd8`（5 片段：anchor、scale(float)、quat）。
- widget 名的 format 表 `0x142ab6d30`（fmt `0x140981660` = `glam::f32::vec3::impl$92::fmt`），即 widget id 内嵌一个 Vec3 的显示。

**常量**：-0.5(`0x14295a0e8`)、2.5(`0x1429cd064`)、3.0(`0x1429cd078`)、1.0(`0x142925904`)、0.5(`0x142925870`)、1.5(`0x1429258d0`)、hover 1.2（立即数）。
**写出**：`UiSignifierStream`（激活时 1 个 0x28 记录 + 每轴 1 个 0x44 记录）；`box_trigger_volume` 的 `Vec<Triangle>` 作为 widget 命中体（由 `UiState::add` 参数/调用者持有）。
**依赖**：`UiState::add::<String>`、`box_trigger_volume`（§11）、`utils::swizzlers::xvy`、`glam` fmt。
**当前仓库**：无此函数；`box_trigger_volume`/`skewed_box_trigger_vol` 已有（§11，EQUAL，直接复用）。需在 `systems/ui_move_shapes.rs`（B 结构）新建本函数。
**测试**：smoothstep 边界（h<=-? / h>=? 钳位）；`count>2` 分支；hover 1.2/1.0（`ctx[+0x18]` 三值）；轴表 tag==2 终止；轴记录字段偏移（+0x30/+0x34/+0x40）；`box_trigger_volume` 尺寸 = (1.5*base,0.5,1.5*base)。
**说明**：`box_trigger_volume` 的 ABI 在 B 里把 3 个标量放在 xmm1/xmm2/xmm3、Transform 指针在栈；实现时直接调用现有 Rust `box_trigger_volume(width,height,depth,&Transform)`（其三角形输出已被测试钉住），传 `(1.5*base, 0.5, 1.5*base, Transform{rotation, translation:anchor, scale:ONE})` 即可，无需复刻寄存器布局。x/z 对称，w/d 顺序不影响结果。
**STATUS: READY**

### §2 补充合同（本轮 RNE 反汇编实读，`mg.asm` + `0x14097ac21` 调用点）`[SEMANTICS-DECODED]`
**完整 8 参签名（4 寄存器 + 4 栈）**（调用点 `0x14097abc4..0x14097ac21`；callee 偏移换算 rbp=caller_rsp+0x1b8）：
```rust
fn move_gizmo_and_trigger_vol(
    anchor:      &Vec3,              // #1 rcx ← caller &[rbp+0x170]（anchor 局部）
    rotation:    &Quat,              // #2 rdx ← caller &[rbp]（16B，形状朝向）
    actions:     &ActionSetStates,   // #3 r8  ← 调用者 [rbp+0x498]；callee `rax=[rbx]; ebx=[rax+0x10]` = digital[2] phase
    ui_state:    &UiState,           // #4 r9  ← 调用者 rbx（prologue `rbx=[*group]`，同 UiState::add 的 self）
    wall_id:     WallId,             // #5 栈 +0x20 (callee rbp+0x220; caller rax=[rbp+0x350] = (*AppMode)[+0x08])
    shape:       &ShapeRec,          // #6 栈 +0x28 (rbp+0x228; caller rdx=[rbp+0x168] = entry-0x2a0)
    signifier_y: f32,                // #7 栈 +0x30 (rbp+0x230; caller xmm12)
    signifiers:  &mut UiSignifierStream, // #8 栈 +0x38 (rbp+0x238; caller [rbp+0x358] = 分组参数+0x20)
) -> UiWidgetContext                 // 返回 (UiState*, WidgetId) 于 rax/rdx；调用者据返回 widget 的 phase 再分派 case
```
> **纠正旧草稿**：第 3 参（r8）是 **`ActionSetStates`**（`[*actions+0x10]` = digital[2] phase），第 4 参（r9）是 **`UiState`**（被 `UiState::add` 当 self 用）；`&ShapeRec` 是第 6 参（栈），不在寄存器。返回 **`UiWidgetContext`**（非 `()`；`0x14097ac28..0x14097ac59` 用 `rax=UiState, rdx=WidgetId` 做 `phase-2` 跳表分派）。
> **case1/case2 供给**：#1 anchor：case1=`xvy(形状平面中心,地形高度)`（`0x14097a996`），case2 同源；#2 rotation：case1=形状轴/角点归一化 Quat（`0x14097aa03..0x14097abc1`）；#7 signifier_y：case1=地形高度采样 `FUN_1409e9640`（`0x14097a96b`），case2=记录字段 `entry-0x1c`（`0x14097b70b`）。#3/#4/#5/#6/#8 由系统外层提供，与 case 无关（#5 来自 `(*AppMode)[+0x08]`，prologue `0x14097a18b/0x14097a280`）。

**`box_trigger_volume` 完整参数**（`0x14097d2c4..0x14097d37d` 实读）：
- `width = base*1.5`（xmm1，1.5@0x1429258d0）、`height = 0.5`（xmm2，0.5@0x142925870）、`depth = base*1.5`（xmm3=xmm1，x/z 对称）
- `Transform`（栈 rbp+0x90，作为第 5 个栈参 `[rsp+0x20]=&Transform`）：`rotation = *rotation`（16B Quat @+0）、`translation = *anchor`（Vec3 x@+0x10, y@+0x14, z@+0x18）、`scale = Vec3::ONE`（+0x1c/+0x20/+0x24 三个 0x3f800000）
- 返回 `Vec<Triangle>`（sret rcx=&rbp-8，24B）。附加字段：无（签名就是 `(width,height,depth,&Transform)`，与 §11.3/repo 一致）。
`base = (1.0 - smooth)*0.5 + smooth`，`smooth=(3-2t)t²`，`t=clamp((shape[+0x30]-0.5)/2.5,0,1)`（所有常量见 §2 主体）。

**`UiState::add` 的 widget 名与几何**（`0x14097d393..0x14097d4b4`）：
- 名字 = `format!("move_shape{}", wall_id)`：pieces 表 `0x142ab6d30 = {ptr→"move_shape"(len10)}`（1 个片段），格式参数 = 第 5 参 WallId（数据指针 `&rbp+0x220`），formatter `0x140981660`。
  > **纠正**：`0x140981660` **不是 glam Vec3 fmt**。其 rodata（0x142ab7140 起）是 Debug 元数据表，含 `"WallId"`、`"Wall"`/`"wall_id"` 等字段描述；格式化的是 **WallId(u64)**，不是 Vec3。
- 调用 `0x1409231f0`（`UiState::add::<String>` 单态）→ 返回 `(UiState*, WidgetId)`。
- 几何 = `WidgetGeometry::TriangleMesh(box_trigger_volume(...))`；`WidgetParams` 标量初值（`0x14097d43e..0x14097d499`）：`+0x00=0, +0x08=8, +0x10=0, +0x18=0, +0x20=0, +0x30=0/2, +0x48=0/3, +0x60=0/3, +0x78=0, +0x88=2, +0xa8=0x0100(word)`；其中 `0/2`、`0/3`、`0/3` 由 `released = digital[2].phase>=3` 决定（`setae`；`ecx=2*released`, `edx=3*released`）。字段名未解，实现按现有 `WidgetParams` 默认 + `geometry()` 构造即可，脚本区分位按上述开关保留。

**轴 signifier 的名字/内容/创建点**（当前实现返回空，需补）：
- 轴表在 `rbp+0x48`，stride `0x18`：entry0 `{tag byte +0=0, &str @+8 = "move_shape"(0x142ab6c90, len10)}`；entry1 `{tag=1, &str="move_shape_outline"(0x142ab6dc0, len18)}`；sentinel tag=2（循环 `0x14097d7a4`，`cmp bl,2 → exit`）。
- 每条记录 `alloc(0x50,0x10)`：`{+0x00 u64=1, +0x08 u64=1, +0x10 name_ptr, +0x18 name_len, +0x20..0x2f Quat(16B)=*rotation, +0x30 f32 scale(=base*hover), +0x34 Vec3 anchor(x@+0x34,y@+0x38,z@+0x3c), +0x40 u8 tag}`（记录有效长 0x41，alloc 0x50；旧稿 0x44B 为估读）。
- 创建点：hover 判定之后、公共尾之前（`0x14097d75b..0x14097d9f6`），每条 `FUN_14146c880(stream, rec, marker=0x142ab6e28)` 推入第 8 参流。
- 激活记录（前置，`shape[+0x280] > 0.0` 时，`0x14097d4c7..0x14097d6e8`）：`alloc(0x28,8)`，`{+0=1,+8=1, +0x10 Vec3 = xvy(shape 平面中心, y=signifier_y), +0x20 Vec3 = *anchor}`；marker `0x142ab6d90`，推入第 8 参流。

**公共尾 anchor/rotation 来源**（`0x14097abdf..0x14097ac14`）：
- `anchor`: 局部 `[rbp+0x2c0]`（Vec3）← `xvy(Vec2, y)`（`0x14097a996 call 0x140c9cde0`）。Vec2 取自形状记录字段并按 variant 选择：`x∈{entry-0x44, entry-0x3c}`、`z∈{entry-0x40, entry-0x38}`（`0x14097a8f3..0x14097a946`，`tag&1` 切换）；`y` = 地形高度采样 `FUN_1409e9640(...)` 与 `entry-0x20` 的 max/NaN 选择（`0x14097a94f..0x14097a990`）。即 **锚点 = 形状平面中心贴到地形高度**。
- `rotation`: `[rbp..rbp+0xf]`（16B Quat）← `xmm0` 在 `0x14097aa03..0x14097abc1` 由形状轴/角点归一化得到（与 §2 的 `rotation` 参数同源）；调用前 `movaps [rbp],xmm0`。

**轴 / 激活 signifier 的 mesh（marker → worker 实读，可照抄）**：
- marker `0x142ab6e28`（轴记录，payload size `0x40`/align `0x10`）→ worker `0x14097e290`：`movabs 0x8000000000000000`（payload 判别）→ **case 0 = Named 资产**；资产名 = 记录里的 `&str`（`"move_shape"` / `"move_shape_outline"`），shader `"vertex_color_shader"`（`0x142a9948c`）。
  → `SignifierMesh::Named("move_shape".into())` / `SignifierMesh::Named("move_shape_outline".into())`；B manifest 有 `assets/meshes/move_shape.json`、`move_shape_outline.json`。
- marker `0x142ab6d90`（激活记录，payload size `0x18`/align `4`）→ worker `0x14097dcc0`：`movabs 0x8000000000000002` → **case 2 = Points**；payload = 2 个 Vec3（24B）。
  → `SignifierMesh::Points(vec![v0, v1])`，`v0 = xvy(中心, signifier_y)`、`v1 = *anchor`。
- 两者 shader 均为 `"vertex_color_shader"`；材质回调同 §7（`0x1412d89b0/0x141564d30`）。
- **激活 `Points` 的 center 来源**（`0x14097d4dd..0x14097d519` 实读）：`v0 = Vec3::new(anchor.x, signifier_y, anchor.z)`（`xvy(Vec2(anchor.x, anchor.z), signifier_y)`；src 取自 **anchor 参数** `[rdi]`/`[rdi+8]`，不是 shape record），`v1 = *anchor`（`[rdi]`+`[rdi+8]` 的完整 Vec3，含 anchor.y）。即两点是“中心贴 signifier 高度”和“原 anchor”。
- **hover 来源**（`0x14097d70b..0x14097d74f`）：返回的 `UiState*` 上 `tag=byte[+0x18]`；`tag==0 → hover=1.0`；`tag==1 → matched = (active_widget[+0x38]==WidgetId && phase[+0x19]!=0)`；`tag>=2 → matched = (active_widget[+0x38]==WidgetId && phase[+0x19]!=2)`。`matched → xmm6=1.2`，最后 `scale = base * hover`。**hover 不是 shape record 字段，而是 `UiState` 的 +0x18/+0x19/+0x38 交互态。**

---

## 3. `ui_move_shapes` — VA `0x14097a060` / 12832B（30 参）

PDB 30 参（权威）+ 当前仓库类型可用性：
| # | PDB 类型 | 现状 |
|---|---|---|
| 1 | `Res<AppMode>` | ✅ |
| 2 | `Res<PublicWalls>` | ✅ |
| 3 | `ResMut<InnerWalls>` | ✅ |
| 4 | `Query<&Roof>` | ✅ `roof_shape::Roof` |
| 5 | `Res<CameraCursorRay>` | ✅ |
| 6 | `Res<CursorTerrainRaycast>` | ✅ |
| 7 | `EventReader<AudioCursorMoved>` | ✅ |
| 8 | `Res<RaycastWorld>` | ✅ `systems/collision/world.rs` |
| 9 | `ResMut<DecoratorStorage>` | ✅ |
| 10 | `ResMut<DecoratorArchivist>` | ✅（`LiveDecoratorArchivist` 为同型别名） |
| 11 | `Res<UiState>` | ✅ |
| 12 | `Res<UiSignifierStream>` | ✅ |
| 13 | `Local<MoveShapeCache>` | ❌ 缺失（见下） |
| 14 | `ResMut<History>` | ✅ |
| 15 | `Res<ActionSetStates>` | ✅（`digital[2].pressed()` 经 `move_gizmo` 读） |
| 16 | `EventWriter<RequestWallMaterialAudio>` | ✅ |
| 17 | `EventWriter<PlayBuildingAudio>` | ✅ |
| 18 | `EventWriter<PlayWallUiSound>` | ✅ |
| 19 | `EventWriter<OnShapeMoved>` | ❌ 缺失 |
| 20 | `EventWriter<CmdCheckForSheepInsideBuilding>` | ✅ |
| 21 | `EventWriter<HideTreesinAreaCmd>` | ✅ |
| 22 | `EventWriter<ShowPinnedDecoClutter>` | ✅ |
| 23 | `DecoratorArchivalResources` | ✅ |
| 24 | `Res<TerrainHeightsData<TerrainHeightSsbo>>` | ✅ |
| 25 | `Res<GladeBorder>` | ✅ |
| 26 | `EventWriter<DisplayOutOfBorder>` | ✅ |
| 27 | `EventWriter<FeedbackInputOutOfBorder>` | ✅ |
| 28 | `Res<GameTime>` | ✅ |
| 29 | `Res<Modifiers>` | ✅ `systems/backend/input/mod_key.rs` |
| 30 | `Res<ShapeIntersections>` | ✅ `resources/mod.rs` |

**输入**：`AppMode` 门（`*(u32*)*param_1 <= 5 && (==1||==2)`，反编译 0x14097a060 行 269-292）；`wall = *((u64*)*param_1+2)`；
`UiState` 选中；`AudioCursorMoved.last()`；`CameraCursorRay`/`RaycastWorld`/`CursorTerrainRaycast`；`ActionSetStates`（`digital[2].pressed()`=camera_rotate_mode，经 `move_gizmo` 读，见 §2）。

**`MoveShapeCache`（Local，缺失类型）已确认布局** `[FIELD-NAMES-SYNTHESIZED]`（反编译 `plVar46`，行 303-308 / 796-805 / 996-1048 / 1438-1449）：
```rust
// 缓存含多个 hashbrown SwissTable（Ghidra 变量在写入点复用；字段名按偏移定）：
struct MoveShapeCache {
    // map1（主表，HashMap<WallId, MoveShapeEntry>）：
    //   ctrl: *u8 @+0x00, bucket_mask: usize @+0x08, len/end: usize @+0x10, items: usize @+0x18
    //   hasher(RandomState) @+0x20      （行304 `plVar46+4` 传入 hash_one）
    //   +0x28 : 第二 hasher 或 Vec       （行796 `puVar12+8`=+0x28 传入 hash_one）
    // map3（第二 HashMap）：
    //   ctrl: *u8 @+0x30, bucket_mask: usize @+0x38, len/end: usize @+0x40, items: usize @+0x48
    //   hasher(RandomState) @+0x50      （行997/1442 `plVar46+10`=+0x50 传入 hash_one）
}
```
- 命中入口：`items = plVar46[3]`（行303，`==0` 跳过全部查询）。
- **Entry 布局（stride 0x2a8，负偏移）**：
```rust
// key:        WallId u64 @ -0x2a8   [SEMANTICS-DECODED]（行357 与 `*param_1+2`=wall 比较）
// state:      u64       @ -0x290    [SEMANTICS-DECODED]（<2 → assert panic，行359/1843）
// activation: f32       @ -0x270    [SEMANTICS-DECODED]（<=0.0 → assert panic；且 move_gizmo 用作 shape+0x30 高度）
// shape_rec:  [u8;0x280] @ -0x2a0   [FIELD-NAMES-SYNTHESIZED]（行368 `lStack_2f0 = base - 0x2a0`；整块传给 move_gizmo）
//   move_gizmo 只读 shape_rec[+0x30](=entry -0x270) 与 shape_rec[+0x280](=entry -0x20)；其余字节本函数不解释
// variant:    u32       @ -0x48    [SEMANTICS-DECODED]（跳表索引）
```
> 修正：`-0x270` 既是 activation 断言字段，也是 `move_gizmo` 的 `shape_rec+0x30`（smoothstep 高度源）；
> `shape_rec+0x280` = entry `-0x20`（>0 激活检查）。`state@-0x290` 落在 shape_rec 内。其余 `-0x2a0..-0x20` 由调用方写、本函数不读。
`Default` = 全空（map1 `items==0` 时跳过）。
**事件 push 点（按反编译行/call-site，行宽与字段实读）**：
| 站点 | 行宽/stride | 字段 |
|---|---|---|
| 行 493-508（case3 提交） | 0x20 | `+0 seq, +8 = (1<flag)*2\|0x11, +0x18 u32, +9..0x1f` |
| 行 1144-1162 | 0x30 | `+0 u64, +8 ptr, +0x10 u64, +0x18 ptr, +0x20 f32, +0x24 f32, +0x28 u64` |
| 行 1170-1175 | 0x10 | `+0 u64, +8 ptr` |
| 行 1598-1600 | 0x10 | `+0 u64, +8 u8=0` |
| 行 1620-1625 | 0x10 | `+0 u64, +8 u8=3, +0x18 u32, +9.. Vec` |
| 行 1644-1646 | 0x10 | `+0 u64, +8 u8=1` |
| 行 2355-2362 | 0x28 | `+0 u64, +8 i64, +0x10 u64, +0x18 ptr, +0x20 u32, +0x24 f32, +0x28 u64` |
| 行 2458-2464 | 0x20 | `+0 u64, +8 ptr, +0x10 u64, +0x18 f32, +0x1c u16=4` |
| 行 2486-2492 | 0x28 | `+0 u64, +8 ptr, +0x10 u64, +0x18 f32, +0x1c f32, +0x20 u8=4` |

**控制流**：cache 空→公共尾（gizmo）；`hash_one` 探针（0x1405b0d70）→ entry 断言（行 356-361）；
读 `variant=*(u32*)(entry-0x48)`；跳表 `0x142ab7514`（实读，见 §0.2）：0→`0x14097c31e`(return) 1→`0x14097ac5b` 2→`0x14097ae9d` 3→`0x14097af78` 4→`0x14097ffb0`(不可达/halt)。
公共尾调用 `move_gizmo_and_trigger_vol`（`0x14097a641` / `FUN_14097d280`，行 614）。
- case1（`0x14097ac5b`，矩形/线拖拽）：`PublicWalls`→Rect/Circle 几何（get_closest_edge_id/get_edge_pts/as_curve2/intersect_ray×2）、`time_continuous_lerp_vec2×2`、`TerrainHeightsData::sample_at_world_xz`、InnerWalls 写入、装饰归档、事件。
- case2（`0x14097ae9d`，hover/snap）：`ShapeIntersections`、`closest_segment_to_snap_to`、roof_entity+Roof、`DebugSphere/DebugLine`。
- case3（`0x14097af78`，提交）：行 419-511 实读可见 `FUN_140aa9800`(update wall state) → `FUN_140a82b40`(CompoundEditBuilder::next) → `FUN_141696f50`(from_iter) → `FUN_140a84ec0`(History::add) → 推一条 **0x20 行** `{+0 seq, +8 = (1<flag)*2 | 0x11, +0x18 u32, +9..0x1f Vec3/浮点}`（行 493-507）。

**事件 writer 分组（ABI 实读 + PDB 连续序）**：`EventWriter` 每项占 **0x20** 字节（结构：`+0x00 Events 基, +0x10 tick 指针, +0x1c tick 值`…；由 `*(u32*)W[2]=*(u32*)(W+0x1c)` 的 EventWriter tick 写法确认）。反编译里 `param_9` 在 `+0x1c/+0x3c/+0x5c` 三处出现 → 3 个连续 writer；`param_10` 在 `+0x1c/+0x3c/+0x5c/+0x7c` 四处 → 4 个连续 writer。PDB 里 7 个 writer 恰好连续（#16-22），故：
- `param_9 = (RequestWallMaterialAudio@+0x00, PlayBuildingAudio@+0x20, PlayWallUiSound@+0x40)`（#16/17/18）
- `param_10 = (OnShapeMoved@+0x00, CmdCheckForSheepInsideBuilding@+0x20, HideTreesinAreaCmd@+0x40, ShowPinnedDecoClutter@+0x60)`（#19/20/21/22）
（`DisplayOutOfBorder`/`FeedbackInputOutOfBorder` #26/27 与 `GladeBorder`/`GameTime` 同组 `param_13`。）
**事件行 = `EventInstance<E>` = `{E, event_id: usize}`**（id 是最后 8B）；上表行宽 - 8 = E 大小。已可据行内 `u16=4` 特征绑定：**行 2458-2464(0x20) = `RequestWallMaterialAudio`**（与 `edit_rectangle` 的 0x20 `{seq,WallId,params,u32,u16=4}` 同型）。其余 push 点→(param_N, writer 索引) 的逐一对应仍见缺证据 2。
跨模块事件：`OnShapeMoved` 属 `resources::events`，`ShowPinnedDecoClutter` 已有、`HideTreesinAreaCmd` 已有、`CmdCheckForSheepInsideBuilding` 已有；仅 `OnShapeMoved` 需新建并在注册文档登记。
**依赖**：`move_gizmo_and_trigger_vol`（§2）、`UiState::add`、`History` compound edit、`SignifierFactory`（禁 Raw）。
**当前仓库**：**无此系统**；`MoveShapeCache`、`OnShapeMoved` 缺失；其余参数类型齐全。
**差异**：需按 B 结构新建 `systems/ui_move_shapes.rs`（含 MoveShapeCache + 本系统 + move_gizmo）。
**测试**：模式门（>5、非1/2 早退）；cache 空/命中；断言 `state>=2`/`activation>0`；跳表四分支；越界→两 border 事件；`move_gizmo` 调用。
**缺证据（PENDING 原因，逐条，均为"需要哪一条指令"）**：
1. `MoveShapeCache` 多表精确切分（map1 边界、+0x28 是 hasher 还是 Vec、是否还有 map2）：需对 `0x14097a060` 的 `plVar46` **首次使用前的初始化**（或在 `MoveShapeCache::default`/drop 0x14087bxxx）反汇编确认；当前 Ghidra 在该点位复用变量导致切分不明。
2. **push 点 → (param_N, writer 索引) 的逐一对应**：分组已由 ABI(tick 偏移)+PDB 连续序确定（见上）；仍需在每个 push 点确认其 writer 基址变量（`*param_10`=writer0 / `uStack_410`=? / `plVar46` 等），才能把上表每行绑到具体事件名。这是 §3 唯一的 ABI 级剩余项。
3. `ActionSetStates`(#15)：**已解**——不是 body 直接读，而是传给 `move_gizmo_and_trigger_vol`，由 helper 读 `digital[2].pressed()`（`camera_rotate_mode`，`cmp 3`/`setae`；见 §2 指令锚）。
4. `OnShapeMoved` 的跨模块读者：需在 `crates/` 或原符号中定位 `EventReader<OnShapeMoved>`，据此定字段；否则按上表 0x20 行自定义并在注册文档登记。
### §3 逐 case 规格（可移植伪代码，供实现线）
分派：`variant = *(u32*)(entry-0x48)`；`switch(variant)`（decomp 640）：`0→0x14097c31e`=no-op return；`1→case1 drag`；`2→case2 hover-snap`；`3→case3 commit`；`4→halt/Debug`。**公共尾**：`move_gizmo_and_trigger_vol`（skeleton 调用清单唯一一次；参数 anchor=Vec3、rotation=Quat、focus=count@+0x10、shape=entry 的 `shape_rec@-0x2a0`）。**hashbrown SVM 块（`SUB161`/`pmovmskb` 循环）= bevy `HashMap` 探测内联，属框架代码；移植用 Rust `HashMap` 等价，不逐位复刻。**

**case3 提交（variant=3）** 指令锚 `code_0x14097af78`，decomp ~419-511；recon 849-987：
```
// entry = 当前 MoveShapeEntry（variant@-0x48==3）
1 if entry 未提交(sentinel != 0x8000..) { update_wall_state(curve) }     // FUN_140aa9800
2 复制 entry-0x53(curve_pts) / entry-0x50(aux) 到新堆缓冲（0xc/点, 4/元素）
3 push 0x30 行 { +0 ptr, +8 ptr, +0x10 ptr, +0x18 ptr, +0x20 f32, +0x24 f32, +0x28 u64 }  // 提交 ring / OnShapeMoved 候选
4 entry[-0x20] = 0x8000... (已提交 sentinel)
5 CompoundEditBuilder::next()                                             // 0x140a82b40
6 ConcurrentEdits::from_iter<…impl_0::new::closure_env_0>()               // 0x141696f50
7 History::add(edit_type = 7)                                            // 0x140a84ec0, &DAT_00000007
8 push 0x18 行 { +0 ptr, +8 ptr, +0x10 u8=1 }                            // ShowPinned/音频 (recon 982-987)
9 move_gizmo_and_trigger_vol(...)
```

**case1 拖拽（variant=1）** 起始 decomp 643+：
```
1 register "move_shape" widget: FUN_1413e77f0(..., &UNK_142ab6c90="move_shape")(decomp 655)
2 DecoratorArchivist::decorator_cull_allow_wall (FUN_140b2bc00)(657)
3 收集墙装饰: Query(FUN_140a1e420, 0x50 步长) → 三元组 {DecoratorId u32, dx f32, dy f32}(0xc) 到临时 Vec (678+; hashbrown 收集循环=框架)
4 复制 entry 曲线点(0xc/点, entry-0x53) 与 aux(4/元素, entry-0x50) 到 in-transit
5 写 param_6(shape 记录/entry): +0x10/+0x14, +0x18(ptr), +0x30/+0x34, +0x38/+0x3c, +0x48(Vec3 投影端点，由 viewport `uStack_148` 投影), +0x4/+0x24/+0x2c 高度/锚点 (decomp 762-788)
6 goto 公共尾
```

**case2 hover-snap（variant=2）** decomp 791+：
```
1 空间哈希查当前段: hash_one(0x1405b0d70) + WallSpatialHash::query → candidates (decomp 798+)
2 closest_segment_to_snap_to → SnapProposal（≤20 用 insert_tail，否则 driftsort_main）(recon 1246/1768)
3 PublicWalls::roof_entity → Query<&Roof>::get → Roof::height / snap_to_roof_ridge (recon 1341/1349/1371)
   roof snap: tag==2 → offset -6.0; tag==1 → -8.0 (recon 1330/1338)
4 DebugSphere::show / DebugLine::show（可视化）
5 move_gizmo_and_trigger_vol(...)
6 不写 InnerWalls（仅预览/高亮）
```
> 诚实边界：调用序列/条件/常量/写目标已给；`param_6`(entry) 的逐个字段**名字**与 0x30 行归 `OnShapeMoved` 还是提交 ring 需各 push 点再确认（偏移已知）。case4=Debug/halt（不可达）。

### §3 case1/case2 实现映射（crate API，符号实证）
**case1 step1/2/3/5**：
- step1 `FUN_1413e77f0` = **`DecoratorArchivist::archive_current_decorators_in_storages::<&str>("move_shape")`**（public-symbols）——**不是 widget 注册**（旧稿误标）；语义=以名 `"move_shape"` 归档当前装饰以便 restore。repo：`decorator_archivist::archive_current_decorators_in_storages`。
- step2 `FUN_140b2bc00` = `DecoratorArchivist::decorator_cull_allow_wall`（repo 同名）。
- step3 `FUN_140a1e420` = **`DecoratorStorage::wall_decorators_mut(wall) -> Option<&mut Decorators>`**（repo `decorator_storage.rs:652`）。
- step5 视口投影：`uStack_148 = *param_3`（decomp 250）= 光标射线 `{origin=(vp[0],vp[1],vp[2]), dir=(vp[3],vp[4],vp[5])}`（`Res<CursorTerrainRaycast>`）；公式 `t=(plane_y - origin.y)/dir.y; world.xz = origin.xz + t*dir.xz`（射线∩ y=const 平面）。`param_6[3]`/`param_6` 各字段为拖拽起点快照（世界点/端点：CONCAT 的 f32 对 = Vec2/Vec3 分量 + 相机投影）。
- **param_6 逐字段名**：需确定 param_6=哪个系统资源（wrapper ABI）才能命名；**值来源（射线世界点/端点/相机）已给**，偏移见 case1 伪码。

**case2 helper 合同（repo 映射，均已存在，无需新建）**：
- `closest_segment_to_snap_to` ↔ repo **`nearest_proposal(public:&PublicWalls, spatial:&WallSpatialHash, cursor:Vec3)->Option<SnapProposal>`**（`shape_placement.rs:240`）+ **`proposal_for_segment(state,segment,cursor)->Option<SnapProposal>`**（`:195/244`）；流程=`query_segments_with_aabb_clamped` → `filter_map(proposal_for_segment)` → `sort_by(distance_xz.partial_cmp().unwrap())` → first。**`sort_by` 即 std 稳定排序**（内部 insertion ≤20 / driftsort），无需复刻 `insert_tail`。
- `SnapProposal { wall:WallId, normal:Vec2, u:f32, point:Vec3 }`（`shape_placement.rs:182`）。
- `Roof::height()`=`roof_shape.rs:77`；`snap_to_roof_ridge(roof:&Roof, position:Vec3, threshold:f32)->RoofRidgeSnap`（`:584`）；`RoofRidgeSnap={None,Point(Vec3),Segment{a,b,t}}`（`:552`）。
- `DebugLine { points:Vec<Vec3>, colors:Option<Vec<Vec3>>, color:DebugColor, line_width:f32, ui_pass:bool, dashed:bool }`（`debug_viz.rs:288`）；`DebugSphere { color, radius:f32, center:Vec3, alpha:f32, solid:bool }`（`:70`）。

**§3 定向解决（本轮）：case1 step5 逐偏移来源 + case2 哈希源**
- **case1 step5 `param_6` 逐偏移 ← 局部（decomp 779-787 实读）**：
  | 偏移 | 写入源（local） | local 的候选 origin（行号；需 CF 确认路径） |
  |---|---|---|
  | `+0x00/+0x08`(qword) | `CONCAT(uStack_2e4,fStack_2e8)` ← `uStack_178` 两 f32 | `uStack_178=CONCAT(fVar70,fVar48)`(554)=世界点 Vec2 |
  | `+0x10`(qword) | `CONCAT(uStack_2dc,fStack_2e0)` ← `uStack_170` | `uStack_170=CONCAT(fStack_124,fStack_120)`(555) |
  | `+0x18`(Vec2) | `param_6[3]` = `(fVar57*vp[3]+vp[0]-fStack_198, fVar57*vp[5]+vp[2]-fStack_190)` | **射线∩y=const 平面**：`fVar57=(fStack_194-vp[1])/vp[4]`；`fStack_194/190/198` 来自射线/相机投影（`=` grep 未命中，需 CF 追踪） |
  | `+0x20`(f32) | `fStack_f8` | `*(f32*)(entry-0x48)`(1846)=**shape variant**（非高度） |
  | `+0x24`(f32) | `fStack_10c` | `SUB84(plVar25,0)`(849)/`fVar48`(654)=世界点 x |
  | `+0x28`(f32) | `fStack_11c` | `SUB84(puVar35,0)`(848)/`fVar70`(653)=世界点 z |
  | `+0x2c`(f32) | `fStack_120` | `*(f32*)(entry-0x3c)`(1852) |
  | `+0x30`(f32) | `fStack_124` | `*(f32*)(entry-0x38)`(1853) |
  | `+0x34`(qword) | `uStack_138` | `*(u64*)(entry-0x34)`(1854)=Vec2 分量 |
  | `+0x3c`(f32) | `fStack_194` | 射线/相机投影 y（同 +0x18 plane_y 源） |
  - **诚实边界**：上表"写入源"逐条 decomp 实读；但 `fStack_194/190/198`、`plVar25/puVar35/fVar48/fVar70` 的**上游公式**因 Ghidra 全局 local 复用跨多路径（set 点 653/848/1846-1854/2280-2283），**未能唯一确定为 case1 路径**——精确公式需对 case1 基本块 `0x14097ac5b..0x14097b476` 做 CF 追踪。**未据此编公式（避免造假）**。`param_6` = 含 `Vec<Vec3>` points（`*param_6`=count、`param_6[1]`=ptr、12B/点、旧缓冲被 free）的拖拽 WIP 状态；资源名需 wrapper ABI。
- **case2 哈希源（已定性）**：候选 = `param_2[4]`（`param_2+0x20`）的 HashMap（`items@+0x30`、`hasher@+0x40`、mask@`+0x10`、ctrl@`+0x0`），以 **Vec2 光标为 key** 做 `hash_one`+probe。→ **位置键空间哈希，不是 `ShapeIntersections`**（#30 键 `[WallId;2]`）；它是 **param_2 的字段（+0x20）**，非独立 PDB 参数。**对 repo**：`nearest_proposal(spatial:&WallSpatialHash)` 应绑定到 **param_2[4]**（空间哈希在 param_2+0x20 内），而非独立 `WallSpatialHash` 资源；需确认 param_2=`PublicWalls`（则空间哈希为 `PublicWalls+0x20` 字段）。**最小改动**：确认 `WallSpatialHash` 持有者；若 repo 独立资源与 param_2[4] 同源则等价，否则把 `nearest_proposal` 的 spatial 改为取 param_2[4]；`nearest_proposal`/`SnapProposal` 若需宿主调用改为 `pub(crate)`。

**§3 CF 追踪结论（case1 基本块 `0x14097ac5b..0x14097b476`）** `[SEMANTICS-DECODED 部分]`：
- `fStack_198/194/190` = **`swizzlers::xvy` 输出 Vec3**（decomp 546 `func_0x140c9cde0(&fStack_198,&fStack_3b0,mask)`；`fStack_3b0={fVar57,fVar77}` 世界 xz + `fVar55=FUN_1409e9640` 地形高度采样）→ **锚点世界点 (x, height, z)**（xvy 重排）。
- `fVar48=SUB84(plVar49,0)`(519)、`fVar70=SUB84(puVar34,0)`(530)，`plVar49=param_2[6]`(246)、`puVar34=param_10[0xe]`(288) → **光标世界 xz**（打包 Vec2）。
- 字段最终右值：`+0x18(Vec2)`=**`(ray_hit.x-anchor.x, ray_hit.z-anchor.z)`**（`ray_hit=origin+t*dir, t=(plane_y-origin.y)/dir.y`，vp=CursorTerrainRaycast 射线）=**拖拽位移向量**(CONFIRMED)；`+0x20`=shape variant(entry-0x48)；`+0x24/+0x28`=光标世界 x/z(fVar48/fVar70)；`+0x2c/+0x30`=entry-0x3c/-0x38；`+0x34(qword)`=entry-0x34(Vec2)；`+0x3c`=anchor.y(fStack_194)。
- **不可判**：`+0x00/+0x04/+0x08/+0x0c/+0x10/+0x14` 的 qword 内 f32 配对（Ghidra local 跨路径复用，`fStack_198/194` 与 `uStack_2dc/2e0`、`fStack_120/124` 配对不能唯一确定）。其余 8 字段 CONFIRMED。
- `param_6` = 含 `Vec<Vec3>` points（`*param_6`=count、`[1]`=ptr、12B/点、旧缓冲 free）的拖拽 WIP 状态。
**§3 case2 哈希源 owner（CF 结论）**：候选 = **`param_2[4]`(+0x20)**（Vec2 键、ctrl@0/mask@+0x10/items@+0x30/hasher@+0x40）；`param_2[6]`(+0x30)=光标世界点 Vec2。⇒ param_2 = **含 空间哈希(@+0x20)+光标点(@+0x30) 的聚合（grouped tuple），owner 不可唯一命名（不可判）**；语义=位置键空间哈希，**与现有 `WallSpatialHash` 等价**。**最小改动**：确认 `WallSpatialHash` 数据与 param_2[4] 同源即用现有资源（无需给 `PublicWalls` 加字段，除非后续证明内嵌）；`nearest_proposal`/`SnapProposal` 改 `pub(crate)`。

**STATUS: PENDING**（**已闭合**：case3 提交、`ActionSetStates.digital[2]`（move_gizmo `0x14097D385 mov ebx,[rax+0x10]`/`0x14097D3F1 cmp 3`/`setae`）、case2 语义与 API 映射。**BLOCKED**：case1 step5 有 6 个 f32（`+0x00..+0x14`）qword 配对不可判（local 跨路径复用）；case2 哈希 owner 不可唯一命名（功能=WallSpatialHash，用现有资源即可）。不阻塞其他叶子。）
**事件类型/消费者（更正此前"未定义"的旧注）**：`OnShapeMoved(pub WallId)`（`resources/events.rs`，已定义；**仓内无 `EventReader<OnShapeMoved>`**，读者未知/外部）；`DisplayOutOfBorder`(glade_border.rs，读者 `systems/glade-border/src/display_glade_border.rs:82`)、`FeedbackInputOutOfBorder`(glade_border.rs，读者 `.../feedback_input_out_of_border.rs:42`)、`ShowPinnedDecoClutter`(读者 `decoclutter_show_pins.rs:202`)、`HideTreesinAreaCmd`(读者 `systems/tree-tool/src/hide_trees.rs:109`)、`RequestWallMaterialAudio`(读者 `wall_audio_orchestrator.rs:56`)、`RecalculateRoofCmd`=`pub use RoofGeometryChanged as RecalculateRoofCmd`(`resources/roofs/commands.rs:40`)。→ 除 `OnShapeMoved` 外均已有类型+读者。
**`UiWidgetContext::dragged` 内联点（同 skeleton 文件 count=1）**：decomp `decomp-0x14097a060` line 436-438 `1 < *(byte*)(puVar12+0x18) && (*(char*)(puVar12+0x19) & 1)!=0 && *(u64*)(puVar12+0x38)==widget` → 读 **UiState `+0x18`(category)/`+0x19`(phase)/`+0x38`(widget id)** 与 `dragged()` 的 widget 交互相位语义一致（gating case1/2/3 的 widget 归属）。

### §3 本轮闭合：case1 `+0x00..+0x14` 不是 6 个 f32，而是缓冲描述符 `[SEMANTICS-DECODED]`
对 `0x14097ac5b..0x14097b476` 的 store 回推（`case1_block.asm`/`ums_part.asm`）：
- `0x14097affd`: `[rbp+0x180] = [rbp+0x2f0]`；`0x14097b00b`: `[rbp+0x170..0x17f] = [rbp+0x2e0]`（movups 16B）。
- `0x14097b046`: `[param_6+0x10] = [rbp+0x180]`；`0x14097b04a`: `[param_6..+0xf] = [rbp+0x170]`。
- ⇒ `param_6+0x00 / +0x08 / +0x10` 分别是 `[rbp+0x2e0] / [+0x2e8] / [+0x2f0]` 三个 **qword**。
- 这三个槽在主导路径（`0x14097ad4c..0x14097ad62`）被初始化为 `{+0x2e0=0, +0x2e8=4, +0x2f0=0}`，随后循环 `0x14097adb9..0x14097add6` 以 12B/元素写入 `[+0x2e8 + idx*12]`（`{DecoratorId u32, dx f32, dy f32}`，与 §3 step3 同一批装饰偏移）并把 `[+0x2f0]=len` 递增；`0x14097b020..0x14097b03a` 以 `[param_6]` 为容量、`[param_6+8]` 为指针做 `dealloc(ptr, cap*12, align 4)`。
- ⇒ 布局 = **`{capacity:u64 @+0x00, pointer:u64 @+0x08, length:u64 @+0x10}`**（12B 元素，align 4）。**旧稿的 "+0x00/+0x08/+0x10 = CONCAT 两 f32" 是 Ghidra 把三个 qword 拆成六个 f32；不存在 f32 配对问题。**
- 旧"6 f32"的出处：另一条 variant 路径 `0x14097a9bd..0x14097a9e9` 在同一个 `rbp+0x2e0` 临时构造 `Rectangle2d{axis,center,size}`（6 f32）供 `0x140c99570` 使用，随后在 `0x14097ad4c` 被上述描述符覆盖；两条路径互斥，最终写入 param_6 的始终是描述符。
- **实现结论**：`param_6+0x00..0x17` 按 24B 的 Vec 头（cap/ptr/len）处理，元素类型 `(u32,f32,f32)`；不要按 6 个独立 f32 或 Vec3 处理。

### §3 case1 step5 逐偏移右值（照抄表）
`ray` = `Res<CameraCursorRay>`（`[rbp+0x310] = [arg3_group+0]`；arg3 = `(CoreCursorRay, CursorTerrainRaycast, RaycastWorld)`，前 24B = `{origin: Vec3 @0, direction: Vec3 @0xc}`）。
`center` = 形状平面中心：`Vec2::new(cx, cz)`，`cx = tag&1 ? f32(entry[-0x3c]) : f32(entry[-0x44])`，`cz = tag&1 ? f32(entry[-0x38]) : f32(entry[-0x40])`；`anchor = Vec3::new(cx, terrain_height, cz)`（`terrain_height = FUN_1409e9640` 与 `f32(entry[-0x20])` 的 max/NaN-select）。

| 偏移 | Rust 右值 |
|---|---|
| `+0x00` | `decorators_capacity`（u64） |
| `+0x04` | `0`（capacity 高 32 位；非独立 f32） |
| `+0x08` | `decorators_pointer`（u64） |
| `+0x0c` | `0`（pointer 高 32 位） |
| `+0x10` | `decorators_length`（u64） |
| `+0x14` | `0`（length 高 32 位） |
| `+0x18` | `{ let t = (anchor.y - ray.origin.y) / ray.direction.y; Vec2::new(ray.origin.x + t*ray.direction.x, ray.origin.z + t*ray.direction.z) - Vec2::new(anchor.x, anchor.z) }` |
| `+0x20` | `u32(entry[-0x48])`（shape variant） |
| `+0x24` | `f32(entry[-0x44])` |
| `+0x28` | `f32(entry[-0x40])` |
| `+0x2c` | `f32(entry[-0x3c])` |
| `+0x30` | `f32(entry[-0x38])` |
| `+0x34` | `u64(entry[-0x34])`（两 f32） |
| `+0x3c` | `anchor.y` |

```rust
// Vec 元素（12B，align 4）：
{ (d.entry[-0x50]) as u32,            // DecoratorId
  f32(d.entry[-0x34]) - center.x,     // dx
  f32(d.entry[-0x2c]) - center.z }    // dy
// d = wall_decorators_mut(wall) 的每个桶元素（stride 0x50）；无需额外变换。
let record = ShapeRecord {
    decorators: Vec<(u32,f32,f32)>,   // 上面收集
    drag_delta: /* +0x18 表达式 */,
    variant: u32(entry[-0x48]),
    v0: f32(entry[-0x44]), v1: f32(entry[-0x40]),
    v2: f32(entry[-0x3c]), v3: f32(entry[-0x38]),
    v4: u64(entry[-0x34]),
    anchor_y: anchor.y,
};
```

### §3 shape record 字段布局（`entry[-X]` ≡ `shape_rec[0x2a0-X]`）`[SEMANTICS-DECODED]`
- `shape_rec` 基址 = `entry - 0x2a0`（`lStack_2f0 = base - 0x2a0`，`0x14097a395/0x14097a39b`），即 **整个 `PublicWallState` 值**（`PublicWallBuildingState([u8;0x2a0])`；spec 旧注 `[u8;0x280]` 是下界写小，实际 0x2a0）。桶 stride 0x2a8 = key `WallId`(8B, `entry-0x2a8`) + value 0x2a0。所以 `entry[-X] = shape_rec[0x2a0 - X]` 成立（X ≤ 0x2a0）。
- `PublicWallState` 内 `ShapeParameters`（28B，`ShapeParameters` / `WallShapeSnapshot`）位于 **`shape_rec[0x258..0x274]`**（repo `PublicWallBuildingState::shape()` = `self.0[0x258..0x274]`，`publication_fields.rs:106`；`ShapeParameters` = `{tag u32@0, payload 24B@4}`，tag bit0：0=Circle、1=Rectangle、≥2=freehand，`valid_enclosure/mod.rs` 文档）。
| entry 偏移 | shape_rec 偏移 | ShapeParameters 内偏移 | Rust 字段 |
|---|---|---|---|
| `-0x48` | `0x258` | `+0x00` | `ShapeParameters.tag: u32`（bit0 0=Circle/1=Rectangle；≥2 freehand） |
| `-0x44` | `0x25c` | `+0x04` | Circle.`cx` / Rectangle.`cos` |
| `-0x40` | `0x260` | `+0x08` | Circle.`cy` / Rectangle.`sin` |
| `-0x3c` | `0x264` | `+0x0c` | Circle.`r` / Rectangle.`cx`（= S2 的“旧半径”） |
| `-0x38` | `0x268` | `+0x10` | Circle.`spare` / Rectangle.`cy` |
| `-0x34` | `0x26c` | `+0x14`(8B) | Rectangle.`width`+`length`；Circle 为 spare+padding |
- **center 选择**（`0x14097a8f3..0x14097a946`）：`tag&1` ⇒ `(SP+0x0c, SP+0x10)`；否则 `(SP+0x04, SP+0x08)` —— 恰为 `ShapeParameters::center()`（`Shape::center()` 的 Circle/Rectangle 分支），不是额外字段。
- **写入 param_6 的形态**：`param_6+0x20..0x3b` = **整块 28B `ShapeParameters` 原样搬运**（tag@+0x20、payload@+0x24..+0x3b）；`+0x24/+0x28/+0x2c/+0x30/+0x34` 就是 SP+4/+8/+0xc/+0x10/+0x14。
- **装饰桶字段**（`-0x50/-0x34/-0x2c` 来自另一张表，**不是 shape_rec**）：`Decorators` 内嵌 `HashMap<u32 DecoratorId, DecoratorInfo 0x48B>`（桶 0x50B，`decorator_storage.rs:151-160`；循环 `add rbx,-0x500`=16×0x50，`0x14097ae00..0x14097ae6e`）。`[entry-0x50]` = key `DecoratorId u32`；`[entry-0x34]`/`[entry-0x2c]` = 桶内 +0x1c/+0x24 处的两个 f32 原始值（若 `(key,DecoratorInfo)` 顺序则落在 `DecoratorInfo+0x14/+0x1c`；repo 这两槽标 U，**具体结构体字段名不可判**）。语义用作 dx/dz 原始值；三元组 = `{ DecoratorId, raw(-0x34) - center.x, raw(-0x2c) - center.z }`。

### §3 case2 落地确认（`WallSpatialHash` 复用）
- 候选查询已实读：`0x14097ae9d` 起 `hash_one(0x1405b0d70)` + `Vec2` 光标键 probe（`param_2[4]`=+0x20 的 HashMap：ctrl@+0x00/mask@+0x10/items@+0x30/hasher@+0x40），语义 = 位置键空间哈希，与现有 `WallSpatialHash` 等价。
- **复用足够**：`nearest_proposal(public:&PublicWalls, spatial:&WallSpatialHash, cursor:Vec3) -> Option<SnapProposal>`（`shape_placement.rs:240`）内部就是 `query_segments_with_aabb_clamped` → `proposal_for_segment` → `sort_by(distance_xz)` → first；case2 的输入（空间哈希 + 光标世界点 Vec2）已能由 `PublicWalls` + `WallSpatialHash` + 第 6 参光标供给。
- **最小改动**：`nearest_proposal` 与 `SnapProposal`（`shape_placement.rs:182/240`）由私有改 `pub(crate)`；`WallSpatialHash` 已是 `pub use`（`public_walls.rs`）。无需给 `PublicWalls` 加字段；若后续证明 `param_2[4]` 与现有资源不同源，再把 `nearest_proposal` 的 `spatial` 参数改指 `param_2[4]`。
- 调用参数：`public = Res<PublicWalls>`（#2），`spatial = &WallSpatialHash`（同源），`cursor = Vec3`（世界 xz = `param_2[6]`(+0x30) 的 Vec2，y 由地形/墙体取，`distance_xz` 只读 xz，y 不影响结果）。

---

## 4. `ui_move_freehand_walls` — VA `0x1409781d0` / 7824B（22 参）

PDB 22 参：1 `Local<MoveWallCache>` 2 `Res<AppMode>` 3 `Res<PublicWalls>` 4 `ResMut<InnerWalls>`
5 `Res<CursorTerrainRaycast>` 6 `EventReader<AudioCursorMoved>` 7 `ResMut<DecoratorStorage>` 8 `ResMut<DecoratorArchivist>`
9 `Res<UiState>` 10 `ResMut<History>` 11 `EventWriter<RequestWallMaterialAudio>` 12 `DecoratorArchivalResources`
13 `Res<ActionSetStates>` 14 `Res<TerrainHeightsData<TerrainHeightSsbo>>` 15 `Res<MainCamera>` 16 `Res<UiSignifierStream>`
17 `Res<GladeBorder>` 18 `EventWriter<DisplayOutOfBorder>` 19 `EventWriter<FeedbackInputOutOfBorder>`
20 `EventWriter<OnShapeMoved>` 21 `EventWriter<HideTreesNextToWallCmd>` 22 `EventWriter<ShowPinnedDecoClutter>`。
控制流（`spec/freehand.md` 实读）：门 `*(i32*)*AppMode==1`；cache find(gid)；断言；`entry[-9]<2 → return`；
光标→曲线段 `get_coord_at_u`；线性插值 `t*b+(1-t)*a`；法线 `(-dy,dx)/|d|`；`eye_direction` 定侧；`max(terrain,entry.height)` 位运算擦 NaN；
`move_gizmo_and_trigger_vol`；二级跳表 `0x142ab7504`：0→return 1→开始拖拽 2→持续+roof-snap 3→提交（CompoundEdit+History）。
常量：0.1/0.15/0.3、60.0 clamp、snap 偏移 -6/-8、2.5、2.25 AABB、3.0/1.0/0.2/0.1。
写出：`OnShapeMoved`、`HideTreesNextToWallCmd`、`ShowPinnedDecoClutter`、`RequestWallMaterialAudio`、History。
当前仓库：**无此系统**；`MoveWallCache` 缺失。
差异：需新建。
**`MoveWallCache`（Local，缺失类型）**（`drop_in_place` @0x14087b530 反推，见 `spec/freehand.md §2.1`）：
```rust
struct MoveWallCache {                 // hashbrown SwissTable<GlobalId, MoveWallEntry>
    // ctrl @+0 / bucket_mask @+8 / growth_left @+0x10 / items @+0x18；entry stride 0x2a8
}
struct MoveWallEntry {                 // stride 0x2a8（负偏移）
    // global_id: GlobalId @ -0x2a8
    // state: u64           @ -0x290（<2 → panic/return）
    // activation: f32      @ -0x270（<=0 → panic）
    // stage: u32           @ -9     （<2 → return）
    // curve_pts: *[f32;2]  @ -0x53 / curve_len @ -0x52  （8B/点）
    // aux: *u32            @ -0x50 / aux_len @ -0x4f     （4B/元素）
    // flags: u32           @ -0x4e
    // height: f32          @ -0x4    （max(terrain, height) 下限）
}
```
`Default` = 空 HashMap。drop 分三段释放（控制块 `cap*12`、键区 `len*8`、辅助 `len*4`）证明空为合法初值。
**writer 绑定（按 PDB 顺序，无需 wrapper）**：`#11 RequestWallMaterialAudio`、`#18 DisplayOutOfBorder`、`#19 FeedbackInputOutOfBorder`、`#20 OnShapeMoved`、`#21 HideTreesNextToWallCmd`、`#22 ShowPinnedDecoClutter`；其余 Res/ResMut 按 PDB #1-17（#10 `ResMut<History>`、#13 `Res<ActionSetStates>`、#14 TerrainHeightsData、#15 MainCamera、#16 UiSignifierStream、#17 GladeBorder）。**跳表 `0x142ab7504`（实读）**：`0→0x14097983e`(return) `1→0x14097863a` `2→0x140978861` `3→0x140978917`。
**本轮实读（`0x1409781d0` recon）`[SEMANTICS-DECODED 偏移]`**：
- `MoveWallEntry` 字段（recon 负索引×8 → 字节）：`-0x298` curve_pts ptr、`-0x290` 点计数（与 idx 比较）、`-0x280` aux ptr、`-0x278` aux len、`-0x270` u32 flags、`-0x20` height f32。
- **roof-snap（repo 已实现，无需新 enum）**：原 `tag==2 → -6.0`、`tag==1 → -8.0` ↔ **`shape_placement.rs:388-410`**：`if AppMode::Wall(WallShape::Rectangle) { -6.0 } else { -8.0 }`；随后 `PublicWalls::roof_entity` → `Query<&Roof>::get` → `snap_to_roof_ridge(roof, hit.position, 0.5)`；`RoofRidgeSnap::Segment` 用 `DebugLine::new([a+Vec3(0,0.2,0), b+Vec3(0,0.2,0)])`。
- **提交行**（recon 849-857）：0x30 行 `{+0 ptr, +8 ptr, +0x10 ptr, +0x18 ptr, +0x20 f32, +0x24 f32, +0x28 u64}`；随后 `CompoundEditBuilder::next`(949) + `ConcurrentEdits::from_iter<…impl_0::new::closure_env_0>`(962) + `History::add`(969，`&DAT_00000007`=edit type 7)；再推 0x18 行 `{+0 ptr, +8 ptr, +0x10 u8=1}`（Ghidra 误标 `tuple<&str,SettingsMenuState>`；按 PDB 归 ShowPinnedDecoClutter/音频）。
**事件类型映射（均已存在，无需新建）**：`OnShapeMoved(pub WallId)`（`resources/events.rs`；仓内无 reader）；`HideTreesNextToWallCmd { wall:WallId, finished:bool }`（`resources/tree_storage.rs:159`）；`ShowPinnedDecoClutter { pinned:Vec<u64>, hits:Vec<u64>, show:bool }`（`decoclutter_show_pins.rs`，含遗留占位字段需按原行校正）；`RequestWallMaterialAudio { id:u64, position:Vec3, kind:u8, flag:u8 }`（`audio/mod.rs`）；`DisplayOutOfBorder`/`FeedbackInputOutOfBorder`（`glade_border.rs`）。
**缺证据（PENDING）**：`MoveWallEntry` 字段名（偏移已知，可按 FIELD-NAMES-SYNTHESIZED）；**各事件 push 行的精确归属**（0x30 行 vs 0x18 行 vs 0x20 行 → `OnShapeMoved`/`HideTreesNextToWallCmd`/`ShowPinnedDecoClutter`/`RequestWallMaterialAudio`，需在各 push 点确认）；提交段资产句柄（`_UNK_142aa9140…`）。（roof-snap、事件类型、ActionSetStates 均已解。）
**STATUS: PENDING**

---

## 5. `ui_edit_freehand_walls` — VA `0x140ac98e0` / 12752B（20 参）

PDB 20 参：1 `Local<EditFreehandWallCache>` 2 `Res<AppMode>` 3 `Res<PublicWalls>` 4 `ResMut<InnerWalls>`
5 `Res<CursorTerrainRaycast>` 6 `EventReader<AudioCursorMoved>` 7 `Res<DecoratorStorage>` 8 `ResMut<DecoratorArchivist>`
9 `Res<TerrainHeightsData<TerrainHeightSsbo>>` 10 `DecoratorArchivalResources` 11 `Res<UiState>` 12 `ResMut<History>`
13 `EventWriter<RequestWallMaterialAudio>` 14 `EventWriter<OnFreehandWallEdited>` 15 `Res<UiSignifierStream>`
16 `Res<GladeBorder>` 17 `EventWriter<DisplayOutOfBorder>` 18 `EventWriter<FeedbackInputOutOfBorder>`
19 `EventWriter<ShowPinnedDecoClutter>` 20 `EventWriter<HideTreesNextToWallCmd>`。
控制流：入口 SwissTable 断言；`entry[-0x48]<2 → return`；主循环逐段 hover；跳表 `0x142ae0cb8`（8 分支，见 §0.2）：
0→主循环 1→开始/注册 2→持续（距离去重 0.1、曲线插入、History）3→提交 ring 4→胶水 5→二级跳表(0x142ae0cc8) 6→wall-eraser 提交前路 7→halt。
提交段 `code_...acd281`：光标→段哈希→wall-constructor→`OnFreehandWallEdited`/音频/树/pin；History 两 ring 0x18B。
常量：0.1/0.15/0.3、AABB 2.25、0x18/0x28/0x98/0x78 步长。
写出：`OnFreehandWallEdited`、`RequestWallMaterialAudio`、`HideTreesNextToWallCmd`、`ShowPinnedDecoClutter`、History。
当前仓库：**无此系统**；`EditFreehandWallCache` 缺失。
**`EditFreehandWallCache`（Local，缺失类型）**（`drop_in_place` @0x14087cf40 反推，见 `spec/freehand.md §2.2`）：
```rust
struct EditFreehandWallCache {
    // @0x00  HashMap<GlobalId, EditEntry>   （SwissTable，entry stride 0x2a8）
    // @0x18  Vec<[f32;2]>  stroke_pts        （8B/点，ptr@0x20,len@0x18）
    // @0x30  Vec<u32>      stroke_aux        （4B/元素）
    // @0x28,0x50,0x78,0xa0,0xc8,0xf0,0x118,0x140,0x168: 约 9~10 个 Vec<u8>（stride 0x28）
    //         → widget-id/临时 String 池（至少 2 个被 widget 注册写入，其余语义 UNKNOWN）
}
```
`Default` = 全空（所有 Vec len=0/ptr=dangling、HashMap cap=0）。
`EditEntry`（stride 0x2a8）至少含：`key@-0x2a8`、`state u64@-0x290`（断言）、`activation f32@-0x270`、`variant u32@-0x48`（<2 → return）、`simplified_curve_2d`（提交前断言 `is_valid()`）。
**writer 绑定（按 PDB 顺序，无需 wrapper）**：`#13 RequestWallMaterialAudio`、`#14 OnFreehandWallEdited`、`#17 DisplayOutOfBorder`、`#18 FeedbackInputOutOfBorder`、`#19 ShowPinnedDecoClutter`、`#20 HideTreesNextToWallCmd`；其余 Res/ResMut 按 PDB #1-12/#15/16。**跳表 `0x142ae0cb8`（实读）**：`0→0x140ac9dc5`(loop) `1→0x140aca18b` `2→0x140aca3e0` `3→0x140aca360` `4→0x140acfded` `5→0x140accd8a` `6→0x140acce45` `7→0x140accedb`；二级 `0x142ae0cc8`：`0→0x140acfdfd 1→0x140accd9a 2→0x140acce55 3→0x140acceeb`。
**缺证据（PENDING，PDB 不可给的真读项）**：10 个 `Vec<u8>` 语义；`EditEntry` 完整字段；wall-eraser/split 子机（`code_...acd281` 资产句柄）；`OnFreehandWallEdited` 载荷；（§5 无 `ActionSetStates` 参数。）
**STATUS: PENDING**

---

## 6. `ui_edit_circle` — VA `0x14165d460` / 14848B（30 参）

PDB 30 参：1 `Res<AppMode>` 2 `ResMut<InnerWalls>` 3 `Res<PublicWalls>` 4 `Res<WallEntities>`
5 `Query<&WallTriggerVolume>` 6 `DecoratorArchivalResources` 7 `Res<GladeBorder>` 8 `Res<UiState>`
9 `Res<UiSignifierStream>` 10 `Local<CircleGrabParams>` 11 `Local<CircleSignifierParams>`
12 `ResMut<AnimatedCircleSignifierResource>` 13 `Res<CursorPositionSS>` 14 `Res<WindowSize>` 15 `Res<MainCamera>`
16 `ResMut<DecoratorArchivist>` 17 `ResMut<DecoratorStorage>` 18 `Res<GameTime>` 19 `Res<Time<()>>`
20 `EventReader<MainCameraMoved>` 21 `EventWriter<DisplayOutOfBorder>` 22 `EventWriter<FeedbackInputOutOfBorder>`
23 `EventWriter<HighlightWallCmd>` 24 `EventWriter<NotifyHintSystemShapeEditWasPerformed>`
25 `EventWriter<CmdCheckForSheepInsideBuilding>` 26 `EventWriter<PlayBuildingAudio>`
27 `EventWriter<RequestWallMaterialAudio>` 28 `EventWriter<HideTreesinAreaCmd>` 29 `EventWriter<ShowPinnedDecoClutter>`
30 `ResMut<History>`。
> **纠正**：原版**无** `EditCircleCmd`/`FinishEditCircleCmd`（PDB 无 EventWriter）；输入是 `UiState` + `WallTriggerVolume` Query + 光标 SS→相机射线。唯一 EventReader 是 `MainCameraMoved`。
控制流（`spec/ui_edit_circle.md`）：AppMode 门 `mode<=5`；`UiState::dragged/intersecting` 取 wall；断言 curve 有效；
`update_circle_radius`（SS→ray→地面；半径 `clamp(0.9, 8.0)`；阈值 `0.01`）；越界→两 border 事件；`Circle2d::as_curve2(0.2)`→`update_wall_state_curve/enclosure`；`show_full_circle_signifier`（3.0/5.0）；
release：`|Δr|<=0.01` → `extract_and_restore_to_storage`；否则 `CompoundEditBuilder` + `History::add` + hint + sheep。
常量：0.01、0.9、8.0、0.2、0.5、1.0、-0.5、3.0(0x40400000)、5.0(0x40a00000)。
当前仓库：`ui_edit_circle.rs`(103 行) 为事件驱动壳（INVENTED）；`CircleEditState` 自创。
差异/删除：删除 `EditCircleCmd`/`FinishEditCircleCmd` 及 `CircleEditState`、事件循环；重写为上述 30 参系统。
**缺失类型与已知布局**：
- `AnimatedCircleSignifierResource`（size 64，注册默认 `+0x38 byte=2`）**已解**：`angle_base: f32 @+0x18`、`position: Vec3 @+0x20`（x@0x20,y@0x24,z@0x28）、`normal: Vec3 @+0x2c`（x@0x2c,y@0x30,z@0x34）、`tag: u8 @+0x38`（0=Circle,1=Rect,2=Freehand）。其余 `+0x00..0x17` 未用，保持 UNKNOWN。
- `CircleGrabParams` / `CircleSignifierParams`（两个 Local）：`impl::default` body 未恢复，字段/默认值 UNKNOWN（反编译中与资源交织，未分离）。
- `show_full_circle_signifier`（`0x141660880`）`[SEMANTICS-DECODED]`：签名 `(stream, name_arg, circle_rec, radius_a:f32, radius_b:f32)`；
  - **0x28 记录**（`alloc(0x28,8)`）：`{ +0x00 u64=1, +0x08 u64=1, +0x10 f32=circle.center.x, +0x14 f32=circle.center.y,
    +0x18 f32=circle_rec[0], +0x1c f32=circle_rec[+0xc], +0x20 f32=radius_a, +0x24 f32=radius_b }`。
  - push marker `0x142c824f8`，名字 format `UNK_142c824b8`（4 片段）；入参半径 3.0(0x40400000)/5.0(0x40a00000) 落到 +0x20/+0x24。
  - 当前 repo 的 `show_full_circle_signifier`（`UiAppendSignifier{tag:1,kind:1,position,scale:radius,name:0}`）**丢失** center.xy、circle_rec 两字段与 radius_b。
**ABI 结构（实读 `0x14165d460` 全函数扫描 `ec_full.asm` + `spec/_edit_circle.c`）** `[分组 SEMANTICS-DECODED / 逐一并列 UNCONFIRMED]`：
- **纠正**：该函数**不是 30 个独立实参**。全函数只引用到 `[rbp+0x490..0x4d0]`（`arg5..arg13`）+ 4 个寄存器参（`arg1..4`），即 **13 个顶层实参**（`arg5=rbp+0x490` … `arg13=rbp+0x4d0`，间距 8B；`0x4d8..0x548` 从不被引用）。因此 30 个 SystemParamItem 被编译器**分组**进 13 个实参（大 tuple 按值/子 tuple 传递），PDB 顺序本身不足以逐一绑定。
- `EventWriter` 每项 **0x20B**（tick `*(u32*)W[2]=*(u32*)(W+0x1c)`）。9 个连续 writer（PDB #21-29）分布在两个子 tuple：
  - `param_9` = **2** 个 writer（tick `+0x1c,+0x3c`，offset `+0x00,+0x20`）。
  - `param_12` = **7** 个 writer（tick `+0x3c,+0x7c,+0x9c,+0xbc,+0xdc,+0xfc,+0x11c`，offset `+0x20,+0x60,+0x80,+0xa0,+0xc0,+0xe0,+0x100`；`+0x40` 区非 writer）。
  - **假设（按 PDB 连续序 + offset 递增）**：`param_9`=`(DisplayOutOfBorder@+0x00, FeedbackInputOutOfBorder@+0x20)`；`param_12`=`(HighlightWallCmd@+0x20, NotifyHint@+0x60, CmdCheckForSheep@+0x80, PlayBuildingAudio@+0xa0, RequestWallMaterialAudio@+0xc0, HideTreesinAreaCmd@+0xe0, ShowPinnedDecoClutter@+0x100)`。
- **不能仅凭 PDB 顺序升为 SEMANTICS-DECODED 的原因**：13 个顶层实参是 30 项经 Rust tuple ABI 的**再分组**，其分组边界不在 PDB 里。
- **调用点已定位（本轮）**：`ui_edit_circle` 的 wrapper 调用 `FUN_14165d460` 在 **`0x1414ba3d8`**（另一处 `0x14173b6a2`）。调用前把 13 参放到：`arg1=rcx=&[rsp+0xa8]`；栈 `arg5..arg13` 依次 `[rsp+0x20]=rcx(old),0x28=r10,0x30=r15,0x38=r12=&[rsp+0x210],0x40=r11=&[rsp+0x270],0x48=r13=&[rsp+0x2b0],0x50=rdi=&[rsp+0x2f0],0x58=r14=&[rsp+0x318],0x60=&[rsp+0x438]`（arg2/3/4=rdx/r8/r9 在窗口外设置）。这 13 个是**指向子结构的指针**（`[rsp+0xa8]/0x210/0x270/0x2b0/0x2f0/0x318/0x438`），各含若干 SystemParamItem。
- **② 具名 run 符号搜索结果（2026-10-28）：无**。`public-symbols` 中按 `ui_edit_circle`+`run`/`run_unsafe`/`run_without_applying_deferred` 检索 **空**；只剩 `drop_in_place<FunctionSystem<…>>`（`0x1405cf3b0`/`0x1408869d0`）等 drop 胶水。故 §6 wrapper 无法经具名 run 反编译。
- **③ 手动 rne_dis 计划（已批准，待执行）**：脚本化反汇编 `0x1414b92bc..0x1414ba3d8`（≈0x1124B），输出"PDB index(1..30) → 寄存器/栈槽 → 首次读取指令"表：① 提取 wrapper 序言对 30 个 SystemParamItem 指针的 spill；② 提取发送前把 item 写入 13 个 call-arg 子结构（`[rsp+0xa8]`/`0x210`/`0x270`/`0x2b0`/`0x2f0`/`0x318`/`0x438` 及 `[rsp+0x20..0x60]`）的 store；③ 以 writer 子结构（`param_9` 2 个 + `param_12` 7 个，0x20 步长）验证 index→名字。**未执行前 §6 的 writer 逐一绑定仍为假设。**
**缺证据（已按 PDB 决策收敛）**：按项目规则 **PDB 有序类型为权威**，§6 的 30 项与 9 个 writer（#21-29）的 index→名字按 PDB 顺序绑定即成立，`param_9(2)+param_12(7)` 只是 tuple 打包形式，不改变顺序——**不再作为阻塞**（wrapper 追踪降为可选确认，② 具名 run 已查无、③ 计划见上）。**唯一仍实读项**：`CircleGrabParams`/`CircleSignifierParams` 的 `FromWorld`/`Default` 字段与默认值（其 `impl::default` 无独立符号、由 Local 的 SystemParamState 内联；需该状态初始化反编译）——标 UNKNOWN，不阻塞其余合同。`FUN_1402c9200/1409231f0/1413e77f0/14146c880/140d79f90` 真名候选见 `ui_edit_circle.rs.dependencies.json`。
**分段 body 规格（可移植伪代码，供实现线）** `[SEMANTICS-DECODED / 指令锚]`：
- **S0 门控**（decomp 1216/1220）：`mode=*(u32*)*AppMode`；`if mode>5 → goto tail(0x14165e258)`；`if mode∉{1,2} → return`。
- **S1 定位墙**：`UiState::dragged/intersecting` 取 wall id；`assert w.simplified_curve_2d.is_valid()`（0x142ada193）；`PublicWalls::get(wall)`。
- **S2 光标→半径**：`hit = project_cursor_onto_ray(CursorPositionSS, WindowSize, MainCamera)`（SS→world，repo `utils/project_cursor_onto_ray`）；`raw_r = dist(hit.xz, center)`；`eased = time_continuous_lerp(prev, raw_r, dt)`（`func_0x140c904a0`，decomp 2100；dt=`Time`/`GameTime`）；`r = clamp(eased, 0.9@0x142a7c880, 8.0@0x142934604)`（decomp 1774-1796）。
- **S3 边界**：`if !GladeBorder::is_shape_inside(circle)`：`r' = search_lerped_shape_that_fits_into_glade(...)`（decomp 4098-4104 的 lerp 半径：`(0.9+t*..)`/`(t+8.0)`）；写 `DisplayOutOfBorder` + `FeedbackInputOutOfBorder`。
- **S4 应用**：`curve = Circle2d::new(center, r).as_curve2(0.2)`（`FUN_140c97cf0`, decomp 2014）；`InnerWalls::update_wall_state_curve(0x140aaa090, decomp 2046)`；`update_wall_state_enclosure(0x140aa9610, decomp 2066)`；`show_full_circle_signifier(0x141660880, decomp 2686/3832, radius 3.0/5.0)`。
- **S5 释放**：`if |r_new - r_old| <= 0.01(0x142960674)`（decomp 2164）→ `DecoratorArchivist::extract_and_restore_to_storage(0x140b2c4b0, decomp 2170)`；否则 `CompoundEditBuilder::next(0x140a82b40, decomp 3562)` → `ConcurrentEdits::from_iter<…impl_0::new::closure_env_0>(0x141696f50, decomp 3600)` → `History::add(edit_type @UNK_142c825c8, 0x140a84ec0, decomp 3622)` + `NotifyHintSystemShapeEditWasPerformed`(#24) + `CmdCheckForSheepInsideBuilding`(#25)。
> 框架内联（hashbrown 探测、`UiState::add`、`EventWriter::write`）用 Rust 等价，不逐位；SS→ray 投影用 repo `project_cursor_onto_ray`；ease 用 repo `time_continuous_lerp`。**S2 `raw_r` 精确式**：`raw_r = distance_xz(projected_cursor, center) = sqrt((hit.x-center.x)² + (hit.z-center.z)²)`（分量顺序/abs 不影响数值）；再 `eased = time_continuous_lerp(prev, raw_r, dt)`，`r = clamp(eased, 0.9, 8.0)`。**S3 lerp 的 `t` 来源（decomp 4080-4104 实读）**：`t = 0.5(_UNK_142925870)` 当 `(bVar43-1) < 2`（bVar43=圈选择/子态，需其 arg 身份）；否则若 `(param_1 & 1)==0` 则 `t = 0.0`。两候选圆端点 `P_lo = B + (0.9+t)*A`、`P_hi = B + (8.0+t)*A`（`A=plStack_c8` Vec2、`B=puVar25` Vec2，=`center`/`dir`），喂 `search_lerped_shape_that_fits_into_glade`。其余调用序列/条件/常量/写目标已锚定。

**STATUS: READY**（30 参按 PDB 有序类型为权威；9 writer #21-29 按 PDB 顺序绑定；`AnimatedCircleSignifierResource` 全布局、`show_full_circle_signifier` 0x28 记录、控制流、常量、分段 body+两个边界已给。**唯一 UNKNOWN**：`CircleGrabParams`/`CircleSignifierParams` 的 `FromWorld`/`Default` 字段（Local 状态内联，标 UNKNOWN，不阻塞）。）

### §6 补充（本轮 RNE 实读：S3 边界函数 / S1 选墙链 / S2 投影实现）
#### S3 `search_lerped_shape_that_fits_into_glade` 合同（VA `0x140ad2ce0`）`[SEMANTICS-DECODED]`
> **纠正**：`0x140ad2ce0` 不是"围合合法性"检查；它就是 `GladeBorder::search_lerped_shape_that_fits_into_glade`。合法性检查是它内部调用的 `is_shape_inside`（VA `0x140ad2970`，同一地址，非两个函数）。
- **签名**（Ghidra `param_1..4` + 寄存器实测）：`fn search_lerped_shape_that_fits_into_glade(&self /*order: sret out(rcx), self(rdx), from(r8), to(r9)*/, from: BorderShape, to: BorderShape) -> BorderShape`
  - 出参 28B：`{tag:u32 @0, payload 24B}`；`BorderShape::Circle(tag=0){center:Vec2@+4, radius:f32@+0xc, rotation:f32@+0x10}`、`::Rectangle(tag=1){axis:Vec2@+4, center:Vec2@+0xc, size:Vec2@+0x14}`（与 repo `BorderShape` 字段序一致）。
- **算法**（`0x140ad2d85..0x140ad2fed` 两 parity 分支同构）：
  1. 若 `from.tag & 1 != to.tag & 1` → `unreachable`/panic（类型不匹配）。
  2. `t=0.0`: `lerped = Shape::lerp(from,to,0.0)`（=from）；`if !self.is_shape_inside(lerped) → 返回 from 原值`（tag 仍为 from.tag，payload=from）。
  3. 否则 `best=lerped`；`for i in 1..20 { t = i as f32 / 20.0; lerped = lerp(from,to,t); if !is_shape_inside(lerped) { break } best=lerped }`。
  4. 返回 `best`（tag = from.tag：0=Circle/1=Rectangle）。**步进 20 等分（0.05），i=1..19，永不测 t=1.0；首步 t=0 失败则原样返回 from。**
- 常量：除数 `20.0` @`0x142a7fa30`。`lerp` 分别调 `Circle2d::lerp`@`0x140c98170` / `Rectangle2d::lerp`@`0x140c9c730`（repo 同名）。
- **调用方**：`ui_edit_circle` S3（`0x14165ee6e`，out=`&rbp+0x360`，self=rsi=GladeBorder，from=`&rbp+0x2d0`、to=`&rbp+0x1b0`，两者 tag=0 即 Circle）；`do_ui_edit_rectangle_dims`（`0x140ad7a2e`）。返回后调用方按 tag 取半径（Circle 半径 @+0xc）。
- **关系**：`is_shape_inside`（0x140ad2970）= 4 分支：Circle 边界/Circle 主体用严距离 `dist+subject.r > border.r`（+0xad2b55）；Circle/Rectangle 用 Rectangle 四角 `is_point_inside`；Rectangle/Circle 用 `Circle2d::as_points2(1.0)` 每点 `contains_point`；Rectangle/Rectangle 用四角 `contains_point`。repo `GladeBorder::is_shape_inside` 已等价。

#### S1 选墙链（`ui_edit_circle`，指令锚 `0x14165daf0..0x14165df7e`）`[SEMANTICS-DECODED]`
> **结论：不是 entity→wall 反查**。原版**遍历墙记录**，对每面墙用 **名字里编码 WallId** 的 widget 注册并比对 `UiState` 活动 widget；`WallEntities`/`Query<&WallTriggerVolume>` 只用于选中后取抓取体。
1. **遍历墙记录**（`0x14165daf0`，hashbrown iterator，entry stride `0x2a8`，key WallId 在 `entry-0x2a8`）→ `wall_id`（rsi，存 `[rbp+0xc8]`）。该 0x2a8 表即 `PublicWalls` 记录表（`PublicWallState`，repo 注释 "key WallId + 0x2a0 value"）。
2. **取记录并断言**（`0x14165db4d..0x14165dc00`）：`hash_one(0x1405b0d70)`+probe；命中后 `if entry[-0x290] < 2 || entry[-0x270] <= 0.0 { panic 0x142ada193 }` —— 即 `PublicWalls::get` 内联的 `simplified_curve_2d.is_valid()` 断言（`state>=2 && activation>0`）。**判空顺序：先 map probe（缺 key → 不触发断言），命中后才断言有效性**。
3. **WallEntities → Entity**（`0x14165dc6d..0x14165dd17`，HashMap<WallId,Entity>，entry 0x10，key WallId，value Entity 在 `entry-0x8`）。
4. **Query<&WallTriggerVolume>::get(entity)**（`0x14165dd25..0x14165ddd8`）：Bevy entity→archetype/row（`[world+0x1a8]` 表 stride 0x48，`[world+0x110]` 列索引），组件 stride **0x68**；`cmp qword [component+0x10], 0` = `quads.len != 0` 即 `is_valid()`。有效→`WidgetParams` volume tag=2（TriangleMesh 起点 `0x14165dde7`）；无效→tag=7（Empty，`0x14165de28`）。
5. **注册 per-wall widget**（`0x14165de3e..0x14165df5b`）：`UiState::add::<String>(format!("edit_circle_wall_{wall_id}"), params)`（pieces `0x142c82350 = "edit_circle_wall_"`，WallId formatter `0x14034eec0`；调用 `0x1409231f0` 返回 `(UiState*, WidgetId)`）。
6. **活动 widget 判定**（`0x14165df67..0x14165df7e`）：`ui_state[+0x18] != 0 && ui_state[+0x38] == WidgetId && ui_state[+0x28] != 2` → 选中该 `wall_id`；否则 `jmp 0x14165dad5` 继续下一面墙。
- 实现等价写法：对每面墙计算 `WidgetId::from_name(format!("edit_circle_wall_{id}"))` 与 `UiState.active_widget()` 比对即可（无需真正插表，除非要保持逐帧注册副作用）；命中后再 `WallEntities::wall_entity(id)` + `Query<&WallTriggerVolume>::get(entity)`。
- **widget 几何构造（`0x14165de0a..0x14165de38` 实读）**：
  - 有效分支（`quads.len@+0x10 != 0`）：`rcx=&rbp+0x2d8`(out)、`rdx=`Query 取到的 WallTriggerVolume 组件基址（stride 0x68）、`r8=&rbp+0x1b0`(params)，`call 0x1408e80f0`。该地址 = **`WallTriggerVolume::get_tris`**（skeleton 编译实例 RVA 0x8e80f0；内部调 `Triangle::new`/`RawVec::grow_one`/`time_continuous_lerp`/`clamp`），返回 `Vec<Triangle>`（24B，落在 `0x2d8..0x2ef`）。随后 `[rbp+0x2d0] = al = 2`（体积 tag = `TriangleMesh`）。
  - 无效分支：`al = 7`（`WidgetGeometry::Empty`），不调用 get_tris。
  - 32B 体积描述符（`0x2d0..0x2ef`）= `{tag u8 @0, Vec<Triangle> @+8}`，整体拷到 `WidgetParams+0x88`（= repo `volume`/geometry 槽，`0x14165dec7..0x14165dedb`）。
  - `get_tris` 的 params 本地（`0x1b0`）：`{+0x00=0, +0x0c=1, +0x10=Vec2(0.2875,0.0), +0x18=-0.5f, +0x1c=1}`（`0x14165dde7..0x14165de03`；0.2875@`0x142aa5d40`）——即 `WallTriggerVolumeParams::edit_shape_default` 一类的编辑形状默认参数。
  - **实现结论**：几何 = `WallTriggerVolume::is_valid() ? WidgetGeometry::TriangleMesh(tv.default_tris().to_vec()) : WidgetGeometry::Empty`；**不是 Sphere/SphereCloud**。repo 无 `get_tris`/`edit_shape_default`，用现有 `default_tris()`（recompute 已按默认参数生成）等价替代；若要与原版逐位一致，补 `get_tris(params)` 并用 `edit_shape_default()` 的 `{u_range 0.2875, offset -0.5, flags 1}`。
  - **WidgetParams 其余字段**（`0x14165dedf..0x14165df43`）：`+0x00=0, +0x08=8, +0x10=0, +0x18=1, +0x20=0, +0x30=2, +0x48=3, +0x60=3, +0x78=0(byte), +0x88=体积描述符, +0xa8=0x0101(word)`；`UiState::add` 前无需其它写操作（无 signifiers 工厂、无 active_params）。

#### S2 投影实现确认
- `ui_edit_circle` 用的是 **`utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray`**（原生 `0x140ac84a0`），不是简化的射线-平面求交：
  - `ProportionalScreenSpace`（原生 `0x140ac8420`，内联）来自 `Res<WindowSize>` 的 u32 `{w,h}` → `{size:(w,h), aspect:w/h, unit:1.0}`（`0x14165e47c..0x14165e4b7`）。
  - `world_to_view`/`view_to_clip` 来自 `Res<MainCamera>`（`[rbp+0xd0]/[rbp+0xe0]` 组）。
  - `range = [0.0, f32::MAX]`（常量 `0x142abcb20 = 00000000 ffff7f7f`）。
  - 调用点 `0x14165e556`（及 `0x14165ea06`）。
- **传给 `project_onto_ray` 的 ray（`0x14165e451..0x14165e556` 实读）**：不是相机过光标射线、不是地平面/竖直平面求交，而是由 **arg7 组第 2 项**（arg7 = `(Local<CircleGrabParams>, Local<CircleSignifierParams>, ResMut<AnimatedCircleSignifierResource>)`，指针 `[rbp+0x3a0] = [rbp+0x4a0]+8`）的 12B 记录 `P` 构造的水平线段：
  ```rust
  let p = &*(p_ptr as *const [f32; 3]);       // P = CircleGrabParams/CircleSignifierParams 的前 12B
  let origin    = Vec3::new(p[0], p[2], p[1]); // xvy(P.xy, P.z)：0x140c9cde0 @0x14165e464
  let direction = Vec3::new(p[0], 0.0, p[1]);  // xz0(P.xy)：0x140c9cdc0 @0x14165e477
  let ray = Ray { origin, direction };
  ```
  - `center`（`raw_r` 的圆心）= **同一个 `origin`**（`xvy` 输出，decomp `fStack_33c/338/334`）。
  - 原生半径增量（decomp 1540-1547）：`sqrt((hit.x-cx)²+(hit.y-cy)²+(hit.z-cz)²) - f32(entry[-0x3c])`；因 `direction.y=0`，`hit.y==origin.y==center.y`，退化为 `distance_xz(hit, center) - old_radius`。
  - 坐标 `self` = `from_screen_space(CursorPositionSS, screen)`；screen 来自 WindowSize；world_to_view/view_to_clip 来自 MainCamera。
  - [FIELD-NAMES-SYNTHESIZED]：`p` 的具体字段名（中心/半径/抓取参数）不可判；但**数值构造与上两行逐位对应**，实现可直接照抄。
- **`entry[-0x3c]` = 圆半径（S2 的“旧半径”）** `[SEMANTICS-DECODED]`：
  - 位置：`entry = PublicWallState` 桶值尾，`entry[-0x3c] = shape_rec[0x264] = ShapeParameters+0x0c`；Circle 变体此槽为 `Circle2d.radius`（Rectangle 变体为 `cx`，本系统是圆编辑 → 半径）。
  - 上游：`PublicWallState::shape()`（`shape_rec[0x258..0x274]`，`WallShapeSnapshot`/`ShapeParameters`）由墙体发布管线从 `InnerWalls` 权威 `WallState.source().shape` 重建；ui_edit_circle 本帧在 S4 才 `update_wall_state_curve(0x140aaa090)`/`update_wall_state_enclosure(0x140aa9610)`，所以读到的 `entry[-0x3c]` 是**本帧提交前的已发布半径**（old_r）。不是 UiState/Local 字段。
  - 为何相减：decomp `*(f32*)(pbStack_130+0x14) = sqrt(...) - f32(entry[-0x3c])` 写入的是 **Δr = raw_r - old_r**（拖拽增量，供半径缓动/释放判定 `|Δr|<=0.01` 使用），不是绝对半径；不能直接用 `distance_xz`。若实现处需要绝对半径，须 `old_r + Δr` 或 `clamp(old_r + Δr, 0.9, 8.0)`。

#### S5 History 构造（`0x14165ee8d..0x14165f000` 实读，可照抄）
```rust
let mut seq = WallCompoundHistoryEdit::new_edit_sequence();   // {0,8,0} 空序列
let edit = WallHistoryEdit::MorphPremadeShape(MorphPremadeShapeEdit {
    wall_index: wall_id,          // [rbp+0x3a8]
    before: prev_shape_params,    // 28B @edit+0x10 ← [rbp+0x338] 16B + [rbp+0x3a8] + [rbp+0x320]
    after:  next_shape_params,    // 28B @edit+0x2c ← [rbp+0x2b0] 16B + [rbp+0x2c0] 8B + …
});
seq.next(edit);                   // 0x140a82b40 = CompoundEditBuilder::next（单条 edit）
let edits = seq.build();          // 等价于 from_iter（0x141696f50；Vec<Arc<WallHistoryEdit>> 组）
history.add(edits, 7, file_ctx);  // 0x140a84ec0；edit_type 常量 = 7
```
- 判别：`edit+0x00 = 0x8000000000000004` = **`MorphPremadeShape`**（repo `WallHistoryEdit::MorphPremadeShape` 已存在，`MorphPremadeShapeEdit { wall_index, before, after }` 布局一致）。
- 与 case3（ui_move_shapes 提交）**不同点**：case3 的单条 edit 判别 = `0x800000000000000b`、载荷 `{wall_index@8, u16=1@0x10}`（repo 映射为 `ChangeSupports(WallValueEdit<bool>{before:true,after:false})`，**不是 StateChange**——StateChange 的判别是 wall id 本身）；S5 = `MorphPremadeShape`（前后 ShapeParameters）。共同点：都是 `next(edit) → from_iter/build → History::add(edit_type=7)`。
- `History::add` 的文件上下文指针：case3 = `0x142ab6c78` 区；S5 = `0x142c825c8`；edit_type 两者均为 **7**。

---

## 7. `signifier_edit_circle_arrow` — VA `0x140b35120` / 2144B（3 参）

PDB 3 参：1 `Query<(&EditCircleSignifier,&mut Transform)>` 2 `Res<AnimatedCircleSignifierResource>` 3 `Res<UiSignifierStream>`。

**`AnimatedCircleSignifierResource`（缺失资源，size 64）确认布局**（`decomp-0x140b35120`/`-0x140a64a40` 实读）：
```rust
#[derive(Resource)]
struct AnimatedCircleSignifierResource {
    // +0x00..0x17 其余字段（注册默认只显式写 +0x38=2；其余 UNKNOWN）
    // +0x18 f32  angle_base
    position: Vec3,   // +0x20 (x@0x20,y@0x24,z@0x28)
    normal:   Vec3,   // +0x2c (x@0x2c,y@0x30,z@0x34)
    tag:      u8,     // +0x38   (0=Circle,1=Rectangle,2=Freehand)
}
```
**控制流（`decomp-0x140b35120` 实读）**：
```
res = *param_2; tag = *(u8*)(res+0x38)
if tag != 2 {
    quat = rotation_from_normal(Vec3{res+0x2c,res+0x30,res+0x34})   // 0x140c90a40（返回 2 qword）
    pos  = Vec3{*(u64*)(res+0x20)(=x,y), *(u32*)(res+0x28)(=z)}
    name = format!(prefix@0x142aed868 + display_closure FUN_140b35b50, res)   // "edit_circle_{display}_arrow"
    rec  = {1,1, quat.0, quat.1, pos.xy, pos.z}   // 0x30 字节，分配 func_0x140613c10(0x30,0x10)
    stream.push(rec, marker 0x142aed888)          // FUN_14146c880
}
// 遍历 Query（密集/稀疏两路径）：
for (_marker: &EditCircleSignifier, mut tr: &mut Transform) in query {
    if res.tag != 2 {
        tr.rotation = quat; tr.translation = pos;   // 写到 Transform 列（0x30 stride：quat@+0..0x0f, pos@+0x10/+0x18）
        // 另写一列 f32（puStack_70，component/scale 面）
    }
}
```
- 门控：`tag==2`(Freehand) 既不发射也不写 Transform；圆(0)/矩形(1) 都发射。`EditCircleSignifier` 在本系统中仅作 Query 过滤/目标列，**其字段不被本系统读取**（仅作 marker + 写入 Transform）。
**常量**：无浮点。
**当前仓库**：`edit_arrow.rs` 用 `PublicWalls` 桶 + `pack2` 自创（INVENTED，数据源错）。
**差异/删除**：删除 `PublicWalls` 数据源与 `pack2`；改 `Query<(&EditCircleSignifier,&mut Transform)>` + `AnimatedCircleSignifierResource` + `UiSignifierStream`；`register_required_components` 从返回空切片改为真组件注册（`EditCircleSignifier` size 0x58、TypeId `(0x1ae899de41d987a3,0xc0085b7b93938bcb)`、name `0x142c37062`）。
**批次5：marker `0x142aed888` 类型描述符 + builder + display + 组件注册（Ghidra 实读）`[SEMANTICS-DECODED]`**：
- marker `0x142aed888` = arrow 载荷类型描述符：`{0x0, 0x20(size), 0x10(align), 0x140b30d90(drop), 0x140b35650, 0x140b35650}`，源路径 @`0x142aed8a8` = `crates/country-core/src/systems/wall/shapes/circle/signifiers/edit_arrow...`（size 0x20）。
- worker `0x140b35650`（133 行）：**与墙高同一管线**——调用 `FUN_14089dd50`（通用 `"signifier custom w colors"` 顶点色网格，shader `"vertex_color_shader"`@`0x142a9948c`），材质回调 `FUN_1412d89b0`/`FUN_141564d30`，返回 **1 个实体**。→ arrow 也是**通用顶点色网格，非专属几何**。
- display format 闭包 `FUN_140b35b50`（11 行）：`FUN_140d86bf0(out, "WidgetId"@0x142aed970 len8, "hash"@0x142aed978 len4, &captured, ...)` → `display` = **WidgetId 的 hash**（渲染为 "WidgetId…hash…"），**不是 "Circle"/"Rectangle" 字符串**；即 `edit_circle_{WidgetId-hash}_arrow`。
- 组件注册 `0x140b35ba0`（430 行）：size **0x58**（`uStack_90=0x58`）、TypeId **(0x1ae899de41d987a3, 0xc0085b7b93938bcb)**、name@`0x142c37062`=`country_core::systems::wall::shapes::circle::signifiers::edit_arrow::EditCircleSignifier`（注册 required 子组件 + insert_slot）。→ **确认：`register_required_components` 应返回此 size/TypeId/name**。
- **每顶点颜色来源（`FUN_14089dd50` default 分支 = "custom w colors"）**：`Vertex_Color` 属性 = **payload 自带的颜色数组**：`param_2[3]`=points ptr、`param_2[4]`=points count（`param_2[3]..+param_2[4]*0xc`，12B/点）；`param_2[5]`=**colors ptr**、`param_2[6]`=**colors count**、`param_2[7]`=**color stride**；`FUN_140a34c90(&out,"Vertex_Color"(0xc), {ptr=param_2[5],count=param_2[6],stride=param_2[7]})`。→ **颜色来自 payload 颜色数组，不是材质 f32 对、也不是常量**。要让 adjust/§7 有真实颜色，**产出方（stream 消费者）必须按 WallId 从墙颜色表填充该颜色数组**（这一步未解，需消费者反编译）。
- **§7 points 来源（纠正"单点占位"）**：arrow worker 把 `param_2[3]`(ptr)/`param_2[4]`(count) 原样转发为 builder 的点数组（`lStack_160..+param_2[4]*0xc`）。即 arrow **不是单点**，点来自 worker 的输入记录（`EditCircleSignifier`/圆资源）的 [3]/[4]；`points=vec![pos]` 是错误占位。**input record 的 [3]/[4] 来源待查**（worker param_2 = ?）。
- **WidgetId 捕获（`FUN_140b35b50`）**：`captured = lStack_110 = lVar17 = *param_2`（arrow 系统 `param_2` = `AnimatedCircleSignifierResource` 指针）。闭包渲染为 `"WidgetId{hash}"`。**待确认**：captured 是资源本身还是其某偏移的 `WidgetId`（资源 +0x00 是否为 WidgetId）。
**测试**：tag 2/0/1 三态（发射与否 + Transform 写与否）；rotation_from_normal 输入 = normal 三 f32；记录 0x30 五字段；Query 密集/稀疏两路径；**颜色数组非白占位**（按 WallId 查色表）。
**STATUS: READY**（系统逻辑、资源布局、记录、门控、名字、marker/builder/display/组件注册均实读；颜色=payload 颜色数组、points=输入记录 [3]/[4] 已澄清；`EditCircleSignifier` 为 Query marker，本系统不读其字段）

### §7 本轮补充：builder 分支判别 + `flat_arrow` + WidgetId 捕获 `[SEMANTICS-DECODED]`
**builder 判别码**（`FUN_14089dd50` `0x14089dd50` 首个 `switch`，`builder_1.c`）：
```
switch (payload[2] ^ 0x8000000000000000) {
  0 => Named 资产（按名字查 AssetMeshLibrary，缺失 panic "Mesh <name> not found"）
  1 => Empty（墙高 "signifier empty" 走此支）
  2 => Points（"signifier custom"）
  3/default => PointsWithColors（"signifier custom w colors" + "Vertex_Color"）
  4 => Generated（"signifier custom fn"）
}
```
- **arrow 的 discriminant = `0x8000000000000000` → case 0 = Named 资产**（worker `arrow_1.c` 写 `uStack_80=0x8000000000000000` 后拷到 payload+0x10）。
- **墙高 = `0x8000000000000001` → case 1 = Empty**（`fn_9756d0.c` `uStack_70=0x8000000000000001`）。**所以 arrow 与墙高不同支。**
- case 0 worker 构造名字 `String "flat_arrow"`：10 字节（`0x7272615f74616c66` + `0x776f` = `66 6c 61 74 5f 61 72 72 6f 77`），payload 里 `{len@+0x18=10, ptr@+0x20, cap@+0x28=10}`；builder 用 `FUN_14092d570(asset_lib, name_ptr, name_len)` 查表（HashMap: hash@+0x68、table@+0x88），命中后 `free(name)` 并返回 1 个实体；`uVar13&1==0` 即 `None` → panic（`"Mesh "` + `asset_libraries/mesh_library.rs`）。
- **shader**：worker 另分配 19 字节 `"vertex_color_shader"`（`0x142a9948c`, len 0x13）；材质回调 `FUN_1412d89b0`/`FUN_141564d30`。
- **修正 §7 旧"颜色来自 payload 颜色数组"**：那是 **case 3 (PointsWithColors)** 的语义；arrow 走 **case 0 Named 资产**，顶点色来自 `flat_arrow` 资产网格本身，**不读 payload 颜色数组**，也不要构造 `colors`。
- **§7 points 纠正**：arrow **没有输入点集**；`points=vec![pos]` 与"payload 点数组"都不适用。case 0 只消费资产名。

**构造结论（实现线照抄）**：
```rust
// edit_arrow.rs：用资产名分支，不是 Empty、不是 PointsWithColors
SignifierMesh::Named("flat_arrow".into())
// 消费端在 SignifierEntityBuilder 中经 AssetMeshLibrary::get_handle_by_name("flat_arrow")
// 解析 Handle<Mesh>；缺失应 panic（与原版一致），不要静默降级为 Empty。
// 需要顶点色 shader 时用 "vertex_color_shader"（原版材质回调 0x1412d89b0/0x141564d30）。
```
> 对照：`ui_adjust_wall_height.rs` 的 `SignifierMesh::Empty` 对应墙高（case 1），**不应套用到 arrow**。

**WidgetId hash 捕获字段**（`FUN_140b35b50` + `spec_arrow.c`）：
- 系统内：`lVar17 = *param_2`（`Res<AnimatedCircleSignifierResource>` 的指针）；`lStack_110 = lVar17`；随后把 `&lStack_110` 放入 fmt 参数（1 个参数）。
- 闭包 `FUN_140b35b50(param_1,param_2)`：`uStack_8 = param_1; FUN_140d86bf0(out,"WidgetId"(8),"hash"(4),&uStack_8, ...)`。
- ⇒ **捕获值 = 资源基址/`+0x00` 处的值**（`*param_2`），不是某个 `+0x08..+0x17` 字段。按 `FUN_140d86bf0` 的 "WidgetId"/"hash" 片段，输出形如 `WidgetId { hash: … }`。
- [FIELD-NAMES-SYNTHESIZED]：`AnimatedCircleSignifierResource +0x00` 存放 **WidgetId (8B)**（被格式化的 hash 源）；`+0x08..+0x17` 本链路不读，保持 UNKNOWN。
- 名字片段：`["edit_circle_", "_arrow"]`（`0x142aed850` len12 / `0x142aed85c` len6）→ `edit_circle_{WidgetId}_arrow`。

**记录字段（`spec_arrow.c`）**：`alloc(0x30,0x10)`，`{+0x00 u64=1, +0x08 u64=1, +0x10 Quat(16B)=rotation_from_normal(res.normal), +0x20 Vec3 pos(x@0x20,y@0x24,z@0x28)}`；push marker `0x142aed888`（payload 类型 size 0x20/align 0x10/drop 0x140b30d90/build 0x140b35650）。门控：`res.tag(+0x38)==2` 时既不 push 也不写 Transform；`0/1` 都发射。
**组件注册**：`EditCircleSignifier` size `0x58`、TypeId `(0x1ae899de41d987a3,0xc0085b7b93938bcb)`（`0x140b35ba0`）——`register_required_components` 必须返回它，不能返回空切片。

---

## 8. `signifier_edit_circle_line` — VA `0x140a64a40` / 3680B（4 参）

PDB 4 参：1 `Query<(&mut SignifierEditCircleShapeLine,&Handle<Mesh>,&SignifierAnimation)>` 2 `Res<AnimatedCircleSignifierResource>` 3 `ResMut<AssetMeshLibrary>` 4 `Res<UiSignifierStream>`。

**控制流（`decomp-0x140a64a40` 逐指令实读）**：
```
res = *param_2; tag = *(u8*)(res+0x38); angle_base = *(f32*)(res+0x18)
if tag==2 || (tag&1)!=0 {                       // Freehand 或奇数(矩形)：走占位
    c = Circle2d::new(0,0, angle_base + 0.23)   // 0.23@0x142a89600
    p = c.get_pos_at_u(...); d = (c.center - p)
    push 0x28 记录 {1,1, d.x,d.y, u32,u32, res[1]}  // 名字 0x142ad44f8 / 闭包 FUN_140a669e0
}
per_wall:
  iterate Query（密集/稀疏）:
    anim = *(f32*)(SignifierAnimation[idx] + 0x18)
    s = anim*0.1 + (1.0 - anim)*0.01            // 0.1@0x142925984, 0.01@0x142960674, 1.0@0x142925904
    a = 0.75 - s                                // 0.75@0x1429edb90
    assert 0.0 <= a <= 1.0 && !NaN(a)           // 否则 panic 0x142aa53d0 len0x1d (col 0x1d)
    b = s + 0.75
    assert b >= 0.0 && b <= 1.0 && !NaN(b)      // panic 同上
    if tag != 2 {
        c = Circle2d::new(0,0, angle_base + 0.23); p = c.get_pos_at_u(...)
        d = c.center - p
        endpoints = [d.x, d.y, aux.x, aux.y]    // 写入组件列
    }
    curve = Circle2d::as_curve2_range(0.2, a, b)   // 0.2@0x1429cd068
    *(u32*)(mesh_lib + 0x1c) = anim_u32
    update_curve_mesh(&curve, mesh, ...)
```
- 门控：`tag==2 || tag&1` → 占位；**仅偶数 tag(圆=0)** 进入真实半径线端点写。
- 常量：0.23/0.75/0.2/0.01/0.1/1.0（§0.2 实读）。
**`AnimatedCircleSignifierResource`**：见 §7（`angle_base@+0x18`、`position@+0x20`、`normal@+0x2c`、`tag@+0x38`）。
**当前仓库**：`edit_line.rs` 用全局 `PreviewCircle` + `pack2`（INVENTED，数据源错）。
**差异/删除**：删 `PreviewCircle` 数据源与 `pack2`；改 Query + `AnimatedCircleSignifierResource` + `AssetMeshLibrary`；`register_required_components`（`SignifierEditCircleShapeLine` size 0x60、TypeId `(0x7c65f1f941b3ec27,0x92748f8a36511053)`、name `0x142c372f6`）。
**缺证据（PENDING，唯一）**：组件列基址歧义——写入点 `pfStack_1b8 = storage + idx*0x28`（0x28 stride）与注册 size **0x60** 不一致，需对 `0x140a64a40` 的列解析段（0x140a64e00..0x140a64f60）逐指令确认 `lVar22`/`lStack_100`（组件列）与 `lStack_100+0x18`（SignifierAnimation 进度）各自的列基址与 stride；其余（公式/门控/常量/记录/网格更新）均已实读。
**STATUS: PENDING**

---

## 9. `ui_rotate_circle` — VA `0x1409ac910` / 7744B（21 参）

PDB 21 参：1 `Res<AppMode>` 2 `ResMut<InnerWalls>` 3 `Res<PublicWalls>` 4 `DecoratorArchivalResources`
5 `ResMut<DecoratorArchivist>` 6 `ResMut<DecoratorStorage>` 7 `Res<UiState>` 8 `Res<UiSignifierStream>`
9 `Res<WindowSize>` 10 `Res<MainCamera>` 11 `Res<CameraCursorRay>` 12 `ResMut<History>`
13 `EventWriter<OnCircleRotated>` 14 `EventWriter<RecalculateRoofCmd>` 15 `EventWriter<ShowPinnedDecoClutter>`
16 `EventWriter<NotifyHintSystemShapeEditWasPerformed>` 17 `EventWriter<PlayBuildingAudio>` 18 `EventWriter<PlayUiSound>`
19 `Local<RotateCircleCache>` 20 `Local<CurrentAngleGizmo>` 21 `Res<GameTime>`。

> **参数数更正**：任务清单写 `ui_rotate_circle` 为 20 参，PDB `FunctionSystem` 拼写为 **21 参**（含 2 个 Local 与 `Res<GameTime>`）。以 PDB 为准。
控制流（`spec/circle_rotate_and_signifiers.md`）：门 `mode<=5`；仅 1/2；UiState wall；断言；非圆 return；
屏幕最近曲线点（view-proj：`sx=(p·row+row3) ; ndc=num/den*0.5+0.5 ; screen_x=ndc*viewport`；`best` 初值 1e7）；方向 `d/|d|`×0.3 偏置；`box_trigger_volume`+`UiState::add`；
stage=ctx[+0x19]（有效）else 2；`switch(stage-2)`：0→skip 1→drag（归档+cull+脊端点反解+polar+angle）2→release 3→提交（History/OnCircleRotated 0x10/RecalculateRoofCmd 0x8）；
角度 gizmo（`active&1`；ease 速率 `GameTime+0x28 × -10.0`；`CircularF32::lerp`）；release 判 `|Δangle|<=0.01`→PlayUiSound{0x23}，否则 eased→PlayUiSound{0x22,eased,polar,gen}。
常量：0.01/0.5/1.0/0.1/0.2/0.23/0.75/90/1e7/0.3/-10.0/0.8（§0.2）。
当前仓库：`ui_rotate_circle.rs`(46 行) `RotateCircleCmd` 事件壳（INVENTED）。差异/删除：删 `RotateCircleCmd` 与事件循环；改 21 参系统；`RotateCircleCache`/`CurrentAngleGizmo` 新建。
**缺失类型与已知布局**：
- `RotateCircleCache`（Local）：至少 7×f32 槽：`+0x00 f32 x`、`+0x04 f32 y`、`+0x08 f32 angle(CircularF32)`、`+0x0c..+0x10 [f32;2]`、`+0x14 f32`、`+0x18 u8 state`（0/1/2 阶段标记）。字段名/类型部分 UNKNOWN。
- `CurrentAngleGizmo`（Local，16B）：`+0x00 u32 active`、`+0x04 f32 angle(CircularF32)`、`+0x08..0x0f 保留 0`；默认全 0。
- `OnCircleRotated`（缺失事件）：push 点行 483-492，行宽 **0x10**，两个 qword `{+0 u64, +8 WallId}`（+0 是写入计数/代数、+8 是 WallId）；属 `resources::events`，需新建并登记。
- `RecalculateRoofCmd`（缺失事件）：push 点行 559-566，行宽 **0x8**（只写一个 `u64 = 计数`）→ 判定为 **ZST/unit 事件**（`EventInstance` 仅 id）；归 `resources::roofs::commands`。
- `PlayUiSound`（已存在）：无变化写 `{0x23}`、有变化写 `{0x22,eased,polar,gen}`；`ShowPinnedDecoClutter`（已存在）0x40 行；`NotifyHintSystemShapeEditWasPerformed`（ZST）0x28 行（见行 ~0x?? 段）。`History::add` 在行 555。
**缺证据（PENDING）**：`RotateCircleCache`/`CurrentAngleGizmo` 字段名（偏移已知）；view-proj 矩阵列序对齐（`MainCamera` 字段↔puVar18 +0x10..0x3c）；stage 枚举名（`UiState::add` 返回体 +0x18/+0x19）；`OnCircleRotated` 行 +0/+8 谁是 event_id 与谁是 payload（需一次 Events::send 内联确认）。
**STATUS: PENDING**

---

## 10. `ui_edit_rectangle_dims` + `do_ui_edit_rectangle_dims`

### 10.1 `ui_edit_rectangle_dims` — VA `0x140ad6250` / 2992B（11 参）
PDB 11 参：1 `Res<AppMode>` 2 `Res<UiSignifierStream>` 3 `Res<UiState>` 4 `Res<CameraCursorRay>`
5 `Res<PublicWalls>` 6 `Res<WallEntities>` 7 `Query<&WallTriggerVolume>` 8 `EventReader<MainCameraMoved>`
9 `ResMut<UiEditRectangleActiveWidgetResource>` 10 `EventWriter<DoUiEditRectangleDims>` 11 `Commands`。
控制流：`func_0x140c16a00()` 拖动门；.rdata 路径扫描分支；遍历 reader（0x70 行）；命中边 `ev[0x60]`；向 `DoUiEditRectangleDims` 写行；`Commands` 用于 widget 实体。
### 10.2 `do_ui_edit_rectangle_dims` — VA `0x140ad6e00` / 4608B（12 参）
PDB 12 参：1 `Res<PublicWalls>` 2 `ResMut<DecoratorArchivist>` 3 `ResMut<WipRectangle>` 4 `EventWriter<EditRectangleCmd>`
5 `EventWriter<FinishEditRectangleCmd>` 6 `Slider3d` 7 `Res<GladeBorder>` 8 `EventWriter<DisplayOutOfBorder>`
9 `EventWriter<FeedbackInputOutOfBorder>` 10 `EventWriter<HideTreesinAreaCmd>` 11 `EventWriter<ShowPinnedDecoClutter>`
12 `EventReader<DoUiEditRectangleDims>`。
控制流（`dd_body.c`）：读 0x70 行；tag==2 skip；`edge=ev[0x60]`；hash 断言；`switch(edge-2)`（含 default 分支）；
`L² = e.x²+e.y²; if L²<=1e-20(0x142a7baa0){L²=1e-20}`；`t=((cur-p_i)·e)/L²; 1-t`；写 EditRectangleCmd（0x40 行，`+0x38=0`）；`Slider3d::project`；有效性 `fun_140ad2970`/`fun_140ad2ce0`。
常量：1e-20(0x142a7baa0)、1.0、1.5(0x3fc00000、0x30 行)。
当前仓库：`ui_edit_rectangle_dims.rs` 用 `RectangleDimsEditInput`（INVENTED）+ `RectangleEditUiState([u8;80])` + `expand_along_edge`。差异/删除：删 `RectangleDimsEditInput`、`RectangleEditUiState` 的 80B 猜布局；新建 `WipRectangle`/`UiEditRectangleActiveWidgetResource`/`DoUiEditRectangleDims`/`Slider3d`。
**缺失类型与已知布局**：
- `WipRectangle`（size 80，注册默认 `+0 u32 = 0`）：判别式 `tag@+0 u32`：0=Idle、1=Dims、2=Rotate。
  - 变体 1（Dims）：`+0x04 i32` wall 低位、`+0x08 i32` wall 高位、`+0x10..0x1f` 旧轴/中心块、`+0x20..0x37` 参数块、`+0x48 f64`。
  - 变体 2（Rotate）：`+0x08 u64` wall、`+0x10..0x1f`、`+0x20..0x3f`、`+0x48 f64`。
  - 字段名多为语义推断（PDB 只给顶层名与 tag 取值）；**字节偏移来自反编译解引用**。
- `UiEditRectangleActiveWidgetResource`（size 80，默认 `+0 qword = 0x8000000000000000`）：`+0x18` 命中边 0..3（do_dims 读 `(char)piVar20[0x18]`）。
- `Slider3d`（SystemParam，非资源）：`FUN_140b1cc80(out, param_6, edge, ...)` 把命中边投影到世界滑移；内部字段未展开。
- `DoUiEditRectangleDims`（缺失事件，行宽 **0x70**，`dd_body.c` `piVar20` 实读）：读点 `+4,+0xc,+0x14,+0x1c,+0x24,+0x2c,+0x34`（每处 8B，覆盖 `+4..+0x37` = 6×f32 Rectangle2d + 锚点）+`+0x38 u64`、`+0x40..0x57`（2×u64）、`+0x58 u64`、**`+0x60 i32 = 命中边 edge`**（`(char)piVar20[0x18]`）。`[SEMANTICS-DECODED 偏移 / FIELD-NAMES-SYNTHESIZED 名]`
- `Slider3d::project`（`FUN_140b1cc80`，Ghidra 130 行）：签名 `project(out:&mut [..;5], self:&Slider3d, edge:u8, geom:&_, cursor:&[f32;2])`；`puVar2=*self`；`if (edge-2)<3`（合法边）；用 `FUN_140ac84a0`(相机投影) 把 cursor 影射到边世界滑移线，写 `out[1..4]`，返回 `out` 或 `self[5]`。**精确投影运算在 `FUN_140ac84a0`**（需其反编译）。
**Slider3d 投影 / 合法性 helper（Ghidra 实读）** `[SEMANTICS-DECODED]`：
- `FUN_140ac84a0`（Slider3d::project，103 行）：`(out, cursor_x, cursor_y, geom, viewproj:&[f32;16], rect, cursor)`；把边两端点用 **4x4 view-proj（param_5 行 0x00..0x3c）**投到屏幕空间，取 `t = ((cursor-s0)·(s1-s0)) / max(|s1-s0|², 1e-20)`（除零下限 `0x142a7baa0`=1e-20），返回世界点 `t*p1 + (1-t)*p0`（`1.0`@0x142925904；屏幕倒数下限 `0x142abaa00`）。
- `FUN_140ad2970`（142 行）：**单候选合法性**——按距离/尺寸比较（`dist <= size`、`1.0`@0x142925904），返回 bool。
- `FUN_140ad2ce0`（184 行）：**围合合法性**——遍历边调用 `FUN_140ad2970`（`0x142a7fa30`=20.0 作阈值）经 `FUN_140c98170` 构建候选。
**缺证据（PENDING）**：`WipRectangle` 变体 1 字段名（偏移已知）；0x70 行 `+0x40..0x58` 块语义。（Slider3d::project 与两合法性 helper 已解，见上。）
**STATUS: PENDING**

---

## 11. `ui_edit_rectangle_rotate` + `do_ui_edit_rectangle_rotate` + `box_trigger_volume`

### 11.1 `ui_edit_rectangle_rotate`（VA `0x140ad8000`）— PDB 10 参
1 `Res<AppMode>` 2 `Res<PublicWalls>` 3 `Res<UiSignifierStream>` 4 `Res<UiState>` 5 `Res<MainCamera>`
6 `Res<CameraCursorRay>` 7 `EventReader<MainCameraMoved>` 8 `Local<ClosestCornerCache>`
9 `EventWriter<DoUiEditRectangleRotate>` 10 `Commands`。
### 11.2 `do_ui_edit_rectangle_rotate`（VA `0x140ad8be0`）— PDB 12 参
1 `Res<PublicWalls>` 2 `ResMut<WipRectangle>` 3 `EventWriter<EditRectangleCmd>` 4 `EventWriter<FinishEditRectangleCmd>`
5 `Res<GameTime>` 6 `Res<CameraCursorRay>` 7 `Res<GladeBorder>` 8 `EventWriter<DisplayOutOfBorder>`
9 `EventWriter<FeedbackInputOutOfBorder>` 10 `EventWriter<HideTreesinAreaCmd>` 11 `EventWriter<ShowPinnedDecoClutter>`
12 `EventReader<DoUiEditRectangleRotate>`。
控制流（`dr_body.c`）：读 0x70 行；`corner=ev[0x60]`；hash 断言；跳表 `0x142ae19b8`：
case1→相机投影取方向（`(coord-pf[1])/pf[4]`，`get_rect_center`，单位化，cos，fast-acos 8 项 + π 补 + cross 符号，写 EditRectangleCmd tag=1）；
case2→朝向吸附/步进（`FUN_1409ef010`，步长 0.1，上限；循环 `FUN_140ad2970`；写 EditRectangleCmd + FinishEditRectangleCmd 1.5）；
case3→写 8B + 0x30 行（1.5）；case0→收尾。
### 11.3 `box_trigger_volume`（VA `0x140ada0e0`）— 非 SystemParam
当前仓库 `box_trigger_volume` + `skewed_box_trigger_vol` 审计判 **EQUAL**（24 点/36 索引/12 三角）；保留。
当前仓库（11.1/11.2）：`ui_edit_rectangle_orientation.rs` 用 `RectangleOrientationEditInput`（INVENTED）；`fast_acos` 系数前 4 项错且缺 π 补/符号；缺 `Slider3d`、`ClosestCornerCache`、`DoUiEditRectangleRotate`、`WipRectangle`。
差异/删除：删 `RectangleOrientationEditInput`/`RectangleEditUiState`；修正 `ACOS_COEFFS` 与 `fast_acos`；补吸附步进与相机投影。

**修正后的 `ACOS_COEFFS` 与 `fast_acos`（RNE 实读 + 反编译，直接照抄）**：
```rust
// 8 个 f32 位型，顺序即 Horner 系数 c0..c7（0x142ae0d44..0x142ae0d60）
const ACOS_COEFFS: [u32; 8] = [
    0xbaa57a2c, 0x3bda90c5, 0xbc8bfc66, 0x3cfd10f8,
    0xbd4d8392, 0x3db63a9e, 0xbe5bbfca, 0x3fc90fda, // 末项 ≈ π/2
];
fn fast_acos(cos: f32) -> f32 {
    let x = cos.abs();
    let mut p = 0.0f32;
    for c in ACOS_COEFFS { p = p * x + f32::from_bits(c); }  // Horner
    let s = (1.0 - x).max(0.0);                              // maxss(0,1-x)
    let a = p * s.sqrt();                                    // poly * sqrt(s)
    if cos < 0.0 { std::f32::consts::PI - a } else { a }     // π - a 负半支
}
// 符号：cross = b.y*a.x - a.y*b.x；最终 angle = if cross<0 { -fast_acos(cos) } else { fast_acos(cos) }
// NaN：原版 cross 为 NaN 时走 NaN 位型合成（结果 NaN），实现可用 f32 语义等价。
```
> 当前源码 `ui_edit_rectangle_orientation.rs:19` 的 `ACOS_COEFFS` 前 4 项错（`0x3bda90c5,0x3cd90a90,0x3c8bfc66,0x3dfd10f8`），且 `fast_acos` 缺 `π-a` 负半支；必须替换为上式。
**缺失类型与已知布局**：
- `DoUiEditRectangleRotate`（缺失事件，行宽 **0x70**）：do_rotate 读 `+0 i32(tag!=2) · +0x28 f32 光标 y · +0x38 i64 WallId · +0x40..0x5f 矩形块 · +0x60 i32 命中角`。
- `ClosestCornerCache`（Local）：`*param_8 = 命中角索引 u32`，`param_8[1] = 上一帧命中`（单 u32 游标 + 附加）。
- `EditRectangleCmd`（已存在但字段不全，行宽 **0x40**）：`+0 seq u64 · +0x08 WallId i64 · +0x10 Rectangle2d(24B: center.xy/dims.xy/orient.xy) · +0x28 Vec3 · +0x34 f32 拖拽标量 · +0x38 u8 flag(0=dims,1=rotate)`。
- `FinishEditRectangleCmd`（已存在）：是否携带 WallId 未定（reader 只读游标）；WallId 实际来自 WipRectangle。
**`DoUiEditRectangleRotate` 0x70 行**（`dr_body.c` `piVar20` 实读）：stride `0x70`；**`+0x28 f32 光标 y`**（`piVar20[10]`）、**`+0x38 i64 WallId`**（`piVar20[0xe]`）、`+0x40/+0x48/+0x50`（rect 块，`piVar20[0x10/0x12/0x14]`）、`+0x58 u64`（`piVar20[0x16]`）、**`+0x60 i32 命中角 corner`**（`piVar20[0x18]`）。`[SEMANTICS-DECODED 偏移 / FIELD-NAMES-SYNTHESIZED 名]`
**`FUN_1409ef010`（旋转缓动步进，Ghidra 106 行）**：签名 `(out, cur:&f32-flags, target:&f32-flags, dt:f32)`；`dt=*(f32*)(*GameTime+0x28)`；核心 `factor = 1.0 - cos(-20.0*dt)`（`-20.0`@`0x14295c698`，`1.0`@`0x142925904`），`move = factor*(target-cur)`，abs 掩码 `0x7fffffff`@`0x14292e980/984` + NaN 位型 `0x7fc00000`@`0x142a8d5c4/610/614`，写 `out[1]=cur+move`。**这是帧率无关缓动逼近，不是量化吸附**（与 rotate gizmo -10.0 ease 同族）。
**缺证据（PENDING）**：`WipRectangle` 变体 2 字段名（偏移已知）；`+0x40..0x50` 块语义；`ClosestCornerCache` 完整字段；`FUN_1409ef010` overshoot 钳制分支逐位（已到"缓动逼近"粒度）。
**STATUS: PENDING**

---

## 12. `signifier_edit_dims` — VA `0x1408dda10` / 4704B（4 参）

PDB 4 参：1 `Res<UiEditRectangleActiveWidgetResource>` 2 `Res<UiSignifierStream>`
3 `Query<(&EditRectangleLinePreview,&Handle<Mesh>,&SignifierAnimation,&mut Transform)>` 4 `ResMut<AssetMeshLibrary>`。
控制流（`sigdims_body.c`）：`a=*active`；`a>0`→widget 实例构造（0x58 记录，从 `plStack_c0[2..9]`）；`a<=0`→遍历曲线点集合（`+0x1a8` 表 0x48 步长，`+0xf8` 边索引 / `+0x100` 计数）：`fVar18=curve_point[+0x18]`；`fVar19=fVar18*0.5+(1-fVar18)*0.01`；`u0=0.5-fVar19`、`u1=fVar19+0.5`；越界/NaN→panic `0x142aa53d0`（col 0x1d）；`FUN_141466bb0`(curve_u 断言)、`FUN_14157ef70`(signifier 流)。
常量：0.01/1.0/0.5。
当前仓库：`signifier_edit_dims_line.rs` 4 边 + `pack2`（INVENTED）。差异/删除：改 Query + `UiEditRectangleActiveWidgetResource`；删 `PreviewRectangle` 数据源与 `pack2`；`register_required_components`（`0x1408e0e80`）。
**缺失类型**：`EditRectangleLinePreview` 组件（signifier Query 第 1 元素；遍历曲线点集合 `+0x1a8` 表 0x48 步长、`+0xf8/+0x100` 边索引，每点 `+0x18` 端点插值；字段名 UNKNOWN）。
**已解控制流（`sigdims_body.c` 实读）`[SEMANTICS-DECODED]`**：`plStack_c0=*param_1`（Res<UiEditRectangleActiveWidgetResource>）；遍历 ECS（`+0x100` archetype 0xa0、`+0x1a8` 组件表 0x48、组件列 `+0x18` 0x30 stride、SignifierAnimation `+0x1c` stride，进度 `+0x18`）：`t = anim*0.5 + (1.0-anim)*0.01`（`0.5`@`0x142925870`、`0.01`@`0x142960674`、`1.0`@`0x142925904`）；`u0 = 0.5 - t`、`u1 = t + 0.5`；断言 `0<=u0,u1<=1 && !NaN`（否则 panic `0x142aa53d0` len0x1d）；`FUN_141466bb0(points+idx*7, u0, u1)`（curve_u 断言）；`FUN_14157ef70(...)`（写 signifier 流）。
**缺证据（PENDING）**：`EditRectangleLinePreview` 组件字段（ECS 列语义，当前只知写入端点/半径）；widget 分支 0x58 记录字段；`UiEditRectangleActiveWidgetResource` 其余字段（默认 `+0 = 0x8000000000000000`）。
**STATUS: PENDING**

---

## 13. `edit_rectangle` — VA `0x14097f620` / 848B（5 参）

PDB 5 参：1 `EventReader<EditRectangleCmd>` 2 `ResMut<InnerWalls>` 3 `EventWriter<RequestWallMaterialAudio>`
4 `EventWriter<PlayBuildingAudio>` 5 `Res<Time>`。
控制流（`14097f620.c` 全文）：双缓冲游标差钳 0；无事件 return；逐 0x40 行：`rect=ev[+0x10]`；`Rectangle2d::as_curve2`；
`_try_update_state::<curve>`、`::<enclosure>`；push 0x20 行 `mat_audio{seq@+0, WallId@+8, params@+0x10, u32@+0x18, u16=4@+0x1c}`；
屋顶高度 ease：`num=ev[+0x34]; den=time[+0x58]; e=inverse_lerp(0, rate, num/den)`；`e=max(0,e); f=min(e,1.0)`；
push 0x28 行 `bld_audio{seq@+0, WallId@+8, params@+0x10, u32@+0x18, f32 ease@+0x1c, flag u8@+0x20}`；
**flag@+0x20 逐字等于 `ev[+0x38]`**（`ev[+0x38]==0` 分支写 0，else 写 1，两分支唯一差别即此字节）。
常量与分支（**反汇编确认**，`er_full.asm`）：
- `ev[+0x38] == 0`（**dims 源**，`do_dims` 写 0）→ `xmm1 = 0x142a13848 = 200.0`，输出 `flag@+0x20 = 0`（`0x14097f895` 块）。
- `ev[+0x38] != 0`（**rotate 源**，`do_rotate` 写 1）→ `xmm1 = 0x142a7fa30 = 20.0`，输出 `flag@+0x20 = 1`（`0x14097f801` 落空块）。
- ease 数学：`xmm2 = ev[+0x34] / Time[+0x58]; xmm0 = 0.0; xmm1 = rate; call inverse_lerp; xmm7 = max(0,xmm0); xmm6 = 1.0; ease = min(1.0, xmm7)`。
  即 `ease = clamp01(inverse_lerp(0.0, rate, num/den))`（先 max(0,·) 再 min(·,1.0)）。
- 1.0(`0x142925904`)、200.0(`0x142a13848`)、20.0(`0x142a7fa30`)。
写出：`RequestWallMaterialAudio`(0x20 行)、`PlayBuildingAudio`(0x28 行)。
当前仓库：`edit_rectangle.rs`(30 行) 已做曲线/围合更新（PARTIAL）；缺 0x20/0x28 两事件与 ease。
差异：补两事件 + inverse_lerp + min/max 顺序；`EditRectangleCmd` 需含 `+0x28 Vec3`/`+0x34 f32`/`+0x38 u8`(当前缺)。
测试：无事件早退；ease clamp 顺序（NaN→0）；flag 两分支（dims→200/flag0，rotate→20/flag1）；行宽。
**STATUS: READY**（唯一未定项已由 `0x14097f620` 反汇编解出：dims→200.0/flag0、rotate→20.0/flag1）

---

## 14. `finish_edit_rectangle` — VA `0x141464a60` / 2016B（9 参）

PDB 9 参 + 反编译映射（`fer_body.c` 实读）：
1 `Res<InnerWalls>`(in_RCX) · 2 `ResMut<WipRectangle>`(in_RDX, `piVar8=*in_RDX`) ·
3 `ResMut<History>`(in_R8) · 4 `ResMut<DecoratorStorage>`(in_R9) · 5 `ResMut<DecoratorArchivist>`(in_stack_00000028) ·
6 `EventReader<FinishEditRectangleCmd>`(in_stack_00000030) · 7 `EventWriter<OnRectangleEdited>`(in_stack_00000038) ·
8 `EventWriter<NotifyHintSystemShapeEditWasPerformed>`(in_stack_00000040) · 9 `EventWriter<CmdCheckForSheepInsideBuilding>`(in_stack_00000048)。

**`WipRectangle` 布局** `[SEMANTICS-DECODED 偏移 / FIELD-NAMES-SYNTHESIZED 名]`（`piVar8`；tag `@+0 u32`：0=Idle,1=Dims,2=Rotate）：
- 变体 1（Dims）：`+4 i32`(iVar4) · `+8 i32`(wall 低位，snapshot 用) · `+0x10..0x1c i32×4` · `+0x20..0x2f 2×u64` · `+0x30..0x47 (u64+16B)` · `+0x48 u64`；
  判据 f32：`+0x20,+0x24,+0x30,+0x34,+0x38,+0x3c,+0x48,+0x4c`；判据 wall=`+0x10 i64`。
- 变体 2（Rotate）：`+4 i32` · `+8 u64`(wall，snapshot+判据) · `+0x10..0x1c i32×4` · `+0x20..0x2f` · `+0x30..0x47` · `+0x48 u64`；
  判据 f32：`+0x18,+0x1c,+0x28,+0x2c,+0x30,+0x34,+0x40,+0x44`；判据 wall=`+0x8 i64`。

**控制流（实读）**：
```
tag = WipRectangle[+0]
if tag != 0 {
    (variant, snapshot) = tag==2 ? (2, read variant2) : (1, read variant1)
    OnRectangleEdited.push_0x58(snapshot)          // 活动态先发（在读 finish 事件之前）
}
// finish reader 游标钳 0；无事件 → return（OnRectangleEdited 已发）
// 双判据（tag==1/2 各用上表偏移，否则 panic）：
dot = f32[lVar21]*f32[lVar16] + f32[lVar15]*f32[lVar32]   // 见变体偏移；Dims:+0x38*+0x20 + +0x3c*+0x24
ang = acosf(dot)
if (|ang| * 57.29578) <= 0.01 {                    // 0.01@0x142960674, 57.29578@0x42652ee1
    if sqrt((fVar41-fVar5)² + (fVar36-fVar6)²) <= 0.01 {
        DecoratorArchivist::extract_and_restore_to_storage(); // in_stack_28
        WipRectangle[+0] = 0; return;              // LAB_14146504f
    }
}
// 超阈：InnerWalls(hash stride 0x78) 查 wall
slot = probe(wall); variant = *(u32*)(slot-0x38)
if variant > 1 { panic_fmt }; if variant != 1 { panic }
CompoundEditBuilder::next(); ConcurrentEdits::from_iter(Map<IntoIter<Vec<Arc<WallHistoryEdit>>>,impl_0::new::closure>); History::add()
NotifyHint.push_8(WriteSettingsFileCmd EventInstance)       // 8B id
CmdCheckForSheep.push_0x10(WindowFocusEvent EventInstance)  // 16B {+0 event_id, +8 WallId}
WipRectangle[+0] = 0; return
```
**`OnRectangleEdited` 0x58 记录**（`[SEMANTICS-DECODED]`，producer=本系统，reader=下游；字段名合成）：
`{ +0x00 u32 variant(1/2), +0x04 i32 iVar4, +0x08 u64 WallId, +0x10..0x1c i32×4, +0x20..0x2f 2×u64, +0x30..0x47 (u64+16B), +0x48 u64 uVar7, +0x50 u64 event_id }`（行宽 0x58）。
**常量**：0.01(`0x142960674`)、57.29578(`0x42652ee1`)。
**当前仓库**：`finish_edit_rectangle.rs`(53 行) 用 `RectangleEditUiState([u8;80])` + `edit_decision`（PARTIAL；判据 EQUAL；外围 ConcurrentEdits/WriteSettingsFileCmd/WindowFocusEvent 缺）。
**差异/删除**：删 `RectangleEditUiState` 80B 猜布局；新建 `WipRectangle`（按上表偏移）；补 `OnRectangleEdited`(0x58)/`DecoratorArchivist::extract_and_restore_to_storage`/`CompoundEditBuilder+ConcurrentEdits`/两 EventInstance。
**测试**：tag 0 早退；活动态先发 0x58；双判据（朝向/维度各自 0.01 边界 + NaN）；无变化→restore；超阈→History+两事件；`InnerWalls` variant 断言。
**STATUS: READY**（控制流/判据偏移/0x58 载荷/9 参读写者均逐指令实读；字段名合成，偏移为权威）

---

## 附录 A：证据补齐优先级（实现 agent 参考）
1. 先做 §1.3 + §1.2 + §13（READY）打通高度编辑与矩形应用历史。
2. 再补 §10/§11（camera 投影 + 吸附 + `Slider3d`）→ §12 → §14 → §13 事件扩展。
3. 圆组 §6→§7→§8→§9（依赖 `AnimatedCircleSignifierResource` 全布局）。
4. move/freehand §2→§3→§4→§5（依赖 `Move*Cache` 布局与 `move_gizmo`）。
5. 每次实现前用 `python scripts/rne-read.py --hex <VA> <n>` 复核本文常量；参数以 PDB 表为准。
