// ==RECONSTRUCTED== shaders/clearing/distant_billboard.raster.hlsl
// 【文件用途】远处山丘(hills)广告板。渲染远景山丘轮廓的两个斜面(y = 12 两侧)。
// 【证据】_clearing_distant_billboard.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin。
//   绑定:sampler_default s9、distant_hills_texture t14、$Globals b13
//   (current_view_index / distant_hills_texture_dims / model_to_world)、frame_constants set1/b0。
//   vs_main 经 model_to_world 把远处山丘四边形(两个绕 y=12 的斜面)缩放到世界空间;
//   ps_main 带 mip 偏移采样山丘纹理,丢弃 alpha > 0.99 的片元,并按天空色/昼夜时刻染色。
// 【状态】O(结构逐字)。
// 重写说明:手写重建段,仅补充中文注释,代码与算术逐字保留。

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/color_library.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/math.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"

// $Globals 成员(隐式 cbuffer,set0/b13)。
uint2 distant_hills_texture_dims;
float4x4 model_to_world;

Texture2D<float4> distant_hills_texture : register(t14, space0);

struct VS_IN
{
    [[vk::location(0)]] float3 vertex_position: Vertex_Position;
    [[vk::location(1)]] float2 vertex_uv: Vertex_UV;
};

struct VS_OUT
{
    float2 C0 : TEXCOORD0;  // 采样的 UV(带重复条纹)
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i)
{
    VS_OUT o;
    float3 pos = i.vertex_position;
    // UV 缩放(近似 5/7,用于山丘纹理横向平铺)
    float2 uv = i.vertex_uv * 0.71428573131561279296875f;
    float3 p;
    // 两个斜面:正 x 一侧斜度与下降量不同(1.5 倍宽 / -74),负 x 一侧 1.45 倍 / -80
    if (pos.x > 0.0f)
    {
        float3 pp = pos;
        pp.y = (pos.y * 0.675f) - 74.0f;
        float2 pp2 = pp.xz * 1.5f;
        p = float3(pp2.x, pp.y, pp2.y);
    }
    else
    {
        float3 pp = pos;
        pp.y = (pos.y * 0.675f) - 80.0f;
        float2 pp2 = pp.xz * 1.45f;
        p = float3(pp2.x, pp.y, pp2.y);
    }
    // 模型空间(抬升 12 世界单位)经 model_to_world 到世界,再投影
    o.position = mul(view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(p.x, p.y + 12.0f, p.z, 1.0f)));
    // 水平 UV 重复 19 份并偏移 0.7,垂直翻转
    o.C0 = float2(frac((uv.x * 19.0f) + 0.7f), 1.0f - uv.y);
    return o;
}

struct PS_IN
{
    float2 C0 : TEXCOORD0;
};

struct PS_OUT
{
    float4 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    // 带全局 mip 偏移采样山丘纹理
    float4 tex = distant_hills_texture.SampleBias(sampler_default, i.C0, global_mip_bias);
    // alpha > 0.99 视为空片元,丢弃
    if (tex.x > 0.99f)
    {
        discard;
    }
    // 反 gamma:sqrt 提亮暗部;按纹理红通道在天空高/低色之间混合
    float3 s = sqrt(tex.xyz + 0.1f);
    float3 tint = (lerp(sky_high_color.xyz, sky_low_color.xyz * 0.5f, s.x.xxx) * 0.7f)
        // 昼夜时刻调整:日出/日落附近变暗
        * clamp(1.1f - ((smoothstep(0.483f, 0.505f, time_of_day) - smoothstep(0.84f, 0.91f, time_of_day)) * 0.75f), 0.0f, 1.0f);
    o.target = float4(s * tint, 1.0f);
    return o;
}
