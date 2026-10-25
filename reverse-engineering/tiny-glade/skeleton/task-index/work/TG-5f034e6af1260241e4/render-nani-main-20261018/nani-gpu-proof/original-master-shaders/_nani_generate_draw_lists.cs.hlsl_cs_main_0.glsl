#version 450
#extension GL_EXT_nonuniform_qualifier : require
layout(local_size_x = 32, local_size_y = 1, local_size_z = 1) in;

struct MeshAtlasPartInfo
{
    uint first_index;
    uint index_count;
    float bounding_sphere_radius;
    uint pad0;
};

struct DrawIndexedIndirectCommand
{
    uint index_count;
    uint instance_count;
    uint first_index;
    uint vertex_offset;
    uint first_instance;
};

layout(set = 0, binding = 13, std140) uniform type_instance_list_offset
{
    uvec4 instance_list_offset_array[27];
} instance_list_offset;

layout(set = 0, binding = 14, std430) readonly buffer type_StructuredBuffer_
{
    MeshAtlasPartInfo _m0[][64];
} atlas_part_info;

layout(set = 0, binding = 15, std430) readonly buffer type_StructuredBuffer_uint
{
    uint _m0[];
} sorted_instances_buffer;

layout(set = 0, binding = 16, std430) readonly buffer thread_count_buffer
{
    uint _m0[];
} thread_count_buffer_1;

layout(set = 0, binding = 17, std430) readonly buffer thread_group_draw_cmd_histogram
{
    uint _m0[];
} thread_group_draw_cmd_histogram_1;

layout(set = 0, binding = 18, std430) readonly buffer thread_group_draw_cmd_subtotals
{
    uint _m0[];
} thread_group_draw_cmd_subtotals_1;

layout(set = 0, binding = 19, std430) buffer type_RWStructuredBuffer_DrawIndexedIndirectCommand
{
    DrawIndexedIndirectCommand _m0[];
} _alias_draw_list_buffer;

layout(set = 0, binding = 20, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_instance_indirection_buffer;

layout(set = 0, binding = 21, std430) buffer _alias_instance_indirection_allocator_buffer
{
    uint _m0[];
} _alias_instance_indirection_allocator_buffer_1;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint world_subset;
    uint draw_list_index;
    uint list_draw_cmd_offset;
    uint list_merge_window_offset;
    uint list_merge_window_subtotal_offset;
} push_constants;

shared uint mesh_histogram[64];
shared uint group_draw_offset;

void main()
{
    for (uint _90 = gl_LocalInvocationIndex; _90 < 64u; )
    {
        mesh_histogram[_90] = 0u;
        _90 += 32u;
        continue;
    }
    if (0u == gl_LocalInvocationIndex)
    {
        group_draw_offset = thread_group_draw_cmd_histogram_1._m0[push_constants.list_merge_window_offset + gl_WorkGroupID.x] + thread_group_draw_cmd_subtotals_1._m0[push_constants.list_merge_window_subtotal_offset + (gl_WorkGroupID.x / 1024u)];
    }
    uint _112 = thread_count_buffer_1._m0[push_constants.draw_list_index];
    bool _113 = gl_GlobalInvocationID.x < _112;
    uint _124;
    if (_113)
    {
        _124 = sorted_instances_buffer._m0[gl_GlobalInvocationID.x + instance_list_offset.instance_list_offset_array[push_constants.draw_list_index / 4u][push_constants.draw_list_index % 4u]];
    }
    else
    {
        _124 = 0u;
    }
    uint _125 = _124 / 64u;
    uint _126 = _124 % 64u;
    barrier();
    if (_113)
    {
        uint _130 = atomicAdd(mesh_histogram[_126], 1u);
    }
    barrier();
    for (uint _132 = gl_LocalInvocationIndex; _132 < 64u; _132 += 32u)
    {
        uint _139 = mesh_histogram[_132];
        if (_139 > 0u)
        {
            uint _143 = atomicAdd(group_draw_offset, 1u);
            uint _148 = atomicAdd(_alias_instance_indirection_allocator_buffer_1._m0[push_constants.draw_list_index], _139);
            _alias_draw_list_buffer._m0[_143 + push_constants.list_draw_cmd_offset] = DrawIndexedIndirectCommand(atlas_part_info._m0[push_constants.world_subset][_132].index_count, _139, atlas_part_info._m0[push_constants.world_subset][_132].first_index, 0u, _148);
            mesh_histogram[_132] = _148;
        }
    }
    barrier();
    if (_113)
    {
        uint _160 = atomicAdd(mesh_histogram[_126], 1u);
        _alias_instance_indirection_buffer._m0[_160] = _125;
    }
}

