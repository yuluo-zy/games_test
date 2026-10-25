0x1408b70d0 push     rbp
0x1408b70d1 push     r15
0x1408b70d3 push     r14
0x1408b70d5 push     rsi
0x1408b70d6 push     rdi
0x1408b70d7 push     rbx
0x1408b70d8 sub      rsp, 0x48
0x1408b70dc lea      rbp, [rsp + 0x40]
0x1408b70e1 mov      qword ptr [rbp], 0xfffffffffffffffe
0x1408b70e9 mov      rsi, rcx
0x1408b70ec mov      rdi, qword ptr [rcx + 0x110]
0x1408b70f3 test     rdi, rdi
0x1408b70f6 je       0x1408b7104
0x1408b70f8 mov      r14, qword ptr [rsi + 0x108]
0x1408b70ff jmp      0x1408b71d6 ; 
0x1408b7104 mov      rdi, qword ptr [rsi + 0xf8]
0x1408b710b test     rdi, rdi
0x1408b710e je       0x1408b714e
0x1408b7110 mov      r14, qword ptr [rsi]
0x1408b7113 mov      rbx, qword ptr [rsi + 8]
0x1408b7117 lea      r15, [rdi*8]
0x1408b711f movzx    eax, byte ptr [rip + 0x2d7dae3]
0x1408b7126 mov      edx, 8
0x1408b712b mov      rcx, r15
0x1408b712e call     0x140613c10 ; __rust_alloc
0x1408b7133 test     rax, rax
0x1408b7136 je       0x1408b71c2
0x1408b713c cmp      rdi, 3
0x1408b7140 mov      qword ptr [rbp - 0x10], r15
0x1408b7144 ja       0x1408b7169
0x1408b7146 xor      r8d, r8d
0x1408b7149 jmp      0x1408b71ff ; 
0x1408b714e mov      r14d, 8
0x1408b7154 mov      rdx, qword ptr [rsi + 0x100]
0x1408b715b test     rdx, rdx
0x1408b715e jne      0x1408b7253
0x1408b7164 jmp      0x1408b7269 ; 
0x1408b7169 movabs   r8, 0x3fffffffffffffc
0x1408b7173 and      r8, rdi
0x1408b7176 mov      rcx, r15
0x1408b7179 and      rcx, 0xffffffffffffffe0
0x1408b717d movdqa   xmm0, xmmword ptr [rip + 0x20f717b]
0x1408b7185 xor      edx, edx
0x1408b7187 movdqa   xmm1, xmmword ptr [rip + 0x20f7181]
0x1408b718f movdqa   xmm2, xmmword ptr [rip + 0x20f7189]
0x1408b7197 nop      word ptr [rax + rax]
0x1408b71a0 movdqa   xmm3, xmm0
0x1408b71a4 paddq    xmm3, xmm1
0x1408b71a8 movdqu   xmmword ptr [rax + rdx], xmm0
0x1408b71ad movdqu   xmmword ptr [rax + rdx + 0x10], xmm3
0x1408b71b3 paddq    xmm0, xmm2
0x1408b71b7 add      rdx, 0x20
0x1408b71bb cmp      rcx, rdx
0x1408b71be jne      0x1408b71a0
0x1408b71c0 jmp      0x1408b7206 ; 
0x1408b71c2 lea      r8, [rip + 0x21e8297]
0x1408b71c9 mov      ecx, 8
0x1408b71ce mov      rdx, r15
0x1408b71d1 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x1408b71d6 lea      rax, [rdi - 1]
0x1408b71da mov      qword ptr [rsi + 0x110], rax
0x1408b71e1 mov      rax, qword ptr [r14 + rdi*8 - 8]
0x1408b71e6 mov      ecx, 1
0x1408b71eb mov      qword ptr [rsi], rcx
0x1408b71ee mov      qword ptr [rsi + 8], rax
0x1408b71f2 add      rsp, 0x48
0x1408b71f6 pop      rbx
0x1408b71f7 pop      rdi
0x1408b71f8 pop      rsi
0x1408b71f9 pop      r14
0x1408b71fb pop      r15
0x1408b71fd pop      rbp
0x1408b71fe ret      
0x1408b71ff mov      qword ptr [rax + r8*8], r8
0x1408b7203 inc      r8
0x1408b7206 cmp      rdi, r8
0x1408b7209 jne      0x1408b71ff
0x1408b720b mov      qword ptr [rbp - 8], rax
0x1408b720f mov      qword ptr [rbp - 0x20], rax
0x1408b7213 mov      qword ptr [rbp - 0x18], rdi
0x1408b7217 lea      rcx, [rip + 0x21e82aa]
0x1408b721e lea      rdx, [rbp - 0x20]
0x1408b7222 call     0x14132b1a0 ; std::thread::local::LocalKey$LT$T$GT$::with::h345ce4518586922f
0x1408b7227 test     r14, r14
0x1408b722a je       0x1408b728e
0x1408b722c mov      r14, qword ptr [rbp - 8]
0x1408b7230 cmp      qword ptr [r14], rbx
0x1408b7233 jne      0x1408b7247
0x1408b7235 mov      rax, rdi
0x1408b7238 and      rax, 0xfffffffffffffffe
0x1408b723c mov      rcx, qword ptr [r14 + rax*4]
0x1408b7240 mov      qword ptr [r14], rcx
0x1408b7243 mov      qword ptr [r14 + rax*4], rbx
0x1408b7247 mov      rdx, qword ptr [rsi + 0x100]
0x1408b724e test     rdx, rdx
0x1408b7251 je       0x1408b7269
0x1408b7253 mov      rcx, qword ptr [rsi + 0x108]
0x1408b725a shl      rdx, 3
0x1408b725e mov      r8d, 8
0x1408b7264 call     0x140613c20 ; __rust_dealloc
0x1408b7269 mov      qword ptr [rsi + 0x100], rdi
0x1408b7270 mov      qword ptr [rsi + 0x108], r14
0x1408b7277 mov      qword ptr [rsi + 0x110], rdi
0x1408b727e test     rdi, rdi
0x1408b7281 jne      0x1408b71d6
0x1408b7287 xor      ecx, ecx
0x1408b7289 jmp      0x1408b71eb ; 
0x1408b728e mov      r14, qword ptr [rbp - 8]
0x1408b7292 mov      rdx, qword ptr [rsi + 0x100]
0x1408b7299 test     rdx, rdx
0x1408b729c jne      0x1408b7253
0x1408b729e jmp      0x1408b7269 ; 
0x1408b72a0 mov      qword ptr [rsp + 0x10], rdx
0x1408b72a5 push     rbp
0x1408b72a6 push     r15
0x1408b72a8 push     r14
0x1408b72aa push     rsi
0x1408b72ab push     rdi
0x1408b72ac push     rbx
0x1408b72ad sub      rsp, 0x28
0x1408b72b1 lea      rbp, [rdx + 0x40]
0x1408b72b5 mov      r8d, 8
0x1408b72bb mov      rcx, qword ptr [rbp - 8]
0x1408b72bf mov      rdx, qword ptr [rbp - 0x10]
0x1408b72c3 call     0x140613c20 ; __rust_dealloc
0x1408b72c8 nop      
0x1408b72c9 add      rsp, 0x28
0x1408b72cd pop      rbx
0x1408b72ce pop      rdi
0x1408b72cf pop      rsi
0x1408b72d0 pop      r14
0x1408b72d2 pop      r15
0x1408b72d4 pop      rbp
0x1408b72d5 ret      