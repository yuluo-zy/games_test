# groupA RE-ANALYSIS — forest（森林系统）

dbid: tg_xa1 · 2026-10-16 · decomp: `groupA/forest/decomp/`

## 1. 覆盖盘点

| VA | PDB 名 | 结论 | 说明 |
|---|---|---|---|
| 0x140650c50 | impl$0::build | O | ForestPlugin::build（2 系统 + 1 资源 init，36B 默认实例 0x142b8fc58） |
| 0x141bf16c0 | manage_tree_prefabs::manage_tree_prefabs | O | 森林树管理主系统（密度采样+间隔+随机缩放） |
| 0x142279500 | handplaced_tree_animation | O | 手放树动画（相机距离淡出 canopy） |
| 0x14227a0d0 | forest_tree_animation | O | 森林树动画（easing 时间驱动） |
| — | manage_tree_prefabs::impl$0::default (line 33) | 胶水 | derive Default |
| — | is_too_close_to_shape (line 211) | 内联 | 逻辑并入 manage_tree_prefabs（§3） |

## 2. 布局 / 契约

**ForestPlugin::build (0x140650c50)**：`init_resource`（36B 资源，静态默认 0x142b8fc58 —— 候选 InteractableForestPlacements{FixedVec24B+12B}，I）+ `add_systems` ×2（0x140ea7c90 / 0x140ea7ff0 —— 3 个系统分两组注册）。

**manage_tree_prefabs (0x141bf16c0)**（14 参）—— 每帧两阶段：
- 输入：param_3 = &Vec<0x50B 树候选记录>（既有树），param_5 = ResMut<Vec<0x50B 树记录>>（树库写侧），param_6 = 树 id→索引表（0x14B/条目 {u32 id, u32 索引}）+ 存在位集（param_6[1] 的 {ptr@+8, mask@+0x28..}），param_9 = RNG 种子资源（seed = `*(u32*)(*param_9+0x2c)`），param_7 = 形状表（stride 0x28：{type@+0? , 半径@+0xc, 顶点/周长数据}），param_12/param_13 = 栅格资源（0x14067dd80 访问器 → Vec + flag@+0x298，同 water 音频格容器形），param_11 = PathRastersResources（as_float_grid 0x140ae4c20）。
- **阶段 1（候选评估）**：对每个 0x50B 候选：位置（PlacementDataSsbo::position 0x140ad51a0）→ 越界判定 `|x|≥65 || |y|≥65`（65.0=0x142a83814，掩 0x7fffffff=0x14292e980）→ 标记 flag=1；否则采样 **6 栅格**（0x140a14df0，scale 130）：路径栅格 ×4 + 2 附加栅格；任一附加栅格 >0（路上/水上）→ flag=0；否则密度 = `clamp(Σ4, 0, 1)`（0x142925904=1.0）；`is_too_close_to_shape`（内联：Circle2d/Rectangle2d 最近距离 `dist ≤ shape_radius + 7.0`，7.0=0x142a7cd48）→ 太近 flag=0；通过 → `Circle2d::new(center, r=7.0)` + `WallSpatialHash::query_with_circle(闭包 0x1422785d0)`（**与墙共享空间哈希**！）→ 无墙冲突才 push 候选 {tree_id u64, flag u8} 到 param_14 Vec（0x10B 条目）。
- **阶段 2（执行）**：遍历候选 {id, flag}：
  - flag==1（新树）：`fastrand::f32`（0x140caf380）→ 缩放 `lerp(0.3, 0.75)`（0.75=0x1429edb90，0.3=0x1429258d8）；`Commands::spawn_empty` + 0x38B bundle {FUN_142278a00, FUN_142278f00, entity, 树 id 指针, u32 颜色/型, f32 y, f32 scale, u8}；地形高过滤 `y ∈ [-65, 65)`；高度抖动 `+ fastrand*10.0`（0x142a95db0 高字=10.0）；`pack_2x16f_uint(pos, 1.0)`（0x140c90df0）；push 0x50B 树记录到 param_5（{Arc<String> prefab, u64 打包 pos, f32 高, f32 缩放, ...}）。
  - flag==0（既有树）：id → 表查 → 存在位集校验 → 缺失则发 despawn bundle（0x18B/0x20B，FUN_142278920/FUN_142278cf0 等）。
  - 末尾 `flag(plVar21+3)=1`（帧内已完成标记）+ ResMut tick 回写 ×N。
- 常量：65.0（界）/ 7.0（树-形间距）/ 0.3..0.75（缩放）/ 10.0（高抖）/ 0x43020000=130.0（栅格尺度）。

**handplaced_tree_animation (0x142279500)**（5 参）：
- puffin 域 + AudioTriggerLibrary::get_triggers/boxes（0x14157ce90/40 —— 树风音频触发）；
- `CameraMatrices::eye_direction/eye_position`（0x140c96d80/40）+ `Aabb3::intersect_segment`（0x141240570）→ **相机视线与树 canopy AABB 求交** → 命中则按距离淡出（写入 canopy 可见性，`pack_2x16f_uint`）；调用点含 should_remove_tree_canopy_from_view（内联 line 152）。

**forest_tree_animation (0x14227a0d0)**（8 参）：
- `EasingFunction::apply`（0x140c95ef0，easing 表）+ 时间参数（1.0/0.3 常量）→ 批量树 sway/bend 动画（0x140562630 grow ×1）。

## 3. 关键小函数

- **0x140a14df0 栅格采样**（见 water §3）—— forest/water/tree 三域共用。
- **0x14067dd80 访问器**：`(res) → res.grid.Vec 起止`（flag@+0x298 未初始化时返回空 Vec 起止）。PDB 名 FinalizedHoleStorage::holes 系共享单态别名，**以机器码行为为准**。
- **0x1409ea800**：形状近距检查辅助（circle/rect 分派，与 is_too_close_to_shape 配合），U 细部。
- **0x142278xxx 家族**（forest crate 内 bundle 构造器）：0x1422782d0（String 匹配）、0x1422785d0（spatial hash 圆查询闭包 = WallSpatialHash::query_with_circle 单态）、0x142278a00/0x142278b10/0x142278c00/0x142278920（Commands bundle 构造）、0x142278cf0/0x142278f00（bundle 析构/第二构造器）。restorer 可在 forest crate 内自建等价物。

## 4. panic / 串

- puffin 名串：0x142efd668 / 0x142efd768（'crates/systems/forest/src/manage_tree_prefabs.rs' 及函数名切片源，rfind('\\'|'/') 后取末段）。
- panic 位置均指 manage_tree_prefabs.rs（0x142efd7f8/810/828/840/858/7c8/7e0/798 为 {file,line} fat 对）。

## 5. 消费者线索

- **WallSpatialHash::query_with_circle_callback**（0x1425defd0 单态=convert_preplaced_trees 用；0x1425df320=hide_trees 用）—— 墙域 wall_spatial_hash 与树域共享（0x1422785d0 同族）→ 树-建筑间距规则属墙域数据面。
- TreeStorage（0x140a1b100 iter / 0x140a38460 get / 0x140a38560 insert / 0x140a385a0 remove / 0x140a38600 next_index）—— 已章节化（chapters/resources/tree_storage.rs.md），布局一致（16B 槽位图 + +0x30 计数）。
- 0x1425de2b0（process_tree_cmds 用）= 树命令 dispatch 辅助（tree-tool crate）。
- 常数 0x142a83814=65.0 亦出现于 tree-tool recalc（地形高过滤界）。

## 6. U 项

- 6 栅格中附加 2 栅格的身份（树掩膜/水掩膜候选：water_mask/garden_mask，I）。
- 36B 资源（init_resource）字段明细。
- 0x1409ea800 精确算法（形状近距的圆/矩形分派细节）。
