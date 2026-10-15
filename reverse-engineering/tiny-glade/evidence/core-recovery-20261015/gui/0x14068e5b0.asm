0x14068e5b0 push     rbp
0x14068e5b1 push     r14
0x14068e5b3 push     rsi
0x14068e5b4 push     rdi
0x14068e5b5 push     rbx
0x14068e5b6 sub      rsp, 0x1b0
0x14068e5bd lea      rbp, [rsp + 0x80]
0x14068e5c5 mov      qword ptr [rbp + 0x128], 0xfffffffffffffffe
0x14068e5d0 mov      eax, dword ptr [r9 + 0x1c]
0x14068e5d4 mov      rcx, qword ptr [r9 + 0x10]
0x14068e5d8 mov      dword ptr [rcx], eax
0x14068e5da mov      rax, qword ptr [r9]
0x14068e5dd mov      rdx, qword ptr [rax]
0x14068e5e0 mov      rcx, qword ptr [rax + 0x58]
0x14068e5e4 mov      qword ptr [rbp + 0xb0], rcx
0x14068e5eb movups   xmm0, xmmword ptr [rax + 0x48]
0x14068e5ef movaps   xmmword ptr [rbp + 0xa0], xmm0
0x14068e5f6 movups   xmm0, xmmword ptr [rax + 0x38]
0x14068e5fa movaps   xmmword ptr [rbp + 0x90], xmm0
0x14068e601 movups   xmm0, xmmword ptr [rax + 0x28]
0x14068e605 movaps   xmmword ptr [rbp + 0x80], xmm0
0x14068e60c movups   xmm0, xmmword ptr [rax + 0x18]
0x14068e610 movaps   xmmword ptr [rbp + 0x70], xmm0
0x14068e614 movups   xmm0, xmmword ptr [rax + 8]
0x14068e618 movaps   xmmword ptr [rbp + 0x60], xmm0
0x14068e61c mov      qword ptr [rax], 0xb
0x14068e623 cmp      rdx, 0xb
0x14068e627 je       0x14068e7bb
0x14068e62d mov      qword ptr [rbp + 0xb8], rdx
0x14068e634 movaps   xmm0, xmmword ptr [rbp + 0x60]
0x14068e638 movaps   xmm1, xmmword ptr [rbp + 0x70]
0x14068e63c movaps   xmm2, xmmword ptr [rbp + 0x80]
0x14068e643 movaps   xmm3, xmmword ptr [rbp + 0x90]
0x14068e64a movups   xmmword ptr [rbp + 0xc0], xmm0
0x14068e651 movups   xmmword ptr [rbp + 0xd0], xmm1
0x14068e658 movups   xmmword ptr [rbp + 0xe0], xmm2
0x14068e65f movups   xmmword ptr [rbp + 0xf0], xmm3
0x14068e666 movaps   xmm0, xmmword ptr [rbp + 0xa0]
0x14068e66d movups   xmmword ptr [rbp + 0x100], xmm0
0x14068e674 mov      rax, qword ptr [rbp + 0xb0]
0x14068e67b mov      qword ptr [rbp + 0x110], rax
0x14068e682 mov      qword ptr [rbp + 0x118], rdx
0x14068e689 cmp      rdx, 2
0x14068e68d jne      0x14068e6e0
0x14068e68f mov      rax, qword ptr [rbp + 0x188]
0x14068e696 mov      ecx, dword ptr [rax + 0x1c]
0x14068e699 mov      rdx, qword ptr [rax + 0x10]
0x14068e69d mov      dword ptr [rdx], ecx
0x14068e69f mov      rsi, qword ptr [rax]
0x14068e6a2 mov      rdi, qword ptr [rsi + 0x30]
0x14068e6a6 mov      rbx, qword ptr [rsi + 0x40]
0x14068e6aa cmp      rdi, qword ptr [rsi + 0x20]
0x14068e6ae jne      0x14068e6cd
0x14068e6b0 lea      rcx, [rsi + 0x20]
0x14068e6b4 mov      byte ptr [rbp + 0x127], 1
0x14068e6bb lea      rdx, [rip + 0x2538fe6]
0x14068e6c2 mov      r14, r8
0x14068e6c5 call     0x140562870
0x14068e6ca mov      r8, r14
0x14068e6cd mov      rax, qword ptr [rsi + 0x28]
0x14068e6d1 mov      qword ptr [rax + rdi*8], rbx
0x14068e6d5 inc      rdi
0x14068e6d8 mov      qword ptr [rsi + 0x30], rdi
0x14068e6dc inc      qword ptr [rsi + 0x40]
0x14068e6e0 mov      rax, qword ptr [rbp + 0x180]
0x14068e6e7 mov      rcx, qword ptr [rbp + 0x108]
0x14068e6ee mov      qword ptr [rbp + 0x50], rcx
0x14068e6f2 mov      rcx, qword ptr [rbp + 0x110]
0x14068e6f9 mov      qword ptr [rbp + 0x58], rcx
0x14068e6fd movups   xmm0, xmmword ptr [rbp + 0xf8]
0x14068e704 movaps   xmmword ptr [rbp + 0x40], xmm0
0x14068e708 movups   xmm0, xmmword ptr [rbp + 0xb8]
0x14068e70f movups   xmm1, xmmword ptr [rbp + 0xc8]
0x14068e716 movups   xmm2, xmmword ptr [rbp + 0xd8]
0x14068e71d movups   xmm3, xmmword ptr [rbp + 0xe8]
0x14068e724 movaps   xmmword ptr [rbp + 0x30], xmm3
0x14068e728 movaps   xmmword ptr [rbp + 0x20], xmm2
0x14068e72c movaps   xmmword ptr [rbp + 0x10], xmm1
0x14068e730 movaps   xmmword ptr [rbp], xmm0
0x14068e734 mov      ecx, dword ptr [rax + 0x1c]
0x14068e737 mov      rdx, qword ptr [rax + 0x10]
0x14068e73b mov      dword ptr [rdx], ecx
0x14068e73d mov      rcx, qword ptr [rax]
0x14068e740 mov      eax, dword ptr [r8 + 0x1c]
0x14068e744 mov      rdx, qword ptr [r8 + 0x10]
0x14068e748 mov      dword ptr [rdx], eax
0x14068e74a mov      rax, qword ptr [r8]
0x14068e74d movups   xmm0, xmmword ptr [rax + 0x50]
0x14068e751 movaps   xmmword ptr [rbp - 0x10], xmm0
0x14068e755 movups   xmm0, xmmword ptr [rax + 0x40]
0x14068e759 movaps   xmmword ptr [rbp - 0x20], xmm0
0x14068e75d movups   xmm0, xmmword ptr [rax]
0x14068e760 movups   xmm1, xmmword ptr [rax + 0x10]
0x14068e764 movups   xmm2, xmmword ptr [rax + 0x20]
0x14068e768 movups   xmm3, xmmword ptr [rax + 0x30]
0x14068e76c movaps   xmmword ptr [rbp - 0x30], xmm3
0x14068e770 movaps   xmmword ptr [rbp - 0x40], xmm2
0x14068e774 movaps   xmmword ptr [rbp - 0x50], xmm1
0x14068e778 movaps   xmmword ptr [rbp - 0x60], xmm0
0x14068e77c movaps   xmm0, xmmword ptr [rbp + 0x50]
0x14068e780 movups   xmmword ptr [rax + 0x50], xmm0
0x14068e784 movaps   xmm0, xmmword ptr [rbp + 0x40]
0x14068e788 movups   xmmword ptr [rax + 0x40], xmm0
0x14068e78c movaps   xmm0, xmmword ptr [rbp]
0x14068e790 movaps   xmm1, xmmword ptr [rbp + 0x10]
0x14068e794 movaps   xmm2, xmmword ptr [rbp + 0x20]
0x14068e798 movaps   xmm3, xmmword ptr [rbp + 0x30]
0x14068e79c movups   xmmword ptr [rax + 0x30], xmm3
0x14068e7a0 movups   xmmword ptr [rax + 0x20], xmm2
0x14068e7a4 movups   xmmword ptr [rax + 0x10], xmm1
0x14068e7a8 movups   xmmword ptr [rax], xmm0
0x14068e7ab mov      byte ptr [rbp + 0x127], 0
0x14068e7b2 lea      rdx, [rbp - 0x60]
0x14068e7b6 call     0x14093fad0
0x14068e7bb nop      
0x14068e7bc add      rsp, 0x1b0
0x14068e7c3 pop      rbx
0x14068e7c4 pop      rdi
0x14068e7c5 pop      rsi
0x14068e7c6 pop      r14
0x14068e7c8 pop      rbp
0x14068e7c9 ret      