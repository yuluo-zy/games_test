141612a90: push rbp
141612a91: push r15
141612a93: push r14
141612a95: push r12
141612a97: push rsi
141612a98: push rdi
141612a99: push rbx
141612a9a: sub rsp, 0x100
141612aa1: lea rbp, [rsp + 0x80]
141612aa9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141612ab1: mov rbx, r8
141612ab4: mov rdi, rdx
141612ab7: mov rsi, rcx
141612aba: test r8, r8
141612abd: je 0x141612b52
141612ac3: mov r14, r9
141612ac6: mov rax, r9
141612ac9: and rax, 3
141612acd: mov qword ptr [rbp - 0x40], rax
141612ad1: jne 0x141612bde
141612ad7: mov rax, rbx
141612ada: and rax, 3
141612ade: mov qword ptr [rbp - 0x40], rax
141612ae2: jne 0x141612bef
141612ae8: cmp r14, 0x3800004
141612aef: jb 0x141612b55
141612af1: lea rax, [rip + 0x148b5c4]
141612af8: mov qword ptr [rbp - 0x40], rax
141612afc: mov qword ptr [rbp - 0x38], 0x39
141612b04: lea rax, [rbp - 0x40]
141612b08: mov qword ptr [rbp + 0x68], rax
141612b0c: lea rax, [rip - 0x1508c83]
141612b13: mov qword ptr [rbp + 0x70], rax
141612b17: lea rax, [rip + 0x166ccaa]
141612b1e: mov qword ptr [rbp + 0x10], rax
141612b22: mov qword ptr [rbp + 0x18], 1
141612b2a: mov qword ptr [rbp + 0x30], 0
141612b32: lea rax, [rbp + 0x68]
141612b36: mov qword ptr [rbp + 0x20], rax
141612b3a: mov qword ptr [rbp + 0x28], 1
141612b42: lea rdx, [rip + 0x166cc8f]
141612b49: lea rcx, [rbp + 0x10]
141612b4d: call 0x1428d9390
141612b52: xor r14d, r14d
141612b55: lea r15, [rbp + 0x10]
141612b59: mov edx, 0x3800000
141612b5e: mov rcx, r15
141612b61: mov r8d, 0x22
141612b67: mov r9b, 1
141612b6a: call 0x140bd4de0
141612b6f: lea r12, [rbp + 0x68]
141612b73: mov r8d, 4
141612b79: mov rcx, r12
141612b7c: mov rdx, r15
141612b7f: call 0x140bd4f20
141612b84: mov qword ptr [rsp + 0x30], r14
141612b89: mov qword ptr [rsp + 0x28], rbx
141612b8e: mov qword ptr [rsp + 0x20], r12
141612b93: lea r8, [rip + 0x166d481]
141612b9a: lea rcx, [rbp - 0x40]
141612b9e: mov r9d, 0x11
141612ba4: mov rdx, rdi
141612ba7: call 0x1416a2dc0
141612bac: cmp dword ptr [rbp - 0x40], 0xb
141612bb0: jne 0x141612c19
141612bb2: mov rax, qword ptr [rbp - 0x18]
141612bb6: mov qword ptr [rsi + 0x20], rax
141612bba: movups xmm0, xmmword ptr [rbp - 0x38]
141612bbe: movups xmm1, xmmword ptr [rbp - 0x28]
141612bc2: movups xmmword ptr [rsi + 0x10], xmm1
141612bc6: movups xmmword ptr [rsi], xmm0
141612bc9: mov rax, rsi
141612bcc: add rsp, 0x100
141612bd3: pop rbx
141612bd4: pop rdi
141612bd5: pop rsi
141612bd6: pop r12
141612bd8: pop r14
141612bda: pop r15
141612bdc: pop rbp
141612bdd: ret 
141612bde: mov qword ptr [rbp + 0x10], 0
141612be6: lea rax, [rip + 0x16b735b]
141612bed: jmp 0x141612bfe
141612bef: mov qword ptr [rbp + 0x10], 0
141612bf7: lea rax, [rip + 0x16b7362]
141612bfe: mov qword ptr [rsp + 0x20], rax
141612c03: lea r8, [rip + 0x16b7336]
141612c0a: lea rdx, [rbp - 0x40]
141612c0e: lea r9, [rbp + 0x10]
141612c12: xor ecx, ecx
141612c14: call 0x1428d9596
141612c19: movups xmm0, xmmword ptr [rbp]
141612c1d: movaps xmmword ptr [rbp + 0x50], xmm0
141612c21: movups xmm0, xmmword ptr [rbp - 0x40]
141612c25: movups xmm1, xmmword ptr [rbp - 0x30]
141612c29: movups xmm2, xmmword ptr [rbp - 0x20]
141612c2d: movups xmm3, xmmword ptr [rbp - 0x10]
141612c31: movaps xmmword ptr [rbp + 0x40], xmm3
141612c35: movaps xmmword ptr [rbp + 0x30], xmm2
141612c39: movaps xmmword ptr [rbp + 0x20], xmm1
141612c3d: movaps xmmword ptr [rbp + 0x10], xmm0
141612c41: lea rax, [rip + 0x166d560]
141612c48: mov qword ptr [rsp + 0x20], rax
141612c4d: lea rcx, [rip + 0x166be94]
141612c54: lea r9, [rip + 0x166be6d]
141612c5b: lea r8, [rbp + 0x10]
141612c5f: mov edx, 0x2b
141612c64: call 0x1428d9760
141612c69: ud2 
141612c6b: nop dword ptr [rax + rax]
141612c70: mov qword ptr [rsp + 0x10], rdx
141612c75: push rbp
141612c76: push r15
141612c78: push r14
141612c7a: push r12
141612c7c: push rsi
141612c7d: push rdi
141612c7e: push rbx
141612c7f: sub rsp, 0x40
141612c83: lea rbp, [rdx + 0x80]
141612c8a: lea rcx, [rbp + 0x10]
141612c8e: call 0x1407cea70
141612c93: nop 
141612c94: add rsp, 0x40
141612c98: pop rbx
141612c99: pop rdi
141612c9a: pop rsi
141612c9b: pop r12
141612c9d: pop r14
141612c9f: pop r15
141612ca1: pop rbp
141612ca2: ret 
