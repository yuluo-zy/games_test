// ============================================================
// ice_plane.raster.hlsl
// 功能:冰面平面着色(water/ice_surface_pass,索引 W45)。
//       vs_main 把冰面平面顶点变换到世界/屏幕空间(C0=(0,1,0)法线,C1=世界坐标);
//       ps_main 实现冰面渲染:六边形格点 surface_roughness SampleGrad 采样、半分辨率
//       反射光线步进、级联阴影 PCF+PCSS、时间阴影历史、折射采样、2x2 子集代表像素
//       反射发射(IsHelperLane/QuadReadAcross),输出 5 个渲染目标。
// 入口:vs_main(VsIn)、ps_main(SV_Target0..4,[earlydepthstencil])。
// 变体:两个 TGV 变体(b903e43ffb3da915 / f0adff76827e73ff)的 vs/ps 结构完全相同,仅
//       提升段路径不同;本文件提供这两个变体的像素着色器提升段。
// 绑定:u23 冰反射计数缓冲、u24 冰反射像素缓冲、t15 shadow_map、t18 half_depth_tex、
//       t19 refraction_tex、t21 shadows_history_tex、t22 surface_roughness_tex、
//       b13 $Globals(见下)、b0 space1 frame_constants。
// ============================================================
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// 说明:以下手写段为语义化版本(非 TGV 变体时编译),与下方提升段逻辑一一对应;
// 提升段把 ps_main 的六边形粗糙度采样/半分辨率反射步进/PCSS 阴影/折射全部内联展开。

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
// inc/blue_noise.hlsl no longer declares the texture (per-review register fix);
// this shader binds none, its blue-noise scroll is computed from the constants only.
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
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float4 position : SV_Position;
};

// 顶点着色器:冰面平面(y=0)顶点变换,输出世界坐标。
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

// 切线空间坐标交换 (x,y,z)->(x,-z,y)。
float3 swap_yz(float3 v)
{
    return mul(v, float3x3(float3(1.0f, -0.0f, 0.0f),
                          float3(-0.0f, 0.0f, 1.0f),
                          float3(-0.0f, -1.0f, 0.0f)));
}

// 2D 格点整数哈希(LCG 混合),最终雪崩后映射到 [0,1)。
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

// 沿 start->end 在半分辨率深度缓冲上步进,返回命中比例 hit_t(0..1) 与是否相交。
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
// 像素着色器:冰面渲染(六边形粗糙度 + 半分辨率反射步进 + PCSS 阴影 + 折射 + 反射发射)。
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

    // 六边形格点粗糙度采样(surface_roughness_tex),使用 SampleGrad。
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

    // 把步进线段裁剪到半分辨率视锥体(Slab 裁剪)。
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

    // --- 级联阴影(类 PCSS)---
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

    // 时间阴影历史(重投影到上一帧)。
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

    // 折射采样。
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

    // 向反射缓冲发射反射(每 2x2 子集一个代表像素)。
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

    // 表面高光 / 折射响应。
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

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 提升段:TGV_b903e43ffb3da915 变体,像素着色器(行为等价,语义重命名)。
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

static float undef_f;
static float undef_f2;

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
    uint current_view_index : packoffset(c0);
    uint history_valid : packoffset(c0.y);
    uint2 blue_noise_texture_dims : packoffset(c0.z);
    uint2 framebuffer_dims : packoffset(c1);
    uint2 water_mask_dims : packoffset(c1.z);
    uint2 half_depth_tex_dims : packoffset(c2);
    uint2 refraction_tex_dims : packoffset(c2.z);
    uint2 reprojection_tex_dims : packoffset(c3);
    uint2 shadows_history_tex_dims : packoffset(c3.z);
    uint2 surface_roughness_tex_dims : packoffset(c4);
    column_major float4x4 model_to_world : packoffset(c5);
};

RWByteAddressBuffer __alias_ice_refl_count_buffer : register(u23, space0);
RWByteAddressBuffer __alias_ice_refl_px_buffer : register(u24, space0);
SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);
Texture2DArray<float4> shadow_map : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t18, space0);
Texture2D<float4> refraction_tex : register(t19, space0);
Texture2D<float4> shadows_history_tex : register(t21, space0);
Texture2D<float4> surface_roughness_tex : register(t22, space0);

static float4 gl_FragCoord;
static float3 in_var_C1;
static float3 out_var_SV_TARGET0;
static float out_var_SV_TARGET1;
static uint out_var_SV_TARGET2;
static uint out_var_SV_TARGET3;
static float out_var_SV_TARGET4;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    float3 out_var_SV_TARGET0 : SV_Target0;
    float out_var_SV_TARGET1 : SV_Target1;
    uint out_var_SV_TARGET2 : SV_Target2;
    uint out_var_SV_TARGET3 : SV_Target3;
    float out_var_SV_TARGET4 : SV_Target4;
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

void frag_main()
{
    // 像素/半分辨率坐标与相机视线。
    bool helper = IsHelperLane();
    uint2 px = uint2(gl_FragCoord.xy);
    uint2 half_px = px / uint2(2u, 2u);
    float2 half_dims = float2(half_depth_tex_dims);
    float2 ndc = (((float2(int2(half_px)) + 0.5f.xx) / half_dims) - 0.5f.xx) * float2(2.0f, -2.0f);
    float ndc_x = ndc.x;
    float4 pos_vs = float4(ndc_x, ndc.y, gl_FragCoord.z, 1.0f);
    float3 view_ray = -normalize(mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(ndc_x, ndc.y, 0.0f, 1.0f))).xyz);
    float4 up_world4 = mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
    float3 up_world = up_world4.xyz;
    float2 roughness_dims = float2(surface_roughness_tex_dims);
    // 六边形格点粗糙度采样(SampleGrad)。
    float2 rough_p = in_var_C1.xz * 0.100000001490116119384765625f;
    float2 rough_hex = mul(rough_p * 3.4639999866485595703125f, float2x2(float2(1.0f, 0.0f), float2(-0.57735025882720947265625f, 1.1547005176544189453125f)));
    float2 rough_cell = floor(rough_hex);
    float2 rough_frac = frac(rough_hex);
    float rough_fx = rough_frac.x;
    float rough_fy = rough_frac.y;
    float rough_omfx = 1.0f - rough_fx;
    float rough_tri = rough_omfx - rough_fy;
    bool tri_up = rough_tri > 0.0f;
    float3 c0;
    if (tri_up)
    {
        c0 = float3(rough_cell, rough_tri);
    }
    else
    {
        c0 = float3(rough_cell + 1.0f.xx, -rough_tri);
    }
    float3 c1;
    if (tri_up)
    {
        c1 = float3(rough_cell + float2(0.0f, 1.0f), rough_fy);
    }
    else
    {
        c1 = float3(rough_cell + float2(1.0f, 0.0f), 1.0f - rough_fy);
    }
    float3 c2;
    if (tri_up)
    {
        c2 = float3(rough_cell + float2(1.0f, 0.0f), rough_fx);
    }
    else
    {
        c2 = float3(rough_cell + float2(0.0f, 1.0f), rough_omfx);
    }
    float4 grad = float4(ddx(rough_p), -ddy(rough_p));
    uint2 cell0 = uint2(c0.xy);
    uint2 h0 = ((cell0 >> uint2(8u, 8u)) ^ cell0.yx) * uint2(1103515245u, 1103515245u);
    float2 grad_x = grad.xy;
    float2 grad_y = grad.zw;
    uint2 cell1 = uint2(c1.xy);
    uint2 h1 = ((cell1 >> uint2(8u, 8u)) ^ cell1.yx) * uint2(1103515245u, 1103515245u);
    uint2 cell2 = uint2(c2.xy);
    uint2 h2 = ((cell2 >> uint2(8u, 8u)) ^ cell2.yx) * uint2(1103515245u, 1103515245u);
    float4 rough_scale4 = surface_roughness_tex.SampleLevel(sampler_llr, 0.0f.xx, (log(min(roughness_dims.x, roughness_dims.y)) * 1.44269502162933349609375f) + 1.0f);
    // 粗糙度:六边形插值后的值。
    float roughness = 0.0199999995529651641845703125f + (0.300000011920928955078125f * (1.0f - ((((((surface_roughness_tex.SampleGrad(sampler_llr, rough_p + (float2(((h0 >> uint2(8u, 8u)) ^ h0.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), grad_x, grad_y) * c0.z) + (surface_roughness_tex.SampleGrad(sampler_llr, rough_p + (float2(((h1 >> uint2(8u, 8u)) ^ h1.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), grad_x, grad_y) * c1.z)) + (surface_roughness_tex.SampleGrad(sampler_llr, rough_p + (float2(((h2 >> uint2(8u, 8u)) ^ h2.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), grad_x, grad_y) * c2.z)) - rough_scale4) / length(float3(c0.z, c1.z, c2.z)).xxxx) + rough_scale4).x));
    float3 view_dir = mul(view_ray, float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
    uint px_x = px.x;
    uint px_y = px.y;
    uint hilbert;
    hilbert = 0u;
    uint hilbert_acc;
    uint nxt_x;
    uint nxt_y;
    for (uint yi = px_y, xi = px_x, s = 32u; s > 0u; yi = nxt_x, xi = nxt_y, hilbert = hilbert_acc, s /= 2u)
    {
        uint bx = uint((xi & s) > 0u);
        uint by = uint((yi & s) > 0u);
        hilbert_acc = hilbert + ((s * s) * ((3u * bx) ^ by));
        if (by == 0u)
        {
            uint hi_y;
            uint hi_x;
            if (bx == 1u)
            {
                hi_y = 63u - yi;
                hi_x = 63u - xi;
            }
            else
            {
                hi_y = yi;
                hi_x = xi;
            }
            nxt_x = hi_x;
            nxt_y = hi_y;
        }
        else
        {
            nxt_x = yi;
            nxt_y = xi;
        }
    }
    float4x4 clip_to_view;
    float frustum_z;
    uint frame_mod = frame_constants_frame_index % 64u;
    float2 bn = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(hilbert + (288u * frame_mod))) + 0.5f.xx);
    float3 rough_dir = normalize(float3(view_dir.xy * 0.07500000298023223876953125f.xx, view_dir.z));
    float a = 6.283185482025146484375f * bn.x;
    float rough_dir_z = rough_dir.z;
    precise float c = mad(1.0f - (bn.y * 0.300000011920928955078125f), 1.0f + rough_dir_z, -rough_dir_z);
    float s_sin = sqrt(clamp(1.0f - (c * c), 0.0f, 1.0f));
    // 反射方向:粗糙锥内的随机方向。
    float3 refl_dir = float3(s_sin * cos(a), s_sin * sin(a), c) + rough_dir;
    float view_ray_y = view_ray.y;
    float om_vry = 1.0f - view_ray_y;
    float om_vry2 = om_vry * om_vry;
    float frame_rand = 5.588237762451171875f * float(frame_mod);
    // 帧随机哈希。
    float hash = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px_x) + frame_rand)) + (0.005837149918079376220703125f * (float(px_y) + frame_rand))));
    // 半分辨率深度对应世界坐标。
    float4 wp4 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(ndc_x, ndc.y, half_depth_tex.Load(int3(half_px, 0u)).x, 1.0f)));
    float3 pos_vs_xyz = pos_vs.xyz;
    // 反射步进端点(参考方向)。
    float4 ref_end4 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(in_var_C1 + ((reflect(view_ray, mul(float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)), normalize(float3(refl_dir.xy * 0.07500000298023223876953125f.xx, refl_dir.z)))) * float3(-1.0f, 1.0f, -1.0f)) * min(4.0f, 4.0f * length(in_var_C1 - (wp4.xyz / wp4.w.xxx)))), 1.0f)));
    float3 ref_dir = ref_end4.xyz / ref_end4.w.xxx;
    float4 march_vs = pos_vs + (float4(ref_dir - pos_vs_xyz, 0.0f) * float(int(sign(ref_dir.z))));
    float march_w = march_vs.w;
    float3 m0 = (march_vs / (float((march_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(march_w))).xxxx).xyz;
    float3 m1m0 = m0 - pos_vs_xyz;
    bool3 neg3 = bool3(m1m0.x < 0.0f.xxx.x, m1m0.y < 0.0f.xxx.y, m1m0.z < 0.0f.xxx.z);
    float3 t0 = (float3(neg3.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, neg3.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, neg3.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - pos_vs_xyz) / m1m0;
    float3 a0 = pos_vs_xyz + (m1m0 * max(0.0f, max(t0.x, t0.y)));
    float3 r3 = m0 - a0;
    bool3 p3 = bool3(r3.x >= 0.0f.xxx.x, r3.y >= 0.0f.xxx.y, r3.z >= 0.0f.xxx.z);
    float3 t1 = (float3(p3.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, p3.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, p3.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - a0) / r3;
    float3 a1 = a0 + (r3 * min(1.0f, min(min(t1.x, t1.y), t1.z)));
    // 半分辨率深度步进(内联的 depth_march)。
    float hit_t;
    do
    {
        clip_to_view = frame_constants_view_constants[0u].clip_to_view;
        frustum_z = clip_to_view[3].z;
        float depth_eps = 0.100000001490116119384765625f * frustum_z;
        uint steps_max = uint(int(max(2u, min(3u, uint(int(floor(length((((a1.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((a0.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * half_dims))))))));
        float dm_raymid2;
        float dm_hit_t;
        float dm_hit_diff;
        float dm_hit_eps;
        float dm_hit_ray;
        bool dm_hit;
        do
        {
            float3 dm_seg = a1 - a0;
            float dm_t_count;
            if (a1.z > 9.5367431640625e-07f)
            {
                dm_t_count = float(steps_max);
            }
            else
            {
                dm_t_count = max(1.0f, float(steps_max) - 1.0f);
            }
            float dm_inv = 1.0f / dm_t_count;
            float dm_lo2;
            float dm_hi2;
            float dm_ray2;
            float dm_eps2;
            float dm_ray2b;
            bool dm_under2b_bs;
            if (steps_max > 0u)
            {
                float dm_t_first = lerp(0.0f, 1.0f, pow(hash * dm_inv, 1.0f));
                float3 dm_sp = a0 + (dm_seg * dm_t_first);
                float dm_inv_z = 1.0f / dm_sp.z;
                float4 dm_sample = half_depth_tex.SampleLevel(sampler_nnc, (dm_sp.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float dm_d = dm_sample.x;
                float dm_diff = (1.00000202655792236328125f / dm_d) - dm_inv_z;
                bool dm_under = (dm_diff < 0.0f) ? true : false;
                float dm_lo;
                float dm_hi;
                float dm_rayhi;
                float dm_epshi;
                float dm_ray_bs;
                bool dm_under_bs;
                float dm_raymid;
                if (dm_under)
                {
                    dm_lo = 0.0f;
                    dm_hi = dm_diff;
                    dm_rayhi = dm_inv_z - (1.0f / dm_d);
                    dm_epshi = depth_eps;
                    dm_ray_bs = dm_t_first;
                    dm_under_bs = dm_under;
                    dm_raymid = 0.0f;
                }
                else
                {
                    float dm_e;
                    float dm_ray;
                    dm_e = dm_diff;
                    dm_ray = dm_t_first;
                    float dm_diff2;
                    bool dm_under2b;
                    float dm_tt;
                    float dm_diffb;
                    float dm_rayb;
                    float dm_epsb;
                    float dm_ttb;
                    bool dm_underb;
                    uint dm_i = 1u;
                    bool dm_under2 = dm_under;
                    for (;;)
                    {
                        if (dm_i < steps_max)
                        {
                            dm_tt = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(dm_i) + hash) * dm_inv, 1.0f)));
                            float3 dm_sp2 = a0 + (dm_seg * dm_tt);
                            float dm_inv_z2 = 1.0f / dm_sp2.z;
                            float4 dm_sample2 = half_depth_tex.SampleLevel(sampler_nnc, (dm_sp2.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                            float dm_d2 = dm_sample2.x;
                            dm_diff2 = (1.00000202655792236328125f / dm_d2) - dm_inv_z2;
                            dm_under2b = (dm_diff2 < 0.0f) ? true : false;
                            if (dm_under2b)
                            {
                                dm_diffb = dm_diff2;
                                dm_rayb = dm_inv_z2 - (1.0f / dm_d2);
                                dm_epsb = depth_eps;
                                dm_ttb = dm_tt;
                                dm_underb = dm_under2b;
                                break;
                            }
                            dm_i++;
                            dm_e = dm_diff2;
                            dm_under2 = dm_under2b;
                            dm_ray = dm_tt;
                            continue;
                        }
                        else
                        {
                            dm_diffb = 0.0f;
                            dm_rayb = 0.0f;
                            dm_epsb = 0.0f;
                            dm_ttb = 1.0f;
                            dm_underb = dm_under2;
                            break;
                        }
                    }
                    dm_lo = dm_e;
                    dm_hi = dm_diffb;
                    dm_rayhi = dm_rayb;
                    dm_epshi = dm_epsb;
                    dm_ray_bs = dm_ttb;
                    dm_under_bs = dm_underb;
                    dm_raymid = dm_ray;
                }
                dm_lo2 = dm_lo;
                dm_hi2 = dm_hi;
                dm_ray2 = dm_rayhi;
                dm_eps2 = dm_epshi;
                dm_ray2b = dm_ray_bs;
                dm_under2b_bs = dm_under_bs;
                dm_raymid2 = dm_raymid;
            }
            else
            {
                dm_lo2 = 0.0f;
                dm_hi2 = 0.0f;
                dm_ray2 = 0.0f;
                dm_eps2 = 0.0f;
                dm_ray2b = 1.0f;
                dm_under2b_bs = false;
                dm_raymid2 = 0.0f;
            }
            if (dm_under2b_bs)
            {
                float dm_mid;
                float dm_lo3;
                float dm_hi3;
                float dm_ray3;
                float dm_eps3;
                float dm_raymid3;
                dm_mid = dm_raymid2;
                dm_lo3 = dm_lo2;
                dm_hi3 = dm_hi2;
                dm_ray3 = dm_ray2;
                dm_eps3 = dm_eps2;
                dm_raymid3 = dm_ray2b;
                for (uint dm_j = 0u; dm_j < 1u; )
                {
                    float dm_midhi = dm_mid + dm_raymid3;
                    float dm_mid2 = dm_midhi * 0.5f;
                    float3 dm_sm = a0 + (dm_seg * dm_mid2);
                    float dm_inv_zm = 1.0f / dm_sm.z;
                    float4 dm_samplem = half_depth_tex.SampleLevel(sampler_nnc, (dm_sm.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float dm_dm = dm_samplem.x;
                    float dm_diffm = (1.00000202655792236328125f / dm_dm) - dm_inv_zm;
                    bool dm_under_m = (dm_diffm < 0.0f) ? true : false;
                    dm_mid = dm_under_m ? dm_mid : dm_mid2;
                    dm_lo3 = dm_under_m ? dm_lo3 : dm_diffm;
                    dm_hi3 = dm_under_m ? dm_diffm : dm_hi3;
                    dm_ray3 = dm_under_m ? (dm_inv_zm - (1.0f / dm_dm)) : dm_ray3;
                    dm_eps3 = dm_under_m ? depth_eps : dm_eps3;
                    dm_raymid3 = dm_under_m ? dm_mid2 : dm_raymid3;
                    dm_j++;
                    continue;
                }
                float dm_interp = lerp(dm_mid, dm_raymid3, dm_lo3 / (dm_lo3 + (-dm_hi3)));
                float3 dm_sm2 = a0 + (dm_seg * dm_interp);
                float dm_inv_z3 = 1.0f / dm_sm2.z;
                float4 dm_sample3 = half_depth_tex.SampleLevel(sampler_nnc, (dm_sm2.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float dm_d3 = dm_sample3.x;
                float dm_diff3 = (1.00000202655792236328125f / dm_d3) - dm_inv_z3;
                bool dm_conv = (abs(dm_diff3) < (dm_lo3 * 0.89999997615814208984375f)) ? true : false;
                dm_hit_t = dm_conv ? dm_interp : dm_raymid3;
                dm_hit_diff = dm_conv ? dm_diff3 : dm_hi3;
                dm_hit_eps = dm_conv ? depth_eps : dm_eps3;
                dm_hit_ray = dm_conv ? (dm_inv_z3 - (1.0f / dm_d3)) : dm_ray3;
                dm_hit = true;
                break;
            }
            else
            {
                dm_hit_t = dm_raymid2;
                dm_hit_diff = undef_f;
                dm_hit_eps = undef_f;
                dm_hit_ray = undef_f;
                dm_hit = false;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool dm_ok2;
        if (dm_hit)
        {
            bool dm_ok;
            if (dm_hit_ray < dm_hit_eps)
            {
                dm_ok = dm_hit_diff < dm_hit_eps;
            }
            else
            {
                dm_ok = false;
            }
            dm_ok2 = dm_ok;
        }
        else
        {
            dm_ok2 = false;
        }
        if (dm_ok2)
        {
            hit_t = dm_hit_t;
            break;
        }
        hit_t = dm_raymid2;
        break;
    } while(false);
    // 命中点:由步进结果插值。
    float3 march_hit = lerp(a0, a1, hit_t.xxx);
    float march_hit_x = march_hit.x;
    float2 hit_uv = (float4(march_hit_x, march_hit.y, undef_f2, 1.0f).xy * float2(0.5f, -0.5f)) + 0.5f.xx;
    float4 wp_hit = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(march_hit_x, march_hit.y, half_depth_tex.SampleLevel(sampler_nnc, hit_uv, 0.0f).x, 1.0f)));
    float3 hit_world = wp_hit.xyz / wp_hit.w.xxx;
    float3 hit_offset = hit_world - in_var_C1;
    float3 hit_normal = -up_world;
    int2 px_i = int2(px);
    // 级联阴影 PCSS。
    float2 shadow;
    float shadow_blur2;
    do
    {
        float4 sc = mul(frame_constants_view_constants[2u].world_to_sample, float4(in_var_C1, 1.0f));
        if (sc.z > 1.0f)
        {
            shadow = 0.0f.xx;
            shadow_blur2 = 0.0f;
            break;
        }
        float2 sc_uv = sc.xy;
        float2 sc_abs = abs(sc_uv);
        bool out0 = any(bool2(sc_abs.x > 0.9833333492279052734375f.xx.x, sc_abs.y > 0.9833333492279052734375f.xx.y));
        float2 uv1;
        if (out0)
        {
            uv1 = sc_uv * 0.5f;
        }
        else
        {
            uv1 = sc_uv;
        }
        float texel_scale = out0 ? 5.0f : 10.0f;
        float sample_scale = out0 ? 0.5f : 1.0f;
        float2 uv1_abs = abs(uv1);
        float2 edge = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * texel_scale));
        bool out1 = any(bool2(uv1_abs.x > edge.x, uv1_abs.y > edge.y));
        float texel_scale2;
        float2 uv1_2;
        float sample_scale2;
        if (out1)
        {
            texel_scale2 = texel_scale * 0.2857142984867095947265625f;
            uv1_2 = uv1 * 0.2857142984867095947265625f;
            sample_scale2 = sample_scale * 0.2857142984867095947265625f;
        }
        else
        {
            texel_scale2 = texel_scale;
            uv1_2 = uv1;
            sample_scale2 = sample_scale;
        }
        float slice = float(out1 ? 2u : uint(out0));
        uint slice_idx = 2u + uint(slice);
        float ndl = hit_normal.y;
        float4 proj = float4(uv1_2.x, uv1_2.y, sc.z, sc.w) + mul(frame_constants_view_constants[slice_idx].world_to_sample, float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - ndl, 0.0f, 1.0f))) / sample_scale2.xxx, 0.0f));
        float2 shadow_uv = (proj.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        float3 shadow_xyz = float3(shadow_uv.x, shadow_uv.y, proj.z);
        float bias = max((0.300000011920928955078125f / sample_scale2) * (1.0f - abs(ndl)), 0.02500000037252902984619140625f);
        float spiral_seed = float((uint((px_i.x * 11) + (px_i.y * 7)) + (frame_constants_frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
        uint2 px_u = uint2(px_i);
        float hash2 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px_u.x) + frame_rand)) + (0.005837149918079376220703125f * (float(px_u.y) + frame_rand))));
        float4 up4 = mul(frame_constants_view_constants[slice_idx].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));
        float blocked;
        int hits;
        blocked = 0.0f;
        hits = 0;
        float blocked2;
        int hits2;
        for (int i = 0; i < 5; blocked = blocked2, hits = hits2, i++)
        {
            float i_f = float(i);
            float ang = (i_f + spiral_seed) * 2.3999631404876708984375f;
            float3 sp = shadow_xyz + float3((((float2(cos(ang), sin(ang)) * (i_f + hash2)) * 0.20000000298023223876953125f.xx) * texel_scale2) * 0.000833333353511989116668701171875f.xx, bias * (-0.0009765625f));
            float4 sp_sample = shadow_map.SampleLevel(sampler_shadow_nearest, float3(sp.xy, slice), 0.0f);
            float d = sp_sample.x;
            if (sp.z > d)
            {
                blocked2 = blocked + d;
                hits2 = hits + 1;
            }
            else
            {
                blocked2 = blocked;
                hits2 = hits;
            }
        }
        float avg = blocked / float(hits);
        if (hits == 0)
        {
            shadow = 0.0f.xx;
            shadow_blur2 = 0.0f;
            break;
        }
        float pen = max(((proj.z - avg) * ((17.0666675567626953125f * texel_scale2) * pow(10.0f, smoothstep(0.0f, 0.25f, frame_constants_fog_tint_thickness.w)))) / avg, 0.0f);
        float shadow_blur = (pen / pow(pow(pen / (texel_scale2 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
        float up4_z = up4.z;
        float2 up_uv = (((up4.xy * float(int(sign(up4_z)))) / max(0.1500000059604644775390625f, abs(up4_z)).xx) * 0.0009765625f.xx) * 2.0f;
        float2 acc;
        acc = 0.0f.xx;
        for (int j = 0; j < 10; )
        {
            float j_f = float(j);
            float ang2 = (j_f + spiral_seed) * 2.3999631404876708984375f;
            float2 sp2 = ((float2(cos(ang2), sin(ang2)) * (j_f + hash2)) * 0.100000001490116119384765625f.xx) * shadow_blur;
            float3 scp = shadow_xyz + float3(sp2 * 0.000833333353511989116668701171875f.xx, bias * (-0.0009765625f));
            float lit = 1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(scp.xy, slice), scp.z + min(0.0f, dot(up_uv, sp2)));
            acc += float2(lit, lit * lit);
            j++;
            continue;
        }
        shadow = acc * 0.100000001490116119384765625f;
        shadow_blur2 = shadow_blur;
        break;
    } while(false);
    // 阴影均值与方差。
    float shadow_m = clamp((shadow.x - 0.100000001490116119384765625f) * 1.4285714626312255859375f, 0.0f, 1.0f);
    float2 shadow_std = shadow_m.xx + ((float2(-1.0f, 1.0f) * sqrt(max(0.0f, shadow.y - (shadow.x * shadow.x)))) * (3.0f * max(0.0f, 1.0f - (shadow_blur2 * 0.20000000298023223876953125f))));
    // 时间阴影历史:重投影到上一帧。
    float2 fb_uv = (float2(px_i) + 0.5f.xx) / float2(framebuffer_dims);
    float4 pos_ndc4 = float4((fb_uv - 0.5f.xx) * float2(2.0f, -2.0f), gl_FragCoord.z, 1.0f);
    float4 prev4 = mul(frame_constants_view_constants[0u].clip_to_prev_clip, mul(frame_constants_view_constants[0u].view_to_clip, mul(clip_to_view, pos_ndc4)));
    float2 prev_uv = ((prev4.xy / prev4.w.xx) * float2(0.5f, -0.5f)) + 0.5f.xx;
    float2 prev_clamped = clamp(prev_uv, 0.0f.xx, 1.0f.xx);
    float4 motion = float4(prev_uv - ((pos_ndc4.xy * float2(0.5f, -0.5f)) + 0.5f.xx), float(all(bool2(prev_clamped.x == prev_uv.x, prev_clamped.y == prev_uv.y))), 1.0f) * float4(1.0f, 1.0f, clamp(1.0f - length(frame_constants_view_constants[0u].clip_to_prev_clip[3].xyz), 0.0f, 1.0f), 1.0f);
    float motion_z = motion.z;
    bool hist_ok2;
    if (motion_z > 0.0f)
    {
        bool hist_ok;
        if (history_valid != 0u)
        {
            hist_ok = !(0u == frame_constants_frame_index);
        }
        else
        {
            hist_ok = false;
        }
        hist_ok2 = hist_ok;
    }
    else
    {
        hist_ok2 = false;
    }
    float shadow_hist;
    if (hist_ok2)
    {
        shadow_hist = shadows_history_tex.SampleLevel(sampler_lnc, fb_uv + motion.xy, 0.0f).x;
    }
    else
    {
        shadow_hist = shadow_m;
    }
    float shadow_temporal = lerp(clamp(shadow_hist, shadow_std.x, shadow_std.y), shadow_m, lerp(1.0f, 0.125f, motion_z));
    float inv_shadow = 1.0f - shadow_temporal;
    // 折射采样:向水面下方步进。
    float4 rp = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(in_var_C1 + (up_world * 0.5f), 1.0f)));
    float3 rp_w = rp.xyz / rp.w.xxx;
    float4 rv = pos_vs + (float4(rp_w - pos_vs_xyz, 0.0f) * float(int(sign(rp_w.z))));
    float rv_w = rv.w;
    float3 r0 = (rv / (float((rv_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(rv_w))).xxxx).xyz;
    float3 r1m0 = r0 - pos_vs_xyz;
    bool3 rneg = bool3(r1m0.x < 0.0f.xxx.x, r1m0.y < 0.0f.xxx.y, r1m0.z < 0.0f.xxx.z);
    float3 rt0 = (float3(rneg.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, rneg.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, rneg.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - pos_vs_xyz) / r1m0;
    float3 ra0 = pos_vs_xyz + (r1m0 * max(0.0f, max(rt0.x, rt0.y)));
    float3 rr3 = r0 - ra0;
    bool3 rp3 = bool3(rr3.x >= 0.0f.xxx.x, rr3.y >= 0.0f.xxx.y, rr3.z >= 0.0f.xxx.z);
    float3 rt1 = (float3(rp3.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, rp3.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, rp3.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - ra0) / rr3;
    float3 ra1 = ra0 + (rr3 * min(1.0f, min(min(rt1.x, rt1.y), rt1.z)));
    bool refr_hit;
    float refr_hit_t;
    float refr_diff;
    do
    {
        float refr_halfz = 0.5f * frustum_z;
        uint refr_steps = uint(int(max(2u, min(3u, uint(int(floor(length((((ra1.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((ra0.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * half_dims))))))));
        float rf_raymid2;
        float rf_hit_ray;
        float rf_hit_lo;
        float rf_hit_hi;
        float rf_hit_eps;
        bool rf_hit;
        do
        {
            float3 rf_seg = ra1 - ra0;
            float rf_t_count;
            if (ra1.z > 9.5367431640625e-07f)
            {
                rf_t_count = float(refr_steps);
            }
            else
            {
                rf_t_count = max(1.0f, float(refr_steps) - 1.0f);
            }
            float rf_inv = 1.0f / rf_t_count;
            float rf_lo2;
            float rf_hi2;
            float rf_eps2;
            float rf_ray2;
            bool rf_under2c;
            if (refr_steps > 0u)
            {
                float rf_t_first = lerp(0.0f, 1.0f, pow(hash * rf_inv, 1.0f));
                float3 rf_sp = ra0 + (rf_seg * rf_t_first);
                float2 rf_uv = (rf_sp.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                float rf_inv_z = 1.0f / rf_sp.z;
                float rf_rd_lo = 1.0f / half_depth_tex.SampleLevel(sampler_llc, rf_uv, 0.0f).x;
                float rf_rd_hi = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, rf_uv, 0.0f).x;
                float rf_ddiff = (max(rf_rd_lo, rf_rd_hi) * 1.00000202655792236328125f) - rf_inv_z;
                float rf_ddiff2 = rf_inv_z - min(rf_rd_lo, rf_rd_hi);
                bool rf_under = (rf_ddiff < 0.0f) ? (rf_ddiff2 < refr_halfz) : false;
                float rf_lo;
                float rf_hi;
                float rf_epshi;
                float rf_ray;
                bool rf_under_bs;
                float rf_raymid;
                if (rf_under)
                {
                    rf_lo = rf_ddiff;
                    rf_hi = rf_ddiff2;
                    rf_epshi = refr_halfz;
                    rf_ray = rf_t_first;
                    rf_under_bs = rf_under;
                    rf_raymid = 0.0f;
                }
                else
                {
                    float rf_e;
                    rf_e = rf_t_first;
                    bool rf_under2b;
                    float rf_tt;
                    float rf_diffb;
                    float rf_ddiffb;
                    float rf_epsb;
                    float rf_ttb;
                    bool rf_underb;
                    uint rf_i = 1u;
                    bool rf_under2 = rf_under;
                    for (;;)
                    {
                        if (rf_i < refr_steps)
                        {
                            rf_tt = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(rf_i) + hash) * rf_inv, 1.0f)));
                            float3 rf_sp2 = ra0 + (rf_seg * rf_tt);
                            float2 rf_uv2 = (rf_sp2.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                            float rf_inv_z2 = 1.0f / rf_sp2.z;
                            float rf_rd_lo2 = 1.0f / half_depth_tex.SampleLevel(sampler_llc, rf_uv2, 0.0f).x;
                            float rf_rd_hi2 = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, rf_uv2, 0.0f).x;
                            float rf_ddiff2b = (max(rf_rd_lo2, rf_rd_hi2) * 1.00000202655792236328125f) - rf_inv_z2;
                            float rf_ddiff22 = rf_inv_z2 - min(rf_rd_lo2, rf_rd_hi2);
                            rf_under2b = (rf_ddiff2b < 0.0f) ? (rf_ddiff22 < refr_halfz) : false;
                            if (rf_under2b)
                            {
                                rf_diffb = rf_ddiff2b;
                                rf_ddiffb = rf_ddiff22;
                                rf_epsb = refr_halfz;
                                rf_ttb = rf_tt;
                                rf_underb = rf_under2b;
                                break;
                            }
                            rf_i++;
                            rf_under2 = rf_under2b;
                            rf_e = rf_tt;
                            continue;
                        }
                        else
                        {
                            rf_diffb = 0.0f;
                            rf_ddiffb = 0.0f;
                            rf_epsb = 0.0f;
                            rf_ttb = 1.0f;
                            rf_underb = rf_under2;
                            break;
                        }
                    }
                    rf_lo = rf_diffb;
                    rf_hi = rf_ddiffb;
                    rf_epshi = rf_epsb;
                    rf_ray = rf_ttb;
                    rf_under_bs = rf_underb;
                    rf_raymid = rf_e;
                }
                rf_lo2 = rf_lo;
                rf_hi2 = rf_hi;
                rf_eps2 = rf_epshi;
                rf_ray2 = rf_ray;
                rf_under2c = rf_under_bs;
                rf_raymid2 = rf_raymid;
            }
            else
            {
                rf_lo2 = 0.0f;
                rf_hi2 = 0.0f;
                rf_eps2 = 0.0f;
                rf_ray2 = 1.0f;
                rf_under2c = false;
                rf_raymid2 = 0.0f;
            }
            if (rf_under2c)
            {
                rf_hit_ray = rf_ray2;
                rf_hit_lo = rf_lo2;
                rf_hit_hi = rf_eps2;
                rf_hit_eps = rf_hi2;
                rf_hit = true;
                break;
            }
            else
            {
                rf_hit_ray = rf_raymid2;
                rf_hit_lo = undef_f;
                rf_hit_hi = undef_f;
                rf_hit_eps = undef_f;
                rf_hit = false;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool rf_ok2;
        if (rf_hit)
        {
            bool rf_ok;
            if (rf_hit_eps < rf_hit_hi)
            {
                rf_ok = rf_hit_lo < rf_hit_hi;
            }
            else
            {
                rf_ok = false;
            }
            rf_ok2 = rf_ok;
        }
        else
        {
            rf_ok2 = false;
        }
        if (rf_ok2)
        {
            refr_hit = true;
            refr_hit_t = rf_hit_ray;
            refr_diff = rf_hit_eps / rf_hit_hi;
            break;
        }
        refr_hit = false;
        refr_hit_t = rf_raymid2;
        refr_diff = 0.0f;
        break;
    } while(false);
    // 折射因子:命中则按差异平滑衰减。
    float refr_factor;
    if (refr_hit)
    {
        refr_factor = min(inv_shadow, smoothstep(0.5f, 0.0f, (1.0f - refr_diff) * (1.0f - refr_hit_t)));
    }
    else
    {
        refr_factor = inv_shadow;
    }
    // 反射发射:2x2 子集一个代表像素写反射缓冲。
    uint quad_flag;
    if (helper)
    {
        quad_flag = 0u;
    }
    else
    {
        uint2 quad_lane = px & uint2(1u, 1u);
        quad_flag = 1u << ((quad_lane.x + (quad_lane.y * 2u)) & 31u);
    }
    uint quad_sum = quad_flag + QuadReadAcrossX(quad_flag);
    uint2 quad_id = px & uint2(1u, 1u);
    if ((quad_id.x + (quad_id.y * 2u)) == firstbithigh(quad_sum + QuadReadAcrossY(quad_sum)))
    {
        uint slot;
        __alias_ice_refl_count_buffer.InterlockedAdd(12, 1u, slot);
        __alias_ice_refl_px_buffer.Store<uint>(slot * 4 + 0, half_px.x + (half_px.y * 65536u));
        if ((slot & 63u) == 0u)
        {
            uint group;
            __alias_ice_refl_count_buffer.InterlockedAdd(0, 1u, group);
        }
    }
    // 高光 BRDF(冰面)。
    float up_w;
    float3 spec;
    do
    {
        up_w = up_world4.y;
        bool spec_ok;
        if (!(up_w <= 0.0f))
        {
            spec_ok = view_ray_y <= 0.0f;
        }
        else
        {
            spec_ok = true;
        }
        if (spec_ok)
        {
            spec = 0.0f.xxx;
            break;
        }
        float r2 = roughness * roughness;
        float3 spec_h2 = normalize(view_ray + up_world);
        float h2_y = spec_h2.y;
        float ndh = max(0.0f, 1.0f - dot(spec_h2, up_world));
        float g2 = ndh * ndh;
        float denom_w = ((h2_y * h2_y) * (r2 - 1.0f)) + 1.0f;
        spec = ((lerp(0.02999999932944774627685546875f.xxx, 1.0f.xxx, ((g2 * g2) * ndh).xxx) * (((2.0f * up_w) * view_ray_y) / ((up_w * sqrt(((((-view_ray_y) * r2) + view_ray_y) * view_ray_y) + r2)) + (view_ray_y * sqrt(((((-up_w) * r2) + up_w) * up_w) + r2))))) * (r2 / ((3.1415927410125732421875f * denom_w) * denom_w))) / ((4.0f * view_ray_y) * up_w).xxx;
        break;
    } while(false);
    float3 hit_off_swap = mul(hit_offset * float3(1.0f, -1.0f, 1.0f), float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
    float hit_len = length(hit_off_swap);
    float3 hit_n = hit_off_swap * (1.0f / hit_len);
    float3 refr_col = clamp((exp(float3(-0.5950000286102294921875f, -0.374000012874603271484375f, -0.3060000240802764892578125f) * ((abs(in_var_C1.y - hit_world.y) + length(hit_offset)) + 0.699999988079071044921875f)) * (0.959999978542327880859375f - (0.959999978542327880859375f * ((om_vry2 * om_vry2) * om_vry)))) * refraction_tex.SampleLevel(sampler_lnc, hit_uv, 0.0f).xyz, 0.0f.xxx, 65408.0f.xxx);
    float scale = asfloat((asuint(max(max(1.52587890625e-05f, refr_col.x), max(refr_col.y, refr_col.z))) + 125845504u) & 2139095040u);
    uint3 mantissa = asuint(refr_col + scale.xxx);
    // 输出 5 个渲染目标。
    out_var_SV_TARGET0 = lerp((frame_constants_sun_color.xyz * spec.x) * (refr_factor * clamp(up_w, 0.0f, 1.0f)), ((frame_constants_sun_color.xyz * (0.699999988079071044921875f * refr_factor)) * 0.3183098733425140380859375f.xxx) + frame_constants_sky_high_color.xyz, 0.00999999977648258209228515625f.xxx);
    out_var_SV_TARGET1 = sqrt(roughness);
    out_var_SV_TARGET2 = ((uint((clamp((hit_n.x * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u) | (uint((clamp((hit_n.y * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) | spvPackHalf2x16(float2(hit_len, 0.0f));
    out_var_SV_TARGET3 = ((((asuint(scale) << 4u) + 268435456u) | (mantissa.z << 18u)) | (mantissa.y << 9u)) | (mantissa.x & 511u);
    out_var_SV_TARGET4 = shadow_temporal + 0.00196078442968428134918212890625f;
}

[earlydepthstencil]
SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    stage_output.out_var_SV_TARGET1 = out_var_SV_TARGET1;
    stage_output.out_var_SV_TARGET2 = out_var_SV_TARGET2;
    stage_output.out_var_SV_TARGET3 = out_var_SV_TARGET3;
    stage_output.out_var_SV_TARGET4 = out_var_SV_TARGET4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 提升段:TGV_f0adff76827e73ff 变体,像素着色器(行为等价,语义重命名)。
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

static float undef_f;
static float undef_f2;

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
    uint current_view_index : packoffset(c0);
    uint history_valid : packoffset(c0.y);
    uint2 blue_noise_texture_dims : packoffset(c0.z);
    uint2 framebuffer_dims : packoffset(c1);
    uint2 water_mask_dims : packoffset(c1.z);
    uint2 half_depth_tex_dims : packoffset(c2);
    uint2 refraction_tex_dims : packoffset(c2.z);
    uint2 reprojection_tex_dims : packoffset(c3);
    uint2 shadows_history_tex_dims : packoffset(c3.z);
    uint2 surface_roughness_tex_dims : packoffset(c4);
    column_major float4x4 model_to_world : packoffset(c5);
};

RWByteAddressBuffer __alias_ice_refl_count_buffer : register(u23, space0);
RWByteAddressBuffer __alias_ice_refl_px_buffer : register(u24, space0);
SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);
Texture2DArray<float4> shadow_map : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t18, space0);
Texture2D<float4> refraction_tex : register(t19, space0);
Texture2D<float4> shadows_history_tex : register(t21, space0);
Texture2D<float4> surface_roughness_tex : register(t22, space0);

static float4 gl_FragCoord;
static float3 in_var_C1;
static float3 out_var_SV_TARGET0;
static float out_var_SV_TARGET1;
static uint out_var_SV_TARGET2;
static uint out_var_SV_TARGET3;
static float out_var_SV_TARGET4;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    float3 out_var_SV_TARGET0 : SV_Target0;
    float out_var_SV_TARGET1 : SV_Target1;
    uint out_var_SV_TARGET2 : SV_Target2;
    uint out_var_SV_TARGET3 : SV_Target3;
    float out_var_SV_TARGET4 : SV_Target4;
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

void frag_main()
{
    // 像素/半分辨率坐标与相机视线。
    bool helper = IsHelperLane();
    uint2 px = uint2(gl_FragCoord.xy);
    uint2 half_px = px / uint2(2u, 2u);
    float2 half_dims = float2(half_depth_tex_dims);
    float2 ndc = (((float2(int2(half_px)) + 0.5f.xx) / half_dims) - 0.5f.xx) * float2(2.0f, -2.0f);
    float ndc_x = ndc.x;
    float4 pos_vs = float4(ndc_x, ndc.y, gl_FragCoord.z, 1.0f);
    float3 view_ray = -normalize(mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(ndc_x, ndc.y, 0.0f, 1.0f))).xyz);
    float4 up_world4 = mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
    float3 up_world = up_world4.xyz;
    float2 roughness_dims = float2(surface_roughness_tex_dims);
    // 六边形格点粗糙度采样(SampleGrad)。
    float2 rough_p = in_var_C1.xz * 0.100000001490116119384765625f;
    float2 rough_hex = mul(rough_p * 3.4639999866485595703125f, float2x2(float2(1.0f, 0.0f), float2(-0.57735025882720947265625f, 1.1547005176544189453125f)));
    float2 rough_cell = floor(rough_hex);
    float2 rough_frac = frac(rough_hex);
    float rough_fx = rough_frac.x;
    float rough_fy = rough_frac.y;
    float rough_omfx = 1.0f - rough_fx;
    float rough_tri = rough_omfx - rough_fy;
    bool tri_up = rough_tri > 0.0f;
    float3 c0;
    if (tri_up)
    {
        c0 = float3(rough_cell, rough_tri);
    }
    else
    {
        c0 = float3(rough_cell + 1.0f.xx, -rough_tri);
    }
    float3 c1;
    if (tri_up)
    {
        c1 = float3(rough_cell + float2(0.0f, 1.0f), rough_fy);
    }
    else
    {
        c1 = float3(rough_cell + float2(1.0f, 0.0f), 1.0f - rough_fy);
    }
    float3 c2;
    if (tri_up)
    {
        c2 = float3(rough_cell + float2(1.0f, 0.0f), rough_fx);
    }
    else
    {
        c2 = float3(rough_cell + float2(0.0f, 1.0f), rough_omfx);
    }
    float4 grad = float4(ddx(rough_p), -ddy(rough_p));
    uint2 cell0 = uint2(c0.xy);
    uint2 h0 = ((cell0 >> uint2(8u, 8u)) ^ cell0.yx) * uint2(1103515245u, 1103515245u);
    float2 grad_x = grad.xy;
    float2 grad_y = grad.zw;
    uint2 cell1 = uint2(c1.xy);
    uint2 h1 = ((cell1 >> uint2(8u, 8u)) ^ cell1.yx) * uint2(1103515245u, 1103515245u);
    uint2 cell2 = uint2(c2.xy);
    uint2 h2 = ((cell2 >> uint2(8u, 8u)) ^ cell2.yx) * uint2(1103515245u, 1103515245u);
    float4 rough_scale4 = surface_roughness_tex.SampleLevel(sampler_llr, 0.0f.xx, (log(min(roughness_dims.x, roughness_dims.y)) * 1.44269502162933349609375f) + 1.0f);
    // 粗糙度:六边形插值后的值。
    float roughness = 0.0199999995529651641845703125f + (0.300000011920928955078125f * (1.0f - ((((((surface_roughness_tex.SampleGrad(sampler_llr, rough_p + (float2(((h0 >> uint2(8u, 8u)) ^ h0.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), grad_x, grad_y) * c0.z) + (surface_roughness_tex.SampleGrad(sampler_llr, rough_p + (float2(((h1 >> uint2(8u, 8u)) ^ h1.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), grad_x, grad_y) * c1.z)) + (surface_roughness_tex.SampleGrad(sampler_llr, rough_p + (float2(((h2 >> uint2(8u, 8u)) ^ h2.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), grad_x, grad_y) * c2.z)) - rough_scale4) / length(float3(c0.z, c1.z, c2.z)).xxxx) + rough_scale4).x));
    float3 view_dir = mul(view_ray, float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
    uint px_x = px.x;
    uint px_y = px.y;
    uint hilbert;
    hilbert = 0u;
    uint hilbert_acc;
    uint nxt_x;
    uint nxt_y;
    for (uint yi = px_y, xi = px_x, s = 32u; s > 0u; yi = nxt_x, xi = nxt_y, hilbert = hilbert_acc, s /= 2u)
    {
        uint bx = uint((xi & s) > 0u);
        uint by = uint((yi & s) > 0u);
        hilbert_acc = hilbert + ((s * s) * ((3u * bx) ^ by));
        if (by == 0u)
        {
            uint hi_y;
            uint hi_x;
            if (bx == 1u)
            {
                hi_y = 63u - yi;
                hi_x = 63u - xi;
            }
            else
            {
                hi_y = yi;
                hi_x = xi;
            }
            nxt_x = hi_x;
            nxt_y = hi_y;
        }
        else
        {
            nxt_x = yi;
            nxt_y = xi;
        }
    }
    float4x4 clip_to_view;
    float frustum_z;
    uint frame_mod = frame_constants_frame_index % 64u;
    float2 bn = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(hilbert + (288u * frame_mod))) + 0.5f.xx);
    float3 rough_dir = normalize(float3(view_dir.xy * 0.07500000298023223876953125f.xx, view_dir.z));
    float a = 6.283185482025146484375f * bn.x;
    float rough_dir_z = rough_dir.z;
    precise float c = mad(1.0f - (bn.y * 0.300000011920928955078125f), 1.0f + rough_dir_z, -rough_dir_z);
    float s_sin = sqrt(clamp(1.0f - (c * c), 0.0f, 1.0f));
    // 反射方向:粗糙锥内的随机方向。
    float3 refl_dir = float3(s_sin * cos(a), s_sin * sin(a), c) + rough_dir;
    float view_ray_y = view_ray.y;
    float om_vry = 1.0f - view_ray_y;
    float om_vry2 = om_vry * om_vry;
    float frame_rand = 5.588237762451171875f * float(frame_mod);
    // 帧随机哈希。
    float hash = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px_x) + frame_rand)) + (0.005837149918079376220703125f * (float(px_y) + frame_rand))));
    // 半分辨率深度对应世界坐标。
    float4 wp4 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(ndc_x, ndc.y, half_depth_tex.Load(int3(half_px, 0u)).x, 1.0f)));
    float3 pos_vs_xyz = pos_vs.xyz;
    // 反射步进端点(参考方向)。
    float4 ref_end4 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(in_var_C1 + ((reflect(view_ray, mul(float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)), normalize(float3(refl_dir.xy * 0.07500000298023223876953125f.xx, refl_dir.z)))) * float3(-1.0f, 1.0f, -1.0f)) * min(4.0f, 4.0f * length(in_var_C1 - (wp4.xyz / wp4.w.xxx)))), 1.0f)));
    float3 ref_dir = ref_end4.xyz / ref_end4.w.xxx;
    float4 march_vs = pos_vs + (float4(ref_dir - pos_vs_xyz, 0.0f) * float(int(sign(ref_dir.z))));
    float march_w = march_vs.w;
    float3 m0 = (march_vs / (float((march_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(march_w))).xxxx).xyz;
    float3 m1m0 = m0 - pos_vs_xyz;
    bool3 neg3 = bool3(m1m0.x < 0.0f.xxx.x, m1m0.y < 0.0f.xxx.y, m1m0.z < 0.0f.xxx.z);
    float3 t0 = (float3(neg3.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, neg3.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, neg3.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - pos_vs_xyz) / m1m0;
    float3 a0 = pos_vs_xyz + (m1m0 * max(0.0f, max(t0.x, t0.y)));
    float3 r3 = m0 - a0;
    bool3 p3 = bool3(r3.x >= 0.0f.xxx.x, r3.y >= 0.0f.xxx.y, r3.z >= 0.0f.xxx.z);
    float3 t1 = (float3(p3.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, p3.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, p3.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - a0) / r3;
    float3 a1 = a0 + (r3 * min(1.0f, min(min(t1.x, t1.y), t1.z)));
    // 半分辨率深度步进(内联的 depth_march)。
    float hit_t;
    do
    {
        clip_to_view = frame_constants_view_constants[0u].clip_to_view;
        frustum_z = clip_to_view[3].z;
        float depth_eps = 0.100000001490116119384765625f * frustum_z;
        uint steps_max = uint(int(max(2u, min(3u, uint(int(floor(length((((a1.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((a0.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * half_dims))))))));
        float dm_raymid2;
        float dm_hit_t;
        float dm_hit_diff;
        float dm_hit_eps;
        float dm_hit_ray;
        bool dm_hit;
        do
        {
            float3 dm_seg = a1 - a0;
            float dm_t_count;
            if (a1.z > 9.5367431640625e-07f)
            {
                dm_t_count = float(steps_max);
            }
            else
            {
                dm_t_count = max(1.0f, float(steps_max) - 1.0f);
            }
            float dm_inv = 1.0f / dm_t_count;
            float dm_lo2;
            float dm_hi2;
            float dm_ray2;
            float dm_eps2;
            float dm_ray2b;
            bool dm_under2b_bs;
            if (steps_max > 0u)
            {
                float dm_t_first = lerp(0.0f, 1.0f, pow(hash * dm_inv, 1.0f));
                float3 dm_sp = a0 + (dm_seg * dm_t_first);
                float dm_inv_z = 1.0f / dm_sp.z;
                float4 dm_sample = half_depth_tex.SampleLevel(sampler_nnc, (dm_sp.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float dm_d = dm_sample.x;
                float dm_diff = (1.00000202655792236328125f / dm_d) - dm_inv_z;
                bool dm_under = (dm_diff < 0.0f) ? true : false;
                float dm_lo;
                float dm_hi;
                float dm_rayhi;
                float dm_epshi;
                float dm_ray_bs;
                bool dm_under_bs;
                float dm_raymid;
                if (dm_under)
                {
                    dm_lo = 0.0f;
                    dm_hi = dm_diff;
                    dm_rayhi = dm_inv_z - (1.0f / dm_d);
                    dm_epshi = depth_eps;
                    dm_ray_bs = dm_t_first;
                    dm_under_bs = dm_under;
                    dm_raymid = 0.0f;
                }
                else
                {
                    float dm_e;
                    float dm_ray;
                    dm_e = dm_diff;
                    dm_ray = dm_t_first;
                    float dm_diff2;
                    bool dm_under2b;
                    float dm_tt;
                    float dm_diffb;
                    float dm_rayb;
                    float dm_epsb;
                    float dm_ttb;
                    bool dm_underb;
                    uint dm_i = 1u;
                    bool dm_under2 = dm_under;
                    for (;;)
                    {
                        if (dm_i < steps_max)
                        {
                            dm_tt = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(dm_i) + hash) * dm_inv, 1.0f)));
                            float3 dm_sp2 = a0 + (dm_seg * dm_tt);
                            float dm_inv_z2 = 1.0f / dm_sp2.z;
                            float4 dm_sample2 = half_depth_tex.SampleLevel(sampler_nnc, (dm_sp2.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                            float dm_d2 = dm_sample2.x;
                            dm_diff2 = (1.00000202655792236328125f / dm_d2) - dm_inv_z2;
                            dm_under2b = (dm_diff2 < 0.0f) ? true : false;
                            if (dm_under2b)
                            {
                                dm_diffb = dm_diff2;
                                dm_rayb = dm_inv_z2 - (1.0f / dm_d2);
                                dm_epsb = depth_eps;
                                dm_ttb = dm_tt;
                                dm_underb = dm_under2b;
                                break;
                            }
                            dm_i++;
                            dm_e = dm_diff2;
                            dm_under2 = dm_under2b;
                            dm_ray = dm_tt;
                            continue;
                        }
                        else
                        {
                            dm_diffb = 0.0f;
                            dm_rayb = 0.0f;
                            dm_epsb = 0.0f;
                            dm_ttb = 1.0f;
                            dm_underb = dm_under2;
                            break;
                        }
                    }
                    dm_lo = dm_e;
                    dm_hi = dm_diffb;
                    dm_rayhi = dm_rayb;
                    dm_epshi = dm_epsb;
                    dm_ray_bs = dm_ttb;
                    dm_under_bs = dm_underb;
                    dm_raymid = dm_ray;
                }
                dm_lo2 = dm_lo;
                dm_hi2 = dm_hi;
                dm_ray2 = dm_rayhi;
                dm_eps2 = dm_epshi;
                dm_ray2b = dm_ray_bs;
                dm_under2b_bs = dm_under_bs;
                dm_raymid2 = dm_raymid;
            }
            else
            {
                dm_lo2 = 0.0f;
                dm_hi2 = 0.0f;
                dm_ray2 = 0.0f;
                dm_eps2 = 0.0f;
                dm_ray2b = 1.0f;
                dm_under2b_bs = false;
                dm_raymid2 = 0.0f;
            }
            if (dm_under2b_bs)
            {
                float dm_mid;
                float dm_lo3;
                float dm_hi3;
                float dm_ray3;
                float dm_eps3;
                float dm_raymid3;
                dm_mid = dm_raymid2;
                dm_lo3 = dm_lo2;
                dm_hi3 = dm_hi2;
                dm_ray3 = dm_ray2;
                dm_eps3 = dm_eps2;
                dm_raymid3 = dm_ray2b;
                for (uint dm_j = 0u; dm_j < 1u; )
                {
                    float dm_midhi = dm_mid + dm_raymid3;
                    float dm_mid2 = dm_midhi * 0.5f;
                    float3 dm_sm = a0 + (dm_seg * dm_mid2);
                    float dm_inv_zm = 1.0f / dm_sm.z;
                    float4 dm_samplem = half_depth_tex.SampleLevel(sampler_nnc, (dm_sm.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float dm_dm = dm_samplem.x;
                    float dm_diffm = (1.00000202655792236328125f / dm_dm) - dm_inv_zm;
                    bool dm_under_m = (dm_diffm < 0.0f) ? true : false;
                    dm_mid = dm_under_m ? dm_mid : dm_mid2;
                    dm_lo3 = dm_under_m ? dm_lo3 : dm_diffm;
                    dm_hi3 = dm_under_m ? dm_diffm : dm_hi3;
                    dm_ray3 = dm_under_m ? (dm_inv_zm - (1.0f / dm_dm)) : dm_ray3;
                    dm_eps3 = dm_under_m ? depth_eps : dm_eps3;
                    dm_raymid3 = dm_under_m ? dm_mid2 : dm_raymid3;
                    dm_j++;
                    continue;
                }
                float dm_interp = lerp(dm_mid, dm_raymid3, dm_lo3 / (dm_lo3 + (-dm_hi3)));
                float3 dm_sm2 = a0 + (dm_seg * dm_interp);
                float dm_inv_z3 = 1.0f / dm_sm2.z;
                float4 dm_sample3 = half_depth_tex.SampleLevel(sampler_nnc, (dm_sm2.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float dm_d3 = dm_sample3.x;
                float dm_diff3 = (1.00000202655792236328125f / dm_d3) - dm_inv_z3;
                bool dm_conv = (abs(dm_diff3) < (dm_lo3 * 0.89999997615814208984375f)) ? true : false;
                dm_hit_t = dm_conv ? dm_interp : dm_raymid3;
                dm_hit_diff = dm_conv ? dm_diff3 : dm_hi3;
                dm_hit_eps = dm_conv ? depth_eps : dm_eps3;
                dm_hit_ray = dm_conv ? (dm_inv_z3 - (1.0f / dm_d3)) : dm_ray3;
                dm_hit = true;
                break;
            }
            else
            {
                dm_hit_t = dm_raymid2;
                dm_hit_diff = undef_f;
                dm_hit_eps = undef_f;
                dm_hit_ray = undef_f;
                dm_hit = false;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool dm_ok2;
        if (dm_hit)
        {
            bool dm_ok;
            if (dm_hit_ray < dm_hit_eps)
            {
                dm_ok = dm_hit_diff < dm_hit_eps;
            }
            else
            {
                dm_ok = false;
            }
            dm_ok2 = dm_ok;
        }
        else
        {
            dm_ok2 = false;
        }
        if (dm_ok2)
        {
            hit_t = dm_hit_t;
            break;
        }
        hit_t = dm_raymid2;
        break;
    } while(false);
    // 命中点:由步进结果插值。
    float3 march_hit = lerp(a0, a1, hit_t.xxx);
    float march_hit_x = march_hit.x;
    float2 hit_uv = (float4(march_hit_x, march_hit.y, undef_f2, 1.0f).xy * float2(0.5f, -0.5f)) + 0.5f.xx;
    float4 wp_hit = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(march_hit_x, march_hit.y, half_depth_tex.SampleLevel(sampler_nnc, hit_uv, 0.0f).x, 1.0f)));
    float3 hit_world = wp_hit.xyz / wp_hit.w.xxx;
    float3 hit_offset = hit_world - in_var_C1;
    float3 hit_normal = -up_world;
    int2 px_i = int2(px);
    // 级联阴影 PCSS。
    float2 shadow;
    float shadow_blur2;
    do
    {
        float4 sc = mul(frame_constants_view_constants[2u].world_to_sample, float4(in_var_C1, 1.0f));
        if (sc.z > 1.0f)
        {
            shadow = 0.0f.xx;
            shadow_blur2 = 0.0f;
            break;
        }
        float2 sc_uv = sc.xy;
        float2 sc_abs = abs(sc_uv);
        bool out0 = any(bool2(sc_abs.x > 0.9833333492279052734375f.xx.x, sc_abs.y > 0.9833333492279052734375f.xx.y));
        float2 uv1;
        if (out0)
        {
            uv1 = sc_uv * 0.5f;
        }
        else
        {
            uv1 = sc_uv;
        }
        float texel_scale = out0 ? 5.0f : 10.0f;
        float sample_scale = out0 ? 0.5f : 1.0f;
        float2 uv1_abs = abs(uv1);
        float2 edge = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * texel_scale));
        bool out1 = any(bool2(uv1_abs.x > edge.x, uv1_abs.y > edge.y));
        float texel_scale2;
        float2 uv1_2;
        float sample_scale2;
        if (out1)
        {
            texel_scale2 = texel_scale * 0.2857142984867095947265625f;
            uv1_2 = uv1 * 0.2857142984867095947265625f;
            sample_scale2 = sample_scale * 0.2857142984867095947265625f;
        }
        else
        {
            texel_scale2 = texel_scale;
            uv1_2 = uv1;
            sample_scale2 = sample_scale;
        }
        float slice = float(out1 ? 2u : uint(out0));
        uint slice_idx = 2u + uint(slice);
        float ndl = hit_normal.y;
        float4 proj = float4(uv1_2.x, uv1_2.y, sc.z, sc.w) + mul(frame_constants_view_constants[slice_idx].world_to_sample, float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - ndl, 0.0f, 1.0f))) / sample_scale2.xxx, 0.0f));
        float2 shadow_uv = (proj.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        float3 shadow_xyz = float3(shadow_uv.x, shadow_uv.y, proj.z);
        float bias = max((0.300000011920928955078125f / sample_scale2) * (1.0f - abs(ndl)), 0.02500000037252902984619140625f);
        float spiral_seed = float((uint((px_i.x * 11) + (px_i.y * 7)) + (frame_constants_frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
        uint2 px_u = uint2(px_i);
        float hash2 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px_u.x) + frame_rand)) + (0.005837149918079376220703125f * (float(px_u.y) + frame_rand))));
        float4 up4 = mul(frame_constants_view_constants[slice_idx].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));
        float blocked;
        int hits;
        blocked = 0.0f;
        hits = 0;
        float blocked2;
        int hits2;
        for (int i = 0; i < 5; blocked = blocked2, hits = hits2, i++)
        {
            float i_f = float(i);
            float ang = (i_f + spiral_seed) * 2.3999631404876708984375f;
            float3 sp = shadow_xyz + float3((((float2(cos(ang), sin(ang)) * (i_f + hash2)) * 0.20000000298023223876953125f.xx) * texel_scale2) * 0.000833333353511989116668701171875f.xx, bias * (-0.0009765625f));
            float4 sp_sample = shadow_map.SampleLevel(sampler_shadow_nearest, float3(sp.xy, slice), 0.0f);
            float d = sp_sample.x;
            if (sp.z > d)
            {
                blocked2 = blocked + d;
                hits2 = hits + 1;
            }
            else
            {
                blocked2 = blocked;
                hits2 = hits;
            }
        }
        float avg = blocked / float(hits);
        if (hits == 0)
        {
            shadow = 0.0f.xx;
            shadow_blur2 = 0.0f;
            break;
        }
        float pen = max(((proj.z - avg) * ((17.0666675567626953125f * texel_scale2) * pow(10.0f, smoothstep(0.0f, 0.25f, frame_constants_fog_tint_thickness.w)))) / avg, 0.0f);
        float shadow_blur = (pen / pow(pow(pen / (texel_scale2 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
        float up4_z = up4.z;
        float2 up_uv = (((up4.xy * float(int(sign(up4_z)))) / max(0.1500000059604644775390625f, abs(up4_z)).xx) * 0.0009765625f.xx) * 2.0f;
        float2 acc;
        acc = 0.0f.xx;
        for (int j = 0; j < 10; )
        {
            float j_f = float(j);
            float ang2 = (j_f + spiral_seed) * 2.3999631404876708984375f;
            float2 sp2 = ((float2(cos(ang2), sin(ang2)) * (j_f + hash2)) * 0.100000001490116119384765625f.xx) * shadow_blur;
            float3 scp = shadow_xyz + float3(sp2 * 0.000833333353511989116668701171875f.xx, bias * (-0.0009765625f));
            float lit = 1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(scp.xy, slice), scp.z + min(0.0f, dot(up_uv, sp2)));
            acc += float2(lit, lit * lit);
            j++;
            continue;
        }
        shadow = acc * 0.100000001490116119384765625f;
        shadow_blur2 = shadow_blur;
        break;
    } while(false);
    // 阴影均值与方差。
    float shadow_m = clamp((shadow.x - 0.100000001490116119384765625f) * 1.4285714626312255859375f, 0.0f, 1.0f);
    float2 shadow_std = shadow_m.xx + ((float2(-1.0f, 1.0f) * sqrt(max(0.0f, shadow.y - (shadow.x * shadow.x)))) * (3.0f * max(0.0f, 1.0f - (shadow_blur2 * 0.20000000298023223876953125f))));
    // 时间阴影历史:重投影到上一帧。
    float2 fb_uv = (float2(px_i) + 0.5f.xx) / float2(framebuffer_dims);
    float4 pos_ndc4 = float4((fb_uv - 0.5f.xx) * float2(2.0f, -2.0f), gl_FragCoord.z, 1.0f);
    float4 prev4 = mul(frame_constants_view_constants[0u].clip_to_prev_clip, mul(frame_constants_view_constants[0u].view_to_clip, mul(clip_to_view, pos_ndc4)));
    float2 prev_uv = ((prev4.xy / prev4.w.xx) * float2(0.5f, -0.5f)) + 0.5f.xx;
    float2 prev_clamped = clamp(prev_uv, 0.0f.xx, 1.0f.xx);
    float4 motion = float4(prev_uv - ((pos_ndc4.xy * float2(0.5f, -0.5f)) + 0.5f.xx), float(all(bool2(prev_clamped.x == prev_uv.x, prev_clamped.y == prev_uv.y))), 1.0f) * float4(1.0f, 1.0f, clamp(1.0f - length(frame_constants_view_constants[0u].clip_to_prev_clip[3].xyz), 0.0f, 1.0f), 1.0f);
    float motion_z = motion.z;
    bool hist_ok2;
    if (motion_z > 0.0f)
    {
        bool hist_ok;
        if (history_valid != 0u)
        {
            hist_ok = !(0u == frame_constants_frame_index);
        }
        else
        {
            hist_ok = false;
        }
        hist_ok2 = hist_ok;
    }
    else
    {
        hist_ok2 = false;
    }
    float shadow_hist;
    if (hist_ok2)
    {
        shadow_hist = shadows_history_tex.SampleLevel(sampler_lnc, fb_uv + motion.xy, 0.0f).x;
    }
    else
    {
        shadow_hist = shadow_m;
    }
    float shadow_temporal = lerp(clamp(shadow_hist, shadow_std.x, shadow_std.y), shadow_m, lerp(1.0f, 0.125f, motion_z));
    float inv_shadow = 1.0f - shadow_temporal;
    // 折射采样:向水面下方步进。
    float4 rp = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(in_var_C1 + (up_world * 0.5f), 1.0f)));
    float3 rp_w = rp.xyz / rp.w.xxx;
    float4 rv = pos_vs + (float4(rp_w - pos_vs_xyz, 0.0f) * float(int(sign(rp_w.z))));
    float rv_w = rv.w;
    float3 r0 = (rv / (float((rv_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(rv_w))).xxxx).xyz;
    float3 r1m0 = r0 - pos_vs_xyz;
    bool3 rneg = bool3(r1m0.x < 0.0f.xxx.x, r1m0.y < 0.0f.xxx.y, r1m0.z < 0.0f.xxx.z);
    float3 rt0 = (float3(rneg.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, rneg.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, rneg.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - pos_vs_xyz) / r1m0;
    float3 ra0 = pos_vs_xyz + (r1m0 * max(0.0f, max(rt0.x, rt0.y)));
    float3 rr3 = r0 - ra0;
    bool3 rp3 = bool3(rr3.x >= 0.0f.xxx.x, rr3.y >= 0.0f.xxx.y, rr3.z >= 0.0f.xxx.z);
    float3 rt1 = (float3(rp3.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, rp3.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, rp3.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - ra0) / rr3;
    float3 ra1 = ra0 + (rr3 * min(1.0f, min(min(rt1.x, rt1.y), rt1.z)));
    bool refr_hit;
    float refr_hit_t;
    float refr_diff;
    do
    {
        float refr_halfz = 0.5f * frustum_z;
        uint refr_steps = uint(int(max(2u, min(3u, uint(int(floor(length((((ra1.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((ra0.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * half_dims))))))));
        float rf_raymid2;
        float rf_hit_ray;
        float rf_hit_lo;
        float rf_hit_hi;
        float rf_hit_eps;
        bool rf_hit;
        do
        {
            float3 rf_seg = ra1 - ra0;
            float rf_t_count;
            if (ra1.z > 9.5367431640625e-07f)
            {
                rf_t_count = float(refr_steps);
            }
            else
            {
                rf_t_count = max(1.0f, float(refr_steps) - 1.0f);
            }
            float rf_inv = 1.0f / rf_t_count;
            float rf_lo2;
            float rf_hi2;
            float rf_eps2;
            float rf_ray2;
            bool rf_under2c;
            if (refr_steps > 0u)
            {
                float rf_t_first = lerp(0.0f, 1.0f, pow(hash * rf_inv, 1.0f));
                float3 rf_sp = ra0 + (rf_seg * rf_t_first);
                float2 rf_uv = (rf_sp.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                float rf_inv_z = 1.0f / rf_sp.z;
                float rf_rd_lo = 1.0f / half_depth_tex.SampleLevel(sampler_llc, rf_uv, 0.0f).x;
                float rf_rd_hi = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, rf_uv, 0.0f).x;
                float rf_ddiff = (max(rf_rd_lo, rf_rd_hi) * 1.00000202655792236328125f) - rf_inv_z;
                float rf_ddiff2 = rf_inv_z - min(rf_rd_lo, rf_rd_hi);
                bool rf_under = (rf_ddiff < 0.0f) ? (rf_ddiff2 < refr_halfz) : false;
                float rf_lo;
                float rf_hi;
                float rf_epshi;
                float rf_ray;
                bool rf_under_bs;
                float rf_raymid;
                if (rf_under)
                {
                    rf_lo = rf_ddiff;
                    rf_hi = rf_ddiff2;
                    rf_epshi = refr_halfz;
                    rf_ray = rf_t_first;
                    rf_under_bs = rf_under;
                    rf_raymid = 0.0f;
                }
                else
                {
                    float rf_e;
                    rf_e = rf_t_first;
                    bool rf_under2b;
                    float rf_tt;
                    float rf_diffb;
                    float rf_ddiffb;
                    float rf_epsb;
                    float rf_ttb;
                    bool rf_underb;
                    uint rf_i = 1u;
                    bool rf_under2 = rf_under;
                    for (;;)
                    {
                        if (rf_i < refr_steps)
                        {
                            rf_tt = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(rf_i) + hash) * rf_inv, 1.0f)));
                            float3 rf_sp2 = ra0 + (rf_seg * rf_tt);
                            float2 rf_uv2 = (rf_sp2.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                            float rf_inv_z2 = 1.0f / rf_sp2.z;
                            float rf_rd_lo2 = 1.0f / half_depth_tex.SampleLevel(sampler_llc, rf_uv2, 0.0f).x;
                            float rf_rd_hi2 = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, rf_uv2, 0.0f).x;
                            float rf_ddiff2b = (max(rf_rd_lo2, rf_rd_hi2) * 1.00000202655792236328125f) - rf_inv_z2;
                            float rf_ddiff22 = rf_inv_z2 - min(rf_rd_lo2, rf_rd_hi2);
                            rf_under2b = (rf_ddiff2b < 0.0f) ? (rf_ddiff22 < refr_halfz) : false;
                            if (rf_under2b)
                            {
                                rf_diffb = rf_ddiff2b;
                                rf_ddiffb = rf_ddiff22;
                                rf_epsb = refr_halfz;
                                rf_ttb = rf_tt;
                                rf_underb = rf_under2b;
                                break;
                            }
                            rf_i++;
                            rf_under2 = rf_under2b;
                            rf_e = rf_tt;
                            continue;
                        }
                        else
                        {
                            rf_diffb = 0.0f;
                            rf_ddiffb = 0.0f;
                            rf_epsb = 0.0f;
                            rf_ttb = 1.0f;
                            rf_underb = rf_under2;
                            break;
                        }
                    }
                    rf_lo = rf_diffb;
                    rf_hi = rf_ddiffb;
                    rf_epshi = rf_epsb;
                    rf_ray = rf_ttb;
                    rf_under_bs = rf_underb;
                    rf_raymid = rf_e;
                }
                rf_lo2 = rf_lo;
                rf_hi2 = rf_hi;
                rf_eps2 = rf_epshi;
                rf_ray2 = rf_ray;
                rf_under2c = rf_under_bs;
                rf_raymid2 = rf_raymid;
            }
            else
            {
                rf_lo2 = 0.0f;
                rf_hi2 = 0.0f;
                rf_eps2 = 0.0f;
                rf_ray2 = 1.0f;
                rf_under2c = false;
                rf_raymid2 = 0.0f;
            }
            if (rf_under2c)
            {
                rf_hit_ray = rf_ray2;
                rf_hit_lo = rf_lo2;
                rf_hit_hi = rf_eps2;
                rf_hit_eps = rf_hi2;
                rf_hit = true;
                break;
            }
            else
            {
                rf_hit_ray = rf_raymid2;
                rf_hit_lo = undef_f;
                rf_hit_hi = undef_f;
                rf_hit_eps = undef_f;
                rf_hit = false;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool rf_ok2;
        if (rf_hit)
        {
            bool rf_ok;
            if (rf_hit_eps < rf_hit_hi)
            {
                rf_ok = rf_hit_lo < rf_hit_hi;
            }
            else
            {
                rf_ok = false;
            }
            rf_ok2 = rf_ok;
        }
        else
        {
            rf_ok2 = false;
        }
        if (rf_ok2)
        {
            refr_hit = true;
            refr_hit_t = rf_hit_ray;
            refr_diff = rf_hit_eps / rf_hit_hi;
            break;
        }
        refr_hit = false;
        refr_hit_t = rf_raymid2;
        refr_diff = 0.0f;
        break;
    } while(false);
    // 折射因子:命中则按差异平滑衰减。
    float refr_factor;
    if (refr_hit)
    {
        refr_factor = min(inv_shadow, smoothstep(0.5f, 0.0f, (1.0f - refr_diff) * (1.0f - refr_hit_t)));
    }
    else
    {
        refr_factor = inv_shadow;
    }
    // 反射发射:2x2 子集一个代表像素写反射缓冲。
    uint quad_flag;
    if (helper)
    {
        quad_flag = 0u;
    }
    else
    {
        uint2 quad_lane = px & uint2(1u, 1u);
        quad_flag = 1u << ((quad_lane.x + (quad_lane.y * 2u)) & 31u);
    }
    uint quad_sum = quad_flag + QuadReadAcrossX(quad_flag);
    uint2 quad_id = px & uint2(1u, 1u);
    if ((quad_id.x + (quad_id.y * 2u)) == firstbithigh(quad_sum + QuadReadAcrossY(quad_sum)))
    {
        uint slot;
        __alias_ice_refl_count_buffer.InterlockedAdd(12, 1u, slot);
        __alias_ice_refl_px_buffer.Store<uint>(slot * 4 + 0, half_px.x + (half_px.y * 65536u));
        if ((slot & 63u) == 0u)
        {
            uint group;
            __alias_ice_refl_count_buffer.InterlockedAdd(0, 1u, group);
        }
    }
    // 高光 BRDF(冰面)。
    float up_w;
    float3 spec;
    do
    {
        up_w = up_world4.y;
        bool spec_ok;
        if (!(up_w <= 0.0f))
        {
            spec_ok = view_ray_y <= 0.0f;
        }
        else
        {
            spec_ok = true;
        }
        if (spec_ok)
        {
            spec = 0.0f.xxx;
            break;
        }
        float r2 = roughness * roughness;
        float3 spec_h2 = normalize(view_ray + up_world);
        float h2_y = spec_h2.y;
        float ndh = max(0.0f, 1.0f - dot(spec_h2, up_world));
        float g2 = ndh * ndh;
        float denom_w = ((h2_y * h2_y) * (r2 - 1.0f)) + 1.0f;
        spec = ((lerp(0.02999999932944774627685546875f.xxx, 1.0f.xxx, ((g2 * g2) * ndh).xxx) * (((2.0f * up_w) * view_ray_y) / ((up_w * sqrt(((((-view_ray_y) * r2) + view_ray_y) * view_ray_y) + r2)) + (view_ray_y * sqrt(((((-up_w) * r2) + up_w) * up_w) + r2))))) * (r2 / ((3.1415927410125732421875f * denom_w) * denom_w))) / ((4.0f * view_ray_y) * up_w).xxx;
        break;
    } while(false);
    float3 hit_off_swap = mul(hit_offset * float3(1.0f, -1.0f, 1.0f), float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
    float hit_len = length(hit_off_swap);
    float3 hit_n = hit_off_swap * (1.0f / hit_len);
    float3 refr_col = clamp((exp(float3(-0.5950000286102294921875f, -0.374000012874603271484375f, -0.3060000240802764892578125f) * ((abs(in_var_C1.y - hit_world.y) + length(hit_offset)) + 0.699999988079071044921875f)) * (0.959999978542327880859375f - (0.959999978542327880859375f * ((om_vry2 * om_vry2) * om_vry)))) * refraction_tex.SampleLevel(sampler_lnc, hit_uv, 0.0f).xyz, 0.0f.xxx, 65408.0f.xxx);
    float scale = asfloat((asuint(max(max(1.52587890625e-05f, refr_col.x), max(refr_col.y, refr_col.z))) + 125845504u) & 2139095040u);
    uint3 mantissa = asuint(refr_col + scale.xxx);
    // 输出 5 个渲染目标。
    out_var_SV_TARGET0 = lerp((frame_constants_sun_color.xyz * spec.x) * (refr_factor * clamp(up_w, 0.0f, 1.0f)), ((frame_constants_sun_color.xyz * (0.699999988079071044921875f * refr_factor)) * 0.3183098733425140380859375f.xxx) + frame_constants_sky_high_color.xyz, 0.00999999977648258209228515625f.xxx);
    out_var_SV_TARGET1 = sqrt(roughness);
    out_var_SV_TARGET2 = ((uint((clamp((hit_n.x * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u) | (uint((clamp((hit_n.y * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) | spvPackHalf2x16(float2(hit_len, 0.0f));
    out_var_SV_TARGET3 = ((((asuint(scale) << 4u) + 268435456u) | (mantissa.z << 18u)) | (mantissa.y << 9u)) | (mantissa.x & 511u);
    out_var_SV_TARGET4 = shadow_temporal + 0.00196078442968428134918212890625f;
}

[earlydepthstencil]
SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    stage_output.out_var_SV_TARGET1 = out_var_SV_TARGET1;
    stage_output.out_var_SV_TARGET2 = out_var_SV_TARGET2;
    stage_output.out_var_SV_TARGET3 = out_var_SV_TARGET3;
    stage_output.out_var_SV_TARGET4 = out_var_SV_TARGET4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif
