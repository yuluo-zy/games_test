1417cd0a0: push r15
1417cd0a2: push r14
1417cd0a4: push r13
1417cd0a6: push r12
1417cd0a8: push rsi
1417cd0a9: push rdi
1417cd0aa: push rbp
1417cd0ab: push rbx
1417cd0ac: sub rsp, 0x78
1417cd0b0: movaps xmmword ptr [rsp + 0x60], xmm7
1417cd0b5: movaps xmmword ptr [rsp + 0x50], xmm6
1417cd0ba: mov rdi, r8
1417cd0bd: mov rsi, rdx
1417cd0c0: mov rbx, rcx
1417cd0c3: mov rax, qword ptr [rsp + 0x118]
1417cd0cb: mov r15, qword ptr [rax]
1417cd0ce: mov r12, qword ptr [r9]
1417cd0d1: test r12, r12
1417cd0d4: je 0x1417cd112
1417cd0d6: call 0x142923570
1417cd0db: mov r14, qword ptr [r12]
1417cd0df: lock inc qword ptr [r14]
1417cd0e3: jle 0x1417ce33f
1417cd0e9: mov qword ptr [rsp + 0x20], r14
1417cd0ee: mov qword ptr [rsp + 0x28], rax
1417cd0f3: mov rcx, rax
1417cd0f6: call 0x1429234f0
1417cd0fb: mov ebp, eax
1417cd0fd: lock dec qword ptr [r14]
1417cd101: jne 0x1417cd10d
1417cd103: lea rcx, [rsp + 0x20]
1417cd108: call 0x140b85350
1417cd10d: test bpl, bpl
1417cd110: jne 0x1417cd11c
1417cd112: mov rax, qword ptr [rdi]
1417cd115: cmp qword ptr [rax + 0x38], 0
1417cd11a: jne 0x1417cd126
1417cd11c: mov rax, qword ptr [rsi]
1417cd11f: cmp qword ptr [rax + 0x38], 0
1417cd124: je 0x1417cd147
1417cd126: mov rax, qword ptr [rsp + 0xf0]
1417cd12e: mov ecx, dword ptr [rax + 0x1c]
1417cd131: mov rdx, qword ptr [rax + 0x10]
1417cd135: mov dword ptr [rdx], ecx
1417cd137: mov r14, qword ptr [rax]
1417cd13a: call 0x140d635e0
1417cd13f: mov qword ptr [r14 + 0x10], rax
1417cd143: mov dword ptr [r14 + 0x18], edx
1417cd147: mov rcx, qword ptr [rsp + 0xe8]
1417cd14f: mov rax, qword ptr [rsp + 0xe0]
1417cd157: mov edx, dword ptr [rcx + 0x1c]
1417cd15a: mov r8, qword ptr [rcx + 0x10]
1417cd15e: mov dword ptr [r8], edx
1417cd161: mov r14, qword ptr [rcx]
1417cd164: mov rdi, qword ptr [rdi]
1417cd167: cmp qword ptr [rdi + 0x18], 0
1417cd16c: je 0x1417cd1fc
1417cd172: mov rcx, qword ptr [rdi]
1417cd175: mov rdx, qword ptr [rdi + 8]
1417cd179: movabs r8, 0x258849ed64261a40
1417cd183: and r8, rdx
1417cd186: xor r9d, r9d
1417cd189: movdqa xmm0, xmmword ptr [rip + 0x118336f]
1417cd191: pcmpeqd xmm1, xmm1
1417cd195: movdqu xmm2, xmmword ptr [rcx + r8]
1417cd19b: movdqa xmm3, xmm2
1417cd19f: pcmpeqb xmm3, xmm0
1417cd1a3: pmovmskb r10d, xmm3
1417cd1a8: test r10d, r10d
1417cd1ab: je 0x1417cd1da
1417cd1ad: nop dword ptr [rax]
1417cd1b0: tzcnt r11d, r10d
1417cd1b5: add r11, r8
1417cd1b8: and r11, rdx
1417cd1bb: shl r11, 2
1417cd1bf: mov r13, rcx
1417cd1c2: sub r13, r11
1417cd1c5: cmp word ptr [r13 - 4], 0
1417cd1cb: je 0x1417cd1f8
1417cd1cd: lea r11d, [r10 - 1]
1417cd1d1: and r11w, r10w
1417cd1d5: mov r10d, r11d
1417cd1d8: jne 0x1417cd1b0
1417cd1da: pcmpeqb xmm2, xmm1
1417cd1de: pmovmskb r10d, xmm2
1417cd1e3: test r10d, r10d
1417cd1e6: jne 0x1417cd1fc
1417cd1e8: add r8, r9
1417cd1eb: add r8, 0x10
1417cd1ef: add r9, 0x10
1417cd1f3: and r8, rdx
1417cd1f6: jmp 0x1417cd195
1417cd1f8: mov cl, 1
1417cd1fa: jmp 0x1417cd1fe
1417cd1fc: xor ecx, ecx
1417cd1fe: or byte ptr [r14 + 0x88], cl
1417cd205: mov rax, qword ptr [rax]
1417cd208: mov rcx, qword ptr [rax]
1417cd20b: cmp rcx, 2
1417cd20f: je 0x1417cd4e0
1417cd215: cmp ecx, 0xa
1417cd218: jne 0x1417cd224
1417cd21a: cmp byte ptr [rax + 8], 1
1417cd21e: jne 0x1417cd4e0
1417cd224: mov rsi, qword ptr [rsi]
1417cd227: mov dword ptr [rsp + 0x20], 0x73
1417cd22f: lea rcx, [rsi + 0x20]
1417cd233: lea rdx, [rsp + 0x20]
1417cd238: call 0x14089aef0
1417cd23d: test rax, rax
1417cd240: setne al
1417cd243: or byte ptr [r14 + 0x9a], al
1417cd24a: mov eax, dword ptr [r15 + 0x150]
1417cd251: cmp eax, 0xc3
1417cd256: je 0x1417cdf36
1417cd25c: mov rcx, qword ptr [r15 + 0x154]
1417cd263: mov dword ptr [rsp + 0x20], eax
1417cd267: mov qword ptr [rsp + 0x24], rcx
1417cd26c: lea rdx, [rsp + 0x20]
1417cd271: mov rcx, rsi
1417cd274: call 0x14089aef0
1417cd279: test rax, rax
1417cd27c: setne al
1417cd27f: or byte ptr [r14 + 0x9a], al
1417cd286: mov eax, dword ptr [r15 + 0x15c]
1417cd28d: cmp eax, 0xc3
1417cd292: je 0x1417cdf51
1417cd298: mov rcx, qword ptr [r15 + 0x160]
1417cd29f: mov dword ptr [rsp + 0x20], eax
1417cd2a3: mov qword ptr [rsp + 0x24], rcx
1417cd2a8: lea rdx, [rsp + 0x20]
1417cd2ad: mov rcx, rsi
1417cd2b0: call 0x14089aef0
1417cd2b5: test rax, rax
1417cd2b8: setne al
1417cd2bb: or byte ptr [r14 + 0x9b], al
1417cd2c2: mov eax, dword ptr [r15 + 0x168]
1417cd2c9: cmp eax, 0xc3
1417cd2ce: je 0x1417cdf6c
1417cd2d4: mov rcx, qword ptr [r15 + 0x16c]
1417cd2db: mov dword ptr [rsp + 0x20], eax
1417cd2df: mov qword ptr [rsp + 0x24], rcx
1417cd2e4: lea rdx, [rsp + 0x20]
1417cd2e9: mov rcx, rsi
1417cd2ec: call 0x14089aef0
1417cd2f1: test rax, rax
1417cd2f4: setne al
1417cd2f7: or byte ptr [r14 + 0x9c], al
1417cd2fe: mov eax, dword ptr [r15 + 0x174]
1417cd305: cmp eax, 0xc3
1417cd30a: je 0x1417cdf87
1417cd310: mov rcx, qword ptr [r15 + 0x178]
1417cd317: mov dword ptr [rsp + 0x20], eax
1417cd31b: mov qword ptr [rsp + 0x24], rcx
1417cd320: lea rdx, [rsp + 0x20]
1417cd325: mov rcx, rsi
1417cd328: call 0x14089aef0
1417cd32d: test rax, rax
1417cd330: setne al
1417cd333: or byte ptr [r14 + 0x9d], al
1417cd33a: mov eax, dword ptr [r15 + 0x180]
1417cd341: cmp eax, 0xc3
1417cd346: je 0x1417cdfa2
1417cd34c: mov rcx, qword ptr [r15 + 0x184]
1417cd353: mov dword ptr [rsp + 0x20], eax
1417cd357: mov qword ptr [rsp + 0x24], rcx
1417cd35c: lea rdx, [rsp + 0x20]
1417cd361: mov rcx, rsi
1417cd364: call 0x14089aef0
1417cd369: test rax, rax
1417cd36c: setne al
1417cd36f: or byte ptr [r14 + 0x9e], al
1417cd376: mov eax, dword ptr [r15 + 0x18c]
1417cd37d: cmp eax, 0xc3
1417cd382: je 0x1417cdfbd
1417cd388: mov rcx, qword ptr [r15 + 0x190]
1417cd38f: mov dword ptr [rsp + 0x20], eax
1417cd393: mov qword ptr [rsp + 0x24], rcx
1417cd398: lea rdx, [rsp + 0x20]
1417cd39d: mov rcx, rsi
1417cd3a0: call 0x14089aef0
1417cd3a5: test rax, rax
1417cd3a8: setne al
1417cd3ab: or byte ptr [r14 + 0x9f], al
1417cd3b2: mov eax, dword ptr [r15 + 0x198]
1417cd3b9: cmp eax, 0xc3
1417cd3be: je 0x1417cdfd8
1417cd3c4: mov rcx, qword ptr [r15 + 0x19c]
1417cd3cb: mov dword ptr [rsp + 0x20], eax
1417cd3cf: mov qword ptr [rsp + 0x24], rcx
1417cd3d4: lea rdx, [rsp + 0x20]
1417cd3d9: mov rcx, rsi
1417cd3dc: call 0x14089aef0
1417cd3e1: test rax, rax
1417cd3e4: setne al
1417cd3e7: or byte ptr [r14 + 0xa0], al
1417cd3ee: mov eax, dword ptr [r15 + 0x1a4]
1417cd3f5: cmp eax, 0xc3
1417cd3fa: je 0x1417cdff3
1417cd400: mov rcx, qword ptr [r15 + 0x1a8]
1417cd407: mov dword ptr [rsp + 0x20], eax
1417cd40b: mov qword ptr [rsp + 0x24], rcx
1417cd410: lea rdx, [rsp + 0x20]
1417cd415: mov rcx, rsi
1417cd418: call 0x14089aef0
1417cd41d: test rax, rax
1417cd420: setne al
1417cd423: or byte ptr [r14 + 0xa1], al
1417cd42a: test r12, r12
1417cd42d: je 0x1417ce005
1417cd433: mov eax, dword ptr [r15 + 0x138]
1417cd43a: cmp eax, 0xc3
1417cd43f: jne 0x1417ce085
1417cd445: xor eax, eax
1417cd447: or byte ptr [r14 + 0xa7], al
1417cd44e: mov eax, dword ptr [r15 + 0x144]
1417cd455: cmp eax, 0xc3
1417cd45a: je 0x1417ce0c1
1417cd460: mov rcx, qword ptr [r15 + 0x148]
1417cd467: mov dword ptr [rsp + 0x20], eax
1417cd46b: mov qword ptr [rsp + 0x24], rcx
1417cd470: lea rdx, [rsp + 0x20]
1417cd475: mov rcx, rsi
1417cd478: call 0x14089aef0
1417cd47d: test rax, rax
1417cd480: setne al
1417cd483: or byte ptr [r14 + 0xa8], al
1417cd48a: mov eax, dword ptr [r15 + 0x1b0]
1417cd491: cmp eax, 0xc3
1417cd496: je 0x1417ce0dc
1417cd49c: mov rcx, qword ptr [r15 + 0x1b4]
1417cd4a3: mov dword ptr [rsp + 0x20], eax
1417cd4a7: mov qword ptr [rsp + 0x24], rcx
1417cd4ac: lea rdx, [rsp + 0x20]
1417cd4b1: mov rcx, rsi
1417cd4b4: call 0x14089aef0
1417cd4b9: test rax, rax
1417cd4bc: setne al
1417cd4bf: or byte ptr [r14 + 0xa9], al
1417cd4c6: mov rdi, qword ptr [rsp + 0x110]
1417cd4ce: cmp byte ptr [rip + 0x1e67803], 0
1417cd4d5: je 0x1417ce243
1417cd4db: jmp 0x1417ce0fa
1417cd4e0: mov rbx, qword ptr [rbx]
1417cd4e3: movss xmm6, dword ptr [rbx + 0x28]
1417cd4e8: mulss xmm6, dword ptr [rip + 0x11ffb44]
1417cd4f0: mov eax, dword ptr [r15]
1417cd4f3: mov rsi, qword ptr [rsi]
1417cd4f6: cmp eax, 0xc3
1417cd4fb: je 0x1417cd52c
1417cd4fd: mov rcx, qword ptr [r15 + 4]
1417cd501: mov dword ptr [rsp + 0x20], eax
1417cd505: mov qword ptr [rsp + 0x24], rcx
1417cd50a: lea rdx, [rsp + 0x20]
1417cd50f: mov rcx, rsi
1417cd512: call 0x14089aef0
1417cd517: test rax, rax
1417cd51a: je 0x1417cd52c
1417cd51c: movss xmm0, dword ptr [r14 + 0x6c]
1417cd522: subss xmm0, xmm6
1417cd526: movss dword ptr [r14 + 0x6c], xmm0
1417cd52c: mov eax, dword ptr [r15 + 0xc]
1417cd530: cmp eax, 0xc3
1417cd535: je 0x1417cd566
1417cd537: mov rcx, qword ptr [r15 + 0x10]
1417cd53b: mov dword ptr [rsp + 0x20], eax
1417cd53f: mov qword ptr [rsp + 0x24], rcx
1417cd544: lea rdx, [rsp + 0x20]
1417cd549: mov rcx, rsi
1417cd54c: call 0x14089aef0
1417cd551: test rax, rax
1417cd554: je 0x1417cd566
1417cd556: movss xmm0, dword ptr [r14 + 0x6c]
1417cd55c: addss xmm0, xmm6
1417cd560: movss dword ptr [r14 + 0x6c], xmm0
1417cd566: mov eax, dword ptr [r15 + 0x18]
1417cd56a: cmp eax, 0xc3
1417cd56f: je 0x1417cd5a0
1417cd571: mov rcx, qword ptr [r15 + 0x1c]
1417cd575: mov dword ptr [rsp + 0x20], eax
1417cd579: mov qword ptr [rsp + 0x24], rcx
1417cd57e: lea rdx, [rsp + 0x20]
1417cd583: mov rcx, rsi
1417cd586: call 0x14089aef0
1417cd58b: test rax, rax
1417cd58e: je 0x1417cd5a0
1417cd590: movss xmm0, dword ptr [r14 + 0x68]
1417cd596: subss xmm0, xmm6
1417cd59a: movss dword ptr [r14 + 0x68], xmm0
1417cd5a0: mov eax, dword ptr [r15 + 0x24]
1417cd5a4: cmp eax, 0xc3
1417cd5a9: je 0x1417cd5da
1417cd5ab: mov rcx, qword ptr [r15 + 0x28]
1417cd5af: mov dword ptr [rsp + 0x20], eax
1417cd5b3: mov qword ptr [rsp + 0x24], rcx
1417cd5b8: lea rdx, [rsp + 0x20]
1417cd5bd: mov rcx, rsi
1417cd5c0: call 0x14089aef0
1417cd5c5: test rax, rax
1417cd5c8: je 0x1417cd5da
1417cd5ca: movss xmm0, dword ptr [r14 + 0x68]
1417cd5d0: addss xmm0, xmm6
1417cd5d4: movss dword ptr [r14 + 0x68], xmm0
1417cd5da: mov eax, dword ptr [r15 + 0x3c]
1417cd5de: cmp eax, 0xc3
1417cd5e3: je 0x1417cd614
1417cd5e5: mov rcx, qword ptr [r15 + 0x40]
1417cd5e9: mov dword ptr [rsp + 0x20], eax
1417cd5ed: mov qword ptr [rsp + 0x24], rcx
1417cd5f2: lea rdx, [rsp + 0x20]
1417cd5f7: mov rcx, rsi
1417cd5fa: call 0x14089aef0
1417cd5ff: test rax, rax
1417cd602: je 0x1417cd614
1417cd604: movss xmm0, dword ptr [r14 + 0x70]
1417cd60a: subss xmm0, xmm6
1417cd60e: movss dword ptr [r14 + 0x70], xmm0
1417cd614: mov eax, dword ptr [r15 + 0x30]
1417cd618: cmp eax, 0xc3
1417cd61d: je 0x1417cd64e
1417cd61f: mov rcx, qword ptr [r15 + 0x34]
1417cd623: mov dword ptr [rsp + 0x20], eax
1417cd627: mov qword ptr [rsp + 0x24], rcx
1417cd62c: lea rdx, [rsp + 0x20]
1417cd631: mov rcx, rsi
1417cd634: call 0x14089aef0
1417cd639: test rax, rax
1417cd63c: je 0x1417cd64e
1417cd63e: movss xmm0, dword ptr [r14 + 0x70]
1417cd644: addss xmm0, xmm6
1417cd648: movss dword ptr [r14 + 0x70], xmm0
1417cd64e: mov eax, dword ptr [r15 + 0x60]
1417cd652: cmp eax, 0xc3
1417cd657: je 0x1417cd69a
1417cd659: mov rcx, qword ptr [r15 + 0x64]
1417cd65d: mov dword ptr [rsp + 0x20], eax
1417cd661: mov qword ptr [rsp + 0x24], rcx
1417cd666: lea rdx, [rsp + 0x20]
1417cd66b: mov rcx, rsi
1417cd66e: call 0x14089aef0
1417cd673: test rax, rax
1417cd676: je 0x1417cd69a
1417cd678: movss xmm0, dword ptr [rip + 0x1158250]
1417cd680: mulss xmm0, xmm6
1417cd684: movsd xmm1, qword ptr [r14 + 0x78]
1417cd68a: pxor xmm2, xmm2
1417cd68e: movss xmm2, xmm0
1417cd692: addps xmm2, xmm1
1417cd695: movlps qword ptr [r14 + 0x78], xmm2
1417cd69a: mov eax, dword ptr [r15 + 0x6c]
1417cd69e: cmp eax, 0xc3
1417cd6a3: je 0x1417cd6d8
1417cd6a5: mov rcx, qword ptr [r15 + 0x70]
1417cd6a9: mov dword ptr [rsp + 0x20], eax
1417cd6ad: mov qword ptr [rsp + 0x24], rcx
1417cd6b2: lea rdx, [rsp + 0x20]
1417cd6b7: mov rcx, rsi
1417cd6ba: call 0x14089aef0
1417cd6bf: test rax, rax
1417cd6c2: je 0x1417cd6d8
1417cd6c4: mulss xmm6, dword ptr [rip + 0x1211ab4]
1417cd6cc: addss xmm6, dword ptr [r14 + 0x78]
1417cd6d2: movss dword ptr [r14 + 0x78], xmm6
1417cd6d8: mov eax, dword ptr [r15 + 0x54]
1417cd6dc: cmp eax, 0xc3
1417cd6e1: je 0x1417cd72b
1417cd6e3: mov rcx, qword ptr [r15 + 0x58]
1417cd6e7: mov dword ptr [rsp + 0x20], eax
1417cd6eb: mov qword ptr [rsp + 0x24], rcx
1417cd6f0: lea rdx, [rsp + 0x20]
1417cd6f5: mov rcx, rsi
1417cd6f8: call 0x14089aef0
1417cd6fd: test rax, rax
1417cd700: je 0x1417cd72b
1417cd702: movss xmm0, dword ptr [rbx + 0x28]
1417cd707: mulss xmm0, dword ptr [rip + 0x1515621]
1417cd70f: movsd xmm1, qword ptr [r14 + 0x80]
1417cd718: pxor xmm2, xmm2
1417cd71c: movss xmm2, xmm0
1417cd720: addps xmm2, xmm1
1417cd723: movlps qword ptr [r14 + 0x80], xmm2
1417cd72b: mov r13, qword ptr [rsp + 0xf8]
1417cd733: mov eax, dword ptr [r15 + 0x48]
1417cd737: cmp eax, 0xc3
1417cd73c: je 0x1417cd77c
1417cd73e: mov rcx, qword ptr [r15 + 0x4c]
1417cd742: mov dword ptr [rsp + 0x20], eax
1417cd746: mov qword ptr [rsp + 0x24], rcx
1417cd74b: lea rdx, [rsp + 0x20]
1417cd750: mov rcx, rsi
1417cd753: call 0x14089aef0
1417cd758: test rax, rax
1417cd75b: je 0x1417cd77c
1417cd75d: movss xmm0, dword ptr [rbx + 0x28]
1417cd762: mulss xmm0, dword ptr [rip + 0x15155ca]
1417cd76a: addss xmm0, dword ptr [r14 + 0x80]
1417cd773: movss dword ptr [r14 + 0x80], xmm0
1417cd77c: mov rax, qword ptr [rsp + 0x108]
1417cd784: mov rcx, qword ptr [r13]
1417cd788: mov r11, qword ptr [r13 + 8]
1417cd78c: mov r13, qword ptr [rcx]
1417cd78f: xor ebx, ebx
1417cd791: mov r8, r13
1417cd794: sub r8, qword ptr [r11 + 0x18]
1417cd798: cmovb r8, rbx
1417cd79c: sub r13, qword ptr [r11 + 0x38]
1417cd7a0: cmovb r13, rbx
1417cd7a4: mov r9, r8
1417cd7a7: shl r9, 5
1417cd7ab: add r9, qword ptr [r11 + 8]
1417cd7af: mov rdx, qword ptr [r11 + 0x10]
1417cd7b3: sub rdx, r8
1417cd7b6: cmovb rdx, rbx
1417cd7ba: mov ebp, 8
1417cd7bf: cmovb r9, rbp
1417cd7c3: mov r10, r13
1417cd7c6: shl r10, 5
1417cd7ca: add r10, qword ptr [r11 + 0x28]
1417cd7ce: mov r8, qword ptr [r11 + 0x30]
1417cd7d2: sub r8, r13
1417cd7d5: cmovb r8, rbx
1417cd7d9: cmovb r10, rbp
1417cd7dd: mov r11, qword ptr [r11 + 0x40]
1417cd7e1: sub r11, r8
1417cd7e4: sub r11, rdx
1417cd7e7: mov qword ptr [rcx], r11
1417cd7ea: shl rdx, 5
1417cd7ee: add rdx, r9
1417cd7f1: shl r8, 5
1417cd7f5: add r8, r10
1417cd7f8: movss xmm0, dword ptr [rip + 0x1158104]
1417cd800: movss xmm1, dword ptr [rip + 0x1166dfc]
1417cd808: jmp 0x1417cd835
1417cd80a: nop word ptr [rax + rax]
1417cd810: movaps xmm2, xmm1
1417cd813: mulss xmm2, dword ptr [r9 + 0x14]
1417cd819: pxor xmm3, xmm3
1417cd81d: movss xmm3, xmm2
1417cd821: movsd xmm2, qword ptr [r14 + 0x78]
1417cd827: addps xmm2, xmm3
1417cd82a: movlps qword ptr [r14 + 0x78], xmm2
1417cd82f: mov r9, rbx
1417cd832: mov r10, r11
1417cd835: test r9, r9
1417cd838: je 0x1417cd860
1417cd83a: cmp r9, rdx
1417cd83d: je 0x1417cd860
1417cd83f: lea rbx, [r9 + 0x20]
1417cd843: mov r11, r10
1417cd846: inc qword ptr [rcx]
1417cd849: cmp byte ptr [r9 + 0x18], 0
1417cd84e: movaps xmm2, xmm0
1417cd851: jne 0x1417cd813
1417cd853: jmp 0x1417cd810
1417cd855: nop word ptr cs:[rax + rax]
1417cd860: test r10, r10
1417cd863: je 0x1417cd882
1417cd865: cmp r10, r8
1417cd868: je 0x1417cd882
1417cd86a: lea r11, [r10 + 0x20]
1417cd86e: xor ebx, ebx
1417cd870: mov r9, r10
1417cd873: inc qword ptr [rcx]
1417cd876: cmp byte ptr [r9 + 0x18], 0
1417cd87b: movaps xmm2, xmm0
1417cd87e: jne 0x1417cd813
1417cd880: jmp 0x1417cd810
1417cd882: mov qword ptr [rsp + 0x38], rdi
1417cd887: mov qword ptr [rsp + 0x40], r12
1417cd88c: mov qword ptr [rsp + 0x48], rsi
1417cd891: mov rcx, qword ptr [rax]
1417cd894: mov r11, qword ptr [rax + 8]
1417cd898: mov rax, qword ptr [rcx]
1417cd89b: xor ebx, ebx
1417cd89d: mov rdx, rax
1417cd8a0: sub rdx, qword ptr [r11 + 0x18]
1417cd8a4: cmovb rdx, rbx
1417cd8a8: sub rax, qword ptr [r11 + 0x38]
1417cd8ac: cmovb rax, rbx
1417cd8b0: mov r10, qword ptr [r11 + 0x10]
1417cd8b4: mov r8, qword ptr [r11 + 0x30]
1417cd8b8: mov qword ptr [rsp + 0x30], r8
1417cd8bd: sub r8, rax
1417cd8c0: cmovb r8, rbx
1417cd8c4: setbe r12b
1417cd8c8: mov rbp, r10
1417cd8cb: sub rbp, rdx
1417cd8ce: cmovb rbp, rbx
1417cd8d2: mov r13, qword ptr [r11 + 8]
1417cd8d6: mov rbx, qword ptr [r11 + 0x28]
1417cd8da: setbe dil
1417cd8de: mov r9, qword ptr [r11 + 0x40]
1417cd8e2: mov r11, r9
1417cd8e5: sub r11, r8
1417cd8e8: sub r11, rbp
1417cd8eb: mov qword ptr [rcx], r11
1417cd8ee: xorps xmm6, xmm6
1417cd8f1: xorps xmm7, xmm7
1417cd8f4: test dil, r12b
1417cd8f7: jne 0x1417cd959
1417cd8f9: mov r11, rax
1417cd8fc: shl r11, 4
1417cd900: add rbx, r11
1417cd903: shl r8, 4
1417cd907: mov r11, rdx
1417cd90a: shl r11, 4
1417cd90e: add r13, r11
1417cd911: shl rbp, 4
1417cd915: xor r11d, r11d
1417cd918: cmp r10, rdx
1417cd91b: mov edx, 8
1417cd920: cmovb r13, rdx
1417cd924: lea r10, [r13 + rbp - 0x10]
1417cd929: cmova r11, r10
1417cd92d: cmp qword ptr [rsp + 0x30], rax
1417cd932: cmovb rbx, rdx
1417cd936: lea rax, [rbx + r8 - 0x10]
1417cd93b: cmovbe rax, r11
1417cd93f: mov qword ptr [rcx], r9
1417cd942: movsd xmm0, qword ptr [rax + 8]
1417cd947: mulps xmm0, xmm0
1417cd94a: movshdup xmm1, xmm0
1417cd94e: addss xmm1, xmm0
1417cd952: xorps xmm7, xmm7
1417cd955: sqrtss xmm7, xmm1
1417cd959: movzx ebx, word ptr [r15 + 0x1c4]
1417cd961: movzx ebp, word ptr [r15 + 0x1c6]
1417cd969: mov rdi, qword ptr [rsp + 0x38]
1417cd96e: mov rcx, rdi
1417cd971: mov edx, ebx
1417cd973: mov r8d, ebp
1417cd976: call 0x14089a890
1417cd97b: test al, al
1417cd97d: mov rsi, qword ptr [rsp + 0x48]
1417cd982: mov r12, qword ptr [rsp + 0x40]
1417cd987: mov r13, qword ptr [rsp + 0x100]
1417cd98f: jne 0x1417cd9ac
1417cd991: mov rcx, rdi
1417cd994: mov edx, ebx
1417cd996: mov r8d, ebp
1417cd999: call 0x14089ab50
1417cd99e: movss xmm6, dword ptr [r13]
1417cd9a4: test al, al
1417cd9a6: je 0x1417cd9b2
1417cd9a8: addss xmm6, xmm7
1417cd9ac: movss dword ptr [r13], xmm6
1417cd9b2: mov rcx, rdi
1417cd9b5: mov edx, ebx
1417cd9b7: mov r8d, ebp
1417cd9ba: call 0x14089a9f0
1417cd9bf: movss xmm0, dword ptr [rip + 0x11ff6b1]
1417cd9c7: ucomiss xmm0, xmm6
1417cd9ca: seta bl
1417cd9cd: and bl, al
1417cd9cf: or bl, byte ptr [r14 + 0x89]
1417cd9d6: mov byte ptr [r14 + 0x89], bl
1417cd9dd: mov eax, dword ptr [r15 + 0x114]
1417cd9e4: cmp eax, 0xc3
1417cd9e9: jne 0x1417cd9ef
1417cd9eb: xor eax, eax
1417cd9ed: jmp 0x1417cda12
1417cd9ef: mov rcx, qword ptr [r15 + 0x118]
1417cd9f6: mov dword ptr [rsp + 0x20], eax
1417cd9fa: mov qword ptr [rsp + 0x24], rcx
1417cd9ff: lea rdx, [rsp + 0x20]
1417cda04: mov rcx, rsi
1417cda07: call 0x14089aef0
1417cda0c: test rax, rax
1417cda0f: setne al
1417cda12: or bl, al
1417cda14: mov byte ptr [r14 + 0x89], bl
1417cda1b: movzx edx, word ptr [r15 + 0x1bc]
1417cda23: movzx r8d, word ptr [r15 + 0x1be]
1417cda2b: mov rcx, rdi
1417cda2e: call 0x14089ab50
1417cda33: or byte ptr [r14 + 0x8a], al
1417cda3a: movzx edx, word ptr [r15 + 0x1c0]
1417cda42: movzx r8d, word ptr [r15 + 0x1c2]
1417cda4a: mov rcx, rdi
1417cda4d: call 0x14089ab50
1417cda52: or byte ptr [r14 + 0x8b], al
1417cda59: mov eax, dword ptr [r15 + 0x78]
1417cda5d: cmp eax, 0xc3
1417cda62: jne 0x1417cdd7b
1417cda68: xor eax, eax
1417cda6a: or byte ptr [r14 + 0x8c], al
1417cda71: mov eax, dword ptr [r15 + 0x84]
1417cda78: cmp eax, 0xc3
1417cda7d: je 0x1417cddb4
1417cda83: mov rcx, qword ptr [r15 + 0x88]
1417cda8a: mov dword ptr [rsp + 0x20], eax
1417cda8e: mov qword ptr [rsp + 0x24], rcx
1417cda93: lea rdx, [rsp + 0x20]
1417cda98: mov rcx, rsi
1417cda9b: call 0x14089aef0
1417cdaa0: test rax, rax
1417cdaa3: setne al
1417cdaa6: or byte ptr [r14 + 0x8d], al
1417cdaad: mov eax, dword ptr [r15 + 0x90]
1417cdab4: cmp eax, 0xc3
1417cdab9: je 0x1417cddcf
1417cdabf: mov rcx, qword ptr [r15 + 0x94]
1417cdac6: mov dword ptr [rsp + 0x20], eax
1417cdaca: mov qword ptr [rsp + 0x24], rcx
1417cdacf: lea rdx, [rsp + 0x20]
1417cdad4: mov rcx, rsi
1417cdad7: call 0x14089aef0
1417cdadc: test rax, rax
1417cdadf: setne al
1417cdae2: or byte ptr [r14 + 0x8e], al
1417cdae9: mov eax, dword ptr [r15 + 0x9c]
1417cdaf0: cmp eax, 0xc3
1417cdaf5: je 0x1417cddea
1417cdafb: mov rcx, qword ptr [r15 + 0xa0]
1417cdb02: mov dword ptr [rsp + 0x20], eax
1417cdb06: mov qword ptr [rsp + 0x24], rcx
1417cdb0b: lea rdx, [rsp + 0x20]
1417cdb10: mov rcx, rsi
1417cdb13: call 0x14089aef0
1417cdb18: test rax, rax
1417cdb1b: setne al
1417cdb1e: or byte ptr [r14 + 0x8f], al
1417cdb25: mov eax, dword ptr [r15 + 0xa8]
1417cdb2c: cmp eax, 0xc3
1417cdb31: je 0x1417cde05
1417cdb37: mov rcx, qword ptr [r15 + 0xac]
1417cdb3e: mov dword ptr [rsp + 0x20], eax
1417cdb42: mov qword ptr [rsp + 0x24], rcx
1417cdb47: lea rdx, [rsp + 0x20]
1417cdb4c: mov rcx, rsi
1417cdb4f: call 0x14089aef0
1417cdb54: test rax, rax
1417cdb57: setne al
1417cdb5a: or byte ptr [r14 + 0x91], al
1417cdb61: mov eax, dword ptr [r15 + 0xb4]
1417cdb68: cmp eax, 0xc3
1417cdb6d: je 0x1417cde20
1417cdb73: mov rcx, qword ptr [r15 + 0xb8]
1417cdb7a: mov dword ptr [rsp + 0x20], eax
1417cdb7e: mov qword ptr [rsp + 0x24], rcx
1417cdb83: lea rdx, [rsp + 0x20]
1417cdb88: mov rcx, rsi
1417cdb8b: call 0x14089aef0
1417cdb90: test rax, rax
1417cdb93: setne al
1417cdb96: or byte ptr [r14 + 0x90], al
1417cdb9d: mov eax, dword ptr [r15 + 0xc0]
1417cdba4: cmp eax, 0xc3
1417cdba9: je 0x1417cde3b
1417cdbaf: mov rcx, qword ptr [r15 + 0xc4]
1417cdbb6: mov dword ptr [rsp + 0x20], eax
1417cdbba: mov qword ptr [rsp + 0x24], rcx
1417cdbbf: lea rdx, [rsp + 0x20]
1417cdbc4: mov rcx, rsi
1417cdbc7: call 0x14089aef0
1417cdbcc: test rax, rax
1417cdbcf: setne al
1417cdbd2: or byte ptr [r14 + 0x92], al
1417cdbd9: mov eax, dword ptr [r15 + 0xcc]
1417cdbe0: cmp eax, 0xc3
1417cdbe5: je 0x1417cde56
1417cdbeb: mov rcx, qword ptr [r15 + 0xd0]
1417cdbf2: mov dword ptr [rsp + 0x20], eax
1417cdbf6: mov qword ptr [rsp + 0x24], rcx
1417cdbfb: lea rdx, [rsp + 0x20]
1417cdc00: mov rcx, rsi
1417cdc03: call 0x14089aef0
1417cdc08: test rax, rax
1417cdc0b: setne al
1417cdc0e: or byte ptr [r14 + 0x93], al
1417cdc15: mov eax, dword ptr [r15 + 0xd8]
1417cdc1c: cmp eax, 0xc3
1417cdc21: je 0x1417cde71
1417cdc27: mov rcx, qword ptr [r15 + 0xdc]
1417cdc2e: mov dword ptr [rsp + 0x20], eax
1417cdc32: mov qword ptr [rsp + 0x24], rcx
1417cdc37: lea rdx, [rsp + 0x20]
1417cdc3c: mov rcx, rsi
1417cdc3f: call 0x14089aef0
1417cdc44: test rax, rax
1417cdc47: setne al
1417cdc4a: or byte ptr [r14 + 0x94], al
1417cdc51: mov eax, dword ptr [r15 + 0xe4]
1417cdc58: cmp eax, 0xc3
1417cdc5d: je 0x1417cde8c
1417cdc63: mov rcx, qword ptr [r15 + 0xe8]
1417cdc6a: mov dword ptr [rsp + 0x20], eax
1417cdc6e: mov qword ptr [rsp + 0x24], rcx
1417cdc73: lea rdx, [rsp + 0x20]
1417cdc78: mov rcx, rsi
1417cdc7b: call 0x14089aef0
1417cdc80: test rax, rax
1417cdc83: setne al
1417cdc86: or byte ptr [r14 + 0x95], al
1417cdc8d: mov eax, dword ptr [r15 + 0xf0]
1417cdc94: cmp eax, 0xc3
1417cdc99: je 0x1417cdea7
1417cdc9f: mov rcx, qword ptr [r15 + 0xf4]
1417cdca6: mov dword ptr [rsp + 0x20], eax
1417cdcaa: mov qword ptr [rsp + 0x24], rcx
1417cdcaf: lea rdx, [rsp + 0x20]
1417cdcb4: mov rcx, rsi
1417cdcb7: call 0x14089aef0
1417cdcbc: test rax, rax
1417cdcbf: setne al
1417cdcc2: or byte ptr [r14 + 0x96], al
1417cdcc9: mov eax, dword ptr [r15 + 0x120]
1417cdcd0: cmp eax, 0xc3
1417cdcd5: je 0x1417cdec2
1417cdcdb: mov rcx, qword ptr [r15 + 0x124]
1417cdce2: mov dword ptr [rsp + 0x20], eax
1417cdce6: mov qword ptr [rsp + 0x24], rcx
1417cdceb: lea rdx, [rsp + 0x20]
1417cdcf0: mov rcx, rsi
1417cdcf3: call 0x14089aef0
1417cdcf8: test rax, rax
1417cdcfb: setne al
1417cdcfe: or byte ptr [r14 + 0x97], al
1417cdd05: mov eax, dword ptr [r15 + 0xfc]
1417cdd0c: cmp eax, 0xc3
1417cdd11: je 0x1417cdedd
1417cdd17: mov rcx, qword ptr [r15 + 0x100]
1417cdd1e: mov dword ptr [rsp + 0x20], eax
1417cdd22: mov qword ptr [rsp + 0x24], rcx
1417cdd27: lea rdx, [rsp + 0x20]
1417cdd2c: mov rcx, rsi
1417cdd2f: call 0x14089aef0
1417cdd34: test rax, rax
1417cdd37: setne al
1417cdd3a: or byte ptr [r14 + 0x98], al
1417cdd41: mov eax, dword ptr [r15 + 0x108]
1417cdd48: cmp eax, 0xc3
1417cdd4d: je 0x1417cdef8
1417cdd53: mov rcx, qword ptr [r15 + 0x10c]
1417cdd5a: mov dword ptr [rsp + 0x20], eax
1417cdd5e: mov qword ptr [rsp + 0x24], rcx
1417cdd63: lea rdx, [rsp + 0x20]
1417cdd68: mov rcx, rsi
1417cdd6b: call 0x14089aef0
1417cdd70: test rax, rax
1417cdd73: setne al
1417cdd76: jmp 0x1417cdefa
1417cdd7b: mov rcx, qword ptr [r15 + 0x7c]
1417cdd7f: mov dword ptr [rsp + 0x20], eax
1417cdd83: mov qword ptr [rsp + 0x24], rcx
1417cdd88: lea rdx, [rsp + 0x20]
1417cdd8d: mov rcx, rsi
1417cdd90: call 0x14089aef0
1417cdd95: test rax, rax
1417cdd98: setne al
1417cdd9b: or byte ptr [r14 + 0x8c], al
1417cdda2: mov eax, dword ptr [r15 + 0x84]
1417cdda9: cmp eax, 0xc3
1417cddae: jne 0x1417cda83
1417cddb4: xor eax, eax
1417cddb6: or byte ptr [r14 + 0x8d], al
1417cddbd: mov eax, dword ptr [r15 + 0x90]
1417cddc4: cmp eax, 0xc3
1417cddc9: jne 0x1417cdabf
1417cddcf: xor eax, eax
1417cddd1: or byte ptr [r14 + 0x8e], al
1417cddd8: mov eax, dword ptr [r15 + 0x9c]
1417cdddf: cmp eax, 0xc3
1417cdde4: jne 0x1417cdafb
1417cddea: xor eax, eax
1417cddec: or byte ptr [r14 + 0x8f], al
1417cddf3: mov eax, dword ptr [r15 + 0xa8]
1417cddfa: cmp eax, 0xc3
1417cddff: jne 0x1417cdb37
1417cde05: xor eax, eax
1417cde07: or byte ptr [r14 + 0x91], al
1417cde0e: mov eax, dword ptr [r15 + 0xb4]
1417cde15: cmp eax, 0xc3
1417cde1a: jne 0x1417cdb73
1417cde20: xor eax, eax
1417cde22: or byte ptr [r14 + 0x90], al
1417cde29: mov eax, dword ptr [r15 + 0xc0]
1417cde30: cmp eax, 0xc3
1417cde35: jne 0x1417cdbaf
1417cde3b: xor eax, eax
1417cde3d: or byte ptr [r14 + 0x92], al
1417cde44: mov eax, dword ptr [r15 + 0xcc]
1417cde4b: cmp eax, 0xc3
1417cde50: jne 0x1417cdbeb
1417cde56: xor eax, eax
1417cde58: or byte ptr [r14 + 0x93], al
1417cde5f: mov eax, dword ptr [r15 + 0xd8]
1417cde66: cmp eax, 0xc3
1417cde6b: jne 0x1417cdc27
1417cde71: xor eax, eax
1417cde73: or byte ptr [r14 + 0x94], al
1417cde7a: mov eax, dword ptr [r15 + 0xe4]
1417cde81: cmp eax, 0xc3
1417cde86: jne 0x1417cdc63
1417cde8c: xor eax, eax
1417cde8e: or byte ptr [r14 + 0x95], al
1417cde95: mov eax, dword ptr [r15 + 0xf0]
1417cde9c: cmp eax, 0xc3
1417cdea1: jne 0x1417cdc9f
1417cdea7: xor eax, eax
1417cdea9: or byte ptr [r14 + 0x96], al
1417cdeb0: mov eax, dword ptr [r15 + 0x120]
1417cdeb7: cmp eax, 0xc3
1417cdebc: jne 0x1417cdcdb
1417cdec2: xor eax, eax
1417cdec4: or byte ptr [r14 + 0x97], al
1417cdecb: mov eax, dword ptr [r15 + 0xfc]
1417cded2: cmp eax, 0xc3
1417cded7: jne 0x1417cdd17
1417cdedd: xor eax, eax
1417cdedf: or byte ptr [r14 + 0x98], al
1417cdee6: mov eax, dword ptr [r15 + 0x108]
1417cdeed: cmp eax, 0xc3
1417cdef2: jne 0x1417cdd53
1417cdef8: xor eax, eax
1417cdefa: or byte ptr [r14 + 0x99], al
1417cdf01: mov dword ptr [rsp + 0x20], 0x73
1417cdf09: lea rcx, [rsi + 0x20]
1417cdf0d: lea rdx, [rsp + 0x20]
1417cdf12: call 0x14089aef0
1417cdf17: test rax, rax
1417cdf1a: setne al
1417cdf1d: or byte ptr [r14 + 0xa3], al
1417cdf24: mov eax, dword ptr [r15 + 0x150]
1417cdf2b: cmp eax, 0xc3
1417cdf30: jne 0x1417cd25c
1417cdf36: xor eax, eax
1417cdf38: or byte ptr [r14 + 0x9a], al
1417cdf3f: mov eax, dword ptr [r15 + 0x15c]
1417cdf46: cmp eax, 0xc3
1417cdf4b: jne 0x1417cd298
1417cdf51: xor eax, eax
1417cdf53: or byte ptr [r14 + 0x9b], al
1417cdf5a: mov eax, dword ptr [r15 + 0x168]
1417cdf61: cmp eax, 0xc3
1417cdf66: jne 0x1417cd2d4
1417cdf6c: xor eax, eax
1417cdf6e: or byte ptr [r14 + 0x9c], al
1417cdf75: mov eax, dword ptr [r15 + 0x174]
1417cdf7c: cmp eax, 0xc3
1417cdf81: jne 0x1417cd310
1417cdf87: xor eax, eax
1417cdf89: or byte ptr [r14 + 0x9d], al
1417cdf90: mov eax, dword ptr [r15 + 0x180]
1417cdf97: cmp eax, 0xc3
1417cdf9c: jne 0x1417cd34c
1417cdfa2: xor eax, eax
1417cdfa4: or byte ptr [r14 + 0x9e], al
1417cdfab: mov eax, dword ptr [r15 + 0x18c]
1417cdfb2: cmp eax, 0xc3
1417cdfb7: jne 0x1417cd388
1417cdfbd: xor eax, eax
1417cdfbf: or byte ptr [r14 + 0x9f], al
1417cdfc6: mov eax, dword ptr [r15 + 0x198]
1417cdfcd: cmp eax, 0xc3
1417cdfd2: jne 0x1417cd3c4
1417cdfd8: xor eax, eax
1417cdfda: or byte ptr [r14 + 0xa0], al
1417cdfe1: mov eax, dword ptr [r15 + 0x1a4]
1417cdfe8: cmp eax, 0xc3
1417cdfed: jne 0x1417cd400
1417cdff3: xor eax, eax
1417cdff5: or byte ptr [r14 + 0xa1], al
1417cdffc: test r12, r12
1417cdfff: jne 0x1417cd433
1417ce005: mov eax, dword ptr [r15 + 0x12c]
1417ce00c: cmp eax, 0xc3
1417ce011: je 0x1417ce06a
1417ce013: mov rcx, qword ptr [r15 + 0x130]
1417ce01a: mov dword ptr [rsp + 0x20], eax
1417ce01e: mov qword ptr [rsp + 0x24], rcx
1417ce023: lea rdx, [rsp + 0x20]
1417ce028: mov rcx, rsi
1417ce02b: call 0x14089aef0
1417ce030: test rax, rax
1417ce033: je 0x1417ce06a
1417ce035: mov dword ptr [rsp + 0x20], 0x3d
1417ce03d: lea rdx, [rsp + 0x20]
1417ce042: mov rcx, rsi
1417ce045: call 0x14089aef0
1417ce04a: test rax, rax
1417ce04d: sete al
1417ce050: or byte ptr [r14 + 0xa5], al
1417ce057: mov eax, dword ptr [r15 + 0x138]
1417ce05e: cmp eax, 0xc3
1417ce063: jne 0x1417ce085
1417ce065: jmp 0x1417cd445
1417ce06a: xor eax, eax
1417ce06c: or byte ptr [r14 + 0xa5], al
1417ce073: mov eax, dword ptr [r15 + 0x138]
1417ce07a: cmp eax, 0xc3
1417ce07f: je 0x1417cd445
1417ce085: mov rcx, qword ptr [r15 + 0x13c]
1417ce08c: mov dword ptr [rsp + 0x20], eax
1417ce090: mov qword ptr [rsp + 0x24], rcx
1417ce095: lea rdx, [rsp + 0x20]
1417ce09a: mov rcx, rsi
1417ce09d: call 0x14089aef0
1417ce0a2: test rax, rax
1417ce0a5: setne al
1417ce0a8: or byte ptr [r14 + 0xa7], al
1417ce0af: mov eax, dword ptr [r15 + 0x144]
1417ce0b6: cmp eax, 0xc3
1417ce0bb: jne 0x1417cd460
1417ce0c1: xor eax, eax
1417ce0c3: or byte ptr [r14 + 0xa8], al
1417ce0ca: mov eax, dword ptr [r15 + 0x1b0]
1417ce0d1: cmp eax, 0xc3
1417ce0d6: jne 0x1417cd49c
1417ce0dc: xor eax, eax
1417ce0de: or byte ptr [r14 + 0xa9], al
1417ce0e5: mov rdi, qword ptr [rsp + 0x110]
1417ce0ed: cmp byte ptr [rip + 0x1e66be4], 0
1417ce0f4: je 0x1417ce243
1417ce0fa: mov dword ptr [rsp + 0x20], 0x35
1417ce102: lea rdx, [rsp + 0x20]
1417ce107: mov rcx, rsi
1417ce10a: call 0x14089aef0
1417ce10f: test rax, rax
1417ce112: je 0x1417ce131
1417ce114: mov dword ptr [rsp + 0x20], 0x3d
1417ce11c: lea rdx, [rsp + 0x20]
1417ce121: mov rcx, rsi
1417ce124: call 0x14089aef0
1417ce129: test rax, rax
1417ce12c: setne al
1417ce12f: jmp 0x1417ce133
1417ce131: xor eax, eax
1417ce133: or byte ptr [r14 + 0xa2], al
1417ce13a: mov dword ptr [rsp + 0x20], 0x40
1417ce142: lea rdx, [rsp + 0x20]
1417ce147: mov rcx, rsi
1417ce14a: call 0x14089aef0
1417ce14f: test rax, rax
1417ce152: setne al
1417ce155: or byte ptr [r14 + 0xa4], al
1417ce15c: mov dword ptr [rsp + 0x20], 0xab
1417ce164: lea rdx, [rsp + 0x20]
1417ce169: mov rcx, rsi
1417ce16c: call 0x14089aef0
1417ce171: test rax, rax
1417ce174: je 0x1417ce193
1417ce176: mov dword ptr [rsp + 0x20], 0x3d
1417ce17e: lea rdx, [rsp + 0x20]
1417ce183: mov rcx, rsi
1417ce186: call 0x14089aef0
1417ce18b: test rax, rax
1417ce18e: setne al
1417ce191: jmp 0x1417ce195
1417ce193: xor eax, eax
1417ce195: or byte ptr [r14 + 0xa6], al
1417ce19c: mov dword ptr [rsp + 0x20], 0x25
1417ce1a4: lea rcx, [rsi + 0x20]
1417ce1a8: lea rdx, [rsp + 0x20]
1417ce1ad: call 0x14089aef0
1417ce1b2: test rax, rax
1417ce1b5: je 0x1417ce1d4
1417ce1b7: mov dword ptr [rsp + 0x20], 0x33
1417ce1bf: lea rdx, [rsp + 0x20]
1417ce1c4: mov rcx, rsi
1417ce1c7: call 0x14089aef0
1417ce1cc: test rax, rax
1417ce1cf: sete al
1417ce1d2: jmp 0x1417ce1d6
1417ce1d4: xor eax, eax
1417ce1d6: or byte ptr [r14 + 0xaa], al
1417ce1dd: mov dword ptr [rsp + 0x20], 0x23
1417ce1e5: lea rdx, [rsp + 0x20]
1417ce1ea: mov rcx, rsi
1417ce1ed: call 0x14089aef0
1417ce1f2: test rax, rax
1417ce1f5: setne al
1417ce1f8: or byte ptr [r14 + 0xab], al
1417ce1ff: mov dword ptr [rsp + 0x20], 0x29
1417ce207: lea rdx, [rsp + 0x20]
1417ce20c: mov rcx, rsi
1417ce20f: call 0x14089aef0
1417ce214: test rax, rax
1417ce217: setne al
1417ce21a: or byte ptr [r14 + 0xac], al
1417ce221: mov dword ptr [rsp + 0x20], 0xa6
1417ce229: lea rdx, [rsp + 0x20]
1417ce22e: mov rcx, rsi
1417ce231: call 0x14089aef0
1417ce236: test rax, rax
1417ce239: setne al
1417ce23c: or byte ptr [r14 + 0xad], al
1417ce243: mov dword ptr [rsp + 0x20], 0xa7
1417ce24b: lea rdx, [rsp + 0x20]
1417ce250: mov rcx, rsi
1417ce253: call 0x14089aef0
1417ce258: test rax, rax
1417ce25b: setne al
1417ce25e: or byte ptr [r14 + 0xae], al
1417ce265: mov rax, qword ptr [rdi]
1417ce268: mov r11, qword ptr [rdi + 8]
1417ce26c: mov rcx, qword ptr [rax]
1417ce26f: xor esi, esi
1417ce271: mov rdx, rcx
1417ce274: sub rdx, qword ptr [r11 + 0x18]
1417ce278: cmovb rdx, rsi
1417ce27c: sub rcx, qword ptr [r11 + 0x38]
1417ce280: cmovb rcx, rsi
1417ce284: mov r10, qword ptr [r11 + 8]
1417ce288: mov r9, qword ptr [r11 + 0x30]
1417ce28c: mov r8, r9
1417ce28f: sub r8, rcx
1417ce292: cmovb r8, rsi
1417ce296: mov rbx, qword ptr [r11 + 0x10]
1417ce29a: setbe bpl
1417ce29e: mov rdi, rbx
1417ce2a1: sub rdi, rdx
1417ce2a4: cmovb rdi, rsi
1417ce2a8: mov rsi, qword ptr [r11 + 0x28]
1417ce2ac: mov r11, qword ptr [r11 + 0x40]
1417ce2b0: setbe r15b
1417ce2b4: mov r13, r11
1417ce2b7: sub r13, r8
1417ce2ba: sub r13, rdi
1417ce2bd: mov qword ptr [rax], r13
1417ce2c0: test r15b, bpl
1417ce2c3: jne 0x1417ce324
1417ce2c5: mov r15, rcx
1417ce2c8: shl r15, 4
1417ce2cc: add rsi, r15
1417ce2cf: shl r8, 4
1417ce2d3: mov r15, rdx
1417ce2d6: shl r15, 4
1417ce2da: add r10, r15
1417ce2dd: shl rdi, 4
1417ce2e1: xor r15d, r15d
1417ce2e4: cmp rbx, rdx
1417ce2e7: mov edx, 8
1417ce2ec: cmovb r10, rdx
1417ce2f0: lea r10, [r10 + rdi - 0x10]
1417ce2f5: cmova r15, r10
1417ce2f9: cmp r9, rcx
1417ce2fc: cmovb rsi, rdx
1417ce300: lea rcx, [rsi + r8 - 0x10]
1417ce305: cmovbe rcx, r15
1417ce309: mov qword ptr [rax], r11
1417ce30c: cmp byte ptr [rcx + 8], 0
1417ce310: je 0x1417ce31c
1417ce312: mov byte ptr [r14 + 0x99], 1
1417ce31a: jmp 0x1417ce324
1417ce31c: mov byte ptr [r14 + 0x98], 1
1417ce324: movaps xmm6, xmmword ptr [rsp + 0x50]
1417ce329: movaps xmm7, xmmword ptr [rsp + 0x60]
1417ce32e: add rsp, 0x78
1417ce332: pop rbx
1417ce333: pop rbp
1417ce334: pop rdi
1417ce335: pop rsi
1417ce336: pop r12
1417ce338: pop r13
1417ce33a: pop r14
1417ce33c: pop r15
1417ce33e: ret 
1417ce33f: ud2 
