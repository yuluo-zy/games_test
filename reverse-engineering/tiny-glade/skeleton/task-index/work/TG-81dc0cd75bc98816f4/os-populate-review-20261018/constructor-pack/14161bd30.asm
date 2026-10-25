14161bd30: push rbp
14161bd31: push r15
14161bd33: push r14
14161bd35: push r12
14161bd37: push rsi
14161bd38: push rdi
14161bd39: push rbx
14161bd3a: sub rsp, 0x110
14161bd41: lea rbp, [rsp + 0x80]
14161bd49: mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
14161bd54: mov rbx, r8
14161bd57: mov rdi, rdx
14161bd5a: mov rsi, rcx
14161bd5d: test r8, r8
14161bd60: je 0x14161bdf5
14161bd66: mov r14, r9
14161bd69: mov rax, r9
14161bd6c: and rax, 7
14161bd70: mov qword ptr [rbp - 0x40], rax
14161bd74: jne 0x14161bf63
14161bd7a: mov rax, rbx
14161bd7d: and rax, 3
14161bd81: mov qword ptr [rbp - 0x40], rax
14161bd85: jne 0x14161bf74
14161bd8b: cmp r14, 0x12c10
14161bd92: jb 0x14161bdf8
14161bd94: lea rax, [rip + 0x1482cb5]
14161bd9b: mov qword ptr [rbp - 0x40], rax
14161bd9f: mov qword ptr [rbp - 0x38], 0x5c
14161bda7: lea rax, [rbp - 0x40]
14161bdab: mov qword ptr [rbp + 0x60], rax
14161bdaf: lea rax, [rip - 0x1511f26]
14161bdb6: mov qword ptr [rbp + 0x68], rax
14161bdba: lea rax, [rip + 0x1663a07]
14161bdc1: mov qword ptr [rbp + 0x10], rax
14161bdc5: mov qword ptr [rbp + 0x18], 1
14161bdcd: mov qword ptr [rbp + 0x30], 0
14161bdd5: lea rax, [rbp + 0x60]
14161bdd9: mov qword ptr [rbp + 0x20], rax
14161bddd: mov qword ptr [rbp + 0x28], 1
14161bde5: lea rdx, [rip + 0x16639ec]
14161bdec: lea rcx, [rbp + 0x10]
14161bdf0: call 0x1428d9390
14161bdf5: xor r14d, r14d
14161bdf8: lea r15, [rbp + 0x10]
14161bdfc: mov edx, 0x12c08
14161be01: mov rcx, r15
14161be04: mov r8d, 0x22
14161be0a: mov r9b, 4
14161be0d: call 0x140bd4de0
14161be12: lea r12, [rbp + 0x60]
14161be16: mov r8d, 4
14161be1c: mov rcx, r12
14161be1f: mov rdx, r15
14161be22: call 0x140bd4f20
14161be27: mov qword ptr [rsp + 0x30], r14
14161be2c: mov qword ptr [rsp + 0x28], rbx
14161be31: mov qword ptr [rsp + 0x20], r12
14161be36: lea r8, [rip + 0x16642e3]
14161be3d: lea rcx, [rbp - 0x40]
14161be41: mov r9d, 0x16
14161be47: mov rdx, rdi
14161be4a: call 0x1416a2dc0
14161be4f: cmp dword ptr [rbp - 0x40], 0xb
14161be53: jne 0x14161bf9e
14161be59: mov rax, qword ptr [rbp - 0x18]
14161be5d: mov qword ptr [rbp + 0x80], rax
14161be64: movups xmm0, xmmword ptr [rbp - 0x38]
14161be68: movups xmm1, xmmword ptr [rbp - 0x28]
14161be6c: movaps xmmword ptr [rbp + 0x70], xmm1
14161be70: movaps xmmword ptr [rbp + 0x60], xmm0
14161be74: lea rcx, [rbp + 0x60]
14161be78: mov rdx, rdi
14161be7b: call 0x140bd4cd0
14161be80: test rax, rax
14161be83: je 0x14161bff0
14161be89: cmp rdx, qword ptr [rbp + 0x78]
14161be8d: jb 0x14161bee8
14161be8f: mov rcx, rdx
14161be92: and rcx, 7
14161be96: mov qword ptr [rbp - 0x40], rcx
14161be9a: jne 0x14161bf05
14161be9c: mov rcx, rax
14161be9f: and rcx, 3
14161bea3: mov qword ptr [rbp - 0x40], rcx
14161bea7: jne 0x14161bf34
14161bead: mov rcx, qword ptr [rbp + 0x80]
14161beb4: mov qword ptr [rsi + 0x30], rcx
14161beb8: movaps xmm0, xmmword ptr [rbp + 0x60]
14161bebc: movaps xmm1, xmmword ptr [rbp + 0x70]
14161bec0: movups xmmword ptr [rsi + 0x20], xmm1
14161bec4: movups xmmword ptr [rsi + 0x10], xmm0
14161bec8: shr rdx, 3
14161becc: mov qword ptr [rsi], rax
14161becf: mov qword ptr [rsi + 8], rdx
14161bed3: mov rax, rsi
14161bed6: add rsp, 0x110
14161bedd: pop rbx
14161bede: pop rdi
14161bedf: pop rsi
14161bee0: pop r12
14161bee2: pop r14
14161bee4: pop r15
14161bee6: pop rbp
14161bee7: ret 
14161bee8: lea rcx, [rip + 0x1664301]
14161beef: lea r8, [rip + 0x1664332]
14161bef6: mov edx, 0x37
14161befb: call 0x1428d9430
14161bf00: jmp 0x14161bffc
14161bf05: mov qword ptr [rbp + 0x10], 0
14161bf0d: lea rax, [rip + 0x1664334]
14161bf14: mov qword ptr [rsp + 0x20], rax
14161bf19: lea r8, [rip + 0x1664320]
14161bf20: lea rdx, [rbp - 0x40]
14161bf24: lea r9, [rbp + 0x10]
14161bf28: xor ecx, ecx
14161bf2a: call 0x1428d9596
14161bf2f: jmp 0x14161bffc
14161bf34: mov qword ptr [rbp + 0x10], 0
14161bf3c: lea rax, [rip + 0x166431d]
14161bf43: mov qword ptr [rsp + 0x20], rax
14161bf48: lea r8, [rip + 0x16642f1]
14161bf4f: lea rdx, [rbp - 0x40]
14161bf53: lea r9, [rbp + 0x10]
14161bf57: xor ecx, ecx
14161bf59: call 0x1428d9596
14161bf5e: jmp 0x14161bffc
14161bf63: mov qword ptr [rbp + 0x10], 0
14161bf6b: lea rax, [rip + 0x16adfd6]
14161bf72: jmp 0x14161bf83
14161bf74: mov qword ptr [rbp + 0x10], 0
14161bf7c: lea rax, [rip + 0x16adfdd]
14161bf83: mov qword ptr [rsp + 0x20], rax
14161bf88: lea r8, [rip + 0x16adfb1]
14161bf8f: lea rdx, [rbp - 0x40]
14161bf93: lea r9, [rbp + 0x10]
14161bf97: xor ecx, ecx
14161bf99: call 0x1428d9596
14161bf9e: movups xmm0, xmmword ptr [rbp]
14161bfa2: movaps xmmword ptr [rbp + 0x50], xmm0
14161bfa6: movups xmm0, xmmword ptr [rbp - 0x40]
14161bfaa: movups xmm1, xmmword ptr [rbp - 0x30]
14161bfae: movups xmm2, xmmword ptr [rbp - 0x20]
14161bfb2: movups xmm3, xmmword ptr [rbp - 0x10]
14161bfb6: movaps xmmword ptr [rbp + 0x40], xmm3
14161bfba: movaps xmmword ptr [rbp + 0x30], xmm2
14161bfbe: movaps xmmword ptr [rbp + 0x20], xmm1
14161bfc2: movaps xmmword ptr [rbp + 0x10], xmm0
14161bfc6: lea rax, [rip + 0x16641db]
14161bfcd: mov qword ptr [rsp + 0x20], rax
14161bfd2: lea rcx, [rip + 0x1662b0f]
14161bfd9: lea r9, [rip + 0x1662ae8]
14161bfe0: lea r8, [rbp + 0x10]
14161bfe4: mov edx, 0x2b
14161bfe9: call 0x1428d9760
14161bfee: jmp 0x14161bffc
14161bff0: lea rcx, [rip + 0x16641e1]
14161bff7: call 0x1428d9310
14161bffc: ud2 
14161bffe: nop 
14161c000: mov qword ptr [rsp + 0x10], rdx
14161c005: push rbp
14161c006: push r15
14161c008: push r14
14161c00a: push r12
14161c00c: push rsi
14161c00d: push rdi
14161c00e: push rbx
14161c00f: sub rsp, 0x40
14161c013: lea rbp, [rdx + 0x80]
14161c01a: lea rcx, [rbp + 0x60]
14161c01e: call 0x140be24b0
14161c023: nop 
14161c024: add rsp, 0x40
14161c028: pop rbx
14161c029: pop rdi
14161c02a: pop rsi
14161c02b: pop r12
14161c02d: pop r14
14161c02f: pop r15
14161c031: pop rbp
14161c032: ret 
14161c033: nop word ptr cs:[rax + rax]
14161c040: mov qword ptr [rsp + 0x10], rdx
14161c045: push rbp
14161c046: push r15
14161c048: push r14
14161c04a: push r12
14161c04c: push rsi
14161c04d: push rdi
14161c04e: push rbx
14161c04f: sub rsp, 0x40
14161c053: lea rbp, [rdx + 0x80]
14161c05a: lea rcx, [rbp + 0x10]
14161c05e: call 0x1407cea70
14161c063: nop 
14161c064: add rsp, 0x40
14161c068: pop rbx
14161c069: pop rdi
14161c06a: pop rsi
14161c06b: pop r12
14161c06d: pop r14
14161c06f: pop r15
14161c071: pop rbp
14161c072: ret 
