14161f1e0: push rbp
14161f1e1: push r15
14161f1e3: push r14
14161f1e5: push r12
14161f1e7: push rsi
14161f1e8: push rdi
14161f1e9: push rbx
14161f1ea: sub rsp, 0x130
14161f1f1: lea rbp, [rsp + 0x80]
14161f1f9: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161f204: mov rdi, rdx
14161f207: mov rsi, rcx
14161f20a: test r8, r8
14161f20d: je 0x14161f249
14161f20f: mov rbx, r9
14161f212: mov rax, r9
14161f215: and rax, 0xf
14161f219: mov qword ptr [rbp - 0x40], rax
14161f21d: jne 0x14161f42e
14161f223: mov r15, r8
14161f226: mov rax, r8
14161f229: and rax, 3
14161f22d: mov qword ptr [rbp - 0x40], rax
14161f231: jne 0x14161f43f
14161f237: cmp rbx, 0x610
14161f23e: jae 0x14161f4da
14161f244: xor r12d, r12d
14161f247: jmp 0x14161f283
14161f249: movzx eax, byte ptr [rip + 0x20159b9]
14161f250: mov ebx, 0x600
14161f255: mov ecx, 0x600
14161f25a: mov edx, 4
14161f25f: call 0x140613c10
14161f264: test rax, rax
14161f267: je 0x14161f544
14161f26d: mov r15, rax
14161f270: mov r8d, 0x600
14161f276: mov rcx, rax
14161f279: xor edx, edx
14161f27b: call 0x142923c20
14161f280: mov r12b, 1
14161f283: mov qword ptr [rbp + 0x98], r15
14161f28a: mov byte ptr [rbp + 0xa7], r12b
14161f291: lea rcx, [rbp + 0x10]
14161f295: mov edx, 0x600
14161f29a: mov r8d, 0x22
14161f2a0: mov r9b, 2
14161f2a3: call 0x140bd4de0
14161f2a8: mov qword ptr [rbp + 0x98], r15
14161f2af: mov byte ptr [rbp + 0xa7], r12b
14161f2b6: lea r14, [rbp + 0x60]
14161f2ba: lea rdx, [rbp + 0x10]
14161f2be: mov r8d, 4
14161f2c4: mov rcx, r14
14161f2c7: call 0x140bd4f20
14161f2cc: mov qword ptr [rbp + 0x98], r15
14161f2d3: mov byte ptr [rbp + 0xa7], r12b
14161f2da: mov qword ptr [rsp + 0x30], rbx
14161f2df: mov qword ptr [rsp + 0x28], r15
14161f2e4: mov qword ptr [rsp + 0x20], r14
14161f2e9: lea r8, [rip + 0x1660caf]
14161f2f0: lea rcx, [rbp - 0x40]
14161f2f4: mov r9d, 0xe
14161f2fa: mov rdx, rdi
14161f2fd: call 0x1416a2dc0
14161f302: cmp dword ptr [rbp - 0x40], 0xb
14161f306: jne 0x14161f469
14161f30c: mov rax, qword ptr [rbp - 0x18]
14161f310: mov qword ptr [rbp + 0x80], rax
14161f317: movups xmm0, xmmword ptr [rbp - 0x38]
14161f31b: movups xmm1, xmmword ptr [rbp - 0x28]
14161f31f: movaps xmmword ptr [rbp + 0x70], xmm1
14161f323: movaps xmmword ptr [rbp + 0x60], xmm0
14161f327: test r12b, r12b
14161f32a: je 0x14161f33f
14161f32c: mov edx, 0x600
14161f331: mov r8d, 4
14161f337: mov rcx, r15
14161f33a: call 0x140613c20
14161f33f: lea rcx, [rbp + 0x60]
14161f343: mov rdx, rdi
14161f346: call 0x140bd4cd0
14161f34b: test rax, rax
14161f34e: je 0x14161f4c9
14161f354: cmp rdx, qword ptr [rbp + 0x78]
14161f358: jb 0x14161f3b3
14161f35a: mov rcx, rdx
14161f35d: and rcx, 0xf
14161f361: mov qword ptr [rbp - 0x40], rcx
14161f365: jne 0x14161f3d0
14161f367: mov rcx, rax
14161f36a: and rcx, 3
14161f36e: mov qword ptr [rbp - 0x40], rcx
14161f372: jne 0x14161f3ff
14161f378: mov rcx, qword ptr [rbp + 0x80]
14161f37f: mov qword ptr [rsi + 0x30], rcx
14161f383: movaps xmm0, xmmword ptr [rbp + 0x60]
14161f387: movaps xmm1, xmmword ptr [rbp + 0x70]
14161f38b: movups xmmword ptr [rsi + 0x20], xmm1
14161f38f: movups xmmword ptr [rsi + 0x10], xmm0
14161f393: shr rdx, 4
14161f397: mov qword ptr [rsi], rax
14161f39a: mov qword ptr [rsi + 8], rdx
14161f39e: mov rax, rsi
14161f3a1: add rsp, 0x130
14161f3a8: pop rbx
14161f3a9: pop rdi
14161f3aa: pop rsi
14161f3ab: pop r12
14161f3ad: pop r14
14161f3af: pop r15
14161f3b1: pop rbp
14161f3b2: ret 
14161f3b3: lea rcx, [rip + 0x1660e36]
14161f3ba: lea r8, [rip + 0x1660e67]
14161f3c1: mov edx, 0x37
14161f3c6: call 0x1428d9430
14161f3cb: jmp 0x14161f561
14161f3d0: mov qword ptr [rbp + 0x10], 0
14161f3d8: lea rax, [rip + 0x1660e69]
14161f3df: mov qword ptr [rsp + 0x20], rax
14161f3e4: lea r8, [rip + 0x1660e55]
14161f3eb: lea rdx, [rbp - 0x40]
14161f3ef: lea r9, [rbp + 0x10]
14161f3f3: xor ecx, ecx
14161f3f5: call 0x1428d9596
14161f3fa: jmp 0x14161f561
14161f3ff: mov qword ptr [rbp + 0x10], 0
14161f407: lea rax, [rip + 0x1660e52]
14161f40e: mov qword ptr [rsp + 0x20], rax
14161f413: lea r8, [rip + 0x1660e26]
14161f41a: lea rdx, [rbp - 0x40]
14161f41e: lea r9, [rbp + 0x10]
14161f422: xor ecx, ecx
14161f424: call 0x1428d9596
14161f429: jmp 0x14161f561
14161f42e: mov qword ptr [rbp + 0x10], 0
14161f436: lea rax, [rip + 0x16aab0b]
14161f43d: jmp 0x14161f44e
14161f43f: mov qword ptr [rbp + 0x10], 0
14161f447: lea rax, [rip + 0x16aab12]
14161f44e: mov qword ptr [rsp + 0x20], rax
14161f453: lea r8, [rip + 0x16aaae6]
14161f45a: lea rdx, [rbp - 0x40]
14161f45e: lea r9, [rbp + 0x10]
14161f462: xor ecx, ecx
14161f464: call 0x1428d9596
14161f469: mov dword ptr [rbp + 0x94], r12d
14161f470: mov qword ptr [rbp - 0x48], r15
14161f474: movups xmm0, xmmword ptr [rbp]
14161f478: movaps xmmword ptr [rbp + 0x50], xmm0
14161f47c: movups xmm0, xmmword ptr [rbp - 0x40]
14161f480: movups xmm1, xmmword ptr [rbp - 0x30]
14161f484: movups xmm2, xmmword ptr [rbp - 0x20]
14161f488: movups xmm3, xmmword ptr [rbp - 0x10]
14161f48c: movaps xmmword ptr [rbp + 0x40], xmm3
14161f490: movaps xmmword ptr [rbp + 0x30], xmm2
14161f494: movaps xmmword ptr [rbp + 0x20], xmm1
14161f498: movaps xmmword ptr [rbp + 0x10], xmm0
14161f49c: lea rax, [rip + 0x1660d05]
14161f4a3: mov qword ptr [rsp + 0x20], rax
14161f4a8: lea rcx, [rip + 0x165f639]
14161f4af: lea r9, [rip + 0x165f612]
14161f4b6: lea r8, [rbp + 0x10]
14161f4ba: mov edx, 0x2b
14161f4bf: call 0x1428d9760
14161f4c4: jmp 0x14161f561
14161f4c9: lea rcx, [rip + 0x1660d08]
14161f4d0: call 0x1428d9310
14161f4d5: jmp 0x14161f561
14161f4da: lea rax, [rip + 0x147dc46]
14161f4e1: mov qword ptr [rbp - 0x40], rax
14161f4e5: mov qword ptr [rbp - 0x38], 0x44
14161f4ed: lea rax, [rbp - 0x40]
14161f4f1: mov qword ptr [rbp + 0x60], rax
14161f4f5: lea rax, [rip - 0x151566c]
14161f4fc: mov qword ptr [rbp + 0x68], rax
14161f500: lea rax, [rip + 0x16602c1]
14161f507: mov qword ptr [rbp + 0x10], rax
14161f50b: mov qword ptr [rbp + 0x18], 1
14161f513: mov qword ptr [rbp + 0x30], 0
14161f51b: lea rax, [rbp + 0x60]
14161f51f: mov qword ptr [rbp + 0x20], rax
14161f523: mov qword ptr [rbp + 0x28], 1
14161f52b: mov byte ptr [rbp + 0xa7], 0
14161f532: lea rdx, [rip + 0x166029f]
14161f539: lea rcx, [rbp + 0x10]
14161f53d: call 0x1428d9390
14161f542: jmp 0x14161f561
14161f544: mov byte ptr [rbp + 0xa7], 0
14161f54b: lea r8, [rip + 0x165f55e]
14161f552: mov ecx, 4
14161f557: mov edx, 0x600
14161f55c: call 0x1428d8fe3
14161f561: ud2 
14161f563: nop word ptr cs:[rax + rax]
14161f570: mov qword ptr [rsp + 0x10], rdx
14161f575: push rbp
14161f576: push r15
14161f578: push r14
14161f57a: push r12
14161f57c: push rsi
14161f57d: push rdi
14161f57e: push rbx
14161f57f: sub rsp, 0x40
14161f583: lea rbp, [rdx + 0x80]
14161f58a: mov rsi, qword ptr [rbp - 0x48]
14161f58e: mov qword ptr [rbp + 0x98], rsi
14161f595: mov edi, dword ptr [rbp + 0x94]
14161f59b: mov byte ptr [rbp + 0xa7], dil
14161f5a2: lea rcx, [rbp + 0x10]
14161f5a6: call 0x1407cea70
14161f5ab: mov qword ptr [rbp + 0x98], rsi
14161f5b2: mov byte ptr [rbp + 0xa7], dil
14161f5b9: add rsp, 0x40
14161f5bd: pop rbx
14161f5be: pop rdi
14161f5bf: pop rsi
14161f5c0: pop r12
14161f5c2: pop r14
14161f5c4: pop r15
14161f5c6: pop rbp
14161f5c7: ret 
14161f5c8: nop dword ptr [rax + rax]
14161f5d0: mov qword ptr [rsp + 0x10], rdx
14161f5d5: push rbp
14161f5d6: push r15
14161f5d8: push r14
14161f5da: push r12
14161f5dc: push rsi
14161f5dd: push rdi
14161f5de: push rbx
14161f5df: sub rsp, 0x40
14161f5e3: lea rbp, [rdx + 0x80]
14161f5ea: test byte ptr [rbp + 0xa7], 1
14161f5f1: je 0x14161f60a
14161f5f3: mov rcx, qword ptr [rbp + 0x98]
14161f5fa: mov edx, 0x600
14161f5ff: mov r8d, 4
14161f605: call 0x140613c20
14161f60a: nop 
14161f60b: add rsp, 0x40
14161f60f: pop rbx
14161f610: pop rdi
14161f611: pop rsi
14161f612: pop r12
14161f614: pop r14
14161f616: pop r15
14161f618: pop rbp
14161f619: ret 
14161f61a: nop word ptr [rax + rax]
14161f620: mov qword ptr [rsp + 0x10], rdx
14161f625: push rbp
14161f626: push r15
14161f628: push r14
14161f62a: push r12
14161f62c: push rsi
14161f62d: push rdi
14161f62e: push rbx
14161f62f: sub rsp, 0x40
14161f633: lea rbp, [rdx + 0x80]
14161f63a: lea rcx, [rbp + 0x60]
14161f63e: call 0x140be24b0
14161f643: nop 
14161f644: add rsp, 0x40
14161f648: pop rbx
14161f649: pop rdi
14161f64a: pop rsi
14161f64b: pop r12
14161f64d: pop r14
14161f64f: pop r15
14161f651: pop rbp
14161f652: ret 
