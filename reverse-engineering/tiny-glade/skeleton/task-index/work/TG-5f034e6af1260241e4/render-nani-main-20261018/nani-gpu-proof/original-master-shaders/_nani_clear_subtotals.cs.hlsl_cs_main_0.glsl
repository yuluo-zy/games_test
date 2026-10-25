#version 450
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 13, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_subtotals;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint item_count;
} push_constants;

void main()
{
    if (gl_GlobalInvocationID.x < push_constants.item_count)
    {
        _alias_subtotals._m0[gl_GlobalInvocationID.x] = 0u;
    }
}

