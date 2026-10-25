0x1408b72e0 push     rbp
0x1408b72e1 push     r15
0x1408b72e3 push     r14
0x1408b72e5 push     rsi
0x1408b72e6 push     rdi
0x1408b72e7 push     rbx
0x1408b72e8 sub      rsp, 0xa8
0x1408b72ef lea      rbp, [rsp + 0x80]
0x1408b72f7 mov      qword ptr [rbp + 0x20], 0xfffffffffffffffe
0x1408b72ff mov      rsi, rcx
0x1408b7302 mov      rax, qword ptr [rcx + 0xf8]
0x1408b7309 movabs   r14, 0x8000000000000004
0x1408b7313 test     rax, rax
0x1408b7316 je       0x1408b73ae
0x1408b731c mov      rdi, qword ptr [rsi + 0xf0]
0x1408b7323 add      rdi, 8
0x1408b7327 shl      rax, 4
0x1408b732b lea      r15, [rax + rax*2]
0x1408b732f lea      rbx, [rbp - 0x58]
0x1408b7333 jmp      0x1408b734a ; 
0x1408b7335 nop      word ptr cs:[rax + rax]
0x1408b7340 add      rdi, 0x30
0x1408b7344 add      r15, -0x30
0x1408b7348 je       0x1408b73ae
0x1408b734a cmp      dword ptr [rdi - 8], 1
0x1408b734e jne      0x1408b7340
0x1408b7350 mov      rcx, rbx
0x1408b7353 mov      rdx, rdi
0x1408b7356 mov      r8b, 1
0x1408b7359 call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x1408b735e cmp      qword ptr [rbp - 0x58], r14
0x1408b7362 je       0x1408b7340
0x1408b7364 mov      rax, qword ptr [rbp - 0x28]
0x1408b7368 mov      qword ptr [rbp + 0x10], rax
0x1408b736c movups   xmm0, xmmword ptr [rbp - 0x58]
0x1408b7370 movups   xmm1, xmmword ptr [rbp - 0x48]
0x1408b7374 movups   xmm2, xmmword ptr [rbp - 0x38]
0x1408b7378 movaps   xmmword ptr [rbp], xmm2
0x1408b737c movaps   xmmword ptr [rbp - 0x10], xmm1
0x1408b7380 movaps   xmmword ptr [rbp - 0x20], xmm0
0x1408b7384 lea      rax, [rip + 0x221b895]
0x1408b738b mov      qword ptr [rsp + 0x20], rax
0x1408b7390 lea      rcx, [rip + 0x2219269] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b7397 lea      r9, [rip + 0x2219222]
0x1408b739e lea      r8, [rbp - 0x20]
0x1408b73a2 mov      edx, 0x2b
0x1408b73a7 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b73ac jmp      0x1408b742c ; 
0x1408b73ae mov      qword ptr [rsi], 0
0x1408b73b5 cmp      dword ptr [rsi + 0x10], 1
0x1408b73b9 jne      0x1408b73d4
0x1408b73bb add      rsi, 0x18
0x1408b73bf lea      rcx, [rbp - 0x58]
0x1408b73c3 mov      rdx, rsi
0x1408b73c6 mov      r8b, 1
0x1408b73c9 call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x1408b73ce cmp      qword ptr [rbp - 0x58], r14
0x1408b73d2 jne      0x1408b73e4
0x1408b73d4 add      rsp, 0xa8
0x1408b73db pop      rbx
0x1408b73dc pop      rdi
0x1408b73dd pop      rsi
0x1408b73de pop      r14
0x1408b73e0 pop      r15
0x1408b73e2 pop      rbp
0x1408b73e3 ret      
0x1408b73e4 mov      rax, qword ptr [rbp - 0x28]
0x1408b73e8 mov      qword ptr [rbp + 0x10], rax
0x1408b73ec movups   xmm0, xmmword ptr [rbp - 0x58]
0x1408b73f0 movups   xmm1, xmmword ptr [rbp - 0x48]
0x1408b73f4 movups   xmm2, xmmword ptr [rbp - 0x38]
0x1408b73f8 movaps   xmmword ptr [rbp], xmm2
0x1408b73fc movaps   xmmword ptr [rbp - 0x10], xmm1
0x1408b7400 movaps   xmmword ptr [rbp - 0x20], xmm0
0x1408b7404 lea      rax, [rip + 0x221b815]
0x1408b740b mov      qword ptr [rsp + 0x20], rax
0x1408b7410 lea      rcx, [rip + 0x22191e9] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b7417 lea      r9, [rip + 0x22191a2]
0x1408b741e lea      r8, [rbp - 0x20]
0x1408b7422 mov      edx, 0x2b
0x1408b7427 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b742c ud2      
0x1408b742e nop      
0x1408b7430 mov      qword ptr [rsp + 0x10], rdx
0x1408b7435 push     rbp
0x1408b7436 push     r15
0x1408b7438 push     r14
0x1408b743a push     rsi
0x1408b743b push     rdi
0x1408b743c push     rbx
0x1408b743d sub      rsp, 0x28
0x1408b7441 lea      rbp, [rdx + 0x80]
0x1408b7448 lea      rcx, [rbp - 0x20]
0x1408b744c call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b7451 nop      
0x1408b7452 add      rsp, 0x28
0x1408b7456 pop      rbx
0x1408b7457 pop      rdi
0x1408b7458 pop      rsi
0x1408b7459 pop      r14
0x1408b745b pop      r15
0x1408b745d pop      rbp
0x1408b745e ret      
0x1408b745f nop      
0x1408b7460 mov      qword ptr [rsp + 0x10], rdx
0x1408b7465 push     rbp
0x1408b7466 push     r15
0x1408b7468 push     r14
0x1408b746a push     rsi
0x1408b746b push     rdi
0x1408b746c push     rbx
0x1408b746d sub      rsp, 0x28
0x1408b7471 lea      rbp, [rdx + 0x80]
0x1408b7478 lea      rcx, [rbp - 0x20]
0x1408b747c call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b7481 nop      
0x1408b7482 add      rsp, 0x28
0x1408b7486 pop      rbx
0x1408b7487 pop      rdi
0x1408b7488 pop      rsi
0x1408b7489 pop      r14
0x1408b748b pop      r15
0x1408b748d pop      rbp
0x1408b748e ret      