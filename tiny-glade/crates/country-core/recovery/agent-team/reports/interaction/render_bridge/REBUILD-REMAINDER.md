# REBUILD-REMAINDER — render_loop 未判等 3 批逐函数判等（build_draw_lists 闭包 / nani update_* / ice_surface compute）

verifier: impl-renderloop-remainder | 日期: 2026-10-17 | 协议: planning/2026-10-16-logic-verify-protocol.md
方法: 全部 fresh 反编译（ghidra `--worker shared`，`get_functions(fields=signature,decompiled_code,callees)`，每批 ≤20）
对照 Rust body 逐分支/循环/调用序/绑定序。红线: 只以 fresh 反编译为准；GPU 宿主调用语义核验调用点。

## 批次 1: build_draw_lists 闭包（前 20，draw_callbacks_a）

| VA | closure | verdict | 关键核对 |
|---|---|---|---|
| 0x1409ae900 | Env6 | **判等** | 3 组件读→反照/alpha(缺省 0x4b2c…)/bool(法线)/法线(缺省 0xe410…)/磁盘 0xe6d1…→预设 0xf/5/0xe/0x13/9/10→draw；常量与序全对 |
| 0x1409af390 | Env27 | **判等** | `has`/`comp_opt` 5 组件→背景地形选键 0xbd39…/缺省 0x68a6…→bool/f32(缺省 0.8)/bool/bool 四 uniform 序→draw |
| 0x1409afcc0 | Env2(羊) | **判等** | 2 组件→i32/f32/bool/Mat4(from_srt 展开核对)→SheepAnimationSsbo→draw |
| 0x1409b0570 | Env48 | **判等** | 四磁盘 0x82c4…/0x3cc1…/0x376b…/0xa24b…→draw_instanced_w_gl_draw_flag |
| 0x1409b07f0 | Env52 | **判等** | `Events<ShowDemoBorderOverlay>` 两 len 空→discard；否则磁盘 0xbf45…→预设 9/10→draw |
| 0x1409b0b20 | Env57 | **判等** | 拓扑 tag1→`state&0xfc\|2`+载荷；Mesh 库 get→i32 索引数/f32 虚线长→draw |
| 0x1409b0fa0 | Env45 | **判等** | 五磁盘 0x7ba5…/0xa241…/0x7680…/0xe7d0…/0xbd39…→draw |
| 0x1409b12b0 | Env0(木板) | **判等** | 3 资源读(编辑高亮/光标 tag&1/擦除半径)→`state&0xf3\|4`→预设 9/10→WoodenPlank→磁盘 0x5b9a…→Halftimber→0x4a2b…→0x1b61…→bool/Vec3/f32→nani_draw |
| 0x1409b1ab0 | Env42 | **判等** | 仅 draw（bodies::plain_draw） |
| 0x1409b1b00 | Env37 | **判等** | LogoAnimation 资源→磁盘 0x4d8c…/0xf1fc…→f32×2→draw |
| 0x1409b1e90 | Env39 | **判等** | 仅 nani_draw(LanternEmissive=0x12) |
| 0x1409b1ee0 | Env19 | **判等** | SplashParticle 组件→f32 uniform→draw |
| 0x1409b2120 | Env32 | **判等** | 磁盘 0x5b9a…/0xa041…→nani_draw(BalconyDoor=0x16) |
| 0x1409b2290 | Env41 | **判等** | LoadingScreen 资源→`byte3&0x9f\|0x40`→磁盘 0x6b1c…→f32×2→draw |
| 0x1409b2580 | Env53 | **判等** | 同 Env48 四磁盘→draw_instanced_w_gl_draw_flag |
| 0x1409b2800 | Env58 | **判等** | panic_fmt "not yet implemented: TODO(vulkan)"（原文同） |
| 0x1409b28c0 | Env23 | **判等** | 2 组件+ActiveSession 会话字节循环(`<=1`选键 0x4c58…/缺省 0xe523…)→i32 色→三磁盘→i32 布→draw |
| 0x1409b2f40 | Env8 | **判等** | 预设 9/10/0xf→磁盘 0x177a…→预设 0x16→nani_draw(PreplacedFlower=0xe) |
| 0x1409b3220 | Env28 | **判等** | 仅 nani_draw(Uber=0x17) |
| 0x1409b3270 | Env16 | **判等** | `state&0xf3\|8`→0x4a2b…→Dormer×2→墙色→0x7db9…→Roof 色→i32×2→石/木地板→Halftimber→0x5b9a…→nani_draw(RoofFloor) |

## 批次 2: build_draw_lists 闭包（次 20，draw_callbacks_a/b）

| VA | closure | verdict | 关键核对 |
|---|---|---|---|
| 0x1409b3780 | Env11 | **判等** | ActiveSession 变体字节 3/4/其它 选键 0x34c2…/0x8535…/0xdb21…；磁盘+预设 9/10/0xf→BushSsbo→nani_draw |
| 0x1409b3be0 | Env20 | **判等** | 预设 0x13→draw |
| 0x1409b3cc0 | Env10 | **判等** | 预设 9/10/0xf→FlowerSsbo→nani_draw(Flower=0xb) |
| 0x1409b3eb0 | Env55 | **判等** | `byte3&0x9f\|0x40`→ArrowGizmo→f32×2→draw |
| 0x1409b4180 | Env33 | **判等** | 磁盘 0x806c…→nani_draw(Cloth=0x14) |
| 0x1409b4270 | Env17 | **判等** | `state&0xf3\|8`→Dormer×2→墙色→Roof 色→石/木地板→Halftimber→0x5b9a…→nani_draw(StairsFloor) |
| 0x1409b4540 | Env59 | **判等** | panic_fmt "TODO(vulkan)"（原文同） |
| 0x1409b4600 | Env40 | **判等** | 预设 0x13/5→磁盘 0x84a0…→预设 0x15→draw |
| 0x1409b4860 | Env22 | **判等** | 2 组件+会话字节循环→f32×3/bool/i32×2→三磁盘→draw |
| 0x1409b5140 | Env3(雨) | **判等** | 强度 ≤0(NaN 含)→discard；否则 AppMode/MainCamera 资源→f32/bool/bool→DofAutofocusSsbo→draw_instanced |
| 0x1409b55e0 | Env30 | **判等** | 预设 9/10→磁盘 0xc669…→nani_draw(SolidAlbedoMapEmissive) |
| 0x1409b57c0 | Env14 | **判等** | WoodenGate→磁盘 0x5b9a…→i32(q18)→GateColor→0x3c67…→0x3487…→i32(d30)→draw |
| 0x1409b5c70 | Env1 | **判等** | BoatSsbo→multi_draw_indexed_indirect |
| 0x1409b5cf0 | Env18 | **判等** | `contains::<SemiTransparent/FlatLighting>`×2→bool×2→draw |
| 0x1409b6100 | Env25 | **判等** | 仅 nani_draw(SolidVertexColor) |
| 0x1409b6150 | Env7 | **判等** | 磁盘 0x25df…→预设 0xf/5/0xe/0x13/9/0xa→Flower03Ssbo→multi_draw_indexed_indirect |
| 0x1409b6540 | Env13 | **判等** | i32(0x052e…,0)→Halftimber→0x5b9a…→预设 5/9/0xa→墙色→Roof 色→砖擦除高亮→nani_draw(GothicWindowBricks) |
| 0x1409b68e0 | Env5 | **判等** | 磁盘 0x5368…→MainCamera eye→Vec3→预设 0xf/5/0x13/0xe/0x15/0x10/9/0xa→组件→draw_instanced_from_placement_data |
| 0x1409b7110 | Env54 | **判等** | 磁盘 0xcf0e…→draw_instanced_w_gl_draw_flag |
| 0x1409b7200 | Env15 | **判等** | `state&0xf3\|8`→五磁盘 0x4a2b…/0x5b9a…/0xf1db…/0x1b61…/0x239b…→nani_draw(CurvedHalfTimber) |

## 批次 3: build_draw_lists 闭包（余 16，draw_callbacks_b + bodies）

| VA | closure | verdict | 关键核对 |
|---|---|---|---|
| 0x1409b7560 | Env34 | **判等** | 4 资源(WallHighlight/编辑高亮/光标/擦除半径)→`state&0xf3\|8`→磁盘 0xe410…/0x8d8a…→墙色→bool/i32/i32/Vec3/f32→nani_draw(Plaster) |
| 0x1409b7d40 | Env26 | **判等** | 预设 9/0xa→GardenStoneSsbo→multi_draw_indexed_indirect |
| 0x1409b7ec0 | Env44 | **判等** | 预设 0xf→nani_draw(Plant) |
| 0x1409b7fa0 | Env36 | **未判等(登记数据流偏差)** | 绑定序核对：Ssbo 库 FrozenMap 查名(0x2d bcmp)+断言→read_ssbo<SparkleSsbo>→draw_multi_instanced 全对上。**分歧点**: 原版每帧经 `sparse_instance_buffer::to_draw_multi_instanced<RoofTileSsbo>`(0x1412e4250) 现算 ranges；Rust 用预计算的 `SparkleDrawRangesView` 视图（`SparkleSsbo` 是 Gap 标签型、crate 无 `SparseSsbo` 实现）。需 resources/ssbo 层接线后才可判等，本批不冒充。 |
| 0x1409b8250 | Env51 | **判等** | 磁盘 0x10b5…→draw |
| 0x1409b8430 | Env60 | **判等** | `state&0xf3\|8`→磁盘 0xc207…→draw |
| 0x1409b8570 | Env46 | **判等** | draw_floor_shader(b,false)（floor 本体已判等） |
| 0x1409b8620 | Env43 | **判等(适配胶水)** | rocky_terrain_shader；顶点数取资源（原版在 shader 内读 World，适配器经视图传参） |
| 0x1409b8680 | Env21 | **判等** | 2 组件+会话字节循环→bool×2(恒 false)→四磁盘 0x61c0…/0x1c9b…/0x93b4…/0x92f0…→i32×2→0xb72f…→会话选键→draw |
| 0x1409b9030 | Env49 | **判等** | 仅 nani_draw(VelocityOnly=0x19) |
| 0x1409b9080 | Env38 | **判等** | LogoAnimation→四磁盘 0xd47a…/0x394d…/0xa9ff…/0x7862…→f32×2→draw |
| 0x1409b9550 | Env12 | **判等** | i32(0x052e…,0)→Halftimber→0x5b9a…→预设 5/9/0xa→墙色→Roof 色→nani_draw(SolidVertexColorBricks) |
| 0x1409b98e0 | Env29 | **判等** | `state&0xf3\|4`→磁盘 0x1b3a…/0x9447…→nani_draw(SolidVertexColorWindowGlass) |
| 0x1409b9aa0 | Env31 | **判等** | 磁盘 0x5b9a…/0xa041…→nani_draw(SolidVertexColorWindow) |
| 0x1409b9c10 | Env9 | **判等** | 预设 9/0xa→磁盘 0xc669…→nani_draw(SolidAlbedoMap) |
| 0x1409b9df0 | Env24 | **判等** | `state&0xf3\|4`→nani_draw(Smoke=0x15) |

## 批次 4: nani update_* 22 系（19 个 update + flush；maintain/solid_vertex_color 已判等）

共同结构（fresh 反编译每函数核对）：puffin scope（宏胶水）→ `GpuWorld::owned_subset_mut` → 按 owned 实例索引遍历 + 查 Query 表行 → 载荷内联 → `OwnedGpuWorldSubset::update_instance_data<T>`；行尾 `release_resource`×2 解锁。行视图模式与已判等的 maintain/solid_vertex_color 同构。

| VA | 函数 | 实例类型(反编译) | verdict | 备注 |
|---|---|---|---|---|
| 0x142071590 | nani_update_solid_vertex_color_window | SolidVertexColorWindowInstanceData | **判等** | |
| 0x142072010 | nani_update_solid_albedo_map | SolidAlbedoMapInstanceData | **判等** | |
| 0x142072b50 | nani_update_solid_albedo_map_emissive | SolidAlbedoMapEmissiveInstanceData | **判等** | |
| 0x142073690 | nani_update_preplaced_flower | PreplacedFlowerInstanceData | **判等** | 内联可见性逻辑与 `flower_visible` 逐句对上：`clamp01(x/130+0.5)*512`、rock 掩码位、水 `>0.01` 藏、四路径栅格和 `clamp01 >0.1` 藏；FrozenMap 取 TerrainRockMask(0x3f bcmp 断言) |
| 0x142074740 | nani_update_plant | PlantInstanceData | **判等** | |
| 0x142075070 | nani_update_solid_vertex_color_window_glass | SolidVertexColorWindowGlassInstanceData | **判等** | Query 行 `init` 的 6 元组(ActiveInstance/Transform/WindowLight/ColorId/Option<Semi>/Option<Albedo>)与 WindowGlassRow 对上；bindless 索引经 hash_one<GlobalId>+handle_to_bindless_index |
| 0x142076230 | nani_update_solid_vertex_color_bricks | SolidVertexColorBricksInstanceData | **判等** | pack_2x16f_uint（砖色打包） |
| 0x142076e40 | nani_update_gothic_window_bricks | GothicWindowBricksInstanceData | **判等** | pack_2x16f_uint |
| 0x142077d20 | nani_update_curved_halftimber | CurvedHalfTimberInstanceData | **判等** | `&mut` 直写 + hash_one<GlobalId> 色键查找 |
| 0x142078780 | nani_update_stone_floor | StoneFloorInstanceData | **判等** | get_tangent_at_u |
| 0x1420797b0 | nani_update_wren | WrenInstanceData | **判等** | |
| 0x14207a4f0 | nani_update_lantern_emissive | LanternEmissiveInstanceData | **判等** | pack_2x16f_uint |
| 0x14207b290 | nani_update_plaster | PlasterInstanceData | **判等** | get_changed_ticks_slice_for（Changed 过滤由调用方完成，已注记） |
| 0x14207c0a0 | nani_update_cloth | ClothInstanceData | **判等** | bindless 法线索引 |
| 0x14207cd60 | nani_update_smoke | SmokeInstanceData | **判等** | |
| 0x14207d770 | nani_update_balcony_door | BalconyDoorInstanceData | **判等** | |
| 0x14207e1f0 | nani_update_uber | UberInstanceData | **判等** | bindless 反照率 |
| 0x14207f060 | nani_update_velocity_only | VelocityOnlyInstanceData | **判等** | |
| 0x14207fcc0 | nani_update_stairs_floor | StairsFloorInstanceData | **判等** | get_changed_ticks_slice_for |
| 0x142080640 | nani_flush_instance_updates | gpu_flush<GpuInstance> | **判等** | 27 槽(0xc0 步长)仅 Owned 调 `gpu_flush`+数据缓冲 vtable+0x30；滚动 `[+0x1448]=[+0x1440];[+0x1440]=0` 与 `roll_instance_counts` 对上。前置 4B 暂存写未归类（GPU 宿主暂存，按红线核验调用点即可） |

## 批次 5: ice_surface_pass 4 个 compute pass 逐绑定（0x1409c08d0 fresh）

| pass | verdict | 绑定序（反编译 vs Rust） |
|---|---|---|
| Ice refl march | **判等** | read_rg×2(count/px)→tex 半深→tex TAA 史→u32 uniform→tex 重投影→磁盘 0x579f39ab91a01865(蓝噪)→tex 冰粗糙→预设 9/10/8(地形高/视觉地形高/地形光)→磁盘蓝噪→bind_ray_tracing→tex_write×2(hit/radiance)→write_rg×2(rt count/ray)→dispatch_indirect_raw |
| Ice refl trace | **判等** | read_rg×2(rt count/ray)→tex 半深→tex TAA 史→u32 uniform→tex 重投影→磁盘蓝噪→预设 9/10/8→磁盘蓝噪→bind_ray_tracing→tex_write×2→dispatch_indirect_raw（无 write_rg，与 Rust 同） |
| Ice refr resolve | **判等** | tex 命中→tex_view(radiance view: new+format+usage)→tex 深度→tex_write refr temporal current→render(dispatch_2d closure_env_0) |
| Ice refl resolve | **判等** | 预设 0(BRDF_FG_LUT)→tex 命中→tex_view radiance→tex 深度→磁盘蓝噪→tex 重投影→tex 冰粗糙→tex_write 颜色→u32 uniform(history_valid=两 temporal fresh 相与&1)→temporal refr previous→tex_write refr current→temporal ray_len previous→tex_write ray_len current→render(dispatch_2d) |

## 结论

- **判等**: env 闭包 **55/56**；nani update_* 22 系 **20/20**（fresh 核对 T 类型全部对上）；ice_surface 4 compute pass **4/4**。
- **未判等**: Env36（ranges 数据流经 `SparkleDrawRangesView` 视图、原版每帧 `SparseInstanceBuffer::to_draw_multi_instanced` 现算；`SparkleSsbo` 无 `SparseSsbo` 实现，需 resources/ssbo 层接线，本批不冒充、不实现）。
- **编译**: 本批未改任何 Rust（除 Env36 登记偏差外全部判等）。crate 全量 check 存在数处错误，全部位于 render_loop/nani 之外（audio/ui/save/rhapsody_contract/onboarding/ivy 等，他模块并发改动，错误集合随并发构建在变），render_loop/nani 路径无错误；本批不引入编译变化。
- 未发现分支条件反/缺分支/循环错/callee 错/自造近似（Env36 除外，已明确登记分歧点）。
