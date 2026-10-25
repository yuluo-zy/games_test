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

struct LanternPacked
{
    float3 position;
    uint intensity_color_id;
};

static uint _162;
static float _163;

RWByteAddressBuffer frame_constants[] : register(u2, space0);
ByteAddressBuffer lantern_buf[] : register(t2, space0);
RWByteAddressBuffer _Globals[] : register(u2, space0);
ByteAddressBuffer light_indirection_grid[] : register(t2, space0);
ByteAddressBuffer grid_light_indices[] : register(t2, space0);
RWByteAddressBuffer _957[] : register(u2, space0);
cbuffer _958_960
{
    uint _960_m0 : packoffset(c0);
};

SamplerState sampler_nnc[] : register(s3, space0);
SamplerState sampler_llc[] : register(s3, space0);
Texture2D<float4> blue_noise_texture[] : register(t0, space0);
Texture2D<float4> reprojection_tex[] : register(t0, space0);
Texture2D<float4> half_depth_tex[] : register(t0, space0);
Texture2D<float4> half_view_normal_tex[] : register(t0, space0);
Texture2D<uint4> rdi_temporal_history_tex[] : register(t0, space0);
RWTexture2D<uint2> rdi_temporal_out_tex[] : register(u1, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    do
    {
        uint _994 = _957[_960_m0].Load(24);
        float4 _203 = half_depth_tex[_994].Load(int3(gl_GlobalInvocationID.xy, 0u));
        float _204 = _203.x;
        uint2 _207 = _Globals[_957[_960_m0].Load(12)].Load2(24);
        float2 _208 = float2(_207);
        float2 _211 = (float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx) / _208;
        uint _213 = frame_constants[_957[_960_m0].Load(48)].Load(128);
        uint _214 = _213 + 192192u;
        uint _218 = _214 + (_214 << 10u);
        uint _220 = _218 ^ (_218 >> 6u);
        uint _222 = _220 + (_220 << 3u);
        uint _224 = _222 ^ (_222 >> 11u);
        uint _230 = (((gl_GlobalInvocationID.y * 1664525u) + (_224 + (_224 << 15u))) + 1013904223u) * 1664525u;
        uint _232 = _230 ^ (_230 >> 11u);
        uint _235 = _232 ^ ((_232 << 7u) & 2636928640u);
        uint _238 = _235 ^ ((_235 << 15u) & 4022730752u);
        uint _244 = (((gl_GlobalInvocationID.x * 1664525u) + (_238 ^ (_238 >> 18u))) + 1013904223u) * 1664525u;
        uint _246 = _244 ^ (_244 >> 11u);
        uint _249 = _246 ^ ((_246 << 7u) & 2636928640u);
        uint _252 = _249 ^ ((_249 << 15u) & 4022730752u);
        uint _254 = _252 ^ (_252 >> 18u);
        float4x4 _256 = asfloat(uint4x4(frame_constants[_957[_960_m0].Load(48)].Load(432), frame_constants[_957[_960_m0].Load(48)].Load(448), frame_constants[_957[_960_m0].Load(48)].Load(464), frame_constants[_957[_960_m0].Load(48)].Load(480), frame_constants[_957[_960_m0].Load(48)].Load(436), frame_constants[_957[_960_m0].Load(48)].Load(452), frame_constants[_957[_960_m0].Load(48)].Load(468), frame_constants[_957[_960_m0].Load(48)].Load(484), frame_constants[_957[_960_m0].Load(48)].Load(440), frame_constants[_957[_960_m0].Load(48)].Load(456), frame_constants[_957[_960_m0].Load(48)].Load(472), frame_constants[_957[_960_m0].Load(48)].Load(488), frame_constants[_957[_960_m0].Load(48)].Load(444), frame_constants[_957[_960_m0].Load(48)].Load(460), frame_constants[_957[_960_m0].Load(48)].Load(476), frame_constants[_957[_960_m0].Load(48)].Load(492)));
        float4x4 _258 = asfloat(uint4x4(frame_constants[_957[_960_m0].Load(48)].Load(560), frame_constants[_957[_960_m0].Load(48)].Load(576), frame_constants[_957[_960_m0].Load(48)].Load(592), frame_constants[_957[_960_m0].Load(48)].Load(608), frame_constants[_957[_960_m0].Load(48)].Load(564), frame_constants[_957[_960_m0].Load(48)].Load(580), frame_constants[_957[_960_m0].Load(48)].Load(596), frame_constants[_957[_960_m0].Load(48)].Load(612), frame_constants[_957[_960_m0].Load(48)].Load(568), frame_constants[_957[_960_m0].Load(48)].Load(584), frame_constants[_957[_960_m0].Load(48)].Load(600), frame_constants[_957[_960_m0].Load(48)].Load(616), frame_constants[_957[_960_m0].Load(48)].Load(572), frame_constants[_957[_960_m0].Load(48)].Load(588), frame_constants[_957[_960_m0].Load(48)].Load(604), frame_constants[_957[_960_m0].Load(48)].Load(620)));
        float2 _260 = (_211 - 0.5f.xx) * float2(2.0f, -2.0f);
        float _261 = _260.x;
        float4 _266 = float4(_261, _260.y, _204, 1.0f);
        float4 _268 = mul(_258, mul(_256, _266));
        float4x4 _270 = asfloat(uint4x4(frame_constants[_957[_960_m0].Load(48)].Load(304), frame_constants[_957[_960_m0].Load(48)].Load(320), frame_constants[_957[_960_m0].Load(48)].Load(336), frame_constants[_957[_960_m0].Load(48)].Load(352), frame_constants[_957[_960_m0].Load(48)].Load(308), frame_constants[_957[_960_m0].Load(48)].Load(324), frame_constants[_957[_960_m0].Load(48)].Load(340), frame_constants[_957[_960_m0].Load(48)].Load(356), frame_constants[_957[_960_m0].Load(48)].Load(312), frame_constants[_957[_960_m0].Load(48)].Load(328), frame_constants[_957[_960_m0].Load(48)].Load(344), frame_constants[_957[_960_m0].Load(48)].Load(360), frame_constants[_957[_960_m0].Load(48)].Load(316), frame_constants[_957[_960_m0].Load(48)].Load(332), frame_constants[_957[_960_m0].Load(48)].Load(348), frame_constants[_957[_960_m0].Load(48)].Load(364)));
        if (((-1.0f) / (_204 * (-_270[3].z))) > 2000.0f)
        {
            rdi_temporal_out_tex[_957[_960_m0].Load(36)][gl_GlobalInvocationID.xy] = uint2(0u, 0u);
            break;
        }
        float3 _286 = mul(_258, float4(half_view_normal_tex[_957[_960_m0].Load(28)].Load(int3(gl_GlobalInvocationID.xy, 0u)).xyz, 0.0f)).xyz;
        float3 _290 = _268.xyz / _268.w.xxx;
        float4 _303 = ((blue_noise_texture[_957[_960_m0].Load(16)].Load(int3((gl_GlobalInvocationID.xy + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_213)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
        uint3 _308 = uint3(clamp(floor((_290 * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint _315 = (32u * ((32u * _308.y) + _308.z)) + _308.x;
        uint _318 = light_indirection_grid[_957[_960_m0].Load(40)].Load(_315 * 4 + 0) & 1048575u;
        uint _320 = min(64u, (light_indirection_grid[_957[_960_m0].Load(40)].Load(_315 * 4 + 0) >> 20u));
        uint _322 = _254 + (_254 << 10u);
        uint _324 = _322 ^ (_322 >> 6u);
        uint _326 = _324 + (_324 << 3u);
        uint _328 = _326 ^ (_326 >> 11u);
        uint _332;
        uint _335;
        bool _337;
        bool _339;
        uint _341;
        float _343;
        float _345;
        uint _347;
        float _349;
        _332 = _162;
        _335 = _162;
        _337 = false;
        _339 = false;
        _341 = _328 + (_328 << 15u);
        _343 = 0.0f;
        _345 = 0.0f;
        _347 = 0u;
        _349 = 0.0f;
        uint _333;
        uint _336;
        bool _338;
        bool _340;
        uint _342;
        float _346;
        float _350;
        float _344;
        uint _348;
        for (uint _351 = 0u; _351 < min(2u, _320); _332 = _333, _335 = _336, _337 = _338, _339 = _340, _341 = _342, _343 = _344, _345 = _346, _347 = _348, _349 = _350, _351++)
        {
            uint _359 = _318 + ((_254 + _351) % _320);
            float2 _367 = spvUnpackHalf2x16(lantern_buf[_957[_960_m0].Load(8)].Load(grid_light_indices[_957[_960_m0].Load(44)].Load(_359 * 4 + 0) * 16 + 12) & 65535u);
            float _368 = _367.x;
            uint _370 = lantern_buf[_957[_960_m0].Load(8)].Load(grid_light_indices[_957[_960_m0].Load(44)].Load(_359 * 4 + 0) * 16 + 12) >> 16u;
            float3 _371 = asfloat(lantern_buf[_957[_960_m0].Load(8)].Load3(grid_light_indices[_957[_960_m0].Load(44)].Load(_359 * 4 + 0) * 16 + 0)) - _290;
            float _373 = dot(_371, _371);
            float3 _380 = _371 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
            float3 _425;
            do
            {
                if (_370 == 0u)
                {
                    _425 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                    break;
                }
                else
                {
                    if (_370 == 1u)
                    {
                        _425 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                        break;
                    }
                    else
                    {
                        if (_370 == 2u)
                        {
                            _425 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                            break;
                        }
                        else
                        {
                            if (_370 == 3u)
                            {
                                _425 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                break;
                            }
                            else
                            {
                                _425 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
                                break;
                            }
                            break; // unreachable workaround
                        }
                        break; // unreachable workaround
                    }
                    break; // unreachable workaround
                }
                break; // unreachable workaround
            } while(false);
            float _429 = dot(((_425 * 4.0f) * _368) * ((max(0.0f, dot(_286, _371 / length(_371).xxx)) * ((1.0f / (0.3359999954700469970703125f + _373)) * smoothstep(36.0f * _368, 16.0f * _368, _373))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_380 / length(_380).xxx).y)), float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
            uint _436 = _341 + (_341 << 10u);
            uint _438 = _436 ^ (_436 >> 6u);
            uint _440 = _438 + (_438 << 3u);
            uint _442 = _440 ^ (_440 >> 11u);
            _342 = _442 + (_442 << 15u);
            _340 = _339 ? _339 : true;
            _336 = _339 ? _335 : 8388607u;
            _338 = _337 ? _337 : true;
            _333 = _337 ? _332 : 1065353216u;
            do
            {
                _346 = _345 + 1.0f;
                float _450 = _429 * (float(_320) / float(_Globals[_957[_960_m0].Load(12)].Load(56)));
                bool _458;
                do
                {
                    _350 = _349 + _450;
                    if ((_450 / _350) >= (asfloat((_341 & _336) | _333) - 1.0f))
                    {
                        _348 = grid_light_indices[_957[_960_m0].Load(44)].Load(_359 * 4 + 0);
                        _458 = true;
                        break;
                    }
                    else
                    {
                        _348 = _347;
                        _458 = false;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
                if (_458)
                {
                    _344 = _429;
                    break;
                }
                else
                {
                    _344 = _343;
                    break;
                }
                break; // unreachable workaround
            } while(false);
        }
        bool _724;
        if (_349 > 0.0f)
        {
            bool _722;
            do
            {
                float3 _469 = asfloat(lantern_buf[_957[_960_m0].Load(8)].Load3(_347 * 16 + 0)) - _290;
                float _470 = length(_469);
                if (_470 > 0.5f)
                {
                    float3 _483 = _266.xyz;
                    float4x4 _485 = asfloat(uint4x4(frame_constants[_957[_960_m0].Load(48)].Load(368), frame_constants[_957[_960_m0].Load(48)].Load(384), frame_constants[_957[_960_m0].Load(48)].Load(400), frame_constants[_957[_960_m0].Load(48)].Load(416), frame_constants[_957[_960_m0].Load(48)].Load(372), frame_constants[_957[_960_m0].Load(48)].Load(388), frame_constants[_957[_960_m0].Load(48)].Load(404), frame_constants[_957[_960_m0].Load(48)].Load(420), frame_constants[_957[_960_m0].Load(48)].Load(376), frame_constants[_957[_960_m0].Load(48)].Load(392), frame_constants[_957[_960_m0].Load(48)].Load(408), frame_constants[_957[_960_m0].Load(48)].Load(424), frame_constants[_957[_960_m0].Load(48)].Load(380), frame_constants[_957[_960_m0].Load(48)].Load(396), frame_constants[_957[_960_m0].Load(48)].Load(412), frame_constants[_957[_960_m0].Load(48)].Load(428)));
                    float4x4 _487 = asfloat(uint4x4(frame_constants[_957[_960_m0].Load(48)].Load(496), frame_constants[_957[_960_m0].Load(48)].Load(512), frame_constants[_957[_960_m0].Load(48)].Load(528), frame_constants[_957[_960_m0].Load(48)].Load(544), frame_constants[_957[_960_m0].Load(48)].Load(500), frame_constants[_957[_960_m0].Load(48)].Load(516), frame_constants[_957[_960_m0].Load(48)].Load(532), frame_constants[_957[_960_m0].Load(48)].Load(548), frame_constants[_957[_960_m0].Load(48)].Load(504), frame_constants[_957[_960_m0].Load(48)].Load(520), frame_constants[_957[_960_m0].Load(48)].Load(536), frame_constants[_957[_960_m0].Load(48)].Load(552), frame_constants[_957[_960_m0].Load(48)].Load(508), frame_constants[_957[_960_m0].Load(48)].Load(524), frame_constants[_957[_960_m0].Load(48)].Load(540), frame_constants[_957[_960_m0].Load(48)].Load(556)));
                    float4 _493 = mul(_485, mul(_487, float4((_290 + ((_469 * (_470 - 0.5f)) / _470.xxx)) + (_286 * (0.0199999995529651641845703125f * _303.y)), 1.0f)));
                    float3 _497 = _493.xyz / _493.w.xxx;
                    float4 _508 = _266 + (float4(_497 - _483, 0.0f) * float(int(sign(_497.z))));
                    float _509 = _508.w;
                    float3 _518 = (_508 / (float((_509 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_509))).xxxx).xyz;
                    float3 _519 = _518 - _483;
                    bool3 _520 = bool3(_519.x < 0.0f.xxx.x, _519.y < 0.0f.xxx.y, _519.z < 0.0f.xxx.z);
                    float3 _523 = (float3(_520.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _520.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _520.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _483) / _519;
                    float3 _529 = _483 + (_519 * max(0.0f, max(_523.x, _523.y)));
                    float3 _530 = _518 - _529;
                    bool3 _531 = bool3(_530.x >= 0.0f.xxx.x, _530.y >= 0.0f.xxx.y, _530.z >= 0.0f.xxx.z);
                    float3 _534 = (float3(_531.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _531.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _531.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _529) / _530;
                    float3 _542 = _529 + (_530 * min(1.0f, min(min(_534.x, _534.y), _534.z)));
                    float _543 = _303.x;
                    bool _687;
                    float _688;
                    float _689;
                    do
                    {
                        float _561 = 0.800000011920928955078125f * _270[3].z;
                        uint _562 = uint(int(max(2u, min(4u, uint(int(floor(length((((_542.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_529.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _208))))))));
                        float _667;
                        float _671;
                        float _672;
                        float _673;
                        float _674;
                        bool _675;
                        do
                        {
                            float3 _567 = _542 - _529;
                            float _575;
                            if (_542.z > 9.5367431640625e-07f)
                            {
                                _575 = float(_562);
                            }
                            else
                            {
                                _575 = max(1.0f, float(_562) - 1.0f);
                            }
                            float _576 = 1.0f / _575;
                            float _662;
                            float _663;
                            float _664;
                            float _665;
                            bool _666;
                            if (_562 > 0u)
                            {
                                float _582 = lerp(0.0f, 1.0f, pow(_543 * _576, 1.0f));
                                float3 _584 = _529 + (_567 * _582);
                                float2 _587 = (_584.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                float _589 = 1.0f / _584.z;
                                float _594 = 1.0f / half_depth_tex[_994].SampleLevel(sampler_llc[_957[_960_m0].Load(4)], _587, 0.0f).x;
                                float _599 = 1.0f / half_depth_tex[_994].SampleLevel(sampler_nnc[_957[_960_m0].Load(0)], _587, 0.0f).x;
                                float _603 = (max(_594, _599) * 1.00000202655792236328125f) - _589;
                                float _604 = _589 - min(_594, _599);
                                bool _607 = (_603 < 0.0f) ? (_604 < _561) : false;
                                float _656;
                                float _657;
                                float _658;
                                float _659;
                                bool _660;
                                float _661;
                                if (_607)
                                {
                                    _656 = _603;
                                    _657 = _604;
                                    _658 = _561;
                                    _659 = _582;
                                    _660 = _607;
                                    _661 = 0.0f;
                                }
                                else
                                {
                                    float _617;
                                    _617 = _582;
                                    bool _616;
                                    float _618;
                                    float _651;
                                    float _652;
                                    float _653;
                                    float _654;
                                    bool _655;
                                    uint _612 = 1u;
                                    bool _615 = _607;
                                    for (;;)
                                    {
                                        if (_612 < _562)
                                        {
                                            _618 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_612) + _543) * _576, 1.0f)));
                                            float3 _628 = _529 + (_567 * _618);
                                            float2 _631 = (_628.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                            float _633 = 1.0f / _628.z;
                                            float _637 = 1.0f / half_depth_tex[_994].SampleLevel(sampler_llc[_957[_960_m0].Load(4)], _631, 0.0f).x;
                                            float _641 = 1.0f / half_depth_tex[_994].SampleLevel(sampler_nnc[_957[_960_m0].Load(0)], _631, 0.0f).x;
                                            float _645 = (max(_637, _641) * 1.00000202655792236328125f) - _633;
                                            float _646 = _633 - min(_637, _641);
                                            _616 = (_645 < 0.0f) ? (_646 < _561) : false;
                                            if (_616)
                                            {
                                                _651 = _645;
                                                _652 = _646;
                                                _653 = _561;
                                                _654 = _618;
                                                _655 = _616;
                                                break;
                                            }
                                            _612++;
                                            _615 = _616;
                                            _617 = _618;
                                            continue;
                                        }
                                        else
                                        {
                                            _651 = 0.0f;
                                            _652 = 0.0f;
                                            _653 = 0.0f;
                                            _654 = 1.0f;
                                            _655 = _615;
                                            break;
                                        }
                                    }
                                    _656 = _651;
                                    _657 = _652;
                                    _658 = _653;
                                    _659 = _654;
                                    _660 = _655;
                                    _661 = _617;
                                }
                                _662 = _656;
                                _663 = _657;
                                _664 = _658;
                                _665 = _659;
                                _666 = _660;
                                _667 = _661;
                            }
                            else
                            {
                                _662 = 0.0f;
                                _663 = 0.0f;
                                _664 = 0.0f;
                                _665 = 1.0f;
                                _666 = false;
                                _667 = 0.0f;
                            }
                            if (_666)
                            {
                                _671 = _665;
                                _672 = _662;
                                _673 = _664;
                                _674 = _663;
                                _675 = true;
                                break;
                            }
                            else
                            {
                                _671 = _667;
                                _672 = _163;
                                _673 = _163;
                                _674 = _163;
                                _675 = false;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        bool _683;
                        if (_675)
                        {
                            bool _682;
                            if (_674 < _673)
                            {
                                _682 = _672 < _673;
                            }
                            else
                            {
                                _682 = false;
                            }
                            _683 = _682;
                        }
                        else
                        {
                            _683 = false;
                        }
                        if (_683)
                        {
                            _687 = true;
                            _688 = _671;
                            _689 = _674 / _270[3].z;
                            break;
                        }
                        _687 = false;
                        _688 = _667;
                        _689 = 0.0f;
                        break;
                    } while(false);
                    if (_687)
                    {
                        float4 _699 = mul(_258, mul(_256, float4(lerp(_529, _542, _688.xxx), 1.0f)));
                        float3 _709 = ((_699.xyz / _699.w.xxx) + ((-normalize(mul(_258, mul(_256, float4(_261, _260.y, 0.0f, 1.0f))).xyz)) * _689)) - asfloat(lantern_buf[_957[_960_m0].Load(8)].Load3(_347 * 16 + 0));
                        bool _719;
                        if (!(abs(_709.y) > 0.5f))
                        {
                            float2 _716 = _709.xz;
                            _719 = dot(_716, _716) > 0.136900007724761962890625f;
                        }
                        else
                        {
                            _719 = true;
                        }
                        if (_719)
                        {
                            _722 = false;
                            break;
                        }
                    }
                }
                _722 = true;
                break;
            } while(false);
            _724 = !_722;
        }
        else
        {
            _724 = false;
        }
        uint _725 = _724 ? 0u : _347;
        float _726 = _724 ? 0.0f : _349;
        bool _736;
        if (_Globals[_957[_960_m0].Load(12)].Load(8) != 0u)
        {
            _736 = !(0u == frame_constants[_957[_960_m0].Load(48)].Load(128));
        }
        else
        {
            _736 = false;
        }
        uint _936;
        float _937;
        float _938;
        float _939;
        if (_736)
        {
            uint _751;
            float _753;
            float _755;
            float _757;
            _751 = _725;
            _753 = _726;
            _755 = _343;
            _757 = _345;
            uint _741;
            uint _744;
            bool _746;
            bool _748;
            uint _750;
            float _754;
            float _758;
            uint _752;
            float _756;
            uint _740 = _332;
            uint _743 = _335;
            bool _745 = _337;
            bool _747 = _339;
            uint _749 = _341;
            uint _759 = 0u;
            for (; _759 < 4u; _740 = _741, _743 = _744, _745 = _746, _747 = _748, _749 = _750, _751 = _752, _753 = _754, _755 = _756, _757 = _758, _759++)
            {
                uint _765 = _749 + (_749 << 10u);
                uint _767 = _765 ^ (_765 >> 6u);
                uint _769 = _767 + (_767 << 3u);
                uint _771 = _769 ^ (_769 >> 11u);
                uint _773 = _771 + (_771 << 15u);
                bool _774 = _747 ? _747 : true;
                uint _775 = _747 ? _743 : 8388607u;
                bool _776 = _745 ? _745 : true;
                uint _777 = _745 ? _740 : 1065353216u;
                uint _783 = _773 + (_773 << 10u);
                uint _785 = _783 ^ (_783 >> 6u);
                uint _787 = _785 + (_785 << 3u);
                uint _789 = _787 ^ (_787 >> 11u);
                uint _791 = _789 + (_789 << 15u);
                bool _792 = _774 ? _774 : true;
                uint _793 = _774 ? _775 : 8388607u;
                bool _794 = _776 ? _776 : true;
                uint _795 = _776 ? _777 : 1065353216u;
                bool _801 = 0u == _759;
                int2 _809;
                if (_801)
                {
                    _809 = int2(0, 0);
                }
                else
                {
                    _809 = int2(((float2(asfloat((_749 & _775) | _777) - 1.0f, asfloat((_773 & _793) | _795) - 1.0f) * 2.0f) - 1.0f.xx) * 12.0f);
                }
                uint4 _823 = rdi_temporal_history_tex[_957[_960_m0].Load(32)].Load(int3(uint2(_809) + uint2((_211 + reprojection_tex[_957[_960_m0].Load(20)].Load(int3(gl_GlobalInvocationID.xy * uint2(2u, 2u), 0u)).xy) * float2(_Globals[_957[_960_m0].Load(12)].Load2(40))), 0u));
                uint _824 = _823.x;
                uint _825 = _823.y;
                float2 _834 = max(0.0f.xx, float2(spvUnpackHalf2x16(_825 & 65535u).x, spvUnpackHalf2x16((_825 >> 16u) & 65535u).x));
                float _839 = min(_834.x, float(_801 ? 100 : 5));
                float2 _845 = spvUnpackHalf2x16(lantern_buf[_957[_960_m0].Load(8)].Load(_824 * 16 + 12) & 65535u);
                float _846 = _845.x;
                uint _848 = lantern_buf[_957[_960_m0].Load(8)].Load(_824 * 16 + 12) >> 16u;
                float3 _849 = asfloat(lantern_buf[_957[_960_m0].Load(8)].Load3(_824 * 16 + 0)) - _290;
                float _851 = dot(_849, _849);
                float3 _858 = _849 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                float3 _903;
                do
                {
                    if (_848 == 0u)
                    {
                        _903 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                        break;
                    }
                    else
                    {
                        if (_848 == 1u)
                        {
                            _903 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                            break;
                        }
                        else
                        {
                            if (_848 == 2u)
                            {
                                _903 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                break;
                            }
                            else
                            {
                                if (_848 == 3u)
                                {
                                    _903 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                    break;
                                }
                                else
                                {
                                    _903 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
                                    break;
                                }
                                break; // unreachable workaround
                            }
                            break; // unreachable workaround
                        }
                        break; // unreachable workaround
                    }
                    break; // unreachable workaround
                } while(false);
                float _907 = dot(((_903 * 4.0f) * _846) * ((max(0.0f, dot(_286, _849 / length(_849).xxx)) * ((1.0f / (0.3359999954700469970703125f + _851)) * smoothstep(36.0f * _846, 16.0f * _846, _851))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_858 / length(_858).xxx).y)), float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                uint _909 = _791 + (_791 << 10u);
                uint _911 = _909 ^ (_909 >> 6u);
                uint _913 = _911 + (_911 << 3u);
                uint _915 = _913 ^ (_913 >> 11u);
                _750 = _915 + (_915 << 15u);
                _748 = _792 ? _792 : true;
                _744 = _792 ? _793 : 8388607u;
                _746 = _794 ? _794 : true;
                _741 = _794 ? _795 : 1065353216u;
                do
                {
                    _758 = _757 + _839;
                    float _924 = (_907 * _834.y) * _839;
                    bool _932;
                    do
                    {
                        _754 = _753 + _924;
                        if ((_924 / _754) >= (asfloat((_791 & _744) | _741) - 1.0f))
                        {
                            _752 = _824;
                            _932 = true;
                            break;
                        }
                        else
                        {
                            _752 = _751;
                            _932 = false;
                            break;
                        }
                        break; // unreachable workaround
                    } while(false);
                    if (_932)
                    {
                        _756 = _907;
                        break;
                    }
                    else
                    {
                        _756 = _755;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
            }
            _936 = _751;
            _937 = _753;
            _938 = _755;
            _939 = _757;
        }
        else
        {
            _936 = _725;
            _937 = _726;
            _938 = _343;
            _939 = _345;
        }
        rdi_temporal_out_tex[_957[_960_m0].Load(36)][gl_GlobalInvocationID.xy] = uint2(_936, spvPackHalf2x16(float2(_939, 0.0f)) | (spvPackHalf2x16(float2(min(_937 / max(9.9999999392252902907785028219223e-09f, _939 * _938), 1.0f), 0.0f)) << 16u));
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
