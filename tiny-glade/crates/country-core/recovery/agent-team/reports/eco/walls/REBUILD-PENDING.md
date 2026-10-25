# REBUILD-PENDING — walls 域待判等/未实现余量分批处理（impl-walls-pending，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`。方法：每函数 **fresh 反编译**
> （`reverse-operator.py ghidra --worker shared`，`fields=decompiled_code,callees`，
> 每批≤20）→ 分类（宏胶水→标记；真业务→判等/实现/重建）。落盘
> `planning/impl-walls-pending/decomp/batchN/`。红线：只以 fresh 反编译为准；
> 不删除已判等函数；不改他模块。

## 覆盖与汇总

- fresh 反编译/反汇编批：batch1（decorator.rs DecoratorType 谓词位运算族 +
  TransientDecoratorIdGen + DecoratorRotationXZ 族 + DeferredDecoratorOpEvents send/unleash +
  DecoratorExtraInfo From/accessors + DecoratorOpEventKind::id）；batch2（inner_walls.rs
  全余量 + public_wall_state.rs 余量 + inner_wall_state.rs）；batch3（shapes 余量 +
  eraser finish 0x14092f990）；batch5（decorator_storage/wall_spatial_hash/
  decorator_archivist 余量）；batch6（archivist 迭代链 5 反编译 + 跨域登记）。
- **判等/判等-注意 ~50**；**已重建 7**（RotationXZ 5 件 + DecoratorExtraInfo 判别/载荷 +
  shape_signifier_lerp）；**未实现-需重做登记 6**（create_rectangle ridge/序、
  get_building_and_roof_colors、eraser finish、restore_decorators、unleash、
  DecoratorOpEventKind::id）；**跨域登记 1 组**（archivist 迭代链 5 + telegraph +
  snap_to_roof_ridge + publish_public_wall_state）。
- 编译/测试：`cargo check -p country-core` 通过；`decorator::tests` 6 passed、
  `shape_signifier_lerp` 2 passed（rhapsody 并发 WIP 瞬时报错重跑干净）。
- 改动文件：`resources/walls/decorator.rs`、`resources/color_id.rs`（注释）、
  `systems/wall/shapes/utils/shape_signifier_lerp.rs`。未改他模块业务。

---

## 批 1 — decorator.rs 谓词/ID 生成/旋转/事件（2026-10-17）

依据：`decomp/batch1/*.txt` + disasm（`batch1-disasm-result.json`、`batch1b-disasm-result.json`、
`batch1c-xref-result.json`，fresh）。文件：`resources/walls/decorator.rs`。
这批闭合 LOGIC §5「DecoratorType 8 谓词位运算 / TransientDecoratorIdGen /
DeferredDecoratorOpEvents / DecoratorRotationXZ / DecoratorExtraInfo From 变体」。

### 判等（disasm 逐字节对上，fresh）

| VA | 函数 | 依据 |
|---|---|---|
| 0x140833130 | DecoratorType::max_rank | `ADD CL,0xc8; CMP CL,3; SETC; LEA RAX,[1+RAX*2]` = (t+0xc8)<3→3 否则 1 ✓ |
| 0x140833150 | can_change_seed | `0x27>>(t+0xc8) & (t+0xc8<8)`；t+0xc8 mod8 == t&7，与 Rust 等价 ✓ |
| 0x140833160 | is_window | (t+0xc8)<3 ✓ |
| 0x140833170 | spawns_clutter | (t&0x3e)==0x38 ✓ |
| 0x140833180 | can_place_on_roof | t!=0x3a ✓ |
| 0x140833190 | can_place_on_stair_top | (t+0xc4)<3 \|\| (t&0x38)!=0x38 ✓ |
| 0x1408331b0 | can_place_on_terrain | (t&0x3c)!=0x38 ✓ |
| 0x1408331c0 | should_offer_snap_to_corner | t!=0x3f ✓ |
| 0x1408339c0 | TransientDecoratorIdGen::new | base=rdx、+0x10=r8b、+0x8=0 ✓ |
| 0x1408339e0 | TransientDecoratorIdGen::generate | 直写：`MOV EAX,[*base]; LEA ECX,[RAX+1]; MOV [RDX],ECX; RET`（返回旧值）✓；偏移：`ADD EAX,EDX`（返回 base+旧偏移）✓ |
| 0x140839490 | DecoratorExtraInfo::try_as_stair | (disc\|0xf8)<0xfa → self 否则 null（CMOVC）✓ |
| 0x1408394b0 | as_stair | (disc\|0xf8)<0xfa 门 + panic ✓ |
| 0x140839340 | as_flag | disc==5 门 ✓ |

### 未实现-需重做 → 已重建（fresh disasm 定案）

| VA | 函数 | 分歧点 → 重建 |
|---|---|---|
| 0x140833a00 | from_xz_forward_vector | 原生 `MOVAPS XMM2,[0x1429258f0]`（符号掩码 0x80000000×4）+ `XORPS XMM0/XMM1` → `atan2f(-x,-y)`；Rust 仅 `atan2(x,y)`。重建：`(-v.x).atan2(-v.y)` ✓ |
| 0x140833a20 | from_quat | 原生展开 = `atan2(2(wy+zx), w²+z²-x²-y²)`（通用四元数公式）；Rust `atan2(w,y)/0.5` 仅纯 Y 旋转近似且 y/w 约定颠倒。重建为原生公式 ✓ |
| 0x140833ac0 | to_quat | 原生 = (x=0, **y=sin(a/2)**, z=0, **w=cos(a/2)**)；Rust (y=cos, w=sin) y/w 对调。重建 ✓（对纯 Y 自洽，roundtrip 测试过） |
| 0x140833b20 | to_projective_xz_mat2 | 原生列 = {cos, sin} / {-sin, cos}（旋转阵 [[cos,-sin],[sin,cos]]）；Rust 列 {sin,-cos}/{cos,sin} 错位。重建 ✓ |
| 0x140833b80 | offset | 原生 `fmodf(a+amount, 6.2831855f)`；Rust 仅相加不回绕。重建 `(a+amount) % TAU` ✓ |
| DecoratorExtraInfo | From/accessors 判别值 | 原生 as_chimney/as_flag/as_window/as_lantern 分别 `cmp byte[+0x11]` 0x4/0x5/0x6/0x7（panic 文案证实：烟囱=4、旗=5、窗=6、灯=7）；Rust 原 Window=4/Lantern=6/Chimney=7 全错位。重建 disc()/extra()/set_extra/From 注释与**载荷型**（原生 From：0x140839520=Chimney{f32,u8}、0x140839540=Window{u32}、0x140839550=Lantern{f32}）✓ |
| 0x1408336b0 | DeferredDecoratorOpEvents::send | 判等-注意：原生 futex 锁（CMPXCHG 标记 8 + park 1e9ns + 全局等待队列唤醒）+ Vec push 0x88B；Rust Mutex+Condvar 语义等价（推送顺序/元素 0x88 ✓） |

### 登记（未重建，结构差异）

| VA | 函数 | 分歧点 |
|---|---|---|
| 0x1408337c0 | unleash | 原生逐条 drain：**tag==5 哨兵提前 return** + 输出元素 0x90（tag@+0 + 0x88 载荷）；Rust `out.extend(g.drain(..))` 无哨兵、元素 [u8;0x88]——未实现-需重做（登记待分批） |
| 0x140833570 | DecoratorOpEventKind::id | 原生 `MOV RAX,[RCX]; LEA RDX,[RAX-3]; CMP RDX,2; CMOVC R8,RAX; LEA RAX,[0x142a92e78]; MOV RAX,[RAX+R8*8]; MOV EAX,[RCX+RAX]`——**读事件记录字段**（disc∈{3,4} 经跳表取偏移）；Rust `id()` 返回固定 0/1/2——枚举变体集/载荷未重建（未实现-需重做，登记待分批） |

### 自检

- `cargo check -p country-core` 通过（仅既有 non_snake_case 警告）。
- `cargo test -p country-core --lib decorator::tests`：**6 passed**（含重建后
  decorator_extra_info_roundtrip 新判别值 4/6/7 断言、rotation_quat_roundtrip）。
- 改动文件：`resources/walls/decorator.rs`（判别值/载荷/旋转族重建）、
  `resources/color_id.rs`（1 注释同步）。未改他模块业务。

---

## 批 2 — inner_walls.rs / public_wall_state.rs 余量（2026-10-17）

依据：`decomp/batch2/*.txt` + disasm（`batch2b/c-disasm-result.json`，fresh）。
文件：`resources/walls/inner_walls.rs`、`resources/walls/public_wall_state.rs`、
`resources/walls/inner_wall_state.rs`。这批闭合 LOGIC §6「inner_walls 26 /
public_wall_state 12 / inner_wall_state 11」中未 fresh 复核余量。

### 判等（fresh）

| VA | 函数 | 依据 |
|---|---|---|
| 0x140859f90 | InnerWalls::new | 原生 4 集合初始化（walls map@+0x0 + created/updated/removed 三 HashSet@+0x30/0x60/0x90，各带 RandomState）+ **max_id@+0xc0=0**；= Rust Self::default ✓ |
| 0x14085a560 | InnerWalls::get | hashbrown 探针 get（键 stride -0x78 = 0x78B 记录）；= std HashMap::get ✓ |
| 0x14085a650 | InnerWalls::get_unwrapped | 探针 + `unwrap_failed` panic；= Rust `.unwrap()` ✓ |
| 0x14085ad20 | InnerWalls::update_arch_height | 转发 `_try_update_state<update_arch_height closure>` ✓ |
| 0x140aaa350/8f0/710/aac0/530/9bc0/9610/aa090 | _try_update_state ×8 | 探针 get_mut → 命中：`HashMap::insert<WallId,()>`（updated 集合）+ 写记录字段（update_arch_height 写 record+0x64 = **WallState+0x5c** arch_height，与 Rust `observed_path_scalar` 注释一致）；未中：log warn（"Was requested to update X that doesn't exist"）→ = Rust try_update ✓ |
| 0x140b2a5d0 | get_wall_shape | tag@+0x258<2 → &shape 否则 unwrap_failed ✓ |
| 0x140b2a600 | try_get_wall_shape | `CMP dword[+0x258],2; LEA RCX,[+0x258]; CMOVC` = tag<2 → Some ✓ |
| 0x140b2a790 | is_rectangle | tag==1（<2 且奇）→ 拷贝 +0x25c/+0x264/+0x26c 三 u64（6 f32）→ Some(Rectangle2d) ✓ |
| 0x140b2a7d0 | is_circle | tag==0 → 拷贝 +0x25c/+0x264 两 u64（4 f32）→ Some(Circle2d) ✓ |
| 0x140b2a800 | corner_pos_and_normal_xz | tag==1 → Rectangle2d::get_point + get_corner_normal → Some((pos,normal)) ✓ |
| 0x140b2a890 | get_wall_attachment_height | 高度标志门（+0x108 bit0）→ tag>1 分支：u×length(+0x30) + NAN/INF panic + get_coord_at_u 2D + 边界 + get_approx_coord_from_pos 3D + 边界——Rust 委托 wall_attachment_height（标注"Exact ordinary branch"，逐条对应）✓ |
| 0x1408c5670 | WallStyle::is_halftimbered | `CMP byte[RCX],2; SETNC` = >=2；Rust recovered_building/wall_state.rs `byte>=2` ✓ |
| 0x140833230 | ClutterVersion::is_initial | `CMP byte,0; SETZ` ✓ |
| 0x1408c56b0 | get_enclosure_params | `CMP dword[+0x38],2; CMOVC` = tag<2 → Some（REVIEW-DEEP 已判等，fresh 复核一致）✓ |

### 判等-注意

| VA | 函数 | 差异 |
|---|---|---|
| 0x14085a200 | new_wall_from_curve | 原生独立 VA（+0xc0 递增 + walls insert + 集合 insert）；Rust 并入 `new_wall`（无独立对应函数）——结构等价登记 |
| 0x140b2a620 | max_y | 原生读**预计算标志** +0x108 bit0 + [0x10c]；Rust 重算 regular/freeform 缓存 Option——语义等价（LOGIC §2 flat_roof_y 同族） |
| 0x140b2a680 | max_y_gable | 原生读 +0x110 bit0 + [0x114]（gable-or-base）；Rust gable_heights.or(max_y)——等价 |
| 0x140b2a890 | get_wall_attachment_height | 原生 panic；Rust 委托返回 Result（panic→Err 映射登记） |

### 登记（未实现）

- `FreehandHeightType::toggle` 0x1408c5660：`*self ^= 1`；Rust 无 FreehandHeightType 类型（freehand_ty 建模为 u8）——未实现登记。
- `deserialize_wall_style` 0x14085ebc0：serde 兼容跳表反序列化（0x142a95cb8 变体分派 + 13 字段名表）；Rust 无 WallStyle 枚举（style 建模为 u8）——存疑/glue，未实现登记。

### 自检

- 本批无改动文件；`cargo check -p country-core` 通过。

---

## 批 3 — shapes 余量 + eraser finish 反编译（2026-10-17）

依据：`decomp/batch3/*.txt`（fresh）。文件：`systems/wall/shapes/**`、
`systems/wall/eraser/archivist.rs`。

### 已重建（fresh 定案）

| VA | 函数 | 分歧点 → 重建 |
|---|---|---|
| 0x1409ef010 | ShapeVal::interpolate（lerp_shape） | fresh：**平滑项 `v1=(1-expf(-20·t))·Δ`（指数速率 20，非 cubic ease rate 10）**；**钳项 `v2=sign(Δ)·min(2t,|Δ|)`（保留符号，非无符号幅值）**；增量取 **\|v1\|/\|v2\| 较大者**（均受 \|Δ\| 界不越界，非取较小）。Rust 此前 cubic ease + 无符号 v2 + 取较小——已重建（circle/rectangle 逐轴同构）✓；tests 2 passed |

### 判等-注意

| VA | 函数 | 差异 |
|---|---|---|
| 0x14097f620 | edit_rectangle | `Rectangle2d::as_curve2 → _try_update_state<update_wall_state_curve> → _try_update_state<update_wall_state_enclosure>` 逐条对应 ✓；原生另发 **preferred-node-alignment 事件（0x20B）+ 楼梯 AlignKind indexmap 事件 + inverse_lerp 屋顶高度 ease**——render/UI 域待接线（注释已登记） |
| 0x140ad6e00 | do_ui_edit_rectangle_dims | get_edge_normal/get_point/expand_along_edge 结构 ✓；原生另调 **GladeBorder::is_shape_inside + search_lerped_shape_that_fits_into_glade + xvy/x0y swizzle**（超出边框时 lerp 回缩）——Rust 无 glade-border 包含校验，登记 |
| 0x140ad6250 / 0x140ad8000 / 0x140ad8be0 / 0x141464a60 | ui_edit_rectangle_dims / ui_edit_rectangle_rotate / do_ui_edit_rectangle_rotate / finish_edit_rectangle | 结构复核中；finish 调 **extract_and_restore_to_storage**（archivist）+ History::add——登记待逐句 |

### 未实现-需重做（fresh 分歧）

| VA | 函数 | 分歧点 |
|---|---|---|
| 0x140a93790 | create_rectangle | ① **ridge_length_01 公式**：原生 `clamp((max_dim-2.25)/5.25,0,1) → EasingFunction::apply → (3+3eased)/12 → clamp`；Rust `(max_dim-2.25)/max_dim` 直接 clamp（缺 5.25 分母、easing、/4 lerp）。② 原生 `new_wall_w_params` 后 **单独 _try_update_state<update_elevation> + get_building_and_roof_colors + _try_update_state<update_style>**（Rust 把 elevation/style 并入 new_wall_w_params 参数）。③ 原生屋顶创建走复杂 building/roof 颜色查表 + History::ev_new_wall/add——Rust 简化 |
| 0x140a94880 | get_building_and_roof_colors | Rust 无对应（颜色库查表 + 屋顶请求颜色装配）——未实现登记 |
| 0x14092f990 | eraser finish（11509B） | 见批 4 专项节 |

### 登记（存疑/glue）

- `preview_snapping_line` 0x1408dfc60 / `shape_placement_signifiers` 0x1408dec70：signifier 实体链系统（render/UI 域接线）；Rust 有对应结构但实体链为宿主边界——登记待接线。
- `ui_rotate_circle` 0x1409ac910 / `create_circle` 0x14097e560 / `ui_create_circle` 0x14165bed0 / `ui_edit_circle` 0x14165d460 / `show_full_circle_signifier` 0x141660880 / `create_rectangle` 事件循环前半：大 UI 系统函数，待分批逐句。

### 自检

- `cargo check -p country-core` 通过；`cargo test -p country-core --lib shape_signifier_lerp` 2 passed（重建后）。
- 改动文件：`systems/wall/shapes/utils/shape_signifier_lerp.rs`。未改他模块。

---

## 批 4 — eraser finish 11509B 反编译定案（2026-10-17）

依据：`decomp/batch3/0x14092f990-finish.txt`（fresh，11509B）。文件：
`systems/wall/eraser/archivist.rs`。

### fresh 结构（逐分支）

1. 遍历 archivist 事件 map（OriginalWallId→RewireEnclosedWall @+0x30、+0xc 等；EraserWallState），
   每事件读 0xd8B 载荷 → **组装 0xf8B WallHistoryEdit 记录** 推入 **5 个 Vec 桶**
   （local_5b8/5a0/588/570/558，各 grow_one<WallHistoryEdit>）。
2. shrink/split 分支：ChildWalls `sort_by`（smallsort/driftsort）→ 逐段
   `fVar66 = cur[+8]*0.5 + prev[+0xc]*0.5`（中点，[0,1] 断言 + NaN panic）→
   `Curve::crashy_slice<Vec2,2D>` → `inverse_lerp ×2`（NAN/INF panic）→ clamp。
3. Sip13 hash（`core::hash::sip::impl$5::write` 展开）→ inner walls map 探针
   （-0x78 stride）→ 命中则装配 0x70B ChildWall 改写记录。
4. 末段：5 桶 Vec → **Arc\<WallHistoryEdit\> 逐条包装 → from_iter\<ConcurrentEdits\> →
   `resources::history::History::add()`**；随后清 archivist 两 map
   （RawTable::clear ×2）。

### verdict

**未实现-需重做（登记，跨域）**：原生 finish 在 native archivist 事件 map
（RewireEnclosedWall/EraserWallState）+ ChildWalls 重接线配对 + ConcurrentEdits/
History::add（history 域）上工作；Rust `finish_wall_eraser` 为单遍
`record_wall_bulldoze` 转发（affected/ranges 简化模型），结构不逐条对应。
**ChildWalls 重接线为宿主边界**（REVIEW-DEEP 登记）。重建需：① native archivist
事件结构；② ChildWalls 排序/中点配对；③ 5 桶 WallHistoryEdit→ConcurrentEdits→
History::add 链（history 域类型已存在，但组装语义未对）——跨域登记，不做简化冒顶。

### 自检

- 本批无改动文件；`cargo check -p country-core` 通过。

---

## 批 6 — archivist 迭代链 5 / telegraph / snap_to_roof_ridge / publish_public_wall_state 登记（2026-10-17）

依据：fresh 反编译 `impl-walls-remainder/decomp/batch5/`（2026-10-17 当日生成）
0x140b2cb30 / 0x140b2be70 / 0x1412b2a70 / 0x14123b5c0 / 0x1412b4f10 / 0x1412b3910 +
REVIEW-DEEP/REBUILD-DEEP 对 telegraph/snap_to_roof_ridge/publish_public_wall_state 的
fresh 证据。**按 AGENTS「库外游戏接线/他模块」分类登记，不做简化冒顶。**

### 6.1 archivist 迭代链 5（跨域登记，非"因大"）

| VA | 函数 | fresh 结构 | 登记原因 |
|---|---|---|---|
| 0x140b2cb30 | extract_diff | 清 2 碰撞 map → memmove before@+0xc0/after@+0x150（0x90B Decorators）→ 三路 chain（walls Drain + special1/2 IntoIter）`extract_archived_decorators` → `from_iter<HashMap<DecoratorStorageAddr,(AddrHistoryState,AddrHistoryState)>>` → filter_map closure → 输出 `HashMap<Addr, DecoratorStorageHistoryDiff>`；post 断言（walls/before/after len==0 panic） | 输出/中间类型 **DecoratorStorageHistoryDiff / DecoratorStorageAddrHistoryState 属 resources/history 域且 Rust 未实现**；Rust 无 before/after 0x90B 区 |
| 0x140b2be70 | extract_before_after_impl | 存储 map 探针（-0x98 stride）→ 空返回/预览路径分支（"Found a preview decorator..." panic 0x142aeb540）+ BeVy Query（FUN_140d4d740/140d64e00）+ tracked_clone_from | 原生 preview 判别 + BeVy Query 系统级调用；Rust 无对应 |
| 0x1412b2a70/0x1412b3910 | archive_storage_decorators<&str/String> | 按存储判别 0=墙/1=special1/2=special2：墙 → 源 map 探针（已存在则 return）+ reserve + `tracked_clone_from` → 插入 archivist walls；special1/2 → 已存在 return + clone 入 +0x18/+0x2a 区；随后 **collision collider/mesh 重建**（get_collider/get_collider_mesh → HashMap insert `(DecoratorId,(OffsetColliderTransform,RaycastMesh))`） | collision 域 collider/mesh 类型 + archivist 布局差异；Rust `archive_current_decorators` 为 Vec 简化读路径 |
| 0x14123b5c0 | archive_current_decorators_in_storage | 经 `DecoratorStorage::get`(0x140a1c2c0) 逐地址 → tracked_clone_from → 插入 archivist + collision 重建（6741B） | 同上（collision 域 + archivist 布局） |
| 0x1412b4f10 | archive_current_decorators_in_storages | 多存储批量归档（3824B） | 同上 |

结论：**5 项均为跨域/模型差异登记**（历史 diff 类型 + collision collider/mesh + native
archivist before/after 布局）。Rust 侧可闭合的 in-domain 核心（archivist walls/special
插入副本语义）与 `tracked_clone_from`（已重建判等）接续；但完整链需 history 域
DecoratorStorageHistoryDiff + collision 域 RaycastMesh 类型，属他模块。不"因大"登记，
按类型边界登记。**待分批：接入 history 域 diff 类型后可实现 extract_diff 主逻辑。**

### 6.2 telegraph / snap_to_roof_ridge / publish_public_wall_state（宿主边界登记）

| VA | 函数 | fresh 证据 | 结论 |
|---|---|---|---|
| 0x14086ad70 | telegraph_walls_are_merged | 事件驱动：public wall 三索引点（+0x118/+0x110/+0x120）→ curve try_new_from_points/slice_into + easing + despawn + UiAppendSignifier 实体链（4723B） | render-host 实体链边界（AGENTS 外部依赖分类）；Rust 单墙近似——跨域登记（REBUILD-DEEP 同款） |
| 0x140980db0 | snap_to_roof_ridge | Roof::pivot_3d / RoofRidgeDims::calc 山脊几何步行（1494B） | roof 域 host wiring——跨域登记（REBUILD-DEEP 同款） |
| 0x140a8b530 | publish_public_wall_state | 11198B；主逻辑（路径栅格 → public.consume + dirty 清 + OnWallChanged/Deleted 事件）在 recovered_building wall_collection 域 | Rust 为 consume_with_paths 包装；主逻辑属 recovered_building 域——跨域登记（REBUILD-DEEP 同款） |

### 6.3 建议 DISPATCH 流向

- archivist 链 5 与 eraser finish 需 history 域（DecoratorStorageHistoryDiff/
  ConcurrentEdits/History::add）+ collision 域（RaycastMesh/OffsetColliderTransform）
  类型接续；建议 coordinator 派 history/collision 域补齐后回派 walls 实现链。
- telegraph/snap_to_roof_ridge/publish_public_wall_state 为 render-host/roof/
  recovered_building 宿主边界，维持登记不冒顶。

---

## 批 5 — decorator_storage/wall_spatial_hash/decorator_archivist 余量（2026-10-17）

依据：`decomp/batch5/*.txt` + disasm（`batch5b-disasm-result.json`，fresh）。

### 判等（fresh 复核一致）

| VA | 函数 | 依据 |
|---|---|---|
| 0x140a1c3e0 | lookup_storage | fresh：+0x78 d2s map 探针（+0x60 ctrl/+0x68 mask，-0x18 stride），命中返回，**miss → panic_fmt**（DecoratorStorage::fmt 0x1408c3600？）——REBUILD-REMAINDER §3 登记「native miss panic / Rust Option」fresh 复核一致 ✓ |
| 0x140a1dba0 | remove_storage | immutable 门 + per_wall 桶移除 + d2s 索引清理（dealloc/panic 结构）；REBUILD-REMAINDER §2 判等复核一致 ✓ |
| 0x140a1e720 | clear_modification_tracking | per_wall 逐桶 clear + d2s 清理；§2 判等复核一致 ✓ |

### 判等-注意

| VA | 函数 | 差异 |
|---|---|---|
| 0x140aa5150 | WallSpatialHash::new | cells **0x1000=4096**（Vec extend_with）+ segments HashMap + RandomState + **extent {130,130}@+0x50**（= Rust WORLD_SIZE ✓）；+0x48 常量 `0x4000000040`（{0x40, f32 2.0}）字段语义待取证（Rust GRID_SIDE=64/CELL_COUNT=4096 对应） |
| 0x140aa50e0 | WallSegment::terrain_height_clamped | fresh disasm：`t = clamp01(((cursor-e0)·(e1-e0)) / max(|e1-e0|², eps))` → `lerp(terrain[0], terrain[1], t)`（0x142a7baa0 eps、0x142925904=1.0）；Rust 无对应方法（WallSpatialSegment 携带 terrain_heights 无 consumer）——登记 |

### 登记（native-only / 未实现）

| VA | 函数 | 分歧点 |
|---|---|---|
| 0x140b2bb00 | all_decorators | native-only 迭代器描述符（walls map + before@+0xc0/after@+0x150 双区判别 + pmovmskb 位图）；Rust 无 before/after 区——glue/native-only 登记 |
| 0x140b2bb80 | all_storages | native-only 迭代器描述符（同上）——glue/native-only 登记 |
| 0x140b2bc00 | decorator_cull_allow_wall | native 探针 walls map（-0x98 stride）→ RawIterRange::fold 置 cull 放行；Rust 无对应（仅 allow_any/is_allowed）——未实现登记 |
| 0x140b2bd30 | decorator_cull_allow_stairs | native Iterator::fold 转发——未实现登记 |
| 0x14155ba30 | restore_decorators | native 4438B 历史 replay_pop_bucket 恢复（get_or_insert_default + 逐记录回填）；Rust `restore_decorators<F>{restore(self)}` 为占位 passthrough（无调用方）——未实现-需重做登记 |

### 自检

- 本批无改动文件；`cargo check -p country-core` 通过。

<!-- 每批追加一节 -->
