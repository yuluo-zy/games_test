1403c7a30: push rbp
1403c7a31: push r14
1403c7a33: push rsi
1403c7a34: push rdi
1403c7a35: push rbx
1403c7a36: sub rsp, 0x2f0
1403c7a3d: lea rbp, [rsp + 0x80]
1403c7a45: mov qword ptr [rbp + 0x268], 0xfffffffffffffffe
1403c7a50: mov ebx, edx
1403c7a52: mov rsi, rcx
1403c7a55: movzx eax, byte ptr [rip + 0x326d1ad]
1403c7a5c: mov ecx, 0x150
1403c7a61: mov edx, 8
1403c7a66: call 0x140613c10
1403c7a6b: test rax, rax
1403c7a6e: je 0x1403c7dcf
1403c7a74: mov qword ptr [rbp + 0x260], rax
1403c7a7b: xorps xmm0, xmm0
1403c7a7e: movups xmmword ptr [rbp + 0x140], xmm0
1403c7a85: movups xmmword ptr [rbp + 0x158], xmm0
1403c7a8c: movups xmmword ptr [rbp + 0x170], xmm0
1403c7a93: movups xmmword ptr [rbp + 0x188], xmm0
1403c7a9a: movups xmmword ptr [rbp + 0x1a0], xmm0
1403c7aa1: mov dword ptr [rbp + 0x1b0], 0
1403c7aab: movups xmmword ptr [rbp + 0x1d8], xmm0
1403c7ab2: movups xmmword ptr [rbp + 0x1f0], xmm0
1403c7ab9: movups xmmword ptr [rbp + 0x208], xmm0
1403c7ac0: movups xmmword ptr [rbp + 0x220], xmm0
1403c7ac7: movups xmmword ptr [rbp + 0x238], xmm0
1403c7ace: mov dword ptr [rbp + 0x248], 0
1403c7ad8: mov qword ptr [rbp + 0x80], 0
1403c7ae3: mov qword ptr [rbp + 0x120], 0
1403c7aee: mov qword ptr [rbp + 0x128], 8
1403c7af9: mov qword ptr [rbp + 0x130], 0
1403c7b04: mov qword ptr [rbp + 0x138], 0x10
1403c7b0f: mov qword ptr [rbp + 0x150], 0x10
1403c7b1a: mov qword ptr [rbp + 0x168], 0x10
1403c7b25: mov qword ptr [rbp + 0x180], 0x10
1403c7b30: mov qword ptr [rbp + 0x198], 0x10
1403c7b3b: movabs r14, 0x8000000000000000
1403c7b45: mov qword ptr [rbp + 0x1b8], r14
1403c7b4c: lea rax, [rip + 0x2608102]
1403c7b53: mov qword ptr [rbp + 0x1c0], rax
1403c7b5a: mov qword ptr [rbp + 0x1c8], 0x50
1403c7b65: mov qword ptr [rbp + 0x1d0], 0x10
1403c7b70: mov qword ptr [rbp + 0x1e8], 0x10
1403c7b7b: mov qword ptr [rbp + 0x200], 0x10
1403c7b86: mov qword ptr [rbp + 0x218], 0x10
1403c7b91: mov qword ptr [rbp + 0x230], 0x10
1403c7b9c: mov dword ptr [rbp + 0x250], 0
1403c7ba6: mov word ptr [rbp + 0x254], 1
1403c7baf: mov dword ptr [rbp + 0x258], 0
1403c7bb9: movzx eax, byte ptr [rip + 0x326d049]
1403c7bc0: mov ecx, 0x1e0
1403c7bc5: mov edx, 8
1403c7bca: call 0x140613c10
1403c7bcf: test rax, rax
1403c7bd2: je 0x1403c7dde
1403c7bd8: mov rdi, rax
1403c7bdb: lea rdx, [rbp + 0x80]
1403c7be2: mov r8d, 0x1e0
1403c7be8: mov rcx, rax
1403c7beb: call 0x142923c00
1403c7bf0: lea r8, [rip + 0x2607f89]
1403c7bf7: lea rcx, [rbp - 0x60]
1403c7bfb: mov rdx, rdi
1403c7bfe: call 0x140ce4b10
1403c7c03: lea rcx, [rbp + 0x10]
1403c7c07: call 0x1400cd930
1403c7c0c: lea rcx, [rbp + 0x80]
1403c7c13: call 0x1400cdb00
1403c7c18: movups xmm0, xmmword ptr [rbp]
1403c7c1c: mov rax, qword ptr [rbp + 0x260]
1403c7c23: movups xmmword ptr [rax + 0x60], xmm0
1403c7c27: movups xmm0, xmmword ptr [rbp - 0x10]
1403c7c2b: movups xmmword ptr [rax + 0x50], xmm0
1403c7c2f: movups xmm0, xmmword ptr [rbp - 0x20]
1403c7c33: movups xmmword ptr [rax + 0x40], xmm0
1403c7c37: movups xmm0, xmmword ptr [rbp - 0x60]
1403c7c3b: movups xmm1, xmmword ptr [rbp - 0x50]
1403c7c3f: movups xmm2, xmmword ptr [rbp - 0x40]
1403c7c43: movups xmm3, xmmword ptr [rbp - 0x30]
1403c7c47: movups xmmword ptr [rax + 0x30], xmm3
1403c7c4b: movups xmmword ptr [rax + 0x20], xmm2
1403c7c4f: movups xmmword ptr [rax + 0x10], xmm1
1403c7c53: movups xmmword ptr [rax], xmm0
1403c7c56: movups xmm0, xmmword ptr [rbp + 0x10]
1403c7c5a: movups xmm1, xmmword ptr [rbp + 0x20]
1403c7c5e: movups xmm2, xmmword ptr [rbp + 0x30]
1403c7c62: movups xmm3, xmmword ptr [rbp + 0x40]
1403c7c66: movups xmmword ptr [rax + 0x70], xmm0
1403c7c6a: movups xmmword ptr [rax + 0x80], xmm1
1403c7c71: movups xmmword ptr [rax + 0x90], xmm2
1403c7c78: movups xmmword ptr [rax + 0xa0], xmm3
1403c7c7f: movups xmm0, xmmword ptr [rbp + 0x50]
1403c7c83: movups xmmword ptr [rax + 0xb0], xmm0
1403c7c8a: movups xmm0, xmmword ptr [rbp + 0x60]
1403c7c8e: movups xmmword ptr [rax + 0xc0], xmm0
1403c7c95: movups xmm0, xmmword ptr [rbp + 0x70]
1403c7c99: movups xmmword ptr [rax + 0xd0], xmm0
1403c7ca0: movups xmm0, xmmword ptr [rbp + 0x80]
1403c7ca7: movups xmm1, xmmword ptr [rbp + 0x90]
1403c7cae: movups xmm2, xmmword ptr [rbp + 0xa0]
1403c7cb5: movups xmm3, xmmword ptr [rbp + 0xb0]
1403c7cbc: movups xmmword ptr [rax + 0xe0], xmm0
1403c7cc3: movups xmmword ptr [rax + 0xf0], xmm1
1403c7cca: movups xmmword ptr [rax + 0x100], xmm2
1403c7cd1: movups xmmword ptr [rax + 0x110], xmm3
1403c7cd8: movups xmm0, xmmword ptr [rbp + 0xc0]
1403c7cdf: movups xmmword ptr [rax + 0x120], xmm0
1403c7ce6: movups xmm0, xmmword ptr [rbp + 0xd0]
1403c7ced: movups xmmword ptr [rax + 0x130], xmm0
1403c7cf4: movups xmm0, xmmword ptr [rbp + 0xe0]
1403c7cfb: movups xmmword ptr [rax + 0x140], xmm0
1403c7d02: mov qword ptr [rbp + 0x88], 3
1403c7d0d: mov qword ptr [rbp + 0x90], rax
1403c7d14: mov qword ptr [rbp + 0x98], 3
1403c7d1f: mov qword ptr [rbp + 0xa0], 0
1403c7d2a: mov qword ptr [rbp + 0xa8], 8
1403c7d35: xorps xmm0, xmm0
1403c7d38: movups xmmword ptr [rbp + 0xb0], xmm0
1403c7d3f: mov qword ptr [rbp + 0x80], r14
1403c7d46: mov byte ptr [rbp + 0x10], bl
1403c7d49: mov rcx, qword ptr [rip + 0x3264620]
1403c7d50: lea r8, [rip + 0x2563241]
1403c7d57: lea rdx, [rbp + 0x10]
1403c7d5b: call 0x140364d20
1403c7d60: mov r8, rdx
1403c7d63: lea rcx, [rbp + 0x80]
1403c7d6a: mov rdx, rax
1403c7d6d: call 0x1400d68c0
1403c7d72: movups xmm0, xmmword ptr [rbp + 0xe0]
1403c7d79: movups xmmword ptr [rsi + 0x60], xmm0
1403c7d7d: movups xmm0, xmmword ptr [rbp + 0xd0]
1403c7d84: movups xmmword ptr [rsi + 0x50], xmm0
1403c7d88: movups xmm0, xmmword ptr [rbp + 0xc0]
1403c7d8f: movups xmmword ptr [rsi + 0x40], xmm0
1403c7d93: movups xmm0, xmmword ptr [rbp + 0x80]
1403c7d9a: movups xmm1, xmmword ptr [rbp + 0x90]
1403c7da1: movups xmm2, xmmword ptr [rbp + 0xa0]
1403c7da8: movups xmm3, xmmword ptr [rbp + 0xb0]
1403c7daf: movups xmmword ptr [rsi + 0x30], xmm3
1403c7db3: movups xmmword ptr [rsi + 0x20], xmm2
1403c7db7: movups xmmword ptr [rsi + 0x10], xmm1
1403c7dbb: movups xmmword ptr [rsi], xmm0
1403c7dbe: mov rax, rsi
1403c7dc1: add rsp, 0x2f0
1403c7dc8: pop rbx
1403c7dc9: pop rdi
1403c7dca: pop rsi
1403c7dcb: pop r14
1403c7dcd: pop rbp
1403c7dce: ret 
1403c7dcf: mov ecx, 8
1403c7dd4: mov edx, 0x150
1403c7dd9: call 0x1428d9000
1403c7dde: mov ecx, 8
1403c7de3: mov edx, 0x1e0
1403c7de8: call 0x1428d9000
1403c7ded: ud2 
1403c7def: nop 
1403c7df0: mov qword ptr [rsp + 0x10], rdx
1403c7df5: push rbp
1403c7df6: push r14
1403c7df8: push rsi
1403c7df9: push rdi
1403c7dfa: push rbx
1403c7dfb: sub rsp, 0x20
1403c7dff: lea rbp, [rdx + 0x80]
1403c7e06: lea rcx, [rbp + 0x80]
1403c7e0d: call 0x1405dcec0
1403c7e12: nop 
1403c7e13: add rsp, 0x20
1403c7e17: pop rbx
1403c7e18: pop rdi
1403c7e19: pop rsi
1403c7e1a: pop r14
1403c7e1c: pop rbp
1403c7e1d: ret 
1403c7e1e: nop 
1403c7e20: mov qword ptr [rsp + 0x10], rdx
1403c7e25: push rbp
1403c7e26: push r14
1403c7e28: push rsi
1403c7e29: push rdi
1403c7e2a: push rbx
1403c7e2b: sub rsp, 0x20
1403c7e2f: lea rbp, [rdx + 0x80]
1403c7e36: mov edx, 0x150
1403c7e3b: mov r8d, 8
1403c7e41: mov rcx, qword ptr [rbp + 0x260]
1403c7e48: call 0x140613c20
1403c7e4d: nop 
1403c7e4e: add rsp, 0x20
1403c7e52: pop rbx
1403c7e53: pop rdi
1403c7e54: pop rsi
1403c7e55: pop r14
1403c7e57: pop rbp
1403c7e58: ret 
1403c7e59: nop dword ptr [rax]
1403c7e60: mov qword ptr [rsp + 0x10], rdx
1403c7e65: push rbp
1403c7e66: push r14
1403c7e68: push rsi
1403c7e69: push rdi
1403c7e6a: push rbx
1403c7e6b: sub rsp, 0x20
1403c7e6f: lea rbp, [rdx + 0x80]
1403c7e76: lea rcx, [rbp - 0x60]
1403c7e7a: call 0x140029880
1403c7e7f: nop 
1403c7e80: add rsp, 0x20
1403c7e84: pop rbx
1403c7e85: pop rdi
1403c7e86: pop rsi
1403c7e87: pop r14
1403c7e89: pop rbp
1403c7e8a: ret 
1403c7e8b: nop dword ptr [rax + rax]
1403c7e90: mov qword ptr [rsp + 0x10], rdx
1403c7e95: push rbp
1403c7e96: push r14
1403c7e98: push rsi
1403c7e99: push rdi
1403c7e9a: push rbx
1403c7e9b: sub rsp, 0x20
1403c7e9f: lea rbp, [rdx + 0x80]
1403c7ea6: lea rcx, [rbp + 0x10]
1403c7eaa: call 0x140029880
1403c7eaf: nop 
1403c7eb0: add rsp, 0x20
1403c7eb4: pop rbx
1403c7eb5: pop rdi
1403c7eb6: pop rsi
1403c7eb7: pop r14
1403c7eb9: pop rbp
1403c7eba: ret 
1403c7ebb: nop dword ptr [rax + rax]
1403c7ec0: mov qword ptr [rsp + 0x10], rdx
1403c7ec5: push rbp
1403c7ec6: push r14
1403c7ec8: push rsi
1403c7ec9: push rdi
1403c7eca: push rbx
1403c7ecb: sub rsp, 0x20
1403c7ecf: lea rbp, [rdx + 0x80]
1403c7ed6: lea rcx, [rbp + 0x80]
1403c7edd: call 0x140029880
1403c7ee2: nop 
1403c7ee3: add rsp, 0x20
1403c7ee7: pop rbx
1403c7ee8: pop rdi
1403c7ee9: pop rsi
1403c7eea: pop r14
1403c7eec: pop rbp
1403c7eed: ret 
