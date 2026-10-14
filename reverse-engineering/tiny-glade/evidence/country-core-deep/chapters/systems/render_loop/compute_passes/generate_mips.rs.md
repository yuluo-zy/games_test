# systems/render_loop/compute_passes/generate_mips.rs

> 文件作用：MIP 生成（SPD 单趟）：`generate_mips<WriteTextureUnchecked>` / `generate_mips<TexturePreset>`（W50，两单态按访问模式/纹理预设）。

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>` | 0x1400fb890 | 13 | MIP 生成（WriteTextureUnchecked 单态）：串证据 `Generate mips/generate_mips.cs.hlsl` + `/generate_mips.cs.hlsl` + `assertion failed: texture_desc.mip_levels <= 13`；SPD；ComputePass::read_write_texture + push_constants + dispatch_groups | O | render-loop\decomp\decomp-0x1400fb890.txt（串：Generate mips、/generate_mips.cs.hlsl、assertion mip_levels<=13）；wp5 compute-passes §18 |
| `country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::resources::texture_presets::TexturePreset>` | 0x140a3a8f0 | 13 | MIP 生成（TexturePreset 单态）：同上；`TODO: add a _get_ only variant for temporal resources`（时域资源只读变体缺口标注）；eq+expect 断言 | O | render-loop\decomp\decomp-0x140a3a8f0.txt；wp5 compute-passes §18 |

## 关键事实

- 断言（O）：`texture_desc.mip_levels <= 13`（纹理最大 13 mip 层）。
- 分支：`param_6[0xa9+(type==0x13)]!=0`（frame-wiring §3）。
- 时域资源注释（O）："TODO: add a _get_ only variant for temporal resources"（开发注记）。
