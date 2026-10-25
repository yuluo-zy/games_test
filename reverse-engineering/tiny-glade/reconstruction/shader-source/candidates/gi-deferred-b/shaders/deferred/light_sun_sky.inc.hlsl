// ==RECONSTRUCTED== shaders/deferred/light_sun_sky.inc.hlsl (candidate gi-deferred-b)
// Evidence: embedded as include in 8 bins (deferred_{canopy,grass,plant,solid}_light_sun_sky);
// direct include edge light_sun_sky.inc.hlsl -> inc/rt.hlsl. Shared deferred sun+sky
// lighting for the four fullscreen composition passes. The observed math (from the
// spirv-cross of _deferred_solid_light_sun_sky ps_main) is:
//   - world position / normal reconstructed from gbuffer+depth (RGB8 normal decode)
//   - sun: energy-preserving GGX diffuse + Cook-Torrance specular * sun_color * NdotL
//   - sky ambient: SH band-1 eval of the indirect radiance 3-slab spatial resolve
//   - ReSTIR lantern light: reservoir gather + stream sampling (inc/reservoir.hlsl)
//   - grid lights: light_indirection_grid / grid_light_indices iteration
// Helper names are Inference (inlined in the module).
// Status: O (algorithm from use-sites); I (function names).

#ifndef TINYGLADE_DEFERRED_LIGHT_SUN_SKY_INC_HLSL
#define TINYGLADE_DEFERRED_LIGHT_SUN_SKY_INC_HLSL

#include "inc/rt.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/lighting_indirect.hlsl"
#include "inc/reservoir.hlsl"
#include "inc/lanterns.hlsl"
#include "gi/gi_settings.inc.hlsl"

// Reconstruct the view-space position of a pixel from its depth and the sample
// UV. view_constants[0].sample_to_view maps [0,1]x[0,1] uv to a view ray.
float3 reconstruct_view_position(float2 uv, float depth, float2 depth_dims)
{
    float2 ndc = (uv - 0.5f.xx) * float2(2.0f, -2.0f);
    float4 view_pos = mul(view_constants[0u].sample_to_view, float4(ndc, depth, 1.0f));
    return view_pos.xyz / view_pos.w;
}

// Decode the gbuffer normal (RGB8, xyz stored as n*0.5+0.5) and the packed flags
// byte (top byte of the albedo word): bit0 = velocity/skylight?, bit1 = ?, bit2 =
// screen-space shadows, bit3 = editing highlight, bit4 = hill highlight, bit5 = ssgi,
// bit6 = canopy-up boost. Names are Inference.
float3 decode_gbuffer_normal(uint packed)
{
    float3 n = float3(float(packed & 255u),
                      float((packed >> 8u) & 255u),
                      float((packed >> 16u) & 255u)) * (1.0f / 255.0f);
    return normalize(n * 2.0f - 1.0f);
}

// Sun diffuse+specular for a surface with a given roughness/albedo (matches the
// observed GGX energy-preserving diffuse and the Cook-Torrance specular).
float3 evaluate_deferred_sun_light(float3 albedo, float roughness, float3 normal,
                                   float3 view_dir, float3 sun_dir, float NdotV)
{
    float NdotL = saturate(dot(normal, sun_dir));
    float3 H = normalize(view_dir + sun_dir);
    float NdotH = saturate(dot(normal, H));
    float HdotL = saturate(dot(H, sun_dir));
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    return get_sun_color() * (diffuse + specular) * NdotL;
}

// Sample the 3-slab indirect radiance texture at a half-res position (slab
// stride = dims.x per channel). Returns the weighted SH band-0/1 irradiance.
float3 sample_indirect_radiance(Texture2D indirect_radiance_tex, uint2 half_pixel, uint2 dims)
{
    float3 ir = float3(indirect_radiance_tex.Load(int3(half_pixel + uint2(0u * dims.x, 0u), 0u)).x,
                       indirect_radiance_tex.Load(int3(half_pixel + uint2(1u * dims.x, 0u), 0u)).x,
                       indirect_radiance_tex.Load(int3(half_pixel + uint2(2u * dims.x, 0u), 0u)).x);
    return ir;
}

#endif // TINYGLADE_DEFERRED_LIGHT_SUN_SKY_INC_HLSL
