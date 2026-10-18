1416148e0: push rbp
1416148e1: push r15
1416148e3: push r14
1416148e5: push r12
1416148e7: push rsi
1416148e8: push rdi
1416148e9: push rbx
1416148ea: sub rsp, 0x100
1416148f1: lea rbp, [rsp + 0x80]
1416148f9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141614901: mov rbx, r8
141614904: mov rdi, rdx
141614907: mov rsi, rcx
14161490a: test r8, r8
14161490d: je 0x14161499f
141614913: mov r14, r9
141614916: mov rax, r9
141614919: and rax, 3
14161491d: mov qword ptr [rbp - 0x40], rax
141614921: jne 0x141614a2b
141614927: mov rax, rbx
14161492a: and rax, 3
14161492e: mov qword ptr [rbp - 0x40], rax
141614932: jne 0x141614a3c
141614938: cmp r14, 0xc
14161493c: jb 0x1416149a2
14161493e: lea rax, [rip + 0x148a417]
141614945: mov qword ptr [rbp - 0x40], rax
141614949: mov qword ptr [rbp - 0x38], 0x48
141614951: lea rax, [rbp - 0x40]
141614955: mov qword ptr [rbp + 0x68], rax
141614959: lea rax, [rip - 0x150aad0]
141614960: mov qword ptr [rbp + 0x70], rax
141614964: lea rax, [rip + 0x166ae5d]
14161496b: mov qword ptr [rbp + 0x10], rax
14161496f: mov qword ptr [rbp + 0x18], 1
141614977: mov qword ptr [rbp + 0x30], 0
14161497f: lea rax, [rbp + 0x68]
141614983: mov qword ptr [rbp + 0x20], rax
141614987: mov qword ptr [rbp + 0x28], 1
14161498f: lea rdx, [rip + 0x166ae42]
141614996: lea rcx, [rbp + 0x10]
14161499a: call 0x1428d9390
14161499f: xor r14d, r14d
1416149a2: lea r15, [rbp + 0x10]
1416149a6: mov edx, 8
1416149ab: mov rcx, r15
1416149ae: mov r8d, 0x122
1416149b4: mov r9b, 1
1416149b7: call 0x140bd4de0
1416149bc: lea r12, [rbp + 0x68]
1416149c0: mov r8d, 4
1416149c6: mov rcx, r12
1416149c9: mov rdx, r15
1416149cc: call 0x140bd4f20
1416149d1: mov qword ptr [rsp + 0x30], r14
1416149d6: mov qword ptr [rsp + 0x28], rbx
1416149db: mov qword ptr [rsp + 0x20], r12
1416149e0: lea r8, [rip + 0x166b5fd]
1416149e7: lea rcx, [rbp - 0x40]
1416149eb: mov r9d, 0x13
1416149f1: mov rdx, rdi
1416149f4: call 0x1416a2dc0
1416149f9: cmp dword ptr [rbp - 0x40], 0xb
1416149fd: jne 0x141614a66
1416149ff: mov rax, qword ptr [rbp - 0x18]
141614a03: mov qword ptr [rsi + 0x20], rax
141614a07: movups xmm0, xmmword ptr [rbp - 0x38]
141614a0b: movups xmm1, xmmword ptr [rbp - 0x28]
141614a0f: movups xmmword ptr [rsi + 0x10], xmm1
141614a13: movups xmmword ptr [rsi], xmm0
141614a16: mov rax, rsi
141614a19: add rsp, 0x100
141614a20: pop rbx
141614a21: pop rdi
141614a22: pop rsi
141614a23: pop r12
141614a25: pop r14
141614a27: pop r15
141614a29: pop rbp
141614a2a: ret 
141614a2b: mov qword ptr [rbp + 0x10], 0
141614a33: lea rax, [rip + 0x16b550e]
141614a3a: jmp 0x141614a4b
141614a3c: mov qword ptr [rbp + 0x10], 0
141614a44: lea rax, [rip + 0x16b5515]
141614a4b: mov qword ptr [rsp + 0x20], rax
141614a50: lea r8, [rip + 0x16b54e9]
141614a57: lea rdx, [rbp - 0x40]
141614a5b: lea r9, [rbp + 0x10]
141614a5f: xor ecx, ecx
141614a61: call 0x1428d9596
141614a66: movups xmm0, xmmword ptr [rbp]
141614a6a: movaps xmmword ptr [rbp + 0x50], xmm0
141614a6e: movups xmm0, xmmword ptr [rbp - 0x40]
141614a72: movups xmm1, xmmword ptr [rbp - 0x30]
141614a76: movups xmm2, xmmword ptr [rbp - 0x20]
141614a7a: movups xmm3, xmmword ptr [rbp - 0x10]
141614a7e: movaps xmmword ptr [rbp + 0x40], xmm3
141614a82: movaps xmmword ptr [rbp + 0x30], xmm2
141614a86: movaps xmmword ptr [rbp + 0x20], xmm1
141614a8a: movaps xmmword ptr [rbp + 0x10], xmm0
141614a8e: lea rax, [rip + 0x166b713]
141614a95: mov qword ptr [rsp + 0x20], rax
141614a9a: lea rcx, [rip + 0x166a047]
141614aa1: lea r9, [rip + 0x166a020]
141614aa8: lea r8, [rbp + 0x10]
141614aac: mov edx, 0x2b
141614ab1: call 0x1428d9760
141614ab6: ud2 
141614ab8: nop dword ptr [rax + rax]
141614ac0: mov qword ptr [rsp + 0x10], rdx
141614ac5: push rbp
141614ac6: push r15
141614ac8: push r14
141614aca: push r12
141614acc: push rsi
141614acd: push rdi
141614ace: push rbx
141614acf: sub rsp, 0x40
141614ad3: lea rbp, [rdx + 0x80]
141614ada: lea rcx, [rbp + 0x10]
141614ade: call 0x1407cea70
141614ae3: nop 
141614ae4: add rsp, 0x40
141614ae8: pop rbx
141614ae9: pop rdi
141614aea: pop rsi
141614aeb: pop r12
141614aed: pop r14
141614aef: pop r15
141614af1: pop rbp
141614af2: ret 
