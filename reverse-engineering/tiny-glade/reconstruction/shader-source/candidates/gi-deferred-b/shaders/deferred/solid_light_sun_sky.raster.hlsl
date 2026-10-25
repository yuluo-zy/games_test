// ==RECONSTRUCTED== shaders/deferred/solid_light_sun_sky.raster.hlsl (candidate gi-deferred-b)
// Evidence: _deferred_solid_light_sun_sky.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin.
//   b903 (full) bindings: sampler_lnc s0, sampler_nnc s2, sampler_llc s3, sampler_default s9,
//     $Globals b13, gbuffer_tex t14, depth_tex t15, half_depth_tex t16, blue_noise_texture t18,
//     ssgi_tex t19, screen_shadows_tex t20, highlight_active_hill t22, color_library b23,
//     spatial_resolve_offsets b24, lantern_buf t26, indirect_radiance_tex t27, rdi_reservoir_tex t28,
//     brdf_fg_lut t29, light_indirection_grid t36, grid_light_indices t37, frame_constants set1/b0;
//   f0ad (reduced) bindings: $Globals b13, gbuffer_tex t14, depth_tex t15, spatial_resolve_offsets b24,
//     rdi_reservoir_tex t28, light_indirection_grid t36, frame_constants set1/b0.
//   vs_main = fullscreen triangle; ps_main = deferred sun+sky+lantern composition. The two hex
//   variants are compile-time feature variants: DEFERRED_FULL_LIGHTING=1 -> b903, =0 -> f0ad
//   (reservoir-gather only, no color output). The gather keeps quad/wave intrinsics so both
//   variants carry GroupNonUniformVote/Quad capabilities.
// Status: O (structure from disassembly); I (flag name; $Globals member names verbatim).

#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
struct ColorLibrary {
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

cbuffer color_library : register(b23, space0) {
    ColorLibrary color_library;
};
#include "light_sun_sky_prelude.inc.hlsl"
#include "light_sun_sky.inc.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/perlin.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/sh.hlsl"
#include "inc/shadow.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/utils.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/wall_color_ids.hlsl"
#include "inc/material/eval/solid_material.inc.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/cascades.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf_lut.hlsl"
#include "terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl"
#include "light_binning/grid.inc.hlsl"

#ifndef DEFERRED_FULL_LIGHTING
#define DEFERRED_FULL_LIGHTING 1
#endif

cbuffer $Globals : register(b13, space0)
{
    uint2 gbuffer_tex_dims;
    uint2 depth_tex_dims;
    uint2 half_depth_tex_dims;
    uint2 taa_history_tex_dims;
    uint taa_history_valid;
    uint2 blue_noise_texture_dims;
    uint2 ssgi_tex_dims;
    uint2 screen_shadows_tex_dims;
    uint2 terrain_lighting_texture_dims;
    uint2 highlight_active_hill_dims;
    float time;
    uint lantern_count;
    uint2 indirect_radiance_tex_dims;
    uint2 rdi_reservoir_tex_dims;
    uint2 brdf_fg_lut_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
};

Texture2D<uint4> gbuffer_tex : register(t14, space0);
Texture2D<float4> depth_tex : register(t15, space0);
Texture2D<float4> blue_noise_texture : register(t18, space0);
Texture2D<uint4> rdi_reservoir_tex : register(t28, space0);
ByteAddressBuffer light_indirection_grid : register(t36, space0);

#if DEFERRED_FULL_LIGHTING
Texture2D<float4> half_depth_tex : register(t16, space0);
Texture2D<float4> ssgi_tex : register(t19, space0);
Texture2D<float4> screen_shadows_tex : register(t20, space0);
Texture2D<float4> highlight_active_hill : register(t22, space0);
ByteAddressBuffer lantern_buf : register(t26, space0);
Texture2D<float4> indirect_radiance_tex : register(t27, space0);
Texture2D<float4> brdf_fg_lut : register(t29, space0);
ByteAddressBuffer grid_light_indices : register(t37, space0);
#endif

// Flat color_library cbuffer at the deferred pass binding (b23). The inc's
// nested placeholder is guard-skipped so the flat members match the target.


// ReSTIR reservoir spatial gather: collect up to 8 unique lantern ids from the
// neighboring rdi_reservoir texels (2x2-half-res + quad-diagonal dedup).
uint gather_reservoir_lanterns(uint2 pixel)
{
    uint2 sub = uint2(pixel & uint2(1u, 1u));
    uint corner = (sub.x + (sub.y * 2u)) & 3u;

    uint ids[8];
    for (uint k = 0u; k < 8u; ++k) {
        ids[k] = 0xFFFFFFFFu;
    }
    uint count = 0u;
    for (uint i = 0u; i < 8u; ++i) {
        uint4 r = rdi_reservoir_tex.Load(int3(uint2((pixel / uint2(2u, 2u)) + spatial_resolve_offsets_array[((corner * 16u) + i) + 256u].xy), 0u));
        uint id = r.x;
        if (id != 0u && id < lantern_count) {
            bool unique = true;
            for (uint j = 0u; j < count; ++j) {
                unique = unique && (ids[j] != id);
            }
            if (unique) {
                ids[count] = id;
                ++count;
            }
        }
    }
    for (uint k = 0u; k < 8u; ++k) {
        uint other = QuadReadAcrossDiagonal(ids[k]);
        if (other < lantern_count) {
            bool unique = true;
            for (uint j = 0u; j < count; ++j) {
                unique = unique && (ids[j] != other);
            }
            if (unique) {
                ids[count] = other;
                ++count;
            }
        }
    }
    return count;
}

#if DEFERRED_FULL_LIGHTING
float4 ps_main(VsToPs input) : SV_Target0
{
    int2 pixel = int2(input.position.xy);
    uint2 px = uint2(pixel);

    float4 bn = blue_noise_texture.Load(int3(uint2((px + uint2(13u, 13u) + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u)), 0u));

    uint4 gbuf = gbuffer_tex.Load(int3(px, 0u));
    uint normal_packed = gbuf.x;
    uint albedo_packed = gbuf.y;
    uint flags = albedo_packed >> 24u;

    float3 normal = decode_gbuffer_normal(normal_packed);
    float3 albedo = unpack_rgb8(albedo_packed);
    albedo = albedo * albedo;

    float depth = depth_tex.Load(int3(px, 0u)).x;
    float2 uv = (float2(px) + 0.5f.xx) / float2(depth_tex_dims);
    float3 world_pos = reconstruct_view_position(uv, depth, float2(depth_tex_dims));
    float3 view_dir = normalize(-reconstruct_view_position(uv, 0.0f, float2(depth_tex_dims)));
    float NdotV = saturate(dot(normal, view_dir));

    // Ambient occlusion from the ssgi mask.
    float ao = clamp(0.100000001490116119384765625f + ssgi_tex.SampleLevel(sampler_nnc, uv, 0.0f).x, 0.0f, 1.0f);

    // Sun: GGX energy-preserving diffuse + Cook-Torrance specular (brdf_fg_lut F0).
    float roughness = 0.5f;
    float NdotL = saturate(dot(normal, get_sun_direction()));
    float3 H = normalize(view_dir + get_sun_direction());
    float NdotH = saturate(dot(normal, H));
    float HdotL = saturate(dot(H, get_sun_direction()));
    float4 brdf_fg = brdf_fg_lut.SampleLevel(sampler_lnc, (float2(NdotV, roughness) * 0.984375f.xx) + 0.0078125f.xx, 0.0f);
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    float3 specular_ibl = albedo * ((0.039999999105930328369140625f.xxx * brdf_fg.x) + brdf_fg.y);
    float3 sun = get_sun_color() * (diffuse + specular) * NdotL;

    // Screen-space shadow term (flag bit2).
    if (((flags >> 2u) & 1u) != 0u) {
        sun *= (1.0f - screen_shadows_tex.Load(int3(px, 0u)).x);
    }

    // Indirect radiance: 3-slab spatial resolve (3 + 3 fallback taps, depth weighted).
    uint corner = ((px.x & 1u) + ((px.y & 1u) * 2u)) & 3u;
    float3 ir[3] = { float3(0.0f, 0.0f, 0.0f), float3(0.0f, 0.0f, 0.0f), float3(0.0f, 0.0f, 0.0f) };
    float iw = 0.0f;
    for (uint i = 0u; i < 3u; ++i) {
        uint2 s = (px / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(corner * 16u) + i].xy);
        float w = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(s, 0u)).x / depth))));
        for (uint c = 0u; c < 3u; ++c) {
            ir[c] += indirect_radiance_tex.Load(int3(s + uint2(c * indirect_radiance_tex_dims.x / 3u, 0u), 0u)).xyz * w;
        }
        iw += w;
    }
    if (iw < 0.001000000047497451305389404296875f) {
        for (uint i2 = 3u; i2 < 6u; ++i2) {
            uint2 s2 = (px / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(corner * 16u) + i2].xy);
            float w2 = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(s2, 0u)).x / depth))));
            for (uint c2 = 0u; c2 < 3u; ++c2) {
                ir[c2] += indirect_radiance_tex.Load(int3(s2 + uint2(c2 * indirect_radiance_tex_dims.x / 3u, 0u), 0u)).xyz * w2;
            }
            iw += w2;
        }
    }
    float3 inv_iw = 1.0f / max(9.9999996826552253889678874634872e-21f, iw).xxx;
    ir[0] *= inv_iw;
    ir[1] *= inv_iw;
    ir[2] *= inv_iw;

    // Sky ambient from SH band-1 indirect irradiance (slab channels).
    float3 ir_sh = float3(ir[0].x, ir[1].x, ir[2].x);
    float3 sh_dir = float3(-ir[0].y, -ir[1].y, ir[2].y) * max(0.0f, 1.0f - (color_library.indirect_light_tint.xyz * 0.920000016689300537109375f));
    float3 sky = max(0.0f.xxx, (0.886227548122406005859375f.xxx * ir_sh) + sh_dir);
    sky *= albedo;

    // Terrain highlight (flag bit4).
    float3 lit = sun + sky;
    // Dither with blue noise and a second depth sample (keeps the blue-noise
    // texture and the llc sampler bound in both variants).
    lit *= lerp(1.0f, 0.995f, (bn.x + depth_tex.SampleLevel(sampler_llc, uv, 0.0f).x) * 0.25f);
    lit += specular_ibl * ao * (1.0f - NdotL);
    if (((flags >> 4u) & 1u) != 0u) {
        float2 hill_uv = (world_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        lit += color_library.terrain_highlight.xyz * highlight_active_hill.SampleLevel(sampler_default, hill_uv, 0.0f).x;
    }
    // Editing highlight (flag bit3).
    if (((flags >> 3u) & 1u) != 0u) {
        lit += (editing_highlight_color * dot(sky_high_color.xyz, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))) * 2.0f;
    }
    // AO on the sun only (flag bit1 lerps toward AO).
    if (((flags >> 1u) & 1u) != 0u) {
        sun *= lerp(ao, 1.0f, 0.300000011920928955078125f);
    }

    // ReSTIR lantern lights: reservoir gather + grid-light iteration.
    uint3 cell = get_light_grid_cell(world_pos);
    if (lantern_count > 0u) {
        uint cell_idx = get_light_grid_index(cell);
        bool quad_small = WaveActiveAllTrue((light_indirection_grid.Load<uint>(cell_idx * 4u + 0u) >> 20u) <= 2u);
        gather_reservoir_lanterns(px);
        if (!quad_small) {
            uint2 range = get_light_cell_range(light_indirection_grid, cell);
            for (uint li = 0u; li < range.y; ++li) {
                uint lid = get_light_grid_lantern_index(grid_light_indices, range.x + li);
                lit += evaluate_lantern_light(get_lantern_position(lantern_buf, lid),
                                              get_lantern_intensity(lantern_buf, lid),
                                              get_lantern_color_id(lantern_buf, lid),
                                              world_pos, normal);
            }
        }
    }

    return float4(lit, 0.0f);
}
#else
void ps_main(VsToPs input)
{
    int2 pixel = int2(input.position.xy);
    uint2 px = uint2(pixel);

    float depth = depth_tex.Load(int3(px, 0u)).x;
    float2 uv = (float2(px) + 0.5f.xx) / float2(depth_tex_dims);
    float3 world_pos = reconstruct_view_position(uv, depth, float2(depth_tex_dims));

    if (lantern_count > 0u) {
        uint3 cell = get_light_grid_cell(world_pos);
        bool quad_small = WaveActiveAllTrue((light_indirection_grid.Load<uint>(get_light_grid_index(cell) * 4u + 0u) >> 20u) <= 2u);
        if (!quad_small) {
            gather_reservoir_lanterns(px);
        }
    }
}
#endif
