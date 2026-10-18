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
} _alias_unsorted_instance_atlas_entry_prefix_buffer;

layout(set = 0, binding = 17, std430) buffer _alias_instance_indirection_buffer
{
    uint _m0[];
} _alias_instance_indirection_buffer_1;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint draw_list_index;
} push_constants;

void main()
{
    uint _57 = gl_GlobalInvocationID.x + instance_list_offset.instance_list_offset_array[push_constants.draw_list_index / 4u][push_constants.draw_list_index % 4u];
    uint _59 = sorted_instances_buffer._m0[_57];
    if (gl_GlobalInvocationID.x < thread_count_buffer_1._m0[push_constants.draw_list_index])
    {
        uint _69 = atomicAdd(_alias_unsorted_instance_atlas_entry_prefix_buffer._m0[(_59 % 64u) + (push_constants.draw_list_index * 64u)], 1u);
        _alias_instance_indirection_buffer_1._m0[_69] = _59 / 64u;
    }
}

