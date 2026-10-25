# systems/forest RESTORE — restorer-eco-forest

> 还原批次：2026-10-17（一轮 + reviewer-eco-forest FIX 6 项重做）。目标：
> `crates/systems/forest`（package `forest`，B 版）。
> 方法：**fresh 反编译**（Ghidra shared，程序 `tiny-glade.exe`，worker `shared`）→ 重建
> 原函数控制流 → Rust body 逐语句对应；country_core 类型缺口处显式标分歧，不伪造。
> 证据索引见文末。

## 结论速览

| 函数 | VA | 分类 | cargo check |
|---|---|---|---|
| `ForestPlugin::build`（`impl$0::build`） | `0x140650c50` | **glue-legitimate**（判等：注册面逐调用对上） | ✅ |
| `manage_tree_prefabs` | `0x141bf16c0` | **重建**（控制流/判定分支逐语句；reviewer FIX 已吸收）+ 未实现分歧 | ✅ |
| `handplaced_tree_animation` | `0x142279500` | **重建**（AppMode 跳表 + 逐树循环；FIX 已吸收）+ 未实现分歧 | ✅ |
| `forest_tree_animation` | `0x14227a0d0` | **判等**（reviewer 复核通过；算法控制流逐语句对上） | ✅ |
| `is_too_close_to_shape`（内联） | 宿主 `0x141bf1d6a` 区 | **重建**（点族几何判等；矩形分派 U） | ✅ |

`cargo check -p forest`：**通过**（exit 0，本还原批内两次独立确认：forest_check5.log 与
`mut` 修复后 0.59s Finished）；`cargo clippy -p forest`：无森林专属告警。
> ⚠ 2026-10-17 会话内 country-core 被其他域 agent 并发编辑（meta_commander/atmosphere/
> time_of_day/save 等 15 处中间态错误，均在 `crates/country-core/src/`，与 forest 无关）——
> 上述通过为 country-core 稳定窗口内实测；forest 代码本身零错误（无 `crates/systems/forest/`
> 路径错误）。country-core 稳定后 `cargo check -p forest` 恢复通过。

> reviewer-eco-forest 复核（REVIEW.md）判等项：forest_tree_animation 全链、AppMode 减集
> {0,3,9,10,13,14,16,20}、相机射线 `eye+dir*4.5`（K=1.5）。FIX 6 项已在本轮吸收（§2/§3）。

---

## 1. `build`（0x140650c50）— glue-legitimate（判等）

fresh 反编译（`evidence/eco_forest_140650c50_build.c`）逐调用：

```
chain_ignore_deferred  (IntoScheduleConfigs, 0x140ea68d0)
intern<dyn SystemSet>  (Interner::intern, 0x140ea74a0)
in_set_inner<Box<dyn System>> (ScheduleConfigs, 0x140ea7c00)
add_systems<Update>    (SubApp::add_systems, 0x140ea7c90)
insert_resource<InteractableForestPlacements> (App, 0x140ea7ff0)
```

即：`insert_resource(InteractableForestPlacements)` + `add_systems(Update,
(manage_tree_prefabs, handplaced_tree_animation, forest_tree_animation).chain().in_set(Set))`。
纯 Bevy 注册胶水 → **glue-legitimate**，不重建。

实现（`src/lib.rs`）：新增 `ForestSystems::Trees`（SystemSet 惯例名，原 1 字节自定义
集合值 0x24 名 U）；`add_forest_cpu_systems` 以 `.chain().in_set(...)` 落地。
`register_forest_events` 的 3 个事件 `init_resource` **不在原 build 面**（O：0x140650c50 无
register_event），属本 crate 独立可运行接线惯例（分歧登记，prefab/timberframe 域宿主应注册）。

## 2. `manage_tree_prefabs`（0x141bf16c0）— 重建 + 未实现分歧

fresh 反编译（`evidence/eco_forest_141bf16c0_manage_tree_prefabs.c`）完整控制流已重建：

**阶段 1（候选评估）**——逐放置记录（原 stride 0x50）：
1. 界外 `|x|>65 || |z|>65`（`0x142a83814=65.0`）→ **keep=1**（界外放置也保留，O 直接观察）；
2. 否则 **6 路** `bilinear_sample_at_world_xz`（reviewer 复核 0x141bf1cf9-1d5b）：
   - **前 4 路求和** → `clamp(Σ4,0,1)` 作**密度项**（`ADDSS XMM11..14` + `MAXSS 0/MINSS 1.0`，
     0x141bf1d1d-3c）——density>0 → keep=0；
   - **后 2 路排除项**（water/garden）任一 `>0` → keep=0（0x141bf1cef/cf9）；
3. 否则 **rock mask 单次布尔**（`sample_at_world_xz` 0x1409ea800；`SETA SIL; OR SIL,AL`）→ keep=0；
4. 缓冲位（`[RBP+0x108]`，构造事件迭代 SETBE 0x141bf1b0f）OR 入跳过条件——**形状/墙段
   判定仅在「存在构造形状 && density==0 && rock==0」时运行**（0x141bf1d62/d64）；
5. 否则 `is_too_close_to_shape`：点形状 `dist < shape_radius+7.0`；矩形
   `dist(closest_point_on_perimeter) < 7.0`（`0x142a7cd48=7.0`）；
6. 否则 `Circle2d::new(center, 7.0)` + `WallSpatialHash::query_with_circle_callback`
   （单态 `0x1422785d0`）→ 回调逐墙段 `is_too_close_to_shape` 翻转 keep。
   push `{tree_id u64 @0, keep u8 @8}`（16B）到 `MemCache`。

**阶段 2（执行）**——`init_rng` 后遍历候选：
- init 标志（placements `+0x18`）：首帧读记录 `+0x1c` f32；后续帧 `fastrand::f32()`
  → `r*0.75+(1-r)*0.3`（`0.75=0x1429edb90`，`0.3=0x1429258d8`）；
- keep≠0：实体未生成（`Entity::generation==0`）→ `spawn_empty` + `insert::<InteractableForestTree>`
  + `insert::<TreeAmbience>`（位置 {x, `TerrainHeightsData::sample_at_world_xz` 高度+jitter*10.0, z}；
  高度采样仅当 `-65≤x,z<65`，O）+ 原子自增引用计数 + `pack_2x16f_uint` + 写 `SpawnPrefabCmd`
  （stride 0x50）+ 记录实体回写（真实 `+0x40`；本实现经 `record.id = entity.to_bits()` 落地，
  避免下帧重复生成——偏移分歧 I）；实体已生成且 `Query<&TreeMarkedForDeletion>::contains` →
  `remove::<TreeMarkedForDeletion>`（warn）；
- keep=0：实体已生成且未标记 → `insert::<TreeMarkedForDeletion>`（panic）；
- 帧末 placements `+0x18 = 1`。

**未实现分歧（country_core 类型缺口，仅登记不伪造）**：
- 放置记录真实 0x50B `{Arc<String> prefab @0, u64 打包 pos @0x10, f32 高 @0x14, f32 z @0x18,
  u32 prefab 索引 @0x1c, …实体 @0x40, u8 有树标志 @0x44}`；当前 `InteractableForestTree`
  `{id, position, scale, _gap}`（0x20B）无实体/init/+0x44 位 → 实体映射以 `record.id` 高位
  generation 承载、init 标志以 `MemCache.finished` 承载（分歧 I）。
- **构造形状的 `is_too_close_to_shape` 判定未实现**（reviewer FIX 2）：形状源 =
  `ConstructTimberframeCmd` 载荷 ShapeParameters 0x28B，当前为 `shape: Vec<u8>` 占位 →
  形状子判定保留结构（有构造形状帧进入该段）、以墙段判定落地，形状判定登记待补证。
- **墙回调精度**（reviewer FIX 3 已吸收）：回调经 `is_too_close_to_shape` 逐墙段判定
  （墙段中点 + 半径=半厚 U 以 0.0 落地），不再「任一段即挡」近似。
- `SpawnPrefabCmd` 真实 stride 0x50 载荷；当前 `{_gap}` → 仅能承载索引。
- rock mask 单次布尔已按 reviewer 复核恢复（FIX 1）；`[RBP+0x108]` 缓冲位 = 构造事件
  存在性以 `EventReader::is_empty()` 判定（分歧 I，语义 O）。
- 引用计数原子自增 / `pack_2x16f_uint` 打包位（无记录字段承载）→ 分歧。

## 3. `handplaced_tree_animation`（0x142279500）— 重建 + 未实现分歧

fresh 反编译 + 定点反汇编（`evidence/eco_forest_142279500_handplaced_tree_animation.c`、
`evidence/hp_step0.txt`，575 条指令 + 跳表 `0x143038050` 读回）已重建：

- **AppMode 跳表分派**：标签 {0,3,9,10,13,14,16,20} → 减（淡出），其余 → 加（淡入）
  （跳表项解码：0x142279812 加 / 0x142279842·848 减 / 0x142279837 兜底 tag 0·3 减）；
- 全局 canopy 状态：`progress = clamp(progress ± delta, 0, 1)`；`smoothstep = t²(3-2t)`
  （`0x1429cd078=3.0`）；写状态 `+0x4` 与输出指针 `+0x10 = +0x1c`（输出身份 U）；
- 相机眼点/朝向 → 线段 × `AudioTriggerCollection::boxes`（`Aabb3::intersect_segment`
  `0x141240570`）；盒扩展常量（±1.5/±1.0）O 读回；
- 逐树：`step = delta/自定时长` 或（标记树/标志非 0）`delta/-0.4`（`0x142a7fcb0=-0.4`）；
  `progress = clamp(+step,0,1)`；淡出分支 `~(subtract & !audio_hit) & (progress==1.0)`
  → `fade += delta`（min1），否则 `fade -= delta`（clamp[0,1]）；
  变化时 `pack_2x16f_uint(progress, 1-(1-fade)²)` 写 `PrefabInstanceData[+0x20]`。

**未实现分歧**：
- `TreeAnimation` 真实 stride 0x10 {+0 时长, +4 进度, +8 淡出, +0xc 标志}；当前
  `{alpha f32, _gap u64}` **缺 +4 进度位** → 进度以 `Local<Vec<f32>>`（按迭代序）承载
  （reviewer FIX 4 采纳的「等效字段」；树集稳定时行为等价，分歧 I）；
- `at_max` 现用真实 `progress == 1.0`（非硬编码）；`pack_2x16f_uint(progress, 1-(1-fade)²)`
  用真实 progress（FIX 4 已吸收）；
- `PrefabInstanceData` 真实 stride 0x30（写 +0x20）；当前 0x20 → `_gap` 承载（分歧 U）；
- 状态第 8 参身份 U（输出指针/值语义）→ 以 `Local<HandplacedCanopyState>` 落地（分歧）；
- 盒扩展 lane 常量（±1.5/±1.0）为 SIMD 逐 lane 加法，非简单 min/max±常量（reviewer 复核，
  分歧 U）；`Aabb3::intersect_segment` 3D 版 utils 未落地 → 本地等效实现（标注）。

## 4. `forest_tree_animation`（0x14227a0d0）— 重建 + 未实现分歧

fresh 反编译（`evidence/eco_forest_14227a0d0_forest_tree_animation.c`）全链重建：

- 逐树：标记树 → `step = delta/-0.3`（`0x1429258fc=-0.3`），growing=false；否则
  timer>0 → `timer -= delta`，step=0，growing=true；timer≤0 → `step = delta+delta`，
  growing=true；
- `progress = clamp(progress+step, 0, 1)`（`MAXSS 0 / MINSS 1.0`）；
- 进度变化时 `EasingFunction::apply(progress)` **× 幅度** → 写 `PrefabInstanceData[+0xc]`
  （当前 type +0xc = `scale`，偏移对上，语义 I）；
  **缓动变体已解**：静态 `0x14303802c` 首字节 `0x06` = `EasingFunction::CubicOut`
  （utils 判别序，O）；
- `!growing && progress == 0.0` → `EventWriter<DespawnPrefabCmd>`（stride 0x10）+ 清放置
  记录 `+0x44` 有树标志。

**未实现分歧**：`placements.records[tree.id].+0x44 = 0` 当前 country_core 记录无该位 →
保留结构、不伪造（`InteractableForestTree` 真实 stride 0x18 {+0x8 幅度, +0xc 进度, +0x10
计时器}；当前以 `position.x/y/z` 承载，偏移恰好对齐，语义名 I）。

## 5. `is_too_close_to_shape`（内联 helper）— 重建（已接线）

从宿主 `0x141bf1d6a–0x141bf1dea` 墙邻近区恢复（O，reviewer 独立复核）：
点形状 `dist = |tree_pos − shape_pos|`，过近 = `dist < shape_radius + 7.0`；矩形形状走
`Rectangle2d::closest_point_on_perimeter`（`0x140c9ae20`），过近 = `dist < 7.0`。
比较算子 `UCOMISS threshold,dist; SETA; XOR 1` → `dist < threshold` 为过近（O）。
Rust 以 `is_too_close_to_shape(tree_pos, shape_pos, shape_radius)` 落地，并**已接入
manage_tree_prefabs 墙回调活路径**（reviewer FIX 6，消除死代码）。
**未实现分歧**：`Shape` 类型/`+7.0` 语义（常量边距 vs 墙半厚补偿）U；矩形分派
（closest_point_on_perimeter）当前 country_core 无 Rectangle2d/Shape 合同——登记待补证。

## 证据索引（fresh 反编译，2026-10-17，worker shared）

`reports/eco/forest/evidence/`：
- `restorer-eco-forest-decomp.json`（get_functions：四函数 signature+decompiled_code+locals+callees）
- `eco_forest_141bf16c0_manage_tree_prefabs.c`（569 行 decomp）
- `eco_forest_142279500_handplaced_tree_animation.c`（puffin 面 + 跳表分派 decomp）
- `hp_step0.txt`（0x142279500 全函数 575 条反汇编；含 AppMode 处理器 0x142279812/842/848、
  相机射线 0x1422798ab、逐树循环 0x142279a30 起）
- `eco_forest_14227a0d0_forest_tree_animation.c`（319 行 decomp）
- `eco_forest_140650c50_build.c`（build 注册面）
- `restorer-eco-forest-handplaced.json`（跳表 `0x143038050` 读回 + 常量）
- `restorer-eco-forest-consts.json`（常量读回：`0x14303802c=0x06`（CubicOut）、
  `0x1429258fc=-0.3`、`0x142925904=1.0`、`0x1429cd078=3.0`、`0x142a7fcb0=-0.4`、
  相机射线常量 3.0/1.5/±1.0/±1.5）

作业 job：`reverse-engineering/tiny-glade/tool-usage/jobs/restorer-eco-forest-*.json`。

## 接线注记

- `src/lib.rs` 保留 `(world, schedule)` 惯例（与 system-water 同批一致）；原 build 经 Bevy
  App 机制，宿主（tiny-glade crate）负责 App 接线。
- `crates/systems/forest/Cargo.toml` 新增 `bytemuck.workspace = true`：reviewer 复核 rock
  区（4×rock mask 采样误读 → 单次 rock 布尔 + 4 路 bilinear 密度）后**仍需要**——单次
  `sample_at_world_xz(mask: &[u32])` 需把 `MappedStorageBuffer.mapped: &mut [TerrainRockMaskSsbo]`
  按 `&[u32]` 重解释（Pod cast），无其它等价契约（原锁文件依赖族含 bytemuck 1.16.0）。
- 事件资源（SpawnPrefabCmd/DespawnPrefabCmd/ConstructTimberframeCmd）原 build 未注册，
  建议由 prefab/timberframe 域宿主统一注册（分歧登记，见 §1）。
- reviewer FIX 6 项吸收情况：FIX1 rock 区重做 ✓、FIX2 形状判定（构造形状源 core 缺口，登记
  未实现）△、FIX3 墙回调逐段判定 ✓、FIX4 handplaced at_max/pack 真值 ✓、FIX5 森林树 +0x44
  清标志（core 类型加位后落地，登记）△、FIX6 is_too_close_to_shape 接线 ✓。
  △ = 受 country_core 类型缺口限制，登记待补证（本 crate 范围无法关闭）。
