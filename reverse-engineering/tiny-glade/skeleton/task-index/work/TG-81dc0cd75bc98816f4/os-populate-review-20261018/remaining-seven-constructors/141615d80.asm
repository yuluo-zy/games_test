141615d80: push rbp
141615d81: push r15
141615d83: push r14
141615d85: push r12
141615d87: push rsi
141615d88: push rdi
141615d89: push rbx
141615d8a: sub rsp, 0x100
141615d91: lea rbp, [rsp + 0x80]
141615d99: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141615da1: mov rsi, rcx
141615da4: test r8, r8
141615da7: sete al
141615daa: cmp r9, 0xc5c2
141615db1: setb cl
141615db4: or cl, al
141615db6: je 0x141615e53
141615dbc: mov r14, r9
141615dbf: mov rbx, r8
141615dc2: mov rdi, rdx
141615dc5: lea r15, [rbp + 0x10]
141615dc9: mov edx, 0x317040
141615dce: mov rcx, r15
141615dd1: mov r8d, 0x22
141615dd7: mov r9b, 1
141615dda: call 0x140bd4de0
141615ddf: lea r12, [rbp + 0x68]
141615de3: mov r8d, 0x10
141615de9: mov rcx, r12
141615dec: mov rdx, r15
141615def: call 0x140bd4f20
141615df4: shl r14, 6
141615df8: mov qword ptr [rsp + 0x30], r14
141615dfd: mov qword ptr [rsp + 0x28], rbx
141615e02: mov qword ptr [rsp + 0x20], r12
141615e07: lea r8, [rip + 0x1669aa9]
141615e0e: lea rcx, [rbp - 0x40]
141615e12: mov r9d, 8
141615e18: mov rdx, rdi
141615e1b: call 0x1416a2dc0
141615e20: cmp dword ptr [rbp - 0x40], 0xb
141615e24: jne 0x141615eb4
141615e2a: mov rax, qword ptr [rbp - 0x18]
141615e2e: mov qword ptr [rsi + 0x20], rax
141615e32: movups xmm0, xmmword ptr [rbp - 0x38]
141615e36: movups xmm1, xmmword ptr [rbp - 0x28]
141615e3a: movups xmmword ptr [rsi + 0x10], xmm1
141615e3e: movups xmmword ptr [rsi], xmm0
141615e41: add rsp, 0x100
141615e48: pop rbx
141615e49: pop rdi
141615e4a: pop rsi
141615e4b: pop r12
141615e4d: pop r14
141615e4f: pop r15
141615e51: pop rbp
141615e52: ret 
141615e53: lea rax, [rip + 0x1487396]
141615e5a: mov qword ptr [rbp - 0x40], rax
141615e5e: mov qword ptr [rbp - 0x38], 0x4e
141615e66: lea rax, [rbp - 0x40]
141615e6a: mov qword ptr [rbp + 0x68], rax
141615e6e: lea rax, [rip - 0x150bfe5]
141615e75: mov qword ptr [rbp + 0x70], rax
141615e79: lea rax, [rip + 0x1669948]
141615e80: mov qword ptr [rbp + 0x10], rax
141615e84: mov qword ptr [rbp + 0x18], 1
141615e8c: mov qword ptr [rbp + 0x30], 0
141615e94: lea rax, [rbp + 0x68]
141615e98: mov qword ptr [rbp + 0x20], rax
141615e9c: mov qword ptr [rbp + 0x28], 1
141615ea4: lea rdx, [rip + 0x166992d]
141615eab: lea rcx, [rbp + 0x10]
141615eaf: call 0x1428d9390
141615eb4: movups xmm0, xmmword ptr [rbp]
141615eb8: movaps xmmword ptr [rbp + 0x50], xmm0
141615ebc: movups xmm0, xmmword ptr [rbp - 0x40]
141615ec0: movups xmm1, xmmword ptr [rbp - 0x30]
141615ec4: movups xmm2, xmmword ptr [rbp - 0x20]
141615ec8: movups xmm3, xmmword ptr [rbp - 0x10]
141615ecc: movaps xmmword ptr [rbp + 0x40], xmm3
141615ed0: movaps xmmword ptr [rbp + 0x30], xmm2
141615ed4: movaps xmmword ptr [rbp + 0x20], xmm1
141615ed8: movaps xmmword ptr [rbp + 0x10], xmm0
141615edc: lea rax, [rip + 0x166a2c5]
141615ee3: mov qword ptr [rsp + 0x20], rax
141615ee8: lea rcx, [rip + 0x1668bf9]
141615eef: lea r9, [rip + 0x1668bd2]
141615ef6: lea r8, [rbp + 0x10]
141615efa: mov edx, 0x2b
141615eff: call 0x1428d9760
141615f04: ud2 
141615f06: nop word ptr cs:[rax + rax]
141615f10: mov qword ptr [rsp + 0x10], rdx
141615f15: push rbp
141615f16: push r15
141615f18: push r14
141615f1a: push r12
141615f1c: push rsi
141615f1d: push rdi
141615f1e: push rbx
141615f1f: sub rsp, 0x40
141615f23: lea rbp, [rdx + 0x80]
141615f2a: lea rcx, [rbp + 0x10]
141615f2e: call 0x1407cea70
141615f33: nop 
141615f34: add rsp, 0x40
141615f38: pop rbx
141615f39: pop rdi
141615f3a: pop rsi
141615f3b: pop r12
141615f3d: pop r14
141615f3f: pop r15
141615f41: pop rbp
141615f42: ret 
