# B 组 sheep SPEC（2026-10-15 第二轮）

> 组：B 生物。文件所有权：`crates/country-core/src/resources/critters/*`。
> 现状：`resources/critters/sheep/control.rs` 的 capture/restore（借字段、按 query 序 zip、短输入停、不动 rotation/scale/其它字段）已实现并通过 2 个测试。
> 本 SPEC 覆盖 contracts.json 剩余缺口：**完整 Sheep component owner 与生命周期、真实 Sheep query 所有者、save/load 接线、spawn/movement**。
> 证据基线：RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；
> 目录 `evidence/core-recovery-20261015/critters/*.asm`、`evidence/country-core-deep/sysres-procedures-v2.json`、
> `evidence/country-core-deep/SOURCE_MAP.json`、`evidence/country-core-deep/chapters/`。
> O=机器码证实；I=推断；U=未知。**旧草稿的"movement 谓词"不要抄，本 SPEC 以原机码为准。**

---

## 1. 剩余缺口清单

| # | 缺口 | 结论 | 见 § |
|---|---|---|---|
| 1 | 完整 Sheep component schema | 部分 O（stride 0x68、move_state@+0x30、being_petted f32@+0x58），其余区间无 assembly 证据 = **待取证** | §2.1 |
| 2 | SheepMoveState 枚举（wire/内存判别） | **O 已实现**，本 SPEC 复述供接线 | §2.2 |
| 3 | 真实 Sheep query 所有者 | **O**：`systems/debug/sheep_controls`（0x1416f11a0）用 `(Entity, &mut Transform, &mut Sheep, &mut CritterGoal)` | §2.5, §3 |
| 4 | save/load 接线 | **O**：`systems/save/save_game.rs` `to_json_string<Option<SheepControlSnapshot>>` @0x140a51d40 = "Sheep" 节载荷 | §2.5, §3 |
| 5 | spawn 行为 | **O（部分）**：`startup_sheep` @0x140a093e0 是资产载入**不是实体 spawn**；实体 spawn 函数不在 country-core sysres map = **待取证** | §2.6 |
| 6 | movement 状态机 | **待取证**：Idle↔Walking 转移不在 country-core sysres map；sheep_controls decomp 可见写 move_state tag=Walking+force_reach_goal 与 timeout_s | §2.6 |
| 7 | being_petted 阈值常量 | 半 O：比较常量 VA 0x142930f74，raw 字节 `2f290000`；f32 解码 ≈1.5e-41（≈0），语义"正值非零"待实现复核 | §2.1 |

---

## 2. 原行为契约

### 2.1 Sheep component（stride 0x68 = 104 B）

来源：capture iterator 0x1412f03f0（索引 `uVar*0x68`）、apply_sheep_snapshot 0x1416f24f0、
sheep_controls 0x1416f11a0（`uVar9 * 0x68`）、`Sheep::being_petted` 0x14089d4c0。

已证实字段（O）：

| 偏移 | 大小 | 字段 | 证据 |
|---|---|---|---|
| +0x30 | 1B | SheepMoveState 判别字节（0=Idle、1=Walking；`cmp byte ptr [rcx],0` 同族） | snapshot-analysis + 0x1412f064f..66f |
| +0x31 | 1B | `force_reach_goal: bool` | 同上（0x1412f0663 读 `[r9+0x30]` qword = tag+bool+timeout_s） |
| +0x34 | 4B | `timeout_s: f32` | 同上 |
| +0x38 | 4B | `goal_change_timeout: f32` | 0x1412f066b 读 `[r9+0x38]` dword |
| +0x58 | 4B | `being_petted` 谓词读的 f32 | 0x14089d4c0 `movss xmm0,[rcx+0x58]`；`ucomiss` 与常量比较 |

**being_petted 谓词（O）**：`0x14089d4c0` = `movss xmm0,[rcx+0x58]; ucomiss xmm0,[rip+0x20c31a8]; seta al; ret`。
常量 VA = 0x14089d4c5+7+0x20c31a8 = **0x142930f74**，raw 字节 `2f 29 00 00`（f32 u32=0x292f ≈ 1.48e-41，
基本等价">0"）。注意 0x142930f74 位于打包数据区，阈值语义实现时须按原字节复核（见 §4 待取证 #T3）。

未证实区间（U，**待取证**）：`+0x00..+0x2f`、`+0x3c..+0x57`、`+0x5c..+0x67`。
这些可能承载动画状态/杂项（GPU 侧有 `SheepAnimationSsbo`），但**没有任何 assembly 证据**；
不要凭空命名/补默认值。**完整 schema 需要 PDB 类型（`skeleton/_pdb/B/` 未落盘）或更多消费函数取证。**

### 2.2 SheepMoveState（已实现，wire 形状 O）

- `Idle { timeout_s: f32 }`（tag 0）→ `{"Idle":{"timeout_s":v}}`
- `Walking { timeout_s: f32, goal_change_timeout: f32, force_reach_goal: bool }`（tag≠0）→ `{"Walking":{"timeout_s":v,"goal_change_timeout":v,"force_reach_goal":v}}`
- 变体名 `"Idle"`@0x142c5b4cf(len4)/`"Walking"`@0x142c5b4dc(len7)；键串 `timeout_s`@0x142c5b4d3(len9)、`goal_change_timeout`@0x142c5b4e3(len0x13)、`force_reach_goal`@0x142c5b4f6(len0x10)。
- 证据：serialize 0x1413ff520、Debug `fmt` 0x1416f2800、deserialize 0x1413ff8a0/0x1413ffb60。
- 内存序 ≠ 序列化序：判别@+0、force_reach_goal@+1、timeout_s@+4、goal_change_timeout@+8（O，snapshot-analysis）。

### 2.3 CritterGoal（已实现）

16 B `Option<Vec3>`（判别@+0 + Vec3），`#[serde(transparent)]`。缺失 goal 无位置载荷。证据：capture/restore 从 16B-stride 列拷贝。

### 2.4 SheepSnapshot / SheepControlSnapshot（已实现，内存布局 O）

`SheepSnapshot` 内存布局（O，0x1412f05fa 填充）：
- **+0x00** `goal: Option<Vec3>`（16 B，`movups` 整块）
- **+0x10** `move_state: SheepMoveState`（12 B：8 B 从 Sheep+0x30 + 4 B 从 Sheep+0x38）
- **+0x1c** `position: Vec3`（12 B：8 B 从 Transform+0x10 + 4 B 从 Transform+0x18）
- 总大小 0x28 = 40 B（0x1416f24f0 步进 `puVar15 += 5` = +0x28 确认）。

序列化字段序 `position, goal, move_state`（0x1416f9eb0 依次 `serialize_entry`）；`SheepControlSnapshot` = `{"sheep":[...]}`（0x1416f9fd0 写键 `"sheep"` len5）。

**Transform（utils，O）**：`{rotation Quat@0x0, translation Vec3@0x10, scale Vec3@0x1c}`，stride 0x30。translation.x@0x10/y@0x14/z@0x18。

### 2.5 消费链（真实消费者接线）

```
capture 生产者: systems/debug/sheep_controls::sheep_controls @0x1416f11a0
    query = (Entity, &mut Transform, &mut Sheep, &mut CritterGoal)     [O]
    → capture_sheep_snapshot closure（0x1412f03f0，map over QueryIter） [O]
    → from_iter 0x141304bd0 → Vec<SheepSnapshot>                        [O]
    → SheepControlSnapshot（resource 内 / 输出）
restore 消费者: systems/debug/sheep_controls::apply_sheep_snapshot @0x1416f24f0
    → zip(query 序, snapshot 序) 短路；只写 Transform.translation、
      CritterGoal、Sheep.move_state；不动 rotation/scale/其它字段         [O]
save 节:      systems/save/save_game::impl$2::to_json_string<Option<SheepControlSnapshot>>
              @0x140a51d40 —— "Sheep" 节载荷（save_game.rs 10 节协议第 5 节） [O]
audio 消费者: systems/audio/sheep_audio::sheep_audio_system @0x1409d5ea0
              （PlaySheepReactSound kind 0..7 + 参数 FirstPersonOff）       [O]
render（后置）: render_loop::resources::critters::SheepAnimationSsbo（GPU 侧）
```

**关键接线事实**：
1. `capture_sheep_snapshot` 与 `apply_sheep_snapshot` 都是 `systems/debug/sheep_controls` 的成员，
   **不是独立 ECS 系统**；它们是 SheepControl 调试资源驱动的一对函数。
2. 快照不存 Entity ID、不排序；恢复按 query 实际顺序（query 所有者顺序）zip。
3. save_game 的 "Sheep" 节是 `Option<SheepControlSnapshot>` —— 无羊则 JSON null。

### 2.6 spawn 与 movement（关键结论）

- **`startup_sheep` @0x140a093e0（47.8KB，O）是资产载入，不是实体 spawn**：
  载入 `assets/meshes/sheep_animation/{1,delighted}.json`、`sheep_heart`、`umbrella`、`floatie` 网格 +
  `sheep_shader` + FMOD `event:/SFX/Sheep/Baah`（参数 `Pitch`）+ 动画网格注册（FUN_14130efe0/1412e23e0/1416f0ea0）。
- **Sheep 实体 spawn：不在 country-core sysres map**（SOURCE_MAP/sysres-procedures-v2 全量搜 `sheep` 无 spawn 函数）。
  邻域 crate 符号显示 `system_critters`（`system_critters::duck::update_duck_ripples`、
  `system_critters::wren::...`）存在；Sheep 实体 spawn 大概率在该 crate（**待取证**；不在本组文件所有权内，
  先落地组件契约与消费者接线，spawn 留真实接口）。
- **movement 状态机：Idle↔Walking 转移不在 country-core sysres map**（**待取证**）。
  `sheep_controls` decomp（0x1416f11a0）可见写 `Sheep+0x30` 两字节 `0x101`（= Walking + force_reach_goal）
  与 `Sheep+0x34` 写 `_UNK_142abcb20`（timeout_s 常量）；这属于调试拾取后的状态写入，不是全局 movement 驱动。
  不要照抄旧草稿的"movement 谓词"。

---

## 3. 可直接落地项（实现 agent 优先）

1. **`resources/critters/sheep.rs` 补 `Sheep` component**：用 §2.1 已证实字段落地。
   ```rust
   #[derive(Component, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
   pub struct Sheep {
       pub move_state: SheepMoveState,   // +0x30（12B 内嵌）
       pub being_petted_timer: f32,      // +0x58
       // 其余区间待取证 —— 结构体只含已证实字段，勿填未知占位
   }
   ```
   约束：布局注释标注 +0x30/+0x58；未知区间留文档说明，不补默认字段。
2. **接线 capture/restore 到真实消费者**：
   - `systems/debug/sheep_controls.rs`：`SheepControl::new` @0x1416f0ea0、`sheep_controls` @0x1416f11a0
     （query `(Entity, &mut Transform, &mut Sheep, &mut CritterGoal)`）、`apply_sheep_snapshot` @0x1416f24f0、
     `PickingMode::is_some`（内联）。
     该文件**在组外**（systems/debug/），落地需经中央 agent 仲裁；或由 B 组提供纯函数接线供其调用。
   - `save_game` "Sheep" 节：`to_json_string<Option<SheepControlSnapshot>>` —— save 域文件在组外，登记接口即可。
3. **`Sheep::being_petted` 谓词**：`self.being_petted_timer > f32::from_bits(0x292f)` 或按实现复核常量。
4. **`sheep_audio_system` @0x1409d5ea0**（组外 systems/audio/）：登记 kind 表 0..7 + FirstPersonOff，作为 Sheep 消费链证据。

---

## 4. 需先取证项（不落地、登记）

- **T1 完整 Sheep 字段布局**：`+0x00..+0x2f`、`+0x3c..+0x57`、`+0x5c..+0x67` 无 assembly 证据。取证路径：
  PDB 类型（`skeleton/_pdb/B/` TPI）、或从更多消费函数（sheep_controls 完整 decomp、sheep_animation GPU 上传、
  原 `system_critters` sheep 驱动）读取偏移。**不凭名字猜字段。**
- **T2 Sheep 实体 spawn**：country-core 内无 spawn 函数；确认 `system_critters` crate 是否持有（其还原不在本组）。
- **T3 movement 状态机**：Idle↔Walking 转移谓词/超时（timeout_s、goal_change_timeout、force_reach_goal 的消费方）。
  证据缺口：无 country-core 系统触碰 move_state（除 sheep_controls 调试写）。**旧草稿 movement 谓词作废。**
- **T4 being_petted 常量语义**：0x142930f74 位于打包数据区，raw `2f290000`；复核真实 f32 值后再定谓词语义。
- **T5 `register_required_components` ×5（0x14089ec50/0x14089f340/0x14089fa30/0x1408a0120/0x1408a0810）**：
  Bevy required-component 注册胶水（impl$1/7/8/9/10），按 AGENTS.md 属胶水不计独立业务；落地用
  `#[require(...)]` 覆盖即可，但**哪些组件被 require（Sheep→?）需与 PDB/消费核对**。

---

## 5. 排除项（明确不做/后置）

- GPU 渲染：`SheepAnimationSsbo` / `draw_instanced_*`（render_loop）后置。
- `system_critters` crate 内（duck/wren/butterfly/firefly/sheep 实体驱动）：不在本组。
- 原 `CritterGoal`/`SheepMoveState` 之外的 critters 类型（BirdSpotManager 等）：resources/critters.rs 范围外。
