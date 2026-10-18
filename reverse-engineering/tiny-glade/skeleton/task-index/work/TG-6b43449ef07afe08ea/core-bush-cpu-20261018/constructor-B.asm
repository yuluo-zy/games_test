14161bd30: 55                             push rbp
14161bd31: 4157                           push r15
14161bd33: 4156                           push r14
14161bd35: 4154                           push r12
14161bd37: 56                             push rsi
14161bd38: 57                             push rdi
14161bd39: 53                             push rbx
14161bd3a: 4881ec10010000                 sub rsp, 0x110
14161bd41: 488dac2480000000               lea rbp, [rsp + 0x80]
14161bd49: 48c78588000000feffffff         mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
14161bd54: 4c89c3                         mov rbx, r8
14161bd57: 4889d7                         mov rdi, rdx
14161bd5a: 4889ce                         mov rsi, rcx
14161bd5d: 4d85c0                         test r8, r8
14161bd60: 0f848f000000                   je 0x14161bdf5
14161bd66: 4d89ce                         mov r14, r9
14161bd69: 4c89c8                         mov rax, r9
14161bd6c: 4883e007                       and rax, 7
14161bd70: 488945c0                       mov qword ptr [rbp - 0x40], rax
14161bd74: 0f85e9010000                   jne 0x14161bf63
14161bd7a: 4889d8                         mov rax, rbx
14161bd7d: 4883e003                       and rax, 3
14161bd81: 488945c0                       mov qword ptr [rbp - 0x40], rax
14161bd85: 0f85e9010000                   jne 0x14161bf74
14161bd8b: 4981fe102c0100                 cmp r14, 0x12c10
14161bd92: 7264                           jb 0x14161bdf8
14161bd94: 488d05b52c4801                 lea rax, [rip + 0x1482cb5]
14161bd9b: 488945c0                       mov qword ptr [rbp - 0x40], rax
14161bd9f: 48c745c85c000000               mov qword ptr [rbp - 0x38], 0x5c
14161bda7: 488d45c0                       lea rax, [rbp - 0x40]
14161bdab: 48894560                       mov qword ptr [rbp + 0x60], rax
14161bdaf: 488d05dae0aefe                 lea rax, [rip - 0x1511f26]
14161bdb6: 48894568                       mov qword ptr [rbp + 0x68], rax
14161bdba: 488d05073a6601                 lea rax, [rip + 0x1663a07]
14161bdc1: 48894510                       mov qword ptr [rbp + 0x10], rax
14161bdc5: 48c7451801000000               mov qword ptr [rbp + 0x18], 1
14161bdcd: 48c7453000000000               mov qword ptr [rbp + 0x30], 0
14161bdd5: 488d4560                       lea rax, [rbp + 0x60]
14161bdd9: 48894520                       mov qword ptr [rbp + 0x20], rax
14161bddd: 48c7452801000000               mov qword ptr [rbp + 0x28], 1
14161bde5: 488d15ec396601                 lea rdx, [rip + 0x16639ec]
14161bdec: 488d4d10                       lea rcx, [rbp + 0x10]
14161bdf0: e89bd52b01                     call 0x1428d9390
14161bdf5: 4531f6                         xor r14d, r14d
14161bdf8: 4c8d7d10                       lea r15, [rbp + 0x10]
14161bdfc: ba082c0100                     mov edx, 0x12c08
14161be01: 4c89f9                         mov rcx, r15
14161be04: 41b822000000                   mov r8d, 0x22
14161be0a: 41b104                         mov r9b, 4
14161be0d: e8ce8f5bff                     call 0x140bd4de0
14161be12: 4c8d6560                       lea r12, [rbp + 0x60]
14161be16: 41b804000000                   mov r8d, 4
14161be1c: 4c89e1                         mov rcx, r12
14161be1f: 4c89fa                         mov rdx, r15
14161be22: e8f9905bff                     call 0x140bd4f20
14161be27: 4c89742430                     mov qword ptr [rsp + 0x30], r14
14161be2c: 48895c2428                     mov qword ptr [rsp + 0x28], rbx
14161be31: 4c89642420                     mov qword ptr [rsp + 0x20], r12
14161be36: 4c8d05e3426601                 lea r8, [rip + 0x16642e3]
14161be3d: 488d4dc0                       lea rcx, [rbp - 0x40]
14161be41: 41b916000000                   mov r9d, 0x16
14161be47: 4889fa                         mov rdx, rdi
14161be4a: e8716f0800                     call 0x1416a2dc0
14161be4f: 837dc00b                       cmp dword ptr [rbp - 0x40], 0xb
14161be53: 0f8545010000                   jne 0x14161bf9e
14161be59: 488b45e8                       mov rax, qword ptr [rbp - 0x18]
14161be5d: 48898580000000                 mov qword ptr [rbp + 0x80], rax
14161be64: 0f1045c8                       movups xmm0, xmmword ptr [rbp - 0x38]
14161be68: 0f104dd8                       movups xmm1, xmmword ptr [rbp - 0x28]
14161be6c: 0f294d70                       movaps xmmword ptr [rbp + 0x70], xmm1
14161be70: 0f294560                       movaps xmmword ptr [rbp + 0x60], xmm0
14161be74: 488d4d60                       lea rcx, [rbp + 0x60]
14161be78: 4889fa                         mov rdx, rdi
14161be7b: e8508e5bff                     call 0x140bd4cd0
14161be80: 4885c0                         test rax, rax
14161be83: 0f8467010000                   je 0x14161bff0
14161be89: 483b5578                       cmp rdx, qword ptr [rbp + 0x78]
14161be8d: 7259                           jb 0x14161bee8
14161be8f: 4889d1                         mov rcx, rdx
14161be92: 4883e107                       and rcx, 7
14161be96: 48894dc0                       mov qword ptr [rbp - 0x40], rcx
14161be9a: 7569                           jne 0x14161bf05
14161be9c: 4889c1                         mov rcx, rax
14161be9f: 4883e103                       and rcx, 3
14161bea3: 48894dc0                       mov qword ptr [rbp - 0x40], rcx
14161bea7: 0f8587000000                   jne 0x14161bf34
14161bead: 488b8d80000000                 mov rcx, qword ptr [rbp + 0x80]
14161beb4: 48894e30                       mov qword ptr [rsi + 0x30], rcx
14161beb8: 0f284560                       movaps xmm0, xmmword ptr [rbp + 0x60]
14161bebc: 0f284d70                       movaps xmm1, xmmword ptr [rbp + 0x70]
14161bec0: 0f114e20                       movups xmmword ptr [rsi + 0x20], xmm1
14161bec4: 0f114610                       movups xmmword ptr [rsi + 0x10], xmm0
14161bec8: 48c1ea03                       shr rdx, 3
14161becc: 488906                         mov qword ptr [rsi], rax
14161becf: 48895608                       mov qword ptr [rsi + 8], rdx
14161bed3: 4889f0                         mov rax, rsi
14161bed6: 4881c410010000                 add rsp, 0x110
14161bedd: 5b                             pop rbx
14161bede: 5f                             pop rdi
14161bedf: 5e                             pop rsi
14161bee0: 415c                           pop r12
14161bee2: 415e                           pop r14
14161bee4: 415f                           pop r15
14161bee6: 5d                             pop rbp
14161bee7: c3                             ret 
14161bee8: 488d0d01436601                 lea rcx, [rip + 0x1664301]
14161beef: 4c8d0532436601                 lea r8, [rip + 0x1664332]
14161bef6: ba37000000                     mov edx, 0x37
14161befb: e830d52b01                     call 0x1428d9430
14161bf00: e9f7000000                     jmp 0x14161bffc
14161bf05: 48c7451000000000               mov qword ptr [rbp + 0x10], 0
14161bf0d: 488d0534436601                 lea rax, [rip + 0x1664334]
14161bf14: 4889442420                     mov qword ptr [rsp + 0x20], rax
14161bf19: 4c8d0520436601                 lea r8, [rip + 0x1664320]
14161bf20: 488d55c0                       lea rdx, [rbp - 0x40]
14161bf24: 4c8d4d10                       lea r9, [rbp + 0x10]
14161bf28: 31c9                           xor ecx, ecx
14161bf2a: e867d62b01                     call 0x1428d9596
14161bf2f: e9c8000000                     jmp 0x14161bffc
14161bf34: 48c7451000000000               mov qword ptr [rbp + 0x10], 0
14161bf3c: 488d051d436601                 lea rax, [rip + 0x166431d]
14161bf43: 4889442420                     mov qword ptr [rsp + 0x20], rax
14161bf48: 4c8d05f1426601                 lea r8, [rip + 0x16642f1]
14161bf4f: 488d55c0                       lea rdx, [rbp - 0x40]
14161bf53: 4c8d4d10                       lea r9, [rbp + 0x10]
14161bf57: 31c9                           xor ecx, ecx
14161bf59: e838d62b01                     call 0x1428d9596
14161bf5e: e999000000                     jmp 0x14161bffc
14161bf63: 48c7451000000000               mov qword ptr [rbp + 0x10], 0
14161bf6b: 488d05d6df6a01                 lea rax, [rip + 0x16adfd6]
14161bf72: eb0f                           jmp 0x14161bf83
14161bf74: 48c7451000000000               mov qword ptr [rbp + 0x10], 0
14161bf7c: 488d05dddf6a01                 lea rax, [rip + 0x16adfdd]
14161bf83: 4889442420                     mov qword ptr [rsp + 0x20], rax
14161bf88: 4c8d05b1df6a01                 lea r8, [rip + 0x16adfb1]
14161bf8f: 488d55c0                       lea rdx, [rbp - 0x40]
14161bf93: 4c8d4d10                       lea r9, [rbp + 0x10]
14161bf97: 31c9                           xor ecx, ecx
14161bf99: e8f8d52b01                     call 0x1428d9596
14161bf9e: 0f104500                       movups xmm0, xmmword ptr [rbp]
14161bfa2: 0f294550                       movaps xmmword ptr [rbp + 0x50], xmm0
14161bfa6: 0f1045c0                       movups xmm0, xmmword ptr [rbp - 0x40]
14161bfaa: 0f104dd0                       movups xmm1, xmmword ptr [rbp - 0x30]
14161bfae: 0f1055e0                       movups xmm2, xmmword ptr [rbp - 0x20]
14161bfb2: 0f105df0                       movups xmm3, xmmword ptr [rbp - 0x10]
14161bfb6: 0f295d40                       movaps xmmword ptr [rbp + 0x40], xmm3
14161bfba: 0f295530                       movaps xmmword ptr [rbp + 0x30], xmm2
14161bfbe: 0f294d20                       movaps xmmword ptr [rbp + 0x20], xmm1
14161bfc2: 0f294510                       movaps xmmword ptr [rbp + 0x10], xmm0
14161bfc6: 488d05db416601                 lea rax, [rip + 0x16641db]
14161bfcd: 4889442420                     mov qword ptr [rsp + 0x20], rax
14161bfd2: 488d0d0f2b6601                 lea rcx, [rip + 0x1662b0f]
14161bfd9: 4c8d0de82a6601                 lea r9, [rip + 0x1662ae8]
14161bfe0: 4c8d4510                       lea r8, [rbp + 0x10]
14161bfe4: ba2b000000                     mov edx, 0x2b
14161bfe9: e872d72b01                     call 0x1428d9760
14161bfee: eb0c                           jmp 0x14161bffc
14161bff0: 488d0de1416601                 lea rcx, [rip + 0x16641e1]
14161bff7: e814d32b01                     call 0x1428d9310
14161bffc: 0f0b                           ud2 
14161bffe: 6690                           nop 
14161c000: 4889542410                     mov qword ptr [rsp + 0x10], rdx
14161c005: 55                             push rbp
14161c006: 4157                           push r15
14161c008: 4156                           push r14
14161c00a: 4154                           push r12
14161c00c: 56                             push rsi
14161c00d: 57                             push rdi
14161c00e: 53                             push rbx
14161c00f: 4883ec40                       sub rsp, 0x40
14161c013: 488daa80000000                 lea rbp, [rdx + 0x80]
14161c01a: 488d4d60                       lea rcx, [rbp + 0x60]
14161c01e: e88d645cff                     call 0x140be24b0
14161c023: 90                             nop 
14161c024: 4883c440                       add rsp, 0x40
14161c028: 5b                             pop rbx
14161c029: 5f                             pop rdi
14161c02a: 5e                             pop rsi
14161c02b: 415c                           pop r12
14161c02d: 415e                           pop r14
14161c02f: 415f                           pop r15
14161c031: 5d                             pop rbp
14161c032: c3                             ret 
14161c033: 666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
14161c040: 4889542410                     mov qword ptr [rsp + 0x10], rdx
14161c045: 55                             push rbp
14161c046: 4157                           push r15
14161c048: 4156                           push r14
14161c04a: 4154                           push r12
14161c04c: 56                             push rsi
14161c04d: 57                             push rdi
14161c04e: 53                             push rbx
14161c04f: 4883ec40                       sub rsp, 0x40
14161c053: 488daa80000000                 lea rbp, [rdx + 0x80]
14161c05a: 488d4d10                       lea rcx, [rbp + 0x10]
14161c05e: e80d2a1bff                     call 0x1407cea70
14161c063: 90                             nop 
14161c064: 4883c440                       add rsp, 0x40
14161c068: 5b                             pop rbx
14161c069: 5f                             pop rdi
14161c06a: 5e                             pop rsi
14161c06b: 415c                           pop r12
14161c06d: 415e                           pop r14
14161c06f: 415f                           pop r15
14161c071: 5d                             pop rbp
14161c072: c3                             ret 
