140b3ad60: push rbp
140b3ad61: push r15
140b3ad63: push r14
140b3ad65: push r12
140b3ad67: push rsi
140b3ad68: push rdi
140b3ad69: push rbx
140b3ad6a: sub rsp, 0x130
140b3ad71: lea rbp, [rsp + 0x80]
140b3ad79: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
140b3ad84: mov rsi, rcx
140b3ad87: movzx eax, byte ptr [rdx]
140b3ad8a: cmp eax, 3
140b3ad8d: mov dword ptr [rbp + 0x70], eax
140b3ad90: mov qword ptr [rbp + 0x68], rdx
140b3ad94: je 0x140b3ae8b
140b3ad9a: cmp eax, 5
140b3ad9d: jne 0x140b3af04
140b3ada3: mov r15, qword ptr [rdx + 8]
140b3ada7: mov r12, qword ptr [rdx + 0x10]
140b3adab: mov rdi, qword ptr [rdx + 0x18]
140b3adaf: xor eax, eax
140b3adb1: test r15, r15
140b3adb4: setne al
140b3adb7: mov rcx, rdi
140b3adba: cmove rcx, r15
140b3adbe: mov qword ptr [rbp - 0x20], rax
140b3adc2: mov qword ptr [rbp - 0x18], 0
140b3adca: mov qword ptr [rbp - 0x10], r15
140b3adce: mov qword ptr [rbp - 8], r12
140b3add2: mov qword ptr [rbp], rax
140b3add6: mov qword ptr [rbp + 8], 0
140b3adde: mov qword ptr [rbp + 0x10], r15
140b3ade2: mov qword ptr [rbp + 0x18], r12
140b3ade6: mov qword ptr [rbp + 0x20], rcx
140b3adea: lea rcx, [rbp + 0x80]
140b3adf1: lea rdx, [rbp - 0x20]
140b3adf5: call 0x14143c450
140b3adfa: mov rax, qword ptr [rbp + 0x80]
140b3ae01: test rax, rax
140b3ae04: je 0x140b3ae4a
140b3ae06: mov rcx, qword ptr [rbp + 0x90]
140b3ae0d: lea rdx, [rcx + rcx*2]
140b3ae11: mov r8, qword ptr [rax + rdx*8 + 0x168]
140b3ae19: mov r9, qword ptr [rax + rdx*8 + 0x170]
140b3ae21: mov rbx, qword ptr [rax + rdx*8 + 0x178]
140b3ae29: shl rcx, 5
140b3ae2d: movups xmm0, xmmword ptr [rax + rcx]
140b3ae31: movups xmm1, xmmword ptr [rax + rcx + 0x10]
140b3ae36: movaps xmmword ptr [rbp + 0x30], xmm0
140b3ae3a: movaps xmmword ptr [rbp + 0x40], xmm1
140b3ae3e: mov rax, r8
140b3ae41: neg rax
140b3ae44: jno 0x140b3afe3
140b3ae4a: mov byte ptr [rbp + 0x80], 0xb
140b3ae51: lea rdx, [rip + 0x1fb2c68]
140b3ae58: lea r8, [rip + 0x1fb2c71]
140b3ae5f: lea rcx, [rbp + 0x80]
140b3ae66: call 0x1428f9b40
140b3ae6b: mov qword ptr [rsi + 8], rax
140b3ae6f: mov qword ptr [rsi], 0xf
140b3ae76: mov byte ptr [rbp + 0xa7], 1
140b3ae7d: lea rcx, [rbp - 0x20]
140b3ae81: call 0x140887df0
140b3ae86: jmp 0x140b3b347
140b3ae8b: mov r15, qword ptr [rdx + 8]
140b3ae8f: mov r12, qword ptr [rdx + 0x10]
140b3ae93: mov rdi, qword ptr [rdx + 0x18]
140b3ae97: mov r14b, 6
140b3ae9a: mov rax, r15
140b3ae9d: mov rcx, r12
140b3aea0: mov rbx, rdi
140b3aea3: mov qword ptr [rbp - 0x20], rax
140b3aea7: mov qword ptr [rbp - 0x18], rcx
140b3aeab: mov qword ptr [rbp - 0x10], rbx
140b3aeaf: movaps xmm0, xmmword ptr [rbp - 0x40]
140b3aeb3: movups xmmword ptr [rbp - 7], xmm0
140b3aeb7: movups xmm0, xmmword ptr [rbp - 0x31]
140b3aebb: movups xmmword ptr [rbp + 8], xmm0
140b3aebf: mov byte ptr [rbp - 8], r14b
140b3aec3: mov byte ptr [rbp + 0xa7], 1
140b3aeca: lea rcx, [rbp + 0x30]
140b3aece: lea rdx, [rbp - 0x20]
140b3aed2: call 0x1415d3950
140b3aed7: movzx ecx, byte ptr [rbp + 0x38]
140b3aedb: mov rax, qword ptr [rbp + 0x30]
140b3aedf: cmp cl, 7
140b3aee2: jne 0x140b3af8f
140b3aee8: mov qword ptr [rsi + 8], rax
140b3aeec: mov qword ptr [rsi], 0xf
140b3aef3: mov eax, dword ptr [rbp + 0x70]
140b3aef6: cmp eax, 3
140b3aef9: jne 0x140b3b313
140b3aeff: jmp 0x140b3b347
140b3af04: movups xmm0, xmmword ptr [rdx]
140b3af07: movups xmm1, xmmword ptr [rdx + 0x10]
140b3af0b: movaps xmmword ptr [rbp - 0x10], xmm1
140b3af0f: movaps xmmword ptr [rbp - 0x20], xmm0
140b3af13: lea rcx, [rbp + 0x30]
140b3af17: lea rdx, [rbp - 0x20]
140b3af1b: call 0x1428c9c10
140b3af20: lea rdx, [rip + 0x1fb2bd9]
140b3af27: lea r8, [rip + 0x1fb2ba2]
140b3af2e: lea rcx, [rbp + 0x30]
140b3af32: call 0x1428c9a00
140b3af37: mov qword ptr [rsi + 8], rax
140b3af3b: mov qword ptr [rsi], 0xf
140b3af42: movzx eax, byte ptr [rbp - 0x20]
140b3af46: cmp eax, 3
140b3af49: jb 0x140b3b347
140b3af4f: je 0x140b3b092
140b3af55: cmp eax, 4
140b3af58: jne 0x140b3b0ae
140b3af5e: mov rcx, qword ptr [rbp - 0x10]
140b3af62: mov rdx, qword ptr [rbp - 8]
140b3af66: mov qword ptr [rbp + 0x78], rcx
140b3af6a: call 0x140888d70
140b3af6f: mov rdx, qword ptr [rbp - 0x18]
140b3af73: test rdx, rdx
140b3af76: je 0x140b3b347
140b3af7c: shl rdx, 5
140b3af80: mov r8d, 8
140b3af86: mov rcx, qword ptr [rbp + 0x78]
140b3af8a: jmp 0x140b3b342
140b3af8f: movups xmm0, xmmword ptr [rbp + 0x39]
140b3af93: movups xmm1, xmmword ptr [rbp + 0x48]
140b3af97: movups xmmword ptr [rbp + 0x90], xmm1
140b3af9e: movups xmmword ptr [rbp + 0x81], xmm0
140b3afa5: mov byte ptr [rbp + 0x80], cl
140b3afab: movzx eax, al
140b3afae: lea rcx, [rip + 0x1fb346f]
140b3afb5: movsxd rax, dword ptr [rcx + rax*4]
140b3afb9: add rax, rcx
140b3afbc: jmp rax
140b3afbe: mov byte ptr [rbp + 0xa7], 1
140b3afc5: lea rcx, [rbp + 0x80]
140b3afcc: call 0x1415f20d0
140b3afd1: xor ecx, ecx
140b3afd3: test al, 1
140b3afd5: mov eax, 0xf
140b3afda: cmove rax, rcx
140b3afde: jmp 0x140b3b2d0
140b3afe3: mov qword ptr [rbp + 0x28], r9
140b3afe7: mov qword ptr [rbp + 0x78], r8
140b3afeb: movaps xmm0, xmmword ptr [rbp + 0x30]
140b3afef: movaps xmm1, xmmword ptr [rbp + 0x40]
140b3aff3: movaps xmmword ptr [rbp + 0x90], xmm1
140b3affa: movaps xmmword ptr [rbp + 0x80], xmm0
140b3b001: lea rcx, [rbp - 0x58]
140b3b005: lea rdx, [rbp - 0x20]
140b3b009: call 0x14143c450
140b3b00e: mov rax, qword ptr [rbp - 0x58]
140b3b012: test rax, rax
140b3b015: je 0x140b3b058
140b3b017: mov rcx, qword ptr [rbp - 0x48]
140b3b01b: lea rdx, [rcx + rcx*2]
140b3b01f: mov r8, qword ptr [rax + rdx*8 + 0x178]
140b3b027: mov qword ptr [rbp + 0x40], r8
140b3b02b: movups xmm0, xmmword ptr [rax + rdx*8 + 0x168]
140b3b033: movaps xmmword ptr [rbp + 0x30], xmm0
140b3b037: shl rcx, 5
140b3b03b: movups xmm0, xmmword ptr [rax + rcx]
140b3b03f: movups xmm1, xmmword ptr [rax + rcx + 0x10]
140b3b044: movups xmmword ptr [rbp + 0x48], xmm0
140b3b048: movups xmmword ptr [rbp + 0x58], xmm1
140b3b04c: mov rdx, qword ptr [rbp + 0x30]
140b3b050: mov rax, rdx
140b3b053: neg rax
140b3b056: jno 0x140b3b0c3
140b3b058: mov r14b, byte ptr [rbp + 0x80]
140b3b05f: movups xmm0, xmmword ptr [rbp + 0x81]
140b3b066: movaps xmmword ptr [rbp - 0x40], xmm0
140b3b06a: movups xmm0, xmmword ptr [rbp + 0x90]
140b3b071: movups xmmword ptr [rbp - 0x31], xmm0
140b3b075: mov byte ptr [rbp + 0xa7], 1
140b3b07c: lea rcx, [rbp - 0x20]
140b3b080: call 0x140887df0
140b3b085: mov rax, qword ptr [rbp + 0x78]
140b3b089: mov rcx, qword ptr [rbp + 0x28]
140b3b08d: jmp 0x140b3aea3
140b3b092: mov rdx, qword ptr [rbp - 0x18]
140b3b096: test rdx, rdx
140b3b099: je 0x140b3b347
140b3b09f: mov rcx, qword ptr [rbp - 0x10]
140b3b0a3: mov r8d, 1
140b3b0a9: jmp 0x140b3b342
140b3b0ae: lea rcx, [rbp - 0x18]
140b3b0b2: mov byte ptr [rbp + 0xa7], 0
140b3b0b9: call 0x140898f40
140b3b0be: jmp 0x140b3b347
140b3b0c3: jae 0x140b3b0d4
140b3b0c5: mov rcx, qword ptr [rbp + 0x38]
140b3b0c9: mov r8d, 1
140b3b0cf: call 0x140613c20
140b3b0d4: movzx eax, byte ptr [rbp + 0x48]
140b3b0d8: cmp eax, 3
140b3b0db: jb 0x140b3b37b
140b3b0e1: je 0x140b3b35c
140b3b0e7: lea rcx, [rbp + 0x50]
140b3b0eb: cmp eax, 4
140b3b0ee: jne 0x140b3b376
140b3b0f4: call 0x140889390
140b3b0f9: jmp 0x140b3b37b
140b3b0fe: mov byte ptr [rbp + 0xa7], 1
140b3b105: lea rcx, [rbp + 0x80]
140b3b10c: call 0x1415f2200
140b3b111: and eax, 1
140b3b114: or rax, 0xe
140b3b118: jmp 0x140b3b2d0
140b3b11d: mov byte ptr [rbp + 0xa7], 1
140b3b124: lea rdx, [rbp + 0x80]
140b3b12b: mov r8d, 8
140b3b131: mov rcx, rsi
140b3b134: xor r9d, r9d
140b3b137: call 0x1415f0fd0
140b3b13c: jmp 0x140b3b30b
140b3b141: mov byte ptr [rbp + 0xa7], 1
140b3b148: lea rcx, [rbp + 0x80]
140b3b14f: call 0x1415f2010
140b3b154: test al, 1
140b3b156: mov eax, 0xf
140b3b15b: mov ecx, 4
140b3b160: jmp 0x140b3b300
140b3b165: mov byte ptr [rbp + 0xa7], 1
140b3b16c: lea rcx, [rbp + 0x80]
140b3b173: call 0x1415f1fb0
140b3b178: and eax, 1
140b3b17b: lea rax, [rax + rax*2]
140b3b17f: add rax, 0xc
140b3b183: jmp 0x140b3b2d0
140b3b188: mov byte ptr [rbp + 0xa7], 1
140b3b18f: lea rcx, [rbp + 0x80]
140b3b196: call 0x1415f1f50
140b3b19b: test al, 1
140b3b19d: mov eax, 0xf
140b3b1a2: mov ecx, 9
140b3b1a7: jmp 0x140b3b300
140b3b1ac: mov byte ptr [rbp + 0xa7], 1
140b3b1b3: lea rcx, [rbp + 0x80]
140b3b1ba: call 0x1415f2010
140b3b1bf: test al, 1
140b3b1c1: mov eax, 0xf
140b3b1c6: mov ecx, 2
140b3b1cb: jmp 0x140b3b300
140b3b1d0: mov byte ptr [rbp + 0xa7], 1
140b3b1d7: lea rcx, [rbp + 0x80]
140b3b1de: call 0x1415f2010
140b3b1e3: test al, 1
140b3b1e5: mov eax, 0xf
140b3b1ea: mov ecx, 3
140b3b1ef: jmp 0x140b3b300
140b3b1f4: mov byte ptr [rbp + 0xa7], 1
140b3b1fb: lea rcx, [rbp + 0x30]
140b3b1ff: lea rdx, [rbp + 0x80]
140b3b206: call 0x1415f2130
140b3b20b: mov rax, qword ptr [rbp + 0x30]
140b3b20f: test rax, rax
140b3b212: je 0x140b3b408
140b3b218: mov rcx, qword ptr [rbp + 0x38]
140b3b21c: mov qword ptr [rsi + 0x10], rcx
140b3b220: mov ecx, 7
140b3b225: jmp 0x140b3b411
140b3b22a: mov byte ptr [rbp + 0xa7], 1
140b3b231: lea rcx, [rbp + 0x80]
140b3b238: call 0x1415f2010
140b3b23d: test al, 1
140b3b23f: mov eax, 0xf
140b3b244: mov ecx, 1
140b3b249: jmp 0x140b3b300
140b3b24e: mov byte ptr [rbp + 0xa7], 1
140b3b255: lea rcx, [rbp + 0x80]
140b3b25c: call 0x1415f2010
140b3b261: test al, 1
140b3b263: mov eax, 0xf
140b3b268: mov ecx, 5
140b3b26d: jmp 0x140b3b300
140b3b272: mov byte ptr [rbp + 0xa7], 1
140b3b279: lea rcx, [rbp + 0x80]
140b3b280: call 0x1415f2070
140b3b285: and eax, 1
140b3b288: lea rax, [rax*2 + 0xd]
140b3b290: jmp 0x140b3b2d0
140b3b292: mov byte ptr [rbp + 0xa7], 1
140b3b299: lea rcx, [rbp + 0x80]
140b3b2a0: call 0x1415f2010
140b3b2a5: and eax, 1
140b3b2a8: lea rax, [rax + rax*8]
140b3b2ac: add rax, 6
140b3b2b0: jmp 0x140b3b2d0
140b3b2b2: mov byte ptr [rbp + 0xa7], 1
140b3b2b9: lea rcx, [rbp + 0x80]
140b3b2c0: call 0x1415f1ef0
140b3b2c5: and eax, 1
140b3b2c8: lea rax, [rax + rax*4]
140b3b2cc: add rax, 0xa
140b3b2d0: mov qword ptr [rsi + 8], rdx
140b3b2d4: mov qword ptr [rsi], rax
140b3b2d7: mov eax, dword ptr [rbp + 0x70]
140b3b2da: cmp eax, 3
140b3b2dd: jne 0x140b3b313
140b3b2df: jmp 0x140b3b347
140b3b2e1: mov byte ptr [rbp + 0xa7], 1
140b3b2e8: lea rcx, [rbp + 0x80]
140b3b2ef: call 0x1415f21a0
140b3b2f4: test al, 1
140b3b2f6: mov eax, 0xf
140b3b2fb: mov ecx, 8
140b3b300: cmovne rcx, rax
140b3b304: mov qword ptr [rsi + 8], rdx
140b3b308: mov qword ptr [rsi], rcx
140b3b30b: mov eax, dword ptr [rbp + 0x70]
140b3b30e: cmp eax, 3
140b3b311: je 0x140b3b347
140b3b313: cmp eax, 5
140b3b316: je 0x140b3b347
140b3b318: mov qword ptr [rbp + 0x70], r15
140b3b31c: mov qword ptr [rbp + 0x68], r12
140b3b320: mov rcx, r12
140b3b323: mov rdx, rdi
140b3b326: call 0x140888d70
140b3b32b: mov rdx, qword ptr [rbp + 0x70]
140b3b32f: test rdx, rdx
140b3b332: mov rcx, qword ptr [rbp + 0x68]
140b3b336: je 0x140b3b347
140b3b338: shl rdx, 5
140b3b33c: mov r8d, 8
140b3b342: call 0x140613c20
140b3b347: mov rax, rsi
140b3b34a: add rsp, 0x130
140b3b351: pop rbx
140b3b352: pop rdi
140b3b353: pop rsi
140b3b354: pop r12
140b3b356: pop r14
140b3b358: pop r15
140b3b35a: pop rbp
140b3b35b: ret 
140b3b35c: mov rdx, qword ptr [rbp + 0x50]
140b3b360: test rdx, rdx
140b3b363: je 0x140b3b37b
140b3b365: mov rcx, qword ptr [rbp + 0x58]
140b3b369: mov r8d, 1
140b3b36f: call 0x140613c20
140b3b374: jmp 0x140b3b37b
140b3b376: call 0x140898f40
140b3b37b: mov byte ptr [rbp + 0x30], 0xb
140b3b37f: lea rdx, [rip + 0x1fb273a]
140b3b386: lea r8, [rip + 0x1fb2743]
140b3b38d: lea rcx, [rbp + 0x30]
140b3b391: call 0x1428f9b40
140b3b396: mov qword ptr [rsi + 8], rax
140b3b39a: mov qword ptr [rsi], 0xf
140b3b3a1: movzx eax, byte ptr [rbp + 0x80]
140b3b3a8: cmp eax, 3
140b3b3ab: jb 0x140b3b3e7
140b3b3ad: je 0x140b3b3c2
140b3b3af: lea rcx, [rbp + 0x88]
140b3b3b6: cmp eax, 4
140b3b3b9: jne 0x140b3b3e2
140b3b3bb: call 0x140889390
140b3b3c0: jmp 0x140b3b3e7
140b3b3c2: mov rdx, qword ptr [rbp + 0x88]
140b3b3c9: test rdx, rdx
140b3b3cc: je 0x140b3b3e7
140b3b3ce: mov rcx, qword ptr [rbp + 0x90]
140b3b3d5: mov r8d, 1
140b3b3db: call 0x140613c20
140b3b3e0: jmp 0x140b3b3e7
140b3b3e2: call 0x140898f40
140b3b3e7: mov rdx, qword ptr [rbp + 0x78]
140b3b3eb: test rdx, rdx
140b3b3ee: je 0x140b3ae76
140b3b3f4: mov r8d, 1
140b3b3fa: mov rcx, qword ptr [rbp + 0x28]
140b3b3fe: call 0x140613c20
140b3b403: jmp 0x140b3ae76
140b3b408: mov ecx, 0xf
140b3b40d: mov rax, qword ptr [rbp + 0x38]
140b3b411: mov qword ptr [rsi + 8], rax
140b3b415: mov qword ptr [rsi], rcx
140b3b418: mov eax, dword ptr [rbp + 0x70]
140b3b41b: cmp eax, 3
140b3b41e: jne 0x140b3b313
140b3b424: jmp 0x140b3b347
140b3b429: nop dword ptr [rax]
140b3b430: mov qword ptr [rsp + 0x10], rdx
140b3b435: push rbp
140b3b436: push r15
140b3b438: push r14
140b3b43a: push r12
140b3b43c: push rsi
140b3b43d: push rdi
140b3b43e: push rbx
140b3b43f: sub rsp, 0x20
140b3b443: lea rbp, [rdx + 0x80]
140b3b44a: lea rcx, [rbp - 0x20]
140b3b44e: call 0x140887df0
140b3b453: mov byte ptr [rbp + 0xa7], 1
140b3b45a: add rsp, 0x20
140b3b45e: pop rbx
140b3b45f: pop rdi
140b3b460: pop rsi
140b3b461: pop r12
140b3b463: pop r14
140b3b465: pop r15
140b3b467: pop rbp
140b3b468: ret 
140b3b469: nop dword ptr [rax]
140b3b470: mov qword ptr [rsp + 0x10], rdx
140b3b475: push rbp
140b3b476: push r15
140b3b478: push r14
140b3b47a: push r12
140b3b47c: push rsi
140b3b47d: push rdi
140b3b47e: push rbx
140b3b47f: sub rsp, 0x20
140b3b483: lea rbp, [rdx + 0x80]
140b3b48a: cmp qword ptr [rbp + 0x78], 0
140b3b48f: je 0x140b3b4a4
140b3b491: mov r8d, 1
140b3b497: mov rcx, qword ptr [rbp + 0x28]
140b3b49b: mov rdx, qword ptr [rbp + 0x78]
140b3b49f: call 0x140613c20
140b3b4a4: nop 
140b3b4a5: add rsp, 0x20
140b3b4a9: pop rbx
140b3b4aa: pop rdi
140b3b4ab: pop rsi
140b3b4ac: pop r12
140b3b4ae: pop r14
140b3b4b0: pop r15
140b3b4b2: pop rbp
140b3b4b3: ret 
140b3b4b4: nop word ptr cs:[rax + rax]
140b3b4c0: mov qword ptr [rsp + 0x10], rdx
140b3b4c5: push rbp
140b3b4c6: push r15
140b3b4c8: push r14
140b3b4ca: push r12
140b3b4cc: push rsi
140b3b4cd: push rdi
140b3b4ce: push rbx
140b3b4cf: sub rsp, 0x20
140b3b4d3: lea rbp, [rdx + 0x80]
140b3b4da: cmp qword ptr [rbp + 0x70], 0
140b3b4df: je 0x140b3b4f8
140b3b4e1: mov rdx, qword ptr [rbp + 0x70]
140b3b4e5: shl rdx, 5
140b3b4e9: mov r8d, 8
140b3b4ef: mov rcx, qword ptr [rbp + 0x68]
140b3b4f3: call 0x140613c20
140b3b4f8: nop 
140b3b4f9: add rsp, 0x20
140b3b4fd: pop rbx
140b3b4fe: pop rdi
140b3b4ff: pop rsi
140b3b500: pop r12
140b3b502: pop r14
140b3b504: pop r15
140b3b506: pop rbp
140b3b507: ret 
140b3b508: nop dword ptr [rax + rax]
140b3b510: mov qword ptr [rsp + 0x10], rdx
140b3b515: push rbp
140b3b516: push r15
140b3b518: push r14
140b3b51a: push r12
140b3b51c: push rsi
140b3b51d: push rdi
140b3b51e: push rbx
140b3b51f: sub rsp, 0x20
140b3b523: lea rbp, [rdx + 0x80]
140b3b52a: lea rcx, [rbp - 0x20]
140b3b52e: call 0x14085da90
140b3b533: mov byte ptr [rbp + 0xa7], 0
140b3b53a: add rsp, 0x20
140b3b53e: pop rbx
140b3b53f: pop rdi
140b3b540: pop rsi
140b3b541: pop r12
140b3b543: pop r14
140b3b545: pop r15
140b3b547: pop rbp
140b3b548: ret 
140b3b549: nop dword ptr [rax]
140b3b550: mov qword ptr [rsp + 0x10], rdx
140b3b555: push rbp
140b3b556: push r15
140b3b558: push r14
140b3b55a: push r12
140b3b55c: push rsi
140b3b55d: push rdi
140b3b55e: push rbx
140b3b55f: sub rsp, 0x20
140b3b563: lea rbp, [rdx + 0x80]
140b3b56a: mov rdx, qword ptr [rbp - 0x18]
140b3b56e: test rdx, rdx
140b3b571: je 0x140b3b586
140b3b573: shl rdx, 5
140b3b577: mov r8d, 8
140b3b57d: mov rcx, qword ptr [rbp + 0x78]
140b3b581: call 0x140613c20
140b3b586: mov byte ptr [rbp + 0xa7], 0
140b3b58d: add rsp, 0x20
140b3b591: pop rbx
140b3b592: pop rdi
140b3b593: pop rsi
140b3b594: pop r12
140b3b596: pop r14
140b3b598: pop r15
140b3b59a: pop rbp
140b3b59b: ret 
140b3b59c: nop dword ptr [rax]
140b3b5a0: mov qword ptr [rsp + 0x10], rdx
140b3b5a5: push rbp
140b3b5a6: push r15
140b3b5a8: push r14
140b3b5aa: push r12
140b3b5ac: push rsi
140b3b5ad: push rdi
140b3b5ae: push rbx
140b3b5af: sub rsp, 0x20
140b3b5b3: lea rbp, [rdx + 0x80]
140b3b5ba: cmp byte ptr [rbp + 0xa7], 0
140b3b5c1: je 0x140b3b5d2
140b3b5c3: cmp byte ptr [rbp + 0x70], 3
140b3b5c7: je 0x140b3b5d2
140b3b5c9: movzx eax, byte ptr [rbp + 0x70]
140b3b5cd: cmp eax, 5
140b3b5d0: jne 0x140b3b5e1
140b3b5d2: add rsp, 0x20
140b3b5d6: pop rbx
140b3b5d7: pop rdi
140b3b5d8: pop rsi
140b3b5d9: pop r12
140b3b5db: pop r14
140b3b5dd: pop r15
140b3b5df: pop rbp
140b3b5e0: ret 
140b3b5e1: mov rcx, qword ptr [rbp + 0x68]
140b3b5e5: call 0x14085da90
140b3b5ea: jmp 0x140b3b5d2
140b3b5ec: nop dword ptr [rax]
140b3b5f0: mov qword ptr [rsp + 0x10], rdx
140b3b5f5: push rbp
140b3b5f6: push r15
140b3b5f8: push r14
140b3b5fa: push r12
140b3b5fc: push rsi
140b3b5fd: push rdi
140b3b5fe: push rbx
140b3b5ff: sub rsp, 0x20
140b3b603: lea rbp, [rdx + 0x80]
140b3b60a: lea rcx, [rbp + 0x80]
140b3b611: call 0x14085da90
140b3b616: nop 
140b3b617: add rsp, 0x20
140b3b61b: pop rbx
140b3b61c: pop rdi
140b3b61d: pop rsi
140b3b61e: pop r12
140b3b620: pop r14
140b3b622: pop r15
140b3b624: pop rbp
140b3b625: ret 
