// ==RECONSTRUCTED== shaders/water/water_surface_frag.inc.hlsl | evidence: _water_plane.raster.hlsl
// and _waterfall.raster.hlsl PS modules (first_line = this file) + spirv-cross: shared water-surface
// fragment shader. Reads the wave normal (rotating 4-octave water_noise_texture + ripple sources),
// reconstructs the view ray at the pixel, DDA-marches the half-res depth buffer for the refracted
// scene distance, applies PCSS shadow + DoF focus, blends refraction (exp falloff by water color)
// with sky/sun specular (Schlick-ish fresnel), adds foam near the mask edge (glade_theme 4), and
// emits a deferred reflection ray into the water RT ray buffer (packed px / half dir / length /
// depth+flags) consumed by water/deferred_rt.cs.hlsl. status: O (structure + bindings + final
// composite from SPIR-V); O/I (condensed inner loops / helper names — inlined in SPIR-V).

#ifndef TINYGLADE_WATER_WATER_SURFACE_FRAG_INC_HLSL
#define TINYGLADE_WATER_WATER_SURFACE_FRAG_INC_HLSL

#include "inc/samplers.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "water/waves.inc.hlsl"

uint pack_half2x(float2 v)
{
    uint2 p = f32tof16(v);
    return p.x | (p.y << 16u);
}

[earlydepthstencil]
float4 ps_main(float3 c1 : TEXCOORD1, float4 gl_FragCoord : SV_Position) : SV_Target0
{
    bool helper = IsHelperLane();
    float3 world_pos = c1;
    uint2 px = uint2(gl_FragCoord.xy);

    // Fast path: the "no water" theme (4 = not-winter? observed as a full normal override).
    float3 surface_normal = float3(0.0f, 1.0f, 0.0f);
    float3 wave_slope = 0.0f;
    float3 ripple = 0.0f;
    if (glade_theme_id != 4u)
    {
        float time = float(game_time_sec) + game_time_frac;
        float2 pixel_seed = px;
        wave_slope = water_wave_normal(world_pos.xz, time, pixel_seed);
        ripple = water_ripple_accum(world_pos.xz, time);
        surface_normal = normalize(float3(wave_slope.y + ripple.y, 1.0f, wave_slope.z + ripple.z));
    }
    float _ = QuadReadAcrossX(surface_normal.x); // wave-lane cap parity (see header)

    // View ray + half-res depth.
    float2 half_px = float2(uint2(gl_FragCoord.xy) / uint2(2u, 2u));
    float2 clip = (((half_px + 0.5f) / float2(half_depth_tex_dims)) - 0.5f) * float2(2.0f, -2.0f);
    float3 view_dir = -normalize(mul(view_constants[0].view_to_world,
                                     mul(view_constants[0].sample_to_view, float4(clip, 0.0f, 1.0f))).xyz);
    float4 half_world = mul(view_constants[0].view_to_world,
                            mul(view_constants[0].sample_to_view, float4(clip, half_depth_tex.Load(int3(uint2(half_px), 0)).x, 1.0f)));
    float3 half_pos = half_world.xyz / half_world.w;

    // Reflected direction (up-flipped).
    float3 refl = reflect(view_dir, surface_normal);
    if (refl.y < 0.0f)
    {
        refl.y = -refl.y;
    }

    // Ray from the surface point toward the reflected direction, clipped to the clip box.
    float4 ray_end = mul(view_constants[0].view_to_sample,
                         mul(view_constants[0].world_to_view,
                             float4(world_pos + (refl * min(4.0f, 4.0f * length(world_pos - half_pos))), 1.0f)));
    float3 end = (ray_end.xyz / ray_end.w) * float(int(sign(ray_end.w)));
    float3 origin = float4(clip, gl_FragCoord.z, 1.0f).xyz;
    float3 delta = end - origin;
    float3 t0 = (float3(delta.x < 0.0f ? 1.0f : -1.0f, delta.y < 0.0f ? 1.0f : -1.0f, delta.z < 0.0f ? 1.0f : -1.0f) - origin) / delta;
    float3 enter = origin + (delta * max(0.0f, max(t0.x, t0.y)));
    float3 delta2 = end - enter;
    float3 t1 = (float3(delta2.x >= 0.0f ? 1.0f : -1.0f, delta2.y >= 0.0f ? 1.0f : -1.0f, delta2.z >= 0.0f ? 1.0f : -1.0f) - enter) / delta2;
    float3 exit = enter + (delta2 * min(1.0f, min(min(t1.x, t1.y), t1.z)));

    // Occlusion distance along the reflected ray (half-res depth DDA, few steps).
    float occl_t = 0.0f;
    {
        float2 uv_a = (enter.xy * float2(0.5f, -0.5f)) + 0.5f;
        float2 uv_b = (exit.xy * float2(0.5f, -0.5f)) + 0.5f;
        uint steps = uint(int(max(2u, min(3u, uint(int(floor(length((uv_b - uv_a) * float2(half_depth_tex_dims)))))))));
        float rng = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px.x) + (5.588237762451171875f * float(frame_index % 64u))))
            + (0.005837149918079376220703125f * (float(px.y) + (5.588237762451171875f * float(frame_index % 64u))))));
        float3 seg = exit - enter;
        float step_t = (exit.z > 9.5367431640625e-07f) ? float(steps) : max(1.0f, float(steps) - 1.0f);
        float inv = 1.0f / step_t;
        float prev_t = 0.0f;
        float prev_d = 0.0f;
        for (uint k = 0u; k < steps; ++k)
        {
            float t = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(k) + rng) * inv, 1.0f)));
            float3 p = enter + (seg * t);
            float inv_z = 1.0f / p.z;
            float half_d = half_depth_tex.SampleLevel(sampler_nnc, (p.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
            float d = (1.00000202655792236328125f / half_d) - inv_z;
            if (d < 0.0f)
            {
                occl_t = t;
                break;
            }
        }
    }

    // Refraction uv offset by the wave slope (view-space).
    float2 refr_uv = (((world_pos.xz + (surface_normal.xz * 0.05f)) * 0.0076923076994717121124267578125f) + 0.5f);
    float3 refr = refraction_tex.SampleLevel(sampler_lnc, refr_uv, 0.0f).xyz;

    // Sun specular + sky.
    float3 sun_dir = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float fres = 1.0f - dot(view_dir, surface_normal);
    float fres2 = fres * fres;
    float fres5 = 0.96f * (fres2 * fres2 * fres);
    float3 spec = sun_color.xyz * pow(saturate(dot(reflect(view_dir, surface_normal), sun_dir)), 60.0f);

    float3 sky = lerp(sky_low_color.xyz, sky_high_color.xyz,
        pow(clamp(1.0f - pow(1.0f - clamp(refl.y + 0.2f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.65f).xxx);

    // Distance-fogged refraction + specular.
    float dist = occl_t + length(world_pos - half_pos) + 0.7f;

    // PCSS-style sun shadow over the water (cascade 2 projection).
    float3 sun_proj = mul(view_constants[2].world_to_sample,
                          float4(world_pos + (surface_normal * 0.1f), 1.0f)).xyz;
    float3 shadow_uv3 = float3((sun_proj.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f);
    float nearest_depth = shadow_map.SampleLevel(sampler_shadow_nearest, shadow_uv3, 0.0f).x;
    float shadow = shadow_map.SampleCmpLevelZero(sampler_shadow_linear, shadow_uv3, sun_proj.z);
    float blocker = (nearest_depth > sun_proj.z) ? 1.0f : 0.0f;
    shadow = min(shadow + blocker * 0.5f, 1.0f);

    // DoF focus gate (autofocus history buffer, per camera).
    float focus = autofocus_buf.Load<float>(camera_idx * 64u + 0u);
    float dof_gate = smoothstep(0.0f, 1.0f, saturate(dist / max(focus, 1e-3f)));

    float3 col = (exp(-water_color * dist) * (0.96f - fres5) * refr * (1.0f + ((length(wave_slope) * length(wave_slope) * length(wave_slope)))))
        + ((sky + (spec * shadow)) * (0.04f + fres5));

    // Temporal blend against the TAA history (reprojected).
    float2 hist_uv = reprojection_tex.SampleLevel(sampler_nnc, refr_uv, 0.0f).xy + refr_uv;
    if (history_valid != 0u && frame_index != 0u)
    {
        col = lerp(col, taa_history_tex.SampleLevel(sampler_lnc, hist_uv, 0.0f).xyz, 0.1f);
    }
    col *= (0.5f + 0.5f * dof_gate);

    // Foam near the mask edge (theme 4).
    float3 foam = 0.0f;
    if (glade_theme_id == 4u)
    {
        float mask = water_mask.SampleLevel(sampler_llc, (world_pos.xz * 0.0076923076994717121124267578125f) + 0.5f, 0.0f).y;
        float2 grid = world_pos.xz * 1.0f;
        int2 cell = int2(floor(grid));
        float2 f = frac(grid);
        float2 sm = f * f * (3.0f - 2.0f * f);
        uint2 base = uint2(cell);
        float h00 = asfloat((((base.x * 1664525u) + base.y) & 8388607u) | 1065353216u) - 1.0f;
        foam = lerp(0.0f, 2.0f, (clamp((1.0f - smoothstep(0.7f, 0.85f, mask + ((h00 * h00) * 0.2f))) * 0.2f, 0.0f, 1.0f) + 0.1f).xxx);
    }

    // Emit a deferred reflection ray (consumed by water/deferred_rt.cs.hlsl).
    if (glade_theme_id != 4u)
    {
        uint ray_idx;
        __alias_water_rt_count_buffer.InterlockedAdd(0, 1u, ray_idx);
        float3 ray_world = (mul(view_constants[0].view_to_world,
                                mul(view_constants[0].sample_to_view, float4(clip, gl_FragCoord.z, 1.0f))).xyz
            / mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4(clip, gl_FragCoord.z, 1.0f))).w.xxx);
        float3 ray_dir_world = normalize(ray_world - world_pos);
        __alias_water_rt_ray_buffer.Store<uint4>(ray_idx * 16u,
            uint4(px.x + (px.y * 65536u),
                  pack_half2x(float2(ray_dir_world.x, ray_dir_world.y)),
                  asuint(dot(refl, (half_pos - world_pos))),
                  (asuint(gl_FragCoord.z) & 4294967280u) + (px.x & 3u)));
    }

    return float4(col + foam, 0.0f);
}

#endif // TINYGLADE_WATER_WATER_SURFACE_FRAG_INC_HLSL
