0x140afa8a0 push     r15
0x140afa8a2 push     r14
0x140afa8a4 push     r12
0x140afa8a6 push     rsi
0x140afa8a7 push     rdi
0x140afa8a8 push     rbx
0x140afa8a9 sub      rsp, 0x28
0x140afa8ad test     cl, cl
0x140afa8af je       0x140afa8ff
0x140afa8b1 mov      eax, dword ptr [rdx + 0x1c]
0x140afa8b4 mov      rcx, qword ptr [rdx + 0x10]
0x140afa8b8 mov      dword ptr [rcx], eax
0x140afa8ba mov      rsi, qword ptr [rdx]
0x140afa8bd mov      rdi, qword ptr [rsi + 0x30]
0x140afa8c1 mov      rbx, qword ptr [rsi + 0x40]
0x140afa8c5 cmp      rdi, qword ptr [rsi + 0x20]
0x140afa8c9 jne      0x140afa8db
0x140afa8cb lea      rcx, [rsi + 0x20]
0x140afa8cf lea      rdx, [rip + 0x213f872]
0x140afa8d6 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140afa8db lea      rax, [rsi + 0x40]
0x140afa8df mov      rcx, qword ptr [rsi + 0x28]
0x140afa8e3 add      rsi, 0x30
0x140afa8e7 mov      rdx, rdi
0x140afa8ea shl      rdx, 5
0x140afa8ee mov      dword ptr [rcx + rdx], 4
0x140afa8f5 mov      qword ptr [rcx + rdx + 0x18], rbx
0x140afa8fa jmp      0x140afa9b9 ; 
0x140afa8ff mov      rax, qword ptr [r8]
0x140afa902 mov      rcx, qword ptr [r8 + 8]
0x140afa906 mov      rdi, qword ptr [r8 + 0x10]
0x140afa90a mov      rsi, qword ptr [r8 + 0x18]
0x140afa90e mov      edx, dword ptr [rax + 0x1c]
0x140afa911 mov      r8, qword ptr [rax + 0x10]
0x140afa915 mov      dword ptr [r8], edx
0x140afa918 mov      rax, qword ptr [rax]
0x140afa91b mov      dword ptr [rax], 0xd
0x140afa921 mov      ebx, dword ptr [rdi]
0x140afa923 mov      eax, dword ptr [rcx + 0x1c]
0x140afa926 mov      rdx, qword ptr [rcx + 0x10]
0x140afa92a mov      dword ptr [rdx], eax
0x140afa92c mov      r14, qword ptr [rcx]
0x140afa92f mov      r15, qword ptr [r14 + 0x30]
0x140afa933 mov      r12, qword ptr [r14 + 0x40]
0x140afa937 cmp      r15, qword ptr [r14 + 0x20]
0x140afa93b jne      0x140afa94d
0x140afa93d lea      rcx, [r14 + 0x20]
0x140afa941 lea      rdx, [rip + 0x213f800]
0x140afa948 call     0x140562630 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::habac6ea2a1af3a81 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h3d92912b2ef5335f
0x140afa94d mov      rax, qword ptr [r14 + 0x28]
0x140afa951 mov      rcx, r15
0x140afa954 shl      rcx, 4
0x140afa958 mov      qword ptr [rax + rcx], r12
0x140afa95c mov      dword ptr [rax + rcx + 8], ebx
0x140afa960 inc      r15
0x140afa963 mov      qword ptr [r14 + 0x30], r15
0x140afa967 inc      qword ptr [r14 + 0x40]
0x140afa96b movzx    ebx, byte ptr [rdi + 1]
0x140afa96f cmp      bl, 2
0x140afa972 jae      0x140afa9d0
0x140afa974 mov      eax, dword ptr [rsi + 0x1c]
0x140afa977 mov      rcx, qword ptr [rsi + 0x10]
0x140afa97b mov      dword ptr [rcx], eax
0x140afa97d mov      rsi, qword ptr [rsi]
0x140afa980 mov      rdi, qword ptr [rsi + 0x30]
0x140afa984 mov      r14, qword ptr [rsi + 0x40]
0x140afa988 cmp      rdi, qword ptr [rsi + 0x20]
0x140afa98c jne      0x140afa99e
0x140afa98e lea      rcx, [rsi + 0x20]
0x140afa992 lea      rdx, [rip + 0x213f7af]
0x140afa999 call     0x140562630 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::habac6ea2a1af3a81 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h3d92912b2ef5335f
0x140afa99e lea      rax, [rsi + 0x40]
0x140afa9a2 mov      rcx, qword ptr [rsi + 0x28]
0x140afa9a6 add      rsi, 0x30
0x140afa9aa mov      rdx, rdi
0x140afa9ad shl      rdx, 4
0x140afa9b1 mov      qword ptr [rcx + rdx], r14
0x140afa9b5 mov      byte ptr [rcx + rdx + 8], bl
0x140afa9b9 inc      rdi
0x140afa9bc mov      qword ptr [rsi], rdi
0x140afa9bf inc      qword ptr [rax]
0x140afa9c2 add      rsp, 0x28
0x140afa9c6 pop      rbx
0x140afa9c7 pop      rdi
0x140afa9c8 pop      rsi
0x140afa9c9 pop      r12
0x140afa9cb pop      r14
0x140afa9cd pop      r15
0x140afa9cf ret      
0x140afa9d0 lea      rcx, [rip + 0x1feac79] ; @0x142ae5650 'internal error: entered unreachable code'
0x140afa9d7 lea      r8, [rip + 0x1fead7a]
0x140afa9de mov      edx, 0x28
0x140afa9e3 call     0x1428d9430 ; core::panicking::panic::hb210ae7b817a9f13
0x140afa9e8 int3     