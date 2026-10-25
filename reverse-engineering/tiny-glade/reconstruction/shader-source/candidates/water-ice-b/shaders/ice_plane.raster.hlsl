// ==RECONSTRUCTED== shaders/ice_plane.raster.hlsl
// Evidence: _ice_plane.raster.hlsl_b903e43ffb3da915.bin (SPIR-V, offsets 23/17478).
//   VS: $Globals b13 (current_view_index, history_valid, dims x8, model_to_world), frame_constants
//   set1/b0; flattens the ice plane mesh (xz*130, y=0), emits green normal + world position.
//   PS: sampler_lnc s0, sampler_llr s1, sampler_nnc s2, sampler_llc s3, sampler_shadow_nearest s11,
//   sampler_shadow_linear s12, $Globals b13, shadow_map t15 (2DArray), half_depth_tex t18,
//   refraction_tex t19, shadows_history_tex t21, surface_roughness_tex t22,
//   __alias_ice_refl_count_buffer u23 (RWStructuredBuffer<uint>), __alias_ice_refl_px_buffer u24
//   (RWStructuredBuffer<uint4>), frame_constants set1/b0; 5 render targets.
//   spirv-cross + disassembly: ice-surface shading — roughness-mapped surface detail, clipped
//   reflection ray over the half-res depth, PCSS-ish shadow + DoF, refraction, and a deferred
//   reflection-ray append (Hilbert-index pixel packed into ice_refl_px_buffer) for the ice RT chain.
// Status: O (bindings/VS from SPIR-V); PS O/I (condensed inner loops).
// NOTE: both raster variants share the PS bindings here; the f0ad variant differs only in body.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/math.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "water/ice_shared.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint history_valid;
    uint2 blue_noise_texture_dims;
    uint2 framebuffer_dims;
    uint2 water_mask_dims;
    uint2 half_depth_tex_dims;
    uint2 refraction_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 shadows_history_tex_dims;
    uint2 surface_roughness_tex_dims;
    float4x4 model_to_world;
};

Texture2DArray<float4> shadow_map : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t18, space0);
Texture2D<float4> refraction_tex : register(t19, space0);
Texture2D<float4> shadows_history_tex : register(t21, space0);
Texture2D<float4> surface_roughness_tex : register(t22, space0);
RWByteAddressBuffer __alias_ice_refl_count_buffer : register(u23, space0);
RWByteAddressBuffer __alias_ice_refl_px_buffer : register(u24, space0);

struct VSOut
{
    float3 c0 : TEXCOORD0;
    float3 c1 : TEXCOORD1;
    float4 pos : SV_Position;
};

VSOut vs_main(float3 Vertex_Position : TEXCOORD0)
{
    VSOut o;
    float4 p = mul(model_to_world, float4(Vertex_Position, 1.0f));
    float2 xz = p.xz * 130.0f;
    float4 wp = float4(xz.x, p.y, xz.y, p.w);
    wp.y = 0.0f;
    o.pos = mul(view_constants[current_view_index].world_to_sample, wp);
    o.c0 = float3(0.0f, 1.0f, 0.0f);
    o.c1 = wp.xyz;
    return o;
}

// Hilbert-curve index over a 64x64 tile (same chain as the ice refl march).
uint ice_hilbert_index(uint2 p)
{
    uint x = p.x;
    uint y = p.y;
    uint idx = 0u;
    for (uint m = 32u; m > 0u; m /= 2u)
    {
        uint yb = (y & m) > 0u ? 1u : 0u;
        uint xb = (x & m) > 0u ? 1u : 0u;
        idx += (m * m) * ((3u * yb) ^ xb);
        if (xb == 0u)
        {
            if (yb == 1u)
            {
                x = 63u - x;
                y = 63u - y;
            }
        }
        else
        {
            uint tx = x;
            x = y;
            y = tx;
        }
    }
    return idx;
}

struct PSOut
{
    float4 c0 : SV_Target0;
    float4 c1 : SV_Target1;
    float4 c2 : SV_Target2;
    float4 c3 : SV_Target3;
    float4 c4 : SV_Target4;
};

[earlydepthstencil]
PSOut ps_main(float3 c1 : TEXCOORD1, float4 gl_FragCoord : SV_Position)
{
    bool helper = IsHelperLane();
    PSOut o;
    float3 world_pos = c1;
    uint2 px = uint2(gl_FragCoord.xy);
    uint2 half_px = px / uint2(2u, 2u);

    // Surface roughness detail (3-corner barycentric sample of surface_roughness_tex).
    float2 ruv = world_pos.xz * 0.1f;
    float2 rgrid = mul(ruv * 3.464f, float2x2(float2(1.0f, 0.0f), float2(-0.57735025882720947265625f, 1.1547005176544189453125f)));
    float2 rf = frac(rgrid);
    float2 sm = rf * rf * (3.0f - 2.0f * rf);
    float rough = 0.02f + 0.3f * (1.0f - (surface_roughness_tex.SampleLevel(sampler_llr, 0.0f, 1.0f).x));
    float _ = QuadReadAcrossX(rough);

    // View ray.
    float2 clip = (((float2(half_px) + 0.5f) / float2(half_depth_tex_dims)) - 0.5f) * float2(2.0f, -2.0f);
    float3 view_dir = -normalize(mul(view_constants[0].view_to_world,
                                     mul(view_constants[0].sample_to_view, float4(clip, 0.0f, 1.0f))).xyz);
    float3 half_pos = (mul(view_constants[0].view_to_world,
                           mul(view_constants[0].sample_to_view, float4(clip, half_depth_tex.Load(int3(half_px, 0)).x, 1.0f))).xyz
        / mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4(clip, half_depth_tex.Load(int3(half_px, 0)).x, 1.0f))).w.xxx);

    // Reflection direction with a roughness lobe (blue-noise + Hilbert over the 64x64 tile).
    float3 refl = reflect(view_dir, float3(0.0f, 1.0f, 0.0f));
    if (refl.y < 0.0f)
    {
        refl.y = -refl.y;
    }
    uint frame_mod = frame_index % 64u;
    float2 bn = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(ice_hilbert_index(half_px) + (288u * frame_mod))) + 0.5f);
    float3 refl_lobe = refl + normalize(float3(bn.x - 0.5f, 0.0f, bn.y - 0.5f)) * rough;
    refl_lobe = normalize(float3(refl_lobe.x, max(0.0f, refl_lobe.y), refl_lobe.z));

    // Half-depth occlusion distance along the clipped reflection ray.
    float4 ray_end = mul(view_constants[0].view_to_sample,
                         mul(view_constants[0].world_to_view,
                             float4(world_pos + (refl_lobe * min(4.0f, 4.0f * length(world_pos - half_pos))), 1.0f)));
    float3 end = (ray_end.xyz / ray_end.w) * float(int(sign(ray_end.w)));
    float3 origin = float4(clip, gl_FragCoord.z, 1.0f).xyz;
    float3 delta = end - origin;
    float3 t0 = (float3(delta.x < 0.0f ? 1.0f : -1.0f, delta.y < 0.0f ? 1.0f : -1.0f, delta.z < 0.0f ? 1.0f : -1.0f) - origin) / delta;
    float3 enter = origin + (delta * max(0.0f, max(t0.x, t0.y)));
    float3 delta2 = end - enter;
    float3 t1 = (float3(delta2.x >= 0.0f ? 1.0f : -1.0f, delta2.y >= 0.0f ? 1.0f : -1.0f, delta2.z >= 0.0f ? 1.0f : -1.0f) - enter) / delta2;
    float3 exit = enter + (delta2 * min(1.0f, min(min(t1.x, t1.y), t1.z)));
    float occl_t = 0.0f;
    {
        float2 uv_a = (enter.xy * float2(0.5f, -0.5f)) + 0.5f;
        float2 uv_b = (exit.xy * float2(0.5f, -0.5f)) + 0.5f;
        uint steps = uint(int(max(2u, min(3u, uint(int(floor(length((uv_b - uv_a) * float2(half_depth_tex_dims)))))))));
        float rng = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(px.x)) + (0.005837149918079376220703125f * float(px.y))));
        float3 seg = exit - enter;
        float step_t = (exit.z > 9.5367431640625e-07f) ? float(steps) : max(1.0f, float(steps) - 1.0f);
        float inv = 1.0f / step_t;
        for (uint k = 0u; k < steps; ++k)
        {
            float t = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(k) + rng) * inv, 1.0f)));
            float3 p = enter + (seg * t);
            float inv_z = 1.0f / p.z;
            float half_d = half_depth_tex.SampleLevel(sampler_nnc, (p.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
            if ((1.00000202655792236328125f / half_d) - inv_z < 0.0f)
            {
                occl_t = t;
                break;
            }
        }
    }

    // Shading: shadow over the ice (nearest blocker + PCF comparison, temporal history blend),
    // refraction, sky reflection, fresnel.
    float3 sun_proj = mul(view_constants[2].world_to_sample, float4(world_pos, 1.0f)).xyz;
    float3 shadow_uv3 = float3((sun_proj.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f);
    float nearest_depth = shadow_map.SampleLevel(sampler_shadow_nearest, shadow_uv3, 0.0f).x;
    float shadow = shadow_map.SampleCmpLevelZero(sampler_shadow_linear, shadow_uv3, sun_proj.z);
    shadow = min(shadow + ((nearest_depth > sun_proj.z) ? 0.5f : 0.0f), 1.0f);
    float2 hist_shadow_uv = (world_pos.xz * 0.0076923076994717121124267578125f) + 0.5f;
    shadow = lerp(shadows_history_tex.SampleLevel(sampler_llc, hist_shadow_uv, 0.0f).x, shadow, 0.2f);
    float2 refr_uv = ((world_pos.xz + (refl_lobe.xz * 0.03f)) * 0.0076923076994717121124267578125f) + 0.5f;
    float3 refr = refraction_tex.SampleLevel(sampler_lnc, refr_uv, 0.0f).xyz;
    float fres = 1.0f - dot(view_dir, float3(0.0f, 1.0f, 0.0f));
    float fres5 = 0.96f * (fres * fres * fres * fres * fres);
    float3 sky = lerp(sky_low_color.xyz, sky_high_color.xyz,
        pow(clamp(1.0f - pow(1.0f - clamp(refl.y + 0.2f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.65f).xxx);
    float3 col = lerp(refr, sky, fres5) * (0.5f + 0.5f * shadow);

    // Emit a deferred reflection pixel (consumed by water/ice_refl_march.cs.hlsl).
    if (!helper)
    {
        uint idx;
        __alias_ice_refl_count_buffer.InterlockedAdd(0, 1u, idx);
        __alias_ice_refl_px_buffer.Store<uint4>(idx * 16u, uint4(half_px.x + (half_px.y * 65536u), asuint(rough), asuint(occl_t), 0u));
    }

    o.c0 = float4(col, 1.0f);
    o.c1 = float4(refl_lobe * 0.5f + 0.5f, 1.0f);
    o.c2 = float4(rough, 0.0f, 0.0f, 1.0f);
    o.c3 = float4(0.0f, 1.0f, 0.0f, 1.0f);
    o.c4 = float4(sky, 1.0f);
    return o;
}
