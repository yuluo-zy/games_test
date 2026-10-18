1416159e0: push rbp
1416159e1: push r15
1416159e3: push r14
1416159e5: push r12
1416159e7: push rsi
1416159e8: push rdi
1416159e9: push rbx
1416159ea: sub rsp, 0x100
1416159f1: lea rbp, [rsp + 0x80]
1416159f9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141615a01: mov rsi, rcx
141615a04: test r8, r8
141615a07: sete al
141615a0a: cmp r9, 0xe2
141615a11: setb cl
141615a14: or cl, al
141615a16: je 0x141615ab3
141615a1c: mov r14, r9
141615a1f: mov rbx, r8
141615a22: mov rdi, rdx
141615a25: lea r15, [rbp + 0x10]
141615a29: mov edx, 0x384
141615a2e: mov rcx, r15
141615a31: mov r8d, 0x22
141615a37: mov r9b, 1
141615a3a: call 0x140bd4de0
141615a3f: lea r12, [rbp + 0x68]
141615a43: mov r8d, 4
141615a49: mov rcx, r12
141615a4c: mov rdx, r15
141615a4f: call 0x140bd4f20
141615a54: shl r14, 2
141615a58: mov qword ptr [rsp + 0x30], r14
141615a5d: mov qword ptr [rsp + 0x28], rbx
141615a62: mov qword ptr [rsp + 0x20], r12
141615a67: lea r8, [rip + 0x1669dfc]
141615a6e: lea rcx, [rbp - 0x40]
141615a72: mov r9d, 0x11
141615a78: mov rdx, rdi
141615a7b: call 0x1416a2dc0
141615a80: cmp dword ptr [rbp - 0x40], 0xb
141615a84: jne 0x141615b14
141615a8a: mov rax, qword ptr [rbp - 0x18]
141615a8e: mov qword ptr [rsi + 0x20], rax
141615a92: movups xmm0, xmmword ptr [rbp - 0x38]
141615a96: movups xmm1, xmmword ptr [rbp - 0x28]
141615a9a: movups xmmword ptr [rsi + 0x10], xmm1
141615a9e: movups xmmword ptr [rsi], xmm0
141615aa1: add rsp, 0x100
141615aa8: pop rbx
141615aa9: pop rdi
141615aaa: pop rsi
141615aab: pop r12
141615aad: pop r14
141615aaf: pop r15
141615ab1: pop rbp
141615ab2: ret 
141615ab3: lea rax, [rip + 0x1487a87]
141615aba: mov qword ptr [rbp - 0x40], rax
141615abe: mov qword ptr [rbp - 0x38], 0x43
141615ac6: lea rax, [rbp - 0x40]
141615aca: mov qword ptr [rbp + 0x68], rax
141615ace: lea rax, [rip - 0x150bc45]
141615ad5: mov qword ptr [rbp + 0x70], rax
141615ad9: lea rax, [rip + 0x1669ce8]
141615ae0: mov qword ptr [rbp + 0x10], rax
141615ae4: mov qword ptr [rbp + 0x18], 1
141615aec: mov qword ptr [rbp + 0x30], 0
141615af4: lea rax, [rbp + 0x68]
141615af8: mov qword ptr [rbp + 0x20], rax
141615afc: mov qword ptr [rbp + 0x28], 1
141615b04: lea rdx, [rip + 0x1669ccd]
141615b0b: lea rcx, [rbp + 0x10]
141615b0f: call 0x1428d9390
141615b14: movups xmm0, xmmword ptr [rbp]
141615b18: movaps xmmword ptr [rbp + 0x50], xmm0
141615b1c: movups xmm0, xmmword ptr [rbp - 0x40]
141615b20: movups xmm1, xmmword ptr [rbp - 0x30]
141615b24: movups xmm2, xmmword ptr [rbp - 0x20]
141615b28: movups xmm3, xmmword ptr [rbp - 0x10]
141615b2c: movaps xmmword ptr [rbp + 0x40], xmm3
141615b30: movaps xmmword ptr [rbp + 0x30], xmm2
141615b34: movaps xmmword ptr [rbp + 0x20], xmm1
141615b38: movaps xmmword ptr [rbp + 0x10], xmm0
141615b3c: lea rax, [rip + 0x166a665]
141615b43: mov qword ptr [rsp + 0x20], rax
141615b48: lea rcx, [rip + 0x1668f99]
141615b4f: lea r9, [rip + 0x1668f72]
141615b56: lea r8, [rbp + 0x10]
141615b5a: mov edx, 0x2b
141615b5f: call 0x1428d9760
141615b64: ud2 
141615b66: nop word ptr cs:[rax + rax]
141615b70: mov qword ptr [rsp + 0x10], rdx
141615b75: push rbp
141615b76: push r15
141615b78: push r14
141615b7a: push r12
141615b7c: push rsi
141615b7d: push rdi
141615b7e: push rbx
141615b7f: sub rsp, 0x40
141615b83: lea rbp, [rdx + 0x80]
141615b8a: lea rcx, [rbp + 0x10]
141615b8e: call 0x1407cea70
141615b93: nop 
141615b94: add rsp, 0x40
141615b98: pop rbx
141615b99: pop rdi
141615b9a: pop rsi
141615b9b: pop r12
141615b9d: pop r14
141615b9f: pop r15
141615ba1: pop rbp
141615ba2: ret 
