
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405b0140: 55                          	push	rbp
1405b0141: 41 56                       	push	r14
1405b0143: 56                          	push	rsi
1405b0144: 57                          	push	rdi
1405b0145: 53                          	push	rbx
1405b0146: 48 81 ec a0 00 00 00        	sub	rsp, 0xa0
1405b014d: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1405b0155: 48 c7 45 18 fe ff ff ff     	mov	qword ptr [rbp + 0x18], -0x2
1405b015d: 4c 89 ce                    	mov	rsi, r9
1405b0160: 4c 89 c7                    	mov	rdi, r8
1405b0163: 89 d3                       	mov	ebx, edx
1405b0165: 41 89 ce                    	mov	r14d, ecx
1405b0168: 4c 8d 0d 71 03 45 02        	lea	r9, [rip + 0x2450371]   # 0x142a004e0
1405b016f: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b0173: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b0179: 48 89 fa                    	mov	rdx, rdi
1405b017c: e8 3f de 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b0181: 31 c0                       	xor	eax, eax
1405b0183: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b0187: 0f 81 da 01 00 00           	jno	0x1405b0367 <.text+0x5af367>
1405b018d: 4c 8d 0d dc 57 45 02        	lea	r9, [rip + 0x24557dc]   # 0x142a05970
1405b0194: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b0198: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b019e: 48 89 fa                    	mov	rdx, rdi
1405b01a1: e8 1a de 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b01a6: 31 c0                       	xor	eax, eax
1405b01a8: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b01ac: 0f 81 13 02 00 00           	jno	0x1405b03c5 <.text+0x5af3c5>
1405b01b2: 4c 8d 0d 4f 79 41 02        	lea	r9, [rip + 0x241794f]   # 0x1429c7b08
1405b01b9: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b01bd: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b01c3: 48 89 fa                    	mov	rdx, rdi
1405b01c6: e8 f5 dd 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b01cb: 31 c0                       	xor	eax, eax
1405b01cd: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b01d1: 0f 81 4c 02 00 00           	jno	0x1405b0423 <.text+0x5af423>
1405b01d7: 4c 8d 0d 12 ca 3a 02        	lea	r9, [rip + 0x23aca12]   # 0x14295cbf0
1405b01de: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b01e2: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b01e8: 48 89 fa                    	mov	rdx, rdi
1405b01eb: e8 d0 dd 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b01f0: 31 c0                       	xor	eax, eax
1405b01f2: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b01f6: 0f 81 85 02 00 00           	jno	0x1405b0481 <.text+0x5af481>
1405b01fc: 4c 8d 0d 1d 97 41 02        	lea	r9, [rip + 0x241971d]   # 0x1429c9920
1405b0203: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b0207: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b020d: 48 89 fa                    	mov	rdx, rdi
1405b0210: e8 ab dd 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b0215: 31 c0                       	xor	eax, eax
1405b0217: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b021b: 0f 81 be 02 00 00           	jno	0x1405b04df <.text+0x5af4df>
1405b0221: 4c 8d 0d 68 32 40 02        	lea	r9, [rip + 0x2403268]   # 0x1429b3490
1405b0228: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b022c: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b0232: 48 89 fa                    	mov	rdx, rdi
1405b0235: e8 86 dd 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b023a: 31 c0                       	xor	eax, eax
1405b023c: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b0240: 0f 81 f7 02 00 00           	jno	0x1405b053d <.text+0x5af53d>
1405b0246: 4c 8d 0d 53 c4 3a 02        	lea	r9, [rip + 0x23ac453]   # 0x14295c6a0
1405b024d: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b0251: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b0257: 48 89 fa                    	mov	rdx, rdi
1405b025a: e8 61 dd 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b025f: 31 c0                       	xor	eax, eax
1405b0261: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b0265: 0f 81 30 03 00 00           	jno	0x1405b059b <.text+0x5af59b>
1405b026b: 4c 8d 0d 9e 4e 3f 02        	lea	r9, [rip + 0x23f4e9e]   # 0x1429a5110
1405b0272: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b0276: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b027c: 48 89 fa                    	mov	rdx, rdi
1405b027f: e8 3c dd 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b0284: 31 c0                       	xor	eax, eax
1405b0286: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b028a: 0f 81 69 03 00 00           	jno	0x1405b05f9 <.text+0x5af5f9>
1405b0290: 44 89 f1                    	mov	ecx, r14d
1405b0293: 89 da                       	mov	edx, ebx
1405b0295: 49 89 f8                    	mov	r8, rdi
1405b0298: 49 89 f1                    	mov	r9, rsi
1405b029b: e8 c0 be dc ff              	call	0x14037c160 <.text+0x37b160>
1405b02a0: 4c 8d 0d b1 95 3a 02        	lea	r9, [rip + 0x23a95b1]   # 0x142959858
1405b02a7: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b02ab: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b02b1: 48 89 fa                    	mov	rdx, rdi
1405b02b4: e8 07 dd 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b02b9: 31 c0                       	xor	eax, eax
1405b02bb: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b02bf: 0f 81 92 03 00 00           	jno	0x1405b0657 <.text+0x5af657>
1405b02c5: 4c 8d 0d 44 5e 38 02        	lea	r9, [rip + 0x2385e44]   # 0x142936110
1405b02cc: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b02d0: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b02d6: 48 89 fa                    	mov	rdx, rdi
1405b02d9: e8 e2 dc 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b02de: 31 c0                       	xor	eax, eax
1405b02e0: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b02e4: 0f 81 cb 03 00 00           	jno	0x1405b06b5 <.text+0x5af6b5>
1405b02ea: 4c 8d 0d cf ac 3f 02        	lea	r9, [rip + 0x23faccf]   # 0x1429aafc0
1405b02f1: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b02f5: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b02fb: 48 89 fa                    	mov	rdx, rdi
1405b02fe: e8 bd dc 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b0303: 31 c0                       	xor	eax, eax
1405b0305: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b0309: 0f 81 04 04 00 00           	jno	0x1405b0713 <.text+0x5af713>
1405b030f: 4c 8d 0d c2 f9 43 02        	lea	r9, [rip + 0x243f9c2]   # 0x1429efcd8
1405b0316: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b031a: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b0320: 48 89 fa                    	mov	rdx, rdi
1405b0323: e8 98 dc 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b0328: 31 c0                       	xor	eax, eax
1405b032a: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b032e: 0f 81 3d 04 00 00           	jno	0x1405b0771 <.text+0x5af771>
1405b0334: 4c 8d 0d 65 55 45 02        	lea	r9, [rip + 0x2455565]   # 0x142a058a0
1405b033b: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1405b033f: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b0345: 48 89 fa                    	mov	rdx, rdi
1405b0348: e8 73 dc 6f 00              	call	0x140cadfc0 <.text+0xcacfc0>
1405b034d: 31 c0                       	xor	eax, eax
1405b034f: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1405b0353: 0f 81 73 04 00 00           	jno	0x1405b07cc <.text+0x5af7cc>
1405b0359: 48 81 c4 a0 00 00 00        	add	rsp, 0xa0
1405b0360: 5b                          	pop	rbx
1405b0361: 5f                          	pop	rdi
1405b0362: 5e                          	pop	rsi
1405b0363: 41 5e                       	pop	r14
1405b0365: 5d                          	pop	rbp
1405b0366: c3                          	ret
1405b0367: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b036b: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b036f: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b0373: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b0377: 48 89 e8                    	mov	rax, rbp
1405b037a: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b037e: 48 8d 05 4b b7 a8 ff        	lea	rax, [rip - 0x5748b5]   # 0x14003bad0 <.text+0x3aad0>
1405b0385: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1405b0389: 48 8d 05 00 02 45 02        	lea	rax, [rip + 0x2450200]  # 0x142a00590
1405b0390: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1405b0394: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1405b039c: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1405b03a4: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1405b03a8: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1405b03ac: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1405b03b4: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1405b03b8: 48 89 f2                    	mov	rdx, rsi
1405b03bb: e8 d0 8f 32 02              	call	0x1428d9390 <.text+0x28d8390>
1405b03c0: e9 60 04 00 00              	jmp	0x1405b0825 <.text+0x5af825>
1405b03c5: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b03c9: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b03cd: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b03d1: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b03d5: 48 89 e8                    	mov	rax, rbp
1405b03d8: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b03dc: 48 8d 05 ed b6 a8 ff        	lea	rax, [rip - 0x574913]   # 0x14003bad0 <.text+0x3aad0>
1405b03e3: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1405b03e7: 48 8d 05 62 55 45 02        	lea	rax, [rip + 0x2455562]  # 0x142a05950
1405b03ee: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1405b03f2: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1405b03fa: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1405b0402: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1405b0406: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1405b040a: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1405b0412: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1405b0416: 48 89 f2                    	mov	rdx, rsi
1405b0419: e8 72 8f 32 02              	call	0x1428d9390 <.text+0x28d8390>
1405b041e: e9 02 04 00 00              	jmp	0x1405b0825 <.text+0x5af825>
1405b0423: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b0427: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b042b: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b042f: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b0433: 48 89 e8                    	mov	rax, rbp
1405b0436: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b043a: 48 8d 05 8f b6 a8 ff        	lea	rax, [rip - 0x574971]   # 0x14003bad0 <.text+0x3aad0>
1405b0441: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1405b0445: 48 8d 05 6c 77 41 02        	lea	rax, [rip + 0x241776c]  # 0x1429c7bb8
1405b044c: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1405b0450: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1405b0458: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1405b0460: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1405b0464: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1405b0468: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1405b0470: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1405b0474: 48 89 f2                    	mov	rdx, rsi
1405b0477: e8 14 8f 32 02              	call	0x1428d9390 <.text+0x28d8390>
1405b047c: e9 a4 03 00 00              	jmp	0x1405b0825 <.text+0x5af825>
1405b0481: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b0485: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b0489: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b048d: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b0491: 48 89 e8                    	mov	rax, rbp
1405b0494: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b0498: 48 8d 05 31 b6 a8 ff        	lea	rax, [rip - 0x5749cf]   # 0x14003bad0 <.text+0x3aad0>
1405b049f: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1405b04a3: 48 8d 05 b6 c6 3a 02        	lea	rax, [rip + 0x23ac6b6]  # 0x14295cb60
1405b04aa: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1405b04ae: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1405b04b6: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1405b04be: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1405b04c2: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1405b04c6: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1405b04ce: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1405b04d2: 48 89 f2                    	mov	rdx, rsi
1405b04d5: e8 b6 8e 32 02              	call	0x1428d9390 <.text+0x28d8390>
1405b04da: e9 46 03 00 00              	jmp	0x1405b0825 <.text+0x5af825>
1405b04df: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b04e3: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b04e7: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b04eb: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b04ef: 48 89 e8                    	mov	rax, rbp
1405b04f2: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b04f6: 48 8d 05 d3 b5 a8 ff        	lea	rax, [rip - 0x574a2d]   # 0x14003bad0 <.text+0x3aad0>
1405b04fd: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1405b0501: 48 8d 05 f8 93 41 02        	lea	rax, [rip + 0x24193f8]  # 0x1429c9900
1405b0508: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1405b050c: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1405b0514: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1405b051c: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1405b0520: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1405b0524: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1405b052c: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1405b0530: 48 89 f2                    	mov	rdx, rsi
1405b0533: e8 58 8e 32 02              	call	0x1428d9390 <.text+0x28d8390>
1405b0538: e9 e8 02 00 00              	jmp	0x1405b0825 <.text+0x5af825>
1405b053d: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b0541: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b0545: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b0549: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b054d: 48 89 e8                    	mov	rax, rbp
1405b0550: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b0554: 48 8d 05 75 b5 a8 ff        	lea	rax, [rip - 0x574a8b]   # 0x14003bad0 <.text+0x3aad0>
1405b055b: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1405b055f: 48 8d 05 da 2f 40 02        	lea	rax, [rip + 0x2402fda]  # 0x1429b3540
1405b0566: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1405b056a: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1405b0572: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1405b057a: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1405b057e: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1405b0582: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1405b058a: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1405b058e: 48 89 f2                    	mov	rdx, rsi
1405b0591: e8 fa 8d 32 02              	call	0x1428d9390 <.text+0x28d8390>
1405b0596: e9 8a 02 00 00              	jmp	0x1405b0825 <.text+0x5af825>
1405b059b: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b059f: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b05a3: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b05a7: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b05ab: 48 89 e8                    	mov	rax, rbp
1405b05ae: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b05b2: 48 8d 05 17 b5 a8 ff        	lea	rax, [rip - 0x574ae9]   # 0x14003bad0 <.text+0x3aad0>
1405b05b9: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1405b05bd: 48 8d 05 8c c1 3a 02        	lea	rax, [rip + 0x23ac18c]  # 0x14295c750
1405b05c4: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1405b05c8: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1405b05d0: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1405b05d8: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1405b05dc: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1405b05e0: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1405b05e8: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1405b05ec: 48 89 f2                    	mov	rdx, rsi
1405b05ef: e8 9c 8d 32 02              	call	0x1428d9390 <.text+0x28d8390>
1405b05f4: e9 2c 02 00 00              	jmp	0x1405b0825 <.text+0x5af825>
1405b05f9: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b05fd: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b0601: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b0605: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b0609: 48 89 e8                    	mov	rax, rbp
1405b060c: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b0610: 48 8d 05 b9 b4 a8 ff        	lea	rax, [rip - 0x574b47]   # 0x14003bad0 <.text+0x3aad0>
1405b0617: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1405b061b: 48 8d 05 ce 4a 3f 02        	lea	rax, [rip + 0x23f4ace]  # 0x1429a50f0
1405b0622: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1405b0626: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1405b062e: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1405b0636: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1405b063a: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1405b063e: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1405b0646: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1405b064a: 48 89 f2                    	mov	rdx, rsi
1405b064d: e8 3e 8d 32 02              	call	0x1428d9390 <.text+0x28d8390>
1405b0652: e9 ce 01 00 00              	jmp	0x1405b0825 <.text+0x5af825>
1405b0657: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b065b: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b065f: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b0663: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b0667: 48 89 e8                    	mov	rax, rbp
1405b066a: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b066e: 48 8d 05 5b b4 a8 ff        	lea	rax, [rip - 0x574ba5]   # 0x14003bad0 <.text+0x3aad0>
1405b0675: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1405b0679: 48 8d 05 88 92 3a 02        	lea	rax, [rip + 0x23a9288]  # 0x142959908
1405b0680: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1405b0684: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1405b068c: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1405b0694: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1405b0698: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1405b069c: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1405b06a4: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1405b06a8: 48 89 f2                    	mov	rdx, rsi
1405b06ab: e8 e0 8c 32 02              	call	0x1428d9390 <.text+0x28d8390>
1405b06b0: e9 70 01 00 00              	jmp	0x1405b0825 <.text+0x5af825>
1405b06b5: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b06b9: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b06bd: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b06c1: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b06c5: 48 89 e8                    	mov	rax, rbp
1405b06c8: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b06cc: 48 8d 05 fd b3 a8 ff        	lea	rax, [rip - 0x574c03]   # 0x14003bad0 <.text+0x3aad0>
1405b06d3: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1405b06d7: 48 8d 05 e2 5a 38 02        	lea	rax, [rip + 0x2385ae2]  # 0x1429361c0
1405b06de: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1405b06e2: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1405b06ea: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1405b06f2: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1405b06f6: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1405b06fa: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1405b0702: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1405b0706: 48 89 f2                    	mov	rdx, rsi
1405b0709: e8 82 8c 32 02              	call	0x1428d9390 <.text+0x28d8390>
1405b070e: e9 12 01 00 00              	jmp	0x1405b0825 <.text+0x5af825>
1405b0713: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b0717: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b071b: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b071f: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b0723: 48 89 e8                    	mov	rax, rbp
1405b0726: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b072a: 48 8d 05 9f b3 a8 ff        	lea	rax, [rip - 0x574c61]   # 0x14003bad0 <.text+0x3aad0>
1405b0731: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1405b0735: 48 8d 05 34 a9 3f 02        	lea	rax, [rip + 0x23fa934]  # 0x1429ab070
1405b073c: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1405b0740: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1405b0748: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1405b0750: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1405b0754: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1405b0758: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1405b0760: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1405b0764: 48 89 f2                    	mov	rdx, rsi
1405b0767: e8 24 8c 32 02              	call	0x1428d9390 <.text+0x28d8390>
1405b076c: e9 b4 00 00 00              	jmp	0x1405b0825 <.text+0x5af825>
1405b0771: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b0775: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b0779: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b077d: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b0781: 48 89 e8                    	mov	rax, rbp
1405b0784: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b0788: 48 8d 05 41 b3 a8 ff        	lea	rax, [rip - 0x574cbf]   # 0x14003bad0 <.text+0x3aad0>
1405b078f: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1405b0793: 48 8d 05 1e f5 43 02        	lea	rax, [rip + 0x243f51e]  # 0x1429efcb8
1405b079a: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1405b079e: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1405b07a6: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1405b07ae: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1405b07b2: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1405b07b6: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1405b07be: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1405b07c2: 48 89 f2                    	mov	rdx, rsi
1405b07c5: e8 c6 8b 32 02              	call	0x1428d9390 <.text+0x28d8390>
1405b07ca: eb 59                       	jmp	0x1405b0825 <.text+0x5af825>
1405b07cc: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1405b07d0: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1405b07d4: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b07d8: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1405b07dc: 48 89 e8                    	mov	rax, rbp
1405b07df: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1405b07e3: 48 8d 05 e6 b2 a8 ff        	lea	rax, [rip - 0x574d1a]   # 0x14003bad0 <.text+0x3aad0>
1405b07ea: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
