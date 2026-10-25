#version 450
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 13, std140) uniform type_instance_list_offset
{
    uvec4 instance_list_offset_array[27];
} instance_list_offset;

layout(set = 0, binding = 14, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_draw_list_length_buffer;

layout(set = 0, binding = 15, std430) buffer _alias_instance_indirection_allocator_buffer
{
    uint _m0[];
} _alias_instance_indirection_allocator_buffer_1;

layout(set = 0, binding = 16, std430) buffer _alias_filtered_instance_allocator_buffer
{
    uint _m0[];
} _alias_filtered_instance_allocator_buffer_1;

void main()
{
    if (gl_GlobalInvocationID.x < 108u)
    {
        uint _42 = gl_GlobalInvocationID.x / 4u;
        uint _43 = gl_GlobalInvocationID.x % 4u;
        _alias_instance_indirection_allocator_buffer_1._m0[gl_GlobalInvocationID.x] = instance_list_offset.instance_list_offset_array[_42][_43];
        _alias_filtered_instance_allocator_buffer_1._m0[gl_GlobalInvocationID.x] = instance_list_offset.instance_list_offset_array[_42][_43];
        _alias_draw_list_length_buffer._m0[gl_GlobalInvocationID.x] = 0u;
    }
}

