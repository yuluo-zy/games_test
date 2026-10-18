#version 450
layout(local_size_x = 512, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 13, std430) buffer type_RWByteAddressBuffer
{
    uint _m0[];
} _alias_inout_buf;

layout(set = 0, binding = 14, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_subtotals_buf;

layout(set = 0, binding = 15, std430) buffer _alias_draw_list_length_buffer
{
    uint _m0[];
} _alias_draw_list_length_buffer_1;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint range_start;
    uint range_end;
    uint subtotals_start;
    uint subtotals_end;
    uint draw_list_index;
} push_constants;

shared uint shared_data2x[1024];

void main()
{
    uint _84 = gl_LocalInvocationID.x * 2u;
    uint _85 = _84 + (push_constants.range_start + (gl_WorkGroupID.x * 1024u));
    bool _87 = (_85 + 1u) < push_constants.range_end;
    uvec2 _113;
    if (_87)
    {
        uint _94 = (4u * (_85)) >> 2u;
        _113 = uvec2(_alias_inout_buf._m0[_94], _alias_inout_buf._m0[_94 + 1u]);
    }
    else
    {
        uvec2 _112;
        if (_85 < push_constants.range_end)
        {
            _112 = uvec2(_alias_inout_buf._m0[(4u * (_85)) >> 2u], 0u);
        }
        else
        {
            _112 = uvec2(0u);
        }
        _113 = _112;
    }
    shared_data2x[_84] = _113.x;
    shared_data2x[_84 + 1u] = _113.y;
    barrier();
    uint _120 = (gl_LocalInvocationID.x >> 0u) << 1u;
    uint _121 = _120 + 1u;
    shared_data2x[_121] += shared_data2x[_120];
    barrier();
    uint _128 = (gl_LocalInvocationID.x >> 1u) << 2u;
    uint _132 = (_128 + 2u) + (gl_LocalInvocationID.x & 1u);
    shared_data2x[_132] += shared_data2x[_128 + 1u];
    barrier();
    uint _139 = (gl_LocalInvocationID.x >> 2u) << 3u;
    uint _143 = (_139 + 4u) + (gl_LocalInvocationID.x & 3u);
    shared_data2x[_143] += shared_data2x[_139 + 3u];
    barrier();
    uint _150 = (gl_LocalInvocationID.x >> 3u) << 4u;
    uint _154 = (_150 + 8u) + (gl_LocalInvocationID.x & 7u);
    shared_data2x[_154] += shared_data2x[_150 + 7u];
    barrier();
    uint _161 = (gl_LocalInvocationID.x >> 4u) << 5u;
    uint _165 = (_161 + 16u) + (gl_LocalInvocationID.x & 15u);
    shared_data2x[_165] += shared_data2x[_161 + 15u];
    barrier();
    uint _172 = (gl_LocalInvocationID.x >> 5u) << 6u;
    uint _176 = (_172 + 32u) + (gl_LocalInvocationID.x & 31u);
    shared_data2x[_176] += shared_data2x[_172 + 31u];
    barrier();
    uint _183 = (gl_LocalInvocationID.x >> 6u) << 7u;
    uint _187 = (_183 + 64u) + (gl_LocalInvocationID.x & 63u);
    shared_data2x[_187] += shared_data2x[_183 + 63u];
    barrier();
    uint _194 = (gl_LocalInvocationID.x >> 7u) << 8u;
    uint _198 = (_194 + 128u) + (gl_LocalInvocationID.x & 127u);
    shared_data2x[_198] += shared_data2x[_194 + 127u];
    barrier();
    uint _205 = (gl_LocalInvocationID.x >> 8u) << 9u;
    uint _209 = (_205 + 256u) + (gl_LocalInvocationID.x & 255u);
    shared_data2x[_209] += shared_data2x[_205 + 255u];
    barrier();
    uint _216 = (gl_LocalInvocationID.x >> 9u) << 10u;
    uint _220 = (_216 + 512u) + (gl_LocalInvocationID.x & 511u);
    shared_data2x[_220] += shared_data2x[_216 + 511u];
    barrier();
    uint _233;
    if (gl_LocalInvocationID.x > 0u)
    {
        _233 = shared_data2x[_84 - 1u];
    }
    else
    {
        _233 = 0u;
    }
    if (_87)
    {
        uint _241 = (4u * (_85)) >> 2u;
        _alias_inout_buf._m0[_241] = _233;
        _alias_inout_buf._m0[_241 + 1u] = shared_data2x[_84];
    }
    else
    {
        if (_85 < push_constants.range_end)
        {
            _alias_inout_buf._m0[(4u * (_85)) >> 2u] = _233;
        }
    }
    uint _254 = shared_data2x[1023];
    if (0u == gl_LocalInvocationID.x)
    {
        uint _259 = atomicAdd(_alias_draw_list_length_buffer_1._m0[push_constants.draw_list_index], _254);
        uint _263 = (1u + gl_WorkGroupID.x) + push_constants.subtotals_start;
        if (_263 < push_constants.subtotals_end)
        {
            _alias_subtotals_buf._m0[_263] = _254;
        }
    }
}

