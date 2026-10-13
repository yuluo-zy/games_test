#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#define TINYGLADE_RASTER_NO_COLOR 1
#line 1 "shaders/wall/wall_brick_lod1.raster.hlsl"




#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

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

struct Affine3Packed
{
    float data[12];
};

struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

static float _192;

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
    uint _Globals_highlight_mode : packoffset(c0.y);
    int _Globals_highlight_bricks_to_delete : packoffset(c0.z);
    float3 _Globals_brush_position : packoffset(c1);
    uint _Globals_bool_render_shadows : packoffset(c1.w);
    float _Globals_eraser_mult : packoffset(c2);
};

ByteAddressBuffer wall_color_ids : register(t14, space0);
ByteAddressBuffer roof_color_ids : register(t18, space0);
ByteAddressBuffer instanced_wall_data : register(t19, space0);
ByteAddressBuffer instance_list : register(t20, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static uint out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C5;
static float out_var_C7;
static float2 out_var_C10;
static int out_var_C11;
static float2 out_var_C12;
static int out_var_C13;
static uint out_var_C14;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] nointerpolation uint out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(5)]] nointerpolation float3 out_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float out_var_C7 : TEXCOORD7;
    [[vk::location(10)]] float2 out_var_C10 : TEXCOORD10;
    [[vk::location(11)]] nointerpolation int out_var_C11 : TEXCOORD11;
    [[vk::location(12)]] nointerpolation float2 out_var_C12 : TEXCOORD12;
    [[vk::location(13)]] nointerpolation int out_var_C13 : TEXCOORD13;
    [[vk::location(14)]] nointerpolation uint out_var_C14 : TEXCOORD14;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _204 = ((1u + (2u * uint(_Globals_highlight_mode != 0u))) * 2097152u) + uint(gl_InstanceIndex);
    float _210[12];
    [unroll]
    for (int _5ident = 0; _5ident < 12; _5ident++)
    {
        _210[_5ident] = instanced_wall_data.Load<float>(_5ident * 4 + instance_list.Load<uint>(_204 * 4 + 0) * 96 + 0);
    }
    float3x4 _228 = transpose(float4x3(float3(_210[0], _210[1], _210[2]), float3(_210[3], _210[4], _210[5]), float3(_210[6], _210[7], _210[8]), float3(_210[9], _210[10], _210[11])));
    float3 _232 = float3(_228[0].w, _228[1].w, _228[2].w);
    float3 _233 = _232 * 0.4000000059604644775390625f;
    int3 _235 = int3(floor(_233));
    float3 _236 = frac(_233);
    uint3 _237 = uint3(_235);
    uint _240 = _237.z;
    uint _242 = _240 + (_240 << 10u);
    uint _244 = _242 ^ (_242 >> 6u);
    uint _246 = _244 + (_244 << 3u);
    uint _248 = _246 ^ (_246 >> 11u);
    uint _254 = (((_237.y * 1664525u) + (_248 + (_248 << 15u))) + 1013904223u) * 1664525u;
    uint _256 = _254 ^ (_254 >> 11u);
    uint _259 = _256 ^ ((_256 << 7u) & 2636928640u);
    uint _262 = _259 ^ ((_259 << 15u) & 4022730752u);
    uint _268 = (((_237.x * 1664525u) + (_262 ^ (_262 >> 18u))) + 1013904223u) * 1664525u;
    uint _270 = _268 ^ (_268 >> 11u);
    uint _273 = _270 ^ ((_270 << 7u) & 2636928640u);
    uint _276 = _273 ^ ((_273 << 15u) & 4022730752u);
    uint _278 = _276 ^ (_276 >> 18u);
    uint _280 = _278 + (_278 << 10u);
    uint _282 = _280 ^ (_280 >> 6u);
    uint _284 = _282 + (_282 << 3u);
    uint _286 = _284 ^ (_284 >> 11u);
    uint3 _299 = uint3(_235 + int3(1, 0, 0));
    uint _302 = _299.z;
    uint _304 = _302 + (_302 << 10u);
    uint _306 = _304 ^ (_304 >> 6u);
    uint _308 = _306 + (_306 << 3u);
    uint _310 = _308 ^ (_308 >> 11u);
    uint _316 = (((_299.y * 1664525u) + (_310 + (_310 << 15u))) + 1013904223u) * 1664525u;
    uint _318 = _316 ^ (_316 >> 11u);
    uint _321 = _318 ^ ((_318 << 7u) & 2636928640u);
    uint _324 = _321 ^ ((_321 << 15u) & 4022730752u);
    uint _330 = (((_299.x * 1664525u) + (_324 ^ (_324 >> 18u))) + 1013904223u) * 1664525u;
    uint _332 = _330 ^ (_330 >> 11u);
    uint _335 = _332 ^ ((_332 << 7u) & 2636928640u);
    uint _338 = _335 ^ ((_335 << 15u) & 4022730752u);
    uint _340 = _338 ^ (_338 >> 18u);
    uint _342 = _340 + (_340 << 10u);
    uint _344 = _342 ^ (_342 >> 6u);
    uint _346 = _344 + (_344 << 3u);
    uint _348 = _346 ^ (_346 >> 11u);
    uint3 _361 = uint3(_235 + int3(0, 1, 0));
    uint _364 = _361.z;
    uint _366 = _364 + (_364 << 10u);
    uint _368 = _366 ^ (_366 >> 6u);
    uint _370 = _368 + (_368 << 3u);
    uint _372 = _370 ^ (_370 >> 11u);
    uint _378 = (((_361.y * 1664525u) + (_372 + (_372 << 15u))) + 1013904223u) * 1664525u;
    uint _380 = _378 ^ (_378 >> 11u);
    uint _383 = _380 ^ ((_380 << 7u) & 2636928640u);
    uint _386 = _383 ^ ((_383 << 15u) & 4022730752u);
    uint _392 = (((_361.x * 1664525u) + (_386 ^ (_386 >> 18u))) + 1013904223u) * 1664525u;
    uint _394 = _392 ^ (_392 >> 11u);
    uint _397 = _394 ^ ((_394 << 7u) & 2636928640u);
    uint _400 = _397 ^ ((_397 << 15u) & 4022730752u);
    uint _402 = _400 ^ (_400 >> 18u);
    uint _404 = _402 + (_402 << 10u);
    uint _406 = _404 ^ (_404 >> 6u);
    uint _408 = _406 + (_406 << 3u);
    uint _410 = _408 ^ (_408 >> 11u);
    uint3 _423 = uint3(_235 + int3(1, 1, 0));
    uint _426 = _423.z;
    uint _428 = _426 + (_426 << 10u);
    uint _430 = _428 ^ (_428 >> 6u);
    uint _432 = _430 + (_430 << 3u);
    uint _434 = _432 ^ (_432 >> 11u);
    uint _440 = (((_423.y * 1664525u) + (_434 + (_434 << 15u))) + 1013904223u) * 1664525u;
    uint _442 = _440 ^ (_440 >> 11u);
    uint _445 = _442 ^ ((_442 << 7u) & 2636928640u);
    uint _448 = _445 ^ ((_445 << 15u) & 4022730752u);
    uint _454 = (((_423.x * 1664525u) + (_448 ^ (_448 >> 18u))) + 1013904223u) * 1664525u;
    uint _456 = _454 ^ (_454 >> 11u);
    uint _459 = _456 ^ ((_456 << 7u) & 2636928640u);
    uint _462 = _459 ^ ((_459 << 15u) & 4022730752u);
    uint _464 = _462 ^ (_462 >> 18u);
    uint _466 = _464 + (_464 << 10u);
    uint _468 = _466 ^ (_466 >> 6u);
    uint _470 = _468 + (_468 << 3u);
    uint _472 = _470 ^ (_470 >> 11u);
    uint3 _485 = uint3(_235 + int3(0, 0, 1));
    uint _488 = _485.z;
    uint _490 = _488 + (_488 << 10u);
    uint _492 = _490 ^ (_490 >> 6u);
    uint _494 = _492 + (_492 << 3u);
    uint _496 = _494 ^ (_494 >> 11u);
    uint _502 = (((_485.y * 1664525u) + (_496 + (_496 << 15u))) + 1013904223u) * 1664525u;
    uint _504 = _502 ^ (_502 >> 11u);
    uint _507 = _504 ^ ((_504 << 7u) & 2636928640u);
    uint _510 = _507 ^ ((_507 << 15u) & 4022730752u);
    uint _516 = (((_485.x * 1664525u) + (_510 ^ (_510 >> 18u))) + 1013904223u) * 1664525u;
    uint _518 = _516 ^ (_516 >> 11u);
    uint _521 = _518 ^ ((_518 << 7u) & 2636928640u);
    uint _524 = _521 ^ ((_521 << 15u) & 4022730752u);
    uint _526 = _524 ^ (_524 >> 18u);
    uint _528 = _526 + (_526 << 10u);
    uint _530 = _528 ^ (_528 >> 6u);
    uint _532 = _530 + (_530 << 3u);
    uint _534 = _532 ^ (_532 >> 11u);
    uint3 _547 = uint3(_235 + int3(1, 0, 1));
    uint _550 = _547.z;
    uint _552 = _550 + (_550 << 10u);
    uint _554 = _552 ^ (_552 >> 6u);
    uint _556 = _554 + (_554 << 3u);
    uint _558 = _556 ^ (_556 >> 11u);
    uint _564 = (((_547.y * 1664525u) + (_558 + (_558 << 15u))) + 1013904223u) * 1664525u;
    uint _566 = _564 ^ (_564 >> 11u);
    uint _569 = _566 ^ ((_566 << 7u) & 2636928640u);
    uint _572 = _569 ^ ((_569 << 15u) & 4022730752u);
    uint _578 = (((_547.x * 1664525u) + (_572 ^ (_572 >> 18u))) + 1013904223u) * 1664525u;
    uint _580 = _578 ^ (_578 >> 11u);
    uint _583 = _580 ^ ((_580 << 7u) & 2636928640u);
    uint _586 = _583 ^ ((_583 << 15u) & 4022730752u);
    uint _588 = _586 ^ (_586 >> 18u);
    uint _590 = _588 + (_588 << 10u);
    uint _592 = _590 ^ (_590 >> 6u);
    uint _594 = _592 + (_592 << 3u);
    uint _596 = _594 ^ (_594 >> 11u);
    uint3 _609 = uint3(_235 + int3(0, 1, 1));
    uint _612 = _609.z;
    uint _614 = _612 + (_612 << 10u);
    uint _616 = _614 ^ (_614 >> 6u);
    uint _618 = _616 + (_616 << 3u);
    uint _620 = _618 ^ (_618 >> 11u);
    uint _626 = (((_609.y * 1664525u) + (_620 + (_620 << 15u))) + 1013904223u) * 1664525u;
    uint _628 = _626 ^ (_626 >> 11u);
    uint _631 = _628 ^ ((_628 << 7u) & 2636928640u);
    uint _634 = _631 ^ ((_631 << 15u) & 4022730752u);
    uint _640 = (((_609.x * 1664525u) + (_634 ^ (_634 >> 18u))) + 1013904223u) * 1664525u;
    uint _642 = _640 ^ (_640 >> 11u);
    uint _645 = _642 ^ ((_642 << 7u) & 2636928640u);
    uint _648 = _645 ^ ((_645 << 15u) & 4022730752u);
    uint _650 = _648 ^ (_648 >> 18u);
    uint _652 = _650 + (_650 << 10u);
    uint _654 = _652 ^ (_652 >> 6u);
    uint _656 = _654 + (_654 << 3u);
    uint _658 = _656 ^ (_656 >> 11u);
    uint3 _671 = uint3(_235 + int3(1, 1, 1));
    uint _674 = _671.z;
    uint _676 = _674 + (_674 << 10u);
    uint _678 = _676 ^ (_676 >> 6u);
    uint _680 = _678 + (_678 << 3u);
    uint _682 = _680 ^ (_680 >> 11u);
    uint _688 = (((_671.y * 1664525u) + (_682 + (_682 << 15u))) + 1013904223u) * 1664525u;
    uint _690 = _688 ^ (_688 >> 11u);
    uint _693 = _690 ^ ((_690 << 7u) & 2636928640u);
    uint _696 = _693 ^ ((_693 << 15u) & 4022730752u);
    uint _702 = (((_671.x * 1664525u) + (_696 ^ (_696 >> 18u))) + 1013904223u) * 1664525u;
    uint _704 = _702 ^ (_702 >> 11u);
    uint _707 = _704 ^ ((_704 << 7u) & 2636928640u);
    uint _710 = _707 ^ ((_707 << 15u) & 4022730752u);
    uint _712 = _710 ^ (_710 >> 18u);
    uint _714 = _712 + (_712 << 10u);
    uint _716 = _714 ^ (_714 >> 6u);
    uint _718 = _716 + (_716 << 3u);
    uint _720 = _718 ^ (_718 >> 11u);
    float3 _735 = (_236 * _236) * (3.0f.xxx - (_236 * 2.0f));
    float _736 = _735.x;
    float _737 = 1.0f - _736;
    float _739 = _735.y;
    float _740 = 1.0f - _739;
    float _742 = _735.z;
    float _743 = 1.0f - _742;
    float2 _760 = ((((((float2(asfloat((_278 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_286 + (_286 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _737) * _740) * _743) + (((float2(asfloat((_340 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_348 + (_348 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _736) * _740) * _743)) + (((float2(asfloat((_402 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_410 + (_410 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _737) * _739) * _743)) + (((float2(asfloat((_464 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_472 + (_472 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _736) * _739) * _743)) + (((float2(asfloat((_526 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_534 + (_534 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _737) * _740) * _742);
    float2 _775 = clamp(((((_760 + (((float2(asfloat((_588 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_596 + (_596 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _736) * _740) * _742)) + (((float2(asfloat((_650 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_658 + (_658 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _737) * _739) * _742)) + (((float2(asfloat((_712 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_720 + (_720 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _736) * _739) * _742)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _782 = (float3(_775.x, 0.0f, _775.y) * 0.0599999986588954925537109375f) + float3(0.0f, dot(instanced_wall_data.Load<float4>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 48), 0.25f.xxxx), 0.0f);
    float3 _783 = _232 + _782;
    float3x4 _785 = _228;
    _785[0].w = _783.x;
    float3x4 _787 = _785;
    _787[1].w = _783.y;
    float3x4 _789 = _787;
    _789[2].w = _783.z;
    uint3 _795 = uint3((uint(gl_VertexIndex) & 4u) >> 2u, (uint(gl_VertexIndex) & 2u) >> 1u, uint(gl_VertexIndex) & 1u);
    float3 _796 = mul(_789, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4 _801 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _814 = mul((_801.xyz / _801.w.xxx) - _796, float3x3(_785[0].xyz, _787[1].xyz, _789[2].xyz));
    bool3 _815 = bool3(_814.x > 0.0f.xxx.x, _814.y > 0.0f.xxx.y, _814.z > 0.0f.xxx.z);
    uint3 _816 = uint3(1u, 1u, 1u) - _795;
    float3 _819 = float3(uint3(_815.x ? _795.x : _816.x, _815.y ? _795.y : _816.y, _815.z ? _795.z : _816.z)) - 0.5f.xxx;
    float4 _824 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(_819, 1.0f));
    float3 _825 = _824.xyz;
    int _828 = int(instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 92));
    float3 _845;
    if ((_828 & 32) != 0)
    {
        float3 _844 = _825;
        _844.z = _824.z * lerp(instanced_wall_data.Load<float>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 64), instanced_wall_data.Load<float>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 68), 0.5f - _824.x);
        _845 = _844;
    }
    else
    {
        _845 = _825;
    }
    float3 _871;
    if ((_828 & 4) != 0)
    {
        uint _853 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 76) + (instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 76) << 10u);
        uint _855 = _853 ^ (_853 >> 6u);
        uint _857 = _855 + (_855 << 3u);
        uint _859 = _857 ^ (_857 >> 11u);
        float _867 = ((asfloat(((_859 + (_859 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.89999997615814208984375f) ? 1.0f : 1.099999904632568359375f;
        float3 _868 = float3(_867, _867, 1.0f);
        bool3 _869 = ((_828 & 1) != 0).xxx;
        _871 = float3(_869.x ? 1.0499999523162841796875f.xxx.x : _868.x, _869.y ? 1.0499999523162841796875f.xxx.y : _868.y, _869.z ? 1.0499999523162841796875f.xxx.z : _868.z);
    }
    else
    {
        _871 = 1.0f.xxx;
    }
    float3 _877 = mul(_789, float4(_845 * _871, 1.0f));
    float4 _881 = float4(_877, 1.0f);
    float _887 = mul(float3x3(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 1.0f, 0.0f), float3(0.0f, 0.0f, 1.0f)), _819).x - (-0.5f);
    uint _896 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 76) & 65535u;
    int _903;
    if (instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 76) < 10u)
    {
        _903 = _828 | 512;
    }
    else
    {
        _903 = _828;
    }
    float _925;
    if (_887 < 0.5f)
    {
        _925 = lerp(instanced_wall_data.Load<float3>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 64).x, instanced_wall_data.Load<float3>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 64).y, _887 * 2.0f);
    }
    else
    {
        _925 = lerp(instanced_wall_data.Load<float3>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 64).y, instanced_wall_data.Load<float3>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 64).z, (_887 * 2.0f) - 1.0f);
    }
    int _958;
    int _959;
    if ((instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 80) & 2147483648u) != 0u)
    {
        bool _953;
        if ((instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 80) & 1073741824u) != 0u)
        {
            _953 = ((instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 80) >> 4u) & 65535u) == frame_constants_highlighted_transient_stair_edge_id;
        }
        else
        {
            _953 = false;
        }
        int _957;
        if (_953)
        {
            _957 = _903 | 2048;
        }
        else
        {
            _957 = _903;
        }
        _958 = int(instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 80) & 15u);
        _959 = _957;
    }
    else
    {
        uint _940;
        if ((_903 & 2) != 0)
        {
            _940 = roof_color_ids.Load<uint>(instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 80) * 4 + 0);
        }
        else
        {
            _940 = wall_color_ids.Load<uint>(instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 80) * 4 + 0);
        }
        _958 = int(_940);
        _959 = _903;
    }
    uint _960 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 80) & 65535u;
    uint _963 = _960 ^ 7993234u;
    uint _965 = _963 + (_963 << 10u);
    uint _967 = _965 ^ (_965 >> 6u);
    uint _969 = _967 + (_967 << 3u);
    uint _971 = _969 ^ (_969 >> 11u);
    uint _978 = _960 ^ 3450234u;
    uint _980 = _978 + (_978 << 10u);
    uint _982 = _980 ^ (_980 >> 6u);
    uint _984 = _982 + (_982 << 3u);
    uint _986 = _984 ^ (_984 >> 11u);
    float2 _1000 = (((float2(lerp(instanced_wall_data.Load<float2>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 84).x, instanced_wall_data.Load<float2>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 84).y, _887), _877.y) * 0.100000001490116119384765625f) * 0.699999988079071044921875f) + (float(_960) * 0.20000000298023223876953125f).xx) + float2(asfloat(((_971 + (_971 << 15u)) & 8388607u) | 1065353216u) - 1.0f, asfloat(((_986 + (_986 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    uint _1001 = _896 ^ 292478890u;
    uint _1003 = _1001 + (_1001 << 10u);
    uint _1005 = _1003 ^ (_1003 >> 6u);
    uint _1007 = _1005 + (_1005 << 3u);
    uint _1009 = _1007 ^ (_1007 >> 11u);
    float _1015 = asfloat(((_1009 + (_1009 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 _1024;
    if (_1015 > 0.89999997615814208984375f)
    {
        _1024 = _1000 * 3.0f;
    }
    else
    {
        _1024 = _1000 + (0.61000001430511474609375f * _1015).xx;
    }
    float2 _1030;
    if (int((_959 & 512) != 0) == 1)
    {
        _1030 = _1024 * 2.2000000476837158203125f;
    }
    else
    {
        _1030 = _1024;
    }
    bool _1032 = (_903 & 32) != 0;
    float2 _1055;
    if (_1032)
    {
        uint _1040 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 76) + (instanced_wall_data.Load<uint>(instance_list.Load<uint>(_204 * 4 + 0) * 96 + 76) << 10u);
        uint _1042 = _1040 ^ (_1040 >> 6u);
        uint _1044 = _1042 + (_1042 << 3u);
        uint _1046 = _1044 ^ (_1044 >> 11u);
        _1055 = ((_881.xz * 0.699999988079071044921875f) * 0.300000011920928955078125f) + (asfloat(((_1046 + (_1046 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx;
    }
    else
    {
        _1055 = _1030;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _881);
    out_var_C0 = _896;
    out_var_C1 = 0.0f.xxx;
    out_var_C2 = _881.xyz;
    out_var_C5 = _782;
    out_var_C7 = _1032 ? (-100.0f) : _925;
    out_var_C10 = _1055;
    out_var_C11 = _958;
    out_var_C12 = float4(_796.x, _192, _796.z, 1.0f).xz;
    out_var_C13 = _959;
    out_var_C14 = instance_list.Load<uint>(_204 * 4 + 0);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C10 = out_var_C10;
    stage_output.out_var_C11 = out_var_C11;
    stage_output.out_var_C12 = out_var_C12;
    stage_output.out_var_C13 = out_var_C13;
    stage_output.out_var_C14 = out_var_C14;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

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

struct Affine3Packed
{
    float data[12];
};

struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

struct PlaneEq
{
    float4 abcd;
};

static uint _219;
static float3 _237;

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
    uint _Globals_highlight_mode : packoffset(c0.y);
    int _Globals_highlight_bricks_to_delete : packoffset(c0.z);
    float3 _Globals_brush_position : packoffset(c1);
    uint _Globals_bool_render_shadows : packoffset(c1.w);
    float _Globals_eraser_mult : packoffset(c2);
};

ByteAddressBuffer instanced_wall_data : register(t19, space0);
cbuffer color_library : register(b21, space0)
{
    float4 color_library_temp_color_1 : packoffset(c0);
    float4 color_library_temp_color_2 : packoffset(c1);
    float4 color_library_temp_color_3 : packoffset(c2);
    float4 color_library_roof_color_1 : packoffset(c3);
    float4 color_library_roof_color_2 : packoffset(c4);
    float4 color_library_sky_low_day : packoffset(c5);
    float4 color_library_sky_high_day : packoffset(c6);
    float4 color_library_sun_tint_day : packoffset(c7);
    float4 color_library_sky_low_dusk : packoffset(c8);
    float4 color_library_sky_high_dusk : packoffset(c9);
    float4 color_library_sun_tint_dusk : packoffset(c10);
    float4 color_library_indirect_light_tint : packoffset(c11);
    float4 color_library_fence_color_1 : packoffset(c12);
    float4 color_library_fence_color_2 : packoffset(c13);
    float4 color_library_flower_color_1 : packoffset(c14);
    float4 color_library_flower_color_2 : packoffset(c15);
    float4 color_library_canopy_color_1 : packoffset(c16);
    float4 color_library_canopy_color_2 : packoffset(c17);
    float4 color_library_grass_1 : packoffset(c18);
    float4 color_library_grass_2 : packoffset(c19);
    float4 color_library_path : packoffset(c20);
    float4 color_library_sky_base : packoffset(c21);
    float4 color_library_sky_horizon : packoffset(c22);
    float4 color_library_sky_clouds_1 : packoffset(c23);
    float4 color_library_sky_clouds_2 : packoffset(c24);
    float4 color_library_terrain_1 : packoffset(c25);
    float4 color_library_terrain_2 : packoffset(c26);
    float4 color_library_terrain_flattened : packoffset(c27);
    float4 color_library_wall_1 : packoffset(c28);
    float4 color_library_wall_2 : packoffset(c29);
    float4 color_library_wall_moss : packoffset(c30);
    float4 color_library_wall_bricks : packoffset(c31);
    float4 color_library_ivy_branch : packoffset(c32);
    float4 color_library_ivy_leaf_1 : packoffset(c33);
    float4 color_library_ivy_leaf_2 : packoffset(c34);
    float4 color_library_terrain_highlight : packoffset(c35);
    float4 color_library_night_tint : packoffset(c36);
};

SamplerState sampler_llr : register(s1, space0);
Texture2DArray<float4> brick_colors : register(t15, space0);
Texture2DArray<float4> roof_colors : register(t17, space0);

static float4 gl_FragCoord;
static float gl_FragDepth;
static uint in_var_C0;
static float3 in_var_C2;
static float3 in_var_C5;
static float in_var_C7;
static float2 in_var_C10;
static int in_var_C11;
static float2 in_var_C12;
static int in_var_C13;
static uint in_var_C14;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] nointerpolation uint in_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(5)]] nointerpolation float3 in_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float in_var_C7 : TEXCOORD7;
    [[vk::location(10)]] float2 in_var_C10 : TEXCOORD10;
    [[vk::location(11)]] nointerpolation int in_var_C11 : TEXCOORD11;
    [[vk::location(12)]] nointerpolation float2 in_var_C12 : TEXCOORD12;
    [[vk::location(13)]] nointerpolation int in_var_C13 : TEXCOORD13;
    [[vk::location(14)]] nointerpolation uint in_var_C14 : TEXCOORD14;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
    float gl_FragDepth : SV_DepthLessEqual;
};

void frag_main()
{
    bool _259 = (in_var_C13 & 8) != 0;
    bool _261 = (in_var_C13 & 16) != 0;
    bool _265 = (in_var_C13 & 2048) != 0;
    float3 _266 = ddx(in_var_C2);
    float3 _267 = ddy(in_var_C2);
    float3 _269 = cross(_266, -_267);
    bool _271 = (in_var_C13 & 4) != 0;
    float3 _295;
    if (_271)
    {
        uint _277 = instanced_wall_data.Load<uint>(in_var_C14 * 96 + 76) + (instanced_wall_data.Load<uint>(in_var_C14 * 96 + 76) << 10u);
        uint _279 = _277 ^ (_277 >> 6u);
        uint _281 = _279 + (_279 << 3u);
        uint _283 = _281 ^ (_281 >> 11u);
        float _291 = ((asfloat(((_283 + (_283 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.89999997615814208984375f) ? 1.0f : 1.099999904632568359375f;
        float3 _292 = float3(_291, _291, 1.0f);
        bool3 _293 = ((in_var_C13 & 1) != 0).xxx;
        _295 = float3(_293.x ? 1.0499999523162841796875f.xxx.x : _292.x, _293.y ? 1.0499999523162841796875f.xxx.y : _292.y, _293.z ? 1.0499999523162841796875f.xxx.z : _292.z);
    }
    else
    {
        _295 = 1.0f.xxx;
    }
    uint _296 = _271 ? 1065353216u : _219;
    uint _297 = _271 ? 8388607u : _219;
    bool _298 = _271 ? true : false;
    float4x4 _302 = float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f));
    _302[0].x = _295.x;
    _302[1].y = _295.y;
    _302[2].z = _295.z;
    _302[3].w = 1.0f;
    float _307[12];
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _307[_3ident] = instanced_wall_data.Load<float>(_3ident * 4 + in_var_C14 * 96 + 0);
    }
    float3x4 _326 = mul(transpose(float4x3(float3(_307[0], _307[1], _307[2]), float3(_307[3], _307[4], _307[5]), float3(_307[6], _307[7], _307[8]), float3(_307[9], _307[10], _307[11]))), _302);
    float3 _331 = float3(_326[0].w, _326[1].w, _326[2].w) + in_var_C5;
    float3x4 _333 = _326;
    _333[0].w = _331.x;
    float3x4 _335 = _333;
    _335[1].w = _331.y;
    float3x4 _337 = _335;
    _337[2].w = _331.z;
    float3x3 _344 = float3x3(_333[0].xyz, _335[1].xyz, _337[2].xyz);
    float3 _345 = mul(_344, float3(1.0f, 0.0f, 0.0f));
    float3 _346 = mul(_344, float3(0.0f, 1.0f, 0.0f));
    float3 _347 = mul(_344, float3(0.0f, 0.0f, 1.0f));
    float3 _350 = cross(_345, _346);
    float3 _353 = (1.0f / dot(_347, _350)).xxx;
    float3x3 _357 = float3x3(cross(_346, _347) * _353, cross(_347, _345) * _353, _350 * _353);
    float4 _362 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _366 = _362.xyz / _362.w.xxx;
    float3 _371 = mul(_357, _366 - mul(_337, float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 _372 = mul(_357, normalize(in_var_C2 - _366));
    float3 _374 = (0.5f.xxx - _371) / _372;
    float3 _376 = ((-0.5f).xxx - _371) / _372;
    float3 _377 = min(_374, _376);
    float3 _378 = max(_374, _376);
    float _383 = max(max(_377.x, _377.y), _377.z);
    float3 _390 = _371 + (_372 * _383);
    float3 _391 = abs(_390);
    float3 _500;
    float _501;
    float3 _502;
    if (dot(1.0f.xxx, select(bool3(_391.x > 0.425000011920928955078125f.xxx.x, _391.y > 0.425000011920928955078125f.xxx.y, _391.z > 0.425000011920928955078125f.xxx.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f))) >= 2.0f)
    {
        int3 _399 = int3(sign(_390));
        float3 _400 = float3(_399);
        float _404 = _400.x;
        PlaneEq _408 = { float4(0.0f, _400.yz, -0.925000011920928955078125f) };
        PlaneEq _409 = { float4(_404, 0.0f, _400.z, -0.925000011920928955078125f) };
        PlaneEq _410 = { float4(_404, _400.y, 0.0f, -0.925000011920928955078125f) };
        PlaneEq _411 = { float4(_404, _400.yz, -1.35000002384185791015625f) };
        PlaneEq _412[4] = { _408, _409, _410, _411 };
        PlaneEq _240[4] = _412;
        float3 _418 = max(_391.x, max(_391.y, _391.z)).xxx;
        float3 _424;
        bool _427;
        float _429;
        float _431;
        _424 = float3(int3(bool3(_391.x == _418.x, _391.y == _418.y, _391.z == _418.z)) * _399);
        _427 = false;
        _429 = min(min(_378.x, _378.y), _378.z);
        _431 = _383;
        bool _428;
        float _430;
        float _432;
        float3 _425;
        for (uint _433 = 0u; _433 < 4u; _424 = _425, _427 = _428, _429 = _430, _431 = _432, _433++)
        {
            float _447 = (_240[_433].abcd.w + dot(_371, _240[_433].abcd.xyz)) / (-dot(_372, _240[_433].abcd.xyz));
            float3 _449 = (_240[_433].abcd.xyz * (-1.0f)).xyz;
            float _454 = (_240[_433].abcd.w + dot(_371, _449)) / (-dot(_372, _449));
            float _455 = min(_447, _454);
            bool _457 = _455 > _431;
            if (_457)
            {
                _425 = _449 * float(-int(sign(dot(_449, _372))));
            }
            else
            {
                _425 = _424;
            }
            _428 = _457 ? true : _427;
            _432 = _457 ? _455 : _431;
            _430 = min(_429, max(_447, _454));
        }
        float3 _497;
        float _498;
        float3 _499;
        if (_431 < _429)
        {
            if (_427)
            {
                float3 _479 = mul(_337, float4(_371 + (_372 * _431), 1.0f));
                float4 _489 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_sample, mul(frame_constants_view_constants[_Globals_current_view_index].world_to_view, float4(_479, 1.0f)));
                float3 _493 = _489.xyz / _489.w.xxx;
                float _494 = _493.z;
                _497 = _479;
                _498 = (_494 < gl_FragCoord.z) ? _494 : gl_FragCoord.z;
                _499 = mul(_424, _357);
            }
            else
            {
                _497 = in_var_C2;
                _498 = gl_FragCoord.z;
                _499 = _269;
            }
        }
        else
        {
            discard;
        }
        _500 = _497;
        _501 = _498;
        _502 = _499;
    }
    else
    {
        _500 = in_var_C2;
        _501 = gl_FragCoord.z;
        _502 = _269;
    }
    float3 _508 = max(_391.x, max(_391.y, _391.z)).xxx;
    float3 _511 = select(bool3(_391.x != _508.x, _391.y != _508.y, _391.z != _508.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * _390;
    float3 _512 = abs(_511);
    float2 _519 = _500.xy * 2.5f;
    float2 _523 = floor(_519 + dot(_519, 0.3660254180431365966796875f.xx).xx);
    float2 _527 = (_519 - _523) + dot(_523, 0.211324870586395263671875f.xx).xx;
    float _528 = _527.x;
    float _529 = _527.y;
    bool2 _531 = (_528 > _529).xx;
    float2 _532 = float2(_531.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, _531.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
    float4 _534 = _527.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
    float2 _536 = _534.xy - _532;
    float4 _537 = float4(_536.x, _536.y, _534.z, _534.w);
    float2 _538 = fmod(_523, 289.0f.xx);
    float3 _543 = _538.y.xxx + float3(0.0f, _532.y, 1.0f);
    float3 _553 = (fmod(((_543 * 34.0f) + 1.0f.xxx) * _543, 289.0f.xxx) + _538.x.xxx) + float3(0.0f, _532.x, 1.0f);
    float2 _559 = _536.xy;
    float2 _561 = _534.zw;
    float3 _565 = max(0.5f.xxx - float3(dot(_527, _527), dot(_559, _559), dot(_561, _561)), 0.0f.xxx);
    float3 _566 = _565 * _565;
    float3 _570 = frac(fmod(((_553 * 34.0f) + 1.0f.xxx) * _553, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
    float3 _571 = _570 - 1.0f.xxx;
    float3 _573 = abs(_571) - 0.5f.xxx;
    float3 _576 = _571 - floor(_570 + (-0.5f).xxx);
    float3 _588;
    _588.x = (_576.x * _528) + (_573.x * _529);
    float2 _595 = (_576.yz * _537.xz) + (_573.yz * _537.yw);
    float3 _600 = _511 + (dot((_566 * _566) * (1.792842864990234375f.xxx - (((_576 * _576) + (_573 * _573)) * 0.8537347316741943359375f)), float3(_588.x, _595.x, _595.y)) * 26.0f).xxx;
    bool _604 = lerp(max(_512.x, max(_512.y, _512.z)), 1.2999999523162841796875f * dot(_600, _600), 0.4000000059604644775390625f) > 0.36000001430511474609375f;
    bool _609;
    if (!_259)
    {
        _609 = in_var_C7 > (-2.5f);
    }
    else
    {
        _609 = false;
    }
    bool _614;
    if (_609)
    {
        _614 = _500.y < in_var_C7;
    }
    else
    {
        _614 = false;
    }
    if (_614)
    {
        discard;
    }
    else
    {
        bool _622;
        if (_259)
        {
            _622 = _500.y > in_var_C7;
        }
        else
        {
            _622 = false;
        }
        if (_622)
        {
            discard;
        }
    }
    uint _628 = uint(in_var_C11);
    float3 _848;
    if ((in_var_C13 & 2) != 0)
    {
        _848 = roof_colors.Sample(sampler_llr, float3((float2(in_var_C0.xx) * 0.20000000298023223876953125f) + (in_var_C2.xz * 0.20000000298023223876953125f), float(_628))).xyz * 0.699999988079071044921875f;
    }
    else
    {
        float2 _640;
        if (_604)
        {
            _640 = in_var_C10 + 0.20999999344348907470703125f.xx;
        }
        else
        {
            _640 = in_var_C10;
        }
        bool _646;
        if (!(_628 == 1u))
        {
            _646 = _628 == 0u;
        }
        else
        {
            _646 = true;
        }
        float3 _653;
        if (_646)
        {
            _653 = pow(color_library_wall_bricks.xyz, 1.2999999523162841796875f.xxx);
        }
        else
        {
            _653 = pow(color_library_wall_moss.xyz, 1.0f.xxx) * 0.800000011920928955078125f;
        }
        int _654 = int(_628);
        bool _660;
        if (!(_654 == 0))
        {
            _660 = _654 == 4;
        }
        else
        {
            _660 = true;
        }
        bool _665;
        if (!_660)
        {
            _665 = _654 == 10;
        }
        else
        {
            _665 = true;
        }
        bool _670;
        if (!_665)
        {
            _670 = _654 == 12;
        }
        else
        {
            _670 = true;
        }
        float3 _761;
        if (_670)
        {
            _761 = brick_colors.Sample(sampler_llr, float3(_640 + 0.85000002384185791015625f.xx, 0.0f)).xyz;
        }
        else
        {
            bool _679;
            if (!(_654 == 1))
            {
                _679 = _654 == 5;
            }
            else
            {
                _679 = true;
            }
            float3 _751;
            if (_679)
            {
                _751 = pow(brick_colors.Sample(sampler_llr, float3(_640 + 0.449999988079071044921875f.xx, 1.0f)).xyz, 1.10000002384185791015625f.xxx);
            }
            else
            {
                bool _688;
                if (!(_654 == 2))
                {
                    _688 = _654 == 7;
                }
                else
                {
                    _688 = true;
                }
                float3 _740;
                if (_688)
                {
                    _740 = brick_colors.Sample(sampler_llr, float3(_640, 2.0f)).xyz;
                }
                else
                {
                    bool _697;
                    if (!(_654 == 3))
                    {
                        _697 = _654 == 6;
                    }
                    else
                    {
                        _697 = true;
                    }
                    bool _702;
                    if (!_697)
                    {
                        _702 = _654 == 8;
                    }
                    else
                    {
                        _702 = true;
                    }
                    bool _707;
                    if (!_702)
                    {
                        _707 = _654 == 9;
                    }
                    else
                    {
                        _707 = true;
                    }
                    float3 _731;
                    if (_707)
                    {
                        _731 = brick_colors.Sample(sampler_llr, float3(_640, 3.0f)).xyz;
                    }
                    else
                    {
                        float3 _722;
                        if (_654 == 11)
                        {
                            _722 = brick_colors.Sample(sampler_llr, float3(_640, 4.0f)).xyz;
                        }
                        else
                        {
                            _722 = 0.0f.xxx;
                        }
                        _731 = _722;
                    }
                    _740 = _731;
                }
                _751 = _740;
            }
            _761 = _751;
        }
        bool _763 = !(_628 == 7u);
        bool _767;
        if (_763)
        {
            _767 = _628 == 2u;
        }
        else
        {
            _767 = true;
        }
        bool _772;
        if (!_767)
        {
            _772 = _628 == 11u;
        }
        else
        {
            _772 = true;
        }
        bool3 _773 = _772.xxx;
        float3 _799;
        uint _800;
        uint _801;
        bool _802;
        if ((in_var_C13 & 512) != 0)
        {
            uint _777 = in_var_C0 ^ 8902390u;
            uint _779 = _777 + (_777 << 10u);
            uint _781 = _779 ^ (_779 >> 6u);
            uint _783 = _781 + (_781 << 3u);
            uint _785 = _783 ^ (_783 >> 11u);
            uint _789 = _298 ? _297 : 8388607u;
            uint _790 = _298 ? _296 : 1065353216u;
            _799 = lerp(_761, float3(_773.x ? _761.x : _653.x, _773.y ? _761.y : _653.y, _773.z ? _761.z : _653.z), lerp(0.0f, 0.5f, smoothstep(0.20000000298023223876953125f, 0.800000011920928955078125f, asfloat(((_785 + (_785 << 15u)) & _789) | _790) - 1.0f)).xxx);
            _800 = _790;
            _801 = _789;
            _802 = _298 ? _298 : true;
        }
        else
        {
            _799 = _761;
            _800 = _296;
            _801 = _297;
            _802 = _298;
        }
        float3 _831;
        if (_604)
        {
            bool _808;
            if (_763)
            {
                _808 = _628 == 2u;
            }
            else
            {
                _808 = true;
            }
            uint _810 = in_var_C0 ^ 239085u;
            uint _812 = _810 + (_810 << 10u);
            uint _814 = _812 ^ (_812 >> 6u);
            uint _816 = _814 + (_814 << 3u);
            uint _818 = _816 ^ (_816 >> 11u);
            _831 = lerp(_799, color_library_wall_1.xyz, lerp(0.0f, _808 ? 0.20000000298023223876953125f : 0.4000000059604644775390625f, smoothstep(0.0f, 0.800000011920928955078125f, asfloat(((_818 + (_818 << 15u)) & (_802 ? _801 : 8388607u)) | (_802 ? _800 : 1065353216u)) - 1.0f)).xxx);
        }
        else
        {
            _831 = _799;
        }
        _848 = _831;
    }
    float3 _849 = normalize(_502);
    bool _888;
    float3 _889;
    if (!((in_var_C13 & 64) != 0))
    {
        bool _866;
        float3 _867;
        if (_Globals_highlight_mode == 1u)
        {
            _866 = true;
            _867 = lerp(_848, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
        }
        else
        {
            bool _859 = _Globals_highlight_mode == 2u;
            float3 _863;
            if (_859)
            {
                _863 = lerp(_848, 0.5f.xxx, 0.100000001490116119384765625f.xxx);
            }
            else
            {
                _863 = _848;
            }
            _866 = _859 ? true : false;
            _867 = _863;
        }
        bool _886;
        float3 _887;
        if (_Globals_highlight_bricks_to_delete == 1)
        {
            bool _880 = distance(_Globals_brush_position.xz, in_var_C12) < (1.88900005817413330078125f * _Globals_eraser_mult);
            float3 _884;
            if (_880)
            {
                _884 = lerp(_848, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.800000011920928955078125f.xxx);
            }
            else
            {
                _884 = _867;
            }
            _886 = _880 ? true : _866;
            _887 = _884;
        }
        else
        {
            _886 = _866;
            _887 = _867;
        }
        _888 = _886;
        _889 = _887;
    }
    else
    {
        _888 = false;
        _889 = _848;
    }
    float3 _893;
    if (_265)
    {
        _893 = lerp(_848, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
    }
    else
    {
        _893 = _889;
    }
    float3 _995;
    if (frame_constants_glade_theme_id == 4u)
    {
        float2 _905 = in_var_C2.xz * 0.4000000059604644775390625f;
        float2 _909 = floor(_905 + dot(_905, 0.3660254180431365966796875f.xx).xx);
        float2 _913 = (_905 - _909) + dot(_909, 0.211324870586395263671875f.xx).xx;
        float _914 = _913.x;
        float _915 = _913.y;
        bool2 _917 = (_914 > _915).xx;
        float2 _918 = float2(_917.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, _917.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
        float4 _920 = _913.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
        float2 _922 = _920.xy - _918;
        float4 _923 = float4(_922.x, _922.y, _920.z, _920.w);
        float2 _924 = fmod(_909, 289.0f.xx);
        float3 _929 = _924.y.xxx + float3(0.0f, _918.y, 1.0f);
        float3 _939 = (fmod(((_929 * 34.0f) + 1.0f.xxx) * _929, 289.0f.xxx) + _924.x.xxx) + float3(0.0f, _918.x, 1.0f);
        float2 _945 = _922.xy;
        float2 _947 = _920.zw;
        float3 _951 = max(0.5f.xxx - float3(dot(_913, _913), dot(_945, _945), dot(_947, _947)), 0.0f.xxx);
        float3 _952 = _951 * _951;
        float3 _956 = frac(fmod(((_939 * 34.0f) + 1.0f.xxx) * _939, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
        float3 _957 = _956 - 1.0f.xxx;
        float3 _959 = abs(_957) - 0.5f.xxx;
        float3 _962 = _957 - floor(_956 + (-0.5f).xxx);
        float3 _974;
        _974.x = (_962.x * _914) + (_959.x * _915);
        float2 _981 = (_962.yz * _923.xz) + (_959.yz * _923.yw);
        _995 = lerp(_893, 0.699999988079071044921875f.xxx, (smoothstep(0.0f, 1.0f, smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C2.y) * clamp(_849.y, 0.0f, 1.0f)) * clamp(clamp((130.0f * dot((_952 * _952) * (1.792842864990234375f.xxx - (((_962 * _962) + (_959 * _959)) * 0.8537347316741943359375f)), float3(_974.x, _981.x, _981.y))) + 0.75f, 0.0f, 1.0f), _261 ? 0.0f : 0.4000000059604644775390625f, _261 ? 0.100000001490116119384765625f : 1.0f)).xxx);
    }
    else
    {
        _995 = _893;
    }
    float3 _997 = (_849 * 0.5f) + 0.5f.xxx;
    float3 _1019 = sqrt(clamp(_995, 0.0f.xxx, 1.0f.xxx));
    uint _1038 = (uint((clamp(_1019.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_1019.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_1019.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _1039 = uint2(((uint((clamp(_997.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_997.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_997.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, _1038);
    _1039.y = _1038 | (((((7u | (uint(_265 ? true : _888) << 3u)) | 0u) | 0u) | 0u) << 24u);
    out_var_SV_TARGET0 = _1039;
    gl_FragDepth = _501;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C0 = stage_input.in_var_C0;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C10 = stage_input.in_var_C10;
    in_var_C11 = stage_input.in_var_C11;
    in_var_C12 = stage_input.in_var_C12;
    in_var_C13 = stage_input.in_var_C13;
    in_var_C14 = stage_input.in_var_C14;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_FragDepth = gl_FragDepth;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

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

struct Affine3Packed
{
    float data[12];
};

struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

static float _179;

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
    uint _Globals_highlight_mode : packoffset(c0.y);
    int _Globals_highlight_bricks_to_delete : packoffset(c0.z);
    float3 _Globals_brush_position : packoffset(c1);
    uint _Globals_bool_render_shadows : packoffset(c1.w);
    float _Globals_eraser_mult : packoffset(c2);
};

ByteAddressBuffer instanced_wall_data : register(t19, space0);
ByteAddressBuffer instance_list : register(t20, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static uint out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C5;
static float out_var_C7;
static float2 out_var_C10;
static int out_var_C11;
static float2 out_var_C12;
static int out_var_C13;
static uint out_var_C14;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] nointerpolation uint out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(5)]] nointerpolation float3 out_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float out_var_C7 : TEXCOORD7;
    [[vk::location(10)]] float2 out_var_C10 : TEXCOORD10;
    [[vk::location(11)]] nointerpolation int out_var_C11 : TEXCOORD11;
    [[vk::location(12)]] nointerpolation float2 out_var_C12 : TEXCOORD12;
    [[vk::location(13)]] nointerpolation int out_var_C13 : TEXCOORD13;
    [[vk::location(14)]] nointerpolation uint out_var_C14 : TEXCOORD14;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _188 = ((_Globals_current_view_index + 2u) * 2097152u) + uint(gl_InstanceIndex);
    float _194[12];
    [unroll]
    for (int _5ident = 0; _5ident < 12; _5ident++)
    {
        _194[_5ident] = instanced_wall_data.Load<float>(_5ident * 4 + instance_list.Load<uint>(_188 * 4 + 0) * 96 + 0);
    }
    float3x4 _212 = transpose(float4x3(float3(_194[0], _194[1], _194[2]), float3(_194[3], _194[4], _194[5]), float3(_194[6], _194[7], _194[8]), float3(_194[9], _194[10], _194[11])));
    float3 _216 = float3(_212[0].w, _212[1].w, _212[2].w);
    float3 _217 = _216 * 0.4000000059604644775390625f;
    int3 _219 = int3(floor(_217));
    float3 _220 = frac(_217);
    uint3 _221 = uint3(_219);
    uint _224 = _221.z;
    uint _226 = _224 + (_224 << 10u);
    uint _228 = _226 ^ (_226 >> 6u);
    uint _230 = _228 + (_228 << 3u);
    uint _232 = _230 ^ (_230 >> 11u);
    uint _238 = (((_221.y * 1664525u) + (_232 + (_232 << 15u))) + 1013904223u) * 1664525u;
    uint _240 = _238 ^ (_238 >> 11u);
    uint _243 = _240 ^ ((_240 << 7u) & 2636928640u);
    uint _246 = _243 ^ ((_243 << 15u) & 4022730752u);
    uint _252 = (((_221.x * 1664525u) + (_246 ^ (_246 >> 18u))) + 1013904223u) * 1664525u;
    uint _254 = _252 ^ (_252 >> 11u);
    uint _257 = _254 ^ ((_254 << 7u) & 2636928640u);
    uint _260 = _257 ^ ((_257 << 15u) & 4022730752u);
    uint _262 = _260 ^ (_260 >> 18u);
    uint _264 = _262 + (_262 << 10u);
    uint _266 = _264 ^ (_264 >> 6u);
    uint _268 = _266 + (_266 << 3u);
    uint _270 = _268 ^ (_268 >> 11u);
    uint3 _283 = uint3(_219 + int3(1, 0, 0));
    uint _286 = _283.z;
    uint _288 = _286 + (_286 << 10u);
    uint _290 = _288 ^ (_288 >> 6u);
    uint _292 = _290 + (_290 << 3u);
    uint _294 = _292 ^ (_292 >> 11u);
    uint _300 = (((_283.y * 1664525u) + (_294 + (_294 << 15u))) + 1013904223u) * 1664525u;
    uint _302 = _300 ^ (_300 >> 11u);
    uint _305 = _302 ^ ((_302 << 7u) & 2636928640u);
    uint _308 = _305 ^ ((_305 << 15u) & 4022730752u);
    uint _314 = (((_283.x * 1664525u) + (_308 ^ (_308 >> 18u))) + 1013904223u) * 1664525u;
    uint _316 = _314 ^ (_314 >> 11u);
    uint _319 = _316 ^ ((_316 << 7u) & 2636928640u);
    uint _322 = _319 ^ ((_319 << 15u) & 4022730752u);
    uint _324 = _322 ^ (_322 >> 18u);
    uint _326 = _324 + (_324 << 10u);
    uint _328 = _326 ^ (_326 >> 6u);
    uint _330 = _328 + (_328 << 3u);
    uint _332 = _330 ^ (_330 >> 11u);
    uint3 _345 = uint3(_219 + int3(0, 1, 0));
    uint _348 = _345.z;
    uint _350 = _348 + (_348 << 10u);
    uint _352 = _350 ^ (_350 >> 6u);
    uint _354 = _352 + (_352 << 3u);
    uint _356 = _354 ^ (_354 >> 11u);
    uint _362 = (((_345.y * 1664525u) + (_356 + (_356 << 15u))) + 1013904223u) * 1664525u;
    uint _364 = _362 ^ (_362 >> 11u);
    uint _367 = _364 ^ ((_364 << 7u) & 2636928640u);
    uint _370 = _367 ^ ((_367 << 15u) & 4022730752u);
    uint _376 = (((_345.x * 1664525u) + (_370 ^ (_370 >> 18u))) + 1013904223u) * 1664525u;
    uint _378 = _376 ^ (_376 >> 11u);
    uint _381 = _378 ^ ((_378 << 7u) & 2636928640u);
    uint _384 = _381 ^ ((_381 << 15u) & 4022730752u);
    uint _386 = _384 ^ (_384 >> 18u);
    uint _388 = _386 + (_386 << 10u);
    uint _390 = _388 ^ (_388 >> 6u);
    uint _392 = _390 + (_390 << 3u);
    uint _394 = _392 ^ (_392 >> 11u);
    uint3 _407 = uint3(_219 + int3(1, 1, 0));
    uint _410 = _407.z;
    uint _412 = _410 + (_410 << 10u);
    uint _414 = _412 ^ (_412 >> 6u);
    uint _416 = _414 + (_414 << 3u);
    uint _418 = _416 ^ (_416 >> 11u);
    uint _424 = (((_407.y * 1664525u) + (_418 + (_418 << 15u))) + 1013904223u) * 1664525u;
    uint _426 = _424 ^ (_424 >> 11u);
    uint _429 = _426 ^ ((_426 << 7u) & 2636928640u);
    uint _432 = _429 ^ ((_429 << 15u) & 4022730752u);
    uint _438 = (((_407.x * 1664525u) + (_432 ^ (_432 >> 18u))) + 1013904223u) * 1664525u;
    uint _440 = _438 ^ (_438 >> 11u);
    uint _443 = _440 ^ ((_440 << 7u) & 2636928640u);
    uint _446 = _443 ^ ((_443 << 15u) & 4022730752u);
    uint _448 = _446 ^ (_446 >> 18u);
    uint _450 = _448 + (_448 << 10u);
    uint _452 = _450 ^ (_450 >> 6u);
    uint _454 = _452 + (_452 << 3u);
    uint _456 = _454 ^ (_454 >> 11u);
    uint3 _469 = uint3(_219 + int3(0, 0, 1));
    uint _472 = _469.z;
    uint _474 = _472 + (_472 << 10u);
    uint _476 = _474 ^ (_474 >> 6u);
    uint _478 = _476 + (_476 << 3u);
    uint _480 = _478 ^ (_478 >> 11u);
    uint _486 = (((_469.y * 1664525u) + (_480 + (_480 << 15u))) + 1013904223u) * 1664525u;
    uint _488 = _486 ^ (_486 >> 11u);
    uint _491 = _488 ^ ((_488 << 7u) & 2636928640u);
    uint _494 = _491 ^ ((_491 << 15u) & 4022730752u);
    uint _500 = (((_469.x * 1664525u) + (_494 ^ (_494 >> 18u))) + 1013904223u) * 1664525u;
    uint _502 = _500 ^ (_500 >> 11u);
    uint _505 = _502 ^ ((_502 << 7u) & 2636928640u);
    uint _508 = _505 ^ ((_505 << 15u) & 4022730752u);
    uint _510 = _508 ^ (_508 >> 18u);
    uint _512 = _510 + (_510 << 10u);
    uint _514 = _512 ^ (_512 >> 6u);
    uint _516 = _514 + (_514 << 3u);
    uint _518 = _516 ^ (_516 >> 11u);
    uint3 _531 = uint3(_219 + int3(1, 0, 1));
    uint _534 = _531.z;
    uint _536 = _534 + (_534 << 10u);
    uint _538 = _536 ^ (_536 >> 6u);
    uint _540 = _538 + (_538 << 3u);
    uint _542 = _540 ^ (_540 >> 11u);
    uint _548 = (((_531.y * 1664525u) + (_542 + (_542 << 15u))) + 1013904223u) * 1664525u;
    uint _550 = _548 ^ (_548 >> 11u);
    uint _553 = _550 ^ ((_550 << 7u) & 2636928640u);
    uint _556 = _553 ^ ((_553 << 15u) & 4022730752u);
    uint _562 = (((_531.x * 1664525u) + (_556 ^ (_556 >> 18u))) + 1013904223u) * 1664525u;
    uint _564 = _562 ^ (_562 >> 11u);
    uint _567 = _564 ^ ((_564 << 7u) & 2636928640u);
    uint _570 = _567 ^ ((_567 << 15u) & 4022730752u);
    uint _572 = _570 ^ (_570 >> 18u);
    uint _574 = _572 + (_572 << 10u);
    uint _576 = _574 ^ (_574 >> 6u);
    uint _578 = _576 + (_576 << 3u);
    uint _580 = _578 ^ (_578 >> 11u);
    uint3 _593 = uint3(_219 + int3(0, 1, 1));
    uint _596 = _593.z;
    uint _598 = _596 + (_596 << 10u);
    uint _600 = _598 ^ (_598 >> 6u);
    uint _602 = _600 + (_600 << 3u);
    uint _604 = _602 ^ (_602 >> 11u);
    uint _610 = (((_593.y * 1664525u) + (_604 + (_604 << 15u))) + 1013904223u) * 1664525u;
    uint _612 = _610 ^ (_610 >> 11u);
    uint _615 = _612 ^ ((_612 << 7u) & 2636928640u);
    uint _618 = _615 ^ ((_615 << 15u) & 4022730752u);
    uint _624 = (((_593.x * 1664525u) + (_618 ^ (_618 >> 18u))) + 1013904223u) * 1664525u;
    uint _626 = _624 ^ (_624 >> 11u);
    uint _629 = _626 ^ ((_626 << 7u) & 2636928640u);
    uint _632 = _629 ^ ((_629 << 15u) & 4022730752u);
    uint _634 = _632 ^ (_632 >> 18u);
    uint _636 = _634 + (_634 << 10u);
    uint _638 = _636 ^ (_636 >> 6u);
    uint _640 = _638 + (_638 << 3u);
    uint _642 = _640 ^ (_640 >> 11u);
    uint3 _655 = uint3(_219 + int3(1, 1, 1));
    uint _658 = _655.z;
    uint _660 = _658 + (_658 << 10u);
    uint _662 = _660 ^ (_660 >> 6u);
    uint _664 = _662 + (_662 << 3u);
    uint _666 = _664 ^ (_664 >> 11u);
    uint _672 = (((_655.y * 1664525u) + (_666 + (_666 << 15u))) + 1013904223u) * 1664525u;
    uint _674 = _672 ^ (_672 >> 11u);
    uint _677 = _674 ^ ((_674 << 7u) & 2636928640u);
    uint _680 = _677 ^ ((_677 << 15u) & 4022730752u);
    uint _686 = (((_655.x * 1664525u) + (_680 ^ (_680 >> 18u))) + 1013904223u) * 1664525u;
    uint _688 = _686 ^ (_686 >> 11u);
    uint _691 = _688 ^ ((_688 << 7u) & 2636928640u);
    uint _694 = _691 ^ ((_691 << 15u) & 4022730752u);
    uint _696 = _694 ^ (_694 >> 18u);
    uint _698 = _696 + (_696 << 10u);
    uint _700 = _698 ^ (_698 >> 6u);
    uint _702 = _700 + (_700 << 3u);
    uint _704 = _702 ^ (_702 >> 11u);
    float3 _719 = (_220 * _220) * (3.0f.xxx - (_220 * 2.0f));
    float _720 = _719.x;
    float _721 = 1.0f - _720;
    float _723 = _719.y;
    float _724 = 1.0f - _723;
    float _726 = _719.z;
    float _727 = 1.0f - _726;
    float2 _744 = ((((((float2(asfloat((_262 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_270 + (_270 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _721) * _724) * _727) + (((float2(asfloat((_324 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_332 + (_332 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _720) * _724) * _727)) + (((float2(asfloat((_386 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_394 + (_394 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _721) * _723) * _727)) + (((float2(asfloat((_448 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_456 + (_456 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _720) * _723) * _727)) + (((float2(asfloat((_510 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_518 + (_518 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _721) * _724) * _726);
    float2 _759 = clamp(((((_744 + (((float2(asfloat((_572 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_580 + (_580 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _720) * _724) * _726)) + (((float2(asfloat((_634 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_642 + (_642 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _721) * _723) * _726)) + (((float2(asfloat((_696 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_704 + (_704 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _720) * _723) * _726)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _766 = (float3(_759.x, 0.0f, _759.y) * 0.0599999986588954925537109375f) + float3(0.0f, dot(instanced_wall_data.Load<float4>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 48), 0.25f.xxxx), 0.0f);
    float3 _767 = _216 + _766;
    float3x4 _769 = _212;
    _769[0].w = _767.x;
    float3x4 _771 = _769;
    _771[1].w = _767.y;
    float3x4 _773 = _771;
    _773[2].w = _767.z;
    uint3 _779 = uint3((uint(gl_VertexIndex) & 4u) >> 2u, (uint(gl_VertexIndex) & 2u) >> 1u, uint(gl_VertexIndex) & 1u);
    float3 _780 = mul(_773, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4 _783 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _796 = mul((_783.xyz / _783.w.xxx) - _780, float3x3(_769[0].xyz, _771[1].xyz, _773[2].xyz));
    bool3 _797 = bool3(_796.x > 0.0f.xxx.x, _796.y > 0.0f.xxx.y, _796.z > 0.0f.xxx.z);
    uint3 _798 = uint3(1u, 1u, 1u) - _779;
    float3 _801 = float3(uint3(_797.x ? _779.x : _798.x, _797.y ? _779.y : _798.y, _797.z ? _779.z : _798.z)) - 0.5f.xxx;
    float4 _806 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(_801, 1.0f));
    float3 _807 = _806.xyz;
    int _810 = int(instanced_wall_data.Load<uint>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 92));
    float3 _827;
    if ((_810 & 32) != 0)
    {
        float3 _826 = _807;
        _826.z = _806.z * lerp(instanced_wall_data.Load<float>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 64), instanced_wall_data.Load<float>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 68), 0.5f - _806.x);
        _827 = _826;
    }
    else
    {
        _827 = _807;
    }
    float3 _853;
    if ((_810 & 4) != 0)
    {
        uint _835 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 76) + (instanced_wall_data.Load<uint>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 76) << 10u);
        uint _837 = _835 ^ (_835 >> 6u);
        uint _839 = _837 + (_837 << 3u);
        uint _841 = _839 ^ (_839 >> 11u);
        float _849 = ((asfloat(((_841 + (_841 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.89999997615814208984375f) ? 1.0f : 1.099999904632568359375f;
        float3 _850 = float3(_849, _849, 1.0f);
        bool3 _851 = ((_810 & 1) != 0).xxx;
        _853 = float3(_851.x ? 1.0499999523162841796875f.xxx.x : _850.x, _851.y ? 1.0499999523162841796875f.xxx.y : _850.y, _851.z ? 1.0499999523162841796875f.xxx.z : _850.z);
    }
    else
    {
        _853 = 1.0f.xxx;
    }
    float4 _864 = float4(mul(_773, float4(_827 * (_853 * 0.949999988079071044921875f), 1.0f)), 1.0f);
    float _870 = mul(float3x3(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 1.0f, 0.0f), float3(0.0f, 0.0f, 1.0f)), _801).x - (-0.5f);
    int _880;
    if (instanced_wall_data.Load<uint>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 76) < 10u)
    {
        _880 = _810 | 512;
    }
    else
    {
        _880 = _810;
    }
    float _900;
    if (_870 < 0.5f)
    {
        _900 = lerp(instanced_wall_data.Load<float3>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 64).x, instanced_wall_data.Load<float3>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 64).y, _870 * 2.0f);
    }
    else
    {
        _900 = lerp(instanced_wall_data.Load<float3>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 64).y, instanced_wall_data.Load<float3>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 64).z, (_870 * 2.0f) - 1.0f);
    }
    bool _902 = (_880 & 32) != 0;
    float2 _925;
    if (_902)
    {
        uint _910 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 76) + (instanced_wall_data.Load<uint>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 76) << 10u);
        uint _912 = _910 ^ (_910 >> 6u);
        uint _914 = _912 + (_912 << 3u);
        uint _916 = _914 ^ (_914 >> 11u);
        _925 = ((_864.xz * 0.699999988079071044921875f) * 0.300000011920928955078125f) + (asfloat(((_916 + (_916 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx;
    }
    else
    {
        _925 = 0.0f.xx;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _864);
    out_var_C0 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_188 * 4 + 0) * 96 + 76) & 65535u;
    out_var_C1 = 0.0f.xxx;
    out_var_C2 = _864.xyz;
    out_var_C5 = _766;
    out_var_C7 = _902 ? (-100.0f) : _900;
    out_var_C10 = _925;
    out_var_C11 = 0;
    out_var_C12 = float4(_780.x, _179, _780.z, 1.0f).xz;
    out_var_C13 = _880;
    out_var_C14 = instance_list.Load<uint>(_188 * 4 + 0);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C10 = out_var_C10;
    stage_output.out_var_C11 = out_var_C11;
    stage_output.out_var_C12 = out_var_C12;
    stage_output.out_var_C13 = out_var_C13;
    stage_output.out_var_C14 = out_var_C14;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float3 in_var_C2;
static float in_var_C7;
static int in_var_C13;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(7)]] float in_var_C7 : TEXCOORD7;
    [[vk::location(13)]] nointerpolation int in_var_C13 : TEXCOORD13;
};

void frag_main()
{
    bool _67;
    if (in_var_C7 > 0.0f)
    {
        _67 = in_var_C2.y < in_var_C7;
    }
    else
    {
        _67 = false;
    }
    if (_67)
    {
        discard;
    }
    bool _76;
    if ((in_var_C13 & 8) != 0)
    {
        _76 = in_var_C2.y > in_var_C7;
    }
    else
    {
        _76 = false;
    }
    if (_76)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C2 = stage_input.in_var_C2;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C13 = stage_input.in_var_C13;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
