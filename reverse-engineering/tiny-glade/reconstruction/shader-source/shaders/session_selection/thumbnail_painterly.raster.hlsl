// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/session_selection/thumbnail_painterly.raster.hlsl | F-E | bins: _session_selection_thumbnail_painterly.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Painterly session thumbnails: instanced (6 verts/instance) tiles scattered
// around first_thumbnail_center, rotated and sized by hashed randomness; the VS
// samples the thumbnail and passes the tinted sprite through C1; the ps applies
// a brush texture mask.

#include "inc/uv.hlsl"
#include "inc/common.hlsl"            // samplers (sampler_lnc s0, sampler_llc s3)
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/quad_mesh.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/color/srgb.hlsl"

cbuffer $Globals : register(b15, space0)
{
    uint2 framebuffer_dims;
    float2 first_thumbnail_center;
    float thumbnail_size;
    float hover;
    uint is_black_and_white;
    uint thumbnail_index;
};

Texture2D<float4> thumbnail_tex : register(t13, space0);
Texture2D<float4> brush_tex : register(t14, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 sprite : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    uint instance = vertex_id / 6u;
    uint vid = vertex_id % 6u;

    float4 corner;
    if (vid == 0u) corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
    else if (vid == 1u) corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
    else if (vid == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vid == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vid == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 0.0f);

    float fi = float(instance);
    float ang = fi * 0.5698403f;
    float s = sin(ang);
    float c = cos(ang);
    float2 rotated = mul(float2x2(float2(c, s), float2(-s, c)), corner.xy);
    float4 local = float4(rotated, corner.z, 1.0f - corner.w);

    uint seed = thumbnail_index + (thumbnail_index << 10u);
    seed = seed ^ (seed >> 6u);
    seed = seed + (seed << 3u);
    seed = seed ^ (seed >> 11u);
    uint rnd = ((instance * 1664525u) + (seed + (seed << 15u))) * 1664525u;
    rnd = rnd ^ (rnd >> 11u);
    rnd = rnd ^ ((rnd << 7u) & 2636928640u);
    rnd = rnd ^ ((rnd << 15u) & 4022730752u);
    uint rand_bits = rnd ^ (rnd >> 18u);

    float2 dir = float2(cos(fi * 4.7430363f), sin(fi * 4.7430363f)) * 0.5f;
    float p = fi * 0.0078125f;
    float size = lerp(0.5f, 0.75f, sqrt(p));

    uint h2 = rand_bits + (rand_bits << 10u);
    h2 = h2 ^ (h2 >> 6u);
    h2 = h2 + (h2 << 3u);
    h2 = h2 ^ (h2 >> 11u);
    bool grid = ((h2 + (h2 << 15u)) % 6u) == 0u;

    float2 offset;
    if (grid)
        offset = frac((float2(0.7548777f, 0.5698403f) * fi) + 0.5f) - 0.5f;
    else
        offset = dir * ((lerp(0.8f, 1.2f, rand01(rand_bits)) * 0.5f) * size / max(abs(dir.x), abs(dir.y)));

    float2 pos = (local.xy * thumbnail_size) * (lerp(0.25f, 0.12f, p) * 2.0f) + (offset * thumbnail_size) + first_thumbnail_center;
    pos.x = pos.x / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float4 clip = float4((pos - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    if ((instance % 2u) != 0u)
        clip = float4(0.0f, 0.0f, 0.0f, 0.0f);

    float4 sprite = thumbnail_tex.SampleLevel(sampler_lnc, offset + 0.5f, 0.0f);
    float4 sp = float4(sprite.rgb * sprite.a, sprite.a);

    float rim = size * 0.08f;
    float4 sprite_col;
    if (!grid)
    {
        float radius = 0.2f * size;
        float2 d = (abs(offset) - (0.5f * size).xx) + rim.xx;
        float dist = abs((length(max(d, 0.0f)) + min(max(d.x, d.y), 0.0f)) - rim);
        float m = (dist > radius) ? 0.0f : 1.0f - (dist / radius) * (dist / radius) * (3.0f - 2.0f * (dist / radius));
        sprite_col = sp * m;
    }
    else
        sprite_col = sp * 0.5f;

    sprite_col *= 0.75f;
    if (sprite_col.w < 1e-4f)
        clip = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = clip;
    o.uv = local.zw;
    o.sprite = sprite_col;
    return o;
}

struct PsIn
{
    float2 uv : TEXCOORD0;
    float4 sprite : TEXCOORD1;
};

float4 ps_main(PsIn pin) : SV_Target0
{
    float4 brush = brush_tex.Sample(sampler_llc, pin.uv);
    float4 col = brush.xxxx * pin.sprite;
    float3 tinted = col.rgb * lerp(1.0f, 1.2f, hover);

    float4 out_col;
    if (is_black_and_white == 1u)
        out_col = float4(dot(tinted, float3(0.2126f, 0.7152f, 0.0722f)).xxx, col.w);
    else
        out_col = float4(tinted, col.w);
    return out_col;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _session_selection_thumbnail_painterly.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b15, space0)
{
    uint2 _Globals_framebuffer_dims : packoffset(c0);
    float2 _Globals_first_thumbnail_center : packoffset(c0.z);
    float _Globals_thumbnail_size : packoffset(c1);
    float _Globals_hover : packoffset(c1.y);
    uint _Globals_is_black_and_white : packoffset(c1.z);
    uint _Globals_thumbnail_index : packoffset(c1.w);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> thumbnail_tex : register(t13, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float2 out_var_C0;
static float4 out_var_C1;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float4 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _105 = uint(gl_VertexIndex) / 6u;
    uint _106 = uint(gl_VertexIndex) % 6u;
    float4 _129;
    do
    {
        if (_106 == 0u)
        {
            _129 = float4(0.5f, -0.5f, 1.0f, 0.0f);
            break;
        }
        else
        {
            if (_106 == 1u)
            {
                _129 = float4(0.5f, 0.5f, 1.0f, 1.0f);
                break;
            }
            else
            {
                if (_106 == 2u)
                {
                    _129 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                    break;
                }
                else
                {
                    if (_106 == 3u)
                    {
                        _129 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                        break;
                    }
                    else
                    {
                        if (_106 == 4u)
                        {
                            _129 = float4(-0.5f, -0.5f, 0.0f, 0.0f);
                            break;
                        }
                        else
                        {
                            _129 = float4(0.5f, -0.5f, 1.0f, 0.0f);
                            break;
                        }
                        break; // unreachable workaround
                    }
                    break; // unreachable workaround
                }
                break; // unreachable workaround
            }
            break; // unreachable workaround
        }
        break; // unreachable workaround
    } while(false);
    float _130 = float(_105);
    float _131 = _130 * 0.56984031200408935546875f;
    float _132 = cos(_131);
    float _133 = sin(_131);
    float2 _139 = mul(float2x2(float2(_132, _133), float2(-_133, _132)), _129.xy);
    float4 _140 = float4(_139.x, _139.y, _129.z, _129.w);
    _140.w = 1.0f - _129.w;
    uint _154 = _Globals_thumbnail_index + (_Globals_thumbnail_index << 10u);
    uint _156 = _154 ^ (_154 >> 6u);
    uint _158 = _156 + (_156 << 3u);
    uint _160 = _158 ^ (_158 >> 11u);
    uint _166 = (((_105 * 1664525u) + (_160 + (_160 << 15u))) + 1013904223u) * 1664525u;
    uint _168 = _166 ^ (_166 >> 11u);
    uint _171 = _168 ^ ((_168 << 7u) & 2636928640u);
    uint _174 = _171 ^ ((_171 << 15u) & 4022730752u);
    uint _176 = _174 ^ (_174 >> 18u);
    float _177 = _130 * 4.7430362701416015625f;
    float2 _181 = float2(cos(_177), sin(_177)) * 0.5f;
    float _182 = _130 * 0.0078125f;
    float _184 = lerp(0.5f, 0.75f, sqrt(_182));
    uint _186 = _176 + (_176 << 10u);
    uint _188 = _186 ^ (_186 >> 6u);
    uint _190 = _188 + (_188 << 3u);
    uint _192 = _190 ^ (_190 >> 11u);
    bool _200 = ((_192 + (_192 << 15u)) % 6u) == 0u;
    float2 _217;
    if (_200)
    {
        _217 = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * _130) + 0.5f.xx) - 0.5f.xx;
    }
    else
    {
        _217 = _181 * (((lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat((_176 & 8388607u) | 1065353216u) - 1.0f) * 0.5f) * _184) / max(abs(_181.x), abs(_181.y)));
    }
    float2 _229 = (((_140.xy * _Globals_thumbnail_size) * (lerp(0.25f, 0.119999997317790985107421875f, _182) * 2.0f)) + (_217 * _Globals_thumbnail_size)) + _Globals_first_thumbnail_center;
    _229.x = _229.x / (float(_Globals_framebuffer_dims.x) / float(_Globals_framebuffer_dims.y));
    float4 _237 = float4((_229 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    bool4 _241 = ((_105 % 2u) != 0u).xxxx;
    float4 _242 = float4(_241.x ? 0.0f.xxxx.x : _237.x, _241.y ? 0.0f.xxxx.y : _237.y, _241.z ? 0.0f.xxxx.z : _237.z, _241.w ? 0.0f.xxxx.w : _237.w);
    float4 _247 = thumbnail_tex.SampleLevel(sampler_lnc, _217 + 0.5f.xx, 0.0f);
    float3 _250 = _247.xyz * _247.w;
    float4 _251 = float4(_250.x, _250.y, _250.z, _247.w);
    float _252 = _184 * 0.080000005662441253662109375f;
    float4 _287;
    if (!_200)
    {
        float _258 = 0.20000000298023223876953125f * _184;
        float2 _264 = (abs(_217) - (0.5f * _184).xx) + _252.xx;
        float _285;
        do
        {
            float _275 = abs((length(max(_264, 0.0f.xx)) + min(max(_264.x, _264.y), 0.0f)) - _252);
            if (_275 > _258)
            {
                _285 = 0.0f;
                break;
            }
            float _279 = _275 / _258;
            _285 = 1.0f - ((_279 * _279) * (3.0f - (2.0f * _279)));
            break;
        } while(false);
        _287 = _251 * _285;
    }
    else
    {
        _287 = _251 * 0.5f;
    }
    float4 _288 = _287 * 0.75f;
    bool4 _291 = (_288.w < 9.9999997473787516355514526367188e-05f).xxxx;
    gl_Position = float4(_291.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : _242.x, _291.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : _242.y, _291.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : _242.z, _291.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : _242.w);
    out_var_C0 = _140.zw;
    out_var_C1 = _288;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _session_selection_thumbnail_painterly.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b15, space0)
{
    uint2 _Globals_framebuffer_dims : packoffset(c0);
    float2 _Globals_first_thumbnail_center : packoffset(c0.z);
    float _Globals_thumbnail_size : packoffset(c1);
    float _Globals_hover : packoffset(c1.y);
    uint _Globals_is_black_and_white : packoffset(c1.z);
    uint _Globals_thumbnail_index : packoffset(c1.w);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> thumbnail_tex : register(t13, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float2 out_var_C0;
static float4 out_var_C1;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float4 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _105 = uint(gl_VertexIndex) / 6u;
    uint _106 = uint(gl_VertexIndex) % 6u;
    float4 _129;
    do
    {
        if (_106 == 0u)
        {
            _129 = float4(0.5f, -0.5f, 1.0f, 0.0f);
            break;
        }
        else
        {
            if (_106 == 1u)
            {
                _129 = float4(0.5f, 0.5f, 1.0f, 1.0f);
                break;
            }
            else
            {
                if (_106 == 2u)
                {
                    _129 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                    break;
                }
                else
                {
                    if (_106 == 3u)
                    {
                        _129 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                        break;
                    }
                    else
                    {
                        if (_106 == 4u)
                        {
                            _129 = float4(-0.5f, -0.5f, 0.0f, 0.0f);
                            break;
                        }
                        else
                        {
                            _129 = float4(0.5f, -0.5f, 1.0f, 0.0f);
                            break;
                        }
                        break; // unreachable workaround
                    }
                    break; // unreachable workaround
                }
                break; // unreachable workaround
            }
            break; // unreachable workaround
        }
        break; // unreachable workaround
    } while(false);
    float _130 = float(_105);
    float _131 = _130 * 0.56984031200408935546875f;
    float _132 = cos(_131);
    float _133 = sin(_131);
    float2 _139 = mul(float2x2(float2(_132, _133), float2(-_133, _132)), _129.xy);
    float4 _140 = float4(_139.x, _139.y, _129.z, _129.w);
    _140.w = 1.0f - _129.w;
    uint _154 = _Globals_thumbnail_index + (_Globals_thumbnail_index << 10u);
    uint _156 = _154 ^ (_154 >> 6u);
    uint _158 = _156 + (_156 << 3u);
    uint _160 = _158 ^ (_158 >> 11u);
    uint _166 = (((_105 * 1664525u) + (_160 + (_160 << 15u))) + 1013904223u) * 1664525u;
    uint _168 = _166 ^ (_166 >> 11u);
    uint _171 = _168 ^ ((_168 << 7u) & 2636928640u);
    uint _174 = _171 ^ ((_171 << 15u) & 4022730752u);
    uint _176 = _174 ^ (_174 >> 18u);
    float _177 = _130 * 4.7430362701416015625f;
    float2 _181 = float2(cos(_177), sin(_177)) * 0.5f;
    float _182 = _130 * 0.0078125f;
    float _184 = lerp(0.5f, 0.75f, sqrt(_182));
    uint _186 = _176 + (_176 << 10u);
    uint _188 = _186 ^ (_186 >> 6u);
    uint _190 = _188 + (_188 << 3u);
    uint _192 = _190 ^ (_190 >> 11u);
    bool _200 = ((_192 + (_192 << 15u)) % 6u) == 0u;
    float2 _217;
    if (_200)
    {
        _217 = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * _130) + 0.5f.xx) - 0.5f.xx;
    }
    else
    {
        _217 = _181 * (((lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat((_176 & 8388607u) | 1065353216u) - 1.0f) * 0.5f) * _184) / max(abs(_181.x), abs(_181.y)));
    }
    float2 _229 = (((_140.xy * _Globals_thumbnail_size) * (lerp(0.25f, 0.119999997317790985107421875f, _182) * 2.0f)) + (_217 * _Globals_thumbnail_size)) + _Globals_first_thumbnail_center;
    _229.x = _229.x / (float(_Globals_framebuffer_dims.x) / float(_Globals_framebuffer_dims.y));
    float4 _237 = float4((_229 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    bool4 _241 = ((_105 % 2u) != 0u).xxxx;
    float4 _242 = float4(_241.x ? 0.0f.xxxx.x : _237.x, _241.y ? 0.0f.xxxx.y : _237.y, _241.z ? 0.0f.xxxx.z : _237.z, _241.w ? 0.0f.xxxx.w : _237.w);
    float4 _247 = thumbnail_tex.SampleLevel(sampler_lnc, _217 + 0.5f.xx, 0.0f);
    float3 _250 = _247.xyz * _247.w;
    float4 _251 = float4(_250.x, _250.y, _250.z, _247.w);
    float _252 = _184 * 0.080000005662441253662109375f;
    float4 _287;
    if (!_200)
    {
        float _258 = 0.20000000298023223876953125f * _184;
        float2 _264 = (abs(_217) - (0.5f * _184).xx) + _252.xx;
        float _285;
        do
        {
            float _275 = abs((length(max(_264, 0.0f.xx)) + min(max(_264.x, _264.y), 0.0f)) - _252);
            if (_275 > _258)
            {
                _285 = 0.0f;
                break;
            }
            float _279 = _275 / _258;
            _285 = 1.0f - ((_279 * _279) * (3.0f - (2.0f * _279)));
            break;
        } while(false);
        _287 = _251 * _285;
    }
    else
    {
        _287 = _251 * 0.5f;
    }
    float4 _288 = _287 * 0.75f;
    bool4 _291 = (_288.w < 9.9999997473787516355514526367188e-05f).xxxx;
    gl_Position = float4(_291.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : _242.x, _291.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : _242.y, _291.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : _242.z, _291.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : _242.w);
    out_var_C0 = _140.zw;
    out_var_C1 = _288;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _session_selection_thumbnail_painterly.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
