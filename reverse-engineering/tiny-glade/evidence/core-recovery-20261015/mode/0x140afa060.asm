0x140afa060 push     rsi
0x140afa061 push     rdi
0x140afa062 push     rbx
0x140afa063 sub      rsp, 0x30
0x140afa067 test     cl, cl
0x140afa069 je       0x140afa0bc
0x140afa06b mov      eax, dword ptr [rdx + 0x1c]
0x140afa06e mov      rcx, qword ptr [rdx + 0x10]
0x140afa072 mov      dword ptr [rcx], eax
0x140afa074 mov      rsi, qword ptr [rdx]
0x140afa077 mov      rdi, qword ptr [rsi + 0x30]
0x140afa07b mov      rbx, qword ptr [rsi + 0x40]
0x140afa07f cmp      rdi, qword ptr [rsi + 0x20]
0x140afa083 jne      0x140afa095
0x140afa085 lea      rcx, [rsi + 0x20]
0x140afa089 lea      rdx, [rip + 0x21400b8]
0x140afa090 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140afa095 mov      rax, qword ptr [rsi + 0x28]
0x140afa099 mov      rcx, rdi
0x140afa09c shl      rcx, 5
0x140afa0a0 mov      dword ptr [rax + rcx], 4
0x140afa0a7 mov      qword ptr [rax + rcx + 0x18], rbx
0x140afa0ac inc      rdi
0x140afa0af mov      qword ptr [rsi + 0x30], rdi
0x140afa0b3 inc      qword ptr [rsi + 0x40]
0x140afa0b7 jmp      0x140afa180 ; 
0x140afa0bc mov      eax, dword ptr [r8 + 0x1c]
0x140afa0c0 mov      rcx, qword ptr [r8 + 0x10]
0x140afa0c4 mov      dword ptr [rcx], eax
0x140afa0c6 mov      rcx, qword ptr [r8]
0x140afa0c9 cmp      qword ptr [rcx + 0x10], 9
0x140afa0ce jb       0x140afa180
0x140afa0d4 mov      r8, qword ptr [rcx + 8]
0x140afa0d8 mov      r10, qword ptr [r8 + 0x110]
0x140afa0df test     r10, r10
0x140afa0e2 je       0x140afa180
0x140afa0e8 cmp      byte ptr [rcx + 0x18], 8
0x140afa0ec jne      0x140afa10b
0x140afa0ee mov      rax, qword ptr [r8 + 0x118]
0x140afa0f5 inc      rax
0x140afa0f8 mov      rdx, rax
0x140afa0fb or       rdx, r10
0x140afa0fe shr      rdx, 0x20
0x140afa102 je       0x140afa129
0x140afa104 xor      edx, edx
0x140afa106 div      r10
0x140afa109 jmp      0x140afa12e ; 
0x140afa10b mov      rdx, qword ptr [r8 + 0x118]
0x140afa112 cmp      rdx, r10
0x140afa115 jb       0x140afa135
0x140afa117 lea      r8, [rip + 0x1feb51a]
0x140afa11e mov      rcx, rdx
0x140afa121 mov      rdx, r10
0x140afa124 call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x140afa129 xor      edx, edx
0x140afa12b div      r10d
0x140afa12e mov      qword ptr [r8 + 0x118], rdx
0x140afa135 mov      r8, qword ptr [r8 + 0x108]
0x140afa13c shl      rdx, 4
0x140afa140 mov      eax, dword ptr [r8 + rdx]
0x140afa144 mov      byte ptr [rcx + 0x18], 8
0x140afa148 cmp      eax, 0x15
0x140afa14b je       0x140afa180
0x140afa14d add      r8, rdx
0x140afa150 mov      ecx, dword ptr [r8 + 0xc]
0x140afa154 mov      dword ptr [rsp + 0x28], ecx
0x140afa158 mov      rcx, qword ptr [r8 + 4]
0x140afa15c mov      qword ptr [rsp + 0x20], rcx
0x140afa161 mov      ecx, dword ptr [r9 + 0x1c]
0x140afa165 mov      rdx, qword ptr [r9 + 0x10]
0x140afa169 mov      dword ptr [rdx], ecx
0x140afa16b mov      rcx, qword ptr [r9]
0x140afa16e mov      dword ptr [rcx], eax
0x140afa170 mov      rax, qword ptr [rsp + 0x20]
0x140afa175 mov      qword ptr [rcx + 4], rax
0x140afa179 mov      eax, dword ptr [rsp + 0x28]
0x140afa17d mov      dword ptr [rcx + 0xc], eax
0x140afa180 add      rsp, 0x30
0x140afa184 pop      rbx
0x140afa185 pop      rdi
0x140afa186 pop      rsi
0x140afa187 ret      