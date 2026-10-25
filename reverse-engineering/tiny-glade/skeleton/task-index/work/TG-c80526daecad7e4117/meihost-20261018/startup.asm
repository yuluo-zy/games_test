0x141e9ce50 push rbp
0x141e9ce51 push r15
0x141e9ce53 push r14
0x141e9ce55 push r13
0x141e9ce57 push r12
0x141e9ce59 push rsi
0x141e9ce5a push rdi
0x141e9ce5b push rbx
0x141e9ce5c sub rsp, 0x318
0x141e9ce63 lea rbp, [rsp + 0x80]
0x141e9ce6b mov qword ptr [rbp + 0x290], 0xfffffffffffffffe
0x141e9ce76 mov rsi, rcx
0x141e9ce79 mov word ptr [rbp + 0x1f0], 0
0x141e9ce82 mov qword ptr [rbp + 0x1d8], 0xa
0x141e9ce8d mov qword ptr [rbp + 0x1e0], 0
0x141e9ce98 mov qword ptr [rbp + 0x1e8], 0xffffffffffffffff
0x141e9cea3 mov qword ptr [rbp + 0x1c8], 0
0x141e9ceae mov byte ptr [rbp + 0x1f2], 0
0x141e9ceb5 lea rdx, [rip + 0x10f864c] ; b'assets/fontsLoaded font: family  language '
0x141e9cebc lea rcx, [rbp + 0x40]
0x141e9cec0 mov r8d, 0xc
0x141e9cec6 call 0x140d61b00
0x141e9cecb lea rax, [rbp + 0x1f1]
0x141e9ced2 mov rcx, qword ptr [rbp + 0x1e8]
0x141e9ced9 mov qword ptr [rbp + 0x170], rcx
0x141e9cee0 movups xmm0, xmmword ptr [rbp + 0x1c8]
0x141e9cee7 movups xmm1, xmmword ptr [rbp + 0x1d8]
0x141e9ceee movups xmmword ptr [rbp + 0x160], xmm1
0x141e9cef5 movups xmmword ptr [rbp + 0x150], xmm0
0x141e9cefc mov ecx, dword ptr [rax]
0x141e9cefe mov eax, dword ptr [rax + 3]
0x141e9cf01 mov dword ptr [rbp + 0x179], ecx
0x141e9cf07 mov dword ptr [rbp + 0x17c], eax
0x141e9cf0d movups xmm0, xmmword ptr [rbp + 0x40]
0x141e9cf11 movups xmm1, xmmword ptr [rbp + 0x50]
0x141e9cf15 movups xmmword ptr [rbp + 0x130], xmm0
0x141e9cf1c movups xmmword ptr [rbp + 0x140], xmm1
0x141e9cf23 mov byte ptr [rbp + 0x178], 1
0x141e9cf2a xorps xmm0, xmm0
0x141e9cf2d movups xmmword ptr [rbp + 0xf8], xmm0
0x141e9cf34 movups xmmword ptr [rbp + 0x110], xmm0
0x141e9cf3b movups xmmword ptr [rbp + 0x180], xmm0
0x141e9cf42 mov qword ptr [rbp + 0xd8], 0
0x141e9cf4d mov qword ptr [rbp + 0xe8], 0
0x141e9cf58 mov qword ptr [rbp + 0xf0], 8
0x141e9cf63 mov qword ptr [rbp + 0x108], 8
0x141e9cf6e mov qword ptr [rbp + 0x120], 8
0x141e9cf79 mov qword ptr [rbp + 0x128], 0
0x141e9cf84 mov eax, dword ptr [rsi + 0x1c]
0x141e9cf87 mov dword ptr [rbp + 0x25c], eax
0x141e9cf8d mov rax, qword ptr [rsi]
0x141e9cf90 mov qword ptr [rbp + 0x1a8], rax
0x141e9cf97 mov rax, qword ptr [rsi + 0x10]
0x141e9cf9b mov qword ptr [rbp + 0xd0], rax
0x141e9cfa2 lea r12, [rbp - 0x50]
0x141e9cfa6 lea r13, [rbp + 0xd8]
0x141e9cfad lea r14, [rbp + 0x1c8]
0x141e9cfb4 lea rsi, [rbp + 0x260]
0x141e9cfbb lea rbx, [rip - 0x1e6e6f2]
0x141e9cfc2 nop word ptr cs:[rax + rax]
0x141e9cfd0 mov rcx, r12
0x141e9cfd3 mov rdx, r13
0x141e9cfd6 call 0x140fb3c30
0x141e9cfdb cmp dword ptr [rbp - 0x50], 3
0x141e9cfdf je 0x141e9d1e9
0x141e9cfe5 mov rcx, r14
0x141e9cfe8 mov rdx, rsi
0x141e9cfeb mov r8, r12
0x141e9cfee call 0x140fb6140
0x141e9cff3 mov rdi, qword ptr [rbp + 0x1c8]
0x141e9cffa cmp rdi, 2
0x141e9cffe je 0x141e9cfd0
0x141e9d000 mov r8d, 0x88
0x141e9d006 lea rcx, [rbp + 0x48]
0x141e9d00a lea rdx, [rbp + 0x1d0]
0x141e9d011 call 0x142923c00
0x141e9d016 mov qword ptr [rbp + 0x40], rdi
0x141e9d01a lea rcx, [rbp + 0x40]
0x141e9d01e call 0x140fc9fb0
0x141e9d023 mov rcx, rax
0x141e9d026 call 0x140d627a0
0x141e9d02b test rax, rax
0x141e9d02e je 0x141e9d1c2
0x141e9d034 mov r8, rdx
0x141e9d037 mov rcx, r14
0x141e9d03a mov rdx, rax
0x141e9d03d call 0x140d58e70
0x141e9d042 test byte ptr [rbp + 0x1c8], 1
0x141e9d049 jne 0x141e9d1c2
0x141e9d04f cmp qword ptr [rbp + 0x1d8], 3
0x141e9d057 jne 0x141e9d1c2
0x141e9d05d mov rax, qword ptr [rbp + 0x1d0]
0x141e9d064 movzx ecx, word ptr [rax]
0x141e9d067 xor ecx, 0x7474
0x141e9d06d movzx edx, byte ptr [rax + 2]
0x141e9d071 xor edx, 0x66
0x141e9d074 or dx, cx
0x141e9d077 je 0x141e9d092
0x141e9d079 movzx ecx, word ptr [rax]
0x141e9d07c xor ecx, 0x746f
0x141e9d082 movzx eax, byte ptr [rax + 2]
0x141e9d086 xor eax, 0x66
0x141e9d089 or ax, cx
0x141e9d08c jne 0x141e9d1c2
0x141e9d092 mov rax, qword ptr [rip + 0x178f30f]
0x141e9d099 mov rax, qword ptr [rax]
0x141e9d09c cmp rax, 5
0x141e9d0a0 jb 0x141e9d18c
0x141e9d0a6 lea rcx, [rbp + 0x40]
0x141e9d0aa call 0x140fc9fb0
0x141e9d0af mov qword ptr [rbp + 0x260], rax
0x141e9d0b6 mov qword ptr [rbp + 0x268], rdx
0x141e9d0bd mov qword ptr [rbp + 0x190], rsi
0x141e9d0c4 mov qword ptr [rbp + 0x198], rbx
0x141e9d0cb lea rcx, [rip + 0x10f84f6]
0x141e9d0d2 call 0x14067dd80
0x141e9d0d7 movups xmm0, xmmword ptr [rax]
0x141e9d0da mov eax, dword ptr [rax + 0x10]
0x141e9d0dd mov qword ptr [rbp + 0x1f8], 5
0x141e9d0e8 lea rcx, [rip + 0x10f84a9] ; b'toolbar::textLoading font file '
0x141e9d0ef mov qword ptr [rbp + 0x200], rcx
0x141e9d0f6 mov qword ptr [rbp + 0x208], 0xd
0x141e9d101 lea rdx, [rip + 0x10f84b0]
0x141e9d108 mov qword ptr [rbp + 0x218], rdx
0x141e9d10f mov qword ptr [rbp + 0x220], 1
0x141e9d11a lea rdx, [rbp + 0x190]
0x141e9d121 mov qword ptr [rbp + 0x228], rdx
0x141e9d128 mov qword ptr [rbp + 0x230], 1
0x141e9d133 mov qword ptr [rbp + 0x238], 0
0x141e9d13e mov qword ptr [rbp + 0x1c8], 0
0x141e9d149 mov qword ptr [rbp + 0x1d0], rcx
0x141e9d150 mov qword ptr [rbp + 0x1d8], 0xd
0x141e9d15b mov qword ptr [rbp + 0x1e0], 0
0x141e9d166 movups xmmword ptr [rbp + 0x1e8], xmm0
0x141e9d16d mov dword ptr [rbp + 0x210], 1
0x141e9d177 mov dword ptr [rbp + 0x214], eax
0x141e9d17d lea rcx, [rbp + 0x280]
0x141e9d184 mov rdx, r14
0x141e9d187 call 0x140d4a330
0x141e9d18c mov eax, dword ptr [rbp + 0x25c]
0x141e9d192 mov rcx, qword ptr [rbp + 0xd0]
0x141e9d199 mov dword ptr [rcx], eax
0x141e9d19b lea rcx, [rbp + 0x40]
0x141e9d19f call 0x140fc9fb0
0x141e9d1a4 mov r8, rdx
0x141e9d1a7 mov rcx, qword ptr [rbp + 0x1a8]
0x141e9d1ae mov rdx, rax
0x141e9d1b1 call 0x140b8f360
0x141e9d1b6 mov r15, rax
0x141e9d1b9 test rax, rax
0x141e9d1bc jne 0x141e9d5fd
0x141e9d1c2 mov rdx, qword ptr [rbp + 0x98]
0x141e9d1c9 test rdx, rdx
0x141e9d1cc je 0x141e9cfd0
0x141e9d1d2 mov rcx, qword ptr [rbp + 0xa0]
0x141e9d1d9 mov r8d, 1
0x141e9d1df call 0x140613c20
0x141e9d1e4 jmp 0x141e9cfd0
0x141e9d1e9 lea r14, [rbp + 0xd8]
0x141e9d1f0 mov rcx, r14
0x141e9d1f3 call 0x140fa4080
0x141e9d1f8 lea rcx, [rbp + 0x1c8]
0x141e9d1ff mov rdx, qword ptr [rbp + 0x1a8]
0x141e9d206 call 0x140b8f3a0
0x141e9d20b mov r15, qword ptr [rbp + 0x1d0]
0x141e9d212 mov rdi, qword ptr [rbp + 0x1e8]
0x141e9d219 lea r12, [rip + 0xd13a88]
0x141e9d220 mov rdx, rdi
0x141e9d223 test rdx, rdx
0x141e9d226 je 0x141e9d2db
0x141e9d22c cmp rdx, qword ptr [rbp + 0x1f0]
0x141e9d233 je 0x141e9d2d0
0x141e9d239 lea rdi, [rdx + 0x20]
0x141e9d23d mov qword ptr [rbp + 0x1e8], rdi
0x141e9d244 lea rcx, [rdx + 0x18]
0x141e9d248 mov r13, qword ptr [rdx + 8]
0x141e9d24c mov rbx, qword ptr [rdx + 0x10]
0x141e9d250 mov qword ptr [rbp + 0x40], 0
0x141e9d258 mov qword ptr [rbp + 0x48], 1
0x141e9d260 mov qword ptr [rbp + 0x50], 0
0x141e9d268 mov qword ptr [rbp + 0xd8], 0
0x141e9d273 mov qword ptr [rbp + 0xe8], 0
0x141e9d27e mov qword ptr [rbp + 0xf8], 0x20
0x141e9d289 mov byte ptr [rbp + 0x100], 3
0x141e9d290 lea rax, [rbp + 0x40]
0x141e9d294 mov qword ptr [rbp + 0x108], rax
0x141e9d29b mov qword ptr [rbp + 0x110], r12
0x141e9d2a2 mov rdx, r14
0x141e9d2a5 call 0x141e31d00
0x141e9d2aa test al, al
0x141e9d2ac jne 0x141e9d5cd
0x141e9d2b2 mov rdx, qword ptr [rbp + 0x40]
0x141e9d2b6 lea rax, [rbp + 0x48]
0x141e9d2ba movups xmm0, xmmword ptr [rax]
0x141e9d2bd movaps xmmword ptr [rbp + 0x260], xmm0
0x141e9d2c4 mov rax, rdx
0x141e9d2c7 neg rax
0x141e9d2ca jno 0x141e9d576
0x141e9d2d0 mov qword ptr [rbp + 0x1e8], 0
0x141e9d2db mov rax, qword ptr [rbp + 0x1c8]
0x141e9d2e2 test rax, rax
0x141e9d2e5 je 0x141e9d36e
0x141e9d2eb mov rcx, qword ptr [rbp + 0x1d8]
0x141e9d2f2 cmp rax, r15
0x141e9d2f5 je 0x141e9d360
0x141e9d2f7 nop word ptr [rax + rax]
0x141e9d300 lea rdx, [rax + 0x80]
0x141e9d307 inc rcx
0x141e9d30a test byte ptr [rax + 0x78], 1
0x141e9d30e jne 0x141e9d320
0x141e9d310 mov rax, rdx
0x141e9d313 cmp rdx, r15
0x141e9d316 jne 0x141e9d300
0x141e9d318 jmp 0x141e9d360
0x141e9d31a nop word ptr [rax + rax]
0x141e9d320 mov qword ptr [rbp + 0x1c8], rdx
0x141e9d327 mov qword ptr [rbp + 0x1d8], rcx
0x141e9d32e dec qword ptr [rbp + 0x1e0]
0x141e9d335 mov rdx, qword ptr [rax + 8]
0x141e9d339 mov rax, qword ptr [rax + 0x10]
0x141e9d33d shl rax, 5
0x141e9d341 add rax, rdx
0x141e9d344 mov qword ptr [rbp + 0x1e8], rdx
0x141e9d34b mov qword ptr [rbp + 0x1f0], rax
0x141e9d352 test rdx, rdx
0x141e9d355 jne 0x141e9d22c
0x141e9d35b jmp 0x141e9d2db
0x141e9d360 mov qword ptr [rbp + 0x1c8], rax
0x141e9d367 mov qword ptr [rbp + 0x1d8], rcx
0x141e9d36e mov rax, qword ptr [rbp + 0x1f8]
0x141e9d375 test rax, rax
0x141e9d378 je 0x141e9d589
0x141e9d37e cmp rax, qword ptr [rbp + 0x200]
0x141e9d385 je 0x141e9d589
0x141e9d38b lea rcx, [rax + 0x20]
0x141e9d38f mov qword ptr [rbp + 0x1f8], rcx
0x141e9d396 lea rcx, [rax + 0x18]
0x141e9d39a mov r13, qword ptr [rax + 8]
0x141e9d39e mov rbx, qword ptr [rax + 0x10]
0x141e9d3a2 mov qword ptr [rbp + 0x40], 0
0x141e9d3aa mov qword ptr [rbp + 0x48], 1
0x141e9d3b2 mov qword ptr [rbp + 0x50], 0
0x141e9d3ba mov qword ptr [rbp + 0xd8], 0
0x141e9d3c5 mov qword ptr [rbp + 0xe8], 0
0x141e9d3d0 mov qword ptr [rbp + 0xf8], 0x20
0x141e9d3db mov byte ptr [rbp + 0x100], 3
0x141e9d3e2 lea rax, [rbp + 0x40]
0x141e9d3e6 mov qword ptr [rbp + 0x108], rax
0x141e9d3ed mov qword ptr [rbp + 0x110], r12
0x141e9d3f4 mov rdx, r14
0x141e9d3f7 call 0x141e31d00
0x141e9d3fc test al, al
0x141e9d3fe jne 0x141e9d59d
0x141e9d404 mov rdx, qword ptr [rbp + 0x40]
0x141e9d408 lea rax, [rbp + 0x48]
0x141e9d40c movups xmm0, xmmword ptr [rax]
0x141e9d40f movaps xmmword ptr [rbp + 0x260], xmm0
0x141e9d416 mov rax, rdx
0x141e9d419 neg rax
0x141e9d41c jo 0x141e9d589
0x141e9d422 movaps xmm0, xmmword ptr [rbp + 0x260]
0x141e9d429 movaps xmmword ptr [rbp + 0x1b0], xmm0
0x141e9d430 xor edi, edi
0x141e9d432 mov qword ptr [rbp + 0x190], r13
0x141e9d439 mov qword ptr [rbp + 0x198], rbx
0x141e9d440 mov qword ptr [rbp + 0x260], rdx
0x141e9d447 movaps xmm0, xmmword ptr [rbp + 0x1b0]
0x141e9d44e lea rax, [rbp + 0x268]
0x141e9d455 movups xmmword ptr [rax], xmm0
0x141e9d458 mov rax, qword ptr [rip + 0x178ef49]
0x141e9d45f mov rax, qword ptr [rax]
0x141e9d462 cmp rax, 5
0x141e9d466 jb 0x141e9d556
0x141e9d46c lea rax, [rbp + 0x190]
0x141e9d473 mov qword ptr [rbp + 0x40], rax
0x141e9d477 lea rax, [rip - 0x1969bde]
0x141e9d47e mov qword ptr [rbp + 0x48], rax
0x141e9d482 mov qword ptr [rbp + 0x50], rsi
0x141e9d486 lea rax, [rip - 0x1dc009d]
0x141e9d48d mov qword ptr [rbp + 0x58], rax
0x141e9d491 lea rcx, [rip + 0x10f80e8]
0x141e9d498 call 0x14067dd80
0x141e9d49d movups xmm0, xmmword ptr [rax]
0x141e9d4a0 mov eax, dword ptr [rax + 0x10]
0x141e9d4a3 mov qword ptr [rbp + 0x108], 5
0x141e9d4ae lea rdx, [rip + 0x10f80e3] ; b'toolbar::textLoading font file '
0x141e9d4b5 mov qword ptr [rbp + 0x110], rdx
0x141e9d4bc mov qword ptr [rbp + 0x118], 0xd
0x141e9d4c7 lea rcx, [rip + 0x10f806a]
0x141e9d4ce mov qword ptr [rbp + 0x128], rcx
0x141e9d4d5 mov qword ptr [rbp + 0x130], 2
0x141e9d4e0 lea rcx, [rbp + 0x40]
0x141e9d4e4 mov qword ptr [rbp + 0x138], rcx
0x141e9d4eb mov qword ptr [rbp + 0x140], 2
0x141e9d4f6 mov qword ptr [rbp + 0x148], 0
0x141e9d501 mov qword ptr [rbp + 0xd8], 0
0x141e9d50c mov qword ptr [rbp + 0xe0], rdx
0x141e9d513 mov qword ptr [rbp + 0xe8], 0xd
0x141e9d51e mov qword ptr [rbp + 0xf0], 0
0x141e9d529 movups xmmword ptr [rbp + 0xf8], xmm0
0x141e9d530 mov dword ptr [rbp + 0x120], 1
0x141e9d53a mov dword ptr [rbp + 0x124], eax
0x141e9d540 lea rcx, [rbp + 0x280]
0x141e9d547 mov rdx, r14
0x141e9d54a call 0x140d4a330
0x141e9d54f mov rdx, qword ptr [rbp + 0x260]
0x141e9d556 test rdx, rdx
0x141e9d559 je 0x141e9d220
0x141e9d55f mov rcx, qword ptr [rbp + 0x268]
0x141e9d566 mov r8d, 1
0x141e9d56c call 0x140613c20
0x141e9d571 jmp 0x141e9d220
0x141e9d576 movaps xmm0, xmmword ptr [rbp + 0x260]
0x141e9d57d movaps xmmword ptr [rbp + 0x1b0], xmm0
0x141e9d584 jmp 0x141e9d432
0x141e9d589 add rsp, 0x318
0x141e9d590 pop rbx
0x141e9d591 pop rdi
0x141e9d592 pop rsi
0x141e9d593 pop r12
0x141e9d595 pop r13
0x141e9d597 pop r14
0x141e9d599 pop r15
0x141e9d59b pop rbp
0x141e9d59c ret 
0x141e9d59d lea rax, [rip + 0xd137bc]
0x141e9d5a4 mov qword ptr [rsp + 0x20], rax
0x141e9d5a9 lea rcx, [rip + 0xd13728] ; b'a Display implementation returned an error unexpectedly/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\alloc\\src\\string.rs'
0x141e9d5b0 lea r9, [rip + 0xd137c1]
0x141e9d5b7 lea r8, [rbp + 0x280]
0x141e9d5be mov edx, 0x37
0x141e9d5c3 call 0x1428d9760
0x141e9d5c8 jmp 0x141e9d701
0x141e9d5cd lea rax, [rip + 0xd1378c]
0x141e9d5d4 mov qword ptr [rsp + 0x20], rax
0x141e9d5d9 lea rcx, [rip + 0xd136f8] ; b'a Display implementation returned an error unexpectedly/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\alloc\\src\\string.rs'
0x141e9d5e0 lea r9, [rip + 0xd13791]
0x141e9d5e7 lea r8, [rbp + 0x280]
0x141e9d5ee mov edx, 0x37
0x141e9d5f3 call 0x1428d9760
0x141e9d5f8 jmp 0x141e9d701
0x141e9d5fd mov qword ptr [rbp + 0x280], r15
0x141e9d604 mov byte ptr [rbp + 0x28f], 1
0x141e9d60b lea rcx, [rbp + 0x40]
0x141e9d60f call 0x140fc9fb0
0x141e9d614 mov qword ptr [rbp + 0x260], rax
0x141e9d61b mov qword ptr [rbp + 0x268], rdx
0x141e9d622 mov qword ptr [rbp + 0x1b0], rsi
0x141e9d629 mov qword ptr [rbp + 0x1b8], rbx
0x141e9d630 lea rax, [rip + 0x11b69e1]
0x141e9d637 mov qword ptr [rbp + 0x1c8], rax
0x141e9d63e mov qword ptr [rbp + 0x1d0], 1
0x141e9d649 lea rax, [rbp + 0x1b0]
0x141e9d650 mov qword ptr [rbp + 0x1d8], rax
0x141e9d657 mov qword ptr [rbp + 0x1e0], 1
0x141e9d662 mov qword ptr [rbp + 0x1e8], 0
0x141e9d66d mov byte ptr [rbp + 0x28f], 1
0x141e9d674 lea rcx, [rbp + 0x190]
0x141e9d67b lea rdx, [rbp + 0x1c8]
0x141e9d682 call 0x140d79f90
0x141e9d687 mov rax, qword ptr [rbp + 0x1a0]
0x141e9d68e mov qword ptr [rbp + 0x270], rax
0x141e9d695 movups xmm0, xmmword ptr [rbp + 0x190]
0x141e9d69c movaps xmmword ptr [rbp + 0x260], xmm0
0x141e9d6a3 mov qword ptr [rbp + 0x278], r15
0x141e9d6aa mov qword ptr [rbp + 0x1c8], 3
0x141e9d6b5 mov byte ptr [rbp + 0x28f], 0
0x141e9d6bc lea rcx, [rbp + 0x260]
0x141e9d6c3 lea rdx, [rbp + 0x1c8]
0x141e9d6ca call 0x1428e5af0
0x141e9d6cf mov qword ptr [rbp + 0x1c8], rax
0x141e9d6d6 lea rax, [rip + 0x10f7f03]
0x141e9d6dd mov qword ptr [rsp + 0x20], rax
0x141e9d6e2 lea rcx, [rip + 0x10f7cff] ; b'called `Result::unwrap()` on an `Err` valuetoolbar::text::update_text_renderer_scale::f'
0x141e9d6e9 lea r9, [rip + 0x10f7cd8]
0x141e9d6f0 lea r8, [rbp + 0x1c8]
0x141e9d6f7 mov edx, 0x2b
0x141e9d6fc call 0x1428d9760
0x141e9d701 ud2 
0x141e9d703 nop word ptr cs:[rax + rax]
0x141e9d710 mov qword ptr [rsp + 0x10], rdx
0x141e9d715 push rbp
0x141e9d716 push r15
0x141e9d718 push r14
0x141e9d71a push r13
0x141e9d71c push r12
0x141e9d71e push rsi
0x141e9d71f push rdi
0x141e9d720 push rbx
0x141e9d721 sub rsp, 0x28
0x141e9d725 lea rbp, [rdx + 0x80]
0x141e9d72c lea rcx, [rbp + 0xd8]
0x141e9d733 call 0x140fa4080
0x141e9d738 nop 
0x141e9d739 add rsp, 0x28
0x141e9d73d pop rbx
0x141e9d73e pop rdi
0x141e9d73f pop rsi
0x141e9d740 pop r12
0x141e9d742 pop r13
0x141e9d744 pop r14
0x141e9d746 pop r15
0x141e9d748 pop rbp
0x141e9d749 ret 
0x141e9d74a nop word ptr [rax + rax]
0x141e9d750 mov qword ptr [rsp + 0x10], rdx
0x141e9d755 push rbp
0x141e9d756 push r15
0x141e9d758 push r14
0x141e9d75a push r13
0x141e9d75c push r12
0x141e9d75e push rsi
0x141e9d75f push rdi
0x141e9d760 push rbx
0x141e9d761 sub rsp, 0x28
0x141e9d765 lea rbp, [rdx + 0x80]
0x141e9d76c mov rdx, qword ptr [rbp + 0x40]
0x141e9d770 test rdx, rdx
0x141e9d773 je 0x141e9d784
0x141e9d775 mov rcx, qword ptr [rbp + 0x48]
0x141e9d779 mov r8d, 1
0x141e9d77f call 0x140613c20
0x141e9d784 nop 
0x141e9d785 add rsp, 0x28
0x141e9d789 pop rbx
0x141e9d78a pop rdi
0x141e9d78b pop rsi
0x141e9d78c pop r12
0x141e9d78e pop r13
0x141e9d790 pop r14
0x141e9d792 pop r15
0x141e9d794 pop rbp
0x141e9d795 ret 
0x141e9d796 nop word ptr cs:[rax + rax]
0x141e9d7a0 mov qword ptr [rsp + 0x10], rdx
0x141e9d7a5 push rbp
0x141e9d7a6 push r15
0x141e9d7a8 push r14
0x141e9d7aa push r13
0x141e9d7ac push r12
0x141e9d7ae push rsi
0x141e9d7af push rdi
0x141e9d7b0 push rbx
0x141e9d7b1 sub rsp, 0x28
0x141e9d7b5 lea rbp, [rdx + 0x80]
0x141e9d7bc mov rdx, qword ptr [rbp + 0x40]
0x141e9d7c0 test rdx, rdx
0x141e9d7c3 je 0x141e9d7d4
0x141e9d7c5 mov rcx, qword ptr [rbp + 0x48]
0x141e9d7c9 mov r8d, 1
0x141e9d7cf call 0x140613c20
0x141e9d7d4 nop 
0x141e9d7d5 add rsp, 0x28
0x141e9d7d9 pop rbx
0x141e9d7da pop rdi
0x141e9d7db pop rsi
0x141e9d7dc pop r12
0x141e9d7de pop r13
0x141e9d7e0 pop r14
0x141e9d7e2 pop r15
0x141e9d7e4 pop rbp
0x141e9d7e5 ret 
0x141e9d7e6 nop word ptr cs:[rax + rax]
0x141e9d7f0 mov qword ptr [rsp + 0x10], rdx
0x141e9d7f5 push rbp
0x141e9d7f6 push r15
0x141e9d7f8 push r14
0x141e9d7fa push r13
0x141e9d7fc push r12
0x141e9d7fe push rsi
0x141e9d7ff push rdi
0x141e9d800 push rbx
0x141e9d801 sub rsp, 0x28
0x141e9d805 lea rbp, [rdx + 0x80]
0x141e9d80c mov rdx, qword ptr [rbp + 0x260]
0x141e9d813 test rdx, rdx
0x141e9d816 je 0x141e9d82a
0x141e9d818 mov rcx, qword ptr [rbp + 0x268]
0x141e9d81f mov r8d, 1
0x141e9d825 call 0x140613c20
0x141e9d82a nop 
0x141e9d82b add rsp, 0x28
0x141e9d82f pop rbx
0x141e9d830 pop rdi
0x141e9d831 pop rsi
0x141e9d832 pop r12
0x141e9d834 pop r13
0x141e9d836 pop r14
0x141e9d838 pop r15
0x141e9d83a pop rbp
0x141e9d83b ret 
0x141e9d83c nop dword ptr [rax]
0x141e9d840 mov qword ptr [rsp + 0x10], rdx
0x141e9d845 push rbp
0x141e9d846 push r15
0x141e9d848 push r14
0x141e9d84a push r13
0x141e9d84c push r12
0x141e9d84e push rsi
0x141e9d84f push rdi
0x141e9d850 push rbx
0x141e9d851 sub rsp, 0x28
0x141e9d855 lea rbp, [rdx + 0x80]
0x141e9d85c mov rdx, qword ptr [rbp + 0x98]
0x141e9d863 test rdx, rdx
0x141e9d866 je 0x141e9d87a
0x141e9d868 mov rcx, qword ptr [rbp + 0xa0]
0x141e9d86f mov r8d, 1
0x141e9d875 call 0x140613c20
0x141e9d87a nop 
0x141e9d87b add rsp, 0x28
0x141e9d87f pop rbx
0x141e9d880 pop rdi
0x141e9d881 pop rsi
0x141e9d882 pop r12
0x141e9d884 pop r13
0x141e9d886 pop r14
0x141e9d888 pop r15
0x141e9d88a pop rbp
0x141e9d88b ret 
0x141e9d88c nop dword ptr [rax]
0x141e9d890 mov qword ptr [rsp + 0x10], rdx
0x141e9d895 push rbp
0x141e9d896 push r15
0x141e9d898 push r14
0x141e9d89a push r13
0x141e9d89c push r12
0x141e9d89e push rsi
0x141e9d89f push rdi
0x141e9d8a0 push rbx
0x141e9d8a1 sub rsp, 0x28
0x141e9d8a5 lea rbp, [rdx + 0x80]
0x141e9d8ac cmp byte ptr [rbp + 0x28f], 0
0x141e9d8b3 je 0x141e9d8c1
0x141e9d8b5 lea rcx, [rbp + 0x280]
0x141e9d8bc call 0x140d4cc30
0x141e9d8c1 nop 
0x141e9d8c2 add rsp, 0x28
0x141e9d8c6 pop rbx
0x141e9d8c7 pop rdi
0x141e9d8c8 pop rsi
0x141e9d8c9 pop r12
0x141e9d8cb pop r13
0x141e9d8cd pop r14
0x141e9d8cf pop r15
0x141e9d8d1 pop rbp
0x141e9d8d2 ret 
0x141e9d8d3 nop word ptr cs:[rax + rax]
0x141e9d8e0 mov qword ptr [rsp + 0x10], rdx
0x141e9d8e5 push rbp
0x141e9d8e6 push r15
0x141e9d8e8 push r14
0x141e9d8ea push r13
0x141e9d8ec push r12
0x141e9d8ee push rsi
0x141e9d8ef push rdi
0x141e9d8f0 push rbx
0x141e9d8f1 sub rsp, 0x28
0x141e9d8f5 lea rbp, [rdx + 0x80]
0x141e9d8fc lea rcx, [rbp + 0x1c8]
0x141e9d903 call 0x140d4cc30
0x141e9d908 nop 
0x141e9d909 add rsp, 0x28
0x141e9d90d pop rbx
0x141e9d90e pop rdi
0x141e9d90f pop rsi
0x141e9d910 pop r12
0x141e9d912 pop r13
0x141e9d914 pop r14
0x141e9d916 pop r15
0x141e9d918 pop rbp
0x141e9d919 ret 
0x141e9d91a nop word ptr [rax + rax]
0x141e9d920 mov qword ptr [rsp + 0x10], rdx
0x141e9d925 push rbp
0x141e9d926 push r15
0x141e9d928 push r14
0x141e9d92a push r13
0x141e9d92c push r12
0x141e9d92e push rsi
0x141e9d92f push rdi
0x141e9d930 push rbx
0x141e9d931 sub rsp, 0x28
0x141e9d935 lea rbp, [rdx + 0x80]
0x141e9d93c lea rcx, [rbp + 0x1c8]
0x141e9d943 call 0x1405dc880
0x141e9d948 nop 
0x141e9d949 add rsp, 0x28
0x141e9d94d pop rbx
0x141e9d94e pop rdi
0x141e9d94f pop rsi
0x141e9d950 pop r12
0x141e9d952 pop r13
0x141e9d954 pop r14
0x141e9d956 pop r15
0x141e9d958 pop rbp
0x141e9d959 ret 