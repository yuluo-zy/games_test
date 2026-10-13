#define PERM_fmt "r16"
#define PERM_ty float
#define TGV_dc5fd7e4b9bf9935 1
#line 1 "shaders/gauss_blur.cs.hlsl"




#if defined(TGV_b9dd0465835cd591) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    uint2 img_in_dims : packoffset(c0);
    uint2 img_out_dims : packoffset(c0.z);
    uint is_horizontal : packoffset(c1);
    float sigma : packoffset(c1.y);
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
    int _55 = int(ceil(sigma * 2.0f));
    int _56 = -_55;
    float4 _58;
    float _61;
    _58 = 0.0f.xxxx;
    _61 = 0.0f;
    float4 _59;
    float _62;
    for (int _63 = _56; _63 <= _55; _58 = _59, _61 = _62, _63++)
    {
        int2 _78;
        if (!(is_horizontal != 0u))
        {
            _78 = int3(gl_GlobalInvocationID).xy + int2(0, _63);
        }
        else
        {
            _78 = int3(gl_GlobalInvocationID).xy + int2(_63, 0);
        }
        uint2 _79 = uint2(_78);
        float4 _81 = img_in.Load(int3(_79, 0u));
        bool _91;
        if (!any(bool2(_78.x < int2(0, 0).x, _78.y < int2(0, 0).y)))
        {
            _91 = any(bool2(_79.x >= img_in_dims.x, _79.y >= img_in_dims.y));
        }
        else
        {
            _91 = true;
        }
        float4 _95;
        if (_91)
        {
            float4 _94 = _81;
            _94.x = 0.09433962404727935791015625f;
            _95 = _94;
        }
        else
        {
            _95 = _81;
        }
        float _96 = float(_63);
        float _102 = exp(((-_96) * _96) / ((2.0f * sigma) * sigma));
        _59 = _58 + (_95 * _102);
        _62 = _61 + _102;
    }
    img_out[uint2(int3(gl_GlobalInvocationID).xy)] = _58 / _61.xxxx;
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_dc5fd7e4b9bf9935) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    uint2 img_in_dims : packoffset(c0);
    uint2 img_out_dims : packoffset(c0.z);
    uint is_horizontal : packoffset(c1);
    float sigma : packoffset(c1.y);
};

Texture2D<float4> img_in : register(t14, space0);
RWTexture2D<unorm float> img_out : register(u15, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    int _54 = int(ceil(sigma * 2.0f));
    int _55 = -_54;
    float _57;
    float _60;
    _57 = 0.0f;
    _60 = 0.0f;
    float _58;
    float _61;
    for (int _62 = _55; _62 <= _54; _57 = _58, _60 = _61, _62++)
    {
        int2 _77;
        if (!(is_horizontal != 0u))
        {
            _77 = int3(gl_GlobalInvocationID).xy + int2(0, _62);
        }
        else
        {
            _77 = int3(gl_GlobalInvocationID).xy + int2(_62, 0);
        }
        uint2 _78 = uint2(_77);
        bool _91;
        if (!any(bool2(_77.x < int2(0, 0).x, _77.y < int2(0, 0).y)))
        {
            _91 = any(bool2(_78.x >= img_in_dims.x, _78.y >= img_in_dims.y));
        }
        else
        {
            _91 = true;
        }
        float _93 = float(_62);
        float _99 = exp(((-_93) * _93) / ((2.0f * sigma) * sigma));
        _58 = _57 + ((_91 ? 0.09433962404727935791015625f : img_in.Load(int3(_78, 0u)).x) * _99);
        _61 = _60 + _99;
    }
    img_out[uint2(int3(gl_GlobalInvocationID).xy)] = _57 / _60;
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
