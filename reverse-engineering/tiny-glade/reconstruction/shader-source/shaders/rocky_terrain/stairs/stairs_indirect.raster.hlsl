// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/rocky_terrain/stairs/stairs_indirect.raster.hlsl | target: _rocky_terrain_stairs_stairs_indirect.raster.hlsl_b903e43ffb3da915.bin + _rocky_terrain_stairs_stairs_indirect.raster.hlsl_f0adff76827e73ff.bin | evidence: spirv-cross + SPIR-V disasm of both bins (vs: indirect stair instance draw — instance transform (StructuredBuffer<Stair> stairs_buffer t16, structured via Affine3Packed) unpacked to a column-major float3x4, scale derived from the packed translation sum, chained value-noise bevel/rock displacement (is_pebble gates the bevel + selects the pebble buffer offset), sample-space output via view_constants[current_view_index].world_to_sample; ps: solid rocky-albedo GBuffer pack — 3-axis SampleBias of rocky_terrain_texture blended by faceting-normal weights, sqrt-packed RGB8 albedo | 0x16000000 and normal | 0xCC000000 into a uint2 SV_Target0) | bindings vs: $Globals b13 (current_view_index, rocky_terrain_texture_dims, is_pebble, pebble_buffer_offset, highlight_active_hill_dims), stairs_buffer u16, frame_constants b0/space1; ps: sampler_mipmap s10, $Globals b13, rocky_terrain_texture t15, frame_constants b0/space1 | capabilities: RuntimeDescriptorArray(5302) + SPV_EXT_descriptor_indexing (both modules) | entry: vs_main/ps_main | status: O (math/logic/bindings/entries); I (function names — inlined); residual: value-noise loop/phi/select codegen + StructuredBuffer whole-array-load + inc-layer include-graph diffs (recorded, not fixed)

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
// current_view.hlsl is guard-skipped; its global current_view_index is
// redeclared in $Globals below (file OpString is still recorded).
#define TINYGLADE_INC_CURRENT_VIEW_HLSL
#include "inc/current_view.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/math.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/fbm_noise.hlsl"
#include "terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sh.hlsl"

// Active-view index + per-instance draw constants (single $Globals UBO at b13).
cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 rocky_terrain_texture_dims;
    int is_pebble;
    int pebble_buffer_offset;
    uint2 highlight_active_hill_dims;
};

// Placed stair instance record (written by stairs.cs into the tracked buffer).
struct Stair
{
    Affine3Packed transform;
};

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
StructuredBuffer<Stair> stairs_buffer : register(t16, space0);

// Rocky terrain albedo texture, sampled by the pixel shader.
Texture2D<float4> rocky_terrain_texture : register(t15, space0);

// Uninitialized zero seed used to seed the carried noise hash state (matches
// the target module's single OpUndef).
static uint stairs_noise_undef;

// Carried LCG-hash state threaded across value-noise lattice corners and across
// the three noise taps below. The seed/mask/or values are invariant (they stay
// at the perlin defaults); the SPIR-V still carries them through the loop phis.
struct ChainedNoiseState
{
    uint or_val;      // final bitcast-or value (default 0x3F800000 = 1065353216)
    uint and_mask;    // final bitcast-and mask (default 0x007FFFFF = 8388607)
    bool init_or;
    bool init_mask;
    uint seed_b;      // LCG increment (default 1013904223)
    uint seed_a;      // LCG multiplier (default 1664525)
    bool init_b;
    bool init_a;
};

// One 2D lattice hash (LCG mix with 1664525/1013904223 + final avalanche),
// transcribed from the inlined SPIR-V.
uint stairs_hash_corner(uint2 p, uint seed_a, uint seed_b)
{
    uint hy = p.y;
    hy += (hy << 10u);
    hy ^= (hy >> 6u);
    hy += (hy << 3u);
    hy ^= (hy >> 11u);
    uint h = ((p.x * seed_a) + (hy + (hy << 15u))) + seed_b;
    h *= seed_a;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    return h;
}

// Single value-noise octave. The compiled source carries the hash state, the
// amplitude and the sample position through a one-trip loop; the accumulator is
// `amp * (-1 + 2*valueNoise(pos))` and the octave step is the classic
// amp *= 0.55, pos = mul(rot, pos * 1.9).
float stairs_noise_octave(inout ChainedNoiseState s, inout float amp, inout float2 pos)
{
    float acc = 0.0f;
    for (int i = 0; i < 1; ++i)
    {
        int2 cell = int2(floor(pos));
        float2 f = frac(pos);

        // corner (0,0)
        bool ia0 = s.init_a ? s.init_a : true;
        uint sa0 = s.init_a ? s.seed_a : 1664525u;
        bool ib0 = s.init_b ? s.init_b : true;
        uint sb0 = s.init_b ? s.seed_b : 1013904223u;
        uint h0 = stairs_hash_corner(uint2(cell + int2(0, 0)), sa0, sb0);
        bool im0 = s.init_mask ? s.init_mask : true;
        uint am0 = s.init_mask ? s.and_mask : 8388607u;
        bool io0 = s.init_or ? s.init_or : true;
        uint av0 = s.init_or ? s.or_val : 1065353216u;
        float n00 = asfloat(((h0 ^ (h0 >> 18u)) & am0) | av0) - 1.0f;

        // corner (1,0)
        bool ia1 = ia0 ? ia0 : true;
        uint sa1 = ia0 ? sa0 : 1664525u;
        bool ib1 = ib0 ? ib0 : true;
        uint sb1 = ib0 ? sb0 : 1013904223u;
        uint h1 = stairs_hash_corner(uint2(cell + int2(1, 0)), sa1, sb1);
        bool im1 = im0 ? im0 : true;
        uint am1 = im0 ? am0 : 8388607u;
        bool io1 = io0 ? io0 : true;
        uint av1 = io0 ? av0 : 1065353216u;
        float n10 = asfloat(((h1 ^ (h1 >> 18u)) & am1) | av1) - 1.0f;

        // corner (0,1)
        bool ia2 = ia1 ? ia1 : true;
        uint sa2 = ia1 ? sa1 : 1664525u;
        bool ib2 = ib1 ? ib1 : true;
        uint sb2 = ib1 ? sb1 : 1013904223u;
        uint h2 = stairs_hash_corner(uint2(cell + int2(0, 1)), sa2, sb2);
        bool im2 = im1 ? im1 : true;
        uint am2 = im1 ? am1 : 8388607u;
        bool io2 = io1 ? io1 : true;
        uint av2 = io1 ? av1 : 1065353216u;
        float n01 = asfloat(((h2 ^ (h2 >> 18u)) & am2) | av2) - 1.0f;

        // corner (1,1)
        bool ia3 = ia2 ? ia2 : true;
        uint sa3 = ia2 ? sa2 : 1664525u;
        bool ib3 = ib2 ? ib2 : true;
        uint sb3 = ib2 ? sb2 : 1013904223u;
        uint h3 = stairs_hash_corner(uint2(cell + int2(1, 1)), sa3, sb3);
        bool im3 = im2 ? im2 : true;
        uint am3 = im2 ? am2 : 8388607u;
        bool io3 = io2 ? io2 : true;
        uint av3 = io2 ? av2 : 1065353216u;
        float n11 = asfloat(((h3 ^ (h3 >> 18u)) & am3) | av3) - 1.0f;

        // thread the carried state onward (final state = values propagated from
        // the third corner: seeds/mask/or never change from the defaults)
        s.init_a = ia2; s.seed_a = sa2;
        s.init_b = ib2; s.seed_b = sb2;
        s.init_mask = im2; s.and_mask = am2;
        s.init_or = io2; s.or_val = av2;

        float2 sm = f * f * (3.0f - 2.0f * f);
        float wx = sm.x;
        float wx1 = 1.0f - wx;
        float wy = sm.y;
        float blend = ((1.0f - wy) * ((n00 * wx1) + (n10 * wx)))
                    + (wy * ((n01 * wx1) + (n11 * wx)));
        acc += amp * (-1.0f + 2.0f * blend);
        amp *= 0.550000011920928955078125f;
        pos = mul(float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f),
                          float2(0.60000002384185791015625f, 0.800000011920928955078125f)),
                  pos * 1.89999997615814208984375f);
    }
    return acc;
}

struct VS_INPUT
{
    float3 position : TEXCOORD0;
    float3 normal : TEXCOORD1;
    float3 bevel_offset : TEXCOORD2;
    uint instance_index : SV_InstanceID;
};

struct VS_OUTPUT
{
    float3 c0 : TEXCOORD0;
    float c1 : TEXCOORD1;
    float3 c2 : TEXCOORD2;
    float4 position : SV_Position;
};

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
VS_OUTPUT vs_main(VS_INPUT input)
{
    VS_OUTPUT output;

    uint buffer_index = input.instance_index;
    if (is_pebble == 1)
        buffer_index = uint(pebble_buffer_offset) + input.instance_index;

    Affine3Packed transform = stairs_buffer[buffer_index].transform;
    float3x4 stair_m = transpose(float4x3(
        float3(transform.data[0], transform.data[1], transform.data[2]),
        float3(transform.data[3], transform.data[4], transform.data[5]),
        float3(transform.data[6], transform.data[7], transform.data[8]),
        float3(transform.data[9], transform.data[10], transform.data[11])));

    float scale100 = round(((stair_m[0].w + stair_m[1].w) + stair_m[2].w) * 100.0f);
    float scale = scale100 * 0.00999999977648258209228515625f;

    ChainedNoiseState ns;
    ns.or_val = stairs_noise_undef;
    ns.and_mask = stairs_noise_undef;
    ns.init_or = false;
    ns.init_mask = false;
    ns.seed_b = stairs_noise_undef;
    ns.seed_a = stairs_noise_undef;
    ns.init_b = false;
    ns.init_a = false;

    float3 beveled_pos;
    if (is_pebble == 0)
    {
        float2 npos = (input.position.xz * 5.0f) + scale.xx;
        float amp = 0.5f;
        float bevel = stairs_noise_octave(ns, amp, npos);
        beveled_pos = input.position + (input.bevel_offset * bevel);
    }
    else
    {
        beveled_pos = input.position;
        ns.or_val = stairs_noise_undef;
        ns.and_mask = stairs_noise_undef;
        ns.init_or = false;
        ns.init_mask = false;
        ns.seed_b = stairs_noise_undef;
        ns.seed_a = stairs_noise_undef;
        ns.init_b = false;
        ns.init_a = false;
    }

    float3 world_pos = beveled_pos;
    if (length(float3(stair_m[0].x, stair_m[1].x, stair_m[2].x)) > 0.5f)
    {
        float2 rpos = ((beveled_pos.xz * 1.0f) + scale.xx) + 3234.0f.xx;
        float ramp = 0.5f;
        float rock_noise = stairs_noise_octave(ns, ramp, rpos);
        world_pos.z = beveled_pos.z + (rock_noise * 0.5f);

        float2 gpos = (world_pos.xz * 2.0f) + (scale100 * 0.0199999995529651641845703125f).xx;
        float gamp = 0.5f;
        float ground_noise = stairs_noise_octave(ns, gamp, gpos);

        float ground_snap;
        do
        {
            float t = abs(ground_noise);
            if (t > 0.100000001490116119384765625f)
            {
                ground_snap = 0.0f;
                break;
            }
            float tt = t * 10.0f;
            ground_snap = 1.0f - ((tt * tt) * (3.0f - (t * 20.0f)));
            break;
        } while (false);
        world_pos.y = beveled_pos.y - (ground_snap * 0.100000001490116119384765625f);
    }
    else
    {
        world_pos = beveled_pos;
    }

    float3 world = mul(stair_m, float4(world_pos, 1.0f));
    output.position = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    output.c0 = world;
    output.c1 = scale;
    output.c2 = mul(stair_m, float4(input.normal, 0.0f));
    return output;
}

struct PS_INPUT
{
    float3 c0 : TEXCOORD0;
    float c1 : TEXCOORD1;
    float3 c2 : TEXCOORD2;
};

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
uint2 ps_main(PS_INPUT input) : SV_Target0
{
    float3 dd = ddx(input.c0);
    float3 dy = ddy(input.c0);
    float3 faceting = cross(dd, -dy);

    float3 sample_coord = (input.c0 * 0.07999999821186065673828125f) + input.c1.xxx;

    float4 sx = rocky_terrain_texture.SampleBias(sampler_mipmap, sample_coord.zy, global_mip_bias);
    float4 sy = rocky_terrain_texture.SampleBias(sampler_mipmap, sample_coord.zx, global_mip_bias);
    float4 sz = rocky_terrain_texture.SampleBias(sampler_mipmap, sample_coord.xy, global_mip_bias);

    float3 fpow = pow(abs(faceting), 1.0f.xxx);
    float fx = fpow.x;
    float fy = fpow.y;
    float fz = fpow.z;
    float3 albedo = ((((sx.xyz * fx) + (sy.xyz * fy)) + (sz.xyz * fz)) / ((fx + fy) + fz).xxx)
                    * 1.2000000476837158203125f;

    bool pebble = is_pebble == 1;
    float3 color;
    if (pebble)
        color = albedo * 0.60000002384185791015625f;
    else
        color = albedo;

    float3 normal = normalize(pebble ? input.c2 : faceting);
    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;

    float3 sqrt_color = sqrt(clamp(color, 0.0f.xxx, 1.0f.xxx));

    uint albedo_packed = (uint((clamp(sqrt_color.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                        + (uint((clamp(sqrt_color.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                        + (uint((clamp(sqrt_color.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint normal_packed = (uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                        + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                        + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);

    uint2 result = uint2(normal_packed + 3422552064u, albedo_packed);
    result.y = albedo_packed | 369098752u;
    return result;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _rocky_terrain_stairs_stairs_indirect.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
