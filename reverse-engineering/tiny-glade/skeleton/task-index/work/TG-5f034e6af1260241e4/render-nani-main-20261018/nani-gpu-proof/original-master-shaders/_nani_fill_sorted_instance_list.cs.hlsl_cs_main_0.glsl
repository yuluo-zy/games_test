#version 450
#extension GL_EXT_nonuniform_qualifier : require
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct GpuFilteredInstance
{
    uint raw;
};

struct GpuFilteredInstanceMeta
{
    uint raw;
};

layout(set = 0, binding = 14, std430) readonly buffer type_StructuredBuffer_uint
{
    uint _m0[];
} thread_count_buffer;

layout(set = 0, binding = 15, std430) readonly buffer type_StructuredBuffer_GpuFilteredInstance
{
    GpuFilteredInstance _m0[];
} filtered_instance_buffer;

layout(set = 0, binding = 16, std430) readonly buffer type_StructuredBuffer_GpuFilteredInstanceMeta
{
    GpuFilteredInstanceMeta _m0[];
} filtered_instance_meta_buffer;

layout(set = 0, binding = 17, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_sorted_instances_buffer;

layout(set = 0, binding = 18, std430) buffer _alias_histogram
{
    uint _m0[];
} _alias_histogram_1;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint draw_list_index;
    uint list_instance_offset;
} push_constants;

void main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= thread_count_buffer._m0[push_constants.draw_list_index])
        {
            break;
        }
        uint _69 = gl_GlobalInvocationID.x + push_constants.list_instance_offset;
        GpuFilteredInstance _71 = filtered_instance_buffer._m0[_69];
        GpuFilteredInstanceMeta _74 = filtered_instance_meta_buffer._m0[_69];
        uint _81 = atomicAdd(_alias_histogram_1._m0[(push_constants.draw_list_index * 1024u) + (_71.raw % 1024u)], 1u);
        _alias_sorted_instances_buffer._m0[_81 + push_constants.list_instance_offset] = ((_71.raw / 1024u) * 64u) + _74.raw;
        break;
    } while(false);
}

