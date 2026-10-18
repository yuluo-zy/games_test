1414fbb50: 4157                     push r15
1414fbb52: 4156                     push r14
1414fbb54: 56                       push rsi
1414fbb55: 57                       push rdi
1414fbb56: 53                       push rbx
1414fbb57: 4881eca0000000           sub rsp, 0xa0
1414fbb5e: 0f29b42490000000         movaps xmmword ptr [rsp + 0x90], xmm6
1414fbb66: f60101                   test byte ptr [rcx], 1
1414fbb69: 0f84a0030000             je 0x1414fbf0f
1414fbb6f: 488b82a0050000           mov rax, qword ptr [rdx + 0x5a0]
1414fbb76: 4889442440               mov qword ptr [rsp + 0x40], rax
1414fbb7b: 48394128                 cmp qword ptr [rcx + 0x28], rax
1414fbb7f: 0f85e2010000             jne 0x1414fbd67
1414fbb85: 488b8208010000           mov rax, qword ptr [rdx + 0x108]
1414fbb8c: 448b8968010000           mov r9d, dword ptr [rcx + 0x168]
1414fbb93: 898168010000             mov dword ptr [rcx + 0x168], eax
1414fbb99: 4c39c8                   cmp rax, r9
1414fbb9c: 0f820f020000             jb 0x1414fbdb1
1414fbba2: be01000000               mov esi, 1
1414fbba7: f00fc1b2b0050000         lock xadd dword ptr [rdx + 0x5b0], esi
1414fbbaf: 488b4108                 mov rax, qword ptr [rcx + 8]
1414fbbb3: 4c8b8218020000           mov r8, qword ptr [rdx + 0x218]
1414fbbba: 4939c0                   cmp r8, rax
1414fbbbd: 0f8600020000             jbe 0x1414fbdc3
1414fbbc3: 4c8b9210020000           mov r10, qword ptr [rdx + 0x210]
1414fbbca: 4d8b0cc2                 mov r9, qword ptr [r10 + rax*8]
1414fbbce: 4d85c9                   test r9, r9
1414fbbd1: 0f84ec010000             je 0x1414fbdc3
1414fbbd7: 488b82e0010000           mov rax, qword ptr [rdx + 0x1e0]
1414fbbde: 49f7d1                   not r9
1414fbbe1: 4f8d0c49                 lea r9, [r9 + r9*2]
1414fbbe5: 49c1e105                 shl r9, 5
1414fbbe9: 4a837c082000             cmp qword ptr [rax + r9 + 0x20], 0
1414fbbef: 0f84ce010000             je 0x1414fbdc3
1414fbbf5: 4c8b5910                 mov r11, qword ptr [rcx + 0x10]
1414fbbf9: 4d39d8                   cmp r8, r11
1414fbbfc: 0f86df010000             jbe 0x1414fbde1
1414fbc02: 4f8b1cda                 mov r11, qword ptr [r10 + r11*8]
1414fbc06: 4d85db                   test r11, r11
1414fbc09: 0f84d2010000             je 0x1414fbde1
1414fbc0f: 49f7d3                   not r11
1414fbc12: 4f8d1c5b                 lea r11, [r11 + r11*2]
1414fbc16: 49c1e305                 shl r11, 5
1414fbc1a: 4a837c182000             cmp qword ptr [rax + r11 + 0x20], 0
1414fbc20: 0f84bb010000             je 0x1414fbde1
1414fbc26: 488b7918                 mov rdi, qword ptr [rcx + 0x18]
1414fbc2a: 4939f8                   cmp r8, rdi
1414fbc2d: 0f8636020000             jbe 0x1414fbe69
1414fbc33: 4d8b04fa                 mov r8, qword ptr [r10 + rdi*8]
1414fbc37: 4d85c0                   test r8, r8
1414fbc3a: 0f8429020000             je 0x1414fbe69
1414fbc40: 49f7d0                   not r8
1414fbc43: 4f8d0440                 lea r8, [r8 + r8*2]
1414fbc47: 49c1e005                 shl r8, 5
1414fbc4b: 4a837c002000             cmp qword ptr [rax + r8 + 0x20], 0
1414fbc51: 0f8412020000             je 0x1414fbe69
1414fbc57: 4901c1                   add r9, rax
1414fbc5a: 4d8b4910                 mov r9, qword ptr [r9 + 0x10]
1414fbc5e: 4901c3                   add r11, rax
1414fbc61: 4d8b5b10                 mov r11, qword ptr [r11 + 0x10]
1414fbc65: 4901c0                   add r8, rax
1414fbc68: 4d8b5010                 mov r10, qword ptr [r8 + 0x10]
1414fbc6c: 8b82b0050000             mov eax, dword ptr [rdx + 0x5b0]
1414fbc72: 488b4120                 mov rax, qword ptr [rcx + 0x20]
1414fbc76: 48398218020000           cmp qword ptr [rdx + 0x218], rax
1414fbc7d: 0f8604020000             jbe 0x1414fbe87
1414fbc83: 488bba10020000           mov rdi, qword ptr [rdx + 0x210]
1414fbc8a: 488b3cc7                 mov rdi, qword ptr [rdi + rax*8]
1414fbc8e: 4885ff                   test rdi, rdi
1414fbc91: 0f84f0010000             je 0x1414fbe87
1414fbc97: 488b82e0010000           mov rax, qword ptr [rdx + 0x1e0]
1414fbc9e: 48f7d7                   not rdi
1414fbca1: 488d3c7f                 lea rdi, [rdi + rdi*2]
1414fbca5: 48c1e705                 shl rdi, 5
1414fbca9: 48837c382000             cmp qword ptr [rax + rdi + 0x20], 0
1414fbcaf: 0f84d2010000             je 0x1414fbe87
1414fbcb5: 4801f8                   add rax, rdi
1414fbcb8: 488b7810                 mov rdi, qword ptr [rax + 0x10]
1414fbcbc: 8b92b0050000             mov edx, dword ptr [rdx + 0x5b0]
1414fbcc2: 41f60301                 test byte ptr [r11], 1
1414fbcc6: 7519                     jne 0x1414fbce1
1414fbcc8: f2410f108148010000       movsd xmm0, qword ptr [r9 + 0x148]
1414fbcd1: 4189705c                 mov dword ptr [r8 + 0x5c], esi
1414fbcd5: f2410f100a               movsd xmm1, qword ptr [r10]
1414fbcda: 0f58c8                   addps xmm1, xmm0
1414fbcdd: 410f130a                 movlps qword ptr [r10], xmm1
1414fbce1: f2410f10b140010000       movsd xmm6, qword ptr [r9 + 0x140]
1414fbcea: 0f57c0                   xorps xmm0, xmm0
1414fbced: 0f2ef0                   ucomiss xmm6, xmm0
1414fbcf0: 750d                     jne 0x1414fbcff
1414fbcf2: 7a0b                     jp 0x1414fbcff
1414fbcf4: f30f16ce                 movshdup xmm1, xmm6
1414fbcf8: 0f2ec8                   ucomiss xmm1, xmm0
1414fbcfb: 7502                     jne 0x1414fbcff
1414fbcfd: 7b49                     jnp 0x1414fbd48
1414fbcff: 89705c                   mov dword ptr [rax + 0x5c], esi
1414fbd02: 488b5f30                 mov rbx, qword ptr [rdi + 0x30]
1414fbd06: 4c8b7740                 mov r14, qword ptr [rdi + 0x40]
1414fbd0a: 483b5f20                 cmp rbx, qword ptr [rdi + 0x20]
1414fbd0e: 7519                     jne 0x1414fbd29
1414fbd10: 488d4720                 lea rax, [rdi + 0x20]
1414fbd14: 488d152de47301           lea rdx, [rip + 0x173e42d]
1414fbd1b: 4989cf                   mov r15, rcx
1414fbd1e: 4889c1                   mov rcx, rax
1414fbd21: e80a6906ff               call 0x140562630
1414fbd26: 4c89f9                   mov rcx, r15
1414fbd29: 488b4728                 mov rax, qword ptr [rdi + 0x28]
1414fbd2d: 4889da                   mov rdx, rbx
1414fbd30: 48c1e204                 shl rdx, 4
1414fbd34: 4c893410                 mov qword ptr [rax + rdx], r14
1414fbd38: 0f13741008               movlps qword ptr [rax + rdx + 8], xmm6
1414fbd3d: 48ffc3                   inc rbx
1414fbd40: 48895f30                 mov qword ptr [rdi + 0x30], rbx
1414fbd44: 48ff4740                 inc qword ptr [rdi + 0x40]
1414fbd48: 89b160010000             mov dword ptr [rcx + 0x160], esi
1414fbd4e: 31c0                     xor eax, eax
1414fbd50: 0f28b42490000000         movaps xmm6, xmmword ptr [rsp + 0x90]
1414fbd58: 4881c4a0000000           add rsp, 0xa0
1414fbd5f: 5b                       pop rbx
1414fbd60: 5f                       pop rdi
1414fbd61: 5e                       pop rsi
1414fbd62: 415e                     pop r14
1414fbd64: 415f                     pop r15
1414fbd66: c3                       ret 
1414fbd67: 488d05f2bb7501           lea rax, [rip + 0x175bbf2]
1414fbd6e: 4889442460               mov qword ptr [rsp + 0x60], rax
1414fbd73: 48c744246801000000       mov qword ptr [rsp + 0x68], 1
1414fbd7c: 48c744247008000000       mov qword ptr [rsp + 0x70], 8
1414fbd85: 0f57c0                   xorps xmm0, xmm0
1414fbd88: 0f11442478               movups xmmword ptr [rsp + 0x78], xmm0
1414fbd8d: 4883c128                 add rcx, 0x28
1414fbd91: 488d05d8bb7501           lea rax, [rip + 0x175bbd8]
1414fbd98: 4889442420               mov qword ptr [rsp + 0x20], rax
1414fbd9d: 4c8d442440               lea r8, [rsp + 0x40]
1414fbda2: 4c8d4c2460               lea r9, [rsp + 0x60]
1414fbda7: 4889ca                   mov rdx, rcx
1414fbdaa: 31c9                     xor ecx, ecx
1414fbdac: e830233d01               call 0x1428ce0e1
1414fbdb1: 4c8d05d0bb7501           lea r8, [rip + 0x175bbd0]
1414fbdb8: 4c89c9                   mov rcx, r9
1414fbdbb: 4889c2                   mov rdx, rax
1414fbdbe: e86dda3d01               call 0x1428d9830
1414fbdc3: 4881c1c8000000           add rcx, 0xc8
1414fbdca: 488d059ecd7601           lea rax, [rip + 0x176cd9e]
1414fbdd1: 4889442430               mov qword ptr [rsp + 0x30], rax
1414fbdd6: 48c744243830000000       mov qword ptr [rsp + 0x38], 0x30
1414fbddf: eb1c                     jmp 0x1414fbdfd
1414fbde1: 4881c1c8000000           add rcx, 0xc8
1414fbde8: 488d05b8a27601           lea rax, [rip + 0x176a2b8]
1414fbdef: 4889442430               mov qword ptr [rsp + 0x30], rax
1414fbdf4: 48c74424383e000000       mov qword ptr [rsp + 0x38], 0x3e
1414fbdfd: 48894c2440               mov qword ptr [rsp + 0x40], rcx
1414fbe02: 488d05c7fcb3fe           lea rax, [rip - 0x14c0339]
1414fbe09: 4889442448               mov qword ptr [rsp + 0x48], rax
1414fbe0e: 488d442430               lea rax, [rsp + 0x30]
1414fbe13: 4889442450               mov qword ptr [rsp + 0x50], rax
1414fbe18: 488d0571e0c0fe           lea rax, [rip - 0x13f1f8f]
1414fbe1f: 4889442458               mov qword ptr [rsp + 0x58], rax
1414fbe24: 488d0555a07601           lea rax, [rip + 0x176a055]
1414fbe2b: 4889442460               mov qword ptr [rsp + 0x60], rax
1414fbe30: 48c744246802000000       mov qword ptr [rsp + 0x68], 2
1414fbe39: 48c784248000000000000000 mov qword ptr [rsp + 0x80], 0
1414fbe45: 488d442440               lea rax, [rsp + 0x40]
1414fbe4a: 4889442470               mov qword ptr [rsp + 0x70], rax
