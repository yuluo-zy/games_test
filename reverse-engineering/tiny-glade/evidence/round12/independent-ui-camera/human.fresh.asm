1408b2b30 55 push rbp
1408b2b31 4157 push r15
1408b2b33 4156 push r14
1408b2b35 4155 push r13
1408b2b37 4154 push r12
1408b2b39 56 push rsi
1408b2b3a 57 push rdi
1408b2b3b 53 push rbx
1408b2b3c 4881ec68010000 sub rsp, 0x168
1408b2b43 488dac2480000000 lea rbp, [rsp + 0x80]
1408b2b4b 440f299dd0000000 movaps xmmword ptr [rbp + 0xd0], xmm11
1408b2b53 440f2995c0000000 movaps xmmword ptr [rbp + 0xc0], xmm10
1408b2b5b 440f298db0000000 movaps xmmword ptr [rbp + 0xb0], xmm9
1408b2b63 440f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm8
1408b2b6b 0f29bd90000000 movaps xmmword ptr [rbp + 0x90], xmm7
1408b2b72 0f29b580000000 movaps xmmword ptr [rbp + 0x80], xmm6
1408b2b79 48c74578feffffff mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
1408b2b81 4d89cf mov r15, r9
1408b2b84 4c89c3 mov rbx, r8
1408b2b87 4889d7 mov rdi, rdx
1408b2b8a 4989ce mov r14, rcx
1408b2b8d e86e3e3600 call 0x140c16a00
1408b2b92 4189c1 mov r9d, eax
1408b2b95 84c0 test al, al
1408b2b97 7461 je 0x1408b2bfa
1408b2b99 44884d60 mov byte ptr [rbp + 0x60], r9b
1408b2b9d 41bc3e000000 mov r12d, 0x3e
1408b2ba3 b93e000000 mov ecx, 0x3e
1408b2ba8 e883f7ffff call 0x1408b2330
1408b2bad 488d35fcc21e02 lea rsi, [rip + 0x21ec2fc]
1408b2bb4 4883f801 cmp rax, 1
1408b2bb8 0f85a8000000 jne 0x1408b2c66
1408b2bbe 4989d1 mov r9, rdx
1408b2bc1 4885d2 test rdx, rdx
1408b2bc4 743d je 0x1408b2c03
1408b2bc6 4983f93e cmp r9, 0x3e
1408b2bca 7335 jae 0x1408b2c01
1408b2bcc 488d05ddc21e02 lea rax, [rip + 0x21ec2dd]
1408b2bd3 41803c01bf cmp byte ptr [r9 + rax], 0xbf
1408b2bd8 7f29 jg 0x1408b2c03
1408b2bda 488d0597c31e02 lea rax, [rip + 0x21ec397]
1408b2be1 4889442420 mov qword ptr [rsp + 0x20], rax
1408b2be6 488d0dc3c21e02 lea rcx, [rip + 0x21ec2c3]
1408b2bed ba3e000000 mov edx, 0x3e
1408b2bf2 4531c0 xor r8d, r8d
1408b2bf5 e8466d0202 call 0x1428d9940
1408b2bfa 31c0 xor eax, eax
1408b2bfc e9d6010000 jmp 0x1408b2dd7
1408b2c01 75d7 jne 0x1408b2bda
1408b2c03 4c89c9 mov rcx, r9
1408b2c06 e825f7ffff call 0x1408b2330
1408b2c0b 4883f801 cmp rax, 1
1408b2c0f 7555 jne 0x1408b2c66
1408b2c11 4989d0 mov r8, rdx
1408b2c14 4983c002 add r8, 2
1408b2c18 7439 je 0x1408b2c53
1408b2c1a 4983f83e cmp r8, 0x3e
1408b2c1e 7331 jae 0x1408b2c51
1408b2c20 488d0589c21e02 lea rax, [rip + 0x21ec289]
1408b2c27 41803c00bf cmp byte ptr [r8 + rax], 0xbf
1408b2c2c 7f25 jg 0x1408b2c53
1408b2c2e 488d055bc31e02 lea rax, [rip + 0x21ec35b]
1408b2c35 4889442420 mov qword ptr [rsp + 0x20], rax
1408b2c3a 488d0d6fc21e02 lea rcx, [rip + 0x21ec26f]
1408b2c41 ba3e000000 mov edx, 0x3e
1408b2c46 41b93e000000 mov r9d, 0x3e
1408b2c4c e8ef6c0202 call 0x1428d9940
1408b2c51 75db jne 0x1408b2c2e
1408b2c53 41bc3c000000 mov r12d, 0x3c
1408b2c59 4929d4 sub r12, rdx
1408b2c5c 488d354dc21e02 lea rsi, [rip + 0x21ec24d]
1408b2c63 4c01c6 add rsi, r8
1408b2c66 4c8d05d3c41e02 lea r8, [rip + 0x21ec4d3]
1408b2c6d 4c8d2d94c41e02 lea r13, [rip + 0x21ec494]
1408b2c74 48897d40 mov qword ptr [rbp + 0x40], rdi
1408b2c78 0f1f840000000000 nop dword ptr [rax + rax]
1408b2c80 410fbe40ff movsx eax, byte ptr [r8 - 1]
1408b2c85 85c0 test eax, eax
1408b2c87 7817 js 0x1408b2ca0
1408b2c89 49ffc8 dec r8
1408b2c8c 83f85c cmp eax, 0x5c
1408b2c8f 7563 jne 0x1408b2cf4
1408b2c91 eb78 jmp 0x1408b2d0b
1408b2c93 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1408b2ca0 410fb648fe movzx ecx, byte ptr [r8 - 2]
1408b2ca5 80f9c0 cmp cl, 0xc0
1408b2ca8 7d23 jge 0x1408b2ccd
1408b2caa 410fb650fd movzx edx, byte ptr [r8 - 3]
1408b2caf 80fac0 cmp dl, 0xc0
1408b2cb2 7d22 jge 0x1408b2cd6
1408b2cb4 450fb648fc movzx r9d, byte ptr [r8 - 4]
1408b2cb9 4983c0fc add r8, -4
1408b2cbd 4183e107 and r9d, 7
1408b2cc1 41c1e106 shl r9d, 6
1408b2cc5 83e23f and edx, 0x3f
1408b2cc8 4409ca or edx, r9d
1408b2ccb eb10 jmp 0x1408b2cdd
1408b2ccd 4983c0fe add r8, -2
1408b2cd1 83e11f and ecx, 0x1f
1408b2cd4 eb0f jmp 0x1408b2ce5
1408b2cd6 4983c0fd add r8, -3
1408b2cda 83e20f and edx, 0xf
1408b2cdd c1e206 shl edx, 6
1408b2ce0 83e13f and ecx, 0x3f
1408b2ce3 09d1 or ecx, edx
1408b2ce5 c1e106 shl ecx, 6
1408b2ce8 243f and al, 0x3f
1408b2cea 0fb6c0 movzx eax, al
1408b2ced 09c8 or eax, ecx
1408b2cef 83f85c cmp eax, 0x5c
1408b2cf2 7417 je 0x1408b2d0b
1408b2cf4 83f82f cmp eax, 0x2f
1408b2cf7 7412 je 0x1408b2d0b
1408b2cf9 4d39e8 cmp r8, r13
1408b2cfc 7582 jne 0x1408b2c80
1408b2cfe 4c89ff mov rdi, r15
1408b2d01 4989df mov r15, rbx
1408b2d04 bb38000000 mov ebx, 0x38
1408b2d09 eb52 jmp 0x1408b2d5d
1408b2d0b 4c8d2df6c31e02 lea r13, [rip + 0x21ec3f6]
1408b2d12 4d29e8 sub r8, r13
1408b2d15 49ffc0 inc r8
1408b2d18 7432 je 0x1408b2d4c
1408b2d1a 4983f838 cmp r8, 0x38
1408b2d1e 732a jae 0x1408b2d4a
1408b2d20 43803c28bf cmp byte ptr [r8 + r13], 0xbf
1408b2d25 7f25 jg 0x1408b2d4c
1408b2d27 488d052ac21e02 lea rax, [rip + 0x21ec22a]
1408b2d2e 4889442420 mov qword ptr [rsp + 0x20], rax
1408b2d33 488d0dcec31e02 lea rcx, [rip + 0x21ec3ce]
1408b2d3a ba38000000 mov edx, 0x38
1408b2d3f 41b938000000 mov r9d, 0x38
1408b2d45 e8f66b0202 call 0x1428d9940
1408b2d4a 75db jne 0x1408b2d27
1408b2d4c 4c89ff mov rdi, r15
1408b2d4f 4989df mov r15, rbx
1408b2d52 bb38000000 mov ebx, 0x38
1408b2d57 4c29c3 sub rbx, r8
1408b2d5a 4d01c5 add r13, r8
1408b2d5d e8ce533600 call 0x140c18130
1408b2d62 4889c1 mov rcx, rax
1408b2d65 488b00 mov rax, qword ptr [rax]
1408b2d68 4883f801 cmp rax, 1
1408b2d6c 0f8580050000 jne 0x1408b32f2
1408b2d72 4883c108 add rcx, 8
1408b2d76 48833900 cmp qword ptr [rcx], 0
1408b2d7a 0f85b0050000 jne 0x1408b3330
1408b2d80 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1408b2d87 48894d50 mov qword ptr [rbp + 0x50], rcx
1408b2d8b 4883c108 add rcx, 8
1408b2d8f 48895c2420 mov qword ptr [rsp + 0x20], rbx
1408b2d94 48c744243000000000 mov qword ptr [rsp + 0x30], 0
1408b2d9d 48c744242801000000 mov qword ptr [rsp + 0x28], 1
1408b2da6 4889f2 mov rdx, rsi
1408b2da9 4d89e0 mov r8, r12
1408b2dac 4d89e9 mov r9, r13
1408b2daf e88c3f3600 call 0x140c16d40
1408b2db4 488b4d50 mov rcx, qword ptr [rbp + 0x50]
1408b2db8 48ff01 inc qword ptr [rcx]
1408b2dbb 48894550 mov qword ptr [rbp + 0x50], rax
1408b2dbf 488945f8 mov qword ptr [rbp - 8], rax
1408b2dc3 b801000000 mov eax, 1
1408b2dc8 440fb64d60 movzx r9d, byte ptr [rbp + 0x60]
1408b2dcd 4c89fb mov rbx, r15
1408b2dd0 4989ff mov r15, rdi
1408b2dd3 488b7d40 mov rdi, qword ptr [rbp + 0x40]
1408b2dd7 488b8d50010000 mov rcx, qword ptr [rbp + 0x150]
1408b2dde 488945f0 mov qword ptr [rbp - 0x10], rax
1408b2de2 488b01 mov rax, qword ptr [rcx]
1408b2de5 80b8f800000000 cmp byte ptr [rax + 0xf8], 0
1408b2dec 7419 je 0x1408b2e07
1408b2dee 488b07 mov rax, qword ptr [rdi]
1408b2df1 8b8810010000 mov ecx, dword ptr [rax + 0x110]
1408b2df7 488d15b2c51e02 lea rdx, [rip + 0x21ec5b2]
1408b2dfe 48630c8a movsxd rcx, dword ptr [rdx + rcx*4]
1408b2e02 4801d1 add rcx, rdx
1408b2e05 ffe1 jmp rcx
1408b2e07 4584c9 test r9b, r9b
1408b2e0a 7442 je 0x1408b2e4e
1408b2e0c e81f533600 call 0x140c18130
1408b2e11 4889c1 mov rcx, rax
1408b2e14 488b00 mov rax, qword ptr [rax]
1408b2e17 4883f801 cmp rax, 1
1408b2e1b 0f85eb040000 jne 0x1408b330c
1408b2e21 4883c108 add rcx, 8
1408b2e25 48833900 cmp qword ptr [rcx], 0
1408b2e29 0f8501050000 jne 0x1408b3330
1408b2e2f 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1408b2e36 48894d60 mov qword ptr [rbp + 0x60], rcx
1408b2e3a 4883c108 add rcx, 8
1408b2e3e 488b5550 mov rdx, qword ptr [rbp + 0x50]
1408b2e42 e8a9423600 call 0x140c170f0
1408b2e47 488b4560 mov rax, qword ptr [rbp + 0x60]
1408b2e4b 48ff00 inc qword ptr [rax]
1408b2e4e 0f28b580000000 movaps xmm6, xmmword ptr [rbp + 0x80]
1408b2e55 0f28bd90000000 movaps xmm7, xmmword ptr [rbp + 0x90]
1408b2e5c 440f2885a0000000 movaps xmm8, xmmword ptr [rbp + 0xa0]
1408b2e64 440f288db0000000 movaps xmm9, xmmword ptr [rbp + 0xb0]
1408b2e6c 440f2895c0000000 movaps xmm10, xmmword ptr [rbp + 0xc0]
1408b2e74 440f289dd0000000 movaps xmm11, xmmword ptr [rbp + 0xd0]
1408b2e7c 4881c468010000 add rsp, 0x168
1408b2e83 5b pop rbx
1408b2e84 5f pop rdi
1408b2e85 5e pop rsi
1408b2e86 415c pop r12
1408b2e88 415d pop r13
1408b2e8a 415e pop r14
1408b2e8c 415f pop r15
1408b2e8e 5d pop rbp
1408b2e8f c3 ret 
1408b2e90 0f57c0 xorps xmm0, xmm0
1408b2e93 0f2e8014010000 ucomiss xmm0, dword ptr [rax + 0x114]
1408b2e9a 0f8267ffffff jb 0x1408b2e07
1408b2ea0 498b0e mov rcx, qword ptr [r14]
1408b2ea3 4c8ba9f8000000 mov r13, qword ptr [rcx + 0xf8]
1408b2eaa 488b8100010000 mov rax, qword ptr [rcx + 0x100]
1408b2eb1 4d8b5f08 mov r11, qword ptr [r15 + 8]
1408b2eb5 4d85db test r11, r11
1408b2eb8 7504 jne 0x1408b2ebe
1408b2eba 4d8b5f10 mov r11, qword ptr [r15 + 0x10]
1408b2ebe 4d8b7e08 mov r15, qword ptr [r14 + 8]
1408b2ec2 4c8d248500000000 lea r12, [rax*4]
1408b2eca 4d01ec add r12, r13
1408b2ecd 48894d38 mov qword ptr [rbp + 0x38], rcx
1408b2ed1 0fb68108010000 movzx eax, byte ptr [rcx + 0x108]
1408b2ed8 884540 mov byte ptr [rbp + 0x40], al
1408b2edb 4c8b33 mov r14, qword ptr [rbx]
1408b2ede b808000000 mov eax, 8
1408b2ee3 48894528 mov qword ptr [rbp + 0x28], rax
1408b2ee7 0f57f6 xorps xmm6, xmm6
1408b2eea 0f283def290702 movaps xmm7, xmmword ptr [rip + 0x20729ef]
1408b2ef1 48894530 mov qword ptr [rbp + 0x30], rax
1408b2ef5 31db xor ebx, ebx
1408b2ef7 31f6 xor esi, esi
1408b2ef9 48c7455800000000 mov qword ptr [rbp + 0x58], 0
1408b2f01 c7457400000000 mov dword ptr [rbp + 0x74], 0
1408b2f08 48c7456800000000 mov qword ptr [rbp + 0x68], 0
1408b2f10 4c897520 mov qword ptr [rbp + 0x20], r14
1408b2f14 44884d60 mov byte ptr [rbp + 0x60], r9b
1408b2f18 807d4000 cmp byte ptr [rbp + 0x40], 0
1408b2f1c 0f853e010000 jne 0x1408b3060
1408b2f22 4839de cmp rsi, rbx
1408b2f25 0f850e010000 jne 0x1408b3039
1408b2f2b 0f1f440000 nop dword ptr [rax + rax]
1408b2f30 4d39e5 cmp r13, r12
1408b2f33 0f84cefeffff je 0x1408b2e07
1408b2f39 418b4d00 mov ecx, dword ptr [r13]
1408b2f3d 4983c504 add r13, 4
1408b2f41 498b8700010000 mov rax, qword ptr [r15 + 0x100]
1408b2f48 488d0c89 lea rcx, [rcx + rcx*4]
1408b2f4c 48c1e105 shl rcx, 5
1408b2f50 488b5c0858 mov rbx, qword ptr [rax + rcx + 0x58]
1408b2f55 4885db test rbx, rbx
1408b2f58 74d6 je 0x1408b2f30
1408b2f5a 4801c8 add rax, rcx
1408b2f5d 8b8894000000 mov ecx, dword ptr [rax + 0x94]
1408b2f63 488d0cc9 lea rcx, [rcx + rcx*8]
1408b2f67 48c1e103 shl rcx, 3
1408b2f6b 49038fa8010000 add rcx, qword ptr [r15 + 0x1a8]
1408b2f72 4c8b5538 mov r10, qword ptr [rbp + 0x38]
1408b2f76 498b9210010000 mov rdx, qword ptr [r10 + 0x110]
1408b2f7d 48399088000000 cmp qword ptr [rax + 0x88], rdx
1408b2f84 764f jbe 0x1408b2fd5
1408b2f86 4c8b8080000000 mov r8, qword ptr [rax + 0x80]
1408b2f8d 49833cd000 cmp qword ptr [r8 + rdx*8], 0
1408b2f92 7441 je 0x1408b2fd5
1408b2f94 41b001 mov r8b, 1
1408b2f97 44894574 mov dword ptr [rbp + 0x74], r8d
1408b2f9b 48395140 cmp qword ptr [rcx + 0x40], rdx
1408b2f9f 764a jbe 0x1408b2feb
1408b2fa1 4c8b4138 mov r8, qword ptr [rcx + 0x38]
1408b2fa5 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
1408b2fa9 4885d2 test rdx, rdx
1408b2fac 743d je 0x1408b2feb
1408b2fae 4c8b4118 mov r8, qword ptr [rcx + 0x18]
1408b2fb2 48f7d2 not rdx
1408b2fb5 488d1452 lea rdx, [rdx + rdx*2]
1408b2fb9 48c1e204 shl rdx, 4
1408b2fbd 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
1408b2fc2 48895558 mov qword ptr [rbp + 0x58], rdx
1408b2fc6 498b9218010000 mov rdx, qword ptr [r10 + 0x118]
1408b2fcd 48395140 cmp qword ptr [rcx + 0x40], rdx
1408b2fd1 7654 jbe 0x1408b3027
1408b2fd3 eb2b jmp 0x1408b3000
1408b2fd5 c7457400000000 mov dword ptr [rbp + 0x74], 0
1408b2fdc 498b9218010000 mov rdx, qword ptr [r10 + 0x118]
1408b2fe3 48395140 cmp qword ptr [rcx + 0x40], rdx
1408b2fe7 763e jbe 0x1408b3027
1408b2fe9 eb15 jmp 0x1408b3000
1408b2feb 48c7455800000000 mov qword ptr [rbp + 0x58], 0
1408b2ff3 498b9218010000 mov rdx, qword ptr [r10 + 0x118]
1408b2ffa 48395140 cmp qword ptr [rcx + 0x40], rdx
1408b2ffe 7627 jbe 0x1408b3027
1408b3000 4c8b4138 mov r8, qword ptr [rcx + 0x38]
1408b3004 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
1408b3008 4885d2 test rdx, rdx
1408b300b 741a je 0x1408b3027
1408b300d 488b4918 mov rcx, qword ptr [rcx + 0x18]
1408b3011 48f7d2 not rdx
1408b3014 488d1452 lea rdx, [rdx + rdx*2]
1408b3018 48c1e204 shl rdx, 4
1408b301c 488b4c1110 mov rcx, qword ptr [rcx + rdx + 0x10]
1408b3021 48894d68 mov qword ptr [rbp + 0x68], rcx
1408b3025 eb08 jmp 0x1408b302f
1408b3027 48c7456800000000 mov qword ptr [rbp + 0x68], 0
1408b302f 488b4050 mov rax, qword ptr [rax + 0x50]
1408b3033 48894530 mov qword ptr [rbp + 0x30], rax
1408b3037 31f6 xor esi, esi
1408b3039 4889f0 mov rax, rsi
1408b303c 48c1e004 shl rax, 4
1408b3040 488b4d30 mov rcx, qword ptr [rbp + 0x30]
1408b3044 488d1401 lea rdx, [rcx + rax]
1408b3048 8b440108 mov eax, dword ptr [rcx + rax + 8]
1408b304c e9eb000000 jmp 0x1408b313c
1408b3051 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1408b3060 4839de cmp rsi, rbx
1408b3063 0f85c9000000 jne 0x1408b3132
1408b3069 0f1f8000000000 nop dword ptr [rax]
1408b3070 4d39e5 cmp r13, r12
1408b3073 0f848efdffff je 0x1408b2e07
1408b3079 418b4d00 mov ecx, dword ptr [r13]
1408b307d 4983c504 add r13, 4
1408b3081 498b87a8010000 mov rax, qword ptr [r15 + 0x1a8]
1408b3088 488d0cc9 lea rcx, [rcx + rcx*8]
1408b308c 488b5cc810 mov rbx, qword ptr [rax + rcx*8 + 0x10]
1408b3091 4885db test rbx, rbx
1408b3094 74da je 0x1408b3070
1408b3096 488d04c8 lea rax, [rax + rcx*8]
1408b309a 4c8b5538 mov r10, qword ptr [rbp + 0x38]
1408b309e 498b9210010000 mov rdx, qword ptr [r10 + 0x110]
1408b30a5 488b4840 mov rcx, qword ptr [rax + 0x40]
1408b30a9 4839d1 cmp rcx, rdx
1408b30ac 7638 jbe 0x1408b30e6
1408b30ae 4c8b4038 mov r8, qword ptr [rax + 0x38]
1408b30b2 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
1408b30b6 4885d2 test rdx, rdx
1408b30b9 742b je 0x1408b30e6
1408b30bb 4c8b4018 mov r8, qword ptr [rax + 0x18]
1408b30bf 48f7d2 not rdx
1408b30c2 488d1452 lea rdx, [rdx + rdx*2]
1408b30c6 48c1e204 shl rdx, 4
1408b30ca 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
1408b30cf 48895558 mov qword ptr [rbp + 0x58], rdx
1408b30d3 b201 mov dl, 1
1408b30d5 895574 mov dword ptr [rbp + 0x74], edx
1408b30d8 498b9218010000 mov rdx, qword ptr [r10 + 0x118]
1408b30df 4839d1 cmp rcx, rdx
1408b30e2 7715 ja 0x1408b30f9
1408b30e4 eb3a jmp 0x1408b3120
1408b30e6 c7457400000000 mov dword ptr [rbp + 0x74], 0
1408b30ed 498b9218010000 mov rdx, qword ptr [r10 + 0x118]
1408b30f4 4839d1 cmp rcx, rdx
1408b30f7 7627 jbe 0x1408b3120
1408b30f9 488b4838 mov rcx, qword ptr [rax + 0x38]
1408b30fd 488b0cd1 mov rcx, qword ptr [rcx + rdx*8]
1408b3101 4885c9 test rcx, rcx
1408b3104 741a je 0x1408b3120
1408b3106 488b5018 mov rdx, qword ptr [rax + 0x18]
1408b310a 48f7d1 not rcx
1408b310d 488d0c49 lea rcx, [rcx + rcx*2]
1408b3111 48c1e104 shl rcx, 4
1408b3115 488b4c0a10 mov rcx, qword ptr [rdx + rcx + 0x10]
1408b311a 48894d68 mov qword ptr [rbp + 0x68], rcx
1408b311e eb08 jmp 0x1408b3128
1408b3120 48c7456800000000 mov qword ptr [rbp + 0x68], 0
1408b3128 488b4008 mov rax, qword ptr [rax + 8]
1408b312c 48894528 mov qword ptr [rbp + 0x28], rax
1408b3130 31f6 xor esi, esi
1408b3132 488b4528 mov rax, qword ptr [rbp + 0x28]
1408b3136 488d14f0 lea rdx, [rax + rsi*8]
1408b313a 89f0 mov eax, esi
1408b313c 488d0c40 lea rcx, [rax + rax*2]
1408b3140 488b4568 mov rax, qword ptr [rbp + 0x68]
1408b3144 488d0488 lea rax, [rax + rcx*4]
1408b3148 48c1e104 shl rcx, 4
1408b314c 48034d58 add rcx, qword ptr [rbp + 0x58]
1408b3150 f6457401 test byte ptr [rbp + 0x74], 1
1408b3154 41b800000000 mov r8d, 0
1408b315a 490f44c8 cmove rcx, r8
1408b315e 488b3a mov rdi, qword ptr [rdx]
1408b3161 4889fa mov rdx, rdi
1408b3164 48c1ea20 shr rdx, 0x20
1408b3168 0f8499fcffff je 0x1408b2e07
1408b316e 48ffc6 inc rsi
1408b3171 4885c9 test rcx, rcx
1408b3174 743d je 0x1408b31b3
1408b3176 498b03 mov rax, qword ptr [r11]
1408b3179 4d8b7310 mov r14, qword ptr [r11 + 0x10]
1408b317d 4c29f0 sub rax, r14
1408b3180 4883f817 cmp rax, 0x17
1408b3184 0f860e010000 jbe 0x1408b3298
1408b318a 498b4308 mov rax, qword ptr [r11 + 8]
1408b318e 488d0ddb9ca200 lea rcx, [rip + 0xa29cdb]
1408b3195 4a890c30 mov qword ptr [rax + r14], rcx
1408b3199 488d0d30aaa200 lea rcx, [rip + 0xa2aa30]
1408b31a0 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1408b31a5 4a897c3010 mov qword ptr [rax + r14 + 0x10], rdi
1408b31aa 4983c618 add r14, 0x18
1408b31ae e9c9000000 jmp 0x1408b327c
1408b31b3 41f60601 test byte ptr [r14], 1
1408b31b7 7414 je 0x1408b31cd
1408b31b9 4c89f1 mov rcx, r14
1408b31bc 4d89de mov r14, r11
1408b31bf f2440f104104 movsd xmm8, qword ptr [rcx + 4]
1408b31c5 f3440f10490c movss xmm9, dword ptr [rcx + 0xc]
1408b31cb eb0b jmp 0x1408b31d8
1408b31cd 4d89de mov r14, r11
1408b31d0 450f57c0 xorps xmm8, xmm8
1408b31d4 450f57c9 xorps xmm9, xmm9
1408b31d8 f3440f105804 movss xmm11, dword ptr [rax + 4]
1408b31de 488d4dc0 lea rcx, [rbp - 0x40]
1408b31e2 e8999f3e00 call 0x140c9d180
1408b31e7 4d89f3 mov r11, r14
1408b31ea 450f28d3 movaps xmm10, xmm11
1408b31ee f3440f59d6 mulss xmm10, xmm6
1408b31f3 450f14d3 unpcklps xmm10, xmm11
1408b31f7 440f59d7 mulps xmm10, xmm7
1408b31fb 440f285dc0 movaps xmm11, xmmword ptr [rbp - 0x40]
1408b3200 488d4ddc lea rcx, [rbp - 0x24]
1408b3204 488b01 mov rax, qword ptr [rcx]
1408b3207 48894510 mov qword ptr [rbp + 0x10], rax
1408b320b 8b4108 mov eax, dword ptr [rcx + 8]
1408b320e 894518 mov dword ptr [rbp + 0x18], eax
1408b3211 498b06 mov rax, qword ptr [r14]
1408b3214 4d8b7610 mov r14, qword ptr [r14 + 0x10]
1408b3218 4c29f0 sub rax, r14
1408b321b 4883f867 cmp rax, 0x67
1408b321f 0f86a0000000 jbe 0x1408b32c5
1408b3225 f3450f58ca addss xmm9, xmm10
1408b322a 450f58c2 addps xmm8, xmm10
1408b322e 498b4308 mov rax, qword ptr [r11 + 8]
1408b3232 488d0d8786a200 lea rcx, [rip + 0xa28687]
1408b3239 4a890c30 mov qword ptr [rax + r14], rcx
1408b323d 460f115c3008 movups xmmword ptr [rax + r14 + 8], xmm11
1408b3243 460f13443018 movlps qword ptr [rax + r14 + 0x18], xmm8
1408b3249 f3460f114c3020 movss dword ptr [rax + r14 + 0x20], xmm9
1408b3250 488b4d10 mov rcx, qword ptr [rbp + 0x10]
1408b3254 4a894c3024 mov qword ptr [rax + r14 + 0x24], rcx
1408b3259 8b4d18 mov ecx, dword ptr [rbp + 0x18]
1408b325c 42894c302c mov dword ptr [rax + r14 + 0x2c], ecx
1408b3261 42c644303800 mov byte ptr [rax + r14 + 0x38], 0
1408b3267 4a897c3048 mov qword ptr [rax + r14 + 0x48], rdi
1408b326c 488d0dbd1acb00 lea rcx, [rip + 0xcb1abd]
1408b3273 4a894c3058 mov qword ptr [rax + r14 + 0x58], rcx
1408b3278 4983c668 add r14, 0x68
1408b327c 4d897310 mov qword ptr [r11 + 0x10], r14
1408b3280 440fb64d60 movzx r9d, byte ptr [rbp + 0x60]
1408b3285 4c8b7520 mov r14, qword ptr [rbp + 0x20]
1408b3289 807d4000 cmp byte ptr [rbp + 0x40], 0
1408b328d 0f848ffcffff je 0x1408b2f22
1408b3293 e9c8fdffff jmp 0x1408b3060
1408b3298 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1408b32a1 41b818000000 mov r8d, 0x18
1408b32a7 41b901000000 mov r9d, 1
1408b32ad 4c89d9 mov rcx, r11
1408b32b0 4c89f2 mov rdx, r14
1408b32b3 4c895d48 mov qword ptr [rbp + 0x48], r11
1408b32b7 e8a467ff01 call 0x1428a9a60
1408b32bc 4c8b5d48 mov r11, qword ptr [rbp + 0x48]
1408b32c0 e9c5feffff jmp 0x1408b318a
1408b32c5 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1408b32ce 41b868000000 mov r8d, 0x68
1408b32d4 41b901000000 mov r9d, 1
1408b32da 4c89d9 mov rcx, r11
1408b32dd 4c89f2 mov rdx, r14
1408b32e0 4c895d48 mov qword ptr [rbp + 0x48], r11
1408b32e4 e87767ff01 call 0x1428a9a60
1408b32e9 4c8b5d48 mov r11, qword ptr [rbp + 0x48]
1408b32ed e933ffffff jmp 0x1408b3225
1408b32f2 4885c0 test rax, rax
1408b32f5 752d jne 0x1408b3324
1408b32f7 31d2 xor edx, edx
1408b32f9 e8d2540402 call 0x1428f87d0
1408b32fe 4889c1 mov rcx, rax
1408b3301 4885c0 test rax, rax
1408b3304 0f856cfaffff jne 0x1408b2d76
1408b330a eb18 jmp 0x1408b3324
1408b330c 4885c0 test rax, rax
1408b330f 7513 jne 0x1408b3324
1408b3311 31d2 xor edx, edx
1408b3313 e8b8540402 call 0x1428f87d0
1408b3318 4889c1 mov rcx, rax
1408b331b 4885c0 test rax, rax
1408b331e 0f8501fbffff jne 0x1408b2e25
1408b3324 488d0d75773802 lea rcx, [rip + 0x2387775]
1408b332b e8a04d0202 call 0x1428d80d0
1408b3330 488d0d39783802 lea rcx, [rip + 0x2387839]
1408b3337 e8145f0202 call 0x1428d9250
1408b333c cc int3 
1408b333d 0f1f00 nop dword ptr [rax]
1408b3340 4889542410 mov qword ptr [rsp + 0x10], rdx
1408b3345 55 push rbp
1408b3346 4157 push r15
1408b3348 4156 push r14
1408b334a 4155 push r13
1408b334c 4154 push r12
1408b334e 56 push rsi
1408b334f 57 push rdi
1408b3350 53 push rbx
1408b3351 4881ec98000000 sub rsp, 0x98
1408b3358 488daa80000000 lea rbp, [rdx + 0x80]
1408b335f 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
1408b3365 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
1408b336b 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1408b3371 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
1408b3377 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1408b337c 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1408b3384 807d6000 cmp byte ptr [rbp + 0x60], 0
1408b3388 741f je 0x1408b33a9
1408b338a 488d45f8 lea rax, [rbp - 8]
1408b338e 48894508 mov qword ptr [rbp + 8], rax
1408b3392 488d4508 lea rax, [rbp + 8]
1408b3396 48894500 mov qword ptr [rbp], rax
1408b339a 488d0d57bb1e02 lea rcx, [rip + 0x21ebb57]
1408b33a1 4889ea mov rdx, rbp
1408b33a4 e827bfa700 call 0x14132f2d0
1408b33a9 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1408b33b1 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
1408b33b6 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
1408b33bc 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1408b33c2 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
1408b33c8 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
1408b33ce 4881c498000000 add rsp, 0x98
1408b33d5 5b pop rbx
1408b33d6 5f pop rdi
1408b33d7 5e pop rsi
1408b33d8 415c pop r12
1408b33da 415d pop r13
1408b33dc 415e pop r14
1408b33de 415f pop r15
1408b33e0 5d pop rbp
1408b33e1 c3 ret 
1408b33e2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1408b33f0 4889542410 mov qword ptr [rsp + 0x10], rdx
1408b33f5 55 push rbp
1408b33f6 4157 push r15
1408b33f8 4156 push r14
1408b33fa 4155 push r13
1408b33fc 4154 push r12
1408b33fe 56 push rsi
1408b33ff 57 push rdi
1408b3400 53 push rbx
1408b3401 4881ec98000000 sub rsp, 0x98
1408b3408 488daa80000000 lea rbp, [rdx + 0x80]
1408b340f 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
1408b3415 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
1408b341b 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1408b3421 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
1408b3427 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1408b342c 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1408b3434 488b4560 mov rax, qword ptr [rbp + 0x60]
1408b3438 48ff00 inc qword ptr [rax]
1408b343b 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1408b3443 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
1408b3448 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
1408b344e 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1408b3454 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
1408b345a 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
1408b3460 4881c498000000 add rsp, 0x98
1408b3467 5b pop rbx
1408b3468 5f pop rdi
1408b3469 5e pop rsi
1408b346a 415c pop r12
1408b346c 415d pop r13
1408b346e 415e pop r14
1408b3470 415f pop r15
1408b3472 5d pop rbp
1408b3473 c3 ret 
1408b3474 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1408b3480 4889542410 mov qword ptr [rsp + 0x10], rdx
1408b3485 55 push rbp
1408b3486 4157 push r15
1408b3488 4156 push r14
1408b348a 4155 push r13
1408b348c 4154 push r12
1408b348e 56 push rsi
1408b348f 57 push rdi
1408b3490 53 push rbx
1408b3491 4881ec98000000 sub rsp, 0x98
1408b3498 488daa80000000 lea rbp, [rdx + 0x80]
1408b349f 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
1408b34a5 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
1408b34ab 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1408b34b1 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
1408b34b7 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1408b34bc 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1408b34c4 488b4550 mov rax, qword ptr [rbp + 0x50]
1408b34c8 48ff00 inc qword ptr [rax]
1408b34cb 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1408b34d3 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
1408b34d8 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
1408b34de 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1408b34e4 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
1408b34ea 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
1408b34f0 4881c498000000 add rsp, 0x98
1408b34f7 5b pop rbx
1408b34f8 5f pop rdi
1408b34f9 5e pop rsi
1408b34fa 415c pop r12
1408b34fc 415d pop r13
1408b34fe 415e pop r14
1408b3500 415f pop r15
1408b3502 5d pop rbp
1408b3503 c3 ret 
