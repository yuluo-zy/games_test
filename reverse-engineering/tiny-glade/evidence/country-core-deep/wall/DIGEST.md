# wall 域深挖 DIGEST（2026-10-14，dbid tg_a2）

域范围：`resources/walls/*` + `systems/wall/*`（含 wall_state、shapes、snapping、eraser、
adjust_wall_height、valid_enclosure、stairs_walls、shape_intersections）。
标注：**O**=反编译/机器码/原字节直接支持；**I**=推断（注明依据）；**U**=未恢复，不臆造。
地址均为 B patched EXE VA。证据落盘 `decomp/decomp-<va>.txt`（**280 条**：268 索引 + 12 索引外补充）。

## 0. 模块清单与批处理状态

| 项 | 值 |
|---|---|
| 索引函数（sysres-procedures.json wall 过滤） | 268 |
| 成功反编译 | 224（222 成功 + draw_wall 0x140a7cba0、history_record_wall_changed 0x1408dc9f0 索引外补充） |
| NotFound（Ghidra 未建函数） | 56，全部用 `rne-read --hex` 原字节手解（多为 <32B 访问器） |
| 索引外按 wp12 合同补跑 | clamp_wall_height、ui_adjust_wall_height_mode、adjust_wall_height_curve、animate_wall_bulldoze、ui_edit_circle、ui_rotate_circle、ui_edit_rectangle_dims、ui_edit_rectangle_orientation、shape_placement_params、remove_wall_id（10/10 成功，dbid tg_a2_c1） |
| 索引缺口（上游 sysres-procedures.json 未含，已登记） | 上述 12 个地址不在索引；若其他域有同类缺口请自查 |
| 批处理事故 | `run-decomp-batch.py --chunks 2` 有 bug：chunk 会话开 `tg_a2_c0/c1`，decompile 步骤却写 `tg_a2` → 全部 NotFound。已手工修复 chunk job 并直接经 `mcp-job.mjs` 跑通；**runner 需修**（chunk 分支生成 csteps 时未替换 dbid） |

## 1. 资源布局表

### 1.1 `public_wall_state.rs` — PublicWallState（O，0x2a0 记录；calc_wall_bend/publish 均以 0x2a8 步长遍历 = 8B key + 0x2a0 value）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00..+0x08 | WallId key（map 条目，非记录内） | 条目步长 0x2a8 |
| +0x08 | raw Curve2 点数组 ptr | get_wall_attachment_height 0xb2a890：`lVar1 = *(self+8)` |
| +0x10 | raw Curve2 点计数 u64 | 同上越界检查读 +0x10 |
| +0x30 | raw Curve2 长度 f32 | 同上 `param_2 * *(self+0x30)` |
| +0x88 | 3D 曲线（Curve3 区域） | 0xb2a890 `FUN_141469e00(self+0x88, x, y)` 投影 |
| +0xe0 / +0xe8 | f32 高度数组 ptr / len | 0xb2a890 第二组越界 + 插值 |
| +0x108 / +0x10c | max_y 缓存存在位 / f32 | 0xb2a890：`(+0x108)&1 == 0` → panic |
| +0x110 / +0x114 | gable max_y 缓存位 / f32 | 0xb2a680（PDB 名 is_premade_shape，见 §7） |
| +0x258..+0x274 | 形状记录 28B（tag u32 + 24B 联合；0=Circle 16B、1=Rectangle 24B、2=Freehand） | 0xb2a890 `*(u32*)(self+0x258)<2`；is_rectangle/is_circle |
| +0x28c | published 曲线长度 f32 | wall_space_length 0xb2a760 直返；0xb2a890 之外亦见 |
| +0x299 | style 字节 | wp11 先验 |

**方法面核实（新证据）**：
- `wall_space()` 0xb2a740 = `{al: tag!=2, xmm0: f32@+0x28c}`（机器码 `cmp [rcx+0x258],2; setne al; movss xmm0,[rcx+0x28c]`）→ **Option<f32> = 预置形状时 Some(published 长度)**。合同说"调 is_enclosed"**不准确**——判别是形状 tag，不是 enclosure。
- `wall_space_length()` 0xb2a760 = 纯 `movss xmm0,[rcx+0x28c]`。
- `u_space()` 0xb2a770 = `{al: tag!=2, xmm0: 全局常量}`。**常量 = 1.0f @0x142925904**（`movss xmm0,[rip+0x01dfb180]`，rip=0x140b2a784 → 0x142925904 = 0x3f800000；round2 已闭合，早前 0x142c55904 为十六进制加法笔误，且原二进制/B 二进制字节一致、无重定位）。语义：预置形状的归一 u 区间 = Some(1.0)（全曲线可用）。
- `get_wall_attachment_height()` 0xb2a890 完整流程 O：先查 max_y 缓存（缺失 panic
  `wall heights have not been calculated yet. Perhaps you are trying to access it before `init_wall_height_and_arch_segment_height` system had a chance to run?`）；premade（tag<2）→ 返回 max_y(+0x10c)；Freehand → `old_x × raw_len(+0x30)`，NaN/inf 分别 panic（`assertion failed: !u.is_infinite()` 行 29/34），clamp01，`FUN_141467ce0`（Curve2 取段+分数）线性插值 2D 点，再 `FUN_141469e00(self+0x88, …)` 投影到 3D 曲线得索引，最后在 +0xe0 f32 高度数组上插值返回锚高。

### 1.2 `inner_wall_state.rs` — WallState（O：**0x70 = 112B**，InnerWalls map 条目步长 0x78 = {WallId u64, WallState 0x70}）

| 偏移 | 字段 | 证据（_try_update_state 闭包族） |
|---|---|---|
| +0x38 | enclosure 形状记录（tag u32，<2=premade；28B 区域） | mark_as_non_enclosed 0x140aaa8f0 写 2（Freehand）；0x1408c56b0 返回 `tag<2 ? &self+0x38 : null`（=get_enclosure_params） |
| +0x54 | 可编辑高度 f32 | update_wall_state_height 0x140aaaca0 写 `value+0x54`（value=key+8）✓ 与 wall_height_authority 先验一致 |
| +0x58 | terrain floor f32 | 先验 wall_height_authority（本轮未见直接写，保留） |
| +0x60 | freehand_ty u8 | update_freehand_ty 0x140aa99e0 写 byte@+0x60 |
| +0x64 | arch_type u8 | update_arch_type 0x140aa9eb0 写 byte@+0x64 |
| +0x67 | removed_supports u8 | update_supports 0x140aa9800 写 byte@+0x67 |
| 曲线存储 | 双 Vec（8B 元素=2D 点，4B 元素=f32 高度），cap-first | update_wall_state_curve 0x140aaa090 闭包 drop 旧数组：`func_0x140613c20(param_3[1], *param_3<<3, 4)`（{cap@0,ptr@8,len@0x10}）+ 第二 Vec @+0x18 |

**WallState serde 字段名全表（O，deserialize 字符串证据 0x14085ebc0 附近 UNK 串）**：
`curve, height, is_enclosed, elevation, arch_height, arch_ty, style, structure_ty, clutter_version, plaster_peel, removed_supports, is_valid_enclosure, has_skirts`（13 字段，前缀截断列表可见全序）。
另有第二张字段表（旧版 WallStyle 结构 serde）：`freehand_ty, halftimber, lantern_light, fences, window_light, flag, windows, gates, flat_rim_style, lantern_brightness, stairs`（兼容层，U 归属）。

**枚举补证**：
- `is_default_arch_height` 合同称 RVA 0x8c5690；PDB 索引把该名挂到 **0x8c56b0**，而 0x8c56b0 机器码 = `tag@+0x38 <2 ? &self+0x38 : null` = **get_enclosure_params**（合同同址）。0x8c5690 的真实字节待补（疑似 `movss [默认]; cmpeqss` 小函数）。
- FreehandHeightType::toggle 0x8c5660 `*self ^= 1`（O，合同一致）。
- WallStyle::is_halftimbered 0x8c5670 `cmp byte,2; setae`（O）。
- StructureType::is_wall 0x833230 `cmp byte,0; sete`（O）；is_fence 0x8c5680 判别读取（O）。

### 1.3 `mod.rs` — WallId / HighlightColor / split_height_into_skirts

- `WallId`：0x140b34f20 机器码 `cmp qword[rcx],0; setne` = **is_valid**（PDB 索引名 get_inner 错位，见 §7）；get_inner 真身在 0x140885d0（合同）。
- `HighlightColor::to_rgb` 0x140b34f30（合同 O，未复核本轮）。
- `split_height_into_skirts` 0x140b34f80（合同 O，未复核本轮）。

### 1.4 `wall_attachment.rs` — WallCornerAttachment

- `to_rectangle_corner_id` 0x140b256f0 手解：`xor eax,eax; test cl,cl; setne al; movzx edx,cl; dec rdx; ret` → 返回 {present: self!=0, id: self-1}（Option<u32> 编码）✓ 与合同 `0=None,1..=4=角id+1` 一致。
- `from_rectangle_corner_id` 0x140b256c0（已反编译 461B，合同 O）。

### 1.5 `decorator.rs` — DecoratorInfo / DecoratorType 判别族

DecoratorInfo 72B 布局按 wp11 合同（+0x22 目的地 tag、+0x2c id、+0x44 rank、+0x46 type）。
**新证实（手解字节）**：
- `try_dst_as_wall` 0x1408333a0：`cmp byte[rcx+0x22],2; cmovb rax,rcx` → tag<2 返回 &self 否则 null ✓。
- `dst_as_wall_mut` 0x1408332c0 反编译可见 `unreachable!("dst is Roof/Stair/Terrain")` 三 panic（tag 判别）。
- `DecoratorDst::as_roof` 0x1408334e0（773B）；`DecoratorDst::as_wall_mut` 0x140833570 手解：niche 指针判别（ptr∈{3,4}→查全局偏移表）后 `mov eax,[rcx+rax]` 返回 u32——**PDB 名可疑**（返回 u32 值而非 &mut 引用；记待复核）。
- `TransientDecoratorIdGen::generate` 0x1408339e0 手解：`{base:*mut u32@+0, +0x8:u64 偏移游标, +0x10:bool 直写模式}`；flag=1 → `*base+=1` 返回旧值；否则 `+0x8+=1`，返回 `*base + 旧偏移`（两模式瞬态 ID）。
- **DecoratorType 谓词表（O，全部手解）**：
  | 方法 | 地址 | 机器码语义 |
  |---|---|---|
  | is_window | 0x140833160 | `(t+0xc8)<3` → t∈{0x38,0x39,0x3a} |
  | spawns_clutter | 0x140833170 | `(t&0x3e)==0x38` → t∈{0x38,0x39} |
  | can_place_on_roof | 0x140833180 | `t!=0x3a` |
  | can_place_on_stair_top | 0x140833190 | `(t+0xc4)<3 \|\| (t&0x38)!=0x38` → t∈{0x3c,0x3d,0x3e} 或非窗 |
  | can_place_on_terrain | 0x1408331b0 | `(t&0x3c)!=0x38` → t∉{0x38..0x3b} |
  | should_offer_snap_to_corner | 0x1408331c0 | `t!=0x3f` |
  | can_change_seed | 0x140833150 | `(t+0xc8)<8 && (0x27>>shifted)&1` → t∈{0x38,0x39,0x3a,0x3d} |
  → 类型值域：0x38 窗 cottage、0x39 窗 gothic、0x3a 第三窗型（不可上屋顶）、0x3b 窗类、0x3c/0x3d/0x3e 楼梯顶可放、0x3f 角吸附型。

### 1.6 `material_stats.rs` — MaterialStats（O，setter 族全手解）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x28..+0x40 | prev 特征长度 7×f32：0x28 plain、0x2c lintels、0x30 arches、0x34 ？（被 lintels 扣减）、0x38 gates、0x3c stairs、0x40 sewers | arches_length_delta 0x140a887d0 配对 |
| +0x88 | fences_segments_seed u64 | 0x140a88630 `mov [rcx+0x88],rdx` |
| +0x90..+0xa8 | cur 特征长度 7×f32（同序） | 同上 + add_* 族 |
| +0xac / +0xb0 / +0xb4 / +0xb8 / +0xbc / +0xc0 / +0xc4 / +0xc8 | path_segmentation / lintels_segments / pillar_count / arches_segments / arch_pillars / gates_segments / sewer_segments / fences_segments 计数 u32 | setter 族：0x140a88440(+0xac)、0x140a885e0(+0xb0)、0x140a88420(+0xb4，PDB 名 pillar_count 实为 setter)、0x140a885f0(+0xb8)、0x140a88430(+0xbc)、0x140a88600(+0xc0)、0x140a88610(+0xc4)、0x140a88620(+0xc8) |
| +0x68 | 自旋锁字节（LOCK bts/btr） | set_pillar_placements 0x140a88320 |
| +0x70/+0x78/+0x80 | Vec<8B 元素>（cap-first {cap,ptr,len}）= pillar placements | set_pillar_placements 换入新 Vec + 唤醒等待者（哈希等待队列 + 1000000000ns 超时 = futex 式 park） |

**add_*_length 语义（O）**：原子 CAS f32 累加器：`add_lintels_length`(+0x94+=x, +0x9c-=x)、`add_arches_length`(+0x98+=x, +0x90-=x)、`add_gate_length`(+0xa0+=x, +0x90-=x)、`add_sewer_length`(+0xa8+=x, +0x90-=x)、`add_stairs_length`(+0xa4+=x, +0x90-=x)——各特征长度增加时从 +0x90（plain）或 +0x9c 预算扣减。
**arches_length_delta（O）**：7 对 |cur-prev| 求 argmax，返回索引 0..6（0=plain,1=lintels,2=arches,3=?,4=gates,5=stairs,6=sewers）；前 5 对增量之和>0 才进入；索引 0 带 bias 常量 **-0.01**（0xbc23d70a @0x142ad8a14）；平局取后序（机器码分支序 E,D,B,A,C,F,G 映射 0..6）。

### 1.7 `wall_holes.rs` — WallHoles / WallHoleStorage

- WallHoleStorage 0x68B 布局、RawWallHole 20B、FinalizedWallHole 20B（{Aabb2 16B, kind, origin}）按现有重建代码（先验 building-holes 768 例）；本轮补充 O：`clear_for_next_frame` 0x140f867c0（32KB）＝ 逐墙 advance_frame + finalized 迁入 PrevWallHoles（hashbrown insert `FUN_1428c3cc0` + 0x14 步长 memmove ✓）；`FinalizedWallHoles`：holes Vec@0..0x18、`wall_length f32@+0x18`（holes_no_duplicates 闭包 `movss xmm0,[rcx+0x18]`）、closed@+0x1c；`InProgressHoleStorage::holes` 0x140c37760 = 返回 {ptr@+8, len@+0x10} 切片。
- **panic 差异**：原作 `finalize` panic 串 = `finalize called on a wall hole storage that has not been cleared yet.`（0x140f87300）；重建代码断言文案为 "finalize"——语义：finalize 要求前帧已 clear，**重建代码的断言条件/文案需改**（见 §5）。

### 1.8 `decorator_storage.rs` — DecoratorStorage（O 骨架）

- 大记录（≥0x1b0）：5 个集合 @+0x18、+0x48、+0x78、+0x90、+0x120 + 空态 flag @+0x1b0（take_snapshot 0x140a20d70 逐一判空）；I：分别为 wall/roof/terrain/stair/… 装饰表。
- `Decorators`（单墙装饰集合）：removed_ids Vec @+0x0..+0x18；modified Vec @+0x60..+0x70；另有 @+0x30、@+0x78 字段；`!self.immutable` 断言（get_or_insert 族 0x140a1cf30 等）→ Decorators 有 immutable 标志（U 偏移）。
- `get_or_insert_default` 0x140a1cf30（21KB）、`on_walls_merged` 0x140a1ea40（21KB，panic `d2s remove: no such `）、`on_wall_split` 0x140a1f590（46KB）、`take_snapshot` 0x140a20d70（26KB，panic `d2s update: no such `）——`d2s`= decorator-to-storage 同步层；`DecoratorStorageAddrSerializedV2`（serde 名）。

### 1.9 `decorator_archivist.rs` — DecoratorArchivist（O 骨架）

- `decorator_interactions` 0x1403c03c0 手解：返回 &self+0x90（interactions 集合在 +0x90）。
- `decorator_cull_allow_any` 0x140b2bc00：遍历 0x98 步长 map（值 0x90B），命中后 `FUN_140b76260` 回调；返回布尔语义 U（Ghidra 未见返回码路径）。
- `archive_storage_decorators` 0x140b2be70（16KB）、`extract_before_after_all` 0x140b2c4b0（10KB）、`validate_param` 0x140b2bb00（手解：从输入 16B 头做 pmovmskb 零字节位图 → 构建校验描述符 {ptr@+0x20, ptr+0x10@+0x28, len@+0x30, u16 掩码@+0x38, u64@+0x40, +0xc0…}）。
- `extract_diff::closure$0` 0x140005c60 手解：`lea rax,[rcx+0x60]`（闭包环境 +0x60 取字段）。

### 1.10 `decorator_subtype.rs`

- `queue` 0x14003bb80 手解 = **`ret` 空函数**（no-op）。
- `into_decorator_subtype` 0x140833250 手解 = `mov eax,ecx; ret`（字节直通）。
- `collision_mesh` 0x140833ba0 手解 = 尾跳转发（`mov edx,eax; jmp` 到偏移表 getter，sret 风格双参）。
- `get_outline::closure$0` 0x140b267f0（已反编译 5.6KB）、`dims_wall_space` 0x141fdc7b0（13KB）、`get::closure$0` 0x140b25c90 手解 = `mov eax,edx; mov edx,ecx; ret`（两参交换，比较器/哈希器）。

### 1.11 `wall_trigger_volume.rs` — WallTriggerVolume

- `get_tris` 0x1408e80f0（21KB）O：`{ptr@+8, len@+0x10}` 的 quad 数组（**元素 0x60=96B**；+0x58 u64、+0x64 u64、[1]/[2]/+0xc/+0x14 u32 字段）；len==0 → panic `assertion failed: !triangles.is_empty()`（行 0x28）；输出按 `len×0x48`（**每 tri 72B**）分配；选项结构 {+0xc bit0 存在位, +0x10 f32, +0x14/+0x18 u32}。
- `recompute_quads` 0x1408eae40（27KB）：字段区 +0x90..+0x100（+0x98 ×5、+0xe0/+0xe8/+0xf0/+0xf8/…）；panic `!mesh_uvs.is_empty()` / `!normals.is_empty()`（0x1408e8d70 域）。
- `register_required_components` 0x1408e80e0 手解：返回 {+0x20, +0x28} 对（组件注册表切片）。

### 1.12 其他资源

- `inner_walls.rs`：**InnerWalls** = hashbrown map（ctrl@+0、mask@+8、…）＋ **+0xc0 u64 脏代计数器**（wall_ids 0x14085a200 每次 `[self+0xc0] += 1`）＋ +0x30 子结构（wall_ids 尾调 `FUN_1402ca2f0(self+0x30, id)`；new_wall_w_params 0x14085a4a0 经 FUN_141529bb0 走 +0x30 工厂）。`max_id` 0x14085a730 仅调 FUN_1402ca2f0。全局 `cRam0000000143634d90`（静态 bool）×5 影响 wall_ids 分支（调试开关，U）。
- `public_walls.rs`：`PublicWalls` 含 **wall_entities_map @+0x30**（delete_wall 0x140aac1e0 从 `*self+0x30` remove_entry）与 roofs map（`roof_entity` 0x140a923a0 hashbrown 查表返回 Option<Entity>）；`simplified_curve_2d` 方法/字段 O（`assertion failed: w.simplified_curve_2d.is_valid()` public_walls.rs:50，被 publish/spatial_hash_update/clamp_wall_height 内联调用）。`WallEntities`/`WallPlasters` 的 map 在各自 +0x0/+0x20 区（delete_wall 用 `*param_3+0x20`/`*param_4+0x20` 作 map/哈希态，两个 map 哈希态指针互相引用，具体结构 U）。
- `cached_decorator_transforms.rs`：`transform` 0x140a1ada0 = 校验缓存 3×3 矩阵 9 分量有限性（NaN/Inf → panic），U 重建。
- `dormers.rs`：`DormerHoleSsbo::gothic` 0x141286680 手解：构造 0x30B 记录 {u64@0, u32@+8, u32@+0xc, u64@+0x10, u32@+0x18, u32@+0x1c=0, f32×4@+0x20..0x2c}；`cottage` 0x1412866e0 同构但 +0x28 处写 **f64**（movsd 全局常量）——gothic 4×f32 vs cottage 2×f32+f64 的尾区差异 O。
- `decorator_outline.rs`：display 族已反编译（0x140b33b00/0x140b348e0），与现有代码一致（Lines/Sphere 判别、+0x18 偏移位移）。

## 2. 系统行为（O/I/U）

### 2.1 wall_state/（发布权威流，全部 O 反编译）

- **publish_public_wall_state** 0x140a8b530（101KB，14 系统参数）：puffin scope（0x142ad8bd1 等 profile 串）→ 收集 lazy 更新源 → 遍历 InnerWalls/PublicWalls（0x2a8 步长 hashbrown SIMD 探针）→ 每墙：`FUN_141458b20/141458a50`（曲线/形状处理）、`FUN_140b772b0`（AnimateWallBulldozeBuffer::contains，动画中墙跳过）、`FUN_1409e9640`（material_stats 相关）、`FUN_140969720(&UNK_142c2e2b8/…c360)`（SparseInstanceBuffer::allocate，两个静态类型 id）、事件写 `OnWallChanged`（16B 行 {id, WallId}）与删除事件；7×`FUN_14087f390` 为栈上 7 个临时值 drop 胶水；panics：`assertion failed: !is_adding`、`w.simplified_curve_2d.is_valid()`。**typed gap**：inner→public 逐字段复制映射仍 U（大函数未逐行）。
- **spatial_hash_update** 0x140a8fd90（O）：4 参 = EventReader<OnWallChanged> + EventReader<OnWallDeleted> + Res<PublicWalls> + **ResMut<WallSpatialHash>**（param_4 +0x1c tick 回写 ✓）。OnWallChanged → `WallSpatialHash::add_segment` 0x140aa5dd0（先经 PublicWalls 查表 + `simplified_curve_2d` 采样）；OnWallDeleted → `remove_wall_id` 0x140aa5360（内部 hashbrown erase FUN_140b768c0）。panic：`SpatialQuery received OnWallChanged with wall index `（fmt）+ `assertion failed: (0.0..=1.0).contains(&t)`（行 0x2a）。
- **wall_spatial_hash**（O）：`add_segment` 0x140aa5dd0：逐格（格条目 0x30B 链表）向成员 Vec push **0x38B 成员**（{WallId, 段数据}，cap-first Vec `FUN_140562870`/`FUN_1406319d0` 扩容）；`remove_wall_id` 0x140aa5360 从格成员 map erase。断言族：`bbx_min.x >= grid_min.x && bbx_min.y >= grid_min.y`、`bbx_max.x <= grid_max.x && bbx_max.y <= grid_max.y`、`RESAMPLING < self.cell_size().x`（wall_spatial_hash.rs）。64×64 格为合同先验（walls-audit），本轮未复测常量。
- **wall_roof_hashmap** 0x140a94cd0（47KB，O 结构）：Query 迭代器（FUN_140eec970/d64e00/d4d740）＋ PublicWalls/WallEntities hashbrown 读写——维护 wall↔roof 实体双向映射；预置形状无 roof → 清除（合同）。param_3 有 +0x1c 回写（1 个 ResMut 证据点，另一 ResMut 走 Query 路径未见回写，U）。
- **spatial_hash_debug** 0x1408ec0e0 / **debug_viz_threshold_curve** 0x140a8b0c0：合同 O（调试 UI，未深挖）。

### 2.2 核心生命周期（root 13 单元）

- **calc_wall_bend** 0x140a8a710（O）：EventReader（16B 行）→ PublicWalls 0x2a8 步长查找 → 点计数 `<6` → bend=0；否则逐段法向 delta dot 累积 → acosf（合同数值一致）。
- **delete_wall** 0x140aac1e0（O 全读）：5 参；事件行 16B{id, WallId}；从 PublicWalls+0x30、WallEntities、WallPlasters 三 map remove_entry，`Commands::despawn` 用两个类型 id（UNK_142add9c8/142add9e0）分别 despawn 墙实体与石膏实体（石膏 Vec<Entity> 逐实体 despawn）。三个 ResMut 的 tick 回写内联存在（param_2/3/4 +0x1c）。
- **draw_wall** 0x140a7cba0（83KB，O 结构）：4 ResMut（param_2/7/8/9 回写）= InnerWalls + InProgressWallResource + CurveNursery + InProgressCurve（合同签名 ✓）；调 `update_freehand_ty`（0x140aa99e0 ×3）、FUN_140892720（×6，曲线构造/吸附路径）、FUN_140ad3f50（×3，SnappingMode 位置）。输入空 → 早退（合同）。
- **finish_wall** 0x140a800a0（59KB，O 结构）：12+ 参，5 ResMut（param_4/6/9/10/11）。panic 串：`Was asked to finish a wall, but wall_manager has no temp curve`、`Finish wall marked a wall as non-enclosed, but it doesn't exist`（模块串 `country_core::systems::wall::finish_wall` 拼接——**模块路径串与消息串相邻存储**）。历史记录路径（CompoundEditBuilder）存在于 615 行 0xf8 拷贝段（O 迹象）。
- **merge_walls** 0x140999920（O 结构）：`DecoratorArchivist::decorator_cull_allow_any` 0x140b2bc00 ×2、InnerWalls 方法 0x14085aa60 ×2、事件 FUN_1413e7230 ×3——合同调用链 ✓。
- **set_wall_style** 0x1409a2290（O 结构）：CompoundEditBuilder 族（0x140a82b40 ×4、0x141696f50 ×3、0x140a84ec0 ×3）＋ 0x140aaa530/0x140aaa350（_try_update_state<update_style/update_plaster_peel> 的另外两个实例化，**不在索引 268 内**）。
- **wall_splitter** 0x140a74c20（61KB，O 结构）：`FUN_141467ce0` ×3（get_curve_segment_from_u/get_u_at_coord 域）；panic 重（FUN_1428d9518 ×11）；signifier 串 `split_wall`/`split_wall_scissors`。
- **wall_highlight** 0x140981390（O 结构）：命令行 0x18B{…}；WallEntities 查表（0x24B 步长？uVar17*0x18 行遍历）；写 WallHighlight{entity, wall, ty}；4 个 rgba 常量未在 grep 中出现（可能以立即数 xmm 存在），合同值保留。
- **upload_instanced_wall_ssbo** 0x14082c170（O 结构）：1 ResMut（AssetSsboLibrary 回写）；`a Display implementation returned an error unexpectedly`（rustc 串）＝ Ssbo 库路径。
- **deferred_wall_history**：`history_record_wall_changed` 0x1408dc9f0（O）：`FUN_141469a20`（approximate_curve_u）投影 → clamp01 → 严格 low<high 否则写 **0x8000000000000000（i64::MIN 哨兵 = None）** ✓ 合同；`CompoundEditBuilder::wall_deleted` 0x140a876a0（O）：按 u64 数组构建 3 分支（0/1/2）Delete 组。
- **structure_outline** 0x140959fc0（98KB，O 结构）：signifier 名串全表：`outline_wall_top_/outline_wall_bottom_/outline_roof_ridge_/outline_roof_side_/outline_circle_roof_/outline_circle_side_/outline_rect_corner_`——逐墙按形状参数画顶/底/脊/侧描边 ✓ 合同。
- **ui_continue_wall** 0x140aadd30（57KB）+ suppress_continue_wall 0x140aadbb0（2.9KB）：`Resource does not exist…GameTime Resource requested by…` panic 串（Bevy 资源缺省）；ResMut<SuppressContinueWall> 回写 ✓。
- **ui_draw_wall** 0x140aafe40（O 结构）：**5 ResMut 回写**（param_5/7/9/10/14）——合同只列 ResMut<WallDrawState>+ResMut<CurveNursery>，另有 3 个 &mut Mut（CurveNursery 之外资源，U）；`water_hint` signifier 串；`RecordMouseInputIntoWall` 写入路径 FUN_14146c880 域。

### 2.3 snapping（O 主体）

- SnappingMode {mode u8@0, snapped bool@1}；**+0x20 处另有状态字节**（snapping_audio 0x140a6a480 读 `pcVar2[0x20]`：mode==2 且 [0x20]==2 → 跳过；mode==3 且 [0x20]>1 → 跳过；否则写 PlayWallUiSound）——I：上次发声模式/计数。
- snapping_init 0x140ada600：mode=0/snapped=false 重置；空进行中墙早退（合同）。
- check_for_disable_snapping 0x140a6a560：mode∈{2,3} → 写事件（0x20B 行）。
- snapping_to_ends 0x140a6ad80 / snapping_to_segments 0x140a6b5e0：均以 mode==1 为前置（I：仅已在段吸附态才继续探测）；distance_segment_point（SQRT 路径）O。
- snapping_signifiers 0x140a6a650：双查询（段/端预览）O。
- telegraph_walls_are_merged 0x14086ad70（合同 O）。

### 2.4 adjust_wall_height

- **adjust_wall_height_impl** 0x140ac1f20（O）：`_try_update_state<update_wall_state_height>` ×2（0x140aaaca0）+ `clamp_wall_height` + `FUN_141467ce0`（曲线 u）+ `FUN_1409e9640`（地形/距离）；ResMut<InnerWalls> 等。
- **clamp_wall_height** 0x140ac7f30（O，新补）：PublicWalls 0x2a8 步长查墙 → 命中调 `PublicWalls::roof_entity`（取 roof 高度参与 clamp，S02 height_rules 数值）→ 返回 1；缺失返回 0。断言 `w.simplified_curve_2d.is_valid()` 同样出现在此。
- ui_adjust_wall_height_mode 0x140ac6d60（15KB，新补）／adjust_wall_height_curve 0x140957390（4.5KB，新补）／signifier_animation 0x14085bb50：UI/widget 层，O 存在性。

### 2.5 eraser

- **animate_wall_bulldoze** 0x140ac27e0（14KB，新补）：曲线切片（FUN_141466bb0/1412f6a20）、`FUN_140aa9bc0`（另一个 _try_update_state 实例化，**不在索引**）、Time 推进（合同 128/768 全验证先验）。
- **wall_eraser** 0x140accab0（20KB，O 结构）：`PublicWalls::roof_entity` ×4（平屋顶高度补偿 `adjust_wall_height_if_erased_flat_roof_w_no_rims` ✓）；WallEraserArchivist 族。
- **ui_bulldozer** 0x1409de190（O 结构）：`delete_tree_` signifier 串、Commands 族（FUN_140d79f90/14146c880/141281be0）。
- **archivist::finish** 0x14092f990（O）：`assertion failed: !before_wall.curve.is_empty()`、`original_wall_state.curve.is_valid()`、`Tried to init CurveU from an invalid u: `；`uespemosarenegylmodnarodsetybdet`（0x1429cd080 域，倒序存储串，U 语义）；`cache_if_original_wall` 0x14092ea50（Ghidra 未建函数，0x168 帧真实函数，U 逐行）。
- `wall_eraser_bulldoze_condition` 0x1409de170 手解：`*app.tag==9 && *app+4==3`（AppMode 判别 u32@+0 + 子判别字节@+4）。

### 2.6 shapes / valid_enclosure

- `add_wall_shape_ux` 0x140945f50：4 资源注册（合同 O）。
- calc_shape_height 0x140ac80e0（合同 O 数值：base 6.0/8.0、clamp[0.7,14.0]）。
- shape_placement_params 0x14097f970（38KB，新补）：`FUN_1409766a0` ×3（I：closest_segment_to_snap_to）、`FUN_141469a20/141469950/1414671d0`（曲线几何）、`FUN_140b04f70` ×3（I：WallSpatialHash 查询）。
- circle/rectangle 全家（create/edit/ui/finish）共 17 函数已反编译（含 94KB ui_edit_circle），合同调用链核实无误；数值细节未逐行。
- **ShapeParameters**（valid_enclosure）：`get_normal_at_u/get_pos_at_u/perimeter` 0x1411ff600/620/660 手解：`test byte[rax],1; jne/jmp` **位 0 判别**双路分发（tag@+0，载荷@+4）；perimeter 载荷读 `[rdx+4]/[rdx+0xc]/[rdx+0x10]`（圆/矩形几何）。`contains_shape` 0x1411ff330（已反编译，parry3d 路径串）。

## 3. panic/日志串清单（原文 + 地址，节选）

| 串 | 地址 | 位置 |
|---|---|---|
| `wall heights have not been calculated yet. Perhaps you are trying to access it before \`init_wall_height_and_arch_segment_height\` system had a chance to run?` | 0x142aeb068 | public_wall_state.rs（max_y 缓存） |
| `assertion failed: !u.is_infinite()` | 0x142aa5498 | public_wall_state.rs:34 / eraser archivist |
| `Was asked to finish a wall, but wall_manager has no temp curve` | 0x142ad75a8 | finish_wall.rs |
| `Finish wall marked a wall as non-enclosed, but it doesn't exist` | 0x142ad75d0 | finish_wall.rs（模块路径串拼接存储） |
| `assertion failed: !is_adding` | 0x140a8b530 域 | publish_public_wall_state |
| `SpatialQuery received OnWallChanged with wall index ` | 0x142ad94a8 | spatial_hash_update.rs（fmt） |
| `assertion failed: (0.0..=1.0).contains(&t)` | 0x140a8fd90 域 | spatial_hash_update.rs:42 |
| `assertion failed: w.simplified_curve_2d.is_valid()` | 0x142ada193 | public_walls.rs:50（publish/clamp/spatial 三处内联） |
| `assertion failed: RESAMPLING < self.cell_size().x` / `bbx_min.x >= grid_min.x && …` / `bbx_max.x <= grid_max.x && …` | 0x1409ef210 域 | wall_spatial_hash.rs |
| `assertion failed: !triangles.is_empty()` | 0x142aa64c0 | wall_trigger_volume.rs:40 |
| `assertion failed: !mesh_uvs.is_empty()` / `!normals.is_empty()` | 0x1408e8d70 域 | wall_trigger_volume.rs |
| `finalize called on a wall hole storage that has not been cleared yet.` | 0x140f87300 域 | wall_holes.rs |
| `assertion failed: !self.immutable` | 0x140a1c800.. 域 | decorator_storage.rs（get_or_insert 族） |
| `d2s remove: no such ` / `d2s update: no such ` | 0x140a1ea40 / 0x140a20d70 | decorator_storage.rs（on_walls_merged / take_snapshot） |
| `internal error: entered unreachable code: dst is Roof / Stair / Terrain` | 0x1408332c0 域 | decorator.rs dst_as_wall_mut |
| `assertion failed: !before_wall.curve.is_empty()` / `original_wall_state.curve.is_valid()` / `Tried to init CurveU from an invalid u: ` | 0x14092f990 域 | eraser/archivist.rs |
| `assertion failed: !curve.points.is_empty()` | 0x140ac7400 | adjust_wall_height（get_closest_point_u_to_cursor） |
| `Resource does not exist…GameTime Resource requested by…` | 0x140aadd30 域 | ui_continue_wall |
| `uespemosarenegylmodnarodsetybdet` / `modnarodsetybdet` | 0x1429cd080/0x1429cd090 | eraser archivist（**倒序存储**，U） |
| `outline_wall_top_/outline_wall_bottom_/outline_roof_ridge_/outline_roof_side_/outline_circle_roof_/outline_circle_side_/outline_rect_corner_` | 0x140959fc0 域 | structure_outline（signifier 名前缀） |
| `split_wall` / `split_wall_scissors` | 0x140a74c20 域 | wall_splitter |
| `delete_tree_` / `edit_circle_` / `snap_signifier_` / `wall_height_edit_` / `rectangle_edit_dims_line_` / `move_decorator_` / `water_hint` | 各系统域 | UI signifier 名前缀 |
| serde 字段表 `curve,height,is_enclosed,elevation,arch_height,arch_ty,style,structure_ty,clutter_version,plaster_peel,removed_supports,is_valid_enclosure,has_skirts` | 0x142a95aa9 域 | WallState serde |
| serde 名 `DecoratorStorageAddrSerializedV2` / `IsEnclosedNoYesPremadeShapeIsEnclosedV2` | 0x14155ba30/0x1416eb640 域 | 存储/围合 serde 枚举 |
| 源路径泄漏：`/home/h3/code/pouncelight/country-slice-private/crates/…` | 0x1408dfc60 域 | utils 曲线断言（**工作室 = Pounce Light，开发机 h3**） |
| puffin 0.14.1 / bevy_ecs 0.16.0 / parry3d 0.17.0 / rustc 05f9846f | 多处 | 依赖版本证据 |

## 4. 与现有合同/代码的差异（逐条）

1. **wp11 state.md 错位**：`max_y_gable` 0xb2a680 的 PDB 索引名为 `is_premade_shape`（§7 已纠）；wp11 表格本身地址/行为正确。
2. **wp11 state.md `wall_space` 语义**：合同"墙可用 u 区间（调 is_enclosed）"；机器码 = `{tag!=2, f32@+0x28c}`（Option<f32>），判别为形状 tag 非 enclosure。`u_space` 返回全局常量 f32（非记录字段）。建议合同改为"预置形状返回 Option<f32> 长度/常量区间"。
3. **wp12 core-lifecycle.md `ui_draw_wall` 签名**：实际 5 个 ResMut（param_5/7/9/10/14 tick 回写），合同只列 2 个 ResMut——至少 3 个未登记 &mut Mut 资源（U 名称）。
4. **wp12 wall-state.md `wall_roof_hashmap`**：仅观察到 1 个 +0x1c 回写（param_3），合同列 ResMut<PublicWalls> + ResMut<WallEntities> 两个——第二 ResMut 经 Query 辅助路径消费，回写模式缺失（I，非否定）。
5. **wp12 eraser.md `wall_eraser_bulldoze_condition`**：机器码 = `*app.tag==9 && byte@+4==3`（AppMode 判别 u32@+0，子判别@+4），合同未给数值，此处补齐。
6. **重建代码 `wall_holes.rs::finalize` 断言**：原作 panic = "finalize called on a wall hole storage that has not been cleared yet."——**要求前帧已 clear**（phase 判别之外另有前置条件）；重建代码断言 `phase == InProgress` + 文案 "finalize" 与原文不符，建议改文案并核对"未清理"前置。
7. **重建代码 `decorator.rs::DecoratorRank`/`DecoratorType`**：DecoratorType 判别值域（0x38..0x3f）与谓词表本轮已全部手解（§1.5），重建代码仍为 Unknown=0 占位——需按表更新；`DecoratorRank` 1..=3 有效为 wp11 先验。
8. **重建代码 `inner_walls.rs`**：`InnerWalls { walls, next_id, gap }` 与证据不符——证据：+0xc0 脏代计数器、+0x30 工厂子结构（wall_ids/new_wall_w_params 均用），map 条为 {WallId, WallState(0x70)}；`next_id` 未在 +0xc0（+0xc0 是每调用递增的代计数，不是 ID 分配游标）。
9. **重建代码 `public_wall_state.rs::point_count` 注释**：calc_wall_bend 读的是 map 值区 +0x18（0x2a8 步长条目内偏移），与 raw 曲线 +0x10 的关系需按 0x2a0 记录重核（本轮 get_wall_attachment_height 证实 raw 计数在 +0x10）。
10. **重建代码 `decorator_storage.rs`**：全 Gap；证据已给出 5 集合偏移（+0x18/+0x48/+0x78/+0x90/+0x120）、+0x1b0 空态 flag、Decorators removed_ids@+0/modified@+0x60、`!self.immutable` 语义——可进入下一轮恢复。
11. **重建代码 `wall_trigger_volume.rs`**：全 Gap；证据：quad 数组 {ptr@+8,len@+0x10}、元素 0x60B、tri 输出 0x48B、选项 {+0xc bit, +0x10 f32, +0x14/+0x18 u32}。
12. **重建代码 `material_stats.rs`**：全 Gap；§1.6 布局表可直接落地（注意 +0xb4 setter 的 PDB 名 pillar_count 少 set_ 前缀，疑为索引名截断/错位）。
13. **wp11 shape.md `is_default_arch_height` 地址**：0x8c56b0 实为 get_enclosure_params（合同 §3 内部已自洽，但 PDB 索引与 inner_wall_state.rs 重建注释把 is_default_arch_height 挂在 0x8c5690，两者需在证据里分开登记）。
14. **重建代码 `dormers.rs`**（若有）：gothic/cottage 0x30B 构造器尾区 f32×4 vs f64 差异（§1.12），cottage 用双精度常量——检查重建是否沿用。

## 5. 跨领域线索

| 线索 | 地址/证据 | 目标域 |
|---|---|---|
| `FUN_141467ce0`（Curve2 取段+分数）、`FUN_141469e00`（2D→3D 投影）、`FUN_141469a20`（approximate_curve_u）、`FUN_141469950`、`FUN_1414671d0` | public_wall_state/wall_splitter/deferred_wall_history/shape_placement | utils::curve 域（curve 域 owner） |
| `CurveU` 构造 panic `Tried to init CurveU from an invalid u: ` | eraser archivist | utils::curve |
| `assertion failed: w.simplified_curve_2d.is_valid()` 三系统内联 | publish/spatial_hash_update/clamp_wall_height | public_walls.rs 自身方法（wall 域内部） |
| `init_wall_height_and_arch_segment_height` 系统名（panic 文案） | public_wall_state.rs | wall 高度初始化系统（wall 域，索引外） |
| TerrainHeightsData 采样 `FUN_1409e9640` | adjust_wall_height_impl / snapping_to_segments | terrain 域 |
| `AppMode` 判别 `tag==9 && +4==3` | wall_eraser_bulldoze_condition | ui/mode 域 |
| `SparseInstanceBuffer::allocate`（两个静态类型 id 0x142c2e2b8/0x142c2e360） | publish_public_wall_state | render/gpu 域 |
| `AnimateWallBulldozeBuffer::contains` 0x140b772b0 | publish | eraser 域（本域） |
| `Roof::is_flat_w_no_rim` / roof height_01（clamp_wall_height → PublicWalls::roof_entity） | adjust_wall_height / eraser | roof 域 |
| `GameTime` 资源（`Resource does not exist` panic） | ui_continue_wall | time 域 |
| puffin 0.14.1 profile 串（全部大系统） | 多处 | 性能域 |
| `/home/h3/code/pouncelight/country-slice-private/` | utils 断言 | 全局（工作室名 + 开发机，供其他域引证） |
| `FUN_141529bb0`（map 遍历 + 闭包驱动） | wall_ids/new_wall_w_params | utils/std 胶水 |
| `func_0x140c18130/140c16d40/140c170f0`（事件流 next/刷新） | publish/finish | bevy events 胶水 |

## 6. 错位/纠错

1. **0x140b34f20**：PDB 索引名 `WallId::get_inner`，机器码 `cmp [rcx],0; setne` = **is_valid**。get_inner 真身 0x140885d0（wp11 合同正确）。
2. **0x140b2a680**：PDB 索引名 `PublicWallState::is_premade_shape`，机器码 `(+0x110)&1 → +0x114 f32` = **max_y_gable**（与 wp11 合同一致；PDB 名与行为错位）。
3. **0x1408c56b0**：PDB 索引名 `is_default_arch_height`，机器码 `tag@+0x38<2 ? &self+0x38 : null` = **get_enclosure_params**（wp11 合同同址同义；is_default_arch_height 真身应在 0x8c5690 附近，待字节复核）。
4. **0x140833570**：PDB 名 `DecoratorDst::as_wall_mut`，返回 u32 值（非 &mut 指针），疑为错位或该方法的真实返回为值型字段——标记待复核。
5. **0x140a88420**：PDB 名 `MaterialStats::pillar_count`，行为是 `mov [rcx+0xb4], edx` **setter**——名应为 set_pillar_count 或索引错位。
6. **`run-decomp-batch.py --chunks` bug**：chunk 会话开 `dbid_c0/c1` 但 decompile 步骤用裸 `dbid` → 268 全 NotFound（本 agent 已绕过修复；建议 runner 在 run_chunk 中替换 `s["args"]["database"] = cdbid`）。
7. **sysres-procedures.json 索引缺口**：12 个 wp12 合同地址不在索引（draw_wall、history_record_wall_changed、clamp_wall_height、ui_adjust_wall_height_mode、adjust_wall_height_curve、animate_wall_bulldoze、ui_edit_circle、ui_rotate_circle、ui_edit_rectangle_dims、ui_edit_rectangle_orientation、shape_placement_params、remove_wall_id），另有 _try_update_state 实例化 aaa350/aaa530/aa9bc0 未列——索引生成规则需补。

## 7. 证据文件清单

- `decomp/decomp-<va>.txt` ×280（224 反编译 + 56 NotFound 错误文本）
- `va-map.json`（VA→PDB 名+文件）、`summary.json`（280 行状态）、`batch-job.json`/`chunk-*-job.json`/`chunk-*-report.json`、`remain-job.json`/`remain-report.json`/`remain-addrs.json`、`extra-job.json`/`extra-report.json`、`extra2-job.json`/`extra2-report.json`/`extra2-map.json`、`run-remaining.py`/`run-extra.py`（锁协议 runner，可复用）
