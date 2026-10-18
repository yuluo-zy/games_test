# LOGIC — walls 域逐函数逻辑判等（logic-verifier-walls2 批 2，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`。本报告只以 **fresh 反编译** 为准
> （`reverse-operator.py ghidra --worker shared`，`fields=decompiled_code,callees`，
> 2026-10-17 17:04-17:26；批量落盘 `planning/impl-walls-verify/decomp/batch1-result.json`
> / `batch2-result.json` / `snapping_to_ends-disasm.json` / `batch2-disasm.json`）。
> 不复用此前 reviewer/restorer 描述；判等标准 = 每分支/循环/调用/数据流在 fresh 反编译中
> 可逐条对应，无遗漏无编造。

## 0. 覆盖与结论概览

- 本轮 fresh 反编译 **39 个** walls 域业务函数（优先项：circle UI 系统 5、snapping_to_ends
  （含 471 条指令 disasm 定案）、telegraph_walls_are_merged、tracked_clone_from ×2、
  shape_placement_params、preview_snapping_line、wall_spatial_hash 查询族、eraser 两件、
  lookup_storage；第 2 批：decorator_subtype 12（含 8 个小函数 disasm 定案）、
  cached_decorator_transforms 7）。
- **判等 10 / 判等-注意 21 / 未实现-需重做 8 / 属性疑点 2**（ui_rotate_circle 保留判等-注意，
  其"角度计算链"若按硬标准升级则归入未实现，见 §3.9）。
- 严重分歧集中在**任务优先级点名区域**：create_circle（height_01 硬编码 1.0 vs 原生
  ease 公式）、ui_create_circle（原生输入动作状态机 + 半径 `old+√dist²`，Rust 简化代理）、
  ui_rotate_circle（原生屏幕投影算角 + 历史/音频/归档链）、snapping_to_ends（原生
  snap 点 = **墙端点 u∈{0,1} 3D 位置**，Rust = 游标投影）、telegraph_walls_are_merged
  （原生 easing+切片+网格+实体 spawn/despawn 链）、shape_placement_params（原生
  raycast 命中分派 + SnapProposal 排序 + ±2.25 查询，Rust ±8.0 简化）、
  preview_snapping_line（原生切片曲线取 u 点）、wall_eraser_bulldoze（原生 ray-pick
  命中分派，Rust 事件→缓冲桥）。
- 其余 ~200 个 §5/§6 函数（inner_walls/public_wall_state/inner_wall_state/archivist 大部/
  shapes 余量等）经此前 REBUILD-PENDING/REBUILD-REMAINDER/restorer 报告 fresh 复核过，
  本 session 未逐一重反编译，**按协议维持「待判等」**（见 §4），不以报告替代。

## 1. 已判等（fresh 逐条对应）

| VA | 函数 | Rust 位置 | 判等依据 |
|---|---|---|---|
| 0x140b25b50 | DecoratorSubtype::can_be_on_corner | decorator_subtype.rs:36 | disasm `MOVZX EAX,[RCX]; MOV ECX,0x91008; BT ECX,EAX; SETC AL` = 位测试 0x91008（bit3/12/16/19）✓（fresh disasm 定案） |
| 0x140b25b60 | DerivedDecoratorSubtypes::get | decorator_subtype.rs:183 | hashbrown 探针 → 命中返回 / miss panic_fmt ✓ |
| 0x140a1ada0 | CachedDecoratorTransforms::insert | cached_decorator_transforms.rs:122 | **三批有限性校验同序**：translation@+0x10/+0x14/+0x18 `&0x7fffffff > 0x7f7fffff → panic` → rotation `Vec4::is_finite_mask; movmskps!=0xf → panic` → scale@+0x1c/+0x20/+0x24 `&0x7fffffff < 0x7f800000 全通过才 insert else panic`；= Rust translation/rotation/scale 三断言 ✓ |
| 0x140a1aa20 | transform | cached_decorator_transforms.rs:92 | 探针（0x40B 条目）命中拷贝 40B / miss panic_fmt ✓ |
| 0x140a1a9f0 | position | cached_decorator_transforms.rs:99 | 尾跳 transform → 拷贝 translation（Vec3 2×u64）✓ |
| 0x140a1ac80 | try_transform | cached_decorator_transforms.rs:104 | Option 版探针（miss → 0）✓ |
| 0x140a1ab80 | try_position | cached_decorator_transforms.rs:109 | Option 探针 → translation ✓ |
| 0x140a1af40 | CachedDecoratorTransformStorage::get | cached_decorator_transforms.rs:56 | 存储探针（miss → None）✓ |
| 0x141256e40 | Decorators::tracked_clone_from\<DecoMoveToStorage\> | decorator_storage.rs:296 | 快路径三集合全空 → clone_from×3；慢路径清 modified → 当前内嵌 map 逐 id 标 removed → 清 map → 逐旧条目 **state==2 break** + d2s.remove → 源 removed 回填 → 源 decorators 重插（removed remove + modified insert）；公共尾逐源 d2s.insert ✓（restorer 已重建，fresh 复核一致） |
| 0x1412573b0 | tracked_clone_from\<tuple\<\>\> | decorator_storage.rs:296 | 同结构（fast/slow 路径 + state==2 break + 尾 d2s insert）✓ |

## 2. 判等-注意（主结构判等，差异登记）

| VA | 函数 | 差异点 |
|---|---|---|
| 0x140a1c3e0 | DecoratorStorage::lookup_storage | 探针命中返回 ✓；**native miss → panic_fmt**（decorator::impl$27::fmt），Rust 返回 Option（调用方按 Option 使用，REBUILD-REMAINDER §3 登记不改签名） |
| 0x1409ac910 | ui_rotate_circle | 主结构 case2：cartesian_to_polar + circular_f32::distance + inverse_lerp clamp → **as_curve2 + update_wall_state_curve + update_wall_state_enclosure** = Rust rotate_circle_wall ✓；差异：① 角度由**屏幕空间投影**（相机矩阵世界→NDC `dot(m44,p)/dot(m44w,p)`）计算，Rust 从 RotateCircleCmd 事件取（注释已登记 render 域待接线）；② native 另发 RequestWallMaterialAudio / archive_current_decorators_in_storage / decorator_cull_allow_wall / CompoundEditBuilder+History::add / DebugLine+signifier / 事件记录 ×2 — Rust 均无 |
| 0x14165d460 | ui_edit_circle | 核心编辑（as_curve2 + update curve + update enclosure）与 release 阈值 `ABS(new-old)<=0.01 → extract_and_restore_to_storage()`（Rust `continue` + 注释）逐条对应；差异：① native 大输入动作状态机（puffin scope / HighlightWallCmd / show_full_circle_signifier / closest_point_on_perimeter / wall_decorators_mut 恢复 / get_tris / DebugLine）Rust 无；② 历史：native CompoundEditBuilder::next → ConcurrentEdits → History::add；Rust 写 WallCompoundHistoryEdit::StateChange（history 域登记） |
| 0x140aa5dd0 | WallSpatialHash::add_segment | 控制流 ✓（+0x58 id 递增 → get_cells_from_segment → 逐格处理 → wall_segments/segment_cells insert → 0x38B 段记录写）；**容器模型差异**：native cells 逐格存 **0x38B 段记录**（并带 wall-id 去重扫描 + 记录 Vec@+8/+0x10），wall_segments = HashMap<WallId,HashSet<usize>>（wall→cells）；Rust cells 存 u32 段 id、wall_segments = HashMap<WallId,Vec<u32>>（wall→ids）+ segment_cells = HashMap<u32,Vec<usize>>。标准容器替换登记；查询结果对消费者（最近优先）语义等价 |
| 0x140aa5a10 | query_with_aabb_clamped | 骨架 ✓（get_cells_from_aabb → smallsort/ipnsort 排序（圆查询比较器）→ 相邻 +0x30 同墙折叠）；差异：native dedup 仅**首个相邻同墙触发一次**（将该墙全部后续段压缩），Rust 用 HashSet 全墙去重 — 最近优先消费者等价，语义细节点登记 |
| 0x140aa5940 | query_with_circle | 同 query_with_aabb_clamped 骨架（REBUILD-REMAINDER §1.3 已修复排序去重，fresh 复核一致） |
| 0x140ac27e0 | animate_wall_bulldoze | 主结构逐条对应：phase+=dt/0.2 clamp[0,1] → ==1 时 InnerWalls::remove + finished（+0x90=1）→ 否则 cubic_in_out ease + low=low·e + high=(1-e)+high·e + crashy_slice + freehand(tag=2)/curve_kind=2/style + update_wall_state + 音频事件 + 末 retain(finished)；差异：native 读墙记录字段直接（0x98B 命令记录 +0x54 源曲线/0x8c phase/0x90 finished），Rust clone snapshot 后重切；native remove 无条件，Rust contains 守卫；style 字节 native 抄 +0x69，Rust 重算 `style==1` — 语义等价，借用模型差异 |
| 0x140002370 | DecoratorSubtype::first | disasm `XOR EAX,EAX; RET` = Rust first() Self(0) ✓；**PDB 名 = WireframePass::clone_behavior**（bevy Component derive 胶水同址）— 归属/共享地址疑点，登记 |
| 0x140833250 | DecoratorSubtype→IntoDecoratorSubtype | disasm `MOV EAX,ECX; RET` 字节直通 = Rust `self` ✓；**PDB 名 = DecoratorRank::new**（同址另一符号）— 归属疑点，登记 |
| 0x140b25c90 | DecoratorLibKey::new | disasm `MOV EAX,EDX; MOV EDX,ECX` — 打包返回 {EAX=第二参, RDX=第一参}（结构 {variant,subtype} 序）；Rust {subtype,variant} — 字段序/参交换已登记（源注释"两参交换同址"） |
| 0x140b25ca0 | DecoratorCollision::empty | Rectangle3d 全零 + `SharedShape::cuboid` + `IsoTransform::from_translation` 三连结构 = Rust bounds/dims 全零 ✓；parry3d cuboid + IsoTransform 属 native-physics feature 域（库外，登记） |
| 0x140b26970 | DerivedDecoratorInfo::subtype | disasm 尾跳 `DerivedDecoratorSubtypes::get`（库查表）；Rust 直接返回 self.subtype（无查表）— 差异登记 |
| 0x140b26980 | try_get_subtype | disasm 探针命中返回 entry+4 / miss → 0（None）；Rust `Some(self.subtype)` — 差异登记 |
| 0x140833ba0 | impl$6::into_decorator_subtype | disasm `MOV EAX,ECX; LEA R8,0x142a929a0; MOV RCX,RDX; MOV EDX,EAX; JMP get`（库查表 + panic 位置）；Rust 直接取 self.subtype — 差异登记 |
| 0x140b26a70 | impl$8::from | disasm `MOV RAX,[RCX]; MOV RDX,[RCX+0x20]` **双字段提取 {+0x0,+0x20}**；Rust 仅取 subtype（单字段）— 差异登记 |
| 0x141fdc7b0 | bbx_ls\<DecoratorSubtype\> | 经 DecoratorLibKey 查表（hash_one + new + panic_fmt）返回 bbx；Rust `(self.bbx_min, self.bbx_max)` 直接字段 — 差异登记 |
| 0x140b25ef0 | insert_collision_and_interaction_as_same | native **4 表全插**（collision+interaction 同值 + outline + collision_outline，Arc 克隆 + drop_slow\<RaycastMeshData\>）；Rust 仅 collision+interaction 2 表（outline 插入挪到 insert）— 结构重分布登记 |
| 0x140b262a0 | DecoratorMeshLibrary::insert | native 4 表全插（同 0x140b25ef0 族）；Rust insert 4 表 ✓ — 与 0x140b25ef0 的分工差异登记 |
| 0x140b26690 | get_outline | 探针命中返回；**miss → panic_fmt**；Rust 返回 Option（None）— miss 语义差异登记 |
| 0x140b267f0 | get_collision_outline | 同 get_outline（miss → panic_fmt vs Rust None）— 差异登记 |
| 0x140a1aeb0 | swap_buffers_and_clear | 6 qword 双缓冲逐字段互换 + 清一册（memset 0xff + len 归 0 + growth 重算）；Rust `current_write ^= 1; buffers[current_write].clear()`（逻辑册索引代替物理互换）— 模型差异，语义等价 |

## 3. 未实现-需重做（fresh 分歧，需重建）

### 3.1 create_circle 0x14097e560（3962B）
- fresh：Circle2d::as_points2 → from_iter → try_new_from_points → `new_wall_w_params` →
  **`_try_update_state<update_elevation>`（独立）** → `_try_update_state<update_wall_state_enclosure>`
  → `get_building_and_roof_colors()` → `_try_update_state<update_style>` → 高度公式
  **`height_01 = clamp01((EasingFunction::apply·10 + (1-e)·4)/12)`** → CreateRoofCmd 0x40B
  （+0x28=0.5 profile、+0x2c=height_01、+0x30=0x3e4ccccd3ecccccd（eave/ridge 打包）、
  +0x39=style 字节）→ ev_new_wall + History::add + 第二个 History::add（tag 4）+ 2 事件写。
- Rust：`circle.as_curve2(0.2)` + `new_wall_w_params(4.0 内联)` + `update_wall_state_enclosure`
  + `build_roof_create_request(id, **1.0**, 1.0, 0.0, 0.0, 0)` + `RecordWallHistory::new_wall`。
- 分歧点：① **height_01 硬编码 1.0**，原生 `(e·10+(1-e)·4)/12 ∈ [0.333,0.833]` 永不为 1；②
  elevation/style 独立序 + get_building_and_roof_colors 链 Rust 缺（create_rectangle §B 同款）；
  ③ 原生另写 2 事件 + 第二个历史记录，Rust 缺；④ as_points2 vs as_curve2(0.2) 点生成路径不同。
  → **未实现-需重做**。

### 3.2 ui_create_circle 0x14165bed0（2936B）
- fresh：AppMode tag6/sub1 门 → UiState::add("wall_circle_canvas"/"cursor_circle_") → 可放置门
  （60.0 阈值）→ 输入动作码分派（code 3 / 4 / <2）→ **半径 = `旧半径 + √(游标x²+游标z²)`，
  >4.0 置位** → 状态跳表（0..7：area+inverse_lerp、GladeBorder::is_shape_inside、snapping、
  circle 编辑 as_curve2+update curve/enclosure、PreferredNodeAlignment/AlignKind 记录）
  → signifier + DebugLine + puffin scope。
- Rust：`selected.is_circle()` 门 + `cursor.tag` 门 + anchor 锁定 +
  **`radius = clamp(dist(anchor,cursor), 0.9, 8.0)`** + submit CreateCircleCmd + 1 circle_signifier。
- 分歧点：① 半径公式 **`old+√dist²` / 上限 4.0** vs Rust `clamp(dist,0.9,8)`；② 输入动作状态机
  （code 3/4/<2 分派）+ 状态跳表 8 分支 Rust 全缺；③ GladeBorder 包含校验 / snapping / circle
  编辑链（update curve/enclosure）/ 多记录写（PreferredNodeAlignment/AlignKind indexmap）
  Rust 无；④ signifier/DebugLine/profiler 链为宿主边界。Rust 为简化代理（注释已登记
  input proxy）。
  → **未实现-需重做**。

### 3.3 snapping_to_ends 0x140a6ad80（2077B，disasm 定案）
- fresh + disasm（471 指令）：mode!=1 门 ✓ → in_progress.wall **unwrap_failed panic**（Rust
  return）→ 游标 bit0 门（Rust 无条件读）→ snapped==0 跳过当前墙 / snapped==1 当前墙参与
  （双路径 ✓）→ **候选 = 最近端点（start=0 / end=1，`UCOMISS dist_end,dist_start`）或当前墙
  首原始点** → `get_coord_at_u(curve, **u=XMM6∈{0.0,1.0}**)` → 3D outline 点按 t 插值 =
  **墙端点 3D 位置** → `1.5<=dist return` + 哨兵 end_flag==2 return → 写 mode=2/snapped
  (byte-10)/snap/wall/end。
- Rust：候选筛选 ✓（最近端点/首原始点、1.5 阈值、哨兵、末写一致）；但 **snap 点 =
  `closest_u_on_curve(curve, cursor)`（游标投影任意 u）**。
- 分歧点：**native snap 点 = 墙端点（u∈{0,1}）**，Rust = 游标在墙上的最近投影点。restorer
  注释"get_approx_u_from_pos 语义"在 fresh callee 清单**无 get_approx_u_from_pos 调用**；
  disasm `140a6b302 MOVAPS XMM1,XMM6`（XMM6=0/1）定案 u 为端点标志。
  → **未实现-需重做**。

### 3.4 telegraph_walls_are_merged 0x14086ad70（4723B）
- fresh：事件驱动，每 public 墙段：`timer = (seg+0x40 累积)/0.75 clamp[0,1]` →
  `EasingFunction::apply` → 未完成：`EntityCommands::despawn`（旧实体）→ eased 双 u 界
  （±ε 扩展 + maxps/minps clamp）→ `Curve::slice_into<Vec3,3D>` → `try_new_from_points` →
  `update_curve_mesh` → 记录写（tag5 PreferredNodeAlignment 0x20B）→ `SignifierEntityBuilder::build`
  → `LineAcrossWallSignifier`/`DashedLineLength`/`DrawTopology` insert 命令。
- Rust：逐 public 墙 → `float_0x284*0.5` 侧向偏移 → 写 **1 个 UiAppendSignifier**（tag5）。
- 分歧点：native = **easing 计时 + 曲线切片 + 网格重建 + 实体 spawn/despawn + 组件插入**的
  完整实体链；Rust = 单 signifier 近似，无计时/切片/网格/实体。render-host 实体链为
  AGENTS 外部依赖边界。
  → **未实现-需重做/跨域**（维持 REBUILD-REMAINDER §4.3 登记，fresh 复核一致）。

### 3.5 shape_placement_params 0x14097f970（2003B）
- fresh：AppMode tag6/sub{1,2} 门 → 游标 bit0 门 → `query_with_aabb_clamped`（**±2.25**）
  → `from_iter<SnapProposal, FilterMap>` → SnapProposal **sort_by**（insert_tail/driftsort）
  → 取最近 → public 墙探针 + 墙形状判别（circle/rectangle/freehand 法向）+ 曲线
  `get_approx_u_from_pos×2` → **Rapier::raycast + 大命中类型跳表**（case0/2/3-5/0xf-0x11/...）
  → snap_to_roof_ridge + DebugSphere/DebugLine + 屋顶实体 Query。
- Rust：cursor.tag 门 → `query_with_aabb_clamped`（**±8.0 = QUERY_HALF_EXTENT**）→ 直取
  最近 wall → get_closest_u_from_pos + get_normal_at_u / 曲线切线 → mode=2 写。
- 分歧点：① **查询半宽 ±2.25 vs Rust ±8.0**；② 原生 raycast 命中跳表 + SnapProposal 排序
  收集 Rust 无；③ snap_to_roof_ridge/DebugViz/屋顶 Query/记录多写 Rust 无；④ native 写
  mode{0,1,2} 多载荷，Rust 仅 mode0/2。
  → **未实现-需重做**。

### 3.6 preview_snapping_line 0x1408dfc60（2076B）
- fresh：tag>1（freehand）→ 3D `get_approx_u_from_pos` + u=clamp(p/len,0,1) →
  slice_into/try_new_from_points 或 circular_slice → 错误 → anyhow；tag<=1（预置）→
  圆 +0.315 / 矩形 +0.63 展开 → as_curve3 + crashy_resample → u±0.5 切片 → **tail：
  `get_coord_at_u` 取参数 u 处点**（3D 插值 + 载荷字段）。
- Rust：`try_get_wall_shape()?` → tag 0 圆 +0.315 / tag 1 矩形 +0.63 → `get_pos_at_u(0.0)`
  （u=0 单点），freehand → None。
- 分歧点：① native 返回**切片曲线上参数 u 处点**（u 为入参），Rust 返回 get_pos_at_u(0.0)
  （固定 u=0）；② native 处理 freehand（曲线切片），Rust None；③ native 返回多载荷字段，
  Rust 裸 Vec3。restorer 报告"u=0 边界点"与机器码不符。
  → **未实现-需重做**。

### 3.7 wall_eraser_bulldoze 0x1409de190（2694B）
- fresh：UiState::add → `Rapier::raycast_simple` → 命中类型跳表（case 0x11 demo /
  0-2,6-9 墙 / 4 / 5 / 13 / 14...）→ 逐墙记录（0x98B）距离 + "delete_tree"/"cursor_hammer"
  signifier 写 + UiState::add + 距离判定 → 末 jumptable 动作分派。
- Rust：`for cmd in AnimateWallBulldozeCmd` → buffer.contains 去重 → inner.get snapshot →
  `buffer.push(WallBulldozeCommand)`。
- 分歧点：native = **ray-pick + 命中类型分派 + signifier**（UI 生产者）；Rust = **命令→缓冲
  桥**（消费者）。两函数语义不对应（Rust 覆盖的缓冲入队在 native 是跳表目标 handler，
  不在本函数体内）。
  → **未实现-需重做**（Rust 实现与 VA 归属错位，需按 ray-pick 重建或重定位）。

### 3.8 show_full_circle_signifier 0x141660880（481B）
- fresh：`format_inner`（WallShape/circle fmt 字符串）→ `AppendOnlyVec::push<UiAppendSignifier>`。
- Rust：无对应函数（circle_signifier 直接在 ui_create_circle.rs 写 UiAppendSignifier）。
- 分歧点：native 格式化 + 推 AppendOnlyVec（render-host signifier 存储）；Rust 无独立实现。
  → **未实现-需重做/跨域**（render/UI 宿主边界）。

### 3.9 ui_rotate_circle 0x1409ac910 — 补充
- 见 §2 判等-注意；若按用户硬标准"角度计算链必须重建"则升级为未实现-需重做。本轮登记为
  判等-注意（核心 CPU 墙更新已判等，角度源为 render 域）。裁决留 coordinator。

## 4. 待判等（本轮未 fresh 复核，维持登记）

以下为任务 §5/§6 清单中本 session 未逐句 fresh 复核的函数（此前 REBUILD-PENDING /
REBUILD-REMAINDER / restorer-walls-return 报告已 fresh 反编译判等/登记；按协议本 verifier
未独立复核前**一律不标判等**）：

- decorator_storage：on_wall_split / on_walls_merged / merge_with / apply_decorator_change /
  restore_snapshot（LOGIC §1 已标未实现-需重做，本轮未重复反编译）。
- decorator 族余量：DecoratorType 8 谓词 / TransientDecoratorIdGen / DeferredDecoratorOpEvents
  send/unleash / DecoratorRotationXZ / DecoratorExtraInfo（REBUILD-PENDING 批 1 已 fresh）。
- inner_walls 26 / public_wall_state 12 / inner_wall_state 11 / decorator_archivist 22
  （REBUILD-PENDING 批 2/6 + REBUILD-REMAINDER 已 fresh）。
- eraser finish 11509B / archivist 5 大函数 / create_rectangle / get_building_and_roof_colors
  （restorer-walls-return 已 fresh 登记）。
- valid_enclosure / wall_compound_edit builder / snapping 其余 / shapes 其余 /
  adjust_wall_height / history_recording / publish_public_wall_state（跨域登记）。

## 5. 总结（回传主会话）

- **判等 10 / 判等-注意 21 / 未实现-需重做 8 / 属性疑点 2（0x140002370、0x140833250
  PDB 归属与源注释不符）**。
- 本轮独立 fresh 反编译确认 restorer 已重建的两处真实成立：`tracked_clone_from ×2`
  （快/慢路径 + state==2 break + d2s 尾插逐条对应）与 `CachedDecoratorTransforms::insert`
  （三批有限性校验同序）。此前 LOGIC/REBUILD 对 snapping_to_ends / preview_snapping_line
  的"游标投影"描述与 fresh disasm 不符（native 为端点 u∈{0,1}）。
- 建议 coordinator 将 §3 的 create_circle（height_01 公式 + 颜色链）与 snapping_to_ends
  （端点 snap 点）派回 restorer 重建；ui_create_circle / shape_placement_params /
  wall_eraser_bulldoze 为输入动作状态机/ray-pick 大函数，建议拆子卡逐步重建。
