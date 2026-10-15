0x140a57470 push     rbp
0x140a57471 sub      rsp, 0xa0
0x140a57478 lea      rbp, [rsp + 0x80]
0x140a57480 mov      qword ptr [rbp + 0x18], 0xfffffffffffffffe
0x140a57488 cmp      dword ptr [rcx], 1
0x140a5748b jne      0x140a574b0
0x140a5748d mov      rdx, rcx
0x140a57490 add      rdx, 8
0x140a57494 lea      rcx, [rbp - 0x58]
0x140a57498 xor      r8d, r8d
0x140a5749b call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x140a574a0 movabs   rax, 0x8000000000000004
0x140a574aa cmp      qword ptr [rbp - 0x58], rax
0x140a574ae jne      0x140a574b9
0x140a574b0 add      rsp, 0xa0
0x140a574b7 pop      rbp
0x140a574b8 ret      
0x140a574b9 mov      rax, qword ptr [rbp - 0x28]
0x140a574bd mov      qword ptr [rbp + 0x10], rax
0x140a574c1 movups   xmm0, xmmword ptr [rbp - 0x58]
0x140a574c5 movups   xmm1, xmmword ptr [rbp - 0x48]
0x140a574c9 movups   xmm2, xmmword ptr [rbp - 0x38]
0x140a574cd movaps   xmmword ptr [rbp], xmm2
0x140a574d1 movaps   xmmword ptr [rbp - 0x10], xmm1
0x140a574d5 movaps   xmmword ptr [rbp - 0x20], xmm0
0x140a574d9 lea      rax, [rip + 0x207b728]
0x140a574e0 mov      qword ptr [rsp + 0x20], rax
0x140a574e5 lea      rcx, [rip + 0x2079114] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x140a574ec lea      r9, [rip + 0x20790cd]
0x140a574f3 lea      r8, [rbp - 0x20]
0x140a574f7 mov      edx, 0x2b
0x140a574fc call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a57501 ud2      
0x140a57503 nop      word ptr cs:[rax + rax]
0x140a57510 mov      qword ptr [rsp + 0x10], rdx
0x140a57515 push     rbp
0x140a57516 sub      rsp, 0x30
0x140a5751a lea      rbp, [rdx + 0x80]
0x140a57521 lea      rcx, [rbp - 0x20]
0x140a57525 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a5752a nop      
0x140a5752b add      rsp, 0x30
0x140a5752f pop      rbp
0x140a57530 ret      