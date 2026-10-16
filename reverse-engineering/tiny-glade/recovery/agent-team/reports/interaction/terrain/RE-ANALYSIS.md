# RE-ANALYSIS: terrain（交互与世界组 · terrain）

> 会话：interaction re-analyst（2026-10-15）。范围：`resources/terrain_editing|terrain_stroke|
> history/terrain_edit|terrain_height|cursor_terrain_raycast|mouse_terrain_raycast|rocky_terrain`
> + `systems/history/replay_terrain` + 中央转发要求补挖的 `systems/terrain/` 5 文件
> （deferred_terrain_history / terrain_hover_idx / ui_draw_terrain / ui_edit_terrain /
> ui_terrain_focus_mode）——工具状态机/掩码/脏格证据 + 输入源(AppMode/ToolUI)接线。
> 依据：B EXE 机器码 + 既有 decomp + 本轮新反编译（interaction-group/decomp/，
> callees 批 results/out-callees.json）。记号：O/I/U；「旧结论 X → 新证据 Y」=修正。
> 不改 reconstruction 源码。

## 1. 函数清单表

| 函数 | 地址 | 状态 | 行为一句话 |
|---|---|---|---|
| TerrainEditingData::new_stroke | 0x140924a10 | O | id=counter+1（上限 0x3fffff），建单点 TerrainStroke{spacing 0.4, scale 2.5} 插入 HashMap |
| TerrainEditingData::get_last_mut_stroke | 0x140924900 | O | counter==0 panic(@0x142aae908)；查 counter−1 返回 &mut/None（hashbrown SIMD） |
| TerrainStroke::insert_or_morph_last_point | 0x140a97880 | O | len0 push / len1 dist>0.001 push / len≥2 spacing≤dist push<1024 否则 morph |
| TerrainEdit::revert / reapply | 0x140858d30 / 0x1408592d0 | O | 0.99 进度阈值 panic + hashbrown 48B stroke 表清/置 active 标志 |
| TerrainEdit::update_terrain_curve | 0x140859740 | O | hash 查表 + 点集拷贝 + fold + 全局时间 (1-t)u+t lerp |
| TerrainReplayCmd::forward / backward | 0x140994f30 / 0x140995040 | O | duration=clamp(Σ点距*0.01,0.4,3.0)（空 0.3），stroke_length 有限断言 |
| replay_terrain | 0x140995150 | O | **ease_out 闭合 = 1−(1−t)³（cubic）**；instant/动画两路径 |
| CursorTerrainRaycast::position | 0x140a12650 | O（本轮 37B） | tag&1 → Vec3（pos@4{x,y}+z@0xc）否则 {0,0,0} |
| CameraCursorRay::intersect_with_xz_plane | 0x1408ef180 | O（本轮 57B） | t=(plane_y−pos.y)/dir.y，pos+t·dir |
| camera_mouse_ray | 0x1408ef1c0 | O | CursorPositionSS → Ray::from_mouse_ss |
| mouse_terrain_raycast | 0x1408ef320 | O | 平面快路径 / TerrainHeightsData::intersect_ray + y=0 平面比距取近；写 {tag,pos,delta,length} 0x20B |
| RockyTerrainMesh::load | 0x1408bbf50 | O | read_to_string → serde_json → 顶点 0x24B→0x30B / 三角 0x20B→0x20B → add_new_with_data |
| terrain_height 5 函数 | 0x1409e9640 等 | O | 权威采样/视觉双线性 shim/intersect_ray/copy_from_gpu/load_index_unchecked |
| history_record_terrain | 0x140ac9770 | O | PendingTerrainHistory 置脏 0x8000000000000002，首次构建历史 History::add(tag 0xd) |
| terrain_hover_idx | 0x140aff720 | O | AppMode<0xb ∧ 0x421 掩码(bits{0,5,10}) → SSBO 读 id@+0x10 → 三重门 → HoveredTerrainStrokeId |
| ui_draw_terrain | 0x140a9fdc0 | O | AppMode==10 门 → 点按新建笔画（new_stroke + 事件） |
| ui_edit_terrain | 0x140aef1f0 | O | AppMode==10 门 → 复杂状态机（每 hovered stroke 分派 6 资源写） |
| ui_terrain_focus_mode | 0x1409a4fb0 | O(部分) | 聚焦模式状态机（raycast_simple + 距离门 + 大量内联） |

## 2. 每函数行为级结论

### 2.1 TerrainEditingData（resources/terrain_editing.rs，O）

布局（既有 + 本轮复核）：`TerrainEditingData { HashMap<TerrainStrokeId,TerrainStroke> @+8
（hashbrown：数据@+8、掩码@+0x10、容量@+0x20、表@+0x28、条目 0x30B），next_stroke_id u32@+0x38 }`。

- **new_stroke @0x140924a10**：`0x3ffffe < next_stroke_id` → 返回 None(0)。否则 id=next+1 写回；
  alloc 0xc(12B) 首点缓冲；值 = `{Vec<Vec3> cap-first{cap=1,ptr,len=1}, +0x18 f32=0x3ecccccd(0.4f),
  +0x1c f32=0x40200000(2.5f)}`（= **TerrainStroke 布局 {points Vec 24B@0, spacing f32@0x18,
  scale f32@0x1c}，0x20B**）；`FUN_141526d10` 插入 HashMap(key=旧 counter)；插入成功→1(Some)；
  失败 → 格式化 panic（&UNK_142aae968，0x142aae978 区）。
- **get_last_mut_stroke @0x140924900**：next==0 → panic(&UNK_142aae908)（空表取末）；
  否则 hashbrown 查 key=next−1（FUN_1405b0ed0 + SIMD 16B probe，命中 i32 相等），
  返回条目值&mut（0x30B 条目内 +8 起）或 None(0)。
- serde：TerrainEditingData serde_json::Value 序列化 key = **"strokes"/"next_stroke_id"**
  （0x1408e6b70，既有纠偏维持）。

### 2.2 TerrainStroke（resources/terrain_stroke.rs，O）

- **insert_or_morph_last_point @0x140a97880**（cap-first Vec{cap@0,ptr@8,len@0x10}）：
  len==0→push；len==1 且 dist>0.001（0x3a83126f）→push；len≥2 且 dist(last,prev)≥spacing→
  push(<1024 上限) 否则覆盖 last（morph）。0.001 常量机器码实证。
- new（行 23，内联）构造空笔画。

### 2.3 TerrainEdit（resources/history/terrain_edit.rs，O，history 域已深挖）

- 判别：首字段 niche `x^0x8000000000000000 < 2` → 变体 0/1/2（NewStroke/Edit/Move）。
- revert 0x140858d30 / reapply 0x1408592d0：判别 0 且 param_6<0.99f（0x142a948d8）→ panic
  "Requested terrain stroke {id}"（terrain_edit.rs，@0x142a94cf8）；否则 hashbrown 查 48B/元素
  stroke 表（FUN_1405b0ed0）清/置 active 标志（+0x28）。
- duration（内联进 TerrainReplayCmd::forward）：`clamp(Σ相邻点距*0.01, 0.4, 3.0)`，空 0.3；
  "assertion failed: stroke_length.is_finite()"（terrain_edit.rs:43，@0x142a94c34）。
- update_terrain_curve 0x140859740：hash 查 48B stroke 表 → 拷 12B 点集 → fold 求进度 →
  与全局时间 FUN_140c95ef0(&0x142a94d48) 做 `(1-t)*u + t` lerp → 0..1 范围断言 →
  回写 stroke 记录。

### 2.4 replay_terrain @0x140995150（O，本轮 ease_out 闭合）

参数面：param_1=Time、param_2=TerrainEditingData(ResMut，HashMap@+8，next_stroke_id@+0x38)、
param_3=ReplayBuffer(ResMut，speed@+0x28、active@+0x18、前端桶 ptr/len/idx)、param_4/5/6=透传。
- 前置：`ReplayBuffer+0x18(active)==0 → return`；speed=+0x28；dt=Time+0x28。
- 前桶 0x30 步长迭代，仅 tag 3；载荷 = TerrainEdit（niche @+0x10==MIN → 变体字段 @+0x18）。
- **instant 路径**：duration@cmd+0x10==0.0 且非 NaN 且 undo@cmd+0x1c==0：拷点集(12B/点，
  断言 `点数<0x3fffff` panic@0x142aae990 行 0x40)；更新 `next_stroke_id = max(旧, id+1)`；
  `FUN_141526d10` 插入 TerrainEditingData；构造默认命令 {id, 0.4f(0x3ecccccd), 2, stroke_id}。
- **动画路径**：`t = (elapsed>=0 && duration>=0.4) ? clamp(elapsed/duration,0,1) : 1.0`；
  **`eased = 1−(1−t)³`**（`fVar27=1−t; fVar32=1−fVar27*fVar27*fVar27`；0.4 下限 0x1429cd060、
  1.0 上限 0x142925904）；undo==0 → reapply 0x1408592d0，否则 revert 0x140858d30；
  eased≥1 → cmd+0x18 计数−−，<0 入完成索引表；`elapsed += dt*speed`。
- 收尾：完成索引表逆序 memmove 压缩 + 按判别字节 tag 0..9/default 分派 Arc drop
  （LOCK;dec；归零后各自释放，tag 3 → FUN_140b5c2a0）。

**旧结论 X → 新证据 Y**：ease_out（行 168，「系数歧义 U」）→ **闭合 = 1−(1−t)³ cubic**
（reconstruction 现用式一致 ✓）。

### 2.5 cursor_terrain_raycast（O）

- position @0x140a12650（37B）：`tag&1 → *out = {pos.x,pos.y}(8B)@+0 + z@+0xc`，否则 `{0,0,0}`。
  布局 `CursorTerrainRaycast {tag u8@0, pos Vec3@4}`（Option<Vec3>）。
- update/new（内联）：Some→tag=1+pos；None→tag=0。

### 2.6 mouse_terrain_raycast（O，本轮布局纠偏）

- **CameraCursorRay 布局纠偏**：`{pos Vec3@0, dir Vec3@0xc, length f32@0x18?}`（机器码：
  pos@0 读 {x,y}、dir@0xc 读 {x,y}、dir.z@0x14、pos.z@0x8）。
  **旧结论 X → 新证据 Y**：章节「{tag u32@0, pos@4, dir@0x10, length@0x1c}」→ 机器码
  pos@0/dir@0xc（intersect_with_xz_plane 0x1408ef180 + mouse_terrain_raycast 0x1408ef320 互证）。
- intersect_with_xz_plane @0x1408ef180（57B）：`t=(plane_y−pos.y)/dir.y; out=pos+t·dir`。
- mouse_terrain_raycast @0x1408ef320：快路径（平面 y=param_4[1]，param_4 首 i32==1 门）→
  xz 平面解；慢路径 → `TerrainHeightsData::intersect_ray`(0x1409e9a20) + y=0 平面交点
  （0.0−pos.y)/dir.y）比距取近（sqrtps 选 min）；**输出资源 = {tag i32@0, pos Vec3@4,
  delta Vec3@0x10, length i32@0x1c} 0x20B**（length = dist/除数，除数=param_7+0x58 f32）；
  精化闭包 FUN_1408efa70（closure$0）经 puffin scope（are_scopes_on 0x140c16a00 门）。
- camera_mouse_ray @0x1408ef1c0：CursorPositionSS → utils::geometry::ray::from_mouse_ss。

### 2.7 RockyTerrainMesh::load @0x1408bbf50（O）

- `fs::read_to_string`(FUN_140d59140) → serde_json(from_str 140141405b00) → 数组。
- **顶点**：源 0x24B(36B) → 目标 0x30B(48B)（拷贝循环读源 +0x0/+0x4/+0x8/+0xc/+0x14/+0x1c，
  dest 步距 0x30）→ `add_new_with_data<VertexSSBO>`（FUN_1412bc1f0 前置）。
- **三角形**：源 0x20B → 目标 0x20B（读源 +0x0..0x1c，dest 步距 0x20）→
  `add_new_with_data<TriangleSSBO>`（FUN_1412bba70）。
- **旧结论 X → 新证据 Y**：章节「三角形 0x10B→0x20B」→ 机器码源记录 0x20B→0x20B
  （读取序 0x0..0x1c 跨 32B）。
- 失败路径：读文件失败/解析失败均 return 1（Err）；尾部逐级释放临时。

### 2.8 terrain_height（O，既有深挖维持）

- 权威 `sample_at_world_xz<TerrainHeightSsbo>` 0x1409e9640（+0x201 偏移，*0x14362c308==0 正常分支）。
- **视觉 `sample_at_world_xz<VisualTerrainHeightSsbo>` 0x140257960（52B shim）→ 尾调
  `utils::float_grid::bilinear_sample_at_world_xz<TerrainHeightFloatGrid>` 0x140257ba0**。
- **`bilinear_sample_at_world_xz<RasterBufferRef<512>>` 0x140a14df0（835B，本轮身份定案）**：
  `u = world/scale + 0.5`（scale=130.0，0x43020000）→ `USE_FLOAT_GRID_SAMPLING_FIX`
  全局门控 maxps(0,·)/minps(1,·) clamp → 3×lerp（0x140c904d0）。**这是 ui_draw_terrain /
  terrain_hover_idx 的高度采样 helper。**
- intersect_ray 0x1409e9a20 / copy_from_gpu 0x1409ea020 / load_index_unchecked 0x1408b54c0
  （51B，Ghidra 只见 return——为内联装载，行为见既有章节）维持 O。

### 2.9 history_record_terrain（systems/terrain/deferred_terrain_history.rs，O 本轮）

@0x140ac9770（314B）：`PendingTerrainHistory {flag qword@0, payload 4 qword@8..0x28}`。
```
flag 非 0x8000000000000002 时（首变化）:
  读 payload 4 qword → 组 0x38B 记录（tag=1@0, 4×u32 载荷@8, …）
  History::add 0x140a84ec0（tag 0xd 事件记录）
  push 到 param_3 事件 Vec（cap@+0x20,ptr@+0x28,len@+0x30,+0x40 计数++）
置 flag = 0x8000000000000002（脏哨兵，每次帧首执行）
```
- **脏格证据**：哨兵 `0x8000000000000002`（= −0x7ffffffffffffffe）即「首次变化待记录」；
  之后不重复记录（幂等）。
- callees：History::add(0x140a84ec0)、`grow_one<EventInstance<WriteSettingsFileCmd>>`(140562870)。

### 2.10 terrain_hover_idx @0x140aff720（O 本轮）

参数面：param_1=Res(模式，mode=*(u32*)*param_1)、param_2=资源组（[0]=SsboLibrary、
[4]=世界栅格资源(+0x298 flag 选双原点)、[8]=CursorTerrainRaycast{pbVar4}）、
param_3=ResMut<HoveredTerrainStrokeId>（tick 回写，data=*param_3）。
```
*param_3 = 0（tag=None 初始化）
if (mode < 0xb) && ((0x421 >> (mode&0x1f)) & 1):      # 掩码 0x421 = bits{0,5,10}（0x1|0x20|0x400）
  puVar8 = SsboLibrary::get<SsboId,Box<BufferState>> 0x140969720(&UNK_142c2e408)
           # 名校验：puVar8[3]==0x6c(108) 且 memcmp(&UNK_142a9df5b,puVar8[2],0x6c)==0
           # = MouseTerrainRaycastSsbo 名（108 字符）；不匹配 → assert_failed panic(1428cab41)
  plVar9 = (vtable+0x18)(*puVar8)                      # 取 SSBO 缓冲
  if plVar9 != 0: id = *(i32*)(*plVar9 + 0x10); else id = -1
  fVar12 = bilinear_sample_at_world_xz<RasterBufferRef<512>> 0x140a14df0(
             &原点2qword, cursor.x, cursor.z, _UNK_142a7b658, 130.0)
  # 三重门（机器码）：
  if fVar12 <= 0.0:   if id == -1: return            # 高度≤0（水下/界外）：仅需 id 有效
  else:               if !(cursor.tag&1): return      # 高度>0：需光标命中
                      if id == -1: return
                      if cursor.pos.y <= 0.5(_UNK_142925870): return   # 命中高度≤0.5 不悬停
  *param_3 = 1; param_3[1] = id                        # HoveredTerrainStrokeId{tag=1, id}
```
- **旧结论 X → 新证据 Y**：章节「水位采样≤0/光标无效/距离≤0.5 不更新」——
  机器码为：高度≤0 时仅当 id==-1 才不更新（高度≤0 且 id 有效 **会写**）；
  高度>0 时要求 光标命中 ∧ id 有效 ∧ pos.y>0.5。门控条件组合与旧述不同。
- callees：SsboLibrary::get(0x140969720)、bilinear_sample(0x140a14df0)、memcmp、
  assert_failed、panic_bounds_check。

### 2.11 ui_draw_terrain @0x140a9fdc0（O 本轮，地址/行为复核通过）

- **门：`*(i32*)*param_1 == 10`**（AppMode==10 才运行）。
- param_4=地形网格资源（+0x298 字节选原点 {+8/+0x10} 或 {+0x20/+0x28}，CENTERED_PATH 关联候选）；
  param_2=CursorTerrainRaycast{pbVar5}。
- 高度门 `fVar16 = bilinear_sample_at_world_xz(原点, cursor.x, cursor.z, …, 130.0)`：
  `fVar16<=0 → 走空路径`；`>0` 且光标命中且 `pos.y > 0.5` → 走「开始笔画」路径；
  否则推 `clearing_canvas_w_layer_and_sound`（0x14092df50，tag 0x11 事件）。
- 开始笔画：推 tag 0xc 事件（0x14092df50）+ `new Vec2/Vec3`（0x140c97540，0x1cB 构造）；
  `is_shape_inside`（0x140ad2970）检查后 push 到笔画 Vec（8B 元素，grow 140562870）；
  末尾 jumptable（&UNK_142adc498）分派后续模式（点按/拖拽）。
- callees：bilinear_sample_at_world_xz、clearing_canvas_w_layer_and_sound、is_shape_inside、
  new、grow_one<WriteSettingsFileCmd>。

### 2.12 ui_edit_terrain @0x140aef1f0（O 本轮，工具状态机/掩码）

参数面（15 参）：param_1=Res(模式门==10)、param_2=点击状态资源（`*pcVar23==1`=按下）、
param_3=StrokeParamsCache（lVar64，元素 16B×N，表@+8、掩码@+0x10、容量@+0x20）、
param_4=哈希表资源、param_5/6/7/8/9/10/11/12=多个事件/记录 Vec（ResMut，tick 回写）、
param_14=Events 双缓冲（+ 光标 @[5]）、param_15=HoveredTerrainStrokeId（{tag@0,id@8}，-1 哨兵）。

```
if mode != 10: return
事件排空 + 光标读取（last event {fVar79,fVar80} Vec2）
iVar39 = HoveredTerrainStrokeId.tag? id@+8 : -1
if (iVar39 != -1) && (click字节 == 1): push 16B {qword, byte=1} 到 param_12 Vec（grow 140562630）
外层 do{ 遍历 StrokeParamsCache 16B 元素（SIMD 位图取有效项）
  per 项: 闭包环境构造（FUN_140d79f90 format_inner）
    disc = (click按下)? 7 : ((iVar39 != 该项id)? 7 : 4)
    闭包载荷含 {0x4302000043020000 (130×2), 0x100(256), 事件有无 bool}
  lVar41 = add<String>（0x1409231f0，HashMap 增/取）→ 读结果 tag@+0x18
  switch(tag)（case 0..4/6/8/0xc/0xf，跳表 UNK_142ae404c）:
    case 1: 推 16B 事件{byte=0} 到 param_9 Vec；写 param_5 {disc,val} 数据；
            写 param_7 +0x1e0=1、f32=0.0
    case 2: 推 8B 事件到 param_10 Vec；组 0x28B 记录 {tag=2@0, f32@4, Vec2@0xc, …}
            （hashbrown 查已存在笔画 0x1405b0ed0 + SIMD probe；dist 计算 + clamp[0,1] 1.0）
    case 3: 推事件到 param_9/param_12；组 0x28B 记录{tag=0@0,+8=2,f32@0xc,Vec2@0x10,…}
            写 param_5 载荷；`*plStack_118 = -0x7fffffffffffffff`(i64::MIN-1 哨兵) 组历史记录；
            状态异常 → panic（&UNK_142ae3acf, 行 0x172=370, &UNK_142ae3c48）
    case 4: 跳表间接调用
    default/6/8/0xc/0xf: 写跳表数据（UNK_142ae404d[idx]=N）状态收口
} while(有更多项)
```
- **掩码/脏格**：StrokeParamsCache 16B 元素按 16 位位图遍历（`(elem>>7k)&1` 取 16 位有效位），
  只处理位图标记的有效项——=「脏笔画」掩码。
- callees：add<String>(0x1409231f0)、format_inner(0x140d79f90)、
  `grow_one<EventInstance<WindowFocusEvent>>`(0x140562630)。
- 旧结论维持：`ui_edit_terrain`（0xaef1f0）为地形编辑主状态机（EditTerrainStrokeCmd 族 + 光标）。

### 2.13 ui_terrain_focus_mode @0x1409a4fb0（O(部分)，167KB 近全内联）

- **门：`*(u32)*param_1 == 1`**（聚焦模式激活标志字节==1）。
- param_2=ResMut（HashMap：data@+8、掩码@+0x10、容量@+0x20；`lVar43+0x20==0` 空 → 走
  raycast 精化路径）。
- **raycast_simple**（0x1408ce180）命中类型 `tag==0x11` 分叉；未命中 → 3D 距离 SQRT
  （hit 与参比点）；`pfStack_350/348` 读目标 Vec3。
- 距离门 + `_UNK_142925870(0.5)` 比较 → 推 16B 事件到 lStack_340 Vec
  （grow 140562630 WindowFocusEvent 族）。
- 旋转/矩阵数学：quat 旋转常量族 `_UNK_1429cd170..19c`、`_UNK_142a81cd0..ec`、
  `_UNK_1429258e0/e4`、`_UNK_142925870(0.5)`、`_UNK_142925904(1.0)`、`_UNK_1429cd064(0.4?)`、
  `0x3f800000(1.0)`、`0x40000000(2.0)`。
- 闭包 4 个（既有）：0x9a3620（as_curve3 0x140c97e10 曲线采样，0.5 参数）、0x9a8640/0x9a88c0/
  0x9a8b40（状态转换）。
- callees（本轮批）：as_curve3(0x140c97e10)、hash_one(0x1405b0ed0)、raycast_simple
  (0x1408ce180)、new(0x140c97540)、x0y(0x140c9cdc0)/xvy(0x140c9cde0)（HashMap helper）、
  `push<UiAppendSignifier>`(0x14146c880)、add<String>(0x1409231f0)、memcpy、
  format_inner、panic_bounds_check(0x1428d9518)。
- 内部状态名维持 U（ui-input 域 owns）。

## 3. AppMode/ToolUI 接线（中央转发要点 → 裁决见 RE-APP-MODE.md）

| 系统 | 机器码门 | 说明 |
|---|---|---|
| ui_draw_terrain / ui_edit_terrain | AppMode disc == 10 | 地形笔画新建/编辑仅在 disc 10 运行 |
| terrain_hover_idx | mode<0xb 且 (0x421>>mode)&1 | bits {0,5,10}（0=All、5=Garden、10=Terrain 工具） |
| history_record_terrain | 无模式门 | PendingTerrainHistory 脏哨兵驱动 |
| ui_terrain_focus_mode | 首资源 u32 == 1 | 聚焦模式激活标志（非 AppMode） |

**★ 裁决（本轮，详见 RE-APP-MODE.md）：AppMode disc 10 = Terrain 工具（地形绘制模式）**——
三个地形系统同一门值 + `AppMode::is_tool_selected`（0x140a86b40，disc∈6..18 工具区）含 10，
行为仲裁无歧义。ui-input §3.3「10+5k=Tool(视觉序 Buildings…/Terrain)，10=Buildings」**作废**；
DIGEST §2.2 env↔类别映射（env_10=cat6 Buildings、env_14=cat10 Water）与之并不自洽。
`0x421 = bits{0,5,10}`（本轮算术复核修正早先 {0,5,9} 之误）。
AppMode 结构：16B {disc u32, payload u64, extra u32}；default=0(All)；0xd=Replay、2=Timelapse。

## 4. 未决（U，不臆造）

1. ~~AppMode==10 语义名~~ **已裁决**（RE-APP-MODE.md）：10 = Terrain 工具；精确 disc(7..18)→变体名
   仍 U（serialize jumptable 融合池歧义，行为裁决为准）。
2. ui_draw_terrain 的 param_4（+0x298 选原点资源）身份与 CENTERED_PATH(0x14362c308) 关联。
3. ui_edit_terrain 各 Vec 的事件类型（16B/8B/0x28B 记录）与 EditTerrainStrokeCmd 字段精确语义
   （0x28B {tag0/2, f32, Vec2, u32, qword} 的部分字段未定名）。
4. ui_terrain_focus_mode 167KB 内联体：进入/退出转移的精确状态表（仅 callees+常量层）。
5. terrain_hover_idx 的 param_2[0]（SsboLibrary）与 CursorTerrainRaycast 读法已 O，
   但 param_2 资源组精确身份（世界栅格/原点）待 toolbar 接线交叉。
6. CameraCursorRay 的 length 字段偏移（+0x18 vs 旧 +0x1c）——本报告按机器码 pos@0/dir@0xc
   推定 +0x18，消费端写长度时核对。

## 5. 证据落盘（本轮新增）

- `evidence/country-core-deep/interaction-group/decomp/decomp-0x140a14df0.txt`（bilinear sample）
- `decomp-0x140969720.txt`（SsboLibrary::get）、`decomp-0x140942b00.txt`（reset，input 域）
- `decomp-0x140a12650.txt`（position 37B）、`decomp-0x1408ef180.txt`（intersect 57B）
- `decomp-0x1408b54c0.txt`（load_index 51B）、`decomp-0x140257960.txt`（visual shim 52B）
- `decomp-0x14003d300.txt`/`decomp-0x140a18500.txt`（auto_clutter_edit serde）
- `decomp-0x140a86b60.txt`（mode_manager 入口）+ results/out-callees.json / out-missing-decomp.json
