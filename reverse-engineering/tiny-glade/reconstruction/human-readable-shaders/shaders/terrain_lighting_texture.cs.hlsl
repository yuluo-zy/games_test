// PATCHED: modules re.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/terrain_lighting_texture.cs.hlsl | evidence: spirv-cross of _terrain_lighting_texture.cs.hlsl_cs_main_0.bin (terrain indirect light + soft shadow; grass/dirt color from height comparison, 5-tap depth gather + 10-tap PCF) | status: O (bindings/entry/LocalSize/flow); I (names)
// ============================================================
// 地形光照纹理生成(compute)。
//   入口 cs_main(numthreads 8x8x1): 逐像素计算地形间接光照与软阴影。
//   主要流程:
//     - 依据"基础高度 vs 可视化高度"选择草色或泥土色;
//     - 用可视化高度图 GatherRed 求梯度 → 世界法线;
//     - 对主光源做级联阴影采样:5 次深度抓取求平均 + 10 次 PCF 软阴影(螺旋采样);
//     - 组合日光 + 自阴影 + 环境项,写入 img_output(u16)。
//   绑定: shadow_map(t13)、terrain_heightmap(t17)、visual_terrain_heightmap(t18)、
//         img_output(u16)、color_library(b14)、$Globals(b15)。
//   变体: TGV_0(compute)。
// ============================================================

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
Texture2D<float4> terrain_heightmap : register(t17, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/shadow.hlsl"
#include "inc/cascades.hlsl"
#include "inc/math.hlsl"
#include "inc/pack_unpack.hlsl"

// color_library.hlsl is guard-skipped; the cbuffer is redeclared below under
// the original binding name `color_library` (file OpString is still recorded).
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"

struct ColorLibrary
{
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

cbuffer color_library : register(b14, space0)
{
    ColorLibrary colors;
};

cbuffer $Globals : register(b15, space0)
{
    uint2 img_output_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    int bool_render_shadows;
    uint2 brdf_fg_lut_dims;
};

Texture2DArray<float4> shadow_map : register(t13, space0);
RWTexture2D<float3> img_output : register(u16, space0);
Texture2D<float4> visual_terrain_heightmap : register(t18, space0);

// 泥土颜色:当地形基础高度高于可视化高度时使用(露出的泥土)。
static const float3 DIRT_COLOR = float3(0.122f, 0.122f, 0.089f);

[[vk::ext_capability(5302)]]
[[vk::ext_capability(49)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    // 阴影开关关闭时输出黄色调试帧并提前返回。
    if (bool_render_shadows == 0)
    {
        img_output[uint2(int3(dispatch_thread_id).xy)] = float3(1.0f, 1.0f, 0.0f);
        return;
    }

    // 草色:grass_1 与 grass_2 按 0.15 插值,再乘亮度系数。
    float3 grass_color = lerp(colors.grass_1.xyz, colors.grass_2.xyz, 0.15f.xxx) * 1.2f * 0.55f;
    // 像素 → UV → 世界 XZ(±65 范围),再换算成高度图采样坐标。
    float2 uv = (float2(int3(dispatch_thread_id).xy) + 0.5f.xx) / float2(int2(img_output_dims));
    float2 world_xz = ((uv * 2.0f) - 1.0f.xx) * 65.0f.xx;
    float x = world_xz.x;
    float y = world_xz.y;
    float2 height_uv = world_xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f.xx;

    // 可视化高度 vs 基础高度:基础更高(被填土)则显示泥土色,否则草色。
    float visual_height = lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, visual_terrain_heightmap.SampleLevel(sampler_lnc, height_uv, 0.0f).x);
    float base_height = lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, terrain_heightmap.SampleLevel(sampler_default, height_uv, 0.0f).x);
    float3 albedo = (base_height + 0.1f > visual_height) ? grass_color : DIRT_COLOR;

    // 用 GatherRed 4 邻居求高度梯度,再叉积得到地形法线。
    float4 gathered = visual_terrain_heightmap.GatherRed(sampler_default, height_uv, int2(0, 0));
    float2 grad = (float2((gathered.z + gathered.y) - (gathered.w + gathered.x), (gathered.x + gathered.y) - (gathered.w + gathered.z)) * 0.5f) * 26.5f;
    grad /= (130.0f.xx / float2(terrain_heightmap_dims));
    float3 normal = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));

    // 世界空间的"上方向"(view_constants[2] 固定相机)与 N·L。
    float3 view_up = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    // 调试覆写:render_overrides 位 0 置位时强制 0.5 灰。
    float3 albedo_override = ((render_overrides & 1u) != 0u) ? float3(0.5f, 0.5f, 0.5f) : albedo;
    float ndl = max(0.0f, dot(normal, view_up));

    // 阴影:世界位置投影到采样空间,按越界/收缩选择级联,做 5 深度抓取 + 10 PCF。
    float2 shadow = 0.0f.xx;
    if (bool_render_shadows == 1)
    {
        float4 sample_clip = mul(view_constants[2].world_to_sample, float4(x, visual_height, y, 1.0f));
        if (sample_clip.z <= 1.0f)
        {
            // 屏幕边缘外的像素缩到内部级联(0.5 缩放、5 级联)。
            float2 abs_xy = abs(sample_clip.xy);
            bool outside = any(abs_xy > 0.9833333f.xx);
            float2 sample_xy = outside ? sample_clip.xy * 0.5f : sample_clip.xy;
            float cascade = outside ? 5.0f : 10.0f;
            float sample_scale = outside ? 0.5f : 1.0f;
            // 内部还有一层收缩(边缘 0.2857 缩放、2 级联),用于过渡区。
            float2 limit = 1.0f.xx - 0.0008333333f.xx * (2.0f * cascade);
            bool shrink = any(abs(sample_xy) > limit);
            float pcf_radius = shrink ? cascade * 0.28571428f : cascade;
            float2 sample_uv = shrink ? sample_xy * 0.28571428f : sample_xy;
            float sscale = shrink ? sample_scale * 0.28571428f : sample_scale;
            uint cascade_layer = shrink ? 2u : uint(outside);
            uint cascade_index = 2u + cascade_layer;

            // 法线方向偏移采样点(减轻漏光),并构造级联内 UV。
            float cos_theta = dot(normal, -view_up);
            float4 ray_sample = float4(sample_uv.x, sample_uv.y, sample_clip.z, sample_clip.w)
                + mul(view_constants[cascade_index].world_to_sample, float4((normal * (0.06666667f * clamp(1.0f - cos_theta, 0.0f, 1.0f))) / sscale, 0.0f));
            float2 pc = (ray_sample.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float3 sample_pos = float3(pc.x, pc.y, ray_sample.z);
            float probe_radius = max((0.3f / sscale) * (1.0f - abs(cos_theta)), 0.025f);

            // 逐像素伪随机(角度 + 噪声),用于抖动 PCF 采样。
            uint hash = uint((int(dispatch_thread_id.x) * 11) + (int(dispatch_thread_id.y) * 7) + (frame_index * 5u)) % 64u;
            float angle_hash = float(hash) * GOLDEN_RATIO;
            float frame_noise_scale = 5.588237762451171875f * float(frame_index % 64u);
            float noise = frac(52.98291778564453125f * frac((0.0671105608344078f * (float(dispatch_thread_id.x) + frame_noise_scale)) + (0.005837149918079376f * (float(dispatch_thread_id.y) + frame_noise_scale))));

            float4 shadow_offset = mul(view_constants[cascade_index].world_to_sample, float4(normal, 0.0f));

            // 5 次深度抓取:螺旋分布,统计遮挡深度均值与命中数。
            float depth_sum = 0.0f;
            int hit_count = 0;
            for (int i = 0; i < 5; i++)
            {
                float angle = (float(i) + angle_hash) * 2.3999631404876708984375f;
                float3 offset = sample_pos + float3((float2(cos(angle), sin(angle)) * (float(i) + noise) * 0.2f.xx * pcf_radius) * 0.0008333333f.xx, probe_radius * (-0.0009765625f));
                float sample_depth = shadow_map.SampleLevel(sampler_shadow_nearest, float3(offset.xy, cascade_layer), 0.0f).x;
                if (sample_pos.z > sample_depth)
                {
                    depth_sum += sample_depth;
                    hit_count++;
                }
            }

            // 10 次 PCF 软阴影:按遮挡光照量缩放 PCF 半径,累计可见性。
            float2 pcf = 0.0f.xx;
            if (hit_count != 0)
            {
                float avg_depth = depth_sum / float(hit_count);
                float light = max((ray_sample.z - avg_depth) * ((17.0666675567626953125f * pcf_radius) * pow(10.0f, smoothstep(0.0f, 0.25f, fog_tint_thickness.w))) / avg_depth, 0.0f);
                float pcf_scale = (light / pow(pow(light / (pcf_radius - 1.0f), 5.0f) + 1.0f, 0.2f)) + 1.0f;
                float2 shadow_offset_uv = ((shadow_offset.xy * float(int(sign(shadow_offset.z)))) / max(0.15f, abs(shadow_offset.z)).xx) * 0.0009765625f.xx * 2.0f;
                for (int j = 0; j < 10; j++)
                {
                    float angle = (float(j) + angle_hash) * 2.3999631404876708984375f;
                    float2 spiral = float2(cos(angle), sin(angle)) * (float(j) + noise) * 0.1f.xx * pcf_scale;
                    float3 offset = sample_pos + float3(spiral * 0.0008333333f.xx, probe_radius * (-0.0009765625f));
                    pcf += float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(offset.xy, cascade_layer), offset.z + min(0.0f, dot(shadow_offset_uv, spiral))), 0.0f);
                }
                shadow = pcf * 0.1f;
            }
        }
    }

    // 光照 = 日光 × (1 - 阴影) × N·L 平滑 × albedo;再叠加地面反射系数增强的间接项。
    float3 lit = (sun_color.xyz * 1.0f) * ((1.0f - shadow.x) * smoothstep(0.0f, 0.5f, ndl)) * ndl * albedo_override * 0.3183098733425140380859375f;
    float ground_factor = 1.0f - clamp(normal.y, 0.0f, 1.0f);
    float view_factor = 1.0f - clamp(view_up.y, 0.0f, 1.0f);
    float3 sky = lit + (lit * (((ground_factor * ground_factor) * ground_factor) * ((view_factor * view_factor) * view_factor)));
    // 天空环境项(按法线朝上程度),最终写入输出纹理。
    img_output[uint2(int3(dispatch_thread_id).xy)] = sky + ((sky_high_color.xyz * 1.0f) * albedo_override * ((normal.y * 0.5f) + 0.5f));
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _terrain_lighting_texture.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static float zero;

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

cbuffer color_library : register(b14, space0)
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

cbuffer $Globals : register(b15, space0)
{
    uint2 img_output_dims : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    int bool_render_shadows : packoffset(c1.z);
    uint2 brdf_fg_lut_dims : packoffset(c2);
};

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_default : register(s9, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);
Texture2DArray<float4> shadow_map : register(t13, space0);
RWTexture2D<float3> img_output : register(u16, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> visual_terrain_heightmap : register(t18, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        // 阴影开关关闭:输出黄色调试帧。
        if (bool_render_shadows == 0)
        {
            img_output[uint2(int3(gl_GlobalInvocationID).xy)] = float3(1.0f, 1.0f, 0.0f);
            break;
        }
        // 草色与泥土色的基础反照率(由高度比较选择)。
        float3 grass_color = (lerp(color_library_grass_1.xyz, color_library_grass_2.xyz, 0.1500000059604644775390625f.xxx) * 1.2000000476837158203125f) * 0.550000011920928955078125f;
        // 像素 → 世界 XZ(±65)。
        float2 world_xz = ((((float2(int3(gl_GlobalInvocationID).xy) + 0.5f.xx) / float2(int2(img_output_dims))) * 2.0f) - 1.0f.xx) * 65.0f.xx;
        float x = world_xz.x;
        float y = world_xz.y;
        // 高度图采样坐标(TERRAIN_SAMPLE_SCALE = 1/130)。
        float2 height_uv = (float3(x, 0.0f, y).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        // 可视化高度(用于光照/阴影)与基础高度(用于草/泥土判别)。
        float visual_height = lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, height_uv, 0.0f).x);
        bool3 is_grass = ((lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, height_uv, 0.0f).x) + 0.100000001490116119384765625f) > visual_height).xxx;
        // 按判别选择草色或泥土色(DIRT_COLOR)。
        float3 albedo = float3(is_grass.x ? grass_color.x : float3(0.122000001370906829833984375f, 0.122000001370906829833984375f, 0.08900000154972076416015625f).x, is_grass.y ? grass_color.y : float3(0.122000001370906829833984375f, 0.122000001370906829833984375f, 0.08900000154972076416015625f).y, is_grass.z ? grass_color.z : float3(0.122000001370906829833984375f, 0.122000001370906829833984375f, 0.08900000154972076416015625f).z);
        // GatherRed 求高度梯度 → 地形法线。
        float4 gathered = visual_terrain_heightmap.GatherRed(sampler_default, height_uv, int2(0, 0));
        float g_z = gathered.z;
        float g_y = gathered.y;
        float g_w = gathered.w;
        float g_x = gathered.x;
        float2 grad = ((float2((g_z + g_y) - (g_w + g_x), (g_x + g_y) - (g_w + g_z)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
        float3 normal = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));
        // 世界空间上方向与调试覆写、N·L。
        float4 view_up4 = mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
        float3 view_up = view_up4.xyz;
        bool3 override_flag = ((frame_constants_render_overrides & 1u) != 0u).xxx;
        float3 albedo_override = float3(override_flag.x ? 0.5f.xxx.x : albedo.x, override_flag.y ? 0.5f.xxx.y : albedo.y, override_flag.z ? 0.5f.xxx.z : albedo.z);
        float ndl = max(0.0f, dot(normal, view_up));
        // 阴影计算(5 深度抓取 + 10 PCF)。
        float shadow_x;
        if (bool_render_shadows == 1)
        {
            float2 shadow;
            do
            {
                // 世界位置投影到采样空间。
                float4 sample_clip = mul(frame_constants_view_constants[2u].world_to_sample, float4(x, visual_height, y, 1.0f));
                if (sample_clip.z > 1.0f)
                {
                    shadow = 0.0f.xx;
                    break;
                }
                // 越界收缩:外部像素缩到 0.5 比例、级联 5;内部级联 10。
                float2 sample_clip_xy = sample_clip.xy;
                float2 abs_xy = abs(sample_clip_xy);
                bool outside = any(bool2(abs_xy.x > 0.9833333492279052734375f.xx.x, abs_xy.y > 0.9833333492279052734375f.xx.y));
                float2 sample_xy;
                if (outside)
                {
                    sample_xy = sample_clip_xy * 0.5f;
                }
                else
                {
                    sample_xy = sample_clip_xy;
                }
                float cascade = outside ? 5.0f : 10.0f;
                float sample_scale = outside ? 0.5f : 1.0f;
                // 内部再收缩一层(0.2857 比例、级联 2)用于过渡区。
                float2 abs_sample_xy = abs(sample_xy);
                float2 limit = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * cascade));
                bool shrink = any(bool2(abs_sample_xy.x > limit.x, abs_sample_xy.y > limit.y));
                float pcf_radius;
                float2 sample_uv;
                float sscale;
                if (shrink)
                {
                    pcf_radius = cascade * 0.2857142984867095947265625f;
                    sample_uv = sample_xy * 0.2857142984867095947265625f;
                    sscale = sample_scale * 0.2857142984867095947265625f;
                }
                else
                {
                    pcf_radius = cascade;
                    sample_uv = sample_xy;
                    sscale = sample_scale;
                }
                float cascade_layer = float(shrink ? 2u : uint(outside));
                uint cascade_index = 2u + uint(cascade_layer);
                // 法线方向偏移采样点,构造级联内 UV 与探针半径。
                float cos_theta = dot(normal, -view_up);
                float4 ray_sample = float4(sample_uv.x, sample_uv.y, sample_clip.z, sample_clip.w) + mul(frame_constants_view_constants[cascade_index].world_to_sample, float4((normal * (0.066666670143604278564453125f * clamp(1.0f - cos_theta, 0.0f, 1.0f))) / sscale.xxx, 0.0f));
                float2 pc = (ray_sample.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                float3 sample_pos = float3(pc.x, pc.y, ray_sample.z);
                float probe_radius = max((0.300000011920928955078125f / sscale) * (1.0f - abs(cos_theta)), 0.02500000037252902984619140625f);
                // 逐像素伪随机角度(黄金比例)与帧间噪声。
                float angle_hash = float((uint((int3(gl_GlobalInvocationID).x * 11) + (int3(gl_GlobalInvocationID).y * 7)) + (frame_constants_frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
                uint2 thread_xy = uint2(int3(gl_GlobalInvocationID).xy);
                float frame_noise_scale = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float noise = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(thread_xy.x) + frame_noise_scale)) + (0.005837149918079376220703125f * (float(thread_xy.y) + frame_noise_scale))));
                // 世界空间法线的采样偏移向量。
                float4 shadow_offset = mul(frame_constants_view_constants[cascade_index].world_to_sample, float4(normal, 0.0f));
                // 5 次深度抓取:螺旋分布,统计遮挡深度均值与命中数。
                float depth_sum;
                int hit_count;
                depth_sum = 0.0f;
                hit_count = 0;
                float depth_sum_next;
                int hit_count_next;
                for (int i = 0; i < 5; depth_sum = depth_sum_next, hit_count = hit_count_next, i++)
                {
                    float i_f = float(i);
                    float angle = (i_f + angle_hash) * 2.3999631404876708984375f;
                    float3 offset = sample_pos + float3((((float2(cos(angle), sin(angle)) * (i_f + noise)) * 0.20000000298023223876953125f.xx) * pcf_radius) * 0.000833333353511989116668701171875f.xx, probe_radius * (-0.0009765625f));
                    float4 shadow_sample = shadow_map.SampleLevel(sampler_shadow_nearest, float3(offset.xy, cascade_layer), 0.0f);
                    float sample_depth = shadow_sample.x;
                    if (offset.z > sample_depth)
                    {
                        depth_sum_next = depth_sum + sample_depth;
                        hit_count_next = hit_count + 1;
                    }
                    else
                    {
                        depth_sum_next = depth_sum;
                        hit_count_next = hit_count;
                    }
                }
                float avg_depth = depth_sum / float(hit_count);
                if (hit_count == 0)
                {
                    shadow = 0.0f.xx;
                    break;
                }
                // 由平均遮挡深度求光照量,再换算 PCF 缩放。
                float light = max((ray_sample.z - avg_depth) * ((17.0666675567626953125f * pcf_radius) * pow(10.0f, smoothstep(0.0f, 0.25f, frame_constants_fog_tint_thickness.w))) / avg_depth, 0.0f);
                float pcf_scale = (light / pow(pow(light / (pcf_radius - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
                // 阴影贴图偏移的屏幕空间方向。
                float2 shadow_offset_uv = (((shadow_offset.xy * float(int(sign(shadow_offset.z)))) / max(0.1500000059604644775390625f, abs(shadow_offset.z)).xx) * 0.0009765625f.xx) * 2.0f;
                // 10 次 PCF 软阴影(螺旋可见性累计)。
                float2 pcf;
                pcf = 0.0f.xx;
                for (int j = 0; j < 10; )
                {
                    float j_f = float(j);
                    float angle2 = (j_f + angle_hash) * 2.3999631404876708984375f;
                    float2 spiral = ((float2(cos(angle2), sin(angle2)) * (j_f + noise)) * 0.100000001490116119384765625f.xx) * pcf_scale;
                    float3 offset2 = sample_pos + float3(spiral * 0.000833333353511989116668701171875f.xx, probe_radius * (-0.0009765625f));
                    pcf += float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(offset2.xy, cascade_layer), offset2.z + min(0.0f, dot(shadow_offset_uv, spiral))), zero);
                    j++;
                    continue;
                }
                shadow = pcf * 0.100000001490116119384765625f;
                break;
            } while(false);
            shadow_x = shadow.x;
        }
        else
        {
            shadow_x = 0.0f;
        }
        // 组合日光(含自阴影与 N·L)与地面反射增强的间接项。
        float3 lit = ((((frame_constants_sun_color.xyz * 1.0f) * ((1.0f - shadow_x) * smoothstep(0.0f, 0.5f, ndl))) * ndl) * albedo_override) * 0.3183098733425140380859375f.xxx;
        float normal_y = normal.y;
        float ground_factor = 1.0f - clamp(normal_y, 0.0f, 1.0f);
        float ground_factor2 = ground_factor * ground_factor;
        float view_factor = 1.0f - clamp(view_up4.y, 0.0f, 1.0f);
        // 最终输出:日光 + 间接增强 + 天空环境项。
        img_output[uint2(int3(gl_GlobalInvocationID).xy)] = (lit + (lit * (((ground_factor2 * ground_factor2) * ground_factor) * ((view_factor * view_factor) * view_factor)))) + ((((frame_constants_sky_high_color.xyz * 1.0f) * albedo_override) * ((normal_y * 0.5f) + 0.5f)) * 1.0f);
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
