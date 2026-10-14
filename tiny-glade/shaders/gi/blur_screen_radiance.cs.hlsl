// 全局光照:屏幕辐射度双边模糊(BLUR)。
// 对三切片 SH 辐射度做深度/法线/ssgi 一致的双边模糊;时间统计计数 >33 时
// 采样步长放大 3 倍(减少重投影静止区域的高频噪声)。行为取自 _gi_blur
// 屏幕辐射度 bin(spirv-cross 转译,语义化重命名,指令不变)。
// 绑定:$Globals b13,half_depth t14,half_normal t15,ssgi t16,input t17,
// output u18,temporal_stats_out u19,frame_constants set1/b0。LocalSize [8,8,1]。

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
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

// 三切片累加器的初始值(全零)。
static const float4 kZeroSlabs[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
// 半分辨率亚像素抖动偏移(逐帧相位)。
static const uint2 kSubpixelOffsets[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

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

cbuffer $Globals : register(b13, space0)
{
    uint2 half_depth_tex_dims : packoffset(c0);
    uint2 half_view_normal_tex_dims : packoffset(c0.z);
    uint2 ssgi_tex_dims : packoffset(c1);
    uint2 input_tex_dims : packoffset(c1.z);
    uint2 output_tex_dims : packoffset(c2);
    uint2 temporal_stats_out_tex_dims : packoffset(c2.z);
};

Texture2D<float4> half_depth_tex : register(t14, space0);
Texture2D<float4> half_view_normal_tex : register(t15, space0);
Texture2D<float4> ssgi_tex : register(t16, space0);
Texture2D<float4> input_tex : register(t17, space0);
RWTexture2D<float4> output_tex : register(u18, space0);
RWTexture2D<unorm float4> temporal_stats_out_tex : register(u19, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    // 本像素深度、法线 .z 下限、ssgi 值。
    float4 depth_px = half_depth_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
    float depth = depth_px.x;
    float normal_z = max(0.300000011920928955078125f, half_view_normal_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).z);
    // 三切片辐射度累加器。
    float4 acc[3] = kZeroSlabs;
    uint sub = frame_constants_frame_index & 3u;
    float4 ssgi_px = ssgi_tex.Load(int3((gl_GlobalInvocationID.xy * uint2(2u, 2u)) + kSubpixelOffsets[sub], 0u));
    float ssgi = ssgi_px.x;
    float3 normal = half_view_normal_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).xyz;
    // 时间统计计数 >33(即已稳定累计)时把采样步长放大到 3 纹素。
    float step_size = ((temporal_stats_out_tex[gl_GlobalInvocationID.xy] * float4(255.0f, 1.0f, 1.0f, 1.0f)).x > 33.0f) ? 1.0f : 3.0f;
    float wsum;
    wsum = 0.0f;
    float next_wsum;
    // 3x3 邻域双边权重累加。
    for (int j = -1; j <= 1; wsum = next_wsum, j++)
    {
        next_wsum = wsum;
        float inner_wsum;
        for (int i = -1; i <= 1; next_wsum = inner_wsum, i++)
        {
            uint2 off = gl_GlobalInvocationID.xy + uint2(int2(float2(int2(i, j)) * step_size));
            // 深度一致 × ssgi 一致 × 法线一致(带 0.9 衰减),得双边权重。
            float nd = dot(normal, half_view_normal_tex.Load(int3(off, 0u)).xyz);
            float normal_w = ((nd * nd) * (-0.89999997615814208984375f)) + 1.0f;
            float weight = (exp2(-min(30.0f, 80.0f * (normal_z * abs(1.0f - (half_depth_tex.Load(int3(off, 0u)).x / depth))))) * exp2((-20.0f) * abs(ssgi_tex.Load(int3((off * uint2(2u, 2u)) + kSubpixelOffsets[sub], 0u)).x - ssgi))) * (0.010000000707805156707763671875f / (normal_w * normal_w));
            // 加权累加三个 SH 切片。
            for (uint c0 = 0u; c0 < 3u; )
            {
                acc[c0] += (input_tex.Load(int3(off + uint2(c0 * half_depth_tex_dims.x, 0u), 0u)) * weight);
                c0++;
                continue;
            }
            inner_wsum = next_wsum + weight;
        }
    }
    // 按总权重归一化。
    for (uint c1 = 0u; c1 < 3u; )
    {
        acc[c1] /= max(1.0000000133514319600180897396058e-10f, wsum).xxxx;
        c1++;
        continue;
    }
    // 三切片按半宽布局写回。
    for (uint c2 = 0u; c2 < 3u; )
    {
        output_tex[gl_GlobalInvocationID.xy + uint2(c2 * half_depth_tex_dims.x, 0u)] = acc[c2];
        c2++;
        continue;
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
