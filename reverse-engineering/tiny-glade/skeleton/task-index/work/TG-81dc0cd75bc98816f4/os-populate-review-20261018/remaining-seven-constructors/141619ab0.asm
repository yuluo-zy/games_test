141619ab0: push rbp
141619ab1: push r15
141619ab3: push r14
141619ab5: push r12
141619ab7: push rsi
141619ab8: push rdi
141619ab9: push rbx
141619aba: sub rsp, 0x100
141619ac1: lea rbp, [rsp + 0x80]
141619ac9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141619ad1: mov rsi, rcx
141619ad4: test r8, r8
141619ad7: sete al
141619ada: cmp r9, 0x7e91
141619ae1: setb cl
141619ae4: or cl, al
141619ae6: je 0x141619b83
141619aec: mov r14, r9
141619aef: mov rbx, r8
141619af2: mov rdi, rdx
141619af5: lea r15, [rbp + 0x10]
141619af9: mov edx, 0x1fa400
141619afe: mov rcx, r15
141619b01: mov r8d, 0x22
141619b07: mov r9b, 1
141619b0a: call 0x140bd4de0
141619b0f: lea r12, [rbp + 0x68]
141619b13: mov r8d, 0x10
141619b19: mov rcx, r12
141619b1c: mov rdx, r15
141619b1f: call 0x140bd4f20
141619b24: shl r14, 6
141619b28: mov qword ptr [rsp + 0x30], r14
141619b2d: mov qword ptr [rsp + 0x28], rbx
141619b32: mov qword ptr [rsp + 0x20], r12
141619b37: lea r8, [rip + 0x166640d]
141619b3e: lea rcx, [rbp - 0x40]
141619b42: mov r9d, 0xc
141619b48: mov rdx, rdi
141619b4b: call 0x1416a2dc0
141619b50: cmp dword ptr [rbp - 0x40], 0xb
141619b54: jne 0x141619be4
141619b5a: mov rax, qword ptr [rbp - 0x18]
141619b5e: mov qword ptr [rsi + 0x20], rax
141619b62: movups xmm0, xmmword ptr [rbp - 0x38]
141619b66: movups xmm1, xmmword ptr [rbp - 0x28]
141619b6a: movups xmmword ptr [rsi + 0x10], xmm1
141619b6e: movups xmmword ptr [rsi], xmm0
141619b71: add rsp, 0x100
141619b78: pop rbx
141619b79: pop rdi
141619b7a: pop rsi
141619b7b: pop r12
141619b7d: pop r14
141619b7f: pop r15
141619b81: pop rbp
141619b82: ret 
141619b83: lea rax, [rip + 0x1484e73]
141619b8a: mov qword ptr [rbp - 0x40], rax
141619b8e: mov qword ptr [rbp - 0x38], 0x53
141619b96: lea rax, [rbp - 0x40]
141619b9a: mov qword ptr [rbp + 0x68], rax
141619b9e: lea rax, [rip - 0x150fd15]
141619ba5: mov qword ptr [rbp + 0x70], rax
141619ba9: lea rax, [rip + 0x1665c18]
141619bb0: mov qword ptr [rbp + 0x10], rax
141619bb4: mov qword ptr [rbp + 0x18], 1
141619bbc: mov qword ptr [rbp + 0x30], 0
141619bc4: lea rax, [rbp + 0x68]
141619bc8: mov qword ptr [rbp + 0x20], rax
141619bcc: mov qword ptr [rbp + 0x28], 1
141619bd4: lea rdx, [rip + 0x1665bfd]
141619bdb: lea rcx, [rbp + 0x10]
141619bdf: call 0x1428d9390
141619be4: movups xmm0, xmmword ptr [rbp]
141619be8: movaps xmmword ptr [rbp + 0x50], xmm0
141619bec: movups xmm0, xmmword ptr [rbp - 0x40]
141619bf0: movups xmm1, xmmword ptr [rbp - 0x30]
141619bf4: movups xmm2, xmmword ptr [rbp - 0x20]
141619bf8: movups xmm3, xmmword ptr [rbp - 0x10]
141619bfc: movaps xmmword ptr [rbp + 0x40], xmm3
141619c00: movaps xmmword ptr [rbp + 0x30], xmm2
141619c04: movaps xmmword ptr [rbp + 0x20], xmm1
141619c08: movaps xmmword ptr [rbp + 0x10], xmm0
141619c0c: lea rax, [rip + 0x1666595]
141619c13: mov qword ptr [rsp + 0x20], rax
141619c18: lea rcx, [rip + 0x1664ec9]
141619c1f: lea r9, [rip + 0x1664ea2]
141619c26: lea r8, [rbp + 0x10]
141619c2a: mov edx, 0x2b
141619c2f: call 0x1428d9760
141619c34: ud2 
141619c36: nop word ptr cs:[rax + rax]
141619c40: mov qword ptr [rsp + 0x10], rdx
141619c45: push rbp
141619c46: push r15
141619c48: push r14
141619c4a: push r12
141619c4c: push rsi
141619c4d: push rdi
141619c4e: push rbx
141619c4f: sub rsp, 0x40
141619c53: lea rbp, [rdx + 0x80]
141619c5a: lea rcx, [rbp + 0x10]
141619c5e: call 0x1407cea70
141619c63: nop 
141619c64: add rsp, 0x40
141619c68: pop rbx
141619c69: pop rdi
141619c6a: pop rsi
141619c6b: pop r12
141619c6d: pop r14
141619c6f: pop r15
141619c71: pop rbp
141619c72: ret 
