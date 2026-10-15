0x14093fce0 push     rbp
0x14093fce1 push     r14
0x14093fce3 push     rsi
0x14093fce4 push     rdi
0x14093fce5 push     rbx
0x14093fce6 sub      rsp, 0xd0
0x14093fced lea      rbp, [rsp + 0x80]
0x14093fcf5 mov      qword ptr [rbp + 0x48], 0xfffffffffffffffe
0x14093fcfd mov      rsi, rcx
0x14093fd00 movabs   rax, 0x8000000000000000
0x14093fd0a mov      qword ptr [rbp - 0x60], rax
0x14093fd0e mov      qword ptr [rbp - 0x40], rax
0x14093fd12 mov      qword ptr [rbp - 0x20], rax
0x14093fd16 mov      qword ptr [rbp], rax
0x14093fd1a mov      qword ptr [rbp + 0x20], rax
0x14093fd1e mov      qword ptr [rbp + 0x40], 0
0x14093fd26 call     0x140d4d740
0x14093fd2b mov      r14, rax
0x14093fd2e test     byte ptr [rax], 1
0x14093fd31 je       0x14093fd44
0x14093fd33 mov      rax, r14
0x14093fd36 add      rax, 8
0x14093fd3a mov      rdi, qword ptr [r14 + 8]
0x14093fd3e mov      rbx, qword ptr [r14 + 0x10]
0x14093fd42 jmp      0x14093fd65
0x14093fd44 call     0x140d64e00
0x14093fd49 mov      rdi, rax
0x14093fd4c mov      rbx, rdx
0x14093fd4f mov      rax, r14
0x14093fd52 add      rax, 8
0x14093fd56 mov      qword ptr [r14], 1
0x14093fd5d mov      qword ptr [r14 + 8], rdi
0x14093fd61 mov      qword ptr [r14 + 0x10], rdx
0x14093fd65 lea      rcx, [rdi + 1]
0x14093fd69 mov      qword ptr [rax], rcx
0x14093fd6c mov      byte ptr [rsi + 0x102], 0
0x14093fd73 mov      dword ptr [rsi + 0xf0], 0
0x14093fd7d mov      byte ptr [rsi + 0xf4], 0
0x14093fd84 mov      word ptr [rsi + 0xf5], 0
0x14093fd8d movsd    xmm0, qword ptr [rip + 0x21725db]
0x14093fd95 movsd    qword ptr [rsi + 0xe8], xmm0
0x14093fd9d mov      qword ptr [rsi], 0
0x14093fda4 mov      byte ptr [rsi + 0xf7], 1
0x14093fdab lea      rcx, [rsi + 0x10]
0x14093fdaf lea      rdx, [rbp - 0x60]
0x14093fdb3 mov      r8d, 0xa8
0x14093fdb9 call     0x142923c00
0x14093fdbe mov      byte ptr [rsi + 0xf8], 0
0x14093fdc5 mov      word ptr [rsi + 0x103], 0
0x14093fdce movups   xmm0, xmmword ptr [rip + 0x217271b]
0x14093fdd5 movups   xmmword ptr [rsi + 0xb8], xmm0
0x14093fddc movups   xmm0, xmmword ptr [rip + 0x217271d]
0x14093fde3 movups   xmmword ptr [rsi + 0xc8], xmm0
0x14093fdea mov      qword ptr [rsi + 0xd8], rdi
0x14093fdf1 mov      qword ptr [rsi + 0xe0], rbx
0x14093fdf8 mov      qword ptr [rsi + 0xf9], 0
0x14093fe03 mov      byte ptr [rsi + 0x101], 0
0x14093fe0a mov      rax, rsi
0x14093fe0d add      rsp, 0xd0
0x14093fe14 pop      rbx
0x14093fe15 pop      rdi
0x14093fe16 pop      rsi
0x14093fe17 pop      r14
0x14093fe19 pop      rbp
0x14093fe1a ret      