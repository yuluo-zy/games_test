# ivy 模块真实逻辑重建报告（rebuild-ivy，2026-10-17）

> 范围：LOGIC.md 判等的 8 个 `未实现-需重做` 业务函数 + `IvySpawnNoise::new`
> （结构判等、消费方 spawner 重建复核）。按 2026-10-16-logic-verify-protocol.md
> 的分类规则先分类、再按 fresh 反编译（ghidra `--worker shared`，
> `fields=decompiled_code,callees`，2026-10-17 00:53）与 RNE 机器码逐语句重建。
> 证据落点：`planning/logic-ivy/decomp/rebuild/*.c.txt`（fresh）、
> `planning/logic-ivy/results/rebuild-fresh-batch1.json`、
> `reverse-engineering/tiny-glade/evidence/country-core-deep/collision-eco/ivy-grower-disasm/`。

## 一、函数分类（协议 2026-10-17 补充：宏生成胶水 vs 真实业务）

| 函数 | VA | 分类 | 结论 |
|---|---|---|---|
| WallCoord::into_world_space | 0x140b199e0 | **真实业务** | 曲线插值+法向偏移算法，重建 |
| ivy_grower（含守卫体 0x14094ac51） | 0x14094a910 | **真实业务** | 10 相分派+生长闭包，重建新段提案/墙 panic |
| intersect_ivy_growth_w_wall_segment | 0x14094cc60 | **真实业务但死代码** | 原版算法完全不同且原/Rust 均无调用点；核对后**删除** |
| ivy_spawner | 0x14094cf90 | **真实业务** | 7 参签名+门/预算/分派/两路径，全重建 |
| ivy_pruner | 0x140997410 | **真实业务** | 尾带门+整株删除+脏墙输入，重建 |
| is_point_still_valid | 0x140998dc0 | **真实业务** | 返墙指针+panic+样点 AABB，重建 |
| ivy_send_to_gpu_leaf | 0x1408c1580 | **半胶水半业务** | 业务=scale 位+48B 打包；框架=ssbo 上传，重建业务部分 |
| ivy_send_to_gpu_segments | 0x140a9b4e0 | **半胶水半业务** | 字段映射按 shader 定案（wall_normal/arch_height 已证）；上传为 render 域接线（保持库内镜像） |
| IvySpawnNoise::new | 0x1409440c0 内联 | **真实业务** | 结构判等维持；阈值 -0.02 经 spawner 噪声门复核（常量注记） |

宏生成胶水（不重建，维持）：add_ivy_systems / insert_ivy_resources /
add_ivy_events（Bevy 注册框架）、remove_ivy_at_location（薄事件生产者）。

## 二、逐函数重建

### 1. into_world_space（0x140b199e0）——Y 分量修正

- 分歧：Rust 原实现 `self.v ± offset_z`（RESTORE FIX-1 误读，把 offset_z 加到 Y）。
- 证据：RNE 0x140b19b72..ba7。`unpcklps xmm3, xmm8`（xmm8 = WallCoord.v）把 v 放入
  out.y；x0y 提升 normal 为 `(nx,0,nz)`，offset 向量 y 槽恒 0；`subps/addps` 只改 x/z。
  out.x = interp.x ± nx·0.315、out.y = v、out.z = interp.z ± nz·0.315。
- 改：`wall_coord.rs` `into_world_space` 输出改为
  `Vec3::new(x ± offset_x, self.v, z ± offset_z)`（side 真 → 减，假 → 加）。

### 2. ivy_grower（0x14094a910）——新段提案 + 墙有效 panic

- 分歧（LOGIC ③①②）：
  - 去掉 Rust `u_next = u + SEGMENT_DURATION/length` 推进（0x14094b59d 直接用 u）。
  - WallCoord.u = `wall_height*u + lateral`（0x14094b5a5 mulss + 0x14094b703 addss），
    lateral = `±direction.x·0.21`（0x14094b65c 常量 0x142ab3194，符号由 0x14094b6d9..b6ff
    尖端-段比较选出；本实现以调用方 `side` 符号重建）。
  - WallCoord.v = `tail.y + direction.y·0.21`（0x14094b661/665，[rbp+0x60]=tail.y）。
  - **墙有效门改为 panic**：`段数<2 或 高<=0` → `panic`（0x14094c1e3 主路径、
    0x14094c200/0x14094c21d 墙跳变两候选），Rust 原为 `return` 跳过，三处同改。
- 改：`ivy_grower.rs` `get_growth_point_proposal` 重写公式；`propose_new_segment`
  主路径与墙跳变路径墙有效检查改 `panic!`。
- 其余相分派/守卫位/冷却/advance_growth/高度带保持判等，未动。

### 3. intersect_ivy_growth_w_wall_segment（0x14094cc60）——删除

- 核对：fresh 反编译显示原版为 `intersection_aabb_segment2` + 投影 + 偏移盒
  （0.707/1.414/0.63·0.5）取近点，与 Rust `perp_dot` 射线-线段求交**算法完全不同**。
- 调用点：grower 全相体反汇编无 `call 0x14094cc60`（该地址仅为本函数入口）；Rust
  全 src 仅定义无调用。**两域均死代码** → 按任务"要么按真实实现要么删除"择**删除**。
  保留 `intersection_aabb_segment2`（utils 已有）为后续接线预留说明。

### 4. ivy_spawner（0x14094cf90）——全重建

- 签名（add_ivy_systems 0x140944070 定案，7 参）：
  `(ResMut<IvyStorage>, Res<IvySpawnNoise>, Res<PublicWalls>,
   Res<CursorTerrainRaycast>, Res<ActionSetStates>, Res<WallSpatialHash>,
   Res<GladeSettings>)`。
- 首门（0x14094cffb..d00a）：`[GladeSettings+0x1c]==1`（`ivy_leaves`），**非 AppMode**；
  删 Rust 凭空加的 `actions.digital[36].pressed()` 附加门。
- 预算（0x14094d09b..a7）：逐 ivy 累加 `acc = total-(n-1)`，`acc+0x12 > 39999 → return`。
- 分派（0x14094d2e7）：`MainActionSet.digital[36].disc < 3`（`pressed()`）→ 光标路径，
  否则随机路径。
- 垂直 u（两路径共享）：`(rand*1.5-0.75)/wall_height + clamp(approx_u)`，
  除数=墙高（Rust 原 A 用 /1.0、B 用 /curve.length，均错）。
- 侧向：幅度 ±0.34（0.68×0.5）；A=`Curve::is_on_right_side` 选号，B=仅 `subtype>1`
  才 `get_bool`（否则恒 +0.34）。
- root 分离：`facing_dot>=0 && dist²<thr` 双条件（0x14094d8dd/0x14094d95d）。
- attempts：共享 `2+usize_less_than(3)`（原 A 固定 3 错）。
- 格内 `is_inside_bbx` 复核（0x14094d..）：根在自身格内某墙段 AABB 内 → 拒绝本次
  （旋转细节以 XZ 轴对齐包含重建，见注释）。
- 每尝试预算（0x14094db6e..b9e）：`0x9c41-acc`、`cap=min(0x46,…)`、`acc+=cap-1`，
  `0x13<… && acc<0x9c41` 才 spawn。
- 路径 A 改走空间哈希格（补 `Res<WallSpatialHash>`）；噪声门严格大于阈值维持。
- 改：`ivy_spawner.rs` 整文件重写（commit_root 共享提交、两路径、7 参系统）。

### 5/6. is_point_still_valid（0x140998dc0）与 ivy_pruner（0x140997410）

- `is_point_still_valid` 返回值由 bool 改为 **`Option<WallId>`**（返所属墙，供 pruner
  尾带门；0x140998dc0 `local_118 = lVar10`）。
- **墙有效 panic**：`段<2 或 高<=0` → `panic`（Rust 原 return false）。
- 插值 u 带 `(-0.01/h, 1+0.01/h)` 维持。
- **样点 ±0.1 AABB 复核**（0x140998dc0）：点 XZ 必须落入所属墙某地形样点 AABB；
  地形样点栅格以简化曲线 0.5m 采样 + 半格 + 0.1 重建（`ivy_spawner::point_inside_wall_terrain_samples`，
  网格分辨率待取证）。
- `ivy_pruner`：tail 有效后追加**高度带门** `tail.y >= arch + (flag?+0.2 :
  subtype<2?(-0.79-0.1):-0.46)`（区别于 grower 提案带 -0.79+0.2）；根环结束后对空
  ivy **`remove_entry` 整株删除**（0x140998711..1312）。
- 脏墙输入：Rust 维持 `EventReader<OnWallChanged>` 为 crate 侧队来源；原版 Circle2d/
  Rectangle2d 包含 + FinalizedWallHoles 的脏墙过滤为跨模块接线（登记待接线）。

### 7. ivy_send_to_gpu_leaf（0x1408c1580）——设置位 scale

- 证据（fresh）：`local_d4 = [device+0x1d] ? 0x3f59999a(0.85) : 0x3f800000(1.0)`，
  该 scale 乘到打包元素 `size`。
- 改：`GraphicsDevice` trait 增 `ivy_leaf_scale(&self)->f32`（默认 1.0，D 组 render
  后端读 device 字节）；`pack_leaf(leaf, walls, scale)` 将 `size` 缩放；
  `upload_ivy_leaves` 用 `device.ivy_leaf_scale()`。
- `seed`=frame、`arch_height`=所属墙 arch（shader 偏移 44，`base=arch_height-min(h,0)`）
  维持；`forward`=vector_2c、`branch_dir`=branch_direction 按 shader 偏移 16/32 定案。

### 8. ivy_send_to_gpu_segments（0x140a9b4e0）——映射按 shader 定案

- 核对：shader `instanced_ivy_branch.raster.hlsl` 读取偏移 0/12/16/28/32/44；
  `wall_normal`（偏移 32）经 `cross(dir, wall_normal)` 出侧向 → 必须是水平墙面法线，
  与 Rust `(-d.z/len,0,d.x/len)` 一致；`arch_height`（偏移 44）用于
  `base=arch_height-min(hstart,0)` → 所属墙 arch，与 Rust `wall_arch_height` 一致。
- 改：仅更新注释为"shader 定案"；SsboId 原键 0x2002d2… vs `SsboId::of(NAME)` 不命中
  维持登记（原键验证已确认）。

### 9. IvySpawnNoise::new —— 结构判等维持

- `insert_ivy_resources`（0x1409440c0）内联序列 seeded(45)+PerlinFractal+oct3+gain2.0
  +lac2.0+freq0.3+seed123 与 Rust `new()` 一致（判等维持）；`threshold` -0.02 为
  spawner 噪声门 `noise <= threshold → skip` 消费常量，本轮未单独 RNE 重验（登记）。

## 三、自检

- `cargo check -p country-core`（lib）：**通过**（仅 16 条既有 warning，ivy 侧仅
  `propose_new_segment` 的 `emit` 未用一条，属重建前既有）。
  验证时刻：重建全部落盘后首次 check 通过。
- `cargo test -p country-core --lib`：当前仓库因**他 agent 并发 WIP**（ui_clutter、
  decorator_storage、translate_input、snapshot 等非 ivy 模块的 cfg(test)/lib 错误）
  整体编译失败；ivy 文件 0 错误（grep `--> ` 无 `systems/ivy|resources/ivy` 命中，
  唯一提及为 `emit` warning）。重建刚完成时的 lib check 对 ivy 代码为全绿。
- ivy 侧既有测试（grower 相位循环/advance、leaf pack、segments pack、spawner
  vertical_u/inside/budget/approx_u）在新代码上类型正确（无错误报告）；新增
  `budget_advance`、`pack_leaf scale` 两断言。

## 四、诚实登记（仍待取证，未宣布完成）

1. grower 新段提案 lateral 符号：native 由 0x14094b6d9..b6ff 尖端-段比较选出，本实现
   以调用方 `side`（direction·normal dot）重建；`into_world_space` 的 side 实参
   （`call 0x141469000` 返回）未逐指令定案。
2. spawner 格内 `is_inside_bbx` 复核：原版 `from_rotation_arc`+utils `is_inside_bbx`，
   本实现用 XZ 轴对齐段 AABB 包含重建。
3. is_point_still_valid 地形样点栅格：0x140998dc0 内样点 AABB 以曲线采样重建
   （网格分辨率待取证）；脏墙输入的 Circle/Rectangle+FinalizedWallHoles 为跨模块接线。
4. spawner 预算边界 `acc=total-(n-1)` 与 `0x9c41/0x46/0x13` 常量按机器码定案；旧
   `total+0x13>=40000` 边界已改。
5. GPU `seed`=frame 仍为候选（leaf 闭包反编译待取证）；IvySpawnNoise::new 阈值 -0.02
   未单独 RNE 重验。

## 五、改动清单（均为库内 ivy 模块，未动他模块）

- `resources/ivy/wall_coord.rs`：into_world_space Y 修正。
- `systems/ivy/ivy_grower.rs`：get_growth_point_proposal 公式、墙有效 panic×2、
  删除 intersect_ivy_growth_w_wall_segment。
- `systems/ivy/ivy_spawner.rs`：整文件重写（7 参签名/门/预算/分派/两路径/共享提交）。
- `systems/ivy/ivy_pruner.rs`：is_point_still_valid 返墙+panic+样点带门、尾高度带门、
  整株 remove_entry。
- `systems/ivy/ivy_send_to_gpu_leaf.rs`：scale 位（pack_leaf scale、upload 取 device）。
- `render/ssbo.rs`：GraphicsDevice trait 增 `ivy_leaf_scale` 钩子（默认 1.0）。
- `systems/ivy/ivy_send_to_gpu_segments.rs`：注释定案（无逻辑改动）。
