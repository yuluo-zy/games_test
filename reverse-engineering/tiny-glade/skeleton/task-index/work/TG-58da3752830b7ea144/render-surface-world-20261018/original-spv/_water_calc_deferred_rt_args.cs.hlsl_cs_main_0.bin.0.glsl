#version 450
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 13, std430) buffer type_RWStructuredBuffer_v4uint
{
    uvec4 _m0[];
} water_rt_count_buffer;

void main()
{
    water_rt_count_buffer._m0[0u] = uvec4((water_rt_count_buffer._m0[0u].x + 63u) / 64u, 1u, 1u, water_rt_count_buffer._m0[0u].x);
}

