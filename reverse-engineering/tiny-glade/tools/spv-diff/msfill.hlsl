// Fill passes for multisampled sampled-image inputs of the differential executor.
// Compile: vs_main (vs_6_4), fs_f / fs_u (ps_6_4) with -spirv; spv-diff reads them via SPVDIFF_MS_VS / _FS_F / _FS_U.
struct PC { uint profile; };
[[vk::push_constant]] PC pc;

float4 vs_main(uint vid : SV_VertexID) : SV_Position
{
    float2 p = float2((vid << 1) & 2, vid & 2);
    return float4(p * 2.0f - 1.0f, 0.5f, 1.0f);
}

uint hash(uint x)
{
    x ^= x >> 16; x *= 0x7feb352du; x ^= x >> 15; x *= 0x846ca68bu; x ^= x >> 16;
    return x;
}
float rnd(uint h) { return float(h & 0xFFFFFFu) / 16777216.0f; }

float4 fs_f(float4 p : SV_Position, uint s : SV_SampleIndex) : SV_Target0
{
    uint h = hash((uint(p.x) * 73856093u) ^ (uint(p.y) * 19349663u) ^ (s * 83492791u) ^ (pc.profile * 2654435761u));
    float4 v = float4(rnd(h), rnd(hash(h + 1u)), rnd(hash(h + 2u)), rnd(hash(h + 3u)));
    switch (pc.profile % 16)
    {
    case 3: v = v * 2.0f - 1.0f; break;
    case 4: v *= 6.0f; break;
    case 7: v = 0.0f; break;
    case 8: v = 1.0f; break;
    case 11: if (rnd(hash(h + 9u)) < 0.75f) v = 0.0f; break;
    case 12: v = 0.45f + v * 0.1f; break;
    case 14: v *= 1.0e4f; break;
    case 15: v = (v * 2.0f - 1.0f) * 1.0e-6f; break;
    default: break;
    }
    return v;
}

uint4 fs_u(float4 p : SV_Position, uint s : SV_SampleIndex) : SV_Target0
{
    uint h = hash((uint(p.x) * 73856093u) ^ (uint(p.y) * 19349663u) ^ (s * 83492791u) ^ (pc.profile * 2654435761u));
    uint4 v = uint4(h % 64u, hash(h + 1u) % 64u, hash(h + 2u) % 64u, hash(h + 3u) % 64u);
    switch (pc.profile % 16)
    {
    case 1: v %= 4u; break;
    case 7: v = 0u; break;
    case 8: v = 1u; break;
    case 13: v = uint4((uint(p.x) + uint(p.y)) % 64u, s, 0u, 1u); break;
    case 14: v = uint4(h % 65536u, 0u, 0u, 0u); break;
    default: break;
    }
    return v;
}
