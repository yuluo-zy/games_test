0x140afa2c0 push     rsi
0x140afa2c1 push     rdi
0x140afa2c2 push     rbx
0x140afa2c3 sub      rsp, 0x30
0x140afa2c7 test     cl, cl
0x140afa2c9 je       0x140afa31c
0x140afa2cb mov      eax, dword ptr [rdx + 0x1c]
0x140afa2ce mov      rcx, qword ptr [rdx + 0x10]
0x140afa2d2 mov      dword ptr [rcx], eax
0x140afa2d4 mov      rsi, qword ptr [rdx]
0x140afa2d7 mov      rdi, qword ptr [rsi + 0x30]
0x140afa2db mov      rbx, qword ptr [rsi + 0x40]
0x140afa2df cmp      rdi, qword ptr [rsi + 0x20]
0x140afa2e3 jne      0x140afa2f5
0x140afa2e5 lea      rcx, [rsi + 0x20]
0x140afa2e9 lea      rdx, [rip + 0x213fe58]
0x140afa2f0 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140afa2f5 mov      rax, qword ptr [rsi + 0x28]
0x140afa2f9 mov      rcx, rdi
0x140afa2fc shl      rcx, 5
0x140afa300 mov      dword ptr [rax + rcx], 4
0x140afa307 mov      qword ptr [rax + rcx + 0x18], rbx
0x140afa30c inc      rdi
0x140afa30f mov      qword ptr [rsi + 0x30], rdi
0x140afa313 inc      qword ptr [rsi + 0x40]
0x140afa317 jmp      0x140afa3e7 ; 
0x140afa31c mov      rax, qword ptr [r8]
0x140afa31f mov      rcx, qword ptr [r8 + 8]
0x140afa323 mov      rdx, qword ptr [r8 + 0x10]
0x140afa327 mov      r8d, dword ptr [rax + 0x1c]
0x140afa32b mov      r9, qword ptr [rax + 0x10]
0x140afa32f mov      dword ptr [r9], r8d
0x140afa332 mov      rax, qword ptr [rax]
0x140afa335 movups   xmm0, xmmword ptr [rcx]
0x140afa338 movaps   xmmword ptr [rsp + 0x20], xmm0
0x140afa33d mov      ecx, dword ptr [rdx + 0x1c]
0x140afa340 mov      r8, qword ptr [rdx + 0x10]
0x140afa344 mov      dword ptr [r8], ecx
0x140afa347 mov      rcx, qword ptr [rdx]
0x140afa34a movaps   xmm0, xmmword ptr [rsp + 0x20]
0x140afa34f movups   xmmword ptr [rax], xmm0
0x140afa352 mov      edx, dword ptr [rsp + 0x20]
0x140afa356 lea      r8d, [rdx - 6]
0x140afa35a add      rdx, -5
0x140afa35e xor      eax, eax
0x140afa360 cmp      r8d, 0xf
0x140afa364 cmovb    rax, rdx
0x140afa368 cmp      rax, 0xf
0x140afa36c je       0x140afa373
0x140afa36e test     rax, rax
0x140afa371 jne      0x140afa3ad
0x140afa373 cmp      qword ptr [rcx + 0x10], 8
0x140afa378 jb       0x140afa3e7
0x140afa37a mov      r8, qword ptr [rcx + 8]
0x140afa37e mov      r9, qword ptr [r8 + 0xf0]
0x140afa385 test     r9, r9
0x140afa388 je       0x140afa3e7
0x140afa38a cmp      byte ptr [rcx + 0x18], 7
0x140afa38e jne      0x140afa3b9
0x140afa390 mov      rax, qword ptr [r8 + 0xf8]
0x140afa397 inc      rax
0x140afa39a mov      rdx, rax
0x140afa39d or       rdx, r9
0x140afa3a0 shr      rdx, 0x20
0x140afa3a4 je       0x140afa3d7
0x140afa3a6 xor      edx, edx
0x140afa3a8 div      r9
0x140afa3ab jmp      0x140afa3dc ; 
0x140afa3ad lea      rdx, [rsp + 0x20]
0x140afa3b2 call     0x140af9850 ; country_core::resources::toolbar::Toolbar::switch_mode::h64573dbffe9d81b9
0x140afa3b7 jmp      0x140afa3e7 ; 
0x140afa3b9 mov      rax, qword ptr [r8 + 0xf8]
0x140afa3c0 cmp      rax, r9
0x140afa3c3 jb       0x140afa3e3
0x140afa3c5 lea      r8, [rip + 0x1feb26c]
0x140afa3cc mov      rcx, rax
0x140afa3cf mov      rdx, r9
0x140afa3d2 call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x140afa3d7 xor      edx, edx
0x140afa3d9 div      r9d
0x140afa3dc mov      qword ptr [r8 + 0xf8], rdx
0x140afa3e3 mov      byte ptr [rcx + 0x18], 7
0x140afa3e7 add      rsp, 0x30
0x140afa3eb pop      rbx
0x140afa3ec pop      rdi
0x140afa3ed pop      rsi
0x140afa3ee ret      