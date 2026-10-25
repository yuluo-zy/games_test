// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/grass_lod1.raster.hlsl
// Evidence: _grass_lod1.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin (SPIR-V).
//   Bindings identical to grass.raster. vs_main builds a 3-vertex LOD1 blade triangle
//   (blades indexed from the END of the buffer: (grass_buffer_size-1) - vertex/3), same
//   terrain-normal/wind/bend machinery; ps_main matches grass.raster ps except the discard
//   lerp uses a constant -0.8 (no C8). Includes grass.raster.hlsl with GRASS_LOD1 set so
//   the shared declarations/helpers are reused and the LOD0 entry points are excluded.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#define GRASS_LOD1
#include "grass.raster.hlsl"

struct LOD1_VS_OUT
{
    float3 C0 : TEXCOORD0;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float3 C5 : TEXCOORD5;
    float3 C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    float3 C9 : TEXCOORD9;
    float4 position : SV_Position;
};

LOD1_VS_OUT vs_main(uint vertex_id : SV_VertexID)
{
    LOD1_VS_OUT o;

    float2 st;
    uint tri = vertex_id % 3u;
    if (tri == 0u)
    {
        st = float2(-1.0f, 1.1f);
    }
    else if (tri == 1u)
    {
        st = float2(0.0f, 1.0f);
    }
    else
    {
        st = float2(1.0f, 1.0f);
    }
    float3 blade_geom = float3(st.x * 0.135f, st.y, 0.0f);

    uint blade_index = (grass_buffer_size - 1u) - (vertex_id / 3u);
    GrassBladeDataPacked blade = grass_blade_buffer_data[blade_index];
    float3 blade_pos = blade.position;
    float3 albedo = blade_color_rgb(blade);
    float3 albedo2 = albedo * albedo;
    float2 widths = grass_blade_widths(blade);
    float width_bottom = widths.x;

    float4 gathered = terrain_heightmap.GatherRed(sampler_default, (blade_pos.xz * (1.0f / 130.0f)) + 0.5f, int2(0, 0));
    float2 grad = ((float2((gathered.z + gathered.y) - (gathered.w + gathered.x),
                           (gathered.x + gathered.y) - (gathered.w + gathered.z)) * 0.5f) * 26.5f)
        / (130.0f / float2(terrain_heightmap_dims));
    float bend_amt = width_bottom * 0.2f;
    float3 blade_scale = float3(min(1.0f, width_bottom * 0.8f) * 0.36f, bend_amt, 1.0f) * 2.5f;

    float dist_scale = max(30.0f / (((-1.0f) / mul(view_constants[0].world_to_view, float4(blade_pos, 1.0f)).z)
        * ((0.5f * view_constants[0].view_to_clip[1].y) * float(clamp(inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float bend_offset = blade_pos.y - ((dist_scale - 1.0f) * blade_scale.y);
    float3 ground_pos = blade_pos;
    ground_pos.y = bend_offset;
    float wind_scale = blade_scale.y * dist_scale;

    float3 axis = normalize(float3((float((blade.packed0 >> 24u) & 255u) * (2.0f / 255.0f)) - 1.0f, 0.0f, grass_blade_bend_xz(blade)));
    float4x4 basis = transpose(float4x4(float4(normalize(cross(axis, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                         float4(0.0f, 1.0f, 0.0f, 0.0f),
                                         float4(axis, 0.0f),
                                         float4(0.0f, 0.0f, 0.0f, 1.0f)));

    float4 cam_pos = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float h_clamped = clamp(st.y, 0.0f, 1.0f);
    float curve = 3.0f * ((0.03f * widths.y) * lerp(1.0f, 2.0f, abs(normalize((cam_pos.xyz / cam_pos.w) - ground_pos).y)));
    float curve_factor = curve * (h_clamped * h_clamped);
    blade_geom.z = curve_factor;

    float3 blade_geom2;
    if (glade_theme_id == 4u)
    {
        blade_geom2 = blade_geom;
        blade_geom2.x = st.x * 0.0675f;
    }
    else
    {
        blade_geom2 = blade_geom;
    }

    // Twist around the blade axis by the packed bend angle (Rodrigues around X).
    float bend_angle = float((blade.packed0 >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float sina = sin(bend_angle);
    float cosa = cos(bend_angle);
    float oc = 1.0f - cosa;
    float axis_x = 1.0f;
    float axis_y = 0.0f;
    float axis_z = 0.0f;
    float t_oc_x = oc * axis_x;
    float t_oc_xy = t_oc_x * axis_y;
    float t_z_sin = axis_z * sina;
    float t_oc_z = oc * axis_z;
    float t_oc_zx = t_oc_z * axis_x;
    float t_y_sin = axis_y * sina;
    float t_oc_y = oc * axis_y;
    float t_oc_yz = t_oc_y * axis_z;
    float t_x_sin = axis_x * sina;
    float4x4 twist = transpose(float4x4(float4((t_oc_x * axis_x) + cosa, t_oc_xy - t_z_sin, t_oc_zx + t_y_sin, 0.0f),
                                        float4(t_oc_xy + t_z_sin, (t_oc_y * axis_y) + cosa, t_oc_yz - t_x_sin, 0.0f),
                                        float4(t_oc_zx - t_y_sin, t_oc_yz + t_x_sin, (t_oc_z * axis_z) + cosa, 0.0f),
                                        float4(0.0f, 0.0f, 0.0f, 1.0f)));

    float3 blade_normal = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 1e-5f * wind_scale, (curve * ((h_clamped + 0.0001f) * (h_clamped + 0.0001f))) - curve_factor)));
    blade_normal.x += 0.3f * st.x;

    float4 rotated = mul(basis, mul(twist, float4(normalize(blade_normal), 0.0f)));
    float3 rotated_xyz = rotated.xyz;
    float4 world_offset = mul(basis, mul(twist, mul(float4x4(float4(blade_scale.x * (float((blade.packed0 >> 8u) & 255u) * (1.0f / 255.0f)) * dist_scale, 0.0f, 0.0f, 0.0f),
                                                          float4(0.0f, wind_scale, 0.0f, 0.0f),
                                                          float4(0.0f, 0.0f, blade_scale.z, 0.0f),
                                                          float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(blade_geom2, 1.0f)))) + float4(blade_pos.x, bend_offset, blade_pos.z, 0.0f);

    // Wind value-noise (inline blend grouping).
    float2 wind_uv = (ground_pos.xz * 0.1f) + (float2(time, 0.0f) * 1.2f);
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    float2 wind_smooth = wind_frac * wind_frac * (3.0f - (2.0f * wind_frac));
    float wind_fx = wind_smooth.x;
    float wind_fy = wind_smooth.y;
    float wind_n00 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 0))));
    float wind_n10 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 0))));
    float wind_n01 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 1))));
    float wind_n11 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 1))));
    float wind_noise = ((1.0f - wind_fy) * ((wind_n00 * (1.0f - wind_fx)) + (wind_n10 * wind_fx)))
        + ((wind_n01 * (1.0f - wind_fx)) * wind_fy)
        + ((wind_n11 * wind_fx) * wind_fy);
    float wind = (smoothstep(-0.2f, 0.5f, bend_amt) * (st.y * st.y))
        * (sin((wind_noise * 3.14f) - 1.57f + 0.3f)) * 0.7f * grass_blade_bend_xz(blade);
    rotated_xyz.y -= wind * 0.7f;
    world_offset.x -= wind * 0.25f;

    o.position = mul(view_constants[current_view_index].world_to_sample, world_offset);
    o.C0 = lerp(albedo2 * 0.8f, albedo2, (st.y * st.y).xxx);
    o.C2 = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));
    o.C3 = world_offset.xyz;
    o.C4 = ground_pos;
    o.C5 = normalize(rotated_xyz);
    o.C6 = axis;
    o.C7 = st;
    o.C8 = 1.0f;
    o.C9 = float3(st * float2(st.y, 1.0f), st.y);
    return o;
}

struct LOD1_PS_IN
{
    float3 C0 : TEXCOORD0;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C5 : TEXCOORD5;
    float2 C7 : TEXCOORD7;
    float3 C9 : TEXCOORD9;
};

struct LOD1_PS_OUT
{
    uint2 target : SV_Target0;
};

LOD1_PS_OUT ps_main(LOD1_PS_IN i)
{
    LOD1_PS_OUT o;
    float4 cam_pos = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (abs(i.C9.x / i.C9.z) < lerp(-0.8f, 1.0f, i.C7.y))
    {
        discard;
    }
    float3 view_normal;
    if (dot(normalize((cam_pos.xyz / cam_pos.w) - i.C3), i.C5) <= 0.0f)
    {
        view_normal = -i.C5;
    }
    else
    {
        view_normal = i.C5;
    }
    float3 n = normalize(view_normal);
    float3 albedo = (i.C0 * 0.55f) * lerp(0.7f, 1.1f, i.C7.y);
    float3 albedo_out;
    if (glade_theme_id == 4u)
    {
        albedo_out = lerp(0.9f.xxx, albedo * 0.5f, pow(i.C7.y, 0.5f).xxx);
    }
    else
    {
        albedo_out = albedo;
    }
    float3 octa = n / ((abs(n.x) + abs(n.y)) + abs(n.z));
    if (octa.z < 0.0f)
    {
        float2 xy = (1.0f - abs(octa.yx)) * ((step(0.0f, octa.xy) * 2.0f) - 1.0f);
        octa = float3(xy.x, xy.y, octa.z);
    }
    float2 octa_uv = (octa.xy * 0.5f) + 0.5f;
    float3 packed_n = (normalize(i.C2) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(albedo_out, 0.0f, 1.0f));
    o.target = uint2(pack_normal8(packed_n) + (uint(clamp(octa_uv.x, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u),
                     pack_normal8(albedo_sqrt) + (uint(clamp(octa_uv.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u));
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _grass_lod1.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct GrassBladeDataPacked
{
    float3 position;
    uint color;
    uint packed0;
    uint packed1;
    uint packed2;
    uint pad;
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
    float time : packoffset(c0.y);
    uint2 terrain_heightmap_dims : packoffset(c0.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint grass_buffer_size : packoffset(c1.z);
};

ByteAddressBuffer grass_blade_buffer_data : register(t17, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 out_var_C0;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;
static float3 out_var_C5;
static float3 out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static float3 out_var_C9;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float3 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float3 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3 out_var_C9 : TEXCOORD9;
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
    uint _168 = uint(gl_VertexIndex) % 3u;
    float2 _176;
    if (_168 == 0u)
    {
        _176 = float2(-1.0f, 1.10000002384185791015625f);
    }
    else
    {
        bool2 _174 = (_168 == 1u).xx;
        _176 = float2(_174.x ? 0.0f.xx.x : 1.0f.xx.x, _174.y ? 0.0f.xx.y : 1.0f.xx.y);
    }
    float3 _180 = float3(_176.x * 0.13500000536441802978515625f, _176.y, 0.0f);
    uint _185 = (grass_buffer_size - 1u) - (uint(gl_VertexIndex) / 3u);
    float3 _187 = grass_blade_buffer_data.Load<float3>(_185 * 32 + 0);
    float3 _201 = float3(float(grass_blade_buffer_data.Load<uint>(_185 * 32 + 12) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 12) >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 12) >> 16u) & 255u) * 0.0039215688593685626983642578125f);
    float3 _202 = _201 * _201;
    float2 _220 = spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(_185 * 32 + 20) & 65535u);
    float _221 = _220.x;
    float4 _241 = terrain_heightmap.GatherRed(sampler_default, (_187.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, int2(0, 0));
    float _246 = _241.z;
    float _247 = _241.y;
    float _249 = _241.w;
    float _250 = _241.x;
    float2 _259 = ((float2((_246 + _247) - (_249 + _250), (_250 + _247) - (_249 + _246)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
    float _270 = _221 * 0.20000000298023223876953125f;
    float3 _272 = float3(min(1.0f, _221 * 0.800000011920928955078125f) * 0.3599999845027923583984375f, _270, 1.0f) * 2.5f;
    float _295 = max(30.0f / (((-1.0f) / mul(frame_constants_view_constants[0u].world_to_view, float4(_187, 1.0f)).z) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float _298 = _272.y;
    float _300 = _187.y - ((_295 - 1.0f) * _298);
    float3 _301 = _187;
    _301.y = _300;
    float _302 = _298 * _295;
    float3 _308 = float3((float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 16) >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f, spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(_185 * 32 + 24) >> 16u) & 65535u).x);
    float3 _309 = normalize(_308);
    float4x4 _321 = transpose(float4x4(float4(normalize(cross(_309, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(_309, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4 _327 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float _338 = clamp(_176.y, 0.0f, 1.0f);
    float _339 = 3.0f * ((0.02999999932944774627685546875f * spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(_185 * 32 + 20) >> 16u) & 65535u).x) * lerp(1.0f, 2.0f, abs(normalize((_327.xyz / _327.w.xxx) - _301).y)));
    float _341 = _339 * (_338 * _338);
    float _342 = _338 + 0.00010001659393310546875f;
    _180.z = _341;
    float3 _354;
    if (frame_constants_glade_theme_id == 4u)
    {
        float3 _353 = _180;
        _353.x = _176.x * 0.067500002682209014892578125f;
        _354 = _353;
    }
    else
    {
        _354 = _180;
    }
    float _355 = float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 16) >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float3 _356 = normalize(float3(1.0f, 0.0f, 0.0f));
    float _357 = sin(_355);
    float _358 = cos(_355);
    float _359 = 1.0f - _358;
    float _360 = _356.x;
    float _361 = _359 * _360;
    float _364 = _356.y;
    float _365 = _361 * _364;
    float _366 = _356.z;
    float _367 = _366 * _357;
    float _369 = _359 * _366;
    float _370 = _369 * _360;
    float _371 = _364 * _357;
    float _374 = _359 * _364;
    float _377 = _374 * _366;
    float _378 = _360 * _357;
    float4x4 _388 = transpose(float4x4(float4((_361 * _360) + _358, _365 - _367, _370 + _371, 0.0f), float4(_365 + _367, (_374 * _364) + _358, _377 - _378, 0.0f), float4(_370 - _371, _377 + _378, (_369 * _366) + _358, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 _392 = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 9.9999997473787516355514526367188e-05f * _302, (_339 * (_342 * _342)) - _341)));
    _392.x = _392.x + (0.300000011920928955078125f * _176.x);
    float4 _403 = mul(_321, mul(_388, float4(normalize(_392), 0.0f)));
    float3 _404 = _403.xyz;
    float4 _413 = mul(_321, mul(_388, mul(float4x4(float4((_272.x * (float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 16) >> 8u) & 255u) * 0.0039215688593685626983642578125f)) * _295, 0.0f, 0.0f, 0.0f), float4(0.0f, _302, 0.0f, 0.0f), float4(0.0f, 0.0f, _272.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(_354, 1.0f)))) + float4(_187.x, _300, _187.z, 0.0f);
    float2 _420 = (_301.xz * 0.100000001490116119384765625f) + (float2(time, 0.0f) * 1.2000000476837158203125f);
    int2 _422 = int2(floor(_420));
    float2 _423 = frac(_420);
    uint2 _424 = uint2(_422);
    uint _426 = _424.y;
    uint _428 = _426 + (_426 << 10u);
    uint _430 = _428 ^ (_428 >> 6u);
    uint _432 = _430 + (_430 << 3u);
    uint _434 = _432 ^ (_432 >> 11u);
    uint _440 = (((_424.x * 1664525u) + (_434 + (_434 << 15u))) + 1013904223u) * 1664525u;
    uint _442 = _440 ^ (_440 >> 11u);
    uint _445 = _442 ^ ((_442 << 7u) & 2636928640u);
    uint _448 = _445 ^ ((_445 << 15u) & 4022730752u);
    uint2 _456 = uint2(_422 + int2(1, 0));
    uint _458 = _456.y;
    uint _460 = _458 + (_458 << 10u);
    uint _462 = _460 ^ (_460 >> 6u);
    uint _464 = _462 + (_462 << 3u);
    uint _466 = _464 ^ (_464 >> 11u);
    uint _472 = (((_456.x * 1664525u) + (_466 + (_466 << 15u))) + 1013904223u) * 1664525u;
    uint _474 = _472 ^ (_472 >> 11u);
    uint _477 = _474 ^ ((_474 << 7u) & 2636928640u);
    uint _480 = _477 ^ ((_477 << 15u) & 4022730752u);
    uint2 _488 = uint2(_422 + int2(0, 1));
    uint _490 = _488.y;
    uint _492 = _490 + (_490 << 10u);
    uint _494 = _492 ^ (_492 >> 6u);
    uint _496 = _494 + (_494 << 3u);
    uint _498 = _496 ^ (_496 >> 11u);
    uint _504 = (((_488.x * 1664525u) + (_498 + (_498 << 15u))) + 1013904223u) * 1664525u;
    uint _506 = _504 ^ (_504 >> 11u);
    uint _509 = _506 ^ ((_506 << 7u) & 2636928640u);
    uint _512 = _509 ^ ((_509 << 15u) & 4022730752u);
    uint2 _520 = uint2(_422 + int2(1, 1));
    uint _522 = _520.y;
    uint _524 = _522 + (_522 << 10u);
    uint _526 = _524 ^ (_524 >> 6u);
    uint _528 = _526 + (_526 << 3u);
    uint _530 = _528 ^ (_528 >> 11u);
    uint _536 = (((_520.x * 1664525u) + (_530 + (_530 << 15u))) + 1013904223u) * 1664525u;
    uint _538 = _536 ^ (_536 >> 11u);
    uint _541 = _538 ^ ((_538 << 7u) & 2636928640u);
    uint _544 = _541 ^ ((_541 << 15u) & 4022730752u);
    float2 _554 = (_423 * _423) * (3.0f.xx - (_423 * 2.0f));
    float _555 = _554.x;
    float _556 = 1.0f - _555;
    float _558 = _554.y;
    float _570 = _176.y * _176.y;
    float _578 = (((smoothstep(-0.20000000298023223876953125f, 0.5f, _270) * _570) * (sin((((((1.0f - _558) * (((asfloat(((_448 ^ (_448 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _556) + ((asfloat(((_480 ^ (_480 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _555))) + (((asfloat(((_512 ^ (_512 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _556) * _558)) + (((asfloat(((_544 ^ (_544 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _555) * _558)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(_185 * 32 + 24) & 65535u).x;
    _404.y = _403.y - (_578 * 0.699999988079071044921875f);
    _413.x = _413.x - (_578 * 0.25f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, _413);
    out_var_C0 = lerp(_202 * 0.800000011920928955078125f, _202, _570.xxx);
    out_var_C2 = cross(normalize(float3(0.0f, _259.y, 1.0f)), normalize(float3(1.0f, _259.x, 0.0f)));
    out_var_C3 = _413.xyz;
    out_var_C4 = _301;
    out_var_C5 = normalize(_404);
    out_var_C6 = _308;
    out_var_C7 = _176;
    out_var_C8 = 1.0f;
    out_var_C9 = float3(_176 * float2(_176.y, 1.0f), _176.y);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
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
// lifted from _grass_lod1.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint current_view_index : packoffset(c0);
    float time : packoffset(c0.y);
    uint2 terrain_heightmap_dims : packoffset(c0.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint grass_buffer_size : packoffset(c1.z);
};


static float3 in_var_C0;
static float3 in_var_C2;
static float3 in_var_C3;
static float3 in_var_C5;
static float2 in_var_C7;
static float3 in_var_C9;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(5)]] float3 in_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(9)]] float3 in_var_C9 : TEXCOORD9;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float4 _105 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (abs(in_var_C9.x / in_var_C9.z) < lerp(-0.800000011920928955078125f, 1.0f, in_var_C7.y))
    {
        discard;
    }
    float3 _126;
    if (dot(normalize((_105.xyz / _105.w.xxx) - in_var_C3), in_var_C5) <= 0.0f)
    {
        _126 = -in_var_C5;
    }
    else
    {
        _126 = in_var_C5;
    }
    float3 _127 = normalize(_126);
    float3 _130 = (in_var_C0 * 0.550000011920928955078125f) * lerp(0.699999988079071044921875f, 1.10000002384185791015625f, in_var_C7.y);
    float3 _140;
    if (frame_constants_glade_theme_id == 4u)
    {
        _140 = lerp(0.89999997615814208984375f.xxx, _130 * 0.5f, pow(in_var_C7.y, 0.5f).xxx);
    }
    else
    {
        _140 = _130;
    }
    float3 _150 = _127 / ((abs(_127.x) + abs(_127.y)) + abs(_127.z)).xxx;
    float3 _164;
    if (_150.z < 0.0f)
    {
        float2 _162 = (1.0f.xx - abs(_150.yx)) * ((step(0.0f.xx, _150.xy) * 2.0f) - 1.0f.xx);
        _164 = float3(_162.x, _162.y, _150.z);
    }
    else
    {
        _164 = _150;
    }
    float2 _167 = (_164.xy * 0.5f) + 0.5f.xx;
    float3 _169 = (normalize(in_var_C2) * 0.5f) + 0.5f.xxx;
    float3 _197 = sqrt(clamp(_140, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_169.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_169.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_169.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_167.x, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), ((uint((clamp(_197.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_197.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_197.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_167.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u));
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C9 = stage_input.in_var_C9;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _grass_lod1.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct GrassBladeDataPacked
{
    float3 position;
    uint color;
    uint packed0;
    uint packed1;
    uint packed2;
    uint pad;
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
    float time : packoffset(c0.y);
    uint2 terrain_heightmap_dims : packoffset(c0.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint grass_buffer_size : packoffset(c1.z);
};

ByteAddressBuffer grass_blade_buffer_data : register(t17, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 out_var_C0;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;
static float3 out_var_C5;
static float3 out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static float3 out_var_C9;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float3 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float3 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3 out_var_C9 : TEXCOORD9;
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
    uint _168 = uint(gl_VertexIndex) % 3u;
    float2 _176;
    if (_168 == 0u)
    {
        _176 = float2(-1.0f, 1.10000002384185791015625f);
    }
    else
    {
        bool2 _174 = (_168 == 1u).xx;
        _176 = float2(_174.x ? 0.0f.xx.x : 1.0f.xx.x, _174.y ? 0.0f.xx.y : 1.0f.xx.y);
    }
    float3 _180 = float3(_176.x * 0.13500000536441802978515625f, _176.y, 0.0f);
    uint _185 = (grass_buffer_size - 1u) - (uint(gl_VertexIndex) / 3u);
    float3 _187 = grass_blade_buffer_data.Load<float3>(_185 * 32 + 0);
    float3 _201 = float3(float(grass_blade_buffer_data.Load<uint>(_185 * 32 + 12) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 12) >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 12) >> 16u) & 255u) * 0.0039215688593685626983642578125f);
    float3 _202 = _201 * _201;
    float2 _220 = spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(_185 * 32 + 20) & 65535u);
    float _221 = _220.x;
    float4 _241 = terrain_heightmap.GatherRed(sampler_default, (_187.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, int2(0, 0));
    float _246 = _241.z;
    float _247 = _241.y;
    float _249 = _241.w;
    float _250 = _241.x;
    float2 _259 = ((float2((_246 + _247) - (_249 + _250), (_250 + _247) - (_249 + _246)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
    float _270 = _221 * 0.20000000298023223876953125f;
    float3 _272 = float3(min(1.0f, _221 * 0.800000011920928955078125f) * 0.3599999845027923583984375f, _270, 1.0f) * 2.5f;
    float _295 = max(30.0f / (((-1.0f) / mul(frame_constants_view_constants[0u].world_to_view, float4(_187, 1.0f)).z) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float _298 = _272.y;
    float _300 = _187.y - ((_295 - 1.0f) * _298);
    float3 _301 = _187;
    _301.y = _300;
    float _302 = _298 * _295;
    float3 _308 = float3((float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 16) >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f, spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(_185 * 32 + 24) >> 16u) & 65535u).x);
    float3 _309 = normalize(_308);
    float4x4 _321 = transpose(float4x4(float4(normalize(cross(_309, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(_309, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4 _327 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float _338 = clamp(_176.y, 0.0f, 1.0f);
    float _339 = 3.0f * ((0.02999999932944774627685546875f * spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(_185 * 32 + 20) >> 16u) & 65535u).x) * lerp(1.0f, 2.0f, abs(normalize((_327.xyz / _327.w.xxx) - _301).y)));
    float _341 = _339 * (_338 * _338);
    float _342 = _338 + 0.00010001659393310546875f;
    _180.z = _341;
    float3 _354;
    if (frame_constants_glade_theme_id == 4u)
    {
        float3 _353 = _180;
        _353.x = _176.x * 0.067500002682209014892578125f;
        _354 = _353;
    }
    else
    {
        _354 = _180;
    }
    float _355 = float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 16) >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float3 _356 = normalize(float3(1.0f, 0.0f, 0.0f));
    float _357 = sin(_355);
    float _358 = cos(_355);
    float _359 = 1.0f - _358;
    float _360 = _356.x;
    float _361 = _359 * _360;
    float _364 = _356.y;
    float _365 = _361 * _364;
    float _366 = _356.z;
    float _367 = _366 * _357;
    float _369 = _359 * _366;
    float _370 = _369 * _360;
    float _371 = _364 * _357;
    float _374 = _359 * _364;
    float _377 = _374 * _366;
    float _378 = _360 * _357;
    float4x4 _388 = transpose(float4x4(float4((_361 * _360) + _358, _365 - _367, _370 + _371, 0.0f), float4(_365 + _367, (_374 * _364) + _358, _377 - _378, 0.0f), float4(_370 - _371, _377 + _378, (_369 * _366) + _358, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 _392 = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 9.9999997473787516355514526367188e-05f * _302, (_339 * (_342 * _342)) - _341)));
    _392.x = _392.x + (0.300000011920928955078125f * _176.x);
    float4 _403 = mul(_321, mul(_388, float4(normalize(_392), 0.0f)));
    float3 _404 = _403.xyz;
    float4 _413 = mul(_321, mul(_388, mul(float4x4(float4((_272.x * (float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 16) >> 8u) & 255u) * 0.0039215688593685626983642578125f)) * _295, 0.0f, 0.0f, 0.0f), float4(0.0f, _302, 0.0f, 0.0f), float4(0.0f, 0.0f, _272.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(_354, 1.0f)))) + float4(_187.x, _300, _187.z, 0.0f);
    float2 _420 = (_301.xz * 0.100000001490116119384765625f) + (float2(time, 0.0f) * 1.2000000476837158203125f);
    int2 _422 = int2(floor(_420));
    float2 _423 = frac(_420);
    uint2 _424 = uint2(_422);
    uint _426 = _424.y;
    uint _428 = _426 + (_426 << 10u);
    uint _430 = _428 ^ (_428 >> 6u);
    uint _432 = _430 + (_430 << 3u);
    uint _434 = _432 ^ (_432 >> 11u);
    uint _440 = (((_424.x * 1664525u) + (_434 + (_434 << 15u))) + 1013904223u) * 1664525u;
    uint _442 = _440 ^ (_440 >> 11u);
    uint _445 = _442 ^ ((_442 << 7u) & 2636928640u);
    uint _448 = _445 ^ ((_445 << 15u) & 4022730752u);
    uint2 _456 = uint2(_422 + int2(1, 0));
    uint _458 = _456.y;
    uint _460 = _458 + (_458 << 10u);
    uint _462 = _460 ^ (_460 >> 6u);
    uint _464 = _462 + (_462 << 3u);
    uint _466 = _464 ^ (_464 >> 11u);
    uint _472 = (((_456.x * 1664525u) + (_466 + (_466 << 15u))) + 1013904223u) * 1664525u;
    uint _474 = _472 ^ (_472 >> 11u);
    uint _477 = _474 ^ ((_474 << 7u) & 2636928640u);
    uint _480 = _477 ^ ((_477 << 15u) & 4022730752u);
    uint2 _488 = uint2(_422 + int2(0, 1));
    uint _490 = _488.y;
    uint _492 = _490 + (_490 << 10u);
    uint _494 = _492 ^ (_492 >> 6u);
    uint _496 = _494 + (_494 << 3u);
    uint _498 = _496 ^ (_496 >> 11u);
    uint _504 = (((_488.x * 1664525u) + (_498 + (_498 << 15u))) + 1013904223u) * 1664525u;
    uint _506 = _504 ^ (_504 >> 11u);
    uint _509 = _506 ^ ((_506 << 7u) & 2636928640u);
    uint _512 = _509 ^ ((_509 << 15u) & 4022730752u);
    uint2 _520 = uint2(_422 + int2(1, 1));
    uint _522 = _520.y;
    uint _524 = _522 + (_522 << 10u);
    uint _526 = _524 ^ (_524 >> 6u);
    uint _528 = _526 + (_526 << 3u);
    uint _530 = _528 ^ (_528 >> 11u);
    uint _536 = (((_520.x * 1664525u) + (_530 + (_530 << 15u))) + 1013904223u) * 1664525u;
    uint _538 = _536 ^ (_536 >> 11u);
    uint _541 = _538 ^ ((_538 << 7u) & 2636928640u);
    uint _544 = _541 ^ ((_541 << 15u) & 4022730752u);
    float2 _554 = (_423 * _423) * (3.0f.xx - (_423 * 2.0f));
    float _555 = _554.x;
    float _556 = 1.0f - _555;
    float _558 = _554.y;
    float _570 = _176.y * _176.y;
    float _578 = (((smoothstep(-0.20000000298023223876953125f, 0.5f, _270) * _570) * (sin((((((1.0f - _558) * (((asfloat(((_448 ^ (_448 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _556) + ((asfloat(((_480 ^ (_480 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _555))) + (((asfloat(((_512 ^ (_512 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _556) * _558)) + (((asfloat(((_544 ^ (_544 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _555) * _558)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(_185 * 32 + 24) & 65535u).x;
    _404.y = _403.y - (_578 * 0.699999988079071044921875f);
    _413.x = _413.x - (_578 * 0.25f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, _413);
    out_var_C0 = lerp(_202 * 0.800000011920928955078125f, _202, _570.xxx);
    out_var_C2 = cross(normalize(float3(0.0f, _259.y, 1.0f)), normalize(float3(1.0f, _259.x, 0.0f)));
    out_var_C3 = _413.xyz;
    out_var_C4 = _301;
    out_var_C5 = normalize(_404);
    out_var_C6 = _308;
    out_var_C7 = _176;
    out_var_C8 = 1.0f;
    out_var_C9 = float3(_176 * float2(_176.y, 1.0f), _176.y);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
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
// lifted from _grass_lod1.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float2 in_var_C7;
static float3 in_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(9)]] float3 in_var_C9 : TEXCOORD9;
};

void frag_main()
{
    if (abs(in_var_C9.x / in_var_C9.z) < lerp(-0.800000011920928955078125f, 1.0f, in_var_C7.y))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C7 = stage_input.in_var_C7;
    in_var_C9 = stage_input.in_var_C9;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
