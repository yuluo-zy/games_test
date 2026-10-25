0x1416e99c0 55 push rbp
0x1416e99c1 4157 push r15
0x1416e99c3 4156 push r14
0x1416e99c5 4154 push r12
0x1416e99c7 56 push rsi
0x1416e99c8 57 push rdi
0x1416e99c9 53 push rbx
0x1416e99ca 4881ec40010000 sub rsp, 0x140
0x1416e99d1 488dac2480000000 lea rbp, [rsp + 0x80]
0x1416e99d9 48c785b8000000feffffff mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
0x1416e99e4 4889ce mov rsi, rcx
0x1416e99e7 0fb602 movzx eax, byte ptr [rdx]
0x1416e99ea 83f803 cmp eax, 3
0x1416e99ed 8985a0000000 mov dword ptr [rbp + 0xa0], eax
0x1416e99f3 48895578 mov qword ptr [rbp + 0x78], rdx
0x1416e99f7 0f84ed000000 je 0x1416e9aea
0x1416e99fd 83f805 cmp eax, 5
0x1416e9a00 0f85cf010000 jne 0x1416e9bd5
0x1416e9a06 4c8b7a08 mov r15, qword ptr [rdx + 8]
0x1416e9a0a 4c8b6210 mov r12, qword ptr [rdx + 0x10]
0x1416e9a0e 488b7a18 mov rdi, qword ptr [rdx + 0x18]
0x1416e9a12 31c0 xor eax, eax
0x1416e9a14 4d85ff test r15, r15
0x1416e9a17 0f95c0 setne al
0x1416e9a1a 4889f9 mov rcx, rdi
0x1416e9a1d 490f44cf cmove rcx, r15
0x1416e9a21 48894520 mov qword ptr [rbp + 0x20], rax
0x1416e9a25 48c7452800000000 mov qword ptr [rbp + 0x28], 0
0x1416e9a2d 4c897d30 mov qword ptr [rbp + 0x30], r15
0x1416e9a31 4c896538 mov qword ptr [rbp + 0x38], r12
0x1416e9a35 48894540 mov qword ptr [rbp + 0x40], rax
0x1416e9a39 48c7454800000000 mov qword ptr [rbp + 0x48], 0
0x1416e9a41 4c897d50 mov qword ptr [rbp + 0x50], r15
0x1416e9a45 4c896558 mov qword ptr [rbp + 0x58], r12
0x1416e9a49 48894d60 mov qword ptr [rbp + 0x60], rcx
0x1416e9a4d 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416e9a54 488d5520 lea rdx, [rbp + 0x20]
0x1416e9a58 e8f329d5ff call 0x14143c450
0x1416e9a5d 488b8580000000 mov rax, qword ptr [rbp + 0x80]
0x1416e9a64 4885c0 test rax, rax
0x1416e9a67 7444 je 0x1416e9aad
0x1416e9a69 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1416e9a70 488d1449 lea rdx, [rcx + rcx*2]
0x1416e9a74 4c8b84d068010000 mov r8, qword ptr [rax + rdx*8 + 0x168]
0x1416e9a7c 4c8b8cd070010000 mov r9, qword ptr [rax + rdx*8 + 0x170]
0x1416e9a84 488b9cd078010000 mov rbx, qword ptr [rax + rdx*8 + 0x178]
0x1416e9a8c 48c1e105 shl rcx, 5
0x1416e9a90 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1416e9a94 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1416e9a99 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1416e9a9d 0f294df0 movaps xmmword ptr [rbp - 0x10], xmm1
0x1416e9aa1 4c89c0 mov rax, r8
0x1416e9aa4 48f7d8 neg rax
0x1416e9aa7 0f81b5010000 jno 0x1416e9c62
0x1416e9aad c685800000000b mov byte ptr [rbp + 0x80], 0xb
0x1416e9ab4 488d152d575a01 lea rdx, [rip + 0x15a572d]
0x1416e9abb 4c8d0536575a01 lea r8, [rip + 0x15a5736]
0x1416e9ac2 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416e9ac9 e872002101 call 0x1428f9b40
0x1416e9ace 48894608 mov qword ptr [rsi + 8], rax
0x1416e9ad2 c60601 mov byte ptr [rsi], 1
0x1416e9ad5 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e9adc 488d4d20 lea rcx, [rbp + 0x20]
0x1416e9ae0 e80be319ff call 0x140887df0
0x1416e9ae5 e973020000 jmp 0x1416e9d5d
0x1416e9aea 4c8b7a08 mov r15, qword ptr [rdx + 8]
0x1416e9aee 4c8b6210 mov r12, qword ptr [rdx + 0x10]
0x1416e9af2 488b7a18 mov rdi, qword ptr [rdx + 0x18]
0x1416e9af6 41b606 mov r14b, 6
0x1416e9af9 4c89f8 mov rax, r15
0x1416e9afc 4c89e1 mov rcx, r12
0x1416e9aff 4889fb mov rbx, rdi
0x1416e9b02 48894520 mov qword ptr [rbp + 0x20], rax
0x1416e9b06 48894d28 mov qword ptr [rbp + 0x28], rcx
0x1416e9b0a 48895d30 mov qword ptr [rbp + 0x30], rbx
0x1416e9b0e 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
0x1416e9b12 0f114539 movups xmmword ptr [rbp + 0x39], xmm0
0x1416e9b16 0f1045cf movups xmm0, xmmword ptr [rbp - 0x31]
0x1416e9b1a 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1416e9b1e 44887538 mov byte ptr [rbp + 0x38], r14b
0x1416e9b22 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e9b29 488d4de0 lea rcx, [rbp - 0x20]
0x1416e9b2d 488d5520 lea rdx, [rbp + 0x20]
0x1416e9b31 e89a8beeff call 0x1415d26d0
0x1416e9b36 0fb65de0 movzx ebx, byte ptr [rbp - 0x20]
0x1416e9b3a 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x1416e9b3e 80fb02 cmp bl, 2
0x1416e9b41 7439 je 0x1416e9b7c
0x1416e9b43 488b4d00 mov rcx, qword ptr [rbp]
0x1416e9b47 48898d98000000 mov qword ptr [rbp + 0x98], rcx
0x1416e9b4e 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
0x1416e9b52 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x1416e9b59 48898580000000 mov qword ptr [rbp + 0x80], rax
0x1416e9b60 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e9b67 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416e9b6e e88de449ff call 0x140b88000
0x1416e9b73 4885c0 test rax, rax
0x1416e9b76 0f849f010000 je 0x1416e9d1b
0x1416e9b7c 48894608 mov qword ptr [rsi + 8], rax
0x1416e9b80 b001 mov al, 1
0x1416e9b82 8806 mov byte ptr [rsi], al
0x1416e9b84 8b85a0000000 mov eax, dword ptr [rbp + 0xa0]
0x1416e9b8a 83f803 cmp eax, 3
0x1416e9b8d 0f84ca010000 je 0x1416e9d5d
0x1416e9b93 83f805 cmp eax, 5
0x1416e9b96 0f84c1010000 je 0x1416e9d5d
0x1416e9b9c 4c89bda0000000 mov qword ptr [rbp + 0xa0], r15
0x1416e9ba3 4c896578 mov qword ptr [rbp + 0x78], r12
0x1416e9ba7 4c89e1 mov rcx, r12
0x1416e9baa 4889fa mov rdx, rdi
0x1416e9bad e8bef119ff call 0x140888d70
0x1416e9bb2 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x1416e9bb9 4885d2 test rdx, rdx
0x1416e9bbc 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416e9bc0 0f8497010000 je 0x1416e9d5d
0x1416e9bc6 48c1e205 shl rdx, 5
0x1416e9bca 41b808000000 mov r8d, 8
0x1416e9bd0 e971010000 jmp 0x1416e9d46
0x1416e9bd5 0f1002 movups xmm0, xmmword ptr [rdx]
0x1416e9bd8 0f104a10 movups xmm1, xmmword ptr [rdx + 0x10]
0x1416e9bdc 0f294d30 movaps xmmword ptr [rbp + 0x30], xmm1
0x1416e9be0 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x1416e9be4 488d4de0 lea rcx, [rbp - 0x20]
0x1416e9be8 488d5520 lea rdx, [rbp + 0x20]
0x1416e9bec e81f001e01 call 0x1428c9c10
0x1416e9bf1 488d1530565a01 lea rdx, [rip + 0x15a5630]
0x1416e9bf8 4c8d05f9555a01 lea r8, [rip + 0x15a55f9]
0x1416e9bff 488d4de0 lea rcx, [rbp - 0x20]
0x1416e9c03 e8f8fd1d01 call 0x1428c9a00
0x1416e9c08 48894608 mov qword ptr [rsi + 8], rax
0x1416e9c0c c60601 mov byte ptr [rsi], 1
0x1416e9c0f 0fb64520 movzx eax, byte ptr [rbp + 0x20]
0x1416e9c13 83f803 cmp eax, 3
0x1416e9c16 0f8241010000 jb 0x1416e9d5d
0x1416e9c1c 0f8411010000 je 0x1416e9d33
0x1416e9c22 83f804 cmp eax, 4
0x1416e9c25 0f8522010000 jne 0x1416e9d4d
0x1416e9c2b 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x1416e9c2f 488b5538 mov rdx, qword ptr [rbp + 0x38]
0x1416e9c33 48898da8000000 mov qword ptr [rbp + 0xa8], rcx
0x1416e9c3a e831f119ff call 0x140888d70
0x1416e9c3f 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416e9c43 4885d2 test rdx, rdx
0x1416e9c46 0f8411010000 je 0x1416e9d5d
0x1416e9c4c 48c1e205 shl rdx, 5
0x1416e9c50 41b808000000 mov r8d, 8
0x1416e9c56 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x1416e9c5d e9e4000000 jmp 0x1416e9d46
0x1416e9c62 4c894d70 mov qword ptr [rbp + 0x70], r9
0x1416e9c66 4c8985a8000000 mov qword ptr [rbp + 0xa8], r8
0x1416e9c6d 0f2845e0 movaps xmm0, xmmword ptr [rbp - 0x20]
0x1416e9c71 0f284df0 movaps xmm1, xmmword ptr [rbp - 0x10]
0x1416e9c75 0f298d90000000 movaps xmmword ptr [rbp + 0x90], xmm1
0x1416e9c7c 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
0x1416e9c83 488d4da8 lea rcx, [rbp - 0x58]
0x1416e9c87 488d5520 lea rdx, [rbp + 0x20]
0x1416e9c8b e8c027d5ff call 0x14143c450
0x1416e9c90 488b45a8 mov rax, qword ptr [rbp - 0x58]
0x1416e9c94 4885c0 test rax, rax
0x1416e9c97 7445 je 0x1416e9cde
0x1416e9c99 488b4db8 mov rcx, qword ptr [rbp - 0x48]
0x1416e9c9d 488d1449 lea rdx, [rcx + rcx*2]
0x1416e9ca1 4c8b84d078010000 mov r8, qword ptr [rax + rdx*8 + 0x178]
0x1416e9ca9 4c8945f0 mov qword ptr [rbp - 0x10], r8
0x1416e9cad 0f1084d068010000 movups xmm0, xmmword ptr [rax + rdx*8 + 0x168]
0x1416e9cb5 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1416e9cb9 48c1e105 shl rcx, 5
0x1416e9cbd 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1416e9cc1 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1416e9cc6 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x1416e9cca 0f114d08 movups xmmword ptr [rbp + 8], xmm1
0x1416e9cce 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x1416e9cd2 4889d0 mov rax, rdx
0x1416e9cd5 48f7d8 neg rax
0x1416e9cd8 0f8194000000 jno 0x1416e9d72
0x1416e9cde 448ab580000000 mov r14b, byte ptr [rbp + 0x80]
0x1416e9ce5 0f108581000000 movups xmm0, xmmword ptr [rbp + 0x81]
0x1416e9cec 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
0x1416e9cf0 0f108590000000 movups xmm0, xmmword ptr [rbp + 0x90]
0x1416e9cf7 0f1145cf movups xmmword ptr [rbp - 0x31], xmm0
0x1416e9cfb c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e9d02 488d4d20 lea rcx, [rbp + 0x20]
0x1416e9d06 e8e5e019ff call 0x140887df0
0x1416e9d0b 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
0x1416e9d12 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416e9d16 e9e7fdffff jmp 0x1416e9b02
0x1416e9d1b 885e01 mov byte ptr [rsi + 1], bl
0x1416e9d1e 31c0 xor eax, eax
0x1416e9d20 8806 mov byte ptr [rsi], al
0x1416e9d22 8b85a0000000 mov eax, dword ptr [rbp + 0xa0]
0x1416e9d28 83f803 cmp eax, 3
0x1416e9d2b 0f8562feffff jne 0x1416e9b93
0x1416e9d31 eb2a jmp 0x1416e9d5d
0x1416e9d33 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416e9d37 4885d2 test rdx, rdx
0x1416e9d3a 7421 je 0x1416e9d5d
0x1416e9d3c 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x1416e9d40 41b801000000 mov r8d, 1
0x1416e9d46 e8d59ef2fe call 0x140613c20
0x1416e9d4b eb10 jmp 0x1416e9d5d
0x1416e9d4d 488d4d28 lea rcx, [rbp + 0x28]
0x1416e9d51 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416e9d58 e8e3f11aff call 0x140898f40
0x1416e9d5d 4889f0 mov rax, rsi
0x1416e9d60 4881c440010000 add rsp, 0x140
0x1416e9d67 5b pop rbx
0x1416e9d68 5f pop rdi
0x1416e9d69 5e pop rsi
0x1416e9d6a 415c pop r12
0x1416e9d6c 415e pop r14
0x1416e9d6e 415f pop r15
0x1416e9d70 5d pop rbp
0x1416e9d71 c3 ret 
0x1416e9d72 730f jae 0x1416e9d83
0x1416e9d74 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x1416e9d78 41b801000000 mov r8d, 1
0x1416e9d7e e89d9ef2fe call 0x140613c20
0x1416e9d83 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1416e9d87 83f803 cmp eax, 3
0x1416e9d8a 7231 jb 0x1416e9dbd
0x1416e9d8c 7410 je 0x1416e9d9e
0x1416e9d8e 488d4d00 lea rcx, [rbp]
0x1416e9d92 83f804 cmp eax, 4
0x1416e9d95 7521 jne 0x1416e9db8
0x1416e9d97 e8f4f519ff call 0x140889390
0x1416e9d9c eb1f jmp 0x1416e9dbd
0x1416e9d9e 488b5500 mov rdx, qword ptr [rbp]
0x1416e9da2 4885d2 test rdx, rdx
0x1416e9da5 7416 je 0x1416e9dbd
0x1416e9da7 488b4d08 mov rcx, qword ptr [rbp + 8]
0x1416e9dab 41b801000000 mov r8d, 1
0x1416e9db1 e86a9ef2fe call 0x140613c20
0x1416e9db6 eb05 jmp 0x1416e9dbd
0x1416e9db8 e883f11aff call 0x140898f40
0x1416e9dbd c645e00b mov byte ptr [rbp - 0x20], 0xb
0x1416e9dc1 488d1520545a01 lea rdx, [rip + 0x15a5420]
0x1416e9dc8 4c8d0529545a01 lea r8, [rip + 0x15a5429]
0x1416e9dcf 488d4de0 lea rcx, [rbp - 0x20]
0x1416e9dd3 e868fd2001 call 0x1428f9b40
0x1416e9dd8 48894608 mov qword ptr [rsi + 8], rax
0x1416e9ddc c60601 mov byte ptr [rsi], 1
0x1416e9ddf 0fb68580000000 movzx eax, byte ptr [rbp + 0x80]
0x1416e9de6 83f803 cmp eax, 3
0x1416e9de9 723a jb 0x1416e9e25
0x1416e9deb 7413 je 0x1416e9e00
0x1416e9ded 488d8d88000000 lea rcx, [rbp + 0x88]
0x1416e9df4 83f804 cmp eax, 4
0x1416e9df7 7527 jne 0x1416e9e20
0x1416e9df9 e892f519ff call 0x140889390
0x1416e9dfe eb25 jmp 0x1416e9e25
0x1416e9e00 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
0x1416e9e07 4885d2 test rdx, rdx
0x1416e9e0a 7419 je 0x1416e9e25
0x1416e9e0c 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1416e9e13 41b801000000 mov r8d, 1
0x1416e9e19 e8029ef2fe call 0x140613c20
0x1416e9e1e eb05 jmp 0x1416e9e25
0x1416e9e20 e81bf11aff call 0x140898f40
0x1416e9e25 488b95a8000000 mov rdx, qword ptr [rbp + 0xa8]
0x1416e9e2c 4885d2 test rdx, rdx
0x1416e9e2f 0f84a0fcffff je 0x1416e9ad5
0x1416e9e35 41b801000000 mov r8d, 1
0x1416e9e3b 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416e9e3f e8dc9df2fe call 0x140613c20
0x1416e9e44 e98cfcffff jmp 0x1416e9ad5
0x1416e9e49 0f1f8000000000 nop dword ptr [rax]
0x1416e9e50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e9e55 55 push rbp
0x1416e9e56 4157 push r15
0x1416e9e58 4156 push r14
0x1416e9e5a 4154 push r12
0x1416e9e5c 56 push rsi
0x1416e9e5d 57 push rdi
0x1416e9e5e 53 push rbx
0x1416e9e5f 4883ec20 sub rsp, 0x20
0x1416e9e63 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e9e6a 488d4d20 lea rcx, [rbp + 0x20]
0x1416e9e6e e87ddf19ff call 0x140887df0
0x1416e9e73 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e9e7a 4883c420 add rsp, 0x20
0x1416e9e7e 5b pop rbx
0x1416e9e7f 5f pop rdi
0x1416e9e80 5e pop rsi
0x1416e9e81 415c pop r12
0x1416e9e83 415e pop r14
0x1416e9e85 415f pop r15
0x1416e9e87 5d pop rbp
0x1416e9e88 c3 ret 
0x1416e9e89 0f1f8000000000 nop dword ptr [rax]
0x1416e9e90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e9e95 55 push rbp
0x1416e9e96 4157 push r15
0x1416e9e98 4156 push r14
0x1416e9e9a 4154 push r12
0x1416e9e9c 56 push rsi
0x1416e9e9d 57 push rdi
0x1416e9e9e 53 push rbx
0x1416e9e9f 4883ec20 sub rsp, 0x20
0x1416e9ea3 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e9eaa 4883bda800000000 cmp qword ptr [rbp + 0xa8], 0
0x1416e9eb2 7416 je 0x1416e9eca
0x1416e9eb4 41b801000000 mov r8d, 1
0x1416e9eba 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416e9ebe 488b95a8000000 mov rdx, qword ptr [rbp + 0xa8]
0x1416e9ec5 e8569df2fe call 0x140613c20
0x1416e9eca 90 nop 
0x1416e9ecb 4883c420 add rsp, 0x20
0x1416e9ecf 5b pop rbx
0x1416e9ed0 5f pop rdi
0x1416e9ed1 5e pop rsi
0x1416e9ed2 415c pop r12
0x1416e9ed4 415e pop r14
0x1416e9ed6 415f pop r15
0x1416e9ed8 5d pop rbp
0x1416e9ed9 c3 ret 
0x1416e9eda 660f1f440000 nop word ptr [rax + rax]
0x1416e9ee0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e9ee5 55 push rbp
0x1416e9ee6 4157 push r15
0x1416e9ee8 4156 push r14
0x1416e9eea 4154 push r12
0x1416e9eec 56 push rsi
0x1416e9eed 57 push rdi
0x1416e9eee 53 push rbx
0x1416e9eef 4883ec20 sub rsp, 0x20
0x1416e9ef3 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e9efa 4883bda000000000 cmp qword ptr [rbp + 0xa0], 0
0x1416e9f02 741a je 0x1416e9f1e
0x1416e9f04 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x1416e9f0b 48c1e205 shl rdx, 5
0x1416e9f0f 41b808000000 mov r8d, 8
0x1416e9f15 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416e9f19 e8029df2fe call 0x140613c20
0x1416e9f1e 90 nop 
0x1416e9f1f 4883c420 add rsp, 0x20
0x1416e9f23 5b pop rbx
0x1416e9f24 5f pop rdi
0x1416e9f25 5e pop rsi
0x1416e9f26 415c pop r12
0x1416e9f28 415e pop r14
0x1416e9f2a 415f pop r15
0x1416e9f2c 5d pop rbp
0x1416e9f2d c3 ret 
0x1416e9f2e 6690 nop 
0x1416e9f30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e9f35 55 push rbp
0x1416e9f36 4157 push r15
0x1416e9f38 4156 push r14
0x1416e9f3a 4154 push r12
0x1416e9f3c 56 push rsi
0x1416e9f3d 57 push rdi
0x1416e9f3e 53 push rbx
0x1416e9f3f 4883ec20 sub rsp, 0x20
0x1416e9f43 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e9f4a 488d4d20 lea rcx, [rbp + 0x20]
0x1416e9f4e e83d3b17ff call 0x14085da90
0x1416e9f53 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416e9f5a 4883c420 add rsp, 0x20
0x1416e9f5e 5b pop rbx
0x1416e9f5f 5f pop rdi
0x1416e9f60 5e pop rsi
0x1416e9f61 415c pop r12
0x1416e9f63 415e pop r14
0x1416e9f65 415f pop r15
0x1416e9f67 5d pop rbp
0x1416e9f68 c3 ret 
0x1416e9f69 0f1f8000000000 nop dword ptr [rax]
0x1416e9f70 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e9f75 55 push rbp
0x1416e9f76 4157 push r15
0x1416e9f78 4156 push r14
0x1416e9f7a 4154 push r12
0x1416e9f7c 56 push rsi
0x1416e9f7d 57 push rdi
0x1416e9f7e 53 push rbx
0x1416e9f7f 4883ec20 sub rsp, 0x20
0x1416e9f83 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e9f8a 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416e9f8e 4885d2 test rdx, rdx
0x1416e9f91 7416 je 0x1416e9fa9
0x1416e9f93 48c1e205 shl rdx, 5
0x1416e9f97 41b808000000 mov r8d, 8
0x1416e9f9d 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x1416e9fa4 e8779cf2fe call 0x140613c20
0x1416e9fa9 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416e9fb0 4883c420 add rsp, 0x20
0x1416e9fb4 5b pop rbx
0x1416e9fb5 5f pop rdi
0x1416e9fb6 5e pop rsi
0x1416e9fb7 415c pop r12
0x1416e9fb9 415e pop r14
0x1416e9fbb 415f pop r15
0x1416e9fbd 5d pop rbp
0x1416e9fbe c3 ret 
0x1416e9fbf 90 nop 
0x1416e9fc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e9fc5 55 push rbp
0x1416e9fc6 4157 push r15
0x1416e9fc8 4156 push r14
0x1416e9fca 4154 push r12
0x1416e9fcc 56 push rsi
0x1416e9fcd 57 push rdi
0x1416e9fce 53 push rbx
0x1416e9fcf 4883ec20 sub rsp, 0x20
0x1416e9fd3 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e9fda 80bdb700000000 cmp byte ptr [rbp + 0xb7], 0
0x1416e9fe1 7415 je 0x1416e9ff8
0x1416e9fe3 80bda000000003 cmp byte ptr [rbp + 0xa0], 3
0x1416e9fea 740c je 0x1416e9ff8
0x1416e9fec 0fb685a0000000 movzx eax, byte ptr [rbp + 0xa0]
0x1416e9ff3 83f805 cmp eax, 5
0x1416e9ff6 750f jne 0x1416ea007
0x1416e9ff8 4883c420 add rsp, 0x20
0x1416e9ffc 5b pop rbx
0x1416e9ffd 5f pop rdi
0x1416e9ffe 5e pop rsi
0x1416e9fff 415c pop r12
0x1416ea001 415e pop r14
0x1416ea003 415f pop r15
0x1416ea005 5d pop rbp
0x1416ea006 c3 ret 
0x1416ea007 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416ea00b e8803a17ff call 0x14085da90
0x1416ea010 ebe6 jmp 0x1416e9ff8
0x1416ea012 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1416ea020 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ea025 55 push rbp
0x1416ea026 4157 push r15
0x1416ea028 4156 push r14
0x1416ea02a 4154 push r12
0x1416ea02c 56 push rsi
0x1416ea02d 57 push rdi
0x1416ea02e 53 push rbx
0x1416ea02f 4883ec20 sub rsp, 0x20
0x1416ea033 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ea03a 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416ea041 e84a3a17ff call 0x14085da90
0x1416ea046 90 nop 
0x1416ea047 4883c420 add rsp, 0x20
0x1416ea04b 5b pop rbx
0x1416ea04c 5f pop rdi
0x1416ea04d 5e pop rsi
0x1416ea04e 415c pop r12
0x1416ea050 415e pop r14
0x1416ea052 415f pop r15
0x1416ea054 5d pop rbp
0x1416ea055 c3 ret 
