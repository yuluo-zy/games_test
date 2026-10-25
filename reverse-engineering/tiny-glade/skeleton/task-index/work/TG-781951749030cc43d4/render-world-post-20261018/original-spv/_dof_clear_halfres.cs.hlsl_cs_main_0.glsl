#version 450
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

struct DofTileOffsets
{
    uint offsets[8];
};

layout(set = 0, binding = 15, std430) readonly buffer type_StructuredBuffer_DofTileOffsets
{
    DofTileOffsets _m0[];
} tile_offsets_buffer;

layout(set = 0, binding = 16, std430) readonly buffer type_StructuredBuffer_v2uint
{
    uvec2 _m0[];
} tile_coords_buffer;

layout(set = 0, binding = 14, rgba16f) uniform writeonly image2D img_output;

void main()
{
    imageStore(img_output, ivec2((gl_GlobalInvocationID.xy & uvec2(7u)) + (uvec2(8u) * tile_coords_buffer._m0[tile_offsets_buffer._m0[0u].offsets[6] + gl_WorkGroupID.x])), vec4(0.0));
}

