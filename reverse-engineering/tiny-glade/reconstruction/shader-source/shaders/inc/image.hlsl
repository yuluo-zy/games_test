// ==RECONSTRUCTED== shaders/inc/image.hlsl | evidence: consumer bins (11) sample image textures for masks/borders; include-graph edge image.hlsl -> curve.hlsl; OpLine attribution shows bilinear-style lookups | status: I (function names/bodies Inference — inlined)
//
// Image/texture helpers for common mask and border operations.

#ifndef TINYGLADE_INC_IMAGE_HLSL
#define TINYGLADE_INC_IMAGE_HLSL

#include "curve.hlsl"

// 1D / 2D texture dimensions helper.
float2 texture_dims(Texture2D<float4> tex)
{
    uint w, h;
    tex.GetDimensions(w, h);
    return float2(w, h);
}

// Sample a 1-channel mask texture at a UV, returning the scalar.
float sample_mask(Texture2D<float4> mask_tex, SamplerState samp, float2 uv)
{
    return mask_tex.SampleLevel(samp, uv, 0.0f).r;
}

#endif // TINYGLADE_INC_IMAGE_HLSL
