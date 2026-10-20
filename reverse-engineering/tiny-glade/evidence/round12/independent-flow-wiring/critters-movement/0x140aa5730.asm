140aa5730: push rsi
140aa5731: sub rsp, 0x50
140aa5735: movaps xmmword ptr [rsp + 0x40], xmm8
140aa573b: movaps xmmword ptr [rsp + 0x30], xmm7
140aa5740: movaps xmmword ptr [rsp + 0x20], xmm6
140aa5745: mov rsi, rcx
140aa5748: movsd xmm0, qword ptr [rcx + 0x50]
140aa574d: movaps xmm6, xmmword ptr [rip + 0x1e8018c]
140aa5754: mulps xmm6, xmm0
140aa5757: unpcklps xmm1, xmm2
140aa575a: addps xmm6, xmm1
140aa575d: movsd xmm8, qword ptr [rcx + 0x48]
140aa5763: cvtdq2ps xmm1, xmm8
140aa5767: divps xmm0, xmm1
140aa576a: divps xmm6, xmm0
140aa576d: movshdup xmm0, xmm6
140aa5771: call 0x142923ee0 ; 
140aa5776: movaps xmm7, xmm0
140aa5779: movaps xmm0, xmm6
140aa577c: call 0x142923ee0 ; 
140aa5781: cvttss2si ecx, xmm0
140aa5785: movss xmm1, dword ptr [rip + 0x1eff22f]
140aa578d: ucomiss xmm0, xmm1
140aa5790: mov eax, 0x7fffffff
140aa5795: cmova ecx, eax
140aa5798: xor edx, edx
140aa579a: ucomiss xmm0, xmm0
140aa579d: cmovp ecx, edx
140aa57a0: cvttss2si r8d, xmm7
140aa57a5: ucomiss xmm7, xmm1
140aa57a8: cmova r8d, eax
140aa57ac: ucomiss xmm7, xmm7
140aa57af: cmovp r8d, edx
140aa57b3: mov eax, 8
140aa57b8: test ecx, ecx
140aa57ba: js 0x140aa582b
140aa57bc: movd xmm0, ecx
140aa57c0: movd xmm1, r8d
140aa57c5: punpckldq xmm0, xmm1
140aa57c9: movq xmm0, xmm0
140aa57cd: movaps xmm1, xmm8
140aa57d1: pcmpgtd xmm1, xmm0
140aa57d5: pshufd xmm1, xmm1, 0x50
140aa57da: movmskpd r8d, xmm1
140aa57df: xor edx, edx
140aa57e1: test r8b, 2
140aa57e5: je 0x140aa582b
140aa57e7: test r8b, 1
140aa57eb: je 0x140aa582b
140aa57ed: pshufd xmm0, xmm0, 0x55
140aa57f2: movd r8d, xmm0
140aa57f7: test r8d, r8d
140aa57fa: js 0x140aa582b
140aa57fc: movd edx, xmm8
140aa5801: imul r8d, edx
140aa5805: add r8d, ecx
140aa5808: movsxd rcx, r8d
140aa580b: cmp qword ptr [rsi + 0x10], rcx
140aa580f: jbe 0x140aa5829
140aa5811: mov rdx, qword ptr [rsi + 8]
140aa5815: lea rcx, [rcx + rcx*2]
140aa5819: shl rcx, 4
140aa581d: mov rax, qword ptr [rdx + rcx + 8]
140aa5822: mov rdx, qword ptr [rdx + rcx + 0x10]
140aa5827: jmp 0x140aa582b
140aa5829: xor edx, edx
140aa582b: movaps xmm6, xmmword ptr [rsp + 0x20]
140aa5830: movaps xmm7, xmmword ptr [rsp + 0x30]
140aa5835: movaps xmm8, xmmword ptr [rsp + 0x40]
140aa583b: add rsp, 0x50
140aa583f: pop rsi
140aa5840: ret 
