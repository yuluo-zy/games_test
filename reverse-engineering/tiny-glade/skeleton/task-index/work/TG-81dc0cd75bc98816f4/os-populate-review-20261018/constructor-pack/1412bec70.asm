1412bec70: push rbp
1412bec71: push rsi
1412bec72: sub rsp, 0xd8
1412bec79: lea rbp, [rsp + 0x80]
1412bec81: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bec89: mov rsi, rcx
1412bec8c: lea rcx, [rbp - 0x58]
1412bec90: xor r8d, r8d
1412bec93: call 0x1416101f0
1412bec98: movzx eax, byte ptr [rip + 0x2375f6a]
1412bec9f: mov ecx, 0x28
1412beca4: mov edx, 8
1412beca9: call 0x140613c10
1412becae: test rax, rax
1412becb1: je 0x1412bed67
1412becb7: mov rcx, qword ptr [rbp - 0x38]
1412becbb: mov qword ptr [rax + 0x20], rcx
1412becbf: movups xmm0, xmmword ptr [rbp - 0x58]
1412becc3: movups xmm1, xmmword ptr [rbp - 0x48]
1412becc7: movups xmmword ptr [rax + 0x10], xmm1
1412beccb: movups xmmword ptr [rax], xmm0
1412becce: lea rcx, [rip + 0x17de75e]
1412becd5: mov qword ptr [rbp], rcx
1412becd9: mov qword ptr [rbp + 8], 0x53
1412bece1: mov qword ptr [rbp + 0x10], rax
1412bece5: lea rax, [rip + 0x196dc4c]
1412becec: mov qword ptr [rbp + 0x18], rax
1412becf0: mov qword ptr [rbp + 0x30], 0
1412becf8: mov byte ptr [rbp + 0x46], 5
1412becfc: mov qword ptr [rbp + 0x20], rcx
1412bed00: mov qword ptr [rbp + 0x28], 0x53
1412bed08: movzx eax, byte ptr [rip + 0x2375efa]
1412bed0f: mov ecx, 0x40
1412bed14: mov edx, 8
1412bed19: call 0x140613c10
1412bed1e: test rax, rax
1412bed21: je 0x1412bed78
1412bed23: movups xmm0, xmmword ptr [rbp + 0x10]
1412bed27: movups xmm1, xmmword ptr [rbp + 0x20]
1412bed2b: movups xmm2, xmmword ptr [rbp + 0x30]
1412bed2f: movups xmm3, xmmword ptr [rbp + 0x40]
1412bed33: movups xmmword ptr [rax + 0x30], xmm3
1412bed37: movups xmmword ptr [rax + 0x20], xmm2
1412bed3b: movups xmmword ptr [rax + 0x10], xmm1
1412bed3f: movups xmmword ptr [rax], xmm0
1412bed42: movabs rdx, 0x530b5658f3632b30
1412bed4c: mov rcx, rsi
1412bed4f: mov r8, rax
1412bed52: call 0x140969bf0
1412bed57: cmp rax, 1
1412bed5b: je 0x1412bed89
1412bed5d: add rsp, 0xd8
1412bed64: pop rsi
1412bed65: pop rbp
1412bed66: ret 
1412bed67: mov ecx, 8
1412bed6c: mov edx, 0x28
1412bed71: call 0x1428d9000
1412bed76: jmp 0x1412bed87
1412bed78: mov ecx, 8
1412bed7d: mov edx, 0x40
1412bed82: call 0x1428d9000
1412bed87: ud2 
1412bed89: mov rax, rdx
1412bed8c: add rdx, 0x10
1412bed90: mov rsi, rdx
1412bed93: cmp qword ptr [rax + 0x18], 0x53
1412bed98: jne 0x1412bee16
1412bed9a: mov rdx, qword ptr [rsi]
1412bed9d: lea rcx, [rip + 0x17de68f]
1412beda4: mov r8d, 0x53
1412bedaa: call 0x142923bf0
1412bedaf: test eax, eax
1412bedb1: jne 0x1412bee16
1412bedb3: lea rax, [rip + 0x196f15e]
1412bedba: mov qword ptr [rbp - 0x30], rax
1412bedbe: lea rax, [rip - 0xd8b525]
1412bedc5: mov qword ptr [rbp - 0x28], rax
1412bedc9: mov rax, rbp
1412bedcc: mov qword ptr [rbp - 0x20], rax
1412bedd0: lea rax, [rip - 0x11b4f47]
1412bedd7: mov qword ptr [rbp - 0x18], rax
1412beddb: lea rax, [rip + 0x196e846]
1412bede2: mov qword ptr [rbp + 0x10], rax
1412bede6: mov qword ptr [rbp + 0x18], 3
1412bedee: mov qword ptr [rbp + 0x30], 0
1412bedf6: lea rax, [rbp - 0x30]
1412bedfa: mov qword ptr [rbp + 0x20], rax
1412bedfe: mov qword ptr [rbp + 0x28], 2
1412bee06: lea rdx, [rip + 0x196e84b]
1412bee0d: lea rcx, [rbp + 0x10]
1412bee11: call 0x1428d9390
1412bee16: lea rax, [rip + 0x196f0fb]
1412bee1d: mov qword ptr [rbp + 0x10], rax
1412bee21: lea rax, [rip - 0xd8b588]
1412bee28: mov qword ptr [rbp + 0x18], rax
1412bee2c: mov qword ptr [rbp + 0x20], rsi
1412bee30: lea rax, [rip - 0x11b4fa7]
1412bee37: mov qword ptr [rbp + 0x28], rax
1412bee3b: mov rcx, rbp
1412bee3e: mov qword ptr [rbp + 0x30], rcx
1412bee42: mov qword ptr [rbp + 0x38], rax
1412bee46: lea rax, [rip + 0x196e71b]
1412bee4d: mov qword ptr [rbp - 0x30], rax
1412bee51: mov qword ptr [rbp - 0x28], 3
1412bee59: mov qword ptr [rbp - 0x10], 0
1412bee61: lea rax, [rbp + 0x10]
1412bee65: mov qword ptr [rbp - 0x20], rax
1412bee69: mov qword ptr [rbp - 0x18], 3
1412bee71: lea rdx, [rip + 0x196e758]
1412bee78: lea rcx, [rbp - 0x30]
1412bee7c: call 0x1428d9390
1412bee81: int3 
1412bee82: nop word ptr cs:[rax + rax]
1412bee90: mov qword ptr [rsp + 0x10], rdx
1412bee95: push rbp
1412bee96: push rsi
1412bee97: sub rsp, 0x28
1412bee9b: lea rbp, [rdx + 0x80]
1412beea2: lea rcx, [rbp - 0x58]
1412beea6: call 0x140be24b0
1412beeab: nop 
1412beeac: add rsp, 0x28
1412beeb0: pop rsi
1412beeb1: pop rbp
1412beeb2: ret 
1412beeb3: nop word ptr cs:[rax + rax]
1412beec0: mov qword ptr [rsp + 0x10], rdx
1412beec5: push rbp
1412beec6: push rsi
1412beec7: sub rsp, 0x28
1412beecb: lea rbp, [rdx + 0x80]
1412beed2: lea rcx, [rbp + 0x10]
1412beed6: call 0x1400c04b0
1412beedb: nop 
1412beedc: add rsp, 0x28
1412beee0: pop rsi
1412beee1: pop rbp
1412beee2: ret 
