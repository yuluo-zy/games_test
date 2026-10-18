#version 450
#extension GL_EXT_nonuniform_qualifier : require
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

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

layout(set = 0, binding = 14, std430) buffer type_RWStructuredBuffer_DrawIndexedIndirectCommand
{
    DrawIndexedIndirectCommand _m0[];
} _alias_draw_list_buffer;

layout(set = 0, binding = 15, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_unsorted_instance_atlas_entry_prefix_buffer;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint draw_list_index;
    uint list_draw_cmd_offset;
} push_constants;

shared uint shared_data1x[64];

void main()
{
    uint _75 = gl_GlobalInvocationID.x + push_constants.list_draw_cmd_offset;
    uint _78 = _alias_draw_list_buffer._m0[_75].index_count;
    uint _80 = _alias_draw_list_buffer._m0[_75].instance_count;
    uint _82 = _alias_draw_list_buffer._m0[_75].first_index;
    uint _84 = _alias_draw_list_buffer._m0[_75].vertex_offset;
    uint _85 = gl_GlobalInvocationID.x % 64u;
    shared_data1x[_85] = _80;
    barrier();
    uint _88 = (_85 >> 0u) << 1u;
    uint _89 = _88 + 1u;
    if (_89 < 64u)
    {
        shared_data1x[_89] += shared_data1x[_88];
    }
    barrier();
    uint _99 = (_85 >> 1u) << 2u;
    uint _103 = (_99 + 2u) + (_85 & 1u);
    if (_103 < 64u)
    {
        shared_data1x[_103] += shared_data1x[_99 + 1u];
    }
    barrier();
    uint _113 = (_85 >> 2u) << 3u;
    uint _117 = (_113 + 4u) + (_85 & 3u);
    if (_117 < 64u)
    {
        shared_data1x[_117] += shared_data1x[_113 + 3u];
    }
    barrier();
    uint _127 = (_85 >> 3u) << 4u;
    uint _131 = (_127 + 8u) + (_85 & 7u);
    if (_131 < 64u)
    {
        shared_data1x[_131] += shared_data1x[_127 + 7u];
    }
    barrier();
    uint _141 = (_85 >> 4u) << 5u;
    uint _145 = (_141 + 16u) + (_85 & 15u);
    if (_145 < 64u)
    {
        shared_data1x[_145] += shared_data1x[_141 + 15u];
    }
    barrier();
    uint _155 = (_85 >> 5u) << 6u;
    uint _159 = (_155 + 32u) + (_85 & 31u);
    if (_159 < 64u)
    {
        shared_data1x[_159] += shared_data1x[_155 + 31u];
    }
    barrier();
    uint _175;
    if (_85 == 0u)
    {
        _175 = 0u;
    }
    else
    {
        _175 = shared_data1x[_85 - 1u];
    }
    uint _180 = _175 + instance_list_offset.instance_list_offset_array[push_constants.draw_list_index / 4u][push_constants.draw_list_index % 4u];
    _alias_draw_list_buffer._m0[_75] = DrawIndexedIndirectCommand(_78, _80, _82, _84, _180);
    _alias_unsorted_instance_atlas_entry_prefix_buffer._m0[gl_GlobalInvocationID.x + (push_constants.draw_list_index * 64u)] = _180;
}

