1412beef0: push rbp
1412beef1: push rsi
1412beef2: sub rsp, 0xd8
1412beef9: lea rbp, [rsp + 0x80]
1412bef01: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bef09: mov rsi, rcx
1412bef0c: lea rcx, [rbp - 0x58]
1412bef10: xor r8d, r8d
1412bef13: call 0x1416146c0
1412bef18: movzx eax, byte ptr [rip + 0x2375cea]
1412bef1f: mov ecx, 0x28
1412bef24: mov edx, 8
1412bef29: call 0x140613c10
1412bef2e: test rax, rax
1412bef31: je 0x1412befe7
1412bef37: mov rcx, qword ptr [rbp - 0x38]
1412bef3b: mov qword ptr [rax + 0x20], rcx
1412bef3f: movups xmm0, xmmword ptr [rbp - 0x58]
1412bef43: movups xmm1, xmmword ptr [rbp - 0x48]
1412bef47: movups xmmword ptr [rax + 0x10], xmm1
1412bef4b: movups xmmword ptr [rax], xmm0
1412bef4e: lea rcx, [rip + 0x17df502]
1412bef55: mov qword ptr [rbp], rcx
1412bef59: mov qword ptr [rbp + 8], 0x55
1412bef61: mov qword ptr [rbp + 0x10], rax
1412bef65: lea rax, [rip + 0x196da04]
1412bef6c: mov qword ptr [rbp + 0x18], rax
1412bef70: mov qword ptr [rbp + 0x30], 0
1412bef78: mov byte ptr [rbp + 0x46], 5
1412bef7c: mov qword ptr [rbp + 0x20], rcx
1412bef80: mov qword ptr [rbp + 0x28], 0x55
1412bef88: movzx eax, byte ptr [rip + 0x2375c7a]
1412bef8f: mov ecx, 0x40
1412bef94: mov edx, 8
1412bef99: call 0x140613c10
1412bef9e: test rax, rax
1412befa1: je 0x1412beff8
1412befa3: movups xmm0, xmmword ptr [rbp + 0x10]
1412befa7: movups xmm1, xmmword ptr [rbp + 0x20]
1412befab: movups xmm2, xmmword ptr [rbp + 0x30]
1412befaf: movups xmm3, xmmword ptr [rbp + 0x40]
1412befb3: movups xmmword ptr [rax + 0x30], xmm3
1412befb7: movups xmmword ptr [rax + 0x20], xmm2
1412befbb: movups xmmword ptr [rax + 0x10], xmm1
1412befbf: movups xmmword ptr [rax], xmm0
1412befc2: movabs rdx, 0xd0f0827e7ff22799
1412befcc: mov rcx, rsi
1412befcf: mov r8, rax
1412befd2: call 0x140969bf0
1412befd7: cmp rax, 1
1412befdb: je 0x1412bf009
1412befdd: add rsp, 0xd8
1412befe4: pop rsi
1412befe5: pop rbp
1412befe6: ret 
1412befe7: mov ecx, 8
1412befec: mov edx, 0x28
1412beff1: call 0x1428d9000
1412beff6: jmp 0x1412bf007
1412beff8: mov ecx, 8
1412beffd: mov edx, 0x40
1412bf002: call 0x1428d9000
1412bf007: ud2 
1412bf009: mov rax, rdx
1412bf00c: add rdx, 0x10
1412bf010: mov rsi, rdx
1412bf013: cmp qword ptr [rax + 0x18], 0x55
1412bf018: jne 0x1412bf096
1412bf01a: mov rdx, qword ptr [rsi]
1412bf01d: lea rcx, [rip + 0x17df433]
1412bf024: mov r8d, 0x55
1412bf02a: call 0x142923bf0
1412bf02f: test eax, eax
1412bf031: jne 0x1412bf096
1412bf033: lea rax, [rip + 0x196e90e]
1412bf03a: mov qword ptr [rbp - 0x30], rax
1412bf03e: lea rax, [rip - 0xd8b7a5]
1412bf045: mov qword ptr [rbp - 0x28], rax
1412bf049: mov rax, rbp
1412bf04c: mov qword ptr [rbp - 0x20], rax
1412bf050: lea rax, [rip - 0x11b51c7]
1412bf057: mov qword ptr [rbp - 0x18], rax
1412bf05b: lea rax, [rip + 0x196e5c6]
1412bf062: mov qword ptr [rbp + 0x10], rax
1412bf066: mov qword ptr [rbp + 0x18], 3
1412bf06e: mov qword ptr [rbp + 0x30], 0
1412bf076: lea rax, [rbp - 0x30]
1412bf07a: mov qword ptr [rbp + 0x20], rax
1412bf07e: mov qword ptr [rbp + 0x28], 2
1412bf086: lea rdx, [rip + 0x196e5cb]
1412bf08d: lea rcx, [rbp + 0x10]
1412bf091: call 0x1428d9390
1412bf096: lea rax, [rip + 0x196e8ab]
1412bf09d: mov qword ptr [rbp + 0x10], rax
1412bf0a1: lea rax, [rip - 0xd8b808]
1412bf0a8: mov qword ptr [rbp + 0x18], rax
1412bf0ac: mov qword ptr [rbp + 0x20], rsi
1412bf0b0: lea rax, [rip - 0x11b5227]
1412bf0b7: mov qword ptr [rbp + 0x28], rax
1412bf0bb: mov rcx, rbp
1412bf0be: mov qword ptr [rbp + 0x30], rcx
1412bf0c2: mov qword ptr [rbp + 0x38], rax
1412bf0c6: lea rax, [rip + 0x196e49b]
1412bf0cd: mov qword ptr [rbp - 0x30], rax
1412bf0d1: mov qword ptr [rbp - 0x28], 3
1412bf0d9: mov qword ptr [rbp - 0x10], 0
1412bf0e1: lea rax, [rbp + 0x10]
1412bf0e5: mov qword ptr [rbp - 0x20], rax
1412bf0e9: mov qword ptr [rbp - 0x18], 3
1412bf0f1: lea rdx, [rip + 0x196e4d8]
1412bf0f8: lea rcx, [rbp - 0x30]
1412bf0fc: call 0x1428d9390
1412bf101: int3 
1412bf102: nop word ptr cs:[rax + rax]
1412bf110: mov qword ptr [rsp + 0x10], rdx
1412bf115: push rbp
1412bf116: push rsi
1412bf117: sub rsp, 0x28
1412bf11b: lea rbp, [rdx + 0x80]
1412bf122: lea rcx, [rbp - 0x58]
1412bf126: call 0x140be24b0
1412bf12b: nop 
1412bf12c: add rsp, 0x28
1412bf130: pop rsi
1412bf131: pop rbp
1412bf132: ret 
1412bf133: nop word ptr cs:[rax + rax]
1412bf140: mov qword ptr [rsp + 0x10], rdx
1412bf145: push rbp
1412bf146: push rsi
1412bf147: sub rsp, 0x28
1412bf14b: lea rbp, [rdx + 0x80]
1412bf152: lea rcx, [rbp + 0x10]
1412bf156: call 0x1400c04b0
1412bf15b: nop 
1412bf15c: add rsp, 0x28
1412bf160: pop rsi
1412bf161: pop rbp
1412bf162: ret 
