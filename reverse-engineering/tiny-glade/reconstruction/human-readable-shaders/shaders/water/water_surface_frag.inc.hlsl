// ==RECONSTRUCTED== shaders/water/water_surface_frag.inc.hlsl
// 中文说明:
//   水面片元共享头文件,被 _water_plane.raster.hlsl 与 _waterfall.raster.hlsl
//   (各 2 bin)嵌入。共享 PS 主体放在本文件(water_plane 与 waterfall 的 PS bin
//   都嵌入本文件 + waves.inc,且 bin PS 只读 TEXCOORD1):
//   - 反射率 0.96*fres^5,PCSS 半影,自动对焦 DoF 弥散圆,surface_blend,
//     主题 4 perlin 泡沫,最终合成;
//   - 主题==4 快速路径(waves=0,ripples=0,normal=(0,1,0));
//   - TAA 抖动表 _244[frame_index&3] 在全变体像素 uv 中被减掉(bin 53/376)。
//   WATER_SURFACE_FULL 门控两个 raster 变体 PS 主体(b903 全 / f0ad 精简);
//   默认 1 = b903。

#ifndef TINYGLADE_WATER_WATER_SURFACE_FRAG_HLSL
#define TINYGLADE_WATER_WATER_SURFACE_FRAG_HLSL

#include "waves.inc.hlsl"

// half2 打包/解包(用于涟漪缓冲与反射射线编码)。
uint pack_half2x16(float2 value)
{
    uint2 packed = f32tof16(value);
    return packed.x | (packed.y << 16u);
}

float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}

// 在视图空间沿 `start->end` 对半分辨率深度缓冲做步进。返回夹取的命中比例
// (0..1)与是否找到深度穿越点。
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
        // 抖动起始步进,逐段采样,记录进入(under)穿越前的状态。
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

    // 对穿越区间做二分精化,返回插值命中点。
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

// 把视图空间线段裁剪到视锥 slab [-1,1]^2 内。
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

// TAA 半像素抖动表(bin _244[frame_index & 3])。
static const uint2 WATER_SURFACE_JITTER[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

#ifndef WATER_SURFACE_FULL
#define WATER_SURFACE_FULL 1
#endif

struct PsIn
{
    float3 C1 : TEXCOORD1;
};

#if WATER_SURFACE_FULL
float4 ps_main(PsIn vin, float4 position : SV_Position) : SV_Target0
{
    bool helper = IsHelperLane();
    float3 world_pos = vin.C1;
    uint2 px = uint2(position.xy);
    float2 half_dims = float2(half_depth_tex_dims);

    // 波浪表面法线(waves.inc:4 倍频 + 涟漪位移)。bin 主题 4 快速路径:
    // waves=0,ripples=0,normal=(0,1,0)。
    float time = float(game_time_sec) + game_time_frac;
    float3 waves = 0.0f;
    float3 ripples = 0.0f;
    float3 surface_normal = float3(0.0f, 1.0f, 0.0f);
    if (glade_theme_id != 4u)
    {
        waves = water_wave_displacement(world_pos, time);
        ripples = water_ripple_displacement(ripple_source_buffer, world_pos, time);
        // bin:normalize((waves.y+ripples.y, 1, waves.z+ripples.z))。
        surface_normal = normalize(float3(waves.yz.x + ripples.yz.x, 1.0f, waves.yz.y + ripples.yz.y));
    }

    // 像素中心视图射线(TAA 抖动被减掉:bin 53/376)。
    float2 fb_uv = ((float2(int2((px + uint2(1u, 1u)) - WATER_SURFACE_JITTER[frame_index & 3u])) + 0.5f) / float2(framebuffer_dims)) - 0.5f;
    float2 ndc = fb_uv * float2(2.0f, -2.0f);
    float4 pos_vs = float4(ndc, position.z, 1.0f);
    float3 view_ray = normalize(mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f))).xyz);
    float3 to_cam = -view_ray;
    float3 up_world = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float fresnel = 1.0f - dot(to_cam, surface_normal);
    float fresnel2 = fresnel * fresnel;
    float reflectivity = 0.959999978542327880859375f * ((fresnel2 * fresnel2) * fresnel);

    // 每帧哈希 + 半分辨率像素。
    float frame_rand = 5.588237762451171875f * float(frame_index % 64u);
    float hash = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px.x) + frame_rand))
        + (0.005837149918079376220703125f * (float(px.y) + frame_rand))));
    uint2 half_px = px / uint2(2u, 2u);
    float4 wp4 = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, half_depth_tex.Load(int3(half_px, 0u)).x, 1.0f)));
    float3 cam_world = wp4.xyz / wp4.w;

    // 反射步进(视图空间 slab 裁剪后的线段)。
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

    // 级联阴影(PCSS + PCF)。
    float2 shadow = 0.0f;
    float shadow_blur = 0.0f;
    float4 sc = mul(view_constants[2].world_to_sample, float4(world_pos, 1.0f));
    if (sc.z <= 1.0f)
    {
        // 超出级联 0 覆盖区时切到内层级联。
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
        // 沿法线偏移避免自阴影(PCSS 半影搜索)。
        float4 proj = float4(uv1, sc.z, sc.w) + mul(view_constants[slice_idx].world_to_sample,
            float4((surface_normal * (0.066666670143604278564453125f * clamp(1.0f - ndl, 0.0f, 1.0f))) / sample_scale, 0.0f));
        float2 shadow_uv = (proj.xy * float2(0.5f, -0.5f)) + 0.5f;
        float3 shadow_xyz = float3(shadow_uv, proj.z);
        float bias = max((0.300000011920928955078125f / sample_scale) * (1.0f - abs(ndl)), 0.02500000037252902984619140625f);
        // 螺旋采样序列做半影遮挡搜索。
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
            // 半影宽度决定 PCF 模糊半径。
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
    // 阴影均值 + 二阶矩方差还原为最终可见度。
    float shadow_m = clamp((shadow.x - 0.100000001490116119384765625f) * 1.4285714626312255859375f, 0.0f, 1.0f);
    float2 shadow_std = shadow_m + (float2(-1.0f, 1.0f) * sqrt(max(0.0f, shadow.y - (shadow.x * shadow.x))) * (3.0f * max(0.0f, 1.0f - (shadow_blur * 0.20000000298023223876953125f))));

    // DoF 自动对焦弥散圆。
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

    // 折射步进。
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

    // 软件 RT 反射射线发射(每个四象限子集一个代表)。
    uint quad_flag = helper ? 0u : (1u << ((px.x & 1u) + ((px.y & 1u) * 2u)));
    uint quad_sum = quad_flag + QuadReadAcrossX(quad_flag);
    quad_sum = quad_sum + QuadReadAcrossY(quad_sum);
    if ((px.x & 1u) + ((px.y & 1u) * 2u) == firstbithigh(quad_sum))
    {
        uint slot;
        __alias_water_rt_count_buffer.InterlockedAdd(0, 1u, slot);
        // 法线打包为八面体 uv。
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
        // 写入反射射线记录(半像素坐标 + 八面体法线 half2 + 深度点积 + quad 和)。
        float3 refl_w = (wp_hit.xyz / wp_hit.w);
        __alias_water_rt_ray_buffer.Store<uint4>(slot * 16u + 0u,
            uint4(half_px.x + (half_px.y * 65536u),
                pack_half2x16(float2(oct_uv.x, 0.0f)) | (pack_half2x16(float2(oct_uv.y, 0.0f)) << 16u),
                asuint(dot(refl_dir, refl_w - cam_world)),
                (asuint(position.z) & 0xFFFFFFF0u) + quad_sum));
    }

    // 镜面 BRDF(GGX)。
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

    // 水面衰减 + 折射色 + 泡沫(仅主题 4)。
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
    // 表面混合系数:随高度差与波浪起伏变化。
    float surface_blend = 0.60000002384185791015625f + ((0.4000000059604644775390625f
        * (exp2((-0.4000000059604644775390625f) * height_diff) * smoothstep(0.0f, 1.0f, height_diff)))
        * (waves.y + ripples.y));

    // 最终合成:水体吸收色 + 反射色 + 泡沫。
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

    // bin 主题 4 快速路径(f0ad 变体中也有)。
    float time = float(game_time_sec) + game_time_frac;
    float3 waves = 0.0f;
    float3 ripples = 0.0f;
    float3 surface_normal = float3(0.0f, 1.0f, 0.0f);
    if (glade_theme_id != 4u)
    {
        waves = water_wave_displacement(world_pos, time);
        ripples = water_ripple_displacement(ripple_source_buffer, world_pos, time);
        surface_normal = normalize(float3(waves.yz.x + ripples.yz.x, 1.0f, waves.yz.y + ripples.yz.y));
    }

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

    // 精简路径:仅发射软件 RT 反射射线记录,输出黑色。
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

#endif // TINYGLADE_WATER_WATER_SURFACE_FRAG_HLSL
