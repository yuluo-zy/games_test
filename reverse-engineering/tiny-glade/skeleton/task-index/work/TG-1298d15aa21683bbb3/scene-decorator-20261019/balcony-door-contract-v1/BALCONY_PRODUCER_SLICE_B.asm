1412aa71d: 80bf4f01000000 cmp byte ptr [rdi + 0x14f], 0
1412aa724: 0f855b030000 jne 0x1412aaa85
1412aaa78: 80bf4f01000000 cmp byte ptr [rdi + 0x14f], 0
1412aaa7f: 0f84a5fcffff je 0x1412aa72a
1412aaa85: 410f28c8 movaps xmm1, xmm8
1412aaa89: 410f59c8 mulps xmm1, xmm8
1412aaa8d: f30f16c1 movshdup xmm0, xmm1
1412aaa91: f30f58c1 addss xmm0, xmm1
1412aaa95: 0f28d1 movaps xmm2, xmm1
1412aaa98: 660f15d1 unpckhpd xmm2, xmm1
1412aaa9c: f30f58d0 addss xmm2, xmm0
1412aaaa0: 0fc6d200 shufps xmm2, xmm2, 0
1412aaaa4: 0fc6c9ff shufps xmm1, xmm1, 0xff
1412aaaa8: 0f5cca subps xmm1, xmm2
1412aaaab: 0f28050e267201 movaps xmm0, xmmword ptr [rip + 0x172260e]
1412aaab2: 0f59c8 mulps xmm1, xmm0
1412aaab5: 410f59c0 mulps xmm0, xmm8
1412aaab9: f30f16d0 movshdup xmm2, xmm0
1412aaabd: f30f58d0 addss xmm2, xmm0
1412aaac1: 0f12c0 movhlps xmm0, xmm0
1412aaac4: f30f58c2 addss xmm0, xmm2
1412aaac8: f30f58c0 addss xmm0, xmm0
1412aaacc: 0fc6c000 shufps xmm0, xmm0, 0
1412aaad0: 410f59c0 mulps xmm0, xmm8
1412aaad4: 0f58c1 addps xmm0, xmm1
1412aaad7: f30f100d25ae6701 movss xmm1, dword ptr [rip + 0x167ae25]
1412aaadf: 410f28d0 movaps xmm2, xmm8
1412aaae3: 0f59d1 mulps xmm2, xmm1
1412aaae6: 410f28d8 movaps xmm3, xmm8
1412aaaea: 410fc6d8c9 shufps xmm3, xmm8, 0xc9
1412aaaef: 0f59d9 mulps xmm3, xmm1
1412aaaf2: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
1412aaaf6: 0f5cda subps xmm3, xmm2
1412aaaf9: 450f28c8 movaps xmm9, xmm8
1412aaafd: 450f58c8 addps xmm9, xmm8
1412aab01: 450fc6c9ff shufps xmm9, xmm9, 0xff
1412aab06: 440f59cb mulps xmm9, xmm3
1412aab0a: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412aab0f: 4885db test rbx, rbx
1412aab12: 7505 jne 0x1412aab19
1412aab14: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412aab19: 440f58c8 addps xmm9, xmm0
1412aab1d: 4c8b7c2460 mov r15, qword ptr [rsp + 0x60]
1412aab22: 488b03 mov rax, qword ptr [rbx]
1412aab25: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412aab29: 4c29f0 sub rax, r14
1412aab2c: 4883f837 cmp rax, 0x37
1412aab30: 0f8632050000 jbe 0x1412ab068
1412aab36: 488b4308 mov rax, qword ptr [rbx + 8]
1412aab3a: 488d0daf07fcff lea rcx, [rip - 0x3f851]
1412aab41: 4a890c30 mov qword ptr [rax + r14], rcx
1412aab45: 488d0d2475fdff lea rcx, [rip - 0x28adc]
1412aab4c: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412aab51: 4e897c3010 mov qword ptr [rax + r14 + 0x10], r15
1412aab56: 420f137c3018 movlps qword ptr [rax + r14 + 0x18], xmm7
1412aab5c: f3420f11743020 movss dword ptr [rax + r14 + 0x20], xmm6
1412aab63: 460f134c3024 movlps qword ptr [rax + r14 + 0x24], xmm9
1412aab69: 450f12c9 movhlps xmm9, xmm9
1412aab6d: f3460f114c302c movss dword ptr [rax + r14 + 0x2c], xmm9
1412aab74: 42c644303000 mov byte ptr [rax + r14 + 0x30], 0
1412aab7a: 4983c638 add r14, 0x38
1412aab7e: 4c897310 mov qword ptr [rbx + 0x10], r14
