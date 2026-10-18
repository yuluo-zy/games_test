141610b60: push rbp
141610b61: push r15
141610b63: push r14
141610b65: push r12
141610b67: push rsi
141610b68: push rdi
141610b69: push rbx
141610b6a: sub rsp, 0x100
141610b71: lea rbp, [rsp + 0x80]
141610b79: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141610b81: mov rbx, r8
141610b84: mov rdi, rdx
141610b87: mov rsi, rcx
141610b8a: test r8, r8
141610b8d: je 0x141610beb
141610b8f: movabs rcx, 0xaaaaaaaaaaaaaaab
141610b99: mov rax, r9
141610b9c: mul rcx
141610b9f: shr rdx, 1
141610ba2: and rdx, 0xfffffffffffffff0
141610ba6: lea rax, [rdx + rdx*2]
141610baa: mov rcx, r9
141610bad: sub rcx, rax
141610bb0: mov qword ptr [rbp - 0x40], rcx
141610bb4: jne 0x141610c7c
141610bba: mov rax, rbx
141610bbd: and rax, 3
141610bc1: mov qword ptr [rbp - 0x40], rax
141610bc5: jne 0x141610c8d
141610bcb: cmp r9, 0x70830
141610bd2: jae 0x141610d09
141610bd8: mov eax, r9d
141610bdb: mov r12d, 0xaaaaaaab
141610be1: imul r12, rax
141610be5: shr r12, 0x25
141610be9: jmp 0x141610beb
141610beb: lea r14, [rbp + 0x10]
141610bef: mov edx, 0x70800
141610bf4: mov rcx, r14
141610bf7: mov r8d, 0x22
141610bfd: mov r9b, 1
141610c00: call 0x140bd4de0
141610c05: lea r15, [rbp + 0x68]
141610c09: mov r8d, 4
141610c0f: mov rcx, r15
141610c12: mov rdx, r14
141610c15: call 0x140bd4f20
141610c1a: shl r12, 4
141610c1e: lea rax, [r12 + r12*2]
141610c22: mov qword ptr [rsp + 0x30], rax
141610c27: mov qword ptr [rsp + 0x28], rbx
141610c2c: mov qword ptr [rsp + 0x20], r15
141610c31: lea r8, [rip + 0x166ef3d]
141610c38: lea rcx, [rbp - 0x40]
141610c3c: mov r9d, 6
141610c42: mov rdx, rdi
141610c45: call 0x1416a2dc0
141610c4a: cmp dword ptr [rbp - 0x40], 0xb
141610c4e: jne 0x141610cb7
141610c50: mov rax, qword ptr [rbp - 0x18]
141610c54: mov qword ptr [rsi + 0x20], rax
141610c58: movups xmm0, xmmword ptr [rbp - 0x38]
141610c5c: movups xmm1, xmmword ptr [rbp - 0x28]
141610c60: movups xmmword ptr [rsi + 0x10], xmm1
141610c64: movups xmmword ptr [rsi], xmm0
141610c67: mov rax, rsi
141610c6a: add rsp, 0x100
141610c71: pop rbx
141610c72: pop rdi
141610c73: pop rsi
141610c74: pop r12
141610c76: pop r14
141610c78: pop r15
141610c7a: pop rbp
141610c7b: ret 
141610c7c: mov qword ptr [rbp + 0x10], 0
141610c84: lea rax, [rip + 0x16b92bd]
141610c8b: jmp 0x141610c9c
141610c8d: mov qword ptr [rbp + 0x10], 0
141610c95: lea rax, [rip + 0x16b92c4]
141610c9c: mov qword ptr [rsp + 0x20], rax
141610ca1: lea r8, [rip + 0x16b9298]
141610ca8: lea rdx, [rbp - 0x40]
141610cac: lea r9, [rbp + 0x10]
141610cb0: xor ecx, ecx
141610cb2: call 0x1428d9596
141610cb7: movups xmm0, xmmword ptr [rbp]
141610cbb: movaps xmmword ptr [rbp + 0x50], xmm0
141610cbf: movups xmm0, xmmword ptr [rbp - 0x40]
141610cc3: movups xmm1, xmmword ptr [rbp - 0x30]
141610cc7: movups xmm2, xmmword ptr [rbp - 0x20]
141610ccb: movups xmm3, xmmword ptr [rbp - 0x10]
141610ccf: movaps xmmword ptr [rbp + 0x40], xmm3
141610cd3: movaps xmmword ptr [rbp + 0x30], xmm2
141610cd7: movaps xmmword ptr [rbp + 0x20], xmm1
141610cdb: movaps xmmword ptr [rbp + 0x10], xmm0
141610cdf: lea rax, [rip + 0x166f4c2]
141610ce6: mov qword ptr [rsp + 0x20], rax
141610ceb: lea rcx, [rip + 0x166ddf6]
141610cf2: lea r9, [rip + 0x166ddcf]
141610cf9: lea r8, [rbp + 0x10]
141610cfd: mov edx, 0x2b
141610d02: call 0x1428d9760
141610d07: ud2 
141610d09: lea rax, [rip + 0x148ce4c]
141610d10: mov qword ptr [rbp - 0x40], rax
141610d14: mov qword ptr [rbp - 0x38], 0x4c
141610d1c: lea rax, [rbp - 0x40]
141610d20: mov qword ptr [rbp + 0x68], rax
141610d24: lea rax, [rip - 0x1506e9b]
141610d2b: mov qword ptr [rbp + 0x70], rax
141610d2f: lea rax, [rip + 0x166ea92]
141610d36: mov qword ptr [rbp + 0x10], rax
141610d3a: mov qword ptr [rbp + 0x18], 1
141610d42: mov qword ptr [rbp + 0x30], 0
141610d4a: lea rax, [rbp + 0x68]
141610d4e: mov qword ptr [rbp + 0x20], rax
141610d52: mov qword ptr [rbp + 0x28], 1
141610d5a: lea rdx, [rip + 0x166ea77]
141610d61: lea rcx, [rbp + 0x10]
141610d65: call 0x1428d9390
141610d6a: int3 
141610d6b: nop dword ptr [rax + rax]
141610d70: mov qword ptr [rsp + 0x10], rdx
141610d75: push rbp
141610d76: push r15
141610d78: push r14
141610d7a: push r12
141610d7c: push rsi
141610d7d: push rdi
141610d7e: push rbx
141610d7f: sub rsp, 0x40
141610d83: lea rbp, [rdx + 0x80]
141610d8a: lea rcx, [rbp + 0x10]
141610d8e: call 0x1407cea70
141610d93: nop 
141610d94: add rsp, 0x40
141610d98: pop rbx
141610d99: pop rdi
141610d9a: pop rsi
141610d9b: pop r12
141610d9d: pop r14
141610d9f: pop r15
141610da1: pop rbp
141610da2: ret 
