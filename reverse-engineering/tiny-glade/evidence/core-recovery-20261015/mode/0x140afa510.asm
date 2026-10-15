0x140afa510 push     rsi
0x140afa511 push     rdi
0x140afa512 push     rbx
0x140afa513 sub      rsp, 0x30
0x140afa517 test     cl, cl
0x140afa519 je       0x140afa56c
0x140afa51b mov      eax, dword ptr [rdx + 0x1c]
0x140afa51e mov      rcx, qword ptr [rdx + 0x10]
0x140afa522 mov      dword ptr [rcx], eax
0x140afa524 mov      rsi, qword ptr [rdx]
0x140afa527 mov      rdi, qword ptr [rsi + 0x30]
0x140afa52b mov      rbx, qword ptr [rsi + 0x40]
0x140afa52f cmp      rdi, qword ptr [rsi + 0x20]
0x140afa533 jne      0x140afa545
0x140afa535 lea      rcx, [rsi + 0x20]
0x140afa539 lea      rdx, [rip + 0x213fc08]
0x140afa540 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140afa545 mov      rax, qword ptr [rsi + 0x28]
0x140afa549 mov      rcx, rdi
0x140afa54c shl      rcx, 5
0x140afa550 mov      dword ptr [rax + rcx], 4
0x140afa557 mov      qword ptr [rax + rcx + 0x18], rbx
0x140afa55c inc      rdi
0x140afa55f mov      qword ptr [rsi + 0x30], rdi
0x140afa563 inc      qword ptr [rsi + 0x40]
0x140afa567 jmp      0x140afa630 ; 
0x140afa56c mov      eax, dword ptr [r8 + 0x1c]
0x140afa570 mov      rcx, qword ptr [r8 + 0x10]
0x140afa574 mov      dword ptr [rcx], eax
0x140afa576 mov      rcx, qword ptr [r8]
0x140afa579 cmp      qword ptr [rcx + 0x10], 6
0x140afa57e jb       0x140afa630
0x140afa584 mov      r8, qword ptr [rcx + 8]
0x140afa588 mov      r10, qword ptr [r8 + 0xb0]
0x140afa58f test     r10, r10
0x140afa592 je       0x140afa630
0x140afa598 cmp      byte ptr [rcx + 0x18], 5
0x140afa59c jne      0x140afa5bb
0x140afa59e mov      rax, qword ptr [r8 + 0xb8]
0x140afa5a5 inc      rax
0x140afa5a8 mov      rdx, rax
0x140afa5ab or       rdx, r10
0x140afa5ae shr      rdx, 0x20
0x140afa5b2 je       0x140afa5d9
0x140afa5b4 xor      edx, edx
0x140afa5b6 div      r10
0x140afa5b9 jmp      0x140afa5de ; 
0x140afa5bb mov      rdx, qword ptr [r8 + 0xb8]
0x140afa5c2 cmp      rdx, r10
0x140afa5c5 jb       0x140afa5e5
0x140afa5c7 lea      r8, [rip + 0x1feb06a]
0x140afa5ce mov      rcx, rdx
0x140afa5d1 mov      rdx, r10
0x140afa5d4 call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x140afa5d9 xor      edx, edx
0x140afa5db div      r10d
0x140afa5de mov      qword ptr [r8 + 0xb8], rdx
0x140afa5e5 mov      r8, qword ptr [r8 + 0xa8]
0x140afa5ec shl      rdx, 4
0x140afa5f0 mov      eax, dword ptr [r8 + rdx]
0x140afa5f4 mov      byte ptr [rcx + 0x18], 5
0x140afa5f8 cmp      eax, 0x15
0x140afa5fb je       0x140afa630
0x140afa5fd add      r8, rdx
0x140afa600 mov      ecx, dword ptr [r8 + 0xc]
0x140afa604 mov      dword ptr [rsp + 0x28], ecx
0x140afa608 mov      rcx, qword ptr [r8 + 4]
0x140afa60c mov      qword ptr [rsp + 0x20], rcx
0x140afa611 mov      ecx, dword ptr [r9 + 0x1c]
0x140afa615 mov      rdx, qword ptr [r9 + 0x10]
0x140afa619 mov      dword ptr [rdx], ecx
0x140afa61b mov      rcx, qword ptr [r9]
0x140afa61e mov      dword ptr [rcx], eax
0x140afa620 mov      rax, qword ptr [rsp + 0x20]
0x140afa625 mov      qword ptr [rcx + 4], rax
0x140afa629 mov      eax, dword ptr [rsp + 0x28]
0x140afa62d mov      dword ptr [rcx + 0xc], eax
0x140afa630 add      rsp, 0x30
0x140afa634 pop      rbx
0x140afa635 pop      rdi
0x140afa636 pop      rsi
0x140afa637 ret      