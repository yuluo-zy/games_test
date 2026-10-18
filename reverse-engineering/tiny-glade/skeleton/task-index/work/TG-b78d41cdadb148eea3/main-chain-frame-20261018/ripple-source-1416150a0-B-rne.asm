00000001416150a0 55                               push rbp
00000001416150a1 4157                             push r15
00000001416150a3 4156                             push r14
00000001416150a5 4154                             push r12
00000001416150a7 56                               push rsi
00000001416150a8 57                               push rdi
00000001416150a9 53                               push rbx
00000001416150aa 4881ec00010000                   sub rsp, 0x100
00000001416150b1 488dac2480000000                 lea rbp, [rsp + 0x80]
00000001416150b9 48c74578feffffff                 mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
00000001416150c1 4c89c3                           mov rbx, r8
00000001416150c4 4889d7                           mov rdi, rdx
00000001416150c7 4889ce                           mov rsi, rcx
00000001416150ca 4d85c0                           test r8, r8
00000001416150cd 745b                             je 0x14161512a
00000001416150cf 48b9e1830f3ef8e0830f             movabs rcx, 0xf83e0f83e0f83e1
00000001416150d9 4c89c8                           mov rax, r9
00000001416150dc 48f7e1                           mul rcx
00000001416150df 48c1ea05                         shr rdx, 5
00000001416150e3 4869c210020000                   imul rax, rdx, 0x210
00000001416150ea 4c89c9                           mov rcx, r9
00000001416150ed 4829c1                           sub rcx, rax
00000001416150f0 48894dc0                         mov qword ptr [rbp - 0x40], rcx
00000001416150f4 0f85c0000000                     jne 0x1416151ba
00000001416150fa 4889d8                           mov rax, rbx
00000001416150fd 4883e003                         and rax, 3
0000000141615101 488945c0                         mov qword ptr [rbp - 0x40], rax
0000000141615105 0f85c0000000                     jne 0x1416151cb
000000014161510b 4531e4                           xor r12d, r12d
000000014161510e 4981f910020000                   cmp r9, 0x210
0000000141615115 0f93c0                           setae al
0000000141615118 4981f920040000                   cmp r9, 0x420
000000014161511f 0f8322010000                     jae 0x141615247
0000000141615125 4188c4                           mov r12b, al
0000000141615128 eb00                             jmp 0x14161512a
000000014161512a 4c8d7510                         lea r14, [rbp + 0x10]
000000014161512e ba10020000                       mov edx, 0x210
0000000141615133 4c89f1                           mov rcx, r14
0000000141615136 41b822000000                     mov r8d, 0x22
000000014161513c 41b101                           mov r9b, 1
000000014161513f e89cfc5bff                       call 0x140bd4de0
0000000141615144 4c8d7d68                         lea r15, [rbp + 0x68]
0000000141615148 41b804000000                     mov r8d, 4
000000014161514e 4c89f9                           mov rcx, r15
0000000141615151 4c89f2                           mov rdx, r14
0000000141615154 e8c7fd5bff                       call 0x140bd4f20
0000000141615159 4969c410020000                   imul rax, r12, 0x210
0000000141615160 4889442430                       mov qword ptr [rsp + 0x30], rax
0000000141615165 48895c2428                       mov qword ptr [rsp + 0x28], rbx
000000014161516a 4c897c2420                       mov qword ptr [rsp + 0x20], r15
000000014161516f 4c8d0549a76601                   lea r8, [rip + 0x166a749]
0000000141615176 488d4dc0                         lea rcx, [rbp - 0x40]
000000014161517a 41b910000000                     mov r9d, 0x10
0000000141615180 4889fa                           mov rdx, rdi
0000000141615183 e838dc0800                       call 0x1416a2dc0
0000000141615188 837dc00b                         cmp dword ptr [rbp - 0x40], 0xb
000000014161518c 7567                             jne 0x1416151f5
000000014161518e 488b45e8                         mov rax, qword ptr [rbp - 0x18]
0000000141615192 48894620                         mov qword ptr [rsi + 0x20], rax
0000000141615196 0f1045c8                         movups xmm0, xmmword ptr [rbp - 0x38]
000000014161519a 0f104dd8                         movups xmm1, xmmword ptr [rbp - 0x28]
000000014161519e 0f114e10                         movups xmmword ptr [rsi + 0x10], xmm1
00000001416151a2 0f1106                           movups xmmword ptr [rsi], xmm0
00000001416151a5 4889f0                           mov rax, rsi
00000001416151a8 4881c400010000                   add rsp, 0x100
00000001416151af 5b                               pop rbx
00000001416151b0 5f                               pop rdi
00000001416151b1 5e                               pop rsi
00000001416151b2 415c                             pop r12
00000001416151b4 415e                             pop r14
00000001416151b6 415f                             pop r15
00000001416151b8 5d                               pop rbp
00000001416151b9 c3                               ret 
00000001416151ba 48c7451000000000                 mov qword ptr [rbp + 0x10], 0
00000001416151c2 488d057f4d6b01                   lea rax, [rip + 0x16b4d7f]
00000001416151c9 eb0f                             jmp 0x1416151da
00000001416151cb 48c7451000000000                 mov qword ptr [rbp + 0x10], 0
00000001416151d3 488d05864d6b01                   lea rax, [rip + 0x16b4d86]
00000001416151da 4889442420                       mov qword ptr [rsp + 0x20], rax
00000001416151df 4c8d055a4d6b01                   lea r8, [rip + 0x16b4d5a]
00000001416151e6 488d55c0                         lea rdx, [rbp - 0x40]
00000001416151ea 4c8d4d10                         lea r9, [rbp + 0x10]
00000001416151ee 31c9                             xor ecx, ecx
00000001416151f0 e8a1432c01                       call 0x1428d9596
00000001416151f5 0f104500                         movups xmm0, xmmword ptr [rbp]
00000001416151f9 0f294550                         movaps xmmword ptr [rbp + 0x50], xmm0
00000001416151fd 0f1045c0                         movups xmm0, xmmword ptr [rbp - 0x40]
0000000141615201 0f104dd0                         movups xmm1, xmmword ptr [rbp - 0x30]
0000000141615205 0f1055e0                         movups xmm2, xmmword ptr [rbp - 0x20]
0000000141615209 0f105df0                         movups xmm3, xmmword ptr [rbp - 0x10]
000000014161520d 0f295d40                         movaps xmmword ptr [rbp + 0x40], xmm3
0000000141615211 0f295530                         movaps xmmword ptr [rbp + 0x30], xmm2
0000000141615215 0f294d20                         movaps xmmword ptr [rbp + 0x20], xmm1
0000000141615219 0f294510                         movaps xmmword ptr [rbp + 0x10], xmm0
000000014161521d 488d0584af6601                   lea rax, [rip + 0x166af84]
0000000141615224 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000141615229 488d0db8986601                   lea rcx, [rip + 0x16698b8]
0000000141615230 4c8d0d91986601                   lea r9, [rip + 0x1669891]
0000000141615237 4c8d4510                         lea r8, [rbp + 0x10]
000000014161523b ba2b000000                       mov edx, 0x2b
0000000141615240 e81b452c01                       call 0x1428d9760
0000000141615245 0f0b                             ud2 
0000000141615247 488d055e984801                   lea rax, [rip + 0x148985e]
000000014161524e 488945c0                         mov qword ptr [rbp - 0x40], rax
0000000141615252 48c745c830000000                 mov qword ptr [rbp - 0x38], 0x30
000000014161525a 488d45c0                         lea rax, [rbp - 0x40]
000000014161525e 48894568                         mov qword ptr [rbp + 0x68], rax
0000000141615262 488d05274caffe                   lea rax, [rip - 0x150b3d9]
0000000141615269 48894570                         mov qword ptr [rbp + 0x70], rax
000000014161526d 488d0554a56601                   lea rax, [rip + 0x166a554]
0000000141615274 48894510                         mov qword ptr [rbp + 0x10], rax
0000000141615278 48c7451801000000                 mov qword ptr [rbp + 0x18], 1
0000000141615280 48c7453000000000                 mov qword ptr [rbp + 0x30], 0
0000000141615288 488d4568                         lea rax, [rbp + 0x68]
000000014161528c 48894520                         mov qword ptr [rbp + 0x20], rax
0000000141615290 48c7452801000000                 mov qword ptr [rbp + 0x28], 1
0000000141615298 488d1539a56601                   lea rdx, [rip + 0x166a539]
000000014161529f 488d4d10                         lea rcx, [rbp + 0x10]
00000001416152a3 e8e8402c01                       call 0x1428d9390
00000001416152a8 cc                               int3 
00000001416152a9 0f1f8000000000                   nop dword ptr [rax]
00000001416152b0 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001416152b5 55                               push rbp
00000001416152b6 4157                             push r15
00000001416152b8 4156                             push r14
00000001416152ba 4154                             push r12
00000001416152bc 56                               push rsi
00000001416152bd 57                               push rdi
00000001416152be 53                               push rbx
00000001416152bf 4883ec40                         sub rsp, 0x40
00000001416152c3 488daa80000000                   lea rbp, [rdx + 0x80]
00000001416152ca 488d4d10                         lea rcx, [rbp + 0x10]
00000001416152ce e89d971bff                       call 0x1407cea70
00000001416152d3 90                               nop 
00000001416152d4 4883c440                         add rsp, 0x40
00000001416152d8 5b                               pop rbx
00000001416152d9 5f                               pop rdi
00000001416152da 5e                               pop rsi
00000001416152db 415c                             pop r12
00000001416152dd 415e                             pop r14
00000001416152df 415f                             pop r15
00000001416152e1 5d                               pop rbp
00000001416152e2 c3                               ret 
