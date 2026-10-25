0x1408c9a30: push rbp
0x1408c9a31: push r15
0x1408c9a33: push r14
0x1408c9a35: push r13
0x1408c9a37: push r12
0x1408c9a39: push rsi
0x1408c9a3a: push rdi
0x1408c9a3b: push rbx
0x1408c9a3c: sub rsp, 0x948
0x1408c9a43: lea rbp, [rsp + 0x80]
0x1408c9a4b: mov qword ptr [rbp + 0x8c0], 0xfffffffffffffffe
0x1408c9a56: mov rbx, r8
0x1408c9a59: mov rsi, rdx
0x1408c9a5c: mov rdi, rcx
0x1408c9a5f: lea r14, [rdx + 0x38]
0x1408c9a63: movabs rdx, 0xd38dae617fb79c7a
0x1408c9a6d: movabs r8, 0x22aadf5c5dad1eae
0x1408c9a77: mov rcx, r14
0x1408c9a7a: call 0x1400dd6f0
0x1408c9a7f: test rax, rax
0x1408c9a82: je 0x1408c9fd4
0x1408c9a88: cmp qword ptr [rsi + 0x218], rdx
0x1408c9a8f: jbe 0x1408c9fd4
0x1408c9a95: mov rax, qword ptr [rsi + 0x210]
0x1408c9a9c: mov rcx, qword ptr [rax + rdx*8]
0x1408c9aa0: test rcx, rcx
0x1408c9aa3: je 0x1408c9fd4
0x1408c9aa9: mov rax, qword ptr [rsi + 0x1e0]
0x1408c9ab0: not rcx
0x1408c9ab3: lea rcx, [rcx + rcx*2]
0x1408c9ab7: shl rcx, 5
0x1408c9abb: cmp qword ptr [rax + rcx + 0x20], 0
0x1408c9ac1: je 0x1408c9fd4
0x1408c9ac7: add rax, rcx
0x1408c9aca: mov rax, qword ptr [rax + 0x10]
0x1408c9ace: mov rcx, qword ptr [rax]
0x1408c9ad1: cmp rcx, 2
0x1408c9ad5: je 0x1408c9f8b
0x1408c9adb: cmp ecx, 0xa
0x1408c9ade: jne 0x1408c9aea
0x1408c9ae0: cmp byte ptr [rax + 8], 1
0x1408c9ae4: jne 0x1408c9f8b
0x1408c9aea: lea r8, [rcx - 2]
0x1408c9aee: cmp r8, 9
0x1408c9af2: mov edx, 4
0x1408c9af7: cmovb rdx, r8
0x1408c9afb: mov r8d, 0xaf
0x1408c9b01: bt r8, rdx
0x1408c9b05: jae 0x1408c9f7f
0x1408c9b0b: cmp ecx, 7
0x1408c9b0e: jne 0x1408c9b1a
0x1408c9b10: cmp byte ptr [rax + 8], 1
0x1408c9b14: je 0x1408c9f8b
0x1408c9b1a: mov qword ptr [rbp + 0x8a8], rdi
0x1408c9b21: movabs rdx, 0xfb32e4dcc8287bca
0x1408c9b2b: movabs r8, 0x80383a8198597dc2
0x1408c9b35: mov rcx, r14
0x1408c9b38: call 0x1400dd6f0
0x1408c9b3d: test rax, rax
0x1408c9b40: je 0x1408c9fe0
0x1408c9b46: cmp qword ptr [rsi + 0x218], rdx
0x1408c9b4d: jbe 0x1408c9fe0
0x1408c9b53: mov rax, qword ptr [rsi + 0x210]
0x1408c9b5a: mov rax, qword ptr [rax + rdx*8]
0x1408c9b5e: test rax, rax
0x1408c9b61: je 0x1408c9fe0
0x1408c9b67: mov r13, qword ptr [rsi + 0x1e0]
0x1408c9b6e: not rax
0x1408c9b71: lea rax, [rax + rax*2]
0x1408c9b75: shl rax, 5
0x1408c9b79: cmp qword ptr [r13 + rax + 0x20], 0
0x1408c9b7f: je 0x1408c9fe0
0x1408c9b85: add r13, rax
0x1408c9b88: mov r12, qword ptr [r13 + 0x10]
0x1408c9b8c: mov eax, dword ptr [rsi + 0x5b0]
0x1408c9b92: mov dword ptr [rbp + 0x8b0], eax
0x1408c9b98: mov rdi, qword ptr [r12]
0x1408c9b9c: lea r15, [rdi + 0x10]
0x1408c9ba0: mov cl, 1
0x1408c9ba2: xor eax, eax
0x1408c9ba4: lock cmpxchg byte ptr [rdi + 0x10], cl
0x1408c9ba9: jne 0x1408c9fab
0x1408c9baf: mov rcx, r15
0x1408c9bb2: call 0x140c81c70
0x1408c9bb7: mov rax, qword ptr [rdi + 0x48]
0x1408c9bbb: mov qword ptr [rbp], rax
0x1408c9bbf: movups xmm0, xmmword ptr [rdi + 0x18]
0x1408c9bc3: movups xmm1, xmmword ptr [rdi + 0x28]
0x1408c9bc7: movups xmm2, xmmword ptr [rdi + 0x38]
0x1408c9bcb: movaps xmmword ptr [rbp - 0x10], xmm2
0x1408c9bcf: movaps xmmword ptr [rbp - 0x20], xmm1
0x1408c9bd3: movaps xmmword ptr [rbp - 0x30], xmm0
0x1408c9bd7: mov byte ptr [rdi + 0x4e], 7
0x1408c9bdb: mov byte ptr [rbp + 0x8be], 1
0x1408c9be2: mov rcx, r15
0x1408c9be5: call 0x140c81e10
0x1408c9bea: xor ecx, ecx
0x1408c9bec: mov al, 1
0x1408c9bee: lock cmpxchg byte ptr [r15], cl
0x1408c9bf3: jne 0x1408c9fbe
0x1408c9bf9: cmp byte ptr [rbp + 6], 7
0x1408c9bfd: je 0x1408c9c97
0x1408c9c03: mov rax, qword ptr [rbp]
0x1408c9c07: mov qword ptr [rbp + 0x3f0], rax
0x1408c9c0e: movaps xmm0, xmmword ptr [rbp - 0x30]
0x1408c9c12: movaps xmm1, xmmword ptr [rbp - 0x20]
0x1408c9c16: movaps xmm2, xmmword ptr [rbp - 0x10]
0x1408c9c1a: movaps xmmword ptr [rbp + 0x3e0], xmm2
0x1408c9c21: movaps xmmword ptr [rbp + 0x3d0], xmm1
0x1408c9c28: movaps xmmword ptr [rbp + 0x3c0], xmm0
0x1408c9c2f: mov eax, dword ptr [rbp + 0x8b0]
0x1408c9c35: mov dword ptr [r13 + 0x5c], eax
0x1408c9c39: lea rax, [r12 + 8]
0x1408c9c3e: mov qword ptr [rbp + 0x8b0], rax
0x1408c9c45: cmp byte ptr [r12 + 0x3e], 7
0x1408c9c4b: je 0x1408c9c65
0x1408c9c4d: mov rcx, qword ptr [rbp + 0x8b0]
0x1408c9c54: call 0x14003bb80
0x1408c9c59: mov rcx, qword ptr [rbp + 0x8b0]
0x1408c9c60: call 0x140be24b0
0x1408c9c65: mov rax, qword ptr [rbp + 0x3f0]
0x1408c9c6c: mov rcx, qword ptr [rbp + 0x8b0]
0x1408c9c73: mov qword ptr [rcx + 0x30], rax
0x1408c9c77: movaps xmm0, xmmword ptr [rbp + 0x3c0]
0x1408c9c7e: movaps xmm1, xmmword ptr [rbp + 0x3d0]
0x1408c9c85: movaps xmm2, xmmword ptr [rbp + 0x3e0]
0x1408c9c8c: movups xmmword ptr [rcx + 0x20], xmm2
0x1408c9c90: movups xmmword ptr [rcx + 0x10], xmm1
0x1408c9c94: movups xmmword ptr [rcx], xmm0
0x1408c9c97: cmp byte ptr [r12 + 0x3e], 7
0x1408c9c9d: mov rdi, qword ptr [rbp + 0x8a8]
0x1408c9ca4: je 0x1408c9f8b
0x1408c9caa: movabs rdx, 0x8597ee0727b461b3
0x1408c9cb4: movabs r8, 0x8b297ec5ae15f15c
0x1408c9cbe: mov rcx, r14
0x1408c9cc1: call 0x1400dd6f0
0x1408c9cc6: test rax, rax
0x1408c9cc9: je 0x1408c9fec
0x1408c9ccf: cmp qword ptr [rsi + 0x218], rdx
0x1408c9cd6: jbe 0x1408c9fec
0x1408c9cdc: mov rax, qword ptr [rsi + 0x210]
0x1408c9ce3: mov rcx, qword ptr [rax + rdx*8]
0x1408c9ce7: test rcx, rcx
0x1408c9cea: je 0x1408c9fec
0x1408c9cf0: mov rax, qword ptr [rsi + 0x1e0]
0x1408c9cf7: not rcx
0x1408c9cfa: lea rcx, [rcx + rcx*2]
0x1408c9cfe: shl rcx, 5
0x1408c9d02: cmp qword ptr [rax + rcx + 0x20], 0
0x1408c9d08: je 0x1408c9fec
0x1408c9d0e: add rax, rcx
0x1408c9d11: mov rcx, qword ptr [rax + 0x10]
0x1408c9d15: lea rdx, [rip + 0x21d94c4]
0x1408c9d1c: mov r8d, 0x18
0x1408c9d22: call 0x140937aa0
0x1408c9d27: test al, 1
0x1408c9d29: je 0x1408c9ff8
0x1408c9d2f: mov r14, rdx
0x1408c9d32: lea rcx, [rbx + 4]
0x1408c9d36: call 0x140bd5790
0x1408c9d3b: mov dword ptr [rbp + 0x3c0], 2
0x1408c9d45: mov qword ptr [rbp + 0x3c8], rbx
0x1408c9d4c: mov dword ptr [rbp + 0x7a8], 2
0x1408c9d56: mov qword ptr [rsp + 0x40], rdi
0x1408c9d5b: mov qword ptr [rsp + 0x38], rsi
0x1408c9d60: mov qword ptr [rsp + 0x30], rax
0x1408c9d65: lea rax, [rbp + 0x7a8]
0x1408c9d6c: mov qword ptr [rsp + 0x28], rax
0x1408c9d71: mov qword ptr [rsp + 0x20], 1
0x1408c9d7a: lea rdx, [rip + 0x21d948f]
0x1408c9d81: lea rcx, [rbp - 0x30]
0x1408c9d85: lea r9, [rbp + 0x3c0]
0x1408c9d8c: mov r8d, 7
0x1408c9d92: call 0x140867970
0x1408c9d97: mov byte ptr [rbp + 0x8bf], 1
0x1408c9d9e: call 0x140be6750
0x1408c9da3: mov esi, eax
0x1408c9da5: mov byte ptr [rbp + 0x8bf], 1
0x1408c9dac: xor ecx, ecx
0x1408c9dae: call 0x1400d9b20
0x1408c9db3: test rax, rax
0x1408c9db6: je 0x1408ca004
0x1408c9dbc: mov dword ptr [rbp + 0x468], esi
0x1408c9dc2: mov word ptr [rbp + 0x470], 0
0x1408c9dcb: mov dword ptr [rbp + 0x46c], 0x3f800000
0x1408c9dd5: mov qword ptr [rbp + 0x3c0], 0
0x1408c9de0: mov qword ptr [rbp + 0x460], r14
0x1408c9de7: mov qword ptr [rbp + 0x3d0], 0
0x1408c9df2: mov qword ptr [rbp + 0x3d8], 4
0x1408c9dfd: mov qword ptr [rbp + 0x3e0], 0
0x1408c9e08: movups xmm0, xmmword ptr [rax]
0x1408c9e0b: inc qword ptr [rax]
0x1408c9e0e: movups xmm1, xmmword ptr [rip + 0x21cc91b]
0x1408c9e15: movups xmmword ptr [rbp + 0x440], xmm1
0x1408c9e1c: movups xmm1, xmmword ptr [rip + 0x21cc8fd]
0x1408c9e23: movups xmmword ptr [rbp + 0x430], xmm1
0x1408c9e2a: movups xmmword ptr [rbp + 0x450], xmm0
0x1408c9e31: movabs rax, 0x100000003
0x1408c9e3b: mov qword ptr [rbp + 0x400], rax
0x1408c9e42: mov qword ptr [rbp + 0x3e8], 0
0x1408c9e4d: mov qword ptr [rbp + 0x3f0], 1
0x1408c9e58: mov qword ptr [rbp + 0x3f8], 0
0x1408c9e63: mov byte ptr [rbp + 0x8bf], 1
0x1408c9e6a: lea rcx, [rbp + 0x7a8]
0x1408c9e71: lea rsi, [rbp + 0x3c0]
0x1408c9e78: lea r8, [rbp - 0x30]
0x1408c9e7c: mov rdx, rsi
0x1408c9e7f: xor r9d, r9d
0x1408c9e82: call 0x140905be0
0x1408c9e87: add r12, 8
0x1408c9e8b: mov qword ptr [rbp + 0x8a0], r12
0x1408c9e92: mov dword ptr [rbp + 0x3c0], 2
0x1408c9e9c: mov qword ptr [rsp + 0x20], rsi
0x1408c9ea1: lea rcx, [rbp + 0x8a0]
0x1408c9ea8: lea rdx, [rbp + 0x7a8]
0x1408c9eaf: movabs r8, 0x11ba5697cc40b3dd
0x1408c9eb9: mov r9b, 0x1d
0x1408c9ebc: call 0x1408fd6c0
0x1408c9ec1: mov rsi, qword ptr [rbp + 0x7a8]
0x1408c9ec8: lea rdx, [rbp + 0x7b0]
0x1408c9ecf: lea rcx, [rbp + 0x3c8]
0x1408c9ed6: mov r8d, 0xa0
0x1408c9edc: call 0x142923c00
0x1408c9ee1: movzx eax, byte ptr [rbp + 0x850]
0x1408c9ee8: movups xmm0, xmmword ptr [rbp + 0x851]
0x1408c9eef: movups xmmword ptr [rbp + 0x469], xmm0
0x1408c9ef6: movups xmm0, xmmword ptr [rbp + 0x861]
0x1408c9efd: movups xmmword ptr [rbp + 0x479], xmm0
0x1408c9f04: movups xmm0, xmmword ptr [rbp + 0x871]
0x1408c9f0b: movups xmmword ptr [rbp + 0x489], xmm0
0x1408c9f12: movups xmm0, xmmword ptr [rbp + 0x881]
0x1408c9f19: movups xmmword ptr [rbp + 0x499], xmm0
0x1408c9f20: movups xmm0, xmmword ptr [rbp + 0x890]
0x1408c9f27: movups xmmword ptr [rbp + 0x4a8], xmm0
0x1408c9f2e: and al, 0x7f
0x1408c9f30: mov qword ptr [rbp + 0x3c0], rsi
0x1408c9f37: mov byte ptr [rbp + 0x468], al
0x1408c9f3d: mov byte ptr [rbp + 0x8bf], 1
0x1408c9f44: lea rcx, [rbp + 0x3c0]
0x1408c9f4b: mov edx, 3
0x1408c9f50: call 0x140906a90
0x1408c9f55: lea rsi, [rbp + 0x3c0]
0x1408c9f5c: lea rdx, [rbp - 0x30]
0x1408c9f60: mov r8d, 0x3e8
0x1408c9f66: mov rcx, rsi
0x1408c9f69: call 0x142923c00
0x1408c9f6e: mov byte ptr [rbp + 0x8bf], 0
0x1408c9f75: mov rcx, rsi
0x1408c9f78: call 0x140868370
0x1408c9f7d: jmp 0x1408c9f8b
0x1408c9f7f: mov r8d, 0x50
0x1408c9f85: bt r8, rdx
0x1408c9f89: jae 0x1408c9f9f
0x1408c9f8b: add rsp, 0x948
0x1408c9f92: pop rbx
0x1408c9f93: pop rdi
0x1408c9f94: pop rsi
0x1408c9f95: pop r12
0x1408c9f97: pop r13
0x1408c9f99: pop r14
0x1408c9f9b: pop r15
0x1408c9f9d: pop rbp
0x1408c9f9e: ret 
0x1408c9f9f: cmp byte ptr [rax + 8], 0
0x1408c9fa3: je 0x1408c9b0b
0x1408c9fa9: jmp 0x1408c9f8b
0x1408c9fab: mov rcx, r15
0x1408c9fae: mov r8d, 0x3b9aca00
0x1408c9fb4: call 0x1428cd210
0x1408c9fb9: jmp 0x1408c9baf
0x1408c9fbe: mov byte ptr [rbp + 0x8be], 1
0x1408c9fc5: mov rcx, r15
0x1408c9fc8: xor edx, edx
0x1408c9fca: call 0x1428cdb90
0x1408c9fcf: jmp 0x1408c9bf9
0x1408c9fd4: lea rcx, [rip + 0x21d91bd]
0x1408c9fdb: call 0x1428d9310
0x1408c9fe0: lea rcx, [rip + 0x21d91c9]
0x1408c9fe7: call 0x1428d9310
0x1408c9fec: lea rcx, [rip + 0x21d91d5]
0x1408c9ff3: call 0x1428d9310
0x1408c9ff8: lea rcx, [rip + 0x21d91f9]
0x1408c9fff: call 0x1428d9310
0x1408ca004: mov byte ptr [rbp + 0x8bf], 1
0x1408ca00b: lea rcx, [rip + 0x2370a8e]
0x1408ca012: call 0x1428d80d0
0x1408ca017: ud2 
0x1408ca019: nop dword ptr [rax]
0x1408ca020: mov qword ptr [rsp + 0x10], rdx
0x1408ca025: push rbp
0x1408ca026: push r15
0x1408ca028: push r14
0x1408ca02a: push r13
0x1408ca02c: push r12
0x1408ca02e: push rsi
0x1408ca02f: push rdi
0x1408ca030: push rbx
0x1408ca031: sub rsp, 0x48
0x1408ca035: lea rbp, [rdx + 0x80]
0x1408ca03c: cmp byte ptr [rbp + 0x8bf], 0
0x1408ca043: jne 0x1408ca056
0x1408ca045: add rsp, 0x48
0x1408ca049: pop rbx
0x1408ca04a: pop rdi
0x1408ca04b: pop rsi
0x1408ca04c: pop r12
0x1408ca04e: pop r13
0x1408ca050: pop r14
0x1408ca052: pop r15
0x1408ca054: pop rbp
0x1408ca055: ret 
0x1408ca056: lea rcx, [rbp - 0x30]
0x1408ca05a: call 0x1408923c0
0x1408ca05f: jmp 0x1408ca045
0x1408ca061: nop word ptr cs:[rax + rax]
0x1408ca070: mov qword ptr [rsp + 0x10], rdx
0x1408ca075: push rbp
0x1408ca076: push r15
0x1408ca078: push r14
0x1408ca07a: push r13
0x1408ca07c: push r12
0x1408ca07e: push rsi
0x1408ca07f: push rdi
0x1408ca080: push rbx
0x1408ca081: sub rsp, 0x48
0x1408ca085: lea rbp, [rdx + 0x80]
0x1408ca08c: cmp byte ptr [rbp + 6], 7
0x1408ca090: je 0x1408ca09b
0x1408ca092: test byte ptr [rbp + 0x8be], 1
0x1408ca099: jne 0x1408ca0ac
0x1408ca09b: add rsp, 0x48
0x1408ca09f: pop rbx
0x1408ca0a0: pop rdi
0x1408ca0a1: pop rsi
0x1408ca0a2: pop r12
0x1408ca0a4: pop r13
0x1408ca0a6: pop r14
0x1408ca0a8: pop r15
0x1408ca0aa: pop rbp
0x1408ca0ab: ret 
0x1408ca0ac: lea rcx, [rbp - 0x30]
0x1408ca0b0: call 0x1405ee5e0
0x1408ca0b5: jmp 0x1408ca09b
0x1408ca0b7: nop word ptr [rax + rax]
0x1408ca0c0: mov qword ptr [rsp + 0x10], rdx
0x1408ca0c5: push rbp
0x1408ca0c6: push r15
0x1408ca0c8: push r14
0x1408ca0ca: push r13
0x1408ca0cc: push r12
0x1408ca0ce: push rsi
0x1408ca0cf: push rdi
0x1408ca0d0: push rbx
0x1408ca0d1: sub rsp, 0x48
0x1408ca0d5: lea rbp, [rdx + 0x80]
0x1408ca0dc: mov rcx, qword ptr [rbp + 0x8b0]
0x1408ca0e3: call 0x140be24b0
0x1408ca0e8: nop 
0x1408ca0e9: add rsp, 0x48
0x1408ca0ed: pop rbx
0x1408ca0ee: pop rdi
0x1408ca0ef: pop rsi
0x1408ca0f0: pop r12
0x1408ca0f2: pop r13
0x1408ca0f4: pop r14
0x1408ca0f6: pop r15
0x1408ca0f8: pop rbp
0x1408ca0f9: ret 
0x1408ca0fa: nop word ptr [rax + rax]
0x1408ca100: mov qword ptr [rsp + 0x10], rdx
0x1408ca105: push rbp
0x1408ca106: push r15
0x1408ca108: push r14
0x1408ca10a: push r13
0x1408ca10c: push r12
0x1408ca10e: push rsi
0x1408ca10f: push rdi
0x1408ca110: push rbx
0x1408ca111: sub rsp, 0x48
0x1408ca115: lea rbp, [rdx + 0x80]
0x1408ca11c: mov rax, qword ptr [rbp + 0x3f0]
0x1408ca123: mov rcx, qword ptr [rbp + 0x8b0]
0x1408ca12a: mov qword ptr [rcx + 0x30], rax
0x1408ca12e: movaps xmm0, xmmword ptr [rbp + 0x3c0]
0x1408ca135: movaps xmm1, xmmword ptr [rbp + 0x3d0]
0x1408ca13c: movaps xmm2, xmmword ptr [rbp + 0x3e0]
0x1408ca143: movups xmmword ptr [rcx + 0x20], xmm2
0x1408ca147: movups xmmword ptr [rcx + 0x10], xmm1
0x1408ca14b: movups xmmword ptr [rcx], xmm0
0x1408ca14e: mov byte ptr [rbp + 0x8be], 0
0x1408ca155: add rsp, 0x48
0x1408ca159: pop rbx
0x1408ca15a: pop rdi
0x1408ca15b: pop rsi
0x1408ca15c: pop r12
0x1408ca15e: pop r13
0x1408ca160: pop r14
0x1408ca162: pop r15
0x1408ca164: pop rbp
0x1408ca165: ret 
0x1408ca166: nop word ptr cs:[rax + rax]
0x1408ca170: mov qword ptr [rsp + 0x10], rdx
0x1408ca175: push rbp
0x1408ca176: push r15
0x1408ca178: push r14
0x1408ca17a: push r13
0x1408ca17c: push r12
0x1408ca17e: push rsi
0x1408ca17f: push rdi
0x1408ca180: push rbx
0x1408ca181: sub rsp, 0x48
0x1408ca185: lea rbp, [rdx + 0x80]
0x1408ca18c: mov byte ptr [rbp + 0x8bf], 1
0x1408ca193: lea rcx, [rbp + 0x7a8]
0x1408ca19a: call 0x1408935c0
0x1408ca19f: mov byte ptr [rbp + 0x8bf], 1
0x1408ca1a6: add rsp, 0x48
0x1408ca1aa: pop rbx
0x1408ca1ab: pop rdi
0x1408ca1ac: pop rsi
0x1408ca1ad: pop r12
0x1408ca1af: pop r13
0x1408ca1b1: pop r14
0x1408ca1b3: pop r15
0x1408ca1b5: pop rbp
0x1408ca1b6: ret 
