// ==RECONSTRUCTED== shaders/inc/lanterns.hlsl | evidence: raw-per-bin OpMemberName LanternPacked{position, intensity__color_id} 16B (lantern_buf stride 16: position@0, packed uint@12 = half16 intensity low | color_id high); spirv-cross (_deferred_*_light_sun_sky/_light_binning_bin_lights) 5-entry normalized lantern color palette + intensity falloff (1/(0.336+dist2), smoothstep(36i,16i,dist2), up-smoothstep(-0.5,0.2)); include-graph edge lighting_indirect.hlsl -> lanterns.hlsl | status: O (struct/palette/falloff); O/I (helper names Inference)

#ifndef TINYGLADE_INC_LANTERNS_HLSL
#define TINYGLADE_INC_LANTERNS_HLSL

// A lantern light source in the CPU-built lantern buffer (16 bytes stride).
struct LanternPacked {
    float3 position;
    uint intensity__color_id; // half16 intensity | color_id << 16
};

// World position of a lantern.
float3 get_lantern_position(ByteAddressBuffer lantern_buf, uint index)
{
    return lantern_buf.Load<float3>(index * 16u + 0u);
}

// Emissive intensity (half16 in the low bits of intensity__color_id).
float get_lantern_intensity(ByteAddressBuffer lantern_buf, uint index)
{
    return f16tof32(lantern_buf.Load<uint>(index * 16u + 12u) & 0xffffu);
}

// Palette color id (high 16 bits of intensity__color_id).
uint get_lantern_color_id(ByteAddressBuffer lantern_buf, uint index)
{
    return lantern_buf.Load<uint>(index * 16u + 12u) >> 16u;
}

// The five lantern colors, normalized per color (matches the embedded literals).
float3 get_lantern_color(uint color_id)
{
    if (color_id == 0u) {
        return float3(0.7919800281524658203125, 0.63358402252197265625, 0.07919800281524658203125) / max(1e-10, 0.7919800281524658203125);
    } else if (color_id == 1u) {
        return float3(0.0, 0.1583960056304931640625, 0.7919800281524658203125) / max(1e-10, 0.215240001678466796875);
    } else if (color_id == 2u) {
        return float3(0.1583960056304931640625, 0.7919800281524658203125, 0.1583960056304931640625) / max(1e-10, 0.772159993648529052734375);
    } else if (color_id == 3u) {
        return float3(0.23759402334690093994140625, 0.1583960056304931640625, 0.7919800281524658203125) / max(1e-10, 0.279020011425018310546875);
    }
    return float3(0.7919800281524658203125, 0.23759402334690093994140625, 0.07919800281524658203125) / max(1e-10, 0.4343800246715545654296875);
}

// Unweighted lantern light contribution (color * NdotL * distance/height
// falloff) before the shadow/visibility term is applied.
float3 evaluate_lantern_light(float3 lantern_position, float lantern_intensity, uint color_id,
                              float3 sample_pos, float3 normal)
{
    float3 to_light = lantern_position - sample_pos;
    float dist2 = dot(to_light, to_light);
    float dist = length(to_light);
    float3 l = to_light / max(dist, 1e-6);
    float3 color = get_lantern_color(color_id) * 4.0 * lantern_intensity;
    float atten = (1.0 / (0.3359999954700469970703125 + dist2)) *
                  smoothstep(36.0 * lantern_intensity, 16.0 * lantern_intensity, dist2);
    float ndl = max(0.0, dot(normal, l));
    float3 up = to_light + float3(0.0, 0.1500000059604644775390625, 0.0);
    float up_atten = smoothstep(-0.5, 0.20000000298023223876953125, up.y / max(length(up), 1e-6));
    return color * ndl * atten * up_atten;
}

#endif // TINYGLADE_INC_LANTERNS_HLSL
