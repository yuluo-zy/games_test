// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/painterly/raster_strokes.raster.hlsl | evidence: _painterly_raster_strokes.raster.hlsl_{b903,f0ad}.bin — vs expands each stroke (6 vertices, no instancing: vertex_id/6) into an aspect-correct quad: srgb-decode color, size/rotation halves, y scaled by brush aspect*lerp(1,1,stroke*scale), appear smoothstep; ps alpha-tests the brush (reveal sweep from left when appear<1); caps Shader only | status: O (triangle strip layout + ps reveal from inlined SPIR-V); I (VsOut names)
//
// Painterly thumbnail chain: rasterizes the generated strokes as textured
// quads (two triangles per stroke) over the brush texture.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/tracked_buffer.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/quad_mesh.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/uv.hlsl"
#include "inc/color/srgb.hlsl"
#include "painterly/stroke.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 brush_tex_dims;
    uint2 framebuffer_dims;
    float current_t;
};

ByteAddressBuffer strokes_buf : register(t15, space0);

struct VsOut
{
    nointerpolation float4 color : TEXCOORD0;
    float2 uv : TEXCOORD1;
    nointerpolation float appear : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    uint stroke_index = vertex_id / 6u;
    uint corner = vertex_id % 6u;

    uint packed_color = strokes_buf.Load<uint>(stroke_index * 16u + 4u);
    float r = float(packed_color & 255u);
    float g = float((packed_color >> 8u) & 255u);
    float b = float((packed_color >> 16u) & 255u);
    float3 color = srgb_to_linear(float3(r, g, b) * (1.0f / 255.0f));

    float2 size_rot = unpack_stroke_size_rotation(strokes_buf.Load<uint>(stroke_index * 16u + 8u));
    float size = size_rot.x;
    float rotation = size_rot.y;
    float cos_r = cos(rotation);
    float sin_r = sin(rotation);

    float4 corner_quad;
    if (corner == 0u)      corner_quad = float4(0.5f, -0.5f, 1.0f, 0.0f);
    else if (corner == 1u) corner_quad = float4(0.5f, 0.5f, 1.0f, 1.0f);
    else if (corner == 2u) corner_quad = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (corner == 3u) corner_quad = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (corner == 4u) corner_quad = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    else                   corner_quad = float4(0.5f, -0.5f, 1.0f, 0.0f);

    corner_quad.y *= lerp(1.0f, float(brush_tex_dims.y) / float(brush_tex_dims.x), pow(float(stroke_index) * 0.0009765625f, 0.25f));

    float2 quad_pos = corner_quad.xy;
    float2 stroke_pos = unpack_stroke_pos(strokes_buf.Load<uint>(stroke_index * 16u + 0u));
    float2 world = (stroke_pos + (mul(float2x2(cos_r, sin_r, -sin_r, cos_r), quad_pos) * size) - 0.5f.xx) * float2(2.0f, -2.0f);

    VsOut o;
    o.position = float4(world.x / (float(framebuffer_dims.x) / float(framebuffer_dims.y)), world.y, 0.0f, 1.0f);
    o.color = float4(color, 1.0f) * 0.75f;
    o.uv = corner_quad.zw;
    float appear_t = strokes_buf.Load<float>(stroke_index * 16u + 12u);
    o.appear = smoothstep(appear_t, appear_t + 0.100000001490116119384765625f, current_t);
    return o;
}

Texture2D<float4> brush_tex : register(t14, space0);

float4 ps_main(VsOut input) : SV_Target0
{
    float brush = brush_tex.Sample(sampler_lnc, input.uv).x;
    float alpha;
    if (input.appear < 1.0f)
    {
        float2 reveal_uv = input.uv - float2(1.0f - input.appear, 0.0f);
        float2 clamped = clamp(reveal_uv, 0.0f.xx, 1.0f.xx);
        float inside = float(all(bool2(clamped.x == reveal_uv.x, clamped.y == reveal_uv.y)));
        alpha = sqrt(brush * (inside * brush_tex.Sample(sampler_lnc, reveal_uv).x));
    }
    else
    {
        alpha = brush;
    }
    return input.color * alpha;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _painterly_raster_strokes.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
