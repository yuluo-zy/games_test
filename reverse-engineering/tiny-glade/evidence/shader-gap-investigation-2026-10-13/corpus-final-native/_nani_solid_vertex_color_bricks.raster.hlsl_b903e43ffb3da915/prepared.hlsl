#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/nani_solid_vertex_color_bricks.raster.hlsl"




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

struct InstanceData
{
    Affine3Packed xform;
    uint wallspace_x_range_packed;
    uint seed;
    uint wall_id_is_wooden_packed;
    float mesh_size_x;
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
    uint current_view_index : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint highlight_mode : packoffset(c1.z);
    int highlight_bricks_to_delete : packoffset(c1.w);
    float3 brush_position : packoffset(c2);
    uint bool_render_shadows : packoffset(c2.w);
    float eraser_mult : packoffset(c3);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
ByteAddressBuffer wall_color_ids : register(t18, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
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
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
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

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void vert_main()
{
    InstanceData _169;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _169.xform.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    _169.wallspace_x_range_packed = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48);
    _169.seed = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    _169.wall_id_is_wooden_packed = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56);
    _169.mesh_size_x = nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60);
    float3x4 _198 = transpose(float4x3(float3(_169.xform.data[0], _169.xform.data[1], _169.xform.data[2]), float3(_169.xform.data[3], _169.xform.data[4], _169.xform.data[5]), float3(_169.xform.data[6], _169.xform.data[7], _169.xform.data[8]), float3(_169.xform.data[9], _169.xform.data[10], _169.xform.data[11])));
    float3 _202 = mul(_198, float4(in_var_Vertex_Position, 1.0f));
    float3 _203 = _202 * 0.4000000059604644775390625f;
    int3 _205 = int3(floor(_203));
    float3 _206 = frac(_203);
    uint3 _207 = uint3(_205);
    uint _210 = _207.z;
    uint _212 = _210 + (_210 << 10u);
    uint _214 = _212 ^ (_212 >> 6u);
    uint _216 = _214 + (_214 << 3u);
    uint _218 = _216 ^ (_216 >> 11u);
    uint _224 = (((_207.y * 1664525u) + (_218 + (_218 << 15u))) + 1013904223u) * 1664525u;
    uint _226 = _224 ^ (_224 >> 11u);
    uint _229 = _226 ^ ((_226 << 7u) & 2636928640u);
    uint _232 = _229 ^ ((_229 << 15u) & 4022730752u);
    uint _238 = (((_207.x * 1664525u) + (_232 ^ (_232 >> 18u))) + 1013904223u) * 1664525u;
    uint _240 = _238 ^ (_238 >> 11u);
    uint _243 = _240 ^ ((_240 << 7u) & 2636928640u);
    uint _246 = _243 ^ ((_243 << 15u) & 4022730752u);
    uint _248 = _246 ^ (_246 >> 18u);
    uint _250 = _248 + (_248 << 10u);
    uint _252 = _250 ^ (_250 >> 6u);
    uint _254 = _252 + (_252 << 3u);
    uint _256 = _254 ^ (_254 >> 11u);
    uint3 _269 = uint3(_205 + int3(1, 0, 0));
    uint _272 = _269.z;
    uint _274 = _272 + (_272 << 10u);
    uint _276 = _274 ^ (_274 >> 6u);
    uint _278 = _276 + (_276 << 3u);
    uint _280 = _278 ^ (_278 >> 11u);
    uint _286 = (((_269.y * 1664525u) + (_280 + (_280 << 15u))) + 1013904223u) * 1664525u;
    uint _288 = _286 ^ (_286 >> 11u);
    uint _291 = _288 ^ ((_288 << 7u) & 2636928640u);
    uint _294 = _291 ^ ((_291 << 15u) & 4022730752u);
    uint _300 = (((_269.x * 1664525u) + (_294 ^ (_294 >> 18u))) + 1013904223u) * 1664525u;
    uint _302 = _300 ^ (_300 >> 11u);
    uint _305 = _302 ^ ((_302 << 7u) & 2636928640u);
    uint _308 = _305 ^ ((_305 << 15u) & 4022730752u);
    uint _310 = _308 ^ (_308 >> 18u);
    uint _312 = _310 + (_310 << 10u);
    uint _314 = _312 ^ (_312 >> 6u);
    uint _316 = _314 + (_314 << 3u);
    uint _318 = _316 ^ (_316 >> 11u);
    uint3 _331 = uint3(_205 + int3(0, 1, 0));
    uint _334 = _331.z;
    uint _336 = _334 + (_334 << 10u);
    uint _338 = _336 ^ (_336 >> 6u);
    uint _340 = _338 + (_338 << 3u);
    uint _342 = _340 ^ (_340 >> 11u);
    uint _348 = (((_331.y * 1664525u) + (_342 + (_342 << 15u))) + 1013904223u) * 1664525u;
    uint _350 = _348 ^ (_348 >> 11u);
    uint _353 = _350 ^ ((_350 << 7u) & 2636928640u);
    uint _356 = _353 ^ ((_353 << 15u) & 4022730752u);
    uint _362 = (((_331.x * 1664525u) + (_356 ^ (_356 >> 18u))) + 1013904223u) * 1664525u;
    uint _364 = _362 ^ (_362 >> 11u);
    uint _367 = _364 ^ ((_364 << 7u) & 2636928640u);
    uint _370 = _367 ^ ((_367 << 15u) & 4022730752u);
    uint _372 = _370 ^ (_370 >> 18u);
    uint _374 = _372 + (_372 << 10u);
    uint _376 = _374 ^ (_374 >> 6u);
    uint _378 = _376 + (_376 << 3u);
    uint _380 = _378 ^ (_378 >> 11u);
    uint3 _393 = uint3(_205 + int3(1, 1, 0));
    uint _396 = _393.z;
    uint _398 = _396 + (_396 << 10u);
    uint _400 = _398 ^ (_398 >> 6u);
    uint _402 = _400 + (_400 << 3u);
    uint _404 = _402 ^ (_402 >> 11u);
    uint _410 = (((_393.y * 1664525u) + (_404 + (_404 << 15u))) + 1013904223u) * 1664525u;
    uint _412 = _410 ^ (_410 >> 11u);
    uint _415 = _412 ^ ((_412 << 7u) & 2636928640u);
    uint _418 = _415 ^ ((_415 << 15u) & 4022730752u);
    uint _424 = (((_393.x * 1664525u) + (_418 ^ (_418 >> 18u))) + 1013904223u) * 1664525u;
    uint _426 = _424 ^ (_424 >> 11u);
    uint _429 = _426 ^ ((_426 << 7u) & 2636928640u);
    uint _432 = _429 ^ ((_429 << 15u) & 4022730752u);
    uint _434 = _432 ^ (_432 >> 18u);
    uint _436 = _434 + (_434 << 10u);
    uint _438 = _436 ^ (_436 >> 6u);
    uint _440 = _438 + (_438 << 3u);
    uint _442 = _440 ^ (_440 >> 11u);
    uint3 _455 = uint3(_205 + int3(0, 0, 1));
    uint _458 = _455.z;
    uint _460 = _458 + (_458 << 10u);
    uint _462 = _460 ^ (_460 >> 6u);
    uint _464 = _462 + (_462 << 3u);
    uint _466 = _464 ^ (_464 >> 11u);
    uint _472 = (((_455.y * 1664525u) + (_466 + (_466 << 15u))) + 1013904223u) * 1664525u;
    uint _474 = _472 ^ (_472 >> 11u);
    uint _477 = _474 ^ ((_474 << 7u) & 2636928640u);
    uint _480 = _477 ^ ((_477 << 15u) & 4022730752u);
    uint _486 = (((_455.x * 1664525u) + (_480 ^ (_480 >> 18u))) + 1013904223u) * 1664525u;
    uint _488 = _486 ^ (_486 >> 11u);
    uint _491 = _488 ^ ((_488 << 7u) & 2636928640u);
    uint _494 = _491 ^ ((_491 << 15u) & 4022730752u);
    uint _496 = _494 ^ (_494 >> 18u);
    uint _498 = _496 + (_496 << 10u);
    uint _500 = _498 ^ (_498 >> 6u);
    uint _502 = _500 + (_500 << 3u);
    uint _504 = _502 ^ (_502 >> 11u);
    uint3 _517 = uint3(_205 + int3(1, 0, 1));
    uint _520 = _517.z;
    uint _522 = _520 + (_520 << 10u);
    uint _524 = _522 ^ (_522 >> 6u);
    uint _526 = _524 + (_524 << 3u);
    uint _528 = _526 ^ (_526 >> 11u);
    uint _534 = (((_517.y * 1664525u) + (_528 + (_528 << 15u))) + 1013904223u) * 1664525u;
    uint _536 = _534 ^ (_534 >> 11u);
    uint _539 = _536 ^ ((_536 << 7u) & 2636928640u);
    uint _542 = _539 ^ ((_539 << 15u) & 4022730752u);
    uint _548 = (((_517.x * 1664525u) + (_542 ^ (_542 >> 18u))) + 1013904223u) * 1664525u;
    uint _550 = _548 ^ (_548 >> 11u);
    uint _553 = _550 ^ ((_550 << 7u) & 2636928640u);
    uint _556 = _553 ^ ((_553 << 15u) & 4022730752u);
    uint _558 = _556 ^ (_556 >> 18u);
    uint _560 = _558 + (_558 << 10u);
    uint _562 = _560 ^ (_560 >> 6u);
    uint _564 = _562 + (_562 << 3u);
    uint _566 = _564 ^ (_564 >> 11u);
    uint3 _579 = uint3(_205 + int3(0, 1, 1));
    uint _582 = _579.z;
    uint _584 = _582 + (_582 << 10u);
    uint _586 = _584 ^ (_584 >> 6u);
    uint _588 = _586 + (_586 << 3u);
    uint _590 = _588 ^ (_588 >> 11u);
    uint _596 = (((_579.y * 1664525u) + (_590 + (_590 << 15u))) + 1013904223u) * 1664525u;
    uint _598 = _596 ^ (_596 >> 11u);
    uint _601 = _598 ^ ((_598 << 7u) & 2636928640u);
    uint _604 = _601 ^ ((_601 << 15u) & 4022730752u);
    uint _610 = (((_579.x * 1664525u) + (_604 ^ (_604 >> 18u))) + 1013904223u) * 1664525u;
    uint _612 = _610 ^ (_610 >> 11u);
    uint _615 = _612 ^ ((_612 << 7u) & 2636928640u);
    uint _618 = _615 ^ ((_615 << 15u) & 4022730752u);
    uint _620 = _618 ^ (_618 >> 18u);
    uint _622 = _620 + (_620 << 10u);
    uint _624 = _622 ^ (_622 >> 6u);
    uint _626 = _624 + (_624 << 3u);
    uint _628 = _626 ^ (_626 >> 11u);
    uint3 _641 = uint3(_205 + int3(1, 1, 1));
    uint _644 = _641.z;
    uint _646 = _644 + (_644 << 10u);
    uint _648 = _646 ^ (_646 >> 6u);
    uint _650 = _648 + (_648 << 3u);
    uint _652 = _650 ^ (_650 >> 11u);
    uint _658 = (((_641.y * 1664525u) + (_652 + (_652 << 15u))) + 1013904223u) * 1664525u;
    uint _660 = _658 ^ (_658 >> 11u);
    uint _663 = _660 ^ ((_660 << 7u) & 2636928640u);
    uint _666 = _663 ^ ((_663 << 15u) & 4022730752u);
    uint _672 = (((_641.x * 1664525u) + (_666 ^ (_666 >> 18u))) + 1013904223u) * 1664525u;
    uint _674 = _672 ^ (_672 >> 11u);
    uint _677 = _674 ^ ((_674 << 7u) & 2636928640u);
    uint _680 = _677 ^ ((_677 << 15u) & 4022730752u);
    uint _682 = _680 ^ (_680 >> 18u);
    uint _684 = _682 + (_682 << 10u);
    uint _686 = _684 ^ (_684 >> 6u);
    uint _688 = _686 + (_686 << 3u);
    uint _690 = _688 ^ (_688 >> 11u);
    float3 _705 = (_206 * _206) * (3.0f.xxx - (_206 * 2.0f));
    float _706 = _705.x;
    float _707 = 1.0f - _706;
    float _709 = _705.y;
    float _710 = 1.0f - _709;
    float _712 = _705.z;
    float _713 = 1.0f - _712;
    float2 _730 = ((((((float2(asfloat((_248 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_256 + (_256 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _707) * _710) * _713) + (((float2(asfloat((_310 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_318 + (_318 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _706) * _710) * _713)) + (((float2(asfloat((_372 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_380 + (_380 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _707) * _709) * _713)) + (((float2(asfloat((_434 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_442 + (_442 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _706) * _709) * _713)) + (((float2(asfloat((_496 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_504 + (_504 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _707) * _710) * _712);
    float2 _745 = clamp(((((_730 + (((float2(asfloat((_558 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_566 + (_566 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _706) * _710) * _712)) + (((float2(asfloat((_620 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_628 + (_628 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _707) * _709) * _712)) + (((float2(asfloat((_682 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_690 + (_690 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _706) * _709) * _712)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _750 = _202 + (float3(_745.x, 0.0f, _745.y) * 0.0599999986588954925537109375f);
    bool _752 = (_169.wall_id_is_wooden_packed & 2147483648u) != 0u;
    int _753 = _752 ? 1536 : 512;
    uint _777 = _169.wall_id_is_wooden_packed & 2147483647u;
    uint _781 = _777 & 65535u;
    uint _784 = _781 ^ 7993234u;
    uint _786 = _784 + (_784 << 10u);
    uint _788 = _786 ^ (_786 >> 6u);
    uint _790 = _788 + (_788 << 3u);
    uint _792 = _790 ^ (_790 >> 11u);
    uint _799 = _781 ^ 3450234u;
    uint _801 = _799 + (_799 << 10u);
    uint _803 = _801 ^ (_801 >> 6u);
    uint _805 = _803 + (_803 << 3u);
    uint _807 = _805 ^ (_805 >> 11u);
    float2 _821 = (((float2(lerp(spvUnpackHalf2x16(_169.wallspace_x_range_packed & 65535u).x, spvUnpackHalf2x16((_169.wallspace_x_range_packed >> 16u) & 65535u).x, ((in_var_Vertex_Position.x / (_169.mesh_size_x * 0.5f)) * 0.5f) + 0.5f), _750.y) * 0.100000001490116119384765625f) * 0.699999988079071044921875f) + (float(_781) * 0.20000000298023223876953125f).xx) + float2(asfloat(((_792 + (_792 << 15u)) & 8388607u) | 1065353216u) - 1.0f, asfloat(((_807 + (_807 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    uint _822 = _169.seed ^ 292478890u;
    uint _824 = _822 + (_822 << 10u);
    uint _826 = _824 ^ (_824 >> 6u);
    uint _828 = _826 + (_826 << 3u);
    uint _830 = _828 ^ (_828 >> 11u);
    float _836 = asfloat(((_830 + (_830 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 _845;
    if (_836 > 0.89999997615814208984375f)
    {
        _845 = _821 * 3.0f;
    }
    else
    {
        _845 = _821 + (0.61000001430511474609375f * _836).xx;
    }
    float2 _851;
    if (int((_753 & 512) != 0) == 1)
    {
        _851 = _845 * 2.2000000476837158203125f;
    }
    else
    {
        _851 = _845;
    }
    bool2 _852 = _752.xx;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_750, 1.0f));
    out_var_C0 = _169.seed;
    out_var_C1 = mul(_198, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _750;
    out_var_C5 = 0.0f.xxx;
    out_var_C7 = 0.0f;
    out_var_C10 = float2(_852.x ? in_var_Vertex_UV.x : _851.x, _852.y ? in_var_Vertex_UV.y : _851.y);
    out_var_C11 = int(wall_color_ids.Load<uint>(_777 * 4 + 0));
    out_var_C12 = 100000.0f.xx;
    out_var_C13 = _753;
    out_var_C14 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
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

struct InstanceData
{
    Affine3Packed xform;
    uint wallspace_x_range_packed;
    uint seed;
    uint wall_id_is_wooden_packed;
    float mesh_size_x;
};

static uint _198;
static float3 _210;

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
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint highlight_mode : packoffset(c1.z);
    int highlight_bricks_to_delete : packoffset(c1.w);
    float3 brush_position : packoffset(c2);
    uint bool_render_shadows : packoffset(c2.w);
    float eraser_mult : packoffset(c3);
};

ByteAddressBuffer nani_instance_data : register(t15, space0);
ByteAddressBuffer halftimber_color_ids : register(t19, space0);
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
Texture2DArray<float4> halftimber_colors : register(t20, space0);
Texture2DArray<float4> brick_colors : register(t22, space0);
Texture2DArray<float4> roof_colors : register(t24, space0);

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
static uint in_var_C14;
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
    bool _228 = (in_var_C13 & 512) != 0;
    bool _230 = (in_var_C13 & 8) != 0;
    bool _234 = (in_var_C13 & 16) != 0;
    bool _238 = (in_var_C13 & 2048) != 0;
    bool _240 = (in_var_C13 & 256) != 0;
    bool _245;
    if (!_230)
    {
        _245 = in_var_C7 > (-2.5f);
    }
    else
    {
        _245 = false;
    }
    bool _250;
    if (_245)
    {
        _250 = in_var_C2.y < in_var_C7;
    }
    else
    {
        _250 = false;
    }
    if (_250)
    {
        discard;
    }
    else
    {
        bool _258;
        if (_230)
        {
            _258 = in_var_C2.y > in_var_C7;
        }
        else
        {
            _258 = false;
        }
        if (_258)
        {
            discard;
        }
    }
    uint _264 = uint(in_var_C11);
    float3 _481;
    if ((in_var_C13 & 2) != 0)
    {
        _481 = roof_colors.Sample(sampler_llr, float3((float2(in_var_C0.xx) * 0.20000000298023223876953125f) + (in_var_C2.xz * 0.20000000298023223876953125f), float(_264))).xyz * 0.699999988079071044921875f;
    }
    else
    {
        float2 _276;
        if (_240)
        {
            _276 = in_var_C10 + 0.20999999344348907470703125f.xx;
        }
        else
        {
            _276 = in_var_C10;
        }
        bool _282;
        if (!(_264 == 1u))
        {
            _282 = _264 == 0u;
        }
        else
        {
            _282 = true;
        }
        float3 _289;
        if (_282)
        {
            _289 = pow(color_library_wall_bricks.xyz, 1.2999999523162841796875f.xxx);
        }
        else
        {
            _289 = pow(color_library_wall_moss.xyz, 1.0f.xxx) * 0.800000011920928955078125f;
        }
        int _290 = int(_264);
        bool _296;
        if (!(_290 == 0))
        {
            _296 = _290 == 4;
        }
        else
        {
            _296 = true;
        }
        bool _301;
        if (!_296)
        {
            _301 = _290 == 10;
        }
        else
        {
            _301 = true;
        }
        bool _306;
        if (!_301)
        {
            _306 = _290 == 12;
        }
        else
        {
            _306 = true;
        }
        float3 _397;
        if (_306)
        {
            _397 = brick_colors.Sample(sampler_llr, float3(_276 + 0.85000002384185791015625f.xx, 0.0f)).xyz;
        }
        else
        {
            bool _315;
            if (!(_290 == 1))
            {
                _315 = _290 == 5;
            }
            else
            {
                _315 = true;
            }
            float3 _387;
            if (_315)
            {
                _387 = pow(brick_colors.Sample(sampler_llr, float3(_276 + 0.449999988079071044921875f.xx, 1.0f)).xyz, 1.10000002384185791015625f.xxx);
            }
            else
            {
                bool _324;
                if (!(_290 == 2))
                {
                    _324 = _290 == 7;
                }
                else
                {
                    _324 = true;
                }
                float3 _376;
                if (_324)
                {
                    _376 = brick_colors.Sample(sampler_llr, float3(_276, 2.0f)).xyz;
                }
                else
                {
                    bool _333;
                    if (!(_290 == 3))
                    {
                        _333 = _290 == 6;
                    }
                    else
                    {
                        _333 = true;
                    }
                    bool _338;
                    if (!_333)
                    {
                        _338 = _290 == 8;
                    }
                    else
                    {
                        _338 = true;
                    }
                    bool _343;
                    if (!_338)
                    {
                        _343 = _290 == 9;
                    }
                    else
                    {
                        _343 = true;
                    }
                    float3 _367;
                    if (_343)
                    {
                        _367 = brick_colors.Sample(sampler_llr, float3(_276, 3.0f)).xyz;
                    }
                    else
                    {
                        float3 _358;
                        if (_290 == 11)
                        {
                            _358 = brick_colors.Sample(sampler_llr, float3(_276, 4.0f)).xyz;
                        }
                        else
                        {
                            _358 = 0.0f.xxx;
                        }
                        _367 = _358;
                    }
                    _376 = _367;
                }
                _387 = _376;
            }
            _397 = _387;
        }
        bool _399 = !(_264 == 7u);
        bool _403;
        if (_399)
        {
            _403 = _264 == 2u;
        }
        else
        {
            _403 = true;
        }
        bool _408;
        if (!_403)
        {
            _408 = _264 == 11u;
        }
        else
        {
            _408 = true;
        }
        bool3 _409 = _408.xxx;
        float3 _432;
        if (_228)
        {
            uint _413 = in_var_C0 ^ 8902390u;
            uint _415 = _413 + (_413 << 10u);
            uint _417 = _415 ^ (_415 >> 6u);
            uint _419 = _417 + (_417 << 3u);
            uint _421 = _419 ^ (_419 >> 11u);
            _432 = lerp(_397, float3(_409.x ? _397.x : _289.x, _409.y ? _397.y : _289.y, _409.z ? _397.z : _289.z), lerp(0.0f, 0.5f, smoothstep(0.20000000298023223876953125f, 0.800000011920928955078125f, asfloat(((_421 + (_421 << 15u)) & 8388607u) | 1065353216u) - 1.0f)).xxx);
        }
        else
        {
            _432 = _397;
        }
        bool _435 = _228 ? true : false;
        float3 _464;
        if (_240)
        {
            bool _441;
            if (_399)
            {
                _441 = _264 == 2u;
            }
            else
            {
                _441 = true;
            }
            uint _443 = in_var_C0 ^ 239085u;
            uint _445 = _443 + (_443 << 10u);
            uint _447 = _445 ^ (_445 >> 6u);
            uint _449 = _447 + (_447 << 3u);
            uint _451 = _449 ^ (_449 >> 11u);
            _464 = lerp(_432, color_library_wall_1.xyz, lerp(0.0f, _441 ? 0.20000000298023223876953125f : 0.4000000059604644775390625f, smoothstep(0.0f, 0.800000011920928955078125f, asfloat(((_451 + (_451 << 15u)) & (_435 ? (_228 ? 8388607u : _198) : 8388607u)) | (_435 ? (_228 ? 1065353216u : _198) : 1065353216u)) - 1.0f)).xxx);
        }
        else
        {
            _464 = _432;
        }
        _481 = _464;
    }
    float3 _482 = normalize(in_var_C1);
    bool _521;
    float3 _522;
    if (!((in_var_C13 & 64) != 0))
    {
        bool _499;
        float3 _500;
        if (highlight_mode == 1u)
        {
            _499 = true;
            _500 = lerp(_481, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
        }
        else
        {
            bool _492 = highlight_mode == 2u;
            float3 _496;
            if (_492)
            {
                _496 = lerp(_481, 0.5f.xxx, 0.100000001490116119384765625f.xxx);
            }
            else
            {
                _496 = _481;
            }
            _499 = _492 ? true : false;
            _500 = _496;
        }
        bool _519;
        float3 _520;
        if (highlight_bricks_to_delete == 1)
        {
            bool _513 = distance(brush_position.xz, in_var_C12) < (1.88900005817413330078125f * eraser_mult);
            float3 _517;
            if (_513)
            {
                _517 = lerp(_481, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.800000011920928955078125f.xxx);
            }
            else
            {
                _517 = _500;
            }
            _519 = _513 ? true : _499;
            _520 = _517;
        }
        else
        {
            _519 = _499;
            _520 = _500;
        }
        _521 = _519;
        _522 = _520;
    }
    else
    {
        _521 = false;
        _522 = _481;
    }
    float3 _526;
    if (_238)
    {
        _526 = lerp(_481, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
    }
    else
    {
        _526 = _522;
    }
    float3 _548;
    if (int((in_var_C13 & 1024) != 0) == 1)
    {
        _548 = halftimber_colors.Sample(sampler_llr, float3(in_var_C10 * 1.0f, float(int(halftimber_color_ids.Load<uint>((nani_instance_data.Load<uint>(in_var_C14 * 64 + 56) & 2147483647u) * 4 + 0))))).xyz;
    }
    else
    {
        _548 = _526;
    }
    float3 _649;
    if (frame_constants_glade_theme_id == 4u)
    {
        float2 _559 = in_var_C2.xz * 0.4000000059604644775390625f;
        float2 _563 = floor(_559 + dot(_559, 0.3660254180431365966796875f.xx).xx);
        float2 _567 = (_559 - _563) + dot(_563, 0.211324870586395263671875f.xx).xx;
        float _568 = _567.x;
        float _569 = _567.y;
        bool2 _571 = (_568 > _569).xx;
        float2 _572 = float2(_571.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, _571.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
        float4 _574 = _567.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
        float2 _576 = _574.xy - _572;
        float4 _577 = float4(_576.x, _576.y, _574.z, _574.w);
        float2 _578 = fmod(_563, 289.0f.xx);
        float3 _583 = _578.y.xxx + float3(0.0f, _572.y, 1.0f);
        float3 _593 = (fmod(((_583 * 34.0f) + 1.0f.xxx) * _583, 289.0f.xxx) + _578.x.xxx) + float3(0.0f, _572.x, 1.0f);
        float2 _599 = _576.xy;
        float2 _601 = _574.zw;
        float3 _605 = max(0.5f.xxx - float3(dot(_567, _567), dot(_599, _599), dot(_601, _601)), 0.0f.xxx);
        float3 _606 = _605 * _605;
        float3 _610 = frac(fmod(((_593 * 34.0f) + 1.0f.xxx) * _593, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
        float3 _611 = _610 - 1.0f.xxx;
        float3 _613 = abs(_611) - 0.5f.xxx;
        float3 _616 = _611 - floor(_610 + (-0.5f).xxx);
        float3 _628;
        _628.x = (_616.x * _568) + (_613.x * _569);
        float2 _635 = (_616.yz * _577.xz) + (_613.yz * _577.yw);
        _649 = lerp(_548, 0.699999988079071044921875f.xxx, (smoothstep(0.0f, 1.0f, smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C2.y) * clamp(_482.y, 0.0f, 1.0f)) * clamp(clamp((130.0f * dot((_606 * _606) * (1.792842864990234375f.xxx - (((_616 * _616) + (_613 * _613)) * 0.8537347316741943359375f)), float3(_628.x, _635.x, _635.y))) + 0.75f, 0.0f, 1.0f), _234 ? 0.0f : 0.4000000059604644775390625f, _234 ? 0.100000001490116119384765625f : 1.0f)).xxx);
    }
    else
    {
        _649 = _548;
    }
    float3 _651 = (_482 * 0.5f) + 0.5f.xxx;
    float3 _673 = sqrt(clamp(_649, 0.0f.xxx, 1.0f.xxx));
    uint _692 = (uint((clamp(_673.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_673.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_673.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _693 = uint2(((uint((clamp(_651.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_651.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_651.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, _692);
    _693.y = _692 | (((((7u | (uint(_238 ? true : _521) << 3u)) | 0u) | 0u) | 0u) << 24u);
    out_var_SV_TARGET0 = _693;
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

struct InstanceData
{
    Affine3Packed xform;
    uint wallspace_x_range_packed;
    uint seed;
    uint wall_id_is_wooden_packed;
    float mesh_size_x;
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
    uint current_view_index : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint highlight_mode : packoffset(c1.z);
    int highlight_bricks_to_delete : packoffset(c1.w);
    float3 brush_position : packoffset(c2);
    uint bool_render_shadows : packoffset(c2.w);
    float eraser_mult : packoffset(c3);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
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
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
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
    Affine3Packed _159;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _159.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _182 = transpose(float4x3(float3(_159.data[0], _159.data[1], _159.data[2]), float3(_159.data[3], _159.data[4], _159.data[5]), float3(_159.data[6], _159.data[7], _159.data[8]), float3(_159.data[9], _159.data[10], _159.data[11])));
    float3 _187 = mul(_182, float4(in_var_Vertex_Position, 1.0f));
    float3 _188 = _187 * 0.4000000059604644775390625f;
    int3 _190 = int3(floor(_188));
    float3 _191 = frac(_188);
    uint3 _192 = uint3(_190);
    uint _195 = _192.z;
    uint _197 = _195 + (_195 << 10u);
    uint _199 = _197 ^ (_197 >> 6u);
    uint _201 = _199 + (_199 << 3u);
    uint _203 = _201 ^ (_201 >> 11u);
    uint _209 = (((_192.y * 1664525u) + (_203 + (_203 << 15u))) + 1013904223u) * 1664525u;
    uint _211 = _209 ^ (_209 >> 11u);
    uint _214 = _211 ^ ((_211 << 7u) & 2636928640u);
    uint _217 = _214 ^ ((_214 << 15u) & 4022730752u);
    uint _223 = (((_192.x * 1664525u) + (_217 ^ (_217 >> 18u))) + 1013904223u) * 1664525u;
    uint _225 = _223 ^ (_223 >> 11u);
    uint _228 = _225 ^ ((_225 << 7u) & 2636928640u);
    uint _231 = _228 ^ ((_228 << 15u) & 4022730752u);
    uint _233 = _231 ^ (_231 >> 18u);
    uint _235 = _233 + (_233 << 10u);
    uint _237 = _235 ^ (_235 >> 6u);
    uint _239 = _237 + (_237 << 3u);
    uint _241 = _239 ^ (_239 >> 11u);
    uint3 _254 = uint3(_190 + int3(1, 0, 0));
    uint _257 = _254.z;
    uint _259 = _257 + (_257 << 10u);
    uint _261 = _259 ^ (_259 >> 6u);
    uint _263 = _261 + (_261 << 3u);
    uint _265 = _263 ^ (_263 >> 11u);
    uint _271 = (((_254.y * 1664525u) + (_265 + (_265 << 15u))) + 1013904223u) * 1664525u;
    uint _273 = _271 ^ (_271 >> 11u);
    uint _276 = _273 ^ ((_273 << 7u) & 2636928640u);
    uint _279 = _276 ^ ((_276 << 15u) & 4022730752u);
    uint _285 = (((_254.x * 1664525u) + (_279 ^ (_279 >> 18u))) + 1013904223u) * 1664525u;
    uint _287 = _285 ^ (_285 >> 11u);
    uint _290 = _287 ^ ((_287 << 7u) & 2636928640u);
    uint _293 = _290 ^ ((_290 << 15u) & 4022730752u);
    uint _295 = _293 ^ (_293 >> 18u);
    uint _297 = _295 + (_295 << 10u);
    uint _299 = _297 ^ (_297 >> 6u);
    uint _301 = _299 + (_299 << 3u);
    uint _303 = _301 ^ (_301 >> 11u);
    uint3 _316 = uint3(_190 + int3(0, 1, 0));
    uint _319 = _316.z;
    uint _321 = _319 + (_319 << 10u);
    uint _323 = _321 ^ (_321 >> 6u);
    uint _325 = _323 + (_323 << 3u);
    uint _327 = _325 ^ (_325 >> 11u);
    uint _333 = (((_316.y * 1664525u) + (_327 + (_327 << 15u))) + 1013904223u) * 1664525u;
    uint _335 = _333 ^ (_333 >> 11u);
    uint _338 = _335 ^ ((_335 << 7u) & 2636928640u);
    uint _341 = _338 ^ ((_338 << 15u) & 4022730752u);
    uint _347 = (((_316.x * 1664525u) + (_341 ^ (_341 >> 18u))) + 1013904223u) * 1664525u;
    uint _349 = _347 ^ (_347 >> 11u);
    uint _352 = _349 ^ ((_349 << 7u) & 2636928640u);
    uint _355 = _352 ^ ((_352 << 15u) & 4022730752u);
    uint _357 = _355 ^ (_355 >> 18u);
    uint _359 = _357 + (_357 << 10u);
    uint _361 = _359 ^ (_359 >> 6u);
    uint _363 = _361 + (_361 << 3u);
    uint _365 = _363 ^ (_363 >> 11u);
    uint3 _378 = uint3(_190 + int3(1, 1, 0));
    uint _381 = _378.z;
    uint _383 = _381 + (_381 << 10u);
    uint _385 = _383 ^ (_383 >> 6u);
    uint _387 = _385 + (_385 << 3u);
    uint _389 = _387 ^ (_387 >> 11u);
    uint _395 = (((_378.y * 1664525u) + (_389 + (_389 << 15u))) + 1013904223u) * 1664525u;
    uint _397 = _395 ^ (_395 >> 11u);
    uint _400 = _397 ^ ((_397 << 7u) & 2636928640u);
    uint _403 = _400 ^ ((_400 << 15u) & 4022730752u);
    uint _409 = (((_378.x * 1664525u) + (_403 ^ (_403 >> 18u))) + 1013904223u) * 1664525u;
    uint _411 = _409 ^ (_409 >> 11u);
    uint _414 = _411 ^ ((_411 << 7u) & 2636928640u);
    uint _417 = _414 ^ ((_414 << 15u) & 4022730752u);
    uint _419 = _417 ^ (_417 >> 18u);
    uint _421 = _419 + (_419 << 10u);
    uint _423 = _421 ^ (_421 >> 6u);
    uint _425 = _423 + (_423 << 3u);
    uint _427 = _425 ^ (_425 >> 11u);
    uint3 _440 = uint3(_190 + int3(0, 0, 1));
    uint _443 = _440.z;
    uint _445 = _443 + (_443 << 10u);
    uint _447 = _445 ^ (_445 >> 6u);
    uint _449 = _447 + (_447 << 3u);
    uint _451 = _449 ^ (_449 >> 11u);
    uint _457 = (((_440.y * 1664525u) + (_451 + (_451 << 15u))) + 1013904223u) * 1664525u;
    uint _459 = _457 ^ (_457 >> 11u);
    uint _462 = _459 ^ ((_459 << 7u) & 2636928640u);
    uint _465 = _462 ^ ((_462 << 15u) & 4022730752u);
    uint _471 = (((_440.x * 1664525u) + (_465 ^ (_465 >> 18u))) + 1013904223u) * 1664525u;
    uint _473 = _471 ^ (_471 >> 11u);
    uint _476 = _473 ^ ((_473 << 7u) & 2636928640u);
    uint _479 = _476 ^ ((_476 << 15u) & 4022730752u);
    uint _481 = _479 ^ (_479 >> 18u);
    uint _483 = _481 + (_481 << 10u);
    uint _485 = _483 ^ (_483 >> 6u);
    uint _487 = _485 + (_485 << 3u);
    uint _489 = _487 ^ (_487 >> 11u);
    uint3 _502 = uint3(_190 + int3(1, 0, 1));
    uint _505 = _502.z;
    uint _507 = _505 + (_505 << 10u);
    uint _509 = _507 ^ (_507 >> 6u);
    uint _511 = _509 + (_509 << 3u);
    uint _513 = _511 ^ (_511 >> 11u);
    uint _519 = (((_502.y * 1664525u) + (_513 + (_513 << 15u))) + 1013904223u) * 1664525u;
    uint _521 = _519 ^ (_519 >> 11u);
    uint _524 = _521 ^ ((_521 << 7u) & 2636928640u);
    uint _527 = _524 ^ ((_524 << 15u) & 4022730752u);
    uint _533 = (((_502.x * 1664525u) + (_527 ^ (_527 >> 18u))) + 1013904223u) * 1664525u;
    uint _535 = _533 ^ (_533 >> 11u);
    uint _538 = _535 ^ ((_535 << 7u) & 2636928640u);
    uint _541 = _538 ^ ((_538 << 15u) & 4022730752u);
    uint _543 = _541 ^ (_541 >> 18u);
    uint _545 = _543 + (_543 << 10u);
    uint _547 = _545 ^ (_545 >> 6u);
    uint _549 = _547 + (_547 << 3u);
    uint _551 = _549 ^ (_549 >> 11u);
    uint3 _564 = uint3(_190 + int3(0, 1, 1));
    uint _567 = _564.z;
    uint _569 = _567 + (_567 << 10u);
    uint _571 = _569 ^ (_569 >> 6u);
    uint _573 = _571 + (_571 << 3u);
    uint _575 = _573 ^ (_573 >> 11u);
    uint _581 = (((_564.y * 1664525u) + (_575 + (_575 << 15u))) + 1013904223u) * 1664525u;
    uint _583 = _581 ^ (_581 >> 11u);
    uint _586 = _583 ^ ((_583 << 7u) & 2636928640u);
    uint _589 = _586 ^ ((_586 << 15u) & 4022730752u);
    uint _595 = (((_564.x * 1664525u) + (_589 ^ (_589 >> 18u))) + 1013904223u) * 1664525u;
    uint _597 = _595 ^ (_595 >> 11u);
    uint _600 = _597 ^ ((_597 << 7u) & 2636928640u);
    uint _603 = _600 ^ ((_600 << 15u) & 4022730752u);
    uint _605 = _603 ^ (_603 >> 18u);
    uint _607 = _605 + (_605 << 10u);
    uint _609 = _607 ^ (_607 >> 6u);
    uint _611 = _609 + (_609 << 3u);
    uint _613 = _611 ^ (_611 >> 11u);
    uint3 _626 = uint3(_190 + int3(1, 1, 1));
    uint _629 = _626.z;
    uint _631 = _629 + (_629 << 10u);
    uint _633 = _631 ^ (_631 >> 6u);
    uint _635 = _633 + (_633 << 3u);
    uint _637 = _635 ^ (_635 >> 11u);
    uint _643 = (((_626.y * 1664525u) + (_637 + (_637 << 15u))) + 1013904223u) * 1664525u;
    uint _645 = _643 ^ (_643 >> 11u);
    uint _648 = _645 ^ ((_645 << 7u) & 2636928640u);
    uint _651 = _648 ^ ((_648 << 15u) & 4022730752u);
    uint _657 = (((_626.x * 1664525u) + (_651 ^ (_651 >> 18u))) + 1013904223u) * 1664525u;
    uint _659 = _657 ^ (_657 >> 11u);
    uint _662 = _659 ^ ((_659 << 7u) & 2636928640u);
    uint _665 = _662 ^ ((_662 << 15u) & 4022730752u);
    uint _667 = _665 ^ (_665 >> 18u);
    uint _669 = _667 + (_667 << 10u);
    uint _671 = _669 ^ (_669 >> 6u);
    uint _673 = _671 + (_671 << 3u);
    uint _675 = _673 ^ (_673 >> 11u);
    float3 _690 = (_191 * _191) * (3.0f.xxx - (_191 * 2.0f));
    float _691 = _690.x;
    float _692 = 1.0f - _691;
    float _694 = _690.y;
    float _695 = 1.0f - _694;
    float _697 = _690.z;
    float _698 = 1.0f - _697;
    float2 _715 = ((((((float2(asfloat((_233 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_241 + (_241 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _692) * _695) * _698) + (((float2(asfloat((_295 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_303 + (_303 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _691) * _695) * _698)) + (((float2(asfloat((_357 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_365 + (_365 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _692) * _694) * _698)) + (((float2(asfloat((_419 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_427 + (_427 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _691) * _694) * _698)) + (((float2(asfloat((_481 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_489 + (_489 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _692) * _695) * _697);
    float2 _730 = clamp(((((_715 + (((float2(asfloat((_543 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_551 + (_551 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _691) * _695) * _697)) + (((float2(asfloat((_605 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_613 + (_613 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _692) * _694) * _697)) + (((float2(asfloat((_667 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_675 + (_675 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _691) * _694) * _697)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _735 = _187 + (float3(_730.x, 0.0f, _730.y) * 0.0599999986588954925537109375f);
    bool _737 = (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56) & 2147483648u) != 0u;
    bool2 _753 = _737.xx;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_735, 1.0f));
    out_var_C0 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    out_var_C1 = mul(_182, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _735;
    out_var_C5 = 0.0f.xxx;
    out_var_C7 = 0.0f;
    out_var_C10 = float2(_753.x ? in_var_Vertex_UV.x : 0.0f.xx.x, _753.y ? in_var_Vertex_UV.y : 0.0f.xx.y);
    out_var_C11 = 0;
    out_var_C12 = 100000.0f.xx;
    out_var_C13 = _737 ? 1536 : 512;
    out_var_C14 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
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
static float4 gl_FragCoord;
static float gl_FragDepth;
static float3 in_var_C2;
static float in_var_C7;
static int in_var_C13;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(7)]] float in_var_C7 : TEXCOORD7;
    [[vk::location(13)]] nointerpolation int in_var_C13 : TEXCOORD13;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    float gl_FragDepth : SV_DepthLessEqual;
};

void frag_main()
{
    bool _70 = (in_var_C13 & 8) != 0;
    bool _75;
    if (!_70)
    {
        _75 = in_var_C7 > (-2.5f);
    }
    else
    {
        _75 = false;
    }
    bool _80;
    if (_75)
    {
        _80 = in_var_C2.y < in_var_C7;
    }
    else
    {
        _80 = false;
    }
    if (_80)
    {
        discard;
    }
    else
    {
        bool _88;
        if (_70)
        {
            _88 = in_var_C2.y > in_var_C7;
        }
        else
        {
            _88 = false;
        }
        if (_88)
        {
            discard;
        }
    }
    gl_FragDepth = gl_FragCoord.z;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C13 = stage_input.in_var_C13;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_FragDepth = gl_FragDepth;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif
