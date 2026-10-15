0x140afa3f0 push     rsi
0x140afa3f1 push     rdi
0x140afa3f2 push     rbx
0x140afa3f3 sub      rsp, 0x30
0x140afa3f7 test     cl, cl
0x140afa3f9 je       0x140afa44c
0x140afa3fb mov      eax, dword ptr [rdx + 0x1c]
0x140afa3fe mov      rcx, qword ptr [rdx + 0x10]
0x140afa402 mov      dword ptr [rcx], eax
0x140afa404 mov      rsi, qword ptr [rdx]
0x140afa407 mov      rdi, qword ptr [rsi + 0x30]
0x140afa40b mov      rbx, qword ptr [rsi + 0x40]
0x140afa40f cmp      rdi, qword ptr [rsi + 0x20]
0x140afa413 jne      0x140afa425
0x140afa415 lea      rcx, [rsi + 0x20]
0x140afa419 lea      rdx, [rip + 0x213fd28]
0x140afa420 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140afa425 mov      rax, qword ptr [rsi + 0x28]
0x140afa429 mov      rcx, rdi
0x140afa42c shl      rcx, 5
0x140afa430 mov      dword ptr [rax + rcx], 4
0x140afa437 mov      qword ptr [rax + rcx + 0x18], rbx
0x140afa43c inc      rdi
0x140afa43f mov      qword ptr [rsi + 0x30], rdi
0x140afa443 inc      qword ptr [rsi + 0x40]
0x140afa447 jmp      0x140afa501 ; 
0x140afa44c mov      eax, dword ptr [r8 + 0x1c]
0x140afa450 mov      rcx, qword ptr [r8 + 0x10]
0x140afa454 mov      dword ptr [rcx], eax
0x140afa456 mov      rcx, qword ptr [r8]
0x140afa459 cmp      qword ptr [rcx + 0x10], 2
0x140afa45e jb       0x140afa501
0x140afa464 mov      r8, qword ptr [rcx + 8]
0x140afa468 mov      r10, qword ptr [r8 + 0x30]
0x140afa46c test     r10, r10
0x140afa46f je       0x140afa501
0x140afa475 cmp      byte ptr [rcx + 0x18], 1
0x140afa479 jne      0x140afa495
0x140afa47b mov      rax, qword ptr [r8 + 0x38]
0x140afa47f inc      rax
0x140afa482 mov      rdx, rax
0x140afa485 or       rdx, r10
0x140afa488 shr      rdx, 0x20
0x140afa48c je       0x140afa4b0
0x140afa48e xor      edx, edx
0x140afa490 div      r10
0x140afa493 jmp      0x140afa4b5 ; 
0x140afa495 mov      rdx, qword ptr [r8 + 0x38]
0x140afa499 cmp      rdx, r10
0x140afa49c jb       0x140afa4b9
0x140afa49e lea      r8, [rip + 0x1feb193]
0x140afa4a5 mov      rcx, rdx
0x140afa4a8 mov      rdx, r10
0x140afa4ab call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x140afa4b0 xor      edx, edx
0x140afa4b2 div      r10d
0x140afa4b5 mov      qword ptr [r8 + 0x38], rdx
0x140afa4b9 mov      r8, qword ptr [r8 + 0x28]
0x140afa4bd shl      rdx, 4
0x140afa4c1 mov      eax, dword ptr [r8 + rdx]
0x140afa4c5 mov      byte ptr [rcx + 0x18], 1
0x140afa4c9 cmp      eax, 0x15
0x140afa4cc je       0x140afa501
0x140afa4ce add      r8, rdx
0x140afa4d1 mov      ecx, dword ptr [r8 + 0xc]
0x140afa4d5 mov      dword ptr [rsp + 0x28], ecx
0x140afa4d9 mov      rcx, qword ptr [r8 + 4]
0x140afa4dd mov      qword ptr [rsp + 0x20], rcx
0x140afa4e2 mov      ecx, dword ptr [r9 + 0x1c]
0x140afa4e6 mov      rdx, qword ptr [r9 + 0x10]
0x140afa4ea mov      dword ptr [rdx], ecx
0x140afa4ec mov      rcx, qword ptr [r9]
0x140afa4ef mov      dword ptr [rcx], eax
0x140afa4f1 mov      rax, qword ptr [rsp + 0x20]
0x140afa4f6 mov      qword ptr [rcx + 4], rax
0x140afa4fa mov      eax, dword ptr [rsp + 0x28]
0x140afa4fe mov      dword ptr [rcx + 0xc], eax
0x140afa501 add      rsp, 0x30
0x140afa505 pop      rbx
0x140afa506 pop      rdi
0x140afa507 pop      rsi
0x140afa508 ret      