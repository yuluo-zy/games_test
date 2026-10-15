0x140afac90 push     rsi
0x140afac91 push     rdi
0x140afac92 push     rbx
0x140afac93 sub      rsp, 0x30
0x140afac97 test     cl, cl
0x140afac99 je       0x140afacec
0x140afac9b mov      eax, dword ptr [rdx + 0x1c]
0x140afac9e mov      rcx, qword ptr [rdx + 0x10]
0x140afaca2 mov      dword ptr [rcx], eax
0x140afaca4 mov      rsi, qword ptr [rdx]
0x140afaca7 mov      rdi, qword ptr [rsi + 0x30]
0x140afacab mov      rbx, qword ptr [rsi + 0x40]
0x140afacaf cmp      rdi, qword ptr [rsi + 0x20]
0x140afacb3 jne      0x140afacc5
0x140afacb5 lea      rcx, [rsi + 0x20]
0x140afacb9 lea      rdx, [rip + 0x213f488]
0x140afacc0 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140afacc5 mov      rax, qword ptr [rsi + 0x28]
0x140afacc9 mov      rcx, rdi
0x140afaccc shl      rcx, 5
0x140afacd0 mov      dword ptr [rax + rcx], 4
0x140afacd7 mov      qword ptr [rax + rcx + 0x18], rbx
0x140afacdc inc      rdi
0x140afacdf mov      qword ptr [rsi + 0x30], rdi
0x140aface3 inc      qword ptr [rsi + 0x40]
0x140aface7 jmp      0x140afada1 ; 
0x140afacec mov      eax, dword ptr [r8 + 0x1c]
0x140afacf0 mov      rcx, qword ptr [r8 + 0x10]
0x140afacf4 mov      dword ptr [rcx], eax
0x140afacf6 mov      rcx, qword ptr [r8]
0x140afacf9 cmp      qword ptr [rcx + 0x10], 4
0x140afacfe jb       0x140afada1
0x140afad04 mov      r8, qword ptr [rcx + 8]
0x140afad08 mov      r10, qword ptr [r8 + 0x70]
0x140afad0c test     r10, r10
0x140afad0f je       0x140afada1
0x140afad15 cmp      byte ptr [rcx + 0x18], 3
0x140afad19 jne      0x140afad35
0x140afad1b mov      rax, qword ptr [r8 + 0x78]
0x140afad1f inc      rax
0x140afad22 mov      rdx, rax
0x140afad25 or       rdx, r10
0x140afad28 shr      rdx, 0x20
0x140afad2c je       0x140afad50
0x140afad2e xor      edx, edx
0x140afad30 div      r10
0x140afad33 jmp      0x140afad55 ; 
0x140afad35 mov      rdx, qword ptr [r8 + 0x78]
0x140afad39 cmp      rdx, r10
0x140afad3c jb       0x140afad59
0x140afad3e lea      r8, [rip + 0x1fea8f3]
0x140afad45 mov      rcx, rdx
0x140afad48 mov      rdx, r10
0x140afad4b call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x140afad50 xor      edx, edx
0x140afad52 div      r10d
0x140afad55 mov      qword ptr [r8 + 0x78], rdx
0x140afad59 mov      r8, qword ptr [r8 + 0x68]
0x140afad5d shl      rdx, 4
0x140afad61 mov      eax, dword ptr [r8 + rdx]
0x140afad65 mov      byte ptr [rcx + 0x18], 3
0x140afad69 cmp      eax, 0x15
0x140afad6c je       0x140afada1
0x140afad6e add      r8, rdx
0x140afad71 mov      ecx, dword ptr [r8 + 0xc]
0x140afad75 mov      dword ptr [rsp + 0x28], ecx
0x140afad79 mov      rcx, qword ptr [r8 + 4]
0x140afad7d mov      qword ptr [rsp + 0x20], rcx
0x140afad82 mov      ecx, dword ptr [r9 + 0x1c]
0x140afad86 mov      rdx, qword ptr [r9 + 0x10]
0x140afad8a mov      dword ptr [rdx], ecx
0x140afad8c mov      rcx, qword ptr [r9]
0x140afad8f mov      dword ptr [rcx], eax
0x140afad91 mov      rax, qword ptr [rsp + 0x20]
0x140afad96 mov      qword ptr [rcx + 4], rax
0x140afad9a mov      eax, dword ptr [rsp + 0x28]
0x140afad9e mov      dword ptr [rcx + 0xc], eax
0x140afada1 add      rsp, 0x30
0x140afada5 pop      rbx
0x140afada6 pop      rdi
0x140afada7 pop      rsi
0x140afada8 ret      