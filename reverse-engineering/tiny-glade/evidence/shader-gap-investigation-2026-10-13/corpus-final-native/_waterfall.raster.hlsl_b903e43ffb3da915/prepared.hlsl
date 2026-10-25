#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/waterfall.raster.hlsl"




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

struct WaterfallMesh
{
    float4 position;
    float4 tangent;
    float4 normal0;
    float4 normal1;
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
    uint history_valid : packoffset(c0.y);
    uint2 terrain_lighting_texture_dims : packoffset(c0.z);
    uint2 blue_noise_texture_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 brdf_fg_lut_dims : packoffset(c2.z);
    uint rt_bvh_node_count : packoffset(c3);
    uint rt_triangle_count : packoffset(c3.y);
    uint2 water_noise_texture_dims : packoffset(c3.z);
    uint2 framebuffer_dims : packoffset(c4);
    uint2 water_mask_dims : packoffset(c4.z);
    uint2 half_depth_tex_dims : packoffset(c5);
    uint2 refraction_tex_dims : packoffset(c5.z);
    uint2 taa_history_tex_dims : packoffset(c6);
    uint2 reprojection_tex_dims : packoffset(c6.z);
    float3 water_color : packoffset(c7);
};

ByteAddressBuffer instance_buffer : register(t14, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 out_var_C0;
static float3 out_var_C1;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    WaterfallMesh _131;
    _131.position = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 0);
    _131.tangent = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 16);
    _131.normal0 = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 32);
    _131.normal1 = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 48);
    float2 _145 = float2(float(uint(gl_VertexIndex) % 2u), float(uint(gl_VertexIndex) / 2u)) * float2(1.0f, 0.100000001490116119384765625f);
    float _148 = _145.x;
    float3 _150 = lerp(_131.normal0.xyz, _131.normal1.xyz, _148.xxx);
    float _152 = _145.y;
    float3 _160 = (_131.position.xyz + (float3(_131.tangent.xy, 0.0f).xzy * _148)) + (_150 * (_152 * 0.5f));
    _160.y = lerp(_131.position.w, _131.position.y, 1.0f - pow(_152, 3.0f));
    float3 _172 = (_160 * float3(0.5f, 1.0f, 0.5f)) + (float3(0.0f, 3.0f, 0.0f) * (float(frame_constants_game_time_sec) + frame_constants_game_time_frac));
    int3 _174 = int3(floor(_172));
    float3 _175 = frac(_172);
    uint3 _176 = uint3(_174);
    uint _179 = _176.z;
    uint _181 = _179 + (_179 << 10u);
    uint _183 = _181 ^ (_181 >> 6u);
    uint _185 = _183 + (_183 << 3u);
    uint _187 = _185 ^ (_185 >> 11u);
    uint _193 = (((_176.y * 1664525u) + (_187 + (_187 << 15u))) + 1013904223u) * 1664525u;
    uint _195 = _193 ^ (_193 >> 11u);
    uint _198 = _195 ^ ((_195 << 7u) & 2636928640u);
    uint _201 = _198 ^ ((_198 << 15u) & 4022730752u);
    uint _207 = (((_176.x * 1664525u) + (_201 ^ (_201 >> 18u))) + 1013904223u) * 1664525u;
    uint _209 = _207 ^ (_207 >> 11u);
    uint _212 = _209 ^ ((_209 << 7u) & 2636928640u);
    uint _215 = _212 ^ ((_212 << 15u) & 4022730752u);
    uint _217 = _215 ^ (_215 >> 18u);
    uint _219 = _217 + (_217 << 10u);
    uint _221 = _219 ^ (_219 >> 6u);
    uint _223 = _221 + (_221 << 3u);
    uint _225 = _223 ^ (_223 >> 11u);
    uint _227 = _225 + (_225 << 15u);
    uint _233 = _227 + (_227 << 10u);
    uint _235 = _233 ^ (_233 >> 6u);
    uint _237 = _235 + (_235 << 3u);
    uint _239 = _237 ^ (_237 >> 11u);
    uint3 _252 = uint3(_174 + int3(1, 0, 0));
    uint _255 = _252.z;
    uint _257 = _255 + (_255 << 10u);
    uint _259 = _257 ^ (_257 >> 6u);
    uint _261 = _259 + (_259 << 3u);
    uint _263 = _261 ^ (_261 >> 11u);
    uint _269 = (((_252.y * 1664525u) + (_263 + (_263 << 15u))) + 1013904223u) * 1664525u;
    uint _271 = _269 ^ (_269 >> 11u);
    uint _274 = _271 ^ ((_271 << 7u) & 2636928640u);
    uint _277 = _274 ^ ((_274 << 15u) & 4022730752u);
    uint _283 = (((_252.x * 1664525u) + (_277 ^ (_277 >> 18u))) + 1013904223u) * 1664525u;
    uint _285 = _283 ^ (_283 >> 11u);
    uint _288 = _285 ^ ((_285 << 7u) & 2636928640u);
    uint _291 = _288 ^ ((_288 << 15u) & 4022730752u);
    uint _293 = _291 ^ (_291 >> 18u);
    uint _295 = _293 + (_293 << 10u);
    uint _297 = _295 ^ (_295 >> 6u);
    uint _299 = _297 + (_297 << 3u);
    uint _301 = _299 ^ (_299 >> 11u);
    uint _303 = _301 + (_301 << 15u);
    uint _309 = _303 + (_303 << 10u);
    uint _311 = _309 ^ (_309 >> 6u);
    uint _313 = _311 + (_311 << 3u);
    uint _315 = _313 ^ (_313 >> 11u);
    uint3 _328 = uint3(_174 + int3(0, 1, 0));
    uint _331 = _328.z;
    uint _333 = _331 + (_331 << 10u);
    uint _335 = _333 ^ (_333 >> 6u);
    uint _337 = _335 + (_335 << 3u);
    uint _339 = _337 ^ (_337 >> 11u);
    uint _345 = (((_328.y * 1664525u) + (_339 + (_339 << 15u))) + 1013904223u) * 1664525u;
    uint _347 = _345 ^ (_345 >> 11u);
    uint _350 = _347 ^ ((_347 << 7u) & 2636928640u);
    uint _353 = _350 ^ ((_350 << 15u) & 4022730752u);
    uint _359 = (((_328.x * 1664525u) + (_353 ^ (_353 >> 18u))) + 1013904223u) * 1664525u;
    uint _361 = _359 ^ (_359 >> 11u);
    uint _364 = _361 ^ ((_361 << 7u) & 2636928640u);
    uint _367 = _364 ^ ((_364 << 15u) & 4022730752u);
    uint _369 = _367 ^ (_367 >> 18u);
    uint _371 = _369 + (_369 << 10u);
    uint _373 = _371 ^ (_371 >> 6u);
    uint _375 = _373 + (_373 << 3u);
    uint _377 = _375 ^ (_375 >> 11u);
    uint _379 = _377 + (_377 << 15u);
    uint _385 = _379 + (_379 << 10u);
    uint _387 = _385 ^ (_385 >> 6u);
    uint _389 = _387 + (_387 << 3u);
    uint _391 = _389 ^ (_389 >> 11u);
    uint3 _404 = uint3(_174 + int3(1, 1, 0));
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
    uint _455 = _453 + (_453 << 15u);
    uint _461 = _455 + (_455 << 10u);
    uint _463 = _461 ^ (_461 >> 6u);
    uint _465 = _463 + (_463 << 3u);
    uint _467 = _465 ^ (_465 >> 11u);
    uint3 _480 = uint3(_174 + int3(0, 0, 1));
    uint _483 = _480.z;
    uint _485 = _483 + (_483 << 10u);
    uint _487 = _485 ^ (_485 >> 6u);
    uint _489 = _487 + (_487 << 3u);
    uint _491 = _489 ^ (_489 >> 11u);
    uint _497 = (((_480.y * 1664525u) + (_491 + (_491 << 15u))) + 1013904223u) * 1664525u;
    uint _499 = _497 ^ (_497 >> 11u);
    uint _502 = _499 ^ ((_499 << 7u) & 2636928640u);
    uint _505 = _502 ^ ((_502 << 15u) & 4022730752u);
    uint _511 = (((_480.x * 1664525u) + (_505 ^ (_505 >> 18u))) + 1013904223u) * 1664525u;
    uint _513 = _511 ^ (_511 >> 11u);
    uint _516 = _513 ^ ((_513 << 7u) & 2636928640u);
    uint _519 = _516 ^ ((_516 << 15u) & 4022730752u);
    uint _521 = _519 ^ (_519 >> 18u);
    uint _523 = _521 + (_521 << 10u);
    uint _525 = _523 ^ (_523 >> 6u);
    uint _527 = _525 + (_525 << 3u);
    uint _529 = _527 ^ (_527 >> 11u);
    uint _531 = _529 + (_529 << 15u);
    uint _537 = _531 + (_531 << 10u);
    uint _539 = _537 ^ (_537 >> 6u);
    uint _541 = _539 + (_539 << 3u);
    uint _543 = _541 ^ (_541 >> 11u);
    uint3 _556 = uint3(_174 + int3(1, 0, 1));
    uint _559 = _556.z;
    uint _561 = _559 + (_559 << 10u);
    uint _563 = _561 ^ (_561 >> 6u);
    uint _565 = _563 + (_563 << 3u);
    uint _567 = _565 ^ (_565 >> 11u);
    uint _573 = (((_556.y * 1664525u) + (_567 + (_567 << 15u))) + 1013904223u) * 1664525u;
    uint _575 = _573 ^ (_573 >> 11u);
    uint _578 = _575 ^ ((_575 << 7u) & 2636928640u);
    uint _581 = _578 ^ ((_578 << 15u) & 4022730752u);
    uint _587 = (((_556.x * 1664525u) + (_581 ^ (_581 >> 18u))) + 1013904223u) * 1664525u;
    uint _589 = _587 ^ (_587 >> 11u);
    uint _592 = _589 ^ ((_589 << 7u) & 2636928640u);
    uint _595 = _592 ^ ((_592 << 15u) & 4022730752u);
    uint _597 = _595 ^ (_595 >> 18u);
    uint _599 = _597 + (_597 << 10u);
    uint _601 = _599 ^ (_599 >> 6u);
    uint _603 = _601 + (_601 << 3u);
    uint _605 = _603 ^ (_603 >> 11u);
    uint _607 = _605 + (_605 << 15u);
    uint _613 = _607 + (_607 << 10u);
    uint _615 = _613 ^ (_613 >> 6u);
    uint _617 = _615 + (_615 << 3u);
    uint _619 = _617 ^ (_617 >> 11u);
    uint3 _632 = uint3(_174 + int3(0, 1, 1));
    uint _635 = _632.z;
    uint _637 = _635 + (_635 << 10u);
    uint _639 = _637 ^ (_637 >> 6u);
    uint _641 = _639 + (_639 << 3u);
    uint _643 = _641 ^ (_641 >> 11u);
    uint _649 = (((_632.y * 1664525u) + (_643 + (_643 << 15u))) + 1013904223u) * 1664525u;
    uint _651 = _649 ^ (_649 >> 11u);
    uint _654 = _651 ^ ((_651 << 7u) & 2636928640u);
    uint _657 = _654 ^ ((_654 << 15u) & 4022730752u);
    uint _663 = (((_632.x * 1664525u) + (_657 ^ (_657 >> 18u))) + 1013904223u) * 1664525u;
    uint _665 = _663 ^ (_663 >> 11u);
    uint _668 = _665 ^ ((_665 << 7u) & 2636928640u);
    uint _671 = _668 ^ ((_668 << 15u) & 4022730752u);
    uint _673 = _671 ^ (_671 >> 18u);
    uint _675 = _673 + (_673 << 10u);
    uint _677 = _675 ^ (_675 >> 6u);
    uint _679 = _677 + (_677 << 3u);
    uint _681 = _679 ^ (_679 >> 11u);
    uint _683 = _681 + (_681 << 15u);
    uint _689 = _683 + (_683 << 10u);
    uint _691 = _689 ^ (_689 >> 6u);
    uint _693 = _691 + (_691 << 3u);
    uint _695 = _693 ^ (_693 >> 11u);
    uint3 _708 = uint3(_174 + int3(1, 1, 1));
    uint _711 = _708.z;
    uint _713 = _711 + (_711 << 10u);
    uint _715 = _713 ^ (_713 >> 6u);
    uint _717 = _715 + (_715 << 3u);
    uint _719 = _717 ^ (_717 >> 11u);
    uint _725 = (((_708.y * 1664525u) + (_719 + (_719 << 15u))) + 1013904223u) * 1664525u;
    uint _727 = _725 ^ (_725 >> 11u);
    uint _730 = _727 ^ ((_727 << 7u) & 2636928640u);
    uint _733 = _730 ^ ((_730 << 15u) & 4022730752u);
    uint _739 = (((_708.x * 1664525u) + (_733 ^ (_733 >> 18u))) + 1013904223u) * 1664525u;
    uint _741 = _739 ^ (_739 >> 11u);
    uint _744 = _741 ^ ((_741 << 7u) & 2636928640u);
    uint _747 = _744 ^ ((_744 << 15u) & 4022730752u);
    uint _749 = _747 ^ (_747 >> 18u);
    uint _751 = _749 + (_749 << 10u);
    uint _753 = _751 ^ (_751 >> 6u);
    uint _755 = _753 + (_753 << 3u);
    uint _757 = _755 ^ (_755 >> 11u);
    uint _759 = _757 + (_757 << 15u);
    uint _765 = _759 + (_759 << 10u);
    uint _767 = _765 ^ (_765 >> 6u);
    uint _769 = _767 + (_767 << 3u);
    uint _771 = _769 ^ (_769 >> 11u);
    float3 _786 = (_175 * _175) * (3.0f.xxx - (_175 * 2.0f));
    float _787 = _786.x;
    float _788 = 1.0f - _787;
    float _790 = _786.y;
    float _791 = 1.0f - _790;
    float _793 = _786.z;
    float _794 = 1.0f - _793;
    float3 _807 = (((((float3(asfloat((_217 & 8388607u) | 1065353216u) - 1.0f, asfloat((_227 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_239 + (_239 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _788) * _791) * _794) + (((float3(asfloat((_293 & 8388607u) | 1065353216u) - 1.0f, asfloat((_303 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_315 + (_315 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _787) * _791) * _794)) + (((float3(asfloat((_369 & 8388607u) | 1065353216u) - 1.0f, asfloat((_379 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_391 + (_391 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _788) * _790) * _794)) + (((float3(asfloat((_445 & 8388607u) | 1065353216u) - 1.0f, asfloat((_455 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_467 + (_467 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _787) * _790) * _794);
    float3 _823 = (((_807 + (((float3(asfloat((_521 & 8388607u) | 1065353216u) - 1.0f, asfloat((_531 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_543 + (_543 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _788) * _791) * _793)) + (((float3(asfloat((_597 & 8388607u) | 1065353216u) - 1.0f, asfloat((_607 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_619 + (_619 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _787) * _791) * _793)) + (((float3(asfloat((_673 & 8388607u) | 1065353216u) - 1.0f, asfloat((_683 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_695 + (_695 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _788) * _790) * _793)) + (((float3(asfloat((_749 & 8388607u) | 1065353216u) - 1.0f, asfloat((_759 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_771 + (_771 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _787) * _790) * _793);
    float3 _828 = _160 + ((((_823 - 0.5f.xxx) * float3(1.0f, 0.0f, 1.0f)) * _152) * 0.20000000298023223876953125f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_828, 1.0f));
    out_var_C0 = normalize((_150 * (-((_131.position.y - _131.position.w) * ((-3.0f) * pow(_152, 2.0f))))) + float3(0.0f, 0.5f, 0.0f));
    out_var_C1 = _828;
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

struct RippleSourceSsbo
{
    uint4 params[32];
    uint count;
    uint pad0;
    uint pad1;
    uint pad2;
};

struct DofFocus
{
    float focus0;
    float focus1;
    float lock_bias0;
    float lock_bias1;
    float3 lock_pos_ws;
    uint is_locked;
    float3 interp_lock_pos_ws;
    uint use_tilt_shift;
    float interp_lock_bias0;
    float interp_lock_bias1;
    float focus_scale;
    uint pad2;
};

static uint _239;
static float _240;
static float _274;
static float _286;
static float3 _287;
static float2 _288;

static const uint2 _244[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };
static const float _246[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _247[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _248[4] = { 17u, 18u, 20u, 21u };

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
    uint history_valid : packoffset(c0.y);
    uint2 terrain_lighting_texture_dims : packoffset(c0.z);
    uint2 blue_noise_texture_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 brdf_fg_lut_dims : packoffset(c2.z);
    uint rt_bvh_node_count : packoffset(c3);
    uint rt_triangle_count : packoffset(c3.y);
    uint2 water_noise_texture_dims : packoffset(c3.z);
    uint2 framebuffer_dims : packoffset(c4);
    uint2 water_mask_dims : packoffset(c4.z);
    uint2 half_depth_tex_dims : packoffset(c5);
    uint2 refraction_tex_dims : packoffset(c5.z);
    uint2 taa_history_tex_dims : packoffset(c6);
    uint2 reprojection_tex_dims : packoffset(c6.z);
    float3 water_color : packoffset(c7);
};

ByteAddressBuffer ripple_source_buffer : register(t26, space0);
ByteAddressBuffer autofocus_buf : register(t33, space0);
RWByteAddressBuffer __alias_water_rt_count_buffer : register(u34, space0);
RWByteAddressBuffer __alias_water_rt_ray_buffer : register(u35, space0);
SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);
Texture2DArray<float4> shadow_map : register(t18, space0);
Texture2D<float4> water_noise_texture : register(t27, space0);
Texture2D<float4> water_mask : register(t28, space0);
Texture2D<float4> half_depth_tex : register(t29, space0);
Texture2D<float4> refraction_tex : register(t30, space0);
Texture2D<float4> taa_history_tex : register(t31, space0);
Texture2D<float4> reprojection_tex : register(t32, space0);

static float4 gl_FragCoord;
static float3 in_var_C1;
static float4 out_var_SV_TARGET;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    float4 out_var_SV_TARGET : SV_Target0;
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

void frag_main()
{
    bool _308;
    bool _298 = IsHelperLane();
    float3 _299 = in_var_C1;
    uint2 _302 = uint2(gl_FragCoord.xy);
    uint _626;
    uint _627;
    bool _628;
    bool _629;
    uint _630;
    uint _631;
    bool _632;
    bool _633;
    float3 _634;
    do
    {
        _308 = frame_constants_glade_theme_id == 4u;
        if (_308)
        {
            _626 = _239;
            _627 = _239;
            _628 = false;
            _629 = false;
            _630 = _239;
            _631 = _239;
            _632 = false;
            _633 = false;
            _634 = float3(0.0f, 1.0f, 0.0f);
            break;
        }
        uint _510;
        uint _511;
        bool _512;
        bool _513;
        uint _514;
        uint _515;
        bool _516;
        bool _517;
        float3 _518;
        do
        {
            if (_308)
            {
                _510 = _239;
                _511 = _239;
                _512 = false;
                _513 = false;
                _514 = _239;
                _515 = _239;
                _516 = false;
                _517 = false;
                _518 = 0.0f.xxx;
                break;
            }
            float _320 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
            uint _325;
            uint _327;
            bool _329;
            bool _331;
            uint _333;
            uint _335;
            bool _337;
            bool _339;
            _325 = _239;
            _327 = _239;
            _329 = false;
            _331 = false;
            _333 = _239;
            _335 = _239;
            _337 = false;
            _339 = false;
            float2 _297[4];
            for (uint _322 = 0u; _322 < 4u; )
            {
                float _345 = float(_248[_322]);
                int _347 = int(floor(_345));
                float _348 = frac(_345);
                uint _349 = uint(_347);
                uint _351 = _349 + (_349 << 10u);
                uint _353 = _351 ^ (_351 >> 6u);
                uint _355 = _353 + (_353 << 3u);
                uint _357 = _355 ^ (_355 >> 11u);
                bool _360 = _339 ? _339 : true;
                uint _361 = _339 ? _335 : 1664525u;
                bool _362 = _337 ? _337 : true;
                uint _363 = _337 ? _333 : 1013904223u;
                uint _367 = (((_349 * _361) + (_357 + (_357 << 15u))) + _363) * _361;
                uint _369 = _367 ^ (_367 >> 11u);
                uint _372 = _369 ^ ((_369 << 7u) & 2636928640u);
                uint _375 = _372 ^ ((_372 << 15u) & 4022730752u);
                uint _377 = _375 ^ (_375 >> 18u);
                uint _379 = _377 + (_377 << 10u);
                uint _381 = _379 ^ (_379 >> 6u);
                uint _383 = _381 + (_381 << 3u);
                uint _385 = _383 ^ (_383 >> 11u);
                bool _388 = _331 ? _331 : true;
                uint _389 = _331 ? _327 : 8388607u;
                bool _390 = _329 ? _329 : true;
                uint _391 = _329 ? _325 : 1065353216u;
                bool _396 = _388 ? _388 : true;
                uint _397 = _388 ? _389 : 8388607u;
                bool _398 = _390 ? _390 : true;
                uint _399 = _390 ? _391 : 1065353216u;
                uint _406 = uint(_347 + 1);
                uint _408 = _406 + (_406 << 10u);
                uint _410 = _408 ^ (_408 >> 6u);
                uint _412 = _410 + (_410 << 3u);
                uint _414 = _412 ^ (_412 >> 11u);
                uint _336 = _360 ? _361 : 1664525u;
                uint _334 = _362 ? _363 : 1013904223u;
                uint _420 = (((_406 * _336) + (_414 + (_414 << 15u))) + _334) * _336;
                uint _422 = _420 ^ (_420 >> 11u);
                uint _425 = _422 ^ ((_422 << 7u) & 2636928640u);
                uint _428 = _425 ^ ((_425 << 15u) & 4022730752u);
                uint _430 = _428 ^ (_428 >> 18u);
                uint _432 = _430 + (_430 << 10u);
                uint _434 = _432 ^ (_432 >> 6u);
                uint _436 = _434 + (_434 << 3u);
                uint _438 = _436 ^ (_436 >> 11u);
                bool _441 = _396 ? _396 : true;
                uint _442 = _396 ? _397 : 8388607u;
                bool _443 = _398 ? _398 : true;
                uint _444 = _398 ? _399 : 1065353216u;
                uint _328 = _441 ? _442 : 8388607u;
                uint _326 = _443 ? _444 : 1065353216u;
                float _457 = (_348 * _348) * (3.0f - (2.0f * _348));
                _297[_322] = (normalize(((float2(asfloat((_377 & _389) | _391) - 1.0f, asfloat(((_385 + (_385 << 15u)) & _397) | _399) - 1.0f) * (1.0f - _457)) + (float2(asfloat((_430 & _442) | _444) - 1.0f, asfloat(((_438 + (_438 << 15u)) & _328) | _326) - 1.0f) * _457)) - 0.5f.xx) * _320) * 3.0f;
                _322++;
                _325 = _326;
                _327 = _328;
                _329 = _443 ? _443 : true;
                _331 = _441 ? _441 : true;
                _333 = _334;
                _335 = _336;
                _337 = _362 ? _362 : true;
                _339 = _360 ? _360 : true;
                continue;
            }
            float3 _468;
            _468 = 0.0f.xxx;
            for (uint _471 = 0u; _471 < 4u; )
            {
                uint _472 = _471 + 1u;
                float _476 = float(_472) * 10.16640758514404296875f;
                float _477 = cos(_476);
                float _478 = sin(_476);
                float2x2 _482 = float2x2(float2(_477, _478), float2(-_478, _477));
                float2 _509 = _468.yz + mul(((((water_noise_texture.Sample(sampler_llr, mul(_482, (_299.xz * _246[_471]) + _297[_471]) * 0.01200000010430812835693359375f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _247[_471]) * float2(1.0f, _246[_471] * 0.01200000010430812835693359375f).xyy).yz, _482);
                _468 = float3(_468.x, _509.x, _509.y);
                _471 = _472;
                continue;
            }
            _510 = _325;
            _511 = _327;
            _512 = _329;
            _513 = _331;
            _514 = _333;
            _515 = _335;
            _516 = _337;
            _517 = _339;
            _518 = _468;
            break;
        } while(false);
        float3 _611;
        do
        {
            if (_308)
            {
                _611 = 0.0f.xxx;
                break;
            }
            uint _525 = min(32u, ripple_source_buffer.Load<uint>(512));
            float3 _527;
            _527 = 0.0f.xxx;
            for (uint _530 = 0u; _530 < _525; )
            {
                float2 _544 = float2(spvUnpackHalf2x16(ripple_source_buffer.Load<uint4>(_530 * 16 + 0).x & 65535u).x, spvUnpackHalf2x16((ripple_source_buffer.Load<uint4>(_530 * 16 + 0).x >> 16u) & 65535u).x);
                float2 _553 = float2(spvUnpackHalf2x16(ripple_source_buffer.Load<uint4>(_530 * 16 + 0).y & 65535u).x, spvUnpackHalf2x16((ripple_source_buffer.Load<uint4>(_530 * 16 + 0).y >> 16u) & 65535u).x);
                float2 _556 = lerp(_544, _553, 0.5f.xx);
                float2 _557 = _556 - _299.xz;
                float _558 = length(_557);
                float2 _560 = (_553 - _544) * 3.0f;
                float _562 = _558 + dot(_557, _560);
                float _570 = (float(frame_constants_game_time_sec) + frame_constants_game_time_frac) * 6.0f;
                float _573 = max(_558, _562);
                float _574 = _573 * _573;
                float _577 = sin((_562 * 16.3649997711181640625f) - _570) / (1.0f + (_574 * _574));
                float2 _579 = _556 - (_299.xz - float2(0.00999999977648258209228515625f, 0.0f));
                float _580 = length(_579);
                float _582 = _580 + dot(_579, _560);
                float _586 = max(_580, _582);
                float _587 = _586 * _586;
                float2 _593 = _556 - (_299.xz - float2(0.0f, 0.00999999977648258209228515625f));
                float _594 = length(_593);
                float _596 = _594 + dot(_593, _560);
                float _600 = max(_594, _596);
                float _601 = _600 * _600;
                float3 _606 = float3(_577, (sin((_582 * 16.3649997711181640625f) - _570) / (1.0f + (_587 * _587))) - _577, (sin((_596 * 16.3649997711181640625f) - _570) / (1.0f + (_601 * _601))) - _577);
                float2 _608 = _606.yz * 0.5f;
                _527 += (float3(_606.x, _608.x, _608.y) * asfloat(ripple_source_buffer.Load<uint4>(_530 * 16 + 0).z));
                _530++;
                continue;
            }
            _611 = _527;
            break;
        } while(false);
        float3 _614 = float3(_518.y, 1.0f, _518.z);
        float2 _623 = _614.xz + (float3(_611.y, 1.0f, _611.z).xz / abs(1.0f).xx);
        _626 = _510;
        _627 = _511;
        _628 = _512;
        _629 = _513;
        _630 = _514;
        _631 = _515;
        _632 = _516;
        _633 = _517;
        _634 = normalize(float3(_623.x, _614.y, _623.y));
        break;
    } while(false);
    float _799;
    float2 _655 = (((float2(int2((_302 + uint2(1u, 1u)) - _244[frame_constants_frame_index & 3u])) + 0.5f.xx) / float2(framebuffer_dims)) - 0.5f.xx) * float2(2.0f, -2.0f);
    float _656 = _655.x;
    float4 _661 = float4(_656, _655.y, gl_FragCoord.z, 1.0f);
    float4 _662 = mul(frame_constants_view_constants[0u].sample_to_view, _661);
    float4 _663 = mul(frame_constants_view_constants[0u].view_to_world, _662);
    float3 _665 = normalize(mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_656, _655.y, 0.0f, 1.0f))).xyz);
    float3 _666 = -_665;
    float4 _669 = mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
    float3 _670 = _669.xyz;
    float _672 = 1.0f - dot(_666, _634);
    float _673 = _672 * _672;
    float _676 = 0.959999978542327880859375f * ((_673 * _673) * _672);
    float _682 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float _692 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_302.x) + _682)) + (0.005837149918079376220703125f * (float(_302.y) + _682))));
    uint2 _693 = _302 / uint2(2u, 2u);
    float4 _699 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_656, _655.y, half_depth_tex.Load(int3(_693, 0u)).x, 1.0f)));
    float2 _713 = float2(half_depth_tex_dims);
    float3 _714 = _661.xyz;
    float4 _730 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(_299 + (normalize(_665 - ((float3(_634.x, 0.0f, _634.z) * 0.5f) * 2.0f)) * min(4.0f, 4.0f * length(_299 - (_699.xyz / _699.w.xxx)))), 1.0f)));
    float3 _734 = _730.xyz / _730.w.xxx;
    float4 _745 = _661 + (float4(_734 - _714, 0.0f) * float(int(sign(_734.z))));
    float _746 = _745.w;
    float3 _755 = (_745 / (float((_746 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_746))).xxxx).xyz;
    float3 _756 = _755 - _714;
    bool3 _757 = bool3(_756.x < 0.0f.xxx.x, _756.y < 0.0f.xxx.y, _756.z < 0.0f.xxx.z);
    float3 _760 = (float3(_757.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _757.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _757.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _714) / _756;
    float3 _766 = _714 + (_756 * max(0.0f, max(_760.x, _760.y)));
    float3 _767 = _755 - _766;
    bool3 _768 = bool3(_767.x >= 0.0f.xxx.x, _767.y >= 0.0f.xxx.y, _767.z >= 0.0f.xxx.z);
    float3 _771 = (float3(_768.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _768.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _768.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _766) / _767;
    float3 _779 = _766 + (_767 * min(1.0f, min(min(_771.x, _771.y), _771.z)));
    float _977;
    do
    {
        _799 = frame_constants_view_constants[0u].clip_to_view[3].z;
        float _800 = 0.100000001490116119384765625f * _799;
        uint _801 = uint(int(max(2u, min(3u, uint(int(floor(length((((_779.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_766.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _713))))))));
        float _894;
        float _962;
        float _963;
        float _964;
        float _965;
        bool _966;
        do
        {
            float3 _806 = _779 - _766;
            float _814;
            if (_779.z > 9.5367431640625e-07f)
            {
                _814 = float(_801);
            }
            else
            {
                _814 = max(1.0f, float(_801) - 1.0f);
            }
            float _815 = 1.0f / _814;
            float _888;
            float _889;
            float _890;
            float _891;
            float _892;
            bool _893;
            if (_801 > 0u)
            {
                float _821 = lerp(0.0f, 1.0f, pow(_692 * _815, 1.0f));
                float3 _823 = _766 + (_806 * _821);
                float _828 = 1.0f / _823.z;
                float4 _831 = half_depth_tex.SampleLevel(sampler_nnc, (_823.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float _832 = _831.x;
                float _835 = (1.00000202655792236328125f / _832) - _828;
                bool _838 = (_835 < 0.0f) ? true : false;
                float _881;
                float _882;
                float _883;
                float _884;
                float _885;
                bool _886;
                float _887;
                if (_838)
                {
                    _881 = 0.0f;
                    _882 = _835;
                    _883 = _828 - (1.0f / _832);
                    _884 = _800;
                    _885 = _821;
                    _886 = _838;
                    _887 = 0.0f;
                }
                else
                {
                    float _846;
                    float _850;
                    _846 = _835;
                    _850 = _821;
                    float _847;
                    bool _849;
                    float _851;
                    float _876;
                    float _877;
                    float _878;
                    float _879;
                    bool _880;
                    uint _843 = 1u;
                    bool _848 = _838;
                    for (;;)
                    {
                        if (_843 < _801)
                        {
                            _851 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_843) + _692) * _815, 1.0f)));
                            float3 _861 = _766 + (_806 * _851);
                            float _866 = 1.0f / _861.z;
                            float4 _868 = half_depth_tex.SampleLevel(sampler_nnc, (_861.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                            float _869 = _868.x;
                            _847 = (1.00000202655792236328125f / _869) - _866;
                            _849 = (_847 < 0.0f) ? true : false;
                            if (_849)
                            {
                                _876 = _847;
                                _877 = _866 - (1.0f / _869);
                                _878 = _800;
                                _879 = _851;
                                _880 = _849;
                                break;
                            }
                            _843++;
                            _846 = _847;
                            _848 = _849;
                            _850 = _851;
                            continue;
                        }
                        else
                        {
                            _876 = 0.0f;
                            _877 = 0.0f;
                            _878 = 0.0f;
                            _879 = 1.0f;
                            _880 = _848;
                            break;
                        }
                    }
                    _881 = _846;
                    _882 = _876;
                    _883 = _877;
                    _884 = _878;
                    _885 = _879;
                    _886 = _880;
                    _887 = _850;
                }
                _888 = _881;
                _889 = _882;
                _890 = _883;
                _891 = _884;
                _892 = _885;
                _893 = _886;
                _894 = _887;
            }
            else
            {
                _888 = 0.0f;
                _889 = 0.0f;
                _890 = 0.0f;
                _891 = 0.0f;
                _892 = 1.0f;
                _893 = false;
                _894 = 0.0f;
            }
            if (_893)
            {
                float _899;
                float _902;
                float _904;
                float _906;
                float _908;
                float _910;
                _899 = _894;
                _902 = _888;
                _904 = _889;
                _906 = _890;
                _908 = _891;
                _910 = _892;
                for (uint _912 = 0u; _912 < 1u; )
                {
                    float _916 = _899 + _910;
                    float _917 = _916 * 0.5f;
                    float3 _919 = _766 + (_806 * _917);
                    float _924 = 1.0f / _919.z;
                    float4 _927 = half_depth_tex.SampleLevel(sampler_nnc, (_919.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float _928 = _927.x;
                    float _931 = (1.00000202655792236328125f / _928) - _924;
                    bool _934 = (_931 < 0.0f) ? true : false;
                    _899 = _934 ? _899 : _917;
                    _902 = _934 ? _902 : _931;
                    _904 = _934 ? _931 : _904;
                    _906 = _934 ? (_924 - (1.0f / _928)) : _906;
                    _908 = _934 ? _800 : _908;
                    _910 = _934 ? _917 : _910;
                    _912++;
                    continue;
                }
                float _938 = lerp(_899, _910, _902 / (_902 + (-_904)));
                float3 _940 = _766 + (_806 * _938);
                float _945 = 1.0f / _940.z;
                float4 _948 = half_depth_tex.SampleLevel(sampler_nnc, (_940.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float _949 = _948.x;
                float _952 = (1.00000202655792236328125f / _949) - _945;
                bool _957 = (abs(_952) < (_902 * 0.89999997615814208984375f)) ? true : false;
                _962 = _957 ? _938 : _910;
                _963 = _957 ? _952 : _904;
                _964 = _957 ? _800 : _908;
                _965 = _957 ? (_945 - (1.0f / _949)) : _906;
                _966 = true;
                break;
            }
            else
            {
                _962 = _894;
                _963 = _240;
                _964 = _240;
                _965 = _240;
                _966 = false;
                break;
            }
            break; 
        } while(false);
        bool _974;
        if (_966)
        {
            bool _973;
            if (_965 < _964)
            {
                _973 = _963 < _964;
            }
            else
            {
                _973 = false;
            }
            _974 = _973;
        }
        else
        {
            _974 = false;
        }
        if (_974)
        {
            _977 = _962;
            break;
        }
        _977 = _894;
        break;
    } while(false);
    float3 _979 = lerp(_766, _779, _977.xxx);
    float _980 = _979.x;
    float2 _985 = (float4(_980, _979.y, _274, 1.0f).xy * float2(0.5f, -0.5f)) + 0.5f.xx;
    float4 _992 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_980, _979.y, half_depth_tex.SampleLevel(sampler_nnc, _985, 0.0f).x, 1.0f)));
    float3 _996 = _992.xyz / _992.w.xxx;
    float _1005 = _996.y;
    float _1007 = abs(_299.y - _1005);
    int2 _1009 = int2(_302);
    float2 _1218;
    do
    {
        float4 _1017 = mul(frame_constants_view_constants[2u].world_to_sample, float4(_299, 1.0f));
        if (_1017.z > 1.0f)
        {
            _1218 = 0.0f.xx;
            break;
        }
        float2 _1022 = _1017.xy;
        float2 _1023 = abs(_1022);
        bool _1025 = any(bool2(_1023.x > 0.9833333492279052734375f.xx.x, _1023.y > 0.9833333492279052734375f.xx.y));
        float2 _1029;
        if (_1025)
        {
            _1029 = _1022 * 0.5f;
        }
        else
        {
            _1029 = _1022;
        }
        float _1030 = _1025 ? 5.0f : 10.0f;
        float _1032 = _1025 ? 0.5f : 1.0f;
        float2 _1033 = abs(_1029);
        float2 _1036 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * _1030));
        bool _1038 = any(bool2(_1033.x > _1036.x, _1033.y > _1036.y));
        float _1044;
        float2 _1045;
        float _1046;
        if (_1038)
        {
            _1044 = _1030 * 0.2857142984867095947265625f;
            _1045 = _1029 * 0.2857142984867095947265625f;
            _1046 = _1032 * 0.2857142984867095947265625f;
        }
        else
        {
            _1044 = _1030;
            _1045 = _1029;
            _1046 = _1032;
        }
        float _1049 = float(_1038 ? 2u : uint(_1025));
        uint _1051 = 2u + uint(_1049);
        float _1054 = dot(_634, -_670);
        float4 _1066 = float4(_1045.x, _1045.y, _1017.z, _1017.w) + mul(frame_constants_view_constants[_1051].world_to_sample, float4((_634 * (0.066666670143604278564453125f * clamp(1.0f - _1054, 0.0f, 1.0f))) / _1046.xxx, 0.0f));
        float2 _1069 = (_1066.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        float3 _1070 = float3(_1069.x, _1069.y, _1066.z);
        float _1075 = max((0.300000011920928955078125f / _1046) * (1.0f - abs(_1054)), 0.02500000037252902984619140625f);
        float _1086 = float((uint((_1009.x * 11) + (_1009.y * 7)) + (frame_constants_frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
        uint2 _1087 = uint2(_1009);
        float _1099 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_1087.x) + _682)) + (0.005837149918079376220703125f * (float(_1087.y) + _682))));
        float4 _1103 = mul(frame_constants_view_constants[_1051].world_to_sample, float4(_634, 0.0f));
        float _1105;
        int _1108;
        _1105 = 0.0f;
        _1108 = 0;
        float _1106;
        int _1109;
        for (int _1110 = 0; _1110 < 5; _1105 = _1106, _1108 = _1109, _1110++)
        {
            float _1115 = float(_1110);
            float _1117 = (_1115 + _1086) * 2.3999631404876708984375f;
            float3 _1130 = _1070 + float3((((float2(cos(_1117), sin(_1117)) * (_1115 + _1099)) * 0.20000000298023223876953125f.xx) * _1044) * 0.000833333353511989116668701171875f.xx, _1075 * (-0.0009765625f));
            float4 _1137 = shadow_map.SampleLevel(sampler_shadow_nearest, float3(_1130.xy, _1049), 0.0f);
            float _1138 = _1137.x;
            if (_1130.z > _1138)
            {
                _1106 = _1105 + _1138;
                _1109 = _1108 + 1;
            }
            else
            {
                _1106 = _1105;
                _1109 = _1108;
            }
        }
        float _1146 = _1105 / float(_1108);
        if (_1108 == 0)
        {
            _1218 = 0.0f.xx;
            break;
        }
        float _1160 = max(((_1066.z - _1146) * ((17.0666675567626953125f * _1044) * pow(10.0f, smoothstep(0.0f, 0.25f, frame_constants_fog_tint_thickness.w)))) / _1146, 0.0f);
        float _1167 = (_1160 / pow(pow(_1160 / (_1044 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
        float _1169 = _1103.z;
        float2 _1179 = (((_1103.xy * float(int(sign(_1169)))) / max(0.1500000059604644775390625f, abs(_1169)).xx) * 0.0009765625f.xx) * 2.0f;
        float2 _1181;
        _1181 = 0.0f.xx;
        for (int _1184 = 0; _1184 < 10; )
        {
            float _1188 = float(_1184);
            float _1190 = (_1188 + _1086) * 2.3999631404876708984375f;
            float2 _1197 = ((float2(cos(_1190), sin(_1190)) * (_1188 + _1099)) * 0.100000001490116119384765625f.xx) * _1167;
            float3 _1203 = _1070 + float3(_1197 * 0.000833333353511989116668701171875f.xx, _1075 * (-0.0009765625f));
            _1181 += float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(_1203.xy, _1049), _1203.z + min(0.0f, dot(_1179, _1197))), _286);
            _1184++;
            continue;
        }
        _1218 = _1181 * 0.100000001490116119384765625f;
        break;
    } while(false);
    float _1230 = autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 56);
    float _1231 = 1.0f / autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 0);
    float _1232 = 1.0f / autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 4);
    float _1235 = max(0.001000000047497451305389404296875f, max(_1231, _1232)) + 9.9999997473787516355514526367188e-06f;
    float _1238 = max(0.001000000047497451305389404296875f, min(_1231, _1232)) - 9.9999997473787516355514526367188e-06f;
    bool _1239 = autofocus_buf.Load<uint>(frame_constants_camera_idx * 64 + 44) != 0u;
    float3 _1243 = _299;
    _1243.y = _299.y * float(_1239 ? 0 : 1);
    float4 _1244 = mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _1252 = normalize(mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float _1255 = 1.0f / max(1.0000000133514319600180897396058e-10f, dot(_1243 - (_1244.xyz / _1244.w.xxx), _1252));
    float _1256 = _1235 - _1238;
    float _1258 = min(0.00999999977648258209228515625f, 0.0500000007450580596923828125f * _1256);
    float _1278;
    if (_1255 > _1235)
    {
        _1278 = (_1255 - _1235) + _1258;
    }
    else
    {
        float _1275;
        if (_1255 < _1238)
        {
            _1275 = (_1255 - _1238) - _1258;
        }
        else
        {
            _1275 = (-_1258) + (((2.0f * _1258) * (_1255 - _1238)) / _1256);
        }
        _1278 = _1275;
    }
    float _1285;
    if (_1239)
    {
        _1285 = _1278 / max(0.00999999977648258209228515625f, length(_1252.xz));
    }
    else
    {
        _1285 = _1278;
    }
    float _1304 = sqrt(lerp(0.16000001132488250732421875f, 9.9999997473787516355514526367188e-05f, exp2((-0.014999999664723873138427734375f) * sqrt(max(abs(((-_1285) * _1230) * frame_constants_view_constants[0u].view_to_clip[1].y), (_662.xyz / _662.w.xxx).z * (-0.0030000000260770320892333984375f))))));
    float3 _1306 = -reflect(_666, _634);
    float _1307 = _1306.y;
    float3 _1313;
    if (_1307 < 0.0f)
    {
        float3 _1312 = _1306;
        _1312.y = -_1307;
        _1313 = _1312;
    }
    else
    {
        _1313 = _1306;
    }
    float4 _1320 = _661 + mul(frame_constants_view_constants[0u].view_to_clip, mul(frame_constants_view_constants[0u].world_to_view, float4(_1313, 0.0f)));
    float _1321 = _1320.w;
    float3 _1330 = (_1320 / (float((_1321 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_1321))).xxxx).xyz;
    float3 _1331 = _1330 - _714;
    bool3 _1332 = bool3(_1331.x < 0.0f.xxx.x, _1331.y < 0.0f.xxx.y, _1331.z < 0.0f.xxx.z);
    float3 _1335 = (float3(_1332.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1332.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1332.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _714) / _1331;
    float3 _1341 = _714 + (_1331 * max(0.0f, max(_1335.x, _1335.y)));
    float3 _1342 = _1330 - _1341;
    bool3 _1343 = bool3(_1342.x >= 0.0f.xxx.x, _1342.y >= 0.0f.xxx.y, _1342.z >= 0.0f.xxx.z);
    float3 _1346 = (float3(_1343.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _1343.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _1343.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _1341) / _1342;
    float3 _1353 = _1341 + (_1342 * min(min(_1346.x, _1346.y), _1346.z));
    bool _1544;
    float _1545;
    do
    {
        uint _1371 = uint(int(max(2u, min(8u, uint(int(floor(length((((_1353.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_1341.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * _713))))))));
        float _1463;
        float _1529;
        float _1530;
        float _1531;
        float _1532;
        bool _1533;
        do
        {
            float3 _1376 = _1353 - _1341;
            float _1384;
            if (_1353.z > 9.5367431640625e-07f)
            {
                _1384 = float(_1371);
            }
            else
            {
                _1384 = max(1.0f, float(_1371) - 1.0f);
            }
            float _1385 = 1.0f / _1384;
            float _1457;
            float _1458;
            float _1459;
            float _1460;
            float _1461;
            bool _1462;
            if (_1371 > 0u)
            {
                float _1391 = lerp(0.0f, 1.0f, pow(_692 * _1385, 1.0f));
                float3 _1393 = _1341 + (_1376 * _1391);
                float _1398 = 1.0f / _1393.z;
                float4 _1400 = half_depth_tex.SampleLevel(sampler_nnc, (_1393.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float _1401 = _1400.x;
                float _1404 = (1.00000202655792236328125f / _1401) - _1398;
                bool _1407 = (_1404 < 0.0f) ? true : false;
                float _1450;
                float _1451;
                float _1452;
                float _1453;
                float _1454;
                bool _1455;
                float _1456;
                if (_1407)
                {
                    _1450 = 0.0f;
                    _1451 = _1404;
                    _1452 = _1398 - (1.0f / _1401);
                    _1453 = _799;
                    _1454 = _1391;
                    _1455 = _1407;
                    _1456 = 0.0f;
                }
                else
                {
                    float _1415;
                    float _1419;
                    _1415 = _1404;
                    _1419 = _1391;
                    float _1416;
                    bool _1418;
                    float _1420;
                    float _1445;
                    float _1446;
                    float _1447;
                    float _1448;
                    bool _1449;
                    uint _1412 = 1u;
                    bool _1417 = _1407;
                    for (;;)
                    {
                        if (_1412 < _1371)
                        {
                            _1420 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1412) + _692) * _1385, 1.0f)));
                            float3 _1430 = _1341 + (_1376 * _1420);
                            float _1435 = 1.0f / _1430.z;
                            float4 _1437 = half_depth_tex.SampleLevel(sampler_nnc, (_1430.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                            float _1438 = _1437.x;
                            _1416 = (1.00000202655792236328125f / _1438) - _1435;
                            _1418 = (_1416 < 0.0f) ? true : false;
                            if (_1418)
                            {
                                _1445 = _1416;
                                _1446 = _1435 - (1.0f / _1438);
                                _1447 = _799;
                                _1448 = _1420;
                                _1449 = _1418;
                                break;
                            }
                            _1412++;
                            _1415 = _1416;
                            _1417 = _1418;
                            _1419 = _1420;
                            continue;
                        }
                        else
                        {
                            _1445 = 0.0f;
                            _1446 = 0.0f;
                            _1447 = 0.0f;
                            _1448 = 1.0f;
                            _1449 = _1417;
                            break;
                        }
                    }
                    _1450 = _1415;
                    _1451 = _1445;
                    _1452 = _1446;
                    _1453 = _1447;
                    _1454 = _1448;
                    _1455 = _1449;
                    _1456 = _1419;
                }
                _1457 = _1450;
                _1458 = _1451;
                _1459 = _1452;
                _1460 = _1453;
                _1461 = _1454;
                _1462 = _1455;
                _1463 = _1456;
            }
            else
            {
                _1457 = 0.0f;
                _1458 = 0.0f;
                _1459 = 0.0f;
                _1460 = 0.0f;
                _1461 = 1.0f;
                _1462 = false;
                _1463 = 0.0f;
            }
            if (_1462)
            {
                float _1468;
                float _1471;
                float _1473;
                float _1475;
                float _1477;
                float _1479;
                _1468 = _1463;
                _1471 = _1457;
                _1473 = _1458;
                _1475 = _1459;
                _1477 = _1460;
                _1479 = _1461;
                for (uint _1481 = 0u; _1481 < 3u; )
                {
                    float _1485 = _1468 + _1479;
                    float _1486 = _1485 * 0.5f;
                    float3 _1488 = _1341 + (_1376 * _1486);
                    float _1493 = 1.0f / _1488.z;
                    float4 _1495 = half_depth_tex.SampleLevel(sampler_nnc, (_1488.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float _1496 = _1495.x;
                    float _1499 = (1.00000202655792236328125f / _1496) - _1493;
                    bool _1502 = (_1499 < 0.0f) ? true : false;
                    _1468 = _1502 ? _1468 : _1486;
                    _1471 = _1502 ? _1471 : _1499;
                    _1473 = _1502 ? _1499 : _1473;
                    _1475 = _1502 ? (_1493 - (1.0f / _1496)) : _1475;
                    _1477 = _1502 ? _799 : _1477;
                    _1479 = _1502 ? _1486 : _1479;
                    _1481++;
                    continue;
                }
                float _1506 = lerp(_1468, _1479, _1471 / (_1471 + (-_1473)));
                float3 _1508 = _1341 + (_1376 * _1506);
                float _1513 = 1.0f / _1508.z;
                float4 _1515 = half_depth_tex.SampleLevel(sampler_nnc, (_1508.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float _1516 = _1515.x;
                float _1519 = (1.00000202655792236328125f / _1516) - _1513;
                bool _1524 = (abs(_1519) < (_1471 * 0.89999997615814208984375f)) ? true : false;
                _1529 = _1524 ? _1506 : _1479;
                _1530 = _1524 ? _1519 : _1473;
                _1531 = _1524 ? _799 : _1477;
                _1532 = _1524 ? (_1513 - (1.0f / _1516)) : _1475;
                _1533 = true;
                break;
            }
            else
            {
                _1529 = _1463;
                _1530 = _240;
                _1531 = _240;
                _1532 = _240;
                _1533 = false;
                break;
            }
            break; 
        } while(false);
        bool _1541;
        if (_1533)
        {
            bool _1540;
            if (_1532 < _1531)
            {
                _1540 = _1530 < _1531;
            }
            else
            {
                _1540 = false;
            }
            _1541 = _1540;
        }
        else
        {
            _1541 = false;
        }
        if (_1541)
        {
            _1544 = true;
            _1545 = _1529;
            break;
        }
        _1544 = false;
        _1545 = _1463;
        break;
    } while(false);
    float _1546 = QuadReadAcrossX(_1545);
    float _1548 = QuadReadAcrossX(float(_1544));
    bool _1554 = (int(_1548 != 0.0f) & int(!_1544)) != 0;
    bool _1555 = _1554 ? true : _1544;
    float _1556 = _1554 ? _1546 : _1545;
    float _1557 = QuadReadAcrossY(_1556);
    float _1559 = QuadReadAcrossY(float(_1555));
    bool _1565 = (int(_1559 != 0.0f) & int(!_1555)) != 0;
    float _1566 = _1565 ? _1557 : _1556;
    float3 _1685;
    if (_1565 ? true : _1555)
    {
        float2 _1662 = (lerp(_1341, _1353, _1566.xxx).xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        bool _1675;
        if (history_valid != 0u)
        {
            _1675 = !(0u == frame_constants_frame_index);
        }
        else
        {
            _1675 = false;
        }
        float3 _1684;
        if (_1675)
        {
            _1684 = taa_history_tex.SampleLevel(sampler_lnc, reprojection_tex.SampleLevel(sampler_nnc, _1662, 0.0f).xy + _1662, 0.0f).xyz;
        }
        else
        {
            _1684 = 0.0f.xxx;
        }
        _1685 = _1684;
    }
    else
    {
        float3 _1572 = lerp(_1341, _1353, _1566.xxx);
        float4 _1578 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_1572, 1.0f)));
        uint _1593;
        if (_298)
        {
            _1593 = 0u;
        }
        else
        {
            uint2 _1586 = _302 & uint2(1u, 1u);
            _1593 = 1u << ((_1586.x + (_1586.y * 2u)) & 31u);
        }
        uint _1594 = QuadReadAcrossX(_1593);
        uint _1595 = _1593 + _1594;
        uint _1596 = QuadReadAcrossY(_1595);
        uint _1597 = _1595 + _1596;
        uint2 _1598 = _302 & uint2(1u, 1u);
        if ((_1598.x + (_1598.y * 2u)) == firstbithigh(_1597))
        {
            uint _1608;
            __alias_water_rt_count_buffer.InterlockedAdd(0, 1u, _1608);
            float3 _1620 = _634 / ((abs(_634.x) + abs(_634.y)) + abs(_634.z)).xxx;
            float3 _1634;
            if (_1620.z < 0.0f)
            {
                float2 _1632 = (1.0f.xx - abs(_1620.yx)) * ((step(0.0f.xx, _1620.xy) * 2.0f) - 1.0f.xx);
                _1634 = float3(_1632.x, _1632.y, _1620.z);
            }
            else
            {
                _1634 = _1620;
            }
            float2 _1637 = (_1634.xy * 0.5f) + 0.5f.xx;
            __alias_water_rt_ray_buffer.Store<uint4>(_1608 * 16 + 0, uint4(_693.x + (_693.y * 65536u), spvPackHalf2x16(float2(_1637.x, 0.0f)) | (spvPackHalf2x16(float2(_1637.y, 0.0f)) << 16u), asuint(dot(_1313, (_1578.xyz / _1578.w.xxx) - (_663.xyz / _663.w.xxx))), (asuint(gl_FragCoord.z) & 4294967280u) + _1597));
        }
        _1685 = 0.0f.xxx;
    }
    float _1689;
    float _1686 = _1304 * _1304;
    float3 _1743;
    do
    {
        _1689 = dot(_634, _670);
        bool _1696;
        if (!(_1689 <= 0.0f))
        {
            _1696 = dot(_634, _666) <= 0.0f;
        }
        else
        {
            _1696 = true;
        }
        if (_1696)
        {
            _1743 = 0.0f.xxx;
            break;
        }
        float _1699 = _1686 * _1686;
        float3 _1701 = normalize(_666 + _670);
        float _1702 = dot(_634, _1701);
        float _1705 = max(0.0f, 1.0f - dot(_1701, _670));
        float _1706 = _1705 * _1705;
        float _1711 = dot(_634, _666);
        float _1734 = ((_1702 * _1702) * (_1699 - 1.0f)) + 1.0f;
        _1743 = ((lerp(0.039999999105930328369140625f.xxx, 1.0f.xxx, ((_1706 * _1706) * _1705).xxx) * (((2.0f * _1689) * _1711) / ((_1689 * sqrt(((((-_1711) * _1699) + _1711) * _1711) + _1699)) + (_1711 * sqrt(((((-_1689) * _1699) + _1689) * _1689) + _1699))))) * (_1699 / ((3.1415927410125732421875f * _1734) * _1734))) / ((4.0f * _1711) * _1689).xxx;
        break;
    } while(false);
    float2 _1773 = (_996 - ((_670 * _1005) / _669.y.xxx)).xz;
    uint _1973;
    uint _1974;
    bool _1975;
    bool _1976;
    uint _1977;
    uint _1978;
    bool _1979;
    bool _1980;
    float3 _1981;
    do
    {
        if (_308)
        {
            _1973 = _626;
            _1974 = _627;
            _1975 = _628;
            _1976 = _629;
            _1977 = _630;
            _1978 = _631;
            _1979 = _632;
            _1980 = _633;
            _1981 = 0.0f.xxx;
            break;
        }
        float _1783 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _1788;
        uint _1790;
        bool _1792;
        bool _1794;
        uint _1796;
        uint _1798;
        bool _1800;
        bool _1802;
        _1788 = _626;
        _1790 = _627;
        _1792 = _628;
        _1794 = _629;
        _1796 = _630;
        _1798 = _631;
        _1800 = _632;
        _1802 = _633;
        float2 _293[4];
        for (uint _1785 = 0u; _1785 < 4u; )
        {
            float _1808 = float(_248[_1785]);
            int _1810 = int(floor(_1808));
            float _1811 = frac(_1808);
            uint _1812 = uint(_1810);
            uint _1814 = _1812 + (_1812 << 10u);
            uint _1816 = _1814 ^ (_1814 >> 6u);
            uint _1818 = _1816 + (_1816 << 3u);
            uint _1820 = _1818 ^ (_1818 >> 11u);
            bool _1823 = _1802 ? _1802 : true;
            uint _1824 = _1802 ? _1798 : 1664525u;
            bool _1825 = _1800 ? _1800 : true;
            uint _1826 = _1800 ? _1796 : 1013904223u;
            uint _1830 = (((_1812 * _1824) + (_1820 + (_1820 << 15u))) + _1826) * _1824;
            uint _1832 = _1830 ^ (_1830 >> 11u);
            uint _1835 = _1832 ^ ((_1832 << 7u) & 2636928640u);
            uint _1838 = _1835 ^ ((_1835 << 15u) & 4022730752u);
            uint _1840 = _1838 ^ (_1838 >> 18u);
            uint _1842 = _1840 + (_1840 << 10u);
            uint _1844 = _1842 ^ (_1842 >> 6u);
            uint _1846 = _1844 + (_1844 << 3u);
            uint _1848 = _1846 ^ (_1846 >> 11u);
            bool _1851 = _1794 ? _1794 : true;
            uint _1852 = _1794 ? _1790 : 8388607u;
            bool _1853 = _1792 ? _1792 : true;
            uint _1854 = _1792 ? _1788 : 1065353216u;
            bool _1859 = _1851 ? _1851 : true;
            uint _1860 = _1851 ? _1852 : 8388607u;
            bool _1861 = _1853 ? _1853 : true;
            uint _1862 = _1853 ? _1854 : 1065353216u;
            uint _1869 = uint(_1810 + 1);
            uint _1871 = _1869 + (_1869 << 10u);
            uint _1873 = _1871 ^ (_1871 >> 6u);
            uint _1875 = _1873 + (_1873 << 3u);
            uint _1877 = _1875 ^ (_1875 >> 11u);
            uint _1799 = _1823 ? _1824 : 1664525u;
            uint _1797 = _1825 ? _1826 : 1013904223u;
            uint _1883 = (((_1869 * _1799) + (_1877 + (_1877 << 15u))) + _1797) * _1799;
            uint _1885 = _1883 ^ (_1883 >> 11u);
            uint _1888 = _1885 ^ ((_1885 << 7u) & 2636928640u);
            uint _1891 = _1888 ^ ((_1888 << 15u) & 4022730752u);
            uint _1893 = _1891 ^ (_1891 >> 18u);
            uint _1895 = _1893 + (_1893 << 10u);
            uint _1897 = _1895 ^ (_1895 >> 6u);
            uint _1899 = _1897 + (_1897 << 3u);
            uint _1901 = _1899 ^ (_1899 >> 11u);
            bool _1904 = _1859 ? _1859 : true;
            uint _1905 = _1859 ? _1860 : 8388607u;
            bool _1906 = _1861 ? _1861 : true;
            uint _1907 = _1861 ? _1862 : 1065353216u;
            uint _1791 = _1904 ? _1905 : 8388607u;
            uint _1789 = _1906 ? _1907 : 1065353216u;
            float _1920 = (_1811 * _1811) * (3.0f - (2.0f * _1811));
            _293[_1785] = (normalize(((float2(asfloat((_1840 & _1852) | _1854) - 1.0f, asfloat(((_1848 + (_1848 << 15u)) & _1860) | _1862) - 1.0f) * (1.0f - _1920)) + (float2(asfloat((_1893 & _1905) | _1907) - 1.0f, asfloat(((_1901 + (_1901 << 15u)) & _1791) | _1789) - 1.0f) * _1920)) - 0.5f.xx) * _1783) * 3.0f;
            _1785++;
            _1788 = _1789;
            _1790 = _1791;
            _1792 = _1906 ? _1906 : true;
            _1794 = _1904 ? _1904 : true;
            _1796 = _1797;
            _1798 = _1799;
            _1800 = _1825 ? _1825 : true;
            _1802 = _1823 ? _1823 : true;
            continue;
        }
        float3 _1931;
        _1931 = 0.0f.xxx;
        for (uint _1934 = 0u; _1934 < 4u; )
        {
            uint _1935 = _1934 + 1u;
            float _1939 = float(_1935) * 10.16640758514404296875f;
            float _1940 = cos(_1939);
            float _1941 = sin(_1939);
            float3 _1972;
            _1972.x = _1931.x + ((((water_noise_texture.Sample(sampler_llr, mul(float2x2(float2(_1940, _1941), float2(-_1941, _1940)), (_1773 * _246[_1934]) + _293[_1934]) * 0.01200000010430812835693359375f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _247[_1934]) * float2(1.0f, _246[_1934] * 0.01200000010430812835693359375f).xyy).x;
            _1931 = float3(_1972.x, _288.x, _288.y);
            _1934 = _1935;
            continue;
        }
        _1973 = _1788;
        _1974 = _1790;
        _1975 = _1792;
        _1976 = _1794;
        _1977 = _1796;
        _1978 = _1798;
        _1979 = _1800;
        _1980 = _1802;
        _1981 = _1931;
        break;
    } while(false);
    float3 _2044;
    do
    {
        if (_308)
        {
            _2044 = 0.0f.xxx;
            break;
        }
        uint _1988 = min(32u, ripple_source_buffer.Load<uint>(512));
        float3 _1990;
        _1990 = 0.0f.xxx;
        for (uint _1993 = 0u; _1993 < _1988; )
        {
            float2 _2007 = float2(spvUnpackHalf2x16(ripple_source_buffer.Load<uint4>(_1993 * 16 + 0).x & 65535u).x, spvUnpackHalf2x16((ripple_source_buffer.Load<uint4>(_1993 * 16 + 0).x >> 16u) & 65535u).x);
            float2 _2016 = float2(spvUnpackHalf2x16(ripple_source_buffer.Load<uint4>(_1993 * 16 + 0).y & 65535u).x, spvUnpackHalf2x16((ripple_source_buffer.Load<uint4>(_1993 * 16 + 0).y >> 16u) & 65535u).x);
            float2 _2020 = lerp(_2007, _2016, 0.5f.xx) - _1773;
            float _2021 = length(_2020);
            float _2025 = _2021 + dot(_2020, (_2016 - _2007) * 3.0f);
            float _2036 = max(_2021, _2025);
            float _2037 = _2036 * _2036;
            _1990 += (float3(float3(sin((_2025 * 16.3649997711181640625f) - ((float(frame_constants_game_time_sec) + frame_constants_game_time_frac) * 6.0f)) / (1.0f + (_2037 * _2037)), _286, _286).x, _288.x, _288.y) * asfloat(ripple_source_buffer.Load<uint4>(_1993 * 16 + 0).z));
            _1993++;
            continue;
        }
        _2044 = _1990;
        break;
    } while(false);
    float _2048 = 0.60000002384185791015625f + ((0.4000000059604644775390625f * (exp2((-0.4000000059604644775390625f) * _1007) * smoothstep(0.0f, 1.0f, _1007))) * (_1981 + _2044).x);
    float3 _2251;
    if (_308)
    {
        float2 _2064 = _299.xz * 1.0f;
        int2 _2066 = int2(floor(_2064));
        float2 _2067 = frac(_2064);
        uint2 _2068 = uint2(_2066);
        uint _2070 = _2068.y;
        uint _2072 = _2070 + (_2070 << 10u);
        uint _2074 = _2072 ^ (_2072 >> 6u);
        uint _2076 = _2074 + (_2074 << 3u);
        uint _2078 = _2076 ^ (_2076 >> 11u);
        bool _2081 = _1980 ? _1980 : true;
        uint _2082 = _1980 ? _1978 : 1664525u;
        bool _2083 = _1979 ? _1979 : true;
        uint _2084 = _1979 ? _1977 : 1013904223u;
        uint _2088 = (((_2068.x * _2082) + (_2078 + (_2078 << 15u))) + _2084) * _2082;
        uint _2090 = _2088 ^ (_2088 >> 11u);
        uint _2093 = _2090 ^ ((_2090 << 7u) & 2636928640u);
        uint _2096 = _2093 ^ ((_2093 << 15u) & 4022730752u);
        bool _2099 = _1976 ? _1976 : true;
        uint _2100 = _1976 ? _1974 : 8388607u;
        bool _2101 = _1975 ? _1975 : true;
        uint _2102 = _1975 ? _1973 : 1065353216u;
        uint2 _2108 = uint2(_2066 + int2(1, 0));
        uint _2110 = _2108.y;
        uint _2112 = _2110 + (_2110 << 10u);
        uint _2114 = _2112 ^ (_2112 >> 6u);
        uint _2116 = _2114 + (_2114 << 3u);
        uint _2118 = _2116 ^ (_2116 >> 11u);
        bool _2121 = _2081 ? _2081 : true;
        uint _2122 = _2081 ? _2082 : 1664525u;
        bool _2123 = _2083 ? _2083 : true;
        uint _2124 = _2083 ? _2084 : 1013904223u;
        uint _2128 = (((_2108.x * _2122) + (_2118 + (_2118 << 15u))) + _2124) * _2122;
        uint _2130 = _2128 ^ (_2128 >> 11u);
        uint _2133 = _2130 ^ ((_2130 << 7u) & 2636928640u);
        uint _2136 = _2133 ^ ((_2133 << 15u) & 4022730752u);
        bool _2139 = _2099 ? _2099 : true;
        uint _2140 = _2099 ? _2100 : 8388607u;
        bool _2141 = _2101 ? _2101 : true;
        uint _2142 = _2101 ? _2102 : 1065353216u;
        uint2 _2148 = uint2(_2066 + int2(0, 1));
        uint _2150 = _2148.y;
        uint _2152 = _2150 + (_2150 << 10u);
        uint _2154 = _2152 ^ (_2152 >> 6u);
        uint _2156 = _2154 + (_2154 << 3u);
        uint _2158 = _2156 ^ (_2156 >> 11u);
        uint _2162 = _2121 ? _2122 : 1664525u;
        uint _2164 = _2123 ? _2124 : 1013904223u;
        uint _2168 = (((_2148.x * _2162) + (_2158 + (_2158 << 15u))) + _2164) * _2162;
        uint _2170 = _2168 ^ (_2168 >> 11u);
        uint _2173 = _2170 ^ ((_2170 << 7u) & 2636928640u);
        uint _2176 = _2173 ^ ((_2173 << 15u) & 4022730752u);
        uint _2180 = _2139 ? _2140 : 8388607u;
        uint _2182 = _2141 ? _2142 : 1065353216u;
        uint2 _2188 = uint2(_2066 + int2(1, 1));
        uint _2190 = _2188.y;
        uint _2192 = _2190 + (_2190 << 10u);
        uint _2194 = _2192 ^ (_2192 >> 6u);
        uint _2196 = _2194 + (_2194 << 3u);
        uint _2198 = _2196 ^ (_2196 >> 11u);
        uint _2201 = (_2121 ? _2121 : true) ? _2162 : 1664525u;
        uint _2206 = (((_2188.x * _2201) + (_2198 + (_2198 << 15u))) + ((_2123 ? _2123 : true) ? _2164 : 1013904223u)) * _2201;
        uint _2208 = _2206 ^ (_2206 >> 11u);
        uint _2211 = _2208 ^ ((_2208 << 7u) & 2636928640u);
        uint _2214 = _2211 ^ ((_2211 << 15u) & 4022730752u);
        float2 _2226 = (_2067 * _2067) * (3.0f.xx - (_2067 * 2.0f));
        float _2227 = _2226.x;
        float _2228 = 1.0f - _2227;
        float _2230 = _2226.y;
        float _2240 = (((1.0f - _2230) * (((asfloat(((_2096 ^ (_2096 >> 18u)) & _2100) | _2102) - 1.0f) * _2228) + ((asfloat(((_2136 ^ (_2136 >> 18u)) & _2140) | _2142) - 1.0f) * _2227))) + (((asfloat(((_2176 ^ (_2176 >> 18u)) & _2180) | _2182) - 1.0f) * _2228) * _2230)) + (((asfloat(((_2214 ^ (_2214 >> 18u)) & ((_2139 ? _2139 : true) ? _2180 : 8388607u)) | ((_2141 ? _2141 : true) ? _2182 : 1065353216u)) - 1.0f) * _2227) * _2230);
        _2251 = lerp(0.0f.xxx, 2.0f.xxx, (clamp((1.0f - smoothstep(0.699999988079071044921875f, 0.85000002384185791015625f, water_mask.SampleLevel(sampler_llc, (_299.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y + ((_2240 * _2240) * 0.20000000298023223876953125f))) * 0.20000000298023223876953125f, 0.0f, 1.0f) + 0.100000001490116119384765625f).xxx);
    }
    else
    {
        _2251 = 0.0f.xxx;
    }
    out_var_SV_TARGET = float4(((((exp((-water_color) * ((_1007 + length(_996 - _299)) + 0.699999988079071044921875f)) * (0.959999978542327880859375f - _676)) * refraction_tex.SampleLevel(sampler_lnc, _985, 0.0f).xyz) * (1.0f + ((_2048 * _2048) * _2048))) + ((_1685 + ((frame_constants_sun_color.xyz * min(30.0f, _1743.x)) * ((1.0f - _1218.x) * clamp(_1689, 0.0f, 1.0f)))) * (0.039999999105930328369140625f + _676))) + _2251, 0.0f);
}

[earlydepthstencil]
SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET = out_var_SV_TARGET;
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

struct WaterfallMesh
{
    float4 position;
    float4 tangent;
    float4 normal0;
    float4 normal1;
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
    uint history_valid : packoffset(c0.y);
    uint2 terrain_lighting_texture_dims : packoffset(c0.z);
    uint2 blue_noise_texture_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 brdf_fg_lut_dims : packoffset(c2.z);
    uint rt_bvh_node_count : packoffset(c3);
    uint rt_triangle_count : packoffset(c3.y);
    uint2 water_noise_texture_dims : packoffset(c3.z);
    uint2 framebuffer_dims : packoffset(c4);
    uint2 water_mask_dims : packoffset(c4.z);
    uint2 half_depth_tex_dims : packoffset(c5);
    uint2 refraction_tex_dims : packoffset(c5.z);
    uint2 taa_history_tex_dims : packoffset(c6);
    uint2 reprojection_tex_dims : packoffset(c6.z);
    float3 water_color : packoffset(c7);
};

ByteAddressBuffer instance_buffer : register(t14, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 out_var_C0;
static float3 out_var_C1;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    WaterfallMesh _131;
    _131.position = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 0);
    _131.tangent = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 16);
    _131.normal0 = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 32);
    _131.normal1 = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 48);
    float2 _145 = float2(float(uint(gl_VertexIndex) % 2u), float(uint(gl_VertexIndex) / 2u)) * float2(1.0f, 0.100000001490116119384765625f);
    float _148 = _145.x;
    float3 _150 = lerp(_131.normal0.xyz, _131.normal1.xyz, _148.xxx);
    float _152 = _145.y;
    float3 _160 = (_131.position.xyz + (float3(_131.tangent.xy, 0.0f).xzy * _148)) + (_150 * (_152 * 0.5f));
    _160.y = lerp(_131.position.w, _131.position.y, 1.0f - pow(_152, 3.0f));
    float3 _172 = (_160 * float3(0.5f, 1.0f, 0.5f)) + (float3(0.0f, 3.0f, 0.0f) * (float(frame_constants_game_time_sec) + frame_constants_game_time_frac));
    int3 _174 = int3(floor(_172));
    float3 _175 = frac(_172);
    uint3 _176 = uint3(_174);
    uint _179 = _176.z;
    uint _181 = _179 + (_179 << 10u);
    uint _183 = _181 ^ (_181 >> 6u);
    uint _185 = _183 + (_183 << 3u);
    uint _187 = _185 ^ (_185 >> 11u);
    uint _193 = (((_176.y * 1664525u) + (_187 + (_187 << 15u))) + 1013904223u) * 1664525u;
    uint _195 = _193 ^ (_193 >> 11u);
    uint _198 = _195 ^ ((_195 << 7u) & 2636928640u);
    uint _201 = _198 ^ ((_198 << 15u) & 4022730752u);
    uint _207 = (((_176.x * 1664525u) + (_201 ^ (_201 >> 18u))) + 1013904223u) * 1664525u;
    uint _209 = _207 ^ (_207 >> 11u);
    uint _212 = _209 ^ ((_209 << 7u) & 2636928640u);
    uint _215 = _212 ^ ((_212 << 15u) & 4022730752u);
    uint _217 = _215 ^ (_215 >> 18u);
    uint _219 = _217 + (_217 << 10u);
    uint _221 = _219 ^ (_219 >> 6u);
    uint _223 = _221 + (_221 << 3u);
    uint _225 = _223 ^ (_223 >> 11u);
    uint _227 = _225 + (_225 << 15u);
    uint _233 = _227 + (_227 << 10u);
    uint _235 = _233 ^ (_233 >> 6u);
    uint _237 = _235 + (_235 << 3u);
    uint _239 = _237 ^ (_237 >> 11u);
    uint3 _252 = uint3(_174 + int3(1, 0, 0));
    uint _255 = _252.z;
    uint _257 = _255 + (_255 << 10u);
    uint _259 = _257 ^ (_257 >> 6u);
    uint _261 = _259 + (_259 << 3u);
    uint _263 = _261 ^ (_261 >> 11u);
    uint _269 = (((_252.y * 1664525u) + (_263 + (_263 << 15u))) + 1013904223u) * 1664525u;
    uint _271 = _269 ^ (_269 >> 11u);
    uint _274 = _271 ^ ((_271 << 7u) & 2636928640u);
    uint _277 = _274 ^ ((_274 << 15u) & 4022730752u);
    uint _283 = (((_252.x * 1664525u) + (_277 ^ (_277 >> 18u))) + 1013904223u) * 1664525u;
    uint _285 = _283 ^ (_283 >> 11u);
    uint _288 = _285 ^ ((_285 << 7u) & 2636928640u);
    uint _291 = _288 ^ ((_288 << 15u) & 4022730752u);
    uint _293 = _291 ^ (_291 >> 18u);
    uint _295 = _293 + (_293 << 10u);
    uint _297 = _295 ^ (_295 >> 6u);
    uint _299 = _297 + (_297 << 3u);
    uint _301 = _299 ^ (_299 >> 11u);
    uint _303 = _301 + (_301 << 15u);
    uint _309 = _303 + (_303 << 10u);
    uint _311 = _309 ^ (_309 >> 6u);
    uint _313 = _311 + (_311 << 3u);
    uint _315 = _313 ^ (_313 >> 11u);
    uint3 _328 = uint3(_174 + int3(0, 1, 0));
    uint _331 = _328.z;
    uint _333 = _331 + (_331 << 10u);
    uint _335 = _333 ^ (_333 >> 6u);
    uint _337 = _335 + (_335 << 3u);
    uint _339 = _337 ^ (_337 >> 11u);
    uint _345 = (((_328.y * 1664525u) + (_339 + (_339 << 15u))) + 1013904223u) * 1664525u;
    uint _347 = _345 ^ (_345 >> 11u);
    uint _350 = _347 ^ ((_347 << 7u) & 2636928640u);
    uint _353 = _350 ^ ((_350 << 15u) & 4022730752u);
    uint _359 = (((_328.x * 1664525u) + (_353 ^ (_353 >> 18u))) + 1013904223u) * 1664525u;
    uint _361 = _359 ^ (_359 >> 11u);
    uint _364 = _361 ^ ((_361 << 7u) & 2636928640u);
    uint _367 = _364 ^ ((_364 << 15u) & 4022730752u);
    uint _369 = _367 ^ (_367 >> 18u);
    uint _371 = _369 + (_369 << 10u);
    uint _373 = _371 ^ (_371 >> 6u);
    uint _375 = _373 + (_373 << 3u);
    uint _377 = _375 ^ (_375 >> 11u);
    uint _379 = _377 + (_377 << 15u);
    uint _385 = _379 + (_379 << 10u);
    uint _387 = _385 ^ (_385 >> 6u);
    uint _389 = _387 + (_387 << 3u);
    uint _391 = _389 ^ (_389 >> 11u);
    uint3 _404 = uint3(_174 + int3(1, 1, 0));
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
    uint _455 = _453 + (_453 << 15u);
    uint _461 = _455 + (_455 << 10u);
    uint _463 = _461 ^ (_461 >> 6u);
    uint _465 = _463 + (_463 << 3u);
    uint _467 = _465 ^ (_465 >> 11u);
    uint3 _480 = uint3(_174 + int3(0, 0, 1));
    uint _483 = _480.z;
    uint _485 = _483 + (_483 << 10u);
    uint _487 = _485 ^ (_485 >> 6u);
    uint _489 = _487 + (_487 << 3u);
    uint _491 = _489 ^ (_489 >> 11u);
    uint _497 = (((_480.y * 1664525u) + (_491 + (_491 << 15u))) + 1013904223u) * 1664525u;
    uint _499 = _497 ^ (_497 >> 11u);
    uint _502 = _499 ^ ((_499 << 7u) & 2636928640u);
    uint _505 = _502 ^ ((_502 << 15u) & 4022730752u);
    uint _511 = (((_480.x * 1664525u) + (_505 ^ (_505 >> 18u))) + 1013904223u) * 1664525u;
    uint _513 = _511 ^ (_511 >> 11u);
    uint _516 = _513 ^ ((_513 << 7u) & 2636928640u);
    uint _519 = _516 ^ ((_516 << 15u) & 4022730752u);
    uint _521 = _519 ^ (_519 >> 18u);
    uint _523 = _521 + (_521 << 10u);
    uint _525 = _523 ^ (_523 >> 6u);
    uint _527 = _525 + (_525 << 3u);
    uint _529 = _527 ^ (_527 >> 11u);
    uint _531 = _529 + (_529 << 15u);
    uint _537 = _531 + (_531 << 10u);
    uint _539 = _537 ^ (_537 >> 6u);
    uint _541 = _539 + (_539 << 3u);
    uint _543 = _541 ^ (_541 >> 11u);
    uint3 _556 = uint3(_174 + int3(1, 0, 1));
    uint _559 = _556.z;
    uint _561 = _559 + (_559 << 10u);
    uint _563 = _561 ^ (_561 >> 6u);
    uint _565 = _563 + (_563 << 3u);
    uint _567 = _565 ^ (_565 >> 11u);
    uint _573 = (((_556.y * 1664525u) + (_567 + (_567 << 15u))) + 1013904223u) * 1664525u;
    uint _575 = _573 ^ (_573 >> 11u);
    uint _578 = _575 ^ ((_575 << 7u) & 2636928640u);
    uint _581 = _578 ^ ((_578 << 15u) & 4022730752u);
    uint _587 = (((_556.x * 1664525u) + (_581 ^ (_581 >> 18u))) + 1013904223u) * 1664525u;
    uint _589 = _587 ^ (_587 >> 11u);
    uint _592 = _589 ^ ((_589 << 7u) & 2636928640u);
    uint _595 = _592 ^ ((_592 << 15u) & 4022730752u);
    uint _597 = _595 ^ (_595 >> 18u);
    uint _599 = _597 + (_597 << 10u);
    uint _601 = _599 ^ (_599 >> 6u);
    uint _603 = _601 + (_601 << 3u);
    uint _605 = _603 ^ (_603 >> 11u);
    uint _607 = _605 + (_605 << 15u);
    uint _613 = _607 + (_607 << 10u);
    uint _615 = _613 ^ (_613 >> 6u);
    uint _617 = _615 + (_615 << 3u);
    uint _619 = _617 ^ (_617 >> 11u);
    uint3 _632 = uint3(_174 + int3(0, 1, 1));
    uint _635 = _632.z;
    uint _637 = _635 + (_635 << 10u);
    uint _639 = _637 ^ (_637 >> 6u);
    uint _641 = _639 + (_639 << 3u);
    uint _643 = _641 ^ (_641 >> 11u);
    uint _649 = (((_632.y * 1664525u) + (_643 + (_643 << 15u))) + 1013904223u) * 1664525u;
    uint _651 = _649 ^ (_649 >> 11u);
    uint _654 = _651 ^ ((_651 << 7u) & 2636928640u);
    uint _657 = _654 ^ ((_654 << 15u) & 4022730752u);
    uint _663 = (((_632.x * 1664525u) + (_657 ^ (_657 >> 18u))) + 1013904223u) * 1664525u;
    uint _665 = _663 ^ (_663 >> 11u);
    uint _668 = _665 ^ ((_665 << 7u) & 2636928640u);
    uint _671 = _668 ^ ((_668 << 15u) & 4022730752u);
    uint _673 = _671 ^ (_671 >> 18u);
    uint _675 = _673 + (_673 << 10u);
    uint _677 = _675 ^ (_675 >> 6u);
    uint _679 = _677 + (_677 << 3u);
    uint _681 = _679 ^ (_679 >> 11u);
    uint _683 = _681 + (_681 << 15u);
    uint _689 = _683 + (_683 << 10u);
    uint _691 = _689 ^ (_689 >> 6u);
    uint _693 = _691 + (_691 << 3u);
    uint _695 = _693 ^ (_693 >> 11u);
    uint3 _708 = uint3(_174 + int3(1, 1, 1));
    uint _711 = _708.z;
    uint _713 = _711 + (_711 << 10u);
    uint _715 = _713 ^ (_713 >> 6u);
    uint _717 = _715 + (_715 << 3u);
    uint _719 = _717 ^ (_717 >> 11u);
    uint _725 = (((_708.y * 1664525u) + (_719 + (_719 << 15u))) + 1013904223u) * 1664525u;
    uint _727 = _725 ^ (_725 >> 11u);
    uint _730 = _727 ^ ((_727 << 7u) & 2636928640u);
    uint _733 = _730 ^ ((_730 << 15u) & 4022730752u);
    uint _739 = (((_708.x * 1664525u) + (_733 ^ (_733 >> 18u))) + 1013904223u) * 1664525u;
    uint _741 = _739 ^ (_739 >> 11u);
    uint _744 = _741 ^ ((_741 << 7u) & 2636928640u);
    uint _747 = _744 ^ ((_744 << 15u) & 4022730752u);
    uint _749 = _747 ^ (_747 >> 18u);
    uint _751 = _749 + (_749 << 10u);
    uint _753 = _751 ^ (_751 >> 6u);
    uint _755 = _753 + (_753 << 3u);
    uint _757 = _755 ^ (_755 >> 11u);
    uint _759 = _757 + (_757 << 15u);
    uint _765 = _759 + (_759 << 10u);
    uint _767 = _765 ^ (_765 >> 6u);
    uint _769 = _767 + (_767 << 3u);
    uint _771 = _769 ^ (_769 >> 11u);
    float3 _786 = (_175 * _175) * (3.0f.xxx - (_175 * 2.0f));
    float _787 = _786.x;
    float _788 = 1.0f - _787;
    float _790 = _786.y;
    float _791 = 1.0f - _790;
    float _793 = _786.z;
    float _794 = 1.0f - _793;
    float3 _807 = (((((float3(asfloat((_217 & 8388607u) | 1065353216u) - 1.0f, asfloat((_227 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_239 + (_239 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _788) * _791) * _794) + (((float3(asfloat((_293 & 8388607u) | 1065353216u) - 1.0f, asfloat((_303 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_315 + (_315 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _787) * _791) * _794)) + (((float3(asfloat((_369 & 8388607u) | 1065353216u) - 1.0f, asfloat((_379 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_391 + (_391 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _788) * _790) * _794)) + (((float3(asfloat((_445 & 8388607u) | 1065353216u) - 1.0f, asfloat((_455 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_467 + (_467 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _787) * _790) * _794);
    float3 _823 = (((_807 + (((float3(asfloat((_521 & 8388607u) | 1065353216u) - 1.0f, asfloat((_531 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_543 + (_543 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _788) * _791) * _793)) + (((float3(asfloat((_597 & 8388607u) | 1065353216u) - 1.0f, asfloat((_607 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_619 + (_619 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _787) * _791) * _793)) + (((float3(asfloat((_673 & 8388607u) | 1065353216u) - 1.0f, asfloat((_683 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_695 + (_695 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _788) * _790) * _793)) + (((float3(asfloat((_749 & 8388607u) | 1065353216u) - 1.0f, asfloat((_759 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_771 + (_771 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _787) * _790) * _793);
    float3 _828 = _160 + ((((_823 - 0.5f.xxx) * float3(1.0f, 0.0f, 1.0f)) * _152) * 0.20000000298023223876953125f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_828, 1.0f));
    out_var_C0 = normalize((_150 * (-((_131.position.y - _131.position.w) * ((-3.0f) * pow(_152, 2.0f))))) + float3(0.0f, 0.5f, 0.0f));
    out_var_C1 = _828;
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

struct RippleSourceSsbo
{
    uint4 params[32];
    uint count;
    uint pad0;
    uint pad1;
    uint pad2;
};

static uint _186;
static float _187;

static const uint2 _191[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };
static const float _192[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _193[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _194[4] = { 17u, 18u, 20u, 21u };

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
    uint history_valid : packoffset(c0.y);
    uint2 terrain_lighting_texture_dims : packoffset(c0.z);
    uint2 blue_noise_texture_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 brdf_fg_lut_dims : packoffset(c2.z);
    uint rt_bvh_node_count : packoffset(c3);
    uint rt_triangle_count : packoffset(c3.y);
    uint2 water_noise_texture_dims : packoffset(c3.z);
    uint2 framebuffer_dims : packoffset(c4);
    uint2 water_mask_dims : packoffset(c4.z);
    uint2 half_depth_tex_dims : packoffset(c5);
    uint2 refraction_tex_dims : packoffset(c5.z);
    uint2 taa_history_tex_dims : packoffset(c6);
    uint2 reprojection_tex_dims : packoffset(c6.z);
    float3 water_color : packoffset(c7);
};

ByteAddressBuffer ripple_source_buffer : register(t26, space0);
RWByteAddressBuffer __alias_water_rt_count_buffer : register(u34, space0);
RWByteAddressBuffer __alias_water_rt_ray_buffer : register(u35, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> water_noise_texture : register(t27, space0);
Texture2D<float4> half_depth_tex : register(t29, space0);

static float4 gl_FragCoord;
static float3 in_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    float4 gl_FragCoord : SV_Position;
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

void frag_main()
{
    bool _214 = IsHelperLane();
    uint2 _218 = uint2(gl_FragCoord.xy);
    float3 _534;
    do
    {
        bool _224 = frame_constants_glade_theme_id == 4u;
        if (_224)
        {
            _534 = float3(0.0f, 1.0f, 0.0f);
            break;
        }
        float3 _426;
        do
        {
            if (_224)
            {
                _426 = 0.0f.xxx;
                break;
            }
            float _236 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
            float2 _213[4];
            uint _238 = 0u;
            uint _241 = 0u;
            uint _243 = 0u;
            bool _245 = false;
            bool _247 = false;
            uint _249 = 0u;
            uint _251 = 0u;
            bool _253 = false;
            bool _255 = false;
            for (; _238 < 4u; )
            {
                float _261 = float(_194[_238]);
                int _263 = int(floor(_261));
                float _264 = frac(_261);
                uint _265 = uint(_263);
                uint _267 = _265 + (_265 << 10u);
                uint _269 = _267 ^ (_267 >> 6u);
                uint _271 = _269 + (_269 << 3u);
                uint _273 = _271 ^ (_271 >> 11u);
                bool _276 = _255 ? _255 : true;
                uint _277 = _255 ? _251 : 1664525u;
                bool _278 = _253 ? _253 : true;
                uint _279 = _253 ? _249 : 1013904223u;
                uint _283 = (((_265 * _277) + (_273 + (_273 << 15u))) + _279) * _277;
                uint _285 = _283 ^ (_283 >> 11u);
                uint _288 = _285 ^ ((_285 << 7u) & 2636928640u);
                uint _291 = _288 ^ ((_288 << 15u) & 4022730752u);
                uint _293 = _291 ^ (_291 >> 18u);
                uint _295 = _293 + (_293 << 10u);
                uint _297 = _295 ^ (_295 >> 6u);
                uint _299 = _297 + (_297 << 3u);
                uint _301 = _299 ^ (_299 >> 11u);
                bool _304 = _247 ? _247 : true;
                uint _305 = _247 ? _243 : 8388607u;
                bool _306 = _245 ? _245 : true;
                uint _307 = _245 ? _241 : 1065353216u;
                bool _312 = _304 ? _304 : true;
                uint _313 = _304 ? _305 : 8388607u;
                bool _314 = _306 ? _306 : true;
                uint _315 = _306 ? _307 : 1065353216u;
                uint _322 = uint(_263 + 1);
                uint _324 = _322 + (_322 << 10u);
                uint _326 = _324 ^ (_324 >> 6u);
                uint _328 = _326 + (_326 << 3u);
                uint _330 = _328 ^ (_328 >> 11u);
                uint _252 = _276 ? _277 : 1664525u;
                uint _250 = _278 ? _279 : 1013904223u;
                uint _336 = (((_322 * _252) + (_330 + (_330 << 15u))) + _250) * _252;
                uint _338 = _336 ^ (_336 >> 11u);
                uint _341 = _338 ^ ((_338 << 7u) & 2636928640u);
                uint _344 = _341 ^ ((_341 << 15u) & 4022730752u);
                uint _346 = _344 ^ (_344 >> 18u);
                uint _348 = _346 + (_346 << 10u);
                uint _350 = _348 ^ (_348 >> 6u);
                uint _352 = _350 + (_350 << 3u);
                uint _354 = _352 ^ (_352 >> 11u);
                bool _357 = _312 ? _312 : true;
                uint _358 = _312 ? _313 : 8388607u;
                bool _359 = _314 ? _314 : true;
                uint _360 = _314 ? _315 : 1065353216u;
                uint _244 = _357 ? _358 : 8388607u;
                uint _242 = _359 ? _360 : 1065353216u;
                float _373 = (_264 * _264) * (3.0f - (2.0f * _264));
                _213[_238] = (normalize(((float2(asfloat((_293 & _305) | _307) - 1.0f, asfloat(((_301 + (_301 << 15u)) & _313) | _315) - 1.0f) * (1.0f - _373)) + (float2(asfloat((_346 & _358) | _360) - 1.0f, asfloat(((_354 + (_354 << 15u)) & _244) | _242) - 1.0f) * _373)) - 0.5f.xx) * _236) * 3.0f;
                _238++;
                _241 = _242;
                _243 = _244;
                _245 = _359 ? _359 : true;
                _247 = _357 ? _357 : true;
                _249 = _250;
                _251 = _252;
                _253 = _278 ? _278 : true;
                _255 = _276 ? _276 : true;
                continue;
            }
            float3 _384;
            _384 = 0.0f.xxx;
            for (uint _387 = 0u; _387 < 4u; )
            {
                uint _388 = _387 + 1u;
                float _392 = float(_388) * 10.16640758514404296875f;
                float _393 = cos(_392);
                float _394 = sin(_392);
                float2x2 _398 = float2x2(float2(_393, _394), float2(-_394, _393));
                float2 _425 = _384.yz + mul(((((water_noise_texture.Sample(sampler_llr, mul(_398, (in_var_C1.xz * _192[_387]) + _213[_387]) * 0.01200000010430812835693359375f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _193[_387]) * float2(1.0f, _192[_387] * 0.01200000010430812835693359375f).xyy).yz, _398);
                _384 = float3(_384.x, _425.x, _425.y);
                _387 = _388;
                continue;
            }
            _426 = _384;
            break;
        } while(false);
        float3 _519;
        do
        {
            if (_224)
            {
                _519 = 0.0f.xxx;
                break;
            }
            uint _433 = min(32u, ripple_source_buffer.Load<uint>(512));
            float3 _435;
            _435 = 0.0f.xxx;
            for (uint _438 = 0u; _438 < _433; )
            {
                float2 _452 = float2(spvUnpackHalf2x16(ripple_source_buffer.Load<uint4>(_438 * 16 + 0).x & 65535u).x, spvUnpackHalf2x16((ripple_source_buffer.Load<uint4>(_438 * 16 + 0).x >> 16u) & 65535u).x);
                float2 _461 = float2(spvUnpackHalf2x16(ripple_source_buffer.Load<uint4>(_438 * 16 + 0).y & 65535u).x, spvUnpackHalf2x16((ripple_source_buffer.Load<uint4>(_438 * 16 + 0).y >> 16u) & 65535u).x);
                float2 _464 = lerp(_452, _461, 0.5f.xx);
                float2 _465 = _464 - in_var_C1.xz;
                float _466 = length(_465);
                float2 _468 = (_461 - _452) * 3.0f;
                float _470 = _466 + dot(_465, _468);
                float _478 = (float(frame_constants_game_time_sec) + frame_constants_game_time_frac) * 6.0f;
                float _481 = max(_466, _470);
                float _482 = _481 * _481;
                float _485 = sin((_470 * 16.3649997711181640625f) - _478) / (1.0f + (_482 * _482));
                float2 _487 = _464 - (in_var_C1.xz - float2(0.00999999977648258209228515625f, 0.0f));
                float _488 = length(_487);
                float _490 = _488 + dot(_487, _468);
                float _494 = max(_488, _490);
                float _495 = _494 * _494;
                float2 _501 = _464 - (in_var_C1.xz - float2(0.0f, 0.00999999977648258209228515625f));
                float _502 = length(_501);
                float _504 = _502 + dot(_501, _468);
                float _508 = max(_502, _504);
                float _509 = _508 * _508;
                float3 _514 = float3(_485, (sin((_490 * 16.3649997711181640625f) - _478) / (1.0f + (_495 * _495))) - _485, (sin((_504 * 16.3649997711181640625f) - _478) / (1.0f + (_509 * _509))) - _485);
                float2 _516 = _514.yz * 0.5f;
                _435 += (float3(_514.x, _516.x, _516.y) * asfloat(ripple_source_buffer.Load<uint4>(_438 * 16 + 0).z));
                _438++;
                continue;
            }
            _519 = _435;
            break;
        } while(false);
        float3 _522 = float3(_426.y, 1.0f, _426.z);
        float2 _531 = _522.xz + (float3(_519.y, 1.0f, _519.z).xz / abs(1.0f).xx);
        _534 = normalize(float3(_531.x, _522.y, _531.y));
        break;
    } while(false);
    float2 _555 = (((float2(int2((_218 + uint2(1u, 1u)) - _191[frame_constants_frame_index & 3u])) + 0.5f.xx) / float2(framebuffer_dims)) - 0.5f.xx) * float2(2.0f, -2.0f);
    float _556 = _555.x;
    float4 _561 = float4(_556, _555.y, gl_FragCoord.z, 1.0f);
    float4 _563 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, _561));
    float _571 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float _581 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_218.x) + _571)) + (0.005837149918079376220703125f * (float(_218.y) + _571))));
    float3 _583 = -reflect(-normalize(mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_556, _555.y, 0.0f, 1.0f))).xyz), _534);
    float _584 = _583.y;
    float3 _590;
    if (_584 < 0.0f)
    {
        float3 _589 = _583;
        _589.y = -_584;
        _590 = _589;
    }
    else
    {
        _590 = _583;
    }
    float3 _595 = _561.xyz;
    float4 _606 = _561 + mul(frame_constants_view_constants[0u].view_to_clip, mul(frame_constants_view_constants[0u].world_to_view, float4(_590, 0.0f)));
    float _607 = _606.w;
    float3 _616 = (_606 / (float((_607 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_607))).xxxx).xyz;
    float3 _617 = _616 - _595;
    bool3 _618 = bool3(_617.x < 0.0f.xxx.x, _617.y < 0.0f.xxx.y, _617.z < 0.0f.xxx.z);
    float3 _621 = (float3(_618.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _618.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _618.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _595) / _617;
    float3 _627 = _595 + (_617 * max(0.0f, max(_621.x, _621.y)));
    float3 _628 = _616 - _627;
    bool3 _629 = bool3(_628.x >= 0.0f.xxx.x, _628.y >= 0.0f.xxx.y, _628.z >= 0.0f.xxx.z);
    float3 _632 = (float3(_629.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _629.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _629.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _627) / _628;
    float3 _639 = _627 + (_628 * min(min(_632.x, _632.y), _632.z));
    bool _836;
    float _837;
    do
    {
        uint _660 = uint(int(max(2u, min(8u, uint(int(floor(length((((_639.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((_627.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * float2(half_depth_tex_dims)))))))));
        float _753;
        float _821;
        float _822;
        float _823;
        float _824;
        bool _825;
        do
        {
            float3 _665 = _639 - _627;
            float _673;
            if (_639.z > 9.5367431640625e-07f)
            {
                _673 = float(_660);
            }
            else
            {
                _673 = max(1.0f, float(_660) - 1.0f);
            }
            float _674 = 1.0f / _673;
            float _747;
            float _748;
            float _749;
            float _750;
            float _751;
            bool _752;
            if (_660 > 0u)
            {
                float _680 = lerp(0.0f, 1.0f, pow(_581 * _674, 1.0f));
                float3 _682 = _627 + (_665 * _680);
                float _687 = 1.0f / _682.z;
                float4 _690 = half_depth_tex.SampleLevel(sampler_nnc, (_682.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float _691 = _690.x;
                float _694 = (1.00000202655792236328125f / _691) - _687;
                bool _697 = (_694 < 0.0f) ? true : false;
                float _740;
                float _741;
                float _742;
                float _743;
                float _744;
                bool _745;
                float _746;
                if (_697)
                {
                    _740 = 0.0f;
                    _741 = _694;
                    _742 = _687 - (1.0f / _691);
                    _743 = frame_constants_view_constants[0u].clip_to_view[3].z;
                    _744 = _680;
                    _745 = _697;
                    _746 = 0.0f;
                }
                else
                {
                    float _705;
                    float _709;
                    _705 = _694;
                    _709 = _680;
                    float _706;
                    bool _708;
                    float _710;
                    float _735;
                    float _736;
                    float _737;
                    float _738;
                    bool _739;
                    uint _702 = 1u;
                    bool _707 = _697;
                    for (;;)
                    {
                        if (_702 < _660)
                        {
                            _710 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_702) + _581) * _674, 1.0f)));
                            float3 _720 = _627 + (_665 * _710);
                            float _725 = 1.0f / _720.z;
                            float4 _727 = half_depth_tex.SampleLevel(sampler_nnc, (_720.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                            float _728 = _727.x;
                            _706 = (1.00000202655792236328125f / _728) - _725;
                            _708 = (_706 < 0.0f) ? true : false;
                            if (_708)
                            {
                                _735 = _706;
                                _736 = _725 - (1.0f / _728);
                                _737 = frame_constants_view_constants[0u].clip_to_view[3].z;
                                _738 = _710;
                                _739 = _708;
                                break;
                            }
                            _702++;
                            _705 = _706;
                            _707 = _708;
                            _709 = _710;
                            continue;
                        }
                        else
                        {
                            _735 = 0.0f;
                            _736 = 0.0f;
                            _737 = 0.0f;
                            _738 = 1.0f;
                            _739 = _707;
                            break;
                        }
                    }
                    _740 = _705;
                    _741 = _735;
                    _742 = _736;
                    _743 = _737;
                    _744 = _738;
                    _745 = _739;
                    _746 = _709;
                }
                _747 = _740;
                _748 = _741;
                _749 = _742;
                _750 = _743;
                _751 = _744;
                _752 = _745;
                _753 = _746;
            }
            else
            {
                _747 = 0.0f;
                _748 = 0.0f;
                _749 = 0.0f;
                _750 = 0.0f;
                _751 = 1.0f;
                _752 = false;
                _753 = 0.0f;
            }
            if (_752)
            {
                float _758;
                float _761;
                float _763;
                float _765;
                float _767;
                float _769;
                _758 = _753;
                _761 = _747;
                _763 = _748;
                _765 = _749;
                _767 = _750;
                _769 = _751;
                for (uint _771 = 0u; _771 < 3u; )
                {
                    float _775 = _758 + _769;
                    float _776 = _775 * 0.5f;
                    float3 _778 = _627 + (_665 * _776);
                    float _783 = 1.0f / _778.z;
                    float4 _786 = half_depth_tex.SampleLevel(sampler_nnc, (_778.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float _787 = _786.x;
                    float _790 = (1.00000202655792236328125f / _787) - _783;
                    bool _793 = (_790 < 0.0f) ? true : false;
                    _758 = _793 ? _758 : _776;
                    _761 = _793 ? _761 : _790;
                    _763 = _793 ? _790 : _763;
                    _765 = _793 ? (_783 - (1.0f / _787)) : _765;
                    _767 = _793 ? frame_constants_view_constants[0u].clip_to_view[3].z : _767;
                    _769 = _793 ? _776 : _769;
                    _771++;
                    continue;
                }
                float _797 = lerp(_758, _769, _761 / (_761 + (-_763)));
                float3 _799 = _627 + (_665 * _797);
                float _804 = 1.0f / _799.z;
                float4 _807 = half_depth_tex.SampleLevel(sampler_nnc, (_799.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float _808 = _807.x;
                float _811 = (1.00000202655792236328125f / _808) - _804;
                bool _816 = (abs(_811) < (_761 * 0.89999997615814208984375f)) ? true : false;
                _821 = _816 ? _797 : _769;
                _822 = _816 ? _811 : _763;
                _823 = _816 ? frame_constants_view_constants[0u].clip_to_view[3].z : _767;
                _824 = _816 ? (_804 - (1.0f / _808)) : _765;
                _825 = true;
                break;
            }
            else
            {
                _821 = _753;
                _822 = _187;
                _823 = _187;
                _824 = _187;
                _825 = false;
                break;
            }
            break; 
        } while(false);
        bool _833;
        if (_825)
        {
            bool _832;
            if (_824 < _823)
            {
                _832 = _822 < _823;
            }
            else
            {
                _832 = false;
            }
            _833 = _832;
        }
        else
        {
            _833 = false;
        }
        if (_833)
        {
            _836 = true;
            _837 = _821;
            break;
        }
        _836 = false;
        _837 = _753;
        break;
    } while(false);
    float _838 = QuadReadAcrossX(_837);
    float _840 = QuadReadAcrossX(float(_836));
    bool _846 = (int(_840 != 0.0f) & int(!_836)) != 0;
    bool _847 = _846 ? true : _836;
    float _848 = _846 ? _838 : _837;
    float _849 = QuadReadAcrossY(_848);
    float _851 = QuadReadAcrossY(float(_847));
    bool _857 = (int(_851 != 0.0f) & int(!_847)) != 0;
    if (_857 ? true : _847)
    {
    }
    else
    {
        float3 _864 = lerp(_627, _639, (_857 ? _849 : _848).xxx);
        float4 _870 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_864, 1.0f)));
        uint _885;
        if (_214)
        {
            _885 = 0u;
        }
        else
        {
            uint2 _878 = _218 & uint2(1u, 1u);
            _885 = 1u << ((_878.x + (_878.y * 2u)) & 31u);
        }
        uint _886 = QuadReadAcrossX(_885);
        uint _887 = _885 + _886;
        uint _888 = QuadReadAcrossY(_887);
        uint _889 = _887 + _888;
        uint2 _890 = _218 & uint2(1u, 1u);
        if ((_890.x + (_890.y * 2u)) == firstbithigh(_889))
        {
            uint _900;
            __alias_water_rt_count_buffer.InterlockedAdd(0, 1u, _900);
            uint2 _901 = _218 / uint2(2u, 2u);
            float3 _915 = _534 / ((abs(_534.x) + abs(_534.y)) + abs(_534.z)).xxx;
            float3 _929;
            if (_915.z < 0.0f)
            {
                float2 _927 = (1.0f.xx - abs(_915.yx)) * ((step(0.0f.xx, _915.xy) * 2.0f) - 1.0f.xx);
                _929 = float3(_927.x, _927.y, _915.z);
            }
            else
            {
                _929 = _915;
            }
            float2 _932 = (_929.xy * 0.5f) + 0.5f.xx;
            __alias_water_rt_ray_buffer.Store<uint4>(_900 * 16 + 0, uint4(_901.x + (_901.y * 65536u), spvPackHalf2x16(float2(_932.x, 0.0f)) | (spvPackHalf2x16(float2(_932.y, 0.0f)) << 16u), asuint(dot(_590, (_870.xyz / _870.w.xxx) - (_563.xyz / _563.w.xxx))), (asuint(gl_FragCoord.z) & 4294967280u) + _889));
        }
    }
}

[earlydepthstencil]
void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
