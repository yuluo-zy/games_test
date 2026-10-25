0x140a82390 push     rbp
0x140a82391 push     r15
0x140a82393 push     r14
0x140a82395 push     r13
0x140a82397 push     r12
0x140a82399 push     rsi
0x140a8239a push     rdi
0x140a8239b push     rbx
0x140a8239c sub      rsp, 0x58
0x140a823a0 lea      rbp, [rsp + 0x50]
0x140a823a5 mov      qword ptr [rbp], 0xfffffffffffffffe
0x140a823ad mov      r12, qword ptr [rdx + 0x10]
0x140a823b1 lea      rbx, [r12*8]
0x140a823b9 xor      r15d, r15d
0x140a823bc mov      rax, r12
0x140a823bf shr      rax, 0x3d
0x140a823c3 jne      0x140a8255c
0x140a823c9 movabs   r13, 0x7ffffffffffffffc
0x140a823d3 cmp      rbx, r13
0x140a823d6 ja       0x140a8255c
0x140a823dc mov      rdi, rdx
0x140a823df mov      rsi, rcx
0x140a823e2 mov      r14, qword ptr [rdx + 8]
0x140a823e6 test     rbx, rbx
0x140a823e9 mov      qword ptr [rbp - 0x18], r12
0x140a823ed je       0x140a8241b
0x140a823ef movzx    eax, byte ptr [rip + 0x2bb2813]
0x140a823f6 mov      r15d, 4
0x140a823fc mov      edx, 4
0x140a82401 mov      rcx, rbx
0x140a82404 call     0x140613c10 ; __rust_alloc
0x140a82409 test     rax, rax
0x140a8240c je       0x140a8255c
0x140a82412 mov      r15, r13
0x140a82415 mov      qword ptr [rbp - 8], r12
0x140a82419 jmp      0x140a8242b ; 
0x140a8241b mov      r15, r13
0x140a8241e mov      eax, 4
0x140a82423 mov      qword ptr [rbp - 8], 0
0x140a8242b mov      qword ptr [rbp - 0x10], rax
0x140a8242f mov      rcx, rax
0x140a82432 mov      rdx, r14
0x140a82435 mov      r8, rbx
0x140a82438 call     0x142923c00 ; memcpy
0x140a8243d mov      r13, qword ptr [rdi + 0x28]
0x140a82441 lea      r14, [r13*4]
0x140a82449 xor      r12d, r12d
0x140a8244c mov      rax, r13
0x140a8244f shr      rax, 0x3e
0x140a82453 jne      0x140a8256e
0x140a82459 cmp      r14, r15
0x140a8245c ja       0x140a8256e
0x140a82462 mov      r15, qword ptr [rdi + 0x20]
0x140a82466 test     r14, r14
0x140a82469 je       0x140a82496
0x140a8246b movzx    eax, byte ptr [rip + 0x2bb2797]
0x140a82472 mov      r12d, 4
0x140a82478 mov      edx, 4
0x140a8247d mov      rcx, r14
0x140a82480 call     0x140613c10 ; __rust_alloc
0x140a82485 test     rax, rax
0x140a82488 je       0x140a8256e
0x140a8248e mov      rbx, rax
0x140a82491 mov      r12, r13
0x140a82494 jmp      0x140a8249e ; 
0x140a82496 mov      ebx, 4
0x140a8249b xor      r12d, r12d
0x140a8249e mov      rcx, rbx
0x140a824a1 mov      rdx, r15
0x140a824a4 mov      r8, r14
0x140a824a7 call     0x142923c00 ; memcpy
0x140a824ac movss    xmm0, dword ptr [rdi + 0x30]
0x140a824b1 movss    xmm1, dword ptr [rdi + 0x54]
0x140a824b6 mov      ecx, dword ptr [rdi + 0x38]
0x140a824b9 lea      edx, [rcx - 2]
0x140a824bc cmp      edx, 2
0x140a824bf mov      eax, 2
0x140a824c4 cmovae   edx, eax
0x140a824c7 test     edx, edx
0x140a824c9 je       0x140a824e9
0x140a824cb cmp      edx, 1
0x140a824ce jne      0x140a824d7
0x140a824d0 mov      eax, 3
0x140a824d5 jmp      0x140a824e9 ; 
0x140a824d7 mov      rax, qword ptr [rdi + 0x4c]
0x140a824db mov      qword ptr [rbp - 0x20], rax
0x140a824df movups   xmm2, xmmword ptr [rdi + 0x3c]
0x140a824e3 movaps   xmmword ptr [rbp - 0x30], xmm2
0x140a824e7 mov      eax, ecx
0x140a824e9 movzx    ecx, word ptr [rdi + 0x68]
0x140a824ed movss    xmm2, dword ptr [rdi + 0x60]
0x140a824f2 mov      rdx, qword ptr [rbp - 8]
0x140a824f6 mov      qword ptr [rsi], rdx
0x140a824f9 mov      rdx, qword ptr [rbp - 0x10]
0x140a824fd mov      qword ptr [rsi + 8], rdx
0x140a82501 mov      rdx, qword ptr [rbp - 0x18]
0x140a82505 mov      qword ptr [rsi + 0x10], rdx
0x140a82509 mov      qword ptr [rsi + 0x18], r12
0x140a8250d mov      qword ptr [rsi + 0x20], rbx
0x140a82511 mov      qword ptr [rsi + 0x28], r13
0x140a82515 movss    dword ptr [rsi + 0x30], xmm0
0x140a8251a movss    dword ptr [rsi + 0x54], xmm1
0x140a8251f mov      dword ptr [rsi + 0x38], eax
0x140a82522 movaps   xmm0, xmmword ptr [rbp - 0x30]
0x140a82526 movups   xmmword ptr [rsi + 0x3c], xmm0
0x140a8252a mov      rax, qword ptr [rbp - 0x20]
0x140a8252e mov      qword ptr [rsi + 0x4c], rax
0x140a82532 movsd    xmm0, qword ptr [rdi + 0x58]
0x140a82537 movsd    qword ptr [rsi + 0x58], xmm0
0x140a8253c mov      word ptr [rsi + 0x68], cx
0x140a82540 movss    dword ptr [rsi + 0x60], xmm2
0x140a82545 mov      eax, dword ptr [rdi + 0x64]
0x140a82548 mov      dword ptr [rsi + 0x64], eax
0x140a8254b add      rsp, 0x58
0x140a8254f pop      rbx
0x140a82550 pop      rdi
0x140a82551 pop      rsi
0x140a82552 pop      r12
0x140a82554 pop      r13
0x140a82556 pop      r14
0x140a82558 pop      r15
0x140a8255a pop      rbp
0x140a8255b ret      
0x140a8255c lea      r8, [rip + 0x21b6615]
0x140a82563 mov      rcx, r15
0x140a82566 mov      rdx, rbx
0x140a82569 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a8256e lea      r8, [rip + 0x21b6603]
0x140a82575 mov      rcx, r12
0x140a82578 mov      rdx, r14
0x140a8257b call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a82580 ud2      
0x140a82582 nop      word ptr cs:[rax + rax]
0x140a82590 mov      qword ptr [rsp + 0x10], rdx
0x140a82595 push     rbp
0x140a82596 push     r15
0x140a82598 push     r14
0x140a8259a push     r13
0x140a8259c push     r12
0x140a8259e push     rsi
0x140a8259f push     rdi
0x140a825a0 push     rbx
0x140a825a1 sub      rsp, 0x28
0x140a825a5 lea      rbp, [rdx + 0x50]
0x140a825a9 cmp      qword ptr [rbp - 8], 0
0x140a825ae je       0x140a825c7
0x140a825b0 mov      rdx, qword ptr [rbp - 8]
0x140a825b4 shl      rdx, 3
0x140a825b8 mov      r8d, 4
0x140a825be mov      rcx, qword ptr [rbp - 0x10]
0x140a825c2 call     0x140613c20 ; __rust_dealloc
0x140a825c7 nop      
0x140a825c8 add      rsp, 0x28
0x140a825cc pop      rbx
0x140a825cd pop      rdi
0x140a825ce pop      rsi
0x140a825cf pop      r12
0x140a825d1 pop      r13
0x140a825d3 pop      r14
0x140a825d5 pop      r15
0x140a825d7 pop      rbp
0x140a825d8 ret      