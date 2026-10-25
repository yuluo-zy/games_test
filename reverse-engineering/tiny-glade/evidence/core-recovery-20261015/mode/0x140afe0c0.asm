0x140afe0c0 push     rbp
0x140afe0c1 push     r15
0x140afe0c3 push     r14
0x140afe0c5 push     r13
0x140afe0c7 push     r12
0x140afe0c9 push     rsi
0x140afe0ca push     rdi
0x140afe0cb push     rbx
0x140afe0cc sub      rsp, 0x58
0x140afe0d0 lea      rbp, [rsp + 0x50]
0x140afe0d5 mov      qword ptr [rbp], 0xfffffffffffffffe
0x140afe0dd mov      rbx, qword ptr [rbp + 0x70]
0x140afe0e1 test     rbx, rbx
0x140afe0e4 sete     al
0x140afe0e7 mov      r14, rdx
0x140afe0ea sub      r14, rbx
0x140afe0ed setbe    r10b
0x140afe0f1 or       r10b, al
0x140afe0f4 jne      0x140afe105
0x140afe0f6 cmp      r14, rbx
0x140afe0f9 mov      r15, rbx
0x140afe0fc cmovb    r15, r14
0x140afe100 cmp      r15, r9
0x140afe103 jbe      0x140afe116
0x140afe105 add      rsp, 0x58
0x140afe109 pop      rbx
0x140afe10a pop      rdi
0x140afe10b pop      rsi
0x140afe10c pop      r12
0x140afe10e pop      r13
0x140afe110 pop      r14
0x140afe112 pop      r15
0x140afe114 pop      rbp
0x140afe115 ret      
0x140afe116 mov      rax, qword ptr [rbp + 0x78]
0x140afe11a mov      qword ptr [rbp - 0x28], rax
0x140afe11e lea      rax, [rbx + rbx*2]
0x140afe122 lea      rdi, [rcx + rax*8]
0x140afe126 cmp      r14, rbx
0x140afe129 mov      qword ptr [rbp - 8], rcx
0x140afe12d mov      rax, rcx
0x140afe130 cmovb    rax, rdi
0x140afe134 lea      rcx, [r15*8]
0x140afe13c lea      rsi, [rcx + rcx*2]
0x140afe140 mov      rcx, r8
0x140afe143 mov      r12, rdx
0x140afe146 mov      rdx, rax
0x140afe149 mov      r13, r8
0x140afe14c mov      r8, rsi
0x140afe14f call     0x142923c00 ; memcpy
0x140afe154 mov      rdx, r13
0x140afe157 add      rsi, r13
0x140afe15a lea      r8, [r12 + r12*2]
0x140afe15e mov      rax, qword ptr [rbp - 0x28]
0x140afe162 mov      r9, qword ptr [rax]
0x140afe165 cmp      r14, rbx
0x140afe168 jae      0x140afe220
0x140afe16e mov      rax, qword ptr [r9]
0x140afe171 mov      rcx, qword ptr [rbp - 8]
0x140afe175 lea      r8, [rcx + r8*8]
0x140afe179 add      r8, -0x18
0x140afe17d nop      dword ptr [rax]
0x140afe180 movsd    xmm0, qword ptr [rsi - 0x18]
0x140afe185 movss    xmm1, dword ptr [rax]
0x140afe189 movss    xmm2, dword ptr [rax + 8]
0x140afe18e unpcklps xmm1, xmm2
0x140afe191 subps    xmm0, xmm1
0x140afe194 mulps    xmm0, xmm0
0x140afe197 movshdup xmm2, xmm0
0x140afe19b addss    xmm2, xmm0
0x140afe19f xorps    xmm0, xmm0
0x140afe1a2 sqrtss   xmm0, xmm2
0x140afe1a6 movsd    xmm2, qword ptr [rdi - 0x18]
0x140afe1ab subps    xmm2, xmm1
0x140afe1ae mulps    xmm2, xmm2
0x140afe1b1 movshdup xmm1, xmm2
0x140afe1b5 addss    xmm1, xmm2
0x140afe1b9 sqrtss   xmm1, xmm1
0x140afe1bd ucomiss  xmm0, xmm1
0x140afe1c0 jp       0x140afe2f2
0x140afe1c6 add      rdi, -0x18
0x140afe1ca add      rsi, -0x18
0x140afe1ce xor      r9d, r9d
0x140afe1d1 xor      r10d, r10d
0x140afe1d4 ucomiss  xmm1, xmm0
0x140afe1d7 seta     r9b
0x140afe1db setbe    r10b
0x140afe1df mov      r11, rsi
0x140afe1e2 cmova    r11, rdi
0x140afe1e6 mov      rbx, qword ptr [r11 + 0x10]
0x140afe1ea mov      qword ptr [r8 + 0x10], rbx
0x140afe1ee movups   xmm0, xmmword ptr [r11]
0x140afe1f2 movups   xmmword ptr [r8], xmm0
0x140afe1f6 lea      r10, [r10 + r10*2]
0x140afe1fa lea      rdi, [rdi + r10*8]
0x140afe1fe lea      r9, [r9 + r9*2]
0x140afe202 lea      rsi, [rsi + r9*8]
0x140afe206 cmp      rdi, rcx
0x140afe209 je       0x140afe218
0x140afe20b add      r8, -0x18
0x140afe20f cmp      rsi, rdx
0x140afe212 jne      0x140afe180
0x140afe218 mov      rcx, rdi
0x140afe21b jmp      0x140afe2d7 ; 
0x140afe220 test     r15, r15
0x140afe223 je       0x140afe2d3
0x140afe229 mov      rcx, qword ptr [rbp - 8]
0x140afe22d lea      rax, [rcx + r8*8]
0x140afe231 mov      r8, qword ptr [r9]
0x140afe234 nop      word ptr cs:[rax + rax]
0x140afe240 movsd    xmm0, qword ptr [rdi]
0x140afe244 movss    xmm1, dword ptr [r8]
0x140afe249 movss    xmm2, dword ptr [r8 + 8]
0x140afe24f unpcklps xmm1, xmm2
0x140afe252 subps    xmm0, xmm1
0x140afe255 mulps    xmm0, xmm0
0x140afe258 movshdup xmm2, xmm0
0x140afe25c addss    xmm2, xmm0
0x140afe260 xorps    xmm0, xmm0
0x140afe263 sqrtss   xmm0, xmm2
0x140afe267 movsd    xmm2, qword ptr [rdx]
0x140afe26b subps    xmm2, xmm1
0x140afe26e mulps    xmm2, xmm2
0x140afe271 movshdup xmm1, xmm2
0x140afe275 addss    xmm1, xmm2
0x140afe279 sqrtss   xmm1, xmm1
0x140afe27d ucomiss  xmm0, xmm1
0x140afe280 jp       0x140afe30c
0x140afe286 xor      r9d, r9d
0x140afe289 xor      r11d, r11d
0x140afe28c ucomiss  xmm1, xmm0
0x140afe28f seta     r10b
0x140afe293 setbe    r11b
0x140afe297 mov      rbx, rdx
0x140afe29a cmova    rbx, rdi
0x140afe29e mov      r14, qword ptr [rbx + 0x10]
0x140afe2a2 mov      qword ptr [rcx + 0x10], r14
0x140afe2a6 movups   xmm0, xmmword ptr [rbx]
0x140afe2a9 movups   xmmword ptr [rcx], xmm0
0x140afe2ac lea      r11, [r11 + r11*2]
0x140afe2b0 lea      rdx, [rdx + r11*8]
0x140afe2b4 add      rcx, 0x18
0x140afe2b8 cmp      rdx, rsi
0x140afe2bb je       0x140afe2d7
0x140afe2bd mov      r9b, r10b
0x140afe2c0 lea      r9, [r9 + r9*2]
0x140afe2c4 lea      rdi, [rdi + r9*8]
0x140afe2c8 cmp      rdi, rax
0x140afe2cb jne      0x140afe240
0x140afe2d1 jmp      0x140afe2d7 ; 
0x140afe2d3 mov      rcx, qword ptr [rbp - 8]
0x140afe2d7 sub      rsi, rdx
0x140afe2da mov      r8, rsi
0x140afe2dd add      rsp, 0x58
0x140afe2e1 pop      rbx
0x140afe2e2 pop      rdi
0x140afe2e3 pop      rsi
0x140afe2e4 pop      r12
0x140afe2e6 pop      r13
0x140afe2e8 pop      r14
0x140afe2ea pop      r15
0x140afe2ec pop      rbp
0x140afe2ed jmp      0x142923c00 ; memcpy
0x140afe2f2 mov      qword ptr [rbp - 0x10], rdi
0x140afe2f6 mov      qword ptr [rbp - 0x18], rsi
0x140afe2fa mov      qword ptr [rbp - 0x20], rdx
0x140afe2fe lea      rcx, [rip + 0x1fe82db]
0x140afe305 call     0x1428d9310 ; core::option::unwrap_failed::h837cefa1ab459360
0x140afe30a jmp      0x140afe324 ; 
0x140afe30c mov      qword ptr [rbp - 0x10], rcx
0x140afe310 mov      qword ptr [rbp - 0x18], rsi
0x140afe314 mov      qword ptr [rbp - 0x20], rdx
0x140afe318 lea      rcx, [rip + 0x1fe82c1]
0x140afe31f call     0x1428d9310 ; core::option::unwrap_failed::h837cefa1ab459360
0x140afe324 ud2      
0x140afe326 nop      word ptr cs:[rax + rax]
0x140afe330 mov      qword ptr [rsp + 0x10], rdx
0x140afe335 push     rbp
0x140afe336 push     r15
0x140afe338 push     r14
0x140afe33a push     r13
0x140afe33c push     r12
0x140afe33e push     rsi
0x140afe33f push     rdi
0x140afe340 push     rbx
0x140afe341 sub      rsp, 0x28
0x140afe345 lea      rbp, [rdx + 0x50]
0x140afe349 mov      rdx, qword ptr [rbp - 0x20]
0x140afe34d mov      r8, qword ptr [rbp - 0x18]
0x140afe351 mov      rcx, qword ptr [rbp - 0x10]
0x140afe355 sub      r8, rdx
0x140afe358 call     0x142923c00 ; memcpy
0x140afe35d nop      
0x140afe35e add      rsp, 0x28
0x140afe362 pop      rbx
0x140afe363 pop      rdi
0x140afe364 pop      rsi
0x140afe365 pop      r12
0x140afe367 pop      r13
0x140afe369 pop      r14
0x140afe36b pop      r15
0x140afe36d pop      rbp
0x140afe36e ret      