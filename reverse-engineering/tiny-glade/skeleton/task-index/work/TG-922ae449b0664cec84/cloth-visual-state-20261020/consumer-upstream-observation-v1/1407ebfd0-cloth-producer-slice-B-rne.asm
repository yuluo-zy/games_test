1407ee2cb: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ee2d2: 8b48e4 mov ecx, dword ptr [rax - 0x1c]
1407ee2d5: e8667f4a00 call 0x140c96240 ; utils::rng::init_rng
1407ee2da: 4889c6 mov rsi, rax
1407ee2dd: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ee2e4: 6940e4b6aaaaaa imul eax, dword ptr [rax - 0x1c], 0xaaaaaab6
1407ee2eb: 054b885655 add eax, 0x5556884b
1407ee2f0: 3d55555555 cmp eax, 0x55555555
1407ee2f5: 0f87aa000000 ja 0x1407ee3a5
1407ee2fb: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ee302: 8078fc03 cmp byte ptr [rax - 4], 3
1407ee306: 0f8399000000 jae 0x1407ee3a5
1407ee30c: 488bbdf0010000 mov rdi, qword ptr [rbp + 0x1f0]
1407ee313: 0fb605ef68e402 movzx eax, byte ptr [rip + 0x2e468ef]
1407ee31a: b950000000 mov ecx, 0x50
1407ee31f: ba10000000 mov edx, 0x10
1407ee324: e8e758e2ff call 0x140613c10 ; unknown
1407ee329: 4885c0 test rax, rax
1407ee32c: 0f841c0b0000 je 0x1407eee4e
1407ee332: 4889c6 mov rsi, rax
1407ee335: 0f288510010000 movaps xmm0, xmmword ptr [rbp + 0x110]
1407ee33c: 0f288d20010000 movaps xmm1, xmmword ptr [rbp + 0x120]
1407ee343: 0f289530010000 movaps xmm2, xmmword ptr [rbp + 0x130]
1407ee34a: 0f295020 movaps xmmword ptr [rax + 0x20], xmm2
1407ee34e: 0f294810 movaps xmmword ptr [rax + 0x10], xmm1
1407ee352: 0f2900 movaps xmmword ptr [rax], xmm0
1407ee355: 48897830 mov qword ptr [rax + 0x30], rdi
1407ee359: f30f108590020000 movss xmm0, dword ptr [rbp + 0x290]
1407ee361: f30f114038 movss dword ptr [rax + 0x38], xmm0
1407ee366: f3440f11503c movss dword ptr [rax + 0x3c], xmm10
1407ee36c: 488b8578020000 mov rax, qword ptr [rbp + 0x278]
1407ee373: 894640 mov dword ptr [rsi + 0x40], eax
1407ee376: 488bbda8020000 mov rdi, qword ptr [rbp + 0x2a8]
1407ee37d: 488b8f10010000 mov rcx, qword ptr [rdi + 0x110]
1407ee384: 4885c9 test rcx, rcx
1407ee387: 7410 je 0x1407ee399
1407ee389: ba50000000 mov edx, 0x50
1407ee38e: 41b810000000 mov r8d, 0x10
1407ee394: e88758e2ff call 0x140613c20 ; unknown
1407ee399: 4889b710010000 mov qword ptr [rdi + 0x110], rsi
1407ee3a0: e929030000 jmp 0x1407ee6ce ; unknown
