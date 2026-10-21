0x1408b0560: push rbp
0x1408b0561: push r15
0x1408b0563: push r14
0x1408b0565: push r13
0x1408b0567: push r12
0x1408b0569: push rsi
0x1408b056a: push rdi
0x1408b056b: push rbx
0x1408b056c: mov eax, 0x1058
0x1408b0571: call 0x14289aa40
0x1408b0576: sub rsp, rax
0x1408b0579: lea rbp, [rsp + 0x80]
0x1408b0581: movaps xmmword ptr [rbp + 0xfc0], xmm9
0x1408b0589: movdqa xmmword ptr [rbp + 0xfb0], xmm8
0x1408b0592: movdqa xmmword ptr [rbp + 0xfa0], xmm7
0x1408b059a: movdqa xmmword ptr [rbp + 0xf90], xmm6
0x1408b05a2: mov qword ptr [rbp + 0xf88], 0xfffffffffffffffe
0x1408b05ad: mov dil, 1
0x1408b05b0: cmp byte ptr [rip + 0x2d847d9], 0
0x1408b05b7: jne 0x1408b1485
0x1408b05bd: mov r14, rdx
0x1408b05c0: mov r15, r8
0x1408b05c3: mov rbx, rcx
0x1408b05c6: lea rsi, [rdx + 0x38]
0x1408b05ca: movabs rdx, 0xaca0d6d443d3ae2e
0x1408b05d4: movabs r8, 0x8d07dd0d83c93360
0x1408b05de: mov rcx, rsi
0x1408b05e1: call 0x1400dd6f0
0x1408b05e6: test rax, rax
0x1408b05e9: je 0x1408b150e
0x1408b05ef: cmp qword ptr [r14 + 0x218], rdx
0x1408b05f6: jbe 0x1408b150e
0x1408b05fc: mov rax, qword ptr [r14 + 0x210]
0x1408b0603: mov rcx, qword ptr [rax + rdx*8]
0x1408b0607: test rcx, rcx
0x1408b060a: je 0x1408b150e
0x1408b0610: mov rax, qword ptr [r14 + 0x1e0]
0x1408b0617: not rcx
0x1408b061a: lea rcx, [rcx + rcx*2]
0x1408b061e: shl rcx, 5
0x1408b0622: cmp qword ptr [rax + rcx + 0x20], 0
0x1408b0628: je 0x1408b150e
0x1408b062e: add rax, rcx
0x1408b0631: mov r12, qword ptr [rax + 0x10]
0x1408b0635: movabs rdx, 0xd38dae617fb79c7a
0x1408b063f: movabs r8, 0x22aadf5c5dad1eae
0x1408b0649: mov rcx, rsi
0x1408b064c: call 0x1400dd6f0
0x1408b0651: test rax, rax
0x1408b0654: je 0x1408b151a
0x1408b065a: cmp qword ptr [r14 + 0x218], rdx
0x1408b0661: jbe 0x1408b151a
0x1408b0667: mov rax, qword ptr [r14 + 0x210]
0x1408b066e: mov rcx, qword ptr [rax + rdx*8]
0x1408b0672: test rcx, rcx
0x1408b0675: je 0x1408b151a
0x1408b067b: mov rax, qword ptr [r14 + 0x1e0]
0x1408b0682: not rcx
0x1408b0685: lea rcx, [rcx + rcx*2]
0x1408b0689: shl rcx, 5
0x1408b068d: cmp qword ptr [rax + rcx + 0x20], 0
0x1408b0693: je 0x1408b151a
0x1408b0699: add rax, rcx
0x1408b069c: mov rax, qword ptr [rax + 0x10]
0x1408b06a0: cmp dword ptr [rax], 0xa
0x1408b06a3: jne 0x1408b1485
0x1408b06a9: cmp byte ptr [rax + 8], 1
0x1408b06ad: jne 0x1408b1485
0x1408b06b3: mov rdi, qword ptr [r12 + 8]
0x1408b06b8: test rdi, rdi
0x1408b06bb: je 0x1408b0711
0x1408b06bd: lea r13, [rdi + 0x10]
0x1408b06c1: mov cl, 1
0x1408b06c3: xor eax, eax
0x1408b06c5: lock cmpxchg byte ptr [rdi + 0x10], cl
0x1408b06ca: jne 0x1408b14b9
0x1408b06d0: mov rcx, r13
0x1408b06d3: call 0x140c81c70
0x1408b06d8: lea rax, [rdi + 0x18]
0x1408b06dc: xor ecx, ecx
0x1408b06de: cmp byte ptr [rdi + 0x4e], 7
0x1408b06e2: cmovne rcx, rax
0x1408b06e6: mov qword ptr [rbp + 0xed0], rcx
0x1408b06ed: jne 0x1408b0718
0x1408b06ef: mov rcx, r13
0x1408b06f2: call 0x140c81e10
0x1408b06f7: xor edi, edi
0x1408b06f9: mov al, 1
0x1408b06fb: lock cmpxchg byte ptr [r13], dil
0x1408b0701: je 0x1408b1485
0x1408b0707: xor edi, edi
0x1408b0709: mov rcx, r13
0x1408b070c: jmp 0x1408b14fb
0x1408b0711: xor edi, edi
0x1408b0713: jmp 0x1408b1485
0x1408b0718: mov qword ptr [rbp + 0xf28], r13
0x1408b071f: movabs rdx, 0x4a142e09642375b9
0x1408b0729: movabs r8, 0x43f6154cfa8c847f
0x1408b0733: mov rcx, rsi
0x1408b0736: call 0x1400dd6f0
0x1408b073b: test rax, rax
0x1408b073e: je 0x1408b1534
0x1408b0744: cmp qword ptr [r14 + 0x218], rdx
0x1408b074b: jbe 0x1408b1534
0x1408b0751: mov rax, qword ptr [r14 + 0x210]
0x1408b0758: mov rcx, qword ptr [rax + rdx*8]
0x1408b075c: test rcx, rcx
0x1408b075f: je 0x1408b1534
0x1408b0765: mov rax, qword ptr [r14 + 0x1e0]
0x1408b076c: not rcx
0x1408b076f: lea rcx, [rcx + rcx*2]
0x1408b0773: shl rcx, 5
0x1408b0777: cmp qword ptr [rax + rcx + 0x20], 0
0x1408b077d: je 0x1408b1534
0x1408b0783: add rax, rcx
0x1408b0786: mov rax, qword ptr [rax + 0x10]
0x1408b078a: mov qword ptr [rbp + 0xf70], rax
0x1408b0791: cmp qword ptr [r14 + 0x88], 0
0x1408b0799: movabs rdi, 0x8b297ec5ae15f15c
0x1408b07a3: movabs r13, 0x8597ee0727b461b3
0x1408b07ad: je 0x1408b084a
0x1408b07b3: mov rax, qword ptr [r14 + 0x70]
0x1408b07b7: mov rcx, qword ptr [r14 + 0x78]
0x1408b07bb: mov rdx, rcx
0x1408b07be: and rdx, rdi
0x1408b07c1: xor r8d, r8d
0x1408b07c4: movdqa xmm0, xmmword ptr [rip + 0x209fe24]
0x1408b07cc: pcmpeqd xmm1, xmm1
0x1408b07d0: movdqu xmm2, xmmword ptr [rax + rdx]
0x1408b07d5: movdqa xmm3, xmm2
0x1408b07d9: pcmpeqb xmm3, xmm0
0x1408b07dd: pmovmskb r9d, xmm3
0x1408b07e2: test r9d, r9d
0x1408b07e5: jne 0x1408b081d
0x1408b07e7: pcmpeqb xmm2, xmm1
0x1408b07eb: pmovmskb r9d, xmm2
0x1408b07f0: test r9d, r9d
0x1408b07f3: jne 0x1408b084a
0x1408b07f5: add rdx, r8
0x1408b07f8: add rdx, 0x10
0x1408b07fc: add r8, 0x10
0x1408b0800: and rdx, rcx
0x1408b0803: jmp 0x1408b07d0
0x1408b0805: nop word ptr cs:[rax + rax]
0x1408b0810: lea r10d, [r9 - 1]
0x1408b0814: and r10w, r9w
0x1408b0818: mov r9d, r10d
0x1408b081b: je 0x1408b07e7
0x1408b081d: tzcnt r10d, r9d
0x1408b0822: add r10, rdx
0x1408b0825: and r10, rcx
0x1408b0828: neg r10
0x1408b082b: lea r10, [r10 + r10*2]
0x1408b082f: cmp qword ptr [rax + r10*8 - 0x18], r13
0x1408b0834: jne 0x1408b0810
0x1408b0836: cmp qword ptr [rax + r10*8 - 0x10], rdi
0x1408b083b: jne 0x1408b0810
0x1408b083d: lea rax, [rax + r10*8]
0x1408b0841: mov rdi, qword ptr [rax - 8]
0x1408b0845: jmp 0x1408b0952
0x1408b084a: lea rcx, [r14 + 0x90]
0x1408b0851: mov eax, dword ptr [r14 + 0x90]
0x1408b0858: cmp eax, 0x3ffffffd
0x1408b085d: mov qword ptr [rbp + 0xf78], rcx
0x1408b0864: ja 0x1408b14cc
0x1408b086a: lea edx, [rax + 1]
0x1408b086d: lock cmpxchg dword ptr [rcx], edx
0x1408b0871: jne 0x1408b14cc
0x1408b0877: movzx eax, byte ptr [r14 + 0x98]
0x1408b087f: cmp qword ptr [r14 + 0xf0], 0
0x1408b0887: je 0x1408b1550
0x1408b088d: mov rax, qword ptr [r14 + 0xd8]
0x1408b0894: mov rcx, qword ptr [r14 + 0xe0]
0x1408b089b: mov rdx, rcx
0x1408b089e: and rdx, rdi
0x1408b08a1: xor r8d, r8d
0x1408b08a4: movdqa xmm0, xmmword ptr [rip + 0x209fd44]
0x1408b08ac: pcmpeqd xmm1, xmm1
0x1408b08b0: movdqu xmm2, xmmword ptr [rax + rdx]
0x1408b08b5: movdqa xmm3, xmm2
0x1408b08b9: pcmpeqb xmm3, xmm0
0x1408b08bd: pmovmskb r9d, xmm3
0x1408b08c2: test r9d, r9d
0x1408b08c5: jne 0x1408b08fd
0x1408b08c7: pcmpeqb xmm2, xmm1
0x1408b08cb: pmovmskb r9d, xmm2
0x1408b08d0: test r9d, r9d
0x1408b08d3: jne 0x1408b1550
0x1408b08d9: add rdx, r8
0x1408b08dc: add rdx, 0x10
0x1408b08e0: add r8, 0x10
0x1408b08e4: and rdx, rcx
0x1408b08e7: jmp 0x1408b08b0
0x1408b08e9: nop dword ptr [rax]
0x1408b08f0: lea r10d, [r9 - 1]
0x1408b08f4: and r10w, r9w
0x1408b08f8: mov r9d, r10d
0x1408b08fb: je 0x1408b08c7
0x1408b08fd: tzcnt r11d, r9d
0x1408b0902: add r11, rdx
0x1408b0905: and r11, rcx
0x1408b0908: mov r10, r11
0x1408b090b: neg r10
0x1408b090e: shl r11, 3
0x1408b0912: shl r10, 7
0x1408b0916: sub r10, r11
0x1408b0919: cmp qword ptr [rax + r10 - 0x88], r13
0x1408b0921: jne 0x1408b08f0
0x1408b0923: cmp qword ptr [rax + r10 - 0x80], rdi
0x1408b0928: jne 0x1408b08f0
0x1408b092a: add rax, r10
0x1408b092d: mov rdi, qword ptr [rax - 8]
0x1408b0931: mov edx, 0xffffffff
0x1408b0936: mov rcx, qword ptr [rbp + 0xf78]
0x1408b093d: lock xadd dword ptr [rcx], edx
0x1408b0941: dec edx
0x1408b0943: mov eax, edx
0x1408b0945: and eax, 0xbfffffff
0x1408b094a: neg eax
0x1408b094c: jo 0x1408b1504
0x1408b0952: cmp qword ptr [r14 + 0x218], rdi
0x1408b0959: jbe 0x1408b156d
0x1408b095f: mov rax, qword ptr [r14 + 0x210]
0x1408b0966: mov rcx, qword ptr [rax + rdi*8]
0x1408b096a: test rcx, rcx
0x1408b096d: je 0x1408b156d
0x1408b0973: mov rax, qword ptr [r14 + 0x1e0]
0x1408b097a: not rcx
0x1408b097d: lea rcx, [rcx + rcx*2]
0x1408b0981: shl rcx, 5
0x1408b0985: cmp qword ptr [rax + rcx + 0x20], 0
0x1408b098b: je 0x1408b156d
0x1408b0991: add rax, rcx
0x1408b0994: mov rcx, qword ptr [rax + 0x10]
0x1408b0998: lea rdx, [rip + 0x21ec473]
0x1408b099f: mov r8d, 0x15
0x1408b09a5: call 0x140937aa0
0x1408b09aa: mov qword ptr [rbp + 0xee0], rdx
0x1408b09b1: test al, 1
0x1408b09b3: je 0x1408b157b
0x1408b09b9: movabs rdx, 0xd387f1aea6230aea
0x1408b09c3: movabs r8, 0xd95b8da689a4c7a3
0x1408b09cd: mov rcx, rsi
0x1408b09d0: call 0x1400dd6f0
0x1408b09d5: test rax, rax
0x1408b09d8: je 0x1408b1542
0x1408b09de: cmp qword ptr [r14 + 0x218], rdx
0x1408b09e5: jbe 0x1408b1542
0x1408b09eb: mov rax, qword ptr [r14 + 0x210]
0x1408b09f2: mov rcx, qword ptr [rax + rdx*8]
0x1408b09f6: test rcx, rcx
0x1408b09f9: je 0x1408b1542
0x1408b09ff: mov rax, qword ptr [r14 + 0x1e0]
0x1408b0a06: not rcx
0x1408b0a09: lea rcx, [rcx + rcx*2]
0x1408b0a0d: shl rcx, 5
0x1408b0a11: cmp qword ptr [rax + rcx + 0x20], 0
0x1408b0a17: je 0x1408b1542
0x1408b0a1d: add rax, rcx
0x1408b0a20: mov rdi, qword ptr [rax + 0x10]
0x1408b0a24: mov rdx, qword ptr [r12]
0x1408b0a28: lea rsi, [rdx + 0x10]
0x1408b0a2c: mov cl, 1
0x1408b0a2e: xor eax, eax
0x1408b0a30: mov qword ptr [rbp + 0xf40], rdx
0x1408b0a37: lock cmpxchg byte ptr [rdx + 0x10], cl
0x1408b0a3c: jne 0x1408b14d6
0x1408b0a42: mov rcx, rsi
0x1408b0a45: call 0x140c81c70
0x1408b0a4a: mov qword ptr [rbp + 0xf30], rsi
0x1408b0a51: mov rsi, qword ptr [rbp + 0xf70]
0x1408b0a58: movdqu xmm0, xmmword ptr [rsi + 8]
0x1408b0a5d: movdqu xmmword ptr [rbp + 0xc70], xmm0
0x1408b0a65: lea rax, [rbp + 0xc70]
0x1408b0a6c: mov qword ptr [rbp + 0xd68], rax
0x1408b0a73: lea rax, [rip - 0x7a6bea]
0x1408b0a7a: mov qword ptr [rbp + 0xd70], rax
0x1408b0a81: lea rax, [rip + 0x2236220]
0x1408b0a88: mov qword ptr [rbp + 0x790], rax
0x1408b0a8f: mov qword ptr [rbp + 0x798], 1
0x1408b0a9a: lea r12, [rbp + 0xd68]
0x1408b0aa1: mov qword ptr [rbp + 0x7a0], r12
0x1408b0aa8: mov qword ptr [rbp + 0x7a8], 1
0x1408b0ab3: mov qword ptr [rbp + 0x7b0], 0
0x1408b0abe: lea rcx, [rbp - 0x38]
0x1408b0ac2: lea rdx, [rbp + 0x790]
0x1408b0ac9: call 0x140d79f90
0x1408b0ace: movd xmm6, dword ptr [rsi + 0x18]
0x1408b0ad3: mov rax, qword ptr [rbp - 0x38]
0x1408b0ad7: mov qword ptr [rbp + 0xf60], rax
0x1408b0ade: mov rax, qword ptr [rbp - 0x30]
0x1408b0ae2: mov qword ptr [rbp + 0xf50], rax
0x1408b0ae9: mov rsi, qword ptr [rbp - 0x28]
0x1408b0aed: mov rax, qword ptr [rdi + 8]
0x1408b0af1: mov qword ptr [rbp + 0xf78], rax
0x1408b0af8: mov rdi, qword ptr [rdi + 0x10]
0x1408b0afc: lea rcx, [r15 + 4]
0x1408b0b00: mov byte ptr [rbp + 0xf87], 1
0x1408b0b07: mov qword ptr [rbp + 0xee8], rcx
0x1408b0b0e: call 0x140bd5790
0x1408b0b13: movdqa xmm0, xmmword ptr [rip + 0x21ec115]
0x1408b0b1b: movdqu xmmword ptr [rbp + 0x790], xmm0
0x1408b0b23: mov dword ptr [rbp + 0x7a0], 0x3f800000
0x1408b0b2d: mov qword ptr [rbp + 0x7a8], r15
0x1408b0b34: mov dword ptr [rbp + 0xd68], 2
0x1408b0b3e: mov byte ptr [rbp + 0xf87], 1
0x1408b0b45: mov qword ptr [rsp + 0x40], rbx
0x1408b0b4a: mov qword ptr [rsp + 0x38], r14
0x1408b0b4f: mov qword ptr [rsp + 0x30], rax
0x1408b0b54: mov qword ptr [rsp + 0x28], r12
0x1408b0b59: mov qword ptr [rsp + 0x20], 1
0x1408b0b62: lea rdx, [rip + 0x21ec38f]
0x1408b0b69: lea rcx, [rbp - 0x38]
0x1408b0b6d: lea r9, [rbp + 0x790]
0x1408b0b74: mov r8d, 0xd
0x1408b0b7a: call 0x140867970
0x1408b0b7f: mov rcx, qword ptr [rbp + 0xf70]
0x1408b0b86: movzx eax, byte ptr [rcx + 0xc0]
0x1408b0b8d: shl eax, 6
0x1408b0b90: movdqu xmm0, xmmword ptr [rcx + rax + 8]
0x1408b0b96: movdqu xmmword ptr [rbp + 0xc70], xmm0
0x1408b0b9e: lea rax, [rbp + 0xc70]
0x1408b0ba5: mov qword ptr [rbp + 0xd68], rax
0x1408b0bac: lea rax, [rip - 0x7a6d23]
0x1408b0bb3: mov qword ptr [rbp + 0xd70], rax
0x1408b0bba: lea rax, [rip + 0x22360e7]
0x1408b0bc1: mov qword ptr [rbp + 0x790], rax
0x1408b0bc8: mov qword ptr [rbp + 0x798], 1
0x1408b0bd3: mov qword ptr [rbp + 0x7a0], r12
0x1408b0bda: mov qword ptr [rbp + 0x7a8], 1
0x1408b0be5: mov qword ptr [rbp + 0x7b0], 0
0x1408b0bf0: mov byte ptr [rbp + 0xf83], 1
0x1408b0bf7: mov byte ptr [rbp + 0xf82], 1
0x1408b0bfe: lea rcx, [rbp + 0xf08]
0x1408b0c05: lea rdx, [rbp + 0x790]
0x1408b0c0c: call 0x140d79f90
0x1408b0c11: mov rcx, qword ptr [rbp + 0xf40]
0x1408b0c18: lea rax, [rcx + 0x14]
0x1408b0c1c: mov ecx, dword ptr [rcx + 0x3c]
0x1408b0c1f: mov qword ptr [rbp + 0xd68], rax
0x1408b0c26: mov dword ptr [rbp + 0xd70], 0
0x1408b0c30: mov dword ptr [rbp + 0xd74], ecx
0x1408b0c36: mov byte ptr [rbp + 0xf86], 1
0x1408b0c3d: mov byte ptr [rbp + 0xf85], 1
0x1408b0c44: lea r8, [rip + 0x22017ed]
0x1408b0c4b: lea r12, [rbp + 0x790]
0x1408b0c52: lea r13, [rbp + 0xd68]
0x1408b0c59: mov rcx, r12
0x1408b0c5c: mov rdx, r13
0x1408b0c5f: call 0x141304fb0
0x1408b0c64: lea rax, [rdi + rdi*4]
0x1408b0c68: mov rdx, qword ptr [rbp + 0xf78]
0x1408b0c6f: lea r10, [rdx + rax*8]
0x1408b0c73: mov r8, qword ptr [rbp + 0x790]
0x1408b0c7a: mov r9, qword ptr [rbp + 0x798]
0x1408b0c81: mov rax, qword ptr [rbp + 0x7a0]
0x1408b0c88: lea r11, [r9 + rax*8]
0x1408b0c8c: pxor xmm0, xmm0
0x1408b0c90: movdqu xmmword ptr [rbp + 0xeb0], xmm0
0x1408b0c98: movdqu xmmword ptr [rbp + 0xec0], xmm0
0x1408b0ca0: mov rax, qword ptr [rbp + 0xf60]
0x1408b0ca7: mov qword ptr [rbp + 0xe60], rax
0x1408b0cae: mov rcx, qword ptr [rbp + 0xf50]
0x1408b0cb5: mov qword ptr [rbp + 0xe68], rcx
0x1408b0cbc: mov qword ptr [rbp + 0xe70], rsi
0x1408b0cc3: movd dword ptr [rbp + 0xe78], xmm6
0x1408b0ccb: mov qword ptr [rbp + 0xe80], rdx
0x1408b0cd2: mov qword ptr [rbp + 0xef0], r10
0x1408b0cd9: mov qword ptr [rbp + 0xe88], r10
0x1408b0ce0: mov qword ptr [rbp + 0xe90], r9
0x1408b0ce7: mov qword ptr [rbp + 0xef8], r8
0x1408b0cee: mov qword ptr [rbp + 0xea0], r8
0x1408b0cf5: mov qword ptr [rbp + 0xed8], r11
0x1408b0cfc: mov qword ptr [rbp + 0xea8], r11
0x1408b0d03: movd xmm8, dword ptr [rip + 0x2074bf8]
0x1408b0d0c: mov qword ptr [rbp + 0xf48], rax
0x1408b0d13: xor edi, edi
0x1408b0d15: mov qword ptr [rbp + 0xf00], r9
0x1408b0d1c: mov rcx, r9
0x1408b0d1f: nop 
0x1408b0d20: cmp rcx, qword ptr [rbp + 0xed8]
0x1408b0d27: je 0x1408b13b4
0x1408b0d2d: lea r15, [rcx + 8]
0x1408b0d31: mov qword ptr [rbp + 0xe98], r15
0x1408b0d38: movd xmm6, dword ptr [rcx]
0x1408b0d3c: movss xmm9, dword ptr [rcx + 4]
0x1408b0d42: movabs rbx, 0x8000000000000001
0x1408b0d4c: cmp rax, rbx
0x1408b0d4f: je 0x1408b0d7c
0x1408b0d51: movabs rcx, 0x8000000000000000
0x1408b0d5b: mov qword ptr [rbp + 0xe60], rcx
0x1408b0d62: mov rcx, rax
0x1408b0d65: neg rcx
0x1408b0d68: jno 0x1408b1366
0x1408b0d6e: mov qword ptr [rbp + 0xe60], rbx
0x1408b0d75: mov qword ptr [rbp + 0xf48], rbx
0x1408b0d7c: mov rax, qword ptr [rbp + 0xf78]
0x1408b0d83: test rax, rax
0x1408b0d86: je 0x1408b13b4
0x1408b0d8c: cmp rax, qword ptr [rbp + 0xef0]
0x1408b0d93: je 0x1408b13b4
0x1408b0d99: lea rsi, [rax + 0x28]
0x1408b0d9d: mov qword ptr [rbp + 0xe80], rsi
0x1408b0da4: movdqu xmm0, xmmword ptr [rax + 8]
0x1408b0da9: movdqu xmmword ptr [rbp + 0x698], xmm0
0x1408b0db1: lea rax, [rbp + 0x698]
0x1408b0db8: mov qword ptr [rbp + 0xc70], rax
0x1408b0dbf: lea rax, [rip - 0x7a6f36]
0x1408b0dc6: mov qword ptr [rbp + 0xc78], rax
0x1408b0dcd: lea rax, [rip + 0x2235ed4]
0x1408b0dd4: mov qword ptr [rbp + 0x790], rax
0x1408b0ddb: mov qword ptr [rbp + 0x798], 1
0x1408b0de6: lea rax, [rbp + 0xc70]
0x1408b0ded: mov qword ptr [rbp + 0x7a0], rax
0x1408b0df4: mov qword ptr [rbp + 0x7a8], 1
0x1408b0dff: mov qword ptr [rbp + 0x7b0], 0
0x1408b0e0a: mov rcx, r13
0x1408b0e0d: mov rdx, r12
0x1408b0e10: call 0x140d79f90
0x1408b0e15: mov rcx, qword ptr [rbp + 0xd68]
0x1408b0e1c: mov rax, rcx
0x1408b0e1f: neg rax
0x1408b0e22: jo 0x1408b13b4
0x1408b0e28: movdqa xmm7, xmm8
0x1408b0e2d: mov rax, rsi
0x1408b0e30: mov qword ptr [rbp + 0xf20], rbx
0x1408b0e37: mov rdx, qword ptr [rbp + 0xd70]
0x1408b0e3e: mov rsi, qword ptr [rbp + 0xd78]
0x1408b0e45: lea r14, [rdi + 1]
0x1408b0e49: mov qword ptr [rbp + 0xec8], r14
0x1408b0e50: cmp qword ptr [rbp + 0xf18], rsi
0x1408b0e57: mov qword ptr [rbp + 0xf78], rax
0x1408b0e5e: mov qword ptr [rbp + 0xf38], rcx
0x1408b0e65: mov qword ptr [rbp + 0xf70], rdx
0x1408b0e6c: jne 0x1408b0e90
0x1408b0e6e: mov rcx, qword ptr [rbp + 0xf10]
0x1408b0e75: mov r8, rsi
0x1408b0e78: call 0x142923bf0
0x1408b0e7d: test eax, eax
0x1408b0e7f: sete al
0x1408b0e82: mov dword ptr [rbp + 0xf6c], eax
0x1408b0e88: jmp 0x1408b0e9a
0x1408b0e8a: nop word ptr [rax + rax]
0x1408b0e90: mov dword ptr [rbp + 0xf6c], 0
0x1408b0e9a: call 0x140be6750
0x1408b0e9f: mov ebx, eax
0x1408b0ea1: xor ecx, ecx
0x1408b0ea3: call 0x1400d9b20
0x1408b0ea8: test rax, rax
0x1408b0eab: je 0x1408b1526
0x1408b0eb1: mov dword ptr [rbp + 0xe10], ebx
0x1408b0eb7: mov word ptr [rbp + 0xe18], 0
0x1408b0ec0: mov dword ptr [rbp + 0xe14], 0x3f800000
0x1408b0eca: mov qword ptr [rbp + 0xd68], 0
0x1408b0ed5: mov rcx, qword ptr [rbp + 0xee0]
0x1408b0edc: mov qword ptr [rbp + 0xe08], rcx
0x1408b0ee3: mov qword ptr [rbp + 0xd78], 0
0x1408b0eee: mov qword ptr [rbp + 0xd80], 4
0x1408b0ef9: mov qword ptr [rbp + 0xd88], 0
0x1408b0f04: movdqu xmm0, xmmword ptr [rax]
0x1408b0f08: inc qword ptr [rax]
0x1408b0f0b: movups xmm1, xmmword ptr [rip + 0x21e581e]
0x1408b0f12: lea rax, [rbp + 0xd70]
0x1408b0f19: movups xmmword ptr [rax + 0x78], xmm1
0x1408b0f1d: movdqu xmm1, xmmword ptr [rip + 0x21e57fb]
0x1408b0f25: movdqu xmmword ptr [rax + 0x68], xmm1
0x1408b0f2a: movdqu xmmword ptr [rbp + 0xdf8], xmm0
0x1408b0f32: movabs rax, 0x100000003
0x1408b0f3c: mov qword ptr [rbp + 0xda8], rax
0x1408b0f43: mov qword ptr [rbp + 0xd90], 0
0x1408b0f4e: mov qword ptr [rbp + 0xd98], 1
0x1408b0f59: mov qword ptr [rbp + 0xda0], 0
0x1408b0f64: mov rcx, r12
0x1408b0f67: mov rdx, r13
0x1408b0f6a: lea r8, [rbp - 0x38]
0x1408b0f6e: xor r9d, r9d
0x1408b0f71: call 0x140905be0
0x1408b0f76: mov rax, qword ptr [rbp + 0xed0]
0x1408b0f7d: mov qword ptr [rbp + 0xf58], rax
0x1408b0f84: mov dword ptr [rbp + 0xd68], 2
0x1408b0f8e: mov qword ptr [rsp + 0x20], r13
0x1408b0f93: lea rbx, [rbp + 0xf58]
0x1408b0f9a: mov rcx, rbx
0x1408b0f9d: mov rdx, r12
0x1408b0fa0: movabs r8, 0x11ba5697cc40b3dd
0x1408b0faa: mov r9b, 0x1d
0x1408b0fad: call 0x1408fd6c0
0x1408b0fb2: mov r8d, 0xf8
0x1408b0fb8: mov rcx, r13
0x1408b0fbb: mov rdx, r12
0x1408b0fbe: call 0x142923c00
0x1408b0fc3: movabs rax, 0xa647e80af4e9395f
0x1408b0fcd: mov qword ptr [rbp + 0xf58], rax
0x1408b0fd4: mov dword ptr [rbp + 0x790], 2
0x1408b0fde: mov qword ptr [rsp + 0x20], r12
0x1408b0fe3: mov rcx, rbx
0x1408b0fe6: mov rdx, r13
0x1408b0fe9: movabs r8, 0xc0b052b653ac9db6
0x1408b0ff3: mov r9b, 0x1d
0x1408b0ff6: call 0x140908610
0x1408b0ffb: mov r8d, 0xf8
0x1408b1001: mov rcx, r12
0x1408b1004: mov rdx, r13
0x1408b1007: call 0x142923c00
0x1408b100c: mov byte ptr [rbp + 0xf84], 1
0x1408b1013: mov rcx, qword ptr [rbp + 0xf70]
0x1408b101a: mov rdx, rsi
0x1408b101d: call 0x140c9cd80
0x1408b1022: mov qword ptr [rbp + 0xf58], rax
0x1408b1029: mov dword ptr [rbp + 0xd68], 2
0x1408b1033: mov qword ptr [rsp + 0x20], r13
0x1408b1038: lea rcx, [rbp + 0xf58]
0x1408b103f: mov rdx, r12
0x1408b1042: movabs r8, 0x422b14c27be4803
0x1408b104c: mov r9b, 0x1d
0x1408b104f: call 0x140908610
0x1408b1054: mov r8d, 0xf8
0x1408b105a: mov rcx, r13
0x1408b105d: mov rdx, r12
0x1408b1060: call 0x142923c00
0x1408b1065: mov byte ptr [rbp + 0xf81], 1
0x1408b106c: mov rcx, qword ptr [rbp + 0xee8]
0x1408b1073: call 0x140bd5790
0x1408b1078: mov byte ptr [rbp + 0xf81], 0
0x1408b107f: lea rcx, [rbp + 0xc70]
0x1408b1086: mov rdx, r13
0x1408b1089: movabs r8, 0x531bb9318eea4971
0x1408b1093: mov r9, rax
0x1408b1096: call 0x141629d20
0x1408b109b: cmp byte ptr [rbp + 0xd4e], 8
0x1408b10a2: je 0x1408b10cd
0x1408b10a4: mov rdx, qword ptr [rbp + 0xd58]
0x1408b10ab: mov eax, 0xe8
0x1408b10b0: add rdx, rax
0x1408b10b3: lea rcx, [rbp + 0xd28]
0x1408b10ba: movabs r8, 0x9c91ae27fdb736fb
0x1408b10c4: movdqa xmm3, xmm7
0x1408b10c8: call 0x14096ba30
0x1408b10cd: mov r8d, 0xf8
0x1408b10d3: lea rcx, [rbp + 0x698]
0x1408b10da: lea rdx, [rbp + 0xc70]
0x1408b10e1: call 0x142923c00
0x1408b10e6: cmp byte ptr [rbp + 0x776], 8
0x1408b10ed: je 0x1408b111f
0x1408b10ef: mov rdx, qword ptr [rbp + 0x780]
0x1408b10f6: mov eax, 0xe8
0x1408b10fb: add rdx, rax
0x1408b10fe: movss dword ptr [rsp + 0x20], xmm9
0x1408b1105: lea rcx, [rbp + 0x750]
0x1408b110c: movabs r8, 0x1d06ec44e216b191
0x1408b1116: movdqa xmm3, xmm6
0x1408b111a: call 0x14096b940
0x1408b111f: mov r8d, 0xf8
0x1408b1125: lea rcx, [rbp + 0x3b0]
0x1408b112c: lea rdx, [rbp + 0x698]
0x1408b1133: call 0x142923c00
0x1408b1138: cmp byte ptr [rbp + 0x48e], 8
0x1408b113f: je 0x1408b117d
0x1408b1141: mov rax, qword ptr [rbp + 0xf40]
0x1408b1148: movd xmm3, dword ptr [rax + 0x24]
0x1408b114d: movd xmm0, dword ptr [rax + 0x28]
0x1408b1152: mov rdx, qword ptr [rbp + 0x498]
0x1408b1159: mov eax, 0xe8
0x1408b115e: add rdx, rax
0x1408b1161: movd dword ptr [rsp + 0x20], xmm0
0x1408b1167: lea rcx, [rbp + 0x468]
0x1408b116e: movabs r8, 0x7038b90f28755919
0x1408b1178: call 0x14096b940
0x1408b117d: mov r8d, 0xf8
0x1408b1183: lea rcx, [rbp + 0x4a8]
0x1408b118a: lea rdx, [rbp + 0x3b0]
0x1408b1191: call 0x142923c00
0x1408b1196: cmp byte ptr [rbp + 0x586], 8
0x1408b119d: je 0x1408b11d8
0x1408b119f: mov rcx, qword ptr [rbp + 0xf40]
0x1408b11a6: mov eax, dword ptr [rcx + 0x14]
0x1408b11a9: cmp dword ptr [rcx + 0x18], edi
0x1408b11ac: sete r9b
0x1408b11b0: and r9b, al
0x1408b11b3: mov rdx, qword ptr [rbp + 0x590]
0x1408b11ba: mov eax, 0xe8
0x1408b11bf: add rdx, rax
0x1408b11c2: lea rcx, [rbp + 0x560]
0x1408b11c9: movabs r8, 0xfed4ed8f674ddb11
0x1408b11d3: call 0x14096b850
0x1408b11d8: mov r8d, 0xf8
0x1408b11de: lea rcx, [rbp + 0x5a0]
0x1408b11e5: lea rdx, [rbp + 0x4a8]
0x1408b11ec: call 0x142923c00
0x1408b11f1: cmp byte ptr [rbp + 0x67e], 8
0x1408b11f8: je 0x1408b1226
0x1408b11fa: mov rdx, qword ptr [rbp + 0x688]
0x1408b1201: mov eax, 0xe8
0x1408b1206: add rdx, rax
0x1408b1209: lea rcx, [rbp + 0x658]
0x1408b1210: movabs r8, 0xfe45681f9b784bd8
0x1408b121a: mov r9d, dword ptr [rbp + 0xf6c]
0x1408b1221: call 0x14096b850
0x1408b1226: mov r8d, 0xf8
0x1408b122c: lea rcx, [rbp + 0xb78]
0x1408b1233: lea rdx, [rbp + 0x5a0]
0x1408b123a: call 0x142923c00
0x1408b123f: cmp byte ptr [rbp + 0xc56], 8
0x1408b1246: je 0x1408b1274
0x1408b1248: test rdi, rdi
0x1408b124b: sete r9b
0x1408b124f: mov rdx, qword ptr [rbp + 0xc60]
0x1408b1256: mov eax, 0xe8
0x1408b125b: add rdx, rax
0x1408b125e: lea rcx, [rbp + 0xc30]
0x1408b1265: movabs r8, 0xfa9824c7f6ddb734
0x1408b126f: call 0x14096b850
0x1408b1274: mov rsi, qword ptr [rbp + 0xb78]
0x1408b127b: mov r8d, 0xa0
0x1408b1281: lea rdi, [rbp + 0x798]
0x1408b1288: mov rcx, rdi
0x1408b128b: lea rdx, [rbp + 0xb80]
0x1408b1292: call 0x142923c00
0x1408b1297: movzx eax, byte ptr [rbp + 0xc20]
0x1408b129e: movzx ecx, word ptr [rbp + 0xc21]
0x1408b12a5: movzx edx, byte ptr [rbp + 0xc23]
0x1408b12ac: lea r8, [rbp + 0xc30]
0x1408b12b3: movups xmm0, xmmword ptr [r8 - 0xc]
0x1408b12b8: movdqu xmm1, xmmword ptr [r8 + 4]
0x1408b12be: movdqu xmm2, xmmword ptr [r8 + 0x14]
0x1408b12c4: movdqu xmm3, xmmword ptr [r8 + 0x24]
0x1408b12ca: movups xmmword ptr [rdi + 0xa4], xmm0
0x1408b12d1: movdqu xmmword ptr [rdi + 0xb4], xmm1
0x1408b12d9: movdqu xmmword ptr [rdi + 0xc4], xmm2
0x1408b12e1: movdqu xmmword ptr [rdi + 0xd4], xmm3
0x1408b12e9: movdqu xmm0, xmmword ptr [r8 + 0x30]
0x1408b12ef: movdqu xmmword ptr [rdi + 0xe0], xmm0
0x1408b12f7: and al, 0x7f
0x1408b12f9: and dl, 0x9f
0x1408b12fc: or dl, 0x40
0x1408b12ff: mov qword ptr [rbp + 0x790], rsi
0x1408b1306: mov byte ptr [rbp + 0x838], al
0x1408b130c: mov word ptr [rbp + 0x839], cx
0x1408b1313: mov byte ptr [rbp + 0x83b], dl
0x1408b1319: mov edx, 6
0x1408b131e: mov rcx, r12
0x1408b1321: call 0x140906a90
0x1408b1326: mov rdi, r14
0x1408b1329: mov rsi, qword ptr [rbp + 0xf20]
0x1408b1330: mov rax, rsi
0x1408b1333: mov rcx, r15
0x1408b1336: mov rdx, qword ptr [rbp + 0xf38]
0x1408b133d: test rdx, rdx
0x1408b1340: je 0x1408b0d20
0x1408b1346: mov r8d, 1
0x1408b134c: mov rcx, qword ptr [rbp + 0xf70]
0x1408b1353: call 0x140613c20
0x1408b1358: mov rdi, r14
0x1408b135b: mov rax, rsi
0x1408b135e: mov rcx, r15
0x1408b1361: jmp 0x1408b0d20
0x1408b1366: lea rdx, [rbp + 0xe68]
0x1408b136d: mov rcx, qword ptr [rdx + 0x10]
0x1408b1371: lea r8, [rbp + 0xd70]
0x1408b1378: mov qword ptr [r8 + 0x10], rcx
0x1408b137c: movdqu xmm0, xmmword ptr [rdx]
0x1408b1380: movdqu xmmword ptr [r8], xmm0
0x1408b1385: movd xmm7, dword ptr [rbp + 0xd80]
0x1408b138d: movabs rcx, 0x8000000000000000
0x1408b1397: mov qword ptr [rbp + 0xf48], rcx
0x1408b139e: mov qword ptr [rbp + 0xf20], rcx
0x1408b13a5: mov rcx, rax
0x1408b13a8: mov rax, qword ptr [rbp + 0xf78]
0x1408b13af: jmp 0x1408b0e37
0x1408b13b4: mov rdx, qword ptr [rbp + 0xef8]
0x1408b13bb: test rdx, rdx
0x1408b13be: je 0x1408b13d6
0x1408b13c0: shl rdx, 3
0x1408b13c4: mov r8d, 4
0x1408b13ca: mov rcx, qword ptr [rbp + 0xf00]
0x1408b13d1: call 0x140613c20
0x1408b13d6: movabs rax, 0x8000000000000002
0x1408b13e0: mov rdx, qword ptr [rbp + 0xf48]
0x1408b13e7: cmp rdx, rax
0x1408b13ea: jl 0x1408b1403
0x1408b13ec: test rdx, rdx
0x1408b13ef: je 0x1408b1403
0x1408b13f1: mov r8d, 1
0x1408b13f7: mov rcx, qword ptr [rbp + 0xf50]
0x1408b13fe: call 0x140613c20
0x1408b1403: lea rsi, [rbp + 0x790]
0x1408b140a: lea rdx, [rbp - 0x38]
0x1408b140e: mov r8d, 0x3e8
0x1408b1414: mov rcx, rsi
0x1408b1417: call 0x142923c00
0x1408b141c: mov byte ptr [rbp + 0xf86], 0
0x1408b1423: mov byte ptr [rbp + 0xf85], 0
0x1408b142a: mov rcx, rsi
0x1408b142d: call 0x140868370
0x1408b1432: mov rdx, qword ptr [rbp + 0xf08]
0x1408b1439: test rdx, rdx
0x1408b143c: je 0x1408b1450
0x1408b143e: mov rcx, qword ptr [rbp + 0xf10]
0x1408b1445: mov r8d, 1
0x1408b144b: call 0x140613c20
0x1408b1450: mov rsi, qword ptr [rbp + 0xf30]
0x1408b1457: mov rcx, rsi
0x1408b145a: call 0x140c81e10
0x1408b145f: xor ecx, ecx
0x1408b1461: mov dil, 1
0x1408b1464: mov al, 1
0x1408b1466: lock cmpxchg byte ptr [rsi], cl
0x1408b146a: jne 0x1408b14e9
0x1408b146c: mov rsi, qword ptr [rbp + 0xf28]
0x1408b1473: mov rcx, rsi
0x1408b1476: call 0x140c81e10
0x1408b147b: xor ecx, ecx
0x1408b147d: mov al, 1
0x1408b147f: lock cmpxchg byte ptr [rsi], cl
0x1408b1483: jne 0x1408b14f8
0x1408b1485: mov eax, edi
0x1408b1487: movaps xmm6, xmmword ptr [rbp + 0xf90]
0x1408b148e: movaps xmm7, xmmword ptr [rbp + 0xfa0]
0x1408b1495: movaps xmm8, xmmword ptr [rbp + 0xfb0]
0x1408b149d: movaps xmm9, xmmword ptr [rbp + 0xfc0]
0x1408b14a5: add rsp, 0x1058
0x1408b14ac: pop rbx
0x1408b14ad: pop rdi
0x1408b14ae: pop rsi
0x1408b14af: pop r12
0x1408b14b1: pop r13
0x1408b14b3: pop r14
0x1408b14b5: pop r15
0x1408b14b7: pop rbp
0x1408b14b8: ret 
0x1408b14b9: mov rcx, r13
0x1408b14bc: mov r8d, 0x3b9aca00
0x1408b14c2: call 0x1428cd210
0x1408b14c7: jmp 0x1408b06d0
0x1408b14cc: call 0x1428d8a30
0x1408b14d1: jmp 0x1408b0877
0x1408b14d6: mov rcx, rsi
0x1408b14d9: mov r8d, 0x3b9aca00
0x1408b14df: call 0x1428cd210
0x1408b14e4: jmp 0x1408b0a42
0x1408b14e9: mov rcx, rsi
0x1408b14ec: xor edx, edx
0x1408b14ee: call 0x1428cdb90
0x1408b14f3: jmp 0x1408b146c
0x1408b14f8: mov rcx, rsi
0x1408b14fb: xor edx, edx
0x1408b14fd: call 0x1428cdb90
0x1408b1502: jmp 0x1408b1485
0x1408b1504: call 0x1428d8d40
0x1408b1509: jmp 0x1408b0952
0x1408b150e: lea rcx, [rip + 0x21eb953]
0x1408b1515: call 0x1428d9310
0x1408b151a: lea rcx, [rip + 0x21eb95f]
0x1408b1521: call 0x1428d9310
0x1408b1526: lea rcx, [rip + 0x2389573]
0x1408b152d: call 0x1428d80d0
0x1408b1532: jmp 0x1408b1587
0x1408b1534: lea rcx, [rip + 0x21eb95d]
0x1408b153b: call 0x1428d9310
0x1408b1540: jmp 0x1408b1587
0x1408b1542: lea rcx, [rip + 0x21eb997]
0x1408b1549: call 0x1428d9310
0x1408b154e: jmp 0x1408b1587
0x1408b1550: mov edx, 0xffffffff
0x1408b1555: mov rcx, qword ptr [rbp + 0xf78]
0x1408b155c: lock xadd dword ptr [rcx], edx
0x1408b1560: dec edx
0x1408b1562: mov eax, edx
0x1408b1564: and eax, 0xbfffffff
0x1408b1569: neg eax
0x1408b156b: jo 0x1408b1589
0x1408b156d: lea rcx, [rip + 0x21eb93c]
0x1408b1574: call 0x1428d9310
0x1408b1579: jmp 0x1408b1587
0x1408b157b: lea rcx, [rip + 0x21eb946]
0x1408b1582: call 0x1428d9310
0x1408b1587: ud2 
0x1408b1589: call 0x1428d8d40
0x1408b158e: jmp 0x1408b156d
0x1408b1590: mov qword ptr [rsp + 0x10], rdx
0x1408b1595: push rbp
0x1408b1596: push r15
0x1408b1598: push r14
0x1408b159a: push r13
0x1408b159c: push r12
0x1408b159e: push rsi
0x1408b159f: push rdi
0x1408b15a0: push rbx
0x1408b15a1: sub rsp, 0x88
0x1408b15a8: lea rbp, [rdx + 0x80]
0x1408b15af: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b15b5: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b15bc: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b15c2: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b15c8: movzx ebx, byte ptr [rbp + 0xf85]
0x1408b15cf: movzx esi, byte ptr [rbp + 0xf86]
0x1408b15d6: mov rdx, qword ptr [rbp + 0xf08]
0x1408b15dd: test rdx, rdx
0x1408b15e0: je 0x1408b15f4
0x1408b15e2: mov rcx, qword ptr [rbp + 0xf10]
0x1408b15e9: mov r8d, 1
0x1408b15ef: call 0x140613c20
0x1408b15f4: and sil, 1
0x1408b15f8: mov byte ptr [rbp + 0xf83], sil
0x1408b15ff: and bl, 1
0x1408b1602: mov byte ptr [rbp + 0xf82], bl
0x1408b1608: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b160e: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1614: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b161b: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1621: add rsp, 0x88
0x1408b1628: pop rbx
0x1408b1629: pop rdi
0x1408b162a: pop rsi
0x1408b162b: pop r12
0x1408b162d: pop r13
0x1408b162f: pop r14
0x1408b1631: pop r15
0x1408b1633: pop rbp
0x1408b1634: ret 
0x1408b1635: nop word ptr cs:[rax + rax]
0x1408b1640: mov qword ptr [rsp + 0x10], rdx
0x1408b1645: push rbp
0x1408b1646: push r15
0x1408b1648: push r14
0x1408b164a: push r13
0x1408b164c: push r12
0x1408b164e: push rsi
0x1408b164f: push rdi
0x1408b1650: push rbx
0x1408b1651: sub rsp, 0x88
0x1408b1658: lea rbp, [rdx + 0x80]
0x1408b165f: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1665: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b166c: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1672: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1678: lea rcx, [rbp + 0xe60]
0x1408b167f: call 0x140889330
0x1408b1684: mov byte ptr [rbp + 0xf86], 1
0x1408b168b: mov byte ptr [rbp + 0xf85], 0
0x1408b1692: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1698: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b169e: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b16a5: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b16ab: add rsp, 0x88
0x1408b16b2: pop rbx
0x1408b16b3: pop rdi
0x1408b16b4: pop rsi
0x1408b16b5: pop r12
0x1408b16b7: pop r13
0x1408b16b9: pop r14
0x1408b16bb: pop r15
0x1408b16bd: pop rbp
0x1408b16be: ret 
0x1408b16bf: nop 
0x1408b16c0: mov qword ptr [rsp + 0x10], rdx
0x1408b16c5: push rbp
0x1408b16c6: push r15
0x1408b16c8: push r14
0x1408b16ca: push r13
0x1408b16cc: push r12
0x1408b16ce: push rsi
0x1408b16cf: push rdi
0x1408b16d0: push rbx
0x1408b16d1: sub rsp, 0x88
0x1408b16d8: lea rbp, [rdx + 0x80]
0x1408b16df: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b16e5: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b16ec: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b16f2: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b16f8: cmp qword ptr [rbp + 0xf38], 0
0x1408b1700: je 0x1408b171b
0x1408b1702: mov r8d, 1
0x1408b1708: mov rcx, qword ptr [rbp + 0xf70]
0x1408b170f: mov rdx, qword ptr [rbp + 0xf38]
0x1408b1716: call 0x140613c20
0x1408b171b: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1721: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1727: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b172e: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1734: add rsp, 0x88
0x1408b173b: pop rbx
0x1408b173c: pop rdi
0x1408b173d: pop rsi
0x1408b173e: pop r12
0x1408b1740: pop r13
0x1408b1742: pop r14
0x1408b1744: pop r15
0x1408b1746: pop rbp
0x1408b1747: ret 
0x1408b1748: nop dword ptr [rax + rax]
0x1408b1750: mov qword ptr [rsp + 0x10], rdx
0x1408b1755: push rbp
0x1408b1756: push r15
0x1408b1758: push r14
0x1408b175a: push r13
0x1408b175c: push r12
0x1408b175e: push rsi
0x1408b175f: push rdi
0x1408b1760: push rbx
0x1408b1761: sub rsp, 0x88
0x1408b1768: lea rbp, [rdx + 0x80]
0x1408b176f: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1775: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b177c: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1782: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1788: lea rcx, [rbp + 0xd68]
0x1408b178f: call 0x1408935c0
0x1408b1794: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b179a: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b17a0: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b17a7: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b17ad: add rsp, 0x88
0x1408b17b4: pop rbx
0x1408b17b5: pop rdi
0x1408b17b6: pop rsi
0x1408b17b7: pop r12
0x1408b17b9: pop r13
0x1408b17bb: pop r14
0x1408b17bd: pop r15
0x1408b17bf: pop rbp
0x1408b17c0: ret 
0x1408b17c1: nop word ptr cs:[rax + rax]
0x1408b17d0: mov qword ptr [rsp + 0x10], rdx
0x1408b17d5: push rbp
0x1408b17d6: push r15
0x1408b17d8: push r14
0x1408b17da: push r13
0x1408b17dc: push r12
0x1408b17de: push rsi
0x1408b17df: push rdi
0x1408b17e0: push rbx
0x1408b17e1: sub rsp, 0x88
0x1408b17e8: lea rbp, [rdx + 0x80]
0x1408b17ef: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b17f5: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b17fc: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1802: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1808: mov byte ptr [rbp + 0xf84], 0
0x1408b180f: lea rcx, [rbp + 0x790]
0x1408b1816: call 0x1408935c0
0x1408b181b: mov byte ptr [rbp + 0xf84], 0
0x1408b1822: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1828: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b182e: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1835: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b183b: add rsp, 0x88
0x1408b1842: pop rbx
0x1408b1843: pop rdi
0x1408b1844: pop rsi
0x1408b1845: pop r12
0x1408b1847: pop r13
0x1408b1849: pop r14
0x1408b184b: pop r15
0x1408b184d: pop rbp
0x1408b184e: ret 
0x1408b184f: nop 
0x1408b1850: mov qword ptr [rsp + 0x10], rdx
0x1408b1855: push rbp
0x1408b1856: push r15
0x1408b1858: push r14
0x1408b185a: push r13
0x1408b185c: push r12
0x1408b185e: push rsi
0x1408b185f: push rdi
0x1408b1860: push rbx
0x1408b1861: sub rsp, 0x88
0x1408b1868: lea rbp, [rdx + 0x80]
0x1408b186f: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1875: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b187c: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1882: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1888: lea rcx, [rbp + 0xc70]
0x1408b188f: call 0x1408935c0
0x1408b1894: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b189a: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b18a0: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b18a7: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b18ad: add rsp, 0x88
0x1408b18b4: pop rbx
0x1408b18b5: pop rdi
0x1408b18b6: pop rsi
0x1408b18b7: pop r12
0x1408b18b9: pop r13
0x1408b18bb: pop r14
0x1408b18bd: pop r15
0x1408b18bf: pop rbp
0x1408b18c0: ret 
0x1408b18c1: nop word ptr cs:[rax + rax]
0x1408b18d0: mov qword ptr [rsp + 0x10], rdx
0x1408b18d5: push rbp
0x1408b18d6: push r15
0x1408b18d8: push r14
0x1408b18da: push r13
0x1408b18dc: push r12
0x1408b18de: push rsi
0x1408b18df: push rdi
0x1408b18e0: push rbx
0x1408b18e1: sub rsp, 0x88
0x1408b18e8: lea rbp, [rdx + 0x80]
0x1408b18ef: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b18f5: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b18fc: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1902: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1908: lea rcx, [rbp + 0x698]
0x1408b190f: call 0x1408935c0
0x1408b1914: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b191a: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1920: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1927: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b192d: add rsp, 0x88
0x1408b1934: pop rbx
0x1408b1935: pop rdi
0x1408b1936: pop rsi
0x1408b1937: pop r12
0x1408b1939: pop r13
0x1408b193b: pop r14
0x1408b193d: pop r15
0x1408b193f: pop rbp
0x1408b1940: ret 
0x1408b1941: nop word ptr cs:[rax + rax]
0x1408b1950: mov qword ptr [rsp + 0x10], rdx
0x1408b1955: push rbp
0x1408b1956: push r15
0x1408b1958: push r14
0x1408b195a: push r13
0x1408b195c: push r12
0x1408b195e: push rsi
0x1408b195f: push rdi
0x1408b1960: push rbx
0x1408b1961: sub rsp, 0x88
0x1408b1968: lea rbp, [rdx + 0x80]
0x1408b196f: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1975: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b197c: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1982: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1988: lea rcx, [rbp + 0x3b0]
0x1408b198f: call 0x1408935c0
0x1408b1994: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b199a: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b19a0: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b19a7: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b19ad: add rsp, 0x88
0x1408b19b4: pop rbx
0x1408b19b5: pop rdi
0x1408b19b6: pop rsi
0x1408b19b7: pop r12
0x1408b19b9: pop r13
0x1408b19bb: pop r14
0x1408b19bd: pop r15
0x1408b19bf: pop rbp
0x1408b19c0: ret 
0x1408b19c1: nop word ptr cs:[rax + rax]
0x1408b19d0: mov qword ptr [rsp + 0x10], rdx
0x1408b19d5: push rbp
0x1408b19d6: push r15
0x1408b19d8: push r14
0x1408b19da: push r13
0x1408b19dc: push r12
0x1408b19de: push rsi
0x1408b19df: push rdi
0x1408b19e0: push rbx
0x1408b19e1: sub rsp, 0x88
0x1408b19e8: lea rbp, [rdx + 0x80]
0x1408b19ef: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b19f5: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b19fc: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1a02: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1a08: lea rcx, [rbp + 0x4a8]
0x1408b1a0f: call 0x1408935c0
0x1408b1a14: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1a1a: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1a20: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1a27: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1a2d: add rsp, 0x88
0x1408b1a34: pop rbx
0x1408b1a35: pop rdi
0x1408b1a36: pop rsi
0x1408b1a37: pop r12
0x1408b1a39: pop r13
0x1408b1a3b: pop r14
0x1408b1a3d: pop r15
0x1408b1a3f: pop rbp
0x1408b1a40: ret 
0x1408b1a41: nop word ptr cs:[rax + rax]
0x1408b1a50: mov qword ptr [rsp + 0x10], rdx
0x1408b1a55: push rbp
0x1408b1a56: push r15
0x1408b1a58: push r14
0x1408b1a5a: push r13
0x1408b1a5c: push r12
0x1408b1a5e: push rsi
0x1408b1a5f: push rdi
0x1408b1a60: push rbx
0x1408b1a61: sub rsp, 0x88
0x1408b1a68: lea rbp, [rdx + 0x80]
0x1408b1a6f: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1a75: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b1a7c: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1a82: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1a88: lea rcx, [rbp + 0x5a0]
0x1408b1a8f: call 0x1408935c0
0x1408b1a94: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1a9a: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1aa0: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1aa7: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1aad: add rsp, 0x88
0x1408b1ab4: pop rbx
0x1408b1ab5: pop rdi
0x1408b1ab6: pop rsi
0x1408b1ab7: pop r12
0x1408b1ab9: pop r13
0x1408b1abb: pop r14
0x1408b1abd: pop r15
0x1408b1abf: pop rbp
0x1408b1ac0: ret 
0x1408b1ac1: nop word ptr cs:[rax + rax]
0x1408b1ad0: mov qword ptr [rsp + 0x10], rdx
0x1408b1ad5: push rbp
0x1408b1ad6: push r15
0x1408b1ad8: push r14
0x1408b1ada: push r13
0x1408b1adc: push r12
0x1408b1ade: push rsi
0x1408b1adf: push rdi
0x1408b1ae0: push rbx
0x1408b1ae1: sub rsp, 0x88
0x1408b1ae8: lea rbp, [rdx + 0x80]
0x1408b1aef: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1af5: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b1afc: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1b02: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1b08: lea rcx, [rbp + 0xb78]
0x1408b1b0f: call 0x1408935c0
0x1408b1b14: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1b1a: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1b20: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1b27: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1b2d: add rsp, 0x88
0x1408b1b34: pop rbx
0x1408b1b35: pop rdi
0x1408b1b36: pop rsi
0x1408b1b37: pop r12
0x1408b1b39: pop r13
0x1408b1b3b: pop r14
0x1408b1b3d: pop r15
0x1408b1b3f: pop rbp
0x1408b1b40: ret 
0x1408b1b41: nop word ptr cs:[rax + rax]
0x1408b1b50: mov qword ptr [rsp + 0x10], rdx
0x1408b1b55: push rbp
0x1408b1b56: push r15
0x1408b1b58: push r14
0x1408b1b5a: push r13
0x1408b1b5c: push r12
0x1408b1b5e: push rsi
0x1408b1b5f: push rdi
0x1408b1b60: push rbx
0x1408b1b61: sub rsp, 0x88
0x1408b1b68: lea rbp, [rdx + 0x80]
0x1408b1b6f: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1b75: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b1b7c: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1b82: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1b88: cmp byte ptr [rbp + 0xf81], 0
0x1408b1b8f: je 0x1408b1bca
0x1408b1b91: lea rcx, [rbp + 0xd68]
0x1408b1b98: call 0x1408935c0
0x1408b1b9d: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1ba3: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1ba9: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1bb0: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1bb6: add rsp, 0x88
0x1408b1bbd: pop rbx
0x1408b1bbe: pop rdi
0x1408b1bbf: pop rsi
0x1408b1bc0: pop r12
0x1408b1bc2: pop r13
0x1408b1bc4: pop r14
0x1408b1bc6: pop r15
0x1408b1bc8: pop rbp
0x1408b1bc9: ret 
0x1408b1bca: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1bd0: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1bd6: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1bdd: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1be3: add rsp, 0x88
0x1408b1bea: pop rbx
0x1408b1beb: pop rdi
0x1408b1bec: pop rsi
0x1408b1bed: pop r12
0x1408b1bef: pop r13
0x1408b1bf1: pop r14
0x1408b1bf3: pop r15
0x1408b1bf5: pop rbp
0x1408b1bf6: ret 
0x1408b1bf7: nop word ptr [rax + rax]
0x1408b1c00: mov qword ptr [rsp + 0x10], rdx
0x1408b1c05: push rbp
0x1408b1c06: push r15
0x1408b1c08: push r14
0x1408b1c0a: push r13
0x1408b1c0c: push r12
0x1408b1c0e: push rsi
0x1408b1c0f: push rdi
0x1408b1c10: push rbx
0x1408b1c11: sub rsp, 0x88
0x1408b1c18: lea rbp, [rdx + 0x80]
0x1408b1c1f: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1c25: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b1c2c: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1c32: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1c38: cmp byte ptr [rbp + 0xf84], 0
0x1408b1c3f: je 0x1408b1c7a
0x1408b1c41: lea rcx, [rbp + 0x790]
0x1408b1c48: call 0x1408935c0
0x1408b1c4d: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1c53: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1c59: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1c60: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1c66: add rsp, 0x88
0x1408b1c6d: pop rbx
0x1408b1c6e: pop rdi
0x1408b1c6f: pop rsi
0x1408b1c70: pop r12
0x1408b1c72: pop r13
0x1408b1c74: pop r14
0x1408b1c76: pop r15
0x1408b1c78: pop rbp
0x1408b1c79: ret 
0x1408b1c7a: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1c80: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1c86: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1c8d: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1c93: add rsp, 0x88
0x1408b1c9a: pop rbx
0x1408b1c9b: pop rdi
0x1408b1c9c: pop rsi
0x1408b1c9d: pop r12
0x1408b1c9f: pop r13
0x1408b1ca1: pop r14
0x1408b1ca3: pop r15
0x1408b1ca5: pop rbp
0x1408b1ca6: ret 
0x1408b1ca7: nop word ptr [rax + rax]
0x1408b1cb0: mov qword ptr [rsp + 0x10], rdx
0x1408b1cb5: push rbp
0x1408b1cb6: push r15
0x1408b1cb8: push r14
0x1408b1cba: push r13
0x1408b1cbc: push r12
0x1408b1cbe: push rsi
0x1408b1cbf: push rdi
0x1408b1cc0: push rbx
0x1408b1cc1: sub rsp, 0x88
0x1408b1cc8: lea rbp, [rdx + 0x80]
0x1408b1ccf: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1cd5: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b1cdc: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1ce2: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1ce8: movzx ebx, byte ptr [rbp + 0xf82]
0x1408b1cef: cmp byte ptr [rbp + 0xf83], 0
0x1408b1cf6: je 0x1408b1d37
0x1408b1cf8: lea rcx, [rbp - 0x38]
0x1408b1cfc: call 0x1408923c0
0x1408b1d01: and bl, 1
0x1408b1d04: mov byte ptr [rbp + 0xf87], bl
0x1408b1d0a: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1d10: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1d16: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1d1d: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1d23: add rsp, 0x88
0x1408b1d2a: pop rbx
0x1408b1d2b: pop rdi
0x1408b1d2c: pop rsi
0x1408b1d2d: pop r12
0x1408b1d2f: pop r13
0x1408b1d31: pop r14
0x1408b1d33: pop r15
0x1408b1d35: pop rbp
0x1408b1d36: ret 
0x1408b1d37: and bl, 1
0x1408b1d3a: mov byte ptr [rbp + 0xf87], bl
0x1408b1d40: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1d46: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1d4c: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1d53: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1d59: add rsp, 0x88
0x1408b1d60: pop rbx
0x1408b1d61: pop rdi
0x1408b1d62: pop rsi
0x1408b1d63: pop r12
0x1408b1d65: pop r13
0x1408b1d67: pop r14
0x1408b1d69: pop r15
0x1408b1d6b: pop rbp
0x1408b1d6c: ret 
0x1408b1d6d: nop dword ptr [rax]
0x1408b1d70: mov qword ptr [rsp + 0x10], rdx
0x1408b1d75: push rbp
0x1408b1d76: push r15
0x1408b1d78: push r14
0x1408b1d7a: push r13
0x1408b1d7c: push r12
0x1408b1d7e: push rsi
0x1408b1d7f: push rdi
0x1408b1d80: push rbx
0x1408b1d81: sub rsp, 0x88
0x1408b1d88: lea rbp, [rdx + 0x80]
0x1408b1d8f: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1d95: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b1d9c: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1da2: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1da8: cmp byte ptr [rbp + 0xf87], 0
0x1408b1daf: je 0x1408b1e14
0x1408b1db1: movabs rax, 0x8000000000000002
0x1408b1dbb: cmp qword ptr [rbp + 0xf60], rax
0x1408b1dc2: jl 0x1408b1de7
0x1408b1dc4: cmp qword ptr [rbp + 0xf60], 0
0x1408b1dcc: je 0x1408b1de7
0x1408b1dce: mov r8d, 1
0x1408b1dd4: mov rcx, qword ptr [rbp + 0xf50]
0x1408b1ddb: mov rdx, qword ptr [rbp + 0xf60]
0x1408b1de2: call 0x140613c20
0x1408b1de7: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1ded: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1df3: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1dfa: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1e00: add rsp, 0x88
0x1408b1e07: pop rbx
0x1408b1e08: pop rdi
0x1408b1e09: pop rsi
0x1408b1e0a: pop r12
0x1408b1e0c: pop r13
0x1408b1e0e: pop r14
0x1408b1e10: pop r15
0x1408b1e12: pop rbp
0x1408b1e13: ret 
0x1408b1e14: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1e1a: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1e20: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1e27: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1e2d: add rsp, 0x88
0x1408b1e34: pop rbx
0x1408b1e35: pop rdi
0x1408b1e36: pop rsi
0x1408b1e37: pop r12
0x1408b1e39: pop r13
0x1408b1e3b: pop r14
0x1408b1e3d: pop r15
0x1408b1e3f: pop rbp
0x1408b1e40: ret 
0x1408b1e41: nop word ptr cs:[rax + rax]
0x1408b1e50: mov qword ptr [rsp + 0x10], rdx
0x1408b1e55: push rbp
0x1408b1e56: push r15
0x1408b1e58: push r14
0x1408b1e5a: push r13
0x1408b1e5c: push r12
0x1408b1e5e: push rsi
0x1408b1e5f: push rdi
0x1408b1e60: push rbx
0x1408b1e61: sub rsp, 0x88
0x1408b1e68: lea rbp, [rdx + 0x80]
0x1408b1e6f: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1e75: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b1e7c: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1e82: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1e88: mov rcx, qword ptr [rbp + 0xf28]
0x1408b1e8f: call 0x140161c30
0x1408b1e94: movaps xmm6, xmmword ptr [rsp + 0x70]
0x1408b1e99: movaps xmm7, xmmword ptr [rsp + 0x60]
0x1408b1e9e: movaps xmm8, xmmword ptr [rsp + 0x50]
0x1408b1ea4: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1eaa: add rsp, 0x88
0x1408b1eb1: pop rbx
0x1408b1eb2: pop rdi
0x1408b1eb3: pop rsi
0x1408b1eb4: pop r12
0x1408b1eb6: pop r13
0x1408b1eb8: pop r14
0x1408b1eba: pop r15
0x1408b1ebc: pop rbp
0x1408b1ebd: ret 
0x1408b1ebe: nop 
0x1408b1ec0: mov qword ptr [rsp + 0x10], rdx
0x1408b1ec5: push rbp
0x1408b1ec6: push r15
0x1408b1ec8: push r14
0x1408b1eca: push r13
0x1408b1ecc: push r12
0x1408b1ece: push rsi
0x1408b1ecf: push rdi
0x1408b1ed0: push rbx
0x1408b1ed1: sub rsp, 0x88
0x1408b1ed8: lea rbp, [rdx + 0x80]
0x1408b1edf: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1ee5: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b1eec: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1ef2: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1ef8: mov rcx, qword ptr [rbp + 0xf30]
0x1408b1eff: call 0x140161c30
0x1408b1f04: movaps xmm6, xmmword ptr [rsp + 0x70]
0x1408b1f09: movaps xmm7, xmmword ptr [rsp + 0x60]
0x1408b1f0e: movaps xmm8, xmmword ptr [rsp + 0x50]
0x1408b1f14: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1f1a: add rsp, 0x88
0x1408b1f21: pop rbx
0x1408b1f22: pop rdi
0x1408b1f23: pop rsi
0x1408b1f24: pop r12
0x1408b1f26: pop r13
0x1408b1f28: pop r14
0x1408b1f2a: pop r15
0x1408b1f2c: pop rbp
0x1408b1f2d: ret 
0x1408b1f2e: nop 
0x1408b1f30: mov qword ptr [rsp + 0x10], rdx
0x1408b1f35: push rbp
0x1408b1f36: push r15
0x1408b1f38: push r14
0x1408b1f3a: push r13
0x1408b1f3c: push r12
0x1408b1f3e: push rsi
0x1408b1f3f: push rdi
0x1408b1f40: push rbx
0x1408b1f41: sub rsp, 0x88
0x1408b1f48: lea rbp, [rdx + 0x80]
0x1408b1f4f: movaps xmmword ptr [rsp + 0x40], xmm9
0x1408b1f55: movdqa xmmword ptr [rsp + 0x50], xmm8
0x1408b1f5c: movdqa xmmword ptr [rsp + 0x60], xmm7
0x1408b1f62: movdqa xmmword ptr [rsp + 0x70], xmm6
0x1408b1f68: lea rcx, [rbp + 0x790]
0x1408b1f6f: call 0x1408935c0
0x1408b1f74: movdqa xmm6, xmmword ptr [rsp + 0x70]
0x1408b1f7a: movdqa xmm7, xmmword ptr [rsp + 0x60]
0x1408b1f80: movdqa xmm8, xmmword ptr [rsp + 0x50]
0x1408b1f87: movaps xmm9, xmmword ptr [rsp + 0x40]
0x1408b1f8d: add rsp, 0x88
0x1408b1f94: pop rbx
0x1408b1f95: pop rdi
0x1408b1f96: pop rsi
0x1408b1f97: pop r12
0x1408b1f99: pop r13
0x1408b1f9b: pop r14
0x1408b1f9d: pop r15
0x1408b1f9f: pop rbp
0x1408b1fa0: ret 
