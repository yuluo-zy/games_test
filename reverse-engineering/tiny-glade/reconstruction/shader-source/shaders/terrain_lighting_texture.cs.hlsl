// ==RECONSTRUCTED== shaders/terrain_lighting_texture.cs.hlsl | evidence: spirv-cross of _terrain_lighting_texture.cs.hlsl_cs_main_0.bin (terrain indirect light + soft shadow; grass/dirt color from height comparison, 5-tap depth gather + 10-tap PCF) | status: O (bindings/entry/LocalSize/flow); I (names)

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

// Dirt color used where the terrain rises above the visual heightmap.
static const float3 DIRT_COLOR = float3(0.122f, 0.122f, 0.089f);

[[vk::ext_capability(5302)]]
[[vk::ext_capability(49)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    if (bool_render_shadows == 0)
    {
        img_output[uint2(int3(dispatch_thread_id).xy)] = float3(1.0f, 1.0f, 0.0f);
        return;
    }

    float3 grass_color = lerp(colors.grass_1.xyz, colors.grass_2.xyz, 0.15f.xxx) * 1.2f * 0.55f;
    float2 uv = (float2(int3(dispatch_thread_id).xy) + 0.5f.xx) / float2(int2(img_output_dims));
    float2 world_xz = ((uv * 2.0f) - 1.0f.xx) * 65.0f.xx;
    float x = world_xz.x;
    float y = world_xz.y;
    float2 height_uv = world_xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f.xx;

    float visual_height = lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, visual_terrain_heightmap.SampleLevel(sampler_lnc, height_uv, 0.0f).x);
    float base_height = lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, terrain_heightmap.SampleLevel(sampler_default, height_uv, 0.0f).x);
    float3 albedo = (base_height + 0.1f > visual_height) ? grass_color : DIRT_COLOR;

    float4 gathered = visual_terrain_heightmap.GatherRed(sampler_default, height_uv, int2(0, 0));
    float2 grad = (float2((gathered.z + gathered.y) - (gathered.w + gathered.x), (gathered.x + gathered.y) - (gathered.w + gathered.z)) * 0.5f) * 26.5f;
    grad /= (130.0f.xx / float2(terrain_heightmap_dims));
    float3 normal = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));

    float3 view_up = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float3 albedo_override = ((render_overrides & 1u) != 0u) ? float3(0.5f, 0.5f, 0.5f) : albedo;
    float ndl = max(0.0f, dot(normal, view_up));

    float2 shadow = 0.0f.xx;
    if (bool_render_shadows == 1)
    {
        float4 sample_clip = mul(view_constants[2].world_to_sample, float4(x, visual_height, y, 1.0f));
        if (sample_clip.z <= 1.0f)
        {
            float2 abs_xy = abs(sample_clip.xy);
            bool outside = any(abs_xy > 0.9833333f.xx);
            float2 sample_xy = outside ? sample_clip.xy * 0.5f : sample_clip.xy;
            float cascade = outside ? 5.0f : 10.0f;
            float sample_scale = outside ? 0.5f : 1.0f;
            float2 limit = 1.0f.xx - 0.0008333333f.xx * (2.0f * cascade);
            bool shrink = any(abs(sample_xy) > limit);
            float pcf_radius = shrink ? cascade * 0.28571428f : cascade;
            float2 sample_uv = shrink ? sample_xy * 0.28571428f : sample_xy;
            float sscale = shrink ? sample_scale * 0.28571428f : sample_scale;
            uint cascade_layer = shrink ? 2u : uint(outside);
            uint cascade_index = 2u + cascade_layer;

            float cos_theta = dot(normal, -view_up);
            float4 ray_sample = float4(sample_uv.x, sample_uv.y, sample_clip.z, sample_clip.w)
                + mul(view_constants[cascade_index].world_to_sample, float4((normal * (0.06666667f * clamp(1.0f - cos_theta, 0.0f, 1.0f))) / sscale, 0.0f));
            float2 pc = (ray_sample.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float3 sample_pos = float3(pc.x, pc.y, ray_sample.z);
            float probe_radius = max((0.3f / sscale) * (1.0f - abs(cos_theta)), 0.025f);

            uint hash = uint((int(dispatch_thread_id.x) * 11) + (int(dispatch_thread_id.y) * 7) + (frame_index * 5u)) % 64u;
            float angle_hash = float(hash) * GOLDEN_RATIO;
            float frame_noise_scale = 5.588237762451171875f * float(frame_index % 64u);
            float noise = frac(52.98291778564453125f * frac((0.0671105608344078f * (float(dispatch_thread_id.x) + frame_noise_scale)) + (0.005837149918079376f * (float(dispatch_thread_id.y) + frame_noise_scale))));

            float4 shadow_offset = mul(view_constants[cascade_index].world_to_sample, float4(normal, 0.0f));

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

    float3 lit = (sun_color.xyz * 1.0f) * ((1.0f - shadow.x) * smoothstep(0.0f, 0.5f, ndl)) * ndl * albedo_override * 0.3183098733425140380859375f;
    float ground_factor = 1.0f - clamp(normal.y, 0.0f, 1.0f);
    float view_factor = 1.0f - clamp(view_up.y, 0.0f, 1.0f);
    float3 sky = lit + (lit * (((ground_factor * ground_factor) * ground_factor) * ((view_factor * view_factor) * view_factor)));
    img_output[uint2(int3(dispatch_thread_id).xy)] = sky + ((sky_high_color.xyz * 1.0f) * albedo_override * ((normal.y * 0.5f) + 0.5f));
}
