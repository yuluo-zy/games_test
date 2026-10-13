// ==SKELETON== -> ==RECONSTRUCTED== (water-ice-a)
// SOURCE PATH : shaders/ice_plane.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _ice_plane.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin
//     (vs+ps are structurally identical across both variants — only embedded paths differ)
// entries: vs:vs_main  ps:ps_main | index_pass W45 (water/ice_surface_pass)
// Status: O (bindings/entry; vs + ps transcribed from spirv-cross)
// Evidence: spirv-cross vs (ice plane at y=0, xz*130, C0=(0,1,0), C1=world) and ps
//   (hex-lattice surface_roughness sampling with SampleGrad, half-res reflection
//   ray march, cascade-shadow PCF+PCSS, temporal shadow history, refraction
//   sampling, IsHelperLane/QuadReadAcross quad-subset reflection emission, and
//   5 render-target outputs).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/math.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/halfres.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/shadow.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/cascades.hlsl"
#include "inc/stochastic_tiling.hlsl"
#include "inc/history_valid.hlsl"
#include "inc/blue_noise.hlsl"
#include "water/ice_shared.inc.hlsl"
#include "inc/pack_unpack.hlsl"

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

RWByteAddressBuffer __alias_ice_refl_count_buffer : register(u23, space0);
RWByteAddressBuffer __alias_ice_refl_px_buffer : register(u24, space0);
Texture2DArray<float4> shadow_map : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t18, space0);
Texture2D<float4> refraction_tex : register(t19, space0);
Texture2D<float4> shadows_history_tex : register(t21, space0);
Texture2D<float4> surface_roughness_tex : register(t22, space0);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    float2 scaled = world.xz * 130.0f.xx;
    float4 p = float4(scaled.x, world.y, scaled.y, world.w);
    p.y = 0.0f;
    o.position = mul(view_constants[current_view_index].world_to_sample, p);
    o.C0 = float3(0.0f, 1.0f, 0.0f);
    o.C1 = p.xyz;
    return o;
}

// Tangent-space coordinate swap (x, y, z) -> (x, -z, y).
float3 swap_yz(float3 v)
{
    return mul(v, float3x3(float3(1.0f, -0.0f, 0.0f),
                          float3(-0.0f, 0.0f, 1.0f),
                          float3(-0.0f, -1.0f, 0.0f)));
}

// Integer hash of a 2D cell (LCG mix), [0,1) after the final avalanche.
float cell_hash01(int2 cell)
{
    uint y = uint(cell.y);
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    uint h = ((uint(cell.x) * 1664525u) + (y + (y << 15u))) + 1013904223u;
    h *= 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    return asfloat(((h ^ (h >> 18u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}

// March `start->end` against the half-res depth buffer, returning the bracketed
// hit fraction `hit_t` (0..1) and whether a crossing was found.
bool depth_march(float3 start, float3 end, float steps_max, float2 frame_hash,
                 out float hit_t)
{
    float3 seg = end - start;
    float t_count = (end.z > 9.5367431640625e-07f) ? steps_max : max(1.0f, steps_max - 1.0f);
    float inv = 1.0f / t_count;
    float depth_eps = 0.100000001490116119384765625f * view_constants[0].clip_to_view[3].z;

    float prev_e = 0.0f;
    float prev_ray = 0.0f;
    bool prev_under = false;
    if (steps_max > 0.0f)
    {
        float t_first = lerp(0.0f, 1.0f, pow(frame_hash.x * inv, 1.0f));
        float3 sp = start + (seg * t_first);
        float inv_z = 1.0f / sp.z;
        float d = half_depth_tex.SampleLevel(sampler_nnc, (sp.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
        float diff = (1.00000202655792236328125f / d) - inv_z;
        bool under = (diff < 0.0f);
        prev_e = under ? 0.0f : diff;
        prev_ray = under ? t_first : 0.0f;
        prev_under = under;
        for (uint i = 1u; i < uint(t_count); ++i)
        {
            float tt = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(i) + frame_hash.x) * inv, 1.0f)));
            float3 sp2 = start + (seg * tt);
            float inv_z2 = 1.0f / sp2.z;
            float d2 = half_depth_tex.SampleLevel(sampler_nnc, (sp2.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
            float diff2 = (1.00000202655792236328125f / d2) - inv_z2;
            bool under2 = (diff2 < 0.0f);
            if (under2)
            {
                prev_e = diff2;
                prev_ray = tt;
                prev_under = true;
                break;
            }
            prev_e = diff2;
            prev_under = under2;
        }
    }

    if (!prev_under)
    {
        hit_t = prev_e;
        return false;
    }

    float lo = prev_e;
    float hi = prev_ray;
    float d_lo = 0.0f;
    float d_hi = prev_e;
    float ray_hi = prev_ray;
    float t_hi = prev_ray;
    for (uint j = 0u; j < 1u; ++j)
    {
        float mid = (lo + hi) * 0.5f;
        float3 sm = start + (seg * mid);
        float inv_zm = 1.0f / sm.z;
        float dm = half_depth_tex.SampleLevel(sampler_nnc, (sm.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
        float diffm = (1.00000202655792236328125f / dm) - inv_zm;
        bool under_m = (diffm < 0.0f);
        lo = under_m ? lo : mid;
        d_hi = under_m ? diffm : d_hi;
        ray_hi = under_m ? ray_hi : (inv_zm - (1.0f / dm));
        t_hi = under_m ? mid : t_hi;
    }
    float interp = lerp(lo, t_hi, d_lo / (d_lo + (-d_hi)));
    hit_t = interp;
    return true;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
};

struct PsOut
{
    float3 C0 : SV_Target0;
    float C1 : SV_Target1;
    uint C2 : SV_Target2;
    uint C3 : SV_Target3;
    float C4 : SV_Target4;
};

[earlydepthstencil]
PsOut ps_main(PsIn vin, float4 position : SV_Position)
{
    PsOut o;
    bool helper = IsHelperLane();
    uint2 px = uint2(position.xy);
    uint2 half_px = px / uint2(2u, 2u);
    float2 half_dims = float2(half_depth_tex_dims);
    float2 ndc = (((float2(int2(half_px)) + 0.5f) / half_dims) - 0.5f) * float2(2.0f, -2.0f);

    float4 pos_vs = float4(ndc, position.z, 1.0f);
    float3 view_ray = -normalize(mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f))).xyz);
    float3 up_world = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;

    // Hex-lattice roughness sampling (surface_roughness_tex) with SampleGrad.
    float2 roughness_dims = float2(surface_roughness_tex_dims);
    float2 rough_p = vin.C1.xz * 0.100000001490116119384765625f;
    float2 rough_hex = mul(rough_p * 3.4639999866485595703125f,
        float2x2(float2(1.0f, 0.0f), float2(-0.57735025882720947265625f, 1.1547005176544189453125f)));
    float2 rough_cell = floor(rough_hex);
    float2 rough_frac = frac(rough_hex);
    bool tri_up = ((1.0f - rough_frac.x) - rough_frac.y) > 0.0f;
    float3 c0 = tri_up ? float3(rough_cell, (1.0f - rough_frac.x) - rough_frac.y)
                       : float3(rough_cell + float2(1.0f, 1.0f), rough_frac.x - (1.0f - rough_frac.y) + rough_frac.y);
    float3 c1 = tri_up ? float3(rough_cell + float2(0.0f, 1.0f), rough_frac.y)
                       : float3(rough_cell + float2(1.0f, 0.0f), 1.0f - rough_frac.y);
    float3 c2 = tri_up ? float3(rough_cell + float2(1.0f, 0.0f), rough_frac.x)
                       : float3(rough_cell + float2(0.0f, 1.0f), 1.0f - rough_frac.x);

    float4 grad = float4(ddx(rough_p), -ddy(rough_p));
    float2 grad_x = grad.xy;
    float2 grad_y = grad.zw;
    float4 rough_scale4 = surface_roughness_tex.SampleLevel(sampler_llr, 0.0f,
        (log(min(roughness_dims.x, roughness_dims.y)) * 1.44269502162933349609375f) + 1.0f);

    uint2 cell0 = uint2(c0.xy);
    uint2 h0 = ((cell0 >> uint2(8u, 8u)) ^ cell0.yx) * uint2(1103515245u, 1103515245u);
    uint2 cell1 = uint2(c1.xy);
    uint2 h1 = ((cell1 >> uint2(8u, 8u)) ^ cell1.yx) * uint2(1103515245u, 1103515245u);
    uint2 cell2 = uint2(c2.xy);
    uint2 h2 = ((cell2 >> uint2(8u, 8u)) ^ cell2.yx) * uint2(1103515245u, 1103515245u);
    float2 j0 = float2((h0 >> uint2(8u, 8u)) ^ h0.yx) * uint2(1103515245u, 1103515245u) * 2.3283064365386962890625e-10f;
    float2 j1 = float2((h1 >> uint2(8u, 8u)) ^ h1.yx) * uint2(1103515245u, 1103515245u) * 2.3283064365386962890625e-10f;
    float2 j2 = float2((h2 >> uint2(8u, 8u)) ^ h2.yx) * uint2(1103515245u, 1103515245u) * 2.3283064365386962890625e-10f;

    float rough_sum = (surface_roughness_tex.SampleGrad(sampler_llr, rough_p + j0, grad_x, grad_y) * c0.z).x
        + (surface_roughness_tex.SampleGrad(sampler_llr, rough_p + j1, grad_x, grad_y) * c1.z).x
        + (surface_roughness_tex.SampleGrad(sampler_llr, rough_p + j2, grad_x, grad_y) * c2.z).x;
    float roughness = 0.0199999995529651641845703125f + (0.300000011920928955078125f
        * (1.0f - ((rough_sum - rough_scale4.x) / length(float3(c0.z, c1.z, c2.z)) + rough_scale4.x)));

    float3 view_dir = swap_yz(view_ray);
    uint quad_seed = ((px.x & 1u) | ((px.y & 1u) * 2u));
    uint hilbert;
    {
        uint acc = 0u;
        uint xi = px.x;
        uint yi = px.y;
        for (uint s = 32u; s > 0u; s /= 2u)
        {
            uint bx = uint((xi & s) > 0u);
            uint by = uint((yi & s) > 0u);
            acc += (s * s) * ((3u * bx) ^ by);
            if (by == 0u && bx == 1u)
            {
                xi = 63u - xi;
                yi = 63u - yi;
            }
        }
        hilbert = acc;
    }
    uint frame_mod = frame_index % 64u;
    float2 bn = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(hilbert + (288u * frame_mod))) + 0.5f);

    float3 rough_dir = normalize(float3(view_dir.xy * 0.07500000298023223876953125f, view_dir.z));
    float a = 6.283185482025146484375f * bn.x;
    float c = 1.0f - (bn.y * 0.300000011920928955078125f);
    float s = sqrt(clamp(1.0f - (c * c), 0.0f, 1.0f));
    float3 refl_dir = float3(s * cos(a), s * sin(a), c) + rough_dir;

    float frame_rand = 5.588237762451171875f * float(frame_mod);
    float hash = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px.x) + frame_rand))
        + (0.005837149918079376220703125f * (float(px.y) + frame_rand))));

    float4 wp4 = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, half_depth_tex.Load(int3(half_px, 0u)).x, 1.0f)));
    float3 world_pos = wp4.xyz / wp4.w;
    float3 start_world = vin.C1;
    float4 ref_end4 = mul(view_constants[0].view_to_sample,
        mul(view_constants[0].world_to_view, float4(vin.C1
            + (swap_yz(normalize(float3(refl_dir.xy * 0.07500000298023223876953125f, refl_dir.z)))
                * float3(-1.0f, 1.0f, -1.0f)) * min(4.0f, 4.0f * length(vin.C1 - world_pos)), 1.0f)));
    float3 ref_dir = ref_end4.xyz / ref_end4.w;
    float4 march_vs = pos_vs + (float4(ref_dir - pos_vs.xyz, 0.0f) * float(int(sign(ref_dir.z))));

    // Slab-clip the march segment to the half-depth view frustum.
    float3 m0 = march_vs.xyz;
    float3 m1 = ref_dir;
    bool3 neg3 = (m1 - m0) < 0.0f;
    float3 t0 = (float3(neg3.x ? 1.0f : -1.0f, neg3.y ? 1.0f : -1.0f, neg3.z ? 1.0f : -1.0f) - m0) / (m1 - m0);
    float3 a0 = m0 + ((m1 - m0) * max(0.0f, max(t0.x, t0.y)));
    float3 r3 = (m1 - m0) - (a0 - m0);
    bool3 p3 = r3 >= 0.0f;
    float3 t1 = (float3(p3.x ? 1.0f : -1.0f, p3.y ? 1.0f : -1.0f, p3.z ? 1.0f : -1.0f) - a0) / r3;
    float3 a1 = a0 + (r3 * min(1.0f, min(min(t1.x, t1.y), t1.z)));

    float hit_t;
    bool depth_hit = depth_march(a0, a1, 3.0f, float2(hash, bn.y), hit_t);
    float3 march_hit = lerp(a0, a1, hit_t);
    float2 hit_uv = (march_hit.xy * float2(0.5f, -0.5f)) + 0.5f;
    float4 wp_hit = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(march_hit.x, march_hit.y, half_depth_tex.SampleLevel(sampler_nnc, hit_uv, 0.0f).x, 1.0f)));
    float3 hit_world = wp_hit.xyz / wp_hit.w;
    float3 hit_offset = hit_world - vin.C1;
    float3 hit_normal = -up_world;

    // --- Cascade shadow (PCSS-like) ---
    float2 shadow = 0.0f;
    float shadow_blur = 0.0f;
    float4 sc = mul(view_constants[2].world_to_sample, float4(vin.C1, 1.0f));
    if (sc.z <= 1.0f)
    {
        float2 sc_uv = sc.xy;
        bool out0 = any(abs(sc_uv) > 0.9833333492279052734375f);
        float2 uv1 = out0 ? sc_uv * 0.5f : sc_uv;
        float texel_scale = out0 ? 5.0f : 10.0f;
        float sample_scale = out0 ? 0.5f : 1.0f;
        bool out1 = any(abs(uv1) > (1.0f - (0.000833333353511989116668701171875f * (2.0f * texel_scale))));
        if (out1)
        {
            texel_scale = texel_scale * 0.2857142984867095947265625f;
            uv1 = uv1 * 0.2857142984867095947265625f;
            sample_scale = sample_scale * 0.2857142984867095947265625f;
        }
        uint slice = uint(out1 ? 2u : uint(out0));
        uint slice_idx = 2u + slice;
        float ndl = up_world.y;
        float4 proj = float4(uv1, sc.z, sc.w) + mul(view_constants[slice_idx].world_to_sample,
            float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - ndl, 0.0f, 1.0f))) / sample_scale, 0.0f));
        float2 shadow_uv = (proj.xy * float2(0.5f, -0.5f)) + 0.5f;
        float3 shadow_xyz = float3(shadow_uv, proj.z);
        float bias = max((0.300000011920928955078125f / sample_scale) * (1.0f - abs(ndl)),
                         0.02500000037252902984619140625f);
        float spiral_seed = float((uint((px.x * 11) + (px.y * 7)) + (frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
        float hash2 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px.x) + frame_rand))
            + (0.005837149918079376220703125f * (float(px.y) + frame_rand))));
        float4 up4 = mul(view_constants[slice_idx].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));
        float blocked = 0.0f;
        int hits = 0;
        for (int i = 0; i < 5; ++i)
        {
            float ang = (float(i) + spiral_seed) * 2.3999631404876708984375f;
            float3 sp = shadow_xyz + float3((float2(cos(ang), sin(ang)) * (float(i) + hash2)) * 0.20000000298023223876953125f * texel_scale * 0.000833333353511989116668701171875f, bias * (-0.0009765625f));
            float d = shadow_map.SampleLevel(sampler_shadow_nearest, float3(sp.xy, float(slice)), 0.0f).x;
            if (sp.z > d)
            {
                blocked += d;
                ++hits;
            }
        }
        if (hits != 0)
        {
            float avg = blocked / float(hits);
            float pen = max((proj.z - avg) * ((17.0666675567626953125f * texel_scale) * pow(10.0f, smoothstep(0.0f, 0.25f, fog_tint_thickness.w))) / avg, 0.0f);
            shadow_blur = (pen / pow(pow(pen / (texel_scale - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
            float2 up_uv = ((up4.xy * float(int(sign(up4.z))) / max(0.1500000059604644775390625f, abs(up4.z))) * 0.0009765625f) * 2.0f;
            float2 acc = 0.0f;
            for (int j = 0; j < 10; ++j)
            {
                float ang2 = (float(j) + spiral_seed) * 2.3999631404876708984375f;
                float2 sp2 = (float2(cos(ang2), sin(ang2)) * (float(j) + hash2)) * 0.100000001490116119384765625f * shadow_blur;
                float3 scp = shadow_xyz + float3(sp2 * 0.000833333353511989116668701171875f, bias * (-0.0009765625f));
                float lit = 1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear,
                    float3(scp.xy, float(slice)), scp.z + min(0.0f, dot(up_uv, sp2)));
                acc += float2(lit, lit * lit);
            }
            shadow = acc * 0.100000001490116119384765625f;
        }
    }

    float shadow_m = clamp((shadow.x - 0.100000001490116119384765625f) * 1.4285714626312255859375f, 0.0f, 1.0f);
    float2 shadow_std = shadow_m + (float2(-1.0f, 1.0f) * sqrt(max(0.0f, shadow.y - (shadow.x * shadow.x))) * (3.0f * max(0.0f, 1.0f - (shadow_blur * 0.20000000298023223876953125f))));

    // Temporal shadow history.
    float2 fb_uv = (float2(px) + 0.5f) / float2(framebuffer_dims);
    float4 prev4 = mul(view_constants[0].clip_to_prev_clip,
        mul(view_constants[0].view_to_clip,
            mul(view_constants[0].clip_to_view, float4((fb_uv - 0.5f) * float2(2.0f, -2.0f), position.z, 1.0f))));
    float2 prev_uv = ((prev4.xy / prev4.w) * float2(0.5f, -0.5f)) + 0.5f;
    float2 prev_clamped = clamp(prev_uv, 0.0f, 1.0f);
    float4 motion = float4(prev_uv - ((float4((fb_uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 0.0f).xy * float2(0.5f, -0.5f)) + 0.5f),
        float(all(prev_clamped == prev_uv)), 1.0f) * float4(1.0f, 1.0f, clamp(1.0f - length(view_constants[0].clip_to_prev_clip[3].xyz), 0.0f, 1.0f), 1.0f);
    float shadow_hist;
    if (motion.z > 0.0f && history_valid != 0u && frame_index != 0u)
    {
        shadow_hist = shadows_history_tex.SampleLevel(sampler_lnc, fb_uv + motion.xy, 0.0f).x;
    }
    else
    {
        shadow_hist = shadow_m;
    }
    float shadow_temporal = lerp(clamp(shadow_hist, shadow_std.x, shadow_std.y), shadow_m, lerp(1.0f, 0.125f, motion.z));
    float inv_shadow = 1.0f - shadow_temporal;

    // Refraction sampling.
    float4 rp = mul(view_constants[0].view_to_sample,
        mul(view_constants[0].world_to_view, float4(vin.C1 + (up_world * 0.5f), 1.0f)));
    float3 rp_w = rp.xyz / rp.w;
    float4 rv = pos_vs + (float4(rp_w - pos_vs.xyz, 0.0f) * float(int(sign(rp_w.z))));
    float3 r0 = rv.xyz;
    float3 r1 = rp_w;
    bool3 rneg = (r1 - r0) < 0.0f;
    float3 rt0 = (float3(rneg.x ? 1.0f : -1.0f, rneg.y ? 1.0f : -1.0f, rneg.z ? 1.0f : -1.0f) - r0) / (r1 - r0);
    float3 ra0 = r0 + ((r1 - r0) * max(0.0f, max(rt0.x, rt0.y)));
    float3 rr3 = (r1 - r0) - (ra0 - r0);
    bool3 rp3 = rr3 >= 0.0f;
    float3 rt1 = (float3(rp3.x ? 1.0f : -1.0f, rp3.y ? 1.0f : -1.0f, rp3.z ? 1.0f : -1.0f) - ra0) / rr3;
    float3 ra1 = ra0 + (rr3 * min(1.0f, min(min(rt1.x, rt1.y), rt1.z)));

    float refr_hit_t;
    bool refr_hit = depth_march(ra0, ra1, 3.0f, float2(hash, bn.y), refr_hit_t);
    float refr_factor;
    if (refr_hit)
    {
        float3 rhp = lerp(ra0, ra1, refr_hit_t);
        float2 rhuv = (rhp.xy * float2(0.5f, -0.5f)) + 0.5f;
        float rl = 1.0f / rhp.z;
        float rd_lo = 1.0f / half_depth_tex.SampleLevel(sampler_llc, rhuv, 0.0f).x;
        float rd_hi = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, rhuv, 0.0f).x;
        float ddiff = (max(rd_lo, rd_hi) * 1.00000202655792236328125f) - rl;
        float ddiff2 = rl - min(rd_lo, rd_hi);
        refr_factor = min(inv_shadow, smoothstep(0.5f, 0.0f, (1.0f - (ddiff2 / (0.5f * view_constants[0].clip_to_view[3].z)))
            * (1.0f - ddiff)));
    }
    else
    {
        refr_factor = inv_shadow;
    }

    // Reflection emission to the refl buffer (one representative per quad subset).
    uint quad_flag = helper ? 0u : (1u << quad_seed);
    uint quad_sum = quad_flag + QuadReadAcrossX(quad_flag);
    if ((px.x & 1u) + ((px.y & 1u) * 2u) == firstbithigh(quad_sum + QuadReadAcrossY(quad_sum)))
    {
        uint slot;
        __alias_ice_refl_count_buffer.InterlockedAdd(12, 1u, slot);
        __alias_ice_refl_px_buffer.Store<uint>(slot * 4u + 0u, half_px.x + (half_px.y * 65536u));
        if ((slot & 63u) == 0u)
        {
            uint group;
            __alias_ice_refl_count_buffer.InterlockedAdd(0, 1u, group);
        }
    }

    // Specular / refraction response for the surface.
    float3 spec = 0.0f;
    float up_w = up_world.y;
    bool spec_ok = up_w > 0.0f && view_ray.y > 0.0f;
    if (spec_ok)
    {
        float r2 = roughness * roughness;
        float3 h2 = normalize(view_ray + up_world);
        float ndh = max(0.0f, 1.0f - dot(h2, up_world));
        float g2 = ndh * ndh;
        float4 denom = float4(1.0f, 1.0f, 1.0f, (h2.y * h2.y * (r2 - 1.0f)) + 1.0f);
        float3 fres = lerp(0.02999999932944774627685546875f, 1.0f, (g2 * g2) * ndh);
        float gterm = (2.0f * up_w * view_ray.y) / (up_w * sqrt(((((-view_ray.y) * r2) + view_ray.y) * view_ray.y) + r2) + (view_ray.y * sqrt(((((-up_w) * r2) + up_w) * up_w) + r2)));
        float3 brdf = (fres * gterm) * (r2 / ((3.1415927410125732421875f * denom.w) * denom.w)) / ((4.0f * view_ray.y) * up_w);
        spec = brdf;
    }

    float3 hit_off = hit_offset * float3(1.0f, -1.0f, 1.0f);
    float3 hit_off_swap = swap_yz(hit_off);
    float hit_len = length(hit_off_swap);
    float3 hit_n = hit_off_swap * (1.0f / max(1e-9f, hit_len));
    float3 refr_col = clamp(exp(float3(-0.5950000286102294921875f, -0.374000012874603271484375f, -0.3060000240802764892578125f)
        * ((abs(vin.C1.y - hit_world.y) + length(hit_offset)) + 0.699999988079071044921875f))
        * (0.959999978542327880859375f - (0.959999978542327880859375f * ((view_ray.y * view_ray.y) * (1.0f - view_ray.y))))
        * refraction_tex.SampleLevel(sampler_lnc, hit_uv, 0.0f).xyz, 0.0f, 65408.0f);

    float scale = asfloat((asuint(max(max(1.52587890625e-05f, refr_col.x), max(refr_col.y, refr_col.z))) + 125845504u) & 2139095040u);
    uint3 mantissa = asuint(refr_col + scale);
    uint radiance = ((((asuint(scale) << 4u) + 268435456u) | (mantissa.z << 18u)) | (mantissa.y << 9u)) | (mantissa.x & 511u);

    o.C0 = lerp(sun_color.xyz * spec * (refr_factor * clamp(up_w, 0.0f, 1.0f)),
        (sun_color.xyz * (0.699999988079071044921875f * refr_factor) * 0.3183098733425140380859375f) + sky_high_color.xyz,
        0.00999999977648258209228515625f);
    o.C1 = sqrt(roughness);
    o.C2 = ((uint((clamp((hit_n.x * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u)
        | (uint((clamp((hit_n.y * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u))
        | pack_half2x16(float2(hit_len, 0.0f));
    o.C3 = radiance;
    o.C4 = shadow_temporal + 0.00196078442968428134918212890625f;
    return o;
}
