140c9a210: 4883ec58 sub rsp, 0x58
140c9a214: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
140c9a21a: 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140c9a21f: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
140c9a224: f30f106210 movss xmm4, dword ptr [rdx + 0x10]
140c9a229: f30f105a14 movss xmm3, dword ptr [rdx + 0x14]
140c9a22e: 0f28c4 movaps xmm0, xmm4
140c9a231: f30f59c3 mulss xmm0, xmm3
140c9a235: 0f57c9 xorps xmm1, xmm1
140c9a238: 0f2ec1 ucomiss xmm0, xmm1
140c9a23b: 0f86f4010000 jbe 0x140c9a435
140c9a241: 4983f803 cmp r8, 3
140c9a245: 0f8702020000 ja 0x140c9a44d
140c9a24b: f20f1032 movsd xmm6, qword ptr [rdx]
140c9a24f: f30f16c6 movshdup xmm0, xmm6
140c9a253: 0f570596b6c801 xorps xmm0, xmmword ptr [rip + 0x1c8b696]
140c9a25a: f20f106a08 movsd xmm5, qword ptr [rdx + 8]
140c9a25f: 0f28ce movaps xmm1, xmm6
140c9a262: 0f16c8 movlhps xmm1, xmm0
140c9a265: 0fc6f124 shufps xmm6, xmm1, 0x24
140c9a269: f30f1005ffb5c801 movss xmm0, dword ptr [rip + 0x1c8b5ff]
140c9a271: 0f28fc movaps xmm7, xmm4
140c9a274: f30f59f8 mulss xmm7, xmm0
140c9a278: f30f59c3 mulss xmm0, xmm3
140c9a27c: 0f28d7 movaps xmm2, xmm7
140c9a27f: 0fc6d000 shufps xmm2, xmm0, 0
140c9a283: 0f59d6 mulps xmm2, xmm6
140c9a286: 0f28ca movaps xmm1, xmm2
140c9a289: 660f15ca unpckhpd xmm1, xmm2
140c9a28d: 0f58ca addps xmm1, xmm2
140c9a290: 0f58cd addps xmm1, xmm5
140c9a293: f30f10154dfecb01 movss xmm2, dword ptr [rip + 0x1cbfe4d]
140c9a29b: f30f59e2 mulss xmm4, xmm2
140c9a29f: 440f28c4 movaps xmm8, xmm4
140c9a2a3: 440fc6c000 shufps xmm8, xmm0, 0
140c9a2a8: 440f59c6 mulps xmm8, xmm6
140c9a2ac: 410f28c0 movaps xmm0, xmm8
140c9a2b0: 66410f15c0 unpckhpd xmm0, xmm8
140c9a2b5: 410f58c0 addps xmm0, xmm8
140c9a2b9: 0f58c5 addps xmm0, xmm5
140c9a2bc: f30f59da mulss xmm3, xmm2
140c9a2c0: 0fc6e300 shufps xmm4, xmm3, 0
140c9a2c4: 0f59e6 mulps xmm4, xmm6
140c9a2c7: 0f28d4 movaps xmm2, xmm4
140c9a2ca: 660f15d4 unpckhpd xmm2, xmm4
140c9a2ce: 0f58d4 addps xmm2, xmm4
140c9a2d1: 0f58d5 addps xmm2, xmm5
140c9a2d4: 0fc6fb00 shufps xmm7, xmm3, 0
140c9a2d8: 0f59fe mulps xmm7, xmm6
140c9a2db: 0f28e7 movaps xmm4, xmm7
140c9a2de: 660f15e7 unpckhpd xmm4, xmm7
140c9a2e2: 0f58e7 addps xmm4, xmm7
140c9a2e5: 0f58e5 addps xmm4, xmm5
140c9a2e8: 488d05f147e901 lea rax, [rip + 0x1e947f1]
140c9a2ef: 4a631480 movsxd rdx, dword ptr [rax + r8*4]
140c9a2f3: 4801c2 add rdx, rax
140c9a2f6: ffe2 jmp rdx
140c9a2f8: 0f5ce2 subps xmm4, xmm2
140c9a2fb: 0f28cc movaps xmm1, xmm4
140c9a2fe: 0f59cc mulps xmm1, xmm4
140c9a301: f30f16d9 movshdup xmm3, xmm1
140c9a305: f30f58d9 addss xmm3, xmm1
140c9a309: 0f57c9 xorps xmm1, xmm1
140c9a30c: f30f51cb sqrtss xmm1, xmm3
140c9a310: f30f101decb5c801 movss xmm3, dword ptr [rip + 0x1c8b5ec]
140c9a318: f30f5ed9 divss xmm3, xmm1
140c9a31c: f30f12db movsldup xmm3, xmm3
140c9a320: 0f59dc mulps xmm3, xmm4
140c9a323: 0f5cc2 subps xmm0, xmm2
140c9a326: 0f28c8 movaps xmm1, xmm0
140c9a329: 0f59c8 mulps xmm1, xmm0
140c9a32c: f30f16e9 movshdup xmm5, xmm1
140c9a330: f30f58e9 addss xmm5, xmm1
140c9a334: e9c4000000 jmp 0x140c9a3fd
140c9a339: 0f5cc1 subps xmm0, xmm1
140c9a33c: 0f28d0 movaps xmm2, xmm0
140c9a33f: 0f59d0 mulps xmm2, xmm0
140c9a342: f30f16da movshdup xmm3, xmm2
140c9a346: f30f58da addss xmm3, xmm2
140c9a34a: 0f57d2 xorps xmm2, xmm2
140c9a34d: f30f51d3 sqrtss xmm2, xmm3
140c9a351: f30f101dabb5c801 movss xmm3, dword ptr [rip + 0x1c8b5ab]
140c9a359: f30f5eda divss xmm3, xmm2
140c9a35d: f30f12db movsldup xmm3, xmm3
140c9a361: 0f59d8 mulps xmm3, xmm0
140c9a364: 0f5ce1 subps xmm4, xmm1
140c9a367: 0f28c4 movaps xmm0, xmm4
140c9a36a: 0f59c4 mulps xmm0, xmm4
140c9a36d: f30f16e8 movshdup xmm5, xmm0
140c9a371: f30f58e8 addss xmm5, xmm0
140c9a375: 0f28c4 movaps xmm0, xmm4
140c9a378: e980000000 jmp 0x140c9a3fd
140c9a37d: 0f5ccc subps xmm1, xmm4
140c9a380: 0f28c1 movaps xmm0, xmm1
140c9a383: 0f59c1 mulps xmm0, xmm1
140c9a386: f30f16d8 movshdup xmm3, xmm0
140c9a38a: f30f58d8 addss xmm3, xmm0
140c9a38e: 0f57c0 xorps xmm0, xmm0
140c9a391: f30f51c3 sqrtss xmm0, xmm3
140c9a395: f30f101d67b5c801 movss xmm3, dword ptr [rip + 0x1c8b567]
140c9a39d: f30f5ed8 divss xmm3, xmm0
140c9a3a1: f30f12db movsldup xmm3, xmm3
140c9a3a5: 0f59d9 mulps xmm3, xmm1
140c9a3a8: 0f5cd4 subps xmm2, xmm4
140c9a3ab: 0f28c2 movaps xmm0, xmm2
140c9a3ae: 0f59c2 mulps xmm0, xmm2
140c9a3b1: f30f16e8 movshdup xmm5, xmm0
140c9a3b5: f30f58e8 addss xmm5, xmm0
140c9a3b9: 0f28c2 movaps xmm0, xmm2
140c9a3bc: eb3f jmp 0x140c9a3fd
140c9a3be: 0f5cd0 subps xmm2, xmm0
140c9a3c1: 0f28da movaps xmm3, xmm2
140c9a3c4: 0f59da mulps xmm3, xmm2
140c9a3c7: f30f16e3 movshdup xmm4, xmm3
140c9a3cb: f30f58e3 addss xmm4, xmm3
140c9a3cf: 0f57db xorps xmm3, xmm3
140c9a3d2: f30f51dc sqrtss xmm3, xmm4
140c9a3d6: f30f102526b5c801 movss xmm4, dword ptr [rip + 0x1c8b526]
140c9a3de: f30f5ee3 divss xmm4, xmm3
140c9a3e2: f30f12dc movsldup xmm3, xmm4
140c9a3e6: 0f59da mulps xmm3, xmm2
140c9a3e9: 0f5cc8 subps xmm1, xmm0
140c9a3ec: 0f28c1 movaps xmm0, xmm1
140c9a3ef: 0f59c1 mulps xmm0, xmm1
140c9a3f2: f30f16e8 movshdup xmm5, xmm0
140c9a3f6: f30f58e8 addss xmm5, xmm0
140c9a3fa: 0f28c1 movaps xmm0, xmm1
140c9a3fd: 0f57c9 xorps xmm1, xmm1
140c9a400: f30f51cd sqrtss xmm1, xmm5
140c9a404: f30f1015f8b4c801 movss xmm2, dword ptr [rip + 0x1c8b4f8]
140c9a40c: f30f5ed1 divss xmm2, xmm1
140c9a410: f30f12ca movsldup xmm1, xmm2
140c9a414: 0f59c8 mulps xmm1, xmm0
140c9a417: 0f16d9 movlhps xmm3, xmm1
140c9a41a: 0f1119 movups xmmword ptr [rcx], xmm3
140c9a41d: 4889c8 mov rax, rcx
140c9a420: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
140c9a425: 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140c9a42a: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140c9a430: 4883c458 add rsp, 0x58
140c9a434: c3 ret 
140c9a435: 488d0d3042e901 lea rcx, [rip + 0x1e94230]
140c9a43c: 4c8d058d42e901 lea r8, [rip + 0x1e9428d]
140c9a443: ba23000000 mov edx, 0x23
140c9a448: e8e3efc301 call 0x1428d9430
140c9a44d: 488d0dcc43e901 lea rcx, [rip + 0x1e943cc]
140c9a454: 4c8d052544e901 lea r8, [rip + 0x1e94425]
140c9a45b: ba28000000 mov edx, 0x28
140c9a460: e8cbefc301 call 0x1428d9430
140c9a465: cc int3 
