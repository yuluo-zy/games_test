0x140afab70 push     rsi
0x140afab71 push     rdi
0x140afab72 push     rbx
0x140afab73 sub      rsp, 0x30
0x140afab77 test     cl, cl
0x140afab79 je       0x140afabcc
0x140afab7b mov      eax, dword ptr [rdx + 0x1c]
0x140afab7e mov      rcx, qword ptr [rdx + 0x10]
0x140afab82 mov      dword ptr [rcx], eax
0x140afab84 mov      rsi, qword ptr [rdx]
0x140afab87 mov      rdi, qword ptr [rsi + 0x30]
0x140afab8b mov      rbx, qword ptr [rsi + 0x40]
0x140afab8f cmp      rdi, qword ptr [rsi + 0x20]
0x140afab93 jne      0x140afaba5
0x140afab95 lea      rcx, [rsi + 0x20]
0x140afab99 lea      rdx, [rip + 0x213f5a8]
0x140afaba0 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140afaba5 mov      rax, qword ptr [rsi + 0x28]
0x140afaba9 mov      rcx, rdi
0x140afabac shl      rcx, 5
0x140afabb0 mov      dword ptr [rax + rcx], 4
0x140afabb7 mov      qword ptr [rax + rcx + 0x18], rbx
0x140afabbc inc      rdi
0x140afabbf mov      qword ptr [rsi + 0x30], rdi
0x140afabc3 inc      qword ptr [rsi + 0x40]
0x140afabc7 jmp      0x140afac81 ; 
0x140afabcc mov      eax, dword ptr [r8 + 0x1c]
0x140afabd0 mov      rcx, qword ptr [r8 + 0x10]
0x140afabd4 mov      dword ptr [rcx], eax
0x140afabd6 mov      rcx, qword ptr [r8]
0x140afabd9 cmp      qword ptr [rcx + 0x10], 0
0x140afabde je       0x140afac81
0x140afabe4 mov      r8, qword ptr [rcx + 8]
0x140afabe8 mov      r10, qword ptr [r8 + 0x10]
0x140afabec test     r10, r10
0x140afabef je       0x140afac81
0x140afabf5 cmp      byte ptr [rcx + 0x18], 0
0x140afabf9 je       0x140afac16
0x140afabfb mov      rdx, qword ptr [r8 + 0x18]
0x140afabff cmp      rdx, r10
0x140afac02 jb       0x140afac39
0x140afac04 lea      r8, [rip + 0x1feaa2d]
0x140afac0b mov      rcx, rdx
0x140afac0e mov      rdx, r10
0x140afac11 call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x140afac16 mov      rax, qword ptr [r8 + 0x18]
0x140afac1a inc      rax
0x140afac1d mov      rdx, rax
0x140afac20 or       rdx, r10
0x140afac23 shr      rdx, 0x20
0x140afac27 je       0x140afac30
0x140afac29 xor      edx, edx
0x140afac2b div      r10
0x140afac2e jmp      0x140afac35 ; 
0x140afac30 xor      edx, edx
0x140afac32 div      r10d
0x140afac35 mov      qword ptr [r8 + 0x18], rdx
0x140afac39 mov      r8, qword ptr [r8 + 8]
0x140afac3d shl      rdx, 4
0x140afac41 mov      eax, dword ptr [r8 + rdx]
0x140afac45 mov      byte ptr [rcx + 0x18], 0
0x140afac49 cmp      eax, 0x15
0x140afac4c je       0x140afac81
0x140afac4e add      r8, rdx
0x140afac51 mov      ecx, dword ptr [r8 + 0xc]
0x140afac55 mov      dword ptr [rsp + 0x28], ecx
0x140afac59 mov      rcx, qword ptr [r8 + 4]
0x140afac5d mov      qword ptr [rsp + 0x20], rcx
0x140afac62 mov      ecx, dword ptr [r9 + 0x1c]
0x140afac66 mov      rdx, qword ptr [r9 + 0x10]
0x140afac6a mov      dword ptr [rdx], ecx
0x140afac6c mov      rcx, qword ptr [r9]
0x140afac6f mov      dword ptr [rcx], eax
0x140afac71 mov      rax, qword ptr [rsp + 0x20]
0x140afac76 mov      qword ptr [rcx + 4], rax
0x140afac7a mov      eax, dword ptr [rsp + 0x28]
0x140afac7e mov      dword ptr [rcx + 0xc], eax
0x140afac81 add      rsp, 0x30
0x140afac85 pop      rbx
0x140afac86 pop      rdi
0x140afac87 pop      rsi
0x140afac88 ret      