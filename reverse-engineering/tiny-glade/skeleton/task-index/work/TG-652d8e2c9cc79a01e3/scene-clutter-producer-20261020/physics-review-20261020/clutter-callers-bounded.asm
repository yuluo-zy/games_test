0x1407e3d0d f30f1085cc020000 movss xmm0, dword ptr [rbp + 0x2cc]
0x1407e3d15 f30f5c85c8020000 subss xmm0, dword ptr [rbp + 0x2c8]
0x1407e3d1d f30f108dc0020000 movss xmm1, dword ptr [rbp + 0x2c0]
0x1407e3d25 660f6e95c4020000 movd xmm2, dword ptr [rbp + 0x2c4]
0x1407e3d2d f30f118d70020000 movss dword ptr [rbp + 0x270], xmm1
0x1407e3d35 f30f118574020000 movss dword ptr [rbp + 0x274], xmm0
0x1407e3d3d 660f7e9578020000 movd dword ptr [rbp + 0x278], xmm2
0x1407e3d45 0f28c8 movaps xmm1, xmm0
0x1407e3d48 f30f590d40c91702 mulss xmm1, dword ptr [rip + 0x217c940]
0x1407e3d50 0f14c8 unpcklps xmm1, xmm0
0x1407e3d53 0f590d861b1402 mulps xmm1, xmmword ptr [rip + 0x2141b86]
0x1407e3d5a 0f138dc0010000 movlps qword ptr [rbp + 0x1c0], xmm1
0x1407e3d61 f30f118dc8010000 movss dword ptr [rbp + 0x1c8], xmm1
0x1407e3d69 488d4d50 lea rcx, [rbp + 0x50]
0x1407e3d6d 488d9570020000 lea rdx, [rbp + 0x270]
0x1407e3d74 4c8d85c0010000 lea r8, [rbp + 0x1c0]
0x1407e3d7b e870c20e00 call 0x1408cfff0
0x1407e3d80 f30f108dcc020000 movss xmm1, dword ptr [rbp + 0x2cc]
0x1407e3d88 f30f5c8dc8020000 subss xmm1, dword ptr [rbp + 0x2c8]
0x1407e3d90 0f2895c0020000 movaps xmm2, xmmword ptr [rbp + 0x2c0]
0x1407e3d97 f30f1085c4020000 movss xmm0, dword ptr [rbp + 0x2c4]
0x1407e3d9f f30f580541631702 addss xmm0, dword ptr [rip + 0x2176341]
0x1407e3da7 f30f5f05d51b1402 maxss xmm0, dword ptr [rip + 0x2141bd5]
0x1407e3daf 0f28d9 movaps xmm3, xmm1
0x1407e3db2 f30f591dd6c81702 mulss xmm3, dword ptr [rip + 0x217c8d6]
0x1407e3dba 0f14d9 unpcklps xmm3, xmm1
0x1407e3dbd 0f16ca movlhps xmm1, xmm2
0x1407e3dc0 0fc6cae2 shufps xmm1, xmm2, 0xe2
0x1407e3dc4 0f580d25342302 addps xmm1, xmmword ptr [rip + 0x2233425]
0x1407e3dcb 410f28d6 movaps xmm2, xmm14
0x1407e3dcf 0f5fd1 maxps xmm2, xmm1
0x1407e3dd2 0fc2c903 cmpunordps xmm1, xmm1
0x1407e3dd6 410f28e6 movaps xmm4, xmm14
0x1407e3dda 0f54e1 andps xmm4, xmm1
0x1407e3ddd 0f55ca andnps xmm1, xmm2
0x1407e3de0 0f56cc orps xmm1, xmm4
0x1407e3de3 0f138d70020000 movlps qword ptr [rbp + 0x270], xmm1
0x1407e3dea f30f118578020000 movss dword ptr [rbp + 0x278], xmm0
0x1407e3df2 0f591de71a1402 mulps xmm3, xmmword ptr [rip + 0x2141ae7]
0x1407e3df9 0f139dc0010000 movlps qword ptr [rbp + 0x1c0], xmm3
0x1407e3e00 f30f119dc8010000 movss dword ptr [rbp + 0x1c8], xmm3
0x1407e3e08 c6852703000001 mov byte ptr [rbp + 0x327], 1
0x1407e3e0f 488d4dc0 lea rcx, [rbp - 0x40]
0x1407e3e13 488d9570020000 lea rdx, [rbp + 0x270]
0x1407e3e1a 4c8d85c0010000 lea r8, [rbp + 0x1c0]
0x1407e3e21 e8cac10e00 call 0x1408cfff0
