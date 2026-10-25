140324d10: push rbp
140324d11: push rsi
140324d12: push rdi
140324d13: push rbx
140324d14: sub rsp, 0x238
140324d1b: lea rbp, [rsp + 0x80]
140324d23: mov qword ptr [rbp + 0x1b0], 0xfffffffffffffffe
140324d2e: mov rsi, rcx
140324d31: movzx eax, byte ptr [rip + 0x330fed1]
140324d38: mov ecx, 0xe0
140324d3d: mov edx, 8
140324d42: call 0x140613c10
140324d47: test rax, rax
140324d4a: je 0x140324f88
140324d50: mov qword ptr [rbp + 0x1a8], rax
140324d57: xorps xmm0, xmm0
140324d5a: movups xmmword ptr [rbp + 0x38], xmm0
140324d5e: movups xmmword ptr [rbp + 0x50], xmm0
140324d62: movups xmmword ptr [rbp + 0x68], xmm0
140324d66: movups xmmword ptr [rbp + 0x80], xmm0
140324d6d: movups xmmword ptr [rbp + 0x98], xmm0
140324d74: mov dword ptr [rbp + 0xa8], 0
140324d7e: movups xmmword ptr [rbp + 0xd0], xmm0
140324d85: movups xmmword ptr [rbp + 0xe8], xmm0
140324d8c: movups xmmword ptr [rbp + 0x100], xmm0
140324d93: movups xmmword ptr [rbp + 0x118], xmm0
140324d9a: movups xmmword ptr [rbp + 0x130], xmm0
140324da1: mov dword ptr [rbp + 0x140], 0
140324dab: mov qword ptr [rbp + 0x18], 0
140324db3: mov qword ptr [rbp + 0x20], 8
140324dbb: mov qword ptr [rbp + 0x28], 0
140324dc3: mov qword ptr [rbp + 0x30], 0x10
140324dcb: mov qword ptr [rbp + 0x48], 0x10
140324dd3: mov qword ptr [rbp + 0x60], 0x10
140324ddb: mov qword ptr [rbp + 0x78], 0x10
140324de3: mov qword ptr [rbp + 0x90], 0x10
140324dee: movabs rbx, 0x8000000000000000
140324df8: mov qword ptr [rbp + 0xb0], rbx
140324dff: lea rax, [rip + 0x2699170]
140324e06: mov qword ptr [rbp + 0xb8], rax
140324e0d: mov qword ptr [rbp + 0xc0], 0x39
140324e18: mov qword ptr [rbp + 0xc8], 0x10
140324e23: mov qword ptr [rbp + 0xe0], 0x10
140324e2e: mov qword ptr [rbp + 0xf8], 0x10
140324e39: mov qword ptr [rbp + 0x110], 0x10
140324e44: mov qword ptr [rbp + 0x128], 0x10
140324e4f: mov dword ptr [rbp + 0x148], 0
140324e59: mov word ptr [rbp + 0x14c], 1
140324e62: mov qword ptr [rbp + 0x160], rbx
140324e69: mov dword ptr [rbp + 0x1a0], 0
140324e73: movzx eax, byte ptr [rip + 0x330fd8f]
140324e7a: mov ecx, 0x190
140324e7f: mov edx, 8
140324e84: call 0x140613c10
140324e89: test rax, rax
140324e8c: je 0x140324f97
140324e92: mov rdi, rax
140324e95: lea rdx, [rbp + 0x18]
140324e99: mov r8d, 0x190
140324e9f: mov rcx, rax
140324ea2: call 0x142923c00
140324ea7: lea r8, [rip + 0x2698d7a]
140324eae: lea rcx, [rbp - 0x58]
140324eb2: mov rdx, rdi
140324eb5: call 0x140ce4b10
140324eba: lea rcx, [rbp + 0x18]
140324ebe: call 0x1403c0c90
140324ec3: movups xmm0, xmmword ptr [rbp + 8]
140324ec7: mov rax, qword ptr [rbp + 0x1a8]
140324ece: movups xmmword ptr [rax + 0x60], xmm0
140324ed2: movups xmm0, xmmword ptr [rbp - 8]
140324ed6: movups xmmword ptr [rax + 0x50], xmm0
140324eda: movups xmm0, xmmword ptr [rbp - 0x18]
140324ede: movups xmmword ptr [rax + 0x40], xmm0
140324ee2: movups xmm0, xmmword ptr [rbp - 0x58]
140324ee6: movups xmm1, xmmword ptr [rbp - 0x48]
140324eea: movups xmm2, xmmword ptr [rbp - 0x38]
140324eee: movups xmm3, xmmword ptr [rbp - 0x28]
140324ef2: movups xmmword ptr [rax + 0x30], xmm3
140324ef6: movups xmmword ptr [rax + 0x20], xmm2
140324efa: movups xmmword ptr [rax + 0x10], xmm1
140324efe: movups xmmword ptr [rax], xmm0
140324f01: movups xmm0, xmmword ptr [rbp + 0x18]
140324f05: movups xmm1, xmmword ptr [rbp + 0x28]
140324f09: movups xmm2, xmmword ptr [rbp + 0x38]
140324f0d: movups xmm3, xmmword ptr [rbp + 0x48]
140324f11: movups xmmword ptr [rax + 0x70], xmm0
140324f15: movups xmmword ptr [rax + 0x80], xmm1
140324f1c: movups xmmword ptr [rax + 0x90], xmm2
140324f23: movups xmmword ptr [rax + 0xa0], xmm3
140324f2a: movups xmm0, xmmword ptr [rbp + 0x58]
140324f2e: movups xmmword ptr [rax + 0xb0], xmm0
140324f35: movups xmm0, xmmword ptr [rbp + 0x68]
140324f39: movups xmmword ptr [rax + 0xc0], xmm0
140324f40: movups xmm0, xmmword ptr [rbp + 0x78]
140324f44: movups xmmword ptr [rax + 0xd0], xmm0
140324f4b: mov qword ptr [rsi + 8], 2
140324f53: mov qword ptr [rsi + 0x10], rax
140324f57: mov qword ptr [rsi + 0x18], 2
140324f5f: mov qword ptr [rsi + 0x20], 0
140324f67: mov qword ptr [rsi + 0x28], 8
140324f6f: xorps xmm0, xmm0
140324f72: movups xmmword ptr [rsi + 0x30], xmm0
140324f76: mov qword ptr [rsi], rbx
140324f79: mov rax, rsi
140324f7c: add rsp, 0x238
140324f83: pop rbx
140324f84: pop rdi
140324f85: pop rsi
140324f86: pop rbp
140324f87: ret 
140324f88: mov ecx, 8
140324f8d: mov edx, 0xe0
140324f92: call 0x1428d9000
140324f97: mov ecx, 8
140324f9c: mov edx, 0x190
140324fa1: call 0x1428d9000
140324fa6: ud2 
140324fa8: nop dword ptr [rax + rax]
140324fb0: mov qword ptr [rsp + 0x10], rdx
140324fb5: push rbp
140324fb6: push rsi
140324fb7: push rdi
140324fb8: push rbx
140324fb9: sub rsp, 0x28
140324fbd: lea rbp, [rdx + 0x80]
140324fc4: lea rcx, [rbp + 0x18]
140324fc8: call 0x1405e6490
140324fcd: nop 
140324fce: add rsp, 0x28
140324fd2: pop rbx
140324fd3: pop rdi
140324fd4: pop rsi
140324fd5: pop rbp
140324fd6: ret 
140324fd7: nop word ptr [rax + rax]
140324fe0: mov qword ptr [rsp + 0x10], rdx
140324fe5: push rbp
140324fe6: push rsi
140324fe7: push rdi
140324fe8: push rbx
140324fe9: sub rsp, 0x28
140324fed: lea rbp, [rdx + 0x80]
140324ff4: mov edx, 0xe0
140324ff9: mov r8d, 8
140324fff: mov rcx, qword ptr [rbp + 0x1a8]
140325006: call 0x140613c20
14032500b: nop 
14032500c: add rsp, 0x28
140325010: pop rbx
140325011: pop rdi
140325012: pop rsi
140325013: pop rbp
140325014: ret 
140325015: nop word ptr cs:[rax + rax]
140325020: mov qword ptr [rsp + 0x10], rdx
140325025: push rbp
140325026: push rsi
140325027: push rdi
140325028: push rbx
140325029: sub rsp, 0x28
14032502d: lea rbp, [rdx + 0x80]
140325034: lea rcx, [rbp - 0x58]
140325038: call 0x140029880
14032503d: nop 
14032503e: add rsp, 0x28
140325042: pop rbx
140325043: pop rdi
140325044: pop rsi
140325045: pop rbp
140325046: ret 
