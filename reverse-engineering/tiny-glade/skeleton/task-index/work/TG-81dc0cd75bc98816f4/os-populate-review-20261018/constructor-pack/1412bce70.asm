1412bce70: push rbp
1412bce71: push rsi
1412bce72: sub rsp, 0xd8
1412bce79: lea rbp, [rsp + 0x80]
1412bce81: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bce89: mov rsi, rcx
1412bce8c: lea rcx, [rbp - 0x58]
1412bce90: xor r8d, r8d
1412bce93: call 0x141610b60
1412bce98: movzx eax, byte ptr [rip + 0x2377d6a]
1412bce9f: mov ecx, 0x28
1412bcea4: mov edx, 8
1412bcea9: call 0x140613c10
1412bceae: test rax, rax
1412bceb1: je 0x1412bcf67
1412bceb7: mov rcx, qword ptr [rbp - 0x38]
1412bcebb: mov qword ptr [rax + 0x20], rcx
1412bcebf: movups xmm0, xmmword ptr [rbp - 0x58]
1412bcec3: movups xmm1, xmmword ptr [rbp - 0x48]
1412bcec7: movups xmmword ptr [rax + 0x10], xmm1
1412bcecb: movups xmmword ptr [rax], xmm0
1412bcece: lea rcx, [rip + 0x17e0c87]
1412bced5: mov qword ptr [rbp], rcx
1412bced9: mov qword ptr [rbp + 8], 0x4c
1412bcee1: mov qword ptr [rbp + 0x10], rax
1412bcee5: lea rax, [rip + 0x196f7ac]
1412bceec: mov qword ptr [rbp + 0x18], rax
1412bcef0: mov qword ptr [rbp + 0x30], 0
1412bcef8: mov byte ptr [rbp + 0x46], 5
1412bcefc: mov qword ptr [rbp + 0x20], rcx
1412bcf00: mov qword ptr [rbp + 0x28], 0x4c
1412bcf08: movzx eax, byte ptr [rip + 0x2377cfa]
1412bcf0f: mov ecx, 0x40
1412bcf14: mov edx, 8
1412bcf19: call 0x140613c10
1412bcf1e: test rax, rax
1412bcf21: je 0x1412bcf78
1412bcf23: movups xmm0, xmmword ptr [rbp + 0x10]
1412bcf27: movups xmm1, xmmword ptr [rbp + 0x20]
1412bcf2b: movups xmm2, xmmword ptr [rbp + 0x30]
1412bcf2f: movups xmm3, xmmword ptr [rbp + 0x40]
1412bcf33: movups xmmword ptr [rax + 0x30], xmm3
1412bcf37: movups xmmword ptr [rax + 0x20], xmm2
1412bcf3b: movups xmmword ptr [rax + 0x10], xmm1
1412bcf3f: movups xmmword ptr [rax], xmm0
1412bcf42: movabs rdx, 0x87df9c07cbe7e324
1412bcf4c: mov rcx, rsi
1412bcf4f: mov r8, rax
1412bcf52: call 0x140969bf0
1412bcf57: cmp rax, 1
1412bcf5b: je 0x1412bcf89
1412bcf5d: add rsp, 0xd8
1412bcf64: pop rsi
1412bcf65: pop rbp
1412bcf66: ret 
1412bcf67: mov ecx, 8
1412bcf6c: mov edx, 0x28
1412bcf71: call 0x1428d9000
1412bcf76: jmp 0x1412bcf87
1412bcf78: mov ecx, 8
1412bcf7d: mov edx, 0x40
1412bcf82: call 0x1428d9000
1412bcf87: ud2 
1412bcf89: mov rax, rdx
1412bcf8c: add rdx, 0x10
1412bcf90: mov rsi, rdx
1412bcf93: cmp qword ptr [rax + 0x18], 0x4c
1412bcf98: jne 0x1412bd016
1412bcf9a: mov rdx, qword ptr [rsi]
1412bcf9d: lea rcx, [rip + 0x17e0bb8]
1412bcfa4: mov r8d, 0x4c
1412bcfaa: call 0x142923bf0
1412bcfaf: test eax, eax
1412bcfb1: jne 0x1412bd016
1412bcfb3: lea rax, [rip + 0x19710de]
1412bcfba: mov qword ptr [rbp - 0x30], rax
1412bcfbe: lea rax, [rip - 0xd89725]
1412bcfc5: mov qword ptr [rbp - 0x28], rax
1412bcfc9: mov rax, rbp
1412bcfcc: mov qword ptr [rbp - 0x20], rax
1412bcfd0: lea rax, [rip - 0x11b3147]
1412bcfd7: mov qword ptr [rbp - 0x18], rax
1412bcfdb: lea rax, [rip + 0x1970646]
1412bcfe2: mov qword ptr [rbp + 0x10], rax
1412bcfe6: mov qword ptr [rbp + 0x18], 3
1412bcfee: mov qword ptr [rbp + 0x30], 0
1412bcff6: lea rax, [rbp - 0x30]
1412bcffa: mov qword ptr [rbp + 0x20], rax
1412bcffe: mov qword ptr [rbp + 0x28], 2
1412bd006: lea rdx, [rip + 0x197064b]
1412bd00d: lea rcx, [rbp + 0x10]
1412bd011: call 0x1428d9390
1412bd016: lea rax, [rip + 0x197107b]
1412bd01d: mov qword ptr [rbp + 0x10], rax
1412bd021: lea rax, [rip - 0xd89788]
1412bd028: mov qword ptr [rbp + 0x18], rax
1412bd02c: mov qword ptr [rbp + 0x20], rsi
1412bd030: lea rax, [rip - 0x11b31a7]
1412bd037: mov qword ptr [rbp + 0x28], rax
1412bd03b: mov rcx, rbp
1412bd03e: mov qword ptr [rbp + 0x30], rcx
1412bd042: mov qword ptr [rbp + 0x38], rax
1412bd046: lea rax, [rip + 0x197051b]
1412bd04d: mov qword ptr [rbp - 0x30], rax
1412bd051: mov qword ptr [rbp - 0x28], 3
1412bd059: mov qword ptr [rbp - 0x10], 0
1412bd061: lea rax, [rbp + 0x10]
1412bd065: mov qword ptr [rbp - 0x20], rax
1412bd069: mov qword ptr [rbp - 0x18], 3
1412bd071: lea rdx, [rip + 0x1970558]
1412bd078: lea rcx, [rbp - 0x30]
1412bd07c: call 0x1428d9390
1412bd081: int3 
1412bd082: nop word ptr cs:[rax + rax]
1412bd090: mov qword ptr [rsp + 0x10], rdx
1412bd095: push rbp
1412bd096: push rsi
1412bd097: sub rsp, 0x28
1412bd09b: lea rbp, [rdx + 0x80]
1412bd0a2: lea rcx, [rbp - 0x58]
1412bd0a6: call 0x140be24b0
1412bd0ab: nop 
1412bd0ac: add rsp, 0x28
1412bd0b0: pop rsi
1412bd0b1: pop rbp
1412bd0b2: ret 
1412bd0b3: nop word ptr cs:[rax + rax]
1412bd0c0: mov qword ptr [rsp + 0x10], rdx
1412bd0c5: push rbp
1412bd0c6: push rsi
1412bd0c7: sub rsp, 0x28
1412bd0cb: lea rbp, [rdx + 0x80]
1412bd0d2: lea rcx, [rbp + 0x10]
1412bd0d6: call 0x1400c04b0
1412bd0db: nop 
1412bd0dc: add rsp, 0x28
1412bd0e0: pop rsi
1412bd0e1: pop rbp
1412bd0e2: ret 
