// ==RECONSTRUCTED== shaders/deferred/light_sun_sky_prelude.inc.hlsl | status: O (GBuffer layout from
// _deferred_*_light_sun_sky disassembly); I (helper names)
// Evidence: the deferred sun/sky pass family shares this prelude (embedded in 8 bins).
// Outgoing include edge: frame_constants.hlsl. It holds the GBuffer decode / view-
// position reconstruction helpers plus the blue-noise dither used by the fragment
// body; the light_sun_sky.inc.hlsl (included later by each pass) carries the
// lighting evaluation. No resources are bound here (per-pass $Globals/bindings are
// declared by the owning .raster.hlsl).

#ifndef TINYGLADE_LIGHT_SUN_SKY_PRELUDE_INC_HLSL
#define TINYGLADE_LIGHT_SUN_SKY_PRELUDE_INC_HLSL

#include "inc/frame_constants.hlsl"

// Reconstruct the view-space position of a texel from its UV and depth.
float3 reconstruct_view_position(float2 uv, float depth)
{
    float2 ndc = (uv - 0.5f) * float2(2.0f, -2.0f);
    return mul(view_constants[0].sample_to_view, float4(ndc, depth, 1.0f)).xyz;
}

// Blue-noise dither (shared by the sun/shadow taps).
float4 screen_blue_noise(Texture2D blue_noise_texture, SamplerState s, uint2 px, uint frame_index)
{
    uint2 noise_px = (px + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) *
                                      float(frame_index)) + 0.5f) * 256.0f)) % uint2(256u, 256u);
    return (blue_noise_texture.SampleLevel(s, (float2(noise_px) + 0.5f) * (1.0f / 256.0f), 0.0f) * 255.0f * 0.00390625f) + 0.001953125f;
}

#endif // TINYGLADE_LIGHT_SUN_SKY_PRELUDE_INC_HLSL
