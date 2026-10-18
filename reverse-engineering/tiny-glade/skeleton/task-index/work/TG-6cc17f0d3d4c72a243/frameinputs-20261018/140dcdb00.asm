140dcdb00: push r15
140dcdb02: push r14
140dcdb04: push rsi
140dcdb05: push rdi
140dcdb06: push rbx
140dcdb07: sub rsp, 0x60
140dcdb0b: mov rdi, rdx
140dcdb0e: mov rsi, rcx
140dcdb11: lea rax, [rdx + 0x78]
140dcdb15: mov qword ptr [rsp + 0x28], rax
140dcdb1a: lea rbx, [rsp + 0x30]
140dcdb1f: lea r14, [rsp + 0x28]
140dcdb24: jmp 0x140dcdb63
140dcdb26: nop word ptr cs:[rax + rax]
140dcdb30: mov rdx, qword ptr [rdi + 0x50]
140dcdb34: mov rax, qword ptr [rdi + 0x58]
140dcdb38: mov r9d, ecx
140dcdb3b: imul r8, r9, 0x58
140dcdb3f: add r8, rdx
140dcdb42: lea r9, [rax + r9*8]
140dcdb46: inc rcx
140dcdb49: mov qword ptr [rdi + 0x68], rcx
140dcdb4d: mov rcx, rbx
140dcdb50: mov rdx, r14
140dcdb53: call 0x1419ee9e0
140dcdb58: test byte ptr [rsp + 0x30], 1
140dcdb5d: jne 0x140dcdda8
140dcdb63: mov rdx, qword ptr [rdi + 8]
140dcdb67: mov rax, qword ptr [rdi + 0x18]
140dcdb6b: mov rcx, qword ptr [rdi + 0x68]
140dcdb6f: mov r9, qword ptr [rdi + 0x60]
140dcdb73: mov r8, qword ptr [rdi + 0x28]
140dcdb77: cmp byte ptr [rdi + 0x70], 0
140dcdb7b: je 0x140dcdc10
140dcdb81: cmp rcx, r9
140dcdb84: jne 0x140dcdb30
140dcdb86: mov rcx, qword ptr [rdi + 0x20]
140dcdb8a: nop word ptr [rax + rax]
140dcdb90: cmp rcx, r8
140dcdb93: je 0x140dcdd91
140dcdb99: lea r11, [rcx + 4]
140dcdb9d: mov qword ptr [rdi + 0x20], r11
140dcdba1: mov ecx, dword ptr [rcx]
140dcdba3: mov r9, qword ptr [rdx + 8]
140dcdba7: lea r10, [rcx + rcx*8]
140dcdbab: cmp qword ptr [r9 + r10*8 + 0x10], 0
140dcdbb1: mov rcx, r11
140dcdbb4: je 0x140dcdb90
140dcdbb6: lea rcx, [r9 + r10*8]
140dcdbba: mov rdx, qword ptr [rax + 0x110]
140dcdbc1: mov r8, qword ptr [rcx + 0x40]
140dcdbc5: cmp r8, rdx
140dcdbc8: jbe 0x140dcdcd1
140dcdbce: mov r9, qword ptr [rcx + 0x38]
140dcdbd2: mov rdx, qword ptr [r9 + rdx*8]
140dcdbd6: test rdx, rdx
140dcdbd9: je 0x140dcdcd1
140dcdbdf: mov r9, qword ptr [rcx + 0x18]
140dcdbe3: not rdx
140dcdbe6: lea rdx, [rdx + rdx*2]
140dcdbea: shl rdx, 4
140dcdbee: mov rdx, qword ptr [r9 + rdx + 0x10]
140dcdbf3: mov rax, qword ptr [rax + 0x118]
140dcdbfa: mov qword ptr [rdi + 0x50], rdx
140dcdbfe: cmp r8, rax
140dcdc01: ja 0x140dcdce3
140dcdc07: jmp 0x140dcdd3b
140dcdc0c: nop dword ptr [rax]
140dcdc10: cmp rcx, r9
140dcdc13: jne 0x140dcdcc0
140dcdc19: mov r9, qword ptr [rdi + 0x10]
140dcdc1d: mov r10, qword ptr [rdi + 0x20]
140dcdc21: nop word ptr cs:[rax + rax]
140dcdc30: cmp r10, r8
140dcdc33: je 0x140dcdd91
140dcdc39: lea r15, [r10 + 4]
140dcdc3d: mov qword ptr [rdi + 0x20], r15
140dcdc41: mov r10d, dword ptr [r10]
140dcdc44: mov rcx, qword ptr [r9 + 8]
140dcdc48: lea r11, [r10 + r10*4]
140dcdc4c: shl r11, 5
140dcdc50: cmp qword ptr [rcx + r11 + 0x58], 0
140dcdc56: mov r10, r15
140dcdc59: je 0x140dcdc30
140dcdc5b: add rcx, r11
140dcdc5e: mov r8d, dword ptr [rcx + 0x94]
140dcdc65: mov r9, qword ptr [rdx + 8]
140dcdc69: lea r10, [r8 + r8*8]
140dcdc6d: lea r8, [r9 + r10*8]
140dcdc71: mov rdx, qword ptr [rax + 0x110]
140dcdc78: mov r9, qword ptr [r9 + r10*8 + 0x40]
140dcdc7d: cmp r9, rdx
140dcdc80: jbe 0x140dcdd06
140dcdc86: mov r10, qword ptr [r8 + 0x38]
140dcdc8a: mov rdx, qword ptr [r10 + rdx*8]
140dcdc8e: test rdx, rdx
140dcdc91: je 0x140dcdd06
140dcdc93: mov r10, qword ptr [r8 + 0x18]
140dcdc97: not rdx
140dcdc9a: lea rdx, [rdx + rdx*2]
140dcdc9e: shl rdx, 4
140dcdca2: mov rdx, qword ptr [r10 + rdx + 0x10]
140dcdca7: mov rax, qword ptr [rax + 0x118]
140dcdcae: mov qword ptr [rdi + 0x50], rdx
140dcdcb2: cmp r9, rax
140dcdcb5: ja 0x140dcdd18
140dcdcb7: jmp 0x140dcdd58
140dcdcbc: nop dword ptr [rax]
140dcdcc0: mov r8, qword ptr [rdi + 0x40]
140dcdcc4: mov rdx, qword ptr [rdi + 0x50]
140dcdcc8: mov rax, qword ptr [rdi + 0x58]
140dcdccc: jmp 0x140dcdd80
140dcdcd1: xor edx, edx
140dcdcd3: mov rax, qword ptr [rax + 0x118]
140dcdcda: mov qword ptr [rdi + 0x50], rdx
140dcdcde: cmp r8, rax
140dcdce1: jbe 0x140dcdd3b
140dcdce3: mov r8, qword ptr [rcx + 0x38]
140dcdce7: mov rax, qword ptr [r8 + rax*8]
140dcdceb: test rax, rax
140dcdcee: je 0x140dcdd3b
140dcdcf0: mov r8, qword ptr [rcx + 0x18]
140dcdcf4: not rax
140dcdcf7: lea rax, [rax + rax*2]
140dcdcfb: shl rax, 4
140dcdcff: mov rax, qword ptr [r8 + rax + 0x10]
140dcdd04: jmp 0x140dcdd3d
140dcdd06: xor edx, edx
140dcdd08: mov rax, qword ptr [rax + 0x118]
140dcdd0f: mov qword ptr [rdi + 0x50], rdx
140dcdd13: cmp r9, rax
140dcdd16: jbe 0x140dcdd58
140dcdd18: mov r9, qword ptr [r8 + 0x38]
140dcdd1c: mov rax, qword ptr [r9 + rax*8]
140dcdd20: test rax, rax
140dcdd23: je 0x140dcdd58
140dcdd25: mov r8, qword ptr [r8 + 0x18]
140dcdd29: not rax
140dcdd2c: lea rax, [rax + rax*2]
140dcdd30: shl rax, 4
140dcdd34: mov rax, qword ptr [r8 + rax + 0x10]
140dcdd39: jmp 0x140dcdd5a
140dcdd3b: xor eax, eax
140dcdd3d: mov qword ptr [rdi + 0x58], rax
140dcdd41: movups xmm0, xmmword ptr [rcx + 8]
140dcdd45: movups xmmword ptr [rdi + 0x30], xmm0
140dcdd49: mov rcx, qword ptr [rcx + 0x10]
140dcdd4d: mov qword ptr [rdi + 0x60], rcx
140dcdd51: xor ecx, ecx
140dcdd53: jmp 0x140dcdb38
140dcdd58: xor eax, eax
140dcdd5a: mov qword ptr [rdi + 0x58], rax
140dcdd5e: mov r8, qword ptr [rcx + 0x50]
140dcdd62: mov r9, qword ptr [rcx + 0x58]
140dcdd66: mov qword ptr [rdi + 0x40], r8
140dcdd6a: mov qword ptr [rdi + 0x48], r9
140dcdd6e: mov rcx, qword ptr [rcx + 0x58]
140dcdd72: mov qword ptr [rdi + 0x60], rcx
140dcdd76: mov qword ptr [rdi + 0x68], 0
140dcdd7e: xor ecx, ecx
140dcdd80: mov r9, rcx
140dcdd83: shl r9, 4
140dcdd87: mov r9d, dword ptr [r8 + r9 + 8]
140dcdd8c: jmp 0x140dcdb3b
140dcdd91: xor eax, eax
140dcdd93: xor ecx, ecx
140dcdd95: mov qword ptr [rsi], rax
140dcdd98: mov qword ptr [rsi + 8], rcx
140dcdd9c: add rsp, 0x60
140dcdda0: pop rbx
140dcdda1: pop rdi
140dcdda2: pop rsi
140dcdda3: pop r14
140dcdda5: pop r15
140dcdda7: ret 
140dcdda8: movaps xmm0, xmmword ptr [rsp + 0x40]
140dcddad: movaps xmm1, xmmword ptr [rsp + 0x50]
140dcddb2: movaps xmmword ptr [rsi + 0x20], xmm1
140dcddb6: movaps xmmword ptr [rsi + 0x10], xmm0
140dcddba: mov eax, 1
140dcddbf: jmp 0x140dcdd93
