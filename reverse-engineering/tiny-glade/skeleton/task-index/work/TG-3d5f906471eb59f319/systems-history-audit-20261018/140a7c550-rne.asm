0x140a7c550 push     r15
0x140a7c552 push     r14
0x140a7c554 push     r13
0x140a7c556 push     r12
0x140a7c558 push     rsi
0x140a7c559 push     rdi
0x140a7c55a push     rbp
0x140a7c55b push     rbx
0x140a7c55c sub      rsp, 0xd8
0x140a7c563 movaps   xmmword ptr [rsp + 0xc0], xmm6
0x140a7c56b mov      rsi, r8
0x140a7c56e mov      rdi, rdx
0x140a7c571 mov      rax, qword ptr [r9]
0x140a7c574 mov      r8, qword ptr [r9 + 8]
0x140a7c578 mov      r9, qword ptr [rax]
0x140a7c57b xor      edx, edx
0x140a7c57d mov      r10, r9
0x140a7c580 sub      r10, qword ptr [r8 + 0x18]
0x140a7c584 cmovb    r10, rdx
0x140a7c588 sub      r9, qword ptr [r8 + 0x38]
0x140a7c58c cmovb    r9, rdx
0x140a7c590 lea      r11, [r10 + r10*4]
0x140a7c594 shl      r11, 3
0x140a7c598 add      r11, qword ptr [r8 + 8]
0x140a7c59c lea      rbx, [r9 + r9*4]
0x140a7c5a0 shl      rbx, 3
0x140a7c5a4 add      rbx, qword ptr [r8 + 0x28]
0x140a7c5a8 mov      r14, qword ptr [r8 + 0x10]
0x140a7c5ac sub      r14, r10
0x140a7c5af cmovb    r14, rdx
0x140a7c5b3 mov      r10, qword ptr [r8 + 0x30]
0x140a7c5b7 mov      r15d, 8
0x140a7c5bd cmovb    r11, r15
0x140a7c5c1 lea      r12, [r14 + r14*4]
0x140a7c5c5 setbe    bpl
0x140a7c5c9 lea      r11, [r11 + r12*8 - 0x28]
0x140a7c5ce cmovbe   r11, rdx
0x140a7c5d2 sub      r10, r9
0x140a7c5d5 cmovb    r10, rdx
0x140a7c5d9 cmovb    rbx, r15
0x140a7c5dd mov      r9, qword ptr [r8 + 0x40]
0x140a7c5e1 lea      r8, [r10 + r10*4]
0x140a7c5e5 setbe    r15b
0x140a7c5e9 lea      r8, [rbx + r8*8 - 0x28]
0x140a7c5ee cmovbe   r8, r11
0x140a7c5f2 add      r10, r14
0x140a7c5f5 test     bpl, r15b
0x140a7c5f8 cmove    r10, rdx
0x140a7c5fc sub      r9, r10
0x140a7c5ff mov      rbx, qword ptr [rsp + 0x140]
0x140a7c607 mov      qword ptr [rax], r9
0x140a7c60a test     bpl, r15b
0x140a7c60d jne      0x140a7c6c9
0x140a7c613 cmp      dword ptr [r8], 0xd
0x140a7c617 jne      0x140a7c6c9
0x140a7c61d mov      rax, qword ptr [rsp + 0x150]
0x140a7c625 mov      r10d, dword ptr [r8 + 0x10]
0x140a7c629 lea      edx, [r10 - 6]
0x140a7c62d mov      r11, r10
0x140a7c630 add      r11, -5
0x140a7c634 xor      r9d, r9d
0x140a7c637 cmp      edx, 0xf
0x140a7c63a cmovae   r11, r9
0x140a7c63e mov      rdx, qword ptr [rsp + 0x148]
0x140a7c646 lea      r14, [rip + 0x205b133]
0x140a7c64d movsxd   r11, dword ptr [r14 + r11*4]
0x140a7c651 add      r11, r14
0x140a7c654 jmp      r11
0x140a7c657 mov      r9d, dword ptr [r8 + 0x1c]
0x140a7c65b mov      dword ptr [rsp + 0x48], r9d
0x140a7c660 mov      r8, qword ptr [r8 + 0x14]
0x140a7c664 mov      qword ptr [rsp + 0x40], r8
0x140a7c669 mov      r9d, r10d
0x140a7c66c mov      r8d, dword ptr [rdx + 0x1c]
0x140a7c670 mov      r10, qword ptr [rdx + 0x10]
0x140a7c674 mov      dword ptr [r10], r8d
0x140a7c677 mov      rdx, qword ptr [rdx]
0x140a7c67a mov      dword ptr [rdx], r9d
0x140a7c67d mov      r8, qword ptr [rsp + 0x40]
0x140a7c682 mov      qword ptr [rdx + 4], r8
0x140a7c686 mov      r8d, dword ptr [rsp + 0x48]
0x140a7c68b mov      dword ptr [rdx + 0xc], r8d
0x140a7c68f mov      edx, dword ptr [rax + 0x1c]
0x140a7c692 mov      r8, qword ptr [rax + 0x10]
0x140a7c696 mov      dword ptr [r8], edx
0x140a7c699 mov      rax, qword ptr [rax]
0x140a7c69c mov      qword ptr [rax], 1
0x140a7c6a3 mov      eax, dword ptr [rsi + 0x1c]
0x140a7c6a6 mov      rdx, qword ptr [rsi + 0x10]
0x140a7c6aa mov      dword ptr [rdx], eax
0x140a7c6ac mov      rax, qword ptr [rsi]
0x140a7c6af mov      dword ptr [rax + 0x28], 0x3f800000
0x140a7c6b6 mov      eax, dword ptr [rbx + 0x1c]
0x140a7c6b9 mov      rdx, qword ptr [rbx + 0x10]
0x140a7c6bd mov      dword ptr [rdx], eax
0x140a7c6bf mov      rax, qword ptr [rbx]
0x140a7c6c2 mov      qword ptr [rax], 0
0x140a7c6c9 mov      rax, qword ptr [rcx]
0x140a7c6cc cmp      dword ptr [rax], 0xd
0x140a7c6cf movzx    eax, byte ptr [rip + 0x2bb86b2]
0x140a7c6d6 jne      0x140a7c82b
0x140a7c6dc mov      r14, qword ptr [rsp + 0x178]
0x140a7c6e4 mov      r15, qword ptr [rsp + 0x170]
0x140a7c6ec test     al, 1
0x140a7c6ee jne      0x140a7c7ba
0x140a7c6f4 mov      rax, qword ptr [rip + 0x2bafcad]
0x140a7c6fb mov      rax, qword ptr [rax]
0x140a7c6fe cmp      rax, 3
0x140a7c702 jbe      0x140a7c7b3
0x140a7c708 lea      rcx, [rip + 0x205aa21]
0x140a7c70f call     0x14067dd80
0x140a7c714 movups   xmm0, xmmword ptr [rax]
0x140a7c717 mov      eax, dword ptr [rax + 0x10]
0x140a7c71a mov      qword ptr [rsp + 0x70], 4
0x140a7c723 lea      rcx, [rip + 0x205aa1e]
0x140a7c72a mov      qword ptr [rsp + 0x78], rcx
0x140a7c72f mov      qword ptr [rsp + 0x80], 0x2b
0x140a7c73b lea      rdx, [rip + 0x205a9de]
0x140a7c742 mov      qword ptr [rsp + 0x90], rdx
0x140a7c74a mov      qword ptr [rsp + 0x98], 1
0x140a7c756 mov      qword ptr [rsp + 0xa0], 8
0x140a7c762 mov      qword ptr [rsp + 0x40], 0
0x140a7c76b xorps    xmm1, xmm1
0x140a7c76e movups   xmmword ptr [rsp + 0xa8], xmm1
0x140a7c776 mov      qword ptr [rsp + 0x48], rcx
0x140a7c77b mov      qword ptr [rsp + 0x50], 0x2b
0x140a7c784 mov      qword ptr [rsp + 0x58], 0
0x140a7c78d movups   xmmword ptr [rsp + 0x60], xmm0
0x140a7c792 mov      dword ptr [rsp + 0x88], 1
0x140a7c79d mov      dword ptr [rsp + 0x8c], eax
0x140a7c7a4 lea      rcx, [rsp + 0x3f]
0x140a7c7a9 lea      rdx, [rsp + 0x40]
0x140a7c7ae call     0x140d4a330
0x140a7c7b3 mov      byte ptr [rip + 0x2bb85ce], 1
0x140a7c7ba mov      rcx, qword ptr [r15]
0x140a7c7bd mov      dword ptr [rsp + 0x40], 0
0x140a7c7c5 mov      dword ptr [rsp + 0x58], 0
0x140a7c7cd mov      dword ptr [rsp + 0x70], 0
0x140a7c7d5 mov      dword ptr [rsp + 0x88], 0
0x140a7c7e0 lea      rax, [rsp + 0x40]
0x140a7c7e5 mov      qword ptr [rsp + 0x28], rax
0x140a7c7ea mov      byte ptr [rsp + 0x20], 3
0x140a7c7ef lea      rdx, [rip + 0x205a97d]
0x140a7c7f6 mov      r8d, 6
0x140a7c7fc mov      r9b, 0xd
0x140a7c7ff call     0x14092df50
0x140a7c804 mov      rax, qword ptr [r14]
0x140a7c807 test     byte ptr [rax], 1
0x140a7c80a je       0x140a7c8fe
0x140a7c810 movss    xmm0, dword ptr [rax + 4]
0x140a7c815 mov      eax, dword ptr [rsi + 0x1c]
0x140a7c818 mov      rcx, qword ptr [rsi + 0x10]
0x140a7c81c mov      dword ptr [rcx], eax
0x140a7c81e mov      rax, qword ptr [rsi]
0x140a7c821 movss    dword ptr [rax + 0x28], xmm0
0x140a7c826 jmp      0x140a7cb64
0x140a7c82b test     al, 1
0x140a7c82d je       0x140a7cb64
0x140a7c833 mov      rax, qword ptr [rip + 0x2bafb6e]
0x140a7c83a mov      rax, qword ptr [rax]
0x140a7c83d cmp      rax, 3
0x140a7c841 jbe      0x140a7c9c3
0x140a7c847 lea      rcx, [rip + 0x205a952]
0x140a7c84e call     0x14067dd80
0x140a7c853 movups   xmm0, xmmword ptr [rax]
0x140a7c856 mov      eax, dword ptr [rax + 0x10]
0x140a7c859 mov      qword ptr [rsp + 0x70], 4
0x140a7c862 lea      rcx, [rip + 0x205a8df]
0x140a7c869 mov      qword ptr [rsp + 0x78], rcx
0x140a7c86e mov      qword ptr [rsp + 0x80], 0x2b
0x140a7c87a lea      rdx, [rip + 0x205a90f]
0x140a7c881 mov      qword ptr [rsp + 0x90], rdx
0x140a7c889 mov      qword ptr [rsp + 0x98], 1
0x140a7c895 mov      qword ptr [rsp + 0xa0], 8
0x140a7c8a1 mov      qword ptr [rsp + 0x40], 0
0x140a7c8aa xorps    xmm1, xmm1
0x140a7c8ad movups   xmmword ptr [rsp + 0xa8], xmm1
0x140a7c8b5 mov      qword ptr [rsp + 0x48], rcx
0x140a7c8ba mov      qword ptr [rsp + 0x50], 0x2b
0x140a7c8c3 mov      qword ptr [rsp + 0x58], 0
0x140a7c8cc movups   xmmword ptr [rsp + 0x60], xmm0
0x140a7c8d1 mov      dword ptr [rsp + 0x88], 1
0x140a7c8dc mov      dword ptr [rsp + 0x8c], eax
0x140a7c8e3 lea      rcx, [rsp + 0x3f]
0x140a7c8e8 lea      rdx, [rsp + 0x40]
0x140a7c8ed call     0x140d4a330
0x140a7c8f2 mov      byte ptr [rip + 0x2bb848f], 0
0x140a7c8f9 jmp      0x140a7cb64
0x140a7c8fe mov      r15, qword ptr [rsp + 0x168]
0x140a7c906 mov      rax, qword ptr [rsp + 0x160]
0x140a7c90e mov      r14, qword ptr [rbx]
0x140a7c911 movss    xmm6, dword ptr [r14 + 4]
0x140a7c917 mov      rcx, qword ptr [rax]
0x140a7c91a mov      edx, dword ptr [rcx + 0x80]
0x140a7c920 test     byte ptr [r14], 1
0x140a7c924 je       0x140a7c9cf
0x140a7c92a mov      rax, qword ptr [rsp + 0x158]
0x140a7c932 cmp      edx, 3
0x140a7c935 jae      0x140a7ca31
0x140a7c93b mov      ecx, dword ptr [rax + 0x1c]
0x140a7c93e mov      rdx, qword ptr [rax + 0x10]
0x140a7c942 mov      dword ptr [rdx], ecx
0x140a7c944 mov      rdi, qword ptr [rax]
0x140a7c947 mov      r12, qword ptr [rdi + 0x30]
0x140a7c94b mov      r13, qword ptr [rdi + 0x40]
0x140a7c94f mov      ebp, 0x3000000
0x140a7c954 cmp      r12, qword ptr [rdi + 0x20]
0x140a7c958 jne      0x140a7c96a
0x140a7c95a lea      rcx, [rdi + 0x20]
0x140a7c95e lea      rdx, [rip + 0x21bd7e3]
0x140a7c965 call     0x140562630
0x140a7c96a mov      rax, qword ptr [rdi + 0x28]
0x140a7c96e mov      rcx, r12
0x140a7c971 shl      rcx, 4
0x140a7c975 mov      qword ptr [rax + rcx], r13
0x140a7c979 mov      dword ptr [rax + rcx + 8], ebp
0x140a7c97d inc      r12
0x140a7c980 mov      qword ptr [rdi + 0x30], r12
0x140a7c984 inc      qword ptr [rdi + 0x40]
0x140a7c988 mov      rax, qword ptr [r15]
0x140a7c98b movss    xmm2, dword ptr [rax + 0x28]
0x140a7c990 movss    xmm1, dword ptr [rip + 0x1eb7cf8]
0x140a7c998 movss    xmm3, dword ptr [rip + 0x1f506c8]
0x140a7c9a0 movaps   xmm0, xmm6
0x140a7c9a3 call     0x140c904e0
0x140a7c9a8 mov      eax, dword ptr [rbx + 0x1c]
0x140a7c9ab mov      rcx, qword ptr [rbx + 0x10]
0x140a7c9af mov      dword ptr [rcx], eax
0x140a7c9b1 mov      dword ptr [r14], 1
0x140a7c9b8 movss    dword ptr [r14 + 4], xmm0
0x140a7c9be mov      rax, qword ptr [rsi]
0x140a7c9c1 jmp      0x140a7ca1e
0x140a7c9c3 mov      byte ptr [rip + 0x2bb83be], 0
0x140a7c9ca jmp      0x140a7cb64
0x140a7c9cf cmp      edx, 3
0x140a7c9d2 jb       0x140a7c9e0
0x140a7c9d4 xorps    xmm1, xmm1
0x140a7c9d7 cmp      dword ptr [rcx + 0x88], 2
0x140a7c9de ja       0x140a7c9eb
0x140a7c9e0 mov      rax, qword ptr [r15]
0x140a7c9e3 addss    xmm6, dword ptr [rax + 0x28]
0x140a7c9e8 movaps   xmm1, xmm6
0x140a7c9eb xor      eax, eax
0x140a7c9ed ucomiss  xmm1, dword ptr [rip + 0x1f50674]
0x140a7c9f4 seta     al
0x140a7c9f7 movss    xmm0, dword ptr [rip + 0x1ea8f05]
0x140a7c9ff movaps   xmm2, xmm0
0x140a7ca02 ja       0x140a7ca07
0x140a7ca04 movaps   xmm2, xmm1
0x140a7ca07 mov      ecx, dword ptr [rbx + 0x1c]
0x140a7ca0a mov      rdx, qword ptr [rbx + 0x10]
0x140a7ca0e mov      dword ptr [rdx], ecx
0x140a7ca10 mov      dword ptr [r14], eax
0x140a7ca13 movss    dword ptr [r14 + 4], xmm2
0x140a7ca19 mov      rax, qword ptr [rsi]
0x140a7ca1c jbe      0x140a7ca62
0x140a7ca1e mov      ecx, dword ptr [rsi + 0x1c]
0x140a7ca21 mov      rdx, qword ptr [rsi + 0x10]
0x140a7ca25 mov      dword ptr [rdx], ecx
0x140a7ca27 movss    dword ptr [rax + 0x28], xmm0
0x140a7ca2c jmp      0x140a7cb64
0x140a7ca31 cmp      dword ptr [rcx + 0x88], 3
0x140a7ca38 jae      0x140a7ca8b
0x140a7ca3a mov      ecx, dword ptr [rax + 0x1c]
0x140a7ca3d mov      rdx, qword ptr [rax + 0x10]
0x140a7ca41 mov      dword ptr [rdx], ecx
0x140a7ca43 mov      rdi, qword ptr [rax]
0x140a7ca46 mov      r12, qword ptr [rdi + 0x30]
0x140a7ca4a mov      r13, qword ptr [rdi + 0x40]
0x140a7ca4e mov      ebp, 0x3000100
0x140a7ca53 cmp      r12, qword ptr [rdi + 0x20]
0x140a7ca57 je       0x140a7c95a
0x140a7ca5d jmp      0x140a7c96a
0x140a7ca62 mov      ecx, dword ptr [rsi + 0x1c]
0x140a7ca65 mov      rdx, qword ptr [rsi + 0x10]
0x140a7ca69 jmp      0x140a7cafc
0x140a7ca6e mov      r8, qword ptr [rdx]
0x140a7ca71 mov      r9d, dword ptr [r8]
0x140a7ca74 mov      r10d, dword ptr [r8 + 0xc]
0x140a7ca78 mov      dword ptr [rsp + 0x48], r10d
0x140a7ca7d mov      r8, qword ptr [r8 + 4]
0x140a7ca81 mov      qword ptr [rsp + 0x40], r8
0x140a7ca86 jmp      0x140a7c66c
0x140a7ca8b mov      ecx, dword ptr [rax + 0x1c]
0x140a7ca8e mov      rdx, qword ptr [rax + 0x10]
0x140a7ca92 mov      dword ptr [rdx], ecx
0x140a7ca94 mov      r15, qword ptr [rax]
0x140a7ca97 mov      r12, qword ptr [r15 + 0x30]
0x140a7ca9b mov      r13, qword ptr [r15 + 0x40]
0x140a7ca9f cmp      r12, qword ptr [r15 + 0x20]
0x140a7caa3 jne      0x140a7cab5
0x140a7caa5 lea      rcx, [r15 + 0x20]
0x140a7caa9 lea      rdx, [rip + 0x21bd698]
0x140a7cab0 call     0x140562630
0x140a7cab5 mov      rax, qword ptr [r15 + 0x28]
0x140a7cab9 mov      rcx, r12
0x140a7cabc shl      rcx, 4
0x140a7cac0 mov      qword ptr [rax + rcx], r13
0x140a7cac4 mov      dword ptr [rax + rcx + 8], 0x3000200
0x140a7cacc inc      r12
0x140a7cacf mov      qword ptr [r15 + 0x30], r12
0x140a7cad3 inc      qword ptr [r15 + 0x40]
0x140a7cad7 mov      ecx, dword ptr [rsi + 0x1c]
0x140a7cada mov      rdx, qword ptr [rsi + 0x10]
0x140a7cade mov      dword ptr [rdx], ecx
0x140a7cae0 mov      rax, qword ptr [rsi]
0x140a7cae3 mov      dword ptr [rax + 0x28], 0x3f800000
0x140a7caea mov      r8d, dword ptr [rbx + 0x1c]
