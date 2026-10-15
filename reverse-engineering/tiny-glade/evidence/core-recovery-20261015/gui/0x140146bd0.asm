0x140146bd0 push     rsi
0x140146bd1 push     rdi
0x140146bd2 sub      rsp, 0x1f8
0x140146bd9 movaps   xmmword ptr [rsp + 0x1e0], xmm15
0x140146be2 movaps   xmmword ptr [rsp + 0x1d0], xmm14
0x140146beb movaps   xmmword ptr [rsp + 0x1c0], xmm13
0x140146bf4 movaps   xmmword ptr [rsp + 0x1b0], xmm12
0x140146bfd movaps   xmmword ptr [rsp + 0x1a0], xmm11
0x140146c06 movaps   xmmword ptr [rsp + 0x190], xmm10
0x140146c0f movaps   xmmword ptr [rsp + 0x180], xmm9
0x140146c18 movaps   xmmword ptr [rsp + 0x170], xmm8
0x140146c21 movaps   xmmword ptr [rsp + 0x160], xmm7
0x140146c29 movaps   xmmword ptr [rsp + 0x150], xmm6
0x140146c31 mov      rsi, rdx
0x140146c34 movabs   rax, 0x101000001010100
0x140146c3e mov      qword ptr [rsp + 0xb8], rax
0x140146c46 movaps   xmm0, xmmword ptr [rip + 0x27e7a53]
0x140146c4d movups   xmmword ptr [rsp + 0xa0], xmm0
0x140146c55 mov      qword ptr [rsp + 0xb0], 0
0x140146c61 mov      dword ptr [rsp + 0x90], 0
0x140146c6c mov      dword ptr [rsp + 0x98], 0
0x140146c77 mov      word ptr [rsp + 0xc0], 0x101
0x140146c81 movups   xmm6, xmmword ptr [rcx]
0x140146c84 movups   xmm7, xmmword ptr [rcx + 0x10]
0x140146c88 movups   xmm8, xmmword ptr [rcx + 0x20]
0x140146c8d movups   xmm9, xmmword ptr [rcx + 0x30]
0x140146c92 movups   xmm10, xmmword ptr [rcx + 0x40]
0x140146c97 movups   xmm11, xmmword ptr [rcx + 0x50]
0x140146c9c movups   xmm12, xmmword ptr [rcx + 0x60]
0x140146ca1 movups   xmm13, xmmword ptr [rcx + 0x70]
0x140146ca6 movups   xmm14, xmmword ptr [rcx + 0x80]
0x140146cae movups   xmm15, xmmword ptr [rcx + 0x90]
0x140146cb6 movups   xmm0, xmmword ptr [rcx + 0xa0]
0x140146cbd movaps   xmmword ptr [rsp + 0x80], xmm0
0x140146cc5 movups   xmm0, xmmword ptr [rcx + 0xb0]
0x140146ccc movaps   xmmword ptr [rsp + 0x70], xmm0
0x140146cd1 movups   xmm0, xmmword ptr [rcx + 0xc0]
0x140146cd8 movaps   xmmword ptr [rsp + 0x60], xmm0
0x140146cdd movups   xmm0, xmmword ptr [rcx + 0xd0]
0x140146ce4 movaps   xmmword ptr [rsp + 0x50], xmm0
0x140146ce9 movups   xmm0, xmmword ptr [rcx + 0xe0]
0x140146cf0 movaps   xmmword ptr [rsp + 0x40], xmm0
0x140146cf5 movups   xmm0, xmmword ptr [rcx + 0xf0]
0x140146cfc movaps   xmmword ptr [rsp + 0x30], xmm0
0x140146d01 mov      rdi, qword ptr [rcx + 0x100]
0x140146d08 movzx    eax, byte ptr [rip + 0x34edefa]
0x140146d0f mov      ecx, 0x108
0x140146d14 mov      edx, 8
0x140146d19 call     0x140613c10
0x140146d1e test     rax, rax
0x140146d21 je       0x140146e37
0x140146d27 movups   xmmword ptr [rax], xmm6
0x140146d2a movups   xmmword ptr [rax + 0x10], xmm7
0x140146d2e movups   xmmword ptr [rax + 0x20], xmm8
0x140146d33 movups   xmmword ptr [rax + 0x30], xmm9
0x140146d38 movups   xmmword ptr [rax + 0x40], xmm10
0x140146d3d movups   xmmword ptr [rax + 0x50], xmm11
0x140146d42 movups   xmmword ptr [rax + 0x60], xmm12
0x140146d47 movups   xmmword ptr [rax + 0x70], xmm13
0x140146d4c movups   xmmword ptr [rax + 0x80], xmm14
0x140146d54 movups   xmmword ptr [rax + 0x90], xmm15
0x140146d5c movaps   xmm0, xmmword ptr [rsp + 0x80]
0x140146d64 movups   xmmword ptr [rax + 0xa0], xmm0
0x140146d6b movaps   xmm0, xmmword ptr [rsp + 0x70]
0x140146d70 movups   xmmword ptr [rax + 0xb0], xmm0
0x140146d77 movaps   xmm0, xmmword ptr [rsp + 0x60]
0x140146d7c movups   xmmword ptr [rax + 0xc0], xmm0
0x140146d83 movaps   xmm0, xmmword ptr [rsp + 0x50]
0x140146d88 movups   xmmword ptr [rax + 0xd0], xmm0
0x140146d8f movaps   xmm0, xmmword ptr [rsp + 0x40]
0x140146d94 movups   xmmword ptr [rax + 0xe0], xmm0
0x140146d9b movaps   xmm0, xmmword ptr [rsp + 0x30]
0x140146da0 movups   xmmword ptr [rax + 0xf0], xmm0
0x140146da7 mov      qword ptr [rax + 0x100], rdi
0x140146dae lea      rcx, [rip + 0x28981c3]
0x140146db5 mov      qword ptr [rsp + 0x20], rcx
0x140146dba lea      rcx, [rsp + 0xc8]
0x140146dc2 lea      rdx, [rsp + 0x90]
0x140146dca mov      r8, rsi
0x140146dcd mov      r9, rax
0x140146dd0 call     0x140492b20
0x140146dd5 movaps   xmm6, xmmword ptr [rsp + 0x150]
0x140146ddd movaps   xmm7, xmmword ptr [rsp + 0x160]
0x140146de5 movaps   xmm8, xmmword ptr [rsp + 0x170]
0x140146dee movaps   xmm9, xmmword ptr [rsp + 0x180]
0x140146df7 movaps   xmm10, xmmword ptr [rsp + 0x190]
0x140146e00 movaps   xmm11, xmmword ptr [rsp + 0x1a0]
0x140146e09 movaps   xmm12, xmmword ptr [rsp + 0x1b0]
0x140146e12 movaps   xmm13, xmmword ptr [rsp + 0x1c0]
0x140146e1b movaps   xmm14, xmmword ptr [rsp + 0x1d0]
0x140146e24 movaps   xmm15, xmmword ptr [rsp + 0x1e0]
0x140146e2d add      rsp, 0x1f8
0x140146e34 pop      rdi
0x140146e35 pop      rsi
0x140146e36 ret      
0x140146e37 mov      ecx, 8
0x140146e3c mov      edx, 0x108
0x140146e41 call     0x1428d9000