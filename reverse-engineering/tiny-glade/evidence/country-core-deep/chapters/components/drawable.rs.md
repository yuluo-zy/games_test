# components/drawable.rs

> 文件作用：**nani 可绘制组件族：`NaniInstanceMeshDesc`（命名/图集 part 选择）、`NaniActiveInstance`(=InstanceKey)、`NaniSolidTexture`/`PrevTransform`/`AlbedoMap`/`AlphaMap`、`DrawableMeshBundle`。业务面 = 组件注册 + `cache_prev_object_transforms` 系统 + `From<MeshAtlasPart>`。**

函数总计 44：业务 19 / 内联无独立符号 5 / 派生胶水 20

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::components::drawable::impl$8::clone_behavior` | 0x14082cac0 | 18 | `clone_behavior` = `lea rdx,[组件类型表]; mov eax,2; ret`（返回行为=2，克隆行为枚举变体；每 impl 独立类型表）。Bevy required-components 胶水。 | **O** | decomp-0x14082cac0.txt |
| `country_core::components::drawable::impl$8::register_required_components` | 0x14082c3d0 | 18 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x14082c3d0.txt |
| `country_core::components::drawable::impl$9::register_required_components` | 0x14082cad0 | 25 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x14082cad0.txt |
| `country_core::components::drawable::impl$1::from` | 0x14082bd60 | 49 | `NaniInstanceMeshDesc::from<MeshAtlasPart>`（机器码 `mov [rcx+4],eax; mov dword[rcx],1` → 判别=1 + 值=0 = `AtlasPart(0)`）。与 reconstruction 一致。 | **O** | decomp-0x14082bd60.txt |
| `country_core::components::drawable::impl$12::register_required_components` | 0x14082d1c0 | 54 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x14082d1c0.txt |
| `country_core::components::drawable::impl$16::register_required_components` | 0x14082d8b0 | 71 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x14082d8b0.txt |
| `country_core::components::drawable::impl$22::register_required_components` | 0x14082dfa0 | 75 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x14082dfa0.txt |
| `country_core::components::drawable::impl$23::clone_behavior` | 0x140002370 | 79 | 共享 `clone_behavior` 0x140002370 = `xor eax,eax; ret`（返回 0 = 无克隆行为）。Bevy 默认胶水。 | **O** | decomp-0x140002370.txt |
| `country_core::components::drawable::impl$23::register_required_components` | 0x14082e690 | 79 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x14082e690.txt |
| `country_core::components::drawable::impl$24::register_required_components` | 0x14082ed80 | 83 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x14082ed80.txt |
| `country_core::components::drawable::impl$25::register_required_components` | 0x14082f470 | 89 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x14082f470.txt |
| `country_core::components::drawable::impl$26::register_required_components` | 0x14082fb60 | 93 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x14082fb60.txt |
| `country_core::components::drawable::impl$27::register_required_components` | 0x140830250 | 99 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x140830250.txt |
| `country_core::components::drawable::impl$28::clone_behavior` | 0x140831030 | 105 | `clone_behavior` = `lea rdx,[组件类型表]; mov eax,2; ret`（返回行为=2，克隆行为枚举变体；每 impl 独立类型表）。Bevy required-components 胶水。 | **O** | decomp-0x140831030.txt |
| `country_core::components::drawable::impl$28::register_required_components` | 0x140830940 | 105 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x140830940.txt |
| `country_core::components::drawable::impl$36::clone_behavior` | 0x140831730 | 119 | `clone_behavior` = `lea rdx,[组件类型表]; mov eax,2; ret`（返回行为=2，克隆行为枚举变体；每 impl 独立类型表）。Bevy required-components 胶水。 | **O** | decomp-0x140831730.txt |
| `country_core::components::drawable::impl$36::register_required_components` | 0x140831040 | 119 | Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。 | **O** | decomp-0x140831040.txt |
| `country_core::components::drawable::impl$2::build` | 0x14082bd70 | 125 | `impl$2::build` = `mov rcx,rdx; jmp` 尾调用共享 Bundle build（DrawableMeshBundle 构造委托）。 | **O** | decomp-0x14082bd70.txt |
| `country_core::components::drawable::cache_prev_object_transforms` | 0x14082bd80 | 130 | **业务系统** `cache_prev_object_transforms`：逐帧遍历 sparse instance buffer（+0xf8/+0x100/+0x108 字段、0xa0/0x48/0x30 行 stride），把实例当前变换拷贝写入 `PrevTransform`（velocity_only/wren 消费）。 | **O** | decomp-0x14082bd80.txt |

## 内联/无独立符号函数

- `country_core::components::drawable::impl$3::component_ids`（行 11）— **I**：`impl$3/impl$5` = Bundle 派生 `component_ids`/`get_components`（DrawableMeshBundle/含 KeepOnLevelUnload/DesignatedRenderPass 变体）。
- `country_core::components::drawable::impl$5::get_components`（行 11）— **I**：`impl$3/impl$5` = Bundle 派生 `component_ids`/`get_components`（DrawableMeshBundle/含 KeepOnLevelUnload/DesignatedRenderPass 变体）。
- `country_core::components::drawable::impl$0::from`（行 43）— **I**：`impl$0::from` = `NaniInstanceMeshDesc::from` 另一变体（内联）。
- `country_core::components::drawable::impl$13::component_ids`（行 61）— **I**：`impl$13::component_ids`/`impl$15::get_components` = 另一 Bundle 变体的派生胶水（同 line 11 族）。
- `country_core::components::drawable::impl$15::get_components`（行 61）— **I**：`impl$13::component_ids`/`impl$15::get_components` = 另一 Bundle 变体的派生胶水（同 line 11 族）。

## 派生胶水

- 20 项：BalconyDoorInstanceData> > >, ClothInstanceData> > >, DesignatedRenderPass> > >, DrawableMeshBundle> >, GothicWindowBricksInstanceData> > >, LanternEmissiveInstanceData> > >, PlantInstanceData> > >, PreplacedFlowerInstanceData> > >, PrevTransform> > >, SmokeInstanceData> > >, SolidVertexColorBricksInstanceData> > >, SolidVertexColorInstanceData> > > 等（derive 直接复用，不计业务）。
