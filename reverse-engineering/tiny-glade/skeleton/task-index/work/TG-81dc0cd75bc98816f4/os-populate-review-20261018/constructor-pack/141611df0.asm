141611df0: push rbp
141611df1: push r15
141611df3: push r14
141611df5: push r12
141611df7: push rsi
141611df8: push rdi
141611df9: push rbx
141611dfa: sub rsp, 0x120
141611e01: lea rbp, [rsp + 0x80]
141611e09: mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
141611e14: mov rdi, rdx
141611e17: mov rsi, rcx
141611e1a: test r8, r8
141611e1d: je 0x141611e59
141611e1f: mov rbx, r9
141611e22: mov rax, r9
141611e25: and rax, 3
141611e29: mov qword ptr [rbp - 0x40], rax
141611e2d: jne 0x141611f9c
141611e33: mov r15, r8
141611e36: mov rax, r8
141611e39: and rax, 3
141611e3d: mov qword ptr [rbp - 0x40], rax
141611e41: jne 0x141611fad
141611e47: cmp rbx, 0x40004
141611e4e: jae 0x141612037
141611e54: xor r12d, r12d
141611e57: jmp 0x141611ed3
141611e59: movzx eax, byte ptr [rip + 0x2022da9]
141611e60: mov ecx, 0x40000
141611e65: mov edx, 4
141611e6a: call 0x140613c10
141611e6f: test rax, rax
141611e72: je 0x1416120a1
141611e78: mov r15, rax
141611e7b: mov eax, 0x1c
141611e80: xorps xmm0, xmm0
141611e83: nop word ptr cs:[rax + rax]
141611e90: movups xmmword ptr [r15 + rax*4 - 0x70], xmm0
141611e96: movups xmmword ptr [r15 + rax*4 - 0x60], xmm0
141611e9c: movups xmmword ptr [r15 + rax*4 - 0x50], xmm0
141611ea2: movups xmmword ptr [r15 + rax*4 - 0x40], xmm0
141611ea8: movups xmmword ptr [r15 + rax*4 - 0x30], xmm0
141611eae: movups xmmword ptr [r15 + rax*4 - 0x20], xmm0
141611eb4: movups xmmword ptr [r15 + rax*4 - 0x10], xmm0
141611eba: movups xmmword ptr [r15 + rax*4], xmm0
141611ebf: add rax, 0x20
141611ec3: cmp rax, 0x1001c
141611ec9: jne 0x141611e90
141611ecb: mov r12b, 1
141611ece: mov ebx, 0x40000
141611ed3: mov qword ptr [rbp + 0x88], r15
141611eda: mov byte ptr [rbp + 0x97], r12b
141611ee1: lea rcx, [rbp + 0x10]
141611ee5: mov edx, 0x40000
141611eea: mov r8d, 0x22
141611ef0: mov r9b, 1
141611ef3: call 0x140bd4de0
141611ef8: mov qword ptr [rbp + 0x88], r15
141611eff: mov byte ptr [rbp + 0x97], r12b
141611f06: lea r14, [rbp + 0x68]
141611f0a: lea rdx, [rbp + 0x10]
141611f0e: mov r8d, 4
141611f14: mov rcx, r14
141611f17: call 0x140bd4f20
141611f1c: mov qword ptr [rbp + 0x88], r15
141611f23: mov byte ptr [rbp + 0x97], r12b
141611f2a: mov qword ptr [rsp + 0x30], rbx
141611f2f: mov qword ptr [rsp + 0x28], r15
141611f34: mov qword ptr [rsp + 0x20], r14
141611f39: lea r8, [rip + 0x166db29]
141611f40: lea rcx, [rbp - 0x40]
141611f44: mov r9d, 0x1e
141611f4a: mov rdx, rdi
141611f4d: call 0x1416a2dc0
141611f52: cmp dword ptr [rbp - 0x40], 0xb
141611f56: jne 0x141611fd7
141611f58: mov rax, qword ptr [rbp - 0x18]
141611f5c: mov qword ptr [rsi + 0x20], rax
141611f60: movups xmm0, xmmword ptr [rbp - 0x38]
141611f64: movups xmm1, xmmword ptr [rbp - 0x28]
141611f68: movups xmmword ptr [rsi + 0x10], xmm1
141611f6c: movups xmmword ptr [rsi], xmm0
141611f6f: test r12b, r12b
141611f72: je 0x141611f87
141611f74: mov edx, 0x40000
141611f79: mov r8d, 4
141611f7f: mov rcx, r15
141611f82: call 0x140613c20
141611f87: mov rax, rsi
141611f8a: add rsp, 0x120
141611f91: pop rbx
141611f92: pop rdi
141611f93: pop rsi
141611f94: pop r12
141611f96: pop r14
141611f98: pop r15
141611f9a: pop rbp
141611f9b: ret 
141611f9c: mov qword ptr [rbp + 0x10], 0
141611fa4: lea rax, [rip + 0x16b7f9d]
141611fab: jmp 0x141611fbc
141611fad: mov qword ptr [rbp + 0x10], 0
141611fb5: lea rax, [rip + 0x16b7fa4]
141611fbc: mov qword ptr [rsp + 0x20], rax
141611fc1: lea r8, [rip + 0x16b7f78]
141611fc8: lea rdx, [rbp - 0x40]
141611fcc: lea r9, [rbp + 0x10]
141611fd0: xor ecx, ecx
141611fd2: call 0x1428d9596
141611fd7: mov dword ptr [rbp + 0x84], r12d
141611fde: mov qword ptr [rbp + 0x78], r15
141611fe2: movups xmm0, xmmword ptr [rbp]
141611fe6: movaps xmmword ptr [rbp + 0x50], xmm0
141611fea: movups xmm0, xmmword ptr [rbp - 0x40]
141611fee: movups xmm1, xmmword ptr [rbp - 0x30]
141611ff2: movups xmm2, xmmword ptr [rbp - 0x20]
141611ff6: movups xmm3, xmmword ptr [rbp - 0x10]
141611ffa: movaps xmmword ptr [rbp + 0x40], xmm3
141611ffe: movaps xmmword ptr [rbp + 0x30], xmm2
141612002: movaps xmmword ptr [rbp + 0x20], xmm1
141612006: movaps xmmword ptr [rbp + 0x10], xmm0
14161200a: lea rax, [rip + 0x166e197]
141612011: mov qword ptr [rsp + 0x20], rax
141612016: lea rcx, [rip + 0x166cacb]
14161201d: lea r9, [rip + 0x166caa4]
141612024: lea r8, [rbp + 0x10]
141612028: mov edx, 0x2b
14161202d: call 0x1428d9760
141612032: jmp 0x1416120be
141612037: lea rax, [rip + 0x148c70e]
14161203e: mov qword ptr [rbp - 0x40], rax
141612042: mov qword ptr [rbp - 0x38], 0x46
14161204a: lea rax, [rbp - 0x40]
14161204e: mov qword ptr [rbp + 0x68], rax
141612052: lea rax, [rip - 0x15081c9]
141612059: mov qword ptr [rbp + 0x70], rax
14161205d: lea rax, [rip + 0x166d764]
141612064: mov qword ptr [rbp + 0x10], rax
141612068: mov qword ptr [rbp + 0x18], 1
141612070: mov qword ptr [rbp + 0x30], 0
141612078: lea rax, [rbp + 0x68]
14161207c: mov qword ptr [rbp + 0x20], rax
141612080: mov qword ptr [rbp + 0x28], 1
141612088: mov byte ptr [rbp + 0x97], 0
14161208f: lea rdx, [rip + 0x166d742]
141612096: lea rcx, [rbp + 0x10]
14161209a: call 0x1428d9390
14161209f: jmp 0x1416120be
1416120a1: mov byte ptr [rbp + 0x97], 0
1416120a8: lea r8, [rip + 0x166ca01]
1416120af: mov ecx, 4
1416120b4: mov edx, 0x40000
1416120b9: call 0x1428d8fe3
1416120be: ud2 
1416120c0: mov qword ptr [rsp + 0x10], rdx
1416120c5: push rbp
1416120c6: push r15
1416120c8: push r14
1416120ca: push r12
1416120cc: push rsi
1416120cd: push rdi
1416120ce: push rbx
1416120cf: sub rsp, 0x40
1416120d3: lea rbp, [rdx + 0x80]
1416120da: mov rsi, qword ptr [rbp + 0x78]
1416120de: mov qword ptr [rbp + 0x88], rsi
1416120e5: mov edi, dword ptr [rbp + 0x84]
1416120eb: mov byte ptr [rbp + 0x97], dil
1416120f2: lea rcx, [rbp + 0x10]
1416120f6: call 0x1407cea70
1416120fb: mov qword ptr [rbp + 0x88], rsi
141612102: mov byte ptr [rbp + 0x97], dil
141612109: add rsp, 0x40
14161210d: pop rbx
14161210e: pop rdi
14161210f: pop rsi
141612110: pop r12
141612112: pop r14
141612114: pop r15
141612116: pop rbp
141612117: ret 
141612118: nop dword ptr [rax + rax]
141612120: mov qword ptr [rsp + 0x10], rdx
141612125: push rbp
141612126: push r15
141612128: push r14
14161212a: push r12
14161212c: push rsi
14161212d: push rdi
14161212e: push rbx
14161212f: sub rsp, 0x40
141612133: lea rbp, [rdx + 0x80]
14161213a: test byte ptr [rbp + 0x97], 1
141612141: je 0x14161215a
141612143: mov rcx, qword ptr [rbp + 0x88]
14161214a: mov edx, 0x40000
14161214f: mov r8d, 4
141612155: call 0x140613c20
14161215a: nop 
14161215b: add rsp, 0x40
14161215f: pop rbx
141612160: pop rdi
141612161: pop rsi
141612162: pop r12
141612164: pop r14
141612166: pop r15
141612168: pop rbp
141612169: ret 
