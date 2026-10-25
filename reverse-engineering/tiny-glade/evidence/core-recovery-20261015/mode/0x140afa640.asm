0x140afa640 push     rsi
0x140afa641 push     rdi
0x140afa642 push     rbx
0x140afa643 sub      rsp, 0x30
0x140afa647 test     cl, cl
0x140afa649 je       0x140afa69c
0x140afa64b mov      eax, dword ptr [rdx + 0x1c]
0x140afa64e mov      rcx, qword ptr [rdx + 0x10]
0x140afa652 mov      dword ptr [rcx], eax
0x140afa654 mov      rsi, qword ptr [rdx]
0x140afa657 mov      rdi, qword ptr [rsi + 0x30]
0x140afa65b mov      rbx, qword ptr [rsi + 0x40]
0x140afa65f cmp      rdi, qword ptr [rsi + 0x20]
0x140afa663 jne      0x140afa675
0x140afa665 lea      rcx, [rsi + 0x20]
0x140afa669 lea      rdx, [rip + 0x213fad8]
0x140afa670 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140afa675 mov      rax, qword ptr [rsi + 0x28]
0x140afa679 mov      rcx, rdi
0x140afa67c shl      rcx, 5
0x140afa680 mov      dword ptr [rax + rcx], 4
0x140afa687 mov      qword ptr [rax + rcx + 0x18], rbx
0x140afa68c inc      rdi
0x140afa68f mov      qword ptr [rsi + 0x30], rdi
0x140afa693 inc      qword ptr [rsi + 0x40]
0x140afa697 jmp      0x140afa760 ; 
0x140afa69c mov      eax, dword ptr [r8 + 0x1c]
0x140afa6a0 mov      rcx, qword ptr [r8 + 0x10]
0x140afa6a4 mov      dword ptr [rcx], eax
0x140afa6a6 mov      rcx, qword ptr [r8]
0x140afa6a9 cmp      qword ptr [rcx + 0x10], 0xa
0x140afa6ae jb       0x140afa760
0x140afa6b4 mov      r8, qword ptr [rcx + 8]
0x140afa6b8 mov      r10, qword ptr [r8 + 0x130]
0x140afa6bf test     r10, r10
0x140afa6c2 je       0x140afa760
0x140afa6c8 cmp      byte ptr [rcx + 0x18], 9
0x140afa6cc jne      0x140afa6eb
0x140afa6ce mov      rax, qword ptr [r8 + 0x138]
0x140afa6d5 inc      rax
0x140afa6d8 mov      rdx, rax
0x140afa6db or       rdx, r10
0x140afa6de shr      rdx, 0x20
0x140afa6e2 je       0x140afa709
0x140afa6e4 xor      edx, edx
0x140afa6e6 div      r10
0x140afa6e9 jmp      0x140afa70e ; 
0x140afa6eb mov      rdx, qword ptr [r8 + 0x138]
0x140afa6f2 cmp      rdx, r10
0x140afa6f5 jb       0x140afa715
0x140afa6f7 lea      r8, [rip + 0x1feaf3a]
0x140afa6fe mov      rcx, rdx
0x140afa701 mov      rdx, r10
0x140afa704 call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x140afa709 xor      edx, edx
0x140afa70b div      r10d
0x140afa70e mov      qword ptr [r8 + 0x138], rdx
0x140afa715 mov      r8, qword ptr [r8 + 0x128]
0x140afa71c shl      rdx, 4
0x140afa720 mov      eax, dword ptr [r8 + rdx]
0x140afa724 mov      byte ptr [rcx + 0x18], 9
0x140afa728 cmp      eax, 0x15
0x140afa72b je       0x140afa760
0x140afa72d add      r8, rdx
0x140afa730 mov      ecx, dword ptr [r8 + 0xc]
0x140afa734 mov      dword ptr [rsp + 0x28], ecx
0x140afa738 mov      rcx, qword ptr [r8 + 4]
0x140afa73c mov      qword ptr [rsp + 0x20], rcx
0x140afa741 mov      ecx, dword ptr [r9 + 0x1c]
0x140afa745 mov      rdx, qword ptr [r9 + 0x10]
0x140afa749 mov      dword ptr [rdx], ecx
0x140afa74b mov      rcx, qword ptr [r9]
0x140afa74e mov      dword ptr [rcx], eax
0x140afa750 mov      rax, qword ptr [rsp + 0x20]
0x140afa755 mov      qword ptr [rcx + 4], rax
0x140afa759 mov      eax, dword ptr [rsp + 0x28]
0x140afa75d mov      dword ptr [rcx + 0xc], eax
0x140afa760 add      rsp, 0x30
0x140afa764 pop      rbx
0x140afa765 pop      rdi
0x140afa766 pop      rsi
0x140afa767 ret      