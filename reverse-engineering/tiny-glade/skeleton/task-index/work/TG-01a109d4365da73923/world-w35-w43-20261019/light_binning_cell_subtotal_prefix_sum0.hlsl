RWByteAddressBuffer push_constants[] : register(u2, space0);
RWByteAddressBuffer _alias_inout_buf[] : register(u2, space0);
cbuffer _222_224
{
    uint _224_m0 : packoffset(c0);
};


static uint3 gl_LocalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_LocalInvocationID : SV_GroupThreadID;
};

groupshared uint shared_data2x[512];

void comp_main()
{
    uint _60 = push_constants[_224_m0].Load(0);
    uint _61 = gl_LocalInvocationID.x * 2u;
    uint _62 = _61 + 1u;
    bool _63 = _62 < _60;
    uint2 _89;
    if (_63)
    {
        uint _70 = (4u * (_61)) >> 2u;
        _89 = uint2(_alias_inout_buf[push_constants[_224_m0].Load(4)].Load(_70 * 4 + 0), _alias_inout_buf[push_constants[_224_m0].Load(4)].Load((_70 + 1u) * 4 + 0));
    }
    else
    {
        uint2 _88;
        if (_61 < _60)
        {
            _88 = uint2(_alias_inout_buf[push_constants[_224_m0].Load(4)].Load(((4u * (_61)) >> 2u) * 4 + 0), 0u);
        }
        else
        {
            _88 = uint2(0u, 0u);
        }
        _89 = _88;
    }
    shared_data2x[_61] = _89.x;
    shared_data2x[_62] = _89.y;
    GroupMemoryBarrierWithGroupSync();
    uint _95 = (gl_LocalInvocationID.x >> 0u) << 1u;
    uint _96 = _95 + 1u;
    shared_data2x[_96] += shared_data2x[_95];
    GroupMemoryBarrierWithGroupSync();
    uint _103 = (gl_LocalInvocationID.x >> 1u) << 2u;
    uint _107 = (_103 + 2u) + (gl_LocalInvocationID.x & 1u);
    shared_data2x[_107] += shared_data2x[_103 + 1u];
    GroupMemoryBarrierWithGroupSync();
    uint _114 = (gl_LocalInvocationID.x >> 2u) << 3u;
    uint _118 = (_114 + 4u) + (gl_LocalInvocationID.x & 3u);
    shared_data2x[_118] += shared_data2x[_114 + 3u];
    GroupMemoryBarrierWithGroupSync();
    uint _125 = (gl_LocalInvocationID.x >> 3u) << 4u;
    uint _129 = (_125 + 8u) + (gl_LocalInvocationID.x & 7u);
    shared_data2x[_129] += shared_data2x[_125 + 7u];
    GroupMemoryBarrierWithGroupSync();
    uint _136 = (gl_LocalInvocationID.x >> 4u) << 5u;
    uint _140 = (_136 + 16u) + (gl_LocalInvocationID.x & 15u);
    shared_data2x[_140] += shared_data2x[_136 + 15u];
    GroupMemoryBarrierWithGroupSync();
    uint _147 = (gl_LocalInvocationID.x >> 5u) << 6u;
    uint _151 = (_147 + 32u) + (gl_LocalInvocationID.x & 31u);
    shared_data2x[_151] += shared_data2x[_147 + 31u];
    GroupMemoryBarrierWithGroupSync();
    uint _158 = (gl_LocalInvocationID.x >> 6u) << 7u;
    uint _162 = (_158 + 64u) + (gl_LocalInvocationID.x & 63u);
    shared_data2x[_162] += shared_data2x[_158 + 63u];
    GroupMemoryBarrierWithGroupSync();
    uint _169 = (gl_LocalInvocationID.x >> 7u) << 8u;
    uint _173 = (_169 + 128u) + (gl_LocalInvocationID.x & 127u);
    shared_data2x[_173] += shared_data2x[_169 + 127u];
    GroupMemoryBarrierWithGroupSync();
    uint _180 = (gl_LocalInvocationID.x >> 8u) << 9u;
    uint _184 = (_180 + 256u) + (gl_LocalInvocationID.x & 255u);
    shared_data2x[_184] += shared_data2x[_180 + 255u];
    GroupMemoryBarrierWithGroupSync();
    uint _197;
    if (gl_LocalInvocationID.x > 0u)
    {
        _197 = shared_data2x[_61 - 1u];
    }
    else
    {
        _197 = 0u;
    }
    if (_63)
    {
        uint _205 = (4u * (_61)) >> 2u;
        _alias_inout_buf[push_constants[_224_m0].Load(4)].Store(_205 * 4 + 0, _197);
        _alias_inout_buf[push_constants[_224_m0].Load(4)].Store((_205 + 1u) * 4 + 0, shared_data2x[_61]);
    }
    else
    {
        if (_61 < _60)
        {
            _alias_inout_buf[push_constants[_224_m0].Load(4)].Store(((4u * (_61)) >> 2u) * 4 + 0, _197);
        }
    }
}

[numthreads(256, 1, 1)]
void main(SPIRV_Cross_Input stage_input)
{
    gl_LocalInvocationID = stage_input.gl_LocalInvocationID;
    comp_main();
}
