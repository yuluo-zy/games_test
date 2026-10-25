141610db0: push rbp
141610db1: push r15
141610db3: push r14
141610db5: push r12
141610db7: push rsi
141610db8: push rdi
141610db9: push rbx
141610dba: sub rsp, 0x100
141610dc1: lea rbp, [rsp + 0x80]
141610dc9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141610dd1: mov rbx, r8
141610dd4: mov rdi, rdx
141610dd7: mov rsi, rcx
141610dda: test r8, r8
141610ddd: je 0x141610e39
141610ddf: movabs rcx, 0xaaaaaaaaaaaaaaab
141610de9: mov rax, r9
141610dec: mul rcx
141610def: shr rdx, 1
141610df2: and rdx, 0xfffffffffffffff0
141610df6: lea rax, [rdx + rdx*2]
141610dfa: mov rcx, r9
141610dfd: sub rcx, rax
141610e00: mov qword ptr [rbp - 0x40], rcx
141610e04: jne 0x141610eca
141610e0a: mov rax, rbx
141610e0d: and rax, 3
141610e11: mov qword ptr [rbp - 0x40], rax
141610e15: jne 0x141610edb
141610e1b: cmp r9, 0x6c30
141610e22: jae 0x141610f57
141610e28: movzx eax, r9w
141610e2c: imul r12d, eax, 0xaaab
141610e33: shr r12d, 0x15
141610e37: jmp 0x141610e39
141610e39: lea r14, [rbp + 0x10]
141610e3d: mov edx, 0x6c00
141610e42: mov rcx, r14
141610e45: mov r8d, 0x22
141610e4b: mov r9b, 1
141610e4e: call 0x140bd4de0
141610e53: lea r15, [rbp + 0x68]
141610e57: mov r8d, 4
141610e5d: mov rcx, r15
141610e60: mov rdx, r14
141610e63: call 0x140bd4f20
141610e68: shl r12, 4
141610e6c: lea rax, [r12 + r12*2]
141610e70: mov qword ptr [rsp + 0x30], rax
141610e75: mov qword ptr [rsp + 0x28], rbx
141610e7a: mov qword ptr [rsp + 0x20], r15
141610e7f: lea r8, [rip + 0x166eef1]
141610e86: lea rcx, [rbp - 0x40]
141610e8a: mov r9d, 0xf
141610e90: mov rdx, rdi
141610e93: call 0x1416a2dc0
141610e98: cmp dword ptr [rbp - 0x40], 0xb
141610e9c: jne 0x141610f05
141610e9e: mov rax, qword ptr [rbp - 0x18]
141610ea2: mov qword ptr [rsi + 0x20], rax
141610ea6: movups xmm0, xmmword ptr [rbp - 0x38]
141610eaa: movups xmm1, xmmword ptr [rbp - 0x28]
141610eae: movups xmmword ptr [rsi + 0x10], xmm1
141610eb2: movups xmmword ptr [rsi], xmm0
141610eb5: mov rax, rsi
141610eb8: add rsp, 0x100
141610ebf: pop rbx
141610ec0: pop rdi
141610ec1: pop rsi
141610ec2: pop r12
141610ec4: pop r14
141610ec6: pop r15
141610ec8: pop rbp
141610ec9: ret 
141610eca: mov qword ptr [rbp + 0x10], 0
141610ed2: lea rax, [rip + 0x16b906f]
141610ed9: jmp 0x141610eea
141610edb: mov qword ptr [rbp + 0x10], 0
141610ee3: lea rax, [rip + 0x16b9076]
141610eea: mov qword ptr [rsp + 0x20], rax
141610eef: lea r8, [rip + 0x16b904a]
141610ef6: lea rdx, [rbp - 0x40]
141610efa: lea r9, [rbp + 0x10]
141610efe: xor ecx, ecx
141610f00: call 0x1428d9596
141610f05: movups xmm0, xmmword ptr [rbp]
141610f09: movaps xmmword ptr [rbp + 0x50], xmm0
141610f0d: movups xmm0, xmmword ptr [rbp - 0x40]
141610f11: movups xmm1, xmmword ptr [rbp - 0x30]
141610f15: movups xmm2, xmmword ptr [rbp - 0x20]
141610f19: movups xmm3, xmmword ptr [rbp - 0x10]
141610f1d: movaps xmmword ptr [rbp + 0x40], xmm3
141610f21: movaps xmmword ptr [rbp + 0x30], xmm2
141610f25: movaps xmmword ptr [rbp + 0x20], xmm1
141610f29: movaps xmmword ptr [rbp + 0x10], xmm0
141610f2d: lea rax, [rip + 0x166f274]
141610f34: mov qword ptr [rsp + 0x20], rax
141610f39: lea rcx, [rip + 0x166dba8]
141610f40: lea r9, [rip + 0x166db81]
141610f47: lea r8, [rbp + 0x10]
141610f4b: mov edx, 0x2b
141610f50: call 0x1428d9760
141610f55: ud2 
141610f57: lea rax, [rip + 0x148d4a4]
141610f5e: mov qword ptr [rbp - 0x40], rax
141610f62: mov qword ptr [rbp - 0x38], 0x55
141610f6a: lea rax, [rbp - 0x40]
141610f6e: mov qword ptr [rbp + 0x68], rax
141610f72: lea rax, [rip - 0x15070e9]
141610f79: mov qword ptr [rbp + 0x70], rax
141610f7d: lea rax, [rip + 0x166e844]
141610f84: mov qword ptr [rbp + 0x10], rax
141610f88: mov qword ptr [rbp + 0x18], 1
141610f90: mov qword ptr [rbp + 0x30], 0
141610f98: lea rax, [rbp + 0x68]
141610f9c: mov qword ptr [rbp + 0x20], rax
141610fa0: mov qword ptr [rbp + 0x28], 1
141610fa8: lea rdx, [rip + 0x166e829]
141610faf: lea rcx, [rbp + 0x10]
141610fb3: call 0x1428d9390
141610fb8: int3 
141610fb9: nop dword ptr [rax]
141610fc0: mov qword ptr [rsp + 0x10], rdx
141610fc5: push rbp
141610fc6: push r15
141610fc8: push r14
141610fca: push r12
141610fcc: push rsi
141610fcd: push rdi
141610fce: push rbx
141610fcf: sub rsp, 0x40
141610fd3: lea rbp, [rdx + 0x80]
141610fda: lea rcx, [rbp + 0x10]
141610fde: call 0x1407cea70
141610fe3: nop 
141610fe4: add rsp, 0x40
141610fe8: pop rbx
141610fe9: pop rdi
141610fea: pop rsi
141610feb: pop r12
141610fed: pop r14
141610fef: pop r15
141610ff1: pop rbp
141610ff2: ret 
