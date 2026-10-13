// ==RECONSTRUCTED== shaders/clearing/inc/instanced_placement.hlsl
// Evidence: embedded in 10 bins (instanced_background_tree, meadow_flower, meadow_plant,
// tree_canopy, tree_trunk x2 variants). Provides the 48-byte PlacementData record
// (position float3 @0, scale @12, rotation_angle @16, id @20, custom0/1, anim_t_packed,
// bend_vector @36, pad0) and the placement transform used by the background tree /
// meadow plant / tree canopy / trunk vertex shaders. Each consumer declares its own
// byte-address buffer (placement_buffer / nani_instance_data) at its own set0 binding
// and reads the record fields inline; the struct + transform helper live here.
// Status: O (struct layout from OpMemberName + byte accesses verbatim); I (helper
// signature — the transform is inlined at use sites).

#ifndef TINYGLADE_CLEARING_INC_INSTANCED_PLACEMENT_HLSL
#define TINYGLADE_CLEARING_INC_INSTANCED_PLACEMENT_HLSL

// Per-instance placement record (48 bytes).
struct PlacementData
{
    float3 position;
    float scale;
    float rotation_angle;
    int id;
    uint custom0;
    uint custom1;
    uint anim_t_packed;
    float2 bend_vector;
    int pad0;
};

// Placement transform: T(position) * R(rotation_angle) * S(scale), matching the
// inlined use sites. `placement` is the consumer's byte-address buffer, `index`
// the record index (48-byte stride).
float4 placement_world(ByteAddressBuffer placement, uint index, float3 vertex_pos)
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
    return mul(translate_m, mul(basis, mul(scale_m, float4(vertex_pos, 1.0f))));
}

#endif // TINYGLADE_CLEARING_INC_INSTANCED_PLACEMENT_HLSL
