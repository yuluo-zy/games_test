0x140afa9f0 push     rsi
0x140afa9f1 push     rdi
0x140afa9f2 push     rbx
0x140afa9f3 sub      rsp, 0x30
0x140afa9f7 test     cl, cl
0x140afa9f9 je       0x140afaa4c
0x140afa9fb mov      eax, dword ptr [rdx + 0x1c]
0x140afa9fe mov      rcx, qword ptr [rdx + 0x10]
0x140afaa02 mov      dword ptr [rcx], eax
0x140afaa04 mov      rsi, qword ptr [rdx]
0x140afaa07 mov      rdi, qword ptr [rsi + 0x30]
0x140afaa0b mov      rbx, qword ptr [rsi + 0x40]
0x140afaa0f cmp      rdi, qword ptr [rsi + 0x20]
0x140afaa13 jne      0x140afaa25
0x140afaa15 lea      rcx, [rsi + 0x20]
0x140afaa19 lea      rdx, [rip + 0x213f728]
0x140afaa20 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140afaa25 mov      rax, qword ptr [rsi + 0x28]
0x140afaa29 mov      rcx, rdi
0x140afaa2c shl      rcx, 5
0x140afaa30 mov      dword ptr [rax + rcx], 4
0x140afaa37 mov      qword ptr [rax + rcx + 0x18], rbx
0x140afaa3c inc      rdi
0x140afaa3f mov      qword ptr [rsi + 0x30], rdi
0x140afaa43 inc      qword ptr [rsi + 0x40]
0x140afaa47 jmp      0x140afab5d ; 
0x140afaa4c mov      r10, qword ptr [r8]
0x140afaa4f cmp      dword ptr [r10], 0x13
0x140afaa53 jne      0x140afaa9a
0x140afaa55 mov      eax, dword ptr [r8 + 0x1c]
0x140afaa59 mov      rcx, qword ptr [r8 + 0x10]
0x140afaa5d mov      dword ptr [rcx], eax
0x140afaa5f mov      dword ptr [r10], 0
0x140afaa66 mov      eax, dword ptr [r9 + 0x1c]
0x140afaa6a mov      rcx, qword ptr [r9 + 0x10]
0x140afaa6e mov      dword ptr [rcx], eax
0x140afaa70 mov      rcx, qword ptr [r9]
0x140afaa73 mov      eax, dword ptr [r10]
0x140afaa76 mov      dword ptr [rsp + 0x20], eax
0x140afaa7a mov      rax, qword ptr [r10 + 4]
0x140afaa7e mov      qword ptr [rsp + 0x24], rax
0x140afaa83 mov      eax, dword ptr [r10 + 0xc]
0x140afaa87 mov      dword ptr [rsp + 0x2c], eax
0x140afaa8b lea      rdx, [rsp + 0x20]
0x140afaa90 call     0x140af9850 ; country_core::resources::toolbar::Toolbar::switch_mode::h64573dbffe9d81b9
0x140afaa95 jmp      0x140afab5d ; 
0x140afaa9a mov      eax, dword ptr [r9 + 0x1c]
0x140afaa9e mov      rcx, qword ptr [r9 + 0x10]
0x140afaaa2 mov      dword ptr [rcx], eax
0x140afaaa4 mov      rcx, qword ptr [r9]
0x140afaaa7 cmp      qword ptr [rcx + 0x10], 0xb
0x140afaaac jb       0x140afab5d
0x140afaab2 mov      r11, qword ptr [rcx + 8]
0x140afaab6 mov      r9, qword ptr [r11 + 0x150]
0x140afaabd test     r9, r9
0x140afaac0 je       0x140afab5d
0x140afaac6 cmp      byte ptr [rcx + 0x18], 0xa
0x140afaaca jne      0x140afaae9
0x140afaacc mov      rax, qword ptr [r11 + 0x158]
0x140afaad3 inc      rax
0x140afaad6 mov      rdx, rax
0x140afaad9 or       rdx, r9
0x140afaadc shr      rdx, 0x20
0x140afaae0 je       0x140afab07
0x140afaae2 xor      edx, edx
0x140afaae4 div      r9
0x140afaae7 jmp      0x140afab0c ; 
0x140afaae9 mov      rdx, qword ptr [r11 + 0x158]
0x140afaaf0 cmp      rdx, r9
0x140afaaf3 jb       0x140afab13
0x140afaaf5 lea      r8, [rip + 0x1feab3c]
0x140afaafc mov      rcx, rdx
0x140afaaff mov      rdx, r9
0x140afab02 call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x140afab07 xor      edx, edx
0x140afab09 div      r9d
0x140afab0c mov      qword ptr [r11 + 0x158], rdx
0x140afab13 mov      r9, qword ptr [r11 + 0x148]
0x140afab1a shl      rdx, 4
0x140afab1e mov      eax, dword ptr [r9 + rdx]
0x140afab22 mov      byte ptr [rcx + 0x18], 0xa
0x140afab26 cmp      eax, 0x15
0x140afab29 je       0x140afab5d
0x140afab2b add      r9, rdx
0x140afab2e mov      ecx, dword ptr [r9 + 0xc]
0x140afab32 mov      dword ptr [rsp + 0x28], ecx
0x140afab36 mov      rcx, qword ptr [r9 + 4]
0x140afab3a mov      qword ptr [rsp + 0x20], rcx
0x140afab3f mov      ecx, dword ptr [r8 + 0x1c]
0x140afab43 mov      rdx, qword ptr [r8 + 0x10]
0x140afab47 mov      dword ptr [rdx], ecx
0x140afab49 mov      dword ptr [r10], eax
0x140afab4c mov      rax, qword ptr [rsp + 0x20]
0x140afab51 mov      qword ptr [r10 + 4], rax
0x140afab55 mov      eax, dword ptr [rsp + 0x28]
0x140afab59 mov      dword ptr [r10 + 0xc], eax
0x140afab5d add      rsp, 0x30
0x140afab61 pop      rbx
0x140afab62 pop      rdi
0x140afab63 pop      rsi
0x140afab64 ret      