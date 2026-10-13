// ==RECONSTRUCTED== shaders/inc/parallax_mapping.hlsl | evidence: embedded in 10 bins (roof dormer_hole includes it); no named functions recoverable (inlined) | status: I/U (names/bodies Inference — standard steep parallax mapping)
//
// Steep parallax mapping helper (depth-map-based UV offset).

#ifndef TINYGLADE_INC_PARALLAX_MAPPING_HLSL
#define TINYGLADE_INC_PARALLAX_MAPPING_HLSL

// Steep parallax occlusion step: march the view ray through `layers` steps of
// the heightmap `height_tex`, returning an offset UV. height_scale scales the
// parallax height; view_dir is the tangent-space view direction.
float2 parallax_mapping(Texture2D<float4> height_tex, SamplerState samp,
                        float2 uv, float3 view_dir, float height_scale, uint layers)
{
    float layer_depth = 1.0f / float(layers);
    float current_depth = 1.0f;
    float2 p = view_dir.xy * (height_scale / float(layers));
    float2 delta = p;

    float2 current_uv = uv;
    float height = 1.0f - height_tex.SampleLevel(samp, current_uv, 0.0f).r;
    while (current_depth > 0.0f && height < current_depth)
    {
        current_uv -= delta;
        height = 1.0f - height_tex.SampleLevel(samp, current_uv, 0.0f).r;
        current_depth -= layer_depth;
    }

    // Linear interpolation between the last two steps for smoothness.
    float2 prev_uv = current_uv + delta;
    float prev_depth = current_depth + layer_depth;
    float prev_height = 1.0f - height_tex.SampleLevel(samp, prev_uv, 0.0f).r;
    float w = (current_depth - height) / max((current_depth - height) - (prev_depth - prev_height), 1.0e-5f);
    return lerp(current_uv, prev_uv, saturate(w));
}

#endif // TINYGLADE_INC_PARALLAX_MAPPING_HLSL
