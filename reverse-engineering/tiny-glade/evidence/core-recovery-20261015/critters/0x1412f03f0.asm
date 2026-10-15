0x1412f03f0 push     rsi
0x1412f03f1 push     rdi
0x1412f03f2 push     rbx
0x1412f03f3 mov      r9, qword ptr [rdx + 8]
0x1412f03f7 mov      rax, qword ptr [rdx + 0x18]
0x1412f03fb mov      r8, qword ptr [rdx + 0xb8]
0x1412f0402 mov      r11, qword ptr [rdx + 0xb0]
0x1412f0409 mov      r10, qword ptr [rdx + 0x28]
0x1412f040d cmp      byte ptr [rdx + 0xc0], 0
0x1412f0414 je       0x1412f04ff
0x1412f041a cmp      r8, r11
0x1412f041d jne      0x1412f05fa
0x1412f0423 mov      r11, qword ptr [rdx + 0x20]
0x1412f0427 nop      word ptr [rax + rax]
0x1412f0430 cmp      r11, r10
0x1412f0433 je       0x1412f05f2
0x1412f0439 lea      rbx, [r11 + 4]
0x1412f043d mov      qword ptr [rdx + 0x20], rbx
0x1412f0441 mov      r8d, dword ptr [r11]
0x1412f0444 mov      rsi, qword ptr [r9 + 8]
0x1412f0448 lea      rdi, [r8 + r8*8]
0x1412f044c mov      r8, qword ptr [rsi + rdi*8 + 0x10]
0x1412f0451 mov      r11, rbx
0x1412f0454 test     r8, r8
0x1412f0457 je       0x1412f0430
0x1412f0459 lea      rsi, [rsi + rdi*8]
0x1412f045d mov      r9, qword ptr [rax + 0x110]
0x1412f0464 mov      rdi, qword ptr [rsi + 0x18]
0x1412f0468 mov      r11, qword ptr [rsi + 0x38]
0x1412f046c mov      r9, qword ptr [r11 + r9*8]
0x1412f0470 not      r9
0x1412f0473 lea      r10, [r9 + r9*2]
0x1412f0477 shl      r10, 4
0x1412f047b mov      r9, qword ptr [rdi + r10 + 0x10]
0x1412f0480 mov      qword ptr [rdx + 0x50], r9
0x1412f0484 movups   xmm0, xmmword ptr [rdi + r10 + 0x20]
0x1412f048a movups   xmmword ptr [rdx + 0x58], xmm0
0x1412f048e mov      r10, qword ptr [rax + 0x118]
0x1412f0495 mov      r10, qword ptr [r11 + r10*8]
0x1412f0499 not      r10
0x1412f049c lea      rbx, [r10 + r10*2]
0x1412f04a0 shl      rbx, 4
0x1412f04a4 mov      r10, qword ptr [rdi + rbx + 0x10]
0x1412f04a9 mov      qword ptr [rdx + 0x70], r10
0x1412f04ad movups   xmm0, xmmword ptr [rdi + rbx + 0x20]
0x1412f04b2 movups   xmmword ptr [rdx + 0x78], xmm0
0x1412f04b6 mov      rax, qword ptr [rax + 0x120]
0x1412f04bd mov      rax, qword ptr [r11 + rax*8]
0x1412f04c1 not      rax
0x1412f04c4 lea      rax, [rax + rax*2]
0x1412f04c8 shl      rax, 4
0x1412f04cc mov      r11, qword ptr [rdi + rax + 0x10]
0x1412f04d1 mov      qword ptr [rdx + 0x90], r11
0x1412f04d8 movups   xmm0, xmmword ptr [rdi + rax + 0x20]
0x1412f04dd movups   xmmword ptr [rdx + 0x98], xmm0
0x1412f04e4 mov      rax, qword ptr [rsi + 8]
0x1412f04e8 mov      qword ptr [rdx + 0x30], rax
0x1412f04ec mov      qword ptr [rdx + 0x38], r8
0x1412f04f0 mov      qword ptr [rdx + 0xb0], r8
0x1412f04f7 xor      r8d, r8d
0x1412f04fa jmp      0x1412f0609
0x1412f04ff cmp      r8, r11
0x1412f0502 jne      0x1412f060e
0x1412f0508 mov      r11, qword ptr [rdx + 0x10]
0x1412f050c mov      rsi, qword ptr [rdx + 0x20]
0x1412f0510 cmp      rsi, r10
0x1412f0513 je       0x1412f05f2
0x1412f0519 lea      rbx, [rsi + 4]
0x1412f051d mov      qword ptr [rdx + 0x20], rbx
0x1412f0521 mov      esi, dword ptr [rsi]
0x1412f0523 mov      r8, qword ptr [r11 + 8]
0x1412f0527 lea      rdi, [rsi + rsi*4]
0x1412f052b shl      rdi, 5
0x1412f052f cmp      qword ptr [r8 + rdi + 0x58], 0
0x1412f0535 mov      rsi, rbx
0x1412f0538 je       0x1412f0510
0x1412f053a add      r8, rdi
0x1412f053d mov      r10d, dword ptr [r8 + 0x94]
0x1412f0544 mov      r9, qword ptr [r9 + 8]
0x1412f0548 lea      r10, [r10 + r10*8]
0x1412f054c mov      r11, qword ptr [rax + 0x110]
0x1412f0553 mov      rsi, qword ptr [r9 + r10*8 + 0x18]
0x1412f0558 mov      rdi, qword ptr [r9 + r10*8 + 0x38]
0x1412f055d mov      r9, qword ptr [rdi + r11*8]
0x1412f0561 not      r9
0x1412f0564 lea      r10, [r9 + r9*2]
0x1412f0568 shl      r10, 4
0x1412f056c mov      r9, qword ptr [rsi + r10 + 0x10]
0x1412f0571 mov      qword ptr [rdx + 0x50], r9
0x1412f0575 movups   xmm0, xmmword ptr [rsi + r10 + 0x20]
0x1412f057b movups   xmmword ptr [rdx + 0x58], xmm0
0x1412f057f mov      r10, qword ptr [rax + 0x118]
0x1412f0586 mov      r10, qword ptr [rdi + r10*8]
0x1412f058a not      r10
0x1412f058d lea      r11, [r10 + r10*2]
0x1412f0591 shl      r11, 4
0x1412f0595 mov      r10, qword ptr [rsi + r11 + 0x10]
0x1412f059a mov      qword ptr [rdx + 0x70], r10
0x1412f059e movups   xmm0, xmmword ptr [rsi + r11 + 0x20]
0x1412f05a4 movups   xmmword ptr [rdx + 0x78], xmm0
0x1412f05a8 mov      rax, qword ptr [rax + 0x120]
0x1412f05af mov      rax, qword ptr [rdi + rax*8]
0x1412f05b3 not      rax
0x1412f05b6 lea      rax, [rax + rax*2]
0x1412f05ba shl      rax, 4
0x1412f05be mov      r11, qword ptr [rsi + rax + 0x10]
0x1412f05c3 mov      qword ptr [rdx + 0x90], r11
0x1412f05ca movups   xmm0, xmmword ptr [rsi + rax + 0x20]
0x1412f05cf movups   xmmword ptr [rdx + 0x98], xmm0
0x1412f05d6 mov      rax, qword ptr [r8 + 0x50]
0x1412f05da mov      r8, qword ptr [r8 + 0x58]
0x1412f05de mov      qword ptr [rdx + 0x40], rax
0x1412f05e2 mov      qword ptr [rdx + 0x48], r8
0x1412f05e6 mov      qword ptr [rdx + 0xb0], r8
0x1412f05ed xor      r8d, r8d
0x1412f05f0 jmp      0x1412f0621
0x1412f05f2 mov      dword ptr [rcx], 2
0x1412f05f8 jmp      0x1412f0672
0x1412f05fa mov      r9, qword ptr [rdx + 0x50]
0x1412f05fe mov      r10, qword ptr [rdx + 0x70]
0x1412f0602 mov      r11, qword ptr [rdx + 0x90]
0x1412f0609 mov      eax, r8d
0x1412f060c jmp      0x1412f062c
0x1412f060e mov      rax, qword ptr [rdx + 0x40]
0x1412f0612 mov      r9, qword ptr [rdx + 0x50]
0x1412f0616 mov      r10, qword ptr [rdx + 0x70]
0x1412f061a mov      r11, qword ptr [rdx + 0x90]
0x1412f0621 mov      rsi, r8
0x1412f0624 shl      rsi, 4
0x1412f0628 mov      eax, dword ptr [rax + rsi + 8]
0x1412f062c lea      rsi, [rax + rax*2]
0x1412f0630 shl      rsi, 4
0x1412f0634 add      rsi, r9
0x1412f0637 imul     r9, rax, 0x68
0x1412f063b add      r9, r10
0x1412f063e shl      rax, 4
0x1412f0642 add      rax, r11
0x1412f0645 inc      r8
0x1412f0648 mov      qword ptr [rdx + 0xb8], r8
0x1412f064f mov      edx, dword ptr [rsi + 0x18]
0x1412f0652 mov      dword ptr [rcx + 0x24], edx
0x1412f0655 mov      rdx, qword ptr [rsi + 0x10]
0x1412f0659 mov      qword ptr [rcx + 0x1c], rdx
0x1412f065d movups   xmm0, xmmword ptr [rax]
0x1412f0660 movups   xmmword ptr [rcx], xmm0
0x1412f0663 mov      rax, qword ptr [r9 + 0x30]
0x1412f0667 mov      qword ptr [rcx + 0x10], rax
0x1412f066b mov      eax, dword ptr [r9 + 0x38]
0x1412f066f mov      dword ptr [rcx + 0x18], eax
0x1412f0672 pop      rbx
0x1412f0673 pop      rdi
0x1412f0674 pop      rsi
0x1412f0675 ret      