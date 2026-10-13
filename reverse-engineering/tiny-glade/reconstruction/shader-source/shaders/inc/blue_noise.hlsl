#pragma once
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

// Texture-parameter helper (the texture is declared per-consumer at its own register).
float4 blue_noise(Texture2D<float4> blue_noise_texture, uint2 pixel_coords, uint frame_index)
{
    uint2 offset = uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float(frame_index)) + 0.5f.xx) * 256.0f);
    uint2 p = (pixel_coords + uint2(13u, 13u) + offset) % uint2(256u, 256u);
    float4 texel = blue_noise_texture.Load(int3(p, 0));
    return texel * 255.0f * (1.0f / 256.0f) + (1.0f / 512.0f);
}
