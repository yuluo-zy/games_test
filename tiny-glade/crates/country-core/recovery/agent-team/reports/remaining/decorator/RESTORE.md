# RESTORE — remaining/decorator（restorer-decorator-remaining，2026-10-17 晚）

> 角色：decorator 剩余实现（续作；上一会话 402 中断）。
> 硬标准：每函数 fresh 反编译/反汇编 → 逐语句判等；还原不了标未实现并给分歧点/解锁条件。
> 工具：ghidra `--worker shared`（只读，含 `disassemble_function`/`read_memory`/跳表解码）+
> radare2 唯一 worker `restorer-decorator-remaining`（RNE 指令 oracle）。
> 身份：B patched EXE `cf048e27…`，`program=tiny-glade.exe`。
> 证据目录：`work/restorer-decorator-remaining/evidence/`
> （`job1..job15-result.json`、`decomp/*.c`、`disasm_*.txt`）。

## 0. 结论速览

| 函数（VA） | verdict | 本轮交付 |
|---|---|---|
| `apply_decorator_op` 0x141276e30 | **判等**（控制流/历史/存储/事件）；归档叶 U | 修 remove 分支多登记 locator 的分歧 |
| `decorator_record_history` 0x140817850 | **判等**（清标志/排空循环/终止哨兵/双重清空）；48B 载荷映射 **未实现**（core 存档模型缺口） | fresh 复核，无代码改动（登记精确化） |
| `get_gothic_hat_bones` 0x1407eb500 | **判等**（243 条反汇编逐语句重建；side/edge 已按 tree-forest FIX 修正，side[2] 按机器码保留 `-dy1/len3`）；墙侧 raw 绑定 U | 从空体重建完整骨骼点生成 + FIX |
| `DecoratorBlueprint::set_interaction_mesh` 0x1408255e0 | **判等**（finite 断言 + 字段写面） | 修 panic-vs-静默、补 rotation/Arc 字段 |
| `DecoratorBlueprint::add_door_stairs` 0x140825830 | **判等** | 确认 0x20B 8×u32 替换语义 |
| `DecoratorBlueprints::{get,insert}` 0x140825ff0/0x1408258b0 | **胶水-等价**（hashbrown 探针/插入） | 无需改 |
| `load_decorators` 0x1407b66e0 | **重建**（扩展名门/stem outline/白色 add_color/panic 语义/JSON 加载）；注册叶 **未实现**（core 无 `load_mesh_into_library` 具体实现） | 整段重写（原为实现近似） |
| `load_collision_as_bbx` 0x1407bb4b0 | **未实现**（new_cuboid/box_trigger_volume 缺口） | 登记 |
| `create_or_restore_decorator_backup` 0x1408238a0 | **部分判等**（状态机/门控）；RESTORE 叶/三辅助 map **未实现** | fresh 复核（未改） |
| `DecoratorBackup::clear` 0x140823660 | **判等** | fresh 复核 |
| `restore_archived_decorators` 0x140817a40 | **部分判等**；archivist/d2s 叶 U | fresh 复核（未改） |
| `convert_raycast_hit_to_decorator_dst` 0x14127c990 | **判等**（跳表 17 项逐项解码） | 补 disc 3/Other、StairTop 8B 载荷 |
| `calculate_move_dst` 0x1412801f0 | **部分判等**（派发键 = `u20` 32 位）；处理器跳表目标 U | 修正索引来源 |
| `show_decorator_remove_signifier` 0x14127fe10 | **重建**（距离/位置/命名/工厂写面）；marker/rotation helper U | 重写近似体 |
| `try_get_grab_offset_ss_if_valid` 0x141277bb0 | **未实现**（跳表处理体未恢复） | 索引式判等 |
| `xz_projective_rotation_bad` 0x140c911d0 | **未实现-需重做**（crate 现为 Mat3 近似，机器码写 16B quat） | 新证据登记（见 §6） |

**计数**：判等 8（含 2 个胶水等价）／重建 4／部分判等 2／未实现 4（含 3 个 U 叶与 1 个 helper 需重做）。
`cargo check -p system-decorator`：**Finished，0 error**（6 warning 均为既有项）。
`cargo test -p system-decorator --lib`：**4 passed**。

---

## 1. `apply_decorator_op` 0x141276e30 — 判等（修正一处分歧）

fresh 全文（`evidence/decomp/0x141276e30_apply_decorator_op.c`）：

- guard：`(archivist_is_none & kind_byte) != 0 → panic`（`&` 位与 ⇒ `assert!(archivist.is_some() || kind_byte == 0)`）；
- `l = 0; if (op[0] - 3u) < 2 { l = op[0] - 2 }` ⇒ 3→1(place)、4→2(remove)、其余→0(change)；
- change：`sub = op[+0x10]`（= `info.dst_secondary`）→ 归档一次；`sub != 3` 再归档一次；history 存在 → push kind=3 + `DecoratorLocator::new` + `HashMap::insert`；`apply_decorator_change`；
- place：归档一次；push kind=4 + locator + insert；`apply_decorator_place`；
- remove：归档一次；push kind=5（**无 locator/new/insert**）；`apply_decorator_remove`；
- 末尾 `DeferredDecoratorOpEvents::send`。

**分歧与修正**：旧 `write_remove_op_history` 对 remove 也调用
`add_potentially_surviving_affected_decorator`（机器码 remove 分支无 `DecoratorLocator::new`/`insert`）。
已删；签名改为 `(history, op)`。其余（归档在历史之前、change 二次归档条件、存储应用、事件发送）逐条对上。

**残留 U**：`archive_current_decorators_in_storage` 等价叶（core `DecoratorArchivist` 字符串键桶缺失）、
`build_op_event` 0x88B 尾部字段放置、`DecoratorLocator::new` 实参（机器码未暴露入参）。

---

## 2. `decorator_record_history` 0x140817850 — 判等 + 48B 载荷未实现

fresh 全文（`evidence/decomp/0x140817850_decorator_record_history.c`）逐语句：

1. `pending = *[res+0x48]; *[res+0x48] = 0;`（读即清，**无条件**）；
2. `pending != 0`：
   - `len = [res+0x10]; [res+0x10] = 0`（记录 Vec 长度清零）；
   - 逐元素（stride 0x20）读 `{kind=w0, f1=w1, f2=w2, f3=w3}`；
   - `kind == 6 → break`（终止哨兵）；
   - `__rust_alloc(0x30)` → 写 `{1,1,kind,f1,f2,f3}` → `History::add`（EditType tag 10）；
3. 循环后再次 `[res+0x10] = 0`；
4. `[res+0x30] != 0` → `memset([res+0x18], 0xff, len+0x11)` + 重置 hashbrown 容量（= `HashMap::clear`）。

Rust 对照：读清标志、排空、哨兵 break、无条件清 Vec/HashMap 全对上。
**未实现**：48B 运行时载荷 `{1,1,kind,f1,f2,f3}` 无法装入 core
`DecoratorHistoryEdit { diffs: HashMap<...> }`（存档模型）；`record_to_history_edit` 仍返回空 edit。
解锁条件：core 落 48B 运行时编辑载荷模型（或证实 diffs 等价）。

---

## 3. `get_gothic_hat_bones` 0x1407eb500 — 判等（243 条反汇编）

证据：`evidence/decomp/0x1407eb500_get_gothic_hat_bones.c` +
`evidence/disasm_0.txt`（完整 243 条）+ 常量 `read_memory`（`evidence/job3-result.json`）+
两处调用点上下文（`evidence/job4-result.json`，RNE）。

参数面（调用点 ABI 实测）：`(out 60B, wall{curve@+0, i32@+0x258, f32@+0x264}, decorator{rank@+0x44}, u, y, half_base)`。

逐语句重建（`decorator_visual/gothic_wall_window.rs::get_gothic_hat_bones`）：

- rank ∈ {1,2,3}，否则 panic（行 0x28）；
- `wall_flag == 0`：`t = inverse_lerp(0.95, 1.6, wall_value)`；
  rank1 `half += -0.05`；rank2 `half -= (1-clamp01(t))*0.21 + 0.1`；
  rank3 `half -= (1-clamp01(t))*0.158 + 0.15`；
  `wall_flag != 0`：rank1/2 `-=0.05`、rank3 `-=0.1`；
- `half *= 0.5`；
- `P1 = to_world_space({project_clamp(u-half), y})`、`P2 = ...(u+half)`、
  `P3 = to_world_space({u, y})`（**中心不 clamp**）；
- 三点各 `clamp01(get_approx_u_from_pos(xz)) → get_coord_at_u` 取段
  （`seg+1` 越界 → panic_bounds_check），取 `{a,b}`/`{c,d}`；
- 输出（60B，2026-10-17 二次复核 + 按 tree-forest FIX 修正）：
  `side=[-dy1/len1, dx1/len1, -dy1/len3, dx3/len3]`、
  `first=xvy(P1.xz,y)`、`center=P3`、`second=xvy(P2.xz,y)`、
  `edge=[-dy2/len2, dx2/len2]`；
  - `edge` 由 `b2.x` 改为 `dx2`（段 2 单位切向；0x1407eb901-90e
    `MOVSLDUP XMM8(1/len2) × UNPCKLPS(XMM13{-dy2,-dy2},XMM7{dx2,dy2})` 证实）。
  - `side[1]` 由 0 改为 `dx1/len1`；`side[3]` 由 `inv3²` 改为 `dx3/len3`。
  - **`side[2]` 与 tree-forest FIX 有一处差异**：FIX 写作 `-dy3/len3`，但字节级
    复核（原始机器码 `45 0f 16 cc` MOVLHPS、`45 0f c6 cb 82` SHUFPS，见
    `work/restorer-decorator-remaining/jobs/job16.json` + 通道模拟）证明
    lane2 = `-dy1/len3`（MOVLHPS 写 lane2=`-dy1`；SHUFPS 0x82 的 lane2 取
    「src[0]=-dy1」，非 src[2]）。已按机器码落 `-dy1*inv3` 并在类型注释留证。
- 常量核对：0.95/1.6/0.21/0.158/0.1/0.15/0.05/0.1/0.5 均 `read_memory` 核验。

**U**：`PublicWallState` raw 偏移 `+0x258/+0x264/+0x44` 未在 country-core 语义模型暴露，
以 `GothicHatWallView`/`GothicHatParams` 显式承载；调用点接线（generate_*）仍 U。
NaN 语义：`MAXSS/MINSS` 序列与 `f32::clamp(0,1)` 一致（含 NaN 传播）。

---

## 4. blueprint 族

### 4.1 `DecoratorBlueprint::set_interaction_mesh` 0x1408255e0 — 判等（已修）

fresh（`evidence/decomp/0x1408255e0_set_interaction_mesh.c`）：

- `xyz` 三 f32 与 `rotation` 四 f32 都必须有限；**否则 panic_fmt**（`assert!`）。
  旧实现"非有限则保持原值"是 panic-vs-return 反向（检查表 #1），已改为断言；
- 旧 Arc `+0x90` 先减引用，归零 `drop_slow<RaycastMeshData>`；再写
  `+0x80=0`（Some 判别）、`+0x84` Vec3、`+0x90` Arc、`+0xa0` 四元数 16B。
- 参数面实测：`(self, mesh: Arc<RaycastMeshData>, xyz: &Vec3, rotation: &Quat)`。
  已按此重写 `InteractionMesh { xyz, mesh, rotation }`（旧 `Arc<()>`/`(usize,usize)` 载荷为错位近似）。

### 4.2 `add_door_stairs` 0x140825830 — 判等

`__rust_alloc(0x20)`；拷贝入参 4 qword；`+0x120` 旧块 dealloc 后替换。
Rust `Box<[u32;8]>` 等价。

### 4.3 `get` / `insert` 0x140825ff0 / 0x1408258b0 — 胶水-等价

`get` = hashbrown `hash_one` + 控制字节探针 + 键比较（返回命中槽），
Rust `HashMap::get` 等价；`insert` 16KB 主体为值搬移/扩容。
不计独立业务（AGENTS 胶水规则）。

---

## 5. `load_decorators` 0x1407b66e0 — 重建 + 注册叶未实现

fresh（`evidence/decomp/0x1407b66e0_load_decorators.c`，含坏跳表警告，主体可读）：

1. `read_dir`；失败 → anyhow context + `unwrap_failed`（panic，**不是静默 return**）；
2. 迭代：`entry.unwrap()`；`file_name → to_str`（`try_from` 失败 → unwrap_failed）；
3. 扩展名门：字节长度 > 4，末 5 字节 bswap 后 `".jso"` 且末字节 `'n'`（= `.json`）；
4. `entry.path()` → `load_json_as_mesh<PathBuf>`（失败 panic）；
5. stem（去 5 字节扩展名）== `"outline"` → `Mesh::add_color(Vec3::ONE)`（0x3f800000×3）；
6. `format context` + `load_mesh_into_library`（0x140b1aca0）。

已重写：`load_decorators<H: DecoratorMeshHost>(dir, host)`，错误 panic、扩展名/stem/白色、
`country_core::utils::load_json::load_json_as_mesh` + `Mesh::add_color` 全按机器码。
**未实现**：`country_core::startup::load_mesh_into_library` 在 core 只有 trait 形态调用点，
无具体资源实现 → 以 `DecoratorMeshHost` 宿主注入（U）。
原实现直接 `DecoratorMeshLibrary::insert` 且静默跳过错误，已删除。

`load_collision_as_bbx` 0x1407bb4b0：`new_cuboid` / `box_trigger_volume` /
Rectangle3d 尾段未在 core 落库 → **未实现**（返回 default 的占位仍为 U，本波未改）。

---

## 6. `xz_projective_rotation_bad` 0x140c911d0 — 新证据：现实现形状错误

反汇编（`evidence/job10-result.json`，133 条）：函数**只写 16 字节**（`MOVAPS [RAX],XMM3`），
输入 `&Vec3` 仅读 `+0`（x）与 `+8`（z）；算法 = `normalize_or_zero(x,z)` +
四分支四元数合成（常量 1.0/0.5/±0.0）。

而 crate `utils::xz_projective_rotation_bad` 现返回 **Mat3**（近似体，注释亦标 O 但形状不符）。
**未实现-需重做**：按上述 4 分支 SIMD 公式重建为 `Quat`；解锁条件 = 完整公式逐分支
（XMM 通道语义）复原。该 helper 被 arrow_slit / cottage_* 等视觉族消费，本轮不改以免破坏在途消费者。

---

## 7. backup / restore 链（fresh 复核，未改）

- `DecoratorBackup::clear` 0x140823660：`impl$3::default` + 逐字段 `clone_from`；
  `*self = default()` 不可观测等价 → **判等**。
- `create_or_restore_decorator_backup` 0x1408238a0（fresh 23KB）：
  - `state = [+0x282]; [+0x282]=0`（读即清）；
  - state==1：`all_decorators()` → `spec_extend<DecoratorHistoryEdit>` → `[+0x281]=1`；
  - state==2 且 `[+0x281]!=0`：先读 `history+0x48` 门控 → `all_storages()` →
    `get_mut` + `Decorators::clear<DecoratorToStorageMap>`；
  - 末尾 `[+0x280] != 0` → 备份重置为 default。
  Rust 状态机/门控方向一致；**未实现**：三 0x30B 辅助 map 反向拷贝（+0x1d0/+0x200/+0x230，
  live 资源身份 U）与 `Vec<DecoratorHistoryEdit>` 辅助槽（当前 `BackupAuxiliary` 为 U 占位）。
- `restore_archived_decorators` 0x140817a40：结构面一致；archivist `all_storages/all_decorators/
  extract_diff` 与 `d2s insert/update` 叶仍 U。

---

## 8. ui_systems

### 8.1 `convert_raycast_hit_to_decorator_dst` 0x14127c990 — 判等

fresh 反汇编 + 跳表 0x142c24118（17 项，`evidence/job14-result.json`）逐项解码：

| disc | 目标 | 行为 |
|---|---|---|
| 0 | 0x14127c9c3 | tag 0 + u64@+8 |
| 2 | 0x14127ca18 | `hit[1]==0 || can_place_on_roof()` → tag 1 + sub@+2 + u64@+8；否则 tag 4 |
| 3 | 0x14127ca51 | `panic`（标准 "internal error: entered unreachable code"，file 行 40） |
| 5,9,12,15,16 | 0x14127ca51→0x14127ca69 | panic_fmt（详细 "Decorators should ignore…" 消息） |
| 11 | 0x14127c9f5 | `can_place_on_terrain()` → tag 2 |
| 13 | 0x14127c9d0 | `can_place_on_stair_top()` → tag 3 + `hit+4` 的 **8 字节** |
| 1,4,6,7,8,10,14 | 0x14127c9b0 | tag 4 |

已补 `RaycastHitDisc::{Unsupported, Other}` 与 `StairTop(u64)`；panic 消息/分支与机器码一致。

### 8.2 `calculate_move_dst` 0x1412801f0 — 部分判等

反汇编：`EAX = *(u32*)(info+0x20)` → 跳表 0x142c248b0。索引来源已由 `dst_tag()`（+0x22 字节）
改为完整 `info.u20`。跳表目标未被 Ghidra 恢复 → 处理器语义 **U**（保留 tag→种类映射并登记）。

### 8.3 `show_decorator_remove_signifier` 0x14127fe10 — 重建

fresh 110 条（`evidence/disasm_1` 部分 + `job7`）：

- `dist = |pos - cursor|`（3D，sqrt(dx²+dy²+dz²)）；
- 位置 = `pos + dist * dir`（dir = RDX+0xc 的 Vec3）；
- rotation = `xz_projective_rotation_bad(dir)`（B 输出 16B；crate helper 形状 U，见 §6）；
- 名字 = `format!("despawn_decorator_{:?}_{:?}", id, pos)`（格式片 `"despawn_decorator_"` + `"_"`；
  id 与 pos 均用 **Debug**，已由 fmt 实现 0x14127fdd0/0x140d4ba00 证实）；
- 48B Arc 载荷 `{rotation 16B, position Vec3}`；
- `SignifierFactory::new(name, inner=Arc, marker)` → `UiSignifierStream::write`。

已重写；**U**：marker 在 B 为静态 0x142c24800（本实现填 0）、helper 形状。

### 8.4 `try_get_grab_offset_ss_if_valid` 0x141277bb0 — 未实现

索引式判等：`idx = (*(u8*)(info+0x22) - 2) < 3 ? tag-1 : 0`（当前 Rust 一致）；
跳表 0x142c23990 目标未恢复 → 处理体 **未实现**。

---

## 9. 证据索引

- `evidence/decomp/0x141276e30_apply_decorator_op.c`、`0x140817850_decorator_record_history.c`
- `evidence/decomp/0x1407eb500_get_gothic_hat_bones.c` + `evidence/disasm_0.txt`（243 条）
- `evidence/job3-result.json`（常量 0.95/1.6/0.21/0.158/0.1/0.15/0.05/0.1/0.5/sign mask/1.0）
- `evidence/job4-result.json`（get_gothic_hat_bones 两处调用点 ABI）、`evidence/job10-11`（rotation helper）
- `evidence/decomp/0x1408258b0_insert.c`、`0x140825ff0_get.c`、`0x140825830_add_door_stairs.c`、
  `0x1408255e0_set_interaction_mesh.c`
- `evidence/decomp/0x1407b66e0_load_decorators.c`、`0x1407bb4b0_load_collision_as_bbx.c`
- `evidence/decomp/0x1408238a0_create_or_restore_decorator_backup.c`、`0x140823660_clear.c`、
  `0x140817a40_restore_archived_decorators.c`
- `evidence/decomp/0x14127c990_*.c` + `evidence/job13/14`（跳表 17 项）、`0x1412801f0_calculate_move_dst.c`、
  `0x14127fe10_show_decorator_remove_signifier.c`、`0x141277bb0_*.c`

## 10. 遗留（ledger-ready）

1. **core**：`DecoratorHistoryEdit` 48B 运行时载荷模型（decorator_record_history）。
2. **core**：`startup::load_mesh_into_library` 具体实现（load_decorators 注册叶）。
3. **re-analysis**：`xz_projective_rotation_bad` 4 分支四元数公式（现 crate 为 Mat3 近似）。
4. **re-analysis**：`calculate_move_dst` 跳表 0x142c248b0 目标；`try_get_grab_offset_ss_if_valid`
   跳表 0x142c23990 目标。
5. **U**：`PublicWallState` raw `+0x258/+0x264/+0x44` 绑定（get_gothic_hat_bones 调用点）。
6. **U**：`SignifierFactory` marker 0x142c24800 / Arc 载荷所有权（show_decorator_remove_signifier）。
7. backup RESTORE 叶：三 0x30B 辅助 map 反向拷贝、`Vec<DecoratorHistoryEdit>` 辅助槽。
