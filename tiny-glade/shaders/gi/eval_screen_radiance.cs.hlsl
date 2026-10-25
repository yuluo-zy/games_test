// 全局光照:屏幕辐射度求值(EVAL)。
// 8 螺旋半分辨率 GI 屏幕辐射度求值:重建世界法线,在吸附到半分辨率的候选
// 位置采样候选光线/辐射度,按深度/ssgi/光线方向一致度加权,把辐射度投影到
// SH 带 1(Y00/Y1x..),累加三个 SH 向量切片。reservoir 的高半段 .y 为辐射度权重。
// 绑定:$Globals b13{8 个纹理尺寸},blue_noise t14,half_depth t15,half_normal t16,
// ssgi t17,restir_radiance t18,restir_ray_dir t19,restir_reservoir t20,output u21,
// frame_constants set1/b0。LocalSize [8,8,1]。

#include "inc/common.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/reservoir.hlsl"
#include "inc/hash.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/halfres.hlsl"
#include "inc/math.hlsl"
#include "inc/sh.hlsl"
#include "gi/restir.inc.hlsl"
#include "inc/blue_noise.hlsl"
#include "inc/hammersley.hlsl"
#include "gi/gi_settings.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 blue_noise_texture_dims;
    uint2 half_depth_tex_dims;
    uint2 half_view_normal_tex_dims;
    uint2 ssgi_tex_dims;
    uint2 restir_radiance_tex_dims;
    uint2 restir_ray_dir_tex_dims;
    uint2 restir_reservoir_tex_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> half_depth_tex : register(t15, space0);
Texture2D<float4> half_view_normal_tex : register(t16, space0);
Texture2D<float4> ssgi_tex : register(t17, space0);
Texture2D<float4> restir_radiance_tex : register(t18, space0);
Texture2D<float4> restir_ray_dir_tex : register(t19, space0);
Texture2D<uint4> restir_reservoir_tex : register(t20, space0);
RWTexture2D<float4> output_tex : register(u21, space0);

float2 unpack_half2x16(uint v)
{
    return f16tof32(uint2(v & 0xffffu, v >> 16u));
}

[numthreads(8, 8, 1)]
void cs_main(uint3 globalThreadID : SV_DispatchThreadID)
{
    uint2 xy = globalThreadID.xy;

    // 蓝色噪声:螺旋起始角与半径偏移(帧相关,防闪烁)。
    float4 bn = blue_noise(blue_noise_texture, xy, 71u + frame_index);

    float depth = half_depth_tex.Load(int3(xy, 0u)).x;
    uint sub = frame_index & 3u;
    // 半分辨率 ssgi,用亚像素抖动对齐全分辨率纹素。
    float ssgi = ssgi_tex.Load(int3((xy * uint2(2u, 2u)) + hi_px_subpixels[sub], 0u)).x;
    // 半分辨率法线转到世界空间,并用 .z 作深度-法线权重下限。
    float3 world_n = mul(view_constants[0u].view_to_world, float4(half_view_normal_tex.Load(int3(xy, 0u)).xyz, 0.0f)).xyz;
    float normal_z = max(0.300000011920928955078125f, half_view_normal_tex.Load(int3(xy, 0u)).z);

    // 三个 SH 向量切片的辐射度累加器。
    float4 acc[3];
    acc[0] = float4(0.0f, 0.0f, 0.0f, 0.0f);
    acc[1] = float4(0.0f, 0.0f, 0.0f, 0.0f);
    acc[2] = float4(0.0f, 0.0f, 0.0f, 0.0f);
    float total = 0.0f;

    // 8 螺旋候选:围绕当前像素按黄金角旋转展开,并吸附到半分辨率网格。
    for (uint i = 0u; i < 8u; ++i) {
        float angle = (float(i) + bn.x) * 2.3999631404876708984375f;
        uint2 sp = uint2((float2(xy) + 0.5f.xx) + (((float2(cos(angle), sin(angle)) * sqrt(float(i) + bn.y)) * 0.3535533845424652099609375f) * 8.0f) * 2.0f);
        sp -= (sp % uint2(2u, 2u));

        // 候选权重:深度一致 × ssgi 一致 × 法线朝光线方向(背光候选剔除)。
        float w = exp2(-min(30.0f, 80.0f * (normal_z * abs(1.0f - (half_depth_tex.Load(int3(sp, 0u)).x / depth)))))
                * exp2((-20.0f) * abs(ssgi_tex.Load(int3((sp * uint2(2u, 2u)) + hi_px_subpixels[sub], 0u)).x - ssgi))
                * float(int(dot(world_n, restir_ray_dir_tex.Load(int3(sp, 0u)).xyz) > 0.0f));

        // 辐射度权重来自 reservoir 高半段的 half16 值。
        float rad_w = max(0.0f, unpack_half2x16((restir_reservoir_tex.Load(int3(sp, 0u)).y >> 16u) & 0xffffu).x);
        float3 rad = restir_radiance_tex.Load(int3(sp, 0u)).xyz * rad_w * w;
        // SH 带 1 基函数:Y00 常数项 + Y1x/Y1y/Y1z 线性项。
        float4 sh = float4(0.2820949852466583251953125f, restir_ray_dir_tex.Load(int3(sp, 0u)).xyz * 0.48860299587249755859375f) * 4.0f;
        acc[0] += sh * rad.x;
        acc[1] += sh * rad.y;
        acc[2] += sh * rad.z;
        total += w;
    }

    // 归一化(0.5 * 1/total,对空累计有最小下限)。
    float factor = 0.5f / max(9.9999997473787516355514526367188e-06f, total);
    acc[0] *= factor;
    acc[1] *= factor;
    acc[2] *= factor;

    // 三个 SH 切片按半宽布局写入输出。
    for (uint c = 0u; c < 3u; ++c) {
        output_tex[xy + uint2(c * half_depth_tex_dims.x, 0u)] = acc[c];
    }
}
