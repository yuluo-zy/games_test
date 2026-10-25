# T3 CONTRACT — sheep register_required_components 目标组件 + 后续取证

> 逆向 agent ① 交付。证据：procs.tsv（5 个 impl 的 register_required_components 地址）+
> 反编译 + 组件名串 0x142c3xxxx 区。状态：**5 宿主组件定案；每宿主精确 required 清单待取证**。

---

## 1. 5 个 register_required_components 的宿主组件（O）

| impl | 地址 | 宿主组件（组件名串） |
|---|---|---|
| impl$1 | 0x14089ec50 | **SheepAudio**（0x142c384ea "country_core::resources::critters::sheep::SheepAudio"） |
| impl$7 | 0x14089f340 | **Sheep**（0x142c3652c） |
| impl$8 | 0x14089fa30 | **UmbrellaHolder**（0x142c36a9f） |
| impl$9 | 0x1408a0120 | **SheepAccessory**（0x142c37bb5） |
| impl$10 | 0x1408a0810 | **FloatieHolder**（0x142c36fd6） |

→ 原生 `resources::critters::sheep` 有 5 个 ECS 组件（Sheep、SheepAudio、UmbrellaHolder、
SheepAccessory、FloatieHolder）都带 `#[require(...)]`。当前源码只有 `Sheep`（+ control），
SheepAudio/UmbrellaHolder/SheepAccessory/FloatieHolder 未落（组外/待恢复组件）。

## 2. 每宿主 required 清单 — **待取证**

- decomp（round4/decomp-sheep-0x14089ec50..0810.txt，各 18KB Bevy 注册样板）未直接暴露
  required 组件名；组件名串 0x142930710 区为构建期 XXXX 屏蔽。
- **可落地近似（O 侧支撑）**：sheep_unstuck 系统 Query = `(&Transform, &mut CritterLocomotion,
  &mut Sheep)`（public-session-names entry 15）→ Sheep 至少 required = `Transform` +
  `CritterLocomotion`（+ Bevy 标准 `Visibility` 族候选）。SheepAudio 挂 audio 属性组件候选。
- **不编造**：② 若补 `#[require]`，先按 Sheep=Transform+CritterLocomotion 落地并标注 I，
  其余 4 个宿主组件本身待恢复（组件字段/结构 U）。

## 3. sheep_controls picking 分支（O，0x1416f11a0，round4/decomp-sheep-0x1416f11a0.txt 25KB）

- 仲裁已批准 `systems/debug/sheep_controls` 落地（SheepControl 资源 + query
  (Entity,&mut Transform,&mut Sheep,&mut CritterGoal) + apply_sheep_snapshot@0x1416f24f0）。
- 任务原题"PickingMode 拾取后写 move_state=0x101 + timeout_s@+0x34"：**待取证**——0x1416f11a0
  反编译未直接现 PickingMode/move_state=0x101 写面（25KB 内未定位到 0x101 立即数写入；
  move_state=0x101 = `{Walking, goal_change_timeout=1, ...}` 判别候选）。② 保持现实现，
  picking 分支登记待取证，不造。

## 4. 证据索引
- procs.tsv：0x14089ec50/0x14089f340/0x14089fa30/0x1408a0120/0x1408a0810（impl$1/7/8/9/10
  register_required_components）；0x1416f11a0（sheep_controls）。
- round4/decomp-sheep-*.txt（6 份反编译）。
- rne-read：0x142c384ea/3652c/36a9f/37bb5/36fd6（宿主组件名串）。
