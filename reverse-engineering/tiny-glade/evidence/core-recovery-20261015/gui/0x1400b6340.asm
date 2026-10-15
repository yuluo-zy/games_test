0x1400b6340 push     rbp
0x1400b6341 push     rsi
0x1400b6342 push     rdi
0x1400b6343 push     rbx
0x1400b6344 sub      rsp, 0x348
0x1400b634b lea      rbp, [rsp + 0x80]
0x1400b6353 mov      qword ptr [rbp + 0x2c0], 0xfffffffffffffffe
0x1400b635e mov      rbx, qword ptr [rcx]
0x1400b6361 mov      rdi, qword ptr [rbx]
0x1400b6364 test     rdi, rdi
0x1400b6367 je       0x1400b64db
0x1400b636d mov      rsi, rdx
0x1400b6370 mov      qword ptr [rbp - 0x58], 4
0x1400b6378 mov      byte ptr [rbp + 0x2bf], 0
0x1400b637f lea      rdx, [rip + 0x287ed60] ; @0x1429350e6 'Animate LogoForce TAA pass-thruFreeze occlusion cullingShow memory breakdownShow memory blocksGP'
0x1400b6386 lea      rcx, [rbp + 0x38]
0x1400b638a mov      r8d, 0xc
0x1400b6390 call     0x1400568d0
0x1400b6395 movups   xmm0, xmmword ptr [rbp + 0x98]
0x1400b639c movaps   xmmword ptr [rbp + 0x240], xmm0
0x1400b63a3 movups   xmm0, xmmword ptr [rbp + 0x88]
0x1400b63aa movaps   xmmword ptr [rbp + 0x230], xmm0
0x1400b63b1 movups   xmm0, xmmword ptr [rbp + 0x78]
0x1400b63b5 movaps   xmmword ptr [rbp + 0x220], xmm0
0x1400b63bc movups   xmm0, xmmword ptr [rbp + 0x38]
0x1400b63c0 movups   xmm1, xmmword ptr [rbp + 0x48]
0x1400b63c4 movups   xmm2, xmmword ptr [rbp + 0x58]
0x1400b63c8 movups   xmm3, xmmword ptr [rbp + 0x68]
0x1400b63cc movaps   xmmword ptr [rbp + 0x210], xmm3
0x1400b63d3 movaps   xmmword ptr [rbp + 0x200], xmm2
0x1400b63da movaps   xmmword ptr [rbp + 0x1f0], xmm1
0x1400b63e1 movaps   xmmword ptr [rbp + 0x1e0], xmm0
0x1400b63e8 mov      byte ptr [rbp + 0x2bf], 1
0x1400b63ef lea      rcx, [rbp + 0x38]
0x1400b63f3 lea      rdx, [rbp - 0x58]
0x1400b63f7 lea      r8, [rbp + 0x1e0]
0x1400b63fe call     0x140c46bd0
0x1400b6403 lea      rcx, [rbp + 0x258]
0x1400b640a lea      rdx, [rbp + 0x38]
0x1400b640e mov      r8, rsi
0x1400b6411 call     0x140c46d20
0x1400b6416 cmp      byte ptr [rbp + 0x2ac], 0
0x1400b641d je       0x1400b644b
0x1400b641f mov      rax, qword ptr [rbp + 0x268]
0x1400b6426 lock dec qword ptr [rax]
0x1400b642a jne      0x1400b6438
0x1400b642c lea      rcx, [rbp + 0x268]
0x1400b6433 call     0x140c41170
0x1400b6438 mov      eax, dword ptr [rbx + 0x1c]
0x1400b643b mov      rcx, qword ptr [rbx + 0x10]
0x1400b643f mov      dword ptr [rcx], eax
0x1400b6441 mov      rcx, rdi
0x1400b6444 call     0x140b19d00
0x1400b6449 jmp      0x1400b6485
0x1400b644b mov      byte ptr [rbp + 0x38], 0
0x1400b644f cmp      byte ptr [rbp + 0x2ab], 0
0x1400b6456 je       0x1400b646c
0x1400b6458 lea      rcx, [rbp + 0x268]
0x1400b645f lea      rdx, [rbp + 0x38]
0x1400b6463 call     0x140042870
0x1400b6468 test     al, al
0x1400b646a jne      0x1400b641f
0x1400b646c mov      rax, qword ptr [rbp + 0x268]
0x1400b6473 lock dec qword ptr [rax]
0x1400b6477 jne      0x1400b6485
0x1400b6479 lea      rcx, [rbp + 0x268]
0x1400b6480 call     0x140c41170
0x1400b6485 movzx    eax, byte ptr [rip + 0x357e77d]
0x1400b648c mov      ecx, 8
0x1400b6491 mov      edx, 8
0x1400b6496 call     0x140613c10
0x1400b649b test     rax, rax
0x1400b649e je       0x1400b64e8
0x1400b64a0 mov      qword ptr [rax], rbx
0x1400b64a3 lea      rcx, [rip + 0x287e82e]
0x1400b64aa mov      qword ptr [rsp + 0x20], rcx
0x1400b64af lea      rcx, [rbp + 0x38]
0x1400b64b3 movabs   r8, 0x20001000000
0x1400b64bd mov      rdx, rsi
0x1400b64c0 mov      r9, rax
0x1400b64c3 call     0x1400c2d20
0x1400b64c8 mov      rax, qword ptr [rbp + 0x48]
0x1400b64cc lock dec qword ptr [rax]
0x1400b64d0 jne      0x1400b64db
0x1400b64d2 lea      rcx, [rbp + 0x48]
0x1400b64d6 call     0x140c41170
0x1400b64db nop      
0x1400b64dc add      rsp, 0x348
0x1400b64e3 pop      rbx
0x1400b64e4 pop      rdi
0x1400b64e5 pop      rsi
0x1400b64e6 pop      rbp
0x1400b64e7 ret      
0x1400b64e8 mov      ecx, 8
0x1400b64ed mov      edx, 8
0x1400b64f2 call     0x1428d9000