0x140c91b80 4883ec18 sub rsp, 0x18
0x140c91b84 0f293424 movaps xmmword ptr [rsp], xmm6
0x140c91b88 4889c8 mov rax, rcx
0x140c91b8b 410f284010 movaps xmm0, xmmword ptr [r8 + 0x10]
0x140c91b90 0f28c8 movaps xmm1, xmm0
0x140c91b93 0f59c8 mulps xmm1, xmm0
0x140c91b96 f30f16d1 movshdup xmm2, xmm1
0x140c91b9a f30f58d1 addss xmm2, xmm1
0x140c91b9e 0f28d9 movaps xmm3, xmm1
0x140c91ba1 660f15d9 unpckhpd xmm3, xmm1
0x140c91ba5 f30f58da addss xmm3, xmm2
0x140c91ba9 0fc6db00 shufps xmm3, xmm3, 0
0x140c91bad 0fc6c9ff shufps xmm1, xmm1, 0xff
0x140c91bb1 0f5ccb subps xmm1, xmm3
0x140c91bb4 0f281a movaps xmm3, xmmword ptr [rdx]
0x140c91bb7 0f59cb mulps xmm1, xmm3
0x140c91bba 0f28d0 movaps xmm2, xmm0
0x140c91bbd 0f59d3 mulps xmm2, xmm3
0x140c91bc0 f30f16e2 movshdup xmm4, xmm2
0x140c91bc4 f30f58e2 addss xmm4, xmm2
0x140c91bc8 0f12d2 movhlps xmm2, xmm2
0x140c91bcb f30f58d4 addss xmm2, xmm4
0x140c91bcf f30f58d2 addss xmm2, xmm2
0x140c91bd3 0fc6d200 shufps xmm2, xmm2, 0
0x140c91bd7 0f59d0 mulps xmm2, xmm0
0x140c91bda 0f58d1 addps xmm2, xmm1
0x140c91bdd 0f28e0 movaps xmm4, xmm0
0x140c91be0 0fc6e0d2 shufps xmm4, xmm0, 0xd2
0x140c91be4 0f59e3 mulps xmm4, xmm3
0x140c91be7 0fc6dbd2 shufps xmm3, xmm3, 0xd2
0x140c91beb 0f59d8 mulps xmm3, xmm0
0x140c91bee 0f5ce3 subps xmm4, xmm3
0x140c91bf1 0fc6e452 shufps xmm4, xmm4, 0x52
0x140c91bf5 0f28c8 movaps xmm1, xmm0
0x140c91bf8 0f58c8 addps xmm1, xmm0
0x140c91bfb 0fc6c9ff shufps xmm1, xmm1, 0xff
0x140c91bff 0f59cc mulps xmm1, xmm4
0x140c91c02 0f58ca addps xmm1, xmm2
0x140c91c05 410f5808 addps xmm1, xmmword ptr [r8]
0x140c91c09 0f285210 movaps xmm2, xmmword ptr [rdx + 0x10]
0x140c91c0d 0f28d8 movaps xmm3, xmm0
0x140c91c10 0fc6d800 shufps xmm3, xmm0, 0
0x140c91c14 0f28e0 movaps xmm4, xmm0
0x140c91c17 0fc6e055 shufps xmm4, xmm0, 0x55
0x140c91c1b 0f28e8 movaps xmm5, xmm0
0x140c91c1e 0fc6e8aa shufps xmm5, xmm0, 0xaa
0x140c91c22 0fc6c0ff shufps xmm0, xmm0, 0xff
0x140c91c26 0f59c2 mulps xmm0, xmm2
0x140c91c29 0f28f2 movaps xmm6, xmm2
0x140c91c2c 0fc6f21b shufps xmm6, xmm2, 0x1b
0x140c91c30 0f59de mulps xmm3, xmm6
0x140c91c33 0fc6f6b1 shufps xmm6, xmm6, 0xb1
0x140c91c37 0f59f4 mulps xmm6, xmm4
0x140c91c3a 0f591d2fb5d301 mulps xmm3, xmmword ptr [rip + 0x1d3b52f]
0x140c91c41 0f58d8 addps xmm3, xmm0
0x140c91c44 0fc6d2b1 shufps xmm2, xmm2, 0xb1
0x140c91c48 0f593531b5d301 mulps xmm6, xmmword ptr [rip + 0x1d3b531]
0x140c91c4f 0f59d5 mulps xmm2, xmm5
0x140c91c52 0f591537b5d301 mulps xmm2, xmmword ptr [rip + 0x1d3b537]
0x140c91c59 0f58d6 addps xmm2, xmm6
0x140c91c5c 0f58d3 addps xmm2, xmm3
0x140c91c5f 0f2909 movaps xmmword ptr [rcx], xmm1
0x140c91c62 0f295110 movaps xmmword ptr [rcx + 0x10], xmm2
0x140c91c66 0f283424 movaps xmm6, xmmword ptr [rsp]
0x140c91c6a 4883c418 add rsp, 0x18
0x140c91c6e c3 ret 
