#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""Append reviewer-B verdict sections to the three groupB REVIEW.md files."""
import io
import os

BASE = os.path.dirname(os.path.abspath(__file__))

CRITTERS = """

## 审查判定明细（2026-10-16，tg_vB1）

### F 项（与机器码矛盾/未实现，需 restorer-B 修复）

1. **F1 Sheep::being_petted 常量错误**（`src/components.rs`）：用了 `0x3c23d70a`（0.01），
   机器码 0x14089d4c0 = `movss [rcx+0x58]; ucomiss [rip+0x20c31a8]; seta al; ret` →
   常量地址 **0x142930f74 = 0x0000292f**（≈1.48e-41 正 denormal，即"显著为正"）。
   RNE 字节已双重验证（函数体 + 0x142930f74）。core `resources/critters/sheep.rs` 已用
   `f32::from_bits(0x292f)`（正确）；restorer 与 reconstruction 草稿/review-audit 章节的
   "0.01"同源错误。**修复 + 向中央索引登记纠错**（章节 critters/sheep.rs.md 与
   review-audit claims-collision-eco.tsv 的 0.01 均错）。
2. **F2 boids frame_update 缺加速度增量 clamp**（`src/boids.rs`）：机器码（Boid_frame_update_fish
   decompiled.c）`fVar22 = 1.0; 水>=0.5 && 有目标 → 1.5` 后
   `clamp(delta, -max, max/2)`（`-1.0*max` / `max*0.5` 双边界 + clamp::do_panic 断言）。
   reconstruction 草稿注释承认未实现，restorer 照抄残缺。fish/wren 两实例同缺。
3. **F3 sim_wren_boids 大缺口**（`src/wren_boids.rs`）：closure_3 可行点判定（RAY_DIRS
   32 方向 raycast，0x141dc42f0）用纯几何 `is_viable_landing_spot` 替代——无 raycast；
   FreeSpot 收集（L700）/ BirdHaven 树查询（L745）/ 树摇出生（L1231，"Tree has been
   shaken" 日志）/ 音频全缺；反射转向为自造随机水平方向（机器码 from_rotation_arc 转
   RAY_DIRS 方向）。
4. **F4 wren.rs 三函数为语义壳**：update_decorator_bird_spots / update_roof_bird_spots
   （机器码 Rectangle2d::get_edge_pts + roof_noise::apply_roof_noise + Roof::height +
   calc_roof_tip_center_ls + random_splits + BirdSpotManager::recycle，restorer 用宿主
   RoofBirdSpotHost 组件替换且 recycle 未接线）/ update_wrens（hop 状态机/阈值全部自造，
   PlacementDataSsbo::to_affine3a/音频/DebugLine 缺失）。
5. **F5 fish 实体转向伪实现**（`src/fish.rs`）：`lerp(target.vel)` 的 target 取
   `all[self_idx]` = 自身 → 恒等空操作。机器码 L786-812 转向目标未钉（re-analyst 需补挖）。
   另 min/max 档位选择（`BackupSaveFileKind::is_corrupted()` 谓词）未接，恒 (0.9,4.0)。
6. **F6 sheep_floatie / sheep_umbrella 生成空壳**：机器码证据明确 `Commands::spawn_empty` +
   `AudioCore::instantiate_audio`（skeleton 依赖注解），restorer 只推进现有实体，
   spawn 部分 `let _ = ...; let _ = &mut commands;` 空置。
7. **F7 duck_splash_particle 新请求不 spawn**：`for event in events.read() { let _ = event; }`
   ——机器码 spawn_empty + AssetMeshLibrary/AssetShaderLibrary 取句柄（"duck_shader"）。

### Q 项（存疑/宿主依赖/需 re-analyst 补挖；不影响交付先行）

- Q1 DuckState 0：机器码（move_ducks decompiled）字节 0 走正向（dir=+1，
  progress>0.25 转 1）；`from_byte(0)->FreeSwim` 与 `move_ducks` 的 `to_byte()==2->skip、
  0->continue` 分支不一致（0 被跳过）。
- Q2 move_ducks 末端写缺失：机器码到末端写 `Duck+0xe0=1` + `+0xe4=CONCAT44(2.0f,4.0f)`
  （restorer 只置 moving=true）；且 despawn 分支缺 `_DEV_OVERRIDE_IS_DEMO_MODE` 门控
  （发行默认不直接 despawn，走事件路径）。
- Q3 update_duck_goals 新目标时 `Duck+0xe8=0.1` 写入缺失（dwell[1]=0.1，语义 U）。
- Q4 plan_duck spawn 组件集缺 DrawableMeshBundle/ObjectGrassPush（机器码 tuple 含）；
  轨迹 lerp/raycast 受阻分支为壳（raycast_hit 恒 None，与 core 壳一致）。
- Q5 RaycastWorldHandle 壳：`slot_query` 恒 None → legacy_spherecast 永不命中 → 避让/
  DespawnDuckCmd 实际不触发。与 core RaycastWorld native-physics 未落地一致（项目现状），
  但**core 侧尚缺 legacy_spherecast（10 槽序）API**——装配层需按 LEGACY_SPHERECAST_SLOT_ORDER
  实现真实槽查询；critters 侧球检未按 locomotion 位测门控（机器码 {0,1,5,6}=纯墙避让无
  spherecast）。
- Q6 update_firefly_goals 查询含 &Sheep 未在证据钉死；CritterControlState.enabled 默认
  false → firefly 目标链恒不跑（需宿主初始化 enabled=1，机器码 `*res+4==1`）。
- Q7 butterfly 目标偏置的 goal-active 维：机器码 uVar33 按 goal active 再选
  -0.2835/0.8529（update_butterfly_goals decompiled），restorer 只实现 state 维。
- Q8 sheep_behavior：共享刷新新计时用固定 [1.0,1.0]（机器码 2xuniform_f32 产出）；
  `fVar13` 阈值 1.0 自造；SheepFollowTarget 组件替代机器码 Res<MainCamera> mode==3 判定。
- Q9 sheep_unstuck 阈值（0.5/8 次/±0.5 偏移/h+4）全部自造（证据仅依赖注解）。
- Q10 ui_pet_* 3.0 距离阈值自造（机器码 UiWidgetContext::intersecting/is_active 交互；
  toolbar 批次 3 后核对）；pet_animation 五相常量源自 C 版 0x14293a350，B 侧 0x14231ac20
  未独立核验。
- Q11 采样工具本地复刻：uniform_sample_disk/circle/sphere/golden_spiral 应为 utils crate
  （证据 VA 0x140c90700 族属 utils）；x0y/x0z 与 `utils::swizzlers` 重复定义（依赖复用违规，
  应改 import）。fish 邻居 self 匹配用距离 <1e-4 hack；wren spawn 帧预算 2 自造；
  WATER_SAMPLE_OFFSETS/DUCK_WATER_SAMPLES 偏移集为推断；despawn_ducks 离场轨迹自造 +
  立即 despawn（机器码推计划由 plan_duck 执行）。
- Q12 build 命名：`configure(world, schedule)` 替代 impl$0::build 0x14067d440 / sheep
  build 0x14067d810（项目装配约定内，登记；系统注册链序与 machine build 的 chain/in_set
  未逐项对机器码）。
- Q13 组件重定义：systems 重定义 Duck/Sheep/Butterfly/Firefly/Fish/Wren/BirdSpots/Velocity/
  CritterVelocityTarget/Multiplier/Locomotion，其中 **core 已有 `Sheep` 组件**
  （resources/critters/sheep.rs，TypeId 不同）→ ECS 装配时同名牌冲突风险，integration
  owner 裁决（建议 systems 复用 core Sheep 或 core 扩展）。

### 合并登记（等价合并，非缺失）

- BoidFlock::remove x2（fish/wren 0x141dc2340/0x141dc2540）→ `flock_remove` 单泛型；
- Boid::frame_update x2（0x141dc2740/0x141dc3510）→ `frame_update` 单泛型（调用点常量
  已逐项 RNE 抽验一致，除 F2 增量 clamp）。
"""

CLUTTER = """

## 审查判定明细（2026-10-16，tg_vB1）

> 总体：结构骨架完整（37 业务均有落点、证据注释详实），但**机器码保真度低**——
> 大量"宿主缝/占位/自造常量"。判定分布：P≈8 / F≈11 / Q≈18。

### F 项（空壳/假实现/语义颠倒，需 restorer-B 修复）

1. **F1 AutoClutterState::mark_dirty 空壳**（0x140ee48c0）：`let _ = cell;` 无实现。
2. **F2 flush_candidates_pending_removal 只清表**（0x140ee4bf0）：注释自称"把 pending
   索引对应候选复制进 region_candidates"，代码仅 `clear()`。
3. **F3 populate_autoclutter_regions 骨架化**（44KB 机器码 → 192 行）：`clutter_kind_for_cell`
   恒返回 1（机器码 switch 种类分派缺失，==2 水位 0.01 分支虽写但 kind 永不为 2）；
   区域 center/basis 全零 + candidate_index=0（机器码 Aabb2::from_center_radius 真实几何）；
   `hash_cell` 用 DefaultHasher 冒充 `calculate_hash<IVec3>`（0x14145c1a0）；
   filter_clutter_object 的 iter_world_space_rect_cells/双采样链未复现；
   pick_clutter_mesh（L918）/align_to_nearby_wall（L944）闭包未实现。
4. **F4 manage_autoclutter_entities 增删语义颠倒**：机器码（draft 记录）新建=spawn_empty+
   insert<ColorId>；删除=insert<ClutterMarkedForDeletion> 标记（entity_command closure）。
   restorer 在**新建**实体上插 ClutterMarkedForDeletion、删除直接 despawn；
   更新路径（closure_0 0x1422d2d20、四元数朝向、init_rng 抖动、byte@+0x30 类型分支）未接线
   （orientation_basis/jitter_position 辅助函数未被系统调用）。
5. **F5 add_birdnests 输出 +0x08 用 String("birdnest")**：机器码该域为 **Arc 强计数**
   （draft 明注"birdnest 为 Arc 强计数域"）；String 使 0x68 stride 布局失真（lantern/tree
   的 +0x08 才是 String）。AutoClutterSpawn 用 String 表达全类型 +0x08 是布局错误。
6. **F6 process_autoclutter_candidates 比较器/几何占位**：`cmp_candidate` 按 index 比较
   （注释声称 +0x34 字段，代码未实现）；重叠矩形 `f38/f40 ± 2.0` 自造
   （机器码 Rectangle3d::overlaps_rectangle3d 真实几何）；filter 条件 `shared!=0` 自造。
7. **F7 autoclutter_subscribe_to_tabloids 空壳**：`if guard.is_none() { *guard = None; }`
   为恒等 no-op；机器码 0x1422d6930 有真实订阅逻辑。
8. **F8 spawn_stitches 位置计算假实现**：`x0y(Vec2::new(point, mid*0.0)) +
   Vec3::new(0.0, jitter*0.4, 0.0)` —— mid*0.0/jitter*0.4 为凑类型自造表达式；
   机器码 = 曲线法向归一 + x0y swizzle。
9. **F9 load_clutter_meshes 空资源**：仅 insert_resource(ClutterMeshes::default())，
   资产装载闭包（ClutterMeshTy 56 资产路径）未实现。
10. **F10 construct_inter_roof_merlons `wall_is_halftimbered` 恒 false**（假实现）；
    门控 `gate = !(halftimbered||flat_no_rim)` 与"半木墙才生成"的既有推断矛盾（Q 并存）。
11. **F11 全部 calculate_hash 用 hashbrown DefaultHashBuilder（随机种子）**冒充机器码
    固定哈希（0x141cf7440/0x141cf6d60/0x141cf7980/0x141cf7600/0x141cf70d0/0x14145c1a0）
    → 放置键跨进程不确定。hash_gate_key/hash_window_key/hash_lantern_key/hash_tree_key/
    hash_preplaced_key/hash_cell 六处同病。

### Q 项（细节自造/宿主依赖/证据不足）

- Q1 add_region 不等价路径 push `pending_indices.len()`（疑应为 region.candidate_index）；
  AutoClutterState Rust 字段序 ≠ 机器偏移（+0x20/+0x30/+0x48/+0x60/+0x90），内部使用可接受。
- Q2 add_autoclutter_on_windows：reflect 公式忠实（✓），但 +0x08 String("window") 自造；
  跳表 6 分支宿主源由 OnWindowInput 资源承载（机器码读系统内宿主查询）。
- Q3 add_lantern_water_setdresssing：is_corrupted 门控缺失；is_started 偏移 0.3/0.6 自造；
  jitter 幅度 0.1 自造；fmodf 语义用 `% 1.0` 近似。
- Q4 add_tree_setdressing：is_startedx4/is_corruptedx4 门控简化为 2 个；Circle 半径 1.5、
  jitter 0.08 自造。
- Q5 autoclutter_animation：Easing/despawn 语义合理但细节（duration 来源）未钉。
- Q6 add_preplaced_autoclutter：Circle2d::new/new_rotated 未用（basis 由 rotation cos/sin
  自造）。
- Q7 read_bush_positions_from_gpu：GPU 回读后置（合规），CPU 侧 AABB 半径 8.0 自造、
  ipnsort 键自造。
- Q8 detect_inter_roof_merlons：dirty_walls 事件汇入 ✓；is_flat 门控 ✓；intersect_shapes
  为占位；WallHoles::add 由宿主缝。
- Q9 detect_intra_shape_corners：is_flatx2 门控 ✓；角点高 -0.67 语义未实现（host 缝）；
  `get_param`（0x141ce3430）与 `init_state`（0x1422e1b40）缺失。
- Q10 inter_roof_merlons_audio：事件输出合理，机器码细节未钉。
- Q11 lib 调度链：19 系统单链 `.chain()`，机器码 build 的 chain_ignore_deferred/in_set(0x24)/
  run_if 组合未复现；`configure` 命名替代 build 0x140652050（约定内登记）。
- Q12 serde_json 3 项（0x140057fd0/0x140057730/0x142890a70）为依赖误标 business，不计。
- Q13 AutoClutterCandidate/Spawn 的 f18/f20/f28/f34... 字段名多为占位（机器码 16 字段
  映射表已列，语义 U）；candidate.shared u64 无法表达 Arc 计数语义（F5 关联）。
"""

ONBOARDING = """

## 审查判定明细（2026-10-16，tg_vB1）

> 总体：60 业务全有结构覆盖（碎而浅特征明显），无既有重建可对照；判定分布：
> P≈20 / F≈4 / Q≈36。**最大风险 = 键码/触发条件/显示几何为自造推断**（均标 I 但未对
> 机器码）。toolbar 依赖（hint_draw/render_hints 的 TemplateView/MeiContext/NodeContext）
> 按批次 3 后置，UI 缝可接受。

### F 项（需 restorer-B 修复或转 re-analyst）

1. **F1 hint_highlight_shape 空壳**（0x141c50120）：只遍历不做事；机器码
   PublicWalls::roof_entity 目标形状高亮未实现。
2. **F2 hint_position_update 固定屏幕锚点**（0x141c50510）：0.9/0.1/0.5 屏幕分数替代
   机器码 world_space_to_screen_space 几何投影（ShapeParameters/Roof::height/pivot_3d/
   PublicWallState::max_y/CameraMatrices::eye_right/Rectangle2d/3d 消费全缺）。
3. **F3 键码/键帽串自造**：`keycode_to_str`（0x01=Esc、0x1d=Ctrl、0x2c=Z、0x38=Alt、
   0x1e=A、0x11=W）与 hint_* 的 Key(0x…) 均未对机器码（game_key_bindings::keycode_to_str
   与真实绑定树）；bevy KeyCode 判别值与 VK 码不一致（0x01 实为鼠标左键 VK）。
4. **F4 steam()=identity / resolve_steam_impl（0x14227e5e0）未实现**；cleanup_impl
   （0x14227f4a0）完全缺失。

### Q 项（宿主化/推断，需 toolbar 批次与 re-analyst 跟进）

- Q1 hint_* 触发条件宿主化：机器码读 DigitalActionState/AppMode::is_tool_selected/
  0x140eaaa50 按键判定 + make_focus_or_camera_binding_hint；restorer 用 HintInput 资源
  （cancel_pressed/undo_pressed/tool_selected 由宿主填）——行为不落地前依赖宿主。
- Q2 hint_camera_display_condition：机器码 0x1408c5680（工具选择判定），restorer 用
  back_off.remaining 检查替代。
- Q3 exponential_back_off：BACKOFF_BASE_S=0.5/2^n 封顶 6/16 槽全部自造（机器码 raw_vec
  + f32 语义，具体公式 U）。
- Q4 prompt_rollercoaster（0x14227bb70）：3.0s/5 提示轮播自造。
- Q5 hint_type_ordinal 显示序自造（机器码 driftsort 比较字段 U）。
- Q6 hint_deletion_system 冷却语义自造（remaining[ord]=2.0 复位）；hint_animation_system
  的 Easing 应用后丢弃（`let _eased`）。
- Q7 hint_orchestrator（结构较忠实 ✓：esc 门控/同型去重/spawn/lang-lib lookup 缝），但
  HintLocalization 宿主缝 + hint_label 键名自造（lang-lib 键应为机器码字符串池）。
- Q8 hints_startup 相机绑定树（MouseMiddle/LMB/RMB/Scroll 组合）键值自造；
  WelcomeScreenState 联动未实现（机器码排队初始提示以欢迎屏状态）。
- Q9 hint_camera_progress/hint_focused_edit/hint_clutter_focus_edit/hint_roof_shape：
  结构存在，触发/显示细节为推断（make_focus_or_camera_binding_hint 载荷自造）。
- Q10 register_required_componentsx12（0x141c53530..0x142282f40）与 alternate/adjacent/
  together x8 实例为胶水/合并（按 AGENTS.md 不计独立业务，登记合并）。
- Q11 `configure` 命名替代 add_onboarding 0x140650d50（约定内登记）；SubApp 装配
  （机器码证据 SubApp::add_systems + chain_ignore_deferred + in_set）未复现。
- Q12 raycast_seam：legacy_raycast_hit/raycast_hit 恒 None（与 core 壳一致，装配层落地）。

### 缺失登记（真实缺项）

- resolve_steam_impl（0x14227e5e0）、cleanup_impl（0x14227f4a0）——F4；
- clutter 侧 init_state（0x1422e1b40）、get_param（0x141ce3430）——见 clutter Q9。
"""

for fname, content in [('groupB/critters/REVIEW.md', CRITTERS),
                       ('groupB/clutter/REVIEW.md', CLUTTER),
                       ('groupB/onboarding/REVIEW.md', ONBOARDING)]:
    p = os.path.join(BASE, fname)
    with io.open(p, 'a', encoding='utf-8') as f:
        f.write(content)
    print('appended', fname)
