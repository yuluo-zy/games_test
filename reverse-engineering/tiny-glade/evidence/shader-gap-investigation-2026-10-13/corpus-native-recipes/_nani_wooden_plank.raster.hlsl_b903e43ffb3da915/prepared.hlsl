#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/nani_wooden_plank.raster.hlsl"




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

struct WoodenPlank
{
    Affine3Packed transform;
    int gate_seed;
    int piece_seed;
    uint wall_id;
    uint flags;
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

cbuffer $Globals : register(b13, space0)
{
    uint _Globals_current_view_index : packoffset(c0);
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    int _Globals_highlight_bricks_to_delete : packoffset(c5);
    float3 _Globals_brush_position : packoffset(c5.y);
    float _Globals_eraser_mult : packoffset(c6);
    uint2 _Globals_dirthpath_heightmap_dims : packoffset(c6.y);
    uint2 _Globals_wood_roughness_dims : packoffset(c7);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_bevel;
static float3 out_var_C0;
static float3 out_var_C1;
static int out_var_C3;
static int out_var_C4;
static uint out_var_C5;
static float2 out_var_C6;
static int out_var_C7;
static int out_var_C8;
static int out_var_C9;
static int out_var_C10;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_bevel: is_bevel;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(3)]] nointerpolation int out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation int out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation uint out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation int out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation int out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] nointerpolation int out_var_C9 : TEXCOORD9;
    [[vk::location(10)]] nointerpolation int out_var_C10 : TEXCOORD10;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 _159 = in_var_Vertex_Position;
    float _167[12];
    [unroll]
    for (int _5ident = 0; _5ident < 12; _5ident++)
    {
        _167[_5ident] = nani_instance_data.Load<float>(_5ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _185 = transpose(float4x3(float3(_167[0], _167[1], _167[2]), float3(_167[3], _167[4], _167[5]), float3(_167[6], _167[7], _167[8]), float3(_167[9], _167[10], _167[11])));
    int _190 = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48) + nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    int _191 = _190 + 45;
    uint _192 = uint(_191);
    uint _194 = _192 + (_192 << 10u);
    uint _196 = _194 ^ (_194 >> 6u);
    uint _198 = _196 + (_196 << 3u);
    uint _200 = _198 ^ (_198 >> 11u);
    float _210 = 1.57000005245208740234375f * float(int((asfloat(((_200 + (_200 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 4.0f));
    float3 _211 = normalize(float3(0.0f, 1.0f, 0.0f));
    float _212 = sin(_210);
    float _213 = cos(_210);
    float _214 = 1.0f - _213;
    float _215 = _211.x;
    float _216 = _214 * _215;
    float _219 = _211.y;
    float _220 = _216 * _219;
    float _221 = _211.z;
    float _222 = _221 * _212;
    float _224 = _214 * _221;
    float _225 = _224 * _215;
    float _226 = _219 * _212;
    float _229 = _214 * _219;
    float _232 = _229 * _221;
    float _233 = _215 * _212;
    float4x4 _243 = transpose(float4x4(float4((_216 * _215) + _213, _220 - _222, _225 + _226, 0.0f), float4(_220 + _222, (_229 * _219) + _213, _232 - _233, 0.0f), float4(_225 - _226, _232 + _233, (_224 * _221) + _213, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    uint _245 = uint(_190 + 500);
    uint _247 = _245 + (_245 << 10u);
    uint _249 = _247 ^ (_247 >> 6u);
    uint _251 = _249 + (_249 << 3u);
    uint _253 = _251 ^ (_251 >> 11u);
    float3 _297;
    float4x4 _298;
    if ((asfloat(((_253 + (_253 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.5f)
    {
        float3 _263 = normalize(float3(0.0f, 0.0f, 1.0f));
        float _264 = _263.x;
        float _265 = 1.9999988079071044921875f * _264;
        float _268 = _263.y;
        float _269 = _265 * _268;
        float _270 = _263.z;
        float _271 = _270 * 0.001592547981999814510345458984375f;
        float _273 = 1.9999988079071044921875f * _270;
        float _274 = _273 * _264;
        float _275 = _268 * 0.001592547981999814510345458984375f;
        float _278 = 1.9999988079071044921875f * _268;
        float _281 = _278 * _270;
        float _282 = _264 * 0.001592547981999814510345458984375f;
        float3 _296 = _159;
        _296.y = _159.y - 1.0f;
        _297 = _296;
        _298 = mul(_243, transpose(float4x4(float4((_265 * _264) + (-0.999998748302459716796875f), _269 - _271, _274 + _275, 0.0f), float4(_269 + _271, (_278 * _268) + (-0.999998748302459716796875f), _281 - _282, 0.0f), float4(_274 - _275, _281 + _282, (_273 * _270) + (-0.999998748302459716796875f), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
    }
    else
    {
        _297 = _159;
        _298 = _243;
    }
    float3x4 _299 = mul(_185, _298);
    float4 _308 = float4(mul(_299, float4(_297, 1.0f)), 1.0f);
    float3 _309 = _308.xyz;
    float3 _310 = _309 * 0.4000000059604644775390625f;
    int3 _312 = int3(floor(_310));
    float3 _313 = frac(_310);
    uint3 _314 = uint3(_312);
    uint _317 = _314.z;
    uint _319 = _317 + (_317 << 10u);
    uint _321 = _319 ^ (_319 >> 6u);
    uint _323 = _321 + (_321 << 3u);
    uint _325 = _323 ^ (_323 >> 11u);
    uint _331 = (((_314.y * 1664525u) + (_325 + (_325 << 15u))) + 1013904223u) * 1664525u;
    uint _333 = _331 ^ (_331 >> 11u);
    uint _336 = _333 ^ ((_333 << 7u) & 2636928640u);
    uint _339 = _336 ^ ((_336 << 15u) & 4022730752u);
    uint _345 = (((_314.x * 1664525u) + (_339 ^ (_339 >> 18u))) + 1013904223u) * 1664525u;
    uint _347 = _345 ^ (_345 >> 11u);
    uint _350 = _347 ^ ((_347 << 7u) & 2636928640u);
    uint _353 = _350 ^ ((_350 << 15u) & 4022730752u);
    uint _355 = _353 ^ (_353 >> 18u);
    uint _357 = _355 + (_355 << 10u);
    uint _359 = _357 ^ (_357 >> 6u);
    uint _361 = _359 + (_359 << 3u);
    uint _363 = _361 ^ (_361 >> 11u);
    uint3 _376 = uint3(_312 + int3(1, 0, 0));
    uint _379 = _376.z;
    uint _381 = _379 + (_379 << 10u);
    uint _383 = _381 ^ (_381 >> 6u);
    uint _385 = _383 + (_383 << 3u);
    uint _387 = _385 ^ (_385 >> 11u);
    uint _393 = (((_376.y * 1664525u) + (_387 + (_387 << 15u))) + 1013904223u) * 1664525u;
    uint _395 = _393 ^ (_393 >> 11u);
    uint _398 = _395 ^ ((_395 << 7u) & 2636928640u);
    uint _401 = _398 ^ ((_398 << 15u) & 4022730752u);
    uint _407 = (((_376.x * 1664525u) + (_401 ^ (_401 >> 18u))) + 1013904223u) * 1664525u;
    uint _409 = _407 ^ (_407 >> 11u);
    uint _412 = _409 ^ ((_409 << 7u) & 2636928640u);
    uint _415 = _412 ^ ((_412 << 15u) & 4022730752u);
    uint _417 = _415 ^ (_415 >> 18u);
    uint _419 = _417 + (_417 << 10u);
    uint _421 = _419 ^ (_419 >> 6u);
    uint _423 = _421 + (_421 << 3u);
    uint _425 = _423 ^ (_423 >> 11u);
    uint3 _438 = uint3(_312 + int3(0, 1, 0));
    uint _441 = _438.z;
    uint _443 = _441 + (_441 << 10u);
    uint _445 = _443 ^ (_443 >> 6u);
    uint _447 = _445 + (_445 << 3u);
    uint _449 = _447 ^ (_447 >> 11u);
    uint _455 = (((_438.y * 1664525u) + (_449 + (_449 << 15u))) + 1013904223u) * 1664525u;
    uint _457 = _455 ^ (_455 >> 11u);
    uint _460 = _457 ^ ((_457 << 7u) & 2636928640u);
    uint _463 = _460 ^ ((_460 << 15u) & 4022730752u);
    uint _469 = (((_438.x * 1664525u) + (_463 ^ (_463 >> 18u))) + 1013904223u) * 1664525u;
    uint _471 = _469 ^ (_469 >> 11u);
    uint _474 = _471 ^ ((_471 << 7u) & 2636928640u);
    uint _477 = _474 ^ ((_474 << 15u) & 4022730752u);
    uint _479 = _477 ^ (_477 >> 18u);
    uint _481 = _479 + (_479 << 10u);
    uint _483 = _481 ^ (_481 >> 6u);
    uint _485 = _483 + (_483 << 3u);
    uint _487 = _485 ^ (_485 >> 11u);
    uint3 _500 = uint3(_312 + int3(1, 1, 0));
    uint _503 = _500.z;
    uint _505 = _503 + (_503 << 10u);
    uint _507 = _505 ^ (_505 >> 6u);
    uint _509 = _507 + (_507 << 3u);
    uint _511 = _509 ^ (_509 >> 11u);
    uint _517 = (((_500.y * 1664525u) + (_511 + (_511 << 15u))) + 1013904223u) * 1664525u;
    uint _519 = _517 ^ (_517 >> 11u);
    uint _522 = _519 ^ ((_519 << 7u) & 2636928640u);
    uint _525 = _522 ^ ((_522 << 15u) & 4022730752u);
    uint _531 = (((_500.x * 1664525u) + (_525 ^ (_525 >> 18u))) + 1013904223u) * 1664525u;
    uint _533 = _531 ^ (_531 >> 11u);
    uint _536 = _533 ^ ((_533 << 7u) & 2636928640u);
    uint _539 = _536 ^ ((_536 << 15u) & 4022730752u);
    uint _541 = _539 ^ (_539 >> 18u);
    uint _543 = _541 + (_541 << 10u);
    uint _545 = _543 ^ (_543 >> 6u);
    uint _547 = _545 + (_545 << 3u);
    uint _549 = _547 ^ (_547 >> 11u);
    uint3 _562 = uint3(_312 + int3(0, 0, 1));
    uint _565 = _562.z;
    uint _567 = _565 + (_565 << 10u);
    uint _569 = _567 ^ (_567 >> 6u);
    uint _571 = _569 + (_569 << 3u);
    uint _573 = _571 ^ (_571 >> 11u);
    uint _579 = (((_562.y * 1664525u) + (_573 + (_573 << 15u))) + 1013904223u) * 1664525u;
    uint _581 = _579 ^ (_579 >> 11u);
    uint _584 = _581 ^ ((_581 << 7u) & 2636928640u);
    uint _587 = _584 ^ ((_584 << 15u) & 4022730752u);
    uint _593 = (((_562.x * 1664525u) + (_587 ^ (_587 >> 18u))) + 1013904223u) * 1664525u;
    uint _595 = _593 ^ (_593 >> 11u);
    uint _598 = _595 ^ ((_595 << 7u) & 2636928640u);
    uint _601 = _598 ^ ((_598 << 15u) & 4022730752u);
    uint _603 = _601 ^ (_601 >> 18u);
    uint _605 = _603 + (_603 << 10u);
    uint _607 = _605 ^ (_605 >> 6u);
    uint _609 = _607 + (_607 << 3u);
    uint _611 = _609 ^ (_609 >> 11u);
    uint3 _624 = uint3(_312 + int3(1, 0, 1));
    uint _627 = _624.z;
    uint _629 = _627 + (_627 << 10u);
    uint _631 = _629 ^ (_629 >> 6u);
    uint _633 = _631 + (_631 << 3u);
    uint _635 = _633 ^ (_633 >> 11u);
    uint _641 = (((_624.y * 1664525u) + (_635 + (_635 << 15u))) + 1013904223u) * 1664525u;
    uint _643 = _641 ^ (_641 >> 11u);
    uint _646 = _643 ^ ((_643 << 7u) & 2636928640u);
    uint _649 = _646 ^ ((_646 << 15u) & 4022730752u);
    uint _655 = (((_624.x * 1664525u) + (_649 ^ (_649 >> 18u))) + 1013904223u) * 1664525u;
    uint _657 = _655 ^ (_655 >> 11u);
    uint _660 = _657 ^ ((_657 << 7u) & 2636928640u);
    uint _663 = _660 ^ ((_660 << 15u) & 4022730752u);
    uint _665 = _663 ^ (_663 >> 18u);
    uint _667 = _665 + (_665 << 10u);
    uint _669 = _667 ^ (_667 >> 6u);
    uint _671 = _669 + (_669 << 3u);
    uint _673 = _671 ^ (_671 >> 11u);
    uint3 _686 = uint3(_312 + int3(0, 1, 1));
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
    uint3 _748 = uint3(_312 + int3(1, 1, 1));
    uint _751 = _748.z;
    uint _753 = _751 + (_751 << 10u);
    uint _755 = _753 ^ (_753 >> 6u);
    uint _757 = _755 + (_755 << 3u);
    uint _759 = _757 ^ (_757 >> 11u);
    uint _765 = (((_748.y * 1664525u) + (_759 + (_759 << 15u))) + 1013904223u) * 1664525u;
    uint _767 = _765 ^ (_765 >> 11u);
    uint _770 = _767 ^ ((_767 << 7u) & 2636928640u);
    uint _773 = _770 ^ ((_770 << 15u) & 4022730752u);
    uint _779 = (((_748.x * 1664525u) + (_773 ^ (_773 >> 18u))) + 1013904223u) * 1664525u;
    uint _781 = _779 ^ (_779 >> 11u);
    uint _784 = _781 ^ ((_781 << 7u) & 2636928640u);
    uint _787 = _784 ^ ((_784 << 15u) & 4022730752u);
    uint _789 = _787 ^ (_787 >> 18u);
    uint _791 = _789 + (_789 << 10u);
    uint _793 = _791 ^ (_791 >> 6u);
    uint _795 = _793 + (_793 << 3u);
    uint _797 = _795 ^ (_795 >> 11u);
    float3 _812 = (_313 * _313) * (3.0f.xxx - (_313 * 2.0f));
    float _813 = _812.x;
    float _814 = 1.0f - _813;
    float _816 = _812.y;
    float _817 = 1.0f - _816;
    float _819 = _812.z;
    float _820 = 1.0f - _819;
    float2 _837 = ((((((float2(asfloat((_355 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_363 + (_363 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _814) * _817) * _820) + (((float2(asfloat((_417 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_425 + (_425 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _813) * _817) * _820)) + (((float2(asfloat((_479 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_487 + (_487 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _814) * _816) * _820)) + (((float2(asfloat((_541 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_549 + (_549 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _813) * _816) * _820)) + (((float2(asfloat((_603 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_611 + (_611 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _814) * _817) * _819);
    float2 _852 = clamp(((((_837 + (((float2(asfloat((_665 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_673 + (_673 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _813) * _817) * _819)) + (((float2(asfloat((_727 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_735 + (_735 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _814) * _816) * _819)) + (((float2(asfloat((_789 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_797 + (_797 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _813) * _816) * _819)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _857 = _309 + (float3(_852.x, 0.0f, _852.y) * 0.0599999986588954925537109375f);
    float3x3 _865 = float3x3(_299[0].xyz, _299[1].xyz, _299[2].xyz);
    float3 _866 = mul(_865, float3(1.0f, 0.0f, 0.0f));
    float3 _867 = mul(_865, float3(0.0f, 1.0f, 0.0f));
    float3 _868 = mul(_865, float3(0.0f, 0.0f, 1.0f));
    float3 _871 = cross(_866, _867);
    float3 _874 = (1.0f / dot(_868, _871)).xxx;
    uint _899 = uint(_191 ^ 2340774);
    uint _901 = _899 + (_899 << 10u);
    uint _903 = _901 ^ (_901 >> 6u);
    uint _905 = _903 + (_903 << 3u);
    uint _907 = _905 ^ (_905 >> 11u);
    uint _915 = uint(_191 ^ 934565);
    uint _917 = _915 + (_915 << 10u);
    uint _919 = _917 ^ (_917 >> 6u);
    uint _921 = _919 + (_919 << 3u);
    uint _923 = _921 ^ (_921 >> 11u);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_857.x, _857.y, _857.z, _308.w));
    out_var_C0 = _857.xyz;
    out_var_C1 = normalize(mul(transpose(float3x3(cross(_867, _868) * _874, cross(_868, _866) * _874, _871 * _874)), in_var_Vertex_Normal));
    out_var_C3 = _191;
    out_var_C4 = in_var_is_bevel;
    out_var_C5 = uint((nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) & 2u) != 0u);
    out_var_C6 = (((in_var_Vertex_UV * 0.4000000059604644775390625f) + float2(asfloat(((_907 + (_907 << 15u)) & 8388607u) | 1065353216u) - 1.0f, asfloat(((_923 + (_923 << 15u)) & 8388607u) | 1065353216u) - 1.0f)) + (float(in_var_is_bevel) * 0.12000000476837158203125f).xx) * float2(1.0f, length(float3(_185[0].y, _185[1].y, _185[2].y)) * 0.4000000059604644775390625f);
    out_var_C7 = int(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56));
    out_var_C8 = int(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) & 4u);
    out_var_C9 = int(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) & 8u);
    out_var_C10 = int(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) & 16u);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_bevel = stage_input.in_var_is_bevel;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    stage_output.out_var_C9 = out_var_C9;
    stage_output.out_var_C10 = out_var_C10;
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
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    int _Globals_highlight_bricks_to_delete : packoffset(c5);
    float3 _Globals_brush_position : packoffset(c5.y);
    float _Globals_eraser_mult : packoffset(c6);
    uint2 _Globals_dirthpath_heightmap_dims : packoffset(c6.y);
    uint2 _Globals_wood_roughness_dims : packoffset(c7);
};

ByteAddressBuffer halftimber_color_ids : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2DArray<float4> halftimber_colors : register(t18, space0);
Texture2D<float4> dirthpath_heightmap : register(t19, space0);
Texture2D<float4> wood_roughness : register(t20, space0);

static float3 in_var_C0;
static float3 in_var_C1;
static int in_var_C4;
static uint in_var_C5;
static float2 in_var_C6;
static int in_var_C7;
static int in_var_C8;
static int in_var_C9;
static int in_var_C10;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0: Vertex_Position;
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(4)]] nointerpolation int in_var_C4: is_bevel;
    [[vk::location(5)]] nointerpolation uint in_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 in_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation int in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation int in_var_C8 : TEXCOORD8;
    [[vk::location(9)]] nointerpolation int in_var_C9 : TEXCOORD9;
    [[vk::location(10)]] nointerpolation int in_var_C10 : TEXCOORD10;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float3 _136 = normalize(in_var_C1);
    float4 _153 = halftimber_colors.SampleBias(sampler_llr, float3(in_var_C6, float((in_var_C10 > 0) ? in_var_C7 : ((in_var_C5 != 0u) ? 0 : int(halftimber_color_ids.Load<uint>(uint(in_var_C7) * 4 + 0))))), frame_constants_global_mip_bias);
    float3 _154 = _153.xyz;
    float3 _158 = lerp(_154, _154 * 1.10000002384185791015625f, float(in_var_C4).xxx);
    float4 _162 = wood_roughness.SampleBias(sampler_llr, in_var_C6 * 1.0f, frame_constants_global_mip_bias);
    float _165 = lerp(0.699999988079071044921875f, 0.89999997615814208984375f, smoothstep(0.89999997615814208984375f, 1.0f, _162.x));
    bool _172;
    if (in_var_C9 == 0)
    {
        _172 = _Globals_highlight_bricks_to_delete == 1;
    }
    else
    {
        _172 = false;
    }
    bool _190;
    float3 _191;
    if (_172)
    {
        bool _184 = distance(_Globals_brush_position.xz, in_var_C0.xz) < ((1.88900005817413330078125f * _Globals_eraser_mult) - 0.20000000298023223876953125f);
        float3 _188;
        if (_184)
        {
            _188 = lerp(_158, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.800000011920928955078125f.xxx);
        }
        else
        {
            _188 = _158;
        }
        _190 = _184 ? true : false;
        _191 = _188;
    }
    else
    {
        _190 = false;
        _191 = _158;
    }
    float3 _229;
    if (frame_constants_glade_theme_id == 4u)
    {
        float _198 = smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C0.y);
        float3 _228;
        if (in_var_C8 > 0)
        {
            float4 _207 = dirthpath_heightmap.SampleBias(sampler_llr, in_var_C0.xz * 0.0500000007450580596923828125f, frame_constants_global_mip_bias);
            float _208 = _207.x;
            _228 = lerp(_191, 0.699999988079071044921875f.xxx, ((((smoothstep(0.0f, 0.20000000298023223876953125f, smoothstep(0.5f, 1.0f, _165) * lerp(0.0f, 0.5f, _208)) * 0.60000002384185791015625f) + (_208 * 0.20000000298023223876953125f)) * clamp(_136.y, 0.0f, 1.0f)) * _198).xxx);
        }
        else
        {
            _228 = lerp(_191, 0.699999988079071044921875f.xxx, (_198 * smoothstep(0.5f, 1.0f, clamp(_136.y, 0.0f, 1.0f))).xxx);
        }
        _229 = _228;
    }
    else
    {
        _229 = _191;
    }
    float3 _231 = (_136 * 0.5f) + 0.5f.xxx;
    float3 _258 = sqrt(clamp(_229, 0.0f.xxx, 1.0f.xxx));
    uint _277 = (uint((clamp(_258.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_258.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_258.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _278 = uint2(((uint((clamp(_231.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_231.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_231.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_165, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), _277);
    _278.y = _277 | (((((7u | (uint(_190) << 3u)) | 0u) | 0u) | 0u) << 24u);
    out_var_SV_TARGET0 = _278;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C6 = stage_input.in_var_C6;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    in_var_C9 = stage_input.in_var_C9;
    in_var_C10 = stage_input.in_var_C10;
    frag_main();
    SPIRV_Cross_Output stage_output;
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

struct WoodenPlank
{
    Affine3Packed transform;
    int gate_seed;
    int piece_seed;
    uint wall_id;
    uint flags;
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

cbuffer $Globals : register(b13, space0)
{
    uint _Globals_current_view_index : packoffset(c0);
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    int _Globals_highlight_bricks_to_delete : packoffset(c5);
    float3 _Globals_brush_position : packoffset(c5.y);
    float _Globals_eraser_mult : packoffset(c6);
    uint2 _Globals_dirthpath_heightmap_dims : packoffset(c6.y);
    uint2 _Globals_wood_roughness_dims : packoffset(c7);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_bevel;
static float3 out_var_C0;
static float3 out_var_C1;
static int out_var_C3;
static int out_var_C4;
static uint out_var_C5;
static float2 out_var_C6;
static int out_var_C7;
static int out_var_C8;
static int out_var_C9;
static int out_var_C10;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_bevel: is_bevel;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(3)]] nointerpolation int out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation int out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation uint out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation int out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation int out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] nointerpolation int out_var_C9 : TEXCOORD9;
    [[vk::location(10)]] nointerpolation int out_var_C10 : TEXCOORD10;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 _159 = in_var_Vertex_Position;
    float _167[12];
    [unroll]
    for (int _5ident = 0; _5ident < 12; _5ident++)
    {
        _167[_5ident] = nani_instance_data.Load<float>(_5ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _185 = transpose(float4x3(float3(_167[0], _167[1], _167[2]), float3(_167[3], _167[4], _167[5]), float3(_167[6], _167[7], _167[8]), float3(_167[9], _167[10], _167[11])));
    int _190 = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48) + nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    int _191 = _190 + 45;
    uint _192 = uint(_191);
    uint _194 = _192 + (_192 << 10u);
    uint _196 = _194 ^ (_194 >> 6u);
    uint _198 = _196 + (_196 << 3u);
    uint _200 = _198 ^ (_198 >> 11u);
    float _210 = 1.57000005245208740234375f * float(int((asfloat(((_200 + (_200 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 4.0f));
    float3 _211 = normalize(float3(0.0f, 1.0f, 0.0f));
    float _212 = sin(_210);
    float _213 = cos(_210);
    float _214 = 1.0f - _213;
    float _215 = _211.x;
    float _216 = _214 * _215;
    float _219 = _211.y;
    float _220 = _216 * _219;
    float _221 = _211.z;
    float _222 = _221 * _212;
    float _224 = _214 * _221;
    float _225 = _224 * _215;
    float _226 = _219 * _212;
    float _229 = _214 * _219;
    float _232 = _229 * _221;
    float _233 = _215 * _212;
    float4x4 _243 = transpose(float4x4(float4((_216 * _215) + _213, _220 - _222, _225 + _226, 0.0f), float4(_220 + _222, (_229 * _219) + _213, _232 - _233, 0.0f), float4(_225 - _226, _232 + _233, (_224 * _221) + _213, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    uint _245 = uint(_190 + 500);
    uint _247 = _245 + (_245 << 10u);
    uint _249 = _247 ^ (_247 >> 6u);
    uint _251 = _249 + (_249 << 3u);
    uint _253 = _251 ^ (_251 >> 11u);
    float3 _297;
    float4x4 _298;
    if ((asfloat(((_253 + (_253 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.5f)
    {
        float3 _263 = normalize(float3(0.0f, 0.0f, 1.0f));
        float _264 = _263.x;
        float _265 = 1.9999988079071044921875f * _264;
        float _268 = _263.y;
        float _269 = _265 * _268;
        float _270 = _263.z;
        float _271 = _270 * 0.001592547981999814510345458984375f;
        float _273 = 1.9999988079071044921875f * _270;
        float _274 = _273 * _264;
        float _275 = _268 * 0.001592547981999814510345458984375f;
        float _278 = 1.9999988079071044921875f * _268;
        float _281 = _278 * _270;
        float _282 = _264 * 0.001592547981999814510345458984375f;
        float3 _296 = _159;
        _296.y = _159.y - 1.0f;
        _297 = _296;
        _298 = mul(_243, transpose(float4x4(float4((_265 * _264) + (-0.999998748302459716796875f), _269 - _271, _274 + _275, 0.0f), float4(_269 + _271, (_278 * _268) + (-0.999998748302459716796875f), _281 - _282, 0.0f), float4(_274 - _275, _281 + _282, (_273 * _270) + (-0.999998748302459716796875f), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
    }
    else
    {
        _297 = _159;
        _298 = _243;
    }
    float3x4 _299 = mul(_185, _298);
    float4 _308 = float4(mul(_299, float4(_297, 1.0f)), 1.0f);
    float3 _309 = _308.xyz;
    float3 _310 = _309 * 0.4000000059604644775390625f;
    int3 _312 = int3(floor(_310));
    float3 _313 = frac(_310);
    uint3 _314 = uint3(_312);
    uint _317 = _314.z;
    uint _319 = _317 + (_317 << 10u);
    uint _321 = _319 ^ (_319 >> 6u);
    uint _323 = _321 + (_321 << 3u);
    uint _325 = _323 ^ (_323 >> 11u);
    uint _331 = (((_314.y * 1664525u) + (_325 + (_325 << 15u))) + 1013904223u) * 1664525u;
    uint _333 = _331 ^ (_331 >> 11u);
    uint _336 = _333 ^ ((_333 << 7u) & 2636928640u);
    uint _339 = _336 ^ ((_336 << 15u) & 4022730752u);
    uint _345 = (((_314.x * 1664525u) + (_339 ^ (_339 >> 18u))) + 1013904223u) * 1664525u;
    uint _347 = _345 ^ (_345 >> 11u);
    uint _350 = _347 ^ ((_347 << 7u) & 2636928640u);
    uint _353 = _350 ^ ((_350 << 15u) & 4022730752u);
    uint _355 = _353 ^ (_353 >> 18u);
    uint _357 = _355 + (_355 << 10u);
    uint _359 = _357 ^ (_357 >> 6u);
    uint _361 = _359 + (_359 << 3u);
    uint _363 = _361 ^ (_361 >> 11u);
    uint3 _376 = uint3(_312 + int3(1, 0, 0));
    uint _379 = _376.z;
    uint _381 = _379 + (_379 << 10u);
    uint _383 = _381 ^ (_381 >> 6u);
    uint _385 = _383 + (_383 << 3u);
    uint _387 = _385 ^ (_385 >> 11u);
    uint _393 = (((_376.y * 1664525u) + (_387 + (_387 << 15u))) + 1013904223u) * 1664525u;
    uint _395 = _393 ^ (_393 >> 11u);
    uint _398 = _395 ^ ((_395 << 7u) & 2636928640u);
    uint _401 = _398 ^ ((_398 << 15u) & 4022730752u);
    uint _407 = (((_376.x * 1664525u) + (_401 ^ (_401 >> 18u))) + 1013904223u) * 1664525u;
    uint _409 = _407 ^ (_407 >> 11u);
    uint _412 = _409 ^ ((_409 << 7u) & 2636928640u);
    uint _415 = _412 ^ ((_412 << 15u) & 4022730752u);
    uint _417 = _415 ^ (_415 >> 18u);
    uint _419 = _417 + (_417 << 10u);
    uint _421 = _419 ^ (_419 >> 6u);
    uint _423 = _421 + (_421 << 3u);
    uint _425 = _423 ^ (_423 >> 11u);
    uint3 _438 = uint3(_312 + int3(0, 1, 0));
    uint _441 = _438.z;
    uint _443 = _441 + (_441 << 10u);
    uint _445 = _443 ^ (_443 >> 6u);
    uint _447 = _445 + (_445 << 3u);
    uint _449 = _447 ^ (_447 >> 11u);
    uint _455 = (((_438.y * 1664525u) + (_449 + (_449 << 15u))) + 1013904223u) * 1664525u;
    uint _457 = _455 ^ (_455 >> 11u);
    uint _460 = _457 ^ ((_457 << 7u) & 2636928640u);
    uint _463 = _460 ^ ((_460 << 15u) & 4022730752u);
    uint _469 = (((_438.x * 1664525u) + (_463 ^ (_463 >> 18u))) + 1013904223u) * 1664525u;
    uint _471 = _469 ^ (_469 >> 11u);
    uint _474 = _471 ^ ((_471 << 7u) & 2636928640u);
    uint _477 = _474 ^ ((_474 << 15u) & 4022730752u);
    uint _479 = _477 ^ (_477 >> 18u);
    uint _481 = _479 + (_479 << 10u);
    uint _483 = _481 ^ (_481 >> 6u);
    uint _485 = _483 + (_483 << 3u);
    uint _487 = _485 ^ (_485 >> 11u);
    uint3 _500 = uint3(_312 + int3(1, 1, 0));
    uint _503 = _500.z;
    uint _505 = _503 + (_503 << 10u);
    uint _507 = _505 ^ (_505 >> 6u);
    uint _509 = _507 + (_507 << 3u);
    uint _511 = _509 ^ (_509 >> 11u);
    uint _517 = (((_500.y * 1664525u) + (_511 + (_511 << 15u))) + 1013904223u) * 1664525u;
    uint _519 = _517 ^ (_517 >> 11u);
    uint _522 = _519 ^ ((_519 << 7u) & 2636928640u);
    uint _525 = _522 ^ ((_522 << 15u) & 4022730752u);
    uint _531 = (((_500.x * 1664525u) + (_525 ^ (_525 >> 18u))) + 1013904223u) * 1664525u;
    uint _533 = _531 ^ (_531 >> 11u);
    uint _536 = _533 ^ ((_533 << 7u) & 2636928640u);
    uint _539 = _536 ^ ((_536 << 15u) & 4022730752u);
    uint _541 = _539 ^ (_539 >> 18u);
    uint _543 = _541 + (_541 << 10u);
    uint _545 = _543 ^ (_543 >> 6u);
    uint _547 = _545 + (_545 << 3u);
    uint _549 = _547 ^ (_547 >> 11u);
    uint3 _562 = uint3(_312 + int3(0, 0, 1));
    uint _565 = _562.z;
    uint _567 = _565 + (_565 << 10u);
    uint _569 = _567 ^ (_567 >> 6u);
    uint _571 = _569 + (_569 << 3u);
    uint _573 = _571 ^ (_571 >> 11u);
    uint _579 = (((_562.y * 1664525u) + (_573 + (_573 << 15u))) + 1013904223u) * 1664525u;
    uint _581 = _579 ^ (_579 >> 11u);
    uint _584 = _581 ^ ((_581 << 7u) & 2636928640u);
    uint _587 = _584 ^ ((_584 << 15u) & 4022730752u);
    uint _593 = (((_562.x * 1664525u) + (_587 ^ (_587 >> 18u))) + 1013904223u) * 1664525u;
    uint _595 = _593 ^ (_593 >> 11u);
    uint _598 = _595 ^ ((_595 << 7u) & 2636928640u);
    uint _601 = _598 ^ ((_598 << 15u) & 4022730752u);
    uint _603 = _601 ^ (_601 >> 18u);
    uint _605 = _603 + (_603 << 10u);
    uint _607 = _605 ^ (_605 >> 6u);
    uint _609 = _607 + (_607 << 3u);
    uint _611 = _609 ^ (_609 >> 11u);
    uint3 _624 = uint3(_312 + int3(1, 0, 1));
    uint _627 = _624.z;
    uint _629 = _627 + (_627 << 10u);
    uint _631 = _629 ^ (_629 >> 6u);
    uint _633 = _631 + (_631 << 3u);
    uint _635 = _633 ^ (_633 >> 11u);
    uint _641 = (((_624.y * 1664525u) + (_635 + (_635 << 15u))) + 1013904223u) * 1664525u;
    uint _643 = _641 ^ (_641 >> 11u);
    uint _646 = _643 ^ ((_643 << 7u) & 2636928640u);
    uint _649 = _646 ^ ((_646 << 15u) & 4022730752u);
    uint _655 = (((_624.x * 1664525u) + (_649 ^ (_649 >> 18u))) + 1013904223u) * 1664525u;
    uint _657 = _655 ^ (_655 >> 11u);
    uint _660 = _657 ^ ((_657 << 7u) & 2636928640u);
    uint _663 = _660 ^ ((_660 << 15u) & 4022730752u);
    uint _665 = _663 ^ (_663 >> 18u);
    uint _667 = _665 + (_665 << 10u);
    uint _669 = _667 ^ (_667 >> 6u);
    uint _671 = _669 + (_669 << 3u);
    uint _673 = _671 ^ (_671 >> 11u);
    uint3 _686 = uint3(_312 + int3(0, 1, 1));
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
    uint3 _748 = uint3(_312 + int3(1, 1, 1));
    uint _751 = _748.z;
    uint _753 = _751 + (_751 << 10u);
    uint _755 = _753 ^ (_753 >> 6u);
    uint _757 = _755 + (_755 << 3u);
    uint _759 = _757 ^ (_757 >> 11u);
    uint _765 = (((_748.y * 1664525u) + (_759 + (_759 << 15u))) + 1013904223u) * 1664525u;
    uint _767 = _765 ^ (_765 >> 11u);
    uint _770 = _767 ^ ((_767 << 7u) & 2636928640u);
    uint _773 = _770 ^ ((_770 << 15u) & 4022730752u);
    uint _779 = (((_748.x * 1664525u) + (_773 ^ (_773 >> 18u))) + 1013904223u) * 1664525u;
    uint _781 = _779 ^ (_779 >> 11u);
    uint _784 = _781 ^ ((_781 << 7u) & 2636928640u);
    uint _787 = _784 ^ ((_784 << 15u) & 4022730752u);
    uint _789 = _787 ^ (_787 >> 18u);
    uint _791 = _789 + (_789 << 10u);
    uint _793 = _791 ^ (_791 >> 6u);
    uint _795 = _793 + (_793 << 3u);
    uint _797 = _795 ^ (_795 >> 11u);
    float3 _812 = (_313 * _313) * (3.0f.xxx - (_313 * 2.0f));
    float _813 = _812.x;
    float _814 = 1.0f - _813;
    float _816 = _812.y;
    float _817 = 1.0f - _816;
    float _819 = _812.z;
    float _820 = 1.0f - _819;
    float2 _837 = ((((((float2(asfloat((_355 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_363 + (_363 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _814) * _817) * _820) + (((float2(asfloat((_417 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_425 + (_425 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _813) * _817) * _820)) + (((float2(asfloat((_479 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_487 + (_487 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _814) * _816) * _820)) + (((float2(asfloat((_541 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_549 + (_549 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _813) * _816) * _820)) + (((float2(asfloat((_603 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_611 + (_611 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _814) * _817) * _819);
    float2 _852 = clamp(((((_837 + (((float2(asfloat((_665 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_673 + (_673 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _813) * _817) * _819)) + (((float2(asfloat((_727 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_735 + (_735 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _814) * _816) * _819)) + (((float2(asfloat((_789 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_797 + (_797 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _813) * _816) * _819)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _857 = _309 + (float3(_852.x, 0.0f, _852.y) * 0.0599999986588954925537109375f);
    float3x3 _865 = float3x3(_299[0].xyz, _299[1].xyz, _299[2].xyz);
    float3 _866 = mul(_865, float3(1.0f, 0.0f, 0.0f));
    float3 _867 = mul(_865, float3(0.0f, 1.0f, 0.0f));
    float3 _868 = mul(_865, float3(0.0f, 0.0f, 1.0f));
    float3 _871 = cross(_866, _867);
    float3 _874 = (1.0f / dot(_868, _871)).xxx;
    uint _899 = uint(_191 ^ 2340774);
    uint _901 = _899 + (_899 << 10u);
    uint _903 = _901 ^ (_901 >> 6u);
    uint _905 = _903 + (_903 << 3u);
    uint _907 = _905 ^ (_905 >> 11u);
    uint _915 = uint(_191 ^ 934565);
    uint _917 = _915 + (_915 << 10u);
    uint _919 = _917 ^ (_917 >> 6u);
    uint _921 = _919 + (_919 << 3u);
    uint _923 = _921 ^ (_921 >> 11u);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_857.x, _857.y, _857.z, _308.w));
    out_var_C0 = _857.xyz;
    out_var_C1 = normalize(mul(transpose(float3x3(cross(_867, _868) * _874, cross(_868, _866) * _874, _871 * _874)), in_var_Vertex_Normal));
    out_var_C3 = _191;
    out_var_C4 = in_var_is_bevel;
    out_var_C5 = uint((nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) & 2u) != 0u);
    out_var_C6 = (((in_var_Vertex_UV * 0.4000000059604644775390625f) + float2(asfloat(((_907 + (_907 << 15u)) & 8388607u) | 1065353216u) - 1.0f, asfloat(((_923 + (_923 << 15u)) & 8388607u) | 1065353216u) - 1.0f)) + (float(in_var_is_bevel) * 0.12000000476837158203125f).xx) * float2(1.0f, length(float3(_185[0].y, _185[1].y, _185[2].y)) * 0.4000000059604644775390625f);
    out_var_C7 = int(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56));
    out_var_C8 = int(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) & 4u);
    out_var_C9 = int(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) & 8u);
    out_var_C10 = int(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) & 16u);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_bevel = stage_input.in_var_is_bevel;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    stage_output.out_var_C9 = out_var_C9;
    stage_output.out_var_C10 = out_var_C10;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

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
