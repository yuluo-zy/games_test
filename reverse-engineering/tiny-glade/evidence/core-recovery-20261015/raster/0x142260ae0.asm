0x142260ae0 push     r15
0x142260ae2 push     r14
0x142260ae4 push     r13
0x142260ae6 push     r12
0x142260ae8 push     rsi
0x142260ae9 push     rdi
0x142260aea push     rbp
0x142260aeb push     rbx
0x142260aec sub      rsp, 0xa8
0x142260af3 movaps   xmmword ptr [rsp + 0x90], xmm6
0x142260afb mov      rsi, r9
0x142260afe mov      r14, r8
0x142260b01 mov      rbx, rdx
0x142260b04 mov      r15, rcx
0x142260b07 mov      r12, qword ptr [rsp + 0x120]
0x142260b0f mov      rax, qword ptr [rsp + 0x118]
0x142260b17 mov      qword ptr [rsp + 0x58], rax
0x142260b1c mov      rbp, qword ptr [rsp + 0x110]
0x142260b24 mov      eax, dword ptr [r8 + 0x1c]
0x142260b28 mov      rcx, qword ptr [r8 + 0x10]
0x142260b2c mov      dword ptr [rcx], eax
0x142260b2e mov      rcx, qword ptr [r8]
0x142260b31 lea      r8, [rip + 0xdd32d8]
0x142260b38 mov      dl, 4
0x142260b3a call     0x140868cf0
0x142260b3f mov      rdi, rax
0x142260b42 mov      r13d, dword ptr [r15]
0x142260b45 lea      rcx, [r13 - 1]
0x142260b49 xor      eax, eax
0x142260b4b cmp      r13, 2
0x142260b4f cmovae   rax, rcx
0x142260b53 test     rax, rax
0x142260b56 je       0x1422610ad
0x142260b5c cmp      rax, 1
0x142260b60 jne      0x1422610fd
0x142260b66 mov      rcx, rdi
0x142260b69 call     0x140868f70
0x142260b6e movss    xmm0, dword ptr [rbx + 4]
0x142260b73 subss    xmm0, dword ptr [rax + 4]
0x142260b78 mulss    xmm0, xmm0
0x142260b7c movss    xmm1, dword ptr [rbx + 8]
0x142260b81 movss    xmm2, dword ptr [rbx]
0x142260b85 unpcklps xmm2, xmm1
0x142260b88 movss    xmm1, dword ptr [rax]
0x142260b8c movss    xmm3, dword ptr [rax + 8]
0x142260b91 unpcklps xmm1, xmm3
0x142260b94 subps    xmm2, xmm1
0x142260b97 mulps    xmm2, xmm2
0x142260b9a addss    xmm0, xmm2
0x142260b9e movshdup xmm1, xmm2
0x142260ba2 addss    xmm1, xmm0
0x142260ba6 xorps    xmm0, xmm0
0x142260ba9 sqrtss   xmm0, xmm1
0x142260bad ucomiss  xmm0, dword ptr [rip + 0x76c4ac]
0x142260bb4 jbe      0x1422612f8
0x142260bba mov      eax, dword ptr [rbx + 8]
0x142260bbd mov      dword ptr [rsp + 0x68], eax
0x142260bc1 mov      rax, qword ptr [rbx]
0x142260bc4 mov      qword ptr [rsp + 0x60], rax
0x142260bc9 lea      rdx, [rsp + 0x60]
0x142260bce mov      rcx, rdi
0x142260bd1 call     0x140868fb0
0x142260bd6 mov      rcx, rdi
0x142260bd9 call     0x140868e40
0x142260bde lea      r15, [rdi + 0x48]
0x142260be2 lea      r12, [rdi + 0x270]
0x142260be9 mov      rcx, r12
0x142260bec call     0x140960c20
0x142260bf1 mov      rcx, r15
0x142260bf4 call     0x140a97860
0x142260bf9 mov      r8, rdx
0x142260bfc movss    xmm3, dword ptr [rdi + 0x60]
0x142260c01 mov      rcx, r12
0x142260c04 mov      rdx, rax
0x142260c07 call     0x1425d9e90
0x142260c0c lea      r13, [rsi + 0x98]
0x142260c13 mov      rcx, r13
0x142260c16 mov      rdx, r12
0x142260c19 call     0x140960c30
0x142260c1e mov      rdx, rdi
0x142260c21 add      rdx, 0x70
0x142260c25 mov      rcx, r13
0x142260c28 call     0x140960c30