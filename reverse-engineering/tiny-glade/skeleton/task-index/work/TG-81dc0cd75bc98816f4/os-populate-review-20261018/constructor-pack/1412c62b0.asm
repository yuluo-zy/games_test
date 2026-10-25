1412c62b0: push rbp
1412c62b1: push rsi
1412c62b2: sub rsp, 0xd8
1412c62b9: lea rbp, [rsp + 0x80]
1412c62c1: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c62c9: mov rsi, rcx
1412c62cc: lea rcx, [rbp - 0x58]
1412c62d0: xor r8d, r8d
1412c62d3: call 0x141610600
1412c62d8: movzx eax, byte ptr [rip + 0x236e92a]
1412c62df: mov ecx, 0x28
1412c62e4: mov edx, 8
1412c62e9: call 0x140613c10
1412c62ee: test rax, rax
1412c62f1: je 0x1412c63a7
1412c62f7: mov rcx, qword ptr [rbp - 0x38]
1412c62fb: mov qword ptr [rax + 0x20], rcx
1412c62ff: movups xmm0, xmmword ptr [rbp - 0x58]
1412c6303: movups xmm1, xmmword ptr [rbp - 0x48]
1412c6307: movups xmmword ptr [rax + 0x10], xmm1
1412c630b: movups xmmword ptr [rax], xmm0
1412c630e: lea rcx, [rip + 0x17d7f14]
1412c6315: mov qword ptr [rbp], rcx
1412c6319: mov qword ptr [rbp + 8], 0x57
1412c6321: mov qword ptr [rbp + 0x10], rax
1412c6325: lea rax, [rip + 0x1967054]
1412c632c: mov qword ptr [rbp + 0x18], rax
1412c6330: mov qword ptr [rbp + 0x30], 0
1412c6338: mov byte ptr [rbp + 0x46], 5
1412c633c: mov qword ptr [rbp + 0x20], rcx
1412c6340: mov qword ptr [rbp + 0x28], 0x57
1412c6348: movzx eax, byte ptr [rip + 0x236e8ba]
1412c634f: mov ecx, 0x40
1412c6354: mov edx, 8
1412c6359: call 0x140613c10
1412c635e: test rax, rax
1412c6361: je 0x1412c63b8
1412c6363: movups xmm0, xmmword ptr [rbp + 0x10]
1412c6367: movups xmm1, xmmword ptr [rbp + 0x20]
1412c636b: movups xmm2, xmmword ptr [rbp + 0x30]
1412c636f: movups xmm3, xmmword ptr [rbp + 0x40]
1412c6373: movups xmmword ptr [rax + 0x30], xmm3
1412c6377: movups xmmword ptr [rax + 0x20], xmm2
1412c637b: movups xmmword ptr [rax + 0x10], xmm1
1412c637f: movups xmmword ptr [rax], xmm0
1412c6382: movabs rdx, 0x59e5b18ab7bc931
1412c638c: mov rcx, rsi
1412c638f: mov r8, rax
1412c6392: call 0x140969bf0
1412c6397: cmp rax, 1
1412c639b: je 0x1412c63c9
1412c639d: add rsp, 0xd8
1412c63a4: pop rsi
1412c63a5: pop rbp
1412c63a6: ret 
1412c63a7: mov ecx, 8
1412c63ac: mov edx, 0x28
1412c63b1: call 0x1428d9000
1412c63b6: jmp 0x1412c63c7
1412c63b8: mov ecx, 8
1412c63bd: mov edx, 0x40
1412c63c2: call 0x1428d9000
1412c63c7: ud2 
1412c63c9: mov rax, rdx
1412c63cc: add rdx, 0x10
1412c63d0: mov rsi, rdx
1412c63d3: cmp qword ptr [rax + 0x18], 0x57
1412c63d8: jne 0x1412c6456
1412c63da: mov rdx, qword ptr [rsi]
1412c63dd: lea rcx, [rip + 0x17d7e45]
1412c63e4: mov r8d, 0x57
1412c63ea: call 0x142923bf0
1412c63ef: test eax, eax
1412c63f1: jne 0x1412c6456
1412c63f3: lea rax, [rip + 0x1967cde]
1412c63fa: mov qword ptr [rbp - 0x30], rax
1412c63fe: lea rax, [rip - 0xd92b65]
1412c6405: mov qword ptr [rbp - 0x28], rax
1412c6409: mov rax, rbp
1412c640c: mov qword ptr [rbp - 0x20], rax
1412c6410: lea rax, [rip - 0x11bc587]
1412c6417: mov qword ptr [rbp - 0x18], rax
1412c641b: lea rax, [rip + 0x1967206]
1412c6422: mov qword ptr [rbp + 0x10], rax
1412c6426: mov qword ptr [rbp + 0x18], 3
1412c642e: mov qword ptr [rbp + 0x30], 0
1412c6436: lea rax, [rbp - 0x30]
1412c643a: mov qword ptr [rbp + 0x20], rax
1412c643e: mov qword ptr [rbp + 0x28], 2
1412c6446: lea rdx, [rip + 0x196720b]
1412c644d: lea rcx, [rbp + 0x10]
1412c6451: call 0x1428d9390
1412c6456: lea rax, [rip + 0x1967c7b]
1412c645d: mov qword ptr [rbp + 0x10], rax
1412c6461: lea rax, [rip - 0xd92bc8]
1412c6468: mov qword ptr [rbp + 0x18], rax
1412c646c: mov qword ptr [rbp + 0x20], rsi
1412c6470: lea rax, [rip - 0x11bc5e7]
1412c6477: mov qword ptr [rbp + 0x28], rax
1412c647b: mov rcx, rbp
1412c647e: mov qword ptr [rbp + 0x30], rcx
1412c6482: mov qword ptr [rbp + 0x38], rax
1412c6486: lea rax, [rip + 0x19670db]
1412c648d: mov qword ptr [rbp - 0x30], rax
1412c6491: mov qword ptr [rbp - 0x28], 3
1412c6499: mov qword ptr [rbp - 0x10], 0
1412c64a1: lea rax, [rbp + 0x10]
1412c64a5: mov qword ptr [rbp - 0x20], rax
1412c64a9: mov qword ptr [rbp - 0x18], 3
1412c64b1: lea rdx, [rip + 0x1967118]
1412c64b8: lea rcx, [rbp - 0x30]
1412c64bc: call 0x1428d9390
1412c64c1: int3 
1412c64c2: nop word ptr cs:[rax + rax]
1412c64d0: mov qword ptr [rsp + 0x10], rdx
1412c64d5: push rbp
1412c64d6: push rsi
1412c64d7: sub rsp, 0x28
1412c64db: lea rbp, [rdx + 0x80]
1412c64e2: lea rcx, [rbp - 0x58]
1412c64e6: call 0x140be24b0
1412c64eb: nop 
1412c64ec: add rsp, 0x28
1412c64f0: pop rsi
1412c64f1: pop rbp
1412c64f2: ret 
1412c64f3: nop word ptr cs:[rax + rax]
1412c6500: mov qword ptr [rsp + 0x10], rdx
1412c6505: push rbp
1412c6506: push rsi
1412c6507: sub rsp, 0x28
1412c650b: lea rbp, [rdx + 0x80]
1412c6512: lea rcx, [rbp + 0x10]
1412c6516: call 0x1400c04b0
1412c651b: nop 
1412c651c: add rsp, 0x28
1412c6520: pop rsi
1412c6521: pop rbp
1412c6522: ret 
