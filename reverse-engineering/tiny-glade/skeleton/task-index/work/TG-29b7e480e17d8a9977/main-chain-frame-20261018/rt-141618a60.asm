141618a60: push rbp
141618a61: push r15
141618a63: push r14
141618a65: push r12
141618a67: push rsi
141618a68: push rdi
141618a69: push rbx
141618a6a: sub rsp, 0x100
141618a71: lea rbp, [rsp + 0x80]
141618a79: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141618a81: mov rsi, rcx
141618a84: test r8, r8
141618a87: sete al
141618a8a: cmp r9, 0x10001
141618a91: setb cl
141618a94: or cl, al
141618a96: je 0x141618b37
141618a9c: mov r14, r9
141618a9f: mov rbx, r8
141618aa2: mov rdi, rdx
141618aa5: lea r15, [rbp + 0x10]
141618aa9: mov edx, 0x500000
141618aae: mov rcx, r15
141618ab1: mov r8d, 0x22
141618ab7: mov r9b, 1
141618aba: call 0x140bd4de0
141618abf: lea r12, [rbp + 0x68]
141618ac3: mov r8d, 4
141618ac9: mov rcx, r12
141618acc: mov rdx, r15
141618acf: call 0x140bd4f20
141618ad4: shl r14, 4
141618ad8: lea rax, [r14 + r14*4]
141618adc: mov qword ptr [rsp + 0x30], rax
141618ae1: mov qword ptr [rsp + 0x28], rbx
141618ae6: mov qword ptr [rsp + 0x20], r12
141618aeb: lea r8, [rip + 0x1667269]
141618af2: lea rcx, [rbp - 0x40]
141618af6: mov r9d, 5
141618afc: mov rdx, rdi
141618aff: call 0x1416a2dc0
141618b04: cmp dword ptr [rbp - 0x40], 0xb
141618b08: jne 0x141618b98
141618b0e: mov rax, qword ptr [rbp - 0x18]
141618b12: mov qword ptr [rsi + 0x20], rax
141618b16: movups xmm0, xmmword ptr [rbp - 0x38]
141618b1a: movups xmm1, xmmword ptr [rbp - 0x28]
141618b1e: movups xmmword ptr [rsi + 0x10], xmm1
141618b22: movups xmmword ptr [rsi], xmm0
141618b25: add rsp, 0x100
141618b2c: pop rbx
141618b2d: pop rdi
141618b2e: pop rsi
141618b2f: pop r12
141618b31: pop r14
141618b33: pop r15
141618b35: pop rbp
141618b36: ret 
141618b37: lea rax, [rip + 0x1485be8]
141618b3e: mov qword ptr [rbp - 0x40], rax
141618b42: mov qword ptr [rbp - 0x38], 0x26
141618b4a: lea rax, [rbp - 0x40]
141618b4e: mov qword ptr [rbp + 0x68], rax
141618b52: lea rax, [rip - 0x150ecc9]
141618b59: mov qword ptr [rbp + 0x70], rax
141618b5d: lea rax, [rip + 0x1666c64]
141618b64: mov qword ptr [rbp + 0x10], rax
141618b68: mov qword ptr [rbp + 0x18], 1
141618b70: mov qword ptr [rbp + 0x30], 0
141618b78: lea rax, [rbp + 0x68]
141618b7c: mov qword ptr [rbp + 0x20], rax
141618b80: mov qword ptr [rbp + 0x28], 1
141618b88: lea rdx, [rip + 0x1666c49]
141618b8f: lea rcx, [rbp + 0x10]
141618b93: call 0x1428d9390
141618b98: movups xmm0, xmmword ptr [rbp]
141618b9c: movaps xmmword ptr [rbp + 0x50], xmm0
141618ba0: movups xmm0, xmmword ptr [rbp - 0x40]
141618ba4: movups xmm1, xmmword ptr [rbp - 0x30]
141618ba8: movups xmm2, xmmword ptr [rbp - 0x20]
141618bac: movups xmm3, xmmword ptr [rbp - 0x10]
141618bb0: movaps xmmword ptr [rbp + 0x40], xmm3
141618bb4: movaps xmmword ptr [rbp + 0x30], xmm2
141618bb8: movaps xmmword ptr [rbp + 0x20], xmm1
141618bbc: movaps xmmword ptr [rbp + 0x10], xmm0
141618bc0: lea rax, [rip + 0x16675e1]
141618bc7: mov qword ptr [rsp + 0x20], rax
141618bcc: lea rcx, [rip + 0x1665f15]
141618bd3: lea r9, [rip + 0x1665eee]
141618bda: lea r8, [rbp + 0x10]
141618bde: mov edx, 0x2b
141618be3: call 0x1428d9760
141618be8: ud2 
141618bea: nop word ptr [rax + rax]
141618bf0: mov qword ptr [rsp + 0x10], rdx
141618bf5: push rbp
141618bf6: push r15
141618bf8: push r14
141618bfa: push r12
141618bfc: push rsi
141618bfd: push rdi
141618bfe: push rbx
141618bff: sub rsp, 0x40
141618c03: lea rbp, [rdx + 0x80]
141618c0a: lea rcx, [rbp + 0x10]
141618c0e: call 0x1407cea70
141618c13: nop 
141618c14: add rsp, 0x40
141618c18: pop rbx
141618c19: pop rdi
141618c1a: pop rsi
141618c1b: pop r12
141618c1d: pop r14
141618c1f: pop r15
141618c21: pop rbp
141618c22: ret 
