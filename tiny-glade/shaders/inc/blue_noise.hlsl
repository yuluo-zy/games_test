#pragma once
// ============================================================
// inc/blue_noise.hlsl —— 蓝噪声采样辅助
// 用途:用金角旋转规律把同帧像素采样点摊开,降低可见的周期重复。
//   消费方:_apply_volumetrics、_gi_eval_screen_radiance、_lut_water_noise。
// 入口:无(纯头文件;纹理由消费方各自声明并绑定到自己的寄存器)
// 关键绑定:纹理 per-consumer 声明(如 water/ice 系 t15、volumetrics t17)
// ============================================================
// ==RECONSTRUCTED== shaders/inc/blue_noise.hlsl | evidence: raw-per-bin.jsonl binding blue_noise_texture (Texture2D, SampledImage 2D, 28 bins); _apply_volumetrics/_gi_eval_screen_radiance/_lut_water_noise inlined golden-ratio-rotated .Load pattern | status: I
// MERGE (water-ice review inc-fix #3, RECOVERY_TRACKER #17): the blue_noise_texture
// global declaration and its fixed register(t14) were removed so consumers bind the
// texture per-register (the water/ice RT family binds t15, apply_volumetrics t17,
// tree_canopy t21, text.raster t14). Consumers declare `blue_noise_texture` locally.
//
// Observation (identical inlined pattern in _apply_volumetrics and _gi_eval_screen_radiance):
//   p = (pixel + uint2(13,13)) + uint2(frac((float2(0.75487768650054931640625, 0.56984031200408935546875)
//       * float(seed)) + 0.5) * 256)  (mod 256)
//   return blue_noise_texture.Load(int3(p,0)) * 255 * (1/256) + (1/512)
// Consumers pass different per-frame seeds (frame_constants_frame_index, 71+frame_constants_frame_index),
// so the seed is a function parameter.  Function name not embedded -> Inference.
// blue_noise_texture_dims appears as a $Globals member in consumers but the observed sampling is
// hardcoded to 256 texels.

// 按像素坐标与帧序号取一帧蓝噪声样本(纹理由消费方声明)。
// 旋转常量 0.754877…/0.569840… 是黄金角方向,(13,13) 为固定抖动偏移。
float4 blue_noise(Texture2D<float4> blue_noise_texture, uint2 pixel_coords, uint frame_index)
{
    uint2 offset = uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float(frame_index)) + 0.5f.xx) * 256.0f);
    uint2 p = (pixel_coords + uint2(13u, 13u) + offset) % uint2(256u, 256u);
    float4 texel = blue_noise_texture.Load(int3(p, 0));
    // 0..255 映射到 [1/512, 1-1/512] 的抖动区间。
    return texel * 255.0f * (1.0f / 256.0f) + (1.0f / 512.0f);
}
