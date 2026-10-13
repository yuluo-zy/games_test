// ==RECONSTRUCTED== shaders/deferred/canopy_light_sun_sky.raster.hlsl | status: O (structure from disassembly); I (flag name; $Globals member names verbatim)
// MERGE (gi-deferred review 2026-10-13): adopted candidate B as the integration base (both
// b903/f0ad variants via DEFERRED_FULL_LIGHTING). The review's octahedral-normal and SV_Target
// alpha=2.0 merge deltas are REJECTED on bin evidence (the canopy b903 ps decodes the gbuffer
// normal as RGB8 normalize of gbuffer.x low bytes and writes alpha 0.0 — both match B as-is).

#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
struct ColorLibrary {
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};

cbuffer color_library : register(b23, space0) {
    ColorLibrary color_library;
};
#include "light_sun_sky_prelude.inc.hlsl"
#include "light_sun_sky.inc.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/blue_noise.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/perlin.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/sh.hlsl"
#include "inc/shadow.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/utils.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/wall_color_ids.hlsl"
#include "inc/material/eval/canopy_material.inc.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/cascades.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf_lut.hlsl"
#include "terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl"
#include "light_binning/grid.inc.hlsl"

#ifndef DEFERRED_FULL_LIGHTING
#define DEFERRED_FULL_LIGHTING 1
#endif

cbuffer $Globals : register(b13, space0)
{
    uint2 gbuffer_tex_dims;
    uint2 depth_tex_dims;
    uint2 half_depth_tex_dims;
    uint2 taa_history_tex_dims;
    uint taa_history_valid;
    uint2 blue_noise_texture_dims;
    uint2 ssgi_tex_dims;
    uint2 screen_shadows_tex_dims;
    uint2 terrain_lighting_texture_dims;
    uint2 highlight_active_hill_dims;
    float time;
    uint lantern_count;
    uint2 indirect_radiance_tex_dims;
    uint2 rdi_reservoir_tex_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
};

Texture2D<uint4> gbuffer_tex : register(t14, space0);
Texture2D<float4> depth_tex : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t16, space0);
Texture2D<float4> blue_noise_texture : register(t18, space0);
Texture2D<uint4> rdi_reservoir_tex : register(t28, space0);
ByteAddressBuffer light_indirection_grid : register(t36, space0);

#if DEFERRED_FULL_LIGHTING
ByteAddressBuffer lantern_buf : register(t26, space0);
Texture2D<float4> indirect_radiance_tex : register(t27, space0);
Texture2D<float4> shadow_map : register(t25, space0);
ByteAddressBuffer grid_light_indices : register(t37, space0);
#endif

// Flat color_library cbuffer at the deferred pass binding (b23). The inc's
// nested placeholder is guard-skipped so the flat members match the target.


#if DEFERRED_FULL_LIGHTING
// b903: pixel stage lifted from _deferred_canopy_light_sun_sky.raster.hlsl_b903e43ffb3da915.bin (spirv-cross, mechanically renamed;
// _NNN names are SPIR-V ids). Behaviour-exact; readability pass pending.
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
static float _262;
static uint _263;
static const float4 _269[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
static const Reservoir1sppStreamState _303 = { 0.0f, 0.0f };
static const Reservoir1spp _304 = { 0.0f, 0u, 0.0f, 0.0f };
float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}
float4 ps_main(VsToPs input) : SV_Target0
{
    float4 out_var_SV_TARGET;
    int2 _327 = int2(input.position.xy);
    uint2 _328 = uint2(_327);
    float4 _343 = ((blue_noise_texture.Load(int3((_328 + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint4 _345 = gbuffer_tex.Load(int3(_328, 0u));
    float4 _347 = depth_tex.Load(int3(_328, 0u));
    float _348 = _347.x;
    uint _349 = _345.x;
    float _363 = float((_349 >> 24u) & 255u);
    float _364 = _363 * 0.0039215688593685626983642578125f;
    float3 _368 = (float4(float(_349 & 255u) * 0.0039215688593685626983642578125f, float((_349 >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((_349 >> 16u) & 255u) * 0.0039215688593685626983642578125f, _364).xyz * 2.0f) - 1.0f.xxx;
    uint _369 = _345.y;
    float _384 = float((_369 >> 24u) & 255u) * 0.0039215688593685626983642578125f;
    float3 _386 = float4(float(_369 & 255u) * 0.0039215688593685626983642578125f, float((_369 >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((_369 >> 16u) & 255u) * 0.0039215688593685626983642578125f, _384).xyz;
    float3 _387 = _386 * _386;
    float3 _391 = mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float2 _394 = float2(depth_tex_dims);
    float2 _403 = (((float2(_327) + 0.5f.xx) / _394) - 0.5f.xx) * float2(2.0f, -2.0f);
    float _404 = _403.x;
    float4 _409 = float4(_404, _403.y, _348, 1.0f);
    float4 _411 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, _409));
    float3 _413 = normalize(mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(_404, _403.y, 0.0f, 1.0f))).xyz);
    float3 _414 = -_413;
    float3 _422 = (_411.xyz / _411.w.xxx) + (((_413 * 1.0f) * _384) * 1.60000002384185791015625f);
    bool3 _430 = ((render_overrides & 1u) != 0u).xxx;
    float3 _431 = float3(_430.x ? 0.5f.xxx.x : _387.x, _430.y ? 0.5f.xxx.y : _387.y, _430.z ? 0.5f.xxx.z : _387.z);
    bool _432 = _364 > 0.0f;
    float3 _475;
    if (_432)
    {
        uint _436 = uint(_363);
        uint _438 = _436 + (_436 << 10u);
        uint _440 = _438 ^ (_438 >> 6u);
        uint _442 = _440 + (_440 << 3u);
        uint _444 = _442 ^ (_442 >> 11u);
        uint _450 = _436 ^ 188053769u;
        uint _452 = _450 + (_450 << 10u);
        uint _454 = _452 ^ (_452 >> 6u);
        uint _456 = _454 + (_454 << 3u);
        uint _458 = _456 ^ (_456 >> 11u);
        float _464 = asfloat(((_458 + (_458 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float _465 = asfloat(((_444 + (_444 << 15u)) & 8388607u) | 1065353216u) * 3.141499996185302734375f;
        float _468 = sqrt(1.0f - (_464 * _464));
        _475 = float3(_468 * cos(_465), _468 * sin(_465), _464) * 0.5f;
    }
    else
    {
        _475 = 0.0f.xxx;
    }
    uint _476 = _432 ? 1065353216u : _263;
    uint _477 = _432 ? 8388607u : _263;
    bool _478 = _432 ? true : false;
    float _582;
    do
    {
        float4 _489 = mul(view_constants[2u].world_to_sample, float4(_422, 1.0f));
        float _490 = _489.z;
        if (_490 > 1.0f)
        {
            _582 = 1.0f;
            break;
        }
        float2 _494 = _489.xy;
        float2 _495 = abs(_494);
        bool _497 = any(bool2(_495.x > 0.98666667938232421875f.xx.x, _495.y > 0.98666667938232421875f.xx.y));
        float2 _501;
        if (_497)
        {
            _501 = _494 * 0.5f;
        }
        else
        {
            _501 = _494;
        }
        float _502 = _497 ? 4.0f : 8.0f;
        float2 _504 = abs(_501);
        float2 _507 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * _502));
        bool _509 = any(bool2(_504.x > _507.x, _504.y > _507.y));
        float _514;
        float2 _515;
        if (_509)
        {
            _514 = _502 * 0.2857142984867095947265625f;
            _515 = _501 * 0.2857142984867095947265625f;
        }
        else
        {
            _514 = _502;
            _515 = _501;
        }
        float _517 = float(_509 ? 2u : uint(_497));
        float2 _520 = (_515.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        float _531 = float((uint((_327.x * 11) + (_327.y * 7)) + (frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
        float _536 = 5.588237762451171875f * float(frame_index % 64u);
        float _546 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_328.x) + _536)) + (0.005837149918079376220703125f * (float(_328.y) + _536))));
        float _548;
        _548 = 0.0f;
        for (int _551 = 0; _551 < 6; )
        {
            float _555 = float(_551);
            float _557 = (_555 + _531) * 2.3999631404876708984375f;
            _548 += exp((-1.0f) * max(0.0f, (_490 - shadow_map.SampleLevel(sampler_shadow_nearest, float3(_520.xy + ((((float2(cos(_557), sin(_557)) * (_555 + _546)) * 0.16666667163372039794921875f.xx) * _514) * 0.000833333353511989116668701171875f.xx), _517), 0.0f).x) * 1024.0f));
            _551++;
            continue;
        }
        _582 = _548 * 0.16666667163372039794921875f;
        break;
    } while(false);
    float _591 = dot(_368, (_422 - _422) + ((normalize(_414) * (((1.0f - _384) * 2.0f) - 1.0f)) * 5.0f));
    float3 _597 = normalize(_368);
    float _601 = lerp(0.20000000298023223876953125f, 1.0f, smoothstep(-0.20000000298023223876953125f, 1.0f, dot(_597, _391))) * 0.800000011920928955078125f;
    float _605 = max(0.0f, 1.0f - abs(dot(_597, _414)));
    float _606 = _605 * _605;
    float _617 = abs(dot(normalize(_368 + _475), normalize(_391 + _414)));
    float _620 = ((_617 * _617) * (-0.5f)) + 1.0f;
    float _625 = max(0.0f, (-0.75f) * dot(_414, _391));
    float4 _321[3] = _269;
    float _641 = _343.x;
    uint _645 = uint((_641 * 3.0f) + 0.5f) & 3u;
    float _647;
    _647 = 0.0f;
    float _648;
    for (uint _650 = 0u; _650 < 3u; _647 = _648, _650++)
    {
        uint2 _662 = (_328 / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(_645 * 16u) + _650].xy);
        float _672 = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(_662, 0u)).x / _348))));
        for (uint _674 = 0u; _674 < 3u; )
        {
            _321[_674] += (indirect_radiance_tex.Load(int3(_662 + uint2((_674 * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * _672);
            _674++;
            continue;
        }
        _648 = _647 + _672;
    }
    float _739;
    if (_647 < 0.001000000047497451305389404296875f)
    {
        float _695;
        _695 = _647;
        float _696;
        for (uint _698 = 3u; _698 < 6u; _695 = _696, _698++)
        {
            uint2 _710 = (_328 / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(_645 * 16u) + _698].xy);
            float _720 = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(_710, 0u)).x / _348))));
            for (uint _722 = 0u; _722 < 3u; )
            {
                _321[_722] += (indirect_radiance_tex.Load(int3(_710 + uint2((_722 * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * _720);
                _722++;
                continue;
            }
            _696 = _695 + _720;
        }
        _739 = _695;
    }
    else
    {
        _739 = _647;
    }
    float _741 = 1.0f / max(9.9999997473787516355514526367188e-05f, _739);
    _321[0] *= _741;
    _321[1] *= _741;
    _321[2] *= _741;
    float3 _759 = (_321[0].yzw + _321[1].yzw) + _321[2].yzw;
    float3 _767 = float3(_321[0].x, _321[1].x, _321[2].x);
    float3 _770 = _321[0].yzw / max(9.9999996826552253889678874634872e-21f, _321[0].x).xxx;
    float3 _773 = _321[1].yzw / max(9.9999996826552253889678874634872e-21f, _321[1].x).xxx;
    float3 _776 = _321[2].yzw / max(9.9999996826552253889678874634872e-21f, _321[2].x).xxx;
    float3 _782 = max(0.0f.xxx, 1.0f.xxx - (color_library.indirect_light_tint.xyz * 0.920000016689300537109375f));
    float3 _790 = max(0.0f.xxx, lerp((_782 / dot(_782, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx) * dot(_767, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), _767, 1.16999995708465576171875f.xxx) * 1.2000000476837158203125f);
    uint _839;
    uint _840;
    bool _841;
    float3 _842;
    if (_432)
    {
        uint _794 = uint(_363);
        uint _796 = _794 + (_794 << 10u);
        uint _798 = _796 ^ (_796 >> 6u);
        uint _800 = _798 + (_798 << 3u);
        uint _802 = _800 ^ (_800 >> 11u);
        bool _805 = _478 ? _478 : true;
        uint _806 = _478 ? _477 : 8388607u;
        uint _807 = _478 ? _476 : 1065353216u;
        uint _811 = _794 ^ 188053769u;
        uint _813 = _811 + (_811 << 10u);
        uint _815 = _813 ^ (_813 >> 6u);
        uint _817 = _815 + (_815 << 3u);
        uint _819 = _817 ^ (_817 >> 11u);
        uint _823 = _805 ? _806 : 8388607u;
        uint _824 = _805 ? _807 : 1065353216u;
        float _828 = asfloat(((_819 + (_819 << 15u)) & _823) | _824) - 1.0f;
        float _829 = asfloat(((_802 + (_802 << 15u)) & _806) | _807) * 3.141499996185302734375f;
        float _832 = sqrt(1.0f - (_828 * _828));
        _839 = _824;
        _840 = _823;
        _841 = _805 ? _805 : true;
        _842 = float3(_832 * cos(_829), _832 * sin(_829), _828) * 0.5f;
    }
    else
    {
        _839 = _476;
        _840 = _477;
        _841 = _478;
        _842 = 0.0f.xxx;
    }
    float3 _850 = (-float3(_770.y, _773.y, _776.y)) * _790;
    float3 _855 = float3(_770.z, _773.z, _776.z) * _790;
    float3 _861 = (-float3(_770.x, _773.x, _776.x)) * _790;
    float3 _868 = normalize(float3(-dot(_861, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), -dot(_850, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), dot(_855, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))));
    float3 _894 = float3(abs(dot(float3(-_861.x, -_850.x, _855.x), _868)), abs(dot(float3(-_861.y, -_850.y, _855.y), _868)), abs(dot(float3(-_861.z, -_850.z, _855.z), _868))) / _790;
    float _900 = dot(_868, _368);
    float3 _914 = 0.886227548122406005859375f.xxx * _790;
    float3 _921 = (_790 * ((_894 * 0.07999999821186065673828125f) + ((_894 * 0.60000002384185791015625f) * _894))) * 0.25f;
    float _928 = clamp(exp(_591 * (-0.20000000298023223876953125f)), 0.0f, 1.0f);
    float3 _937 = normalize(reflect(_413, normalize(_368 + _842)));
    float _938 = dot(_868, _937);
    float _964 = clamp(dot(normalize(reflect(_413, _368)), _759 / max(9.9999996826552253889678874634872e-21f, length(_759)).xxx), 0.0f, 1.0f);
    float3 _971 = (((((_431 * 0.3183098733425140380859375f) * _601) * sun_color.xyz) * _582) + (((((((lerp(0.20000000298023223876953125f, 0.699999988079071044921875f, _606 * _606).xxx * (0.15915493667125701904296875f / ((3.1415927410125732421875f * _620) * _620))) * lerp(_601, 1.0f, _625)) * lerp(lerp(float(_432), clamp(exp(_591 * (-0.4000000059604644775390625f)), 0.0f, 1.0f), 0.5f), 1.0f, _625)) * sun_color.xyz) * lerp(0.800000011920928955078125f, 0.300000011920928955078125f, smoothstep(50.0f, 150.0f, length(_422.xz)))) * _582) * 1.5f)) + ((((lerp(0.300000011920928955078125f, 1.0f, _928).xxx * 1.0f) * _431) * (max(0.0f.xxx, (((_914 + ((1.0f.xxx * (_368.y * (-1.02332770824432373046875f))) * _850)) + ((1.0f.xxx * (_368.z * 1.02332770824432373046875f)) * _855)) + ((1.0f.xxx * (_368.x * (-1.02332770824432373046875f))) * _861)) + (_921 * (0.3153915703296661376953125f * (((3.0f * _900) * _900) - 1.0f)))) * 1.2999999523162841796875f)) + ((max(0.0f.xxx, (((_914 + ((1.0f.xxx * (_937.y * (-1.02332770824432373046875f))) * _850)) + ((1.0f.xxx * (_937.z * 1.02332770824432373046875f)) * _855)) + ((1.0f.xxx * (_937.x * (-1.02332770824432373046875f))) * _861)) + (_921 * (0.3153915703296661376953125f * (((3.0f * _938) * _938) - 1.0f)))) * (0.039999999105930328369140625f * lerp(_928, (0.75f * _928) + 0.75f, _964 * _964))) * 2.0f));
    float3 _2444;
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 _982 = uint3(clamp(floor((_422 * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint _989 = (32u * ((32u * _982.y) + _982.z)) + _982.x;
        bool _994 = WaveActiveAllTrue((light_indirection_grid.Load<uint>(_989 * 4 + 0) >> 20u) <= 2u);
        uint _995 = _994 ? 2u : 6u;
        float3 _2443;
        if (6u == _995)
        {
            uint _322[8];
            for (uint _1547 = 0u; _1547 < 8u; )
            {
                _322[_1547] = 4294967295u;
                _1547++;
                continue;
            }
            uint2 _1554 = uint2(_327 & int2(1, 1));
            uint _1559 = (_1554.x + (_1554.y * 2u)) & 3u;
            uint _1561;
            _1561 = 0u;
            uint _1562;
            uint _1564 = 0u;
            for (;;)
            {
                bool _1572;
                if (_1564 < 8u)
                {
                    _1572 = _1561 < 8u;
                }
                else
                {
                    _1572 = false;
                }
                if (_1572)
                {
                    uint4 _1584 = rdi_reservoir_tex.Load(int3(uint2((_327 / int2(2, 2)) + spatial_resolve_offsets_array[((_1559 * 16u) + _1564) + 256u].xy), 0u));
                    uint2 _1585 = _1584.xy;
                    uint _1586 = _1584.x;
                    bool _1592;
                    if (any(bool2(_1585.x != uint2(0u, 0u).x, _1585.y != uint2(0u, 0u).y)))
                    {
                        _1592 = _1586 < lantern_count;
                    }
                    else
                    {
                        _1592 = false;
                    }
                    bool _1598;
                    if (_1592)
                    {
                        _1598 = _322[0u] != _1586;
                    }
                    else
                    {
                        _1598 = false;
                    }
                    bool _1604;
                    if (_1598)
                    {
                        _1604 = _322[1u] != _1586;
                    }
                    else
                    {
                        _1604 = false;
                    }
                    bool _1610;
                    if (_1604)
                    {
                        _1610 = _322[2u] != _1586;
                    }
                    else
                    {
                        _1610 = false;
                    }
                    bool _1616;
                    if (_1610)
                    {
                        _1616 = _322[3u] != _1586;
                    }
                    else
                    {
                        _1616 = false;
                    }
                    bool _1622;
                    if (_1616)
                    {
                        _1622 = _322[4u] != _1586;
                    }
                    else
                    {
                        _1622 = false;
                    }
                    bool _1628;
                    if (_1622)
                    {
                        _1628 = _322[5u] != _1586;
                    }
                    else
                    {
                        _1628 = false;
                    }
                    bool _1634;
                    if (_1628)
                    {
                        _1634 = _322[6u] != _1586;
                    }
                    else
                    {
                        _1634 = false;
                    }
                    bool _1640;
                    if (_1634)
                    {
                        _1640 = _322[7u] != _1586;
                    }
                    else
                    {
                        _1640 = false;
                    }
                    if (_1640)
                    {
                        _322[_1561] = _1586;
                        _1562 = _1561 + 1u;
                    }
                    else
                    {
                        _1562 = _1561;
                    }
                    _1561 = _1562;
                    _1564++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint _1646;
            _1646 = _1561;
            uint _1647;
            for (uint _1649 = 0u; _1649 < 8u; _1646 = _1647, _1649++)
            {
                uint _1656 = QuadReadAcrossDiagonal(_322[_1649]);
                bool _1661;
                if (_1656 < lantern_count)
                {
                    _1661 = _1646 < 8u;
                }
                else
                {
                    _1661 = false;
                }
                if (_1661)
                {
                    bool _1672;
                    if (_322[0u] != _1656)
                    {
                        _1672 = _322[1u] != _1656;
                    }
                    else
                    {
                        _1672 = false;
                    }
                    bool _1678;
                    if (_1672)
                    {
                        _1678 = _322[2u] != _1656;
                    }
                    else
                    {
                        _1678 = false;
                    }
                    bool _1684;
                    if (_1678)
                    {
                        _1684 = _322[3u] != _1656;
                    }
                    else
                    {
                        _1684 = false;
                    }
                    bool _1690;
                    if (_1684)
                    {
                        _1690 = _322[4u] != _1656;
                    }
                    else
                    {
                        _1690 = false;
                    }
                    bool _1696;
                    if (_1690)
                    {
                        _1696 = _322[5u] != _1656;
                    }
                    else
                    {
                        _1696 = false;
                    }
                    bool _1702;
                    if (_1696)
                    {
                        _1702 = _322[6u] != _1656;
                    }
                    else
                    {
                        _1702 = false;
                    }
                    bool _1708;
                    if (_1702)
                    {
                        _1708 = _322[7u] != _1656;
                    }
                    else
                    {
                        _1708 = false;
                    }
                    uint _1713;
                    if (_1708)
                    {
                        _322[_1646] = _1656;
                        _1713 = _1646 + 1u;
                    }
                    else
                    {
                        _1713 = _1646;
                    }
                    _1647 = _1713;
                }
                else
                {
                    _1647 = _1646;
                }
            }
            Reservoir1sppStreamState _323[2];
            Reservoir1spp _324[2];
            for (uint _1715 = 0u; _1715 < 2u; )
            {
                _323[_1715] = _303;
                _324[_1715] = _304;
                _1715++;
                continue;
            }
            for (uint _1723 = 0u; _1723 < _1646; _1723++)
            {
                float2 _1736 = spvUnpackHalf2x16(lantern_buf.Load<uint>(_322[_1723] * 16 + 12) & 65535u);
                float _1737 = _1736.x;
                uint _1739 = lantern_buf.Load<uint>(_322[_1723] * 16 + 12) >> 16u;
                float3 _1740 = lantern_buf.Load<float3>(_322[_1723] * 16 + 0) - _422;
                float _1742 = dot(_1740, _1740);
                float3 _1749 = _1740 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                float3 _1794;
                do
                {
                    if (_1739 == 0u)
                    {
                        _1794 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                        break;
                    }
                    else
                    {
                        if (_1739 == 1u)
                        {
                            _1794 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                            break;
                        }
                        else
                        {
                            if (_1739 == 2u)
                            {
                                _1794 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                break;
                            }
                            else
                            {
                                if (_1739 == 3u)
                                {
                                    _1794 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                    break;
                                }
                                else
                                {
                                    _1794 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                float _1798 = dot(((_1794 * 4.0f) * _1737) * ((max(0.0f, dot(_368, _1740 / length(_1740).xxx)) * ((1.0f / (0.3359999954700469970703125f + _1742)) * smoothstep(36.0f * _1737, 16.0f * _1737, _1742))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_1749 / length(_1749).xxx).y)), float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _1799 = float(_1646);
                float _1803 = frac(_641 + (float(_1723) / _1799));
                if (_324[0].w_sum <= _324[1].w_sum)
                {
                    do
                    {
                        _323[0].M_sum += 1.0f;
                        float _1842 = _1798 * _1799;
                        bool _1857;
                        do
                        {
                            _324[0].w_sum += _1842;
                            _324[0].M += 1.0f;
                            if ((_1842 / _324[0].w_sum) >= _1803)
                            {
                                _324[0].payload = _322[_1723];
                                _1857 = true;
                                break;
                            }
                            else
                            {
                                _1857 = false;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        if (_1857)
                        {
                            _323[0].p_q_sel = _1798;
                            break;
                        }
                        else
                        {
                            break;
                        }
                        break; // unreachable workaround
                    } while(false);
                }
                else
                {
                    do
                    {
                        _323[1].M_sum += 1.0f;
                        float _1817 = _1798 * _1799;
                        bool _1832;
                        do
                        {
                            _324[1].w_sum += _1817;
                            _324[1].M += 1.0f;
                            if ((_1817 / _324[1].w_sum) >= _1803)
                            {
                                _324[1].payload = _322[_1723];
                                _1832 = true;
                                break;
                            }
                            else
                            {
                                _1832 = false;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        if (_1832)
                        {
                            _323[1].p_q_sel = _1798;
                            break;
                        }
                        else
                        {
                            break;
                        }
                        break; // unreachable workaround
                    } while(false);
                }
            }
            for (uint _1863 = 1u; _1863 < 2u; )
            {
                _323[0].M_sum += _323[_1863].M_sum;
                _1863++;
                continue;
            }
            for (uint _1874 = 1u; _1874 < 2u; )
            {
                _323[_1874].M_sum = _323[0].M_sum;
                _1874++;
                continue;
            }
            float3 _1890;
            _1890 = _971;
            float _1884;
            float _1887;
            float _1889;
            float3 _1891;
            uint _1895;
            uint _1897;
            bool _1899;
            bool _1901;
            float _1883 = 0.0f;
            float _1886 = 0.0f;
            float _1888 = 0.0f;
            uint _1892 = 0u;
            uint _1894 = _839;
            uint _1896 = _840;
            bool _1898 = _841;
            bool _1900 = _841;
            for (; _1892 < 2u; _1883 = _1884, _1886 = _1887, _1888 = _1889, _1890 = _1891, _1892++, _1894 = _1895, _1896 = _1897, _1898 = _1899, _1900 = _1901)
            {
                _324[_1892].M = _323[_1892].M_sum;
                _324[_1892].W = _324[_1892].w_sum / max(9.9999999392252902907785028219223e-09f, _324[_1892].M * _323[_1892].p_q_sel);
                if (_324[_1892].W > 0.0f)
                {
                    float2 _1928 = spvUnpackHalf2x16(lantern_buf.Load<uint>(_324[_1892].payload * 16 + 12) & 65535u);
                    float _1929 = _1928.x;
                    uint _1931 = lantern_buf.Load<uint>(_324[_1892].payload * 16 + 12) >> 16u;
                    float3 _1932 = lantern_buf.Load<float3>(_324[_1892].payload * 16 + 0) - _422;
                    float _1933 = length(_1932);
                    float _1934 = dot(_1932, _1932);
                    float3 _1941 = _1932 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 _1947 = _1933.xxx;
                    float3 _1986;
                    do
                    {
                        if (_1931 == 0u)
                        {
                            _1986 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (_1931 == 1u)
                            {
                                _1986 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (_1931 == 2u)
                                {
                                    _1986 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (_1931 == 3u)
                                    {
                                        _1986 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        _1986 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 _1989 = ((_1986 * 4.0f) * _1929) * ((max(0.0f, dot(_368, _1932 / _1947)) * ((1.0f / (0.3359999954700469970703125f + _1934)) * smoothstep(36.0f * _1929, 16.0f * _1929, _1934))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_1941 / length(_1941).xxx).y));
                    float _2434;
                    float _2435;
                    float _2436;
                    uint _2437;
                    uint _2438;
                    bool _2439;
                    bool _2440;
                    float3 _2441;
                    if ((_324[_1892].W * max(max(_1989.x, _1989.y), _1989.z)) > 0.0f)
                    {
                        float _2249;
                        float _2250;
                        float _2251;
                        float3 _2252;
                        if (_1933 > 0.5f)
                        {
                            float3 _2009 = (_422 + ((_1932 * (_1933 - 0.5f)) / _1947)) + (_368 * (0.0199999995529651641845703125f * _343.y));
                            float3 _2010 = _409.xyz;
                            float4 _2020 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(_2009, 1.0f)));
                            float3 _2024 = _2020.xyz / _2020.w.xxx;
                            float4 _2035 = _409 + (float4(_2024 - _2010, 0.0f) * float(int(sign(_2024.z))));
                            float _2036 = _2035.w;
                            float3 _2045 = (_2035 / (float((_2036 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_2036))).xxxx).xyz;
                            float3 _2046 = _2045 - _2010;
                            bool3 _2047 = bool3(_2046.x < 0.0f.xxx.x, _2046.y < 0.0f.xxx.y, _2046.z < 0.0f.xxx.z);
                            float3 _2050 = (float3(_2047.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _2047.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _2047.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _2010) / _2046;
                            float3 _2056 = _2010 + (_2046 * max(0.0f, max(_2050.x, _2050.y)));
                            float3 _2057 = _2045 - _2056;
                            bool3 _2058 = bool3(_2057.x >= 0.0f.xxx.x, _2057.y >= 0.0f.xxx.y, _2057.z >= 0.0f.xxx.z);
                            float3 _2061 = (float3(_2058.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _2058.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _2058.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _2056) / _2057;
                            float3 _2069 = _2056 + (_2057 * min(1.0f, min(min(_2061.x, _2061.y), _2061.z)));
                            float _2201;
                            float _2202;
                            float _2203;
                            bool _2216;
                            float _2217;
                            float _2218;
                            do
                            {
                                float _2090 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint _2091 = uint(int(max(2u, min(4u, uint(int(floor(length((((_2069.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_2056.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _394))))))));
                                float _2196;
                                float _2200;
                                bool _2204;
                                do
                                {
                                    float3 _2096 = _2069 - _2056;
                                    float _2104;
                                    if (_2069.z > 9.5367431640625e-07f)
                                    {
                                        _2104 = float(_2091);
                                    }
                                    else
                                    {
                                        _2104 = max(1.0f, float(_2091) - 1.0f);
                                    }
                                    float _2105 = 1.0f / _2104;
                                    float _2191;
                                    float _2192;
                                    float _2193;
                                    float _2194;
                                    bool _2195;
                                    if (_2091 > 0u)
                                    {
                                        float _2111 = lerp(0.0f, 1.0f, pow(_641 * _2105, 1.0f));
                                        float3 _2113 = _2056 + (_2096 * _2111);
                                        float2 _2116 = (_2113.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float _2118 = 1.0f / _2113.z;
                                        float _2123 = 1.0f / depth_tex.SampleLevel(sampler_llc, _2116, 0.0f).x;
                                        float _2128 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _2116, 0.0f).x;
                                        float _2132 = (max(_2123, _2128) * 1.00000202655792236328125f) - _2118;
                                        float _2133 = _2118 - min(_2123, _2128);
                                        bool _2136 = (_2132 < 0.0f) ? (_2133 < _2090) : false;
                                        float _2185;
                                        float _2186;
                                        float _2187;
                                        float _2188;
                                        bool _2189;
                                        float _2190;
                                        if (_2136)
                                        {
                                            _2185 = _2132;
                                            _2186 = _2133;
                                            _2187 = _2090;
                                            _2188 = _2111;
                                            _2189 = _2136;
                                            _2190 = 0.0f;
                                        }
                                        else
                                        {
                                            float _2146;
                                            _2146 = _2111;
                                            bool _2145;
                                            float _2147;
                                            float _2180;
                                            float _2181;
                                            float _2182;
                                            float _2183;
                                            bool _2184;
                                            uint _2141 = 1u;
                                            bool _2144 = _2136;
                                            for (;;)
                                            {
                                                if (_2141 < _2091)
                                                {
                                                    _2147 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_2141) + _641) * _2105, 1.0f)));
                                                    float3 _2157 = _2056 + (_2096 * _2147);
                                                    float2 _2160 = (_2157.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float _2162 = 1.0f / _2157.z;
                                                    float _2166 = 1.0f / depth_tex.SampleLevel(sampler_llc, _2160, 0.0f).x;
                                                    float _2170 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _2160, 0.0f).x;
                                                    float _2174 = (max(_2166, _2170) * 1.00000202655792236328125f) - _2162;
                                                    float _2175 = _2162 - min(_2166, _2170);
                                                    _2145 = (_2174 < 0.0f) ? (_2175 < _2090) : false;
                                                    if (_2145)
                                                    {
                                                        _2180 = _2174;
                                                        _2181 = _2175;
                                                        _2182 = _2090;
                                                        _2183 = _2147;
                                                        _2184 = _2145;
                                                        break;
                                                    }
                                                    _2141++;
                                                    _2144 = _2145;
                                                    _2146 = _2147;
                                                    continue;
                                                }
                                                else
                                                {
                                                    _2180 = 0.0f;
                                                    _2181 = 0.0f;
                                                    _2182 = 0.0f;
                                                    _2183 = 1.0f;
                                                    _2184 = _2144;
                                                    break;
                                                }
                                            }
                                            _2185 = _2180;
                                            _2186 = _2181;
                                            _2187 = _2182;
                                            _2188 = _2183;
                                            _2189 = _2184;
                                            _2190 = _2146;
                                        }
                                        _2191 = _2185;
                                        _2192 = _2186;
                                        _2193 = _2187;
                                        _2194 = _2188;
                                        _2195 = _2189;
                                        _2196 = _2190;
                                    }
                                    else
                                    {
                                        _2191 = 0.0f;
                                        _2192 = 0.0f;
                                        _2193 = 0.0f;
                                        _2194 = 1.0f;
                                        _2195 = false;
                                        _2196 = 0.0f;
                                    }
                                    if (_2195)
                                    {
                                        _2200 = _2194;
                                        _2201 = _2191;
                                        _2202 = _2193;
                                        _2203 = _2192;
                                        _2204 = true;
                                        break;
                                    }
                                    else
                                    {
                                        _2200 = _2196;
                                        _2201 = _1883;
                                        _2202 = _1886;
                                        _2203 = _1888;
                                        _2204 = false;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                bool _2212;
                                if (_2204)
                                {
                                    bool _2211;
                                    if (_2203 < _2202)
                                    {
                                        _2211 = _2201 < _2202;
                                    }
                                    else
                                    {
                                        _2211 = false;
                                    }
                                    _2212 = _2211;
                                }
                                else
                                {
                                    _2212 = false;
                                }
                                if (_2212)
                                {
                                    _2216 = true;
                                    _2217 = _2200;
                                    _2218 = _2203 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                _2216 = false;
                                _2217 = _2196;
                                _2218 = 0.0f;
                                break;
                            } while(false);
                            float3 _2248;
                            if (_2216)
                            {
                                float4 _2228 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(_2056, _2069, _2217.xxx), 1.0f)));
                                float3 _2235 = ((_2228.xyz / _2228.w.xxx) + (_414 * _2218)) - lantern_buf.Load<float3>(_324[_1892].payload * 16 + 0);
                                bool _2245;
                                if (!(abs(_2235.y) > 0.5f))
                                {
                                    float2 _2242 = _2235.xz;
                                    _2245 = dot(_2242, _2242) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    _2245 = true;
                                }
                                bool3 _2246 = _2245.xxx;
                                _2248 = float3(_2246.x ? 0.0f.xxx.x : _1989.x, _2246.y ? 0.0f.xxx.y : _1989.y, _2246.z ? 0.0f.xxx.z : _1989.z);
                            }
                            else
                            {
                                _2248 = _1989;
                            }
                            _2249 = _2201;
                            _2250 = _2202;
                            _2251 = _2203;
                            _2252 = _2248;
                        }
                        else
                        {
                            _2249 = _1883;
                            _2250 = _1886;
                            _2251 = _1888;
                            _2252 = _1989;
                        }
                        float2 _2262 = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(_324[_1892].payload * 10u));
                        int2 _2264 = int2(floor(_2262));
                        float2 _2265 = frac(_2262);
                        uint2 _2266 = uint2(_2264);
                        uint _2268 = _2266.y;
                        uint _2270 = _2268 + (_2268 << 10u);
                        uint _2272 = _2270 ^ (_2270 >> 6u);
                        uint _2274 = _2272 + (_2272 << 3u);
                        uint _2276 = _2274 ^ (_2274 >> 11u);
                        uint _2282 = (((_2266.x * 1664525u) + (_2276 + (_2276 << 15u))) + 1013904223u) * 1664525u;
                        uint _2284 = _2282 ^ (_2282 >> 11u);
                        uint _2287 = _2284 ^ ((_2284 << 7u) & 2636928640u);
                        uint _2290 = _2287 ^ ((_2287 << 15u) & 4022730752u);
                        bool _2293 = _1900 ? _1900 : true;
                        uint _2294 = _1900 ? _1896 : 8388607u;
                        bool _2295 = _1898 ? _1898 : true;
                        uint _2296 = _1898 ? _1894 : 1065353216u;
                        uint2 _2302 = uint2(_2264 + int2(1, 0));
                        uint _2304 = _2302.y;
                        uint _2306 = _2304 + (_2304 << 10u);
                        uint _2308 = _2306 ^ (_2306 >> 6u);
                        uint _2310 = _2308 + (_2308 << 3u);
                        uint _2312 = _2310 ^ (_2310 >> 11u);
                        uint _2318 = (((_2302.x * 1664525u) + (_2312 + (_2312 << 15u))) + 1013904223u) * 1664525u;
                        uint _2320 = _2318 ^ (_2318 >> 11u);
                        uint _2323 = _2320 ^ ((_2320 << 7u) & 2636928640u);
                        uint _2326 = _2323 ^ ((_2323 << 15u) & 4022730752u);
                        bool _2329 = _2293 ? _2293 : true;
                        uint _2330 = _2293 ? _2294 : 8388607u;
                        bool _2331 = _2295 ? _2295 : true;
                        uint _2332 = _2295 ? _2296 : 1065353216u;
                        uint2 _2338 = uint2(_2264 + int2(0, 1));
                        uint _2340 = _2338.y;
                        uint _2342 = _2340 + (_2340 << 10u);
                        uint _2344 = _2342 ^ (_2342 >> 6u);
                        uint _2346 = _2344 + (_2344 << 3u);
                        uint _2348 = _2346 ^ (_2346 >> 11u);
                        uint _2354 = (((_2338.x * 1664525u) + (_2348 + (_2348 << 15u))) + 1013904223u) * 1664525u;
                        uint _2356 = _2354 ^ (_2354 >> 11u);
                        uint _2359 = _2356 ^ ((_2356 << 7u) & 2636928640u);
                        uint _2362 = _2359 ^ ((_2359 << 15u) & 4022730752u);
                        bool _2365 = _2329 ? _2329 : true;
                        uint _2366 = _2329 ? _2330 : 8388607u;
                        bool _2367 = _2331 ? _2331 : true;
                        uint _2368 = _2331 ? _2332 : 1065353216u;
                        uint2 _2374 = uint2(_2264 + int2(1, 1));
                        uint _2376 = _2374.y;
                        uint _2378 = _2376 + (_2376 << 10u);
                        uint _2380 = _2378 ^ (_2378 >> 6u);
                        uint _2382 = _2380 + (_2380 << 3u);
                        uint _2384 = _2382 ^ (_2382 >> 11u);
                        uint _2390 = (((_2374.x * 1664525u) + (_2384 + (_2384 << 15u))) + 1013904223u) * 1664525u;
                        uint _2392 = _2390 ^ (_2390 >> 11u);
                        uint _2395 = _2392 ^ ((_2392 << 7u) & 2636928640u);
                        uint _2398 = _2395 ^ ((_2395 << 15u) & 4022730752u);
                        uint _2402 = _2365 ? _2366 : 8388607u;
                        uint _2404 = _2367 ? _2368 : 1065353216u;
                        float2 _2412 = (_2265 * _2265) * (3.0f.xx - (_2265 * 2.0f));
                        float _2413 = _2412.x;
                        float _2414 = 1.0f - _2413;
                        float _2416 = _2412.y;
                        _2434 = _2249;
                        _2435 = _2250;
                        _2436 = _2251;
                        _2437 = _2404;
                        _2438 = _2402;
                        _2439 = _2367 ? _2367 : true;
                        _2440 = _2365 ? _2365 : true;
                        _2441 = (((_2252 * lerp(0.5f, 1.0f, (((1.0f - _2416) * (((asfloat(((_2290 ^ (_2290 >> 18u)) & _2294) | _2296) - 1.0f) * _2414) + ((asfloat(((_2326 ^ (_2326 >> 18u)) & _2330) | _2332) - 1.0f) * _2413))) + (((asfloat(((_2362 ^ (_2362 >> 18u)) & _2366) | _2368) - 1.0f) * _2414) * _2416)) + (((asfloat(((_2398 ^ (_2398 >> 18u)) & _2402) | _2404) - 1.0f) * _2413) * _2416))) * _324[_1892].W) * _431) * ev_mult;
                    }
                    else
                    {
                        _2434 = _1883;
                        _2435 = _1886;
                        _2436 = _1888;
                        _2437 = _1894;
                        _2438 = _1896;
                        _2439 = _1898;
                        _2440 = _1900;
                        _2441 = 0.0f.xxx;
                    }
                    _1884 = _2434;
                    _1887 = _2435;
                    _1889 = _2436;
                    _1891 = _1890 + _2441;
                    _1895 = _2437;
                    _1897 = _2438;
                    _1899 = _2439;
                    _1901 = _2440;
                }
                else
                {
                    _1884 = _1883;
                    _1887 = _1886;
                    _1889 = _1888;
                    _1891 = _1890;
                    _1895 = _1894;
                    _1897 = _1896;
                    _1899 = _1898;
                    _1901 = _1900;
                }
            }
            _2443 = _1890;
        }
        else
        {
            float3 _1545;
            if (2u == _995)
            {
                uint _1005 = light_indirection_grid.Load<uint>(_989 * 4 + 0) & 1048575u;
                uint _1007 = min(64u, (light_indirection_grid.Load<uint>(_989 * 4 + 0) >> 20u));
                float3 _1009;
                _1009 = _971;
                float3 _1010;
                float _1013;
                float _1015;
                float _1017;
                uint _1021;
                uint _1023;
                bool _1025;
                bool _1027;
                float _1012 = 0.0f;
                float _1014 = 0.0f;
                float _1016 = 0.0f;
                uint _1018 = 0u;
                uint _1020 = _839;
                uint _1022 = _840;
                bool _1024 = _841;
                bool _1026 = _841;
                for (; _1018 < _1007; _1009 = _1010, _1012 = _1013, _1014 = _1015, _1016 = _1017, _1018++, _1020 = _1021, _1022 = _1023, _1024 = _1025, _1026 = _1027)
                {
                    uint _1031 = _1005 + _1018;
                    float2 _1039 = spvUnpackHalf2x16(lantern_buf.Load<uint>(grid_light_indices.Load<uint>(_1031 * 4 + 0) * 16 + 12) & 65535u);
                    float _1040 = _1039.x;
                    uint _1042 = lantern_buf.Load<uint>(grid_light_indices.Load<uint>(_1031 * 4 + 0) * 16 + 12) >> 16u;
                    float3 _1043 = lantern_buf.Load<float3>(grid_light_indices.Load<uint>(_1031 * 4 + 0) * 16 + 0) - _422;
                    float _1044 = length(_1043);
                    float _1045 = dot(_1043, _1043);
                    float3 _1052 = _1043 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 _1058 = _1044.xxx;
                    float3 _1097;
                    do
                    {
                        if (_1042 == 0u)
                        {
                            _1097 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (_1042 == 1u)
                            {
                                _1097 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (_1042 == 2u)
                                {
                                    _1097 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (_1042 == 3u)
                                    {
                                        _1097 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        _1097 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 _1100 = ((_1097 * 4.0f) * _1040) * ((max(0.0f, dot(_368, _1043 / _1058)) * ((1.0f / (0.3359999954700469970703125f + _1045)) * smoothstep(36.0f * _1040, 16.0f * _1040, _1045))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_1052 / length(_1052).xxx).y));
                    float3 _1544;
                    if (max(max(_1100.x, _1100.y), _1100.z) > 0.0f)
                    {
                        float _1359;
                        float _1360;
                        float _1361;
                        float3 _1362;
                        if (_1044 > 0.5f)
                        {
                            float3 _1119 = (_422 + ((_1043 * (_1044 - 0.5f)) / _1058)) + (_368 * (0.0199999995529651641845703125f * _343.y));
                            float3 _1120 = _409.xyz;
                            float4 _1130 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(_1119, 1.0f)));
                            float3 _1134 = _1130.xyz / _1130.w.xxx;
                            float4 _1145 = _409 + (float4(_1134 - _1120, 0.0f) * float(int(sign(_1134.z))));
                            float _1146 = _1145.w;
                            float3 _1155 = (_1145 / (float((_1146 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_1146))).xxxx).xyz;
                            float3 _1156 = _1155 - _1120;
                            bool3 _1157 = bool3(_1156.x < 0.0f.xxx.x, _1156.y < 0.0f.xxx.y, _1156.z < 0.0f.xxx.z);
                            float3 _1160 = (float3(_1157.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1157.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1157.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1120) / _1156;
                            float3 _1166 = _1120 + (_1156 * max(0.0f, max(_1160.x, _1160.y)));
                            float3 _1167 = _1155 - _1166;
                            bool3 _1168 = bool3(_1167.x >= 0.0f.xxx.x, _1167.y >= 0.0f.xxx.y, _1167.z >= 0.0f.xxx.z);
                            float3 _1171 = (float3(_1168.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1168.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1168.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1166) / _1167;
                            float3 _1179 = _1166 + (_1167 * min(1.0f, min(min(_1171.x, _1171.y), _1171.z)));
                            float _1311;
                            float _1312;
                            float _1313;
                            bool _1326;
                            float _1327;
                            float _1328;
                            do
                            {
                                float _1200 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint _1201 = uint(int(max(2u, min(4u, uint(int(floor(length((((_1179.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_1166.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _394))))))));
                                float _1306;
                                float _1310;
                                bool _1314;
                                do
                                {
                                    float3 _1206 = _1179 - _1166;
                                    float _1214;
                                    if (_1179.z > 9.5367431640625e-07f)
                                    {
                                        _1214 = float(_1201);
                                    }
                                    else
                                    {
                                        _1214 = max(1.0f, float(_1201) - 1.0f);
                                    }
                                    float _1215 = 1.0f / _1214;
                                    float _1301;
                                    float _1302;
                                    float _1303;
                                    float _1304;
                                    bool _1305;
                                    if (_1201 > 0u)
                                    {
                                        float _1221 = lerp(0.0f, 1.0f, pow(_641 * _1215, 1.0f));
                                        float3 _1223 = _1166 + (_1206 * _1221);
                                        float2 _1226 = (_1223.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float _1228 = 1.0f / _1223.z;
                                        float _1233 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1226, 0.0f).x;
                                        float _1238 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1226, 0.0f).x;
                                        float _1242 = (max(_1233, _1238) * 1.00000202655792236328125f) - _1228;
                                        float _1243 = _1228 - min(_1233, _1238);
                                        bool _1246 = (_1242 < 0.0f) ? (_1243 < _1200) : false;
                                        float _1295;
                                        float _1296;
                                        float _1297;
                                        float _1298;
                                        bool _1299;
                                        float _1300;
                                        if (_1246)
                                        {
                                            _1295 = _1242;
                                            _1296 = _1243;
                                            _1297 = _1200;
                                            _1298 = _1221;
                                            _1299 = _1246;
                                            _1300 = 0.0f;
                                        }
                                        else
                                        {
                                            float _1256;
                                            _1256 = _1221;
                                            bool _1255;
                                            float _1257;
                                            float _1290;
                                            float _1291;
                                            float _1292;
                                            float _1293;
                                            bool _1294;
                                            uint _1251 = 1u;
                                            bool _1254 = _1246;
                                            for (;;)
                                            {
                                                if (_1251 < _1201)
                                                {
                                                    _1257 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1251) + _641) * _1215, 1.0f)));
                                                    float3 _1267 = _1166 + (_1206 * _1257);
                                                    float2 _1270 = (_1267.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float _1272 = 1.0f / _1267.z;
                                                    float _1276 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1270, 0.0f).x;
                                                    float _1280 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1270, 0.0f).x;
                                                    float _1284 = (max(_1276, _1280) * 1.00000202655792236328125f) - _1272;
                                                    float _1285 = _1272 - min(_1276, _1280);
                                                    _1255 = (_1284 < 0.0f) ? (_1285 < _1200) : false;
                                                    if (_1255)
                                                    {
                                                        _1290 = _1284;
                                                        _1291 = _1285;
                                                        _1292 = _1200;
                                                        _1293 = _1257;
                                                        _1294 = _1255;
                                                        break;
                                                    }
                                                    _1251++;
                                                    _1254 = _1255;
                                                    _1256 = _1257;
                                                    continue;
                                                }
                                                else
                                                {
                                                    _1290 = 0.0f;
                                                    _1291 = 0.0f;
                                                    _1292 = 0.0f;
                                                    _1293 = 1.0f;
                                                    _1294 = _1254;
                                                    break;
                                                }
                                            }
                                            _1295 = _1290;
                                            _1296 = _1291;
                                            _1297 = _1292;
                                            _1298 = _1293;
                                            _1299 = _1294;
                                            _1300 = _1256;
                                        }
                                        _1301 = _1295;
                                        _1302 = _1296;
                                        _1303 = _1297;
                                        _1304 = _1298;
                                        _1305 = _1299;
                                        _1306 = _1300;
                                    }
                                    else
                                    {
                                        _1301 = 0.0f;
                                        _1302 = 0.0f;
                                        _1303 = 0.0f;
                                        _1304 = 1.0f;
                                        _1305 = false;
                                        _1306 = 0.0f;
                                    }
                                    if (_1305)
                                    {
                                        _1310 = _1304;
                                        _1311 = _1301;
                                        _1312 = _1303;
                                        _1313 = _1302;
                                        _1314 = true;
                                        break;
                                    }
                                    else
                                    {
                                        _1310 = _1306;
                                        _1311 = _1012;
                                        _1312 = _1014;
                                        _1313 = _1016;
                                        _1314 = false;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                bool _1322;
                                if (_1314)
                                {
                                    bool _1321;
                                    if (_1313 < _1312)
                                    {
                                        _1321 = _1311 < _1312;
                                    }
                                    else
                                    {
                                        _1321 = false;
                                    }
                                    _1322 = _1321;
                                }
                                else
                                {
                                    _1322 = false;
                                }
                                if (_1322)
                                {
                                    _1326 = true;
                                    _1327 = _1310;
                                    _1328 = _1313 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                _1326 = false;
                                _1327 = _1306;
                                _1328 = 0.0f;
                                break;
                            } while(false);
                            float3 _1358;
                            if (_1326)
                            {
                                float4 _1338 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(_1166, _1179, _1327.xxx), 1.0f)));
                                float3 _1345 = ((_1338.xyz / _1338.w.xxx) + (_414 * _1328)) - lantern_buf.Load<float3>(grid_light_indices.Load<uint>(_1031 * 4 + 0) * 16 + 0);
                                bool _1355;
                                if (!(abs(_1345.y) > 0.5f))
                                {
                                    float2 _1352 = _1345.xz;
                                    _1355 = dot(_1352, _1352) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    _1355 = true;
                                }
                                bool3 _1356 = _1355.xxx;
                                _1358 = float3(_1356.x ? 0.0f.xxx.x : _1100.x, _1356.y ? 0.0f.xxx.y : _1100.y, _1356.z ? 0.0f.xxx.z : _1100.z);
                            }
                            else
                            {
                                _1358 = _1100;
                            }
                            _1359 = _1311;
                            _1360 = _1312;
                            _1361 = _1313;
                            _1362 = _1358;
                        }
                        else
                        {
                            _1359 = _1012;
                            _1360 = _1014;
                            _1361 = _1016;
                            _1362 = _1100;
                        }
                        float2 _1372 = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(grid_light_indices.Load<uint>(_1031 * 4 + 0) * 10u));
                        int2 _1374 = int2(floor(_1372));
                        float2 _1375 = frac(_1372);
                        uint2 _1376 = uint2(_1374);
                        uint _1378 = _1376.y;
                        uint _1380 = _1378 + (_1378 << 10u);
                        uint _1382 = _1380 ^ (_1380 >> 6u);
                        uint _1384 = _1382 + (_1382 << 3u);
                        uint _1386 = _1384 ^ (_1384 >> 11u);
                        uint _1392 = (((_1376.x * 1664525u) + (_1386 + (_1386 << 15u))) + 1013904223u) * 1664525u;
                        uint _1394 = _1392 ^ (_1392 >> 11u);
                        uint _1397 = _1394 ^ ((_1394 << 7u) & 2636928640u);
                        uint _1400 = _1397 ^ ((_1397 << 15u) & 4022730752u);
                        bool _1403 = _1026 ? _1026 : true;
                        uint _1404 = _1026 ? _1022 : 8388607u;
                        bool _1405 = _1024 ? _1024 : true;
                        uint _1406 = _1024 ? _1020 : 1065353216u;
                        uint2 _1412 = uint2(_1374 + int2(1, 0));
                        uint _1414 = _1412.y;
                        uint _1416 = _1414 + (_1414 << 10u);
                        uint _1418 = _1416 ^ (_1416 >> 6u);
                        uint _1420 = _1418 + (_1418 << 3u);
                        uint _1422 = _1420 ^ (_1420 >> 11u);
                        uint _1428 = (((_1412.x * 1664525u) + (_1422 + (_1422 << 15u))) + 1013904223u) * 1664525u;
                        uint _1430 = _1428 ^ (_1428 >> 11u);
                        uint _1433 = _1430 ^ ((_1430 << 7u) & 2636928640u);
                        uint _1436 = _1433 ^ ((_1433 << 15u) & 4022730752u);
                        bool _1439 = _1403 ? _1403 : true;
                        uint _1440 = _1403 ? _1404 : 8388607u;
                        bool _1441 = _1405 ? _1405 : true;
                        uint _1442 = _1405 ? _1406 : 1065353216u;
                        uint2 _1448 = uint2(_1374 + int2(0, 1));
                        uint _1450 = _1448.y;
                        uint _1452 = _1450 + (_1450 << 10u);
                        uint _1454 = _1452 ^ (_1452 >> 6u);
                        uint _1456 = _1454 + (_1454 << 3u);
                        uint _1458 = _1456 ^ (_1456 >> 11u);
                        uint _1464 = (((_1448.x * 1664525u) + (_1458 + (_1458 << 15u))) + 1013904223u) * 1664525u;
                        uint _1466 = _1464 ^ (_1464 >> 11u);
                        uint _1469 = _1466 ^ ((_1466 << 7u) & 2636928640u);
                        uint _1472 = _1469 ^ ((_1469 << 15u) & 4022730752u);
                        bool _1475 = _1439 ? _1439 : true;
                        uint _1476 = _1439 ? _1440 : 8388607u;
                        bool _1477 = _1441 ? _1441 : true;
                        uint _1478 = _1441 ? _1442 : 1065353216u;
                        uint2 _1484 = uint2(_1374 + int2(1, 1));
                        uint _1486 = _1484.y;
                        uint _1488 = _1486 + (_1486 << 10u);
                        uint _1490 = _1488 ^ (_1488 >> 6u);
                        uint _1492 = _1490 + (_1490 << 3u);
                        uint _1494 = _1492 ^ (_1492 >> 11u);
                        uint _1500 = (((_1484.x * 1664525u) + (_1494 + (_1494 << 15u))) + 1013904223u) * 1664525u;
                        uint _1502 = _1500 ^ (_1500 >> 11u);
                        uint _1505 = _1502 ^ ((_1502 << 7u) & 2636928640u);
                        uint _1508 = _1505 ^ ((_1505 << 15u) & 4022730752u);
                        uint _1512 = _1475 ? _1476 : 8388607u;
                        uint _1514 = _1477 ? _1478 : 1065353216u;
                        float2 _1522 = (_1375 * _1375) * (3.0f.xx - (_1375 * 2.0f));
                        float _1523 = _1522.x;
                        float _1524 = 1.0f - _1523;
                        float _1526 = _1522.y;
                        _1013 = _1359;
                        _1015 = _1360;
                        _1017 = _1361;
                        _1021 = _1514;
                        _1023 = _1512;
                        _1025 = _1477 ? _1477 : true;
                        _1027 = _1475 ? _1475 : true;
                        _1544 = (((_1362 * lerp(0.5f, 1.0f, (((1.0f - _1526) * (((asfloat(((_1400 ^ (_1400 >> 18u)) & _1404) | _1406) - 1.0f) * _1524) + ((asfloat(((_1436 ^ (_1436 >> 18u)) & _1440) | _1442) - 1.0f) * _1523))) + (((asfloat(((_1472 ^ (_1472 >> 18u)) & _1476) | _1478) - 1.0f) * _1524) * _1526)) + (((asfloat(((_1508 ^ (_1508 >> 18u)) & _1512) | _1514) - 1.0f) * _1523) * _1526))) * 1.0f) * _431) * ev_mult;
                    }
                    else
                    {
                        _1013 = _1012;
                        _1015 = _1014;
                        _1017 = _1016;
                        _1021 = _1020;
                        _1023 = _1022;
                        _1025 = _1024;
                        _1027 = _1026;
                        _1544 = 0.0f.xxx;
                    }
                    _1010 = _1009 + _1544;
                }
                _1545 = _1009;
            }
            else
            {
                _1545 = _971;
            }
            _2443 = _1545;
        }
        _2444 = _2443;
    }
    else
    {
        _2444 = _971;
    }
    out_var_SV_TARGET = float4(_2444, 0.0f);
    return out_var_SV_TARGET;
}
#pragma dxc diagnostic pop
#else
// f0ad: pixel stage lifted from _deferred_canopy_light_sun_sky.raster.hlsl_f0adff76827e73ff.bin (spirv-cross, mechanically renamed;
// _NNN names are SPIR-V ids). Behaviour-exact; readability pass pending.
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"

void ps_main(VsToPs input)
{
    int2 _148 = int2(input.position.xy);
    uint2 _149 = uint2(_148);
    float2 _171 = (((float2(_148) + 0.5f.xx) / float2(depth_tex_dims)) - 0.5f.xx) * float2(2.0f, -2.0f);
    float _172 = _171.x;
    float4 _179 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(_172, _171.y, depth_tex.Load(int3(_149, 0u)).x, 1.0f)));
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 _200 = uint3(clamp(floor((((_179.xyz / _179.w.xxx) + (((normalize(mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(_172, _171.y, 0.0f, 1.0f))).xyz) * 1.0f) * (float((gbuffer_tex.Load(int3(_149, 0u)).y >> 24u) & 255u) * 0.0039215688593685626983642578125f)) * 1.60000002384185791015625f)) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        if (6u == (WaveActiveAllTrue((light_indirection_grid.Load<uint>(((32u * ((32u * _200.y) + _200.z)) + _200.x) * 4 + 0) >> 20u) <= 2u) ? 2u : 6u))
        {
            uint _145[8];
            for (uint _219 = 0u; _219 < 8u; )
            {
                _145[_219] = 4294967295u;
                _219++;
                continue;
            }
            uint2 _226 = uint2(_148 & int2(1, 1));
            uint _231 = (_226.x + (_226.y * 2u)) & 3u;
            uint _233;
            _233 = 0u;
            uint _234;
            uint _236 = 0u;
            for (;;)
            {
                bool _244;
                if (_236 < 8u)
                {
                    _244 = _233 < 8u;
                }
                else
                {
                    _244 = false;
                }
                if (_244)
                {
                    uint4 _256 = rdi_reservoir_tex.Load(int3(uint2((_148 / int2(2, 2)) + spatial_resolve_offsets_array[((_231 * 16u) + _236) + 256u].xy), 0u));
                    uint2 _257 = _256.xy;
                    uint _258 = _256.x;
                    bool _264;
                    if (any(bool2(_257.x != uint2(0u, 0u).x, _257.y != uint2(0u, 0u).y)))
                    {
                        _264 = _258 < lantern_count;
                    }
                    else
                    {
                        _264 = false;
                    }
                    bool _270;
                    if (_264)
                    {
                        _270 = _145[0u] != _258;
                    }
                    else
                    {
                        _270 = false;
                    }
                    bool _276;
                    if (_270)
                    {
                        _276 = _145[1u] != _258;
                    }
                    else
                    {
                        _276 = false;
                    }
                    bool _282;
                    if (_276)
                    {
                        _282 = _145[2u] != _258;
                    }
                    else
                    {
                        _282 = false;
                    }
                    bool _288;
                    if (_282)
                    {
                        _288 = _145[3u] != _258;
                    }
                    else
                    {
                        _288 = false;
                    }
                    bool _294;
                    if (_288)
                    {
                        _294 = _145[4u] != _258;
                    }
                    else
                    {
                        _294 = false;
                    }
                    bool _300;
                    if (_294)
                    {
                        _300 = _145[5u] != _258;
                    }
                    else
                    {
                        _300 = false;
                    }
                    bool _306;
                    if (_300)
                    {
                        _306 = _145[6u] != _258;
                    }
                    else
                    {
                        _306 = false;
                    }
                    bool _312;
                    if (_306)
                    {
                        _312 = _145[7u] != _258;
                    }
                    else
                    {
                        _312 = false;
                    }
                    if (_312)
                    {
                        _145[_233] = _258;
                        _234 = _233 + 1u;
                    }
                    else
                    {
                        _234 = _233;
                    }
                    _233 = _234;
                    _236++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint _319;
            for (uint _318 = _233, _321 = 0u; _321 < 8u; _318 = _319, _321++)
            {
                uint _328 = QuadReadAcrossDiagonal(_145[_321]);
                bool _333;
                if (_328 < lantern_count)
                {
                    _333 = _318 < 8u;
                }
                else
                {
                    _333 = false;
                }
                if (_333)
                {
                    bool _344;
                    if (_145[0u] != _328)
                    {
                        _344 = _145[1u] != _328;
                    }
                    else
                    {
                        _344 = false;
                    }
                    bool _350;
                    if (_344)
                    {
                        _350 = _145[2u] != _328;
                    }
                    else
                    {
                        _350 = false;
                    }
                    bool _356;
                    if (_350)
                    {
                        _356 = _145[3u] != _328;
                    }
                    else
                    {
                        _356 = false;
                    }
                    bool _362;
                    if (_356)
                    {
                        _362 = _145[4u] != _328;
                    }
                    else
                    {
                        _362 = false;
                    }
                    bool _368;
                    if (_362)
                    {
                        _368 = _145[5u] != _328;
                    }
                    else
                    {
                        _368 = false;
                    }
                    bool _374;
                    if (_368)
                    {
                        _374 = _145[6u] != _328;
                    }
                    else
                    {
                        _374 = false;
                    }
                    bool _380;
                    if (_374)
                    {
                        _380 = _145[7u] != _328;
                    }
                    else
                    {
                        _380 = false;
                    }
                    uint _385;
                    if (_380)
                    {
                        _145[_318] = _328;
                        _385 = _318 + 1u;
                    }
                    else
                    {
                        _385 = _318;
                    }
                    _319 = _385;
                }
                else
                {
                    _319 = _318;
                }
            }
        }
        else
        {
        }
    }
}
#pragma dxc diagnostic pop
#endif
