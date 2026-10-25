// ======================================================================
// inc/raymarch.hlsl —— 深度纹理射线步进公共头文件
// ======================================================================
// 用途:被 grass/plant 材质与 _deferred_*/_dof_*/_water_* 消费者用于对视空间
//       射线沿深度/高度场步进,实现草/植物的半透明与体积轮廓。
//
// 关键函数:
//   raymarch_depth_texture  沿射线对线性化深度纹理步进,返回首次命中参数 t
//   raymarch_opacity        沿线段累加逐步行透明度,返回 0..1 的不透明度
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/raymarch.hlsl | evidence: include-graph edges material/{grass,plant}_material -> raymarch.hlsl; consumers _deferred_*/_dof_*/_water_* march view rays against depth/height fields; exact helper names inlined | status: O/I (function names + step heuristics Inference)

#ifndef TINYGLADE_INC_RAYMARCH_HLSL
#define TINYGLADE_INC_RAYMARCH_HLSL

// 沿视空间射线从起点到终点对线性化深度纹理步进。返回首次命中的射线参数
// t(0..1),无透明物被穿越时返回 1.0。用于草/植物半透明与体积轮廓。
float raymarch_depth_texture(Texture2D depth_tex, SamplerState s, float2 texel_size,
                             float3 ray_start_view, float3 ray_end_view)
{
    float3 delta = ray_end_view - ray_start_view;
    float dist = length(delta);
    float t = 0.0;
    // 固定分辨率步进:~dist 个视空间单位 / 步长。
    float step = max(0.05, dist / 16.0);
    for (uint i = 0u; i < 64u; ++i) {
        t += step / max(dist, 1e-6);
        if (t >= 1.0) {
            return 1.0;
        }
        float3 p = ray_start_view + delta * t;
        if (p.z <= 0.0) {
            return 1.0;
        }
        float2 uv = p.xy * 0.5 + 0.5;
        uv.y = 1.0 - uv.y;
        float sampled_depth = depth_tex.SampleLevel(s, uv, 0.0).x;
        float inv_depth = 1.0 / sampled_depth;
        float my_inv = 1.0 / p.z;
        if (my_inv > inv_depth) {
            // 已穿越不透明几何体;二分精化求交位置。
            float lo = max(0.0, t - step / max(dist, 1e-6));
            for (uint j = 0u; j < 8u; ++j) {
                float mid = (lo + t) * 0.5;
                float3 pm = ray_start_view + delta * mid;
                float2 uvm = pm.xy * 0.5 + 0.5;
                uvm.y = 1.0 - uvm.y;
                if (depth_tex.SampleLevel(s, uvm, 0.0).x > 1.0 / pm.z) {
                    t = mid;
                } else {
                    lo = mid;
                }
            }
            return t;
        }
    }
    return 1.0;
}

// 沿线段对逐步行透明度求和(用于穿过草/植物簇的柔和半透明):返回 0..1 不透明度。
float raymarch_opacity(Texture2D depth_tex, SamplerState s, float2 texel_size,
                       float3 ray_start_view, float3 ray_end_view, float density)
{
    float3 delta = ray_end_view - ray_start_view;
    float dist = length(delta);
    float acc = 0.0;
    uint steps = 12u;
    for (uint i = 0u; i < steps; ++i) {
        float t = (float(i) + 0.5) / float(steps);
        float3 p = ray_start_view + delta * t;
        if (p.z <= 0.0) {
            break;
        }
        float2 uv = p.xy * 0.5 + 0.5;
        uv.y = 1.0 - uv.y;
        float inv_depth = 1.0 / max(depth_tex.SampleLevel(s, uv, 0.0).x, 1e-6);
        if (inv_depth < 1.0 / max(p.z, 1e-6)) {
            acc += density * (dist / float(steps));
        }
    }
    return saturate(acc);
}

#endif // TINYGLADE_INC_RAYMARCH_HLSL
