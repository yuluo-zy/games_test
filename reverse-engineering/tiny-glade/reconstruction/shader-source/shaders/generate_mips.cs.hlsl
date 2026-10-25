// ==RECONSTRUCTED== shaders/generate_mips.cs.hlsl | F-D
// Evidence: _generate_mips.cs.hlsl_cs_main_0.bin, entry cs_main, LocalSize [256,1,1].
//   FidelityFX Single Pass Downsampler variant (blur_mips / 4-channel): push constants
//   spdConstants (mips, numWorkGroups, workGroupOffset; NO input_dims), imgSrc u0
//   (RWTexture2D<float4> r11g11b10f storage), imgDst u1 (RWTexture2DArray<float4>
//   r11g11b10f, 12 slices = mip0..11), imgDst6 u2 (r11g11b10f, coherent),
//   spdGlobalAtomic u3 (coherent RWStructuredBuffer { uint counter[6] }). groupshared
//   spdCounter + spdIntermediateR/G/B/A[16][16]. 2x2 box average (*0.25), wave 4-lane
//   reduce (GroupNonUniformShuffle). Ops reconstructed from spirv-cross (structure
//   preserved: OpSwitch execute-once blocks, full-vec4 image reads/writes).
// Status: O (structure from disassembly).

struct SpdConstants
{
    uint mips;
    uint numWorkGroups;
    uint2 workGroupOffset;
};

struct SpdGlobalAtomicBuffer
{
    uint counter[6];
};

[[vk::push_constant]]
ConstantBuffer<SpdConstants> spdConstants;

[[vk::image_format("r11g11b10f")]] RWTexture2D<float4> imgSrc : register(u0, space0);
[[vk::image_format("r11g11b10f")]] RWTexture2DArray<float4> imgDst[12] : register(u1, space0);
[[vk::image_format("r11g11b10f")]] globallycoherent RWTexture2DArray<float4> imgDst6 : register(u2, space0);
globallycoherent RWStructuredBuffer<SpdGlobalAtomicBuffer> spdGlobalAtomic : register(u3, space0);

groupshared uint spdCounter;
groupshared float spdIntermediateR[16][16];
groupshared float spdIntermediateG[16][16];
groupshared float spdIntermediateB[16][16];
groupshared float spdIntermediateA[16][16];

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
            uint x2;
            uint y2;
            int xi2;
            int yi2;
            int xi;
            int yi;
            uint x4;
            uint y4;

            // ---- mip0 2x2 box average + mip1 4x4 wave average (block1) ----
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

                    uint2 q0 = uint2(tile64 + int2(xi2, yi2));
                    float4 a00 = imgSrc[uint2(int2(q0))];
                    float4 a01 = imgSrc[uint2(int2(q0 + uint2(0u, 1u)))];
                    float4 a10 = imgSrc[uint2(int2(q0 + uint2(1u, 0u)))];
                    float4 a11 = imgSrc[uint2(int2(q0 + uint2(1u, 1u)))];
                    float4 m0_00 = (((a00 + a01) + a10) + a11) * 0.25f;
                    imgDst[0][uint3(int3(tile32 + int2(xi, yi), z))] = m0_00;

                    int xo2 = int(x2 + 32u);
                    int xo1 = int(x + 16u);
                    uint2 qx = uint2(tile64 + int2(xo2, yi2));
                    float4 b00 = imgSrc[uint2(int2(qx))];
                    float4 b01 = imgSrc[uint2(int2(qx + uint2(0u, 1u)))];
                    float4 b10 = imgSrc[uint2(int2(qx + uint2(1u, 0u)))];
                    float4 b11 = imgSrc[uint2(int2(qx + uint2(1u, 1u)))];
                    float4 m0_10 = (((b00 + b01) + b10) + b11) * 0.25f;
                    imgDst[0][uint3(int3(tile32 + int2(xo1, yi), z))] = m0_10;

                    int yo2 = int(y2 + 32u);
                    int yo1 = int(y + 16u);
                    uint2 qy = uint2(tile64 + int2(xi2, yo2));
                    float4 c00 = imgSrc[uint2(int2(qy))];
                    float4 c01 = imgSrc[uint2(int2(qy + uint2(0u, 1u)))];
                    float4 c10 = imgSrc[uint2(int2(qy + uint2(1u, 0u)))];
                    float4 c11 = imgSrc[uint2(int2(qy + uint2(1u, 1u)))];
                    float4 m0_01 = (((c00 + c01) + c10) + c11) * 0.25f;
                    imgDst[0][uint3(int3(tile32 + int2(xi, yo1), z))] = m0_01;

                    uint2 qxy = uint2(tile64 + int2(xo2, yo2));
                    float4 d00 = imgSrc[uint2(int2(qxy))];
                    float4 d01 = imgSrc[uint2(int2(qxy + uint2(0u, 1u)))];
                    float4 d10 = imgSrc[uint2(int2(qxy + uint2(1u, 0u)))];
                    float4 d11 = imgSrc[uint2(int2(qxy + uint2(1u, 1u)))];
                    float4 m0_11 = (((d00 + d01) + d10) + d11) * 0.25f;
                    imgDst[0][uint3(int3(tile32 + int2(xo1, yo1), z))] = m0_11;

                    if (spdConstants.mips <= 1u)
                    {
                        break;
                    }

                    // 2x2 wave average over mip0 (4 lanes per quad)
                    uint base = WaveGetLaneIndex() & 0xFFFFFFFCu;
                    uint lane1 = base | 1u;
                    float4 m0_00_w1 = WaveReadLaneAt(m0_00, lane1);
                    uint lane2 = base | 2u;
                    float4 m0_00_w2 = WaveReadLaneAt(m0_00, lane2);
                    uint lane3 = base | 3u;
                    float4 m0_00_w3 = WaveReadLaneAt(m0_00, lane3);
                    float4 m1_00 = (((m0_00 + m0_00_w1) + m0_00_w2) + m0_00_w3) * 0.25f;
                    float4 m0_10_w1 = WaveReadLaneAt(m0_10, lane1);
                    float4 m0_10_w2 = WaveReadLaneAt(m0_10, lane2);
                    float4 m0_10_w3 = WaveReadLaneAt(m0_10, lane3);
                    float4 m1_10 = (((m0_10 + m0_10_w1) + m0_10_w2) + m0_10_w3) * 0.25f;
                    float4 m0_01_w1 = WaveReadLaneAt(m0_01, lane1);
                    float4 m0_01_w2 = WaveReadLaneAt(m0_01, lane2);
                    float4 m0_01_w3 = WaveReadLaneAt(m0_01, lane3);
                    float4 m1_01 = (((m0_01 + m0_01_w1) + m0_01_w2) + m0_01_w3) * 0.25f;
                    float4 m0_11_w1 = WaveReadLaneAt(m0_11, lane1);
                    float4 m0_11_w2 = WaveReadLaneAt(m0_11, lane2);
                    float4 m0_11_w3 = WaveReadLaneAt(m0_11, lane3);
                    float4 m1_11 = (((m0_11 + m0_11_w1) + m0_11_w2) + m0_11_w3) * 0.25f;

                    if ((gl_LocalInvocationIndex % 4u) == 0u)
                    {
                        int2 tile16 = int2(workGroup * uint2(16u, 16u));
                        uint xh = x / 2u;
                        int xhi = int(xh);
                        uint yh = y / 2u;
                        int yhi = int(yh);
                        imgDst[1][uint3(int3(tile16 + int2(xhi, yhi), z))] = m1_00;
                        spdIntermediateR[xh][yh] = m1_00.x;
                        spdIntermediateG[xh][yh] = m1_00.y;
                        spdIntermediateB[xh][yh] = m1_00.z;
                        spdIntermediateA[xh][yh] = m1_00.w;
                        uint xh8 = xh + 8u;
                        int xh8i = int(xh8);
                        imgDst[1][uint3(int3(tile16 + int2(xh8i, yhi), z))] = m1_10;
                        spdIntermediateR[xh8][yh] = m1_10.x;
                        spdIntermediateG[xh8][yh] = m1_10.y;
                        spdIntermediateB[xh8][yh] = m1_10.z;
                        spdIntermediateA[xh8][yh] = m1_10.w;
                        uint yh8 = yh + 8u;
                        int yh8i = int(yh8);
                        imgDst[1][uint3(int3(tile16 + int2(xhi, yh8i), z))] = m1_01;
                        spdIntermediateR[xh][yh8] = m1_01.x;
                        spdIntermediateG[xh][yh8] = m1_01.y;
                        spdIntermediateB[xh][yh8] = m1_01.z;
                        spdIntermediateA[xh][yh8] = m1_01.w;
                        imgDst[1][uint3(int3(tile16 + int2(xh8i, yh8i), z))] = m1_11;
                        spdIntermediateR[xh8][yh8] = m1_11.x;
                        spdIntermediateG[xh8][yh8] = m1_11.y;
                        spdIntermediateB[xh8][yh8] = m1_11.z;
                        spdIntermediateA[xh8][yh8] = m1_11.w;
                    }
                    break;
                }
            }

            // ---- mips 2..5 (block2) ----
            switch (0)
            {
                default:
                {
                    if (spdConstants.mips <= 2u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    float4 iv_r = float4(spdIntermediateR[x][y], spdIntermediateG[x][y], spdIntermediateB[x][y], spdIntermediateA[x][y]);
                    uint base2 = WaveGetLaneIndex() & 0xFFFFFFFCu;
                    uint l2a = base2 | 1u;
                    float4 iv_rw1 = WaveReadLaneAt(iv_r, l2a);
                    uint l2b = base2 | 2u;
                    float4 iv_rw2 = WaveReadLaneAt(iv_r, l2b);
                    uint l2c = base2 | 3u;
                    float4 iv_rw3 = WaveReadLaneAt(iv_r, l2c);
                    float4 m2 = (((iv_r + iv_rw1) + iv_rw2) + iv_rw3) * 0.25f;
                    bool isQuad = (gl_LocalInvocationIndex % 4u) == 0u;
                    if (isQuad)
                    {
                        uint y2h = y / 2u;
                        imgDst[2][uint3(int3(int2(workGroup * uint2(8u, 8u)) + int2(int(x / 2u), int(y2h)), z))] = m2;
                        uint xx = x + (y2h % 2u);
                        spdIntermediateR[xx][y] = m2.x;
                        spdIntermediateG[xx][y] = m2.y;
                        spdIntermediateB[xx][y] = m2.z;
                        spdIntermediateA[xx][y] = m2.w;
                    }
                    if (spdConstants.mips <= 3u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    if (gl_LocalInvocationIndex < 64u)
                    {
                        uint xo = x2 + (y % 2u);
                        float4 iv2_r = float4(spdIntermediateR[xo][y2], spdIntermediateG[xo][y2], spdIntermediateB[xo][y2], spdIntermediateA[xo][y2]);
                        float4 iv2_w1 = WaveReadLaneAt(iv2_r, l2a);
                        float4 iv2_w2 = WaveReadLaneAt(iv2_r, l2b);
                        float4 iv2_w3 = WaveReadLaneAt(iv2_r, l2c);
                        float4 m3 = (((iv2_r + iv2_w1) + iv2_w2) + iv2_w3) * 0.25f;
                        if (isQuad)
                        {
                            uint y3h = y / 2u;
                            imgDst[3][uint3(int3(int2(workGroup * uint2(4u, 4u)) + int2(int(x / 2u), int(y3h)), z))] = m3;
                            uint xo2_ = x2 + y3h;
                            spdIntermediateR[xo2_][y2] = m3.x;
                            spdIntermediateG[xo2_][y2] = m3.y;
                            spdIntermediateB[xo2_][y2] = m3.z;
                            spdIntermediateA[xo2_][y2] = m3.w;
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
                        float4 iv4_r = float4(spdIntermediateR[i4][j4], spdIntermediateG[i4][j4], spdIntermediateB[i4][j4], spdIntermediateA[i4][j4]);
                        float4 iv4_w1 = WaveReadLaneAt(iv4_r, l2a);
                        float4 iv4_w2 = WaveReadLaneAt(iv4_r, l2b);
                        float4 iv4_w3 = WaveReadLaneAt(iv4_r, l2c);
                        float4 m4 = (((iv4_r + iv4_w1) + iv4_w2) + iv4_w3) * 0.25f;
                        if (isQuad)
                        {
                            uint x2h = x / 2u;
                            imgDst[4][uint3(int3(int2(workGroup * uint2(2u, 2u)) + int2(int(x2h), int(y / 2u)), z))] = m4;
                            uint xo3 = x2h + y;
                            spdIntermediateR[xo3][0u] = m4.x;
                            spdIntermediateG[xo3][0u] = m4.y;
                            spdIntermediateB[xo3][0u] = m4.z;
                            spdIntermediateA[xo3][0u] = m4.w;
                        }
                    }
                    if (spdConstants.mips <= 5u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    if (gl_LocalInvocationIndex < 4u)
                    {
                        float4 iv6_r = float4(spdIntermediateR[gl_LocalInvocationIndex][0u], spdIntermediateG[gl_LocalInvocationIndex][0u], spdIntermediateB[gl_LocalInvocationIndex][0u], spdIntermediateA[gl_LocalInvocationIndex][0u]);
                        float4 iv6_w1 = WaveReadLaneAt(iv6_r, l2a);
                        float4 iv6_w2 = WaveReadLaneAt(iv6_r, l2b);
                        float4 iv6_w3 = WaveReadLaneAt(iv6_r, l2c);
                        if (isQuad)
                        {
                            imgDst6[uint3(int3(int2(workGroup), z))] = (((iv6_r + iv6_w1) + iv6_w2) + iv6_w3) * 0.25f;
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

            // ---- mip6 4x4 box average of mip6 (block3) ----
            switch (0)
            {
                default:
                {
                    x4 = x * 4u;
                    int x4i = int(x4);
                    y4 = y * 4u;
                    int y4i = int(y4);
                    uint2 r0 = uint2(int2(x4i, y4i));
                    float4 e0 = imgDst6[uint3(int3(int2(r0), z))];
                    float4 e1 = imgDst6[uint3(int3(int2(r0 + uint2(0u, 1u)), z))];
                    float4 e2 = imgDst6[uint3(int3(int2(r0 + uint2(1u, 0u)), z))];
                    float4 e3 = imgDst6[uint3(int3(int2(r0 + uint2(1u, 1u)), z))];
                    float4 m7_00 = (((e0 + e1) + e2) + e3) * 0.25f;
                    imgDst[6][uint3(int3(xi2, yi2, z))] = m7_00;

                    int x4i2 = int(x4 + 2u);
                    int xi1 = int(x2 + 1u);
                    uint2 r1 = uint2(int2(x4i2, y4i));
                    float4 f0 = imgDst6[uint3(int3(int2(r1), z))];
                    float4 f1 = imgDst6[uint3(int3(int2(r1 + uint2(0u, 1u)), z))];
                    float4 f2 = imgDst6[uint3(int3(int2(r1 + uint2(1u, 0u)), z))];
                    float4 f3 = imgDst6[uint3(int3(int2(r1 + uint2(1u, 1u)), z))];
                    float4 m7_10 = (((f0 + f1) + f2) + f3) * 0.25f;
                    imgDst[6][uint3(int3(xi1, yi2, z))] = m7_10;

                    int y4i2 = int(y4 + 2u);
                    int yi1 = int(y2 + 1u);
                    uint2 r2 = uint2(int2(x4i, y4i2));
                    float4 g0 = imgDst6[uint3(int3(int2(r2), z))];
                    float4 g1 = imgDst6[uint3(int3(int2(r2 + uint2(0u, 1u)), z))];
                    float4 g2 = imgDst6[uint3(int3(int2(r2 + uint2(1u, 0u)), z))];
                    float4 g3 = imgDst6[uint3(int3(int2(r2 + uint2(1u, 1u)), z))];
                    float4 m7_01 = (((g0 + g1) + g2) + g3) * 0.25f;
                    imgDst[6][uint3(int3(xi2, yi1, z))] = m7_01;

                    uint2 r3 = uint2(int2(x4i2, y4i2));
                    float4 h0 = imgDst6[uint3(int3(int2(r3), z))];
                    float4 h1 = imgDst6[uint3(int3(int2(r3 + uint2(0u, 1u)), z))];
                    float4 h2 = imgDst6[uint3(int3(int2(r3 + uint2(1u, 0u)), z))];
                    float4 h3 = imgDst6[uint3(int3(int2(r3 + uint2(1u, 1u)), z))];
                    float4 m7_11 = (((h0 + h1) + h2) + h3) * 0.25f;
                    imgDst[6][uint3(int3(xi1, yi1, z))] = m7_11;

                    if (spdConstants.mips <= 7u)
                    {
                        break;
                    }
                    float4 m8 = (((m7_00 + m7_10) + m7_01) + m7_11) * 0.25f;
                    imgDst[7][uint3(int3(xi, yi, z))] = m8;
                    spdIntermediateR[x][y] = m8.x;
                    spdIntermediateG[x][y] = m8.y;
                    spdIntermediateB[x][y] = m8.z;
                    spdIntermediateA[x][y] = m8.w;
                    break;
                }
            }

            // ---- mips 8..11 (block3b) ----
            switch (0)
            {
                default:
                {
                    if (spdConstants.mips <= 8u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    float4 j0_r = float4(spdIntermediateR[x][y], spdIntermediateG[x][y], spdIntermediateB[x][y], spdIntermediateA[x][y]);
                    uint base3 = WaveGetLaneIndex() & 0xFFFFFFFCu;
                    uint l3a = base3 | 1u;
                    float4 j0_w1 = WaveReadLaneAt(j0_r, l3a);
                    uint l3b = base3 | 2u;
                    float4 j0_w2 = WaveReadLaneAt(j0_r, l3b);
                    uint l3c = base3 | 3u;
                    float4 j0_w3 = WaveReadLaneAt(j0_r, l3c);
                    float4 m9 = (((j0_r + j0_w1) + j0_w2) + j0_w3) * 0.25f;
                    bool isQuad3 = (gl_LocalInvocationIndex % 4u) == 0u;
                    if (isQuad3)
                    {
                        uint y2h3 = y / 2u;
                        imgDst[8][uint3(int3(int(x / 2u), int(y2h3), z))] = m9;
                        uint xx3 = x + (y2h3 % 2u);
                        spdIntermediateR[xx3][y] = m9.x;
                        spdIntermediateG[xx3][y] = m9.y;
                        spdIntermediateB[xx3][y] = m9.z;
                        spdIntermediateA[xx3][y] = m9.w;
                    }
                    if (spdConstants.mips <= 9u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    if (gl_LocalInvocationIndex < 64u)
                    {
                        uint xo9 = x2 + (y % 2u);
                        float4 j1_r = float4(spdIntermediateR[xo9][y2], spdIntermediateG[xo9][y2], spdIntermediateB[xo9][y2], spdIntermediateA[xo9][y2]);
                        float4 j1_w1 = WaveReadLaneAt(j1_r, l3a);
                        float4 j1_w2 = WaveReadLaneAt(j1_r, l3b);
                        float4 j1_w3 = WaveReadLaneAt(j1_r, l3c);
                        float4 m10 = (((j1_r + j1_w1) + j1_w2) + j1_w3) * 0.25f;
                        if (isQuad3)
                        {
                            uint y3h3 = y / 2u;
                            imgDst[9][uint3(int3(int(x / 2u), int(y3h3), z))] = m10;
                            uint xo9_ = x2 + y3h3;
                            spdIntermediateR[xo9_][y2] = m10.x;
                            spdIntermediateG[xo9_][y2] = m10.y;
                            spdIntermediateB[xo9_][y2] = m10.z;
                            spdIntermediateA[xo9_][y2] = m10.w;
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
                        float4 j2_r = float4(spdIntermediateR[i4b][y4], spdIntermediateG[i4b][y4], spdIntermediateB[i4b][y4], spdIntermediateA[i4b][y4]);
                        float4 j2_w1 = WaveReadLaneAt(j2_r, l3a);
                        float4 j2_w2 = WaveReadLaneAt(j2_r, l3b);
                        float4 j2_w3 = WaveReadLaneAt(j2_r, l3c);
                        float4 m11 = (((j2_r + j2_w1) + j2_w2) + j2_w3) * 0.25f;
                        if (isQuad3)
                        {
                            uint x2h3 = x / 2u;
                            imgDst[10][uint3(int3(int(x2h3), int(y / 2u), z))] = m11;
                            uint xo10 = x2h3 + y;
                            spdIntermediateR[xo10][0u] = m11.x;
                            spdIntermediateG[xo10][0u] = m11.y;
                            spdIntermediateB[xo10][0u] = m11.z;
                            spdIntermediateA[xo10][0u] = m11.w;
                        }
                    }
                    if (spdConstants.mips <= 11u)
                    {
                        break;
                    }
                    GroupMemoryBarrierWithGroupSync();
                    if (gl_LocalInvocationIndex < 4u)
                    {
                        float4 j3_r = float4(spdIntermediateR[gl_LocalInvocationIndex][0u], spdIntermediateG[gl_LocalInvocationIndex][0u], spdIntermediateB[gl_LocalInvocationIndex][0u], spdIntermediateA[gl_LocalInvocationIndex][0u]);
                        float4 j3_w1 = WaveReadLaneAt(j3_r, l3a);
                        float4 j3_w2 = WaveReadLaneAt(j3_r, l3b);
                        float4 j3_w3 = WaveReadLaneAt(j3_r, l3c);
                        if (isQuad3)
                        {
                            imgDst[11][uint3(int3(0, 0, z))] = (((j3_r + j3_w1) + j3_w2) + j3_w3) * 0.25f;
                        }
                    }
                    break;
                }
            }
            break;
        }
    }
}
