// ==SKELETON== -> ==RECONSTRUCTED== (water-ice-a)
// SOURCE PATH : shaders/water_plane.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _water_plane.raster.hlsl_b903e43ffb3da915.bin  (full water surface render)
//   _water_plane.raster.hlsl_f0adff76827e73ff.bin  (simplified: wave normal + half-res
//     reflection march + software-RT ray emission only)
// entries: vs:vs_main  ps:ps_main
// Status: O (bindings/entry; vs + ps transcribed from spirv-cross)
// Evidence: spirv-cross vs (plane at y=-0.01, xz*130, C1=world) and ps (b903e:
//   waves.inc 4-octave + ripple-source displacement, reflection + refraction
//   half-depth marches, cascade-shadow PCSS/PCF, autofocus DoF coc, software-RT
//   reflection-ray emission, specular BRDF, water attenuation + foam; f0adff:
//   wave normal + reflection march + ray emission only).
// Note: two 16-hex variants are compile-time feature variants — gated on
//   WATER_PLANE_FULL (b903e = full render, f0adff = RT-ray source).

#ifndef WATER_PLANE_FULL
#define WATER_PLANE_FULL 1
#endif

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"
// NOTE: original embedded list includes inc/terrain_heightmap.inc.hlsl, but the
// reconstructed inc references a consumer-declared `terrain_heightmap` texture
// this shader never binds — the include is omitted (see decisions.md).
#include "water/water_surface_frag.inc.hlsl"
#include "inc/utils.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/math.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/fbm_noise.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/halfres.hlsl"
#include "inc/coc.hlsl"
#include "inc/history_valid.hlsl"
#include "inc/blue_noise.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/rt.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/shadow.hlsl"
#include "inc/cascades.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sh.hlsl"
#include "inc/lighting_indirect.hlsl"
#include "inc/material/eval/solid_material.inc.hlsl"
#include "inc/brdf_lut.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/wall_color_ids.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    float4x4 model_to_world;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint history_valid;
    uint2 terrain_lighting_texture_dims;
    uint2 blue_noise_texture_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
    uint2 water_noise_texture_dims;
    uint2 framebuffer_dims;
    uint2 water_mask_dims;
    uint2 half_depth_tex_dims;
    uint2 refraction_tex_dims;
    uint2 taa_history_tex_dims;
    uint2 reprojection_tex_dims;
    float3 water_color;
};

ByteAddressBuffer ripple_source_buffer : register(t25, space0);
ByteAddressBuffer autofocus_buf : register(t32, space0);
RWByteAddressBuffer __alias_water_rt_count_buffer : register(u33, space0);
RWByteAddressBuffer __alias_water_rt_ray_buffer : register(u34, space0);
Texture2DArray<float4> shadow_map : register(t19, space0);
Texture2D<float4> water_noise_texture : register(t26, space0);
Texture2D<float4> water_mask : register(t27, space0);
Texture2D<float4> half_depth_tex : register(t28, space0);
Texture2D<float4> refraction_tex : register(t29, space0);
Texture2D<float4> taa_history_tex : register(t30, space0);
Texture2D<float4> reprojection_tex : register(t31, space0);

#include "water/waves.inc.hlsl"

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
    p.y = -0.00999999977648258209228515625f;
    o.position = mul(view_constants[current_view_index].world_to_sample, p);
    o.C0 = float3(0.0f, 1.0f, 0.0f);
    o.C1 = p.xyz;
    return o;
}

// March `start->end` in view space against the half-res depth buffer. Returns the
// bracketed hit fraction (0..1) and whether a depth crossing was found.
bool halfres_depth_march(float3 start, float3 end, uint steps, float2 hash,
                         float clip_z, out float hit_t)
{
    float3 seg = end - start;
    float t_count = (end.z > 9.5367431640625e-07f) ? float(steps) : max(1.0f, float(steps) - 1.0f);
    float inv = 1.0f / t_count;
    float depth_eps = 0.100000001490116119384765625f * clip_z;

    float prev_e = 0.0f;
    float prev_ray = 0.0f;
    bool prev_under = false;
    if (steps > 0u)
    {
        float t_first = lerp(0.0f, 1.0f, pow(hash.x * inv, 1.0f));
        float3 sp = start + (seg * t_first);
        float inv_z = 1.0f / sp.z;
        float d = half_depth_tex.SampleLevel(sampler_nnc, (sp.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
        float diff = (1.00000202655792236328125f / d) - inv_z;
        bool under = (diff < 0.0f);
        prev_e = under ? 0.0f : diff;
        prev_ray = under ? t_first : 0.0f;
        prev_under = under;
        for (uint i = 1u; i < steps; ++i)
        {
            float tt = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(i) + hash.x) * inv, 1.0f)));
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

// Clip a view-space segment to the view frustum slab [-1,1]^2.
float3 clip_to_slab(float3 a, float3 b)
{
    bool3 neg = (b - a) < 0.0f;
    float3 t0 = (float3(neg.x ? 1.0f : -1.0f, neg.y ? 1.0f : -1.0f, neg.z ? 1.0f : -1.0f) - a) / (b - a);
    float3 a0 = a + ((b - a) * max(0.0f, max(t0.x, t0.y)));
    float3 r = (b - a) - (a0 - a);
    bool3 pos = r >= 0.0f;
    float3 t1 = (float3(pos.x ? 1.0f : -1.0f, pos.y ? 1.0f : -1.0f, pos.z ? 1.0f : -1.0f) - a0) / r;
    float3 a1 = a0 + (r * min(1.0f, min(min(t1.x, t1.y), t1.z)));
    return float3(a1.x, a1.y, a1.z);
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
};

#if WATER_PLANE_FULL
float4 ps_main(PsIn vin, float4 position : SV_Position) : SV_Target0
{
    bool helper = IsHelperLane();
    float3 world_pos = vin.C1;
    uint2 px = uint2(position.xy);
    float2 half_dims = float2(half_depth_tex_dims);

    // Wave surface normal (waves.inc: 4-octave + ripple displacement).
    float time = float(game_time_sec) + game_time_frac;
    float3 waves = water_wave_displacement(world_pos, time);
    float3 ripples = water_ripple_displacement(ripple_source_buffer, world_pos, time);
    float2 wave_xz = waves.yz;
    float2 ripple_xz = ripples.yz / abs(1.0f);
    float3 surface_normal = normalize(float3(wave_xz.x + ripple_xz.x, waves.y, wave_xz.y + ripple_xz.y));

    // Pixel-center view ray.
    float2 fb_uv = ((float2(int2(px + uint2(1u, 1u))) + 0.5f) / float2(framebuffer_dims)) - 0.5f;
    float2 ndc = fb_uv * float2(2.0f, -2.0f);
    float4 pos_vs = float4(ndc, position.z, 1.0f);
    float3 view_ray = normalize(mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f))).xyz);
    float3 to_cam = -view_ray;
    float3 up_world = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float fresnel = 1.0f - dot(to_cam, surface_normal);
    float fresnel2 = fresnel * fresnel;
    float reflectivity = 0.959999978542327880859375f * ((fresnel2 * fresnel2) * fresnel);

    float frame_rand = 5.588237762451171875f * float(frame_index % 64u);
    float hash = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px.x) + frame_rand))
        + (0.005837149918079376220703125f * (float(px.y) + frame_rand))));
    uint2 half_px = px / uint2(2u, 2u);
    float4 wp4 = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, half_depth_tex.Load(int3(half_px, 0u)).x, 1.0f)));
    float3 cam_world = wp4.xyz / wp4.w;

    // Reflection march (view-space slab-clipped segment).
    float3 refl_dir = normalize(view_ray - ((float3(surface_normal.x, 0.0f, surface_normal.z) * 0.5f) * 2.0f));
    float4 ref_end4 = mul(view_constants[0].view_to_sample,
        mul(view_constants[0].world_to_view, float4(world_pos + (refl_dir * min(4.0f, 4.0f * length(world_pos - cam_world))), 1.0f)));
    float3 ref_end = ref_end4.xyz / ref_end4.w;
    float4 march_vs = pos_vs + (float4(ref_end - pos_vs.xyz, 0.0f) * float(int(sign(ref_end.z))));
    float3 m_start = march_vs.xyz;
    float3 m_end = clip_to_slab(m_start, ref_end);

    float hit_t;
    bool depth_hit = halfres_depth_march(m_start, m_end, 3u, float2(hash, 0.0f), view_constants[0].clip_to_view[3].z, hit_t);
    float3 march_hit = lerp(m_start, m_end, hit_t);
    float2 hit_uv = (march_hit.xy * float2(0.5f, -0.5f)) + 0.5f;
    float4 wp_hit = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(march_hit.x, march_hit.y, half_depth_tex.SampleLevel(sampler_nnc, hit_uv, 0.0f).x, 1.0f)));
    float3 hit_world = wp_hit.xyz / wp_hit.w;
    float hit_dy = hit_world.y;
    float height_diff = abs(world_pos.y - hit_dy);

    // Cascade shadow (PCSS + PCF).
    float2 shadow = 0.0f;
    float shadow_blur = 0.0f;
    float4 sc = mul(view_constants[2].world_to_sample, float4(world_pos, 1.0f));
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
        float slice_f = float(out1 ? 2u : uint(out0));
        uint slice_idx = 2u + uint(slice_f);
        float ndl = dot(surface_normal, -up_world);
        float4 proj = float4(uv1, sc.z, sc.w) + mul(view_constants[slice_idx].world_to_sample,
            float4((surface_normal * (0.066666670143604278564453125f * clamp(1.0f - ndl, 0.0f, 1.0f))) / sample_scale, 0.0f));
        float2 shadow_uv = (proj.xy * float2(0.5f, -0.5f)) + 0.5f;
        float3 shadow_xyz = float3(shadow_uv, proj.z);
        float bias = max((0.300000011920928955078125f / sample_scale) * (1.0f - abs(ndl)), 0.02500000037252902984619140625f);
        float spiral_seed = float((uint((px.x * 11) + (px.y * 7)) + (frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
        float hash2 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px.x) + frame_rand))
            + (0.005837149918079376220703125f * (float(px.y) + frame_rand))));
        float4 up4 = mul(view_constants[slice_idx].world_to_sample, float4(surface_normal, 0.0f));
        float blocked = 0.0f;
        int hits = 0;
        for (int i = 0; i < 5; ++i)
        {
            float ang = (float(i) + spiral_seed) * 2.3999631404876708984375f;
            float3 sp = shadow_xyz + float3((float2(cos(ang), sin(ang)) * (float(i) + hash2)) * 0.20000000298023223876953125f * texel_scale * 0.000833333353511989116668701171875f, bias * (-0.0009765625f));
            float d = shadow_map.SampleLevel(sampler_shadow_nearest, float3(sp.xy, slice_f), 0.0f).x;
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
                float lit = 1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(scp.xy, slice_f), scp.z + min(0.0f, dot(up_uv, sp2)));
                acc += float2(lit, lit * lit);
            }
            shadow = acc * 0.100000001490116119384765625f;
        }
    }
    float shadow_m = clamp((shadow.x - 0.100000001490116119384765625f) * 1.4285714626312255859375f, 0.0f, 1.0f);
    float2 shadow_std = shadow_m + (float2(-1.0f, 1.0f) * sqrt(max(0.0f, shadow.y - (shadow.x * shadow.x))) * (3.0f * max(0.0f, 1.0f - (shadow_blur * 0.20000000298023223876953125f))));

    // DoF autofocus circle-of-confusion.
    float coc_focus = autofocus_buf.Load<float>(camera_idx * 64u + 56u);
    float coc_inv0 = 1.0f / autofocus_buf.Load<float>(camera_idx * 64u + 0u);
    float coc_inv1 = 1.0f / autofocus_buf.Load<float>(camera_idx * 64u + 4u);
    float coc_far = max(0.001000000047497451305389404296875f, max(coc_inv0, coc_inv1)) + 9.9999997473787516355514526367188e-06f;
    float coc_near = max(0.001000000047497451305389404296875f, min(coc_inv0, coc_inv1)) - 9.9999997473787516355514526367188e-06f;
    bool locked = autofocus_buf.Load<uint>(camera_idx * 64u + 44u) != 0u;
    float3 cam_at = world_pos;
    cam_at.y = world_pos.y * (locked ? 0.0f : 1.0f);
    float4 cam4 = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_fwd = normalize(mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float dist_to_plane = 1.0f / max(1.0000000133514319600180897396058e-10f, dot(cam_at - (cam4.xyz / cam4.w), cam_fwd));
    float coc_range = coc_far - coc_near;
    float coc_eps = min(0.00999999977648258209228515625f, 0.0500000007450580596923828125f * coc_range);
    float coc_raw = (dist_to_plane > coc_far) ? ((dist_to_plane - coc_far) + coc_eps)
        : ((dist_to_plane < coc_near) ? ((dist_to_plane - coc_near) - coc_eps)
        : (-coc_eps) + ((2.0f * coc_eps) * (dist_to_plane - coc_near)) / coc_range);
    float coc = locked ? (coc_raw / max(0.00999999977648258209228515625f, length(cam_fwd.xz))) : coc_raw;
    float coc_scale = sqrt(lerp(0.16000001132488250732421875f, 9.9999997473787516355514526367188e-05f,
        exp2((-0.014999999664723873138427734375f) * sqrt(max(abs((-coc * coc_focus) * view_constants[0].view_to_clip[1].y), (pos_vs.xyz / pos_vs.w).z * (-0.0030000000260770320892333984375f))))));

    // Refraction march.
    float4 rp4 = mul(view_constants[0].view_to_sample,
        mul(view_constants[0].world_to_view, float4(world_pos + (up_world * 0.5f), 1.0f)));
    float3 rp_w = rp4.xyz / rp4.w;
    float4 rv = pos_vs + (float4(rp_w - pos_vs.xyz, 0.0f) * float(int(sign(rp_w.z))));
    float3 r_start = rv.xyz;
    float3 r_end = clip_to_slab(r_start, rp_w);

    float refr_hit_t;
    bool refr_hit = halfres_depth_march(r_start, r_end, 8u, float2(hash, 0.0f), view_constants[0].clip_to_view[3].z, refr_hit_t);
    float3 refr_hit_vs = lerp(r_start, r_end, refr_hit_t);
    float2 refr_uv = (refr_hit_vs.xy * float2(0.5f, -0.5f)) + 0.5f;
    float4 wp_refr = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(refr_hit_vs.x, refr_hit_vs.y, half_depth_tex.SampleLevel(sampler_nnc, refr_uv, 0.0f).x, 1.0f)));
    float3 refr_world = wp_refr.xyz / wp_refr.w;

    // Software-RT reflection-ray emission (one representative per quad subset).
    uint quad_flag = helper ? 0u : (1u << ((px.x & 1u) + ((px.y & 1u) * 2u)));
    uint quad_sum = quad_flag + QuadReadAcrossX(quad_flag);
    quad_sum = quad_sum + QuadReadAcrossY(quad_sum);
    if ((px.x & 1u) + ((px.y & 1u) * 2u) == firstbithigh(quad_sum))
    {
        uint slot;
        __alias_water_rt_count_buffer.InterlockedAdd(0, 1u, slot);
        float3 nd = surface_normal / ((abs(surface_normal.x) + abs(surface_normal.y)) + abs(surface_normal.z));
        float3 oct;
        if (nd.z < 0.0f)
        {
            float2 o = (1.0f - abs(nd.yx)) * ((step(0.0f, nd.xy) * 2.0f) - 1.0f);
            oct = float3(o.x, o.y, nd.z);
        }
        else
        {
            oct = nd;
        }
        float2 oct_uv = (oct.xy * 0.5f) + 0.5f;
        float3 refl_w = (wp_hit.xyz / wp_hit.w);
        __alias_water_rt_ray_buffer.Store<uint4>(slot * 16u + 0u,
            uint4(half_px.x + (half_px.y * 65536u),
                pack_half2x16(float2(oct_uv.x, 0.0f)) | (pack_half2x16(float2(oct_uv.y, 0.0f)) << 16u),
                asuint(dot(refl_dir, refl_w - cam_world)),
                (asuint(position.z) & 0xFFFFFFF0u) + quad_sum));
    }

    // Specular BRDF.
    float3 spec = 0.0f;
    float ndv = dot(surface_normal, up_world);
    float ndc2 = dot(surface_normal, to_cam);
    bool spec_ok = ndv > 0.0f && ndc2 > 0.0f;
    if (spec_ok)
    {
        float r2 = coc_scale * coc_scale;
        float3 h2 = normalize(to_cam + up_world);
        float nh = dot(surface_normal, h2);
        float ndh = max(0.0f, 1.0f - dot(h2, up_world));
        float g2 = ndh * ndh;
        float denom = ((nh * nh) * (r2 - 1.0f)) + 1.0f;
        float3 fres = lerp(0.039999999105930328369140625f, 1.0f, (g2 * g2) * ndh);
        float gterm = (2.0f * ndv * ndc2) / (ndv * sqrt(((((-ndc2) * r2) + ndc2) * ndc2) + r2) + (ndc2 * sqrt(((((-ndv) * r2) + ndv) * ndv) + r2)));
        spec = (fres * gterm) * (r2 / ((3.1415927410125732421875f * denom) * denom)) / ((4.0f * ndc2) * ndv);
    }

    // Water-surface attenuation + refraction color + foam (theme 4 only).
    float3 reflected = 0.0f;
    if (depth_hit && history_valid != 0u && frame_index != 0u)
    {
        reflected = taa_history_tex.SampleLevel(sampler_lnc,
            reprojection_tex.SampleLevel(sampler_nnc, hit_uv, 0.0f).xy + hit_uv, 0.0f).xyz;
    }
    float foam = 0.0f;
    if (glade_theme_id == 4u)
    {
        float noise = perlin(world_pos.xz);
        foam = lerp(0.0f, 2.0f,
            clamp((1.0f - smoothstep(0.699999988079071044921875f, 0.85000002384185791015625f,
                water_mask.SampleLevel(sampler_llc, (world_pos.xz * 0.0076923076994717121124267578125f) + 0.5f, 0.0f).y
                + ((noise * noise) * 0.20000000298023223876953125f))) * 0.20000000298023223876953125f, 0.0f, 1.0f)
            + 0.100000001490116119384765625f);
    }
    float surface_blend = 0.60000002384185791015625f + ((0.4000000059604644775390625f
        * (exp2((-0.4000000059604644775390625f) * height_diff) * smoothstep(0.0f, 1.0f, height_diff)))
        * (waves.y + ripples.y));

    float3 water_col = (exp(-water_color * ((height_diff + length(hit_world - world_pos)) + 0.699999988079071044921875f))
        * (0.959999978542327880859375f - reflectivity)
        * refraction_tex.SampleLevel(sampler_lnc, refr_uv, 0.0f).xyz)
        * (1.0f + ((surface_blend * surface_blend) * surface_blend));
    float3 reflect_col = (reflected + ((sun_color.xyz * min(30.0f, spec)) * ((1.0f - shadow_m) * clamp(ndv, 0.0f, 1.0f))))
        * (0.039999999105930328369140625f + reflectivity);

    return float4(water_col + reflect_col + foam, 0.0f);
}
#else
float4 ps_main(PsIn vin, float4 position : SV_Position) : SV_Target0
{
    float3 world_pos = vin.C1;
    uint2 px = uint2(position.xy);
    float2 half_dims = float2(half_depth_tex_dims);

    float time = float(game_time_sec) + game_time_frac;
    float3 waves = water_wave_displacement(world_pos, time);
    float3 ripples = water_ripple_displacement(ripple_source_buffer, world_pos, time);
    float3 surface_normal = normalize(float3(waves.yz.x + ripples.yz.x / 1.0f, waves.y, waves.yz.y + ripples.yz.y / 1.0f));

    float2 ndc = ((float2(int2(px + uint2(1u, 1u))) + 0.5f) / float2(framebuffer_dims)) - 0.5f;
    ndc *= float2(2.0f, -2.0f);
    float4 pos_vs = float4(ndc, position.z, 1.0f);
    float3 view_ray = normalize(mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f))).xyz);
    uint2 half_px = px / uint2(2u, 2u);
    float4 wp4 = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, half_depth_tex.Load(int3(half_px, 0u)).x, 1.0f)));
    float3 cam_world = wp4.xyz / wp4.w;

    float3 refl_dir = normalize(view_ray - (float3(surface_normal.x, 0.0f, surface_normal.z) * 0.5f * 2.0f));
    float4 ref_end4 = mul(view_constants[0].view_to_sample,
        mul(view_constants[0].world_to_view, float4(world_pos + (refl_dir * min(4.0f, 4.0f * length(world_pos - cam_world))), 1.0f)));
    float3 ref_end = ref_end4.xyz / ref_end4.w;
    float4 march_vs = pos_vs + (float4(ref_end - pos_vs.xyz, 0.0f) * float(int(sign(ref_end.z))));
    float3 m_start = march_vs.xyz;
    float3 m_end = clip_to_slab(m_start, ref_end);
    float hit_t;
    bool depth_hit = halfres_depth_march(m_start, m_end, 3u, float2(frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px.x) + 0.0f)) + (0.005837149918079376220703125f * (float(px.y) + 0.0f)))), 0.0f), view_constants[0].clip_to_view[3].z, hit_t);
    float3 march_hit = lerp(m_start, m_end, hit_t);
    float4 wp_hit = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(march_hit.x, march_hit.y, half_depth_tex.SampleLevel(sampler_nnc, (march_hit.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x, 1.0f)));

    bool helper = IsHelperLane();
    uint quad_flag = helper ? 0u : (1u << ((px.x & 1u) + ((px.y & 1u) * 2u)));
    uint quad_sum = quad_flag + QuadReadAcrossX(quad_flag) + QuadReadAcrossY(quad_flag + QuadReadAcrossX(quad_flag));
    if ((px.x & 1u) + ((px.y & 1u) * 2u) == firstbithigh(quad_sum))
    {
        uint slot;
        __alias_water_rt_count_buffer.InterlockedAdd(0, 1u, slot);
        float3 nd = surface_normal / ((abs(surface_normal.x) + abs(surface_normal.y)) + abs(surface_normal.z));
        float3 oct;
        if (nd.z < 0.0f)
        {
            float2 o = (1.0f - abs(nd.yx)) * ((step(0.0f, nd.xy) * 2.0f) - 1.0f);
            oct = float3(o.x, o.y, nd.z);
        }
        else
        {
            oct = nd;
        }
        float2 oct_uv = (oct.xy * 0.5f) + 0.5f;
        __alias_water_rt_ray_buffer.Store<uint4>(slot * 16u + 0u,
            uint4(half_px.x + (half_px.y * 65536u),
                pack_half2x16(float2(oct_uv.x, 0.0f)) | (pack_half2x16(float2(oct_uv.y, 0.0f)) << 16u),
                asuint(dot(refl_dir, (wp_hit.xyz / wp_hit.w) - cam_world)),
                (asuint(position.z) & 0xFFFFFFF0u) + quad_sum));
    }
    return float4(0.0f, 0.0f, 0.0f, 0.0f);
}
#endif
