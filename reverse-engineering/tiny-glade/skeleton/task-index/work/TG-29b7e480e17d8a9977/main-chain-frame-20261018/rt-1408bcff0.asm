1408bcff0: push rbp
1408bcff1: push r15
1408bcff3: push r14
1408bcff5: push r13
1408bcff7: push r12
1408bcff9: push rsi
1408bcffa: push rdi
1408bcffb: push rbx
1408bcffc: sub rsp, 0x2d8
1408bd003: lea rbp, [rsp + 0x80]
1408bd00b: mov qword ptr [rbp + 0x250], 0xfffffffffffffffe
1408bd016: mov rbx, r8
1408bd019: mov rdi, rdx
1408bd01c: mov r13, rcx
1408bd01f: call 0x140c16a00
1408bd024: test al, al
1408bd026: je 0x1408bd088
1408bd028: lea r15, [rip + 0x21e359b]
1408bd02f: mov r14d, 0x2c
1408bd035: mov edx, 0x2c
1408bd03a: mov rcx, r15
1408bd03d: call 0x1408b9b00
1408bd042: cmp rax, 1
1408bd046: jne 0x1408bd0fe
1408bd04c: mov r9, rdx
1408bd04f: test rdx, rdx
1408bd052: je 0x1408bd091
1408bd054: cmp r9, 0x2c
1408bd058: jae 0x1408bd08f
1408bd05a: lea rax, [rip + 0x21e3569]
1408bd061: cmp byte ptr [r9 + rax], 0xbf
1408bd066: jg 0x1408bd091
1408bd068: lea rax, [rip + 0x21e3689]
1408bd06f: mov qword ptr [rsp + 0x20], rax
1408bd074: lea rcx, [rip + 0x21e354f]
1408bd07b: mov edx, 0x2c
1408bd080: xor r8d, r8d
1408bd083: call 0x1428d9940
1408bd088: xor eax, eax
1408bd08a: jmp 0x1408bd25d
1408bd08f: jne 0x1408bd068
1408bd091: lea r15, [rip + 0x21e3532]
1408bd098: mov rcx, r15
1408bd09b: mov rdx, r9
1408bd09e: call 0x1408b9b00
1408bd0a3: cmp rax, 1
1408bd0a7: jne 0x1408bd0fe
1408bd0a9: mov r8, rdx
1408bd0ac: add r8, 2
1408bd0b0: je 0x1408bd0eb
1408bd0b2: cmp r8, 0x2c
1408bd0b6: jae 0x1408bd0e9
1408bd0b8: lea rax, [rip + 0x21e350b]
1408bd0bf: cmp byte ptr [r8 + rax], 0xbf
1408bd0c4: jg 0x1408bd0eb
1408bd0c6: lea rax, [rip + 0x21e3643]
1408bd0cd: mov qword ptr [rsp + 0x20], rax
1408bd0d2: lea rcx, [rip + 0x21e34f1]
1408bd0d9: mov edx, 0x2c
1408bd0de: mov r9d, 0x2c
1408bd0e4: call 0x1428d9940
1408bd0e9: jne 0x1408bd0c6
1408bd0eb: mov r14d, 0x2a
1408bd0f1: sub r14, rdx
1408bd0f4: lea r15, [rip + 0x21e34cf]
1408bd0fb: add r15, r8
1408bd0fe: lea rax, [rip + 0x21e38bb]
1408bd105: lea r12, [rip + 0x21e3889]
1408bd10c: nop dword ptr [rax]
1408bd110: movsx ecx, byte ptr [rax - 1]
1408bd114: test ecx, ecx
1408bd116: js 0x1408bd130
1408bd118: dec rax
1408bd11b: cmp ecx, 0x5c
1408bd11e: jne 0x1408bd189
1408bd120: jmp 0x1408bd19e
1408bd122: nop word ptr cs:[rax + rax]
1408bd130: movzx edx, byte ptr [rax - 2]
1408bd134: cmp dl, 0xc0
1408bd137: jge 0x1408bd15e
1408bd139: movzx r8d, byte ptr [rax - 3]
1408bd13e: cmp r8b, 0xc0
1408bd142: jge 0x1408bd167
1408bd144: movzx r9d, byte ptr [rax - 4]
1408bd149: add rax, -4
1408bd14d: and r9d, 7
1408bd151: shl r9d, 6
1408bd155: and r8d, 0x3f
1408bd159: or r8d, r9d
1408bd15c: jmp 0x1408bd16f
1408bd15e: add rax, -2
1408bd162: and edx, 0x1f
1408bd165: jmp 0x1408bd179
1408bd167: add rax, -3
1408bd16b: and r8d, 0xf
1408bd16f: shl r8d, 6
1408bd173: and edx, 0x3f
1408bd176: or edx, r8d
1408bd179: shl edx, 6
1408bd17c: and cl, 0x3f
1408bd17f: movzx ecx, cl
1408bd182: or ecx, edx
1408bd184: cmp ecx, 0x5c
1408bd187: je 0x1408bd19e
1408bd189: cmp ecx, 0x2f
1408bd18c: je 0x1408bd19e
1408bd18e: cmp rax, r12
1408bd191: jne 0x1408bd110
1408bd197: mov esi, 0x2b
1408bd19c: jmp 0x1408bd1ed
1408bd19e: lea r12, [rip + 0x21e37f0]
1408bd1a5: sub rax, r12
1408bd1a8: mov r8, rax
1408bd1ab: inc r8
1408bd1ae: je 0x1408bd1e2
1408bd1b0: cmp r8, 0x2b
1408bd1b4: jae 0x1408bd1e0
1408bd1b6: cmp byte ptr [r8 + r12], 0xbf
1408bd1bb: jg 0x1408bd1e2
1408bd1bd: lea rax, [rip + 0x21e3514]
1408bd1c4: mov qword ptr [rsp + 0x20], rax
1408bd1c9: lea rcx, [rip + 0x21e37c5]
1408bd1d0: mov edx, 0x2b
1408bd1d5: mov r9d, 0x2b
1408bd1db: call 0x1428d9940
1408bd1e0: jne 0x1408bd1bd
1408bd1e2: mov esi, 0x2a
1408bd1e7: sub rsi, rax
1408bd1ea: add r12, r8
1408bd1ed: call 0x140c18130
1408bd1f2: mov rcx, rax
1408bd1f5: mov rax, qword ptr [rax]
1408bd1f8: cmp rax, 1
1408bd1fc: jne 0x1408bdaf0
1408bd202: add rcx, 8
1408bd206: cmp qword ptr [rcx], 0
1408bd20a: jne 0x1408bdb28
1408bd210: mov qword ptr [rcx], 0xffffffffffffffff
1408bd217: mov qword ptr [rbp + 0x240], rcx
1408bd21e: add rcx, 8
1408bd222: mov qword ptr [rsp + 0x20], rsi
1408bd227: mov qword ptr [rsp + 0x30], 0
1408bd230: mov qword ptr [rsp + 0x28], 1
1408bd239: mov rdx, r15
1408bd23c: mov r8, r14
1408bd23f: mov r9, r12
1408bd242: call 0x140c16d40
1408bd247: mov rcx, qword ptr [rbp + 0x240]
1408bd24e: inc qword ptr [rcx]
1408bd251: mov qword ptr [rbp + 0x178], rax
1408bd258: mov eax, 1
1408bd25d: mov qword ptr [rbp + 0x170], rax
1408bd264: mov rax, qword ptr [rbx]
1408bd267: mov rsi, qword ptr [rax]
1408bd26a: lea rbx, [rsi + 0x10]
1408bd26e: mov ecx, 0x3fffffff
1408bd273: xor eax, eax
1408bd275: lock cmpxchg dword ptr [rsi + 0x10], ecx
1408bd27a: jne 0x1408bdb34
1408bd280: mov r15, qword ptr [rip + 0x2d6f199]
1408bd287: mov rax, qword ptr [r15]
1408bd28a: shl rax, 1
1408bd28d: test rax, rax
1408bd290: jne 0x1408bdb41
1408bd296: xor eax, eax
1408bd298: movzx ecx, byte ptr [rsi + 0x18]
1408bd29c: test cl, cl
1408bd29e: jne 0x1408bdb54
1408bd2a4: mov rcx, qword ptr [rsi + 0x60]
1408bd2a8: mov qword ptr [rbp + 0xf0], rcx
1408bd2af: movups xmm0, xmmword ptr [rsi + 0x20]
1408bd2b3: movups xmm1, xmmword ptr [rsi + 0x30]
1408bd2b7: movups xmm2, xmmword ptr [rsi + 0x40]
1408bd2bb: movups xmm3, xmmword ptr [rsi + 0x50]
1408bd2bf: movaps xmmword ptr [rbp + 0xe0], xmm3
1408bd2c6: movaps xmmword ptr [rbp + 0xd0], xmm2
1408bd2cd: movaps xmmword ptr [rbp + 0xc0], xmm1
1408bd2d4: movaps xmmword ptr [rbp + 0xb0], xmm0
1408bd2db: movabs rcx, 0x8000000000000000
1408bd2e5: mov qword ptr [rsi + 0x20], rcx
1408bd2e9: test al, al
1408bd2eb: jne 0x1408bd2fc
1408bd2ed: mov rax, qword ptr [r15]
1408bd2f0: shl rax, 1
1408bd2f3: test rax, rax
1408bd2f6: jne 0x1408bdb91
1408bd2fc: mov edx, 0xc0000001
1408bd301: lock xadd dword ptr [rbx], edx
1408bd305: add edx, 0xc0000001
1408bd30b: cmp edx, 0x40000000
1408bd311: jae 0x1408bdb14
1408bd317: xor eax, eax
1408bd319: cmp rax, qword ptr [rbp + 0xb0]
1408bd320: jo 0x1408bda75
1408bd326: movaps xmm0, xmmword ptr [rbp + 0xb0]
1408bd32d: movaps xmm1, xmmword ptr [rbp + 0xc0]
1408bd334: movaps xmm2, xmmword ptr [rbp + 0xd0]
1408bd33b: movaps xmm3, xmmword ptr [rbp + 0xe0]
1408bd342: movaps xmmword ptr [rbp + 0x120], xmm2
1408bd349: movaps xmmword ptr [rbp + 0x130], xmm3
1408bd350: movaps xmmword ptr [rbp + 0x100], xmm0
1408bd357: movaps xmmword ptr [rbp + 0x110], xmm1
1408bd35e: mov r14, qword ptr [rbp + 0xf0]
1408bd365: mov qword ptr [rbp + 0x140], r14
1408bd36c: mov rsi, qword ptr [rbp + 0x128]
1408bd373: mov rcx, qword ptr [rbp + 0x110]
1408bd37a: mov rax, r14
1408bd37d: or rax, rsi
1408bd380: cmp rax, 0x40000
1408bd386: setb al
1408bd389: mov qword ptr [rbp + 0x240], rcx
1408bd390: cmp rcx, 0x10000
1408bd397: setb cl
1408bd39a: test al, cl
1408bd39c: je 0x1408bd6dc
1408bd3a2: mov r15, qword ptr [rbp + 0x120]
1408bd3a9: mov rax, qword ptr [rbp + 0x138]
1408bd3b0: mov qword ptr [rbp + 0x188], rax
1408bd3b7: mov rax, qword ptr [rbp + 0x108]
1408bd3be: mov qword ptr [rbp + 0x180], rax
1408bd3c5: mov rdi, qword ptr [rdi]
1408bd3c8: lea rdx, [rip + 0x2371009]
1408bd3cf: mov rcx, rdi
1408bd3d2: call 0x140969720
1408bd3d7: mov r12, rax
1408bd3da: test rax, rax
1408bd3dd: je 0x1408bdcdc
1408bd3e3: mov rbx, r13
1408bd3e6: lea rax, [rip + 0x21e0843]
1408bd3ed: mov qword ptr [rbp + 0x210], rax
1408bd3f4: mov qword ptr [rbp + 0x218], 0x2c
1408bd3ff: mov r13, r12
1408bd402: add r13, 0x10
1408bd406: cmp qword ptr [r12 + 0x18], 0x2c
1408bd40c: jne 0x1408bdbf0
1408bd412: mov rdx, qword ptr [r13]
1408bd416: lea rcx, [rip + 0x21e0813]
1408bd41d: mov r8d, 0x2c
1408bd423: call 0x142923bf0
1408bd428: test eax, eax
1408bd42a: jne 0x1408bdbf0
1408bd430: mov rcx, qword ptr [r12]
1408bd434: mov rax, qword ptr [r12 + 8]
1408bd439: call qword ptr [rax + 0x18]
1408bd43c: mov r12, rax
1408bd43f: test rax, rax
1408bd442: je 0x1408bdcdc
1408bd448: mov r13, qword ptr [rbx]
1408bd44b: test rsi, rsi
1408bd44e: je 0x1408bd4cb
1408bd450: lea rax, [rsi*8]
1408bd458: lea rax, [rax + rax*2]
1408bd45c: cmp rax, qword ptr [r12 + 0x20]
1408bd461: ja 0x1408bdd75
1408bd467: mov rdx, qword ptr [r13]
1408bd46b: add rdx, 0x10
1408bd46f: mov r8, r12
1408bd472: add r8, 8
1408bd476: mov qword ptr [rbp + 0x210], 0
1408bd481: mov qword ptr [rbp + 0x218], rax
1408bd488: lea rcx, [rbp + 0x210]
1408bd48f: mov qword ptr [rsp + 0x28], rcx
1408bd494: mov qword ptr [rsp + 0x20], rax
1408bd499: mov qword ptr [rsp + 0x38], 0
1408bd4a2: mov qword ptr [rsp + 0x30], 1
1408bd4ab: lea rcx, [rbp + 0x190]
1408bd4b2: mov r9, r15
1408bd4b5: call 0x140beefa0
1408bd4ba: mov rax, qword ptr [rbp + 0x190]
1408bd4c1: cmp rax, 0xb
1408bd4c5: jne 0x1408bdd7c
1408bd4cb: mov qword ptr [r12], rsi
1408bd4cf: lea rdx, [rip + 0x2370d9a]
1408bd4d6: mov rcx, rdi
1408bd4d9: call 0x140969720
1408bd4de: mov rsi, rax
1408bd4e1: test rax, rax
1408bd4e4: je 0x1408bdced
1408bd4ea: lea rax, [rip + 0x21dfa84]
1408bd4f1: mov qword ptr [rbp + 0x210], rax
1408bd4f8: mov qword ptr [rbp + 0x218], 0x2f
1408bd503: mov r15, rsi
1408bd506: add r15, 0x10
1408bd50a: cmp qword ptr [rsi + 0x18], 0x2f
1408bd50f: jne 0x1408bdc5e
1408bd515: mov rdx, qword ptr [r15]
1408bd518: lea rcx, [rip + 0x21dfa56]
1408bd51f: mov r8d, 0x2f
1408bd525: call 0x142923bf0
1408bd52a: test eax, eax
1408bd52c: jne 0x1408bdc5e
1408bd532: mov rcx, qword ptr [rsi]
1408bd535: mov rax, qword ptr [rsi + 8]
1408bd539: call qword ptr [rax + 0x18]
1408bd53c: mov rsi, rax
1408bd53f: test rax, rax
1408bd542: je 0x1408bdced
1408bd548: test r14, r14
1408bd54b: je 0x1408bd5c7
1408bd54d: lea rax, [r14*4]
1408bd555: cmp rax, qword ptr [rsi + 0x20]
1408bd559: ja 0x1408bde33
1408bd55f: mov rdx, qword ptr [r13]
1408bd563: add rdx, 0x10
1408bd567: mov r8, rsi
1408bd56a: add r8, 8
1408bd56e: mov qword ptr [rbp + 0x210], 0
1408bd579: mov qword ptr [rbp + 0x218], rax
1408bd580: lea rcx, [rbp + 0x210]
1408bd587: mov qword ptr [rsp + 0x28], rcx
1408bd58c: mov qword ptr [rsp + 0x20], rax
1408bd591: mov qword ptr [rsp + 0x38], 0
1408bd59a: mov qword ptr [rsp + 0x30], 1
1408bd5a3: lea rcx, [rbp + 0x190]
1408bd5aa: mov r9, qword ptr [rbp + 0x188]
1408bd5b1: call 0x140beefa0
1408bd5b6: mov rax, qword ptr [rbp + 0x190]
1408bd5bd: cmp rax, 0xb
1408bd5c1: jne 0x1408bde3a
1408bd5c7: mov qword ptr [rsi], r14
1408bd5ca: lea rdx, [rip + 0x2370d7f]
1408bd5d1: mov rcx, rdi
1408bd5d4: call 0x140969720
1408bd5d9: mov rsi, rax
1408bd5dc: test rax, rax
1408bd5df: je 0x1408bdcfe
1408bd5e5: lea rax, [rip + 0x21e113a]
1408bd5ec: mov qword ptr [rbp + 0x210], rax
1408bd5f3: mov qword ptr [rbp + 0x218], 0x26
1408bd5fe: mov rdi, rsi
1408bd601: add rdi, 0x10
1408bd605: cmp qword ptr [rsi + 0x18], 0x26
1408bd60a: jne 0x1408bdc92
1408bd610: mov rdx, qword ptr [rdi]
1408bd613: lea rcx, [rip + 0x21e110c]
1408bd61a: mov r8d, 0x26
1408bd620: call 0x142923bf0
1408bd625: test eax, eax
1408bd627: jne 0x1408bdc92
1408bd62d: mov rcx, qword ptr [rsi]
1408bd630: mov rax, qword ptr [rsi + 8]
1408bd634: call qword ptr [rax + 0x18]
1408bd637: mov rsi, rax
1408bd63a: test rax, rax
1408bd63d: je 0x1408bdcfe
1408bd643: mov rax, qword ptr [rbp + 0x240]
1408bd64a: test rax, rax
1408bd64d: je 0x1408bd6d4
1408bd653: shl rax, 4
1408bd657: lea rax, [rax + rax*4]
1408bd65b: cmp rax, qword ptr [rsi + 0x20]
1408bd65f: ja 0x1408bdedf
1408bd665: mov rdx, qword ptr [r13]
1408bd669: add rdx, 0x10
1408bd66d: mov r8, rsi
1408bd670: add r8, 8
1408bd674: mov qword ptr [rbp + 0x210], 0
1408bd67f: mov qword ptr [rbp + 0x218], rax
1408bd686: lea rcx, [rbp + 0x210]
1408bd68d: mov qword ptr [rsp + 0x28], rcx
1408bd692: mov qword ptr [rsp + 0x20], rax
1408bd697: mov qword ptr [rsp + 0x38], 0
1408bd6a0: mov qword ptr [rsp + 0x30], 1
1408bd6a9: lea rcx, [rbp + 0x190]
1408bd6b0: mov r9, qword ptr [rbp + 0x180]
1408bd6b7: call 0x140beefa0
1408bd6bc: mov rax, qword ptr [rbp + 0x190]
1408bd6c3: cmp rax, 0xb
1408bd6c7: jne 0x1408bdee6
1408bd6cd: mov rax, qword ptr [rbp + 0x240]
1408bd6d4: mov qword ptr [rsi], rax
1408bd6d7: jmp 0x1408bda07
1408bd6dc: mov eax, dword ptr [rip + 0x2d775c6]
1408bd6e2: cmp eax, 3
1408bd6e5: jne 0x1408bdbae
1408bd6eb: mov rdx, qword ptr [rip + 0x2d775ae]
1408bd6f2: lea rcx, [rbp + 0x190]
1408bd6f9: call 0x140542890
1408bd6fe: cmp dword ptr [rbp + 0x190], 1
1408bd705: je 0x1408bdd0f
1408bd70b: mov rax, qword ptr [rbp + 0x198]
1408bd712: mov qword ptr [rbp + 0x188], rax
1408bd719: movzx eax, byte ptr [rbp + 0x1a0]
1408bd720: mov byte ptr [rbp + 0x180], al
1408bd726: movzx eax, byte ptr [rip + 0x2d774dc]
1408bd72d: mov ecx, 0xf
1408bd732: mov edx, 1
1408bd737: call 0x140613c10
1408bd73c: test rax, rax
1408bd73f: je 0x1408bdd5a
1408bd745: movabs rcx, 0x2928202168746170
1408bd74f: mov qword ptr [rax + 7], rcx
1408bd753: movabs rcx, 0x705f656c75646f6d
1408bd75d: mov qword ptr [rax], rcx
1408bd760: mov qword ptr [rbp + 0x190], 0xf
1408bd76b: mov qword ptr [rbp + 0x198], rax
1408bd772: mov qword ptr [rbp + 0x1a0], 0xf
1408bd77d: mov qword ptr [rsp + 0x20], 1
1408bd786: lea rcx, [rbp + 0x190]
1408bd78d: mov edx, 0xf
1408bd792: mov r8d, 0x18
1408bd798: mov r9d, 1
1408bd79e: call 0x1428a9a60
1408bd7a3: mov rax, qword ptr [rbp + 0x198]
1408bd7aa: mov rdx, qword ptr [rbp + 0x1a0]
1408bd7b1: movups xmm0, xmmword ptr [rip + 0x21e32e3]
1408bd7b8: movups xmmword ptr [rax + rdx], xmm0
1408bd7bc: movabs rcx, 0x726f727245203a3a
1408bd7c6: mov qword ptr [rax + rdx + 0x10], rcx
1408bd7cb: add rdx, 0x18
1408bd7cf: mov qword ptr [rbp + 0x220], rdx
1408bd7d6: movups xmm0, xmmword ptr [rbp + 0x190]
1408bd7dd: movaps xmmword ptr [rbp + 0x210], xmm0
1408bd7e4: mov rax, qword ptr [rbp + 0x210]
1408bd7eb: sub rax, rdx
1408bd7ee: cmp rax, 0x35
1408bd7f2: jbe 0x1408bdc24
1408bd7f8: mov rax, qword ptr [rbp + 0x218]
1408bd7ff: movups xmm0, xmmword ptr [rip + 0x21e32cd]
1408bd806: movups xmmword ptr [rax + rdx + 0x20], xmm0
1408bd80b: movups xmm0, xmmword ptr [rip + 0x21e32b1]
1408bd812: movups xmmword ptr [rax + rdx + 0x10], xmm0
1408bd817: movups xmm0, xmmword ptr [rip + 0x21e3295]
1408bd81e: movups xmmword ptr [rax + rdx], xmm0
1408bd822: movabs rcx, 0x617461642d676e69
1408bd82c: mov qword ptr [rax + rdx + 0x2e], rcx
1408bd831: add rdx, 0x36
1408bd835: mov qword ptr [rbp + 0x220], rdx
1408bd83c: mov qword ptr [rbp + 0x1a0], rdx
1408bd843: movaps xmm0, xmmword ptr [rbp + 0x210]
1408bd84a: movaps xmmword ptr [rbp + 0x190], xmm0
1408bd851: mov rax, qword ptr [rbp + 0x188]
1408bd858: lea rcx, [rax + 8]
1408bd85c: lea rdi, [rbp + 0x190]
1408bd863: mov rdx, rdi
1408bd866: call 0x14143b8b0
1408bd86b: test al, al
1408bd86d: jne 0x1408bd938
1408bd873: mov rax, qword ptr [rip + 0x2d6eb2e]
1408bd87a: mov rax, qword ptr [rax]
1408bd87d: test rax, rax
1408bd880: je 0x1408bd938
1408bd886: lea rcx, [rip + 0x21e31e3]
1408bd88d: call 0x14067dd80
1408bd892: movups xmm0, xmmword ptr [rax]
1408bd895: mov eax, dword ptr [rax + 0x10]
1408bd898: mov qword ptr [rbp + 0x1c0], 1
1408bd8a3: lea rdx, [rip + 0x21e3176]
1408bd8aa: mov qword ptr [rbp + 0x1c8], rdx
1408bd8b1: mov qword ptr [rbp + 0x1d0], 0x1b
1408bd8bc: lea rcx, [rip + 0x21e322d]
1408bd8c3: mov qword ptr [rbp + 0x1e0], rcx
1408bd8ca: mov qword ptr [rbp + 0x1e8], 1
1408bd8d5: lea rcx, [rbp + 0x210]
1408bd8dc: mov qword ptr [rbp + 0x1f0], rcx
1408bd8e3: mov qword ptr [rbp + 0x190], 0
1408bd8ee: xorps xmm1, xmm1
1408bd8f1: movups xmmword ptr [rbp + 0x1f8], xmm1
1408bd8f8: mov qword ptr [rbp + 0x198], rdx
1408bd8ff: mov qword ptr [rbp + 0x1a0], 0x1b
1408bd90a: mov qword ptr [rbp + 0x1a8], 0
1408bd915: movups xmmword ptr [rbp + 0x1b0], xmm0
1408bd91c: mov dword ptr [rbp + 0x1d8], 1
1408bd926: mov dword ptr [rbp + 0x1dc], eax
1408bd92c: lea rdx, [rbp + 0x190]
1408bd933: call 0x140d4a330
1408bd938: cmp byte ptr [rbp + 0x180], 0
1408bd93f: mov rbx, qword ptr [rbp + 0x188]
1408bd946: jne 0x1408bd957
1408bd948: mov rax, qword ptr [r15]
1408bd94b: shl rax, 1
1408bd94e: test rax, rax
1408bd951: jne 0x1408bdcc6
1408bd957: xor eax, eax
1408bd959: xchg byte ptr [rbx], al
1408bd95b: cmp al, 2
1408bd95d: je 0x1408bdc51
1408bd963: mov qword ptr [rbp + 0x168], rsi
1408bd96a: mov qword ptr [rbp + 0x160], r14
1408bd971: mov rax, qword ptr [rbp + 0x240]
1408bd978: mov qword ptr [rbp + 0x158], rax
1408bd97f: lea rax, [rbp + 0x168]
1408bd986: mov qword ptr [rbp + 0x190], rax
1408bd98d: lea rax, [rip + 0x4cfd2c]
1408bd994: mov qword ptr [rbp + 0x198], rax
1408bd99b: lea rcx, [rbp + 0x160]
1408bd9a2: mov qword ptr [rbp + 0x1a0], rcx
1408bd9a9: mov qword ptr [rbp + 0x1a8], rax
1408bd9b0: lea rcx, [rbp + 0x158]
1408bd9b7: mov qword ptr [rbp + 0x1b0], rcx
1408bd9be: mov qword ptr [rbp + 0x1b8], rax
1408bd9c5: lea rax, [rip + 0x21e3164]
1408bd9cc: mov qword ptr [rbp + 0x210], rax
1408bd9d3: mov qword ptr [rbp + 0x218], 4
1408bd9de: mov qword ptr [rbp + 0x230], 0
1408bd9e9: mov qword ptr [rbp + 0x220], rdi
1408bd9f0: mov qword ptr [rbp + 0x228], 3
1408bd9fb: lea rcx, [rbp + 0x210]
1408bda02: call 0x140d5caa0
1408bda07: mov rax, qword ptr [rbp + 0x100]
1408bda0e: test rax, rax
1408bda11: je 0x1408bda2d
1408bda13: mov rcx, qword ptr [rbp + 0x108]
1408bda1a: shl rax, 4
1408bda1e: lea rdx, [rax + rax*4]
1408bda22: mov r8d, 4
1408bda28: call 0x140613c20
1408bda2d: mov rax, qword ptr [rbp + 0x118]
1408bda34: test rax, rax
1408bda37: je 0x1408bda53
1408bda39: mov rcx, qword ptr [rbp + 0x120]
1408bda40: shl rax, 3
1408bda44: lea rdx, [rax + rax*2]
1408bda48: mov r8d, 4
1408bda4e: call 0x140613c20
1408bda53: mov rdx, qword ptr [rbp + 0x130]
1408bda5a: test rdx, rdx
1408bda5d: je 0x1408bda75
1408bda5f: mov rcx, qword ptr [rbp + 0x138]
1408bda66: shl rdx, 2
1408bda6a: mov r8d, 4
1408bda70: call 0x140613c20
1408bda75: cmp qword ptr [rbp + 0x170], 0
1408bda7d: je 0x1408bdac6
1408bda7f: call 0x140c18130
1408bda84: mov rcx, rax
1408bda87: mov rax, qword ptr [rax]
1408bda8a: cmp rax, 1
1408bda8e: jne 0x1408bdada
1408bda90: add rcx, 8
1408bda94: cmp qword ptr [rcx], 0
1408bda98: jne 0x1408bdb28
1408bda9e: mov qword ptr [rcx], 0xffffffffffffffff
1408bdaa5: mov qword ptr [rbp + 0x240], rcx
1408bdaac: add rcx, 8
1408bdab0: mov rdx, qword ptr [rbp + 0x178]
1408bdab7: call 0x140c170f0
1408bdabc: mov rax, qword ptr [rbp + 0x240]
1408bdac3: inc qword ptr [rax]
1408bdac6: add rsp, 0x2d8
1408bdacd: pop rbx
1408bdace: pop rdi
1408bdacf: pop rsi
1408bdad0: pop r12
1408bdad2: pop r13
1408bdad4: pop r14
1408bdad6: pop r15
1408bdad8: pop rbp
1408bdad9: ret 
1408bdada: test rax, rax
1408bdadd: jne 0x1408bdb08
1408bdadf: xor edx, edx
1408bdae1: call 0x1428f87d0
1408bdae6: mov rcx, rax
1408bdae9: test rax, rax
1408bdaec: jne 0x1408bda94
1408bdaee: jmp 0x1408bdb08
1408bdaf0: test rax, rax
1408bdaf3: jne 0x1408bdb08
1408bdaf5: xor edx, edx
1408bdaf7: call 0x1428f87d0
1408bdafc: mov rcx, rax
1408bdaff: test rax, rax
1408bdb02: jne 0x1408bd206
1408bdb08: lea rcx, [rip + 0x237cf91]
1408bdb0f: call 0x1428d80d0
1408bdb14: mov byte ptr [rbp + 0x24f], 1
1408bdb1b: mov rcx, rbx
1408bdb1e: call 0x1428d8d40
1408bdb23: jmp 0x1408bd317
1408bdb28: lea rcx, [rip + 0x237d041]
1408bdb2f: call 0x1428d9250
1408bdb34: mov rcx, rbx
1408bdb37: call 0x1428d8c10
1408bdb3c: jmp 0x1408bd280
1408bdb41: call 0x1428d86b0
1408bdb46: xor al, 1
1408bdb48: movzx ecx, byte ptr [rsi + 0x18]
1408bdb4c: test cl, cl
1408bdb4e: je 0x1408bd2a4
1408bdb54: mov qword ptr [rbp + 0x190], rbx
1408bdb5b: mov byte ptr [rbp + 0x198], al
1408bdb61: lea rax, [rip + 0x21e2ef0]
1408bdb68: mov qword ptr [rsp + 0x20], rax
1408bdb6d: lea rcx, [rip + 0x21e2844]
1408bdb74: lea r9, [rip + 0x21e290d]
1408bdb7b: lea r8, [rbp + 0x190]
1408bdb82: mov edx, 0x2b
1408bdb87: call 0x1428d9760
1408bdb8c: jmp 0x1408bdf86
1408bdb91: mov byte ptr [rbp + 0x24f], 1
1408bdb98: call 0x1428d86b0
1408bdb9d: test al, al
1408bdb9f: jne 0x1408bd2fc
1408bdba5: mov byte ptr [rsi + 0x18], 1
1408bdba9: jmp 0x1408bd2fc
1408bdbae: mov byte ptr [rbp + 0x210], 1
1408bdbb5: lea rax, [rbp + 0x210]
1408bdbbc: mov qword ptr [rbp + 0x190], rax
1408bdbc3: lea rax, [rip + 0x21e2ea6]
1408bdbca: mov qword ptr [rsp + 0x20], rax
1408bdbcf: lea rcx, [rip + 0x2d770d2]
1408bdbd6: lea r9, [rip + 0x21e25c3]
1408bdbdd: lea r8, [rbp + 0x190]
1408bdbe4: xor edx, edx
1408bdbe6: call 0x1428d8820
1408bdbeb: jmp 0x1408bd6eb
1408bdbf0: mov qword ptr [rbp + 0x190], 0
1408bdbfb: lea rax, [rip + 0x237057e]
1408bdc02: mov qword ptr [rsp + 0x20], rax
1408bdc07: lea rdx, [rbp + 0x210]
1408bdc0e: lea r9, [rbp + 0x190]
1408bdc15: xor ecx, ecx
1408bdc17: mov r8, r13
1408bdc1a: call 0x1428cab41
1408bdc1f: jmp 0x1408bdf86
1408bdc24: mov qword ptr [rsp + 0x20], 1
1408bdc2d: lea rcx, [rbp + 0x210]
1408bdc34: mov r8d, 0x36
1408bdc3a: mov r9d, 1
1408bdc40: call 0x1428a9a60
1408bdc45: mov rdx, qword ptr [rbp + 0x220]
1408bdc4c: jmp 0x1408bd7f8
1408bdc51: mov rcx, rbx
1408bdc54: call 0x1428d8810
1408bdc59: jmp 0x1408bd963
1408bdc5e: mov qword ptr [rbp + 0x190], 0
1408bdc69: lea rax, [rip + 0x2370510]
1408bdc70: mov qword ptr [rsp + 0x20], rax
1408bdc75: lea rdx, [rbp + 0x210]
1408bdc7c: lea r9, [rbp + 0x190]
1408bdc83: xor ecx, ecx
1408bdc85: mov r8, r15
1408bdc88: call 0x1428cab41
1408bdc8d: jmp 0x1408bdf86
1408bdc92: mov qword ptr [rbp + 0x190], 0
1408bdc9d: lea rax, [rip + 0x23704dc]
1408bdca4: mov qword ptr [rsp + 0x20], rax
1408bdca9: lea rdx, [rbp + 0x210]
1408bdcb0: lea r9, [rbp + 0x190]
1408bdcb7: xor ecx, ecx
1408bdcb9: mov r8, rdi
1408bdcbc: call 0x1428cab41
1408bdcc1: jmp 0x1408bdf86
1408bdcc6: call 0x1428d86b0
1408bdccb: test al, al
1408bdccd: jne 0x1408bd957
1408bdcd3: mov byte ptr [rbx + 1], 1
1408bdcd7: jmp 0x1408bd957
1408bdcdc: lea rcx, [rip + 0x21e2e8d]
1408bdce3: call 0x1428d9310
1408bdce8: jmp 0x1408bdf86
1408bdced: lea rcx, [rip + 0x21e2eac]
1408bdcf4: call 0x1428d9310
1408bdcf9: jmp 0x1408bdf86
1408bdcfe: lea rcx, [rip + 0x21e2ecb]
1408bdd05: call 0x1428d9310
1408bdd0a: jmp 0x1408bdf86
1408bdd0f: mov rax, qword ptr [rbp + 0x198]
1408bdd16: movzx ecx, byte ptr [rbp + 0x1a0]
1408bdd1d: mov qword ptr [rbp + 0x210], rax
1408bdd24: mov byte ptr [rbp + 0x218], cl
1408bdd2a: lea rax, [rip + 0x21e2d3f]
1408bdd31: mov qword ptr [rsp + 0x20], rax
1408bdd36: lea rcx, [rip + 0x21e2d4b]
1408bdd3d: lea r9, [rip + 0x21e2614]
1408bdd44: lea r8, [rbp + 0x210]
1408bdd4b: mov edx, 0x13
1408bdd50: call 0x1428d9760
1408bdd55: jmp 0x1408bdf86
1408bdd5a: lea r8, [rip + 0x21e27bf]
1408bdd61: mov ecx, 1
1408bdd66: mov edx, 0xf
1408bdd6b: call 0x1428d8fe3
1408bdd70: jmp 0x1408bdf86
1408bdd75: mov eax, 0xb
1408bdd7a: jmp 0x1408bddbc
1408bdd7c: mov rcx, qword ptr [rbp + 0x1d8]
1408bdd83: mov qword ptr [rbp + 0xa0], rcx
1408bdd8a: movups xmm0, xmmword ptr [rbp + 0x198]
1408bdd91: movups xmm1, xmmword ptr [rbp + 0x1a8]
1408bdd98: movups xmm2, xmmword ptr [rbp + 0x1b8]
1408bdd9f: movups xmm3, xmmword ptr [rbp + 0x1c8]
1408bdda6: movaps xmmword ptr [rbp + 0x90], xmm3
1408bddad: movaps xmmword ptr [rbp + 0x80], xmm2
1408bddb4: movaps xmmword ptr [rbp + 0x70], xmm1
1408bddb8: movaps xmmword ptr [rbp + 0x60], xmm0
1408bddbc: mov qword ptr [rbp + 0x190], rax
1408bddc3: movaps xmm0, xmmword ptr [rbp + 0x60]
1408bddc7: movaps xmm1, xmmword ptr [rbp + 0x70]
1408bddcb: movaps xmm2, xmmword ptr [rbp + 0x80]
1408bddd2: movaps xmm3, xmmword ptr [rbp + 0x90]
1408bddd9: movups xmmword ptr [rbp + 0x198], xmm0
1408bdde0: movups xmmword ptr [rbp + 0x1a8], xmm1
1408bdde7: movups xmmword ptr [rbp + 0x1b8], xmm2
1408bddee: movups xmmword ptr [rbp + 0x1c8], xmm3
1408bddf5: mov rax, qword ptr [rbp + 0xa0]
1408bddfc: mov qword ptr [rbp + 0x1d8], rax
1408bde03: lea rax, [rip + 0x21e2d7e]
1408bde0a: mov qword ptr [rsp + 0x20], rax
1408bde0f: lea rcx, [rip + 0x21e25a2]
1408bde16: lea r9, [rip + 0x21e25eb]
1408bde1d: lea r8, [rbp + 0x190]
1408bde24: mov edx, 0x2b
1408bde29: call 0x1428d9760
1408bde2e: jmp 0x1408bdf86
1408bde33: mov eax, 0xb
1408bde38: jmp 0x1408bde71
1408bde3a: mov rcx, qword ptr [rbp + 0x1d8]
1408bde41: mov qword ptr [rbp + 0x50], rcx
1408bde45: movups xmm0, xmmword ptr [rbp + 0x198]
1408bde4c: movups xmm1, xmmword ptr [rbp + 0x1a8]
1408bde53: movups xmm2, xmmword ptr [rbp + 0x1b8]
1408bde5a: movups xmm3, xmmword ptr [rbp + 0x1c8]
1408bde61: movaps xmmword ptr [rbp + 0x40], xmm3
1408bde65: movaps xmmword ptr [rbp + 0x30], xmm2
1408bde69: movaps xmmword ptr [rbp + 0x20], xmm1
1408bde6d: movaps xmmword ptr [rbp + 0x10], xmm0
1408bde71: mov qword ptr [rbp + 0x190], rax
1408bde78: movaps xmm0, xmmword ptr [rbp + 0x10]
1408bde7c: movaps xmm1, xmmword ptr [rbp + 0x20]
1408bde80: movaps xmm2, xmmword ptr [rbp + 0x30]
1408bde84: movaps xmm3, xmmword ptr [rbp + 0x40]
1408bde88: movups xmmword ptr [rbp + 0x198], xmm0
1408bde8f: movups xmmword ptr [rbp + 0x1a8], xmm1
1408bde96: movups xmmword ptr [rbp + 0x1b8], xmm2
1408bde9d: movups xmmword ptr [rbp + 0x1c8], xmm3
1408bdea4: mov rax, qword ptr [rbp + 0x50]
1408bdea8: mov qword ptr [rbp + 0x1d8], rax
1408bdeaf: lea rax, [rip + 0x21e2d02]
1408bdeb6: mov qword ptr [rsp + 0x20], rax
1408bdebb: lea rcx, [rip + 0x21e24f6]
1408bdec2: lea r9, [rip + 0x21e253f]
1408bdec9: lea r8, [rbp + 0x190]
1408bded0: mov edx, 0x2b
1408bded5: call 0x1428d9760
1408bdeda: jmp 0x1408bdf86
1408bdedf: mov eax, 0xb
1408bdee4: jmp 0x1408bdf1d
1408bdee6: mov rcx, qword ptr [rbp + 0x1d8]
1408bdeed: mov qword ptr [rbp], rcx
1408bdef1: movups xmm0, xmmword ptr [rbp + 0x198]
1408bdef8: movups xmm1, xmmword ptr [rbp + 0x1a8]
1408bdeff: movups xmm2, xmmword ptr [rbp + 0x1b8]
1408bdf06: movups xmm3, xmmword ptr [rbp + 0x1c8]
1408bdf0d: movaps xmmword ptr [rbp - 0x10], xmm3
1408bdf11: movaps xmmword ptr [rbp - 0x20], xmm2
1408bdf15: movaps xmmword ptr [rbp - 0x30], xmm1
1408bdf19: movaps xmmword ptr [rbp - 0x40], xmm0
1408bdf1d: mov qword ptr [rbp + 0x190], rax
1408bdf24: movaps xmm0, xmmword ptr [rbp - 0x40]
1408bdf28: movaps xmm1, xmmword ptr [rbp - 0x30]
1408bdf2c: movaps xmm2, xmmword ptr [rbp - 0x20]
1408bdf30: movaps xmm3, xmmword ptr [rbp - 0x10]
1408bdf34: movups xmmword ptr [rbp + 0x198], xmm0
1408bdf3b: movups xmmword ptr [rbp + 0x1a8], xmm1
1408bdf42: movups xmmword ptr [rbp + 0x1b8], xmm2
1408bdf49: movups xmmword ptr [rbp + 0x1c8], xmm3
1408bdf50: mov rax, qword ptr [rbp]
1408bdf54: mov qword ptr [rbp + 0x1d8], rax
1408bdf5b: lea rax, [rip + 0x21e2c86]
1408bdf62: mov qword ptr [rsp + 0x20], rax
1408bdf67: lea rcx, [rip + 0x21e244a]
1408bdf6e: lea r9, [rip + 0x21e2493]
1408bdf75: lea r8, [rbp + 0x190]
1408bdf7c: mov edx, 0x2b
1408bdf81: call 0x1428d9760
1408bdf86: ud2 
1408bdf88: nop dword ptr [rax + rax]
1408bdf90: mov qword ptr [rsp + 0x10], rdx
1408bdf95: push rbp
1408bdf96: push r15
1408bdf98: push r14
1408bdf9a: push r13
1408bdf9c: push r12
1408bdf9e: push rsi
1408bdf9f: push rdi
1408bdfa0: push rbx
1408bdfa1: sub rsp, 0x48
1408bdfa5: lea rbp, [rdx + 0x80]
1408bdfac: mov rax, qword ptr [rbp + 0x240]
1408bdfb3: inc qword ptr [rax]
1408bdfb6: add rsp, 0x48
1408bdfba: pop rbx
1408bdfbb: pop rdi
1408bdfbc: pop rsi
1408bdfbd: pop r12
1408bdfbf: pop r13
1408bdfc1: pop r14
1408bdfc3: pop r15
1408bdfc5: pop rbp
1408bdfc6: ret 
1408bdfc7: nop word ptr [rax + rax]
1408bdfd0: mov qword ptr [rsp + 0x10], rdx
1408bdfd5: push rbp
1408bdfd6: push r15
1408bdfd8: push r14
1408bdfda: push r13
1408bdfdc: push r12
1408bdfde: push rsi
1408bdfdf: push rdi
1408bdfe0: push rbx
1408bdfe1: sub rsp, 0x48
1408bdfe5: lea rbp, [rdx + 0x80]
1408bdfec: cmp qword ptr [rbp + 0x170], 0
1408bdff4: je 0x1408be025
1408bdff6: lea rax, [rbp + 0x178]
1408bdffd: mov qword ptr [rbp + 0x150], rax
1408be004: lea rax, [rbp + 0x150]
1408be00b: mov qword ptr [rbp + 0x148], rax
1408be012: lea rcx, [rip + 0x21e265f]
1408be019: lea rdx, [rbp + 0x148]
1408be020: call 0x14132f2d0
1408be025: nop 
1408be026: add rsp, 0x48
1408be02a: pop rbx
1408be02b: pop rdi
1408be02c: pop rsi
1408be02d: pop r12
1408be02f: pop r13
1408be031: pop r14
1408be033: pop r15
1408be035: pop rbp
1408be036: ret 
1408be037: nop word ptr [rax + rax]
1408be040: mov qword ptr [rsp + 0x10], rdx
1408be045: push rbp
1408be046: push r15
1408be048: push r14
1408be04a: push r13
1408be04c: push r12
1408be04e: push rsi
1408be04f: push rdi
1408be050: push rbx
1408be051: sub rsp, 0x48
1408be055: lea rbp, [rdx + 0x80]
1408be05c: mov rcx, qword ptr [rbp + 0x188]
1408be063: movzx edx, byte ptr [rbp + 0x180]
1408be06a: call 0x140043000
1408be06f: nop 
1408be070: add rsp, 0x48
1408be074: pop rbx
1408be075: pop rdi
1408be076: pop rsi
1408be077: pop r12
1408be079: pop r13
1408be07b: pop r14
1408be07d: pop r15
1408be07f: pop rbp
1408be080: ret 
1408be081: nop word ptr cs:[rax + rax]
1408be090: mov qword ptr [rsp + 0x10], rdx
1408be095: push rbp
1408be096: push r15
1408be098: push r14
1408be09a: push r13
1408be09c: push r12
1408be09e: push rsi
1408be09f: push rdi
1408be0a0: push rbx
1408be0a1: sub rsp, 0x48
1408be0a5: lea rbp, [rdx + 0x80]
1408be0ac: mov rdx, qword ptr [rbp + 0x190]
1408be0b3: test rdx, rdx
1408be0b6: je 0x1408be0ca
1408be0b8: mov rcx, qword ptr [rbp + 0x198]
1408be0bf: mov r8d, 1
1408be0c5: call 0x140613c20
1408be0ca: nop 
1408be0cb: add rsp, 0x48
1408be0cf: pop rbx
1408be0d0: pop rdi
1408be0d1: pop rsi
1408be0d2: pop r12
1408be0d4: pop r13
1408be0d6: pop r14
1408be0d8: pop r15
1408be0da: pop rbp
1408be0db: ret 
1408be0dc: nop dword ptr [rax]
1408be0e0: mov qword ptr [rsp + 0x10], rdx
1408be0e5: push rbp
1408be0e6: push r15
1408be0e8: push r14
1408be0ea: push r13
1408be0ec: push r12
1408be0ee: push rsi
1408be0ef: push rdi
1408be0f0: push rbx
1408be0f1: sub rsp, 0x48
1408be0f5: lea rbp, [rdx + 0x80]
1408be0fc: mov rdx, qword ptr [rbp + 0x210]
1408be103: test rdx, rdx
1408be106: je 0x1408be11a
1408be108: mov rcx, qword ptr [rbp + 0x218]
1408be10f: mov r8d, 1
1408be115: call 0x140613c20
1408be11a: nop 
1408be11b: add rsp, 0x48
1408be11f: pop rbx
1408be120: pop rdi
1408be121: pop rsi
1408be122: pop r12
1408be124: pop r13
1408be126: pop r14
1408be128: pop r15
1408be12a: pop rbp
1408be12b: ret 
1408be12c: nop dword ptr [rax]
1408be130: mov qword ptr [rsp + 0x10], rdx
1408be135: push rbp
1408be136: push r15
1408be138: push r14
1408be13a: push r13
1408be13c: push r12
1408be13e: push rsi
1408be13f: push rdi
1408be140: push rbx
1408be141: sub rsp, 0x48
1408be145: lea rbp, [rdx + 0x80]
1408be14c: mov rax, qword ptr [rbp + 0x240]
1408be153: inc qword ptr [rax]
1408be156: add rsp, 0x48
1408be15a: pop rbx
1408be15b: pop rdi
1408be15c: pop rsi
1408be15d: pop r12
1408be15f: pop r13
1408be161: pop r14
1408be163: pop r15
1408be165: pop rbp
1408be166: ret 
1408be167: nop word ptr [rax + rax]
1408be170: mov qword ptr [rsp + 0x10], rdx
1408be175: push rbp
1408be176: push r15
1408be178: push r14
1408be17a: push r13
1408be17c: push r12
1408be17e: push rsi
1408be17f: push rdi
1408be180: push rbx
1408be181: sub rsp, 0x48
1408be185: lea rbp, [rdx + 0x80]
1408be18c: xor eax, eax
1408be18e: cmp rax, qword ptr [rbp + 0xb0]
1408be195: jo 0x1408be1be
1408be197: test byte ptr [rbp + 0x24f], 1
1408be19e: je 0x1408be1cf
1408be1a0: lea rcx, [rbp + 0xb0]
1408be1a7: call 0x14088b0c0
1408be1ac: nop 
1408be1ad: add rsp, 0x48
1408be1b1: pop rbx
1408be1b2: pop rdi
1408be1b3: pop rsi
1408be1b4: pop r12
1408be1b6: pop r13
1408be1b8: pop r14
1408be1ba: pop r15
1408be1bc: pop rbp
1408be1bd: ret 
1408be1be: add rsp, 0x48
1408be1c2: pop rbx
1408be1c3: pop rdi
1408be1c4: pop rsi
1408be1c5: pop r12
1408be1c7: pop r13
1408be1c9: pop r14
1408be1cb: pop r15
1408be1cd: pop rbp
1408be1ce: ret 
1408be1cf: add rsp, 0x48
1408be1d3: pop rbx
1408be1d4: pop rdi
1408be1d5: pop rsi
1408be1d6: pop r12
1408be1d8: pop r13
1408be1da: pop r14
1408be1dc: pop r15
1408be1de: pop rbp
1408be1df: ret 
1408be1e0: mov qword ptr [rsp + 0x10], rdx
1408be1e5: push rbp
1408be1e6: push r15
1408be1e8: push r14
1408be1ea: push r13
1408be1ec: push r12
1408be1ee: push rsi
1408be1ef: push rdi
1408be1f0: push rbx
1408be1f1: sub rsp, 0x48
1408be1f5: lea rbp, [rdx + 0x80]
1408be1fc: lea rcx, [rbp + 0x190]
1408be203: call 0x1405d0df0
1408be208: nop 
1408be209: add rsp, 0x48
1408be20d: pop rbx
1408be20e: pop rdi
1408be20f: pop rsi
1408be210: pop r12
1408be212: pop r13
1408be214: pop r14
1408be216: pop r15
1408be218: pop rbp
1408be219: ret 
1408be21a: nop word ptr [rax + rax]
1408be220: mov qword ptr [rsp + 0x10], rdx
1408be225: push rbp
1408be226: push r15
1408be228: push r14
1408be22a: push r13
1408be22c: push r12
1408be22e: push rsi
1408be22f: push rdi
1408be230: push rbx
1408be231: sub rsp, 0x48
1408be235: lea rbp, [rdx + 0x80]
1408be23c: lea rcx, [rbp + 0x100]
1408be243: call 0x14088b0c0
1408be248: mov byte ptr [rbp + 0x24f], 0
1408be24f: add rsp, 0x48
1408be253: pop rbx
1408be254: pop rdi
1408be255: pop rsi
1408be256: pop r12
1408be258: pop r13
1408be25a: pop r14
1408be25c: pop r15
1408be25e: pop rbp
1408be25f: ret 
1408be260: mov qword ptr [rsp + 0x10], rdx
1408be265: push rbp
1408be266: push r15
1408be268: push r14
1408be26a: push r13
1408be26c: push r12
1408be26e: push rsi
1408be26f: push rdi
1408be270: push rbx
1408be271: sub rsp, 0x48
1408be275: lea rbp, [rdx + 0x80]
1408be27c: lea rcx, [rbp + 0x210]
1408be283: call 0x1405d2870
1408be288: nop 
1408be289: add rsp, 0x48
1408be28d: pop rbx
1408be28e: pop rdi
1408be28f: pop rsi
1408be290: pop r12
1408be292: pop r13
1408be294: pop r14
1408be296: pop r15
1408be298: pop rbp
1408be299: ret 
1408be29a: nop word ptr [rax + rax]
1408be2a0: mov qword ptr [rsp + 0x10], rdx
1408be2a5: push rbp
1408be2a6: push r15
1408be2a8: push r14
1408be2aa: push r13
1408be2ac: push r12
1408be2ae: push rsi
1408be2af: push rdi
1408be2b0: push rbx
1408be2b1: sub rsp, 0x48
1408be2b5: lea rbp, [rdx + 0x80]
1408be2bc: lea rcx, [rbp + 0x190]
1408be2c3: call 0x1407b1810
1408be2c8: nop 
1408be2c9: add rsp, 0x48
1408be2cd: pop rbx
1408be2ce: pop rdi
1408be2cf: pop rsi
1408be2d0: pop r12
1408be2d2: pop r13
1408be2d4: pop r14
1408be2d6: pop r15
1408be2d8: pop rbp
1408be2d9: ret 
1408be2da: nop word ptr [rax + rax]
1408be2e0: mov qword ptr [rsp + 0x10], rdx
1408be2e5: push rbp
1408be2e6: push r15
1408be2e8: push r14
1408be2ea: push r13
1408be2ec: push r12
1408be2ee: push rsi
1408be2ef: push rdi
1408be2f0: push rbx
1408be2f1: sub rsp, 0x48
1408be2f5: lea rbp, [rdx + 0x80]
1408be2fc: lea rcx, [rbp + 0x190]
1408be303: call 0x1407b1810
1408be308: nop 
1408be309: add rsp, 0x48
1408be30d: pop rbx
1408be30e: pop rdi
1408be30f: pop rsi
1408be310: pop r12
1408be312: pop r13
1408be314: pop r14
1408be316: pop r15
1408be318: pop rbp
1408be319: ret 
1408be31a: nop word ptr [rax + rax]
1408be320: mov qword ptr [rsp + 0x10], rdx
1408be325: push rbp
1408be326: push r15
1408be328: push r14
1408be32a: push r13
1408be32c: push r12
1408be32e: push rsi
1408be32f: push rdi
1408be330: push rbx
1408be331: sub rsp, 0x48
1408be335: lea rbp, [rdx + 0x80]
1408be33c: lea rcx, [rbp + 0x190]
1408be343: call 0x1407b1810
1408be348: nop 
1408be349: add rsp, 0x48
1408be34d: pop rbx
1408be34e: pop rdi
1408be34f: pop rsi
1408be350: pop r12
1408be352: pop r13
1408be354: pop r14
1408be356: pop r15
1408be358: pop rbp
1408be359: ret 
