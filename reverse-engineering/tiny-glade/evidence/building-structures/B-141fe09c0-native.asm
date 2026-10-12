141fe09c0 push rsi
141fe09c1 sub rsp, 0x60
141fe09c5 movaps xmmword ptr [rsp + 0x50], xmm9
141fe09cb movaps xmmword ptr [rsp + 0x40], xmm8
141fe09d1 movaps xmmword ptr [rsp + 0x30], xmm7
141fe09d6 movaps xmmword ptr [rsp + 0x20], xmm6
141fe09db mov r10b, 2
141fe09de cmp byte ptr [rdx + 0x28], 0
141fe09e2 jne 0x141fe0b41
141fe09e8 mov rax, qword ptr [rsp + 0x90]
141fe09f0 cmp r8, rax
141fe09f3 jae 0x141fe0b64
141fe09f9 lea r11, [r8 + r8*2]
141fe09fd movss xmm0, dword ptr [r9 + r11*4 + 8]
141fe0a04 movaps xmm1, xmm0
141fe0a07 subss xmm1, dword ptr [rdx + 0x20]
141fe0a0c movaps xmm4, xmm1
141fe0a0f mulss xmm4, xmm1
141fe0a13 movsd xmm3, qword ptr [rdx + 0x18]
141fe0a18 movsd xmm2, qword ptr [r9 + r11*4]
141fe0a1e subps xmm2, xmm3
141fe0a21 movaps xmm3, xmm2
141fe0a24 mulps xmm3, xmm2
141fe0a27 movshdup xmm5, xmm3
141fe0a2b addss xmm5, xmm3
141fe0a2f addss xmm3, xmm4
141fe0a33 sqrtss xmm3, xmm3
141fe0a37 addss xmm5, xmm4
141fe0a3b xorps xmm4, xmm4
141fe0a3e sqrtss xmm4, xmm5
141fe0a42 movss xmm7, dword ptr [rip + 0x944eba]
141fe0a4a movaps xmm6, xmm7
141fe0a4d divss xmm6, xmm4
141fe0a51 mulss xmm1, xmm6
141fe0a55 mov esi, dword ptr [rdx]
141fe0a57 test sil, 1
141fe0a5b jne 0x141fe0a62
141fe0a5d movaps xmm4, xmm1
141fe0a60 jmp 0x141fe0a67
141fe0a62 movss xmm4, dword ptr [rdx + 0xc]
141fe0a67 movss xmm8, dword ptr [rdx + 0x24]
141fe0a6d movaps xmm5, xmm3
141fe0a70 addss xmm5, xmm8
141fe0a75 ucomiss xmm5, dword ptr [rip + 0xc3ed60]
141fe0a7c ja 0x141fe0b41
141fe0a82 mov rax, qword ptr [rsp + 0xa0]
141fe0a8a cmp r8, rax
141fe0a8d jae 0x141fe0b79
141fe0a93 movsldup xmm6, xmm6
141fe0a97 mulps xmm2, xmm6
141fe0a9a mov r10, qword ptr [rsp + 0x98]
141fe0aa2 movss xmm6, dword ptr [r9 + r11*4]
141fe0aa8 mov eax, dword ptr [r9 + r11*4 + 4]
141fe0aad movzx r9d, byte ptr [r10 + r8]
141fe0ab2 xorps xmm9, xmm9
141fe0ab6 ucomiss xmm7, xmm8
141fe0aba ja 0x141fe0b14
141fe0abc movd xmm7, esi
141fe0ac0 pshufd xmm7, xmm7, 0x50
141fe0ac5 pslld xmm7, 0x1f
141fe0aca movq xmm8, qword ptr [rdx + 4]
141fe0ad0 psrad xmm7, 0x1f
141fe0ad5 pand xmm8, xmm7
141fe0ada pandn xmm7, xmm2
141fe0ade por xmm7, xmm8
141fe0ae3 subps xmm7, xmm2
141fe0ae6 subss xmm4, xmm1
141fe0aea mulps xmm7, xmm7
141fe0aed movshdup xmm8, xmm7
141fe0af2 addss xmm8, xmm7
141fe0af7 mulss xmm4, xmm4
141fe0afb addss xmm4, xmm8
141fe0b00 sqrtss xmm4, xmm4
141fe0b04 addss xmm4, dword ptr [rip + 0x944d64]
141fe0b0c mulss xmm5, xmm4
141fe0b10 movaps xmm9, xmm5
141fe0b14 test r9b, 0xf
141fe0b18 setne r10b
141fe0b1c mov qword ptr [rcx], r8
141fe0b1f movss dword ptr [rcx + 8], xmm3
141fe0b24 movlpd qword ptr [rcx + 0xc], xmm2
141fe0b29 movss dword ptr [rcx + 0x14], xmm1
141fe0b2e movss dword ptr [rcx + 0x18], xmm9
141fe0b34 movss dword ptr [rcx + 0x1c], xmm6
141fe0b39 mov dword ptr [rcx + 0x20], eax
141fe0b3c movss dword ptr [rcx + 0x24], xmm0
141fe0b41 mov byte ptr [rcx + 0x28], r10b
141fe0b45 mov rax, rcx
141fe0b48 movaps xmm6, xmmword ptr [rsp + 0x20]
141fe0b4d movaps xmm7, xmmword ptr [rsp + 0x30]
141fe0b52 movaps xmm8, xmmword ptr [rsp + 0x40]
141fe0b58 movaps xmm9, xmmword ptr [rsp + 0x50]
141fe0b5e add rsp, 0x60
141fe0b62 pop rsi
141fe0b63 ret 
141fe0b64 lea r9, [rip + 0xfea7d5]
141fe0b6b mov rcx, r8
141fe0b6e mov rdx, rax
141fe0b71 mov r8, r9
141fe0b74 call 0x1428d9518
141fe0b79 lea r9, [rip + 0xfea7d8]
141fe0b80 mov rcx, r8
141fe0b83 mov rdx, rax
141fe0b86 mov r8, r9
141fe0b89 call 0x1428d9518
141fe0b8e int3 
