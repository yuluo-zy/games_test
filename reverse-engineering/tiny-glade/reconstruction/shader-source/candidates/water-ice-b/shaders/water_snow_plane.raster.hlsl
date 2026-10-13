// ==RECONSTRUCTED== shaders/water_snow_plane.raster.hlsl
// Evidence: _water_snow_plane.raster.hlsl_b903e43ffb3da915.bin (SPIR-V, offsets 23/14938).
//   VS: $Globals b13 (current_view_index, model_to_world, water_mask_dims, bool_render_shadows),
//   frame_constants set1/b0; outputs only C1 (world position) -> flat snow plane over water.
//   PS: sampler_llc s3, water_mask t14; discards outside the water mask (green channel < 0.2),
//   then a per-cell value-noise (LCG/hash chain 1664525/1013904223 + mantissa random) bilinear
//   smoothstep-gated to punch irregular holes; outputs a fixed packed uint2 id/color.
// Status: O (algorithm + bindings from SPIR-V; hash chain transcribed from the disassembly).
// NOTE: the f0ad PS is a reduced variant (no noise hole-punch, only mask test); single source cannot
// reproduce both — b903 is the reconstruction target (recorded residual).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/math.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/perlin.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    float4x4 model_to_world;
    uint2 water_mask_dims;
    int bool_render_shadows;
};

Texture2D<float4> water_mask : register(t14, space0);

struct VSOut
{
    float3 c1 : TEXCOORD1;
    float4 pos : SV_Position;
};

VSOut vs_main(float3 Vertex_Position : TEXCOORD0)
{
    VSOut o;
    float4 p = mul(model_to_world, float4(Vertex_Position, 1.0f));
    float2 xz = p.xz * 130.0f;
    float4 wp = float4(xz.x, p.y, xz.y, p.w);
    wp.y = -0.01f;
    o.pos = mul(view_constants[current_view_index].world_to_sample, wp);
    o.c1 = wp.xyz;
    return o;
}

// Per-cell hash chain (LCG constant 1664525 / 1013904223 with a split-y avalanche),
// transcribed from the disassembly. Returns the final mixing word.
uint snow_cell_hash(uint2 p)
{
    uint y = p.y;
    y = y + (y << 10u);
    y ^= y >> 6u;
    y += y << 3u;
    y ^= y >> 11u;
    uint h = ((p.x * 1664525u) + (y + (y << 15u)) + 1013904223u) * 1664525u;
    h ^= h >> 11u;
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    return h;
}

// Unit random from a hash word (exponent/mantissa trick).
float snow_hash_unit(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

uint2 ps_main(float3 c1 : TEXCOORD1) : SV_Target0
{
    float2 uv = c1.xz * 0.0076923076994717121124267578125f + 0.5f;
    float mask = water_mask.SampleLevel(sampler_llc, uv, 0.0f).y;
    if (mask < 0.2f)
    {
        discard;
    }

    // Per-cell value noise over a 1.5-unit world grid; the 4 corners of the containing cell.
    float2 fp = c1.xz * 1.5f;
    int2 cell = int2(floor(fp));
    float2 f = frac(fp);
    float2 sm = f * f * (3.0f - 2.0f * f);

    uint2 base = uint2(cell);
    float r00 = snow_hash_unit(snow_cell_hash(base));
    float r10 = snow_hash_unit(snow_cell_hash(base + uint2(1u, 0u)));
    float r01 = snow_hash_unit(snow_cell_hash(base + uint2(0u, 1u)));
    float r11 = snow_hash_unit(snow_cell_hash(base + uint2(1u, 1u)));

    float n = ((1.0f - sm.y) * (r00 * (1.0f - sm.x) + r10 * sm.x)
        + (r01 * (1.0f - sm.x)) * sm.y) + (r11 * sm.x) * sm.y;

    if (clamp(1.0f - smoothstep(0.6f, 0.65f, mask + n * n * 0.3f), 0.0f, 1.0f) < 0.8f)
    {
        discard;
    }

    return uint2(4286644096u, 114677205u);
}
