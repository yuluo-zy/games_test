0x140b17a80 push     r15
0x140b17a82 push     r14
0x140b17a84 push     r13
0x140b17a86 push     r12
0x140b17a88 push     rsi
0x140b17a89 push     rdi
0x140b17a8a push     rbp
0x140b17a8b push     rbx
0x140b17a8c sub      rsp, 0x48
0x140b17a90 movaps   xmmword ptr [rsp + 0x30], xmm7
0x140b17a95 movaps   xmmword ptr [rsp + 0x20], xmm6
0x140b17a9a mov      rsi, r9
0x140b17a9d mov      rax, qword ptr [rsp + 0xb8]
0x140b17aa5 mov      r9, qword ptr [rsp + 0xb0]
0x140b17aad mov      rdi, qword ptr [rcx]
0x140b17ab0 movsd    xmm6, qword ptr [rdi]
0x140b17ab4 mov      rcx, qword ptr [r9]
0x140b17ab7 mov      r9, qword ptr [r9 + 8]
0x140b17abb mov      r14, qword ptr [rcx]
0x140b17abe xor      r10d, r10d
0x140b17ac1 mov      r11, r14
0x140b17ac4 sub      r11, qword ptr [r9 + 0x18]
0x140b17ac8 cmovb    r11, r10
0x140b17acc sub      r14, qword ptr [r9 + 0x38]
0x140b17ad0 cmovb    r14, r10
0x140b17ad4 lea      rbx, [r11 + r11*2]
0x140b17ad8 shl      rbx, 3
0x140b17adc add      rbx, qword ptr [r9 + 8]
0x140b17ae0 mov      r15, qword ptr [r9 + 0x10]
0x140b17ae4 sub      r15, r11
0x140b17ae7 cmovb    r15, r10
0x140b17aeb mov      r12, qword ptr [r8]
0x140b17aee mov      r8d, 8
0x140b17af4 cmovb    rbx, r8
0x140b17af8 lea      r11, [r14 + r14*2]
0x140b17afc shl      r11, 3
0x140b17b00 add      r11, qword ptr [r9 + 0x28]
0x140b17b04 mov      r13, qword ptr [r9 + 0x30]
0x140b17b08 sub      r13, r14
0x140b17b0b cmovb    r13, r10
0x140b17b0f cmovb    r11, r8
0x140b17b13 mov      r8, qword ptr [r9 + 0x40]
0x140b17b17 sub      r8, r13
0x140b17b1a sub      r8, r15
0x140b17b1d cmp      dword ptr [r12], 0
0x140b17b22 mov      qword ptr [rcx], r8
0x140b17b25 lea      r9, [r15 + r15*2]
0x140b17b29 lea      r9, [rbx + r9*8]
0x140b17b2d lea      r10, [r13 + r13*2]
0x140b17b32 lea      r10, [r11 + r10*8]
0x140b17b36 je       0x140b17bab
0x140b17b38 xorps    xmm7, xmm7
0x140b17b3b test     rbx, rbx
0x140b17b3e jne      0x140b17b6f
0x140b17b40 jmp      0x140b17b74
0x140b17b42 nop      word ptr cs:[rax + rax]
0x140b17b50 lea      r14, [rbx + 0x18]
0x140b17b54 mov      rdx, r11
0x140b17b57 mov      r15, rbx
0x140b17b5a inc      r8
0x140b17b5d mov      qword ptr [rcx], r8
0x140b17b60 cmp      dword ptr [r15 + 8], 1
0x140b17b65 mov      rbx, r14
0x140b17b68 jne      0x140b17b95
0x140b17b6a test     rbx, rbx
0x140b17b6d je       0x140b17b74
0x140b17b6f cmp      rbx, r9
0x140b17b72 jne      0x140b17b50
0x140b17b74 test     r11, r11
0x140b17b77 je       0x140b17c6a
0x140b17b7d cmp      r11, r10
0x140b17b80 je       0x140b17c6a
0x140b17b86 lea      rdx, [r11 + 0x18]
0x140b17b8a xor      r14d, r14d
0x140b17b8d mov      r15, r11
0x140b17b90 mov      r11, rdx
0x140b17b93 jmp      0x140b17b5a
0x140b17b95 movsd    xmm0, qword ptr [r15 + 0xc]
0x140b17b9b addps    xmm7, xmm0
0x140b17b9e mov      r11, rdx
0x140b17ba1 mov      rbx, r14
0x140b17ba4 test     rbx, rbx
0x140b17ba7 jne      0x140b17b6f
0x140b17ba9 jmp      0x140b17b74
0x140b17bab mov      rdx, qword ptr [rdx]
0x140b17bae xorpd    xmm0, xmm0
0x140b17bb2 movapd   xmm1, xmmword ptr [rip + 0x1ed19f6]
0x140b17bba xorps    xmm7, xmm7
0x140b17bbd jmp      0x140b17bd2
0x140b17bbf nop      
0x140b17bc0 movsd    xmm2, qword ptr [r15 + 0xc]
0x140b17bc6 movaps   xmm3, xmm2
0x140b17bc9 subps    xmm3, xmm6
0x140b17bcc addps    xmm7, xmm3
0x140b17bcf movaps   xmm6, xmm2
0x140b17bd2 mov      r14, r11
0x140b17bd5 mov      r12, rbx
0x140b17bd8 jmp      0x140b17bfd
0x140b17bda nop      word ptr [rax + rax]
0x140b17be0 lea      rbx, [r12 + 0x18]
0x140b17be5 mov      r11, r14
0x140b17be8 mov      r15, r12
0x140b17beb inc      r8
0x140b17bee mov      qword ptr [rcx], r8
0x140b17bf1 mov      ebp, dword ptr [r15 + 8]
0x140b17bf5 mov      r12, rbx
0x140b17bf8 cmp      ebp, 2
0x140b17bfb jne      0x140b17c20
0x140b17bfd test     r12, r12
0x140b17c00 je       0x140b17c07
0x140b17c02 cmp      r12, r9
0x140b17c05 jne      0x140b17be0
0x140b17c07 test     r14, r14
0x140b17c0a je       0x140b17c6a
0x140b17c0c cmp      r14, r10
0x140b17c0f je       0x140b17c6a
0x140b17c11 lea      r11, [r14 + 0x18]
0x140b17c15 xor      ebx, ebx
0x140b17c17 mov      r15, r14
0x140b17c1a mov      r14, r11
0x140b17c1d jmp      0x140b17beb
0x140b17c1f nop      
0x140b17c20 test     ebp, ebp
0x140b17c22 jne      0x140b17bc0
0x140b17c24 movsd    xmm2, qword ptr [r15 + 0xc]
0x140b17c2a movsd    xmm3, qword ptr [rdx]
0x140b17c2e unpcklps xmm3, xmm0
0x140b17c31 orpd     xmm3, xmm1
0x140b17c35 subpd    xmm3, xmm1
0x140b17c39 cvtpd2ps xmm3, xmm3
0x140b17c3d addps    xmm6, xmm2
0x140b17c40 xorps    xmm4, xmm4
0x140b17c43 maxps    xmm4, xmm6
0x140b17c46 cmpordps xmm6, xmm6
0x140b17c4a andps    xmm6, xmm4
0x140b17c4d movapd   xmm4, xmm3
0x140b17c51 minps    xmm4, xmm6
0x140b17c54 cmpunordps xmm6, xmm6
0x140b17c58 andpd    xmm3, xmm6
0x140b17c5c andnps   xmm6, xmm4
0x140b17c5f orps     xmm6, xmm3
0x140b17c62 addps    xmm7, xmm2
0x140b17c65 jmp      0x140b17bd2
0x140b17c6a mov      ecx, dword ptr [rax + 0x1c]
0x140b17c6d mov      rdx, qword ptr [rax + 0x10]
0x140b17c71 mov      dword ptr [rdx], ecx
0x140b17c73 mov      rbx, qword ptr [rax]
0x140b17c76 mov      r14, qword ptr [rbx + 0x30]
0x140b17c7a mov      r15, qword ptr [rbx + 0x40]
0x140b17c7e cmp      r14, qword ptr [rbx + 0x20]
0x140b17c82 jne      0x140b17c94
0x140b17c84 lea      rcx, [rbx + 0x20]
0x140b17c88 lea      rdx, [rip + 0x21224b9]
0x140b17c8f call     0x140562630
0x140b17c94 mov      rax, qword ptr [rbx + 0x28]
0x140b17c98 mov      rcx, r14
0x140b17c9b shl      rcx, 4
0x140b17c9f mov      qword ptr [rax + rcx], r15
0x140b17ca3 movlps   qword ptr [rax + rcx + 8], xmm7
0x140b17ca8 inc      r14
0x140b17cab mov      qword ptr [rbx + 0x30], r14
0x140b17caf inc      qword ptr [rbx + 0x40]
0x140b17cb3 mov      eax, dword ptr [rsi + 0x1c]
0x140b17cb6 mov      rcx, qword ptr [rsi + 0x10]
0x140b17cba mov      dword ptr [rcx], eax
0x140b17cbc mov      rdx, qword ptr [rsi]
0x140b17cbf movsd    xmm0, qword ptr [rdx + 0x58]
0x140b17cc4 addps    xmm0, xmm7
0x140b17cc7 movlps   qword ptr [rdx + 0x58], xmm0
0x140b17ccb mov      dword ptr [rcx], eax
0x140b17ccd movsd    xmm0, qword ptr [rdi]
0x140b17cd1 subps    xmm6, xmm0
0x140b17cd4 movsd    xmm0, qword ptr [rdx + 0x60]
0x140b17cd9 addps    xmm0, xmm6
0x140b17cdc movlps   qword ptr [rdx + 0x60], xmm0
0x140b17ce0 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x140b17ce5 movaps   xmm7, xmmword ptr [rsp + 0x30]
0x140b17cea add      rsp, 0x48
0x140b17cee pop      rbx
0x140b17cef pop      rbp
0x140b17cf0 pop      rdi
0x140b17cf1 pop      rsi
0x140b17cf2 pop      r12
0x140b17cf4 pop      r13
0x140b17cf6 pop      r14
0x140b17cf8 pop      r15
0x140b17cfa ret      
0x140b17cfb int3     
0x140b17cfc int3     
0x140b17cfd int3     
0x140b17cfe int3     
0x140b17cff int3     
0x140b17d00 push     rbp
0x140b17d01 push     r15
0x140b17d03 push     r14
0x140b17d05 push     r12
0x140b17d07 push     rsi
0x140b17d08 push     rdi
0x140b17d09 push     rbx
0x140b17d0a sub      rsp, 0x2e0
0x140b17d11 lea      rbp, [rsp + 0x80]
0x140b17d19 mov      qword ptr [rbp + 0x258], 0xfffffffffffffffe
0x140b17d24 mov      rdi, rcx
0x140b17d27 mov      rsi, qword ptr [rdx]
0x140b17d2a mov      qword ptr [rbp + 0x230], 0
0x140b17d35 mov      qword ptr [rbp + 0x238], 8
0x140b17d40 mov      qword ptr [rbp + 0x240], 0
0x140b17d4b mov      rbx, qword ptr [rcx]
0x140b17d4e cmp      qword ptr [rbx + 0x30], 0
0x140b17d53 movabs   rax, 0x1f83c18c67995024
0x140b17d5d movabs   rcx, 0x1ddeca6f5ff4d997
0x140b17d67 je       0x140b17e07
0x140b17d6d mov      rdx, qword ptr [rbx + 0x18]
0x140b17d71 mov      r8, qword ptr [rbx + 0x20]
0x140b17d75 mov      r9, r8
0x140b17d78 and      r9, rax
0x140b17d7b xor      r10d, r10d
0x140b17d7e movdqa   xmm0, xmmword ptr [rip + 0x1e3889a]
0x140b17d86 pcmpeqd  xmm1, xmm1
0x140b17d8a movdqu   xmm2, xmmword ptr [rdx + r9]
0x140b17d90 movdqa   xmm3, xmm2
0x140b17d94 pcmpeqb  xmm3, xmm0
0x140b17d98 pmovmskb r11d, xmm3
0x140b17d9d test     r11d, r11d
0x140b17da0 jne      0x140b17dcd
0x140b17da2 pcmpeqb  xmm2, xmm1
0x140b17da6 pmovmskb r11d, xmm2
0x140b17dab test     r11d, r11d
0x140b17dae jne      0x140b17e07
0x140b17db0 add      r9, r10
0x140b17db3 add      r9, 0x10
0x140b17db7 add      r10, 0x10
0x140b17dbb and      r9, r8
0x140b17dbe jmp      0x140b17d8a
0x140b17dc0 lea      r14d, [r11 - 1]
0x140b17dc4 and      r14w, r11w
0x140b17dc8 mov      r11d, r14d
0x140b17dcb je       0x140b17da2
0x140b17dcd tzcnt    r14d, r11d
0x140b17dd2 add      r14, r9
0x140b17dd5 and      r14, r8
0x140b17dd8 neg      r14
0x140b17ddb lea      r14, [r14 + r14*2]
0x140b17ddf cmp      qword ptr [rdx + r14*8 - 0x18], rcx
0x140b17de4 jne      0x140b17dc0
0x140b17de6 cmp      qword ptr [rdx + r14*8 - 0x10], rax
0x140b17deb jne      0x140b17dc0
0x140b17ded lea      rax, [rdx + r14*8]
0x140b17df1 mov      r14, qword ptr [rax - 8]
0x140b17df5 mov      rbx, qword ptr [rsi + 0x10]
0x140b17df9 cmp      rbx, qword ptr [rsi]
0x140b17dfc je       0x140b18154
0x140b17e02 jmp      0x140b18163
0x140b17e07 movzx    edx, byte ptr [rbx + 0x60]
0x140b17e0b mov      rdx, qword ptr [rbx + 0x80]
0x140b17e12 mov      r9, qword ptr [rbx + 0x88]
0x140b17e19 mov      r10, r9
0x140b17e1c and      r10, rax
0x140b17e1f xor      r11d, r11d
0x140b17e22 movdqa   xmm0, xmmword ptr [rip + 0x1e387f6]
0x140b17e2a pcmpeqd  xmm1, xmm1