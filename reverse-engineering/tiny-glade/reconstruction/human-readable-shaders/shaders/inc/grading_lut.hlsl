// ============================================================
// inc/grading_lut.hlsl —— 颜色分级 LUT 采样
// 用途:对 3D 颜色分级 LUT(grading_lut/icc_lut/tony_mc_mapface)采样。
//   消费方:_final_post、_lut_gallery_thumbnail。
// 入口:无(纯头文件)
// 关键绑定:Texture3D LUT 由消费方声明
// ============================================================
// ==RECONSTRUCTED== shaders/inc/grading_lut.hlsl | evidence: raw-per-bin (_final_post/_lut_gallery_thumbnail) grading_lut_tex/icc_lut/tony_mc_mapface_lut (Texture3D) + shadow_mask_mean read in final grading; exact helper names inlined | status: O (resources); O/I (function names/bodies Inference)

#ifndef TINYGLADE_INC_GRADING_LUT_HLSL
#define TINYGLADE_INC_GRADING_LUT_HLSL

// 在 (rgb) 颜色坐标处采样 3D 颜色分级 LUT。
float3 sample_grading_lut(Texture3D lut, SamplerState s, float3 color)
{
    return lut.SampleLevel(s, saturate(color), 0.0).xyz;
}

// 应用 Tony McMapface LUT(近似恒等的 mapface)到线性颜色。
float3 apply_tony_mc_mapface(Texture3D lut, SamplerState s, float3 color)
{
    return sample_grading_lut(lut, s, color);
}

// 按 `amount` 把分级后的颜色与未分级输入混合。
float3 apply_grading_lut(Texture3D lut, SamplerState s, float3 color, float amount)
{
    float3 graded = sample_grading_lut(lut, s, color);
    return lerp(color, graded, saturate(amount));
}

#endif // TINYGLADE_INC_GRADING_LUT_HLSL
