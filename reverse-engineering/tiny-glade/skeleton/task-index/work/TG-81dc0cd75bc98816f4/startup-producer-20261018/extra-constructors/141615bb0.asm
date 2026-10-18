141615bb0: push rbp
141615bb1: push r15
141615bb3: push r14
141615bb5: push r12
141615bb7: push rsi
141615bb8: push rdi
141615bb9: push rbx
141615bba: sub rsp, 0x100
141615bc1: lea rbp, [rsp + 0x80]
141615bc9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141615bd1: mov rsi, rcx
141615bd4: test r8, r8
141615bd7: sete al
141615bda: cmp r9, 0x8001
141615be1: setb cl
141615be4: or cl, al
141615be6: je 0x141615c83
141615bec: mov r14, r9
141615bef: mov rbx, r8
141615bf2: mov rdi, rdx
141615bf5: lea r15, [rbp + 0x10]
141615bf9: mov edx, 0x80000
141615bfe: mov rcx, r15
141615c01: mov r8d, 0x22
141615c07: mov r9b, 1
141615c0a: call 0x140bd4de0
141615c0f: lea r12, [rbp + 0x68]
141615c13: mov r8d, 4
141615c19: mov rcx, r12
141615c1c: mov rdx, r15
141615c1f: call 0x140bd4f20
141615c24: shl r14, 4
141615c28: mov qword ptr [rsp + 0x30], r14
141615c2d: mov qword ptr [rsp + 0x28], rbx
141615c32: mov qword ptr [rsp + 0x20], r12
141615c37: lea r8, [rip + 0x1669c3d]
141615c3e: lea rcx, [rbp - 0x40]
141615c42: mov r9d, 0xb
141615c48: mov rdx, rdi
141615c4b: call 0x1416a2dc0
141615c50: cmp dword ptr [rbp - 0x40], 0xb
141615c54: jne 0x141615ce4
141615c5a: mov rax, qword ptr [rbp - 0x18]
141615c5e: mov qword ptr [rsi + 0x20], rax
141615c62: movups xmm0, xmmword ptr [rbp - 0x38]
141615c66: movups xmm1, xmmword ptr [rbp - 0x28]
141615c6a: movups xmmword ptr [rsi + 0x10], xmm1
141615c6e: movups xmmword ptr [rsi], xmm0
141615c71: add rsp, 0x100
141615c78: pop rbx
141615c79: pop rdi
141615c7a: pop rsi
141615c7b: pop r12
141615c7d: pop r14
141615c7f: pop r15
141615c81: pop rbp
141615c82: ret 
141615c83: lea rax, [rip + 0x1487674]
141615c8a: mov qword ptr [rbp - 0x40], rax
141615c8e: mov qword ptr [rbp - 0x38], 0x35
141615c96: lea rax, [rbp - 0x40]
141615c9a: mov qword ptr [rbp + 0x68], rax
141615c9e: lea rax, [rip - 0x150be15]
141615ca5: mov qword ptr [rbp + 0x70], rax
141615ca9: lea rax, [rip + 0x1669b18]
141615cb0: mov qword ptr [rbp + 0x10], rax
141615cb4: mov qword ptr [rbp + 0x18], 1
141615cbc: mov qword ptr [rbp + 0x30], 0
141615cc4: lea rax, [rbp + 0x68]
141615cc8: mov qword ptr [rbp + 0x20], rax
141615ccc: mov qword ptr [rbp + 0x28], 1
141615cd4: lea rdx, [rip + 0x1669afd]
141615cdb: lea rcx, [rbp + 0x10]
141615cdf: call 0x1428d9390
141615ce4: movups xmm0, xmmword ptr [rbp]
141615ce8: movaps xmmword ptr [rbp + 0x50], xmm0
141615cec: movups xmm0, xmmword ptr [rbp - 0x40]
141615cf0: movups xmm1, xmmword ptr [rbp - 0x30]
141615cf4: movups xmm2, xmmword ptr [rbp - 0x20]
141615cf8: movups xmm3, xmmword ptr [rbp - 0x10]
141615cfc: movaps xmmword ptr [rbp + 0x40], xmm3
141615d00: movaps xmmword ptr [rbp + 0x30], xmm2
141615d04: movaps xmmword ptr [rbp + 0x20], xmm1
141615d08: movaps xmmword ptr [rbp + 0x10], xmm0
141615d0c: lea rax, [rip + 0x166a495]
141615d13: mov qword ptr [rsp + 0x20], rax
141615d18: lea rcx, [rip + 0x1668dc9]
141615d1f: lea r9, [rip + 0x1668da2]
141615d26: lea r8, [rbp + 0x10]
141615d2a: mov edx, 0x2b
141615d2f: call 0x1428d9760
141615d34: ud2 
141615d36: nop word ptr cs:[rax + rax]
141615d40: mov qword ptr [rsp + 0x10], rdx
141615d45: push rbp
141615d46: push r15
141615d48: push r14
141615d4a: push r12
141615d4c: push rsi
141615d4d: push rdi
141615d4e: push rbx
141615d4f: sub rsp, 0x40
141615d53: lea rbp, [rdx + 0x80]
141615d5a: lea rcx, [rbp + 0x10]
141615d5e: call 0x1407cea70
141615d63: nop 
141615d64: add rsp, 0x40
141615d68: pop rbx
141615d69: pop rdi
141615d6a: pop rsi
141615d6b: pop r12
141615d6d: pop r14
141615d6f: pop r15
141615d71: pop rbp
141615d72: ret 
