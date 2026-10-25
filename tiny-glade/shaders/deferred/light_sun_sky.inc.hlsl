// ==RECONSTRUCTED== shaders/deferred/light_sun_sky.inc.hlsl
// 中文说明:
//   延迟"太阳+天空"光照共享头文件,被 8 个 bin(deferred_{canopy,grass,plant,solid}
//   _light_sun_sky)嵌入。四个全屏合成 pass 共享的太阳+天空光照。观察到的数学
//   (来自 _deferred_solid_light_sun_sky ps_main 的 spirv-cross):
//     - 由 gbuffer+深度重建世界位置/法线(RGB8 法线解码);
//     - 太阳:能量守恒 GGX 漫反射 + Cook-Torrance 镜面 * sun_color * NdotL;
//     - 天空环境:对间接辐射 3-slab 空间解析做 SH 带 1 求值;
//     - ReSTIR 灯笼光:reservoir 收集 + 流式采样(inc/reservoir.hlsl);
//     - 网格灯:light_indirection_grid / grid_light_indices 迭代。
//   辅助函数名为推断值(模块内被内联)。

#ifndef TINYGLADE_DEFERRED_LIGHT_SUN_SKY_INC_HLSL
#define TINYGLADE_DEFERRED_LIGHT_SUN_SKY_INC_HLSL

#include "inc/rt.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/lighting_indirect.hlsl"
#include "inc/reservoir.hlsl"
#include "inc/lanterns.hlsl"
#include "gi/gi_settings.inc.hlsl"

// terrain_heightmap.inc 属于目标 include 闭包(在 rt.hlsl 之后被拉入);
// 延迟 pass 不绑定地形纹理,故跳过引用全局的辅助函数。
#define TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL
#include "inc/terrain_heightmap.inc.hlsl"

// 由像素深度 + 采样 UV 重建视图空间位置。view_constants[0].sample_to_view
// 把 [0,1]x[0,1] 的 uv 映射到一条视图射线。
float3 reconstruct_view_position(float2 uv, float depth, float2 depth_dims)
{
    float2 ndc = (uv - 0.5f.xx) * float2(2.0f, -2.0f);
    float4 view_pos = mul(view_constants[0u].sample_to_view, float4(ndc, depth, 1.0f));
    return view_pos.xyz / view_pos.w;
}

// 解码 gbuffer 法线(RGB8,xyz 以 n*0.5+0.5 存储)与打包标志字节(反照率字的高字节):
// bit0 = velocity/skylight?, bit1 = ?, bit2 = 屏幕空间阴影, bit3 = 编辑高亮,
// bit4 = 山丘高亮, bit5 = ssgi, bit6 = 树冠向上增强。名字为推断值。
float3 decode_gbuffer_normal(uint packed)
{
    float3 n = float3(float(packed & 255u),
                      float((packed >> 8u) & 255u),
                      float((packed >> 16u) & 255u)) * (1.0f / 255.0f);
    return normalize(n * 2.0f - 1.0f);
}

// 给定粗糙度/反照率表面对太阳光的漫反射+镜面(匹配观察到的 GGX 能量守恒
// 漫反射与 Cook-Torrance 镜面)。
float3 evaluate_deferred_sun_light(float3 albedo, float roughness, float3 normal,
                                   float3 view_dir, float3 sun_dir, float NdotV)
{
    float NdotL = saturate(dot(normal, sun_dir));
    float3 H = normalize(view_dir + sun_dir);
    float NdotH = saturate(dot(normal, H));
    float HdotL = saturate(dot(H, sun_dir));
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    return get_sun_color() * (diffuse + specular) * NdotL;
}

// 在半分辨率位置采样 3-slab 间接辐射纹理(slab 每通道步长 = dims.x)。
// 返回加权 SH 带 0/1 辐照度。
float3 sample_indirect_radiance(Texture2D indirect_radiance_tex, uint2 half_pixel, uint2 dims)
{
    float3 ir = float3(indirect_radiance_tex.Load(int3(half_pixel + uint2(0u * dims.x, 0u), 0u)).x,
                       indirect_radiance_tex.Load(int3(half_pixel + uint2(1u * dims.x, 0u), 0u)).x,
                       indirect_radiance_tex.Load(int3(half_pixel + uint2(2u * dims.x, 0u), 0u)).x);
    return ir;
}

#endif // TINYGLADE_DEFERRED_LIGHT_SUN_SKY_INC_HLSL
