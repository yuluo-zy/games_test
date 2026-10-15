# 组 D 渲染域 未探究项审查（render-loop + roofs-rt + backend-misc + asset-lib-render / render-nani / render-gap）

日期：2026-10-15。review agent（组 D）。
范围：render 顶层 37 章 + `systems/render_loop/**`（78 章）+ `resources/{roofs,rt,render}/**`（20 章）+ backend-misc（16 文件）。
标注：**O** 实测 / **I** 推断 / **U** 未测量；〔类别〕= 卡点 / 弱证据 / 语义 I / 字段 U / 接线 / 去重 / 算法。
只做审查，未改任何源码/章节。

---

## 0. 台账结论核对（MODULE-DEEP-DIG / 各 DIGEST）

- render_loop DIGEST 口径：78 文件 / 159 业务 / 48 内联 / 121 胶水；decomp 156 个（146 OK + 10 NotFound 手解）；章节占位符清零。抽查复核：**成立**（见 §5、§7）。
- roofs-rt DIGEST 口径：20 章全填充，decomp 63/89 批处理 + 26 手解 + 12 既有 .c；"所有业务函数至少 I 级，无臆造字段名"。抽查复核：**成立**（3 个 I 全列出，见 §1.2）。
- backend-misc DIGEST：16 文件 21 业务 + 5 助手地址。复核：与既有关联记录一致，无新 I/U 引入。
- RENDER_GAP §10 闭合记录 9 项 O / 2 卡点——卡点复查见 §2。

---

## 1. U/I 行盘点

### 1.1 render-loop I 行（任务注记 I=19，实查 **26** 行）

逐章表行（逆向结论列以 I 开头）26 条。差额（26 vs 19）说明：主张的 19 应不含 draw_shaders/water_planes_draw 后来补的"同型"I 行与 clearing_shaders 等 7 条（**待中央裁决口径**，但 26 行全部列全如下）。

**A. 无 build 结构 / 弱证据（3）**
| 行 | 地址 | 缺口 | 建议 |
|---|---|---|---|
| `lut_gallery::impl$0::build` | — | "无 build 结构，判定为弱证据"（wp5 无记录） | 回查反汇编确认是否 ICF 空桩（同 fsr_plugin 0x14003bb80 形态），重判 O/I |
| `menu_background::impl$0::build` | — | 同上 | 同上 |
| `logo_overlay::impl$0::build` | — | 同上（wp5 记为 O） | 同上 |

**B. wp5 记录含糊（3）**
| 行 | 缺口 | 建议 |
|---|---|---|
| `primary_rt_debug::primary_rt_debug_*` | wp5 无此记录，判定为弱证据 | 补 rne-read 首字节 + procs.tsv 身份 |
| `visibility_queries::run_*` | wp5 记录含 `` `\x01` `` 不可读 | 重读串（可能 fat ptr 错位） |
| `grass::impl$4::clone_behavior` | 纯胶水 I（O 名） | 已由 roofs-rt 证 0x140002370 为空桩，真实现 0x1408e6af0（roof 域），此处保持 I 即可 |

**C. 已证实但与代码不符（1）**
| 行 | 地址 | 缺口 | 建议 |
|---|---|---|---|
| `draw_lists::DrawLists::new` | 0x1409ba340 | "缺失证实初始化模式，为映射推断"——机器码为小结构初始化（2 内联 Vec + 计数器=1），与 reconstruction `default()`（21 空 Vec）不符 | **中央裁决**（地址→符号可能错位；render-loop DIGEST §3 已登记） |

**D. shader body 属 GPU 后置（9 draw_shaders 同型 + clearing_shaders）**
`clearing_shaders::tree_trunk`、`common_includes::{bind_water_object_push<ComputePass>, bind_water_object_push<DrawBuilder>, bind_garden_exclusion<ComputePass>}`、
`draw_shaders::{draw_bricks::indirect_draw_bricks<0/1>, floor_shader, grass_shader, grass_shader::draw_grass_lod1_shader, instanced_roof::draw_instanced_roof_shader_nani/spire/support_beam, path_pebble_draw, rocky_terrain_shader, stairs_draw, water_planes_draw/lilypad/lilypad_flower/reed}`（共 17 行，其中纯 `I` 9 行）：
缺口 = 结构 O（draw_instanced / draw_*_w_gl_draw_flag 接线）但 shader 内部算法无目标（**GPU 后置，wave-2 HLSL 域**，项目规则明确）。建议：维持"结构 O / 内部算法后置"，不逐行反编译。

**E. 额外 I 标记（非表行，散见内联/要点清单）**
- `gpu_generate_draw_lists::IndirectArgsType::buffer_offset` — O 名、语义 I。
- `compute_path_masks` 运行条件 A∥B∥C∥E非空∥D 中 E 的事件驱动语义（I）。
- `prefix_scan` 复用链 `/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl`（I）。
- `session_selection` 三层索引中 render_session_selection_previews 与 wp5 无冲突（未列 I，但部分路径依赖 painterly 缩略图语义，I）。

### 1.2 roofs-rt I 行（**3**，全列出）

| 行 | 地址 | 缺口 | 建议 |
|---|---|---|---|
| `roof_shape::FlatRoofStyle::is_wooden` | — | 变体语义 I（该族 5B 谓词与 wall 谓词跨类型去重，机器码判 is_stepped=tag==0 / is_overhang=tag!=0；is_wooden 未单独判） | 按同族机器码差分补 rne-read；低优先 |
| `rt::mod::impl$1::from` | 0x1408bc460 | 打包转换常量未全解 | 定点反编译（小函数，字节已存） |
| `RtBvhBuilderGlueResource::is_idle` | — | 小函数字节已存但 busy refcount==0 语义仅 I | 按 `+0x10` refcount 读解补 O；低优先 |

（roof_shape 另有"RoofShapeParams 字段名 = JSON 语义推断，字节偏移/数学行为独立确认"的保留意见——roofs-rt DIGEST §7.4，维持。）

### 1.3 render 顶层 37 章 I/U 行（**非表行为主**）

render 顶层业务表行 550 O + 1 空结论（nani/buffers 布局表误解析，非真空行）；I 标记主要散见"纠错/差异"与"要点"清单，逐文件已收：
- `RasterPipelineDesc` +0x18 字段语义（feature/binding 选择子）——U。
- `FrameConstants` 全 UBO 字段列表（main_view/sun_view 成员名 O）——U。
- `ComputeShaderProgram` 字段级布局细节——U。
- `AvailableGraphicsDevice` 160 B 记录字段布局（reconstruction `_raw:[u64;20]` typed gap）——U。
- `nani::draw_ext::nani_draw`：External subset 的 draw_count 来源；Owned 的实例间接缓冲 RG 句柄——U。
- `nani::buffers`：`GpuInstance` 字段（B_n_world U 清单）；`NaniDrawListParams::{list_draw_merge_window_range, list_draw_merge_window_subtotal_range}` 语义——I/U。
- `nani::systems`：`nani_update_*` 各 kind 载荷字段已有 20 系统 O，残留 `Roof/Floor 双缺 unwrap` 等边界语义已 O；`InstanceHandle::subset`（nani.rs.md）——待推断残留。
- `render_config::VideoSettings::upgrade`（render_config.rs.md 行 20）——待推断残留。
- `dirty_index_range::DirtyIndexRange::to_range`（dirty_index_range.rs.md 行 12）——待推断残留。
- `nani.rs.md` `InstanceHandle::subset` / `impl$3::component_ids`——待推断残留。

**〔质量〕7 章文件头仍留"文件作用：**待推断**（由章节 agent 按函数名+证据填写）"占位**：
`adaptive_render_state.rs.md`、`dirty_index_range.rs.md`、`fsr_plugin.rs.md`、`nani.rs.md`、`raster_single.rs.md`、`render_config.rs.md`、`uniform.rs.md`。
建议：下一轮统一回填文件作用（多数实际已有内容，仅头部模板未清）。

---

## 2. 渲染 U 卡点复查

### 2.1 Flower03Ssbo NAME —— **仍卡（U）**
- SsboId `0xF78C9C2EB9E1F6FB` O（`add_new<compute_garden::Flower03Ssbo>` 0x1412c4480 立即数）；NAME 穷举（全二进制串 / 1459 SPIR-V 标识符 / 12558 HLSL `__alias_*` / ~1.2 万花名组合 / 类型名串）无命中；工具自检 6 例全中。
- 同态 U：`IvyFlowerSsbo`=0x1d6de5e69ac50d2a、`LilypadFlowerSsbo`=0xAE37CBCAFBE88DA4（add_new 立即数 O）。
- reconstruction `compute_garden.rs` 仍以占位 `Flower03Ssbo`（xxh3=0x5f3a187f0dd4b356≠键，**已确认错误**）并自注 U。
- 建议：以 SsboId 常量锚定占位（安全），NAME 保持 U 登记；可选低价值方向：w2/w4 已验证的 `get_rg_buffer` 槽名表（0x142c8151d 系）再扫一轮——若花族 ssbo 的 NAME 实为槽名则已在搜，否保持卡点。

### 2.2 TextureDesc::from(preset) 14/25 预设名 —— **仍卡（U）**
- 0..0x18 全映射表 O（尺寸/格式/用法链/key）；11/25 名字 xxh3 复算命中（brdf_lut/terrain_lighting/ShadowMap/water_mask/garden_mask/rocky_terrain/ObjectGrassPush/…）；0x19..0x1f 死变体 O。
- 名字为编译期常量、二进制无串、候选空间（SPIR-V + 二进制串 + 结构化候选）穷尽 → 仍 U。
- **可推进方向（新）**：映射 preset 索引 → 消费方 pass。`From<TexturePreset>` 调用点（各 compute/draw pass 的 TextureDesc::from(preset=N)）已可定位；preset N 对应 pass 内 shader 资源名/槽名已知族（reconstructed HLSL 字典 95% 命中）——用"preset 槽在哪个 pass 以哪个 name 绑定"交叉推 14 个未知名（如 preset 0xb/0xc/0xd 的 storage+sampled 512² 常被 water/GI 链使用）。该方向未在 RENDER-U-CLOSURE 记录中执行，属**未探究**。

### 2.3 RENDER_GAP §5.1 接线项复查

| # | 项 | 状态（本次复核） |
|---|---|---|
| 1 | 整个 crate 编译（dolly/mint 缺失） | U，环境项，不在 render 域内；**在跑**（core↔rhapsody 调用链 agent abecd05c 覆盖编译接线） |
| 2 | `render_world`/`render` 接真实 pass 入口 | 未做；`render_world.rs.md` 章仅 13 行 + 全局门控表（54-pass 序列在 B_s_h.md 未并入章节）——**在跑**（abecd05c） |
| 3 | 水/冰 `SurfaceCompute` | 未做，集成者提供——未动 |
| 4 | `PassResources::new_from(&World)` | **仍 open**：reconstruction `pass_resources.rs` 头注 7 帧级资源 TypeId O 已捕获（movabs 立即数），但 TypeId→类型名映射仍未建立，`new_from` 对 `&World` 仍走 unwrap_failed panic 路径（§5.1 #4 原文维持）；round3 §3 的三 TypeId 闭合是 bundle components，**不是**这 7 个。建议：`typeid_resolve.py` 22+33 真名表扩展到这 7 个新 TypeId，建立资源表 |
| 5 | `PassHost` 缺 World 访问器 | 未动；`nani_draw` 显式传 `GpuWorld`/`AssetSsboLibrary`/`RenderGraph`（B_i_nani §2 记）——未在跑 |
| 6 | `rhapsody_contract`（lib.rs:82–330）stub | 仍 stub；lib.rs:156/177 两处 `todo!`（managed handle get）——**在跑**（abecd05c） |
| 7 | `shader_pipeline_contract` 等 4 stub | 未动 |
| 8 | `rg_boundary`/`rhapsody_boundary`/`shader.rs` boundary | 未动 |
| 9 | `frozen_map.rs` RwLock<HashMap> 占位 | 未动 |
| 10 | `reused.rs` hotwatch 替身 | 未动 |

---

## 3. 无结论行：SOURCE_MAP render 业务函数 vs 章节覆盖

- SOURCE_MAP-render.json：render/** **782 业务函数**。
- 章节表行地址覆盖：598/782（**76%**）。未逐地址覆盖的 **184**：
  - `render/ssbo.rs` 113 —— **族级归并**（6 族，代表地址 + 布局表；ssbo.rs.md "113 业务 → 6 族"）。族内各实例（如 `new_storage_buffer` 38 实例）未逐实例反编译，靠同族形态推断——**结论级 I 可接受**，但每个实例的 T 类型→地址配对只在代表行登记。
  - `render/shadowed_buffer.rs` 39 —— 族级归并（22 类型 new+gpu_flush），同前。
  - `render/sparse_instance_buffer.rs` 31 —— 族级归并（4 类型族），且章内自注"每类型族内 hide/show/take_out_waste/gpu_flush/push/clear/to_draw 精确 VA 配对未逐条反编译，按同类型簇地址连续推断（**I**）"。
  - `render/mesh.rs` 1 —— `impl$7::from` 已并入"from（Vec 属性 From impls）"合并行（0x140a0db00/0x140a0db90），非缺口。
- 文件级覆盖 **37/37 全有章节**（无缺章文件）。
- **〔未探究〕族级归并行的逐实例地址配对**是 render 域最大的"无结论"面：782 业务里 184 行（24%）只有代表地址，实例级 VA 未独立核验（ICF 折叠、同型异参 callee 等以 w1 的"read/write_access_buffer 53/37 实例 ICF 描述不精确"为证，见 §5 末）。建议：对 6 族各抽 1-2 个非代表实例做 rne-read 首字节 + procs.tsv 身份核验，闭合"族归并"风险。

---

## 4. 算法未深挖（算法级 vs 结论级）

| 面 | 结论级（已 O） | 算法级（未深挖） | 归属 |
|---|---|---|---|
| pass 声明层 ComputePass/DrawBuilder | `new`/`read_/write_access_buffer`/`dispatch_with`/`multi_draw_indexed_indirect` 全链 O（w1 29 条 29/29 一致；构造/注册 FUN_1416032a0 模式、收尾 FUN_1409dd4d0、dispatch 尾链 O） | **dispatch 模式判别 2/3/4（ComputePass）/1/3/4/5/7/8（DrawBuilder）→ Rust 枚举名仍 U**（render-loop DIGEST U #5）；`DrawLists`/`DrawData` 字段布局与 `DrawEntry` 定义 U（DIGEST U #1） | render-loop + render 顶层 |
| nani GPU-driven（gpu_generate_draw_lists） | CPU 编排全链 O（round0/round≠0 双路径、clear→cull_and_bucket→histogram_prefix_sum→group_draw_cmd_prefix_sum→generate_draw_lists→fill indirection→dispatch 各 shader URI/缓冲名，DIGEST §1 + 章）；`nani_draw` 9 步 O；20 个 `nani_update_*` 打包 O | **shader 内部算法（cull_and_bucket 遮挡剔除数学、GPU histogram/prefix sum、排序、间接分派参数生成）全部 GPU 后置（wave-2 HLSL 域）**——项目规则明确，不重复；残留 CPU 侧 U：External subset draw_count 来源、Owned 实例间接缓冲 RG 句柄 | render 顶层 nani/** + render-loop compute_passes |
| render_loop 帧编排 | 帧序 W#（W1..W54）、54-pass 调用序（B_s_h.md）、render_world 全局门控（round3 闭合 freeze_occlusion_culling/demo） | **`render_world.rs.md` 章只有 13 行**：54-pass 逐条序列未并入章节（仍指向 B_s_h.md，未独立复核）；回读/高亮链（W5-W8）与 render_ssbo 域的 `ScheduledTerrainReadback`/`BushCpuSsbo` 语义 I | render-loop |
| BVH（resources/rt/bvh.rs） | `build_gpu_bvh` 布局级 O：输入三角 stride 0x28、输出节点 stride 0x30、空→3 节点退化根、两三角一组重排复制；`bvh_builder_worker` 后台循环 O | **构建算法未深挖**：无 LBVH/morton/中位切分/扫描线分类任何分析——"逐 u32 重排包围盒/子信息"停留在结构观察，节点间父子链接/每层容量规则 U | roofs-rt |

---

## 5. 证据链抽查（15 条 render 顶层 O 行，seed=20261015）

抽样：`chapters/render/**` 550 O 行中 549 个带地址行，剔除共享空桩 0x140002370，随机 15 条。

**结果：身份 15/15 全中（procs.tsv PDB 精确符号逐条吻合）；机器码形态 11/11 抽查一致（rne-read 首字节 + PDB size 对照）；decomp 落盘 0/15 存在。**

| # | 地址 | PDB 符号（procs.tsv，截断） | 机器码/形态 | decomp |
|---|---|---|---|---|
| 1 | 0x141607210 | ComputePass::write_storage_buffer<NormalDebugSsbo>（464B） | prologue+0x32 access 调 write_access_buffer 族 | ✗ |
| 2 | 0x1415ffc60 | ComputePass::new<HalfResDof>（2311B） | 400B 返回结构；2311B 带 PERM 变体（与 w1 核 2279B 实例不同，**不矛盾**） | ✗ |
| 3 | 0x140973a20 | write_access_buffer<BrickNextVisibilityBitmaskSsbo>（253B） | 标准 write_access 形态 | ✗ |
| 4 | 0x140976170 | write_access_buffer_concurrent<BushSsbo>（267B） | 同写+同步组实参 | ✗ |
| 5 | 0x141572d80 | impl$36::register_required_components<SolidAlbedoMapEmissiveInstanceData>（1776B） | bevy 派生展开族 | ✗ |
| 6 | 0x14156de90 | update_instance_data<SolidVertexColorInstanceData>（836B） | 与 w2 已核 0x14156e880 同 fast-path（44 8b 72 04 / 4c 39 71 10 …）逐字节同构 | ✗ |
| 7 | 0x141570340 | update_instance_data<ClothInstanceData>（836B） | 与 #6 首 48B 逐字节相同（合法单态差异仅寄存器） | ✗ |
| 8 | 0x1415ff350 | ComputePass::new<NaniCullAndBucketMain>（2311B） | 同 #2 形态 | ✗ |
| 9 | 0x14048f750 | write_optional_uniform<…closure_env$4>（272B） | hashbrown 查找形态（shr r8,0x39）可见 | ✗ |
| 10 | 0x140972220 | read_access_buffer<DofAutofocusSsbo>（256B） | 与 w1 chunk#7（0x140972220 已核"一致"）同一地址——双重命中 | ✗ |
| 11 | 0x140970420 | read_access_buffer<RoofTileSsbo>（256B） | 标准 read_access 形态 | ✗ |
| 12 | 0x140973620 | read_access_buffer<RockyTerrainTriangleSSBO>（256B） | 同上 | ✗ |
| 13 | 0x141605170 | write_storage_buffer<StairsTransformSsbo>（464B） | 0x32 access 调 write_access_buffer 族 | ✗ |
| 14 | 0x141623a10 | cull_shaders::impl$18::as_compute_pipeline（1147B） | 路径分配 37B（`b9 25`）+cs_main+PERM define；路径长表吻合；**章节归置在 pass_builder/shader_uri.rs.md 属"URI 消费者"归置可接受，但文件作用为待推断** | ✗ |
| 15 | 0x141572250 | impl$14::type_name<…>（13B） | `lea rax,[rip]; mov edx,0x4d; ret`——类型名字符串 {ptr, len 0x4d} 与 type_name 语义吻合 | ✗ |

**结论**：
- **身份与机器码层面全部通过**（这些 O 行虽由远端会话按 B_* 审计文档 + wp1/wp3/wp4 合同填写，但地址→符号→机器码三者一致，抽样未发现错误结论）。
- **证据链层面存在系统性缺口**：render 顶层 37 章 498 个唯一地址中 **490 个无任何 decomp-*.txt 落盘**（仅 8 个有，且其中 0x140002370/0x14003bb80 是跨域 NotFound 副本/已知 ICF）。对比 render-loop 域 132 地址仅 1 个缺 decomp（0x140a66970，grass::impl$4::register_required_components，且该族 round3 有 rrc 兄弟文件）。→ render 顶层的 O 结论依赖 B 阶段审计文档/合同，非独立 decomp 链；**若需交付级证据，须为这 490 个地址补 decomp（或至少每个归并族抽代表实例落盘）**。
- **已知措辞瑕疵与本章节冲突**：w1 已核 "read/write_access_buffer '53/37 实例 ICF' 描述不精确（每实例 per-T callee 不同，非折叠）"，而 render 顶层 `pass_resource_access.rs.md` 的 `read_access_buffer` 行仍写"同一泛型（53 实例，ICF）"——**章节措辞待修正**（同族实例是独立函数，非 ICF 折叠）。
- `raster_single.rs.md` 0x1409d00a0 行的证据引 `round3 oa2-out/w2-decomp-1409d00a0.txt`——**该文件不存在**（oa2-out 目录不存在；w2 的核验在 MCP 会话 tg_oa2w2 内完成未落盘）。结论经 w2 核验成立，但证据引用需补落盘或改引。
- 7 章文件头"文件作用：**待推断**"占位残留（§1.3）。

---

## 6. 渲染顶层 37 章质量审计（小结）

- **内容质量分层**：nani/{gpu_world,systems,draw_ext,buffers}、pass_builder/{compute_pass_builder,pass_resource_access,shader_uri}、mesh.rs、shader.rs、ssbo.rs 等为高信息量章（含布局表/族归并表/纠错）。fsr_plugin.rs.md 质量好（0x14003bb80 ICF 空桩判定与 round2 复核一致）。
- **模板残留**：7 章文件头"待推断"（§1.3）；`dirty_index_range/run*`、`render_config::VideoSettings::upgrade`、`nani.rs` 若干行仍"待推断"。
- **证据链**：见 §5 —— decomp 落盘率 8/498（1.6%），与 render-loop 131/132 形成鲜明对比。
- **归置**：`cull_shaders::impl$18::as_compute_pipeline` 行放在 `pass_builder/shader_uri.rs.md` 而非 nani/cull_shaders 章（URI 消费者归置，可接受但注意）。
- **抽查未发现错误结论**（15/15 身份全中），但**不可独立复验**是最大短板。

---

## 7. 未探究项汇总（交付清单）

| # | 未探究项 | 类别 | 缺口 | 建议 |
|---|---|---|---|---|
| 1 | `DrawLists`/`DrawData` 字段布局、`DrawEntry` 定义 | 字段 U | 21 层 Vec<DrawEntry> 内容与元素定义（wp5 同 U） | 定点反编译 `DrawLists::new`（0x1409ba340，同时裁决地址错位候选） |
| 2 | dispatch 模式判别 → Rust 枚举名 | 语义 I | ComputePass 2/3/4 与 DrawBuilder 1/3/4/5/7/8 无对应枚举 | 以 w1 反编译记录建判别表 → 命名枚举 |
| 3 | `PassResources::new_from` 7 帧级 TypeId→类型名 | 接线/类型 | §5.1 #4 仍 open，`&World` 调用必 panic | `typeid_resolve.py` 扩展到 7 新 TypeId，建资源表闭合 |
| 4 | render 顶层 490 地址无 decomp 落盘 | 证据链 | O 行不可独立复验 | 每归并族抽代表实例补 decomp + rne-read；修"53/37 ICF"措辞 |
| 5 | 7 章文件头 + 若干行"待推断"残留 | 模板 | 文件作用未回填 | 统一回填（内容多数已有） |
| 6 | 族级归并行逐实例地址配对（ssbo 113 / shadowed_buffer 39 / sparse_instance_buffer 31） | 弱证据 | 实例级 VA 未独立核验，sparse_instance_buffer 自注按地址连续推断（I） | 6 族各抽 1-2 非代表实例核验身份 |
| 7 | `render_world.rs.md` 章仅 13 行 | 结论级薄 | 54-pass 序列仍在 B_s_h.md 未并入/独立复核 | 章节扩写 54-pass 逐条（接线后随 #2 项） |
| 8 | BVH 构建算法 | 算法 U | 节点父子/每层容量/排序规则未分析（仅结构级） | 对 build_gpu_bvh 0x140b0da50 做算法级反编译（morton/中位/分层判定） |
| 9 | Flower03Ssbo / IvyFlower / LilypadFlower NAME | 卡点 | 编译期常量、候选穷尽 | 维持 SsboId 锚定 + U 登记；低价值续搜 |
| 10 | 14/25 预设名 | 卡点 | 同上 | **新方向**：preset 索引→消费 pass→shader 资源名交叉推 |
| 11 | 水/冰 `SurfaceCompute`、`PassHost` World 访问器 | 接线 | 集成者/显式参数 | 待 core↔rhapsody 收尾后补 |
| 12 | 各类 `*Ssbo` 字段布局（DeferredRasterResources/FinalPostResources/ShapeStreamSsbo/TextPrimSsbo/…） | 字段 U | render-loop DIGEST U #3 | 后续批量布局表 |
| 13 | `roofs-rt` 3 个 I（is_wooden / impl$1::from / is_idle） | 语义 I | 常量/变体未全解 | 小函数定点反编译，低优先 |
| 14 | `render_config::VideoSettings::upgrade` / `DirtyIndexRange::to_range` / `nani.rs` 2 行 | 待推断残留 | 无结论 | 下一轮补 |
| 15 | `raster_single` 0x1409d00a0 证据引用指向不存在文件 | 证据链 | oa2-out 目录不存在 | 补落盘 w2 反编译或改引 |

---

## 8. 一句话总结

render-loop 域（decomp 链完整）与 roofs-rt 域（3 个 I 全部登记）审查通过；**render 顶层 37 章抽查 15/15 身份一致、0 错误结论，但 decomp 落盘率仅 1.6%（490/498 地址无文件），是全渲染域最大的证据链缺口**；渲染 U 两卡点（Flower03Ssbo NAME、14/25 预设名）维持 U（预设名新增"preset→消费 pass"交叉推方向未执行）；RENDER_GAP §5.1 的 #2/#6（render_world 接线、rhapsody_contract）在 core↔rhapsody agent（abecd05c）处理中，**#4 PassResources::new_from 不在跑、仍必 panic**，建议纳入后续。
