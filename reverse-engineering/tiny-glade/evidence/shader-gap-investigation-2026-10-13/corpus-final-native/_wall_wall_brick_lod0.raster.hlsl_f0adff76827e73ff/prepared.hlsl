#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#define TINYGLADE_RASTER_NO_COLOR 1
#line 1 "shaders/wall/wall_brick_lod0.raster.hlsl"




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

static uint _176;
static float _188;

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
    uint current_view_index : packoffset(c0);
    uint highlight_mode : packoffset(c0.y);
    int highlight_bricks_to_delete : packoffset(c0.z);
    float3 brush_position : packoffset(c1);
    uint bool_render_shadows : packoffset(c1.w);
    float eraser_mult : packoffset(c2);
};

ByteAddressBuffer wall_color_ids : register(t14, space0);
ByteAddressBuffer roof_color_ids : register(t18, space0);
ByteAddressBuffer instanced_wall_data : register(t19, space0);
ByteAddressBuffer instance_list : register(t20, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static int in_var_is_bevel;
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
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_Vertex_Normal : TEXCOORD1;
    [[vk::location(2)]] int in_var_is_bevel : TEXCOORD2;
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
    uint _201 = (uint(highlight_mode != 0u) * 4194304u) + uint(gl_InstanceIndex);
    float _211 = 1.57000005245208740234375f * float(int(instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 76) & 3u));
    float3 _212 = normalize(float3(0.0f, 1.0f, 0.0f));
    float _213 = sin(_211);
    float _214 = cos(_211);
    float _215 = 1.0f - _214;
    float _216 = _212.x;
    float _217 = _215 * _216;
    float _220 = _212.y;
    float _221 = _217 * _220;
    float _222 = _212.z;
    float _223 = _222 * _213;
    float _225 = _215 * _222;
    float _226 = _225 * _216;
    float _227 = _220 * _213;
    float _230 = _215 * _220;
    float _233 = _230 * _222;
    float _234 = _216 * _213;
    float4x4 _244 = transpose(float4x4(float4((_217 * _216) + _214, _221 - _223, _226 + _227, 0.0f), float4(_221 + _223, (_230 * _220) + _214, _233 - _234, 0.0f), float4(_226 - _227, _233 + _234, (_225 * _222) + _214, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float _246[12];
    [unroll]
    for (int _5ident = 0; _5ident < 12; _5ident++)
    {
        _246[_5ident] = instanced_wall_data.Load<float>(_5ident * 4 + instance_list.Load<uint>(_201 * 4 + 0) * 96 + 0);
    }
    float3x4 _264 = transpose(float4x3(float3(_246[0], _246[1], _246[2]), float3(_246[3], _246[4], _246[5]), float3(_246[6], _246[7], _246[8]), float3(_246[9], _246[10], _246[11])));
    float3x4 _268 = _264;
    _268[0].w = _264[0].w;
    float3x4 _269 = _268;
    _269[1].w = _264[1].w;
    float3x4 _270 = _269;
    _270[2].w = _264[2].w;
    float4 _275 = mul(_244, float4(in_var_Vertex_Position, 1.0f));
    float3 _276 = _275.xyz;
    float4 _281 = mul(_244, float4(in_var_Vertex_Normal, 0.0f));
    int _285 = int(instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 92));
    float3 _302;
    if ((_285 & 32) != 0)
    {
        float3 _301 = _276;
        _301.z = _275.z * lerp(instanced_wall_data.Load<float>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 64), instanced_wall_data.Load<float>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 68), 0.5f - _275.x);
        _302 = _301;
    }
    else
    {
        _302 = _276;
    }
    bool _304 = (_285 & 4) != 0;
    float3 _327;
    if (_304)
    {
        uint _309 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 76) + (instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 76) << 10u);
        uint _311 = _309 ^ (_309 >> 6u);
        uint _313 = _311 + (_311 << 3u);
        uint _315 = _313 ^ (_313 >> 11u);
        float _323 = ((asfloat(((_315 + (_315 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.89999997615814208984375f) ? 1.0f : 1.099999904632568359375f;
        float3 _324 = float3(_323, _323, 1.0f);
        bool3 _325 = ((_285 & 1) != 0).xxx;
        _327 = float3(_325.x ? 1.0499999523162841796875f.xxx.x : _324.x, _325.y ? 1.0499999523162841796875f.xxx.y : _324.y, _325.z ? 1.0499999523162841796875f.xxx.z : _324.z);
    }
    else
    {
        _327 = 1.0f.xxx;
    }
    bool _330 = _304 ? true : false;
    float4 _340 = float4(mul(_270, float4(_302 * _327, 1.0f)), 1.0f);
    float3 _341 = _340.xyz;
    float3 _342 = _341 * 0.4000000059604644775390625f;
    int3 _344 = int3(floor(_342));
    float3 _345 = frac(_342);
    uint3 _346 = uint3(_344);
    uint _349 = _346.z;
    uint _351 = _349 + (_349 << 10u);
    uint _353 = _351 ^ (_351 >> 6u);
    uint _355 = _353 + (_353 << 3u);
    uint _357 = _355 ^ (_355 >> 11u);
    uint _363 = (((_346.y * 1664525u) + (_357 + (_357 << 15u))) + 1013904223u) * 1664525u;
    uint _365 = _363 ^ (_363 >> 11u);
    uint _368 = _365 ^ ((_365 << 7u) & 2636928640u);
    uint _371 = _368 ^ ((_368 << 15u) & 4022730752u);
    uint _377 = (((_346.x * 1664525u) + (_371 ^ (_371 >> 18u))) + 1013904223u) * 1664525u;
    uint _379 = _377 ^ (_377 >> 11u);
    uint _382 = _379 ^ ((_379 << 7u) & 2636928640u);
    uint _385 = _382 ^ ((_382 << 15u) & 4022730752u);
    uint _387 = _385 ^ (_385 >> 18u);
    uint _389 = _387 + (_387 << 10u);
    uint _391 = _389 ^ (_389 >> 6u);
    uint _393 = _391 + (_391 << 3u);
    uint _395 = _393 ^ (_393 >> 11u);
    bool _398 = _330 ? _330 : true;
    uint _399 = _330 ? (_304 ? 8388607u : _176) : 8388607u;
    uint _400 = _330 ? (_304 ? 1065353216u : _176) : 1065353216u;
    bool _405 = _398 ? _398 : true;
    uint _406 = _398 ? _399 : 8388607u;
    uint _407 = _398 ? _400 : 1065353216u;
    uint3 _414 = uint3(_344 + int3(1, 0, 0));
    uint _417 = _414.z;
    uint _419 = _417 + (_417 << 10u);
    uint _421 = _419 ^ (_419 >> 6u);
    uint _423 = _421 + (_421 << 3u);
    uint _425 = _423 ^ (_423 >> 11u);
    uint _431 = (((_414.y * 1664525u) + (_425 + (_425 << 15u))) + 1013904223u) * 1664525u;
    uint _433 = _431 ^ (_431 >> 11u);
    uint _436 = _433 ^ ((_433 << 7u) & 2636928640u);
    uint _439 = _436 ^ ((_436 << 15u) & 4022730752u);
    uint _445 = (((_414.x * 1664525u) + (_439 ^ (_439 >> 18u))) + 1013904223u) * 1664525u;
    uint _447 = _445 ^ (_445 >> 11u);
    uint _450 = _447 ^ ((_447 << 7u) & 2636928640u);
    uint _453 = _450 ^ ((_450 << 15u) & 4022730752u);
    uint _455 = _453 ^ (_453 >> 18u);
    uint _457 = _455 + (_455 << 10u);
    uint _459 = _457 ^ (_457 >> 6u);
    uint _461 = _459 + (_459 << 3u);
    uint _463 = _461 ^ (_461 >> 11u);
    bool _466 = _405 ? _405 : true;
    uint _467 = _405 ? _406 : 8388607u;
    uint _468 = _405 ? _407 : 1065353216u;
    bool _473 = _466 ? _466 : true;
    uint _474 = _466 ? _467 : 8388607u;
    uint _475 = _466 ? _468 : 1065353216u;
    uint3 _482 = uint3(_344 + int3(0, 1, 0));
    uint _485 = _482.z;
    uint _487 = _485 + (_485 << 10u);
    uint _489 = _487 ^ (_487 >> 6u);
    uint _491 = _489 + (_489 << 3u);
    uint _493 = _491 ^ (_491 >> 11u);
    uint _499 = (((_482.y * 1664525u) + (_493 + (_493 << 15u))) + 1013904223u) * 1664525u;
    uint _501 = _499 ^ (_499 >> 11u);
    uint _504 = _501 ^ ((_501 << 7u) & 2636928640u);
    uint _507 = _504 ^ ((_504 << 15u) & 4022730752u);
    uint _513 = (((_482.x * 1664525u) + (_507 ^ (_507 >> 18u))) + 1013904223u) * 1664525u;
    uint _515 = _513 ^ (_513 >> 11u);
    uint _518 = _515 ^ ((_515 << 7u) & 2636928640u);
    uint _521 = _518 ^ ((_518 << 15u) & 4022730752u);
    uint _523 = _521 ^ (_521 >> 18u);
    uint _525 = _523 + (_523 << 10u);
    uint _527 = _525 ^ (_525 >> 6u);
    uint _529 = _527 + (_527 << 3u);
    uint _531 = _529 ^ (_529 >> 11u);
    bool _534 = _473 ? _473 : true;
    uint _535 = _473 ? _474 : 8388607u;
    uint _536 = _473 ? _475 : 1065353216u;
    bool _541 = _534 ? _534 : true;
    uint _542 = _534 ? _535 : 8388607u;
    uint _543 = _534 ? _536 : 1065353216u;
    uint3 _550 = uint3(_344 + int3(1, 1, 0));
    uint _553 = _550.z;
    uint _555 = _553 + (_553 << 10u);
    uint _557 = _555 ^ (_555 >> 6u);
    uint _559 = _557 + (_557 << 3u);
    uint _561 = _559 ^ (_559 >> 11u);
    uint _567 = (((_550.y * 1664525u) + (_561 + (_561 << 15u))) + 1013904223u) * 1664525u;
    uint _569 = _567 ^ (_567 >> 11u);
    uint _572 = _569 ^ ((_569 << 7u) & 2636928640u);
    uint _575 = _572 ^ ((_572 << 15u) & 4022730752u);
    uint _581 = (((_550.x * 1664525u) + (_575 ^ (_575 >> 18u))) + 1013904223u) * 1664525u;
    uint _583 = _581 ^ (_581 >> 11u);
    uint _586 = _583 ^ ((_583 << 7u) & 2636928640u);
    uint _589 = _586 ^ ((_586 << 15u) & 4022730752u);
    uint _591 = _589 ^ (_589 >> 18u);
    uint _593 = _591 + (_591 << 10u);
    uint _595 = _593 ^ (_593 >> 6u);
    uint _597 = _595 + (_595 << 3u);
    uint _599 = _597 ^ (_597 >> 11u);
    bool _602 = _541 ? _541 : true;
    uint _603 = _541 ? _542 : 8388607u;
    uint _604 = _541 ? _543 : 1065353216u;
    bool _609 = _602 ? _602 : true;
    uint _610 = _602 ? _603 : 8388607u;
    uint _611 = _602 ? _604 : 1065353216u;
    uint3 _618 = uint3(_344 + int3(0, 0, 1));
    uint _621 = _618.z;
    uint _623 = _621 + (_621 << 10u);
    uint _625 = _623 ^ (_623 >> 6u);
    uint _627 = _625 + (_625 << 3u);
    uint _629 = _627 ^ (_627 >> 11u);
    uint _635 = (((_618.y * 1664525u) + (_629 + (_629 << 15u))) + 1013904223u) * 1664525u;
    uint _637 = _635 ^ (_635 >> 11u);
    uint _640 = _637 ^ ((_637 << 7u) & 2636928640u);
    uint _643 = _640 ^ ((_640 << 15u) & 4022730752u);
    uint _649 = (((_618.x * 1664525u) + (_643 ^ (_643 >> 18u))) + 1013904223u) * 1664525u;
    uint _651 = _649 ^ (_649 >> 11u);
    uint _654 = _651 ^ ((_651 << 7u) & 2636928640u);
    uint _657 = _654 ^ ((_654 << 15u) & 4022730752u);
    uint _659 = _657 ^ (_657 >> 18u);
    uint _661 = _659 + (_659 << 10u);
    uint _663 = _661 ^ (_661 >> 6u);
    uint _665 = _663 + (_663 << 3u);
    uint _667 = _665 ^ (_665 >> 11u);
    bool _670 = _609 ? _609 : true;
    uint _671 = _609 ? _610 : 8388607u;
    uint _672 = _609 ? _611 : 1065353216u;
    bool _677 = _670 ? _670 : true;
    uint _678 = _670 ? _671 : 8388607u;
    uint _679 = _670 ? _672 : 1065353216u;
    uint3 _686 = uint3(_344 + int3(1, 0, 1));
    uint _689 = _686.z;
    uint _691 = _689 + (_689 << 10u);
    uint _693 = _691 ^ (_691 >> 6u);
    uint _695 = _693 + (_693 << 3u);
    uint _697 = _695 ^ (_695 >> 11u);
    uint _703 = (((_686.y * 1664525u) + (_697 + (_697 << 15u))) + 1013904223u) * 1664525u;
    uint _705 = _703 ^ (_703 >> 11u);
    uint _708 = _705 ^ ((_705 << 7u) & 2636928640u);
    uint _711 = _708 ^ ((_708 << 15u) & 4022730752u);
    uint _717 = (((_686.x * 1664525u) + (_711 ^ (_711 >> 18u))) + 1013904223u) * 1664525u;
    uint _719 = _717 ^ (_717 >> 11u);
    uint _722 = _719 ^ ((_719 << 7u) & 2636928640u);
    uint _725 = _722 ^ ((_722 << 15u) & 4022730752u);
    uint _727 = _725 ^ (_725 >> 18u);
    uint _729 = _727 + (_727 << 10u);
    uint _731 = _729 ^ (_729 >> 6u);
    uint _733 = _731 + (_731 << 3u);
    uint _735 = _733 ^ (_733 >> 11u);
    bool _738 = _677 ? _677 : true;
    uint _739 = _677 ? _678 : 8388607u;
    uint _740 = _677 ? _679 : 1065353216u;
    bool _745 = _738 ? _738 : true;
    uint _746 = _738 ? _739 : 8388607u;
    uint _747 = _738 ? _740 : 1065353216u;
    uint3 _754 = uint3(_344 + int3(0, 1, 1));
    uint _757 = _754.z;
    uint _759 = _757 + (_757 << 10u);
    uint _761 = _759 ^ (_759 >> 6u);
    uint _763 = _761 + (_761 << 3u);
    uint _765 = _763 ^ (_763 >> 11u);
    uint _771 = (((_754.y * 1664525u) + (_765 + (_765 << 15u))) + 1013904223u) * 1664525u;
    uint _773 = _771 ^ (_771 >> 11u);
    uint _776 = _773 ^ ((_773 << 7u) & 2636928640u);
    uint _779 = _776 ^ ((_776 << 15u) & 4022730752u);
    uint _785 = (((_754.x * 1664525u) + (_779 ^ (_779 >> 18u))) + 1013904223u) * 1664525u;
    uint _787 = _785 ^ (_785 >> 11u);
    uint _790 = _787 ^ ((_787 << 7u) & 2636928640u);
    uint _793 = _790 ^ ((_790 << 15u) & 4022730752u);
    uint _795 = _793 ^ (_793 >> 18u);
    uint _797 = _795 + (_795 << 10u);
    uint _799 = _797 ^ (_797 >> 6u);
    uint _801 = _799 + (_799 << 3u);
    uint _803 = _801 ^ (_801 >> 11u);
    bool _806 = _745 ? _745 : true;
    uint _807 = _745 ? _746 : 8388607u;
    uint _808 = _745 ? _747 : 1065353216u;
    bool _813 = _806 ? _806 : true;
    uint _814 = _806 ? _807 : 8388607u;
    uint _815 = _806 ? _808 : 1065353216u;
    uint3 _822 = uint3(_344 + int3(1, 1, 1));
    uint _825 = _822.z;
    uint _827 = _825 + (_825 << 10u);
    uint _829 = _827 ^ (_827 >> 6u);
    uint _831 = _829 + (_829 << 3u);
    uint _833 = _831 ^ (_831 >> 11u);
    uint _839 = (((_822.y * 1664525u) + (_833 + (_833 << 15u))) + 1013904223u) * 1664525u;
    uint _841 = _839 ^ (_839 >> 11u);
    uint _844 = _841 ^ ((_841 << 7u) & 2636928640u);
    uint _847 = _844 ^ ((_844 << 15u) & 4022730752u);
    uint _853 = (((_822.x * 1664525u) + (_847 ^ (_847 >> 18u))) + 1013904223u) * 1664525u;
    uint _855 = _853 ^ (_853 >> 11u);
    uint _858 = _855 ^ ((_855 << 7u) & 2636928640u);
    uint _861 = _858 ^ ((_858 << 15u) & 4022730752u);
    uint _863 = _861 ^ (_861 >> 18u);
    uint _865 = _863 + (_863 << 10u);
    uint _867 = _865 ^ (_865 >> 6u);
    uint _869 = _867 + (_867 << 3u);
    uint _871 = _869 ^ (_869 >> 11u);
    bool _874 = _813 ? _813 : true;
    uint _875 = _813 ? _814 : 8388607u;
    uint _876 = _813 ? _815 : 1065353216u;
    bool _881 = _874 ? _874 : true;
    uint _882 = _874 ? _875 : 8388607u;
    uint _883 = _874 ? _876 : 1065353216u;
    float3 _892 = (_345 * _345) * (3.0f.xxx - (_345 * 2.0f));
    float _893 = _892.x;
    float _894 = 1.0f - _893;
    float _896 = _892.y;
    float _897 = 1.0f - _896;
    float _899 = _892.z;
    float _900 = 1.0f - _899;
    float2 _913 = (((((float2(asfloat((_387 & _399) | _400) - 1.0f, asfloat(((_395 + (_395 << 15u)) & _406) | _407) - 1.0f) * _894) * _897) * _900) + (((float2(asfloat((_455 & _467) | _468) - 1.0f, asfloat(((_463 + (_463 << 15u)) & _474) | _475) - 1.0f) * _893) * _897) * _900)) + (((float2(asfloat((_523 & _535) | _536) - 1.0f, asfloat(((_531 + (_531 << 15u)) & _542) | _543) - 1.0f) * _894) * _896) * _900)) + (((float2(asfloat((_591 & _603) | _604) - 1.0f, asfloat(((_599 + (_599 << 15u)) & _610) | _611) - 1.0f) * _893) * _896) * _900);
    float2 _929 = (((_913 + (((float2(asfloat((_659 & _671) | _672) - 1.0f, asfloat(((_667 + (_667 << 15u)) & _678) | _679) - 1.0f) * _894) * _897) * _899)) + (((float2(asfloat((_727 & _739) | _740) - 1.0f, asfloat(((_735 + (_735 << 15u)) & _746) | _747) - 1.0f) * _893) * _897) * _899)) + (((float2(asfloat((_795 & _807) | _808) - 1.0f, asfloat(((_803 + (_803 << 15u)) & _814) | _815) - 1.0f) * _894) * _896) * _899)) + (((float2(asfloat((_863 & _875) | _876) - 1.0f, asfloat(((_871 + (_871 << 15u)) & _882) | _883) - 1.0f) * _893) * _896) * _899);
    float2 _932 = clamp((_929 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _937 = _341 + (float3(_932.x, 0.0f, _932.y) * 0.0599999986588954925537109375f);
    float4 _938 = float4(_937.x, _937.y, _937.z, _340.w);
    float3 _939 = mul(_270, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _950 = mul(float3x3(_244[0].xyz, _244[1].xyz, _244[2].xyz), in_var_Vertex_Position);
    float _952 = _950.x - (-0.5f);
    float _963 = _937.y + lerp(lerp(instanced_wall_data.Load<float4>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 48).x, instanced_wall_data.Load<float4>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 48).y, _952), lerp(instanced_wall_data.Load<float4>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 48).z, instanced_wall_data.Load<float4>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 48).w, _952), _950.y - (-0.5f));
    _938.y = _963;
    float3x3 _973 = float3x3(_268[0].xyz, _269[1].xyz, _270[2].xyz);
    float3 _974 = mul(_973, float3(1.0f, 0.0f, 0.0f));
    float3 _975 = mul(_973, float3(0.0f, 1.0f, 0.0f));
    float3 _976 = mul(_973, float3(0.0f, 0.0f, 1.0f));
    float3 _979 = cross(_974, _975);
    float3 _982 = (1.0f / dot(_976, _979)).xxx;
    float2 _991 = float2(lerp(instanced_wall_data.Load<float2>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 84).x, instanced_wall_data.Load<float2>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 84).y, _952), _963);
    uint _993 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 76) & 65535u;
    bool _996 = in_var_is_bevel != 0;
    int _1000;
    if (_996)
    {
        _1000 = _285 | 256;
    }
    else
    {
        _1000 = _285;
    }
    int _1006;
    if (instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 76) < 10u)
    {
        _1006 = _1000 | 512;
    }
    else
    {
        _1006 = _1000;
    }
    float _1030;
    if (_952 < 0.5f)
    {
        _1030 = lerp(instanced_wall_data.Load<float3>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 64).x, instanced_wall_data.Load<float3>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 64).y, _952 * 2.0f);
    }
    else
    {
        _1030 = lerp(instanced_wall_data.Load<float3>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 64).y, instanced_wall_data.Load<float3>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 64).z, (_952 * 2.0f) - 1.0f);
    }
    int _1063;
    int _1064;
    if ((instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 80) & 2147483648u) != 0u)
    {
        bool _1058;
        if ((instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 80) & 1073741824u) != 0u)
        {
            _1058 = ((instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 80) >> 4u) & 65535u) == frame_constants_highlighted_transient_stair_edge_id;
        }
        else
        {
            _1058 = false;
        }
        int _1062;
        if (_1058)
        {
            _1062 = _1006 | 2048;
        }
        else
        {
            _1062 = _1006;
        }
        _1063 = int(instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 80) & 15u);
        _1064 = _1062;
    }
    else
    {
        uint _1045;
        if ((_1006 & 2) != 0)
        {
            _1045 = roof_color_ids.Load<uint>(instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 80) * 4 + 0);
        }
        else
        {
            _1045 = wall_color_ids.Load<uint>(instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 80) * 4 + 0);
        }
        _1063 = int(_1045);
        _1064 = _1006;
    }
    uint _1065 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 80) & 65535u;
    uint _1068 = _1065 ^ 7993234u;
    uint _1070 = _1068 + (_1068 << 10u);
    uint _1072 = _1070 ^ (_1070 >> 6u);
    uint _1074 = _1072 + (_1072 << 3u);
    uint _1076 = _1074 ^ (_1074 >> 11u);
    bool _1079 = _881 ? _881 : true;
    uint _1080 = _881 ? _882 : 8388607u;
    uint _1081 = _881 ? _883 : 1065353216u;
    uint _1086 = _1065 ^ 3450234u;
    uint _1088 = _1086 + (_1086 << 10u);
    uint _1090 = _1088 ^ (_1088 >> 6u);
    uint _1092 = _1090 + (_1090 << 3u);
    uint _1094 = _1092 ^ (_1092 >> 11u);
    bool _1097 = _1079 ? _1079 : true;
    uint _1098 = _1079 ? _1080 : 8388607u;
    uint _1099 = _1079 ? _1081 : 1065353216u;
    float2 _1111 = (((_991 * 0.100000001490116119384765625f) * 0.699999988079071044921875f) + (float(_1065) * 0.20000000298023223876953125f).xx) + float2(asfloat(((_1076 + (_1076 << 15u)) & _1080) | _1081) - 1.0f, asfloat(((_1094 + (_1094 << 15u)) & _1098) | _1099) - 1.0f);
    uint _1112 = _993 ^ 292478890u;
    uint _1114 = _1112 + (_1112 << 10u);
    uint _1116 = _1114 ^ (_1114 >> 6u);
    uint _1118 = _1116 + (_1116 << 3u);
    uint _1120 = _1118 ^ (_1118 >> 11u);
    bool _1123 = _1097 ? _1097 : true;
    uint _1124 = _1097 ? _1098 : 8388607u;
    uint _1125 = _1097 ? _1099 : 1065353216u;
    float _1129 = asfloat(((_1120 + (_1120 << 15u)) & _1124) | _1125) - 1.0f;
    float2 _1138;
    if (_1129 > 0.89999997615814208984375f)
    {
        _1138 = _1111 * 3.0f;
    }
    else
    {
        _1138 = _1111 + (0.61000001430511474609375f * _1129).xx;
    }
    float2 _1144;
    if (int((_1064 & 512) != 0) == 1)
    {
        _1144 = _1138 * 2.2000000476837158203125f;
    }
    else
    {
        _1144 = _1138;
    }
    bool _1146 = (_1006 & 32) != 0;
    float2 _1190;
    if (_1146)
    {
        float2 _1165;
        if (_281.y > 0.89999997615814208984375f)
        {
            _1165 = _938.xz * 0.699999988079071044921875f;
        }
        else
        {
            float2 _1162;
            if (abs(_281.x) > 0.89999997615814208984375f)
            {
                _1162 = _938.zy * 0.699999988079071044921875f;
            }
            else
            {
                _1162 = _991 * 0.699999988079071044921875f;
            }
            _1165 = _1162;
        }
        float2 _1169;
        if (_996)
        {
            _1169 = instanced_wall_data.Load<float2>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 84).x.xx;
        }
        else
        {
            _1169 = _1165;
        }
        uint _1173 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 76) + (instanced_wall_data.Load<uint>(instance_list.Load<uint>(_201 * 4 + 0) * 96 + 76) << 10u);
        uint _1175 = _1173 ^ (_1173 >> 6u);
        uint _1177 = _1175 + (_1175 << 3u);
        uint _1179 = _1177 ^ (_1177 >> 11u);
        _1190 = (_1169 * 0.300000011920928955078125f) + (asfloat(((_1179 + (_1179 << 15u)) & (_1123 ? _1124 : 8388607u)) | (_1123 ? _1125 : 1065353216u)) - 1.0f).xx;
    }
    else
    {
        _1190 = _1144;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, _938);
    out_var_C0 = _993;
    out_var_C1 = mul(transpose(float3x3(cross(_975, _976) * _982, cross(_976, _974) * _982, _979 * _982)), _281.xyz);
    out_var_C2 = _938.xyz;
    out_var_C5 = 0.0f.xxx;
    out_var_C7 = _1146 ? (-100.0f) : _1030;
    out_var_C10 = _1190;
    out_var_C11 = _1063;
    out_var_C12 = float4(_939.x, _188, _939.z, 1.0f).xz;
    out_var_C13 = _1064;
    out_var_C14 = instance_list.Load<uint>(_201 * 4 + 0);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_is_bevel = stage_input.in_var_is_bevel;
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

static uint _183;
static float3 _195;

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
    uint current_view_index : packoffset(c0);
    uint highlight_mode : packoffset(c0.y);
    int highlight_bricks_to_delete : packoffset(c0.z);
    float3 brush_position : packoffset(c1);
    uint bool_render_shadows : packoffset(c1.w);
    float eraser_mult : packoffset(c2);
};

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
static float3 in_var_C1;
static float3 in_var_C2;
static float in_var_C7;
static float2 in_var_C10;
static int in_var_C11;
static float2 in_var_C12;
static int in_var_C13;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] nointerpolation uint in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(7)]] float in_var_C7 : TEXCOORD7;
    [[vk::location(10)]] float2 in_var_C10 : TEXCOORD10;
    [[vk::location(11)]] nointerpolation int in_var_C11 : TEXCOORD11;
    [[vk::location(12)]] nointerpolation float2 in_var_C12 : TEXCOORD12;
    [[vk::location(13)]] nointerpolation int in_var_C13 : TEXCOORD13;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
    float gl_FragDepth : SV_DepthLessEqual;
};

void frag_main()
{
    bool _212 = (in_var_C13 & 512) != 0;
    bool _214 = (in_var_C13 & 8) != 0;
    bool _216 = (in_var_C13 & 16) != 0;
    bool _220 = (in_var_C13 & 2048) != 0;
    bool _222 = (in_var_C13 & 256) != 0;
    bool _227;
    if (!_214)
    {
        _227 = in_var_C7 > (-2.5f);
    }
    else
    {
        _227 = false;
    }
    bool _232;
    if (_227)
    {
        _232 = in_var_C2.y < in_var_C7;
    }
    else
    {
        _232 = false;
    }
    if (_232)
    {
        discard;
    }
    else
    {
        bool _240;
        if (_214)
        {
            _240 = in_var_C2.y > in_var_C7;
        }
        else
        {
            _240 = false;
        }
        if (_240)
        {
            discard;
        }
    }
    uint _246 = uint(in_var_C11);
    float3 _463;
    if ((in_var_C13 & 2) != 0)
    {
        _463 = roof_colors.Sample(sampler_llr, float3((float2(in_var_C0.xx) * 0.20000000298023223876953125f) + (in_var_C2.xz * 0.20000000298023223876953125f), float(_246))).xyz * 0.699999988079071044921875f;
    }
    else
    {
        float2 _258;
        if (_222)
        {
            _258 = in_var_C10 + 0.20999999344348907470703125f.xx;
        }
        else
        {
            _258 = in_var_C10;
        }
        bool _264;
        if (!(_246 == 1u))
        {
            _264 = _246 == 0u;
        }
        else
        {
            _264 = true;
        }
        float3 _271;
        if (_264)
        {
            _271 = pow(color_library_wall_bricks.xyz, 1.2999999523162841796875f.xxx);
        }
        else
        {
            _271 = pow(color_library_wall_moss.xyz, 1.0f.xxx) * 0.800000011920928955078125f;
        }
        int _272 = int(_246);
        bool _278;
        if (!(_272 == 0))
        {
            _278 = _272 == 4;
        }
        else
        {
            _278 = true;
        }
        bool _283;
        if (!_278)
        {
            _283 = _272 == 10;
        }
        else
        {
            _283 = true;
        }
        bool _288;
        if (!_283)
        {
            _288 = _272 == 12;
        }
        else
        {
            _288 = true;
        }
        float3 _379;
        if (_288)
        {
            _379 = brick_colors.Sample(sampler_llr, float3(_258 + 0.85000002384185791015625f.xx, 0.0f)).xyz;
        }
        else
        {
            bool _297;
            if (!(_272 == 1))
            {
                _297 = _272 == 5;
            }
            else
            {
                _297 = true;
            }
            float3 _369;
            if (_297)
            {
                _369 = pow(brick_colors.Sample(sampler_llr, float3(_258 + 0.449999988079071044921875f.xx, 1.0f)).xyz, 1.10000002384185791015625f.xxx);
            }
            else
            {
                bool _306;
                if (!(_272 == 2))
                {
                    _306 = _272 == 7;
                }
                else
                {
                    _306 = true;
                }
                float3 _358;
                if (_306)
                {
                    _358 = brick_colors.Sample(sampler_llr, float3(_258, 2.0f)).xyz;
                }
                else
                {
                    bool _315;
                    if (!(_272 == 3))
                    {
                        _315 = _272 == 6;
                    }
                    else
                    {
                        _315 = true;
                    }
                    bool _320;
                    if (!_315)
                    {
                        _320 = _272 == 8;
                    }
                    else
                    {
                        _320 = true;
                    }
                    bool _325;
                    if (!_320)
                    {
                        _325 = _272 == 9;
                    }
                    else
                    {
                        _325 = true;
                    }
                    float3 _349;
                    if (_325)
                    {
                        _349 = brick_colors.Sample(sampler_llr, float3(_258, 3.0f)).xyz;
                    }
                    else
                    {
                        float3 _340;
                        if (_272 == 11)
                        {
                            _340 = brick_colors.Sample(sampler_llr, float3(_258, 4.0f)).xyz;
                        }
                        else
                        {
                            _340 = 0.0f.xxx;
                        }
                        _349 = _340;
                    }
                    _358 = _349;
                }
                _369 = _358;
            }
            _379 = _369;
        }
        bool _381 = !(_246 == 7u);
        bool _385;
        if (_381)
        {
            _385 = _246 == 2u;
        }
        else
        {
            _385 = true;
        }
        bool _390;
        if (!_385)
        {
            _390 = _246 == 11u;
        }
        else
        {
            _390 = true;
        }
        bool3 _391 = _390.xxx;
        float3 _414;
        if (_212)
        {
            uint _395 = in_var_C0 ^ 8902390u;
            uint _397 = _395 + (_395 << 10u);
            uint _399 = _397 ^ (_397 >> 6u);
            uint _401 = _399 + (_399 << 3u);
            uint _403 = _401 ^ (_401 >> 11u);
            _414 = lerp(_379, float3(_391.x ? _379.x : _271.x, _391.y ? _379.y : _271.y, _391.z ? _379.z : _271.z), lerp(0.0f, 0.5f, smoothstep(0.20000000298023223876953125f, 0.800000011920928955078125f, asfloat(((_403 + (_403 << 15u)) & 8388607u) | 1065353216u) - 1.0f)).xxx);
        }
        else
        {
            _414 = _379;
        }
        bool _417 = _212 ? true : false;
        float3 _446;
        if (_222)
        {
            bool _423;
            if (_381)
            {
                _423 = _246 == 2u;
            }
            else
            {
                _423 = true;
            }
            uint _425 = in_var_C0 ^ 239085u;
            uint _427 = _425 + (_425 << 10u);
            uint _429 = _427 ^ (_427 >> 6u);
            uint _431 = _429 + (_429 << 3u);
            uint _433 = _431 ^ (_431 >> 11u);
            _446 = lerp(_414, color_library_wall_1.xyz, lerp(0.0f, _423 ? 0.20000000298023223876953125f : 0.4000000059604644775390625f, smoothstep(0.0f, 0.800000011920928955078125f, asfloat(((_433 + (_433 << 15u)) & (_417 ? (_212 ? 8388607u : _183) : 8388607u)) | (_417 ? (_212 ? 1065353216u : _183) : 1065353216u)) - 1.0f)).xxx);
        }
        else
        {
            _446 = _414;
        }
        _463 = _446;
    }
    float3 _464 = normalize(in_var_C1);
    bool _503;
    float3 _504;
    if (!((in_var_C13 & 64) != 0))
    {
        bool _481;
        float3 _482;
        if (highlight_mode == 1u)
        {
            _481 = true;
            _482 = lerp(_463, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
        }
        else
        {
            bool _474 = highlight_mode == 2u;
            float3 _478;
            if (_474)
            {
                _478 = lerp(_463, 0.5f.xxx, 0.100000001490116119384765625f.xxx);
            }
            else
            {
                _478 = _463;
            }
            _481 = _474 ? true : false;
            _482 = _478;
        }
        bool _501;
        float3 _502;
        if (highlight_bricks_to_delete == 1)
        {
            bool _495 = distance(brush_position.xz, in_var_C12) < (1.88900005817413330078125f * eraser_mult);
            float3 _499;
            if (_495)
            {
                _499 = lerp(_463, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.800000011920928955078125f.xxx);
            }
            else
            {
                _499 = _482;
            }
            _501 = _495 ? true : _481;
            _502 = _499;
        }
        else
        {
            _501 = _481;
            _502 = _482;
        }
        _503 = _501;
        _504 = _502;
    }
    else
    {
        _503 = false;
        _504 = _463;
    }
    float3 _508;
    if (_220)
    {
        _508 = lerp(_463, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
    }
    else
    {
        _508 = _504;
    }
    float3 _610;
    if (frame_constants_glade_theme_id == 4u)
    {
        float2 _520 = in_var_C2.xz * 0.4000000059604644775390625f;
        float2 _524 = floor(_520 + dot(_520, 0.3660254180431365966796875f.xx).xx);
        float2 _528 = (_520 - _524) + dot(_524, 0.211324870586395263671875f.xx).xx;
        float _529 = _528.x;
        float _530 = _528.y;
        bool2 _532 = (_529 > _530).xx;
        float2 _533 = float2(_532.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, _532.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
        float4 _535 = _528.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
        float2 _537 = _535.xy - _533;
        float4 _538 = float4(_537.x, _537.y, _535.z, _535.w);
        float2 _539 = fmod(_524, 289.0f.xx);
        float3 _544 = _539.y.xxx + float3(0.0f, _533.y, 1.0f);
        float3 _554 = (fmod(((_544 * 34.0f) + 1.0f.xxx) * _544, 289.0f.xxx) + _539.x.xxx) + float3(0.0f, _533.x, 1.0f);
        float2 _560 = _537.xy;
        float2 _562 = _535.zw;
        float3 _566 = max(0.5f.xxx - float3(dot(_528, _528), dot(_560, _560), dot(_562, _562)), 0.0f.xxx);
        float3 _567 = _566 * _566;
        float3 _571 = frac(fmod(((_554 * 34.0f) + 1.0f.xxx) * _554, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
        float3 _572 = _571 - 1.0f.xxx;
        float3 _574 = abs(_572) - 0.5f.xxx;
        float3 _577 = _572 - floor(_571 + (-0.5f).xxx);
        float3 _589;
        _589.x = (_577.x * _529) + (_574.x * _530);
        float2 _596 = (_577.yz * _538.xz) + (_574.yz * _538.yw);
        _610 = lerp(_508, 0.699999988079071044921875f.xxx, (smoothstep(0.0f, 1.0f, smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C2.y) * clamp(_464.y, 0.0f, 1.0f)) * clamp(clamp((130.0f * dot((_567 * _567) * (1.792842864990234375f.xxx - (((_577 * _577) + (_574 * _574)) * 0.8537347316741943359375f)), float3(_589.x, _596.x, _596.y))) + 0.75f, 0.0f, 1.0f), _216 ? 0.0f : 0.4000000059604644775390625f, _216 ? 0.100000001490116119384765625f : 1.0f)).xxx);
    }
    else
    {
        _610 = _508;
    }
    float3 _612 = (_464 * 0.5f) + 0.5f.xxx;
    float3 _634 = sqrt(clamp(_610, 0.0f.xxx, 1.0f.xxx));
    uint _653 = (uint((clamp(_634.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_634.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_634.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _654 = uint2(((uint((clamp(_612.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_612.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_612.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, _653);
    _654.y = _653 | (((((7u | (uint(_220 ? true : _503) << 3u)) | 0u) | 0u) | 0u) << 24u);
    out_var_SV_TARGET0 = _654;
    gl_FragDepth = gl_FragCoord.z;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C10 = stage_input.in_var_C10;
    in_var_C11 = stage_input.in_var_C11;
    in_var_C12 = stage_input.in_var_C12;
    in_var_C13 = stage_input.in_var_C13;
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

static uint _167;
static float _178;

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
    uint current_view_index : packoffset(c0);
    uint highlight_mode : packoffset(c0.y);
    int highlight_bricks_to_delete : packoffset(c0.z);
    float3 brush_position : packoffset(c1);
    uint bool_render_shadows : packoffset(c1.w);
    float eraser_mult : packoffset(c2);
};

ByteAddressBuffer instanced_wall_data : register(t19, space0);
ByteAddressBuffer instance_list : register(t20, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static int in_var_is_bevel;
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
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_Vertex_Normal : TEXCOORD1;
    [[vk::location(2)]] int in_var_is_bevel : TEXCOORD2;
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
    uint _190 = ((current_view_index + 2u) * 2097152u) + uint(gl_InstanceIndex);
    float _200 = 1.57000005245208740234375f * float(int(instanced_wall_data.Load<uint>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 76) & 3u));
    float3 _201 = normalize(float3(0.0f, 1.0f, 0.0f));
    float _202 = sin(_200);
    float _203 = cos(_200);
    float _204 = 1.0f - _203;
    float _205 = _201.x;
    float _206 = _204 * _205;
    float _209 = _201.y;
    float _210 = _206 * _209;
    float _211 = _201.z;
    float _212 = _211 * _202;
    float _214 = _204 * _211;
    float _215 = _214 * _205;
    float _216 = _209 * _202;
    float _219 = _204 * _209;
    float _222 = _219 * _211;
    float _223 = _205 * _202;
    float4x4 _233 = transpose(float4x4(float4((_206 * _205) + _203, _210 - _212, _215 + _216, 0.0f), float4(_210 + _212, (_219 * _209) + _203, _222 - _223, 0.0f), float4(_215 - _216, _222 + _223, (_214 * _211) + _203, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float _235[12];
    [unroll]
    for (int _5ident = 0; _5ident < 12; _5ident++)
    {
        _235[_5ident] = instanced_wall_data.Load<float>(_5ident * 4 + instance_list.Load<uint>(_190 * 4 + 0) * 96 + 0);
    }
    float3x4 _253 = transpose(float4x3(float3(_235[0], _235[1], _235[2]), float3(_235[3], _235[4], _235[5]), float3(_235[6], _235[7], _235[8]), float3(_235[9], _235[10], _235[11])));
    float3x4 _257 = _253;
    _257[0].w = _253[0].w;
    float3x4 _258 = _257;
    _258[1].w = _253[1].w;
    float3x4 _259 = _258;
    _259[2].w = _253[2].w;
    float4 _264 = mul(_233, float4(in_var_Vertex_Position, 1.0f));
    float3 _265 = _264.xyz;
    float4 _270 = mul(_233, float4(in_var_Vertex_Normal, 0.0f));
    int _274 = int(instanced_wall_data.Load<uint>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 92));
    float3 _291;
    if ((_274 & 32) != 0)
    {
        float3 _290 = _265;
        _290.z = _264.z * lerp(instanced_wall_data.Load<float>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 64), instanced_wall_data.Load<float>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 68), 0.5f - _264.x);
        _291 = _290;
    }
    else
    {
        _291 = _265;
    }
    bool _293 = (_274 & 4) != 0;
    float3 _316;
    if (_293)
    {
        uint _298 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 76) + (instanced_wall_data.Load<uint>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 76) << 10u);
        uint _300 = _298 ^ (_298 >> 6u);
        uint _302 = _300 + (_300 << 3u);
        uint _304 = _302 ^ (_302 >> 11u);
        float _312 = ((asfloat(((_304 + (_304 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.89999997615814208984375f) ? 1.0f : 1.099999904632568359375f;
        float3 _313 = float3(_312, _312, 1.0f);
        bool3 _314 = ((_274 & 1) != 0).xxx;
        _316 = float3(_314.x ? 1.0499999523162841796875f.xxx.x : _313.x, _314.y ? 1.0499999523162841796875f.xxx.y : _313.y, _314.z ? 1.0499999523162841796875f.xxx.z : _313.z);
    }
    else
    {
        _316 = 1.0f.xxx;
    }
    bool _319 = _293 ? true : false;
    float4 _330 = float4(mul(_259, float4(_291 * (_316 * 0.949999988079071044921875f), 1.0f)), 1.0f);
    float3 _331 = _330.xyz;
    float3 _332 = _331 * 0.4000000059604644775390625f;
    int3 _334 = int3(floor(_332));
    float3 _335 = frac(_332);
    uint3 _336 = uint3(_334);
    uint _339 = _336.z;
    uint _341 = _339 + (_339 << 10u);
    uint _343 = _341 ^ (_341 >> 6u);
    uint _345 = _343 + (_343 << 3u);
    uint _347 = _345 ^ (_345 >> 11u);
    uint _353 = (((_336.y * 1664525u) + (_347 + (_347 << 15u))) + 1013904223u) * 1664525u;
    uint _355 = _353 ^ (_353 >> 11u);
    uint _358 = _355 ^ ((_355 << 7u) & 2636928640u);
    uint _361 = _358 ^ ((_358 << 15u) & 4022730752u);
    uint _367 = (((_336.x * 1664525u) + (_361 ^ (_361 >> 18u))) + 1013904223u) * 1664525u;
    uint _369 = _367 ^ (_367 >> 11u);
    uint _372 = _369 ^ ((_369 << 7u) & 2636928640u);
    uint _375 = _372 ^ ((_372 << 15u) & 4022730752u);
    uint _377 = _375 ^ (_375 >> 18u);
    uint _379 = _377 + (_377 << 10u);
    uint _381 = _379 ^ (_379 >> 6u);
    uint _383 = _381 + (_381 << 3u);
    uint _385 = _383 ^ (_383 >> 11u);
    bool _388 = _319 ? _319 : true;
    uint _389 = _319 ? (_293 ? 8388607u : _167) : 8388607u;
    uint _390 = _319 ? (_293 ? 1065353216u : _167) : 1065353216u;
    bool _395 = _388 ? _388 : true;
    uint _396 = _388 ? _389 : 8388607u;
    uint _397 = _388 ? _390 : 1065353216u;
    uint3 _404 = uint3(_334 + int3(1, 0, 0));
    uint _407 = _404.z;
    uint _409 = _407 + (_407 << 10u);
    uint _411 = _409 ^ (_409 >> 6u);
    uint _413 = _411 + (_411 << 3u);
    uint _415 = _413 ^ (_413 >> 11u);
    uint _421 = (((_404.y * 1664525u) + (_415 + (_415 << 15u))) + 1013904223u) * 1664525u;
    uint _423 = _421 ^ (_421 >> 11u);
    uint _426 = _423 ^ ((_423 << 7u) & 2636928640u);
    uint _429 = _426 ^ ((_426 << 15u) & 4022730752u);
    uint _435 = (((_404.x * 1664525u) + (_429 ^ (_429 >> 18u))) + 1013904223u) * 1664525u;
    uint _437 = _435 ^ (_435 >> 11u);
    uint _440 = _437 ^ ((_437 << 7u) & 2636928640u);
    uint _443 = _440 ^ ((_440 << 15u) & 4022730752u);
    uint _445 = _443 ^ (_443 >> 18u);
    uint _447 = _445 + (_445 << 10u);
    uint _449 = _447 ^ (_447 >> 6u);
    uint _451 = _449 + (_449 << 3u);
    uint _453 = _451 ^ (_451 >> 11u);
    bool _456 = _395 ? _395 : true;
    uint _457 = _395 ? _396 : 8388607u;
    uint _458 = _395 ? _397 : 1065353216u;
    bool _463 = _456 ? _456 : true;
    uint _464 = _456 ? _457 : 8388607u;
    uint _465 = _456 ? _458 : 1065353216u;
    uint3 _472 = uint3(_334 + int3(0, 1, 0));
    uint _475 = _472.z;
    uint _477 = _475 + (_475 << 10u);
    uint _479 = _477 ^ (_477 >> 6u);
    uint _481 = _479 + (_479 << 3u);
    uint _483 = _481 ^ (_481 >> 11u);
    uint _489 = (((_472.y * 1664525u) + (_483 + (_483 << 15u))) + 1013904223u) * 1664525u;
    uint _491 = _489 ^ (_489 >> 11u);
    uint _494 = _491 ^ ((_491 << 7u) & 2636928640u);
    uint _497 = _494 ^ ((_494 << 15u) & 4022730752u);
    uint _503 = (((_472.x * 1664525u) + (_497 ^ (_497 >> 18u))) + 1013904223u) * 1664525u;
    uint _505 = _503 ^ (_503 >> 11u);
    uint _508 = _505 ^ ((_505 << 7u) & 2636928640u);
    uint _511 = _508 ^ ((_508 << 15u) & 4022730752u);
    uint _513 = _511 ^ (_511 >> 18u);
    uint _515 = _513 + (_513 << 10u);
    uint _517 = _515 ^ (_515 >> 6u);
    uint _519 = _517 + (_517 << 3u);
    uint _521 = _519 ^ (_519 >> 11u);
    bool _524 = _463 ? _463 : true;
    uint _525 = _463 ? _464 : 8388607u;
    uint _526 = _463 ? _465 : 1065353216u;
    bool _531 = _524 ? _524 : true;
    uint _532 = _524 ? _525 : 8388607u;
    uint _533 = _524 ? _526 : 1065353216u;
    uint3 _540 = uint3(_334 + int3(1, 1, 0));
    uint _543 = _540.z;
    uint _545 = _543 + (_543 << 10u);
    uint _547 = _545 ^ (_545 >> 6u);
    uint _549 = _547 + (_547 << 3u);
    uint _551 = _549 ^ (_549 >> 11u);
    uint _557 = (((_540.y * 1664525u) + (_551 + (_551 << 15u))) + 1013904223u) * 1664525u;
    uint _559 = _557 ^ (_557 >> 11u);
    uint _562 = _559 ^ ((_559 << 7u) & 2636928640u);
    uint _565 = _562 ^ ((_562 << 15u) & 4022730752u);
    uint _571 = (((_540.x * 1664525u) + (_565 ^ (_565 >> 18u))) + 1013904223u) * 1664525u;
    uint _573 = _571 ^ (_571 >> 11u);
    uint _576 = _573 ^ ((_573 << 7u) & 2636928640u);
    uint _579 = _576 ^ ((_576 << 15u) & 4022730752u);
    uint _581 = _579 ^ (_579 >> 18u);
    uint _583 = _581 + (_581 << 10u);
    uint _585 = _583 ^ (_583 >> 6u);
    uint _587 = _585 + (_585 << 3u);
    uint _589 = _587 ^ (_587 >> 11u);
    bool _592 = _531 ? _531 : true;
    uint _593 = _531 ? _532 : 8388607u;
    uint _594 = _531 ? _533 : 1065353216u;
    bool _599 = _592 ? _592 : true;
    uint _600 = _592 ? _593 : 8388607u;
    uint _601 = _592 ? _594 : 1065353216u;
    uint3 _608 = uint3(_334 + int3(0, 0, 1));
    uint _611 = _608.z;
    uint _613 = _611 + (_611 << 10u);
    uint _615 = _613 ^ (_613 >> 6u);
    uint _617 = _615 + (_615 << 3u);
    uint _619 = _617 ^ (_617 >> 11u);
    uint _625 = (((_608.y * 1664525u) + (_619 + (_619 << 15u))) + 1013904223u) * 1664525u;
    uint _627 = _625 ^ (_625 >> 11u);
    uint _630 = _627 ^ ((_627 << 7u) & 2636928640u);
    uint _633 = _630 ^ ((_630 << 15u) & 4022730752u);
    uint _639 = (((_608.x * 1664525u) + (_633 ^ (_633 >> 18u))) + 1013904223u) * 1664525u;
    uint _641 = _639 ^ (_639 >> 11u);
    uint _644 = _641 ^ ((_641 << 7u) & 2636928640u);
    uint _647 = _644 ^ ((_644 << 15u) & 4022730752u);
    uint _649 = _647 ^ (_647 >> 18u);
    uint _651 = _649 + (_649 << 10u);
    uint _653 = _651 ^ (_651 >> 6u);
    uint _655 = _653 + (_653 << 3u);
    uint _657 = _655 ^ (_655 >> 11u);
    bool _660 = _599 ? _599 : true;
    uint _661 = _599 ? _600 : 8388607u;
    uint _662 = _599 ? _601 : 1065353216u;
    bool _667 = _660 ? _660 : true;
    uint _668 = _660 ? _661 : 8388607u;
    uint _669 = _660 ? _662 : 1065353216u;
    uint3 _676 = uint3(_334 + int3(1, 0, 1));
    uint _679 = _676.z;
    uint _681 = _679 + (_679 << 10u);
    uint _683 = _681 ^ (_681 >> 6u);
    uint _685 = _683 + (_683 << 3u);
    uint _687 = _685 ^ (_685 >> 11u);
    uint _693 = (((_676.y * 1664525u) + (_687 + (_687 << 15u))) + 1013904223u) * 1664525u;
    uint _695 = _693 ^ (_693 >> 11u);
    uint _698 = _695 ^ ((_695 << 7u) & 2636928640u);
    uint _701 = _698 ^ ((_698 << 15u) & 4022730752u);
    uint _707 = (((_676.x * 1664525u) + (_701 ^ (_701 >> 18u))) + 1013904223u) * 1664525u;
    uint _709 = _707 ^ (_707 >> 11u);
    uint _712 = _709 ^ ((_709 << 7u) & 2636928640u);
    uint _715 = _712 ^ ((_712 << 15u) & 4022730752u);
    uint _717 = _715 ^ (_715 >> 18u);
    uint _719 = _717 + (_717 << 10u);
    uint _721 = _719 ^ (_719 >> 6u);
    uint _723 = _721 + (_721 << 3u);
    uint _725 = _723 ^ (_723 >> 11u);
    bool _728 = _667 ? _667 : true;
    uint _729 = _667 ? _668 : 8388607u;
    uint _730 = _667 ? _669 : 1065353216u;
    bool _735 = _728 ? _728 : true;
    uint _736 = _728 ? _729 : 8388607u;
    uint _737 = _728 ? _730 : 1065353216u;
    uint3 _744 = uint3(_334 + int3(0, 1, 1));
    uint _747 = _744.z;
    uint _749 = _747 + (_747 << 10u);
    uint _751 = _749 ^ (_749 >> 6u);
    uint _753 = _751 + (_751 << 3u);
    uint _755 = _753 ^ (_753 >> 11u);
    uint _761 = (((_744.y * 1664525u) + (_755 + (_755 << 15u))) + 1013904223u) * 1664525u;
    uint _763 = _761 ^ (_761 >> 11u);
    uint _766 = _763 ^ ((_763 << 7u) & 2636928640u);
    uint _769 = _766 ^ ((_766 << 15u) & 4022730752u);
    uint _775 = (((_744.x * 1664525u) + (_769 ^ (_769 >> 18u))) + 1013904223u) * 1664525u;
    uint _777 = _775 ^ (_775 >> 11u);
    uint _780 = _777 ^ ((_777 << 7u) & 2636928640u);
    uint _783 = _780 ^ ((_780 << 15u) & 4022730752u);
    uint _785 = _783 ^ (_783 >> 18u);
    uint _787 = _785 + (_785 << 10u);
    uint _789 = _787 ^ (_787 >> 6u);
    uint _791 = _789 + (_789 << 3u);
    uint _793 = _791 ^ (_791 >> 11u);
    bool _796 = _735 ? _735 : true;
    uint _797 = _735 ? _736 : 8388607u;
    uint _798 = _735 ? _737 : 1065353216u;
    bool _803 = _796 ? _796 : true;
    uint _804 = _796 ? _797 : 8388607u;
    uint _805 = _796 ? _798 : 1065353216u;
    uint3 _812 = uint3(_334 + int3(1, 1, 1));
    uint _815 = _812.z;
    uint _817 = _815 + (_815 << 10u);
    uint _819 = _817 ^ (_817 >> 6u);
    uint _821 = _819 + (_819 << 3u);
    uint _823 = _821 ^ (_821 >> 11u);
    uint _829 = (((_812.y * 1664525u) + (_823 + (_823 << 15u))) + 1013904223u) * 1664525u;
    uint _831 = _829 ^ (_829 >> 11u);
    uint _834 = _831 ^ ((_831 << 7u) & 2636928640u);
    uint _837 = _834 ^ ((_834 << 15u) & 4022730752u);
    uint _843 = (((_812.x * 1664525u) + (_837 ^ (_837 >> 18u))) + 1013904223u) * 1664525u;
    uint _845 = _843 ^ (_843 >> 11u);
    uint _848 = _845 ^ ((_845 << 7u) & 2636928640u);
    uint _851 = _848 ^ ((_848 << 15u) & 4022730752u);
    uint _853 = _851 ^ (_851 >> 18u);
    uint _855 = _853 + (_853 << 10u);
    uint _857 = _855 ^ (_855 >> 6u);
    uint _859 = _857 + (_857 << 3u);
    uint _861 = _859 ^ (_859 >> 11u);
    bool _864 = _803 ? _803 : true;
    uint _865 = _803 ? _804 : 8388607u;
    uint _866 = _803 ? _805 : 1065353216u;
    bool _871 = _864 ? _864 : true;
    uint _872 = _864 ? _865 : 8388607u;
    uint _873 = _864 ? _866 : 1065353216u;
    float3 _882 = (_335 * _335) * (3.0f.xxx - (_335 * 2.0f));
    float _883 = _882.x;
    float _884 = 1.0f - _883;
    float _886 = _882.y;
    float _887 = 1.0f - _886;
    float _889 = _882.z;
    float _890 = 1.0f - _889;
    float2 _903 = (((((float2(asfloat((_377 & _389) | _390) - 1.0f, asfloat(((_385 + (_385 << 15u)) & _396) | _397) - 1.0f) * _884) * _887) * _890) + (((float2(asfloat((_445 & _457) | _458) - 1.0f, asfloat(((_453 + (_453 << 15u)) & _464) | _465) - 1.0f) * _883) * _887) * _890)) + (((float2(asfloat((_513 & _525) | _526) - 1.0f, asfloat(((_521 + (_521 << 15u)) & _532) | _533) - 1.0f) * _884) * _886) * _890)) + (((float2(asfloat((_581 & _593) | _594) - 1.0f, asfloat(((_589 + (_589 << 15u)) & _600) | _601) - 1.0f) * _883) * _886) * _890);
    float2 _919 = (((_903 + (((float2(asfloat((_649 & _661) | _662) - 1.0f, asfloat(((_657 + (_657 << 15u)) & _668) | _669) - 1.0f) * _884) * _887) * _889)) + (((float2(asfloat((_717 & _729) | _730) - 1.0f, asfloat(((_725 + (_725 << 15u)) & _736) | _737) - 1.0f) * _883) * _887) * _889)) + (((float2(asfloat((_785 & _797) | _798) - 1.0f, asfloat(((_793 + (_793 << 15u)) & _804) | _805) - 1.0f) * _884) * _886) * _889)) + (((float2(asfloat((_853 & _865) | _866) - 1.0f, asfloat(((_861 + (_861 << 15u)) & _872) | _873) - 1.0f) * _883) * _886) * _889);
    float2 _922 = clamp((_919 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _927 = _331 + (float3(_922.x, 0.0f, _922.y) * 0.0599999986588954925537109375f);
    float4 _928 = float4(_927.x, _927.y, _927.z, _330.w);
    float3 _929 = mul(_259, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _940 = mul(float3x3(_233[0].xyz, _233[1].xyz, _233[2].xyz), in_var_Vertex_Position);
    float _942 = _940.x - (-0.5f);
    float _953 = _927.y + lerp(lerp(instanced_wall_data.Load<float4>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 48).x, instanced_wall_data.Load<float4>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 48).y, _942), lerp(instanced_wall_data.Load<float4>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 48).z, instanced_wall_data.Load<float4>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 48).w, _942), _940.y - (-0.5f));
    _928.y = _953;
    float3x3 _963 = float3x3(_257[0].xyz, _258[1].xyz, _259[2].xyz);
    float3 _964 = mul(_963, float3(1.0f, 0.0f, 0.0f));
    float3 _965 = mul(_963, float3(0.0f, 1.0f, 0.0f));
    float3 _966 = mul(_963, float3(0.0f, 0.0f, 1.0f));
    float3 _969 = cross(_964, _965);
    float3 _972 = (1.0f / dot(_966, _969)).xxx;
    bool _986 = in_var_is_bevel != 0;
    int _990;
    if (_986)
    {
        _990 = _274 | 256;
    }
    else
    {
        _990 = _274;
    }
    int _996;
    if (instanced_wall_data.Load<uint>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 76) < 10u)
    {
        _996 = _990 | 512;
    }
    else
    {
        _996 = _990;
    }
    float _1016;
    if (_942 < 0.5f)
    {
        _1016 = lerp(instanced_wall_data.Load<float3>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 64).x, instanced_wall_data.Load<float3>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 64).y, _942 * 2.0f);
    }
    else
    {
        _1016 = lerp(instanced_wall_data.Load<float3>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 64).y, instanced_wall_data.Load<float3>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 64).z, (_942 * 2.0f) - 1.0f);
    }
    bool _1018 = (_996 & 32) != 0;
    float2 _1062;
    if (_1018)
    {
        float2 _1037;
        if (_270.y > 0.89999997615814208984375f)
        {
            _1037 = _928.xz * 0.699999988079071044921875f;
        }
        else
        {
            float2 _1034;
            if (abs(_270.x) > 0.89999997615814208984375f)
            {
                _1034 = _928.zy * 0.699999988079071044921875f;
            }
            else
            {
                _1034 = float2(lerp(instanced_wall_data.Load<float2>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 84).x, instanced_wall_data.Load<float2>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 84).y, _942), _953) * 0.699999988079071044921875f;
            }
            _1037 = _1034;
        }
        float2 _1041;
        if (_986)
        {
            _1041 = instanced_wall_data.Load<float2>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 84).x.xx;
        }
        else
        {
            _1041 = _1037;
        }
        uint _1045 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 76) + (instanced_wall_data.Load<uint>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 76) << 10u);
        uint _1047 = _1045 ^ (_1045 >> 6u);
        uint _1049 = _1047 + (_1047 << 3u);
        uint _1051 = _1049 ^ (_1049 >> 11u);
        _1062 = (_1041 * 0.300000011920928955078125f) + (asfloat(((_1051 + (_1051 << 15u)) & (_871 ? _872 : 8388607u)) | (_871 ? _873 : 1065353216u)) - 1.0f).xx;
    }
    else
    {
        _1062 = 0.0f.xx;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, _928);
    out_var_C0 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(_190 * 4 + 0) * 96 + 76) & 65535u;
    out_var_C1 = mul(transpose(float3x3(cross(_965, _966) * _972, cross(_966, _964) * _972, _969 * _972)), _270.xyz);
    out_var_C2 = _928.xyz;
    out_var_C5 = 0.0f.xxx;
    out_var_C7 = _1018 ? (-100.0f) : _1016;
    out_var_C10 = _1062;
    out_var_C11 = 0;
    out_var_C12 = float4(_929.x, _178, _929.z, 1.0f).xz;
    out_var_C13 = _996;
    out_var_C14 = instance_list.Load<uint>(_190 * 4 + 0);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_is_bevel = stage_input.in_var_is_bevel;
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
