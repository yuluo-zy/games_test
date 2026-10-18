1412c00d0: push rbp
1412c00d1: push rsi
1412c00d2: sub rsp, 0xd8
1412c00d9: lea rbp, [rsp + 0x80]
1412c00e1: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c00e9: mov rsi, rcx
1412c00ec: lea rcx, [rbp - 0x58]
1412c00f0: xor r8d, r8d
1412c00f3: call 0x141617f80
1412c00f8: mov qword ptr [rbp - 0x60], 0
1412c0100: movzx eax, byte ptr [rip + 0x2374b02]
1412c0107: mov ecx, 0x30
1412c010c: mov edx, 8
1412c0111: call 0x140613c10
1412c0116: test rax, rax
1412c0119: je 0x1412c01cf
1412c011f: movups xmm0, xmmword ptr [rbp - 0x60]
1412c0123: movups xmm1, xmmword ptr [rbp - 0x50]
1412c0127: movups xmm2, xmmword ptr [rbp - 0x40]
1412c012b: movups xmmword ptr [rax + 0x20], xmm2
1412c012f: movups xmmword ptr [rax + 0x10], xmm1
1412c0133: movups xmmword ptr [rax], xmm0
1412c0136: lea rcx, [rip + 0x17dce38]
1412c013d: mov qword ptr [rbp], rcx
1412c0141: mov qword ptr [rbp + 8], 0x2f
1412c0149: mov qword ptr [rbp + 0x10], rax
1412c014d: lea rax, [rip + 0x196c9a4]
1412c0154: mov qword ptr [rbp + 0x18], rax
1412c0158: mov qword ptr [rbp + 0x30], 0
1412c0160: mov byte ptr [rbp + 0x46], 5
1412c0164: mov qword ptr [rbp + 0x20], rcx
1412c0168: mov qword ptr [rbp + 0x28], 0x2f
1412c0170: movzx eax, byte ptr [rip + 0x2374a92]
1412c0177: mov ecx, 0x40
1412c017c: mov edx, 8
1412c0181: call 0x140613c10
1412c0186: test rax, rax
1412c0189: je 0x1412c01e0
1412c018b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c018f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c0193: movups xmm2, xmmword ptr [rbp + 0x30]
1412c0197: movups xmm3, xmmword ptr [rbp + 0x40]
1412c019b: movups xmmword ptr [rax + 0x30], xmm3
1412c019f: movups xmmword ptr [rax + 0x20], xmm2
1412c01a3: movups xmmword ptr [rax + 0x10], xmm1
1412c01a7: movups xmmword ptr [rax], xmm0
1412c01aa: movabs rdx, 0xaa4d4a026938e06
1412c01b4: mov rcx, rsi
1412c01b7: mov r8, rax
1412c01ba: call 0x140969bf0
1412c01bf: cmp rax, 1
1412c01c3: je 0x1412c01f1
1412c01c5: add rsp, 0xd8
1412c01cc: pop rsi
1412c01cd: pop rbp
1412c01ce: ret 
1412c01cf: mov ecx, 8
1412c01d4: mov edx, 0x30
1412c01d9: call 0x1428d9000
1412c01de: jmp 0x1412c01ef
1412c01e0: mov ecx, 8
1412c01e5: mov edx, 0x40
1412c01ea: call 0x1428d9000
1412c01ef: ud2 
1412c01f1: mov rax, rdx
1412c01f4: add rdx, 0x10
1412c01f8: mov rsi, rdx
1412c01fb: cmp qword ptr [rax + 0x18], 0x2f
1412c0200: jne 0x1412c027e
1412c0202: mov rdx, qword ptr [rsi]
1412c0205: lea rcx, [rip + 0x17dcd69]
1412c020c: mov r8d, 0x2f
1412c0212: call 0x142923bf0
1412c0217: test eax, eax
1412c0219: jne 0x1412c027e
1412c021b: lea rax, [rip + 0x196df46]
1412c0222: mov qword ptr [rbp - 0x30], rax
1412c0226: lea rax, [rip - 0xd8c98d]
1412c022d: mov qword ptr [rbp - 0x28], rax
1412c0231: mov rax, rbp
1412c0234: mov qword ptr [rbp - 0x20], rax
1412c0238: lea rax, [rip - 0x11b63af]
1412c023f: mov qword ptr [rbp - 0x18], rax
1412c0243: lea rax, [rip + 0x196d3de]
1412c024a: mov qword ptr [rbp + 0x10], rax
1412c024e: mov qword ptr [rbp + 0x18], 3
1412c0256: mov qword ptr [rbp + 0x30], 0
1412c025e: lea rax, [rbp - 0x30]
1412c0262: mov qword ptr [rbp + 0x20], rax
1412c0266: mov qword ptr [rbp + 0x28], 2
1412c026e: lea rdx, [rip + 0x196d3e3]
1412c0275: lea rcx, [rbp + 0x10]
1412c0279: call 0x1428d9390
1412c027e: lea rax, [rip + 0x196dee3]
1412c0285: mov qword ptr [rbp + 0x10], rax
1412c0289: lea rax, [rip - 0xd8c9f0]
1412c0290: mov qword ptr [rbp + 0x18], rax
1412c0294: mov qword ptr [rbp + 0x20], rsi
1412c0298: lea rax, [rip - 0x11b640f]
1412c029f: mov qword ptr [rbp + 0x28], rax
1412c02a3: mov rcx, rbp
1412c02a6: mov qword ptr [rbp + 0x30], rcx
1412c02aa: mov qword ptr [rbp + 0x38], rax
1412c02ae: lea rax, [rip + 0x196d2b3]
1412c02b5: mov qword ptr [rbp - 0x30], rax
1412c02b9: mov qword ptr [rbp - 0x28], 3
1412c02c1: mov qword ptr [rbp - 0x10], 0
1412c02c9: lea rax, [rbp + 0x10]
1412c02cd: mov qword ptr [rbp - 0x20], rax
1412c02d1: mov qword ptr [rbp - 0x18], 3
1412c02d9: lea rdx, [rip + 0x196d2f0]
1412c02e0: lea rcx, [rbp - 0x30]
1412c02e4: call 0x1428d9390
1412c02e9: int3 
1412c02ea: nop word ptr [rax + rax]
1412c02f0: mov qword ptr [rsp + 0x10], rdx
1412c02f5: push rbp
1412c02f6: push rsi
1412c02f7: sub rsp, 0x28
1412c02fb: lea rbp, [rdx + 0x80]
1412c0302: lea rcx, [rbp - 0x58]
1412c0306: call 0x140be24b0
1412c030b: nop 
1412c030c: add rsp, 0x28
1412c0310: pop rsi
1412c0311: pop rbp
1412c0312: ret 
1412c0313: nop word ptr cs:[rax + rax]
1412c0320: mov qword ptr [rsp + 0x10], rdx
1412c0325: push rbp
1412c0326: push rsi
1412c0327: sub rsp, 0x28
1412c032b: lea rbp, [rdx + 0x80]
1412c0332: lea rcx, [rbp + 0x10]
1412c0336: call 0x1400c04b0
1412c033b: nop 
1412c033c: add rsp, 0x28
1412c0340: pop rsi
1412c0341: pop rbp
1412c0342: ret 
