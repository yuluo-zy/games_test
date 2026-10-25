141617f80: push rbp
141617f81: push r15
141617f83: push r14
141617f85: push r12
141617f87: push rsi
141617f88: push rdi
141617f89: push rbx
141617f8a: sub rsp, 0x100
141617f91: lea rbp, [rsp + 0x80]
141617f99: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141617fa1: mov rsi, rcx
141617fa4: test r8, r8
141617fa7: sete al
141617faa: cmp r9, 0x40001
141617fb1: setb cl
141617fb4: or cl, al
141617fb6: je 0x141618053
141617fbc: mov r14, r9
141617fbf: mov rbx, r8
141617fc2: mov rdi, rdx
141617fc5: lea r15, [rbp + 0x10]
141617fc9: mov edx, 0x100000
141617fce: mov rcx, r15
141617fd1: mov r8d, 0x22
141617fd7: mov r9b, 1
141617fda: call 0x140bd4de0
141617fdf: lea r12, [rbp + 0x68]
141617fe3: mov r8d, 4
141617fe9: mov rcx, r12
141617fec: mov rdx, r15
141617fef: call 0x140bd4f20
141617ff4: shl r14, 2
141617ff8: mov qword ptr [rsp + 0x30], r14
141617ffd: mov qword ptr [rsp + 0x28], rbx
141618002: mov qword ptr [rsp + 0x20], r12
141618007: lea r8, [rip + 0x1667bb2]
14161800e: lea rcx, [rbp - 0x40]
141618012: mov r9d, 0xe
141618018: mov rdx, rdi
14161801b: call 0x1416a2dc0
141618020: cmp dword ptr [rbp - 0x40], 0xb
141618024: jne 0x1416180b4
14161802a: mov rax, qword ptr [rbp - 0x18]
14161802e: mov qword ptr [rsi + 0x20], rax
141618032: movups xmm0, xmmword ptr [rbp - 0x38]
141618036: movups xmm1, xmmword ptr [rbp - 0x28]
14161803a: movups xmmword ptr [rsi + 0x10], xmm1
14161803e: movups xmmword ptr [rsi], xmm0
141618041: add rsp, 0x100
141618048: pop rbx
141618049: pop rdi
14161804a: pop rsi
14161804b: pop r12
14161804d: pop r14
14161804f: pop r15
141618051: pop rbp
141618052: ret 
141618053: lea rax, [rip + 0x1484f1b]
14161805a: mov qword ptr [rbp - 0x40], rax
14161805e: mov qword ptr [rbp - 0x38], 0x2f
141618066: lea rax, [rbp - 0x40]
14161806a: mov qword ptr [rbp + 0x68], rax
14161806e: lea rax, [rip - 0x150e1e5]
141618075: mov qword ptr [rbp + 0x70], rax
141618079: lea rax, [rip + 0x1667748]
141618080: mov qword ptr [rbp + 0x10], rax
141618084: mov qword ptr [rbp + 0x18], 1
14161808c: mov qword ptr [rbp + 0x30], 0
141618094: lea rax, [rbp + 0x68]
141618098: mov qword ptr [rbp + 0x20], rax
14161809c: mov qword ptr [rbp + 0x28], 1
1416180a4: lea rdx, [rip + 0x166772d]
1416180ab: lea rcx, [rbp + 0x10]
1416180af: call 0x1428d9390
1416180b4: movups xmm0, xmmword ptr [rbp]
1416180b8: movaps xmmword ptr [rbp + 0x50], xmm0
1416180bc: movups xmm0, xmmword ptr [rbp - 0x40]
1416180c0: movups xmm1, xmmword ptr [rbp - 0x30]
1416180c4: movups xmm2, xmmword ptr [rbp - 0x20]
1416180c8: movups xmm3, xmmword ptr [rbp - 0x10]
1416180cc: movaps xmmword ptr [rbp + 0x40], xmm3
1416180d0: movaps xmmword ptr [rbp + 0x30], xmm2
1416180d4: movaps xmmword ptr [rbp + 0x20], xmm1
1416180d8: movaps xmmword ptr [rbp + 0x10], xmm0
1416180dc: lea rax, [rip + 0x16680c5]
1416180e3: mov qword ptr [rsp + 0x20], rax
1416180e8: lea rcx, [rip + 0x16669f9]
1416180ef: lea r9, [rip + 0x16669d2]
1416180f6: lea r8, [rbp + 0x10]
1416180fa: mov edx, 0x2b
1416180ff: call 0x1428d9760
141618104: ud2 
141618106: nop word ptr cs:[rax + rax]
141618110: mov qword ptr [rsp + 0x10], rdx
141618115: push rbp
141618116: push r15
141618118: push r14
14161811a: push r12
14161811c: push rsi
14161811d: push rdi
14161811e: push rbx
14161811f: sub rsp, 0x40
141618123: lea rbp, [rdx + 0x80]
14161812a: lea rcx, [rbp + 0x10]
14161812e: call 0x1407cea70
141618133: nop 
141618134: add rsp, 0x40
141618138: pop rbx
141618139: pop rdi
14161813a: pop rsi
14161813b: pop r12
14161813d: pop r14
14161813f: pop r15
141618141: pop rbp
141618142: ret 
