141614d30: push rbp
141614d31: push r15
141614d33: push r14
141614d35: push r12
141614d37: push rsi
141614d38: push rdi
141614d39: push rbx
141614d3a: sub rsp, 0x120
141614d41: lea rbp, [rsp + 0x80]
141614d49: mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
141614d54: mov rdi, rdx
141614d57: mov rsi, rcx
141614d5a: test r8, r8
141614d5d: je 0x141614db8
141614d5f: movabs rcx, 0xcccccccccccccccd
141614d69: mov rax, r9
141614d6c: mul rcx
141614d6f: mov rbx, rdx
141614d72: shr rbx, 4
141614d76: lea rax, [rbx*4]
141614d7e: lea rax, [rax + rax*4]
141614d82: mov rcx, r9
141614d85: sub rcx, rax
141614d88: mov qword ptr [rbp - 0x40], rcx
141614d8c: jne 0x141614ec3
141614d92: mov r15, r8
141614d95: mov rax, r8
141614d98: and rax, 3
141614d9c: mov qword ptr [rbp - 0x40], rax
141614da0: jne 0x141614ed4
141614da6: cmp r9, 0x1f4
141614dad: jae 0x141614f5e
141614db3: xor r12d, r12d
141614db6: jmp 0x141614df2
141614db8: movzx eax, byte ptr [rip + 0x201fe4a]
141614dbf: mov ecx, 0x1e0
141614dc4: mov edx, 4
141614dc9: call 0x140613c10
141614dce: test rax, rax
141614dd1: je 0x141614fc8
141614dd7: mov r15, rax
141614dda: mov r8d, 0x1e0
141614de0: mov rcx, rax
141614de3: xor edx, edx
141614de5: call 0x142923c20
141614dea: mov r12b, 1
141614ded: mov ebx, 0x18
141614df2: mov qword ptr [rbp + 0x88], r15
141614df9: mov byte ptr [rbp + 0x97], r12b
141614e00: lea rcx, [rbp + 0x10]
141614e04: mov edx, 0x1e0
141614e09: mov r8d, 0x122
141614e0f: mov r9b, 1
141614e12: call 0x140bd4de0
141614e17: mov qword ptr [rbp + 0x88], r15
141614e1e: mov byte ptr [rbp + 0x97], r12b
141614e25: lea r14, [rbp + 0x68]
141614e29: lea rdx, [rbp + 0x10]
141614e2d: mov r8d, 4
141614e33: mov rcx, r14
141614e36: call 0x140bd4f20
141614e3b: shl rbx, 2
141614e3f: lea rax, [rbx + rbx*4]
141614e43: mov qword ptr [rbp + 0x88], r15
141614e4a: mov byte ptr [rbp + 0x97], r12b
141614e51: mov qword ptr [rsp + 0x30], rax
141614e56: mov qword ptr [rsp + 0x28], r15
141614e5b: mov qword ptr [rsp + 0x20], r14
141614e60: lea r8, [rip + 0x166ad19]
141614e67: lea rcx, [rbp - 0x40]
141614e6b: mov r9d, 0x19
141614e71: mov rdx, rdi
141614e74: call 0x1416a2dc0
141614e79: cmp dword ptr [rbp - 0x40], 0xb
141614e7d: jne 0x141614efe
141614e7f: mov rax, qword ptr [rbp - 0x18]
141614e83: mov qword ptr [rsi + 0x20], rax
141614e87: movups xmm0, xmmword ptr [rbp - 0x38]
141614e8b: movups xmm1, xmmword ptr [rbp - 0x28]
141614e8f: movups xmmword ptr [rsi + 0x10], xmm1
141614e93: movups xmmword ptr [rsi], xmm0
141614e96: test r12b, r12b
141614e99: je 0x141614eae
141614e9b: mov edx, 0x1e0
141614ea0: mov r8d, 4
141614ea6: mov rcx, r15
141614ea9: call 0x140613c20
141614eae: mov rax, rsi
141614eb1: add rsp, 0x120
141614eb8: pop rbx
141614eb9: pop rdi
141614eba: pop rsi
141614ebb: pop r12
141614ebd: pop r14
141614ebf: pop r15
141614ec1: pop rbp
141614ec2: ret 
141614ec3: mov qword ptr [rbp + 0x10], 0
141614ecb: lea rax, [rip + 0x16b5076]
141614ed2: jmp 0x141614ee3
141614ed4: mov qword ptr [rbp + 0x10], 0
141614edc: lea rax, [rip + 0x16b507d]
141614ee3: mov qword ptr [rsp + 0x20], rax
141614ee8: lea r8, [rip + 0x16b5051]
141614eef: lea rdx, [rbp - 0x40]
141614ef3: lea r9, [rbp + 0x10]
141614ef7: xor ecx, ecx
141614ef9: call 0x1428d9596
141614efe: mov dword ptr [rbp + 0x84], r12d
141614f05: mov qword ptr [rbp + 0x78], r15
141614f09: movups xmm0, xmmword ptr [rbp]
141614f0d: movaps xmmword ptr [rbp + 0x50], xmm0
141614f11: movups xmm0, xmmword ptr [rbp - 0x40]
141614f15: movups xmm1, xmmword ptr [rbp - 0x30]
141614f19: movups xmm2, xmmword ptr [rbp - 0x20]
141614f1d: movups xmm3, xmmword ptr [rbp - 0x10]
141614f21: movaps xmmword ptr [rbp + 0x40], xmm3
141614f25: movaps xmmword ptr [rbp + 0x30], xmm2
141614f29: movaps xmmword ptr [rbp + 0x20], xmm1
141614f2d: movaps xmmword ptr [rbp + 0x10], xmm0
141614f31: lea rax, [rip + 0x166b270]
141614f38: mov qword ptr [rsp + 0x20], rax
141614f3d: lea rcx, [rip + 0x1669ba4]
141614f44: lea r9, [rip + 0x1669b7d]
141614f4b: lea r8, [rbp + 0x10]
141614f4f: mov edx, 0x2b
141614f54: call 0x1428d9760
141614f59: jmp 0x141614fe5
141614f5e: lea rax, [rip + 0x148939e]
141614f65: mov qword ptr [rbp - 0x40], rax
141614f69: mov qword ptr [rbp - 0x38], 0x4e
141614f71: lea rax, [rbp - 0x40]
141614f75: mov qword ptr [rbp + 0x68], rax
141614f79: lea rax, [rip - 0x150b0f0]
141614f80: mov qword ptr [rbp + 0x70], rax
141614f84: lea rax, [rip + 0x166a83d]
141614f8b: mov qword ptr [rbp + 0x10], rax
141614f8f: mov qword ptr [rbp + 0x18], 1
141614f97: mov qword ptr [rbp + 0x30], 0
141614f9f: lea rax, [rbp + 0x68]
141614fa3: mov qword ptr [rbp + 0x20], rax
141614fa7: mov qword ptr [rbp + 0x28], 1
141614faf: mov byte ptr [rbp + 0x97], 0
141614fb6: lea rdx, [rip + 0x166a81b]
141614fbd: lea rcx, [rbp + 0x10]
141614fc1: call 0x1428d9390
141614fc6: jmp 0x141614fe5
141614fc8: mov byte ptr [rbp + 0x97], 0
141614fcf: lea r8, [rip + 0x1669ada]
141614fd6: mov ecx, 4
141614fdb: mov edx, 0x1e0
141614fe0: call 0x1428d8fe3
141614fe5: ud2 
141614fe7: nop word ptr [rax + rax]
141614ff0: mov qword ptr [rsp + 0x10], rdx
141614ff5: push rbp
141614ff6: push r15
141614ff8: push r14
141614ffa: push r12
141614ffc: push rsi
141614ffd: push rdi
141614ffe: push rbx
141614fff: sub rsp, 0x40
141615003: lea rbp, [rdx + 0x80]
14161500a: mov rsi, qword ptr [rbp + 0x78]
14161500e: mov qword ptr [rbp + 0x88], rsi
141615015: mov edi, dword ptr [rbp + 0x84]
14161501b: mov byte ptr [rbp + 0x97], dil
141615022: lea rcx, [rbp + 0x10]
141615026: call 0x1407cea70
14161502b: mov qword ptr [rbp + 0x88], rsi
141615032: mov byte ptr [rbp + 0x97], dil
141615039: add rsp, 0x40
14161503d: pop rbx
14161503e: pop rdi
14161503f: pop rsi
141615040: pop r12
141615042: pop r14
141615044: pop r15
141615046: pop rbp
141615047: ret 
141615048: nop dword ptr [rax + rax]
141615050: mov qword ptr [rsp + 0x10], rdx
141615055: push rbp
141615056: push r15
141615058: push r14
14161505a: push r12
14161505c: push rsi
14161505d: push rdi
14161505e: push rbx
14161505f: sub rsp, 0x40
141615063: lea rbp, [rdx + 0x80]
14161506a: test byte ptr [rbp + 0x97], 1
141615071: je 0x14161508a
141615073: mov rcx, qword ptr [rbp + 0x88]
14161507a: mov edx, 0x1e0
14161507f: mov r8d, 4
141615085: call 0x140613c20
14161508a: nop 
14161508b: add rsp, 0x40
14161508f: pop rbx
141615090: pop rdi
141615091: pop rsi
141615092: pop r12
141615094: pop r14
141615096: pop r15
141615098: pop rbp
141615099: ret 
