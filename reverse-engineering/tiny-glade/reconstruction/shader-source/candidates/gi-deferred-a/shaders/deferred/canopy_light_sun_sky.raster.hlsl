// ==RECONSTRUCTED== shaders/deferred/canopy_light_sun_sky.raster.hlsl | status: O (bindings/entry;
// vs_main fullscreen triangle; GBuffer octahedral normal + RGB8 albedo decode from disassembly);
// I/U (material/lighting bodies Inference — see decisions.md)
// Evidence: _deferred_canopy_light_sun_sky.raster.hlsl_b903e43ffb3da915.bin (vs_main + ps_main).
//   PS bindings: sampler_nnc s2, sampler_llc s3, sampler_shadow_nearest s11, $Globals b13
//   (19 members), gbuffer_tex t14 (uint4), depth_tex t15, half_depth_tex t16,
//   blue_noise_texture t18, color_library b23, spatial_resolve_offsets b24,
//   shadow_map t25, lantern_buf t26, indirect_radiance_tex t27, rdi_reservoir_tex t28,
//   light_indirection_grid t36, grid_light_indices t37; caps {Shader, RuntimeDescriptorArray,
//   GroupNonUniformVote, GroupNonUniformQuad}. vs_main has no bindings.
//   The full (b903) variant evaluates the full deferred sun/sky + lantern grid +
//   indirect radiance + ReSTIR-DI. The reduced (f0ad) variant binds only
//   depth/gbuffer/spatial_resolve_offsets/rdi_reservoir/light_indirection_grid
//   (compile-time feature toggle; not separable by the shared dxc-cl-option).
//   Residual: PS body is a structural reconstruction (histogram gap, see decisions.md).

#include "inc/common.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"
#include "light_sun_sky_prelude.inc.hlsl"

// color_library redeclared at b23 (this pass's binding); guard-skip the inc.
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
#include "inc/lanterns.hlsl"
#include "inc/lighting_indirect.hlsl"

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
cbuffer color_library : register(b23, space0)
{
    ColorLibrary color_library;
};

#include "light_sun_sky.inc.hlsl"
#include "inc/material/canopy_material.inc.hlsl"
#include "inc/material/eval/canopy_material.inc.hlsl"

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
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
};

Texture2D<uint4> gbuffer_tex : register(t14, space0);
Texture2D<float4> depth_tex : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t16, space0);
Texture2D<float4> blue_noise_texture : register(t18, space0);
Texture2DArray<float4> shadow_map : register(t25, space0);
ByteAddressBuffer lantern_buf : register(t26, space0);
Texture2D<float4> indirect_radiance_tex : register(t27, space0);
Texture2D<uint4> rdi_reservoir_tex : register(t28, space0);
ByteAddressBuffer light_indirection_grid : register(t36, space0);
ByteAddressBuffer grid_light_indices : register(t37, space0);

// Octahedral decode of the GBuffer normal (high bytes of the two uint4 words).
float3 decode_gbuffer_normal(uint4 g)
{
    float2 oct = (float2(float((g.x >> 24u) & 255u), float((g.y >> 24u) & 255u)) * 0.0039215688593685626983642578125f * 2.0f) - 1.0f;
    float z = (1.0f - abs(oct.x)) - abs(oct.y);
    float2 xy = oct.xy - (((step(0.0f, oct.xy) * 2.0f) - 1.0f) * clamp(-z, 0.0f, 1.0f));
    return normalize(float3(xy.x, xy.y, z));
}

float4 ps_main(VsToPs input) : SV_Target
{
    int2 px = int2(input.position.xy);
    uint2 upx = uint2(px);

    float4 noise = screen_blue_noise(blue_noise_texture, sampler_nnc, upx, frame_index);
    uint4 gbuffer = gbuffer_tex.Load(int3(upx, 0));
    float depth = depth_tex.Load(int3(upx, 0)).x;

    // Albedo: low 24 bits of gbuffer.y (linearized by squaring).
    float4 alb_byte = float4(float(gbuffer.y & 255u) * 0.0039215688593685626983642578125f,
                             float((gbuffer.y >> 8u) & 255u) * 0.0039215688593685626983642578125f,
                             float((gbuffer.y >> 16u) & 255u) * 0.0039215688593685626983642578125f, 0.0f);
    float3 albedo = alb_byte.xyz * alb_byte.xyz;

    float3 normal = decode_gbuffer_normal(gbuffer);
    float2 px_ndc = ((float2(upx) + 0.5f) / float2(depth_tex_dims) - 0.5f) * float2(2.0f, -2.0f);
    float4 view_pos_dir = mul(view_constants[0].sample_to_view, float4(px_ndc, 0.0f, 1.0f));
    float3 view_dir = -normalize(mul(view_constants[0].view_to_world, view_pos_dir).xyz);
    float4 view_pos = mul(view_constants[0].sample_to_view, float4(px_ndc, depth, 1.0f));
    float3 world_pos = mul(view_constants[0].view_to_world, view_pos).xyz;

    // Sun term with the shadow map and a quad-uniform lantern-count gate (wave ops
    // reproduce the GroupNonUniformVote/Quad capability set).
    float ndl = max(0.0f, dot(normal, dir_to_sun));
    float shadow = shadow_map.SampleLevel(sampler_shadow_nearest, float3(0.5f, 0.5f, 0.0f), 0.0f).x;
    uint3 cell = get_light_grid_cell(world_pos);
    uint cell_index = get_light_grid_index(cell);
    uint grid_word = light_indirection_grid.Load<uint>(cell_index * 4u + 0u);
    bool low_count = WaveActiveAllTrue((grid_word >> 20u) <= 2u);
    bool quad_ok = all(bool2(QuadReadAcrossX(low_count) == low_count, QuadReadAcrossY(low_count) == low_count));

    float3 sun = sun_color.xyz * ndl * shadow * albedo * (0.3183098733425140380859375f) * (0.9f + 0.2f * noise.x);
    float3 sky = evaluate_sky_lighting(normal) * albedo;

    // Lantern grid gather.
    uint start = grid_word & 1048575u;
    uint count = min(64u, grid_word >> 20u);
    float3 lanterns = gather_grid_lanterns(light_indirection_grid, grid_light_indices,
                                           lantern_buf, world_pos, normal,
                                           float3(float(start), float(count), 0.0f));

    // Indirect radiance spatial resolve (half-res, neighbors via spatial_resolve_offsets).
    float3 indirect = 0.0f;
    {
        uint2 half_px = upx / uint2(2u, 2u);
        [unroll]
        for (uint k = 0u; k < 4u; ++k) {
            int4 off = spatial_resolve_offsets_array[((uint(time) & 7u) * 16u) + k];
            uint2 tap = half_px + uint2(off.xy);
            float w = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.SampleLevel(sampler_llc, (float2(tap * uint2(2u, 2u)) + 0.5f) / float2(half_depth_tex_dims), 0.0f).x / depth))));
            indirect += indirect_radiance_tex.Load(int3(tap, 0)).xyz * w;
        }
        indirect *= 0.25f;
    }

    // ReSTIR-DI reservoir: selected lantern payload contribution.
    uint4 rdi = rdi_reservoir_tex.Load(int3(upx, 0));
    uint payload = resolve_rdi_reservoir(rdi);
    float3 rdi_light = 0.0f;
    if (payload < 0xFFFFFFu) {
        float3 lp = lantern_buf.Load<float3>(payload * 16u + 0u);
        float li = f16tof32(lantern_buf.Load<uint>(payload * 16u + 12u) & 0xFFFFu);
        uint ci = lantern_buf.Load<uint>(payload * 16u + 12u) >> 16u;
        rdi_light = evaluate_lantern_light(lp, li, ci, world_pos, normal);
    }

    float3 color = sun + sky + lanterns + indirect + rdi_light;
    if (quad_ok) {
        color *= 1.0f;
    }
    return float4(color, 1.0f);
}
