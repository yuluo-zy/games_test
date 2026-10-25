
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140959fc0: 55                          	push	rbp
140959fc1: 41 57                       	push	r15
140959fc3: 41 56                       	push	r14
140959fc5: 41 55                       	push	r13
140959fc7: 41 54                       	push	r12
140959fc9: 56                          	push	rsi
140959fca: 57                          	push	rdi
140959fcb: 53                          	push	rbx
140959fcc: 48 81 ec b8 04 00 00        	sub	rsp, 0x4b8
140959fd3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140959fdb: 44 0f 29 bd 20 04 00 00     	movaps	xmmword ptr [rbp + 0x420], xmm15
140959fe3: 44 0f 29 b5 10 04 00 00     	movaps	xmmword ptr [rbp + 0x410], xmm14
140959feb: 44 0f 29 ad 00 04 00 00     	movaps	xmmword ptr [rbp + 0x400], xmm13
140959ff3: 44 0f 29 a5 f0 03 00 00     	movaps	xmmword ptr [rbp + 0x3f0], xmm12
140959ffb: 44 0f 29 9d e0 03 00 00     	movaps	xmmword ptr [rbp + 0x3e0], xmm11
14095a003: 44 0f 29 95 d0 03 00 00     	movaps	xmmword ptr [rbp + 0x3d0], xmm10
14095a00b: 44 0f 29 8d c0 03 00 00     	movaps	xmmword ptr [rbp + 0x3c0], xmm9
14095a013: 44 0f 29 85 b0 03 00 00     	movaps	xmmword ptr [rbp + 0x3b0], xmm8
14095a01b: 0f 29 bd a0 03 00 00        	movaps	xmmword ptr [rbp + 0x3a0], xmm7
14095a022: 0f 29 b5 90 03 00 00        	movaps	xmmword ptr [rbp + 0x390], xmm6
14095a029: 48 c7 85 88 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x388], -0x2
14095a034: 4c 89 8d d8 01 00 00        	mov	qword ptr [rbp + 0x1d8], r9
14095a03b: 4c 89 85 a8 02 00 00        	mov	qword ptr [rbp + 0x2a8], r8
14095a042: 48 89 d6                    	mov	rsi, rdx
14095a045: 49 89 cd                    	mov	r13, rcx
14095a048: e8 b3 c9 2b 00              	call	0x140c16a00 <.text+0xc15a00>
14095a04d: 84 c0                       	test	al, al
14095a04f: 74 5f                       	je	0x14095a0b0 <.text+0x9590b0>
14095a051: 41 89 c7                    	mov	r15d, eax
14095a054: bf 41 00 00 00              	mov	edi, 0x41
14095a059: b9 41 00 00 00              	mov	ecx, 0x41
14095a05e: e8 0d f6 ff ff              	call	0x140959670 <.text+0x958670>
14095a063: 48 8d 1d 7e ab 15 02        	lea	rbx, [rip + 0x215ab7e]  # 0x142ab4be8
14095a06a: 48 83 f8 01                 	cmp	rax, 0x1
14095a06e: 0f 85 a7 00 00 00           	jne	0x14095a11b <.text+0x95911b>
14095a074: 49 89 d1                    	mov	r9, rdx
14095a077: 48 85 d2                    	test	rdx, rdx
14095a07a: 74 3d                       	je	0x14095a0b9 <.text+0x9590b9>
14095a07c: 49 83 f9 41                 	cmp	r9, 0x41
14095a080: 73 35                       	jae	0x14095a0b7 <.text+0x9590b7>
14095a082: 48 8d 05 5f ab 15 02        	lea	rax, [rip + 0x215ab5f]  # 0x142ab4be8
14095a089: 41 80 3c 01 bf              	cmp	byte ptr [r9 + rax], -0x41
14095a08e: 7f 29                       	jg	0x14095a0b9 <.text+0x9590b9>
14095a090: 48 8d 05 19 ac 15 02        	lea	rax, [rip + 0x215ac19]  # 0x142ab4cb0
14095a097: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14095a09c: 48 8d 0d 45 ab 15 02        	lea	rcx, [rip + 0x215ab45]  # 0x142ab4be8
14095a0a3: ba 41 00 00 00              	mov	edx, 0x41
14095a0a8: 45 31 c0                    	xor	r8d, r8d
14095a0ab: e8 90 f8 f7 01              	call	0x1428d9940 <.text+0x28d8940>
14095a0b0: 31 d2                       	xor	edx, edx
14095a0b2: e9 c6 01 00 00              	jmp	0x14095a27d <.text+0x95927d>
14095a0b7: 75 d7                       	jne	0x14095a090 <.text+0x959090>
14095a0b9: 4c 89 c9                    	mov	rcx, r9
14095a0bc: e8 af f5 ff ff              	call	0x140959670 <.text+0x958670>
14095a0c1: 48 83 f8 01                 	cmp	rax, 0x1
14095a0c5: 75 54                       	jne	0x14095a11b <.text+0x95911b>
14095a0c7: 49 89 d0                    	mov	r8, rdx
14095a0ca: 49 83 c0 02                 	add	r8, 0x2
14095a0ce: 74 39                       	je	0x14095a109 <.text+0x959109>
14095a0d0: 49 83 f8 41                 	cmp	r8, 0x41
14095a0d4: 73 31                       	jae	0x14095a107 <.text+0x959107>
14095a0d6: 48 8d 05 0b ab 15 02        	lea	rax, [rip + 0x215ab0b]  # 0x142ab4be8
14095a0dd: 41 80 3c 00 bf              	cmp	byte ptr [r8 + rax], -0x41
14095a0e2: 7f 25                       	jg	0x14095a109 <.text+0x959109>
14095a0e4: 48 8d 05 dd ab 15 02        	lea	rax, [rip + 0x215abdd]  # 0x142ab4cc8
14095a0eb: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14095a0f0: 48 8d 0d f1 aa 15 02        	lea	rcx, [rip + 0x215aaf1]  # 0x142ab4be8
14095a0f7: ba 41 00 00 00              	mov	edx, 0x41
14095a0fc: 41 b9 41 00 00 00           	mov	r9d, 0x41
14095a102: e8 39 f8 f7 01              	call	0x1428d9940 <.text+0x28d8940>
14095a107: 75 db                       	jne	0x14095a0e4 <.text+0x9590e4>
14095a109: bf 3f 00 00 00              	mov	edi, 0x3f
14095a10e: 48 29 d7                    	sub	rdi, rdx
14095a111: 48 8d 1d d0 aa 15 02        	lea	rbx, [rip + 0x215aad0]  # 0x142ab4be8
14095a118: 4c 01 c3                    	add	rbx, r8
14095a11b: 4c 8d 05 af ac 15 02        	lea	r8, [rip + 0x215acaf]   # 0x142ab4dd1
14095a122: 4c 8d 25 6f ac 15 02        	lea	r12, [rip + 0x215ac6f]  # 0x142ab4d98
14095a129: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
14095a130: 41 0f be 40 ff              	movsx	eax, byte ptr [r8 - 0x1]
14095a135: 85 c0                       	test	eax, eax
14095a137: 78 17                       	js	0x14095a150 <.text+0x959150>
14095a139: 49 ff c8                    	dec	r8
14095a13c: 83 f8 5c                    	cmp	eax, 0x5c
14095a13f: 75 63                       	jne	0x14095a1a4 <.text+0x9591a4>
14095a141: eb 73                       	jmp	0x14095a1b6 <.text+0x9591b6>
14095a143: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14095a150: 41 0f b6 48 fe              	movzx	ecx, byte ptr [r8 - 0x2]
14095a155: 80 f9 c0                    	cmp	cl, -0x40
14095a158: 7d 23                       	jge	0x14095a17d <.text+0x95917d>
14095a15a: 41 0f b6 50 fd              	movzx	edx, byte ptr [r8 - 0x3]
14095a15f: 80 fa c0                    	cmp	dl, -0x40
14095a162: 7d 22                       	jge	0x14095a186 <.text+0x959186>
14095a164: 45 0f b6 48 fc              	movzx	r9d, byte ptr [r8 - 0x4]
14095a169: 49 83 c0 fc                 	add	r8, -0x4
14095a16d: 41 83 e1 07                 	and	r9d, 0x7
14095a171: 41 c1 e1 06                 	shl	r9d, 0x6
14095a175: 83 e2 3f                    	and	edx, 0x3f
14095a178: 44 09 ca                    	or	edx, r9d
14095a17b: eb 10                       	jmp	0x14095a18d <.text+0x95918d>
14095a17d: 49 83 c0 fe                 	add	r8, -0x2
14095a181: 83 e1 1f                    	and	ecx, 0x1f
14095a184: eb 0f                       	jmp	0x14095a195 <.text+0x959195>
14095a186: 49 83 c0 fd                 	add	r8, -0x3
14095a18a: 83 e2 0f                    	and	edx, 0xf
14095a18d: c1 e2 06                    	shl	edx, 0x6
14095a190: 83 e1 3f                    	and	ecx, 0x3f
14095a193: 09 d1                       	or	ecx, edx
14095a195: c1 e1 06                    	shl	ecx, 0x6
14095a198: 24 3f                       	and	al, 0x3f
14095a19a: 0f b6 c0                    	movzx	eax, al
14095a19d: 09 c8                       	or	eax, ecx
14095a19f: 83 f8 5c                    	cmp	eax, 0x5c
14095a1a2: 74 12                       	je	0x14095a1b6 <.text+0x9591b6>
14095a1a4: 83 f8 2f                    	cmp	eax, 0x2f
14095a1a7: 74 0d                       	je	0x14095a1b6 <.text+0x9591b6>
14095a1a9: 4d 39 e0                    	cmp	r8, r12
14095a1ac: 75 82                       	jne	0x14095a130 <.text+0x959130>
14095a1ae: 41 be 39 00 00 00           	mov	r14d, 0x39
14095a1b4: eb 4d                       	jmp	0x14095a203 <.text+0x959203>
14095a1b6: 4c 8d 25 db ab 15 02        	lea	r12, [rip + 0x215abdb]  # 0x142ab4d98
14095a1bd: 4d 29 e0                    	sub	r8, r12
14095a1c0: 49 ff c0                    	inc	r8
14095a1c3: 74 32                       	je	0x14095a1f7 <.text+0x9591f7>
14095a1c5: 49 83 f8 39                 	cmp	r8, 0x39
14095a1c9: 73 2a                       	jae	0x14095a1f5 <.text+0x9591f5>
14095a1cb: 43 80 3c 20 bf              	cmp	byte ptr [r8 + r12], -0x41
14095a1d0: 7f 25                       	jg	0x14095a1f7 <.text+0x9591f7>
14095a1d2: 48 8d 05 b7 aa 15 02        	lea	rax, [rip + 0x215aab7]  # 0x142ab4c90
14095a1d9: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14095a1de: 48 8d 0d b3 ab 15 02        	lea	rcx, [rip + 0x215abb3]  # 0x142ab4d98
14095a1e5: ba 39 00 00 00              	mov	edx, 0x39
14095a1ea: 41 b9 39 00 00 00           	mov	r9d, 0x39
14095a1f0: e8 4b f7 f7 01              	call	0x1428d9940 <.text+0x28d8940>
14095a1f5: 75 db                       	jne	0x14095a1d2 <.text+0x9591d2>
14095a1f7: 41 be 39 00 00 00           	mov	r14d, 0x39
14095a1fd: 4d 29 c6                    	sub	r14, r8
14095a200: 4d 01 c4                    	add	r12, r8
14095a203: e8 28 df 2b 00              	call	0x140c18130 <.text+0xc17130>
14095a208: 48 89 c1                    	mov	rcx, rax
14095a20b: 48 8b 00                    	mov	rax, qword ptr [rax]
14095a20e: 48 83 f8 01                 	cmp	rax, 0x1
14095a212: 0f 85 aa 07 00 00           	jne	0x14095a9c2 <.text+0x9599c2>
14095a218: 48 83 c1 08                 	add	rcx, 0x8
14095a21c: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
14095a220: 0f 85 39 1d 00 00           	jne	0x14095bf5f <.text+0x95af5f>
14095a226: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
14095a22d: 48 89 8d 78 03 00 00        	mov	qword ptr [rbp + 0x378], rcx
14095a234: 48 83 c1 08                 	add	rcx, 0x8
14095a238: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
14095a23d: 48 c7 44 24 30 00 00 00 00  	mov	qword ptr [rsp + 0x30], 0x0
14095a246: 48 c7 44 24 28 01 00 00 00  	mov	qword ptr [rsp + 0x28], 0x1
14095a24f: 48 89 da                    	mov	rdx, rbx
14095a252: 49 89 f8                    	mov	r8, rdi
14095a255: 4d 89 e1                    	mov	r9, r12
14095a258: e8 e3 ca 2b 00              	call	0x140c16d40 <.text+0xc15d40>
14095a25d: 48 8b 8d 78 03 00 00        	mov	rcx, qword ptr [rbp + 0x378]
14095a264: 48 ff 01                    	inc	qword ptr [rcx]
14095a267: 48 89 85 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rax
14095a26e: 48 89 85 88 01 00 00        	mov	qword ptr [rbp + 0x188], rax
14095a275: ba 01 00 00 00              	mov	edx, 0x1
14095a27a: 44 89 f8                    	mov	eax, r15d
14095a27d: 48 8b 8d d8 04 00 00        	mov	rcx, qword ptr [rbp + 0x4d8]
14095a284: 48 89 95 80 01 00 00        	mov	qword ptr [rbp + 0x180], rdx
14095a28b: 48 8b 11                    	mov	rdx, qword ptr [rcx]
14095a28e: 48 8b 0a                    	mov	rcx, qword ptr [rdx]
14095a291: 48 83 f9 02                 	cmp	rcx, 0x2
14095a295: 74 13                       	je	0x14095a2aa <.text+0x9592aa>
14095a297: 83 f9 0a                    	cmp	ecx, 0xa
14095a29a: 0f 85 b5 04 00 00           	jne	0x14095a755 <.text+0x959755>
14095a2a0: 80 7a 08 00                 	cmp	byte ptr [rdx + 0x8], 0x0
14095a2a4: 0f 85 ab 04 00 00           	jne	0x14095a755 <.text+0x959755>
14095a2aa: 4c 89 ad f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], r13
14095a2b1: 88 85 78 02 00 00           	mov	byte ptr [rbp + 0x278], al
14095a2b7: 4c 8b 85 d0 04 00 00        	mov	r8, qword ptr [rbp + 0x4d0]
14095a2be: 48 8b 95 c8 04 00 00        	mov	rdx, qword ptr [rbp + 0x4c8]
14095a2c5: 4c 8b 26                    	mov	r12, qword ptr [rsi]
14095a2c8: 4c 8b 4e 08                 	mov	r9, qword ptr [rsi + 0x8]
14095a2cc: 4d 8b 1c 24                 	mov	r11, qword ptr [r12]
14095a2d0: 45 31 d2                    	xor	r10d, r10d
14095a2d3: 4c 89 d8                    	mov	rax, r11
14095a2d6: 49 2b 41 18                 	sub	rax, qword ptr [r9 + 0x18]
14095a2da: 49 0f 42 c2                 	cmovb	rax, r10
14095a2de: 4d 2b 59 38                 	sub	r11, qword ptr [r9 + 0x38]
14095a2e2: 4d 0f 42 da                 	cmovb	r11, r10
14095a2e6: 48 8d 0c 40                 	lea	rcx, [rax + 2*rax]
14095a2ea: 48 c1 e1 03                 	shl	rcx, 0x3
14095a2ee: 49 03 49 08                 	add	rcx, qword ptr [r9 + 0x8]
14095a2f2: 49 8b 71 10                 	mov	rsi, qword ptr [r9 + 0x10]
14095a2f6: 48 29 c6                    	sub	rsi, rax
14095a2f9: 49 0f 42 f2                 	cmovb	rsi, r10
14095a2fd: 49 8b 79 30                 	mov	rdi, qword ptr [r9 + 0x30]
14095a301: bb 08 00 00 00              	mov	ebx, 0x8
14095a306: 48 0f 42 cb                 	cmovb	rcx, rbx
14095a30a: 4b 8d 04 5b                 	lea	rax, [r11 + 2*r11]
14095a30e: 48 c1 e0 03                 	shl	rax, 0x3
14095a312: 49 03 41 28                 	add	rax, qword ptr [r9 + 0x28]
14095a316: 4c 29 df                    	sub	rdi, r11
14095a319: 49 0f 42 fa                 	cmovb	rdi, r10
14095a31d: 48 0f 42 c3                 	cmovb	rax, rbx
14095a321: 4d 8b 49 40                 	mov	r9, qword ptr [r9 + 0x40]
14095a325: 49 29 f9                    	sub	r9, rdi
14095a328: 49 29 f1                    	sub	r9, rsi
14095a32b: 4d 89 0c 24                 	mov	qword ptr [r12], r9
14095a32f: 4c 8d 0c 76                 	lea	r9, [rsi + 2*rsi]
14095a333: 4e 8d 2c c9                 	lea	r13, [rcx + 8*r9]
14095a337: 4c 8d 0c 7f                 	lea	r9, [rdi + 2*rdi]
14095a33b: 4a 8d 3c c8                 	lea	rdi, [rax + 8*r9]
14095a33f: 4d 8b 00                    	mov	r8, qword ptr [r8]
14095a342: 4c 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], r8
14095a349: 48 8b 12                    	mov	rdx, qword ptr [rdx]
14095a34c: 48 89 95 68 03 00 00        	mov	qword ptr [rbp + 0x368], rdx
14095a353: 0f 57 f6                    	xorps	xmm6, xmm6
14095a356: 4c 8d b5 10 01 00 00        	lea	r14, [rbp + 0x110]
14095a35d: 48 89 bd 58 03 00 00        	mov	qword ptr [rbp + 0x358], rdi
14095a364: eb 21                       	jmp	0x14095a387 <.text+0x959387>
14095a366: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
14095a370: 4d 89 f4                    	mov	r12, r14
14095a373: 48 8b bd 58 03 00 00        	mov	rdi, qword ptr [rbp + 0x358]
14095a37a: 4c 8d b5 10 01 00 00        	lea	r14, [rbp + 0x110]
14095a381: 48 89 f0                    	mov	rax, rsi
14095a384: 48 89 d9                    	mov	rcx, rbx
14095a387: 48 85 c9                    	test	rcx, rcx
14095a38a: 74 14                       	je	0x14095a3a0 <.text+0x9593a0>
14095a38c: 4c 39 e9                    	cmp	rcx, r13
14095a38f: 74 0f                       	je	0x14095a3a0 <.text+0x9593a0>
14095a391: 48 8d 59 18                 	lea	rbx, [rcx + 0x18]
14095a395: 48 89 c6                    	mov	rsi, rax
14095a398: 48 89 ca                    	mov	rdx, rcx
14095a39b: eb 21                       	jmp	0x14095a3be <.text+0x9593be>
14095a39d: 0f 1f 00                    	nop	dword ptr [rax]
14095a3a0: 48 85 c0                    	test	rax, rax
14095a3a3: 0f 84 d2 01 00 00           	je	0x14095a57b <.text+0x95957b>
14095a3a9: 48 39 f8                    	cmp	rax, rdi
14095a3ac: 0f 84 c9 01 00 00           	je	0x14095a57b <.text+0x95957b>
14095a3b2: 48 8d 70 18                 	lea	rsi, [rax + 0x18]
14095a3b6: 31 db                       	xor	ebx, ebx
14095a3b8: 48 89 c2                    	mov	rdx, rax
14095a3bb: 48 89 f0                    	mov	rax, rsi
14095a3be: 49 ff 04 24                 	inc	qword ptr [r12]
14095a3c2: f3 0f 10 42 10              	movss	xmm0, dword ptr [rdx + 0x10]
14095a3c7: 0f 2e c6                    	ucomiss	xmm0, xmm6
14095a3ca: 48 89 d9                    	mov	rcx, rbx
14095a3cd: 76 b8                       	jbe	0x14095a387 <.text+0x959387>
14095a3cf: 44 8b 42 08                 	mov	r8d, dword ptr [rdx + 0x8]
14095a3d3: 8b 42 0c                    	mov	eax, dword ptr [rdx + 0xc]
14095a3d6: 80 7a 15 00                 	cmp	byte ptr [rdx + 0x15], 0x0
14095a3da: f3 0f 11 85 20 02 00 00     	movss	dword ptr [rbp + 0x220], xmm0
14095a3e2: 0f b6 4a 14                 	movzx	ecx, byte ptr [rdx + 0x14]
14095a3e6: 88 8d 10 01 00 00           	mov	byte ptr [rbp + 0x110], cl
14095a3ec: 4c 89 b5 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], r14
14095a3f3: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
14095a3fa: 48 89 8d c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rcx
14095a401: 48 8b 8d a0 04 00 00        	mov	rcx, qword ptr [rbp + 0x4a0]
14095a408: 48 89 8d d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rcx
14095a40f: 0f 84 db 00 00 00           	je	0x14095a4f0 <.text+0x9594f0>
14095a415: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095a41c: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095a423: e8 98 44 fa ff              	call	0x1408fe8c0 <.text+0x8fd8c0>
14095a428: 4d 89 e6                    	mov	r14, r12
14095a42b: 48 8b 85 60 03 00 00        	mov	rax, qword ptr [rbp + 0x360]
14095a432: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
14095a436: 48 85 c0                    	test	rax, rax
14095a439: 0f 84 11 01 00 00           	je	0x14095a550 <.text+0x959550>
14095a43f: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
14095a446: 4c 8b 79 08                 	mov	r15, qword ptr [rcx + 0x8]
14095a44a: 48 69 f8 50 01 00 00        	imul	rdi, rax, 0x150
14095a451: 4c 01 ff                    	add	rdi, r15
14095a454: 48 8b 85 08 03 00 00        	mov	rax, qword ptr [rbp + 0x308]
14095a45b: 48 89 85 78 03 00 00        	mov	qword ptr [rbp + 0x378], rax
14095a462: 4c 8b a5 10 03 00 00        	mov	r12, qword ptr [rbp + 0x310]
14095a469: 49 c1 e4 02                 	shl	r12, 0x2
14095a46d: eb 16                       	jmp	0x14095a485 <.text+0x959485>
14095a46f: 90                          	nop
14095a470: 48 8d 8d c0 02 00 00        	lea	rcx, [rbp + 0x2c0]
14095a477: e8 24 58 00 00              	call	0x14095fca0 <.text+0x95eca0>
14095a47c: 49 39 ff                    	cmp	r15, rdi
14095a47f: 0f 84 cb 00 00 00           	je	0x14095a550 <.text+0x959550>
14095a485: 4c 89 fa                    	mov	rdx, r15
14095a488: 49 81 c7 50 01 00 00        	add	r15, 0x150
14095a48f: 8b 82 28 01 00 00           	mov	eax, dword ptr [rdx + 0x128]
14095a495: 31 c9                       	xor	ecx, ecx
14095a497: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
14095a4a0: 49 39 cc                    	cmp	r12, rcx
14095a4a3: 74 1b                       	je	0x14095a4c0 <.text+0x9594c0>
14095a4a5: 4c 8d 41 04                 	lea	r8, [rcx + 0x4]
14095a4a9: 4c 8b 8d 78 03 00 00        	mov	r9, qword ptr [rbp + 0x378]
14095a4b0: 41 3b 04 09                 	cmp	eax, dword ptr [r9 + rcx]
14095a4b4: 4c 89 c1                    	mov	rcx, r8
14095a4b7: 75 e7                       	jne	0x14095a4a0 <.text+0x9594a0>
14095a4b9: eb b5                       	jmp	0x14095a470 <.text+0x959470>
14095a4bb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
14095a4c0: 8b 82 2c 01 00 00           	mov	eax, dword ptr [rdx + 0x12c]
14095a4c6: 31 c9                       	xor	ecx, ecx
14095a4c8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
14095a4d0: 49 39 cc                    	cmp	r12, rcx
14095a4d3: 74 a7                       	je	0x14095a47c <.text+0x95947c>
14095a4d5: 4c 8d 41 04                 	lea	r8, [rcx + 0x4]
14095a4d9: 4c 8b 8d 78 03 00 00        	mov	r9, qword ptr [rbp + 0x378]
14095a4e0: 41 3b 04 09                 	cmp	eax, dword ptr [r9 + rcx]
14095a4e4: 4c 89 c1                    	mov	rcx, r8
14095a4e7: 75 e7                       	jne	0x14095a4d0 <.text+0x9594d0>
14095a4e9: eb 85                       	jmp	0x14095a470 <.text+0x959470>
14095a4eb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
14095a4f0: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
14095a4f7: 48 8b 51 08                 	mov	rdx, qword ptr [rcx + 0x8]
14095a4fb: 49 c7 c1 b0 fe ff ff        	mov	r9, -0x150
14095a502: 4c 01 ca                    	add	rdx, r9
14095a505: 48 69 49 10 50 01 00 00     	imul	rcx, qword ptr [rcx + 0x10], 0x150
14095a50d: 0f 1f 00                    	nop	dword ptr [rax]
14095a510: 48 85 c9                    	test	rcx, rcx
14095a513: 0f 84 68 fe ff ff           	je	0x14095a381 <.text+0x959381>
14095a519: 49 89 d1                    	mov	r9, rdx
14095a51c: 48 81 c2 50 01 00 00        	add	rdx, 0x150
14095a523: 48 81 c1 b0 fe ff ff        	add	rcx, -0x150
14095a52a: 45 3b 81 78 02 00 00        	cmp	r8d, dword ptr [r9 + 0x278]
14095a531: 75 dd                       	jne	0x14095a510 <.text+0x959510>
14095a533: 41 3b 81 7c 02 00 00        	cmp	eax, dword ptr [r9 + 0x27c]
14095a53a: 75 d4                       	jne	0x14095a510 <.text+0x959510>
14095a53c: 48 8d 8d c0 02 00 00        	lea	rcx, [rbp + 0x2c0]
14095a543: e8 58 57 00 00              	call	0x14095fca0 <.text+0x95eca0>
14095a548: e9 34 fe ff ff              	jmp	0x14095a381 <.text+0x959381>
14095a54d: 0f 1f 00                    	nop	dword ptr [rax]
14095a550: 48 8b 95 00 03 00 00        	mov	rdx, qword ptr [rbp + 0x300]
14095a557: 48 85 d2                    	test	rdx, rdx
14095a55a: 0f 84 10 fe ff ff           	je	0x14095a370 <.text+0x959370>
14095a560: 48 8b 8d 08 03 00 00        	mov	rcx, qword ptr [rbp + 0x308]
14095a567: 48 c1 e2 02                 	shl	rdx, 0x2
14095a56b: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095a571: e8 aa 96 cb ff              	call	0x140613c20 <.text+0x612c20>
14095a576: e9 f5 fd ff ff              	jmp	0x14095a370 <.text+0x959370>
14095a57b: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
14095a582: 48 8b 01                    	mov	rax, qword ptr [rcx]
14095a585: 4c 8b 51 08                 	mov	r10, qword ptr [rcx + 0x8]
14095a589: 48 8b 08                    	mov	rcx, qword ptr [rax]
14095a58c: 31 f6                       	xor	esi, esi
14095a58e: 49 89 c8                    	mov	r8, rcx
14095a591: 4d 2b 42 18                 	sub	r8, qword ptr [r10 + 0x18]
14095a595: 4c 0f 42 c6                 	cmovb	r8, rsi
14095a599: 49 2b 4a 38                 	sub	rcx, qword ptr [r10 + 0x38]
14095a59d: 48 0f 42 ce                 	cmovb	rcx, rsi
14095a5a1: 4d 8b 5a 08                 	mov	r11, qword ptr [r10 + 0x8]
14095a5a5: 4d 8b 4a 30                 	mov	r9, qword ptr [r10 + 0x30]
14095a5a9: 4c 89 ca                    	mov	rdx, r9
14095a5ac: 48 29 ca                    	sub	rdx, rcx
14095a5af: 48 0f 42 d6                 	cmovb	rdx, rsi
14095a5b3: 49 8b 5a 10                 	mov	rbx, qword ptr [r10 + 0x10]
14095a5b7: 41 0f 96 c6                 	setbe	r14b
14095a5bb: 48 89 df                    	mov	rdi, rbx
14095a5be: 4c 29 c7                    	sub	rdi, r8
14095a5c1: 48 0f 42 fe                 	cmovb	rdi, rsi
14095a5c5: 49 8b 72 28                 	mov	rsi, qword ptr [r10 + 0x28]
14095a5c9: 4d 8b 52 40                 	mov	r10, qword ptr [r10 + 0x40]
14095a5cd: 41 0f 96 c7                 	setbe	r15b
14095a5d1: 4d 89 d5                    	mov	r13, r10
14095a5d4: 49 29 d5                    	sub	r13, rdx
14095a5d7: 49 29 fd                    	sub	r13, rdi
14095a5da: 4c 89 28                    	mov	qword ptr [rax], r13
14095a5dd: 45 84 f7                    	test	r15b, r14b
14095a5e0: 74 54                       	je	0x14095a636 <.text+0x959636>
14095a5e2: 48 83 bd 80 01 00 00 00     	cmp	qword ptr [rbp + 0x180], 0x0
14095a5ea: 0f 84 b4 01 00 00           	je	0x14095a7a4 <.text+0x9597a4>
14095a5f0: e8 3b db 2b 00              	call	0x140c18130 <.text+0xc17130>
14095a5f5: 48 89 c1                    	mov	rcx, rax
14095a5f8: 48 8b 00                    	mov	rax, qword ptr [rax]
14095a5fb: 48 83 f8 01                 	cmp	rax, 0x1
14095a5ff: 0f 85 f1 03 00 00           	jne	0x14095a9f6 <.text+0x9599f6>
14095a605: 48 83 c1 08                 	add	rcx, 0x8
14095a609: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
14095a60d: 0f 85 4c 19 00 00           	jne	0x14095bf5f <.text+0x95af5f>
14095a613: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
14095a61a: 48 89 8d 78 03 00 00        	mov	qword ptr [rbp + 0x378], rcx
14095a621: 48 83 c1 08                 	add	rcx, 0x8
14095a625: 48 8b 95 88 01 00 00        	mov	rdx, qword ptr [rbp + 0x188]
14095a62c: e8 bf ca 2b 00              	call	0x140c170f0 <.text+0xc160f0>
14095a631: e9 64 01 00 00              	jmp	0x14095a79a <.text+0x95979a>
14095a636: 45 31 f6                    	xor	r14d, r14d
14095a639: 4c 39 c3                    	cmp	rbx, r8
14095a63c: 4f 8d 04 40                 	lea	r8, [r8 + 2*r8]
14095a640: 4f 8d 04 c3                 	lea	r8, [r11 + 8*r8]
14095a644: 41 bb 08 00 00 00           	mov	r11d, 0x8
14095a64a: 4d 0f 42 c3                 	cmovb	r8, r11
14095a64e: 48 8d 3c 7f                 	lea	rdi, [rdi + 2*rdi]
14095a652: 4d 8d 44 f8 e8              	lea	r8, [r8 + 8*rdi - 0x18]
14095a657: 4d 0f 47 f0                 	cmova	r14, r8
14095a65b: 49 39 c9                    	cmp	r9, rcx
14095a65e: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
14095a662: 48 8d 0c ce                 	lea	rcx, [rsi + 8*rcx]
14095a666: 49 0f 42 cb                 	cmovb	rcx, r11
14095a66a: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
14095a66e: 48 8d 4c d1 e8              	lea	rcx, [rcx + 8*rdx - 0x18]
14095a673: 49 0f 46 ce                 	cmovbe	rcx, r14
14095a677: 4c 89 10                    	mov	qword ptr [rax], r10
14095a67a: 48 8b 79 08                 	mov	rdi, qword ptr [rcx + 0x8]
14095a67e: 48 89 bd b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rdi
14095a685: 0f b6 71 14                 	movzx	esi, byte ptr [rcx + 0x14]
14095a689: f3 44 0f 10 59 10           	movss	xmm11, dword ptr [rcx + 0x10]
14095a68f: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
14095a696: 48 8b 18                    	mov	rbx, qword ptr [rax]
14095a699: 48 89 bd 00 03 00 00        	mov	qword ptr [rbp + 0x300], rdi
14095a6a0: 48 83 7b 18 00              	cmp	qword ptr [rbx + 0x18], 0x0
14095a6a5: 0f 84 a3 00 00 00           	je	0x14095a74e <.text+0x95974e>
14095a6ab: 48 8d 4b 20                 	lea	rcx, [rbx + 0x20]
14095a6af: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095a6b6: e8 b5 66 c5 ff              	call	0x1405b0d70 <.text+0x5afd70>
14095a6bb: 48 8b 0b                    	mov	rcx, qword ptr [rbx]
14095a6be: 48 8b 53 08                 	mov	rdx, qword ptr [rbx + 0x8]
14095a6c2: 49 89 d0                    	mov	r8, rdx
14095a6c5: 49 21 c0                    	and	r8, rax
14095a6c8: 48 c1 e8 39                 	shr	rax, 0x39
14095a6cc: 66 0f 6e c0                 	movd	xmm0, eax
14095a6d0: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14095a6d4: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14095a6d9: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
14095a6de: 48 8d 81 58 fd ff ff        	lea	rax, [rcx - 0x2a8]
14095a6e5: 45 31 c9                    	xor	r9d, r9d
14095a6e8: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
14095a6ec: f3 42 0f 6f 14 01           	movdqu	xmm2, xmmword ptr [rcx + r8]
14095a6f2: 66 0f 6f da                 	movdqa	xmm3, xmm2
14095a6f6: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
14095a6fa: 66 44 0f d7 d3              	pmovmskb	r10d, xmm3
14095a6ff: 45 85 d2                    	test	r10d, r10d
14095a702: 74 2c                       	je	0x14095a730 <.text+0x959730>
14095a704: f3 45 0f bc da              	tzcnt	r11d, r10d
14095a709: 4d 01 c3                    	add	r11, r8
14095a70c: 49 21 d3                    	and	r11, rdx
14095a70f: 49 f7 db                    	neg	r11
14095a712: 4d 69 db a8 02 00 00        	imul	r11, r11, 0x2a8
14095a719: 4a 39 3c 18                 	cmp	qword ptr [rax + r11], rdi
14095a71d: 0f 84 e3 00 00 00           	je	0x14095a806 <.text+0x959806>
14095a723: 45 8d 5a ff                 	lea	r11d, [r10 - 0x1]
14095a727: 66 45 21 d3                 	and	r11w, r10w
14095a72b: 45 89 da                    	mov	r10d, r11d
14095a72e: 75 d4                       	jne	0x14095a704 <.text+0x959704>
14095a730: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
14095a734: 66 44 0f d7 d2              	pmovmskb	r10d, xmm2
14095a739: 45 85 d2                    	test	r10d, r10d
14095a73c: 75 10                       	jne	0x14095a74e <.text+0x95974e>
14095a73e: 4d 01 c8                    	add	r8, r9
14095a741: 49 83 c0 10                 	add	r8, 0x10
14095a745: 49 83 c1 10                 	add	r9, 0x10
14095a749: 49 21 d0                    	and	r8, rdx
14095a74c: eb 9e                       	jmp	0x14095a6ec <.text+0x9596ec>
14095a74e: 0f b6 85 78 02 00 00        	movzx	eax, byte ptr [rbp + 0x278]
14095a755: 84 c0                       	test	al, al
14095a757: 74 4b                       	je	0x14095a7a4 <.text+0x9597a4>
14095a759: e8 d2 d9 2b 00              	call	0x140c18130 <.text+0xc17130>
14095a75e: 48 89 c1                    	mov	rcx, rax
14095a761: 48 8b 00                    	mov	rax, qword ptr [rax]
14095a764: 48 83 f8 01                 	cmp	rax, 0x1
14095a768: 0f 85 6e 02 00 00           	jne	0x14095a9dc <.text+0x9599dc>
14095a76e: 48 83 c1 08                 	add	rcx, 0x8
14095a772: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
14095a776: 0f 85 e3 17 00 00           	jne	0x14095bf5f <.text+0x95af5f>
14095a77c: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
14095a783: 48 89 8d 78 03 00 00        	mov	qword ptr [rbp + 0x378], rcx
14095a78a: 48 83 c1 08                 	add	rcx, 0x8
14095a78e: 48 8b 95 b0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2b0]
14095a795: e8 56 c9 2b 00              	call	0x140c170f0 <.text+0xc160f0>
14095a79a: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095a7a1: 48 ff 00                    	inc	qword ptr [rax]
14095a7a4: 0f 28 b5 90 03 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x390]
14095a7ab: 0f 28 bd a0 03 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x3a0]
14095a7b2: 44 0f 28 85 b0 03 00 00     	movaps	xmm8, xmmword ptr [rbp + 0x3b0]
14095a7ba: 44 0f 28 8d c0 03 00 00     	movaps	xmm9, xmmword ptr [rbp + 0x3c0]
14095a7c2: 44 0f 28 95 d0 03 00 00     	movaps	xmm10, xmmword ptr [rbp + 0x3d0]
14095a7ca: 44 0f 28 9d e0 03 00 00     	movaps	xmm11, xmmword ptr [rbp + 0x3e0]
14095a7d2: 44 0f 28 a5 f0 03 00 00     	movaps	xmm12, xmmword ptr [rbp + 0x3f0]
14095a7da: 44 0f 28 ad 00 04 00 00     	movaps	xmm13, xmmword ptr [rbp + 0x400]
14095a7e2: 44 0f 28 b5 10 04 00 00     	movaps	xmm14, xmmword ptr [rbp + 0x410]
14095a7ea: 44 0f 28 bd 20 04 00 00     	movaps	xmm15, xmmword ptr [rbp + 0x420]
14095a7f2: 48 81 c4 b8 04 00 00        	add	rsp, 0x4b8
14095a7f9: 5b                          	pop	rbx
14095a7fa: 5f                          	pop	rdi
14095a7fb: 5e                          	pop	rsi
14095a7fc: 41 5c                       	pop	r12
14095a7fe: 41 5d                       	pop	r13
14095a800: 41 5e                       	pop	r14
14095a802: 41 5f                       	pop	r15
14095a804: 5d                          	pop	rbp
14095a805: c3                          	ret
14095a806: 4a 83 bc 19 70 fd ff ff 02  	cmp	qword ptr [rcx + r11 - 0x290], 0x2
14095a80f: 0f 82 5d 28 00 00           	jb	0x14095d072 <.text+0x95c072>
14095a815: f3 42 0f 10 84 19 90 fd ff ff       	movss	xmm0, dword ptr [rcx + r11 - 0x270]
14095a81f: 66 0f ef c9                 	pxor	xmm1, xmm1
14095a823: 0f 2e c1                    	ucomiss	xmm0, xmm1
14095a826: 0f 86 46 28 00 00           	jbe	0x14095d072 <.text+0x95c072>
14095a82c: 4e 8d 0c 19                 	lea	r9, [rcx + r11]
14095a830: 49 01 cb                    	add	r11, rcx
14095a833: 49 81 c3 60 fd ff ff        	add	r11, -0x2a0
14095a83a: c7 45 c4 00 00 00 00        	mov	dword ptr [rbp - 0x3c], 0x0
14095a841: 48 8d 45 c4                 	lea	rax, [rbp - 0x3c]
14095a845: 48 89 85 10 01 00 00        	mov	qword ptr [rbp + 0x110], rax
14095a84c: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
14095a853: 48 89 85 18 01 00 00        	mov	qword ptr [rbp + 0x118], rax
14095a85a: 48 8d 85 b8 02 00 00        	lea	rax, [rbp + 0x2b8]
14095a861: 48 89 85 20 01 00 00        	mov	qword ptr [rbp + 0x120], rax
14095a868: 48 8b 85 b8 04 00 00        	mov	rax, qword ptr [rbp + 0x4b8]
14095a86f: 48 89 85 28 01 00 00        	mov	qword ptr [rbp + 0x128], rax
14095a876: 48 8b 85 b0 04 00 00        	mov	rax, qword ptr [rbp + 0x4b0]
14095a87d: 48 89 85 30 01 00 00        	mov	qword ptr [rbp + 0x130], rax
14095a884: 45 31 ff                    	xor	r15d, r15d
14095a887: 85 f6                       	test	esi, esi
14095a889: 4c 89 8d 60 03 00 00        	mov	qword ptr [rbp + 0x360], r9
14095a890: 4c 89 9d 40 01 00 00        	mov	qword ptr [rbp + 0x140], r11
14095a897: 0f 84 29 15 00 00           	je	0x14095bdc6 <.text+0x95adc6>
14095a89d: 83 fe 01                    	cmp	esi, 0x1
14095a8a0: 74 03                       	je	0x14095a8a5 <.text+0x9598a5>
14095a8a2: 41 b7 01                    	mov	r15b, 0x1
14095a8a5: 41 83 79 b8 02              	cmp	dword ptr [r9 - 0x48], 0x2
14095a8aa: 44 89 bd 9c 01 00 00        	mov	dword ptr [rbp + 0x19c], r15d
14095a8b1: 0f 83 91 00 00 00           	jae	0x14095a948 <.text+0x959948>
14095a8b7: 49 8b 81 a0 fd ff ff        	mov	rax, qword ptr [r9 - 0x260]
14095a8be: 49 8b 89 a8 fd ff ff        	mov	rcx, qword ptr [r9 - 0x258]
14095a8c5: 48 8d 14 49                 	lea	rdx, [rcx + 2*rcx]
14095a8c9: 48 8d 14 90                 	lea	rdx, [rax + 4*rdx]
14095a8cd: 4d 8b 81 b8 fd ff ff        	mov	r8, qword ptr [r9 - 0x248]
14095a8d4: 4d 8b 89 c0 fd ff ff        	mov	r9, qword ptr [r9 - 0x240]
14095a8db: 4f 8d 14 88                 	lea	r10, [r8 + 4*r9]
14095a8df: 49 39 c9                    	cmp	r9, rcx
14095a8e2: 4c 0f 43 c9                 	cmovae	r9, rcx
14095a8e6: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095a8ed: 48 89 95 08 03 00 00        	mov	qword ptr [rbp + 0x308], rdx
14095a8f4: 4c 89 85 10 03 00 00        	mov	qword ptr [rbp + 0x310], r8
14095a8fb: 4c 89 95 18 03 00 00        	mov	qword ptr [rbp + 0x318], r10
14095a902: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095a90d: 4c 89 8d 28 03 00 00        	mov	qword ptr [rbp + 0x328], r9
14095a914: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
14095a91b: 4c 89 9d 38 03 00 00        	mov	qword ptr [rbp + 0x338], r11
14095a922: 4c 8d 05 3f a2 15 02        	lea	r8, [rip + 0x215a23f]   # 0x142ab4b68
14095a929: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
14095a930: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095a937: e8 34 30 9a 00              	call	0x1412fd970 <.text+0x12fc970>
14095a93c: 4c 8d ad c0 02 00 00        	lea	r13, [rbp + 0x2c0]
14095a943: e9 fd 00 00 00              	jmp	0x14095aa45 <.text+0x959a45>
14095a948: 49 8b b9 a8 fd ff ff        	mov	rdi, qword ptr [r9 - 0x258]
14095a94f: 45 31 ff                    	xor	r15d, r15d
14095a952: b9 0c 00 00 00              	mov	ecx, 0xc
14095a957: 48 89 f8                    	mov	rax, rdi
14095a95a: 48 f7 e1                    	mul	rcx
14095a95d: 48 89 c3                    	mov	rbx, rax
14095a960: 4c 8d ad c0 02 00 00        	lea	r13, [rbp + 0x2c0]
14095a967: 0f 80 59 28 00 00           	jo	0x14095d1c6 <.text+0x95c1c6>
14095a96d: 48 b8 fc ff ff ff ff ff ff 7f       	movabs	rax, 0x7ffffffffffffffc
14095a977: 48 39 c3                    	cmp	rbx, rax
14095a97a: 0f 87 46 28 00 00           	ja	0x14095d1c6 <.text+0x95c1c6>
14095a980: 48 8b 85 60 03 00 00        	mov	rax, qword ptr [rbp + 0x360]
14095a987: 4c 8b b0 a0 fd ff ff        	mov	r14, qword ptr [rax - 0x260]
14095a98e: 48 85 db                    	test	rbx, rbx
14095a991: 0f 84 83 00 00 00           	je	0x14095aa1a <.text+0x959a1a>
14095a997: 0f b6 05 6b a2 cd 02        	movzx	eax, byte ptr [rip + 0x2cda26b] # 0x143634c09
14095a99e: 41 bf 04 00 00 00           	mov	r15d, 0x4
14095a9a4: ba 04 00 00 00              	mov	edx, 0x4
14095a9a9: 48 89 d9                    	mov	rcx, rbx
14095a9ac: e8 5f 92 cb ff              	call	0x140613c10 <.text+0x612c10>
14095a9b1: 48 85 c0                    	test	rax, rax
14095a9b4: 0f 84 0c 28 00 00           	je	0x14095d1c6 <.text+0x95c1c6>
14095a9ba: 49 89 c4                    	mov	r12, rax
14095a9bd: 48 89 fe                    	mov	rsi, rdi
14095a9c0: eb 60                       	jmp	0x14095aa22 <.text+0x959a22>
14095a9c2: 48 85 c0                    	test	rax, rax
14095a9c5: 75 47                       	jne	0x14095aa0e <.text+0x959a0e>
14095a9c7: 31 d2                       	xor	edx, edx
14095a9c9: e8 02 de f9 01              	call	0x1428f87d0 <.text+0x28f77d0>
14095a9ce: 48 89 c1                    	mov	rcx, rax
14095a9d1: 48 85 c0                    	test	rax, rax
14095a9d4: 0f 85 42 f8 ff ff           	jne	0x14095a21c <.text+0x95921c>
14095a9da: eb 32                       	jmp	0x14095aa0e <.text+0x959a0e>
14095a9dc: 48 85 c0                    	test	rax, rax
14095a9df: 75 2d                       	jne	0x14095aa0e <.text+0x959a0e>
14095a9e1: 31 d2                       	xor	edx, edx
14095a9e3: e8 e8 dd f9 01              	call	0x1428f87d0 <.text+0x28f77d0>
14095a9e8: 48 89 c1                    	mov	rcx, rax
14095a9eb: 48 85 c0                    	test	rax, rax
14095a9ee: 0f 85 7e fd ff ff           	jne	0x14095a772 <.text+0x959772>
14095a9f4: eb 18                       	jmp	0x14095aa0e <.text+0x959a0e>
14095a9f6: 48 85 c0                    	test	rax, rax
14095a9f9: 75 13                       	jne	0x14095aa0e <.text+0x959a0e>
14095a9fb: 31 d2                       	xor	edx, edx
14095a9fd: e8 ce dd f9 01              	call	0x1428f87d0 <.text+0x28f77d0>
14095aa02: 48 89 c1                    	mov	rcx, rax
14095aa05: 48 85 c0                    	test	rax, rax
14095aa08: 0f 85 fb fb ff ff           	jne	0x14095a609 <.text+0x959609>
14095aa0e: 48 8d 0d 8b 00 2e 02        	lea	rcx, [rip + 0x22e008b]  # 0x142c3aaa0
14095aa15: e8 b6 d6 f7 01              	call	0x1428d80d0 <.text+0x28d70d0>
14095aa1a: 41 bc 04 00 00 00           	mov	r12d, 0x4
14095aa20: 31 f6                       	xor	esi, esi
14095aa22: 4c 89 e1                    	mov	rcx, r12
14095aa25: 4c 89 f2                    	mov	rdx, r14
14095aa28: 49 89 d8                    	mov	r8, rbx
14095aa2b: e8 d0 91 fc 01              	call	0x142923c00 <.text+0x2922c00>
14095aa30: 48 89 b5 c0 01 00 00        	mov	qword ptr [rbp + 0x1c0], rsi
14095aa37: 4c 89 a5 c8 01 00 00        	mov	qword ptr [rbp + 0x1c8], r12
14095aa3e: 48 89 bd d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rdi
14095aa45: 48 8b 85 60 03 00 00        	mov	rax, qword ptr [rbp + 0x360]
14095aa4c: f3 0f 10 b0 c8 fd ff ff     	movss	xmm6, dword ptr [rax - 0x238]
14095aa54: 48 8b 85 a0 04 00 00        	mov	rax, qword ptr [rbp + 0x4a0]
14095aa5b: 48 8b 00                    	mov	rax, qword ptr [rax]
14095aa5e: 48 89 85 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rax
14095aa65: 48 8b 85 a8 04 00 00        	mov	rax, qword ptr [rbp + 0x4a8]
14095aa6c: 48 8b 08                    	mov	rcx, qword ptr [rax]
14095aa6f: 48 83 c1 2c                 	add	rcx, 0x2c
14095aa73: 48 8d 85 b8 02 00 00        	lea	rax, [rbp + 0x2b8]
14095aa7a: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
14095aa81: 48 8d 05 38 a4 1d 00        	lea	rax, [rip + 0x1da438]   # 0x140b34ec0 <.text+0xb33ec0>
14095aa88: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095aa8f: 48 89 8d 78 03 00 00        	mov	qword ptr [rbp + 0x378], rcx
14095aa96: 48 89 8d d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rcx
14095aa9d: 48 8d 05 bc 2a 43 00        	lea	rax, [rip + 0x432abc]   # 0x140d8d560 <.text+0xd8c560>
14095aaa4: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
14095aaab: 48 8d 05 56 a3 15 02        	lea	rax, [rip + 0x215a356]  # 0x142ab4e08
14095aab2: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095aab9: 48 c7 85 08 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x308], 0x2
14095aac4: 4c 89 ad 10 03 00 00        	mov	qword ptr [rbp + 0x310], r13
14095aacb: 48 c7 85 18 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x318], 0x2
14095aad6: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095aae1: c6 85 86 03 00 00 01        	mov	byte ptr [rbp + 0x386], 0x1
14095aae8: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
14095aaef: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095aaf6: e8 95 f4 41 00              	call	0x140d79f90 <.text+0xd78f90>
14095aafb: 4c 89 ef                    	mov	rdi, r13
14095aafe: 4c 8b b5 d0 00 00 00        	mov	r14, qword ptr [rbp + 0xd0]
14095ab05: 48 8b 85 d8 00 00 00        	mov	rax, qword ptr [rbp + 0xd8]
14095ab0c: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
14095ab13: 48 8b 9d e0 00 00 00        	mov	rbx, qword ptr [rbp + 0xe0]
14095ab1a: 4c 8b ad c0 01 00 00        	mov	r13, qword ptr [rbp + 0x1c0]
14095ab21: 4c 8b a5 c8 01 00 00        	mov	r12, qword ptr [rbp + 0x1c8]
14095ab28: 48 8b b5 d0 01 00 00        	mov	rsi, qword ptr [rbp + 0x1d0]
14095ab2f: 0f b6 05 d3 a0 cd 02        	movzx	eax, byte ptr [rip + 0x2cda0d3] # 0x143634c09
14095ab36: b9 30 00 00 00              	mov	ecx, 0x30
14095ab3b: ba 08 00 00 00              	mov	edx, 0x8
14095ab40: e8 cb 90 cb ff              	call	0x140613c10 <.text+0x612c10>
14095ab45: 48 85 c0                    	test	rax, rax
14095ab48: 4c 89 b5 58 03 00 00        	mov	qword ptr [rbp + 0x358], r14
14095ab4f: 0f 84 2e 26 00 00           	je	0x14095d183 <.text+0x95c183>
14095ab55: 49 89 c7                    	mov	r15, rax
14095ab58: 48 8b 85 60 03 00 00        	mov	rax, qword ptr [rbp + 0x360]
14095ab5f: 48 05 98 fd ff ff           	add	rax, -0x268
14095ab65: 48 89 85 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rax
14095ab6c: 49 c7 07 01 00 00 00        	mov	qword ptr [r15], 0x1
14095ab73: 49 c7 47 08 01 00 00 00     	mov	qword ptr [r15 + 0x8], 0x1
14095ab7b: 4d 89 6f 10                 	mov	qword ptr [r15 + 0x10], r13
14095ab7f: 4d 89 67 18                 	mov	qword ptr [r15 + 0x18], r12
14095ab83: 49 89 77 20                 	mov	qword ptr [r15 + 0x20], rsi
14095ab87: f3 45 0f 11 5f 28           	movss	dword ptr [r15 + 0x28], xmm11
14095ab8d: f3 41 0f 11 77 2c           	movss	dword ptr [r15 + 0x2c], xmm6
14095ab93: 4c 89 bd 00 03 00 00        	mov	qword ptr [rbp + 0x300], r15
14095ab9a: 48 8d 35 87 a2 15 02        	lea	rsi, [rip + 0x215a287]  # 0x142ab4e28
14095aba1: 48 89 b5 08 03 00 00        	mov	qword ptr [rbp + 0x308], rsi
14095aba8: 48 85 db                    	test	rbx, rbx
14095abab: 0f 88 f4 25 00 00           	js	0x14095d1a5 <.text+0x95c1a5>
14095abb1: 49 89 fc                    	mov	r12, rdi
14095abb4: 74 36                       	je	0x14095abec <.text+0x959bec>
14095abb6: 4c 89 bd 78 02 00 00        	mov	qword ptr [rbp + 0x278], r15
14095abbd: 0f b6 05 45 a0 cd 02        	movzx	eax, byte ptr [rip + 0x2cda045] # 0x143634c09
14095abc4: 41 bf 01 00 00 00           	mov	r15d, 0x1
14095abca: ba 01 00 00 00              	mov	edx, 0x1
14095abcf: 48 89 d9                    	mov	rcx, rbx
14095abd2: e8 39 90 cb ff              	call	0x140613c10 <.text+0x612c10>
14095abd7: 48 85 c0                    	test	rax, rax
14095abda: 0f 84 cf 25 00 00           	je	0x14095d1af <.text+0x95c1af>
14095abe0: 49 89 c6                    	mov	r14, rax
14095abe3: 4c 8b bd 78 02 00 00        	mov	r15, qword ptr [rbp + 0x278]
14095abea: eb 06                       	jmp	0x14095abf2 <.text+0x959bf2>
14095abec: 41 be 01 00 00 00           	mov	r14d, 0x1
14095abf2: 4c 89 f1                    	mov	rcx, r14
14095abf5: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095abfc: 49 89 d8                    	mov	r8, rbx
14095abff: e8 fc 8f fc 01              	call	0x142923c00 <.text+0x2922c00>
14095ac04: 48 89 9d 00 03 00 00        	mov	qword ptr [rbp + 0x300], rbx
14095ac0b: 4c 89 b5 08 03 00 00        	mov	qword ptr [rbp + 0x308], r14
14095ac12: 48 89 9d 10 03 00 00        	mov	qword ptr [rbp + 0x310], rbx
14095ac19: 4c 89 bd 18 03 00 00        	mov	qword ptr [rbp + 0x318], r15
14095ac20: 48 89 b5 20 03 00 00        	mov	qword ptr [rbp + 0x320], rsi
14095ac27: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095ac2e: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
14095ac35: e8 46 1c b1 00              	call	0x14146c880 <.text+0x146b880>
14095ac3a: 48 8b 95 58 03 00 00        	mov	rdx, qword ptr [rbp + 0x358]
14095ac41: 48 85 d2                    	test	rdx, rdx
14095ac44: 48 8d b5 b8 02 00 00        	lea	rsi, [rbp + 0x2b8]
14095ac4b: 48 8b bd 60 03 00 00        	mov	rdi, qword ptr [rbp + 0x360]
14095ac52: 74 12                       	je	0x14095ac66 <.text+0x959c66>
14095ac54: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095ac5a: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
14095ac61: e8 ba 8f cb ff              	call	0x140613c20 <.text+0x612c20>
14095ac66: 8b 47 b8                    	mov	eax, dword ptr [rdi - 0x48]
14095ac69: 83 f8 01                    	cmp	eax, 0x1
14095ac6c: 0f 87 ba 00 00 00           	ja	0x14095ad2c <.text+0x959d2c>
14095ac72: 48 8b 4f bc                 	mov	rcx, qword ptr [rdi - 0x44]
14095ac76: f3 0f 10 47 c4              	movss	xmm0, dword ptr [rdi - 0x3c]
14095ac7b: 8b 57 c8                    	mov	edx, dword ptr [rdi - 0x38]
14095ac7e: 45 31 c0                    	xor	r8d, r8d
14095ac81: 85 c0                       	test	eax, eax
14095ac83: 41 0f 94 c1                 	sete	r9b
14095ac87: 75 08                       	jne	0x14095ac91 <.text+0x959c91>
14095ac89: f3 0f 58 05 ef 85 14 02     	addss	xmm0, dword ptr [rip + 0x21485ef] # 0x142aa3280
14095ac91: f2 0f 10 4f cc              	movsd	xmm1, qword ptr [rdi - 0x34]
14095ac96: 0f 28 15 b3 98 14 02        	movaps	xmm2, xmmword ptr [rip + 0x21498b3] # 0x142aa4550
14095ac9d: 0f 58 d1                    	addps	xmm2, xmm1
14095aca0: 45 88 c8                    	mov	r8b, r9b
14095aca3: 66 41 0f 6e d8              	movd	xmm3, r8d
14095aca8: 66 0f 70 db 50              	pshufd	xmm3, xmm3, 0x50        # xmm3 = xmm3[0,0,1,1]
14095acad: 66 0f 72 f3 1f              	pslld	xmm3, 0x1f
14095acb2: 66 0f 72 e3 1f              	psrad	xmm3, 0x1f
14095acb7: 0f 54 cb                    	andps	xmm1, xmm3
14095acba: 0f 55 da                    	andnps	xmm3, xmm2
14095acbd: 0f 56 d9                    	orps	xmm3, xmm1
14095acc0: 89 85 58 02 00 00           	mov	dword ptr [rbp + 0x258], eax
14095acc6: 48 89 8d 5c 02 00 00        	mov	qword ptr [rbp + 0x25c], rcx
14095accd: f3 0f 11 85 64 02 00 00     	movss	dword ptr [rbp + 0x264], xmm0
14095acd5: 89 95 68 02 00 00           	mov	dword ptr [rbp + 0x268], edx
14095acdb: 0f 13 9d 6c 02 00 00        	movlps	qword ptr [rbp + 0x26c], xmm3
14095ace2: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
14095ace9: 48 8b 08                    	mov	rcx, qword ptr [rax]
14095acec: 48 8b 95 b8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2b8]
14095acf3: c6 85 86 03 00 00 00        	mov	byte ptr [rbp + 0x386], 0x0
14095acfa: 4c 8b 85 d8 01 00 00        	mov	r8, qword ptr [rbp + 0x1d8]
14095ad01: e8 9a 76 13 00              	call	0x140a923a0 <.text+0xa913a0>
14095ad06: 48 85 c0                    	test	rax, rax
14095ad09: 0f 84 11 02 00 00           	je	0x14095af20 <.text+0x959f20>
14095ad0f: f3 0f 10 05 6d ac fc 01     	movss	xmm0, dword ptr [rip + 0x1fcac6d] # 0x142925984
14095ad17: 0f 2e 40 30                 	ucomiss	xmm0, dword ptr [rax + 0x30]
14095ad1b: 0f 93 c1                    	setae	cl
14095ad1e: 80 78 3d 00                 	cmp	byte ptr [rax + 0x3d], 0x0
14095ad22: 0f 95 c0                    	setne	al
14095ad25: 20 c8                       	and	al, cl
14095ad27: e9 f6 01 00 00              	jmp	0x14095af22 <.text+0x959f22>
14095ad2c: 48 8b 87 f0 fd ff ff        	mov	rax, qword ptr [rdi - 0x210]
14095ad33: 48 8b 8f f8 fd ff ff        	mov	rcx, qword ptr [rdi - 0x208]
14095ad3a: 48 8d 14 49                 	lea	rdx, [rcx + 2*rcx]
14095ad3e: 48 8d 14 90                 	lea	rdx, [rax + 4*rdx]
14095ad42: 4c 8b 87 40 fe ff ff        	mov	r8, qword ptr [rdi - 0x1c0]
14095ad49: 4c 8b 8f 48 fe ff ff        	mov	r9, qword ptr [rdi - 0x1b8]
14095ad50: 4f 8d 14 88                 	lea	r10, [r8 + 4*r9]
14095ad54: 49 39 c9                    	cmp	r9, rcx
14095ad57: 4c 0f 43 c9                 	cmovae	r9, rcx
14095ad5b: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095ad62: 48 89 95 08 03 00 00        	mov	qword ptr [rbp + 0x308], rdx
14095ad69: 4c 89 85 10 03 00 00        	mov	qword ptr [rbp + 0x310], r8
14095ad70: 4c 89 95 18 03 00 00        	mov	qword ptr [rbp + 0x318], r10
14095ad77: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095ad82: 4c 89 8d 28 03 00 00        	mov	qword ptr [rbp + 0x328], r9
14095ad89: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
14095ad90: c6 85 86 03 00 00 00        	mov	byte ptr [rbp + 0x386], 0x0
14095ad97: 4c 8d 05 ca 9d 15 02        	lea	r8, [rip + 0x2159dca]   # 0x142ab4b68
14095ad9e: 48 8d 8d 58 02 00 00        	lea	rcx, [rbp + 0x258]
14095ada5: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095adac: e8 4f 1b 9a 00              	call	0x1412fc900 <.text+0x12fb900>
14095adb1: 48 8b 85 a0 04 00 00        	mov	rax, qword ptr [rbp + 0x4a0]
14095adb8: 48 8b 00                    	mov	rax, qword ptr [rax]
14095adbb: 48 89 85 58 03 00 00        	mov	qword ptr [rbp + 0x358], rax
14095adc2: 48 89 b5 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rsi
14095adc9: 48 8d 05 f0 a0 1d 00        	lea	rax, [rip + 0x1da0f0]   # 0x140b34ec0 <.text+0xb33ec0>
14095add0: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095add7: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095adde: 48 89 85 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rax
14095ade5: 48 8d 05 74 27 43 00        	lea	rax, [rip + 0x432774]   # 0x140d8d560 <.text+0xd8c560>
14095adec: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
14095adf3: 48 8d 05 76 a0 15 02        	lea	rax, [rip + 0x215a076]  # 0x142ab4e70
14095adfa: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095ae01: 48 c7 85 08 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x308], 0x2
14095ae0c: 4c 89 a5 10 03 00 00        	mov	qword ptr [rbp + 0x310], r12
14095ae13: 48 c7 85 18 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x318], 0x2
14095ae1e: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095ae29: c6 85 85 03 00 00 01        	mov	byte ptr [rbp + 0x385], 0x1
14095ae30: 48 8d 8d b8 00 00 00        	lea	rcx, [rbp + 0xb8]
14095ae37: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095ae3e: e8 4d f1 41 00              	call	0x140d79f90 <.text+0xd78f90>
14095ae43: 4c 8b a5 b8 00 00 00        	mov	r12, qword ptr [rbp + 0xb8]
14095ae4a: 4c 8b ad c0 00 00 00        	mov	r13, qword ptr [rbp + 0xc0]
14095ae51: 48 8b 9d c8 00 00 00        	mov	rbx, qword ptr [rbp + 0xc8]
14095ae58: 48 8b bd 58 02 00 00        	mov	rdi, qword ptr [rbp + 0x258]
14095ae5f: 4c 8b b5 60 02 00 00        	mov	r14, qword ptr [rbp + 0x260]
14095ae66: 48 8b b5 68 02 00 00        	mov	rsi, qword ptr [rbp + 0x268]
14095ae6d: 0f b6 05 95 9d cd 02        	movzx	eax, byte ptr [rip + 0x2cd9d95] # 0x143634c09
14095ae74: b9 30 00 00 00              	mov	ecx, 0x30
14095ae79: ba 08 00 00 00              	mov	edx, 0x8
14095ae7e: e8 8d 8d cb ff              	call	0x140613c10 <.text+0x612c10>
14095ae83: 48 85 c0                    	test	rax, rax
14095ae86: 4c 89 a5 68 03 00 00        	mov	qword ptr [rbp + 0x368], r12
14095ae8d: 4c 89 ad f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], r13
14095ae94: 0f 84 90 23 00 00           	je	0x14095d22a <.text+0x95c22a>
14095ae9a: 49 89 c7                    	mov	r15, rax
14095ae9d: 48 c7 00 01 00 00 00        	mov	qword ptr [rax], 0x1
14095aea4: 48 c7 40 08 01 00 00 00     	mov	qword ptr [rax + 0x8], 0x1
14095aeac: 48 89 78 10                 	mov	qword ptr [rax + 0x10], rdi
14095aeb0: 4c 89 70 18                 	mov	qword ptr [rax + 0x18], r14
14095aeb4: 48 89 70 20                 	mov	qword ptr [rax + 0x20], rsi
14095aeb8: f3 44 0f 11 58 28           	movss	dword ptr [rax + 0x28], xmm11
14095aebe: f3 0f 11 70 2c              	movss	dword ptr [rax + 0x2c], xmm6
14095aec3: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095aeca: 48 8d 35 bf 9f 15 02        	lea	rsi, [rip + 0x2159fbf]  # 0x142ab4e90
14095aed1: 48 89 b5 08 03 00 00        	mov	qword ptr [rbp + 0x308], rsi
14095aed8: 48 85 db                    	test	rbx, rbx
14095aedb: 0f 88 6b 23 00 00           	js	0x14095d24c <.text+0x95c24c>
14095aee1: 0f 84 cf 02 00 00           	je	0x14095b1b6 <.text+0x95a1b6>
14095aee7: 4c 89 bd b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], r15
14095aeee: 0f b6 05 14 9d cd 02        	movzx	eax, byte ptr [rip + 0x2cd9d14] # 0x143634c09
14095aef5: 41 bf 01 00 00 00           	mov	r15d, 0x1
14095aefb: ba 01 00 00 00              	mov	edx, 0x1
14095af00: 48 89 d9                    	mov	rcx, rbx
14095af03: e8 08 8d cb ff              	call	0x140613c10 <.text+0x612c10>
14095af08: 48 85 c0                    	test	rax, rax
14095af0b: 0f 84 45 23 00 00           	je	0x14095d256 <.text+0x95c256>
14095af11: 49 89 c6                    	mov	r14, rax
14095af14: 4c 8b bd b0 02 00 00        	mov	r15, qword ptr [rbp + 0x2b0]
14095af1b: e9 9c 02 00 00              	jmp	0x14095b1bc <.text+0x95a1bc>
14095af20: 31 c0                       	xor	eax, eax
14095af22: f6 87 68 fe ff ff 01        	test	byte ptr [rdi - 0x198], 0x1
14095af29: 0f 84 ae 22 00 00           	je	0x14095d1dd <.text+0x95c1dd>
14095af2f: 84 c0                       	test	al, al
14095af31: 75 05                       	jne	0x14095af38 <.text+0x959f38>
14095af33: 0f 57 c0                    	xorps	xmm0, xmm0
14095af36: eb 08                       	jmp	0x14095af40 <.text+0x959f40>
14095af38: f3 0f 10 05 30 19 12 02     	movss	xmm0, dword ptr [rip + 0x2121930] # 0x142a7c870
14095af40: f3 0f 58 87 6c fe ff ff     	addss	xmm0, dword ptr [rdi - 0x194]
14095af48: 48 8d bd 5c 02 00 00        	lea	rdi, [rbp + 0x25c]
14095af4f: f3 0f 11 85 a4 02 00 00     	movss	dword ptr [rbp + 0x2a4], xmm0
14095af57: f6 85 58 02 00 00 01        	test	byte ptr [rbp + 0x258], 0x1
14095af5e: 74 18                       	je	0x14095af78 <.text+0x959f78>
14095af60: c6 85 86 03 00 00 00        	mov	byte ptr [rbp + 0x386], 0x0
14095af67: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095af6e: 48 89 fa                    	mov	rdx, rdi
14095af71: e8 7a dd 33 00              	call	0x140c98cf0 <.text+0xc97cf0>
14095af76: eb 1e                       	jmp	0x14095af96 <.text+0x959f96>
14095af78: c6 85 86 03 00 00 00        	mov	byte ptr [rbp + 0x386], 0x0
14095af7f: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095af86: f3 0f 10 15 da 20 07 02     	movss	xmm2, dword ptr [rip + 0x20720da] # 0x1429cd068
14095af8e: 48 89 fa                    	mov	rdx, rdi
14095af91: e8 5a cd 33 00              	call	0x140c97cf0 <.text+0xc96cf0>
14095af96: 48 8b 9d 08 03 00 00        	mov	rbx, qword ptr [rbp + 0x308]
14095af9d: 48 8b 85 10 03 00 00        	mov	rax, qword ptr [rbp + 0x310]
14095afa4: 48 8d 04 c3                 	lea	rax, [rbx + 8*rax]
14095afa8: 48 89 9d c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rbx
14095afaf: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095afb6: 48 8d 85 a4 02 00 00        	lea	rax, [rbp + 0x2a4]
14095afbd: 48 89 85 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rax
14095afc4: 4c 8d 05 9d 9b 15 02        	lea	r8, [rip + 0x2159b9d]   # 0x142ab4b68
14095afcb: 48 8d 8d e0 01 00 00        	lea	rcx, [rbp + 0x1e0]
14095afd2: 48 8d 95 c0 02 00 00        	lea	rdx, [rbp + 0x2c0]
14095afd9: e8 12 1e 9a 00              	call	0x1412fcdf0 <.text+0x12fbdf0>
14095afde: 48 8b 95 00 03 00 00        	mov	rdx, qword ptr [rbp + 0x300]
14095afe5: 48 85 d2                    	test	rdx, rdx
14095afe8: 74 12                       	je	0x14095affc <.text+0x959ffc>
14095afea: 48 c1 e2 03                 	shl	rdx, 0x3
14095afee: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095aff4: 48 89 d9                    	mov	rcx, rbx
14095aff7: e8 24 8c cb ff              	call	0x140613c20 <.text+0x612c20>
14095affc: 48 8b 95 18 03 00 00        	mov	rdx, qword ptr [rbp + 0x318]
14095b003: 48 85 d2                    	test	rdx, rdx
14095b006: 74 16                       	je	0x14095b01e <.text+0x95a01e>
14095b008: 48 8b 8d 20 03 00 00        	mov	rcx, qword ptr [rbp + 0x320]
14095b00f: 48 c1 e2 02                 	shl	rdx, 0x2
14095b013: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095b019: e8 02 8c cb ff              	call	0x140613c20 <.text+0x612c20>
14095b01e: f6 85 58 02 00 00 01        	test	byte ptr [rbp + 0x258], 0x1
14095b025: 74 18                       	je	0x14095b03f <.text+0x95a03f>
14095b027: c6 85 87 03 00 00 01        	mov	byte ptr [rbp + 0x387], 0x1
14095b02e: 48 89 f9                    	mov	rcx, rdi
14095b031: e8 fa e4 33 00              	call	0x140c99530 <.text+0xc98530>
14095b036: 48 8d 8d c0 02 00 00        	lea	rcx, [rbp + 0x2c0]
14095b03d: eb 16                       	jmp	0x14095b055 <.text+0x95a055>
14095b03f: c6 85 87 03 00 00 01        	mov	byte ptr [rbp + 0x387], 0x1
14095b046: 48 89 f9                    	mov	rcx, rdi
14095b049: e8 22 c9 33 00              	call	0x140c97970 <.text+0xc96970>
14095b04e: 48 8d 8d c0 02 00 00        	lea	rcx, [rbp + 0x2c0]
14095b055: 0f 28 f0                    	movaps	xmm6, xmm0
14095b058: 48 8b 85 a0 04 00 00        	mov	rax, qword ptr [rbp + 0x4a0]
14095b05f: 48 8b 18                    	mov	rbx, qword ptr [rax]
14095b062: 48 89 b5 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rsi
14095b069: 48 8d 05 50 9e 1d 00        	lea	rax, [rip + 0x1d9e50]   # 0x140b34ec0 <.text+0xb33ec0>
14095b070: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095b077: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095b07e: 48 89 85 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rax
14095b085: 48 8d 05 d4 24 43 00        	lea	rax, [rip + 0x4324d4]   # 0x140d8d560 <.text+0xd8c560>
14095b08c: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
14095b093: 48 8d 05 d6 9d 15 02        	lea	rax, [rip + 0x2159dd6]  # 0x142ab4e70
14095b09a: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095b0a1: 48 c7 85 08 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x308], 0x2
14095b0ac: 48 89 8d 10 03 00 00        	mov	qword ptr [rbp + 0x310], rcx
14095b0b3: 48 c7 85 18 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x318], 0x2
14095b0be: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095b0c9: c6 85 87 03 00 00 01        	mov	byte ptr [rbp + 0x387], 0x1
14095b0d0: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
14095b0d7: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095b0de: e8 ad ee 41 00              	call	0x140d79f90 <.text+0xd78f90>
14095b0e3: 4c 8b bd 88 00 00 00        	mov	r15, qword ptr [rbp + 0x88]
14095b0ea: 4c 8b ad 90 00 00 00        	mov	r13, qword ptr [rbp + 0x90]
14095b0f1: 4c 8b b5 98 00 00 00        	mov	r14, qword ptr [rbp + 0x98]
14095b0f8: 48 8b bd e0 01 00 00        	mov	rdi, qword ptr [rbp + 0x1e0]
14095b0ff: 4c 8b a5 e8 01 00 00        	mov	r12, qword ptr [rbp + 0x1e8]
14095b106: 48 8b b5 f0 01 00 00        	mov	rsi, qword ptr [rbp + 0x1f0]
14095b10d: 0f b6 05 f5 9a cd 02        	movzx	eax, byte ptr [rip + 0x2cd9af5] # 0x143634c09
14095b114: b9 30 00 00 00              	mov	ecx, 0x30
14095b119: ba 08 00 00 00              	mov	edx, 0x8
14095b11e: e8 ed 8a cb ff              	call	0x140613c10 <.text+0x612c10>
14095b123: 48 85 c0                    	test	rax, rax
14095b126: 4c 89 bd 68 03 00 00        	mov	qword ptr [rbp + 0x368], r15
14095b12d: 4c 89 ad f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], r13
14095b134: 0f 84 4b 21 00 00           	je	0x14095d285 <.text+0x95c285>
14095b13a: 48 c7 00 01 00 00 00        	mov	qword ptr [rax], 0x1
14095b141: 48 c7 40 08 01 00 00 00     	mov	qword ptr [rax + 0x8], 0x1
14095b149: 48 89 78 10                 	mov	qword ptr [rax + 0x10], rdi
14095b14d: 4c 89 60 18                 	mov	qword ptr [rax + 0x18], r12
14095b151: 48 89 70 20                 	mov	qword ptr [rax + 0x20], rsi
14095b155: f3 44 0f 11 58 28           	movss	dword ptr [rax + 0x28], xmm11
14095b15b: f3 0f 11 70 2c              	movss	dword ptr [rax + 0x2c], xmm6
14095b160: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095b167: 48 8d 35 42 9e 15 02        	lea	rsi, [rip + 0x2159e42]  # 0x142ab4fb0
14095b16e: 48 89 b5 08 03 00 00        	mov	qword ptr [rbp + 0x308], rsi
14095b175: 4d 85 f6                    	test	r14, r14
14095b178: 48 89 85 58 03 00 00        	mov	qword ptr [rbp + 0x358], rax
14095b17f: 0f 88 22 21 00 00           	js	0x14095d2a7 <.text+0x95c2a7>
14095b185: 0f 84 d3 03 00 00           	je	0x14095b55e <.text+0x95a55e>
14095b18b: 0f b6 05 77 9a cd 02        	movzx	eax, byte ptr [rip + 0x2cd9a77] # 0x143634c09
14095b192: 41 bf 01 00 00 00           	mov	r15d, 0x1
14095b198: ba 01 00 00 00              	mov	edx, 0x1
14095b19d: 4c 89 f1                    	mov	rcx, r14
14095b1a0: e8 6b 8a cb ff              	call	0x140613c10 <.text+0x612c10>
14095b1a5: 48 85 c0                    	test	rax, rax
14095b1a8: 0f 84 fc 20 00 00           	je	0x14095d2aa <.text+0x95c2aa>
14095b1ae: 49 89 c4                    	mov	r12, rax
14095b1b1: e9 ae 03 00 00              	jmp	0x14095b564 <.text+0x95a564>
14095b1b6: 41 be 01 00 00 00           	mov	r14d, 0x1
14095b1bc: 4c 89 f1                    	mov	rcx, r14
14095b1bf: 4c 89 ea                    	mov	rdx, r13
14095b1c2: 49 89 d8                    	mov	r8, rbx
14095b1c5: e8 36 8a fc 01              	call	0x142923c00 <.text+0x2922c00>
14095b1ca: 48 89 9d 00 03 00 00        	mov	qword ptr [rbp + 0x300], rbx
14095b1d1: 4c 89 b5 08 03 00 00        	mov	qword ptr [rbp + 0x308], r14
14095b1d8: 48 89 9d 10 03 00 00        	mov	qword ptr [rbp + 0x310], rbx
14095b1df: 4c 89 bd 18 03 00 00        	mov	qword ptr [rbp + 0x318], r15
14095b1e6: 48 89 b5 20 03 00 00        	mov	qword ptr [rbp + 0x320], rsi
14095b1ed: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095b1f4: 48 8b 8d 58 03 00 00        	mov	rcx, qword ptr [rbp + 0x358]
14095b1fb: e8 80 16 b1 00              	call	0x14146c880 <.text+0x146b880>
14095b200: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095b207: 48 85 d2                    	test	rdx, rdx
14095b20a: 48 8b b5 60 03 00 00        	mov	rsi, qword ptr [rbp + 0x360]
14095b211: 74 12                       	je	0x14095b225 <.text+0x95a225>
14095b213: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095b219: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
14095b220: e8 fb 89 cb ff              	call	0x140613c20 <.text+0x612c20>
14095b225: 48 8b 8e f8 fd ff ff        	mov	rcx, qword ptr [rsi - 0x208]
14095b22c: 48 85 c9                    	test	rcx, rcx
14095b22f: 0f 84 38 20 00 00           	je	0x14095d26d <.text+0x95c26d>
14095b235: 48 8b 96 48 fe ff ff        	mov	rdx, qword ptr [rsi - 0x1b8]
14095b23c: 48 85 d2                    	test	rdx, rdx
14095b23f: 0f 84 7c 20 00 00           	je	0x14095d2c1 <.text+0x95c2c1>
14095b245: 48 8b 86 f0 fd ff ff        	mov	rax, qword ptr [rsi - 0x210]
14095b24c: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
14095b250: 48 8d 0c 88                 	lea	rcx, [rax + 4*rcx]
14095b254: 48 83 c1 f4                 	add	rcx, -0xc
14095b258: 0f 84 7b 20 00 00           	je	0x14095d2d9 <.text+0x95c2d9>
14095b25e: 4c 8b 86 40 fe ff ff        	mov	r8, qword ptr [rsi - 0x1c0]
14095b265: 49 8d 14 90                 	lea	rdx, [r8 + 4*rdx]
14095b269: 48 83 c2 fc                 	add	rdx, -0x4
14095b26d: 0f 84 7e 20 00 00           	je	0x14095d2f1 <.text+0x95c2f1>
14095b273: f3 41 0f 10 00              	movss	xmm0, dword ptr [r8]
14095b278: f3 0f 10 0a                 	movss	xmm1, dword ptr [rdx]
14095b27c: 8b 51 08                    	mov	edx, dword ptr [rcx + 0x8]
14095b27f: 89 95 38 03 00 00           	mov	dword ptr [rbp + 0x338], edx
14095b285: 48 8b 09                    	mov	rcx, qword ptr [rcx]
14095b288: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
14095b28f: 48 8b 08                    	mov	rcx, qword ptr [rax]
14095b292: 48 89 8d 10 03 00 00        	mov	qword ptr [rbp + 0x310], rcx
14095b299: 8b 40 08                    	mov	eax, dword ptr [rax + 0x8]
14095b29c: 89 85 18 03 00 00           	mov	dword ptr [rbp + 0x318], eax
14095b2a2: 48 8d 05 47 9c 15 02        	lea	rax, [rip + 0x2159c47]  # 0x142ab4ef0
14095b2a9: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095b2b0: 48 c7 85 08 03 00 00 11 00 00 00    	mov	qword ptr [rbp + 0x308], 0x11
14095b2bb: f3 0f 11 85 1c 03 00 00     	movss	dword ptr [rbp + 0x31c], xmm0
14095b2c3: 48 8d 05 6e 9c 15 02        	lea	rax, [rip + 0x2159c6e]  # 0x142ab4f38
14095b2ca: 48 89 85 20 03 00 00        	mov	qword ptr [rbp + 0x320], rax
14095b2d1: 48 c7 85 28 03 00 00 12 00 00 00    	mov	qword ptr [rbp + 0x328], 0x12
14095b2dc: f3 0f 11 8d 3c 03 00 00     	movss	dword ptr [rbp + 0x33c], xmm1
14095b2e4: 48 c7 85 48 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x348], 0x2
14095b2ef: 48 8d 9d c0 02 00 00        	lea	rbx, [rbp + 0x2c0]
14095b2f6: 48 8d 35 83 9c 15 02        	lea	rsi, [rip + 0x2159c83]  # 0x142ab4f80
14095b2fd: 31 c0                       	xor	eax, eax
14095b2ff: eb 12                       	jmp	0x14095b313 <.text+0x95a313>
14095b301: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
14095b310: 48 89 f8                    	mov	rax, rdi
14095b313: 48 83 f8 02                 	cmp	rax, 0x2
14095b317: 0f 84 99 0a 00 00           	je	0x14095bdb6 <.text+0x95adb6>
14095b31d: 48 8d 78 01                 	lea	rdi, [rax + 0x1]
14095b321: 48 89 bd 40 03 00 00        	mov	qword ptr [rbp + 0x340], rdi
14095b328: 48 c1 e0 05                 	shl	rax, 0x5
14095b32c: 48 8b 8c 05 00 03 00 00     	mov	rcx, qword ptr [rbp + rax + 0x300]
14095b334: 48 85 c9                    	test	rcx, rcx
14095b337: 0f 84 79 0a 00 00           	je	0x14095bdb6 <.text+0x95adb6>
14095b33d: 48 8d 14 28                 	lea	rdx, [rax + rbp]
14095b341: 48 81 c2 00 03 00 00        	add	rdx, 0x300
14095b348: 48 89 8d 80 02 00 00        	mov	qword ptr [rbp + 0x280], rcx
14095b34f: 44 8b a4 05 10 03 00 00     	mov	r12d, dword ptr [rbp + rax + 0x310]
14095b357: f3 0f 10 bc 05 14 03 00 00  	movss	xmm7, dword ptr [rbp + rax + 0x314]
14095b360: f3 0f 10 b4 05 1c 03 00 00  	movss	xmm6, dword ptr [rbp + rax + 0x31c]
14095b369: 44 8b ac 05 18 03 00 00     	mov	r13d, dword ptr [rbp + rax + 0x318]
14095b371: 48 8b 42 08                 	mov	rax, qword ptr [rdx + 0x8]
14095b375: 48 89 85 88 02 00 00        	mov	qword ptr [rbp + 0x288], rax
14095b37c: 48 8b 85 a0 04 00 00        	mov	rax, qword ptr [rbp + 0x4a0]
14095b383: 4c 8b 30                    	mov	r14, qword ptr [rax]
14095b386: 48 8d 85 80 02 00 00        	lea	rax, [rbp + 0x280]
14095b38d: 48 89 85 20 02 00 00        	mov	qword ptr [rbp + 0x220], rax
14095b394: 48 8d 05 f5 ea 7a ff        	lea	rax, [rip - 0x85150b]   # 0x140109e90 <.text+0x108e90>
14095b39b: 48 89 85 28 02 00 00        	mov	qword ptr [rbp + 0x228], rax
14095b3a2: 48 8d 85 b8 02 00 00        	lea	rax, [rbp + 0x2b8]
14095b3a9: 48 89 85 30 02 00 00        	mov	qword ptr [rbp + 0x230], rax
14095b3b0: 48 8d 05 09 9b 1d 00        	lea	rax, [rip + 0x1d9b09]   # 0x140b34ec0 <.text+0xb33ec0>
14095b3b7: 48 89 85 38 02 00 00        	mov	qword ptr [rbp + 0x238], rax
14095b3be: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095b3c5: 48 89 85 40 02 00 00        	mov	qword ptr [rbp + 0x240], rax
14095b3cc: 48 8d 05 8d 21 43 00        	lea	rax, [rip + 0x43218d]   # 0x140d8d560 <.text+0xd8c560>
14095b3d3: 48 89 85 48 02 00 00        	mov	qword ptr [rbp + 0x248], rax
14095b3da: 48 8d 05 6f 9b 15 02        	lea	rax, [rip + 0x2159b6f]  # 0x142ab4f50
14095b3e1: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
14095b3e8: 48 c7 85 c8 02 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x2c8], 0x3
14095b3f3: 48 8d 85 20 02 00 00        	lea	rax, [rbp + 0x220]
14095b3fa: 48 89 85 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rax
14095b401: 48 c7 85 d8 02 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x2d8], 0x3
14095b40c: 48 c7 85 e0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2e0], 0x0
14095b417: c6 85 85 03 00 00 00        	mov	byte ptr [rbp + 0x385], 0x0
14095b41e: 48 8d 8d a0 00 00 00        	lea	rcx, [rbp + 0xa0]
14095b425: 48 89 da                    	mov	rdx, rbx
14095b428: e8 63 eb 41 00              	call	0x140d79f90 <.text+0xd78f90>
14095b42d: 48 8b 85 a0 00 00 00        	mov	rax, qword ptr [rbp + 0xa0]
14095b434: 48 89 85 58 03 00 00        	mov	qword ptr [rbp + 0x358], rax
14095b43b: 48 8b 85 a8 00 00 00        	mov	rax, qword ptr [rbp + 0xa8]
14095b442: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
14095b449: 4c 8b bd b0 00 00 00        	mov	r15, qword ptr [rbp + 0xb0]
14095b450: 0f b6 05 b2 97 cd 02        	movzx	eax, byte ptr [rip + 0x2cd97b2] # 0x143634c09
14095b457: b9 28 00 00 00              	mov	ecx, 0x28
14095b45c: ba 08 00 00 00              	mov	edx, 0x8
14095b461: e8 aa 87 cb ff              	call	0x140613c10 <.text+0x612c10>
14095b466: 48 85 c0                    	test	rax, rax
14095b469: 0f 84 e6 1c 00 00           	je	0x14095d155 <.text+0x95c155>
14095b46f: 0f 28 c6                    	movaps	xmm0, xmm6
14095b472: f3 0f 5c c7                 	subss	xmm0, xmm7
14095b476: 48 c7 00 01 00 00 00        	mov	qword ptr [rax], 0x1
14095b47d: 48 c7 40 08 01 00 00 00     	mov	qword ptr [rax + 0x8], 0x1
14095b485: 44 89 60 10                 	mov	dword ptr [rax + 0x10], r12d
14095b489: f3 0f 11 78 14              	movss	dword ptr [rax + 0x14], xmm7
14095b48e: 44 89 68 18                 	mov	dword ptr [rax + 0x18], r13d
14095b492: f3 0f 11 70 1c              	movss	dword ptr [rax + 0x1c], xmm6
14095b497: f3 44 0f 11 58 20           	movss	dword ptr [rax + 0x20], xmm11
14095b49d: f3 0f 11 40 24              	movss	dword ptr [rax + 0x24], xmm0
14095b4a2: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
14095b4a9: 48 89 b5 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rsi
14095b4b0: 4d 85 ff                    	test	r15, r15
14095b4b3: 48 89 85 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rax
14095b4ba: 0f 88 a9 1c 00 00           	js	0x14095d169 <.text+0x95c169>
14095b4c0: 74 28                       	je	0x14095b4ea <.text+0x95a4ea>
14095b4c2: 0f b6 05 40 97 cd 02        	movzx	eax, byte ptr [rip + 0x2cd9740] # 0x143634c09
14095b4c9: 41 bd 01 00 00 00           	mov	r13d, 0x1
14095b4cf: ba 01 00 00 00              	mov	edx, 0x1
14095b4d4: 4c 89 f9                    	mov	rcx, r15
14095b4d7: e8 34 87 cb ff              	call	0x140613c10 <.text+0x612c10>
14095b4dc: 48 85 c0                    	test	rax, rax
14095b4df: 0f 84 87 1c 00 00           	je	0x14095d16c <.text+0x95c16c>
14095b4e5: 49 89 c4                    	mov	r12, rax
14095b4e8: eb 06                       	jmp	0x14095b4f0 <.text+0x95a4f0>
14095b4ea: 41 bc 01 00 00 00           	mov	r12d, 0x1
14095b4f0: 4c 89 e1                    	mov	rcx, r12
14095b4f3: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095b4fa: 4d 89 f8                    	mov	r8, r15
14095b4fd: e8 fe 86 fc 01              	call	0x142923c00 <.text+0x2922c00>
14095b502: 4c 89 bd c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], r15
14095b509: 4c 89 a5 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], r12
14095b510: 4c 89 bd d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], r15
14095b517: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
14095b51e: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
14095b525: 48 89 b5 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rsi
14095b52c: 4c 89 f1                    	mov	rcx, r14
14095b52f: 48 89 da                    	mov	rdx, rbx
14095b532: e8 49 13 b1 00              	call	0x14146c880 <.text+0x146b880>
14095b537: 48 8b 95 58 03 00 00        	mov	rdx, qword ptr [rbp + 0x358]
14095b53e: 48 85 d2                    	test	rdx, rdx
14095b541: 0f 84 c9 fd ff ff           	je	0x14095b310 <.text+0x95a310>
14095b547: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095b54d: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
14095b554: e8 c7 86 cb ff              	call	0x140613c20 <.text+0x612c20>
14095b559: e9 b2 fd ff ff              	jmp	0x14095b310 <.text+0x95a310>
14095b55e: 41 bc 01 00 00 00           	mov	r12d, 0x1
14095b564: 4c 89 e1                    	mov	rcx, r12
14095b567: 4c 89 ea                    	mov	rdx, r13
14095b56a: 4d 89 f0                    	mov	r8, r14
14095b56d: e8 8e 86 fc 01              	call	0x142923c00 <.text+0x2922c00>
14095b572: 4c 89 b5 00 03 00 00        	mov	qword ptr [rbp + 0x300], r14
14095b579: 4c 89 a5 08 03 00 00        	mov	qword ptr [rbp + 0x308], r12
14095b580: 4c 89 b5 10 03 00 00        	mov	qword ptr [rbp + 0x310], r14
14095b587: 48 8b 85 58 03 00 00        	mov	rax, qword ptr [rbp + 0x358]
14095b58e: 48 89 85 18 03 00 00        	mov	qword ptr [rbp + 0x318], rax
14095b595: 48 89 b5 20 03 00 00        	mov	qword ptr [rbp + 0x320], rsi
14095b59c: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095b5a3: 48 89 d9                    	mov	rcx, rbx
14095b5a6: e8 d5 12 b1 00              	call	0x14146c880 <.text+0x146b880>
14095b5ab: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095b5b2: 48 85 d2                    	test	rdx, rdx
14095b5b5: 48 8d b5 b8 02 00 00        	lea	rsi, [rbp + 0x2b8]
14095b5bc: 4c 8d a5 c0 02 00 00        	lea	r12, [rbp + 0x2c0]
14095b5c3: 74 12                       	je	0x14095b5d7 <.text+0x95a5d7>
14095b5c5: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095b5cb: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
14095b5d2: e8 49 86 cb ff              	call	0x140613c20 <.text+0x612c20>
14095b5d7: f6 85 58 02 00 00 01        	test	byte ptr [rbp + 0x258], 0x1
14095b5de: 0f 84 8f 03 00 00           	je	0x14095b973 <.text+0x95a973>
14095b5e4: 48 8d 8d 5c 02 00 00        	lea	rcx, [rbp + 0x25c]
14095b5eb: 48 8b 41 10                 	mov	rax, qword ptr [rcx + 0x10]
14095b5ef: 48 89 85 10 02 00 00        	mov	qword ptr [rbp + 0x210], rax
14095b5f6: 0f 10 01                    	movups	xmm0, xmmword ptr [rcx]
14095b5f9: 0f 29 85 00 02 00 00        	movaps	xmmword ptr [rbp + 0x200], xmm0
14095b600: 48 8d bd 00 02 00 00        	lea	rdi, [rbp + 0x200]
14095b607: f3 44 0f 10 0d f4 a2 fc 01  	movss	xmm9, dword ptr [rip + 0x1fca2f4] # 0x142925904
14095b610: 45 31 f6                    	xor	r14d, r14d
14095b613: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14095b620: 49 83 fe 04                 	cmp	r14, 0x4
14095b624: 0f 83 8c 07 00 00           	jae	0x14095bdb6 <.text+0x95adb6>
14095b62a: 4c 89 b5 78 01 00 00        	mov	qword ptr [rbp + 0x178], r14
14095b631: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095b638: 48 89 f9                    	mov	rcx, rdi
14095b63b: 4c 89 f2                    	mov	rdx, r14
14095b63e: e8 bd df 33 00              	call	0x140c99600 <.text+0xc98600>
14095b643: 0f 28 d1                    	movaps	xmm2, xmm1
14095b646: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095b64d: 48 8b 8d b0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2b0]
14095b654: 0f 28 c8                    	movaps	xmm1, xmm0
14095b657: e8 a4 e7 b0 00              	call	0x141469e00 <.text+0x1468e00>
14095b65c: 89 c3                       	mov	ebx, eax
14095b65e: 44 0f 28 c0                 	movaps	xmm8, xmm0
14095b662: 48 8b 95 78 01 00 00        	mov	rdx, qword ptr [rbp + 0x178]
14095b669: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095b670: 48 89 f9                    	mov	rcx, rdi
14095b673: e8 88 df 33 00              	call	0x140c99600 <.text+0xc98600>
14095b678: 89 d9                       	mov	ecx, ebx
14095b67a: 4c 8b 85 60 03 00 00        	mov	r8, qword ptr [rbp + 0x360]
14095b681: 49 8b 90 a8 fd ff ff        	mov	rdx, qword ptr [r8 - 0x258]
14095b688: 48 39 ca                    	cmp	rdx, rcx
14095b68b: 0f 86 63 1a 00 00           	jbe	0x14095d0f4 <.text+0x95c0f4>
14095b691: 48 8d 41 01                 	lea	rax, [rcx + 0x1]
14095b695: 48 39 d0                    	cmp	rax, rdx
14095b698: 0f 83 6e 1a 00 00           	jae	0x14095d10c <.text+0x95c10c>
14095b69e: 0f 28 f0                    	movaps	xmm6, xmm0
14095b6a1: 0f 28 f9                    	movaps	xmm7, xmm1
14095b6a4: 49 8b 90 a0 fd ff ff        	mov	rdx, qword ptr [r8 - 0x260]
14095b6ab: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
14095b6af: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14095b6b3: 45 0f 28 d1                 	movaps	xmm10, xmm9
14095b6b7: f3 45 0f 5c d0              	subss	xmm10, xmm8
14095b6bc: f3 44 0f 59 54 8a 04        	mulss	xmm10, dword ptr [rdx + 4*rcx + 0x4]
14095b6c3: f3 44 0f 10 64 82 04        	movss	xmm12, dword ptr [rdx + 4*rax + 0x4]
14095b6ca: f3 45 0f 59 e0              	mulss	xmm12, xmm8
14095b6cf: f3 44 0f 10 85 a4 02 00 00  	movss	xmm8, dword ptr [rbp + 0x2a4]
14095b6d8: 48 8b 85 a0 04 00 00        	mov	rax, qword ptr [rbp + 0x4a0]
14095b6df: 4c 8b 28                    	mov	r13, qword ptr [rax]
14095b6e2: 48 8d 85 78 01 00 00        	lea	rax, [rbp + 0x178]
14095b6e9: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
14095b6f0: 48 8d 05 c9 1f 43 00        	lea	rax, [rip + 0x431fc9]   # 0x140d8d6c0 <.text+0xd8c6c0>
14095b6f7: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095b6fe: 48 8d 05 c3 9a 15 02        	lea	rax, [rip + 0x2159ac3]  # 0x142ab51c8
14095b705: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095b70c: 48 c7 85 08 03 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x308], 0x1
14095b717: 4c 89 a5 10 03 00 00        	mov	qword ptr [rbp + 0x310], r12
14095b71e: 48 c7 85 18 03 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x318], 0x1
14095b729: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095b734: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095b73b: 48 8d 4d 28                 	lea	rcx, [rbp + 0x28]
14095b73f: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095b746: e8 45 e8 41 00              	call	0x140d79f90 <.text+0xd78f90>
14095b74b: 0f 10 45 28                 	movups	xmm0, xmmword ptr [rbp + 0x28]
14095b74f: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
14095b756: 48 8b 45 38                 	mov	rax, qword ptr [rbp + 0x38]
14095b75a: 48 89 85 30 02 00 00        	mov	qword ptr [rbp + 0x230], rax
14095b761: 48 8d 85 20 02 00 00        	lea	rax, [rbp + 0x220]
14095b768: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
14095b76f: 48 8d 05 5a 03 6e ff        	lea	rax, [rip - 0x91fca6]   # 0x14003bad0 <.text+0x3aad0>
14095b776: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095b77d: 48 89 b5 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rsi
14095b784: 48 8d 05 35 97 1d 00        	lea	rax, [rip + 0x1d9735]   # 0x140b34ec0 <.text+0xb33ec0>
14095b78b: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
14095b792: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095b799: 48 89 85 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rax
14095b7a0: 48 8d 05 b9 1d 43 00        	lea	rax, [rip + 0x431db9]   # 0x140d8d560 <.text+0xd8c560>
14095b7a7: 48 89 85 e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], rax
14095b7ae: 48 8d 05 9b 97 15 02        	lea	rax, [rip + 0x215979b]  # 0x142ab4f50
14095b7b5: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095b7bc: 48 c7 85 08 03 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x308], 0x3
14095b7c7: 4c 89 a5 10 03 00 00        	mov	qword ptr [rbp + 0x310], r12
14095b7ce: 48 c7 85 18 03 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x318], 0x3
14095b7d9: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095b7e4: 48 8d 8d 80 02 00 00        	lea	rcx, [rbp + 0x280]
14095b7eb: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095b7f2: e8 99 e7 41 00              	call	0x140d79f90 <.text+0xd78f90>
14095b7f7: 48 8b 95 20 02 00 00        	mov	rdx, qword ptr [rbp + 0x220]
14095b7fe: 48 85 d2                    	test	rdx, rdx
14095b801: 74 12                       	je	0x14095b815 <.text+0x95a815>
14095b803: 48 8b 8d 28 02 00 00        	mov	rcx, qword ptr [rbp + 0x228]
14095b80a: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095b810: e8 0b 84 cb ff              	call	0x140613c20 <.text+0x612c20>
14095b815: 48 8b 85 80 02 00 00        	mov	rax, qword ptr [rbp + 0x280]
14095b81c: 48 89 85 58 03 00 00        	mov	qword ptr [rbp + 0x358], rax
14095b823: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
14095b82a: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
14095b831: 48 8b 9d 90 02 00 00        	mov	rbx, qword ptr [rbp + 0x290]
14095b838: f3 44 0f 10 ad a4 02 00 00  	movss	xmm13, dword ptr [rbp + 0x2a4]
14095b841: 0f b6 05 c1 93 cd 02        	movzx	eax, byte ptr [rip + 0x2cd93c1] # 0x143634c09
14095b848: b9 28 00 00 00              	mov	ecx, 0x28
14095b84d: ba 08 00 00 00              	mov	edx, 0x8
14095b852: e8 b9 83 cb ff              	call	0x140613c10 <.text+0x612c10>
14095b857: 48 85 c0                    	test	rax, rax
14095b85a: 0f 84 c7 18 00 00           	je	0x14095d127 <.text+0x95c127>
14095b860: 49 ff c6                    	inc	r14
14095b863: f3 45 0f 58 d4              	addss	xmm10, xmm12
14095b868: f3 45 0f 5c c2              	subss	xmm8, xmm10
14095b86d: 48 c7 00 01 00 00 00        	mov	qword ptr [rax], 0x1
14095b874: 48 c7 40 08 01 00 00 00     	mov	qword ptr [rax + 0x8], 0x1
14095b87c: f3 0f 11 70 10              	movss	dword ptr [rax + 0x10], xmm6
14095b881: f3 0f 11 78 14              	movss	dword ptr [rax + 0x14], xmm7
14095b886: f3 44 0f 11 50 18           	movss	dword ptr [rax + 0x18], xmm10
14095b88c: f3 44 0f 11 68 1c           	movss	dword ptr [rax + 0x1c], xmm13
14095b892: f3 44 0f 11 58 20           	movss	dword ptr [rax + 0x20], xmm11
14095b898: f3 44 0f 11 40 24           	movss	dword ptr [rax + 0x24], xmm8
14095b89e: 48 89 85 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rax
14095b8a5: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095b8ac: 48 8d 05 25 99 15 02        	lea	rax, [rip + 0x2159925]  # 0x142ab51d8
14095b8b3: 48 89 85 08 03 00 00        	mov	qword ptr [rbp + 0x308], rax
14095b8ba: 48 85 db                    	test	rbx, rbx
14095b8bd: 0f 88 78 18 00 00           	js	0x14095d13b <.text+0x95c13b>
14095b8c3: 74 2f                       	je	0x14095b8f4 <.text+0x95a8f4>
14095b8c5: 0f b6 05 3d 93 cd 02        	movzx	eax, byte ptr [rip + 0x2cd933d] # 0x143634c09
14095b8cc: 41 bc 01 00 00 00           	mov	r12d, 0x1
14095b8d2: ba 01 00 00 00              	mov	edx, 0x1
14095b8d7: 48 89 d9                    	mov	rcx, rbx
14095b8da: e8 31 83 cb ff              	call	0x140613c10 <.text+0x612c10>
14095b8df: 48 85 c0                    	test	rax, rax
14095b8e2: 0f 84 56 18 00 00           	je	0x14095d13e <.text+0x95c13e>
14095b8e8: 49 89 c7                    	mov	r15, rax
14095b8eb: 4c 8d a5 c0 02 00 00        	lea	r12, [rbp + 0x2c0]
14095b8f2: eb 06                       	jmp	0x14095b8fa <.text+0x95a8fa>
14095b8f4: 41 bf 01 00 00 00           	mov	r15d, 0x1
14095b8fa: 4c 89 f9                    	mov	rcx, r15
14095b8fd: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095b904: 49 89 d8                    	mov	r8, rbx
14095b907: e8 f4 82 fc 01              	call	0x142923c00 <.text+0x2922c00>
14095b90c: 48 89 9d 00 03 00 00        	mov	qword ptr [rbp + 0x300], rbx
14095b913: 4c 89 bd 08 03 00 00        	mov	qword ptr [rbp + 0x308], r15
14095b91a: 48 89 9d 10 03 00 00        	mov	qword ptr [rbp + 0x310], rbx
14095b921: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
14095b928: 48 89 85 18 03 00 00        	mov	qword ptr [rbp + 0x318], rax
14095b92f: 48 8d 05 a2 98 15 02        	lea	rax, [rip + 0x21598a2]  # 0x142ab51d8
14095b936: 48 89 85 20 03 00 00        	mov	qword ptr [rbp + 0x320], rax
14095b93d: 4c 89 e9                    	mov	rcx, r13
14095b940: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095b947: e8 34 0f b1 00              	call	0x14146c880 <.text+0x146b880>
14095b94c: 48 8b 95 58 03 00 00        	mov	rdx, qword ptr [rbp + 0x358]
14095b953: 48 85 d2                    	test	rdx, rdx
14095b956: 0f 84 c4 fc ff ff           	je	0x14095b620 <.text+0x95a620>
14095b95c: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095b962: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
14095b969: e8 b2 82 cb ff              	call	0x140613c20 <.text+0x612c20>
14095b96e: e9 ad fc ff ff              	jmp	0x14095b620 <.text+0x95a620>
14095b973: 48 8d 85 5c 02 00 00        	lea	rax, [rbp + 0x25c]
14095b97a: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14095b97d: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
14095b981: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095b988: 4c 8d bd c0 02 00 00        	lea	r15, [rbp + 0x2c0]
14095b98f: 48 8d 95 10 01 00 00        	lea	rdx, [rbp + 0x110]
14095b996: 4c 8d 45 40                 	lea	r8, [rbp + 0x40]
14095b99a: 4c 89 f9                    	mov	rcx, r15
14095b99d: e8 ae 4c 00 00              	call	0x140960650 <.text+0x95f650>
14095b9a2: 8b 85 c8 02 00 00           	mov	eax, dword ptr [rbp + 0x2c8]
14095b9a8: 89 85 08 03 00 00           	mov	dword ptr [rbp + 0x308], eax
14095b9ae: 48 8b 85 c0 02 00 00        	mov	rax, qword ptr [rbp + 0x2c0]
14095b9b5: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095b9bc: 48 8b 85 cc 02 00 00        	mov	rax, qword ptr [rbp + 0x2cc]
14095b9c3: 48 89 85 0c 03 00 00        	mov	qword ptr [rbp + 0x30c], rax
14095b9ca: 8b 85 d4 02 00 00           	mov	eax, dword ptr [rbp + 0x2d4]
14095b9d0: 89 85 14 03 00 00           	mov	dword ptr [rbp + 0x314], eax
14095b9d6: 48 8b 85 10 03 00 00        	mov	rax, qword ptr [rbp + 0x310]
14095b9dd: 48 89 85 40 02 00 00        	mov	qword ptr [rbp + 0x240], rax
14095b9e4: 48 8b 85 00 03 00 00        	mov	rax, qword ptr [rbp + 0x300]
14095b9eb: 48 89 85 30 02 00 00        	mov	qword ptr [rbp + 0x230], rax
14095b9f2: 8b 85 08 03 00 00           	mov	eax, dword ptr [rbp + 0x308]
14095b9f8: 89 85 38 02 00 00           	mov	dword ptr [rbp + 0x238], eax
14095b9fe: 8b 85 0c 03 00 00           	mov	eax, dword ptr [rbp + 0x30c]
14095ba04: 89 85 3c 02 00 00           	mov	dword ptr [rbp + 0x23c], eax
14095ba0a: 48 c7 85 28 02 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x228], 0x2
14095ba15: 48 8b 85 c0 04 00 00        	mov	rax, qword ptr [rbp + 0x4c0]
14095ba1c: 48 8b 00                    	mov	rax, qword ptr [rax]
14095ba1f: 48 89 85 38 01 00 00        	mov	qword ptr [rbp + 0x138], rax
14095ba26: 48 8d 9d 00 03 00 00        	lea	rbx, [rbp + 0x300]
14095ba2d: 4c 8d a5 e8 00 00 00        	lea	r12, [rbp + 0xe8]
14095ba34: 4c 8d 2d cd 95 15 02        	lea	r13, [rip + 0x21595cd]  # 0x142ab5008
14095ba3b: 31 c0                       	xor	eax, eax
14095ba3d: eb 04                       	jmp	0x14095ba43 <.text+0x95aa43>
14095ba3f: 90                          	nop
14095ba40: 48 89 f8                    	mov	rax, rdi
14095ba43: 48 83 f8 02                 	cmp	rax, 0x2
14095ba47: 0f 84 69 03 00 00           	je	0x14095bdb6 <.text+0x95adb6>
14095ba4d: 4c 8d 70 01                 	lea	r14, [rax + 0x1]
14095ba51: 4c 89 b5 20 02 00 00        	mov	qword ptr [rbp + 0x220], r14
14095ba58: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14095ba5c: 48 8d 8d 30 02 00 00        	lea	rcx, [rbp + 0x230]
14095ba63: f3 0f 10 34 81              	movss	xmm6, dword ptr [rcx + 4*rax]
14095ba68: f3 0f 10 95 a4 02 00 00     	movss	xmm2, dword ptr [rbp + 0x2a4]
14095ba70: 44 0f 28 c2                 	movaps	xmm8, xmm2
14095ba74: f3 44 0f 5c 44 81 04        	subss	xmm8, dword ptr [rcx + 4*rax + 0x4]
14095ba7b: f3 0f 10 7c 81 08           	movss	xmm7, dword ptr [rcx + 4*rax + 0x8]
14095ba81: f3 0f 11 b5 00 03 00 00     	movss	dword ptr [rbp + 0x300], xmm6
14095ba89: f3 0f 11 bd 04 03 00 00     	movss	dword ptr [rbp + 0x304], xmm7
14095ba91: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095ba98: 48 8d 8d f4 00 00 00        	lea	rcx, [rbp + 0xf4]
14095ba9f: 48 89 da                    	mov	rdx, rbx
14095baa2: e8 39 13 34 00              	call	0x140c9cde0 <.text+0xc9bde0>
14095baa7: f3 0f 11 b5 00 03 00 00     	movss	dword ptr [rbp + 0x300], xmm6
14095baaf: f3 0f 11 bd 04 03 00 00     	movss	dword ptr [rbp + 0x304], xmm7
14095bab7: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095babe: 48 8b 8d 38 01 00 00        	mov	rcx, qword ptr [rbp + 0x138]
14095bac5: 0f 28 ce                    	movaps	xmm1, xmm6
14095bac8: 0f 28 d7                    	movaps	xmm2, xmm7
14095bacb: e8 70 db 08 00              	call	0x1409e9640 <.text+0x9e8640>
14095bad0: 48 8b 85 60 03 00 00        	mov	rax, qword ptr [rbp + 0x360]
14095bad7: f3 0f 10 48 e0              	movss	xmm1, dword ptr [rax - 0x20]
14095badc: 0f 28 d0                    	movaps	xmm2, xmm0
14095badf: f3 0f c2 d0 03              	cmpunordss	xmm2, xmm0
14095bae4: 0f 28 da                    	movaps	xmm3, xmm2
14095bae7: 0f 54 d9                    	andps	xmm3, xmm1
14095baea: f3 0f 5f c8                 	maxss	xmm1, xmm0
14095baee: 0f 55 d1                    	andnps	xmm2, xmm1
14095baf1: 0f 56 d3                    	orps	xmm2, xmm3
14095baf4: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095bafb: 4c 89 e1                    	mov	rcx, r12
14095bafe: 48 89 da                    	mov	rdx, rbx
14095bb01: e8 da 12 34 00              	call	0x140c9cde0 <.text+0xc9bde0>
14095bb06: 48 8b 85 a0 04 00 00        	mov	rax, qword ptr [rbp + 0x4a0]
14095bb0d: 48 8b 38                    	mov	rdi, qword ptr [rax]
14095bb10: 4c 89 a5 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], r12
14095bb17: 48 8d 05 b2 fc 3e 00        	lea	rax, [rip + 0x3efcb2]   # 0x140d4b7d0 <.text+0xd4a7d0>
14095bb1e: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095bb25: 48 8d 05 cc 94 15 02        	lea	rax, [rip + 0x21594cc]  # 0x142ab4ff8
14095bb2c: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095bb33: 48 c7 85 08 03 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x308], 0x1
14095bb3e: 4c 89 bd 10 03 00 00        	mov	qword ptr [rbp + 0x310], r15
14095bb45: 48 c7 85 18 03 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x318], 0x1
14095bb50: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095bb5b: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095bb62: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14095bb66: 48 89 da                    	mov	rdx, rbx
14095bb69: e8 22 e4 41 00              	call	0x140d79f90 <.text+0xd78f90>
14095bb6e: 0f 10 45 10                 	movups	xmm0, xmmword ptr [rbp + 0x10]
14095bb72: 0f 29 85 80 02 00 00        	movaps	xmmword ptr [rbp + 0x280], xmm0
14095bb79: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
14095bb7d: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
14095bb84: 48 8d 85 80 02 00 00        	lea	rax, [rbp + 0x280]
14095bb8b: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
14095bb92: 48 8d 05 37 ff 6d ff        	lea	rax, [rip - 0x9200c9]   # 0x14003bad0 <.text+0x3aad0>
14095bb99: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095bba0: 48 89 b5 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rsi
14095bba7: 48 8d 05 12 93 1d 00        	lea	rax, [rip + 0x1d9312]   # 0x140b34ec0 <.text+0xb33ec0>
14095bbae: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
14095bbb5: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095bbbc: 48 89 85 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rax
14095bbc3: 48 8d 05 96 19 43 00        	lea	rax, [rip + 0x431996]   # 0x140d8d560 <.text+0xd8c560>
14095bbca: 48 89 85 e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], rax
14095bbd1: 48 8d 05 78 93 15 02        	lea	rax, [rip + 0x2159378]  # 0x142ab4f50
14095bbd8: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095bbdf: 48 c7 85 08 03 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x308], 0x3
14095bbea: 4c 89 bd 10 03 00 00        	mov	qword ptr [rbp + 0x310], r15
14095bbf1: 48 c7 85 18 03 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x318], 0x3
14095bbfc: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095bc07: 48 8d 8d 00 02 00 00        	lea	rcx, [rbp + 0x200]
14095bc0e: 48 89 da                    	mov	rdx, rbx
14095bc11: e8 7a e3 41 00              	call	0x140d79f90 <.text+0xd78f90>
14095bc16: 48 89 bd b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rdi
14095bc1d: 48 8b 95 80 02 00 00        	mov	rdx, qword ptr [rbp + 0x280]
14095bc24: 48 85 d2                    	test	rdx, rdx
14095bc27: 74 12                       	je	0x14095bc3b <.text+0x95ac3b>
14095bc29: 48 8b 8d 88 02 00 00        	mov	rcx, qword ptr [rbp + 0x288]
14095bc30: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095bc36: e8 e5 7f cb ff              	call	0x140613c20 <.text+0x612c20>
14095bc3b: 48 8b 85 00 02 00 00        	mov	rax, qword ptr [rbp + 0x200]
14095bc42: 48 89 85 58 03 00 00        	mov	qword ptr [rbp + 0x358], rax
14095bc49: 48 8b 85 08 02 00 00        	mov	rax, qword ptr [rbp + 0x208]
14095bc50: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
14095bc57: 48 8b bd 10 02 00 00        	mov	rdi, qword ptr [rbp + 0x210]
14095bc5e: 8b 85 f0 00 00 00           	mov	eax, dword ptr [rbp + 0xf0]
14095bc64: 89 85 08 03 00 00           	mov	dword ptr [rbp + 0x308], eax
14095bc6a: 48 8b 85 e8 00 00 00        	mov	rax, qword ptr [rbp + 0xe8]
14095bc71: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095bc78: 8b 85 fc 00 00 00           	mov	eax, dword ptr [rbp + 0xfc]
14095bc7e: 48 8d 8d 0c 03 00 00        	lea	rcx, [rbp + 0x30c]
14095bc85: 89 41 08                    	mov	dword ptr [rcx + 0x8], eax
14095bc88: 48 8b 85 f4 00 00 00        	mov	rax, qword ptr [rbp + 0xf4]
14095bc8f: 48 89 01                    	mov	qword ptr [rcx], rax
14095bc92: 0f b6 05 70 8f cd 02        	movzx	eax, byte ptr [rip + 0x2cd8f70] # 0x143634c09
14095bc99: b9 30 00 00 00              	mov	ecx, 0x30
14095bc9e: ba 08 00 00 00              	mov	edx, 0x8
14095bca3: e8 68 7f cb ff              	call	0x140613c10 <.text+0x612c10>
14095bca8: 48 85 c0                    	test	rax, rax
14095bcab: 0f 84 2f 14 00 00           	je	0x14095d0e0 <.text+0x95c0e0>
14095bcb1: 48 c7 00 01 00 00 00        	mov	qword ptr [rax], 0x1
14095bcb8: 48 c7 40 08 01 00 00 00     	mov	qword ptr [rax + 0x8], 0x1
14095bcc0: 0f 10 85 00 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x300]
14095bcc7: 0f 11 40 10                 	movups	xmmword ptr [rax + 0x10], xmm0
14095bccb: 48 8b 8d 10 03 00 00        	mov	rcx, qword ptr [rbp + 0x310]
14095bcd2: 48 89 48 20                 	mov	qword ptr [rax + 0x20], rcx
14095bcd6: f3 44 0f 11 58 28           	movss	dword ptr [rax + 0x28], xmm11
14095bcdc: f3 44 0f 11 40 2c           	movss	dword ptr [rax + 0x2c], xmm8
14095bce2: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095bce9: 4c 89 ad 08 03 00 00        	mov	qword ptr [rbp + 0x308], r13
14095bcf0: 48 85 ff                    	test	rdi, rdi
14095bcf3: 48 89 85 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rax
14095bcfa: 0f 88 ba 13 00 00           	js	0x14095d0ba <.text+0x95c0ba>
14095bd00: 4c 89 b5 78 02 00 00        	mov	qword ptr [rbp + 0x278], r14
14095bd07: 74 2e                       	je	0x14095bd37 <.text+0x95ad37>
14095bd09: 0f b6 05 f9 8e cd 02        	movzx	eax, byte ptr [rip + 0x2cd8ef9] # 0x143634c09
14095bd10: b8 01 00 00 00              	mov	eax, 0x1
14095bd15: 48 89 85 90 01 00 00        	mov	qword ptr [rbp + 0x190], rax
14095bd1c: ba 01 00 00 00              	mov	edx, 0x1
14095bd21: 48 89 f9                    	mov	rcx, rdi
14095bd24: e8 e7 7e cb ff              	call	0x140613c10 <.text+0x612c10>
14095bd29: 48 85 c0                    	test	rax, rax
14095bd2c: 0f 84 93 13 00 00           	je	0x14095d0c5 <.text+0x95c0c5>
14095bd32: 49 89 c6                    	mov	r14, rax
14095bd35: eb 06                       	jmp	0x14095bd3d <.text+0x95ad3d>
14095bd37: 41 be 01 00 00 00           	mov	r14d, 0x1
14095bd3d: 4c 89 f1                    	mov	rcx, r14
14095bd40: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095bd47: 49 89 f8                    	mov	r8, rdi
14095bd4a: e8 b1 7e fc 01              	call	0x142923c00 <.text+0x2922c00>
14095bd4f: 48 89 bd 00 03 00 00        	mov	qword ptr [rbp + 0x300], rdi
14095bd56: 4c 89 b5 08 03 00 00        	mov	qword ptr [rbp + 0x308], r14
14095bd5d: 48 89 bd 10 03 00 00        	mov	qword ptr [rbp + 0x310], rdi
14095bd64: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
14095bd6b: 48 89 85 18 03 00 00        	mov	qword ptr [rbp + 0x318], rax
14095bd72: 4c 89 ad 20 03 00 00        	mov	qword ptr [rbp + 0x320], r13
14095bd79: 48 8b 8d b0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2b0]
14095bd80: 48 89 da                    	mov	rdx, rbx
14095bd83: e8 f8 0a b1 00              	call	0x14146c880 <.text+0x146b880>
14095bd88: 48 8b 95 58 03 00 00        	mov	rdx, qword ptr [rbp + 0x358]
14095bd8f: 48 85 d2                    	test	rdx, rdx
14095bd92: 48 8b bd 78 02 00 00        	mov	rdi, qword ptr [rbp + 0x278]
14095bd99: 0f 84 a1 fc ff ff           	je	0x14095ba40 <.text+0x95aa40>
14095bd9f: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095bda5: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
14095bdac: e8 6f 7e cb ff              	call	0x140613c20 <.text+0x612c20>
14095bdb1: e9 8a fc ff ff              	jmp	0x14095ba40 <.text+0x95aa40>
14095bdb6: 80 bd 9c 01 00 00 00        	cmp	byte ptr [rbp + 0x19c], 0x0
14095bdbd: 41 b7 01                    	mov	r15b, 0x1
14095bdc0: 0f 84 1c e8 ff ff           	je	0x14095a5e2 <.text+0x9595e2>
14095bdc6: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
14095bdcd: 48 8b 38                    	mov	rdi, qword ptr [rax]
14095bdd0: 48 8b b5 b8 02 00 00        	mov	rsi, qword ptr [rbp + 0x2b8]
14095bdd7: 48 89 b5 00 03 00 00        	mov	qword ptr [rbp + 0x300], rsi
14095bdde: 48 83 7f 48 00              	cmp	qword ptr [rdi + 0x48], 0x0
14095bde3: 0f 84 f9 e7 ff ff           	je	0x14095a5e2 <.text+0x9595e2>
14095bde9: 48 8d 4f 50                 	lea	rcx, [rdi + 0x50]
14095bded: 48 8d 9d 00 03 00 00        	lea	rbx, [rbp + 0x300]
14095bdf4: 48 89 da                    	mov	rdx, rbx
14095bdf7: e8 74 4f c5 ff              	call	0x1405b0d70 <.text+0x5afd70>
14095bdfc: 48 8b 4f 30                 	mov	rcx, qword ptr [rdi + 0x30]
14095be00: 48 8b 57 38                 	mov	rdx, qword ptr [rdi + 0x38]
14095be04: 49 89 d0                    	mov	r8, rdx
14095be07: 49 21 c0                    	and	r8, rax
14095be0a: 48 c1 e8 39                 	shr	rax, 0x39
14095be0e: 66 0f 6e c0                 	movd	xmm0, eax
14095be12: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14095be16: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14095be1b: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
14095be20: 48 8d 41 f0                 	lea	rax, [rcx - 0x10]
14095be24: 45 31 c9                    	xor	r9d, r9d
14095be27: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
14095be2b: f3 42 0f 6f 14 01           	movdqu	xmm2, xmmword ptr [rcx + r8]
14095be31: 66 0f 6f da                 	movdqa	xmm3, xmm2
14095be35: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
14095be39: 66 44 0f d7 d3              	pmovmskb	r10d, xmm3
14095be3e: 45 85 d2                    	test	r10d, r10d
14095be41: 74 2a                       	je	0x14095be6d <.text+0x95ae6d>
14095be43: f3 45 0f bc da              	tzcnt	r11d, r10d
14095be48: 4d 01 c3                    	add	r11, r8
14095be4b: 49 21 d3                    	and	r11, rdx
14095be4e: 4c 89 df                    	mov	rdi, r11
14095be51: 48 c1 e7 04                 	shl	rdi, 0x4
14095be55: 49 89 c6                    	mov	r14, rax
14095be58: 49 29 fe                    	sub	r14, rdi
14095be5b: 49 39 36                    	cmp	qword ptr [r14], rsi
14095be5e: 74 2f                       	je	0x14095be8f <.text+0x95ae8f>
14095be60: 45 8d 5a ff                 	lea	r11d, [r10 - 0x1]
14095be64: 66 45 21 d3                 	and	r11w, r10w
14095be68: 45 89 da                    	mov	r10d, r11d
14095be6b: 75 d6                       	jne	0x14095be43 <.text+0x95ae43>
14095be6d: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
14095be71: 66 44 0f d7 d2              	pmovmskb	r10d, xmm2
14095be76: 45 85 d2                    	test	r10d, r10d
14095be79: 0f 85 63 e7 ff ff           	jne	0x14095a5e2 <.text+0x9595e2>
14095be7f: 4d 01 c8                    	add	r8, r9
14095be82: 49 83 c0 10                 	add	r8, 0x10
14095be86: 49 83 c1 10                 	add	r9, 0x10
14095be8a: 49 21 d0                    	and	r8, rdx
14095be8d: eb 9c                       	jmp	0x14095be2b <.text+0x95ae2b>
14095be8f: 49 f7 db                    	neg	r11
14095be92: 49 c1 e3 04                 	shl	r11, 0x4
14095be96: 4a 8b 54 19 f8              	mov	rdx, qword ptr [rcx + r11 - 0x8]
14095be9b: 48 89 d0                    	mov	rax, rdx
14095be9e: 48 c1 e8 20                 	shr	rax, 0x20
14095bea2: 0f 84 3a e7 ff ff           	je	0x14095a5e2 <.text+0x9595e2>
14095bea8: 48 8b 8d d8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1d8]
14095beaf: 48 8b 49 08                 	mov	rcx, qword ptr [rcx + 0x8]
14095beb3: 89 d2                       	mov	edx, edx
14095beb5: 48 3b 51 10                 	cmp	rdx, qword ptr [rcx + 0x10]
14095beb9: 0f 83 23 e7 ff ff           	jae	0x14095a5e2 <.text+0x9595e2>
14095bebf: 4c 8b 41 08                 	mov	r8, qword ptr [rcx + 0x8]
14095bec3: 48 8d 14 92                 	lea	rdx, [rdx + 4*rdx]
14095bec7: 41 39 04 90                 	cmp	dword ptr [r8 + 4*rdx], eax
14095becb: 0f 85 11 e7 ff ff           	jne	0x14095a5e2 <.text+0x9595e2>
14095bed1: 49 8d 04 90                 	lea	rax, [r8 + 4*rdx]
14095bed5: 44 8b 40 04                 	mov	r8d, dword ptr [rax + 0x4]
14095bed9: ba ff ff ff ff              	mov	edx, 0xffffffff
14095bede: 49 39 d0                    	cmp	r8, rdx
14095bee1: 0f 84 fb e6 ff ff           	je	0x14095a5e2 <.text+0x9595e2>
14095bee7: 48 8b 95 d8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1d8]
14095beee: 48 8b 12                    	mov	rdx, qword ptr [rdx]
14095bef1: 4c 39 42 38                 	cmp	qword ptr [rdx + 0x38], r8
14095bef5: 0f 86 e7 e6 ff ff           	jbe	0x14095a5e2 <.text+0x9595e2>
14095befb: 4c 8b 4a 28                 	mov	r9, qword ptr [rdx + 0x28]
14095beff: 45 89 c2                    	mov	r10d, r8d
14095bf02: 41 c1 ea 06                 	shr	r10d, 0x6
14095bf06: 4f 8b 0c d1                 	mov	r9, qword ptr [r9 + 8*r10]
14095bf0a: 4d 0f a3 c1                 	bt	r9, r8
14095bf0e: 0f 83 ce e6 ff ff           	jae	0x14095a5e2 <.text+0x9595e2>
14095bf14: 44 8b 48 0c                 	mov	r9d, dword ptr [rax + 0xc]
14095bf18: 4c 8b 81 a8 01 00 00        	mov	r8, qword ptr [rcx + 0x1a8]
14095bf1f: 4f 8d 0c c9                 	lea	r9, [r9 + 8*r9]
14095bf23: 48 8b 8a 10 01 00 00        	mov	rcx, qword ptr [rdx + 0x110]
14095bf2a: 4b 39 4c c8 40              	cmp	qword ptr [r8 + 8*r9 + 0x40], rcx
14095bf2f: 76 3a                       	jbe	0x14095bf6b <.text+0x95af6b>
14095bf31: 4b 8d 14 c8                 	lea	rdx, [r8 + 8*r9]
14095bf35: 4c 8b 42 38                 	mov	r8, qword ptr [rdx + 0x38]
14095bf39: 49 8b 0c c8                 	mov	rcx, qword ptr [r8 + 8*rcx]
14095bf3d: 48 85 c9                    	test	rcx, rcx
14095bf40: 74 29                       	je	0x14095bf6b <.text+0x95af6b>
14095bf42: 48 8b 52 18                 	mov	rdx, qword ptr [rdx + 0x18]
14095bf46: 48 f7 d1                    	not	rcx
14095bf49: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
14095bf4d: 48 c1 e1 04                 	shl	rcx, 0x4
14095bf51: 48 8b 4c 0a 10              	mov	rcx, qword ptr [rdx + rcx + 0x10]
14095bf56: 48 89 8d 78 03 00 00        	mov	qword ptr [rbp + 0x378], rcx
14095bf5d: eb 17                       	jmp	0x14095bf76 <.text+0x95af76>
14095bf5f: 48 8d 0d 0a ec 2d 02        	lea	rcx, [rip + 0x22dec0a]  # 0x142c3ab70
14095bf66: e8 e5 d2 f7 01              	call	0x1428d9250 <.text+0x28d8250>
14095bf6b: 48 c7 85 78 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x378], 0x0
14095bf76: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
14095bf7d: 8b 49 b8                    	mov	ecx, dword ptr [rcx - 0x48]
14095bf80: 83 f9 01                    	cmp	ecx, 0x1
14095bf83: 0f 87 59 e6 ff ff           	ja	0x14095a5e2 <.text+0x9595e2>
14095bf89: 4c 8b 85 60 03 00 00        	mov	r8, qword ptr [rbp + 0x360]
14095bf90: 49 8b 50 bc                 	mov	rdx, qword ptr [r8 - 0x44]
14095bf94: f3 41 0f 10 40 c4           	movss	xmm0, dword ptr [r8 - 0x3c]
14095bf9a: 45 8b 40 c8                 	mov	r8d, dword ptr [r8 - 0x38]
14095bf9e: 45 31 c9                    	xor	r9d, r9d
14095bfa1: 85 c9                       	test	ecx, ecx
14095bfa3: 41 0f 94 c2                 	sete	r10b
14095bfa7: 45 89 fb                    	mov	r11d, r15d
14095bfaa: 75 08                       	jne	0x14095bfb4 <.text+0x95afb4>
14095bfac: f3 0f 58 05 cc 72 14 02     	addss	xmm0, dword ptr [rip + 0x21472cc] # 0x142aa3280
14095bfb4: 44 8b 60 10                 	mov	r12d, dword ptr [rax + 0x10]
14095bfb8: 48 8b 85 60 03 00 00        	mov	rax, qword ptr [rbp + 0x360]
14095bfbf: f2 0f 10 48 cc              	movsd	xmm1, qword ptr [rax - 0x34]
14095bfc4: 0f 28 15 85 85 14 02        	movaps	xmm2, xmmword ptr [rip + 0x2148585] # 0x142aa4550
14095bfcb: 0f 58 d1                    	addps	xmm2, xmm1
14095bfce: 45 88 d1                    	mov	r9b, r10b
14095bfd1: 66 41 0f 6e d9              	movd	xmm3, r9d
14095bfd6: 66 0f 70 db 50              	pshufd	xmm3, xmm3, 0x50        # xmm3 = xmm3[0,0,1,1]
14095bfdb: 66 0f 72 f3 1f              	pslld	xmm3, 0x1f
14095bfe0: 66 0f 72 e3 1f              	psrad	xmm3, 0x1f
14095bfe5: 0f 54 cb                    	andps	xmm1, xmm3
14095bfe8: 0f 55 da                    	andnps	xmm3, xmm2
14095bfeb: 0f 56 d9                    	orps	xmm3, xmm1
14095bfee: 89 8d 58 02 00 00           	mov	dword ptr [rbp + 0x258], ecx
14095bff4: 48 89 95 5c 02 00 00        	mov	qword ptr [rbp + 0x25c], rdx
14095bffb: f3 0f 11 85 64 02 00 00     	movss	dword ptr [rbp + 0x264], xmm0
14095c003: 44 89 85 68 02 00 00        	mov	dword ptr [rbp + 0x268], r8d
14095c00a: 0f 13 9d 6c 02 00 00        	movlps	qword ptr [rbp + 0x26c], xmm3
14095c011: 45 84 db                    	test	r11b, r11b
14095c014: 0f 85 f5 01 00 00           	jne	0x14095c20f <.text+0x95b20f>
14095c01a: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095c021: 48 8d 95 58 02 00 00        	lea	rdx, [rbp + 0x258]
14095c028: e8 83 32 8a 00              	call	0x1411ff2b0 <.text+0x11fe2b0>
14095c02d: 48 8b 85 08 03 00 00        	mov	rax, qword ptr [rbp + 0x308]
14095c034: 48 8b 8d 10 03 00 00        	mov	rcx, qword ptr [rbp + 0x310]
14095c03b: 48 8d 0c c8                 	lea	rcx, [rax + 8*rcx]
14095c03f: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
14095c046: 48 89 8d c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rcx
14095c04d: 48 8b 85 40 01 00 00        	mov	rax, qword ptr [rbp + 0x140]
14095c054: 48 89 85 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rax
14095c05b: 4c 8d 05 06 8b 15 02        	lea	r8, [rip + 0x2158b06]   # 0x142ab4b68
14095c062: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
14095c069: 4c 8d bd c0 02 00 00        	lea	r15, [rbp + 0x2c0]
14095c070: 4c 89 fa                    	mov	rdx, r15
14095c073: e8 d8 64 9a 00              	call	0x141302550 <.text+0x1301550>
14095c078: c6 85 77 03 00 00 01        	mov	byte ptr [rbp + 0x377], 0x1
14095c07f: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095c086: e8 65 64 c7 ff              	call	0x1405d24f0 <.text+0x5d14f0>
14095c08b: 48 8d b5 b8 02 00 00        	lea	rsi, [rbp + 0x2b8]
14095c092: c6 85 77 03 00 00 01        	mov	byte ptr [rbp + 0x377], 0x1
14095c099: 48 8d 8d 58 02 00 00        	lea	rcx, [rbp + 0x258]
14095c0a0: e8 9b 35 8a 00              	call	0x1411ff640 <.text+0x11fe640>
14095c0a5: 0f 28 f0                    	movaps	xmm6, xmm0
14095c0a8: 48 8b 85 a0 04 00 00        	mov	rax, qword ptr [rbp + 0x4a0]
14095c0af: 4c 8b 30                    	mov	r14, qword ptr [rax]
14095c0b2: 48 8b 85 a8 04 00 00        	mov	rax, qword ptr [rbp + 0x4a8]
14095c0b9: 48 8b 00                    	mov	rax, qword ptr [rax]
14095c0bc: 48 83 c0 2c                 	add	rax, 0x2c
14095c0c0: 48 89 b5 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rsi
14095c0c7: 48 8d 0d f2 8d 1d 00        	lea	rcx, [rip + 0x1d8df2]   # 0x140b34ec0 <.text+0xb33ec0>
14095c0ce: 48 89 8d c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rcx
14095c0d5: 48 89 85 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rax
14095c0dc: 48 8d 05 7d 14 43 00        	lea	rax, [rip + 0x43147d]   # 0x140d8d560 <.text+0xd8c560>
14095c0e3: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
14095c0ea: 48 8d 05 7f 8d 15 02        	lea	rax, [rip + 0x2158d7f]  # 0x142ab4e70
14095c0f1: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095c0f8: 48 c7 85 08 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x308], 0x2
14095c103: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095c10e: 4c 89 bd 10 03 00 00        	mov	qword ptr [rbp + 0x310], r15
14095c115: 48 c7 85 18 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x318], 0x2
14095c120: c6 85 77 03 00 00 01        	mov	byte ptr [rbp + 0x377], 0x1
14095c127: 48 8d 4d 70                 	lea	rcx, [rbp + 0x70]
14095c12b: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095c132: e8 69 d9 ff ff              	call	0x140959aa0 <.text+0x958aa0>
14095c137: 48 8b 45 70                 	mov	rax, qword ptr [rbp + 0x70]
14095c13b: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
14095c142: 48 8b 45 78                 	mov	rax, qword ptr [rbp + 0x78]
14095c146: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
14095c14d: 4c 8b bd 80 00 00 00        	mov	r15, qword ptr [rbp + 0x80]
14095c154: 48 8b 85 30 02 00 00        	mov	rax, qword ptr [rbp + 0x230]
14095c15b: 48 89 85 10 03 00 00        	mov	qword ptr [rbp + 0x310], rax
14095c162: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
14095c169: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
14095c170: f3 44 0f 11 9d 18 03 00 00  	movss	dword ptr [rbp + 0x318], xmm11
14095c179: f3 0f 11 b5 1c 03 00 00     	movss	dword ptr [rbp + 0x31c], xmm6
14095c181: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095c188: e8 f3 d9 ff ff              	call	0x140959b80 <.text+0x958b80>
14095c18d: 48 8d 0d a4 8e 15 02        	lea	rcx, [rip + 0x2158ea4]  # 0x142ab5038
14095c194: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
14095c199: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095c1a0: 48 8b 95 60 03 00 00        	mov	rdx, qword ptr [rbp + 0x360]
14095c1a7: 4d 89 f8                    	mov	r8, r15
14095c1aa: 49 89 c1                    	mov	r9, rax
14095c1ad: e8 fe 16 f4 ff              	call	0x14089d8b0 <.text+0x89c8b0>
14095c1b2: 48 8b 85 20 03 00 00        	mov	rax, qword ptr [rbp + 0x320]
14095c1b9: 48 89 85 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rax
14095c1c0: 0f 10 85 00 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x300]
14095c1c7: 0f 10 8d 10 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x310]
14095c1ce: 0f 29 8d d0 02 00 00        	movaps	xmmword ptr [rbp + 0x2d0], xmm1
14095c1d5: 0f 29 85 c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm0
14095c1dc: 48 8d 95 c0 02 00 00        	lea	rdx, [rbp + 0x2c0]
14095c1e3: 4c 89 f1                    	mov	rcx, r14
14095c1e6: e8 95 06 b1 00              	call	0x14146c880 <.text+0x146b880>
14095c1eb: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095c1f2: 48 85 d2                    	test	rdx, rdx
14095c1f5: 74 12                       	je	0x14095c209 <.text+0x95b209>
14095c1f7: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095c1fd: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
14095c204: e8 17 7a cb ff              	call	0x140613c20 <.text+0x612c20>
14095c209: 8b 8d 58 02 00 00           	mov	ecx, dword ptr [rbp + 0x258]
14095c20f: 49 6b c4 58                 	imul	rax, r12, 0x58
14095c213: 48 01 85 78 03 00 00        	add	qword ptr [rbp + 0x378], rax
14095c21a: f6 c1 01                    	test	cl, 0x1
14095c21d: 0f 84 15 06 00 00           	je	0x14095c838 <.text+0x95b838>
14095c223: 44 0f 10 85 5c 02 00 00     	movups	xmm8, xmmword ptr [rbp + 0x25c]
14095c22b: 4c 8d bd 00 03 00 00        	lea	r15, [rbp + 0x300]
14095c232: 4c 89 f9                    	mov	rcx, r15
14095c235: 48 8b 95 78 03 00 00        	mov	rdx, qword ptr [rbp + 0x378]
14095c23c: e8 7f be 12 00              	call	0x140a880c0 <.text+0xa870c0>
14095c241: f6 85 00 03 00 00 01        	test	byte ptr [rbp + 0x300], 0x1
14095c248: 0f 84 25 11 00 00           	je	0x14095d373 <.text+0x95c373>
14095c24e: 66 0f 6e 8d 04 03 00 00     	movd	xmm1, dword ptr [rbp + 0x304]
14095c256: 66 0f 6e 95 08 03 00 00     	movd	xmm2, dword ptr [rbp + 0x308]
14095c25e: f3 0f 10 b5 0c 03 00 00     	movss	xmm6, dword ptr [rbp + 0x30c]
14095c266: f3 0f 10 bd 10 03 00 00     	movss	xmm7, dword ptr [rbp + 0x310]
14095c26e: 4c 8d 0d 4b 8e 15 02        	lea	r9, [rip + 0x2158e4b]   # 0x142ab50c0
14095c275: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
14095c27c: e8 0f c6 33 00              	call	0x140c98890 <.text+0xc97890>
14095c281: 44 0f 29 85 c0 01 00 00     	movaps	xmmword ptr [rbp + 0x1c0], xmm8
14095c289: 4c 8d 0d 48 8e 15 02        	lea	r9, [rip + 0x2158e48]   # 0x142ab50d8
14095c290: 48 8d 8d e0 01 00 00        	lea	rcx, [rbp + 0x1e0]
14095c297: 0f 28 ce                    	movaps	xmm1, xmm6
14095c29a: 0f 28 d7                    	movaps	xmm2, xmm7
14095c29d: e8 ee c5 33 00              	call	0x140c98890 <.text+0xc97890>
14095c2a2: 44 0f 29 85 e0 01 00 00     	movaps	xmmword ptr [rbp + 0x1e0], xmm8
14095c2aa: 48 8b 85 a8 04 00 00        	mov	rax, qword ptr [rbp + 0x4a8]
14095c2b1: 48 8b 00                    	mov	rax, qword ptr [rax]
14095c2b4: 48 83 c0 2c                 	add	rax, 0x2c
14095c2b8: 48 89 85 a8 02 00 00        	mov	qword ptr [rbp + 0x2a8], rax
14095c2bf: f3 0f 10 35 bd 96 fc 01     	movss	xmm6, dword ptr [rip + 0x1fc96bd] # 0x142925984
14095c2c7: f3 44 0f 10 15 a0 96 fc 01  	movss	xmm10, dword ptr [rip + 0x1fc96a0] # 0x142925970
14095c2d0: 45 0f 57 e4                 	xorps	xmm12, xmm12
14095c2d4: f3 44 0f 10 2d 27 96 fc 01  	movss	xmm13, dword ptr [rip + 0x1fc9627] # 0x142925904
14095c2dd: f3 44 0f 10 35 92 0d 07 02  	movss	xmm14, dword ptr [rip + 0x2070d92] # 0x1429cd078
14095c2e6: f3 44 0f 10 3d e5 0f 07 02  	movss	xmm15, dword ptr [rip + 0x2070fe5] # 0x1429cd2d4
14095c2ef: 48 8d bd 80 02 00 00        	lea	rdi, [rbp + 0x280]
14095c2f6: 45 31 ed                    	xor	r13d, r13d
14095c2f9: 49 83 fd 04                 	cmp	r13, 0x4
14095c2fd: 0f 83 85 0b 00 00           	jae	0x14095ce88 <.text+0x95be88>
14095c303: 4c 89 ad 70 01 00 00        	mov	qword ptr [rbp + 0x170], r13
14095c30a: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
14095c311: 4c 89 ea                    	mov	rdx, r13
14095c314: e8 e7 d2 33 00              	call	0x140c99600 <.text+0xc98600>
14095c319: f3 0f 11 85 a8 01 00 00     	movss	dword ptr [rbp + 0x1a8], xmm0
14095c321: f3 0f 11 8d ac 01 00 00     	movss	dword ptr [rbp + 0x1ac], xmm1
14095c329: 48 8b 95 70 01 00 00        	mov	rdx, qword ptr [rbp + 0x170]
14095c330: 48 8d 8d e0 01 00 00        	lea	rcx, [rbp + 0x1e0]
14095c337: e8 c4 d2 33 00              	call	0x140c99600 <.text+0xc98600>
14095c33c: f3 0f 11 85 a0 01 00 00     	movss	dword ptr [rbp + 0x1a0], xmm0
14095c344: f3 0f 11 8d a4 01 00 00     	movss	dword ptr [rbp + 0x1a4], xmm1
14095c34c: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095c353: 66 0f 6e 50 4c              	movd	xmm2, dword ptr [rax + 0x4c]
14095c358: 48 8d 8d 58 01 00 00        	lea	rcx, [rbp + 0x158]
14095c35f: 48 8d 95 a8 01 00 00        	lea	rdx, [rbp + 0x1a8]
14095c366: e8 75 0a 34 00              	call	0x140c9cde0 <.text+0xc9bde0>
14095c36b: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095c372: f3 0f 10 40 30              	movss	xmm0, dword ptr [rax + 0x30]
14095c377: f3 0f 10 50 4c              	movss	xmm2, dword ptr [rax + 0x4c]
14095c37c: 0f 57 c9                    	xorps	xmm1, xmm1
14095c37f: 0f 2e f0                    	ucomiss	xmm6, xmm0
14095c382: 0f 83 df 00 00 00           	jae	0x14095c467 <.text+0x95b467>
14095c388: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095c38f: f6 40 08 01                 	test	byte ptr [rax + 0x8], 0x1
14095c393: 74 43                       	je	0x14095c3d8 <.text+0x95b3d8>
14095c395: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095c39c: f3 0f 10 58 1c              	movss	xmm3, dword ptr [rax + 0x1c]
14095c3a1: f3 0f 10 60 20              	movss	xmm4, dword ptr [rax + 0x20]
14095c3a6: 0f 28 cb                    	movaps	xmm1, xmm3
14095c3a9: f3 0f c2 cb 03              	cmpunordss	xmm1, xmm3
14095c3ae: 0f 28 e9                    	movaps	xmm5, xmm1
14095c3b1: 0f 54 ec                    	andps	xmm5, xmm4
14095c3b4: f3 0f 5f e3                 	maxss	xmm4, xmm3
14095c3b8: 0f 55 cc                    	andnps	xmm1, xmm4
14095c3bb: 0f 56 cd                    	orps	xmm1, xmm5
14095c3be: f3 0f 58 0d be 8c 14 02     	addss	xmm1, dword ptr [rip + 0x2148cbe] # 0x142aa5084
14095c3c6: f3 0f 5e 0d ba 8c 14 02     	divss	xmm1, dword ptr [rip + 0x2148cba] # 0x142aa5088
14095c3ce: f3 0f 10 1d ae f6 11 02     	movss	xmm3, dword ptr [rip + 0x211f6ae] # 0x142a7ba84
14095c3d6: eb 24                       	jmp	0x14095c3fc <.text+0x95b3fc>
14095c3d8: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095c3df: f3 0f 10 48 14              	movss	xmm1, dword ptr [rax + 0x14]
14095c3e4: f3 0f 58 0d a0 8c 14 02     	addss	xmm1, dword ptr [rip + 0x2148ca0] # 0x142aa508c
14095c3ec: f3 0f 5e 0d 9c 8c 14 02     	divss	xmm1, dword ptr [rip + 0x2148c9c] # 0x142aa5090
14095c3f4: f3 0f 10 1d 08 82 fd 01     	movss	xmm3, dword ptr [rip + 0x1fd8208] # 0x142934604
14095c3fc: f3 41 0f 5e ca              	divss	xmm1, xmm10
14095c401: 0f 57 e4                    	xorps	xmm4, xmm4
14095c404: f3 0f 5f e1                 	maxss	xmm4, xmm1
14095c408: 41 0f 28 ed                 	movaps	xmm5, xmm13
14095c40c: f3 0f 5d ec                 	minss	xmm5, xmm4
14095c410: 0f 28 e5                    	movaps	xmm4, xmm5
14095c413: f3 0f 59 e5                 	mulss	xmm4, xmm5
14095c417: f3 0f 58 ed                 	addss	xmm5, xmm5
14095c41b: 41 0f 28 ce                 	movaps	xmm1, xmm14
14095c41f: f3 0f 5c cd                 	subss	xmm1, xmm5
14095c423: f3 0f 59 cc                 	mulss	xmm1, xmm4
14095c427: 41 0f 28 e5                 	movaps	xmm4, xmm13
14095c42b: f3 0f 5c e1                 	subss	xmm4, xmm1
14095c42f: f3 0f 59 e3                 	mulss	xmm4, xmm3
14095c433: f3 41 0f 59 cf              	mulss	xmm1, xmm15
14095c438: f3 0f 58 cc                 	addss	xmm1, xmm4
14095c43c: 41 0f 2e cc                 	ucomiss	xmm1, xmm12
14095c440: 0f 82 60 0c 00 00           	jb	0x14095d0a6 <.text+0x95c0a6>
14095c446: 41 0f 28 dd                 	movaps	xmm3, xmm13
14095c44a: f3 0f 5c d8                 	subss	xmm3, xmm0
14095c44e: f3 41 0f 59 da              	mulss	xmm3, xmm10
14095c453: f3 41 0f 59 c7              	mulss	xmm0, xmm15
14095c458: f3 0f 58 c3                 	addss	xmm0, xmm3
14095c45c: 0f 57 db                    	xorps	xmm3, xmm3
14095c45f: f3 0f 5f d8                 	maxss	xmm3, xmm0
14095c463: f3 0f 5d cb                 	minss	xmm1, xmm3
14095c467: f3 0f 58 d1                 	addss	xmm2, xmm1
14095c46b: 48 8d 8d 4c 01 00 00        	lea	rcx, [rbp + 0x14c]
14095c472: 48 8d 95 a0 01 00 00        	lea	rdx, [rbp + 0x1a0]
14095c479: e8 62 09 34 00              	call	0x140c9cde0 <.text+0xc9bde0>
14095c47e: f3 44 0f 10 8d 58 01 00 00  	movss	xmm9, dword ptr [rbp + 0x158]
14095c487: f3 0f 10 bd 5c 01 00 00     	movss	xmm7, dword ptr [rbp + 0x15c]
14095c48f: f3 44 0f 10 85 60 01 00 00  	movss	xmm8, dword ptr [rbp + 0x160]
14095c498: f3 44 0f 5c 8d 4c 01 00 00  	subss	xmm9, dword ptr [rbp + 0x14c]
14095c4a1: f3 0f 5c bd 50 01 00 00     	subss	xmm7, dword ptr [rbp + 0x150]
14095c4a9: f3 44 0f 5c 85 54 01 00 00  	subss	xmm8, dword ptr [rbp + 0x154]
14095c4b2: f2 0f 10 85 a8 01 00 00     	movsd	xmm0, qword ptr [rbp + 0x1a8]
14095c4ba: f2 0f 11 85 80 02 00 00     	movsd	qword ptr [rbp + 0x280], xmm0
14095c4c2: f2 0f 10 85 a0 01 00 00     	movsd	xmm0, qword ptr [rbp + 0x1a0]
14095c4ca: f2 0f 11 85 20 02 00 00     	movsd	qword ptr [rbp + 0x220], xmm0
14095c4d2: 4c 89 f9                    	mov	rcx, r15
14095c4d5: 48 8b 95 78 03 00 00        	mov	rdx, qword ptr [rbp + 0x378]
14095c4dc: e8 3f 6f f8 ff              	call	0x1408e3420 <.text+0x8e2420>
14095c4e1: 4c 89 bd c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], r15
14095c4e8: 48 89 bd c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rdi
14095c4ef: 48 8d 85 20 02 00 00        	lea	rax, [rbp + 0x220]
14095c4f6: 48 89 85 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rax
14095c4fd: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095c504: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
14095c50b: 48 c7 85 e0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2e0], 0x0
14095c516: 48 c7 85 e8 02 00 00 05 00 00 00    	mov	qword ptr [rbp + 0x2e8], 0x5
14095c521: 48 8d 8d 00 02 00 00        	lea	rcx, [rbp + 0x200]
14095c528: 48 8d 95 c0 02 00 00        	lea	rdx, [rbp + 0x2c0]
14095c52f: 4c 8d 05 32 86 15 02        	lea	r8, [rip + 0x2158632]   # 0x142ab4b68
14095c536: e8 15 31 9a 00              	call	0x1412ff650 <.text+0x12fe650>
14095c53b: 48 8b 95 00 03 00 00        	mov	rdx, qword ptr [rbp + 0x300]
14095c542: 48 85 d2                    	test	rdx, rdx
14095c545: 74 16                       	je	0x14095c55d <.text+0x95b55d>
14095c547: 48 8b 8d 08 03 00 00        	mov	rcx, qword ptr [rbp + 0x308]
14095c54e: 48 c1 e2 03                 	shl	rdx, 0x3
14095c552: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095c558: e8 c3 76 cb ff              	call	0x140613c20 <.text+0x612c20>
14095c55d: 48 8b 95 18 03 00 00        	mov	rdx, qword ptr [rbp + 0x318]
14095c564: 48 85 d2                    	test	rdx, rdx
14095c567: 48 8d b5 b8 02 00 00        	lea	rsi, [rbp + 0x2b8]
14095c56e: 74 16                       	je	0x14095c586 <.text+0x95b586>
14095c570: 48 8b 8d 20 03 00 00        	mov	rcx, qword ptr [rbp + 0x320]
14095c577: 48 c1 e2 02                 	shl	rdx, 0x2
14095c57b: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095c581: e8 9a 76 cb ff              	call	0x140613c20 <.text+0x612c20>
14095c586: 48 8b 85 a0 04 00 00        	mov	rax, qword ptr [rbp + 0x4a0]
14095c58d: 4c 8b 30                    	mov	r14, qword ptr [rax]
14095c590: 48 8d 85 70 01 00 00        	lea	rax, [rbp + 0x170]
14095c597: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
14095c59e: 48 8d 05 1b 11 43 00        	lea	rax, [rip + 0x43111b]   # 0x140d8d6c0 <.text+0xd8c6c0>
14095c5a5: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095c5ac: 48 8d 05 bd 8b 15 02        	lea	rax, [rip + 0x2158bbd]  # 0x142ab5170
14095c5b3: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095c5ba: 48 c7 85 08 03 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x308], 0x1
14095c5c5: 48 8d 9d c0 02 00 00        	lea	rbx, [rbp + 0x2c0]
14095c5cc: 48 89 9d 10 03 00 00        	mov	qword ptr [rbp + 0x310], rbx
14095c5d3: 48 c7 85 18 03 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x318], 0x1
14095c5de: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095c5e9: c6 85 56 03 00 00 01        	mov	byte ptr [rbp + 0x356], 0x1
14095c5f0: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
14095c5f4: 4c 89 fa                    	mov	rdx, r15
14095c5f7: e8 94 d9 41 00              	call	0x140d79f90 <.text+0xd78f90>
14095c5fc: 0f 10 45 e0                 	movups	xmm0, xmmword ptr [rbp - 0x20]
14095c600: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
14095c607: 48 8b 45 f0                 	mov	rax, qword ptr [rbp - 0x10]
14095c60b: 48 89 85 30 02 00 00        	mov	qword ptr [rbp + 0x230], rax
14095c612: 48 8d 85 20 02 00 00        	lea	rax, [rbp + 0x220]
14095c619: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
14095c620: 48 8d 05 a9 f4 6d ff        	lea	rax, [rip - 0x920b57]   # 0x14003bad0 <.text+0x3aad0>
14095c627: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095c62e: 48 89 b5 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rsi
14095c635: 48 8d 05 84 88 1d 00        	lea	rax, [rip + 0x1d8884]   # 0x140b34ec0 <.text+0xb33ec0>
14095c63c: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
14095c643: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
14095c64a: 48 89 85 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rax
14095c651: 48 8d 05 08 0f 43 00        	lea	rax, [rip + 0x430f08]   # 0x140d8d560 <.text+0xd8c560>
14095c658: 48 89 85 e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], rax
14095c65f: 48 8d 05 ea 88 15 02        	lea	rax, [rip + 0x21588ea]  # 0x142ab4f50
14095c666: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095c66d: 48 c7 85 08 03 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x308], 0x3
14095c678: 48 89 9d 10 03 00 00        	mov	qword ptr [rbp + 0x310], rbx
14095c67f: 48 c7 85 18 03 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x318], 0x3
14095c68a: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095c695: 48 89 f9                    	mov	rcx, rdi
14095c698: 4c 89 fa                    	mov	rdx, r15
14095c69b: e8 f0 d8 41 00              	call	0x140d79f90 <.text+0xd78f90>
14095c6a0: 48 8b 95 20 02 00 00        	mov	rdx, qword ptr [rbp + 0x220]
14095c6a7: 48 85 d2                    	test	rdx, rdx
14095c6aa: 74 12                       	je	0x14095c6be <.text+0x95b6be>
14095c6ac: 48 8b 8d 28 02 00 00        	mov	rcx, qword ptr [rbp + 0x228]
14095c6b3: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095c6b9: e8 62 75 cb ff              	call	0x140613c20 <.text+0x612c20>
14095c6be: 48 8b 9d 80 02 00 00        	mov	rbx, qword ptr [rbp + 0x280]
14095c6c5: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
14095c6cc: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
14095c6d3: 4c 8b bd 90 02 00 00        	mov	r15, qword ptr [rbp + 0x290]
14095c6da: 4c 8b a5 00 02 00 00        	mov	r12, qword ptr [rbp + 0x200]
14095c6e1: 48 8b bd 08 02 00 00        	mov	rdi, qword ptr [rbp + 0x208]
14095c6e8: 48 8b b5 10 02 00 00        	mov	rsi, qword ptr [rbp + 0x210]
14095c6ef: 0f b6 05 13 85 cd 02        	movzx	eax, byte ptr [rip + 0x2cd8513] # 0x143634c09
14095c6f6: b9 30 00 00 00              	mov	ecx, 0x30
14095c6fb: ba 08 00 00 00              	mov	edx, 0x8
14095c700: e8 0b 75 cb ff              	call	0x140613c10 <.text+0x612c10>
14095c705: 48 85 c0                    	test	rax, rax
14095c708: 48 89 9d 68 03 00 00        	mov	qword ptr [rbp + 0x368], rbx
14095c70f: 0f 84 29 0c 00 00           	je	0x14095d33e <.text+0x95c33e>
14095c715: 49 ff c5                    	inc	r13
14095c718: f3 45 0f 59 c9              	mulss	xmm9, xmm9
14095c71d: f3 0f 59 ff                 	mulss	xmm7, xmm7
14095c721: f3 41 0f 58 f9              	addss	xmm7, xmm9
14095c726: f3 45 0f 59 c0              	mulss	xmm8, xmm8
14095c72b: f3 44 0f 58 c7              	addss	xmm8, xmm7
14095c730: 0f 57 c0                    	xorps	xmm0, xmm0
14095c733: f3 41 0f 51 c0              	sqrtss	xmm0, xmm8
14095c738: 48 c7 00 01 00 00 00        	mov	qword ptr [rax], 0x1
14095c73f: 48 c7 40 08 01 00 00 00     	mov	qword ptr [rax + 0x8], 0x1
14095c747: 4c 89 60 10                 	mov	qword ptr [rax + 0x10], r12
14095c74b: 48 89 78 18                 	mov	qword ptr [rax + 0x18], rdi
14095c74f: 48 89 70 20                 	mov	qword ptr [rax + 0x20], rsi
14095c753: f3 44 0f 11 58 28           	movss	dword ptr [rax + 0x28], xmm11
14095c759: f3 0f 11 40 2c              	movss	dword ptr [rax + 0x2c], xmm0
14095c75e: 48 89 85 58 03 00 00        	mov	qword ptr [rbp + 0x358], rax
14095c765: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095c76c: 48 8d 05 0d 8a 15 02        	lea	rax, [rip + 0x2158a0d]  # 0x142ab5180
14095c773: 48 89 85 08 03 00 00        	mov	qword ptr [rbp + 0x308], rax
14095c77a: 4d 85 ff                    	test	r15, r15
14095c77d: 0f 88 da 0b 00 00           	js	0x14095d35d <.text+0x95c35d>
14095c783: 48 8d b5 00 03 00 00        	lea	rsi, [rbp + 0x300]
14095c78a: 74 27                       	je	0x14095c7b3 <.text+0x95b7b3>
14095c78c: 0f b6 05 76 84 cd 02        	movzx	eax, byte ptr [rip + 0x2cd8476] # 0x143634c09
14095c793: bb 01 00 00 00              	mov	ebx, 0x1
14095c798: ba 01 00 00 00              	mov	edx, 0x1
14095c79d: 4c 89 f9                    	mov	rcx, r15
14095c7a0: e8 6b 74 cb ff              	call	0x140613c10 <.text+0x612c10>
14095c7a5: 48 85 c0                    	test	rax, rax
14095c7a8: 0f 84 b1 0b 00 00           	je	0x14095d35f <.text+0x95c35f>
14095c7ae: 49 89 c4                    	mov	r12, rax
14095c7b1: eb 06                       	jmp	0x14095c7b9 <.text+0x95b7b9>
14095c7b3: 41 bc 01 00 00 00           	mov	r12d, 0x1
14095c7b9: 4c 89 e1                    	mov	rcx, r12
14095c7bc: 48 8b 95 60 03 00 00        	mov	rdx, qword ptr [rbp + 0x360]
14095c7c3: 4d 89 f8                    	mov	r8, r15
14095c7c6: e8 35 74 fc 01              	call	0x142923c00 <.text+0x2922c00>
14095c7cb: 4c 89 bd 00 03 00 00        	mov	qword ptr [rbp + 0x300], r15
14095c7d2: 4c 89 a5 08 03 00 00        	mov	qword ptr [rbp + 0x308], r12
14095c7d9: 4c 89 bd 10 03 00 00        	mov	qword ptr [rbp + 0x310], r15
14095c7e0: 48 8b 85 58 03 00 00        	mov	rax, qword ptr [rbp + 0x358]
14095c7e7: 48 89 85 18 03 00 00        	mov	qword ptr [rbp + 0x318], rax
14095c7ee: 48 8d 05 8b 89 15 02        	lea	rax, [rip + 0x215898b]  # 0x142ab5180
14095c7f5: 48 89 85 20 03 00 00        	mov	qword ptr [rbp + 0x320], rax
14095c7fc: 4c 89 f1                    	mov	rcx, r14
14095c7ff: 49 89 f7                    	mov	r15, rsi
14095c802: 48 89 f2                    	mov	rdx, rsi
14095c805: e8 76 00 b1 00              	call	0x14146c880 <.text+0x146b880>
14095c80a: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095c811: 48 85 d2                    	test	rdx, rdx
14095c814: 48 8d bd 80 02 00 00        	lea	rdi, [rbp + 0x280]
14095c81b: 0f 84 d8 fa ff ff           	je	0x14095c2f9 <.text+0x95b2f9>
14095c821: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095c827: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
14095c82e: e8 ed 73 cb ff              	call	0x140613c20 <.text+0x612c20>
14095c833: e9 c1 fa ff ff              	jmp	0x14095c2f9 <.text+0x95b2f9>
14095c838: 48 8d 85 5c 02 00 00        	lea	rax, [rbp + 0x25c]
14095c83f: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14095c842: 0f 29 85 c0 01 00 00        	movaps	xmmword ptr [rbp + 0x1c0], xmm0
14095c849: 48 8d 8d c0 02 00 00        	lea	rcx, [rbp + 0x2c0]
14095c850: 48 8d 95 10 01 00 00        	lea	rdx, [rbp + 0x110]
14095c857: 4c 8d 85 c0 01 00 00        	lea	r8, [rbp + 0x1c0]
14095c85e: e8 ed 3d 00 00              	call	0x140960650 <.text+0x95f650>
14095c863: 8b 85 c8 02 00 00           	mov	eax, dword ptr [rbp + 0x2c8]
14095c869: 89 85 08 03 00 00           	mov	dword ptr [rbp + 0x308], eax
14095c86f: 48 8b 85 c0 02 00 00        	mov	rax, qword ptr [rbp + 0x2c0]
14095c876: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095c87d: 48 8b 85 cc 02 00 00        	mov	rax, qword ptr [rbp + 0x2cc]
14095c884: 48 89 85 0c 03 00 00        	mov	qword ptr [rbp + 0x30c], rax
14095c88b: 8b 85 d4 02 00 00           	mov	eax, dword ptr [rbp + 0x2d4]
14095c891: 89 85 14 03 00 00           	mov	dword ptr [rbp + 0x314], eax
14095c897: 48 8b 85 10 03 00 00        	mov	rax, qword ptr [rbp + 0x310]
14095c89e: 48 89 85 40 02 00 00        	mov	qword ptr [rbp + 0x240], rax
14095c8a5: 48 8b 85 00 03 00 00        	mov	rax, qword ptr [rbp + 0x300]
14095c8ac: 48 89 85 30 02 00 00        	mov	qword ptr [rbp + 0x230], rax
14095c8b3: 8b 85 08 03 00 00           	mov	eax, dword ptr [rbp + 0x308]
14095c8b9: 89 85 38 02 00 00           	mov	dword ptr [rbp + 0x238], eax
14095c8bf: 8b 85 0c 03 00 00           	mov	eax, dword ptr [rbp + 0x30c]
14095c8c5: 89 85 3c 02 00 00           	mov	dword ptr [rbp + 0x23c], eax
14095c8cb: 48 c7 85 28 02 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x228], 0x2
14095c8d6: 48 8b 85 a8 04 00 00        	mov	rax, qword ptr [rbp + 0x4a8]
14095c8dd: 48 8b 00                    	mov	rax, qword ptr [rax]
14095c8e0: 48 83 c0 2c                 	add	rax, 0x2c
14095c8e4: 48 89 85 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rax
14095c8eb: f3 44 0f 10 2d 7c 90 fc 01  	movss	xmm13, dword ptr [rip + 0x1fc907c] # 0x142925970
14095c8f4: f3 44 0f 10 35 d7 09 07 02  	movss	xmm14, dword ptr [rip + 0x20709d7] # 0x1429cd2d4
14095c8fd: 48 8d bd 80 02 00 00        	lea	rdi, [rbp + 0x280]
14095c904: 31 c0                       	xor	eax, eax
14095c906: eb 03                       	jmp	0x14095c90b <.text+0x95b90b>
14095c908: 4c 89 f0                    	mov	rax, r14
14095c90b: 48 83 f8 02                 	cmp	rax, 0x2
14095c90f: 0f 84 cd dc ff ff           	je	0x14095a5e2 <.text+0x9595e2>
14095c915: 4c 8d 70 01                 	lea	r14, [rax + 0x1]
14095c919: 4c 89 b5 20 02 00 00        	mov	qword ptr [rbp + 0x220], r14
14095c920: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14095c924: 48 8d 95 30 02 00 00        	lea	rdx, [rbp + 0x230]
14095c92b: 48 8b 0c 82                 	mov	rcx, qword ptr [rdx + 4*rax]
14095c92f: 48 89 8d b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rcx
14095c936: 8b 44 82 08                 	mov	eax, dword ptr [rdx + 4*rax + 0x8]
14095c93a: 89 85 b8 01 00 00           	mov	dword ptr [rbp + 0x1b8], eax
14095c940: f3 0f 10 85 b0 01 00 00     	movss	xmm0, dword ptr [rbp + 0x1b0]
14095c948: 66 0f 6e 8d b8 01 00 00     	movd	xmm1, dword ptr [rbp + 0x1b8]
14095c950: f3 0f 11 85 00 03 00 00     	movss	dword ptr [rbp + 0x300], xmm0
14095c958: 66 0f 7e 8d 04 03 00 00     	movd	dword ptr [rbp + 0x304], xmm1
14095c960: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095c967: f3 0f 10 50 4c              	movss	xmm2, dword ptr [rax + 0x4c]
14095c96c: 48 8d 8d 64 01 00 00        	lea	rcx, [rbp + 0x164]
14095c973: 48 89 da                    	mov	rdx, rbx
14095c976: e8 65 04 34 00              	call	0x140c9cde0 <.text+0xc9bde0>
14095c97b: 48 8b 95 78 03 00 00        	mov	rdx, qword ptr [rbp + 0x378]
14095c982: 8b 42 08                    	mov	eax, dword ptr [rdx + 0x8]
14095c985: 8d 0c c5 00 00 00 00        	lea	ecx, [8*rax]
14095c98c: f3 44 0f 10 4c 0a 0c        	movss	xmm9, dword ptr [rdx + rcx + 0xc]
14095c993: f3 44 0f 10 54 0a 10        	movss	xmm10, dword ptr [rdx + rcx + 0x10]
14095c99a: f3 44 0f 10 7a 4c           	movss	xmm15, dword ptr [rdx + 0x4c]
14095c9a0: f3 0f 10 42 30              	movss	xmm0, dword ptr [rdx + 0x30]
14095c9a5: 0f 57 f6                    	xorps	xmm6, xmm6
14095c9a8: f3 0f 10 0d d4 8f fc 01     	movss	xmm1, dword ptr [rip + 0x1fc8fd4] # 0x142925984
14095c9b0: 0f 2e c8                    	ucomiss	xmm1, xmm0
14095c9b3: 0f 83 e5 00 00 00           	jae	0x14095ca9e <.text+0x95ba9e>
14095c9b9: 84 c0                       	test	al, al
14095c9bb: f3 0f 10 2d 41 8f fc 01     	movss	xmm5, dword ptr [rip + 0x1fc8f41] # 0x142925904
14095c9c3: 74 43                       	je	0x14095ca08 <.text+0x95ba08>
14095c9c5: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095c9cc: f3 0f 10 50 1c              	movss	xmm2, dword ptr [rax + 0x1c]
14095c9d1: f3 0f 10 58 20              	movss	xmm3, dword ptr [rax + 0x20]
14095c9d6: 0f 28 ca                    	movaps	xmm1, xmm2
14095c9d9: f3 0f c2 ca 03              	cmpunordss	xmm1, xmm2
14095c9de: 0f 28 e1                    	movaps	xmm4, xmm1
14095c9e1: 0f 54 e3                    	andps	xmm4, xmm3
14095c9e4: f3 0f 5f da                 	maxss	xmm3, xmm2
14095c9e8: 0f 55 cb                    	andnps	xmm1, xmm3
14095c9eb: 0f 56 cc                    	orps	xmm1, xmm4
14095c9ee: f3 0f 58 0d 8e 86 14 02     	addss	xmm1, dword ptr [rip + 0x214868e] # 0x142aa5084
14095c9f6: f3 0f 5e 0d 8a 86 14 02     	divss	xmm1, dword ptr [rip + 0x214868a] # 0x142aa5088
14095c9fe: f3 0f 10 15 7e f0 11 02     	movss	xmm2, dword ptr [rip + 0x211f07e] # 0x142a7ba84
14095ca06: eb 24                       	jmp	0x14095ca2c <.text+0x95ba2c>
14095ca08: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095ca0f: f3 0f 10 48 14              	movss	xmm1, dword ptr [rax + 0x14]
14095ca14: f3 0f 58 0d 70 86 14 02     	addss	xmm1, dword ptr [rip + 0x2148670] # 0x142aa508c
14095ca1c: f3 0f 5e 0d 6c 86 14 02     	divss	xmm1, dword ptr [rip + 0x214866c] # 0x142aa5090
14095ca24: f3 0f 10 15 d8 7b fd 01     	movss	xmm2, dword ptr [rip + 0x1fd7bd8] # 0x142934604
14095ca2c: 41 0f 28 e5                 	movaps	xmm4, xmm13
14095ca30: f3 41 0f 5e cd              	divss	xmm1, xmm13
14095ca35: 0f 57 db                    	xorps	xmm3, xmm3
14095ca38: f3 0f 5f d9                 	maxss	xmm3, xmm1
14095ca3c: 0f 28 cd                    	movaps	xmm1, xmm5
14095ca3f: f3 0f 5d cb                 	minss	xmm1, xmm3
14095ca43: 0f 28 d9                    	movaps	xmm3, xmm1
14095ca46: f3 0f 59 d9                 	mulss	xmm3, xmm1
14095ca4a: f3 0f 58 c9                 	addss	xmm1, xmm1
14095ca4e: f3 0f 10 35 22 06 07 02     	movss	xmm6, dword ptr [rip + 0x2070622] # 0x1429cd078
14095ca56: f3 0f 5c f1                 	subss	xmm6, xmm1
14095ca5a: f3 0f 59 f3                 	mulss	xmm6, xmm3
14095ca5e: 0f 28 cd                    	movaps	xmm1, xmm5
14095ca61: f3 0f 5c ce                 	subss	xmm1, xmm6
14095ca65: f3 0f 59 ca                 	mulss	xmm1, xmm2
14095ca69: f3 41 0f 59 f6              	mulss	xmm6, xmm14
14095ca6e: f3 0f 58 f1                 	addss	xmm6, xmm1
14095ca72: 0f 2e 35 17 3c 00 02        	ucomiss	xmm6, dword ptr [rip + 0x2003c17] # 0x142960690
14095ca79: 0f 82 10 06 00 00           	jb	0x14095d08f <.text+0x95c08f>
14095ca7f: 0f 28 cd                    	movaps	xmm1, xmm5
14095ca82: f3 0f 5c c8                 	subss	xmm1, xmm0
14095ca86: f3 0f 59 cc                 	mulss	xmm1, xmm4
14095ca8a: f3 41 0f 59 c6              	mulss	xmm0, xmm14
14095ca8f: f3 0f 58 c1                 	addss	xmm0, xmm1
14095ca93: 0f 57 c9                    	xorps	xmm1, xmm1
14095ca96: f3 0f 5f c8                 	maxss	xmm1, xmm0
14095ca9a: f3 0f 5d f1                 	minss	xmm6, xmm1
14095ca9e: f3 44 0f 10 85 64 01 00 00  	movss	xmm8, dword ptr [rbp + 0x164]
14095caa7: f3 0f 10 bd 68 01 00 00     	movss	xmm7, dword ptr [rbp + 0x168]
14095caaf: f3 44 0f 10 a5 6c 01 00 00  	movss	xmm12, dword ptr [rbp + 0x16c]
14095cab8: f3 0f 10 85 b0 01 00 00     	movss	xmm0, dword ptr [rbp + 0x1b0]
14095cac0: 66 0f 6e 8d b8 01 00 00     	movd	xmm1, dword ptr [rbp + 0x1b8]
14095cac8: f3 0f 11 85 00 02 00 00     	movss	dword ptr [rbp + 0x200], xmm0
14095cad0: 66 0f 7e 8d 04 02 00 00     	movd	dword ptr [rbp + 0x204], xmm1
14095cad8: f3 44 0f 11 8d 80 02 00 00  	movss	dword ptr [rbp + 0x280], xmm9
14095cae1: f3 44 0f 11 95 84 02 00 00  	movss	dword ptr [rbp + 0x284], xmm10
14095caea: 48 89 d9                    	mov	rcx, rbx
14095caed: 48 8b 95 78 03 00 00        	mov	rdx, qword ptr [rbp + 0x378]
14095caf4: e8 27 69 f8 ff              	call	0x1408e3420 <.text+0x8e2420>
14095caf9: 48 89 9d c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rbx
14095cb00: 48 8d 85 00 02 00 00        	lea	rax, [rbp + 0x200]
14095cb07: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095cb0e: 48 89 bd d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rdi
14095cb15: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095cb1c: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
14095cb23: 48 c7 85 e0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2e0], 0x0
14095cb2e: 48 c7 85 e8 02 00 00 05 00 00 00    	mov	qword ptr [rbp + 0x2e8], 0x5
14095cb39: 48 8d 8d e0 01 00 00        	lea	rcx, [rbp + 0x1e0]
14095cb40: 48 8d 95 c0 02 00 00        	lea	rdx, [rbp + 0x2c0]
14095cb47: 4c 8d 05 1a 80 15 02        	lea	r8, [rip + 0x215801a]   # 0x142ab4b68
14095cb4e: e8 fd 2a 9a 00              	call	0x1412ff650 <.text+0x12fe650>
14095cb53: 48 8b 95 00 03 00 00        	mov	rdx, qword ptr [rbp + 0x300]
14095cb5a: 48 85 d2                    	test	rdx, rdx
14095cb5d: 74 16                       	je	0x14095cb75 <.text+0x95bb75>
14095cb5f: 48 8b 8d 08 03 00 00        	mov	rcx, qword ptr [rbp + 0x308]
14095cb66: 48 c1 e2 03                 	shl	rdx, 0x3
14095cb6a: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095cb70: e8 ab 70 cb ff              	call	0x140613c20 <.text+0x612c20>
14095cb75: 48 8b 95 18 03 00 00        	mov	rdx, qword ptr [rbp + 0x318]
14095cb7c: 48 85 d2                    	test	rdx, rdx
14095cb7f: 74 16                       	je	0x14095cb97 <.text+0x95bb97>
14095cb81: 48 8b 8d 20 03 00 00        	mov	rcx, qword ptr [rbp + 0x320]
14095cb88: 48 c1 e2 02                 	shl	rdx, 0x2
14095cb8c: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095cb92: e8 89 70 cb ff              	call	0x140613c20 <.text+0x612c20>
14095cb97: 48 8b 85 a0 04 00 00        	mov	rax, qword ptr [rbp + 0x4a0]
14095cb9e: 4c 8b 20                    	mov	r12, qword ptr [rax]
14095cba1: 48 8d 85 b0 01 00 00        	lea	rax, [rbp + 0x1b0]
14095cba8: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
14095cbaf: 48 8d 05 1a ec 3e 00        	lea	rax, [rip + 0x3eec1a]   # 0x140d4b7d0 <.text+0xd4a7d0>
14095cbb6: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095cbbd: 48 8d 05 bc 84 15 02        	lea	rax, [rip + 0x21584bc]  # 0x142ab5080
14095cbc4: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095cbcb: 48 c7 85 08 03 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x308], 0x1
14095cbd6: 48 8d 85 c0 02 00 00        	lea	rax, [rbp + 0x2c0]
14095cbdd: 48 89 85 10 03 00 00        	mov	qword ptr [rbp + 0x310], rax
14095cbe4: 48 c7 85 18 03 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x318], 0x1
14095cbef: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095cbfa: c6 85 57 03 00 00 01        	mov	byte ptr [rbp + 0x357], 0x1
14095cc01: 48 8d 4d f8                 	lea	rcx, [rbp - 0x8]
14095cc05: 48 89 da                    	mov	rdx, rbx
14095cc08: e8 83 d3 41 00              	call	0x140d79f90 <.text+0xd78f90>
14095cc0d: 0f 10 45 f8                 	movups	xmm0, xmmword ptr [rbp - 0x8]
14095cc11: 0f 29 85 80 02 00 00        	movaps	xmmword ptr [rbp + 0x280], xmm0
14095cc18: 48 8b 45 08                 	mov	rax, qword ptr [rbp + 0x8]
14095cc1c: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
14095cc23: 48 89 bd c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rdi
14095cc2a: 48 8d 05 9f ee 6d ff        	lea	rax, [rip - 0x921161]   # 0x14003bad0 <.text+0x3aad0>
14095cc31: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095cc38: 48 8d 85 b8 02 00 00        	lea	rax, [rbp + 0x2b8]
14095cc3f: 48 89 85 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rax
14095cc46: 48 8d 05 73 82 1d 00        	lea	rax, [rip + 0x1d8273]   # 0x140b34ec0 <.text+0xb33ec0>
14095cc4d: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
14095cc54: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
14095cc5b: 48 89 85 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rax
14095cc62: 48 8d 05 f7 08 43 00        	lea	rax, [rip + 0x4308f7]   # 0x140d8d560 <.text+0xd8c560>
14095cc69: 48 89 85 e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], rax
14095cc70: 48 8d 05 d9 82 15 02        	lea	rax, [rip + 0x21582d9]  # 0x142ab4f50
14095cc77: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095cc7e: 48 c7 85 08 03 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x308], 0x3
14095cc89: 48 8d 85 c0 02 00 00        	lea	rax, [rbp + 0x2c0]
14095cc90: 48 89 85 10 03 00 00        	mov	qword ptr [rbp + 0x310], rax
14095cc97: 48 c7 85 18 03 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x318], 0x3
14095cca2: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095ccad: 48 8d 8d 00 02 00 00        	lea	rcx, [rbp + 0x200]
14095ccb4: 48 89 da                    	mov	rdx, rbx
14095ccb7: e8 d4 d2 41 00              	call	0x140d79f90 <.text+0xd78f90>
14095ccbc: 4c 89 b5 d8 01 00 00        	mov	qword ptr [rbp + 0x1d8], r14
14095ccc3: 48 8b 95 80 02 00 00        	mov	rdx, qword ptr [rbp + 0x280]
14095ccca: 48 85 d2                    	test	rdx, rdx
14095cccd: 74 12                       	je	0x14095cce1 <.text+0x95bce1>
14095cccf: 48 8b 8d 88 02 00 00        	mov	rcx, qword ptr [rbp + 0x288]
14095ccd6: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095ccdc: e8 3f 6f cb ff              	call	0x140613c20 <.text+0x612c20>
14095cce1: 4c 89 a5 a8 02 00 00        	mov	qword ptr [rbp + 0x2a8], r12
14095cce8: 48 8b 85 00 02 00 00        	mov	rax, qword ptr [rbp + 0x200]
14095ccef: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
14095ccf6: 48 8b 85 08 02 00 00        	mov	rax, qword ptr [rbp + 0x208]
14095ccfd: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
14095cd04: 4c 8b ad 10 02 00 00        	mov	r13, qword ptr [rbp + 0x210]
14095cd0b: 4c 8b bd e0 01 00 00        	mov	r15, qword ptr [rbp + 0x1e0]
14095cd12: 48 8b b5 e8 01 00 00        	mov	rsi, qword ptr [rbp + 0x1e8]
14095cd19: 4c 8b b5 f0 01 00 00        	mov	r14, qword ptr [rbp + 0x1f0]
14095cd20: 0f b6 05 e2 7e cd 02        	movzx	eax, byte ptr [rip + 0x2cd7ee2] # 0x143634c09
14095cd27: b9 30 00 00 00              	mov	ecx, 0x30
14095cd2c: ba 08 00 00 00              	mov	edx, 0x8
14095cd31: e8 da 6e cb ff              	call	0x140613c10 <.text+0x612c10>
14095cd36: 48 85 c0                    	test	rax, rax
14095cd39: 0f 84 ca 05 00 00           	je	0x14095d309 <.text+0x95c309>
14095cd3f: 48 89 c1                    	mov	rcx, rax
14095cd42: f3 44 0f 58 fe              	addss	xmm15, xmm6
14095cd47: f3 0f 59 35 41 39 00 02     	mulss	xmm6, dword ptr [rip + 0x2003941] # 0x142960690
14095cd4f: f3 44 0f 58 ce              	addss	xmm9, xmm6
14095cd54: f3 45 0f 5c c1              	subss	xmm8, xmm9
14095cd59: f3 41 0f 5c ff              	subss	xmm7, xmm15
14095cd5e: f3 44 0f 58 d6              	addss	xmm10, xmm6
14095cd63: f3 45 0f 5c e2              	subss	xmm12, xmm10
14095cd68: f3 45 0f 59 c0              	mulss	xmm8, xmm8
14095cd6d: f3 0f 59 ff                 	mulss	xmm7, xmm7
14095cd71: f3 41 0f 58 f8              	addss	xmm7, xmm8
14095cd76: f3 45 0f 59 e4              	mulss	xmm12, xmm12
14095cd7b: f3 44 0f 58 e7              	addss	xmm12, xmm7
14095cd80: 0f 57 c0                    	xorps	xmm0, xmm0
14095cd83: f3 41 0f 51 c4              	sqrtss	xmm0, xmm12
14095cd88: 48 c7 00 01 00 00 00        	mov	qword ptr [rax], 0x1
14095cd8f: 48 c7 40 08 01 00 00 00     	mov	qword ptr [rax + 0x8], 0x1
14095cd97: 4c 89 78 10                 	mov	qword ptr [rax + 0x10], r15
14095cd9b: 48 89 70 18                 	mov	qword ptr [rax + 0x18], rsi
14095cd9f: 4c 89 70 20                 	mov	qword ptr [rax + 0x20], r14
14095cda3: f3 44 0f 11 58 28           	movss	dword ptr [rax + 0x28], xmm11
14095cda9: f3 0f 11 40 2c              	movss	dword ptr [rax + 0x2c], xmm0
14095cdae: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095cdb5: 48 8d 05 d4 82 15 02        	lea	rax, [rip + 0x21582d4]  # 0x142ab5090
14095cdbc: 48 89 85 08 03 00 00        	mov	qword ptr [rbp + 0x308], rax
14095cdc3: 4d 85 ed                    	test	r13, r13
14095cdc6: 48 89 8d 58 03 00 00        	mov	qword ptr [rbp + 0x358], rcx
14095cdcd: 0f 88 55 05 00 00           	js	0x14095d328 <.text+0x95c328>
14095cdd3: 49 89 dc                    	mov	r12, rbx
14095cdd6: 74 27                       	je	0x14095cdff <.text+0x95bdff>
14095cdd8: 0f b6 05 2a 7e cd 02        	movzx	eax, byte ptr [rip + 0x2cd7e2a] # 0x143634c09
14095cddf: bb 01 00 00 00              	mov	ebx, 0x1
14095cde4: ba 01 00 00 00              	mov	edx, 0x1
14095cde9: 4c 89 e9                    	mov	rcx, r13
14095cdec: e8 1f 6e cb ff              	call	0x140613c10 <.text+0x612c10>
14095cdf1: 48 85 c0                    	test	rax, rax
14095cdf4: 0f 84 30 05 00 00           	je	0x14095d32a <.text+0x95c32a>
14095cdfa: 49 89 c6                    	mov	r14, rax
14095cdfd: eb 06                       	jmp	0x14095ce05 <.text+0x95be05>
14095cdff: 41 be 01 00 00 00           	mov	r14d, 0x1
14095ce05: 4c 89 f1                    	mov	rcx, r14
14095ce08: 48 8b 95 60 03 00 00        	mov	rdx, qword ptr [rbp + 0x360]
14095ce0f: 4d 89 e8                    	mov	r8, r13
14095ce12: e8 e9 6d fc 01              	call	0x142923c00 <.text+0x2922c00>
14095ce17: 4c 89 ad 00 03 00 00        	mov	qword ptr [rbp + 0x300], r13
14095ce1e: 4c 89 b5 08 03 00 00        	mov	qword ptr [rbp + 0x308], r14
14095ce25: 4c 89 ad 10 03 00 00        	mov	qword ptr [rbp + 0x310], r13
14095ce2c: 48 8b 85 58 03 00 00        	mov	rax, qword ptr [rbp + 0x358]
14095ce33: 48 89 85 18 03 00 00        	mov	qword ptr [rbp + 0x318], rax
14095ce3a: 48 8d 05 4f 82 15 02        	lea	rax, [rip + 0x215824f]  # 0x142ab5090
14095ce41: 48 89 85 20 03 00 00        	mov	qword ptr [rbp + 0x320], rax
14095ce48: 48 8b 8d a8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2a8]
14095ce4f: 4c 89 e3                    	mov	rbx, r12
14095ce52: 4c 89 e2                    	mov	rdx, r12
14095ce55: e8 26 fa b0 00              	call	0x14146c880 <.text+0x146b880>
14095ce5a: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095ce61: 48 85 d2                    	test	rdx, rdx
14095ce64: 4c 8b b5 d8 01 00 00        	mov	r14, qword ptr [rbp + 0x1d8]
14095ce6b: 0f 84 97 fa ff ff           	je	0x14095c908 <.text+0x95b908>
14095ce71: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095ce77: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
14095ce7e: e8 9d 6d cb ff              	call	0x140613c20 <.text+0x612c20>
14095ce83: e9 80 fa ff ff              	jmp	0x14095c908 <.text+0x95b908>
14095ce88: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095ce8f: 80 78 3c 00                 	cmp	byte ptr [rax + 0x3c], 0x0
14095ce93: 74 27                       	je	0x14095cebc <.text+0x95bebc>
14095ce95: 48 8d 8d e0 01 00 00        	lea	rcx, [rbp + 0x1e0]
14095ce9c: 31 d2                       	xor	edx, edx
14095ce9e: e8 5d c7 33 00              	call	0x140c99600 <.text+0xc98600>
14095cea3: 0f 28 f0                    	movaps	xmm6, xmm0
14095cea6: 0f 28 f9                    	movaps	xmm7, xmm1
14095cea9: 48 8d 8d e0 01 00 00        	lea	rcx, [rbp + 0x1e0]
14095ceb0: ba 01 00 00 00              	mov	edx, 0x1
14095ceb5: e8 46 c7 33 00              	call	0x140c99600 <.text+0xc98600>
14095ceba: eb 25                       	jmp	0x14095cee1 <.text+0x95bee1>
14095cebc: 48 8d 8d e0 01 00 00        	lea	rcx, [rbp + 0x1e0]
14095cec3: 31 d2                       	xor	edx, edx
14095cec5: e8 36 c7 33 00              	call	0x140c99600 <.text+0xc98600>
14095ceca: 0f 28 f0                    	movaps	xmm6, xmm0
14095cecd: 0f 28 f9                    	movaps	xmm7, xmm1
14095ced0: 48 8d 8d e0 01 00 00        	lea	rcx, [rbp + 0x1e0]
14095ced7: ba 03 00 00 00              	mov	edx, 0x3
14095cedc: e8 1f c7 33 00              	call	0x140c99600 <.text+0xc98600>
14095cee1: 44 0f 28 c0                 	movaps	xmm8, xmm0
14095cee5: 44 0f 28 c9                 	movaps	xmm9, xmm1
14095cee9: 48 8b 8d 78 03 00 00        	mov	rcx, qword ptr [rbp + 0x378]
14095cef0: f3 44 0f 10 61 4c           	movss	xmm12, dword ptr [rcx + 0x4c]
14095cef6: e8 65 63 f8 ff              	call	0x1408e3260 <.text+0x8e2260>
14095cefb: 44 0f 28 d0                 	movaps	xmm10, xmm0
14095ceff: 48 8b 85 a0 04 00 00        	mov	rax, qword ptr [rbp + 0x4a0]
14095cf06: 48 8b 38                    	mov	rdi, qword ptr [rax]
14095cf09: 48 8d 85 b8 02 00 00        	lea	rax, [rbp + 0x2b8]
14095cf10: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
14095cf17: 48 8d 05 a2 7f 1d 00        	lea	rax, [rip + 0x1d7fa2]   # 0x140b34ec0 <.text+0xb33ec0>
14095cf1e: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
14095cf25: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
14095cf2c: 48 89 85 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rax
14095cf33: 48 8d 05 26 06 43 00        	lea	rax, [rip + 0x430626]   # 0x140d8d560 <.text+0xd8c560>
14095cf3a: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
14095cf41: 48 8d 05 c0 81 15 02        	lea	rax, [rip + 0x21581c0]  # 0x142ab5108
14095cf48: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095cf4f: 48 c7 85 08 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x308], 0x2
14095cf5a: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
14095cf65: 48 8d 85 c0 02 00 00        	lea	rax, [rbp + 0x2c0]
14095cf6c: 48 89 85 10 03 00 00        	mov	qword ptr [rbp + 0x310], rax
14095cf73: 48 c7 85 18 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x318], 0x2
14095cf7e: 48 8d 4d 58                 	lea	rcx, [rbp + 0x58]
14095cf82: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
14095cf89: e8 12 cb ff ff              	call	0x140959aa0 <.text+0x958aa0>
14095cf8e: f3 45 0f 58 e2              	addss	xmm12, xmm10
14095cf93: 48 8b 45 58                 	mov	rax, qword ptr [rbp + 0x58]
14095cf97: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
14095cf9e: 48 8b 45 60                 	mov	rax, qword ptr [rbp + 0x60]
14095cfa2: 48 89 85 78 03 00 00        	mov	qword ptr [rbp + 0x378], rax
14095cfa9: 48 8b 75 68                 	mov	rsi, qword ptr [rbp + 0x68]
14095cfad: f3 0f 11 b5 00 03 00 00     	movss	dword ptr [rbp + 0x300], xmm6
14095cfb5: f3 0f 11 bd 04 03 00 00     	movss	dword ptr [rbp + 0x304], xmm7
14095cfbd: f3 44 0f 11 a5 10 03 00 00  	movss	dword ptr [rbp + 0x310], xmm12
14095cfc6: f3 44 0f 11 85 08 03 00 00  	movss	dword ptr [rbp + 0x308], xmm8
14095cfcf: f3 44 0f 11 8d 0c 03 00 00  	movss	dword ptr [rbp + 0x30c], xmm9
14095cfd8: f3 44 0f 11 9d 14 03 00 00  	movss	dword ptr [rbp + 0x314], xmm11
14095cfe1: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095cfe8: e8 53 cc ff ff              	call	0x140959c40 <.text+0x958c40>
14095cfed: 48 8d 0d 34 81 15 02        	lea	rcx, [rip + 0x2158134]  # 0x142ab5128
14095cff4: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
14095cff9: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095d000: 48 8b 95 78 03 00 00        	mov	rdx, qword ptr [rbp + 0x378]
14095d007: 49 89 f0                    	mov	r8, rsi
14095d00a: 49 89 c1                    	mov	r9, rax
14095d00d: e8 9e 08 f4 ff              	call	0x14089d8b0 <.text+0x89c8b0>
14095d012: 48 8b 85 20 03 00 00        	mov	rax, qword ptr [rbp + 0x320]
14095d019: 48 89 85 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rax
14095d020: 0f 10 85 00 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x300]
14095d027: 0f 10 8d 10 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x310]
14095d02e: 0f 29 8d d0 02 00 00        	movaps	xmmword ptr [rbp + 0x2d0], xmm1
14095d035: 0f 29 85 c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm0
14095d03c: 48 8d 95 c0 02 00 00        	lea	rdx, [rbp + 0x2c0]
14095d043: 48 89 f9                    	mov	rcx, rdi
14095d046: e8 35 f8 b0 00              	call	0x14146c880 <.text+0x146b880>
14095d04b: 48 8b 95 60 03 00 00        	mov	rdx, qword ptr [rbp + 0x360]
14095d052: 48 85 d2                    	test	rdx, rdx
14095d055: 0f 84 87 d5 ff ff           	je	0x14095a5e2 <.text+0x9595e2>
14095d05b: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095d061: 48 8b 8d 78 03 00 00        	mov	rcx, qword ptr [rbp + 0x378]
14095d068: e8 b3 6b cb ff              	call	0x140613c20 <.text+0x612c20>
14095d06d: e9 70 d5 ff ff              	jmp	0x14095a5e2 <.text+0x9595e2>
14095d072: 48 8d 0d 1a d1 17 02        	lea	rcx, [rip + 0x217d11a]  # 0x142ada193
14095d079: 4c 8d 05 58 7d 15 02        	lea	r8, [rip + 0x2157d58]   # 0x142ab4dd8
14095d080: ba 32 00 00 00              	mov	edx, 0x32
14095d085: e8 a6 c3 f7 01              	call	0x1428d9430 <.text+0x28d8430>
14095d08a: e9 fc 02 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d08f: 4c 8d 05 fa 80 14 02        	lea	r8, [rip + 0x21480fa]   # 0x142aa5190
14095d096: 0f 57 c0                    	xorps	xmm0, xmm0
14095d099: 0f 28 ce                    	movaps	xmm1, xmm6
14095d09c: e8 df e3 42 00              	call	0x140d8b480 <.text+0xd8a480>
14095d0a1: e9 e5 02 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d0a6: 4c 8d 05 e3 80 14 02        	lea	r8, [rip + 0x21480e3]   # 0x142aa5190
14095d0ad: 0f 57 c0                    	xorps	xmm0, xmm0
14095d0b0: e8 cb e3 42 00              	call	0x140d8b480 <.text+0xd8a480>
14095d0b5: e9 d1 02 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d0ba: 48 c7 85 90 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x190], 0x0
14095d0c5: 4c 8d 05 54 be 13 02        	lea	r8, [rip + 0x213be54]   # 0x142a98f20
14095d0cc: 48 8b 8d 90 01 00 00        	mov	rcx, qword ptr [rbp + 0x190]
14095d0d3: 48 89 fa                    	mov	rdx, rdi
14095d0d6: e8 08 bf f7 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14095d0db: e9 ab 02 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d0e0: b9 08 00 00 00              	mov	ecx, 0x8
14095d0e5: ba 30 00 00 00              	mov	edx, 0x30
14095d0ea: e8 11 bf f7 01              	call	0x1428d9000 <.text+0x28d8000>
14095d0ef: e9 97 02 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d0f4: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095d0fb: 4c 8d 05 56 65 30 02        	lea	r8, [rip + 0x2306556]   # 0x142c63658
14095d102: e8 11 c4 f7 01              	call	0x1428d9518 <.text+0x28d8518>
14095d107: e9 7f 02 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d10c: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095d113: 4c 8d 05 56 65 30 02        	lea	r8, [rip + 0x2306556]   # 0x142c63670
14095d11a: 48 89 c1                    	mov	rcx, rax
14095d11d: e8 f6 c3 f7 01              	call	0x1428d9518 <.text+0x28d8518>
14095d122: e9 64 02 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d127: b9 08 00 00 00              	mov	ecx, 0x8
14095d12c: ba 28 00 00 00              	mov	edx, 0x28
14095d131: e8 ca be f7 01              	call	0x1428d9000 <.text+0x28d8000>
14095d136: e9 50 02 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d13b: 45 31 e4                    	xor	r12d, r12d
14095d13e: 4c 8d 05 db bd 13 02        	lea	r8, [rip + 0x213bddb]   # 0x142a98f20
14095d145: 4c 89 e1                    	mov	rcx, r12
14095d148: 48 89 da                    	mov	rdx, rbx
14095d14b: e8 93 be f7 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14095d150: e9 36 02 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d155: b9 08 00 00 00              	mov	ecx, 0x8
14095d15a: ba 28 00 00 00              	mov	edx, 0x28
14095d15f: e8 9c be f7 01              	call	0x1428d9000 <.text+0x28d8000>
14095d164: e9 22 02 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d169: 45 31 ed                    	xor	r13d, r13d
14095d16c: 4c 8d 05 ad bd 13 02        	lea	r8, [rip + 0x213bdad]   # 0x142a98f20
14095d173: 4c 89 e9                    	mov	rcx, r13
14095d176: 4c 89 fa                    	mov	rdx, r15
14095d179: e8 65 be f7 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14095d17e: e9 08 02 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d183: 4c 89 ad 78 03 00 00        	mov	qword ptr [rbp + 0x378], r13
14095d18a: 4c 89 a5 60 03 00 00        	mov	qword ptr [rbp + 0x360], r12
14095d191: b9 08 00 00 00              	mov	ecx, 0x8
14095d196: ba 30 00 00 00              	mov	edx, 0x30
14095d19b: e8 60 be f7 01              	call	0x1428d9000 <.text+0x28d8000>
14095d1a0: e9 e6 01 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d1a5: 4c 89 bd 78 02 00 00        	mov	qword ptr [rbp + 0x278], r15
14095d1ac: 45 31 ff                    	xor	r15d, r15d
14095d1af: 4c 8d 05 6a bd 13 02        	lea	r8, [rip + 0x213bd6a]   # 0x142a98f20
14095d1b6: 4c 89 f9                    	mov	rcx, r15
14095d1b9: 48 89 da                    	mov	rdx, rbx
14095d1bc: e8 22 be f7 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14095d1c1: e9 c5 01 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d1c6: 4c 8d 05 ab b9 2d 02        	lea	r8, [rip + 0x22db9ab]   # 0x142c38b78
14095d1cd: 4c 89 f9                    	mov	rcx, r15
14095d1d0: 48 89 da                    	mov	rdx, rbx
14095d1d3: e8 0b be f7 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14095d1d8: e9 ae 01 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d1dd: 48 8d 05 bc dc 18 02        	lea	rax, [rip + 0x218dcbc]  # 0x142aeaea0
14095d1e4: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
14095d1eb: 48 c7 85 08 03 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x308], 0x1
14095d1f6: 48 c7 85 10 03 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x310], 0x8
14095d201: 0f 57 c0                    	xorps	xmm0, xmm0
14095d204: 0f 11 85 18 03 00 00        	movups	xmmword ptr [rbp + 0x318], xmm0
14095d20b: c6 85 86 03 00 00 00        	mov	byte ptr [rbp + 0x386], 0x0
14095d212: 48 8d 15 97 dc 18 02        	lea	rdx, [rip + 0x218dc97]  # 0x142aeaeb0
14095d219: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095d220: e8 6b c1 f7 01              	call	0x1428d9390 <.text+0x28d8390>
14095d225: e9 61 01 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d22a: 48 89 bd 78 03 00 00        	mov	qword ptr [rbp + 0x378], rdi
14095d231: 4c 89 b5 60 03 00 00        	mov	qword ptr [rbp + 0x360], r14
14095d238: b9 08 00 00 00              	mov	ecx, 0x8
14095d23d: ba 30 00 00 00              	mov	edx, 0x30
14095d242: e8 b9 bd f7 01              	call	0x1428d9000 <.text+0x28d8000>
14095d247: e9 3f 01 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d24c: 4c 89 bd b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], r15
14095d253: 45 31 ff                    	xor	r15d, r15d
14095d256: 4c 8d 05 c3 bc 13 02        	lea	r8, [rip + 0x213bcc3]   # 0x142a98f20
14095d25d: 4c 89 f9                    	mov	rcx, r15
14095d260: 48 89 da                    	mov	rdx, rbx
14095d263: e8 7b bd f7 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14095d268: e9 1e 01 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d26d: c6 85 85 03 00 00 00        	mov	byte ptr [rbp + 0x385], 0x0
14095d274: 48 8d 0d 45 7c 15 02        	lea	rcx, [rip + 0x2157c45]  # 0x142ab4ec0
14095d27b: e8 90 c0 f7 01              	call	0x1428d9310 <.text+0x28d8310>
14095d280: e9 06 01 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d285: 48 89 bd 78 03 00 00        	mov	qword ptr [rbp + 0x378], rdi
14095d28c: 4c 89 a5 60 03 00 00        	mov	qword ptr [rbp + 0x360], r12
14095d293: b9 08 00 00 00              	mov	ecx, 0x8
14095d298: ba 30 00 00 00              	mov	edx, 0x30
14095d29d: e8 5e bd f7 01              	call	0x1428d9000 <.text+0x28d8000>
14095d2a2: e9 e4 00 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d2a7: 45 31 ff                    	xor	r15d, r15d
14095d2aa: 4c 8d 05 6f bc 13 02        	lea	r8, [rip + 0x213bc6f]   # 0x142a98f20
14095d2b1: 4c 89 f9                    	mov	rcx, r15
14095d2b4: 4c 89 f2                    	mov	rdx, r14
14095d2b7: e8 27 bd f7 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14095d2bc: e9 ca 00 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d2c1: c6 85 85 03 00 00 00        	mov	byte ptr [rbp + 0x385], 0x0
14095d2c8: 48 8d 0d 09 7c 15 02        	lea	rcx, [rip + 0x2157c09]  # 0x142ab4ed8
14095d2cf: e8 3c c0 f7 01              	call	0x1428d9310 <.text+0x28d8310>
14095d2d4: e9 b2 00 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d2d9: c6 85 85 03 00 00 00        	mov	byte ptr [rbp + 0x385], 0x0
14095d2e0: 48 8d 0d 21 7c 15 02        	lea	rcx, [rip + 0x2157c21]  # 0x142ab4f08
14095d2e7: e8 24 c0 f7 01              	call	0x1428d9310 <.text+0x28d8310>
14095d2ec: e9 9a 00 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d2f1: c6 85 85 03 00 00 00        	mov	byte ptr [rbp + 0x385], 0x0
14095d2f8: 48 8d 0d 21 7c 15 02        	lea	rcx, [rip + 0x2157c21]  # 0x142ab4f20
14095d2ff: e8 0c c0 f7 01              	call	0x1428d9310 <.text+0x28d8310>
14095d304: e9 82 00 00 00              	jmp	0x14095d38b <.text+0x95c38b>
14095d309: 4c 89 bd 78 03 00 00        	mov	qword ptr [rbp + 0x378], r15
14095d310: 48 89 b5 58 03 00 00        	mov	qword ptr [rbp + 0x358], rsi
14095d317: b9 08 00 00 00              	mov	ecx, 0x8
14095d31c: ba 30 00 00 00              	mov	edx, 0x30
14095d321: e8 da bc f7 01              	call	0x1428d9000 <.text+0x28d8000>
14095d326: eb 63                       	jmp	0x14095d38b <.text+0x95c38b>
14095d328: 31 db                       	xor	ebx, ebx
14095d32a: 4c 8d 05 ef bb 13 02        	lea	r8, [rip + 0x213bbef]   # 0x142a98f20
14095d331: 48 89 d9                    	mov	rcx, rbx
14095d334: 4c 89 ea                    	mov	rdx, r13
14095d337: e8 a7 bc f7 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14095d33c: eb 4d                       	jmp	0x14095d38b <.text+0x95c38b>
14095d33e: 4c 89 a5 78 03 00 00        	mov	qword ptr [rbp + 0x378], r12
14095d345: 48 89 bd 58 03 00 00        	mov	qword ptr [rbp + 0x358], rdi
14095d34c: b9 08 00 00 00              	mov	ecx, 0x8
14095d351: ba 30 00 00 00              	mov	edx, 0x30
14095d356: e8 a5 bc f7 01              	call	0x1428d9000 <.text+0x28d8000>
14095d35b: eb 2e                       	jmp	0x14095d38b <.text+0x95c38b>
14095d35d: 31 db                       	xor	ebx, ebx
14095d35f: 4c 8d 05 ba bb 13 02        	lea	r8, [rip + 0x213bbba]   # 0x142a98f20
14095d366: 48 89 d9                    	mov	rcx, rbx
14095d369: 4c 89 fa                    	mov	rdx, r15
14095d36c: e8 72 bc f7 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14095d371: eb 18                       	jmp	0x14095d38b <.text+0x95c38b>
14095d373: 48 8d 0d 16 ba 17 02        	lea	rcx, [rip + 0x217ba16]  # 0x142ad8d90
14095d37a: 4c 8d 05 8f ba 17 02        	lea	r8, [rip + 0x217ba8f]   # 0x142ad8e10
14095d381: ba 28 00 00 00              	mov	edx, 0x28
14095d386: e8 a5 c0 f7 01              	call	0x1428d9430 <.text+0x28d8430>
14095d38b: 0f 0b                       	ud2
14095d38d: 0f 1f 00                    	nop	dword ptr [rax]
14095d390: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095d395: 55                          	push	rbp
14095d396: 41 57                       	push	r15
14095d398: 41 56                       	push	r14
14095d39a: 41 55                       	push	r13
14095d39c: 41 54                       	push	r12
14095d39e: 56                          	push	rsi
14095d39f: 57                          	push	rdi
14095d3a0: 53                          	push	rbx
14095d3a1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095d3a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095d3af: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095d3b5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095d3bb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095d3c1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095d3c7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095d3cd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095d3d6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095d3df: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095d3e8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095d3f0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095d3f8: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095d3ff: e8 ec 50 c7 ff              	call	0x1405d24f0 <.text+0x5d14f0>
14095d404: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095d40c: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095d414: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095d41d: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095d426: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095d42f: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095d435: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095d43b: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095d441: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095d447: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095d44d: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095d454: 5b                          	pop	rbx
14095d455: 5f                          	pop	rdi
14095d456: 5e                          	pop	rsi
14095d457: 41 5c                       	pop	r12
14095d459: 41 5d                       	pop	r13
14095d45b: 41 5e                       	pop	r14
14095d45d: 41 5f                       	pop	r15
14095d45f: 5d                          	pop	rbp
14095d460: c3                          	ret
14095d461: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
14095d470: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095d475: 55                          	push	rbp
14095d476: 41 57                       	push	r15
14095d478: 41 56                       	push	r14
14095d47a: 41 55                       	push	r13
14095d47c: 41 54                       	push	r12
14095d47e: 56                          	push	rsi
14095d47f: 57                          	push	rdi
14095d480: 53                          	push	rbx
14095d481: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095d488: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095d48f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095d495: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095d49b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095d4a1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095d4a7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095d4ad: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095d4b6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095d4bf: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095d4c8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095d4d0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095d4d8: 48 83 bd 60 03 00 00 00     	cmp	qword ptr [rbp + 0x360], 0x0
14095d4e0: 74 19                       	je	0x14095d4fb <.text+0x95c4fb>
14095d4e2: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095d4e8: 48 8b 8d 78 03 00 00        	mov	rcx, qword ptr [rbp + 0x378]
14095d4ef: 48 8b 95 60 03 00 00        	mov	rdx, qword ptr [rbp + 0x360]
14095d4f6: e8 25 67 cb ff              	call	0x140613c20 <.text+0x612c20>
14095d4fb: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095d503: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095d50b: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095d514: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095d51d: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095d526: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095d52c: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095d532: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095d538: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095d53e: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095d544: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095d54b: 5b                          	pop	rbx
14095d54c: 5f                          	pop	rdi
14095d54d: 5e                          	pop	rsi
14095d54e: 41 5c                       	pop	r12
14095d550: 41 5d                       	pop	r13
14095d552: 41 5e                       	pop	r14
14095d554: 41 5f                       	pop	r15
14095d556: 5d                          	pop	rbp
14095d557: c3                          	ret
14095d558: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
14095d560: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095d565: 55                          	push	rbp
14095d566: 41 57                       	push	r15
14095d568: 41 56                       	push	r14
14095d56a: 41 55                       	push	r13
14095d56c: 41 54                       	push	r12
14095d56e: 56                          	push	rsi
14095d56f: 57                          	push	rdi
14095d570: 53                          	push	rbx
14095d571: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095d578: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095d57f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095d585: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095d58b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095d591: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095d597: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095d59d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095d5a6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095d5af: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095d5b8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095d5c0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095d5c8: 48 83 bd 68 03 00 00 00     	cmp	qword ptr [rbp + 0x368], 0x0
14095d5d0: 74 19                       	je	0x14095d5eb <.text+0x95c5eb>
14095d5d2: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095d5d8: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
14095d5df: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095d5e6: e8 35 66 cb ff              	call	0x140613c20 <.text+0x612c20>
14095d5eb: c6 85 77 03 00 00 00        	mov	byte ptr [rbp + 0x377], 0x0
14095d5f2: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095d5fa: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095d602: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095d60b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095d614: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095d61d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095d623: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095d629: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095d62f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095d635: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095d63b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095d642: 5b                          	pop	rbx
14095d643: 5f                          	pop	rdi
14095d644: 5e                          	pop	rsi
14095d645: 41 5c                       	pop	r12
14095d647: 41 5d                       	pop	r13
14095d649: 41 5e                       	pop	r14
14095d64b: 41 5f                       	pop	r15
14095d64d: 5d                          	pop	rbp
14095d64e: c3                          	ret
14095d64f: 90                          	nop
14095d650: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095d655: 55                          	push	rbp
14095d656: 41 57                       	push	r15
14095d658: 41 56                       	push	r14
14095d65a: 41 55                       	push	r13
14095d65c: 41 54                       	push	r12
14095d65e: 56                          	push	rsi
14095d65f: 57                          	push	rdi
14095d660: 53                          	push	rbx
14095d661: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095d668: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095d66f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095d675: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095d67b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095d681: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095d687: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095d68d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095d696: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095d69f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095d6a8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095d6b0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095d6b8: 80 bd 77 03 00 00 00        	cmp	byte ptr [rbp + 0x377], 0x0
14095d6bf: 0f 84 83 00 00 00           	je	0x14095d748 <.text+0x95c748>
14095d6c5: 48 8b 85 20 02 00 00        	mov	rax, qword ptr [rbp + 0x220]
14095d6cc: 48 85 c0                    	test	rax, rax
14095d6cf: 74 1a                       	je	0x14095d6eb <.text+0x95c6eb>
14095d6d1: 48 8b 8d 28 02 00 00        	mov	rcx, qword ptr [rbp + 0x228]
14095d6d8: 48 c1 e0 02                 	shl	rax, 0x2
14095d6dc: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
14095d6e0: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095d6e6: e8 35 65 cb ff              	call	0x140613c20 <.text+0x612c20>
14095d6eb: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095d6f3: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095d6fb: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095d704: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095d70d: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095d716: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095d71c: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095d722: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095d728: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095d72e: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095d734: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095d73b: 5b                          	pop	rbx
14095d73c: 5f                          	pop	rdi
14095d73d: 5e                          	pop	rsi
14095d73e: 41 5c                       	pop	r12
14095d740: 41 5d                       	pop	r13
14095d742: 41 5e                       	pop	r14
14095d744: 41 5f                       	pop	r15
14095d746: 5d                          	pop	rbp
14095d747: c3                          	ret
14095d748: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095d750: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095d758: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095d761: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095d76a: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095d773: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095d779: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095d77f: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095d785: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095d78b: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095d791: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095d798: 5b                          	pop	rbx
14095d799: 5f                          	pop	rdi
14095d79a: 5e                          	pop	rsi
14095d79b: 41 5c                       	pop	r12
14095d79d: 41 5d                       	pop	r13
14095d79f: 41 5e                       	pop	r14
14095d7a1: 41 5f                       	pop	r15
14095d7a3: 5d                          	pop	rbp
14095d7a4: c3                          	ret
14095d7a5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
14095d7b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095d7b5: 55                          	push	rbp
14095d7b6: 41 57                       	push	r15
14095d7b8: 41 56                       	push	r14
14095d7ba: 41 55                       	push	r13
14095d7bc: 41 54                       	push	r12
14095d7be: 56                          	push	rsi
14095d7bf: 57                          	push	rdi
14095d7c0: 53                          	push	rbx
14095d7c1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095d7c8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095d7cf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095d7d5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095d7db: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095d7e1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095d7e7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095d7ed: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095d7f6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095d7ff: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095d808: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095d810: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095d818: 48 83 bd 78 03 00 00 00     	cmp	qword ptr [rbp + 0x378], 0x0
14095d820: 74 21                       	je	0x14095d843 <.text+0x95c843>
14095d822: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095d829: 48 c1 e0 02                 	shl	rax, 0x2
14095d82d: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
14095d831: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095d837: 48 8b 8d 58 03 00 00        	mov	rcx, qword ptr [rbp + 0x358]
14095d83e: e8 dd 63 cb ff              	call	0x140613c20 <.text+0x612c20>
14095d843: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095d84b: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095d853: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095d85c: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095d865: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095d86e: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095d874: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095d87a: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095d880: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095d886: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095d88c: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095d893: 5b                          	pop	rbx
14095d894: 5f                          	pop	rdi
14095d895: 5e                          	pop	rsi
14095d896: 41 5c                       	pop	r12
14095d898: 41 5d                       	pop	r13
14095d89a: 41 5e                       	pop	r14
14095d89c: 41 5f                       	pop	r15
14095d89e: 5d                          	pop	rbp
14095d89f: c3                          	ret
14095d8a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095d8a5: 55                          	push	rbp
14095d8a6: 41 57                       	push	r15
14095d8a8: 41 56                       	push	r14
14095d8aa: 41 55                       	push	r13
14095d8ac: 41 54                       	push	r12
14095d8ae: 56                          	push	rsi
14095d8af: 57                          	push	rdi
14095d8b0: 53                          	push	rbx
14095d8b1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095d8b8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095d8bf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095d8c5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095d8cb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095d8d1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095d8d7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095d8dd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095d8e6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095d8ef: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095d8f8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095d900: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095d908: 48 8b 95 20 02 00 00        	mov	rdx, qword ptr [rbp + 0x220]
14095d90f: 48 85 d2                    	test	rdx, rdx
14095d912: 74 12                       	je	0x14095d926 <.text+0x95c926>
14095d914: 48 8b 8d 28 02 00 00        	mov	rcx, qword ptr [rbp + 0x228]
14095d91b: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095d921: e8 fa 62 cb ff              	call	0x140613c20 <.text+0x612c20>
14095d926: c6 85 56 03 00 00 01        	mov	byte ptr [rbp + 0x356], 0x1
14095d92d: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095d935: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095d93d: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095d946: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095d94f: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095d958: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095d95e: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095d964: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095d96a: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095d970: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095d976: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095d97d: 5b                          	pop	rbx
14095d97e: 5f                          	pop	rdi
14095d97f: 5e                          	pop	rsi
14095d980: 41 5c                       	pop	r12
14095d982: 41 5d                       	pop	r13
14095d984: 41 5e                       	pop	r14
14095d986: 41 5f                       	pop	r15
14095d988: 5d                          	pop	rbp
14095d989: c3                          	ret
14095d98a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14095d990: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095d995: 55                          	push	rbp
14095d996: 41 57                       	push	r15
14095d998: 41 56                       	push	r14
14095d99a: 41 55                       	push	r13
14095d99c: 41 54                       	push	r12
14095d99e: 56                          	push	rsi
14095d99f: 57                          	push	rdi
14095d9a0: 53                          	push	rbx
14095d9a1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095d9a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095d9af: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095d9b5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095d9bb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095d9c1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095d9c7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095d9cd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095d9d6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095d9df: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095d9e8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095d9f0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095d9f8: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095d9ff: e8 ec 4a c7 ff              	call	0x1405d24f0 <.text+0x5d14f0>
14095da04: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095da0c: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095da14: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095da1d: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095da26: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095da2f: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095da35: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095da3b: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095da41: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095da47: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095da4d: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095da54: 5b                          	pop	rbx
14095da55: 5f                          	pop	rdi
14095da56: 5e                          	pop	rsi
14095da57: 41 5c                       	pop	r12
14095da59: 41 5d                       	pop	r13
14095da5b: 41 5e                       	pop	r14
14095da5d: 41 5f                       	pop	r15
14095da5f: 5d                          	pop	rbp
14095da60: c3                          	ret
14095da61: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
14095da70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095da75: 55                          	push	rbp
14095da76: 41 57                       	push	r15
14095da78: 41 56                       	push	r14
14095da7a: 41 55                       	push	r13
14095da7c: 41 54                       	push	r12
14095da7e: 56                          	push	rsi
14095da7f: 57                          	push	rdi
14095da80: 53                          	push	rbx
14095da81: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095da88: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095da8f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095da95: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095da9b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095daa1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095daa7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095daad: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095dab6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095dabf: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095dac8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095dad0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095dad8: 48 83 bd 78 03 00 00 00     	cmp	qword ptr [rbp + 0x378], 0x0
14095dae0: 74 21                       	je	0x14095db03 <.text+0x95cb03>
14095dae2: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095dae9: 48 c1 e0 02                 	shl	rax, 0x2
14095daed: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
14095daf1: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095daf7: 48 8b 8d 58 03 00 00        	mov	rcx, qword ptr [rbp + 0x358]
14095dafe: e8 1d 61 cb ff              	call	0x140613c20 <.text+0x612c20>
14095db03: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095db0b: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095db13: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095db1c: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095db25: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095db2e: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095db34: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095db3a: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095db40: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095db46: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095db4c: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095db53: 5b                          	pop	rbx
14095db54: 5f                          	pop	rdi
14095db55: 5e                          	pop	rsi
14095db56: 41 5c                       	pop	r12
14095db58: 41 5d                       	pop	r13
14095db5a: 41 5e                       	pop	r14
14095db5c: 41 5f                       	pop	r15
14095db5e: 5d                          	pop	rbp
14095db5f: c3                          	ret
14095db60: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095db65: 55                          	push	rbp
14095db66: 41 57                       	push	r15
14095db68: 41 56                       	push	r14
14095db6a: 41 55                       	push	r13
14095db6c: 41 54                       	push	r12
14095db6e: 56                          	push	rsi
14095db6f: 57                          	push	rdi
14095db70: 53                          	push	rbx
14095db71: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095db78: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095db7f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095db85: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095db8b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095db91: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095db97: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095db9d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095dba6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095dbaf: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095dbb8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095dbc0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095dbc8: 48 8b 95 80 02 00 00        	mov	rdx, qword ptr [rbp + 0x280]
14095dbcf: 48 85 d2                    	test	rdx, rdx
14095dbd2: 74 12                       	je	0x14095dbe6 <.text+0x95cbe6>
14095dbd4: 48 8b 8d 88 02 00 00        	mov	rcx, qword ptr [rbp + 0x288]
14095dbdb: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095dbe1: e8 3a 60 cb ff              	call	0x140613c20 <.text+0x612c20>
14095dbe6: c6 85 57 03 00 00 01        	mov	byte ptr [rbp + 0x357], 0x1
14095dbed: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095dbf5: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095dbfd: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095dc06: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095dc0f: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095dc18: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095dc1e: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095dc24: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095dc2a: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095dc30: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095dc36: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095dc3d: 5b                          	pop	rbx
14095dc3e: 5f                          	pop	rdi
14095dc3f: 5e                          	pop	rsi
14095dc40: 41 5c                       	pop	r12
14095dc42: 41 5d                       	pop	r13
14095dc44: 41 5e                       	pop	r14
14095dc46: 41 5f                       	pop	r15
14095dc48: 5d                          	pop	rbp
14095dc49: c3                          	ret
14095dc4a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14095dc50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095dc55: 55                          	push	rbp
14095dc56: 41 57                       	push	r15
14095dc58: 41 56                       	push	r14
14095dc5a: 41 55                       	push	r13
14095dc5c: 41 54                       	push	r12
14095dc5e: 56                          	push	rsi
14095dc5f: 57                          	push	rdi
14095dc60: 53                          	push	rbx
14095dc61: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095dc68: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095dc6f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095dc75: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095dc7b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095dc81: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095dc87: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095dc8d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095dc96: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095dc9f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095dca8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095dcb0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095dcb8: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095dcbf: e8 2c 48 c7 ff              	call	0x1405d24f0 <.text+0x5d14f0>
14095dcc4: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095dccc: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095dcd4: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095dcdd: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095dce6: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095dcef: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095dcf5: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095dcfb: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095dd01: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095dd07: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095dd0d: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095dd14: 5b                          	pop	rbx
14095dd15: 5f                          	pop	rdi
14095dd16: 5e                          	pop	rsi
14095dd17: 41 5c                       	pop	r12
14095dd19: 41 5d                       	pop	r13
14095dd1b: 41 5e                       	pop	r14
14095dd1d: 41 5f                       	pop	r15
14095dd1f: 5d                          	pop	rbp
14095dd20: c3                          	ret
14095dd21: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
14095dd30: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095dd35: 55                          	push	rbp
14095dd36: 41 57                       	push	r15
14095dd38: 41 56                       	push	r14
14095dd3a: 41 55                       	push	r13
14095dd3c: 41 54                       	push	r12
14095dd3e: 56                          	push	rsi
14095dd3f: 57                          	push	rdi
14095dd40: 53                          	push	rbx
14095dd41: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095dd48: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095dd4f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095dd55: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095dd5b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095dd61: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095dd67: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095dd6d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095dd76: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095dd7f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095dd88: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095dd90: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095dd98: 48 8b 85 58 03 00 00        	mov	rax, qword ptr [rbp + 0x358]
14095dd9f: f0                          	lock
14095dda0: 48 ff 08                    	dec	qword ptr [rax]
14095dda3: 75 0c                       	jne	0x14095ddb1 <.text+0x95cdb1>
14095dda5: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095ddac: e8 1f 1c cf ff              	call	0x14064f9d0 <.text+0x64e9d0>
14095ddb1: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095ddb9: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095ddc1: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095ddca: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095ddd3: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095dddc: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095dde2: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095dde8: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095ddee: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095ddf4: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095ddfa: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095de01: 5b                          	pop	rbx
14095de02: 5f                          	pop	rdi
14095de03: 5e                          	pop	rsi
14095de04: 41 5c                       	pop	r12
14095de06: 41 5d                       	pop	r13
14095de08: 41 5e                       	pop	r14
14095de0a: 41 5f                       	pop	r15
14095de0c: 5d                          	pop	rbp
14095de0d: c3                          	ret
14095de0e: 66 90                       	nop
14095de10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095de15: 55                          	push	rbp
14095de16: 41 57                       	push	r15
14095de18: 41 56                       	push	r14
14095de1a: 41 55                       	push	r13
14095de1c: 41 54                       	push	r12
14095de1e: 56                          	push	rsi
14095de1f: 57                          	push	rdi
14095de20: 53                          	push	rbx
14095de21: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095de28: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095de2f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095de35: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095de3b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095de41: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095de47: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095de4d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095de56: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095de5f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095de68: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095de70: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095de78: 48 8b 85 58 03 00 00        	mov	rax, qword ptr [rbp + 0x358]
14095de7f: f0                          	lock
14095de80: 48 ff 08                    	dec	qword ptr [rax]
14095de83: 75 0c                       	jne	0x14095de91 <.text+0x95ce91>
14095de85: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095de8c: e8 3f 1b cf ff              	call	0x14064f9d0 <.text+0x64e9d0>
14095de91: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095de99: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095dea1: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095deaa: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095deb3: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095debc: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095dec2: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095dec8: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095dece: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095ded4: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095deda: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095dee1: 5b                          	pop	rbx
14095dee2: 5f                          	pop	rdi
14095dee3: 5e                          	pop	rsi
14095dee4: 41 5c                       	pop	r12
14095dee6: 41 5d                       	pop	r13
14095dee8: 41 5e                       	pop	r14
14095deea: 41 5f                       	pop	r15
14095deec: 5d                          	pop	rbp
14095deed: c3                          	ret
14095deee: 66 90                       	nop
14095def0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095def5: 55                          	push	rbp
14095def6: 41 57                       	push	r15
14095def8: 41 56                       	push	r14
14095defa: 41 55                       	push	r13
14095defc: 41 54                       	push	r12
14095defe: 56                          	push	rsi
14095deff: 57                          	push	rdi
14095df00: 53                          	push	rbx
14095df01: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095df08: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095df0f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095df15: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095df1b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095df21: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095df27: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095df2d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095df36: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095df3f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095df48: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095df50: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095df58: 48 83 bd 68 03 00 00 00     	cmp	qword ptr [rbp + 0x368], 0x0
14095df60: 74 19                       	je	0x14095df7b <.text+0x95cf7b>
14095df62: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095df68: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
14095df6f: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095df76: e8 a5 5c cb ff              	call	0x140613c20 <.text+0x612c20>
14095df7b: c6 85 56 03 00 00 00        	mov	byte ptr [rbp + 0x356], 0x0
14095df82: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095df8a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095df92: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095df9b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095dfa4: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095dfad: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095dfb3: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095dfb9: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095dfbf: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095dfc5: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095dfcb: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095dfd2: 5b                          	pop	rbx
14095dfd3: 5f                          	pop	rdi
14095dfd4: 5e                          	pop	rsi
14095dfd5: 41 5c                       	pop	r12
14095dfd7: 41 5d                       	pop	r13
14095dfd9: 41 5e                       	pop	r14
14095dfdb: 41 5f                       	pop	r15
14095dfdd: 5d                          	pop	rbp
14095dfde: c3                          	ret
14095dfdf: 90                          	nop
14095dfe0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095dfe5: 55                          	push	rbp
14095dfe6: 41 57                       	push	r15
14095dfe8: 41 56                       	push	r14
14095dfea: 41 55                       	push	r13
14095dfec: 41 54                       	push	r12
14095dfee: 56                          	push	rsi
14095dfef: 57                          	push	rdi
14095dff0: 53                          	push	rbx
14095dff1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095dff8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095dfff: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095e005: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095e00b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095e011: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095e017: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095e01d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095e026: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095e02f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095e038: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095e040: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095e048: 48 83 bd 68 03 00 00 00     	cmp	qword ptr [rbp + 0x368], 0x0
14095e050: 74 19                       	je	0x14095e06b <.text+0x95d06b>
14095e052: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095e058: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
14095e05f: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095e066: e8 b5 5b cb ff              	call	0x140613c20 <.text+0x612c20>
14095e06b: c6 85 57 03 00 00 00        	mov	byte ptr [rbp + 0x357], 0x0
14095e072: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095e07a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095e082: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095e08b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095e094: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095e09d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095e0a3: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095e0a9: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095e0af: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095e0b5: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095e0bb: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095e0c2: 5b                          	pop	rbx
14095e0c3: 5f                          	pop	rdi
14095e0c4: 5e                          	pop	rsi
14095e0c5: 41 5c                       	pop	r12
14095e0c7: 41 5d                       	pop	r13
14095e0c9: 41 5e                       	pop	r14
14095e0cb: 41 5f                       	pop	r15
14095e0cd: 5d                          	pop	rbp
14095e0ce: c3                          	ret
14095e0cf: 90                          	nop
14095e0d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095e0d5: 55                          	push	rbp
14095e0d6: 41 57                       	push	r15
14095e0d8: 41 56                       	push	r14
14095e0da: 41 55                       	push	r13
14095e0dc: 41 54                       	push	r12
14095e0de: 56                          	push	rsi
14095e0df: 57                          	push	rdi
14095e0e0: 53                          	push	rbx
14095e0e1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095e0e8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095e0ef: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095e0f5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095e0fb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095e101: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095e107: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095e10d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095e116: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095e11f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095e128: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095e130: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095e138: 80 bd 56 03 00 00 00        	cmp	byte ptr [rbp + 0x356], 0x0
14095e13f: 0f 84 83 00 00 00           	je	0x14095e1c8 <.text+0x95d1c8>
14095e145: 48 8b 85 00 02 00 00        	mov	rax, qword ptr [rbp + 0x200]
14095e14c: 48 85 c0                    	test	rax, rax
14095e14f: 74 1a                       	je	0x14095e16b <.text+0x95d16b>
14095e151: 48 8b 8d 08 02 00 00        	mov	rcx, qword ptr [rbp + 0x208]
14095e158: 48 c1 e0 02                 	shl	rax, 0x2
14095e15c: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
14095e160: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095e166: e8 b5 5a cb ff              	call	0x140613c20 <.text+0x612c20>
14095e16b: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095e173: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095e17b: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095e184: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095e18d: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095e196: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095e19c: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095e1a2: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095e1a8: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095e1ae: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095e1b4: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095e1bb: 5b                          	pop	rbx
14095e1bc: 5f                          	pop	rdi
14095e1bd: 5e                          	pop	rsi
14095e1be: 41 5c                       	pop	r12
14095e1c0: 41 5d                       	pop	r13
14095e1c2: 41 5e                       	pop	r14
14095e1c4: 41 5f                       	pop	r15
14095e1c6: 5d                          	pop	rbp
14095e1c7: c3                          	ret
14095e1c8: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095e1d0: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095e1d8: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095e1e1: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095e1ea: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095e1f3: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095e1f9: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095e1ff: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095e205: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095e20b: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095e211: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095e218: 5b                          	pop	rbx
14095e219: 5f                          	pop	rdi
14095e21a: 5e                          	pop	rsi
14095e21b: 41 5c                       	pop	r12
14095e21d: 41 5d                       	pop	r13
14095e21f: 41 5e                       	pop	r14
14095e221: 41 5f                       	pop	r15
14095e223: 5d                          	pop	rbp
14095e224: c3                          	ret
14095e225: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
14095e230: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095e235: 55                          	push	rbp
14095e236: 41 57                       	push	r15
14095e238: 41 56                       	push	r14
14095e23a: 41 55                       	push	r13
14095e23c: 41 54                       	push	r12
14095e23e: 56                          	push	rsi
14095e23f: 57                          	push	rdi
14095e240: 53                          	push	rbx
14095e241: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095e248: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095e24f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095e255: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095e25b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095e261: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095e267: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095e26d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095e276: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095e27f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095e288: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095e290: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095e298: 80 bd 57 03 00 00 00        	cmp	byte ptr [rbp + 0x357], 0x0
14095e29f: 0f 84 83 00 00 00           	je	0x14095e328 <.text+0x95d328>
14095e2a5: 48 8b 85 e0 01 00 00        	mov	rax, qword ptr [rbp + 0x1e0]
14095e2ac: 48 85 c0                    	test	rax, rax
14095e2af: 74 1a                       	je	0x14095e2cb <.text+0x95d2cb>
14095e2b1: 48 8b 8d e8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1e8]
14095e2b8: 48 c1 e0 02                 	shl	rax, 0x2
14095e2bc: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
14095e2c0: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095e2c6: e8 55 59 cb ff              	call	0x140613c20 <.text+0x612c20>
14095e2cb: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095e2d3: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095e2db: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095e2e4: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095e2ed: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095e2f6: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095e2fc: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095e302: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095e308: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095e30e: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095e314: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095e31b: 5b                          	pop	rbx
14095e31c: 5f                          	pop	rdi
14095e31d: 5e                          	pop	rsi
14095e31e: 41 5c                       	pop	r12
14095e320: 41 5d                       	pop	r13
14095e322: 41 5e                       	pop	r14
14095e324: 41 5f                       	pop	r15
14095e326: 5d                          	pop	rbp
14095e327: c3                          	ret
14095e328: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095e330: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095e338: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095e341: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095e34a: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095e353: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095e359: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095e35f: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095e365: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095e36b: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095e371: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095e378: 5b                          	pop	rbx
14095e379: 5f                          	pop	rdi
14095e37a: 5e                          	pop	rsi
14095e37b: 41 5c                       	pop	r12
14095e37d: 41 5d                       	pop	r13
14095e37f: 41 5e                       	pop	r14
14095e381: 41 5f                       	pop	r15
14095e383: 5d                          	pop	rbp
14095e384: c3                          	ret
14095e385: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
14095e390: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095e395: 55                          	push	rbp
14095e396: 41 57                       	push	r15
14095e398: 41 56                       	push	r14
14095e39a: 41 55                       	push	r13
14095e39c: 41 54                       	push	r12
14095e39e: 56                          	push	rsi
14095e39f: 57                          	push	rdi
14095e3a0: 53                          	push	rbx
14095e3a1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095e3a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095e3af: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095e3b5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095e3bb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095e3c1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095e3c7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095e3cd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095e3d6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095e3df: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095e3e8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095e3f0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095e3f8: 48 83 bd 78 03 00 00 00     	cmp	qword ptr [rbp + 0x378], 0x0
14095e400: 74 21                       	je	0x14095e423 <.text+0x95d423>
14095e402: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095e409: 48 c1 e0 02                 	shl	rax, 0x2
14095e40d: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
14095e411: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095e417: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
14095e41e: e8 fd 57 cb ff              	call	0x140613c20 <.text+0x612c20>
14095e423: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095e42b: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095e433: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095e43c: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095e445: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095e44e: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095e454: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095e45a: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095e460: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095e466: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095e46c: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095e473: 5b                          	pop	rbx
14095e474: 5f                          	pop	rdi
14095e475: 5e                          	pop	rsi
14095e476: 41 5c                       	pop	r12
14095e478: 41 5d                       	pop	r13
14095e47a: 41 5e                       	pop	r14
14095e47c: 41 5f                       	pop	r15
14095e47e: 5d                          	pop	rbp
14095e47f: c3                          	ret
14095e480: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095e485: 55                          	push	rbp
14095e486: 41 57                       	push	r15
14095e488: 41 56                       	push	r14
14095e48a: 41 55                       	push	r13
14095e48c: 41 54                       	push	r12
14095e48e: 56                          	push	rsi
14095e48f: 57                          	push	rdi
14095e490: 53                          	push	rbx
14095e491: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095e498: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095e49f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095e4a5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095e4ab: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095e4b1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095e4b7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095e4bd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095e4c6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095e4cf: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095e4d8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095e4e0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095e4e8: 48 83 bd 78 03 00 00 00     	cmp	qword ptr [rbp + 0x378], 0x0
14095e4f0: 74 21                       	je	0x14095e513 <.text+0x95d513>
14095e4f2: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095e4f9: 48 c1 e0 02                 	shl	rax, 0x2
14095e4fd: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
14095e501: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095e507: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
14095e50e: e8 0d 57 cb ff              	call	0x140613c20 <.text+0x612c20>
14095e513: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095e51b: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095e523: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095e52c: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095e535: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095e53e: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095e544: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095e54a: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095e550: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095e556: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095e55c: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095e563: 5b                          	pop	rbx
14095e564: 5f                          	pop	rdi
14095e565: 5e                          	pop	rsi
14095e566: 41 5c                       	pop	r12
14095e568: 41 5d                       	pop	r13
14095e56a: 41 5e                       	pop	r14
14095e56c: 41 5f                       	pop	r15
14095e56e: 5d                          	pop	rbp
14095e56f: c3                          	ret
14095e570: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095e575: 55                          	push	rbp
14095e576: 41 57                       	push	r15
14095e578: 41 56                       	push	r14
14095e57a: 41 55                       	push	r13
14095e57c: 41 54                       	push	r12
14095e57e: 56                          	push	rsi
14095e57f: 57                          	push	rdi
14095e580: 53                          	push	rbx
14095e581: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095e588: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095e58f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095e595: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095e59b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095e5a1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095e5a7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095e5ad: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095e5b6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095e5bf: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095e5c8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095e5d0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095e5d8: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095e5df: e8 0c 3f c7 ff              	call	0x1405d24f0 <.text+0x5d14f0>
14095e5e4: c6 85 86 03 00 00 00        	mov	byte ptr [rbp + 0x386], 0x0
14095e5eb: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095e5f3: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095e5fb: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095e604: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095e60d: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095e616: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095e61c: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095e622: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095e628: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095e62e: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095e634: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095e63b: 5b                          	pop	rbx
14095e63c: 5f                          	pop	rdi
14095e63d: 5e                          	pop	rsi
14095e63e: 41 5c                       	pop	r12
14095e640: 41 5d                       	pop	r13
14095e642: 41 5e                       	pop	r14
14095e644: 41 5f                       	pop	r15
14095e646: 5d                          	pop	rbp
14095e647: c3                          	ret
14095e648: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
14095e650: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095e655: 55                          	push	rbp
14095e656: 41 57                       	push	r15
14095e658: 41 56                       	push	r14
14095e65a: 41 55                       	push	r13
14095e65c: 41 54                       	push	r12
14095e65e: 56                          	push	rsi
14095e65f: 57                          	push	rdi
14095e660: 53                          	push	rbx
14095e661: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095e668: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095e66f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095e675: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095e67b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095e681: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095e687: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095e68d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095e696: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095e69f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095e6a8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095e6b0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095e6b8: 48 8b 85 58 03 00 00        	mov	rax, qword ptr [rbp + 0x358]
14095e6bf: f0                          	lock
14095e6c0: 48 ff 08                    	dec	qword ptr [rax]
14095e6c3: 75 0c                       	jne	0x14095e6d1 <.text+0x95d6d1>
14095e6c5: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095e6cc: e8 ff 12 cf ff              	call	0x14064f9d0 <.text+0x64e9d0>
14095e6d1: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095e6d9: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095e6e1: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095e6ea: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095e6f3: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095e6fc: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095e702: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095e708: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095e70e: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095e714: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095e71a: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095e721: 5b                          	pop	rbx
14095e722: 5f                          	pop	rdi
14095e723: 5e                          	pop	rsi
14095e724: 41 5c                       	pop	r12
14095e726: 41 5d                       	pop	r13
14095e728: 41 5e                       	pop	r14
14095e72a: 41 5f                       	pop	r15
14095e72c: 5d                          	pop	rbp
14095e72d: c3                          	ret
14095e72e: 66 90                       	nop
14095e730: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095e735: 55                          	push	rbp
14095e736: 41 57                       	push	r15
14095e738: 41 56                       	push	r14
14095e73a: 41 55                       	push	r13
14095e73c: 41 54                       	push	r12
14095e73e: 56                          	push	rsi
14095e73f: 57                          	push	rdi
14095e740: 53                          	push	rbx
14095e741: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095e748: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095e74f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095e755: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095e75b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095e761: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095e767: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095e76d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095e776: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095e77f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095e788: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095e790: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095e798: 48 8b 85 b0 02 00 00        	mov	rax, qword ptr [rbp + 0x2b0]
14095e79f: f0                          	lock
14095e7a0: 48 ff 08                    	dec	qword ptr [rax]
14095e7a3: 75 0c                       	jne	0x14095e7b1 <.text+0x95d7b1>
14095e7a5: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095e7ac: e8 1f 12 cf ff              	call	0x14064f9d0 <.text+0x64e9d0>
14095e7b1: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095e7b9: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095e7c1: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095e7ca: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095e7d3: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095e7dc: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095e7e2: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095e7e8: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095e7ee: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095e7f4: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095e7fa: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095e801: 5b                          	pop	rbx
14095e802: 5f                          	pop	rdi
14095e803: 5e                          	pop	rsi
14095e804: 41 5c                       	pop	r12
14095e806: 41 5d                       	pop	r13
14095e808: 41 5e                       	pop	r14
14095e80a: 41 5f                       	pop	r15
14095e80c: 5d                          	pop	rbp
14095e80d: c3                          	ret
14095e80e: 66 90                       	nop
14095e810: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095e815: 55                          	push	rbp
14095e816: 41 57                       	push	r15
14095e818: 41 56                       	push	r14
14095e81a: 41 55                       	push	r13
14095e81c: 41 54                       	push	r12
14095e81e: 56                          	push	rsi
14095e81f: 57                          	push	rdi
14095e820: 53                          	push	rbx
14095e821: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095e828: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095e82f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095e835: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095e83b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095e841: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095e847: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095e84d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095e856: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095e85f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095e868: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095e870: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095e878: 48 83 bd 78 03 00 00 00     	cmp	qword ptr [rbp + 0x378], 0x0
14095e880: 74 21                       	je	0x14095e8a3 <.text+0x95d8a3>
14095e882: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095e889: 48 c1 e0 02                 	shl	rax, 0x2
14095e88d: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
14095e891: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095e897: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
14095e89e: e8 7d 53 cb ff              	call	0x140613c20 <.text+0x612c20>
14095e8a3: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095e8ab: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095e8b3: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095e8bc: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095e8c5: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095e8ce: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095e8d4: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095e8da: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095e8e0: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095e8e6: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095e8ec: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095e8f3: 5b                          	pop	rbx
14095e8f4: 5f                          	pop	rdi
14095e8f5: 5e                          	pop	rsi
14095e8f6: 41 5c                       	pop	r12
14095e8f8: 41 5d                       	pop	r13
14095e8fa: 41 5e                       	pop	r14
14095e8fc: 41 5f                       	pop	r15
14095e8fe: 5d                          	pop	rbp
14095e8ff: c3                          	ret
14095e900: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095e905: 55                          	push	rbp
14095e906: 41 57                       	push	r15
14095e908: 41 56                       	push	r14
14095e90a: 41 55                       	push	r13
14095e90c: 41 54                       	push	r12
14095e90e: 56                          	push	rsi
14095e90f: 57                          	push	rdi
14095e910: 53                          	push	rbx
14095e911: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095e918: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095e91f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095e925: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095e92b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095e931: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095e937: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095e93d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095e946: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095e94f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095e958: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095e960: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095e968: 48 8b 85 78 02 00 00        	mov	rax, qword ptr [rbp + 0x278]
14095e96f: f0                          	lock
14095e970: 48 ff 08                    	dec	qword ptr [rax]
14095e973: 75 0c                       	jne	0x14095e981 <.text+0x95d981>
14095e975: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095e97c: e8 4f 10 cf ff              	call	0x14064f9d0 <.text+0x64e9d0>
14095e981: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095e989: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095e991: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095e99a: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095e9a3: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095e9ac: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095e9b2: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095e9b8: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095e9be: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095e9c4: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095e9ca: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095e9d1: 5b                          	pop	rbx
14095e9d2: 5f                          	pop	rdi
14095e9d3: 5e                          	pop	rsi
14095e9d4: 41 5c                       	pop	r12
14095e9d6: 41 5d                       	pop	r13
14095e9d8: 41 5e                       	pop	r14
14095e9da: 41 5f                       	pop	r15
14095e9dc: 5d                          	pop	rbp
14095e9dd: c3                          	ret
14095e9de: 66 90                       	nop
14095e9e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095e9e5: 55                          	push	rbp
14095e9e6: 41 57                       	push	r15
14095e9e8: 41 56                       	push	r14
14095e9ea: 41 55                       	push	r13
14095e9ec: 41 54                       	push	r12
14095e9ee: 56                          	push	rsi
14095e9ef: 57                          	push	rdi
14095e9f0: 53                          	push	rbx
14095e9f1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095e9f8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095e9ff: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095ea05: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095ea0b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095ea11: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095ea17: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095ea1d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095ea26: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095ea2f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095ea38: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095ea40: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095ea48: 48 83 bd 68 03 00 00 00     	cmp	qword ptr [rbp + 0x368], 0x0
14095ea50: 74 19                       	je	0x14095ea6b <.text+0x95da6b>
14095ea52: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095ea58: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
14095ea5f: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095ea66: e8 b5 51 cb ff              	call	0x140613c20 <.text+0x612c20>
14095ea6b: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095ea72: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095ea7a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095ea82: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095ea8b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095ea94: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095ea9d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095eaa3: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095eaa9: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095eaaf: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095eab5: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095eabb: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095eac2: 5b                          	pop	rbx
14095eac3: 5f                          	pop	rdi
14095eac4: 5e                          	pop	rsi
14095eac5: 41 5c                       	pop	r12
14095eac7: 41 5d                       	pop	r13
14095eac9: 41 5e                       	pop	r14
14095eacb: 41 5f                       	pop	r15
14095eacd: 5d                          	pop	rbp
14095eace: c3                          	ret
14095eacf: 90                          	nop
14095ead0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095ead5: 55                          	push	rbp
14095ead6: 41 57                       	push	r15
14095ead8: 41 56                       	push	r14
14095eada: 41 55                       	push	r13
14095eadc: 41 54                       	push	r12
14095eade: 56                          	push	rsi
14095eadf: 57                          	push	rdi
14095eae0: 53                          	push	rbx
14095eae1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095eae8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095eaef: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095eaf5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095eafb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095eb01: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095eb07: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095eb0d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095eb16: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095eb1f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095eb28: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095eb30: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095eb38: 48 83 bd 68 03 00 00 00     	cmp	qword ptr [rbp + 0x368], 0x0
14095eb40: 74 19                       	je	0x14095eb5b <.text+0x95db5b>
14095eb42: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095eb48: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
14095eb4f: 48 8b 95 68 03 00 00        	mov	rdx, qword ptr [rbp + 0x368]
14095eb56: e8 c5 50 cb ff              	call	0x140613c20 <.text+0x612c20>
14095eb5b: c6 85 85 03 00 00 00        	mov	byte ptr [rbp + 0x385], 0x0
14095eb62: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095eb6a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095eb72: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095eb7b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095eb84: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095eb8d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095eb93: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095eb99: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095eb9f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095eba5: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095ebab: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095ebb2: 5b                          	pop	rbx
14095ebb3: 5f                          	pop	rdi
14095ebb4: 5e                          	pop	rsi
14095ebb5: 41 5c                       	pop	r12
14095ebb7: 41 5d                       	pop	r13
14095ebb9: 41 5e                       	pop	r14
14095ebbb: 41 5f                       	pop	r15
14095ebbd: 5d                          	pop	rbp
14095ebbe: c3                          	ret
14095ebbf: 90                          	nop
14095ebc0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095ebc5: 55                          	push	rbp
14095ebc6: 41 57                       	push	r15
14095ebc8: 41 56                       	push	r14
14095ebca: 41 55                       	push	r13
14095ebcc: 41 54                       	push	r12
14095ebce: 56                          	push	rsi
14095ebcf: 57                          	push	rdi
14095ebd0: 53                          	push	rbx
14095ebd1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095ebd8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095ebdf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095ebe5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095ebeb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095ebf1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095ebf7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095ebfd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095ec06: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095ec0f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095ec18: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095ec20: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095ec28: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095ec2f: 48 ff 00                    	inc	qword ptr [rax]
14095ec32: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095ec3a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095ec42: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095ec4b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095ec54: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095ec5d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095ec63: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095ec69: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095ec6f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095ec75: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095ec7b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095ec82: 5b                          	pop	rbx
14095ec83: 5f                          	pop	rdi
14095ec84: 5e                          	pop	rsi
14095ec85: 41 5c                       	pop	r12
14095ec87: 41 5d                       	pop	r13
14095ec89: 41 5e                       	pop	r14
14095ec8b: 41 5f                       	pop	r15
14095ec8d: 5d                          	pop	rbp
14095ec8e: c3                          	ret
14095ec8f: 90                          	nop
14095ec90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095ec95: 55                          	push	rbp
14095ec96: 41 57                       	push	r15
14095ec98: 41 56                       	push	r14
14095ec9a: 41 55                       	push	r13
14095ec9c: 41 54                       	push	r12
14095ec9e: 56                          	push	rsi
14095ec9f: 57                          	push	rdi
14095eca0: 53                          	push	rbx
14095eca1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095eca8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095ecaf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095ecb5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095ecbb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095ecc1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095ecc7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095eccd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095ecd6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095ecdf: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095ece8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095ecf0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095ecf8: 48 83 bd 58 03 00 00 00     	cmp	qword ptr [rbp + 0x358], 0x0
14095ed00: 74 19                       	je	0x14095ed1b <.text+0x95dd1b>
14095ed02: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095ed08: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
14095ed0f: 48 8b 95 58 03 00 00        	mov	rdx, qword ptr [rbp + 0x358]
14095ed16: e8 05 4f cb ff              	call	0x140613c20 <.text+0x612c20>
14095ed1b: c6 85 86 03 00 00 00        	mov	byte ptr [rbp + 0x386], 0x0
14095ed22: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095ed2a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095ed32: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095ed3b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095ed44: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095ed4d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095ed53: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095ed59: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095ed5f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095ed65: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095ed6b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095ed72: 5b                          	pop	rbx
14095ed73: 5f                          	pop	rdi
14095ed74: 5e                          	pop	rsi
14095ed75: 41 5c                       	pop	r12
14095ed77: 41 5d                       	pop	r13
14095ed79: 41 5e                       	pop	r14
14095ed7b: 41 5f                       	pop	r15
14095ed7d: 5d                          	pop	rbp
14095ed7e: c3                          	ret
14095ed7f: 90                          	nop
14095ed80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095ed85: 55                          	push	rbp
14095ed86: 41 57                       	push	r15
14095ed88: 41 56                       	push	r14
14095ed8a: 41 55                       	push	r13
14095ed8c: 41 54                       	push	r12
14095ed8e: 56                          	push	rsi
14095ed8f: 57                          	push	rdi
14095ed90: 53                          	push	rbx
14095ed91: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095ed98: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095ed9f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095eda5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095edab: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095edb1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095edb7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095edbd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095edc6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095edcf: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095edd8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095ede0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095ede8: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095edef: 48 ff 00                    	inc	qword ptr [rax]
14095edf2: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095edfa: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095ee02: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095ee0b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095ee14: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095ee1d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095ee23: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095ee29: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095ee2f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095ee35: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095ee3b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095ee42: 5b                          	pop	rbx
14095ee43: 5f                          	pop	rdi
14095ee44: 5e                          	pop	rsi
14095ee45: 41 5c                       	pop	r12
14095ee47: 41 5d                       	pop	r13
14095ee49: 41 5e                       	pop	r14
14095ee4b: 41 5f                       	pop	r15
14095ee4d: 5d                          	pop	rbp
14095ee4e: c3                          	ret
14095ee4f: 90                          	nop
14095ee50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095ee55: 55                          	push	rbp
14095ee56: 41 57                       	push	r15
14095ee58: 41 56                       	push	r14
14095ee5a: 41 55                       	push	r13
14095ee5c: 41 54                       	push	r12
14095ee5e: 56                          	push	rsi
14095ee5f: 57                          	push	rdi
14095ee60: 53                          	push	rbx
14095ee61: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095ee68: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095ee6f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095ee75: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095ee7b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095ee81: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095ee87: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095ee8d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095ee96: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095ee9f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095eea8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095eeb0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095eeb8: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
14095eebf: 48 ff 00                    	inc	qword ptr [rax]
14095eec2: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095eeca: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095eed2: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095eedb: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095eee4: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095eeed: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095eef3: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095eef9: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095eeff: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095ef05: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095ef0b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095ef12: 5b                          	pop	rbx
14095ef13: 5f                          	pop	rdi
14095ef14: 5e                          	pop	rsi
14095ef15: 41 5c                       	pop	r12
14095ef17: 41 5d                       	pop	r13
14095ef19: 41 5e                       	pop	r14
14095ef1b: 41 5f                       	pop	r15
14095ef1d: 5d                          	pop	rbp
14095ef1e: c3                          	ret
14095ef1f: 90                          	nop
14095ef20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095ef25: 55                          	push	rbp
14095ef26: 41 57                       	push	r15
14095ef28: 41 56                       	push	r14
14095ef2a: 41 55                       	push	r13
14095ef2c: 41 54                       	push	r12
14095ef2e: 56                          	push	rsi
14095ef2f: 57                          	push	rdi
14095ef30: 53                          	push	rbx
14095ef31: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095ef38: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095ef3f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095ef45: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095ef4b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095ef51: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095ef57: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095ef5d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095ef66: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095ef6f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095ef78: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095ef80: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095ef88: 48 8b 95 20 02 00 00        	mov	rdx, qword ptr [rbp + 0x220]
14095ef8f: 48 85 d2                    	test	rdx, rdx
14095ef92: 74 12                       	je	0x14095efa6 <.text+0x95dfa6>
14095ef94: 48 8b 8d 28 02 00 00        	mov	rcx, qword ptr [rbp + 0x228]
14095ef9b: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095efa1: e8 7a 4c cb ff              	call	0x140613c20 <.text+0x612c20>
14095efa6: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095efad: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095efb5: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095efbd: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095efc6: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095efcf: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095efd8: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095efde: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095efe4: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095efea: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095eff0: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095eff6: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095effd: 5b                          	pop	rbx
14095effe: 5f                          	pop	rdi
14095efff: 5e                          	pop	rsi
14095f000: 41 5c                       	pop	r12
14095f002: 41 5d                       	pop	r13
14095f004: 41 5e                       	pop	r14
14095f006: 41 5f                       	pop	r15
14095f008: 5d                          	pop	rbp
14095f009: c3                          	ret
14095f00a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14095f010: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095f015: 55                          	push	rbp
14095f016: 41 57                       	push	r15
14095f018: 41 56                       	push	r14
14095f01a: 41 55                       	push	r13
14095f01c: 41 54                       	push	r12
14095f01e: 56                          	push	rsi
14095f01f: 57                          	push	rdi
14095f020: 53                          	push	rbx
14095f021: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095f028: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095f02f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095f035: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095f03b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095f041: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095f047: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095f04d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095f056: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095f05f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095f068: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095f070: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095f078: 48 8b 95 80 02 00 00        	mov	rdx, qword ptr [rbp + 0x280]
14095f07f: 48 85 d2                    	test	rdx, rdx
14095f082: 74 12                       	je	0x14095f096 <.text+0x95e096>
14095f084: 48 8b 8d 88 02 00 00        	mov	rcx, qword ptr [rbp + 0x288]
14095f08b: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095f091: e8 8a 4b cb ff              	call	0x140613c20 <.text+0x612c20>
14095f096: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095f09d: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095f0a5: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095f0ad: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095f0b6: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095f0bf: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095f0c8: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095f0ce: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095f0d4: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095f0da: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095f0e0: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095f0e6: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095f0ed: 5b                          	pop	rbx
14095f0ee: 5f                          	pop	rdi
14095f0ef: 5e                          	pop	rsi
14095f0f0: 41 5c                       	pop	r12
14095f0f2: 41 5d                       	pop	r13
14095f0f4: 41 5e                       	pop	r14
14095f0f6: 41 5f                       	pop	r15
14095f0f8: 5d                          	pop	rbp
14095f0f9: c3                          	ret
14095f0fa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14095f100: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095f105: 55                          	push	rbp
14095f106: 41 57                       	push	r15
14095f108: 41 56                       	push	r14
14095f10a: 41 55                       	push	r13
14095f10c: 41 54                       	push	r12
14095f10e: 56                          	push	rsi
14095f10f: 57                          	push	rdi
14095f110: 53                          	push	rbx
14095f111: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095f118: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095f11f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095f125: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095f12b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095f131: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095f137: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095f13d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095f146: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095f14f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095f158: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095f160: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095f168: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
14095f16f: f0                          	lock
14095f170: 48 ff 08                    	dec	qword ptr [rax]
14095f173: 75 0c                       	jne	0x14095f181 <.text+0x95e181>
14095f175: 48 8d 8d c0 02 00 00        	lea	rcx, [rbp + 0x2c0]
14095f17c: e8 4f 08 cf ff              	call	0x14064f9d0 <.text+0x64e9d0>
14095f181: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095f189: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095f191: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095f19a: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095f1a3: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095f1ac: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095f1b2: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095f1b8: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095f1be: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095f1c4: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095f1ca: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095f1d1: 5b                          	pop	rbx
14095f1d2: 5f                          	pop	rdi
14095f1d3: 5e                          	pop	rsi
14095f1d4: 41 5c                       	pop	r12
14095f1d6: 41 5d                       	pop	r13
14095f1d8: 41 5e                       	pop	r14
14095f1da: 41 5f                       	pop	r15
14095f1dc: 5d                          	pop	rbp
14095f1dd: c3                          	ret
14095f1de: 66 90                       	nop
14095f1e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095f1e5: 55                          	push	rbp
14095f1e6: 41 57                       	push	r15
14095f1e8: 41 56                       	push	r14
14095f1ea: 41 55                       	push	r13
14095f1ec: 41 54                       	push	r12
14095f1ee: 56                          	push	rsi
14095f1ef: 57                          	push	rdi
14095f1f0: 53                          	push	rbx
14095f1f1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095f1f8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095f1ff: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095f205: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095f20b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095f211: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095f217: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095f21d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095f226: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095f22f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095f238: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095f240: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095f248: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
14095f24f: f0                          	lock
14095f250: 48 ff 08                    	dec	qword ptr [rax]
14095f253: 75 0c                       	jne	0x14095f261 <.text+0x95e261>
14095f255: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095f25c: e8 6f 07 cf ff              	call	0x14064f9d0 <.text+0x64e9d0>
14095f261: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095f269: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095f271: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095f27a: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095f283: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095f28c: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095f292: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095f298: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095f29e: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095f2a4: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095f2aa: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095f2b1: 5b                          	pop	rbx
14095f2b2: 5f                          	pop	rdi
14095f2b3: 5e                          	pop	rsi
14095f2b4: 41 5c                       	pop	r12
14095f2b6: 41 5d                       	pop	r13
14095f2b8: 41 5e                       	pop	r14
14095f2ba: 41 5f                       	pop	r15
14095f2bc: 5d                          	pop	rbp
14095f2bd: c3                          	ret
14095f2be: 66 90                       	nop
14095f2c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095f2c5: 55                          	push	rbp
14095f2c6: 41 57                       	push	r15
14095f2c8: 41 56                       	push	r14
14095f2ca: 41 55                       	push	r13
14095f2cc: 41 54                       	push	r12
14095f2ce: 56                          	push	rsi
14095f2cf: 57                          	push	rdi
14095f2d0: 53                          	push	rbx
14095f2d1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095f2d8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095f2df: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095f2e5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095f2eb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095f2f1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095f2f7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095f2fd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095f306: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095f30f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095f318: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095f320: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095f328: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
14095f32f: f0                          	lock
14095f330: 48 ff 08                    	dec	qword ptr [rax]
14095f333: 75 0c                       	jne	0x14095f341 <.text+0x95e341>
14095f335: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
14095f33c: e8 8f 06 cf ff              	call	0x14064f9d0 <.text+0x64e9d0>
14095f341: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095f349: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095f351: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095f35a: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095f363: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095f36c: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095f372: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095f378: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095f37e: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095f384: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095f38a: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095f391: 5b                          	pop	rbx
14095f392: 5f                          	pop	rdi
14095f393: 5e                          	pop	rsi
14095f394: 41 5c                       	pop	r12
14095f396: 41 5d                       	pop	r13
14095f398: 41 5e                       	pop	r14
14095f39a: 41 5f                       	pop	r15
14095f39c: 5d                          	pop	rbp
14095f39d: c3                          	ret
14095f39e: 66 90                       	nop
14095f3a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095f3a5: 55                          	push	rbp
14095f3a6: 41 57                       	push	r15
14095f3a8: 41 56                       	push	r14
14095f3aa: 41 55                       	push	r13
14095f3ac: 41 54                       	push	r12
14095f3ae: 56                          	push	rsi
14095f3af: 57                          	push	rdi
14095f3b0: 53                          	push	rbx
14095f3b1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095f3b8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095f3bf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095f3c5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095f3cb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095f3d1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095f3d7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095f3dd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095f3e6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095f3ef: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095f3f8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095f400: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095f408: 48 83 bd 58 03 00 00 00     	cmp	qword ptr [rbp + 0x358], 0x0
14095f410: 74 19                       	je	0x14095f42b <.text+0x95e42b>
14095f412: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095f418: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
14095f41f: 48 8b 95 58 03 00 00        	mov	rdx, qword ptr [rbp + 0x358]
14095f426: e8 f5 47 cb ff              	call	0x140613c20 <.text+0x612c20>
14095f42b: c6 85 85 03 00 00 00        	mov	byte ptr [rbp + 0x385], 0x0
14095f432: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095f43a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095f442: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095f44b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095f454: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095f45d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095f463: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095f469: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095f46f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095f475: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095f47b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095f482: 5b                          	pop	rbx
14095f483: 5f                          	pop	rdi
14095f484: 5e                          	pop	rsi
14095f485: 41 5c                       	pop	r12
14095f487: 41 5d                       	pop	r13
14095f489: 41 5e                       	pop	r14
14095f48b: 41 5f                       	pop	r15
14095f48d: 5d                          	pop	rbp
14095f48e: c3                          	ret
14095f48f: 90                          	nop
14095f490: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095f495: 55                          	push	rbp
14095f496: 41 57                       	push	r15
14095f498: 41 56                       	push	r14
14095f49a: 41 55                       	push	r13
14095f49c: 41 54                       	push	r12
14095f49e: 56                          	push	rsi
14095f49f: 57                          	push	rdi
14095f4a0: 53                          	push	rbx
14095f4a1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095f4a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095f4af: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095f4b5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095f4bb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095f4c1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095f4c7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095f4cd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095f4d6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095f4df: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095f4e8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095f4f0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095f4f8: 48 83 bd 58 03 00 00 00     	cmp	qword ptr [rbp + 0x358], 0x0
14095f500: 74 19                       	je	0x14095f51b <.text+0x95e51b>
14095f502: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095f508: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
14095f50f: 48 8b 95 58 03 00 00        	mov	rdx, qword ptr [rbp + 0x358]
14095f516: e8 05 47 cb ff              	call	0x140613c20 <.text+0x612c20>
14095f51b: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095f522: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095f52a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095f532: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095f53b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095f544: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095f54d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095f553: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095f559: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095f55f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095f565: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095f56b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095f572: 5b                          	pop	rbx
14095f573: 5f                          	pop	rdi
14095f574: 5e                          	pop	rsi
14095f575: 41 5c                       	pop	r12
14095f577: 41 5d                       	pop	r13
14095f579: 41 5e                       	pop	r14
14095f57b: 41 5f                       	pop	r15
14095f57d: 5d                          	pop	rbp
14095f57e: c3                          	ret
14095f57f: 90                          	nop
14095f580: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095f585: 55                          	push	rbp
14095f586: 41 57                       	push	r15
14095f588: 41 56                       	push	r14
14095f58a: 41 55                       	push	r13
14095f58c: 41 54                       	push	r12
14095f58e: 56                          	push	rsi
14095f58f: 57                          	push	rdi
14095f590: 53                          	push	rbx
14095f591: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095f598: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095f59f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095f5a5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095f5ab: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095f5b1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095f5b7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095f5bd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095f5c6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095f5cf: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095f5d8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095f5e0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095f5e8: 48 83 bd 58 03 00 00 00     	cmp	qword ptr [rbp + 0x358], 0x0
14095f5f0: 74 19                       	je	0x14095f60b <.text+0x95e60b>
14095f5f2: 41 b8 01 00 00 00           	mov	r8d, 0x1
14095f5f8: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
14095f5ff: 48 8b 95 58 03 00 00        	mov	rdx, qword ptr [rbp + 0x358]
14095f606: e8 15 46 cb ff              	call	0x140613c20 <.text+0x612c20>
14095f60b: c6 85 87 03 00 00 00        	mov	byte ptr [rbp + 0x387], 0x0
14095f612: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095f61a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095f622: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095f62b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095f634: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095f63d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095f643: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095f649: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095f64f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095f655: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095f65b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095f662: 5b                          	pop	rbx
14095f663: 5f                          	pop	rdi
14095f664: 5e                          	pop	rsi
14095f665: 41 5c                       	pop	r12
14095f667: 41 5d                       	pop	r13
14095f669: 41 5e                       	pop	r14
14095f66b: 41 5f                       	pop	r15
14095f66d: 5d                          	pop	rbp
14095f66e: c3                          	ret
14095f66f: 90                          	nop
14095f670: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095f675: 55                          	push	rbp
14095f676: 41 57                       	push	r15
14095f678: 41 56                       	push	r14
14095f67a: 41 55                       	push	r13
14095f67c: 41 54                       	push	r12
14095f67e: 56                          	push	rsi
14095f67f: 57                          	push	rdi
14095f680: 53                          	push	rbx
14095f681: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095f688: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095f68f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095f695: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095f69b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095f6a1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095f6a7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095f6ad: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095f6b6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095f6bf: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095f6c8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095f6d0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095f6d8: 80 bd 85 03 00 00 00        	cmp	byte ptr [rbp + 0x385], 0x0
14095f6df: 0f 84 8a 00 00 00           	je	0x14095f76f <.text+0x95e76f>
14095f6e5: 48 8b 85 58 02 00 00        	mov	rax, qword ptr [rbp + 0x258]
14095f6ec: 48 85 c0                    	test	rax, rax
14095f6ef: 74 1a                       	je	0x14095f70b <.text+0x95e70b>
14095f6f1: 48 8b 8d 60 02 00 00        	mov	rcx, qword ptr [rbp + 0x260]
14095f6f8: 48 c1 e0 02                 	shl	rax, 0x2
14095f6fc: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
14095f700: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095f706: e8 15 45 cb ff              	call	0x140613c20 <.text+0x612c20>
14095f70b: c6 85 86 03 00 00 00        	mov	byte ptr [rbp + 0x386], 0x0
14095f712: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095f71a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095f722: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095f72b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095f734: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095f73d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095f743: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095f749: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095f74f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095f755: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095f75b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095f762: 5b                          	pop	rbx
14095f763: 5f                          	pop	rdi
14095f764: 5e                          	pop	rsi
14095f765: 41 5c                       	pop	r12
14095f767: 41 5d                       	pop	r13
14095f769: 41 5e                       	pop	r14
14095f76b: 41 5f                       	pop	r15
14095f76d: 5d                          	pop	rbp
14095f76e: c3                          	ret
14095f76f: c6 85 86 03 00 00 00        	mov	byte ptr [rbp + 0x386], 0x0
14095f776: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095f77e: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095f786: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095f78f: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095f798: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095f7a1: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095f7a7: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095f7ad: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095f7b3: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095f7b9: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095f7bf: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095f7c6: 5b                          	pop	rbx
14095f7c7: 5f                          	pop	rdi
14095f7c8: 5e                          	pop	rsi
14095f7c9: 41 5c                       	pop	r12
14095f7cb: 41 5d                       	pop	r13
14095f7cd: 41 5e                       	pop	r14
14095f7cf: 41 5f                       	pop	r15
14095f7d1: 5d                          	pop	rbp
14095f7d2: c3                          	ret
14095f7d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14095f7e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095f7e5: 55                          	push	rbp
14095f7e6: 41 57                       	push	r15
14095f7e8: 41 56                       	push	r14
14095f7ea: 41 55                       	push	r13
14095f7ec: 41 54                       	push	r12
14095f7ee: 56                          	push	rsi
14095f7ef: 57                          	push	rdi
14095f7f0: 53                          	push	rbx
14095f7f1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095f7f8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095f7ff: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095f805: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095f80b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095f811: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095f817: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095f81d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095f826: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095f82f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095f838: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095f840: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095f848: 80 bd 87 03 00 00 00        	cmp	byte ptr [rbp + 0x387], 0x0
14095f84f: 0f 84 8a 00 00 00           	je	0x14095f8df <.text+0x95e8df>
14095f855: 48 8b 85 e0 01 00 00        	mov	rax, qword ptr [rbp + 0x1e0]
14095f85c: 48 85 c0                    	test	rax, rax
14095f85f: 74 1a                       	je	0x14095f87b <.text+0x95e87b>
14095f861: 48 8b 8d e8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1e8]
14095f868: 48 c1 e0 02                 	shl	rax, 0x2
14095f86c: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
14095f870: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095f876: e8 a5 43 cb ff              	call	0x140613c20 <.text+0x612c20>
14095f87b: c6 85 86 03 00 00 00        	mov	byte ptr [rbp + 0x386], 0x0
14095f882: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095f88a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095f892: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095f89b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095f8a4: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095f8ad: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095f8b3: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095f8b9: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095f8bf: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095f8c5: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095f8cb: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095f8d2: 5b                          	pop	rbx
14095f8d3: 5f                          	pop	rdi
14095f8d4: 5e                          	pop	rsi
14095f8d5: 41 5c                       	pop	r12
14095f8d7: 41 5d                       	pop	r13
14095f8d9: 41 5e                       	pop	r14
14095f8db: 41 5f                       	pop	r15
14095f8dd: 5d                          	pop	rbp
14095f8de: c3                          	ret
14095f8df: c6 85 86 03 00 00 00        	mov	byte ptr [rbp + 0x386], 0x0
14095f8e6: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095f8ee: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095f8f6: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095f8ff: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095f908: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095f911: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095f917: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095f91d: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095f923: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095f929: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095f92f: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095f936: 5b                          	pop	rbx
14095f937: 5f                          	pop	rdi
14095f938: 5e                          	pop	rsi
14095f939: 41 5c                       	pop	r12
14095f93b: 41 5d                       	pop	r13
14095f93d: 41 5e                       	pop	r14
14095f93f: 41 5f                       	pop	r15
14095f941: 5d                          	pop	rbp
14095f942: c3                          	ret
14095f943: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14095f950: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095f955: 55                          	push	rbp
14095f956: 41 57                       	push	r15
14095f958: 41 56                       	push	r14
14095f95a: 41 55                       	push	r13
14095f95c: 41 54                       	push	r12
14095f95e: 56                          	push	rsi
14095f95f: 57                          	push	rdi
14095f960: 53                          	push	rbx
14095f961: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095f968: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095f96f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095f975: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095f97b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095f981: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095f987: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095f98d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095f996: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095f99f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095f9a8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095f9b0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095f9b8: 80 bd 86 03 00 00 00        	cmp	byte ptr [rbp + 0x386], 0x0
14095f9bf: 0f 84 83 00 00 00           	je	0x14095fa48 <.text+0x95ea48>
14095f9c5: 48 8b 85 c0 01 00 00        	mov	rax, qword ptr [rbp + 0x1c0]
14095f9cc: 48 85 c0                    	test	rax, rax
14095f9cf: 74 1a                       	je	0x14095f9eb <.text+0x95e9eb>
14095f9d1: 48 8b 8d c8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1c8]
14095f9d8: 48 c1 e0 02                 	shl	rax, 0x2
14095f9dc: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
14095f9e0: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095f9e6: e8 35 42 cb ff              	call	0x140613c20 <.text+0x612c20>
14095f9eb: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095f9f3: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095f9fb: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095fa04: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095fa0d: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095fa16: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095fa1c: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095fa22: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095fa28: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095fa2e: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095fa34: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095fa3b: 5b                          	pop	rbx
14095fa3c: 5f                          	pop	rdi
14095fa3d: 5e                          	pop	rsi
14095fa3e: 41 5c                       	pop	r12
14095fa40: 41 5d                       	pop	r13
14095fa42: 41 5e                       	pop	r14
14095fa44: 41 5f                       	pop	r15
14095fa46: 5d                          	pop	rbp
14095fa47: c3                          	ret
14095fa48: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095fa50: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095fa58: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095fa61: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095fa6a: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095fa73: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095fa79: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095fa7f: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095fa85: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095fa8b: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095fa91: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095fa98: 5b                          	pop	rbx
14095fa99: 5f                          	pop	rdi
14095fa9a: 5e                          	pop	rsi
14095fa9b: 41 5c                       	pop	r12
14095fa9d: 41 5d                       	pop	r13
14095fa9f: 41 5e                       	pop	r14
14095faa1: 41 5f                       	pop	r15
14095faa3: 5d                          	pop	rbp
14095faa4: c3                          	ret
14095faa5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
14095fab0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095fab5: 55                          	push	rbp
14095fab6: 41 57                       	push	r15
14095fab8: 41 56                       	push	r14
14095faba: 41 55                       	push	r13
14095fabc: 41 54                       	push	r12
14095fabe: 56                          	push	rsi
14095fabf: 57                          	push	rdi
14095fac0: 53                          	push	rbx
14095fac1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095fac8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095facf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095fad5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095fadb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095fae1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095fae7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095faed: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095faf6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095faff: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095fb08: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095fb10: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095fb18: 48 8b 95 00 03 00 00        	mov	rdx, qword ptr [rbp + 0x300]
14095fb1f: 48 85 d2                    	test	rdx, rdx
14095fb22: 74 16                       	je	0x14095fb3a <.text+0x95eb3a>
14095fb24: 48 c1 e2 02                 	shl	rdx, 0x2
14095fb28: 41 b8 04 00 00 00           	mov	r8d, 0x4
14095fb2e: 48 8b 8d 78 03 00 00        	mov	rcx, qword ptr [rbp + 0x378]
14095fb35: e8 e6 40 cb ff              	call	0x140613c20 <.text+0x612c20>
14095fb3a: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095fb42: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095fb4a: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095fb53: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095fb5c: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095fb65: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095fb6b: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095fb71: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095fb77: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095fb7d: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095fb83: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095fb8a: 5b                          	pop	rbx
14095fb8b: 5f                          	pop	rdi
14095fb8c: 5e                          	pop	rsi
14095fb8d: 41 5c                       	pop	r12
14095fb8f: 41 5d                       	pop	r13
14095fb91: 41 5e                       	pop	r14
14095fb93: 41 5f                       	pop	r15
14095fb95: 5d                          	pop	rbp
14095fb96: c3                          	ret
14095fb97: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
14095fba0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14095fba5: 55                          	push	rbp
14095fba6: 41 57                       	push	r15
14095fba8: 41 56                       	push	r14
14095fbaa: 41 55                       	push	r13
14095fbac: 41 54                       	push	r12
14095fbae: 56                          	push	rsi
14095fbaf: 57                          	push	rdi
14095fbb0: 53                          	push	rbx
14095fbb1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14095fbb8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14095fbbf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14095fbc5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14095fbcb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14095fbd1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14095fbd7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14095fbdd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14095fbe6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14095fbef: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14095fbf8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14095fc00: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14095fc08: 48 83 bd 80 01 00 00 00     	cmp	qword ptr [rbp + 0x180], 0x0
14095fc10: 74 2f                       	je	0x14095fc41 <.text+0x95ec41>
14095fc12: 48 8d 85 88 01 00 00        	lea	rax, [rbp + 0x188]
14095fc19: 48 89 85 08 01 00 00        	mov	qword ptr [rbp + 0x108], rax
14095fc20: 48 8d 85 08 01 00 00        	lea	rax, [rbp + 0x108]
14095fc27: 48 89 85 00 01 00 00        	mov	qword ptr [rbp + 0x100], rax
14095fc2e: 48 8d 0d fb 4f 15 02        	lea	rcx, [rip + 0x2154ffb]  # 0x142ab4c30
14095fc35: 48 8d 95 00 01 00 00        	lea	rdx, [rbp + 0x100]
14095fc3c: e8 8f f6 9c 00              	call	0x14132f2d0 <.text+0x132e2d0>
14095fc41: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14095fc49: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14095fc51: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14095fc5a: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14095fc63: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14095fc6c: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14095fc72: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14095fc78: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14095fc7e: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14095fc84: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14095fc8a: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14095fc91: 5b                          	pop	rbx
14095fc92: 5f                          	pop	rdi
14095fc93: 5e                          	pop	rsi
14095fc94: 41 5c                       	pop	r12
14095fc96: 41 5d                       	pop	r13
14095fc98: 41 5e                       	pop	r14
14095fc9a: 41 5f                       	pop	r15
14095fc9c: 5d                          	pop	rbp
14095fc9d: c3                          	ret
14095fc9e: cc                          	int3
14095fc9f: cc                          	int3
14095fca0: 55                          	push	rbp
14095fca1: 41 57                       	push	r15
14095fca3: 41 56                       	push	r14
14095fca5: 41 55                       	push	r13
14095fca7: 41 54                       	push	r12
14095fca9: 56                          	push	rsi
14095fcaa: 57                          	push	rdi
14095fcab: 53                          	push	rbx
14095fcac: 48 81 ec b8 01 00 00        	sub	rsp, 0x1b8
