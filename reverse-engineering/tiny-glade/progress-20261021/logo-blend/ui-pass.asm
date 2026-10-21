0x140837c40: push rbp
0x140837c41: push r15
0x140837c43: push r14
0x140837c45: push r13
0x140837c47: push r12
0x140837c49: push rsi
0x140837c4a: push rdi
0x140837c4b: push rbx
0x140837c4c: sub rsp, 0x938
0x140837c53: lea rbp, [rsp + 0x80]
0x140837c5b: mov qword ptr [rbp + 0x8b0], 0xfffffffffffffffe
0x140837c66: mov rbx, r9
0x140837c69: mov rdi, r8
0x140837c6c: mov rsi, rdx
0x140837c6f: mov r14, rcx
0x140837c72: lea rcx, [r8 + 4]
0x140837c76: call 0x140bd5790
0x140837c7b: mov dword ptr [rbp + 0x3b8], 0
0x140837c85: mov qword ptr [rbp + 0x3c0], rdi
0x140837c8c: mov dword ptr [rbp + 0x7a0], 2
0x140837c96: mov qword ptr [rsp + 0x40], r14
0x140837c9b: mov qword ptr [rsp + 0x38], rsi
0x140837ca0: mov qword ptr [rsp + 0x30], rax
0x140837ca5: lea rax, [rbp + 0x7a0]
0x140837cac: mov qword ptr [rsp + 0x28], rax
0x140837cb1: mov qword ptr [rsp + 0x20], 1
0x140837cba: lea rdx, [rip + 0x225af1f]
0x140837cc1: lea rcx, [rbp - 0x30]
0x140837cc5: lea r9, [rbp + 0x3b8]
0x140837ccc: mov r8d, 2
0x140837cd2: call 0x140867970
0x140837cd7: mov qword ptr [rbp + 0x8a0], rsi
0x140837cde: lea rcx, [rsi + 0x38]
0x140837ce2: mov byte ptr [rbp + 0x8af], 1
0x140837ce9: movabs rdx, 0x4bc362195b80f729
0x140837cf3: movabs r8, 0x982798ce80cbcefb
0x140837cfd: call 0x1400dd6f0
0x140837d02: test rax, rax
0x140837d05: mov r8, qword ptr [rbp + 0x8a0]
0x140837d0c: je 0x140837f7d
0x140837d12: cmp qword ptr [r8 + 0x218], rdx
0x140837d19: jbe 0x140837f7d
0x140837d1f: mov rax, qword ptr [r8 + 0x210]
0x140837d26: mov rcx, qword ptr [rax + rdx*8]
0x140837d2a: test rcx, rcx
0x140837d2d: je 0x140837f7d
0x140837d33: mov rax, qword ptr [r8 + 0x1e0]
0x140837d3a: not rcx
0x140837d3d: lea rcx, [rcx + rcx*2]
0x140837d41: shl rcx, 5
0x140837d45: cmp qword ptr [rax + rcx + 0x20], 0
0x140837d4b: je 0x140837f7d
0x140837d51: add rax, rcx
0x140837d54: mov rax, qword ptr [rax + 0x10]
0x140837d58: mov rcx, qword ptr [rax]
0x140837d5b: test rcx, rcx
0x140837d5e: js 0x140837d67
0x140837d60: cvtsi2ss xmm0, rcx
0x140837d65: jmp 0x140837d7c
0x140837d67: mov rdx, rcx
0x140837d6a: shr rdx, 1
0x140837d6d: and ecx, 1
0x140837d70: or rcx, rdx
0x140837d73: cvtsi2ss xmm0, rcx
0x140837d78: addss xmm0, xmm0
0x140837d7c: cvtsi2ss xmm1, dword ptr [rax + 8]
0x140837d81: divss xmm1, dword ptr [rip + 0x216f8b7]
0x140837d89: addss xmm1, xmm0
0x140837d8d: movss dword ptr [rbp + 0x898], xmm1
0x140837d95: mov byte ptr [rbp + 0x89c], 1
0x140837d9c: mov rdi, qword ptr [rbx + 0x1d0]
0x140837da3: mov r15, qword ptr [rbx + 0x1d8]
0x140837daa: shl r15, 7
0x140837dae: add r15, rdi
0x140837db1: lea rbx, [rbp + 0x3b8]
0x140837db8: lea r14, [rbp + 0x7a0]
0x140837dbf: cmp rdi, r15
0x140837dc2: je 0x140837f2b
0x140837dc8: nop dword ptr [rax + rax]
0x140837dd0: mov r12, qword ptr [rdi + 0x50]
0x140837dd4: mov rsi, qword ptr [rdi + 0x58]
0x140837dd8: mov byte ptr [rbp + 0x8af], 1
0x140837ddf: call 0x140be6750
0x140837de4: mov r13d, eax
0x140837de7: mov byte ptr [rbp + 0x8af], 1
0x140837dee: xor ecx, ecx
0x140837df0: call 0x1400d9b20
0x140837df5: test rax, rax
0x140837df8: je 0x140837f68
0x140837dfe: mov dword ptr [rbp + 0x848], r13d
0x140837e05: mov word ptr [rbp + 0x850], 0
0x140837e0e: mov dword ptr [rbp + 0x84c], 0x3f800000
0x140837e18: mov qword ptr [rbp + 0x7a0], 1
0x140837e23: mov qword ptr [rbp + 0x7a8], r12
0x140837e2a: mov qword ptr [rbp + 0x840], rsi
0x140837e31: mov qword ptr [rbp + 0x7b0], 0
0x140837e3c: mov qword ptr [rbp + 0x7b8], 4
0x140837e47: mov qword ptr [rbp + 0x7c0], 0
0x140837e52: movups xmm0, xmmword ptr [rax]
0x140837e55: inc qword ptr [rax]
0x140837e58: movups xmm1, xmmword ptr [rip + 0x225e8d1]
0x140837e5f: lea rax, [rbp + 0x810]
0x140837e66: movups xmmword ptr [rax + 0x10], xmm1
0x140837e6a: movups xmm1, xmmword ptr [rip + 0x225e8af]
0x140837e71: movups xmmword ptr [rax], xmm1
0x140837e74: movups xmmword ptr [rbp + 0x830], xmm0
0x140837e7b: movabs rax, 0x100000003
0x140837e85: mov qword ptr [rbp + 0x7e0], rax
0x140837e8c: mov qword ptr [rbp + 0x7c8], 0
0x140837e97: mov qword ptr [rbp + 0x7d0], 1
0x140837ea2: mov qword ptr [rbp + 0x7d8], 0
0x140837ead: mov byte ptr [rbp + 0x8af], 1
0x140837eb4: mov rcx, rbx
0x140837eb7: mov rdx, r14
0x140837eba: lea r8, [rbp - 0x30]
0x140837ebe: xor r9d, r9d
0x140837ec1: call 0x140905be0
0x140837ec6: mov byte ptr [rbp + 0x8af], 1
0x140837ecd: mov rcx, r14
0x140837ed0: mov rdx, rbx
0x140837ed3: lea r8, [rbp + 0x898]
0x140837eda: mov r9, rdi
0x140837edd: call 0x14090b460
0x140837ee2: movzx eax, byte ptr [rbp + 0x84b]
0x140837ee9: and al, 0x9f
0x140837eeb: or al, 0x40
0x140837eed: mov byte ptr [rbp + 0x84b], al
0x140837ef3: mov rsi, qword ptr [rdi + 0x70]
0x140837ef7: mov r8d, 0xf8
0x140837efd: mov rcx, rbx
0x140837f00: mov rdx, r14
0x140837f03: call 0x142923c00
0x140837f08: mov byte ptr [rbp + 0x8af], 1
0x140837f0f: mov rcx, rbx
0x140837f12: mov rdx, rdi
0x140837f15: mov r8, qword ptr [rbp + 0x8a0]
0x140837f1c: call rsi
0x140837f1e: sub rdi, -0x80
0x140837f22: cmp rdi, r15
0x140837f25: jne 0x140837dd0
0x140837f2b: lea rsi, [rbp + 0x3b8]
0x140837f32: lea rdx, [rbp - 0x30]
0x140837f36: mov r8d, 0x3e8
0x140837f3c: mov rcx, rsi
0x140837f3f: call 0x142923c00
0x140837f44: mov byte ptr [rbp + 0x8af], 0
0x140837f4b: mov rcx, rsi
0x140837f4e: call 0x140868370
0x140837f53: nop 
0x140837f54: add rsp, 0x938
0x140837f5b: pop rbx
0x140837f5c: pop rdi
0x140837f5d: pop rsi
0x140837f5e: pop r12
0x140837f60: pop r13
0x140837f62: pop r14
0x140837f64: pop r15
0x140837f66: pop rbp
0x140837f67: ret 
0x140837f68: mov byte ptr [rbp + 0x8af], 1
0x140837f6f: lea rcx, [rip + 0x2402b2a]
0x140837f76: call 0x1428d80d0
0x140837f7b: jmp 0x140837f90
0x140837f7d: mov byte ptr [rbp + 0x8af], 1
0x140837f84: lea rcx, [rip + 0x226dc4d]
0x140837f8b: call 0x1428d9310
0x140837f90: ud2 
0x140837f92: nop word ptr cs:[rax + rax]
0x140837fa0: mov qword ptr [rsp + 0x10], rdx
0x140837fa5: push rbp
0x140837fa6: push r15
0x140837fa8: push r14
0x140837faa: push r13
0x140837fac: push r12
0x140837fae: push rsi
0x140837faf: push rdi
0x140837fb0: push rbx
0x140837fb1: sub rsp, 0x48
0x140837fb5: lea rbp, [rdx + 0x80]
0x140837fbc: cmp byte ptr [rbp + 0x8af], 0
0x140837fc3: jne 0x140837fd6
0x140837fc5: add rsp, 0x48
0x140837fc9: pop rbx
0x140837fca: pop rdi
0x140837fcb: pop rsi
0x140837fcc: pop r12
0x140837fce: pop r13
0x140837fd0: pop r14
0x140837fd2: pop r15
0x140837fd4: pop rbp
0x140837fd5: ret 
0x140837fd6: lea rcx, [rbp - 0x30]
0x140837fda: call 0x1408923c0
0x140837fdf: jmp 0x140837fc5
