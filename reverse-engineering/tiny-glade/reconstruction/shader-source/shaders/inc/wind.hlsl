// ==RECONSTRUCTED== shaders/inc/wind.hlsl | evidence: consumers _flag_*/_leaf_particles/_garden_lowpoly_flower/_lilypad bend vertices by gusting noise + time; include-graph edge clearing/inc/instanced_placement.hlsl -> wind.hlsl; INC-B noise functions (perlin/simplex) used in the original are referenced via the INC-B contract | status: O/I (model + function names Inference; noise dependency on INC-B)

#ifndef TINYGLADE_INC_WIND_HLSL
#define TINYGLADE_INC_WIND_HLSL

// Deterministic pseudo-random from a position+seed hash (fallback value noise;
// the original combines INC-B perlin/simplex with this same gust envelope).
float wind_noise(float2 p)
{
    uint2 ip = uint2(int2(p * 0.5));
    float2 fp = frac(p * 0.5);
    float2 u = fp * fp * (3.0 - 2.0 * fp);
    uint h00 = (ip.x * 374761393u) + (ip.y * 668265263u);
    uint h01 = ((ip.x + 1u) * 374761393u) + (ip.y * 668265263u);
    uint h10 = (ip.x * 374761393u) + ((ip.y + 1u) * 668265263u);
    uint h11 = ((ip.x + 1u) * 374761393u) + ((ip.y + 1u) * 668265263u);
    float n00 = float(h00 & 0x00FFFFFFu) / 16777215.0;
    float n01 = float(h01 & 0x00FFFFFFu) / 16777215.0;
    float n10 = float(h10 & 0x00FFFFFFu) / 16777215.0;
    float n11 = float(h11 & 0x00FFFFFFu) / 16777215.0;
    return lerp(lerp(n00, n01, u.x), lerp(n10, n11, u.x), u.y) * 2.0 - 1.0;
}

// Wind displacement for a vertex at world_pos. gust is a slowly moving low
// frequency envelope; flutter is a faster high-frequency term; amplitude and
// direction bias are per-vertex from the seed.
float3 evaluate_wind(float3 world_pos, float seed, float amplitude)
{
    float t = game_time_sec + game_time_frac;
    float2 xz = world_pos.xz;
    float gust = wind_noise(xz * 0.02 + float2(t * 0.03, 0.0) + float2(seed * 13.0, 0.0));
    float flutter = wind_noise(xz * 0.3 + float2(t * 0.5, seed) );
    float bend = saturate(gust * 0.5 + 0.5) * amplitude;
    // Bend toward -x gusting, plus a vertical flutter.
    float3 disp = float3(-gust * bend, flutter * 0.05 * amplitude, 0.0);
    disp *= smoothstep(0.0, 1.0, max(world_pos.y, 0.0));
    return disp;
}

#endif // TINYGLADE_INC_WIND_HLSL
