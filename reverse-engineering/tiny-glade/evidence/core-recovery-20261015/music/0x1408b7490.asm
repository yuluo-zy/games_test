0x1408b7490 push     rbp
0x1408b7491 push     rsi
0x1408b7492 sub      rsp, 0xa8
0x1408b7499 lea      rbp, [rsp + 0x80]
0x1408b74a1 mov      qword ptr [rbp + 0x20], 0xfffffffffffffffe
0x1408b74a9 mov      rsi, rcx
0x1408b74ac cmp      qword ptr [rcx], 0
0x1408b74b0 je       0x1408b74f7
0x1408b74b2 mov      rax, qword ptr [rsi + 8]
0x1408b74b6 cmp      rax, qword ptr [rsi + 0xf8]
0x1408b74bd jae      0x1408b74f7
0x1408b74bf mov      rdx, qword ptr [rsi + 0xf0]
0x1408b74c6 lea      rax, [rax + rax*2]
0x1408b74ca shl      rax, 4
0x1408b74ce cmp      dword ptr [rdx + rax], 1
0x1408b74d2 jne      0x1408b74f7
0x1408b74d4 add      rdx, rax
0x1408b74d7 add      rdx, 8
0x1408b74db lea      rcx, [rbp - 0x58]
0x1408b74df mov      r8b, 1
0x1408b74e2 call     0x141437160 ; libfmod::EventInstance::set_paused::h32fbd681052381c4
0x1408b74e7 movabs   rax, 0x8000000000000004
0x1408b74f1 cmp      qword ptr [rbp - 0x58], rax
0x1408b74f5 jne      0x1408b7574
0x1408b74f7 cmp      dword ptr [rsi + 0x10], 1
0x1408b74fb jne      0x1408b7520
0x1408b74fd add      rsi, 0x18
0x1408b7501 lea      rcx, [rbp - 0x58]
0x1408b7505 mov      rdx, rsi
0x1408b7508 mov      r8b, 1
0x1408b750b call     0x141437160 ; libfmod::EventInstance::set_paused::h32fbd681052381c4
0x1408b7510 movabs   rax, 0x8000000000000004
0x1408b751a cmp      qword ptr [rbp - 0x58], rax
0x1408b751e jne      0x1408b752a
0x1408b7520 add      rsp, 0xa8
0x1408b7527 pop      rsi
0x1408b7528 pop      rbp
0x1408b7529 ret      
0x1408b752a mov      rax, qword ptr [rbp - 0x28]
0x1408b752e mov      qword ptr [rbp + 0x10], rax
0x1408b7532 movups   xmm0, xmmword ptr [rbp - 0x58]
0x1408b7536 movups   xmm1, xmmword ptr [rbp - 0x48]
0x1408b753a movups   xmm2, xmmword ptr [rbp - 0x38]
0x1408b753e movaps   xmmword ptr [rbp], xmm2
0x1408b7542 movaps   xmmword ptr [rbp - 0x10], xmm1
0x1408b7546 movaps   xmmword ptr [rbp - 0x20], xmm0
0x1408b754a lea      rax, [rip + 0x221b6e7]
0x1408b7551 mov      qword ptr [rsp + 0x20], rax
0x1408b7556 lea      rcx, [rip + 0x22190a3] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b755d lea      r9, [rip + 0x221905c]
0x1408b7564 lea      r8, [rbp - 0x20]
0x1408b7568 mov      edx, 0x2b
0x1408b756d call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b7572 jmp      0x1408b75bc ; 
0x1408b7574 mov      rax, qword ptr [rbp - 0x28]
0x1408b7578 mov      qword ptr [rbp + 0x10], rax
0x1408b757c movups   xmm0, xmmword ptr [rbp - 0x58]
0x1408b7580 movups   xmm1, xmmword ptr [rbp - 0x48]
0x1408b7584 movups   xmm2, xmmword ptr [rbp - 0x38]
0x1408b7588 movaps   xmmword ptr [rbp], xmm2
0x1408b758c movaps   xmmword ptr [rbp - 0x10], xmm1
0x1408b7590 movaps   xmmword ptr [rbp - 0x20], xmm0
0x1408b7594 lea      rax, [rip + 0x221b69d]
0x1408b759b mov      qword ptr [rsp + 0x20], rax
0x1408b75a0 lea      rcx, [rip + 0x2219059] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b75a7 lea      r9, [rip + 0x2219012]
0x1408b75ae lea      r8, [rbp - 0x20]
0x1408b75b2 mov      edx, 0x2b
0x1408b75b7 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b75bc ud2      
0x1408b75be nop      
0x1408b75c0 mov      qword ptr [rsp + 0x10], rdx
0x1408b75c5 push     rbp
0x1408b75c6 push     rsi
0x1408b75c7 sub      rsp, 0x28
0x1408b75cb lea      rbp, [rdx + 0x80]
0x1408b75d2 lea      rcx, [rbp - 0x20]
0x1408b75d6 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b75db nop      
0x1408b75dc add      rsp, 0x28
0x1408b75e0 pop      rsi
0x1408b75e1 pop      rbp
0x1408b75e2 ret      
0x1408b75e3 nop      word ptr cs:[rax + rax]
0x1408b75f0 mov      qword ptr [rsp + 0x10], rdx
0x1408b75f5 push     rbp
0x1408b75f6 push     rsi
0x1408b75f7 sub      rsp, 0x28
0x1408b75fb lea      rbp, [rdx + 0x80]
0x1408b7602 lea      rcx, [rbp - 0x20]
0x1408b7606 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b760b nop      
0x1408b760c add      rsp, 0x28
0x1408b7610 pop      rsi
0x1408b7611 pop      rbp
0x1408b7612 ret      