// ==RECONSTRUCTED== shaders/text.raster.hlsl | F-E | bins: _text.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Text / vector-shape renderer. vs expands packed PrimPacked records into quads.
// ps has two paths: a shape-stream interpreter (per-primitive op list sampled
// from shape_stream with bindless textures) and a glyph-atlas path with a
// procedural dither-noise offset.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0, sampler_llc s3)
#include "inc/quad_mesh.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/uv.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/perlin.hlsl"
#include "inc/image.hlsl"
#include "inc/curve.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
// inc/blue_noise.hlsl no longer declares the texture (register removed per
// water-ice review inc-fix #3 / tracker #17); this consumer binds it at t14.
Texture2D<float4> blue_noise_texture : register(t14, space0);
#include "inc/blue_noise.hlsl"
#include "inc/hammersley.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 blue_noise_texture_dims;
    float2 window_size_pixels;
    uint2 font_atlas_tex_dims;
};

struct PrimPacked
{
    float2 pos_min;
    float2 pos_extent;
    float2 uv_min;
    float2 uv_extent;
    uint color;
    uint stream;
    uint pad0;
    uint pad1;
};

ByteAddressBuffer primitives : register(t15, space0);
ByteAddressBuffer shape_stream : register(t16, space0);
Texture2D<float4> font_atlas_tex : register(t17, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 local : TEXCOORD1;
    float4 color : TEXCOORD2;
    nointerpolation uint stream : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    uint vid = vertex_id % 6u;
    uint prim = vertex_id / 6u;

    float4 corner;
    if (vid == 0u) corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
    else if (vid == 1u) corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
    else if (vid == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vid == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vid == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 0.0f);

    float pixel_scale = window_size_pixels.y * 0.001f;

    uint packed_color = primitives.Load<uint>(prim * 48u + 32u);
    float r = float(packed_color & 255u) * (1.0f / 255.0f);
    float g_raw = float((packed_color >> 8u) & 255u);
    float b_raw = float((packed_color >> 16u) & 255u);
    float a_raw = float((packed_color >> 24u) & 255u);
    float g = g_raw * (1.0f / 255.0f);
    float b = b_raw * (1.0f / 255.0f);
    float a = a_raw * (1.0f / 255.0f);

    float da;
    float dg;
    float db;
    if (0.04045f < a) da = pow((a + 0.055f) * 0.947867333f, 2.4f);
    else da = a_raw * 0.000303526991f;
    if (0.04045f < b) db = pow((b + 0.055f) * 0.947867333f, 2.4f);
    else db = b_raw * 0.000303526991f;
    if (0.04045f < g) dg = pow((g + 0.055f) * 0.947867333f, 2.4f);
    else dg = g_raw * 0.000303526991f;

    uint stream = primitives.Load<uint>(prim * 48u + 36u);
    bool snap = (stream == 0u) || (stream == 0xFFFFFFFFu);

    float2 pos_min = snap ? round(primitives.Load<float2>(prim * 48u + 0u) * pixel_scale) / pixel_scale
                          : primitives.Load<float2>(prim * 48u + 0u);
    float2 pos = pos_min + primitives.Load<float2>(prim * 48u + 8u) * (corner.xy + 0.5f);
    float2 uv = (primitives.Load<float2>(prim * 48u + 16u) + primitives.Load<float2>(prim * 48u + 24u) * corner.zw)
              / float2(font_atlas_tex_dims);

    o.position = float4((pos / (window_size_pixels / pixel_scale) - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    o.uv = (stream != 0u) ? corner.zw : uv;
    o.local = float4(pos - (pos_min + primitives.Load<float2>(prim * 48u + 8u) * 0.5f), primitives.Load<float2>(prim * 48u + 8u));
    o.color = float4(float3(da, db, dg) * r, r);
    o.stream = stream;
    return o;
}

// Decode an RGBA8 uint (sRGB) into a linear float4.
float4 decode_rgba8(uint p)
{
    float r = float(p & 255u) * (1.0f / 255.0f);
    float g = float((p >> 8u) & 255u) * (1.0f / 255.0f);
    float b = float((p >> 16u) & 255u) * (1.0f / 255.0f);
    float a = float((p >> 24u) & 255u) * (1.0f / 255.0f);
    float dr;
    float dg;
    float db;
    if (0.04045f < r) dr = pow((r + 0.055f) * 0.947867333f, 2.4f);
    else dr = float(p & 255u) * 0.000303526991f;
    if (0.04045f < g) dg = pow((g + 0.055f) * 0.947867333f, 2.4f);
    else dg = float((p >> 8u) & 255u) * 0.000303526991f;
    if (0.04045f < b) db = pow((b + 0.055f) * 0.947867333f, 2.4f);
    else db = float((p >> 16u) & 255u) * 0.000303526991f;
    return float4(dr, dg, db, a);
}

// Hashed value noise in [0,1) for the glyph dither (bilinear of corner hashes).
float hash_noise_octave(float2 p)
{
    int2 base = int2(floor(p));
    float2 f = frac(p);
    float2 s = f * f * (3.0f - f * 2.0f);

    uint2 c00 = uint2(base);
    uint2 c10 = uint2(base + int2(1, 0));
    uint2 c01 = uint2(base + int2(0, 1));
    uint2 c11 = uint2(base + int2(1, 1));

    uint h00 = c00.y + (c00.y << 10u); h00 ^= h00 >> 6u; h00 += h00 << 3u; h00 ^= h00 >> 11u;
    h00 = ((c00.x * 1664525u) + (h00 + (h00 << 15u)) + 1013904223u) * 1664525u;
    h00 ^= h00 >> 11u; h00 ^= (h00 << 7u) & 2636928640u; h00 ^= (h00 << 15u) & 4022730752u;
    float n00 = asfloat((h00 & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

    uint h10 = c10.y + (c10.y << 10u); h10 ^= h10 >> 6u; h10 += h10 << 3u; h10 ^= h10 >> 11u;
    h10 = ((c10.x * 1664525u) + (h10 + (h10 << 15u)) + 1013904223u) * 1664525u;
    h10 ^= h10 >> 11u; h10 ^= (h10 << 7u) & 2636928640u; h10 ^= (h10 << 15u) & 4022730752u;
    float n10 = asfloat((h10 & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

    uint h01 = c01.y + (c01.y << 10u); h01 ^= h01 >> 6u; h01 += h01 << 3u; h01 ^= h01 >> 11u;
    h01 = ((c01.x * 1664525u) + (h01 + (h01 << 15u)) + 1013904223u) * 1664525u;
    h01 ^= h01 >> 11u; h01 ^= (h01 << 7u) & 2636928640u; h01 ^= (h01 << 15u) & 4022730752u;
    float n01 = asfloat((h01 & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

    uint h11 = c11.y + (c11.y << 10u); h11 ^= h11 >> 6u; h11 += h11 << 3u; h11 ^= h11 >> 11u;
    h11 = ((c11.x * 1664525u) + (h11 + (h11 << 15u)) + 1013904223u) * 1664525u;
    h11 ^= h11 >> 11u; h11 ^= (h11 << 7u) & 2636928640u; h11 ^= (h11 << 15u) & 4022730752u;
    float n11 = asfloat((h11 & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

    float nx = lerp(n00, n10, s.x);
    float ny = lerp(n01, n11, s.x);
    return lerp(nx, ny, s.y);
}

struct PsIn
{
    float2 uv : TEXCOORD0;
    float4 local : TEXCOORD1;
    float4 color : TEXCOORD2;
    nointerpolation uint stream : TEXCOORD3;
};

// Sampled glyph atlas path: procedural dither noise offsets the lookup.
float4 glyph_sample(PsIn vin, float4 frag_coord)
{
    float2 n = (frag_coord.xy / window_size_pixels.y) * 1440.0f * 0.3f;
    float2 n2 = n * 2.5f;

    float rnd1 = hash_noise_octave(n2 * 0.7192f);
    float rnd2 = hash_noise_octave(n2 * 0.89812f);
    float rnd3 = hash_noise_octave(n * 0.13128f);

    float4 atlas = font_atlas_tex.SampleLevel(sampler_lnc, vin.uv, 0.0f);

    float4 c = atlas;
    c = lerp(c, min(c, font_atlas_tex.SampleLevel(sampler_lnc, vin.uv + float2(0.008f, -0.008f) * (rnd1 - 0.5f), 0.0f)), (rnd2 - 0.48f).xxxx);
    c = lerp(c, min(c, font_atlas_tex.SampleLevel(sampler_lnc, vin.uv + 0.004f * (rnd2 - 0.5f), 0.0f)), (rnd3 - 0.48f).xxxx);
    c.w *= 0.97f;
    c = clamp(c, 0.0f, 1.0f);

    float4 col = float4(c.rgb * c.a, c.a);
    if (all(atlas.rgb == float3(1.0f, 1.0f, 1.0f)))
        col = vin.color * col;
    return col;
}

// Sample one bindless sprite and multiply it into acc (returns acc * sprite).
float4 apply_sprite(PsIn vin, float4 acc, uint tex_idx, uint mode, float4 frag_coord)
{
    float2 uv0 = vin.uv;
    float2 px = 0.0f;
    float2 nx = 0.0f;
    if (mode == 1u)
    {
        uint w, h;
        bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].GetDimensions(w, h);
        float2 dims = float2(w, h);
        uv0.x = (vin.uv.x - 0.5f) * (vin.local.z / vin.local.w) / (dims.x / dims.y) + 0.5f;
        nx = -ddy(uv0) * 0.5f;
        px = ddx(uv0) * 0.5f;
    }
    else if (mode == 2u)
    {
        uint w, h;
        bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].GetDimensions(w, h);
        float2 dims = float2(w, h);
        float u = (vin.uv.x - 0.5f) * (vin.local.z / vin.local.w) / (dims.x / dims.y);
        uv0 = float2(u + 0.5f, vin.uv.y);
        float aa = smoothstep(0.0f, 1.0f, max(0.0f, (abs(u) - 0.4f) * 2.0f)) * 0.25f;
        float2 n = -ddy(uv0) * 0.5f;
        n.y += float(int(sign(n.y))) * aa * 0.5f;
        nx = n;
        float2 blue = blue_noise_texture.Load(int3((uint2(frag_coord.xy) + uint2(frac(float2(9.5585f, 7.3381f)) * 256.0f)) % uint2(256u, 256u), 0u)).rg;
        uv0 += (((blue * 255.0f) * (1.0f / 256.0f) + (1.0f / 512.0f)) - 0.5f) * aa;
        px = ddx(uv0) * 0.5f;
    }
    else if (mode == 3u)
    {
        // Repeated tiling sprite (2x2 nearest blocks).
        uint w, h;
        bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].GetDimensions(w, h);
        float2 dims = float2(w, h);
        float2 size = dims * 0.33333f;
        float2 uv_rep = vin.local.zw;
        float scale = max(1.0f, (0.66667f * max(size.x, size.y)) / vin.local.z);
        uv_rep *= scale;
        float2 uv_scaled = vin.uv * scale;
        float2 block = size * 0.33333f;
        float texel_a[2][2];
        float weight[2][2];
        for (int c = 0; c < 2; ++c)
        {
            if (uv_scaled[c] < block[c])
            {
                texel_a[c][0] = texel_a[c][1] = (uv_scaled[c] / block[c]) * 0.33333f;
                weight[c][0] = weight[c][1] = 0.5f;
            }
            else if (uv_scaled[c] < (uv_rep[c] - block[c]))
            {
                float d = uv_scaled[c] - block[c];
                float denom = uv_rep[c] - 2.0f * block[c];
                float wfrac = denom / block[c];
                texel_a[c][0] = (abs(1.0f - 2.0f * frac(0.5f + ((wfrac * d / denom) * 0.5f))) * 0.33333f) + 0.33333f;
                texel_a[c][1] = (abs(1.0f - 2.0f * frac(((wfrac * ((uv_rep[c] - block[c]) - uv_scaled[c])) / denom) * 0.5f)) * 0.33333f) + 0.33333f;
                weight[c][1] = d / denom;
                weight[c][0] = 1.0f - weight[c][1];
            }
            else
            {
                float v = (((uv_scaled[c] - (uv_rep[c] - block[c])) / block[c]) * 0.33333f) + 0.66667f;
                texel_a[c][0] = texel_a[c][1] = v;
                weight[c][0] = weight[c][1] = 0.5f;
            }
        }
        float wsum = 0.0f;
        float4 sum = 0.0f;
        for (int j = 0; j < 2; ++j)
        {
            for (int i = 0; i < 2; ++i)
            {
                float w = weight[0][i] * weight[1][j];
                w = w * w * w;
                wsum += w;
                sum += bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].SampleLevel(sampler_llc, float2(texel_a[0][i], texel_a[1][j]), 0.0f) * w;
            }
        }
        float4 s = sum / wsum;
        return acc * float4(s.rgb * s.a, s.a);
    }
    float4 s = bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].SampleGrad(sampler_llc, uv0, px, nx);
    return acc * float4(s.rgb * s.a, s.a);
}

float4 ps_main(PsIn pin, float4 position : SV_Position) : SV_Target0
{
    if (pin.stream == 0u)
        return pin.color;
    if (pin.stream == 0xFFFFFFFFu)
        return glyph_sample(pin, position);

    // Shape-stream interpreter: iterate ops in [stream>>8, (stream&255)+(stream>>8)).
    uint start = pin.stream >> 8u;
    uint end = (pin.stream & 255u) + start;

    float4 acc = pin.color;
    uint op = start;
    uint count = 0u;
    while (op < end)
    {
        if (count >= 256u)
            return float4(1.0f, 0.0f, 1.0f, 1.0f); // error: op stream runaway

        uint word = shape_stream.Load<uint>(((op * 4u) >> 2u) * 4u + 0u);
        uint type = word & 255u;
        if (type == 0u)
        {
            // Color multiply.
            float4 tint = decode_rgba8(shape_stream.Load<uint>((((op + 1u) * 4u) >> 2u) * 4u + 0u));
            acc *= tint;
            op += 2u;
        }
        else if (type == 1u)
        {
            // Sprite (bindless) sample.
            uint tex_idx = (word >> 8u) & 0xFFFFu;
            uint mode = (word >> 24u) & 3u;
            acc = apply_sprite(pin, acc, tex_idx, mode, position);
            op += 1u;
        }
        else if (type == 2u)
        {
            // Rounded-rect mask.
            float radius = float((word >> 8u) & 255u);
            float2 d = (abs(pin.local.xy) - (pin.local.zw * 0.5f)) + radius;
            float mask = clamp(-(length(max(d, 0.0f)) + min(max(d.x, d.y), 0.0f) - radius - 0.5f), 0.0f, 1.0f);
            acc *= mask;
            op += 1u;
        }
        else
        {
            break;
        }
        count++;
    }

    return acc;
}
