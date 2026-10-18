141615f50: push rbp
141615f51: push r15
141615f53: push r14
141615f55: push r12
141615f57: push rsi
141615f58: push rdi
141615f59: push rbx
141615f5a: sub rsp, 0x100
141615f61: lea rbp, [rsp + 0x80]
141615f69: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141615f71: mov rsi, rcx
141615f74: test r8, r8
141615f77: sete al
141615f7a: cmp r9, 0x3e9
141615f81: setb cl
141615f84: or cl, al
141615f86: je 0x141616027
141615f8c: mov r14, r9
141615f8f: mov rbx, r8
141615f92: mov rdi, rdx
141615f95: lea r15, [rbp + 0x10]
141615f99: mov edx, 0xbb80
141615f9e: mov rcx, r15
141615fa1: mov r8d, 0x22
141615fa7: mov r9b, 1
141615faa: call 0x140bd4de0
141615faf: lea r12, [rbp + 0x68]
141615fb3: mov r8d, 4
141615fb9: mov rcx, r12
141615fbc: mov rdx, r15
141615fbf: call 0x140bd4f20
141615fc4: shl r14, 4
141615fc8: lea rax, [r14 + r14*2]
141615fcc: mov qword ptr [rsp + 0x30], rax
141615fd1: mov qword ptr [rsp + 0x28], rbx
141615fd6: mov qword ptr [rsp + 0x20], r12
141615fdb: lea r8, [rip + 0x16698ed]
141615fe2: lea rcx, [rbp - 0x40]
141615fe6: mov r9d, 0x11
141615fec: mov rdx, rdi
141615fef: call 0x1416a2dc0
141615ff4: cmp dword ptr [rbp - 0x40], 0xb
141615ff8: jne 0x141616088
141615ffe: mov rax, qword ptr [rbp - 0x18]
141616002: mov qword ptr [rsi + 0x20], rax
141616006: movups xmm0, xmmword ptr [rbp - 0x38]
14161600a: movups xmm1, xmmword ptr [rbp - 0x28]
14161600e: movups xmmword ptr [rsi + 0x10], xmm1
141616012: movups xmmword ptr [rsi], xmm0
141616015: add rsp, 0x100
14161601c: pop rbx
14161601d: pop rdi
14161601e: pop rsi
14161601f: pop r12
141616021: pop r14
141616023: pop r15
141616025: pop rbp
141616026: ret 
141616027: lea rax, [rip + 0x1487a56]
14161602e: mov qword ptr [rbp - 0x40], rax
141616032: mov qword ptr [rbp - 0x38], 0x3f
14161603a: lea rax, [rbp - 0x40]
14161603e: mov qword ptr [rbp + 0x68], rax
141616042: lea rax, [rip - 0x150c1b9]
141616049: mov qword ptr [rbp + 0x70], rax
14161604d: lea rax, [rip + 0x1669774]
141616054: mov qword ptr [rbp + 0x10], rax
141616058: mov qword ptr [rbp + 0x18], 1
141616060: mov qword ptr [rbp + 0x30], 0
141616068: lea rax, [rbp + 0x68]
14161606c: mov qword ptr [rbp + 0x20], rax
141616070: mov qword ptr [rbp + 0x28], 1
141616078: lea rdx, [rip + 0x1669759]
14161607f: lea rcx, [rbp + 0x10]
141616083: call 0x1428d9390
141616088: movups xmm0, xmmword ptr [rbp]
14161608c: movaps xmmword ptr [rbp + 0x50], xmm0
141616090: movups xmm0, xmmword ptr [rbp - 0x40]
141616094: movups xmm1, xmmword ptr [rbp - 0x30]
141616098: movups xmm2, xmmword ptr [rbp - 0x20]
14161609c: movups xmm3, xmmword ptr [rbp - 0x10]
1416160a0: movaps xmmword ptr [rbp + 0x40], xmm3
1416160a4: movaps xmmword ptr [rbp + 0x30], xmm2
1416160a8: movaps xmmword ptr [rbp + 0x20], xmm1
1416160ac: movaps xmmword ptr [rbp + 0x10], xmm0
1416160b0: lea rax, [rip + 0x166a0f1]
1416160b7: mov qword ptr [rsp + 0x20], rax
1416160bc: lea rcx, [rip + 0x1668a25]
1416160c3: lea r9, [rip + 0x16689fe]
1416160ca: lea r8, [rbp + 0x10]
1416160ce: mov edx, 0x2b
1416160d3: call 0x1428d9760
1416160d8: ud2 
1416160da: nop word ptr [rax + rax]
1416160e0: mov qword ptr [rsp + 0x10], rdx
1416160e5: push rbp
1416160e6: push r15
1416160e8: push r14
1416160ea: push r12
1416160ec: push rsi
1416160ed: push rdi
1416160ee: push rbx
1416160ef: sub rsp, 0x40
1416160f3: lea rbp, [rdx + 0x80]
1416160fa: lea rcx, [rbp + 0x10]
1416160fe: call 0x1407cea70
141616103: nop 
141616104: add rsp, 0x40
141616108: pop rbx
141616109: pop rdi
14161610a: pop rsi
14161610b: pop r12
14161610d: pop r14
14161610f: pop r15
141616111: pop rbp
141616112: ret 
