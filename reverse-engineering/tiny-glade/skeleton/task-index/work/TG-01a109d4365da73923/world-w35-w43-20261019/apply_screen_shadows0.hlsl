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

static float _151;

RWByteAddressBuffer frame_constants[] : register(u2, space0);
RWByteAddressBuffer _Globals[] : register(u2, space0);
RWByteAddressBuffer _alias_shadow_mask_histogram[] : register(u2, space0);
RWByteAddressBuffer _528[] : register(u2, space0);
cbuffer _529_531
{
    uint _531_m0 : packoffset(c0);
};

SamplerState sampler_lnc[] : register(s3, space0);
SamplerState sampler_shadow_nearest[] : register(s3, space0);
SamplerComparisonState sampler_shadow_linear[] : register(s3, space0);
Texture2D<uint4> gbuffer_tex[] : register(t0, space0);
Texture2D<float4> depth_tex[] : register(t0, space0);
Texture2D<float4> history_tex[] : register(t0, space0);
Texture2D<float4> reprojection_tex[] : register(t0, space0);
Texture2DArray<float4> shadow_map[] : register(t0, space0);
RWTexture2D<unorm float> output_tex[] : register(u1, space0);

static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint thread_group_shadow_mask_sum;
groupshared uint thread_group_alive;

void comp_main()
{
    bool _162 = 0u == gl_LocalInvocationIndex;
    if (_162)
    {
        thread_group_alive = 0u;
        thread_group_shadow_mask_sum = 0u;
    }
    GroupMemoryBarrierWithGroupSync();
    uint4 _169 = gbuffer_tex[_528[_531_m0].Load(16)].Load(int3(gl_GlobalInvocationID.xy, 0u));
    uint _170 = _169.x;
    float3 _185 = (float4(float(_170 & 255u) * 0.0039215688593685626983642578125f, float((_170 >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((_170 >> 16u) & 255u) * 0.0039215688593685626983642578125f, _151).xyz * 2.0f) - 1.0f.xxx;
    float4x4 _187 = asfloat(uint4x4(frame_constants[_528[_531_m0].Load(44)].Load(2160), frame_constants[_528[_531_m0].Load(44)].Load(2176), frame_constants[_528[_531_m0].Load(44)].Load(2192), frame_constants[_528[_531_m0].Load(44)].Load(2208), frame_constants[_528[_531_m0].Load(44)].Load(2164), frame_constants[_528[_531_m0].Load(44)].Load(2180), frame_constants[_528[_531_m0].Load(44)].Load(2196), frame_constants[_528[_531_m0].Load(44)].Load(2212), frame_constants[_528[_531_m0].Load(44)].Load(2168), frame_constants[_528[_531_m0].Load(44)].Load(2184), frame_constants[_528[_531_m0].Load(44)].Load(2200), frame_constants[_528[_531_m0].Load(44)].Load(2216), frame_constants[_528[_531_m0].Load(44)].Load(2172), frame_constants[_528[_531_m0].Load(44)].Load(2188), frame_constants[_528[_531_m0].Load(44)].Load(2204), frame_constants[_528[_531_m0].Load(44)].Load(2220)));
    float3 _189 = mul(_187, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    int2 _190 = int2(gl_GlobalInvocationID.xy);
    float2 _196 = (float2(_190) + 0.5f.xx) / float2(_Globals[_528[_531_m0].Load(12)].Load2(8));
    float4x4 _198 = asfloat(uint4x4(frame_constants[_528[_531_m0].Load(44)].Load(432), frame_constants[_528[_531_m0].Load(44)].Load(448), frame_constants[_528[_531_m0].Load(44)].Load(464), frame_constants[_528[_531_m0].Load(44)].Load(480), frame_constants[_528[_531_m0].Load(44)].Load(436), frame_constants[_528[_531_m0].Load(44)].Load(452), frame_constants[_528[_531_m0].Load(44)].Load(468), frame_constants[_528[_531_m0].Load(44)].Load(484), frame_constants[_528[_531_m0].Load(44)].Load(440), frame_constants[_528[_531_m0].Load(44)].Load(456), frame_constants[_528[_531_m0].Load(44)].Load(472), frame_constants[_528[_531_m0].Load(44)].Load(488), frame_constants[_528[_531_m0].Load(44)].Load(444), frame_constants[_528[_531_m0].Load(44)].Load(460), frame_constants[_528[_531_m0].Load(44)].Load(476), frame_constants[_528[_531_m0].Load(44)].Load(492)));
    float4x4 _200 = asfloat(uint4x4(frame_constants[_528[_531_m0].Load(44)].Load(560), frame_constants[_528[_531_m0].Load(44)].Load(576), frame_constants[_528[_531_m0].Load(44)].Load(592), frame_constants[_528[_531_m0].Load(44)].Load(608), frame_constants[_528[_531_m0].Load(44)].Load(564), frame_constants[_528[_531_m0].Load(44)].Load(580), frame_constants[_528[_531_m0].Load(44)].Load(596), frame_constants[_528[_531_m0].Load(44)].Load(612), frame_constants[_528[_531_m0].Load(44)].Load(568), frame_constants[_528[_531_m0].Load(44)].Load(584), frame_constants[_528[_531_m0].Load(44)].Load(600), frame_constants[_528[_531_m0].Load(44)].Load(616), frame_constants[_528[_531_m0].Load(44)].Load(572), frame_constants[_528[_531_m0].Load(44)].Load(588), frame_constants[_528[_531_m0].Load(44)].Load(604), frame_constants[_528[_531_m0].Load(44)].Load(620)));
    float2 _201 = _196 - 0.5f.xx;
    float4 _207 = mul(_200, mul(_198, float4(_201 * float2(2.0f, -2.0f), depth_tex[_528[_531_m0].Load(20)].Load(int3(gl_GlobalInvocationID.xy, 0u)).x, 1.0f)));
    float2 _436;
    float _437;
    do
    {
        float4x4 _222 = asfloat(uint4x4(frame_constants[_528[_531_m0].Load(44)].Load(2224), frame_constants[_528[_531_m0].Load(44)].Load(2240), frame_constants[_528[_531_m0].Load(44)].Load(2256), frame_constants[_528[_531_m0].Load(44)].Load(2272), frame_constants[_528[_531_m0].Load(44)].Load(2228), frame_constants[_528[_531_m0].Load(44)].Load(2244), frame_constants[_528[_531_m0].Load(44)].Load(2260), frame_constants[_528[_531_m0].Load(44)].Load(2276), frame_constants[_528[_531_m0].Load(44)].Load(2232), frame_constants[_528[_531_m0].Load(44)].Load(2248), frame_constants[_528[_531_m0].Load(44)].Load(2264), frame_constants[_528[_531_m0].Load(44)].Load(2280), frame_constants[_528[_531_m0].Load(44)].Load(2236), frame_constants[_528[_531_m0].Load(44)].Load(2252), frame_constants[_528[_531_m0].Load(44)].Load(2268), frame_constants[_528[_531_m0].Load(44)].Load(2284)));
        float4 _227 = mul(_222, float4((_207.xyz / _207.w.xxx) + (_185 * (0.066666670143604278564453125f * clamp(1.0f - dot(_185, _189), 0.0f, 1.0f))), 1.0f));
        if (_227.z > 1.0f)
        {
            _436 = 0.0f.xx;
            _437 = 0.0f;
            break;
        }
        float2 _232 = _227.xy;
        float2 _233 = abs(_232);
        bool _235 = any(bool2(_233.x > 0.9833333492279052734375f.xx.x, _233.y > 0.9833333492279052734375f.xx.y));
        float2 _239;
        if (_235)
        {
            _239 = _232 * 0.5f;
        }
        else
        {
            _239 = _232;
        }
        float _240 = _235 ? 5.0f : 10.0f;
        float _242 = _235 ? 0.5f : 1.0f;
        float2 _243 = abs(_239);
        float2 _246 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * _240));
        bool _248 = any(bool2(_243.x > _246.x, _243.y > _246.y));
        float _254;
        float2 _255;
        float _256;
        if (_248)
        {
            _254 = _240 * 0.2857142984867095947265625f;
            _255 = _239 * 0.2857142984867095947265625f;
            _256 = _242 * 0.2857142984867095947265625f;
        }
        else
        {
            _254 = _240;
            _255 = _239;
            _256 = _242;
        }
        float _259 = float(_248 ? 2u : uint(_235));
        float4x4 _263 = asfloat(uint4x4(frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 624), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 640), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 656), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 672), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 628), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 644), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 660), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 676), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 632), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 648), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 664), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 680), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 636), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 652), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 668), frame_constants[_528[_531_m0].Load(44)].Load((2u + uint(_259)) * 800 + 684)));
        float _264 = dot(_185, -_189);
        float4 _276 = float4(_255.x, _255.y, _227.z, _227.w) + mul(_263, float4((_185 * (0.066666670143604278564453125f * clamp(1.0f - _264, 0.0f, 1.0f))) / _256.xxx, 0.0f));
        float2 _279 = (_276.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        float _285 = max((0.300000011920928955078125f / _256) * (1.0f - abs(_264)), 0.02500000037252902984619140625f);
        float _298 = float((uint((_190.x * 11) + (_190.y * 7)) + (frame_constants[_528[_531_m0].Load(44)].Load(128) * 5u)) % 64u) * 1.61803400516510009765625f;
        uint2 _299 = uint2(_190);
        float _304 = 5.588237762451171875f * float(frame_constants[_528[_531_m0].Load(44)].Load(128) % 64u);
        float _314 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_299.x) + _304)) + (0.005837149918079376220703125f * (float(_299.y) + _304))));
        float4 _320 = mul(_263, float4(_185, 0.0f));
        float _322;
        int _325;
        _322 = 0.0f;
        _325 = 0;
        float _323;
        int _326;
        for (int _327 = 0; _327 < 5; _322 = _323, _325 = _326, _327++)
        {
            float _332 = float(_327);
            float _334 = (_332 + _298) * 2.3999631404876708984375f;
            float3 _347 = float3(_279.x, _279.y, _276.z) + float3((((float2(cos(_334), sin(_334)) * (_332 + _314)) * 0.20000000298023223876953125f.xx) * _254) * 0.000833333353511989116668701171875f.xx, _285 * (-0.0009765625f));
            float4 _354 = shadow_map[_528[_531_m0].Load(32)].SampleLevel(sampler_shadow_nearest[_528[_531_m0].Load(4)], float3(_347.xy, _259), 0.0f);
            float _355 = _354.x;
            if (_347.z > _355)
            {
                _323 = _322 + _355;
                _326 = _325 + 1;
            }
            else
            {
                _323 = _322;
                _326 = _325;
            }
        }
        float _363 = _322 / float(_325);
        if (_325 == 0)
        {
            _436 = 0.0f.xx;
            _437 = 0.0f;
            break;
        }
        float _377 = max(((_276.z - _363) * ((17.0666675567626953125f * _254) * pow(10.0f, smoothstep(0.0f, 0.25f, asfloat(frame_constants[_528[_531_m0].Load(44)].Load4(48)).w)))) / _363, 0.0f);
        float _384 = (_377 / pow(pow(_377 / (_254 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
        float _386 = _320.z;
        float2 _396 = (((_320.xy * float(int(sign(_386)))) / max(0.1500000059604644775390625f, abs(_386)).xx) * 0.0009765625f.xx) * 2.0f;
        float2 _398;
        _398 = 0.0f.xx;
        for (int _401 = 0; _401 < 10; )
        {
            float _405 = float(_401);
            float _407 = (_405 + _298) * 2.3999631404876708984375f;
            float2 _414 = ((float2(cos(_407), sin(_407)) * (_405 + _314)) * 0.100000001490116119384765625f.xx) * _384;
            float3 _420 = float3(_279.x, _279.y, _276.z) + float3(_414 * 0.000833333353511989116668701171875f.xx, _285 * (-0.0009765625f));
            float _432 = 1.0f - shadow_map[_528[_531_m0].Load(32)].SampleCmpLevelZero(sampler_shadow_linear[_528[_531_m0].Load(8)], float3(_420.xy, _259), _420.z + min(0.0f, dot(_396, _414)));
            _398 += float2(_432, _432 * _432);
            _401++;
            continue;
        }
        _436 = _398 * 0.100000001490116119384765625f;
        _437 = _384;
        break;
    } while(false);
    float _438 = _436.x;
    float _441 = clamp((_438 - 0.100000001490116119384765625f) * 1.111111164093017578125f, 0.0f, 1.0f);
    float _443 = _436.y;
    float _448 = _437 * 0.20000000298023223876953125f;
    float2 _454 = _441.xx + ((float2(-1.0f, 1.0f) * sqrt(max(0.0f, _443 - (_438 * _438)))) * (3.0f * max(0.0f, 1.0f - _448)));
    float4 _456 = reprojection_tex[_528[_531_m0].Load(28)].Load(int3(gl_GlobalInvocationID.xy, 0u));
    bool _473;
    if (_456.z > 0.0f)
    {
        bool _472;
        if (_Globals[_528[_531_m0].Load(12)].Load(32) != 0u)
        {
            _472 = !(0u == frame_constants[_528[_531_m0].Load(44)].Load(128));
        }
        else
        {
            _472 = false;
        }
        _473 = _472;
    }
    else
    {
        _473 = false;
    }
    float _482;
    if (_473)
    {
        _482 = history_tex[_528[_531_m0].Load(24)].SampleLevel(sampler_lnc[_528[_531_m0].Load(0)], _196 + _456.xy, 0.0f).x;
    }
    else
    {
        _482 = _441;
    }
    float _485 = clamp(_482, _454.x, _454.y);
    float _486 = lerp(_485, _441, 0.125f);
    if (length(_201 * float2(float(_Globals[_528[_531_m0].Load(12)].Load(8)) / float(_Globals[_528[_531_m0].Load(12)].Load(12)), 1.0f)) < 0.449999988079071044921875f)
    {
        uint _500;
        InterlockedAdd(thread_group_alive, 1u, _500);
        uint _506;
        InterlockedAdd(thread_group_shadow_mask_sum, clamp(uint(clamp(1.0f - _486, 0.0f, 1.0f) * 256.0f), 0u, 255u), _506);
    }
    GroupMemoryBarrierWithGroupSync();
    if (_162)
    {
        if (thread_group_alive > 16u)
        {
            uint _520;
            _alias_shadow_mask_histogram[_528[_531_m0].Load(40)].InterlockedAdd(clamp(uint(float(thread_group_shadow_mask_sum) / float(thread_group_alive)), 0u, 255u) * 4 + 0, 1u, _520);
        }
    }
    output_tex[_528[_531_m0].Load(36)][gl_GlobalInvocationID.xy] = _486 + 0.00196078442968428134918212890625f;
}

[numthreads(16, 16, 1)]
void main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
