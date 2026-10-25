# clutter 剩余实现 RESTORE（restorer-clutter-remaining · 2026-10-17）

范围：`crates/systems/clutter`（package `system-clutter`）剩余未实现函数。
硬标准：每函数 fresh 反编译（ghidra `--worker shared` 只读）→ 重建控制流 → 与 Rust 逐语句判等；
无法逐语句还原者如实登记 U，不造近似体冒充判等。

## 证据

- Ghidra fresh 反编译（本会话）：`work/restorer-clutter-remaining/evidence/decomp/`
  - 0x140eebaa0 / 0x1422ccef0 / 0x1422cc390 / 0x1422cefe0 / 0x1422d2fa0 / 0x141ce5900 /
    0x1422d3800 / 0x1422d4230 / 0x1422d04e0 / 0x141ce3a70 / 0x1422d4870 / 0x141cd1a90 /
    0x141cd3ce0 / 0x141ce65b0 / 0x141ce7190 / 0x1422d6930。
- 逐指令反汇编：`evidence/disasm0.txt`、`disasm1.txt`、`disasm3~6_*.txt`。
- 常量/字符串/符号实读：`evidence/consts*-result.json`、`names*-result.json`、`strs-result.json`
  （如 Aabb2 半径 0.7 @0x1429cd030、BushCpuSsbo 名/ID @0x142f3b328/0x142f3b268、
  lantern 0.65/-0.05/-0.8/-0.1/1.3/1.2/0.1/0.25/5.0/0.8、
  tree 0.9/0.4/12、merlon 0.46/0.23/0.115/0.32/0.07/0.28/0.21/0.8/0.368、
  stitch 0.2/1000/0.5/0.1/0.4/-0.1/0.3、window 名 decorator_right/left/trap_door）。
- 结果：`cargo check -p system-clutter` **通过**（exit 0）；`cargo test -p system-clutter`
  **4/4 通过**（numeric_assertions）。

## 三态计数（本批 19 项）

| 状态 | 数 | 项 |
|---|---|---|
| 判等（核心逐语句；边界 U 单列） | 3 | read_bush_positions_from_gpu 核心 6 步、autoclutter_subscribe_to_tabloids 13 订阅顺序、remove_region_from_lazy_grid 键删除 |
| 重建（控制流/常量/门控判等，几何或宿主类型面登记 I/U） | 14 | manage_autoclutter_entities、autoclutter_animation、add_autoclutter_around_gates、add_autoclutter_around_windows、add_lantern_water_setdresssing、add_preplaced_autoclutter、add_tree_setdressing、construct_inter_roof_merlons、detect_inter_roof_merlons、request_inter_roof_merlon_audio、detect_intra_shape_corners、add_hole_at_shape_intersection、spawn_stitches、spawn_stitch_bricks |
| 未实现（无独立符号/内联不可定点） | 2 | set_current_tick、reset_autoclutter_for_new_frame |
| 附带 | — | add_region_to_lazy_grid/remove 由占位 0 改为区域 GlobalId 键（O：闭包写键） |

## 逐函数要点

### 1. read_bush_positions_from_gpu（0x140eebaa0）— 判等（核心）
- `FrozenMap::get<SsboId=0x0deff5ab6a35972a>` → 类型名断言 0x5c
  （`...read_bush_positions_for_cpu::BushCpuSsbo`）→ 数据 `{ptr,len}`；`len==0` panic、
  `count=*(u32*)data`、`len<=count` slice_end_index_len_fail。
- 清每格 positions.len（记录 stride 0x20）；每 bush `Aabb2::from_center_radius(pos, 0.7)`
  → 每覆盖格 `idx=x*64+y`（0x140aac630）追加 **bush 位置**。
- 每格 len>1 → `sort_unstable`（<0x15 insertion/否则 ipnsort）；hash（SipHash-1-3 零键：
  len→每点 2+2×f32 位型）；hash 变 → 置位 **idx**（0x140833250 恒等）→ publish DirtyTiles。
- U：GPU 回读/FrozenMap 设备面由 `BushCpuSsbo` 资源承载；sort 比较器为 I（同一确定性全序）。
- Rust：`read_bush_positions_from_gpu.rs`（全部重写）。

### 2. add_lantern_water_setdresssing（0x1422d2fa0）— 重建
- 修：圆半径 **0.65**、内层 **5**、u=`rem_euclid(f32*0.1+i/5)`、两点抖动 ×**0.25**、
  采样 `0.0 < sample` 门、键 tuple<u32,usize,&str>(i,id,"lantern")；
  偏移/缩放三路：started → `i/5×-0.05+base, 0.8`；非 started 且 **键偶** → `i/5×-0.1+base, 1.3`；
  否则 `-0.8, 0.8`；base 每轮更新；scale=`factor×(r×1.2+(1-r)×0.8)`。
- U：base 三张宿主表（+0x100/108/110）以装饰物 y 承载；输出候选 0x68 偏移为类型合同 U。

### 3. add_tree_setdressing（0x1422d3800）— 重建
- 修：**无全局早退**；`bVar7=!is_dragging && kind(+0x2c)==2`；半径=`scale+0.9`
  （started/corrupted 再 +0.4）；内层 **12**、u=`rem_euclid(i/12+f32×0.1)`；
  两点抖动（bVar7 或 started/corrupted ×1，否则 ×0.5）；三路偏移（started/corrupted 或 bVar7
  → -0.05/0.8；否则 -0.8/0.8，键偶 → -0.1/1.3）；scale 同 lantern。
- U：树存储/宿主表由 `TreeEntry` 承载；`IsCorrupted` 新资源。

### 4. add_preplaced_autoclutter（0x141ce5900）— 重建
- 修：键改为 `calculate_hash<tuple<&str,usize>>`（**不含 host**）；候选索引初值 **0xffff**；
  一次性 `Wait` 门控（`placed`）。
- U：0x1c 记录布局/swizzle 目标及 preplaced 数据来源。

### 5. add_autoclutter_around_gates（0x1422cc390）— 重建
- 修：宿主 stride 0x38（curve@+8/len+0x10/WallId+0x18/方向+0x20/+0x28/u32+0x30/u8+0x34）；
  **曲线点数 >2** 才处理；`gate_right` 用首点、`gate_left` 用末点（各 +2.0 y）；normal=(-dy,dx)/|d|；
  side -1/+1；键 tuple<&str,u32,WallId>；候选索引 0xffff。
- I：位置 0.5/normal swizzle 的 SSE 展开取近似；区域基向量 2.0 常量语义 U。

### 6. add_autoclutter_around_windows（0x1422cefe0）— 重建
- 修：`DecoratorType::spawns_clutter` 门控；子类型分派 `k = if 2 < b-2 {0} else {b-1}`；
  k==0 → 名 **decorator_right/decorator_left**、键 tuple<&str,usize,u32>；
  k==1 且 subtype∈{5,10} → 名 **trap_door**、键 tuple<&str,i32,usize,u32>、2 次、±1.5。
- I：曲线 extend_right_until/位置 4.72/0.28 项；宿主由 `WindowHost` 承载。

### 7. manage_autoclutter_entities（0x1422d04e0）— 重建
- 修：现存集排除 marked；删除只 insert ClutterMarkedForDeletion；新建 `spawn_empty` +
  scale 分派（kind4/≥7 → r×0.1；kind6 → (1-r)×0.2+r；其它 count>4 →
  (1-r)×0.3+r×0.8，否则 ((1-r)×0.1+r)×0.3）；四元数
  `Quat::from_rotation_arc(Z, v)`（v=+0x54/+0x58/+0x5c）；kind∉{2,3..6\1} →
  WindowShutter+ColorId。
- U：NaniBundle 实例选择（mesh selector/subset kind）与 AutoClutterConvertableToDecoClutter
  类型未落库 → 不伪造插入（core-type-gap）。原 `jitter_position` 近似已删除。

### 8. autoclutter_animation（0x1422ccef0）— 重建
- 修：delta 三路（forced→dt/-0.3；timer≤0→dt/(has_base?0.4:0.3)；否则 timer-=dt,delta=0）；
  progress clamp01；无 base → progress==0 && !NaN despawn；有 base → sign=flag?1:-1，
  `angle=(eased×sign×60-(1-eased)×sign×50)×0.017453292×0.5`，
  `quat=from_rotation_z(angle)×base`；forced 路径 scale=eased。
- U：状态表 stride 0x18 以组件承载；timer<0 的 0x18B 事件类型未落库，不伪造。

### 9. construct_inter_roof_merlons（0x1422d4870）— 重建
- 修：门控 `!(half || flat_w_no_rim)`；count=`ceil(周长/0.46)`；`random_splits_into(count, 0.32/周长)`；
  普通段偏移 `宽基+0.115`；额外段
  `((r×0.5-(1-r)×0.5)×0.21×0.8+0.368)×0.5+(宽基+0.23)`；砖缩放
  `((r×0.5-(1-r)×0.5)×0.07+0.28)`。
- I：段端点/法向 SSE 展开；BrickTransformSsbo 写入为宿主边界。

### 10. detect_inter_roof_merlons（0x141ce3a70）— 重建
- 修：收尾顺序改为先收集 descriptions、循环后逐条 `WallHoles::add`（O 尾部 stride 0x28 循环）；
  retain 两表；flat OR 门控；`roof_shape_at_intersection_y != 2` → expand ×2 → intersect_shapes。
- I：expand 量 0.3/0.6 与交点几何。

### 11. request_inter_roof_merlon_audio（0x1422d4230）— 重建
- 修：删除自造“距砖 1.0 内”循环；改为 `AppMode::native_tag()==13`（Replay）早退 +
  两次 `CursorTerrainRaycast::position()` 事件（bucket 判别 8/9）。
- U：两事件精确条件与 0x28B bucket 载荷；描述 Vec 搬迁为宿主缓冲。

### 12. detect_intra_shape_corners（0x141cd1a90）+ add_hole（0x141cd3ce0）— 重建
- 修：角高 `max_y + (is_pitched^1)×-0.67`；新增 `wall_space_hole`：
  `WallSpaceCoord::from_world_space ×2 → subtract`；y<0 加墙宽补偿；clamp；
  `0.0 < 高度差` 才置 kind=1（否则 5）；收尾统一 `WallHoles::add`。
- I：intersect_shapes 交点采样（64×64 最近邻 <0.2）与 wall-space 精确投影。

### 13. spawn_stitches（0x141ce65b0）/ spawn_stitch_bricks（0x141ce7190）— 重建
- 修：**极性纠正**——`n=trunc(min(1000,2×|start-length|))`，`n>2` 才处理（0x141ce6912 `JA`）；
  `threshold>=0.2` 跳过；`random_splits_into(n, 0.2/|diff|)`；jitter=`(r×0.5-(1-r)×0.5)×0.1+0.4`；
  delta=`Δ+(-0.1)`；c24=0.3；`spawn_stitch_bricks` clear 后逐条 push。
- I/U：缝合曲线几何（宿主 curve/slot）与 SparseInstanceBufferWriter/BrickTransformSsbo 写入。

### 14. autoclutter_subscribe_to_tabloids（0x1422d6930）— 判等（顺序）
- 13 订阅按 O 顺序：**8× RaycastWorld::subscribe_to_changes → 1× Publisher<DirtyTiles>
  → 3× WorldRaster::subscribe<512> → 1× Publisher<DirtyTiles>**（TweakerView 访问为宿主缝）。

### 15. remove_region_from_lazy_grid（0x140ee4a70）/ add_region 闭包 — 判等（键）
- 懒网格闭包改为写入/删除区域 **GlobalId（key）**，删除占位 `0`；`remove_region_from_lazy_grid`
  按区域矩形逐格 `retain(g != key)`。

### 16. set_current_tick / reset_autoclutter_for_new_frame — 未实现（U）
- B 无独立符号（内联）；仅 PDB 签名 `(Res<GameTime>, ResMut<AutoClutterState>,
  ResMut<InterShapeBrickStitches>)` 与 epoch/清空可见。系统包装已提供但**未注册**，
  不伪造与 stitch 的交互。

## 分歧检查表命中

- #2 门控极性反：spawn_stitches `n<=2` 原为 `continue`（反）→ 改 `n<=2 skip / n>2 process`（已修）。
- #3 载荷平面近似：manage NaniBundle 选择、lantern/tree 输出 0x68 偏移、window 0x28B
  bucket → 登记 U，不伪造。
- #4 事件/命令自造：merlon audio 原“距砖 1.0”自造逻辑删除，改回 Replay 门 + 光标事件。
- #8 偏移/字段错位：window 名字（decorator_right/left/trap_door）、gate 宿主偏移、
  preplaced 键 tuple 已按机器码纠正。

## 红线

- 未改 shared（country-core / utils）；只改 `crates/systems/clutter/**` 与本地证据/报告。
- 未使用 `--write` worker（全部只读 shared）。
