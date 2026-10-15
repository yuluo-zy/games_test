# render/pass_builder/bind_resources.rs

> 文件作用：BindGameResources trait：把颜色库（ColorLibrarySsbo）与软件 BVH 组（RtTriangles/RtTriangleMeta/RtBvh + WallColorIds）绑定进 ComputePass / DrawBuilder（O：0x1405896e0 / 0x1415fc230 / 0x1416286f0 / 0x1416289d0；B_i_api §3 复核槽名与哈希）。

函数总计 5：业务 4 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::compute_pass_builder::ComputePass>` | 0x1405896e0 | 55 | 登记 ColorLibrarySsbo 读访问：read_access_buffer<ColorLibrarySsbo>（access 0x1a，组 1）→ 注册表命中 hash 0x52ec6a17beb8fef0 追加 AccessRange；**无槽表** | O | render-top/decomp/decomp-0x1405896e0.txt（原 B_i_api §3；B_a4_compute §2；wp1/contracts/bind_resources.md §2；src/bind_resources.rs 头注（0x1405896e0/0x1415fc070 ICF 同体）） |
| `country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::draw_builder::DrawBuilder>` | 0x1416286f0 | 55 | 同上，但经 HashMap<&str,PassResourceSlot>::rustc_entry 槽表（已占槽不重复登记）；access 0x1f | O | render-top/decomp/decomp-0x1416286f0.txt（原 B_i_api §3；B_a4_draw §2；wp1/contracts/bind_resources.md §2（0x1416286f0）） |
| `country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::compute_pass_builder::ComputePass>` | 0x1415fc230 | 59 | 取 RtTriangles/RtBvh 缓冲（SsboId 库查 + 名字断言 + unwrap）→ 4 槽 read_access_buffer（Tri/Meta/Bvh/WallColorId，0x1a）→ 2×write_uniform<u32>（Tri/Bvh 缓冲字）；ComputePass 版**不**写 CPU 缓冲 | O | render-top/decomp/decomp-0x1415fc230.txt（原 B_i_api §1.3/§3；B_a4_compute §2；wp1/contracts/bind_resources.md §3（0x1415fc230）） |
| `country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::draw_builder::DrawBuilder>` | 0x1416289d0 | 59 | 同上 + WallColorId 回读（read_storage_buffer<WallColorIdSsbo> 槽 "WallColorIds"，xxh3=0xaf42315d263aa22a）；DrawBuilder 版写 CPU 缓冲（+0x198/+0xa0）供 CPU 侧使用 | O | render-top/decomp/decomp-0x1416289d0.txt（原 B_i_api §1.2/§2（0x1416289d0，4167B）；B_a4_draw §2；wp1/contracts/bind_resources.md §3） |

## 内联/无独立符号函数

- `country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap`（行 50）— 占位 shadowmap 绑定（inline 12 宿主；无独立过程；与 bind_dummy_shadowmap 相关）—— I

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `bind_color_library`（0x1405896e0 ComputePass / 0x1416286f0 DrawBuilder，ICF 两族）
- ComputePass 版：`read_access_buffer<ColorLibrarySsbo>`（access 0x1a、组 1）→ 注册表命中 `fnv1a("ColorLibrary")=0x52ec6a17beb8fef0` 追加 AccessRange；**无槽表**。
- DrawBuilder 版：经 `HashMap<&str, PassResourceSlot>::rustc_entry` **槽表去重**（已占槽不重复登记访问）；access **0x1f**。
- 位置：`ComputePass::new` 末尾与 draw 回调绑定序末尾各调一次（O）。

### 2. `bind_ray_tracing<Tri,Meta,Bvh,Wall>`（0x1415fc230 ComputePass / 0x1416289d0 DrawBuilder）
处理链（O，B_i_api §1.3/§2）：
1. 取 RtTriangles/RtBvh 缓冲（SsboId 库查 + **名字断言 memcmp**（"RtTriangles"/"RtBvh"） + 缺失 unwrap）；
2. 4 槽 `read_access_buffer`：RtTriangles / RtTriangleMeta / RtBvh / WallColorId（槽名 xxh3 复算吻合；ComputePass 版 access 0x1a，DrawBuilder 版 access 0x1f + 槽表）；
3. DrawBuilder 版多一步 `read_storage_buffer<WallColorIdSsbo>`（槽 **"WallColorIds"**，`xxh3 = 0xaf42315d263aa22a`，复数）并把墙色**回读进 CPU 缓冲**（+0x198/+0xa0）；
4. 2×`write_uniform<u32>`：键 0x1c9f2eef0ac739ac（Tri 缓冲字）/ 0xc2b7f2fdf5e21521（Bvh 缓冲字；取值来源 I：`Buffer+0` 低 32 位）。
- 调用方：`deferred_raster`、`water_planes_draw_rt`、`ray_trace_gi`、`primary_rt_debug` 等软件 BVH 消费者（HLSL 侧为软件 BVH 追踪，无 RT capability，O）。
- panic：Tri/Bvh 缓冲缺失 unwrap（名字断言失败）；uniform 未命中静默。

## 证据注记（tg_rn1）
- **哈希常量**（O 立即数）：COLOR_LIBRARY_BINDING_HASH 0x52ec6a17beb8fef0；RT_TRIANGLES 0x168ddb2c3b5f38a9 / RT_TRIANGLE_META 0x47ddebfb264383d0 / RT_BVH 0x339e76846042a271 / WALL_COLOR_ID 0xe9701a257aa8828c；uniform 键 0x1c9f2eef0ac739ac（Tri 缓冲字）/ 0xc2b7f2fdf5e21521（Bvh 缓冲字，取值来源 I）。
- **槽名纠错**（B_i_api §1.2）：墙色槽名是复数 **"WallColorIds"**（0x14162c290 的 rustc_entry 键 0xc，xxh3=0xaf42315d263aa22a），Rt 三个槽名 = "RtTriangles"/"RtTriangleMeta"/"RtBvh"（xxh3 复算吻合）。
- **两族差异**：ComputePass 版无槽表、不写 CPU 缓冲；DrawBuilder 版有 `HashMap<&str, PassResourceSlot>` 槽表、回读墙色到 CPU 缓冲。
- U：`bind_ray_tracing` 两个 u32 uniform 的取值来源（I：`Buffer+0` 低 32 位）；接入 `resources::rt::*` 真实类型前的载体（含 Vec 不满足 StorageElement: Copy）。
