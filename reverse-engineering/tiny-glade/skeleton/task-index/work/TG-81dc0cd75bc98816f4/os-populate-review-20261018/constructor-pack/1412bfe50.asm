1412bfe50: push rbp
1412bfe51: push rsi
1412bfe52: sub rsp, 0xe8
1412bfe59: lea rbp, [rsp + 0x80]
1412bfe61: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412bfe69: mov rsi, rcx
1412bfe6c: lea rcx, [rbp - 0x58]
1412bfe70: xor r8d, r8d
1412bfe73: call 0x14161cd80
1412bfe78: movzx eax, byte ptr [rip + 0x2374d8a]
1412bfe7f: mov ecx, 0x38
1412bfe84: mov edx, 8
1412bfe89: call 0x140613c10
1412bfe8e: test rax, rax
1412bfe91: je 0x1412bff4f
1412bfe97: mov rcx, qword ptr [rbp - 0x28]
1412bfe9b: mov qword ptr [rax + 0x30], rcx
1412bfe9f: movups xmm0, xmmword ptr [rbp - 0x58]
1412bfea3: movups xmm1, xmmword ptr [rbp - 0x48]
1412bfea7: movups xmm2, xmmword ptr [rbp - 0x38]
1412bfeab: movups xmmword ptr [rax + 0x20], xmm2
1412bfeaf: movups xmmword ptr [rax + 0x10], xmm1
1412bfeb3: movups xmmword ptr [rax], xmm0
1412bfeb6: lea rcx, [rip + 0x17de09e]
1412bfebd: mov qword ptr [rbp + 0x10], rcx
1412bfec1: mov qword ptr [rbp + 0x18], 0x6c
1412bfec9: mov qword ptr [rbp + 0x20], rax
1412bfecd: lea rax, [rip + 0x196cbec]
1412bfed4: mov qword ptr [rbp + 0x28], rax
1412bfed8: mov qword ptr [rbp + 0x40], 0
1412bfee0: mov byte ptr [rbp + 0x56], 5
1412bfee4: mov qword ptr [rbp + 0x30], rcx
1412bfee8: mov qword ptr [rbp + 0x38], 0x6c
1412bfef0: movzx eax, byte ptr [rip + 0x2374d12]
1412bfef7: mov ecx, 0x40
1412bfefc: mov edx, 8
1412bff01: call 0x140613c10
1412bff06: test rax, rax
1412bff09: je 0x1412bff60
1412bff0b: movups xmm0, xmmword ptr [rbp + 0x20]
1412bff0f: movups xmm1, xmmword ptr [rbp + 0x30]
1412bff13: movups xmm2, xmmword ptr [rbp + 0x40]
1412bff17: movups xmm3, xmmword ptr [rbp + 0x50]
1412bff1b: movups xmmword ptr [rax + 0x30], xmm3
1412bff1f: movups xmmword ptr [rax + 0x20], xmm2
1412bff23: movups xmmword ptr [rax + 0x10], xmm1
1412bff27: movups xmmword ptr [rax], xmm0
1412bff2a: movabs rdx, 0x50787cf1536c7500
1412bff34: mov rcx, rsi
1412bff37: mov r8, rax
1412bff3a: call 0x140969bf0
1412bff3f: cmp rax, 1
1412bff43: je 0x1412bff71
1412bff45: add rsp, 0xe8
1412bff4c: pop rsi
1412bff4d: pop rbp
1412bff4e: ret 
1412bff4f: mov ecx, 8
1412bff54: mov edx, 0x38
1412bff59: call 0x1428d9000
1412bff5e: jmp 0x1412bff6f
1412bff60: mov ecx, 8
1412bff65: mov edx, 0x40
1412bff6a: call 0x1428d9000
1412bff6f: ud2 
1412bff71: mov rax, rdx
1412bff74: add rdx, 0x10
1412bff78: mov rsi, rdx
1412bff7b: cmp qword ptr [rax + 0x18], 0x6c
1412bff80: jne 0x1412bffff
1412bff82: mov rdx, qword ptr [rsi]
1412bff85: lea rcx, [rip + 0x17ddfcf]
1412bff8c: mov r8d, 0x6c
1412bff92: call 0x142923bf0
1412bff97: test eax, eax
1412bff99: jne 0x1412bffff
1412bff9b: lea rax, [rip + 0x196db7e]
1412bffa2: mov qword ptr [rbp - 0x20], rax
1412bffa6: lea rax, [rip - 0xd8c70d]
1412bffad: mov qword ptr [rbp - 0x18], rax
1412bffb1: lea rax, [rbp + 0x10]
1412bffb5: mov qword ptr [rbp - 0x10], rax
1412bffb9: lea rax, [rip - 0x11b6130]
1412bffc0: mov qword ptr [rbp - 8], rax
1412bffc4: lea rax, [rip + 0x196d65d]
1412bffcb: mov qword ptr [rbp + 0x20], rax
1412bffcf: mov qword ptr [rbp + 0x28], 3
1412bffd7: mov qword ptr [rbp + 0x40], 0
1412bffdf: lea rax, [rbp - 0x20]
1412bffe3: mov qword ptr [rbp + 0x30], rax
1412bffe7: mov qword ptr [rbp + 0x38], 2
1412bffef: lea rdx, [rip + 0x196d662]
1412bfff6: lea rcx, [rbp + 0x20]
1412bfffa: call 0x1428d9390
1412bffff: lea rax, [rip + 0x196db1a]
1412c0006: mov qword ptr [rbp + 0x20], rax
1412c000a: lea rax, [rip - 0xd8c771]
1412c0011: mov qword ptr [rbp + 0x28], rax
1412c0015: mov qword ptr [rbp + 0x30], rsi
1412c0019: lea rax, [rip - 0x11b6190]
1412c0020: mov qword ptr [rbp + 0x38], rax
1412c0024: lea rcx, [rbp + 0x10]
1412c0028: mov qword ptr [rbp + 0x40], rcx
1412c002c: mov qword ptr [rbp + 0x48], rax
1412c0030: lea rax, [rip + 0x196d531]
1412c0037: mov qword ptr [rbp - 0x20], rax
1412c003b: mov qword ptr [rbp - 0x18], 3
1412c0043: mov qword ptr [rbp], 0
1412c004b: lea rax, [rbp + 0x20]
1412c004f: mov qword ptr [rbp - 0x10], rax
1412c0053: mov qword ptr [rbp - 8], 3
1412c005b: lea rdx, [rip + 0x196d56e]
1412c0062: lea rcx, [rbp - 0x20]
1412c0066: call 0x1428d9390
1412c006b: int3 
1412c006c: nop dword ptr [rax]
1412c0070: mov qword ptr [rsp + 0x10], rdx
1412c0075: push rbp
1412c0076: push rsi
1412c0077: sub rsp, 0x28
1412c007b: lea rbp, [rdx + 0x80]
1412c0082: lea rcx, [rbp - 0x48]
1412c0086: call 0x140be24b0
1412c008b: nop 
1412c008c: add rsp, 0x28
1412c0090: pop rsi
1412c0091: pop rbp
1412c0092: ret 
1412c0093: nop word ptr cs:[rax + rax]
1412c00a0: mov qword ptr [rsp + 0x10], rdx
1412c00a5: push rbp
1412c00a6: push rsi
1412c00a7: sub rsp, 0x28
1412c00ab: lea rbp, [rdx + 0x80]
1412c00b2: lea rcx, [rbp + 0x20]
1412c00b6: call 0x1400c04b0
1412c00bb: nop 
1412c00bc: add rsp, 0x28
1412c00c0: pop rsi
1412c00c1: pop rbp
1412c00c2: ret 
