0x140af9e00 push     rsi
0x140af9e01 push     rdi
0x140af9e02 push     rbx
0x140af9e03 sub      rsp, 0x90
0x140af9e0a test     cl, cl
0x140af9e0c je       0x140af9e5f
0x140af9e0e mov      eax, dword ptr [rdx + 0x1c]
0x140af9e11 mov      rcx, qword ptr [rdx + 0x10]
0x140af9e15 mov      dword ptr [rcx], eax
0x140af9e17 mov      rsi, qword ptr [rdx]
0x140af9e1a mov      rdi, qword ptr [rsi + 0x30]
0x140af9e1e mov      rbx, qword ptr [rsi + 0x40]
0x140af9e22 cmp      rdi, qword ptr [rsi + 0x20]
0x140af9e26 jne      0x140af9e38
0x140af9e28 lea      rcx, [rsi + 0x20]
0x140af9e2c lea      rdx, [rip + 0x2140315]
0x140af9e33 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140af9e38 mov      rax, qword ptr [rsi + 0x28]
0x140af9e3c mov      rcx, rdi
0x140af9e3f shl      rcx, 5
0x140af9e43 mov      dword ptr [rax + rcx], 4
0x140af9e4a mov      qword ptr [rax + rcx + 0x18], rbx
0x140af9e4f inc      rdi
0x140af9e52 mov      qword ptr [rsi + 0x30], rdi
0x140af9e56 inc      qword ptr [rsi + 0x40]
0x140af9e5a jmp      0x140af9f24 ; 
0x140af9e5f mov      rax, qword ptr [r8]
0x140af9e62 mov      rcx, qword ptr [r8 + 8]
0x140af9e66 mov      rdx, qword ptr [r8 + 0x10]
0x140af9e6a mov      r9, qword ptr [r8 + 0x18]
0x140af9e6e mov      rdi, qword ptr [r8 + 0x20]
0x140af9e72 mov      rbx, qword ptr [r8 + 0x28]
0x140af9e76 mov      rsi, qword ptr [r8 + 0x30]
0x140af9e7a mov      rcx, qword ptr [rcx]
0x140af9e7d movups   xmm0, xmmword ptr [rcx]
0x140af9e80 movaps   xmmword ptr [rsp + 0x20], xmm0
0x140af9e85 mov      ecx, dword ptr [rax + 0x1c]
0x140af9e88 mov      r8, qword ptr [rax + 0x10]
0x140af9e8c mov      dword ptr [r8], ecx
0x140af9e8f mov      rax, qword ptr [rax]
0x140af9e92 movaps   xmm0, xmmword ptr [rsp + 0x20]
0x140af9e97 movups   xmmword ptr [rax], xmm0
0x140af9e9a mov      eax, dword ptr [rdx + 0x1c]
0x140af9e9d mov      rcx, qword ptr [rdx + 0x10]
0x140af9ea1 mov      dword ptr [rcx], eax
0x140af9ea3 mov      rcx, qword ptr [rdx]
0x140af9ea6 mov      rax, qword ptr [r9]
0x140af9ea9 movups   xmm0, xmmword ptr [rax + 0x60]
0x140af9ead movups   xmmword ptr [rsp + 0x28], xmm0
0x140af9eb2 movabs   rax, 0x8000000000000000
0x140af9ebc mov      qword ptr [rsp + 0x20], rax
0x140af9ec1 mov      qword ptr [rsp + 0x38], 0
0x140af9eca mov      byte ptr [rsp + 0x40], 1
0x140af9ecf movabs   rax, 0x8000000000000001
0x140af9ed9 mov      qword ptr [rsp + 0x58], rax
0x140af9ede lea      r8, [rip + 0x2160f53]
0x140af9ee5 lea      rdx, [rsp + 0x20]
0x140af9eea call     0x140b59be0 ; _$LT$alloc..collections..vec_deque..VecDeque$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$T$GT$$GT$::extend::haf34d79b96a8c0b5
0x140af9eef mov      rax, qword ptr [rbx]
0x140af9ef2 movss    xmm0, dword ptr [rax]
0x140af9ef6 mulss    xmm0, dword ptr [rip + 0x1e2ba8a]
0x140af9efe mov      eax, dword ptr [rdi + 0x1c]
0x140af9f01 mov      rcx, qword ptr [rdi + 0x10]
0x140af9f05 mov      dword ptr [rcx], eax
0x140af9f07 mov      rax, qword ptr [rdi]
0x140af9f0a mov      dword ptr [rax], 1
0x140af9f10 movss    dword ptr [rax + 4], xmm0
0x140af9f15 mov      eax, dword ptr [rsi + 0x1c]
0x140af9f18 mov      rcx, qword ptr [rsi + 0x10]
0x140af9f1c mov      dword ptr [rcx], eax
0x140af9f1e mov      rax, qword ptr [rsi]
0x140af9f21 mov      byte ptr [rax], 0
0x140af9f24 add      rsp, 0x90
0x140af9f2b pop      rbx
0x140af9f2c pop      rdi
0x140af9f2d pop      rsi
0x140af9f2e ret      