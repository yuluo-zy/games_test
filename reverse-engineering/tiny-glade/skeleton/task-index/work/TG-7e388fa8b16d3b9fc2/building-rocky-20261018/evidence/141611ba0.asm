0x141611ba0 55 push rbp
0x141611ba1 4157 push r15
0x141611ba3 4156 push r14
0x141611ba5 4154 push r12
0x141611ba7 56 push rsi
0x141611ba8 57 push rdi
0x141611ba9 53 push rbx
0x141611baa 4881ec00010000 sub rsp, 0x100
0x141611bb1 488dac2480000000 lea rbp, [rsp + 0x80]
0x141611bb9 48c74578feffffff mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
0x141611bc1 4c89c3 mov rbx, r8
0x141611bc4 4889d7 mov rdi, rdx
0x141611bc7 4889ce mov rsi, rcx
0x141611bca 4d85c0 test r8, r8
0x141611bcd 745c je 0x141611c2b
0x141611bcf 48b9abaaaaaaaaaaaaaa movabs rcx, 0xaaaaaaaaaaaaaaab
0x141611bd9 4c89c8 mov rax, r9
0x141611bdc 48f7e1 mul rcx
0x141611bdf 48d1ea shr rdx, 1
0x141611be2 4883e2f0 and rdx, 0xfffffffffffffff0
0x141611be6 488d0452 lea rax, [rdx + rdx*2]
0x141611bea 4c89c9 mov rcx, r9
0x141611bed 4829c1 sub rcx, rax
0x141611bf0 48894dc0 mov qword ptr [rbp - 0x40], rcx
0x141611bf4 0f85c2000000 jne 0x141611cbc
0x141611bfa 4889d8 mov rax, rbx
0x141611bfd 4883e003 and rax, 3
0x141611c01 488945c0 mov qword ptr [rbp - 0x40], rax
0x141611c05 0f85c2000000 jne 0x141611ccd
0x141611c0b 4981f910fb6c00 cmp r9, 0x6cfb10
0x141611c12 0f8331010000 jae 0x141611d49
0x141611c18 4489c8 mov eax, r9d
0x141611c1b 41bcabaaaaaa mov r12d, 0xaaaaaaab
0x141611c21 4c0fafe0 imul r12, rax
0x141611c25 49c1ec25 shr r12, 0x25
0x141611c29 eb00 jmp 0x141611c2b
0x141611c2b 4c8d7510 lea r14, [rbp + 0x10]
0x141611c2f bae0fa6c00 mov edx, 0x6cfae0
0x141611c34 4c89f1 mov rcx, r14
0x141611c37 41b822000000 mov r8d, 0x22
0x141611c3d 41b101 mov r9b, 1
0x141611c40 e89b315cff call 0x140bd4de0
0x141611c45 4c8d7d68 lea r15, [rbp + 0x68]
0x141611c49 41b804000000 mov r8d, 4
0x141611c4f 4c89f9 mov rcx, r15
0x141611c52 4c89f2 mov rdx, r14
0x141611c55 e8c6325cff call 0x140bd4f20
0x141611c5a 49c1e404 shl r12, 4
0x141611c5e 4b8d0464 lea rax, [r12 + r12*2]
0x141611c62 4889442430 mov qword ptr [rsp + 0x30], rax
0x141611c67 48895c2428 mov qword ptr [rsp + 0x28], rbx
0x141611c6c 4c897c2420 mov qword ptr [rsp + 0x20], r15
0x141611c71 4c8d056bdf6601 lea r8, [rip + 0x166df6b]
0x141611c78 488d4dc0 lea rcx, [rbp - 0x40]
0x141611c7c 41b916000000 mov r9d, 0x16
0x141611c82 4889fa mov rdx, rdi
0x141611c85 e836110900 call 0x1416a2dc0
0x141611c8a 837dc00b cmp dword ptr [rbp - 0x40], 0xb
0x141611c8e 7567 jne 0x141611cf7
0x141611c90 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x141611c94 48894620 mov qword ptr [rsi + 0x20], rax
0x141611c98 0f1045c8 movups xmm0, xmmword ptr [rbp - 0x38]
0x141611c9c 0f104dd8 movups xmm1, xmmword ptr [rbp - 0x28]
0x141611ca0 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
0x141611ca4 0f1106 movups xmmword ptr [rsi], xmm0
0x141611ca7 4889f0 mov rax, rsi
0x141611caa 4881c400010000 add rsp, 0x100
0x141611cb1 5b pop rbx
0x141611cb2 5f pop rdi
0x141611cb3 5e pop rsi
0x141611cb4 415c pop r12
0x141611cb6 415e pop r14
0x141611cb8 415f pop r15
0x141611cba 5d pop rbp
0x141611cbb c3 ret 
0x141611cbc 48c7451000000000 mov qword ptr [rbp + 0x10], 0
0x141611cc4 488d057d826b01 lea rax, [rip + 0x16b827d]
0x141611ccb eb0f jmp 0x141611cdc
0x141611ccd 48c7451000000000 mov qword ptr [rbp + 0x10], 0
0x141611cd5 488d0584826b01 lea rax, [rip + 0x16b8284]
0x141611cdc 4889442420 mov qword ptr [rsp + 0x20], rax
0x141611ce1 4c8d0558826b01 lea r8, [rip + 0x16b8258]
0x141611ce8 488d55c0 lea rdx, [rbp - 0x40]
0x141611cec 4c8d4d10 lea r9, [rbp + 0x10]
0x141611cf0 31c9 xor ecx, ecx
0x141611cf2 e89f782c01 call 0x1428d9596
0x141611cf7 0f104500 movups xmm0, xmmword ptr [rbp]
0x141611cfb 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
0x141611cff 0f1045c0 movups xmm0, xmmword ptr [rbp - 0x40]
0x141611d03 0f104dd0 movups xmm1, xmmword ptr [rbp - 0x30]
0x141611d07 0f1055e0 movups xmm2, xmmword ptr [rbp - 0x20]
0x141611d0b 0f105df0 movups xmm3, xmmword ptr [rbp - 0x10]
0x141611d0f 0f295d40 movaps xmmword ptr [rbp + 0x40], xmm3
0x141611d13 0f295530 movaps xmmword ptr [rbp + 0x30], xmm2
0x141611d17 0f294d20 movaps xmmword ptr [rbp + 0x20], xmm1
0x141611d1b 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x141611d1f 488d0582e46601 lea rax, [rip + 0x166e482]
0x141611d26 4889442420 mov qword ptr [rsp + 0x20], rax
0x141611d2b 488d0db6cd6601 lea rcx, [rip + 0x166cdb6]
0x141611d32 4c8d0d8fcd6601 lea r9, [rip + 0x166cd8f]
0x141611d39 4c8d4510 lea r8, [rbp + 0x10]
0x141611d3d ba2b000000 mov edx, 0x2b
0x141611d42 e8197a2c01 call 0x1428d9760
0x141611d47 0f0b ud2 
0x141611d49 488d05cecf4801 lea rax, [rip + 0x148cfce]
0x141611d50 488945c0 mov qword ptr [rbp - 0x40], rax
0x141611d54 48c745c83e000000 mov qword ptr [rbp - 0x38], 0x3e
0x141611d5c 488d45c0 lea rax, [rbp - 0x40]
0x141611d60 48894568 mov qword ptr [rbp + 0x68], rax
0x141611d64 488d052581affe lea rax, [rip - 0x1507edb]
0x141611d6b 48894570 mov qword ptr [rbp + 0x70], rax
0x141611d6f 488d0552da6601 lea rax, [rip + 0x166da52]
0x141611d76 48894510 mov qword ptr [rbp + 0x10], rax
0x141611d7a 48c7451801000000 mov qword ptr [rbp + 0x18], 1
0x141611d82 48c7453000000000 mov qword ptr [rbp + 0x30], 0
0x141611d8a 488d4568 lea rax, [rbp + 0x68]
0x141611d8e 48894520 mov qword ptr [rbp + 0x20], rax
0x141611d92 48c7452801000000 mov qword ptr [rbp + 0x28], 1
0x141611d9a 488d1537da6601 lea rdx, [rip + 0x166da37]
0x141611da1 488d4d10 lea rcx, [rbp + 0x10]
0x141611da5 e8e6752c01 call 0x1428d9390
0x141611daa cc int3 
0x141611dab 0f1f440000 nop dword ptr [rax + rax]
0x141611db0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141611db5 55 push rbp
0x141611db6 4157 push r15
0x141611db8 4156 push r14
0x141611dba 4154 push r12
0x141611dbc 56 push rsi
0x141611dbd 57 push rdi
0x141611dbe 53 push rbx
0x141611dbf 4883ec40 sub rsp, 0x40
0x141611dc3 488daa80000000 lea rbp, [rdx + 0x80]
0x141611dca 488d4d10 lea rcx, [rbp + 0x10]
0x141611dce e89dcc1bff call 0x1407cea70
0x141611dd3 90 nop 
0x141611dd4 4883c440 add rsp, 0x40
0x141611dd8 5b pop rbx
0x141611dd9 5f pop rdi
0x141611dda 5e pop rsi
0x141611ddb 415c pop r12
0x141611ddd 415e pop r14
0x141611ddf 415f pop r15
0x141611de1 5d pop rbp
0x141611de2 c3 ret 
