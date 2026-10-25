141617be0: push rbp
141617be1: push r15
141617be3: push r14
141617be5: push r12
141617be7: push rsi
141617be8: push rdi
141617be9: push rbx
141617bea: sub rsp, 0x100
141617bf1: lea rbp, [rsp + 0x80]
141617bf9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141617c01: mov rsi, rcx
141617c04: test r8, r8
141617c07: sete al
141617c0a: cmp r9, 0x57e5
141617c11: setb cl
141617c14: or cl, al
141617c16: je 0x141617cb7
141617c1c: mov r14, r9
141617c1f: mov rbx, r8
141617c22: mov rdi, rdx
141617c25: lea r15, [rbp + 0x10]
141617c29: mov edx, 0x107ac0
141617c2e: mov rcx, r15
141617c31: mov r8d, 0x22
141617c37: mov r9b, 1
141617c3a: call 0x140bd4de0
141617c3f: lea r12, [rbp + 0x68]
141617c43: mov r8d, 4
141617c49: mov rcx, r12
141617c4c: mov rdx, r15
141617c4f: call 0x140bd4f20
141617c54: shl r14, 4
141617c58: lea rax, [r14 + r14*2]
141617c5c: mov qword ptr [rsp + 0x30], rax
141617c61: mov qword ptr [rsp + 0x28], rbx
141617c66: mov qword ptr [rsp + 0x20], r12
141617c6b: lea r8, [rip + 0x1667f09]
141617c72: lea rcx, [rbp - 0x40]
141617c76: mov r9d, 5
141617c7c: mov rdx, rdi
141617c7f: call 0x1416a2dc0
141617c84: cmp dword ptr [rbp - 0x40], 0xb
141617c88: jne 0x141617d18
141617c8e: mov rax, qword ptr [rbp - 0x18]
141617c92: mov qword ptr [rsi + 0x20], rax
141617c96: movups xmm0, xmmword ptr [rbp - 0x38]
141617c9a: movups xmm1, xmmword ptr [rbp - 0x28]
141617c9e: movups xmmword ptr [rsi + 0x10], xmm1
141617ca2: movups xmmword ptr [rsi], xmm0
141617ca5: add rsp, 0x100
141617cac: pop rbx
141617cad: pop rdi
141617cae: pop rsi
141617caf: pop r12
141617cb1: pop r14
141617cb3: pop r15
141617cb5: pop rbp
141617cb6: ret 
141617cb7: lea rax, [rip + 0x1485675]
141617cbe: mov qword ptr [rbp - 0x40], rax
141617cc2: mov qword ptr [rbp - 0x38], 0x4b
141617cca: lea rax, [rbp - 0x40]
141617cce: mov qword ptr [rbp + 0x68], rax
141617cd2: lea rax, [rip - 0x150de49]
141617cd9: mov qword ptr [rbp + 0x70], rax
141617cdd: lea rax, [rip + 0x1667ae4]
141617ce4: mov qword ptr [rbp + 0x10], rax
141617ce8: mov qword ptr [rbp + 0x18], 1
141617cf0: mov qword ptr [rbp + 0x30], 0
141617cf8: lea rax, [rbp + 0x68]
141617cfc: mov qword ptr [rbp + 0x20], rax
141617d00: mov qword ptr [rbp + 0x28], 1
141617d08: lea rdx, [rip + 0x1667ac9]
141617d0f: lea rcx, [rbp + 0x10]
141617d13: call 0x1428d9390
141617d18: movups xmm0, xmmword ptr [rbp]
141617d1c: movaps xmmword ptr [rbp + 0x50], xmm0
141617d20: movups xmm0, xmmword ptr [rbp - 0x40]
141617d24: movups xmm1, xmmword ptr [rbp - 0x30]
141617d28: movups xmm2, xmmword ptr [rbp - 0x20]
141617d2c: movups xmm3, xmmword ptr [rbp - 0x10]
141617d30: movaps xmmword ptr [rbp + 0x40], xmm3
141617d34: movaps xmmword ptr [rbp + 0x30], xmm2
141617d38: movaps xmmword ptr [rbp + 0x20], xmm1
141617d3c: movaps xmmword ptr [rbp + 0x10], xmm0
141617d40: lea rax, [rip + 0x1668461]
141617d47: mov qword ptr [rsp + 0x20], rax
141617d4c: lea rcx, [rip + 0x1666d95]
141617d53: lea r9, [rip + 0x1666d6e]
141617d5a: lea r8, [rbp + 0x10]
141617d5e: mov edx, 0x2b
141617d63: call 0x1428d9760
141617d68: ud2 
141617d6a: nop word ptr [rax + rax]
141617d70: mov qword ptr [rsp + 0x10], rdx
141617d75: push rbp
141617d76: push r15
141617d78: push r14
141617d7a: push r12
141617d7c: push rsi
141617d7d: push rdi
141617d7e: push rbx
141617d7f: sub rsp, 0x40
141617d83: lea rbp, [rdx + 0x80]
141617d8a: lea rcx, [rbp + 0x10]
141617d8e: call 0x1407cea70
141617d93: nop 
141617d94: add rsp, 0x40
141617d98: pop rbx
141617d99: pop rdi
141617d9a: pop rsi
141617d9b: pop r12
141617d9d: pop r14
141617d9f: pop r15
141617da1: pop rbp
141617da2: ret 
