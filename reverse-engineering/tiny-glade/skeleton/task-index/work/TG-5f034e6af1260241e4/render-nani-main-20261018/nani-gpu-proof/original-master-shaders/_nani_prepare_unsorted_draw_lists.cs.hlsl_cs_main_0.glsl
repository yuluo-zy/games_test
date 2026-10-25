#version 450
#extension GL_EXT_nonuniform_qualifier : require
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

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

layout(set = 0, binding = 13, std430) readonly buffer type_StructuredBuffer_
{
    MeshAtlasPartInfo _m0[][64];
} atlas_part_info;

layout(set = 0, binding = 14, std430) buffer type_RWStructuredBuffer_DrawIndexedIndirectCommand
{
    DrawIndexedIndirectCommand _m0[];
} _alias_draw_list_buffer;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint world_subset;
    uint list_draw_cmd_offset;
} push_constants;

void main()
{
    _alias_draw_list_buffer._m0[gl_GlobalInvocationID.x + push_constants.list_draw_cmd_offset] = DrawIndexedIndirectCommand(atlas_part_info._m0[push_constants.world_subset][gl_GlobalInvocationID.x].index_count, 0u, atlas_part_info._m0[push_constants.world_subset][gl_GlobalInvocationID.x].first_index, 0u, 0u);
}

