// ==RECONSTRUCTED== shaders/roof/stone_floor.inc.hlsl | F-V | embedded in nani_roof_floor.raster (2) + nani_stairs_floor.raster (2) + terrain_editing_floor.raster (2)
// Stone-floor procedural helpers. Texture bindings (stone_floor_normal/seed/height/roughness,
// x3 variants) and the heightmap ray-march + normal/seed/roughness sampling were observed
// inlined in _nani_roof_floor ps_main. The full per-variant dispatch and color/pattern logic
// is Unknown in this pass (wave-2 nani reconstruction refines it).

#ifndef TINYGLADE_ROOF_STONE_FLOOR_HLSL
#define TINYGLADE_ROOF_STONE_FLOOR_HLSL

// Ray-march a heightmap with bisection (reconstructed from the inlined use-site: 3-step
// coarse march + 4 bisection iterations), returning the parametric hit distance along `dir`
// from `origin` (origin.xy = texture-space position, origin.z = height bias).
float stone_floor_march_height(float3 origin, float2 dir, Texture2D height_map, SamplerState s,
                               float mip_bias, float height_scale)
{
    float hit = 1.0f;
    float t_lo = 0.0f;
    float t_hi = 1.0f;
    float prev = origin.z;
    for (uint i = 1u; i < 3u; i++)
    {
        float2 p = origin.xy + (dir * t_hi);
        float h = height_map.SampleLevel(s, p, mip_bias).x - 1.0f;
        float d = origin.z - (h * height_scale);
        if (d < 0.0f) break;
        prev = d;
        t_lo = t_hi;
        t_hi = saturate(min(0.9999847f, pow((float(i) + frac(0.0f)) * 0.5f, 2.0f)));
    }
    // bisection
    for (uint i = 0u; i < 4u; i++)
    {
        float t = (t_lo + t_hi) * 0.5f;
        float2 p = origin.xy + (dir * t);
        float h = height_map.SampleLevel(s, p, mip_bias).x - 1.0f;
        float d = origin.z - (h * height_scale);
        if (d < 0.0f) { t_hi = t; prev = d; }
        else { t_lo = t; prev = d; }
    }
    return lerp(t_lo, t_hi, prev / (prev + (prev - prev)));
}

// Sample a stone-floor texel set (normal/seed/height/roughness) at a texture-space position.
void stone_floor_sample(float2 uv, Texture2D normal_map, Texture2D seed_map, Texture2D height_map,
                        Texture2D roughness_map, SamplerState s, float mip_bias,
                        out float3 normal, out float4 seed, out float4 height, out float4 roughness)
{
    normal = (normal_map.SampleLevel(s, uv, mip_bias).xyz * 2.0f) - 1.0f;
    normal.z *= 0.13333332538604736328125f;
    seed = seed_map.SampleBias(s, uv, mip_bias);
    height = height_map.SampleBias(s, uv, mip_bias);
    roughness = roughness_map.SampleBias(s, uv, mip_bias);
}

#endif // TINYGLADE_ROOF_STONE_FLOOR_HLSL
