14160fb70: push rbp
14160fb71: push r15
14160fb73: push r14
14160fb75: push r12
14160fb77: push rsi
14160fb78: push rdi
14160fb79: push rbx
14160fb7a: sub rsp, 0x100
14160fb81: lea rbp, [rsp + 0x80]
14160fb89: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
14160fb91: mov rbx, r8
14160fb94: mov rdi, rdx
14160fb97: mov rsi, rcx
14160fb9a: test r8, r8
14160fb9d: je 0x14160fc32
14160fba3: mov r14, r9
14160fba6: mov rax, r9
14160fba9: and rax, 3
14160fbad: mov qword ptr [rbp - 0x40], rax
14160fbb1: jne 0x14160fcbe
14160fbb7: mov rax, rbx
14160fbba: and rax, 3
14160fbbe: mov qword ptr [rbp - 0x40], rax
14160fbc2: jne 0x14160fccf
14160fbc8: cmp r14, 0x400004
14160fbcf: jb 0x14160fc35
14160fbd1: lea rax, [rip + 0x148e7a9]
14160fbd8: mov qword ptr [rbp - 0x40], rax
14160fbdc: mov qword ptr [rbp - 0x38], 0x4c
14160fbe4: lea rax, [rbp - 0x40]
14160fbe8: mov qword ptr [rbp + 0x68], rax
14160fbec: lea rax, [rip - 0x1505d63]
14160fbf3: mov qword ptr [rbp + 0x70], rax
14160fbf7: lea rax, [rip + 0x166fbca]
14160fbfe: mov qword ptr [rbp + 0x10], rax
14160fc02: mov qword ptr [rbp + 0x18], 1
14160fc0a: mov qword ptr [rbp + 0x30], 0
14160fc12: lea rax, [rbp + 0x68]
14160fc16: mov qword ptr [rbp + 0x20], rax
14160fc1a: mov qword ptr [rbp + 0x28], 1
14160fc22: lea rdx, [rip + 0x166fbaf]
14160fc29: lea rcx, [rbp + 0x10]
14160fc2d: call 0x1428d9390
14160fc32: xor r14d, r14d
14160fc35: lea r15, [rbp + 0x10]
14160fc39: mov edx, 0x400000
14160fc3e: mov rcx, r15
14160fc41: mov r8d, 0x22
14160fc47: mov r9b, 1
14160fc4a: call 0x140bd4de0
14160fc4f: lea r12, [rbp + 0x68]
14160fc53: mov r8d, 4
14160fc59: mov rcx, r12
14160fc5c: mov rdx, r15
14160fc5f: call 0x140bd4f20
14160fc64: mov qword ptr [rsp + 0x30], r14
14160fc69: mov qword ptr [rsp + 0x28], rbx
14160fc6e: mov qword ptr [rsp + 0x20], r12
14160fc73: lea r8, [rip + 0x16704dd]
14160fc7a: lea rcx, [rbp - 0x40]
14160fc7e: mov r9d, 0xb
14160fc84: mov rdx, rdi
14160fc87: call 0x1416a2dc0
14160fc8c: cmp dword ptr [rbp - 0x40], 0xb
14160fc90: jne 0x14160fcf9
14160fc92: mov rax, qword ptr [rbp - 0x18]
14160fc96: mov qword ptr [rsi + 0x20], rax
14160fc9a: movups xmm0, xmmword ptr [rbp - 0x38]
14160fc9e: movups xmm1, xmmword ptr [rbp - 0x28]
14160fca2: movups xmmword ptr [rsi + 0x10], xmm1
14160fca6: movups xmmword ptr [rsi], xmm0
14160fca9: mov rax, rsi
14160fcac: add rsp, 0x100
14160fcb3: pop rbx
14160fcb4: pop rdi
14160fcb5: pop rsi
14160fcb6: pop r12
14160fcb8: pop r14
14160fcba: pop r15
14160fcbc: pop rbp
14160fcbd: ret 
14160fcbe: mov qword ptr [rbp + 0x10], 0
14160fcc6: lea rax, [rip + 0x16ba27b]
14160fccd: jmp 0x14160fcde
14160fccf: mov qword ptr [rbp + 0x10], 0
14160fcd7: lea rax, [rip + 0x16ba282]
14160fcde: mov qword ptr [rsp + 0x20], rax
14160fce3: lea r8, [rip + 0x16ba256]
14160fcea: lea rdx, [rbp - 0x40]
14160fcee: lea r9, [rbp + 0x10]
14160fcf2: xor ecx, ecx
14160fcf4: call 0x1428d9596
14160fcf9: movups xmm0, xmmword ptr [rbp]
14160fcfd: movaps xmmword ptr [rbp + 0x50], xmm0
14160fd01: movups xmm0, xmmword ptr [rbp - 0x40]
14160fd05: movups xmm1, xmmword ptr [rbp - 0x30]
14160fd09: movups xmm2, xmmword ptr [rbp - 0x20]
14160fd0d: movups xmm3, xmmword ptr [rbp - 0x10]
14160fd11: movaps xmmword ptr [rbp + 0x40], xmm3
14160fd15: movaps xmmword ptr [rbp + 0x30], xmm2
14160fd19: movaps xmmword ptr [rbp + 0x20], xmm1
14160fd1d: movaps xmmword ptr [rbp + 0x10], xmm0
14160fd21: lea rax, [rip + 0x1670480]
14160fd28: mov qword ptr [rsp + 0x20], rax
14160fd2d: lea rcx, [rip + 0x166edb4]
14160fd34: lea r9, [rip + 0x166ed8d]
14160fd3b: lea r8, [rbp + 0x10]
14160fd3f: mov edx, 0x2b
14160fd44: call 0x1428d9760
14160fd49: ud2 
14160fd4b: nop dword ptr [rax + rax]
14160fd50: mov qword ptr [rsp + 0x10], rdx
14160fd55: push rbp
14160fd56: push r15
14160fd58: push r14
14160fd5a: push r12
14160fd5c: push rsi
14160fd5d: push rdi
14160fd5e: push rbx
14160fd5f: sub rsp, 0x40
14160fd63: lea rbp, [rdx + 0x80]
14160fd6a: lea rcx, [rbp + 0x10]
14160fd6e: call 0x1407cea70
14160fd73: nop 
14160fd74: add rsp, 0x40
14160fd78: pop rbx
14160fd79: pop rdi
14160fd7a: pop rsi
14160fd7b: pop r12
14160fd7d: pop r14
14160fd7f: pop r15
14160fd81: pop rbp
14160fd82: ret 
