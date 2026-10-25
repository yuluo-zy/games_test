0x140945c60: push rbp
0x140945c61: push rsi
0x140945c62: push rdi
0x140945c63: sub rsp, 0x210
0x140945c6a: lea rbp, [rsp + 0x80]
0x140945c72: mov qword ptr [rbp + 0x188], 0xfffffffffffffffe
0x140945c7d: mov rsi, rcx
0x140945c80: lea rcx, [rbp - 0x60]
0x140945c84: call 0x14175a380
0x140945c89: mov byte ptr [rbp + 0x187], 6
0x140945c90: mov rcx, qword ptr [rip + 0x2ce66d9]
0x140945c97: lea r8, [rip + 0x231a0c2]
0x140945c9e: lea rdx, [rbp + 0x187]
0x140945ca5: call 0x140b218a0
0x140945caa: mov r8, rdx
0x140945cad: lea rcx, [rbp - 0x60]
0x140945cb1: mov rdx, rax
0x140945cb4: call 0x141453000
0x140945cb9: movups xmm0, xmmword ptr [rbp]
0x140945cbd: movaps xmmword ptr [rbp + 0x170], xmm0
0x140945cc4: movups xmm0, xmmword ptr [rbp - 0x10]
0x140945cc8: movaps xmmword ptr [rbp + 0x160], xmm0
0x140945ccf: movups xmm0, xmmword ptr [rbp - 0x20]
0x140945cd3: movaps xmmword ptr [rbp + 0x150], xmm0
0x140945cda: movups xmm0, xmmword ptr [rbp - 0x60]
0x140945cde: movups xmm1, xmmword ptr [rbp - 0x50]
0x140945ce2: movups xmm2, xmmword ptr [rbp - 0x40]
0x140945ce6: movups xmm3, xmmword ptr [rbp - 0x30]
0x140945cea: movaps xmmword ptr [rbp + 0x140], xmm3
0x140945cf1: movaps xmmword ptr [rbp + 0x130], xmm2
0x140945cf8: movaps xmmword ptr [rbp + 0x120], xmm1
0x140945cff: movaps xmmword ptr [rbp + 0x110], xmm0
0x140945d06: lea rdx, [rbp + 0x110]
0x140945d0d: mov rcx, rsi
0x140945d10: call 0x140b67280
0x140945d15: xor ecx, ecx
0x140945d17: call 0x1400d9b20
0x140945d1c: test rax, rax
0x140945d1f: je 0x140945e89
0x140945d25: mov qword ptr [rbp], 0
0x140945d2d: movups xmm0, xmmword ptr [rax]
0x140945d30: inc qword ptr [rax]
0x140945d33: movups xmm1, xmmword ptr [rip + 0x2169d36]
0x140945d3a: movups xmmword ptr [rbp + 0x18], xmm1
0x140945d3e: movups xmm1, xmmword ptr [rip + 0x2169d1b]
0x140945d45: movups xmmword ptr [rbp + 8], xmm1
0x140945d49: movups xmmword ptr [rbp + 0x28], xmm0
0x140945d4d: mov qword ptr [rbp - 0x60], 0
0x140945d55: mov qword ptr [rbp - 0x58], 8
0x140945d5d: mov qword ptr [rbp - 0x50], 0
0x140945d65: mov byte ptr [rbp - 0x48], 0
0x140945d69: lea rdx, [rbp - 0x60]
0x140945d6d: mov rcx, rsi
0x140945d70: call 0x1415398d0
0x140945d75: mov qword ptr [rbp - 0x60], 0
0x140945d7d: mov qword ptr [rbp - 0x50], 0
0x140945d85: mov dword ptr [rbp - 0x38], 0x3b9aca00
0x140945d8c: mov byte ptr [rbp - 0x30], 0
0x140945d90: lea rdx, [rbp - 0x60]
0x140945d94: mov rcx, rax
0x140945d97: call 0x141540380
0x140945d9c: mov rsi, rax
0x140945d9f: mov rdi, rax
0x140945da2: add rdi, 0x38
0x140945da6: movabs rdx, 0x3917598efcc5579e
0x140945db0: movabs r8, 0xaba8f5962f5afb37
0x140945dba: mov rcx, rdi
0x140945dbd: call 0x1400dd6f0
0x140945dc2: test rax, rax
0x140945dc5: je 0x140945dfa
0x140945dc7: cmp qword ptr [rsi + 0x218], rdx
0x140945dce: jbe 0x140945dfa
0x140945dd0: mov rax, qword ptr [rsi + 0x210]
0x140945dd7: mov rax, qword ptr [rax + rdx*8]
0x140945ddb: test rax, rax
0x140945dde: je 0x140945dfa
0x140945de0: mov rcx, qword ptr [rsi + 0x1e0]
0x140945de7: not rax
0x140945dea: lea rax, [rax + rax*2]
0x140945dee: shl rax, 5
0x140945df2: cmp qword ptr [rcx + rax + 0x20], 0
0x140945df8: jne 0x140945e02
0x140945dfa: mov rcx, rsi
0x140945dfd: call 0x14150fbd0
0x140945e02: movabs rdx, 0x2a88611d41c83a99
0x140945e0c: movabs r8, 0x3f8125525bf8d24e
0x140945e16: mov rcx, rdi
0x140945e19: call 0x1400dd6f0
0x140945e1e: test rax, rax
0x140945e21: je 0x140945e56
0x140945e23: cmp qword ptr [rsi + 0x218], rdx
0x140945e2a: jbe 0x140945e56
0x140945e2c: mov rax, qword ptr [rsi + 0x210]
0x140945e33: mov rax, qword ptr [rax + rdx*8]
0x140945e37: test rax, rax
0x140945e3a: je 0x140945e56
0x140945e3c: mov rcx, qword ptr [rsi + 0x1e0]
0x140945e43: not rax
0x140945e46: lea rax, [rax + rax*2]
0x140945e4a: shl rax, 5
0x140945e4e: cmp qword ptr [rcx + rax + 0x20], 0
0x140945e54: jne 0x140945e5e
0x140945e56: mov rcx, rsi
0x140945e59: call 0x14150ca50
0x140945e5e: lea rdi, [rbp - 0x60]
0x140945e62: mov r8d, 0x170
0x140945e68: mov rcx, rdi
0x140945e6b: xor edx, edx
0x140945e6d: call 0x142923c20
0x140945e72: mov rcx, rsi
0x140945e75: mov rdx, rdi
0x140945e78: call 0x1415468b0
0x140945e7d: nop 
0x140945e7e: add rsp, 0x210
0x140945e85: pop rdi
0x140945e86: pop rsi
0x140945e87: pop rbp
0x140945e88: ret 
0x140945e89: lea rcx, [rip + 0x22f4c10]
0x140945e90: call 0x1428d80d0
0x140945e95: int3 
0x140945e96: nop word ptr cs:[rax + rax]
0x140945ea0: mov qword ptr [rsp + 0x10], rdx
0x140945ea5: push rbp
0x140945ea6: push rsi
0x140945ea7: push rdi
0x140945ea8: sub rsp, 0x20
0x140945eac: lea rbp, [rdx + 0x80]
0x140945eb3: lea rcx, [rbp - 0x60]
0x140945eb7: call 0x140029880
0x140945ebc: nop 
0x140945ebd: add rsp, 0x20
0x140945ec1: pop rdi
0x140945ec2: pop rsi
0x140945ec3: pop rbp
0x140945ec4: ret 
