0x140942f80 push rbp
0x140942f81 push rsi
0x140942f82 push rdi
0x140942f83 push rbx
0x140942f84 sub rsp, 0x2f8
0x140942f8b lea rbp, [rsp + 0x80]
0x140942f93 movaps xmmword ptr [rbp + 0x260], xmm6
0x140942f9a mov qword ptr [rbp + 0x258], 0xfffffffffffffffe
0x140942fa5 mov rsi, rcx
0x140942fa8 mov rax, qword ptr [rdx]
0x140942fab movabs rbx, 0x8000000000000000
0x140942fb5 test rax, rax
0x140942fb8 je 0x1409430d8
0x140942fbe cmp eax, 1
0x140942fc1 jne 0x1409431b3
0x140942fc7 mov rax, qword ptr [rdx + 8]
0x140942fcb mov qword ptr [rbp + 0x250], rax
0x140942fd2 mov rdi, rdx
0x140942fd5 mov rax, qword ptr [rdx + 0x10]
0x140942fd9 mov qword ptr [rbp + 0x248], rax
0x140942fe0 lea rcx, [rbp + 0x40]
0x140942fe4 mov rdx, r8
0x140942fe7 call 0x140911710
0x140942fec mov dword ptr [rbp + 0x200], 0
0x140942ff6 lea rcx, [rbp + 0x1e0]
0x140942ffd call 0x140af8c50
0x140943002 mov eax, dword ptr [rbp + 0x60]
0x140943005 mov dword ptr [rbp + 0x230], eax
0x14094300b movups xmm0, xmmword ptr [rbp + 0x40]
0x14094300f movups xmm1, xmmword ptr [rbp + 0x50]
0x140943013 movups xmmword ptr [rbp + 0x220], xmm1
0x14094301a movups xmmword ptr [rbp + 0x210], xmm0
0x140943021 mov rax, qword ptr [rbp + 0x230]
0x140943028 mov qword ptr [rbp + 0x90], rax
0x14094302f movaps xmmword ptr [rbp + 0x80], xmm1
0x140943036 movaps xmmword ptr [rbp + 0x70], xmm0
0x14094303a movups xmm0, xmmword ptr [rbp + 0x1e0]
0x140943041 movups xmm1, xmmword ptr [rbp + 0x1f0]
0x140943048 movups xmm2, xmmword ptr [rbp + 0x200]
0x14094304f movaps xmmword ptr [rbp + 0x60], xmm2
0x140943053 movaps xmmword ptr [rbp + 0x50], xmm1
0x140943057 movaps xmmword ptr [rbp + 0x40], xmm0
0x14094305b mov qword ptr [rbp + 0x98], 0
0x140943066 lea r8, [rip + 0x21702d3] ; 0x142ab3340 e832ab42010000005800000000000000 f32=85.59942626953125
0x14094306d lea rcx, [rbp + 0x1b0]
0x140943074 lea rdx, [rbp + 0x40]
0x140943078 call 0x141301640
0x14094307d add rdi, 0x18
0x140943081 movups xmm0, xmmword ptr [rbp + 0x1b0]
0x140943088 mov rax, qword ptr [rbp + 0x1c0]
0x14094308f xorps xmm1, xmm1
0x140943092 movaps xmmword ptr [rbp + 0x10], xmm1
0x140943096 mov rcx, qword ptr [rbp + 0x250]
0x14094309d mov qword ptr [rbp + 0x20], rcx
0x1409430a1 mov rcx, qword ptr [rbp + 0x248]
0x1409430a8 mov qword ptr [rbp + 0x28], rcx
0x1409430ac movups xmm1, xmmword ptr [rdi]
0x1409430af movaps xmmword ptr [rbp + 0x30], xmm1
0x1409430b3 movaps xmmword ptr [rbp - 0x20], xmm0
0x1409430b7 mov qword ptr [rbp - 0x10], 0
0x1409430bf mov qword ptr [rbp - 8], rax
0x1409430c3 mov qword ptr [rbp], 0
0x1409430cb mov qword ptr [rbp + 8], 8
0x1409430d3 jmp 0x14094324a
0x1409430d8 lea rcx, [rbp + 0x40]
0x1409430dc mov rdx, r8
0x1409430df call 0x140911710
0x1409430e4 mov dword ptr [rbp + 0x200], 0
0x1409430ee lea rcx, [rbp + 0x1e0]
0x1409430f5 call 0x140af8c50
0x1409430fa mov eax, dword ptr [rbp + 0x60]
0x1409430fd mov dword ptr [rbp + 0x230], eax
0x140943103 movups xmm0, xmmword ptr [rbp + 0x40]
0x140943107 movups xmm1, xmmword ptr [rbp + 0x50]
0x14094310b movups xmmword ptr [rbp + 0x220], xmm1
0x140943112 movups xmmword ptr [rbp + 0x210], xmm0
0x140943119 mov rax, qword ptr [rbp + 0x230]
0x140943120 mov qword ptr [rbp + 0x90], rax
0x140943127 movaps xmmword ptr [rbp + 0x80], xmm1
0x14094312e movaps xmmword ptr [rbp + 0x70], xmm0
0x140943132 movups xmm0, xmmword ptr [rbp + 0x1e0]
0x140943139 movups xmm1, xmmword ptr [rbp + 0x1f0]
0x140943140 movups xmm2, xmmword ptr [rbp + 0x200]
0x140943147 movaps xmmword ptr [rbp + 0x60], xmm2
0x14094314b movaps xmmword ptr [rbp + 0x50], xmm1
0x14094314f movaps xmmword ptr [rbp + 0x40], xmm0
0x140943153 mov qword ptr [rbp + 0x98], 0
0x14094315e lea r8, [rip + 0x21701db] ; 0x142ab3340 e832ab42010000005800000000000000 f32=85.59942626953125
0x140943165 lea rcx, [rbp + 0x1b0]
0x14094316c lea rdx, [rbp + 0x40]
0x140943170 call 0x141301640
0x140943175 movups xmm0, xmmword ptr [rbp + 0x1b0]
0x14094317c mov rax, qword ptr [rbp + 0x1c0]
0x140943183 xorps xmm1, xmm1
0x140943186 movaps xmmword ptr [rbp + 0x10], xmm1
0x14094318a movaps xmmword ptr [rbp - 0x20], xmm0
0x14094318e mov qword ptr [rbp - 0x10], 0
0x140943196 mov qword ptr [rbp - 8], rax
0x14094319a mov qword ptr [rbp], 0
0x1409431a2 mov qword ptr [rbp + 8], 8
0x1409431aa mov qword ptr [rbp + 0x20], rbx
0x1409431ae jmp 0x14094324a
0x1409431b3 movups xmm0, xmmword ptr [rdx + 8]
0x1409431b7 movups xmm1, xmmword ptr [rdx + 0x18]
0x1409431bb movaps xmmword ptr [rbp - 0x30], xmm1
0x1409431bf movaps xmmword ptr [rbp - 0x40], xmm0
0x1409431c3 lea rdi, [rbp - 0x40]
0x1409431c7 mov rcx, rdi
0x1409431ca call 0x140d76ed0
0x1409431cf mov r8, rdx
0x1409431d2 lea rcx, [rbp + 0x1e0]
0x1409431d9 mov rdx, rax
0x1409431dc call 0x1409499c0
0x1409431e1 xor eax, eax
0x1409431e3 cmp rax, qword ptr [rbp + 0x1e0]
0x1409431ea jo 0x1409434a5
0x1409431f0 movups xmm0, xmmword ptr [rbp + 0x230]
0x1409431f7 movaps xmmword ptr [rbp + 0x30], xmm0
0x1409431fb movups xmm0, xmmword ptr [rbp + 0x220]
0x140943202 movaps xmmword ptr [rbp + 0x20], xmm0
0x140943206 movups xmm0, xmmword ptr [rbp + 0x1e0]
0x14094320d movups xmm1, xmmword ptr [rbp + 0x1f0]
0x140943214 movups xmm2, xmmword ptr [rbp + 0x200]
0x14094321b movups xmm3, xmmword ptr [rbp + 0x210]
0x140943222 movaps xmmword ptr [rbp + 0x10], xmm3
0x140943226 movaps xmmword ptr [rbp], xmm2
0x14094322a movaps xmmword ptr [rbp - 0x10], xmm1
0x14094322e movaps xmmword ptr [rbp - 0x20], xmm0
0x140943232 mov rdx, qword ptr [rbp - 0x40]
0x140943236 test rdx, rdx
0x140943239 je 0x14094324a
0x14094323b mov rcx, qword ptr [rbp - 0x38]
0x14094323f mov r8d, 1
0x140943245 call 0x140613c20
0x14094324a lea rdx, [rbp - 0x20]
0x14094324e mov rcx, rsi
0x140943251 call 0x14154bdd0
0x140943256 xorps xmm6, xmm6
0x140943259 movaps xmmword ptr [rbp + 0xe0], xmm6
0x140943260 movaps xmmword ptr [rbp + 0xd0], xmm6
0x140943267 movaps xmmword ptr [rbp + 0xc0], xmm6
0x14094326e movaps xmmword ptr [rbp + 0xb0], xmm6
0x140943275 movaps xmmword ptr [rbp + 0xa0], xmm6
0x14094327c movaps xmmword ptr [rbp + 0x90], xmm6
0x140943283 movaps xmmword ptr [rbp + 0x80], xmm6
0x14094328a movaps xmmword ptr [rbp + 0x70], xmm6
0x14094328e movaps xmmword ptr [rbp + 0x60], xmm6
0x140943292 movaps xmmword ptr [rbp + 0x50], xmm6
0x140943296 movaps xmmword ptr [rbp + 0x40], xmm6
0x14094329a lea rdx, [rbp + 0x40]
0x14094329e mov rcx, rax
0x1409432a1 call 0x141530550
0x1409432a6 movups xmmword ptr [rbp + 0x204], xmm6
0x1409432ad movups xmmword ptr [rbp + 0x1f4], xmm6
0x1409432b4 movups xmmword ptr [rbp + 0x1e4], xmm6
0x1409432bb mov dword ptr [rbp + 0x40], 4
0x1409432c2 mov dword ptr [rbp + 0x48], 4
0x1409432c9 mov dword ptr [rbp + 0x50], 4
0x1409432d0 mov dword ptr [rbp + 0x58], 4
0x1409432d7 mov dword ptr [rbp + 0x60], 4
0x1409432de mov dword ptr [rbp + 0x68], 4
0x1409432e5 mov dword ptr [rbp + 0x70], 4
0x1409432ec mov dword ptr [rbp + 0x78], 4
0x1409432f3 mov dword ptr [rbp + 0x80], 4
0x1409432fd mov dword ptr [rbp + 0x88], 4
0x140943307 mov dword ptr [rbp + 0x90], 4
0x140943311 mov dword ptr [rbp + 0x98], 4
0x14094331b mov dword ptr [rbp + 0xa0], 4
0x140943325 mov dword ptr [rbp + 0xa8], 4
0x14094332f mov dword ptr [rbp + 0xb0], 4
0x140943339 mov dword ptr [rbp + 0xb8], 4
0x140943343 mov dword ptr [rbp + 0xc0], 4
0x14094334d mov dword ptr [rbp + 0xc8], 4
0x140943357 mov dword ptr [rbp + 0xd0], 4
0x140943361 mov dword ptr [rbp + 0xd8], 4
0x14094336b mov dword ptr [rbp + 0xe0], 4
0x140943375 mov dword ptr [rbp + 0xe8], 4
0x14094337f mov dword ptr [rbp + 0xf0], 4
0x140943389 mov dword ptr [rbp + 0xf8], 4
0x140943393 mov dword ptr [rbp + 0x100], 4
0x14094339d mov dword ptr [rbp + 0x108], 4
0x1409433a7 mov dword ptr [rbp + 0x110], 4
0x1409433b1 mov dword ptr [rbp + 0x118], 4
0x1409433bb mov dword ptr [rbp + 0x120], 4
0x1409433c5 mov dword ptr [rbp + 0x128], 4
0x1409433cf mov dword ptr [rbp + 0x130], 4
0x1409433d9 mov dword ptr [rbp + 0x138], 4
0x1409433e3 mov dword ptr [rbp + 0x140], 4
0x1409433ed mov dword ptr [rbp + 0x148], 4
0x1409433f7 mov dword ptr [rbp + 0x150], 4
0x140943401 mov dword ptr [rbp + 0x158], 4
0x14094340b mov dword ptr [rbp + 0x160], 4
0x140943415 mov dword ptr [rbp + 0x168], 4
0x14094341f mov dword ptr [rbp + 0x170], 4
0x140943429 mov dword ptr [rbp + 0x178], 4
0x140943433 mov ecx, dword ptr [rbp + 0x210]
0x140943439 mov dword ptr [rbp + 0x1ac], ecx
0x14094343f movups xmm0, xmmword ptr [rbp + 0x1e0]
0x140943446 movups xmm1, xmmword ptr [rbp + 0x1f0]
0x14094344d movups xmm2, xmmword ptr [rbp + 0x200]
0x140943454 movups xmmword ptr [rbp + 0x19c], xmm2
0x14094345b movups xmmword ptr [rbp + 0x18c], xmm1
0x140943462 movups xmmword ptr [rbp + 0x17c], xmm0
0x140943469 lea rdx, [rbp + 0x40]
0x14094346d mov rcx, rax
0x140943470 call 0x14154b880
0x140943475 mov rcx, rax
0x140943478 xor edx, edx
0x14094347a xor r8d, r8d
0x14094347d call 0x141540ed0
0x140943482 mov qword ptr [rbp + 0x40], rbx
0x140943486 lea rdx, [rbp + 0x40]
0x14094348a mov rcx, rax
0x14094348d call 0x14154a700
0x140943492 movaps xmm6, xmmword ptr [rbp + 0x260]
0x140943499 add rsp, 0x2f8
0x1409434a0 pop rbx
0x1409434a1 pop rdi
0x1409434a2 pop rsi
0x1409434a3 pop rbp
0x1409434a4 ret 
0x1409434a5 mov rax, qword ptr [rbp + 0x1e8]
0x1409434ac mov qword ptr [rbp + 0x240], rax
0x1409434b3 mov qword ptr [rbp - 0x60], rdi
0x1409434b7 lea rax, [rip + 0x41e622] ; 0x140d61ae0 554889e54989d0488b4108488b511048 f32=-8.103731232774271e+22
0x1409434be mov qword ptr [rbp - 0x58], rax
0x1409434c2 lea rax, [rbp + 0x240]
0x1409434c9 mov qword ptr [rbp - 0x50], rax
0x1409434cd lea rax, [rip + 0x40973c] ; 0x140d4cc10 488b09e948020000cccccccccccccccc f32=-1.0392535899618524e+25
0x1409434d4 mov qword ptr [rbp - 0x48], rax
0x1409434d8 mov qword ptr [rbp + 0x40], 2
0x1409434e0 mov qword ptr [rbp + 0x50], 2
0x1409434e8 mov qword ptr [rbp + 0x60], 0
0x1409434f0 mov qword ptr [rbp + 0x68], 0x20
0x1409434f8 mov byte ptr [rbp + 0x70], 3
0x1409434fc mov qword ptr [rbp + 0x78], 2
0x140943504 mov qword ptr [rbp + 0x88], 2
0x14094350f mov qword ptr [rbp + 0x98], 1
0x14094351a movabs rax, 0x400000020
0x140943524 mov qword ptr [rbp + 0xa0], rax
0x14094352b mov byte ptr [rbp + 0xa8], 3
0x140943532 lea rax, [rip + 0x216f1a7] ; 0x142ab26e0 b226ab42010000002700000000000000 f32=85.57557678222656
0x140943539 mov qword ptr [rbp + 0x1b0], rax
0x140943540 mov qword ptr [rbp + 0x1b8], 2
0x14094354b lea rax, [rbp + 0x40]
0x14094354f mov qword ptr [rbp + 0x1d0], rax
0x140943556 mov qword ptr [rbp + 0x1d8], 2
0x140943561 lea rax, [rbp - 0x60]
0x140943565 mov qword ptr [rbp + 0x1c0], rax
0x14094356c mov qword ptr [rbp + 0x1c8], 2
0x140943577 lea rdx, [rip + 0x216f37a] ; 0x142ab28f8 5028ab42010000002e00000000000000 f32=85.5787353515625
0x14094357e lea rcx, [rbp + 0x1b0]
0x140943585 call 0x1428d9390
0x14094358a ud2 
0x14094358c nop dword ptr [rax]
0x140943590 mov qword ptr [rsp + 0x10], rdx
0x140943595 push rbp
0x140943596 push rsi
0x140943597 push rdi
0x140943598 push rbx
0x140943599 sub rsp, 0x38
0x14094359d lea rbp, [rdx + 0x80]
0x1409435a4 movaps xmmword ptr [rsp + 0x20], xmm6
0x1409435a9 cmp qword ptr [rbp + 0x250], 0
0x1409435b1 je 0x1409435cc
0x1409435b3 mov r8d, 1
0x1409435b9 mov rcx, qword ptr [rbp + 0x248]
0x1409435c0 mov rdx, qword ptr [rbp + 0x250]
0x1409435c7 call 0x140613c20
0x1409435cc movaps xmm6, xmmword ptr [rsp + 0x20]
0x1409435d1 add rsp, 0x38
0x1409435d5 pop rbx
0x1409435d6 pop rdi
0x1409435d7 pop rsi
0x1409435d8 pop rbp
0x1409435d9 ret 
0x1409435da nop word ptr [rax + rax]
0x1409435e0 mov qword ptr [rsp + 0x10], rdx
0x1409435e5 push rbp
0x1409435e6 push rsi
0x1409435e7 push rdi
0x1409435e8 push rbx
0x1409435e9 sub rsp, 0x38
0x1409435ed lea rbp, [rdx + 0x80]
0x1409435f4 movaps xmmword ptr [rsp + 0x20], xmm6
0x1409435f9 mov rdx, qword ptr [rbp - 0x40]
0x1409435fd test rdx, rdx
0x140943600 je 0x140943611
0x140943602 mov rcx, qword ptr [rbp - 0x38]
0x140943606 mov r8d, 1
0x14094360c call 0x140613c20
0x140943611 movaps xmm6, xmmword ptr [rsp + 0x20]
0x140943616 add rsp, 0x38
0x14094361a pop rbx
0x14094361b pop rdi
0x14094361c pop rsi
0x14094361d pop rbp
0x14094361e ret 
0x14094361f nop 
0x140943620 mov qword ptr [rsp + 0x10], rdx
0x140943625 push rbp
0x140943626 push rsi
0x140943627 push rdi
0x140943628 push rbx
0x140943629 sub rsp, 0x38
0x14094362d lea rbp, [rdx + 0x80]
0x140943634 movaps xmmword ptr [rsp + 0x20], xmm6
0x140943639 lea rcx, [rbp + 0x240]
0x140943640 call 0x140d4cc30
0x140943645 movaps xmm6, xmmword ptr [rsp + 0x20]
0x14094364a add rsp, 0x38
0x14094364e pop rbx
0x14094364f pop rdi
0x140943650 pop rsi
0x140943651 pop rbp
0x140943652 ret 