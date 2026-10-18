#version 450
layout(local_size_x = 512, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 13, std430) buffer type_RWByteAddressBuffer
{
    uint _m0[];
} _alias_inout_buf;

layout(push_constant, std430) uniform type_ConstantBuffer_PushConstants
{
    uint range_start;
    uint range_end;
} push_constants;

shared uint shared_data2x[1024];

void main()
{
    uint _69 = gl_LocalInvocationID.x * 2u;
    uint _70 = _69 + push_constants.range_start;
    bool _72 = (_70 + 1u) < push_constants.range_end;
    uvec2 _98;
    if (_72)
    {
        uint _79 = (4u * (_70)) >> 2u;
        _98 = uvec2(_alias_inout_buf._m0[_79], _alias_inout_buf._m0[_79 + 1u]);
    }
    else
    {
        uvec2 _97;
        if (_70 < push_constants.range_end)
        {
            _97 = uvec2(_alias_inout_buf._m0[(4u * (_70)) >> 2u], 0u);
        }
        else
        {
            _97 = uvec2(0u);
        }
        _98 = _97;
    }
    shared_data2x[_69] = _98.x;
    uint _102 = _69 + 1u;
    shared_data2x[_102] = _98.y;
    barrier();
    uint _105 = (gl_LocalInvocationID.x >> 0u) << 1u;
    uint _106 = _105 + 1u;
    shared_data2x[_106] += shared_data2x[_105];
    barrier();
    uint _113 = (gl_LocalInvocationID.x >> 1u) << 2u;
    uint _117 = (_113 + 2u) + (gl_LocalInvocationID.x & 1u);
    shared_data2x[_117] += shared_data2x[_113 + 1u];
    barrier();
    uint _124 = (gl_LocalInvocationID.x >> 2u) << 3u;
    uint _128 = (_124 + 4u) + (gl_LocalInvocationID.x & 3u);
    shared_data2x[_128] += shared_data2x[_124 + 3u];
    barrier();
    uint _135 = (gl_LocalInvocationID.x >> 3u) << 4u;
    uint _139 = (_135 + 8u) + (gl_LocalInvocationID.x & 7u);
    shared_data2x[_139] += shared_data2x[_135 + 7u];
    barrier();
    uint _146 = (gl_LocalInvocationID.x >> 4u) << 5u;
    uint _150 = (_146 + 16u) + (gl_LocalInvocationID.x & 15u);
    shared_data2x[_150] += shared_data2x[_146 + 15u];
    barrier();
    uint _157 = (gl_LocalInvocationID.x >> 5u) << 6u;
    uint _161 = (_157 + 32u) + (gl_LocalInvocationID.x & 31u);
    shared_data2x[_161] += shared_data2x[_157 + 31u];
    barrier();
    uint _168 = (gl_LocalInvocationID.x >> 6u) << 7u;
    uint _172 = (_168 + 64u) + (gl_LocalInvocationID.x & 63u);
    shared_data2x[_172] += shared_data2x[_168 + 63u];
    barrier();
    uint _179 = (gl_LocalInvocationID.x >> 7u) << 8u;
    uint _183 = (_179 + 128u) + (gl_LocalInvocationID.x & 127u);
    shared_data2x[_183] += shared_data2x[_179 + 127u];
    barrier();
    uint _190 = (gl_LocalInvocationID.x >> 8u) << 9u;
    uint _194 = (_190 + 256u) + (gl_LocalInvocationID.x & 255u);
    shared_data2x[_194] += shared_data2x[_190 + 255u];
    barrier();
    uint _201 = (gl_LocalInvocationID.x >> 9u) << 10u;
    uint _205 = (_201 + 512u) + (gl_LocalInvocationID.x & 511u);
    shared_data2x[_205] += shared_data2x[_201 + 511u];
    barrier();
    if (_72)
    {
        uint _219 = (4u * (_70)) >> 2u;
        _alias_inout_buf._m0[_219] = shared_data2x[_69];
        _alias_inout_buf._m0[_219 + 1u] = shared_data2x[_102];
    }
    else
    {
        if (_70 < push_constants.range_end)
        {
            _alias_inout_buf._m0[(4u * (_70)) >> 2u] = shared_data2x[_69];
        }
    }
}

