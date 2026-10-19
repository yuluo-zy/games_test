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

struct PackedBlBvhNode
{
    uint4 data[5];
};

struct PackedTriangle
{
    float v[3];
    uint e[3];
};

static float _299;
static float3 _300;
static float3 _364;
static float _365;

static const uint2 _310[4] = { uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u) };
static const float3 _327[13] = { float3(0.62999999523162841796875f, 0.37000000476837158203125f, 0.17000000178813934326171875f), float3(0.560000002384185791015625f, 0.37000000476837158203125f, 0.14000000059604644775390625f), float3(0.2700000107288360595703125f, 0.2199999988079071044921875f, 0.12999999523162841796875f), float3(0.37999999523162841796875f, 0.3300000131130218505859375f, 0.20999999344348907470703125f), float3(0.689999997615814208984375f, 0.2199999988079071044921875f, 0.12999999523162841796875f), float3(0.709999978542327880859375f, 0.38999998569488525390625f, 0.100000001490116119384765625f), float3(0.1500000059604644775390625f, 0.36000001430511474609375f, 0.3499999940395355224609375f), float3(0.2599999904632568359375f, 0.2599999904632568359375f, 0.17000000178813934326171875f), 0.4699999988079071044921875f.xxx, float3(0.25f, 0.300000011920928955078125f, 0.20000000298023223876953125f), float3(0.4099999964237213134765625f, 0.4000000059604644775390625f, 0.319999992847442626953125f), float3(0.2849999964237213134765625f, 0.247500002384185791015625f, 0.1574999988079071044921875f), float3(0.4099999964237213134765625f, 0.4000000059604644775390625f, 0.319999992847442626953125f) };

RWByteAddressBuffer frame_constants[] : register(u2, space0);
RWByteAddressBuffer _Globals[] : register(u2, space0);
RWByteAddressBuffer color_library[] : register(u2, space0);
ByteAddressBuffer wall_color_ids[] : register(t2, space0);
ByteAddressBuffer rt_triangle_meta[] : register(t2, space0);
ByteAddressBuffer rt_bl_bvh[] : register(t2, space0);
ByteAddressBuffer rt_triangles[] : register(t2, space0);
RWByteAddressBuffer _2946[] : register(u2, space0);
cbuffer _2947_2949
{
    uint _2949_m0 : packoffset(c0);
};

SamplerState sampler_lnc[] : register(s3, space0);
SamplerState sampler_nnc[] : register(s3, space0);
SamplerState sampler_llc[] : register(s3, space0);
SamplerState sampler_default[] : register(s3, space0);
SamplerState sampler_shadow_nearest[] : register(s3, space0);
SamplerComparisonState sampler_shadow_linear[] : register(s3, space0);
Texture2D<float4> terrain_heightmap[] : register(t0, space0);
Texture2D<float4> visual_terrain_heightmap[] : register(t0, space0);
Texture2D<float4> blue_noise_texture[] : register(t0, space0);
Texture2D<float4> terrain_lighting_texture[] : register(t0, space0);
Texture2DArray<float4> shadow_map[] : register(t0, space0);
Texture2D<float4> taa_history_tex[] : register(t0, space0);
Texture2D<float4> reprojection_tex[] : register(t0, space0);
Texture2D<float4> half_depth_tex[] : register(t0, space0);
Texture2D<float4> half_view_normal_tex[] : register(t0, space0);
RWTexture2D<float4> candidate_radiance_tex[] : register(u1, space0);
RWTexture2D<float4> candidate_hit_tex[] : register(u1, space0);

static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint2 rt_lds_stack[256];

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
        uint2 _374 = gl_GlobalInvocationID.xy * uint2(2u, 2u);
        uint _3005 = _2946[_2949_m0].Load(76);
        float4 _376 = half_depth_tex[_3005].Load(int3(_374, 0u));
        float _377 = _376.x;
        int2 _378 = int2(_374);
        uint2 _380 = _Globals[_2946[_2949_m0].Load(24)].Load2(88);
        float2 _381 = float2(_380);
        float4x4 _386 = asfloat(uint4x4(frame_constants[_2946[_2949_m0].Load(92)].Load(432), frame_constants[_2946[_2949_m0].Load(92)].Load(448), frame_constants[_2946[_2949_m0].Load(92)].Load(464), frame_constants[_2946[_2949_m0].Load(92)].Load(480), frame_constants[_2946[_2949_m0].Load(92)].Load(436), frame_constants[_2946[_2949_m0].Load(92)].Load(452), frame_constants[_2946[_2949_m0].Load(92)].Load(468), frame_constants[_2946[_2949_m0].Load(92)].Load(484), frame_constants[_2946[_2949_m0].Load(92)].Load(440), frame_constants[_2946[_2949_m0].Load(92)].Load(456), frame_constants[_2946[_2949_m0].Load(92)].Load(472), frame_constants[_2946[_2949_m0].Load(92)].Load(488), frame_constants[_2946[_2949_m0].Load(92)].Load(444), frame_constants[_2946[_2949_m0].Load(92)].Load(460), frame_constants[_2946[_2949_m0].Load(92)].Load(476), frame_constants[_2946[_2949_m0].Load(92)].Load(492)));
        float4x4 _388 = asfloat(uint4x4(frame_constants[_2946[_2949_m0].Load(92)].Load(560), frame_constants[_2946[_2949_m0].Load(92)].Load(576), frame_constants[_2946[_2949_m0].Load(92)].Load(592), frame_constants[_2946[_2949_m0].Load(92)].Load(608), frame_constants[_2946[_2949_m0].Load(92)].Load(564), frame_constants[_2946[_2949_m0].Load(92)].Load(580), frame_constants[_2946[_2949_m0].Load(92)].Load(596), frame_constants[_2946[_2949_m0].Load(92)].Load(612), frame_constants[_2946[_2949_m0].Load(92)].Load(568), frame_constants[_2946[_2949_m0].Load(92)].Load(584), frame_constants[_2946[_2949_m0].Load(92)].Load(600), frame_constants[_2946[_2949_m0].Load(92)].Load(616), frame_constants[_2946[_2949_m0].Load(92)].Load(572), frame_constants[_2946[_2949_m0].Load(92)].Load(588), frame_constants[_2946[_2949_m0].Load(92)].Load(604), frame_constants[_2946[_2949_m0].Load(92)].Load(620)));
        float4 _393 = float4((((float2(_378) + 0.5f.xx) / _381) - 0.5f.xx) * float2(2.0f, -2.0f), _377, 1.0f);
        float4 _395 = mul(_388, mul(_386, _393));
        float4x4 _397 = asfloat(uint4x4(frame_constants[_2946[_2949_m0].Load(92)].Load(304), frame_constants[_2946[_2949_m0].Load(92)].Load(320), frame_constants[_2946[_2949_m0].Load(92)].Load(336), frame_constants[_2946[_2949_m0].Load(92)].Load(352), frame_constants[_2946[_2949_m0].Load(92)].Load(308), frame_constants[_2946[_2949_m0].Load(92)].Load(324), frame_constants[_2946[_2949_m0].Load(92)].Load(340), frame_constants[_2946[_2949_m0].Load(92)].Load(356), frame_constants[_2946[_2949_m0].Load(92)].Load(312), frame_constants[_2946[_2949_m0].Load(92)].Load(328), frame_constants[_2946[_2949_m0].Load(92)].Load(344), frame_constants[_2946[_2949_m0].Load(92)].Load(360), frame_constants[_2946[_2949_m0].Load(92)].Load(316), frame_constants[_2946[_2949_m0].Load(92)].Load(332), frame_constants[_2946[_2949_m0].Load(92)].Load(348), frame_constants[_2946[_2949_m0].Load(92)].Load(364)));
        if (((-1.0f) / (_377 * (-_397[3].z))) > 2000.0f)
        {
            uint _406;
            _406 = 0u;
            for (; _406 < 2u; _406++)
            {
                for (uint _413 = 0u; _413 < 2u; )
                {
                    uint2 _419 = _374 + uint2(_413, _406);
                    candidate_radiance_tex[_2946[_2949_m0].Load(84)][_419] = 0.0f.xxxx;
                    candidate_hit_tex[_2946[_2949_m0].Load(88)][_419] = float4(0.0f, 1.0f, 0.0f, 0.0f);
                    _413++;
                    continue;
                }
            }
            break;
        }
        float4 _428 = mul(_388, float4(half_view_normal_tex[_2946[_2949_m0].Load(80)].Load(int3(_374, 0u)).xyz, 0.0f));
        uint _437;
        _437 = 0u;
        uint _438;
        uint _433;
        uint _436;
        for (uint _432 = gl_GlobalInvocationID.y, _435 = gl_GlobalInvocationID.x, _439 = 32u; _439 > 0u; _432 = _433, _435 = _436, _437 = _438, _439 /= 2u)
        {
            uint _446 = uint((_435 & _439) > 0u);
            uint _449 = uint((_432 & _439) > 0u);
            _438 = _437 + ((_439 * _439) * ((3u * _446) ^ _449));
            if (_449 == 0u)
            {
                uint _462;
                uint _463;
                if (_446 == 1u)
                {
                    _462 = 63u - _432;
                    _463 = 63u - _435;
                }
                else
                {
                    _462 = _432;
                    _463 = _435;
                }
                _433 = _463;
                _436 = _462;
            }
            else
            {
                _433 = _432;
                _436 = _435;
            }
        }
        uint _465 = frame_constants[_2946[_2949_m0].Load(92)].Load(128);
        uint _466 = _465 % 64u;
        float2 _472 = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_437 + (288u * _466))) + 0.5f.xx);
        float _473 = _428.z;
        float3 _510;
        float3 _511;
        if (_473 < 0.0f)
        {
            float _495 = 1.0f / (1.0f - _473);
            float _496 = _428.x;
            float _497 = _428.y;
            float _499 = (_496 * _497) * _495;
            _510 = float3(_499, ((_497 * _497) * _495) - 1.0f, -_497);
            _511 = float3(1.0f - ((_496 * _496) * _495), -_499, _496);
        }
        else
        {
            float _479 = 1.0f / (1.0f + _473);
            float _480 = _428.x;
            float _481 = -_480;
            float _482 = _428.y;
            float _484 = (_481 * _482) * _479;
            _510 = float3(_484, 1.0f - ((_482 * _482) * _479), -_482);
            _511 = float3(1.0f - ((_480 * _480) * _479), _484, _481);
        }
        float _525 = 1.0f - _472.x;
        float _527 = _472.y * 6.283185482025146484375f;
        float _530 = sqrt(1.0f - (_525 * _525));
        float3 _536 = mul(float3x3(float3(_511.x, _510.x, _428.x), float3(_511.y, _510.y, _428.y), float3(_511.z, _510.z, _473)), float3(cos(_527) * _530, sin(_527) * _530, _525));
        float4 _2908;
        do
        {
            float _637;
            float _553 = frac(frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(_374.x)) + (0.005837149918079376220703125f * float(_374.y)))) + (float(_465 % 4u) * 0.25f));
            float3 _554 = _393.xyz;
            float3 _558 = _395.xyz / _395.w.xxx;
            float3 _560 = _558 + (_536 * 0.5f);
            float4x4 _562 = asfloat(uint4x4(frame_constants[_2946[_2949_m0].Load(92)].Load(368), frame_constants[_2946[_2949_m0].Load(92)].Load(384), frame_constants[_2946[_2949_m0].Load(92)].Load(400), frame_constants[_2946[_2949_m0].Load(92)].Load(416), frame_constants[_2946[_2949_m0].Load(92)].Load(372), frame_constants[_2946[_2949_m0].Load(92)].Load(388), frame_constants[_2946[_2949_m0].Load(92)].Load(404), frame_constants[_2946[_2949_m0].Load(92)].Load(420), frame_constants[_2946[_2949_m0].Load(92)].Load(376), frame_constants[_2946[_2949_m0].Load(92)].Load(392), frame_constants[_2946[_2949_m0].Load(92)].Load(408), frame_constants[_2946[_2949_m0].Load(92)].Load(424), frame_constants[_2946[_2949_m0].Load(92)].Load(380), frame_constants[_2946[_2949_m0].Load(92)].Load(396), frame_constants[_2946[_2949_m0].Load(92)].Load(412), frame_constants[_2946[_2949_m0].Load(92)].Load(428)));
            float4x4 _564 = asfloat(uint4x4(frame_constants[_2946[_2949_m0].Load(92)].Load(496), frame_constants[_2946[_2949_m0].Load(92)].Load(512), frame_constants[_2946[_2949_m0].Load(92)].Load(528), frame_constants[_2946[_2949_m0].Load(92)].Load(544), frame_constants[_2946[_2949_m0].Load(92)].Load(500), frame_constants[_2946[_2949_m0].Load(92)].Load(516), frame_constants[_2946[_2949_m0].Load(92)].Load(532), frame_constants[_2946[_2949_m0].Load(92)].Load(548), frame_constants[_2946[_2949_m0].Load(92)].Load(504), frame_constants[_2946[_2949_m0].Load(92)].Load(520), frame_constants[_2946[_2949_m0].Load(92)].Load(536), frame_constants[_2946[_2949_m0].Load(92)].Load(552), frame_constants[_2946[_2949_m0].Load(92)].Load(508), frame_constants[_2946[_2949_m0].Load(92)].Load(524), frame_constants[_2946[_2949_m0].Load(92)].Load(540), frame_constants[_2946[_2949_m0].Load(92)].Load(556)));
            float4 _570 = mul(_562, mul(_564, float4(_560, 1.0f)));
            float3 _574 = _570.xyz / _570.w.xxx;
            float4 _585 = _393 + (float4(_574 - _554, 0.0f) * float(int(sign(_574.z))));
            float _586 = _585.w;
            float3 _595 = (_585 / (float((_586 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_586))).xxxx).xyz;
            float3 _596 = _595 - _554;
            bool3 _597 = bool3(_596.x < 0.0f.xxx.x, _596.y < 0.0f.xxx.y, _596.z < 0.0f.xxx.z);
            float3 _600 = (float3(_597.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _597.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _597.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _554) / _596;
            float3 _606 = _554 + (_596 * max(0.0f, max(_600.x, _600.y)));
            float3 _607 = _595 - _606;
            bool3 _608 = bool3(_607.x >= 0.0f.xxx.x, _607.y >= 0.0f.xxx.y, _607.z >= 0.0f.xxx.z);
            float3 _611 = (float3(_608.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _608.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _608.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _606) / _607;
            float3 _619 = _606 + (_607 * min(1.0f, min(min(_611.x, _611.y), _611.z)));
            bool _874;
            float _875;
            float2 _876;
            do
            {
                float2 _624 = (_606.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                float2 _627 = (_619.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                _637 = 0.5f * _397[3].z;
                uint _638 = uint(int(max(2u, min(8u, uint(int(floor(length((_627 - _624) * _381))))))));
                float _758;
                float _855;
                float _856;
                float _857;
                float _858;
                bool _859;
                do
                {
                    float3 _643 = _619 - _606;
                    float _651;
                    if (_619.z > 9.5367431640625e-07f)
                    {
                        _651 = float(_638);
                    }
                    else
                    {
                        _651 = max(1.0f, float(_638) - 1.0f);
                    }
                    float _652 = 1.0f / _651;
                    float _752;
                    float _753;
                    float _754;
                    float _755;
                    float _756;
                    bool _757;
                    if (_638 > 0u)
                    {
                        float _658 = lerp(0.0f, 1.0f, pow(_553 * _652, 1.0f));
                        float3 _660 = _606 + (_643 * _658);
                        float2 _663 = (_660.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                        float _665 = 1.0f / _660.z;
                        float _670 = 1.0f / half_depth_tex[_3005].SampleLevel(sampler_llc[_2946[_2949_m0].Load(8)], _663, 0.0f).x;
                        float4 _673 = half_depth_tex[_3005].SampleLevel(sampler_nnc[_2946[_2949_m0].Load(4)], _663, 0.0f);
                        float _674 = _673.x;
                        float _675 = 1.0f / _674;
                        float _679 = (max(_670, _675) * 1.00000202655792236328125f) - _665;
                        float _687;
                        if ((asuint(_674) & 1u) != 0u)
                        {
                            _687 = _397[3].z * 0.0500000007450580596923828125f;
                        }
                        else
                        {
                            _687 = _637;
                        }
                        bool _689 = (_679 < 0.0f) ? true : false;
                        float _745;
                        float _746;
                        float _747;
                        float _748;
                        float _749;
                        bool _750;
                        float _751;
                        if (_689)
                        {
                            _745 = 0.0f;
                            _746 = _679;
                            _747 = _665 - min(_670, _675);
                            _748 = _687;
                            _749 = _658;
                            _750 = _689;
                            _751 = 0.0f;
                        }
                        else
                        {
                            float _697;
                            float _701;
                            _697 = _679;
                            _701 = _658;
                            float _698;
                            bool _700;
                            float _702;
                            float _740;
                            float _741;
                            float _742;
                            float _743;
                            bool _744;
                            uint _694 = 1u;
                            bool _699 = _689;
                            for (;;)
                            {
                                if (_694 < _638)
                                {
                                    _702 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_694) + _553) * _652, 1.0f)));
                                    float3 _712 = _606 + (_643 * _702);
                                    float2 _715 = (_712.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                    float _717 = 1.0f / _712.z;
                                    float _721 = 1.0f / half_depth_tex[_3005].SampleLevel(sampler_llc[_2946[_2949_m0].Load(8)], _715, 0.0f).x;
                                    float4 _723 = half_depth_tex[_3005].SampleLevel(sampler_nnc[_2946[_2949_m0].Load(4)], _715, 0.0f);
                                    float _724 = _723.x;
                                    float _725 = 1.0f / _724;
                                    _698 = (max(_721, _725) * 1.00000202655792236328125f) - _717;
                                    float _736;
                                    if ((asuint(_724) & 1u) != 0u)
                                    {
                                        _736 = _397[3].z * 0.0500000007450580596923828125f;
                                    }
                                    else
                                    {
                                        _736 = _637;
                                    }
                                    _700 = (_698 < 0.0f) ? true : false;
                                    if (_700)
                                    {
                                        _740 = _698;
                                        _741 = _717 - min(_721, _725);
                                        _742 = _736;
                                        _743 = _702;
                                        _744 = _700;
                                        break;
                                    }
                                    _694++;
                                    _697 = _698;
                                    _699 = _700;
                                    _701 = _702;
                                    continue;
                                }
                                else
                                {
                                    _740 = 0.0f;
                                    _741 = 0.0f;
                                    _742 = 0.0f;
                                    _743 = 1.0f;
                                    _744 = _699;
                                    break;
                                }
                            }
                            _745 = _697;
                            _746 = _740;
                            _747 = _741;
                            _748 = _742;
                            _749 = _743;
                            _750 = _744;
                            _751 = _701;
                        }
                        _752 = _745;
                        _753 = _746;
                        _754 = _747;
                        _755 = _748;
                        _756 = _749;
                        _757 = _750;
                        _758 = _751;
                    }
                    else
                    {
                        _752 = 0.0f;
                        _753 = 0.0f;
                        _754 = 0.0f;
                        _755 = 0.0f;
                        _756 = 1.0f;
                        _757 = false;
                        _758 = 0.0f;
                    }
                    if (_757)
                    {
                        float _763;
                        float _766;
                        float _768;
                        float _770;
                        float _772;
                        float _774;
                        _763 = _758;
                        _766 = _752;
                        _768 = _753;
                        _770 = _754;
                        _772 = _755;
                        _774 = _756;
                        float _764;
                        float _767;
                        float _769;
                        float _771;
                        float _773;
                        float _775;
                        for (uint _776 = 0u; _776 < 2u; _763 = _764, _766 = _767, _768 = _769, _770 = _771, _772 = _773, _774 = _775, _776++)
                        {
                            float _782 = (_763 + _774) * 0.5f;
                            float3 _784 = _606 + (_643 * _782);
                            float2 _787 = (_784.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                            float _789 = 1.0f / _784.z;
                            float _794 = 1.0f / half_depth_tex[_3005].SampleLevel(sampler_llc[_2946[_2949_m0].Load(8)], _787, 0.0f).x;
                            float4 _797 = half_depth_tex[_3005].SampleLevel(sampler_nnc[_2946[_2949_m0].Load(4)], _787, 0.0f);
                            float _798 = _797.x;
                            float _799 = 1.0f / _798;
                            float _803 = (max(_794, _799) * 1.00000202655792236328125f) - _789;
                            float _811;
                            if ((asuint(_798) & 1u) != 0u)
                            {
                                _811 = _397[3].z * 0.0500000007450580596923828125f;
                            }
                            else
                            {
                                _811 = _637;
                            }
                            bool _813 = (_803 < 0.0f) ? true : false;
                            _764 = _813 ? _763 : _782;
                            _767 = _813 ? _766 : _803;
                            _769 = _813 ? _803 : _768;
                            _771 = _813 ? (_789 - min(_794, _799)) : _770;
                            _773 = _813 ? _811 : _772;
                            _775 = _813 ? _782 : _774;
                        }
                        float _817 = lerp(_763, _774, _766 / (_766 + (-_768)));
                        float3 _819 = _606 + (_643 * _817);
                        float2 _822 = (_819.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                        float _824 = 1.0f / _819.z;
                        float _829 = 1.0f / half_depth_tex[_3005].SampleLevel(sampler_llc[_2946[_2949_m0].Load(8)], _822, 0.0f).x;
                        float4 _832 = half_depth_tex[_3005].SampleLevel(sampler_nnc[_2946[_2949_m0].Load(4)], _822, 0.0f);
                        float _833 = _832.x;
                        float _834 = 1.0f / _833;
                        float _838 = (max(_829, _834) * 1.00000202655792236328125f) - _824;
                        float _846;
                        if ((asuint(_833) & 1u) != 0u)
                        {
                            _846 = _397[3].z * 0.0500000007450580596923828125f;
                        }
                        else
                        {
                            _846 = _637;
                        }
                        bool _850 = (abs(_838) < (_766 * 0.89999997615814208984375f)) ? true : false;
                        _855 = _850 ? _817 : _774;
                        _856 = _850 ? _838 : _768;
                        _857 = _850 ? _846 : _772;
                        _858 = _850 ? (_824 - min(_829, _834)) : _770;
                        _859 = true;
                        break;
                    }
                    else
                    {
                        _855 = _758;
                        _856 = _299;
                        _857 = _299;
                        _858 = _299;
                        _859 = false;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
                bool _867;
                if (_859)
                {
                    bool _866;
                    if (_858 < _857)
                    {
                        _866 = _856 < _857;
                    }
                    else
                    {
                        _866 = false;
                    }
                    _867 = _866;
                }
                else
                {
                    _867 = false;
                }
                if (_867)
                {
                    _874 = true;
                    _875 = _855;
                    _876 = lerp(_624, _627, _855.xx);
                    break;
                }
                _874 = false;
                _875 = _758;
                _876 = lerp(_624, _627, _758.xx);
                break;
            } while(false);
            float3 _1157;
            float2 _1158;
            bool _1159;
            float _1160;
            float3 _1161;
            if (!_874)
            {
                float3 _881 = lerp(_606, _619, _875.xxx);
                float4x4 _883 = asfloat(uint4x4(frame_constants[_2946[_2949_m0].Load(92)].Load(240), frame_constants[_2946[_2949_m0].Load(92)].Load(256), frame_constants[_2946[_2949_m0].Load(92)].Load(272), frame_constants[_2946[_2949_m0].Load(92)].Load(288), frame_constants[_2946[_2949_m0].Load(92)].Load(244), frame_constants[_2946[_2949_m0].Load(92)].Load(260), frame_constants[_2946[_2949_m0].Load(92)].Load(276), frame_constants[_2946[_2949_m0].Load(92)].Load(292), frame_constants[_2946[_2949_m0].Load(92)].Load(248), frame_constants[_2946[_2949_m0].Load(92)].Load(264), frame_constants[_2946[_2949_m0].Load(92)].Load(280), frame_constants[_2946[_2949_m0].Load(92)].Load(296), frame_constants[_2946[_2949_m0].Load(92)].Load(252), frame_constants[_2946[_2949_m0].Load(92)].Load(268), frame_constants[_2946[_2949_m0].Load(92)].Load(284), frame_constants[_2946[_2949_m0].Load(92)].Load(300)));
                float4 _894 = float4(_881, 1.0f) + mul(_883, mul(_564, float4(_536, 0.0f)));
                float _895 = _894.w;
                float3 _904 = (_894 / (float((_895 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_895))).xxxx).xyz;
                float3 _905 = _904 - _881;
                bool3 _906 = bool3(_905.x < 0.0f.xxx.x, _905.y < 0.0f.xxx.y, _905.z < 0.0f.xxx.z);
                float3 _909 = (float3(_906.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _906.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _906.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _881) / _905;
                float3 _915 = _881 + (_905 * max(0.0f, max(_909.x, _909.y)));
                float3 _916 = _904 - _915;
                bool3 _917 = bool3(_916.x >= 0.0f.xxx.x, _916.y >= 0.0f.xxx.y, _916.z >= 0.0f.xxx.z);
                float3 _920 = (float3(_917.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _917.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _917.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _915) / _916;
                float3 _927 = _915 + (_916 * min(min(_920.x, _920.y), _920.z));
                bool _1154;
                float _1155;
                float2 _1156;
                do
                {
                    float2 _932 = (_915.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                    float2 _935 = (_927.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                    uint _945 = uint(int(max(2u, min(8u, uint(int(floor(length((_935 - _932) * _381))))))));
                    float _1052;
                    float _1135;
                    float _1136;
                    float _1137;
                    float _1138;
                    bool _1139;
                    do
                    {
                        float3 _950 = _927 - _915;
                        float _958;
                        if (_927.z > 9.5367431640625e-07f)
                        {
                            _958 = float(_945);
                        }
                        else
                        {
                            _958 = max(1.0f, float(_945) - 1.0f);
                        }
                        float _959 = 1.0f / _958;
                        float _1046;
                        float _1047;
                        float _1048;
                        float _1049;
                        float _1050;
                        bool _1051;
                        if (_945 > 0u)
                        {
                            float _965 = lerp(0.0f, 1.0f, pow(_553 * _959, 2.0f));
                            float3 _967 = _915 + (_950 * _965);
                            float _972 = 1.0f / _967.z;
                            float4 _975 = half_depth_tex[_3005].SampleLevel(sampler_nnc[_2946[_2949_m0].Load(4)], (_967.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                            float _976 = _975.x;
                            float _979 = (1.00000202655792236328125f / _976) - _972;
                            float _987;
                            if ((asuint(_976) & 1u) != 0u)
                            {
                                _987 = _397[3].z * 0.0500000007450580596923828125f;
                            }
                            else
                            {
                                _987 = _637;
                            }
                            bool _989 = (_979 < 0.0f) ? true : false;
                            float _1039;
                            float _1040;
                            float _1041;
                            float _1042;
                            float _1043;
                            bool _1044;
                            float _1045;
                            if (_989)
                            {
                                _1039 = 0.0f;
                                _1040 = _979;
                                _1041 = _972 - (1.0f / _976);
                                _1042 = _987;
                                _1043 = _965;
                                _1044 = _989;
                                _1045 = 0.0f;
                            }
                            else
                            {
                                float _997;
                                float _1001;
                                _997 = _979;
                                _1001 = _965;
                                float _998;
                                bool _1000;
                                float _1002;
                                float _1034;
                                float _1035;
                                float _1036;
                                float _1037;
                                bool _1038;
                                uint _994 = 1u;
                                bool _999 = _989;
                                for (;;)
                                {
                                    if (_994 < _945)
                                    {
                                        _1002 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_994) + _553) * _959, 2.0f)));
                                        float3 _1012 = _915 + (_950 * _1002);
                                        float _1017 = 1.0f / _1012.z;
                                        float4 _1019 = half_depth_tex[_3005].SampleLevel(sampler_nnc[_2946[_2949_m0].Load(4)], (_1012.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                                        float _1020 = _1019.x;
                                        _998 = (1.00000202655792236328125f / _1020) - _1017;
                                        float _1030;
                                        if ((asuint(_1020) & 1u) != 0u)
                                        {
                                            _1030 = _397[3].z * 0.0500000007450580596923828125f;
                                        }
                                        else
                                        {
                                            _1030 = _637;
                                        }
                                        _1000 = (_998 < 0.0f) ? true : false;
                                        if (_1000)
                                        {
                                            _1034 = _998;
                                            _1035 = _1017 - (1.0f / _1020);
                                            _1036 = _1030;
                                            _1037 = _1002;
                                            _1038 = _1000;
                                            break;
                                        }
                                        _994++;
                                        _997 = _998;
                                        _999 = _1000;
                                        _1001 = _1002;
                                        continue;
                                    }
                                    else
                                    {
                                        _1034 = 0.0f;
                                        _1035 = 0.0f;
                                        _1036 = 0.0f;
                                        _1037 = 1.0f;
                                        _1038 = _999;
                                        break;
                                    }
                                }
                                _1039 = _997;
                                _1040 = _1034;
                                _1041 = _1035;
                                _1042 = _1036;
                                _1043 = _1037;
                                _1044 = _1038;
                                _1045 = _1001;
                            }
                            _1046 = _1039;
                            _1047 = _1040;
                            _1048 = _1041;
                            _1049 = _1042;
                            _1050 = _1043;
                            _1051 = _1044;
                            _1052 = _1045;
                        }
                        else
                        {
                            _1046 = 0.0f;
                            _1047 = 0.0f;
                            _1048 = 0.0f;
                            _1049 = 0.0f;
                            _1050 = 1.0f;
                            _1051 = false;
                            _1052 = 0.0f;
                        }
                        if (_1051)
                        {
                            float _1057;
                            float _1060;
                            float _1062;
                            float _1064;
                            float _1066;
                            float _1068;
                            _1057 = _1052;
                            _1060 = _1046;
                            _1062 = _1047;
                            _1064 = _1048;
                            _1066 = _1049;
                            _1068 = _1050;
                            float _1058;
                            float _1061;
                            float _1063;
                            float _1065;
                            float _1067;
                            float _1069;
                            for (uint _1070 = 0u; _1070 < 4u; _1057 = _1058, _1060 = _1061, _1062 = _1063, _1064 = _1065, _1066 = _1067, _1068 = _1069, _1070++)
                            {
                                float _1076 = (_1057 + _1068) * 0.5f;
                                float3 _1078 = _915 + (_950 * _1076);
                                float _1083 = 1.0f / _1078.z;
                                float4 _1086 = half_depth_tex[_3005].SampleLevel(sampler_nnc[_2946[_2949_m0].Load(4)], (_1078.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                                float _1087 = _1086.x;
                                float _1090 = (1.00000202655792236328125f / _1087) - _1083;
                                float _1098;
                                if ((asuint(_1087) & 1u) != 0u)
                                {
                                    _1098 = _397[3].z * 0.0500000007450580596923828125f;
                                }
                                else
                                {
                                    _1098 = _637;
                                }
                                bool _1100 = (_1090 < 0.0f) ? true : false;
                                _1058 = _1100 ? _1057 : _1076;
                                _1061 = _1100 ? _1060 : _1090;
                                _1063 = _1100 ? _1090 : _1062;
                                _1065 = _1100 ? (_1083 - (1.0f / _1087)) : _1064;
                                _1067 = _1100 ? _1098 : _1066;
                                _1069 = _1100 ? _1076 : _1068;
                            }
                            float _1104 = lerp(_1057, _1068, _1060 / (_1060 + (-_1062)));
                            float3 _1106 = _915 + (_950 * _1104);
                            float _1111 = 1.0f / _1106.z;
                            float4 _1114 = half_depth_tex[_3005].SampleLevel(sampler_nnc[_2946[_2949_m0].Load(4)], (_1106.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                            float _1115 = _1114.x;
                            float _1118 = (1.00000202655792236328125f / _1115) - _1111;
                            float _1126;
                            if ((asuint(_1115) & 1u) != 0u)
                            {
                                _1126 = _397[3].z * 0.0500000007450580596923828125f;
                            }
                            else
                            {
                                _1126 = _637;
                            }
                            bool _1130 = (abs(_1118) < (_1060 * 0.89999997615814208984375f)) ? true : false;
                            _1135 = _1130 ? _1104 : _1068;
                            _1136 = _1130 ? _1118 : _1062;
                            _1137 = _1130 ? _1126 : _1066;
                            _1138 = _1130 ? (_1111 - (1.0f / _1115)) : _1064;
                            _1139 = true;
                            break;
                        }
                        else
                        {
                            _1135 = _1052;
                            _1136 = _299;
                            _1137 = _299;
                            _1138 = _299;
                            _1139 = false;
                            break;
                        }
                        break; // unreachable workaround
                    } while(false);
                    bool _1147;
                    if (_1139)
                    {
                        bool _1146;
                        if (_1138 < _1137)
                        {
                            _1146 = _1136 < _1137;
                        }
                        else
                        {
                            _1146 = false;
                        }
                        _1147 = _1146;
                    }
                    else
                    {
                        _1147 = false;
                    }
                    if (_1147)
                    {
                        _1154 = true;
                        _1155 = _1135;
                        _1156 = lerp(_932, _935, _1135.xx);
                        break;
                    }
                    _1154 = false;
                    _1155 = _1052;
                    _1156 = lerp(_932, _935, _1052.xx);
                    break;
                } while(false);
                _1157 = _927;
                _1158 = _1156;
                _1159 = _1154;
                _1160 = _1155;
                _1161 = _915;
            }
            else
            {
                _1157 = _619;
                _1158 = _876;
                _1159 = _874;
                _1160 = _875;
                _1161 = _606;
            }
            float4 _1169 = mul(_388, mul(_386, float4(lerp(_1161, _1157, _1160.xxx), 1.0f)));
            float _1175 = length((_1169.xyz / _1169.w.xxx) - _558);
            bool _1185;
            if (_Globals[_2946[_2949_m0].Load(24)].Load(48) != 0u)
            {
                _1185 = !(0u == frame_constants[_2946[_2949_m0].Load(92)].Load(128));
            }
            else
            {
                _1185 = false;
            }
            if (_1185 ? _1159 : false)
            {
                float4 _1192 = reprojection_tex[_2946[_2949_m0].Load(72)].SampleLevel(sampler_nnc[_2946[_2949_m0].Load(4)], _1158, 0.0f);
                if (_1192.z > 0.0f)
                {
                    _2908 = float4(taa_history_tex[_2946[_2949_m0].Load(68)].SampleLevel(sampler_nnc[_2946[_2949_m0].Load(4)], _1158 + _1192.xy, 0.0f).xyz, _1175);
                    break;
                }
            }
            float4 _1210;
            float3 _1213;
            float3 _1215;
            float3 _1217;
            float3 _1219;
            _1210 = float4(0.0f, 0.0f, 0.0f, 100000.0f);
            _1213 = _536;
            _1215 = 1.0f.xxx;
            _1217 = _300;
            _1219 = _558 + (_536 * max(0.100000001490116119384765625f, _1175));
            float4 _1211;
            float3 _1214;
            float3 _1216;
            float3 _1218;
            float3 _1220;
            float4 _2905;
            uint _1221 = 0u;
            for (;;)
            {
                if (_1221 < 1u)
                {
                    float _1247;
                    float4 _1241 = ((blue_noise_texture[_2946[_2949_m0].Load(36)].Load(int3((_374 + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float((_465 + 1234567u) + (_1221 * 137u))) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
                    float3 _1903;
                    uint _1904;
                    uint _1905;
                    float _1906;
                    do
                    {
                        uint2 _367[4] = _310;
                        bool _1245 = _1213.x < 0.0f;
                        _1247 = _1213.y;
                        bool _1248 = _1247 < 0.0f;
                        bool _1252 = _1213.z < 0.0f;
                        uint _1257 = (7u - uint(((_1245 ? 4 : 0) | (_1248 ? 2 : 0)) | int(_1252))) * 16843009u;
                        uint2 _1259;
                        float3 _1268;
                        _1259 = uint2(0u, 2147483648u);
                        _1268 = _1217;
                        uint2 _1260;
                        float _1263;
                        int _1265;
                        float3 _1269;
                        int _1271;
                        float _1893;
                        int _1894;
                        float _1262 = 3.4028234663852885981170418348452e+38f;
                        int _1264 = 0;
                        uint _1266 = 0u;
                        int _1270 = -1;
                        [loop]
                        for (;;)
                        {
                            if (_1266 < 1024u)
                            {
                                uint2 _1777;
                                uint2 _1778;
                                int _1779;
                                if ((_1259.y & 4278190080u) != 0u)
                                {
                                    uint _1281 = firstbithigh(_1259.y);
                                    uint _1286 = _1259.y & (~(1u << (_1281 & 31u)));
                                    uint2 _1287 = _1259;
                                    _1287.y = _1286;
                                    int _1306;
                                    if ((_1286 & 4278190080u) != 0u)
                                    {
                                        int _1305;
                                        do
                                        {
                                            if (_1264 >= 4)
                                            {
                                                rt_lds_stack[gl_LocalInvocationIndex + uint((_1264 - 4) * 64)] = _1287;
                                                _1305 = _1264 + 1;
                                                break;
                                            }
                                            _367[_1264] = _1287;
                                            _1305 = _1264 + 1;
                                            break;
                                        } while(false);
                                        _1306 = _1305;
                                    }
                                    else
                                    {
                                        _1306 = _1264;
                                    }
                                    PackedBlBvhNode _1317;
                                    [unroll]
                                    for (int _6ident = 0; _6ident < 5; _6ident++)
                                    {
                                        _1317.data[_6ident] = rt_bl_bvh[_2946[_2949_m0].Load(60)].Load4(_6ident * 16 + (_1259.x + countbits(_1259.y & (~(4294967295u << (((_1281 - 24u) ^ (_1257 & 255u)) & 31u))))) * 80 + 0);
                                    }
                                    float3 _1342 = float3(asfloat(((_1317.data[0].w >> 0u) & 255u) << 23u) / _1213.x, asfloat(((_1317.data[0].w >> 8u) & 255u) << 23u) / _1247, asfloat(((_1317.data[0].w >> 16u) & 255u) << 23u) / _1213.z);
                                    float3 _1344 = (asfloat(_1317.data[0].xyz) - _1219) / _1213;
                                    uint _1353 = (_1317.data[1].z ^ (_1257 & ((((_1317.data[1].z & (_1317.data[1].z << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
                                    uint _1355 = (_1317.data[1].z >> 5u) & 117901063u;
                                    uint _1362 = _1245 ? _1317.data[2].z : _1317.data[2].x;
                                    uint _1363 = _1245 ? _1317.data[2].x : _1317.data[2].z;
                                    uint _1364 = _1248 ? _1317.data[3].z : _1317.data[3].x;
                                    uint _1365 = _1248 ? _1317.data[3].x : _1317.data[3].z;
                                    uint _1366 = _1252 ? _1317.data[4].z : _1317.data[4].x;
                                    uint _1367 = _1252 ? _1317.data[4].x : _1317.data[4].z;
                                    float3 _1389 = (float3(float((_1362 >> 0u) & 255u), float((_1364 >> 0u) & 255u), float((_1366 >> 0u) & 255u)) * _1342) + _1344;
                                    float3 _1391 = (float3(float((_1363 >> 0u) & 255u), float((_1365 >> 0u) & 255u), float((_1367 >> 0u) & 255u)) * _1342) + _1344;
                                    uint _1414;
                                    if (max(_1389.x, max(_1389.y, max(_1389.z, 9.9999997473787516355514526367188e-05f))) <= min(_1391.x, min(_1391.y, min(_1391.z, _1262))))
                                    {
                                        _1414 = 0u | (((_1355 >> 0u) & 255u) << (((_1353 >> 0u) & 255u) & 31u));
                                    }
                                    else
                                    {
                                        _1414 = 0u;
                                    }
                                    float3 _1436 = (float3(float((_1362 >> 8u) & 255u), float((_1364 >> 8u) & 255u), float((_1366 >> 8u) & 255u)) * _1342) + _1344;
                                    float3 _1438 = (float3(float((_1363 >> 8u) & 255u), float((_1365 >> 8u) & 255u), float((_1367 >> 8u) & 255u)) * _1342) + _1344;
                                    uint _1461;
                                    if (max(_1436.x, max(_1436.y, max(_1436.z, 9.9999997473787516355514526367188e-05f))) <= min(_1438.x, min(_1438.y, min(_1438.z, _1262))))
                                    {
                                        _1461 = _1414 | (((_1355 >> 8u) & 255u) << (((_1353 >> 8u) & 255u) & 31u));
                                    }
                                    else
                                    {
                                        _1461 = _1414;
                                    }
                                    float3 _1483 = (float3(float((_1362 >> 16u) & 255u), float((_1364 >> 16u) & 255u), float((_1366 >> 16u) & 255u)) * _1342) + _1344;
                                    float3 _1485 = (float3(float((_1363 >> 16u) & 255u), float((_1365 >> 16u) & 255u), float((_1367 >> 16u) & 255u)) * _1342) + _1344;
                                    uint _1508;
                                    if (max(_1483.x, max(_1483.y, max(_1483.z, 9.9999997473787516355514526367188e-05f))) <= min(_1485.x, min(_1485.y, min(_1485.z, _1262))))
                                    {
                                        _1508 = _1461 | (((_1355 >> 16u) & 255u) << (((_1353 >> 16u) & 255u) & 31u));
                                    }
                                    else
                                    {
                                        _1508 = _1461;
                                    }
                                    float3 _1530 = (float3(float((_1362 >> 24u) & 255u), float((_1364 >> 24u) & 255u), float((_1366 >> 24u) & 255u)) * _1342) + _1344;
                                    float3 _1532 = (float3(float((_1363 >> 24u) & 255u), float((_1365 >> 24u) & 255u), float((_1367 >> 24u) & 255u)) * _1342) + _1344;
                                    uint _1555;
                                    if (max(_1530.x, max(_1530.y, max(_1530.z, 9.9999997473787516355514526367188e-05f))) <= min(_1532.x, min(_1532.y, min(_1532.z, _1262))))
                                    {
                                        _1555 = _1508 | (((_1355 >> 24u) & 255u) << (((_1353 >> 24u) & 255u) & 31u));
                                    }
                                    else
                                    {
                                        _1555 = _1508;
                                    }
                                    uint _1564 = (_1317.data[1].w ^ (_1257 & ((((_1317.data[1].w & (_1317.data[1].w << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
                                    uint _1566 = (_1317.data[1].w >> 5u) & 117901063u;
                                    uint _1573 = _1245 ? _1317.data[2].w : _1317.data[2].y;
                                    uint _1574 = _1245 ? _1317.data[2].y : _1317.data[2].w;
                                    uint _1575 = _1248 ? _1317.data[3].w : _1317.data[3].y;
                                    uint _1576 = _1248 ? _1317.data[3].y : _1317.data[3].w;
                                    uint _1577 = _1252 ? _1317.data[4].w : _1317.data[4].y;
                                    uint _1578 = _1252 ? _1317.data[4].y : _1317.data[4].w;
                                    float3 _1600 = (float3(float((_1573 >> 0u) & 255u), float((_1575 >> 0u) & 255u), float((_1577 >> 0u) & 255u)) * _1342) + _1344;
                                    float3 _1602 = (float3(float((_1574 >> 0u) & 255u), float((_1576 >> 0u) & 255u), float((_1578 >> 0u) & 255u)) * _1342) + _1344;
                                    uint _1625;
                                    if (max(_1600.x, max(_1600.y, max(_1600.z, 9.9999997473787516355514526367188e-05f))) <= min(_1602.x, min(_1602.y, min(_1602.z, _1262))))
                                    {
                                        _1625 = _1555 | (((_1566 >> 0u) & 255u) << (((_1564 >> 0u) & 255u) & 31u));
                                    }
                                    else
                                    {
                                        _1625 = _1555;
                                    }
                                    float3 _1647 = (float3(float((_1573 >> 8u) & 255u), float((_1575 >> 8u) & 255u), float((_1577 >> 8u) & 255u)) * _1342) + _1344;
                                    float3 _1649 = (float3(float((_1574 >> 8u) & 255u), float((_1576 >> 8u) & 255u), float((_1578 >> 8u) & 255u)) * _1342) + _1344;
                                    uint _1672;
                                    if (max(_1647.x, max(_1647.y, max(_1647.z, 9.9999997473787516355514526367188e-05f))) <= min(_1649.x, min(_1649.y, min(_1649.z, _1262))))
                                    {
                                        _1672 = _1625 | (((_1566 >> 8u) & 255u) << (((_1564 >> 8u) & 255u) & 31u));
                                    }
                                    else
                                    {
                                        _1672 = _1625;
                                    }
                                    float3 _1694 = (float3(float((_1573 >> 16u) & 255u), float((_1575 >> 16u) & 255u), float((_1577 >> 16u) & 255u)) * _1342) + _1344;
                                    float3 _1696 = (float3(float((_1574 >> 16u) & 255u), float((_1576 >> 16u) & 255u), float((_1578 >> 16u) & 255u)) * _1342) + _1344;
                                    uint _1719;
                                    if (max(_1694.x, max(_1694.y, max(_1694.z, 9.9999997473787516355514526367188e-05f))) <= min(_1696.x, min(_1696.y, min(_1696.z, _1262))))
                                    {
                                        _1719 = _1672 | (((_1566 >> 16u) & 255u) << (((_1564 >> 16u) & 255u) & 31u));
                                    }
                                    else
                                    {
                                        _1719 = _1672;
                                    }
                                    float3 _1741 = (float3(float((_1573 >> 24u) & 255u), float((_1575 >> 24u) & 255u), float((_1577 >> 24u) & 255u)) * _1342) + _1344;
                                    float3 _1743 = (float3(float((_1574 >> 24u) & 255u), float((_1576 >> 24u) & 255u), float((_1578 >> 24u) & 255u)) * _1342) + _1344;
                                    uint _1766;
                                    if (max(_1741.x, max(_1741.y, max(_1741.z, 9.9999997473787516355514526367188e-05f))) <= min(_1743.x, min(_1743.y, min(_1743.z, _1262))))
                                    {
                                        _1766 = _1719 | (((_1566 >> 24u) & 255u) << (((_1564 >> 24u) & 255u) & 31u));
                                    }
                                    else
                                    {
                                        _1766 = _1719;
                                    }
                                    uint2 _1770 = uint2(_1317.data[1].x, _1286);
                                    _1770.y = (_1766 & 4278190080u) | ((_1317.data[0].w >> 24u) & 255u);
                                    _1777 = _1770;
                                    _1778 = uint2(_1317.data[1].y, _1766 & 16777215u);
                                    _1779 = _1306;
                                }
                                else
                                {
                                    _1777 = uint2(0u, 0u);
                                    _1778 = _1259;
                                    _1779 = _1264;
                                }
                                uint2 _1781;
                                _1781 = _1778;
                                _1269 = _1268;
                                _1263 = _1262;
                                _1271 = _1270;
                                uint2 _1782;
                                float3 _1784;
                                float _1785;
                                int _1786;
                                uint _1787;
                                for (;;)
                                {
                                    _1787 = _1781.y;
                                    if (_1787 != 0u)
                                    {
                                        uint _1791 = firstbithigh(_1787);
                                        _1782 = _1781;
                                        _1782.y = _1787 & (~(1u << (_1791 & 31u)));
                                        uint _1797 = _1781.x + _1791;
                                        PackedTriangle _1799;
                                        [unroll]
                                        for (int _7ident = 0; _7ident < 3; _7ident++)
                                        {
                                            _1799.v[_7ident] = asfloat(rt_triangles[_2946[_2949_m0].Load(64)].Load(_7ident * 4 + _1797 * 24 + 0));
                                        }
                                        [unroll]
                                        for (int _8ident = 0; _8ident < 3; _8ident++)
                                        {
                                            _1799.e[_8ident] = rt_triangles[_2946[_2949_m0].Load(64)].Load(_8ident * 4 + _1797 * 24 + 12);
                                        }
                                        float3 _1830 = float3(spvUnpackHalf2x16((_1799.e[0] >> 16u) & 65535u).x, spvUnpackHalf2x16((_1799.e[1] >> 16u) & 65535u).x, spvUnpackHalf2x16((_1799.e[2] >> 16u) & 65535u).x);
                                        float3 _1831 = float3(spvUnpackHalf2x16(_1799.e[0] & 65535u).x, spvUnpackHalf2x16(_1799.e[1] & 65535u).x, spvUnpackHalf2x16(_1799.e[2] & 65535u).x);
                                        bool _1865;
                                        do
                                        {
                                            float3 _1834 = cross(_1213, _1831);
                                            float _1835 = dot(_1830, _1834);
                                            float3 _1836 = _1219 - float3(_1799.v[0], _1799.v[1], _1799.v[2]);
                                            float3 _1837 = cross(_1836, _1830);
                                            float _1839 = dot(_1831, _1837) / _1835;
                                            bool _1844;
                                            if (_1839 >= 0.0f)
                                            {
                                                _1844 = _1839 < _1263;
                                            }
                                            else
                                            {
                                                _1844 = false;
                                            }
                                            if (_1844 ? true : false)
                                            {
                                                float3 _1849;
                                                _1849.x = dot(_1836, _1834);
                                                _1849.y = dot(_1213, _1837);
                                                float2 _1854 = _1849.xy / _1835.xx;
                                                float3 _1855 = float3(_1854.x, _1854.y, _1849.z);
                                                _1855.z = (1.0f - _1854.x) - _1854.y;
                                                if (all(bool3(_1855.x >= (-9.9999997473787516355514526367188e-05f).xxx.x, _1855.y >= (-9.9999997473787516355514526367188e-05f).xxx.y, _1855.z >= (-9.9999997473787516355514526367188e-05f).xxx.z)))
                                                {
                                                    _1785 = _1839;
                                                    _1865 = true;
                                                    break;
                                                }
                                            }
                                            _1785 = _1263;
                                            _1865 = false;
                                            break;
                                        } while(false);
                                        if (_1865)
                                        {
                                            _1784 = cross(_1830, _1831);
                                            _1786 = int(_1797);
                                        }
                                        else
                                        {
                                            _1784 = _1269;
                                            _1786 = _1271;
                                        }
                                        _1781 = _1782;
                                        _1269 = _1784;
                                        _1263 = _1785;
                                        _1271 = _1786;
                                        continue;
                                    }
                                    else
                                    {
                                        break;
                                    }
                                }
                                if ((_1777.y & 4278190080u) == 0u)
                                {
                                    if (_1779 == 0)
                                    {
                                        _1218 = _1269;
                                        _1893 = _1263;
                                        _1894 = _1271;
                                        break;
                                    }
                                    int _1880;
                                    uint2 _1892;
                                    do
                                    {
                                        _1880 = _1779 - 1;
                                        if (_1880 >= 4)
                                        {
                                            _1892 = rt_lds_stack[gl_LocalInvocationIndex + uint((_1779 - 5) * 64)];
                                            break;
                                        }
                                        _1892 = _367[_1880];
                                        break;
                                    } while(false);
                                    _1260 = _1892;
                                    _1265 = _1880;
                                }
                                else
                                {
                                    _1260 = _1777;
                                    _1265 = _1779;
                                }
                                _1259 = _1260;
                                _1262 = _1263;
                                _1264 = _1265;
                                _1266++;
                                _1268 = _1269;
                                _1270 = _1271;
                                continue;
                            }
                            else
                            {
                                _1218 = _1268;
                                _1893 = _1262;
                                _1894 = _1270;
                                break;
                            }
                        }
                        bool _1899;
                        if (_1894 != (-1))
                        {
                            _1899 = _1893 < 100000.0f;
                        }
                        else
                        {
                            _1899 = false;
                        }
                        if (_1899)
                        {
                            _1903 = _1218;
                            _1904 = 3u;
                            _1905 = uint(_1894);
                            _1906 = _1893;
                            break;
                        }
                        _1903 = float3(0.0f, 1.0f, 0.0f);
                        _1904 = 0u;
                        _1905 = 0u;
                        _1906 = 100000.0f;
                        break;
                    } while(false);
                    uint _2021;
                    float _2022;
                    do
                    {
                        float3 _1910 = (float3(65.0f, 24.0f, 65.0f) - _1219) / _1213;
                        float3 _1912 = (float3(-65.0f, 0.0f, -65.0f) - _1219) / _1213;
                        float3 _1913 = min(_1910, _1912);
                        float3 _1914 = max(_1910, _1912);
                        float _1919 = max(max(_1913.x, _1913.y), _1913.z);
                        float _1924 = min(min(_1914.x, _1914.y), _1914.z);
                        bool _1929;
                        if (_1919 <= _1924)
                        {
                            _1929 = _1924 >= 0.0f;
                        }
                        else
                        {
                            _1929 = false;
                        }
                        if (!_1929)
                        {
                            _2021 = _1904;
                            _2022 = _1906;
                            break;
                        }
                        float _1933 = max(_1919, 0.0f);
                        float _1935 = (_1924 - _1933) * 0.03125f;
                        float _1937;
                        _1937 = _1933;
                        float _1938;
                        uint _1941;
                        float _1960;
                        uint _1940 = 0u;
                        bool _1942;
                        for (;;)
                        {
                            _1942 = _1940 < 32u;
                            if (_1942)
                            {
                                _1938 = _1937 + _1935;
                                float3 _1946 = _1219 + (_1213 * _1938);
                                if (_1946.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap[_2946[_2949_m0].Load(32)].SampleLevel(sampler_lnc[_2946[_2949_m0].Load(0)], (_1946.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x))
                                {
                                    _1960 = _1938;
                                    break;
                                }
                                _1941 = _1940 + 1u;
                                _1937 = _1938;
                                _1940 = _1941;
                                continue;
                            }
                            else
                            {
                                _1960 = _1924;
                                break;
                            }
                        }
                        bool _1964;
                        if (_1942)
                        {
                            _1964 = _1937 < _1906;
                        }
                        else
                        {
                            _1964 = false;
                        }
                        bool _1968;
                        if (_1964)
                        {
                            _1968 = _1937 < _1960;
                        }
                        else
                        {
                            _1968 = false;
                        }
                        if (_1968)
                        {
                            float _1976;
                            _1976 = _1937;
                            float _1973 = _1960;
                            uint _1978 = 0u;
                            for (; _1978 < 8u; )
                            {
                                float _1982 = _1976 + _1973;
                                float _1983 = _1982 * 0.5f;
                                float3 _1985 = _1219 + (_1213 * _1983);
                                bool _1996 = _1985.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap[_2946[_2949_m0].Load(32)].SampleLevel(sampler_lnc[_2946[_2949_m0].Load(0)], (_1985.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                _1973 = _1996 ? _1983 : _1973;
                                _1976 = _1996 ? _1976 : _1983;
                                _1978++;
                                continue;
                            }
                            if (_1976 < _1906)
                            {
                                float2 _2005 = ((_1219 + (_1213 * _1976)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                _2021 = ((lerp(-2.5f, 24.0f, terrain_heightmap[_2946[_2949_m0].Load(28)].SampleLevel(sampler_default[_2946[_2949_m0].Load(12)], _2005, 0.0f).x) + 0.100000001490116119384765625f) > lerp(-2.5f, 24.0f, visual_terrain_heightmap[_2946[_2949_m0].Load(32)].SampleLevel(sampler_lnc[_2946[_2949_m0].Load(0)], _2005, 0.0f).x)) ? 1u : 2u;
                                _2022 = _1976;
                                break;
                            }
                            else
                            {
                                _2021 = _1904;
                                _2022 = _1906;
                                break;
                            }
                            break; // unreachable workaround
                        }
                        else
                        {
                            _2021 = _1904;
                            _2022 = _1906;
                            break;
                        }
                        break; // unreachable workaround
                    } while(false);
                    if (_2021 == 0u)
                    {
                        float4 _2904;
                        if (_1247 > 0.0f)
                        {
                            float _2841 = dot(_1213, asfloat(frame_constants[_2946[_2949_m0].Load(92)].Load4(80)).xyz);
                            float3 _2861 = lerp(lerp(float3(0.20000000298023223876953125f, 0.4000000059604644775390625f, 1.0f), float3(1.0f, 0.4000000059604644775390625f, 0.20000000298023223876953125f), ((_2841 * 0.5f) + 0.5f).xxx), 1.0f.xxx, (_1247 * _1247).xxx);
                            float3 _2866 = ((asfloat(frame_constants[_2946[_2949_m0].Load(92)].Load4(64)).xyz * pow(max(0.0f, _2841), 12.0f)) + (asfloat(frame_constants[_2946[_2949_m0].Load(92)].Load4(96)).xyz * pow(max(0.0f, dot(_1213, asfloat(frame_constants[_2946[_2949_m0].Load(92)].Load4(112)).xyz)), 12.0f))) * 3.0f;
                            float3 _2872 = max(0.0f.xxx, 1.0f.xxx - (asfloat(color_library[_2946[_2949_m0].Load(44)].Load4(176)).xyz * 0.920000016689300537109375f));
                            float3 _2902 = _1210.xyz + (_1215 * (((lerp(asfloat(frame_constants[_2946[_2949_m0].Load(92)].Load4(16)).xyz * 1.0f, asfloat(frame_constants[_2946[_2949_m0].Load(92)].Load4(32)).xyz, pow(clamp(1.0f - pow(1.0f - clamp(_1247 + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.64999997615814208984375f).xxx) * 1.099999904632568359375f) * (_2861 / dot(_2861, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx)) + max(0.0f.xxx, lerp((_2872 / dot(_2872, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx) * dot(_2866, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), _2866, 0.854700863361358642578125f.xxx) * 0.833333313465118408203125f.xxx)));
                            _2904 = float4(_2902.x, _2902.y, _2902.z, _1210.w);
                        }
                        else
                        {
                            float4 _2834;
                            if (frame_constants[_2946[_2949_m0].Load(92)].Load(140) == 2u)
                            {
                                float3 _2832 = _1210.xyz + ((((lerp(asfloat(color_library[_2946[_2949_m0].Load(44)].Load4(288)).xyz, asfloat(color_library[_2946[_2949_m0].Load(44)].Load4(304)).xyz, 0.1500000059604644775390625f.xxx) * 1.2000000476837158203125f) * 0.550000011920928955078125f) * 5.0f) * (1.0f - (smoothstep(0.4830000102519989013671875f, 0.50499999523162841796875f, asfloat(frame_constants[_2946[_2949_m0].Load(92)].Load(172))) - smoothstep(0.839999973773956298828125f, 0.910000026226043701171875f, asfloat(frame_constants[_2946[_2949_m0].Load(92)].Load(172))))));
                                _2834 = float4(_2832.x, _2832.y, _2832.z, _1210.w);
                            }
                            else
                            {
                                _2834 = _1210;
                            }
                            _2904 = _2834;
                        }
                        _2905 = _2904;
                        break;
                    }
                    else
                    {
                        float4 _2031;
                        if (0u == _1221)
                        {
                            float4 _2030 = _1210;
                            _2030.w = _2022;
                            _2031 = _2030;
                        }
                        else
                        {
                            _2031 = _1210;
                        }
                        float3 _2033 = _1219 + (_1213 * _2022);
                        bool _2034 = _2021 == 1u;
                        bool _2039;
                        if (!_2034)
                        {
                            _2039 = _2021 == 2u;
                        }
                        else
                        {
                            _2039 = true;
                        }
                        if (_2039)
                        {
                            float4 _2773;
                            float3 _2775;
                            float3 _2776;
                            float3 _2792;
                            if ((_1221 + 1u) == 1u)
                            {
                                float3 _2803 = _2031.xyz + (_1215 * terrain_lighting_texture[_2946[_2949_m0].Load(40)].SampleLevel(sampler_llc[_2946[_2949_m0].Load(8)], (_2033.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).xyz);
                                _2905 = float4(_2803.x, _2803.y, _2803.z, _2031.w);
                                break;
                            }
                            else
                            {
                                float3 _2457;
                                if (_2034)
                                {
                                    float3 _2449 = (lerp(asfloat(color_library[_2946[_2949_m0].Load(44)].Load4(288)).xyz, asfloat(color_library[_2946[_2949_m0].Load(44)].Load4(304)).xyz, 0.1500000059604644775390625f.xxx) * 1.2000000476837158203125f) * 0.550000011920928955078125f;
                                    float3 _2456;
                                    if (frame_constants[_2946[_2949_m0].Load(92)].Load(140) == 4u)
                                    {
                                        _2456 = lerp(_2449, 0.699999988079071044921875f.xxx, 0.5f.xxx);
                                    }
                                    else
                                    {
                                        _2456 = _2449;
                                    }
                                    _2457 = _2456;
                                }
                                else
                                {
                                    _2457 = float3(0.122000001370906829833984375f, 0.122000001370906829833984375f, 0.08900000154972076416015625f);
                                }
                                float4x4 _2459 = asfloat(uint4x4(frame_constants[_2946[_2949_m0].Load(92)].Load(2160), frame_constants[_2946[_2949_m0].Load(92)].Load(2176), frame_constants[_2946[_2949_m0].Load(92)].Load(2192), frame_constants[_2946[_2949_m0].Load(92)].Load(2208), frame_constants[_2946[_2949_m0].Load(92)].Load(2164), frame_constants[_2946[_2949_m0].Load(92)].Load(2180), frame_constants[_2946[_2949_m0].Load(92)].Load(2196), frame_constants[_2946[_2949_m0].Load(92)].Load(2212), frame_constants[_2946[_2949_m0].Load(92)].Load(2168), frame_constants[_2946[_2949_m0].Load(92)].Load(2184), frame_constants[_2946[_2949_m0].Load(92)].Load(2200), frame_constants[_2946[_2949_m0].Load(92)].Load(2216), frame_constants[_2946[_2949_m0].Load(92)].Load(2172), frame_constants[_2946[_2949_m0].Load(92)].Load(2188), frame_constants[_2946[_2949_m0].Load(92)].Load(2204), frame_constants[_2946[_2949_m0].Load(92)].Load(2220)));
                                float4 _2460 = mul(_2459, float4(0.0f, 0.0f, 1.0f, 0.0f));
                                float3 _2461 = _2460.xyz;
                                float3 _2462 = -_1213;
                                bool3 _2470 = ((frame_constants[_2946[_2949_m0].Load(92)].Load(204) & 1u) != 0u).xxx;
                                float3 _2471 = float3(_2470.x ? 0.5f.xxx.x : _2457.x, _2470.y ? 0.5f.xxx.y : _2457.y, _2470.z ? 0.5f.xxx.z : _2457.z);
                                float3 _2472 = -_2461;
                                float2 _2682;
                                do
                                {
                                    float4x4 _2476 = asfloat(uint4x4(frame_constants[_2946[_2949_m0].Load(92)].Load(2224), frame_constants[_2946[_2949_m0].Load(92)].Load(2240), frame_constants[_2946[_2949_m0].Load(92)].Load(2256), frame_constants[_2946[_2949_m0].Load(92)].Load(2272), frame_constants[_2946[_2949_m0].Load(92)].Load(2228), frame_constants[_2946[_2949_m0].Load(92)].Load(2244), frame_constants[_2946[_2949_m0].Load(92)].Load(2260), frame_constants[_2946[_2949_m0].Load(92)].Load(2276), frame_constants[_2946[_2949_m0].Load(92)].Load(2232), frame_constants[_2946[_2949_m0].Load(92)].Load(2248), frame_constants[_2946[_2949_m0].Load(92)].Load(2264), frame_constants[_2946[_2949_m0].Load(92)].Load(2280), frame_constants[_2946[_2949_m0].Load(92)].Load(2236), frame_constants[_2946[_2949_m0].Load(92)].Load(2252), frame_constants[_2946[_2949_m0].Load(92)].Load(2268), frame_constants[_2946[_2949_m0].Load(92)].Load(2284)));
                                    float4 _2481 = mul(_2476, float4(_2033, 1.0f));
                                    if (_2481.z > 1.0f)
                                    {
                                        _2682 = 0.0f.xx;
                                        break;
                                    }
                                    float2 _2486 = _2481.xy;
                                    float2 _2487 = abs(_2486);
                                    bool _2489 = any(bool2(_2487.x > 0.9833333492279052734375f.xx.x, _2487.y > 0.9833333492279052734375f.xx.y));
                                    float2 _2493;
                                    if (_2489)
                                    {
                                        _2493 = _2486 * 0.5f;
                                    }
                                    else
                                    {
                                        _2493 = _2486;
                                    }
                                    float _2494 = _2489 ? 5.0f : 10.0f;
                                    float _2496 = _2489 ? 0.5f : 1.0f;
                                    float2 _2497 = abs(_2493);
                                    float2 _2500 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * _2494));
                                    bool _2502 = any(bool2(_2497.x > _2500.x, _2497.y > _2500.y));
                                    float _2508;
                                    float2 _2509;
                                    float _2510;
                                    if (_2502)
                                    {
                                        _2508 = _2494 * 0.2857142984867095947265625f;
                                        _2509 = _2493 * 0.2857142984867095947265625f;
                                        _2510 = _2496 * 0.2857142984867095947265625f;
                                    }
                                    else
                                    {
                                        _2508 = _2494;
                                        _2509 = _2493;
                                        _2510 = _2496;
                                    }
                                    float _2513 = float(_2502 ? 2u : uint(_2489));
                                    float4x4 _2517 = asfloat(uint4x4(frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 624), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 640), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 656), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 672), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 628), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 644), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 660), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 676), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 632), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 648), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 664), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 680), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 636), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 652), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 668), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2513)) * 800 + 684)));
                                    float _2518 = _2472.y;
                                    float4 _2530 = float4(_2509.x, _2509.y, _2481.z, _2481.w) + mul(_2517, float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - _2518, 0.0f, 1.0f))) / _2510.xxx, 0.0f));
                                    float2 _2533 = (_2530.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                    float3 _2534 = float3(_2533.x, _2533.y, _2530.z);
                                    float _2539 = max((0.300000011920928955078125f / _2510) * (1.0f - abs(_2518)), 0.02500000037252902984619140625f);
                                    float _2550 = float((uint((_378.x * 11) + (_378.y * 7)) + (_465 * 5u)) % 64u) * 1.61803400516510009765625f;
                                    uint2 _2551 = uint2(_378);
                                    float _2555 = 5.588237762451171875f * float(_466);
                                    float _2565 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_2551.x) + _2555)) + (0.005837149918079376220703125f * (float(_2551.y) + _2555))));
                                    float4 _2567 = mul(_2517, float4(0.0f, 1.0f, 0.0f, 0.0f));
                                    float _2569;
                                    int _2572;
                                    _2569 = 0.0f;
                                    _2572 = 0;
                                    float _2570;
                                    int _2573;
                                    for (int _2574 = 0; _2574 < 5; _2569 = _2570, _2572 = _2573, _2574++)
                                    {
                                        float _2579 = float(_2574);
                                        float _2581 = (_2579 + _2550) * 2.3999631404876708984375f;
                                        float3 _2594 = _2534 + float3((((float2(cos(_2581), sin(_2581)) * (_2579 + _2565)) * 0.20000000298023223876953125f.xx) * _2508) * 0.000833333353511989116668701171875f.xx, _2539 * (-0.0009765625f));
                                        float4 _2601 = shadow_map[_2946[_2949_m0].Load(48)].SampleLevel(sampler_shadow_nearest[_2946[_2949_m0].Load(16)], float3(_2594.xy, _2513), 0.0f);
                                        float _2602 = _2601.x;
                                        if (_2594.z > _2602)
                                        {
                                            _2570 = _2569 + _2602;
                                            _2573 = _2572 + 1;
                                        }
                                        else
                                        {
                                            _2570 = _2569;
                                            _2573 = _2572;
                                        }
                                    }
                                    float _2610 = _2569 / float(_2572);
                                    if (_2572 == 0)
                                    {
                                        _2682 = 0.0f.xx;
                                        break;
                                    }
                                    float _2624 = max(((_2530.z - _2610) * ((17.0666675567626953125f * _2508) * pow(10.0f, smoothstep(0.0f, 0.25f, asfloat(frame_constants[_2946[_2949_m0].Load(92)].Load4(48)).w)))) / _2610, 0.0f);
                                    float _2631 = (_2624 / pow(pow(_2624 / (_2508 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
                                    float _2633 = _2567.z;
                                    float2 _2643 = (((_2567.xy * float(int(sign(_2633)))) / max(0.1500000059604644775390625f, abs(_2633)).xx) * 0.0009765625f.xx) * 2.0f;
                                    float2 _2645;
                                    _2645 = 0.0f.xx;
                                    for (int _2648 = 0; _2648 < 10; )
                                    {
                                        float _2652 = float(_2648);
                                        float _2654 = (_2652 + _2550) * 2.3999631404876708984375f;
                                        float2 _2661 = ((float2(cos(_2654), sin(_2654)) * (_2652 + _2565)) * 0.100000001490116119384765625f.xx) * _2631;
                                        float3 _2667 = _2534 + float3(_2661 * 0.000833333353511989116668701171875f.xx, _2539 * (-0.0009765625f));
                                        _2645 += float2(1.0f - shadow_map[_2946[_2949_m0].Load(48)].SampleCmpLevelZero(sampler_shadow_linear[_2946[_2949_m0].Load(20)], float3(_2667.xy, _2513), _2667.z + min(0.0f, dot(_2643, _2661))), _365);
                                        _2648++;
                                        continue;
                                    }
                                    _2682 = _2645 * 0.100000001490116119384765625f;
                                    break;
                                } while(false);
                                float3 _2684 = normalize(float3(0.0f, 1.0f, 0.0f));
                                float _2690 = clamp(dot(_2684, _2461), 0.0f, 1.0f);
                                float _2692 = clamp(dot(_2461, normalize(_2461 + _2462)), 0.0f, 1.0f);
                                float _2695 = ((1.28000009059906005859375f * _2692) * _2692) + (-0.5f);
                                float _2696 = 1.0f - _2690;
                                float _2697 = _2696 * _2696;
                                float _2702 = 1.0f - max(dot(_2684, _2462), 9.9999997473787516355514526367188e-05f);
                                float _2703 = _2702 * _2702;
                                float3 _2763;
                                do
                                {
                                    float _2713 = _2460.y;
                                    bool _2720;
                                    if (!(_2713 <= 0.0f))
                                    {
                                        _2720 = _2462.y <= 0.0f;
                                    }
                                    else
                                    {
                                        _2720 = true;
                                    }
                                    if (_2720)
                                    {
                                        _2763 = 0.0f.xxx;
                                        break;
                                    }
                                    float3 _2724 = normalize(_2462 + _2461);
                                    float _2725 = _2724.y;
                                    float _2728 = max(0.0f, 1.0f - dot(_2724, _2461));
                                    float _2729 = _2728 * _2728;
                                    float _2734 = _2462.y;
                                    float _2754 = ((_2725 * _2725) * (-0.5903999805450439453125f)) + 1.0f;
                                    _2763 = ((lerp(0.039999999105930328369140625f.xxx, 1.0f.xxx, ((_2729 * _2729) * _2728).xxx) * (((2.0f * _2713) * _2734) / ((_2713 * sqrt((((_2734 * (-0.4096000492572784423828125f)) + _2734) * _2734) + 0.4096000492572784423828125f)) + (_2734 * sqrt((((_2713 * (-0.4096000492572784423828125f)) + _2713) * _2713) + 0.4096000492572784423828125f))))) * (0.4096000492572784423828125f / ((3.1415927410125732421875f * _2754) * _2754))) / ((4.0f * _2734) * _2713).xxx;
                                    break;
                                } while(false);
                                float3 _2772 = _2031.xyz + (_1215 * ((((asfloat(frame_constants[_2946[_2949_m0].Load(92)].Load4(0)).xyz * ((_2471 * (((1.0f + (_2695 * ((_2697 * _2697) * _2696))) * (1.0f + (_2695 * ((_2703 * _2703) * _2702)))) * 0.3183098733425140380859375f)) + _2763)) * _2690) * 1.0f) * (1.0f - _2682.x)));
                                _2773 = float4(_2772.x, _2772.y, _2772.z, _2031.w);
                                _2775 = _1215 * float3(_2470.x ? 0.5f.xxx.x : _2471.x, _2470.y ? 0.5f.xxx.y : _2471.y, _2470.z ? 0.5f.xxx.z : _2471.z);
                                _2776 = _2033 + float3(0.0f, 0.001000000047497451305389404296875f, 0.0f);
                                float _2779 = 1.0f - (2.0f * _1241.x);
                                float _2783 = sqrt(max(0.0f, 1.0f - (_2779 * _2779)));
                                float _2785 = 6.283185482025146484375f * _1241.y;
                                _2792 = normalize(float3(0.0f, 1.0f, 0.0f) + float3(cos(_2785) * _2783, sin(_2785) * _2783, _2779));
                            }
                            _1211 = _2773;
                            _1214 = _2792;
                            _1216 = _2775;
                            _1220 = _2776;
                        }
                        else
                        {
                            float4 _2429;
                            float3 _2430;
                            float3 _2431;
                            float3 _2432;
                            if (_2021 == 3u)
                            {
                                if (dot(_1903, _1213) > 0.0f)
                                {
                                    _2905 = _2031;
                                    break;
                                }
                                float3 _2087;
                                do
                                {
                                    if ((rt_triangle_meta[_2946[_2949_m0].Load(56)].Load(_1905 * 4 + 0) & 16777216u) != 0u)
                                    {
                                        int _2076 = int(wall_color_ids[_2946[_2949_m0].Load(52)].Load((rt_triangle_meta[_2946[_2949_m0].Load(56)].Load(_1905 * 4 + 0) & 16777215u) * 4 + 0));
                                        float3 _2086;
                                        do
                                        {
                                            if (uint(_2076) < 13u)
                                            {
                                                _2086 = _327[_2076];
                                                break;
                                            }
                                            else
                                            {
                                                _2086 = float3(1.0f, 0.0f, 1.0f);
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        _2087 = _2086;
                                        break;
                                    }
                                    else
                                    {
                                        uint _2059 = rt_triangle_meta[_2946[_2949_m0].Load(56)].Load(_1905 * 4 + 0) & 16777215u;
                                        float3 _2071 = float3(float(_2059 & 255u) * 0.0039215688593685626983642578125f, float((_2059 >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((_2059 >> 16u) & 255u) * 0.0039215688593685626983642578125f);
                                        _2087 = _2071 * _2071;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                float4x4 _2089 = asfloat(uint4x4(frame_constants[_2946[_2949_m0].Load(92)].Load(2160), frame_constants[_2946[_2949_m0].Load(92)].Load(2176), frame_constants[_2946[_2949_m0].Load(92)].Load(2192), frame_constants[_2946[_2949_m0].Load(92)].Load(2208), frame_constants[_2946[_2949_m0].Load(92)].Load(2164), frame_constants[_2946[_2949_m0].Load(92)].Load(2180), frame_constants[_2946[_2949_m0].Load(92)].Load(2196), frame_constants[_2946[_2949_m0].Load(92)].Load(2212), frame_constants[_2946[_2949_m0].Load(92)].Load(2168), frame_constants[_2946[_2949_m0].Load(92)].Load(2184), frame_constants[_2946[_2949_m0].Load(92)].Load(2200), frame_constants[_2946[_2949_m0].Load(92)].Load(2216), frame_constants[_2946[_2949_m0].Load(92)].Load(2172), frame_constants[_2946[_2949_m0].Load(92)].Load(2188), frame_constants[_2946[_2949_m0].Load(92)].Load(2204), frame_constants[_2946[_2949_m0].Load(92)].Load(2220)));
                                float3 _2091 = mul(_2089, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
                                float3 _2092 = -_1213;
                                float3 _2093 = normalize(_1903);
                                bool3 _2101 = ((frame_constants[_2946[_2949_m0].Load(92)].Load(204) & 1u) != 0u).xxx;
                                float3 _2102 = float3(_2101.x ? 0.5f.xxx.x : _2087.x, _2101.y ? 0.5f.xxx.y : _2087.y, _2101.z ? 0.5f.xxx.z : _2087.z);
                                float2 _2317;
                                do
                                {
                                    float4x4 _2107 = asfloat(uint4x4(frame_constants[_2946[_2949_m0].Load(92)].Load(2224), frame_constants[_2946[_2949_m0].Load(92)].Load(2240), frame_constants[_2946[_2949_m0].Load(92)].Load(2256), frame_constants[_2946[_2949_m0].Load(92)].Load(2272), frame_constants[_2946[_2949_m0].Load(92)].Load(2228), frame_constants[_2946[_2949_m0].Load(92)].Load(2244), frame_constants[_2946[_2949_m0].Load(92)].Load(2260), frame_constants[_2946[_2949_m0].Load(92)].Load(2276), frame_constants[_2946[_2949_m0].Load(92)].Load(2232), frame_constants[_2946[_2949_m0].Load(92)].Load(2248), frame_constants[_2946[_2949_m0].Load(92)].Load(2264), frame_constants[_2946[_2949_m0].Load(92)].Load(2280), frame_constants[_2946[_2949_m0].Load(92)].Load(2236), frame_constants[_2946[_2949_m0].Load(92)].Load(2252), frame_constants[_2946[_2949_m0].Load(92)].Load(2268), frame_constants[_2946[_2949_m0].Load(92)].Load(2284)));
                                    float4 _2112 = mul(_2107, float4(_2033, 1.0f));
                                    if (_2112.z > 1.0f)
                                    {
                                        _2317 = 0.0f.xx;
                                        break;
                                    }
                                    float2 _2117 = _2112.xy;
                                    float2 _2118 = abs(_2117);
                                    bool _2120 = any(bool2(_2118.x > 0.9833333492279052734375f.xx.x, _2118.y > 0.9833333492279052734375f.xx.y));
                                    float2 _2124;
                                    if (_2120)
                                    {
                                        _2124 = _2117 * 0.5f;
                                    }
                                    else
                                    {
                                        _2124 = _2117;
                                    }
                                    float _2125 = _2120 ? 5.0f : 10.0f;
                                    float _2127 = _2120 ? 0.5f : 1.0f;
                                    float2 _2128 = abs(_2124);
                                    float2 _2131 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * _2125));
                                    bool _2133 = any(bool2(_2128.x > _2131.x, _2128.y > _2131.y));
                                    float _2139;
                                    float2 _2140;
                                    float _2141;
                                    if (_2133)
                                    {
                                        _2139 = _2125 * 0.2857142984867095947265625f;
                                        _2140 = _2124 * 0.2857142984867095947265625f;
                                        _2141 = _2127 * 0.2857142984867095947265625f;
                                    }
                                    else
                                    {
                                        _2139 = _2125;
                                        _2140 = _2124;
                                        _2141 = _2127;
                                    }
                                    float _2144 = float(_2133 ? 2u : uint(_2120));
                                    float4x4 _2148 = asfloat(uint4x4(frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 624), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 640), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 656), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 672), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 628), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 644), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 660), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 676), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 632), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 648), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 664), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 680), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 636), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 652), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 668), frame_constants[_2946[_2949_m0].Load(92)].Load((2u + uint(_2144)) * 800 + 684)));
                                    float _2149 = dot(_2093, -_2091);
                                    float4 _2161 = float4(_2140.x, _2140.y, _2112.z, _2112.w) + mul(_2148, float4((_2093 * (0.066666670143604278564453125f * clamp(1.0f - _2149, 0.0f, 1.0f))) / _2141.xxx, 0.0f));
                                    float2 _2164 = (_2161.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                    float3 _2165 = float3(_2164.x, _2164.y, _2161.z);
                                    float _2170 = max((0.300000011920928955078125f / _2141) * (1.0f - abs(_2149)), 0.02500000037252902984619140625f);
                                    float _2181 = float((uint((_378.x * 11) + (_378.y * 7)) + (_465 * 5u)) % 64u) * 1.61803400516510009765625f;
                                    uint2 _2182 = uint2(_378);
                                    float _2186 = 5.588237762451171875f * float(_466);
                                    float _2196 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_2182.x) + _2186)) + (0.005837149918079376220703125f * (float(_2182.y) + _2186))));
                                    float4 _2202 = mul(_2148, float4(_2093, 0.0f));
                                    float _2204;
                                    int _2207;
                                    _2204 = 0.0f;
                                    _2207 = 0;
                                    float _2205;
                                    int _2208;
                                    for (int _2209 = 0; _2209 < 5; _2204 = _2205, _2207 = _2208, _2209++)
                                    {
                                        float _2214 = float(_2209);
                                        float _2216 = (_2214 + _2181) * 2.3999631404876708984375f;
                                        float3 _2229 = _2165 + float3((((float2(cos(_2216), sin(_2216)) * (_2214 + _2196)) * 0.20000000298023223876953125f.xx) * _2139) * 0.000833333353511989116668701171875f.xx, _2170 * (-0.0009765625f));
                                        float4 _2236 = shadow_map[_2946[_2949_m0].Load(48)].SampleLevel(sampler_shadow_nearest[_2946[_2949_m0].Load(16)], float3(_2229.xy, _2144), 0.0f);
                                        float _2237 = _2236.x;
                                        if (_2229.z > _2237)
                                        {
                                            _2205 = _2204 + _2237;
                                            _2208 = _2207 + 1;
                                        }
                                        else
                                        {
                                            _2205 = _2204;
                                            _2208 = _2207;
                                        }
                                    }
                                    float _2245 = _2204 / float(_2207);
                                    if (_2207 == 0)
                                    {
                                        _2317 = 0.0f.xx;
                                        break;
                                    }
                                    float _2259 = max(((_2161.z - _2245) * ((17.0666675567626953125f * _2139) * pow(10.0f, smoothstep(0.0f, 0.25f, asfloat(frame_constants[_2946[_2949_m0].Load(92)].Load4(48)).w)))) / _2245, 0.0f);
                                    float _2266 = (_2259 / pow(pow(_2259 / (_2139 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
                                    float _2268 = _2202.z;
                                    float2 _2278 = (((_2202.xy * float(int(sign(_2268)))) / max(0.1500000059604644775390625f, abs(_2268)).xx) * 0.0009765625f.xx) * 2.0f;
                                    float2 _2280;
                                    _2280 = 0.0f.xx;
                                    for (int _2283 = 0; _2283 < 10; )
                                    {
                                        float _2287 = float(_2283);
                                        float _2289 = (_2287 + _2181) * 2.3999631404876708984375f;
                                        float2 _2296 = ((float2(cos(_2289), sin(_2289)) * (_2287 + _2196)) * 0.100000001490116119384765625f.xx) * _2266;
                                        float3 _2302 = _2165 + float3(_2296 * 0.000833333353511989116668701171875f.xx, _2170 * (-0.0009765625f));
                                        _2280 += float2(1.0f - shadow_map[_2946[_2949_m0].Load(48)].SampleCmpLevelZero(sampler_shadow_linear[_2946[_2949_m0].Load(20)], float3(_2302.xy, _2144), _2302.z + min(0.0f, dot(_2278, _2296))), _365);
                                        _2283++;
                                        continue;
                                    }
                                    _2317 = _2280 * 0.100000001490116119384765625f;
                                    break;
                                } while(false);
                                float3 _2319 = normalize(_2093);
                                float _2325 = clamp(dot(_2319, _2091), 0.0f, 1.0f);
                                float _2327 = clamp(dot(_2091, normalize(_2091 + _2092)), 0.0f, 1.0f);
                                float _2330 = ((1.28000009059906005859375f * _2327) * _2327) + (-0.5f);
                                float _2331 = 1.0f - _2325;
                                float _2332 = _2331 * _2331;
                                float _2337 = 1.0f - max(dot(_2319, _2092), 9.9999997473787516355514526367188e-05f);
                                float _2338 = _2337 * _2337;
                                float3 _2398;
                                do
                                {
                                    float _2348 = dot(_2093, _2091);
                                    bool _2355;
                                    if (!(_2348 <= 0.0f))
                                    {
                                        _2355 = dot(_2093, _2092) <= 0.0f;
                                    }
                                    else
                                    {
                                        _2355 = true;
                                    }
                                    if (_2355)
                                    {
                                        _2398 = 0.0f.xxx;
                                        break;
                                    }
                                    float3 _2359 = normalize(_2092 + _2091);
                                    float _2360 = dot(_2093, _2359);
                                    float _2363 = max(0.0f, 1.0f - dot(_2359, _2091));
                                    float _2364 = _2363 * _2363;
                                    float _2369 = dot(_2093, _2092);
                                    float _2389 = ((_2360 * _2360) * (-0.5903999805450439453125f)) + 1.0f;
                                    _2398 = ((lerp(0.039999999105930328369140625f.xxx, 1.0f.xxx, ((_2364 * _2364) * _2363).xxx) * (((2.0f * _2348) * _2369) / ((_2348 * sqrt((((_2369 * (-0.4096000492572784423828125f)) + _2369) * _2369) + 0.4096000492572784423828125f)) + (_2369 * sqrt((((_2348 * (-0.4096000492572784423828125f)) + _2348) * _2348) + 0.4096000492572784423828125f))))) * (0.4096000492572784423828125f / ((3.1415927410125732421875f * _2389) * _2389))) / ((4.0f * _2369) * _2348).xxx;
                                    break;
                                } while(false);
                                float3 _2407 = _2031.xyz + (_1215 * ((((asfloat(frame_constants[_2946[_2949_m0].Load(92)].Load4(0)).xyz * ((_2102 * (((1.0f + (_2330 * ((_2332 * _2332) * _2331))) * (1.0f + (_2330 * ((_2338 * _2338) * _2337)))) * 0.3183098733425140380859375f)) + _2398)) * _2325) * 1.0f) * (1.0f - _2317.x)));
                                float _2415 = 1.0f - (2.0f * _1241.x);
                                float _2419 = sqrt(max(0.0f, 1.0f - (_2415 * _2415)));
                                float _2421 = 6.283185482025146484375f * _1241.y;
                                _2429 = float4(_2407.x, _2407.y, _2407.z, _2031.w);
                                _2430 = normalize(_2093 + float3(cos(_2421) * _2419, sin(_2421) * _2419, _2415));
                                _2431 = _1215 * float3(_2101.x ? 0.5f.xxx.x : _2102.x, _2101.y ? 0.5f.xxx.y : _2102.y, _2101.z ? 0.5f.xxx.z : _2102.z);
                                _2432 = _2033 + (_2093 * 0.001000000047497451305389404296875f);
                            }
                            else
                            {
                                _2429 = _2031;
                                _2430 = _1213;
                                _2431 = _1215;
                                _2432 = _1219;
                            }
                            _1211 = _2429;
                            _1214 = _2430;
                            _1216 = _2431;
                            _1220 = _2432;
                        }
                    }
                    _1210 = _1211;
                    _1213 = _1214;
                    _1215 = _1216;
                    _1217 = _1218;
                    _1219 = _1220;
                    _1221++;
                    continue;
                }
                else
                {
                    _2905 = _1210;
                    break;
                }
            }
            _2908 = _2905 + float4(0.0f, 0.0f, 0.0f, _1175);
            break;
        } while(false);
        float _2913 = max(_2908.x, max(_2908.y, _2908.z));
        float3 _2918 = _2908.xyz * (min(_2913, 10.0f) / max(9.9999996826552253889678874634872e-21f, _2913));
        float4 _2919 = float4(_2918.x, _2918.y, _2918.z, _2908.w);
        uint _2921;
        _2921 = 0u;
        for (; _2921 < 2u; _2921++)
        {
            for (uint _2928 = 0u; _2928 < 2u; )
            {
                uint2 _2934 = _374 + uint2(_2928, _2921);
                candidate_radiance_tex[_2946[_2949_m0].Load(84)][_2934] = _2919;
                candidate_hit_tex[_2946[_2949_m0].Load(88)][_2934] = float4(_536, 0.0f);
                _2928++;
                continue;
            }
        }
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
