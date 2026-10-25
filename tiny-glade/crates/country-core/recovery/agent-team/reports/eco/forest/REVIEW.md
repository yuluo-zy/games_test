# systems/forest REVIEW — reviewer-eco-forest

> **复验更新（2026-10-17，restorer 已吸收 FIX 6 项后）**：状态 **fix_verified**。
> 4 项聚焦复核结论：
> 1) **rock 区 6 路 bilinear** ✓ —— `for slot in 0..4 { density += path_rasters
>    .as_float_grid(slot).bilinear_sample(...) }`（前 4 求和→clamp 密度项）+ water/garden
>    后 2 排除 + `rock_mask_bool`（单次 `sample_at_world_xz(...) != 0`）——与反汇编
>    （ADDSS XMM11..14→MAXSS0/MINSS1.0→SETA→OR AL；后两路 UCOMISS>0）**结构一致**；
>    `mask_sum` 误读已消除，bytemuck 保留（单次 rock 布尔 Pod cast 确需，见下）。
> 2) **at_max/pack 真实进度、墙回调逐段** ✓ —— `Local<Vec<f32>>` 按查询迭代序承载
>    TreeAnimation[+4] 进度，`at_max = new_progress == 1.0`（非硬编码 true）、
>    `pack_2x16f_uint(new_progress, 1-(1-fade)²)`（非 1.0 占位）；墙回调改为
>    `is_too_close_to_shape(pos, segment 中点, 0.0)` 逐段判定（FIX3）。
> 3) **is_too_close_to_shape 接线** ✓ —— 已从墙回调调用，消除死代码；矩形分派仍登记 U。
> 4) **未实现/缺口登记** ✓ —— FIX2（构造形状源）、FIX5（+0x44）登记 country_core 缺口
>    未实现，准确无伪造。
> **残余登记（仍有待证，不阻断 fix_verified）**：
> - **`has_construct_shape = !constructs.is_empty()` 门控位来源待证**：restorer 注释将
>   `[RBP+0x108]` 缓冲位（0x141bf1d62/d64 门）归为「构造事件非耗尽」；按 reviewer 独立
>   反汇编（0x141bf1b0f SETBE BL & R14B，源为 SSBO 缓冲列边界比较，见 §8 与
>   `evidence/reviewer/reviewer-mtp-disasm.txt` L238-260）该位来自 **SSBO 缓冲状态**，
>   非直接 EventReader<ConstructTimberframeCmd>。门控语义（何时运行形状/墙段判定）需补证，
>   建议按 U 登记。构造形状子判定本身仍为未实现（FIX2，core 类型缺口）。
> - 墙回调形状 = 中点+半径 0.0（原墙段矩形 perimeter 距离 + 7.0）为近似，已登记。
> - `cargo check -p forest` 现被 **country-core 他组并行模块**（time_of_day / lighting /
>   save / signifier_create 等）编译错误阻断（country-core lib 11 errors，与 forest 无关，
>   森林代码仅引用已验证存在的 API）；restorer 早先「通过」在 country-core 可编译时有效。

# systems/forest REVIEW — reviewer-eco-forest

> 审查批次：2026-10-17。独立 fresh 反编译（Ghidra shared，程序 `tiny-glade.exe`，
> worker `shared`）+ 独立反汇编（disassemble_function）+ 独立常量/跳表读回，不依赖
> restorer 描述。证据：`evidence/reviewer/`（reviewer-fresh-decomp.json、
> reviewer-mtp-disasm.txt 895 条、reviewer-hp-disasm2.txt 575 条、reviewer-consts.json 等）。
> 对照对象：`crates/systems/forest/src/{lib.rs,manage_tree_prefabs.rs,tree_animation.rs}`。
> `cargo check -p forest`：通过（exit 0，与 restorer 一致）。

## 审查结论速览

| 函数 | VA | verdict | 核心分歧 |
|---|---|---|---|
| `ForestPlugin::build` | 0x140650c50 | **glue-legitimate**（判等为胶水） | 3 事件 init_resource 非原 build 面（增补） |
| `manage_tree_prefabs` | 0x141bf16c0 | **fix_verified**（原未实现-需重做；rock 区/墙回调/接线已修） | 构造形状子判定 FIX2 登记未实现（core 缺口）；`has_construct_shape` 门控位来源待证 |
| `is_too_close_to_shape`（内联） | 宿主 0x141bf1d6a 区 | **fix_verified**（已接线消除死代码） | 矩形分派未落地（core 缺口，登记） |
| `handplaced_tree_animation` | 0x142279500 | **fix_verified**（at_max/pack 真实进度已修） | 输出指针写未建模、盒扩展近似（登记） |
| `forest_tree_animation` | 0x14227a0d0 | **判等（算法控制流逐语句对上）** | `+0x44` 清标志未落地、DespawnPrefabCmd 载荷形状（core 类型缺口，FIX5 登记） |

---

## 1. `build`（0x140650c50）— glue-legitimate

**独立复述**（fresh decomp，`reviewer_0x140650c50_decomp.c`）：函数体仅 5 个调用面——
`chain_ignore_deferred`（0x140ea68d0）、`intern<dyn SystemSet>`（0x140ea74a0）、
`in_set_inner<Box<dyn System>>`（0x140ea7c00）、`SubApp::add_systems<Update>`（0x140ea7c90）、
`App::insert_resource<InteractableForestPlacements>`（0x140ea7ff0）。无任何业务分支。

**Rust 对照**（`lib.rs::build` → `register_forest_events` + `add_forest_cpu_systems`）：
- `add_forest_cpu_systems` = `(manage_tree_prefabs, handplaced_tree_animation,
  forest_tree_animation).chain().in_set(ForestSystems::Trees)` — 与原
  `chain_ignore_deferred → in_set_inner → add_systems(Update)` 注册面一致 ✓。
- `register_forest_events` 的 `init_resource<Events<SpawnPrefabCmd / DespawnPrefabCmd /
  ConstructTimberframeCmd>>` **不在 0x140650c50 面内**（fresh decomp 无 register_event /
  init_resource 调用）——属本 crate 独立可运行增补，已登记。原事件资源由
  prefab/timberframe 域宿主注册（接线惯例，非原 build 行为）。
- `ForestSystems::Trees` 为具名 SystemSet，替代原 1 字节匿名 0x24 值（名 U）。

**verdict：glue-legitimate**（纯 Bevy 注册胶水，注册面逐调用对上；增补的 3 个事件
init_resource 已登记、非原面）。

---

## 2. `manage_tree_prefabs`（0x141bf16c0）— 未实现-需重做

**独立复述**（fresh decomp `reviewer_0x141bf16c0_decomp.c` + 895 条反汇编）：

参数面（与 ANALYSIS/restorer 一致，14 参）✓。资源 `InteractableForestPlacements`
元素 stride **0x50**；`+0x18` = 已初始化字节；元素 `+0x1c` f32、`+0x40` 实体、`+0x44`
有树标志。

**阶段 1（候选评估，逐 0x50B 记录）**：
1. `PlacementDataSsbo::position()` 取 x/z；`|x|>65 || |z|>65`（`0x142a83814=65.0`）→
   keep=1（反汇编 0x141bf1be9-0x141bf1c05，XMM8 取 abs 掩码 `0x14292e980`，XMM6=65.0）。
   ✓ Rust `pos.x.abs()>WORLD_BOUND || pos.y.abs()>WORLD_BOUND`。
2. **6 路** `bilinear_sample_at_world_xz<RasterBufferRef<512>>`（0x140a14df0×6，XMM3=130.0、
   `[RSP+0x20]=130.0`）。结果存 XMM11-14（第1-4路）、XMM15（第5路）、XMM0（第6路）。
   - **后两路（第5/6）**：`UCOMISS XMM15/0 vs 0.0; JA → keep=0`（0x141bf1cef/cf9）。
     ✓ 与 Rust `water_level>0 || garden_level>0 → false` 结构一致（具体栅格归属 U）。
   - **前四路（第1-4）并非「无后续引用」**：`ADDSS XMM11,XMM12/XMM13/XMM14`
     （0x141bf1d1d-27）**求和**，`MAXSS 0 / MINSS 1.0` **clamp[0,1]**
     （0x141bf1d2c-3c），`UCOMISS >0`（0x141bf1d43）→ 密度项。⚠ **restorer 把这一
     项误读为 rock mask 采样**（RESTORE.md「前 4 路结果无后续引用」与反汇编矛盾）。
   - rock mask 单次调用 `sample_at_world_xz`（0x1409ea800）返回 AL，`SETA SIL; OR SIL,AL`
     （0x141bf1d46-4a）→ **AL 即 rock 布尔**。另有 `[RBP+0x108]` 缓冲有效位
     （0x141bf1d4d-57，来自 0x141bf1b0f SETBE BL & R14B）再 OR 入。
   - 汇合 `keep = !(密度>0 | rock_mask_bool | 缓冲有效位)`。
3. **形状判定（is_too_close_to_shape 内联区 0x141bf1d6a-0x141bf1dea）**：
   - `[shape]&1==0` 点形状：`dist = |shape_pos([+0x4] 打包{x,z}) − 点|`（MOVSD+SUBPS+MULPS+
     MOVSHDUP+SQRTSS），阈值 `shape_radius([+0xc]) + 7.0`（0x141bf1ddd-de2）。
   - `[shape]&1==1` 矩形：`Rectangle2d::closest_point_on_perimeter`（0x140c9ae20）→
     `dist(|最近点−点|) < 7.0`（0x141bf1d7e-1ab）。
   - `UCOMISS threshold,dist; SETA; XOR 1` → keep = !(dist<threshold)。
   - **shape 源 = 帧前置构造形状（RDI，来自 ConstructTimberframeCmd 面）**。
4. **墙查询**：`Circle2d::new(x,z,7.0)`（0x140c97540，半径直接 =7.0）+ 
   `WallSpatialHash::query_with_circle_callback<closure_env_0>`（0x1422785d0，R8=&keep）。
   回调逐命中墙段执行形状判定（closure_env_0 内 is_too_close_to_shape）翻转 keep。
   keep 为 0 则跳过墙查询（`TEST SIL,SIL; JZ`）。
5. push `{u64 记录+0x40 实体, u8 keep}`（16B）到 MemCache（Vec）。

**阶段 2（执行，`init_rng` 0x140c96240 后遍历候选）**：
- `placements[+0x18]==1`（后续帧）：`fastrand::f32()`（0x140caf380）→
  `r*0.75+(1-r)*0.3`（0x1429edb90/0x1429258d8）；jitter=0、flag32=0。
- 首帧：读 `record[+0x1c]` f32（jitter）、scale=0、flag32=1.0。
- keep≠0 且实体 generation==0（候选 u64>>32==0）：`spawn_empty`（0x140d146a0）→
  `insert<InteractableForestTree>`（载荷 {index, 候选索引, +0x1c, flag32, scale, 0}）→
  高度条件 `(-65≤x<65)&&(-65≤z<65)`（UCOMISS vs XMM12=-65/65，XMM13={0,10}）→
  `TerrainHeightsData::sample_at_world_xz` → `insert<TreeAmbience>`（位置
  {x, 高+jitter*10, z}，MOVSLDUP XMM14×XMM13={0,10} 使 x/z 不加抖动）→
  Arc<String> 原子自增（`INC.LOCK [record+0]`）→ `pack_2x16f_uint` →
  **SpawnPrefabCmd 事件（stride 0x50，载荷取自 +0x20/0x24/0x2c/0x34/0x3c/0x48）** →
  `record[+0x40]=实体`。
- keep≠0 且已生成：实体在 marked Query 中 → `remove<TreeMarkedForDeletion>`（warn）。
- keep==0 且已生成且未标记：`insert<TreeMarkedForDeletion>`（panic）。
- 帧末 `placements[+0x18]=1`。

**Rust 逐语句对照 & 分歧**：
| 原 | Rust | 判定 |
|---|---|---|
| 6 路 bilinear（前 4 求和、后 2 排除） | 仅 water/garden 2 采样排除；rock 区改 4×rock mask 采样同点求和 | **未实现**（数据源误读） |
| 密度=clamp(Σ4 路 bilinear)；keep=!(密度>0|rock_mask_bool|缓冲位) | rock=clamp(Σ4×sample_at_world_xz 同点)；keep=!(rock>0\|extra_al=false) | **未实现**（源与布尔都错） |
| is_too_close_to_shape（构造形状） | `shape_too_close=false` 硬编码短路 | **未实现**（注册） |
| 墙回调逐段形状判定 | `query_with_circle(7.0, \|_,_\| blocked=true)` 任一段即挡 | **未实现**（近似，过度拦截） |
| `record[+0x40]=实体` | `record.id=entity.to_bits()`（+0 偏移） | 分歧 I（注册） |
| `placements[+0x18]=1` | `cache.finished=1`（Local 代理） | 分歧 I（注册） |
| SpawnPrefabCmd 0x50B 载荷 | `{_gap: tree_id}` | 分歧 U（注册） |
| Arc 原子自增 | 省略（无 Arc 字段） | 分歧 U（注册） |
| pack_2x16f 打包载荷 | 未落地（无字段承载） | 分歧 U（注册） |
| 界外 keep=1 / 后两路排除 / 墙查询 7.0 / spawn/remove/标记逻辑 | 对应 | **判等** |

**verdict：未实现-需重做**。控制流骨架正确，但 keep 决策区有实质行为分歧：
rock 区把「4 路 bilinear 求和 + 1 次 rock 布尔」误构为「4×rock mask 采样 + false」；
构造形状判定短路；墙回调精度丢失。FIX 见 §6。

---

## 3. `is_too_close_to_shape`（内联 helper）— 未实现-需重做

**独立复述**（0x141bf1d6a-0x141bf1dea 反汇编）：两路几何谓词。
- 点形状（`[shape]&1==0`）：`dist=|shape_pos(+4 {x,z})−point|`，过近 =
  `dist < shape_radius(+0xc) + 7.0`。
- 矩形形状（`[shape]&1==1`）：`closest_point_on_perimeter` → 过近 = `dist < 7.0`。
- 比较算子：`UCOMISS XMM1,XMM0; SETA; XOR 1` → `threshold <= dist` 为 keep，即
  `dist < threshold` 为过近。

**Rust 对照**（`manage_tree_prefabs.rs::is_too_close_to_shape`）：`dist < shape_radius +
TREE_SHAPE_GAP` 仅实现**点族**；矩形分派未落地（注册 U）；函数**未被
manage_tree_prefabs 调用**（死代码，墙判定走 query_with_circle 近似）。

**verdict：未实现-需重做**。点族几何判等；矩形分派（closest_point_on_perimeter + 7.0）
未实现；且需接入墙回调（或构造形状判定）成为活函数，否则不参与真实 keep 决策。

---

## 4. `handplaced_tree_animation`（0x142279500）— 未实现-需重做

**独立复述**（fresh decomp + 575 条反汇编 `reviewer-hp-disasm2.txt`）：

1. **AppMode 跳表分派**（0x143038050，`tag-6<0xf`→`table[tag-5]`，否则 table[0]=0x142279837）：
   独立解码 16 项（i32 偏移 → 目标）：
   - 0x142279837 兜底：`CMP EAX,3; JZ 848; TEST EAX; JZ 848; JMP 812`（tag 0/3→减，其余→加）。
   - 0x142279812 **加**：`progress += delta`、R14B=0。
   - 0x142279848 **减**：`progress -= delta`、R14B=1（tag 9→0x142279842 同族）。
   - 归纳：**减集合 = {0,3,9,10,13,14,16,20}**。✓ **与 restorer `matches!(
     native_tag(), 0|3|9|10|13|14|16|20)` 完全一致**。
2. **全局 canopy 状态**（RDI，第 8 参，{+0 进度,+4 smoothstep,+0x10 输出指针,+0x1c 值}）：
   `progress=clamp(progress±delta,0,1)`（MAXSS 0/MINSS 1.0，0x142925904）；`[+4]=
   t²(3−2t)`（0x1429cd078=3.0）；`[+0x10]=[+0x1c]`（输出指针写值）。
   ✓ Rust `progress.clamp(0,1)` + `smooth=progress²(3−2·progress)`；⚠ 输出指针写未建模。
3. **相机射线**：`eye_position`（0x140c96d40）+`eye_direction`（0x140c96d80）→
   `dir.xy×Vec2(3.0,3.0)[0x142b2da40]×Vec2(1.5,1.5)[0x142ad3650]`=**dir.xy×4.5**；
   `dir.z×3.0×K(0x1429258d0=1.5)`=**dir.z×4.5**；+eye。→ **`eye+dir*4.5` 精确判等**
   ✓（restorer 记「K U 以 4.5 落地」实际 K=1.5，恰为精确）。
   `get_triggers(kind=0)`（AudioTriggerKind::Primary ✓）+`boxes()`，逐盒扩展后
   `Aabb3::intersect_segment`（0x141240570）；命中 → `NOT subtract&DIL`。✓ audio_hit。
   ⚠ 盒扩展为 SIMD 逐 lane 加法（XMM9=-1.0、XMM10={-1,1}、XMM11={-1.5,-1,-1.5,1.5}、
   XMM12={1,1.5}、XMM6=1.0），**非简单 min/max±常量**；restorer `min+(-1.5,-1,-1.5)/
   max+(1,1.5,0)` 为近似（注册 U）。
4. **逐树循环**（0x142279a30 起）：
   - marked（[RBP+0xb0]）→ `step=delta/-0.4`（0x142a7fcb0=-0.4）；否则
     `[anim+0xc] 标志≠0→delta/-0.4，=0→delta/[anim+0]`。✓
   - `progress=[anim+4]=clamp(progress+step,0,1)`（写回 +4）。⚠ **Rust 无 +4 位，不写回**。
   - `UCOMISS progress vs 1.0; SETZ/SETNP` → `at_max=(progress==1.0)`；
     `!(subtract&!audio_hit) & at_max` → `fade+delta(min1)`，否则 `fade−delta(clamp[0,1])`
     （0x142279d6f-98）。✓ 分支结构对；⚠ **Rust `at_max=true` 硬编码**（真实进度位缺失）。
   - 变化检测（progress/fade 与旧值比，含 NaN）→
     `pack_2x16f_uint(progress, 1−(1−fade)²)`（0x140c90df0）写 `PrefabInstanceData[+0x20]`
     （stride 0x30）。⚠ Rust `pack_2x16f_uint(1.0, …)` 用 1.0 占位进度，写当前 `_gap`。
   - 写 `[anim+0xc]=0` 清标志。

**Rust 逐语句对照 & 分歧**：
| 原 | Rust | 判定 |
|---|---|---|
| AppMode 跳表减集 {0,3,9,10,13,14,16,20} | matches 同集 | **判等** |
| 全局 progress/smoothstep | 同 | **判等**（输出指针写未建模→分歧 U） |
| 相机射线 eye+dir×{4.5,4.5}（xy 与 z 同） | `eye+dir*4.5` | **判等**（K 恰=1.5） |
| 盒扩展逐 lane 常量 | min/max±常量近似 | 分歧 U（近似） |
| `at_max=(progress==1.0)` 真值 | `at_max=true` 硬编码 | **未实现**（TreeAnimation 缺 +4） |
| progress 写回 anim[+4] | 局部落地不写回 | 分歧 U（缺位） |
| `pack(progress, 1-(1-fade)²)`→[+0x20] | `pack(1.0, …)`→_gap | **未实现**（占位/布局） |
| 输出指针写 | 未建模 | 分歧 U（注册） |
| Aabb3::intersect_segment 原实现 | 本地 slab 等效 | 分歧 U（utils 未落地） |
| should_remove_tree_canopy_from_view 内联 | 孤立函数未接线 | 分歧 U（注册） |

**verdict：未实现-需重做**。AppMode 分派/全局状态/相机射线/逐树 step 结构与 fade 分支
条件判等；但 `at_max` 硬编码 true（原须 progress==1.0 才走淡入）与 pack 用 1.0 占位进度
使 fade/pack 输出行为与原不符。FIX 见 §6。

---

## 5. `forest_tree_animation`（0x14227a0d0）— 判等（算法控制流）

**独立复述**（fresh decomp `reviewer_0x14227a0d0_decomp.c`，319 行）：
逐树（`InteractableForestTree` stride 0x18，+0x8 幅度/+0xc 进度/+0x10 计时器；
`PrefabInstanceData` stride 0x30）：
- marked（bVar33）→ `step=delta/-0.3`（0x1429258fc），growing=false。
- 未标记：`timer=[+0x10]`；`timer>0 → timer-=delta, step=0`；`timer≤0 → step=delta+delta`。
  growing=true。
- `progress=[+0xc]=clamp(progress+step,0,1)`（MAXSS 0/MINSS 1.0，写回 +0xc）。
- 变化（`!=旧值 || NaN`）→ `EasingFunction::apply(progress)`（static 0x14303802c 首字节
  **0x06**，utils 判别序 6=CubicOut ✓）`×幅度(+0x8)` → `PrefabInstanceData[+0xc]`。
- `!growing && progress==0.0` → `EventWriter<DespawnPrefabCmd>::write(实体)`（事件元素
  stride 0x10={实体 8B,EventId 8B}）+ `placements[树id][+0x44]=0`（stride 0x50，
  `panic_bounds_check` 防越界）。

**Rust 逐语句对照**：
| 原 | Rust | 判定 |
|---|---|---|
| marked→delta/-0.3, growing=false | 同 | **判等** |
| timer>0→timer-=delta, step=0 / ≤0→2×delta, growing=true | 同（position.z 承载 timer） | **判等**（语义名 I，偏移 +0x10 对齐） |
| progress=clamp(+step,0,1) 写回 +0xc | position.y 同 | **判等**（偏移 +0xc 对齐） |
| 变化→CubicOut.apply(progress)×幅度(+0x8)→[+0xc] | EasingFunction::CubicOut.apply × position.x → instance.scale | **判等**（判别值 0x06✓；+0xc 写 offset 对齐） |
| !growing && progress==0 → Despawn + 清 +0x44 | `despawns.write(DespawnPrefabCmd{entity:Some, _gap:id})`；`let _=&mut placements` 未清 | **判等**（事件写）+ **分歧 U**（+0x44 未落地、载荷形状） |
| 事件元素 stride 0x10 | DespawnPrefabCmd 当前 0x10B {Option<Entity>, _gap} | 分歧 U（注册） |

**verdict：判等（算法控制流逐语句对上）**。四分支 step/growing、clamp、缓动×幅度、
despawn 条件全部独立复述一致；幅度/进度/计时器以 position.x/y/z 承载、偏移
+0x8/+0xc/+0x10 与原型恰好对齐（语义名 I）。**FIX**：`placements.records[id].+0x44=0`
有树标志清除未落地（core 类型无该位）；DespawnPrefabCmd 载荷原为 8B Entity，当前
Option 承载（core 类型缺口）。

---

## 6. FIX 清单（按优先级）— 复验后全部已吸收

> 2026-10-17 复验：FIX1/FIX3/FIX4/FIX6 已在源码落地并核对一致；FIX2/FIX5 按约定登记
> country_core 类型缺口未实现。残余待证项见 §9 复验注。

1. **manage_tree_prefabs rock 区重做**（关键）✅ 已修：`for slot in 0..4` 求和
   `path_rasters.as_float_grid(slot).bilinear_sample(...)` → clamp 密度项；water/garden
   后 2 排除；`rock_mask_bool` 单次 `sample_at_world_xz != 0`。
   残余：`has_construct_shape` 门控位来源待证（[RBP+0x108] 独立反汇编指向 SSBO 缓冲状态，
   非直接构造事件存在性；建议补证后按 U 登记）。
2. **manage_tree_prefabs 构造形状判定** ✅ 登记未实现（FIX2）：`ConstructTimberframeCmd`
   载荷 ShapeParameters 落地前保留门控结构、不伪造成真实形状。
3. **manage_tree_prefabs 墙回调精度** ✅ 已修（FIX3）：回调改 `is_too_close_to_shape(pos,
   segment 中点, 0.0)` 逐段判定。残余：墙段形状以「中点+半径 0」近似（原矩形 perimeter
   距离 + 7.0），登记 U。
4. **handplaced_tree_animation** ✅ 已修（FIX4）：`Local<Vec<f32>>` 承载 TreeAnimation[+4]
   进度，`at_max = new_progress == 1.0`、`pack_2x16f_uint(new_progress, 1-(1-fade)²)`。
   残余：输出指针写（+0x10/+0x1c）未建模、盒扩展 lane 常量近似（登记）。
5. **forest_tree_animation** ✅ 登记未实现（FIX5）：`placements.records[id].+0x44=0`
   清有树标志（core 类型加位后落地）。
6. **is_too_close_to_shape** ✅ 已接线（FIX6）：墙回调已调用，死代码消除；矩形分派
   （closest_point_on_perimeter + 7.0）仍登记待补证（core 无 Rectangle2d 合同）。

---

## 7. Cargo.toml `bytemuck` 核实

`bytemuck = { version = "=1.16.0", ... }` 确在 workspace（Cargo.toml L104），
`bytemuck.workspace = true` 引用合法。`MappedStorageBuffer.mapped: &mut [T]`
（ssbo.rs L573），`sample_at_world_xz(mask: &[u32], ...)` 取 `&[u32]`，故
`bytemuck::cast_slice(storage.mapped)`（`&mut [TerrainRockMaskSsbo]` → `&[u32]`）在
**单次 rock 布尔路径下确属必要**（FIX1 重做后仍需）。之前误读路径下为误用、重做后
**保留理由成立**——核实结论：bytemuck 保留是必要的（非仅误读路径）。

## 8. 未实现/分歧汇总（复验后）

- **fix_verified**：`manage_tree_prefabs`（rock 区/墙回调/接线已修）、
  `handplaced_tree_animation`（at_max/pack 真实进度已修）、`is_too_close_to_shape`
  （已接线消除死代码）。
- 判等：`forest_tree_animation`（算法控制流），附 2 项 core 类型缺口 FIX（+0x44、载荷）。
- 胶水：`build`（glue-legitimate），附 3 事件 init_resource 增补登记。
- **残余登记**：`has_construct_shape` 门控位来源待证（[RBP+0x108] 独立反汇编来自 SSBO
  缓冲状态，非直接构造事件存在性，建议补证后按 U 登记）；墙回调形状「中点+半径 0」
  近似（原矩形 perimeter 距离）；TreeAnimation[+4] 以 `Local<Vec<f32>>` 按迭代序承载
  （树集变化会失配，登记）；构造形状子判定与矩形分派登记 core 缺口未实现。
- 全部 core 类型缺口（记录 0x50B/实体位/+0x44、SpawnPrefabCmd 0x50B、DespawnPrefabCmd
  载荷、TreeAnimation 布局、PrefabInstanceData 0x30）均为 restorer 已显式登记的分歧，
  无静默伪造。

## 9. 复验注

- 聚焦 4 项全部通过（rock 结构 / at_max·pack·逐段回调 / 接线无死代码 / 登记准确）。
- `cargo check -p forest` 现被 **country-core 他组并行模块**（time_of_day/lighting/save/
  signifier_create 等）编译错误阻断（country-core lib 11 errors，与 forest 无关）；森林
  代码仅引用已验证存在的 country_core API，country-core 恢复可编译后应能通过。
- 原始回执见下（保留首版结论）；复验后状态以本文顶部「复验更新」为准。

## 10. 回执（首版，保留）

5 函数独立 fresh 反编译+反汇编复核：build=胶水(glue-legitimate)、forest_tree_animation=
判等（算法逐语句对上）、manage_tree_prefabs/handplaced_tree_animation/is_too_close_to_shape=
未实现-需重做。FIX 6 项，关键 1 项：manage_tree_prefabs rock 区把「4 路 bilinear 求和 +
1 次 rock 布尔」误构为「4×rock mask 采样 + false」（restorer 断言「前 4 路无后续引用」
与反汇编矛盾）；另 at_max 硬编码、构造形状短路、墙回调近似、矩形分派未接线。无过度
断言——restorer 未宣称判等、分歧均已登记；但 rock 区误读需重做。bytemuck 依赖仅在
误读路径下必要，重做后可移除。
