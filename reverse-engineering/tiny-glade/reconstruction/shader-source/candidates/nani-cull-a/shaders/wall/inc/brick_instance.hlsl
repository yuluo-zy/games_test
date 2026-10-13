// ==RECONSTRUCTED== shaders/wall/inc/brick_instance.hlsl | nani-cull-a
// Observation: embedded as include in 6 wall bins. Per-brick-wall instance
// record and its packed affine transform, recovered verbatim from consumer
// OpMemberName tables (InstancedWallData, stride 96: transform 48 + sin_deform_y
// 16 + global_arch_height_vals 12 + seed 4 + source_id 4 + wallspace_x_range 8
// + flags 4).
#ifndef WALL_INC_BRICK_INSTANCE_HLSL
#define WALL_INC_BRICK_INSTANCE_HLSL

struct Affine3Packed
{
    float data[12];
};

struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

#endif // WALL_INC_BRICK_INSTANCE_HLSL
