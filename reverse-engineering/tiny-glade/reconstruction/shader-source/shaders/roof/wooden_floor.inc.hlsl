// ==RECONSTRUCTED== shaders/roof/wooden_floor.inc.hlsl | F-V | embedded in nani_roof_floor.raster (2) + nani_stairs_floor.raster (2)
// Wooden-floor procedural helpers. Texture bindings (wooden_floor_normal/height/seed/roughness,
// x3 variants) observed inlined in _nani_roof_floor ps_main. The per-plank pattern/color logic
// is Unknown in this pass (wave-2 nani reconstruction refines it).

#ifndef TINYGLADE_ROOF_WOODEN_FLOOR_HLSL
#define TINYGLADE_ROOF_WOODEN_FLOOR_HLSL

// Sample a wooden-floor texel set at a texture-space position.
void wooden_floor_sample(float2 uv, Texture2D normal_map, Texture2D seed_map, Texture2D height_map,
                         Texture2D roughness_map, SamplerState s, float mip_bias,
                         out float3 normal, out float4 seed, out float4 height, out float4 roughness)
{
    normal = (normal_map.SampleLevel(s, uv, mip_bias).xyz * 2.0f) - 1.0f;
    seed = seed_map.SampleBias(s, uv, mip_bias);
    height = height_map.SampleBias(s, uv, mip_bias);
    roughness = roughness_map.SampleBias(s, uv, mip_bias);
}

#endif // TINYGLADE_ROOF_WOODEN_FLOOR_HLSL
