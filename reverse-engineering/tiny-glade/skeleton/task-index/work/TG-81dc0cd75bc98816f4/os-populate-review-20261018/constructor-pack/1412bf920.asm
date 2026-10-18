1412bf920: push rbp
1412bf921: push rsi
1412bf922: sub rsp, 0xd8
1412bf929: lea rbp, [rsp + 0x80]
1412bf931: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bf939: mov rsi, rcx
1412bf93c: lea rcx, [rbp - 0x58]
1412bf940: xor r8d, r8d
1412bf943: call 0x141615f50
1412bf948: mov qword ptr [rbp - 0x60], 0
1412bf950: movzx eax, byte ptr [rip + 0x23752b2]
1412bf957: mov ecx, 0x30
1412bf95c: mov edx, 8
1412bf961: call 0x140613c10
1412bf966: test rax, rax
1412bf969: je 0x1412bfa1f
1412bf96f: movups xmm0, xmmword ptr [rbp - 0x60]
1412bf973: movups xmm1, xmmword ptr [rbp - 0x50]
1412bf977: movups xmm2, xmmword ptr [rbp - 0x40]
1412bf97b: movups xmmword ptr [rax + 0x20], xmm2
1412bf97f: movups xmmword ptr [rax + 0x10], xmm1
1412bf983: movups xmmword ptr [rax], xmm0
1412bf986: lea rcx, [rip + 0x17de0f7]
1412bf98d: mov qword ptr [rbp], rcx
1412bf991: mov qword ptr [rbp + 8], 0x3f
1412bf999: mov qword ptr [rbp + 0x10], rax
1412bf99d: lea rax, [rip + 0x196d0ac]
1412bf9a4: mov qword ptr [rbp + 0x18], rax
1412bf9a8: mov qword ptr [rbp + 0x30], 0
1412bf9b0: mov byte ptr [rbp + 0x46], 5
1412bf9b4: mov qword ptr [rbp + 0x20], rcx
1412bf9b8: mov qword ptr [rbp + 0x28], 0x3f
1412bf9c0: movzx eax, byte ptr [rip + 0x2375242]
1412bf9c7: mov ecx, 0x40
1412bf9cc: mov edx, 8
1412bf9d1: call 0x140613c10
1412bf9d6: test rax, rax
1412bf9d9: je 0x1412bfa30
1412bf9db: movups xmm0, xmmword ptr [rbp + 0x10]
1412bf9df: movups xmm1, xmmword ptr [rbp + 0x20]
1412bf9e3: movups xmm2, xmmword ptr [rbp + 0x30]
1412bf9e7: movups xmm3, xmmword ptr [rbp + 0x40]
1412bf9eb: movups xmmword ptr [rax + 0x30], xmm3
1412bf9ef: movups xmmword ptr [rax + 0x20], xmm2
1412bf9f3: movups xmmword ptr [rax + 0x10], xmm1
1412bf9f7: movups xmmword ptr [rax], xmm0
1412bf9fa: movabs rdx, 0x1d6de5e69ac50d2a
1412bfa04: mov rcx, rsi
1412bfa07: mov r8, rax
1412bfa0a: call 0x140969bf0
1412bfa0f: cmp rax, 1
1412bfa13: je 0x1412bfa41
1412bfa15: add rsp, 0xd8
1412bfa1c: pop rsi
1412bfa1d: pop rbp
1412bfa1e: ret 
1412bfa1f: mov ecx, 8
1412bfa24: mov edx, 0x30
1412bfa29: call 0x1428d9000
1412bfa2e: jmp 0x1412bfa3f
1412bfa30: mov ecx, 8
1412bfa35: mov edx, 0x40
1412bfa3a: call 0x1428d9000
1412bfa3f: ud2 
1412bfa41: mov rax, rdx
1412bfa44: add rdx, 0x10
1412bfa48: mov rsi, rdx
1412bfa4b: cmp qword ptr [rax + 0x18], 0x3f
1412bfa50: jne 0x1412bface
1412bfa52: mov rdx, qword ptr [rsi]
1412bfa55: lea rcx, [rip + 0x17de028]
1412bfa5c: mov r8d, 0x3f
1412bfa62: call 0x142923bf0
1412bfa67: test eax, eax
1412bfa69: jne 0x1412bface
1412bfa6b: lea rax, [rip + 0x196e22e]
1412bfa72: mov qword ptr [rbp - 0x30], rax
1412bfa76: lea rax, [rip - 0xd8c1dd]
1412bfa7d: mov qword ptr [rbp - 0x28], rax
1412bfa81: mov rax, rbp
1412bfa84: mov qword ptr [rbp - 0x20], rax
1412bfa88: lea rax, [rip - 0x11b5bff]
1412bfa8f: mov qword ptr [rbp - 0x18], rax
1412bfa93: lea rax, [rip + 0x196db8e]
1412bfa9a: mov qword ptr [rbp + 0x10], rax
1412bfa9e: mov qword ptr [rbp + 0x18], 3
1412bfaa6: mov qword ptr [rbp + 0x30], 0
1412bfaae: lea rax, [rbp - 0x30]
1412bfab2: mov qword ptr [rbp + 0x20], rax
1412bfab6: mov qword ptr [rbp + 0x28], 2
1412bfabe: lea rdx, [rip + 0x196db93]
1412bfac5: lea rcx, [rbp + 0x10]
1412bfac9: call 0x1428d9390
1412bface: lea rax, [rip + 0x196e1cb]
1412bfad5: mov qword ptr [rbp + 0x10], rax
1412bfad9: lea rax, [rip - 0xd8c240]
1412bfae0: mov qword ptr [rbp + 0x18], rax
1412bfae4: mov qword ptr [rbp + 0x20], rsi
1412bfae8: lea rax, [rip - 0x11b5c5f]
1412bfaef: mov qword ptr [rbp + 0x28], rax
1412bfaf3: mov rcx, rbp
1412bfaf6: mov qword ptr [rbp + 0x30], rcx
1412bfafa: mov qword ptr [rbp + 0x38], rax
1412bfafe: lea rax, [rip + 0x196da63]
1412bfb05: mov qword ptr [rbp - 0x30], rax
1412bfb09: mov qword ptr [rbp - 0x28], 3
1412bfb11: mov qword ptr [rbp - 0x10], 0
1412bfb19: lea rax, [rbp + 0x10]
1412bfb1d: mov qword ptr [rbp - 0x20], rax
1412bfb21: mov qword ptr [rbp - 0x18], 3
1412bfb29: lea rdx, [rip + 0x196daa0]
1412bfb30: lea rcx, [rbp - 0x30]
1412bfb34: call 0x1428d9390
1412bfb39: int3 
1412bfb3a: nop word ptr [rax + rax]
1412bfb40: mov qword ptr [rsp + 0x10], rdx
1412bfb45: push rbp
1412bfb46: push rsi
1412bfb47: sub rsp, 0x28
1412bfb4b: lea rbp, [rdx + 0x80]
1412bfb52: lea rcx, [rbp - 0x58]
1412bfb56: call 0x140be24b0
1412bfb5b: nop 
1412bfb5c: add rsp, 0x28
1412bfb60: pop rsi
1412bfb61: pop rbp
1412bfb62: ret 
1412bfb63: nop word ptr cs:[rax + rax]
1412bfb70: mov qword ptr [rsp + 0x10], rdx
1412bfb75: push rbp
1412bfb76: push rsi
1412bfb77: sub rsp, 0x28
1412bfb7b: lea rbp, [rdx + 0x80]
1412bfb82: lea rcx, [rbp + 0x10]
1412bfb86: call 0x1400c04b0
1412bfb8b: nop 
1412bfb8c: add rsp, 0x28
1412bfb90: pop rsi
1412bfb91: pop rbp
1412bfb92: ret 
