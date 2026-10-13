cbuffer type_Globals : register(b13, space0)
{
    uint2 _Globals_img_in_dims : packoffset(c0);
    uint2 _Globals_img_out_dims : packoffset(c0.z);
};

Texture2D<float4> img_in : register(t14, space0);
RWTexture2D<float4> img_out : register(u15, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    int2 _48 = int2(_Globals_img_in_dims);
    float2 _50 = 7.0f.xx / float2(_48);
    float _54 = float(_48.x);
    float _59 = float(_48.y);
    float2 _61 = float2(float(int3(gl_GlobalInvocationID).x) / _54, float(int3(gl_GlobalInvocationID).y) / _59);
    uint2 _62 = uint2(int3(gl_GlobalInvocationID).xy);
    float4 _66;
    float _69;
    _66 = img_in.Load(int3(_62, 0u));
    _69 = 0.0f;
    float4 _67;
    for (; _69 < 6.283185482025146484375f; _66 = _67, _69 += 0.2094395160675048828125f)
    {
        _67 = _66;
        for (float _77 = 0.100000001490116119384765625f; _77 <= 1.0f; )
        {
            float2 _86 = _61 + ((float2(cos(_69), sin(_69)) * _50) * _77);
            _67 += img_in.Load(int3(uint2(int2(int(_86.x * _54), int(_86.y * _59))), 0u));
            _77 += 0.100000001490116119384765625f;
            continue;
        }
    }
    img_out[_62] = float4((_66 * 0.00350877200253307819366455078125f.xxxx).xxx, 1.0f);
}

[numthreads(8, 8, 1)]
void main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
