1412bc470: push rbp
1412bc471: push rsi
1412bc472: sub rsp, 0xe8
1412bc479: lea rbp, [rsp + 0x80]
1412bc481: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412bc489: mov rsi, rcx
1412bc48c: lea rcx, [rbp - 0x58]
1412bc490: xor r8d, r8d
1412bc493: call 0x14161fea0
1412bc498: movzx eax, byte ptr [rip + 0x237876a]
1412bc49f: mov ecx, 0x38
1412bc4a4: mov edx, 8
1412bc4a9: call 0x140613c10
1412bc4ae: test rax, rax
1412bc4b1: je 0x1412bc56f
1412bc4b7: mov rcx, qword ptr [rbp - 0x28]
1412bc4bb: mov qword ptr [rax + 0x30], rcx
1412bc4bf: movups xmm0, xmmword ptr [rbp - 0x58]
1412bc4c3: movups xmm1, xmmword ptr [rbp - 0x48]
1412bc4c7: movups xmm2, xmmword ptr [rbp - 0x38]
1412bc4cb: movups xmmword ptr [rax + 0x20], xmm2
1412bc4cf: movups xmmword ptr [rax + 0x10], xmm1
1412bc4d3: movups xmmword ptr [rax], xmm0
1412bc4d6: lea rcx, [rip + 0x17e1cfe]
1412bc4dd: mov qword ptr [rbp + 0x10], rcx
1412bc4e1: mov qword ptr [rbp + 0x18], 0x4e
1412bc4e9: mov qword ptr [rbp + 0x20], rax
1412bc4ed: lea rax, [rip + 0x19700c4]
1412bc4f4: mov qword ptr [rbp + 0x28], rax
1412bc4f8: mov qword ptr [rbp + 0x40], 0
1412bc500: mov byte ptr [rbp + 0x56], 5
1412bc504: mov qword ptr [rbp + 0x30], rcx
1412bc508: mov qword ptr [rbp + 0x38], 0x4e
1412bc510: movzx eax, byte ptr [rip + 0x23786f2]
1412bc517: mov ecx, 0x40
1412bc51c: mov edx, 8
1412bc521: call 0x140613c10
1412bc526: test rax, rax
1412bc529: je 0x1412bc580
1412bc52b: movups xmm0, xmmword ptr [rbp + 0x20]
1412bc52f: movups xmm1, xmmword ptr [rbp + 0x30]
1412bc533: movups xmm2, xmmword ptr [rbp + 0x40]
1412bc537: movups xmm3, xmmword ptr [rbp + 0x50]
1412bc53b: movups xmmword ptr [rax + 0x30], xmm3
1412bc53f: movups xmmword ptr [rax + 0x20], xmm2
1412bc543: movups xmmword ptr [rax + 0x10], xmm1
1412bc547: movups xmmword ptr [rax], xmm0
1412bc54a: movabs rdx, 0x9248897309ea4fea
1412bc554: mov rcx, rsi
1412bc557: mov r8, rax
1412bc55a: call 0x140969bf0
1412bc55f: cmp rax, 1
1412bc563: je 0x1412bc591
1412bc565: add rsp, 0xe8
1412bc56c: pop rsi
1412bc56d: pop rbp
1412bc56e: ret 
1412bc56f: mov ecx, 8
1412bc574: mov edx, 0x38
1412bc579: call 0x1428d9000
1412bc57e: jmp 0x1412bc58f
1412bc580: mov ecx, 8
1412bc585: mov edx, 0x40
1412bc58a: call 0x1428d9000
1412bc58f: ud2 
1412bc591: mov rax, rdx
1412bc594: add rdx, 0x10
1412bc598: mov rsi, rdx
1412bc59b: cmp qword ptr [rax + 0x18], 0x4e
1412bc5a0: jne 0x1412bc61f
1412bc5a2: mov rdx, qword ptr [rsi]
1412bc5a5: lea rcx, [rip + 0x17e1c2f]
1412bc5ac: mov r8d, 0x4e
1412bc5b2: call 0x142923bf0
1412bc5b7: test eax, eax
1412bc5b9: jne 0x1412bc61f
1412bc5bb: lea rax, [rip + 0x197174e]
1412bc5c2: mov qword ptr [rbp - 0x20], rax
1412bc5c6: lea rax, [rip - 0xd88d2d]
1412bc5cd: mov qword ptr [rbp - 0x18], rax
1412bc5d1: lea rax, [rbp + 0x10]
1412bc5d5: mov qword ptr [rbp - 0x10], rax
1412bc5d9: lea rax, [rip - 0x11b2750]
1412bc5e0: mov qword ptr [rbp - 8], rax
1412bc5e4: lea rax, [rip + 0x197103d]
1412bc5eb: mov qword ptr [rbp + 0x20], rax
1412bc5ef: mov qword ptr [rbp + 0x28], 3
1412bc5f7: mov qword ptr [rbp + 0x40], 0
1412bc5ff: lea rax, [rbp - 0x20]
1412bc603: mov qword ptr [rbp + 0x30], rax
1412bc607: mov qword ptr [rbp + 0x38], 2
1412bc60f: lea rdx, [rip + 0x1971042]
1412bc616: lea rcx, [rbp + 0x20]
1412bc61a: call 0x1428d9390
1412bc61f: lea rax, [rip + 0x19716ea]
1412bc626: mov qword ptr [rbp + 0x20], rax
1412bc62a: lea rax, [rip - 0xd88d91]
1412bc631: mov qword ptr [rbp + 0x28], rax
1412bc635: mov qword ptr [rbp + 0x30], rsi
1412bc639: lea rax, [rip - 0x11b27b0]
1412bc640: mov qword ptr [rbp + 0x38], rax
1412bc644: lea rcx, [rbp + 0x10]
1412bc648: mov qword ptr [rbp + 0x40], rcx
1412bc64c: mov qword ptr [rbp + 0x48], rax
1412bc650: lea rax, [rip + 0x1970f11]
1412bc657: mov qword ptr [rbp - 0x20], rax
1412bc65b: mov qword ptr [rbp - 0x18], 3
1412bc663: mov qword ptr [rbp], 0
1412bc66b: lea rax, [rbp + 0x20]
1412bc66f: mov qword ptr [rbp - 0x10], rax
1412bc673: mov qword ptr [rbp - 8], 3
1412bc67b: lea rdx, [rip + 0x1970f4e]
1412bc682: lea rcx, [rbp - 0x20]
1412bc686: call 0x1428d9390
1412bc68b: int3 
1412bc68c: nop dword ptr [rax]
1412bc690: mov qword ptr [rsp + 0x10], rdx
1412bc695: push rbp
1412bc696: push rsi
1412bc697: sub rsp, 0x28
1412bc69b: lea rbp, [rdx + 0x80]
1412bc6a2: lea rcx, [rbp - 0x48]
1412bc6a6: call 0x140be24b0
1412bc6ab: nop 
1412bc6ac: add rsp, 0x28
1412bc6b0: pop rsi
1412bc6b1: pop rbp
1412bc6b2: ret 
1412bc6b3: nop word ptr cs:[rax + rax]
1412bc6c0: mov qword ptr [rsp + 0x10], rdx
1412bc6c5: push rbp
1412bc6c6: push rsi
1412bc6c7: sub rsp, 0x28
1412bc6cb: lea rbp, [rdx + 0x80]
1412bc6d2: lea rcx, [rbp + 0x20]
1412bc6d6: call 0x1400c04b0
1412bc6db: nop 
1412bc6dc: add rsp, 0x28
1412bc6e0: pop rsi
1412bc6e1: pop rbp
1412bc6e2: ret 
