# system-clutter RE-ANALYSIS（组 B re-analyst，dbid tg_xb1，2026-10-16）

> 深挖范围：skeleton/B/crates/systems/clutter（37 业务）。新增反编译 35/35。
> 前置证据（复用）：`evidence/multi-baseline/clutter-placement-analysis.md`（populate/process/add_*/manage/AutoClutterState 布局）、
> `clutter-extra-analysis.md`（host_geometry 全 9 函数、on_windows 跳表、AutoClutterEdit 历史体系、process 字段映射）。
> 反编译落盘：`systems-recovery/groupB/decomp/`。O=机器码证实 / I=推断 / U=未知。

## 1. 本次新增深挖（前两轮未覆盖/未精读项）

| VA | PDB 名 | 结论 | 契约 |
|---|---|---|---|
| 0x1422c0760 | load_clutter_meshes | **O 新** | §2.1 杂物网格目录（约 50 项，逐条 (name,path,id)） |
| 0x1422ccef0 | autoclutter_animation | **O 新** | §2.2 摇摆动画系统 |
| 0x140eebaa0 | read_bush_positions_from_gpu | **O 新** | §2.3 GPU 灌木位置 → 脏格发布 |
| 0x1422d6930 | autoclutter_subscribe_to_tabloids | **O 新** | §2.4 订阅 8 个 tabloid 通道 + 资源访问器注册 |
| 0x1422e1b40 | populate::_::init_state | **O 部分** | §2.5 区域状态初始化（多个字段构造器） |
| 0x140ee48c0 | AutoClutterState::mark_dirty | **O 新** | §2.6 |
| 0x140ee4a70 | AutoClutterState::remove_region_from_lazy_grid | **O 新** | §2.7 |
| 0x140ee4bf0 | AutoClutterState::flush_candidates_pending_removal | **O 新** | §2.8 |
| 0x140ee4ee0 | AutoClutterState::register_required_components | 胶水 | Bevy required-component 注册（AGENTS.md 不计业务；详见 decomp） |
| 0x1422d4230 | request_inter_roof_merlon_audio | **O 部分** | §2.9 |
| 0x141cd16b0 | WallAndRoofChanges::dirty_walls | **O 复核** | 三 EventReader 汇脏墙集（extra-analysis 语义 + 本次布局补证） |
| 0x141ce3430 | detect_intra_shape_corners::get_param | **O 部分** | §2.10 |
| 0x140edd250 / 0x140edbf60 | intersect_shapes<closure_env$4> ×2 | O（extra-analysis 已覆盖） | 补充：两实例分别被 detect_inter_roof_merlons / detect_intra_shape_corners 消费 |
| 0x141cd3ce0 | add_hole_at_shape_intersection | O（extra-analysis 已覆盖） | — |
| 0x1422cd7d0 | add_autoclutter_on_windows | O（extra-analysis 跳表语义已钉） | 本次 Ghidra 新反编译仍截断（bad instruction data），**以 radare2 全函数反汇编为准** |

## 2. 契约

### 2.1 load_clutter_meshes @0x1422c0760（O）
- 参数 (param_1, param_2, param_3)；对**约 50 个网格**逐条调 `FUN_1422c1cc0(ctx, lib, name_ptr, name_len, path_ptr, path_len, mesh_id, …)`（7 组参数，后 4 位标志）。
- **网格目录**（.data 连续串表，`assets/meshes/clutter/<n>.json` + 库名 `clutter/<n>`）：
  plant_pot_v1, ladder, box_w_stuff, watering_can, birdnest, wheel, pitchfork, shovel, firewood, bucket,
  window_flowerpot, window_shutter, reed, lilypad, fallen_leaves, fallen_petals, olden_glade_rock,
  tree_log, tree_stump, bench, bench_w_basket, bench_short, barrel, barrel_w_food_basket, haystack,
  plant_pot_v2, birdhouse, basket, basket_w_food, potato_sack, road_sign, sunflower, cart, cart2,
  crates_w_tools, crates_w_flowers, pumpkin, pumpkin2, …（余项见 decomp-0x1422c0760.txt 串表；
  第二段 0x14303fe01 起为另一组（wet/季节?）网格）。
- mesh_id 序（首个参数表）：7,9,2,3,0,10,0xc,0xd,0xb,4,7,0xe,0x11(×4),0x1e,0x1c,0x1d,0x13(×2),5,0x17(×2),0x18,0x14,0x19(×2),0x15,0x16,7,5(×2),0x1a(×2),0x1b(×2),0x1f,0x20,0x21,0x22,0x23,0x24,0x25,2,5,1,0x26,0x1f(×2)…（与库内 ClutterMeshTy 枚举序对应，供 restorer 对表）。
- 收尾：`FUN_141cea830`/`FUN_1422cbde0`（库注册收尾）。

### 2.2 autoclutter_animation @0x1422ccef0（O）
- 查询：(Transform@0x30 [pfVar50], 0x18 槽动画状态记录 [lVar45], 4B 记录, Option<0x20 槽记录> [pfVar49])；param_3=Res<Time>（dt@+0x28）；param_4=ResMut（+0x1c tick）。
- 每杂物：
  - 计时：`记录+0x8 (f32)`：<=0 → `progress = dt / 间隔`（间隔 = 0x143042278 双 f32 表按 Option 有无取）；>0 → `-= dt`（到时 → 向 Vec（0x18 槽，param_4）push 新动画实例记录 `{u64 id, 8B?, byte 状态@+0x14}`，grow_one 0x18）。
  - 进度：`记录+0xc = clamp(记录+0xc + dt, 0, 1.0)`。
  - 姿态：`Transform.scale = curve(0x1430422d8)(t)`（无 Option 记录）；有记录 → `fVar52 = curve(0x1430422d0)(t)` 经权重
    （0x143042280/0x143042284/0x142aab4b0/0x142925870）与 **12 常量旋转基**（0x1429cd170..0x19c，3×4 四元数旋转矩阵）
    × sin/cos（0x142923fd0/0x142923e80）→ 写 `Transform.rotation`；scale=1.0。
  - **`progress == 0`（且非 NaN）→ EntityCommands::despawn**（0x140d14770，@0x1430422e0）。
- 语义：杂物摇曳/生长动画（周期推进，完成即移除），曲线求值 = 0x140c95ef0 同族。

### 2.3 read_bush_positions_from_gpu @0x140eebaa0（O）
- `FrozenMap::get`（0x141cf6a60）取灌木 SSBO buffer（版本串 0x5c 校验 + vtable）→ 读回 GPU 灌木位置表（u32 计数 + 8B/条）。
- 每条 `FUN_141240070`（解包）→ `FUN_141cf3ee0`（写入灌木 lazy grid，0x20 槽单元格）。
- 第二遍：逐格 `FUN_141cf7270` 哈希与格内版本 u32（+0x18）比对；变化 → `func_0x000140833250(i)`（格→世界索引）置 0x200B 脏位集
  （`apuStack_258[i>>6] |= 1<<(i&0x3f)`）。
- 收尾：`FUN_141cf6500(grid+0x18, &dirty_set)` → **发布 DirtyTiles（0x200B 位集，populate_autoclutter_regions 同源消费）**。
- 即：**GPU 灌木位置 → CPU 灌木格 → 脏格发布**，衔接 populate 主链（GPU 后置项的前置 CPU 侧）。

### 2.4 autoclutter_subscribe_to_tabloids @0x1422d6930（O）
- `func_0x0001408cdc20(uVar8, 0,1,2,3,4,5,6,7)` — **8 个 tabloid 通道订阅**；每通道 `FUN_140562870(plVar3, &UNK_143043488)`（grow_one 注册表）。
- 随后逐资源（param_3..7）：`FUN_141cf68e0(*p+0x18)`（格引用）、`func_0x00014067dd80(*p)`（WaterRaster as_inner_mut）等 ×4 + grow_one（UNK_143043410/428/440/458/470 注册表项）。
- 即：把订阅句柄与资源访问器注册进 AutoClutterState 侧表（系统启动接线）。

### 2.5 populate::_::impl$0::init_state @0x1422e1b40（O 部分）
- `FUN_141cb69b0(param_2, param_3)` + `FUN_140ae5b20` + `FUN_141cbda70` + `FUN_141cb7d00` + `FUN_141cba840` + `FUN_141cbdcc0` + `FUN_141cb9f80` + `FUN_141ccde40(param_2, param_3+0xb0)` + `FUN_141cd09a0` — 区域状态子字段构造（hashbrown/集合族，字段语义 U）；供 populate 的 init_state 闭包路径。

### 2.6 AutoClutterState::mark_dirty @0x140ee48c0（O）
- `(self, cell: IVec3)`：`FUN_1428d9518(cell, self+0x10 值, …)`（哈希插入：cell → 值，0x10 为 epoch/计数）；
  `FUN_1405b0d70(self+0x80, …)`（hash_one）；`FUN_140d54320(self+0x30, …)`（grow_one<u16>，候选待处理索引 push）。
- 即：脏格集合（self+0x80）+ 待处理索引列表（self+0x30）登记；panic UNK_142b98018。

### 2.7 AutoClutterState::remove_region_from_lazy_grid @0x140ee4a70（O 部分）
- `func_0x000140c9cdc0`（矩形/Aabb 构）+ `FUN_140c98980`（区域 rect 扩展）+ `func_0x000140c9b680` + `FUN_141cf3540` — 从 lazy grid 移除区域覆盖的格（几何语义 U，工具函数族）。

### 2.8 AutoClutterState::flush_candidates_pending_removal @0x140ee4bf0（O）
- 对候选 Vec（stride 0x98）做 memmove 右移压缩（0x142923c10）+ `func_0x000140b5d130`（哈希表清）+ 按索引表删除
  （FUN_1428d9518 删除、panic UNK_142b97ef0/142b98048/142b98060 bounds-check）；
- `FUN_141cf41b0(puVar10, uVar9, &plStack_f0)` 收尾。即：**把"待移除"候选从 0x98 候选缓冲物理移除**（与 populate 2 处调用点一致）。

### 2.9 request_inter_roof_merlon_audio @0x1422d4230（O 部分）
- 门控 `*param_1 == 0xd`（状态值）；`func_0x000140a12650` = **CursorTerrainRaycast::position**（光标世界位）。
- push 0x28 槽音频请求记录进 Vec（grow_one 0x28）：`{u64, pos 3×f32, 4×f32}`，kind 字节 8（或 9）；
- 随后按 param_5 Vec（0x34 槽）做**去重压缩拷贝**（逐条 memmove，跳过与当前请求同位项，count clamp）——语义：同位置请求合并。I（部分）。

### 2.10 detect_intra_shape_corners::get_param @0x141ce3430（O 部分）
- 反编译含 hashbrown 表操作与形状参数读取；语义按 extra-analysis 的 detect_intra_shape_corners 主链消费（该文件内辅助 getter）。

## 3. reconstruction/clutter 对齐审计（DISPATCH 要求）

`reconstruction/clutter/src/**` 现状：**骨架/占位为主，未对齐 B 机器码**：
- `autoclutter/populate_autoclutter_regions.rs`：参数面骨架 + `TODO(evidence): 44KB 主体按 PDB 分段逐段反汇编复现` —— 主体未实现。
- `autoclutter/process_autoclutter_candidates.rs`（93 行）、`manage_autoclutter_entities.rs`（56 行）、`add_rules.rs`（172 行）：均以注释登记证据为主。
- `merlons.rs`/`stitches.rs`/`auto_clutter_edit.rs`：结构草稿。
- **缺失文件**（restorer 需新建）：add_autoclutter_around_gates.rs、add_autoclutter_around_windows.rs、add_autoclutter_on_windows.rs、
  add_birdnests.rs、add_lantern_water_setdresssing.rs、add_preplaced_autoclutter.rs、add_tree_vegetation_setdressing.rs、
  autoclutter_animation.rs、load_clutter_meshes.rs、read_bush_positions_from_gpu.rs、inter_roof_merlons/（detect/construct/audio）、
  inter_shape_stitches/（detect_intra_shape_corners/stitch_bricks）。
- **对齐结论**：clutter 链的权威证据已在 multi-baseline 两份文档 + 本分析中齐备（含 AutoClutterState 布局、
  0x98/0x68/0x50 候选记录字段映射、add_* 常量与键、manage 实体命令、host_geometry 全语义）；restorer 可直接按证据落地，无需再取证。

## 4. 跨领域线索

- `func_0x0001408cdc20(uVar8, 0..7)` = tabloid `TabloidHub::subscribe`（8 通道，与 session/collision 领域共用——登记中央索引）。
- `func_0x00014067dd80` = WaterRaster as_inner_mut（+0x298 双缓冲），多处复用。
- `FUN_140960ac0` = DirtyTiles::new_clear；`FUN_140c95ef0` = Curve1D 求值（0x1430422d0/2d8 = autoclutter 动画曲线资产）。
- `func_0x000140833250` = 格→世界索引（BushGrid 族，与 read_bush 共用）；`FUN_141cf6a60/141cf3ee0/141cf6500` = FrozenMap/格操作族。
- 0x140833170（spawns_clutter）/0x140b26970（subtype）/0x140a1aa20（transform）为窗组件访问器（country-core 侧）。

## 5. 仍 U（不臆造）

- load_clutter_meshes 第二段网格组（0x14303fe01 起，mesh_id 0x11..0x26）精确资源归属（wet/seasonal?）。
- init_state 子字段构造器（0x141cb* 族）具体语义；autoclutter_animation 的 0x18 槽记录字段（id/byte@+0x14 状态）完整布局。
- request_inter_roof_merlon_audio 的 0x34 槽去重压缩语义细节（I 级）。
- on_windows 的 RBP-0x60 位置槽疑点（extra-analysis ①，保持登记）。
- get_param 的具体 getter 语义（hashbrown 表字段）。
