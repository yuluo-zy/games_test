// ======================================================================
// inc/unjitter_taa.hlsl —— TAA 抖动去除辅助
// ======================================================================
// 用途:被 _taa/_tgsr 用当前帧的 sample_offset_pixels(view_constants)读取
//       重投影历史,以去除 TAA 抖动。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/unjitter_taa.hlsl | evidence: consumers _taa/_tgsr read reprojected history using the current frame's sample_offset_pixels (view_constants) to remove TAA jitter; include-graph edge math.hlsl -> unjitter_taa.hlsl | status: O/I (function names/body Inference, inlined)

#ifndef TINYGLADE_INC_UNJITTER_TAA_HLSL
#define TINYGLADE_INC_UNJITTER_TAA_HLSL

// 从像素坐标去除当前帧的 TAA 亚像素抖动。
float2 unjitter_taa_pixel(float2 jittered_pixel, float2 current_offset_pixels)
{
    return jittered_pixel - current_offset_pixels;
}

// 从归一化 UV 坐标去除 TAA 抖动。
float2 unjitter_taa_uv(float2 jittered_uv, float2 current_offset_pixels, float2 render_target_dims)
{
    return jittered_uv - (current_offset_pixels / render_target_dims);
}

// 历史帧 UV(供历史取样):当前去抖动 UV 加上上一帧的运动(通过重投影贴图,
// 由调用方施加)。
float2 taa_history_uv(float2 unjittered_uv, float2 reprojection_offset)
{
    return unjittered_uv + reprojection_offset;
}

#endif // TINYGLADE_INC_UNJITTER_TAA_HLSL
