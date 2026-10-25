#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/nani_solid_vertex_color_window_glass.raster.hlsl"




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
    uint color_id;
    float light_intensity;
    uint has_albedo_texture;
    uint albedo_texture;
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
    uint2 curtains_texture_dims : packoffset(c0.y);
    uint2 indoors_texture_dims : packoffset(c1);
    int bool_render_shadows : packoffset(c1.z);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_glass;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static uint out_var_C4;
static uint out_var_C5;
static float3 out_var_C6;
static uint out_var_C7;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_glass: is_glass;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation uint out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation float3 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation uint out_var_C7 : TEXCOORD7;
    float4 gl_Position : SV_Position;
};

uint spvBitfieldUExtract(uint Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

uint2 spvBitfieldUExtract(uint2 Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

uint3 spvBitfieldUExtract(uint3 Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

uint4 spvBitfieldUExtract(uint4 Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

int spvBitfieldSExtract(int Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

int2 spvBitfieldSExtract(int2 Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int2 Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

int3 spvBitfieldSExtract(int3 Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int3 Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

int4 spvBitfieldSExtract(int4 Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int4 Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

void vert_main()
{
    Affine3Packed _142;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _142.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _163 = transpose(float4x3(float3(_142.data[0], _142.data[1], _142.data[2]), float3(_142.data[3], _142.data[4], _142.data[5]), float3(_142.data[6], _142.data[7], _142.data[8]), float3(_142.data[9], _142.data[10], _142.data[11])));
    float3 _168 = mul(_163, float4(in_var_Vertex_Position, 1.0f));
    float3 _169 = _168 * 0.4000000059604644775390625f;
    int3 _171 = int3(floor(_169));
    float3 _172 = frac(_169);
    uint3 _173 = uint3(_171);
    uint _176 = _173.z;
    uint _178 = _176 + (_176 << 10u);
    uint _180 = _178 ^ (_178 >> 6u);
    uint _182 = _180 + (_180 << 3u);
    uint _184 = _182 ^ (_182 >> 11u);
    uint _190 = (((_173.y * 1664525u) + (_184 + (_184 << 15u))) + 1013904223u) * 1664525u;
    uint _192 = _190 ^ (_190 >> 11u);
    uint _195 = _192 ^ ((_192 << 7u) & 2636928640u);
    uint _198 = _195 ^ ((_195 << 15u) & 4022730752u);
    uint _204 = (((_173.x * 1664525u) + (_198 ^ (_198 >> 18u))) + 1013904223u) * 1664525u;
    uint _206 = _204 ^ (_204 >> 11u);
    uint _209 = _206 ^ ((_206 << 7u) & 2636928640u);
    uint _212 = _209 ^ ((_209 << 15u) & 4022730752u);
    uint _214 = _212 ^ (_212 >> 18u);
    uint _216 = _214 + (_214 << 10u);
    uint _218 = _216 ^ (_216 >> 6u);
    uint _220 = _218 + (_218 << 3u);
    uint _222 = _220 ^ (_220 >> 11u);
    uint3 _235 = uint3(_171 + int3(1, 0, 0));
    uint _238 = _235.z;
    uint _240 = _238 + (_238 << 10u);
    uint _242 = _240 ^ (_240 >> 6u);
    uint _244 = _242 + (_242 << 3u);
    uint _246 = _244 ^ (_244 >> 11u);
    uint _252 = (((_235.y * 1664525u) + (_246 + (_246 << 15u))) + 1013904223u) * 1664525u;
    uint _254 = _252 ^ (_252 >> 11u);
    uint _257 = _254 ^ ((_254 << 7u) & 2636928640u);
    uint _260 = _257 ^ ((_257 << 15u) & 4022730752u);
    uint _266 = (((_235.x * 1664525u) + (_260 ^ (_260 >> 18u))) + 1013904223u) * 1664525u;
    uint _268 = _266 ^ (_266 >> 11u);
    uint _271 = _268 ^ ((_268 << 7u) & 2636928640u);
    uint _274 = _271 ^ ((_271 << 15u) & 4022730752u);
    uint _276 = _274 ^ (_274 >> 18u);
    uint _278 = _276 + (_276 << 10u);
    uint _280 = _278 ^ (_278 >> 6u);
    uint _282 = _280 + (_280 << 3u);
    uint _284 = _282 ^ (_282 >> 11u);
    uint3 _297 = uint3(_171 + int3(0, 1, 0));
    uint _300 = _297.z;
    uint _302 = _300 + (_300 << 10u);
    uint _304 = _302 ^ (_302 >> 6u);
    uint _306 = _304 + (_304 << 3u);
    uint _308 = _306 ^ (_306 >> 11u);
    uint _314 = (((_297.y * 1664525u) + (_308 + (_308 << 15u))) + 1013904223u) * 1664525u;
    uint _316 = _314 ^ (_314 >> 11u);
    uint _319 = _316 ^ ((_316 << 7u) & 2636928640u);
    uint _322 = _319 ^ ((_319 << 15u) & 4022730752u);
    uint _328 = (((_297.x * 1664525u) + (_322 ^ (_322 >> 18u))) + 1013904223u) * 1664525u;
    uint _330 = _328 ^ (_328 >> 11u);
    uint _333 = _330 ^ ((_330 << 7u) & 2636928640u);
    uint _336 = _333 ^ ((_333 << 15u) & 4022730752u);
    uint _338 = _336 ^ (_336 >> 18u);
    uint _340 = _338 + (_338 << 10u);
    uint _342 = _340 ^ (_340 >> 6u);
    uint _344 = _342 + (_342 << 3u);
    uint _346 = _344 ^ (_344 >> 11u);
    uint3 _359 = uint3(_171 + int3(1, 1, 0));
    uint _362 = _359.z;
    uint _364 = _362 + (_362 << 10u);
    uint _366 = _364 ^ (_364 >> 6u);
    uint _368 = _366 + (_366 << 3u);
    uint _370 = _368 ^ (_368 >> 11u);
    uint _376 = (((_359.y * 1664525u) + (_370 + (_370 << 15u))) + 1013904223u) * 1664525u;
    uint _378 = _376 ^ (_376 >> 11u);
    uint _381 = _378 ^ ((_378 << 7u) & 2636928640u);
    uint _384 = _381 ^ ((_381 << 15u) & 4022730752u);
    uint _390 = (((_359.x * 1664525u) + (_384 ^ (_384 >> 18u))) + 1013904223u) * 1664525u;
    uint _392 = _390 ^ (_390 >> 11u);
    uint _395 = _392 ^ ((_392 << 7u) & 2636928640u);
    uint _398 = _395 ^ ((_395 << 15u) & 4022730752u);
    uint _400 = _398 ^ (_398 >> 18u);
    uint _402 = _400 + (_400 << 10u);
    uint _404 = _402 ^ (_402 >> 6u);
    uint _406 = _404 + (_404 << 3u);
    uint _408 = _406 ^ (_406 >> 11u);
    uint3 _421 = uint3(_171 + int3(0, 0, 1));
    uint _424 = _421.z;
    uint _426 = _424 + (_424 << 10u);
    uint _428 = _426 ^ (_426 >> 6u);
    uint _430 = _428 + (_428 << 3u);
    uint _432 = _430 ^ (_430 >> 11u);
    uint _438 = (((_421.y * 1664525u) + (_432 + (_432 << 15u))) + 1013904223u) * 1664525u;
    uint _440 = _438 ^ (_438 >> 11u);
    uint _443 = _440 ^ ((_440 << 7u) & 2636928640u);
    uint _446 = _443 ^ ((_443 << 15u) & 4022730752u);
    uint _452 = (((_421.x * 1664525u) + (_446 ^ (_446 >> 18u))) + 1013904223u) * 1664525u;
    uint _454 = _452 ^ (_452 >> 11u);
    uint _457 = _454 ^ ((_454 << 7u) & 2636928640u);
    uint _460 = _457 ^ ((_457 << 15u) & 4022730752u);
    uint _462 = _460 ^ (_460 >> 18u);
    uint _464 = _462 + (_462 << 10u);
    uint _466 = _464 ^ (_464 >> 6u);
    uint _468 = _466 + (_466 << 3u);
    uint _470 = _468 ^ (_468 >> 11u);
    uint3 _483 = uint3(_171 + int3(1, 0, 1));
    uint _486 = _483.z;
    uint _488 = _486 + (_486 << 10u);
    uint _490 = _488 ^ (_488 >> 6u);
    uint _492 = _490 + (_490 << 3u);
    uint _494 = _492 ^ (_492 >> 11u);
    uint _500 = (((_483.y * 1664525u) + (_494 + (_494 << 15u))) + 1013904223u) * 1664525u;
    uint _502 = _500 ^ (_500 >> 11u);
    uint _505 = _502 ^ ((_502 << 7u) & 2636928640u);
    uint _508 = _505 ^ ((_505 << 15u) & 4022730752u);
    uint _514 = (((_483.x * 1664525u) + (_508 ^ (_508 >> 18u))) + 1013904223u) * 1664525u;
    uint _516 = _514 ^ (_514 >> 11u);
    uint _519 = _516 ^ ((_516 << 7u) & 2636928640u);
    uint _522 = _519 ^ ((_519 << 15u) & 4022730752u);
    uint _524 = _522 ^ (_522 >> 18u);
    uint _526 = _524 + (_524 << 10u);
    uint _528 = _526 ^ (_526 >> 6u);
    uint _530 = _528 + (_528 << 3u);
    uint _532 = _530 ^ (_530 >> 11u);
    uint3 _545 = uint3(_171 + int3(0, 1, 1));
    uint _548 = _545.z;
    uint _550 = _548 + (_548 << 10u);
    uint _552 = _550 ^ (_550 >> 6u);
    uint _554 = _552 + (_552 << 3u);
    uint _556 = _554 ^ (_554 >> 11u);
    uint _562 = (((_545.y * 1664525u) + (_556 + (_556 << 15u))) + 1013904223u) * 1664525u;
    uint _564 = _562 ^ (_562 >> 11u);
    uint _567 = _564 ^ ((_564 << 7u) & 2636928640u);
    uint _570 = _567 ^ ((_567 << 15u) & 4022730752u);
    uint _576 = (((_545.x * 1664525u) + (_570 ^ (_570 >> 18u))) + 1013904223u) * 1664525u;
    uint _578 = _576 ^ (_576 >> 11u);
    uint _581 = _578 ^ ((_578 << 7u) & 2636928640u);
    uint _584 = _581 ^ ((_581 << 15u) & 4022730752u);
    uint _586 = _584 ^ (_584 >> 18u);
    uint _588 = _586 + (_586 << 10u);
    uint _590 = _588 ^ (_588 >> 6u);
    uint _592 = _590 + (_590 << 3u);
    uint _594 = _592 ^ (_592 >> 11u);
    uint3 _607 = uint3(_171 + int3(1, 1, 1));
    uint _610 = _607.z;
    uint _612 = _610 + (_610 << 10u);
    uint _614 = _612 ^ (_612 >> 6u);
    uint _616 = _614 + (_614 << 3u);
    uint _618 = _616 ^ (_616 >> 11u);
    uint _624 = (((_607.y * 1664525u) + (_618 + (_618 << 15u))) + 1013904223u) * 1664525u;
    uint _626 = _624 ^ (_624 >> 11u);
    uint _629 = _626 ^ ((_626 << 7u) & 2636928640u);
    uint _632 = _629 ^ ((_629 << 15u) & 4022730752u);
    uint _638 = (((_607.x * 1664525u) + (_632 ^ (_632 >> 18u))) + 1013904223u) * 1664525u;
    uint _640 = _638 ^ (_638 >> 11u);
    uint _643 = _640 ^ ((_640 << 7u) & 2636928640u);
    uint _646 = _643 ^ ((_643 << 15u) & 4022730752u);
    uint _648 = _646 ^ (_646 >> 18u);
    uint _650 = _648 + (_648 << 10u);
    uint _652 = _650 ^ (_650 >> 6u);
    uint _654 = _652 + (_652 << 3u);
    uint _656 = _654 ^ (_654 >> 11u);
    float3 _671 = (_172 * _172) * (3.0f.xxx - (_172 * 2.0f));
    float _672 = _671.x;
    float _673 = 1.0f - _672;
    float _675 = _671.y;
    float _676 = 1.0f - _675;
    float _678 = _671.z;
    float _679 = 1.0f - _678;
    float2 _696 = ((((((float2(asfloat((_214 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_222 + (_222 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _673) * _676) * _679) + (((float2(asfloat((_276 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_284 + (_284 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _672) * _676) * _679)) + (((float2(asfloat((_338 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_346 + (_346 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _673) * _675) * _679)) + (((float2(asfloat((_400 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_408 + (_408 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _672) * _675) * _679)) + (((float2(asfloat((_462 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_470 + (_470 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _673) * _676) * _678);
    float2 _711 = clamp(((((_696 + (((float2(asfloat((_524 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_532 + (_532 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _672) * _676) * _678)) + (((float2(asfloat((_586 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_594 + (_594 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _673) * _675) * _678)) + (((float2(asfloat((_648 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_656 + (_656 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _672) * _675) * _678)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _716 = _168 + (float3(_711.x, 0.0f, _711.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_716, 1.0f));
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(_163, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _716;
    out_var_C3 = in_var_Vertex_UV * 0.75f;
    out_var_C4 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
    out_var_C5 = uint(in_var_is_glass);
    out_var_C6 = mul(_163, float4(0.0f, 0.0f, 0.0f, 1.0f));
    out_var_C7 = spvBitfieldUExtract(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48), 0u, 31u);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_glass = stage_input.in_var_is_glass;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
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
    uint color_id;
    float light_intensity;
    uint has_albedo_texture;
    uint albedo_texture;
};

static float3 _173;
static uint _174;

static const uint2 _178[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };
static const float _180[2] = { 0.20000000298023223876953125f, 1.2000000476837158203125f };

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
    uint2 curtains_texture_dims : packoffset(c0.y);
    uint2 indoors_texture_dims : packoffset(c1);
    int bool_render_shadows : packoffset(c1.z);
};

ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> bindless_textures_dyn[] : register(t0, space2);
Texture2D<float4> curtains_texture : register(t16, space0);
Texture2D<float4> indoors_texture : register(t17, space0);

static float4 gl_FragCoord;
static float3 in_var_C0;
static float3 in_var_C2;
static float2 in_var_C3;
static uint in_var_C4;
static uint in_var_C5;
static float3 in_var_C6;
static uint in_var_C7;
static uint2 out_var_SV_TARGET0;
static float3 out_var_SV_TARGET1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation uint in_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation float3 in_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation uint in_var_C7 : TEXCOORD7;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
    float3 out_var_SV_TARGET1 : SV_Target1;
};

uint spvBitfieldUExtract(uint Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

uint2 spvBitfieldUExtract(uint2 Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

uint3 spvBitfieldUExtract(uint3 Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

uint4 spvBitfieldUExtract(uint4 Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

int spvBitfieldSExtract(int Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

int2 spvBitfieldSExtract(int2 Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int2 Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

int3 spvBitfieldSExtract(int3 Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int3 Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

int4 spvBitfieldSExtract(int4 Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int4 Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

void frag_main()
{
    bool _224;
    if (spvBitfieldUExtract(nani_instance_data.Load<uint>(in_var_C4 * 64 + 48), 31u, 1u) != 0u)
    {
        _224 = bool_render_shadows != 0;
    }
    else
    {
        _224 = false;
    }
    if (_224)
    {
        uint2 _229 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        uint _232 = frame_constants_frame_index & 3u;
        if (all(bool2(_229.x != _178[_232].x, _229.y != _178[_232].y)))
        {
            discard;
        }
    }
    bool _239 = in_var_C5 != 0u;
    bool _252;
    float3 _253;
    if (nani_instance_data.Load<uint>(in_var_C4 * 64 + 56) != 0u)
    {
        uint _61 = nani_instance_data.Load<uint>(in_var_C4 * 64 + 60);
        float3 _248 = bindless_textures_dyn[NonUniformResourceIndex(_61)].SampleBias(sampler_llr, in_var_C3, frame_constants_global_mip_bias).xyz;
        _252 = (length(_248) > 0.119999997317790985107421875f) ? true : _239;
        _253 = _248;
    }
    else
    {
        _252 = _239;
        _253 = in_var_C0;
    }
    float3 _254 = ddx(in_var_C2);
    float3 _255 = ddy(in_var_C2);
    float3 _258 = normalize(cross(_254, -_255));
    uint _566;
    uint _567;
    bool _568;
    bool _569;
    uint _570;
    uint _571;
    bool _572;
    bool _573;
    float3 _574;
    if (_252)
    {
        float3 _265 = float3(-_258.z, 0.0f, _258.x);
        float4 _268 = mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 _274 = normalize(in_var_C2 - (_268.xyz / _268.w.xxx));
        float4 _276;
        uint _279;
        uint _281;
        bool _283;
        bool _285;
        uint _287;
        uint _289;
        bool _291;
        bool _293;
        _276 = 0.0f.xxxx;
        _279 = _174;
        _281 = _174;
        _283 = false;
        _285 = false;
        _287 = _174;
        _289 = _174;
        _291 = false;
        _293 = false;
        float4 _277;
        uint _280;
        uint _282;
        bool _284;
        bool _286;
        uint _288;
        uint _290;
        bool _292;
        bool _294;
        for (int _295 = 0; _295 < 2; _276 = _277, _279 = _280, _281 = _282, _283 = _284, _285 = _286, _287 = _288, _289 = _290, _291 = _292, _293 = _294, _295++)
        {
            float3 _310 = (in_var_C2 + (_274 * (dot((in_var_C6 - (_258 * _180[_295])) - in_var_C2, _258) / dot(_274, _258)))) - in_var_C6;
            float _311 = dot(_310, _265);
            float _312 = _310.y;
            float2 _313 = float2(_311, _312);
            float _314 = _312 * (-1.0f);
            _313.y = _314;
            float4 _559;
            if (0 == _295)
            {
                float2 _345 = (((_311 - (_312 * (-0.300000011920928955078125f))) + in_var_C6.x) + in_var_C6.y).xx;
                int2 _347 = int2(floor(_345));
                float2 _348 = frac(_345);
                uint2 _349 = uint2(_347);
                uint _351 = _349.y;
                uint _353 = _351 + (_351 << 10u);
                uint _355 = _353 ^ (_353 >> 6u);
                uint _357 = _355 + (_355 << 3u);
                uint _359 = _357 ^ (_357 >> 11u);
                bool _362 = _293 ? _293 : true;
                uint _363 = _293 ? _289 : 1664525u;
                bool _364 = _291 ? _291 : true;
                uint _365 = _291 ? _287 : 1013904223u;
                uint _369 = (((_349.x * _363) + (_359 + (_359 << 15u))) + _365) * _363;
                uint _371 = _369 ^ (_369 >> 11u);
                uint _374 = _371 ^ ((_371 << 7u) & 2636928640u);
                uint _377 = _374 ^ ((_374 << 15u) & 4022730752u);
                bool _380 = _285 ? _285 : true;
                uint _381 = _285 ? _281 : 8388607u;
                bool _382 = _283 ? _283 : true;
                uint _383 = _283 ? _279 : 1065353216u;
                uint2 _389 = uint2(_347 + int2(1, 0));
                uint _391 = _389.y;
                uint _393 = _391 + (_391 << 10u);
                uint _395 = _393 ^ (_393 >> 6u);
                uint _397 = _395 + (_395 << 3u);
                uint _399 = _397 ^ (_397 >> 11u);
                bool _402 = _362 ? _362 : true;
                uint _403 = _362 ? _363 : 1664525u;
                bool _404 = _364 ? _364 : true;
                uint _405 = _364 ? _365 : 1013904223u;
                uint _409 = (((_389.x * _403) + (_399 + (_399 << 15u))) + _405) * _403;
                uint _411 = _409 ^ (_409 >> 11u);
                uint _414 = _411 ^ ((_411 << 7u) & 2636928640u);
                uint _417 = _414 ^ ((_414 << 15u) & 4022730752u);
                bool _420 = _380 ? _380 : true;
                uint _421 = _380 ? _381 : 8388607u;
                bool _422 = _382 ? _382 : true;
                uint _423 = _382 ? _383 : 1065353216u;
                uint2 _429 = uint2(_347 + int2(0, 1));
                uint _431 = _429.y;
                uint _433 = _431 + (_431 << 10u);
                uint _435 = _433 ^ (_433 >> 6u);
                uint _437 = _435 + (_435 << 3u);
                uint _439 = _437 ^ (_437 >> 11u);
                bool _442 = _402 ? _402 : true;
                uint _443 = _402 ? _403 : 1664525u;
                bool _444 = _404 ? _404 : true;
                uint _445 = _404 ? _405 : 1013904223u;
                uint _449 = (((_429.x * _443) + (_439 + (_439 << 15u))) + _445) * _443;
                uint _451 = _449 ^ (_449 >> 11u);
                uint _454 = _451 ^ ((_451 << 7u) & 2636928640u);
                uint _457 = _454 ^ ((_454 << 15u) & 4022730752u);
                bool _460 = _420 ? _420 : true;
                uint _461 = _420 ? _421 : 8388607u;
                bool _462 = _422 ? _422 : true;
                uint _463 = _422 ? _423 : 1065353216u;
                uint2 _469 = uint2(_347 + int2(1, 1));
                uint _471 = _469.y;
                uint _473 = _471 + (_471 << 10u);
                uint _475 = _473 ^ (_473 >> 6u);
                uint _477 = _475 + (_475 << 3u);
                uint _479 = _477 ^ (_477 >> 11u);
                uint _483 = _442 ? _443 : 1664525u;
                uint _485 = _444 ? _445 : 1013904223u;
                uint _489 = (((_469.x * _483) + (_479 + (_479 << 15u))) + _485) * _483;
                uint _491 = _489 ^ (_489 >> 11u);
                uint _494 = _491 ^ ((_491 << 7u) & 2636928640u);
                uint _497 = _494 ^ ((_494 << 15u) & 4022730752u);
                uint _501 = _460 ? _461 : 8388607u;
                uint _503 = _462 ? _463 : 1065353216u;
                float2 _511 = (_348 * _348) * (3.0f.xx - (_348 * 2.0f));
                float _512 = _511.x;
                float _513 = 1.0f - _512;
                float _515 = _511.y;
                float _528 = _311 + ((((((1.0f - _515) * (((asfloat(((_377 ^ (_377 >> 18u)) & _381) | _383) - 1.0f) * _513) + ((asfloat(((_417 ^ (_417 >> 18u)) & _421) | _423) - 1.0f) * _512))) + (((asfloat(((_457 ^ (_457 >> 18u)) & _461) | _463) - 1.0f) * _513) * _515)) + (((asfloat(((_497 ^ (_497 >> 18u)) & _501) | _503) - 1.0f) * _512) * _515)) - 0.5f) * 0.300000011920928955078125f);
                float _552;
                do
                {
                    float _542 = abs(max(0.0f, abs(_528) - 0.3549999892711639404296875f));
                    if (_542 > 0.100000001490116119384765625f)
                    {
                        _552 = 0.0f;
                        break;
                    }
                    float _546 = _542 * 10.0f;
                    _552 = 1.0f - ((_546 * _546) * (3.0f - (_542 * 20.0f)));
                    break;
                } while(false);
                _280 = _503;
                _282 = _501;
                _284 = _462 ? _462 : true;
                _286 = _460 ? _460 : true;
                _288 = _485;
                _290 = _483;
                _292 = _444 ? _444 : true;
                _294 = _442 ? _442 : true;
                _559 = float4(sqrt(curtains_texture.SampleGrad(sampler_llr, float2(_528, _314) * float2(3.0f, 1.0f), float2(0.001000000047497451305389404296875f, 0.0f), float2(0.0f, 0.4000000059604644775390625f)).xyz), 1.0f) * (1.0f - _552);
            }
            else
            {
                _280 = _279;
                _282 = _281;
                _284 = _283;
                _286 = _285;
                _288 = _287;
                _290 = _289;
                _292 = _291;
                _294 = _293;
                _559 = float4(0.07500000298023223876953125f.xxx + (sqrt(indoors_texture.SampleLevel(sampler_llr, _313 + 0.5f.xx, 4.5f).xyz) * (0.4000000059604644775390625f * exp((-0.5f) * ((_311 * _311) + (_314 * _314))))), 1.0f);
            }
            _277 = lerp(_276, _559, (1.0f - _276.w).xxxx);
        }
        _566 = _279;
        _567 = _281;
        _568 = _283;
        _569 = _285;
        _570 = _287;
        _571 = _289;
        _572 = _291;
        _573 = _293;
        _574 = clamp(_276 * 0.4000000059604644775390625f, 0.0f.xxxx, 1.0f.xxxx).xyz;
    }
    else
    {
        _566 = _174;
        _567 = _174;
        _568 = false;
        _569 = false;
        _570 = _174;
        _571 = _174;
        _572 = false;
        _573 = false;
        _574 = _253 * 0.5f;
    }
    float3 _794;
    if (nani_instance_data.Load<float>(in_var_C4 * 64 + 52) > 0.0500000007450580596923828125f)
    {
        float3 _601;
        if (in_var_C7 == 0u)
        {
            _601 = float3(0.87999999523162841796875f, 0.84178292751312255859375f, 0.21120001375675201416015625f);
        }
        else
        {
            float3 _600;
            if (in_var_C7 == 1u)
            {
                _600 = float3(0.0f, 0.300000011920928955078125f, 1.5f);
            }
            else
            {
                float3 _599;
                if (in_var_C7 == 2u)
                {
                    _599 = float3(0.16000001132488250732421875f, 0.800000011920928955078125f, 0.16000001132488250732421875f);
                }
                else
                {
                    float3 _598;
                    if (in_var_C7 == 3u)
                    {
                        _598 = float3(0.480000019073486328125f, 0.3200000226497650146484375f, 1.60000002384185791015625f);
                    }
                    else
                    {
                        bool3 _596 = (in_var_C7 == 4u).xxx;
                        _598 = float3(_596.x ? float3(0.800000011920928955078125f, 0.16000001132488250732421875f, 0.080000005662441253662109375f).x : _173.x, _596.y ? float3(0.800000011920928955078125f, 0.16000001132488250732421875f, 0.080000005662441253662109375f).y : _173.y, _596.z ? float3(0.800000011920928955078125f, 0.16000001132488250732421875f, 0.080000005662441253662109375f).z : _173.z);
                    }
                    _599 = _598;
                }
                _600 = _599;
            }
            _601 = _600;
        }
        float2 _606 = ((in_var_C2.xy + in_var_C2.zy) + 1234.0f.xx) * 0.20000000298023223876953125f;
        int2 _608 = int2(floor(_606));
        float2 _609 = frac(_606);
        uint2 _610 = uint2(_608);
        uint _612 = _610.y;
        uint _614 = _612 + (_612 << 10u);
        uint _616 = _614 ^ (_614 >> 6u);
        uint _618 = _616 + (_616 << 3u);
        uint _620 = _618 ^ (_618 >> 11u);
        bool _623 = _573 ? _573 : true;
        uint _624 = _573 ? _571 : 1664525u;
        bool _625 = _572 ? _572 : true;
        uint _626 = _572 ? _570 : 1013904223u;
        uint _630 = (((_610.x * _624) + (_620 + (_620 << 15u))) + _626) * _624;
        uint _632 = _630 ^ (_630 >> 11u);
        uint _635 = _632 ^ ((_632 << 7u) & 2636928640u);
        uint _638 = _635 ^ ((_635 << 15u) & 4022730752u);
        bool _641 = _569 ? _569 : true;
        uint _642 = _569 ? _567 : 8388607u;
        bool _643 = _568 ? _568 : true;
        uint _644 = _568 ? _566 : 1065353216u;
        uint2 _650 = uint2(_608 + int2(1, 0));
        uint _652 = _650.y;
        uint _654 = _652 + (_652 << 10u);
        uint _656 = _654 ^ (_654 >> 6u);
        uint _658 = _656 + (_656 << 3u);
        uint _660 = _658 ^ (_658 >> 11u);
        bool _663 = _623 ? _623 : true;
        uint _664 = _623 ? _624 : 1664525u;
        bool _665 = _625 ? _625 : true;
        uint _666 = _625 ? _626 : 1013904223u;
        uint _670 = (((_650.x * _664) + (_660 + (_660 << 15u))) + _666) * _664;
        uint _672 = _670 ^ (_670 >> 11u);
        uint _675 = _672 ^ ((_672 << 7u) & 2636928640u);
        uint _678 = _675 ^ ((_675 << 15u) & 4022730752u);
        bool _681 = _641 ? _641 : true;
        uint _682 = _641 ? _642 : 8388607u;
        bool _683 = _643 ? _643 : true;
        uint _684 = _643 ? _644 : 1065353216u;
        uint2 _690 = uint2(_608 + int2(0, 1));
        uint _692 = _690.y;
        uint _694 = _692 + (_692 << 10u);
        uint _696 = _694 ^ (_694 >> 6u);
        uint _698 = _696 + (_696 << 3u);
        uint _700 = _698 ^ (_698 >> 11u);
        uint _704 = _663 ? _664 : 1664525u;
        uint _706 = _665 ? _666 : 1013904223u;
        uint _710 = (((_690.x * _704) + (_700 + (_700 << 15u))) + _706) * _704;
        uint _712 = _710 ^ (_710 >> 11u);
        uint _715 = _712 ^ ((_712 << 7u) & 2636928640u);
        uint _718 = _715 ^ ((_715 << 15u) & 4022730752u);
        uint _722 = _681 ? _682 : 8388607u;
        uint _724 = _683 ? _684 : 1065353216u;
        uint2 _730 = uint2(_608 + int2(1, 1));
        uint _732 = _730.y;
        uint _734 = _732 + (_732 << 10u);
        uint _736 = _734 ^ (_734 >> 6u);
        uint _738 = _736 + (_736 << 3u);
        uint _740 = _738 ^ (_738 >> 11u);
        uint _743 = (_663 ? _663 : true) ? _704 : 1664525u;
        uint _748 = (((_730.x * _743) + (_740 + (_740 << 15u))) + ((_665 ? _665 : true) ? _706 : 1013904223u)) * _743;
        uint _750 = _748 ^ (_748 >> 11u);
        uint _753 = _750 ^ ((_750 << 7u) & 2636928640u);
        uint _756 = _753 ^ ((_753 << 15u) & 4022730752u);
        float2 _768 = (_609 * _609) * (3.0f.xx - (_609 * 2.0f));
        float _769 = _768.x;
        float _770 = 1.0f - _769;
        float _772 = _768.y;
        float _786 = (lerp(0.0f, 1.0f, clamp((((1.0f - _772) * (((asfloat(((_638 ^ (_638 >> 18u)) & _642) | _644) - 1.0f) * _770) + ((asfloat(((_678 ^ (_678 >> 18u)) & _682) | _684) - 1.0f) * _769))) + (((asfloat(((_718 ^ (_718 >> 18u)) & _722) | _724) - 1.0f) * _770) * _772)) + (((asfloat(((_756 ^ (_756 >> 18u)) & ((_681 ? _681 : true) ? _722 : 8388607u)) | ((_683 ? _683 : true) ? _724 : 1065353216u)) - 1.0f) * _769) * _772), 0.0f, 1.0f)) * 0.60000002384185791015625f) * nani_instance_data.Load<float>(in_var_C4 * 64 + 52);
        float3 _793;
        if (_252)
        {
            _793 = _601 * _786;
        }
        else
        {
            _793 = (_601 * 0.20000000298023223876953125f) * _786;
        }
        _794 = _793;
    }
    else
    {
        _794 = 0.0f.xxx;
    }
    float3 _796 = (_258 * 0.5f) + 0.5f.xxx;
    float3 _823 = sqrt(clamp(_574, 0.0f.xxx, 1.0f.xxx));
    uint _842 = (uint((clamp(_823.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_823.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_823.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _843 = uint2(((uint((clamp(_796.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_796.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_796.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_252 ? 0.07999999821186065673828125f : 0.800000011920928955078125f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), _842);
    _843.y = _842 | 117440512u;
    out_var_SV_TARGET0 = _843;
    out_var_SV_TARGET1 = _794;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C0 = stage_input.in_var_C0;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C6 = stage_input.in_var_C6;
    in_var_C7 = stage_input.in_var_C7;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    stage_output.out_var_SV_TARGET1 = out_var_SV_TARGET1;
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
    uint color_id;
    float light_intensity;
    uint has_albedo_texture;
    uint albedo_texture;
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
    uint2 curtains_texture_dims : packoffset(c0.y);
    uint2 indoors_texture_dims : packoffset(c1);
    int bool_render_shadows : packoffset(c1.z);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_glass;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static uint out_var_C4;
static uint out_var_C5;
static float3 out_var_C6;
static uint out_var_C7;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_glass: is_glass;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation uint out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation float3 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation uint out_var_C7 : TEXCOORD7;
    float4 gl_Position : SV_Position;
};

uint spvBitfieldUExtract(uint Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

uint2 spvBitfieldUExtract(uint2 Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

uint3 spvBitfieldUExtract(uint3 Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

uint4 spvBitfieldUExtract(uint4 Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

int spvBitfieldSExtract(int Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

int2 spvBitfieldSExtract(int2 Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int2 Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

int3 spvBitfieldSExtract(int3 Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int3 Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

int4 spvBitfieldSExtract(int4 Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int4 Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

void vert_main()
{
    Affine3Packed _142;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _142.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _163 = transpose(float4x3(float3(_142.data[0], _142.data[1], _142.data[2]), float3(_142.data[3], _142.data[4], _142.data[5]), float3(_142.data[6], _142.data[7], _142.data[8]), float3(_142.data[9], _142.data[10], _142.data[11])));
    float3 _168 = mul(_163, float4(in_var_Vertex_Position, 1.0f));
    float3 _169 = _168 * 0.4000000059604644775390625f;
    int3 _171 = int3(floor(_169));
    float3 _172 = frac(_169);
    uint3 _173 = uint3(_171);
    uint _176 = _173.z;
    uint _178 = _176 + (_176 << 10u);
    uint _180 = _178 ^ (_178 >> 6u);
    uint _182 = _180 + (_180 << 3u);
    uint _184 = _182 ^ (_182 >> 11u);
    uint _190 = (((_173.y * 1664525u) + (_184 + (_184 << 15u))) + 1013904223u) * 1664525u;
    uint _192 = _190 ^ (_190 >> 11u);
    uint _195 = _192 ^ ((_192 << 7u) & 2636928640u);
    uint _198 = _195 ^ ((_195 << 15u) & 4022730752u);
    uint _204 = (((_173.x * 1664525u) + (_198 ^ (_198 >> 18u))) + 1013904223u) * 1664525u;
    uint _206 = _204 ^ (_204 >> 11u);
    uint _209 = _206 ^ ((_206 << 7u) & 2636928640u);
    uint _212 = _209 ^ ((_209 << 15u) & 4022730752u);
    uint _214 = _212 ^ (_212 >> 18u);
    uint _216 = _214 + (_214 << 10u);
    uint _218 = _216 ^ (_216 >> 6u);
    uint _220 = _218 + (_218 << 3u);
    uint _222 = _220 ^ (_220 >> 11u);
    uint3 _235 = uint3(_171 + int3(1, 0, 0));
    uint _238 = _235.z;
    uint _240 = _238 + (_238 << 10u);
    uint _242 = _240 ^ (_240 >> 6u);
    uint _244 = _242 + (_242 << 3u);
    uint _246 = _244 ^ (_244 >> 11u);
    uint _252 = (((_235.y * 1664525u) + (_246 + (_246 << 15u))) + 1013904223u) * 1664525u;
    uint _254 = _252 ^ (_252 >> 11u);
    uint _257 = _254 ^ ((_254 << 7u) & 2636928640u);
    uint _260 = _257 ^ ((_257 << 15u) & 4022730752u);
    uint _266 = (((_235.x * 1664525u) + (_260 ^ (_260 >> 18u))) + 1013904223u) * 1664525u;
    uint _268 = _266 ^ (_266 >> 11u);
    uint _271 = _268 ^ ((_268 << 7u) & 2636928640u);
    uint _274 = _271 ^ ((_271 << 15u) & 4022730752u);
    uint _276 = _274 ^ (_274 >> 18u);
    uint _278 = _276 + (_276 << 10u);
    uint _280 = _278 ^ (_278 >> 6u);
    uint _282 = _280 + (_280 << 3u);
    uint _284 = _282 ^ (_282 >> 11u);
    uint3 _297 = uint3(_171 + int3(0, 1, 0));
    uint _300 = _297.z;
    uint _302 = _300 + (_300 << 10u);
    uint _304 = _302 ^ (_302 >> 6u);
    uint _306 = _304 + (_304 << 3u);
    uint _308 = _306 ^ (_306 >> 11u);
    uint _314 = (((_297.y * 1664525u) + (_308 + (_308 << 15u))) + 1013904223u) * 1664525u;
    uint _316 = _314 ^ (_314 >> 11u);
    uint _319 = _316 ^ ((_316 << 7u) & 2636928640u);
    uint _322 = _319 ^ ((_319 << 15u) & 4022730752u);
    uint _328 = (((_297.x * 1664525u) + (_322 ^ (_322 >> 18u))) + 1013904223u) * 1664525u;
    uint _330 = _328 ^ (_328 >> 11u);
    uint _333 = _330 ^ ((_330 << 7u) & 2636928640u);
    uint _336 = _333 ^ ((_333 << 15u) & 4022730752u);
    uint _338 = _336 ^ (_336 >> 18u);
    uint _340 = _338 + (_338 << 10u);
    uint _342 = _340 ^ (_340 >> 6u);
    uint _344 = _342 + (_342 << 3u);
    uint _346 = _344 ^ (_344 >> 11u);
    uint3 _359 = uint3(_171 + int3(1, 1, 0));
    uint _362 = _359.z;
    uint _364 = _362 + (_362 << 10u);
    uint _366 = _364 ^ (_364 >> 6u);
    uint _368 = _366 + (_366 << 3u);
    uint _370 = _368 ^ (_368 >> 11u);
    uint _376 = (((_359.y * 1664525u) + (_370 + (_370 << 15u))) + 1013904223u) * 1664525u;
    uint _378 = _376 ^ (_376 >> 11u);
    uint _381 = _378 ^ ((_378 << 7u) & 2636928640u);
    uint _384 = _381 ^ ((_381 << 15u) & 4022730752u);
    uint _390 = (((_359.x * 1664525u) + (_384 ^ (_384 >> 18u))) + 1013904223u) * 1664525u;
    uint _392 = _390 ^ (_390 >> 11u);
    uint _395 = _392 ^ ((_392 << 7u) & 2636928640u);
    uint _398 = _395 ^ ((_395 << 15u) & 4022730752u);
    uint _400 = _398 ^ (_398 >> 18u);
    uint _402 = _400 + (_400 << 10u);
    uint _404 = _402 ^ (_402 >> 6u);
    uint _406 = _404 + (_404 << 3u);
    uint _408 = _406 ^ (_406 >> 11u);
    uint3 _421 = uint3(_171 + int3(0, 0, 1));
    uint _424 = _421.z;
    uint _426 = _424 + (_424 << 10u);
    uint _428 = _426 ^ (_426 >> 6u);
    uint _430 = _428 + (_428 << 3u);
    uint _432 = _430 ^ (_430 >> 11u);
    uint _438 = (((_421.y * 1664525u) + (_432 + (_432 << 15u))) + 1013904223u) * 1664525u;
    uint _440 = _438 ^ (_438 >> 11u);
    uint _443 = _440 ^ ((_440 << 7u) & 2636928640u);
    uint _446 = _443 ^ ((_443 << 15u) & 4022730752u);
    uint _452 = (((_421.x * 1664525u) + (_446 ^ (_446 >> 18u))) + 1013904223u) * 1664525u;
    uint _454 = _452 ^ (_452 >> 11u);
    uint _457 = _454 ^ ((_454 << 7u) & 2636928640u);
    uint _460 = _457 ^ ((_457 << 15u) & 4022730752u);
    uint _462 = _460 ^ (_460 >> 18u);
    uint _464 = _462 + (_462 << 10u);
    uint _466 = _464 ^ (_464 >> 6u);
    uint _468 = _466 + (_466 << 3u);
    uint _470 = _468 ^ (_468 >> 11u);
    uint3 _483 = uint3(_171 + int3(1, 0, 1));
    uint _486 = _483.z;
    uint _488 = _486 + (_486 << 10u);
    uint _490 = _488 ^ (_488 >> 6u);
    uint _492 = _490 + (_490 << 3u);
    uint _494 = _492 ^ (_492 >> 11u);
    uint _500 = (((_483.y * 1664525u) + (_494 + (_494 << 15u))) + 1013904223u) * 1664525u;
    uint _502 = _500 ^ (_500 >> 11u);
    uint _505 = _502 ^ ((_502 << 7u) & 2636928640u);
    uint _508 = _505 ^ ((_505 << 15u) & 4022730752u);
    uint _514 = (((_483.x * 1664525u) + (_508 ^ (_508 >> 18u))) + 1013904223u) * 1664525u;
    uint _516 = _514 ^ (_514 >> 11u);
    uint _519 = _516 ^ ((_516 << 7u) & 2636928640u);
    uint _522 = _519 ^ ((_519 << 15u) & 4022730752u);
    uint _524 = _522 ^ (_522 >> 18u);
    uint _526 = _524 + (_524 << 10u);
    uint _528 = _526 ^ (_526 >> 6u);
    uint _530 = _528 + (_528 << 3u);
    uint _532 = _530 ^ (_530 >> 11u);
    uint3 _545 = uint3(_171 + int3(0, 1, 1));
    uint _548 = _545.z;
    uint _550 = _548 + (_548 << 10u);
    uint _552 = _550 ^ (_550 >> 6u);
    uint _554 = _552 + (_552 << 3u);
    uint _556 = _554 ^ (_554 >> 11u);
    uint _562 = (((_545.y * 1664525u) + (_556 + (_556 << 15u))) + 1013904223u) * 1664525u;
    uint _564 = _562 ^ (_562 >> 11u);
    uint _567 = _564 ^ ((_564 << 7u) & 2636928640u);
    uint _570 = _567 ^ ((_567 << 15u) & 4022730752u);
    uint _576 = (((_545.x * 1664525u) + (_570 ^ (_570 >> 18u))) + 1013904223u) * 1664525u;
    uint _578 = _576 ^ (_576 >> 11u);
    uint _581 = _578 ^ ((_578 << 7u) & 2636928640u);
    uint _584 = _581 ^ ((_581 << 15u) & 4022730752u);
    uint _586 = _584 ^ (_584 >> 18u);
    uint _588 = _586 + (_586 << 10u);
    uint _590 = _588 ^ (_588 >> 6u);
    uint _592 = _590 + (_590 << 3u);
    uint _594 = _592 ^ (_592 >> 11u);
    uint3 _607 = uint3(_171 + int3(1, 1, 1));
    uint _610 = _607.z;
    uint _612 = _610 + (_610 << 10u);
    uint _614 = _612 ^ (_612 >> 6u);
    uint _616 = _614 + (_614 << 3u);
    uint _618 = _616 ^ (_616 >> 11u);
    uint _624 = (((_607.y * 1664525u) + (_618 + (_618 << 15u))) + 1013904223u) * 1664525u;
    uint _626 = _624 ^ (_624 >> 11u);
    uint _629 = _626 ^ ((_626 << 7u) & 2636928640u);
    uint _632 = _629 ^ ((_629 << 15u) & 4022730752u);
    uint _638 = (((_607.x * 1664525u) + (_632 ^ (_632 >> 18u))) + 1013904223u) * 1664525u;
    uint _640 = _638 ^ (_638 >> 11u);
    uint _643 = _640 ^ ((_640 << 7u) & 2636928640u);
    uint _646 = _643 ^ ((_643 << 15u) & 4022730752u);
    uint _648 = _646 ^ (_646 >> 18u);
    uint _650 = _648 + (_648 << 10u);
    uint _652 = _650 ^ (_650 >> 6u);
    uint _654 = _652 + (_652 << 3u);
    uint _656 = _654 ^ (_654 >> 11u);
    float3 _671 = (_172 * _172) * (3.0f.xxx - (_172 * 2.0f));
    float _672 = _671.x;
    float _673 = 1.0f - _672;
    float _675 = _671.y;
    float _676 = 1.0f - _675;
    float _678 = _671.z;
    float _679 = 1.0f - _678;
    float2 _696 = ((((((float2(asfloat((_214 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_222 + (_222 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _673) * _676) * _679) + (((float2(asfloat((_276 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_284 + (_284 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _672) * _676) * _679)) + (((float2(asfloat((_338 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_346 + (_346 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _673) * _675) * _679)) + (((float2(asfloat((_400 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_408 + (_408 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _672) * _675) * _679)) + (((float2(asfloat((_462 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_470 + (_470 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _673) * _676) * _678);
    float2 _711 = clamp(((((_696 + (((float2(asfloat((_524 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_532 + (_532 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _672) * _676) * _678)) + (((float2(asfloat((_586 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_594 + (_594 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _673) * _675) * _678)) + (((float2(asfloat((_648 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_656 + (_656 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _672) * _675) * _678)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _716 = _168 + (float3(_711.x, 0.0f, _711.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_716, 1.0f));
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(_163, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _716;
    out_var_C3 = in_var_Vertex_UV * 0.75f;
    out_var_C4 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
    out_var_C5 = uint(in_var_is_glass);
    out_var_C6 = mul(_163, float4(0.0f, 0.0f, 0.0f, 1.0f));
    out_var_C7 = spvBitfieldUExtract(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48), 0u, 31u);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_glass = stage_input.in_var_is_glass;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

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
    uint color_id;
    float light_intensity;
    uint has_albedo_texture;
    uint albedo_texture;
};

static const uint2 _83[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

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
    uint2 curtains_texture_dims : packoffset(c0.y);
    uint2 indoors_texture_dims : packoffset(c1);
    int bool_render_shadows : packoffset(c1.z);
};

ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_FragCoord;
static uint in_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    float4 gl_FragCoord : SV_Position;
};

uint spvBitfieldUExtract(uint Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

uint2 spvBitfieldUExtract(uint2 Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

uint3 spvBitfieldUExtract(uint3 Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

uint4 spvBitfieldUExtract(uint4 Base, uint Offset, uint Count)
{
    uint Mask = Count == 32 ? 0xffffffff : ((1 << Count) - 1);
    return (Base >> Offset) & Mask;
}

int spvBitfieldSExtract(int Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

int2 spvBitfieldSExtract(int2 Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int2 Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

int3 spvBitfieldSExtract(int3 Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int3 Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

int4 spvBitfieldSExtract(int4 Base, int Offset, int Count)
{
    int Mask = Count == 32 ? -1 : ((1 << Count) - 1);
    int4 Masked = (Base >> Offset) & Mask;
    int ExtendShift = (32 - Count) & 31;
    return (Masked << ExtendShift) >> ExtendShift;
}

void frag_main()
{
    bool _96;
    if (spvBitfieldUExtract(nani_instance_data.Load<uint>(in_var_C4 * 64 + 48), 31u, 1u) != 0u)
    {
        _96 = bool_render_shadows != 0;
    }
    else
    {
        _96 = false;
    }
    if (_96)
    {
        uint2 _101 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        uint _104 = frame_constants_frame_index & 3u;
        if (all(bool2(_101.x != _83[_104].x, _101.y != _83[_104].y)))
        {
            discard;
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C4 = stage_input.in_var_C4;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
