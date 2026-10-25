1412e4a90: push rbp
1412e4a91: push rsi
1412e4a92: push rdi
1412e4a93: push rbx
1412e4a94: sub rsp, 0xb8
1412e4a9b: lea rbp, [rsp + 0x80]
1412e4aa3: movdqa xmmword ptr [rbp + 0x20], xmm7
1412e4aa8: movdqa xmmword ptr [rbp + 0x10], xmm6
1412e4aad: mov qword ptr [rbp + 8], 0xfffffffffffffffe
1412e4ab5: test r8, r8
1412e4ab8: jne 0x1412e4e54
1412e4abe: mov rdi, rdx
1412e4ac1: mov rsi, rcx
1412e4ac4: movzx eax, byte ptr [rip + 0x235013e]
1412e4acb: mov ecx, 0x2001
1412e4ad0: mov edx, 1
1412e4ad5: call 0x140613c10
1412e4ada: test rax, rax
1412e4add: je 0x1412e4e6c
1412e4ae3: mov rbx, rax
1412e4ae6: mov r8d, 0x2000
1412e4aec: mov rcx, rax
1412e4aef: xor edx, edx
1412e4af1: call 0x142923c20
1412e4af6: mov byte ptr [rbx + 0x2000], 0x20
1412e4afd: movzx eax, byte ptr [rip + 0x2350105]
1412e4b04: mov ecx, 0x1000000
1412e4b09: mov edx, 4
1412e4b0e: call 0x140613c10
1412e4b13: test rax, rax
1412e4b16: mov qword ptr [rbp - 0x20], rbx
1412e4b1a: je 0x1412e4e82
1412e4b20: xor ecx, ecx
1412e4b22: pxor xmm0, xmm0
1412e4b26: nop word ptr cs:[rax + rax]
1412e4b30: movdqu xmmword ptr [rax + rcx + 0x30], xmm0
1412e4b36: movdqu xmmword ptr [rax + rcx + 0x20], xmm0
1412e4b3c: movdqu xmmword ptr [rax + rcx + 0x10], xmm0
1412e4b42: movdqu xmmword ptr [rax + rcx], xmm0
1412e4b47: movdqu xmmword ptr [rax + rcx + 0x40], xmm0
1412e4b4d: movdqu xmmword ptr [rax + rcx + 0x50], xmm0
1412e4b53: movdqu xmmword ptr [rax + rcx + 0x60], xmm0
1412e4b59: movdqu xmmword ptr [rax + rcx + 0x70], xmm0
1412e4b5f: movdqu xmmword ptr [rax + rcx + 0x80], xmm0
1412e4b68: movdqu xmmword ptr [rax + rcx + 0x90], xmm0
1412e4b71: movdqu xmmword ptr [rax + rcx + 0xa0], xmm0
1412e4b7a: movdqu xmmword ptr [rax + rcx + 0xb0], xmm0
1412e4b83: movdqu xmmword ptr [rax + rcx + 0xc0], xmm0
1412e4b8c: movdqu xmmword ptr [rax + rcx + 0xd0], xmm0
1412e4b95: movdqu xmmword ptr [rax + rcx + 0xe0], xmm0
1412e4b9e: movdqu xmmword ptr [rax + rcx + 0xf0], xmm0
1412e4ba7: movdqu xmmword ptr [rax + rcx + 0x130], xmm0
1412e4bb0: movdqu xmmword ptr [rax + rcx + 0x120], xmm0
1412e4bb9: movdqu xmmword ptr [rax + rcx + 0x110], xmm0
1412e4bc2: movdqu xmmword ptr [rax + rcx + 0x100], xmm0
1412e4bcb: movdqu xmmword ptr [rax + rcx + 0x170], xmm0
1412e4bd4: movdqu xmmword ptr [rax + rcx + 0x160], xmm0
1412e4bdd: movdqu xmmword ptr [rax + rcx + 0x150], xmm0
1412e4be6: movdqu xmmword ptr [rax + rcx + 0x140], xmm0
1412e4bef: movdqu xmmword ptr [rax + rcx + 0x180], xmm0
1412e4bf8: movdqu xmmword ptr [rax + rcx + 0x190], xmm0
1412e4c01: movdqu xmmword ptr [rax + rcx + 0x1a0], xmm0
1412e4c0a: movdqu xmmword ptr [rax + rcx + 0x1b0], xmm0
1412e4c13: movdqu xmmword ptr [rax + rcx + 0x1c0], xmm0
1412e4c1c: movdqu xmmword ptr [rax + rcx + 0x1d0], xmm0
1412e4c25: movdqu xmmword ptr [rax + rcx + 0x1e0], xmm0
1412e4c2e: movdqu xmmword ptr [rax + rcx + 0x1f0], xmm0
1412e4c37: movdqu xmmword ptr [rax + rcx + 0x200], xmm0
1412e4c40: movdqu xmmword ptr [rax + rcx + 0x210], xmm0
1412e4c49: movdqu xmmword ptr [rax + rcx + 0x220], xmm0
1412e4c52: movdqu xmmword ptr [rax + rcx + 0x230], xmm0
1412e4c5b: add rcx, 0x240
1412e4c62: cmp rcx, 0xffffc0
1412e4c69: jne 0x1412e4b30
1412e4c6f: pxor xmm0, xmm0
1412e4c73: movdqu xmmword ptr [rax + rcx + 0x30], xmm0
1412e4c79: movdqu xmmword ptr [rax + rcx + 0x20], xmm0
1412e4c7f: movdqu xmmword ptr [rax + rcx + 0x10], xmm0
1412e4c85: mov qword ptr [rbp - 0x18], rax
1412e4c89: movdqu xmmword ptr [rax + rcx], xmm0
1412e4c8e: movzx eax, byte ptr [rip + 0x234ff74]
1412e4c95: mov ecx, 0x8000
1412e4c9a: mov edx, 4
1412e4c9f: call 0x140613c10
1412e4ca4: test rax, rax
1412e4ca7: je 0x1412e4e9e
1412e4cad: movdqa xmm0, xmmword ptr [rip + 0x17efcab]
1412e4cb5: mov ecx, 0xc
1412e4cba: movdqa xmm1, xmmword ptr [rip + 0x19499ce]
1412e4cc2: movdqa xmm2, xmmword ptr [rip + 0x19499d6]
1412e4cca: movdqa xmm3, xmmword ptr [rip + 0x19499de]
1412e4cd2: movdqa xmm4, xmmword ptr [rip + 0x19499e6]
1412e4cda: movdqa xmm5, xmmword ptr [rip + 0x19499ee]
1412e4ce2: nop word ptr cs:[rax + rax]
1412e4cf0: movdqa xmm6, xmm1
1412e4cf4: psubd xmm6, xmm0
1412e4cf8: movdqa xmm7, xmm2
1412e4cfc: psubd xmm7, xmm0
1412e4d00: movdqu xmmword ptr [rax + rcx*4 - 0x30], xmm6
1412e4d06: movdqu xmmword ptr [rax + rcx*4 - 0x20], xmm7
1412e4d0c: movdqa xmm6, xmm3
1412e4d10: psubd xmm6, xmm0
1412e4d14: movdqa xmm7, xmm4
1412e4d18: psubd xmm7, xmm0
1412e4d1c: movdqu xmmword ptr [rax + rcx*4 - 0x10], xmm6
1412e4d22: movdqu xmmword ptr [rax + rcx*4], xmm7
1412e4d27: paddd xmm0, xmm5
1412e4d2b: add rcx, 0x10
1412e4d2f: cmp rcx, 0x200c
1412e4d36: jne 0x1412e4cf0
1412e4d38: mov qword ptr [rbp - 0x10], rax
1412e4d3c: mov byte ptr [rbp + 4], 1
1412e4d40: mov byte ptr [rbp + 3], 1
1412e4d44: lea rcx, [rbp - 0x60]
1412e4d48: mov rdx, rdi
1412e4d4b: xor r8d, r8d
1412e4d4e: call 0x1416191a0
1412e4d53: mov byte ptr [rbp + 2], 1
1412e4d57: mov edx, 0x80
1412e4d5c: xor ecx, ecx
1412e4d5e: call 0x1416969a0
1412e4d63: mov qword ptr [rbp - 0x30], rax
1412e4d67: mov qword ptr [rbp - 8], rdx
1412e4d6b: mov edx, 0x80
1412e4d70: xor ecx, ecx
1412e4d72: call 0x1416969a0
1412e4d77: mov rbx, rax
1412e4d7a: mov rdi, rdx
1412e4d7d: movzx eax, byte ptr [rip + 0x234fe85]
1412e4d84: mov ecx, 0x30
1412e4d89: mov edx, 8
1412e4d8e: call 0x140613c10
1412e4d93: test rax, rax
1412e4d96: je 0x1412e4ebe
1412e4d9c: mov qword ptr [rax], 1
1412e4da3: mov qword ptr [rax + 8], 1
1412e4dab: mov byte ptr [rax + 0x10], 0
1412e4daf: mov qword ptr [rax + 0x18], 0
1412e4db7: mov qword ptr [rax + 0x20], 8
1412e4dbf: mov qword ptr [rax + 0x28], 0
1412e4dc7: mov rcx, qword ptr [rbp - 0x40]
1412e4dcb: mov qword ptr [rsi + 0x88], rcx
1412e4dd2: movups xmm0, xmmword ptr [rbp - 0x60]
1412e4dd6: movups xmm1, xmmword ptr [rbp - 0x50]
1412e4dda: movups xmmword ptr [rsi + 0x78], xmm1
1412e4dde: movups xmmword ptr [rsi + 0x68], xmm0
1412e4de2: mov rcx, qword ptr [rbp - 0x20]
1412e4de6: mov qword ptr [rsi + 0x20], rcx
1412e4dea: mov qword ptr [rsi + 0x28], 0x2001
1412e4df2: mov rcx, qword ptr [rbp - 0x30]
1412e4df6: mov qword ptr [rsi + 0x30], rcx
1412e4dfa: mov rcx, qword ptr [rbp - 8]
1412e4dfe: mov qword ptr [rsi + 0x38], rcx
1412e4e02: mov qword ptr [rsi], 0
1412e4e09: mov qword ptr [rsi + 8], 0x2000
1412e4e11: mov rcx, qword ptr [rbp - 0x10]
1412e4e15: mov qword ptr [rsi + 0x10], rcx
1412e4e19: mov qword ptr [rsi + 0x18], 0x2000
1412e4e21: mov rcx, qword ptr [rbp - 0x18]
1412e4e25: mov qword ptr [rsi + 0x40], rcx
1412e4e29: mov qword ptr [rsi + 0x48], 0x40000
1412e4e31: mov qword ptr [rsi + 0x50], rbx
1412e4e35: mov qword ptr [rsi + 0x58], rdi
1412e4e39: mov qword ptr [rsi + 0x60], rax
1412e4e3d: mov rax, rsi
1412e4e40: movaps xmm6, xmmword ptr [rbp + 0x10]
1412e4e44: movaps xmm7, xmmword ptr [rbp + 0x20]
1412e4e48: add rsp, 0xb8
1412e4e4f: pop rbx
1412e4e50: pop rdi
1412e4e51: pop rsi
1412e4e52: pop rbp
1412e4e53: ret 
1412e4e54: lea rcx, [rip + 0x1949edd]
1412e4e5b: lea r8, [rip + 0x1949ef6]
1412e4e62: mov edx, 0x20
1412e4e67: call 0x1428d9430
1412e4e6c: lea r8, [rip + 0x1949efd]
1412e4e73: mov ecx, 1
1412e4e78: mov edx, 0x2001
1412e4e7d: call 0x1428d8fe3
1412e4e82: mov byte ptr [rbp + 7], 1
1412e4e86: lea r8, [rip + 0x1949efb]
1412e4e8d: mov ecx, 4
1412e4e92: mov edx, 0x1000000
1412e4e97: call 0x1428d8fe3
1412e4e9c: jmp 0x1412e4ed5
1412e4e9e: mov byte ptr [rbp + 6], 1
1412e4ea2: mov byte ptr [rbp + 5], 1
1412e4ea6: lea r8, [rip + 0x1949aeb]
1412e4ead: mov ecx, 4
1412e4eb2: mov edx, 0x8000
1412e4eb7: call 0x1428d8fe3
1412e4ebc: jmp 0x1412e4ed5
1412e4ebe: mov qword ptr [rbp - 0x38], rbx
1412e4ec2: mov qword ptr [rbp - 0x28], rdi
1412e4ec6: mov ecx, 8
1412e4ecb: mov edx, 0x30
1412e4ed0: call 0x1428d9000
1412e4ed5: ud2 
1412e4ed7: nop word ptr [rax + rax]
1412e4ee0: mov qword ptr [rsp + 0x10], rdx
1412e4ee5: push rbp
1412e4ee6: push rsi
1412e4ee7: push rdi
1412e4ee8: push rbx
1412e4ee9: sub rsp, 0x48
1412e4eed: lea rbp, [rdx + 0x80]
1412e4ef4: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e4efa: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e4f00: movzx ebx, byte ptr [rbp + 2]
1412e4f04: mov edx, 0x2001
1412e4f09: mov r8d, 1
1412e4f0f: mov rcx, qword ptr [rbp - 0x20]
1412e4f13: call 0x140613c20
1412e4f18: lea rcx, [rbp - 0x60]
1412e4f1c: call 0x140be24b0
1412e4f21: mov byte ptr [rbp + 4], 0
1412e4f25: mov byte ptr [rbp + 3], bl
1412e4f28: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e4f2e: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e4f34: add rsp, 0x48
1412e4f38: pop rbx
1412e4f39: pop rdi
1412e4f3a: pop rsi
1412e4f3b: pop rbp
1412e4f3c: ret 
1412e4f3d: nop dword ptr [rax]
1412e4f40: mov qword ptr [rsp + 0x10], rdx
1412e4f45: push rbp
1412e4f46: push rsi
1412e4f47: push rdi
1412e4f48: push rbx
1412e4f49: sub rsp, 0x48
1412e4f4d: lea rbp, [rdx + 0x80]
1412e4f54: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e4f5a: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e4f60: mov edx, 0x8000
1412e4f65: mov r8d, 4
1412e4f6b: mov rcx, qword ptr [rbp - 0x10]
1412e4f6f: call 0x140613c20
1412e4f74: mov edx, 0x1000000
1412e4f79: mov r8d, 4
1412e4f7f: mov rcx, qword ptr [rbp - 0x18]
1412e4f83: call 0x140613c20
1412e4f88: cmp qword ptr [rbp - 8], 0
1412e4f8d: je 0x1412e4fa6
1412e4f8f: mov rdx, qword ptr [rbp - 8]
1412e4f93: shl rdx, 3
1412e4f97: mov r8d, 8
1412e4f9d: mov rcx, qword ptr [rbp - 0x30]
1412e4fa1: call 0x140613c20
1412e4fa6: mov byte ptr [rbp + 2], 0
1412e4faa: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e4fb0: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e4fb6: add rsp, 0x48
1412e4fba: pop rbx
1412e4fbb: pop rdi
1412e4fbc: pop rsi
1412e4fbd: pop rbp
1412e4fbe: ret 
1412e4fbf: nop 
1412e4fc0: mov qword ptr [rsp + 0x10], rdx
1412e4fc5: push rbp
1412e4fc6: push rsi
1412e4fc7: push rdi
1412e4fc8: push rbx
1412e4fc9: sub rsp, 0x48
1412e4fcd: lea rbp, [rdx + 0x80]
1412e4fd4: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e4fda: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e4fe0: cmp qword ptr [rbp - 0x28], 0
1412e4fe5: je 0x1412e4ffe
1412e4fe7: mov rdx, qword ptr [rbp - 0x28]
1412e4feb: shl rdx, 3
1412e4fef: mov r8d, 8
1412e4ff5: mov rcx, qword ptr [rbp - 0x38]
1412e4ff9: call 0x140613c20
1412e4ffe: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5004: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e500a: add rsp, 0x48
1412e500e: pop rbx
1412e500f: pop rdi
1412e5010: pop rsi
1412e5011: pop rbp
1412e5012: ret 
1412e5013: nop word ptr cs:[rax + rax]
1412e5020: mov qword ptr [rsp + 0x10], rdx
1412e5025: push rbp
1412e5026: push rsi
1412e5027: push rdi
1412e5028: push rbx
1412e5029: sub rsp, 0x48
1412e502d: lea rbp, [rdx + 0x80]
1412e5034: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e503a: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e5040: movzx ebx, byte ptr [rbp + 4]
1412e5044: cmp byte ptr [rbp + 3], 0
1412e5048: je 0x1412e507d
1412e504a: mov edx, 0x8000
1412e504f: mov r8d, 4
1412e5055: mov rcx, qword ptr [rbp - 0x10]
1412e5059: call 0x140613c20
1412e505e: and bl, 1
1412e5061: mov byte ptr [rbp + 6], bl
1412e5064: mov byte ptr [rbp + 5], 1
1412e5068: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e506e: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5074: add rsp, 0x48
1412e5078: pop rbx
1412e5079: pop rdi
1412e507a: pop rsi
1412e507b: pop rbp
1412e507c: ret 
1412e507d: and bl, 1
1412e5080: mov byte ptr [rbp + 6], bl
1412e5083: mov byte ptr [rbp + 5], 0
1412e5087: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e508d: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5093: add rsp, 0x48
1412e5097: pop rbx
1412e5098: pop rdi
1412e5099: pop rsi
1412e509a: pop rbp
1412e509b: ret 
1412e509c: nop dword ptr [rax]
1412e50a0: mov qword ptr [rsp + 0x10], rdx
1412e50a5: push rbp
1412e50a6: push rsi
1412e50a7: push rdi
1412e50a8: push rbx
1412e50a9: sub rsp, 0x48
1412e50ad: lea rbp, [rdx + 0x80]
1412e50b4: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e50ba: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e50c0: movzx ebx, byte ptr [rbp + 6]
1412e50c4: cmp byte ptr [rbp + 5], 0
1412e50c8: je 0x1412e50f9
1412e50ca: mov edx, 0x1000000
1412e50cf: mov r8d, 4
1412e50d5: mov rcx, qword ptr [rbp - 0x18]
1412e50d9: call 0x140613c20
1412e50de: and bl, 1
1412e50e1: mov byte ptr [rbp + 7], bl
1412e50e4: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e50ea: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e50f0: add rsp, 0x48
1412e50f4: pop rbx
1412e50f5: pop rdi
1412e50f6: pop rsi
1412e50f7: pop rbp
1412e50f8: ret 
1412e50f9: and bl, 1
1412e50fc: mov byte ptr [rbp + 7], bl
1412e50ff: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5105: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e510b: add rsp, 0x48
1412e510f: pop rbx
1412e5110: pop rdi
1412e5111: pop rsi
1412e5112: pop rbp
1412e5113: ret 
1412e5114: nop word ptr cs:[rax + rax]
1412e5120: mov qword ptr [rsp + 0x10], rdx
1412e5125: push rbp
1412e5126: push rsi
1412e5127: push rdi
1412e5128: push rbx
1412e5129: sub rsp, 0x48
1412e512d: lea rbp, [rdx + 0x80]
1412e5134: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e513a: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e5140: cmp byte ptr [rbp + 7], 0
1412e5144: je 0x1412e515a
1412e5146: mov edx, 0x2001
1412e514b: mov r8d, 1
1412e5151: mov rcx, qword ptr [rbp - 0x20]
1412e5155: call 0x140613c20
1412e515a: movaps xmm6, xmmword ptr [rsp + 0x30]
1412e515f: movaps xmm7, xmmword ptr [rsp + 0x20]
1412e5164: add rsp, 0x48
1412e5168: pop rbx
1412e5169: pop rdi
1412e516a: pop rsi
1412e516b: pop rbp
1412e516c: ret 
