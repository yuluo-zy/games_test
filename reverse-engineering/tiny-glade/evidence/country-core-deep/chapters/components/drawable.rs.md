# components/drawable.rs

> 文件作用：**nani 可绘制组件族：`NaniInstanceMeshDesc`（命名/图集 part 选择）、`NaniActiveInstance`(=InstanceKey)、`NaniSolidTexture`/`PrevTransform`/`AlbedoMap`/`AlphaMap`、`DrawableMeshBundle`。业务面 = 组件注册 + `cache_prev_object_transforms` 系统 + `From<MeshAtlasPart>`。**

函数总计 44：业务 19 / 内联无独立符号 5 / 派生胶水 20

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::components::drawable::impl$8::clone_behavior` | 0x14082cac0 | 18 | 胶水：策略归类 — `lea rdx,[组件类型表]; mov eax,2; ret` = `ComponentCloneBehavior::Custom`。组件名=**DrawTopology**（PDB `enum2$<DrawTopology>`——枚举 2 变体，非零尺寸 marker）。 | **O** | 机器码 + public-symbols |
| `country_core::components::drawable::impl$8::register_required_components` | 0x14082c3d0 | 18 | 胶水：策略归类 — Bevy register 单态化（仅注册自身，无 required；TypeId 0x541721773055ec85/0xb04edb2d6216372f 命中 `enum2$<DrawTopology>`）。 | **O** | TypeId 扫描 + public-symbols |
| `country_core::components::drawable::impl$9::register_required_components` | 0x14082cad0 | 25 | 胶水：策略归类 — 同上（组件名=NaniInstanceDesc）。 | **O** | TypeId 扫描 + public-symbols |
| `country_core::components::drawable::impl$1::from` | 0x14082bd60 | 49 | **业务** `NaniInstanceMeshDesc::from<MeshAtlasPart>`：机器码 `48 89 c8 / 89 51 04 / c7 01 01 00 00 00 / c3` = `mov rax,rcx; mov [rcx+4],edx; mov dword[rcx],1; ret` → **判别=1（AtlasPart）+ 值=edx（part 索引原样保留）**。修正旧行"值=0"；reconstruction 桩 `AtlasPart(0)` 与证据不符（part 被保留）。 | **O** | 机器码 0x14082bd60（Ghidra NotFound 手解） |
| `country_core::components::drawable::impl$12::register_required_components` | 0x14082d1c0 | 54 | 胶水：策略归类 — 同上（组件名=NaniActiveInstance）。 | **O** | TypeId 扫描 + public-symbols |
| `country_core::components::drawable::impl$16::register_required_components` | 0x14082d8b0 | 71 | 胶水：策略归类 — 同上（组件名=DesignatedRenderPass）。 | **O** | TypeId 扫描 + public-symbols |
| `country_core::components::drawable::impl$22::register_required_components` | 0x14082dfa0 | 75 | 胶水：策略归类 — 同上（组件名=UiPass）。 | **O** | TypeId 扫描 + public-symbols |
| `country_core::components::drawable::impl$23::clone_behavior` | 0x140002370 | 79 | 胶水：策略归类 — 共享 `clone_behavior` = `xor eax,eax; ret` = `Default`（组件名=WireframePass，非 Clone）。 | **O** | 机器码 0x140002370（31c0c3） |
| `country_core::components::drawable::impl$23::register_required_components` | 0x14082e690 | 79 | 胶水：策略归类 — 同上（组件名=WireframePass）。 | **O** | TypeId 扫描 + public-symbols |
| `country_core::components::drawable::impl$24::register_required_components` | 0x14082ed80 | 83 | 胶水：策略归类 — 同上（组件名=DontCastShadow）。 | **O** | TypeId 扫描 + public-symbols |
| `country_core::components::drawable::impl$25::register_required_components` | 0x14082f470 | 89 | 胶水：策略归类 — 同上（组件名=OcclusionCulled）。 | **O** | TypeId 扫描 + public-symbols |
| `country_core::components::drawable::impl$26::register_required_components` | 0x14082fb60 | 93 | 胶水：策略归类 — 同上（组件名=AlbedoMap）。 | **O** | TypeId 扫描 + public-symbols |
| `country_core::components::drawable::impl$27::register_required_components` | 0x140830250 | 99 | 胶水：策略归类 — 同上（组件名=AlphaMap）。 | **O** | TypeId 扫描 + public-symbols |
| `country_core::components::drawable::impl$28::clone_behavior` | 0x140831030 | 105 | 胶水：策略归类 — `Custom`（组件名=NaniSolidTexture，derive Clone）。 | **O** | 机器码 + public-symbols |
| `country_core::components::drawable::impl$28::register_required_components` | 0x140830940 | 105 | 胶水：策略归类 — 同上（组件名=NaniSolidTexture）。 | **O** | TypeId 扫描 + public-symbols |
| `country_core::components::drawable::impl$36::clone_behavior` | 0x140831730 | 119 | 胶水：策略归类 — `Custom`（组件名=PrevTransform，derive Clone）。 | **O** | 机器码 + public-symbols |
| `country_core::components::drawable::impl$36::register_required_components` | 0x140831040 | 119 | 胶水：策略归类 — 同上（组件名=PrevTransform）。 | **O** | TypeId 扫描 + public-symbols |
| `country_core::components::drawable::impl$2::build` | 0x14082bd70 | 125 | `impl$2::build` = `mov rcx,rdx; jmp 0x140b6633f`（e9 c8a53300 尾调用）→ 共享 Bundle build（DrawableMeshBundle 构造委托；0x140b6633f 无 PDB 符号——私有/内联函数，未独立建表）。 | **O** | 机器码 0x14082bd70 |
| `country_core::components::drawable::cache_prev_object_transforms` | 0x14082bd80 | 130 | **业务系统** 见下方算法节。 | **O** | decomp-0x14082bd80.txt |

## 内联/无独立符号函数

- `country_core::components::drawable::impl$3::component_ids`（行 11）— **I**：`impl$3/impl$5` = Bundle 派生 `component_ids`/`get_components`（DrawableMeshBundle/含 KeepOnLevelUnload/DesignatedRenderPass 变体）。
- `country_core::components::drawable::impl$5::get_components`（行 11）— **I**：同上。
- `country_core::components::drawable::impl$0::from`（行 43）— **I**：`NaniInstanceMeshDesc::from` 另一变体（内联；`From<&str>`/命名路径，maintain `get_by_name<NameHash>` 对应 Named(u64) 分支）。
- `country_core::components::drawable::impl$13::component_ids`（行 61）— **I**：另一 Bundle 变体的派生胶水（同 line 11 族）。
- `country_core::components::drawable::impl$15::get_components`（行 61）— **I**：同上。

## 派生胶水

- 20 项：BalconyDoorInstanceData> > >, ClothInstanceData> > >, DesignatedRenderPass> > >, DrawableMeshBundle> >, GothicWindowBricksInstanceData> > >, LanternEmissiveInstanceData> > >, PlantInstanceData> > >, PreplacedFlowerInstanceData> > >, PrevTransform> > >, SmokeInstanceData> > >, SolidVertexColorBricksInstanceData> > >, SolidVertexColorInstanceData> > > 等（derive 直接复用，不计业务）。

## 算法级结论（round 算法深挖，2026-10-15）

### 1. `cache_prev_object_transforms`（0x14082bd80）全算法

**实现方式**（decomp-0x14082bd80.txt，无任何外部调用——纯内联指针运算）：

- 参数面（推断）：`param_1` = 系统闭包环境：`[0]`=sparse instance buffer/world A、`[1]`=实例表 world B、`+0x14`=u32 kind（每实例写入值）。world A 布局：`+0xf8`=chunk id 数组 ptr、`+0x100`=数组尾、`+0x108`=布局标志字节、`+0x110`/`+0x118`=两个帧/槽索引。world B 布局：`+0x100`=chunk 描述符表（0xa0 stride）、`+0x1a8`=0x48-stride 记录表指针。
- **两布局分支**（`+0x108` 标志）：
  - 标志==0：按 chunk id 迭代（A+0xf8..+0x100，**跳过 desc+0x58 == 0** 的空 chunk）；实例数 = desc+0x94；每实例槽索引取自 chunk 专属 per-instance 描述符表（`(desc+0x50)+8 + idx*0x10` 的 u32，stride 0x10）；经记录表（0x48 stride，rec+0x18=实例数组 base、rec+0x38=指针数组、rec+0x40=len）定位源/目标槽。
  - 标志≠0：直接经记录表（0x48 stride，rec+0x10=实例数、rec+0x38=指针数组、rec+0x18=实例数组 base）；每实例槽索引 = 循环计数（无描述符表）。
- **槽定位**：源/目标槽都用**取补索引**（`~slot * 0x30` 负偏移 = base - (slot+1)*0x30，槽从数组尾向前分配）；源入口 = 指针数组 `[(A+0x110)]`，目标入口 = 指针数组 `[(A+0x118)]`（当前帧/上一帧索引）；`帧 >= rec.len` 或入口==0 → 源=0（空槽）。
- **拷贝**：每实例 0x30=48B（Affine3A 变换：3×16B 移动）从源槽拷到目标槽，并把 `param_1+0x14` 的 u32 kind 写入**目标槽**的 kind 指针表（`base+0x28 + ~target*0x30`，每实例偏移 idx*4）。
- **纠错**：旧文"跳过 desc[0xa0] 首字段==0"——实际空 chunk 判定为 **desc+0x58 == 0**（`*(*(B+0x100)+0x58 + id*0xa0)` 判零），非 desc+0x00。
- **语义**（I，与函数名一致）：每帧把实例当前变换写入 PrevTransform 槽（velocity_only / wren 消费），u32 = 实例 kind/subset 标记。

### 2. `NaniInstanceMeshDesc::from<MeshAtlasPart>`（0x14082bd60）

- 8B 函数：`mov rax,rcx; mov [rcx+4],edx; mov dword[rcx],1; ret` → 输出 {disc=1=AtlasPart, value=part}——**part 原样保留**。
- **修正**：旧行写"值=0 = AtlasPart(0)"；reconstruction `fn from(_part)` 桩返回 `AtlasPart(0)` 与机器码不符（应返回 `AtlasPart(part)`）。登记差异，不改源码。

### 3. register 族策略归类

- 12 个 register_required_components 全部**无 required 组件**（TypeId movabs 仅自身一对；与 mod.rs 44 个同结论）。
- impl$N 映射：impl$8=DrawTopology（**枚举 enum2$**）、impl$9=NaniInstanceDesc、impl$12=NaniActiveInstance、impl$16=DesignatedRenderPass、impl$22=UiPass、impl$23=WireframePass、impl$24=DontCastShadow、impl$25=OcclusionCulled、impl$26=AlbedoMap、impl$27=AlphaMap、impl$28=NaniSolidTexture、impl$36=PrevTransform。
- clone_behavior：DrawTopology/NaniSolidTexture/PrevTransform=Custom（Clone）；WireframePass=Default（共享 0x140002370）。
- 旧"impl$23 clone_behavior 共享 0x140002370"与"impl$8 行 18"映射确认：行 18=DrawTopology、行 79=WireframePass。
