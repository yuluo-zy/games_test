0x1400ddc80: push rbp
0x1400ddc81: push r15
0x1400ddc83: push r14
0x1400ddc85: push r13
0x1400ddc87: push r12
0x1400ddc89: push rsi
0x1400ddc8a: push rdi
0x1400ddc8b: push rbx
0x1400ddc8c: sub rsp, 0xb68
0x1400ddc93: lea rbp, [rsp + 0x80]
0x1400ddc9b: movaps xmmword ptr [rbp + 0xad0], xmm11
0x1400ddca3: movaps xmmword ptr [rbp + 0xac0], xmm10
0x1400ddcab: movaps xmmword ptr [rbp + 0xab0], xmm9
0x1400ddcb3: movaps xmmword ptr [rbp + 0xaa0], xmm8
0x1400ddcbb: movaps xmmword ptr [rbp + 0xa90], xmm7
0x1400ddcc2: movdqa xmmword ptr [rbp + 0xa80], xmm6
0x1400ddcca: mov qword ptr [rbp + 0xa78], 0xfffffffffffffffe
0x1400ddcd5: mov rbx, r9
0x1400ddcd8: mov rsi, r8
0x1400ddcdb: mov rdi, rdx
0x1400ddcde: mov rax, rcx
0x1400ddce1: mov rcx, qword ptr [rdx + 0x190]
0x1400ddce8: mov qword ptr [rbp + 0x9f8], rcx
0x1400ddcef: lea rcx, [rax + 0x48]
0x1400ddcf3: mov qword ptr [rbp + 0x940], rcx
0x1400ddcfa: mov rcx, qword ptr [rax + 0x50]
0x1400ddcfe: mov rdx, qword ptr [rax + 0x58]
0x1400ddd02: mov qword ptr [rbp + 0xa68], rdx
0x1400ddd09: mov qword ptr [rbp + 0xa50], rax
0x1400ddd10: mov qword ptr [rax + 0x58], 0
0x1400ddd18: nop dword ptr [rax + rax]
0x1400ddd20: mov rax, qword ptr [rbp + 0xa68]
0x1400ddd27: sub rax, 1
0x1400ddd2b: jb 0x1400ddd49
0x1400ddd2d: mov qword ptr [rbp + 0xa68], rax
0x1400ddd34: lea r14, [rcx + 0x28]
0x1400ddd38: mov qword ptr [rbp + 0xa58], r14
0x1400ddd3f: call 0x140be24b0
0x1400ddd44: mov rcx, r14
0x1400ddd47: jmp 0x1400ddd20
0x1400ddd49: mov qword ptr [rbp + 0x7b8], rbx
0x1400ddd50: mov qword ptr [rbp + 0x7b0], rdi
0x1400ddd57: mov qword ptr [rbp + 0x7a8], rsi
0x1400ddd5e: mov r9, qword ptr [rbp + 0xa50]
0x1400ddd65: lea rdx, [r9 + 0x18]
0x1400ddd69: mov rax, qword ptr [r9 + 0x20]
0x1400ddd6d: mov r8, qword ptr [r9 + 0x28]
0x1400ddd71: mov qword ptr [r9 + 0x28], 0
0x1400ddd79: imul rcx, r8, 0x58
0x1400ddd7d: add rcx, rax
0x1400ddd80: mov qword ptr [rbp + 0x9b8], r8
0x1400ddd87: mov qword ptr [rbp + 0x9c0], 0
0x1400ddd92: mov qword ptr [rbp + 0x9a0], rax
0x1400ddd99: mov qword ptr [rbp + 0x9a8], rcx
0x1400ddda0: mov qword ptr [rbp + 0x9b0], rdx
0x1400ddda7: lea rdx, [r9 + 0x78]
0x1400dddab: mov qword ptr [rbp + 0x948], rdx
0x1400dddb2: lea rdx, [r9 + 0x98]
0x1400dddb9: mov qword ptr [rbp + 0x9f0], rdx
0x1400dddc0: lea r15, [rip + 0x285d349]
0x1400dddc7: pcmpeqd xmm6, xmm6
0x1400dddcb: movss xmm7, dword ptr [rip + 0x285ce9d]
0x1400dddd3: movss xmm8, dword ptr [rip + 0x2847a94]
0x1400ddddc: movaps xmm9, xmmword ptr [rip + 0x285ce6c]
0x1400ddde4: movaps xmm10, xmmword ptr [rip + 0x285ce74]
0x1400dddec: movdqa xmm11, xmmword ptr [rip + 0x285ce8b]
0x1400dddf5: jmp 0x1400dde0e
0x1400dddf7: nop word ptr [rax + rax]
0x1400dde00: mov rax, qword ptr [rbp + 0x9a0]
0x1400dde07: mov rcx, qword ptr [rbp + 0x9a8]
0x1400dde0e: cmp rax, rcx
0x1400dde11: je 0x1400de874
0x1400dde17: lea rcx, [rax + 0x58]
0x1400dde1b: mov qword ptr [rbp + 0x9a0], rcx
0x1400dde22: mov r13, qword ptr [rax]
0x1400dde25: mov rcx, qword ptr [rax + 8]
0x1400dde29: mov qword ptr [rbp + 0x9e0], rcx
0x1400dde30: mov rcx, qword ptr [rax + 0x50]
0x1400dde34: mov qword ptr [rbp + 0x990], rcx
0x1400dde3b: movups xmm0, xmmword ptr [rax + 0x40]
0x1400dde3f: movaps xmmword ptr [rbp + 0x980], xmm0
0x1400dde46: movups xmm0, xmmword ptr [rax + 0x30]
0x1400dde4a: movaps xmmword ptr [rbp + 0x970], xmm0
0x1400dde51: movups xmm0, xmmword ptr [rax + 0x20]
0x1400dde55: movaps xmmword ptr [rbp + 0x960], xmm0
0x1400dde5c: movups xmm0, xmmword ptr [rax + 0x10]
0x1400dde60: movaps xmmword ptr [rbp + 0x950], xmm0
0x1400dde67: cmp r13, 2
0x1400dde6b: je 0x1400de874
0x1400dde71: mov rax, qword ptr [rbp + 0x990]
0x1400dde78: mov qword ptr [rbp + 0x800], rax
0x1400dde7f: movdqa xmm0, xmmword ptr [rbp + 0x950]
0x1400dde87: movaps xmm1, xmmword ptr [rbp + 0x960]
0x1400dde8e: movaps xmm2, xmmword ptr [rbp + 0x970]
0x1400dde95: movaps xmm3, xmmword ptr [rbp + 0x980]
0x1400dde9c: movaps xmmword ptr [rbp + 0x7f0], xmm3
0x1400ddea3: movaps xmmword ptr [rbp + 0x7e0], xmm2
0x1400ddeaa: movaps xmmword ptr [rbp + 0x7d0], xmm1
0x1400ddeb1: movdqa xmmword ptr [rbp + 0x7c0], xmm0
0x1400ddeb9: call 0x140c16a00
0x1400ddebe: test al, al
0x1400ddec0: je 0x1400ddf00
0x1400ddec2: mov ecx, 0x36
0x1400ddec7: call 0x1400dcfc0
0x1400ddecc: mov esi, 0x36
0x1400dded1: cmp rax, 1
0x1400dded5: jne 0x1400ddf10
0x1400dded7: mov r9, rdx
0x1400ddeda: test rdx, rdx
0x1400ddedd: je 0x1400ddf1f
0x1400ddedf: cmp r9, 0x36
0x1400ddee3: jae 0x1400ddf19
0x1400ddee5: lea rax, [rip + 0x285cf0f]
0x1400ddeec: cmp byte ptr [r9 + rax], 0xbf
0x1400ddef1: jg 0x1400ddf1f
0x1400ddef3: jmp 0x1400df699
0x1400ddef8: nop dword ptr [rax + rax]
0x1400ddf00: xor r8d, r8d
0x1400ddf03: jmp 0x1400de0cb
0x1400ddf08: nop dword ptr [rax + rax]
0x1400ddf10: lea r12, [rip + 0x285cee4]
0x1400ddf17: jmp 0x1400ddf6e
0x1400ddf19: jne 0x1400df699
0x1400ddf1f: mov rcx, r9
0x1400ddf22: call 0x1400dcfc0
0x1400ddf27: lea r12, [rip + 0x285cecd]
0x1400ddf2e: cmp rax, 1
0x1400ddf32: jne 0x1400ddf6e
0x1400ddf34: mov r12, rdx
0x1400ddf37: add r12, 2
0x1400ddf3b: je 0x1400ddf5c
0x1400ddf3d: cmp r12, 0x36
0x1400ddf41: jae 0x1400ddf56
0x1400ddf43: lea rax, [rip + 0x285ceb1]
0x1400ddf4a: cmp byte ptr [r12 + rax], 0xbf
0x1400ddf4f: jg 0x1400ddf5c
0x1400ddf51: jmp 0x1400df6be
0x1400ddf56: jne 0x1400df6be
0x1400ddf5c: mov esi, 0x34
0x1400ddf61: sub rsi, rdx
0x1400ddf64: lea rax, [rip + 0x285ce90]
0x1400ddf6b: add r12, rax
0x1400ddf6e: lea r14, [rip + 0x285d1c0]
0x1400ddf75: nop word ptr cs:[rax + rax]
0x1400ddf80: movsx eax, byte ptr [r14 - 1]
0x1400ddf85: test eax, eax
0x1400ddf87: js 0x1400ddfa0
0x1400ddf89: dec r14
0x1400ddf8c: cmp eax, 0x5c
0x1400ddf8f: jne 0x1400ddff4
0x1400ddf91: jmp 0x1400de010
0x1400ddf93: nop word ptr cs:[rax + rax]
0x1400ddfa0: movzx ecx, byte ptr [r14 - 2]
0x1400ddfa5: cmp cl, 0xc0
0x1400ddfa8: jge 0x1400ddfcd
0x1400ddfaa: movzx edx, byte ptr [r14 - 3]
0x1400ddfaf: cmp dl, 0xc0
0x1400ddfb2: jge 0x1400ddfd6
0x1400ddfb4: movzx r8d, byte ptr [r14 - 4]
0x1400ddfb9: add r14, -4
0x1400ddfbd: and r8d, 7
0x1400ddfc1: shl r8d, 6
0x1400ddfc5: and edx, 0x3f
0x1400ddfc8: or edx, r8d
0x1400ddfcb: jmp 0x1400ddfdd
0x1400ddfcd: add r14, -2
0x1400ddfd1: and ecx, 0x1f
0x1400ddfd4: jmp 0x1400ddfe5
0x1400ddfd6: add r14, -3
0x1400ddfda: and edx, 0xf
0x1400ddfdd: shl edx, 6
0x1400ddfe0: and ecx, 0x3f
0x1400ddfe3: or ecx, edx
0x1400ddfe5: shl ecx, 6
0x1400ddfe8: and al, 0x3f
0x1400ddfea: movzx eax, al
0x1400ddfed: or eax, ecx
0x1400ddfef: cmp eax, 0x5c
0x1400ddff2: je 0x1400de010
0x1400ddff4: cmp eax, 0x2f
0x1400ddff7: je 0x1400de010
0x1400ddff9: cmp r14, r15
0x1400ddffc: jne 0x1400ddf80
0x1400ddffe: mov ebx, 0x25
0x1400de003: mov r14, r15
0x1400de006: jmp 0x1400de03b
0x1400de008: nop dword ptr [rax + rax]
0x1400de010: sub r14, r15
0x1400de013: inc r14
0x1400de016: je 0x1400de030
0x1400de018: cmp r14, 0x25
0x1400de01c: jae 0x1400de02a
0x1400de01e: cmp byte ptr [r14 + r15], 0xbf
0x1400de023: jg 0x1400de030
0x1400de025: jmp 0x1400df66e
0x1400de02a: jne 0x1400df66e
0x1400de030: mov ebx, 0x25
0x1400de035: sub rbx, r14
0x1400de038: add r14, r15
0x1400de03b: call 0x140c18130
0x1400de040: mov rcx, rax
0x1400de043: mov rax, qword ptr [rax]
0x1400de046: test rax, rax
0x1400de049: je 0x1400de060
0x1400de04b: cmp eax, 1
0x1400de04e: jne 0x1400df577
0x1400de054: add rcx, 8
0x1400de058: jmp 0x1400de073
0x1400de05a: nop word ptr [rax + rax]
0x1400de060: xor edx, edx
0x1400de062: call 0x1428a1640
0x1400de067: mov rcx, rax
0x1400de06a: test rax, rax
0x1400de06d: je 0x1400df577
0x1400de073: cmp qword ptr [rcx], 0
0x1400de077: jne 0x1400df42f
0x1400de07d: mov qword ptr [rcx], 0xffffffffffffffff
0x1400de084: mov qword ptr [rbp + 0xa60], rcx
0x1400de08b: add rcx, 8
0x1400de08f: mov qword ptr [rsp + 0x20], rbx
0x1400de094: mov qword ptr [rsp + 0x30], 0
0x1400de09d: mov qword ptr [rsp + 0x28], 1
0x1400de0a6: mov rdx, r12
0x1400de0a9: mov r8, rsi
0x1400de0ac: mov r9, r14
0x1400de0af: call 0x140c16d40
0x1400de0b4: mov rcx, qword ptr [rbp + 0xa60]
0x1400de0bb: inc qword ptr [rcx]
0x1400de0be: mov qword ptr [rbp + 0x918], rax
0x1400de0c5: mov r8d, 1
0x1400de0cb: mov qword ptr [rbp + 0x910], r8
0x1400de0d2: mov r15, qword ptr [rbp + 0x7e0]
0x1400de0d9: lea rax, [r15 + 0x28]
0x1400de0dd: lea rcx, [r15 + 0x30]
0x1400de0e1: xor edx, edx
0x1400de0e3: cmp rdx, qword ptr [rbp + 0x7d8]
0x1400de0ea: lea rdx, [rbp + 0x7f0]
0x1400de0f1: cmovno rax, rdx
0x1400de0f5: mov rdi, qword ptr [rax]
0x1400de0f8: lea rax, [rbp + 0x7f8]
0x1400de0ff: cmovno rcx, rax
0x1400de103: mov rbx, qword ptr [rcx]
0x1400de106: mov qword ptr [rbp + 0x9e8], r8
0x1400de10d: jno 0x1400de130
0x1400de10f: mov r12, qword ptr [r15 + 0x18]
0x1400de113: mov r14, qword ptr [r15 + 0x20]
0x1400de117: xor r11d, r11d
0x1400de11a: mov rax, qword ptr [rbp + 0xa58]
0x1400de121: mov rcx, qword ptr [rbp + 0xa68]
0x1400de128: jmp 0x1400de2bc
0x1400de12d: nop dword ptr [rax]
0x1400de130: mov r14, qword ptr [rbp + 0x7e8]
0x1400de137: mov r11b, 1
0x1400de13a: test r14, r14
0x1400de13d: je 0x1400de18c
0x1400de13f: lea rsi, [r14*4]
0x1400de147: movzx eax, byte ptr [rip + 0x3556abb]
0x1400de14e: mov edx, 1
0x1400de153: mov rcx, rsi
0x1400de156: call 0x140613c10
0x1400de15b: test rax, rax
0x1400de15e: je 0x1400df5f1
0x1400de164: mov r12, rax
0x1400de167: cmp r14, 4
0x1400de16b: setb al
0x1400de16e: mov rcx, r12
0x1400de171: sub rcx, r15
0x1400de174: cmp rcx, 0x10
0x1400de178: setb cl
0x1400de17b: or cl, al
0x1400de17d: cmp cl, 1
0x1400de180: jne 0x1400de1a1
0x1400de182: xor eax, eax
0x1400de184: mov r11b, 1
0x1400de187: jmp 0x1400de280
0x1400de18c: mov eax, 1
0x1400de191: xor ecx, ecx
0x1400de193: xor r14d, r14d
0x1400de196: mov r12d, 1
0x1400de19c: jmp 0x1400de2bc
0x1400de1a1: mov rax, r14
0x1400de1a4: and rax, 0xfffffffffffffffc
0x1400de1a8: xor ecx, ecx
0x1400de1aa: mov r11b, 1
0x1400de1ad: nop dword ptr [rax]
0x1400de1b0: movups xmm0, xmmword ptr [r15 + rcx*4]
0x1400de1b5: mulps xmm0, xmm9
0x1400de1b9: addps xmm0, xmm10
0x1400de1bd: movaps xmm1, xmm0
0x1400de1c0: movaps xmm2, xmm0
0x1400de1c3: shufps xmm2, xmm0, 0xff
0x1400de1c7: xorps xmm3, xmm3
0x1400de1ca: maxss xmm3, xmm2
0x1400de1ce: movaps xmm2, xmm7
0x1400de1d1: minss xmm2, xmm3
0x1400de1d5: cvttss2si edx, xmm2
0x1400de1d9: xorps xmm2, xmm2
0x1400de1dc: maxss xmm2, xmm0
0x1400de1e0: unpckhpd xmm1, xmm0
0x1400de1e4: xorps xmm3, xmm3
0x1400de1e7: maxss xmm3, xmm1
0x1400de1eb: movaps xmm1, xmm7
0x1400de1ee: minss xmm1, xmm3
0x1400de1f2: cvttss2si r8d, xmm1
0x1400de1f7: movaps xmm1, xmm7
0x1400de1fa: minss xmm1, xmm2
0x1400de1fe: cvttss2si r9d, xmm1
0x1400de203: movshdup xmm0, xmm0
0x1400de207: xorps xmm1, xmm1
0x1400de20a: maxss xmm1, xmm0
0x1400de20e: movaps xmm0, xmm7
0x1400de211: minss xmm0, xmm1
0x1400de215: cvttss2si r10d, xmm0
0x1400de21a: movzx r10d, r10b
0x1400de21e: movzx r9d, r9b
0x1400de222: movd xmm0, r9d
0x1400de227: pinsrw xmm0, r10d, 2
0x1400de22d: movzx r8d, r8b
0x1400de231: pinsrw xmm0, r8d, 4
0x1400de237: movzx edx, dl
0x1400de23a: movdqa xmm1, xmm0
0x1400de23e: pinsrw xmm1, edx, 6
0x1400de243: pmuludq xmm0, xmm11
0x1400de248: pshufd xmm0, xmm0, 0xe8
0x1400de24d: pshufd xmm1, xmm1, 0xf5
0x1400de252: pmuludq xmm1, xmm11
0x1400de257: pshufd xmm1, xmm1, 0xe8
0x1400de25c: punpckldq xmm0, xmm1
0x1400de260: movdqu xmmword ptr [r12 + rcx*4], xmm0
0x1400de266: add rcx, 4
0x1400de26a: cmp rax, rcx
0x1400de26d: jne 0x1400de1b0
0x1400de273: jmp 0x1400de2b1
0x1400de275: nop word ptr cs:[rax + rax]
0x1400de280: movss xmm0, dword ptr [r15 + rax*4]
0x1400de286: mulss xmm0, xmm7
0x1400de28a: addss xmm0, xmm8
0x1400de28f: xorps xmm1, xmm1
0x1400de292: maxss xmm1, xmm0
0x1400de296: movaps xmm0, xmm7
0x1400de299: minss xmm0, xmm1
0x1400de29d: cvttss2si ecx, xmm0
0x1400de2a1: movzx ecx, cl
0x1400de2a4: imul ecx, ecx, 0x1010101
0x1400de2aa: mov dword ptr [r12 + rax*4], ecx
0x1400de2ae: inc rax
0x1400de2b1: cmp r14, rax
0x1400de2b4: jne 0x1400de280
0x1400de2b6: mov rax, r12
0x1400de2b9: mov rcx, r14
0x1400de2bc: mov qword ptr [rbp + 0x3b0], r13
0x1400de2c3: mov rdx, qword ptr [rbp + 0x9e0]
0x1400de2ca: mov qword ptr [rbp + 0x3b8], rdx
0x1400de2d1: mov qword ptr [rbp + 0xa58], rax
0x1400de2d8: mov qword ptr [rbp + 0xa48], rax
0x1400de2df: mov qword ptr [rbp + 0xa68], rcx
0x1400de2e6: mov qword ptr [rbp + 0xa40], rcx
0x1400de2ed: mov dword ptr [rbp + 0xa60], r11d
0x1400de2f4: mov byte ptr [rbp + 0xa75], r11b
0x1400de2fb: mov rcx, qword ptr [rbp + 0x9f0]
0x1400de302: lea rdx, [rbp + 0x3b0]
0x1400de309: call 0x1405b1d60
0x1400de30e: mov rsi, rax
0x1400de311: mov r15, rax
0x1400de314: shr r15, 0x39
0x1400de318: mov rcx, qword ptr [rbp + 0xa50]
0x1400de31f: mov rax, qword ptr [rcx + 0x78]
0x1400de323: mov rcx, qword ptr [rcx + 0x80]
0x1400de32a: mov rdx, rcx
0x1400de32d: and rdx, rsi
0x1400de330: movd xmm0, r15d
0x1400de335: punpcklbw xmm0, xmm0
0x1400de339: pshuflw xmm0, xmm0, 0
0x1400de33e: pshufd xmm0, xmm0, 0
0x1400de343: xor r8d, r8d
0x1400de346: movdqu xmm1, xmmword ptr [rax + rdx]
0x1400de34b: movdqa xmm2, xmm1
0x1400de34f: pcmpeqb xmm2, xmm0
0x1400de353: pmovmskb r9d, xmm2
0x1400de358: test r9d, r9d
0x1400de35b: je 0x1400de3a0
0x1400de35d: tzcnt r10d, r9d
0x1400de362: add r10, rdx
0x1400de365: and r10, rcx
0x1400de368: neg r10
0x1400de36b: lea r10, [r10 + r10*8]
0x1400de36f: cmp qword ptr [rax + r10*8 - 0x48], r13
0x1400de374: jne 0x1400de384
0x1400de376: mov r11, qword ptr [rbp + 0x9e0]
0x1400de37d: cmp qword ptr [rax + r10*8 - 0x40], r11
0x1400de382: je 0x1400de3d0
0x1400de384: lea r10d, [r9 - 1]
0x1400de388: and r10w, r9w
0x1400de38c: mov r9d, r10d
0x1400de38f: jne 0x1400de35d
0x1400de391: nop word ptr cs:[rax + rax]
0x1400de3a0: pcmpeqb xmm1, xmm6
0x1400de3a4: pmovmskb r9d, xmm1
0x1400de3a9: test r9d, r9d
0x1400de3ac: jne 0x1400de4a0
0x1400de3b2: add rdx, r8
0x1400de3b5: add rdx, 0x10
0x1400de3b9: add r8, 0x10
0x1400de3bd: and rdx, rcx
0x1400de3c0: jmp 0x1400de346
0x1400de3c2: nop word ptr cs:[rax + rax]
0x1400de3d0: xor ecx, ecx
0x1400de3d2: test byte ptr [rbp + 0x7c0], 1
0x1400de3d9: mov edx, 0
0x1400de3de: je 0x1400de3ec
0x1400de3e0: mov ecx, dword ptr [rbp + 0x7c8]
0x1400de3e6: mov edx, dword ptr [rbp + 0x7d0]
0x1400de3ec: lea r8, [rax + r10*8]
0x1400de3f0: shl r14, 2
0x1400de3f4: add r8, -0x38
0x1400de3f8: mov qword ptr [rbp + 0xa00], r12
0x1400de3ff: mov qword ptr [rbp + 0xa08], r14
0x1400de406: mov dword ptr [rbp + 0xa10], ecx
0x1400de40c: mov dword ptr [rbp + 0xa14], edx
0x1400de412: mov dword ptr [rbp + 0xa18], 0
0x1400de41c: mov dword ptr [rbp + 0xa1c], edi
0x1400de422: mov dword ptr [rbp + 0xa20], ebx
0x1400de428: mov qword ptr [rbp + 0xa24], 1
0x1400de433: mov dword ptr [rbp + 0xa2c], 0
0x1400de43d: mov rax, qword ptr [rbp + 0xa58]
0x1400de444: mov qword ptr [rbp + 0xa48], rax
0x1400de44b: mov rax, qword ptr [rbp + 0xa68]
0x1400de452: mov qword ptr [rbp + 0xa40], rax
0x1400de459: mov eax, dword ptr [rbp + 0xa60]
0x1400de45f: mov byte ptr [rbp + 0xa75], al
0x1400de465: mov byte ptr [rsp + 0x28], 0x22
0x1400de46a: mov qword ptr [rsp + 0x20], 1
0x1400de473: lea rcx, [rbp - 0x38]
0x1400de477: mov rdx, qword ptr [rbp + 0x9f8]
0x1400de47e: lea r9, [rbp + 0xa00]
0x1400de485: call 0x140bfe9c0
0x1400de48a: cmp dword ptr [rbp - 0x38], 0xb
0x1400de48e: je 0x1400de6d0
0x1400de494: jmp 0x1400df4fd
0x1400de499: nop dword ptr [rax]
0x1400de4a0: mov rax, qword ptr [rbp + 0xa50]
0x1400de4a7: cmp qword ptr [rax + 0x88], 0
0x1400de4af: je 0x1400de7e1
0x1400de4b5: mov qword ptr [rbp + 0xa00], r14
0x1400de4bc: imul ebx, edi
0x1400de4bf: mov qword ptr [rbp - 0x38], rbx
0x1400de4c3: cmp r14, rbx
0x1400de4c6: jne 0x1400df440
0x1400de4cc: mov rax, qword ptr [rbp + 0x7e0]
0x1400de4d3: lea rcx, [rax + 0x28]
0x1400de4d7: add rax, 0x30
0x1400de4db: xor edx, edx
0x1400de4dd: cmp rdx, qword ptr [rbp + 0x7d8]
0x1400de4e4: lea rdx, [rbp + 0x7f0]
0x1400de4eb: cmovno rcx, rdx
0x1400de4ef: mov ecx, dword ptr [rcx]
0x1400de4f1: lea rdx, [rbp + 0x7f8]
0x1400de4f8: cmovno rax, rdx
0x1400de4fc: mov rdx, qword ptr [rax]
0x1400de4ff: shl rdx, 0x20
0x1400de503: or rdx, rcx
0x1400de506: mov rax, qword ptr [rbp + 0xa58]
0x1400de50d: mov qword ptr [rbp + 0xa48], rax
0x1400de514: mov rax, qword ptr [rbp + 0xa68]
0x1400de51b: mov qword ptr [rbp + 0xa40], rax
0x1400de522: mov eax, dword ptr [rbp + 0xa60]
0x1400de528: mov byte ptr [rbp + 0xa75], al
0x1400de52e: lea rcx, [rbp + 0x3b0]
0x1400de535: mov r8d, 0x2b
0x1400de53b: call 0x140bd52a0
0x1400de540: mov rax, qword ptr [rbp + 0xa58]
0x1400de547: mov qword ptr [rbp + 0xa48], rax
0x1400de54e: mov rax, qword ptr [rbp + 0xa68]
0x1400de555: mov qword ptr [rbp + 0xa40], rax
0x1400de55c: mov eax, dword ptr [rbp + 0xa60]
0x1400de562: mov byte ptr [rbp + 0xa75], al
0x1400de568: lea rcx, [rbp + 0xa00]
0x1400de56f: lea rdx, [rbp + 0x3b0]
0x1400de576: call 0x140bd5370
0x1400de57b: shl r14, 2
0x1400de57f: mov rax, qword ptr [rbp + 0xa58]
0x1400de586: mov qword ptr [rbp + 0xa48], rax
0x1400de58d: mov rax, qword ptr [rbp + 0xa68]
0x1400de594: mov qword ptr [rbp + 0xa40], rax
0x1400de59b: mov eax, dword ptr [rbp + 0xa60]
0x1400de5a1: mov byte ptr [rbp + 0xa75], al
0x1400de5a7: mov qword ptr [rsp + 0x30], r14
0x1400de5ac: mov qword ptr [rsp + 0x28], r12
0x1400de5b1: lea rax, [rbp + 0xa00]
0x1400de5b8: mov qword ptr [rsp + 0x20], rax
0x1400de5bd: mov r9d, 0xc
0x1400de5c3: lea rcx, [rbp - 0x38]
0x1400de5c7: mov rdx, qword ptr [rbp + 0x9f8]
0x1400de5ce: lea r8, [rip + 0x285cce3]
0x1400de5d5: call 0x1405b9b00
0x1400de5da: cmp dword ptr [rbp - 0x38], 0xb
0x1400de5de: jne 0x1400df588
0x1400de5e4: lea rcx, [rbp - 0x30]
0x1400de5e8: mov rax, qword ptr [rcx + 0x30]
0x1400de5ec: mov qword ptr [rbp + 0x3e0], rax
0x1400de5f3: movups xmm0, xmmword ptr [rcx]
0x1400de5f6: movdqu xmm1, xmmword ptr [rcx + 0x10]
0x1400de5fb: movdqu xmm2, xmmword ptr [rcx + 0x20]
0x1400de600: movdqa xmmword ptr [rbp + 0x3d0], xmm2
0x1400de608: movdqa xmmword ptr [rbp + 0x3c0], xmm1
0x1400de610: movaps xmmword ptr [rbp + 0x3b0], xmm0
0x1400de617: mov rcx, qword ptr [rbp + 0xa50]
0x1400de61e: mov rax, qword ptr [rcx + 0x78]
0x1400de622: mov rcx, qword ptr [rcx + 0x80]
0x1400de629: and rsi, rcx
0x1400de62c: movdqu xmm0, xmmword ptr [rax + rsi]
0x1400de631: pmovmskb edx, xmm0
0x1400de635: test edx, edx
0x1400de637: je 0x1400de83b
0x1400de63d: tzcnt edx, edx
0x1400de641: add rdx, rsi
0x1400de644: and rdx, rcx
0x1400de647: movzx r8d, byte ptr [rax + rdx]
0x1400de64c: test r8b, r8b
0x1400de64f: jns 0x1400de85e
0x1400de655: lea r9, [rdx - 0x10]
0x1400de659: and r9, rcx
0x1400de65c: mov byte ptr [rax + rdx], r15b
0x1400de660: mov byte ptr [rax + r9 + 0x10], r15b
0x1400de665: neg rdx
0x1400de668: lea rcx, [rdx + rdx*8]
0x1400de66c: and r8b, 1
0x1400de670: movzx edx, r8b
0x1400de674: mov r8, qword ptr [rbp + 0xa50]
0x1400de67b: sub qword ptr [r8 + 0x88], rdx
0x1400de682: mov qword ptr [rax + rcx*8 - 0x48], r13
0x1400de687: mov rdx, qword ptr [rbp + 0x9e0]
0x1400de68e: mov qword ptr [rax + rcx*8 - 0x40], rdx
0x1400de693: movdqa xmm0, xmmword ptr [rbp + 0x3b0]
0x1400de69b: movdqa xmm1, xmmword ptr [rbp + 0x3c0]
0x1400de6a3: movdqa xmm2, xmmword ptr [rbp + 0x3d0]
0x1400de6ab: movdqu xmmword ptr [rax + rcx*8 - 0x38], xmm0
0x1400de6b1: movdqu xmmword ptr [rax + rcx*8 - 0x28], xmm1
0x1400de6b7: movdqu xmmword ptr [rax + rcx*8 - 0x18], xmm2
0x1400de6bd: mov rdx, qword ptr [rbp + 0x3e0]
0x1400de6c4: mov qword ptr [rax + rcx*8 - 8], rdx
0x1400de6c9: inc qword ptr [r8 + 0x90]
0x1400de6d0: lea r15, [rip + 0x285ca39]
0x1400de6d7: mov rcx, qword ptr [rbp + 0x9e8]
0x1400de6de: mov edx, dword ptr [rbp + 0xa60]
0x1400de6e4: cmp qword ptr [rbp + 0xa68], 0
0x1400de6ec: setne al
0x1400de6ef: test dl, al
0x1400de6f1: je 0x1400de71b
0x1400de6f3: mov rax, qword ptr [rbp + 0xa68]
0x1400de6fa: lea rdx, [rax*4]
0x1400de702: mov r8d, 1
0x1400de708: mov rcx, qword ptr [rbp + 0xa58]
0x1400de70f: call 0x140613c20
0x1400de714: mov rcx, qword ptr [rbp + 0x910]
0x1400de71b: test rcx, rcx
0x1400de71e: je 0x1400de785
0x1400de720: call 0x140c18130
0x1400de725: mov rcx, rax
0x1400de728: mov rax, qword ptr [rax]
0x1400de72b: test rax, rax
0x1400de72e: je 0x1400de740
0x1400de730: cmp eax, 1
0x1400de733: jne 0x1400df566
0x1400de739: add rcx, 8
0x1400de73d: jmp 0x1400de753
0x1400de73f: nop 
0x1400de740: xor edx, edx
0x1400de742: call 0x1428a1640
0x1400de747: mov rcx, rax
0x1400de74a: test rax, rax
0x1400de74d: je 0x1400df566
0x1400de753: cmp qword ptr [rcx], 0
0x1400de757: jne 0x1400df41e
0x1400de75d: mov qword ptr [rcx], 0xffffffffffffffff
0x1400de764: mov qword ptr [rbp + 0xa60], rcx
0x1400de76b: add rcx, 8
0x1400de76f: mov rdx, qword ptr [rbp + 0x918]
0x1400de776: call 0x140c170f0
0x1400de77b: mov rax, qword ptr [rbp + 0xa60]
0x1400de782: inc qword ptr [rax]
0x1400de785: mov rdx, qword ptr [rbp + 0x7d8]
0x1400de78c: mov rax, rdx
0x1400de78f: neg rax
0x1400de792: jno 0x1400de7c0
0x1400de794: mov rax, qword ptr [rbp + 0x7e0]
0x1400de79b: lock dec qword ptr [rax]
0x1400de79f: jne 0x1400dde00
0x1400de7a5: lea rcx, [rbp + 0x7e0]
0x1400de7ac: call 0x140c7de40
0x1400de7b1: jmp 0x1400dde00
0x1400de7b6: nop word ptr cs:[rax + rax]
0x1400de7c0: jae 0x1400dde00
0x1400de7c6: mov rcx, qword ptr [rbp + 0x7e0]
0x1400de7cd: shl rdx, 2
0x1400de7d1: mov r8d, 4
0x1400de7d7: call 0x140613c20
0x1400de7dc: jmp 0x1400dde00
0x1400de7e1: mov rax, qword ptr [rbp + 0xa58]
0x1400de7e8: mov qword ptr [rbp + 0xa48], rax
0x1400de7ef: mov rax, qword ptr [rbp + 0xa68]
0x1400de7f6: mov qword ptr [rbp + 0xa40], rax
0x1400de7fd: mov eax, dword ptr [rbp + 0xa60]
0x1400de803: mov byte ptr [rbp + 0xa75], al
0x1400de809: mov edx, 1
0x1400de80e: mov rcx, qword ptr [rbp + 0x948]
0x1400de815: mov r8, qword ptr [rbp + 0x9f0]
0x1400de81c: call 0x14289e380
0x1400de821: mov r13, qword ptr [rbp + 0x3b0]
0x1400de828: mov rax, qword ptr [rbp + 0x3b8]
0x1400de82f: mov qword ptr [rbp + 0x9e0], rax
0x1400de836: jmp 0x1400de4b5
0x1400de83b: mov r8d, 0x10
0x1400de841: add rsi, r8
0x1400de844: and rsi, rcx
0x1400de847: movdqu xmm0, xmmword ptr [rax + rsi]
0x1400de84c: pmovmskb edx, xmm0
0x1400de850: add r8, 0x10
0x1400de854: test edx, edx
0x1400de856: jne 0x1400de63d
0x1400de85c: jmp 0x1400de841
0x1400de85e: movdqa xmm0, xmmword ptr [rax]
0x1400de862: pmovmskb edx, xmm0
0x1400de866: tzcnt edx, edx
0x1400de86a: movzx r8d, byte ptr [rax + rdx]
0x1400de86f: jmp 0x1400de655
0x1400de874: lea rcx, [rbp + 0x9a0]
0x1400de87b: call 0x1405d21b0
0x1400de880: mov rsi, qword ptr [rbp + 0x7a8]
0x1400de887: lea r15, [rsi + 0x38]
0x1400de88b: movabs rdx, 0x8597ee0727b461b3
0x1400de895: movabs r8, 0x8b297ec5ae15f15c
0x1400de89f: mov rcx, r15
0x1400de8a2: call 0x1400dd6f0
0x1400de8a7: test rax, rax
0x1400de8aa: je 0x1400df64a
0x1400de8b0: cmp qword ptr [rsi + 0x218], rdx
0x1400de8b7: jbe 0x1400df64a
0x1400de8bd: mov rax, qword ptr [rsi + 0x210]
0x1400de8c4: mov rcx, qword ptr [rax + rdx*8]
0x1400de8c8: test rcx, rcx
0x1400de8cb: je 0x1400df64a
0x1400de8d1: mov rax, qword ptr [rsi + 0x1e0]
0x1400de8d8: not rcx
0x1400de8db: lea rcx, [rcx + rcx*2]
0x1400de8df: shl rcx, 5
0x1400de8e3: cmp qword ptr [rax + rcx + 0x20], 0
0x1400de8e9: je 0x1400df64a
0x1400de8ef: add rax, rcx
0x1400de8f2: mov rcx, qword ptr [rax + 0x10]
0x1400de8f6: lea rdx, [rip + 0x285c853]
0x1400de8fd: mov r8d, 4
0x1400de903: call 0x140937aa0
0x1400de908: mov qword ptr [rbp + 0xa68], rdx
0x1400de90f: test al, 1
0x1400de911: mov rdi, qword ptr [rbp + 0x7b0]
0x1400de918: mov rbx, qword ptr [rbp + 0x7b8]
0x1400de91f: je 0x1400df7f2
0x1400de925: lea rcx, [rbx + 4]
0x1400de929: call 0x140bd5790
0x1400de92e: mov r14, rax
0x1400de931: mov dword ptr [rbp + 0x7c0], 0
0x1400de93b: mov qword ptr [rbp + 0x7c8], rbx
0x1400de942: lea rcx, [rbp + 0x7c0]
0x1400de949: call 0x1409092f0
0x1400de94e: mov rcx, rax
0x1400de951: call 0x140bd5790
0x1400de956: movups xmm0, xmmword ptr [rbp + 0x7c0]
0x1400de95d: movdqu xmm1, xmmword ptr [rbp + 0x7d0]
0x1400de965: movdqa xmmword ptr [rbp + 0x3c0], xmm1
0x1400de96d: movaps xmmword ptr [rbp + 0x3b0], xmm0
0x1400de974: mov dword ptr [rbp + 0x950], 2
0x1400de97e: mov qword ptr [rsp + 0x40], rdi
0x1400de983: mov qword ptr [rsp + 0x38], rsi
0x1400de988: mov qword ptr [rsp + 0x30], rax
0x1400de98d: lea rax, [rbp + 0x950]
0x1400de994: mov qword ptr [rsp + 0x28], rax
0x1400de999: mov qword ptr [rsp + 0x20], 1
0x1400de9a2: lea rdx, [rip + 0x285c7a7]
0x1400de9a9: lea rcx, [rbp - 0x38]
0x1400de9ad: lea r9, [rbp + 0x3b0]
0x1400de9b4: mov r8d, 4
0x1400de9ba: call 0x140867970
0x1400de9bf: mov byte ptr [rbp + 0xa74], 1
0x1400de9c6: movabs rdx, 0x905c303950d415d1
0x1400de9d0: movabs r8, 0x4667de267035c872
0x1400de9da: mov rcx, r15
0x1400de9dd: call 0x1400dd6f0
0x1400de9e2: test rax, rax
0x1400de9e5: je 0x1400df656
0x1400de9eb: cmp qword ptr [rsi + 0x218], rdx
0x1400de9f2: jbe 0x1400df656
0x1400de9f8: mov rax, qword ptr [rsi + 0x210]
0x1400de9ff: mov rcx, qword ptr [rax + rdx*8]
0x1400dea03: test rcx, rcx
0x1400dea06: je 0x1400df656
0x1400dea0c: mov rax, qword ptr [rsi + 0x1e0]
0x1400dea13: not rcx
0x1400dea16: lea rcx, [rcx + rcx*2]
0x1400dea1a: shl rcx, 5
0x1400dea1e: cmp qword ptr [rax + rcx + 0x20], 0
0x1400dea24: je 0x1400df656
0x1400dea2a: add rax, rcx
0x1400dea2d: mov r15, qword ptr [rax + 0x10]
0x1400dea31: mov byte ptr [rbp + 0xa74], 1
0x1400dea38: mov rcx, r15
0x1400dea3b: call 0x14067dd80
0x1400dea40: mov byte ptr [rbp + 0xa74], 1
0x1400dea47: mov rcx, rax
0x1400dea4a: call 0x140041e40
0x1400dea4f: movaps xmm6, xmm0
0x1400dea52: mov byte ptr [rbp + 0xa74], 1
0x1400dea59: mov rcx, r15
0x1400dea5c: call 0x14067dd80
0x1400dea61: mov rdx, qword ptr [rbp + 0xa50]
0x1400dea68: mov rcx, qword ptr [rdx + 0x10]
0x1400dea6c: mov qword ptr [rbp + 0x3c0], rcx
0x1400dea73: movdqu xmm0, xmmword ptr [rdx]
0x1400dea77: movdqa xmmword ptr [rbp + 0x3b0], xmm0
0x1400dea7f: mov qword ptr [rdx], 0
0x1400dea86: mov qword ptr [rdx + 8], 8
0x1400dea8e: mov qword ptr [rdx + 0x10], 0
0x1400dea96: mov byte ptr [rbp + 0xa74], 1
0x1400dea9d: lea rcx, [rbp + 0x7c0]
0x1400deaa4: lea r12, [rbp + 0x3b0]
0x1400deaab: mov rdx, rax
0x1400deaae: mov r8, r12
0x1400deab1: movaps xmm3, xmm6
0x1400deab4: call 0x140c691c0
0x1400deab9: mov rdx, r14
0x1400deabc: shr rdx, 0x20
0x1400deac0: mov r8, qword ptr [rbp + 0x7c0]
0x1400deac7: mov rax, qword ptr [rbp + 0x7c8]
0x1400deace: mov rcx, qword ptr [rbp + 0x7d0]
0x1400dead5: lea rcx, [rcx + rcx*4]
0x1400dead9: shl rcx, 4
0x1400deadd: add rcx, rax
0x1400deae0: mov qword ptr [rbp + 0x8f0], rax
0x1400deae7: mov qword ptr [rbp + 0x8f8], rax
0x1400deaee: mov qword ptr [rbp + 0x900], r8
0x1400deaf5: mov qword ptr [rbp + 0x908], rcx
0x1400deafc: lea rdi, [rbp + 0x7c8]
0x1400deb03: mov r8d, r14d
0x1400deb06: cvtsi2ss xmm7, r8
0x1400deb0b: mov edx, edx
0x1400deb0d: cvtsi2ss xmm8, rdx
0x1400deb12: lea r14, [rbp + 0xa00]
0x1400deb19: pcmpeqd xmm9, xmm9
0x1400deb1e: lea r13, [rbp + 0x7c0]
0x1400deb25: jmp 0x1400deb3e
0x1400deb27: nop word ptr [rax + rax]
0x1400deb30: mov rax, qword ptr [rbp + 0x8f8]
0x1400deb37: mov rcx, qword ptr [rbp + 0x908]
0x1400deb3e: cmp rax, rcx
0x1400deb41: je 0x1400df24a
0x1400deb47: lea rcx, [rax + 0x50]
0x1400deb4b: mov qword ptr [rbp + 0x8f8], rcx
0x1400deb52: mov rcx, qword ptr [rax]
0x1400deb55: mov rdx, qword ptr [rax + 0x48]
0x1400deb59: mov qword ptr [rbp + 0x990], rdx
0x1400deb60: movups xmm0, xmmword ptr [rax + 0x38]
0x1400deb64: movaps xmmword ptr [rbp + 0x980], xmm0
0x1400deb6b: movups xmm0, xmmword ptr [rax + 0x28]
0x1400deb6f: movaps xmmword ptr [rbp + 0x970], xmm0
0x1400deb76: movups xmm0, xmmword ptr [rax + 0x18]
0x1400deb7a: movaps xmmword ptr [rbp + 0x960], xmm0
0x1400deb81: movups xmm0, xmmword ptr [rax + 8]
0x1400deb85: movaps xmmword ptr [rbp + 0x950], xmm0
0x1400deb8c: cmp rcx, 3
0x1400deb90: je 0x1400df24a
0x1400deb96: mov qword ptr [rbp + 0x9a0], rcx
0x1400deb9d: mov rax, qword ptr [rbp + 0x980]
0x1400deba4: lea rdx, [rbp + 0x9a8]
0x1400debab: mov qword ptr [rdx + 0x30], rax
0x1400debaf: movaps xmm0, xmmword ptr [rbp + 0x950]
0x1400debb6: movaps xmm1, xmmword ptr [rbp + 0x960]
0x1400debbd: movaps xmm2, xmmword ptr [rbp + 0x970]
0x1400debc4: movups xmmword ptr [rdx + 0x20], xmm2
0x1400debc8: movups xmmword ptr [rdx + 0x10], xmm1
0x1400debcc: movups xmmword ptr [rdx], xmm0
0x1400debcf: cmp rcx, 2
0x1400debd3: je 0x1400df62d
0x1400debd9: mov rax, qword ptr [rbp + 0x9a0]
0x1400debe0: mov rcx, qword ptr [rbp + 0x9a8]
0x1400debe7: movdqu xmm0, xmmword ptr [rbp + 0x9b0]
0x1400debef: movdqu xmm1, xmmword ptr [rbp + 0x9c0]
0x1400debf7: movdqu xmm2, xmmword ptr [rbp + 0x9d0]
0x1400debff: movdqa xmmword ptr [rbp + 0xa30], xmm2
0x1400dec07: movdqa xmmword ptr [rbp + 0xa20], xmm1
0x1400dec0f: movdqa xmmword ptr [rbp + 0xa10], xmm0
0x1400dec17: mov qword ptr [rbp + 0xa00], rax
0x1400dec1e: mov qword ptr [rbp + 0xa08], rcx
0x1400dec25: cmp qword ptr [rbp + 0xa38], 0
0x1400dec2d: je 0x1400ded20
0x1400dec33: cmp qword ptr [rbp + 0xa20], 0
0x1400dec3b: je 0x1400ded20
0x1400dec41: mov rax, qword ptr [rbp + 0xa50]
0x1400dec48: cmp qword ptr [rax + 0x90], 0
0x1400dec50: je 0x1400df239
0x1400dec56: mov rcx, qword ptr [rbp + 0x9f0]
0x1400dec5d: mov rdx, r14
0x1400dec60: call 0x1405b1d60
0x1400dec65: mov rcx, qword ptr [rbp + 0xa50]
0x1400dec6c: mov rbx, qword ptr [rcx + 0x78]
0x1400dec70: mov rcx, qword ptr [rcx + 0x80]
0x1400dec77: mov rdx, rcx
0x1400dec7a: and rdx, rax
0x1400dec7d: shr rax, 0x39
0x1400dec81: movd xmm0, eax
0x1400dec85: punpcklbw xmm0, xmm0
0x1400dec89: pshuflw xmm0, xmm0, 0
0x1400dec8e: pshufd xmm0, xmm0, 0
0x1400dec93: lea rax, [rbx - 0x48]
0x1400dec97: mov r8, qword ptr [rbp + 0xa00]
0x1400dec9e: mov r9, qword ptr [rbp + 0xa08]
0x1400deca5: xor r10d, r10d
0x1400deca8: movdqu xmm1, xmmword ptr [rbx + rdx]
0x1400decad: movdqa xmm2, xmm1
0x1400decb1: pcmpeqb xmm2, xmm0
0x1400decb5: pmovmskb r11d, xmm2
0x1400decba: test r11d, r11d
0x1400decbd: je 0x1400decf0
0x1400decbf: tzcnt esi, r11d
0x1400decc4: add rsi, rdx
0x1400decc7: and rsi, rcx
0x1400decca: neg rsi
0x1400deccd: lea rsi, [rsi + rsi*8]
0x1400decd1: cmp r8, qword ptr [rax + rsi*8]
0x1400decd5: jne 0x1400dece2
0x1400decd7: cmp r9, qword ptr [rax + rsi*8 + 8]
0x1400decdc: je 0x1400deda0
0x1400dece2: lea esi, [r11 - 1]
0x1400dece6: and si, r11w
0x1400decea: mov r11d, esi
0x1400deced: jne 0x1400decbf
0x1400decef: nop 
0x1400decf0: pcmpeqb xmm1, xmm9
0x1400decf5: pmovmskb r11d, xmm1
0x1400decfa: test r11d, r11d
0x1400decfd: jne 0x1400df239
0x1400ded03: add rdx, r10
0x1400ded06: add rdx, 0x10
0x1400ded0a: add r10, 0x10
0x1400ded0e: and rdx, rcx
0x1400ded11: jmp 0x1400deca8
0x1400ded13: nop word ptr cs:[rax + rax]
0x1400ded20: mov rdx, qword ptr [rbp + 0xa10]
0x1400ded27: test rdx, rdx
0x1400ded2a: je 0x1400ded42
0x1400ded2c: mov rcx, qword ptr [rbp + 0xa18]
0x1400ded33: shl rdx, 2
0x1400ded37: mov r8d, 4
0x1400ded3d: call 0x140613c20
0x1400ded42: mov rax, qword ptr [rbp + 0xa28]
0x1400ded49: test rax, rax
0x1400ded4c: je 0x1400ded68
0x1400ded4e: mov rcx, qword ptr [rbp + 0xa30]
0x1400ded55: shl rax, 2
0x1400ded59: lea rdx, [rax + rax*4]
0x1400ded5d: mov r8d, 4
0x1400ded63: call 0x140613c20
0x1400ded68: cmp dword ptr [rbp + 0x9a0], 2
0x1400ded6f: jne 0x1400deb30
0x1400ded75: mov rax, qword ptr [rbp + 0x9a8]
0x1400ded7c: lock dec qword ptr [rax]
0x1400ded80: jne 0x1400deb30
0x1400ded86: lea rcx, [rbp + 0x9a8]
0x1400ded8d: call 0x14064f9d0
0x1400ded92: jmp 0x1400deb30
0x1400ded97: nop word ptr [rax + rax]
0x1400deda0: mov rdx, qword ptr [rbp + 0xa20]
0x1400deda7: shl rdx, 2
0x1400dedab: mov rcx, r12
0x1400dedae: mov r8d, 0x40
0x1400dedb4: call 0x140bd4e60
0x1400dedb9: mov qword ptr [rsp + 0x20], r12
0x1400dedbe: mov r9d, 0x11
0x1400dedc4: mov rcx, r13
0x1400dedc7: mov rdx, qword ptr [rbp + 0x9f8]
0x1400dedce: lea r8, [rip + 0x285c3cb]
0x1400dedd5: call 0x140beea80
0x1400dedda: cmp dword ptr [rbp + 0x7c0], 0xb
0x1400dede1: jne 0x1400df6e9
0x1400dede7: mov rax, qword ptr [rdi + 0x20]
0x1400dedeb: mov qword ptr [rbp + 0x8e0], rax
0x1400dedf2: movups xmm0, xmmword ptr [rdi]
0x1400dedf5: movups xmm1, xmmword ptr [rdi + 0x10]
0x1400dedf9: movaps xmmword ptr [rbp + 0x8d0], xmm1
0x1400dee00: movaps xmmword ptr [rbp + 0x8c0], xmm0
0x1400dee07: mov r8, qword ptr [rbp + 0x7f8]
0x1400dee0e: mov rdx, qword ptr [rbp + 0xa20]
0x1400dee15: shl rdx, 2
0x1400dee19: cmp r8, rdx
0x1400dee1c: jne 0x1400df49d
0x1400dee22: mov rcx, qword ptr [rbp + 0x7f0]
0x1400dee29: mov rdx, qword ptr [rbp + 0xa18]
0x1400dee30: call 0x142923c00
0x1400dee35: mov rax, qword ptr [rbp + 0xa38]
0x1400dee3c: shl rax, 2
0x1400dee40: lea rdx, [rax + rax*4]
0x1400dee44: mov byte ptr [rbp + 0xa73], 1
0x1400dee4b: mov rcx, r12
0x1400dee4e: mov r8d, 0x20
0x1400dee54: call 0x140bd4e60
0x1400dee59: mov byte ptr [rbp + 0xa73], 1
0x1400dee60: mov qword ptr [rsp + 0x20], r12
0x1400dee65: mov r9d, 0x12
0x1400dee6b: mov rcx, r13
0x1400dee6e: mov rdx, qword ptr [rbp + 0x9f8]
0x1400dee75: lea r8, [rip + 0x285c36c]
0x1400dee7c: call 0x140beea80
0x1400dee81: cmp dword ptr [rbp + 0x7c0], 0xb
0x1400dee88: jne 0x1400df765
0x1400dee8e: mov rax, qword ptr [rdi + 0x20]
0x1400dee92: mov qword ptr [rbp + 0x930], rax
0x1400dee99: movups xmm0, xmmword ptr [rdi]
0x1400dee9c: movups xmm1, xmmword ptr [rdi + 0x10]
0x1400deea0: movaps xmmword ptr [rbp + 0x920], xmm1
0x1400deea7: movaps xmmword ptr [rbp + 0x910], xmm0
0x1400deeae: mov r8, qword ptr [rbp + 0x7f8]
0x1400deeb5: mov rax, qword ptr [rbp + 0xa38]
0x1400deebc: shl rax, 2
0x1400deec0: lea rdx, [rax + rax*4]
0x1400deec4: cmp r8, rdx
0x1400deec7: jne 0x1400df4bb
0x1400deecd: mov rcx, qword ptr [rbp + 0x7f0]
0x1400deed4: mov rdx, qword ptr [rbp + 0xa30]
0x1400deedb: call 0x142923c00
0x1400deee0: mov byte ptr [rbp + 0xa77], 1
0x1400deee7: mov byte ptr [rbp + 0xa76], 1
0x1400deeee: mov rdi, r15
0x1400deef1: mov rcx, r15
0x1400deef4: call 0x14067dd80
0x1400deef9: mov byte ptr [rbp + 0xa77], 1
0x1400def00: mov byte ptr [rbp + 0xa76], 1
0x1400def07: mov rcx, rax
0x1400def0a: call 0x140041e40
0x1400def0f: movaps xmm6, xmm0
0x1400def12: mov byte ptr [rbp + 0xa77], 1
0x1400def19: mov byte ptr [rbp + 0xa76], 1
0x1400def20: mov rcx, r13
0x1400def23: mov rdx, qword ptr [rbp + 0xa68]
0x1400def2a: xor r8d, r8d
0x1400def2d: call 0x140867850
0x1400def32: mov byte ptr [rbp + 0xa77], 1
0x1400def39: mov byte ptr [rbp + 0xa76], 1
0x1400def40: mov rcx, r12
0x1400def43: mov rdx, r13
0x1400def46: lea r8, [rbp - 0x38]
0x1400def4a: xor r9d, r9d
0x1400def4d: call 0x140905be0
0x1400def52: mov r15, r14
0x1400def55: movzx eax, byte ptr [rip + 0x3555cad]
0x1400def5c: mov ecx, 8
0x1400def61: mov edx, 1
0x1400def66: call 0x140613c10
0x1400def6b: test rax, rax
0x1400def6e: je 0x1400df7da
0x1400def74: mov r14, rax
0x1400def77: movaps xmm0, xmm7
0x1400def7a: divss xmm0, xmm6
0x1400def7e: movaps xmm1, xmm8
0x1400def82: divss xmm1, xmm6
0x1400def86: movd eax, xmm1
0x1400def8a: shl rax, 0x20
0x1400def8e: movd ecx, xmm0
0x1400def92: or rcx, rax
0x1400def95: mov qword ptr [r14], rcx
0x1400def98: mov rdx, qword ptr [rbp + 0x3d8]
0x1400def9f: test rdx, rdx
0x1400defa2: je 0x1400defb6
0x1400defa4: mov rcx, qword ptr [rbp + 0x3e0]
0x1400defab: mov r8d, 1
0x1400defb1: call 0x140613c20
0x1400defb6: lea rsi, [rbx + rsi*8]
0x1400defba: add rsi, -0x38
0x1400defbe: mov qword ptr [rbp + 0x3d8], 8
0x1400defc9: mov qword ptr [rbp + 0x3e0], r14
0x1400defd0: mov qword ptr [rbp + 0x3e8], 8
0x1400defdb: mov r8d, 0xf8
0x1400defe1: mov rbx, r13
0x1400defe4: mov rcx, r13
0x1400defe7: mov r14, r12
0x1400defea: mov rdx, r12
0x1400defed: call 0x142923c00
0x1400deff2: mov qword ptr [rbp + 0x7a0], rsi
0x1400deff9: mov dword ptr [rbp + 0x3b0], 2
0x1400df003: mov qword ptr [rsp + 0x20], r12
0x1400df008: lea rcx, [rbp + 0x7a0]
0x1400df00f: mov rdx, r13
0x1400df012: movabs r8, 0xd9978e541ce9a95e
0x1400df01c: mov r9b, 0x1d
0x1400df01f: call 0x1403728a0
0x1400df024: mov r8d, 0xf8
0x1400df02a: mov rcx, r14
0x1400df02d: mov rdx, rbx
0x1400df030: call 0x142923c00
0x1400df035: mov byte ptr [rbp + 0xa77], 1
0x1400df03c: mov byte ptr [rbp + 0xa76], 1
0x1400df043: mov rcx, rbx
0x1400df046: mov rdx, r14
0x1400df049: movabs r8, 0x12ef39a9447f1f0e
0x1400df053: lea r9, [rbp + 0x910]
0x1400df05a: call 0x140906400
0x1400df05f: mov r14, r15
0x1400df062: and byte ptr [rbp + 0x868], 0x7f
0x1400df069: movzx eax, byte ptr [rbp + 0x86b]
0x1400df070: and al, 0x9f
0x1400df072: or al, 0x40
0x1400df074: mov byte ptr [rbp + 0x86b], al
0x1400df07a: mov r8d, 0xf8
0x1400df080: mov rcx, r12
0x1400df083: mov rdx, r13
0x1400df086: call 0x142923c00
0x1400df08b: mov r8, qword ptr [rbp + 0xa20]
0x1400df092: mov byte ptr [rbp + 0xa77], 1
0x1400df099: mov byte ptr [rbp + 0xa76], 1
0x1400df0a0: mov rcx, r12
0x1400df0a3: lea rdx, [rbp + 0x8c0]
0x1400df0aa: call 0x140906cb0
0x1400df0af: mov rax, qword ptr [rbp + 0x8e0]
0x1400df0b6: mov qword ptr [rbp + 0x3d0], rax
0x1400df0bd: movaps xmm0, xmmword ptr [rbp + 0x8c0]
0x1400df0c4: movaps xmm1, xmmword ptr [rbp + 0x8d0]
0x1400df0cb: movaps xmmword ptr [rbp + 0x3c0], xmm1
0x1400df0d2: movaps xmmword ptr [rbp + 0x3b0], xmm0
0x1400df0d9: mov rax, qword ptr [rbp + 0xa50]
0x1400df0e0: mov rsi, qword ptr [rax + 0x58]
0x1400df0e4: cmp rsi, qword ptr [rax + 0x48]
0x1400df0e8: mov r15, rdi
0x1400df0eb: jne 0x1400df100
0x1400df0ed: mov rcx, qword ptr [rbp + 0x940]
0x1400df0f4: lea rdx, [rip + 0x285c135]
0x1400df0fb: call 0x140562ab0
0x1400df100: mov r8, qword ptr [rbp + 0xa50]
0x1400df107: mov rax, qword ptr [r8 + 0x50]
0x1400df10b: lea rcx, [rsi + rsi*4]
0x1400df10f: movaps xmm0, xmmword ptr [rbp + 0x3b0]
0x1400df116: movaps xmm1, xmmword ptr [rbp + 0x3c0]
0x1400df11d: mov rdx, qword ptr [rbp + 0x3d0]
0x1400df124: mov qword ptr [rax + rcx*8 + 0x20], rdx
0x1400df129: movups xmmword ptr [rax + rcx*8 + 0x10], xmm1
0x1400df12e: movups xmmword ptr [rax + rcx*8], xmm0
0x1400df132: lea rbx, [rsi + 1]
0x1400df136: mov qword ptr [r8 + 0x58], rbx
0x1400df13a: mov rax, qword ptr [rbp + 0x930]
0x1400df141: mov qword ptr [rbp + 0x3d0], rax
0x1400df148: movaps xmm0, xmmword ptr [rbp + 0x910]
0x1400df14f: movaps xmm1, xmmword ptr [rbp + 0x920]
0x1400df156: movaps xmmword ptr [rbp + 0x3c0], xmm1
0x1400df15d: movaps xmmword ptr [rbp + 0x3b0], xmm0
0x1400df164: cmp rbx, qword ptr [r8 + 0x48]
0x1400df168: lea rdi, [rbp + 0x7c8]
0x1400df16f: jne 0x1400df184
0x1400df171: mov rcx, qword ptr [rbp + 0x940]
0x1400df178: lea rdx, [rip + 0x285c0c9]
0x1400df17f: call 0x140562ab0
0x1400df184: mov r8, qword ptr [rbp + 0xa50]
0x1400df18b: mov rax, qword ptr [r8 + 0x50]
0x1400df18f: lea rcx, [rbx + rbx*4]
0x1400df193: movdqa xmm0, xmmword ptr [rbp + 0x3b0]
0x1400df19b: movdqa xmm1, xmmword ptr [rbp + 0x3c0]
0x1400df1a3: mov rdx, qword ptr [rbp + 0x3d0]
0x1400df1aa: mov qword ptr [rax + rcx*8 + 0x20], rdx
0x1400df1af: movdqu xmmword ptr [rax + rcx*8 + 0x10], xmm1
0x1400df1b5: movdqu xmmword ptr [rax + rcx*8], xmm0
0x1400df1ba: add rsi, 2
0x1400df1be: mov qword ptr [r8 + 0x58], rsi
0x1400df1c2: mov rdx, qword ptr [rbp + 0xa10]
0x1400df1c9: test rdx, rdx
0x1400df1cc: je 0x1400df1e4
0x1400df1ce: mov rcx, qword ptr [rbp + 0xa18]
0x1400df1d5: shl rdx, 2
0x1400df1d9: mov r8d, 4
0x1400df1df: call 0x140613c20
0x1400df1e4: mov rax, qword ptr [rbp + 0xa28]
0x1400df1eb: test rax, rax
0x1400df1ee: je 0x1400df20a
0x1400df1f0: mov rcx, qword ptr [rbp + 0xa30]
0x1400df1f7: shl rax, 2
0x1400df1fb: lea rdx, [rax + rax*4]
0x1400df1ff: mov r8d, 4
0x1400df205: call 0x140613c20
0x1400df20a: cmp dword ptr [rbp + 0x9a0], 2
0x1400df211: jne 0x1400deb30
0x1400df217: mov rax, qword ptr [rbp + 0x9a8]
0x1400df21e: lock dec qword ptr [rax]
0x1400df222: jne 0x1400deb30
0x1400df228: lea rcx, [rbp + 0x9a8]
0x1400df22f: call 0x14064f9d0
0x1400df234: jmp 0x1400deb30
0x1400df239: lea rcx, [rip + 0x285bf48]
0x1400df240: call 0x1428d9310
0x1400df245: jmp 0x1400df7f0
0x1400df24a: mov byte ptr [rbp + 0xa74], 1
0x1400df251: lea rcx, [rbp + 0x8f0]
0x1400df258: call 0x140605ea0
0x1400df25d: mov r12, qword ptr [rbp + 0x948]
0x1400df264: lea rsi, [rbp + 0x3b0]
0x1400df26b: lea rdx, [rbp - 0x38]
0x1400df26f: mov r8d, 0x3e8
0x1400df275: mov rcx, rsi
0x1400df278: call 0x142923c00
0x1400df27d: mov byte ptr [rbp + 0xa74], 0
0x1400df284: mov rcx, rsi
0x1400df287: call 0x140868370
0x1400df28c: mov rdx, qword ptr [rbp + 0xa50]
0x1400df293: lea rcx, [rdx + 0x30]
0x1400df297: mov rax, qword ptr [rdx + 0x38]
0x1400df29b: mov rdi, qword ptr [rdx + 0x40]
0x1400df29f: mov qword ptr [rdx + 0x40], 0
0x1400df2a7: mov qword ptr [rbp + 0x968], rdi
0x1400df2ae: shl rdi, 4
0x1400df2b2: add rdi, rax
0x1400df2b5: mov qword ptr [rbp + 0x970], 0
0x1400df2c0: mov qword ptr [rbp + 0x958], rdi
0x1400df2c7: mov qword ptr [rbp + 0x960], rcx
0x1400df2ce: lea r14, [rbp + 0x3c0]
0x1400df2d5: lea rsi, [rbp + 0xa00]
0x1400df2dc: lea rbx, [rbp + 0x3b0]
0x1400df2e3: cmp rax, rdi
0x1400df2e6: je 0x1400df3dc
0x1400df2ec: nop dword ptr [rax]
0x1400df2f0: lea r15, [rax + 0x10]
0x1400df2f4: mov qword ptr [rbp + 0x950], r15
0x1400df2fb: movups xmm0, xmmword ptr [rax]
0x1400df2fe: movaps xmmword ptr [rbp + 0xa00], xmm0
0x1400df305: mov rcx, qword ptr [rbp + 0x9f0]
0x1400df30c: mov rdx, rsi
0x1400df30f: call 0x1405b1d60
0x1400df314: mov rcx, rbx
0x1400df317: mov rdx, r12
0x1400df31a: mov r8, rax
0x1400df31d: mov r9, rsi
0x1400df320: call 0x140053470
0x1400df325: cmp dword ptr [rbp + 0x3b0], 2
0x1400df32c: je 0x1400df4e0
0x1400df332: mov rax, qword ptr [r14 + 0x2e]
0x1400df336: mov qword ptr [rbp + 0x7ee], rax
0x1400df33d: movups xmm0, xmmword ptr [r14]
0x1400df341: movups xmm1, xmmword ptr [r14 + 0x10]
0x1400df346: movups xmm2, xmmword ptr [r14 + 0x20]
0x1400df34b: movaps xmmword ptr [rbp + 0x7e0], xmm2
0x1400df352: movaps xmmword ptr [rbp + 0x7d0], xmm1
0x1400df359: movaps xmmword ptr [rbp + 0x7c0], xmm0
0x1400df360: movzx eax, byte ptr [rbp + 0x3f6]
0x1400df367: cmp al, 7
0x1400df369: je 0x1400df4e0
0x1400df36f: movzx ecx, byte ptr [rbp + 0x3f7]
0x1400df376: mov rdx, qword ptr [rbp + 0x7ee]
0x1400df37d: mov qword ptr [rbp + 0x3de], rdx
0x1400df384: movdqa xmm0, xmmword ptr [rbp + 0x7c0]
0x1400df38c: movdqa xmm1, xmmword ptr [rbp + 0x7d0]
0x1400df394: movdqa xmm2, xmmword ptr [rbp + 0x7e0]
0x1400df39c: movdqa xmmword ptr [rbp + 0x3d0], xmm2
0x1400df3a4: movdqa xmmword ptr [rbp + 0x3c0], xmm1
0x1400df3ac: movdqa xmmword ptr [rbp + 0x3b0], xmm0
0x1400df3b4: mov byte ptr [rbp + 0x3e6], al
0x1400df3ba: mov byte ptr [rbp + 0x3e7], cl
0x1400df3c0: mov rcx, rbx
0x1400df3c3: call 0x14003bb80
0x1400df3c8: mov rcx, rbx
0x1400df3cb: call 0x140be24b0
0x1400df3d0: mov rax, r15
0x1400df3d3: cmp rax, rdi
0x1400df3d6: jne 0x1400df2f0
0x1400df3dc: movaps xmm6, xmmword ptr [rbp + 0xa80]
0x1400df3e3: movaps xmm7, xmmword ptr [rbp + 0xa90]
0x1400df3ea: movaps xmm8, xmmword ptr [rbp + 0xaa0]
0x1400df3f2: movaps xmm9, xmmword ptr [rbp + 0xab0]
0x1400df3fa: movaps xmm10, xmmword ptr [rbp + 0xac0]
0x1400df402: movaps xmm11, xmmword ptr [rbp + 0xad0]
0x1400df40a: add rsp, 0xb68
0x1400df411: pop rbx
0x1400df412: pop rdi
0x1400df413: pop rsi
0x1400df414: pop r12
0x1400df416: pop r13
0x1400df418: pop r14
0x1400df41a: pop r15
0x1400df41c: pop rbp
0x1400df41d: ret 
0x1400df41e: lea rcx, [rip + 0x2853693]
0x1400df425: call 0x1428d9250
0x1400df42a: jmp 0x1400df7f0
0x1400df42f: lea rcx, [rip + 0x2853682]
0x1400df436: call 0x1428d9250
0x1400df43b: jmp 0x1400df7f0
0x1400df440: mov qword ptr [rbp + 0x3b0], 0
0x1400df44b: mov rax, qword ptr [rbp + 0xa58]
0x1400df452: mov qword ptr [rbp + 0xa48], rax
0x1400df459: mov rax, qword ptr [rbp + 0xa68]
0x1400df460: mov qword ptr [rbp + 0xa40], rax
0x1400df467: mov eax, dword ptr [rbp + 0xa60]
0x1400df46d: mov byte ptr [rbp + 0xa75], al
0x1400df473: lea rax, [rip + 0x285be26]
0x1400df47a: mov qword ptr [rsp + 0x20], rax
0x1400df47f: lea rdx, [rbp + 0xa00]
0x1400df486: lea r8, [rbp - 0x38]
0x1400df48a: lea r9, [rbp + 0x3b0]
0x1400df491: xor ecx, ecx
0x1400df493: call 0x1428d9596
0x1400df498: jmp 0x1400df7f0
0x1400df49d: mov byte ptr [rbp + 0xa73], 1
0x1400df4a4: lea rax, [rip + 0x285bd25]
0x1400df4ab: mov rcx, r8
0x1400df4ae: mov r8, rax
0x1400df4b1: call 0x1428d98e0
0x1400df4b6: jmp 0x1400df7f0
0x1400df4bb: mov byte ptr [rbp + 0xa77], 1
0x1400df4c2: mov byte ptr [rbp + 0xa76], 1
0x1400df4c9: lea rax, [rip + 0x285bd48]
0x1400df4d0: mov rcx, r8
0x1400df4d3: mov r8, rax
0x1400df4d6: call 0x1428d98e0
0x1400df4db: jmp 0x1400df7f0
0x1400df4e0: lea rcx, [rip + 0x285bdf9]
0x1400df4e7: lea r8, [rip + 0x285be0a]
0x1400df4ee: mov edx, 0x11
0x1400df4f3: call 0x1428d9330
0x1400df4f8: jmp 0x1400df7f0
0x1400df4fd: movups xmm0, xmmword ptr [rbp + 8]
0x1400df501: movaps xmmword ptr [rbp + 0x3f0], xmm0
0x1400df508: movdqu xmm0, xmmword ptr [rbp - 0x38]
0x1400df50d: movups xmm1, xmmword ptr [rbp - 0x28]
0x1400df511: movups xmm2, xmmword ptr [rbp - 0x18]
0x1400df515: movups xmm3, xmmword ptr [rbp - 8]
0x1400df519: movaps xmmword ptr [rbp + 0x3e0], xmm3
0x1400df520: movaps xmmword ptr [rbp + 0x3d0], xmm2
0x1400df527: movaps xmmword ptr [rbp + 0x3c0], xmm1
0x1400df52e: movdqa xmmword ptr [rbp + 0x3b0], xmm0
0x1400df536: lea rax, [rip + 0x285bd4b]
0x1400df53d: mov qword ptr [rsp + 0x20], rax
0x1400df542: lea rcx, [rip + 0x285b887]
0x1400df549: lea r9, [rip + 0x285b860]
0x1400df550: lea r8, [rbp + 0x3b0]
0x1400df557: mov edx, 0x2b
0x1400df55c: call 0x1428d9760
0x1400df561: jmp 0x1400df7f0
0x1400df566: lea rcx, [rip + 0x28534a3]
0x1400df56d: call 0x1428d80d0
0x1400df572: jmp 0x1400df7f0
0x1400df577: lea rcx, [rip + 0x2853492]
0x1400df57e: call 0x1428d80d0
0x1400df583: jmp 0x1400df7f0
0x1400df588: movups xmm0, xmmword ptr [rbp + 8]
0x1400df58c: movaps xmmword ptr [rbp + 0x3f0], xmm0
0x1400df593: movdqu xmm0, xmmword ptr [rbp - 0x38]
0x1400df598: movups xmm1, xmmword ptr [rbp - 0x28]
0x1400df59c: movups xmm2, xmmword ptr [rbp - 0x18]
0x1400df5a0: movups xmm3, xmmword ptr [rbp - 8]
0x1400df5a4: movaps xmmword ptr [rbp + 0x3e0], xmm3
0x1400df5ab: movaps xmmword ptr [rbp + 0x3d0], xmm2
0x1400df5b2: movaps xmmword ptr [rbp + 0x3c0], xmm1
0x1400df5b9: movdqa xmmword ptr [rbp + 0x3b0], xmm0
0x1400df5c1: lea rax, [rip + 0x285bd00]
0x1400df5c8: mov qword ptr [rsp + 0x20], rax
0x1400df5cd: lea rcx, [rip + 0x285b7fc]
0x1400df5d4: lea r9, [rip + 0x285b7d5]
0x1400df5db: lea r8, [rbp + 0x3b0]
0x1400df5e2: mov edx, 0x2b
0x1400df5e7: call 0x1428d9760
0x1400df5ec: jmp 0x1400df7f0
0x1400df5f1: mov rax, qword ptr [rbp + 0xa58]
0x1400df5f8: mov qword ptr [rbp + 0xa48], rax
0x1400df5ff: mov rax, qword ptr [rbp + 0xa68]
0x1400df606: mov qword ptr [rbp + 0xa40], rax
0x1400df60d: mov byte ptr [rbp + 0xa75], 0
0x1400df614: lea r8, [rip + 0x285b77d]
0x1400df61b: mov ecx, 1
0x1400df620: mov rdx, rsi
0x1400df623: call 0x1428d8fe3
0x1400df628: jmp 0x1400df7f0
0x1400df62d: lea rcx, [rip + 0x285bc2c]
0x1400df634: lea r8, [rip + 0x285bc35]
0x1400df63b: mov edx, 0xf
0x1400df640: call 0x1428d9430
0x1400df645: jmp 0x1400df7f0
0x1400df64a: lea rcx, [rip + 0x285bae7]
0x1400df651: call 0x1428d9310
0x1400df656: mov byte ptr [rbp + 0xa74], 1
0x1400df65d: lea rcx, [rip + 0x285bb0c]
0x1400df664: call 0x1428d9310
0x1400df669: jmp 0x1400df7f0
0x1400df66e: lea rax, [rip + 0x285b823]
0x1400df675: mov qword ptr [rsp + 0x20], rax
0x1400df67a: lea rcx, [rip + 0x285ba8f]
0x1400df681: mov edx, 0x25
0x1400df686: mov r9d, 0x25
0x1400df68c: mov r8, r14
0x1400df68f: call 0x1428d9940
0x1400df694: jmp 0x1400df7f0
0x1400df699: lea rax, [rip + 0x285b818]
0x1400df6a0: mov qword ptr [rsp + 0x20], rax
0x1400df6a5: lea rcx, [rip + 0x285b74f]
0x1400df6ac: mov edx, 0x36
0x1400df6b1: xor r8d, r8d
0x1400df6b4: call 0x1428d9940
0x1400df6b9: jmp 0x1400df7f0
0x1400df6be: lea rax, [rip + 0x285b80b]
0x1400df6c5: mov qword ptr [rsp + 0x20], rax
0x1400df6ca: lea rcx, [rip + 0x285b72a]
0x1400df6d1: mov edx, 0x36
0x1400df6d6: mov r9d, 0x36
0x1400df6dc: mov r8, r12
0x1400df6df: call 0x1428d9940
0x1400df6e4: jmp 0x1400df7f0
0x1400df6e9: movups xmm0, xmmword ptr [rbp + 0x800]
0x1400df6f0: movaps xmmword ptr [rbp + 0x3f0], xmm0
0x1400df6f7: movdqu xmm0, xmmword ptr [rbp + 0x7c0]
0x1400df6ff: movdqu xmm1, xmmword ptr [rbp + 0x7d0]
0x1400df707: movdqu xmm2, xmmword ptr [rbp + 0x7e0]
0x1400df70f: movups xmm3, xmmword ptr [rbp + 0x7f0]
0x1400df716: movaps xmmword ptr [rbp + 0x3e0], xmm3
0x1400df71d: movdqa xmmword ptr [rbp + 0x3d0], xmm2
0x1400df725: movdqa xmmword ptr [rbp + 0x3c0], xmm1
0x1400df72d: movdqa xmmword ptr [rbp + 0x3b0], xmm0
0x1400df735: lea rax, [rip + 0x285ba7c]
0x1400df73c: mov qword ptr [rsp + 0x20], rax
0x1400df741: lea rcx, [rip + 0x285b688]
0x1400df748: lea r9, [rip + 0x285b661]
0x1400df74f: lea r8, [rbp + 0x3b0]
0x1400df756: mov edx, 0x2b
0x1400df75b: call 0x1428d9760
0x1400df760: jmp 0x1400df7f0
0x1400df765: movups xmm0, xmmword ptr [rbp + 0x800]
0x1400df76c: movaps xmmword ptr [rbp + 0x3f0], xmm0
0x1400df773: movups xmm0, xmmword ptr [rbp + 0x7c0]
0x1400df77a: movups xmm1, xmmword ptr [rbp + 0x7d0]
0x1400df781: movdqu xmm2, xmmword ptr [rbp + 0x7e0]
0x1400df789: movups xmm3, xmmword ptr [rbp + 0x7f0]
0x1400df790: movaps xmmword ptr [rbp + 0x3e0], xmm3
0x1400df797: movdqa xmmword ptr [rbp + 0x3d0], xmm2
0x1400df79f: movaps xmmword ptr [rbp + 0x3c0], xmm1
0x1400df7a6: movaps xmmword ptr [rbp + 0x3b0], xmm0
0x1400df7ad: lea rax, [rip + 0x285ba4c]
0x1400df7b4: mov qword ptr [rsp + 0x20], rax
0x1400df7b9: lea rcx, [rip + 0x285b610]
0x1400df7c0: lea r9, [rip + 0x285b5e9]
0x1400df7c7: lea r8, [rbp + 0x3b0]
0x1400df7ce: mov edx, 0x2b
0x1400df7d3: call 0x1428d9760
0x1400df7d8: jmp 0x1400df7f0
0x1400df7da: lea r8, [rip + 0x2880567]
0x1400df7e1: mov ecx, 1
0x1400df7e6: mov edx, 8
0x1400df7eb: call 0x1428d8fe3
0x1400df7f0: ud2 
0x1400df7f2: lea rcx, [rip + 0x285b95f]
0x1400df7f9: call 0x1428d9310
0x1400df7fe: int3 
0x1400df7ff: nop 
0x1400df800: mov qword ptr [rsp + 0x10], rdx
0x1400df805: push rbp
0x1400df806: push r15
0x1400df808: push r14
0x1400df80a: push r13
0x1400df80c: push r12
0x1400df80e: push rsi
0x1400df80f: push rdi
0x1400df810: push rbx
0x1400df811: sub rsp, 0xa8
0x1400df818: lea rbp, [rdx + 0x80]
0x1400df81f: movaps xmmword ptr [rsp + 0x40], xmm11
0x1400df825: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400df82b: movaps xmmword ptr [rsp + 0x60], xmm9
0x1400df831: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400df837: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400df83f: movdqa xmmword ptr [rsp + 0x90], xmm6
0x1400df848: mov rsi, qword ptr [rbp + 0xa68]
0x1400df84f: test rsi, rsi
0x1400df852: mov rdi, qword ptr [rbp + 0xa58]
0x1400df859: je 0x1400df86c
0x1400df85b: mov rcx, rdi
0x1400df85e: call 0x140be24b0
0x1400df863: add rdi, 0x28
0x1400df867: dec rsi
0x1400df86a: jne 0x1400df85b
0x1400df86c: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400df874: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400df87c: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400df882: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400df888: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400df88e: movaps xmm11, xmmword ptr [rsp + 0x40]
0x1400df894: add rsp, 0xa8
0x1400df89b: pop rbx
0x1400df89c: pop rdi
0x1400df89d: pop rsi
0x1400df89e: pop r12
0x1400df8a0: pop r13
0x1400df8a2: pop r14
0x1400df8a4: pop r15
0x1400df8a6: pop rbp
0x1400df8a7: ret 
0x1400df8a8: nop dword ptr [rax + rax]
0x1400df8b0: mov qword ptr [rsp + 0x10], rdx
0x1400df8b5: push rbp
0x1400df8b6: push r15
0x1400df8b8: push r14
0x1400df8ba: push r13
0x1400df8bc: push r12
0x1400df8be: push rsi
0x1400df8bf: push rdi
0x1400df8c0: push rbx
0x1400df8c1: sub rsp, 0xa8
0x1400df8c8: lea rbp, [rdx + 0x80]
0x1400df8cf: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400df8d6: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400df8dc: movaps xmmword ptr [rsp + 0x60], xmm9
0x1400df8e2: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400df8e8: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400df8f0: movdqa xmmword ptr [rsp + 0x90], xmm6
0x1400df8f9: mov rax, qword ptr [rbp + 0xa60]
0x1400df900: inc qword ptr [rax]
0x1400df903: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400df90b: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400df913: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400df919: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400df91f: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400df925: movaps xmm11, xmmword ptr [rsp + 0x40]
0x1400df92b: add rsp, 0xa8
0x1400df932: pop rbx
0x1400df933: pop rdi
0x1400df934: pop rsi
0x1400df935: pop r12
0x1400df937: pop r13
0x1400df939: pop r14
0x1400df93b: pop r15
0x1400df93d: pop rbp
0x1400df93e: ret 
0x1400df93f: nop 
0x1400df940: mov qword ptr [rsp + 0x10], rdx
0x1400df945: push rbp
0x1400df946: push r15
0x1400df948: push r14
0x1400df94a: push r13
0x1400df94c: push r12
0x1400df94e: push rsi
0x1400df94f: push rdi
0x1400df950: push rbx
0x1400df951: sub rsp, 0xa8
0x1400df958: lea rbp, [rdx + 0x80]
0x1400df95f: movaps xmmword ptr [rsp + 0x40], xmm11
0x1400df965: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400df96b: movaps xmmword ptr [rsp + 0x60], xmm9
0x1400df971: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400df977: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400df97f: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400df987: cmp qword ptr [rbp + 0x938], 0
0x1400df98f: je 0x1400df9c0
0x1400df991: lea rax, [rbp + 0x918]
0x1400df998: mov qword ptr [rbp + 0x8f0], rax
0x1400df99f: lea rax, [rbp + 0x8f0]
0x1400df9a6: mov qword ptr [rbp + 0x8c0], rax
0x1400df9ad: lea rcx, [rip + 0x285b484]
0x1400df9b4: lea rdx, [rbp + 0x8c0]
0x1400df9bb: call 0x1400a2580
0x1400df9c0: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400df9c8: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400df9d0: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400df9d6: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400df9dc: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400df9e2: movaps xmm11, xmmword ptr [rsp + 0x40]
0x1400df9e8: add rsp, 0xa8
0x1400df9ef: pop rbx
0x1400df9f0: pop rdi
0x1400df9f1: pop rsi
0x1400df9f2: pop r12
0x1400df9f4: pop r13
0x1400df9f6: pop r14
0x1400df9f8: pop r15
0x1400df9fa: pop rbp
0x1400df9fb: ret 
0x1400df9fc: nop dword ptr [rax]
0x1400dfa00: mov qword ptr [rsp + 0x10], rdx
0x1400dfa05: push rbp
0x1400dfa06: push r15
0x1400dfa08: push r14
0x1400dfa0a: push r13
0x1400dfa0c: push r12
0x1400dfa0e: push rsi
0x1400dfa0f: push rdi
0x1400dfa10: push rbx
0x1400dfa11: sub rsp, 0xa8
0x1400dfa18: lea rbp, [rdx + 0x80]
0x1400dfa1f: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400dfa26: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400dfa2c: movaps xmmword ptr [rsp + 0x60], xmm9
0x1400dfa32: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400dfa38: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400dfa40: movdqa xmmword ptr [rsp + 0x90], xmm6
0x1400dfa49: mov rsi, qword ptr [rbp + 0xa58]
0x1400dfa50: mov qword ptr [rbp + 0xa48], rsi
0x1400dfa57: mov rdi, qword ptr [rbp + 0xa68]
0x1400dfa5e: mov qword ptr [rbp + 0xa40], rdi
0x1400dfa65: mov ebx, dword ptr [rbp + 0xa60]
0x1400dfa6b: mov byte ptr [rbp + 0xa75], bl
0x1400dfa71: lea rcx, [rbp + 0x3b0]
0x1400dfa78: call 0x1400dcef0
0x1400dfa7d: mov qword ptr [rbp + 0xa48], rsi
0x1400dfa84: mov qword ptr [rbp + 0xa40], rdi
0x1400dfa8b: mov byte ptr [rbp + 0xa75], bl
0x1400dfa91: movdqa xmm6, xmmword ptr [rsp + 0x90]
0x1400dfa9a: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400dfaa2: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400dfaa8: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400dfaae: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400dfab4: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400dfabb: add rsp, 0xa8
0x1400dfac2: pop rbx
0x1400dfac3: pop rdi
0x1400dfac4: pop rsi
0x1400dfac5: pop r12
0x1400dfac7: pop r13
0x1400dfac9: pop r14
0x1400dfacb: pop r15
0x1400dfacd: pop rbp
0x1400dface: ret 
0x1400dfacf: nop 
0x1400dfad0: mov qword ptr [rsp + 0x10], rdx
0x1400dfad5: push rbp
0x1400dfad6: push r15
0x1400dfad8: push r14
0x1400dfada: push r13
0x1400dfadc: push r12
0x1400dfade: push rsi
0x1400dfadf: push rdi
0x1400dfae0: push rbx
0x1400dfae1: sub rsp, 0xa8
0x1400dfae8: lea rbp, [rdx + 0x80]
0x1400dfaef: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400dfaf6: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400dfafc: movaps xmmword ptr [rsp + 0x60], xmm9
0x1400dfb02: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400dfb08: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400dfb10: movdqa xmmword ptr [rsp + 0x90], xmm6
0x1400dfb19: mov rax, qword ptr [rbp + 0xa60]
0x1400dfb20: inc qword ptr [rax]
0x1400dfb23: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400dfb2b: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400dfb33: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400dfb39: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400dfb3f: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400dfb45: movaps xmm11, xmmword ptr [rsp + 0x40]
0x1400dfb4b: add rsp, 0xa8
0x1400dfb52: pop rbx
0x1400dfb53: pop rdi
0x1400dfb54: pop rsi
0x1400dfb55: pop r12
0x1400dfb57: pop r13
0x1400dfb59: pop r14
0x1400dfb5b: pop r15
0x1400dfb5d: pop rbp
0x1400dfb5e: ret 
0x1400dfb5f: nop 
0x1400dfb60: mov qword ptr [rsp + 0x10], rdx
0x1400dfb65: push rbp
0x1400dfb66: push r15
0x1400dfb68: push r14
0x1400dfb6a: push r13
0x1400dfb6c: push r12
0x1400dfb6e: push rsi
0x1400dfb6f: push rdi
0x1400dfb70: push rbx
0x1400dfb71: sub rsp, 0xa8
0x1400dfb78: lea rbp, [rdx + 0x80]
0x1400dfb7f: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400dfb86: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400dfb8c: movaps xmmword ptr [rsp + 0x60], xmm9
0x1400dfb92: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400dfb98: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400dfba0: movdqa xmmword ptr [rsp + 0x90], xmm6
0x1400dfba9: test byte ptr [rbp + 0xa75], 1
0x1400dfbb0: je 0x1400dfc25
0x1400dfbb2: mov rdx, qword ptr [rbp + 0xa40]
0x1400dfbb9: test rdx, rdx
0x1400dfbbc: mov rax, qword ptr [rbp + 0x9e8]
0x1400dfbc3: je 0x1400dfbe2
0x1400dfbc5: mov rcx, qword ptr [rbp + 0xa48]
0x1400dfbcc: shl rdx, 2
0x1400dfbd0: mov r8d, 1
0x1400dfbd6: call 0x140613c20
0x1400dfbdb: mov rax, qword ptr [rbp + 0x910]
0x1400dfbe2: mov qword ptr [rbp + 0x938], rax
0x1400dfbe9: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400dfbf1: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400dfbf9: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400dfbff: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400dfc05: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400dfc0b: movaps xmm11, xmmword ptr [rsp + 0x40]
0x1400dfc11: add rsp, 0xa8
0x1400dfc18: pop rbx
0x1400dfc19: pop rdi
0x1400dfc1a: pop rsi
0x1400dfc1b: pop r12
0x1400dfc1d: pop r13
0x1400dfc1f: pop r14
0x1400dfc21: pop r15
0x1400dfc23: pop rbp
0x1400dfc24: ret 
0x1400dfc25: mov rax, qword ptr [rbp + 0x9e8]
0x1400dfc2c: mov qword ptr [rbp + 0x938], rax
0x1400dfc33: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400dfc3b: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400dfc43: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400dfc49: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400dfc4f: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400dfc55: movaps xmm11, xmmword ptr [rsp + 0x40]
0x1400dfc5b: add rsp, 0xa8
0x1400dfc62: pop rbx
0x1400dfc63: pop rdi
0x1400dfc64: pop rsi
0x1400dfc65: pop r12
0x1400dfc67: pop r13
0x1400dfc69: pop r14
0x1400dfc6b: pop r15
0x1400dfc6d: pop rbp
0x1400dfc6e: ret 
0x1400dfc6f: nop 
0x1400dfc70: mov qword ptr [rsp + 0x10], rdx
0x1400dfc75: push rbp
0x1400dfc76: push r15
0x1400dfc78: push r14
0x1400dfc7a: push r13
0x1400dfc7c: push r12
0x1400dfc7e: push rsi
0x1400dfc7f: push rdi
0x1400dfc80: push rbx
0x1400dfc81: sub rsp, 0xa8
0x1400dfc88: lea rbp, [rdx + 0x80]
0x1400dfc8f: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400dfc96: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400dfc9c: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400dfca3: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400dfca9: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400dfcb1: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400dfcb9: lea rcx, [rbp + 0x3b0]
0x1400dfcc0: call 0x140be24b0
0x1400dfcc5: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400dfccd: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400dfcd5: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400dfcdb: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400dfce1: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400dfce7: movaps xmm11, xmmword ptr [rsp + 0x40]
0x1400dfced: add rsp, 0xa8
0x1400dfcf4: pop rbx
0x1400dfcf5: pop rdi
0x1400dfcf6: pop rsi
0x1400dfcf7: pop r12
0x1400dfcf9: pop r13
0x1400dfcfb: pop r14
0x1400dfcfd: pop r15
0x1400dfcff: pop rbp
0x1400dfd00: ret 
0x1400dfd01: nop word ptr cs:[rax + rax]
0x1400dfd10: mov qword ptr [rsp + 0x10], rdx
0x1400dfd15: push rbp
0x1400dfd16: push r15
0x1400dfd18: push r14
0x1400dfd1a: push r13
0x1400dfd1c: push r12
0x1400dfd1e: push rsi
0x1400dfd1f: push rdi
0x1400dfd20: push rbx
0x1400dfd21: sub rsp, 0xa8
0x1400dfd28: lea rbp, [rdx + 0x80]
0x1400dfd2f: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400dfd36: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400dfd3c: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400dfd43: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400dfd49: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400dfd51: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400dfd59: mov rdx, qword ptr [rbp + 0xa10]
0x1400dfd60: test rdx, rdx
0x1400dfd63: je 0x1400dfd7b
0x1400dfd65: mov rcx, qword ptr [rbp + 0xa18]
0x1400dfd6c: shl rdx, 2
0x1400dfd70: mov r8d, 4
0x1400dfd76: call 0x140613c20
0x1400dfd7b: mov rax, qword ptr [rbp + 0xa28]
0x1400dfd82: test rax, rax
0x1400dfd85: je 0x1400dfda1
0x1400dfd87: mov rcx, qword ptr [rbp + 0xa30]
0x1400dfd8e: shl rax, 2
0x1400dfd92: lea rdx, [rax + rax*4]
0x1400dfd96: mov r8d, 4
0x1400dfd9c: call 0x140613c20
0x1400dfda1: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400dfda9: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400dfdb1: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400dfdb7: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400dfdbd: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400dfdc3: movaps xmm11, xmmword ptr [rsp + 0x40]
0x1400dfdc9: add rsp, 0xa8
0x1400dfdd0: pop rbx
0x1400dfdd1: pop rdi
0x1400dfdd2: pop rsi
0x1400dfdd3: pop r12
0x1400dfdd5: pop r13
0x1400dfdd7: pop r14
0x1400dfdd9: pop r15
0x1400dfddb: pop rbp
0x1400dfddc: ret 
0x1400dfddd: nop dword ptr [rax]
0x1400dfde0: mov qword ptr [rsp + 0x10], rdx
0x1400dfde5: push rbp
0x1400dfde6: push r15
0x1400dfde8: push r14
0x1400dfdea: push r13
0x1400dfdec: push r12
0x1400dfdee: push rsi
0x1400dfdef: push rdi
0x1400dfdf0: push rbx
0x1400dfdf1: sub rsp, 0xa8
0x1400dfdf8: lea rbp, [rdx + 0x80]
0x1400dfdff: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400dfe06: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400dfe0c: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400dfe13: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400dfe19: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400dfe21: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400dfe29: movzx ebx, byte ptr [rbp + 0xa76]
0x1400dfe30: cmp byte ptr [rbp + 0xa77], 0
0x1400dfe37: je 0x1400dfe8c
0x1400dfe39: lea rcx, [rbp + 0x910]
0x1400dfe40: call 0x140be24b0
0x1400dfe45: and bl, 1
0x1400dfe48: mov byte ptr [rbp + 0xa73], bl
0x1400dfe4e: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400dfe56: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400dfe5e: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400dfe64: movdqa xmm9, xmmword ptr [rsp + 0x60]
0x1400dfe6b: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400dfe71: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400dfe78: add rsp, 0xa8
0x1400dfe7f: pop rbx
0x1400dfe80: pop rdi
0x1400dfe81: pop rsi
0x1400dfe82: pop r12
0x1400dfe84: pop r13
0x1400dfe86: pop r14
0x1400dfe88: pop r15
0x1400dfe8a: pop rbp
0x1400dfe8b: ret 
0x1400dfe8c: and bl, 1
0x1400dfe8f: mov byte ptr [rbp + 0xa73], bl
0x1400dfe95: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400dfe9d: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400dfea5: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400dfeab: movdqa xmm9, xmmword ptr [rsp + 0x60]
0x1400dfeb2: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400dfeb8: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400dfebf: add rsp, 0xa8
0x1400dfec6: pop rbx
0x1400dfec7: pop rdi
0x1400dfec8: pop rsi
0x1400dfec9: pop r12
0x1400dfecb: pop r13
0x1400dfecd: pop r14
0x1400dfecf: pop r15
0x1400dfed1: pop rbp
0x1400dfed2: ret 
0x1400dfed3: nop word ptr cs:[rax + rax]
0x1400dfee0: mov qword ptr [rsp + 0x10], rdx
0x1400dfee5: push rbp
0x1400dfee6: push r15
0x1400dfee8: push r14
0x1400dfeea: push r13
0x1400dfeec: push r12
0x1400dfeee: push rsi
0x1400dfeef: push rdi
0x1400dfef0: push rbx
0x1400dfef1: sub rsp, 0xa8
0x1400dfef8: lea rbp, [rdx + 0x80]
0x1400dfeff: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400dff06: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400dff0c: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400dff13: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400dff19: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400dff21: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400dff29: cmp byte ptr [rbp + 0xa73], 0
0x1400dff30: je 0x1400dff7c
0x1400dff32: lea rcx, [rbp + 0x8c0]
0x1400dff39: call 0x140be24b0
0x1400dff3e: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400dff46: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400dff4e: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400dff54: movdqa xmm9, xmmword ptr [rsp + 0x60]
0x1400dff5b: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400dff61: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400dff68: add rsp, 0xa8
0x1400dff6f: pop rbx
0x1400dff70: pop rdi
0x1400dff71: pop rsi
0x1400dff72: pop r12
0x1400dff74: pop r13
0x1400dff76: pop r14
0x1400dff78: pop r15
0x1400dff7a: pop rbp
0x1400dff7b: ret 
0x1400dff7c: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400dff84: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400dff8c: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400dff92: movdqa xmm9, xmmword ptr [rsp + 0x60]
0x1400dff99: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400dff9f: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400dffa6: add rsp, 0xa8
0x1400dffad: pop rbx
0x1400dffae: pop rdi
0x1400dffaf: pop rsi
0x1400dffb0: pop r12
0x1400dffb2: pop r13
0x1400dffb4: pop r14
0x1400dffb6: pop r15
0x1400dffb8: pop rbp
0x1400dffb9: ret 
0x1400dffba: nop word ptr [rax + rax]
0x1400dffc0: mov qword ptr [rsp + 0x10], rdx
0x1400dffc5: push rbp
0x1400dffc6: push r15
0x1400dffc8: push r14
0x1400dffca: push r13
0x1400dffcc: push r12
0x1400dffce: push rsi
0x1400dffcf: push rdi
0x1400dffd0: push rbx
0x1400dffd1: sub rsp, 0xa8
0x1400dffd8: lea rbp, [rdx + 0x80]
0x1400dffdf: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400dffe6: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400dffec: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400dfff3: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400dfff9: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e0001: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400e0009: cmp dword ptr [rbp + 0x9a0], 2
0x1400e0010: jne 0x1400e0069
0x1400e0012: mov rax, qword ptr [rbp + 0x9a8]
0x1400e0019: lock dec qword ptr [rax]
0x1400e001d: jne 0x1400e002b
0x1400e001f: lea rcx, [rbp + 0x9a8]
0x1400e0026: call 0x14064f9d0
0x1400e002b: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400e0033: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e003b: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e0041: movdqa xmm9, xmmword ptr [rsp + 0x60]
0x1400e0048: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e004e: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400e0055: add rsp, 0xa8
0x1400e005c: pop rbx
0x1400e005d: pop rdi
0x1400e005e: pop rsi
0x1400e005f: pop r12
0x1400e0061: pop r13
0x1400e0063: pop r14
0x1400e0065: pop r15
0x1400e0067: pop rbp
0x1400e0068: ret 
0x1400e0069: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400e0071: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e0079: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e007f: movdqa xmm9, xmmword ptr [rsp + 0x60]
0x1400e0086: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e008c: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400e0093: add rsp, 0xa8
0x1400e009a: pop rbx
0x1400e009b: pop rdi
0x1400e009c: pop rsi
0x1400e009d: pop r12
0x1400e009f: pop r13
0x1400e00a1: pop r14
0x1400e00a3: pop r15
0x1400e00a5: pop rbp
0x1400e00a6: ret 
0x1400e00a7: nop word ptr [rax + rax]
0x1400e00b0: mov qword ptr [rsp + 0x10], rdx
0x1400e00b5: push rbp
0x1400e00b6: push r15
0x1400e00b8: push r14
0x1400e00ba: push r13
0x1400e00bc: push r12
0x1400e00be: push rsi
0x1400e00bf: push rdi
0x1400e00c0: push rbx
0x1400e00c1: sub rsp, 0xa8
0x1400e00c8: lea rbp, [rdx + 0x80]
0x1400e00cf: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400e00d6: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400e00dc: movaps xmmword ptr [rsp + 0x60], xmm9
0x1400e00e2: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400e00e8: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e00f0: movdqa xmmword ptr [rsp + 0x90], xmm6
0x1400e00f9: cmp byte ptr [rbp + 0xa74], 0
0x1400e0100: jne 0x1400e013e
0x1400e0102: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400e010a: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e0112: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e0118: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400e011e: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e0124: movaps xmm11, xmmword ptr [rsp + 0x40]
0x1400e012a: add rsp, 0xa8
0x1400e0131: pop rbx
0x1400e0132: pop rdi
0x1400e0133: pop rsi
0x1400e0134: pop r12
0x1400e0136: pop r13
0x1400e0138: pop r14
0x1400e013a: pop r15
0x1400e013c: pop rbp
0x1400e013d: ret 
0x1400e013e: lea rcx, [rbp - 0x38]
0x1400e0142: call 0x140603110
0x1400e0147: jmp 0x1400e0102
0x1400e0149: nop dword ptr [rax]
0x1400e0150: mov qword ptr [rsp + 0x10], rdx
0x1400e0155: push rbp
0x1400e0156: push r15
0x1400e0158: push r14
0x1400e015a: push r13
0x1400e015c: push r12
0x1400e015e: push rsi
0x1400e015f: push rdi
0x1400e0160: push rbx
0x1400e0161: sub rsp, 0xa8
0x1400e0168: lea rbp, [rdx + 0x80]
0x1400e016f: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400e0176: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400e017c: movaps xmmword ptr [rsp + 0x60], xmm9
0x1400e0182: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400e0188: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e0190: movdqa xmmword ptr [rsp + 0x90], xmm6
0x1400e0199: lea rcx, [rbp + 0x9a0]
0x1400e01a0: call 0x1405d21b0
0x1400e01a5: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400e01ad: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e01b5: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e01bb: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400e01c1: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e01c7: movaps xmm11, xmmword ptr [rsp + 0x40]
0x1400e01cd: add rsp, 0xa8
0x1400e01d4: pop rbx
0x1400e01d5: pop rdi
0x1400e01d6: pop rsi
0x1400e01d7: pop r12
0x1400e01d9: pop r13
0x1400e01db: pop r14
0x1400e01dd: pop r15
0x1400e01df: pop rbp
0x1400e01e0: ret 
0x1400e01e1: nop word ptr cs:[rax + rax]
0x1400e01f0: mov qword ptr [rsp + 0x10], rdx
0x1400e01f5: push rbp
0x1400e01f6: push r15
0x1400e01f8: push r14
0x1400e01fa: push r13
0x1400e01fc: push r12
0x1400e01fe: push rsi
0x1400e01ff: push rdi
0x1400e0200: push rbx
0x1400e0201: sub rsp, 0xa8
0x1400e0208: lea rbp, [rdx + 0x80]
0x1400e020f: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400e0216: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400e021c: movaps xmmword ptr [rsp + 0x60], xmm9
0x1400e0222: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400e0228: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e0230: movdqa xmmword ptr [rsp + 0x90], xmm6
0x1400e0239: lea rcx, [rbp + 0x7c0]
0x1400e0240: call 0x1405e94a0
0x1400e0245: movdqa xmm6, xmmword ptr [rsp + 0x90]
0x1400e024e: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e0256: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e025c: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400e0262: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e0268: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400e026f: add rsp, 0xa8
0x1400e0276: pop rbx
0x1400e0277: pop rdi
0x1400e0278: pop rsi
0x1400e0279: pop r12
0x1400e027b: pop r13
0x1400e027d: pop r14
0x1400e027f: pop r15
0x1400e0281: pop rbp
0x1400e0282: ret 
0x1400e0283: nop word ptr cs:[rax + rax]
0x1400e0290: mov qword ptr [rsp + 0x10], rdx
0x1400e0295: push rbp
0x1400e0296: push r15
0x1400e0298: push r14
0x1400e029a: push r13
0x1400e029c: push r12
0x1400e029e: push rsi
0x1400e029f: push rdi
0x1400e02a0: push rbx
0x1400e02a1: sub rsp, 0xa8
0x1400e02a8: lea rbp, [rdx + 0x80]
0x1400e02af: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400e02b6: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400e02bc: movaps xmmword ptr [rsp + 0x60], xmm9
0x1400e02c2: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400e02c8: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e02d0: movdqa xmmword ptr [rsp + 0x90], xmm6
0x1400e02d9: mov rsi, qword ptr [rbp + 0xa58]
0x1400e02e0: mov qword ptr [rbp + 0xa48], rsi
0x1400e02e7: mov rdi, qword ptr [rbp + 0xa68]
0x1400e02ee: mov qword ptr [rbp + 0xa40], rdi
0x1400e02f5: mov ebx, dword ptr [rbp + 0xa60]
0x1400e02fb: mov byte ptr [rbp + 0xa75], bl
0x1400e0301: lea rcx, [rbp + 0x3b0]
0x1400e0308: call 0x1400dcef0
0x1400e030d: mov qword ptr [rbp + 0xa48], rsi
0x1400e0314: mov qword ptr [rbp + 0xa40], rdi
0x1400e031b: mov byte ptr [rbp + 0xa75], bl
0x1400e0321: movdqa xmm6, xmmword ptr [rsp + 0x90]
0x1400e032a: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e0332: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e0338: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400e033e: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e0344: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400e034b: add rsp, 0xa8
0x1400e0352: pop rbx
0x1400e0353: pop rdi
0x1400e0354: pop rsi
0x1400e0355: pop r12
0x1400e0357: pop r13
0x1400e0359: pop r14
0x1400e035b: pop r15
0x1400e035d: pop rbp
0x1400e035e: ret 
0x1400e035f: nop 
0x1400e0360: mov qword ptr [rsp + 0x10], rdx
0x1400e0365: push rbp
0x1400e0366: push r15
0x1400e0368: push r14
0x1400e036a: push r13
0x1400e036c: push r12
0x1400e036e: push rsi
0x1400e036f: push rdi
0x1400e0370: push rbx
0x1400e0371: sub rsp, 0xa8
0x1400e0378: lea rbp, [rdx + 0x80]
0x1400e037f: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400e0386: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400e038c: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400e0393: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400e0399: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e03a1: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400e03a9: lea rcx, [rbp + 0x8f0]
0x1400e03b0: call 0x140605ea0
0x1400e03b5: mov byte ptr [rbp + 0xa74], 1
0x1400e03bc: movdqa xmm6, xmmword ptr [rsp + 0x90]
0x1400e03c5: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e03cd: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e03d3: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400e03d9: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e03df: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400e03e6: add rsp, 0xa8
0x1400e03ed: pop rbx
0x1400e03ee: pop rdi
0x1400e03ef: pop rsi
0x1400e03f0: pop r12
0x1400e03f2: pop r13
0x1400e03f4: pop r14
0x1400e03f6: pop r15
0x1400e03f8: pop rbp
0x1400e03f9: ret 
0x1400e03fa: nop word ptr [rax + rax]
0x1400e0400: mov qword ptr [rsp + 0x10], rdx
0x1400e0405: push rbp
0x1400e0406: push r15
0x1400e0408: push r14
0x1400e040a: push r13
0x1400e040c: push r12
0x1400e040e: push rsi
0x1400e040f: push rdi
0x1400e0410: push rbx
0x1400e0411: sub rsp, 0xa8
0x1400e0418: lea rbp, [rdx + 0x80]
0x1400e041f: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400e0426: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400e042c: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400e0433: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400e0439: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e0441: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400e0449: lea rcx, [rbp + 0x950]
0x1400e0450: call 0x1405fc780
0x1400e0455: mov byte ptr [rbp + 0xa74], 0
0x1400e045c: movdqa xmm6, xmmword ptr [rsp + 0x90]
0x1400e0465: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e046d: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e0473: movaps xmm9, xmmword ptr [rsp + 0x60]
0x1400e0479: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e047f: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400e0486: add rsp, 0xa8
0x1400e048d: pop rbx
0x1400e048e: pop rdi
0x1400e048f: pop rsi
0x1400e0490: pop r12
0x1400e0492: pop r13
0x1400e0494: pop r14
0x1400e0496: pop r15
0x1400e0498: pop rbp
0x1400e0499: ret 
0x1400e049a: nop word ptr [rax + rax]
0x1400e04a0: mov qword ptr [rsp + 0x10], rdx
0x1400e04a5: push rbp
0x1400e04a6: push r15
0x1400e04a8: push r14
0x1400e04aa: push r13
0x1400e04ac: push r12
0x1400e04ae: push rsi
0x1400e04af: push rdi
0x1400e04b0: push rbx
0x1400e04b1: sub rsp, 0xa8
0x1400e04b8: lea rbp, [rdx + 0x80]
0x1400e04bf: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400e04c6: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400e04cc: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400e04d3: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400e04d9: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e04e1: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400e04e9: lea rcx, [rbp + 0x3b0]
0x1400e04f0: call 0x1400dcef0
0x1400e04f5: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400e04fd: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e0505: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e050b: movdqa xmm9, xmmword ptr [rsp + 0x60]
0x1400e0512: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e0518: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400e051f: add rsp, 0xa8
0x1400e0526: pop rbx
0x1400e0527: pop rdi
0x1400e0528: pop rsi
0x1400e0529: pop r12
0x1400e052b: pop r13
0x1400e052d: pop r14
0x1400e052f: pop r15
0x1400e0531: pop rbp
0x1400e0532: ret 
0x1400e0533: nop word ptr cs:[rax + rax]
0x1400e0540: mov qword ptr [rsp + 0x10], rdx
0x1400e0545: push rbp
0x1400e0546: push r15
0x1400e0548: push r14
0x1400e054a: push r13
0x1400e054c: push r12
0x1400e054e: push rsi
0x1400e054f: push rdi
0x1400e0550: push rbx
0x1400e0551: sub rsp, 0xa8
0x1400e0558: lea rbp, [rdx + 0x80]
0x1400e055f: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400e0566: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400e056c: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400e0573: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400e0579: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e0581: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400e0589: mov byte ptr [rbp + 0xa73], 1
0x1400e0590: lea rcx, [rbp + 0x3b0]
0x1400e0597: call 0x1400dcef0
0x1400e059c: mov byte ptr [rbp + 0xa73], 1
0x1400e05a3: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400e05ab: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e05b3: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e05b9: movdqa xmm9, xmmword ptr [rsp + 0x60]
0x1400e05c0: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e05c6: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400e05cd: add rsp, 0xa8
0x1400e05d4: pop rbx
0x1400e05d5: pop rdi
0x1400e05d6: pop rsi
0x1400e05d7: pop r12
0x1400e05d9: pop r13
0x1400e05db: pop r14
0x1400e05dd: pop r15
0x1400e05df: pop rbp
0x1400e05e0: ret 
0x1400e05e1: nop word ptr cs:[rax + rax]
0x1400e05f0: mov qword ptr [rsp + 0x10], rdx
0x1400e05f5: push rbp
0x1400e05f6: push r15
0x1400e05f8: push r14
0x1400e05fa: push r13
0x1400e05fc: push r12
0x1400e05fe: push rsi
0x1400e05ff: push rdi
0x1400e0600: push rbx
0x1400e0601: sub rsp, 0xa8
0x1400e0608: lea rbp, [rdx + 0x80]
0x1400e060f: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400e0616: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400e061c: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400e0623: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400e0629: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e0631: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400e0639: mov byte ptr [rbp + 0xa77], 1
0x1400e0640: mov byte ptr [rbp + 0xa76], 1
0x1400e0647: lea rcx, [rbp + 0x3b0]
0x1400e064e: call 0x1406048f0
0x1400e0653: mov byte ptr [rbp + 0xa77], 1
0x1400e065a: mov byte ptr [rbp + 0xa76], 1
0x1400e0661: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400e0669: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e0671: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e0677: movdqa xmm9, xmmword ptr [rsp + 0x60]
0x1400e067e: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e0684: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400e068b: add rsp, 0xa8
0x1400e0692: pop rbx
0x1400e0693: pop rdi
0x1400e0694: pop rsi
0x1400e0695: pop r12
0x1400e0697: pop r13
0x1400e0699: pop r14
0x1400e069b: pop r15
0x1400e069d: pop rbp
0x1400e069e: ret 
0x1400e069f: nop 
0x1400e06a0: mov qword ptr [rsp + 0x10], rdx
0x1400e06a5: push rbp
0x1400e06a6: push r15
0x1400e06a8: push r14
0x1400e06aa: push r13
0x1400e06ac: push r12
0x1400e06ae: push rsi
0x1400e06af: push rdi
0x1400e06b0: push rbx
0x1400e06b1: sub rsp, 0xa8
0x1400e06b8: lea rbp, [rdx + 0x80]
0x1400e06bf: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400e06c6: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400e06cc: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400e06d3: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400e06d9: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e06e1: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400e06e9: mov byte ptr [rbp + 0xa77], 1
0x1400e06f0: mov byte ptr [rbp + 0xa76], 1
0x1400e06f7: lea rcx, [rbp + 0x7c0]
0x1400e06fe: call 0x1406048f0
0x1400e0703: mov byte ptr [rbp + 0xa77], 1
0x1400e070a: mov byte ptr [rbp + 0xa76], 1
0x1400e0711: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400e0719: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e0721: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e0727: movdqa xmm9, xmmword ptr [rsp + 0x60]
0x1400e072e: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e0734: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400e073b: add rsp, 0xa8
0x1400e0742: pop rbx
0x1400e0743: pop rdi
0x1400e0744: pop rsi
0x1400e0745: pop r12
0x1400e0747: pop r13
0x1400e0749: pop r14
0x1400e074b: pop r15
0x1400e074d: pop rbp
0x1400e074e: ret 
0x1400e074f: nop 
0x1400e0750: mov qword ptr [rsp + 0x10], rdx
0x1400e0755: push rbp
0x1400e0756: push r15
0x1400e0758: push r14
0x1400e075a: push r13
0x1400e075c: push r12
0x1400e075e: push rsi
0x1400e075f: push rdi
0x1400e0760: push rbx
0x1400e0761: sub rsp, 0xa8
0x1400e0768: lea rbp, [rdx + 0x80]
0x1400e076f: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400e0776: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400e077c: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400e0783: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400e0789: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e0791: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400e0799: mov byte ptr [rbp + 0xa77], 1
0x1400e07a0: mov byte ptr [rbp + 0xa76], 0
0x1400e07a7: lea rcx, [rbp + 0x3b0]
0x1400e07ae: call 0x140be24b0
0x1400e07b3: mov byte ptr [rbp + 0xa77], 1
0x1400e07ba: mov byte ptr [rbp + 0xa76], 0
0x1400e07c1: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400e07c9: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e07d1: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e07d7: movdqa xmm9, xmmword ptr [rsp + 0x60]
0x1400e07de: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e07e4: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400e07eb: add rsp, 0xa8
0x1400e07f2: pop rbx
0x1400e07f3: pop rdi
0x1400e07f4: pop rsi
0x1400e07f5: pop r12
0x1400e07f7: pop r13
0x1400e07f9: pop r14
0x1400e07fb: pop r15
0x1400e07fd: pop rbp
0x1400e07fe: ret 
0x1400e07ff: nop 
0x1400e0800: mov qword ptr [rsp + 0x10], rdx
0x1400e0805: push rbp
0x1400e0806: push r15
0x1400e0808: push r14
0x1400e080a: push r13
0x1400e080c: push r12
0x1400e080e: push rsi
0x1400e080f: push rdi
0x1400e0810: push rbx
0x1400e0811: sub rsp, 0xa8
0x1400e0818: lea rbp, [rdx + 0x80]
0x1400e081f: movdqa xmmword ptr [rsp + 0x40], xmm11
0x1400e0826: movaps xmmword ptr [rsp + 0x50], xmm10
0x1400e082c: movdqa xmmword ptr [rsp + 0x60], xmm9
0x1400e0833: movaps xmmword ptr [rsp + 0x70], xmm8
0x1400e0839: movaps xmmword ptr [rsp + 0x80], xmm7
0x1400e0841: movaps xmmword ptr [rsp + 0x90], xmm6
0x1400e0849: mov byte ptr [rbp + 0xa77], 0
0x1400e0850: mov byte ptr [rbp + 0xa76], 0
0x1400e0857: lea rcx, [rbp + 0x3b0]
0x1400e085e: call 0x140be24b0
0x1400e0863: mov byte ptr [rbp + 0xa77], 0
0x1400e086a: mov byte ptr [rbp + 0xa76], 0
0x1400e0871: movaps xmm6, xmmword ptr [rsp + 0x90]
0x1400e0879: movaps xmm7, xmmword ptr [rsp + 0x80]
0x1400e0881: movaps xmm8, xmmword ptr [rsp + 0x70]
0x1400e0887: movdqa xmm9, xmmword ptr [rsp + 0x60]
0x1400e088e: movaps xmm10, xmmword ptr [rsp + 0x50]
0x1400e0894: movdqa xmm11, xmmword ptr [rsp + 0x40]
0x1400e089b: add rsp, 0xa8
0x1400e08a2: pop rbx
0x1400e08a3: pop rdi
0x1400e08a4: pop rsi
0x1400e08a5: pop r12
0x1400e08a7: pop r13
0x1400e08a9: pop r14
0x1400e08ab: pop r15
0x1400e08ad: pop rbp
0x1400e08ae: ret 
