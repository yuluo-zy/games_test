0000000141610600 55                               push rbp
0000000141610601 4157                             push r15
0000000141610603 4156                             push r14
0000000141610605 4154                             push r12
0000000141610607 56                               push rsi
0000000141610608 57                               push rdi
0000000141610609 53                               push rbx
000000014161060a 4881ec00010000                   sub rsp, 0x100
0000000141610611 488dac2480000000                 lea rbp, [rsp + 0x80]
0000000141610619 48c74578feffffff                 mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
0000000141610621 4c89c3                           mov rbx, r8
0000000141610624 4889d7                           mov rdi, rdx
0000000141610627 4889ce                           mov rsi, rcx
000000014161062a 4d85c0                           test r8, r8
000000014161062d 0f848f000000                     je 0x1416106c2
0000000141610633 4d89ce                           mov r14, r9
0000000141610636 4c89c8                           mov rax, r9
0000000141610639 4883e00f                         and rax, 0xf
000000014161063d 488945c0                         mov qword ptr [rbp - 0x40], rax
0000000141610641 0f8507010000                     jne 0x14161074e
0000000141610647 4889d8                           mov rax, rbx
000000014161064a 4883e003                         and rax, 3
000000014161064e 488945c0                         mov qword ptr [rbp - 0x40], rax
0000000141610652 0f8507010000                     jne 0x14161075f
0000000141610658 4981fed0440000                   cmp r14, 0x44d0
000000014161065f 7264                             jb 0x1416106c5
0000000141610661 488d05c1db4801                   lea rax, [rip + 0x148dbc1]
0000000141610668 488945c0                         mov qword ptr [rbp - 0x40], rax
000000014161066c 48c745c857000000                 mov qword ptr [rbp - 0x38], 0x57
0000000141610674 488d45c0                         lea rax, [rbp - 0x40]
0000000141610678 48894568                         mov qword ptr [rbp + 0x68], rax
000000014161067c 488d050d98affe                   lea rax, [rip - 0x15067f3]
0000000141610683 48894570                         mov qword ptr [rbp + 0x70], rax
0000000141610687 488d053af16601                   lea rax, [rip + 0x166f13a]
000000014161068e 48894510                         mov qword ptr [rbp + 0x10], rax
0000000141610692 48c7451801000000                 mov qword ptr [rbp + 0x18], 1
000000014161069a 48c7453000000000                 mov qword ptr [rbp + 0x30], 0
00000001416106a2 488d4568                         lea rax, [rbp + 0x68]
00000001416106a6 48894520                         mov qword ptr [rbp + 0x20], rax
00000001416106aa 48c7452801000000                 mov qword ptr [rbp + 0x28], 1
00000001416106b2 488d151ff16601                   lea rdx, [rip + 0x166f11f]
00000001416106b9 488d4d10                         lea rcx, [rbp + 0x10]
00000001416106bd e8ce8c2c01                       call 0x1428d9390
00000001416106c2 4531f6                           xor r14d, r14d
00000001416106c5 4c8d7d10                         lea r15, [rbp + 0x10]
00000001416106c9 bac0440000                       mov edx, 0x44c0
00000001416106ce 4c89f9                           mov rcx, r15
00000001416106d1 41b822000000                     mov r8d, 0x22
00000001416106d7 41b101                           mov r9b, 1
00000001416106da e801475cff                       call 0x140bd4de0
00000001416106df 4c8d6568                         lea r12, [rbp + 0x68]
00000001416106e3 41b804000000                     mov r8d, 4
00000001416106e9 4c89e1                           mov rcx, r12
00000001416106ec 4c89fa                           mov rdx, r15
00000001416106ef e82c485cff                       call 0x140bd4f20
00000001416106f4 4c89742430                       mov qword ptr [rsp + 0x30], r14
00000001416106f9 48895c2428                       mov qword ptr [rsp + 0x28], rbx
00000001416106fe 4c89642420                       mov qword ptr [rsp + 0x20], r12
0000000141610703 4c8d05b6ef6601                   lea r8, [rip + 0x166efb6]
000000014161070a 488d4dc0                         lea rcx, [rbp - 0x40]
000000014161070e 41b90f000000                     mov r9d, 0xf
0000000141610714 4889fa                           mov rdx, rdi
0000000141610717 e8a4260900                       call 0x1416a2dc0
000000014161071c 837dc00b                         cmp dword ptr [rbp - 0x40], 0xb
0000000141610720 7567                             jne 0x141610789
0000000141610722 488b45e8                         mov rax, qword ptr [rbp - 0x18]
0000000141610726 48894620                         mov qword ptr [rsi + 0x20], rax
000000014161072a 0f1045c8                         movups xmm0, xmmword ptr [rbp - 0x38]
000000014161072e 0f104dd8                         movups xmm1, xmmword ptr [rbp - 0x28]
0000000141610732 0f114e10                         movups xmmword ptr [rsi + 0x10], xmm1
0000000141610736 0f1106                           movups xmmword ptr [rsi], xmm0
0000000141610739 4889f0                           mov rax, rsi
000000014161073c 4881c400010000                   add rsp, 0x100
0000000141610743 5b                               pop rbx
0000000141610744 5f                               pop rdi
0000000141610745 5e                               pop rsi
0000000141610746 415c                             pop r12
0000000141610748 415e                             pop r14
000000014161074a 415f                             pop r15
000000014161074c 5d                               pop rbp
000000014161074d c3                               ret 
000000014161074e 48c7451000000000                 mov qword ptr [rbp + 0x10], 0
0000000141610756 488d05eb976b01                   lea rax, [rip + 0x16b97eb]
000000014161075d eb0f                             jmp 0x14161076e
000000014161075f 48c7451000000000                 mov qword ptr [rbp + 0x10], 0
0000000141610767 488d05f2976b01                   lea rax, [rip + 0x16b97f2]
000000014161076e 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000141610773 4c8d05c6976b01                   lea r8, [rip + 0x16b97c6]
000000014161077a 488d55c0                         lea rdx, [rbp - 0x40]
000000014161077e 4c8d4d10                         lea r9, [rbp + 0x10]
0000000141610782 31c9                             xor ecx, ecx
0000000141610784 e80d8e2c01                       call 0x1428d9596
0000000141610789 0f104500                         movups xmm0, xmmword ptr [rbp]
000000014161078d 0f294550                         movaps xmmword ptr [rbp + 0x50], xmm0
0000000141610791 0f1045c0                         movups xmm0, xmmword ptr [rbp - 0x40]
0000000141610795 0f104dd0                         movups xmm1, xmmword ptr [rbp - 0x30]
0000000141610799 0f1055e0                         movups xmm2, xmmword ptr [rbp - 0x20]
000000014161079d 0f105df0                         movups xmm3, xmmword ptr [rbp - 0x10]
00000001416107a1 0f295d40                         movaps xmmword ptr [rbp + 0x40], xmm3
00000001416107a5 0f295530                         movaps xmmword ptr [rbp + 0x30], xmm2
00000001416107a9 0f294d20                         movaps xmmword ptr [rbp + 0x20], xmm1
00000001416107ad 0f294510                         movaps xmmword ptr [rbp + 0x10], xmm0
00000001416107b1 488d05f0f96601                   lea rax, [rip + 0x166f9f0]
00000001416107b8 4889442420                       mov qword ptr [rsp + 0x20], rax
00000001416107bd 488d0d24e36601                   lea rcx, [rip + 0x166e324]
00000001416107c4 4c8d0dfde26601                   lea r9, [rip + 0x166e2fd]
00000001416107cb 4c8d4510                         lea r8, [rbp + 0x10]
00000001416107cf ba2b000000                       mov edx, 0x2b
00000001416107d4 e8878f2c01                       call 0x1428d9760
00000001416107d9 0f0b                             ud2 
00000001416107db 0f1f440000                       nop dword ptr [rax + rax]
00000001416107e0 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001416107e5 55                               push rbp
00000001416107e6 4157                             push r15
00000001416107e8 4156                             push r14
00000001416107ea 4154                             push r12
00000001416107ec 56                               push rsi
00000001416107ed 57                               push rdi
00000001416107ee 53                               push rbx
00000001416107ef 4883ec40                         sub rsp, 0x40
00000001416107f3 488daa80000000                   lea rbp, [rdx + 0x80]
00000001416107fa 488d4d10                         lea rcx, [rbp + 0x10]
00000001416107fe e86de21bff                       call 0x1407cea70
0000000141610803 90                               nop 
0000000141610804 4883c440                         add rsp, 0x40
0000000141610808 5b                               pop rbx
0000000141610809 5f                               pop rdi
000000014161080a 5e                               pop rsi
000000014161080b 415c                             pop r12
000000014161080d 415e                             pop r14
000000014161080f 415f                             pop r15
0000000141610811 5d                               pop rbp
0000000141610812 c3                               ret 
