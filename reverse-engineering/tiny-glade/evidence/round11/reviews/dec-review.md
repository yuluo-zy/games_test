# 第 11 轮 DEC 组复核（`crates/systems/decorator`，B 版）

> 复核角色：只读 reviewer。复核对象为工作树 `D:/game/tiny-glade/crates/systems/decorator/**`。
>
> **文档结构：§0–§9 = 首轮复核（2026-10-19 16:04 快照，历史记录）；§10–§11 = 增量复评
> （2026-10-19 18:56 快照，含 DEC 交付）。当前结论以 §10 为准。**
>
> 首轮快照：最后代码修改 15:54；交付文件当时不存在，故首轮只能核对代码与注释声明。
> 增量快照：最后代码修改 18:49；交付 `evidence/round11/deliveries/dec-decorator-DELIVERY.md`
> 已存在（18:37 版）。

---

## 0. 结论（首轮 16:04 快照；当前结论见 §10）

**不通过（NOT PASS）**。理由按严重度：

1. **P0 编译门（已实测失败）**：`cargo check -p system-decorator` 在 15:29 失败于 E0283
   （`register_ui_systems` 未给 `ui_change_stair_height::<C>` 指定类型）；DEC② 15:35 修复后，
   15:43 启动的隔离 target `cargo test -p system-decorator --lib`（16:03 完成）**再次失败**，
   报 3 处 E0614：`decorator_visual/lantern.rs:258/263/268` 对已是 `u32` 的 `id` 解引用
   （`collect(*id, info)`）。即 **本快照 `system-decorator` 不能编译**，`tiny-glade` 亦不可能
   通过（依赖它）。
2. **P0 运行门**：即使编译绿，当前注册路径存在三处必崩：
   `ui_place_decorator` 以空指针构造 `TransientDecoratorIdGen` 并调用 `generate()`（空指针解引用）；
   `ui_change_chimney_height` 释放相位向 `apply_decorator_op` 传 `None` archivist + 临时 Default
   事件（assert/unwrap 必 panic）；`ui_change_chimney_height`/`ui_move_decorator`/`ui_stairs_viz`
   所需的 `ChimneyVisuals`/`ChimneyPreviewState`/`MoveDecoratorDeltaPositions`/`InsertPointState`
   从未 `init_resource`（Bevy 资源缺失 panic）。
3. **P0 生命周期未接线**：宿主 `crates/tiny-glade/src/decorator_plugin.rs` 的 `build()` 仍是
   `todo!()`，并明确声明“不转发 `add_decorator_systems`”。也就是说 crate 内的注册函数在真实
   App 中没有消费者；装载→放置→附着→变换→可视化→历史→删除的顺序既未注册也未排序
   （crate 内无一处 `.chain()/.before()/.after()`）。
4. **P0 62 空壳**：按契约 §5.1 判据，62 项中只有 3 项真实实现、3 项部分、1 项经机器码证实
   原作不可达、4 项按契约删除/统一，**51 项仍是空壳**；楼梯装配、差分重装、老虎窗/门窗、
   蓝图 6 真缺失函数全部未闭合。
5. **P1 水分**：仍有约 16 类“声称已恢复/返回伪造默认值/日志冒充写入”的实现（§2）。

---

## 1. 水分检查（计数：16 类，约 30 处）

判据：契约 §5.1 禁止的 `Vec::new()/0/false/None/Vec3::ZERO/默认构造` 冒充业务输出，以及
“只 log/只写蓝图/只构造 builder 不 build”。

| # | 位置 | 现象 | 判定 |
|---|---|---|---|
| W1 | `decorator_visual/stairs_assemble.rs:33-49` | 注释称 `determine_ladder_support` “O，capdis 全文”，函数 `let _=(a,b); 0` | 声称全文、实际常量 0 |
| W2 | `stairs_assemble.rs:51-57` | 注释称 `structural_graph_subscribe` “订阅回调注册”，函数空体 | 空体 |
| W3 | `stairs_assemble.rs:59-74` | 注释称 kind==0x11 → `remove_collider`，实际只有 `let _ = _world;` | 空体 |
| W4 | `stairs_assemble.rs:76-85` | `stairs_subscribe_lazy` 注释“惰性订阅”，空体 | 空体 |
| W5 | `load_decorators.rs:84-97` | `load_collision_as_bbx` 读 `bbx` 后 `let _ = bbx; Some(RaycastMeshData::default())` | **伪造输出**（应为 `new_cuboid(bbx)`） |
| W6 | `cache_decorator_transforms.rs:78-86, 115` | 墙分支 `translation = Vec3::ZERO`（应为墙曲线坐标+附着高） | **伪造变换** |
| W7 | `cache_decorator_transforms.rs:131` | 屋顶分支 `get_position_from_mesh_uv(Vec2::ZERO, 0.0)`（uv 未算） | **伪造变换** |
| W8 | `decorator_visual/{cottage_wall_window,cottage_corner_window,cottage_balcony_door,gothic_balcony_door,gothic_dormer,gothic_wall_window,lantern,decorator_clutter,trap_doors,arrow_slit,chimney_*}.rs` | 逐文件 `WindowVisualEntityBuilder::new(Vec3::ZERO, …)` 后 `let _ = &builder; let _ = &mut transient;`，从不 `build/build_transient` | 系统性“只写蓝图不产几何”（≥11 文件） |
| W9 | `utils/mod.rs:122-131` | `new_cuboid_collider` 返回 `RaycastMeshData::default()`；`clutter_mesh_ty_data` 恒 0 | 伪造碰撞/类型 |
| W10 | `remove_ivy_under_windows.rs:83-84` | 窗盒 `width=1.0, height=3.0` 默认值（注释 I/U） | 默认值冒充分支 |
| W11 | `decorator_visual/trap_doors.rs:56-75` | `pos = Vec3::ZERO`、`set_colliders(Vec::new())`、洞载荷全 0 | 伪造输出 |
| W12 | `cull_oob_decorators.rs:120-122, 370-381` | `is_wall_decorator_location_valid` 恒 true；dormer 变体 7/8 恒 true | 谓词恒真 |
| W13 | `decorator_visual/cottage_balcony_door.rs:88-92`、`gothic_balcony_door.rs:84-88` | `is_balcony_door` 恒 true（注释 U） | 谓词恒真 |
| W14 | `ui_systems/ui_place_decorator.rs:69` | `TransientDecoratorIdGen::new(std::ptr::null_mut(), false)` + `generate()`（非直写路径解引用 base） | **运行期空指针崩溃** |
| W15 | `ui_systems/ui_change_chimney_height.rs:95-103,152-154` | `apply_decorator_op(..., None::<…>, …, &Default::default())`：释放相位 `kind_byte=1` → assert 失败；即使 0 也在 `unwrap()` panic。`preview_ratio` 恒 0.0（契约 #55 明确给了公式） | **运行期 panic + 公式未落** |
| W16 | `apply_decorator_op.rs:174-181` | 非墙地址 `archive_decorator_state` 只 `log::warn!` 后 `return`（地形/楼梯的 place/change/remove 归档被静默跳过） | 日志冒充写入 |

另：`_tracked_surface` / `_stairs_backup_surface` / `_wall_id_surface` / `_circle_position_write`
等 no-op surface（`apply_decorator_op.rs:482`、`backup.rs:187`、`restore_archived_decorators.rs:152`、
`decorator_on_freehand_wall_edited.rs:97`）本身不算业务水分，但会掩盖“函数存在”的假象。

### 1.1 伪造 VA / 字段核查

- 抽查未发现伪造 VA：`set_interaction_mesh 0x1408255e0`、`add_door_stairs 0x140825830`、
  `determine_segment_type 0x14080c810`、`0x140c9cde0` 等与证据一致。
- **契约 §2.0 自身有一处 VA 笔误**：`ui_change_stair_height` 跳表 idx0/idx3 的目标实际是
  `0x1407ae26f`，不是 `0x1407ade6f`（差 0x400）。见 §4.1 的机器字节证据。
- 字段面：`DecoratorOp.dst` 是新引入的 `Option`，同时 `info.dst_secondary=3` 表示哨兵，
  同一“+0x10 第二定位器”被建模两次；`build_stair_height_op` 把存储判别（0/1/2）写进 `op.kind`
  （op.kind 语义是 3=place/4=remove/其余 change），属字段语义混用，需修。

---

## 2. 62 个空壳逐项现状

分类：**R**=真实实现（按 §5.1 有业务产物）；**P**=部分；**D**=按契约删除/统一；
**U0**=原作不可达（实现正确但非业务）；**E**=仍空壳。

### 2.1 楼梯视觉 `stairs.rs`（7 项，全部 E）

| # | 位置 | 函数 | 现状 |
|---|---|---|---|
| 18 | `stairs.rs:78` | `find_alignment_preferences` | E（`Vec::new()`） |
| 19 | `stairs.rs:89` | `find_best_neighbor_alignments` | E（`None`） |
| 20 | `stairs.rs:99` | `generate_playermade_stair_decorators` | E（空体） |
| 21 | `stairs.rs:110` | `generate_playermade_stair_requests<const>` | E（空体） |
| 22 | `stairs.rs:116` | `walk_shape` | E（`Vec::new()`） |
| 23 | `stairs.rs:124` | `maintain_stair_graph` | E（空体） |
| 24 | `stairs.rs:134` | `compute_stair_assembly_params` | E（`Vec::new()`） |

（`determine_segment_type`/`densely_sample_spline_segment`/`derive_stair_node_params` 不在 62 内；
前者为 R，后两者为部分/常量。）

### 2.2 楼梯装配 `stairs_assemble.rs`（16 项，全部 E）

| # | 位置 | 函数 | 现状 |
|---|---|---|---|
| 25 | `:52` | `structural_graph_subscribe` | E |
| 26 | `:78` | `stairs_subscribe_lazy` | E |
| 27 | `:89` | `calculate_adjusted_ladder_positions` | E |
| 28 | `:103` | `construct_ladder` | E |
| 29 | `:111` | `playermade_stairs_assemble<const>` | E（硬 U，见 §5） |
| 30 | `:116` | `playermade_stairs_interpolate_and_compare` | E（恒 false） |
| 31 | `:124` | `playermade_stairs_assembly_propagate_dirty_flags` | E |
| 32 | `:131` | `playermade_stairs_add_wall_holes` | E |
| 33 | `:139` | `create_structural_colliders_and_initial_graph` | E |
| 34 | `:148` | `calculate_distortion_from_structural_graph` | E |
| 35 | `:155` | `split_segment_into_supported_and_unsupported_subsegments` | E |
| 36 | `:162` | `construct_stair_steps` | E |
| 37 | `:169` | `gen_stair_floors` | E |
| 38 | `:177` | `gen_stairs_collider_and_rt_geo` | E |
| 39 | `:185` | `gen_stairs_structural_collider` | E |
| 40 | `:192` | `floor_tri_count_to_atlas_idx` | E（恒 0） |

### 2.3 楼梯扶手（4 项，全部 E）

| # | 位置 | 函数 | 现状 |
|---|---|---|---|
| 41 | `stairs_railings_assemble.rs:20` | `assemble_railings` | E |
| 42 | `:30` | `railings_tips` | E |
| 43 | `:48` | `construct_wooden_railing` | E |
| 44 | `:62` | `add_corner_stone_railing_piece` | E |

### 2.4 差分/灯笼/历史/装载/合并/杂项

| # | 位置 | 函数 | 现状 |
|---|---|---|---|
| 9 | `deco_differential_state.rs:104` | `chimney_reassemble` | E |
| 10 | `deco_differential_state.rs:115` | `door_stairs_reassemble` | E |
| 11 | `deco_differential_state.rs:125` | `compare_assembly_params` | E（恒 false） |
| 12 | `flag_connections.rs:41` | `greedily_connect` | E |
| 13 | `flag_connections.rs:49` | `clothline` | E |
| 16 | `lantern.rs:227`（快照时已改） | `maintain_lantern_lights` | **P**：收集/排序/差分/双缓冲已落；记录 +4..+0xc 载荷 U、GPU `update_buffer_bytes` 未接 |
| 17 | `lantern.rs:95`（快照时已改） | `compare_light_lists` | **R**：归并扫描 old/new 0x1cB 记录并分类，含测试 |
| 45 | `deferred_decorator_history.rs:175` | `record_to_history_edit` | **P**：映射到 `DecoratorV5Edit{kind,fields[3]}`；原生 48B `{1,1,kind,f1,f2,f3}` 的两个前导 1 无法承载，未证明恒真 |
| 46 | `derive_decorator_subtype.rs:85` | `is_half_dormer` | E（恒 false；契约给了完整分支） |
| 47 | `load_decorators.rs:85` | `load_collision_as_bbx` | E（伪造 default，W5） |
| 48 | `merge_proposal_roof_decorators.rs:33` | `find_best_roof_decorator_merge_position` | E（None） |
| 49 | `merge_proposal_roof_decorators.rs` | 本地重复 `try_merge_decorators` | **D**（已删除，统一走 lib.rs） |
| 50 | `merge_proposal_wall_decorators.rs:57` | `find_best_wall_decorator_merge_position` | E（None） |
| 51 | `merge_proposal_wall_decorators.rs` | 本地重复 `snapping_direction` | **D**（已删除） |
| 52 | `replay_decorator.rs:90` | `decorator_cmd_of` | **R**（快照时已改）：`ReplayCommand::Decorator` 真接线；`bucket.remove_command` 压缩 timing；cull_allow take；含测试 |
| 61 | `utils/mod.rs:128` | `clutter_mesh_ty_data` | E（恒 0；契约要求定点反编译 0x1408b8e90） |
| 62 | `window_lights.rs` | `rng_seed` | **D**（快照时已删除）：改用全局 `fastrand::f32()`，`timer=2*rng` |

### 2.5 老虎窗/阳台门/窗/UI 处理器

| # | 位置 | 函数 | 现状 |
|---|---|---|---|
| 1 | `blueprint.rs:80` | `DecoratorBlueprint::new` | **R**（trivial：`Default`） |
| 2 | `blueprint.rs:138` | `brick_source` | E（None） |
| 3 | `cottage_balcony_door.rs:68` | `add_door_autoclutter` | E |
| 4 | `cottage_dormers/assemble_dormer_roof_tiles.rs:22` | `dormer_roof_height` | E（0.0） |
| 5 | `…:27` | `is_bay_window` | E（false） |
| 6 | `cottage_dormers/collision_geo.rs:30` | `get_collision_mesh` | E（`Vec::new()`） |
| 7 | `cottage_dormers/cottage_dormer.rs:76` | `get_dormer_panel_xforms` | E |
| 8 | `cottage_wall_window.rs:116` | `add_lintels` | E |
| 14 | `gothic_balcony_door.rs:25` | `check_for_door_stairs` | E（恒 0） |
| 15 | `gothic_wall_window.rs:311` | `get_window_holes` | E（`Vec::new()`） |
| 53 | `handle_splitting_roof_decorators.rs:68` | `handle_moving_out_roof_decorator_part` | E（空体） |
| 54 | `handle_splitting_wall_decorators.rs:63` | `handle_moving_out_wall_decorator_part` | E（空体） |
| 55 | `ui_change_chimney_height.rs:151` | `preview_ratio` | E（恒 0.0；契约给公式） |
| 56 | `ui_change_stair_height.rs` | `change_stair_height_wall`（旧名） | **U0**：跳表 idx0/idx3 = `0x1407ae26f` = `unreachable!()`，不是墙/楼梯写回（契约语义需纠，见 §4.1） |
| 57 | `ui_change_stair_height.rs:195/213` | 屋顶/地形处理器 | **P**：屋顶 = `as_wall`→`PublicWalls::get`→`flat_roof_y`→缓存位置；地形 = 打包 xz→地形采样→xvy；op 构造已落，但依赖 core 字段适配且无 handler 级测试 |
| 58 | `ui_rotate_lantern.rs:73` | `rotate_wall_handler` | E |
| 59 | `ui_rotate_lantern.rs:101` | `rotate_stair_handler` | E |
| 60 | `ui_rotate_lantern.rs` | `dst_addr` | **D**（已删除） |

**统计**：R=3（#1,#17,#52）+ P=3（#16,#45,#57）+ U0=1（#56）+ D=4（#49,#51,#60,#62）
+ E=51。

> 五条重点链状态：**楼梯**（#18–#44，31 项）全部空；**老虎窗/阳台门**（#3–#8,#14,#15）全部空；
> **差分重装**（#9–#11）全部空；**灯笼灯光** #16/#17 已落 CPU 侧（GPU 待接）。

---

## 3. 生命周期顺序（实际注册/调用）

契约 §1 的六阶段在代码里**没有对应的执行顺序**：

| 阶段 | 契约系统 | 实际 |
|---|---|---|
| 0 装载 | `load_decorators`、`set_colliders`、`derive_decorator_subtypes` | `load_decorators` 只是普通函数（宿主注入 trait），无 startup 注册；`derive_decorator_subtypes` 未注册 |
| 1 放置/操作 | UI 5 系统 + `apply_decorator_op` + `handle_external_decorator_cmds` | UI 12 系统在 `register_ui_systems`（`ui_systems/mod.rs:91-114`）；`apply_decorator_op` 不是系统（仅被 stair/chimney 高度与 cull 内联调用）；`handle_external_decorator_cmds` 未注册 |
| 2 附着 | `decorator_on_*`、`move_decorators_following_anchors` | 全部未注册；`reanchor_one` 空体 |
| 3 变换缓存 | `cache_decorator_transforms` | 未注册 |
| 4 可视化 | `derive_subtypes` → clear → 差分重装 → generate_* → instantiate_* → seal | 仅 `clear_out_*` 注册；差分重装/generate/instantiate/derive 均未注册；蓝图 6 函数不存在；seal 无系统 |
| 5 历史/备份/剔除 | `create_or_restore`、`decorator_record_history`、`ui_decorator_inactive`、cull 族 | 前两者在 `add_decorator_systems`；inactive 在 UI 注册；cull 在独立 `register_cull_oob_systems` |
| 6 删除/回放/恢复 | `apply remove`、`replay_decorator`、`restore_archived_decorators` | `replay_decorator`/`restore_archived_decorators` 均未注册 |

并且：
- `add_decorator_systems`（`lib.rs:127-143`）只加 10 个系统，`register_ui_systems` 12 个，
  `register_cull_oob_systems` 4 个；三组之间及组内均无 `.chain()/.before()/.after()`
  （全 crate grep 为空）→ “清 → 差分 → 生成 → 实例化 → seal”的帧内顺序不受约束。
- **宿主未接线**：`crates/tiny-glade/src/decorator_plugin.rs:22-32` 的 `build()` 仍是 `todo!()`，
  注释明确“不转发 `add_decorator_systems`”。即三组注册函数在当前 App 中没有调用者。
- 资源初始化缺口：`ChimneyVisuals`、`ChimneyPreviewState`、`MoveDecoratorDeltaPositions`、
  `InsertPointState` 被已注册系统 `Res/ResMut` 消费，但 crate 内没有任何 `init_resource`
  （`add_decorator_events_and_resources` 只初始化了 8 个资源）→ 一旦注册即 panic。

结论：生命周期顺序既未注册完整，也未排序，且宿主层断开；契约 §1 的“可执行顺序契约”当前不成立。

---

## 4. 算法正确性抽查（8 项）

### 4.1 `ui_change_stair_height` 三处理器：**实现正确，契约地址有误**

机器字节（`workspace/input/tiny-glade.exe`，VA `0x142a832b0`）：
```
bfafd2fd 32add2fd b0add2fd bfafd2fd
→ idx0 = 0x1407ae26f, idx1 = 0x1407adfe2, idx2 = 0x1407ae060, idx3 = 0x1407ae26f
```
`0x1407ae26f` 的字节是 `lea rcx,[rip+…]`（"internal error: entered unreachable code"，
`0x142a83270`，长 0x28）+ `mov edx,0x28` + call → **idx0/idx3 是 `unreachable!()`**。
因此契约 §2.0 “idx0/idx3→`0x1407ade6f`（墙/楼梯共用）”既有 +0x400 地址笔误，也有语义错误。
（附注：`0x1407ade6f` 处确为真实指令——`b8 20 00 00 00` = `mov eax,0x20` 起的代码——但它是
邻近函数的内部标签，不是 `0x142a832b0` 的任何表项目标；表中 idx0/idx3 明确指向 panic 桩。）
当前 Rust 的 `tag 2→屋顶、tag 3→地形、其余 unreachable` 与跳表索引公式
`(tag-2)<3 ? tag-1 : 0` 一致。

处理器调用面复核（机器字节）：
- `0x1407adfe2`：`call 0x140a1c240`(as_wall) → `0x140a91ef0`(PublicWalls::get) →
  `0x140b2a6e0`(flat_roof_y) → `0x140a1a9f0`(position)，与 `roof_height_position` 一致；
- `0x1407ae060`：`call 0x140257ba0`(bilinear_sample) → `0x140c9cde0`(SwizzlerXVY::xvy，
  证据 `abc-building-35/pack-B/…asm.txt`：`movq %rcx,%rax; movss; …` 组装 Vec3) →
  `0x1408394b0`(as_stair)，与 `terrain_height_position`/`stair_height_of` 一致。

遗留风险：`Slider3dMatrices` 由 `camera.read_matrix(0x40/0xc0).inverse()` 构造，与注释
“second=view_to_clip/first=world_to_view”自相矛盾，未找到原调用面对照；释放相位的
`slider.offset` 依赖上一帧拖中状态；`build_stair_height_op` 把存储判别写进 `op.kind`。
三项都需要反编译对照 + 测试。

### 4.2 `restore_archived_decorators`（`restore_archived_decorators.rs:101`）

已落步骤 1–4（mark_mutable、逐存储清追踪、逐装饰 `get_or_insert_default + insert + d2s`、
`stairs.restore_backup`）。**步骤 5（colliders/interactions Arc 落位）与步骤 6
（`extract_diff` + 0x70B 冲刷）缺失**，注释已如实登记。状态字节取 `decorator_info.state==2`
（合同写“状态字节==2”，未证明二者同源）。判定：部分。

### 4.3 `apply_decorator_op`（`apply_decorator_op.rs:313`）

- 分派/归档/历史/存储/事件五段齐；change 二次归档改由 `op.dst: Option` 承载，语义正确。
- **非墙地址归档被 `log::warn` + `return` 跳过**（W16）：地形/楼梯的 place/change/remove
  会丢失归档，`restore_archived_decorators` 只遍历 archivist 桶 → 恢复不完整。
- `decorator_archivist.unwrap()` 无条件执行：`kind_byte==0` 时 assert 允许 None，随后仍 panic；
  若原机器码每分支确有无条件 unwrap，则语义成立，但注释与断言组合误导，应写清。
- `apply_decorator_change_field_update`：`disc = ((rank_byte).wrapping_sub(2).min(6)) | 4; if disc != 6 panic`
  的 `min(6)|4` 组合可疑（可达 4/5/7），需反编译逐字节核对。
- 判定：部分（核心正常路径可用；地形/楼梯归档缺口 + 判别式需复核）。

### 4.4 `window_lights`（`window_lights.rs:109`）

- 区间 `0.51/0.44 <= t < 0.72`、`lights_on` 写控制字节、`timer=2*fastrand::f32()`
  （15:45 已改为全局 fastrand，删除 `rng_seed`）、retain 在帧末推进之前——均与契约一致。
- 仍 U：retain 谓词是自拟的 `timer>0 || actual_on!=0`；`time_continuous_lerp` 的
  `F` thunk 用 `sin` 冒充；窗实体收集（墙/屋顶双模式桶）未实现（注释登记）。
- 判定：部分（关键公式正确，收集与插值函数 U）。

### 4.5 `playermade_stairs_assemble`（`stairs_assemble.rs:111`）

空体 `<const X: bool>()`；注释明确“反编译超时 2 次、U、re-analyst 补”。**硬阻塞属实**，
但当前空体违反契约“禁止空体冒充”，应按契约保持显式未实现并在交付中作为唯一硬 U。

### 4.6 `get_collision_mesh` / `load_collision_as_bbx`

- `get_collision_mesh` 返回 `Vec::new()`，DormerKey `round(dist*5)` 未落 → 老虎窗碰撞全缺。
- `load_collision_as_bbx` 返回 `RaycastMeshData::default()`，`Vec2==(1,0)` 断言/`new_cuboid`
  未落 → 伪造输出。判定：不通过。

### 4.7 `construct_ladder` / `compute_stair_assembly_params`

均为空；0.65 档距、0.4/-0.593 档高、0.25/2.5 坡度常量全部未落。判定：不通过。

### 4.8 `decorator_record_history` / `record_to_history_edit`

排空循环、kind 6 终止、无条件清空已落；但 `DecoratorV5Edit` 只有 `{kind, fields[3]}`，
原生 48B 的两个前导 1 无法表示，未证明恒真。判定：部分。

---

## 5. 跨 crate 缺口（7 项）

`country-core` 检索结果（`grep -rn "struct <name>"`）：
1. `PlayermadeStairsAssemblyRequests` — **core 无定义**，本 crate 也未定义。
2. `AdjustedStairEdge` — **core 无定义**。
3. `StairSubSegment` — **core 无定义**。
4. `OverhangSupportFinderCache` — **只在 wall-constructor 本地**
   （`construct_elevation_supports/find_overlap_segments.rs:14`），未上收/共享；DEC 的
   `add_lintels` 仍空，未消费。
5. `StairsLazySubscriber` / `StructuralGraphSubscription` — **core 无定义**。
6. `SampledStairsPoints` — **core 无定义**。
7. `DecoratorBlueprint` 0x140B TPI 布局 — crate 仍是消费面子集（`blueprint.rs`，
   `+0x50/+0x58/+0x60/+0x80/+0x120/+0x128/+0x130/+0x138` 多为 U），未与 core 碰撞 API 对齐。

**诚实登记情况**：代码注释对 1/2/3/5/6 有“core 缺口”文字，但没有中央交付文档；
7 项均未上报到 `evidence/round11`（deliveries 为空）。`playermade_stairs_assemble` 的
硬阻塞在代码注释中明确（“反编译超时 2 次”“U”），但没有交付文档可核对。判定：代码层
基本诚实，交付层缺失。

---

## 6. 编译与测试（实际输出）

### 6.1 `cargo check -p system-decorator`（2026-10-19 15:29 起，15:35 前快照）

```
Checking system-decorator v0.1.0 (D:\game\tiny-glade\crates\systems\decorator)
error[E0283]: type annotations needed
   --> crates\systems\decorator\src\ui_systems\mod.rs:112:9
    |
112 |         ui_change_stair_height::ui_change_stair_height,
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `C`
...
error: could not compile `system-decorator` (lib) due to 1 previous error; 3 warnings emitted
```
（另有 3 个 unused_mut warning：`op_confirm_audio.rs:132/141`、`replay_decorator.rs:88`。）

DEC② 于 15:35:16 把 `mod.rs` 改为 `ui_change_stair_height::<C>` 并给
`register_ui_systems<C>` 加泛型，E0283 已消除。但 15:48 新增的 `lantern.rs`
（#16/#17 实现）引入了新的编译错误。**最终实测（隔离 target，15:43–16:03，`--offline`
依赖已全部编译完）：**

```
   Compiling system-decorator v0.1.0 (D:\game\tiny-glade\crates\systems\decorator)
error[E0614]: type `u32` cannot be dereferenced
   --> crates\systems\decorator\src\decorator_visual\lantern.rs:258:21
    |
258 |             collect(*id, info);
    |                     ^^^ can't be dereferenced
error[E0614]: type `u32` cannot be dereferenced
   --> crates\systems\decorator\src\decorator_visual\lantern.rs:263:21
...
error[E0614]: type `u32` cannot be dereferenced
   --> crates\systems\decorator\src\decorator_visual\lantern.rs:268:21
...
warning: `system-decorator` (lib test) generated 2 warnings
error: could not compile `system-decorator` (lib test) due to 3 previous errors; 2 warnings emitted
```

（`iter_terrain_decorators`/`iter_stair_decorators` 的 `iter_modified()` 已返回 `(u32, &Info)`，
调用处不应再 `*id`；墙桶 `storage.iter()` 分支的 `*id` 是对的。）

其他运行：
- `cargo check -p system-decorator` 在主 target 重跑返回
  `Blocking waiting for file lock on build directory`，280s 超时（其他 agent 并发占用）；
  但隔离 target 的完整编译已经证明同一工作树失败。
- `cargo check -p tiny-glade`（15:35 启动）在复核截止时仍在等待/构建，无输出；由于
  `system-decorator` 编译失败，它不可能通过（“不得回退”门未通过）。

### 6.2 测试现状

- 工作树 `#[test]` 至 15:56 共 **26** 个：`apply_decorator_op.rs` 3、`backup.rs` 2、
  `cache_decorator_transforms.rs` 1、`cull_oob_decorators.rs` 3、`decorator_visual/lantern.rs` 4、
  `decorator_visual/stairs.rs` 1、`replay_decorator.rs` 2、`ui_systems/ui_change_stair_height.rs` 4、
  `utils/visual_entity_builder.rs` 4、`window_lights.rs` 2。
- 没有覆盖：楼梯任何函数、差分重装、老虎窗/门窗、蓝图 6 函数、生命周期注册顺序、
  `apply_decorator_op` 非墙归档、`ui_place_decorator`/`ui_change_chimney_height` 的运行期路径。

### 6.3 回归门

`cargo check -p tiny-glade` 未能在窗口内完成（见上）。宿主 `decorator_plugin::build`
仍是 `todo!()`，因此即使编译通过也不能视为装饰生命周期已接入。

---

## 7. 缺口继承与交付诚实性

- **交付文档缺失**：无法核对“交付声明”。代码注释总体按 O/I/U 标注，未发现把渲染后置项
  说成完成；但多处注释“已恢复/全文”与空体并存（W1–W4），属于注释层面的失真。
- **契约继承错误被 DEC② 发现并纠正**：`ui_change_stair_height` 的 0x400 地址笔误与
  墙/楼梯处理器语义，代码已按机器字节修正；应在交付中反向更正契约 §2.0。
- **缺口继承不完整**：`replay_decorator.rs` 早期注释称 core 无 `ReplayCommand::Decorator`
  等（复核开始时仍如此），实际 core 已闭合；DEC② 15:46 已更新。说明交付前需要一次
  “core 缺口全量重扫”，否则会继承过期缺口。
- `playermade_stairs_assemble` 硬阻塞明确，但未在交付文档中单列为“唯一硬 U”。

---

## 8. 必须修复清单

### P0（阻断验收）

1. **编译门**：先修 `lantern.rs:258/263/268` 的 `*id`（E0614），再在无并发 build 锁时重跑并记录
   `cargo check -p system-decorator`、`cargo check -p tiny-glade`、`cargo test -p system-decorator --lib`
   的真实输出（本复核实测：E0283 已修，但随后 E0614×3 仍使 crate 编译失败）。
2. **`ui_place_decorator` 空指针**：不得以 `null_mut()` 构造 `TransientDecoratorIdGen`；
   需要真实 base（原版 id 生成器资源/宿主）或显式未实现路径，禁止每帧解引用空指针。
3. **`ui_change_chimney_height`**：释放相位必须传真实 archivist 与真实
   `DeferredDecoratorOpEvents`；`preview_ratio` 按契约公式（`abs(Info+0x14 - (cursor+Info+0x24))/reference`）
   落地；`local.ratio` 目前从不赋值。
4. **资源初始化**：为 `ChimneyVisuals`/`ChimneyPreviewState`/`MoveDecoratorDeltaPositions`/
   `InsertPointState` 补 `init_resource`，否则注册即 panic。
5. **生命周期接线**：实现或明确登记宿主 `decorator_plugin::build`（0x140342c10）的注册面；
   给出 `add_decorator_systems`/`register_ui_systems`/`register_cull_oob_systems` 的 schedule+
   set 顺序（或在文档中声明未取证，禁止假装已接线）。
6. **蓝图 6 真缺失函数**：`clear_blueprints` / `instantiate_decorator_colliders` /
   `instantiate_blueprint_colliders` / `instantiate_blueprints` / `validate_blueprints` /
   `show_max_wall_y` / `copy_blueprints_to_prev` 全部尚未存在；可视化和剔除链因此无输出。
7. **非墙归档**：`archive_decorator_state` 对 Terrain/Stair 不得 `log::warn` 后返回；
   需 core 支持特殊桶或在本 crate 建立真实桶并接线。

### P1（功能闭合）

8. 楼梯 31 项（`stairs.rs` 7 + `stairs_assemble.rs` 16 + railings 4 + `maintain_stair_graph` 等）
   按契约 §6 顺序落地；`playermade_stairs_assemble` 先补 4 个闭包取证。
9. 差分重装三函数（含 40B 参数逐字段比较）。
10. 老虎窗/门窗/杂乱（#3–#8、#14、#15、#61）与 `get_collision_mesh`。
11. `load_collision_as_bbx` 真 `new_cuboid(bbx)`；`cache_decorator_transforms` 墙/屋顶公式；
    `remove_ivy` 尺寸查表；cull 有效性谓词。
12. `generate_*` 全部接 `WindowVisualEntityBuilder::build*/build_transient`，
    禁止 `Vec3::ZERO + let _ = &builder`。
13. 删除/统一残留 no-op surface；`apply_decorator_change_field_update` 的判别式需逐字节复核。
14. 补测试：楼梯常量/分段、差分变化项、蓝图生命周期、生命周期注册顺序、错误路径。

---

## 9. 给 ①（契约/中央）的下一步

1. **更正契约 §2.0**：`0x142a832b0` 的实际表项是
   `idx0=idx3=0x1407ae26f`（`unreachable!()` panic 桩），`idx1=0x1407adfe2`（屋顶），
   `idx2=0x1407ae060`（地形）；`0x1407ade6f` 不是表项目标（它是邻近函数内部标签）。
   删除“墙/楼梯共用 0x1407ade6f”的表述，并附 `0x142a832b0` 的 16 字节与
   `0x1407ae26f` 的 panic 串证据。
2. **补落盘反编译**：`0x1407adfe2`、`0x1407ae060` 目前不在 `groupD/ui_systems/decomp`；
   需补，以便 ② 的处理器实现可复核。
3. **交付纪律**：要求 DEC② 先交 `DEC-DELIVERY.md`（逐项 62 状态 + 水分清单 + 编译/测试原始输出 +
   7 跨 crate 缺口 + 硬阻塞），再进入复核；当前“无交付”状态下任何“完成”声明不可验收。
4. **顺序**：先把 P0（编译/运行/资源/生命周期/蓝图 6 函数）做完，再推进楼梯与几何；
   楼梯取证（4 闭包）可并行，但不得用空体占位。
5. **跨 crate**：将 7 项缺口一次性上报中央；特别确认 `OverhangSupportFinderCache`
   是否从 wall-constructor 上收，避免 DEC 重复定义。
6. **复核复跑**：请在无其他 agent 编译占用时运行三项 cargo 命令，把 stdout 原文贴入交付；
   本复核的 E0283 只代表 15:29 的快照。

---

## 10. 增量复评（2026-10-19 18:56，交付已更新）

> 交付：`evidence/round11/deliveries/dec-decorator-DELIVERY.md`（18:37 版，11533B）。
> 工作树最后修改 18:49；本节复核该快照。首轮 §0–§9 保留为 15:56 快照的历史记录。

### 10.1 结论更新

**DEC 自身 P0 编译/崩溃门：PASS；workspace `tiny-glade` 门被外部 `system-tree` 错误阻断；
功能验收：仍 NOT PASS。**

- `cargo check -p system-decorator`：**PASS**（独立 target `target-agent-dec-review`，
  exit 0，2m09s，4 warnings：`op_confirm_audio.rs:132/141` unused_mut、
  `backup.rs:222` / `restore_archived_decorators.rs:170` dead `LOG_TARGET`）。
- `cargo test -p system-decorator --lib`：**PASS：143 passed / 0 failed / 0 ignored**（exit 0）。
  （DEC 交付写 75 passed；复核时工作树测试已增至 143，实际全绿。）
- `cargo check -p tiny-glade`：**本快照失败，但错误不在 DEC**。独立 target 复跑在
  `crates/systems/tree-tool/src/convert_preplaced_trees.rs:104` 报 E0599
  （`PrefabNameRef::to_string` 缺 `Display`），失败 crate = `system-tree`；
  该文件 18:38 被其它工作流修改（DEC 自报 18:16 的 tiny-glade PASS 早于此修改）。
  `system-decorator` 本身在本次构建中先编译成功（4 warnings）。因此“不得回退”门
  当前未绿，但归因于 tree-tool，需其 owner 修复后重跑，不能记在 DEC 账上。
- 62 项口径：**R 4 / P 13 / D 4 / U0 1 / E 40**（见 §10.3），因此按契约 §5.1
  “真实输出 + 真实消费者”仍不满足整体验收；但首轮 P0 中的编译失败、三处必崩、
  伪造默认值已实质修复。

### 10.2 首轮 P0 逐项复核

| 首轮 P0 | 现状 | 证据 |
|---|---|---|
| 编译失败（E0283→E0614） | **闭合** | 独立 target check/test 全绿（§10.5）；`lantern.rs` 的 `*id` 已改 `id`，`<C>` turbofish 已补 |
| `ui_place_decorator` 空指针 | **闭合** | `ui_place_decorator.rs:42-78`：新增 `TransientDecoratorIdBase` 资源，`TransientDecoratorIdGen::new(&mut id_base.0, false)`；无 `null_mut()` |
| `ui_change_chimney_height` None archivist | **闭合** | `:48-108`：泛型 `C` + `ResMut<DecoratorArchivist<C>>` + `Res<DeferredDecoratorOpEvents>`；释放相位 `Some(&mut *archivist)` / `&events` |
| 缺资源 init | **闭合** | `ui_systems/mod.rs:94-108` 新增 6 个 `init_resource`；`lib.rs` 新增 `LanternLightLists`、`DecoratorStorage`、`CachedDecoratorTransforms`、`DerivedDecoratorSubtypes`、`DecoratorMeshLibrary`、`DeferredDecoratorOpEvents`、`Events<ExternalDecoratorCmd>` |
| 非墙归档 log 冒充 | **闭合** | `apply_decorator_op.rs:168-205`：地形→`special1`、楼梯→`special2` 真实 clone；无 log-skip |
| 伪造默认值（W5–W7、W9、W10） | **主要项闭合** | `load_collision_as_bbx`→None；`new_cuboid_collider`/`clutter_mesh_ty_data`→None/Option；墙/屋顶缓存→None；ivy 尺寸→`DecoratorLibKey` 查表（未命中 panic） |
| 重复 `try_merge_decorators`/`snapping_direction`/`dst_addr` | **闭合** | 已删除并统一 `lib.rs` |
| 生命周期接线 | **crate 侧闭合；宿主待用户决定** | `lib.rs:127-198` 已用 `.chain()/.after()` 串起 derive→cache→visual/audio→history→cull/replay；`restore_archived` 与 generate 族仍未注册（交付已声明）。`decorator_plugin::build`（宿主文件）已写入真实调用，但**无调用者**（全仓 grep 仅定义）；`run_game.rs:331` 仅调用 `build_startup_resources`（资源/事件，不挂系统）。按用户“不要接入宿主接线”的要求，**不判缺陷，记为待用户决定**。 |
| 蓝图 6 真缺失函数 | **部分闭合（仍缺 5）** | 已补 `clear_blueprints`、`copy_blueprints_to_prev`（`blueprint.rs:161-191`）、`overwrite_decorator_state_from_archivist`（`restore_archived_decorators.rs:132`）、`build_dormer_hole_grid`（`dormer_holes.rs:141`）；**仍不存在**：`instantiate_decorator_colliders`、`instantiate_blueprint_colliders`、`instantiate_blueprints`、`validate_blueprints`、`show_max_wall_y` |

### 10.3 62 项重新分类（按代码体 + 常量/分支，快照 18:49）

| 类别 | 数量 | 项目 |
|---|---|---|
| **R 真实** | **4** | #1 `blueprint::new`（消费者 = 各 generate 写入蓝图库）、#17 `compare_light_lists`（消费者 = `maintain_lantern_lights`）、#46 `is_half_dormer`（消费者 = `derive_decorator_subtype`）、#52 `decorator_cmd_of`/`replay_decorator`（crate 侧系统入口） |
| **P 部分** | **13** | #3 `add_door_autoclutter`（hash 回调为占位式）、#8 `add_lintels`（体真，调用方 `mesh_entry_height=None` 且丢弃输出）、#9/#10/#11 差分三函数（差分/回调真，砖列主体由宿主）、#12 `greedily_connect`（体真，遮挡谓词恒 false、输出被丢弃）、#13 `clothline`（体真，几何算完丢弃、不写瞬态）、#14 `check_for_door_stairs`（逻辑真，调用方 `wall_y=0.0` 默认）、#16 `maintain_lantern_lights`（CPU 真，GPU U）、#45 `record_to_history_edit`（前导 1 未承载）、#48 `find_best_roof_decorator_merge_position`（体真，无调用者）、#50 `find_best_wall_decorator_merge_position`（完整 `_in` 已实现，契约 4 参签名仍 log+None）、#57 屋顶/地形高度处理器（真实 op，依赖 core 字段适配） |
| **D 删除/统一** | **4** | #49、#51、#60、#62 |
| **U0 原作不可达** | **1** | #56（跳表 idx0/idx3 = `0x1407ae26f` `unreachable!()`） |
| **E 仍空壳** | **40** | #2、#4–#7、#15、#18–#44（楼梯视觉 7 + 装配 16 + 扶手 4）、#47、#53、#54、#55、#58、#59、#61 |

重点链：
- **楼梯视觉/装配/扶手 27 项**（#18–#44）全 E；`playermade_stairs_assemble` 硬 U 保持
  （4 闭包 0x140284b0/0x140275b0/0x140286d0/0x14027840 无落盘反编译）。
- **差分重装** #9–#11 升为 P（仅变化项 + 40B 比较 + 回调已落；砖列装配由宿主）。
- **旗帜/布绳** #12/#13 升为 P（贪心容量/窗口/采样/0.08 下垂已落；但两入口把几何算完丢弃，
  不写瞬态/实体，按 §5.1 无真实消费者）。
- **老虎窗/门窗**：#3/#8/#14 升 R/P；#4/#5/#6/#7/#15 仍 E（`dormer_roof_height`/
  `is_bay_window` 已改为显式 `None`，不再伪造 0/false）。
- **灯笼灯光** #16/#17：CPU 差分与归并已落，GPU `update_buffer_bytes` U。

### 10.4 水分复查（仍存 8 类，约 20 处）

已清除：伪造 `RaycastMeshData::default`、`Vec3::ZERO` 变换、`log` 冒充归档、固定 seed、
重复实现、spurious `dst_addr`。

仍存：

| # | 位置 | 现象 |
|---|---|---|
| R1 | 11 个 `generate_*` 文件（`arrow_slit.rs:119`、`cottage_balcony_door.rs:169`、`cottage_corner_window.rs:53`、`cottage_dormer.rs:117`、`cottage_wall_window.rs:231`、`decorator_clutter.rs:97`、`gothic_balcony_door.rs:163`、`gothic_dormer.rs:102`、`gothic_wall_window.rs:310`、`lantern.rs:217`、`trap_doors.rs:123`） | `WindowVisualEntityBuilder` 构造后 `let _ = &builder`，从不 `build/build_transient`；蓝图写入但实体/瞬态表为空。契约 §5.2 明确禁止“只写蓝图不产几何”。 |
| R2 | `ui_change_chimney_height.rs:143-145` | `preview_ratio` 仍恒 0.0；`local.ratio` 从未赋值（契约 #55 已给公式）。 |
| R3 | `generate_clothline.rs:60-79`、`flag_connections.rs:344-354` | `quads` 计算后 `let _ = quads`，不写瞬态/实体；布绳几何无消费者。 |
| R4 | `cottage_dormers/collision_geo.rs:100-101` | `get_collision_mesh` 返回 `Arc::clone(cache)`，miss 时构建 `Vec::new()` —— 空碰撞网格被当作成功输出（建议 Option）。 |
| R5 | `gothic_wall_window.rs:316-317` | `get_window_holes` 仍 `Vec::new() // U`。 |
| R6 | `merge_proposal_wall_decorators.rs:254-272` | 契约 4 参 `find_best_wall_decorator_merge_position` 对非空候选 `log::warn!` + None；真实实现移到了新 `_in` 签名。 |
| R7 | `flag_connections.rs:344` | `greedily_connect(&anchors, &params, |_,_,_| false)` —— 遮挡谓词恒 false（默认值冒充）。 |
| R8 | `cottage_balcony_door.rs:177-184`、`gothic_balcony_door.rs:168` | autoclutter 的 hash 回调是 `id*黄金比` 占位；`render_door_wall_y` 恒 0.0 作为 `check_for_door_stairs` 输入。 |

未发现新的伪造 VA。契约 §2.0 的 `0x1407ade6f` 仍应改为 `0x1407ae26f`（复核证据不变）。

### 10.5 编译/测试原始输出（独立 target `target-agent-dec-review`）

```
$ CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-dec-review cargo check -p system-decorator --offline
    Checking system-decorator v0.1.0 (D:\game\tiny-glade\crates\systems\decorator)
warning: `system-decorator` (lib) generated 4 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2m 09s
PIPE_EXIT=0

$ ... cargo test -p system-decorator --lib --offline
test result: ok. 143 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
PIPE_EXIT=0
```

`cargo check -p tiny-glade`：独立 target 复跑见 §11 补记。

### 10.6 剩余缺口（按优先级）

**P0（功能阻断，非编译）**
1. 蓝图生成链仍缺 5 函数：`instantiate_decorator_colliders`、`instantiate_blueprint_colliders`、
   `instantiate_blueprints`、`validate_blueprints`、`show_max_wall_y` → 碰撞 upsert/实体 seal
   无法闭环。
2. 生成族（`generate_*`）未注册进任何 schedule；且 11 个生成函数构造 builder 后丢弃。
3. 楼梯 27 项 + `playermade_stairs_assemble` 硬 U。

**P1**
4. `preview_ratio`/`get_window_holes`/`get_collision_mesh` 输出未闭合。
5. 差分砖列主体、灯笼 GPU 上传、`record_to_history_edit` 48B 前导常量。
6. 水分 R1/R3（丢弃几何）、R6/R7/R8（legacy shim / 恒 false / 占位 hash）。
7. `restore_archived_decorators` 未注册（历史游标守卫未闭合，交付已声明）。

**待用户决定**
8. 宿主 `decorator_plugin::build` 已写好真实调用但无调用者；`run_game.rs:331` 只调
   `build_startup_resources`（资源/事件）。按“不接入宿主接线”要求，本轮不判缺陷。

---

## 11. 复核命令结果补记

- `cargo check -p system-decorator --offline`（target-agent-dec-review）：**exit 0**，
  `Finished dev profile ... in 2m 09s`，4 warnings。
- `cargo test -p system-decorator --lib --offline`（同 target）：**143 passed / 0 failed**，
  exit 0。
- `cargo check -p tiny-glade --offline`（同 target）：**PIPE_EXIT=101**。
  失败点为 `crates/systems/tree-tool/src/convert_preplaced_trees.rs:104` E0599
  （`asset::prefab::PrefabNameRef` 无 `Display`/`ToString`），失败 crate `system-tree`；
  `system-decorator` 在同一构建中编译成功。该文件修改时间 18:38（非 DEC 写权）；
  因此本门失败不归因 DEC，但当前工作树不能宣称 `tiny-glade` 绿。

---

## 附：复核使用的关键证据

- 跳表：`workspace/input/tiny-glade.exe` VA `0x142a832b0` → `[0x1407ae26f, 0x1407adfe2, 0x1407ae060, 0x1407ae26f]`。
- `0x1407ae26f` 首指令 `48 8d 0d fa 4f 2d 02` → `0x142a83270` = "internal error: entered unreachable code"（0x28 字节）。
- 屋顶/地形处理器 call 目标（见 §4.1）。
- `0x140c9cde0` 反汇编：`movq %rcx,%rax; movss (%rdx),%xmm0; …` = SwizzlerXVY::xvy
  （`evidence/abc-building-35-2026-10-12/pack-B/f99ccfabf54b5b948f-0.asm.txt`）。
- core 类型：`DecoratorV5Edit{kind,fields:[u64;3]}`（`country-core/src/resources/history/decorator_edit.rs:818`）；
  `ReplayCommand::Decorator`（`replay_cmd_buffer.rs:284`）；`TransientDecoratorIdGen::generate`
  非直写路径解引用 `*base`（`resources/walls/decorator.rs:150-162`）。
- 宿主：`crates/tiny-glade/src/decorator_plugin.rs:22-32`（`todo!()`）。
