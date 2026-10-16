# REVIEW — resources/ivy + systems/ivy（reviewer-eco，2026-10-15）

独立核对 restorer 现有实现（src 状态）是否真正实现原二进制业务逻辑。未采信作者解释，以
原反编译/汇编（collision-eco/decomp + RNE 直读）为准。restorer 尚未提交 RESTORE.md。

> **时序注记（2026-10-15 ~21:19）**：审查期间 restorer 正在并发编辑本模块——
> `ivy_grower.rs`（21:17）、`ivy_pruner.rs`（21:18）、`registration.rs`（21:18）、
> 新 `ivy_spawner.rs`（21:19）相继改动，`ivy.rs` 已新增 `pub mod ivy_spawner;`
> **但尚未新增 `pub mod ivy_grower;`**。本报告基于 21:19 的最新文件；下列 FIX/GAP
> 在 restorer 收尾后需复验。

## 编译状态（实测）

- `cargo check -p country-core --offline --locked`（lib）**通过**。
- `cargo check -p country-core --lib --tests`（测试目标）**编译失败**：
  - `E0432 unresolved import 'super::wall_segments_from_state'` @ `ivy_pruner.rs:10`、`ivy_spawner.rs:34`
  - `E0432 unresolved import 'super::ivy_grower'` @ `registration.rs:4`
  - 根因：`ivy_grower.rs` 仍未被 `ivy.rs` 声明为子模块（`wall_segments_from_state` 定义于
    ivy_grower、`ivy_grower` 本身均不可见）。lib 通过仅因这些引用在 cfg(test)/未接线侧。
  - 另有 interaction/state 组测试编译错误（input_record_replay/translate_input/session 等 30+ 条）
    不在本模块范围，但说明**全 crate 测试目标当前不可编译**，README "all-targets 编译" 不成立。

## 逐函数三栏对照

### resources/ivy/ivy_storage.rs（原 20 项）

| 函数 | 原行为（证据） | Rust 现状 | 判定 |
|---|---|---|---|
| Ivy::add_point 0x140869460 | push_back 40B 点 {id=next+1,reserved=0,...} | id=++next_point_id, push 36B 点 | **OK**（字段偏移一致；stride 40→36，见 §注1） |
| Ivy::last_point 0x1408694f0 | 末点（空 unwrap 失败） | back().expect panic | **OK** |
| Ivy::last_segment_point_ids 0x140869540 | 末两端点 id（len<2 panic） | 断言 n>=2 | **OK** |
| IvyStorage::spawn_ivy 0x1408696f0 | RNG 序=count→float；max=start+rng(end-start)；scalar=(end-start)*rng+start；root id 0；next_point_id=1；旧计数当键；assert start<=end | 一致 | **OK** |
| 12 inline（point_count/…/get/new/roots/iter…） | 内联查询/清理 | 齐 | **OK** |

### resources/ivy/ivy_leaf_storage.rs（原 7 项）

| 函数 | 原行为 | Rust 现状 | 判定 |
|---|---|---|---|
| add 0x1408fe280 | push 104B IvyLeaf（字段序/偏移已比对） | 字段偏移一致 | **OK**（stride 104→100，见 §注1） |
| delete_affected/all | 按 ivy 过滤/清空 | 齐 | **OK** |

### resources/ivy/ivy_direction_proposer.rs（原 3 项）

| 函数 | 原行为 | Rust 现状 | 判定 |
|---|---|---|---|
| get_direction 0x141234590 | pos/2.8；height=clamp(3y,1,8)；y=bias*0.6+(1-bias)*0.35+ny；perchance seed=2s+1（新(seed)→状态[2s+1,0]，与 insert "0→[1,0]、10→[21,0]" 一致）；rot 采样 -24/7/5.5±u*100；blend=(2n+1)/2 clamp；角=(2b-1)*80°（sin/cos 吃半角=±40°，旋转=±80°） | 全部一致 | **OK** |
| IvySpawnNoise::new | insert 构造 seeded45/oct3/gain2.0/freq0.3/seed123 | 一致 | **OK** |

### resources/ivy/wall_coord.rs（原 3 项）

| 函数 | 原行为 | Rust 现状 | 判定 |
|---|---|---|---|
| into_world_space 0x140b199e0 | 曲线长<u、len==0、u/l∉[0,1]、NaN→panic；采样曲线点；法向偏移 ±0.63*0.5；**out.y=v±offset_z、out.z=curve_z±offset_z**（RNE 直读 0x140b19acd/0x140b19b72..ba7 定案：`unpcklps xmm3,xmm8` 把 v 并入第 2 lane，法向第二分量偏移同加 Y 与 Z） | x/z 偏移一致；**y 仅=v，未加 offset_z** | **FIX-1** |
| from_wall_u | v=0 | 一致 | **OK** |
| max_ivy_grow_height | grower 引用 | 未消费（grower 用本地常量 0.5，待取证） | **注记** |

### systems/ivy/ivy.rs（closest_segment_index 0x14094c800）

| 函数 | 原行为 | Rust 现状 | 判定 |
|---|---|---|---|
| closest_segment_index | 56B 段；过滤1=两端点均>max 才跳过（RNE 直读 `ucomiss h0,max; jbe 处理; ucomiss h1,max; ja 跳过` 定案）；过滤2=插值高>max 跳过；t=clamp(dot/len²,0,1)；距离=点到直线垂直距；初值 100；命中 t∉[0,1] panic；extra=lerp | 全部一致 | **OK**；纠错 ALGORITHMS.md §4.2（过滤 1 方向写反） |

### systems/ivy/ivy_grower.rs（原 11 项）

| 函数 | 原行为 | Rust 现状 | 判定 |
|---|---|---|---|
| ivy_grower 0x14094a910 | **10 相跳表状态机**（0x142ab3be0，world+0x118 索引；守卫 bit0@+0x48、计时+0x11c；主闭包 last_point→get_direction→closest_segment_index→墙跳→add_point） | 现实现为候选链近似：每株 pending 存在则 advance_growth（已验证路径），否则 nearest_wall_normal+check_for_wall_jump+get_growth_point_proposal 推进。**10 相状态机/守卫位/计时槽未重构** | **PARTIAL（主状态机 GAP）** |
| advance_growth | 完成路径：progress+=delta/0.075、clamp、插值尾点、0.99 清 pending+last_segment_point_ids+发事件 | 与 0x14094b1bb..b267 逐条一致 | **OK** |
| get_growth_point_proposal | 内联提议 | 候选链近似（step=SEGMENT_DURATION=0.075 世界单位 + 法向 0.63*0.5），原接受门槛未重构 | **PARTIAL** |
| check_for_wall_jump | 内联墙跳（wall spatial hash） | 全墙 closest_segment_index 近似 | **PARTIAL** |
| get_ivy_x_direction_relative_to_wall_direction | 内联方向 | =wall_direction 转 90° | **PARTIAL 待取证** |
| closest_segment_index 0x14094c800 | O（见上） | 已实现于 ivy.rs | **OK** |
| get_wall_double_sided_normal 0x14094caa0 | 以 u 参采样曲线（FUN_141467ce0）、点-插值点定侧翻转 | 改从 point.x/curve.length 反推 u、点-段首定侧 | **PARTIAL（近似）** |
| intersect_ivy_growth_w_wall_segment 0x14094cc60 | 生长线穿墙判定（引擎 ray 辅助） | 实现合理，未逐条核对 | **PARTIAL 待取证** |
| wall_segments_from_state/nearest_wall_normal | （自造候选链 helper，非原 inventory 函数） | 供候选链/新 spawner/pruner 复用 | **注记（不算原业务）** |

### systems/ivy/ivy_spawner.rs（原 5 项，21:19 新增）

| 函数 | 原行为 | Rust 现状 | 判定 |
|---|---|---|---|
| ivy_spawner 0x14094cf90 | settings +0x1c 门；`39999 < total+0x12` → return（≈40000 上限）；+0x120 累加器选 pass（<3 随机根/≥3 光标根）；随机根=地形栅格格采样；光标根=param_4 CursorTerrainRaycast | 门控（ivy_leaves/count 40000/预算翻转）已实现；**随机根=uniform 世界采样近似（非地形栅格）**；+0x120 语义建模为预算计数（待取证） | **PARTIAL** |
| try_random_ivy_root/try_ivy_root_next_to_cursor/is_point_inside_wall_segment | 内联 | 候选链近似（nearest_wall_normal + 0.63*0.5 半宽） | **PARTIAL** |
| spawn 提交参数 | 原 range 待取证 | spawn_ivy(pos,dir,1.0..2.0,4..16,0) | **待取证** |

### systems/ivy/ivy_pruner.rs（原 4 项，21:18 增补）

| 函数 | 原行为 | Rust 现状 | 判定 |
|---|---|---|---|
| ivy_pruner 0x140997410 | 墙空间哈希校验（"Spatial Hash returned a wall id " 0x142abb028）+逐点 is_point_still_valid；脏墙 deque 消费 | 现实现=OnWallChanged 事件 + 全墙 nearest-wall 重验；**空间哈希/地形格采样近似** | **PARTIAL** |
| is_point_still_valid 0x140998dc0 | 地形格→closest_segment_index(max=0.5=0x3f000000)→段附加样点复检 | 直接全墙 closest_segment_index(0.5)，地形格/附加样点未还原 | **PARTIAL** |
| ivy_pruner_cmd 0x140999330 | RemoveIvyCmd 40B 消费→盒内删尾+删叶 | apply_remove_ivy_cmd 偏移/语义一致 | **OK** |
| remove_ivy_at_location 0x140999860 | 事件构造 | 一致 | **OK** |

### systems/ivy/ivy_leaf_animation.rs / ivy_branch_thickness_animation.rs / ivy_leaf_spawner.rs

| 函数 | 原行为 | Rust 现状 | 判定 |
|---|---|---|---|
| ivy_leaf_animation 0x140b0e380 | growth<1→delay<=0: growth+=2*delta clamp1+发事件，否则 delay-=delta；branch_thickness=(1-avg)*0.5+avg；缺 ivy/端点→**日志+继续**（frame 门控，非 panic） | 全部一致；log::warn+continue | **OK** |
| ivy_branch_thickness_animation 0x140995a80 | atten=expf(count/-10)；taper=1-i/(count-1)；thk=taper*(1-atten)+(1-taper)*0 | 一致 | **OK** |
| ivy_leaf_spawner 0x140996080 | 容量 10000；selection≥0.4；noise 2a+(89,-3456,-34)>-0.05；normal/disc*0.15/pos/vector_58/vector_2c/size=(1-mix)*0.8+mix*1.2/delay=u*0.3；**缺 ivy→日志、缺点→panic**（0x142abae30/48） | 全部一致（warn+return / expect） | **OK** |

### systems/ivy/registration.rs + 原 systems/ivy/mod.rs（原 3 项）

| 函数 | 原行为 | Rust 现状 | 判定 |
|---|---|---|---|
| add_ivy_systems 0x140944070 | grower→pruner→leaf_spawner→send_to_gpu 调度序 | add_ivy_systems 现含 grower/spawner/pruner/pruner_cmd/leaf_spawner/两动画（21:18 增补）；**仍缺 GPU 上传（ivy_send_to_gpu_leaf/segments）** | **PARTIAL** |
| insert_ivy_resources 0x1409440c0 | 四资源入 App | register_ivy_storage 齐 | **OK** |
| add_ivy_events 0x140944280 | 事件注册 | 齐 | **OK** |

## FIX / GAP 清单

- **FIX-1（wall_coord.rs::into_world_space）**：原生 `out.y = v ± offset_z`（法向第二分量偏移同时
  加到 Y 与 Z），现实现 `out.y = v`（`self.v - 0.0` / `0.0 + self.v` 丢掉 ± offset_z）。
  修法：`Vec3::new(x ± offset_x, self.v ± offset_z, z ± offset_z)`（符号同侧）。
  证据：RNE 直读 0x140b199e0（prologue `mov r14,rdx` 证 r14=WallCoord）、0x140b19acd
  （`movss xmm8,[r14+4]`=v）、0x140b19b72（`unpcklps xmm3,xmm8` 并入 lane1）、0x140b19b7a..ba7。
  注：该函数当前无调用者（grower 改为自行计算偏移），修复优先级可后置，但函数本体须对齐原生。
- **GAP-1（编译断点）**：`ivy.rs` 缺 `pub mod ivy_grower;` → 测试目标 E0432 ×3
  （registration.rs:4 / ivy_pruner.rs:10 / ivy_spawner.rs:34）。restorer 收尾时须补声明。
- **GAP-2（主状态机）**：grower 10 相跳表状态机（0x142ab3be0、world+0x118、守卫+0x48、计时
  +0x11c）仍未重构；现主闭包为候选链近似。按 DISPATCH"11 grower 逐函数闭合"，grower 主体、
  墙空间哈希遍历、接受高度/噪声阈值均未闭合（作者已自注"candidate chain / 待取证"）。
- **GAP-3**：ivy_spawner 随机根地形栅格采样、+0x120 预算语义、spawn range 原值待取证；ivy_pruner
  空间哈希/地形格、is_point_still_valid 附加样点待取证。
- **注 1（stride）**：IvyPoint 原生 40B / Rust 36B、IvyLeaf 原生 104B / Rust 100B——字段偏移一致，
  仅尾部 4B 内嵌。GPU 上传消费前须补 padding。
- **注 2（NaN）**：closest_segment_index 的 sse_max/sse_min 对 NaN 返回第二操作数，与原生条件分支
  （NaN→0）在病态输入分歧；正常路径无影响。
- **注 3（日志 vs panic）**：ivy_leaf_spawner/ivy_leaf_animation 缺 ivy 原生为**日志**（非章节所写
  panic）；缺点才 panic。现实现 warn/expect 正确。

## 结论

- **OK**：storage（20+7）、direction_proposer、closest_segment_index、leaf_animation、
  branch_thickness、leaf_spawner、pruner_cmd/remove_ivy_at_location——与机器码一致。
- **PARTIAL**：grower（主状态机缺）、spawner、pruner（wall-change 校验）——已有可调用逻辑，
  但主状态机/空间哈希/地形格/阈值未闭合，作者自注"候选链/待取证"。
- **DISPATCH 验收"20+7 存储 + 11 grower + 动画/剪枝逐函数闭合"未达成**；且测试目标当前不可编译。
- 状态建议：**fix_pending**（restorer 收尾：补 `pub mod ivy_grower;`、闭合 grower 主状态机与
  spawner/pruner 的待取证项、按 FIX-1 修 wall_coord，再交复验）。
