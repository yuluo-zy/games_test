141610600: push rbp
141610601: push r15
141610603: push r14
141610605: push r12
141610607: push rsi
141610608: push rdi
141610609: push rbx
14161060a: sub rsp, 0x100
141610611: lea rbp, [rsp + 0x80]
141610619: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141610621: mov rbx, r8
141610624: mov rdi, rdx
141610627: mov rsi, rcx
14161062a: test r8, r8
14161062d: je 0x1416106c2
141610633: mov r14, r9
141610636: mov rax, r9
141610639: and rax, 0xf
14161063d: mov qword ptr [rbp - 0x40], rax
141610641: jne 0x14161074e
141610647: mov rax, rbx
14161064a: and rax, 3
14161064e: mov qword ptr [rbp - 0x40], rax
141610652: jne 0x14161075f
141610658: cmp r14, 0x44d0
14161065f: jb 0x1416106c5
141610661: lea rax, [rip + 0x148dbc1]
141610668: mov qword ptr [rbp - 0x40], rax
14161066c: mov qword ptr [rbp - 0x38], 0x57
141610674: lea rax, [rbp - 0x40]
141610678: mov qword ptr [rbp + 0x68], rax
14161067c: lea rax, [rip - 0x15067f3]
141610683: mov qword ptr [rbp + 0x70], rax
141610687: lea rax, [rip + 0x166f13a]
14161068e: mov qword ptr [rbp + 0x10], rax
141610692: mov qword ptr [rbp + 0x18], 1
14161069a: mov qword ptr [rbp + 0x30], 0
1416106a2: lea rax, [rbp + 0x68]
1416106a6: mov qword ptr [rbp + 0x20], rax
1416106aa: mov qword ptr [rbp + 0x28], 1
1416106b2: lea rdx, [rip + 0x166f11f]
1416106b9: lea rcx, [rbp + 0x10]
1416106bd: call 0x1428d9390
1416106c2: xor r14d, r14d
1416106c5: lea r15, [rbp + 0x10]
1416106c9: mov edx, 0x44c0
1416106ce: mov rcx, r15
1416106d1: mov r8d, 0x22
1416106d7: mov r9b, 1
1416106da: call 0x140bd4de0
1416106df: lea r12, [rbp + 0x68]
1416106e3: mov r8d, 4
1416106e9: mov rcx, r12
1416106ec: mov rdx, r15
1416106ef: call 0x140bd4f20
1416106f4: mov qword ptr [rsp + 0x30], r14
1416106f9: mov qword ptr [rsp + 0x28], rbx
1416106fe: mov qword ptr [rsp + 0x20], r12
141610703: lea r8, [rip + 0x166efb6]
14161070a: lea rcx, [rbp - 0x40]
14161070e: mov r9d, 0xf
141610714: mov rdx, rdi
141610717: call 0x1416a2dc0
14161071c: cmp dword ptr [rbp - 0x40], 0xb
141610720: jne 0x141610789
141610722: mov rax, qword ptr [rbp - 0x18]
141610726: mov qword ptr [rsi + 0x20], rax
14161072a: movups xmm0, xmmword ptr [rbp - 0x38]
14161072e: movups xmm1, xmmword ptr [rbp - 0x28]
141610732: movups xmmword ptr [rsi + 0x10], xmm1
141610736: movups xmmword ptr [rsi], xmm0
141610739: mov rax, rsi
14161073c: add rsp, 0x100
141610743: pop rbx
141610744: pop rdi
141610745: pop rsi
141610746: pop r12
141610748: pop r14
14161074a: pop r15
14161074c: pop rbp
14161074d: ret 
14161074e: mov qword ptr [rbp + 0x10], 0
141610756: lea rax, [rip + 0x16b97eb]
14161075d: jmp 0x14161076e
14161075f: mov qword ptr [rbp + 0x10], 0
141610767: lea rax, [rip + 0x16b97f2]
14161076e: mov qword ptr [rsp + 0x20], rax
141610773: lea r8, [rip + 0x16b97c6]
14161077a: lea rdx, [rbp - 0x40]
14161077e: lea r9, [rbp + 0x10]
141610782: xor ecx, ecx
141610784: call 0x1428d9596
141610789: movups xmm0, xmmword ptr [rbp]
14161078d: movaps xmmword ptr [rbp + 0x50], xmm0
141610791: movups xmm0, xmmword ptr [rbp - 0x40]
141610795: movups xmm1, xmmword ptr [rbp - 0x30]
141610799: movups xmm2, xmmword ptr [rbp - 0x20]
14161079d: movups xmm3, xmmword ptr [rbp - 0x10]
1416107a1: movaps xmmword ptr [rbp + 0x40], xmm3
1416107a5: movaps xmmword ptr [rbp + 0x30], xmm2
1416107a9: movaps xmmword ptr [rbp + 0x20], xmm1
1416107ad: movaps xmmword ptr [rbp + 0x10], xmm0
1416107b1: lea rax, [rip + 0x166f9f0]
1416107b8: mov qword ptr [rsp + 0x20], rax
1416107bd: lea rcx, [rip + 0x166e324]
1416107c4: lea r9, [rip + 0x166e2fd]
1416107cb: lea r8, [rbp + 0x10]
1416107cf: mov edx, 0x2b
1416107d4: call 0x1428d9760
1416107d9: ud2 
1416107db: nop dword ptr [rax + rax]
1416107e0: mov qword ptr [rsp + 0x10], rdx
1416107e5: push rbp
1416107e6: push r15
1416107e8: push r14
1416107ea: push r12
1416107ec: push rsi
1416107ed: push rdi
1416107ee: push rbx
1416107ef: sub rsp, 0x40
1416107f3: lea rbp, [rdx + 0x80]
1416107fa: lea rcx, [rbp + 0x10]
1416107fe: call 0x1407cea70
141610803: nop 
141610804: add rsp, 0x40
141610808: pop rbx
141610809: pop rdi
14161080a: pop rsi
14161080b: pop r12
14161080d: pop r14
14161080f: pop r15
141610811: pop rbp
141610812: ret 
