141616860: push rbp
141616861: push r15
141616863: push r14
141616865: push r12
141616867: push rsi
141616868: push rdi
141616869: push rbx
14161686a: sub rsp, 0x100
141616871: lea rbp, [rsp + 0x80]
141616879: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141616881: mov rsi, rcx
141616884: test r8, r8
141616887: sete al
14161688a: cmp r9, 0x3e9
141616891: setb cl
141616894: or cl, al
141616896: je 0x141616933
14161689c: mov r14, r9
14161689f: mov rbx, r8
1416168a2: mov rdi, rdx
1416168a5: lea r15, [rbp + 0x10]
1416168a9: mov edx, 0x7d00
1416168ae: mov rcx, r15
1416168b1: mov r8d, 0x22
1416168b7: mov r9b, 1
1416168ba: call 0x140bd4de0
1416168bf: lea r12, [rbp + 0x68]
1416168c3: mov r8d, 0x10
1416168c9: mov rcx, r12
1416168cc: mov rdx, r15
1416168cf: call 0x140bd4f20
1416168d4: shl r14, 5
1416168d8: mov qword ptr [rsp + 0x30], r14
1416168dd: mov qword ptr [rsp + 0x28], rbx
1416168e2: mov qword ptr [rsp + 0x20], r12
1416168e7: lea r8, [rip + 0x16690c2]
1416168ee: lea rcx, [rbp - 0x40]
1416168f2: mov r9d, 0x13
1416168f8: mov rdx, rdi
1416168fb: call 0x1416a2dc0
141616900: cmp dword ptr [rbp - 0x40], 0xb
141616904: jne 0x141616994
14161690a: mov rax, qword ptr [rbp - 0x18]
14161690e: mov qword ptr [rsi + 0x20], rax
141616912: movups xmm0, xmmword ptr [rbp - 0x38]
141616916: movups xmm1, xmmword ptr [rbp - 0x28]
14161691a: movups xmmword ptr [rsi + 0x10], xmm1
14161691e: movups xmmword ptr [rsi], xmm0
141616921: add rsp, 0x100
141616928: pop rbx
141616929: pop rdi
14161692a: pop rsi
14161692b: pop r12
14161692d: pop r14
14161692f: pop r15
141616931: pop rbp
141616932: ret 
141616933: lea rax, [rip + 0x1487723]
14161693a: mov qword ptr [rbp - 0x40], rax
14161693e: mov qword ptr [rbp - 0x38], 0x36
141616946: lea rax, [rbp - 0x40]
14161694a: mov qword ptr [rbp + 0x68], rax
14161694e: lea rax, [rip - 0x150cac5]
141616955: mov qword ptr [rbp + 0x70], rax
141616959: lea rax, [rip + 0x1668e68]
141616960: mov qword ptr [rbp + 0x10], rax
141616964: mov qword ptr [rbp + 0x18], 1
14161696c: mov qword ptr [rbp + 0x30], 0
141616974: lea rax, [rbp + 0x68]
141616978: mov qword ptr [rbp + 0x20], rax
14161697c: mov qword ptr [rbp + 0x28], 1
141616984: lea rdx, [rip + 0x1668e4d]
14161698b: lea rcx, [rbp + 0x10]
14161698f: call 0x1428d9390
141616994: movups xmm0, xmmword ptr [rbp]
141616998: movaps xmmword ptr [rbp + 0x50], xmm0
14161699c: movups xmm0, xmmword ptr [rbp - 0x40]
1416169a0: movups xmm1, xmmword ptr [rbp - 0x30]
1416169a4: movups xmm2, xmmword ptr [rbp - 0x20]
1416169a8: movups xmm3, xmmword ptr [rbp - 0x10]
1416169ac: movaps xmmword ptr [rbp + 0x40], xmm3
1416169b0: movaps xmmword ptr [rbp + 0x30], xmm2
1416169b4: movaps xmmword ptr [rbp + 0x20], xmm1
1416169b8: movaps xmmword ptr [rbp + 0x10], xmm0
1416169bc: lea rax, [rip + 0x16697e5]
1416169c3: mov qword ptr [rsp + 0x20], rax
1416169c8: lea rcx, [rip + 0x1668119]
1416169cf: lea r9, [rip + 0x16680f2]
1416169d6: lea r8, [rbp + 0x10]
1416169da: mov edx, 0x2b
1416169df: call 0x1428d9760
1416169e4: ud2 
1416169e6: nop word ptr cs:[rax + rax]
1416169f0: mov qword ptr [rsp + 0x10], rdx
1416169f5: push rbp
1416169f6: push r15
1416169f8: push r14
1416169fa: push r12
1416169fc: push rsi
1416169fd: push rdi
1416169fe: push rbx
1416169ff: sub rsp, 0x40
141616a03: lea rbp, [rdx + 0x80]
141616a0a: lea rcx, [rbp + 0x10]
141616a0e: call 0x1407cea70
141616a13: nop 
141616a14: add rsp, 0x40
141616a18: pop rbx
141616a19: pop rdi
141616a1a: pop rsi
141616a1b: pop r12
141616a1d: pop r14
141616a1f: pop r15
141616a21: pop rbp
141616a22: ret 
