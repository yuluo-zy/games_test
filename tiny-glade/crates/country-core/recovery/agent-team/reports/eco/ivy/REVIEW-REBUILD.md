# REVIEW-REBUILD — ivy 真实逻辑重建复验（reviewer-eco，2026-10-17）

复核对象：rebuild-ivy 重建（REBUILD.md，8 业务函数 + IvySpawnNoise）。方法：对 fresh 反编译/
RNE 机器码逐函数判等。实测：cargo check 过（ivy 文件 0 错误）；全 lib 测试目标因他 agent 并发
WIP（ui_clutter/decorator_storage/translate_input/snapshot）整体编译失败，与 ivy 无关。

## 关键纠偏：我首轮 FIX-1 是误读 — REBUILD 正确

**REBUILD 结论（out.y = v 无 offset）经本机 RNE 复析确认，我首轮"FIX-1: out.y=v±offset_z"
是误读，此处正式撤回。**

复析 0x140b19b40..ba7：
- `func_0x140c9cdc0`（x0y 提升）把法向 {nx,nz} 写成 Vec3 {nx,0,nz}@[rsp+0x50]；
- `movsd xmm1,[rsp+0x50]` = **{nx, 0}**（前 8B，y 槽恒 0），`movss xmm0,[rsp+0x58]` = nz 标量；
- 两乘 0.63·0.5 后：xmm1={nx·0.315, **0**}，xmm0=nz·0.315；
- `unpcklps xmm3,xmm8`（xmm8=WallCoord.v）→ xmm3={curve_x, v, 0, 0}；
- side 加减：xmm3 ± xmm1 → {curve_x±nx·0.315, **v∓0**}=out.x/out.y；xmm2(curve_z)±xmm0 → out.z。
**out.y = v（offset 向量 y 槽恒 0），仅 x/z 带 ± offset**——首轮我把 xmm1 的 lane1 误读为
offset_z 导致 FIX-1 错误；现 wall_coord.rs 已改回 `Vec3::new(x±offset_x, self.v, z±offset_z)` ✓。

## 逐函数判等

| 函数 | REBUILD 声明 | 复验 | 判定 |
|---|---|---|---|
| into_world_space | out.y=v 无 offset，仅 x/z ±0.315 | 本机 RNE 复析确认（上） | **判等（纠我 FIX-1）** |
| ivy_grower | 去 u_next；WallCoord.u=wall_height·u±dir.x·0.21、v=tail.y+dir.y·0.21（0x14094b5a5/b65c/b661）；墙无效三处 panic（0x14094c1e3/200/21d） | `get_growth_point_proposal` 现用 `WallCoord{u:wall_growth_height(wall)*u+lateral, v}.into_world_space`，lateral=±dir.x·0.21（DIRECTION_SCALE=0.21）；墙有效改 panic | **判等** |
| intersect_ivy_growth_w_wall_segment | 死代码删除（原版算法完全不同且两域无调用点） | src 全树无引用（ivy_grower.rs/ivy.rs grep 空） | **判等（删除正确）** |
| ivy_spawner | 7 参签名（0x140944070）；首门 GladeSettings+0x1c=ivy_leaves 非 AppMode；预算 acc=total-(n-1)，acc+0x12>39999；分派 digital[36].disc<3；垂直除数=墙高；侧向 ±0.34（A=is_on_right_side / B=subtype>1 get_bool）；facing-dot 双条件；is_inside_bbx；Res<WallSpatialHash> | 签名 7 参、GladeSettings 门（删 AppMode 误门）、BUDGET_GATE=39999、IVY_SPAWN_ACTION_SLOT=36、vertical_u(/wall_height)、curve_is_on_right_side、get_bool 仅 subtype>1 | **判等** |
| is_point_still_valid | 返 Option<WallId>；墙无效 panic；插值 u (-0.01/h, 1+0.01/h)；样点 ±0.1 AABB 带门 | 返 Option<WallId>、wall_arch_height 带门、样点 AABB（网格分辨率待取证） | **判等（网格待取证）** |
| ivy_pruner | tail 有效后高度带门 arch+(flag?+0.2:subtype<2?(-0.79-0.1):-0.46)；空株 remove_entry | 高度带门 + remove_entry 整株删除 | **判等** |
| ivy_send_to_gpu_leaf | [device+0x1d] 设置位 scale 1.0/0.85 → pack size；GraphicsDevice trait 增 ivy_leaf_scale 钩子 | pack_leaf(leaf,walls,scale)、device.ivy_leaf_scale()、0.85/1.0 | **判等** |
| ivy_send_to_gpu_segments | 字段映射按 shader 定案（wall_normal 偏移 32 / arch_height 44）；注释更新 | 注释定案，无逻辑改动 | **判等（注释级）** |
| IvySpawnNoise::new | 结构判等维持（seeded45+PerlinFractal+oct3+gain2.0+lac2.0+freq0.3+seed123）；阈值 -0.02 待 RNE 重验 | 与 insert_ivy_resources 构造一致 | **判等（阈值登记）** |

## 宏生成胶水（不重建，确认）

add_ivy_systems / insert_ivy_resources / add_ivy_events / remove_ivy_at_location =
Bevy 注册框架/薄事件生产者，按 AGENTS.md 计胶水不重建——**确认分类正确** ✓。

## 诚实登记（REBUILD §四，未宣布完成）

lateral 符号（调用方 side 重建，原生尖端-段比较未逐指令定案）、is_inside_bbx XZ 轴对齐近似、
样点栅格分辨率、spawner 预算 0x9c41/0x46/0x13 常量、GPU seed=frame 候选、阈值 -0.02 未单独
重验——均诚实。

## 结论与 DISPATCH 建议

- **ivy 重建项 = 闭合**：8 业务函数 + IvySpawnNoise 与 fresh 反编译/RNE 判等；into_world_space
  复析确认 REBUILD 正确、**我首轮 FIX-1 撤回**。
- 残留分歧/待取证（不阻塞主体，库内可闭合）：grower lateral 符号逐指令定案、spawner is_inside_bbx
  与预算常量复核、样点栅格分辨率、GPU seed、阈值 -0.02。
- 建议：REBUILD=review_passed（限定）；补 is_point_still_valid 样点栅格分辨率 + spawner
  is_inside_bbx 逐指令后 ivy 库内主算法闭合。
