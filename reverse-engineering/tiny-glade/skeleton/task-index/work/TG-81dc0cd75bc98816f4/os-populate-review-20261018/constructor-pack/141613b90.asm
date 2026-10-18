141613b90: push rbp
141613b91: push r15
141613b93: push r14
141613b95: push r12
141613b97: push rsi
141613b98: push rdi
141613b99: push rbx
141613b9a: sub rsp, 0x100
141613ba1: lea rbp, [rsp + 0x80]
141613ba9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141613bb1: mov rbx, r8
141613bb4: mov rdi, rdx
141613bb7: mov rsi, rcx
141613bba: test r8, r8
141613bbd: je 0x141613c52
141613bc3: mov r14, r9
141613bc6: mov rax, r9
141613bc9: and rax, 0x3f
141613bcd: mov qword ptr [rbp - 0x40], rax
141613bd1: jne 0x141613cde
141613bd7: mov rax, rbx
141613bda: and rax, 3
141613bde: mov qword ptr [rbp - 0x40], rax
141613be2: jne 0x141613cef
141613be8: cmp r14, 0x33e040
141613bef: jb 0x141613c55
141613bf1: lea rax, [rip + 0x148b01b]
141613bf8: mov qword ptr [rbp - 0x40], rax
141613bfc: mov qword ptr [rbp - 0x38], 0x4e
141613c04: lea rax, [rbp - 0x40]
141613c08: mov qword ptr [rbp + 0x68], rax
141613c0c: lea rax, [rip - 0x1509d83]
141613c13: mov qword ptr [rbp + 0x70], rax
141613c17: lea rax, [rip + 0x166bbaa]
141613c1e: mov qword ptr [rbp + 0x10], rax
141613c22: mov qword ptr [rbp + 0x18], 1
141613c2a: mov qword ptr [rbp + 0x30], 0
141613c32: lea rax, [rbp + 0x68]
141613c36: mov qword ptr [rbp + 0x20], rax
141613c3a: mov qword ptr [rbp + 0x28], 1
141613c42: lea rdx, [rip + 0x166bb8f]
141613c49: lea rcx, [rbp + 0x10]
141613c4d: call 0x1428d9390
141613c52: xor r14d, r14d
141613c55: lea r15, [rbp + 0x10]
141613c59: mov edx, 0x33e000
141613c5e: mov rcx, r15
141613c61: mov r8d, 0x22
141613c67: mov r9b, 1
141613c6a: call 0x140bd4de0
141613c6f: lea r12, [rbp + 0x68]
141613c73: mov r8d, 4
141613c79: mov rcx, r12
141613c7c: mov rdx, r15
141613c7f: call 0x140bd4f20
141613c84: mov qword ptr [rsp + 0x30], r14
141613c89: mov qword ptr [rsp + 0x28], rbx
141613c8e: mov qword ptr [rsp + 0x20], r12
141613c93: lea r8, [rip + 0x166c2bd]
141613c9a: lea rcx, [rbp - 0x40]
141613c9e: mov r9d, 0xa
141613ca4: mov rdx, rdi
141613ca7: call 0x1416a2dc0
141613cac: cmp dword ptr [rbp - 0x40], 0xb
141613cb0: jne 0x141613d19
141613cb2: mov rax, qword ptr [rbp - 0x18]
141613cb6: mov qword ptr [rsi + 0x20], rax
141613cba: movups xmm0, xmmword ptr [rbp - 0x38]
141613cbe: movups xmm1, xmmword ptr [rbp - 0x28]
141613cc2: movups xmmword ptr [rsi + 0x10], xmm1
141613cc6: movups xmmword ptr [rsi], xmm0
141613cc9: mov rax, rsi
141613ccc: add rsp, 0x100
141613cd3: pop rbx
141613cd4: pop rdi
141613cd5: pop rsi
141613cd6: pop r12
141613cd8: pop r14
141613cda: pop r15
141613cdc: pop rbp
141613cdd: ret 
141613cde: mov qword ptr [rbp + 0x10], 0
141613ce6: lea rax, [rip + 0x16b625b]
141613ced: jmp 0x141613cfe
141613cef: mov qword ptr [rbp + 0x10], 0
141613cf7: lea rax, [rip + 0x16b6262]
141613cfe: mov qword ptr [rsp + 0x20], rax
141613d03: lea r8, [rip + 0x16b6236]
141613d0a: lea rdx, [rbp - 0x40]
141613d0e: lea r9, [rbp + 0x10]
141613d12: xor ecx, ecx
141613d14: call 0x1428d9596
141613d19: movups xmm0, xmmword ptr [rbp]
141613d1d: movaps xmmword ptr [rbp + 0x50], xmm0
141613d21: movups xmm0, xmmword ptr [rbp - 0x40]
141613d25: movups xmm1, xmmword ptr [rbp - 0x30]
141613d29: movups xmm2, xmmword ptr [rbp - 0x20]
141613d2d: movups xmm3, xmmword ptr [rbp - 0x10]
141613d31: movaps xmmword ptr [rbp + 0x40], xmm3
141613d35: movaps xmmword ptr [rbp + 0x30], xmm2
141613d39: movaps xmmword ptr [rbp + 0x20], xmm1
141613d3d: movaps xmmword ptr [rbp + 0x10], xmm0
141613d41: lea rax, [rip + 0x166c460]
141613d48: mov qword ptr [rsp + 0x20], rax
141613d4d: lea rcx, [rip + 0x166ad94]
141613d54: lea r9, [rip + 0x166ad6d]
141613d5b: lea r8, [rbp + 0x10]
141613d5f: mov edx, 0x2b
141613d64: call 0x1428d9760
141613d69: ud2 
141613d6b: nop dword ptr [rax + rax]
141613d70: mov qword ptr [rsp + 0x10], rdx
141613d75: push rbp
141613d76: push r15
141613d78: push r14
141613d7a: push r12
141613d7c: push rsi
141613d7d: push rdi
141613d7e: push rbx
141613d7f: sub rsp, 0x40
141613d83: lea rbp, [rdx + 0x80]
141613d8a: lea rcx, [rbp + 0x10]
141613d8e: call 0x1407cea70
141613d93: nop 
141613d94: add rsp, 0x40
141613d98: pop rbx
141613d99: pop rdi
141613d9a: pop rsi
141613d9b: pop r12
141613d9d: pop r14
141613d9f: pop r15
141613da1: pop rbp
141613da2: ret 
