140c9a140: 4883ec28 sub rsp, 0x28
140c9a144: 4883fa04 cmp rdx, 4
140c9a148: 0f839d000000 jae 0x140c9a1eb
140c9a14e: f30f10058effcb01 movss xmm0, dword ptr [rip + 0x1cbff8e]
140c9a156: 488d057349e901 lea rax, [rip + 0x1e94973]
140c9a15d: 48631490 movsxd rdx, dword ptr [rax + rdx*4]
140c9a161: 4801c2 add rdx, rax
140c9a164: ffe2 jmp rdx
140c9a166: 0f28c8 movaps xmm1, xmm0
140c9a169: eb27 jmp 0x140c9a192
140c9a16b: f30f100d91b7c801 movss xmm1, dword ptr [rip + 0x1c8b791]
140c9a173: 0f28c1 movaps xmm0, xmm1
140c9a176: eb1a jmp 0x140c9a192
140c9a178: f30f100d84b7c801 movss xmm1, dword ptr [rip + 0x1c8b784]
140c9a180: eb10 jmp 0x140c9a192
140c9a182: f30f10057ab7c801 movss xmm0, dword ptr [rip + 0x1c8b77a]
140c9a18a: f30f100d52ffcb01 movss xmm1, dword ptr [rip + 0x1cbff52]
140c9a192: f20f1011 movsd xmm2, qword ptr [rcx]
140c9a196: f30f16da movshdup xmm3, xmm2
140c9a19a: 0f571d4fb7c801 xorps xmm3, xmmword ptr [rip + 0x1c8b74f]
140c9a1a1: 0fc6c800 shufps xmm1, xmm0, 0
140c9a1a5: 0f28c2 movaps xmm0, xmm2
140c9a1a8: 0f16c3 movlhps xmm0, xmm3
140c9a1ab: 0fc6d024 shufps xmm2, xmm0, 0x24
140c9a1af: 0f59d1 mulps xmm2, xmm1
140c9a1b2: 0f28c2 movaps xmm0, xmm2
140c9a1b5: 660f15c2 unpckhpd xmm0, xmm2
140c9a1b9: 0f58c2 addps xmm0, xmm2
140c9a1bc: f30f16d0 movshdup xmm2, xmm0
140c9a1c0: 0f28c8 movaps xmm1, xmm0
140c9a1c3: 0f59c8 mulps xmm1, xmm0
140c9a1c6: f30f16d9 movshdup xmm3, xmm1
140c9a1ca: f30f58d9 addss xmm3, xmm1
140c9a1ce: f30f51db sqrtss xmm3, xmm3
140c9a1d2: f30f100d2ab7c801 movss xmm1, dword ptr [rip + 0x1c8b72a]
140c9a1da: f30f5ecb divss xmm1, xmm3
140c9a1de: f30f59c1 mulss xmm0, xmm1
140c9a1e2: f30f59ca mulss xmm1, xmm2
140c9a1e6: 4883c428 add rsp, 0x28
140c9a1ea: c3 ret 
140c9a1eb: 488d0d5646e901 lea rcx, [rip + 0x1e94656]
140c9a1f2: 4c8d056f46e901 lea r8, [rip + 0x1e9466f]
140c9a1f9: ba1f000000 mov edx, 0x1f
140c9a1fe: e82df2c301 call 0x1428d9430
140c9a203: cc int3 
