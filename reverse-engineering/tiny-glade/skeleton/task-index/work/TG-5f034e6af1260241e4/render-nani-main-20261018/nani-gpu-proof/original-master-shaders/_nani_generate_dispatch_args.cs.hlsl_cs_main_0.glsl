#version 450
#if defined(GL_EXT_control_flow_attributes)
#extension GL_EXT_control_flow_attributes : require
#define SPIRV_CROSS_FLATTEN [[flatten]]
#define SPIRV_CROSS_BRANCH [[dont_flatten]]
#define SPIRV_CROSS_UNROLL [[unroll]]
#define SPIRV_CROSS_LOOP [[dont_unroll]]
#else
#define SPIRV_CROSS_FLATTEN
#define SPIRV_CROSS_BRANCH
#define SPIRV_CROSS_UNROLL
#define SPIRV_CROSS_LOOP
#endif
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 13, std140) uniform type_instance_list_offset
{
    uvec4 instance_list_offset_array[27];
} instance_list_offset;

layout(set = 0, binding = 14, std430) readonly buffer type_StructuredBuffer_uint
{
    uint _m0[];
} filtered_instance_allocator_buffer;

layout(set = 0, binding = 15, std430) buffer type_RWStructuredBuffer_v4uint
{
    uvec4 _m0[];
} _alias_dispatch_args_buffer;

layout(set = 0, binding = 16, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_thread_count_buffer;

void main()
{
    if (gl_GlobalInvocationID.x < 108u)
    {
        uint _61 = gl_GlobalInvocationID.x / 4u;
        uint _62 = gl_GlobalInvocationID.x % 4u;
        uint _72;
        do
        {
            SPIRV_CROSS_BRANCH
            if (filtered_instance_allocator_buffer._m0[gl_GlobalInvocationID.x] >= instance_list_offset.instance_list_offset_array[_61][_62])
            {
                _72 = filtered_instance_allocator_buffer._m0[gl_GlobalInvocationID.x] - instance_list_offset.instance_list_offset_array[_61][_62];
                break;
            }
            else
            {
                _72 = 0u;
                break;
            }
            break; // unreachable workaround
        } while(false);
        _alias_thread_count_buffer._m0[gl_GlobalInvocationID.x] = _72;
        _alias_dispatch_args_buffer._m0[gl_GlobalInvocationID.x] = uvec4((_72 + 63u) / 64u, 1u, 1u, 0u);
        _alias_dispatch_args_buffer._m0[gl_GlobalInvocationID.x + 108u] = uvec4((_72 + 31u) / 32u, 1u, 1u, 0u);
        _alias_dispatch_args_buffer._m0[gl_GlobalInvocationID.x + 216u] = uvec4((_72 + 32767u) / 32768u, 1u, 1u, 0u);
        _alias_dispatch_args_buffer._m0[gl_GlobalInvocationID.x + 324u] = uvec4(uint(int(_72 >= 65536u)) / 1u, 1u, 1u, 0u);
    }
}

