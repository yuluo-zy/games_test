140825090: 4885c9 test rcx, rcx
140825093: 0f94c0 sete al
140825096: 4885d2 test rdx, rdx
140825099: 410f94c0 sete r8b
14082509d: 4108c0 or r8b, al
1408250a0: 4889c8 mov rax, rcx
1408250a3: 4809d0 or rax, rdx
1408250a6: 0f94c0 sete al
1408250a9: 4180f801 cmp r8b, 1
1408250ad: 7501 jne 0x1408250b0
1408250af: c3 ret 
1408250b0: 488b4130 mov rax, qword ptr [rcx + 0x30]
1408250b4: 483b4230 cmp rax, qword ptr [rdx + 0x30]
1408250b8: 755a jne 0x140825114
1408250ba: 8b4140 mov eax, dword ptr [rcx + 0x40]
1408250bd: 3b4240 cmp eax, dword ptr [rdx + 0x40]
1408250c0: 7552 jne 0x140825114
1408250c2: f30f104138 movss xmm0, dword ptr [rcx + 0x38]
1408250c7: 31c0 xor eax, eax
1408250c9: 0f2e4238 ucomiss xmm0, dword ptr [rdx + 0x38]
1408250cd: 75e0 jne 0x1408250af
1408250cf: 7ade jp 0x1408250af
1408250d1: f30f10413c movss xmm0, dword ptr [rcx + 0x3c]
1408250d6: 0f2e423c ucomiss xmm0, dword ptr [rdx + 0x3c]
1408250da: 75d3 jne 0x1408250af
1408250dc: 7ad1 jp 0x1408250af
1408250de: f30f104110 movss xmm0, dword ptr [rcx + 0x10]
1408250e3: 0f2e4210 ucomiss xmm0, dword ptr [rdx + 0x10]
1408250e7: 752b jne 0x140825114
1408250e9: 7a29 jp 0x140825114
1408250eb: f30f104114 movss xmm0, dword ptr [rcx + 0x14]
1408250f0: 0f2e4214 ucomiss xmm0, dword ptr [rdx + 0x14]
1408250f4: 751e jne 0x140825114
1408250f6: 7a1c jp 0x140825114
1408250f8: f30f104118 movss xmm0, dword ptr [rcx + 0x18]
1408250fd: 0f2e4218 ucomiss xmm0, dword ptr [rdx + 0x18]
140825101: 7511 jne 0x140825114
140825103: 7a0f jp 0x140825114
140825105: 0f2802 movaps xmm0, xmmword ptr [rdx]
140825108: 0fc20100 cmpeqps xmm0, xmmword ptr [rcx]
14082510c: 0f50c0 movmskps eax, xmm0
14082510f: 83f80f cmp eax, 0xf
140825112: 7403 je 0x140825117
140825114: 31c0 xor eax, eax
140825116: c3 ret 
140825117: f30f10411c movss xmm0, dword ptr [rcx + 0x1c]
14082511c: 0f2e421c ucomiss xmm0, dword ptr [rdx + 0x1c]
140825120: 75f2 jne 0x140825114
140825122: 7af0 jp 0x140825114
140825124: f30f104120 movss xmm0, dword ptr [rcx + 0x20]
140825129: 0f2e4220 ucomiss xmm0, dword ptr [rdx + 0x20]
14082512d: 75e5 jne 0x140825114
14082512f: 7ae3 jp 0x140825114
140825131: f30f104224 movss xmm0, dword ptr [rdx + 0x24]
140825136: f30fc2412400 cmpeqss xmm0, dword ptr [rcx + 0x24]
14082513c: 660f7ec0 movd eax, xmm0
140825140: 83e001 and eax, 1
140825143: c3 ret 
