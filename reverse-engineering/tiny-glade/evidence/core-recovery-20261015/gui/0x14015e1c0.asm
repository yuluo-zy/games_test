0x14015e1c0 push     rbp
0x14015e1c1 push     r15
0x14015e1c3 push     r14
0x14015e1c5 push     r13
0x14015e1c7 push     r12
0x14015e1c9 push     rsi
0x14015e1ca push     rdi
0x14015e1cb push     rbx
0x14015e1cc sub      rsp, 0x198
0x14015e1d3 lea      rbp, [rsp + 0x80]
0x14015e1db movaps   xmmword ptr [rbp + 0x100], xmm6
0x14015e1e2 mov      qword ptr [rbp + 0xf8], 0xfffffffffffffffe
0x14015e1ed mov      qword ptr [rbp + 0xe8], rdx
0x14015e1f4 mov      rax, qword ptr [rcx]
0x14015e1f7 mov      rcx, qword ptr [rcx + 8]
0x14015e1fb mov      edx, dword ptr [rax + 0x1c]
0x14015e1fe mov      r8, qword ptr [rax + 0x10]
0x14015e202 mov      dword ptr [r8], edx
0x14015e205 mov      rbx, qword ptr [rax]
0x14015e208 lea      r14, [rbp + 0xd9]
0x14015e20f mov      rax, qword ptr [rcx]
0x14015e212 mov      qword ptr [rbp + 0xf0], rax
0x14015e219 xor      edi, edi
0x14015e21b xorps    xmm6, xmm6
0x14015e21e xor      esi, esi
0x14015e220 cmp      rsi, 0xa0
0x14015e227 jne      0x14015e241
0x14015e229 jmp      0x14015e42b
0x14015e22e nop      
0x14015e230 add      rsi, 0x20
0x14015e234 cmp      rsi, 0xa0
0x14015e23b je       0x14015e42b
0x14015e241 cmp      rdi, qword ptr [rbx + rsi + 0x10]
0x14015e246 jo       0x14015e230
0x14015e248 mov      r15, qword ptr [rbx + rsi + 0x20]
0x14015e24d test     r15, r15
0x14015e250 js       0x14015e454
0x14015e256 mov      r12, qword ptr [rbx + rsi + 0x18]
0x14015e25b je       0x14015e286
0x14015e25d movzx    eax, byte ptr [rip + 0x34d69a5]
0x14015e264 mov      edi, 1
0x14015e269 mov      edx, 1
0x14015e26e mov      rcx, r15
0x14015e271 call     0x140613c10
0x14015e276 test     rax, rax
0x14015e279 je       0x14015e456
0x14015e27f mov      r13, rax
0x14015e282 xor      edi, edi
0x14015e284 jmp      0x14015e28c
0x14015e286 mov      r13d, 1
0x14015e28c mov      rcx, r13
0x14015e28f mov      rdx, r12
0x14015e292 mov      r8, r15
0x14015e295 call     0x142923c00
0x14015e29a lea      rax, [rbp + 4]
0x14015e29e mov      dword ptr [rax], 0
0x14015e2a4 mov      dword ptr [rax + 3], 0
0x14015e2ab mov      qword ptr [rbp + 0x78], 3
0x14015e2b3 mov      qword ptr [rbp + 0x90], 4
0x14015e2be mov      qword ptr [rbp + 0xa8], r15
0x14015e2c5 mov      qword ptr [rbp + 0xb0], r13
0x14015e2cc mov      qword ptr [rbp + 0xb8], r15
0x14015e2d3 mov      dword ptr [rbp + 0xc0], 0
0x14015e2dd mov      dword ptr [rbp + 0xc8], 0
0x14015e2e7 mov      byte ptr [r14 - 1], 0
0x14015e2ec mov      qword ptr [r14 - 9], 0
0x14015e2f4 mov      eax, dword ptr [rbp + 7]
0x14015e2f7 mov      dword ptr [r14 + 7], eax
0x14015e2fb mov      rax, qword ptr [rbp]
0x14015e2ff mov      qword ptr [r14], rax
0x14015e302 cmp      rdi, qword ptr [rbx + rsi + 0x10]
0x14015e307 jo       0x14015e446
0x14015e30d mov      eax, dword ptr [rbx + rsi + 0x28]
0x14015e311 mov      byte ptr [rbp + 0xd8], 1
0x14015e318 mov      dword ptr [rbp + 0xd9], eax
0x14015e31e movzx    eax, byte ptr [rbp + 0xd8]
0x14015e325 mov      byte ptr [rbp + 0x60], al
0x14015e328 mov      eax, dword ptr [rbp + 0xd9]
0x14015e32e mov      dword ptr [rbp + 0x61], eax
0x14015e331 mov      rax, qword ptr [rbp + 0xdd]
0x14015e338 mov      qword ptr [rbp + 0x65], rax
0x14015e33c movzx    eax, word ptr [rbp + 0xe5]
0x14015e343 mov      word ptr [rbp + 0x6d], ax
0x14015e347 movzx    eax, byte ptr [rbp + 0xe7]
0x14015e34e mov      byte ptr [rbp + 0x6f], al
0x14015e351 movups   xmm0, xmmword ptr [rbp + 0xc8]
0x14015e358 movaps   xmmword ptr [rbp + 0x50], xmm0
0x14015e35c movups   xmm0, xmmword ptr [rbp + 0xb8]
0x14015e363 movaps   xmmword ptr [rbp + 0x40], xmm0
0x14015e367 movups   xmm0, xmmword ptr [rbp + 0x78]
0x14015e36b movups   xmm1, xmmword ptr [rbp + 0x88]
0x14015e372 movups   xmm2, xmmword ptr [rbp + 0x98]
0x14015e379 movups   xmm3, xmmword ptr [rbp + 0xa8]
0x14015e380 movaps   xmmword ptr [rbp + 0x30], xmm3
0x14015e384 movaps   xmmword ptr [rbp + 0x20], xmm2
0x14015e388 movaps   xmmword ptr [rbp + 0x10], xmm1
0x14015e38c movaps   xmmword ptr [rbp], xmm0
0x14015e390 mov      word ptr [rbp + 0x70], 0x200
0x14015e396 mov      byte ptr [rbp + 0x72], 2
0x14015e39a mov      byte ptr [rbp + 0x75], 2
0x14015e39e lea      rcx, [rbp - 0x60]
0x14015e3a2 mov      rdx, rbp
0x14015e3a5 mov      r8, qword ptr [rbp + 0xe8]
0x14015e3ac call     0x140c28d40
0x14015e3b1 mov      rax, qword ptr [rbp - 0x50]
0x14015e3b5 lock dec qword ptr [rax]
0x14015e3b9 jne      0x14015e3c4
0x14015e3bb lea      rcx, [rbp - 0x50]
0x14015e3bf call     0x140c41170
0x14015e3c4 mov      rdx, qword ptr [rbx + rsi + 0x10]
0x14015e3c9 mov      rax, rdx
0x14015e3cc neg      rax
0x14015e3cf jo       0x14015e468
0x14015e3d5 movss    xmm0, dword ptr [rbx + rsi + 0x2c]
0x14015e3db mov      rax, qword ptr [rbp + 0xf0]
0x14015e3e2 subss    xmm0, dword ptr [rax + 0x28]
0x14015e3e7 movss    dword ptr [rbx + rsi + 0x2c], xmm0
0x14015e3ed ucomiss  xmm6, xmm0
0x14015e3f0 jbe      0x14015e230
0x14015e3f6 test     rdx, rdx
0x14015e3f9 je       0x14015e40b
0x14015e3fb mov      rcx, qword ptr [rbx + rsi + 0x18]
0x14015e400 mov      r8d, 1
0x14015e406 call     0x140613c20
0x14015e40b movabs   rax, 0x8000000000000000
0x14015e415 mov      qword ptr [rbx + rsi + 0x10], rax
0x14015e41a add      rsi, 0x20
0x14015e41e cmp      rsi, 0xa0
0x14015e425 jne      0x14015e241
0x14015e42b movaps   xmm6, xmmword ptr [rbp + 0x100]
0x14015e432 add      rsp, 0x198
0x14015e439 pop      rbx
0x14015e43a pop      rdi
0x14015e43b pop      rsi
0x14015e43c pop      r12
0x14015e43e pop      r13
0x14015e440 pop      r14
0x14015e442 pop      r15
0x14015e444 pop      rbp
0x14015e445 ret      
0x14015e446 lea      rcx, [rip + 0x27fe5a3]
0x14015e44d call     0x1428d9310
0x14015e452 ud2      
0x14015e454 xor      edi, edi
0x14015e456 lea      r8, [rip + 0x27fe433]
0x14015e45d mov      rcx, rdi
0x14015e460 mov      rdx, r15
0x14015e463 call     0x1428d8fe3
0x14015e468 lea      rcx, [rip + 0x27fe599]