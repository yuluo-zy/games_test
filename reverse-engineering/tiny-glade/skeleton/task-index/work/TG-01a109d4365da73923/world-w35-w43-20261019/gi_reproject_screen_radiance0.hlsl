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

static const float4 _74[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };

RWByteAddressBuffer frame_constants[] : register(u2, space0);
RWByteAddressBuffer _Globals[] : register(u2, space0);
RWByteAddressBuffer _469[] : register(u2, space0);
cbuffer _470_472
{
    uint _472_m0 : packoffset(c0);
};

SamplerState sampler_lnc[] : register(s3, space0);
SamplerState sampler_nnc[] : register(s3, space0);
Texture2D<float4> half_depth_tex[] : register(t0, space0);
Texture2D<float4> half_view_normal_tex[] : register(t0, space0);
Texture2D<float4> reprojection_tex[] : register(t0, space0);
Texture2D<float4> history_tex[] : register(t0, space0);
Texture2D<float4> temporal_stats_tex[] : register(t0, space0);
RWTexture2D<float4> output_tex[] : register(u1, space0);
RWTexture2D<unorm float4> temporal_stats_out_tex[] : register(u1, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        uint _502 = _469[_472_m0].Load(16);
        float4 _131 = half_view_normal_tex[_502].Load(int3(gl_GlobalInvocationID.xy, 0u));
        float3 _132 = _131.xyz;
        float _133 = _131.w;
        float _147 = _131.z;
        float3 _151 = _132 / ((abs(_131.x) + abs(_131.y)) + abs(_147)).xxx;
        bool _153 = _151.z < 0.0f;
        float3 _165;
        if (_153)
        {
            float2 _163 = (1.0f.xx - abs(_151.yx)) * ((step(0.0f.xx, _151.xy) * 2.0f) - 1.0f.xx);
            _165 = float3(_163.x, _163.y, _151.z);
        }
        else
        {
            _165 = _151;
        }
        float2 _168 = (_165.xy * 0.5f) + 0.5f.xx;
        float4 _169 = float4(0.0f.xxxx.x, _168.x, _168.y, 0.0f.xxxx.w);
        bool _179;
        if (_Globals[_469[_472_m0].Load(8)].Load(0) != 0u)
        {
            _179 = !(0u == frame_constants[_469[_472_m0].Load(40)].Load(128));
        }
        else
        {
            _179 = false;
        }
        if (!_179)
        {
            temporal_stats_out_tex[_469[_472_m0].Load(36)][gl_GlobalInvocationID.xy] = _169;
            break;
        }
        uint2 _186 = _Globals[_469[_472_m0].Load(8)].Load2(4);
        float2 _187 = float2(_186);
        float4 _192 = reprojection_tex[_469[_472_m0].Load(20)].Load(int3((gl_GlobalInvocationID.xy * uint2(2u, 2u)) + uint2(uint(abs(_133) > 0.75f), uint(_133 >= 0.0f)), 0u));
        float2 _194 = ((float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx) / _187) + _192.xy;
        float _195 = _192.z;
        float4 _336;
        bool _337;
        if (_195 > 0.0f)
        {
            float2 _223;
            if (_195 != 1.0f)
            {
                uint4 _206 = uint((_195 * 15.0f) + 0.5f).xxxx & uint4(1u, 2u, 4u, 8u);
                float4 _208 = float4(bool4(_206.x != uint4(0u, 0u, 0u, 0u).x, _206.y != uint4(0u, 0u, 0u, 0u).y, _206.z != uint4(0u, 0u, 0u, 0u).z, _206.w != uint4(0u, 0u, 0u, 0u).w));
                _223 = _194 + (((((((-0.5f).xx * _208.x) + (float2(0.5f, -0.5f) * _208.y)) + (float2(-0.5f, 0.5f) * _208.z)) + (0.5f.xx * _208.w)) * 2.0f) / _187);
            }
            else
            {
                _223 = _194;
            }
            uint _225 = _Globals[_469[_472_m0].Load(8)].Load(4);
            float2 _228 = float2(0.5f / float(_225), 0.0f);
            float2 _230 = clamp(_223, _228, 1.0f.xx - _228);
            float4 _123[3] = _74;
            for (uint _232 = 0u; _232 < 3u; )
            {
                _123[_232] = history_tex[_469[_472_m0].Load(24)].SampleLevel(sampler_lnc[_469[_472_m0].Load(0)], (_230 * float2(0.3333333432674407958984375f, 1.0f)) + float2(float(_232) * 0.3333333432674407958984375f, 0.0f), 0.0f);
                _232++;
                continue;
            }
            float3 _263;
            if (_153)
            {
                float2 _261 = (1.0f.xx - abs(_151.yx)) * ((step(0.0f.xx, _151.xy) * 2.0f) - 1.0f.xx);
                _263 = float3(_261.x, _261.y, _151.z);
            }
            else
            {
                _263 = _151;
            }
            float2 _266 = (_263.xy * 0.5f) + 0.5f.xx;
            float4 _273 = abs(_266.xxxx - temporal_stats_tex[_469[_472_m0].Load(28)].GatherGreen(sampler_nnc[_469[_472_m0].Load(4)], _230, int2(0, 0)));
            float4 _283 = abs(_266.yyyy - temporal_stats_tex[_469[_472_m0].Load(28)].GatherBlue(sampler_nnc[_469[_472_m0].Load(4)], _230, int2(0, 0)));
            bool _292 = (min(min(_273.x, _273.y), min(_273.z, _273.w)) + min(min(_283.x, _283.y), min(_283.z, _283.w))) < 0.20000000298023223876953125f;
            float4 _334;
            if (_292)
            {
                float _297 = clamp((temporal_stats_tex[_469[_472_m0].Load(28)].SampleLevel(sampler_nnc[_469[_472_m0].Load(4)], _230, 0.0f) * float4(255.0f, 1.0f, 1.0f, 1.0f)).x + 1.0f, 1.0f, 48.0f);
                float4 _298 = _169;
                _298.x = _297;
                float _299 = 1.0f / _297;
                float4 _124[3];
                for (uint _301 = 0u; _301 < 3u; )
                {
                    float4 _314 = lerp(_123[_301], output_tex[_469[_472_m0].Load(32)][gl_GlobalInvocationID.xy + uint2(_301 * _225, 0u)], _299.xxxx);
                    _314.x = max(0.0f, _314.x);
                    float3 _319 = clamp(_314.yzw, (-1024.0f).xxx, 1024.0f.xxx);
                    _124[_301] = float4(_314.x, _319.x, _319.y, _319.z);
                    _301++;
                    continue;
                }
                for (uint _323 = 0u; _323 < 3u; )
                {
                    output_tex[_469[_472_m0].Load(32)][gl_GlobalInvocationID.xy + uint2(_323 * _225, 0u)] = _124[_323];
                    _323++;
                    continue;
                }
                _334 = _298;
            }
            else
            {
                _334 = _169;
            }
            _336 = _334;
            _337 = _292 ? false : true;
        }
        else
        {
            _336 = _169;
            _337 = true;
        }
        if (_337)
        {
            temporal_stats_out_tex[_469[_472_m0].Load(36)][gl_GlobalInvocationID.xy] = _336;
        }
        bool _346;
        if (_337)
        {
            _346 = _Globals[_469[_472_m0].Load(8)].Load(64) != 0u;
        }
        else
        {
            _346 = false;
        }
        float4 _461;
        if (_346)
        {
            float4 _350 = half_depth_tex[_469[_472_m0].Load(12)].Load(int3(gl_GlobalInvocationID.xy, 0u));
            float _351 = _350.x;
            float _352 = max(0.300000011920928955078125f, _147);
            float4 _125[3] = _74;
            float4 _354;
            float _357;
            float _359;
            _354 = 0.0f.xxxx;
            _357 = 0.0f;
            _359 = 0.0f;
            float4 _355;
            float _358;
            float _360;
            for (int _361 = -2; _361 <= 2; _354 = _355, _357 = _358, _359 = _360, _361++)
            {
                _355 = _354;
                _358 = _357;
                _360 = _359;
                float4 _367;
                float _369;
                float _370;
                for (int _371 = -2; _371 <= 2; _355 = _367, _358 = _369, _360 = _370, _371++)
                {
                    uint2 _379 = gl_GlobalInvocationID.xy + uint2(int2(_371, _361) * int2(2, 2));
                    if (any(bool2(_379.x >= _186.x, _379.y >= _186.y)))
                    {
                        _367 = _355;
                        _369 = _358;
                        _370 = _360;
                        continue;
                    }
                    float _396 = dot(_132, half_view_normal_tex[_502].Load(int3(_379, 0u)).xyz);
                    float _399 = ((_396 * _396) * (-0.89999997615814208984375f)) + 1.0f;
                    float _402 = exp2(-min(30.0f, 80.0f * (_352 * abs(1.0f - (half_depth_tex[_469[_472_m0].Load(12)].Load(int3(_379, 0u)).x / _351))))) * (0.010000000707805156707763671875f / (_399 * _399));
                    float4 _405 = temporal_stats_out_tex[_469[_472_m0].Load(36)][_379] * float4(255.0f, 1.0f, 1.0f, 1.0f);
                    float _407 = max(_405.x, 1.0f);
                    float4 _408 = _405;
                    _408.x = _407;
                    for (uint _412 = 0u; _412 < 3u; )
                    {
                        _125[_412] += ((output_tex[_469[_472_m0].Load(32)][_379 + uint2(_412 * _Globals[_469[_472_m0].Load(8)].Load(4), 0u)] * _402) * _407);
                        _412++;
                        continue;
                    }
                    _367 = _355 + (_408 * _402);
                    _369 = _358 + _402;
                    _370 = _360 + (_402 * _407);
                }
            }
            float _432 = max(9.9999996826552253889678874634872e-21f, _359);
            for (uint _435 = 0u; _435 < 3u; )
            {
                float4 _448 = _125[_435] / _432.xxxx;
                _448.x = max(0.0f, _448.x);
                float3 _453 = clamp(_448.yzw, (-1024.0f).xxx, 1024.0f.xxx);
                output_tex[_469[_472_m0].Load(32)][gl_GlobalInvocationID.xy + uint2(_435 * _Globals[_469[_472_m0].Load(8)].Load(4), 0u)] = float4(_448.x, _453.x, _453.y, _453.z);
                _435++;
                continue;
            }
            float4 _457 = _354 * (1.0f / max(9.9999996826552253889678874634872e-21f, _357));
            _457.x = min(_457.x, 24.0f);
            _461 = _457;
        }
        else
        {
            _461 = _336;
        }
        temporal_stats_out_tex[_469[_472_m0].Load(36)][gl_GlobalInvocationID.xy] = _461 * float4(0.0039215688593685626983642578125f, 1.0f, 1.0f, 1.0f);
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
