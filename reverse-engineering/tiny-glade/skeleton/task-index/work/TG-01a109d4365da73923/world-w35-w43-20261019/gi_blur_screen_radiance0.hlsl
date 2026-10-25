struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

static const float4 _54[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
static const uint2 _92[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

RWByteAddressBuffer frame_constants[] : register(u2, space0);
RWByteAddressBuffer _Globals[] : register(u2, space0);
RWByteAddressBuffer _223[] : register(u2, space0);
cbuffer _224_226
{
    uint _226_m0 : packoffset(c0);
};

Texture2D<float4> half_depth_tex[] : register(t0, space0);
Texture2D<float4> half_view_normal_tex[] : register(t0, space0);
Texture2D<float4> ssgi_tex[] : register(t0, space0);
Texture2D<float4> input_tex[] : register(t0, space0);
RWTexture2D<float4> output_tex[] : register(u1, space0);
RWTexture2D<unorm float4> temporal_stats_out_tex[] : register(u1, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    float4 _100 = half_depth_tex[_223[_226_m0].Load(4)].Load(int3(gl_GlobalInvocationID.xy, 0u));
    float _101 = _100.x;
    float _105 = max(0.300000011920928955078125f, half_view_normal_tex[_223[_226_m0].Load(8)].Load(int3(gl_GlobalInvocationID.xy, 0u)).z);
    float4 _96[3] = _54;
    uint _109 = frame_constants[_223[_226_m0].Load(28)].Load(128) & 3u;
    float4 _114 = ssgi_tex[_223[_226_m0].Load(12)].Load(int3((gl_GlobalInvocationID.xy * uint2(2u, 2u)) + _92[_109], 0u));
    float _115 = _114.x;
    float3 _117 = half_view_normal_tex[_223[_226_m0].Load(8)].Load(int3(gl_GlobalInvocationID.xy, 0u)).xyz;
    float _123 = ((temporal_stats_out_tex[_223[_226_m0].Load(24)][gl_GlobalInvocationID.xy] * float4(255.0f, 1.0f, 1.0f, 1.0f)).x > 33.0f) ? 1.0f : 3.0f;
    float _125;
    _125 = 0.0f;
    float _126;
    for (int _128 = -1; _128 <= 1; _125 = _126, _128++)
    {
        _126 = _125;
        float _134;
        for (int _136 = -1; _136 <= 1; _126 = _134, _136++)
        {
            uint2 _146 = gl_GlobalInvocationID.xy + uint2(int2(float2(int2(_136, _128)) * _123));
            float _169 = dot(_117, half_view_normal_tex[_223[_226_m0].Load(8)].Load(int3(_146, 0u)).xyz);
            float _172 = ((_169 * _169) * (-0.89999997615814208984375f)) + 1.0f;
            float _175 = (exp2(-min(30.0f, 80.0f * (_105 * abs(1.0f - (half_depth_tex[_223[_226_m0].Load(4)].Load(int3(_146, 0u)).x / _101))))) * exp2((-20.0f) * abs(ssgi_tex[_223[_226_m0].Load(12)].Load(int3((_146 * uint2(2u, 2u)) + _92[_109], 0u)).x - _115))) * (0.010000000707805156707763671875f / (_172 * _172));
            for (uint _177 = 0u; _177 < 3u; )
            {
                _96[_177] += (input_tex[_223[_226_m0].Load(16)].Load(int3(_146 + uint2(_177 * _Globals[_223[_226_m0].Load(0)].Load(0), 0u), 0u)) * _175);
                _177++;
                continue;
            }
            _134 = _126 + _175;
        }
    }
    for (uint _194 = 0u; _194 < 3u; )
    {
        _96[_194] /= max(1.0000000133514319600180897396058e-10f, _125).xxxx;
        _194++;
        continue;
    }
    for (uint _205 = 0u; _205 < 3u; )
    {
        output_tex[_223[_226_m0].Load(20)][gl_GlobalInvocationID.xy + uint2(_205 * _Globals[_223[_226_m0].Load(0)].Load(0), 0u)] = _96[_205];
        _205++;
        continue;
    }
}

[numthreads(8, 8, 1)]
void main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
