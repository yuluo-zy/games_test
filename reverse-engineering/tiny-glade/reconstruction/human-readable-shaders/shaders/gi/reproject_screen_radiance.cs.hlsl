// 全局光照:三切片屏幕辐射度的时间重投影(REPROJECT)。
// 立方体面方向存于 temporal stats;历史经重投影运动向量 + 2x2 抖动采样;
// 历史有效时与当前帧混合(计数钳制 1..48,yzw 钳制 ±1024);历史无效且
// allow_non_determinism 时对当前帧跑 5x5 双边空间滤波。stats 字节 = count/255。
// 绑定:$Globals b13{history_valid,7 个纹理尺寸,allow_non_determinism},
// half_depth t14,half_normal t15,reprojection t16,history t17,temporal_stats t18,
// output u19,temporal_stats_out u20,frame_constants set1/b0。LocalSize [8,8,1]。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/pack_unpack.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/history_valid.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint history_valid;
    uint2 half_depth_tex_dims;
    uint2 half_view_normal_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 history_tex_dims;
    uint2 temporal_stats_tex_dims;
    uint2 output_tex_dims;
    uint2 temporal_stats_out_tex_dims;
    uint allow_non_determinism;
};

Texture2D<float4> half_depth_tex : register(t14, space0);
Texture2D<float4> half_view_normal_tex : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
Texture2D<float4> history_tex : register(t17, space0);
Texture2D<float4> temporal_stats_tex : register(t18, space0);
RWTexture2D<float4> output_tex : register(u19, space0);
RWTexture2D<unorm float4> temporal_stats_out_tex : register(u20, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 globalThreadID : SV_DispatchThreadID)
{
    uint2 xy = globalThreadID.xy;

    // 法线与权重;nw 的符号/幅值用于对齐全分辨率抖动相位。
    float4 n4 = half_view_normal_tex.Load(int3(xy, 0u));
    float3 n = n4.xyz;
    float nw = n4.w;
    float nz = n4.z;

    // 把法线投影到立方体面 UV(octahedral-ish):取最大绝对值面,翻转底面。
    float3 p = n / (abs(n.x) + abs(n.y) + abs(n.z)).xxx;
    bool flip = p.z < 0.0f;
    float3 q;
    if (flip) {
        float2 s = (1.0f.xx - abs(p.yx)) * ((step(0.0f.xx, p.xy) * 2.0f) - 1.0f.xx);
        q = float3(s.x, s.y, p.z);
    } else {
        q = p;
    }
    float2 reproj_uv = (q.xy * 0.5f) + 0.5f.xx;
    float4 stats = float4(0.0f, reproj_uv.x, reproj_uv.y, 0.0f);

    // 历史不可用时只写统计并提前返回。
    if (!(history_valid != 0u && frame_index != 0u)) {
        temporal_stats_out_tex[xy] = stats;
        return;
    }

    float2 dims = float2(half_depth_tex_dims);
    // 重投影向量:半分辨率读取,按 nw 选择抖动相位。
    float4 reproj = reprojection_tex.Load(int3((xy * uint2(2u, 2u)) + uint2(uint(abs(nw) > 0.75f), uint(nw >= 0.0f)), 0u));
    float2 uv = ((float2(int2(xy)) + 0.5f.xx) / dims) + reproj.xy;
    float rjz = reproj.z;

    float4 stats_out;
    bool updated;
    if (rjz > 0.0f) {
        // rjz 编码了亚采样掩码:非 1 时按 2x2 抖动选择半像素偏移。
        float2 uv2;
        if (rjz != 1.0f) {
            uint mask = uint((rjz * 15.0f) + 0.5f);
            uint4 sel = uint4(mask, mask, mask, mask) & uint4(1u, 2u, 4u, 8u);
            float4 fsel = float4(sel != uint4(0u, 0u, 0u, 0u));
            uv2 = uv + (((((-0.5f).xx * fsel.x) + (float2(0.5f, -0.5f) * fsel.y)) + (float2(-0.5f, 0.5f) * fsel.z)) + (0.5f.xx * fsel.w)) * 2.0f / dims;
        } else {
            uv2 = uv;
        }
        // 钳制到半分辨率纹素内,避免采到屏幕外。
        float2 uv_clamped = clamp(uv2, float2(0.5f / float(half_depth_tex_dims.x), 0.0f), 1.0f.xx - float2(0.5f / float(half_depth_tex_dims.x), 0.0f));

        // 采样三个 SH 切片的历史。
        float4 hist[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
        for (uint c0 = 0u; c0 < 3u; ++c0) {
            hist[c0] = history_tex.SampleLevel(sampler_lnc, (uv_clamped * float2(0.3333333432674407958984375f, 1.0f)) + float2(float(c0) * 0.3333333432674407958984375f, 0.0f), 0.0f);
        }

        // 用时间统计的 Gather 校验重投影方向是否与历史一致(面方向误差 < 0.2)。
        float3 q2;
        if (flip) {
            float2 s2 = (1.0f.xx - abs(p.yx)) * ((step(0.0f.xx, p.xy) * 2.0f) - 1.0f.xx);
            q2 = float3(s2.x, s2.y, p.z);
        } else {
            q2 = p;
        }
        float2 uv_q2 = (q2.xy * 0.5f) + 0.5f.xx;
        float4 gv = abs(uv_q2.xxxx - temporal_stats_tex.GatherGreen(sampler_nnc, uv_clamped, int2(0, 0)));
        float4 gb = abs(uv_q2.yyyy - temporal_stats_tex.GatherBlue(sampler_nnc, uv_clamped, int2(0, 0)));
        bool valid = (min(min(gv.x, gv.y), min(gv.z, gv.w)) + min(min(gb.x, gb.y), min(gb.z, gb.w))) < 0.20000000298023223876953125f;

        if (valid) {
            // 历史有效:累加计数(钳制 1..48)并按 1/cnt 与当前帧指数混合。
            float cnt = clamp((temporal_stats_tex.SampleLevel(sampler_nnc, uv_clamped, 0.0f) * float4(255.0f, 1.0f, 1.0f, 1.0f)).x + 1.0f, 1.0f, 48.0f);
            stats_out = stats;
            stats_out.x = cnt;
            float inv = 1.0f / cnt;
            float4 out_slab[3];
            for (uint c1 = 0u; c1 < 3u; ++c1) {
                float4 v = lerp(hist[c1], output_tex[xy + uint2(c1 * half_depth_tex_dims.x, 0u)], inv.xxxx);
                v.x = max(0.0f, v.x);
                v.yzw = clamp(v.yzw, (-1024.0f).xxx, 1024.0f.xxx);
                out_slab[c1] = v;
            }
            for (uint c2 = 0u; c2 < 3u; ++c2) {
                output_tex[xy + uint2(c2 * half_depth_tex_dims.x, 0u)] = out_slab[c2];
            }
            updated = false;
        } else {
            stats_out = stats;
            updated = true;
        }
    } else {
        stats_out = stats;
        updated = true;
    }

    if (updated) {
        temporal_stats_out_tex[xy] = stats_out;
    }

    // 历史无效且允许非确定时,对当前帧做 5x5 双边空间滤波。
    float4 result;
    if (updated && allow_non_determinism != 0u) {
        float depth = half_depth_tex.Load(int3(xy, 0u)).x;
        float normal_z = max(0.300000011920928955078125f, nz);
        float4 sum[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
        float4 accw = float4(0.0f, 0.0f, 0.0f, 0.0f);
        float wsum = 0.0f;
        float wsum2 = 0.0f;
        // 5x5 邻域(隔纹素采样,对应半分辨率),按深度/法线一致加权,并用时间计数放大。
        for (int yy = -2; yy <= 2; ++yy) {
            for (int xx = -2; xx <= 2; ++xx) {
                uint2 off = xy + uint2(int2(xx, yy) * int2(2, 2));
                if (any(bool2(off.x >= half_depth_tex_dims.x, off.y >= half_depth_tex_dims.y))) {
                    continue;
                }
                float nd = dot(n, half_view_normal_tex.Load(int3(off, 0u)).xyz);
                float w = exp2(-min(30.0f, 80.0f * (normal_z * abs(1.0f - (half_depth_tex.Load(int3(off, 0u)).x / depth))))) * (0.010000000707805156707763671875f / ((1.0f - 0.89999997615814208984375f * nd * nd) * (1.0f - 0.89999997615814208984375f * nd * nd)));
                float4 ts = temporal_stats_out_tex[off] * float4(255.0f, 1.0f, 1.0f, 1.0f);
                float tw = max(ts.x, 1.0f);
                float4 tsw = ts;
                tsw.x = tw;
                for (uint c3 = 0u; c3 < 3u; ++c3) {
                    sum[c3] += (output_tex[off + uint2(c3 * half_depth_tex_dims.x, 0u)] * w) * tw;
                }
                accw += tsw * w;
                wsum += w;
                wsum2 += w * tw;
            }
        }
        float inv2 = max(9.9999996826552253889678874634872e-21f, wsum2);
        for (uint c4 = 0u; c4 < 3u; ++c4) {
            float4 v = sum[c4] / inv2.xxxx;
            v.x = max(0.0f, v.x);
            v.yzw = clamp(v.yzw, (-1024.0f).xxx, 1024.0f.xxx);
            output_tex[xy + uint2(c4 * half_depth_tex_dims.x, 0u)] = v;
        }
        result = accw * (1.0f / max(9.9999996826552253889678874634872e-21f, wsum));
        result.x = min(result.x, 24.0f);
    } else {
        result = stats_out;
    }

    temporal_stats_out_tex[xy] = result * float4(0.0039215688593685626983642578125f, 1.0f, 1.0f, 1.0f);
}
