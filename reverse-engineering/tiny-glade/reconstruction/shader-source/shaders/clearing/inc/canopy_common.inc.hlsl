// ==RECONSTRUCTED== shaders/clearing/inc/canopy_common.inc.hlsl
// Evidence: embedded in 6 bins (tree_canopy, tree_trunk, deferred canopy light sun sky x2).
// Shared tree-canopy helpers: growth age from the placement anim_t_packed half (low half,
// byte 32 of the 48-byte placement record) and the canopy placement/rotation basis used by
// the canopy / trunk vertex shaders. Consumers declare their own nani_instance_data /
// instance_indirection_buffer byte buffers.
// Status: O (placement record layout + growth-age mapping from use sites); I (helper names).

#ifndef TINYGLADE_CLEARING_INC_CANOPY_COMMON_HLSL
#define TINYGLADE_CLEARING_INC_CANOPY_COMMON_HLSL

// Growth age in [0,1] from the placement record's anim_t_packed low half (byte 32).
float canopy_age(ByteAddressBuffer placement, uint index)
{
    return f16tof32(placement.Load<uint>(index * 48u + 32) & 0xFFFFu);
}

// The placement record's bend vector (bytes 36..43).
float2 canopy_bend(ByteAddressBuffer placement, uint index)
{
    return placement.Load<float2>(index * 48u + 36);
}

// Placement transform (T * R * S) shared by the canopy and trunk vertex shaders.
float4 canopy_place(ByteAddressBuffer placement, uint index, float3 local_pos)
{
    float3 pos = placement.Load<float3>(index * 48u + 0);
    float scale = placement.Load<float>(index * 48u + 12);
    float rot = radians(placement.Load<float>(index * 48u + 16));
    float3 fwd = normalize(float3(cos(rot), 0.0f, sin(rot)));
    float4x4 basis = transpose(float4x4(float4(fwd, 0.0f),
                                        float4(0.0f, 1.0f, 0.0f, 0.0f),
                                        float4(normalize(cross(fwd, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                        float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4x4 scale_m = float4x4(float4(scale, 0.0f, 0.0f, 0.0f),
                                float4(0.0f, scale, 0.0f, 0.0f),
                                float4(0.0f, 0.0f, scale, 0.0f),
                                float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 translate_m = float4x4(float4(1.0f, 0.0f, 0.0f, pos.x),
                                    float4(0.0f, 1.0f, 0.0f, pos.y),
                                    float4(0.0f, 0.0f, 1.0f, pos.z),
                                    float4(0.0f, 0.0f, 0.0f, 1.0f));
    return mul(translate_m, mul(basis, mul(scale_m, float4(local_pos, 1.0f))));
}

#endif // TINYGLADE_CLEARING_INC_CANOPY_COMMON_HLSL
