0000000141611980 55                               push rbp
0000000141611981 4157                             push r15
0000000141611983 4156                             push r14
0000000141611985 4154                             push r12
0000000141611987 56                               push rsi
0000000141611988 57                               push rdi
0000000141611989 53                               push rbx
000000014161198a 4881ec00010000                   sub rsp, 0x100
0000000141611991 488dac2480000000                 lea rbp, [rsp + 0x80]
0000000141611999 48c74578feffffff                 mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
00000001416119a1 4c89c3                           mov rbx, r8
00000001416119a4 4889d7                           mov rdi, rdx
00000001416119a7 4889ce                           mov rsi, rcx
00000001416119aa 4d85c0                           test r8, r8
00000001416119ad 0f848f000000                     je 0x141611a42
00000001416119b3 4d89ce                           mov r14, r9
00000001416119b6 4c89c8                           mov rax, r9
00000001416119b9 4883e01f                         and rax, 0x1f
00000001416119bd 488945c0                         mov qword ptr [rbp - 0x40], rax
00000001416119c1 0f8507010000                     jne 0x141611ace
00000001416119c7 4889d8                           mov rax, rbx
00000001416119ca 4883e003                         and rax, 3
00000001416119ce 488945c0                         mov qword ptr [rbp - 0x40], rax
00000001416119d2 0f8507010000                     jne 0x141611adf
00000001416119d8 4981fe20002001                   cmp r14, 0x1200020
00000001416119df 7264                             jb 0x141611a45
00000001416119e1 488d058eb84801                   lea rax, [rip + 0x148b88e]
00000001416119e8 488945c0                         mov qword ptr [rbp - 0x40], rax
00000001416119ec 48c745c84d000000                 mov qword ptr [rbp - 0x38], 0x4d
00000001416119f4 488d45c0                         lea rax, [rbp - 0x40]
00000001416119f8 48894568                         mov qword ptr [rbp + 0x68], rax
00000001416119fc 488d058d84affe                   lea rax, [rip - 0x1507b73]
0000000141611a03 48894570                         mov qword ptr [rbp + 0x70], rax
0000000141611a07 488d05badd6601                   lea rax, [rip + 0x166ddba]
0000000141611a0e 48894510                         mov qword ptr [rbp + 0x10], rax
0000000141611a12 48c7451801000000                 mov qword ptr [rbp + 0x18], 1
0000000141611a1a 48c7453000000000                 mov qword ptr [rbp + 0x30], 0
0000000141611a22 488d4568                         lea rax, [rbp + 0x68]
0000000141611a26 48894520                         mov qword ptr [rbp + 0x20], rax
0000000141611a2a 48c7452801000000                 mov qword ptr [rbp + 0x28], 1
0000000141611a32 488d159fdd6601                   lea rdx, [rip + 0x166dd9f]
0000000141611a39 488d4d10                         lea rcx, [rbp + 0x10]
0000000141611a3d e84e792c01                       call 0x1428d9390
0000000141611a42 4531f6                           xor r14d, r14d
0000000141611a45 4c8d7d10                         lea r15, [rbp + 0x10]
0000000141611a49 ba00002001                       mov edx, 0x1200000
0000000141611a4e 4c89f9                           mov rcx, r15
0000000141611a51 41b822000000                     mov r8d, 0x22
0000000141611a57 41b101                           mov r9b, 1
0000000141611a5a e881335cff                       call 0x140bd4de0
0000000141611a5f 4c8d6568                         lea r12, [rbp + 0x68]
0000000141611a63 41b804000000                     mov r8d, 4
0000000141611a69 4c89e1                           mov rcx, r12
0000000141611a6c 4c89fa                           mov rdx, r15
0000000141611a6f e8ac345cff                       call 0x140bd4f20
0000000141611a74 4c89742430                       mov qword ptr [rsp + 0x30], r14
0000000141611a79 48895c2428                       mov qword ptr [rsp + 0x28], rbx
0000000141611a7e 4c89642420                       mov qword ptr [rsp + 0x20], r12
0000000141611a83 4c8d05d8e66601                   lea r8, [rip + 0x166e6d8]
0000000141611a8a 488d4dc0                         lea rcx, [rbp - 0x40]
0000000141611a8e 41b90b000000                     mov r9d, 0xb
0000000141611a94 4889fa                           mov rdx, rdi
0000000141611a97 e824130900                       call 0x1416a2dc0
0000000141611a9c 837dc00b                         cmp dword ptr [rbp - 0x40], 0xb
0000000141611aa0 7567                             jne 0x141611b09
0000000141611aa2 488b45e8                         mov rax, qword ptr [rbp - 0x18]
0000000141611aa6 48894620                         mov qword ptr [rsi + 0x20], rax
0000000141611aaa 0f1045c8                         movups xmm0, xmmword ptr [rbp - 0x38]
0000000141611aae 0f104dd8                         movups xmm1, xmmword ptr [rbp - 0x28]
0000000141611ab2 0f114e10                         movups xmmword ptr [rsi + 0x10], xmm1
0000000141611ab6 0f1106                           movups xmmword ptr [rsi], xmm0
0000000141611ab9 4889f0                           mov rax, rsi
0000000141611abc 4881c400010000                   add rsp, 0x100
0000000141611ac3 5b                               pop rbx
0000000141611ac4 5f                               pop rdi
0000000141611ac5 5e                               pop rsi
0000000141611ac6 415c                             pop r12
0000000141611ac8 415e                             pop r14
0000000141611aca 415f                             pop r15
0000000141611acc 5d                               pop rbp
0000000141611acd c3                               ret 
0000000141611ace 48c7451000000000                 mov qword ptr [rbp + 0x10], 0
0000000141611ad6 488d056b846b01                   lea rax, [rip + 0x16b846b]
0000000141611add eb0f                             jmp 0x141611aee
0000000141611adf 48c7451000000000                 mov qword ptr [rbp + 0x10], 0
0000000141611ae7 488d0572846b01                   lea rax, [rip + 0x16b8472]
0000000141611aee 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000141611af3 4c8d0546846b01                   lea r8, [rip + 0x16b8446]
0000000141611afa 488d55c0                         lea rdx, [rbp - 0x40]
0000000141611afe 4c8d4d10                         lea r9, [rbp + 0x10]
0000000141611b02 31c9                             xor ecx, ecx
0000000141611b04 e88d7a2c01                       call 0x1428d9596
0000000141611b09 0f104500                         movups xmm0, xmmword ptr [rbp]
0000000141611b0d 0f294550                         movaps xmmword ptr [rbp + 0x50], xmm0
0000000141611b11 0f1045c0                         movups xmm0, xmmword ptr [rbp - 0x40]
0000000141611b15 0f104dd0                         movups xmm1, xmmword ptr [rbp - 0x30]
0000000141611b19 0f1055e0                         movups xmm2, xmmword ptr [rbp - 0x20]
0000000141611b1d 0f105df0                         movups xmm3, xmmword ptr [rbp - 0x10]
0000000141611b21 0f295d40                         movaps xmmword ptr [rbp + 0x40], xmm3
0000000141611b25 0f295530                         movaps xmmword ptr [rbp + 0x30], xmm2
0000000141611b29 0f294d20                         movaps xmmword ptr [rbp + 0x20], xmm1
0000000141611b2d 0f294510                         movaps xmmword ptr [rbp + 0x10], xmm0
0000000141611b31 488d0570e66601                   lea rax, [rip + 0x166e670]
0000000141611b38 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000141611b3d 488d0da4cf6601                   lea rcx, [rip + 0x166cfa4]
0000000141611b44 4c8d0d7dcf6601                   lea r9, [rip + 0x166cf7d]
0000000141611b4b 4c8d4510                         lea r8, [rbp + 0x10]
0000000141611b4f ba2b000000                       mov edx, 0x2b
0000000141611b54 e8077c2c01                       call 0x1428d9760
0000000141611b59 0f0b                             ud2 
0000000141611b5b 0f1f440000                       nop dword ptr [rax + rax]
0000000141611b60 4889542410                       mov qword ptr [rsp + 0x10], rdx
0000000141611b65 55                               push rbp
0000000141611b66 4157                             push r15
0000000141611b68 4156                             push r14
0000000141611b6a 4154                             push r12
0000000141611b6c 56                               push rsi
0000000141611b6d 57                               push rdi
0000000141611b6e 53                               push rbx
0000000141611b6f 4883ec40                         sub rsp, 0x40
0000000141611b73 488daa80000000                   lea rbp, [rdx + 0x80]
0000000141611b7a 488d4d10                         lea rcx, [rbp + 0x10]
0000000141611b7e e8edce1bff                       call 0x1407cea70
0000000141611b83 90                               nop 
0000000141611b84 4883c440                         add rsp, 0x40
0000000141611b88 5b                               pop rbx
0000000141611b89 5f                               pop rdi
0000000141611b8a 5e                               pop rsi
0000000141611b8b 415c                             pop r12
0000000141611b8d 415e                             pop r14
0000000141611b8f 415f                             pop r15
0000000141611b91 5d                               pop rbp
0000000141611b92 c3                               ret 
