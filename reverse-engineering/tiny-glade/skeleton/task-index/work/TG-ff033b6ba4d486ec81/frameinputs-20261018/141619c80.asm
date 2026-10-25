141619c80: push rbp
141619c81: push r15
141619c83: push r14
141619c85: push r12
141619c87: push rsi
141619c88: push rdi
141619c89: push rbx
141619c8a: sub rsp, 0x100
141619c91: lea rbp, [rsp + 0x80]
141619c99: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141619ca1: mov rsi, rcx
141619ca4: test r8, r8
141619ca7: sete al
141619caa: cmp r9, 0x20001
141619cb1: setb cl
141619cb4: or cl, al
141619cb6: je 0x141619d56
141619cbc: mov r14, r9
141619cbf: mov rbx, r8
141619cc2: mov rdi, rdx
141619cc5: lea r15, [rbp + 0x10]
141619cc9: mov edx, 0x800000
141619cce: mov rcx, r15
141619cd1: mov r8d, 0x22
141619cd7: mov r9b, 1
141619cda: call 0x140bd4de0
141619cdf: lea r12, [rbp + 0x68]
141619ce3: mov r8d, 4
141619ce9: mov rcx, r12
141619cec: mov rdx, r15
141619cef: call 0x140bd4f20
141619cf4: shl r14, 6
141619cf8: mov qword ptr [rsp + 0x30], r14
141619cfd: mov qword ptr [rsp + 0x28], rbx
141619d02: mov qword ptr [rsp + 0x20], r12
141619d07: lea r8, [rip + 0x1666253]
141619d0e: lea rcx, [rbp - 0x40]
141619d12: mov r9d, 0xb
141619d18: mov rdx, rdi
141619d1b: call 0x1416a2dc0
141619d20: cmp dword ptr [rbp - 0x40], 0xb
141619d24: jne 0x141619db7
141619d2a: mov rax, qword ptr [rbp - 0x18]
141619d2e: mov qword ptr [rsi + 0x20], rax
141619d32: movups xmm0, xmmword ptr [rbp - 0x38]
141619d36: movups xmm1, xmmword ptr [rbp - 0x28]
141619d3a: movups xmmword ptr [rsi + 0x10], xmm1
141619d3e: movups xmmword ptr [rsi], xmm0
141619d41: mov rax, rsi
141619d44: add rsp, 0x100
141619d4b: pop rbx
141619d4c: pop rdi
141619d4d: pop rsi
141619d4e: pop r12
141619d50: pop r14
141619d52: pop r15
141619d54: pop rbp
141619d55: ret 
141619d56: lea rax, [rip + 0x14840dc]
141619d5d: mov qword ptr [rbp - 0x40], rax
141619d61: mov qword ptr [rbp - 0x38], 0x3d
141619d69: lea rax, [rbp - 0x40]
141619d6d: mov qword ptr [rbp + 0x68], rax
141619d71: lea rax, [rip - 0x150fee8]
141619d78: mov qword ptr [rbp + 0x70], rax
141619d7c: lea rax, [rip + 0x1665a45]
141619d83: mov qword ptr [rbp + 0x10], rax
141619d87: mov qword ptr [rbp + 0x18], 1
141619d8f: mov qword ptr [rbp + 0x30], 0
141619d97: lea rax, [rbp + 0x68]
141619d9b: mov qword ptr [rbp + 0x20], rax
141619d9f: mov qword ptr [rbp + 0x28], 1
141619da7: lea rdx, [rip + 0x1665a2a]
141619dae: lea rcx, [rbp + 0x10]
141619db2: call 0x1428d9390
141619db7: movups xmm0, xmmword ptr [rbp]
141619dbb: movaps xmmword ptr [rbp + 0x50], xmm0
141619dbf: movups xmm0, xmmword ptr [rbp - 0x40]
141619dc3: movups xmm1, xmmword ptr [rbp - 0x30]
141619dc7: movups xmm2, xmmword ptr [rbp - 0x20]
141619dcb: movups xmm3, xmmword ptr [rbp - 0x10]
141619dcf: movaps xmmword ptr [rbp + 0x40], xmm3
141619dd3: movaps xmmword ptr [rbp + 0x30], xmm2
141619dd7: movaps xmmword ptr [rbp + 0x20], xmm1
141619ddb: movaps xmmword ptr [rbp + 0x10], xmm0
141619ddf: lea rax, [rip + 0x16663c2]
141619de6: mov qword ptr [rsp + 0x20], rax
141619deb: lea rcx, [rip + 0x1664cf6]
141619df2: lea r9, [rip + 0x1664ccf]
141619df9: lea r8, [rbp + 0x10]
141619dfd: mov edx, 0x2b
141619e02: call 0x1428d9760
141619e07: ud2 
141619e09: nop dword ptr [rax]
141619e10: mov qword ptr [rsp + 0x10], rdx
141619e15: push rbp
141619e16: push r15
141619e18: push r14
141619e1a: push r12
141619e1c: push rsi
141619e1d: push rdi
141619e1e: push rbx
141619e1f: sub rsp, 0x40
141619e23: lea rbp, [rdx + 0x80]
141619e2a: lea rcx, [rbp + 0x10]
141619e2e: call 0x1407cea70
141619e33: nop 
141619e34: add rsp, 0x40
141619e38: pop rbx
141619e39: pop rdi
141619e3a: pop rsi
141619e3b: pop r12
141619e3d: pop r14
141619e3f: pop r15
141619e41: pop rbp
141619e42: ret 
