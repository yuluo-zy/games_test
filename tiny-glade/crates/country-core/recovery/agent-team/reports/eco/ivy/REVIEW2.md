# REVIEW2 — resources/ivy + systems/ivy 二轮复核（reviewer-eco，2026-10-15）

复核对象：restorer 二轮交付（RESTORE.md，2026-10-15）。逐项核对 restorer 声称的新增/修正
是否属实、是否引入新问题。证据：RNE 直读（wall_coord/spawner 常量/sheep 字段名）、re-analyst
O 章节（ivy_spawner.rs.md）、decomp（0x140944070 add_ivy_systems 0x16=22、0x1413ff520、
0x1416f9eb0、0x1405c9c50、0x140080440）、实测 `cargo test --lib`（1584 过 / 3 败，3 败均在他组
geometry/input/terrain，与 eco 无关；ivy/garden/sheep 测试全过）。

## 逐项复核

### 1. FIX-1（wall_coord.rs::into_world_space）— ✅ 已修复

现实现 `Vec3::new(x ± offset_x, self.v ± offset_z, z ± offset_z)`，注释引 RNE 0x140b19acd/b72..ba7。
对照 RNE 直读（prologue `mov r14,rdx`、`movss xmm8,[r14+4]`=v、`unpcklps xmm3,xmm8` 并入 lane1、
0x140b19b7a..ba7 侧分支）：**out.y = v ± offset_z 成立**，与首轮 RNE 定案一致。OK。

### 2. ivy_spawner.rs（按 re-analyst O 全解重写）— 门/常量大部属实，路径 B 有缺口

| 项 | re-analyst O（章节 ivy_spawner.rs.md） | 现实现 | 判定 |
|---|---|---|---|
| 模式门 | `*(byte)(*AppMode+0x1c)==0x01`（Editing/Wall tag1） | `mode.native_tag()!=1 return` | **OK** |
| 数量门 | `total_ivy_count > 39999+0x12(=40017)` 且为**活株 Query 计数** | `next_id() >= 40000`（单调计数代理） | **PARTIAL**（活株 vs 单调计数、40000 vs 40017 有差，待取证） |
| 分派 | `uVar21<3`→路径 A 光标；`>=3`→路径 B 随机 | `next_id()<3`→光标，否则随机 | **OK**（方向与 O 一致；首轮版本 <3 随机是反的，已纠正；`next_id` 代 +0x120 计数待取证） |
| ±0.34 偏移 | 0.68×0.5（0x142ab31e0/0x142ab3200） | ROOT_OFFSET=0.34 | **OK** |
| 距根校验 | 路径 A 用 0.25（0x142925950）、**路径 B 用 2.56**（0x142ab3214，距 1.6） | 路径 A 用 0.25；**路径 B 无距校验** | **GAP**（路径 B 缺 2.56 校验） |
| 垂直钳 | `(rand×1.5−0.75)/墙高+段高` 钳 [0.000928482,1.0] | `(rand*1.5−0.75)/1.0+段高` 钳 [0.000928482,1.0] | **PARTIAL**（`/1.0` 硬编码墙高=1.0，原生除墙高——待取证） |
| 段高来源 | 命中段插值 `fVar30=t*p1+(1-t)*p0`（t=命中投影） | `lerp(extra_a,extra_b,0.5)`（**t 硬编码 0.5 且用 extra 字段**） | **PARTIAL**（应用 `hit.interpolated`=lerp(extra,t_hit)，现用中点近似） |
| 尝试次数 | 3（0x142ab3218） | SPAWN_ATTEMPTS=3 | **OK** |
| 随机根地形格 | 世界→格（32/65、偏移 32.0）+ 格内墙段 | 全墙 closest_segment_index 复检近似 | **PARTIAL**（作者自注待取证） |
| 路径 B 侧向 | re-analyst：逐墙随机 u→func_0x140c9cdc0 采样根位（该几何 helper 内部如何定侧未译） | **`uniform<0.5` 掷硬币选 ± 侧**（自造） | **PARTIAL/待取证**（掷硬币是猜的模型，非 decomp 直接证实；应译 func_0x140c9cdc0 的定侧后再对齐） |
| spawn 提交 | 原 range 待取证 | spawn_ivy(pos,dir,1.0..2.0,4..16,0) | **待取证** |

### 3. ivy_grower.rs 新增三函数 + pub mod ivy_grower

- **`pub mod ivy_grower;` 已声明**（ivy.rs:18）——首轮 E0432×3（registration/pruner/spawner 引用）
  已消解；grower 自入库并接入 add_ivy_systems。OK。
- **advance_growth 改闭包回调**：`emit: &mut impl FnMut`；进度/delta/0.075、clamp[0,1]、插值尾点、
  0.99 清 pending+last_segment_point_ids+emit 与首轮机器码核对**语义无变**。OK。
- **get_growth_point_proposal / check_for_wall_jump / get_ivy_x_direction_relative_to_wall_direction**：
  候选链近似（step=0.075 世界单位 + 法向 0.63*0.5；全墙最近墙跳；方向=法向转 90°），
  均**非原 10 相状态机/空间哈希遍历**，作者自注。PARTIAL。
- **10 相跳表状态机（0x142ab3be0 / world+0x118 / 守卫+0x48 / 计时+0x11c）仍未重构**——GAP 维持。

### 4. IvyPoint +32 标量对账（flags vs thickness）— 无冲突

- re-analyst：`u32 flags@+0x20=0`（spawn/add 初置 0）。
- branch_thickness_animation（首轮机器码核对）：写 `float thickness` 到 +0x20。
- 结论：+0x20 是同一 4B 槽，初值 0 又被动画写 float；`thickness:f32` 命名符合动画消费者，
  `flags` 命名来自初值。当前无冲突；**若 send_to_gpu_segments 把该位打包成 flag 需再对账**（待接线时验）。

### 5. registration 与原 22 系统对账 — PARTIAL

- decomp 0x140944070：`FUN_141664d80(auStack_88, 0x16)`=**22 系统元组**。
- 现 `add_ivy_systems` 只挂 7（grower/spawner/pruner/pruner_cmd/leaf_spawner/branch_thickness/
  leaf_animation）。**缺**：ivy_send_to_gpu_leaf(4)、ivy_send_to_gpu_segments(4)、
  systems/audio/ivy_audio.rs::ivy_audio_init_system(1)、其余 ~10 系统（疑逐变体/调试）。
- `insert_ivy_resources`/`add_ivy_events` 命名与资源/事件注册已补。OK。

### 6. GPU 发送链（send_to_gpu_leaf/segments）— **库内缺失，非库外接线**

- `ivy_send_to_gpu_leaf.rs`、`ivy_send_to_gpu_segments.rs` 是**原 country-core 源码文件**
  （inventory 各 4 业务函数），当前 src **未建**。
- 判定：读取 IvyStorage/IvyLeafStorage、组装 GPU SSBO 行的系统体是**库内业务**；仅实际提交
  （GraphicsDevice/RenderGraph）依赖 render 域。restorer 登记"待接线"低估了它——这是**库内缺失
  GAP**，不是纯库外接线。eco 批"生产/消费链闭合"的**消费端（GPU）未闭合**。

### 7. 待取证项诚实标注 — ✅

RESTORE.md §四 与代码注释诚实列出 grower 10 相状态机、空间哈希格采样、max_ivy_grow_height
数值来源、send_to_gpu 待接线——未冒充完成。OK。

## 结论与 DISPATCH 建议

- **ivy = fix_pending**：
  - FIX-1 ✅；spawner 门/常量大部对 O；grower 接线/编译恢复 ✅。
  - 仍缺：grower 10 相状态机（GAP）；send_to_gpu 2 文件 8 函数（库内缺失 GAP）；
    spawner 路径 B 距校验 2.56 缺、侧向掷硬币自造、`/1.0` 墙高、段高用中点近似、范围待取证
    （PARTIAL）；registration 22/7（PARTIAL）。
  - 修法建议：①补 `try_random_ivy_root` 的 2.56 距校验与按墙几何定侧（删自造掷硬币）；
    ②vertical_height 改 `(rand*1.5−0.75)/wall_height+hit.interpolated`（取命中 t 的插值）；
    ③按 O 补 send_to_gpu_leaf/segments 系统体；④补 registration 至 22 系统（含 ivy_audio）。
- **garden = review_passed（限定）**：radius 公式与 build 见 garden REVIEW2 注。
- **sheep = review_passed（限定）**：serde wire 全验证，见 sheep REVIEW2 注。
