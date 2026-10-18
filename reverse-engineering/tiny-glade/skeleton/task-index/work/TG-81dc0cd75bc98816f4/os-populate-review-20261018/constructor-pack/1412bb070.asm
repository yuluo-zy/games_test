1412bb070: push rbp
1412bb071: push rsi
1412bb072: sub rsp, 0xd8
1412bb079: lea rbp, [rsp + 0x80]
1412bb081: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bb089: mov rsi, rcx
1412bb08c: shl r9, 2
1412bb090: lea rcx, [rbp - 0x58]
1412bb094: call 0x1416132d0
1412bb099: movzx eax, byte ptr [rip + 0x2379b69]
1412bb0a0: mov ecx, 0x28
1412bb0a5: mov edx, 8
1412bb0aa: call 0x140613c10
1412bb0af: test rax, rax
1412bb0b2: je 0x1412bb168
1412bb0b8: mov rcx, qword ptr [rbp - 0x38]
1412bb0bc: mov qword ptr [rax + 0x20], rcx
1412bb0c0: movups xmm0, xmmword ptr [rbp - 0x58]
1412bb0c4: movups xmm1, xmmword ptr [rbp - 0x48]
1412bb0c8: movups xmmword ptr [rax + 0x10], xmm1
1412bb0cc: movups xmmword ptr [rax], xmm0
1412bb0cf: lea rcx, [rip + 0x17e36f9]
1412bb0d6: mov qword ptr [rbp], rcx
1412bb0da: mov qword ptr [rbp + 8], 0x40
1412bb0e2: mov qword ptr [rbp + 0x10], rax
1412bb0e6: lea rax, [rip + 0x197130b]
1412bb0ed: mov qword ptr [rbp + 0x18], rax
1412bb0f1: mov qword ptr [rbp + 0x30], 0
1412bb0f9: mov byte ptr [rbp + 0x46], 5
1412bb0fd: mov qword ptr [rbp + 0x20], rcx
1412bb101: mov qword ptr [rbp + 0x28], 0x40
1412bb109: movzx eax, byte ptr [rip + 0x2379af9]
1412bb110: mov ecx, 0x40
1412bb115: mov edx, 8
1412bb11a: call 0x140613c10
1412bb11f: test rax, rax
1412bb122: je 0x1412bb179
1412bb124: movups xmm0, xmmword ptr [rbp + 0x10]
1412bb128: movups xmm1, xmmword ptr [rbp + 0x20]
1412bb12c: movups xmm2, xmmword ptr [rbp + 0x30]
1412bb130: movups xmm3, xmmword ptr [rbp + 0x40]
1412bb134: movups xmmword ptr [rax + 0x30], xmm3
1412bb138: movups xmmword ptr [rax + 0x20], xmm2
1412bb13c: movups xmmword ptr [rax + 0x10], xmm1
1412bb140: movups xmmword ptr [rax], xmm0
1412bb143: movabs rdx, 0x5081c3fddd284158
1412bb14d: mov rcx, rsi
1412bb150: mov r8, rax
1412bb153: call 0x140969bf0
1412bb158: cmp rax, 1
1412bb15c: je 0x1412bb18a
1412bb15e: add rsp, 0xd8
1412bb165: pop rsi
1412bb166: pop rbp
1412bb167: ret 
1412bb168: mov ecx, 8
1412bb16d: mov edx, 0x28
1412bb172: call 0x1428d9000
1412bb177: jmp 0x1412bb188
1412bb179: mov ecx, 8
1412bb17e: mov edx, 0x40
1412bb183: call 0x1428d9000
1412bb188: ud2 
1412bb18a: mov rax, rdx
1412bb18d: add rdx, 0x10
1412bb191: mov rsi, rdx
1412bb194: cmp qword ptr [rax + 0x18], 0x40
1412bb199: jne 0x1412bb217
1412bb19b: mov rdx, qword ptr [rsi]
1412bb19e: lea rcx, [rip + 0x17e362a]
1412bb1a5: mov r8d, 0x40
1412bb1ab: call 0x142923bf0
1412bb1b0: test eax, eax
1412bb1b2: jne 0x1412bb217
1412bb1b4: lea rax, [rip + 0x1972815]
1412bb1bb: mov qword ptr [rbp - 0x30], rax
1412bb1bf: lea rax, [rip - 0xd87926]
1412bb1c6: mov qword ptr [rbp - 0x28], rax
1412bb1ca: mov rax, rbp
1412bb1cd: mov qword ptr [rbp - 0x20], rax
1412bb1d1: lea rax, [rip - 0x11b1348]
1412bb1d8: mov qword ptr [rbp - 0x18], rax
1412bb1dc: lea rax, [rip + 0x1972445]
1412bb1e3: mov qword ptr [rbp + 0x10], rax
1412bb1e7: mov qword ptr [rbp + 0x18], 3
1412bb1ef: mov qword ptr [rbp + 0x30], 0
1412bb1f7: lea rax, [rbp - 0x30]
1412bb1fb: mov qword ptr [rbp + 0x20], rax
1412bb1ff: mov qword ptr [rbp + 0x28], 2
1412bb207: lea rdx, [rip + 0x197244a]
1412bb20e: lea rcx, [rbp + 0x10]
1412bb212: call 0x1428d9390
1412bb217: lea rax, [rip + 0x19727b2]
1412bb21e: mov qword ptr [rbp + 0x10], rax
1412bb222: lea rax, [rip - 0xd87989]
1412bb229: mov qword ptr [rbp + 0x18], rax
1412bb22d: mov qword ptr [rbp + 0x20], rsi
1412bb231: lea rax, [rip - 0x11b13a8]
1412bb238: mov qword ptr [rbp + 0x28], rax
1412bb23c: mov rcx, rbp
1412bb23f: mov qword ptr [rbp + 0x30], rcx
1412bb243: mov qword ptr [rbp + 0x38], rax
1412bb247: lea rax, [rip + 0x197231a]
1412bb24e: mov qword ptr [rbp - 0x30], rax
1412bb252: mov qword ptr [rbp - 0x28], 3
1412bb25a: mov qword ptr [rbp - 0x10], 0
1412bb262: lea rax, [rbp + 0x10]
1412bb266: mov qword ptr [rbp - 0x20], rax
1412bb26a: mov qword ptr [rbp - 0x18], 3
1412bb272: lea rdx, [rip + 0x1972357]
1412bb279: lea rcx, [rbp - 0x30]
1412bb27d: call 0x1428d9390
1412bb282: int3 
1412bb283: nop word ptr cs:[rax + rax]
1412bb290: mov qword ptr [rsp + 0x10], rdx
1412bb295: push rbp
1412bb296: push rsi
1412bb297: sub rsp, 0x28
1412bb29b: lea rbp, [rdx + 0x80]
1412bb2a2: lea rcx, [rbp - 0x58]
1412bb2a6: call 0x140be24b0
1412bb2ab: nop 
1412bb2ac: add rsp, 0x28
1412bb2b0: pop rsi
1412bb2b1: pop rbp
1412bb2b2: ret 
1412bb2b3: nop word ptr cs:[rax + rax]
1412bb2c0: mov qword ptr [rsp + 0x10], rdx
1412bb2c5: push rbp
1412bb2c6: push rsi
1412bb2c7: sub rsp, 0x28
1412bb2cb: lea rbp, [rdx + 0x80]
1412bb2d2: lea rcx, [rbp + 0x10]
1412bb2d6: call 0x1400c04b0
1412bb2db: nop 
1412bb2dc: add rsp, 0x28
1412bb2e0: pop rsi
1412bb2e1: pop rbp
1412bb2e2: ret 
