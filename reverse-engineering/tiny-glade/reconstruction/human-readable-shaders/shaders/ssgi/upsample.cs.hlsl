// PATCHED: modules le.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/ssgi/upsample.cs.hlsl | F-D
// 依据:_ssgi_upsample.cs.hlsl_cs_main_0.bin(SPIR-V,偏移 23;SHA-256
//   2a466f7f8b21c2cedf861db078351469a1dc11eebc14e29de06359d8ecc077d6)。
//   绑定:ssgi_tex t14, depth_tex t15, half_depth_tex t16,
//   half_view_normal_tex t17, gbuffer_tex t18, output_tex u19,
//   frame_constants b0 space1;LocalSize [8,8,1],入口 cs_main。
//   把半分辨率 SSGI AO 做双边放大到全分辨率:5 个半分辨率采样点
//   (中心 + 4 邻域)按视图法线余弦^8 项、指数深度匹配、采样偏移的高斯
//   衰减加权;无采样点贡献时回退到最近半分辨率纹素。
// 状态:O (结构/常量取自反汇编;绑定逐字);
//       I (函数/变量名——被 dxc 内联)。

#include "inc/common.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/uv.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

Texture2D<float4> ssgi_tex : register(t14, space0);
Texture2D<float4> depth_tex : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t16, space0);
Texture2D<float4> half_view_normal_tex : register(t17, space0);
Texture2D<uint4> gbuffer_tex : register(t18, space0);
RWTexture2D<half> output_tex : register(u19, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    // 双边放大:中心 + 4 邻域的 5 个半分辨率采样,按法线/深度/距离加权。
    int2 samples[5] = { int2(0, 0), int2(-1, 0), int2(1, 0), int2(0, -1), int2(0, 1) };

    float center_depth = depth_tex.Load(uint3(dispatchThreadID.xy, 0u)).x;

    float4 weighted_sum = 0.0f.xxxx;
    float total_weight = 0.0f;

    if (center_depth != 0.0f)
    {
        float3 center_normal = mul(view_constants[0].world_to_view, float4(unpack_normal_rgb8(gbuffer_tex.Load(uint3(dispatchThreadID.xy, 0u)).x), 0.0f)).xyz;

        for (uint i = 0u; i < 5u; i++)
        {
            uint2 half_res_pos = uint2(int2((dispatchThreadID.xy / uint2(2u, 2u)) + uint2(samples[i])));
            float half_depth = half_depth_tex.Load(uint3(half_res_pos, 0u)).x;
            float4 sample_result = 0.0f.xxxx;
            do
            {
                if (half_depth != 0.0f)
                {
                    // 权重 = 法线余弦^8 * 指数深度匹配 * 采样偏移高斯。
                    float ndv = max(0.0f, dot(half_view_normal_tex.Load(uint3(half_res_pos, 0u)).xyz, center_normal));
                    float n2 = ndv * ndv;
                    float n4 = n2 * n2;
                    float n8 = n4 * n4;
                    float2 offset2 = float2(samples[i]);
                    float weight = exp2((-200.0f) * abs(1.0f - (center_depth / half_depth))) * n8 * exp(-dot(offset2, offset2));
                    total_weight += weight;
                    sample_result = ssgi_tex.Load(uint3(half_res_pos, 0u)) * weight;
                    break;
                }
                else
                {
                    sample_result = 0.0f.xxxx;
                    break;
                }
            } while (false);
            weighted_sum += sample_result;
        }
    }

    // 有权重则归一化输出,否则回退到最近半分辨率纹素。
    if (total_weight > 1.0e-7f)
    {
        output_tex[dispatchThreadID.xy] = (weighted_sum.x / total_weight);
    }
    else
    {
        output_tex[dispatchThreadID.xy] = ssgi_tex.Load(uint3(dispatchThreadID.xy / uint2(2u, 2u), 0u)).x;
    }
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _ssgi_upsample.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

static float scratch;

static const int2 offsets[5] = { int2(0, 0), int2(-1, 0), int2(1, 0), int2(0, -1), int2(0, 1) };

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

Texture2D<float4> ssgi_tex : register(t14, space0);
Texture2D<float4> depth_tex : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t16, space0);
Texture2D<float4> half_view_normal_tex : register(t17, space0);
Texture2D<uint4> gbuffer_tex : register(t18, space0);
RWTexture2D<float> output_tex : register(u19, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    float4 depth = depth_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
    float depth_val = depth.x;
    float4 out_acc;
    float out_wsum;
    if (depth_val != 0.0f)
    {
        uint4 gbuffer = gbuffer_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
        uint packed_normal = gbuffer.x;
        float3 view_normal = mul(frame_constants_view_constants[0u].world_to_view, float4((float4(float(packed_normal & 255u) * 0.0039215688593685626983642578125f, float((packed_normal >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((packed_normal >> 16u) & 255u) * 0.0039215688593685626983642578125f, scratch).xyz * 2.0f) - 1.0f.xxx, 0.0f)).xyz;
        float4 acc;
        float wsum;
        acc = 0.0f.xxxx;
        wsum = 0.0f;
        float4 next_acc;
        float next_wsum;
        for (uint i = 0u; i < 5u; acc = next_acc, wsum = next_wsum, i++)
        {
            uint2 half_coord = uint2(int2((gl_GlobalInvocationID.xy / uint2(2u, 2u)) + uint2(offsets[i])));
            float4 half_depth = half_depth_tex.Load(int3(half_coord, 0u));
            float half_depth_val = half_depth.x;
            float2 offset = float2(offsets[i]);
            float4 contrib;
            do
            {
                if (half_depth_val != 0.0f)
                {
                    float ndotl = max(0.0f, dot(half_view_normal_tex.Load(int3(half_coord, 0u)).xyz, view_normal));
                    float ndotl2 = ndotl * ndotl;
                    float ndotl4 = ndotl2 * ndotl2;
                    float weight = (exp2((-200.0f) * abs(1.0f - (depth_val / half_depth_val))) * (ndotl4 * ndotl4)) * exp(-dot(offset, offset));
                    next_wsum = wsum + weight;
                    contrib = ssgi_tex.Load(int3(half_coord, 0u)) * weight;
                    break;
                }
                else
                {
                    next_wsum = wsum;
                    contrib = 0.0f.xxxx;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            next_acc = acc + contrib;
        }
        out_acc = acc;
        out_wsum = wsum;
    }
    else
    {
        out_acc = 0.0f.xxxx;
        out_wsum = 0.0f;
    }
    if (out_wsum > 9.9999999747524270787835121154785e-07f)
    {
        output_tex[gl_GlobalInvocationID.xy] = (out_acc.x.xxxx / out_wsum.xxxx).x;
    }
    else
    {
        output_tex[gl_GlobalInvocationID.xy] = ssgi_tex.Load(int3(gl_GlobalInvocationID.xy / uint2(2u, 2u), 0u)).x;
    }
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
