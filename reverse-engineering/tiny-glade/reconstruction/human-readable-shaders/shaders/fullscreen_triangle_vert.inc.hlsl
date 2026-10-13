// ==RECONSTRUCTED== shaders/fullscreen_triangle_vert.inc.hlsl | F-E | 证据:
//   blit/resolve/fullscreen 光栅化 VS 模块(mod0 of _blit_* 及全屏 pass)— SV_VertexID
//   分支: v0 位置(-1,-1) uv(0,1); v1 位置(3,-1) uv(2,1); v2 位置(-1,3) uv(0,-1);
//   输出 TEXCOORD0=uv 与 SV_Position;以 include-only 形式内嵌于 24 个二进制 |
//   状态: O(顶点位置/uv/语义来自内联 SPIR-V); I(结构/输出变量名 — 内联;
//   VS 输出名未内嵌)
//
// 用途: 全屏三角形顶点着色器,用于 blit / resolve / 延迟全屏 pass。
// 无顶点缓冲:三个顶点由 SV_VertexID 合成。

#ifndef TINYGLADE_FULLSCREEN_TRIANGLE_VERT_INC_HLSL
#define TINYGLADE_FULLSCREEN_TRIANGLE_VERT_INC_HLSL

// 顶点着色器到像素着色器的插值数据。
struct VsToPs
{
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

VsToPs vs_main(uint vertex_id : SV_VertexID)
{
    VsToPs o;

    // 左上角顶点:位置(-1,-1),uv(0,1)。
    if (vertex_id == 0u)
    {
        o.position = float4(-1.0f, -1.0f, 0.0f, 1.0f);
        o.uv = float2(0.0f, 1.0f);
    }
    // 右下外侧顶点:位置(3,-1),uv(2,1)。
    else if (vertex_id == 1u)
    {
        o.position = float4(3.0f, -1.0f, 0.0f, 1.0f);
        o.uv = float2(2.0f, 1.0f);
    }
    // 左下外侧顶点:位置(-1,3),uv(0,-1)。
    else
    {
        o.position = float4(-1.0f, 3.0f, 0.0f, 1.0f);
        o.uv = float2(0.0f, -1.0f);
    }

    return o;
}

#endif // TINYGLADE_FULLSCREEN_TRIANGLE_VERT_INC_HLSL
