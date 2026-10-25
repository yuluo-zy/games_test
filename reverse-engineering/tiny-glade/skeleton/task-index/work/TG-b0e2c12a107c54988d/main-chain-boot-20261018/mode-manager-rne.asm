0x140a86b60: push r15
0x140a86b62: push r14
0x140a86b64: push r13
0x140a86b66: push r12
0x140a86b68: push rsi
0x140a86b69: push rdi
0x140a86b6a: push rbp
0x140a86b6b: push rbx
0x140a86b6c: sub rsp, 0x58
0x140a86b70: mov qword ptr [rsp + 0x48], r8
0x140a86b75: mov qword ptr [rsp + 0x50], rdx
0x140a86b7a: mov rsi, rcx
0x140a86b7d: mov rdi, qword ptr [r9]
0x140a86b80: lea rax, [rip + 0x205197a]
0x140a86b87: mov qword ptr [rsp + 0x28], rax
0x140a86b8c: mov qword ptr [rsp + 0x30], 0xf
0x140a86b95: lea rcx, [rsp + 0x28]
0x140a86b9a: call 0x1405bdda0
0x140a86b9f: cmp byte ptr [rdi + 0x18], 0
0x140a86ba3: je 0x140a86bb5
0x140a86ba5: cmp qword ptr [rdi + 0x38], rax
0x140a86ba9: jne 0x140a86bb5
0x140a86bab: cmp dword ptr [rdi + 0x28], 2
0x140a86baf: jne 0x140a875ae
0x140a86bb5: mov r12, qword ptr [rsp + 0x100]
0x140a86bbd: mov rcx, qword ptr [rsp + 0xf0]
0x140a86bc5: mov rax, qword ptr [rcx]
0x140a86bc8: mov r13, qword ptr [rcx + 8]
0x140a86bcc: mov rcx, qword ptr [rax]
0x140a86bcf: xor r10d, r10d
0x140a86bd2: mov rdx, rcx
0x140a86bd5: sub rdx, qword ptr [r13 + 0x18]
0x140a86bd9: cmovb rdx, r10
0x140a86bdd: sub rcx, qword ptr [r13 + 0x38]
0x140a86be1: cmovb rcx, r10
0x140a86be5: mov r11, qword ptr [r13 + 0x10]
0x140a86be9: mov r9, qword ptr [r13 + 0x30]
0x140a86bed: mov r8, r9
0x140a86bf0: sub r8, rcx
0x140a86bf3: cmovb r8, r10
0x140a86bf7: mov qword ptr [rsp + 0x20], rsi
0x140a86bfc: setbe bl
0x140a86bff: mov r15, r11
0x140a86c02: sub r15, rdx
0x140a86c05: cmovb r15, r10
0x140a86c09: mov rsi, qword ptr [r13 + 0x40]
0x140a86c0d: setbe r14b
0x140a86c11: mov r10, rsi
0x140a86c14: sub r10, r8
0x140a86c17: sub r10, r15
0x140a86c1a: mov rdi, qword ptr [r13 + 8]
0x140a86c1e: mov rbp, qword ptr [r13 + 0x28]
0x140a86c22: mov qword ptr [rax], r10
0x140a86c25: mov r13, qword ptr [rsp + 0xd0]
0x140a86c2d: test r14b, bl
0x140a86c30: mov r14, r12
0x140a86c33: mov rbx, qword ptr [rsp + 0x20]
0x140a86c38: jne 0x140a86cc1
0x140a86c3e: mov r10, rcx
0x140a86c41: shl r10, 4
0x140a86c45: add rbp, r10
0x140a86c48: shl r8, 4
0x140a86c4c: mov r10, rdx
0x140a86c4f: shl r10, 4
0x140a86c53: add rdi, r10
0x140a86c56: shl r15, 4
0x140a86c5a: xor r10d, r10d
0x140a86c5d: cmp r11, rdx
0x140a86c60: mov edx, 8
0x140a86c65: cmovb rdi, rdx
0x140a86c69: lea r11, [rdi + r15 - 0x10]
0x140a86c6e: cmovbe r11, r10
0x140a86c72: cmp r9, rcx
0x140a86c75: cmovb rbp, rdx
0x140a86c79: lea rdx, [rbp + r8 - 0x10]
0x140a86c7e: cmovbe rdx, r11
0x140a86c82: mov r8, qword ptr [rsp + 0xc8]
0x140a86c8a: add rdx, 8
0x140a86c8e: mov qword ptr [rax], rsi
0x140a86c91: mov rax, qword ptr [r14]
0x140a86c94: cmp r10, qword ptr [rax + 0x30]
0x140a86c98: setno cl
0x140a86c9b: mov qword ptr [rsp + 0x28], rbx
0x140a86ca0: mov qword ptr [rsp + 0x30], r13
0x140a86ca5: mov qword ptr [rsp + 0x38], rdx
0x140a86caa: mov qword ptr [rsp + 0x40], r8
0x140a86caf: lea r8, [rsp + 0x28]
0x140a86cb4: mov rdx, qword ptr [rsp + 0x108]
0x140a86cbc: call 0x140afa8a0
0x140a86cc1: mov rax, qword ptr [rsp + 0xf8]
0x140a86cc9: mov r12, qword ptr [rax]
0x140a86ccc: mov eax, dword ptr [r12 + 0x80]
0x140a86cd4: lea rcx, [rip + 0x2051c31]
0x140a86cdb: movsxd rax, dword ptr [rcx + rax*4]
0x140a86cdf: add rax, rcx
0x140a86ce2: jmp rax
0x140a86ce4: xorps xmm0, xmm0
0x140a86ce7: ucomiss xmm0, dword ptr [r12 + 0x84]
0x140a86cf0: jb 0x140a86dbb
0x140a86cf6: mov rax, qword ptr [r14]
0x140a86cf9: xor ecx, ecx
0x140a86cfb: cmp rcx, qword ptr [rax + 0x30]
0x140a86cff: jno 0x140a86d60
0x140a86d01: mov eax, dword ptr [rbx + 0x1c]
0x140a86d04: mov rcx, qword ptr [rbx + 0x10]
0x140a86d08: mov dword ptr [rcx], eax
0x140a86d0a: mov rax, qword ptr [rbx]
0x140a86d0d: mov dword ptr [rax], 0xd
0x140a86d13: mov eax, dword ptr [r13 + 0x1c]
0x140a86d17: mov rcx, qword ptr [r13 + 0x10]
0x140a86d1b: mov dword ptr [rcx], eax
0x140a86d1d: mov rbp, qword ptr [r13]
0x140a86d21: mov r15, qword ptr [rbp + 0x30]
0x140a86d25: mov rdi, qword ptr [rbp + 0x40]
0x140a86d29: cmp r15, qword ptr [rbp + 0x20]
0x140a86d2d: jne 0x140a86d3f
0x140a86d2f: lea rcx, [rbp + 0x20]
0x140a86d33: lea rdx, [rip + 0x21b340e]
0x140a86d3a: call 0x140562630
0x140a86d3f: lea rax, [rbp + 0x40]
0x140a86d43: mov rcx, qword ptr [rbp + 0x28]
0x140a86d47: add rbp, 0x30
0x140a86d4b: mov rdx, r15
0x140a86d4e: shl rdx, 4
0x140a86d52: mov qword ptr [rcx + rdx], rdi
0x140a86d56: mov dword ptr [rcx + rdx + 8], 0
0x140a86d5e: jmp 0x140a86db1
0x140a86d60: mov rdx, qword ptr [rsp + 0x108]
0x140a86d68: mov eax, dword ptr [rdx + 0x1c]
0x140a86d6b: mov rcx, qword ptr [rdx + 0x10]
0x140a86d6f: mov dword ptr [rcx], eax
0x140a86d71: mov rbp, qword ptr [rdx]
0x140a86d74: mov r15, qword ptr [rbp + 0x30]
0x140a86d78: mov rdi, qword ptr [rbp + 0x40]
0x140a86d7c: cmp r15, qword ptr [rbp + 0x20]
0x140a86d80: jne 0x140a86d92
0x140a86d82: lea rcx, [rbp + 0x20]
0x140a86d86: lea rdx, [rip + 0x21b33bb]
0x140a86d8d: call 0x140562c30
0x140a86d92: lea rax, [rbp + 0x40]
0x140a86d96: mov rcx, qword ptr [rbp + 0x28]
0x140a86d9a: add rbp, 0x30
0x140a86d9e: mov rdx, r15
0x140a86da1: shl rdx, 5
0x140a86da5: mov dword ptr [rcx + rdx], 4
0x140a86dac: mov qword ptr [rcx + rdx + 0x18], rdi
0x140a86db1: inc r15
0x140a86db4: mov qword ptr [rbp], r15
0x140a86db8: inc qword ptr [rax]
0x140a86dbb: mov eax, dword ptr [r12 + 0x88]
0x140a86dc3: lea rcx, [rip + 0x2051b56]
0x140a86dca: movsxd rax, dword ptr [rcx + rax*4]
0x140a86dce: add rax, rcx
0x140a86dd1: jmp rax
0x140a86dd3: xorps xmm0, xmm0
0x140a86dd6: ucomiss xmm0, dword ptr [r12 + 0x8c]
0x140a86ddf: jb 0x140a86eaa
0x140a86de5: mov rax, qword ptr [r14]
0x140a86de8: xor ecx, ecx
0x140a86dea: cmp rcx, qword ptr [rax + 0x30]
0x140a86dee: jno 0x140a86e4f
0x140a86df0: mov eax, dword ptr [rbx + 0x1c]
0x140a86df3: mov rcx, qword ptr [rbx + 0x10]
0x140a86df7: mov dword ptr [rcx], eax
0x140a86df9: mov rax, qword ptr [rbx]
0x140a86dfc: mov dword ptr [rax], 0xd
0x140a86e02: mov eax, dword ptr [r13 + 0x1c]
0x140a86e06: mov rcx, qword ptr [r13 + 0x10]
0x140a86e0a: mov dword ptr [rcx], eax
0x140a86e0c: mov r13, qword ptr [r13]
0x140a86e10: mov r15, qword ptr [r13 + 0x30]
0x140a86e14: mov rdi, qword ptr [r13 + 0x40]
0x140a86e18: cmp r15, qword ptr [r13 + 0x20]
0x140a86e1c: jne 0x140a86e2e
0x140a86e1e: lea rcx, [r13 + 0x20]
0x140a86e22: lea rdx, [rip + 0x21b331f]
0x140a86e29: call 0x140562630
0x140a86e2e: lea rax, [r13 + 0x40]
0x140a86e32: mov rcx, qword ptr [r13 + 0x28]
0x140a86e36: add r13, 0x30
0x140a86e3a: mov rdx, r15
0x140a86e3d: shl rdx, 4
0x140a86e41: mov qword ptr [rcx + rdx], rdi
0x140a86e45: mov dword ptr [rcx + rdx + 8], 0x1000100
0x140a86e4d: jmp 0x140a86ea0
0x140a86e4f: mov rdx, qword ptr [rsp + 0x108]
0x140a86e57: mov eax, dword ptr [rdx + 0x1c]
0x140a86e5a: mov rcx, qword ptr [rdx + 0x10]
0x140a86e5e: mov dword ptr [rcx], eax
0x140a86e60: mov r13, qword ptr [rdx]
0x140a86e63: mov r15, qword ptr [r13 + 0x30]
0x140a86e67: mov rdi, qword ptr [r13 + 0x40]
0x140a86e6b: cmp r15, qword ptr [r13 + 0x20]
0x140a86e6f: jne 0x140a86e81
0x140a86e71: lea rcx, [r13 + 0x20]
0x140a86e75: lea rdx, [rip + 0x21b32cc]
0x140a86e7c: call 0x140562c30
0x140a86e81: lea rax, [r13 + 0x40]
0x140a86e85: mov rcx, qword ptr [r13 + 0x28]
0x140a86e89: add r13, 0x30
0x140a86e8d: mov rdx, r15
0x140a86e90: shl rdx, 5
0x140a86e94: mov dword ptr [rcx + rdx], 4
0x140a86e9b: mov qword ptr [rcx + rdx + 0x18], rdi
0x140a86ea0: inc r15
0x140a86ea3: mov qword ptr [r13], r15
0x140a86ea7: inc qword ptr [rax]
0x140a86eaa: mov rax, qword ptr [rbx]
0x140a86ead: cmp dword ptr [rax], 0xd
0x140a86eb0: je 0x140a875ae
0x140a86eb6: mov rax, qword ptr [rsp + 0xd8]
0x140a86ebe: mov r9, qword ptr [rax]
0x140a86ec1: mov r10, qword ptr [rax + 8]
0x140a86ec5: mov rax, qword ptr [r9]
0x140a86ec8: xor esi, esi
0x140a86eca: mov rdx, rax
0x140a86ecd: sub rdx, qword ptr [r10 + 0x18]
0x140a86ed1: cmovb rdx, rsi
0x140a86ed5: sub rax, qword ptr [r10 + 0x38]
0x140a86ed9: cmovb rax, rsi
0x140a86edd: mov r11, qword ptr [r10 + 0x10]
0x140a86ee1: mov r8, qword ptr [r10 + 0x30]
0x140a86ee5: mov rcx, r8
0x140a86ee8: sub rcx, rax
0x140a86eeb: cmovb rcx, rsi
0x140a86eef: setbe bl
0x140a86ef2: mov r15, r11
0x140a86ef5: sub r15, rdx
0x140a86ef8: cmovb r15, rsi
0x140a86efc: mov rdi, qword ptr [r10 + 8]
0x140a86f00: mov r14, qword ptr [r10 + 0x40]
0x140a86f04: setbe bpl
0x140a86f08: lea r13, [rcx + r15]
0x140a86f0c: test bpl, bl
0x140a86f0f: cmove r13, rsi
0x140a86f13: sub r14, r13
0x140a86f16: mov r10, qword ptr [r10 + 0x28]
0x140a86f1a: mov qword ptr [r9], r14
0x140a86f1d: test bpl, bl
0x140a86f20: jne 0x140a86fdc
0x140a86f26: xor r9d, r9d
0x140a86f29: cmp r11, rdx
0x140a86f2c: lea rdx, [rdx + rdx*2]
0x140a86f30: lea rdx, [rdi + rdx*8]
0x140a86f34: mov r11d, 8
0x140a86f3a: cmovb rdx, r11
0x140a86f3e: lea rsi, [r15 + r15*2]
0x140a86f42: lea rdx, [rdx + rsi*8 - 0x18]
0x140a86f47: cmova r9, rdx
0x140a86f4b: cmp r8, rax
0x140a86f4e: lea rax, [rax + rax*2]
0x140a86f52: lea rax, [r10 + rax*8]
0x140a86f56: cmovb rax, r11
0x140a86f5a: lea rcx, [rcx + rcx*2]
0x140a86f5e: lea rdx, [rax + rcx*8 - 0x18]
0x140a86f63: cmovbe rdx, r9
0x140a86f67: mov r8, qword ptr [rsp + 0x20]
0x140a86f6c: mov eax, dword ptr [r8 + 0x1c]
0x140a86f70: mov rcx, qword ptr [r8 + 0x10]
0x140a86f74: mov dword ptr [rcx], eax
0x140a86f76: mov rax, qword ptr [r8]
0x140a86f79: mov dword ptr [rax], 0xd
0x140a86f7f: mov r8, qword ptr [rsp + 0x50]
0x140a86f84: mov eax, dword ptr [r8 + 0x1c]
0x140a86f88: mov rcx, qword ptr [r8 + 0x10]
0x140a86f8c: mov dword ptr [rcx], eax
0x140a86f8e: mov rax, qword ptr [r8]
0x140a86f91: mov ecx, dword ptr [rdx]
0x140a86f93: mov r8d, dword ptr [rdx + 0xc]
0x140a86f97: mov dword ptr [rsp + 0x30], r8d
0x140a86f9c: mov rdx, qword ptr [rdx + 4]
0x140a86fa0: mov qword ptr [rsp + 0x28], rdx
0x140a86fa5: lea edx, [rcx - 6]
0x140a86fa8: cmp edx, 0xf
0x140a86fab: setb dl
0x140a86fae: mov r8d, ecx
0x140a86fb1: add r8, -0xd
0x140a86fb5: cmp r8, 2
0x140a86fb9: setb r8b
0x140a86fbd: and r8b, dl
0x140a86fc0: cmp r8b, 1
0x140a86fc4: je 0x140a875bf
0x140a86fca: mov dword ptr [rax], ecx
0x140a86fcc: mov rcx, qword ptr [rsp + 0x28]
0x140a86fd1: mov qword ptr [rax + 4], rcx
0x140a86fd5: mov ecx, dword ptr [rsp + 0x30]
0x140a86fd9: mov dword ptr [rax + 0xc], ecx
0x140a86fdc: mov rax, qword ptr [rsp + 0xe0]
0x140a86fe4: mov r9, qword ptr [rax]
0x140a86fe7: mov r10, qword ptr [rax + 8]
0x140a86feb: mov rax, qword ptr [r9]
0x140a86fee: xor esi, esi
0x140a86ff0: mov rdx, rax
0x140a86ff3: sub rdx, qword ptr [r10 + 0x18]
0x140a86ff7: cmovb rdx, rsi
0x140a86ffb: sub rax, qword ptr [r10 + 0x38]
0x140a86fff: cmovb rax, rsi
0x140a87003: mov r11, qword ptr [r10 + 0x10]
0x140a87007: mov r8, qword ptr [r10 + 0x30]
0x140a8700b: mov rcx, r8
0x140a8700e: sub rcx, rax
0x140a87011: cmovb rcx, rsi
0x140a87015: setbe bl
0x140a87018: mov r15, r11
0x140a8701b: sub r15, rdx
0x140a8701e: cmovb r15, rsi
0x140a87022: mov rdi, qword ptr [r10 + 8]
0x140a87026: mov r13, qword ptr [r10 + 0x40]
0x140a8702a: setbe bpl
0x140a8702e: lea r14, [rcx + r15]
0x140a87032: test bpl, bl
0x140a87035: cmove r14, rsi
0x140a87039: sub r13, r14
0x140a8703c: mov r10, qword ptr [r10 + 0x28]
0x140a87040: mov qword ptr [r9], r13
0x140a87043: test bpl, bl
0x140a87046: jne 0x140a870c6
0x140a87048: xor r9d, r9d
0x140a8704b: cmp r11, rdx
0x140a8704e: lea rdx, [rdx + rdx*2]
0x140a87052: lea rdx, [rdi + rdx*8]
0x140a87056: mov r11d, 8
0x140a8705c: cmovb rdx, r11
0x140a87060: lea rsi, [r15 + r15*2]
0x140a87064: lea rdx, [rdx + rsi*8 - 0x18]
0x140a87069: cmovbe rdx, r9
0x140a8706d: cmp r8, rax
0x140a87070: lea rax, [rax + rax*2]
0x140a87074: lea rax, [r10 + rax*8]
0x140a87078: cmovb rax, r11
0x140a8707c: lea rcx, [rcx + rcx*2]
0x140a87080: lea rax, [rax + rcx*8 - 0x18]
0x140a87085: cmovbe rax, rdx
0x140a87089: mov rcx, qword ptr [rsp + 0x100]
0x140a87091: mov rcx, qword ptr [rcx]
0x140a87094: cmp r9, qword ptr [rcx + 0x30]
0x140a87098: setno cl
0x140a8709b: mov rdx, qword ptr [rsp + 0x20]
0x140a870a0: mov qword ptr [rsp + 0x28], rdx
0x140a870a5: mov qword ptr [rsp + 0x30], rax
0x140a870aa: mov rax, qword ptr [rsp + 0x48]
0x140a870af: mov qword ptr [rsp + 0x38], rax
0x140a870b4: lea r8, [rsp + 0x28]
0x140a870b9: mov rdx, qword ptr [rsp + 0x108]
0x140a870c1: call 0x140afa2c0
0x140a870c6: mov rcx, qword ptr [rsp + 0xe8]
0x140a870ce: mov rax, qword ptr [rcx]
0x140a870d1: mov r11, qword ptr [rcx + 8]
0x140a870d5: mov rcx, qword ptr [rax]
0x140a870d8: xor esi, esi
0x140a870da: mov rdx, rcx
0x140a870dd: sub rdx, qword ptr [r11 + 0x18]
0x140a870e1: cmovb rdx, rsi
0x140a870e5: sub rcx, qword ptr [r11 + 0x38]
0x140a870e9: cmovb rcx, rsi
0x140a870ed: mov r10, qword ptr [r11 + 8]
0x140a870f1: mov r9, qword ptr [r11 + 0x30]
0x140a870f5: mov r8, r9
0x140a870f8: sub r8, rcx
0x140a870fb: cmovb r8, rsi
0x140a870ff: mov r13, qword ptr [r11 + 0x10]
0x140a87103: setbe bl
0x140a87106: mov r15, r13
0x140a87109: sub r15, rdx
0x140a8710c: cmovb r15, rsi
0x140a87110: mov r14, qword ptr [r11 + 0x28]
0x140a87114: mov r11, qword ptr [r11 + 0x40]
0x140a87118: setbe sil
0x140a8711c: mov rbp, r11
0x140a8711f: sub rbp, r8
0x140a87122: sub rbp, r15
0x140a87125: mov qword ptr [rax], rbp
0x140a87128: test sil, bl
0x140a8712b: mov rdi, qword ptr [rsp + 0x20]
0x140a87130: mov rbx, qword ptr [rsp + 0x48]
0x140a87135: jne 0x140a871b4
0x140a87137: mov rsi, rcx
0x140a8713a: shl rsi, 4
0x140a8713e: add r14, rsi
0x140a87141: shl r8, 4
0x140a87145: mov rsi, rdx
0x140a87148: shl rsi, 4
0x140a8714c: add r10, rsi
0x140a8714f: shl r15, 4
0x140a87153: xor esi, esi
0x140a87155: cmp r13, rdx
0x140a87158: mov edx, 8
0x140a8715d: cmovb r10, rdx
0x140a87161: lea r10, [r10 + r15 - 0x10]
0x140a87166: cmovbe r10, rsi
0x140a8716a: cmp r9, rcx
0x140a8716d: cmovb r14, rdx
0x140a87171: lea rdx, [r14 + r8 - 0x10]
0x140a87176: cmovbe rdx, r10
0x140a8717a: add rdx, 8
0x140a8717e: mov qword ptr [rax], r11
0x140a87181: mov rax, qword ptr [rsp + 0x100]
0x140a87189: mov rax, qword ptr [rax]
0x140a8718c: cmp rsi, qword ptr [rax + 0x30]
0x140a87190: setno cl
0x140a87193: mov qword ptr [rsp + 0x28], rbx
0x140a87198: mov qword ptr [rsp + 0x30], rdx
0x140a8719d: mov qword ptr [rsp + 0x38], rdi
0x140a871a2: lea r8, [rsp + 0x28]
0x140a871a7: mov rdx, qword ptr [rsp + 0x108]
0x140a871af: call 0x140afa770
0x140a871b4: mov eax, dword ptr [r12 + 0x20]
0x140a871b9: lea rcx, [rip + 0x2051774]
0x140a871c0: movsxd rax, dword ptr [rcx + rax*4]
0x140a871c4: add rax, rcx
0x140a871c7: mov rsi, qword ptr [rsp + 0x100]
0x140a871cf: jmp rax
0x140a871d1: xorps xmm0, xmm0
0x140a871d4: ucomiss xmm0, dword ptr [r12 + 0x24]
0x140a871da: jb 0x140a871fb
0x140a871dc: mov rax, qword ptr [rsi]
0x140a871df: xor ecx, ecx
0x140a871e1: cmp rcx, qword ptr [rax + 0x30]
0x140a871e5: setno cl
0x140a871e8: mov rdx, qword ptr [rsp + 0x108]
0x140a871f0: mov r8, rbx
0x140a871f3: mov r9, rdi
0x140a871f6: call 0x140afab70
0x140a871fb: mov eax, dword ptr [r12 + 0x28]
0x140a87200: lea rcx, [rip + 0x2051741]
0x140a87207: movsxd rax, dword ptr [rcx + rax*4]
0x140a8720b: add rax, rcx
0x140a8720e: jmp rax
0x140a87210: xorps xmm0, xmm0
0x140a87213: ucomiss xmm0, dword ptr [r12 + 0x2c]
0x140a87219: jb 0x140a8723a
0x140a8721b: mov rax, qword ptr [rsi]
0x140a8721e: xor ecx, ecx
0x140a87220: cmp rcx, qword ptr [rax + 0x30]
0x140a87224: setno cl
0x140a87227: mov rdx, qword ptr [rsp + 0x108]
0x140a8722f: mov r8, rbx
0x140a87232: mov r9, rdi
0x140a87235: call 0x140afa3f0
0x140a8723a: mov eax, dword ptr [r12 + 0x30]
0x140a8723f: lea rcx, [rip + 0x2051716]
0x140a87246: movsxd rax, dword ptr [rcx + rax*4]
0x140a8724a: add rax, rcx
0x140a8724d: jmp rax
0x140a8724f: xorps xmm0, xmm0
0x140a87252: ucomiss xmm0, dword ptr [r12 + 0x34]
0x140a87258: jb 0x140a87279
0x140a8725a: mov rax, qword ptr [rsi]
0x140a8725d: xor ecx, ecx
0x140a8725f: cmp rcx, qword ptr [rax + 0x30]
0x140a87263: setno cl
0x140a87266: mov rdx, qword ptr [rsp + 0x108]
0x140a8726e: mov r8, rbx
0x140a87271: mov r9, rdi
0x140a87274: call 0x140afadb0
0x140a87279: mov eax, dword ptr [r12 + 0x38]
0x140a8727e: lea rcx, [rip + 0x20516eb]
0x140a87285: movsxd rax, dword ptr [rcx + rax*4]
0x140a87289: add rax, rcx
0x140a8728c: jmp rax
0x140a8728e: xorps xmm0, xmm0
0x140a87291: ucomiss xmm0, dword ptr [r12 + 0x3c]
0x140a87297: jb 0x140a872b8
0x140a87299: mov rax, qword ptr [rsi]
0x140a8729c: xor ecx, ecx
0x140a8729e: cmp rcx, qword ptr [rax + 0x30]
0x140a872a2: setno cl
0x140a872a5: mov rdx, qword ptr [rsp + 0x108]
0x140a872ad: mov r8, rbx
0x140a872b0: mov r9, rdi
0x140a872b3: call 0x140afac90
0x140a872b8: mov eax, dword ptr [r12 + 0x48]
0x140a872bd: lea rcx, [rip + 0x20516c0]
0x140a872c4: movsxd rax, dword ptr [rcx + rax*4]
0x140a872c8: add rax, rcx
0x140a872cb: jmp rax
0x140a872cd: xorps xmm0, xmm0
0x140a872d0: ucomiss xmm0, dword ptr [r12 + 0x4c]
0x140a872d6: jb 0x140a872f7
0x140a872d8: mov rax, qword ptr [rsi]
0x140a872db: xor ecx, ecx
0x140a872dd: cmp rcx, qword ptr [rax + 0x30]
0x140a872e1: setno cl
0x140a872e4: mov rdx, qword ptr [rsp + 0x108]
0x140a872ec: mov r8, rbx
0x140a872ef: mov r9, rdi
0x140a872f2: call 0x140af9f30
0x140a872f7: mov eax, dword ptr [r12 + 0x40]
0x140a872fc: lea rcx, [rip + 0x2051695]
0x140a87303: movsxd rax, dword ptr [rcx + rax*4]
0x140a87307: add rax, rcx
0x140a8730a: jmp rax
0x140a8730c: xorps xmm0, xmm0
0x140a8730f: ucomiss xmm0, dword ptr [r12 + 0x44]
0x140a87315: jb 0x140a87336
0x140a87317: mov rax, qword ptr [rsi]
0x140a8731a: xor ecx, ecx
0x140a8731c: cmp rcx, qword ptr [rax + 0x30]
0x140a87320: setno cl
0x140a87323: mov rdx, qword ptr [rsp + 0x108]
0x140a8732b: mov r8, rbx
0x140a8732e: mov r9, rdi
0x140a87331: call 0x140afa190
0x140a87336: mov eax, dword ptr [r12 + 0x50]
0x140a8733b: lea rcx, [rip + 0x205166a]
0x140a87342: movsxd rax, dword ptr [rcx + rax*4]
0x140a87346: add rax, rcx
0x140a87349: jmp rax
0x140a8734b: xorps xmm0, xmm0
0x140a8734e: ucomiss xmm0, dword ptr [r12 + 0x54]
0x140a87354: jb 0x140a87375
0x140a87356: mov rax, qword ptr [rsi]
0x140a87359: xor ecx, ecx
0x140a8735b: cmp rcx, qword ptr [rax + 0x30]
0x140a8735f: setno cl
0x140a87362: mov rdx, qword ptr [rsp + 0x108]
0x140a8736a: mov r8, rbx
0x140a8736d: mov r9, rdi
0x140a87370: call 0x140afa640
0x140a87375: mov eax, dword ptr [r12 + 0x58]
0x140a8737a: lea rcx, [rip + 0x205163f]
0x140a87381: movsxd rax, dword ptr [rcx + rax*4]
0x140a87385: add rax, rcx
0x140a87388: jmp rax
0x140a8738a: xorps xmm0, xmm0
0x140a8738d: ucomiss xmm0, dword ptr [r12 + 0x5c]
0x140a87393: jb 0x140a873b4
0x140a87395: mov rax, qword ptr [rsi]
0x140a87398: xor ecx, ecx
0x140a8739a: cmp rcx, qword ptr [rax + 0x30]
0x140a8739e: setno cl
0x140a873a1: mov rdx, qword ptr [rsp + 0x108]
0x140a873a9: mov r8, rbx
0x140a873ac: mov r9, rdi
0x140a873af: call 0x140afa510
0x140a873b4: mov eax, dword ptr [r12 + 0x60]
0x140a873b9: lea rcx, [rip + 0x2051614]
0x140a873c0: movsxd rax, dword ptr [rcx + rax*4]
0x140a873c4: add rax, rcx
0x140a873c7: jmp rax
0x140a873c9: xorps xmm0, xmm0
0x140a873cc: ucomiss xmm0, dword ptr [r12 + 0x64]
0x140a873d2: jb 0x140a873f3
0x140a873d4: mov rax, qword ptr [rsi]
0x140a873d7: xor ecx, ecx
0x140a873d9: cmp rcx, qword ptr [rax + 0x30]
0x140a873dd: setno cl
0x140a873e0: mov rdx, qword ptr [rsp + 0x108]
0x140a873e8: mov r8, rbx
0x140a873eb: mov r9, rdi
0x140a873ee: call 0x140afa060
0x140a873f3: mov eax, dword ptr [r12 + 0x68]
0x140a873f8: lea rcx, [rip + 0x20515e9]
0x140a873ff: movsxd rax, dword ptr [rcx + rax*4]
0x140a87403: add rax, rcx
0x140a87406: jmp rax
0x140a87408: xorps xmm0, xmm0
0x140a8740b: ucomiss xmm0, dword ptr [r12 + 0x6c]
0x140a87411: jb 0x140a87432
0x140a87413: mov rax, qword ptr [rsi]
0x140a87416: xor ecx, ecx
0x140a87418: cmp rcx, qword ptr [rax + 0x30]
0x140a8741c: setno cl
0x140a8741f: mov rdx, qword ptr [rsp + 0x108]
0x140a87427: mov r8, rdi
0x140a8742a: mov r9, rbx
0x140a8742d: call 0x140afa9f0
0x140a87432: mov eax, dword ptr [r12 + 0x70]
0x140a87437: lea rcx, [rip + 0x20515be]
0x140a8743e: movsxd rax, dword ptr [rcx + rax*4]
0x140a87442: add rax, rcx
0x140a87445: jmp rax
0x140a87447: xorps xmm0, xmm0
0x140a8744a: ucomiss xmm0, dword ptr [r12 + 0x74]
0x140a87450: jb 0x140a874d8
0x140a87456: mov rax, qword ptr [rsp + 0x110]
0x140a8745e: mov rcx, qword ptr [rax]
0x140a87461: mov rdx, qword ptr [rax + 0x10]
0x140a87465: movzx r8d, byte ptr [rcx + 0xa]
0x140a8746a: mov eax, dword ptr [rax + 0x1c]
0x140a8746d: mov dword ptr [rdx], eax
0x140a8746f: xor r8b, 1
0x140a87473: mov byte ptr [rcx + 0xa], r8b
0x140a87477: mov rdx, qword ptr [rsp + 0x108]
0x140a8747f: mov eax, dword ptr [rdx + 0x1c]
0x140a87482: mov rcx, qword ptr [rdx + 0x10]
0x140a87486: mov dword ptr [rcx], eax
0x140a87488: mov rdi, qword ptr [rdx]
0x140a8748b: mov rbx, qword ptr [rdi + 0x30]
0x140a8748f: mov r14, qword ptr [rdi + 0x40]
0x140a87493: cmp rbx, qword ptr [rdi + 0x20]
0x140a87497: jne 0x140a874a9
0x140a87499: lea rcx, [rdi + 0x20]
0x140a8749d: lea rdx, [rip + 0x21b2ca4]
0x140a874a4: call 0x140562c30
0x140a874a9: mov rax, qword ptr [rdi + 0x28]
0x140a874ad: mov rcx, rbx
0x140a874b0: shl rcx, 5
0x140a874b4: mov dword ptr [rax + rcx], 0xe
0x140a874bb: mov qword ptr [rax + rcx + 0x18], r14
0x140a874c0: inc rbx
0x140a874c3: mov qword ptr [rdi + 0x30], rbx
0x140a874c7: inc qword ptr [rdi + 0x40]
0x140a874cb: mov rdi, qword ptr [rsp + 0x20]
0x140a874d0: mov rsi, qword ptr [rsp + 0x100]
0x140a874d8: cmp dword ptr [r12 + 0xd0], 2
0x140a874e1: ja 0x140a875ae
0x140a874e7: mov rax, qword ptr [rsi]
0x140a874ea: xor ecx, ecx
0x140a874ec: cmp rcx, qword ptr [rax + 0x30]
0x140a874f0: jno 0x140a87554
0x140a874f2: mov rax, qword ptr [rsp + 0xc0]
0x140a874fa: mov ecx, dword ptr [rdi + 0x1c]
0x140a874fd: mov rdx, qword ptr [rdi + 0x10]
0x140a87501: mov dword ptr [rdx], ecx
0x140a87503: mov rcx, qword ptr [rdi]
0x140a87506: mov dword ptr [rcx], 0
0x140a8750c: mov ecx, dword ptr [rax + 0x1c]
0x140a8750f: mov rdx, qword ptr [rax + 0x10]
0x140a87513: mov dword ptr [rdx], ecx
0x140a87515: mov rsi, qword ptr [rax]
0x140a87518: mov rdi, qword ptr [rsi + 0x30]
0x140a8751c: mov rbx, qword ptr [rsi + 0x40]
0x140a87520: cmp rdi, qword ptr [rsi + 0x20]
0x140a87524: jne 0x140a87536
0x140a87526: lea rcx, [rsi + 0x20]
0x140a8752a: lea rdx, [rip + 0x21b2c17]
0x140a87531: call 0x140562630
0x140a87536: lea rax, [rsi + 0x40]
0x140a8753a: mov rcx, qword ptr [rsi + 0x28]
0x140a8753e: add rsi, 0x30
0x140a87542: mov rdx, rdi
0x140a87545: shl rdx, 4
0x140a87549: mov qword ptr [rcx + rdx], rbx
0x140a8754d: mov byte ptr [rcx + rdx + 8], 0
0x140a87552: jmp 0x140a875a5
0x140a87554: mov rdx, qword ptr [rsp + 0x108]
0x140a8755c: mov eax, dword ptr [rdx + 0x1c]
0x140a8755f: mov rcx, qword ptr [rdx + 0x10]
0x140a87563: mov dword ptr [rcx], eax
0x140a87565: mov rsi, qword ptr [rdx]
0x140a87568: mov rdi, qword ptr [rsi + 0x30]
0x140a8756c: mov rbx, qword ptr [rsi + 0x40]
0x140a87570: cmp rdi, qword ptr [rsi + 0x20]
0x140a87574: jne 0x140a87586
0x140a87576: lea rcx, [rsi + 0x20]
0x140a8757a: lea rdx, [rip + 0x21b2bc7]
0x140a87581: call 0x140562c30
0x140a87586: lea rax, [rsi + 0x40]
0x140a8758a: mov rcx, qword ptr [rsi + 0x28]
0x140a8758e: add rsi, 0x30
0x140a87592: mov rdx, rdi
0x140a87595: shl rdx, 5
0x140a87599: mov dword ptr [rcx + rdx], 4
0x140a875a0: mov qword ptr [rcx + rdx + 0x18], rbx
0x140a875a5: inc rdi
0x140a875a8: mov qword ptr [rsi], rdi
0x140a875ab: inc qword ptr [rax]
0x140a875ae: add rsp, 0x58
0x140a875b2: pop rbx
0x140a875b3: pop rbp
0x140a875b4: pop rdi
0x140a875b5: pop rsi
0x140a875b6: pop r12
0x140a875b8: pop r13
0x140a875ba: pop r14
0x140a875bc: pop r15
0x140a875be: ret 
0x140a875bf: lea rcx, [rip + 0x204faca]
0x140a875c6: lea r8, [rip + 0x204fb23]
0x140a875cd: mov edx, 0x28
0x140a875d2: call 0x1428d9430
0x140a875d7: int3 
