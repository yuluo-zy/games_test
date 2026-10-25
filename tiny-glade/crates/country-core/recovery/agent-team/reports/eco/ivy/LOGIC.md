# ivy 模块逐函数逻辑判等报告（logic-verifier-ivy，2026-10-17）

> 按 2026-10-16-logic-verify-protocol.md：全部业务函数以 ghidra fresh 反编译
> （`--worker shared`）+ RNE 机器码交叉核对为准，不采信 RESTORE/作者描述。
> verdict：`判等` / `未实现-需重做`(附分歧点) / `胶水-非业务`。

## 一、resources/ivy（权威存储）

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x140869460 | Ivy::add_point | 判等 | —（next_point_id 先增后用、id=新值、push_back、thickness=0，逐写一致） |
| 0x1408694f0 | Ivy::last_point | 判等 | —（len!=0 返 back，否则 unwrap_failed） |
| 0x140869540 | Ivy::last_segment_point_ids | 判等 | —（断言 n>=2，返 (points[n-2].id, points[n-1].id)） |
| 0x1408696f0 | IvyStorage::spawn_ivy | 判等 | —（root id=0/thickness=0、count 范围断言、RNG 序 count→float、insert、next_ivy_id+1） |
| 0x1408fe280 | IvyLeafStorage::add | 判等 | —（104B 元素 15 字段逐写一致，含 branch_thickness/growth 双 0） |
| 0x1412343b0 | IvyDirectionProposer::default | 判等 | —（RNE 逐参：base seeded(45)/oct2/gain2.0/lac2.0/freq1.0/seed13；rotation seeded(45)/oct3/gain1.0/lac1.3/freq2.0/seed103） |
| 0x141234590 | IvyDirectionProposer::get_direction | 判等 | —（RNE 全链：/2.8、-23/-40/-32.5、clamp(3y,1,8)*nx、12.03/67/785、-12.03/-0.82/-785、0.6/0.35、normalize、seed u128、3×uniform*100、-24/7/5.5、blend、80°、deg2rad*0.5、sinf/cosf 旋转+normalize） |
| 0x140b199e0 | WallCoord::into_world_space | **未实现-需重做** | **Y 分量错**：RNE 0x140b19b72..ba7 机器码 out.y = v（offset 向量的 y 槽恒 0，因 x0y 提升 (normal.x,0,normal.z)），仅 out.z = interp.z ± offset_z；Rust 实现为 `self.v ± offset_z`（RESTORE FIX-1 误读，将 offset_z 错加到 Y）。X/Z 与断言路径一致。 |
| —（内联） | IvySpawnNoise::new | 判等(结构) | insert_ivy_resources 0x1409440c0 显示 seeded+oct+gain+seed 序列一致；seed=123/freq=0.3/PerlinFractal 本轮未单独 RNE 重验（消费方 spawner 已判未实现） |

## 二、systems/ivy（消费者）

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x14094a910 | ivy_grower（10 相分派） | **未实现-需重做** | 分派/守卫翻转(0x14094c09f: flag_48^=1→守卫体)/相1/3/4 按位/相2 计时门/相5-9 panic 尾=判等；**守卫体新段提案分歧**：①墙有效(段<2 或 高<=0)原版 **panic**(0x14094c1e3) 而 Rust return；②Rust `u_next = u + SEGMENT_DURATION/length` 原版无此推进（0x14094b594 直接用 u）；③Rust `v = wall_height*u_next + dir.y*0.21` 原版 `out.x=wall_height*u+lateral`、`out.y=tail.y+dir.y*0.21`；④into_world_space Y 错(见上)；⑤方向 SIMD 精确式 0x14094ba5d..bd34 部分重构。守卫体内 0x14094b1bb 推进/0.99 完成事件/0.001 冷却/flag_8c 消费/洞门 逐条判等 |
| 0x14094c800 | closest_segment_index | 判等 | —（双端>max 才跳、插值高过滤、t=clamp(dot/len²)、垂距+|t-t|*len*100、初值 100、t 越界 Err 仅命中判定处等价） |
| 0x14094caa0 | get_wall_double_sided_normal | 判等 | —（段垂线+sign 判侧：`recip*dy*(px-ix)<=recip*dx*(pz-iy)` ≡ normal·(p-a)>=0；边界 panic-vs-min-clamp 实际不可达） |
| 0x14094cc60 | intersect_ivy_growth_w_wall_segment | **未实现-需重做** | **算法完全不同**：原版是 `intersection_aabb_segment2` + 投影 + 偏移盒(0.707/0.63*0.5)取近点 t；Rust 是 `perp_dot` 射线-线段求交。且 Rust 中该函数为死代码（无调用点） |
| 0x14094cf90 | ivy_spawner（路径 A/B） | **未实现-需重做** | ①**门来源错**：RNE 0x14094cffb..d00a 首门 = `[GladeSettings+0x1c]==1`（第7参=settings，由 add_ivy_systems 签名定案），Rust 用 `AppMode.native_tag()!=1`；Rust 另有 `actions.digital[36].pressed()` 附加门（原版无）；②预算公式：原版逐 ivy 累加(值-1)再 `>39999→return`，Rust `total_point_count()+0x13>=40000` 边界不一致；③路径A 侧向：原版 `is_on_right_side` 选 ±0.68*0.5=±0.34，Rust 恒 +0.34；④路径B 侧向：原版仅 subtype>1 才 `get_bool`，且 ±0.68*0.5=±0.34，Rust 无条件 get_bool 且幅度 0.68；⑤root 距离：原版 `facing_dot>=0 && dist²<thr` 双条件，Rust 只查 dist²；⑥attempts：原版共享 `2+usize_less_than`，Rust 路径A 固定 3；⑦垂直 u：原版 `(rand*1.5-0.75)/[wall-0x270]+clamp(u)`，Rust 路径A `/1.0+interpolated`、路径B `/curve.length+u_on_curve`；⑧原版另做格内 `is_inside_bbx` 复核，Rust 无；⑨参数缺 Res<WallSpatialHash>（原版 7 参含之，路径A 走空间哈希格） |
| 0x140997410 | ivy_pruner | **未实现-需重做** | ①**尾部高度带门缺失**：原版 is_point_still_valid 通过后还要 `tail.y >= arch + (flag?+0.2 : subtype<2?(-0.79-0.1):-0.46)`（读 wall+0x284/+0x296/+600），Rust 仅 is_point_still_valid；②**整株删除缺失**：原版根环结束后对空 ivy `remove_entry` 从 map 删除（0x140998711..1312），Rust 文档称"无整株删除"且不删；③is_point_still_valid 原版返回 **wall 指针**（供带门），Rust 建模 bool；④脏墙输入：原版 Circle2d/Rectangle2d::contains_point + FinalizedWallHoles 表，Rust 用 EventReader<OnWallChanged> 近似 |
| 0x140998dc0 | is_point_still_valid | **未实现-需重做** | ①**样点 AABB 复核缺失**：原版 band 通过后查 wall 附加样点（±0.1 AABB 逐条 contains，含地形栅格 0x140998dc0 内），命中才有效；Rust band 后直接 return true；②墙有效(段<2/高<=0)原版 **panic**，Rust return false；③返回值原版=owning wall 指针，Rust bool |
| 0x140999330 | ivy_pruner_cmd | 判等 | —（box 严格 `<` 三分量、首个命中点、suffix 弹出至 first_hit、growth_progress/pending 清零、delete_affected_leaves(point_a/b != id)） |
| 0x140999860 | remove_ivy_at_location | 胶水-非业务 | 薄事件生产者；payload 40B（source_0/8、center、dimensions{width,height,width}）+event id 逐写一致 |
| 0x140b0e380 | ivy_leaf_animation | 判等 | —（growth<1→delay<=0 时 growth+=2delta clamp1.0 写事件/否则 delay-=delta；按 id 找端点；branch_thickness=(1-avg)*0.5+avg；缺端点 log） |
| 0x140996080 | ivy_leaf_spawner | 判等 | —（容量 10000、选中 `uniform>=0.4`、缺 ivy 继续、点查找、噪声 `>-0.05`、normal=avg(a.dir,b.dir) normalize、disc、3×normal 抖动 clamp[0,1]、3×normal(angle/down/size)、angle=clamp(n/1.5)*80°*rad*0.5、sinf/cosf 旋转、y-=clamp(|down|/1.5)、normalize、delay uniform、add()。注：噪声样点(89,-3456,-34)/disc 0.15/size 0.8-1.2/delay 0.3 为内联常量，与 crate 机器码测试一致但本轮未从 fresh 反编译逐一重导） |
| 0x140995a80 | ivy_branch_thickness_animation | 判等 | —（expf(len/-10)、thickness=taper*(1-att) ；len=1 时 0/0=NaN 语义一致） |
| 0x1408c1580 | ivy_send_to_gpu_leaf | **未实现-需重做** | ①原版读 `[GraphicsDevice+0x1d]` 设置位 → 元素 scale 1.0/0.85，Rust 无此 scale；②原版 IvyLeafSsbo 打包闭包直接映射字段，`seed`/`arch_height` 来源（frame?/wall arch?）仍待取证；③SsboId 由原键 0x9e3cfa…/0x1d6de5…，Rust `SsboId::of(NAME)` 不命中（已知 gap）；④原版二次 IvyFlowerSsbo 通道 `Take(0x3e8)`，Rust 无花集合 |
| 0x140a9b4e0 | ivy_send_to_gpu_segments | **未实现-需重做** | ①`wall_normal`（切垂线）与 `arch_height` 字段映射待取证；②SsboId 原键 0x2002d2… 与 `SsboId::of(NAME)` 不命中；③Rust upload 为库内镜像，原版为 render 域 render-loop 内函数 |
| 0x140944070/0c0/280 | add_ivy_systems / insert_ivy_resources / add_ivy_events | 胶水-非业务 | Bevy 注册框架；注：insert_ivy_resources 原版仅插 IvyStorage/IvySpawnNoise/IvyLeafStorage/IvyLeafRng 4 资源，Rust register_ivy_storage 另 init IvyDirectionProposer/PrevWallHoles/MainActionSet/3 事件（注册域偏差，非业务） |

## 三、统计与严重分歧模式

- 判等 13（含 IvySpawnNoise::new 结构判等、leaf_spawner 常量注记）；胶水-非业务 4；**未实现-需重做 9**。
- 严重分歧模式（贯穿 grower/spawner/pruner）：
  1. **wall 有效门 panic-vs-return**：原版 `[wall-0x290]<2 || [wall-0x270]<=0` 一律 **panic**（0x14094c1e3 / 0x140998dc0），Rust 一律 return 跳过 —— 三处同错。
  2. **垂直/侧向公式与常量错位**：spawner 垂直 u 除数=wall 高（非 1.0/curve_length）；路径 B 侧向幅度=0.34（非 0.68）、条件=get_bool 仅 subtype>1；路径 A 侧向=is_on_right_side（非固定 +0.34）。
  3. **is_point_still_valid 返回值类型错**：原版返 owning wall 指针并附带样点 AABB 复核与尾部带门，Rust 降为 bool 且缺该二门。
  4. **grower 新段提案公式**：u 无推进、v=tail.y+dir.y*0.21、into_world_space Y 无 offset —— 与原版不符。
  5. **门来源错位**：spawner 首门=GladeSettings+0x1c（原 7 参签名定案），Rust 误作 AppMode，并凭空加了 ActionSet pressed 门。

## 四、范围注记

- startup/startup_ivy、systems/audio/ivy_audio、render_loop/draw_shaders/instanced_ivy 不在本 verifier 范围（非 resources/systems-ivy 清单）。
- 未实现函数均未改 src；本审计未使用 --write worker（只读 shared + RNE radare2 交叉）。
