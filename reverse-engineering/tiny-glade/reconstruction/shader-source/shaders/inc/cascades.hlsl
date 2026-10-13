// ==RECONSTRUCTED== shaders/inc/cascades.hlsl | evidence: spirv-cross (_apply_screen_shadows/_terrain_lighting_texture) cascade selection against frame_constants.view_constants[2..4] (slice=0|1|2 via world_to_sample border tests 0.9833 and 1-0.000833*2*scale, remap *0.5 or *0.285714, view index 2+slice); include-graph edge shadow.hlsl -> cascades.hlsl | status: O (constants/math); O/I (struct + function names Inference)

#ifndef TINYGLADE_INC_CASCADES_HLSL
#define TINYGLADE_INC_CASCADES_HLSL

// Result of projecting a world position into the sun's shadow cascades.
struct ShadowCascadeSelection {
    int slice;          // -1 when behind the sun / outside all cascades
    float2 shadow_uv;   // final shadow-map UV (0..1)
    float texel_scale;  // 10 (inner) or 5 (outer) — texel width for border margin
    float sample_scale; // 1.0, 0.5 or 1/3.5 — UV remap scale
};

// Select the shadow cascade for world_pos using the sun view-projection
// matrices in view_constants[2 + slice]. Mirrors the observed opcodes.
ShadowCascadeSelection select_shadow_cascade(float3 world_pos)
{
    ShadowCascadeSelection sel;
    sel.slice = -1;
    sel.shadow_uv = float2(0.0, 0.0);
    sel.texel_scale = 10.0;
    sel.sample_scale = 1.0;

    float4 c0 = mul(view_constants[2].world_to_sample, float4(world_pos, 1.0));
    if (c0.z > 1.0) {
        return sel; // behind the sun
    }
    float2 uv = c0.xy;
    bool c0_out = any(abs(uv) > 0.983333349f);
    float texel_scale = c0_out ? 5.0 : 10.0;
    float sample_scale = c0_out ? 0.5 : 1.0;
    uint slice = c0_out ? 1u : 0u;
    if (c0_out) {
        uv *= 0.5;
    }
    bool c1_out = any(abs(uv) > (1.0 - (0.00083333335f * (2.0 * texel_scale))));
    if (c1_out) {
        texel_scale = 10.0 * 0.2857142984867095947265625f;
        uv *= 0.2857142984867095947265625f;
        sample_scale = sample_scale * 0.2857142984867095947265625f;
        slice = 2u;
    }
    // Remap to [0,1] shadow-map UV (y flipped).
    float4 final_uv = mul(view_constants[2 + slice].world_to_sample, float4(world_pos, 1.0));
    sel.slice = int(slice);
    sel.shadow_uv = float2(final_uv.x, -final_uv.y) * 0.5 + 0.5;
    sel.texel_scale = texel_scale;
    sel.sample_scale = sample_scale;
    return sel;
}

#endif // TINYGLADE_INC_CASCADES_HLSL
