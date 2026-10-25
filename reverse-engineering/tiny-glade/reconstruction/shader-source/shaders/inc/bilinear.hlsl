// ==RECONSTRUCTED== shaders/inc/bilinear.hlsl | evidence: _calculate_reprojection_map.cs.hlsl_cs_main_0 (only consumer bin) inlined 2x2 texel neighborhood (int2(base), base+(1,0), base+(0,1), base+(1,1); bounds mask dot(valid,(1,2,4,8))*1/15); include-graph edge pack_unpack.hlsl -> bilinear.hlsl | status: O (2x2-neighborhood + valid-bitmask pattern from inlined use-site); I (function names/signature — inlined, not embedded)
//
// 2x2 bilinear texel neighborhood helper used by explicit-LOD gather paths.

#ifndef TINYGLADE_INC_BILINEAR_HLSL
#define TINYGLADE_INC_BILINEAR_HLSL

struct BilinearCorners
{
    int2 p00;
    int2 p10;
    int2 p01;
    int2 p11;
};

// 2x2 neighborhood around a base texel. `mask` bit i (weights (1,2,4,8) ->
// corners (p00,p10,p01,p11)) is set when that corner is inside [0, dims).
BilinearCorners bilinear_corners(int2 base, uint2 dims)
{
    BilinearCorners c;
    c.p00 = base;
    c.p10 = base + int2(1, 0);
    c.p01 = base + int2(0, 1);
    c.p11 = base + int2(1, 1);
    return c;
}

// Pack the in-bounds state of the four corners into a uint bitmask.
uint bilinear_corners_mask(BilinearCorners c, uint2 dims)
{
    uint mask = 0u;
    int2 p[4] = { c.p00, c.p10, c.p01, c.p11 };
    int2 d = int2(dims);
    for (int i = 0; i < 4; i++)
    {
        if (p[i].x >= 0 && p[i].y >= 0 && p[i].x < d.x && p[i].y < d.y)
            mask |= (1u << i);
    }
    return mask;
}

#endif // TINYGLADE_INC_BILINEAR_HLSL
