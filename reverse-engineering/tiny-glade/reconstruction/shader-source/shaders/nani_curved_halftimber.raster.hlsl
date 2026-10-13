// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_curved_halftimber.raster.hlsl | B nani raster | bins: _nani_curved_halftimber.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani curved half-timber: VS builds the curved timber beam from instance
// control points; PS shades halftimber + wood roughness + plaster fill and
// packs the gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "buildings/curved_halftimber.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/parallax_mapping.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

uint2 wood_normal_dims;
uint2 wood_roughness_dims;
uint2 wood_edge_damage_dims;
uint2 dirthpath_heightmap_dims;

struct InstanceData
{
    Affine3Packed xform;
    uint flags;
    float beam_count;
    uint curve_type;
    uint color_id;
    uint wall_id;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> wood_normal : register(t16);
Texture2D<float4> wood_roughness : register(t17);
Texture2D<float4> wood_edge_damage : register(t18);
Texture2DArray<float4> halftimber_colors : register(t20);
Texture2D<float4> dirthpath_heightmap : register(t21);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    uint vertex_id : SV_VertexID;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    float C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    nointerpolation uint C9 : TEXCOORD9;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C1 = mul(xform, float4(0.0f, 1.0f, 0.0f, 0.0f));
    o.C2 = world;
    o.C4 = idx;
    o.C5 = vin.Vertex_Position.z;
    o.C6 = 1.0f;
    o.C7 = vin.Vertex_Position.xy;
    o.C8 = vin.Vertex_Position.x;
    o.C9 = 0u;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    float C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    nointerpolation uint C9 : TEXCOORD9;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float2 uv = pin.C7 * float2(0.4f, 0.06f);
    float3 normal = pin.C1;
    float2 beam_uv = uv;
    float edge = 0.0f;
    if (pin.C5 <= 0.01f)
    {
        normal = normalize(cross(ddx(pin.C2), -ddy(pin.C2)));
        beam_uv = float2(uv.y, uv.x * 2.0f) + 0.87f.xx;
        edge = 1.0f - wood_edge_damage.SampleBias(sampler_llr, pin.C7 * float2(1.0f, 0.05f), global_mip_bias).x;
    }
    int color_id = int(nani_instance_data[pin.C4].color_id);
    float2 sample_uv = beam_uv + (edge * 0.3f);
    float3 albedo = halftimber_colors.SampleBias(sampler_llr, float3(sample_uv.x, sample_uv.y, float(color_id)), global_mip_bias).xyz;
    float2 rough_uv = sample_uv * float2(0.5f, (pin.C8 < 0.0f) ? 3.0f : 2.0f);
    float roughness = lerp(0.7f, 0.9f, 1.0f - wood_roughness.SampleBias(sampler_llr, rough_uv, global_mip_bias).x);
    if (pin.C5 <= 0.01f)
    {
        albedo = lerp(albedo, 0.75f.xxx, (pin.C8 < 0.0f ? 0.2f : 0.0f).xxx);
    }
    float3 normal_map = wood_normal.SampleBias(sampler_llr, sample_uv, global_mip_bias).xyz * 2.0f - 1.0f.xxx;
    float3 shaded_normal = normalize(normal + (normal_map * 0.4f));
    if (glade_theme_id == 4u)
    {
        float path = dirthpath_heightmap.SampleBias(sampler_llr, pin.C2.xz * 0.05f, global_mip_bias).x;
        albedo = lerp(albedo, 0.7f.xxx, (((smoothstep(0.0f, 0.2f, smoothstep(0.5f, 1.0f, roughness) * lerp(0.0f, 0.5f, path)) * 0.6f) + (path * 0.2f)) * clamp(shaded_normal.y, 0.0f, 1.0f)).xxx);
    }
    float3 packed_normal = (normalize(shaded_normal) * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + (uint(clamp(roughness, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 117440512u;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_curved_halftimber.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
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

struct CurvedHalftimberInstanceData
{
    float3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    int is_straight;
    int is_xz;
    float plank_depth;
    int caps;
    int mesh_atlas_part_idx;
    int pad0;
    int pad1;
    int pad2;
};

static const float2 _126[7] = { 0.0f.xx, float2(0.0f, 0.89999997615814208984375f), float2(0.100000001490116119384765625f, 1.0f), float2(0.89999997615814208984375f, 1.0f), float2(1.0f, 0.89999997615814208984375f), float2(1.0f, 0.0f), 0.0f.xx };
static const float _130[7] = { 0.0f, 0.89999997615814208984375f, 1.10000002384185791015625f, 1.89999997615814208984375f, 2.099999904632568359375f, 3.0f, 4.0f };
static const float2 _134[7] = { float2(-1.0f, 0.0f), float2(-1.0f, 0.0f), float2(0.0f, 1.0f), float2(0.0f, 1.0f), float2(1.0f, 0.0f), float2(1.0f, 0.0f), float2(0.0f, -1.0f) };

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_wood_normal_dims : packoffset(c0.y);
    uint2 _Globals_wood_roughness_dims : packoffset(c1);
    uint2 _Globals_wood_edge_damage_dims : packoffset(c1.z);
    uint2 _Globals_dirthpath_heightmap_dims : packoffset(c2);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float3 out_var_C2;
static uint out_var_C4;
static float out_var_C5;
static float out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static uint out_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(4)]] nointerpolation uint out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] nointerpolation uint out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _155;
    float4 _440;
    float3 _441;
    float3 _442;
    float _443;
    float _444;
    float2 _445;
    float _446;
    uint _447;
    do
    {
        _155 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
        int _179 = int(1u << (uint(nani_instance_data.Load<int>(_155 * 64 + 48)) & 31u));
        uint _180 = uint(gl_VertexIndex) % 7u;
        float _187 = _130[_180] / _130[5];
        bool _190 = nani_instance_data.Load<int>(_155 * 64 + 36) == 0;
        float2 _199;
        float2 _200;
        if (_190)
        {
            _199 = float2(_134[_180].yx);
            _200 = float2(_126[_180].yx);
        }
        else
        {
            _199 = _134[_180];
            _200 = _126[_180];
        }
        float _202 = 1.0f / float(_179);
        if (nani_instance_data.Load<int>(_155 * 64 + 32) > 0)
        {
            float3 _353 = float3(nani_instance_data.Load<float>(_155 * 64 + 16), 0.0f, nani_instance_data.Load<float>(_155 * 64 + 20));
            float _392;
            if (nani_instance_data.Load<int>(_155 * 64 + 32) == 2)
            {
                float _391;
                if (in_var_Vertex_Position.z < 0.5f)
                {
                    _391 = nani_instance_data.Load<float>(_155 * 64 + 28) + (nani_instance_data.Load<float>(_155 * 64 + 24) * lerp(-1.0f, 1.0f, _200.x));
                }
                else
                {
                    _391 = nani_instance_data.Load<float>(_155 * 64 + 28);
                }
                _392 = _391;
            }
            else
            {
                float _382;
                if (nani_instance_data.Load<int>(_155 * 64 + 32) == 3)
                {
                    float _381;
                    if (in_var_Vertex_Position.z > 0.5f)
                    {
                        _381 = nani_instance_data.Load<float>(_155 * 64 + 28) + (nani_instance_data.Load<float>(_155 * 64 + 24) * lerp(-1.0f, 1.0f, _200.x));
                    }
                    else
                    {
                        _381 = nani_instance_data.Load<float>(_155 * 64 + 28);
                    }
                    _382 = _381;
                }
                else
                {
                    float _372;
                    if (nani_instance_data.Load<int>(_155 * 64 + 32) == 4)
                    {
                        _372 = nani_instance_data.Load<float>(_155 * 64 + 28) + (nani_instance_data.Load<float>(_155 * 64 + 24) * lerp(-1.0f, 1.0f, _200.x));
                    }
                    else
                    {
                        _372 = nani_instance_data.Load<float>(_155 * 64 + 28);
                    }
                    _382 = _372;
                }
                _392 = _382;
            }
            float3 _402 = (nani_instance_data.Load<float3>(_155 * 64 + 0) + ((_353 * lerp(-0.5f, 0.5f, _200.x)) * nani_instance_data.Load<float>(_155 * 64 + 24))) + (((-float3(-nani_instance_data.Load<float>(_155 * 64 + 20), 0.0f, nani_instance_data.Load<float>(_155 * 64 + 16))) * lerp(-0.5f, 0.5f, in_var_Vertex_Position.z)) * _392);
            uint _408 = _155 + (_155 << 10u);
            uint _410 = _408 ^ (_408 >> 6u);
            uint _412 = _410 + (_410 << 3u);
            uint _414 = _412 ^ (_412 >> 11u);
            float _422 = (_402.y + (_200.y * nani_instance_data.Load<float>(_155 * 64 + 40))) + ((asfloat(((_414 + (_414 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.00999999977648258209228515625f);
            float3 _423 = _402;
            _423.y = _422;
            _440 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_402.x, _422, _402.z, 1.0f));
            _441 = (_353 * _199.x) + float3(0.0f, _199.y, 0.0f);
            _442 = _423;
            _443 = 0.0f;
            _444 = in_var_Vertex_Position.z;
            _445 = float2(_187, in_var_Vertex_Position.z * _392);
            _446 = -1.0f;
            _447 = uint(_179);
            break;
        }
        else
        {
            float _272;
            float _273;
            if (_190)
            {
                _272 = 0.0f;
                _273 = lerp(nani_instance_data.Load<float>(_155 * 64 + 24), nani_instance_data.Load<float>(_155 * 64 + 28), 1.0f - in_var_Vertex_Position.z);
            }
            else
            {
                float _266;
                float _267;
                if (nani_instance_data.Load<int>(_155 * 64 + 44) == 1)
                {
                    float _264;
                    float _265;
                    if (in_var_Vertex_Position.z >= _202)
                    {
                        _264 = 0.0f;
                        _265 = (in_var_Vertex_Position.z - _202) / (1.0f - _202);
                    }
                    else
                    {
                        bool _258 = in_var_Vertex_Position.z < _202;
                        _264 = float(_258);
                        _265 = _258 ? 0.0f : in_var_Vertex_Position.z;
                    }
                    _266 = _264;
                    _267 = _265;
                }
                else
                {
                    float _252;
                    float _253;
                    if (nani_instance_data.Load<int>(_155 * 64 + 44) == 2)
                    {
                        float _241 = 1.0f - _202;
                        float _250;
                        float _251;
                        if (in_var_Vertex_Position.z <= _241)
                        {
                            _250 = 0.0f;
                            _251 = in_var_Vertex_Position.z / _241;
                        }
                        else
                        {
                            bool _246 = in_var_Vertex_Position.z > _241;
                            _250 = float(_246);
                            _251 = _246 ? 1.0f : in_var_Vertex_Position.z;
                        }
                        _252 = _250;
                        _253 = _251;
                    }
                    else
                    {
                        float _239;
                        float _240;
                        if (nani_instance_data.Load<int>(_155 * 64 + 44) == 3)
                        {
                            bool _227;
                            if (in_var_Vertex_Position.z >= _202)
                            {
                                _227 = in_var_Vertex_Position.z <= (1.0f - _202);
                            }
                            else
                            {
                                _227 = false;
                            }
                            float _237;
                            if (_227)
                            {
                                _237 = (in_var_Vertex_Position.z - _202) / ((1.0f - _202) - _202);
                            }
                            else
                            {
                                _237 = (in_var_Vertex_Position.z < _202) ? 0.0f : 1.0f;
                            }
                            _239 = _227 ? 0.0f : 1.0f;
                            _240 = _237;
                        }
                        else
                        {
                            _239 = 0.0f;
                            _240 = in_var_Vertex_Position.z;
                        }
                        _252 = _239;
                        _253 = _240;
                    }
                    _266 = _252;
                    _267 = _253;
                }
                _272 = _266;
                _273 = lerp(nani_instance_data.Load<float>(_155 * 64 + 24), nani_instance_data.Load<float>(_155 * 64 + 28), _267);
            }
            float _276 = cos(_273);
            float _278 = -sin(_273);
            float3 _281 = (float3(_276, 0.0f, _278) * lerp(nani_instance_data.Load<float>(_155 * 64 + 16), nani_instance_data.Load<float>(_155 * 64 + 20), _200.x)) + nani_instance_data.Load<float3>(_155 * 64 + 0);
            float3 _322;
            if (nani_instance_data.Load<int>(_155 * 64 + 44) == 0)
            {
                float3 _321 = _281;
                _321.y = _281.y + (_200.y * nani_instance_data.Load<float>(_155 * 64 + 40));
                _322 = _321;
            }
            else
            {
                float _310;
                if (nani_instance_data.Load<int>(_155 * 64 + 44) == 1)
                {
                    _310 = ((in_var_Vertex_Position.z >= _202) ? 1.0f : 0.0f);
                }
                else
                {
                    float _306;
                    if (nani_instance_data.Load<int>(_155 * 64 + 44) == 2)
                    {
                        _306 = ((in_var_Vertex_Position.z <= (1.0f - _202)) ? 1.0f : 0.0f);
                    }
                    else
                    {
                        bool _300;
                        if (in_var_Vertex_Position.z >= _202)
                        {
                            _300 = in_var_Vertex_Position.z <= (1.0f - _202);
                        }
                        else
                        {
                            _300 = false;
                        }
                        _306 = float(_300);
                    }
                    _310 = _306;
                }
                float3 _316 = _281;
                _316.y = _281.y + ((_310 * _200.y) * nani_instance_data.Load<float>(_155 * 64 + 40));
                _322 = _316;
            }
            float _330 = 0.001953125f * (floor(_322.y * 512.0f) + frac(float(_155) * 1.61803400516510009765625f));
            float3 _331 = _322;
            _331.y = _330;
            _440 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_322.x, _330, _322.z, 1.0f));
            _441 = float3(_276, 1.0f, _278) * float3(_199.xyx);
            _442 = _331;
            _443 = (uint(gl_VertexIndex) == 6u) ? 1.0f : _272;
            _444 = in_var_Vertex_Position.z;
            _445 = float2(_187, (_273 * nani_instance_data.Load<float>(_155 * 64 + 16)) * 3.1400001049041748046875f);
            _446 = _273 * 0.159235656261444091796875f;
            _447 = uint(_179);
            break;
        }
        break; // unreachable workaround
    } while(false);
    gl_Position = _440;
    out_var_C1 = _441;
    out_var_C2 = _442;
    out_var_C4 = _155;
    out_var_C5 = _443;
    out_var_C6 = _444;
    out_var_C7 = _445;
    out_var_C8 = _446;
    out_var_C9 = _447;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    stage_output.out_var_C9 = out_var_C9;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_curved_halftimber.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
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

struct CurvedHalftimberInstanceData
{
    float3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    int is_straight;
    int is_xz;
    float plank_depth;
    int caps;
    int mesh_atlas_part_idx;
    int pad0;
    int pad1;
    int pad2;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> wood_normal : register(t16, space0);
Texture2D<float4> wood_roughness : register(t17, space0);
Texture2D<float4> wood_edge_damage : register(t18, space0);
Texture2DArray<float4> halftimber_colors : register(t20, space0);
Texture2D<float4> dirthpath_heightmap : register(t21, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static uint in_var_C4;
static float in_var_C5;
static float2 in_var_C7;
static float in_var_C8;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float in_var_C8 : TEXCOORD8;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float2 _126 = in_var_C7 * float2(0.4000000059604644775390625f, 0.0599999986588954925537109375f);
    float3 _150;
    float2 _151;
    float _152;
    if (in_var_C5 > 0.00999999977648258209228515625f)
    {
        _150 = normalize(cross(ddx(in_var_C2), -ddy(in_var_C2)));
        _151 = float2(_126.y, _126.x * 2.0f) + 0.87000000476837158203125f.xx;
        _152 = 0.0f;
    }
    else
    {
        _150 = in_var_C1;
        _151 = _126;
        _152 = 1.0f - wood_edge_damage.SampleBias(sampler_llr, in_var_C7 * float2(1.0f, 0.0500000007450580596923828125f), frame_constants_global_mip_bias).x;
    }
    int _155 = int(nani_instance_data.Load<uint>(in_var_C4 * 64 + 12));
    float2 _158 = _151 + (_152.xx * 0.300000011920928955078125f);
    float _161 = float(_155);
    float _162 = _158.x;
    float4 _168 = halftimber_colors.SampleBias(sampler_llr, float3(_162, _158.y, _161), frame_constants_global_mip_bias);
    float3 _169 = _168.xyz;
    float2 _173 = _158 * float2(0.5f, (in_var_C8 < 0.0f) ? 3.0f : 2.0f);
    float4 _176 = wood_roughness.SampleBias(sampler_llr, _173, frame_constants_global_mip_bias);
    float _179 = lerp(0.699999988079071044921875f, 0.89999997615814208984375f, 1.0f - _176.x);
    float3 _220;
    if (in_var_C5 <= 0.00999999977648258209228515625f)
    {
        float2 _186 = ddx(_173);
        float2 _188 = -ddy(_173);
        float3 _189 = cross(in_var_C1, -ddy(in_var_C2));
        float3 _190 = cross(ddx(in_var_C2), in_var_C1);
        float _191 = _186.x;
        float _192 = _188.y;
        float _194 = _186.y;
        float _195 = _188.x;
        float _198 = 1.0f / ((_191 * _192) - (_194 * _195));
        float3 _214 = (wood_normal.SampleBias(sampler_llr, _173, frame_constants_global_mip_bias).xyz * 2.0f) - 1.0f.xxx;
        _214.z = _214.z * 18.1818180084228515625f;
        _220 = normalize(mul(transpose(float3x3(((_189 * _191) + (_190 * _195)) * _198, ((_189 * _194) + (_190 * _192)) * _198, in_var_C1)), _214));
    }
    else
    {
        _220 = _150;
    }
    float3 _252;
    float _253;
    float3 _254;
    if (in_var_C8 > 0.0f)
    {
        float3 _249;
        float _250;
        float3 _251;
        if (in_var_C8 > 0.89999997615814208984375f)
        {
            float _228 = smoothstep(0.89999997615814208984375f, 1.0f, in_var_C8);
            float3 _235 = _228.xxx;
            _249 = lerp(_220, in_var_C1, _235);
            _250 = lerp(_179, 0.89999997615814208984375f, _228);
            _251 = lerp(_169, halftimber_colors.SampleBias(sampler_llr, float3(_151.x, -_228, _161), frame_constants_global_mip_bias).xyz, _235);
        }
        else
        {
            float3 _247;
            float _248;
            if (in_var_C8 < 0.100000001490116119384765625f)
            {
                float _243 = (0.100000001490116119384765625f - in_var_C8) * 10.0f;
                _247 = lerp(_220, in_var_C1, _243.xxx);
                _248 = lerp(_179, 0.89999997615814208984375f, _243);
            }
            else
            {
                _247 = _220;
                _248 = _179;
            }
            _249 = _247;
            _250 = _248;
            _251 = _169;
        }
        _252 = _249;
        _253 = _250;
        _254 = _251;
    }
    else
    {
        _252 = _220;
        _253 = _179;
        _254 = _169;
    }
    float4 _257 = halftimber_colors.SampleBias(sampler_llr, float3(_162, _158.y, 0.0f), frame_constants_global_mip_bias);
    float3 _264 = lerp(_254, _257.xyz * ((_155 == 0) ? 2.0f : 1.0f), (_152 * 0.4000000059604644775390625f).xxx);
    float3 _288;
    if (frame_constants_glade_theme_id == 4u)
    {
        float4 _274 = dirthpath_heightmap.SampleBias(sampler_llr, in_var_C2.xz * 0.0500000007450580596923828125f, frame_constants_global_mip_bias);
        float _275 = _274.x;
        _288 = lerp(_264, 0.699999988079071044921875f.xxx, (((smoothstep(0.0f, 0.20000000298023223876953125f, smoothstep(0.5f, 1.0f, _253) * lerp(0.0f, 0.5f, _275)) * 0.60000002384185791015625f) + (_275 * 0.20000000298023223876953125f)) * clamp(_252.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        _288 = _264;
    }
    float3 _290 = (_252 * 0.5f) + 0.5f.xxx;
    float3 _317 = sqrt(clamp(_288, 0.0f.xxx, 1.0f.xxx));
    uint _336 = (uint((clamp(_317.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_317.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_317.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _337 = uint2(((uint((clamp(_290.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_290.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_290.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_253, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), _336);
    _337.y = _336 | 117440512u;
    out_var_SV_TARGET0 = _337;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_curved_halftimber.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
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

struct CurvedHalftimberInstanceData
{
    float3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    int is_straight;
    int is_xz;
    float plank_depth;
    int caps;
    int mesh_atlas_part_idx;
    int pad0;
    int pad1;
    int pad2;
};

static const float2 _126[7] = { 0.0f.xx, float2(0.0f, 0.89999997615814208984375f), float2(0.100000001490116119384765625f, 1.0f), float2(0.89999997615814208984375f, 1.0f), float2(1.0f, 0.89999997615814208984375f), float2(1.0f, 0.0f), 0.0f.xx };
static const float _130[7] = { 0.0f, 0.89999997615814208984375f, 1.10000002384185791015625f, 1.89999997615814208984375f, 2.099999904632568359375f, 3.0f, 4.0f };
static const float2 _134[7] = { float2(-1.0f, 0.0f), float2(-1.0f, 0.0f), float2(0.0f, 1.0f), float2(0.0f, 1.0f), float2(1.0f, 0.0f), float2(1.0f, 0.0f), float2(0.0f, -1.0f) };

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_wood_normal_dims : packoffset(c0.y);
    uint2 _Globals_wood_roughness_dims : packoffset(c1);
    uint2 _Globals_wood_edge_damage_dims : packoffset(c1.z);
    uint2 _Globals_dirthpath_heightmap_dims : packoffset(c2);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float3 out_var_C2;
static uint out_var_C4;
static float out_var_C5;
static float out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static uint out_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(4)]] nointerpolation uint out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] nointerpolation uint out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _155;
    float4 _440;
    float3 _441;
    float3 _442;
    float _443;
    float _444;
    float2 _445;
    float _446;
    uint _447;
    do
    {
        _155 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
        int _179 = int(1u << (uint(nani_instance_data.Load<int>(_155 * 64 + 48)) & 31u));
        uint _180 = uint(gl_VertexIndex) % 7u;
        float _187 = _130[_180] / _130[5];
        bool _190 = nani_instance_data.Load<int>(_155 * 64 + 36) == 0;
        float2 _199;
        float2 _200;
        if (_190)
        {
            _199 = float2(_134[_180].yx);
            _200 = float2(_126[_180].yx);
        }
        else
        {
            _199 = _134[_180];
            _200 = _126[_180];
        }
        float _202 = 1.0f / float(_179);
        if (nani_instance_data.Load<int>(_155 * 64 + 32) > 0)
        {
            float3 _353 = float3(nani_instance_data.Load<float>(_155 * 64 + 16), 0.0f, nani_instance_data.Load<float>(_155 * 64 + 20));
            float _392;
            if (nani_instance_data.Load<int>(_155 * 64 + 32) == 2)
            {
                float _391;
                if (in_var_Vertex_Position.z < 0.5f)
                {
                    _391 = nani_instance_data.Load<float>(_155 * 64 + 28) + (nani_instance_data.Load<float>(_155 * 64 + 24) * lerp(-1.0f, 1.0f, _200.x));
                }
                else
                {
                    _391 = nani_instance_data.Load<float>(_155 * 64 + 28);
                }
                _392 = _391;
            }
            else
            {
                float _382;
                if (nani_instance_data.Load<int>(_155 * 64 + 32) == 3)
                {
                    float _381;
                    if (in_var_Vertex_Position.z > 0.5f)
                    {
                        _381 = nani_instance_data.Load<float>(_155 * 64 + 28) + (nani_instance_data.Load<float>(_155 * 64 + 24) * lerp(-1.0f, 1.0f, _200.x));
                    }
                    else
                    {
                        _381 = nani_instance_data.Load<float>(_155 * 64 + 28);
                    }
                    _382 = _381;
                }
                else
                {
                    float _372;
                    if (nani_instance_data.Load<int>(_155 * 64 + 32) == 4)
                    {
                        _372 = nani_instance_data.Load<float>(_155 * 64 + 28) + (nani_instance_data.Load<float>(_155 * 64 + 24) * lerp(-1.0f, 1.0f, _200.x));
                    }
                    else
                    {
                        _372 = nani_instance_data.Load<float>(_155 * 64 + 28);
                    }
                    _382 = _372;
                }
                _392 = _382;
            }
            float3 _402 = (nani_instance_data.Load<float3>(_155 * 64 + 0) + ((_353 * lerp(-0.5f, 0.5f, _200.x)) * nani_instance_data.Load<float>(_155 * 64 + 24))) + (((-float3(-nani_instance_data.Load<float>(_155 * 64 + 20), 0.0f, nani_instance_data.Load<float>(_155 * 64 + 16))) * lerp(-0.5f, 0.5f, in_var_Vertex_Position.z)) * _392);
            uint _408 = _155 + (_155 << 10u);
            uint _410 = _408 ^ (_408 >> 6u);
            uint _412 = _410 + (_410 << 3u);
            uint _414 = _412 ^ (_412 >> 11u);
            float _422 = (_402.y + (_200.y * nani_instance_data.Load<float>(_155 * 64 + 40))) + ((asfloat(((_414 + (_414 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.00999999977648258209228515625f);
            float3 _423 = _402;
            _423.y = _422;
            _440 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_402.x, _422, _402.z, 1.0f));
            _441 = (_353 * _199.x) + float3(0.0f, _199.y, 0.0f);
            _442 = _423;
            _443 = 0.0f;
            _444 = in_var_Vertex_Position.z;
            _445 = float2(_187, in_var_Vertex_Position.z * _392);
            _446 = -1.0f;
            _447 = uint(_179);
            break;
        }
        else
        {
            float _272;
            float _273;
            if (_190)
            {
                _272 = 0.0f;
                _273 = lerp(nani_instance_data.Load<float>(_155 * 64 + 24), nani_instance_data.Load<float>(_155 * 64 + 28), 1.0f - in_var_Vertex_Position.z);
            }
            else
            {
                float _266;
                float _267;
                if (nani_instance_data.Load<int>(_155 * 64 + 44) == 1)
                {
                    float _264;
                    float _265;
                    if (in_var_Vertex_Position.z >= _202)
                    {
                        _264 = 0.0f;
                        _265 = (in_var_Vertex_Position.z - _202) / (1.0f - _202);
                    }
                    else
                    {
                        bool _258 = in_var_Vertex_Position.z < _202;
                        _264 = float(_258);
                        _265 = _258 ? 0.0f : in_var_Vertex_Position.z;
                    }
                    _266 = _264;
                    _267 = _265;
                }
                else
                {
                    float _252;
                    float _253;
                    if (nani_instance_data.Load<int>(_155 * 64 + 44) == 2)
                    {
                        float _241 = 1.0f - _202;
                        float _250;
                        float _251;
                        if (in_var_Vertex_Position.z <= _241)
                        {
                            _250 = 0.0f;
                            _251 = in_var_Vertex_Position.z / _241;
                        }
                        else
                        {
                            bool _246 = in_var_Vertex_Position.z > _241;
                            _250 = float(_246);
                            _251 = _246 ? 1.0f : in_var_Vertex_Position.z;
                        }
                        _252 = _250;
                        _253 = _251;
                    }
                    else
                    {
                        float _239;
                        float _240;
                        if (nani_instance_data.Load<int>(_155 * 64 + 44) == 3)
                        {
                            bool _227;
                            if (in_var_Vertex_Position.z >= _202)
                            {
                                _227 = in_var_Vertex_Position.z <= (1.0f - _202);
                            }
                            else
                            {
                                _227 = false;
                            }
                            float _237;
                            if (_227)
                            {
                                _237 = (in_var_Vertex_Position.z - _202) / ((1.0f - _202) - _202);
                            }
                            else
                            {
                                _237 = (in_var_Vertex_Position.z < _202) ? 0.0f : 1.0f;
                            }
                            _239 = _227 ? 0.0f : 1.0f;
                            _240 = _237;
                        }
                        else
                        {
                            _239 = 0.0f;
                            _240 = in_var_Vertex_Position.z;
                        }
                        _252 = _239;
                        _253 = _240;
                    }
                    _266 = _252;
                    _267 = _253;
                }
                _272 = _266;
                _273 = lerp(nani_instance_data.Load<float>(_155 * 64 + 24), nani_instance_data.Load<float>(_155 * 64 + 28), _267);
            }
            float _276 = cos(_273);
            float _278 = -sin(_273);
            float3 _281 = (float3(_276, 0.0f, _278) * lerp(nani_instance_data.Load<float>(_155 * 64 + 16), nani_instance_data.Load<float>(_155 * 64 + 20), _200.x)) + nani_instance_data.Load<float3>(_155 * 64 + 0);
            float3 _322;
            if (nani_instance_data.Load<int>(_155 * 64 + 44) == 0)
            {
                float3 _321 = _281;
                _321.y = _281.y + (_200.y * nani_instance_data.Load<float>(_155 * 64 + 40));
                _322 = _321;
            }
            else
            {
                float _310;
                if (nani_instance_data.Load<int>(_155 * 64 + 44) == 1)
                {
                    _310 = ((in_var_Vertex_Position.z >= _202) ? 1.0f : 0.0f);
                }
                else
                {
                    float _306;
                    if (nani_instance_data.Load<int>(_155 * 64 + 44) == 2)
                    {
                        _306 = ((in_var_Vertex_Position.z <= (1.0f - _202)) ? 1.0f : 0.0f);
                    }
                    else
                    {
                        bool _300;
                        if (in_var_Vertex_Position.z >= _202)
                        {
                            _300 = in_var_Vertex_Position.z <= (1.0f - _202);
                        }
                        else
                        {
                            _300 = false;
                        }
                        _306 = float(_300);
                    }
                    _310 = _306;
                }
                float3 _316 = _281;
                _316.y = _281.y + ((_310 * _200.y) * nani_instance_data.Load<float>(_155 * 64 + 40));
                _322 = _316;
            }
            float _330 = 0.001953125f * (floor(_322.y * 512.0f) + frac(float(_155) * 1.61803400516510009765625f));
            float3 _331 = _322;
            _331.y = _330;
            _440 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_322.x, _330, _322.z, 1.0f));
            _441 = float3(_276, 1.0f, _278) * float3(_199.xyx);
            _442 = _331;
            _443 = (uint(gl_VertexIndex) == 6u) ? 1.0f : _272;
            _444 = in_var_Vertex_Position.z;
            _445 = float2(_187, (_273 * nani_instance_data.Load<float>(_155 * 64 + 16)) * 3.1400001049041748046875f);
            _446 = _273 * 0.159235656261444091796875f;
            _447 = uint(_179);
            break;
        }
        break; // unreachable workaround
    } while(false);
    gl_Position = _440;
    out_var_C1 = _441;
    out_var_C2 = _442;
    out_var_C4 = _155;
    out_var_C5 = _443;
    out_var_C6 = _444;
    out_var_C7 = _445;
    out_var_C8 = _446;
    out_var_C9 = _447;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    stage_output.out_var_C9 = out_var_C9;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_curved_halftimber.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
