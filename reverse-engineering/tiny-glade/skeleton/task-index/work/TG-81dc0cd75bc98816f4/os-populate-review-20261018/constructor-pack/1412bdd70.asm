1412bdd70: push rbp
1412bdd71: push rsi
1412bdd72: sub rsp, 0xd8
1412bdd79: lea rbp, [rsp + 0x80]
1412bdd81: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bdd89: mov rsi, rcx
1412bdd8c: lea rcx, [rbp - 0x58]
1412bdd90: xor r8d, r8d
1412bdd93: call 0x141616860
1412bdd98: mov qword ptr [rbp - 0x60], 0
1412bdda0: movzx eax, byte ptr [rip + 0x2376e62]
1412bdda7: mov ecx, 0x30
1412bddac: mov edx, 8
1412bddb1: call 0x140613c10
1412bddb6: test rax, rax
1412bddb9: je 0x1412bde6f
1412bddbf: movups xmm0, xmmword ptr [rbp - 0x60]
1412bddc3: movups xmm1, xmmword ptr [rbp - 0x50]
1412bddc7: movups xmm2, xmmword ptr [rbp - 0x40]
1412bddcb: movups xmmword ptr [rax + 0x20], xmm2
1412bddcf: movups xmmword ptr [rax + 0x10], xmm1
1412bddd3: movups xmmword ptr [rax], xmm0
1412bddd6: lea rcx, [rip + 0x17e0280]
1412bdddd: mov qword ptr [rbp], rcx
1412bdde1: mov qword ptr [rbp + 8], 0x36
1412bdde9: mov qword ptr [rbp + 0x10], rax
1412bdded: lea rax, [rip + 0x196e9f4]
1412bddf4: mov qword ptr [rbp + 0x18], rax
1412bddf8: mov qword ptr [rbp + 0x30], 0
1412bde00: mov byte ptr [rbp + 0x46], 5
1412bde04: mov qword ptr [rbp + 0x20], rcx
1412bde08: mov qword ptr [rbp + 0x28], 0x36
1412bde10: movzx eax, byte ptr [rip + 0x2376df2]
1412bde17: mov ecx, 0x40
1412bde1c: mov edx, 8
1412bde21: call 0x140613c10
1412bde26: test rax, rax
1412bde29: je 0x1412bde80
1412bde2b: movups xmm0, xmmword ptr [rbp + 0x10]
1412bde2f: movups xmm1, xmmword ptr [rbp + 0x20]
1412bde33: movups xmm2, xmmword ptr [rbp + 0x30]
1412bde37: movups xmm3, xmmword ptr [rbp + 0x40]
1412bde3b: movups xmmword ptr [rax + 0x30], xmm3
1412bde3f: movups xmmword ptr [rax + 0x20], xmm2
1412bde43: movups xmmword ptr [rax + 0x10], xmm1
1412bde47: movups xmmword ptr [rax], xmm0
1412bde4a: movabs rdx, 0x3d1661290bab42e1
1412bde54: mov rcx, rsi
1412bde57: mov r8, rax
1412bde5a: call 0x140969bf0
1412bde5f: cmp rax, 1
1412bde63: je 0x1412bde91
1412bde65: add rsp, 0xd8
1412bde6c: pop rsi
1412bde6d: pop rbp
1412bde6e: ret 
1412bde6f: mov ecx, 8
1412bde74: mov edx, 0x30
1412bde79: call 0x1428d9000
1412bde7e: jmp 0x1412bde8f
1412bde80: mov ecx, 8
1412bde85: mov edx, 0x40
1412bde8a: call 0x1428d9000
1412bde8f: ud2 
1412bde91: mov rax, rdx
1412bde94: add rdx, 0x10
1412bde98: mov rsi, rdx
1412bde9b: cmp qword ptr [rax + 0x18], 0x36
1412bdea0: jne 0x1412bdf1e
1412bdea2: mov rdx, qword ptr [rsi]
1412bdea5: lea rcx, [rip + 0x17e01b1]
1412bdeac: mov r8d, 0x36
1412bdeb2: call 0x142923bf0
1412bdeb7: test eax, eax
1412bdeb9: jne 0x1412bdf1e
1412bdebb: lea rax, [rip + 0x196fe06]
1412bdec2: mov qword ptr [rbp - 0x30], rax
1412bdec6: lea rax, [rip - 0xd8a62d]
1412bdecd: mov qword ptr [rbp - 0x28], rax
1412bded1: mov rax, rbp
1412bded4: mov qword ptr [rbp - 0x20], rax
1412bded8: lea rax, [rip - 0x11b404f]
1412bdedf: mov qword ptr [rbp - 0x18], rax
1412bdee3: lea rax, [rip + 0x196f73e]
1412bdeea: mov qword ptr [rbp + 0x10], rax
1412bdeee: mov qword ptr [rbp + 0x18], 3
1412bdef6: mov qword ptr [rbp + 0x30], 0
1412bdefe: lea rax, [rbp - 0x30]
1412bdf02: mov qword ptr [rbp + 0x20], rax
1412bdf06: mov qword ptr [rbp + 0x28], 2
1412bdf0e: lea rdx, [rip + 0x196f743]
1412bdf15: lea rcx, [rbp + 0x10]
1412bdf19: call 0x1428d9390
1412bdf1e: lea rax, [rip + 0x196fda3]
1412bdf25: mov qword ptr [rbp + 0x10], rax
1412bdf29: lea rax, [rip - 0xd8a690]
1412bdf30: mov qword ptr [rbp + 0x18], rax
1412bdf34: mov qword ptr [rbp + 0x20], rsi
1412bdf38: lea rax, [rip - 0x11b40af]
1412bdf3f: mov qword ptr [rbp + 0x28], rax
1412bdf43: mov rcx, rbp
1412bdf46: mov qword ptr [rbp + 0x30], rcx
1412bdf4a: mov qword ptr [rbp + 0x38], rax
1412bdf4e: lea rax, [rip + 0x196f613]
1412bdf55: mov qword ptr [rbp - 0x30], rax
1412bdf59: mov qword ptr [rbp - 0x28], 3
1412bdf61: mov qword ptr [rbp - 0x10], 0
1412bdf69: lea rax, [rbp + 0x10]
1412bdf6d: mov qword ptr [rbp - 0x20], rax
1412bdf71: mov qword ptr [rbp - 0x18], 3
1412bdf79: lea rdx, [rip + 0x196f650]
1412bdf80: lea rcx, [rbp - 0x30]
1412bdf84: call 0x1428d9390
1412bdf89: int3 
1412bdf8a: nop word ptr [rax + rax]
1412bdf90: mov qword ptr [rsp + 0x10], rdx
1412bdf95: push rbp
1412bdf96: push rsi
1412bdf97: sub rsp, 0x28
1412bdf9b: lea rbp, [rdx + 0x80]
1412bdfa2: lea rcx, [rbp - 0x58]
1412bdfa6: call 0x140be24b0
1412bdfab: nop 
1412bdfac: add rsp, 0x28
1412bdfb0: pop rsi
1412bdfb1: pop rbp
1412bdfb2: ret 
1412bdfb3: nop word ptr cs:[rax + rax]
1412bdfc0: mov qword ptr [rsp + 0x10], rdx
1412bdfc5: push rbp
1412bdfc6: push rsi
1412bdfc7: sub rsp, 0x28
1412bdfcb: lea rbp, [rdx + 0x80]
1412bdfd2: lea rcx, [rbp + 0x10]
1412bdfd6: call 0x1400c04b0
1412bdfdb: nop 
1412bdfdc: add rsp, 0x28
1412bdfe0: pop rsi
1412bdfe1: pop rbp
1412bdfe2: ret 
