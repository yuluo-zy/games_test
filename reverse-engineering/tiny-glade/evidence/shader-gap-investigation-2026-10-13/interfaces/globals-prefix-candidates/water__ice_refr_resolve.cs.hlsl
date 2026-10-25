// PATCHED: modules ve.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/water/ice_refr_resolve.cs.hlsl
// SOURCE PATH : shaders/water/ice_refr_resolve.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins): _water_ice_refr_resolve.cs.hlsl_cs_main_0.bin
//   entry cs:cs_main | local_size [8,8,1] | index_pass W45 (water/ice_surface_pass)
// Status: O (bindings/entry/LocalSize/caps; body transcribed from spirv-cross)
// Evidence: spirv-cross — refraction resolve: reads refr_hit_tex (uint4) packed
//   normal/depth, reconstructs world position, spirals 8 taps over refr_hit_tex
//   projecting world-space offset samples, weights by normal-aligned fresnel-ish
//   term (0.99437 quadratic in view normal z), writes normalized radiance to
//   refl_refr_radiance_out_tex.
// MERGE (water-ice review water/ice_refr_resolve.cs.hlsl): A adopted wholesale
//   (include closure matches the bin OpString list exactly; behavior equivalent).

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
#include "inc/color/srgb.hlsl"
#include "ice_shared.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims;
    uint2 refr_hit_tex_dims;
    uint2 refr_hit_radiance_tex_dims;
    uint2 refl_refr_radiance_out_tex_dims;
};

Texture2D<float4> depth_tex : register(t14, space0);
Texture2D<uint4> refr_hit_tex : register(t15, space0);
Texture2D<float4> refr_hit_radiance_tex : register(t16, space0);
// r11g11b10f storage format (target OpTypeImage fmt 8). NOTE: target also carries
// StorageImageWriteWithoutFormat; the installed dxc emits only
// StorageImageExtendedFormats for an explicit-format storage image — caps residual.
[[vk::image_format("r11g11b10f")]] RWTexture2D<float3> refl_refr_radiance_out_tex : register(u17, space0);

// Tangent-space coordinate swap (x, y, z) -> (x, -z, y) applied to view dirs;
// matches the float3x3( (1,0,0), (0,0,1), (0,-1,0) ) row-matrix in the bins.
float3 swap_yz(float3 v)
{
    return mul(v, float3x3(float3(1.0f, -0.0f, 0.0f),
                          float3(-0.0f, 0.0f, 1.0f),
                          float3(-0.0f, -1.0f, 0.0f)));
}

// Reconstruct the third component of a unit normal stored as two bytes (fast
// normalize via the magic-constant sqrt(1-dot) trick).
float normal_z_from_xy(float2 xy)
{
    return asfloat(532487669u + (asint(max(0.0f, 1.0f - dot(xy, xy))) >> 1));
}

[numthreads(8, 8, 1)]
void cs_main(uint3 gtid : SV_DispatchThreadID)
{
    uint4 hit = refr_hit_tex.Load(int3(gtid.xy, 0u));
    if (0u == hit.x)
    {
        return;
    }

    float3 n_local = float3((float((hit.x >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                            (float((hit.x >> 16u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                            0.0f);
    n_local.z = normal_z_from_xy(n_local.xy);

    float2 ndc = (((float2(int2(gtid.xy)) + 0.5f) / float2(depth_tex_dims)) - 0.5f) * float2(2.0f, -2.0f);
    float4 view_ray_vs4 = mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f));
    float3 view_ray = normalize(mul(view_constants[0].view_to_world, view_ray_vs4).xyz);
    float4 wp4 = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, depth_tex.Load(int3(gtid.xy, 0u)).x, 1.0f)));
    float3 world_pos = wp4.xyz / wp4.w;

    float3 view_dir = swap_yz(-view_ray);
    float roughness_raylen = length(n_local * unpack_half2x16(hit.x & 0xFFFFu).x);
    float dist = length(mul(view_constants[0].world_to_view, float4(world_pos, 1.0f)).xyz);
    float vdotn = -normalize(view_ray_vs4.xyz).z;
    float blur = max(roughness_raylen,
        (((dist / vdotn) * view_constants[0].clip_to_view[1].y) * 0.20000000298023223876953125f)
            * smoothstep(0.0f, 0.0500000007450580596923828125f * dist, roughness_raylen));
    float proj_scale = (dist * vdotn) * view_constants[0].clip_to_view[1].y;

    float3 refl_dir = swap_yz(-normalize(lerp(float3(0.0f, 1.0f, 0.0f),
        reflect(view_ray, float3(0.0f, 1.0f, 0.0f)), 0.959011614322662353515625f)));
    float3 binorm = normalize(cross(float3(0.0f, 1.0f, 0.0f), refl_dir))
        * max(min((((blur / (blur + dist)) * dist) * vdotn) * 0.0684653222560882568359375f,
            0.100000001490116119384765625f * proj_scale),
            (4.0f / float(refl_refr_radiance_out_tex_dims.y)) * proj_scale);
    float3 tangent = cross(refl_dir, binorm);

    float golden = (float((frame_index * 59u) % 128u) * 1.324717998504638671875f)
        + (dot(float2(gtid.xy & uint2(1u, 1u)), float2(1.0f, 2.0f)) * 0.654508531093597412109375f);

    float4 acc = 0.0f;
    float t = 0.0f;
    for (uint i = 0u; i < 8u; ++i)
    {
        float angle = (float(i) + golden) * 2.3999631404876708984375f;
        float3 sample_pos = world_pos + ((binorm * cos(angle)) + (tangent * sin(angle))) * t;
        float4 s4 = mul(view_constants[0].view_to_sample,
            mul(view_constants[0].world_to_view, float4(sample_pos, 1.0f)));
        uint2 px = uint2(int2(floor((((s4.xyz / s4.w).xy * float2(0.5f, -0.5f)) + 0.5f) * float2(refr_hit_tex_dims))));
        uint4 sample_hit = refr_hit_tex.Load(int3(px, 0u));
        if (sample_hit.x != 0u)
        {
            float3 sn = float3((float((sample_hit.x >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                               (float((sample_hit.x >> 16u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                               0.0f);
            sn.z = normal_z_from_xy(sn.xy);
            float3 v = normalize(view_dir + normalize((sn * unpack_half2x16(sample_hit.x & 0xFFFFu).x)
                + swap_yz(sample_pos - world_pos)));
            float weight = 3.1640625820728018879890441894531e-05f / pow((-0.9943749904632568359375f * (v.z * v.z)) + 1.0f, 2.0f);
            acc += float4(refr_hit_radiance_tex.Load(int3(px, 0u)).xyz, 1.0f) * weight;
        }
        t += 0.125f;
    }

    refl_refr_radiance_out_tex[gtid.xy] = (acc.xyz / max(9.9999998245167004418121337039338e-15f, acc.w)).xyz;
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _water_ice_refr_resolve.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint2 depth_tex_dims : packoffset(c0);
    uint2 refr_hit_tex_dims : packoffset(c0.z);
    uint2 refr_hit_radiance_tex_dims : packoffset(c1);
    uint2 refl_refr_radiance_out_tex_dims : packoffset(c1.z);
};

Texture2D<float4> depth_tex : register(t14, space0);
Texture2D<uint4> refr_hit_tex : register(t15, space0);
Texture2D<float4> refr_hit_radiance_tex : register(t16, space0);
RWTexture2D<float3> refl_refr_radiance_out_tex : register(u17, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    do
    {
        uint4 _116 = refr_hit_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
        uint _117 = _116.x;
        if (0u == _117)
        {
            break;
        }
        float3 _134 = float3((float((_117 >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, (float((_117 >> 16u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f);
        float2 _135 = _134.xy;
        _134.z = asfloat(532487669 + (asint(max(0.0f, 1.0f - dot(_135, _135))) >> 1));
        float2 _160 = (((float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx) / float2(depth_tex_dims)) - 0.5f.xx) * float2(2.0f, -2.0f);
        float _161 = _160.x;
        float4 _164 = mul(frame_constants_view_constants[0u].sample_to_view, float4(_161, _160.y, 0.0f, 1.0f));
        float4 _168 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_161, _160.y, depth_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, 1.0f)));
        float3 _172 = _168.xyz / _168.w.xxx;
        float3 _183 = normalize(normalize(mul(frame_constants_view_constants[0u].view_to_world, _164).xyz));
        float3 _185 = mul(-_183, float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
        float _186 = length(_134 * spvUnpackHalf2x16(_117 & 65535u).x);
        float _187 = length(mul(frame_constants_view_constants[0u].world_to_view, float4(_172, 1.0f)).xyz);
        float _191 = -normalize(_164.xyz).z;
        float _201 = max(_186, (((_187 / _191) * frame_constants_view_constants[0u].clip_to_view[1].y) * 0.20000000298023223876953125f) * smoothstep(0.0f, 0.0500000007450580596923828125f * _187, _186));
        float _208 = (_187 * _191) * frame_constants_view_constants[0u].clip_to_view[1].y;
        float3 _221 = reflect(-normalize(lerp(float3(0.0f, 1.0f, 0.0f), reflect(_183, float3(0.0f, 1.0f, 0.0f)), 0.959011614322662353515625f.xxx)), float3(0.0f, 1.0f, 0.0f));
        float3 _224 = normalize(cross(float3(0.0f, 1.0f, 0.0f), _221)) * max(min((((_201 / (_201 + _187)) * _187) * _191) * 0.0684653222560882568359375f, 0.100000001490116119384765625f * _208), (4.0f / float(refl_refr_radiance_out_tex_dims.y)) * _208);
        float3 _225 = cross(_221, _224);
        float _236 = (float((frame_constants_frame_index * 59u) % 128u) * 1.324717998504638671875f) + (dot(float2(gl_GlobalInvocationID.xy & uint2(1u, 1u)), float2(1.0f, 2.0f)) * 0.654508531093597412109375f);
        float4 _238;
        _238 = 0.0f.xxxx;
        float4 _239;
        float _241 = 0.0f;
        int _243 = 0;
        for (; uint(_243) < 8u; _238 = _239, _241 += 0.125f, _243++)
        {
            float _251 = (float(_243) + _236) * 2.3999631404876708984375f;
            float3 _258 = _172 + (((_224 * cos(_251)) + (_225 * sin(_251))) * _241);
            float4 _266 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(_258, 1.0f)));
            uint2 _280 = uint2(int2(floor((((_266.xyz / _266.w.xxx).xy * float2(0.5f, -0.5f)) + 0.5f.xx) * float2(refr_hit_tex_dims))));
            uint4 _281 = refr_hit_tex.Load(int3(_280, 0u));
            uint _282 = _281.x;
            if (_282 != 0u)
            {
                float3 _299 = float3((float((_282 >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, (float((_282 >> 16u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f);
                float2 _300 = _299.xy;
                _299.z = asfloat(532487669 + (asint(max(0.0f, 1.0f - dot(_300, _300))) >> 1));
                float3 _315 = normalize(_185 + normalize((_299 * spvUnpackHalf2x16(_282 & 65535u).x) + mul(_258 - _172, float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)))));
                float _316 = _315.z;
                float _319 = ((_316 * _316) * (-0.9943749904632568359375f)) + 1.0f;
                _239 = _238 + (float4(refr_hit_radiance_tex.Load(int3(_280, 0u)).xyz, 1.0f) * (3.1640625820728018879890441894531e-05f / (_319 * _319)));
            }
            else
            {
                _239 = _238;
            }
        }
        refl_refr_radiance_out_tex[gl_GlobalInvocationID.xy] = (_238.xyz / max(9.9999998245167004418121337039338e-15f, _238.w).xxx).xyz;
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
