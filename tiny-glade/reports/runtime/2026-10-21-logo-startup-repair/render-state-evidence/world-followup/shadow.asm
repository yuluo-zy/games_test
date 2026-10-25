0x1409cc270: push rbp
0x1409cc271: push r15
0x1409cc273: push r14
0x1409cc275: push r13
0x1409cc277: push r12
0x1409cc279: push rsi
0x1409cc27a: push rdi
0x1409cc27b: push rbx
0x1409cc27c: sub rsp, 0x9b8
0x1409cc283: lea rbp, [rsp + 0x80]
0x1409cc28b: movaps xmmword ptr [rbp + 0x920], xmm6
0x1409cc292: mov qword ptr [rbp + 0x918], 0xfffffffffffffffe
0x1409cc29d: mov rbx, r8
0x1409cc2a0: mov r13, rdx
0x1409cc2a3: mov qword ptr [rbp + 0x8d8], rcx
0x1409cc2aa: call 0x140c16a00
0x1409cc2af: test al, al
0x1409cc2b1: je 0x1409cc312
0x1409cc2b3: lea r14, [rip + 0x20f449e]
0x1409cc2ba: mov edi, 0x49
0x1409cc2bf: mov edx, 0x49
0x1409cc2c4: mov rcx, r14
0x1409cc2c7: call 0x1409c8970
0x1409cc2cc: cmp rax, 1
0x1409cc2d0: jne 0x1409cc387
0x1409cc2d6: mov r9, rdx
0x1409cc2d9: test rdx, rdx
0x1409cc2dc: je 0x1409cc31b
0x1409cc2de: cmp r9, 0x49
0x1409cc2e2: jae 0x1409cc319
0x1409cc2e4: lea rax, [rip + 0x20f446d]
0x1409cc2eb: cmp byte ptr [r9 + rax], 0xbf
0x1409cc2f0: jg 0x1409cc31b
0x1409cc2f2: lea rax, [rip + 0x20f4627]
0x1409cc2f9: mov qword ptr [rsp + 0x20], rax
0x1409cc2fe: lea rcx, [rip + 0x20f4453]
0x1409cc305: mov edx, 0x49
0x1409cc30a: xor r8d, r8d
0x1409cc30d: call 0x1428d9940
0x1409cc312: xor eax, eax
0x1409cc314: jmp 0x1409cc4ed
0x1409cc319: jne 0x1409cc2f2
0x1409cc31b: lea r14, [rip + 0x20f4436]
0x1409cc322: mov rcx, r14
0x1409cc325: mov rdx, r9
0x1409cc328: call 0x1409c8970
0x1409cc32d: cmp rax, 1
0x1409cc331: jne 0x1409cc387
0x1409cc333: mov r8, rdx
0x1409cc336: add r8, 2
0x1409cc33a: je 0x1409cc375
0x1409cc33c: cmp r8, 0x49
0x1409cc340: jae 0x1409cc373
0x1409cc342: lea rax, [rip + 0x20f440f]
0x1409cc349: cmp byte ptr [r8 + rax], 0xbf
0x1409cc34e: jg 0x1409cc375
0x1409cc350: lea rax, [rip + 0x20f45e1]
0x1409cc357: mov qword ptr [rsp + 0x20], rax
0x1409cc35c: lea rcx, [rip + 0x20f43f5]
0x1409cc363: mov edx, 0x49
0x1409cc368: mov r9d, 0x49
0x1409cc36e: call 0x1428d9940
0x1409cc373: jne 0x1409cc350
0x1409cc375: mov edi, 0x47
0x1409cc37a: sub rdi, rdx
0x1409cc37d: lea r14, [rip + 0x20f43d4]
0x1409cc384: add r14, r8
0x1409cc387: lea rax, [rip + 0x20f4938]
0x1409cc38e: lea r15, [rip + 0x20f48eb]
0x1409cc395: nop word ptr cs:[rax + rax]
0x1409cc3a0: movsx ecx, byte ptr [rax - 1]
0x1409cc3a4: test ecx, ecx
0x1409cc3a6: js 0x1409cc3c0
0x1409cc3a8: dec rax
0x1409cc3ab: cmp ecx, 0x5c
0x1409cc3ae: jne 0x1409cc419
0x1409cc3b0: jmp 0x1409cc42e
0x1409cc3b2: nop word ptr cs:[rax + rax]
0x1409cc3c0: movzx edx, byte ptr [rax - 2]
0x1409cc3c4: cmp dl, 0xc0
0x1409cc3c7: jge 0x1409cc3ee
0x1409cc3c9: movzx r8d, byte ptr [rax - 3]
0x1409cc3ce: cmp r8b, 0xc0
0x1409cc3d2: jge 0x1409cc3f7
0x1409cc3d4: movzx r9d, byte ptr [rax - 4]
0x1409cc3d9: add rax, -4
0x1409cc3dd: and r9d, 7
0x1409cc3e1: shl r9d, 6
0x1409cc3e5: and r8d, 0x3f
0x1409cc3e9: or r8d, r9d
0x1409cc3ec: jmp 0x1409cc3ff
0x1409cc3ee: add rax, -2
0x1409cc3f2: and edx, 0x1f
0x1409cc3f5: jmp 0x1409cc409
0x1409cc3f7: add rax, -3
0x1409cc3fb: and r8d, 0xf
0x1409cc3ff: shl r8d, 6
0x1409cc403: and edx, 0x3f
0x1409cc406: or edx, r8d
0x1409cc409: shl edx, 6
0x1409cc40c: and cl, 0x3f
0x1409cc40f: movzx ecx, cl
0x1409cc412: or ecx, edx
0x1409cc414: cmp ecx, 0x5c
0x1409cc417: je 0x1409cc42e
0x1409cc419: cmp ecx, 0x2f
0x1409cc41c: je 0x1409cc42e
0x1409cc41e: cmp rax, r15
0x1409cc421: jne 0x1409cc3a0
0x1409cc427: mov esi, 0x46
0x1409cc42c: jmp 0x1409cc47d
0x1409cc42e: lea r15, [rip + 0x20f484b]
0x1409cc435: sub rax, r15
0x1409cc438: mov r8, rax
0x1409cc43b: inc r8
0x1409cc43e: je 0x1409cc472
0x1409cc440: cmp r8, 0x46
0x1409cc444: jae 0x1409cc470
0x1409cc446: cmp byte ptr [r8 + r15], 0xbf
0x1409cc44b: jg 0x1409cc472
0x1409cc44d: lea rax, [rip + 0x20f44ac]
0x1409cc454: mov qword ptr [rsp + 0x20], rax
0x1409cc459: lea rcx, [rip + 0x20f4820]
0x1409cc460: mov edx, 0x46
0x1409cc465: mov r9d, 0x46
0x1409cc46b: call 0x1428d9940
0x1409cc470: jne 0x1409cc44d
0x1409cc472: mov esi, 0x45
0x1409cc477: sub rsi, rax
0x1409cc47a: add r15, r8
0x1409cc47d: call 0x140c18130
0x1409cc482: mov rcx, rax
0x1409cc485: mov rax, qword ptr [rax]
0x1409cc488: cmp rax, 1
0x1409cc48c: jne 0x1409cc92e
0x1409cc492: add rcx, 8
0x1409cc496: cmp qword ptr [rcx], 0
0x1409cc49a: jne 0x1409cc967
0x1409cc4a0: mov qword ptr [rcx], 0xffffffffffffffff
0x1409cc4a7: mov qword ptr [rbp + 0x908], rcx
0x1409cc4ae: add rcx, 8
0x1409cc4b2: mov qword ptr [rsp + 0x20], rsi
0x1409cc4b7: mov qword ptr [rsp + 0x30], 0
0x1409cc4c0: mov qword ptr [rsp + 0x28], 1
0x1409cc4c9: mov rdx, r14
0x1409cc4cc: mov r8, rdi
0x1409cc4cf: mov r9, r15
0x1409cc4d2: call 0x140c16d40
0x1409cc4d7: mov rcx, qword ptr [rbp + 0x908]
0x1409cc4de: inc qword ptr [rcx]
0x1409cc4e1: mov qword ptr [rbp + 0x900], rax
0x1409cc4e8: mov eax, 1
0x1409cc4ed: mov qword ptr [rbp + 0x8f8], rax
0x1409cc4f4: lea rcx, [r13 + 0x38]
0x1409cc4f8: movabs rdx, 0xa68561b396784eaa
0x1409cc502: movabs r8, 0xf5fd74e6faf4ed14
0x1409cc50c: mov qword ptr [rbp + 0x8e0], rcx
0x1409cc513: call 0x1400dd6f0
0x1409cc518: test rax, rax
0x1409cc51b: je 0x1409cc988
0x1409cc521: cmp qword ptr [r13 + 0x218], rdx
0x1409cc528: jbe 0x1409cc988
0x1409cc52e: mov rax, qword ptr [r13 + 0x210]
0x1409cc535: mov rcx, qword ptr [rax + rdx*8]
0x1409cc539: test rcx, rcx
0x1409cc53c: je 0x1409cc988
0x1409cc542: mov rax, qword ptr [r13 + 0x1e0]
0x1409cc549: not rcx
0x1409cc54c: lea rcx, [rcx + rcx*2]
0x1409cc550: shl rcx, 5
0x1409cc554: cmp qword ptr [rax + rcx + 0x20], 0
0x1409cc55a: je 0x1409cc988
0x1409cc560: add rax, rcx
0x1409cc563: mov rdx, qword ptr [rax + 0x10]
0x1409cc567: mov eax, dword ptr [r13 + 0x5b0]
0x1409cc56e: lea rcx, [rbp + 0x89c]
0x1409cc575: mov r8b, 0x11
0x1409cc578: call 0x14093cf30
0x1409cc57d: mov rax, qword ptr [rbx + 8]
0x1409cc581: mov rcx, qword ptr [rbx + 0x10]
0x1409cc585: shl rcx, 7
0x1409cc589: mov qword ptr [rbp + 0x8e8], rax
0x1409cc590: add rcx, rax
0x1409cc593: mov qword ptr [rbp + 0x908], rcx
0x1409cc59a: xor r14d, r14d
0x1409cc59d: lea rbx, [rbp - 0x30]
0x1409cc5a1: lea r12, [rbp + 0x7a0]
0x1409cc5a8: movss xmm6, dword ptr [rip + 0x1fdb090]
0x1409cc5b0: cmp r14, 3
0x1409cc5b4: jae 0x1409cc8ac
0x1409cc5ba: nop word ptr [rax + rax]
0x1409cc5c0: lea rcx, [rbp + 0x8a0]
0x1409cc5c7: call 0x140bd5790
0x1409cc5cc: mov rcx, r14
0x1409cc5cf: shl rcx, 0x20
0x1409cc5d3: mov qword ptr [rbp - 0x30], rcx
0x1409cc5d7: lea rcx, [rbp + 0x89c]
0x1409cc5de: mov qword ptr [rbp - 0x28], rcx
0x1409cc5e2: movabs rcx, 0x3f80000000000001
0x1409cc5ec: mov qword ptr [rbp - 0x20], rcx
0x1409cc5f0: mov rcx, qword ptr [rbp + 0x8d8]
0x1409cc5f7: mov qword ptr [rsp + 0x40], rcx
0x1409cc5fc: mov qword ptr [rsp + 0x38], r13
0x1409cc601: mov qword ptr [rsp + 0x30], rax
0x1409cc606: mov qword ptr [rsp + 0x28], rbx
0x1409cc60b: mov qword ptr [rsp + 0x20], 0
0x1409cc614: mov r8d, 7
0x1409cc61a: lea rcx, [rbp + 0x3b8]
0x1409cc621: lea rdx, [rip + 0x20f46b8]
0x1409cc628: mov r9, r12
0x1409cc62b: call 0x140867970
0x1409cc630: mov rsi, r13
0x1409cc633: mov byte ptr [rbp + 0x917], 1
0x1409cc63a: mov rcx, qword ptr [rbp + 0x8e0]
0x1409cc641: movabs rdx, 0x4bc362195b80f729
0x1409cc64b: movabs r8, 0x982798ce80cbcefb
0x1409cc655: call 0x1400dd6f0
0x1409cc65a: test rax, rax
0x1409cc65d: je 0x1409cc973
0x1409cc663: mov r8, rsi
0x1409cc666: cmp qword ptr [rsi + 0x218], rdx
0x1409cc66d: jbe 0x1409cc973
0x1409cc673: mov rax, qword ptr [r8 + 0x210]
0x1409cc67a: mov rcx, qword ptr [rax + rdx*8]
0x1409cc67e: test rcx, rcx
0x1409cc681: je 0x1409cc973
0x1409cc687: mov rax, qword ptr [r8 + 0x1e0]
0x1409cc68e: not rcx
0x1409cc691: lea rcx, [rcx + rcx*2]
0x1409cc695: shl rcx, 5
0x1409cc699: cmp qword ptr [rax + rcx + 0x20], 0
0x1409cc69f: je 0x1409cc973
0x1409cc6a5: add rax, rcx
0x1409cc6a8: mov rax, qword ptr [rax + 0x10]
0x1409cc6ac: mov rcx, qword ptr [rax]
0x1409cc6af: test rcx, rcx
0x1409cc6b2: js 0x1409cc6c0
0x1409cc6b4: cvtsi2ss xmm0, rcx
0x1409cc6b9: jmp 0x1409cc6d5
0x1409cc6bb: nop dword ptr [rax + rax]
0x1409cc6c0: mov rdx, rcx
0x1409cc6c3: shr rdx, 1
0x1409cc6c6: and ecx, 1
0x1409cc6c9: or rcx, rdx
0x1409cc6cc: cvtsi2ss xmm0, rcx
0x1409cc6d1: addss xmm0, xmm0
0x1409cc6d5: lea rcx, [r14 + 1]
0x1409cc6d9: mov qword ptr [rbp + 0x8d0], rcx
0x1409cc6e0: add r14b, 2
0x1409cc6e4: cvtsi2ss xmm1, dword ptr [rax + 8]
0x1409cc6e9: divss xmm1, xmm6
0x1409cc6ed: addss xmm1, xmm0
0x1409cc6f1: movss dword ptr [rbp + 0x8f0], xmm1
0x1409cc6f9: mov byte ptr [rbp + 0x8f4], r14b
0x1409cc700: mov r13, qword ptr [rbp + 0x8e8]
0x1409cc707: cmp r13, qword ptr [rbp + 0x908]
0x1409cc70e: je 0x1409cc874
0x1409cc714: nop word ptr cs:[rax + rax]
0x1409cc720: mov r15, qword ptr [r13 + 0x50]
0x1409cc724: mov rdi, qword ptr [r13 + 0x58]
0x1409cc728: mov byte ptr [rbp + 0x917], 1
0x1409cc72f: call 0x140be6750
0x1409cc734: mov r14d, eax
0x1409cc737: mov byte ptr [rbp + 0x917], 1
0x1409cc73e: xor ecx, ecx
0x1409cc740: call 0x1400d9b20
0x1409cc745: test rax, rax
0x1409cc748: je 0x1409cc952
0x1409cc74e: mov dword ptr [rbp + 0x848], r14d
0x1409cc755: mov word ptr [rbp + 0x850], 0
0x1409cc75e: mov dword ptr [rbp + 0x84c], 0x3f800000
0x1409cc768: mov qword ptr [rbp + 0x7a0], 1
0x1409cc773: mov qword ptr [rbp + 0x7a8], r15
0x1409cc77a: mov qword ptr [rbp + 0x840], rdi
0x1409cc781: mov qword ptr [rbp + 0x7b0], 0
0x1409cc78c: mov qword ptr [rbp + 0x7b8], 4
0x1409cc797: mov qword ptr [rbp + 0x7c0], 0
0x1409cc7a2: movups xmm0, xmmword ptr [rax]
0x1409cc7a5: inc qword ptr [rax]
0x1409cc7a8: movups xmm1, xmmword ptr [rip + 0x20c9f81]
0x1409cc7af: lea rax, [rbp + 0x810]
0x1409cc7b6: movups xmmword ptr [rax + 0x10], xmm1
0x1409cc7ba: movups xmm1, xmmword ptr [rip + 0x20c9f5f]
0x1409cc7c1: movups xmmword ptr [rax], xmm1
0x1409cc7c4: movups xmmword ptr [rbp + 0x830], xmm0
0x1409cc7cb: movabs rax, 0x100000003
0x1409cc7d5: mov qword ptr [rbp + 0x7e0], rax
0x1409cc7dc: mov qword ptr [rbp + 0x7c8], 0
0x1409cc7e7: mov qword ptr [rbp + 0x7d0], 1
0x1409cc7f2: mov qword ptr [rbp + 0x7d8], 0
0x1409cc7fd: mov byte ptr [rbp + 0x917], 1
0x1409cc804: mov rcx, rbx
0x1409cc807: mov rdx, r12
0x1409cc80a: lea r8, [rbp + 0x3b8]
0x1409cc811: mov r9b, 1
0x1409cc814: call 0x140905be0
0x1409cc819: mov byte ptr [rbp + 0x917], 1
0x1409cc820: mov rcx, r12
0x1409cc823: mov rdx, rbx
0x1409cc826: lea r8, [rbp + 0x8f0]
0x1409cc82d: mov r9, r13
0x1409cc830: call 0x14090b460
0x1409cc835: and byte ptr [rbp + 0x848], 0xcf
0x1409cc83c: mov rdi, qword ptr [r13 + 0x70]
0x1409cc840: mov r8d, 0xf8
0x1409cc846: mov rcx, rbx
0x1409cc849: mov rdx, r12
0x1409cc84c: call 0x142923c00
0x1409cc851: mov byte ptr [rbp + 0x917], 1
0x1409cc858: mov rcx, rbx
0x1409cc85b: mov rdx, r13
0x1409cc85e: mov r8, rsi
0x1409cc861: call rdi
0x1409cc863: sub r13, -0x80
0x1409cc867: cmp r13, qword ptr [rbp + 0x908]
0x1409cc86e: jne 0x1409cc720
0x1409cc874: mov r8d, 0x3e8
0x1409cc87a: mov rcx, rbx
0x1409cc87d: lea rdx, [rbp + 0x3b8]
0x1409cc884: call 0x142923c00
0x1409cc889: mov byte ptr [rbp + 0x917], 0
0x1409cc890: mov rcx, rbx
0x1409cc893: call 0x140868370
0x1409cc898: mov r14, qword ptr [rbp + 0x8d0]
0x1409cc89f: mov r13, rsi
0x1409cc8a2: cmp r14, 3
0x1409cc8a6: jb 0x1409cc5c0
0x1409cc8ac: cmp qword ptr [rbp + 0x8f8], 0
0x1409cc8b4: je 0x1409cc8fd
0x1409cc8b6: call 0x140c18130
0x1409cc8bb: mov rcx, rax
0x1409cc8be: mov rax, qword ptr [rax]
0x1409cc8c1: cmp rax, 1
0x1409cc8c5: jne 0x1409cc918
0x1409cc8c7: add rcx, 8
0x1409cc8cb: cmp qword ptr [rcx], 0
0x1409cc8cf: jne 0x1409cc967
0x1409cc8d5: mov qword ptr [rcx], 0xffffffffffffffff
0x1409cc8dc: mov qword ptr [rbp + 0x908], rcx
0x1409cc8e3: add rcx, 8
0x1409cc8e7: mov rdx, qword ptr [rbp + 0x900]
0x1409cc8ee: call 0x140c170f0
0x1409cc8f3: mov rax, qword ptr [rbp + 0x908]
0x1409cc8fa: inc qword ptr [rax]
0x1409cc8fd: movaps xmm6, xmmword ptr [rbp + 0x920]
0x1409cc904: add rsp, 0x9b8
0x1409cc90b: pop rbx
0x1409cc90c: pop rdi
0x1409cc90d: pop rsi
0x1409cc90e: pop r12
0x1409cc910: pop r13
0x1409cc912: pop r14
0x1409cc914: pop r15
0x1409cc916: pop rbp
0x1409cc917: ret 
0x1409cc918: test rax, rax
0x1409cc91b: jne 0x1409cc946
0x1409cc91d: xor edx, edx
0x1409cc91f: call 0x1428f87d0
0x1409cc924: mov rcx, rax
0x1409cc927: test rax, rax
0x1409cc92a: jne 0x1409cc8cb
0x1409cc92c: jmp 0x1409cc946
0x1409cc92e: test rax, rax
0x1409cc931: jne 0x1409cc946
0x1409cc933: xor edx, edx
0x1409cc935: call 0x1428f87d0
0x1409cc93a: mov rcx, rax
0x1409cc93d: test rax, rax
0x1409cc940: jne 0x1409cc496
0x1409cc946: lea rcx, [rip + 0x226e153]
0x1409cc94d: call 0x1428d80d0
0x1409cc952: mov byte ptr [rbp + 0x917], 1
0x1409cc959: lea rcx, [rip + 0x226e140]
0x1409cc960: call 0x1428d80d0
0x1409cc965: jmp 0x1409cc994
0x1409cc967: lea rcx, [rip + 0x226e202]
0x1409cc96e: call 0x1428d9250
0x1409cc973: mov byte ptr [rbp + 0x917], 1
0x1409cc97a: lea rcx, [rip + 0x20d9257]
0x1409cc981: call 0x1428d9310
0x1409cc986: jmp 0x1409cc994
0x1409cc988: lea rcx, [rip + 0x20f4339]
0x1409cc98f: call 0x1428d9310
0x1409cc994: ud2 
0x1409cc996: nop word ptr cs:[rax + rax]
0x1409cc9a0: mov qword ptr [rsp + 0x10], rdx
0x1409cc9a5: push rbp
0x1409cc9a6: push r15
0x1409cc9a8: push r14
0x1409cc9aa: push r13
0x1409cc9ac: push r12
0x1409cc9ae: push rsi
0x1409cc9af: push rdi
0x1409cc9b0: push rbx
0x1409cc9b1: sub rsp, 0x58
0x1409cc9b5: lea rbp, [rdx + 0x80]
0x1409cc9bc: movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cc9c1: mov rax, qword ptr [rbp + 0x908]
0x1409cc9c8: inc qword ptr [rax]
0x1409cc9cb: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cc9d0: add rsp, 0x58
0x1409cc9d4: pop rbx
0x1409cc9d5: pop rdi
0x1409cc9d6: pop rsi
0x1409cc9d7: pop r12
0x1409cc9d9: pop r13
0x1409cc9db: pop r14
0x1409cc9dd: pop r15
0x1409cc9df: pop rbp
0x1409cc9e0: ret 
0x1409cc9e1: nop word ptr cs:[rax + rax]
0x1409cc9f0: mov qword ptr [rsp + 0x10], rdx
0x1409cc9f5: push rbp
0x1409cc9f6: push r15
0x1409cc9f8: push r14
0x1409cc9fa: push r13
0x1409cc9fc: push r12
0x1409cc9fe: push rsi
0x1409cc9ff: push rdi
0x1409cca00: push rbx
0x1409cca01: sub rsp, 0x58
0x1409cca05: lea rbp, [rdx + 0x80]
0x1409cca0c: movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cca11: cmp qword ptr [rbp + 0x8f8], 0
0x1409cca19: je 0x1409cca4a
0x1409cca1b: lea rax, [rbp + 0x900]
0x1409cca22: mov qword ptr [rbp + 0x8c8], rax
0x1409cca29: lea rax, [rbp + 0x8c8]
0x1409cca30: mov qword ptr [rbp + 0x8c0], rax
0x1409cca37: lea rcx, [rip + 0x20f3e62]
0x1409cca3e: lea rdx, [rbp + 0x8c0]
0x1409cca45: call 0x14132f2d0
0x1409cca4a: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cca4f: add rsp, 0x58
0x1409cca53: pop rbx
0x1409cca54: pop rdi
0x1409cca55: pop rsi
0x1409cca56: pop r12
0x1409cca58: pop r13
0x1409cca5a: pop r14
0x1409cca5c: pop r15
0x1409cca5e: pop rbp
0x1409cca5f: ret 
0x1409cca60: mov qword ptr [rsp + 0x10], rdx
0x1409cca65: push rbp
0x1409cca66: push r15
0x1409cca68: push r14
0x1409cca6a: push r13
0x1409cca6c: push r12
0x1409cca6e: push rsi
0x1409cca6f: push rdi
0x1409cca70: push rbx
0x1409cca71: sub rsp, 0x58
0x1409cca75: lea rbp, [rdx + 0x80]
0x1409cca7c: movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cca81: mov rax, qword ptr [rbp + 0x908]
0x1409cca88: inc qword ptr [rax]
0x1409cca8b: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cca90: add rsp, 0x58
0x1409cca94: pop rbx
0x1409cca95: pop rdi
0x1409cca96: pop rsi
0x1409cca97: pop r12
0x1409cca99: pop r13
0x1409cca9b: pop r14
0x1409cca9d: pop r15
0x1409cca9f: pop rbp
0x1409ccaa0: ret 
0x1409ccaa1: nop word ptr cs:[rax + rax]
0x1409ccab0: mov qword ptr [rsp + 0x10], rdx
0x1409ccab5: push rbp
0x1409ccab6: push r15
0x1409ccab8: push r14
0x1409ccaba: push r13
0x1409ccabc: push r12
0x1409ccabe: push rsi
0x1409ccabf: push rdi
0x1409ccac0: push rbx
0x1409ccac1: sub rsp, 0x58
0x1409ccac5: lea rbp, [rdx + 0x80]
0x1409ccacc: movaps xmmword ptr [rsp + 0x40], xmm6
0x1409ccad1: cmp byte ptr [rbp + 0x917], 0
0x1409ccad8: je 0x1409ccafc
0x1409ccada: lea rcx, [rbp + 0x3b8]
0x1409ccae1: call 0x1408923c0
0x1409ccae6: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409ccaeb: add rsp, 0x58
0x1409ccaef: pop rbx
0x1409ccaf0: pop rdi
0x1409ccaf1: pop rsi
0x1409ccaf2: pop r12
0x1409ccaf4: pop r13
0x1409ccaf6: pop r14
0x1409ccaf8: pop r15
0x1409ccafa: pop rbp
0x1409ccafb: ret 
0x1409ccafc: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409ccb01: add rsp, 0x58
0x1409ccb05: pop rbx
0x1409ccb06: pop rdi
0x1409ccb07: pop rsi
0x1409ccb08: pop r12
0x1409ccb0a: pop r13
0x1409ccb0c: pop r14
0x1409ccb0e: pop r15
0x1409ccb10: pop rbp
0x1409ccb11: ret 
