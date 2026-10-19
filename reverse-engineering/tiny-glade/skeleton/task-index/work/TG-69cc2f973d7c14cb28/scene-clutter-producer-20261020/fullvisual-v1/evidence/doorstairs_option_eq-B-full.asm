0x140825150 4885c9 test rcx, rcx
0x140825153 0f94c0 sete al
0x140825156 4885d2 test rdx, rdx
0x140825159 410f94c0 sete r8b
0x14082515d 4108c0 or r8b, al
0x140825160 4889c8 mov rax, rcx
0x140825163 4809d0 or rax, rdx
0x140825166 0f94c0 sete al
0x140825169 4180f801 cmp r8b, 1
0x14082516d 7476 je 0x1408251e5
0x14082516f f30f1001 movss xmm0, dword ptr [rcx]
0x140825173 31c0 xor eax, eax
0x140825175 0f2e02 ucomiss xmm0, dword ptr [rdx]
0x140825178 756b jne 0x1408251e5
0x14082517a 7a69 jp 0x1408251e5
0x14082517c f30f104104 movss xmm0, dword ptr [rcx + 4]
0x140825181 0f2e4204 ucomiss xmm0, dword ptr [rdx + 4]
0x140825185 755e jne 0x1408251e5
0x140825187 7a5c jp 0x1408251e5
0x140825189 f30f104110 movss xmm0, dword ptr [rcx + 0x10]
0x14082518e 0f2e4210 ucomiss xmm0, dword ptr [rdx + 0x10]
0x140825192 754f jne 0x1408251e3
0x140825194 7a4d jp 0x1408251e3
0x140825196 f30f104114 movss xmm0, dword ptr [rcx + 0x14]
0x14082519b 0f2e4214 ucomiss xmm0, dword ptr [rdx + 0x14]
0x14082519f 7542 jne 0x1408251e3
0x1408251a1 7a40 jp 0x1408251e3
0x1408251a3 f30f104118 movss xmm0, dword ptr [rcx + 0x18]
0x1408251a8 0f2e4218 ucomiss xmm0, dword ptr [rdx + 0x18]
0x1408251ac 7535 jne 0x1408251e3
0x1408251ae 7a33 jp 0x1408251e3
0x1408251b0 f30f10411c movss xmm0, dword ptr [rcx + 0x1c]
0x1408251b5 0f2e421c ucomiss xmm0, dword ptr [rdx + 0x1c]
0x1408251b9 7528 jne 0x1408251e3
0x1408251bb 7a26 jp 0x1408251e3
0x1408251bd f30f10410c movss xmm0, dword ptr [rcx + 0xc]
0x1408251c2 f30f104a0c movss xmm1, dword ptr [rdx + 0xc]
0x1408251c7 f30f105208 movss xmm2, dword ptr [rdx + 8]
0x1408251cc f30f105908 movss xmm3, dword ptr [rcx + 8]
0x1408251d1 0fc2da00 cmpeqps xmm3, xmm2
0x1408251d5 0fc2c800 cmpeqps xmm1, xmm0
0x1408251d9 0f54cb andps xmm1, xmm3
0x1408251dc 660f7ec8 movd eax, xmm1
0x1408251e0 2401 and al, 1
0x1408251e2 c3 ret 
0x1408251e3 31c0 xor eax, eax
0x1408251e5 2401 and al, 1
0x1408251e7 c3 ret 
