# RE-ANALYSIS2: terrain（二轮补挖 · terrain）

> 会话：interaction re-analyst（2026-10-15，二轮）。reviewer 第 3 项：ui_terrain_focus_mode 主体；
> 第 4 项（ui_draw 消费面）见 §3。依据：B EXE 机器码 + 既有 decomp（ui-input/decomp/
> decomp-0x1409a4fb0.txt 171KB 全量 + terrain-water/decomp/decomp-0x140a9fdc0.txt）+ RNE 常量。
> 记号：O/I/U；「旧结论 X → 新证据 Y」=修正。**167KB 内联体受 Ghidra 坏指令/跳表恢复限制，
> 状态转移表按可核实事实给出 + 卡点如实标注。**

## 1. ui_terrain_focus_mode @0x1409a4fb0（O(部分)）

### 1.1 门与参数面（O）

- **门：`*(u32)*param_1 == 1`**（param_1 = 聚焦模式激活标志资源，bit0==1 才进入主体）。
- param_2 = ResMut<HashMap>（数据@+8、掩码@+0x10、容量@+0x20；`lVar43+0x20==0` 空表 → 走
  raycast 精化路径）；param_3..param_13 = 资源组（光标/鼠标、PublicWalls、OutlineWallCmd 等）。
- 参数块 param_10（读取 +0..0x7c：光标/鼠标状态）、param_13（+0..0x70）。

### 1.2 行为级结论（可落实现，O）

1. **raycast_simple 主路径**：`FUN_1408ce180(&uStack_1d8, ray, pfVar48, 0)`；
   命中类型 `(char)uStack_1d8 == 0x11` 分叉（0x11 = 特定 hit 类型，如墙/笔画命中）；
   未命中 → 3D 距离 SQRT（hit 与参比点 pfVar48 之差）。
2. **距离门（0.5）**：`SQRT((fVar54-pfVar48[2])² + (fVar83-pfVar48[1])² + (fVar75-*pfVar48)²) < fVar52`，
   fVar52 = _UNK_142925980 = **f32MAX（3.40282e38）**（未命中初始距离）或命中分支置值；
   且 `*pfStack_348 == 1.4013e-45`（bit 0x1 标志）→ 推 16B 事件到 Vec（grow 0x140562630
   WindowFocusEvent 族）：`{qword@0, qword@8}` 含 frame_index/目标。
3. **HashMap 交互**：`hash_one`(0x1405b0ed0) + SIMD probe 查 lVar43 HashMap（strokes/焦点表，
   表@+0x28、数据@+0x8、掩码@+0x10）；`xvy`(0x140c9cde0) 写回。
4. **as_curve3 曲线采样**：`FUN_140c97e10`（0.5 参数）采样曲线点 → 焦点/轮廓定位。
5. **事件输出**：`push<UiAppendSignifier>`(0x14146c880) 推 signifier（箭头/高亮 UI 件）；
   `add<String>`(0x1409231f0) 写 HashMap。
6. **状态机分派**：主体有 0..0x3c+ 大 switch（跳表恢复不完整，Ghidra UNRECOVERED_JUMPTABLE），
   命中类型/模式 → 进入/退出转移（进入=置聚焦态、退出=恢复模式）。**转移精确表未从该
   Ghidra 输出干净恢复（卡点）。**
7. **Replay 交互**：`UNRECOVERED_JUMPTABLE = 0x8000000000000002`（= MIN+2 Replay 哨兵）写入
   状态 —— 聚焦模式与回放模式联动（进入聚焦暂停/退出恢复，I）。
8. **panic**：`auVar55._0_8_ == -0x7ffffffffffffffe` 异常 → panic（&UNK_142abd7e8，行 0xa8 区）。

### 1.3 常量表（O，RNE 直读）

| 常量 | 值 | 用途 |
|---|---|---|
| _UNK_142925870 | 0.5 | 距离/旋转 0.5 阈值 |
| _UNK_142925904 | 1.0 | clamp 上限 |
| _UNK_142925980 | 3.40282e38 (f32MAX) | 未命中初始距离 |
| _UNK_142960674 | 0.01 | 聚焦判定阈值（FUN_1416f3260 后比较） |
| _UNK_142a16d10 | -10.0 | 聚焦推进速度 |
| _UNK_1429cd128 | 1.2 | 聚焦常量 |
| _UNK_142a7cd48 | 7.0 | lerp 系数 |
| _UNK_1429cd064 | 2.5 | 缩放 |
| _UNK_1429258e0/e4 | 0.5 | 归一化 scale（与 input 回放共用） |
| _UNK_1429cd170..19c | ±1.0 | 旋转矩阵（反射/轴翻转） |
| CircularF32::as_f32 0x1416f3210 | fmod 2π | 旋转角回绕 |

### 1.4 闭包 4 个（既有 O 地址维持）

0x9a3620（as_curve3 0x140c97e10 曲线采样，0.5 参数）/ 0x9a8640 / 0x9a88c0 / 0x9a8b40
（状态转换，OutlineWallCmd/PublicWalls 相关）。

### 1.5 卡点（如实标注）

167KB 内联体 Ghidra 输出含坏指令数据（0x1409ad542 重叠、多段 unreachable block、
UNRECOVERED_JUMPTABLE）→ **进入/退出状态转移的精确 switch 表未恢复**（U）；已给行为级
事实（raycast tag 0x11、距离门、HashMap、as_curve3、Replay 哨兵、常量表）供 restorer 落地
功能骨架，转移细节建议 High P-code/逐段 capstone 手解。

## 2. 旧结论修正

- 章节「聚焦模式状态机：alloc/memcpy 为主」→ 主体实为 **raycast_simple 驱动 + 0.5 距离门 +
  焦点 HashMap + Replay 联动**（§1.2）。
- **_UNK_142925980 语义**：f32MAX（未命中哨兵），非此前隐含的 0.5。

## 3. ui_draw_terrain 消费面（item 4 前半，O）

生产面（新建笔画/事件）既有 ✓；**消费面（读入）**：
- 参数面（wp8 合同 + 本轮机器码）：`Res<AppMode>（门==10）, Res<CursorTerrainRaycast>（pbVar5,
  {tag@0,pos@4,dist@8}）, Res<世界栅格>（param_4，+0x298 flag 选双原点 {+8/+0x10} 或
  {+0x20/+0x28}）, ResMut<PendingTerrainHistory>（in_stack_0x60[6]）, Local<DrawState>（Vec
  @+0x20..0x40，16B 元素 {qword, byte}）, Local<CacheMouseRaycastHeight>, 事件组（param_5/6/7,
  0x10B 元素）`。
- **读入链**：`bilinear_sample_at_world_xz<RasterBufferRef<512>>`(0x140a14df0, scale 130) 采样
  光标处地形高度 → 门控（≤0 空路径 / >0 且光标命中且 dist>0.5 → 开始笔画）→
  `is_shape_inside`(0x140ad2970) 判定界内 → push 到 DrawState 16B 记录 →
  推 `clearing_canvas_w_layer_and_sound`(0x14092df50, tag 0x11) 事件 → 末尾 jumptable 分派
  （点按/拖拽后续）。
- **消费的游标/高度缓存**：`CacheMouseRaycastHeight`（Local）缓存上次 raycast 高度避免重采样
  （I，from wp8 Local 名单）。
- 生产（发事件）：EvTerrainStrokeChanged（new_stroke 后）、NotifyHintSystemTerrain、
  NotifyHintSystemToolWasJustUsed、DisplayOutOfBorder、FeedbackInputOutOfBorder、PlayTerrainSound。
- **旧结论维持**：消费面与 wp8 合同一致；DrawState Local = 16B 记录 Vec（机器码 push {qword,byte}）。

## 4. 二轮证据落盘

- 常量表（§1.3）RNE 直读；CircularF32::as_f32 定案（0x1416f3210，66B）
- 既有 decomp 复核：ui-input/decomp/decomp-0x1409a4fb0.txt（3908 行）、
  terrain-water/decomp/decomp-0x140a9fdc0.txt
