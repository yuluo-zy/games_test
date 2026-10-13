// ==RECONSTRUCTED== shaders/dof/upsample_halfres.inc.hlsl | B (dof-aa review, 2026-10-13): helper content; the include edge exists in the 3 consumer bins so a live helper is more faithful.
//
// fullres/fullres_upsample bins inline an 8-texel half-res gather:
//   fullres_pixel/2 + spatial_resolve_offsets[quadrant*16+i].xy, averaged *1/8,
//   then the caller renormalizes alpha.  Function name/signature are Inference
//   (inlined in consumers); the body reproduces the observed inlined opcodes.
//
// === 中文说明 ===
// 半分辨率 8 抽头采集辅助函数。fullres / fullres_upsample 的 bin 内联了 8 纹素
//   半分辨率采集: fullres_pixel/2 + spatial_resolve_offsets[quadrant*16+i].xy,
//   平均 *1/8,调用方再对 alpha 归一化。函数名/签名为推断值(消费者中内联),
//   函数体复现了观察到的内联操作码。被 fullres 系列 include,不改名。

#ifndef TINYGLADE_DOF_UPSAMPLE_HALFRES_INC_HLSL
#define TINYGLADE_DOF_UPSAMPLE_HALFRES_INC_HLSL

// 围绕 2x2 全分辨率象限采集 8 个半分辨率纹素并平均。
float4 upsample_halfres(Texture2D<float4> src_tex,
                        int4 resolve_offsets[512],
                        uint2 fullres_pixel,
                        uint quadrant)
{
    float4 sum = 0.0f;
    for (uint i = 0u; i < 8u; ++i)
    {
        sum += src_tex[(fullres_pixel / uint2(2u, 2u))
            + uint2(resolve_offsets[(quadrant * 16u) + i].xy)];
    }
    return sum * 0.125f.xxxx;
}

#endif // TINYGLADE_DOF_UPSAMPLE_HALFRES_INC_HLSL
