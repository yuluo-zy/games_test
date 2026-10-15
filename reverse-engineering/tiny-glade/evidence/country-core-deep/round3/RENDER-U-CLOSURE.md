# RENDER-U-CLOSURE：渲染逻辑 U 项闭合记录（round3，2026-10-15）

任务：闭合 `evidence/country-core-audit/RENDER_GAP_ANALYSIS.md` §5.2（渲染逻辑 U 项）与 §5.4（公共 API 缺口）。
标注：**O** = 机器码/反编译/哈希复算证实；**U** = 卡点（注明已排除的候选与证据边界）。
dbid `tg_rev2`（新反编译落盘 `round3/decomp-*.txt`）；RIP 相对引用扫描工具 `tools/country-core-audit/rip_scan.py`、
指针写入者检查 `check_pointer_writers.py`（本轮新增，随证据提交）。

---

## 1. 全局 `*0x14362c0d8` / `*0x14362c098`（B_s_h 帧外层条件分支）—— **O（闭合）**

两全局同为 `.data` **指针槽 → 零填充区字节标志**（同 CENTERED_PATH 形制；.data 初值 = 指针，目标字节在 rawsize 之外，加载时 0）：

| 指针槽 | 目标字节 | 语义 | 写入者（O） | render_world 用法（O） | 其他读取 |
|---|---|---|---|---|---|
| `*0x14362c098`（.data @0x3629898） | `0x143634d90` | **demo 开关**（进程级，gui/照片模式/纹理加载/装饰/水/生物/工具栏等 66 处 RIP 相对读取；`lut_gallery`/`vermeer`/`load_textures::should_load`/`Toolbar::new`/`image_formation::default` 直读 movzx） | **`tiny_glade::main` 0x1403dcf78**：`movzx eax,[rbp+0x376]; mov [rcx],al`——值来自零化/None 哨兵结构字节 6 = **0**（本 build 恒写 0） | `0x1404eca6c`：`==0` 时执行 **`refraction_copy_pass`（0x1409bf320）+ create_raw_resource("refraction source"，0x1429ec028，len 0x11，format 0x7a) + `water_surface_pass`(0x1409bf800)/`ice_surface_pass`(0x1409c08d0，is_winter 选变体)`**；非 0 → 整段跳过（demo 下不渲染水面折射） | 66 处（gui_escape_menu、key_bindings、settings、load_decorators、spawn_ducks、draw_water、ui_place_tree、show_color_menu、new_session/welcome/load UI、setup_photomode_toolbar、demo_stuff、RenderGraphState::compile 等） |
| `*0x14362c0d8`（.data @0x36298d8） | `0x143634da0` | **`freeze_occlusion_culling` 调试开关**（O：grass 把该字节作为 uniform 值写入 `fnv1a("freeze_occlusion_culling")=0x6d7707561ec0da2b`，键值逐位吻合） | **绑定方 = `do_gui_debug` closure_env$5（0x1400b68e6）**：字节指针存入 UI 结构（+0x178）传入 `egui::widgets::button::ui`（0x140c48930）——调试窗口按钮，点击处理为唯一写入路径；无其他写入 | `0x1404ec5b1`：`==0` 时执行 **`generate_wall_draw_lists_pass2`（0x14098b2a0）+ `gpu_generate_draw_lists`（0x140987dd0）**；非 0 → 跳过（冻结遮挡剔除 → 不重生成画列表） | `grass_compute` 0x140a60575 直读（movzx r9d → uniform 值）；do_gui_debug 按钮 |

要点：
- 直读/经指针读混用：`0x143634d90` 与 `0x143634da0` 均有直读 movzx 点（vermeer_render_from_target、image_formation、load_textures、Toolbar::new、grass），也经指针槽读（render_world、gui 族）。**全部 .text 引用已穷举**（RIP 相对 8 种 ModRM 形 + 绝对 `[disp32]` SIB 形扫描，均无遗漏）。
- `0x143634d90` 无其他写 1 的点（main 写 0、UI 只读），本 build 恒 0 → demo 分支恒不启用；`0x143634da0` 经调试按钮可运行时置 1。
- 纠错：render-gap §3.4 的 `freeze_occlusion_culling` 标签经 uniform 名哈希**证实**；"demo" 标签维持（写 0 + 全游戏读取 + 照片模式/菜单门控语义吻合）。

## 2. CENTERED_PATH `*0x14362c308`（B_x_out）—— **O（写入者找到，闭合）**

- 指针槽 `.data @0x3629b08` = `0x143638fac`（.bss 类零填充，初值 0）；6 个读取点（float_grid 三实例 + TerrainHeightsData 两实例 + **load_game**）。
- **写入者 = `country_core::systems::save::load_game::load_game`（0x14146dc82-0x14092）**（O，rip_scan + 反汇编）：
  ```
  14146dc82: mov rcx,[rbp+0x948]         ; 配置结构
  14146dc88: mov eax,[rcx+0xc0]          ; 字段 @+0xc0
  14146dc8b: mov rax,[rip] # 0x14362c308 ; 取 CENTERED_PATH 指针
  14146dc92: setge byte ptr [rax]        ; *0x143638fac = (字段 >= 0x1a) ? 1 : 0
  14146dc95: setg  byte ptr [rip] # 0x14362c011 ; *0x14362c011 = (字段 > 0x1a)（楼梯校验开关同源共写）
  ```
- **纠错（render-gap §3.3 结论反转）**：render-gap 判"写入者为零、标志恒 0、恒走向量路径"——其扫描只查目标字节地址（0x143638fac）的绝对引用，**漏了经指针槽的写**（`setge [rax]`）。实际**存在运行期写入者**：load_game 按配置字段 `>= 0x1a` 置 0/1。`float_grid` 的路径选择是**按存档加载结果**的，不是恒向量路径；字段 `+0xc0` 的语义 U（0x1a=26 的含义未知，候选：存档版本/世界尺寸类参数，I）。共享 utils 的 `sampling_fix: bool` 参数应由 load_game 调用点按该字节传入（接线层）。
- `0x14362c011`（楼梯校验开关，多域已确认）与 CENTERED_PATH 在同一 `cmp/setcc` 块内共写——同一配置字段驱动两个全局（跨域线索）。

## 3. 3 个只有 TypeId 的组件（B_d_cb_a）—— **O（真名补全）**

Bundle `component_ids<KeepOnLevelUnload, DrawableMeshBundle, BackgroundTerrain, DontReceiveContactShadows, OverwriteRoughness>`（0x1416dcd10，bevy_ecs::bundle::impl$24）机器码处理序 = 声明序（O）：
首元素 `register_component<KeepOnLevelUnload>`（0x1416e2ac0，全名证实）→ 次元素 `DrawableMeshBundle::component_ids`（0x140b59730，`drawable::impl$3`）→ 三/四/五元素各经 `hashbrown 查找 → HashMap::insert(lo,hi) → register_required_components` 块。三块常量序 = `[0x66476a13…]`（@0x1416dcda7/dcdb1）→ `[0xa6c0fc01…]`（@0x1416dd201/dd20b）→ `[0x192600af…]`（@0x1416dd6e2/dd6ec），后续 call 分别为 **impl$66 / impl$25 / impl$29 的 register_required_components**（0x140852d60 / 0x14084a260 / 0x14084be20）。

| TypeId（128 位，lo/hi O） | 类型（O） | impl | 语义侧证 |
|---|---|---|---|
| lo `0x66476a13bbcc5e41` / hi `0xba435ad0d8465a44` | **`components::BackgroundTerrain`** | impl$66（0x140852d60） | env27 `contains` 存在标记（B_d_cb_a §2 行 8） |
| lo `0xa6c0fc01ec67ef4c` / hi `0x7a8927565f75e3ef` | **`components::DontReceiveContactShadows`** | impl$25（0x14084a260） | env27 `Option` → `bool(!has)`（存在性） |
| lo `0x192600af2434483b` / hi `0x2988839ec6eae656` | **`components::OverwriteRoughness`** | impl$29（0x14084be20） | env27 f32 缺省 0.8（0x3f4ccccd）——粗糙度覆盖值 |

- 反例排除：`register_component<KeepOnLevelUnload>` 全名（0x1416e2ac0）与 `drawable::impl$3::component_ids`（0x140b59730）证实**前两个 bundle 元素不是**这三个 TypeId。
- B_d_cb_a 候选名全部命中；stub 名 `MarkerU66476a13`/`MarkerUa6c0fc01`/`FloatU192600af` 可替换为真名。

## 4. shader_refactoring 状态机 disc 6..=9（B_c3）—— **O（真实行为解出；维持"不可达"结论）**

两跳转表目标逐条复算（0x142ab9ff0 / 0x142aba004，表重叠：table2 = table1+0x14）：

- **disc 6 → 0x14099061b**（指令对齐起点，= 第二表 idx2 更新处理器 0x140990613 的 8 字节前缀之后）：`payload<4 → {disc=2,payload+1}，否则 {disc=3,payload=0}` + 写 tick + 返回——但作为第一表入口时 `ecx/r8d` 非 payload（遗留表基/状态指针低 32 位），且 tick 槽 [rbp+0x3c8/3d0/3dc] 未初始化 → **UB**。
- **disc 7 → 0x1409905ff**（= `movdqa xmm1,[rbp+0x20]` **指令中部**；第二表 idx3/4 处理器 0x1409905e6 的 `payload+1` + 36 B feedback desc → 帧句柄 +0x0..0x24 拷贝 + tick 的块本为合法，但第一表入口落在中间）→ 崩溃/UB。
- **disc 8/9 → 0x1409905d2** = `mov edx,0x2` 指令**中部**（`add [rax],al` 起）→ 崩溃；**disc 5 → 0x140990721** = `lea r9` 指令中部（`cmp [rcx],esp` 起）→ 崩溃（render-gap §3.1 同）。
- 第二表更新规则只产出 disc ∈ {0,2,3,4} → **disc 5..9 实际不可达**；6/7 的第一表条目指向第二表共享块的"带前缀跳过"形态，属编译器对死变体的冷块合并。
- 结论：6..=9 **不是 no-op**（机器码有确定转移/拷贝块），但**在可执行流中不可达**；reconstruction 按 no-op 处理安全正确，注释应更新为"死变体：6/7 = 第二表 idx2/idx3-4 同一转移码（第一表入口 UB），5/8/9 = 指令中部死条目（崩溃）"。

## 5. reset_indirect_args 的 0x1e0 B 暂存区（B_c3）—— **O（真实使用，写入存在）**

- 0x14090a870 反编译 + 机器码（0x14090ab28 `lea rdx,[rip] # 0x142aaaa78`；`mov r8d,0x1e0`；`call 0x142923c00`）证实：**0x1e0 暂存区由 `memcpy` 从 .rdata 静态表 0x142aaaa78 写入**——B_c3"机器码无任何写入"判据**不成立**（漏掉 memcpy 调用）。
- 表内容（O）：0x1e0 B = **24 × 20 B `DrawIndexedIndirectCommand`**，仅 `index_count` 非零：`[0x1b,0x12c,0x60,0x39,0x24,0x6,0x3c,0x6,0x30,0xae,0,0,0x10b,0x126c,0x42,0xb4,0xf,0x384,0x12,0x384,0x12,0x12,0x12,0x12]`（与 render-gap §3.2 一致）。
- 另三块初值复核（O）：0x80 = 8×`[0,1,1,0]`（0x142aaa1e0/e8）；0x10 = `[0,1,0,0]`（0x142aaaf74/7c）；8 B = `0x0009000000000000`。闭包 0x14169a830 4 次 `ResourceRegistry::buffer` + `RenderPassApi::copy_buffer`（O）。
- 结论：0x1e0 是**真实初始化**（索引初值表），非残留；reconstruction 的"按分配返回值原样拷贝"应改为先 memcpy 表内容（render-gap 已列同结论）。

## 6. roof_tiles_gravity f32 uniform 键 `0x601a45ce43af912f`（B_c2）—— **O（名字还原）**

- **`fnv1a("delta_seconds") = 0x601a45ce43af912f`**（逐位吻合，binding_hash 规则同 B_c2/c3 的 8 个已证名字）。
- 值来源（O）：资源 TypeId `0x982798ce80cbcefb / 0x4bc362195b80f729` = **`resources::time::GameTime`**（EXE 全搜 53 处：`insert_resource<GameTime>`/`register_resource_with<GameTime>`/main_pass/water_surface_pass/ui_pass 等）；uniform 值 = **`GameTime.delta_f32 @ +0x28`**（time.rs 章布局 O）。
- 条件（O）：调用方模式字节（[rbp+0xf6]，栈传入）`!= 8` 才写——与 grass 的 `mode != 8`（跳过 freeze uniform）同形；mode==8 语义 I（候选：AppMode 拍照槽），写入方为调用者。
- 名字不在任何 SPIR-V/重建 HLSL（CPU 侧 uniform 名，常量折叠不落串）——与 95% 命中族不同源，但 fnv1a 键值证明名字就是 `delta_seconds`。

## 7. TextureDesc::from(preset) 0xa59890（§5.4）—— **O（映射表+名字，闭合）**

- 函数结构（O）：微型分发器（prologue 0x80 帧 + 表 0x142ad35b0 分发）→ 25 个真实预设（0..0x18）各建 desc → 公共收尾 0x140a59fe8 写 48 B 输出 `{desc32, key u64@+0x20, byte0@+0x28, new_2d 规格 7B@+0x29}`。
- **预设→desc 全表（O）**（extent/格式/用法链/键）：

| preset | 尺寸 | 格式 | 用法链 | key | 名字（xxh3 复算 O） |
|---|---|---|---|---|---|
| 0 | 64² | 0x53 | storage+sampled | 0xe3624a5fa0363c58 | **brdf_lut** |
| 1-5 | 512² | 0x4c | storage+sampled | 各 0xc2fc59b8a62a7bb9 / 0x50f50a6fd3a4462f / 0xced8911eca0705eb / 0xa25c046ba6c05bb4 / 0x240ea7ee12ea47f7 | 1-4 U；**5=paths_exclusion_mask** |
| 6 | 512² | 0x61 | storage+sampled | 0x6749a66bcf038be9 | U |
| 7 | 512² | 0x4c | storage+sampled | 0xa5be6bd038976cb3 | **dirt_mask** |
| 8 | 512² | 0x7a | storage+sampled+mip(5) | 0x09fa1678d41f0c18 | **terrain_lighting** |
| 9 | 512² | 0x46 | storage+sampled | 0x4ed8767ebd4c321c | **terrain_heightmap_texture** |
| 0xa | 512² | 0x7c | sampled+depth_stencil | 0xe673b662ce97671c | **visual_terrain_heightmap_texture** |
| 0xb | 512² | 0x63 | storage+sampled | 0x7efe10a2cce11c9f | U |
| 0xc | 256² | 0x4c | storage+sampled | 0x1cac83c580b2d77b | U |
| 0xd | 512² | 0x53 | storage+sampled | 0x3887a26d3181f0e2 | U |
| 0xe | 512² | 0x61 | storage+sampled | 0x84a037e697de0324 | **rocky_terrain** |
| 0xf | 512² | 0x61 | sampled+color_attachment | 0x35daf42788d4cb5b | **ObjectGrassPush**（二进制内 10 处实串；xxh3 命中；草地对象纹理槽） |
| 0x10 | 512² | 9 | sampled+color_attachment | 0x7abd76ee26e8d462 | U |
| 0x11 | 1200²×3 | 0x7e | 2D array+samples 3+depth_stencil | 0x98c11fe95d7d3b89 | **ShadowMap** |
| 0x12 | 1²×3 | 0x7e | 2D array+depth_stencil | 0xe6dee57299866376 | U（ShadowMap 的低配/立方变体，I） |
| 0x13 | 512² | 0x61 | storage+sampled | 0xa73804971c10b016 | **water_mask** |
| 0x14 | 512² | 0x61 | storage+sampled | 0xc3f7d6e6c37cec9d | **garden_mask** |
| 0x15 | 512² | 0x53 | storage+sampled | 0x6759183d0a9c8971 | U |
| 0x16 | 512² | 9 | storage+sampled | 0xffb5ef44962bea4d | U |
| 0x17 | 225² | 0x4c | storage+sampled | 0x354571e6bba69ded | U |
| 0x18 | 256² | 0x25 | storage+sampled | 0x70b01834b4b40879 | U |

- **名字方法**：key = `xxh3_64(预设名)`（const 折叠，名串不落二进制——"ShadowMap"/"brdf_lut"/"paths_exclusion_mask"/"dirt_mask"/"terrain_heightmap_texture"/"visual_terrain_heightmap_texture" 在二进制 0 处、仍与键逐位吻合，证明名字来自编译期常量）。命中 11/25；其余 14 名字 U（已穷举全部 SPIR-V 标识符 + 二进制串 + 结构化候选）。
- **0x19..0x1f（25..31）为死变体**（O）：表条目指向邻接函数（0x140a5a020 `dof_prepare` 之后的 0x748 帧函数）**指令中部**——与 shader_refactoring 5/8/9 同形；`From<TexturePreset> for TextureDesc` 只覆盖 0..0x18。
- 用法链 helper 全名（O）：0x140bd52a0 new_2d / 0x140bd52e0 new_2d_array / 0x140bd5370 sampled / 0x140bd5390 storage / 0x140bd53d0 color_attachment / 0x140bd53f0 depth_stencil_attachment / 0x140bd5460 mip_levels。
- 纠错（对 reconstruction `usage()`）：预设 0xa 是 sampled+depth_stencil（10/11/12 中只有 0xa 是 depth）；0xb/0xc 是普通 storage+sampled 非 depth；0xf/0x10 color_attachment ✓；8 mip5 ✓；0x11/0x12 2D array+depth ✓（与 render-gap 章一致）。

## 8. Flower03Ssbo NAME / compute_path_masks 资源 E / grass 主题资源（B_c2）—— **2 O + 1 U**

- **Flower03Ssbo**：SsboId = **`0xF78C9C2EB9E1F6FB`**（O：`AssetSsboLibrary::add_new<compute_garden::Flower03Ssbo>` 0x1412c4480 `movabs rdx,-0x837363d1461e905`）。**NAME 仍 U**：已穷举（1）全二进制串（.rdata/.data/.text 全部可打印串，xxh3 无命中）、（2）全部 SPIR-V 标识符 1459 个、（3）重建 HLSL 全部标识符/`__alias_*` 12558 个、（4）~1.2 万结构化花名组合、（5）类型名串本身——均不吻合。工具自检通过（"RoofTileSsbo"/"LanternSsbo"/"ColorLibrary"/"Cobblestone3RasterStagingSsbo" 6 例全中）。**新增同态 U**：`IvyFlowerSsbo` = `0x1d6de5e69ac50d2a`、`LilypadFlowerSsbo` = `0xAE37CBCAFBE88DA4`（add_new 立即数 O，NAME 同 U）——花族 ssbo 的 NAME 均非类型名/HLSL 名。reconstruction 占位 `"Flower03Ssbo"`（xxh3=0x5f3a187f0dd4b356≠键）**确认错误**，需改（候选无；建议以 SsboId 常量锚定）。
- **compute_path_masks 资源 E**（`0xcda6af2f6afb130e / 0x036e88a927bd70aa`）＝ **`Events<rasterize_terrain_stroke::RecalculateHeightMapCmd>`**（O：全 EXE 9 处命中全为 `Events<RecalculateHeightMapCmd>` 的 init_state/register_event 实例）——"非空"判断的 `+0x10/+0x30` = events_a/events_b 两 Vec 长度（同 Env52 `Events<ShowDemoBorderOverlay>` 形制，B_d_cb_a §2）。`PathStrokeQueues` 占位名作废。
- **grass 主题资源**（`0xc2b8e941c91a6428 / 0x59f3c6d1d3da43a0`）＝ **`systems::mode_manager::AppMode`**（O：34 处命中含 `register_resource_with<enum2$<mode_manager::…>>`；低 64 与 B_d_cb_a 表一致）——"GladeThemeId"占位名作废；首个 u32 = AppMode 判别（tag），grass 的 {1,1,2} 表按 tag 7..=9 取 brush index（tag 语义属 mode_manager 域，I）。

---

## 汇总

| 项 | 出处 | 结论 | 卡点 |
|---|---|---|---|
| 1. 全局 `*0x14362c0d8`/`*0x14362c098` | B_s_h | **O**（freeze_occlusion_culling / demo；写入者、refraction 链、uniform 名哈希全证实） | 无（demo 字节"写 0 来源结构"的字段名 U） |
| 2. CENTERED_PATH `*0x14362c308` | B_x_out | **O**（load_game 写入者，`字段>=0x1a`） | 配置字段 `+0xc0` 语义 U（0x1a 含义） |
| 3. 三 TypeId 组件 | B_d_cb_a | **O**（BackgroundTerrain / DontReceiveContactShadows / OverwriteRoughness，128 位全对） | 无 |
| 4. shader_refactoring disc 6..=9 | B_c3 | **O**（6/7=第二表共享转移码、第一表入口 UB；5/8/9 死条目） | 无（维持不可达结论） |
| 5. reset_indirect_args 0x1e0 暂存区 | B_c3 | **O**（memcpy 静态表 0x142aaaa78，24×20B index_count 初值） | 无（B_c3"无写入"判据作废） |
| 6. roof uniform `0x601a45ce43af912f` | B_c2 | **O**（`delta_seconds`；值=GameTime.delta_f32） | mode==8 的语义 I |
| 7. TextureDesc::from(preset) 0xa59890 | §5.4 | **O**（0..0x18 全表+11 名字；0x19..0x1f 死变体） | 14 个预设名字 U |
| 8a. Flower03Ssbo NAME | B_c2 | **U**（SsboId `0xF78C9C2EB9E1F6FB` O；NAME 穷举无命中） | NAME 卡点（新增 IvyFlower/LilypadFlower 同态） |
| 8b. 资源 E | B_c2 | **O**（Events\<RecalculateHeightMapCmd\>） | 无 |
| 8c. grass 主题资源 | B_c2 | **O**（AppMode） | tag 7..9 语义属 mode_manager 域 |

闭合 9 项（O），卡点 2 项（Flower03Ssbo NAME；预设 14 名字 U——均因名字是编译期常量、二进制无串、候选空间穷尽）。
