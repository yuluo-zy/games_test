1412be9f0: push rbp
1412be9f1: push rsi
1412be9f2: sub rsp, 0xe8
1412be9f9: lea rbp, [rsp + 0x80]
1412bea01: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412bea09: mov rsi, rcx
1412bea0c: lea rcx, [rbp - 0x58]
1412bea10: xor r8d, r8d
1412bea13: call 0x141620320
1412bea18: movzx eax, byte ptr [rip + 0x23761ea]
1412bea1f: mov ecx, 0x38
1412bea24: mov edx, 8
1412bea29: call 0x140613c10
1412bea2e: test rax, rax
1412bea31: je 0x1412beaef
1412bea37: mov rcx, qword ptr [rbp - 0x28]
1412bea3b: mov qword ptr [rax + 0x30], rcx
1412bea3f: movups xmm0, xmmword ptr [rbp - 0x58]
1412bea43: movups xmm1, xmmword ptr [rbp - 0x48]
1412bea47: movups xmm2, xmmword ptr [rbp - 0x38]
1412bea4b: movups xmmword ptr [rax + 0x20], xmm2
1412bea4f: movups xmmword ptr [rax + 0x10], xmm1
1412bea53: movups xmmword ptr [rax], xmm0
1412bea56: lea rcx, [rip + 0x17dea64]
1412bea5d: mov qword ptr [rbp + 0x10], rcx
1412bea61: mov qword ptr [rbp + 0x18], 0x3f
1412bea69: mov qword ptr [rbp + 0x20], rax
1412bea6d: lea rax, [rip + 0x196de8c]
1412bea74: mov qword ptr [rbp + 0x28], rax
1412bea78: mov qword ptr [rbp + 0x40], 0
1412bea80: mov byte ptr [rbp + 0x56], 5
1412bea84: mov qword ptr [rbp + 0x30], rcx
1412bea88: mov qword ptr [rbp + 0x38], 0x3f
1412bea90: movzx eax, byte ptr [rip + 0x2376172]
1412bea97: mov ecx, 0x40
1412bea9c: mov edx, 8
1412beaa1: call 0x140613c10
1412beaa6: test rax, rax
1412beaa9: je 0x1412beb00
1412beaab: movups xmm0, xmmword ptr [rbp + 0x20]
1412beaaf: movups xmm1, xmmword ptr [rbp + 0x30]
1412beab3: movups xmm2, xmmword ptr [rbp + 0x40]
1412beab7: movups xmm3, xmmword ptr [rbp + 0x50]
1412beabb: movups xmmword ptr [rax + 0x30], xmm3
1412beabf: movups xmmword ptr [rax + 0x20], xmm2
1412beac3: movups xmmword ptr [rax + 0x10], xmm1
1412beac7: movups xmmword ptr [rax], xmm0
1412beaca: movabs rdx, 0x257fea44ceee898e
1412bead4: mov rcx, rsi
1412bead7: mov r8, rax
1412beada: call 0x140969bf0
1412beadf: cmp rax, 1
1412beae3: je 0x1412beb11
1412beae5: add rsp, 0xe8
1412beaec: pop rsi
1412beaed: pop rbp
1412beaee: ret 
1412beaef: mov ecx, 8
1412beaf4: mov edx, 0x38
1412beaf9: call 0x1428d9000
1412beafe: jmp 0x1412beb0f
1412beb00: mov ecx, 8
1412beb05: mov edx, 0x40
1412beb0a: call 0x1428d9000
1412beb0f: ud2 
1412beb11: mov rax, rdx
1412beb14: add rdx, 0x10
1412beb18: mov rsi, rdx
1412beb1b: cmp qword ptr [rax + 0x18], 0x3f
1412beb20: jne 0x1412beb9f
1412beb22: mov rdx, qword ptr [rsi]
1412beb25: lea rcx, [rip + 0x17de995]
1412beb2c: mov r8d, 0x3f
1412beb32: call 0x142923bf0
1412beb37: test eax, eax
1412beb39: jne 0x1412beb9f
1412beb3b: lea rax, [rip + 0x196f5e6]
1412beb42: mov qword ptr [rbp - 0x20], rax
1412beb46: lea rax, [rip - 0xd8b2ad]
1412beb4d: mov qword ptr [rbp - 0x18], rax
1412beb51: lea rax, [rbp + 0x10]
1412beb55: mov qword ptr [rbp - 0x10], rax
1412beb59: lea rax, [rip - 0x11b4cd0]
1412beb60: mov qword ptr [rbp - 8], rax
1412beb64: lea rax, [rip + 0x196eabd]
1412beb6b: mov qword ptr [rbp + 0x20], rax
1412beb6f: mov qword ptr [rbp + 0x28], 3
1412beb77: mov qword ptr [rbp + 0x40], 0
1412beb7f: lea rax, [rbp - 0x20]
1412beb83: mov qword ptr [rbp + 0x30], rax
1412beb87: mov qword ptr [rbp + 0x38], 2
1412beb8f: lea rdx, [rip + 0x196eac2]
1412beb96: lea rcx, [rbp + 0x20]
1412beb9a: call 0x1428d9390
1412beb9f: lea rax, [rip + 0x196f582]
1412beba6: mov qword ptr [rbp + 0x20], rax
1412bebaa: lea rax, [rip - 0xd8b311]
1412bebb1: mov qword ptr [rbp + 0x28], rax
1412bebb5: mov qword ptr [rbp + 0x30], rsi
1412bebb9: lea rax, [rip - 0x11b4d30]
1412bebc0: mov qword ptr [rbp + 0x38], rax
1412bebc4: lea rcx, [rbp + 0x10]
1412bebc8: mov qword ptr [rbp + 0x40], rcx
1412bebcc: mov qword ptr [rbp + 0x48], rax
1412bebd0: lea rax, [rip + 0x196e991]
1412bebd7: mov qword ptr [rbp - 0x20], rax
1412bebdb: mov qword ptr [rbp - 0x18], 3
1412bebe3: mov qword ptr [rbp], 0
1412bebeb: lea rax, [rbp + 0x20]
1412bebef: mov qword ptr [rbp - 0x10], rax
1412bebf3: mov qword ptr [rbp - 8], 3
1412bebfb: lea rdx, [rip + 0x196e9ce]
1412bec02: lea rcx, [rbp - 0x20]
1412bec06: call 0x1428d9390
1412bec0b: int3 
1412bec0c: nop dword ptr [rax]
1412bec10: mov qword ptr [rsp + 0x10], rdx
1412bec15: push rbp
1412bec16: push rsi
1412bec17: sub rsp, 0x28
1412bec1b: lea rbp, [rdx + 0x80]
1412bec22: lea rcx, [rbp - 0x48]
1412bec26: call 0x140be24b0
1412bec2b: nop 
1412bec2c: add rsp, 0x28
1412bec30: pop rsi
1412bec31: pop rbp
1412bec32: ret 
1412bec33: nop word ptr cs:[rax + rax]
1412bec40: mov qword ptr [rsp + 0x10], rdx
1412bec45: push rbp
1412bec46: push rsi
1412bec47: sub rsp, 0x28
1412bec4b: lea rbp, [rdx + 0x80]
1412bec52: lea rcx, [rbp + 0x20]
1412bec56: call 0x1400c04b0
1412bec5b: nop 
1412bec5c: add rsp, 0x28
1412bec60: pop rsi
1412bec61: pop rbp
1412bec62: ret 
