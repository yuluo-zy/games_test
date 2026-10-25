RWByteAddressBuffer _alias_inout_buf[] : register(u2, space0);
RWByteAddressBuffer _alias_subtotals_buf[] : register(u2, space0);
RWByteAddressBuffer _213[] : register(u2, space0);
cbuffer _214_216
{
    uint _216_m0 : packoffset(c0);
};


static uint3 gl_WorkGroupID;
static uint3 gl_LocalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_WorkGroupID : SV_GroupID;
    uint3 gl_LocalInvocationID : SV_GroupThreadID;
};

groupshared uint shared_data2x[1024];

void comp_main()
{
    uint _67 = gl_LocalInvocationID.x * 2u;
    uint _73 = (4u * (_67 + (gl_WorkGroupID.x * 1024u))) >> 2u;
    uint _76 = _73 + 1u;
    uint2 _80 = min(uint2(_alias_inout_buf[_213[_216_m0].Load(0)].Load(_73 * 4 + 0), _alias_inout_buf[_213[_216_m0].Load(0)].Load(_76 * 4 + 0)), uint2(64u, 64u));
    shared_data2x[_67] = _80.x;
    shared_data2x[_67 + 1u] = _80.y;
    GroupMemoryBarrierWithGroupSync();
    uint _87 = (gl_LocalInvocationID.x >> 0u) << 1u;
    uint _88 = _87 + 1u;
    shared_data2x[_88] += shared_data2x[_87];
    GroupMemoryBarrierWithGroupSync();
    uint _95 = (gl_LocalInvocationID.x >> 1u) << 2u;
    uint _99 = (_95 + 2u) + (gl_LocalInvocationID.x & 1u);
    shared_data2x[_99] += shared_data2x[_95 + 1u];
    GroupMemoryBarrierWithGroupSync();
    uint _106 = (gl_LocalInvocationID.x >> 2u) << 3u;
    uint _110 = (_106 + 4u) + (gl_LocalInvocationID.x & 3u);
    shared_data2x[_110] += shared_data2x[_106 + 3u];
    GroupMemoryBarrierWithGroupSync();
    uint _117 = (gl_LocalInvocationID.x >> 3u) << 4u;
    uint _121 = (_117 + 8u) + (gl_LocalInvocationID.x & 7u);
    shared_data2x[_121] += shared_data2x[_117 + 7u];
    GroupMemoryBarrierWithGroupSync();
    uint _128 = (gl_LocalInvocationID.x >> 4u) << 5u;
    uint _132 = (_128 + 16u) + (gl_LocalInvocationID.x & 15u);
    shared_data2x[_132] += shared_data2x[_128 + 15u];
    GroupMemoryBarrierWithGroupSync();
    uint _139 = (gl_LocalInvocationID.x >> 5u) << 6u;
    uint _143 = (_139 + 32u) + (gl_LocalInvocationID.x & 31u);
    shared_data2x[_143] += shared_data2x[_139 + 31u];
    GroupMemoryBarrierWithGroupSync();
    uint _150 = (gl_LocalInvocationID.x >> 6u) << 7u;
    uint _154 = (_150 + 64u) + (gl_LocalInvocationID.x & 63u);
    shared_data2x[_154] += shared_data2x[_150 + 63u];
    GroupMemoryBarrierWithGroupSync();
    uint _161 = (gl_LocalInvocationID.x >> 7u) << 8u;
    uint _165 = (_161 + 128u) + (gl_LocalInvocationID.x & 127u);
    shared_data2x[_165] += shared_data2x[_161 + 127u];
    GroupMemoryBarrierWithGroupSync();
    uint _172 = (gl_LocalInvocationID.x >> 8u) << 9u;
    uint _176 = (_172 + 256u) + (gl_LocalInvocationID.x & 255u);
    shared_data2x[_176] += shared_data2x[_172 + 255u];
    GroupMemoryBarrierWithGroupSync();
    uint _183 = (gl_LocalInvocationID.x >> 9u) << 10u;
    uint _187 = (_183 + 512u) + (gl_LocalInvocationID.x & 511u);
    shared_data2x[_187] += shared_data2x[_183 + 511u];
    GroupMemoryBarrierWithGroupSync();
    uint _200;
    if (gl_LocalInvocationID.x > 0u)
    {
        _200 = shared_data2x[_67 - 1u];
    }
    else
    {
        _200 = 0u;
    }
    _alias_inout_buf[_213[_216_m0].Load(0)].Store(_73 * 4 + 0, _200);
    _alias_inout_buf[_213[_216_m0].Load(0)].Store(_76 * 4 + 0, shared_data2x[_67]);
    if (0u == gl_LocalInvocationID.x)
    {
        _alias_subtotals_buf[_213[_216_m0].Load(4)].Store(gl_WorkGroupID.x * 4 + 0, shared_data2x[1023]);
    }
}

[numthreads(512, 1, 1)]
void main(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_LocalInvocationID = stage_input.gl_LocalInvocationID;
    comp_main();
}
