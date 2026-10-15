# systems/forest 模块内部逻辑分析（B 版）

> 范围：只分析本 crate（`crates/systems/forest`）模块内部实现。不展开上下层调用链、不分析与其他 crate 的接线（country-core / tree-tool 等仅在被本模块直接消费时按需点名，不展开其内部）。
> 依赖库（std / glam / fastrand / serde / bevy_* 等）按约定直接复用，不逆向。
> 证据标注：**O**=原指令/反编译直接观察；**I**=由 PDB 类型信息/内联证据推断；**U**=unknown。
>
> 证据来源：
> - 骨架 PDB 元数据：`skeleton/B/crates/systems/forest/src/*.pdb.json`、`*.dependencies.json`、`*.structures.json.gz`
> - 反编译：Ghidra 共享库（`tiny-glade.exe`，sha256 `cf048e27…`），`get_functions`/`disassemble_function`/`read_memory`
> - 系统签名来自 PDB 泛型拼写（`FunctionSystem<fn(...)>` 的实例化），可靠性高（O/I）。

## 地址表（B，程序 `tiny-glade.exe`）

| 函数 | RVA | VA | 大小 |
|---|---|---|---|
| `forest::manage_tree_prefabs::manage_tree_prefabs` | `0x1bf16c0` | `0x141bf16c0` | 4187 B |
| `forest::tree_animation::handplaced_tree_animation` | `0x2279500` | `0x142279500` | 821 B |
| `forest::tree_animation::forest_tree_animation` | `0x227a0d0` | `0x14227a0d0` | 1879 B |
| `forest::impl$0::build`（`ForestPlugin::build`） | `0x650c50` | `0x140650c50` | 207 B |

内联/无独立符号（本 crate 命名空间，仅登记）：
- `forest::manage_tree_prefabs::impl$0::default`（`MemCache::default`，源行 33；`inlinee_declaration, instances=2`）
- `forest::manage_tree_prefabs::manage_tree_prefabs::closure_env_0`（墙查询回调）
- `forest::manage_tree_prefabs::is_too_close_to_shape`（源行 211；`inline_instance, inlinee=97574, instances=2`，被 `manage_tree_prefabs` 内联 —— 见 2.2）
- `forest::tree_animation::handplaced_tree_animation::closure$0`（源行 74；`inlinee=121211`）
- `forest::tree_animation::should_remove_tree_canopy_from_view`（源行 152；`inlinee=121209`，被 `handplaced_tree_animation` 内联）

---

# 1. `src/lib.rs`

## 1.1 `forest::impl$0::build` — `ForestPlugin::build`  @ `0x140650c50`

**内部逻辑（O）**：函数体只做 Bevy 插件注册，无游戏业务分支：

```
chain_ignore_deferred(...)                            // 0x140ea68d0  IntoScheduleConfigs::chain_ignore_deferred
intern::<dyn SystemSet>(...)                          // 0x140ea74a0  Interner::intern，把某 SystemSet 标签内联化
ScheduleConfigs::in_set_inner::<Box<dyn System>>(...) // 0x140ea7c00  把整串系统放进该 SystemSet
SubApp::add_systems::<(), Update, _>(...)             // 0x140ea7c90  注册到 Update 调度
App::insert_resource::<InteractableForestPlacements>(...) // 0x140ea7ff0  插入资源
```

调用点：`0x140650c73 / 0x140650c94 / 0x140650ca3 / 0x140650ce7 / 0x140650d0f`。
- 注册模式：先 chain（串行）→ 放进一个命名 SystemSet → 整体 add 到 `Update`。
- 资源插入：`country_core::resources::tree_storage::InteractableForestPlacements`（O，泛型拼写）。

**算法**：无（纯调度/注册胶水）。

**数据结构/常量（O）**：`Update` 调度；自定义 `SystemSet`。

**未知（U）**
- 被注册的**系统清单与顺序**：`build` 的 ABI 在 PDB 里是 `void()` 占位，`add_systems` 接收的系统元组经内联后未在体内出现函数指针；由 `get_xrefs_to` 可知 `manage_tree_prefabs` / `handplaced_tree_animation` / `forest_tree_animation` 各有 `FunctionSystem` 包装体（被 `run_unsafe` / `run_without_applying_deferred` 调用），推断三者由本插件注册到 `Update` 同一链，但顺序未能从体内直接读出。
- **SystemSet 名称/类型未知**：`intern` 处 `LEA R8,[0x142b8fc58]`（疑似 trait 对象 vtable）与 `LEA RDX,[RBP+0x8f]`（单字节 `0x24`），指向一个 1 字节自定义 SystemSet 值；类型/字符串未恢复。

---

# 2. `src/manage_tree_prefabs.rs`

## 2.1 `forest::manage_tree_prefabs::manage_tree_prefabs`  @ `0x141bf16c0`

**系统签名（O/I，来自 PDB `FunctionSystem<fn(...)>` 泛型拼写，高可靠）**

```rust
fn manage_tree_prefabs(
    ui_signifier_stream: Res<UiSignifierStream>,
    ssbo_library: Res<AssetSsboLibrary>,
    interactable_forest_placements: ResMut<InteractableForestPlacements>,
    commands: Commands,
    spawn_prefab_cmd: EventWriter<SpawnPrefabCmd>,
    tree_marked_for_deletion: Query<&TreeMarkedForDeletion>,
    construct_timberframe_cmd: EventReader<ConstructTimberframeCmd>,
    wall_spatial_hash: Res<WallSpatialHash>,
    game_time: Res<GameTime>,
    terrain_heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    path_rasters: PathRastersResources,
    water_raster: Res<WaterRaster>,
    garden_raster: Res<GardenRaster>,
    mem_cache: Local<MemCache>,
)
```

**内部逻辑（O，逐段）**

1. **清空 MemCache**：`mem_cache` 第 3 个字（`in_stack_00000070[2]`）置 0 —— 每帧复用本帧临时缓存向量。
2. **取 rock mask SSBO**：`ssbo_library` → `FrozenMap::get<SsboId, Box<BufferState>>`，随后 `assert` 该 SSBO 类型名为 `"country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo"`（长度 0x3f=63），即拿到 **`TerrainRockMaskSsbo`**（O，字符串常量直接可见）。
3. **遍历 `InteractableForestPlacements`**（元素 stride **0x50**）：
   - 读 `PlacementDataSsbo::position()` 得世界 x,z（`local_1f4` / `local_1ec`）。
   - **界外判定**：`|x| > 65.0 || |z| > 65.0` → 置 `flag = 1`（O，常量 `0x42820000 = 65.0`；栅格采样半径参数 `0x43020000 = 130.0`，即世界范围 ±65）。
   - 否则对 **6 个 float 栅格**做 `bilinear_sample_at_world_xz<RasterBufferRef<512>>`（water / garden / path 栅格，O）。若其中两个（water/garden）采样 > 0 → 置 `flag = 0`。
   - 否则读 `TerrainRockMaskSsbo::sample_at_world_xz()`，4 个采样求和并 `clamp(…,0,1)`；`flag = !(rock>0 | AL)`。
   - 若 `(rock>0 | AL)==0` 再做**墙体邻近**判定：`Circle2d::new` + `WallSpatialHash::query_with_circle_callback::<closure_env_0>` 查半径内的墙；半径 = 墙半厚 + `7.0`（常量 `0x142a7cd48`）；命中则翻转 `flag`（墙内 → 不可放树）。
   - 把 `(entity, flag)`（16 字节对）push 进 `mem_cache` 临时 Vec。
4. **`utils::rng::init_rng()`**：初始化 RNG。
5. **第二遍遍历缓存对**（`LAB_141bf1f20`）：
   - `InteractableForestPlacements[+0x18]` 是「已初始化」标志字节。为 0（首帧）取 `placements[i][+0x1c]` 的浮点值；为 1（后续帧）用 `fastrand::Rng::f32()` 生成 `rand*0.75 + (1-rand)*0.3`（O，常量 0.75/0.3 可见）。→ 每棵树的随机尺寸/朝向参数。
   - `flag != 0`（放置有效）：
     - entity 尚不存在（generation==0）：`Commands::spawn_empty()` → 排入 `insert::<InteractableForestTree>`（携带 index、prefab 值、尺寸/随机值、标志字节 0）与 `insert::<TreeAmbience>`（位置 = x、`TerrainHeightsData::sample_at_world_xz` 得高度、z，标志字节 0）；原子自增维护引用计数；`utils::pack_2x16f_uint` 打包；写 `EventWriter<SpawnPrefabCmd>`（事件元素 stride 0x50，O）。
     - entity 已存在且有效（generation 匹配 + 组件位图命中）：排入 `remove::<TreeMarkedForDeletion>`（取消删除标记，warn 处理器）。
   - `flag == 0`（放置无效）：若 entity 已存在且有效，排入 `insert::<TreeMarkedForDeletion>`（panic 处理器）。
   - 循环结束把 `InteractableForestPlacements[+0x18] = 1`。

**算法**：世界栅格双线性采样（water/garden/path）+ rock mask 采样 + 墙空间哈希圆查询 → 地形/占用有效性；事件驱动实体生命周期（`Commands` spawn/insert/remove + `EventWriter<SpawnPrefabCmd>`）；`fastrand` 每树随机化；`utils::pack_2x16f_uint` 打包两个 f16 到 u32。

**数据结构 / 常量 / 字段偏移（O）**
- `InteractableForestPlacements` 元素 stride **0x50**；资源内 +0x18 = 「已初始化」字节；元素 +0x1c = f32；+0x1c/+0x20/+0x24/+0x2c/+0x34/+0x3c/+0x48 等被读作组件数据。
- `SpawnPrefabCmd` 事件元素 stride **0x50**。
- 常量：`65.0`（世界半边界）、`130.0`（采样半径）、墙判定半径附加 `7.0`。
- 组件：`InteractableForestTree`、`TreeAmbience`、`TreeMarkedForDeletion`（均 country-core，O）。
- SSBO 断言字符串：`country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo`。

**未知（U）**
- `MemCache` 完整字段/布局（PDB 仅名字，fields/size=unknown）；本函数可见其至少含一个 `Vec<(Entity,u8)>` 风格缓冲（元素 16 字节，`[2]` 为长度）。
- `flag` 语义精确命名：观察到 `flag==1 ⇒ 保持/生成树`、`flag==0 ⇒ 标记删除`；`flag=1` 同时出现在「界外」分支——即「界外放置也保留」。此处语义（界外为何算有效）未定。
- `PlacementDataSsbo::position()` 返回 x/z 语义、`+0x1c` 等字段名未知。
- `AL`（`extraout_AL_00`，与 rock 结果相或）来源不明，疑为另一布尔参数。
- 6 个栅格各自对应哪个 raster（water/garden/path 细分）未逐一确认。
- `insert::<InteractableForestTree>` 的 `uVar3`（prefab 索引？）/`uVar48` 字段语义未定。
- 本函数**不解析 prefab 资源本身**（schema 属 asset 域，符合任务卡点）。

## 2.2 `forest::manage_tree_prefabs::is_too_close_to_shape`（源行 211，内联 helper）

**地址**：**无独立地址**。记录形式 `inline_instance, inlinee_declaration, instances=2`（`inlinee=97574`），被内联进宿主 `manage_tree_prefabs`（module 2492, inline_record=6748）。所以只能从宿主内联点（`0x141bf1d6a`–`0x141bf1dea`，墙邻近判定处）恢复其逻辑（O）。

**内部逻辑（O，从宿主内联区恢复）** —— 纯几何谓词「某点是否离某 shape 太近」，两路：

```
// 路径 A（shape 是矩形，[RDI]&1 == 1）：先算最近点，再算距离
CALL Rectangle2d::closest_point_on_perimeter(wall_rect, point_xz)   // 0x140c9ae20
d = (closest - point); dist2 = dot(d,d); dist = sqrt(dist2)          // SUBPS/MULPS/MOVSHDUP/ADDSS/SQRTSS
radius = 7.0                                                        // MOVSS [0x142a7cd48]
// 路径 B（shape 是点，[RDI]&1 == 0）：
d = shape_pos([RDI+4] as Vec2) - point; dist2 = dot(d,d); dist = sqrt(dist2)
radius = shape_radius([RDI+0xc]) + 7.0
// 汇合：UCOMISS radius, dist  → 返回 dist <= radius（即 point 距 shape <= 半径+7.0）
```

- 依赖（O，内联证据）：`glam::Vec2::distance`(×2)、`Vec2::sub`、`Vec2::dot`、`f32::sqrt`。与「`point.distance(shape_pos) <= shape_radius`」等价；`instances=2` 对应上面两条距离计算。
- 被 `closure_env_0`（墙查询回调，见 2.1 第 3 步）调用，用来判定树放置点是否落在墙的 `半径+7.0` 缓冲区内。

**算法**：欧氏距离（`sqrt(dot(sub,sub))`）+ 半径比较（「距离阈值」判定）。

**数据结构 / 常量 / 字段偏移（O）**
- 常量 `0x142a7cd48 = 7.0`（额外缓冲半径）。
- shape 判别位 `[RDI]&1`（1=矩形走 `Rectangle2d::closest_point_on_perimeter`，0=点）。
- 点 shape：`[+0x4]` = Vec2 位置，`[+0xc]` = f32 半径。

**未知（U）**
- 参数名/返回类型（PDB 占位 `void()`）；推断为 `fn(point: Vec2, shape: &Shape) -> bool`，`Shape` 具体类型未定（可能是墙段/矩形）。
- `+7.0` 是常量边距还是墙半厚的补偿未定。
- 比较算子 `<=` / `<` 及 NaN 语义（`UCOMISS` 组合）未逐位确认。

---

# 3. `src/tree_animation.rs`

## 3.1 `forest::tree_animation::handplaced_tree_animation`  @ `0x142279500`

**系统签名（O/I，PDB 泛型拼写）**

```rust
fn handplaced_tree_animation(
    query: Query<(&mut TreeAnimation, &mut PrefabInstanceData,
                  Option<&TreeMarkedForDeletion>)>,
    game_time: Res<GameTime>,
    app_mode: Res<AppMode>,
    main_camera: Res<MainCamera>,
    audio_trigger_library: Res<AudioTriggerLibrary>,
    ui_signifier_stream: Res<UiSignifierStream>,
    tree_canopy_max_cull_distance: ResMut<TreeCanopyMaxCullDistance>,
)
```

**内部逻辑（O，逐段）**

1. **`match app_mode`**：`[app_mode]` 作为 u32 判别值走跳转表（`0x143038050`，判别值范围 6..=20，15 路；`0x1422795e3`–`0x142279608`）。不同 AppMode 生长累加方向不同。
2. **生长/淡出累加**（`0x142279812` 起）：`XMM0 = GameTime[+0x28]`（**delta 秒**，O）累加到某进度字段（`ADDSS`/`SUBSS` 取决于 AppMode）；`progress = clamp(progress, 0.0, 1.0)`（常量 `0x142925904 = 1.0`）；**smoothstep** `fade = t*t*(3.0 - 2.0*t)`（O，常量 `0x1429cd078 = 3.0`）写入 `[RDX+4]`（与 `utils::smoothstep` 内联一致）。
3. **相机射线**：`MainCamera::eye_position()`（`0x140c96d40`）与 `eye_direction()`（`0x140c96d80`）取眼点/朝向，构造一条线段（`0x14157ce90`/`0x14157ce40`），对 `AudioTriggerLibrary::get_triggers()`（`0x14157ce90`）→ `AudioTriggerCollection::boxes()`（`0x14157ce40`）返回的触发盒逐个做 `Aabb3::intersect_segment`（`0x141240570`，O）。命中即与音频触发相关（树叶声等）。
4. **逐树循环**（`0x142279a30` 起）：遍历 `query`：
   - 从 archetype 位图判定是否带 `TreeMarkedForDeletion`（`[RBP+0xb0]`）。
   - 带删除标记：`step = delta / (-0.4)`（常量 `0x142a7fcb0 = -0.4`，O）→ 进度下降。
   - 否则：若 `[RCX+0xc]==0` 用 `step = delta / [RCX]`（每树自定时长），否则 `step = delta / (-0.4)`。
   - `progress = clamp(progress + step, 0, 1)` 写回；第二个值按 `± delta` 累加/减并 clamp。
   - `utils::pack_2x16f_uint`（`0x140c90df0`）打包两个 f16 写入输出（O）。
5. `should_remove_tree_canopy_from_view`（源行 152）被内联，决定树冠是否从视图剔除。

**算法**：smoothstep 曲线（`t²(3-2t)`）；clamp；视锥/射线剔除（相机眼点+朝向线段 × 触发盒 `Aabb3::intersect_segment`）；f16 打包。

**数据结构 / 常量 / 字段偏移（O）**
- **`TreeAnimation` 组件元素 stride = 0x10**（O：`SHL RCX,0x4; ADD RCX,[RBP+0x90]` @ `0x142279cc9`），字段 `[+0]=f32`、`[+4]=f32 进度`、`[+8]=f32 第二值`、`[+0xc]=u8 标志`。
- **`PrefabInstanceData` 组件元素 stride = 0x30**（O：`SHL RDI,0x4; ADD RDI,[RBP+0x98]` @ `0x142279cd4`，其中 `RDI=3*index`）。
- `GameTime.delta` 在 **`+0x28`**（本模块两动画函数共用，O）。
- 常量：`1.0`(0x142925904)、`3.0`(0x1429cd078)、`-0.4`(0x142a7fcb0)、`1.5/0.63`(0x1429258d0)、`-1.0`(0x14295a0e4)。
- 跳转表 `0x143038050`（AppMode 判别 6..20）。

**未知（U）**
- `TreeAnimation` 组件字段**名**（步长 0x10 已确认，见「跨模块核对」）；字段语义（哪个是进度/尺寸/标志）未定。
- `AppMode` 15 路分支各自语义未逐一还原。
- 触发盒命中后的具体效果（是否写信号/触发音频事件）未定——`ui_signifier_stream` 参数用途未观察到明确写入。
- `should_remove_tree_canopy_from_view` 判定条件（阈值/距离来源）未单独恢复（已内联）；`TreeCanopyMaxCullDistance` 如何参与未确认。
- 相机线段几何构造（乘的常量 `0x142b2da40/0x142ad3650/0x14292e6c0/0x143037e10/0x142ab2370`）未完全还原。
- `-0.4`/`-0.3`（见 3.4）时间尺度语义（每秒衰减率？）未定。

## 3.2 `forest::tree_animation::handplaced_tree_animation::closure$0`（源行 74）

- 内联闭包（`inlinee=121211`）。
- 依赖（O）：`Option::is_some`、`glam::Vec3::add`、`add_assign`、`sub_assign`。→ 对某 `Option<Vec3>` 做向量累加/相减（疑为相机线段端点或位置增量）。
- 未知（U）：捕获变量与循环中的角色。

## 3.3 `forest::tree_animation::should_remove_tree_canopy_from_view`（源行 152）

- 被 3.1 内联（`inlinee=121209`），无独立函数体可反编译；依赖注解为空。
- 未知（U）：判定逻辑（纯谓词：距离/可见性）；需从宿主内联点恢复。

## 3.4 `forest::tree_animation::forest_tree_animation`  @ `0x14227a0d0`

**系统签名（O/I，PDB 泛型拼写）**

```rust
fn forest_tree_animation(
    query: Query<(Entity, &mut InteractableForestTree, &mut PrefabInstanceData,
                  Option<&TreeMarkedForDeletion>)>,
    game_time: Res<GameTime>,
    despawn_prefab_cmd: EventWriter<DespawnPrefabCmd>,
    interactable_forest_placements: ResMut<InteractableForestPlacements>,
)
```

**内部逻辑（O，逐段）** 逐树循环（`0x14227a5xx`–`0x14227a6xx`）：

```
delta = GameTime[+0x28]                        // 0x14227a624/0x14227a65b，delta 秒
if 带 TreeMarkedForDeletion:                   // [RBP+0x70] bit
    step = delta / -0.3                        // 常量 0x142925900 = -0.3（DIVSS XMM1,XMM6）
    growing = false
else:
    timer = [R15+0x10]                          // 每树延迟计时器
    if timer > 0.0:
        timer -= delta; [R15+0x10] = timer      // 递减
        step = 0.0
    else:
        step = delta + delta                    // 2*delta（timer<=0 时加速）
    growing = true

progress = clamp(progress + step, 0.0, 1.0)     // MAXSS 0 / MINSS 1.0
[R15+0xc] = progress                            // 写回进度
if progress != 旧值:
    eased = EasingFunction::apply(progress)     // 0x140c95ef0
    output = eased * [R15+0x8]                  // [R15+0x8] = 幅度 amplitude
    [输出数组 + i*0x30 + 0xc] = output           // 0x14227a6c7

if (growing && progress == 0.0):               // 衰减到 0
    EventWriter::<DespawnPrefabCmd>::write(实体) // 0x14227a6f0 起，事件元素 stride 0x10
    InteractableForestPlacements[entity][+0x44] = 0  // 清「有树」标志（元素 stride 0x50）
```

- **`InteractableForestTree` 组件元素 stride = 0x18**（O：`LEA RDI,[RCX+RCX*0x2]; LEA R15,[RAX+RDI*0x8]` @ `0x14227a5e4`），字段 `[+0x8]=f32 幅度`、`[+0xc]=f32 进度(0..1)`、`[+0x10]=f32 延迟计时器`。
- **`PrefabInstanceData` 组件元素 stride = 0x30**（O：`SHL RDI,0x4; ADD RDI,[RBP+0x50]` @ `0x14227a5f8`，其中 `RDI=3*index`），写字段 `+0xc`。
- 查询迭代游标数组 stride 0x10（`[RBP+0x18]`，元素 = {ptr:8B, index:u32@+8}），非组件本身。

**算法**：一阶线性推进 + clamp 到 [0,1] 的生长/收缩动画；带**延迟计时器**（timer>0 暂停生长，≤0 以 2×delta 加速）；施加 `utils::easing_functions::EasingFunction::apply` 缓动曲线乘幅度得输出；衰减到 0 发 `DespawnPrefabCmd` 并清放置标志。

**数据结构 / 常量 / 字段偏移（O）**
- 常量（`0x142925900` 起连续）：`-0.3`、`1.0`、`0.6`、`0.0`。本函数用 `-0.3`（衰减速率）与 `1.0`（clamp 上界）。
- `GameTime.delta` 在 `+0x28`。
- `DespawnPrefabCmd` 事件元素 stride **0x10**。
- `InteractableForestPlacements` 元素 stride **0x50**，`+0x44` = 有树标志字节。
- puffin 文件名：`crates/systems/forest/src/tree_animation.rs` @ `0x143038000`。

**未知（U）**
- 缓动曲线具体变体：`EasingFunction::apply` 的 `self` 由 `LEA RCX,[0x14303802c]` 给出，该静态首字节 `0x06`（疑为 EasingFunction 枚举判别值 ≈6）；具体曲线名未定。
- 每树 `+0x8` 幅度来源（由谁写入）未在本函数内确定（可能来自 prefab 实例数据或 manage_tree_prefabs）。
- 延迟计时器 `[+0x10]` 初值来源未知。
- 输出数组所属组件 = `PrefabInstanceData`（步长 0x30 已确认），字段语义未知。
- `0.6`（`0x142925908`）在本函数未被引用，属相邻常量（归属其它函数）。

---

# 逻辑总览

`forest` crate 只做三件事，全部围绕 country-core 的 `InteractableForestPlacements` / `TreeStorage` 权威数据（本模块只消费，不定义）：

1. **放置→实体（`manage_tree_prefabs`）**：每帧扫描森林树放置点，用世界栅格（water/garden/path）+ rock mask + 墙体空间哈希判定该点是否适合长树；适合的生成 `InteractableForestTree` + `TreeAmbience` 实体并发 `SpawnPrefabCmd`，不适合的打 `TreeMarkedForDeletion`。
2. **动画**：
   - 手放树（`handplaced_tree_animation`）：按 AppMode 推进淡出/生长进度 → smoothstep → 相机射线与音频触发盒求交 → f16 打包输出（树冠剔除/淡出）。
   - 森林树（`forest_tree_animation`）：一阶生长/收缩动画（带延迟计时器）+ 缓动曲线 → 输出幅度；衰减到 0 发 `DespawnPrefabCmd` 并清标志。
3. **注册（`build`）**：把上述系统 chain 后放进命名 SystemSet，加到 `Update`，并 `insert_resource(InteractableForestPlacements)`。

依赖边界：prefab 资源 schema、`TreeStorage`/组件定义、`GameTime`/`MainCamera`/`AudioTriggerLibrary`/栅格与 SSBO 均在 core 或 asset 域（本模块只消费）。

# 跨模块核对：TreeAnimation 步长 0x10 vs 0x18（forest 侧核实）

复核提出 tree-tool 报 `TreeAnimation` 步长 0x10、本报告曾报 0x18。forest 侧核实结论：**两者并不冲突，指的是不同组件**（O，原指令）：

| 组件 | 步长 | 证据（forest 侧原指令） |
|---|---|---|
| `TreeAnimation` | **0x10** | `handplaced_tree_animation`：`SHL RCX,0x4; ADD RCX,[RBP+0x90]` @ `0x142279cc9`（`RCX` 为元素基址，字段 +0/+4/+8/+0xc） |
| `InteractableForestTree` | **0x18** | `forest_tree_animation`：`LEA RDI,[RCX+RCX*0x2]; LEA R15,[RAX+RDI*0x8]` @ `0x14227a5e4`（`R15` 为元素基址，字段 +0x8/+0xc/+0x10） |
| `PrefabInstanceData` | **0x30** | 两函数：`SHL RDI,0x4; ADD RDI,[RBP+0x98/0x50]`（`RDI=3*index`） |

- tree-tool 报的 **0x10 = `TreeAnimation`**，与本模块 `handplaced_tree_animation` 一致（正确）。
- 本报告先前的 0x18 实为 **`InteractableForestTree`**（`forest_tree_animation` 的查询组件），非 `TreeAnimation`；先前措辞把二者混为一谈，现已改正。
- 另注：查询迭代游标数组步长为 0x10（`{ptr,index}`），勿与 `TreeAnimation` 0x10 混淆。

# 算法清单

| 算法 | 位置 | 说明 | 已知对应 |
|---|---|---|---|
| 世界栅格双线性采样 | `manage_tree_prefabs` | water/garden/path，6 次 `bilinear_sample_at_world_xz` | `utils::float_grid`（O） |
| rock mask 采样 + clamp 求和 | `manage_tree_prefabs` | `TerrainRockMaskSsbo::sample_at_world_xz` ×4 求和 clamp | — |
| 墙体邻近圆查询 | `manage_tree_prefabs` | `Circle2d` + `WallSpatialHash::query_with_circle_callback` | — |
| 点-形状距离阈值判定 | `manage_tree_prefabs` 内联 `is_too_close_to_shape` | `sqrt(dot(sub,sub))` + 半径(+7.0) 比较；矩形走 `closest_point_on_perimeter` | glam `Vec2::distance/dot/sub`（O） |
| 确定性 RNG 初始化 + 随机插值 | `manage_tree_prefabs` | `utils::rng::init_rng` + `fastrand`，`r*0.75+(1-r)*0.3` | — |
| f16 双值打包 | 两动画 + 管理 | `utils::pack_2x16f_uint` | — |
| smoothstep | `handplaced_tree_animation` | `t²(3-2t)`（内联 `utils::smoothstep`） | 标准 smoothstep（O） |
| 射线-盒求交 | `handplaced_tree_animation` | 相机线段 × `AudioTriggerCollection::boxes`，`Aabb3::intersect_segment` | — |
| 一阶生长 + clamp + 延迟计时器 | `forest_tree_animation` | 线性推进、`[0,1]` clamp、timer 门控、2×delta 加速 | — |
| 缓动曲线 | `forest_tree_animation` | `utils::easing_functions::EasingFunction::apply` | 通用缓动（变体未知） |
| 风/季节曲线 | — | **未观察到** | 无（动画由 delta + 缓动 + 事件驱动） |

> 注：任务描述提到「风/季节曲线」，但两动画函数的原指令证据中未见风力场/季节相位；进度由 `GameTime.delta` 线性推进，衰减速率是常量（-0.3 / -0.4 / 每树自定），并非正弦风场。若原作风场在 shader/其它系统，则不在本 crate 内。

# 未知清单（汇总）

1. `build` 注册的系统顺序与完整清单（`add_systems` 元组被内联）；SystemSet 名称/类型（1 字节自定义值 0x24）。
2. `manage_tree_prefabs`：`MemCache` 布局；`flag` 的「界外=保留」语义；`PlacementDataSsbo::position()` 与各 `+0x1c` 字段名；`AL` 布尔来源；6 个栅格与 water/garden/path 的逐一对应；`insert::<InteractableForestTree>` 的 `uVar3`/`uVar48` 字段。
2b. `is_too_close_to_shape`（内联 helper）：无独立地址；参数/返回类型未知（推断 `fn(point: Vec2, shape: &Shape) -> bool`），`Shape` 具体类型、`+7.0` 边距含义、比较算子（`<=`/`<`）未定。
3. `handplaced_tree_animation`：`TreeAnimation` 组件完整布局（已确认步长 0x10，字段名仍缺）；AppMode 15 路分支语义；相机线段几何构造与常量含义；`should_remove_tree_canopy_from_view` 判定条件；`TreeCanopyMaxCullDistance` 用法；触发盒命中后的实际效果；`closure$0` 捕获变量。
4. `forest_tree_animation`：缓动曲线变体（静态首字节 0x06）；每树幅度/计时器初值来源；输出数组所属组件（`PrefabInstanceData`，步长 0x30 已确认）字段名；`-0.3`/`-0.4` 时间尺度语义。
5. 风/季节：本 crate 无显式风/季节算法，若存在应在别处（shader/其它 crate）。

# 附：可确定的实现草案（不写入工程）

## `build`（胶水，逻辑完全确定，仅系统清单未知）

```rust
impl Plugin for ForestPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InteractableForestPlacements::default()); // O
        app.add_systems(
            Update,
            (manage_tree_prefabs, handplaced_tree_animation, forest_tree_animation)
                .chain()                       // O: chain_ignore_deferred
                .in_set(/* ForestSystems */),  // O: intern<dyn SystemSet> + in_set
        );
    }
}
```

## `forest_tree_animation`（算法已确定，字段名待定）

```rust
fn forest_tree_animation(
    mut q: Query<(Entity, &mut InteractableForestTree, &mut PrefabInstanceData,
                  Option<&TreeMarkedForDeletion>)>,
    time: Res<GameTime>,
    mut despawn: EventWriter<DespawnPrefabCmd>,
    mut placements: ResMut<InteractableForestPlacements>,
) {
    let dt = time.delta_secs(); // GameTime[+0x28]
    for (e, _tree, _prefab, marked) in &mut q {
        let a = /* per-tree anim state (stride 0x18) */;
        let (step, growing) = if marked.is_some() {
            (dt / -0.3, false)
        } else if a.timer > 0.0 {
            a.timer -= dt;
            (0.0, true)
        } else {
            (dt * 2.0, true)
        };
        let old = a.progress;
        a.progress = (a.progress + step).clamp(0.0, 1.0);
        if a.progress != old {
            let eased = EasingFunction::/*variant*/().apply(a.progress);
            /* output */ = eased * a.amplitude; // [out + i*0x30 + 0xc]
        }
        if growing && a.progress == 0.0 {
            despawn.write(DespawnPrefabCmd { /* entity */ });
            /* placements[e][+0x44] = 0 */;
        }
    }
}
```
（缓动变体、`TreeAnimation`/输出结构字段名、`InteractableForestTree` 与 `PrefabInstanceData` 的字段归属仍为 unknown，故草案不完整。）
