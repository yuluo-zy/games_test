// ==RECONSTRUCTED== shaders/deferred/grass_light_sun_sky.raster.hlsl | status: O (structure from disassembly); I (flag name; $Globals member names verbatim)
// MERGE (gi-deferred review 2026-10-13): adopted candidate B as the integration base (both
// b903/f0ad variants via DEFERRED_FULL_LIGHTING). The review's octahedral-normal and SV_Target
// alpha=2.0 merge deltas are REJECTED on bin evidence (the PS decodes the gbuffer normal as
// RGB8 normalize of gbuffer.x low bytes and writes alpha 0.0 — both match B as-is).

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
#include "inc/material/eval/grass_material.inc.hlsl"
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
    uint2 brdf_fg_lut_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
};

Texture2D<uint4> gbuffer_tex : register(t14, space0);
Texture2D<float4> depth_tex : register(t15, space0);
Texture2D<float4> blue_noise_texture : register(t18, space0);
Texture2D<uint4> rdi_reservoir_tex : register(t28, space0);
ByteAddressBuffer light_indirection_grid : register(t36, space0);

#if DEFERRED_FULL_LIGHTING
Texture2D<float4> half_depth_tex : register(t16, space0);
Texture2D<float4> ssgi_tex : register(t19, space0);
Texture2D<float4> screen_shadows_tex : register(t20, space0);
Texture2D<float4> highlight_active_hill : register(t22, space0);
ByteAddressBuffer lantern_buf : register(t26, space0);
Texture2D<float4> indirect_radiance_tex : register(t27, space0);
Texture2D<float4> brdf_fg_lut : register(t29, space0);
ByteAddressBuffer grid_light_indices : register(t37, space0);
#endif

// Flat color_library cbuffer at the deferred pass binding (b23). The inc's
// nested placeholder is guard-skipped so the flat members match the target.


#if DEFERRED_FULL_LIGHTING
// b903: pixel stage lifted from _deferred_grass_light_sun_sky.raster.hlsl_b903e43ffb3da915.bin (spirv-cross, mechanically renamed;
// _NNN names are SPIR-V ids). Behaviour-exact; readability pass pending.
static float _246;
static uint _247;
static const float4 _256[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
static const Reservoir1sppStreamState _288 = { 0.0f, 0.0f };
static const Reservoir1spp _289 = { 0.0f, 0u, 0.0f, 0.0f };
float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}
float4 ps_main(VsToPs input) : SV_Target0
{
    float4 out_var_SV_TARGET;
    int2 _313 = int2(input.position.xy);
    uint2 _314 = uint2(_313);
    float4 _329 = ((blue_noise_texture.Load(int3((_314 + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint4 _331 = gbuffer_tex.Load(int3(_314, 0u));
    float4 _333 = depth_tex.Load(int3(_314, 0u));
    float _334 = _333.x;
    uint _335 = _331.x;
    float _350 = float((_335 >> 24u) & 255u) * 0.0039215688593685626983642578125f;
    uint _352 = _331.y;
    float _367 = float((_352 >> 24u) & 255u) * 0.0039215688593685626983642578125f;
    float3 _369 = float4(float(_352 & 255u) * 0.0039215688593685626983642578125f, float((_352 >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((_352 >> 16u) & 255u) * 0.0039215688593685626983642578125f, _367).xyz;
    float3 _370 = _369 * _369;
    float3 _374 = normalize((float4(float(_335 & 255u) * 0.0039215688593685626983642578125f, float((_335 >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((_335 >> 16u) & 255u) * 0.0039215688593685626983642578125f, _350).xyz * 2.0f) - 1.0f.xxx);
    float2 _377 = (float2(_350, _367) * 2.0f) - 1.0f.xx;
    float _378 = _377.x;
    float _383 = (1.0f - abs(_378)) - abs(_377.y);
    float3 _384 = float3(_378, _377.y, _383);
    float2 _387 = _384.xy;
    float2 _392 = _387 - (((step(0.0f.xx, _387) * 2.0f) - 1.0f.xx) * clamp(-_383, 0.0f, 1.0f));
    float3 _394 = normalize(float3(_392.x, _392.y, _384.z));
    float3 _398 = mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float2 _401 = float2(depth_tex_dims);
    float2 _404 = (float2(_313) + 0.5f.xx) / _401;
    float2 _410 = (_404 - 0.5f.xx) * float2(2.0f, -2.0f);
    float _411 = _410.x;
    float4 _416 = float4(_411, _410.y, _334, 1.0f);
    float4 _418 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, _416));
    float3 _421 = -normalize(mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(_411, _410.y, 0.0f, 1.0f))).xyz);
    float3 _425 = _418.xyz / _418.w.xxx;
    bool3 _440 = ((render_overrides & 1u) != 0u).xxx;
    float3 _441 = float3(_440.x ? 0.5f.xxx.x : _370.x, _440.y ? 0.5f.xxx.y : _370.y, _440.z ? 0.5f.xxx.z : _370.z);
    float _443 = max(0.0f, dot(_374, _398));
    float _444 = dot(_394, _398);
    float3 _453 = (sun_color.xyz * 1.0f) * ((1.0f - screen_shadows_tex.Load(int3(_314, 0u)).x) * smoothstep(0.0f, 0.5f, _443));
    float3 _506;
    do
    {
        bool _463;
        if (!(_444 <= 0.0f))
        {
            _463 = dot(_394, _421) <= 0.0f;
        }
        else
        {
            _463 = true;
        }
        if (_463)
        {
            _506 = 0.0f.xxx;
            break;
        }
        float3 _467 = normalize(_421 + _398);
        float _468 = dot(_394, _467);
        float _471 = max(0.0f, 1.0f - dot(_467, _398));
        float _472 = _471 * _471;
        float _477 = dot(_394, _421);
        float _497 = ((_468 * _468) * (-0.9084937572479248046875f)) + 1.0f;
        _506 = ((lerp(0.039999999105930328369140625f.xxx, 1.0f.xxx, ((_472 * _472) * _471).xxx) * (((2.0f * _444) * _477) / ((_444 * sqrt((((_477 * (-0.09150625765323638916015625f)) + _477) * _477) + 0.09150625765323638916015625f)) + (_477 * sqrt((((_444 * (-0.09150625765323638916015625f)) + _444) * _444) + 0.09150625765323638916015625f))))) * (0.09150625765323638916015625f / ((3.1415927410125732421875f * _497) * _497))) / ((4.0f * _477) * _444).xxx;
        break;
    } while(false);
    float3 _510 = ((_453 * _443) * _441) * 0.3183098733425140380859375f.xxx;
    float _516 = 1.0f - clamp(dot(_374, _421), 0.0f, 1.0f);
    float _517 = _516 * _516;
    float _520 = 1.0f - clamp(dot(_398, _421), 0.0f, 1.0f);
    float3 _552;
    do
    {
        bool _545;
        if (!(abs(_425.x) > 65.0f))
        {
            _545 = abs(_425.z) > 65.0f;
        }
        else
        {
            _545 = true;
        }
        if (_545)
        {
            _552 = 0.0f.xxx;
            break;
        }
        _552 = color_library.terrain_highlight.xyz * highlight_active_hill.SampleLevel(sampler_default, (_425.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x;
        break;
    } while(false);
    float4 _307[3] = _256;
    float _554 = _329.x;
    uint _558 = uint((_554 * 3.0f) + 0.5f) & 3u;
    float _560;
    _560 = 0.0f;
    float _561;
    for (uint _563 = 0u; _563 < 3u; _560 = _561, _563++)
    {
        uint2 _575 = (_314 / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(_558 * 16u) + _563].xy);
        float _585 = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(_575, 0u)).x / _334))));
        for (uint _587 = 0u; _587 < 3u; )
        {
            _307[_587] += (indirect_radiance_tex.Load(int3(_575 + uint2((_587 * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * _585);
            _587++;
            continue;
        }
        _561 = _560 + _585;
    }
    float _652;
    if (_560 < 0.001000000047497451305389404296875f)
    {
        float _608;
        _608 = _560;
        float _609;
        for (uint _611 = 3u; _611 < 6u; _608 = _609, _611++)
        {
            uint2 _623 = (_314 / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(_558 * 16u) + _611].xy);
            float _633 = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(_623, 0u)).x / _334))));
            for (uint _635 = 0u; _635 < 3u; )
            {
                _307[_635] += (indirect_radiance_tex.Load(int3(_623 + uint2((_635 * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * _633);
                _635++;
                continue;
            }
            _609 = _608 + _633;
        }
        _652 = _608;
    }
    else
    {
        _652 = _560;
    }
    float _654 = 1.0f / max(9.9999997473787516355514526367188e-05f, _652);
    _307[0] *= _654;
    _307[1] *= _654;
    _307[2] *= _654;
    float3 _671 = float3(_307[0].x, _307[1].x, _307[2].x);
    float3 _675 = _307[0].yzw / max(9.9999996826552253889678874634872e-21f, _307[0].x).xxx;
    float3 _679 = _307[1].yzw / max(9.9999996826552253889678874634872e-21f, _307[1].x).xxx;
    float3 _683 = _307[2].yzw / max(9.9999996826552253889678874634872e-21f, _307[2].x).xxx;
    float3 _689 = max(0.0f.xxx, 1.0f.xxx - (color_library.indirect_light_tint.xyz * 0.920000016689300537109375f));
    float3 _697 = max(0.0f.xxx, lerp((_689 / dot(_689, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx) * dot(_671, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), _671, 1.16999995708465576171875f.xxx) * 1.2000000476837158203125f);
    float3 _703 = (-float3(_675.y, _679.y, _683.y)) * _697;
    float3 _708 = float3(_675.z, _679.z, _683.z) * _697;
    float3 _714 = (-float3(_675.x, _679.x, _683.x)) * _697;
    float3 _721 = normalize(float3(-dot(_714, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), -dot(_703, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), dot(_708, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))));
    float3 _747 = float3(abs(dot(float3(-_714.x, -_703.x, _708.x), _721)), abs(dot(float3(-_714.y, -_703.y, _708.y), _721)), abs(dot(float3(-_714.z, -_703.z, _708.z), _721))) / _697;
    float _753 = dot(_721, _374);
    float3 _777 = max(0.0f.xxx, ((((0.886227548122406005859375f.xxx * _697) + ((1.0f.xxx * (_374.y * (-1.02332770824432373046875f))) * _703)) + ((1.0f.xxx * (_374.z * 1.02332770824432373046875f)) * _708)) + ((1.0f.xxx * (_374.x * (-1.02332770824432373046875f))) * _714)) + (((_697 * ((_747 * 0.07999999821186065673828125f) + ((_747 * 0.60000002384185791015625f) * _747))) * 0.25f) * (0.3153915703296661376953125f * (((3.0f * _753) * _753) - 1.0f))));
    float _778 = lerp(clamp(0.100000001490116119384765625f + ssgi_tex.SampleLevel(sampler_nnc, _404, 0.0f).x, 0.0f, 1.0f), 1.0f, 0.5f);
    float4 _786 = brdf_fg_lut.SampleLevel(sampler_lnc, (float2(dot(_394, _421), 0.550000011920928955078125f) * 0.984375f.xx) + 0.0078125f.xx, 0.0f);
    float _787 = _786.x;
    float _789 = _786.y;
    float3 _791 = (0.0199999995529651641845703125f.xxx * _787) + _789.xxx;
    float _792 = _787 + _789;
    float3 _797 = lerp(_791 / _792.xxx, 1.0f.xxx, 0.4000000059604644775390625f.xxx) * (1.0f - _792);
    float3 _801 = _791 * (1.0f.xxx + (_797 / (1.0f.xxx - _797)));
    float3 _810 = (((((_453 * max(0.0f, _444)) * _506) + _510) + (_510 * (((_517 * _517) * _516) * ((_520 * _520) * _520)))) + _552) + (((((1.0f.xxx - _801) * _778) * _441) * _777) + (((_777 * 1.0f) * _801) * _778));
    float3 _2281;
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 _821 = uint3(clamp(floor((_425 * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint _828 = (32u * ((32u * _821.y) + _821.z)) + _821.x;
        bool _833 = WaveActiveAllTrue((light_indirection_grid.Load<uint>(_828 * 4 + 0) >> 20u) <= 2u);
        uint _834 = _833 ? 2u : 6u;
        float3 _2280;
        if (6u == _834)
        {
            uint _308[8];
            for (uint _1385 = 0u; _1385 < 8u; )
            {
                _308[_1385] = 4294967295u;
                _1385++;
                continue;
            }
            uint2 _1392 = uint2(_313 & int2(1, 1));
            uint _1397 = (_1392.x + (_1392.y * 2u)) & 3u;
            uint _1399;
            _1399 = 0u;
            uint _1400;
            uint _1402 = 0u;
            for (;;)
            {
                bool _1410;
                if (_1402 < 8u)
                {
                    _1410 = _1399 < 8u;
                }
                else
                {
                    _1410 = false;
                }
                if (_1410)
                {
                    uint4 _1422 = rdi_reservoir_tex.Load(int3(uint2((_313 / int2(2, 2)) + spatial_resolve_offsets_array[((_1397 * 16u) + _1402) + 256u].xy), 0u));
                    uint2 _1423 = _1422.xy;
                    uint _1424 = _1422.x;
                    bool _1430;
                    if (any(bool2(_1423.x != uint2(0u, 0u).x, _1423.y != uint2(0u, 0u).y)))
                    {
                        _1430 = _1424 < lantern_count;
                    }
                    else
                    {
                        _1430 = false;
                    }
                    bool _1436;
                    if (_1430)
                    {
                        _1436 = _308[0u] != _1424;
                    }
                    else
                    {
                        _1436 = false;
                    }
                    bool _1442;
                    if (_1436)
                    {
                        _1442 = _308[1u] != _1424;
                    }
                    else
                    {
                        _1442 = false;
                    }
                    bool _1448;
                    if (_1442)
                    {
                        _1448 = _308[2u] != _1424;
                    }
                    else
                    {
                        _1448 = false;
                    }
                    bool _1454;
                    if (_1448)
                    {
                        _1454 = _308[3u] != _1424;
                    }
                    else
                    {
                        _1454 = false;
                    }
                    bool _1460;
                    if (_1454)
                    {
                        _1460 = _308[4u] != _1424;
                    }
                    else
                    {
                        _1460 = false;
                    }
                    bool _1466;
                    if (_1460)
                    {
                        _1466 = _308[5u] != _1424;
                    }
                    else
                    {
                        _1466 = false;
                    }
                    bool _1472;
                    if (_1466)
                    {
                        _1472 = _308[6u] != _1424;
                    }
                    else
                    {
                        _1472 = false;
                    }
                    bool _1478;
                    if (_1472)
                    {
                        _1478 = _308[7u] != _1424;
                    }
                    else
                    {
                        _1478 = false;
                    }
                    if (_1478)
                    {
                        _308[_1399] = _1424;
                        _1400 = _1399 + 1u;
                    }
                    else
                    {
                        _1400 = _1399;
                    }
                    _1399 = _1400;
                    _1402++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint _1484;
            _1484 = _1399;
            uint _1485;
            for (uint _1487 = 0u; _1487 < 8u; _1484 = _1485, _1487++)
            {
                uint _1494 = QuadReadAcrossDiagonal(_308[_1487]);
                bool _1499;
                if (_1494 < lantern_count)
                {
                    _1499 = _1484 < 8u;
                }
                else
                {
                    _1499 = false;
                }
                if (_1499)
                {
                    bool _1510;
                    if (_308[0u] != _1494)
                    {
                        _1510 = _308[1u] != _1494;
                    }
                    else
                    {
                        _1510 = false;
                    }
                    bool _1516;
                    if (_1510)
                    {
                        _1516 = _308[2u] != _1494;
                    }
                    else
                    {
                        _1516 = false;
                    }
                    bool _1522;
                    if (_1516)
                    {
                        _1522 = _308[3u] != _1494;
                    }
                    else
                    {
                        _1522 = false;
                    }
                    bool _1528;
                    if (_1522)
                    {
                        _1528 = _308[4u] != _1494;
                    }
                    else
                    {
                        _1528 = false;
                    }
                    bool _1534;
                    if (_1528)
                    {
                        _1534 = _308[5u] != _1494;
                    }
                    else
                    {
                        _1534 = false;
                    }
                    bool _1540;
                    if (_1534)
                    {
                        _1540 = _308[6u] != _1494;
                    }
                    else
                    {
                        _1540 = false;
                    }
                    bool _1546;
                    if (_1540)
                    {
                        _1546 = _308[7u] != _1494;
                    }
                    else
                    {
                        _1546 = false;
                    }
                    uint _1551;
                    if (_1546)
                    {
                        _308[_1484] = _1494;
                        _1551 = _1484 + 1u;
                    }
                    else
                    {
                        _1551 = _1484;
                    }
                    _1485 = _1551;
                }
                else
                {
                    _1485 = _1484;
                }
            }
            Reservoir1sppStreamState _309[2];
            Reservoir1spp _310[2];
            for (uint _1553 = 0u; _1553 < 2u; )
            {
                _309[_1553] = _288;
                _310[_1553] = _289;
                _1553++;
                continue;
            }
            for (uint _1561 = 0u; _1561 < _1484; _1561++)
            {
                float2 _1574 = spvUnpackHalf2x16(lantern_buf.Load<uint>(_308[_1561] * 16 + 12) & 65535u);
                float _1575 = _1574.x;
                uint _1577 = lantern_buf.Load<uint>(_308[_1561] * 16 + 12) >> 16u;
                float3 _1578 = lantern_buf.Load<float3>(_308[_1561] * 16 + 0) - _425;
                float _1580 = dot(_1578, _1578);
                float3 _1587 = _1578 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                float3 _1632;
                do
                {
                    if (_1577 == 0u)
                    {
                        _1632 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                        break;
                    }
                    else
                    {
                        if (_1577 == 1u)
                        {
                            _1632 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                            break;
                        }
                        else
                        {
                            if (_1577 == 2u)
                            {
                                _1632 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                break;
                            }
                            else
                            {
                                if (_1577 == 3u)
                                {
                                    _1632 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                    break;
                                }
                                else
                                {
                                    _1632 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                float _1636 = dot(((_1632 * 4.0f) * _1575) * ((max(0.0f, dot(_374, _1578 / length(_1578).xxx)) * ((1.0f / (0.3359999954700469970703125f + _1580)) * smoothstep(36.0f * _1575, 16.0f * _1575, _1580))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_1587 / length(_1587).xxx).y)), float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float _1637 = float(_1484);
                float _1641 = frac(_554 + (float(_1561) / _1637));
                if (_310[0].w_sum <= _310[1].w_sum)
                {
                    do
                    {
                        _309[0].M_sum += 1.0f;
                        float _1680 = _1636 * _1637;
                        bool _1695;
                        do
                        {
                            _310[0].w_sum += _1680;
                            _310[0].M += 1.0f;
                            if ((_1680 / _310[0].w_sum) >= _1641)
                            {
                                _310[0].payload = _308[_1561];
                                _1695 = true;
                                break;
                            }
                            else
                            {
                                _1695 = false;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        if (_1695)
                        {
                            _309[0].p_q_sel = _1636;
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
                        _309[1].M_sum += 1.0f;
                        float _1655 = _1636 * _1637;
                        bool _1670;
                        do
                        {
                            _310[1].w_sum += _1655;
                            _310[1].M += 1.0f;
                            if ((_1655 / _310[1].w_sum) >= _1641)
                            {
                                _310[1].payload = _308[_1561];
                                _1670 = true;
                                break;
                            }
                            else
                            {
                                _1670 = false;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        if (_1670)
                        {
                            _309[1].p_q_sel = _1636;
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
            for (uint _1701 = 1u; _1701 < 2u; )
            {
                _309[0].M_sum += _309[_1701].M_sum;
                _1701++;
                continue;
            }
            for (uint _1712 = 1u; _1712 < 2u; )
            {
                _309[_1712].M_sum = _309[0].M_sum;
                _1712++;
                continue;
            }
            float3 _1728;
            _1728 = _810;
            float _1722;
            float _1725;
            float _1727;
            float3 _1729;
            uint _1733;
            uint _1735;
            bool _1737;
            bool _1739;
            float _1721 = 0.0f;
            float _1724 = 0.0f;
            float _1726 = 0.0f;
            uint _1730 = 0u;
            uint _1732 = 0u;
            uint _1734 = 0u;
            bool _1736 = false;
            bool _1738 = false;
            for (; _1730 < 2u; _1721 = _1722, _1724 = _1725, _1726 = _1727, _1728 = _1729, _1730++, _1732 = _1733, _1734 = _1735, _1736 = _1737, _1738 = _1739)
            {
                _310[_1730].M = _309[_1730].M_sum;
                _310[_1730].W = _310[_1730].w_sum / max(9.9999999392252902907785028219223e-09f, _310[_1730].M * _309[_1730].p_q_sel);
                if (_310[_1730].W > 0.0f)
                {
                    float2 _1766 = spvUnpackHalf2x16(lantern_buf.Load<uint>(_310[_1730].payload * 16 + 12) & 65535u);
                    float _1767 = _1766.x;
                    uint _1769 = lantern_buf.Load<uint>(_310[_1730].payload * 16 + 12) >> 16u;
                    float3 _1770 = lantern_buf.Load<float3>(_310[_1730].payload * 16 + 0) - _425;
                    float _1771 = length(_1770);
                    float _1772 = dot(_1770, _1770);
                    float3 _1779 = _1770 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 _1785 = _1771.xxx;
                    float3 _1824;
                    do
                    {
                        if (_1769 == 0u)
                        {
                            _1824 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (_1769 == 1u)
                            {
                                _1824 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (_1769 == 2u)
                                {
                                    _1824 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (_1769 == 3u)
                                    {
                                        _1824 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        _1824 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 _1827 = ((_1824 * 4.0f) * _1767) * ((max(0.0f, dot(_374, _1770 / _1785)) * ((1.0f / (0.3359999954700469970703125f + _1772)) * smoothstep(36.0f * _1767, 16.0f * _1767, _1772))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_1779 / length(_1779).xxx).y));
                    float _2271;
                    float _2272;
                    float _2273;
                    uint _2274;
                    uint _2275;
                    bool _2276;
                    bool _2277;
                    float3 _2278;
                    if ((_310[_1730].W * max(max(_1827.x, _1827.y), _1827.z)) > 0.0f)
                    {
                        float _2086;
                        float _2087;
                        float _2088;
                        float3 _2089;
                        if (_1771 > 0.5f)
                        {
                            float3 _1847 = (_425 + ((_1770 * (_1771 - 0.5f)) / _1785)) + (_374 * (0.0199999995529651641845703125f * _329.y));
                            float3 _1848 = _416.xyz;
                            float4 _1858 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(_1847, 1.0f)));
                            float3 _1862 = _1858.xyz / _1858.w.xxx;
                            float4 _1873 = _416 + (float4(_1862 - _1848, 0.0f) * float(int(sign(_1862.z))));
                            float _1874 = _1873.w;
                            float3 _1883 = (_1873 / (float((_1874 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_1874))).xxxx).xyz;
                            float3 _1884 = _1883 - _1848;
                            bool3 _1885 = bool3(_1884.x < 0.0f.xxx.x, _1884.y < 0.0f.xxx.y, _1884.z < 0.0f.xxx.z);
                            float3 _1888 = (float3(_1885.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1885.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1885.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1848) / _1884;
                            float3 _1894 = _1848 + (_1884 * max(0.0f, max(_1888.x, _1888.y)));
                            float3 _1895 = _1883 - _1894;
                            bool3 _1896 = bool3(_1895.x >= 0.0f.xxx.x, _1895.y >= 0.0f.xxx.y, _1895.z >= 0.0f.xxx.z);
                            float3 _1899 = (float3(_1896.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1896.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1896.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1894) / _1895;
                            float3 _1907 = _1894 + (_1895 * min(1.0f, min(min(_1899.x, _1899.y), _1899.z)));
                            float _2038;
                            float _2039;
                            float _2040;
                            bool _2053;
                            float _2054;
                            float _2055;
                            do
                            {
                                float _1928 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint _1929 = uint(int(max(2u, min(4u, uint(int(floor(length((((_1907.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_1894.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _401))))))));
                                float _2033;
                                float _2037;
                                bool _2041;
                                do
                                {
                                    float3 _1934 = _1907 - _1894;
                                    float _1942;
                                    if (_1907.z > 9.5367431640625e-07f)
                                    {
                                        _1942 = float(_1929);
                                    }
                                    else
                                    {
                                        _1942 = max(1.0f, float(_1929) - 1.0f);
                                    }
                                    float _1943 = 1.0f / _1942;
                                    float _2028;
                                    float _2029;
                                    float _2030;
                                    float _2031;
                                    bool _2032;
                                    if (_1929 > 0u)
                                    {
                                        float _1949 = lerp(0.0f, 1.0f, pow(_554 * _1943, 1.0f));
                                        float3 _1951 = _1894 + (_1934 * _1949);
                                        float2 _1954 = (_1951.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float _1956 = 1.0f / _1951.z;
                                        float _1961 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1954, 0.0f).x;
                                        float _1965 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1954, 0.0f).x;
                                        float _1969 = (max(_1961, _1965) * 1.00000202655792236328125f) - _1956;
                                        float _1970 = _1956 - min(_1961, _1965);
                                        bool _1973 = (_1969 < 0.0f) ? (_1970 < _1928) : false;
                                        float _2022;
                                        float _2023;
                                        float _2024;
                                        float _2025;
                                        bool _2026;
                                        float _2027;
                                        if (_1973)
                                        {
                                            _2022 = _1969;
                                            _2023 = _1970;
                                            _2024 = _1928;
                                            _2025 = _1949;
                                            _2026 = _1973;
                                            _2027 = 0.0f;
                                        }
                                        else
                                        {
                                            float _1983;
                                            _1983 = _1949;
                                            bool _1982;
                                            float _1984;
                                            float _2017;
                                            float _2018;
                                            float _2019;
                                            float _2020;
                                            bool _2021;
                                            uint _1978 = 1u;
                                            bool _1981 = _1973;
                                            for (;;)
                                            {
                                                if (_1978 < _1929)
                                                {
                                                    _1984 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1978) + _554) * _1943, 1.0f)));
                                                    float3 _1994 = _1894 + (_1934 * _1984);
                                                    float2 _1997 = (_1994.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float _1999 = 1.0f / _1994.z;
                                                    float _2003 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1997, 0.0f).x;
                                                    float _2007 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1997, 0.0f).x;
                                                    float _2011 = (max(_2003, _2007) * 1.00000202655792236328125f) - _1999;
                                                    float _2012 = _1999 - min(_2003, _2007);
                                                    _1982 = (_2011 < 0.0f) ? (_2012 < _1928) : false;
                                                    if (_1982)
                                                    {
                                                        _2017 = _2011;
                                                        _2018 = _2012;
                                                        _2019 = _1928;
                                                        _2020 = _1984;
                                                        _2021 = _1982;
                                                        break;
                                                    }
                                                    _1978++;
                                                    _1981 = _1982;
                                                    _1983 = _1984;
                                                    continue;
                                                }
                                                else
                                                {
                                                    _2017 = 0.0f;
                                                    _2018 = 0.0f;
                                                    _2019 = 0.0f;
                                                    _2020 = 1.0f;
                                                    _2021 = _1981;
                                                    break;
                                                }
                                            }
                                            _2022 = _2017;
                                            _2023 = _2018;
                                            _2024 = _2019;
                                            _2025 = _2020;
                                            _2026 = _2021;
                                            _2027 = _1983;
                                        }
                                        _2028 = _2022;
                                        _2029 = _2023;
                                        _2030 = _2024;
                                        _2031 = _2025;
                                        _2032 = _2026;
                                        _2033 = _2027;
                                    }
                                    else
                                    {
                                        _2028 = 0.0f;
                                        _2029 = 0.0f;
                                        _2030 = 0.0f;
                                        _2031 = 1.0f;
                                        _2032 = false;
                                        _2033 = 0.0f;
                                    }
                                    if (_2032)
                                    {
                                        _2037 = _2031;
                                        _2038 = _2028;
                                        _2039 = _2030;
                                        _2040 = _2029;
                                        _2041 = true;
                                        break;
                                    }
                                    else
                                    {
                                        _2037 = _2033;
                                        _2038 = _1721;
                                        _2039 = _1724;
                                        _2040 = _1726;
                                        _2041 = false;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                bool _2049;
                                if (_2041)
                                {
                                    bool _2048;
                                    if (_2040 < _2039)
                                    {
                                        _2048 = _2038 < _2039;
                                    }
                                    else
                                    {
                                        _2048 = false;
                                    }
                                    _2049 = _2048;
                                }
                                else
                                {
                                    _2049 = false;
                                }
                                if (_2049)
                                {
                                    _2053 = true;
                                    _2054 = _2037;
                                    _2055 = _2040 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                _2053 = false;
                                _2054 = _2033;
                                _2055 = 0.0f;
                                break;
                            } while(false);
                            float3 _2085;
                            if (_2053)
                            {
                                float4 _2065 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(_1894, _1907, _2054.xxx), 1.0f)));
                                float3 _2072 = ((_2065.xyz / _2065.w.xxx) + (_421 * _2055)) - lantern_buf.Load<float3>(_310[_1730].payload * 16 + 0);
                                bool _2082;
                                if (!(abs(_2072.y) > 0.5f))
                                {
                                    float2 _2079 = _2072.xz;
                                    _2082 = dot(_2079, _2079) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    _2082 = true;
                                }
                                bool3 _2083 = _2082.xxx;
                                _2085 = float3(_2083.x ? 0.0f.xxx.x : _1827.x, _2083.y ? 0.0f.xxx.y : _1827.y, _2083.z ? 0.0f.xxx.z : _1827.z);
                            }
                            else
                            {
                                _2085 = _1827;
                            }
                            _2086 = _2038;
                            _2087 = _2039;
                            _2088 = _2040;
                            _2089 = _2085;
                        }
                        else
                        {
                            _2086 = _1721;
                            _2087 = _1724;
                            _2088 = _1726;
                            _2089 = _1827;
                        }
                        float2 _2099 = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(_310[_1730].payload * 10u));
                        int2 _2101 = int2(floor(_2099));
                        float2 _2102 = frac(_2099);
                        uint2 _2103 = uint2(_2101);
                        uint _2105 = _2103.y;
                        uint _2107 = _2105 + (_2105 << 10u);
                        uint _2109 = _2107 ^ (_2107 >> 6u);
                        uint _2111 = _2109 + (_2109 << 3u);
                        uint _2113 = _2111 ^ (_2111 >> 11u);
                        uint _2119 = (((_2103.x * 1664525u) + (_2113 + (_2113 << 15u))) + 1013904223u) * 1664525u;
                        uint _2121 = _2119 ^ (_2119 >> 11u);
                        uint _2124 = _2121 ^ ((_2121 << 7u) & 2636928640u);
                        uint _2127 = _2124 ^ ((_2124 << 15u) & 4022730752u);
                        bool _2130 = _1738 ? _1738 : true;
                        uint _2131 = _1738 ? _1734 : 8388607u;
                        bool _2132 = _1736 ? _1736 : true;
                        uint _2133 = _1736 ? _1732 : 1065353216u;
                        uint2 _2139 = uint2(_2101 + int2(1, 0));
                        uint _2141 = _2139.y;
                        uint _2143 = _2141 + (_2141 << 10u);
                        uint _2145 = _2143 ^ (_2143 >> 6u);
                        uint _2147 = _2145 + (_2145 << 3u);
                        uint _2149 = _2147 ^ (_2147 >> 11u);
                        uint _2155 = (((_2139.x * 1664525u) + (_2149 + (_2149 << 15u))) + 1013904223u) * 1664525u;
                        uint _2157 = _2155 ^ (_2155 >> 11u);
                        uint _2160 = _2157 ^ ((_2157 << 7u) & 2636928640u);
                        uint _2163 = _2160 ^ ((_2160 << 15u) & 4022730752u);
                        bool _2166 = _2130 ? _2130 : true;
                        uint _2167 = _2130 ? _2131 : 8388607u;
                        bool _2168 = _2132 ? _2132 : true;
                        uint _2169 = _2132 ? _2133 : 1065353216u;
                        uint2 _2175 = uint2(_2101 + int2(0, 1));
                        uint _2177 = _2175.y;
                        uint _2179 = _2177 + (_2177 << 10u);
                        uint _2181 = _2179 ^ (_2179 >> 6u);
                        uint _2183 = _2181 + (_2181 << 3u);
                        uint _2185 = _2183 ^ (_2183 >> 11u);
                        uint _2191 = (((_2175.x * 1664525u) + (_2185 + (_2185 << 15u))) + 1013904223u) * 1664525u;
                        uint _2193 = _2191 ^ (_2191 >> 11u);
                        uint _2196 = _2193 ^ ((_2193 << 7u) & 2636928640u);
                        uint _2199 = _2196 ^ ((_2196 << 15u) & 4022730752u);
                        bool _2202 = _2166 ? _2166 : true;
                        uint _2203 = _2166 ? _2167 : 8388607u;
                        bool _2204 = _2168 ? _2168 : true;
                        uint _2205 = _2168 ? _2169 : 1065353216u;
                        uint2 _2211 = uint2(_2101 + int2(1, 1));
                        uint _2213 = _2211.y;
                        uint _2215 = _2213 + (_2213 << 10u);
                        uint _2217 = _2215 ^ (_2215 >> 6u);
                        uint _2219 = _2217 + (_2217 << 3u);
                        uint _2221 = _2219 ^ (_2219 >> 11u);
                        uint _2227 = (((_2211.x * 1664525u) + (_2221 + (_2221 << 15u))) + 1013904223u) * 1664525u;
                        uint _2229 = _2227 ^ (_2227 >> 11u);
                        uint _2232 = _2229 ^ ((_2229 << 7u) & 2636928640u);
                        uint _2235 = _2232 ^ ((_2232 << 15u) & 4022730752u);
                        uint _2239 = _2202 ? _2203 : 8388607u;
                        uint _2241 = _2204 ? _2205 : 1065353216u;
                        float2 _2249 = (_2102 * _2102) * (3.0f.xx - (_2102 * 2.0f));
                        float _2250 = _2249.x;
                        float _2251 = 1.0f - _2250;
                        float _2253 = _2249.y;
                        _2271 = _2086;
                        _2272 = _2087;
                        _2273 = _2088;
                        _2274 = _2241;
                        _2275 = _2239;
                        _2276 = _2204 ? _2204 : true;
                        _2277 = _2202 ? _2202 : true;
                        _2278 = (((_2089 * lerp(0.5f, 1.0f, (((1.0f - _2253) * (((asfloat(((_2127 ^ (_2127 >> 18u)) & _2131) | _2133) - 1.0f) * _2251) + ((asfloat(((_2163 ^ (_2163 >> 18u)) & _2167) | _2169) - 1.0f) * _2250))) + (((asfloat(((_2199 ^ (_2199 >> 18u)) & _2203) | _2205) - 1.0f) * _2251) * _2253)) + (((asfloat(((_2235 ^ (_2235 >> 18u)) & _2239) | _2241) - 1.0f) * _2250) * _2253))) * _310[_1730].W) * _441) * ev_mult;
                    }
                    else
                    {
                        _2271 = _1721;
                        _2272 = _1724;
                        _2273 = _1726;
                        _2274 = _1732;
                        _2275 = _1734;
                        _2276 = _1736;
                        _2277 = _1738;
                        _2278 = 0.0f.xxx;
                    }
                    _1722 = _2271;
                    _1725 = _2272;
                    _1727 = _2273;
                    _1729 = _1728 + _2278;
                    _1733 = _2274;
                    _1735 = _2275;
                    _1737 = _2276;
                    _1739 = _2277;
                }
                else
                {
                    _1722 = _1721;
                    _1725 = _1724;
                    _1727 = _1726;
                    _1729 = _1728;
                    _1733 = _1732;
                    _1735 = _1734;
                    _1737 = _1736;
                    _1739 = _1738;
                }
            }
            _2280 = _1728;
        }
        else
        {
            float3 _1383;
            if (2u == _834)
            {
                uint _844 = light_indirection_grid.Load<uint>(_828 * 4 + 0) & 1048575u;
                uint _846 = min(64u, (light_indirection_grid.Load<uint>(_828 * 4 + 0) >> 20u));
                float3 _848;
                _848 = _810;
                float3 _849;
                float _852;
                float _854;
                float _856;
                uint _860;
                uint _862;
                bool _864;
                bool _866;
                float _851 = 0.0f;
                float _853 = 0.0f;
                float _855 = 0.0f;
                uint _857 = 0u;
                uint _859 = 0u;
                uint _861 = 0u;
                bool _863 = false;
                bool _865 = false;
                for (; _857 < _846; _848 = _849, _851 = _852, _853 = _854, _855 = _856, _857++, _859 = _860, _861 = _862, _863 = _864, _865 = _866)
                {
                    uint _870 = _844 + _857;
                    float2 _878 = spvUnpackHalf2x16(lantern_buf.Load<uint>(grid_light_indices.Load<uint>(_870 * 4 + 0) * 16 + 12) & 65535u);
                    float _879 = _878.x;
                    uint _881 = lantern_buf.Load<uint>(grid_light_indices.Load<uint>(_870 * 4 + 0) * 16 + 12) >> 16u;
                    float3 _882 = lantern_buf.Load<float3>(grid_light_indices.Load<uint>(_870 * 4 + 0) * 16 + 0) - _425;
                    float _883 = length(_882);
                    float _884 = dot(_882, _882);
                    float3 _891 = _882 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 _897 = _883.xxx;
                    float3 _936;
                    do
                    {
                        if (_881 == 0u)
                        {
                            _936 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (_881 == 1u)
                            {
                                _936 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (_881 == 2u)
                                {
                                    _936 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (_881 == 3u)
                                    {
                                        _936 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        _936 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 _939 = ((_936 * 4.0f) * _879) * ((max(0.0f, dot(_374, _882 / _897)) * ((1.0f / (0.3359999954700469970703125f + _884)) * smoothstep(36.0f * _879, 16.0f * _879, _884))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (_891 / length(_891).xxx).y));
                    float3 _1382;
                    if (max(max(_939.x, _939.y), _939.z) > 0.0f)
                    {
                        float _1197;
                        float _1198;
                        float _1199;
                        float3 _1200;
                        if (_883 > 0.5f)
                        {
                            float3 _958 = (_425 + ((_882 * (_883 - 0.5f)) / _897)) + (_374 * (0.0199999995529651641845703125f * _329.y));
                            float3 _959 = _416.xyz;
                            float4 _969 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(_958, 1.0f)));
                            float3 _973 = _969.xyz / _969.w.xxx;
                            float4 _984 = _416 + (float4(_973 - _959, 0.0f) * float(int(sign(_973.z))));
                            float _985 = _984.w;
                            float3 _994 = (_984 / (float((_985 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_985))).xxxx).xyz;
                            float3 _995 = _994 - _959;
                            bool3 _996 = bool3(_995.x < 0.0f.xxx.x, _995.y < 0.0f.xxx.y, _995.z < 0.0f.xxx.z);
                            float3 _999 = (float3(_996.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _996.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _996.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _959) / _995;
                            float3 _1005 = _959 + (_995 * max(0.0f, max(_999.x, _999.y)));
                            float3 _1006 = _994 - _1005;
                            bool3 _1007 = bool3(_1006.x >= 0.0f.xxx.x, _1006.y >= 0.0f.xxx.y, _1006.z >= 0.0f.xxx.z);
                            float3 _1010 = (float3(_1007.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1007.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1007.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1005) / _1006;
                            float3 _1018 = _1005 + (_1006 * min(1.0f, min(min(_1010.x, _1010.y), _1010.z)));
                            float _1149;
                            float _1150;
                            float _1151;
                            bool _1164;
                            float _1165;
                            float _1166;
                            do
                            {
                                float _1039 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint _1040 = uint(int(max(2u, min(4u, uint(int(floor(length((((_1018.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_1005.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _401))))))));
                                float _1144;
                                float _1148;
                                bool _1152;
                                do
                                {
                                    float3 _1045 = _1018 - _1005;
                                    float _1053;
                                    if (_1018.z > 9.5367431640625e-07f)
                                    {
                                        _1053 = float(_1040);
                                    }
                                    else
                                    {
                                        _1053 = max(1.0f, float(_1040) - 1.0f);
                                    }
                                    float _1054 = 1.0f / _1053;
                                    float _1139;
                                    float _1140;
                                    float _1141;
                                    float _1142;
                                    bool _1143;
                                    if (_1040 > 0u)
                                    {
                                        float _1060 = lerp(0.0f, 1.0f, pow(_554 * _1054, 1.0f));
                                        float3 _1062 = _1005 + (_1045 * _1060);
                                        float2 _1065 = (_1062.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float _1067 = 1.0f / _1062.z;
                                        float _1072 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1065, 0.0f).x;
                                        float _1076 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1065, 0.0f).x;
                                        float _1080 = (max(_1072, _1076) * 1.00000202655792236328125f) - _1067;
                                        float _1081 = _1067 - min(_1072, _1076);
                                        bool _1084 = (_1080 < 0.0f) ? (_1081 < _1039) : false;
                                        float _1133;
                                        float _1134;
                                        float _1135;
                                        float _1136;
                                        bool _1137;
                                        float _1138;
                                        if (_1084)
                                        {
                                            _1133 = _1080;
                                            _1134 = _1081;
                                            _1135 = _1039;
                                            _1136 = _1060;
                                            _1137 = _1084;
                                            _1138 = 0.0f;
                                        }
                                        else
                                        {
                                            float _1094;
                                            _1094 = _1060;
                                            bool _1093;
                                            float _1095;
                                            float _1128;
                                            float _1129;
                                            float _1130;
                                            float _1131;
                                            bool _1132;
                                            uint _1089 = 1u;
                                            bool _1092 = _1084;
                                            for (;;)
                                            {
                                                if (_1089 < _1040)
                                                {
                                                    _1095 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1089) + _554) * _1054, 1.0f)));
                                                    float3 _1105 = _1005 + (_1045 * _1095);
                                                    float2 _1108 = (_1105.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float _1110 = 1.0f / _1105.z;
                                                    float _1114 = 1.0f / depth_tex.SampleLevel(sampler_llc, _1108, 0.0f).x;
                                                    float _1118 = 1.0f / depth_tex.SampleLevel(sampler_nnc, _1108, 0.0f).x;
                                                    float _1122 = (max(_1114, _1118) * 1.00000202655792236328125f) - _1110;
                                                    float _1123 = _1110 - min(_1114, _1118);
                                                    _1093 = (_1122 < 0.0f) ? (_1123 < _1039) : false;
                                                    if (_1093)
                                                    {
                                                        _1128 = _1122;
                                                        _1129 = _1123;
                                                        _1130 = _1039;
                                                        _1131 = _1095;
                                                        _1132 = _1093;
                                                        break;
                                                    }
                                                    _1089++;
                                                    _1092 = _1093;
                                                    _1094 = _1095;
                                                    continue;
                                                }
                                                else
                                                {
                                                    _1128 = 0.0f;
                                                    _1129 = 0.0f;
                                                    _1130 = 0.0f;
                                                    _1131 = 1.0f;
                                                    _1132 = _1092;
                                                    break;
                                                }
                                            }
                                            _1133 = _1128;
                                            _1134 = _1129;
                                            _1135 = _1130;
                                            _1136 = _1131;
                                            _1137 = _1132;
                                            _1138 = _1094;
                                        }
                                        _1139 = _1133;
                                        _1140 = _1134;
                                        _1141 = _1135;
                                        _1142 = _1136;
                                        _1143 = _1137;
                                        _1144 = _1138;
                                    }
                                    else
                                    {
                                        _1139 = 0.0f;
                                        _1140 = 0.0f;
                                        _1141 = 0.0f;
                                        _1142 = 1.0f;
                                        _1143 = false;
                                        _1144 = 0.0f;
                                    }
                                    if (_1143)
                                    {
                                        _1148 = _1142;
                                        _1149 = _1139;
                                        _1150 = _1141;
                                        _1151 = _1140;
                                        _1152 = true;
                                        break;
                                    }
                                    else
                                    {
                                        _1148 = _1144;
                                        _1149 = _851;
                                        _1150 = _853;
                                        _1151 = _855;
                                        _1152 = false;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                bool _1160;
                                if (_1152)
                                {
                                    bool _1159;
                                    if (_1151 < _1150)
                                    {
                                        _1159 = _1149 < _1150;
                                    }
                                    else
                                    {
                                        _1159 = false;
                                    }
                                    _1160 = _1159;
                                }
                                else
                                {
                                    _1160 = false;
                                }
                                if (_1160)
                                {
                                    _1164 = true;
                                    _1165 = _1148;
                                    _1166 = _1151 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                _1164 = false;
                                _1165 = _1144;
                                _1166 = 0.0f;
                                break;
                            } while(false);
                            float3 _1196;
                            if (_1164)
                            {
                                float4 _1176 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(_1005, _1018, _1165.xxx), 1.0f)));
                                float3 _1183 = ((_1176.xyz / _1176.w.xxx) + (_421 * _1166)) - lantern_buf.Load<float3>(grid_light_indices.Load<uint>(_870 * 4 + 0) * 16 + 0);
                                bool _1193;
                                if (!(abs(_1183.y) > 0.5f))
                                {
                                    float2 _1190 = _1183.xz;
                                    _1193 = dot(_1190, _1190) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    _1193 = true;
                                }
                                bool3 _1194 = _1193.xxx;
                                _1196 = float3(_1194.x ? 0.0f.xxx.x : _939.x, _1194.y ? 0.0f.xxx.y : _939.y, _1194.z ? 0.0f.xxx.z : _939.z);
                            }
                            else
                            {
                                _1196 = _939;
                            }
                            _1197 = _1149;
                            _1198 = _1150;
                            _1199 = _1151;
                            _1200 = _1196;
                        }
                        else
                        {
                            _1197 = _851;
                            _1198 = _853;
                            _1199 = _855;
                            _1200 = _939;
                        }
                        float2 _1210 = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(grid_light_indices.Load<uint>(_870 * 4 + 0) * 10u));
                        int2 _1212 = int2(floor(_1210));
                        float2 _1213 = frac(_1210);
                        uint2 _1214 = uint2(_1212);
                        uint _1216 = _1214.y;
                        uint _1218 = _1216 + (_1216 << 10u);
                        uint _1220 = _1218 ^ (_1218 >> 6u);
                        uint _1222 = _1220 + (_1220 << 3u);
                        uint _1224 = _1222 ^ (_1222 >> 11u);
                        uint _1230 = (((_1214.x * 1664525u) + (_1224 + (_1224 << 15u))) + 1013904223u) * 1664525u;
                        uint _1232 = _1230 ^ (_1230 >> 11u);
                        uint _1235 = _1232 ^ ((_1232 << 7u) & 2636928640u);
                        uint _1238 = _1235 ^ ((_1235 << 15u) & 4022730752u);
                        bool _1241 = _865 ? _865 : true;
                        uint _1242 = _865 ? _861 : 8388607u;
                        bool _1243 = _863 ? _863 : true;
                        uint _1244 = _863 ? _859 : 1065353216u;
                        uint2 _1250 = uint2(_1212 + int2(1, 0));
                        uint _1252 = _1250.y;
                        uint _1254 = _1252 + (_1252 << 10u);
                        uint _1256 = _1254 ^ (_1254 >> 6u);
                        uint _1258 = _1256 + (_1256 << 3u);
                        uint _1260 = _1258 ^ (_1258 >> 11u);
                        uint _1266 = (((_1250.x * 1664525u) + (_1260 + (_1260 << 15u))) + 1013904223u) * 1664525u;
                        uint _1268 = _1266 ^ (_1266 >> 11u);
                        uint _1271 = _1268 ^ ((_1268 << 7u) & 2636928640u);
                        uint _1274 = _1271 ^ ((_1271 << 15u) & 4022730752u);
                        bool _1277 = _1241 ? _1241 : true;
                        uint _1278 = _1241 ? _1242 : 8388607u;
                        bool _1279 = _1243 ? _1243 : true;
                        uint _1280 = _1243 ? _1244 : 1065353216u;
                        uint2 _1286 = uint2(_1212 + int2(0, 1));
                        uint _1288 = _1286.y;
                        uint _1290 = _1288 + (_1288 << 10u);
                        uint _1292 = _1290 ^ (_1290 >> 6u);
                        uint _1294 = _1292 + (_1292 << 3u);
                        uint _1296 = _1294 ^ (_1294 >> 11u);
                        uint _1302 = (((_1286.x * 1664525u) + (_1296 + (_1296 << 15u))) + 1013904223u) * 1664525u;
                        uint _1304 = _1302 ^ (_1302 >> 11u);
                        uint _1307 = _1304 ^ ((_1304 << 7u) & 2636928640u);
                        uint _1310 = _1307 ^ ((_1307 << 15u) & 4022730752u);
                        bool _1313 = _1277 ? _1277 : true;
                        uint _1314 = _1277 ? _1278 : 8388607u;
                        bool _1315 = _1279 ? _1279 : true;
                        uint _1316 = _1279 ? _1280 : 1065353216u;
                        uint2 _1322 = uint2(_1212 + int2(1, 1));
                        uint _1324 = _1322.y;
                        uint _1326 = _1324 + (_1324 << 10u);
                        uint _1328 = _1326 ^ (_1326 >> 6u);
                        uint _1330 = _1328 + (_1328 << 3u);
                        uint _1332 = _1330 ^ (_1330 >> 11u);
                        uint _1338 = (((_1322.x * 1664525u) + (_1332 + (_1332 << 15u))) + 1013904223u) * 1664525u;
                        uint _1340 = _1338 ^ (_1338 >> 11u);
                        uint _1343 = _1340 ^ ((_1340 << 7u) & 2636928640u);
                        uint _1346 = _1343 ^ ((_1343 << 15u) & 4022730752u);
                        uint _1350 = _1313 ? _1314 : 8388607u;
                        uint _1352 = _1315 ? _1316 : 1065353216u;
                        float2 _1360 = (_1213 * _1213) * (3.0f.xx - (_1213 * 2.0f));
                        float _1361 = _1360.x;
                        float _1362 = 1.0f - _1361;
                        float _1364 = _1360.y;
                        _852 = _1197;
                        _854 = _1198;
                        _856 = _1199;
                        _860 = _1352;
                        _862 = _1350;
                        _864 = _1315 ? _1315 : true;
                        _866 = _1313 ? _1313 : true;
                        _1382 = (((_1200 * lerp(0.5f, 1.0f, (((1.0f - _1364) * (((asfloat(((_1238 ^ (_1238 >> 18u)) & _1242) | _1244) - 1.0f) * _1362) + ((asfloat(((_1274 ^ (_1274 >> 18u)) & _1278) | _1280) - 1.0f) * _1361))) + (((asfloat(((_1310 ^ (_1310 >> 18u)) & _1314) | _1316) - 1.0f) * _1362) * _1364)) + (((asfloat(((_1346 ^ (_1346 >> 18u)) & _1350) | _1352) - 1.0f) * _1361) * _1364))) * 1.0f) * _441) * ev_mult;
                    }
                    else
                    {
                        _852 = _851;
                        _854 = _853;
                        _856 = _855;
                        _860 = _859;
                        _862 = _861;
                        _864 = _863;
                        _866 = _865;
                        _1382 = 0.0f.xxx;
                    }
                    _849 = _848 + _1382;
                }
                _1383 = _848;
            }
            else
            {
                _1383 = _810;
            }
            _2280 = _1383;
        }
        _2281 = _2280;
    }
    else
    {
        _2281 = _810;
    }
    out_var_SV_TARGET = float4(_2281, 0.0f);
    return out_var_SV_TARGET;
}
#else
// f0ad: pixel stage lifted from _deferred_grass_light_sun_sky.raster.hlsl_f0adff76827e73ff.bin (spirv-cross, mechanically renamed;
// _NNN names are SPIR-V ids). Behaviour-exact; readability pass pending.

void ps_main(VsToPs input)
{
    int2 _142 = int2(input.position.xy);
    float4 _163 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4((((float2(_142) + 0.5f.xx) / float2(depth_tex_dims)) - 0.5f.xx) * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(_142), 0u)).x, 1.0f)));
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 _178 = uint3(clamp(floor(((_163.xyz / _163.w.xxx) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        if (6u == (WaveActiveAllTrue((light_indirection_grid.Load<uint>(((32u * ((32u * _178.y) + _178.z)) + _178.x) * 4 + 0) >> 20u) <= 2u) ? 2u : 6u))
        {
            uint _139[8];
            for (uint _197 = 0u; _197 < 8u; )
            {
                _139[_197] = 4294967295u;
                _197++;
                continue;
            }
            uint2 _204 = uint2(_142 & int2(1, 1));
            uint _209 = (_204.x + (_204.y * 2u)) & 3u;
            uint _211;
            _211 = 0u;
            uint _212;
            uint _214 = 0u;
            for (;;)
            {
                bool _222;
                if (_214 < 8u)
                {
                    _222 = _211 < 8u;
                }
                else
                {
                    _222 = false;
                }
                if (_222)
                {
                    uint4 _234 = rdi_reservoir_tex.Load(int3(uint2((_142 / int2(2, 2)) + spatial_resolve_offsets_array[((_209 * 16u) + _214) + 256u].xy), 0u));
                    uint2 _235 = _234.xy;
                    uint _236 = _234.x;
                    bool _242;
                    if (any(bool2(_235.x != uint2(0u, 0u).x, _235.y != uint2(0u, 0u).y)))
                    {
                        _242 = _236 < lantern_count;
                    }
                    else
                    {
                        _242 = false;
                    }
                    bool _248;
                    if (_242)
                    {
                        _248 = _139[0u] != _236;
                    }
                    else
                    {
                        _248 = false;
                    }
                    bool _254;
                    if (_248)
                    {
                        _254 = _139[1u] != _236;
                    }
                    else
                    {
                        _254 = false;
                    }
                    bool _260;
                    if (_254)
                    {
                        _260 = _139[2u] != _236;
                    }
                    else
                    {
                        _260 = false;
                    }
                    bool _266;
                    if (_260)
                    {
                        _266 = _139[3u] != _236;
                    }
                    else
                    {
                        _266 = false;
                    }
                    bool _272;
                    if (_266)
                    {
                        _272 = _139[4u] != _236;
                    }
                    else
                    {
                        _272 = false;
                    }
                    bool _278;
                    if (_272)
                    {
                        _278 = _139[5u] != _236;
                    }
                    else
                    {
                        _278 = false;
                    }
                    bool _284;
                    if (_278)
                    {
                        _284 = _139[6u] != _236;
                    }
                    else
                    {
                        _284 = false;
                    }
                    bool _290;
                    if (_284)
                    {
                        _290 = _139[7u] != _236;
                    }
                    else
                    {
                        _290 = false;
                    }
                    if (_290)
                    {
                        _139[_211] = _236;
                        _212 = _211 + 1u;
                    }
                    else
                    {
                        _212 = _211;
                    }
                    _211 = _212;
                    _214++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint _297;
            for (uint _296 = _211, _299 = 0u; _299 < 8u; _296 = _297, _299++)
            {
                uint _306 = QuadReadAcrossDiagonal(_139[_299]);
                bool _311;
                if (_306 < lantern_count)
                {
                    _311 = _296 < 8u;
                }
                else
                {
                    _311 = false;
                }
                if (_311)
                {
                    bool _322;
                    if (_139[0u] != _306)
                    {
                        _322 = _139[1u] != _306;
                    }
                    else
                    {
                        _322 = false;
                    }
                    bool _328;
                    if (_322)
                    {
                        _328 = _139[2u] != _306;
                    }
                    else
                    {
                        _328 = false;
                    }
                    bool _334;
                    if (_328)
                    {
                        _334 = _139[3u] != _306;
                    }
                    else
                    {
                        _334 = false;
                    }
                    bool _340;
                    if (_334)
                    {
                        _340 = _139[4u] != _306;
                    }
                    else
                    {
                        _340 = false;
                    }
                    bool _346;
                    if (_340)
                    {
                        _346 = _139[5u] != _306;
                    }
                    else
                    {
                        _346 = false;
                    }
                    bool _352;
                    if (_346)
                    {
                        _352 = _139[6u] != _306;
                    }
                    else
                    {
                        _352 = false;
                    }
                    bool _358;
                    if (_352)
                    {
                        _358 = _139[7u] != _306;
                    }
                    else
                    {
                        _358 = false;
                    }
                    uint _363;
                    if (_358)
                    {
                        _139[_296] = _306;
                        _363 = _296 + 1u;
                    }
                    else
                    {
                        _363 = _296;
                    }
                    _297 = _363;
                }
                else
                {
                    _297 = _296;
                }
            }
        }
        else
        {
        }
    }
}
#endif
