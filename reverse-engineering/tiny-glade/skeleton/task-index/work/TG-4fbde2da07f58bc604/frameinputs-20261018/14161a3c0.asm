14161a3c0: push rbp
14161a3c1: push r15
14161a3c3: push r14
14161a3c5: push r12
14161a3c7: push rsi
14161a3c8: push rdi
14161a3c9: push rbx
14161a3ca: sub rsp, 0x100
14161a3d1: lea rbp, [rsp + 0x80]
14161a3d9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
14161a3e1: mov rsi, rcx
14161a3e4: test r8, r8
14161a3e7: sete al
14161a3ea: cmp r9, 0x9c41
14161a3f1: setb cl
14161a3f4: or cl, al
14161a3f6: je 0x14161a497
14161a3fc: mov r14, r9
14161a3ff: mov rbx, r8
14161a402: mov rdi, rdx
14161a405: lea r15, [rbp + 0x10]
14161a409: mov edx, 0x1d4c00
14161a40e: mov rcx, r15
14161a411: mov r8d, 0x22
14161a417: mov r9b, 1
14161a41a: call 0x140bd4de0
14161a41f: lea r12, [rbp + 0x68]
14161a423: mov r8d, 4
14161a429: mov rcx, r12
14161a42c: mov rdx, r15
14161a42f: call 0x140bd4f20
14161a434: shl r14, 4
14161a438: lea rax, [r14 + r14*2]
14161a43c: mov qword ptr [rsp + 0x30], rax
14161a441: mov qword ptr [rsp + 0x28], rbx
14161a446: mov qword ptr [rsp + 0x20], r12
14161a44b: lea r8, [rip + 0x1665c1b]
14161a452: lea rcx, [rbp - 0x40]
14161a456: mov r9d, 0x13
14161a45c: mov rdx, rdi
14161a45f: call 0x1416a2dc0
14161a464: cmp dword ptr [rbp - 0x40], 0xb
14161a468: jne 0x14161a4f8
14161a46e: mov rax, qword ptr [rbp - 0x18]
14161a472: mov qword ptr [rsi + 0x20], rax
14161a476: movups xmm0, xmmword ptr [rbp - 0x38]
14161a47a: movups xmm1, xmmword ptr [rbp - 0x28]
14161a47e: movups xmmword ptr [rsi + 0x10], xmm1
14161a482: movups xmmword ptr [rsi], xmm0
14161a485: add rsp, 0x100
14161a48c: pop rbx
14161a48d: pop rdi
14161a48e: pop rsi
14161a48f: pop r12
14161a491: pop r14
14161a493: pop r15
14161a495: pop rbp
14161a496: ret 
14161a497: lea rax, [rip + 0x1484206]
14161a49e: mov qword ptr [rbp - 0x40], rax
14161a4a2: mov qword ptr [rbp - 0x38], 0x44
14161a4aa: lea rax, [rbp - 0x40]
14161a4ae: mov qword ptr [rbp + 0x68], rax
14161a4b2: lea rax, [rip - 0x1510629]
14161a4b9: mov qword ptr [rbp + 0x70], rax
14161a4bd: lea rax, [rip + 0x1665304]
14161a4c4: mov qword ptr [rbp + 0x10], rax
14161a4c8: mov qword ptr [rbp + 0x18], 1
14161a4d0: mov qword ptr [rbp + 0x30], 0
14161a4d8: lea rax, [rbp + 0x68]
14161a4dc: mov qword ptr [rbp + 0x20], rax
14161a4e0: mov qword ptr [rbp + 0x28], 1
14161a4e8: lea rdx, [rip + 0x16652e9]
14161a4ef: lea rcx, [rbp + 0x10]
14161a4f3: call 0x1428d9390
14161a4f8: movups xmm0, xmmword ptr [rbp]
14161a4fc: movaps xmmword ptr [rbp + 0x50], xmm0
14161a500: movups xmm0, xmmword ptr [rbp - 0x40]
14161a504: movups xmm1, xmmword ptr [rbp - 0x30]
14161a508: movups xmm2, xmmword ptr [rbp - 0x20]
14161a50c: movups xmm3, xmmword ptr [rbp - 0x10]
14161a510: movaps xmmword ptr [rbp + 0x40], xmm3
14161a514: movaps xmmword ptr [rbp + 0x30], xmm2
14161a518: movaps xmmword ptr [rbp + 0x20], xmm1
14161a51c: movaps xmmword ptr [rbp + 0x10], xmm0
14161a520: lea rax, [rip + 0x1665c81]
14161a527: mov qword ptr [rsp + 0x20], rax
14161a52c: lea rcx, [rip + 0x16645b5]
14161a533: lea r9, [rip + 0x166458e]
14161a53a: lea r8, [rbp + 0x10]
14161a53e: mov edx, 0x2b
14161a543: call 0x1428d9760
14161a548: ud2 
14161a54a: nop word ptr [rax + rax]
14161a550: mov qword ptr [rsp + 0x10], rdx
14161a555: push rbp
14161a556: push r15
14161a558: push r14
14161a55a: push r12
14161a55c: push rsi
14161a55d: push rdi
14161a55e: push rbx
14161a55f: sub rsp, 0x40
14161a563: lea rbp, [rdx + 0x80]
14161a56a: lea rcx, [rbp + 0x10]
14161a56e: call 0x1407cea70
14161a573: nop 
14161a574: add rsp, 0x40
14161a578: pop rbx
14161a579: pop rdi
14161a57a: pop rsi
14161a57b: pop r12
14161a57d: pop r14
14161a57f: pop r15
14161a581: pop rbp
14161a582: ret 
