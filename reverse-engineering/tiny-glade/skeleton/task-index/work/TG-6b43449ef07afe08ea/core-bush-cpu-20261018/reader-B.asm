140eebaa0: 4157                           push r15
140eebaa2: 4156                           push r14
140eebaa4: 4155                           push r13
140eebaa6: 4154                           push r12
140eebaa8: 56                             push rsi
140eebaa9: 57                             push rdi
140eebaaa: 55                             push rbp
140eebaab: 53                             push rbx
140eebaac: 4881ec58040000                 sub rsp, 0x458
140eebab3: 0f29b42440040000               movaps xmmword ptr [rsp + 0x440], xmm6
140eebabb: 4889ce                         mov rsi, rcx
140eebabe: 488b0a                         mov rcx, qword ptr [rdx]
140eebac1: 488d15a0f70402                 lea rdx, [rip + 0x204f7a0]
140eebac8: e893afe000                     call 0x141cf6a60
140eebacd: 4885c0                         test rax, rax
140eebad0: 0f8437020000                   je 0x140eebd0d
140eebad6: 4889c7                         mov rdi, rax
140eebad9: 488d0548f80402                 lea rax, [rip + 0x204f848]
140eebae0: 4889842440020000               mov qword ptr [rsp + 0x240], rax
140eebae8: 48c78424480200005c000000       mov qword ptr [rsp + 0x248], 0x5c
140eebaf4: 48837f185c                     cmp qword ptr [rdi + 0x18], 0x5c
140eebaf9: 0f852a020000                   jne 0x140eebd29
140eebaff: 488b5710                       mov rdx, qword ptr [rdi + 0x10]
140eebb03: 488d0d1ef80402                 lea rcx, [rip + 0x204f81e]
140eebb0a: 41b85c000000                   mov r8d, 0x5c
140eebb10: e8db80a301                     call 0x142923bf0
140eebb15: 85c0                           test eax, eax
140eebb17: 0f850c020000                   jne 0x140eebd29
140eebb1d: 488b0f                         mov rcx, qword ptr [rdi]
140eebb20: 488b4708                       mov rax, qword ptr [rdi + 8]
140eebb24: ff5018                         call qword ptr [rax + 0x18]
140eebb27: 4885c0                         test rax, rax
140eebb2a: 0f84dd010000                   je 0x140eebd0d
140eebb30: 488b5008                       mov rdx, qword ptr [rax + 8]
140eebb34: 4885d2                         test rdx, rdx
140eebb37: 0f842c020000                   je 0x140eebd69
140eebb3d: 4c8b30                         mov r14, qword ptr [rax]
140eebb40: 418b06                         mov eax, dword ptr [r14]
140eebb43: 4839c2                         cmp rdx, rax
140eebb46: 0f860d020000                   jbe 0x140eebd59
140eebb4c: 8b4e1c                         mov ecx, dword ptr [rsi + 0x1c]
140eebb4f: 488b5610                       mov rdx, qword ptr [rsi + 0x10]
140eebb53: 890a                           mov dword ptr [rdx], ecx
140eebb55: 488b36                         mov rsi, qword ptr [rsi]
140eebb58: 488b4e10                       mov rcx, qword ptr [rsi + 0x10]
140eebb5c: 4885c9                         test rcx, rcx
140eebb5f: 7421                           je 0x140eebb82
140eebb61: 488b5608                       mov rdx, qword ptr [rsi + 8]
140eebb65: 48c1e105                       shl rcx, 5
140eebb69: 4531c0                         xor r8d, r8d
140eebb6c: 0f1f4000                       nop dword ptr [rax]
140eebb70: 4ac744021000000000             mov qword ptr [rdx + r8 + 0x10], 0
140eebb79: 4983c020                       add r8, 0x20
140eebb7d: 4c39c1                         cmp rcx, r8
140eebb80: 75ee                           jne 0x140eebb70
140eebb82: 85c0                           test eax, eax
140eebb84: 745d                           je 0x140eebbe3
140eebb86: 4c8d3cc500000000               lea r15, [rax*8]
140eebb8e: 4531e4                         xor r12d, r12d
140eebb91: 488d7c2440                     lea rdi, [rsp + 0x40]
140eebb96: f30f10359214ae01               movss xmm6, dword ptr [rip + 0x1ae1492]
140eebb9e: 488d5c2438                     lea rbx, [rsp + 0x38]
140eebba3: 666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
140eebbb0: f2430f104c2608                 movsd xmm1, qword ptr [r14 + r12 + 8]
140eebbb7: f20f114c2438                   movsd qword ptr [rsp + 0x38], xmm1
140eebbbd: f30f16d1                       movshdup xmm2, xmm1
140eebbc1: 4889f9                         mov rcx, rdi
140eebbc4: 0f28de                         movaps xmm3, xmm6
140eebbc7: e8a4443500                     call 0x141240070
140eebbcc: 4889f9                         mov rcx, rdi
140eebbcf: 4889f2                         mov rdx, rsi
140eebbd2: 4989d8                         mov r8, rbx
140eebbd5: e80683e000                     call 0x141cf3ee0
140eebbda: 4983c408                       add r12, 8
140eebbde: 4d39e7                         cmp r15, r12
140eebbe1: 75cd                           jne 0x140eebbb0
140eebbe3: 48c744244004000000             mov qword ptr [rsp + 0x40], 4
140eebbec: 48c744244800000000             mov qword ptr [rsp + 0x48], 0
140eebbf5: 488d4c2440                     lea rcx, [rsp + 0x40]
140eebbfa: e871b6e000                     call 0x141cf7270
140eebbff: 4889c7                         mov rdi, rax
140eebc02: 488d8c2440020000               lea rcx, [rsp + 0x240]
140eebc0a: e8b14ea7ff                     call 0x140960ac0
140eebc0f: 4c8b7e10                       mov r15, qword ptr [rsi + 0x10]
140eebc13: 4d85ff                         test r15, r15
140eebc16: 0f84f1000000                   je 0x140eebd0d
140eebc1c: 4c8b6608                       mov r12, qword ptr [rsi + 8]
140eebc20: 49c1e705                       shl r15, 5
140eebc24: 4d01e7                         add r15, r12
140eebc27: 4531ed                         xor r13d, r13d
140eebc2a: 488d5c2440                     lea rbx, [rsp + 0x40]
140eebc2f: 4c8d742437                     lea r14, [rsp + 0x37]
140eebc34: 31ed                           xor ebp, ebp
140eebc36: 4983c420                       add r12, 0x20
140eebc3a: 660f1f440000                   nop word ptr [rax + rax]
140eebc40: 498b4c24e8                     mov rcx, qword ptr [r12 - 0x18]
140eebc45: 498b5424f0                     mov rdx, qword ptr [r12 - 0x10]
140eebc4a: 48894c2440                     mov qword ptr [rsp + 0x40], rcx
140eebc4f: 4889542448                     mov qword ptr [rsp + 0x48], rdx
140eebc54: 4883fa02                       cmp rdx, 2
140eebc58: 7322                           jae 0x140eebc7c
140eebc5a: 4889d9                         mov rcx, rbx
140eebc5d: e80eb6e000                     call 0x141cf7270
140eebc62: 31f8                           xor eax, edi
140eebc64: 41394424f8                     cmp dword ptr [r12 - 8], eax
140eebc69: 7535                           jne 0x140eebca0
140eebc6b: 498d442420                     lea rax, [r12 + 0x20]
140eebc70: ffc5                           inc ebp
140eebc72: 4d39fc                         cmp r12, r15
140eebc75: 4989c4                         mov r12, rax
140eebc78: 75c6                           jne 0x140eebc40
140eebc7a: eb61                           jmp 0x140eebcdd
140eebc7c: 4883fa15                       cmp rdx, 0x15
140eebc80: 7310                           jae 0x140eebc92
140eebc82: 41b801000000                   mov r8d, 1
140eebc88: 4d89f1                         mov r9, r14
140eebc8b: e8e0dfe000                     call 0x141cf9c70
140eebc90: ebc8                           jmp 0x140eebc5a
140eebc92: 4d89f0                         mov r8, r14
140eebc95: e8f6e8e000                     call 0x141cfa590
140eebc9a: ebbe                           jmp 0x140eebc5a
140eebc9c: 0f1f4000                       nop dword ptr [rax]
140eebca0: 41894424f8                     mov dword ptr [r12 - 8], eax
140eebca5: 89e9                           mov ecx, ebp
140eebca7: e8a47594ff                     call 0x140833250
140eebcac: 89c1                           mov ecx, eax
140eebcae: c1e806                         shr eax, 6
140eebcb1: 81f900100000                   cmp ecx, 0x1000
140eebcb7: 0f83bc000000                   jae 0x140eebd79
140eebcbd: ba01000000                     mov edx, 1
140eebcc2: 48d3e2                         shl rdx, cl
140eebcc5: 480994c440020000               or qword ptr [rsp + rax*8 + 0x240], rdx
140eebccd: ffc5                           inc ebp
140eebccf: 41b501                         mov r13b, 1
140eebcd2: 4d39fc                         cmp r12, r15
140eebcd5: 0f855bffffff                   jne 0x140eebc36
140eebcdb: eb06                           jmp 0x140eebce3
140eebcdd: 41f6c501                       test r13b, 1
140eebce1: 742a                           je 0x140eebd0d
140eebce3: 4883c618                       add rsi, 0x18
140eebce7: 488d7c2440                     lea rdi, [rsp + 0x40]
140eebcec: 488d942440020000               lea rdx, [rsp + 0x240]
140eebcf4: 41b800020000                   mov r8d, 0x200
140eebcfa: 4889f9                         mov rcx, rdi
140eebcfd: e8fe7ea301                     call 0x142923c00
140eebd02: 4889f1                         mov rcx, rsi
140eebd05: 4889fa                         mov rdx, rdi
140eebd08: e8f3a7e000                     call 0x141cf6500
140eebd0d: 0f28b42440040000               movaps xmm6, xmmword ptr [rsp + 0x440]
140eebd15: 4881c458040000                 add rsp, 0x458
140eebd1c: 5b                             pop rbx
140eebd1d: 5d                             pop rbp
140eebd1e: 5f                             pop rdi
140eebd1f: 5e                             pop rsi
140eebd20: 415c                           pop r12
140eebd22: 415d                           pop r13
140eebd24: 415e                           pop r14
140eebd26: 415f                           pop r15
140eebd28: c3                             ret 
140eebd29: 48c744244000000000             mov qword ptr [rsp + 0x40], 0
140eebd32: 4883c710                       add rdi, 0x10
140eebd36: 488d0513f50402                 lea rax, [rip + 0x204f513]
140eebd3d: 4889442420                     mov qword ptr [rsp + 0x20], rax
140eebd42: 488d942440020000               lea rdx, [rsp + 0x240]
140eebd4a: 4c8d4c2440                     lea r9, [rsp + 0x40]
140eebd4f: 31c9                           xor ecx, ecx
140eebd51: 4989f8                         mov r8, rdi
140eebd54: e8e8ed9d01                     call 0x1428cab41
140eebd59: 488d4801                       lea rcx, [rax + 1]
140eebd5d: 4c8d0544ccca01                 lea r8, [rip + 0x1cacc44]
140eebd64: e8d7da9e01                     call 0x1428d9840
140eebd69: 4c8d0520ccca01                 lea r8, [rip + 0x1cacc20]
140eebd70: 31c9                           xor ecx, ecx
140eebd72: 31d2                           xor edx, edx
140eebd74: e89fd79e01                     call 0x1428d9518
140eebd79: 4c8d0598cbca01                 lea r8, [rip + 0x1cacb98]
140eebd80: ba40000000                     mov edx, 0x40
140eebd85: 4889c1                         mov rcx, rax
