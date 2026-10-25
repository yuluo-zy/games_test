14161a590: push rbp
14161a591: push r15
14161a593: push r14
14161a595: push r12
14161a597: push rsi
14161a598: push rdi
14161a599: push rbx
14161a59a: sub rsp, 0x100
14161a5a1: lea rbp, [rsp + 0x80]
14161a5a9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
14161a5b1: mov rsi, rcx
14161a5b4: test r8, r8
14161a5b7: sete al
14161a5ba: cmp r9, 0x20001
14161a5c1: setb cl
14161a5c4: or cl, al
14161a5c6: je 0x14161a667
14161a5cc: mov r14, r9
14161a5cf: mov rbx, r8
14161a5d2: mov rdi, rdx
14161a5d5: lea r15, [rbp + 0x10]
14161a5d9: mov edx, 0xa00000
14161a5de: mov rcx, r15
14161a5e1: mov r8d, 0x22
14161a5e7: mov r9b, 1
14161a5ea: call 0x140bd4de0
14161a5ef: lea r12, [rbp + 0x68]
14161a5f3: mov r8d, 4
14161a5f9: mov rcx, r12
14161a5fc: mov rdx, r15
14161a5ff: call 0x140bd4f20
14161a604: shl r14, 4
14161a608: lea rax, [r14 + r14*4]
14161a60c: mov qword ptr [rsp + 0x30], rax
14161a611: mov qword ptr [rsp + 0x28], rbx
14161a616: mov qword ptr [rsp + 0x20], r12
14161a61b: lea r8, [rip + 0x1665aae]
14161a622: lea rcx, [rbp - 0x40]
14161a626: mov r9d, 0x10
14161a62c: mov rdx, rdi
14161a62f: call 0x1416a2dc0
14161a634: cmp dword ptr [rbp - 0x40], 0xb
14161a638: jne 0x14161a6c8
14161a63e: mov rax, qword ptr [rbp - 0x18]
14161a642: mov qword ptr [rsi + 0x20], rax
14161a646: movups xmm0, xmmword ptr [rbp - 0x38]
14161a64a: movups xmm1, xmmword ptr [rbp - 0x28]
14161a64e: movups xmmword ptr [rsi + 0x10], xmm1
14161a652: movups xmmword ptr [rsi], xmm0
14161a655: add rsp, 0x100
14161a65c: pop rbx
14161a65d: pop rdi
14161a65e: pop rsi
14161a65f: pop r12
14161a661: pop r14
14161a663: pop r15
14161a665: pop rbp
14161a666: ret 
14161a667: lea rax, [rip + 0x14833a0]
14161a66e: mov qword ptr [rbp - 0x40], rax
14161a672: mov qword ptr [rbp - 0x38], 0x41
14161a67a: lea rax, [rbp - 0x40]
14161a67e: mov qword ptr [rbp + 0x68], rax
14161a682: lea rax, [rip - 0x15107f9]
14161a689: mov qword ptr [rbp + 0x70], rax
14161a68d: lea rax, [rip + 0x1665134]
14161a694: mov qword ptr [rbp + 0x10], rax
14161a698: mov qword ptr [rbp + 0x18], 1
14161a6a0: mov qword ptr [rbp + 0x30], 0
14161a6a8: lea rax, [rbp + 0x68]
14161a6ac: mov qword ptr [rbp + 0x20], rax
14161a6b0: mov qword ptr [rbp + 0x28], 1
14161a6b8: lea rdx, [rip + 0x1665119]
14161a6bf: lea rcx, [rbp + 0x10]
14161a6c3: call 0x1428d9390
14161a6c8: movups xmm0, xmmword ptr [rbp]
14161a6cc: movaps xmmword ptr [rbp + 0x50], xmm0
14161a6d0: movups xmm0, xmmword ptr [rbp - 0x40]
14161a6d4: movups xmm1, xmmword ptr [rbp - 0x30]
14161a6d8: movups xmm2, xmmword ptr [rbp - 0x20]
14161a6dc: movups xmm3, xmmword ptr [rbp - 0x10]
14161a6e0: movaps xmmword ptr [rbp + 0x40], xmm3
14161a6e4: movaps xmmword ptr [rbp + 0x30], xmm2
14161a6e8: movaps xmmword ptr [rbp + 0x20], xmm1
14161a6ec: movaps xmmword ptr [rbp + 0x10], xmm0
14161a6f0: lea rax, [rip + 0x1665ab1]
14161a6f7: mov qword ptr [rsp + 0x20], rax
14161a6fc: lea rcx, [rip + 0x16643e5]
14161a703: lea r9, [rip + 0x16643be]
14161a70a: lea r8, [rbp + 0x10]
14161a70e: mov edx, 0x2b
14161a713: call 0x1428d9760
14161a718: ud2 
14161a71a: nop word ptr [rax + rax]
14161a720: mov qword ptr [rsp + 0x10], rdx
14161a725: push rbp
14161a726: push r15
14161a728: push r14
14161a72a: push r12
14161a72c: push rsi
14161a72d: push rdi
14161a72e: push rbx
14161a72f: sub rsp, 0x40
14161a733: lea rbp, [rdx + 0x80]
14161a73a: lea rcx, [rbp + 0x10]
14161a73e: call 0x1407cea70
14161a743: nop 
14161a744: add rsp, 0x40
14161a748: pop rbx
14161a749: pop rdi
14161a74a: pop rsi
14161a74b: pop r12
14161a74d: pop r14
14161a74f: pop r15
14161a751: pop rbp
14161a752: ret 
