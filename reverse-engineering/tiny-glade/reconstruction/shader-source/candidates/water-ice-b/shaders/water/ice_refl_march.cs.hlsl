// ==RECONSTRUCTED== shaders/water/ice_refl_march.cs.hlsl
// Evidence: _water_ice_refl_march.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: $Globals b13 (14 members: dims + rt counts/history_valid), sampler_lnc s0, sampler_nnc
//   s2, ice_refl_count_buffer t25, ice_refl_px_buffer t26 (ByteAddressBuffers), half_depth_tex t27,
//   taa_history_tex t28, reprojection_tex t29, ice_roughness_tex t30, refl_hit_tex u31
//   (RWTexture2D<float2> rg16f), refl_hit_radiance_tex u32 (RWTexture2D<uint> r32ui),
//   __alias_ice_rt_count_buffer u33 / __alias_ice_rt_ray_buffer u34 (RWByteAddressBuffers),
//   frame_constants set1/b0. LocalSize [64,1,1], entry cs_main.
//   spirv-cross + disassembly: ice reflection ray march. For each refl pixel (half-res): builds the
//   camera ray, finds the y=0 plane intersection, importance-samples a reflection direction over the
//   roughness lobe (blue-noise golden-ratio + Hilbert-curve index over a 64x64 tile, up to 8 retries),
//   clips the segment against the clip box, DDA-marches the half-res depth buffer (~16 steps + 3
//   bisection iterations) with a depth-crossing test, then either reads TAA history at the
//   reprojected uv (hit) or appends a packed ray to the ice RT ray buffer for ice_deferred_rt; writes
//   refl_hit_tex (dist, roughness) and refl_hit_radiance_tex (3x9-bit mantissa + 5-bit exponent).
// Status: O (algorithm + bindings from SPIR-V; Hilbert/blue-noise chain transcribed from disassembly).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "water/ice_shared.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_lighting_texture_dims;
    uint2 blue_noise_texture_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
    uint history_valid;
    uint2 half_depth_tex_dims;
    uint2 taa_history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 ice_roughness_tex_dims;
    uint2 refl_hit_tex_dims;
    uint2 refl_hit_radiance_tex_dims;
};

ByteAddressBuffer ice_refl_count_buffer : register(t25, space0);
ByteAddressBuffer ice_refl_px_buffer : register(t26, space0);
RWByteAddressBuffer __alias_ice_rt_count_buffer : register(u33, space0);
RWByteAddressBuffer __alias_ice_rt_ray_buffer : register(u34, space0);
Texture2D<float4> half_depth_tex : register(t27, space0);
Texture2D<float4> taa_history_tex : register(t28, space0);
Texture2D<float4> reprojection_tex : register(t29, space0);
Texture2D<float4> ice_roughness_tex : register(t30, space0);
[[vk::image_format("rg16f")]] RWTexture2D<float2> refl_hit_tex : register(u31, space0);
[[vk::image_format("r32ui")]] RWTexture2D<uint> refl_hit_radiance_tex : register(u32, space0);

uint pack_half2(float2 v)
{
    uint2 p = f32tof16(v);
    return p.x | (p.y << 16u);
}

// Hilbert-curve index over a 64x64 tile (bit-reversal loop transcribed from the disassembly);
// decorrelates the blue-noise reflection samples per pixel.
uint ice_hilbert_index(uint2 p)
{
    uint x = p.x;
    uint y = p.y;
    uint idx = 0u;
    for (uint m = 32u; m > 0u; m /= 2u)
    {
        uint yb = (y & m) > 0u ? 1u : 0u;
        uint xb = (x & m) > 0u ? 1u : 0u;
        idx += (m * m) * ((3u * yb) ^ xb);
        if (xb == 0u)
        {
            if (yb == 1u)
            {
                x = 63u - x;
                y = 63u - y;
            }
        }
        else
        {
            uint tx = x;
            x = y;
            y = tx;
        }
    }
    return idx;
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x >= ice_refl_count_buffer.Load<uint>(12))
    {
        return;
    }
    uint px_packed = ice_refl_px_buffer.Load<uint>(gl_GlobalInvocationID.x * 4u);
    uint2 px = uint2(px_packed & 65535u, (px_packed >> 16u) & 65535u);

    float2 clip = ((float2(int2(px)) + 0.5f) / float2(half_depth_tex_dims) - 0.5f) * float2(2.0f, -2.0f);
    float3 ray_dir = normalize(mul(view_constants[0].view_to_world,
                                   mul(view_constants[0].sample_to_view, float4(clip, 0.0f, 1.0f))).xyz);
    float4 cam4 = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam4.xyz / cam4.w;
    // Intersection of the camera ray with the y = 0 water plane.
    float3 plane_pt = cam_pos + (ray_dir * ((-cam_pos.y) / ray_dir.y));

    // View-space plane point (projected back).
    float4 plane_view = mul(view_constants[0].view_to_sample,
                            mul(view_constants[0].world_to_view, float4(plane_pt, 1.0f)));
    float4 view_pos = float4(clip, (plane_view.xyz / plane_view.w.xxx).z, 1.0f);

    uint frame_mod = frame_index % 64u;
    float frame_rand = 5.588237762451171875f * float(frame_mod);
    float rng = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px.x) + frame_rand))
        + (0.005837149918079376220703125f * (float(px.y) + frame_rand))));

    float rough = ice_roughness_tex.Load(int3(px * uint2(2u, 2u), 0)).x;
    float rough2 = max(0.000600000028498470783233642578125f, rough * rough);
    float3 basis = mul(-ray_dir, float3x3(float3(1.0f, -0.0f, 0.0f),
                                          float3(-0.0f, 0.0f, 1.0f),
                                          float3(-0.0f, -1.0f, 0.0f)));

    // Importance-sample the reflection lobe; retry while the reflection points below the horizon.
    float3 sample_dir = float3(0.0f, 1.0f, 0.0f);
    uint it = 0u;
    for (; it < 8u; ++it)
    {
        uint hilbert = ice_hilbert_index(px);
        float2 bn = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
            * float(hilbert + (288u * frame_mod))) + 0.5f);

        float nz = basis.z;
        float3 n = normalize(float3(rough2 * basis.x, rough2 * basis.y, nz));
        float3 t = (n.z < 0.99989998340606689453125f) ? normalize(cross(float3(0.0f, 0.0f, 1.0f), n))
                                                       : float3(1.0f, 0.0f, 0.0f);
        float r = sqrt(bn.x);
        float phi = bn.y * 3.1415927410125732421875f;
        float cx = r * cos(phi);
        float cz = 0.5f * (1.0f + n.z);
        float cy = ((1.0f - cz) * sqrt(1.0f - (cx * cx))) + (cz * (r * sin(phi)));
        float3 dir = ((t * cx) + (cross(n, t) * cy)) + (n * sqrt(max(0.0f, (1.0f - (cx * cx)) - (cy * cy))));
        float3 nd = normalize(float3(rough2 * dir.x, rough2 * dir.y, max(0.0f, dir.z)));

        bool bad = (nd.z <= 9.9999997473787516355514526367188e-06f)
            || (reflect(-basis, nd).z <= 9.9999997473787516355514526367188e-06f)
            || (nz <= 9.9999997473787516355514526367188e-06f);
        if (bad)
        {
            sample_dir = float3(0.0f, 0.0f, 0.0f);
            break;
        }
        if (reflect(basis, nd).z < 0.0f)
        {
            sample_dir = nd;
            break;
        }
    }

    float3 refl = -reflect(-ray_dir, mul(float3x3(float3(1.0f, -0.0f, 0.0f),
                                                  float3(-0.0f, 0.0f, 1.0f),
                                                  float3(-0.0f, -1.0f, 0.0f)), sample_dir));
    if (refl.y < 0.0f)
    {
        refl.y = -refl.y;
    }

    // Segment from the plane point along the reflection, clipped to the clip-space unit box.
    float3 origin = view_pos.xyz;
    float4 end4 = view_pos + mul(view_constants[0].view_to_clip,
                                 mul(view_constants[0].world_to_view, float4(refl, 0.0f)));
    float end_w = end4.w;
    float3 end = (end4 / ((end_w >= 0.0f ? 1.0f : -1.0f) * max(1.0000000133514319600180897396058e-10f, abs(end_w)))).xyz;
    float3 delta = end - origin;
    float3 t0 = (float3(delta.x < 0.0f ? 1.0f : -1.0f, delta.y < 0.0f ? 1.0f : -1.0f, delta.z < 0.0f ? 1.0f : -1.0f) - origin) / delta;
    float3 enter = origin + (delta * max(0.0f, max(t0.x, t0.y)));
    float3 delta2 = end - enter;
    float3 t1 = (float3(delta2.x >= 0.0f ? 1.0f : -1.0f, delta2.y >= 0.0f ? 1.0f : -1.0f, delta2.z >= 0.0f ? 1.0f : -1.0f) - enter) / delta2;
    float3 exit = enter + (delta2 * min(min(t1.x, t1.y), t1.z));

    // DDA march over the half-res depth buffer along the segment.
    float2 uv_a = (enter.xy * float2(0.5f, -0.5f)) + 0.5f;
    float2 uv_b = (exit.xy * float2(0.5f, -0.5f)) + 0.5f;
    uint steps = uint(int(max(2u, min(16u, uint(int(floor(length((uv_b - uv_a) * float2(half_depth_tex_dims)))))))));
    float step_count = (exit.z > 9.5367431640625e-07f) ? float(steps) : max(1.0f, float(steps) - 1.0f);
    float inv_steps = 1.0f / step_count;
    float3 seg = exit - enter;
    float4x4 clip_to_view_m = view_constants[0].clip_to_view;

    // Find the first depth crossing (sampled half depth in front of the segment point).
    bool crossed = false;
    float lo_t = 0.0f;
    float lo_d = 0.0f;
    float hi_d = 0.0f;
    float hi_den = 0.0f;
    float hi_plane = 0.0f;
    float hi_t = 0.0f;
    float prev_t = 0.0f;
    float prev_d = 0.0f;

    float t0s = lerp(0.0f, 1.0f, pow(rng * inv_steps, 2.0f));
    float3 p0 = enter + (seg * t0s);
    float z0 = p0.z;
    float half0 = half_depth_tex.SampleLevel(sampler_nnc, (p0.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
    float d0 = (1.00000202655792236328125f / half0) - (1.0f / z0);
    if (d0 < 0.0f)
    {
        crossed = true;
        lo_t = 0.0f;
        lo_d = 0.0f;
        hi_d = d0;
        hi_den = (1.0f / z0) - (1.0f / half0);
        hi_plane = clip_to_view_m[3].z;
        hi_t = t0s;
    }
    else
    {
        prev_t = t0s;
        prev_d = d0;
        for (uint k = 1u; k < steps; ++k)
        {
            float t = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(k) + rng) * inv_steps, 2.0f)));
            float3 p = enter + (seg * t);
            float z = p.z;
            float half_d = half_depth_tex.SampleLevel(sampler_nnc, (p.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
            float d = (1.00000202655792236328125f / half_d) - (1.0f / z);
            if (d < 0.0f)
            {
                crossed = true;
                lo_t = prev_t;
                lo_d = prev_d;
                hi_d = d;
                hi_den = (1.0f / z) - (1.0f / half_d);
                hi_plane = clip_to_view_m[3].z;
                hi_t = t;
                break;
            }
            prev_t = t;
            prev_d = d;
        }
    }

    bool found = false;
    float t_hit = prev_t;
    float d_ref = prev_d;
    float den_ref = 0.0f;
    float plane_ref = 0.0f;

    if (crossed)
    {
        // Bisection refine over [lo_t, hi_t].
        float a = lo_t;
        float b = lo_d;
        float c = hi_d;
        float e = hi_t;
        float dden = hi_den;
        float pplane = hi_plane;
        for (uint j = 0u; j < 3u; ++j)
        {
            float mid = (a + e) * 0.5f;
            float3 p = enter + (seg * mid);
            float z = p.z;
            float half_d = half_depth_tex.SampleLevel(sampler_nnc, (p.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
            float dm = (1.00000202655792236328125f / half_d) - (1.0f / z);
            if (dm < 0.0f)
            {
                a = a;
                b = b;
                c = dm;
                dden = (1.0f / z) - (1.0f / half_d);
                pplane = clip_to_view_m[3].z;
                e = mid;
            }
            else
            {
                a = mid;
                b = dm;
                e = e;
            }
        }
        float refined = lerp(a, e, b / (b + (-c)));
        float3 pr = enter + (seg * refined);
        float zr = pr.z;
        float half_r = half_depth_tex.SampleLevel(sampler_nnc, (pr.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
        float dr = (1.00000202655792236328125f / half_r) - (1.0f / zr);
        float den_r = (1.0f / zr) - (1.0f / half_r);
        if (abs(dr) < (b * 0.89999997615814208984375f))
        {
            found = true;
            t_hit = refined;
            d_ref = dr;
            den_ref = den_r;
            plane_ref = clip_to_view_m[3].z;
        }
        else
        {
            found = true;
            t_hit = e;
            d_ref = c;
            den_ref = dden;
            plane_ref = pplane;
        }
        // Plane comparison gate (near-plane depth).
        if (!(den_ref < plane_ref && d_ref < plane_ref))
        {
            found = false;
        }
    }

    float3 hit_view = lerp(enter, exit, t_hit.xxx);
    float hit_dist;
    float3 radiance;

    if (found)
    {
        float4 world4 = mul(view_constants[0].view_to_world, mul(clip_to_view_m, float4(hit_view, 1.0f)));
        float3 history = 0.0f;
        if (history_valid != 0u && frame_index != 0u)
        {
            float2 h_uv = lerp(uv_a, uv_b, t_hit.xx);
            history = taa_history_tex.SampleLevel(sampler_lnc, reprojection_tex.SampleLevel(sampler_nnc, h_uv, 0.0f).xy + h_uv, 0.0f).xyz;
        }
        hit_dist = distance((world4.xyz / world4.w.xxx), plane_pt);
        radiance = history;
    }
    else
    {
        float4 view4 = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4(hit_view, 1.0f)));
        float3 world = view4.xyz / view4.w.xxx;
        uint ray_idx;
        __alias_ice_rt_count_buffer.InterlockedAdd(12, 1u, ray_idx);
        float3 ray_dir_world = world - cam_pos;
        __alias_ice_rt_ray_buffer.Store<uint4>(ray_idx * 16u,
            uint4(px_packed,
                  pack_half2(float2(ray_dir_world.x, 0.0f)) | (pack_half2(float2(refl.x, 0.0f)) << 16u),
                  pack_half2(float2(ray_dir_world.y, 0.0f)) | (pack_half2(float2(refl.y, 0.0f)) << 16u),
                  pack_half2(float2(ray_dir_world.z, 0.0f)) | (pack_half2(float2(refl.z, 0.0f)) << 16u)));
        if ((ray_idx & 63u) == 0u)
        {
            uint g;
            __alias_ice_rt_count_buffer.InterlockedAdd(0, 1u, g);
        }
        hit_dist = distance(world, plane_pt);
        radiance = 0.0f;
    }

    refl_hit_tex[px] = float2(hit_dist, rough2);

    // Pack radiance into a 3x9-bit mantissa + 5-bit shared exponent word.
    float3 clamp_rad = clamp(radiance, 0.0f, 65408.0f);
    float exp_f = asfloat((asuint(max(max(1.52587890625e-05f, clamp_rad.x), max(clamp_rad.y, clamp_rad.z)))
        + 125845504u) & 2139095040u);
    uint3 mant = asuint(clamp_rad + exp_f);
    refl_hit_radiance_tex[px] = (((asuint(exp_f) << 4u) + 268435456u) | (mant.z << 18u))
        | (mant.y << 9u) | (mant.x & 511u);
}
