# render/nani/systems.rs

> 文件作用：nani ECS 系统族：nani_maintain_instances（删→建→改三阶段）+ 20 个 nani_update_*（组件→实例载荷打包 + update_instance_data）+ nani_flush_instance_updates（27 槽冲刷 + 计数滚动）。CPU 侧全部 O。

函数总计 30：业务 22 / 内联无独立符号 0 / 派生胶水 7

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::nani::systems::nani_update_curved_halftimber` | 0x142077d20 | 0 | CurvedHalfTimber 打包（kind 0x18，0x40 载荷，**&mut 组件直写**触发 tick）：选色 direct!=0→HalftimberColorIds[key]（越界 panic）/PublicWalls.roofs→Roof(byte@+0x40==2→u32@+0x44)/ids；写 data[3]（pad_0c）；无 cap 掩码；position=data[0..3]/sort=1.0 | O | B_n_sys §1/§4；§4 curved；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_stone_floor` | 0x142078780 | 0 | StoneFloor 打包（kind 9）：Roof(0x58B)/Floor(0x28B)/RoofAnimation(8B 缺省 1.0)；Roof 与 Floor 都缺 → unwrap；circle_tangent_u0（Circle2d::get_tangent_at_u(0.0) 0x140c97660）U=0 | O | B_n_sys §1/§4；§4 stone_floor；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_maintain_instances` | 0x14206f300 | 36 | 实例维护三阶段（**先删→后建→再改**）：RemovedComponents→Local map remove_entry（缺→unwrap，行 51:41）→remove_instance；新增 Query<Entity,&NaniInstanceDesc,&Transform> Without<NaniActiveInstance> → MeshAtlasLibrary get_by_name（Named 需 part.subset==kind，缺→panic 行 63:21 文案）→ extend_no_data（closure$1 0x141565950 收 12B {u32,u32,kind u8,[u8;3]}）→ Local HashMap insert + Commands Insert<NaniActiveInstance>；容量满 → warn_once("Instance limit reached…")；Changed<NaniInstanceDesc> → parts.len()==1 断言 → update_instance_mesh | O | B_n_sys §1/§4；§2 maintain；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_solid_vertex_color` | 0x142070ac0 | 119 | 纯色顶点色（kind 3，0x40）：通用 12 字仿射（Mat3::from_quat 各列×scale + 平移）；w12=SemiTransparent 在场、w13=FlatLighting 在场 | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_solid_vertex_color_window` | 0x142071590 | 152 | 窗纯色（kind 4）：w12=Semi、w13=ColorId.0 | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_solid_albedo_map` | 0x142072010 | 185 | 贴图（kind 0xf）：w12=[0,1,8,2,3,4,5,6,7][NaniSolidTexture byte]（表 0x142fd38c8） | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_solid_albedo_map_emissive` | 0x142072b50 | 223 | 发光贴图（kind 0x10，机器码与 albedo_map **同体**） | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_preplaced_flower` | 0x142073690 | 261 | 预置花（kind 0xe，载荷 **0x30**）：TerrainRockMaskSsbo 掩码（u/v=min(511,trunc(clamp01(c/130+0.5)*512))，位 1→隐藏）/水采样>0.01/4 路径栅格 clamp01 和>0.1 → 隐藏；隐藏载荷=单位旋转、scale 1、平移(0,-100,0)、sort_key 0 | O | B_n_sys §1/§4；§4 preplaced_flower；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_plant` | 0x142074740 | 313 | 植物（kind 0xa，0x30）：position/sort_key 覆写且保持 mesh_index（ECS 测试） | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_solid_vertex_color_window_glass` | 0x142075070 | 337 | 窗玻璃（kind 5）：ColorId|semi<<31（ColorId bit31→unwrap "value out of bounds for field WindowGlassFlags.color_id"）、窗光 f32、AlbedoMap 在场、bindless 下标（缺 name → unwrap） | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_solid_vertex_color_bricks` | 0x142076230 | 393 | 砖（kind 6）：pack_2x16f_uint(f2,f3)（0x140c90df0 = f16(a)|f16(b)<<16） | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_gothic_window_bricks` | 0x142076e40 | 425 | 哥特窗砖（kind 7，0x70 载荷） | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_wren` | 0x1420797b0 | 671 | 鹪鹩（kind 0x11，0x70 载荷） | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_lantern_emissive` | 0x14207a4f0 | 700 | 灯笼发光（kind 0x12） | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_plaster` | 0x14207b290 | 750 | 石膏（kind 0x13，0x30）：position=(0,0,0)/sort_key=1.0；断言 w.simplified_curve_2d.is_valid()（文案 O） | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_cloth` | 0x14207c0a0 | 800 | 布料（kind 0x14）：+ClothlineCloth + NormalMap + Res<AssetTextureLibrary> | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_smoke` | 0x14207cd60 | 835 | 烟（kind 0x15） | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_balcony_door` | 0x14207d770 | 868 | 阳台门（kind 0x16） | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_uber` | 0x14207e1f0 | 903 | Uber（kind 0x17）：flags = albedo*1 + semi*2 + 4；+Option<SemiTransparent> + Option<AlbedoMap> + Res<AssetTextureLibrary> | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_velocity_only` | 0x14207f060 | 954 | 速度（kind 0x19，0x60）：当前 + 上一帧（PrevTransform）两个仿射；position/sort 取当前 Transform | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_update_stairs_floor` | 0x14207fcc0 | 979 | 楼梯地板（kind 0x1a，0x30）：9 字拷自 StairsFloorMeshChunk+8 + 3 零字；position=0/sort=1.0 | O | B_n_sys §1/§4；wp3/contracts/systems.md §3；src/nani/systems.rs |
| `country_core::render::nani::systems::nani_flush_instance_updates` | 0x142080640 | 1011 | 冲刷（0x142080640，1053B）：27 槽（stride 0xc0），仅 Owned 调 ShadowedBuffer<GpuInstance>::gpu_flush(device) + 数据缓冲 vtable+0x30；末尾 +0x1448=+0x1440; +0x1440=0；参数顺序（Res<GraphicsDeviceResource> 位置）为 I | O | B_n_sys §1/§4；§5；wp3/contracts/systems.md §3；src/nani/systems.rs |

## 派生胶水（7 项：closure、default 等）

7 项 glue = impl$0::default（NaniActiveInstanceCollection Local 缺省）+ maintain 的 closure$0/$1（0x141565950 = extend_no_data 回调，把 12B 记录存入 Option，O）+ curved_halftimber/stone_floor 的 4 个打包闭包。

## 证据注记（tg_rn1）
- **kind ↔ 系统映射**（O，B_n_sys §1/§4）：SolidVertexColor=3、Window=4、Glass=5、Bricks=6、GothicWindowBricks=7、StoneFloor=9、Plant=0xa、PreplacedFlower=0xe、SolidAlbedoMap=0xf、Emissive=0x10、Wren=0x11、LanternEmissive=0x12、Plaster=0x13、Cloth=0x14、Smoke=0x15、BalconyDoor=0x16、Uber=0x17、CurvedHalfTimber=0x18、VelocityOnly=0x19、StairsFloor=0x1a。
- **ECS 参数面**（O，PDB FunctionSystem 单态化；全表见 wp3/contracts/systems.md §3）：maintain = Query<(Entity,&NaniInstanceDesc,&Transform), Without<NaniActiveInstance>> + Changed 查询 + RemovedComponents + Res<MeshAtlasLibrary> + ResMut<GpuWorld> + Local<NaniActiveInstanceCollection> + Commands；update 族 = Query<(&NaniActiveInstance, &Transform[, 组件]), With<InstanceDataMarker<T>>> + Res<GpuWorld>；curved_halftimber 是唯一 `&mut InstanceData` 直写（+Res<PublicWalls>+Res<HalftimberColorIds>+Query<&Roof>）；plaster/stairs_floor 用 Or<(Changed<source>, Changed<NaniActiveInstance>)>。
- **maintain 关键 O**：顺序先删后建再改；NaniInstanceDesc 24B {mesh: bit0 判别 AtlasPart(u32@+4)/Named(NameHash u64@+8), subset_kind u8@+0x10}；Named 经 get_by_name 线性找 part.subset==kind，缺 → panic（行 63:21 文案 "NaniInstanceDesc requested mesh {} and shader {:?},…"）；紧凑 key 16B = {Vec3, f16, u16 mesh_part}（旧文档 {chunk_id,u64…} 错）；flush 27 槽（stride 0xc0）。
- **update_instance_data 签名**（O，0x14156fff0 系）：`(subset, key: &mut{u32,u32}, position: &Vec3, sort_key: f32, data: &T)`；fast-path compact 命中否则 HashMap 回写 key.index。
- **U**：gap 组件（BrickColor/NaniSolidTexture/WindowLight/AlbedoMap/PrevTransform/Wren/GothicWindowBrickDeformation/CurvedHalfTimber/Roof/RoofAnimation/Floor/LanternLightParams/Plaster/ClothlineCloth/NormalMap/Smoke/StairsFloorMeshChunk）经 gap_words 读 0，注释含机器码偏移；terrain_rock_mask_words（有行时未注册 → 原版 unwrap panic，B_x_out 已补访问器）；circle_tangent_u0=0；animated_peel_lookup=None；window_light_value=0.0；`nani_flush_instance_updates` 第二参数类型（I：Res<GraphicsDeviceResource>）；External subset 的 draw_count。
