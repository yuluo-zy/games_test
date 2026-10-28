
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14011d170: 55                          	push	rbp
14011d171: 41 57                       	push	r15
14011d173: 41 56                       	push	r14
14011d175: 41 54                       	push	r12
14011d177: 56                          	push	rsi
14011d178: 57                          	push	rdi
14011d179: 53                          	push	rbx
14011d17a: 48 81 ec 50 02 00 00        	sub	rsp, 0x250
14011d181: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14011d189: 48 c7 85 c8 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x1c8], -0x2
14011d194: 89 d3                       	mov	ebx, edx
14011d196: 48 89 ce                    	mov	rsi, rcx
14011d199: 48 8d 79 38                 	lea	rdi, [rcx + 0x38]
14011d19d: 48 8d 81 a8 05 00 00        	lea	rax, [rcx + 0x5a8]
14011d1a4: 48 89 bd a8 01 00 00        	mov	qword ptr [rbp + 0x1a8], rdi
14011d1ab: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
14011d1b2: 48 83 b9 88 00 00 00 00     	cmp	qword ptr [rcx + 0x88], 0x0
14011d1ba: 48 b8 5b 50 98 59 aa 1c d7 4a       	movabs	rax, 0x4ad71caa5998505b
14011d1c4: 48 b9 6e 3c 95 ef 33 28 0d 3d       	movabs	rcx, 0x3d0d2833ef953c6e
14011d1ce: 0f 84 96 00 00 00           	je	0x14011d26a <.text+0x11c26a>
14011d1d4: 48 8b 56 70                 	mov	rdx, qword ptr [rsi + 0x70]
14011d1d8: 4c 8b 46 78                 	mov	r8, qword ptr [rsi + 0x78]
14011d1dc: 4d 89 c1                    	mov	r9, r8
14011d1df: 49 21 c1                    	and	r9, rax
14011d1e2: 45 31 d2                    	xor	r10d, r10d
14011d1e5: 66 0f 6f 05 93 32 83 02     	movdqa	xmm0, xmmword ptr [rip + 0x2833293] # 0x142950480
14011d1ed: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
14011d1f1: f3 42 0f 6f 14 0a           	movdqu	xmm2, xmmword ptr [rdx + r9]
14011d1f7: 66 0f 6f da                 	movdqa	xmm3, xmm2
14011d1fb: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
14011d1ff: 66 44 0f d7 db              	pmovmskb	r11d, xmm3
14011d204: 45 85 db                    	test	r11d, r11d
14011d207: 75 34                       	jne	0x14011d23d <.text+0x11c23d>
14011d209: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
14011d20d: 66 44 0f d7 da              	pmovmskb	r11d, xmm2
14011d212: 45 85 db                    	test	r11d, r11d
14011d215: 75 53                       	jne	0x14011d26a <.text+0x11c26a>
14011d217: 4d 01 d1                    	add	r9, r10
14011d21a: 49 83 c1 10                 	add	r9, 0x10
14011d21e: 49 83 c2 10                 	add	r10, 0x10
14011d222: 4d 21 c1                    	and	r9, r8
14011d225: eb ca                       	jmp	0x14011d1f1 <.text+0x11c1f1>
14011d227: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
14011d230: 45 8d 73 ff                 	lea	r14d, [r11 - 0x1]
14011d234: 66 45 21 de                 	and	r14w, r11w
14011d238: 45 89 f3                    	mov	r11d, r14d
14011d23b: 74 cc                       	je	0x14011d209 <.text+0x11c209>
14011d23d: f3 45 0f bc f3              	tzcnt	r14d, r11d
14011d242: 4d 01 ce                    	add	r14, r9
14011d245: 4d 21 c6                    	and	r14, r8
14011d248: 49 f7 de                    	neg	r14
14011d24b: 4f 8d 34 76                 	lea	r14, [r14 + 2*r14]
14011d24f: 4a 39 4c f2 e8              	cmp	qword ptr [rdx + 8*r14 - 0x18], rcx
14011d254: 75 da                       	jne	0x14011d230 <.text+0x11c230>
14011d256: 4a 39 44 f2 f0              	cmp	qword ptr [rdx + 8*r14 - 0x10], rax
14011d25b: 75 d3                       	jne	0x14011d230 <.text+0x11c230>
14011d25d: 4a 8d 04 f2                 	lea	rax, [rdx + 8*r14]
14011d261: 4c 8b 70 f8                 	mov	r14, qword ptr [rax - 0x8]
14011d265: e9 ed 02 00 00              	jmp	0x14011d557 <.text+0x11c557>
14011d26a: 0f b6 96 98 00 00 00        	movzx	edx, byte ptr [rsi + 0x98]
14011d271: 48 8b 96 d8 00 00 00        	mov	rdx, qword ptr [rsi + 0xd8]
14011d278: 4c 8b 8e e0 00 00 00        	mov	r9, qword ptr [rsi + 0xe0]
14011d27f: 4d 89 ca                    	mov	r10, r9
14011d282: 49 21 c2                    	and	r10, rax
14011d285: 45 31 db                    	xor	r11d, r11d
14011d288: 66 0f 6f 05 f0 31 83 02     	movdqa	xmm0, xmmword ptr [rip + 0x28331f0] # 0x142950480
14011d290: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
14011d294: f3 42 0f 6f 14 12           	movdqu	xmm2, xmmword ptr [rdx + r10]
14011d29a: 66 0f 6f da                 	movdqa	xmm3, xmm2
14011d29e: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
14011d2a2: 66 44 0f d7 f3              	pmovmskb	r14d, xmm3
14011d2a7: 45 85 f6                    	test	r14d, r14d
14011d2aa: 75 31                       	jne	0x14011d2dd <.text+0x11c2dd>
14011d2ac: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
14011d2b0: 66 44 0f d7 c2              	pmovmskb	r8d, xmm2
14011d2b5: 45 85 c0                    	test	r8d, r8d
14011d2b8: 0f 85 ef 00 00 00           	jne	0x14011d3ad <.text+0x11c3ad>
14011d2be: 4d 01 da                    	add	r10, r11
14011d2c1: 49 83 c2 10                 	add	r10, 0x10
14011d2c5: 49 83 c3 10                 	add	r11, 0x10
14011d2c9: 4d 21 ca                    	and	r10, r9
14011d2cc: eb c6                       	jmp	0x14011d294 <.text+0x11c294>
14011d2ce: 66 90                       	nop
14011d2d0: 45 8d 46 ff                 	lea	r8d, [r14 - 0x1]
14011d2d4: 66 45 21 f0                 	and	r8w, r14w
14011d2d8: 45 89 c6                    	mov	r14d, r8d
14011d2db: 74 cf                       	je	0x14011d2ac <.text+0x11c2ac>
14011d2dd: f3 45 0f bc fe              	tzcnt	r15d, r14d
14011d2e2: 4d 01 d7                    	add	r15, r10
14011d2e5: 4d 21 cf                    	and	r15, r9
14011d2e8: 4d 89 f8                    	mov	r8, r15
14011d2eb: 49 f7 d8                    	neg	r8
14011d2ee: 4e 8d 24 fd 00 00 00 00     	lea	r12, [8*r15]
14011d2f6: 49 c1 e0 07                 	shl	r8, 0x7
14011d2fa: 4d 29 e0                    	sub	r8, r12
14011d2fd: 4a 39 8c 02 78 ff ff ff     	cmp	qword ptr [rdx + r8 - 0x88], rcx
14011d305: 75 c9                       	jne	0x14011d2d0 <.text+0x11c2d0>
14011d307: 4a 39 44 02 80              	cmp	qword ptr [rdx + r8 - 0x80], rax
14011d30c: 75 c2                       	jne	0x14011d2d0 <.text+0x11c2d0>
14011d30e: 4d 89 fa                    	mov	r10, r15
14011d311: 49 c1 e2 07                 	shl	r10, 0x7
14011d315: 4f 8d 1c fa                 	lea	r11, [r10 + 8*r15]
14011d319: 49 c1 fb 03                 	sar	r11, 0x3
14011d31d: 49 ba f1 f0 f0 f0 f0 f0 f0 f0       	movabs	r10, -0xf0f0f0f0f0f0f0f
14011d327: 4d 0f af d3                 	imul	r10, r11
14011d32b: 4d 8d 5a f0                 	lea	r11, [r10 - 0x10]
14011d32f: 4d 21 cb                    	and	r11, r9
14011d332: f3 42 0f 6f 04 1a           	movdqu	xmm0, xmmword ptr [rdx + r11]
14011d338: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
14011d33c: 66 0f 74 c1                 	pcmpeqb	xmm0, xmm1
14011d340: 66 44 0f d7 f0              	pmovmskb	r14d, xmm0
14011d345: f3 42 0f 6f 04 12           	movdqu	xmm0, xmmword ptr [rdx + r10]
14011d34b: 66 0f 74 c1                 	pcmpeqb	xmm0, xmm1
14011d34f: 66 44 0f d7 c8              	pmovmskb	r9d, xmm0
14011d354: 45 85 f6                    	test	r14d, r14d
14011d357: 74 0b                       	je	0x14011d364 <.text+0x11c364>
14011d359: 66 45 0f bd f6              	bsr	r14w, r14w
14011d35e: 41 83 f6 0f                 	xor	r14d, 0xf
14011d362: eb 05                       	jmp	0x14011d369 <.text+0x11c369>
14011d364: 66 41 be 10 00              	mov	r14w, 0x10
14011d369: 49 01 d0                    	add	r8, rdx
14011d36c: 41 81 c9 00 00 01 00        	or	r9d, 0x10000
14011d373: f3 45 0f bc c9              	tzcnt	r9d, r9d
14011d378: 45 01 ce                    	add	r14d, r9d
14011d37b: 41 b1 80                    	mov	r9b, -0x80
14011d37e: 66 41 83 fe 0f              	cmp	r14w, 0xf
14011d383: 77 0a                       	ja	0x14011d38f <.text+0x11c38f>
14011d385: 48 ff 86 e8 00 00 00        	inc	qword ptr [rsi + 0xe8]
14011d38c: 41 b1 ff                    	mov	r9b, -0x1
14011d38f: 46 88 0c 12                 	mov	byte ptr [rdx + r10], r9b
14011d393: 46 88 4c 1a 10              	mov	byte ptr [rdx + r11 + 0x10], r9b
14011d398: 48 ff 8e f0 00 00 00        	dec	qword ptr [rsi + 0xf0]
14011d39f: 49 8b 50 88                 	mov	rdx, qword ptr [r8 - 0x78]
14011d3a3: 48 83 fa 02                 	cmp	rdx, 0x2
14011d3a7: 0f 85 37 01 00 00           	jne	0x14011d4e4 <.text+0x11c4e4>
14011d3ad: 4c 8b b6 a8 05 00 00        	mov	r14, qword ptr [rsi + 0x5a8]
14011d3b4: 4d 8d 7e 01                 	lea	r15, [r14 + 0x1]
14011d3b8: 4c 89 be a8 05 00 00        	mov	qword ptr [rsi + 0x5a8], r15
14011d3bf: 48 ba 00 00 00 00 00 00 00 80       	movabs	rdx, -0x8000000000000000
14011d3c9: 48 89 95 80 01 00 00        	mov	qword ptr [rbp + 0x180], rdx
14011d3d0: 48 8d 15 69 9c 83 02        	lea	rdx, [rip + 0x2839c69]  # 0x142957040
14011d3d7: 48 89 95 88 01 00 00        	mov	qword ptr [rbp + 0x188], rdx
14011d3de: 48 c7 85 90 01 00 00 49 00 00 00    	mov	qword ptr [rbp + 0x190], 0x49
14011d3e9: 66 c7 85 a0 01 00 00 00 01  	mov	word ptr [rbp + 0x1a0], 0x100
14011d3f2: 48 c7 85 48 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x148], 0x1
14011d3fd: 48 89 8d 50 01 00 00        	mov	qword ptr [rbp + 0x150], rcx
14011d404: 48 89 85 58 01 00 00        	mov	qword ptr [rbp + 0x158], rax
14011d40b: 48 c7 85 70 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x170], 0x1
14011d416: 48 c7 85 78 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x178], 0x1
14011d421: 48 c7 85 98 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x198], 0x0
14011d42c: c6 85 a2 01 00 00 01        	mov	byte ptr [rbp + 0x1a2], 0x1
14011d433: 48 c7 85 60 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x160], 0x0
14011d43e: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
14011d442: 4c 8d 85 48 01 00 00        	lea	r8, [rbp + 0x148]
14011d449: 4c 89 f2                    	mov	rdx, r14
14011d44c: e8 1f 24 c2 00              	call	0x140d3f870 <.text+0xd3e870>
14011d451: 4c 39 7e 48                 	cmp	qword ptr [rsi + 0x48], r15
14011d455: 73 19                       	jae	0x14011d470 <.text+0x11c470>
14011d457: c6 85 c7 01 00 00 01        	mov	byte ptr [rbp + 0x1c7], 0x1
14011d45e: 4c 8d 05 1b 34 83 02        	lea	r8, [rip + 0x283341b]   # 0x142950880
14011d465: 48 89 f9                    	mov	rcx, rdi
14011d468: 4c 89 fa                    	mov	rdx, r15
14011d46b: e8 20 82 19 00              	call	0x1402b5690 <.text+0x2b4690>
14011d470: 4c 8b 7e 40                 	mov	r15, qword ptr [rsi + 0x40]
14011d474: 4d 69 e6 d0 00 00 00        	imul	r12, r14, 0xd0
14011d47b: 4b 8d 04 27                 	lea	rax, [r15 + r12]
14011d47f: 48 89 85 b8 01 00 00        	mov	qword ptr [rbp + 0x1b8], rax
14011d486: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
14011d48a: 48 8d 55 a8                 	lea	rdx, [rbp - 0x58]
14011d48e: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
14011d494: e8 67 67 80 02              	call	0x142923c00 <.text+0x2922c00>
14011d499: 43 83 3c 27 02              	cmp	dword ptr [r15 + r12], 0x2
14011d49e: 74 0c                       	je	0x14011d4ac <.text+0x11c4ac>
14011d4a0: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
14011d4a7: e8 a4 36 4d 00              	call	0x1405f0b50 <.text+0x5efb50>
14011d4ac: 48 8d 55 78                 	lea	rdx, [rbp + 0x78]
14011d4b0: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
14011d4b6: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
14011d4bd: e8 3e 67 80 02              	call	0x142923c00 <.text+0x2922c00>
14011d4c2: 48 8d 4e 70                 	lea	rcx, [rsi + 0x70]
14011d4c6: 48 ba 6e 3c 95 ef 33 28 0d 3d       	movabs	rdx, 0x3d0d2833ef953c6e
14011d4d0: 49 b8 5b 50 98 59 aa 1c d7 4a       	movabs	r8, 0x4ad71caa5998505b
14011d4da: 4d 89 f1                    	mov	r9, r14
14011d4dd: e8 3e 57 f6 ff              	call	0x140082c20 <.text+0x81c20>
14011d4e2: eb 73                       	jmp	0x14011d557 <.text+0x11c557>
14011d4e4: 41 0f 10 40 f0              	movups	xmm0, xmmword ptr [r8 - 0x10]
14011d4e9: 0f 11 85 e0 00 00 00        	movups	xmmword ptr [rbp + 0xe0], xmm0
14011d4f0: 41 0f 10 40 e0              	movups	xmm0, xmmword ptr [r8 - 0x20]
14011d4f5: 0f 11 85 d0 00 00 00        	movups	xmmword ptr [rbp + 0xd0], xmm0
14011d4fc: 41 0f 10 40 d0              	movups	xmm0, xmmword ptr [r8 - 0x30]
14011d501: 0f 11 85 c0 00 00 00        	movups	xmmword ptr [rbp + 0xc0], xmm0
14011d508: f3 41 0f 6f 40 90           	movdqu	xmm0, xmmword ptr [r8 - 0x70]
14011d50e: f3 41 0f 6f 48 a0           	movdqu	xmm1, xmmword ptr [r8 - 0x60]
14011d514: f3 41 0f 6f 50 b0           	movdqu	xmm2, xmmword ptr [r8 - 0x50]
14011d51a: f3 41 0f 6f 58 c0           	movdqu	xmm3, xmmword ptr [r8 - 0x40]
14011d520: f3 0f 7f 9d b0 00 00 00     	movdqu	xmmword ptr [rbp + 0xb0], xmm3
14011d528: f3 0f 7f 95 a0 00 00 00     	movdqu	xmmword ptr [rbp + 0xa0], xmm2
14011d530: f3 0f 7f 8d 90 00 00 00     	movdqu	xmmword ptr [rbp + 0x90], xmm1
14011d538: f3 0f 7f 85 80 00 00 00     	movdqu	xmmword ptr [rbp + 0x80], xmm0
14011d540: 48 89 55 78                 	mov	qword ptr [rbp + 0x78], rdx
14011d544: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
14011d548: 48 8d 95 a8 01 00 00        	lea	rdx, [rbp + 0x1a8]
14011d54f: e8 ec 24 c2 00              	call	0x140d3fa40 <.text+0xd3ea40>
14011d554: 49 89 c6                    	mov	r14, rax
14011d557: 88 9d 48 01 00 00           	mov	byte ptr [rbp + 0x148], bl
14011d55d: 8b 9e b0 05 00 00           	mov	ebx, dword ptr [rsi + 0x5b0]
14011d563: 48 89 f1                    	mov	rcx, rsi
14011d566: e8 e5 ac bd 00              	call	0x140cf8250 <.text+0xcf7250>
14011d56b: 48 8d 86 f8 00 00 00        	lea	rax, [rsi + 0xf8]
14011d572: 4c 89 75 a8                 	mov	qword ptr [rbp - 0x58], r14
14011d576: 48 8d 8e d8 01 00 00        	lea	rcx, [rsi + 0x1d8]
14011d57d: 48 89 7d 78                 	mov	qword ptr [rbp + 0x78], rdi
14011d581: 48 8d 55 a8                 	lea	rdx, [rbp - 0x58]
14011d585: 48 89 95 80 00 00 00        	mov	qword ptr [rbp + 0x80], rdx
14011d58c: 48 89 85 88 00 00 00        	mov	qword ptr [rbp + 0x88], rax
14011d593: 4c 8d 45 78                 	lea	r8, [rbp + 0x78]
14011d597: 4c 89 f2                    	mov	rdx, r14
14011d59a: e8 e1 bb 3e 00              	call	0x140509180 <.text+0x508180>
14011d59f: 48 89 c7                    	mov	rdi, rax
14011d5a2: 48 83 78 20 00              	cmp	qword ptr [rax + 0x20], 0x0
14011d5a7: 74 13                       	je	0x14011d5bc <.text+0x11c5bc>
14011d5a9: 4c 8d 85 48 01 00 00        	lea	r8, [rbp + 0x148]
14011d5b0: 48 89 f9                    	mov	rcx, rdi
14011d5b3: 31 d2                       	xor	edx, edx
14011d5b5: e8 b6 20 be 00              	call	0x140cff670 <.text+0xcfe670>
14011d5ba: eb 2c                       	jmp	0x14011d5e8 <.text+0x11c5e8>
14011d5bc: 48 83 7f 18 00              	cmp	qword ptr [rdi + 0x18], 0x0
14011d5c1: 74 3d                       	je	0x14011d600 <.text+0x11c600>
14011d5c3: 31 c9                       	xor	ecx, ecx
14011d5c5: 48 8d 41 01                 	lea	rax, [rcx + 0x1]
14011d5c9: 48 89 47 20                 	mov	qword ptr [rdi + 0x20], rax
14011d5cd: 4c 8b 47 08                 	mov	r8, qword ptr [rdi + 0x8]
14011d5d1: 49 0f af c8                 	imul	rcx, r8
14011d5d5: 48 03 4f 10                 	add	rcx, qword ptr [rdi + 0x10]
14011d5d9: 48 8d 95 48 01 00 00        	lea	rdx, [rbp + 0x148]
14011d5e0: e8 1b 66 80 02              	call	0x142923c00 <.text+0x2922c00>
14011d5e5: 89 5f 58                    	mov	dword ptr [rdi + 0x58], ebx
14011d5e8: 89 5f 5c                    	mov	dword ptr [rdi + 0x5c], ebx
14011d5eb: 48 89 f0                    	mov	rax, rsi
14011d5ee: 48 81 c4 50 02 00 00        	add	rsp, 0x250
14011d5f5: 5b                          	pop	rbx
14011d5f6: 5f                          	pop	rdi
14011d5f7: 5e                          	pop	rsi
14011d5f8: 41 5c                       	pop	r12
14011d5fa: 41 5e                       	pop	r14
14011d5fc: 41 5f                       	pop	r15
14011d5fe: 5d                          	pop	rbp
14011d5ff: c3                          	ret
14011d600: ba 01 00 00 00              	mov	edx, 0x1
14011d605: 48 89 f9                    	mov	rcx, rdi
14011d608: e8 33 16 7b 02              	call	0x1428cec40 <.text+0x28cdc40>
14011d60d: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14011d611: eb b2                       	jmp	0x14011d5c5 <.text+0x11c5c5>
14011d613: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14011d620: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14011d625: 55                          	push	rbp
14011d626: 41 57                       	push	r15
14011d628: 41 56                       	push	r14
14011d62a: 41 54                       	push	r12
14011d62c: 56                          	push	rsi
14011d62d: 57                          	push	rdi
14011d62e: 53                          	push	rbx
14011d62f: 48 83 ec 20                 	sub	rsp, 0x20
14011d633: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14011d63a: 48 8d 55 78                 	lea	rdx, [rbp + 0x78]
14011d63e: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
14011d644: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
14011d64b: e8 b0 65 80 02              	call	0x142923c00 <.text+0x2922c00>
14011d650: c6 85 c7 01 00 00 00        	mov	byte ptr [rbp + 0x1c7], 0x0
14011d657: 48 83 c4 20                 	add	rsp, 0x20
14011d65b: 5b                          	pop	rbx
14011d65c: 5f                          	pop	rdi
14011d65d: 5e                          	pop	rsi
14011d65e: 41 5c                       	pop	r12
14011d660: 41 5e                       	pop	r14
14011d662: 41 5f                       	pop	r15
14011d664: 5d                          	pop	rbp
14011d665: c3                          	ret
14011d666: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
14011d670: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14011d675: 55                          	push	rbp
14011d676: 41 57                       	push	r15
14011d678: 41 56                       	push	r14
14011d67a: 41 54                       	push	r12
14011d67c: 56                          	push	rsi
14011d67d: 57                          	push	rdi
14011d67e: 53                          	push	rbx
14011d67f: 48 83 ec 20                 	sub	rsp, 0x20
14011d683: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14011d68a: 80 bd c7 01 00 00 00        	cmp	byte ptr [rbp + 0x1c7], 0x0
14011d691: 75 0f                       	jne	0x14011d6a2 <.text+0x11c6a2>
14011d693: 48 83 c4 20                 	add	rsp, 0x20
14011d697: 5b                          	pop	rbx
14011d698: 5f                          	pop	rdi
14011d699: 5e                          	pop	rsi
14011d69a: 41 5c                       	pop	r12
14011d69c: 41 5e                       	pop	r14
14011d69e: 41 5f                       	pop	r15
14011d6a0: 5d                          	pop	rbp
14011d6a1: c3                          	ret
14011d6a2: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
14011d6a6: e8 a5 34 4d 00              	call	0x1405f0b50 <.text+0x5efb50>
14011d6ab: eb e6                       	jmp	0x14011d693 <.text+0x11c693>
