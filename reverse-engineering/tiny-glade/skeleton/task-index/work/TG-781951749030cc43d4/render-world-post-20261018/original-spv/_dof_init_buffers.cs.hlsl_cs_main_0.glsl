#version 450
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct DofTileOffsetsPacked
{
    uvec4 offsets[2];
};

layout(set = 0, binding = 13, std140) uniform type_ConstantBuffer_DofTileOffsetsPacked
{
    uvec4 offsets[2];
} tile_offsets_initial_values;

layout(set = 0, binding = 14, std430) buffer type_RWStructuredBuffer_v4uint
{
    uvec4 _m0[];
} _alias_dispatch_args_buffer;

layout(set = 0, binding = 15, std430) buffer type_RWStructuredBuffer_DofTileOffsetsPacked
{
    DofTileOffsetsPacked _m0[];
} tile_offsets_buffer;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint args_type_count;
} push_constants;

void main()
{
    if (gl_GlobalInvocationID.x < push_constants.args_type_count)
    {
        _alias_dispatch_args_buffer._m0[gl_GlobalInvocationID.x] = uvec4(0u, 1u, 1u, 0u);
    }
    if (0u == gl_GlobalInvocationID.x)
    {
        tile_offsets_buffer._m0[0u] = DofTileOffsetsPacked(tile_offsets_initial_values.offsets);
    }
}

