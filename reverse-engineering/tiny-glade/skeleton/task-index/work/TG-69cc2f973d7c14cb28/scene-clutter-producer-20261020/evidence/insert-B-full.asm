0x1408258b0 55 push rbp
0x1408258b1 4157 push r15
0x1408258b3 4156 push r14
0x1408258b5 4155 push r13
0x1408258b7 4154 push r12
0x1408258b9 56 push rsi
0x1408258ba 57 push rdi
0x1408258bb 53 push rbx
0x1408258bc 4881ec88010000 sub rsp, 0x188
0x1408258c3 488dac2480000000 lea rbp, [rsp + 0x80]
0x1408258cb 48c78500010000feffffff mov qword ptr [rbp + 0x100], 0xfffffffffffffffe
0x1408258d6 4c89ce mov rsi, r9
0x1408258d9 4d89c6 mov r14, r8
0x1408258dc 4889cb mov rbx, rcx
0x1408258df 8995f8000000 mov dword ptr [rbp + 0xf8], edx
0x1408258e5 8955a0 mov dword ptr [rbp - 0x60], edx
0x1408258e8 4c8d7920 lea r15, [rcx + 0x20]
0x1408258ec 488d55a0 lea rdx, [rbp - 0x60]
0x1408258f0 4c89f9 mov rcx, r15
0x1408258f3 e8d8b5d8ff call 0x1405b0ed0
0x1408258f8 4989c5 mov r13, rax
0x1408258fb 49c1ed39 shr r13, 0x39
0x1408258ff 488b0b mov rcx, qword ptr [rbx]
0x140825902 488b5308 mov rdx, qword ptr [rbx + 8]
0x140825906 4989d0 mov r8, rdx
0x140825909 4921c0 and r8, rax
0x14082590c 66410f6ec5 movd xmm0, r13d
0x140825911 660f60c0 punpcklbw xmm0, xmm0
0x140825915 f20f70c000 pshuflw xmm0, xmm0, 0
0x14082591a 660f70c000 pshufd xmm0, xmm0, 0
0x14082591f 488d79f0 lea rdi, [rcx - 0x10]
0x140825923 4531d2 xor r10d, r10d
0x140825926 660f76c9 pcmpeqd xmm1, xmm1
0x14082592a f3420f6f1401 movdqu xmm2, xmmword ptr [rcx + r8]
0x140825930 660f6fda movdqa xmm3, xmm2
0x140825934 660f74d8 pcmpeqb xmm3, xmm0
0x140825938 66440fd7db pmovmskb r11d, xmm3
0x14082593d 4585db test r11d, r11d
0x140825940 744e je 0x140825990
0x140825942 4c89b5f0000000 mov qword ptr [rbp + 0xf0], r14
0x140825949 4989f6 mov r14, rsi
0x14082594c f3450fbce3 tzcnt r12d, r11d
0x140825951 4d01c4 add r12, r8
0x140825954 4921d4 and r12, rdx
0x140825957 4c89e6 mov rsi, r12
0x14082595a 48c1e604 shl rsi, 4
0x14082595e 4989f9 mov r9, rdi
0x140825961 4929f1 sub r9, rsi
0x140825964 8bb5f8000000 mov esi, dword ptr [rbp + 0xf8]
0x14082596a 413931 cmp dword ptr [r9], esi
0x14082596d 7446 je 0x1408259b5
0x14082596f 458d4bff lea r9d, [r11 - 1]
0x140825973 664521d9 and r9w, r11w
0x140825977 4589cb mov r11d, r9d
0x14082597a 4c89f6 mov rsi, r14
0x14082597d 4c8bb5f0000000 mov r14, qword ptr [rbp + 0xf0]
0x140825984 75c3 jne 0x140825949
0x140825986 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140825990 660f74d1 pcmpeqb xmm2, xmm1
0x140825994 66440fd7ca pmovmskb r9d, xmm2
0x140825999 4585c9 test r9d, r9d
0x14082599c 0f85e6020000 jne 0x140825c88
0x1408259a2 4d01d0 add r8, r10
0x1408259a5 4983c010 add r8, 0x10
0x1408259a9 4983c210 add r10, 0x10
0x1408259ad 4921d0 and r8, rdx
0x1408259b0 e975ffffff jmp 0x14082592a
0x1408259b5 49f7dc neg r12
0x1408259b8 49c1e404 shl r12, 4
0x1408259bc 4e8d3c21 lea r15, [rcx + r12]
0x1408259c0 4a8d3c21 lea rdi, [rcx + r12]
0x1408259c4 4883c7f8 add rdi, -8
0x1408259c8 4a8b4c21f8 mov rcx, qword ptr [rcx + r12 - 8]
0x1408259cd 48c7c2ffffffff mov rdx, 0xffffffffffffffff
0x1408259d4 b801000000 mov eax, 1
0x1408259d9 f0480fb15108 lock cmpxchg qword ptr [rcx + 8], rdx
0x1408259df 0f85a1010000 jne 0x140825b86
0x1408259e5 488b01 mov rax, qword ptr [rcx]
0x1408259e8 48c7410801000000 mov qword ptr [rcx + 8], 1
0x1408259f0 4883f801 cmp rax, 1
0x1408259f4 0f858c010000 jne 0x140825b86
0x1408259fa 488b07 mov rax, qword ptr [rdi]
0x1408259fd 48c7405001000000 mov qword ptr [rax + 0x50], 1
0x140825a05 488985f8000000 mov qword ptr [rbp + 0xf8], rax
0x140825a0c 83786000 cmp dword ptr [rax + 0x60], 0
0x140825a10 4c89f6 mov rsi, r14
0x140825a13 7521 jne 0x140825a36
0x140825a15 488b85f8000000 mov rax, qword ptr [rbp + 0xf8]
0x140825a1c 488b4070 mov rax, qword ptr [rax + 0x70]
0x140825a20 f048ff08 lock dec qword ptr [rax]
0x140825a24 7510 jne 0x140825a36
0x140825a26 488b85f8000000 mov rax, qword ptr [rbp + 0xf8]
0x140825a2d 488d4870 lea rcx, [rax + 0x70]
0x140825a31 e8ea6b3300 call 0x140b5c620
0x140825a36 488b85f8000000 mov rax, qword ptr [rbp + 0xf8]
0x140825a3d c7406002000000 mov dword ptr [rax + 0x60], 2
0x140825a44 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
0x140825a48 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
0x140825a4c 0f1055bc movups xmm2, xmmword ptr [rbp - 0x44]
0x140825a50 0f114064 movups xmmword ptr [rax + 0x64], xmm0
0x140825a54 0f114874 movups xmmword ptr [rax + 0x74], xmm1
0x140825a58 0f119080000000 movups xmmword ptr [rax + 0x80], xmm2
0x140825a5f 83b89000000000 cmp dword ptr [rax + 0x90], 0
0x140825a66 4c8bb5f0000000 mov r14, qword ptr [rbp + 0xf0]
0x140825a6d 7527 jne 0x140825a96
0x140825a6f 488b85f8000000 mov rax, qword ptr [rbp + 0xf8]
0x140825a76 488b80a0000000 mov rax, qword ptr [rax + 0xa0]
0x140825a7d f048ff08 lock dec qword ptr [rax]
0x140825a81 7513 jne 0x140825a96
0x140825a83 488b85f8000000 mov rax, qword ptr [rbp + 0xf8]
0x140825a8a 488d88a0000000 lea rcx, [rax + 0xa0]
0x140825a91 e88a6b3300 call 0x140b5c620
0x140825a96 488bbdf8000000 mov rdi, qword ptr [rbp + 0xf8]
0x140825a9d c7879000000002000000 mov dword ptr [rdi + 0x90], 2
0x140825aa7 f30f6f45a0 movdqu xmm0, xmmword ptr [rbp - 0x60]
0x140825aac f30f6f4db0 movdqu xmm1, xmmword ptr [rbp - 0x50]
0x140825ab1 f30f6f55bc movdqu xmm2, xmmword ptr [rbp - 0x44]
0x140825ab6 f30f7f8794000000 movdqu xmmword ptr [rdi + 0x94], xmm0
0x140825abe f30f7f8fa4000000 movdqu xmmword ptr [rdi + 0xa4], xmm1
0x140825ac6 f30f7f97b0000000 movdqu xmmword ptr [rdi + 0xb0], xmm2
0x140825ace 660fefc0 pxor xmm0, xmm0
0x140825ad2 660f7f4710 movdqa xmmword ptr [rdi + 0x10], xmm0
0x140825ad7 488b8f20010000 mov rcx, qword ptr [rdi + 0x120]
0x140825ade 4885c9 test rcx, rcx
0x140825ae1 7410 je 0x140825af3
0x140825ae3 ba50000000 mov edx, 0x50
0x140825ae8 41b810000000 mov r8d, 0x10
0x140825aee e82de1deff call 0x140613c20
0x140825af3 48c7872001000000000000 mov qword ptr [rdi + 0x120], 0
0x140825afe 488b8f28010000 mov rcx, qword ptr [rdi + 0x128]
0x140825b05 4885c9 test rcx, rcx
0x140825b08 7410 je 0x140825b1a
0x140825b0a ba28000000 mov edx, 0x28
0x140825b0f 41b804000000 mov r8d, 4
0x140825b15 e806e1deff call 0x140613c20
0x140825b1a 48c7872801000000000000 mov qword ptr [rdi + 0x128], 0
0x140825b25 488b8f30010000 mov rcx, qword ptr [rdi + 0x130]
0x140825b2c 4885c9 test rcx, rcx
0x140825b2f 7410 je 0x140825b41
0x140825b31 ba20000000 mov edx, 0x20
0x140825b36 41b804000000 mov r8d, 4
0x140825b3c e8dfe0deff call 0x140613c20
0x140825b41 48c7873001000000000000 mov qword ptr [rdi + 0x130], 0
0x140825b4c 48c787d000000000000000 mov qword ptr [rdi + 0xd0], 0
0x140825b57 48c787e800000000000000 mov qword ptr [rdi + 0xe8], 0
0x140825b62 48c7870001000000000000 mov qword ptr [rdi + 0x100], 0
0x140825b6d 48c7871801000000000000 mov qword ptr [rdi + 0x118], 0
0x140825b78 66c787380100000000 mov word ptr [rdi + 0x138], 0
0x140825b81 e93b020000 jmp 0x140825dc1
0x140825b86 660fefc0 pxor xmm0, xmm0
0x140825b8a 660f7f4570 movdqa xmmword ptr [rbp + 0x70], xmm0
0x140825b8f f30f7f8588000000 movdqu xmmword ptr [rbp + 0x88], xmm0
0x140825b97 f30f7f85b8000000 movdqu xmmword ptr [rbp + 0xb8], xmm0
0x140825b9f f30f7f85c8000000 movdqu xmmword ptr [rbp + 0xc8], xmm0
0x140825ba7 66c785d80000000000 mov word ptr [rbp + 0xd8], 0
0x140825bb0 48c745a001000000 mov qword ptr [rbp - 0x60], 1
0x140825bb8 48c745a801000000 mov qword ptr [rbp - 0x58], 1
0x140825bc0 660f7f45b0 movdqa xmmword ptr [rbp - 0x50], xmm0
0x140825bc5 48c745f001000000 mov qword ptr [rbp - 0x10], 1
0x140825bcd c7450002000000 mov dword ptr [rbp], 2
0x140825bd4 c7453002000000 mov dword ptr [rbp + 0x30], 2
0x140825bdb 48c7456000000000 mov qword ptr [rbp + 0x60], 0
0x140825be3 48c7456810000000 mov qword ptr [rbp + 0x68], 0x10
0x140825beb 48c7858000000004000000 mov qword ptr [rbp + 0x80], 4
0x140825bf6 48c7859800000004000000 mov qword ptr [rbp + 0x98], 4
0x140825c01 660f7f85a0000000 movdqa xmmword ptr [rbp + 0xa0], xmm0
0x140825c09 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x140825c14 0fb605eeefe002 movzx eax, byte ptr [rip + 0x2e0efee]
0x140825c1b b940010000 mov ecx, 0x140
0x140825c20 ba10000000 mov edx, 0x10
0x140825c25 e8e6dfdeff call 0x140613c10
0x140825c2a 4885c0 test rax, rax
0x140825c2d 0f8452020000 je 0x140825e85
0x140825c33 4c89f6 mov rsi, r14
0x140825c36 488d55a0 lea rdx, [rbp - 0x60]
0x140825c3a 41b840010000 mov r8d, 0x140
0x140825c40 488985e8000000 mov qword ptr [rbp + 0xe8], rax
0x140825c47 4889c1 mov rcx, rax
0x140825c4a e8b1df0f02 call 0x142923c00
0x140825c4f 488b07 mov rax, qword ptr [rdi]
0x140825c52 f048ff08 lock dec qword ptr [rax]
0x140825c56 4c8bb5f0000000 mov r14, qword ptr [rbp + 0xf0]
0x140825c5d 4889bdf8000000 mov qword ptr [rbp + 0xf8], rdi
0x140825c64 750c jne 0x140825c72
0x140825c66 488b8df8000000 mov rcx, qword ptr [rbp + 0xf8]
0x140825c6d e82e550000 call 0x14082b1a0
0x140825c72 488b85e8000000 mov rax, qword ptr [rbp + 0xe8]
0x140825c79 488b8df8000000 mov rcx, qword ptr [rbp + 0xf8]
0x140825c80 488901 mov qword ptr [rcx], rax
0x140825c83 e939010000 jmp 0x140825dc1
0x140825c88 4989c4 mov r12, rax
0x140825c8b 48837b1000 cmp qword ptr [rbx + 0x10], 0
0x140825c90 0f84b3010000 je 0x140825e49
0x140825c96 0fb6056cefe002 movzx eax, byte ptr [rip + 0x2e0ef6c]
0x140825c9d b940010000 mov ecx, 0x140
0x140825ca2 ba10000000 mov edx, 0x10
0x140825ca7 e864dfdeff call 0x140613c10
0x140825cac 4885c0 test rax, rax
0x140825caf 0f84e1010000 je 0x140825e96
0x140825cb5 48c70001000000 mov qword ptr [rax], 1
0x140825cbc 48c7400801000000 mov qword ptr [rax + 8], 1
0x140825cc4 660fefc0 pxor xmm0, xmm0
0x140825cc8 660f7f4010 movdqa xmmword ptr [rax + 0x10], xmm0
0x140825ccd 48c7405001000000 mov qword ptr [rax + 0x50], 1
0x140825cd5 c7406002000000 mov dword ptr [rax + 0x60], 2
0x140825cdc c7809000000002000000 mov dword ptr [rax + 0x90], 2
0x140825ce6 48c780c000000000000000 mov qword ptr [rax + 0xc0], 0
0x140825cf1 48c780c800000010000000 mov qword ptr [rax + 0xc8], 0x10
0x140825cfc 660f7f80d0000000 movdqa xmmword ptr [rax + 0xd0], xmm0
0x140825d04 48c780e000000004000000 mov qword ptr [rax + 0xe0], 4
0x140825d0f f30f7f80e8000000 movdqu xmmword ptr [rax + 0xe8], xmm0
0x140825d17 48c780f800000004000000 mov qword ptr [rax + 0xf8], 4
0x140825d22 660f7f8000010000 movdqa xmmword ptr [rax + 0x100], xmm0
0x140825d2a 48c7801001000010000000 mov qword ptr [rax + 0x110], 0x10
0x140825d35 f30f7f8018010000 movdqu xmmword ptr [rax + 0x118], xmm0
0x140825d3d f30f7f8028010000 movdqu xmmword ptr [rax + 0x128], xmm0
0x140825d45 66c780380100000000 mov word ptr [rax + 0x138], 0
0x140825d4e 488b0b mov rcx, qword ptr [rbx]
0x140825d51 488b5308 mov rdx, qword ptr [rbx + 8]
0x140825d55 4921d4 and r12, rdx
0x140825d58 f3420f6f0421 movdqu xmm0, xmmword ptr [rcx + r12]
0x140825d5e 66440fd7c0 pmovmskb r8d, xmm0
0x140825d63 4585c0 test r8d, r8d
0x140825d66 0f84b7000000 je 0x140825e23
0x140825d6c f3450fbcc0 tzcnt r8d, r8d
0x140825d71 4d01e0 add r8, r12
0x140825d74 4921d0 and r8, rdx
0x140825d77 460fb60c01 movzx r9d, byte ptr [rcx + r8]
0x140825d7c 4584c9 test r9b, r9b
0x140825d7f 0f89dc000000 jns 0x140825e61
0x140825d85 4d8d50f0 lea r10, [r8 - 0x10]
0x140825d89 4921d2 and r10, rdx
0x140825d8c 46882c01 mov byte ptr [rcx + r8], r13b
0x140825d90 46886c1110 mov byte ptr [rcx + r10 + 0x10], r13b
0x140825d95 49c1e004 shl r8, 4
0x140825d99 4180e101 and r9b, 1
0x140825d9d 410fb6d1 movzx edx, r9b
0x140825da1 48295310 sub qword ptr [rbx + 0x10], rdx
0x140825da5 4989cf mov r15, rcx
0x140825da8 4d29c7 sub r15, r8
0x140825dab 49f7d8 neg r8
0x140825dae 8b95f8000000 mov edx, dword ptr [rbp + 0xf8]
0x140825db4 42895401f0 mov dword ptr [rcx + r8 - 0x10], edx
0x140825db9 498947f8 mov qword ptr [r15 - 8], rax
0x140825dbd 48ff4318 inc qword ptr [rbx + 0x18]
0x140825dc1 498b4ff8 mov rcx, qword ptr [r15 - 8]
0x140825dc5 48c7c2ffffffff mov rdx, 0xffffffffffffffff
0x140825dcc b801000000 mov eax, 1
0x140825dd1 f0480fb15108 lock cmpxchg qword ptr [rcx + 8], rdx
0x140825dd7 0f859c000000 jne 0x140825e79
0x140825ddd 488b01 mov rax, qword ptr [rcx]
0x140825de0 48c7410801000000 mov qword ptr [rcx + 8], 1
0x140825de8 4883f801 cmp rax, 1
0x140825dec 0f8587000000 jne 0x140825e79
0x140825df2 498b4ff8 mov rcx, qword ptr [r15 - 8]
0x140825df6 488d4110 lea rax, [rcx + 0x10]
0x140825dfa 4c897150 mov qword ptr [rcx + 0x50], r14
0x140825dfe 48897158 mov qword ptr [rcx + 0x58], rsi
0x140825e02 0fb69570010000 movzx edx, byte ptr [rbp + 0x170]
0x140825e09 889138010000 mov byte ptr [rcx + 0x138], dl
0x140825e0f 4881c488010000 add rsp, 0x188
0x140825e16 5b pop rbx
0x140825e17 5f pop rdi
0x140825e18 5e pop rsi
0x140825e19 415c pop r12
0x140825e1b 415d pop r13
0x140825e1d 415e pop r14
0x140825e1f 415f pop r15
0x140825e21 5d pop rbp
0x140825e22 c3 ret 
0x140825e23 41b910000000 mov r9d, 0x10
0x140825e29 4d01cc add r12, r9
0x140825e2c 4921d4 and r12, rdx
0x140825e2f f3420f6f0421 movdqu xmm0, xmmword ptr [rcx + r12]
0x140825e35 66440fd7c0 pmovmskb r8d, xmm0
0x140825e3a 4983c110 add r9, 0x10
0x140825e3e 4585c0 test r8d, r8d
0x140825e41 0f8525ffffff jne 0x140825d6c
0x140825e47 ebe0 jmp 0x140825e29
0x140825e49 ba01000000 mov edx, 1
0x140825e4e 4889d9 mov rcx, rbx
0x140825e51 4d89f8 mov r8, r15
0x140825e54 41b101 mov r9b, 1
0x140825e57 e804d90c02 call 0x1428f3760
0x140825e5c e935feffff jmp 0x140825c96
0x140825e61 660f6f01 movdqa xmm0, xmmword ptr [rcx]
0x140825e65 66440fd7c0 pmovmskb r8d, xmm0
0x140825e6a f3450fbcc0 tzcnt r8d, r8d
0x140825e6f 460fb60c01 movzx r9d, byte ptr [rcx + r8]
0x140825e74 e90cffffff jmp 0x140825d85
0x140825e79 488d0dd0b82602 lea rcx, [rip + 0x226b8d0]
0x140825e80 e88b340b02 call 0x1428d9310
0x140825e85 b910000000 mov ecx, 0x10
0x140825e8a ba40010000 mov edx, 0x140
0x140825e8f e86c310b02 call 0x1428d9000
0x140825e94 0f0b ud2 
0x140825e96 b910000000 mov ecx, 0x10
0x140825e9b ba40010000 mov edx, 0x140
0x140825ea0 e85b310b02 call 0x1428d9000
0x140825ea5 cc int3 
0x140825ea6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140825eb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140825eb5 55 push rbp
0x140825eb6 4157 push r15
0x140825eb8 4156 push r14
0x140825eba 4155 push r13
0x140825ebc 4154 push r12
0x140825ebe 56 push rsi
0x140825ebf 57 push rdi
0x140825ec0 53 push rbx
0x140825ec1 4883ec28 sub rsp, 0x28
0x140825ec5 488daa80000000 lea rbp, [rdx + 0x80]
0x140825ecc 488d4db0 lea rcx, [rbp - 0x50]
0x140825ed0 e8db3ea300 call 0x141259db0
0x140825ed5 90 nop 
0x140825ed6 4883c428 add rsp, 0x28
0x140825eda 5b pop rbx
0x140825edb 5f pop rdi
0x140825edc 5e pop rsi
0x140825edd 415c pop r12
0x140825edf 415d pop r13
0x140825ee1 415e pop r14
0x140825ee3 415f pop r15
0x140825ee5 5d pop rbp
0x140825ee6 c3 ret 
0x140825ee7 660f1f840000000000 nop word ptr [rax + rax]
0x140825ef0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140825ef5 55 push rbp
0x140825ef6 4157 push r15
0x140825ef8 4156 push r14
0x140825efa 4155 push r13
0x140825efc 4154 push r12
0x140825efe 56 push rsi
0x140825eff 57 push rdi
0x140825f00 53 push rbx
0x140825f01 4883ec28 sub rsp, 0x28
0x140825f05 488daa80000000 lea rbp, [rdx + 0x80]
0x140825f0c 488b85f8000000 mov rax, qword ptr [rbp + 0xf8]
0x140825f13 c7406002000000 mov dword ptr [rax + 0x60], 2
0x140825f1a 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
0x140825f1e 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
0x140825f22 0f1055bc movups xmm2, xmmword ptr [rbp - 0x44]
0x140825f26 0f114064 movups xmmword ptr [rax + 0x64], xmm0
0x140825f2a 0f114874 movups xmmword ptr [rax + 0x74], xmm1
0x140825f2e 0f119080000000 movups xmmword ptr [rax + 0x80], xmm2
0x140825f35 4883c428 add rsp, 0x28
0x140825f39 5b pop rbx
0x140825f3a 5f pop rdi
0x140825f3b 5e pop rsi
0x140825f3c 415c pop r12
0x140825f3e 415d pop r13
0x140825f40 415e pop r14
0x140825f42 415f pop r15
0x140825f44 5d pop rbp
0x140825f45 c3 ret 
0x140825f46 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140825f50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140825f55 55 push rbp
0x140825f56 4157 push r15
0x140825f58 4156 push r14
0x140825f5a 4155 push r13
0x140825f5c 4154 push r12
0x140825f5e 56 push rsi
0x140825f5f 57 push rdi
0x140825f60 53 push rbx
0x140825f61 4883ec28 sub rsp, 0x28
0x140825f65 488daa80000000 lea rbp, [rdx + 0x80]
0x140825f6c 488b85f8000000 mov rax, qword ptr [rbp + 0xf8]
0x140825f73 c7809000000002000000 mov dword ptr [rax + 0x90], 2
0x140825f7d 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
0x140825f81 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
0x140825f85 0f1055bc movups xmm2, xmmword ptr [rbp - 0x44]
0x140825f89 0f118094000000 movups xmmword ptr [rax + 0x94], xmm0
0x140825f90 0f1188a4000000 movups xmmword ptr [rax + 0xa4], xmm1
0x140825f97 0f1190b0000000 movups xmmword ptr [rax + 0xb0], xmm2
0x140825f9e 4883c428 add rsp, 0x28
0x140825fa2 5b pop rbx
0x140825fa3 5f pop rdi
0x140825fa4 5e pop rsi
0x140825fa5 415c pop r12
0x140825fa7 415d pop r13
0x140825fa9 415e pop r14
0x140825fab 415f pop r15
0x140825fad 5d pop rbp
0x140825fae c3 ret 
0x140825faf 90 nop 
0x140825fb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140825fb5 55 push rbp
0x140825fb6 4157 push r15
0x140825fb8 4156 push r14
0x140825fba 4155 push r13
0x140825fbc 4154 push r12
0x140825fbe 56 push rsi
0x140825fbf 57 push rdi
0x140825fc0 53 push rbx
0x140825fc1 4883ec28 sub rsp, 0x28
0x140825fc5 488daa80000000 lea rbp, [rdx + 0x80]
0x140825fcc 488b85e8000000 mov rax, qword ptr [rbp + 0xe8]
0x140825fd3 488b8df8000000 mov rcx, qword ptr [rbp + 0xf8]
0x140825fda 488901 mov qword ptr [rcx], rax
0x140825fdd 4883c428 add rsp, 0x28
0x140825fe1 5b pop rbx
0x140825fe2 5f pop rdi
0x140825fe3 5e pop rsi
0x140825fe4 415c pop r12
0x140825fe6 415d pop r13
0x140825fe8 415e pop r14
0x140825fea 415f pop r15
0x140825fec 5d pop rbp
0x140825fed c3 ret 
