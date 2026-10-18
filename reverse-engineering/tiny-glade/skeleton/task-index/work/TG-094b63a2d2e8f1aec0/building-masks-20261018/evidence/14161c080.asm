14161c080: push rbp
14161c081: push r15
14161c083: push r14
14161c085: push r12
14161c087: push rsi
14161c088: push rdi
14161c089: push rbx
14161c08a: sub rsp, 0x130
14161c091: lea rbp, [rsp + 0x80]
14161c099: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161c0a4: mov rdi, rdx
14161c0a7: mov rsi, rcx
14161c0aa: test r8, r8
14161c0ad: je 0x14161c0e9
14161c0af: mov rbx, r9
14161c0b2: mov rax, r9
14161c0b5: and rax, 3
14161c0b9: mov qword ptr [rbp - 0x40], rax
14161c0bd: jne 0x14161c30e
14161c0c3: mov r15, r8
14161c0c6: mov rax, r8
14161c0c9: and rax, 3
14161c0cd: mov qword ptr [rbp - 0x40], rax
14161c0d1: jne 0x14161c31f
14161c0d7: cmp rbx, 0x8004
14161c0de: jae 0x14161c3ba
14161c0e4: xor r12d, r12d
14161c0e7: jmp 0x14161c163
14161c0e9: movzx eax, byte ptr [rip + 0x2018b19]
14161c0f0: mov ecx, 0x8000
14161c0f5: mov edx, 4
14161c0fa: call 0x140613c10
14161c0ff: test rax, rax
14161c102: je 0x14161c424
14161c108: mov r15, rax
14161c10b: mov eax, 0x1c
14161c110: xorps xmm0, xmm0
14161c113: nop word ptr cs:[rax + rax]
14161c120: movups xmmword ptr [r15 + rax*4 - 0x70], xmm0
14161c126: movups xmmword ptr [r15 + rax*4 - 0x60], xmm0
14161c12c: movups xmmword ptr [r15 + rax*4 - 0x50], xmm0
14161c132: movups xmmword ptr [r15 + rax*4 - 0x40], xmm0
14161c138: movups xmmword ptr [r15 + rax*4 - 0x30], xmm0
14161c13e: movups xmmword ptr [r15 + rax*4 - 0x20], xmm0
14161c144: movups xmmword ptr [r15 + rax*4 - 0x10], xmm0
14161c14a: movups xmmword ptr [r15 + rax*4], xmm0
14161c14f: add rax, 0x20
14161c153: cmp rax, 0x201c
14161c159: jne 0x14161c120
14161c15b: mov r12b, 1
14161c15e: mov ebx, 0x8000
14161c163: mov qword ptr [rbp + 0x98], r15
14161c16a: mov byte ptr [rbp + 0xa7], r12b
14161c171: lea rcx, [rbp + 0x10]
14161c175: mov edx, 0x8000
14161c17a: mov r8d, 0x22
14161c180: mov r9b, 4
14161c183: call 0x140bd4de0
14161c188: mov qword ptr [rbp + 0x98], r15
14161c18f: mov byte ptr [rbp + 0xa7], r12b
14161c196: lea r14, [rbp + 0x60]
14161c19a: lea rdx, [rbp + 0x10]
14161c19e: mov r8d, 4
14161c1a4: mov rcx, r14
14161c1a7: call 0x140bd4f20
14161c1ac: mov qword ptr [rbp + 0x98], r15
14161c1b3: mov byte ptr [rbp + 0xa7], r12b
14161c1ba: mov qword ptr [rsp + 0x30], rbx
14161c1bf: mov qword ptr [rsp + 0x28], r15
14161c1c4: mov qword ptr [rsp + 0x20], r14
14161c1c9: lea r8, [rip + 0x1663c68]
14161c1d0: lea rcx, [rbp - 0x40]
14161c1d4: mov r9d, 0x17
14161c1da: mov rdx, rdi
14161c1dd: call 0x1416a2dc0
14161c1e2: cmp dword ptr [rbp - 0x40], 0xb
14161c1e6: jne 0x14161c349
14161c1ec: mov rax, qword ptr [rbp - 0x18]
14161c1f0: mov qword ptr [rbp + 0x80], rax
14161c1f7: movups xmm0, xmmword ptr [rbp - 0x38]
14161c1fb: movups xmm1, xmmword ptr [rbp - 0x28]
14161c1ff: movaps xmmword ptr [rbp + 0x70], xmm1
14161c203: movaps xmmword ptr [rbp + 0x60], xmm0
14161c207: test r12b, r12b
14161c20a: je 0x14161c21f
14161c20c: mov edx, 0x8000
14161c211: mov r8d, 4
14161c217: mov rcx, r15
14161c21a: call 0x140613c20
14161c21f: lea rcx, [rbp + 0x60]
14161c223: mov rdx, rdi
14161c226: call 0x140bd4cd0
14161c22b: test rax, rax
14161c22e: je 0x14161c3a9
14161c234: cmp rdx, qword ptr [rbp + 0x78]
14161c238: jb 0x14161c293
14161c23a: mov rcx, rdx
14161c23d: and rcx, 3
14161c241: mov qword ptr [rbp - 0x40], rcx
14161c245: jne 0x14161c2b0
14161c247: mov rcx, rax
14161c24a: and rcx, 3
14161c24e: mov qword ptr [rbp - 0x40], rcx
14161c252: jne 0x14161c2df
14161c258: mov rcx, qword ptr [rbp + 0x80]
14161c25f: mov qword ptr [rsi + 0x30], rcx
14161c263: movaps xmm0, xmmword ptr [rbp + 0x60]
14161c267: movaps xmm1, xmmword ptr [rbp + 0x70]
14161c26b: movups xmmword ptr [rsi + 0x20], xmm1
14161c26f: movups xmmword ptr [rsi + 0x10], xmm0
14161c273: shr rdx, 2
14161c277: mov qword ptr [rsi], rax
14161c27a: mov qword ptr [rsi + 8], rdx
14161c27e: mov rax, rsi
14161c281: add rsp, 0x130
14161c288: pop rbx
14161c289: pop rdi
14161c28a: pop rsi
14161c28b: pop r12
14161c28d: pop r14
14161c28f: pop r15
14161c291: pop rbp
14161c292: ret 
14161c293: lea rcx, [rip + 0x1663f56]
14161c29a: lea r8, [rip + 0x1663f87]
14161c2a1: mov edx, 0x37
14161c2a6: call 0x1428d9430
14161c2ab: jmp 0x14161c441
14161c2b0: mov qword ptr [rbp + 0x10], 0
14161c2b8: lea rax, [rip + 0x1663f89]
14161c2bf: mov qword ptr [rsp + 0x20], rax
14161c2c4: lea r8, [rip + 0x1663f75]
14161c2cb: lea rdx, [rbp - 0x40]
14161c2cf: lea r9, [rbp + 0x10]
14161c2d3: xor ecx, ecx
14161c2d5: call 0x1428d9596
14161c2da: jmp 0x14161c441
14161c2df: mov qword ptr [rbp + 0x10], 0
14161c2e7: lea rax, [rip + 0x1663f72]
14161c2ee: mov qword ptr [rsp + 0x20], rax
14161c2f3: lea r8, [rip + 0x1663f46]
14161c2fa: lea rdx, [rbp - 0x40]
14161c2fe: lea r9, [rbp + 0x10]
14161c302: xor ecx, ecx
14161c304: call 0x1428d9596
14161c309: jmp 0x14161c441
14161c30e: mov qword ptr [rbp + 0x10], 0
14161c316: lea rax, [rip + 0x16adc2b]
14161c31d: jmp 0x14161c32e
14161c31f: mov qword ptr [rbp + 0x10], 0
14161c327: lea rax, [rip + 0x16adc32]
14161c32e: mov qword ptr [rsp + 0x20], rax
14161c333: lea r8, [rip + 0x16adc06]
14161c33a: lea rdx, [rbp - 0x40]
14161c33e: lea r9, [rbp + 0x10]
14161c342: xor ecx, ecx
14161c344: call 0x1428d9596
14161c349: mov dword ptr [rbp + 0x94], r12d
14161c350: mov qword ptr [rbp - 0x48], r15
14161c354: movups xmm0, xmmword ptr [rbp]
14161c358: movaps xmmword ptr [rbp + 0x50], xmm0
14161c35c: movups xmm0, xmmword ptr [rbp - 0x40]
14161c360: movups xmm1, xmmword ptr [rbp - 0x30]
14161c364: movups xmm2, xmmword ptr [rbp - 0x20]
14161c368: movups xmm3, xmmword ptr [rbp - 0x10]
14161c36c: movaps xmmword ptr [rbp + 0x40], xmm3
14161c370: movaps xmmword ptr [rbp + 0x30], xmm2
14161c374: movaps xmmword ptr [rbp + 0x20], xmm1
14161c378: movaps xmmword ptr [rbp + 0x10], xmm0
14161c37c: lea rax, [rip + 0x1663e25]
14161c383: mov qword ptr [rsp + 0x20], rax
14161c388: lea rcx, [rip + 0x1662759]
14161c38f: lea r9, [rip + 0x1662732]
14161c396: lea r8, [rbp + 0x10]
14161c39a: mov edx, 0x2b
14161c39f: call 0x1428d9760
14161c3a4: jmp 0x14161c441
14161c3a9: lea rcx, [rip + 0x1663e28]
14161c3b0: call 0x1428d9310
14161c3b5: jmp 0x14161c441
14161c3ba: lea rax, [rip + 0x1482530]
14161c3c1: mov qword ptr [rbp - 0x40], rax
14161c3c5: mov qword ptr [rbp - 0x38], 0x50
14161c3cd: lea rax, [rbp - 0x40]
14161c3d1: mov qword ptr [rbp + 0x60], rax
14161c3d5: lea rax, [rip - 0x151254c]
14161c3dc: mov qword ptr [rbp + 0x68], rax
14161c3e0: lea rax, [rip + 0x16633e1]
14161c3e7: mov qword ptr [rbp + 0x10], rax
14161c3eb: mov qword ptr [rbp + 0x18], 1
14161c3f3: mov qword ptr [rbp + 0x30], 0
14161c3fb: lea rax, [rbp + 0x60]
14161c3ff: mov qword ptr [rbp + 0x20], rax
14161c403: mov qword ptr [rbp + 0x28], 1
14161c40b: mov byte ptr [rbp + 0xa7], 0
14161c412: lea rdx, [rip + 0x16633bf]
14161c419: lea rcx, [rbp + 0x10]
14161c41d: call 0x1428d9390
14161c422: jmp 0x14161c441
14161c424: mov byte ptr [rbp + 0xa7], 0
14161c42b: lea r8, [rip + 0x166267e]
14161c432: mov ecx, 4
14161c437: mov edx, 0x8000
14161c43c: call 0x1428d8fe3
14161c441: ud2 
14161c443: nop word ptr cs:[rax + rax]
14161c450: mov qword ptr [rsp + 0x10], rdx
14161c455: push rbp
14161c456: push r15
14161c458: push r14
14161c45a: push r12
14161c45c: push rsi
14161c45d: push rdi
14161c45e: push rbx
14161c45f: sub rsp, 0x40
14161c463: lea rbp, [rdx + 0x80]
14161c46a: mov rsi, qword ptr [rbp - 0x48]
14161c46e: mov qword ptr [rbp + 0x98], rsi
14161c475: mov edi, dword ptr [rbp + 0x94]
14161c47b: mov byte ptr [rbp + 0xa7], dil
14161c482: lea rcx, [rbp + 0x10]
14161c486: call 0x1407cea70
14161c48b: mov qword ptr [rbp + 0x98], rsi
14161c492: mov byte ptr [rbp + 0xa7], dil
14161c499: add rsp, 0x40
14161c49d: pop rbx
14161c49e: pop rdi
14161c49f: pop rsi
14161c4a0: pop r12
14161c4a2: pop r14
14161c4a4: pop r15
14161c4a6: pop rbp
14161c4a7: ret 
14161c4a8: nop dword ptr [rax + rax]
14161c4b0: mov qword ptr [rsp + 0x10], rdx
14161c4b5: push rbp
14161c4b6: push r15
14161c4b8: push r14
14161c4ba: push r12
14161c4bc: push rsi
14161c4bd: push rdi
14161c4be: push rbx
14161c4bf: sub rsp, 0x40
14161c4c3: lea rbp, [rdx + 0x80]
14161c4ca: test byte ptr [rbp + 0xa7], 1
14161c4d1: je 0x14161c4ea
14161c4d3: mov rcx, qword ptr [rbp + 0x98]
14161c4da: mov edx, 0x8000
14161c4df: mov r8d, 4
14161c4e5: call 0x140613c20
14161c4ea: nop 
14161c4eb: add rsp, 0x40
14161c4ef: pop rbx
14161c4f0: pop rdi
14161c4f1: pop rsi
14161c4f2: pop r12
14161c4f4: pop r14
14161c4f6: pop r15
14161c4f8: pop rbp
14161c4f9: ret 
14161c4fa: nop word ptr [rax + rax]
14161c500: mov qword ptr [rsp + 0x10], rdx
14161c505: push rbp
14161c506: push r15
14161c508: push r14
14161c50a: push r12
14161c50c: push rsi
14161c50d: push rdi
14161c50e: push rbx
14161c50f: sub rsp, 0x40
14161c513: lea rbp, [rdx + 0x80]
14161c51a: lea rcx, [rbp + 0x60]
14161c51e: call 0x140be24b0
14161c523: nop 
14161c524: add rsp, 0x40
14161c528: pop rbx
14161c529: pop rdi
14161c52a: pop rsi
14161c52b: pop r12
14161c52d: pop r14
14161c52f: pop r15
14161c531: pop rbp
14161c532: ret 
