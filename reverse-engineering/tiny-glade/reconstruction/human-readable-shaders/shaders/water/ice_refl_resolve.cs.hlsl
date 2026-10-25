// PATCHED: modules ve.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.

// 冰面反射解析:8 螺旋采样聚集反射命中/辐射度,按粗糙度与射线长度加权,
// 用 FrexpStructType 量化,做历史帧混合并输出射线长度。
// 转译段(TGV_0,变量为 SPIR-V 编号)已语义化命名;逻辑未改。
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/water/ice_refl_resolve.cs.hlsl
// SOURCE PATH : shaders/water/ice_refl_resolve.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins): _water_ice_refl_resolve.cs.hlsl_cs_main_0.bin
//   entry cs:cs_main | local_size [8,8,1] | index_pass W45 (water/ice_surface_pass)
// Status: O (bindings/entry/LocalSize/caps; body transcribed from spirv-cross)
// Evidence: spirv-cross — ice reflection resolve: 8-tap spiral gather over
//   refl_hit_tex/refl_hit_radiance_tex with roughness/ray-length matched weights,
//   FrexpStructType quantization, history blend and ray-length output.
// MERGE (water-ice review water/ice_refl_resolve.cs.hlsl): A adopted wholesale.
//   The bin carries `FrexpStructType` (OpName) and its include closure has
//   ice_shared.inc but NOT inc/curve.hlsl — so the inc/curve.hlsl include is
//   dropped here and FrexpStructType + the frexp workaround are sourced from
//   water/ice_shared.inc.hlsl (see review inc-fix #6).

#include "inc/common.hlsl"
#include "inc/samplers.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/brdf_lut.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/history_valid.hlsl"
#include "ice_shared.inc.hlsl"
#include "inc/hammersley.hlsl"
// 注:inc/blue_noise.hlsl 不再声明 blue_noise_texture(按 inc-fix #3 移除 register);
// 冰面家族在 t15 绑定并在本地声明。

cbuffer $Globals : register(b13, space0)
{
    uint2 brdf_fg_lut_dims;
    uint history_valid;
    uint2 blue_noise_texture_dims;
    uint2 depth_tex_dims;
    uint2 ice_roughness_tex_dims;
    uint2 refl_hit_tex_dims;
    uint2 refl_hit_radiance_tex_dims;
    uint2 color_tex_dims;
    uint2 refl_refr_radiance_history_tex_dims;
    uint2 refl_refr_radiance_out_tex_dims;
    uint2 ray_len_history_tex_dims;
    uint2 ray_len_out_tex_dims;
    uint2 reprojection_tex_dims;
};

Texture2D<float4> blue_noise_texture : register(t15, space0);
Texture2D<float4> depth_tex : register(t16, space0);
Texture2D<float4> ice_roughness_tex : register(t17, space0);
Texture2D<float4> refl_hit_tex : register(t18, space0);
Texture2D<float4> refl_hit_radiance_tex : register(t19, space0);
RWTexture2D<float3> color_tex : register(u20, space0);
Texture2D<float4> refl_refr_radiance_history_tex : register(t21, space0);
[[vk::image_format("r11g11b10f")]] RWTexture2D<float3> refl_refr_radiance_out_tex : register(u22, space0);
Texture2D<float4> ray_len_history_tex : register(t23, space0);
[[vk::image_format("rg16f")]] RWTexture2D<float2> ray_len_out_tex : register(u24, space0);

// 切空间坐标交换 (x, y, z) -> (x, -z, y);见 ice_refr_resolve。
float3 swap_yz(float3 v)
{
    return mul(v, float3x3(float3(1.0f, -0.0f, 0.0f),
                          float3(-0.0f, 0.0f, 1.0f),
                          float3(-0.0f, -1.0f, 0.0f)));
}

[numthreads(8, 8, 1)]
void cs_main(uint3 gtid : SV_DispatchThreadID)
{
    if (ice_roughness_tex.Load(int3(gtid.xy, 0u)).x == 1.0f)
    {
        refl_refr_radiance_out_tex[gtid.xy] = 0.0f;
        return;
    }

    float2 uv = (float2(int2(gtid.xy)) + 0.5f) / float2(depth_tex_dims);
    float4 bn = ((blue_noise_texture.Load(int3(
        (gtid.xy + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
            * float(frame_index & 255u)) + 0.5f) * 256.0f)) % uint2(256u, 256u), 0u)) * 255.0f)
        * 0.00390625f) + 0.001953125f;

    float2 ndc = (uv - 0.5f) * float2(2.0f, -2.0f);
    float4 ray_vs = mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f));
    float3 view_ray = normalize(mul(view_constants[0].view_to_world, ray_vs).xyz);
    float4 pos_vs = float4(ndc, depth_tex.Load(int3(gtid.xy, 0u)).x, 1.0f);
    float4 wp4 = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, pos_vs));
    float3 world_pos = wp4.xyz / wp4.w;

    float4 refl_hit = refl_hit_tex.Load(int3(gtid.xy / uint2(2u, 2u), 0u));
    float roughness = ice_roughness_tex.Load(int3(gtid.xy, 0u)).x;
    float rough2 = max(0.000600000028498470783233642578125f, roughness * roughness);

    float3 view_dir = swap_yz(-view_ray);
    float3 view_dir_fix;
    if (view_dir.z < 0.0f)
    {
        view_dir_fix = normalize(float3(view_dir.x, view_dir.y, view_dir.z * (-0.25f)));
    }
    else
    {
        view_dir_fix = view_dir;
    }

    uint quad_id = ((gtid.x & 1u) | ((gtid.y & 1u) * 2u)) & 3u;
    float dist = length(mul(view_constants[0].world_to_view, float4(world_pos, 1.0f)).xyz);
    float vdotn = -normalize(ray_vs.xyz).z;
    float inv_dist = max(9.9999997473787516355514526367188e-06f, dist);
    float ray_scale = 16.0f / inv_dist;

    float4 prev = mul(view_constants[0].clip_to_prev_clip,
        mul(view_constants[0].view_to_clip, mul(view_constants[0].clip_to_view, pos_vs)));
    float2 prev_uv = ((prev.xy / prev.w) * float2(0.5f, -0.5f)) + 0.5f;
    float2 prev_uv_clamped = clamp(prev_uv, 0.0f, 1.0f);
    float in_view = float(all(prev_uv_clamped == prev_uv));
    float2 reproj_uv = uv + (prev_uv - ((pos_vs.xy * float2(0.5f, -0.5f)) + 0.5f));

    float cur_exp = exp2(-clamp(ray_scale * refl_hit.x, 0.0f, 100.0f));
    float ray_log = (-0.0625f) * inv_dist;
    float ray_len = max(0.0f, ray_log * log2(1.0000000031710768509710513471353e-30f
        + lerp(exp2(-clamp(ray_scale * ray_len_history_tex.SampleLevel(sampler_lnc, reproj_uv, 0.0f).y, 0.0f, 100.0f)),
            cur_exp, 0.100000001490116119384765625f)));
    float ray_len_f = max(ray_len,
        (((dist / vdotn) * view_constants[0].clip_to_view[1].y) * 0.20000000298023223876953125f)
            * smoothstep(0.0f, 0.0500000007450580596923828125f * dist, ray_len));
    float proj_scale = (dist * vdotn) * view_constants[0].clip_to_view[1].y;

    float3 roughness3 = ((1.0f - rough2) * (sqrt(1.0f - rough2) + rough2)).xxx;
    float3 refl_dir = reflect(-normalize(lerp(float3(0.0f, 1.0f, 0.0f),
        reflect(view_ray, float3(0.0f, 1.0f, 0.0f)), roughness3)), float3(0.0f, 1.0f, 0.0f));
    float3 binorm = normalize(cross(float3(0.0f, 1.0f, 0.0f), refl_dir))
        * max(min((((ray_len_f / (ray_len_f + dist)) * dist) * vdotn) * (sqrt(rough2) * 0.25f),
            0.100000001490116119384765625f * proj_scale),
            (4.0f / float(color_tex_dims.y)) * proj_scale);
    float3 tangent = cross(refl_dir, binorm);

    float golden = float((frame_index * 59u) % 128u) * 1.324717998504638671875f;

    float4 acc = 0.0f;
    float acc_w = 0.0f;
    float t = 0.0f;
    for (uint i = 0u; i < 8u; ++i)
    {
        float angle = ((float(i) + golden) * 2.3999631404876708984375f) + (float(quad_id) * 1.57079637050628662109375f);
        float4 s4 = mul(view_constants[0].view_to_sample,
            mul(view_constants[0].world_to_view, float4(
                world_pos + ((binorm * cos(angle)) + (tangent * sin(angle))) * (pow(t + bn.y, 1.0f) * 0.125f),
                1.0f)));
        uint2 px = uint2(int2(floor((((s4.xyz / s4.w).xy * float2(0.5f, -0.5f)) + 0.5f) * float2(refl_hit_tex_dims))));
        float4 rh = refl_hit_tex.Load(int3(px, 0u));
        if (rh.y != 0.0f)
        {
            float w = (9.9999999747524270787835121154785e-07f
                * exp((-1.0f) * abs((4.0f / (1.0f + (ray_len * ray_len))) - (4.0f / (1.0f + (rh.x * rh.x))))))
                * (9.9999999747524270787835121154785e-07f + clamp(1.0f - (8.0f * abs(refl_hit.y - rh.y)), 0.0f, 1.0f));
            acc += float4(refl_hit_radiance_tex.Load(int3(px, 0u)).xyz, 1.0f) * w;
            acc_w += cur_exp * w;
        }
        t += 1.0f;
    }

    float acc_norm = max(9.9999998245167004418121337039338e-15f, acc.w);
    float3 refl_dir2 = normalize(lerp(float3(0.0f, 0.0f, 1.0f),
        reflect(-view_dir_fix, float3(0.0f, 0.0f, 1.0f)), roughness3));
    float3 radiance = ((acc.xyz / acc_norm) * lerp(0.100000001490116119384765625f, 1.0f,
        exp2((-1.0f) - (8.656169891357421875f * dot(refl_dir2, view_dir_fix + refl_dir2)))))
        * (1.0f + rough2)
        + refl_refr_radiance_out_tex[gtid.xy].xyz;

    float3 blended;
    if (history_valid != 0u && frame_index != 0u)
    {
        float3 hist = refl_refr_radiance_history_tex.SampleLevel(sampler_lnc, reproj_uv, 0.0f).xyz;
        if (all(hist > 0.0f))
        {
            blended = lerp(hist, radiance, lerp(1.0f, 0.25f, in_view));
        }
        else
        {
            blended = radiance;
        }
    }
    else
    {
        blended = radiance;
    }

    // FrexpStructType + frexp come from water/ice_shared.inc.hlsl (bin name/closure;
    // inc/curve.hlsl is NOT in the bin closure for this consumer).
    FrexpStructType f;
    f.mantissa = frexp(blended, f.exponent);
    float3 quant = blended + exp2(float3(f.exponent) - float3(8.0f, 8.0f, 7.0f));
    refl_refr_radiance_out_tex[gtid.xy] = quant;
    color_tex[gtid.xy] = color_tex[gtid.xy].xyz + blended;
    ray_len_out_tex[gtid.xy] = float2(max(0.0f, ray_log * log2(1.0000000031710768509710513471353e-30f + (acc_w / acc_norm))), ray_len);
}

#endif


#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 转译段:与手写段相同算法的 spirv-cross 输出(语义化命名)。 && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _water_ice_refl_resolve.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
// dxc 1.9/1.10 会把原生 frexp() 降级为非法 SPIR-V(对非 int 做 OpConvertSToF);
// 与 inc/curve.hlsl 相同的工作区。
float3 incb_frexp(float3 x, out int3 exponent)
{
    // 次正规数先归一(x * 2^24),使指数/尾数与 GLSL Frexp 完全一致
    uint3 bits0 = asuint(x);
    bool3 denorm = ((bits0 & 0x7F800000u) == 0u) & ((bits0 & 0x7FFFFFFFu) != 0u);
    float3 xs = select(denorm, x * 16777216.0f, x);
    uint3 bits = asuint(xs);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu), int((bits.y >> 23u) & 0xFFu), int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127) - select(denorm, int3(24, 24, 24), int3(0, 0, 0));
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, xs, m);
}
#define frexp incb_frexp
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

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

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
    uint2 brdf_fg_lut_dims : packoffset(c0);
    uint history_valid : packoffset(c0.z);
    uint2 blue_noise_texture_dims : packoffset(c1);
    uint2 depth_tex_dims : packoffset(c1.z);
    uint2 ice_roughness_tex_dims : packoffset(c2);
    uint2 refl_hit_tex_dims : packoffset(c2.z);
    uint2 refl_hit_radiance_tex_dims : packoffset(c3);
    uint2 color_tex_dims : packoffset(c3.z);
    uint2 refl_refr_radiance_history_tex_dims : packoffset(c4);
    uint2 refl_refr_radiance_out_tex_dims : packoffset(c4.z);
    uint2 ray_len_history_tex_dims : packoffset(c5);
    uint2 ray_len_out_tex_dims : packoffset(c5.z);
    uint2 reprojection_tex_dims : packoffset(c6);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> blue_noise_texture : register(t15, space0);
Texture2D<float4> depth_tex : register(t16, space0);
Texture2D<float4> ice_roughness_tex : register(t17, space0);
Texture2D<float4> refl_hit_tex : register(t18, space0);
Texture2D<float4> refl_hit_radiance_tex : register(t19, space0);
RWTexture2D<float3> color_tex : register(u20, space0);
Texture2D<float4> refl_refr_radiance_history_tex : register(t21, space0);
RWTexture2D<float3> refl_refr_radiance_out_tex : register(u22, space0);
Texture2D<float4> ray_len_history_tex : register(t23, space0);
RWTexture2D<float2> ray_len_out_tex : register(u24, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        if (ice_roughness_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).x == 1.0f)
        {
            refl_refr_radiance_out_tex[gl_GlobalInvocationID.xy] = 0.0f.xxx;
            break;
        }
        float2 uv = (float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx) / float2(depth_tex_dims);
        float4 bn = ((blue_noise_texture.Load(int3((gl_GlobalInvocationID.xy + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_constants_frame_index & 255u)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
        float2 ndc = (uv - 0.5f.xx) * float2(2.0f, -2.0f);
        float ndc_x = ndc.x;
        float4 ray_vs = mul(frame_constants_view_constants[0u].sample_to_view, float4(ndc_x, ndc.y, 0.0f, 1.0f));
        float4 pos_vs = float4(ndc_x, ndc.y, depth_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, 1.0f);
        float4 wp4 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, pos_vs));
        float3 world_pos = wp4.xyz / wp4.w.xxx;
        float4 refl_hit = refl_hit_tex.Load(int3(gl_GlobalInvocationID.xy / uint2(2u, 2u), 0u));
        float refl_hit_y = refl_hit.y;
        float4 roughness_tex = ice_roughness_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
        float roughness = roughness_tex.x;
        float rough2 = max(0.000600000028498470783233642578125f, roughness * roughness);
        float3 view_ray = normalize(normalize(mul(frame_constants_view_constants[0u].view_to_world, ray_vs).xyz));
        float3 view_dir = mul(-view_ray, float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
        float view_dir_z = view_dir.z;
        float3 view_dir_fix;
        if (view_dir_z < 0.0f)
        {
            float3 view_dir_fix_pre = view_dir;
            view_dir_fix_pre.z = view_dir_z * (-0.25f);
            view_dir_fix = normalize(view_dir_fix_pre);
        }
        else
        {
            view_dir_fix = view_dir;
        }
        uint quad_id = ((gl_GlobalInvocationID.x & 1u) | ((gl_GlobalInvocationID.y & 1u) * 2u)) & 3u;
        float dist = length(mul(frame_constants_view_constants[0u].world_to_view, float4(world_pos, 1.0f)).xyz);
        float vdotn = -normalize(ray_vs.xyz).z;
        float inv_dist = max(9.9999997473787516355514526367188e-06f, dist);
        float ray_scale = 16.0f / inv_dist;
        float4 prev = mul(frame_constants_view_constants[0u].clip_to_prev_clip, mul(frame_constants_view_constants[0u].view_to_clip, mul(frame_constants_view_constants[0u].clip_to_view, pos_vs)));
        float2 prev_uv = ((prev.xy / prev.w.xx) * float2(0.5f, -0.5f)) + 0.5f.xx;
        float2 prev_uv_clamped = clamp(prev_uv, 0.0f.xx, 1.0f.xx);
        float in_view = float(all(bool2(prev_uv_clamped.x == prev_uv.x, prev_uv_clamped.y == prev_uv.y)));
        float2 reproj_uv = uv + float4(prev_uv - ((pos_vs.xy * float2(0.5f, -0.5f)) + 0.5f.xx), in_view, 1.0f).xy;
        float cur_exp = exp2(-clamp(ray_scale * length(refl_hit.x), 0.0f, 100.0f));
        float ray_log = (-0.0625f) * inv_dist;
        float ray_len = max(0.0f, ray_log * log2(1.0000000031710768509710513471353e-30f + lerp(exp2(-clamp(ray_scale * ray_len_history_tex.SampleLevel(sampler_lnc, reproj_uv, 0.0f).y, 0.0f, 100.0f)), cur_exp, 0.100000001490116119384765625f)));
        float ray_len_f = max(ray_len, (((dist / vdotn) * frame_constants_view_constants[0u].clip_to_view[1].y) * 0.20000000298023223876953125f) * smoothstep(0.0f, 0.0500000007450580596923828125f * dist, ray_len));
        float proj_scale = (dist * vdotn) * frame_constants_view_constants[0u].clip_to_view[1].y;
        float one_minus_rough2 = 1.0f - rough2;
        float3 roughness3 = (one_minus_rough2 * (sqrt(one_minus_rough2) + rough2)).xxx;
        float3 refl_dir = reflect(-normalize(lerp(float3(0.0f, 1.0f, 0.0f), reflect(view_ray, float3(0.0f, 1.0f, 0.0f)), roughness3)), float3(0.0f, 1.0f, 0.0f));
        float3 binorm = normalize(cross(float3(0.0f, 1.0f, 0.0f), refl_dir)) * max(min((((ray_len_f / (ray_len_f + dist)) * dist) * vdotn) * (sqrt(rough2) * 0.25f), 0.100000001490116119384765625f * proj_scale), (4.0f / float(color_tex_dims.y)) * proj_scale);
        float3 tangent = cross(refl_dir, binorm);
        float golden = float((frame_constants_frame_index * 59u) % 128u) * 1.324717998504638671875f;
        float4 acc;
        float acc_w;
        acc = 0.0f.xxxx;
        acc_w = 0.0f;
        float4 next_acc;
        float next_acc_w;
        float t = 0.0f;
        int i = 0;
        for (; uint(i) < 8u; acc = next_acc, t += 1.0f, acc_w = next_acc_w, i++)
        {
            float angle = ((float(i) + golden) * 2.3999631404876708984375f) + (float(quad_id) * 1.57079637050628662109375f);
            float4 s4 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(world_pos + (((binorm * cos(angle)) + (tangent * sin(angle))) * (pow(t + bn.y, 1.0f) * 0.125f)), 1.0f)));
            uint2 px = uint2(int2(floor((((s4.xyz / s4.w.xxx).xy * float2(0.5f, -0.5f)) + 0.5f.xx) * float2(refl_hit_tex_dims))));
            float4 rh = refl_hit_tex.Load(int3(px, 0u));
            float rh_x = rh.x;
            float rh_y = rh.y;
            if (rh_y != 0.0f)
            {
                float w = (9.9999999747524270787835121154785e-07f * exp((-1.0f) * abs((4.0f / (1.0f + (ray_len * ray_len))) - (4.0f / (1.0f + (rh_x * rh_x)))))) * (9.9999999747524270787835121154785e-07f + clamp(1.0f - (8.0f * abs(refl_hit_y - rh_y)), 0.0f, 1.0f));
                next_acc = acc + (float4(refl_hit_radiance_tex.Load(int3(px, 0u)).xyz, 1.0f) * w);
                next_acc_w = acc_w + (cur_exp * w);
            }
            else
            {
                next_acc = acc;
                next_acc_w = acc_w;
            }
        }
        float acc_norm = max(9.9999998245167004418121337039338e-15f, acc.w);
        float3 refl_dir2 = normalize(lerp(float3(0.0f, 0.0f, 1.0f), reflect(-view_dir_fix, float3(0.0f, 0.0f, 1.0f)), roughness3));
        float3 radiance = (((acc.xyz / acc_norm.xxx).xyz * lerp(0.100000001490116119384765625f, 1.0f, exp2((-1.0f) - (8.656169891357421875f * dot(refl_dir2, view_dir_fix + refl_dir2))))) * (1.0f + rough2)) + refl_refr_radiance_out_tex[gl_GlobalInvocationID.xy].xyzz.xyz;
        bool use_history;
        if (history_valid != 0u)
        {
            use_history = !(0u == frame_constants_frame_index);
        }
        else
        {
            use_history = false;
        }
        float3 blended;
        if (use_history)
        {
            float3 hist = refl_refr_radiance_history_tex.SampleLevel(sampler_lnc, reproj_uv, 0.0f).xyz;
            float3 hist_lerped;
            if (all(bool3(hist.x > 0.0f.xxx.x, hist.y > 0.0f.xxx.y, hist.z > 0.0f.xxx.z)))
            {
                hist_lerped = lerp(hist, radiance, lerp(1.0f, 0.25f, in_view).xxx);
            }
            else
            {
                hist_lerped = radiance;
            }
            blended = hist_lerped;
        }
        else
        {
            blended = radiance;
        }
        FrexpStructType f;
        { int3 e_tmp; f.mantissa = frexp(blended, e_tmp); f.exponent = e_tmp; }
        refl_refr_radiance_out_tex[gl_GlobalInvocationID.xy] = blended + exp2(float3(f.exponent) - float3(8.0f, 8.0f, 7.0f));
        color_tex[gl_GlobalInvocationID.xy] = color_tex[gl_GlobalInvocationID.xy].xyzz.xyz + blended;
        ray_len_out_tex[gl_GlobalInvocationID.xy] = float2(max(0.0f, ray_log * log2(1.0000000031710768509710513471353e-30f + (acc_w / acc_norm))), ray_len);
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
