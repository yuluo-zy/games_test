// ==RECONSTRUCTED== shaders/inc/grading_lut.hlsl | evidence: raw-per-bin (_final_post/_lut_gallery_thumbnail) grading_lut_tex/icc_lut/tony_mc_mapface_lut (Texture3D) + shadow_mask_mean read in final grading; exact helper names inlined | status: O (resources); O/I (function names/bodies Inference)

#ifndef TINYGLADE_INC_GRADING_LUT_HLSL
#define TINYGLADE_INC_GRADING_LUT_HLSL

// Sample a 3D color-grading LUT at the (rgb) color coordinates.
float3 sample_grading_lut(Texture3D lut, SamplerState s, float3 color)
{
    return lut.SampleLevel(s, saturate(color), 0.0).xyz;
}

// Apply the Tony McMapface LUT (an identity-ish mapface) to linear color.
float3 apply_tony_mc_mapface(Texture3D lut, SamplerState s, float3 color)
{
    return sample_grading_lut(lut, s, color);
}

// Blend a graded color with the ungraded input by `amount`.
float3 apply_grading_lut(Texture3D lut, SamplerState s, float3 color, float amount)
{
    float3 graded = sample_grading_lut(lut, s, color);
    return lerp(color, graded, saturate(amount));
}

#endif // TINYGLADE_INC_GRADING_LUT_HLSL
