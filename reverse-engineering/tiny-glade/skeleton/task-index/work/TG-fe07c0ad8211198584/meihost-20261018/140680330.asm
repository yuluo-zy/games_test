0x140680330 sub rsp, 0x28
0x140680334 movss xmm1, dword ptr [rcx + 0x38]
0x140680339 movss xmm0, dword ptr [rcx + 0x3c]
0x14068033e cmp byte ptr [rcx + 0x1c], 1
0x140680342 jne 0x140680360
0x140680344 movaps xmm2, xmm0
0x140680347 maxss xmm2, dword ptr [rip + 0x22a55b5] ; 0x142925904 0000803f9a99193f00000000feffffff f32=1.0
0x14068034f movss xmm3, dword ptr [rdx + 0x14]
0x140680354 divss xmm3, xmm2
0x140680358 mulss xmm1, xmm3
0x14068035c mulss xmm0, xmm3
0x140680360 cmp byte ptr [rcx + 0x44], 0
0x140680364 je 0x14068036b
0x140680366 movss xmm1, dword ptr [rdx + 0x10]
0x14068036b movss xmm2, dword ptr [rdx + 8]
0x140680370 movss xmm3, dword ptr [rdx + 0xc]
0x140680375 movss xmm4, dword ptr [rip + 0x22a54f3] ; 0x142925870 0000003fc10ffc3e34a6c93900000042 f32=0.5
0x14068037d mulss xmm1, xmm4
0x140680381 mulss xmm0, xmm4
0x140680385 movaps xmm4, xmm2
0x140680388 subss xmm4, xmm1
0x14068038c movaps xmm5, xmm3
0x14068038f subss xmm5, xmm0
0x140680393 addss xmm1, xmm2
0x140680397 addss xmm0, xmm3
0x14068039b add rcx, 0x20
0x14068039f mov rax, qword ptr [rdx]
0x1406803a2 movd r9d, xmm4
0x1406803a7 movd r8d, xmm5
0x1406803ac shl r8, 0x20
0x1406803b0 or r9, r8
0x1406803b3 subss xmm1, xmm4
0x1406803b7 subss xmm0, xmm5
0x1406803bb movd r8d, xmm1
0x1406803c0 movd r10d, xmm0
0x1406803c5 shl r10, 0x20
0x1406803c9 or r8, r10
0x1406803cc movss xmm3, dword ptr [rdx + 0x18]
0x1406803d1 mov qword ptr [rsp + 0x20], rcx
0x1406803d6 mov rcx, rax
0x1406803d9 mov rdx, r9
0x1406803dc call 0x140b8f510
0x1406803e1 nop 
0x1406803e2 add rsp, 0x28
0x1406803e6 ret 