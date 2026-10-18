0000000140843740: 55                             push rbp
0000000140843741: 4157                           push r15
0000000140843743: 4156                           push r14
0000000140843745: 4155                           push r13
0000000140843747: 4154                           push r12
0000000140843749: 56                             push rsi
000000014084374a: 57                             push rdi
000000014084374b: 53                             push rbx
000000014084374c: 4881ec58020000                 sub rsp, 0x258
0000000140843753: 488dac2480000000               lea rbp, [rsp + 0x80]
000000014084375b: 66440f7f85c0010000             movdqa xmmword ptr [rbp + 0x1c0], xmm8
0000000140843764: 0f29bdb0010000                 movaps xmmword ptr [rbp + 0x1b0], xmm7
000000014084376b: 0f29b5a0010000                 movaps xmmword ptr [rbp + 0x1a0], xmm6
0000000140843772: 48c78598010000feffffff         mov qword ptr [rbp + 0x198], 0xfffffffffffffffe
000000014084377d: 4c894d58                       mov qword ptr [rbp + 0x58], r9
0000000140843781: 4c89c6                         mov rsi, r8
0000000140843784: 4889d7                         mov rdi, rdx
0000000140843787: 4889cb                         mov rbx, rcx
000000014084378a: 31c9                           xor ecx, ecx
000000014084378c: e88f6389ff                     call 0x1400d9b20
0000000140843791: 4885c0                         test rax, rax
0000000140843794: 0f84270e0000                   je 0x1408445c1
000000014084379a: f30f6f00                       movdqu xmm0, xmmword ptr [rax]
000000014084379e: 48ff00                         inc qword ptr [rax]
00000001408437a1: 660f7f8590000000               movdqa xmmword ptr [rbp + 0x90], xmm0
00000001408437a9: f30f6f354f874202               movdqu xmm6, xmmword ptr [rip + 0x242874f]
00000001408437b1: 660f7f7570                     movdqa xmmword ptr [rbp + 0x70], xmm6
00000001408437b6: f30f6f3d52874202               movdqu xmm7, xmmword ptr [rip + 0x2428752]
00000001408437be: 660f7fbd80000000               movdqa xmmword ptr [rbp + 0x80], xmm7
00000001408437c6: 31c9                           xor ecx, ecx
00000001408437c8: e8536389ff                     call 0x1400d9b20
00000001408437cd: 48899dc0000000                 mov qword ptr [rbp + 0xc0], rbx
00000001408437d4: 4889bd78010000                 mov qword ptr [rbp + 0x178], rdi
00000001408437db: 4889b568010000                 mov qword ptr [rbp + 0x168], rsi
00000001408437e2: 4885c0                         test rax, rax
00000001408437e5: 0f84e20d0000                   je 0x1408445cd
00000001408437eb: 488b8d58020000                 mov rcx, qword ptr [rbp + 0x258]
00000001408437f2: f30f6f00                       movdqu xmm0, xmmword ptr [rax]
00000001408437f6: 48ff00                         inc qword ptr [rax]
00000001408437f9: 660f7f4540                     movdqa xmmword ptr [rbp + 0x40], xmm0
00000001408437fe: 660f7f7520                     movdqa xmmword ptr [rbp + 0x20], xmm6
0000000140843803: 660f7f7d30                     movdqa xmmword ptr [rbp + 0x30], xmm7
0000000140843808: 488b19                         mov rbx, qword ptr [rcx]
000000014084380b: 488b4908                       mov rcx, qword ptr [rcx + 8]
000000014084380f: 4c8b03                         mov r8, qword ptr [rbx]
0000000140843812: 31d2                           xor edx, edx
0000000140843814: 4c89c0                         mov rax, r8
0000000140843817: 482b4118                       sub rax, qword ptr [rcx + 0x18]
000000014084381b: 480f42c2                       cmovb rax, rdx
000000014084381f: 4c2b4138                       sub r8, qword ptr [rcx + 0x38]
0000000140843823: 4c0f42c2                       cmovb r8, rdx
0000000140843827: 4c6be838                       imul r13, rax, 0x38
000000014084382b: 4c036908                       add r13, qword ptr [rcx + 8]
000000014084382f: 4c8b4910                       mov r9, qword ptr [rcx + 0x10]
0000000140843833: 4929c1                         sub r9, rax
0000000140843836: 4c0f42ca                       cmovb r9, rdx
000000014084383a: 41ba08000000                   mov r10d, 8
0000000140843840: 4d0f42ea                       cmovb r13, r10
0000000140843844: 496bc038                       imul rax, r8, 0x38
0000000140843848: 48034128                       add rax, qword ptr [rcx + 0x28]
000000014084384c: 4c8b5930                       mov r11, qword ptr [rcx + 0x30]
0000000140843850: 4d29c3                         sub r11, r8
0000000140843853: 4c0f42da                       cmovb r11, rdx
0000000140843857: 490f42c2                       cmovb rax, r10
000000014084385b: 488b4940                       mov rcx, qword ptr [rcx + 0x40]
000000014084385f: 4c29d9                         sub rcx, r11
0000000140843862: 4c29c9                         sub rcx, r9
0000000140843865: 48890b                         mov qword ptr [rbx], rcx
0000000140843868: 496bf138                       imul rsi, r9, 0x38
000000014084386c: 4c01ee                         add rsi, r13
000000014084386f: 496bcb38                       imul rcx, r11, 0x38
0000000140843873: 4801c1                         add rcx, rax
0000000140843876: 48898d88010000                 mov qword ptr [rbp + 0x188], rcx
000000014084387d: e994000000                     jmp 0x140843916
0000000140843882: 66666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
0000000140843890: 498b7d00                       mov rdi, qword ptr [r13]
0000000140843894: 4d8b6508                       mov r12, qword ptr [r13 + 8]
0000000140843898: 660f6f37                       movdqa xmm6, xmmword ptr [rdi]
000000014084389c: 4d8b6d18                       mov r13, qword ptr [r13 + 0x18]
00000001408438a0: 498d5501                       lea rdx, [r13 + 1]
00000001408438a4: 48d1ea                         shr rdx, 1
00000001408438a7: 48399580000000                 cmp qword ptr [rbp + 0x80], rdx
00000001408438ae: 0f82d1000000                   jb 0x140843985
00000001408438b4: 4a8d0427                       lea rax, [rdi + r12]
00000001408438b8: 48ffc0                         inc rax
00000001408438bb: 660fd7ce                       pmovmskb ecx, xmm6
00000001408438bf: f7d1                           not ecx
00000001408438c1: 488d5570                       lea rdx, [rbp + 0x70]
00000001408438c5: 488995a0000000                 mov qword ptr [rbp + 0xa0], rdx
00000001408438cc: 488d95a0000000                 lea rdx, [rbp + 0xa0]
00000001408438d3: 488995e8000000                 mov qword ptr [rbp + 0xe8], rdx
00000001408438da: 4889bd30010000                 mov qword ptr [rbp + 0x130], rdi
00000001408438e1: 4883c710                       add rdi, 0x10
00000001408438e5: 4889bd38010000                 mov qword ptr [rbp + 0x138], rdi
00000001408438ec: 48898540010000                 mov qword ptr [rbp + 0x140], rax
00000001408438f3: 66898d48010000                 mov word ptr [rbp + 0x148], cx
00000001408438fa: 488d8d30010000                 lea rcx, [rbp + 0x130]
0000000140843901: 4c89ea                         mov rdx, r13
0000000140843904: 4c8d85e8000000                 lea r8, [rbp + 0xe8]
000000014084390b: e800273300                     call 0x140b76010
0000000140843910: 4d89fd                         mov r13, r15
0000000140843913: 4c89f0                         mov rax, r14
0000000140843916: 4d85ed                         test r13, r13
0000000140843919: 7425                           je 0x140843940
000000014084391b: 4939f5                         cmp r13, rsi
000000014084391e: 7420                           je 0x140843940
0000000140843920: 4d8d7d38                       lea r15, [r13 + 0x38]
0000000140843924: 4989c6                         mov r14, rax
0000000140843927: 48ff03                         inc qword ptr [rbx]
000000014084392a: 4883bd8800000000               cmp qword ptr [rbp + 0x88], 0
0000000140843932: 0f8558ffffff                   jne 0x140843890
0000000140843938: eb2f                           jmp 0x140843969
000000014084393a: 660f1f440000                   nop word ptr [rax + rax]
0000000140843940: 4885c0                         test rax, rax
0000000140843943: 7458                           je 0x14084399d
0000000140843945: 483b8588010000                 cmp rax, qword ptr [rbp + 0x188]
000000014084394c: 744f                           je 0x14084399d
000000014084394e: 4c8d7038                       lea r14, [rax + 0x38]
0000000140843952: 4531ff                         xor r15d, r15d
0000000140843955: 4989c5                         mov r13, rax
0000000140843958: 48ff03                         inc qword ptr [rbx]
000000014084395b: 4883bd8800000000               cmp qword ptr [rbp + 0x88], 0
0000000140843963: 0f8527ffffff                   jne 0x140843890
0000000140843969: 488d4d70                       lea rcx, [rbp + 0x70]
000000014084396d: 4c89ea                         mov rdx, r13
0000000140843970: e83bd93200                     call 0x140b712b0
0000000140843975: f3410f6f4520                   movdqu xmm0, xmmword ptr [r13 + 0x20]
000000014084397b: 660f7f8590000000               movdqa xmmword ptr [rbp + 0x90], xmm0
0000000140843983: eb8b                           jmp 0x140843910
0000000140843985: 488d4d70                       lea rcx, [rbp + 0x70]
0000000140843989: 4c8d8590000000                 lea r8, [rbp + 0x90]
0000000140843990: 41b101                         mov r9b, 1
0000000140843993: e878e70602                     call 0x1428b2110
0000000140843998: e917ffffff                     jmp 0x1408438b4
000000014084399d: 488b8560020000                 mov rax, qword ptr [rbp + 0x260]
00000001408439a4: 488b18                         mov rbx, qword ptr [rax]
00000001408439a7: 488b4808                       mov rcx, qword ptr [rax + 8]
00000001408439ab: 4c8b03                         mov r8, qword ptr [rbx]
00000001408439ae: 31d2                           xor edx, edx
00000001408439b0: 4c89c0                         mov rax, r8
00000001408439b3: 482b4118                       sub rax, qword ptr [rcx + 0x18]
00000001408439b7: 480f42c2                       cmovb rax, rdx
00000001408439bb: 4c2b4138                       sub r8, qword ptr [rcx + 0x38]
00000001408439bf: 4c0f42c2                       cmovb r8, rdx
00000001408439c3: 4c6be838                       imul r13, rax, 0x38
00000001408439c7: 4c036908                       add r13, qword ptr [rcx + 8]
00000001408439cb: 4c8b4910                       mov r9, qword ptr [rcx + 0x10]
00000001408439cf: 4929c1                         sub r9, rax
00000001408439d2: 4c0f42ca                       cmovb r9, rdx
00000001408439d6: 4c8b5130                       mov r10, qword ptr [rcx + 0x30]
00000001408439da: 41bb08000000                   mov r11d, 8
00000001408439e0: 4d0f42eb                       cmovb r13, r11
00000001408439e4: 496bc038                       imul rax, r8, 0x38
00000001408439e8: 48034128                       add rax, qword ptr [rcx + 0x28]
00000001408439ec: 4d29c2                         sub r10, r8
00000001408439ef: 4c0f42d2                       cmovb r10, rdx
00000001408439f3: 490f42c3                       cmovb rax, r11
00000001408439f7: 488b4940                       mov rcx, qword ptr [rcx + 0x40]
00000001408439fb: 4c29d1                         sub rcx, r10
00000001408439fe: 4c29c9                         sub rcx, r9
0000000140843a01: 48890b                         mov qword ptr [rbx], rcx
0000000140843a04: 496bf138                       imul rsi, r9, 0x38
0000000140843a08: 4c01ee                         add rsi, r13
0000000140843a0b: 496bca38                       imul rcx, r10, 0x38
0000000140843a0f: 4801c1                         add rcx, rax
0000000140843a12: 48898d88010000                 mov qword ptr [rbp + 0x188], rcx
0000000140843a19: e985000000                     jmp 0x140843aa3
0000000140843a1e: 6690                           nop 
0000000140843a20: 498b7d00                       mov rdi, qword ptr [r13]
0000000140843a24: 4d8b7508                       mov r14, qword ptr [r13 + 8]
0000000140843a28: 660f6f37                       movdqa xmm6, xmmword ptr [rdi]
0000000140843a2c: 4d8b6d18                       mov r13, qword ptr [r13 + 0x18]
0000000140843a30: 498d5501                       lea rdx, [r13 + 1]
0000000140843a34: 48d1ea                         shr rdx, 1
0000000140843a37: 48395530                       cmp qword ptr [rbp + 0x30], rdx
0000000140843a3b: 0f82ce000000                   jb 0x140843b0f
0000000140843a41: 4a8d0437                       lea rax, [rdi + r14]
0000000140843a45: 48ffc0                         inc rax
0000000140843a48: 660fd7ce                       pmovmskb ecx, xmm6
0000000140843a4c: f7d1                           not ecx
0000000140843a4e: 488d5520                       lea rdx, [rbp + 0x20]
0000000140843a52: 488995a0000000                 mov qword ptr [rbp + 0xa0], rdx
0000000140843a59: 488d95a0000000                 lea rdx, [rbp + 0xa0]
0000000140843a60: 488995e8000000                 mov qword ptr [rbp + 0xe8], rdx
0000000140843a67: 4889bd30010000                 mov qword ptr [rbp + 0x130], rdi
0000000140843a6e: 4883c710                       add rdi, 0x10
0000000140843a72: 4889bd38010000                 mov qword ptr [rbp + 0x138], rdi
0000000140843a79: 48898540010000                 mov qword ptr [rbp + 0x140], rax
0000000140843a80: 66898d48010000                 mov word ptr [rbp + 0x148], cx
0000000140843a87: 488d8d30010000                 lea rcx, [rbp + 0x130]
0000000140843a8e: 4c89ea                         mov rdx, r13
0000000140843a91: 4c8d85e8000000                 lea r8, [rbp + 0xe8]
0000000140843a98: e823273300                     call 0x140b761c0
0000000140843a9d: 4d89fd                         mov r13, r15
0000000140843aa0: 4c89e0                         mov rax, r12
0000000140843aa3: 4d85ed                         test r13, r13
0000000140843aa6: 7428                           je 0x140843ad0
0000000140843aa8: 4939f5                         cmp r13, rsi
0000000140843aab: 7423                           je 0x140843ad0
0000000140843aad: 4d8d7d38                       lea r15, [r13 + 0x38]
0000000140843ab1: 4989c4                         mov r12, rax
0000000140843ab4: 48ff03                         inc qword ptr [rbx]
0000000140843ab7: 48837d3800                     cmp qword ptr [rbp + 0x38], 0
0000000140843abc: 0f855effffff                   jne 0x140843a20
0000000140843ac2: eb32                           jmp 0x140843af6
0000000140843ac4: 6666662e0f1f840000000000       nop word ptr cs:[rax + rax]
0000000140843ad0: 4885c0                         test rax, rax
0000000140843ad3: 744f                           je 0x140843b24
0000000140843ad5: 483b8588010000                 cmp rax, qword ptr [rbp + 0x188]
0000000140843adc: 7446                           je 0x140843b24
0000000140843ade: 4c8d6038                       lea r12, [rax + 0x38]
0000000140843ae2: 4531ff                         xor r15d, r15d
0000000140843ae5: 4989c5                         mov r13, rax
0000000140843ae8: 48ff03                         inc qword ptr [rbx]
0000000140843aeb: 48837d3800                     cmp qword ptr [rbp + 0x38], 0
0000000140843af0: 0f852affffff                   jne 0x140843a20
0000000140843af6: 488d4d20                       lea rcx, [rbp + 0x20]
0000000140843afa: 4c89ea                         mov rdx, r13
0000000140843afd: e8eee93200                     call 0x140b724f0
0000000140843b02: f3410f6f4520                   movdqu xmm0, xmmword ptr [r13 + 0x20]
0000000140843b08: 660f7f4540                     movdqa xmmword ptr [rbp + 0x40], xmm0
0000000140843b0d: eb8e                           jmp 0x140843a9d
0000000140843b0f: 488d4d20                       lea rcx, [rbp + 0x20]
0000000140843b13: 4c8d4540                       lea r8, [rbp + 0x40]
0000000140843b17: 41b101                         mov r9b, 1
0000000140843b1a: e891cd0702                     call 0x1428c08b0
0000000140843b1f: e91dffffff                     jmp 0x140843a41
0000000140843b24: 488b9550020000                 mov rdx, qword ptr [rbp + 0x250]
0000000140843b2b: 488b0a                         mov rcx, qword ptr [rdx]
0000000140843b2e: 488b4208                       mov rax, qword ptr [rdx + 8]
0000000140843b32: 488985e0000000                 mov qword ptr [rbp + 0xe0], rax
0000000140843b39: 8b4214                         mov eax, dword ptr [rdx + 0x14]
0000000140843b3c: 8985b4000000                   mov dword ptr [rbp + 0xb4], eax
0000000140843b42: 4c8bb1f8000000                 mov r14, qword ptr [rcx + 0xf8]
0000000140843b49: 488b8100010000                 mov rax, qword ptr [rcx + 0x100]
0000000140843b50: 498d0486                       lea rax, [r14 + rax*4]
0000000140843b54: 48898510010000                 mov qword ptr [rbp + 0x110], rax
0000000140843b5b: 488b9540020000                 mov rdx, qword ptr [rbp + 0x240]
0000000140843b62: 8b421c                         mov eax, dword ptr [rdx + 0x1c]
0000000140843b65: 898564010000                   mov dword ptr [rbp + 0x164], eax
0000000140843b6b: 488b02                         mov rax, qword ptr [rdx]
0000000140843b6e: 48898508010000                 mov qword ptr [rbp + 0x108], rax
0000000140843b75: 488b4210                       mov rax, qword ptr [rdx + 0x10]
0000000140843b79: 48894550                       mov qword ptr [rbp + 0x50], rax
0000000140843b7d: 488b85c0000000                 mov rax, qword ptr [rbp + 0xc0]
0000000140843b84: 488b00                         mov rax, qword ptr [rax]
0000000140843b87: 48894510                       mov qword ptr [rbp + 0x10], rax
0000000140843b8b: 488b8578010000                 mov rax, qword ptr [rbp + 0x178]
0000000140843b92: 488b00                         mov rax, qword ptr [rax]
0000000140843b95: 48894518                       mov qword ptr [rbp + 0x18], rax
0000000140843b99: 488b8568010000                 mov rax, qword ptr [rbp + 0x168]
0000000140843ba0: 488b00                         mov rax, qword ptr [rax]
0000000140843ba3: 488945c8                       mov qword ptr [rbp - 0x38], rax
0000000140843ba7: 488b9568020000                 mov rdx, qword ptr [rbp + 0x268]
0000000140843bae: 488b02                         mov rax, qword ptr [rdx]
0000000140843bb1: 488945d0                       mov qword ptr [rbp - 0x30], rax
0000000140843bb5: 488b4208                       mov rax, qword ptr [rdx + 8]
0000000140843bb9: 488945d8                       mov qword ptr [rbp - 0x28], rax
0000000140843bbd: 488b4218                       mov rax, qword ptr [rdx + 0x18]
0000000140843bc1: 488945e0                       mov qword ptr [rbp - 0x20], rax
0000000140843bc5: 488b4210                       mov rax, qword ptr [rdx + 0x10]
0000000140843bc9: 488945e8                       mov qword ptr [rbp - 0x18], rax
0000000140843bcd: 660f76f6                       pcmpeqd xmm6, xmm6
0000000140843bd1: 48898d28010000                 mov qword ptr [rbp + 0x128], rcx
0000000140843bd8: 0fb68108010000                 movzx eax, byte ptr [rcx + 0x108]
0000000140843bdf: 888597010000                   mov byte ptr [rbp + 0x197], al
0000000140843be5: 488b4558                       mov rax, qword ptr [rbp + 0x58]
0000000140843be9: 8b481c                         mov ecx, dword ptr [rax + 0x1c]
0000000140843bec: 898db8000000                   mov dword ptr [rbp + 0xb8], ecx
0000000140843bf2: 488b08                         mov rcx, qword ptr [rax]
0000000140843bf5: 48894df0                       mov qword ptr [rbp - 0x10], rcx
0000000140843bf9: 488b4010                       mov rax, qword ptr [rax + 0x10]
0000000140843bfd: 488945f8                       mov qword ptr [rbp - 8], rax
0000000140843c01: 488b8548020000                 mov rax, qword ptr [rbp + 0x248]
0000000140843c08: 8b481c                         mov ecx, dword ptr [rax + 0x1c]
0000000140843c0b: 898dbc000000                   mov dword ptr [rbp + 0xbc], ecx
0000000140843c11: 488b08                         mov rcx, qword ptr [rax]
0000000140843c14: 48894d00                       mov qword ptr [rbp], rcx
0000000140843c18: 488b4010                       mov rax, qword ptr [rax + 0x10]
0000000140843c1c: 48894508                       mov qword ptr [rbp + 8], rax
0000000140843c20: b808000000                     mov eax, 8
0000000140843c25: 48894560                       mov qword ptr [rbp + 0x60], rax
0000000140843c29: 48c7852001000000000000         mov qword ptr [rbp + 0x120], 0
0000000140843c34: 48894568                       mov qword ptr [rbp + 0x68], rax
0000000140843c38: 48c7858001000000000000         mov qword ptr [rbp + 0x180], 0
0000000140843c43: 48c7851801000000000000         mov qword ptr [rbp + 0x118], 0
0000000140843c4e: 48c7857001000000000000         mov qword ptr [rbp + 0x170], 0
0000000140843c59: 80bd9701000000                 cmp byte ptr [rbp + 0x197], 0
0000000140843c60: 746e                           je 0x140843cd0
0000000140843c62: 488b8570010000                 mov rax, qword ptr [rbp + 0x170]
0000000140843c69: 483b8518010000                 cmp rax, qword ptr [rbp + 0x118]
0000000140843c70: 0f85c4010000                   jne 0x140843e3a
0000000140843c76: 4c3bb510010000                 cmp r14, qword ptr [rbp + 0x110]
0000000140843c7d: 0f8428080000                   je 0x1408444ab
0000000140843c83: 488b85e0000000                 mov rax, qword ptr [rbp + 0xe0]
0000000140843c8a: 488b80a8010000                 mov rax, qword ptr [rax + 0x1a8]
0000000140843c91: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0000000140843ca0: 418b0e                         mov ecx, dword ptr [r14]
0000000140843ca3: 488d0cc9                       lea rcx, [rcx + rcx*8]
0000000140843ca7: 488b54c810                     mov rdx, qword ptr [rax + rcx*8 + 0x10]
0000000140843cac: 4885d2                         test rdx, rdx
0000000140843caf: 7576                           jne 0x140843d27
0000000140843cb1: 4983c604                       add r14, 4
0000000140843cb5: 4c3bb510010000                 cmp r14, qword ptr [rbp + 0x110]
0000000140843cbc: 75e2                           jne 0x140843ca0
0000000140843cbe: e9e8070000                     jmp 0x1408444ab
0000000140843cc3: 666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
0000000140843cd0: 488b8570010000                 mov rax, qword ptr [rbp + 0x170]
0000000140843cd7: 483b8518010000                 cmp rax, qword ptr [rbp + 0x118]
0000000140843cde: 0f85c1010000                   jne 0x140843ea5
0000000140843ce4: 4c3bb510010000                 cmp r14, qword ptr [rbp + 0x110]
0000000140843ceb: 0f84ba070000                   je 0x1408444ab
0000000140843cf1: 488b85e0000000                 mov rax, qword ptr [rbp + 0xe0]
0000000140843cf8: 488b8000010000                 mov rax, qword ptr [rax + 0x100]
0000000140843cff: 90                             nop 
0000000140843d00: 418b0e                         mov ecx, dword ptr [r14]
0000000140843d03: 488d0c89                       lea rcx, [rcx + rcx*4]
0000000140843d07: 48c1e105                       shl rcx, 5
0000000140843d0b: 488b540858                     mov rdx, qword ptr [rax + rcx + 0x58]
0000000140843d10: 4885d2                         test rdx, rdx
0000000140843d13: 7563                           jne 0x140843d78
0000000140843d15: 4983c604                       add r14, 4
0000000140843d19: 4c3bb510010000                 cmp r14, qword ptr [rbp + 0x110]
0000000140843d20: 75de                           jne 0x140843d00
0000000140843d22: e984070000                     jmp 0x1408444ab
0000000140843d27: 48899518010000                 mov qword ptr [rbp + 0x118], rdx
0000000140843d2e: 488d04c8                       lea rax, [rax + rcx*8]
0000000140843d32: 488b8d28010000                 mov rcx, qword ptr [rbp + 0x128]
0000000140843d39: 4c8b8118010000                 mov r8, qword ptr [rcx + 0x118]
0000000140843d40: 488b4818                       mov rcx, qword ptr [rax + 0x18]
0000000140843d44: 488b5038                       mov rdx, qword ptr [rax + 0x38]
0000000140843d48: 4c394040                       cmp qword ptr [rax + 0x40], r8
0000000140843d4c: 0f8691000000                   jbe 0x140843de3
0000000140843d52: 4e8b04c2                       mov r8, qword ptr [rdx + r8*8]
0000000140843d56: 4d85c0                         test r8, r8
0000000140843d59: 0f8484000000                   je 0x140843de3
0000000140843d5f: 49f7d0                         not r8
0000000140843d62: 4f8d0440                       lea r8, [r8 + r8*2]
0000000140843d66: 49c1e004                       shl r8, 4
0000000140843d6a: 4e8b440110                     mov r8, qword ptr [rcx + r8 + 0x10]
0000000140843d6f: 4c898580010000                 mov qword ptr [rbp + 0x180], r8
0000000140843d76: eb76                           jmp 0x140843dee
0000000140843d78: 48899518010000                 mov qword ptr [rbp + 0x118], rdx
0000000140843d7f: 4801c8                         add rax, rcx
0000000140843d82: 8b8894000000                   mov ecx, dword ptr [rax + 0x94]
0000000140843d88: 488b95e0000000                 mov rdx, qword ptr [rbp + 0xe0]
0000000140843d8f: 4c8b8aa8010000                 mov r9, qword ptr [rdx + 0x1a8]
0000000140843d96: 4c8d14c9                       lea r10, [rcx + rcx*8]
0000000140843d9a: 488b8d28010000                 mov rcx, qword ptr [rbp + 0x128]
0000000140843da1: 4c8b8118010000                 mov r8, qword ptr [rcx + 0x118]
0000000140843da8: 4b8b4cd118                     mov rcx, qword ptr [r9 + r10*8 + 0x18]
0000000140843dad: 4b8b54d138                     mov rdx, qword ptr [r9 + r10*8 + 0x38]
0000000140843db2: 4f3944d140                     cmp qword ptr [r9 + r10*8 + 0x40], r8
0000000140843db7: 0f8691000000                   jbe 0x140843e4e
0000000140843dbd: 4e8b04c2                       mov r8, qword ptr [rdx + r8*8]
0000000140843dc1: 4d85c0                         test r8, r8
0000000140843dc4: 0f8484000000                   je 0x140843e4e
0000000140843dca: 49f7d0                         not r8
0000000140843dcd: 4f8d0440                       lea r8, [r8 + r8*2]
0000000140843dd1: 49c1e004                       shl r8, 4
0000000140843dd5: 4e8b440110                     mov r8, qword ptr [rcx + r8 + 0x10]
0000000140843dda: 4c898580010000                 mov qword ptr [rbp + 0x180], r8
0000000140843de1: eb76                           jmp 0x140843e59
0000000140843de3: 48c7858001000000000000         mov qword ptr [rbp + 0x180], 0
0000000140843dee: 4c8b8528010000                 mov r8, qword ptr [rbp + 0x128]
0000000140843df5: 4d8b8010010000                 mov r8, qword ptr [r8 + 0x110]
0000000140843dfc: 4a8b14c2                       mov rdx, qword ptr [rdx + r8*8]
0000000140843e00: 48f7d2                         not rdx
0000000140843e03: 488d1452                       lea rdx, [rdx + rdx*2]
0000000140843e07: 48c1e204                       shl rdx, 4
0000000140843e0b: 4c8b441110                     mov r8, qword ptr [rcx + rdx + 0x10]
0000000140843e10: 4c898520010000                 mov qword ptr [rbp + 0x120], r8
0000000140843e17: 488b4c1128                     mov rcx, qword ptr [rcx + rdx + 0x28]
0000000140843e1c: 48898dd8000000                 mov qword ptr [rbp + 0xd8], rcx
0000000140843e23: 4983c604                       add r14, 4
0000000140843e27: 488b4008                       mov rax, qword ptr [rax + 8]
0000000140843e2b: 48894568                       mov qword ptr [rbp + 0x68], rax
0000000140843e2f: 48c7857001000000000000         mov qword ptr [rbp + 0x170], 0
0000000140843e3a: 488b4568                       mov rax, qword ptr [rbp + 0x68]
0000000140843e3e: 4c8b8570010000                 mov r8, qword ptr [rbp + 0x170]
0000000140843e45: 4a8d04c0                       lea rax, [rax + r8*8]
0000000140843e49: 4489c1                         mov ecx, r8d
0000000140843e4c: eb71                           jmp 0x140843ebf
0000000140843e4e: 48c7858001000000000000         mov qword ptr [rbp + 0x180], 0
0000000140843e59: 4c8b8528010000                 mov r8, qword ptr [rbp + 0x128]
0000000140843e60: 4d8b8010010000                 mov r8, qword ptr [r8 + 0x110]
0000000140843e67: 4a8b14c2                       mov rdx, qword ptr [rdx + r8*8]
0000000140843e6b: 48f7d2                         not rdx
0000000140843e6e: 488d1452                       lea rdx, [rdx + rdx*2]
0000000140843e72: 48c1e204                       shl rdx, 4
0000000140843e76: 4c8b441110                     mov r8, qword ptr [rcx + rdx + 0x10]
0000000140843e7b: 4c898520010000                 mov qword ptr [rbp + 0x120], r8
0000000140843e82: 488b4c1128                     mov rcx, qword ptr [rcx + rdx + 0x28]
0000000140843e87: 48898dd8000000                 mov qword ptr [rbp + 0xd8], rcx
0000000140843e8e: 4983c604                       add r14, 4
0000000140843e92: 488b4050                       mov rax, qword ptr [rax + 0x50]
0000000140843e96: 48894560                       mov qword ptr [rbp + 0x60], rax
0000000140843e9a: 48c7857001000000000000         mov qword ptr [rbp + 0x170], 0
0000000140843ea5: 4c8b8570010000                 mov r8, qword ptr [rbp + 0x170]
0000000140843eac: 4c89c1                         mov rcx, r8
0000000140843eaf: 48c1e104                       shl rcx, 4
0000000140843eb3: 488b5560                       mov rdx, qword ptr [rbp + 0x60]
0000000140843eb7: 488d040a                       lea rax, [rdx + rcx]
0000000140843ebb: 8b4c0a08                       mov ecx, dword ptr [rdx + rcx + 8]
0000000140843ebf: 4c6bc958                       imul r9, rcx, 0x58
0000000140843ec3: 4c038d20010000                 add r9, qword ptr [rbp + 0x120]
0000000140843eca: 488b95d8000000                 mov rdx, qword ptr [rbp + 0xd8]
0000000140843ed1: 4c8d2c8a                       lea r13, [rdx + rcx*4]
0000000140843ed5: 488d0c49                       lea rcx, [rcx + rcx*2]
0000000140843ed9: 48c1e104                       shl rcx, 4
0000000140843edd: 48038d80010000                 add rcx, qword ptr [rbp + 0x180]
0000000140843ee4: 48898d88010000                 mov qword ptr [rbp + 0x188], rcx
0000000140843eeb: 488b00                         mov rax, qword ptr [rax]
0000000140843eee: 488985c0000000                 mov qword ptr [rbp + 0xc0], rax
0000000140843ef5: 49ffc0                         inc r8
0000000140843ef8: 4c898570010000                 mov qword ptr [rbp + 0x170], r8
0000000140843eff: 4c898d78010000                 mov qword ptr [rbp + 0x178], r9
0000000140843f06: 498d4148                       lea rax, [r9 + 0x48]
0000000140843f0a: 48898568010000                 mov qword ptr [rbp + 0x168], rax
0000000140843f11: 4883bd8800000000               cmp qword ptr [rbp + 0x88], 0
0000000140843f19: 0f8401010000                   je 0x140844020
0000000140843f1f: 488d8d90000000                 lea rcx, [rbp + 0x90]
0000000140843f26: 488b9568010000                 mov rdx, qword ptr [rbp + 0x168]
0000000140843f2d: e8ce3baa00                     call 0x1412e7b00
0000000140843f32: 4c8b7d70                       mov r15, qword ptr [rbp + 0x70]
0000000140843f36: 488b5d78                       mov rbx, qword ptr [rbp + 0x78]
0000000140843f3a: 4989dc                         mov r12, rbx
0000000140843f3d: 4921c4                         and r12, rax
0000000140843f40: 48c1e839                       shr rax, 0x39
0000000140843f44: 660f6ec0                       movd xmm0, eax
0000000140843f48: 660f60c0                       punpcklbw xmm0, xmm0
0000000140843f4c: f20f70c000                     pshuflw xmm0, xmm0, 0
0000000140843f51: 660f70f800                     pshufd xmm7, xmm0, 0
0000000140843f56: 498d57f0                       lea rdx, [r15 - 0x10]
0000000140843f5a: 31c9                           xor ecx, ecx
0000000140843f5c: f3470f6f0427                   movdqu xmm8, xmmword ptr [r15 + r12]
0000000140843f62: 66410f6fc0                     movdqa xmm0, xmm8
0000000140843f67: 660f74c7                       pcmpeqb xmm0, xmm7
0000000140843f6b: 660fd7f0                       pmovmskb esi, xmm0
0000000140843f6f: 85f6                           test esi, esi
0000000140843f71: 747d                           je 0x140843ff0
0000000140843f73: 488b8578010000                 mov rax, qword ptr [rbp + 0x178]
0000000140843f7a: 488b7850                       mov rdi, qword ptr [rax + 0x50]
0000000140843f7e: 488995c8000000                 mov qword ptr [rbp + 0xc8], rdx
0000000140843f85: 48898dd0000000                 mov qword ptr [rbp + 0xd0], rcx
0000000140843f8c: f30fbcce                       tzcnt ecx, esi
0000000140843f90: 4c01e1                         add rcx, r12
0000000140843f93: 4821d9                         and rcx, rbx
0000000140843f96: 48c1e104                       shl rcx, 4
0000000140843f9a: 4889d0                         mov rax, rdx
0000000140843f9d: 4829c8                         sub rax, rcx
0000000140843fa0: 483b7808                       cmp rdi, qword ptr [rax + 8]
0000000140843fa4: 7525                           jne 0x140843fcb
0000000140843fa6: 488b10                         mov rdx, qword ptr [rax]
0000000140843fa9: 4883c210                       add rdx, 0x10
0000000140843fad: 488b8568010000                 mov rax, qword ptr [rbp + 0x168]
0000000140843fb4: 488b08                         mov rcx, qword ptr [rax]
0000000140843fb7: 4883c110                       add rcx, 0x10
0000000140843fbb: 4989f8                         mov r8, rdi
0000000140843fbe: e82dfc0d02                     call 0x142923bf0
0000000140843fc3: 85c0                           test eax, eax
0000000140843fc5: 0f8435010000                   je 0x140844100
0000000140843fcb: 8d46ff                         lea eax, [rsi - 1]
0000000140843fce: 6621f0                         and ax, si
0000000140843fd1: 89c6                           mov esi, eax
0000000140843fd3: 488b95c8000000                 mov rdx, qword ptr [rbp + 0xc8]
0000000140843fda: 488b8dd0000000                 mov rcx, qword ptr [rbp + 0xd0]
0000000140843fe1: 75a9                           jne 0x140843f8c
0000000140843fe3: 666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
0000000140843ff0: 66440f74c6                     pcmpeqb xmm8, xmm6
0000000140843ff5: 66410fd7c0                     pmovmskb eax, xmm8
0000000140843ffa: 85c0                           test eax, eax
0000000140843ffc: 7522                           jne 0x140844020
0000000140843ffe: 4901cc                         add r12, rcx
0000000140844001: 4983c410                       add r12, 0x10
0000000140844005: 4883c110                       add rcx, 0x10
0000000140844009: 4921dc                         and r12, rbx
000000014084400c: e94bffffff                     jmp 0x140843f5c
0000000140844011: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0000000140844020: 488b8d78010000                 mov rcx, qword ptr [rbp + 0x178]
0000000140844027: 488b4110                       mov rax, qword ptr [rcx + 0x10]
000000014084402b: 4885c0                         test rax, rax
000000014084402e: 0f8425fcffff                   je 0x140843c59
0000000140844034: 48837d3800                     cmp qword ptr [rbp + 0x38], 0
0000000140844039: 0f8459040000                   je 0x140844498
000000014084403f: 488b7908                       mov rdi, qword ptr [rcx + 8]
0000000140844043: 488d0440                       lea rax, [rax + rax*2]
0000000140844047: 488d3487                       lea rsi, [rdi + rax*4]
000000014084404b: 4c8b7d20                       mov r15, qword ptr [rbp + 0x20]
000000014084404f: 4c8b6528                       mov r12, qword ptr [rbp + 0x28]
0000000140844053: 0fb65f08                       movzx ebx, byte ptr [rdi + 8]
0000000140844057: 4883c70c                       add rdi, 0xc
000000014084405b: 889d30010000                   mov byte ptr [rbp + 0x130], bl
0000000140844061: 488d4d40                       lea rcx, [rbp + 0x40]
0000000140844065: 488d9530010000                 lea rdx, [rbp + 0x130]
000000014084406c: e8df35aa00                     call 0x1412e7650
0000000140844071: 4889c1                         mov rcx, rax
0000000140844074: 48c1e939                       shr rcx, 0x39
0000000140844078: 660f6ec1                       movd xmm0, ecx
000000014084407c: 660f60c0                       punpcklbw xmm0, xmm0
0000000140844080: f20f70c000                     pshuflw xmm0, xmm0, 0
0000000140844085: 660f70c000                     pshufd xmm0, xmm0, 0
000000014084408a: 31c9                           xor ecx, ecx
000000014084408c: 4c21e0                         and rax, r12
000000014084408f: f3410f6f0c07                   movdqu xmm1, xmmword ptr [r15 + rax]
0000000140844095: 660f6fd1                       movdqa xmm2, xmm1
0000000140844099: 660f74d0                       pcmpeqb xmm2, xmm0
000000014084409d: 660fd7d2                       pmovmskb edx, xmm2
00000001408440a1: 85d2                           test edx, edx
00000001408440a3: 742b                           je 0x1408440d0
00000001408440a5: f3440fbcc2                     tzcnt r8d, edx
00000001408440aa: 4901c0                         add r8, rax
00000001408440ad: 4d21e0                         and r8, r12
00000001408440b0: 49f7d0                         not r8
00000001408440b3: 433a1c38                       cmp bl, byte ptr [r8 + r15]
00000001408440b7: 7447                           je 0x140844100
00000001408440b9: 448d42ff                       lea r8d, [rdx - 1]
00000001408440bd: 664121d0                       and r8w, dx
00000001408440c1: 4489c2                         mov edx, r8d
00000001408440c4: 75df                           jne 0x1408440a5
00000001408440c6: 662e0f1f840000000000           nop word ptr cs:[rax + rax]
00000001408440d0: 660f74ce                       pcmpeqb xmm1, xmm6
00000001408440d4: 660fd7d1                       pmovmskb edx, xmm1
00000001408440d8: 85d2                           test edx, edx
00000001408440da: 7514                           jne 0x1408440f0
00000001408440dc: 4801c8                         add rax, rcx
00000001408440df: 4883c010                       add rax, 0x10
00000001408440e3: 4883c110                       add rcx, 0x10
00000001408440e7: eba3                           jmp 0x14084408c
00000001408440e9: 0f1f8000000000                 nop dword ptr [rax]
00000001408440f0: 4839f7                         cmp rdi, rsi
00000001408440f3: 0f855affffff                   jne 0x140844053
00000001408440f9: e95bfbffff                     jmp 0x140843c59
00000001408440fe: 6690                           nop 
0000000140844100: 8b85b4000000                   mov eax, dword ptr [rbp + 0xb4]
0000000140844106: 41894500                       mov dword ptr [r13], eax
000000014084410a: 8b85b8000000                   mov eax, dword ptr [rbp + 0xb8]
0000000140844110: 488b4df8                       mov rcx, qword ptr [rbp - 8]
0000000140844114: 8901                           mov dword ptr [rcx], eax
0000000140844116: 8b85bc000000                   mov eax, dword ptr [rbp + 0xbc]
000000014084411c: 488b4d08                       mov rcx, qword ptr [rbp + 8]
0000000140844120: 8901                           mov dword ptr [rcx], eax
0000000140844122: 8b8564010000                   mov eax, dword ptr [rbp + 0x164]
0000000140844128: 488b4d50                       mov rcx, qword ptr [rbp + 0x50]
000000014084412c: 8901                           mov dword ptr [rcx], eax
000000014084412e: 488b8d78010000                 mov rcx, qword ptr [rbp + 0x178]
0000000140844135: 488b55f0                       mov rdx, qword ptr [rbp - 0x10]
0000000140844139: 4c8b4500                       mov r8, qword ptr [rbp]
000000014084413d: 4c8b8d08010000                 mov r9, qword ptr [rbp + 0x108]
0000000140844144: e847f2ffff                     call 0x140843390
0000000140844149: 488b4d10                       mov rcx, qword ptr [rbp + 0x10]
000000014084414d: 488b9568010000                 mov rdx, qword ptr [rbp + 0x168]
0000000140844154: e817ecd000                     call 0x141552d70
0000000140844159: 4889c7                         mov rdi, rax
000000014084415c: 488b8d68010000                 mov rcx, qword ptr [rbp + 0x168]
0000000140844163: e8c81fd100                     call 0x141556130
0000000140844168: 4989d1                         mov r9, rdx
000000014084416b: 488b4d58                       mov rcx, qword ptr [rbp + 0x58]
000000014084416f: 48894c2428                     mov qword ptr [rsp + 0x28], rcx
0000000140844174: 488b8d88010000                 mov rcx, qword ptr [rbp + 0x188]
000000014084417b: 48894c2420                     mov qword ptr [rsp + 0x20], rcx
0000000140844180: 488d8d30010000                 lea rcx, [rbp + 0x130]
0000000140844187: 488b5518                       mov rdx, qword ptr [rbp + 0x18]
000000014084418b: 4989c0                         mov r8, rax
000000014084418e: e86dd9ffff                     call 0x140841b00
0000000140844193: 488bb578010000                 mov rsi, qword ptr [rbp + 0x178]
000000014084419a: 488b06                         mov rax, qword ptr [rsi]
000000014084419d: 4885c0                         test rax, rax
00000001408441a0: 7417                           je 0x1408441b9
00000001408441a2: 488b4e08                       mov rcx, qword ptr [rsi + 8]
00000001408441a6: 48c1e002                       shl rax, 2
00000001408441aa: 488d1440                       lea rdx, [rax + rax*2]
00000001408441ae: 41b804000000                   mov r8d, 4
00000001408441b4: e867fadcff                     call 0x140613c20
00000001408441b9: 488b8540010000                 mov rax, qword ptr [rbp + 0x140]
00000001408441c0: 48894610                       mov qword ptr [rsi + 0x10], rax
00000001408441c4: f30f6f8530010000               movdqu xmm0, xmmword ptr [rbp + 0x130]
00000001408441cc: f30f7f06                       movdqu xmmword ptr [rsi], xmm0
00000001408441d0: 4c69671098000000               imul r12, qword ptr [rdi + 0x10], 0x98
00000001408441d8: 488b7f08                       mov rdi, qword ptr [rdi + 8]
00000001408441dc: 4901fc                         add r12, rdi
00000001408441df: 488d4630                       lea rax, [rsi + 0x30]
00000001408441e3: 488985d0000000                 mov qword ptr [rbp + 0xd0], rax
00000001408441ea: 488d4618                       lea rax, [rsi + 0x18]
00000001408441ee: 488985c8000000                 mov qword ptr [rbp + 0xc8], rax
00000001408441f5: 4c39e7                         cmp rdi, r12
00000001408441f8: 0f845bfaffff                   je 0x140843c59
00000001408441fe: 6690                           nop 
0000000140844200: 488b8d68010000                 mov rcx, qword ptr [rbp + 0x168]
0000000140844207: e8241fd100                     call 0x141556130
000000014084420c: 488985a0000000                 mov qword ptr [rbp + 0xa0], rax
0000000140844213: 488995a8000000                 mov qword ptr [rbp + 0xa8], rdx
000000014084421a: 31c0                           xor eax, eax
000000014084421c: 483b4718                       cmp rax, qword ptr [rdi + 0x18]
0000000140844220: 0f80ed010000                   jo 0x140844413
0000000140844226: 48837f2804                     cmp qword ptr [rdi + 0x28], 4
000000014084422b: 0f85ef020000                   jne 0x140844520
0000000140844231: 488b4720                       mov rax, qword ptr [rdi + 0x20]
0000000140844235: 813854726565                   cmp dword ptr [rax], 0x65657254
000000014084423b: 0f85df020000                   jne 0x140844520
0000000140844241: 488d5f30                       lea rbx, [rdi + 0x30]
0000000140844245: 488b8588010000                 mov rax, qword ptr [rbp + 0x188]
000000014084424c: 660f6e400c                     movd xmm0, dword ptr [rax + 0xc]
0000000140844251: 660f7e85e8000000               movd dword ptr [rbp + 0xe8], xmm0
0000000140844259: 660f7e85ec000000               movd dword ptr [rbp + 0xec], xmm0
0000000140844261: 660f7e85f0000000               movd dword ptr [rbp + 0xf0], xmm0
0000000140844269: 488d8d30010000                 lea rcx, [rbp + 0x130]
0000000140844270: 4889da                         mov rdx, rbx
0000000140844273: 4c8d85e8000000                 lea r8, [rbp + 0xe8]
000000014084427a: e811c29f00                     call 0x141240490
000000014084427f: 488b8588010000                 mov rax, qword ptr [rbp + 0x188]
0000000140844286: 660f6e4008                     movd xmm0, dword ptr [rax + 8]
000000014084428b: f30f7e08                       movq xmm1, qword ptr [rax]
000000014084428f: 660fd64db8                     movq qword ptr [rbp - 0x48], xmm1
0000000140844294: 660f7e45c0                     movd dword ptr [rbp - 0x40], xmm0
0000000140844299: 488d8de8000000                 lea rcx, [rbp + 0xe8]
00000001408442a0: 488d9530010000                 lea rdx, [rbp + 0x130]
00000001408442a7: 4c8d45b8                       lea r8, [rbp - 0x48]
00000001408442ab: e810c29f00                     call 0x1412404c0
00000001408442b0: 488b4310                       mov rax, qword ptr [rbx + 0x10]
00000001408442b4: 48898540010000                 mov qword ptr [rbp + 0x140], rax
00000001408442bb: f30f6f03                       movdqu xmm0, xmmword ptr [rbx]
00000001408442bf: 660f7f8530010000               movdqa xmmword ptr [rbp + 0x130], xmm0
00000001408442c7: 8b8564010000                   mov eax, dword ptr [rbp + 0x164]
00000001408442cd: 488b4d50                       mov rcx, qword ptr [rbp + 0x50]
00000001408442d1: 8901                           mov dword ptr [rcx], eax
00000001408442d3: 4c8b8508010000                 mov r8, qword ptr [rbp + 0x108]
00000001408442da: 418b581c                       mov ebx, dword ptr [r8 + 0x1c]
00000001408442de: ffc3                           inc ebx
00000001408442e0: 83fbff                         cmp ebx, -1
00000001408442e3: 0f84be020000                   je 0x1408445a7
00000001408442e9: 498b7010                       mov rsi, qword ptr [r8 + 0x10]
00000001408442ed: 458b6818                       mov r13d, dword ptr [r8 + 0x18]
00000001408442f1: 4c39ee                         cmp rsi, r13
00000001408442f4: 764a                           jbe 0x140844340
00000001408442f6: 498b4008                       mov rax, qword ptr [r8 + 8]
00000001408442fa: 4a8d0ced00000000               lea rcx, [r13*8]
0000000140844302: 4c01e9                         add rcx, r13
0000000140844305: 488d0c49                       lea rcx, [rcx + rcx*2]
0000000140844309: 4c01e9                         add rcx, r13
000000014084430c: 8b1408                         mov edx, dword ptr [rax + rcx]
000000014084430f: 448b7c0818                     mov r15d, dword ptr [rax + rcx + 0x18]
0000000140844314: 4183cf01                       or r15d, 1
0000000140844318: 41895018                       mov dword ptr [r8 + 0x18], edx
000000014084431c: 488b95f8000000                 mov rdx, qword ptr [rbp + 0xf8]
0000000140844323: 4889540810                     mov qword ptr [rax + rcx + 0x10], rdx
0000000140844328: f30f6f85e8000000               movdqu xmm0, xmmword ptr [rbp + 0xe8]
0000000140844330: f30f7f0408                     movdqu xmmword ptr [rax + rcx], xmm0
0000000140844335: 44897c0818                     mov dword ptr [rax + rcx + 0x18], r15d
000000014084433a: eb6d                           jmp 0x1408443a9
000000014084433c: 0f1f4000                       nop dword ptr [rax]
0000000140844340: 493b30                         cmp rsi, qword ptr [r8]
0000000140844343: 7513                           jne 0x140844358
0000000140844345: 488b8d08010000                 mov rcx, qword ptr [rbp + 0x108]
000000014084434c: 488d151d954202                 lea rdx, [rip + 0x242951d]
0000000140844353: e82888f4ff                     call 0x14078cb80
0000000140844358: 4c8b8508010000                 mov r8, qword ptr [rbp + 0x108]
000000014084435f: 498b4008                       mov rax, qword ptr [r8 + 8]
0000000140844363: 488d0cf6                       lea rcx, [rsi + rsi*8]
0000000140844367: 488d0c49                       lea rcx, [rcx + rcx*2]
000000014084436b: 4801f1                         add rcx, rsi
000000014084436e: 488b95f8000000                 mov rdx, qword ptr [rbp + 0xf8]
0000000140844375: 4889540810                     mov qword ptr [rax + rcx + 0x10], rdx
000000014084437a: f30f6f85e8000000               movdqu xmm0, xmmword ptr [rbp + 0xe8]
0000000140844382: f30f7f0408                     movdqu xmmword ptr [rax + rcx], xmm0
0000000140844387: c744081801000000               mov dword ptr [rax + rcx + 0x18], 1
000000014084438f: 4889f0                         mov rax, rsi
0000000140844392: 48ffc0                         inc rax
0000000140844395: 49894010                       mov qword ptr [r8 + 0x10], rax
0000000140844399: 8d4601                         lea eax, [rsi + 1]
000000014084439c: 41894018                       mov dword ptr [r8 + 0x18], eax
00000001408443a0: 41bf01000000                   mov r15d, 1
00000001408443a6: 4189f5                         mov r13d, esi
00000001408443a9: 4189581c                       mov dword ptr [r8 + 0x1c], ebx
00000001408443ad: 488b8578010000                 mov rax, qword ptr [rbp + 0x178]
00000001408443b4: 488b7040                       mov rsi, qword ptr [rax + 0x40]
00000001408443b8: 483b7030                       cmp rsi, qword ptr [rax + 0x30]
00000001408443bc: 7513                           jne 0x1408443d1
00000001408443be: 488b8dd0000000                 mov rcx, qword ptr [rbp + 0xd0]
00000001408443c5: 488d15ecfe2402                 lea rdx, [rip + 0x224feec]
00000001408443cc: e8dfe6d1ff                     call 0x140562ab0
00000001408443d1: 4c8b8578010000                 mov r8, qword ptr [rbp + 0x178]
00000001408443d8: 498b4038                       mov rax, qword ptr [r8 + 0x38]
00000001408443dc: 488d0cb6                       lea rcx, [rsi + rsi*4]
00000001408443e0: 488b9540010000                 mov rdx, qword ptr [rbp + 0x140]
00000001408443e7: 488954c810                     mov qword ptr [rax + rcx*8 + 0x10], rdx
00000001408443ec: 660f6f8530010000               movdqa xmm0, xmmword ptr [rbp + 0x130]
00000001408443f4: f30f7f04c8                     movdqu xmmword ptr [rax + rcx*8], xmm0
00000001408443f9: 48c744c81800000000             mov qword ptr [rax + rcx*8 + 0x18], 0
0000000140844402: 44897cc820                     mov dword ptr [rax + rcx*8 + 0x20], r15d
0000000140844407: 44896cc824                     mov dword ptr [rax + rcx*8 + 0x24], r13d
000000014084440c: 48ffc6                         inc rsi
000000014084440f: 49897040                       mov qword ptr [r8 + 0x40], rsi
0000000140844413: 488b85c0000000                 mov rax, qword ptr [rbp + 0xc0]
000000014084441a: 48898550010000                 mov qword ptr [rbp + 0x150], rax
0000000140844421: 488b45d0                       mov rax, qword ptr [rbp - 0x30]
0000000140844425: 48898530010000                 mov qword ptr [rbp + 0x130], rax
000000014084442c: 488b45d8                       mov rax, qword ptr [rbp - 0x28]
0000000140844430: 48898538010000                 mov qword ptr [rbp + 0x138], rax
0000000140844437: 488b45e8                       mov rax, qword ptr [rbp - 0x18]
000000014084443b: 48898540010000                 mov qword ptr [rbp + 0x140], rax
0000000140844442: 488b45e0                       mov rax, qword ptr [rbp - 0x20]
0000000140844446: 48898548010000                 mov qword ptr [rbp + 0x148], rax
000000014084444d: 488b45c8                       mov rax, qword ptr [rbp - 0x38]
0000000140844451: 0fb65058                       movzx edx, byte ptr [rax + 0x58]
0000000140844455: 488d8530010000                 lea rax, [rbp + 0x130]
000000014084445c: 4889442428                     mov qword ptr [rsp + 0x28], rax
0000000140844461: 488b85c8000000                 mov rax, qword ptr [rbp + 0xc8]
0000000140844468: 4889442420                     mov qword ptr [rsp + 0x20], rax
000000014084446d: 4889f9                         mov rcx, rdi
0000000140844470: 4c8b8548020000                 mov r8, qword ptr [rbp + 0x248]
0000000140844477: 4c8b8d88010000                 mov r9, qword ptr [rbp + 0x188]
000000014084447e: e8cdd0ffff                     call 0x140841550
0000000140844483: 4881c798000000                 add rdi, 0x98
000000014084448a: 4c39e7                         cmp rdi, r12
000000014084448d: 0f856dfdffff                   jne 0x140844200
0000000140844493: e9c1f7ffff                     jmp 0x140843c59
0000000140844498: 48c1e002                       shl rax, 2
000000014084449c: 488d0440                       lea rax, [rax + rax*2]
00000001408444a0: 4883c0f4                       add rax, -0xc
00000001408444a4: 75fa                           jne 0x1408444a0
00000001408444a6: e9aef7ffff                     jmp 0x140843c59
00000001408444ab: 488b5528                       mov rdx, qword ptr [rbp + 0x28]
00000001408444af: 4885d2                         test rdx, rdx
00000001408444b2: 7426                           je 0x1408444da
00000001408444b4: 4889d0                         mov rax, rdx
00000001408444b7: 4883e0f0                       and rax, 0xfffffffffffffff0
00000001408444bb: 4801c2                         add rdx, rax
00000001408444be: 4883c221                       add rdx, 0x21
00000001408444c2: 7416                           je 0x1408444da
00000001408444c4: 488b4d20                       mov rcx, qword ptr [rbp + 0x20]
00000001408444c8: 4829c1                         sub rcx, rax
00000001408444cb: 4883c1f0                       add rcx, -0x10
00000001408444cf: 41b810000000                   mov r8d, 0x10
00000001408444d5: e846f7dcff                     call 0x140613c20
00000001408444da: 488d4d70                       lea rcx, [rbp + 0x70]
00000001408444de: 41b810000000                   mov r8d, 0x10
00000001408444e4: 41b910000000                   mov r9d, 0x10
00000001408444ea: 488d9590000000                 lea rdx, [rbp + 0x90]
00000001408444f1: e80afa3200                     call 0x140b73f00
00000001408444f6: 0f28b5a0010000                 movaps xmm6, xmmword ptr [rbp + 0x1a0]
00000001408444fd: 0f28bdb0010000                 movaps xmm7, xmmword ptr [rbp + 0x1b0]
0000000140844504: 440f2885c0010000               movaps xmm8, xmmword ptr [rbp + 0x1c0]
000000014084450c: 4881c458020000                 add rsp, 0x258
0000000140844513: 5b                             pop rbx
0000000140844514: 5f                             pop rdi
0000000140844515: 5e                             pop rsi
0000000140844516: 415c                           pop r12
0000000140844518: 415d                           pop r13
000000014084451a: 415e                           pop r14
000000014084451c: 415f                           pop r15
000000014084451e: 5d                             pop rbp
000000014084451f: c3                             ret 
0000000140844520: 4883c718                       add rdi, 0x18
0000000140844524: 4889bde8000000                 mov qword ptr [rbp + 0xe8], rdi
000000014084452b: 488d05be8e89ff                 lea rax, [rip - 0x767142]
0000000140844532: 488985f0000000                 mov qword ptr [rbp + 0xf0], rax
0000000140844539: 488d85a0000000                 lea rax, [rbp + 0xa0]
0000000140844540: 488985f8000000                 mov qword ptr [rbp + 0xf8], rax
0000000140844547: 488d057252d100                 lea rax, [rip + 0xd15272]
000000014084454e: 48898500010000                 mov qword ptr [rbp + 0x100], rax
0000000140844555: 488d0524fd2402                 lea rax, [rip + 0x224fd24]
000000014084455c: 48898530010000                 mov qword ptr [rbp + 0x130], rax
0000000140844563: 48c7853801000002000000         mov qword ptr [rbp + 0x138], 2
000000014084456e: 48c7855001000000000000         mov qword ptr [rbp + 0x150], 0
0000000140844579: 488d85e8000000                 lea rax, [rbp + 0xe8]
0000000140844580: 48898540010000                 mov qword ptr [rbp + 0x140], rax
0000000140844587: 48c7854801000002000000         mov qword ptr [rbp + 0x148], 2
0000000140844592: 488d1507fd2402                 lea rdx, [rip + 0x224fd07]
0000000140844599: 488d8d30010000                 lea rcx, [rbp + 0x130]
00000001408445a0: e8eb4d0902                     call 0x1428d9390
00000001408445a5: eb32                           jmp 0x1408445d9
00000001408445a7: 488d0dda924202                 lea rcx, [rip + 0x24292da]
00000001408445ae: 4c8d05fb924202                 lea r8, [rip + 0x24292fb]
00000001408445b5: ba23000000                     mov edx, 0x23
00000001408445ba: e866410902                     call 0x1428d8725
00000001408445bf: eb18                           jmp 0x1408445d9
00000001408445c1: 488d0dd8643f02                 lea rcx, [rip + 0x23f64d8]
00000001408445c8: e8033b0902                     call 0x1428d80d0
00000001408445cd: 488d0dcc643f02                 lea rcx, [rip + 0x23f64cc]
00000001408445d4: e8f73a0902                     call 0x1428d80d0
00000001408445d9: 0f0b                           ud2 
00000001408445db: 0f1f440000                     nop dword ptr [rax + rax]
00000001408445e0: 4889542410                     mov qword ptr [rsp + 0x10], rdx
00000001408445e5: 55                             push rbp
00000001408445e6: 4157                           push r15
00000001408445e8: 4156                           push r14
00000001408445ea: 4155                           push r13
00000001408445ec: 4154                           push r12
00000001408445ee: 56                             push rsi
00000001408445ef: 57                             push rdi
00000001408445f0: 53                             push rbx
00000001408445f1: 4883ec68                       sub rsp, 0x68
00000001408445f5: 488daa80000000                 lea rbp, [rdx + 0x80]
00000001408445fc: 66440f7f442430                 movdqa xmmword ptr [rsp + 0x30], xmm8
0000000140844603: 660f7f7c2440                   movdqa xmmword ptr [rsp + 0x40], xmm7
0000000140844609: 660f7f742450                   movdqa xmmword ptr [rsp + 0x50], xmm6
000000014084460f: 488d4d20                       lea rcx, [rbp + 0x20]
0000000140844613: e8e8740300                     call 0x14087bb00
0000000140844618: 0f28742450                     movaps xmm6, xmmword ptr [rsp + 0x50]
000000014084461d: 0f287c2440                     movaps xmm7, xmmword ptr [rsp + 0x40]
0000000140844622: 440f28442430                   movaps xmm8, xmmword ptr [rsp + 0x30]
0000000140844628: 4883c468                       add rsp, 0x68
000000014084462c: 5b                             pop rbx
000000014084462d: 5f                             pop rdi
000000014084462e: 5e                             pop rsi
000000014084462f: 415c                           pop r12
0000000140844631: 415d                           pop r13
0000000140844633: 415e                           pop r14
0000000140844635: 415f                           pop r15
0000000140844637: 5d                             pop rbp
0000000140844638: c3                             ret 
0000000140844639: 0f1f8000000000                 nop dword ptr [rax]
0000000140844640: 4889542410                     mov qword ptr [rsp + 0x10], rdx
0000000140844645: 55                             push rbp
0000000140844646: 4157                           push r15
0000000140844648: 4156                           push r14
000000014084464a: 4155                           push r13
000000014084464c: 4154                           push r12
000000014084464e: 56                             push rsi
000000014084464f: 57                             push rdi
0000000140844650: 53                             push rbx
0000000140844651: 4883ec68                       sub rsp, 0x68
0000000140844655: 488daa80000000                 lea rbp, [rdx + 0x80]
000000014084465c: 66440f7f442430                 movdqa xmmword ptr [rsp + 0x30], xmm8
0000000140844663: 660f7f7c2440                   movdqa xmmword ptr [rsp + 0x40], xmm7
0000000140844669: 660f7f742450                   movdqa xmmword ptr [rsp + 0x50], xmm6
000000014084466f: 488d4d70                       lea rcx, [rbp + 0x70]
0000000140844673: 41b810000000                   mov r8d, 0x10
0000000140844679: 41b910000000                   mov r9d, 0x10
000000014084467f: 488d9590000000                 lea rdx, [rbp + 0x90]
0000000140844686: e875f83200                     call 0x140b73f00
000000014084468b: 0f28742450                     movaps xmm6, xmmword ptr [rsp + 0x50]
0000000140844690: 0f287c2440                     movaps xmm7, xmmword ptr [rsp + 0x40]
0000000140844695: 440f28442430                   movaps xmm8, xmmword ptr [rsp + 0x30]
000000014084469b: 4883c468                       add rsp, 0x68
000000014084469f: 5b                             pop rbx
00000001408446a0: 5f                             pop rdi
00000001408446a1: 5e                             pop rsi
00000001408446a2: 415c                           pop r12
00000001408446a4: 415d                           pop r13
00000001408446a6: 415e                           pop r14
00000001408446a8: 415f                           pop r15
00000001408446aa: 5d                             pop rbp
00000001408446ab: c3                             ret 
