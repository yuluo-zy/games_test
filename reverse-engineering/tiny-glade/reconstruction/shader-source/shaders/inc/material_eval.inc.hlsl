// ==RECONSTRUCTED== shaders/inc/material_eval.inc.hlsl | evidence: include-graph edge material_eval.inc.hlsl -> brdf.hlsl; consumers _deferred_*/_billboard_*/_clearing_* decode gbuffer normal/roughness + BRDF lighting; raw-per-bin shows no material-specific struct member tables (params are SSA-inlined) | status: O/I (struct/function names Inference, inlined in consumers)

#ifndef TINYGLADE_INC_MATERIAL_EVAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_HLSL

// Shared surface-shading inputs produced by the per-material eval functions and
// consumed by the deferred/forward lighting code.
struct MaterialEvalData {
    float3 albedo;
    float3 normal;
    float roughness;
    float opacity;
    float3 emissive;
};

// Decode a normal packed as 3 bytes into a float4 gbuffer word: bytes are
// (n*0.5+0.5) in xyz, top byte holds a separate value (e.g. roughness).
float3 unpack_gbuffer_normal(uint packed_normal)
{
    float3 n = float3(float(packed_normal & 255u),
                      float((packed_normal >> 8u) & 255u),
                      float((packed_normal >> 16u) & 255u)) * (1.0 / 255.0);
    n = n * 2.0 - 1.0;
    return normalize(n);
}

// Initialize a material eval result with identity defaults.
void init_material_eval_data(out MaterialEvalData d)
{
    d.albedo = float3(1.0, 1.0, 1.0);
    d.normal = float3(0.0, 1.0, 0.0);
    d.roughness = 1.0;
    d.opacity = 1.0;
    d.emissive = float3(0.0, 0.0, 0.0);
}

#endif // TINYGLADE_INC_MATERIAL_EVAL_HLSL
