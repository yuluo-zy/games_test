// ==RECONSTRUCTED== shaders/generate_hzb.cs.hlsl | F-D
// Evidence: _generate_hzb.cs.hlsl_cs_main_0.bin, entry cs_main, LocalSize [256,1,1].
//   FidelityFX Single Pass Downsampler variant (hzb / depth chain): push constants
//   spdConstants (mips, numWorkGroups, workGroupOffset, input_dims), imgSrc t0
//   (Texture2D sampled), imgDst u1 (RWTexture2DArray<float> r16f, 13 slices =
//   mip0..12), imgDst6 u2 (r16f, coherent), spdGlobalAtomic u3 (coherent
//   RWStructuredBuffer { uint counter[6] }). groupshared spdCounter + spdIntermediateR[16][16].
//   GroupNonUniform / GroupNonUniformShuffle (WaveReadLaneAt 2x2 quad reduce).
//   Ops reconstructed from spirv-cross (structure preserved: vector bounds compare
//   all(coord < input_dims), OpSwitch execute-once blocks, min-of-4 downsample).
// Status: O (structure from disassembly).

struct SpdConstants
{
    uint mips;
    uint numWorkGroups;
    uint2 workGroupOffset;
    uint2 input_dims;
};

struct SpdGlobalAtomicBuffer
{
    uint counter[6];
};

[[vk::push_constant]]
ConstantBuffer<SpdConstants> spdConstants;

Texture2D<float4> imgSrc : register(t0, space0);
[[vk::image_format("r16f")]] RWTexture2DArray<float> imgDst[13] : register(u1, space0);
[[vk::image_format("r16f")]] globallycoherent RWTexture2DArray<float> imgDst6 : register(u2, space0);
globallycoherent RWStructuredBuffer<SpdGlobalAtomicBuffer> spdGlobalAtomic : register(u3, space0);

groupshared uint spdCounter;
groupshared float spdIntermediateR[16][16];

[numthreads(256, 1, 1)]
void cs_main(uint3 gl_WorkGroupID : SV_GroupID,
             uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    uint2 workGroup = gl_WorkGroupID.xy + spdConstants.workGroupOffset;

    switch (0)
    {
        default:
        {
            uint i0 = gl_LocalInvocationIndex % 64u;
            uint x = ((i0 & 1u) | (((i0 >> 2u) & 7u) & 0xFFFFFFFEu)) + (8u * ((gl_LocalInvocationIndex >> 6u) % 2u));
            uint y = ((((i0 >> 1u) & 3u) & 3u) | (((i0 >> 3u) & 7u) & 0xFFFFFFFCu)) + (8u * (gl_LocalInvocationIndex >> 7u));
            int z = int(gl_WorkGroupID.z);

            // ---- mip0 2x2 min + mip1 4x4 min (block1) ----
            uint x2;
            uint y2;
            int xi2;
            int yi2;
            int xi;
            int yi;
            switch (0)
            {
                default:
                {
                    int2 tile64 = int2(workGroup * uint2(64u, 64u));
                    x2 = x * 2u;
                    xi2 = int(x2);
                    y2 = y * 2u;
                    yi2 = int(y2);
                    int2 tile32 = int2(workGroup * uint2(32u, 32u));
                    xi = int(x);
                    yi = int(y);

                    // quadrant (0,0)
                    uint2 q0 = uint2(tile64 + int2(xi2, yi2));
                    int2 q0i = int2(q0);
                    uint2 s0 = uint2(q0i * int2(2, 2));
                    float l00;
                    if (all(s0 < spdConstants.input_dims)) { l00 = imgSrc.Load(int3(s0, 0u)).x; }
                    else { l00 = 1.0f; }
                    uint2 s1 = s0 + uint2(0u, 1u);
                    float l01;
                    if (all(s1 < spdConstants.input_dims)) { l01 = imgSrc.Load(int3(s1, 0u)).x; }
                    else { l01 = 1.0f; }
                    uint2 s2 = s0 + uint2(1u, 0u);
                    float l10;
                    if (all(s2 < spdConstants.input_dims)) { l10 = imgSrc.Load(int3(s2, 0u)).x; }
                    else { l10 = 1.0f; }
                    uint2 s3 = s0 + uint2(1u, 1u);
                    float l11;
                    if (all(s3 < spdConstants.input_dims)) { l11 = imgSrc.Load(int3(s3, 0u)).x; }
                    else { l11 = 1.0f; }
                    float m0_00 = min(min(l00, l01), min(l10, l11));
                    imgDst[0][uint3(int3(q0i, 0))] = m0_00;

                    // quadrant (0,1)
                    int2 q1 = int2(q0 + uint2(0u, 1u));
                    uint2 s4 = uint2(q1 * int2(2, 2));
                    float l20;
                    if (all(s4 < spdConstants.input_dims)) { l20 = imgSrc.Load(int3(s4, 0u)).x; }
                    else { l20 = 1.0f; }
                    uint2 s5 = s4 + uint2(0u, 1u);
                    float l21;
                    if (all(s5 < spdConstants.input_dims)) { l21 = imgSrc.Load(int3(s5, 0u)).x; }
                    else { l21 = 1.0f; }
                    uint2 s6 = s4 + uint2(1u, 0u);
                    float l30;
                    if (all(s6 < spdConstants.input_dims)) { l30 = imgSrc.Load(int3(s6, 0u)).x; }
                    else { l30 = 1.0f; }
                    uint2 s7 = s4 + uint2(1u, 1u);
                    float l31;
                    if (all(s7 < spdConstants.input_dims)) { l31 = imgSrc.Load(int3(s7, 0u)).x; }
                    else { l31 = 1.0f; }
                    float m0_01 = min(min(l20, l21), min(l30, l31));
                    imgDst[0][uint3(int3(q1, 0))] = m0_01;

                    // quadrant (1,0)
                    int2 q2 = int2(q0 + uint2(1u, 0u));
                    uint2 s8 = uint2(q2 * int2(2, 2));
                    float l40;
                    if (all(s8 < spdConstants.input_dims)) { l40 = imgSrc.Load(int3(s8, 0u)).x; }
                    else { l40 = 1.0f; }
                    uint2 s9 = s8 + uint2(0u, 1u);
                    float l41;
                    if (all(s9 < spdConstants.input_dims)) { l41 = imgSrc.Load(int3(s9, 0u)).x; }
                    else { l41 = 1.0f; }
                    uint2 sA = s8 + uint2(1u, 0u);
                    float l50;
                    if (all(sA < spdConstants.input_dims)) { l50 = imgSrc.Load(int3(sA, 0u)).x; }
                    else { l50 = 1.0f; }
                    uint2 sB = s8 + uint2(1u, 1u);
                    float l51;
                    if (all(sB < spdConstants.input_dims)) { l51 = imgSrc.Load(int3(sB, 0u)).x; }
                    else { l51 = 1.0f; }
                    float m0_10 = min(min(l40, l41), min(l50, l51));
                    imgDst[0][uint3(int3(q2, 0))] = m0_10;

                    // quadrant (1,1)
                    int2 q3 = int2(q0 + uint2(1u, 1u));
                    uint2 sC = uint2(q3 * int2(2, 2));
                    float l60;
                    if (all(sC < spdConstants.input_dims)) { l60 = imgSrc.Load(int3(sC, 0u)).x; }
                    else { l60 = 1.0f; }
                    uint2 sD = sC + uint2(0u, 1u);
                    float l61;
                    if (all(sD < spdConstants.input_dims)) { l61 = imgSrc.Load(int3(sD, 0u)).x; }
                    else { l61 = 1.0f; }
                    uint2 sE = sC + uint2(1u, 0u);
                    float l70;
                    if (all(sE < spdConstants.input_dims)) { l70 = imgSrc.Load(int3(sE, 0u)).x; }
                    else { l70 = 1.0f; }
                    uint2 sF = sC + uint2(1u, 1u);
                    float l71;
                    if (all(sF < spdConstants.input_dims)) { l71 = imgSrc.Load(int3(sF, 0u)).x; }
                    else { l71 = 1.0f; }
                    float m0_11 = min(min(l60, l61), min(l70, l71));
                    imgDst[0][uint3(int3(q3, 0))] = m0_11;

                    float m1_00 = min(min(m0_00, m0_01), min(m0_10, m0_11));
                    float4 m1_00f = m1_00.xxxx;
                    imgDst[1][uint3(int3(tile32 + int2(xi, yi), z))] = m1_00;

                    // 16-pixel x-offset quadrant group
                    int xo2 = int(x2 + 32u);
                    int xo1 = int(x + 16u);
                    uint2 qx = uint2(tile64 + int2(xo2, yi2));
                    int2 qxi = int2(qx);
                    uint2 t0 = uint2(qxi * int2(2, 2));
                    float m00;
                    if (all(t0 < spdConstants.input_dims)) { m00 = imgSrc.Load(int3(t0, 0u)).x; }
                    else { m00 = 1.0f; }
                    uint2 t1 = t0 + uint2(0u, 1u);
                    float m01;
                    if (all(t1 < spdConstants.input_dims)) { m01 = imgSrc.Load(int3(t1, 0u)).x; }
                    else { m01 = 1.0f; }
                    uint2 t2 = t0 + uint2(1u, 0u);
                    float m10;
                    if (all(t2 < spdConstants.input_dims)) { m10 = imgSrc.Load(int3(t2, 0u)).x; }
                    else { m10 = 1.0f; }
                    uint2 t3 = t0 + uint2(1u, 1u);
                    float m11;
                    if (all(t3 < spdConstants.input_dims)) { m11 = imgSrc.Load(int3(t3, 0u)).x; }
                    else { m11 = 1.0f; }
                    float p00 = min(min(m00, m01), min(m10, m11));
                    imgDst[0][uint3(int3(qxi, 0))] = p00;

                    int2 qx1 = int2(qx + uint2(0u, 1u));
                    uint2 t4 = uint2(qx1 * int2(2, 2));
                    float m20;
                    if (all(t4 < spdConstants.input_dims)) { m20 = imgSrc.Load(int3(t4, 0u)).x; }
                    else { m20 = 1.0f; }
                    uint2 t5 = t4 + uint2(0u, 1u);
                    float m21;
                    if (all(t5 < spdConstants.input_dims)) { m21 = imgSrc.Load(int3(t5, 0u)).x; }
                    else { m21 = 1.0f; }
                    uint2 t6 = t4 + uint2(1u, 0u);
                    float m30;
                    if (all(t6 < spdConstants.input_dims)) { m30 = imgSrc.Load(int3(t6, 0u)).x; }
                    else { m30 = 1.0f; }
                    uint2 t7 = t4 + uint2(1u, 1u);
                    float m31;
                    if (all(t7 < spdConstants.input_dims)) { m31 = imgSrc.Load(int3(t7, 0u)).x; }
                    else { m31 = 1.0f; }
                    float p01 = min(min(m20, m21), min(m30, m31));
                    imgDst[0][uint3(int3(qx1, 0))] = p01;

                    int2 qx2 = int2(qx + uint2(1u, 0u));
                    uint2 t8 = uint2(qx2 * int2(2, 2));
                    float m40;
                    if (all(t8 < spdConstants.input_dims)) { m40 = imgSrc.Load(int3(t8, 0u)).x; }
                    else { m40 = 1.0f; }
                    uint2 t9 = t8 + uint2(0u, 1u);
                    float m41;
                    if (all(t9 < spdConstants.input_dims)) { m41 = imgSrc.Load(int3(t9, 0u)).x; }
                    else { m41 = 1.0f; }
                    uint2 tA = t8 + uint2(1u, 0u);
                    float m50;
                    if (all(tA < spdConstants.input_dims)) { m50 = imgSrc.Load(int3(tA, 0u)).x; }
                    else { m50 = 1.0f; }
                    uint2 tB = t8 + uint2(1u, 1u);
                    float m51;
                    if (all(tB < spdConstants.input_dims)) { m51 = imgSrc.Load(int3(tB, 0u)).x; }
                    else { m51 = 1.0f; }
                    float p10 = min(min(m40, m41), min(m50, m51));
                    imgDst[0][uint3(int3(qx2, 0))] = p10;

                    int2 qx3 = int2(qx + uint2(1u, 1u));
                    uint2 tC = uint2(qx3 * int2(2, 2));
                    float m60;
                    if (all(tC < spdConstants.input_dims)) { m60 = imgSrc.Load(int3(tC, 0u)).x; }
                    else { m60 = 1.0f; }
                    uint2 tD = tC + uint2(0u, 1u);
                    float m61;
                    if (all(tD < spdConstants.input_dims)) { m61 = imgSrc.Load(int3(tD, 0u)).x; }
                    else { m61 = 1.0f; }
                    uint2 tE = tC + uint2(1u, 0u);
                    float m70;
                    if (all(tE < spdConstants.input_dims)) { m70 = imgSrc.Load(int3(tE, 0u)).x; }
                    else { m70 = 1.0f; }
                    uint2 tF = tC + uint2(1u, 1u);
                    float m71;
                    if (all(tF < spdConstants.input_dims)) { m71 = imgSrc.Load(int3(tF, 0u)).x; }
                    else { m71 = 1.0f; }
                    float p11 = min(min(m60, m61), min(m70, m71));
                    imgDst[0][uint3(int3(qx3, 0))] = p11;

                    float m1_10 = min(min(p00, p01), min(p10, p11));
                    float4 m1_10f = m1_10.xxxx;
                    imgDst[1][uint3(int3(tile32 + int2(xo1, yi), z))] = m1_10;

                    // 16-pixel y-offset quadrant group
                    int yo2 = int(y2 + 32u);
                    int yo1 = int(y + 16u);
                    uint2 qy = uint2(tile64 + int2(xi2, yo2));
                    int2 qyi = int2(qy);
                    uint2 u0 = uint2(qyi * int2(2, 2));
                    float n00;
                    if (all(u0 < spdConstants.input_dims)) { n00 = imgSrc.Load(int3(u0, 0u)).x; }
                    else { n00 = 1.0f; }
                    uint2 u1 = u0 + uint2(0u, 1u);
                    float n01;
                    if (all(u1 < spdConstants.input_dims)) { n01 = imgSrc.Load(int3(u1, 0u)).x; }
                    else { n01 = 1.0f; }
                    uint2 u2 = u0 + uint2(1u, 0u);
                    float n10;
                    if (all(u2 < spdConstants.input_dims)) { n10 = imgSrc.Load(int3(u2, 0u)).x; }
                    else { n10 = 1.0f; }
                    uint2 u3 = u0 + uint2(1u, 1u);
                    float n11;
                    if (all(u3 < spdConstants.input_dims)) { n11 = imgSrc.Load(int3(u3, 0u)).x; }
                    else { n11 = 1.0f; }
                    float q00 = min(min(n00, n01), min(n10, n11));
                    imgDst[0][uint3(int3(qyi, 0))] = q00;

                    int2 qy1 = int2(qy + uint2(0u, 1u));
                    uint2 u4 = uint2(qy1 * int2(2, 2));
                    float n20;
                    if (all(u4 < spdConstants.input_dims)) { n20 = imgSrc.Load(int3(u4, 0u)).x; }
                    else { n20 = 1.0f; }
                    uint2 u5 = u4 + uint2(0u, 1u);
                    float n21;
                    if (all(u5 < spdConstants.input_dims)) { n21 = imgSrc.Load(int3(u5, 0u)).x; }
                    else { n21 = 1.0f; }
                    uint2 u6 = u4 + uint2(1u, 0u);
                    float n30;
                    if (all(u6 < spdConstants.input_dims)) { n30 = imgSrc.Load(int3(u6, 0u)).x; }
                    else { n30 = 1.0f; }
                    uint2 u7 = u4 + uint2(1u, 1u);
                    float n31;
                    if (all(u7 < spdConstants.input_dims)) { n31 = imgSrc.Load(int3(u7, 0u)).x; }
                    else { n31 = 1.0f; }
                    float q01 = min(min(n20, n21), min(n30, n31));
                    imgDst[0][uint3(int3(qy1, 0))] = q01;

                    int2 qy2 = int2(qy + uint2(1u, 0u));
                    uint2 u8 = uint2(qy2 * int2(2, 2));
                    float n40;
                    if (all(u8 < spdConstants.input_dims)) { n40 = imgSrc.Load(int3(u8, 0u)).x; }
                    else { n40 = 1.0f; }
                    uint2 u9 = u8 + uint2(0u, 1u);
                    float n41;
                    if (all(u9 < spdConstants.input_dims)) { n41 = imgSrc.Load(int3(u9, 0u)).x; }
                    else { n41 = 1.0f; }
                    uint2 uA = u8 + uint2(1u, 0u);
                    float n50;
                    if (all(uA < spdConstants.input_dims)) { n50 = imgSrc.Load(int3(uA, 0u)).x; }
                    else { n50 = 1.0f; }
                    uint2 uB = u8 + uint2(1u, 1u);
                    float n51;
                    if (all(uB < spdConstants.input_dims)) { n51 = imgSrc.Load(int3(uB, 0u)).x; }
                    else { n51 = 1.0f; }
                    float q10 = min(min(n40, n41), min(n50, n51));
                    imgDst[0][uint3(int3(qy2, 0))] = q10;

                    int2 qy3 = int2(qy + uint2(1u, 1u));
                    uint2 uC = uint2(qy3 * int2(2, 2));
                    float n60;
                    if (all(uC < spdConstants.input_dims)) { n60 = imgSrc.Load(int3(uC, 0u)).x; }
                    else { n60 = 1.0f; }
                    uint2 uD = uC + uint2(0u, 1u);
                    float n61;
                    if (all(uD < spdConstants.input_dims)) { n61 = imgSrc.Load(int3(uD, 0u)).x; }
                    else { n61 = 1.0f; }
                    uint2 uE = uC + uint2(1u, 0u);
                    float n70;
                    if (all(uE < spdConstants.input_dims)) { n70 = imgSrc.Load(int3(uE, 0u)).x; }
                    else { n70 = 1.0f; }
                    uint2 uF = uC + uint2(1u, 1u);
                    float n71;
                    if (all(uF < spdConstants.input_dims)) { n71 = imgSrc.Load(int3(uF, 0u)).x; }
                    else { n71 = 1.0f; }
                    float q11 = min(min(n60, n61), min(n70, n71));
                    imgDst[0][uint3(int3(qy3, 0))] = q11;

                    float m1_01 = min(min(q00, q01), min(q10, q11));
                    float4 m1_01f = m1_01.xxxx;
                    imgDst[1][uint3(int3(tile32 + int2(xi, yo1), z))] = m1_01;

                    // 16-pixel xy-offset quadrant group
                    uint2 qxy = uint2(tile64 + int2(xo2, yo2));
                    int2 qxyi = int2(qxy);
                    uint2 v0 = uint2(qxyi * int2(2, 2));
                    float r00;
                    if (all(v0 < spdConstants.input_dims)) { r00 = imgSrc.Load(int3(v0, 0u)).x; }
                    else { r00 = 1.0f; }
                    uint2 v1 = v0 + uint2(0u, 1u);
                    float r01;
                    if (all(v1 < spdConstants.input_dims)) { r01 = imgSrc.Load(int3(v1, 0u)).x; }
                    else { r01 = 1.0f; }
                    uint2 v2 = v0 + uint2(1u, 0u);
                    float r10;
                    if (all(v2 < spdConstants.input_dims)) { r10 = imgSrc.Load(int3(v2, 0u)).x; }
                    else { r10 = 1.0f; }
                    uint2 v3 = v0 + uint2(1u, 1u);
                    float r11;
                    if (all(v3 < spdConstants.input_dims)) { r11 = imgSrc.Load(int3(v3, 0u)).x; }
                    else { r11 = 1.0f; }
                    float mip0_q0 = min(min(r00, r01), min(r10, r11));
                    imgDst[0][uint3(int3(qxyi, 0))] = mip0_q0;

                    int2 qxy1 = int2(qxy + uint2(0u, 1u));
                    uint2 v4 = uint2(qxy1 * int2(2, 2));
                    float r20;
                    if (all(v4 < spdConstants.input_dims)) { r20 = imgSrc.Load(int3(v4, 0u)).x; }
                    else { r20 = 1.0f; }
                    uint2 v5 = v4 + uint2(0u, 1u);
                    float r21;
                    if (all(v5 < spdConstants.input_dims)) { r21 = imgSrc.Load(int3(v5, 0u)).x; }
                    else { r21 = 1.0f; }
                    uint2 v6 = v4 + uint2(1u, 0u);
                    float r30;
                    if (all(v6 < spdConstants.input_dims)) { r30 = imgSrc.Load(int3(v6, 0u)).x; }
                    else { r30 = 1.0f; }
                    uint2 v7 = v4 + uint2(1u, 1u);
                    float r31;
                    if (all(v7 < spdConstants.input_dims)) { r31 = imgSrc.Load(int3(v7, 0u)).x; }
                    else { r31 = 1.0f; }
                    float mip0_q1 = min(min(r20, r21), min(r30, r31));
                    imgDst[0][uint3(int3(qxy1, 0))] = mip0_q1;

                    int2 qxy2 = int2(qxy + uint2(1u, 0u));
                    uint2 v8 = uint2(qxy2 * int2(2, 2));
                    float r40;
                    if (all(v8 < spdConstants.input_dims)) { r40 = imgSrc.Load(int3(v8, 0u)).x; }
                    else { r40 = 1.0f; }
                    uint2 v9 = v8 + uint2(0u, 1u);
                    float r41;
                    if (all(v9 < spdConstants.input_dims)) { r41 = imgSrc.Load(int3(v9, 0u)).x; }
                    else { r41 = 1.0f; }
                    uint2 vA = v8 + uint2(1u, 0u);
                    float r50;
                    if (all(vA < spdConstants.input_dims)) { r50 = imgSrc.Load(int3(vA, 0u)).x; }
                    else { r50 = 1.0f; }
                    uint2 vB = v8 + uint2(1u, 1u);
                    float r51;
                    if (all(vB < spdConstants.input_dims)) { r51 = imgSrc.Load(int3(vB, 0u)).x; }
                    else { r51 = 1.0f; }
                    float mip0_q2 = min(min(r40, r41), min(r50, r51));
                    imgDst[0][uint3(int3(qxy2, 0))] = mip0_q2;

                    int2 qxy3 = int2(qxy + uint2(1u, 1u));
                    uint2 vC = uint2(qxy3 * int2(2, 2));
                    float r60;
                    if (all(vC < spdConstants.input_dims)) { r60 = imgSrc.Load(int3(vC, 0u)).x; }
                    else { r60 = 1.0f; }
                    uint2 vD = vC + uint2(0u, 1u);
                    float r61;
                    if (all(vD < spdConstants.input_dims)) { r61 = imgSrc.Load(int3(vD, 0u)).x; }
                    else { r61 = 1.0f; }
                    uint2 vE = vC + uint2(1u, 0u);
                    float r70;
                    if (all(vE < spdConstants.input_dims)) { r70 = imgSrc.Load(int3(vE, 0u)).x; }
                    else { r70 = 1.0f; }
                    uint2 vF = vC + uint2(1u, 1u);
                    float r71;
                    if (all(vF < spdConstants.input_dims)) { r71 = imgSrc.Load(int3(vF, 0u)).x; }
                    else { r71 = 1.0f; }
                    float mip0_q3 = min(min(r60, r61), min(r70, r71));
                    imgDst[0][uint3(int3(qxy3, 0))] = mip0_q3;

                    float m1_11 = min(min(mip0_q0, mip0_q1), min(mip0_q2, mip0_q3));
                    float4 m1_11f = m1_11.xxxx;
                    imgDst[1][uint3(int3(tile32 + int2(xo1, yo1), z))] = m1_11;

                    if (spdConstants.mips <= 1u)
                    {
                        break;
                    }

                    // 2x2 wave reduce over mip1 (4 lanes per quad)
                    uint base = WaveGetLaneIndex() & 0xFFFFFFFCu;
                    uint lane1 = base | 1u;
                    float4 m1_00_w1 = WaveReadLaneAt(m1_00f, lane1);
                    uint lane2 = base | 2u;
                    float4 m1_00_w2 = WaveReadLaneAt(m1_00f, lane2);
                    uint lane3 = base | 3u;
                    float4 m1_00_w3 = WaveReadLaneAt(m1_00f, lane3);
                    float m2_00 = min(min(m1_00, m1_00_w1.x), min(m1_00_w2.x, m1_00_w3.x));
                    float4 m1_10_w1 = WaveReadLaneAt(m1_10f, lane1);
                    float4 m1_10_w2 = WaveReadLaneAt(m1_10f, lane2);
                    float4 m1_10_w3 = WaveReadLaneAt(m1_10f, lane3);
                    float m2_10 = min(min(m1_10, m1_10_w1.x), min(m1_10_w2.x, m1_10_w3.x));
                    float4 m1_01_w1 = WaveReadLaneAt(m1_01f, lane1);
                    float4 m1_01_w2 = WaveReadLaneAt(m1_01f, lane2);
                    float4 m1_01_w3 = WaveReadLaneAt(m1_01f, lane3);
                    float m2_01 = min(min(m1_01, m1_01_w1.x), min(m1_01_w2.x, m1_01_w3.x));
                    float4 m1_11_w1 = WaveReadLaneAt(m1_11f, lane1);
                    float4 m1_11_w2 = WaveReadLaneAt(m1_11f, lane2);
                    float4 m1_11_w3 = WaveReadLaneAt(m1_11f, lane3);
                    float m2_11 = min(min(m1_11, m1_11_w1.x), min(m1_11_w2.x, m1_11_w3.x));

                    if ((gl_LocalInvocationIndex % 4u) == 0u)
                    {
                        int2 tile16 = int2(workGroup * uint2(16u, 16u));
                        uint xh = x / 2u;
                        int xhi = int(xh);
                        uint yh = y / 2u;
                        int yhi = int(yh);
                        imgDst[2][uint3(int3(tile16 + int2(xhi, yhi), z))] = m2_00;
                        spdIntermediateR[xh][yh] = m2_00;
                        uint xh8 = xh + 8u;
                        int xh8i = int(xh8);
                        imgDst[2][uint3(int3(tile16 + int2(xh8i, yhi), z))] = m2_10;
                        spdIntermediateR[xh8][yh] = m2_10;
                        uint yh8 = yh + 8u;
                        int yh8i = int(yh8);
                        imgDst[2][uint3(int3(tile16 + int2(xhi, yh8i), z))] = m2_01;
                        spdIntermediateR[xh][yh8] = m2_01;
                        imgDst[2][uint3(int3(tile16 + int2(xh8i, yh8i), z))] = m2_11;
                        spdIntermediateR[xh8][yh8] = m2_11;
                    }
                    break;
                }
            }

            // ---- mips 2..6 (block2) ----
            switch (0)
            {
                default:
                {
                    if (spdConstants.mips <= 2u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    float4 iv_r = float4(spdIntermediateR[x][y], 0.0f, 0.0f, 0.0f);
                    uint base2 = WaveGetLaneIndex() & 0xFFFFFFFCu;
                    uint l2a = base2 | 1u;
                    float4 iv_rw1 = WaveReadLaneAt(iv_r, l2a);
                    uint l2b = base2 | 2u;
                    float4 iv_rw2 = WaveReadLaneAt(iv_r, l2b);
                    uint l2c = base2 | 3u;
                    float4 iv_rw3 = WaveReadLaneAt(iv_r, l2c);
                    float m3 = min(min(spdIntermediateR[x][y], iv_rw1.x), min(iv_rw2.x, iv_rw3.x));
                    bool isQuad = (gl_LocalInvocationIndex % 4u) == 0u;
                    if (isQuad)
                    {
                        uint y2h = y / 2u;
                        imgDst[3][uint3(int3(int2(workGroup * uint2(8u, 8u)) + int2(int(x / 2u), int(y2h)), z))] = m3;
                        spdIntermediateR[x + (y2h % 2u)][y] = m3;
                    }
                    if (spdConstants.mips <= 3u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    if (gl_LocalInvocationIndex < 64u)
                    {
                        uint xx = x2 + (y % 2u);
                        float4 iv2_r = float4(spdIntermediateR[xx][y2], 0.0f, 0.0f, 0.0f);
                        float4 iv2_w1 = WaveReadLaneAt(iv2_r, l2a);
                        float4 iv2_w2 = WaveReadLaneAt(iv2_r, l2b);
                        float4 iv2_w3 = WaveReadLaneAt(iv2_r, l2c);
                        float m4 = min(min(spdIntermediateR[xx][y2], iv2_w1.x), min(iv2_w2.x, iv2_w3.x));
                        if (isQuad)
                        {
                            uint y3h = y / 2u;
                            imgDst[4][uint3(int3(int2(workGroup * uint2(4u, 4u)) + int2(int(x / 2u), int(y3h)), z))] = m4;
                            spdIntermediateR[x2 + y3h][y2] = m4;
                        }
                    }
                    if (spdConstants.mips <= 4u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    if (gl_LocalInvocationIndex < 16u)
                    {
                        uint i4 = (x * 4u) + y;
                        uint j4 = y * 4u;
                        float4 iv4_r = float4(spdIntermediateR[i4][j4], 0.0f, 0.0f, 0.0f);
                        float4 iv4_w1 = WaveReadLaneAt(iv4_r, l2a);
                        float4 iv4_w2 = WaveReadLaneAt(iv4_r, l2b);
                        float4 iv4_w3 = WaveReadLaneAt(iv4_r, l2c);
                        float m5 = min(min(spdIntermediateR[i4][j4], iv4_w1.x), min(iv4_w2.x, iv4_w3.x));
                        if (isQuad)
                        {
                            uint x2h = x / 2u;
                            imgDst[5][uint3(int3(int2(workGroup * uint2(2u, 2u)) + int2(int(x2h), int(y / 2u)), z))] = m5;
                            spdIntermediateR[x2h + y][0u] = m5;
                        }
                    }
                    if (spdConstants.mips <= 5u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    if (gl_LocalInvocationIndex < 4u)
                    {
                        float4 iv6_r = float4(spdIntermediateR[gl_LocalInvocationIndex][0u], 0.0f, 0.0f, 0.0f);
                        float4 iv6_w1 = WaveReadLaneAt(iv6_r, l2a);
                        float4 iv6_w2 = WaveReadLaneAt(iv6_r, l2b);
                        float4 iv6_w3 = WaveReadLaneAt(iv6_r, l2c);
                        if (isQuad)
                        {
                            imgDst6[uint3(int3(int2(workGroup), z))] = min(min(spdIntermediateR[gl_LocalInvocationIndex][0u], iv6_w1.x), min(iv6_w2.x, iv6_w3.x));
                        }
                    }
                    break;
                }
            }

            if (spdConstants.mips <= 6u)
            {
                break;
            }
            if (gl_LocalInvocationIndex == 0u)
            {
                uint prev;
                InterlockedAdd(spdGlobalAtomic[0].counter[gl_WorkGroupID.z], 1u, prev);
                spdCounter = prev;
            }
            GroupMemoryBarrierWithGroupSync();
            if (spdCounter != (spdConstants.numWorkGroups - 1u))
            {
                break;
            }
            spdGlobalAtomic[0].counter[gl_WorkGroupID.z] = 0u;

            // ---- mip7 4x4 downsample of mip6 (block3) ----
            uint x4 = x * 4u;
            uint y4 = y * 4u;
            switch (0)
            {
                default:
                {
                    int x4i = int(x4);
                    int y4i = int(y4);
                    uint2 r0 = uint2(int2(x4i, y4i));
                    float a0 = imgDst6[uint3(int3(int2(r0), z))];
                    float a1 = imgDst6[uint3(int3(int2(r0 + uint2(0u, 1u)), z))];
                    float a2 = imgDst6[uint3(int3(int2(r0 + uint2(1u, 0u)), z))];
                    float a3 = imgDst6[uint3(int3(int2(r0 + uint2(1u, 1u)), z))];
                    float m7_00 = min(min(a0, a1), min(a2, a3));
                    imgDst[7][uint3(int3(xi2, yi2, z))] = m7_00;

                    int x4i2 = int(x4 + 2u);
                    int xi1 = int(x2 + 1u);
                    uint2 r1 = uint2(int2(x4i2, y4i));
                    float b0 = imgDst6[uint3(int3(int2(r1), z))];
                    float b1 = imgDst6[uint3(int3(int2(r1 + uint2(0u, 1u)), z))];
                    float b2 = imgDst6[uint3(int3(int2(r1 + uint2(1u, 0u)), z))];
                    float b3 = imgDst6[uint3(int3(int2(r1 + uint2(1u, 1u)), z))];
                    float m7_10 = min(min(b0, b1), min(b2, b3));
                    imgDst[7][uint3(int3(xi1, yi2, z))] = m7_10;

                    int y4i2 = int(y4 + 2u);
                    int yi1 = int(y2 + 1u);
                    uint2 r2 = uint2(int2(x4i, y4i2));
                    float c0 = imgDst6[uint3(int3(int2(r2), z))];
                    float c1 = imgDst6[uint3(int3(int2(r2 + uint2(0u, 1u)), z))];
                    float c2 = imgDst6[uint3(int3(int2(r2 + uint2(1u, 0u)), z))];
                    float c3 = imgDst6[uint3(int3(int2(r2 + uint2(1u, 1u)), z))];
                    float m7_01 = min(min(c0, c1), min(c2, c3));
                    imgDst[7][uint3(int3(xi2, yi1, z))] = m7_01;

                    uint2 r3 = uint2(int2(x4i2, y4i2));
                    float d0 = imgDst6[uint3(int3(int2(r3), z))];
                    float d1 = imgDst6[uint3(int3(int2(r3 + uint2(0u, 1u)), z))];
                    float d2 = imgDst6[uint3(int3(int2(r3 + uint2(1u, 0u)), z))];
                    float d3 = imgDst6[uint3(int3(int2(r3 + uint2(1u, 1u)), z))];
                    float m7_11 = min(min(d0, d1), min(d2, d3));
                    imgDst[7][uint3(int3(xi1, yi1, z))] = m7_11;

                    if (spdConstants.mips <= 7u)
                    {
                        break;
                    }
                    float m8 = min(min(m7_00, m7_10), min(m7_01, m7_11));
                    imgDst[8][uint3(int3(xi, yi, z))] = m8;
                    spdIntermediateR[x][y] = m8;
                    break;
                }
            }

            // ---- mips 8..12 (block3b) ----
            switch (0)
            {
                default:
                {
                    if (spdConstants.mips <= 8u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    float4 j0_r = float4(spdIntermediateR[x][y], 0.0f, 0.0f, 0.0f);
                    uint base3 = WaveGetLaneIndex() & 0xFFFFFFFCu;
                    uint l3a = base3 | 1u;
                    float4 j0_w1 = WaveReadLaneAt(j0_r, l3a);
                    uint l3b = base3 | 2u;
                    float4 j0_w2 = WaveReadLaneAt(j0_r, l3b);
                    uint l3c = base3 | 3u;
                    float4 j0_w3 = WaveReadLaneAt(j0_r, l3c);
                    float m9 = min(min(spdIntermediateR[x][y], j0_w1.x), min(j0_w2.x, j0_w3.x));
                    bool isQuad3 = (gl_LocalInvocationIndex % 4u) == 0u;
                    if (isQuad3)
                    {
                        uint y2h3 = y / 2u;
                        imgDst[9][uint3(int3(int(x / 2u), int(y2h3), z))] = m9;
                        spdIntermediateR[x + (y2h3 % 2u)][y] = m9;
                    }
                    if (spdConstants.mips <= 9u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    if (gl_LocalInvocationIndex < 64u)
                    {
                        uint xx3 = x2 + (y % 2u);
                        float4 j1_r = float4(spdIntermediateR[xx3][y2], 0.0f, 0.0f, 0.0f);
                        float4 j1_w1 = WaveReadLaneAt(j1_r, l3a);
                        float4 j1_w2 = WaveReadLaneAt(j1_r, l3b);
                        float4 j1_w3 = WaveReadLaneAt(j1_r, l3c);
                        float m10 = min(min(spdIntermediateR[xx3][y2], j1_w1.x), min(j1_w2.x, j1_w3.x));
                        if (isQuad3)
                        {
                            uint y3h3 = y / 2u;
                            imgDst[10][uint3(int3(int(x / 2u), int(y3h3), z))] = m10;
                            spdIntermediateR[x2 + y3h3][y2] = m10;
                        }
                    }
                    if (spdConstants.mips <= 10u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    if (gl_LocalInvocationIndex < 16u)
                    {
                        uint i4b = x4 + y;
                        float4 j2_r = float4(spdIntermediateR[i4b][y4], 0.0f, 0.0f, 0.0f);
                        float4 j2_w1 = WaveReadLaneAt(j2_r, l3a);
                        float4 j2_w2 = WaveReadLaneAt(j2_r, l3b);
                        float4 j2_w3 = WaveReadLaneAt(j2_r, l3c);
                        float m11 = min(min(spdIntermediateR[i4b][y4], j2_w1.x), min(j2_w2.x, j2_w3.x));
                        if (isQuad3)
                        {
                            uint x2h3 = x / 2u;
                            imgDst[11][uint3(int3(int(x2h3), int(y / 2u), z))] = m11;
                            spdIntermediateR[x2h3 + y][0u] = m11;
                        }
                    }
                    if (spdConstants.mips <= 11u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    if (gl_LocalInvocationIndex < 4u)
                    {
                        float4 j3_r = float4(spdIntermediateR[gl_LocalInvocationIndex][0u], 0.0f, 0.0f, 0.0f);
                        float4 j3_w1 = WaveReadLaneAt(j3_r, l3a);
                        float4 j3_w2 = WaveReadLaneAt(j3_r, l3b);
                        float4 j3_w3 = WaveReadLaneAt(j3_r, l3c);
                        if (isQuad3)
                        {
                            imgDst[12][uint3(int3(0, 0, z))] = min(min(spdIntermediateR[gl_LocalInvocationIndex][0u], j3_w1.x), min(j3_w2.x, j3_w3.x));
                        }
                    }
                    break;
                }
            }
            break;
        }
    }
}
