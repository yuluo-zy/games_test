#version 450
#extension GL_EXT_nonuniform_qualifier : require
layout(local_size_x = 32, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 13, std140) uniform type_instance_list_offset
{
    uvec4 instance_list_offset_array[27];
} instance_list_offset;

layout(set = 0, binding = 14, std430) readonly buffer type_StructuredBuffer_uint
{
    uint _m0[];
} sorted_instances_buffer;

layout(set = 0, binding = 15, std430) readonly buffer thread_count_buffer
{
    uint _m0[];
} thread_count_buffer_1;

layout(set = 0, binding = 16, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_thread_group_draw_cmd_histogram;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint draw_list_index;
    uint histogram_offset;
} push_constants;

shared uint mesh_histogram[64];
shared uint group_draw_count;

void main()
{
    for (uint _66 = gl_LocalInvocationIndex; _66 < 64u; )
    {
        mesh_histogram[_66] = 0u;
        _66 += 32u;
        continue;
    }
    bool _72 = 0u == gl_LocalInvocationIndex;
    if (_72)
    {
        group_draw_count = 0u;
    }
    uint _78 = thread_count_buffer_1._m0[push_constants.draw_list_index];
    bool _79 = gl_GlobalInvocationID.x < _78;
    uint _90;
    if (_79)
    {
        _90 = sorted_instances_buffer._m0[gl_GlobalInvocationID.x + instance_list_offset.instance_list_offset_array[push_constants.draw_list_index / 4u][push_constants.draw_list_index % 4u]];
    }
    else
    {
        _90 = 0u;
    }
    uint _91 = _90 % 64u;
    barrier();
    if (_79)
    {
        uint _95 = atomicAdd(mesh_histogram[_91], 1u);
    }
    barrier();
    for (uint _97 = gl_LocalInvocationIndex; _97 < 64u; _97 += 32u)
    {
        if (mesh_histogram[_97] > 0u)
        {
            uint _108 = atomicAdd(group_draw_count, 1u);
        }
    }
    barrier();
    if (_72)
    {
        _alias_thread_group_draw_cmd_histogram._m0[push_constants.histogram_offset + gl_WorkGroupID.x] = group_draw_count;
    }
}

