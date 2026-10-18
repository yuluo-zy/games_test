#version 450
layout(local_size_x = 512, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 13, std430) buffer type_RWByteAddressBuffer
{
    uint _m0[];
} _alias_inout_buf;

shared uint shared_data2x[1024];

void main()
{
    uint _57 = gl_LocalInvocationID.x * 2u;
    uint _63 = (4u * (_57 + (gl_WorkGroupID.x * 1024u))) >> 2u;
    uint _66 = _63 + 1u;
    shared_data2x[_57] = _alias_inout_buf._m0[_63];
    shared_data2x[_57 + 1u] = _alias_inout_buf._m0[_66];
    barrier();
    uint _73 = (gl_LocalInvocationID.x >> 0u) << 1u;
    uint _74 = _73 + 1u;
    shared_data2x[_74] += shared_data2x[_73];
    barrier();
    uint _81 = (gl_LocalInvocationID.x >> 1u) << 2u;
    uint _85 = (_81 + 2u) + (gl_LocalInvocationID.x & 1u);
    shared_data2x[_85] += shared_data2x[_81 + 1u];
    barrier();
    uint _92 = (gl_LocalInvocationID.x >> 2u) << 3u;
    uint _96 = (_92 + 4u) + (gl_LocalInvocationID.x & 3u);
    shared_data2x[_96] += shared_data2x[_92 + 3u];
    barrier();
    uint _103 = (gl_LocalInvocationID.x >> 3u) << 4u;
    uint _107 = (_103 + 8u) + (gl_LocalInvocationID.x & 7u);
    shared_data2x[_107] += shared_data2x[_103 + 7u];
    barrier();
    uint _114 = (gl_LocalInvocationID.x >> 4u) << 5u;
    uint _118 = (_114 + 16u) + (gl_LocalInvocationID.x & 15u);
    shared_data2x[_118] += shared_data2x[_114 + 15u];
    barrier();
    uint _125 = (gl_LocalInvocationID.x >> 5u) << 6u;
    uint _129 = (_125 + 32u) + (gl_LocalInvocationID.x & 31u);
    shared_data2x[_129] += shared_data2x[_125 + 31u];
    barrier();
    uint _136 = (gl_LocalInvocationID.x >> 6u) << 7u;
    uint _140 = (_136 + 64u) + (gl_LocalInvocationID.x & 63u);
    shared_data2x[_140] += shared_data2x[_136 + 63u];
    barrier();
    uint _147 = (gl_LocalInvocationID.x >> 7u) << 8u;
    uint _151 = (_147 + 128u) + (gl_LocalInvocationID.x & 127u);
    shared_data2x[_151] += shared_data2x[_147 + 127u];
    barrier();
    uint _158 = (gl_LocalInvocationID.x >> 8u) << 9u;
    uint _162 = (_158 + 256u) + (gl_LocalInvocationID.x & 255u);
    shared_data2x[_162] += shared_data2x[_158 + 255u];
    barrier();
    uint _169 = (gl_LocalInvocationID.x >> 9u) << 10u;
    uint _173 = (_169 + 512u) + (gl_LocalInvocationID.x & 511u);
    shared_data2x[_173] += shared_data2x[_169 + 511u];
    barrier();
    uint _186;
    if (gl_LocalInvocationID.x > 0u)
    {
        _186 = shared_data2x[_57 - 1u];
    }
    else
    {
        _186 = 0u;
    }
    _alias_inout_buf._m0[_63] = _186;
    _alias_inout_buf._m0[_66] = shared_data2x[_57];
}

