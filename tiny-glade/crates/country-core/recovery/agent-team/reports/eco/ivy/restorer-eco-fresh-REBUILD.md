# ivy 9 函数 fresh 重建/复核报告（restorer-eco-fresh，2026-10-17）

> 依据 `2026-10-16-logic-verify-protocol.md` 与复原 SOP。全部结论只以本轮
> fresh 反编译/反汇编为准（Ghidra `--worker shared`，2026-10-17 19:32–19:48）：
> `0x140b199e0 / 0x14094a910 / 0x14094cf90 / 0x140997410 / 0x140998dc0 /
> 0x14094cc60 / 0x1408c1580 / 0x140a9b4e0 / 0x140944070 / 0x141308470 /
> 0x1413084c0 / 0x14094a910 反汇编 / 0x140998dc0 反汇编 / 0x140997410 反汇编 /
> 0x1417d11d0 / 0x1417d1940 / 0x1417d1b90`；常量经 `read_memory` 逐字节确认。
> 证据落点：`planning/jobs/ivy-fresh-restorer/decomp|decomp2/*`。

## 一、逐函数 verdict 与修正

| # | 函数 | VA | verdict | 本轮修正/证据 |
|---|---|---|---|---|
| 1 | WallCoord::into_world_space | 0x140b199e0 | **判等** | fresh 反编译确认 `out.y = v`（offset 向量 y 槽恒 0），`out.x/z = interp ± 0.63·0.5·normal`；`0x140b19b72..ba7`。现实现已是 `Vec3::new(x±offset_x, self.v, z±offset_z)`，无误加 Y。 |
| 2 | ivy_grower（守卫体） | 0x14094a910 + 体 0x14094ac51 | **已重建（1 项登记）** | 见 §2。修正 4 处公式/门；再拆解 0x14094aef9..bfb1 反汇编。 |
| 3 | ivy_spawner | 0x14094cf90 | **已重建** | 见 §3。垂直 u 由“clamp”改为“越界拒绝”；其余逐条判等。 |
| 4 | ivy_pruner | 0x140997410 | **已重建** | 见 §4。7 参签名（0x1414bf4e2 typo 证明）、双事件脏墙、PrevWallHoles、尾带门方向修正。 |
| 5 | is_point_still_valid | 0x140998dc0 | **已重建** | 见 §4。洞 AABB 命中 → **无效**（0x140999285..2ad `XOR EAX,EAX`），此前模型方向相反。 |
| 6 | intersect_ivy_growth_w_wall_segment | 0x14094cc60 | **已恢复（非死代码）** | 调用点 = `check_for_wall_jump::closure_env$1`（0x141308501 / 0x1413085fa），前次“删除”错误。见 §5。 |
| 7 | ivy_send_to_gpu_leaf | 0x1408c1580 | **已重建（1 项登记）** | `[device+0x1d]` → scale 1.0/0.85；48B 打包；SsboId 原键；**补二次 IvyFlowerSsbo `Take(0x3e8)` 通道**。见 §6。 |
| 8 | ivy_send_to_gpu_segments | 0x140a9b4e0 | **判等（字段按 shader）** | `wall_normal` = 段切线的水平垂线；`arch_height` = 所属墙 arch。closure 0x1417d1b90 另见 §6 登记 gable 修正项。 |
| 9 | IvySpawnNoise::new | 0x1409440c0 内联 | **判等（结构）** | 维持前判；阈值 -0.02 消费于 spawner 噪声门。 |

## 二、ivy_grower 精确式（0x14094aef9..0x14094bfb1）

fresh 反汇编（`ivy_grower_guard_body_0x94ac51.txt` + 本轮重反汇编）逐条：

1. **`[wall-0x270]` = curve length，不是墙高**。桶值基址 = `bucket+8`，
   `bucket-0x270 = value+0x30` = `Curve2.length`（与 into_world_space 的
   `[in_R8+0x30]` 同字段）。故：
   - `WallCoord.u = length * approx_u + lateral`（0x14094b5a5/b703）；
   - 接受条件 `0 <= u <= length`（0x14094b71c/b72f），不是 `next.y <= 墙高`；
   - `v = tail.y + direction.y*0.21` 对比 `arch+band`（0x14094b661/b665、
     b743..b77a）。
2. **侧向符号**：`cross = (tail.x - interp.x)*tangent.y -
   (tail.z - interp.z)*tangent.x`；`cross > 0 → -direction.x*0.21`，否则
   `+`（0x14094b6d9..b6ff）。此前 `direction·normal` 判号错误。
3. **into_world_space 的 side 参数** = `Curve::is_on_right_side(tail)`
   （0x14094b7a0 call 0x141469000 → r9d），与侧向符号是两个不同判断。
4. **待加入点方向** = 落点处 `get_wall_double_sided_normal` 归一化
   （0x14094be99 → 0x14094bee7..bef2），不是 tail 处法线。
5. panic 三处：主路径 0x14094c1e3、跳墙两候选 0x14094c200/21d；条件
   `points<2 || length<=0`（0x14094b31d/b32c）。
6. `u` 无推进：直接用 `approx_u`（0x14094b59d）；保持前次修正。
7. 同段 `check_for_wall_jump` 重建（见 §5）。

登记（未逐位定案）：跳墙后落点的 SIMD 校正 0x14094ba5d..0x14094bd34
（normal 选号、project_point、按剩余距离缩放后 `distance<0.42`）。现实现按
“候选墙重算 GrowthProposal + 0.42 门”重建结构；该段的逐指令数值等价待补。

## 三、ivy_spawner（0x14094cf90）

逐条判等（fresh 反编译）：
- 首门 `[GladeSettings+0x1c]==1`（0x14094cffb..d00a）✓。
- 预算 `acc = Σcount-(n-1)`，`acc+0x12 > 39999 → return`（0x14094d09b..a7）✓。
- 分派 `ActionSetStates+0x120`（= `MainActionSet.digital[36]`，0x170 布局）
  的 `disc<3`（0x14094d2e7；`pressed()` 即 disc<3）✓。
- 路径 A：cell→closest_segment(0.5)→段命中<0.365→墙；`is_on_right_side`
  选 ±0.34（0x14094d6b5..786）✓。
- 路径 B：随机墙 + `uniform_f32` + FastNoise 严格 > `[res+0x78]`；仅
  `subtype>1` 才 `get_bool`，幅度 ±0.34（0x14094d565..6b5）✓。
- root 分离：`facing_dot >= 0 && dist² < 0.25/2.56`（0x14094d8dd/d95d）✓。
- attempts 共享 `2+usize_less_than(3)`（0x14094d2e7/d702）✓。
- 格内 `is_inside_bbx` 复核（0x14094d763..d809）✓（旋转细节近似，登记）。
- **修正**：垂直 u = `(rand*1.5-0.75)/curve_length + clamp(approx_u)`
  （0x14094db23，除数 `[wall-0x270]`=curve length）；随后
  `0<=v<=1` 才 spawn，否则**跳过本次尝试**（0x14094db32..）；NaN/∞ panic。
  现实现返回 `Option<f32>`，不再把越界值 clamp 到 0/1。
- 每尝试预算 `0x9c41-acc`、`cap=min(0x46,·)`、`acc+=cap-1`、
  `0x13<· && acc<0x9c41`（0x14094db6e..b9e）✓。

## 四、ivy_pruner / is_point_still_valid

**签名（fresh xref）**：
`(ResMut<IvyStorage>, ResMut<IvyLeafStorage>, Res<PublicWalls>,
Res<WallSpatialHash>, Res<PrevWallHoles>,
EventReader<CmdCheckForSheepInsideBuilding>, EventReader<OnCircleRotated>)`
（0x1414bf4e2 的 FunctionSystem 单态名逐字给出）。

- 脏墙过滤（0x1409979b2..ae3）：事件 +0x08 = WallId → PublicWalls 查表 →
  墙无效 panic；对每株 ivy，用墙的 `Circle2d`/`Rectangle2d`
  （shape tag `&1`）对 **tail 与 root 的 XZ** 做 `contains_point`，命中才
  进入复验。现实现按此过滤。
- 复验：tail 无效→弹尾+清 growth/pending+删叶，循环；tail 有效后高度带门
  `0x140997bda..b30`：**`tail.y < band` 才停止，`>= band` 则弹尾**
  （此前方向相反，已修）。root 无效→弹根+刷新缓存 root。空株收集后
  `remove_entry` 整株删（0x140998711..1312）。
- 尾带：`arch + (flag? +0.2 : subtype<2 ? -0.79-0.1 : -0.46)`
  （0x142a7e320=-0.79、0x1429eb6f8=-0.1、0x142a7f250=-0.46、
  0x1429cd068=+0.2）✓。
- `is_point_still_valid`：closest_segment(0.5) <0.365 → 墙；墙无效
  （points<2 / length<=0）panic；u 带 `(−0.01/length, 1+0.01/length)`；
  **FinalizedWallHoles 的 AABB（±0.1，0x142abaab0）命中 → 返回 0（无效）**；
  无洞表/无命中 → 返回墙指针（有效）。返回值模型为 `Option<WallId>`。
  洞探针 = `(approx_u·[wall-0x14], world_y)`；`[wall-0x14]` 取 curve length
  （同一字段族），逐字段定案待补。

## 五、intersect_ivy_growth_w_wall_segment（0x14094cc60）恢复

- 调用点：`check_for_wall_jump` 的 filter-map closure（0x141308501/5fa），
  两处 `CALL 0x14094cc60`；closure 以 `(distance, hit, WallId)` 建候选。
- 算法逐条（0x14094ccb4..cf22）：
  1. 段帧 `u=dir/len, n=(-u.y,u.x)`，中点 `mid`；
  2. `q,s` 投影到 `(n,u)`；调用 `utils::geometry::aabb::intersection_aabb_segment2`
     （0x141240b30），盒 `[-0.315,0.315] × [-0.5len,0.5len]`
     （常量 0x142ab31b4=-0.315/0x142aa3280=+0.315、0x1429258e0=0.5、
     0x14295a0e8=-0.5）；
  3. 命中点回投到墙段 `t=dot(mid+offset-p0,dir)/len²` 并 clamp；
  4. 返回 `seg_point ± 0.315·u` 中按 `d_b>d_a` 选择者（0x14094cedc..cf1f）。
- 现实现逐式对应（`ROOT_BOX_HALF = 0.63*0.5`）。
- `check_for_wall_jump` 重建为
  `query_with_segment(tail, proposed)` → 剔除当前墙（0x1412f6b60）→
  逐段 intersect → 距离排序取最近（0x141338fe0/0x140b4baa0）。

## 六、GPU 两条链（0x1408c1580 / 0x140a9b4e0）

- leaf：`[device+0x1d]` 决定 scale `0x3f59999a(0.85)` / `1.0`
  （0x1408c1631/181d）；48B 打包；成功上传后写 instance count；SsboId 原键
  `0x9e3cfa7f2993f406` / `0x1d6de5e69ac50d2a` 保留。
- **二次 IvyFlowerSsbo 通道**：同一 device 位为真时，
  `filter_map(closure_env$1).take(0x3e8)` 打包上传（0x1408c1e..）。
  已补 `pack_flower` / `upload_ivy_flowers` / `FLOWER_LIMIT=1000`；
  flower 用 `vector_58`（closure 0x1417d1940 读 +0x58/+0x60，替代 leaf 的
  +0x2c/+0x34）。两个 closure 用同一捕获浮点做 `rand < p` / `p <= rand`
  互补划分（0x1417d11d0/1940）；该概率来源待取证，接口以 `flower_rate`
  参数暴露。
- segments：closure 0x1417d1b90 先按 `ivy.source_60` 查墙并 panic 校验，
  再算 arch；`wall_normal` = 段切线水平垂线，`arch_height` = 墙 arch。
  登记：native arch = `[wall-0x18]`，当 `[wall-0xc]!=0` 时再加
  `|wall-0x28 - wall-0x2c|`（gable 修正）；本库暂用 wall arch，差值字段
  映射待取证。

## 七、编译与测试

- `cargo check -p country-core --lib`：**通过**，ivy 文件 0 warning。
- `cargo test -p country-core --lib`：**1974 passed / 0 failed**（并发 WIP
  曾短时破坏测试目标，已复跑稳定）。
- 域过滤：`systems::ivy` **25 passed**；`critters::sheep` 6；`garden` 20；
  `startup_garden_brush` 1。新增断言：intersect 盒角结果、curve 帧/右侧、
  flower `vector_58` + `Take(0x3e8)`、vertical_u 越界拒绝。
- 临时 integration probe（已删除）3 项全过，用于他组 WIP 阻断 lib test 时局部复证。

## 八、登记 / 解锁条件

1. **0x14094ba5d..bd34 跳墙落点 SIMD**：需逐指令数值等价；现按结构重建。
2. **spawner 格内 `is_inside_bbx` 旋转**：现 XZ 轴对齐包含；需
   `from_rotation_arc` 逐指令。
3. **洞探针缩放 `[wall-0x14]`**：现取 curve length；需字段定案。
4. **flower/leaf 概率来源**：需 decompile 捕获环境；接口已留参。
5. **segments gable arch 修正**：需 `[wall-0x18]/[-0x28]/[-0x2c]/[-0xc]`
   到 PublicWallState 字段映射。
6. **`resources::events::OnCircleRotated` 跨域**：本库未声明；pruner 现用
   `OnWallChanged` 等价脏墙源，事件落地后切回 7 参原生签名。
7. SsboId `SsboId::of(NAME)` 与原键仍不命中（共享 ssbo_library 层缺口，
   原键常量已保留）。
