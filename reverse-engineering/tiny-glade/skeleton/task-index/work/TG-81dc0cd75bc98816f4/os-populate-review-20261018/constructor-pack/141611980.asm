141611980: push rbp
141611981: push r15
141611983: push r14
141611985: push r12
141611987: push rsi
141611988: push rdi
141611989: push rbx
14161198a: sub rsp, 0x100
141611991: lea rbp, [rsp + 0x80]
141611999: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
1416119a1: mov rbx, r8
1416119a4: mov rdi, rdx
1416119a7: mov rsi, rcx
1416119aa: test r8, r8
1416119ad: je 0x141611a42
1416119b3: mov r14, r9
1416119b6: mov rax, r9
1416119b9: and rax, 0x1f
1416119bd: mov qword ptr [rbp - 0x40], rax
1416119c1: jne 0x141611ace
1416119c7: mov rax, rbx
1416119ca: and rax, 3
1416119ce: mov qword ptr [rbp - 0x40], rax
1416119d2: jne 0x141611adf
1416119d8: cmp r14, 0x1200020
1416119df: jb 0x141611a45
1416119e1: lea rax, [rip + 0x148b88e]
1416119e8: mov qword ptr [rbp - 0x40], rax
1416119ec: mov qword ptr [rbp - 0x38], 0x4d
1416119f4: lea rax, [rbp - 0x40]
1416119f8: mov qword ptr [rbp + 0x68], rax
1416119fc: lea rax, [rip - 0x1507b73]
141611a03: mov qword ptr [rbp + 0x70], rax
141611a07: lea rax, [rip + 0x166ddba]
141611a0e: mov qword ptr [rbp + 0x10], rax
141611a12: mov qword ptr [rbp + 0x18], 1
141611a1a: mov qword ptr [rbp + 0x30], 0
141611a22: lea rax, [rbp + 0x68]
141611a26: mov qword ptr [rbp + 0x20], rax
141611a2a: mov qword ptr [rbp + 0x28], 1
141611a32: lea rdx, [rip + 0x166dd9f]
141611a39: lea rcx, [rbp + 0x10]
141611a3d: call 0x1428d9390
141611a42: xor r14d, r14d
141611a45: lea r15, [rbp + 0x10]
141611a49: mov edx, 0x1200000
141611a4e: mov rcx, r15
141611a51: mov r8d, 0x22
141611a57: mov r9b, 1
141611a5a: call 0x140bd4de0
141611a5f: lea r12, [rbp + 0x68]
141611a63: mov r8d, 4
141611a69: mov rcx, r12
141611a6c: mov rdx, r15
141611a6f: call 0x140bd4f20
141611a74: mov qword ptr [rsp + 0x30], r14
141611a79: mov qword ptr [rsp + 0x28], rbx
141611a7e: mov qword ptr [rsp + 0x20], r12
141611a83: lea r8, [rip + 0x166e6d8]
141611a8a: lea rcx, [rbp - 0x40]
141611a8e: mov r9d, 0xb
141611a94: mov rdx, rdi
141611a97: call 0x1416a2dc0
141611a9c: cmp dword ptr [rbp - 0x40], 0xb
141611aa0: jne 0x141611b09
141611aa2: mov rax, qword ptr [rbp - 0x18]
141611aa6: mov qword ptr [rsi + 0x20], rax
141611aaa: movups xmm0, xmmword ptr [rbp - 0x38]
141611aae: movups xmm1, xmmword ptr [rbp - 0x28]
141611ab2: movups xmmword ptr [rsi + 0x10], xmm1
141611ab6: movups xmmword ptr [rsi], xmm0
141611ab9: mov rax, rsi
141611abc: add rsp, 0x100
141611ac3: pop rbx
141611ac4: pop rdi
141611ac5: pop rsi
141611ac6: pop r12
141611ac8: pop r14
141611aca: pop r15
141611acc: pop rbp
141611acd: ret 
141611ace: mov qword ptr [rbp + 0x10], 0
141611ad6: lea rax, [rip + 0x16b846b]
141611add: jmp 0x141611aee
141611adf: mov qword ptr [rbp + 0x10], 0
141611ae7: lea rax, [rip + 0x16b8472]
141611aee: mov qword ptr [rsp + 0x20], rax
141611af3: lea r8, [rip + 0x16b8446]
141611afa: lea rdx, [rbp - 0x40]
141611afe: lea r9, [rbp + 0x10]
141611b02: xor ecx, ecx
141611b04: call 0x1428d9596
141611b09: movups xmm0, xmmword ptr [rbp]
141611b0d: movaps xmmword ptr [rbp + 0x50], xmm0
141611b11: movups xmm0, xmmword ptr [rbp - 0x40]
141611b15: movups xmm1, xmmword ptr [rbp - 0x30]
141611b19: movups xmm2, xmmword ptr [rbp - 0x20]
141611b1d: movups xmm3, xmmword ptr [rbp - 0x10]
141611b21: movaps xmmword ptr [rbp + 0x40], xmm3
141611b25: movaps xmmword ptr [rbp + 0x30], xmm2
141611b29: movaps xmmword ptr [rbp + 0x20], xmm1
141611b2d: movaps xmmword ptr [rbp + 0x10], xmm0
141611b31: lea rax, [rip + 0x166e670]
141611b38: mov qword ptr [rsp + 0x20], rax
141611b3d: lea rcx, [rip + 0x166cfa4]
141611b44: lea r9, [rip + 0x166cf7d]
141611b4b: lea r8, [rbp + 0x10]
141611b4f: mov edx, 0x2b
141611b54: call 0x1428d9760
141611b59: ud2 
141611b5b: nop dword ptr [rax + rax]
141611b60: mov qword ptr [rsp + 0x10], rdx
141611b65: push rbp
141611b66: push r15
141611b68: push r14
141611b6a: push r12
141611b6c: push rsi
141611b6d: push rdi
141611b6e: push rbx
141611b6f: sub rsp, 0x40
141611b73: lea rbp, [rdx + 0x80]
141611b7a: lea rcx, [rbp + 0x10]
141611b7e: call 0x1407cea70
141611b83: nop 
141611b84: add rsp, 0x40
141611b88: pop rbx
141611b89: pop rdi
141611b8a: pop rsi
141611b8b: pop r12
141611b8d: pop r14
141611b8f: pop r15
141611b91: pop rbp
141611b92: ret 
