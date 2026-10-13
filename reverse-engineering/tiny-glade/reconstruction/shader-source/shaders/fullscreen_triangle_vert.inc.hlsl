// ==RECONSTRUCTED== shaders/fullscreen_triangle_vert.inc.hlsl | F-E | evidence: blit/resolve/fullscreen raster VS modules (mod0 of _blit_* and other fullscreen passes) — SV_VertexID switch: v0 pos(-1,-1) uv(0,1); v1 pos(3,-1) uv(2,1); v2 pos(-1,3) uv(0,-1); output TEXCOORD0=uv + SV_Position; embedded in 24 bins as include-only | status: O (vertex positions / uvs / semantics from inlined SPIR-V); I (struct/output variable names — inlined; VS output name not embedded)
//
// Fullscreen triangle vertex shader used by blit / resolve / deferred fullscreen
// passes. No vertex buffer: three vertices are synthesized from SV_VertexID.

#ifndef TINYGLADE_FULLSCREEN_TRIANGLE_VERT_INC_HLSL
#define TINYGLADE_FULLSCREEN_TRIANGLE_VERT_INC_HLSL

struct VsToPs
{
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

VsToPs vs_main(uint vertex_id : SV_VertexID)
{
    VsToPs o;

    if (vertex_id == 0u)
    {
        o.position = float4(-1.0f, -1.0f, 0.0f, 1.0f);
        o.uv = float2(0.0f, 1.0f);
    }
    else if (vertex_id == 1u)
    {
        o.position = float4(3.0f, -1.0f, 0.0f, 1.0f);
        o.uv = float2(2.0f, 1.0f);
    }
    else
    {
        o.position = float4(-1.0f, 3.0f, 0.0f, 1.0f);
        o.uv = float2(0.0f, -1.0f);
    }

    return o;
}

#endif // TINYGLADE_FULLSCREEN_TRIANGLE_VERT_INC_HLSL
