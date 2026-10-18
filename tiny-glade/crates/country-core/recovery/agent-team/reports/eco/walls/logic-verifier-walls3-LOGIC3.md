# LOGIC3 — walls 域逐函数 fresh 判等（logic-verifier-walls3，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`（含宏胶水分类 + 分歧检查表 8 条）、
> `planning/2026-10-17-recovery-sop.md`。本报告**只以本轮 fresh 反编译为准**：
> `reverse-operator.py ghidra --worker shared`，`fields=signature,decompiled_code,callees`；
> `void()`/反编译空体用 `disassemble_function` 定案；常量用 `read_memory`。
> fresh 证据落盘 `planning/impl-walls-verify4/decomp/`：
> `batch1..6.result.json`、`batch1b/2b/2c/2e/4b-disasm.result.json`、`batch1c/1d/2d/2f-read.result.json`。
> 未采信 restorer/reviewer 描述；已判等 VDA 按任务清单去重（LOGIC.md §2/§3、
> logic-verifier-walls2、restorer-walls-return×2、restorer-eraser 不重复）。

## 0. 覆盖与结论概览

- 本轮 fresh 反编译/反汇编 **~130 个** walls 域函数（含小函数 disasm 与常量读取），
  逐函数判等 **46**、判等-注意 **17**、未实现-需重做 **49**、胶水-非业务 **10**（另 8 个
  serde derive 由符号/derive 属性分类）。
- 重点覆盖：inner_walls 25 / inner_wall_state 14 / public_wall_state 12 /
  decorator_archivist 17 / wall_spatial_hash 20 / adjust_wall_height 10 /
  history_recording 1 / wall_state 5 / shapes 18 / decorator_subtype 3。
- **推翻/新发现的关键分歧**：
  1. `InnerWalls::load_state` 0x14085ad80 原生**重建 walls map + 填 changes.created +
     max_id=max(id)**；Rust 只打日志（**空实现**）。
  2. `get_cells_from_segment` 端点越界分支原生**推一个中点格**；Rust `segment_cells`
     越界直接返回空（add_segment/query_with_segment 同受影响）。
  3. `get_cells_from_rectangle2d` 原生逐格 `Rectangle2d::aabb + Aabb2::intersects` 筛格；
     Rust `rectangle_cells` 只按包围盒铺格（**超集**）。
  4. `PublicWallState::max_y_gable` 原生 gable 未就绪 **panic**；Rust 回退 max_y
     （panic-vs-return 反）。
  5. `adjust_wall_height_impl` / `edit_rectangle` 原生写
     `(DecoratorId, Vec<PreferredNodeAlignment>)` + `(DecoratorId,DecoratorId)→AlignKind`
     indexmap 记录；Rust 无。
  6. `edit_arrow` 门控原生 `tag != 2`（圆+矩形都发箭头）；Rust `tag == 0`（仅圆）。
  7. `snap_to_roof_ridge` 原生含屋顶脊几何/缓动/阈值全链；Rust 为恒返回 Some 的近似桩。
  8. `ui_modes edit_dims/edit_orientation` 的 widget/signifier 在 Rust 是空桩。
- 已核实与 Rust 对齐的底层布局（fresh disasm 定案）：
  InnerWalls = walls@0 / changes.created@0x30 / updated@0x60 / removed@0x90 / max_id@0xc0；
  WallState 0x70B 字段 +0x38 shape tag / +0x54 height / +0x58 terrain_floor / +0x5c arch_scalar /
  +0x60 plaster u32 / +0x64 arch_type / +0x66 freehand / +0x67 supports / +0x68 style。

## 1. 判等（fresh 逐条对应）

| VA | 函数 | Rust 位置 | 判等依据 |
|---|---|---|---|
| 0x140859f90 | InnerWalls::new | inner_walls.rs:69 | 4 map + max_id@0xc0=0（4×HashMap+3 set 初始化逐 qword 对应） |
| 0x14085a370 | InnerWalls::new_wall_w_params | inner_walls.rs:150 | max_id+1 → walls insert → created set insert；旧值 dealloc ✓ |
| 0x14085a4a0 | InnerWalls::new_wall_with_id | inner_walls.rs:164 | walls insert；旧行存在 → panic_fmt（Duplicate）；否则 created insert ✓ |
| 0x14085a550 | InnerWalls::reset_max_id | inner_walls.rs:82 | +0xc0 = 0 ✓ |
| 0x14085a560 | InnerWalls::get | inner_walls.rs:86 | walls 探针（stride -0x78）命中返回 / miss None ✓ |
| 0x14085a650 | InnerWalls::get_unwrapped | inner_walls.rs:90 | 同探针，miss → unwrap_failed ✓ |
| 0x14085a730 | InnerWalls::remove_all | inner_walls.rs:181 | 逐 walls 键 insert removed set + 清 walls 表（memset 0xff/len=0/growth 重算）✓ |
| 0x14085ad20 | InnerWalls::update_arch_height | inner_walls.rs:210 | 尾跳 _try_update_state<…>（closure 写 +0x5c）✓ |
| 0x14085ad40 | InnerWalls::update_arch_type | inner_walls.rs:255 | 尾跳 closure 写 +0x64 ✓ |
| 0x14085ad60 | InnerWalls::update_wall_state_height | inner_walls.rs:202 | 尾跳 closure 写 +0x54 ✓ |
| 0x140aa9610 | _try_update_state\<update_wall_state_enclosure\> | inner_walls.rs:220 | 探针 → updated set insert → 0x1cB shape 拷贝到 WallState+0x38 ✓ |
| 0x140aa9800 | _try_update_state\<update_supports\> | inner_walls.rs:261 | 探针 → updated insert → byte@+0x67 ✓ |
| 0x140aa99e0 | _try_update_state\<update_freehand_ty\> | inner_walls.rs:241 | 探针 → updated insert → byte@+0x66（disasm `[R14+R15-0xa]`=key+0x6e）✓ |
| 0x140aa9bc0 | _try_update_state\<update_wall_state\> | inner_walls.rs:198 | 探针 → updated insert → 整 0x70B WallState 拷贝 ✓ |
| 0x140aa9eb0 | _try_update_state\<update_arch_type\> | inner_walls.rs:255 | byte@+0x64 ✓ |
| 0x140aaa090 | _try_update_state\<update_wall_state_curve\> | inner_walls.rs:216 | 0x40B Curve2 拷贝到 WallState+0 ✓ |
| 0x140aaa350 | _try_update_state\<update_style\> | inner_walls.rs:237 | byte@+0x68 ✓ |
| 0x140aaa530 | _try_update_state\<update_plaster_peel\> | inner_walls.rs:247 | u32@+0x60（disasm MOVSS `[R14+R15-0x10]`=key+0x68）✓ |
| 0x140aaa710 | _try_update_state\<update_arch_height\> | inner_walls.rs:210 | f32@+0x5c ✓ |
| 0x140aaa8f0 | _try_update_state\<mark_as_non_enclosed\> | inner_walls.rs:228 | u32=2@+0x38（shape tag）✓ |
| 0x140aaaac0 | _try_update_state\<update_elevation\> | inner_walls.rs:206 | f32@+0x58 ✓ |
| 0x140aaaca0 | _try_update_state\<update_wall_state_height\> | inner_walls.rs:202 | f32@+0x54 ✓ |
| 0x1408c5690 | is_default_arch_height | inner_wall_state.rs:28 | CMPEQSS vs 0x40333333（read 0x142a8cc60=2.8），NaN 有序相等 ✓ |
| 0x1408c56b0 | WallState::get_enclosure_params | inner_wall_state.rs:140 | tag@+0x38<2 → Some(self+0x38) else None ✓ |
| 0x140833230 | ClutterVersion::is_initial（共享地址） | decorator.rs:216 | `CMP byte[RCX],0; SETZ` = byte==0 ✓（另两符号同址，见 §2） |
| 0x140b2a5d0 | PublicWallState::get_wall_shape | public_wall_state.rs:128 | tag@+0x258<2 返回 / 否则 unwrap_failed ✓ |
| 0x140b2a600 | try_get_wall_shape | public_wall_state.rs:125 | disasm `CMP [RCX+0x258],2; LEA RCX,+0x258; CMOVC RAX,RCX` ✓ |
| 0x140b2a620 | max_y | public_wall_state.rs:176 | flag@+0x108&1 → f32@+0x10c / 否则 panic（MISSING_HEIGHT）✓ |
| 0x140b2a6e0 | flat_roof_y | public_wall_state.rs:185 | max_y@+0x10c + (-0.79，read 0x142a7e320=0xbf4a3d71) ✓ |
| 0x140b2a740 | wall_space | public_wall_state.rs:133 | `tag!=2` 置 AL + f32@+0x28c ✓ |
| 0x140b2a760 | wall_space_length | public_wall_state.rs:136 | f32@+0x28c ✓ |
| 0x140b2a770 | u_space | public_wall_state.rs:139 | `tag!=2` + 常量 0x3f800000=1.0 ✓ |
| 0x140b2a790 | is_rectangle | public_wall_state.rs:154 | tag<2 且 tag&1 → 拷贝 axis/center/size 三 Vec2 ✓ |
| 0x140b2a7d0 | is_circle | public_wall_state.rs:147 | tag==0 → 拷贝 center/radius/rotation ✓ |
| 0x140b2a800 | corner_pos_and_normal_xz | public_wall_state.rs:161 | tag<2 && tag&1 → Rectangle2d::get_point + get_corner_normal ✓ |
| 0x140a1e190 | DecoratorStorage::wall_decorators | decorator_storage.rs:583 | per_wall 探针（stride 0x98）命中返回 / miss None ✓ |
| 0x140b2bbf0 | decorator_cull_allow_any | decorator_archivist.rs:47 | byte@+0x1e0=1 ✓ |
| 0x140b2bd90 | is_decorator_cull_allowed | decorator_archivist.rs:52 | allow_any 短路；否则 +0x30 set 探针命中 true ✓ |
| 0x1409f08f0 | get_cells_from_circle\<query\> | wall_spatial_hash.rs:181 | Circle2d::aabb → 世界界 clamp → floor 格区间逐格 push；Rust clamp_to_cell 循环等价 ✓ |
| 0x140aa5150 | WallSpatialHash::new | wall_spatial_hash.rs:41 | Vec 4096 格（0x1000）+ 4 空 map + dims{64,64}@+0x48 + extent{130,130}@+0x50 + next=0 ✓ |
| 0x140aa5c70 | cell_center | wall_spatial_hash.rs:271 | dims==0 panic；col=index%dims,row=index/dims；( coord+0.5)·cell−extent/2 ✓ |
| 0x140aa6520 | world_space_to_grid_coord_unclamped | wall_spatial_index.rs:12 | floor((extent/2+p)/(extent/dims)) ✓ |
| 0x140aa65d0 | grid_coord_to_cell_index | wall_spatial_index.rs:15 | `IMUL R8D,[RCX+0x48]; ADD R8D,EDX; MOVSXD` = y*dims+x ✓ |
| 0x140aa65e0 | cell_size | wall_spatial_hash.rs:278 | {extent.x/dims.x, extent.y/dims.y} ✓ |
| 0x140aa6600 | bounds_ws | wall_spatial_hash.rs:287 | extent 两 f32 → {−e/2,−e/2,+e/2,+e/2} ✓ |
| 0x1409ef010 | ShapeVal::interpolate | shape_signifier_lerp.rs:53 | expf(-20t)；v1=(1-e)Δ、v2=sign(Δ)min(2t,|Δ|)、取 |·| 较大；圆/矩形两路 + 跨型 panic ✓ |

## 2. 判等-注意（主结构对应，差异已登记）

| VA | 函数 | 差异点 |
|---|---|---|
| 0x14085aa60 | InnerWalls::remove | 结构 ✓（log>4 "Removing wall"、removed insert、remove_entry、panic 文案一致）；Rust 缺 trace 日志调用 |
| 0x1408c5680 | StructureType::is_fence | 4B 空反编译；机器码面按既有 O 证据为 `movzx byte[rcx]`（非零=true）。反编译体不可逐句判等，登记 |
| 0x140b2a890 | get_wall_attachment_height | 委托 `decorator_info::wall_attachment_height`：原生 max_y 门 panic + `u*curve_len` 有限性 panic、`get_coord_at_u<Vec2>` + `get_approx_coord_from_pos<Vec3,Thresholded3D>`；Rust 返回 Err（panic-vs-Err）且用 nearest_segment 替代第二个曲线投影 |
| 0x1412b2a70 | archive_storage_decorators\<&str\> | 选择键原生 **&str**（Rust u8 selector）；已存在 return/克隆入 walls/special1/2 ✓；原生另做 collision `get_collider/get_collider_mesh + insert` + `drop_slow<RaycastMeshData>`（collision 域，登记） |
| 0x1412b3910 | archive_storage_decorators\<String\> | 同上（String 单态） |
| 0x1413e5360 | archive_storage_decorators\<&str\>（另一单态） | 同上 |
| 0x1413e6150 | archive_storage_decorators\<String\>（另一单态） | 同上 |
| 0x1412b4a40 | archive_current_decorators_in_storage | 原生 `DecoratorStorage::get` + archive_storage_decorators 两单态；Rust 逐 wall/terrain/stairs 归档 ✓；collision 重建登记 |
| 0x1412b4f10 | archive_current_decorators_in_storages | 多存储循环 ✓；collision 重建登记 |
| 0x1409f0e80 | get_cells_from_circle\<spatial_hash_debug\> | 与 query 单态同几何；调试消费闭包，覆盖一致 |
| 0x1409f12e0 | get_cells_from_aabb\<query_with_aabb_clamped\> | 原生入口对 min/max 越界 **panic**，再按 padding(in_stack+0x30) floor；Rust 调用方 clamp 后铺格（panic-vs-clamp） |
| 0x140aa5360 | remove_wall_id | 原生 wall_segments→HashSet<usize> 逐格在两 Vec 内 retain 非本墙；Rust cells=Vec<u32> 单 Vec + 清 segment_cells/segments（容器模型登记） |
| 0x140aa5730 | query | 原生计算 clamp 后 grid 坐标返回；Rust 以内联 clamp_to_cell/cell_index_of_world_point 承担（返回形态不同） |
| 0x140aa5850 | query_with_segment | get_cells → sort_unstable_by(圆查询比较器) → 相邻同墙去重 keep 最近 ✓；Rust 复用 segment_cells（越界分支见 §3.2） |
| 0x140aa5ba0 | query_with_rectangle2d | 同骨架排序/去重 ✓；Rust 用 rectangle_cells（覆盖超集，见 §3.1） |
| 0x140a8fd90 | spatial_hash_update | 两事件流：changed→public 探针→逐点 add_segment；deleted→remove_wall_id ✓；Rust replace_wall 先 remove 再逐段投影，log 门一致 |
| 0x140a8b530 | publish_public_wall_state | 11198B 系统：dirty tiles/lazy sources/terrain 采样/crashy_resample_remap_per_segment_2/PublicWallState insert-remove-retain/Ssbo get/事件 grow_one/puffin。Rust 为 `PublicWalls::consume_with_paths` 包装 + OnWallChanged/Deleted 事件；主逻辑跨 recovered_building wall_collection 域（任务已注明），GPU Ssbo/宿主为域外 |

## 3. 未实现-需重做（fresh 分歧点）

### 3.1 inner_walls / inner_wall_state
| VA | 函数 | 分歧点 |
|---|---|---|
| 0x14085a200 | InnerWalls::new_wall_from_curve | **无 Rust body**；原生与 new_wall_w_params 同构：max_id+1 → walls insert → created insert（Rust 仅 new_wall_w_params 覆盖，from_curve 缺失） |
| 0x14085ad80 | InnerWalls::load_state | **Rust 空实现（仅日志）**。原生：前置 3 panic（walls/updated/removed 非空；**不检查 created**）；逐 0x78B 记录把 id 插 changes.created；`HashMap::from_iter<(WallId,WallState)>` 重建 walls；max_id = **记录 id 最大值**；并重建空表。Rust 不建表/不填集合/不设 max_id |
| 0x1408c5660 | FreehandHeightType::toggle | 原生 `*self ^= 1`；Rust **无该类型/方法** |
| 0x1408c5670 | WallStyle::is_halftimbered（同址 gable_roof_style） | 原生 `CMP byte[RCX],2; SETNC` = style>=2；Rust **无 WallStyle 类型** |
| 0x140833230 | StructureType::is_wall / FreehandHeightType::is_follow_terrain（共享地址） | 机器码 byte==0；Rust 仅有 ClutterVersion::is_initial 承载同式，**两谓词无 body**（地址/符号归属登记） |
| 0x14085ebc0 / 0x140860d50 | deserialize_wall_style / visit_str | 自定义 serde 策略（业务）：字符串映射 **"Stone"→0、"StoneWithSkirts"→1、"HalfTimbered"→2、"WattleAndDaub"→2**，未知变体 unknown_variant；Rust 无 WallStyle 类型/实现 |

### 3.2 public_wall_state
| VA | 函数 | 分歧点 |
|---|---|---|
| 0x140b2a680 | max_y_gable | 原生 flag@+0x110 未置位 → **panic_fmt**（专用文案）；Rust `.unwrap_or_else(|| self.max_y())` 回退（panic-vs-return 反） |

### 3.3 decorator_archivist
| VA | 函数 | 分歧点 |
|---|---|---|
| 0x140b2bb00 | all_decorators | 原生返回 walls map iter + special1/special2 两条 Option 链的三元迭代器（+0x18/+0x2a）；Rust **无 body** |
| 0x140b2bb80 | all_storages | 原生 walls + 可选 special1/2 的地址迭代器（`(p==0)*2+1` / `(q==0)+2`）；Rust **无 body** |
| 0x140b2bc00 | decorator_cull_allow_wall | 原生 walls 探针（stride 0x98）命中后 fold 该墙 decorators 集；Rust **无 body** |
| 0x140b2bd30 | decorator_cull_allow_stairs | 原生 Map::fold 单态；Rust **无 body** |
| 0x140b2be70 | extract_before_after_impl | 原生桶 items→DecoratorHistoryState 映射 ✓；但命中 preview 装饰（DecoratorInfo+0xf 非 0）→ **panic "Found a preview decorator…"**；Rust 无该 panic/检查 |
| 0x140b2c4b0 | extract_and_restore_to_storage | 原生 collider map clear + memcpy special1/2 + Drain/chain + `tracked_clone_from<DecoMoveToStorage>` + `DecoratorStorage::get_or_insert_default`；Rust **无 body** |
| 0x140b2cb30 | extract_diff | 原生 chain 三源（walls Drain + before/after IntoIter）→ filter_map extract_before_after_all → from_iter；**Terrain/Stair 地址也产 diff**；Rust 仅遍历 walls，special1/special2 不产条目；配对函数自认"非判等" |
| 0x14123b5c0 | archive_current_decorators_in_storage（大单态） | 原生 `DecoratorStorage::get` + tracked_clone + **collision get_collider/get_collider_mesh + insert + drop_slow** + format_inner；Rust 逐 wall/terrain/stairs 但无 collision 重建（跨域阻塞登记） |

### 3.4 wall_spatial_hash
| VA | 函数 | 分歧点 |
|---|---|---|
| 0x1409ef210 | get_cells_from_rectangle2d | 原生每格 `Rectangle2d::aabb + Aabb2::intersects` 才收；Rust `rectangle_cells` 只按四角 bbox 铺格 → **候选超集** |
| 0x1409ef870 | get_cells_from_segment\<add_segment\> | 端点在 strict bbx 内走矩形相交筛格 ✓；**任一端点越界时原生取中点 grid 坐标并推 1 格**；Rust `segment_cells` 越界 `Vec::new()`（漏格） |
| 0x1409eff80 | get_cells_from_segment\<query_with_segment\> | 同几何 + 越界中点格分支；Rust 复用 segment_cells（同漏格），且收集语义由调用方替代 |
| 0x140aa50e0 | WallSegment::terrain_height_closest_to_ws_pos | 原生 t=clamp01(dot(p−a,b−a)/max(|ab|²,eps)) 后 `(1−t)·h0+t·h1`；Rust **无该函数** |

### 3.5 adjust_wall_height / history_recording
| VA | 函数 | 分歧点 |
|---|---|---|
| 0x140ac1f20 | adjust_wall_height_impl | 原生：mode0 跳过/`!=1` **panic**；walls 探针 miss → **unwrap_failed**；clamp_wall_height、get_coord_at_u、inverse_lerp、sample_at_world_xz、x0y；**写 PreferredNodeAlignment Vec + AlignKind indexmap**；update_wall_state_height 可重复。Rust：错误发 WallHeightCommandFailed 事件（panic-vs-事件）、**无两条 indexmap 记录写** |
| 0x1409566d0 | UiWallHeight::widget | **无 Rust body**（原生 1445B UI widget） |
| 0x140956cd0 | UiWallHeight::signifier | **无 Rust body** |
| 0x1409576d0 | impl$0::signifier::closure$1 | **无 Rust body** |
| 0x14085bb50 | ui_adjust_wall_height_signifier_animation | **无 Rust body** |
| 0x140b2fc00 | ui_adjust_wall_height | **无 Rust body** |
| 0x140ac6d60 | ui_adjust_wall_height_mode | **无 Rust body** |
| 0x140ac7400 | get_closest_point_u_to_cursor | **无 Rust body** |
| 0x140ac79e0 | history_record_adjust_wall_height | **无 Rust body** |
| 0x140ac7f30 | clamp_wall_height | **无同名 body**（策略仅部分落在 resources/walls/height_rules.rs） |
| 0x1409dd790 | history_finish_recording | **无 Rust body**（原生多迭代器/集合操作，621B） |

### 3.6 wall_state
| VA | 函数 | 分歧点 |
|---|---|---|
| 0x140a94cd0 | wall_roof_hashmap | 原生系统（has_roof + AutoClutterObject map insert + 事件 grow_one/puffin，4558B）；Rust 仅 re-export 事件类型，**无 body** |
| 0x1408ec0e0 | spatial_hash_debug | **无 Rust body**（调试可视化） |
| 0x140a8b0c0 | debug_viz_threshold_curve | 原生受 debug 字节@+0xf3 门控的阈值曲线绘制；Rust **无 body** |

### 3.7 shapes
| VA | 函数 | 分歧点 |
|---|---|---|
| 0x14097f620 | edit_rectangle | 原生 as_curve2 + update curve/enclosure ✓；另写 **PreferredNodeAlignment(u64,u64,u64,u32,u16=4) + AlignKind(inverse_lerp clamp)**；Rust 缺两记录 |
| 0x141464a60 | finish_edit_rectangle | 原生 **extract_and_restore_to_storage + ConcurrentEdits(from_iter Arc<WallHistoryEdit>) + WindowFocusEvent/WriteSettingsFileCmd/ProfilerOutputBuffer 事件 + acosf**；Rust 仅 finish_rectangle_edit_decision + 单条 RecordWallHistory，restore 为 host TODO |
| 0x1408dda10 | signifier_edit_dims | 原生 **crashy_slice + update_curve_mesh** + push signifier；Rust 静态 4 边 UiAppendSignifier（无切片/网格） |
| 0x14155d410 | ui_create_rectangle | 原生 add("rectangle_canvas")、is_shape_inside、rectangle_signifier、inverse_lerp、PreferredNodeAlignment/AlignKind、eye_right；Rust 为 RectangleCanvasInput 代理 + dot 近似（注释自认待取证） |
| 0x140ad6250 | ui_edit_rectangle_dims | 原生 **raycast（intersect_ray/is_hit/xz_flatten_aabb/aabb）+ widget/signifier + puffin/command buffer**；Rust 仅 submitting 代理，无 hover/raycast |
| 0x140ad8000 | ui_edit_rectangle_rotate | 原生 eye_position + intersect_ray/is_hit + as_points2 + widget/signifier；Rust 代理无 raycast |
| 0x140ad6e00 | do_ui_edit_rectangle_dims | 原生 get_edge_normal + x0y + miss **panic/unwrap**；Rust 静默 return + expand_along_edge 近似 |
| 0x140ad8be0 | do_ui_edit_rectangle_rotate | 原生 miss **panic/unwrap**；Rust 静默 return；acos 多项式 callee 面不可见，未证实 |
| 0x14155eba0 | UiRectangleDimsEdit::signifier | 原生 get_edge_normal + get_point + `try_new_from_points` + xvy + push；Rust "最近边距离" 近似单 signifier |
| 0x14155fc70 | UiRectangleDimsEdit::widget | **Rust 空函数桩**；原生 get_tris/rotation_from_normal/get_point_u 等 2527B |
| 0x14083d3f0 | UiRectangleOrientationEdit::widget | **Rust 空函数桩（仅保留常量）**；原生 triangle flat_map + box_trigger_volume + orientation_mat3 + get_edge_dirs_from_corner_id |
| 0x14083da10 | UiRectangleOrientationEdit::signifier | **Rust 空函数桩**；原生 get_point/get_corner_normal/rotation_from_normal/x0y/xvy + push |
| 0x140b35120 | signifier_edit_circle_arrow | 原生门控 **tag != 2**（圆+矩形）且迭代 ECS Query/发 format "edit_circle_arrow"；Rust 仅 tag==0（圆）+ public 资源代理 |
| 0x140a64a40 | signifier_edit_circle_line | 原生门控 `tag!=2 && (tag&1)==0` + Circle2d::get_pos_at_u + ECS Query/AppendOnlyVec；Rust PreviewCircle 资源代理（无 Query） |
| 0x1408dec70 | shape_placement_signifiers | 原生门控 **SelectedWallShape tag==6 且 ty∈{1,2} + public tag!=2**，mode==2 与 else 两分支分别 format "snap_signifier_"+push；Rust 仅 mode.wall!=0 门 + 单 signifier |
| 0x140980db0 | snap_to_roof_ridge | 原生屋顶脊几何全链（roof dims 缓动、Rectangle2d::get_point、阈值距离、比例夹取/0.3 边界）；Rust 恒返回 Some(position)（注释自认近似桩） |

## 4. 胶水-非业务（不按业务审计）

| VA | 函数 | 依据 |
|---|---|---|
| 0x140005c60 | decorator_colliders | bevy SystemParam `component_access` 空体 |
| 0x1403c03c0 | decorator_interactions | InfallibleSystemWrapper::component_access 空体 |
| 0x140a1b100 | archived_walls（反编译名 Decorators::iter_removed_ids） | hashset 迭代器构造（访问器）；与 iter_removed_ids 同址，Rust 已有 iter 访问器 |
| 0x1408b99e0 | Sequence::next（ClutterVersion） | enum_iterator derive 6B |
| 0x140b26e40 | SystemParam::init_state | 两资源 init_state 转发 |
| 0x14078cdd0 | Debug::fmt（DecoratorSubtype） | derive Debug（variant 0x1a 单字符串特判） |
| 0x140945f50 | add_wall_shape_ux | 注册 8 系统/4 资源 + 事件（register_* / insert_resource） |
| 0x1422785d0 / 0x1425defd0 / 0x1425df320 | query_with_circle_callback ×3 | 消费者闭包单态（tree/forest）；半径回调转发 |
| 0x1404f3da0 / 0x1404f40a0 / 0x14085ff00 / 0x1408605a0 / 0x1408606b0 / 0x140860e30 / 0x140861220 / 0x140861780 | inner_wall_state serde serialize/deserialize | serde derive visitor（按 inventory kind/符号名分类；未逐句审计） |

## 5. 待判等（本轮未 fresh 复核，不标判等）

- decorator_archivist 其余单态：`0x1413e7230` / `0x1413e77f0` / `0x1421a0470`
  （与已判单态同族，但本轮未 fresh 取证，不推导）。
- shapes 闭包/注册/其它未取证：`0x14165cf80`、`0x141660600`、`0x141660b30`、`0x1409a3620`、
  `0x1409ae3d0`、`0x14155e6b0`、`0x141559a10`、`0x14155f620`、`0x14155f9b0`、`0x14083df50`、
  `0x1408df6c0`、`0x1408dfa00`、`0x14165b330`、`0x1428a8300`、`0x140b35ba0`、`0x140a66c90`、
  `0x1408e0e80`、`0x140b35650`、`0x140a65510`、`0x1408de7e0`。
- walls 域其它已实现但本批未纳入 fresh 的函数（cached_decorator_transforms 已由 walls2 判等；
  decorator.rs 余量、decorator_storage 其余访问器、valid_enclosure 余量、snapping 余量、
  eraser 余量、shapes/circle 余量等）。
- **剩余待判等估计 ~90**（walls 域；不含已判等/已重建清单与本批 63 个判等+49 个需重做）。

## 6. 总结（回传主会话）

- 本批 fresh 判等 **46**、判等-注意 **17**、未实现-需重做 **49**、胶水-非业务 **10**
  （另 8 serde derive 按 derive 分类）；剩余待判等 ~90。
- 最需回派 restorer 的硬分歧：`InnerWalls::load_state`（空实现）、
  `segment_cells` 越界中点格分支、`rectangle_cells` 逐格相交筛、
  `max_y_gable` panic 语义、`adjust_wall_height_impl`/`edit_rectangle` 的
  PreferredNodeAlignment/AlignKind 记录、`edit_arrow` 门控（tag!=2）、
  `snap_to_roof_ridge` 桩。
- UI 代理类（ui_create_rectangle / ui_edit_rectangle_dims/rotate / ui_modes widget/signifier /
  edit_arrow/edit_line / shape_placement_signifiers）目前均为代理或空桩，均按 fresh 标需重做。
- 未改任何代码；只读 `--worker shared`。
