# components/mod.rs

> 文件作用：**游戏组件集总模块：44 个 ECS 组件类型（marker / 参数组件）。本文件主要业务面 = 每个组件的 `register_required_components`（Bevy required-components 接线）。组件类型清单已由 reconstruction mod.rs 按 WP6 恢复（含 typed gap）。**

函数总计 76：业务 54 / 内联无独立符号 1 / 派生胶水 21

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::components::impl$2::register_required_components` | 0x140844f20 | 23 | 胶水：策略归类 — Bevy `register_required_components` 单态化胶水（bevy_ecs 0.16 component.rs）。**仅注册自身**：排队 Self 描述符（QueuedRegistration 0xd0B）+ `HashMap<TypeId,ComponentId>::insert`（键=自身 TypeId u128）+ 经 `register_component_unchecked` 递归自身（indices 命中终止）+ 递归栈 push/pop；**无 required 组件**（TypeId movabs 常量仅自身一对）。组件名=FollowMouse。 | **O** | decomp-0x140844f20.txt + movabs 0x25811ca3a04e00db/0xf006cb42c80dc9f0 |
| `country_core::components::impl$3::register_required_components` | 0x140845610 | 31 | 胶水：策略归类 — 同上（组件名=TestCube）。 | **O** | decomp-0x140845610.txt |
| `country_core::components::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x14036a4f0 | 51 | serde_json CompactFormatter 整数序列化（迭代 /10000 数字化，UNK_1429c6964 数字表）。derive 胶水。 | **O** | decomp-0x14036a4f0.txt |
| `country_core::components::impl$15::register_required_components` | 0x140845d00 | 69 | 胶水：策略归类 — 同上（组件名=TreeComponent）。 | **O** | decomp-0x140845d00.txt |
| `country_core::components::impl$16::register_required_components` | 0x1408463f0 | 76 | 胶水：策略归类 — 同上（组件名=ConvertedPreplacedTree）。 | **O** | decomp-0x1408463f0.txt |
| `country_core::components::impl$17::register_required_components` | 0x140846ae0 | 83 | 胶水：策略归类 — 同上（组件名=TreeAnimation）。 | **O** | decomp-0x140846ae0.txt |
| `country_core::components::impl$18::register_required_components` | 0x1408471d0 | 92 | 胶水：策略归类 — 同上（组件名=TreeMoveBend）。 | **O** | decomp-0x1408471d0.txt |
| `country_core::components::impl$19::register_required_components` | 0x1408478c0 | 98 | 胶水：策略归类 — 同上（组件名=TreeMarkedForDeletion）。 | **O** | decomp-0x1408478c0.txt |
| `country_core::components::impl$20::register_required_components` | 0x140847fb0 | 102 | 胶水：策略归类 — 同上（组件名=StairsStep）。 | **O** | decomp-0x140847fb0.txt |
| `country_core::components::impl$21::register_required_components` | 0x1408486a0 | 108 | 胶水：策略归类 — 同上（组件名=ArrowGizmo）。 | **O** | decomp-0x1408486a0.txt |
| `country_core::components::impl$22::register_required_components` | 0x140848d90 | 115 | 胶水：策略归类 — 同上（组件名=SnappingToEndPreview）。 | **O** | decomp-0x140848d90.txt |
| `country_core::components::impl$23::clone_behavior` | 0x140002370 | 119 | 胶水：策略归类 — 共享 `clone_behavior` = `xor eax,eax; ret`（返回 0 = `ComponentCloneBehavior::Default`，无自定义克隆 handler；ICF 折叠共享）。组件名=SnappingToSegmentPreview。 | **O** | 机器码 0x140002370（31c0c3；Ghidra NotFound 手解） |
| `country_core::components::impl$23::register_required_components` | 0x140849480 | 119 | 胶水：策略归类 — 同上（组件名=SnappingToSegmentPreview）。 | **O** | decomp-0x140849480.txt |
| `country_core::components::impl$24::register_required_components` | 0x140849b70 | 123 | 胶水：策略归类 — 同上（组件名=DashedLineLength）。 | **O** | decomp-0x140849b70.txt |
| `country_core::components::impl$25::register_required_components` | 0x14084a260 | 127 | 胶水：策略归类 — 同上（组件名=DontReceiveContactShadows）。 | **O** | decomp-0x14084a260.txt |
| `country_core::components::impl$26::register_required_components` | 0x14084a950 | 132 | 胶水：策略归类 — 同上（组件名=SemiTransparent）。 | **O** | decomp-0x14084a950.txt |
| `country_core::components::impl$27::register_required_components` | 0x14084b040 | 137 | 胶水：策略归类 — 同上（组件名=FlatLighting）。 | **O** | decomp-0x14084b040.txt |
| `country_core::components::impl$28::register_required_components` | 0x14084b730 | 143 | 胶水：策略归类 — 同上（组件名=NormalMap）。 | **O** | decomp-0x14084b730.txt |
| `country_core::components::impl$29::register_required_components` | 0x14084be20 | 150 | 胶水：策略归类 — 同上（组件名=OverwriteRoughness）。 | **O** | decomp-0x14084be20.txt |
| `country_core::components::impl$30::clone_behavior` | 0x14084cc00 | 158 | 胶水：策略归类 — `lea rdx,[组件类型表]; mov eax,2; ret`（返回 2 = `ComponentCloneBehavior::Custom(component_clone_via_clone::<T>)`）。组件名=ClothlineCloth（derive Clone → autoderef 特化）。 | **O** | 机器码（Ghidra NotFound 手解）+ public-symbols |
| `country_core::components::impl$30::register_required_components` | 0x14084c510 | 158 | 胶水：策略归类 — 同上（组件名=ClothlineCloth）。 | **O** | decomp-0x14084c510.txt |
| `country_core::components::impl$34::clone_behavior` | 0x14084d300 | 166 | 胶水：策略归类 — `Custom`（组件名=GothicWindowBrickDeformation）。 | **O** | 机器码 + public-symbols |
| `country_core::components::impl$34::register_required_components` | 0x14084cc10 | 166 | 胶水：策略归类 — 同上（组件名=GothicWindowBrickDeformation）。 | **O** | decomp-0x14084cc10.txt |
| `country_core::components::impl$38::clone_behavior` | 0x14084da00 | 178 | 胶水：策略归类 — `Custom`（组件名=BrickColor）。 | **O** | 机器码 + public-symbols |
| `country_core::components::impl$38::register_required_components` | 0x14084d310 | 178 | 胶水：策略归类 — 同上（组件名=BrickColor）。 | **O** | decomp-0x14084d310.txt |
| `country_core::components::impl$42::register_required_components` | 0x14084da10 | 194 | 胶水：策略归类 — 同上（组件名=Ui2DFollowMouse）。 | **O** | decomp-0x14084da10.txt |
| `country_core::components::impl$43::register_required_components` | 0x14084e100 | 199 | 胶水：策略归类 — 同上（组件名=UiInactive）。 | **O** | decomp-0x14084e100.txt |
| `country_core::components::impl$44::register_required_components` | 0x14084e7f0 | 205 | 胶水：策略归类 — 同上（组件名=Ui2dExtent）。 | **O** | decomp-0x14084e7f0.txt |
| `country_core::components::impl$46::register_required_components` | 0x14084eee0 | 213 | 胶水：策略归类 — 同上（组件名=Ui2dWsExtent）。 | **O** | decomp-0x14084eee0.txt |
| `country_core::components::impl$48::register_required_components` | 0x14084f5d0 | 221 | 胶水：策略归类 — 同上（组件名=Ui2dImage）。 | **O** | decomp-0x14084f5d0.txt |
| `country_core::components::impl$49::register_required_components` | 0x14084fcc0 | 237 | 胶水：策略归类 — 同上（组件名=UiInteractionState）。 | **O** | decomp-0x14084fcc0.txt |
| `country_core::components::impl$50::register_required_components` | 0x1408503b0 | 247 | 胶水：策略归类 — 同上（组件名=UiMouseInteractable）。 | **O** | decomp-0x1408503b0.txt |
| `country_core::components::impl$52::register_required_components` | 0x140850aa0 | 255 | 胶水：策略归类 — 同上（组件名=UiDrawOrder）。 | **O** | decomp-0x140850aa0.txt |
| `country_core::components::impl$56::register_required_components` | 0x140851190 | 270 | 胶水：策略归类 — 同上（组件名=UiNav）。 | **O** | decomp-0x140851190.txt |
| `country_core::components::impl$58::clone_behavior` | 0x140851f70 | 279 | 胶水：策略归类 — `Custom`（组件名=HistoryButton）。 | **O** | 机器码 + public-symbols |
| `country_core::components::impl$58::register_required_components` | 0x140851880 | 279 | 胶水：策略归类 — 同上（组件名=HistoryButton）。 | **O** | decomp-0x140851880.txt |
| `country_core::components::impl$64::register_required_components` | 0x140851f80 | 286 | 胶水：策略归类 — 同上（组件名=MeshFollowTerrain）。 | **O** | decomp-0x140851f80.txt |
| `country_core::components::impl$65::register_required_components` | 0x140852670 | 291 | 胶水：策略归类 — 同上（组件名=KeepOnLevelUnload）。 | **O** | decomp-0x140852670.txt |
| `country_core::components::impl$66::register_required_components` | 0x140852d60 | 295 | 胶水：策略归类 — 同上（组件名=BackgroundTerrain）。 | **O** | decomp-0x140852d60.txt |
| `country_core::components::impl$67::register_required_components` | 0x140853450 | 300 | 胶水：策略归类 — 同上（组件名=WoodenGate）。 | **O** | decomp-0x140853450.txt |
| `country_core::components::impl$68::register_required_components` | 0x140853b40 | 315 | 胶水：策略归类 — 同上（组件名=BalconyDoor）。 | **O** | decomp-0x140853b40.txt |
| `country_core::components::impl$74::clone_behavior` | 0x140854920 | 336 | 胶水：策略归类 — `Custom`（组件名=Smoke）。 | **O** | 机器码 + public-symbols |
| `country_core::components::impl$74::register_required_components` | 0x140854230 | 336 | 胶水：策略归类 — 同上（组件名=Smoke）。 | **O** | decomp-0x140854230.txt |
| `country_core::components::impl$77::clone_behavior` | 0x140855020 | 345 | 胶水：策略归类 — `Custom`（组件名=RoofFlagCloth）。 | **O** | 机器码 + public-symbols |
| `country_core::components::impl$77::register_required_components` | 0x140854930 | 345 | 胶水：策略归类 — 同上（组件名=RoofFlagCloth）。 | **O** | decomp-0x140854930.txt |
| `country_core::components::impl$82::clone_behavior` | 0x140855720 | 351 | 胶水：策略归类 — `Custom`（组件名=WallFlagCloth）。 | **O** | 机器码 + public-symbols |
| `country_core::components::impl$82::register_required_components` | 0x140855030 | 351 | 胶水：策略归类 — 同上（组件名=WallFlagCloth）。 | **O** | decomp-0x140855030.txt |
| `country_core::components::impl$87::register_required_components` | 0x140855730 | 361 | 胶水：策略归类 — 同上（组件名=SplashParticle）。 | **O** | decomp-0x140855730.txt |
| `country_core::components::impl$88::register_required_components` | 0x140855e20 | 368 | 胶水：策略归类 — 同上（组件名=TreeAmbience）。 | **O** | decomp-0x140855e20.txt |
| `country_core::components::impl$89::clone_behavior` | 0x140856c00 | 375 | 胶水：策略归类 — `Custom`（组件名=LanternLightParams）。 | **O** | 机器码 + public-symbols |
| `country_core::components::impl$89::register_required_components` | 0x140856510 | 375 | 胶水：策略归类 — 同上（组件名=LanternLightParams）。 | **O** | decomp-0x140856510.txt |
| `country_core::components::impl$91::register_required_components` | 0x140856c10 | 385 | 胶水：策略归类 — 同上（组件名=AutoClutterConvertableToDecoClutter）。 | **O** | decomp-0x140856c10.txt |
| `country_core::components::impl$93::register_required_components` | 0x140857300 | 413 | 胶水：策略归类 — 同上（组件名=AutoClutter）。 | **O** | decomp-0x140857300.txt |
| `country_core::components::impl$96::register_required_components` | 0x1408579f0 | 439 | 胶水：策略归类 — 同上（组件名=SittableForBirds）。 | **O** | decomp-0x1408579f0.txt |

## 内联/无独立符号函数

- `country_core::components::Ui2dImage::new`（行 229）— **I**：`Ui2dImage::new`（UI 2D 图片组件构造）— 内联无独立符号，功能由消费方（UI/工具栏构造）推断。

## 派生胶水

- 21 项：clone, default, deserialize, eq, fmt, hash, next, partial_cmp, serialize 等（derive 直接复用，不计业务）。

## 算法级结论（round 算法深挖，2026-10-15）

### 1. required-components 策略：全模块 **无 required 组件**

对全部 44 个 register_required_components（连同 drawable 12 个、transient 1 个与全二进制 213 个 register 函数）做 movabs TypeId 常量扫描（typeid-extract 方法：双 64-bit 半常量配对 + 二进制归属解析，`tools/country-core-audit/comp_register_map.py` 本轮新增）：

- 每个函数**仅含自身 TypeId u128 一对（出现 2 次）**（描述符构造 + indices 插入各一次），无任何其他 TypeId 常量 → **没有任何组件声明 `#[require(...)]`**。
- 函数体 = bevy_ecs-0.16.0 `register_required_components` 单态化胶水：`enforce_no_required_components_recursion`（0x140d40d70）→ `register_component::<Self>()`（排队 0xd0B QueuedRegistration：`ComponentInfo::new` 0x140d3f870 + 哈希表 `HashMap<TypeId,ComponentId>::insert` 0x140082c20）→ 经 `register_component_unchecked` 递归调用自身 `register_required_components`（`indices` 命中终止递归）→ 递归栈 push/pop（Vec 操作）。
- 推论：这些组件的实体附着（Transform 等）全部由**外部 bundle / spawn 时显式插入**完成，不依赖 required-components 自动补全。
- 附带证实：游戏自有 `utils::components::transform::Transform` 的 register 函数同样只有自身 TypeId——bevy 官方 0.16 Transform 的 `require(GlobalTransform, TransformTreeChanged)` 在**分叉中不成立**。

### 2. clone_behavior 归类

| 模式 | 机器码 | 语义 | 组件 |
|---|---|---|---|
| 共享 0x140002370 | `xor eax,eax; ret` | `ComponentCloneBehavior::Default`(0)——组件未实现 Clone，无自定义克隆 handler（ICF 折叠共享） | SnappingToSegmentPreview(impl$23)、drawable WireframePass、TransientMesh 及所有未列出的组件 |
| 每 impl 独立 | `lea rdx,[类型表]; mov eax,2; ret` | `Custom(component_clone_via_clone::<T>)`(2)——组件 derive Clone，autoderef 特化给出克隆 handler（rdx=类型表=handler 函数指针） | ClothlineCloth(impl$30)、GothicWindowBrickDeformation(impl$34)、BrickColor(impl$38)、HistoryButton(impl$58)、Smoke(impl$74)、RoofFlagCloth(impl$77)、WallFlagCloth(impl$82)、LanternLightParams(impl$89)；drawable：DrawTopology、NaniSolidTexture、PrevTransform；他域：BrushPreview、Roof、CurvedHalfTimberInstanceData、ColorId、utils Transform、CritterLocomotion |

- 与 reconstruction 全组件 `#[derive(Clone,…)]` 标注**不符**：二进制证据仅上述 17 个组件有 Custom clone handler，其余（SemiTransparent、FlatLighting、FollowMouse 等）clone_behavior=Default → 原代码未 derive Clone（差异登记，不改源码）。

### 3. impl$N → 组件名映射（round 新增，public-symbols PDB 权威）

impl$2=FollowMouse、impl$3=TestCube、impl$15=TreeComponent、impl$16=ConvertedPreplacedTree、impl$17=TreeAnimation、impl$18=TreeMoveBend、impl$19=TreeMarkedForDeletion、impl$20=StairsStep、impl$21=ArrowGizmo、impl$22=SnappingToEndPreview、impl$23=SnappingToSegmentPreview、impl$24=DashedLineLength、impl$25=DontReceiveContactShadows、impl$26=SemiTransparent、impl$27=FlatLighting、impl$28=NormalMap、impl$29=OverwriteRoughness、impl$30=ClothlineCloth、impl$34=GothicWindowBrickDeformation、impl$38=BrickColor、impl$42=Ui2DFollowMouse、impl$43=UiInactive、impl$44=Ui2dExtent、impl$46=Ui2dWsExtent、impl$48=Ui2dImage、impl$49=UiInteractionState、impl$50=UiMouseInteractable、impl$52=UiDrawOrder、impl$56=UiNav、impl$58=HistoryButton、impl$64=MeshFollowTerrain、impl$65=KeepOnLevelUnload、impl$66=BackgroundTerrain、impl$67=WoodenGate、impl$68=BalconyDoor、impl$74=Smoke、impl$77=RoofFlagCloth、impl$82=WallFlagCloth、impl$87=SplashParticle、impl$88=TreeAmbience、impl$89=LanternLightParams、impl$91=AutoClutterConvertableToDecoClutter、impl$93=AutoClutter、impl$96=SittableForBirds。

### 4. 错位/纠错

- **计数纠错（round3）**：本章头旧写"45 个 ECS 组件类型"→ 实为 **44**。核对口径：regmap 扫描 mod.rs register_required_components 恰 44 个（0x140844f20…0x1408579f0），加 drawable 12 + transient 1 = **57**；业务函数 54 = 44 register + 9 clone_behavior + 1 serialize（与"函数总计 76：业务 54"一致）。"45" 为 off-by-one，正文 44 行表为准。
- 旧行"18KB 哈希/类型表 + 每 impl 独立组件类型名表"表述修正：函数体含自身描述符静态（名字 &str 指向类型名表，如 FollowMouse 名串 @0x142c37745，len 0x25）与 `HashMap<TypeId,ComponentId>` 控制组常量；**无 required 类型表**（本轮 TypeId 扫描证实）。
- decomp 证据文件 decomp-0x140002370.txt / decomp-0x14084cc00.txt 为 Ghidra NotFound 错误文件（小函数未建），结论来自机器码手解（31c0c3 / lea+mov eax,2），证据列已改为机器码地址。
