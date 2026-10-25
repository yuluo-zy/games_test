14173d9e0: 4157                     push r15
14173d9e2: 4156                     push r14
14173d9e4: 56                       push rsi
14173d9e5: 57                       push rdi
14173d9e6: 53                       push rbx
14173d9e7: 4881ec90000000           sub rsp, 0x90
14173d9ee: 0f29b42480000000         movaps xmmword ptr [rsp + 0x80], xmm6
14173d9f6: be01000000               mov esi, 1
14173d9fb: f00fc1b2b0050000         lock xadd dword ptr [rdx + 0x5b0], esi
14173da03: f60101                   test byte ptr [rcx], 1
14173da06: 0f84fe020000             je 0x14173dd0a
14173da0c: 488b4108                 mov rax, qword ptr [rcx + 8]
14173da10: 4c8b8218020000           mov r8, qword ptr [rdx + 0x218]
14173da17: 4939c0                   cmp r8, rax
14173da1a: 0f86a4010000             jbe 0x14173dbc4
14173da20: 4c8b9210020000           mov r10, qword ptr [rdx + 0x210]
14173da27: 4d8b0cc2                 mov r9, qword ptr [r10 + rax*8]
14173da2b: 4d85c9                   test r9, r9
14173da2e: 0f8490010000             je 0x14173dbc4
14173da34: 488b82e0010000           mov rax, qword ptr [rdx + 0x1e0]
14173da3b: 49f7d1                   not r9
14173da3e: 4f8d0c49                 lea r9, [r9 + r9*2]
14173da42: 49c1e105                 shl r9, 5
14173da46: 4a837c082000             cmp qword ptr [rax + r9 + 0x20], 0
14173da4c: 0f8472010000             je 0x14173dbc4
14173da52: 4c8b5910                 mov r11, qword ptr [rcx + 0x10]
14173da56: 4d39d8                   cmp r8, r11
14173da59: 0f8683010000             jbe 0x14173dbe2
14173da5f: 4f8b1cda                 mov r11, qword ptr [r10 + r11*8]
14173da63: 4d85db                   test r11, r11
14173da66: 0f8476010000             je 0x14173dbe2
14173da6c: 49f7d3                   not r11
14173da6f: 4f8d1c5b                 lea r11, [r11 + r11*2]
14173da73: 49c1e305                 shl r11, 5
14173da77: 4a837c182000             cmp qword ptr [rax + r11 + 0x20], 0
14173da7d: 0f845f010000             je 0x14173dbe2
14173da83: 488b7918                 mov rdi, qword ptr [rcx + 0x18]
14173da87: 4939f8                   cmp r8, rdi
14173da8a: 0f86d7010000             jbe 0x14173dc67
14173da90: 4d8b04fa                 mov r8, qword ptr [r10 + rdi*8]
14173da94: 4d85c0                   test r8, r8
14173da97: 0f84ca010000             je 0x14173dc67
14173da9d: 49f7d0                   not r8
14173daa0: 4f8d0440                 lea r8, [r8 + r8*2]
14173daa4: 49c1e005                 shl r8, 5
14173daa8: 4a837c002000             cmp qword ptr [rax + r8 + 0x20], 0
14173daae: 0f84b3010000             je 0x14173dc67
14173dab4: 4901c1                   add r9, rax
14173dab7: 4d8b4910                 mov r9, qword ptr [r9 + 0x10]
14173dabb: 4901c3                   add r11, rax
14173dabe: 4d8b5b10                 mov r11, qword ptr [r11 + 0x10]
14173dac2: 4901c0                   add r8, rax
14173dac5: 4d8b5010                 mov r10, qword ptr [r8 + 0x10]
14173dac9: 8b82b0050000             mov eax, dword ptr [rdx + 0x5b0]
14173dacf: 488b4120                 mov rax, qword ptr [rcx + 0x20]
14173dad3: 48398218020000           cmp qword ptr [rdx + 0x218], rax
14173dada: 0f86a5010000             jbe 0x14173dc85
14173dae0: 488bba10020000           mov rdi, qword ptr [rdx + 0x210]
14173dae7: 488b3cc7                 mov rdi, qword ptr [rdi + rax*8]
14173daeb: 4885ff                   test rdi, rdi
14173daee: 0f8491010000             je 0x14173dc85
14173daf4: 488b82e0010000           mov rax, qword ptr [rdx + 0x1e0]
14173dafb: 48f7d7                   not rdi
14173dafe: 488d3c7f                 lea rdi, [rdi + rdi*2]
14173db02: 48c1e705                 shl rdi, 5
14173db06: 48837c382000             cmp qword ptr [rax + rdi + 0x20], 0
14173db0c: 0f8473010000             je 0x14173dc85
14173db12: 4801f8                   add rax, rdi
14173db15: 488b7810                 mov rdi, qword ptr [rax + 0x10]
14173db19: 8b92b0050000             mov edx, dword ptr [rdx + 0x5b0]
14173db1f: 41f60301                 test byte ptr [r11], 1
14173db23: 7519                     jne 0x14173db3e
14173db25: f2410f108148010000       movsd xmm0, qword ptr [r9 + 0x148]
14173db2e: 4189705c                 mov dword ptr [r8 + 0x5c], esi
14173db32: f2410f100a               movsd xmm1, qword ptr [r10]
14173db37: 0f58c8                   addps xmm1, xmm0
14173db3a: 410f130a                 movlps qword ptr [r10], xmm1
14173db3e: f2410f10b140010000       movsd xmm6, qword ptr [r9 + 0x140]
14173db47: 0f57c0                   xorps xmm0, xmm0
14173db4a: 0f2ef0                   ucomiss xmm6, xmm0
14173db4d: 750d                     jne 0x14173db5c
14173db4f: 7a0b                     jp 0x14173db5c
14173db51: f30f16ce                 movshdup xmm1, xmm6
14173db55: 0f2ec8                   ucomiss xmm1, xmm0
14173db58: 7502                     jne 0x14173db5c
14173db5a: 7b49                     jnp 0x14173dba5
14173db5c: 89705c                   mov dword ptr [rax + 0x5c], esi
14173db5f: 488b5f30                 mov rbx, qword ptr [rdi + 0x30]
14173db63: 4c8b7740                 mov r14, qword ptr [rdi + 0x40]
14173db67: 483b5f20                 cmp rbx, qword ptr [rdi + 0x20]
14173db6b: 7519                     jne 0x14173db86
14173db6d: 488d4720                 lea rax, [rdi + 0x20]
14173db71: 488d15d0c54f01           lea rdx, [rip + 0x14fc5d0]
14173db78: 4989cf                   mov r15, rcx
14173db7b: 4889c1                   mov rcx, rax
14173db7e: e8ad4ae2fe               call 0x140562630
14173db83: 4c89f9                   mov rcx, r15
14173db86: 488b4728                 mov rax, qword ptr [rdi + 0x28]
14173db8a: 4889da                   mov rdx, rbx
14173db8d: 48c1e204                 shl rdx, 4
14173db91: 4c893410                 mov qword ptr [rax + rdx], r14
14173db95: 0f13741008               movlps qword ptr [rax + rdx + 8], xmm6
14173db9a: 48ffc3                   inc rbx
14173db9d: 48895f30                 mov qword ptr [rdi + 0x30], rbx
14173dba1: 48ff4740                 inc qword ptr [rdi + 0x40]
14173dba5: 89b160010000             mov dword ptr [rcx + 0x160], esi
14173dbab: 31c0                     xor eax, eax
14173dbad: 0f28b42480000000         movaps xmm6, xmmword ptr [rsp + 0x80]
14173dbb5: 4881c490000000           add rsp, 0x90
14173dbbc: 5b                       pop rbx
14173dbbd: 5f                       pop rdi
14173dbbe: 5e                       pop rsi
14173dbbf: 415e                     pop r14
14173dbc1: 415f                     pop r15
14173dbc3: c3                       ret 
14173dbc4: 4881c1c8000000           add rcx, 0xc8
14173dbcb: 488d05a3165901           lea rax, [rip + 0x15916a3]
14173dbd2: 4889442420               mov qword ptr [rsp + 0x20], rax
14173dbd7: 48c744242830000000       mov qword ptr [rsp + 0x28], 0x30
14173dbe0: eb1c                     jmp 0x14173dbfe
14173dbe2: 4881c1c8000000           add rcx, 0xc8
14173dbe9: 488d057a135901           lea rax, [rip + 0x159137a]
14173dbf0: 4889442420               mov qword ptr [rsp + 0x20], rax
14173dbf5: 48c74424283e000000       mov qword ptr [rsp + 0x28], 0x3e
14173dbfe: 48894c2430               mov qword ptr [rsp + 0x30], rcx
14173dc03: 488d05c6de8ffe           lea rax, [rip - 0x170213a]
14173dc0a: 4889442438               mov qword ptr [rsp + 0x38], rax
14173dc0f: 488d442420               lea rax, [rsp + 0x20]
14173dc14: 4889442440               mov qword ptr [rsp + 0x40], rax
14173dc19: 488d0570c29cfe           lea rax, [rip - 0x1633d90]
14173dc20: 4889442448               mov qword ptr [rsp + 0x48], rax
14173dc25: 488d05a4135901           lea rax, [rip + 0x15913a4]
14173dc2c: 4889442450               mov qword ptr [rsp + 0x50], rax
14173dc31: 48c744245802000000       mov qword ptr [rsp + 0x58], 2
14173dc3a: 48c744247000000000       mov qword ptr [rsp + 0x70], 0
14173dc43: 488d442430               lea rax, [rsp + 0x30]
14173dc48: 4889442460               mov qword ptr [rsp + 0x60], rax
14173dc4d: 48c744246802000000       mov qword ptr [rsp + 0x68], 2
14173dc56: 488d15fb135901           lea rdx, [rip + 0x15913fb]
14173dc5d: 488d4c2450               lea rcx, [rsp + 0x50]
14173dc62: e829b71901               call 0x1428d9390
14173dc67: 4881c1c8000000           add rcx, 0xc8
14173dc6e: 488d053d185901           lea rax, [rip + 0x159183d]
14173dc75: 4889442420               mov qword ptr [rsp + 0x20], rax
14173dc7a: 48c744242840000000       mov qword ptr [rsp + 0x28], 0x40
14173dc83: eb1c                     jmp 0x14173dca1
14173dc85: 4881c1c8000000           add rcx, 0xc8
14173dc8c: 488d055f185901           lea rax, [rip + 0x159185f]
14173dc93: 4889442420               mov qword ptr [rsp + 0x20], rax
14173dc98: 48c74424285d000000       mov qword ptr [rsp + 0x28], 0x5d
14173dca1: 48894c2430               mov qword ptr [rsp + 0x30], rcx
14173dca6: 488d0523de8ffe           lea rax, [rip - 0x17021dd]
14173dcad: 4889442438               mov qword ptr [rsp + 0x38], rax
14173dcb2: 488d442420               lea rax, [rsp + 0x20]
14173dcb7: 4889442440               mov qword ptr [rsp + 0x40], rax
14173dcbc: 488d05cdc19cfe           lea rax, [rip - 0x1633e33]
14173dcc3: 4889442448               mov qword ptr [rsp + 0x48], rax
14173dcc8: 488d0501135901           lea rax, [rip + 0x1591301]
14173dccf: 4889442450               mov qword ptr [rsp + 0x50], rax
14173dcd4: 48c744245802000000       mov qword ptr [rsp + 0x58], 2
