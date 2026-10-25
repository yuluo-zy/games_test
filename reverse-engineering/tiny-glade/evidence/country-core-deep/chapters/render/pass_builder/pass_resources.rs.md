# render/pass_builder/pass_resources.rs

> 文件作用：PassResources：PassBuilder::render 的 UserData（56B = 7 个帧级资源指针；new_from 按 7 个 128 位 TypeId 查 ECS world 资源表）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::pass_builder::pass_resources::PassResources::new_from` | 0x140909310 | 24 | 按 7 个 128 位 TypeId（A..G）查 world 资源表；B 可选（缺→0）、其余缺失 Option::unwrap；56B 布局 +0 着色器库/+8 纹理库/+0x10 SSBO 库/+0x28 Mutex<DynamicConstants>，+0x18/+0x20/+0x30 类型 U | O | render-top/decomp/decomp-0x140909310.txt（原 B_a4_compute §2/§3.6；wp1/contracts/pass_resources.md；src/pass_resources.rs 头注（0x140909310）） |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `PassResources::new_from`（0x140909310）—— 唯一数据契约（CALL-CHAIN §0）
- 处理链（O）：7 个 128 位 TypeId 常量 → 逐个 `Components::get_resource_id(TypeId)` 查 ECS world 稀疏表（+0x210 类型→id 表、+0x1e0 资源槽）→ 缺失走 `Option::unwrap`（6 处必选 + B 可选记 0）。
- 查找顺序 A,B,C,D,E,F,G（O）；56B 布局：+0 A=AssetShaderLibrary、+8 C=AssetTextureLibrary、+0x10 D=AssetSsboLibrary、+0x18 E、+0x20 G、+0x28 F=Mutex<DynamicConstants>（`rg_buffer_copy_from_slice` 与 `ComputePass::new` 都对其 lock，O）、+0x30 B（可选）。
- 数据流：`render` 帧外层在**主图 execute 前**（R7）与**展示图 execute 前**（R12）各调一次（CALL-CHAIN §2.1 0x140909310 两处）；PassResources 是 `RenderGraphRecord<PassResources>` 的 UserData —— country_core 与 rhapsody 之间唯一共享数据契约。
- panic：A/C/D/E/F/G 任一缺失 → unwrap 失败（六处）；B 缺失静默记 0。
- U：+0x18/+0x20/+0x30 真名（E/G 帧级核心，I 候选 frame_constants/camera/Globals/sampler set）；`ResourceLookup for World` = typed gap（恒 None → 走原版 unwrap panic 路径，RENDER_GAP_ANALYSIS §5.1 #4）。

## 证据注记（tg_rn1）
- **7 个 TypeId**（O，`movabs` 立即数）：A SHADER_LIBRARY 0x8597ee0727b461b3（→+0x00，必选）、B OPTIONAL 0xf0b963d163fe760f（→+0x30，**可选**）、C TEXTURE_LIBRARY 0xa68561b396784eaa（→+0x08）、D SSBO_LIBRARY 0x8226cc786e679355（→+0x10，与 bind_ray_tracing 同立即数）、E 0x874b1541877dacfd（→+0x18）、F 0x47c94cb78283a72e（→+0x28，= Mutex<DynamicConstants>，I）、G 0xb65bf3141f93e938（→+0x20）。
- 查找顺序 A,B,C,D,E,F,G；B 缺失记 0，其余缺失 `Option::unwrap_failed`（六处）。
- U：+0x18/+0x20/+0x30 的**资源真名**（角色：E/G 帧级核心，候选 frame_constants/camera/Globals/sampler set，I）；`ResourceLookup for World` = typed gap（恒 None，走原版 unwrap panic 路径）。
