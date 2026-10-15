0x1400b78b0 push     rbp
0x1400b78b1 push     r14
0x1400b78b3 push     rsi
0x1400b78b4 push     rdi
0x1400b78b5 push     rbx
0x1400b78b6 sub      rsp, 0x340
0x1400b78bd lea      rbp, [rsp + 0x80]
0x1400b78c5 mov      qword ptr [rbp + 0x2b8], 0xfffffffffffffffe
0x1400b78d0 mov      rsi, rdx
0x1400b78d3 mov      r14, qword ptr [rcx]
0x1400b78d6 mov      rdi, qword ptr [r14]
0x1400b78d9 lea      rcx, [rbp + 0x68]
0x1400b78dd mov      rdx, rdi
0x1400b78e0 call     0x140a84ba0
0x1400b78e5 mov      byte ptr [rbp + 0x98], 0
0x1400b78ec mov      dword ptr [rbp + 0x9d], 0
0x1400b78f6 mov      dword ptr [rbp + 0xa0], 0
0x1400b7900 mov      qword ptr [rbp + 0x38], 3
0x1400b7908 mov      qword ptr [rbp + 0x50], 6
0x1400b7910 mov      dword ptr [rbp + 0x80], 0
0x1400b791a mov      dword ptr [rbp + 0x88], 0
0x1400b7924 mov      qword ptr [rbp + 0x90], 0
0x1400b792f mov      word ptr [rbp + 0xa8], 0x200
0x1400b7938 mov      byte ptr [rbp + 0xaa], 2
0x1400b793f mov      byte ptr [rbp + 0xad], 2
0x1400b7946 lea      rcx, [rbp - 0x58]
0x1400b794a lea      rdx, [rbp + 0x38]
0x1400b794e mov      r8, rsi
0x1400b7951 call     0x140c28d40
0x1400b7956 mov      rax, qword ptr [rbp - 0x48]
0x1400b795a lock dec qword ptr [rax]
0x1400b795e jne      0x1400b7969
0x1400b7960 lea      rcx, [rbp - 0x48]
0x1400b7964 call     0x140c41170
0x1400b7969 mov      ecx, dword ptr [rdi + 0x68]
0x1400b796c call     0x140833250
0x1400b7971 mov      ebx, eax
0x1400b7973 mov      qword ptr [rbp - 0x58], 4
0x1400b797b mov      byte ptr [rbp + 0x2b7], 0
0x1400b7982 lea      rdx, [rip + 0x287d6c7] ; @0x142935050 'Set history start to NOWReset history start to beginningSkip serializing trees into snapshotForc'
0x1400b7989 lea      rcx, [rbp + 0x38]
0x1400b798d mov      r8d, 0x18
0x1400b7993 call     0x1400568d0
0x1400b7998 movups   xmm0, xmmword ptr [rbp + 0x98]
0x1400b799f movaps   xmmword ptr [rbp + 0x240], xmm0
0x1400b79a6 movups   xmm0, xmmword ptr [rbp + 0x88]
0x1400b79ad movaps   xmmword ptr [rbp + 0x230], xmm0
0x1400b79b4 movups   xmm0, xmmword ptr [rbp + 0x78]
0x1400b79b8 movaps   xmmword ptr [rbp + 0x220], xmm0
0x1400b79bf movups   xmm0, xmmword ptr [rbp + 0x38]
0x1400b79c3 movups   xmm1, xmmword ptr [rbp + 0x48]
0x1400b79c7 movups   xmm2, xmmword ptr [rbp + 0x58]
0x1400b79cb movups   xmm3, xmmword ptr [rbp + 0x68]
0x1400b79cf movaps   xmmword ptr [rbp + 0x210], xmm3
0x1400b79d6 movaps   xmmword ptr [rbp + 0x200], xmm2
0x1400b79dd movaps   xmmword ptr [rbp + 0x1f0], xmm1
0x1400b79e4 movaps   xmmword ptr [rbp + 0x1e0], xmm0
0x1400b79eb mov      byte ptr [rbp + 0x2b7], 1
0x1400b79f2 lea      rcx, [rbp + 0x38]
0x1400b79f6 lea      rdx, [rbp - 0x58]
0x1400b79fa lea      r8, [rbp + 0x1e0]
0x1400b7a01 call     0x140c46bd0
0x1400b7a06 lea      rcx, [rbp + 0x250]
0x1400b7a0d lea      rdx, [rbp + 0x38]
0x1400b7a11 mov      r8, rsi
0x1400b7a14 call     0x140c46d20
0x1400b7a19 cmp      byte ptr [rbp + 0x2a4], 0
0x1400b7a20 je       0x1400b7a51
0x1400b7a22 mov      rax, qword ptr [rbp + 0x260]
0x1400b7a29 lock dec qword ptr [rax]
0x1400b7a2d jne      0x1400b7a3b
0x1400b7a2f lea      rcx, [rbp + 0x260]
0x1400b7a36 call     0x140c41170
0x1400b7a3b mov      eax, dword ptr [r14 + 0x1c]
0x1400b7a3f mov      rcx, qword ptr [r14 + 0x10]
0x1400b7a43 mov      dword ptr [rcx], eax
0x1400b7a45 mov      rcx, rdi
0x1400b7a48 mov      edx, ebx
0x1400b7a4a call     0x140a84c40
0x1400b7a4f jmp      0x1400b7a8b
0x1400b7a51 mov      byte ptr [rbp + 0x38], 0
0x1400b7a55 cmp      byte ptr [rbp + 0x2a3], 0
0x1400b7a5c je       0x1400b7a72
0x1400b7a5e lea      rcx, [rbp + 0x260]
0x1400b7a65 lea      rdx, [rbp + 0x38]
0x1400b7a69 call     0x140042870
0x1400b7a6e test     al, al
0x1400b7a70 jne      0x1400b7a22
0x1400b7a72 mov      rax, qword ptr [rbp + 0x260]
0x1400b7a79 lock dec qword ptr [rax]
0x1400b7a7d jne      0x1400b7a8b
0x1400b7a7f lea      rcx, [rbp + 0x260]
0x1400b7a86 call     0x140c41170
0x1400b7a8b mov      qword ptr [rbp - 0x58], 4
0x1400b7a93 mov      byte ptr [rbp + 0x2b6], 0
0x1400b7a9a lea      rdx, [rip + 0x287d5c7] ; @0x142935068 'Reset history start to beginningSkip serializing trees into snapshotForce Music Richness to 1.0S'
0x1400b7aa1 lea      rcx, [rbp + 0x38]
0x1400b7aa5 mov      r8d, 0x20
0x1400b7aab call     0x1400568d0
0x1400b7ab0 movups   xmm0, xmmword ptr [rbp + 0x98]
0x1400b7ab7 movaps   xmmword ptr [rbp + 0x240], xmm0
0x1400b7abe movups   xmm0, xmmword ptr [rbp + 0x88]
0x1400b7ac5 movaps   xmmword ptr [rbp + 0x230], xmm0
0x1400b7acc movups   xmm0, xmmword ptr [rbp + 0x78]
0x1400b7ad0 movaps   xmmword ptr [rbp + 0x220], xmm0
0x1400b7ad7 movups   xmm0, xmmword ptr [rbp + 0x38]
0x1400b7adb movups   xmm1, xmmword ptr [rbp + 0x48]
0x1400b7adf movups   xmm2, xmmword ptr [rbp + 0x58]
0x1400b7ae3 movups   xmm3, xmmword ptr [rbp + 0x68]
0x1400b7ae7 movaps   xmmword ptr [rbp + 0x210], xmm3
0x1400b7aee movaps   xmmword ptr [rbp + 0x200], xmm2
0x1400b7af5 movaps   xmmword ptr [rbp + 0x1f0], xmm1
0x1400b7afc movaps   xmmword ptr [rbp + 0x1e0], xmm0
0x1400b7b03 mov      byte ptr [rbp + 0x2b6], 1
0x1400b7b0a lea      rcx, [rbp + 0x38]
0x1400b7b0e lea      rdx, [rbp - 0x58]
0x1400b7b12 lea      r8, [rbp + 0x1e0]
0x1400b7b19 call     0x140c46bd0
0x1400b7b1e lea      rcx, [rbp + 0x250]
0x1400b7b25 lea      rdx, [rbp + 0x38]
0x1400b7b29 mov      r8, rsi
0x1400b7b2c call     0x140c46d20
0x1400b7b31 cmp      byte ptr [rbp + 0x2a4], 0
0x1400b7b38 je       0x1400b7b6c
0x1400b7b3a mov      rax, qword ptr [rbp + 0x260]
0x1400b7b41 lock dec qword ptr [rax]
0x1400b7b45 jne      0x1400b7b53
0x1400b7b47 lea      rcx, [rbp + 0x260]
0x1400b7b4e call     0x140c41170
0x1400b7b53 mov      eax, dword ptr [r14 + 0x1c]
0x1400b7b57 mov      rcx, qword ptr [r14 + 0x10]
0x1400b7b5b mov      dword ptr [rcx], eax
0x1400b7b5d mov      rcx, rdi
0x1400b7b60 mov      edx, 0xffffffff
0x1400b7b65 call     0x140a84c40
0x1400b7b6a jmp      0x1400b7ba6
0x1400b7b6c mov      byte ptr [rbp + 0x38], 0
0x1400b7b70 cmp      byte ptr [rbp + 0x2a3], 0
0x1400b7b77 je       0x1400b7b8d
0x1400b7b79 lea      rcx, [rbp + 0x260]
0x1400b7b80 lea      rdx, [rbp + 0x38]
0x1400b7b84 call     0x140042870
0x1400b7b89 test     al, al
0x1400b7b8b jne      0x1400b7b3a
0x1400b7b8d mov      rax, qword ptr [rbp + 0x260]
0x1400b7b94 lock dec qword ptr [rax]
0x1400b7b98 jne      0x1400b7ba6
0x1400b7b9a lea      rcx, [rbp + 0x260]
0x1400b7ba1 call     0x140c41170
0x1400b7ba6 movzx    eax, byte ptr [rip + 0x357d05c]
0x1400b7bad mov      ecx, 0x24
0x1400b7bb2 mov      edx, 1
0x1400b7bb7 call     0x140613c10
0x1400b7bbc test     rax, rax
0x1400b7bbf je       0x1400b7c86
0x1400b7bc5 movups   xmm0, xmmword ptr [rip + 0x287d4cc] ; @0x142935098 ' trees into snapshotForce Music Richness to 1.0SourcesEvent filterClear filterAnimate LogoForce '
0x1400b7bcc movups   xmmword ptr [rax + 0x10], xmm0
0x1400b7bd0 movups   xmm0, xmmword ptr [rip + 0x287d4b1] ; @0x142935088 'Skip serializing trees into snapshotForce Music Richness to 1.0SourcesEvent filterClear filterAn'
0x1400b7bd7 movups   xmmword ptr [rax], xmm0
0x1400b7bda mov      dword ptr [rax + 0x20], 0x746f6873
0x1400b7be1 mov      qword ptr [rbp + 0x68], 0x24
0x1400b7be9 mov      qword ptr [rbp + 0x70], rax
0x1400b7bed mov      qword ptr [rbp + 0x78], 0x24
0x1400b7bf5 mov      dword ptr [rbp + 0x80], 0
0x1400b7bff mov      qword ptr [rbp + 0x90], 0
0x1400b7c0a mov      dword ptr [rbp + 0x88], 0
0x1400b7c14 mov      qword ptr [rbp + 0x38], 3
0x1400b7c1c mov      qword ptr [rbp + 0x50], 6
0x1400b7c24 mov      byte ptr [rbp + 0x98], 0
0x1400b7c2b mov      dword ptr [rbp + 0x9d], 0
0x1400b7c35 mov      dword ptr [rbp + 0xa0], 0
0x1400b7c3f mov      rax, qword ptr [rip + 0x357446a]
0x1400b7c46 mov      qword ptr [rbp + 0xa8], rax
0x1400b7c4d mov      byte ptr [rbp + 0xb0], 0
0x1400b7c54 lea      rcx, [rbp - 0x58]
0x1400b7c58 lea      rdx, [rbp + 0x38]
0x1400b7c5c mov      r8, rsi
0x1400b7c5f call     0x140c48930
0x1400b7c64 mov      rax, qword ptr [rbp - 0x48]
0x1400b7c68 lock dec qword ptr [rax]
0x1400b7c6c jne      0x1400b7c77
0x1400b7c6e lea      rcx, [rbp - 0x48]
0x1400b7c72 call     0x140c41170
0x1400b7c77 nop      
0x1400b7c78 add      rsp, 0x340
0x1400b7c7f pop      rbx
0x1400b7c80 pop      rdi
0x1400b7c81 pop      rsi
0x1400b7c82 pop      r14
0x1400b7c84 pop      rbp
0x1400b7c85 ret      
0x1400b7c86 lea      r8, [rip + 0x287d0bb]
0x1400b7c8d mov      ecx, 1
0x1400b7c92 mov      edx, 0x24
0x1400b7c97 call     0x1428d8fe3