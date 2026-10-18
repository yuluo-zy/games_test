1415fc230: push rbp
1415fc231: push r15
1415fc233: push r14
1415fc235: push r13
1415fc237: push r12
1415fc239: push rsi
1415fc23a: push rdi
1415fc23b: push rbx
1415fc23c: sub rsp, 0x368
1415fc243: lea rbp, [rsp + 0x80]
1415fc24b: mov qword ptr [rbp + 0x2e0], 0xfffffffffffffffe
1415fc256: mov rdi, r8
1415fc259: mov qword ptr [rbp + 0x2d0], rdx
1415fc260: mov rsi, rcx
1415fc263: lea rcx, [r8 + 0x38]
1415fc267: mov byte ptr [rbp + 0x2df], 1
1415fc26e: movabs rdx, 0x8226cc786e679355
1415fc278: movabs r8, 0x65cfda6a55a841ac
1415fc282: call 0x1400dd6f0
1415fc287: test rax, rax
1415fc28a: je 0x1415fcc9b
1415fc290: cmp qword ptr [rdi + 0x218], rdx
1415fc297: jbe 0x1415fcc9b
1415fc29d: mov rax, qword ptr [rdi + 0x210]
1415fc2a4: mov rcx, qword ptr [rax + rdx*8]
1415fc2a8: test rcx, rcx
1415fc2ab: je 0x1415fcc9b
1415fc2b1: mov rax, qword ptr [rdi + 0x1e0]
1415fc2b8: not rcx
1415fc2bb: lea rcx, [rcx + rcx*2]
1415fc2bf: shl rcx, 5
1415fc2c3: cmp qword ptr [rax + rcx + 0x20], 0
1415fc2c9: je 0x1415fcc9b
1415fc2cf: add rax, rcx
1415fc2d2: mov rdi, qword ptr [rax + 0x10]
1415fc2d6: mov byte ptr [rbp + 0x2df], 1
1415fc2dd: lea rdx, [rip + 0x16320f4]
1415fc2e4: mov rcx, rdi
1415fc2e7: call 0x140969720
1415fc2ec: mov rbx, rax
1415fc2ef: test rax, rax
1415fc2f2: je 0x1415fccb0
1415fc2f8: lea rax, [rip + 0x14a1931]
1415fc2ff: mov qword ptr [rbp + 0x138], rax
1415fc306: mov qword ptr [rbp + 0x140], 0x2c
1415fc311: mov r14, rbx
1415fc314: add r14, 0x10
1415fc318: cmp qword ptr [rbx + 0x18], 0x2c
1415fc31d: jne 0x1415fcb44
1415fc323: mov rdx, qword ptr [r14]
1415fc326: lea rcx, [rip + 0x14a1903]
1415fc32d: mov r8d, 0x2c
1415fc333: call 0x142923bf0
1415fc338: test eax, eax
1415fc33a: jne 0x1415fcb44
1415fc340: mov rcx, qword ptr [rbx]
1415fc343: mov rax, qword ptr [rbx + 8]
1415fc347: mov rax, qword ptr [rax + 0x18]
1415fc34b: mov byte ptr [rbp + 0x2df], 1
1415fc352: call rax
1415fc354: mov rbx, rax
1415fc357: test rax, rax
1415fc35a: je 0x1415fccb0
1415fc360: mov byte ptr [rbp + 0x2df], 1
1415fc367: lea rdx, [rip + 0x1631fe2]
1415fc36e: mov rcx, rdi
1415fc371: call 0x140969720
1415fc376: mov rdi, rax
1415fc379: test rax, rax
1415fc37c: je 0x1415fccc5
1415fc382: lea rax, [rip + 0x14a239d]
1415fc389: mov qword ptr [rbp + 0x138], rax
1415fc390: mov qword ptr [rbp + 0x140], 0x26
1415fc39b: mov r14, rdi
1415fc39e: add r14, 0x10
1415fc3a2: cmp qword ptr [rdi + 0x18], 0x26
1415fc3a7: jne 0x1415fcb79
1415fc3ad: mov rdx, qword ptr [r14]
1415fc3b0: lea rcx, [rip + 0x14a236f]
1415fc3b7: mov r8d, 0x26
1415fc3bd: call 0x142923bf0
1415fc3c2: test eax, eax
1415fc3c4: jne 0x1415fcb79
1415fc3ca: mov rcx, qword ptr [rdi]
1415fc3cd: mov rax, qword ptr [rdi + 8]
1415fc3d1: mov rax, qword ptr [rax + 0x18]
1415fc3d5: mov byte ptr [rbp + 0x2df], 1
1415fc3dc: call rax
1415fc3de: mov rdi, rax
1415fc3e1: test rax, rax
1415fc3e4: je 0x1415fccc5
1415fc3ea: lea r14, [rbp - 0x58]
1415fc3ee: mov r8d, 0x190
1415fc3f4: mov rcx, r14
1415fc3f7: mov rdx, qword ptr [rbp + 0x2d0]
1415fc3fe: call 0x142923c00
1415fc403: lea rcx, [rbp + 0xf0]
1415fc40a: mov rdx, qword ptr [rbp + 0xe8]
1415fc411: lea r8, [rbp + 8]
1415fc415: mov byte ptr [rsp + 0x20], 0x1a
1415fc41a: mov r9, r14
1415fc41d: call 0x140972c20
1415fc422: mov rcx, qword ptr [rbp + 0xe0]
1415fc429: cmp qword ptr [rcx + 0xb0], 0
1415fc431: je 0x1415fc51b
1415fc437: mov r14d, eax
1415fc43a: mov rax, qword ptr [rcx + 0x98]
1415fc441: mov rcx, qword ptr [rcx + 0xa0]
1415fc448: movabs rdx, 0x168ddb2c3b5f38a9
1415fc452: mov r8, rcx
1415fc455: and r8, rdx
1415fc458: lea r9, [rax - 0x10]
1415fc45c: xor r10d, r10d
1415fc45f: movdqa xmm0, xmmword ptr [rip + 0x13a8739]
1415fc467: pcmpeqd xmm1, xmm1
1415fc46b: movdqu xmm2, xmmword ptr [rax + r8]
1415fc471: movdqa xmm3, xmm2
1415fc475: pcmpeqb xmm3, xmm0
1415fc479: pmovmskb r11d, xmm3
1415fc47e: test r11d, r11d
1415fc481: je 0x1415fc4b0
1415fc483: tzcnt r15d, r11d
1415fc488: add r15, r8
1415fc48b: and r15, rcx
1415fc48e: mov r12, r15
1415fc491: shl r12, 4
1415fc495: mov r13, r9
1415fc498: sub r13, r12
1415fc49b: cmp qword ptr [r13], rdx
1415fc49f: je 0x1415fc4ce
1415fc4a1: lea r15d, [r11 - 1]
1415fc4a5: and r15w, r11w
1415fc4a9: mov r11d, r15d
1415fc4ac: jne 0x1415fc483
1415fc4ae: nop 
1415fc4b0: pcmpeqb xmm2, xmm1
1415fc4b4: pmovmskb r11d, xmm2
1415fc4b9: test r11d, r11d
1415fc4bc: jne 0x1415fc51b
1415fc4be: add r8, r10
1415fc4c1: add r8, 0x10
1415fc4c5: add r10, 0x10
1415fc4c9: and r8, rcx
1415fc4cc: jmp 0x1415fc46b
1415fc4ce: neg r15
1415fc4d1: shl r15, 4
1415fc4d5: mov ecx, dword ptr [rax + r15 - 8]
1415fc4da: mov r15d, dword ptr [rax + r15 - 4]
1415fc4df: mov dword ptr [rbp + 0x2d8], ecx
1415fc4e5: test ecx, ecx
1415fc4e7: jne 0x1415fcbae
1415fc4ed: mov r12, qword ptr [rbp - 0x30]
1415fc4f1: cmp r12, qword ptr [rbp - 0x40]
1415fc4f5: jne 0x1415fc507
1415fc4f7: lea rcx, [rbp - 0x40]
1415fc4fb: lea rdx, [rip + 0x1681776]
1415fc502: call 0x140562e70
1415fc507: mov rax, qword ptr [rbp - 0x38]
1415fc50b: mov dword ptr [rax + r12*8], r15d
1415fc50f: mov byte ptr [rax + r12*8 + 4], r14b
1415fc514: inc r12
1415fc517: mov qword ptr [rbp - 0x30], r12
1415fc51b: lea r14, [rbp + 0x138]
1415fc522: lea rdx, [rbp - 0x58]
1415fc526: mov r8d, 0x190
1415fc52c: mov rcx, r14
1415fc52f: call 0x142923c00
1415fc534: lea rcx, [rbp + 0x280]
1415fc53b: mov rdx, qword ptr [rbp + 0x278]
1415fc542: lea r8, [rbp + 0x198]
1415fc549: mov byte ptr [rsp + 0x20], 0x1a
1415fc54e: mov r9, r14
1415fc551: call 0x140971d20
1415fc556: mov rcx, qword ptr [rbp + 0x270]
1415fc55d: cmp qword ptr [rcx + 0xb0], 0
1415fc565: je 0x1415fc66a
1415fc56b: mov r14d, eax
1415fc56e: mov rax, qword ptr [rcx + 0x98]
1415fc575: mov rcx, qword ptr [rcx + 0xa0]
1415fc57c: movabs rdx, 0x47ddebfb264383d0
1415fc586: mov r8, rcx
1415fc589: and r8, rdx
1415fc58c: lea r9, [rax - 0x10]
1415fc590: xor r10d, r10d
1415fc593: movdqa xmm0, xmmword ptr [rip + 0x1354095]
1415fc59b: pcmpeqd xmm1, xmm1
1415fc59f: movdqu xmm2, xmmword ptr [rax + r8]
1415fc5a5: movdqa xmm3, xmm2
1415fc5a9: pcmpeqb xmm3, xmm0
1415fc5ad: pmovmskb r11d, xmm3
1415fc5b2: test r11d, r11d
1415fc5b5: je 0x1415fc5f0
1415fc5b7: tzcnt r15d, r11d
1415fc5bc: add r15, r8
1415fc5bf: and r15, rcx
1415fc5c2: mov r12, r15
1415fc5c5: shl r12, 4
1415fc5c9: mov r13, r9
1415fc5cc: sub r13, r12
1415fc5cf: cmp qword ptr [r13], rdx
1415fc5d3: je 0x1415fc60e
1415fc5d5: lea r15d, [r11 - 1]
1415fc5d9: and r15w, r11w
1415fc5dd: mov r11d, r15d
1415fc5e0: jne 0x1415fc5b7
1415fc5e2: nop word ptr cs:[rax + rax]
1415fc5f0: pcmpeqb xmm2, xmm1
1415fc5f4: pmovmskb r11d, xmm2
1415fc5f9: test r11d, r11d
1415fc5fc: jne 0x1415fc66a
1415fc5fe: add r8, r10
1415fc601: add r8, 0x10
1415fc605: add r10, 0x10
1415fc609: and r8, rcx
1415fc60c: jmp 0x1415fc59f
1415fc60e: neg r15
1415fc611: shl r15, 4
1415fc615: mov ecx, dword ptr [rax + r15 - 8]
1415fc61a: mov r15d, dword ptr [rax + r15 - 4]
1415fc61f: mov dword ptr [rbp + 0x2d8], ecx
1415fc625: test ecx, ecx
1415fc627: jne 0x1415fcbe6
1415fc62d: mov r12, qword ptr [rbp + 0x160]
1415fc634: cmp r12, qword ptr [rbp + 0x150]
1415fc63b: jne 0x1415fc650
1415fc63d: lea rcx, [rbp + 0x150]
1415fc644: lea rdx, [rip + 0x168162d]
1415fc64b: call 0x140562e70
1415fc650: mov rax, qword ptr [rbp + 0x158]
1415fc657: mov dword ptr [rax + r12*8], r15d
1415fc65b: mov byte ptr [rax + r12*8 + 4], r14b
1415fc660: inc r12
1415fc663: mov qword ptr [rbp + 0x160], r12
1415fc66a: lea r14, [rbp - 0x58]
1415fc66e: lea rdx, [rbp + 0x138]
1415fc675: mov r8d, 0x190
1415fc67b: mov rcx, r14
1415fc67e: call 0x142923c00
1415fc683: lea rcx, [rbp + 0xf0]
1415fc68a: mov rdx, qword ptr [rbp + 0xe8]
1415fc691: lea r8, [rbp + 8]
1415fc695: mov byte ptr [rsp + 0x20], 0x1a
1415fc69a: mov r9, r14
1415fc69d: call 0x140970120
1415fc6a2: mov rcx, qword ptr [rbp + 0xe0]
1415fc6a9: cmp qword ptr [rcx + 0xb0], 0
1415fc6b1: je 0x1415fc79b
1415fc6b7: mov r14d, eax
1415fc6ba: mov rax, qword ptr [rcx + 0x98]
1415fc6c1: mov rcx, qword ptr [rcx + 0xa0]
1415fc6c8: movabs rdx, 0x339e76846042a271
1415fc6d2: mov r8, rcx
1415fc6d5: and r8, rdx
1415fc6d8: lea r9, [rax - 0x10]
1415fc6dc: xor r10d, r10d
1415fc6df: movdqa xmm0, xmmword ptr [rip + 0x1353fe9]
1415fc6e7: pcmpeqd xmm1, xmm1
1415fc6eb: movdqu xmm2, xmmword ptr [rax + r8]
1415fc6f1: movdqa xmm3, xmm2
1415fc6f5: pcmpeqb xmm3, xmm0
1415fc6f9: pmovmskb r11d, xmm3
1415fc6fe: test r11d, r11d
1415fc701: je 0x1415fc730
1415fc703: tzcnt r15d, r11d
1415fc708: add r15, r8
1415fc70b: and r15, rcx
1415fc70e: mov r12, r15
1415fc711: shl r12, 4
1415fc715: mov r13, r9
1415fc718: sub r13, r12
1415fc71b: cmp qword ptr [r13], rdx
1415fc71f: je 0x1415fc74e
1415fc721: lea r15d, [r11 - 1]
1415fc725: and r15w, r11w
1415fc729: mov r11d, r15d
1415fc72c: jne 0x1415fc703
1415fc72e: nop 
1415fc730: pcmpeqb xmm2, xmm1
1415fc734: pmovmskb r11d, xmm2
1415fc739: test r11d, r11d
1415fc73c: jne 0x1415fc79b
1415fc73e: add r8, r10
1415fc741: add r8, 0x10
1415fc745: add r10, 0x10
1415fc749: and r8, rcx
1415fc74c: jmp 0x1415fc6eb
1415fc74e: neg r15
1415fc751: shl r15, 4
1415fc755: mov ecx, dword ptr [rax + r15 - 8]
1415fc75a: mov r15d, dword ptr [rax + r15 - 4]
1415fc75f: mov dword ptr [rbp + 0x2d8], ecx
1415fc765: test ecx, ecx
1415fc767: jne 0x1415fcc18
1415fc76d: mov r12, qword ptr [rbp - 0x30]
1415fc771: cmp r12, qword ptr [rbp - 0x40]
1415fc775: jne 0x1415fc787
1415fc777: lea rcx, [rbp - 0x40]
1415fc77b: lea rdx, [rip + 0x16814f6]
1415fc782: call 0x140562e70
1415fc787: mov rax, qword ptr [rbp - 0x38]
1415fc78b: mov dword ptr [rax + r12*8], r15d
1415fc78f: mov byte ptr [rax + r12*8 + 4], r14b
1415fc794: inc r12
1415fc797: mov qword ptr [rbp - 0x30], r12
1415fc79b: lea r14, [rbp + 0x138]
1415fc7a2: lea rdx, [rbp - 0x58]
1415fc7a6: mov r8d, 0x190
1415fc7ac: mov rcx, r14
1415fc7af: call 0x142923c00
1415fc7b4: lea rcx, [rbp + 0x280]
1415fc7bb: mov rdx, qword ptr [rbp + 0x278]
1415fc7c2: lea r8, [rbp + 0x198]
1415fc7c9: mov byte ptr [rsp + 0x20], 0x1a
1415fc7ce: mov r9, r14
1415fc7d1: call 0x140972b20
1415fc7d6: mov rcx, qword ptr [rbp + 0x270]
1415fc7dd: cmp qword ptr [rcx + 0xb0], 0
1415fc7e5: je 0x1415fc8ea
1415fc7eb: mov r14d, eax
1415fc7ee: mov rax, qword ptr [rcx + 0x98]
1415fc7f5: mov rcx, qword ptr [rcx + 0xa0]
1415fc7fc: movabs rdx, 0xe9701a257aa8828c
1415fc806: mov r8, rcx
1415fc809: and r8, rdx
1415fc80c: lea r9, [rax - 0x10]
1415fc810: xor r10d, r10d
1415fc813: movdqa xmm0, xmmword ptr [rip + 0x14000d5]
1415fc81b: pcmpeqd xmm1, xmm1
1415fc81f: movdqu xmm2, xmmword ptr [rax + r8]
1415fc825: movdqa xmm3, xmm2
1415fc829: pcmpeqb xmm3, xmm0
1415fc82d: pmovmskb r11d, xmm3
1415fc832: test r11d, r11d
1415fc835: je 0x1415fc870
1415fc837: tzcnt r15d, r11d
1415fc83c: add r15, r8
1415fc83f: and r15, rcx
1415fc842: mov r12, r15
1415fc845: shl r12, 4
1415fc849: mov r13, r9
1415fc84c: sub r13, r12
1415fc84f: cmp qword ptr [r13], rdx
1415fc853: je 0x1415fc88e
1415fc855: lea r15d, [r11 - 1]
1415fc859: and r15w, r11w
1415fc85d: mov r11d, r15d
1415fc860: jne 0x1415fc837
1415fc862: nop word ptr cs:[rax + rax]
1415fc870: pcmpeqb xmm2, xmm1
1415fc874: pmovmskb r11d, xmm2
1415fc879: test r11d, r11d
1415fc87c: jne 0x1415fc8ea
1415fc87e: add r8, r10
1415fc881: add r8, 0x10
1415fc885: add r10, 0x10
1415fc889: and r8, rcx
1415fc88c: jmp 0x1415fc81f
1415fc88e: neg r15
1415fc891: shl r15, 4
1415fc895: mov ecx, dword ptr [rax + r15 - 8]
1415fc89a: mov r15d, dword ptr [rax + r15 - 4]
1415fc89f: mov dword ptr [rbp + 0x2d8], ecx
1415fc8a5: test ecx, ecx
1415fc8a7: jne 0x1415fcc50
1415fc8ad: mov r12, qword ptr [rbp + 0x160]
1415fc8b4: cmp r12, qword ptr [rbp + 0x150]
1415fc8bb: jne 0x1415fc8d0
1415fc8bd: lea rcx, [rbp + 0x150]
1415fc8c4: lea rdx, [rip + 0x16813ad]
1415fc8cb: call 0x140562e70
1415fc8d0: mov rax, qword ptr [rbp + 0x158]
1415fc8d7: mov dword ptr [rax + r12*8], r15d
1415fc8db: mov byte ptr [rax + r12*8 + 4], r14b
1415fc8e0: inc r12
1415fc8e3: mov qword ptr [rbp + 0x160], r12
1415fc8ea: lea rcx, [rbp - 0x58]
1415fc8ee: lea rdx, [rbp + 0x138]
1415fc8f5: mov r8d, 0x190
1415fc8fb: call 0x142923c00
1415fc900: mov rax, qword ptr [rbx]
1415fc903: cmp byte ptr [rbp + 0xde], 8
1415fc90a: je 0x1415fca17
1415fc910: mov r9, qword ptr [rbp + 0xe0]
1415fc917: cmp qword ptr [r9 + 0xe8], 0
1415fc91f: je 0x1415fca17
1415fc925: cmp qword ptr [r9 + 0x130], 0
1415fc92d: je 0x1415fca17
1415fc933: mov r8, qword ptr [rbp + 0xb8]
1415fc93a: mov rdx, qword ptr [rbp + 0xc0]
1415fc941: mov r10, qword ptr [r9 + 0x118]
1415fc948: mov rcx, qword ptr [r9 + 0x120]
1415fc94f: movabs r13, 0x1c9f2eef0ac739ac
1415fc959: mov r11, rcx
1415fc95c: and r11, r13
1415fc95f: lea rbx, [r10 - 0x10]
1415fc963: xor r12d, r12d
1415fc966: movdqa xmm0, xmmword ptr [rip + 0x1353b72]
1415fc96e: pcmpeqd xmm1, xmm1
1415fc972: movdqu xmm2, xmmword ptr [r10 + r11]
1415fc978: movdqa xmm3, xmm2
1415fc97c: pcmpeqb xmm3, xmm0
1415fc980: pmovmskb r15d, xmm3
1415fc985: test r15d, r15d
1415fc988: je 0x1415fc9d7
1415fc98a: mov qword ptr [rbp + 0x2c8], r12
1415fc991: mov r14, rdi
1415fc994: mov rdi, r8
1415fc997: tzcnt r12d, r15d
1415fc99c: add r12, r11
1415fc99f: and r12, rcx
1415fc9a2: mov r8, r13
1415fc9a5: mov r13, r12
1415fc9a8: shl r13, 4
1415fc9ac: mov r9, rbx
1415fc9af: sub rbx, r13
1415fc9b2: cmp qword ptr [rbx], r8
1415fc9b5: je 0x1415fc9fc
1415fc9b7: mov r13, r8
1415fc9ba: lea ebx, [r15 - 1]
1415fc9be: and bx, r15w
1415fc9c2: mov r15d, ebx
1415fc9c5: mov r12, qword ptr [rbp + 0x2c8]
1415fc9cc: mov r8, rdi
1415fc9cf: mov rdi, r14
1415fc9d2: mov rbx, r9
1415fc9d5: jne 0x1415fc991
1415fc9d7: mov r9, rbx
1415fc9da: pcmpeqb xmm2, xmm1
1415fc9de: pmovmskb ebx, xmm2
1415fc9e2: test ebx, ebx
1415fc9e4: jne 0x1415fca17
1415fc9e6: add r11, r12
1415fc9e9: add r11, 0x10
1415fc9ed: add r12, 0x10
1415fc9f1: and r11, rcx
1415fc9f4: mov rbx, r9
1415fc9f7: jmp 0x1415fc972
1415fc9fc: neg r12
1415fc9ff: shl r12, 4
1415fca03: mov ecx, dword ptr [r10 + r12 - 8]
1415fca08: cmp rdx, rcx
1415fca0b: jb 0x1415fcc7f
1415fca11: mov dword ptr [rdi + rcx], eax
1415fca14: mov rdi, r14
1415fca17: lea rcx, [rbp + 0x138]
1415fca1e: lea rdx, [rbp - 0x58]
1415fca22: mov r8d, 0x190
1415fca28: call 0x142923c00
1415fca2d: mov rax, qword ptr [rdi]
1415fca30: cmp byte ptr [rbp + 0x26e], 8
1415fca37: je 0x1415fcb18
1415fca3d: mov r9, qword ptr [rbp + 0x270]
1415fca44: cmp qword ptr [r9 + 0xe8], 0
1415fca4c: je 0x1415fcb18
1415fca52: cmp qword ptr [r9 + 0x130], 0
1415fca5a: je 0x1415fcb18
1415fca60: mov r8, qword ptr [rbp + 0x248]
1415fca67: mov rdx, qword ptr [rbp + 0x250]
1415fca6e: mov rcx, qword ptr [r9 + 0x118]
1415fca75: mov r9, qword ptr [r9 + 0x120]
1415fca7c: movabs r10, 0xc2b7f2fdf5e21521
1415fca86: mov r11, r9
1415fca89: and r11, r10
1415fca8c: lea rdi, [rcx - 0x10]
1415fca90: xor ebx, ebx
1415fca92: movdqa xmm0, xmmword ptr [rip + 0x1353966]
1415fca9a: pcmpeqd xmm1, xmm1
1415fca9e: movdqu xmm2, xmmword ptr [rcx + r11]
1415fcaa4: movdqa xmm3, xmm2
1415fcaa8: pcmpeqb xmm3, xmm0
1415fcaac: pmovmskb r14d, xmm3
1415fcab1: test r14d, r14d
1415fcab4: je 0x1415fcae1
1415fcab6: tzcnt r15d, r14d
1415fcabb: add r15, r11
1415fcabe: and r15, r9
1415fcac1: mov r12, r15
1415fcac4: shl r12, 4
1415fcac8: mov r13, rdi
1415fcacb: sub r13, r12
1415fcace: cmp qword ptr [r13], r10
1415fcad2: je 0x1415fcaff
1415fcad4: lea r15d, [r14 - 1]
1415fcad8: and r15w, r14w
1415fcadc: mov r14d, r15d
1415fcadf: jne 0x1415fcab6
1415fcae1: pcmpeqb xmm2, xmm1
1415fcae5: pmovmskb r14d, xmm2
1415fcaea: test r14d, r14d
1415fcaed: jne 0x1415fcb18
1415fcaef: add r11, rbx
1415fcaf2: add r11, 0x10
1415fcaf6: add rbx, 0x10
1415fcafa: and r11, r9
1415fcafd: jmp 0x1415fca9e
1415fcaff: neg r15
1415fcb02: shl r15, 4
1415fcb06: mov ecx, dword ptr [rcx + r15 - 8]
1415fcb0b: cmp rdx, rcx
1415fcb0e: jb 0x1415fcc8d
1415fcb14: mov dword ptr [r8 + rcx], eax
1415fcb18: lea rdx, [rbp + 0x138]
1415fcb1f: mov r8d, 0x190
1415fcb25: mov rcx, rsi
1415fcb28: call 0x142923c00
1415fcb2d: mov rax, rsi
1415fcb30: add rsp, 0x368
1415fcb37: pop rbx
1415fcb38: pop rdi
1415fcb39: pop rsi
1415fcb3a: pop r12
1415fcb3c: pop r13
1415fcb3e: pop r14
1415fcb40: pop r15
1415fcb42: pop rbp
1415fcb43: ret 
1415fcb44: mov qword ptr [rbp - 0x58], 0
1415fcb4c: mov byte ptr [rbp + 0x2df], 1
1415fcb53: lea rax, [rip + 0x1631626]
1415fcb5a: mov qword ptr [rsp + 0x20], rax
1415fcb5f: lea rdx, [rbp + 0x138]
1415fcb66: lea r9, [rbp - 0x58]
1415fcb6a: xor ecx, ecx
1415fcb6c: mov r8, r14
1415fcb6f: call 0x1428cab41
1415fcb74: jmp 0x1415fccd8
1415fcb79: mov qword ptr [rbp - 0x58], 0
1415fcb81: mov byte ptr [rbp + 0x2df], 1
1415fcb88: lea rax, [rip + 0x16315f1]
1415fcb8f: mov qword ptr [rsp + 0x20], rax
1415fcb94: lea rdx, [rbp + 0x138]
1415fcb9b: lea r9, [rbp - 0x58]
1415fcb9f: xor ecx, ecx
1415fcba1: mov r8, r14
1415fcba4: call 0x1428cab41
1415fcba9: jmp 0x1415fccd8
1415fcbae: mov qword ptr [rbp + 0x138], 0
1415fcbb9: lea rax, [rip + 0x16810a0]
1415fcbc0: mov qword ptr [rsp + 0x20], rax
1415fcbc5: lea r8, [rip + 0x1680e1c]
1415fcbcc: lea rdx, [rbp + 0x2d8]
1415fcbd3: lea r9, [rbp + 0x138]
1415fcbda: xor ecx, ecx
1415fcbdc: call 0x1428a994f
1415fcbe1: jmp 0x1415fccd8
1415fcbe6: mov qword ptr [rbp - 0x58], 0
1415fcbee: lea rax, [rip + 0x168106b]
1415fcbf5: mov qword ptr [rsp + 0x20], rax
1415fcbfa: lea r8, [rip + 0x1680de7]
1415fcc01: lea rdx, [rbp + 0x2d8]
1415fcc08: lea r9, [rbp - 0x58]
1415fcc0c: xor ecx, ecx
1415fcc0e: call 0x1428a994f
1415fcc13: jmp 0x1415fccd8
1415fcc18: mov qword ptr [rbp + 0x138], 0
1415fcc23: lea rax, [rip + 0x1681036]
1415fcc2a: mov qword ptr [rsp + 0x20], rax
1415fcc2f: lea r8, [rip + 0x1680db2]
1415fcc36: lea rdx, [rbp + 0x2d8]
1415fcc3d: lea r9, [rbp + 0x138]
1415fcc44: xor ecx, ecx
1415fcc46: call 0x1428a994f
1415fcc4b: jmp 0x1415fccd8
1415fcc50: mov qword ptr [rbp - 0x58], 0
1415fcc58: lea rax, [rip + 0x1681001]
1415fcc5f: mov qword ptr [rsp + 0x20], rax
1415fcc64: lea r8, [rip + 0x1680d7d]
1415fcc6b: lea rdx, [rbp + 0x2d8]
1415fcc72: lea r9, [rbp - 0x58]
1415fcc76: xor ecx, ecx
1415fcc78: call 0x1428a994f
1415fcc7d: jmp 0x1415fccd8
1415fcc7f: lea r8, [rip + 0x14b8f3a]
1415fcc86: call 0x1428d9830
1415fcc8b: jmp 0x1415fccd8
1415fcc8d: lea r8, [rip + 0x14b8f2c]
1415fcc94: call 0x1428d9830
1415fcc99: jmp 0x1415fccd8
1415fcc9b: mov byte ptr [rbp + 0x2df], 1
1415fcca2: lea rcx, [rip + 0x1680c7f]
1415fcca9: call 0x1428d9310
1415fccae: jmp 0x1415fccd8
1415fccb0: mov byte ptr [rbp + 0x2df], 1
1415fccb7: lea rcx, [rip + 0x1680c82]
1415fccbe: call 0x1428d9310
1415fccc3: jmp 0x1415fccd8
1415fccc5: mov byte ptr [rbp + 0x2df], 1
1415fcccc: lea rcx, [rip + 0x1680c85]
1415fccd3: call 0x1428d9310
1415fccd8: ud2 
1415fccda: nop word ptr [rax + rax]
1415fcce0: mov qword ptr [rsp + 0x10], rdx
1415fcce5: push rbp
1415fcce6: push r15
1415fcce8: push r14
1415fccea: push r13
1415fccec: push r12
1415fccee: push rsi
1415fccef: push rdi
1415fccf0: push rbx
1415fccf1: sub rsp, 0x28
1415fccf5: lea rbp, [rdx + 0x80]
1415fccfc: mov byte ptr [rbp + 0x2df], 0
1415fcd03: lea rcx, [rbp - 0x58]
1415fcd07: call 0x140896fb0
1415fcd0c: mov byte ptr [rbp + 0x2df], 0
1415fcd13: add rsp, 0x28
1415fcd17: pop rbx
1415fcd18: pop rdi
1415fcd19: pop rsi
1415fcd1a: pop r12
1415fcd1c: pop r13
1415fcd1e: pop r14
1415fcd20: pop r15
1415fcd22: pop rbp
1415fcd23: ret 
1415fcd24: nop word ptr cs:[rax + rax]
1415fcd30: mov qword ptr [rsp + 0x10], rdx
1415fcd35: push rbp
1415fcd36: push r15
1415fcd38: push r14
1415fcd3a: push r13
1415fcd3c: push r12
1415fcd3e: push rsi
1415fcd3f: push rdi
1415fcd40: push rbx
1415fcd41: sub rsp, 0x28
1415fcd45: lea rbp, [rdx + 0x80]
1415fcd4c: cmp byte ptr [rbp + 0x2df], 0
1415fcd53: jne 0x1415fcd66
1415fcd55: add rsp, 0x28
1415fcd59: pop rbx
1415fcd5a: pop rdi
1415fcd5b: pop rsi
1415fcd5c: pop r12
1415fcd5e: pop r13
1415fcd60: pop r14
1415fcd62: pop r15
1415fcd64: pop rbp
1415fcd65: ret 
1415fcd66: mov rcx, qword ptr [rbp + 0x2d0]
1415fcd6d: call 0x140896fb0
1415fcd72: jmp 0x1415fcd55
1415fcd74: nop word ptr cs:[rax + rax]
1415fcd80: mov qword ptr [rsp + 0x10], rdx
1415fcd85: push rbp
1415fcd86: push r15
1415fcd88: push r14
1415fcd8a: push r13
1415fcd8c: push r12
1415fcd8e: push rsi
1415fcd8f: push rdi
1415fcd90: push rbx
1415fcd91: sub rsp, 0x28
1415fcd95: lea rbp, [rdx + 0x80]
1415fcd9c: mov byte ptr [rbp + 0x2df], 0
1415fcda3: lea rcx, [rbp + 0x138]
1415fcdaa: call 0x140896fb0
1415fcdaf: mov byte ptr [rbp + 0x2df], 0
1415fcdb6: add rsp, 0x28
1415fcdba: pop rbx
1415fcdbb: pop rdi
1415fcdbc: pop rsi
1415fcdbd: pop r12
1415fcdbf: pop r13
1415fcdc1: pop r14
1415fcdc3: pop r15
1415fcdc5: pop rbp
1415fcdc6: ret 
1415fcdc7: nop word ptr [rax + rax]
1415fcdd0: mov qword ptr [rsp + 0x10], rdx
1415fcdd5: push rbp
1415fcdd6: push r15
1415fcdd8: push r14
1415fcdda: push r13
1415fcddc: push r12
1415fcdde: push rsi
1415fcddf: push rdi
1415fcde0: push rbx
1415fcde1: sub rsp, 0x28
1415fcde5: lea rbp, [rdx + 0x80]
1415fcdec: mov byte ptr [rbp + 0x2df], 0
1415fcdf3: lea rcx, [rbp - 0x58]
1415fcdf7: call 0x140896fb0
1415fcdfc: mov byte ptr [rbp + 0x2df], 0
1415fce03: add rsp, 0x28
1415fce07: pop rbx
1415fce08: pop rdi
1415fce09: pop rsi
1415fce0a: pop r12
1415fce0c: pop r13
1415fce0e: pop r14
1415fce10: pop r15
1415fce12: pop rbp
1415fce13: ret 
1415fce14: nop word ptr cs:[rax + rax]
1415fce20: mov qword ptr [rsp + 0x10], rdx
1415fce25: push rbp
1415fce26: push r15
1415fce28: push r14
1415fce2a: push r13
1415fce2c: push r12
1415fce2e: push rsi
1415fce2f: push rdi
1415fce30: push rbx
1415fce31: sub rsp, 0x28
1415fce35: lea rbp, [rdx + 0x80]
1415fce3c: mov byte ptr [rbp + 0x2df], 0
1415fce43: lea rcx, [rbp + 0x138]
1415fce4a: call 0x140896fb0
1415fce4f: mov byte ptr [rbp + 0x2df], 0
1415fce56: add rsp, 0x28
1415fce5a: pop rbx
1415fce5b: pop rdi
1415fce5c: pop rsi
1415fce5d: pop r12
1415fce5f: pop r13
1415fce61: pop r14
1415fce63: pop r15
1415fce65: pop rbp
1415fce66: ret 
1415fce67: nop word ptr [rax + rax]
1415fce70: mov qword ptr [rsp + 0x10], rdx
1415fce75: push rbp
1415fce76: push r15
1415fce78: push r14
1415fce7a: push r13
1415fce7c: push r12
1415fce7e: push rsi
1415fce7f: push rdi
1415fce80: push rbx
1415fce81: sub rsp, 0x28
1415fce85: lea rbp, [rdx + 0x80]
1415fce8c: mov byte ptr [rbp + 0x2df], 0
1415fce93: lea rcx, [rbp - 0x58]
1415fce97: call 0x140896fb0
1415fce9c: mov byte ptr [rbp + 0x2df], 0
1415fcea3: add rsp, 0x28
1415fcea7: pop rbx
1415fcea8: pop rdi
1415fcea9: pop rsi
1415fceaa: pop r12
1415fceac: pop r13
1415fceae: pop r14
1415fceb0: pop r15
1415fceb2: pop rbp
1415fceb3: ret 
1415fceb4: nop word ptr cs:[rax + rax]
1415fcec0: mov qword ptr [rsp + 0x10], rdx
1415fcec5: push rbp
1415fcec6: push r15
1415fcec8: push r14
1415fceca: push r13
1415fcecc: push r12
1415fcece: push rsi
1415fcecf: push rdi
1415fced0: push rbx
1415fced1: sub rsp, 0x28
1415fced5: lea rbp, [rdx + 0x80]
1415fcedc: mov byte ptr [rbp + 0x2df], 0
1415fcee3: lea rcx, [rbp + 0x138]
1415fceea: call 0x140896fb0
1415fceef: mov byte ptr [rbp + 0x2df], 0
1415fcef6: add rsp, 0x28
1415fcefa: pop rbx
1415fcefb: pop rdi
1415fcefc: pop rsi
1415fcefd: pop r12
1415fceff: pop r13
1415fcf01: pop r14
1415fcf03: pop r15
1415fcf05: pop rbp
1415fcf06: ret 
