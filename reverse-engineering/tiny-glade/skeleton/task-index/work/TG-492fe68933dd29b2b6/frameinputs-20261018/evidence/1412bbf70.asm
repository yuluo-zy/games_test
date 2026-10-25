1412bbf70: push rbp
1412bbf71: push rsi
1412bbf72: sub rsp, 0xd8
1412bbf79: lea rbp, [rsp + 0x80]
1412bbf81: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bbf89: mov rsi, rcx
1412bbf8c: shl r9, 2
1412bbf90: lea rcx, [rbp - 0x58]
1412bbf94: call 0x14160ffc0
1412bbf99: movzx eax, byte ptr [rip + 0x2378c69]
1412bbfa0: mov ecx, 0x28
1412bbfa5: mov edx, 8
1412bbfaa: call 0x140613c10
1412bbfaf: test rax, rax
1412bbfb2: je 0x1412bc068
1412bbfb8: mov rcx, qword ptr [rbp - 0x38]
1412bbfbc: mov qword ptr [rax + 0x20], rcx
1412bbfc0: movups xmm0, xmmword ptr [rbp - 0x58]
1412bbfc4: movups xmm1, xmmword ptr [rbp - 0x48]
1412bbfc8: movups xmmword ptr [rax + 0x10], xmm1
1412bbfcc: movups xmmword ptr [rax], xmm0
1412bbfcf: lea rcx, [rip + 0x17e170e]
1412bbfd6: mov qword ptr [rbp], rcx
1412bbfda: mov qword ptr [rbp + 8], 0x3b
1412bbfe2: mov qword ptr [rbp + 0x10], rax
1412bbfe6: lea rax, [rip + 0x197055b]
1412bbfed: mov qword ptr [rbp + 0x18], rax
1412bbff1: mov qword ptr [rbp + 0x30], 0
1412bbff9: mov byte ptr [rbp + 0x46], 5
1412bbffd: mov qword ptr [rbp + 0x20], rcx
1412bc001: mov qword ptr [rbp + 0x28], 0x3b
1412bc009: movzx eax, byte ptr [rip + 0x2378bf9]
1412bc010: mov ecx, 0x40
1412bc015: mov edx, 8
1412bc01a: call 0x140613c10
1412bc01f: test rax, rax
1412bc022: je 0x1412bc079
1412bc024: movups xmm0, xmmword ptr [rbp + 0x10]
1412bc028: movups xmm1, xmmword ptr [rbp + 0x20]
1412bc02c: movups xmm2, xmmword ptr [rbp + 0x30]
1412bc030: movups xmm3, xmmword ptr [rbp + 0x40]
1412bc034: movups xmmword ptr [rax + 0x30], xmm3
1412bc038: movups xmmword ptr [rax + 0x20], xmm2
1412bc03c: movups xmmword ptr [rax + 0x10], xmm1
1412bc040: movups xmmword ptr [rax], xmm0
1412bc043: movabs rdx, 0xe58f5c956cbba85a
1412bc04d: mov rcx, rsi
1412bc050: mov r8, rax
1412bc053: call 0x140969bf0
1412bc058: cmp rax, 1
1412bc05c: je 0x1412bc08a
1412bc05e: add rsp, 0xd8
1412bc065: pop rsi
1412bc066: pop rbp
1412bc067: ret 
1412bc068: mov ecx, 8
1412bc06d: mov edx, 0x28
1412bc072: call 0x1428d9000
1412bc077: jmp 0x1412bc088
1412bc079: mov ecx, 8
1412bc07e: mov edx, 0x40
1412bc083: call 0x1428d9000
1412bc088: ud2 
1412bc08a: mov rax, rdx
1412bc08d: add rdx, 0x10
1412bc091: mov rsi, rdx
1412bc094: cmp qword ptr [rax + 0x18], 0x3b
1412bc099: jne 0x1412bc117
1412bc09b: mov rdx, qword ptr [rsi]
1412bc09e: lea rcx, [rip + 0x17e163f]
1412bc0a5: mov r8d, 0x3b
1412bc0ab: call 0x142923bf0
1412bc0b0: test eax, eax
1412bc0b2: jne 0x1412bc117
1412bc0b4: lea rax, [rip + 0x197142d]
1412bc0bb: mov qword ptr [rbp - 0x30], rax
1412bc0bf: lea rax, [rip - 0xd88826]
1412bc0c6: mov qword ptr [rbp - 0x28], rax
1412bc0ca: mov rax, rbp
1412bc0cd: mov qword ptr [rbp - 0x20], rax
1412bc0d1: lea rax, [rip - 0x11b2248]
1412bc0d8: mov qword ptr [rbp - 0x18], rax
1412bc0dc: lea rax, [rip + 0x1971545]
1412bc0e3: mov qword ptr [rbp + 0x10], rax
1412bc0e7: mov qword ptr [rbp + 0x18], 3
1412bc0ef: mov qword ptr [rbp + 0x30], 0
1412bc0f7: lea rax, [rbp - 0x30]
1412bc0fb: mov qword ptr [rbp + 0x20], rax
1412bc0ff: mov qword ptr [rbp + 0x28], 2
1412bc107: lea rdx, [rip + 0x197154a]
1412bc10e: lea rcx, [rbp + 0x10]
1412bc112: call 0x1428d9390
1412bc117: lea rax, [rip + 0x19713ca]
1412bc11e: mov qword ptr [rbp + 0x10], rax
1412bc122: lea rax, [rip - 0xd88889]
1412bc129: mov qword ptr [rbp + 0x18], rax
1412bc12d: mov qword ptr [rbp + 0x20], rsi
1412bc131: lea rax, [rip - 0x11b22a8]
1412bc138: mov qword ptr [rbp + 0x28], rax
1412bc13c: mov rcx, rbp
1412bc13f: mov qword ptr [rbp + 0x30], rcx
1412bc143: mov qword ptr [rbp + 0x38], rax
1412bc147: lea rax, [rip + 0x197141a]
1412bc14e: mov qword ptr [rbp - 0x30], rax
1412bc152: mov qword ptr [rbp - 0x28], 3
1412bc15a: mov qword ptr [rbp - 0x10], 0
1412bc162: lea rax, [rbp + 0x10]
1412bc166: mov qword ptr [rbp - 0x20], rax
1412bc16a: mov qword ptr [rbp - 0x18], 3
1412bc172: lea rdx, [rip + 0x1971457]
1412bc179: lea rcx, [rbp - 0x30]
1412bc17d: call 0x1428d9390
1412bc182: int3 
1412bc183: nop word ptr cs:[rax + rax]
1412bc190: mov qword ptr [rsp + 0x10], rdx
1412bc195: push rbp
1412bc196: push rsi
1412bc197: sub rsp, 0x28
1412bc19b: lea rbp, [rdx + 0x80]
1412bc1a2: lea rcx, [rbp - 0x58]
1412bc1a6: call 0x140be24b0
1412bc1ab: nop 
1412bc1ac: add rsp, 0x28
1412bc1b0: pop rsi
1412bc1b1: pop rbp
1412bc1b2: ret 
1412bc1b3: nop word ptr cs:[rax + rax]
1412bc1c0: mov qword ptr [rsp + 0x10], rdx
1412bc1c5: push rbp
1412bc1c6: push rsi
1412bc1c7: sub rsp, 0x28
1412bc1cb: lea rbp, [rdx + 0x80]
1412bc1d2: lea rcx, [rbp + 0x10]
1412bc1d6: call 0x1400c04b0
1412bc1db: nop 
1412bc1dc: add rsp, 0x28
1412bc1e0: pop rsi
1412bc1e1: pop rbp
1412bc1e2: ret 
