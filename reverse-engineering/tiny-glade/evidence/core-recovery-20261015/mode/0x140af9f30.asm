0x140af9f30 push     rsi
0x140af9f31 push     rdi
0x140af9f32 push     rbx
0x140af9f33 sub      rsp, 0x30
0x140af9f37 test     cl, cl
0x140af9f39 je       0x140af9f8c
0x140af9f3b mov      eax, dword ptr [rdx + 0x1c]
0x140af9f3e mov      rcx, qword ptr [rdx + 0x10]
0x140af9f42 mov      dword ptr [rcx], eax
0x140af9f44 mov      rsi, qword ptr [rdx]
0x140af9f47 mov      rdi, qword ptr [rsi + 0x30]
0x140af9f4b mov      rbx, qword ptr [rsi + 0x40]
0x140af9f4f cmp      rdi, qword ptr [rsi + 0x20]
0x140af9f53 jne      0x140af9f65
0x140af9f55 lea      rcx, [rsi + 0x20]
0x140af9f59 lea      rdx, [rip + 0x21401e8]
0x140af9f60 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140af9f65 mov      rax, qword ptr [rsi + 0x28]
0x140af9f69 mov      rcx, rdi
0x140af9f6c shl      rcx, 5
0x140af9f70 mov      dword ptr [rax + rcx], 4
0x140af9f77 mov      qword ptr [rax + rcx + 0x18], rbx
0x140af9f7c inc      rdi
0x140af9f7f mov      qword ptr [rsi + 0x30], rdi
0x140af9f83 inc      qword ptr [rsi + 0x40]
0x140af9f87 jmp      0x140afa050 ; 
0x140af9f8c mov      eax, dword ptr [r8 + 0x1c]
0x140af9f90 mov      rcx, qword ptr [r8 + 0x10]
0x140af9f94 mov      dword ptr [rcx], eax
0x140af9f96 mov      rcx, qword ptr [r8]
0x140af9f99 cmp      qword ptr [rcx + 0x10], 5
0x140af9f9e jb       0x140afa050
0x140af9fa4 mov      r8, qword ptr [rcx + 8]
0x140af9fa8 mov      r10, qword ptr [r8 + 0x90]
0x140af9faf test     r10, r10
0x140af9fb2 je       0x140afa050
0x140af9fb8 cmp      byte ptr [rcx + 0x18], 4
0x140af9fbc jne      0x140af9fdb
0x140af9fbe mov      rax, qword ptr [r8 + 0x98]
0x140af9fc5 inc      rax
0x140af9fc8 mov      rdx, rax
0x140af9fcb or       rdx, r10
0x140af9fce shr      rdx, 0x20
0x140af9fd2 je       0x140af9ff9
0x140af9fd4 xor      edx, edx
0x140af9fd6 div      r10
0x140af9fd9 jmp      0x140af9ffe ; 
0x140af9fdb mov      rdx, qword ptr [r8 + 0x98]
0x140af9fe2 cmp      rdx, r10
0x140af9fe5 jb       0x140afa005
0x140af9fe7 lea      r8, [rip + 0x1feb64a]
0x140af9fee mov      rcx, rdx
0x140af9ff1 mov      rdx, r10
0x140af9ff4 call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x140af9ff9 xor      edx, edx
0x140af9ffb div      r10d
0x140af9ffe mov      qword ptr [r8 + 0x98], rdx
0x140afa005 mov      r8, qword ptr [r8 + 0x88]
0x140afa00c shl      rdx, 4
0x140afa010 mov      eax, dword ptr [r8 + rdx]
0x140afa014 mov      byte ptr [rcx + 0x18], 4
0x140afa018 cmp      eax, 0x15
0x140afa01b je       0x140afa050
0x140afa01d add      r8, rdx
0x140afa020 mov      ecx, dword ptr [r8 + 0xc]
0x140afa024 mov      dword ptr [rsp + 0x28], ecx
0x140afa028 mov      rcx, qword ptr [r8 + 4]
0x140afa02c mov      qword ptr [rsp + 0x20], rcx
0x140afa031 mov      ecx, dword ptr [r9 + 0x1c]
0x140afa035 mov      rdx, qword ptr [r9 + 0x10]
0x140afa039 mov      dword ptr [rdx], ecx
0x140afa03b mov      rcx, qword ptr [r9]
0x140afa03e mov      dword ptr [rcx], eax
0x140afa040 mov      rax, qword ptr [rsp + 0x20]
0x140afa045 mov      qword ptr [rcx + 4], rax
0x140afa049 mov      eax, dword ptr [rsp + 0x28]
0x140afa04d mov      dword ptr [rcx + 0xc], eax
0x140afa050 add      rsp, 0x30
0x140afa054 pop      rbx
0x140afa055 pop      rdi
0x140afa056 pop      rsi
0x140afa057 ret      