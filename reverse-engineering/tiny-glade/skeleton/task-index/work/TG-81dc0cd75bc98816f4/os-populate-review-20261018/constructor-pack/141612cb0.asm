141612cb0: push rbp
141612cb1: push r15
141612cb3: push r14
141612cb5: push r12
141612cb7: push rsi
141612cb8: push rdi
141612cb9: push rbx
141612cba: sub rsp, 0x100
141612cc1: lea rbp, [rsp + 0x80]
141612cc9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141612cd1: mov rbx, r8
141612cd4: mov rdi, rdx
141612cd7: mov rsi, rcx
141612cda: test r8, r8
141612cdd: je 0x141612d3b
141612cdf: movabs rcx, 0xaaaaaaaaaaaaaaab
141612ce9: mov rax, r9
141612cec: mul rcx
141612cef: shr rdx, 1
141612cf2: and rdx, 0xfffffffffffffff0
141612cf6: lea rax, [rdx + rdx*2]
141612cfa: mov rcx, r9
141612cfd: sub rcx, rax
141612d00: mov qword ptr [rbp - 0x40], rcx
141612d04: jne 0x141612dcc
141612d0a: mov rax, rbx
141612d0d: and rax, 3
141612d11: mov qword ptr [rbp - 0x40], rax
141612d15: jne 0x141612ddd
141612d1b: cmp r9, 0x1e07b0
141612d22: jae 0x141612e59
141612d28: mov eax, r9d
141612d2b: mov r12d, 0xaaaaaaab
141612d31: imul r12, rax
141612d35: shr r12, 0x25
141612d39: jmp 0x141612d3b
141612d3b: lea r14, [rbp + 0x10]
141612d3f: mov edx, 0x1e0780
141612d44: mov rcx, r14
141612d47: mov r8d, 0x22
141612d4d: mov r9b, 1
141612d50: call 0x140bd4de0
141612d55: lea r15, [rbp + 0x68]
141612d59: mov r8d, 4
141612d5f: mov rcx, r15
141612d62: mov rdx, r14
141612d65: call 0x140bd4f20
141612d6a: shl r12, 4
141612d6e: lea rax, [r12 + r12*2]
141612d72: mov qword ptr [rsp + 0x30], rax
141612d77: mov qword ptr [rsp + 0x28], rbx
141612d7c: mov qword ptr [rsp + 0x20], r15
141612d81: lea r8, [rip + 0x166ccd0]
141612d88: lea rcx, [rbp - 0x40]
141612d8c: mov r9d, 0x11
141612d92: mov rdx, rdi
141612d95: call 0x1416a2dc0
141612d9a: cmp dword ptr [rbp - 0x40], 0xb
141612d9e: jne 0x141612e07
141612da0: mov rax, qword ptr [rbp - 0x18]
141612da4: mov qword ptr [rsi + 0x20], rax
141612da8: movups xmm0, xmmword ptr [rbp - 0x38]
141612dac: movups xmm1, xmmword ptr [rbp - 0x28]
141612db0: movups xmmword ptr [rsi + 0x10], xmm1
141612db4: movups xmmword ptr [rsi], xmm0
141612db7: mov rax, rsi
141612dba: add rsp, 0x100
141612dc1: pop rbx
141612dc2: pop rdi
141612dc3: pop rsi
141612dc4: pop r12
141612dc6: pop r14
141612dc8: pop r15
141612dca: pop rbp
141612dcb: ret 
141612dcc: mov qword ptr [rbp + 0x10], 0
141612dd4: lea rax, [rip + 0x16b716d]
141612ddb: jmp 0x141612dec
141612ddd: mov qword ptr [rbp + 0x10], 0
141612de5: lea rax, [rip + 0x16b7174]
141612dec: mov qword ptr [rsp + 0x20], rax
141612df1: lea r8, [rip + 0x16b7148]
141612df8: lea rdx, [rbp - 0x40]
141612dfc: lea r9, [rbp + 0x10]
141612e00: xor ecx, ecx
141612e02: call 0x1428d9596
141612e07: movups xmm0, xmmword ptr [rbp]
141612e0b: movaps xmmword ptr [rbp + 0x50], xmm0
141612e0f: movups xmm0, xmmword ptr [rbp - 0x40]
141612e13: movups xmm1, xmmword ptr [rbp - 0x30]
141612e17: movups xmm2, xmmword ptr [rbp - 0x20]
141612e1b: movups xmm3, xmmword ptr [rbp - 0x10]
141612e1f: movaps xmmword ptr [rbp + 0x40], xmm3
141612e23: movaps xmmword ptr [rbp + 0x30], xmm2
141612e27: movaps xmmword ptr [rbp + 0x20], xmm1
141612e2b: movaps xmmword ptr [rbp + 0x10], xmm0
141612e2f: lea rax, [rip + 0x166d372]
141612e36: mov qword ptr [rsp + 0x20], rax
141612e3b: lea rcx, [rip + 0x166bca6]
141612e42: lea r9, [rip + 0x166bc7f]
141612e49: lea r8, [rbp + 0x10]
141612e4d: mov edx, 0x2b
141612e52: call 0x1428d9760
141612e57: ud2 
141612e59: lea rax, [rip + 0x148bd64]
141612e60: mov qword ptr [rbp - 0x40], rax
141612e64: mov qword ptr [rbp - 0x38], 0x4f
141612e6c: lea rax, [rbp - 0x40]
141612e70: mov qword ptr [rbp + 0x68], rax
141612e74: lea rax, [rip - 0x1508feb]
141612e7b: mov qword ptr [rbp + 0x70], rax
141612e7f: lea rax, [rip + 0x166c942]
141612e86: mov qword ptr [rbp + 0x10], rax
141612e8a: mov qword ptr [rbp + 0x18], 1
141612e92: mov qword ptr [rbp + 0x30], 0
141612e9a: lea rax, [rbp + 0x68]
141612e9e: mov qword ptr [rbp + 0x20], rax
141612ea2: mov qword ptr [rbp + 0x28], 1
141612eaa: lea rdx, [rip + 0x166c927]
141612eb1: lea rcx, [rbp + 0x10]
141612eb5: call 0x1428d9390
141612eba: int3 
141612ebb: nop dword ptr [rax + rax]
141612ec0: mov qword ptr [rsp + 0x10], rdx
141612ec5: push rbp
141612ec6: push r15
141612ec8: push r14
141612eca: push r12
141612ecc: push rsi
141612ecd: push rdi
141612ece: push rbx
141612ecf: sub rsp, 0x40
141612ed3: lea rbp, [rdx + 0x80]
141612eda: lea rcx, [rbp + 0x10]
141612ede: call 0x1407cea70
141612ee3: nop 
141612ee4: add rsp, 0x40
141612ee8: pop rbx
141612ee9: pop rdi
141612eea: pop rsi
141612eeb: pop r12
141612eed: pop r14
141612eef: pop r15
141612ef1: pop rbp
141612ef2: ret 
