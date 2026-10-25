RWByteAddressBuffer _alias_inout_buf[] : register(u2, space0);
ByteAddressBuffer subtotals_buf[] : register(t2, space0);
RWByteAddressBuffer _41[] : register(u2, space0);
cbuffer _42_44
{
    uint _44_m0 : packoffset(c0);
};


static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    _alias_inout_buf[_41[_44_m0].Load(0)].Store(gl_GlobalInvocationID.x * 4 + 0, _alias_inout_buf[_41[_44_m0].Load(0)].Load(gl_GlobalInvocationID.x * 4 + 0) + subtotals_buf[_41[_44_m0].Load(4)].Load((gl_GlobalInvocationID.x / 1024u) * 4 + 0));
}

[numthreads(64, 1, 1)]
void main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
