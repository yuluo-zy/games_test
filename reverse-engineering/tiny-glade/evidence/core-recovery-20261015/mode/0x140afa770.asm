0x140afa770 push     rsi
0x140afa771 push     rdi
0x140afa772 push     rbx
0x140afa773 sub      rsp, 0x30
0x140afa777 test     cl, cl
0x140afa779 je       0x140afa7cc
0x140afa77b mov      eax, dword ptr [rdx + 0x1c]
0x140afa77e mov      rcx, qword ptr [rdx + 0x10]
0x140afa782 mov      dword ptr [rcx], eax
0x140afa784 mov      rsi, qword ptr [rdx]
0x140afa787 mov      rdi, qword ptr [rsi + 0x30]
0x140afa78b mov      rbx, qword ptr [rsi + 0x40]
0x140afa78f cmp      rdi, qword ptr [rsi + 0x20]
0x140afa793 jne      0x140afa7a5
0x140afa795 lea      rcx, [rsi + 0x20]
0x140afa799 lea      rdx, [rip + 0x213f9a8]
0x140afa7a0 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140afa7a5 mov      rax, qword ptr [rsi + 0x28]
0x140afa7a9 mov      rcx, rdi
0x140afa7ac shl      rcx, 5
0x140afa7b0 mov      dword ptr [rax + rcx], 4
0x140afa7b7 mov      qword ptr [rax + rcx + 0x18], rbx
0x140afa7bc inc      rdi
0x140afa7bf mov      qword ptr [rsi + 0x30], rdi
0x140afa7c3 inc      qword ptr [rsi + 0x40]
0x140afa7c7 jmp      0x140afa897 ; 
0x140afa7cc mov      rax, qword ptr [r8]
0x140afa7cf mov      rdx, qword ptr [r8 + 8]
0x140afa7d3 mov      rcx, qword ptr [r8 + 0x10]
0x140afa7d7 mov      r8d, dword ptr [rax + 0x1c]
0x140afa7db mov      r9, qword ptr [rax + 0x10]
0x140afa7df mov      dword ptr [r9], r8d
0x140afa7e2 mov      r8, qword ptr [rax]
0x140afa7e5 movzx    r10d, byte ptr [rdx]
0x140afa7e9 cmp      qword ptr [r8 + 0x10], r10
0x140afa7ed jbe      0x140afa897
0x140afa7f3 mov      r11d, r10d
0x140afa7f6 shl      r11d, 5
0x140afa7fa add      r11, qword ptr [r8 + 8]
0x140afa7fe mov      r9, qword ptr [r11 + 0x10]
0x140afa802 test     r9, r9
0x140afa805 je       0x140afa897
0x140afa80b cmp      byte ptr [r8 + 0x18], r10b
0x140afa80f jne      0x140afa82b
0x140afa811 mov      rax, qword ptr [r11 + 0x18]
0x140afa815 inc      rax
0x140afa818 mov      rdx, rax
0x140afa81b or       rdx, r9
0x140afa81e shr      rdx, 0x20
0x140afa822 je       0x140afa846
0x140afa824 xor      edx, edx
0x140afa826 div      r9
0x140afa829 jmp      0x140afa84b ; 
0x140afa82b mov      rdx, qword ptr [r11 + 0x18]
0x140afa82f cmp      rdx, r9
0x140afa832 jb       0x140afa84f
0x140afa834 lea      r8, [rip + 0x1feadfd]
0x140afa83b mov      rcx, rdx
0x140afa83e mov      rdx, r9
0x140afa841 call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x140afa846 xor      edx, edx
0x140afa848 div      r9d
0x140afa84b mov      qword ptr [r11 + 0x18], rdx
0x140afa84f mov      r9, qword ptr [r11 + 8]
0x140afa853 shl      rdx, 4
0x140afa857 mov      eax, dword ptr [r9 + rdx]
0x140afa85b mov      byte ptr [r8 + 0x18], r10b
0x140afa85f cmp      eax, 0x15
0x140afa862 je       0x140afa897
0x140afa864 add      r9, rdx
0x140afa867 mov      edx, dword ptr [r9 + 0xc]
0x140afa86b mov      dword ptr [rsp + 0x28], edx
0x140afa86f mov      rdx, qword ptr [r9 + 4]
0x140afa873 mov      qword ptr [rsp + 0x20], rdx
0x140afa878 mov      edx, dword ptr [rcx + 0x1c]
0x140afa87b mov      r8, qword ptr [rcx + 0x10]
0x140afa87f mov      dword ptr [r8], edx
0x140afa882 mov      rcx, qword ptr [rcx]
0x140afa885 mov      dword ptr [rcx], eax
0x140afa887 mov      rax, qword ptr [rsp + 0x20]
0x140afa88c mov      qword ptr [rcx + 4], rax
0x140afa890 mov      eax, dword ptr [rsp + 0x28]
0x140afa894 mov      dword ptr [rcx + 0xc], eax
0x140afa897 add      rsp, 0x30
0x140afa89b pop      rbx
0x140afa89c pop      rdi
0x140afa89d pop      rsi
0x140afa89e ret      