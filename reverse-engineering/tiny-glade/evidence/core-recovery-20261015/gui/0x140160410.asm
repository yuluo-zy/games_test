0x140160410 push     rbp
0x140160411 push     r15
0x140160413 push     r14
0x140160415 push     r12
0x140160417 push     rsi
0x140160418 push     rdi
0x140160419 push     rbx
0x14016041a sub      rsp, 0x4e0
0x140160421 lea      rbp, [rsp + 0x80]
0x140160429 mov      qword ptr [rbp + 0x458], 0xfffffffffffffffe
0x140160434 mov      rax, qword ptr [rcx]
0x140160437 xor      r9d, r9d
0x14016043a xor      r10d, r10d
0x14016043d cmp      r9, qword ptr [rax + 0x10]
0x140160441 setno    r10b
0x140160445 xor      r11d, r11d
0x140160448 cmp      r9, qword ptr [rax + 0x30]
0x14016044c setno    r11b
0x140160450 add      r11, r10
0x140160453 xor      r10d, r10d
0x140160456 cmp      r9, qword ptr [rax + 0x50]
0x14016045a setno    r10b
0x14016045e xor      esi, esi
0x140160460 cmp      r9, qword ptr [rax + 0x70]
0x140160464 setno    sil
0x140160468 add      rsi, r10
0x14016046b add      rsi, r11
0x14016046e xor      r10d, r10d
0x140160471 cmp      r9, qword ptr [rax + 0x90]
0x140160478 setno    r10b
0x14016047c or       r10, rsi
0x14016047f je       0x14016064e
0x140160485 mov      rsi, qword ptr [rdx]
0x140160488 test     rsi, rsi
0x14016048b je       0x14016064e
0x140160491 cmp      byte ptr [rax + 0xf7], 1
0x140160498 jne      0x14016064e
0x14016049e mov      r14, rcx
0x1401604a1 mov      rbx, r8
0x1401604a4 lea      rdx, [rip + 0x27fc515] ; @0x14295c9c0 ' crates/tiny-glade/src/gui/gui_error_log.rs'
0x1401604ab lea      rdi, [rbp + 0x138]
0x1401604b2 mov      r8d, 1
0x1401604b8 mov      rcx, rdi
0x1401604bb call     0x140053bb0
0x1401604c0 mov      byte ptr [rbp + 0x2bc], 0
0x1401604c7 lea      rcx, [rbp + 0x2c0]
0x1401604ce mov      r8d, 0x188
0x1401604d4 mov      rdx, rdi
0x1401604d7 call     0x142923c00
0x1401604dc lea      rcx, [rbp + 0x138]
0x1401604e3 call     0x14064f610
0x1401604e8 mov      dword ptr [rbp + 0x3f0], 1
0x1401604f2 movups   xmm0, xmmword ptr [rbp + 0x138]
0x1401604f9 movups   xmm1, xmmword ptr [rbp + 0x148]
0x140160500 movups   xmm2, xmmword ptr [rbp + 0x158]
0x140160507 movups   xmm3, xmmword ptr [rbp + 0x168]
0x14016050e movups   xmmword ptr [rbp + 0x3f4], xmm0
0x140160515 movups   xmmword ptr [rbp + 0x404], xmm1
0x14016051c movups   xmmword ptr [rbp + 0x414], xmm2
0x140160523 movups   xmmword ptr [rbp + 0x424], xmm3
0x14016052a movups   xmm0, xmmword ptr [rbp + 0x178]
0x140160531 movups   xmmword ptr [rbp + 0x434], xmm0
0x140160538 lea      rcx, [rbp - 0x50]
0x14016053c lea      rdx, [rbp + 0x2c0]
0x140160543 mov      r8d, 0xdc
0x140160549 call     0x142923c00
0x14016054e mov      di, word ptr [rbp + 0x39e]
0x140160555 mov      r15w, word ptr [rbp + 0x3a8]
0x14016055d mov      r12b, byte ptr [rbp + 0x3ab]
0x140160564 lea      rdx, [rbp + 0x3ac]
0x14016056b lea      rcx, [rbp + 0x9c]
0x140160572 mov      r8d, 0x9c
0x140160578 call     0x142923c00
0x14016057d and      r12b, 1
0x140160581 mov      word ptr [rbp + 0x8c], 2
0x14016058a mov      word ptr [rbp + 0x8e], di
0x140160591 movss    xmm0, dword ptr [rip + 0x27fc0ff]
0x140160599 movlps   qword ptr [rbp + 0x90], xmm0
0x1401605a0 mov      word ptr [rbp + 0x98], r15w
0x1401605a8 mov      byte ptr [rbp + 0x9a], 0
0x1401605af mov      byte ptr [rbp + 0x9b], r12b
0x1401605b6 mov      byte ptr [rbp + 0x457], 1
0x1401605bd mov      rcx, rsi
0x1401605c0 call     0x14067dd80
0x1401605c5 mov      rsi, rax
0x1401605c8 lea      rcx, [rbp + 0x138]
0x1401605cf lea      rdx, [rbp - 0x50]
0x1401605d3 mov      r8d, 0x188
0x1401605d9 call     0x142923c00
0x1401605de movzx    eax, byte ptr [rip + 0x34d4624]
0x1401605e5 mov      ecx, 0x10
0x1401605ea mov      edx, 8
0x1401605ef call     0x140613c10
0x1401605f4 test     rax, rax
0x1401605f7 mov      qword ptr [rbp + 0x448], rax
0x1401605fe je       0x140160661
0x140160600 mov      r9, rax
0x140160603 mov      qword ptr [rax], r14
0x140160606 mov      qword ptr [rax + 8], rbx
0x14016060a lea      rax, [rip + 0x27fc20f]
0x140160611 mov      qword ptr [rsp + 0x20], rax
0x140160616 lea      rcx, [rbp + 0x2c0]
0x14016061d lea      rdx, [rbp + 0x138]
0x140160624 mov      r8, rsi
0x140160627 call     0x140053ee0
0x14016062c cmp      dword ptr [rbp + 0x300], 2
0x140160633 je       0x14016064e
0x140160635 mov      rax, qword ptr [rbp + 0x2d0]
0x14016063c lock dec qword ptr [rax]
0x140160640 jne      0x14016064e
0x140160642 lea      rcx, [rbp + 0x2d0]
0x140160649 call     0x140c41170
0x14016064e nop      
0x14016064f add      rsp, 0x4e0
0x140160656 pop      rbx
0x140160657 pop      rdi
0x140160658 pop      rsi
0x140160659 pop      r12
0x14016065b pop      r14
0x14016065d pop      r15
0x14016065f pop      rbp
0x140160660 ret      
0x140160661 mov      ecx, 8
0x140160666 mov      edx, 0x10
0x14016066b call     0x1428d9000