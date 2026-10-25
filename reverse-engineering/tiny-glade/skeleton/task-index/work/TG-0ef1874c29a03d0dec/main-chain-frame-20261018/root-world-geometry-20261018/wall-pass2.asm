14098b2a0 55 push rbp
14098b2a1 4157 push r15
14098b2a3 4156 push r14
14098b2a5 4155 push r13
14098b2a7 4154 push r12
14098b2a9 56 push rsi
14098b2aa 57 push rdi
14098b2ab 53 push rbx
14098b2ac 4881ece8060000 sub rsp, 0x6e8
14098b2b3 488dac2480000000 lea rbp, [rsp + 0x80]
14098b2bb 48c78560060000feffffff mov qword ptr [rbp + 0x660], 0xfffffffffffffffe
14098b2c6 4c89c6 mov rsi, r8
14098b2c9 4889d7 mov rdi, rdx
14098b2cc 4889cb mov rbx, rcx
14098b2cf 48894c2430 mov qword ptr [rsp + 0x30], rcx
14098b2d4 4889542428 mov qword ptr [rsp + 0x28], rdx
14098b2d9 48c744242025000000 mov qword ptr [rsp + 0x20], 0x25
14098b2e2 488d1539dc1202 lea rdx, [rip + 0x212dc39]
14098b2e9 4c8d0d4edc1202 lea r9, [rip + 0x212dc4e]
14098b2f0 4c8db570040000 lea r14, [rbp + 0x470]
14098b2f7 41b81c000000 mov r8d, 0x1c
14098b2fd 4c89f1 mov rcx, r14
14098b300 e89b7fc700 call 0x1416032a0
14098b305 488d8de0020000 lea rcx, [rbp + 0x2e0]
14098b30c 49b85a7612802b7ac0f1 movabs r8, 0xf1c07a2b8012765a
14098b316 4c89f2 mov rdx, r14
14098b319 e892c2c700 call 0x1416075b0
14098b31e 80bd1604000008 cmp byte ptr [rbp + 0x416], 8
14098b325 7428 je 0x14098b34f
14098b327 488d8df0030000 lea rcx, [rbp + 0x3f0]
14098b32e bae8000000 mov edx, 0xe8
14098b333 48039518040000 add rdx, qword ptr [rbp + 0x418]
14098b33a 49b88957b716a611950e movabs r8, 0xe9511a616b75789
14098b344 41b911000000 mov r9d, 0x11
14098b34a e82102feff call 0x14096b570
14098b34f 488d8d70040000 lea rcx, [rbp + 0x470]
14098b356 488d95e0020000 lea rdx, [rbp + 0x2e0]
14098b35d 41b890010000 mov r8d, 0x190
14098b363 e89888f901 call 0x142923c00
14098b368 0f108510050000 movups xmm0, xmmword ptr [rbp + 0x510]
14098b36f 0f298590010000 movaps xmmword ptr [rbp + 0x190], xmm0
14098b376 0f108500050000 movups xmm0, xmmword ptr [rbp + 0x500]
14098b37d 0f298580010000 movaps xmmword ptr [rbp + 0x180], xmm0
14098b384 0f1085f0040000 movups xmm0, xmmword ptr [rbp + 0x4f0]
14098b38b 0f298570010000 movaps xmmword ptr [rbp + 0x170], xmm0
14098b392 0f1085e0040000 movups xmm0, xmmword ptr [rbp + 0x4e0]
14098b399 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
14098b3a0 0f1085d0040000 movups xmm0, xmmword ptr [rbp + 0x4d0]
14098b3a7 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
14098b3ae 488b85a8050000 mov rax, qword ptr [rbp + 0x5a8]
14098b3b5 488b8df0050000 mov rcx, qword ptr [rbp + 0x5f0]
14098b3bc 488b9580040000 mov rdx, qword ptr [rbp + 0x480]
14098b3c3 488995f0020000 mov qword ptr [rbp + 0x2f0], rdx
14098b3ca 0f108570040000 movups xmm0, xmmword ptr [rbp + 0x470]
14098b3d1 0f2985e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm0
14098b3d8 0f108588040000 movups xmm0, xmmword ptr [rbp + 0x488]
14098b3df 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
14098b3e6 488b9598040000 mov rdx, qword ptr [rbp + 0x498]
14098b3ed 48899520030000 mov qword ptr [rbp + 0x320], rdx
14098b3f4 0f1085a0040000 movups xmm0, xmmword ptr [rbp + 0x4a0]
14098b3fb 0f1185f8020000 movups xmmword ptr [rbp + 0x2f8], xmm0
14098b402 488b95b0040000 mov rdx, qword ptr [rbp + 0x4b0]
14098b409 48899508030000 mov qword ptr [rbp + 0x308], rdx
14098b410 0f108520050000 movups xmm0, xmmword ptr [rbp + 0x520]
14098b417 0f108d30050000 movups xmm1, xmmword ptr [rbp + 0x530]
14098b41e 0f109540050000 movups xmm2, xmmword ptr [rbp + 0x540]
14098b425 0f109d50050000 movups xmm3, xmmword ptr [rbp + 0x550]
14098b42c 0f118558030000 movups xmmword ptr [rbp + 0x358], xmm0
14098b433 0f118d68030000 movups xmmword ptr [rbp + 0x368], xmm1
14098b43a 0f119578030000 movups xmmword ptr [rbp + 0x378], xmm2
14098b441 0f119d88030000 movups xmmword ptr [rbp + 0x388], xmm3
14098b448 0f108560050000 movups xmm0, xmmword ptr [rbp + 0x560]
14098b44f 0f118598030000 movups xmmword ptr [rbp + 0x398], xmm0
14098b456 0f108570050000 movups xmm0, xmmword ptr [rbp + 0x570]
14098b45d 0f1185a8030000 movups xmmword ptr [rbp + 0x3a8], xmm0
14098b464 488b95c8040000 mov rdx, qword ptr [rbp + 0x4c8]
14098b46b 48899538030000 mov qword ptr [rbp + 0x338], rdx
14098b472 0f1085b8040000 movups xmm0, xmmword ptr [rbp + 0x4b8]
14098b479 0f118528030000 movups xmmword ptr [rbp + 0x328], xmm0
14098b480 48898dc0030000 mov qword ptr [rbp + 0x3c0], rcx
14098b487 488985b8030000 mov qword ptr [rbp + 0x3b8], rax
14098b48e 0f28056bcf1202 movaps xmm0, xmmword ptr [rip + 0x212cf6b]
14098b495 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
14098b49c 488d8d50010000 lea rcx, [rbp + 0x150]
14098b4a3 488d95e0020000 lea rdx, [rbp + 0x2e0]
14098b4aa e821200500 call 0x1409dd4d0
14098b4af 80bda605000008 cmp byte ptr [rbp + 0x5a6], 8
14098b4b6 740c je 0x14098b4c4
14098b4b8 488d8d90050000 lea rcx, [rbp + 0x590]
14098b4bf e8ec6f2500 call 0x140be24b0
14098b4c4 4c8d6738 lea r12, [rdi + 0x38]
14098b4c8 48ba5593676e78cc2682 movabs rdx, 0x8226cc786e679355
14098b4d2 49b8ac41a8556adacf65 movabs r8, 0x65cfda6a55a841ac
14098b4dc 4c89e1 mov rcx, r12
14098b4df e80c2275ff call 0x1400dd6f0
14098b4e4 4885c0 test rax, rax
14098b4e7 0f84e3060000 je 0x14098bbd0
14098b4ed 48399718020000 cmp qword ptr [rdi + 0x218], rdx
14098b4f4 0f86d6060000 jbe 0x14098bbd0
14098b4fa 488b8710020000 mov rax, qword ptr [rdi + 0x210]
14098b501 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
14098b505 4885c9 test rcx, rcx
14098b508 0f84c2060000 je 0x14098bbd0
14098b50e 488b87e0010000 mov rax, qword ptr [rdi + 0x1e0]
14098b515 48f7d1 not rcx
14098b518 488d0c49 lea rcx, [rcx + rcx*2]
14098b51c 48c1e105 shl rcx, 5
14098b520 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
14098b526 0f84a4060000 je 0x14098bbd0
14098b52c 4889b500060000 mov qword ptr [rbp + 0x600], rsi
14098b533 4801c8 add rax, rcx
14098b536 4c8b7010 mov r14, qword ptr [rax + 0x10]
14098b53a 8b8fb0050000 mov ecx, dword ptr [rdi + 0x5b0]
14098b540 89485c mov dword ptr [rax + 0x5c], ecx
14098b543 4c89f1 mov rcx, r14
14098b546 e865bc9300 call 0x1412c71b0
14098b54b 4885c0 test rax, rax
14098b54e 0f84a4060000 je 0x14098bbf8
14098b554 4989c7 mov r15, rax
14098b557 c6855f06000001 mov byte ptr [rbp + 0x65f], 1
14098b55e c6855e06000001 mov byte ptr [rbp + 0x65e], 1
14098b565 48ba56f7feea51bd0d85 movabs rdx, 0x850dbd51eafef756
14098b56f 49b8354c85394b2a7885 movabs r8, 0x85782a4b39854c35
14098b579 4c89e1 mov rcx, r12
14098b57c e86f2175ff call 0x1400dd6f0
14098b581 4885c0 test rax, rax
14098b584 0f8452060000 je 0x14098bbdc
14098b58a 48399718020000 cmp qword ptr [rdi + 0x218], rdx
14098b591 0f8645060000 jbe 0x14098bbdc
14098b597 488b8710020000 mov rax, qword ptr [rdi + 0x210]
14098b59e 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
14098b5a2 4885c9 test rcx, rcx
14098b5a5 0f8431060000 je 0x14098bbdc
14098b5ab 488b87e0010000 mov rax, qword ptr [rdi + 0x1e0]
14098b5b2 48f7d1 not rcx
14098b5b5 488d0c49 lea rcx, [rcx + rcx*2]
14098b5b9 48c1e105 shl rcx, 5
14098b5bd 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
14098b5c3 0f8413060000 je 0x14098bbdc
14098b5c9 4801c8 add rax, rcx
14098b5cc 488b4010 mov rax, qword ptr [rax + 0x10]
14098b5d0 80781000 cmp byte ptr [rax + 0x10], 0
14098b5d4 0f84a5010000 je 0x14098b77f
14098b5da 488b00 mov rax, qword ptr [rax]
14098b5dd 89c1 mov ecx, eax
14098b5df 483b4f10 cmp rcx, qword ptr [rdi + 0x10]
14098b5e3 0f8396010000 jae 0x14098b77f
14098b5e9 488b5708 mov rdx, qword ptr [rdi + 8]
14098b5ed 48c1e820 shr rax, 0x20
14098b5f1 488d0c89 lea rcx, [rcx + rcx*4]
14098b5f5 39048a cmp dword ptr [rdx + rcx*4], eax
14098b5f8 0f8581010000 jne 0x14098b77f
14098b5fe 488d048a lea rax, [rdx + rcx*4]
14098b602 837804ff cmp dword ptr [rax + 4], -1
14098b606 0f8473010000 je 0x14098b77f
14098b60c 448b680c mov r13d, dword ptr [rax + 0xc]
14098b610 8b7010 mov esi, dword ptr [rax + 0x10]
14098b613 c6855f06000001 mov byte ptr [rbp + 0x65f], 1
14098b61a c6855e06000001 mov byte ptr [rbp + 0x65e], 1
14098b621 48ba52fa262202bc6e3d movabs rdx, 0x3d6ebc022226fa52
14098b62b 49b89dc3237af05805b2 movabs r8, 0xb20558f07a23c39d
14098b635 4c89e1 mov rcx, r12
14098b638 e863bbffff call 0x1409871a0
14098b63d 4885c0 test rax, rax
14098b640 0f8439010000 je 0x14098b77f
14098b646 4c39afb0010000 cmp qword ptr [rdi + 0x1b0], r13
14098b64d 0f862c010000 jbe 0x14098b77f
14098b653 4a8d0ced00000000 lea rcx, [r13*8]
14098b65b 4c01e9 add rcx, r13
14098b65e 48c1e103 shl rcx, 3
14098b662 48038fa8010000 add rcx, qword ptr [rdi + 0x1a8]
14098b669 c6855f06000001 mov byte ptr [rbp + 0x65f], 1
14098b670 c6855e06000001 mov byte ptr [rbp + 0x65e], 1
14098b677 4189f0 mov r8d, esi
14098b67a e8d1543700 call 0x140d00b50
14098b67f 4989c4 mov r12, rax
14098b682 4885c0 test rax, rax
14098b685 0f84f4000000 je 0x14098b77f
14098b68b 49837c242000 cmp qword ptr [r12 + 0x20], 0
14098b691 0f846d050000 je 0x14098bc04
14098b697 4983c420 add r12, 0x20
14098b69b c6855f06000001 mov byte ptr [rbp + 0x65f], 1
14098b6a2 c6855e06000001 mov byte ptr [rbp + 0x65e], 1
14098b6a9 488d8d70040000 lea rcx, [rbp + 0x470]
14098b6b0 4c89fa mov rdx, r15
14098b6b3 4d89e0 mov r8, r12
14098b6b6 e8a5839500 call 0x1412e3a60
14098b6bb 4c8bad70040000 mov r13, qword ptr [rbp + 0x470]
14098b6c2 488bb578040000 mov rsi, qword ptr [rbp + 0x478]
14098b6c9 488b8580040000 mov rax, qword ptr [rbp + 0x480]
14098b6d0 48898508060000 mov qword ptr [rbp + 0x608], rax
14098b6d7 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098b6de 4c89ad48060000 mov qword ptr [rbp + 0x648], r13
14098b6e5 c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098b6ec c6855e06000001 mov byte ptr [rbp + 0x65e], 1
14098b6f3 4c89f9 mov rcx, r15
14098b6f6 4c89e2 mov rdx, r12
14098b6f9 e802859500 call 0x1412e3c00
14098b6fe 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098b705 4c89ad48060000 mov qword ptr [rbp + 0x648], r13
14098b70c c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098b713 c6855e06000001 mov byte ptr [rbp + 0x65e], 1
14098b71a 488d8d70040000 lea rcx, [rbp + 0x470]
14098b721 4c89fa mov rdx, r15
14098b724 e8278b9500 call 0x1412e4250
14098b729 488b8570040000 mov rax, qword ptr [rbp + 0x470]
14098b730 4c89e2 mov rdx, r12
14098b733 4c8ba578040000 mov r12, qword ptr [rbp + 0x478]
14098b73a 488b8d80040000 mov rcx, qword ptr [rbp + 0x480]
14098b741 48898d10060000 mov qword ptr [rbp + 0x610], rcx
14098b748 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098b74f 4c89ad48060000 mov qword ptr [rbp + 0x648], r13
14098b756 4c89a540060000 mov qword ptr [rbp + 0x640], r12
14098b75d 48898538060000 mov qword ptr [rbp + 0x638], rax
14098b764 c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098b76b c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098b772 4c89f9 mov rcx, r15
14098b775 4989c7 mov r15, rax
14098b778 e803859500 call 0x1412e3c80
14098b77d eb4c jmp 0x14098b7cb
14098b77f c6855f06000001 mov byte ptr [rbp + 0x65f], 1
14098b786 c6855e06000001 mov byte ptr [rbp + 0x65e], 1
14098b78d 488d8d70040000 lea rcx, [rbp + 0x470]
14098b794 4c89fa mov rdx, r15
14098b797 e8b48a9500 call 0x1412e4250
14098b79c 4c8bbd70040000 mov r15, qword ptr [rbp + 0x470]
14098b7a3 4c8ba578040000 mov r12, qword ptr [rbp + 0x478]
14098b7aa 488b8580040000 mov rax, qword ptr [rbp + 0x480]
14098b7b1 48898510060000 mov qword ptr [rbp + 0x610], rax
14098b7b8 be04000000 mov esi, 4
14098b7bd 48c7850806000000000000 mov qword ptr [rbp + 0x608], 0
14098b7c8 4531ed xor r13d, r13d
14098b7cb 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098b7d2 4c89ad48060000 mov qword ptr [rbp + 0x648], r13
14098b7d9 4c89a540060000 mov qword ptr [rbp + 0x640], r12
14098b7e0 4c89bd38060000 mov qword ptr [rbp + 0x638], r15
14098b7e7 c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098b7ee c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098b7f5 488d8d70040000 lea rcx, [rbp + 0x470]
14098b7fc 4c89f2 mov rdx, r14
14098b7ff 4989d8 mov r8, rbx
14098b802 e849569400 call 0x1412d0e50
14098b807 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098b80e 4c89ad48060000 mov qword ptr [rbp + 0x648], r13
14098b815 4c89a540060000 mov qword ptr [rbp + 0x640], r12
14098b81c 4c89bd38060000 mov qword ptr [rbp + 0x638], r15
14098b823 c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098b82a c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098b831 4c8d8570040000 lea r8, [rbp + 0x470]
14098b838 4889d9 mov rcx, rbx
14098b83b 4889fa mov rdx, rdi
14098b83e e86d230500 call 0x1409ddbb0
14098b843 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098b84a 4c89ad48060000 mov qword ptr [rbp + 0x648], r13
14098b851 4c89a540060000 mov qword ptr [rbp + 0x640], r12
14098b858 4c89bd38060000 mov qword ptr [rbp + 0x638], r15
14098b85f c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098b866 c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098b86d 48895c2430 mov qword ptr [rsp + 0x30], rbx
14098b872 48897c2428 mov qword ptr [rsp + 0x28], rdi
14098b877 48c74424202a000000 mov qword ptr [rsp + 0x20], 0x2a
14098b880 488d1541d71202 lea rdx, [rip + 0x212d741]
14098b887 4c8d0d57d71202 lea r9, [rip + 0x212d757]
14098b88e 488d8d70040000 lea rcx, [rbp + 0x470]
14098b895 41b81d000000 mov r8d, 0x1d
14098b89b e8007ac700 call 0x1416032a0
14098b8a0 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098b8a7 4c89ad48060000 mov qword ptr [rbp + 0x648], r13
14098b8ae 4c89a540060000 mov qword ptr [rbp + 0x640], r12
14098b8b5 4c89bd38060000 mov qword ptr [rbp + 0x638], r15
14098b8bc c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098b8c3 c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098b8ca 488d8de0020000 lea rcx, [rbp + 0x2e0]
14098b8d1 488dbd70040000 lea rdi, [rbp + 0x470]
14098b8d8 49b8499ac42ad83ef2a2 movabs r8, 0xa2f23ed82ac49a49
14098b8e2 4889fa mov rdx, rdi
14098b8e5 e85616c800 call 0x14160cf40
14098b8ea 4c89a530060000 mov qword ptr [rbp + 0x630], r12
14098b8f1 4c89bd28060000 mov qword ptr [rbp + 0x628], r15
14098b8f8 4c89ad20060000 mov qword ptr [rbp + 0x620], r13
14098b8ff 4889b518060000 mov qword ptr [rbp + 0x618], rsi
14098b906 488bb500060000 mov rsi, qword ptr [rbp + 0x600]
14098b90d 488d4608 lea rax, [rsi + 8]
14098b911 48898550010000 mov qword ptr [rbp + 0x150], rax
14098b918 c7857004000002000000 mov dword ptr [rbp + 0x470], 2
14098b922 48897c2420 mov qword ptr [rsp + 0x20], rdi
14098b927 488d8d50010000 lea rcx, [rbp + 0x150]
14098b92e 488d95e0020000 lea rdx, [rbp + 0x2e0]
14098b935 49b869a5a8892f81b662 movabs r8, 0x62b6812f89a8a569
14098b93f 41b119 mov r9b, 0x19
14098b942 e8f928f6ff call 0x1408ee240
14098b947 488d8d70040000 lea rcx, [rbp + 0x470]
14098b94e 488d95e0020000 lea rdx, [rbp + 0x2e0]
14098b955 41b890010000 mov r8d, 0x190
14098b95b e8a082f901 call 0x142923c00
14098b960 80bda605000008 cmp byte ptr [rbp + 0x5a6], 8
14098b967 7425 je 0x14098b98e
14098b969 4c8b0e mov r9, qword ptr [rsi]
14098b96c 488d8d80050000 lea rcx, [rbp + 0x580]
14098b973 bae8000000 mov edx, 0xe8
14098b978 480395a8050000 add rdx, qword ptr [rbp + 0x5a8]
14098b97f 49b8d34103961336d2e8 movabs r8, 0xe8d23613960341d3
14098b989 e8d2fdfdff call 0x14096b760
14098b98e 488db5e0020000 lea rsi, [rbp + 0x2e0]
14098b995 488d9570040000 lea rdx, [rbp + 0x470]
14098b99c 41b890010000 mov r8d, 0x190
14098b9a2 4889f1 mov rcx, rsi
14098b9a5 e85682f901 call 0x142923c00
14098b9aa 488bbd18060000 mov rdi, qword ptr [rbp + 0x618]
14098b9b1 4889bd50060000 mov qword ptr [rbp + 0x650], rdi
14098b9b8 488b9d20060000 mov rbx, qword ptr [rbp + 0x620]
14098b9bf 48899d48060000 mov qword ptr [rbp + 0x648], rbx
14098b9c6 4c8bbd30060000 mov r15, qword ptr [rbp + 0x630]
14098b9cd 4c89bd40060000 mov qword ptr [rbp + 0x640], r15
14098b9d4 4c8bb528060000 mov r14, qword ptr [rbp + 0x628]
14098b9db 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098b9e2 c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098b9e9 c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098b9f0 488d8d70040000 lea rcx, [rbp + 0x470]
14098b9f7 49b8edb8c743bc36ab35 movabs r8, 0x35ab36bc43c7b8ed
14098ba01 4889f2 mov rdx, rsi
14098ba04 e88708c800 call 0x14160c290
14098ba09 4889bd50060000 mov qword ptr [rbp + 0x650], rdi
14098ba10 48899d48060000 mov qword ptr [rbp + 0x648], rbx
14098ba17 4c89bd40060000 mov qword ptr [rbp + 0x640], r15
14098ba1e 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098ba25 c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098ba2c c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098ba33 488d8de0020000 lea rcx, [rbp + 0x2e0]
14098ba3a 488d9570040000 lea rdx, [rbp + 0x470]
14098ba41 49b8c5677f3b90a98337 movabs r8, 0x3783a9903b7f67c5
14098ba4b e830bdc700 call 0x141607780
14098ba50 4889bd50060000 mov qword ptr [rbp + 0x650], rdi
14098ba57 48899d48060000 mov qword ptr [rbp + 0x648], rbx
14098ba5e 4c89bd40060000 mov qword ptr [rbp + 0x640], r15
14098ba65 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098ba6c c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098ba73 c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098ba7a 488d8d50010000 lea rcx, [rbp + 0x150]
14098ba81 488d95e0020000 lea rdx, [rbp + 0x2e0]
14098ba88 49b8b2513d42f7c60c7d movabs r8, 0x7d0cc6f7423d51b2
14098ba92 e859c2c700 call 0x141607cf0
14098ba97 4889bd50060000 mov qword ptr [rbp + 0x650], rdi
14098ba9e 48899d48060000 mov qword ptr [rbp + 0x648], rbx
14098baa5 4c89bd40060000 mov qword ptr [rbp + 0x640], r15
14098baac 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098bab3 c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098baba c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098bac1 488d8d70040000 lea rcx, [rbp + 0x470]
14098bac8 488d9550010000 lea rdx, [rbp + 0x150]
14098bacf 49b85a7612802b7ac0f1 movabs r8, 0xf1c07a2b8012765a
14098bad9 41b901000000 mov r9d, 1
14098badf e82cd1c700 call 0x141608c10
14098bae4 80bda605000008 cmp byte ptr [rbp + 0x5a6], 8
14098baeb 7428 je 0x14098bb15
14098baed 488d8d80050000 lea rcx, [rbp + 0x580]
14098baf4 bae8000000 mov edx, 0xe8
14098baf9 480395a8050000 add rdx, qword ptr [rbp + 0x5a8]
14098bb00 49b88957b716a611950e movabs r8, 0xe9511a616b75789
14098bb0a 41b911000000 mov r9d, 0x11
14098bb10 e85bfafdff call 0x14096b570
14098bb15 488d75c0 lea rsi, [rbp - 0x40]
14098bb19 488d9570040000 lea rdx, [rbp + 0x470]
14098bb20 41b890010000 mov r8d, 0x190
14098bb26 4889f1 mov rcx, rsi
14098bb29 e8d280f901 call 0x142923c00
14098bb2e 488b9528060000 mov rdx, qword ptr [rbp + 0x628]
14098bb35 48899570040000 mov qword ptr [rbp + 0x470], rdx
14098bb3c 4c8b8530060000 mov r8, qword ptr [rbp + 0x630]
14098bb43 4c898578040000 mov qword ptr [rbp + 0x478], r8
14098bb4a 488b8510060000 mov rax, qword ptr [rbp + 0x610]
14098bb51 48898580040000 mov qword ptr [rbp + 0x480], rax
14098bb58 488b8d20060000 mov rcx, qword ptr [rbp + 0x620]
14098bb5f 48898d88040000 mov qword ptr [rbp + 0x488], rcx
14098bb66 488b8518060000 mov rax, qword ptr [rbp + 0x618]
14098bb6d 48898590040000 mov qword ptr [rbp + 0x490], rax
14098bb74 4c8b8d08060000 mov r9, qword ptr [rbp + 0x608]
14098bb7b 4c898d98040000 mov qword ptr [rbp + 0x498], r9
14098bb82 48898550060000 mov qword ptr [rbp + 0x650], rax
14098bb89 48898d48060000 mov qword ptr [rbp + 0x648], rcx
14098bb90 4c898540060000 mov qword ptr [rbp + 0x640], r8
14098bb97 48899538060000 mov qword ptr [rbp + 0x638], rdx
14098bb9e c6855f06000001 mov byte ptr [rbp + 0x65f], 1
14098bba5 c6855e06000001 mov byte ptr [rbp + 0x65e], 1
14098bbac 488d9570040000 lea rdx, [rbp + 0x470]
14098bbb3 4889f1 mov rcx, rsi
14098bbb6 e8b5e9c700 call 0x14160a570
14098bbbb 90 nop 
14098bbbc 4881c4e8060000 add rsp, 0x6e8
14098bbc3 5b pop rbx
14098bbc4 5f pop rdi
14098bbc5 5e pop rsi
14098bbc6 415c pop r12
14098bbc8 415d pop r13
14098bbca 415e pop r14
14098bbcc 415f pop r15
14098bbce 5d pop rbp
14098bbcf c3 ret 
14098bbd0 488d0d91d31202 lea rcx, [rip + 0x212d391]
14098bbd7 e834d7f401 call 0x1428d9310
14098bbdc c6855f06000001 mov byte ptr [rbp + 0x65f], 1
14098bbe3 c6855e06000001 mov byte ptr [rbp + 0x65e], 1
14098bbea 488d0da7d31202 lea rcx, [rip + 0x212d3a7]
14098bbf1 e81ad7f401 call 0x1428d9310
14098bbf6 eb26 jmp 0x14098bc1e
14098bbf8 488d0d81d31202 lea rcx, [rip + 0x212d381]
14098bbff e80cd7f401 call 0x1428d9310
14098bc04 c6855f06000001 mov byte ptr [rbp + 0x65f], 1
14098bc0b c6855e06000001 mov byte ptr [rbp + 0x65e], 1
14098bc12 488d0d97d31202 lea rcx, [rip + 0x212d397]
14098bc19 e8f2d6f401 call 0x1428d9310
14098bc1e 0f0b ud2 
14098bc20 4889542410 mov qword ptr [rsp + 0x10], rdx
14098bc25 55 push rbp
14098bc26 4157 push r15
14098bc28 4156 push r14
14098bc2a 4155 push r13
14098bc2c 4154 push r12
14098bc2e 56 push rsi
14098bc2f 57 push rdi
14098bc30 53 push rbx
14098bc31 4883ec38 sub rsp, 0x38
14098bc35 488daa80000000 lea rbp, [rdx + 0x80]
14098bc3c 80bda605000008 cmp byte ptr [rbp + 0x5a6], 8
14098bc43 740c je 0x14098bc51
14098bc45 488d8d90050000 lea rcx, [rbp + 0x590]
14098bc4c e85f682500 call 0x140be24b0
14098bc51 90 nop 
14098bc52 4883c438 add rsp, 0x38
14098bc56 5b pop rbx
14098bc57 5f pop rdi
14098bc58 5e pop rsi
14098bc59 415c pop r12
14098bc5b 415d pop r13
14098bc5d 415e pop r14
14098bc5f 415f pop r15
14098bc61 5d pop rbp
14098bc62 c3 ret 
14098bc63 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14098bc70 4889542410 mov qword ptr [rsp + 0x10], rdx
14098bc75 55 push rbp
14098bc76 4157 push r15
14098bc78 4156 push r14
14098bc7a 4155 push r13
14098bc7c 4154 push r12
14098bc7e 56 push rsi
14098bc7f 57 push rdi
14098bc80 53 push rbx
14098bc81 4883ec38 sub rsp, 0x38
14098bc85 488daa80000000 lea rbp, [rdx + 0x80]
14098bc8c 0fb69d5e060000 movzx ebx, byte ptr [rbp + 0x65e]
14098bc93 488bb538060000 mov rsi, qword ptr [rbp + 0x638]
14098bc9a 488bbd40060000 mov rdi, qword ptr [rbp + 0x640]
14098bca1 488b9548060000 mov rdx, qword ptr [rbp + 0x648]
14098bca8 4885d2 test rdx, rdx
14098bcab 0f94c0 sete al
14098bcae 0a855f060000 or al, byte ptr [rbp + 0x65f]
14098bcb4 a801 test al, 1
14098bcb6 7516 jne 0x14098bcce
14098bcb8 488b8d50060000 mov rcx, qword ptr [rbp + 0x650]
14098bcbf 48c1e203 shl rdx, 3
14098bcc3 41b804000000 mov r8d, 4
14098bcc9 e8527fc8ff call 0x140613c20
14098bcce 4885f6 test rsi, rsi
14098bcd1 0f94c0 sete al
14098bcd4 08c3 or bl, al
14098bcd6 f6c301 test bl, 1
14098bcd9 7515 jne 0x14098bcf0
14098bcdb 48c1e603 shl rsi, 3
14098bcdf 41b804000000 mov r8d, 4
14098bce5 4889f9 mov rcx, rdi
14098bce8 4889f2 mov rdx, rsi
14098bceb e8307fc8ff call 0x140613c20
14098bcf0 90 nop 
14098bcf1 4883c438 add rsp, 0x38
14098bcf5 5b pop rbx
14098bcf6 5f pop rdi
14098bcf7 5e pop rsi
14098bcf8 415c pop r12
14098bcfa 415d pop r13
14098bcfc 415e pop r14
14098bcfe 415f pop r15
14098bd00 5d pop rbp
14098bd01 c3 ret 
14098bd02 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14098bd10 4889542410 mov qword ptr [rsp + 0x10], rdx
14098bd15 55 push rbp
14098bd16 4157 push r15
14098bd18 4156 push r14
14098bd1a 4155 push r13
14098bd1c 4154 push r12
14098bd1e 56 push rsi
14098bd1f 57 push rdi
14098bd20 53 push rbx
14098bd21 4883ec38 sub rsp, 0x38
14098bd25 488daa80000000 lea rbp, [rdx + 0x80]
14098bd2c 488d8de0020000 lea rcx, [rbp + 0x2e0]
14098bd33 e878b2f0ff call 0x140896fb0
14098bd38 90 nop 
14098bd39 4883c438 add rsp, 0x38
14098bd3d 5b pop rbx
14098bd3e 5f pop rdi
14098bd3f 5e pop rsi
14098bd40 415c pop r12
14098bd42 415d pop r13
14098bd44 415e pop r14
14098bd46 415f pop r15
14098bd48 5d pop rbp
14098bd49 c3 ret 
14098bd4a 660f1f440000 nop word ptr [rax + rax]
14098bd50 4889542410 mov qword ptr [rsp + 0x10], rdx
14098bd55 55 push rbp
14098bd56 4157 push r15
14098bd58 4156 push r14
14098bd5a 4155 push r13
14098bd5c 4154 push r12
14098bd5e 56 push rsi
14098bd5f 57 push rdi
14098bd60 53 push rbx
14098bd61 4883ec38 sub rsp, 0x38
14098bd65 488daa80000000 lea rbp, [rdx + 0x80]
14098bd6c 488bb518060000 mov rsi, qword ptr [rbp + 0x618]
14098bd73 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098bd7a 488bbd20060000 mov rdi, qword ptr [rbp + 0x620]
14098bd81 4889bd48060000 mov qword ptr [rbp + 0x648], rdi
14098bd88 4c8bb530060000 mov r14, qword ptr [rbp + 0x630]
14098bd8f 4c89b540060000 mov qword ptr [rbp + 0x640], r14
14098bd96 488b9d28060000 mov rbx, qword ptr [rbp + 0x628]
14098bd9d 48899d38060000 mov qword ptr [rbp + 0x638], rbx
14098bda4 c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098bdab c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098bdb2 488d8de0020000 lea rcx, [rbp + 0x2e0]
14098bdb9 e8f2b1f0ff call 0x140896fb0
14098bdbe 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098bdc5 4889bd48060000 mov qword ptr [rbp + 0x648], rdi
14098bdcc 4c89b540060000 mov qword ptr [rbp + 0x640], r14
14098bdd3 48899d38060000 mov qword ptr [rbp + 0x638], rbx
14098bdda c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098bde1 c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098bde8 4883c438 add rsp, 0x38
14098bdec 5b pop rbx
14098bded 5f pop rdi
14098bdee 5e pop rsi
14098bdef 415c pop r12
14098bdf1 415d pop r13
14098bdf3 415e pop r14
14098bdf5 415f pop r15
14098bdf7 5d pop rbp
14098bdf8 c3 ret 
14098bdf9 0f1f8000000000 nop dword ptr [rax]
14098be00 4889542410 mov qword ptr [rsp + 0x10], rdx
14098be05 55 push rbp
14098be06 4157 push r15
14098be08 4156 push r14
14098be0a 4155 push r13
14098be0c 4154 push r12
14098be0e 56 push rsi
14098be0f 57 push rdi
14098be10 53 push rbx
14098be11 4883ec38 sub rsp, 0x38
14098be15 488daa80000000 lea rbp, [rdx + 0x80]
14098be1c 488bb518060000 mov rsi, qword ptr [rbp + 0x618]
14098be23 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098be2a 488bbd20060000 mov rdi, qword ptr [rbp + 0x620]
14098be31 4889bd48060000 mov qword ptr [rbp + 0x648], rdi
14098be38 4c8bb530060000 mov r14, qword ptr [rbp + 0x630]
14098be3f 4c89b540060000 mov qword ptr [rbp + 0x640], r14
14098be46 488b9d28060000 mov rbx, qword ptr [rbp + 0x628]
14098be4d 48899d38060000 mov qword ptr [rbp + 0x638], rbx
14098be54 c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098be5b c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098be62 488d8d70040000 lea rcx, [rbp + 0x470]
14098be69 e842b1f0ff call 0x140896fb0
14098be6e 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098be75 4889bd48060000 mov qword ptr [rbp + 0x648], rdi
14098be7c 4c89b540060000 mov qword ptr [rbp + 0x640], r14
14098be83 48899d38060000 mov qword ptr [rbp + 0x638], rbx
14098be8a c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098be91 c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098be98 4883c438 add rsp, 0x38
14098be9c 5b pop rbx
14098be9d 5f pop rdi
14098be9e 5e pop rsi
14098be9f 415c pop r12
14098bea1 415d pop r13
14098bea3 415e pop r14
14098bea5 415f pop r15
14098bea7 5d pop rbp
14098bea8 c3 ret 
14098bea9 0f1f8000000000 nop dword ptr [rax]
14098beb0 4889542410 mov qword ptr [rsp + 0x10], rdx
14098beb5 55 push rbp
14098beb6 4157 push r15
14098beb8 4156 push r14
14098beba 4155 push r13
14098bebc 4154 push r12
14098bebe 56 push rsi
14098bebf 57 push rdi
14098bec0 53 push rbx
14098bec1 4883ec38 sub rsp, 0x38
14098bec5 488daa80000000 lea rbp, [rdx + 0x80]
14098becc 488bb518060000 mov rsi, qword ptr [rbp + 0x618]
14098bed3 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098beda 488bbd20060000 mov rdi, qword ptr [rbp + 0x620]
14098bee1 4889bd48060000 mov qword ptr [rbp + 0x648], rdi
14098bee8 4c8bb530060000 mov r14, qword ptr [rbp + 0x630]
14098beef 4c89b540060000 mov qword ptr [rbp + 0x640], r14
14098bef6 488b9d28060000 mov rbx, qword ptr [rbp + 0x628]
14098befd 48899d38060000 mov qword ptr [rbp + 0x638], rbx
14098bf04 c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098bf0b c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098bf12 488d8d70040000 lea rcx, [rbp + 0x470]
14098bf19 e892b0f0ff call 0x140896fb0
14098bf1e 4889b550060000 mov qword ptr [rbp + 0x650], rsi
14098bf25 4889bd48060000 mov qword ptr [rbp + 0x648], rdi
14098bf2c 4c89b540060000 mov qword ptr [rbp + 0x640], r14
14098bf33 48899d38060000 mov qword ptr [rbp + 0x638], rbx
14098bf3a c6855f06000000 mov byte ptr [rbp + 0x65f], 0
14098bf41 c6855e06000000 mov byte ptr [rbp + 0x65e], 0
14098bf48 4883c438 add rsp, 0x38
14098bf4c 5b pop rbx
14098bf4d 5f pop rdi
14098bf4e 5e pop rsi
14098bf4f 415c pop r12
14098bf51 415d pop r13
14098bf53 415e pop r14
14098bf55 415f pop r15
14098bf57 5d pop rbp
14098bf58 c3 ret 
