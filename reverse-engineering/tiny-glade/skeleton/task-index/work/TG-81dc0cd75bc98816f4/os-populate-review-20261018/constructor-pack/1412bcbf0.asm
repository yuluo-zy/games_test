1412bcbf0: push rbp
1412bcbf1: push rsi
1412bcbf2: sub rsp, 0xe8
1412bcbf9: lea rbp, [rsp + 0x80]
1412bcc01: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412bcc09: mov rsi, rcx
1412bcc0c: lea rcx, [rbp - 0x58]
1412bcc10: xor r8d, r8d
1412bcc13: call 0x14161ed70
1412bcc18: movzx eax, byte ptr [rip + 0x2377fea]
1412bcc1f: mov ecx, 0x38
1412bcc24: mov edx, 8
1412bcc29: call 0x140613c10
1412bcc2e: test rax, rax
1412bcc31: je 0x1412bccef
1412bcc37: mov rcx, qword ptr [rbp - 0x28]
1412bcc3b: mov qword ptr [rax + 0x30], rcx
1412bcc3f: movups xmm0, xmmword ptr [rbp - 0x58]
1412bcc43: movups xmm1, xmmword ptr [rbp - 0x48]
1412bcc47: movups xmm2, xmmword ptr [rbp - 0x38]
1412bcc4b: movups xmmword ptr [rax + 0x20], xmm2
1412bcc4f: movups xmmword ptr [rax + 0x10], xmm1
1412bcc53: movups xmmword ptr [rax], xmm0
1412bcc56: lea rcx, [rip + 0x17e1bb2]
1412bcc5d: mov qword ptr [rbp + 0x10], rcx
1412bcc61: mov qword ptr [rbp + 0x18], 0x57
1412bcc69: mov qword ptr [rbp + 0x20], rax
1412bcc6d: lea rax, [rip + 0x196f9ec]
1412bcc74: mov qword ptr [rbp + 0x28], rax
1412bcc78: mov qword ptr [rbp + 0x40], 0
1412bcc80: mov byte ptr [rbp + 0x56], 5
1412bcc84: mov qword ptr [rbp + 0x30], rcx
1412bcc88: mov qword ptr [rbp + 0x38], 0x57
1412bcc90: movzx eax, byte ptr [rip + 0x2377f72]
1412bcc97: mov ecx, 0x40
1412bcc9c: mov edx, 8
1412bcca1: call 0x140613c10
1412bcca6: test rax, rax
1412bcca9: je 0x1412bcd00
1412bccab: movups xmm0, xmmword ptr [rbp + 0x20]
1412bccaf: movups xmm1, xmmword ptr [rbp + 0x30]
1412bccb3: movups xmm2, xmmword ptr [rbp + 0x40]
1412bccb7: movups xmm3, xmmword ptr [rbp + 0x50]
1412bccbb: movups xmmword ptr [rax + 0x30], xmm3
1412bccbf: movups xmmword ptr [rax + 0x20], xmm2
1412bccc3: movups xmmword ptr [rax + 0x10], xmm1
1412bccc7: movups xmmword ptr [rax], xmm0
1412bccca: movabs rdx, 0x29faa4608bf58d67
1412bccd4: mov rcx, rsi
1412bccd7: mov r8, rax
1412bccda: call 0x140969bf0
1412bccdf: cmp rax, 1
1412bcce3: je 0x1412bcd11
1412bcce5: add rsp, 0xe8
1412bccec: pop rsi
1412bcced: pop rbp
1412bccee: ret 
1412bccef: mov ecx, 8
1412bccf4: mov edx, 0x38
1412bccf9: call 0x1428d9000
1412bccfe: jmp 0x1412bcd0f
1412bcd00: mov ecx, 8
1412bcd05: mov edx, 0x40
1412bcd0a: call 0x1428d9000
1412bcd0f: ud2 
1412bcd11: mov rax, rdx
1412bcd14: add rdx, 0x10
1412bcd18: mov rsi, rdx
1412bcd1b: cmp qword ptr [rax + 0x18], 0x57
1412bcd20: jne 0x1412bcd9f
1412bcd22: mov rdx, qword ptr [rsi]
1412bcd25: lea rcx, [rip + 0x17e1ae3]
1412bcd2c: mov r8d, 0x57
1412bcd32: call 0x142923bf0
1412bcd37: test eax, eax
1412bcd39: jne 0x1412bcd9f
1412bcd3b: lea rax, [rip + 0x19712a6]
1412bcd42: mov qword ptr [rbp - 0x20], rax
1412bcd46: lea rax, [rip - 0xd894ad]
1412bcd4d: mov qword ptr [rbp - 0x18], rax
1412bcd51: lea rax, [rbp + 0x10]
1412bcd55: mov qword ptr [rbp - 0x10], rax
1412bcd59: lea rax, [rip - 0x11b2ed0]
1412bcd60: mov qword ptr [rbp - 8], rax
1412bcd64: lea rax, [rip + 0x19708bd]
1412bcd6b: mov qword ptr [rbp + 0x20], rax
1412bcd6f: mov qword ptr [rbp + 0x28], 3
1412bcd77: mov qword ptr [rbp + 0x40], 0
1412bcd7f: lea rax, [rbp - 0x20]
1412bcd83: mov qword ptr [rbp + 0x30], rax
1412bcd87: mov qword ptr [rbp + 0x38], 2
1412bcd8f: lea rdx, [rip + 0x19708c2]
1412bcd96: lea rcx, [rbp + 0x20]
1412bcd9a: call 0x1428d9390
1412bcd9f: lea rax, [rip + 0x1971242]
1412bcda6: mov qword ptr [rbp + 0x20], rax
1412bcdaa: lea rax, [rip - 0xd89511]
1412bcdb1: mov qword ptr [rbp + 0x28], rax
1412bcdb5: mov qword ptr [rbp + 0x30], rsi
1412bcdb9: lea rax, [rip - 0x11b2f30]
1412bcdc0: mov qword ptr [rbp + 0x38], rax
1412bcdc4: lea rcx, [rbp + 0x10]
1412bcdc8: mov qword ptr [rbp + 0x40], rcx
1412bcdcc: mov qword ptr [rbp + 0x48], rax
1412bcdd0: lea rax, [rip + 0x1970791]
1412bcdd7: mov qword ptr [rbp - 0x20], rax
1412bcddb: mov qword ptr [rbp - 0x18], 3
1412bcde3: mov qword ptr [rbp], 0
1412bcdeb: lea rax, [rbp + 0x20]
1412bcdef: mov qword ptr [rbp - 0x10], rax
1412bcdf3: mov qword ptr [rbp - 8], 3
1412bcdfb: lea rdx, [rip + 0x19707ce]
1412bce02: lea rcx, [rbp - 0x20]
1412bce06: call 0x1428d9390
1412bce0b: int3 
1412bce0c: nop dword ptr [rax]
1412bce10: mov qword ptr [rsp + 0x10], rdx
1412bce15: push rbp
1412bce16: push rsi
1412bce17: sub rsp, 0x28
1412bce1b: lea rbp, [rdx + 0x80]
1412bce22: lea rcx, [rbp - 0x48]
1412bce26: call 0x140be24b0
1412bce2b: nop 
1412bce2c: add rsp, 0x28
1412bce30: pop rsi
1412bce31: pop rbp
1412bce32: ret 
1412bce33: nop word ptr cs:[rax + rax]
1412bce40: mov qword ptr [rsp + 0x10], rdx
1412bce45: push rbp
1412bce46: push rsi
1412bce47: sub rsp, 0x28
1412bce4b: lea rbp, [rdx + 0x80]
1412bce52: lea rcx, [rbp + 0x20]
1412bce56: call 0x1400c04b0
1412bce5b: nop 
1412bce5c: add rsp, 0x28
1412bce60: pop rsi
1412bce61: pop rbp
1412bce62: ret 
