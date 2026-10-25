0000000141614110 55                               push rbp
0000000141614111 4157                             push r15
0000000141614113 4156                             push r14
0000000141614115 4154                             push r12
0000000141614117 56                               push rsi
0000000141614118 57                               push rdi
0000000141614119 53                               push rbx
000000014161411a 4881ec20010000                   sub rsp, 0x120
0000000141614121 488dac2480000000                 lea rbp, [rsp + 0x80]
0000000141614129 48c78598000000feffffff           mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
0000000141614134 4889d7                           mov rdi, rdx
0000000141614137 4889ce                           mov rsi, rcx
000000014161413a 4d85c0                           test r8, r8
000000014161413d 743a                             je 0x141614179
000000014161413f 4c89cb                           mov rbx, r9
0000000141614142 4c89c8                           mov rax, r9
0000000141614145 4883e01f                         and rax, 0x1f
0000000141614149 488945c0                         mov qword ptr [rbp - 0x40], rax
000000014161414d 0f856b010000                     jne 0x1416142be
0000000141614153 4d89c7                           mov r15, r8
0000000141614156 4c89c0                           mov rax, r8
0000000141614159 4883e003                         and rax, 3
000000014161415d 488945c0                         mov qword ptr [rbp - 0x40], rax
0000000141614161 0f8568010000                     jne 0x1416142cf
0000000141614167 4881fb20680000                   cmp rbx, 0x6820
000000014161416e 0f83e5010000                     jae 0x141614359
0000000141614174 4531e4                           xor r12d, r12d
0000000141614177 eb7c                             jmp 0x1416141f5
0000000141614179 0fb605890a0202                   movzx eax, byte ptr [rip + 0x2020a89]
0000000141614180 b900680000                       mov ecx, 0x6800
0000000141614185 ba04000000                       mov edx, 4
000000014161418a e881fafffe                       call 0x140613c10
000000014161418f 4885c0                           test rax, rax
0000000141614192 0f842b020000                     je 0x1416143c3
0000000141614198 4989c7                           mov r15, rax
000000014161419b b834000000                       mov eax, 0x34
00000001416141a0 0f57c0                           xorps xmm0, xmm0
00000001416141a3 488b0d5e374901                   mov rcx, qword ptr [rip + 0x149375e]
00000001416141aa 8b1560374901                     mov edx, dword ptr [rip + 0x1493760]
00000001416141b0 410f114407cc                     movups xmmword ptr [r15 + rax - 0x34], xmm0
00000001416141b6 41c74407dc0000803f               mov dword ptr [r15 + rax - 0x24], 0x3f800000
00000001416141bf 49894c07e0                       mov qword ptr [r15 + rax - 0x20], rcx
00000001416141c4 41895407e8                       mov dword ptr [r15 + rax - 0x18], edx
00000001416141c9 410f114407ec                     movups xmmword ptr [r15 + rax - 0x14], xmm0
00000001416141cf 41c74407fc0000803f               mov dword ptr [r15 + rax - 4], 0x3f800000
00000001416141d8 49890c07                         mov qword ptr [r15 + rax], rcx
00000001416141dc 4189540708                       mov dword ptr [r15 + rax + 8], edx
00000001416141e1 4883c040                         add rax, 0x40
00000001416141e5 483d34680000                     cmp rax, 0x6834
00000001416141eb 75c3                             jne 0x1416141b0
00000001416141ed 41b401                           mov r12b, 1
00000001416141f0 bb00680000                       mov ebx, 0x6800
00000001416141f5 4c89bd88000000                   mov qword ptr [rbp + 0x88], r15
00000001416141fc 4488a597000000                   mov byte ptr [rbp + 0x97], r12b
0000000141614203 488d4d10                         lea rcx, [rbp + 0x10]
0000000141614207 ba00680000                       mov edx, 0x6800
000000014161420c 41b822000000                     mov r8d, 0x22
0000000141614212 41b101                           mov r9b, 1
0000000141614215 e8c60b5cff                       call 0x140bd4de0
000000014161421a 4c89bd88000000                   mov qword ptr [rbp + 0x88], r15
0000000141614221 4488a597000000                   mov byte ptr [rbp + 0x97], r12b
0000000141614228 4c8d7568                         lea r14, [rbp + 0x68]
000000014161422c 488d5510                         lea rdx, [rbp + 0x10]
0000000141614230 41b804000000                     mov r8d, 4
0000000141614236 4c89f1                           mov rcx, r14
0000000141614239 e8e20c5cff                       call 0x140bd4f20
000000014161423e 4c89bd88000000                   mov qword ptr [rbp + 0x88], r15
0000000141614245 4488a597000000                   mov byte ptr [rbp + 0x97], r12b
000000014161424c 48895c2430                       mov qword ptr [rsp + 0x30], rbx
0000000141614251 4c897c2428                       mov qword ptr [rsp + 0x28], r15
0000000141614256 4c89742420                       mov qword ptr [rsp + 0x20], r14
000000014161425b 4c8d05c6bc6601                   lea r8, [rip + 0x166bcc6]
0000000141614262 488d4dc0                         lea rcx, [rbp - 0x40]
0000000141614266 41b90d000000                     mov r9d, 0xd
000000014161426c 4889fa                           mov rdx, rdi
000000014161426f e84ceb0800                       call 0x1416a2dc0
0000000141614274 837dc00b                         cmp dword ptr [rbp - 0x40], 0xb
0000000141614278 757f                             jne 0x1416142f9
000000014161427a 488b45e8                         mov rax, qword ptr [rbp - 0x18]
000000014161427e 48894620                         mov qword ptr [rsi + 0x20], rax
0000000141614282 0f1045c8                         movups xmm0, xmmword ptr [rbp - 0x38]
0000000141614286 0f104dd8                         movups xmm1, xmmword ptr [rbp - 0x28]
000000014161428a 0f114e10                         movups xmmword ptr [rsi + 0x10], xmm1
000000014161428e 0f1106                           movups xmmword ptr [rsi], xmm0
0000000141614291 4584e4                           test r12b, r12b
0000000141614294 7413                             je 0x1416142a9
0000000141614296 ba00680000                       mov edx, 0x6800
000000014161429b 41b804000000                     mov r8d, 4
00000001416142a1 4c89f9                           mov rcx, r15
00000001416142a4 e877f9fffe                       call 0x140613c20
00000001416142a9 4889f0                           mov rax, rsi
00000001416142ac 4881c420010000                   add rsp, 0x120
00000001416142b3 5b                               pop rbx
00000001416142b4 5f                               pop rdi
00000001416142b5 5e                               pop rsi
00000001416142b6 415c                             pop r12
00000001416142b8 415e                             pop r14
00000001416142ba 415f                             pop r15
00000001416142bc 5d                               pop rbp
00000001416142bd c3                               ret 
00000001416142be 48c7451000000000                 mov qword ptr [rbp + 0x10], 0
00000001416142c6 488d057b5c6b01                   lea rax, [rip + 0x16b5c7b]
00000001416142cd eb0f                             jmp 0x1416142de
00000001416142cf 48c7451000000000                 mov qword ptr [rbp + 0x10], 0
00000001416142d7 488d05825c6b01                   lea rax, [rip + 0x16b5c82]
00000001416142de 4889442420                       mov qword ptr [rsp + 0x20], rax
00000001416142e3 4c8d05565c6b01                   lea r8, [rip + 0x16b5c56]
00000001416142ea 488d55c0                         lea rdx, [rbp - 0x40]
00000001416142ee 4c8d4d10                         lea r9, [rbp + 0x10]
00000001416142f2 31c9                             xor ecx, ecx
00000001416142f4 e89d522c01                       call 0x1428d9596
00000001416142f9 4489a584000000                   mov dword ptr [rbp + 0x84], r12d
0000000141614300 4c897d78                         mov qword ptr [rbp + 0x78], r15
0000000141614304 0f104500                         movups xmm0, xmmword ptr [rbp]
0000000141614308 0f294550                         movaps xmmword ptr [rbp + 0x50], xmm0
000000014161430c 0f1045c0                         movups xmm0, xmmword ptr [rbp - 0x40]
0000000141614310 0f104dd0                         movups xmm1, xmmword ptr [rbp - 0x30]
0000000141614314 0f1055e0                         movups xmm2, xmmword ptr [rbp - 0x20]
0000000141614318 0f105df0                         movups xmm3, xmmword ptr [rbp - 0x10]
000000014161431c 0f295d40                         movaps xmmword ptr [rbp + 0x40], xmm3
0000000141614320 0f295530                         movaps xmmword ptr [rbp + 0x30], xmm2
0000000141614324 0f294d20                         movaps xmmword ptr [rbp + 0x20], xmm1
0000000141614328 0f294510                         movaps xmmword ptr [rbp + 0x10], xmm0
000000014161432c 488d0575be6601                   lea rax, [rip + 0x166be75]
0000000141614333 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000141614338 488d0da9a76601                   lea rcx, [rip + 0x166a7a9]
000000014161433f 4c8d0d82a76601                   lea r9, [rip + 0x166a782]
0000000141614346 4c8d4510                         lea r8, [rbp + 0x10]
000000014161434a ba2b000000                       mov edx, 0x2b
000000014161434f e80c542c01                       call 0x1428d9760
0000000141614354 e987000000                       jmp 0x1416143e0
0000000141614359 488d053da54801                   lea rax, [rip + 0x148a53d]
0000000141614360 488945c0                         mov qword ptr [rbp - 0x40], rax
0000000141614364 48c745c854000000                 mov qword ptr [rbp - 0x38], 0x54
000000014161436c 488d45c0                         lea rax, [rbp - 0x40]
0000000141614370 48894568                         mov qword ptr [rbp + 0x68], rax
0000000141614374 488d05155baffe                   lea rax, [rip - 0x150a4eb]
000000014161437b 48894570                         mov qword ptr [rbp + 0x70], rax
000000014161437f 488d0542b46601                   lea rax, [rip + 0x166b442]
0000000141614386 48894510                         mov qword ptr [rbp + 0x10], rax
000000014161438a 48c7451801000000                 mov qword ptr [rbp + 0x18], 1
0000000141614392 48c7453000000000                 mov qword ptr [rbp + 0x30], 0
000000014161439a 488d4568                         lea rax, [rbp + 0x68]
000000014161439e 48894520                         mov qword ptr [rbp + 0x20], rax
00000001416143a2 48c7452801000000                 mov qword ptr [rbp + 0x28], 1
00000001416143aa c6859700000000                   mov byte ptr [rbp + 0x97], 0
00000001416143b1 488d1520b46601                   lea rdx, [rip + 0x166b420]
00000001416143b8 488d4d10                         lea rcx, [rbp + 0x10]
00000001416143bc e8cf4f2c01                       call 0x1428d9390
00000001416143c1 eb1d                             jmp 0x1416143e0
00000001416143c3 c6859700000000                   mov byte ptr [rbp + 0x97], 0
00000001416143ca 4c8d05dfa66601                   lea r8, [rip + 0x166a6df]
00000001416143d1 b904000000                       mov ecx, 4
00000001416143d6 ba00680000                       mov edx, 0x6800
00000001416143db e8034c2c01                       call 0x1428d8fe3
00000001416143e0 0f0b                             ud2 
00000001416143e2 66666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
00000001416143f0 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001416143f5 55                               push rbp
00000001416143f6 4157                             push r15
00000001416143f8 4156                             push r14
00000001416143fa 4154                             push r12
00000001416143fc 56                               push rsi
00000001416143fd 57                               push rdi
00000001416143fe 53                               push rbx
00000001416143ff 4883ec40                         sub rsp, 0x40
0000000141614403 488daa80000000                   lea rbp, [rdx + 0x80]
000000014161440a 488b7578                         mov rsi, qword ptr [rbp + 0x78]
000000014161440e 4889b588000000                   mov qword ptr [rbp + 0x88], rsi
0000000141614415 8bbd84000000                     mov edi, dword ptr [rbp + 0x84]
000000014161441b 4088bd97000000                   mov byte ptr [rbp + 0x97], dil
0000000141614422 488d4d10                         lea rcx, [rbp + 0x10]
0000000141614426 e845a61bff                       call 0x1407cea70
000000014161442b 4889b588000000                   mov qword ptr [rbp + 0x88], rsi
0000000141614432 4088bd97000000                   mov byte ptr [rbp + 0x97], dil
0000000141614439 4883c440                         add rsp, 0x40
000000014161443d 5b                               pop rbx
000000014161443e 5f                               pop rdi
000000014161443f 5e                               pop rsi
0000000141614440 415c                             pop r12
0000000141614442 415e                             pop r14
0000000141614444 415f                             pop r15
0000000141614446 5d                               pop rbp
0000000141614447 c3                               ret 
0000000141614448 0f1f840000000000                 nop dword ptr [rax + rax]
0000000141614450 4889542410                       mov qword ptr [rsp + 0x10], rdx
0000000141614455 55                               push rbp
0000000141614456 4157                             push r15
0000000141614458 4156                             push r14
000000014161445a 4154                             push r12
000000014161445c 56                               push rsi
000000014161445d 57                               push rdi
000000014161445e 53                               push rbx
000000014161445f 4883ec40                         sub rsp, 0x40
0000000141614463 488daa80000000                   lea rbp, [rdx + 0x80]
000000014161446a f6859700000001                   test byte ptr [rbp + 0x97], 1
0000000141614471 7417                             je 0x14161448a
0000000141614473 488b8d88000000                   mov rcx, qword ptr [rbp + 0x88]
000000014161447a ba00680000                       mov edx, 0x6800
000000014161447f 41b804000000                     mov r8d, 4
0000000141614485 e896f7fffe                       call 0x140613c20
000000014161448a 90                               nop 
000000014161448b 4883c440                         add rsp, 0x40
000000014161448f 5b                               pop rbx
0000000141614490 5f                               pop rdi
0000000141614491 5e                               pop rsi
0000000141614492 415c                             pop r12
0000000141614494 415e                             pop r14
0000000141614496 415f                             pop r15
0000000141614498 5d                               pop rbp
0000000141614499 c3                               ret 
