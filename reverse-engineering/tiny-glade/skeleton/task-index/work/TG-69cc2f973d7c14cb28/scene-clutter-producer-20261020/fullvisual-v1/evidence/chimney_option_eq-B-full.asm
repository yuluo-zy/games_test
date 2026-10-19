0x140824fe0 4885c9 test rcx, rcx
0x140824fe3 0f94c0 sete al
0x140824fe6 4885d2 test rdx, rdx
0x140824fe9 410f94c0 sete r8b
0x140824fed 4108c0 or r8b, al
0x140824ff0 4889c8 mov rax, rcx
0x140824ff3 4809d0 or rax, rdx
0x140824ff6 0f94c0 sete al
0x140824ff9 4180f801 cmp r8b, 1
0x140824ffd 7501 jne 0x140825000
0x140824fff c3 ret 
0x140825000 f30f104108 movss xmm0, dword ptr [rcx + 8]
0x140825005 31c0 xor eax, eax
0x140825007 0f2e4208 ucomiss xmm0, dword ptr [rdx + 8]
0x14082500b 75f2 jne 0x140824fff
0x14082500d 7af0 jp 0x140824fff
0x14082500f f30f10410c movss xmm0, dword ptr [rcx + 0xc]
0x140825014 0f2e420c ucomiss xmm0, dword ptr [rdx + 0xc]
0x140825018 75e5 jne 0x140824fff
0x14082501a 7ae3 jp 0x140824fff
0x14082501c f30f104110 movss xmm0, dword ptr [rcx + 0x10]
0x140825021 0f2e4210 ucomiss xmm0, dword ptr [rdx + 0x10]
0x140825025 7562 jne 0x140825089
0x140825027 7a60 jp 0x140825089
0x140825029 f30f104114 movss xmm0, dword ptr [rcx + 0x14]
0x14082502e 0f2e4214 ucomiss xmm0, dword ptr [rdx + 0x14]
0x140825032 7555 jne 0x140825089
0x140825034 7a53 jp 0x140825089
0x140825036 f30f104118 movss xmm0, dword ptr [rcx + 0x18]
0x14082503b 31c0 xor eax, eax
0x14082503d 0f2e4218 ucomiss xmm0, dword ptr [rdx + 0x18]
0x140825041 75bc jne 0x140824fff
0x140825043 7aba jp 0x140824fff
0x140825045 f30f10411c movss xmm0, dword ptr [rcx + 0x1c]
0x14082504a 0f2e421c ucomiss xmm0, dword ptr [rdx + 0x1c]
0x14082504e 75af jne 0x140824fff
0x140825050 7aad jp 0x140824fff
0x140825052 8b01 mov eax, dword ptr [rcx]
0x140825054 3b02 cmp eax, dword ptr [rdx]
0x140825056 7531 jne 0x140825089
0x140825058 85c0 test eax, eax
0x14082505a 7405 je 0x140825061
0x14082505c 83f801 cmp eax, 1
0x14082505f 740d je 0x14082506e
0x140825061 f30f104104 movss xmm0, dword ptr [rcx + 4]
0x140825066 0f2e4204 ucomiss xmm0, dword ptr [rdx + 4]
0x14082506a 751d jne 0x140825089
0x14082506c 7a1b jp 0x140825089
0x14082506e 8b4120 mov eax, dword ptr [rcx + 0x20]
0x140825071 3b4220 cmp eax, dword ptr [rdx + 0x20]
0x140825074 7513 jne 0x140825089
0x140825076 f30f104224 movss xmm0, dword ptr [rdx + 0x24]
0x14082507b f30fc2412400 cmpeqss xmm0, dword ptr [rcx + 0x24]
0x140825081 660f7ec0 movd eax, xmm0
0x140825085 83e001 and eax, 1
0x140825088 c3 ret 
0x140825089 31c0 xor eax, eax
0x14082508b c3 ret 
