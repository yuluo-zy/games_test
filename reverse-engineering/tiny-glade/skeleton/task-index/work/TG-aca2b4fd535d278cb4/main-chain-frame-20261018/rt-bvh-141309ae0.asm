0000000141309ae0  55                       push       rbp
0000000141309ae1  4157                     push       r15
0000000141309ae3  4156                     push       r14
0000000141309ae5  4155                     push       r13
0000000141309ae7  4154                     push       r12
0000000141309ae9  56                       push       rsi
0000000141309aea  57                       push       rdi
0000000141309aeb  53                       push       rbx
0000000141309aec  4883ec78                 sub        rsp, 0x78
0000000141309af0  488d6c2470               lea        rbp, [rsp + 0x70]
0000000141309af5  48c74500feffffff         mov        qword ptr [rbp], 0xfffffffffffffffe
0000000141309afd  4c8945e0                 mov        qword ptr [rbp - 0x20], r8
0000000141309b01  4889d7                   mov        rdi, rdx
0000000141309b04  4889ce                   mov        rsi, rcx
0000000141309b07  4c8b2a                   mov        r13, qword ptr [rdx]
0000000141309b0a  4c8b7208                 mov        r14, qword ptr [rdx + 8]
0000000141309b0e  4c89f3                   mov        rbx, r14
0000000141309b11  4c29eb                   sub        rbx, r13
0000000141309b14  48c1eb02                 shr        rbx, 2
0000000141309b18  4531e4                   xor        r12d, r12d
0000000141309b1b  b918000000               mov        ecx, 0x18
0000000141309b20  4889d8                   mov        rax, rbx
0000000141309b23  48f7e1                   mul        rcx
0000000141309b26  4989c7                   mov        r15, rax
0000000141309b29  7033                     jo         0x141309b5e
0000000141309b2b  48b8fcffffffffffff7f     movabs     rax, 0x7ffffffffffffffc
0000000141309b35  4939c7                   cmp        r15, rax
0000000141309b38  7724                     ja         0x141309b5e
0000000141309b3a  4d85ff                   test       r15, r15
0000000141309b3d  742e                     je         0x141309b6d
0000000141309b3f  0fb605c3b03202           movzx      eax, byte ptr [rip + 0x232b0c3]
0000000141309b46  41bc04000000             mov        r12d, 4
0000000141309b4c  ba04000000               mov        edx, 4
0000000141309b51  4c89f9                   mov        rcx, r15
0000000141309b54  e8b7a030ff               call       0x140613c10
0000000141309b59  4885c0                   test       rax, rax
0000000141309b5c  7516                     jne        0x141309b74
0000000141309b5e  4c89e1                   mov        rcx, r12
0000000141309b61  4c89fa                   mov        rdx, r15
0000000141309b64  4c8b45e0                 mov        r8, qword ptr [rbp - 0x20]
0000000141309b68  e876f45c01               call       0x1428d8fe3
0000000141309b6d  b804000000               mov        eax, 4
0000000141309b72  31db                     xor        ebx, ebx
0000000141309b74  48895de8                 mov        qword ptr [rbp - 0x18], rbx
0000000141309b78  488945f0                 mov        qword ptr [rbp - 0x10], rax
0000000141309b7c  488d4df8                 lea        rcx, [rbp - 8]
0000000141309b80  48c745f800000000         mov        qword ptr [rbp - 8], 0
0000000141309b88  488b5710                 mov        rdx, qword ptr [rdi + 0x10]
0000000141309b8c  4c896dc8                 mov        qword ptr [rbp - 0x38], r13
0000000141309b90  4c8975d0                 mov        qword ptr [rbp - 0x30], r14
0000000141309b94  488955d8                 mov        qword ptr [rbp - 0x28], rdx
0000000141309b98  48894db0                 mov        qword ptr [rbp - 0x50], rcx
0000000141309b9c  48c745b800000000         mov        qword ptr [rbp - 0x48], 0
0000000141309ba4  488945c0                 mov        qword ptr [rbp - 0x40], rax
0000000141309ba8  488d4dc8                 lea        rcx, [rbp - 0x38]
0000000141309bac  488d55b0                 lea        rdx, [rbp - 0x50]
0000000141309bb0  e81bcc1600               call       0x1414767d0
0000000141309bb5  488b45f8                 mov        rax, qword ptr [rbp - 8]
0000000141309bb9  48894610                 mov        qword ptr [rsi + 0x10], rax
0000000141309bbd  0f1045e8                 movups     xmm0, xmmword ptr [rbp - 0x18]
0000000141309bc1  0f1106                   movups     xmmword ptr [rsi], xmm0
0000000141309bc4  4889f0                   mov        rax, rsi
0000000141309bc7  4883c478                 add        rsp, 0x78
0000000141309bcb  5b                       pop        rbx
0000000141309bcc  5f                       pop        rdi
0000000141309bcd  5e                       pop        rsi
0000000141309bce  415c                     pop        r12
0000000141309bd0  415d                     pop        r13
0000000141309bd2  415e                     pop        r14
0000000141309bd4  415f                     pop        r15
0000000141309bd6  5d                       pop        rbp
0000000141309bd7  c3                       ret        
0000000141309bd8  0f1f840000000000         nop        dword ptr [rax + rax]
0000000141309be0  4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000141309be5  55                       push       rbp
0000000141309be6  4157                     push       r15
0000000141309be8  4156                     push       r14
0000000141309bea  4155                     push       r13
0000000141309bec  4154                     push       r12
0000000141309bee  56                       push       rsi
0000000141309bef  57                       push       rdi
0000000141309bf0  53                       push       rbx
0000000141309bf1  4883ec28                 sub        rsp, 0x28
0000000141309bf5  488d6a70                 lea        rbp, [rdx + 0x70]
0000000141309bf9  488b45e8                 mov        rax, qword ptr [rbp - 0x18]
0000000141309bfd  4885c0                   test       rax, rax
0000000141309c00  7417                     je         0x141309c19
0000000141309c02  488b4df0                 mov        rcx, qword ptr [rbp - 0x10]
0000000141309c06  48c1e003                 shl        rax, 3
0000000141309c0a  488d1440                 lea        rdx, [rax + rax*2]
0000000141309c0e  41b804000000             mov        r8d, 4
0000000141309c14  e807a030ff               call       0x140613c20
0000000141309c19  90                       nop        
0000000141309c1a  4883c428                 add        rsp, 0x28
0000000141309c1e  5b                       pop        rbx
0000000141309c1f  5f                       pop        rdi
0000000141309c20  5e                       pop        rsi
0000000141309c21  415c                     pop        r12
0000000141309c23  415d                     pop        r13
0000000141309c25  415e                     pop        r14
0000000141309c27  415f                     pop        r15
0000000141309c29  5d                       pop        rbp
0000000141309c2a  c3                       ret        
