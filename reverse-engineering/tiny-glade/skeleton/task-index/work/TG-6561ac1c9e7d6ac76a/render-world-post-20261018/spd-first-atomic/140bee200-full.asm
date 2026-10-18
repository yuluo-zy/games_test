
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140bee200: 55                          	push	rbp
140bee201: 41 57                       	push	r15
140bee203: 41 56                       	push	r14
140bee205: 41 55                       	push	r13
140bee207: 41 54                       	push	r12
140bee209: 56                          	push	rsi
140bee20a: 57                          	push	rdi
140bee20b: 53                          	push	rbx
140bee20c: 48 81 ec f8 01 00 00        	sub	rsp, 0x1f8
140bee213: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140bee21b: 48 c7 85 70 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x170], -0x2
140bee226: 4c 89 8d 48 01 00 00        	mov	qword ptr [rbp + 0x148], r9
140bee22d: 4d 89 c6                    	mov	r14, r8
140bee230: 49 89 d5                    	mov	r13, rdx
140bee233: 48 89 ce                    	mov	rsi, rcx
140bee236: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
140bee23d: 48 8b 38                    	mov	rdi, qword ptr [rax]
140bee240: 8b 58 08                    	mov	ebx, dword ptr [rax + 0x8]
140bee243: c7 45 e8 0c 00 00 00        	mov	dword ptr [rbp - 0x18], 0xc
140bee24a: 48 c7 45 f0 00 00 00 00     	mov	qword ptr [rbp - 0x10], 0x0
140bee252: c7 45 f8 00 00 00 00        	mov	dword ptr [rbp - 0x8], 0x0
140bee259: 48 89 7d 00                 	mov	qword ptr [rbp], rdi
140bee25d: 89 5d 08                    	mov	dword ptr [rbp + 0x8], ebx
140bee260: 48 c7 45 0c 00 00 00 00     	mov	qword ptr [rbp + 0xc], 0x0
140bee268: 48 c7 45 18 00 00 00 00     	mov	qword ptr [rbp + 0x18], 0x0
140bee270: 48 8b 8a c8 05 00 00        	mov	rcx, qword ptr [rdx + 0x5c8]
140bee277: 48 8d 55 e8                 	lea	rdx, [rbp - 0x18]
140bee27b: 4c 8d 8d d0 00 00 00        	lea	r9, [rbp + 0xd0]
140bee282: 48 89 8d 58 01 00 00        	mov	qword ptr [rbp + 0x158], rcx
140bee289: 45 31 c0                    	xor	r8d, r8d
140bee28c: 41 ff 95 08 01 00 00        	call	qword ptr [r13 + 0x108]
140bee293: 85 c0                       	test	eax, eax
140bee295: 0f 85 17 05 00 00           	jne	0x140bee7b2 <.text+0xbed7b2>
140bee29b: 48 8b 95 f0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1f0]
140bee2a2: 4c 8b a5 d0 00 00 00        	mov	r12, qword ptr [rbp + 0xd0]
140bee2a9: 4d 85 f6                    	test	r14, r14
140bee2ac: 0f 84 a0 00 00 00           	je	0x140bee352 <.text+0xbed352>
140bee2b2: 4c 89 ad 60 01 00 00        	mov	qword ptr [rbp + 0x160], r13
140bee2b9: 49 89 fd                    	mov	r13, rdi
140bee2bc: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
140bee2c3: 4c 8b 85 f8 01 00 00        	mov	r8, qword ptr [rbp + 0x1f8]
140bee2ca: e8 01 b5 18 00              	call	0x140d797d0 <.text+0xd787d0>
140bee2cf: 31 c0                       	xor	eax, eax
140bee2d1: 48 3b 85 88 00 00 00        	cmp	rax, qword ptr [rbp + 0x88]
140bee2d8: 0f 81 42 05 00 00           	jno	0x140bee820 <.text+0xbed820>
140bee2de: 48 8b bd 90 00 00 00        	mov	rdi, qword ptr [rbp + 0x90]
140bee2e5: 4c 8b bd 98 00 00 00        	mov	r15, qword ptr [rbp + 0x98]
140bee2ec: c7 85 d0 00 00 00 00 be 9c 3b       	mov	dword ptr [rbp + 0xd0], 0x3b9cbe00
140bee2f6: 48 c7 85 d8 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xd8], 0x0
140bee301: c7 85 e0 00 00 00 09 00 00 00       	mov	dword ptr [rbp + 0xe0], 0x9
140bee30b: 4c 89 a5 e8 00 00 00        	mov	qword ptr [rbp + 0xe8], r12
140bee312: 48 89 bd f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rdi
140bee319: 49 8b 4e 40                 	mov	rcx, qword ptr [r14 + 0x40]
140bee31d: 48 8d 95 d0 00 00 00        	lea	rdx, [rbp + 0xd0]
140bee324: 41 ff 16                    	call	qword ptr [r14]
140bee327: 85 c0                       	test	eax, eax
140bee329: 0f 85 3a 05 00 00           	jne	0x140bee869 <.text+0xbed869>
140bee32f: c6 07 00                    	mov	byte ptr [rdi], 0x0
140bee332: 4d 85 ff                    	test	r15, r15
140bee335: 48 89 f9                    	mov	rcx, rdi
140bee338: 4c 89 ef                    	mov	rdi, r13
140bee33b: 4c 8b ad 60 01 00 00        	mov	r13, qword ptr [rbp + 0x160]
140bee342: 74 0e                       	je	0x140bee352 <.text+0xbed352>
140bee344: 41 b8 01 00 00 00           	mov	r8d, 0x1
140bee34a: 4c 89 fa                    	mov	rdx, r15
140bee34d: e8 ce 58 a2 ff              	call	0x140613c20 <.text+0x612c20>
140bee352: 4c 8b bd e0 01 00 00        	mov	r15, qword ptr [rbp + 0x1e0]
140bee359: 4c 8d 45 58                 	lea	r8, [rbp + 0x58]
140bee35d: 48 8b 8d 58 01 00 00        	mov	rcx, qword ptr [rbp + 0x158]
140bee364: 4c 89 e2                    	mov	rdx, r12
140bee367: 41 ff 55 70                 	call	qword ptr [r13 + 0x70]
140bee36b: 48 8b 45 60                 	mov	rax, qword ptr [rbp + 0x60]
140bee36f: 4c 8b b5 e8 01 00 00        	mov	r14, qword ptr [rbp + 0x1e8]
140bee376: 41 0f b7 4e 0c              	movzx	ecx, word ptr [r14 + 0xc]
140bee37b: 48 39 c1                    	cmp	rcx, rax
140bee37e: 48 0f 47 c1                 	cmova	rax, rcx
140bee382: f7 c3 00 04 00 00           	test	ebx, 0x400
140bee388: b9 10 00 00 00              	mov	ecx, 0x10
140bee38d: ba 40 00 00 00              	mov	edx, 0x40
140bee392: 48 0f 44 d1                 	cmove	rdx, rcx
140bee396: 48 39 c2                    	cmp	rdx, rax
140bee399: 48 0f 46 d0                 	cmovbe	rdx, rax
140bee39d: 48 89 55 60                 	mov	qword ptr [rbp + 0x60], rdx
140bee3a1: b1 01                       	mov	cl, 0x1
140bee3a3: 31 c0                       	xor	eax, eax
140bee3a5: f0                          	lock
140bee3a6: 41 0f b0 0f                 	cmpxchg	byte ptr [r15], cl
140bee3aa: 0f 85 b2 03 00 00           	jne	0x140bee762 <.text+0xbed762>
140bee3b0: 4c 89 f9                    	mov	rcx, r15
140bee3b3: e8 b8 38 09 00              	call	0x140c81c70 <.text+0xc80c70>
140bee3b8: 49 8d 57 08                 	lea	rdx, [r15 + 0x8]
140bee3bc: 48 8b 45 68                 	mov	rax, qword ptr [rbp + 0x68]
140bee3c0: 48 89 45 d8                 	mov	qword ptr [rbp - 0x28], rax
140bee3c4: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
140bee3c8: 0f 11 45 c8                 	movups	xmmword ptr [rbp - 0x38], xmm0
140bee3cc: 41 0f b6 46 0e              	movzx	eax, byte ptr [r14 + 0xe]
140bee3d1: 48 c7 45 b8 01 00 00 00     	mov	qword ptr [rbp - 0x48], 0x1
140bee3d9: 48 c7 45 c0 00 00 00 00     	mov	qword ptr [rbp - 0x40], 0x0
140bee3e1: 88 45 e1                    	mov	byte ptr [rbp - 0x1f], al
140bee3e4: c6 45 e0 01                 	mov	byte ptr [rbp - 0x20], 0x1
140bee3e8: 48 c7 45 a8 02 00 00 00     	mov	qword ptr [rbp - 0x58], 0x2
140bee3f0: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
140bee3f7: 4c 8d 45 a8                 	lea	r8, [rbp - 0x58]
140bee3fb: e8 40 f6 01 00              	call	0x140c0da40 <.text+0xc0ca40>
140bee400: 48 8b 45 68                 	mov	rax, qword ptr [rbp + 0x68]
140bee404: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
140bee408: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
140bee40c: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
140bee410: 80 bd 14 01 00 00 02        	cmp	byte ptr [rbp + 0x114], 0x2
140bee417: 4c 8b 8d d0 00 00 00        	mov	r9, qword ptr [rbp + 0xd0]
140bee41e: 48 8b 9d e8 00 00 00        	mov	rbx, qword ptr [rbp + 0xe8]
140bee425: 4c 8b bd f0 01 00 00        	mov	r15, qword ptr [rbp + 0x1f0]
140bee42c: 75 61                       	jne	0x140bee48f <.text+0xbed48f>
140bee42e: 48 8b 85 d8 00 00 00        	mov	rax, qword ptr [rbp + 0xd8]
140bee435: 48 89 85 50 01 00 00        	mov	qword ptr [rbp + 0x150], rax
140bee43c: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140bee443: 48 8b 8d f8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1f8]
140bee44a: 48 85 c9                    	test	rcx, rcx
140bee44d: 48 89 85 40 01 00 00        	mov	qword ptr [rbp + 0x140], rax
140bee454: 4c 89 8d 60 01 00 00        	mov	qword ptr [rbp + 0x160], r9
140bee45b: 0f 88 49 04 00 00           	js	0x140bee8aa <.text+0xbed8aa>
140bee461: 4c 89 65 38                 	mov	qword ptr [rbp + 0x38], r12
140bee465: 74 5f                       	je	0x140bee4c6 <.text+0xbed4c6>
140bee467: 0f b6 05 9b 67 a4 02        	movzx	eax, byte ptr [rip + 0x2a4679b] # 0x143634c09
140bee46e: 41 bc 01 00 00 00           	mov	r12d, 0x1
140bee474: ba 01 00 00 00              	mov	edx, 0x1
140bee479: e8 92 57 a2 ff              	call	0x140613c10 <.text+0x612c10>
140bee47e: 48 85 c0                    	test	rax, rax
140bee481: 0f 84 26 04 00 00           	je	0x140bee8ad <.text+0xbed8ad>
140bee487: 49 89 c6                    	mov	r14, rax
140bee48a: 49 89 fc                    	mov	r12, rdi
140bee48d: eb 40                       	jmp	0x140bee4cf <.text+0xbed4cf>
140bee48f: 4c 8b 85 d8 00 00 00        	mov	r8, qword ptr [rbp + 0xd8]
140bee496: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140bee49d: 4c 8b b5 f0 00 00 00        	mov	r14, qword ptr [rbp + 0xf0]
140bee4a4: 48 8b 85 f8 00 00 00        	mov	rax, qword ptr [rbp + 0xf8]
140bee4ab: 0f 10 85 00 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x100]
140bee4b2: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
140bee4b6: 48 8b 95 10 01 00 00        	mov	rdx, qword ptr [rbp + 0x110]
140bee4bd: 48 89 95 80 00 00 00        	mov	qword ptr [rbp + 0x80], rdx
140bee4c4: eb 60                       	jmp	0x140bee526 <.text+0xbed526>
140bee4c6: 49 89 fc                    	mov	r12, rdi
140bee4c9: 41 be 01 00 00 00           	mov	r14d, 0x1
140bee4cf: 4c 89 f1                    	mov	rcx, r14
140bee4d2: 4c 89 fa                    	mov	rdx, r15
140bee4d5: 48 8b bd f8 01 00 00        	mov	rdi, qword ptr [rbp + 0x1f8]
140bee4dc: 49 89 f8                    	mov	r8, rdi
140bee4df: e8 1c 57 d3 01              	call	0x142923c00 <.text+0x2922c00>
140bee4e4: 0f 28 45 20                 	movaps	xmm0, xmmword ptr [rbp + 0x20]
140bee4e8: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
140bee4ec: 48 8b 45 30                 	mov	rax, qword ptr [rbp + 0x30]
140bee4f0: 48 89 85 80 00 00 00        	mov	qword ptr [rbp + 0x80], rax
140bee4f7: 48 8b 8d 60 01 00 00        	mov	rcx, qword ptr [rbp + 0x160]
140bee4fe: 48 83 f9 0b                 	cmp	rcx, 0xb
140bee502: 0f 85 de 01 00 00           	jne	0x140bee6e6 <.text+0xbed6e6>
140bee508: 4c 8b 8d 50 01 00 00        	mov	r9, qword ptr [rbp + 0x150]
140bee50f: 48 89 d9                    	mov	rcx, rbx
140bee512: 48 89 f8                    	mov	rax, rdi
140bee515: 48 89 fb                    	mov	rbx, rdi
140bee518: 4c 89 e7                    	mov	rdi, r12
140bee51b: 4c 8b 65 38                 	mov	r12, qword ptr [rbp + 0x38]
140bee51f: 4c 8b 85 40 01 00 00        	mov	r8, qword ptr [rbp + 0x140]
140bee526: 48 8b 95 80 00 00 00        	mov	rdx, qword ptr [rbp + 0x80]
140bee52d: 48 89 55 50                 	mov	qword ptr [rbp + 0x50], rdx
140bee531: 0f 28 45 70                 	movaps	xmm0, xmmword ptr [rbp + 0x70]
140bee535: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
140bee539: 4c 89 8d 88 00 00 00        	mov	qword ptr [rbp + 0x88], r9
140bee540: 4c 89 85 90 00 00 00        	mov	qword ptr [rbp + 0x90], r8
140bee547: 48 89 8d 98 00 00 00        	mov	qword ptr [rbp + 0x98], rcx
140bee54e: 0f 11 85 b8 00 00 00        	movups	xmmword ptr [rbp + 0xb8], xmm0
140bee555: 48 89 95 c8 00 00 00        	mov	qword ptr [rbp + 0xc8], rdx
140bee55c: 48 89 9d a0 00 00 00        	mov	qword ptr [rbp + 0xa0], rbx
140bee563: 4c 89 b5 a8 00 00 00        	mov	qword ptr [rbp + 0xa8], r14
140bee56a: 48 89 85 b0 00 00 00        	mov	qword ptr [rbp + 0xb0], rax
140bee571: c6 85 6f 01 00 00 01        	mov	byte ptr [rbp + 0x16f], 0x1
140bee578: 4c 8b b5 e0 01 00 00        	mov	r14, qword ptr [rbp + 0x1e0]
140bee57f: 4c 89 f1                    	mov	rcx, r14
140bee582: e8 89 38 09 00              	call	0x140c81e10 <.text+0xc80e10>
140bee587: 48 8b 9d 48 01 00 00        	mov	rbx, qword ptr [rbp + 0x148]
140bee58e: 31 c9                       	xor	ecx, ecx
140bee590: b0 01                       	mov	al, 0x1
140bee592: f0                          	lock
140bee593: 41 0f b0 0e                 	cmpxchg	byte ptr [r14], cl
140bee597: 0f 85 d8 01 00 00           	jne	0x140bee775 <.text+0xbed775>
140bee59d: c6 85 6f 01 00 00 01        	mov	byte ptr [rbp + 0x16f], 0x1
140bee5a4: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
140bee5ab: e8 00 27 ae ff              	call	0x1406d0cb0 <.text+0x6cfcb0>
140bee5b0: 49 89 c6                    	mov	r14, rax
140bee5b3: c6 85 6f 01 00 00 01        	mov	byte ptr [rbp + 0x16f], 0x1
140bee5ba: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
140bee5c1: e8 ea 82 a9 ff              	call	0x1406868b0 <.text+0x6858b0>
140bee5c6: 48 8b 8d 58 01 00 00        	mov	rcx, qword ptr [rbp + 0x158]
140bee5cd: 4c 89 e2                    	mov	rdx, r12
140bee5d0: 4d 89 f0                    	mov	r8, r14
140bee5d3: 49 89 c1                    	mov	r9, rax
140bee5d6: 41 ff 55 60                 	call	qword ptr [r13 + 0x60]
140bee5da: 85 c0                       	test	eax, eax
140bee5dc: 0f 85 01 02 00 00           	jne	0x140bee7e3 <.text+0xbed7e3>
140bee5e2: c6 85 6f 01 00 00 01        	mov	byte ptr [rbp + 0x16f], 0x1
140bee5e9: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
140bee5f0: e8 db 2d 02 00              	call	0x140c113d0 <.text+0xc103d0>
140bee5f5: 49 89 c6                    	mov	r14, rax
140bee5f8: 48 85 c0                    	test	rax, rax
140bee5fb: 0f 95 c0                    	setne	al
140bee5fe: 48 39 d7                    	cmp	rdi, rdx
140bee601: 0f 97 c1                    	seta	cl
140bee604: 20 c1                       	and	cl, al
140bee606: 80 f9 01                    	cmp	cl, 0x1
140bee609: 0f 84 7c 01 00 00           	je	0x140bee78b <.text+0xbed78b>
140bee60f: 48 8b 85 c8 00 00 00        	mov	rax, qword ptr [rbp + 0xc8]
140bee616: 48 89 85 38 01 00 00        	mov	qword ptr [rbp + 0x138], rax
140bee61d: 0f 10 85 88 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x88]
140bee624: 0f 10 8d 98 00 00 00        	movups	xmm1, xmmword ptr [rbp + 0x98]
140bee62b: 0f 10 95 a8 00 00 00        	movups	xmm2, xmmword ptr [rbp + 0xa8]
140bee632: 0f 10 9d b8 00 00 00        	movups	xmm3, xmmword ptr [rbp + 0xb8]
140bee639: 0f 11 9d 28 01 00 00        	movups	xmmword ptr [rbp + 0x128], xmm3
140bee640: 0f 11 95 18 01 00 00        	movups	xmmword ptr [rbp + 0x118], xmm2
140bee647: 0f 11 8d 08 01 00 00        	movups	xmmword ptr [rbp + 0x108], xmm1
140bee64e: 0f 11 85 f8 00 00 00        	movups	xmmword ptr [rbp + 0xf8], xmm0
140bee655: 4c 89 a5 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], r12
140bee65c: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
140bee663: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
140bee666: 0f 11 85 e0 00 00 00        	movups	xmmword ptr [rbp + 0xe0], xmm0
140bee66d: 4c 89 bd d0 00 00 00        	mov	qword ptr [rbp + 0xd0], r15
140bee674: 48 8b 85 f8 01 00 00        	mov	rax, qword ptr [rbp + 0x1f8]
140bee67b: 48 89 85 d8 00 00 00        	mov	qword ptr [rbp + 0xd8], rax
140bee682: 48 8d 8b 80 01 00 00        	lea	rcx, [rbx + 0x180]
140bee689: c6 85 6f 01 00 00 00        	mov	byte ptr [rbp + 0x16f], 0x0
140bee690: 48 8d 95 d0 00 00 00        	lea	rdx, [rbp + 0xd0]
140bee697: e8 04 02 cc 00              	call	0x1418ae8a0 <.text+0x18ad8a0>
140bee69c: 8b 48 0c                    	mov	ecx, dword ptr [rax + 0xc]
140bee69f: 0f b7 93 50 0a 00 00        	movzx	edx, word ptr [rbx + 0xa50]
140bee6a6: 4c 8b 03                    	mov	r8, qword ptr [rbx]
140bee6a9: 49 89 c1                    	mov	r9, rax
140bee6ac: 49 83 c1 10                 	add	r9, 0x10
140bee6b0: 4c 89 4e 08                 	mov	qword ptr [rsi + 0x8], r9
140bee6b4: 4c 89 46 10                 	mov	qword ptr [rsi + 0x10], r8
140bee6b8: 48 89 46 18                 	mov	qword ptr [rsi + 0x18], rax
140bee6bc: 89 4e 20                    	mov	dword ptr [rsi + 0x20], ecx
140bee6bf: 66 89 56 24                 	mov	word ptr [rsi + 0x24], dx
140bee6c3: c6 46 26 01                 	mov	byte ptr [rsi + 0x26], 0x1
140bee6c7: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
140bee6ce: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
140bee6d1: 0f 11 46 28                 	movups	xmmword ptr [rsi + 0x28], xmm0
140bee6d5: 4c 89 76 38                 	mov	qword ptr [rsi + 0x38], r14
140bee6d9: 48 89 7e 40                 	mov	qword ptr [rsi + 0x40], rdi
140bee6dd: 48 c7 06 0b 00 00 00        	mov	qword ptr [rsi], 0xb
140bee6e4: eb 65                       	jmp	0x140bee74b <.text+0xbed74b>
140bee6e6: 48 8b 85 80 00 00 00        	mov	rax, qword ptr [rbp + 0x80]
140bee6ed: 48 89 45 50                 	mov	qword ptr [rbp + 0x50], rax
140bee6f1: 0f 28 45 70                 	movaps	xmm0, xmmword ptr [rbp + 0x70]
140bee6f5: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
140bee6f9: 48 8b 85 50 01 00 00        	mov	rax, qword ptr [rbp + 0x150]
140bee700: 48 89 46 08                 	mov	qword ptr [rsi + 0x8], rax
140bee704: 48 8b 85 40 01 00 00        	mov	rax, qword ptr [rbp + 0x140]
140bee70b: 48 89 46 10                 	mov	qword ptr [rsi + 0x10], rax
140bee70f: 48 89 5e 18                 	mov	qword ptr [rsi + 0x18], rbx
140bee713: 0f 28 45 40                 	movaps	xmm0, xmmword ptr [rbp + 0x40]
140bee717: 0f 11 46 38                 	movups	xmmword ptr [rsi + 0x38], xmm0
140bee71b: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
140bee71f: 48 89 46 48                 	mov	qword ptr [rsi + 0x48], rax
140bee723: 48 89 0e                    	mov	qword ptr [rsi], rcx
140bee726: 48 89 7e 20                 	mov	qword ptr [rsi + 0x20], rdi
140bee72a: 4c 89 76 28                 	mov	qword ptr [rsi + 0x28], r14
140bee72e: 48 89 7e 30                 	mov	qword ptr [rsi + 0x30], rdi
140bee732: 48 8b bd e0 01 00 00        	mov	rdi, qword ptr [rbp + 0x1e0]
140bee739: 48 89 f9                    	mov	rcx, rdi
140bee73c: e8 cf 36 09 00              	call	0x140c81e10 <.text+0xc80e10>
140bee741: 31 c9                       	xor	ecx, ecx
140bee743: b0 01                       	mov	al, 0x1
140bee745: f0                          	lock
140bee746: 0f b0 0f                    	cmpxchg	byte ptr [rdi], cl
140bee749: 75 5b                       	jne	0x140bee7a6 <.text+0xbed7a6>
140bee74b: 48 89 f0                    	mov	rax, rsi
140bee74e: 48 81 c4 f8 01 00 00        	add	rsp, 0x1f8
140bee755: 5b                          	pop	rbx
140bee756: 5f                          	pop	rdi
140bee757: 5e                          	pop	rsi
140bee758: 41 5c                       	pop	r12
140bee75a: 41 5d                       	pop	r13
140bee75c: 41 5e                       	pop	r14
140bee75e: 41 5f                       	pop	r15
140bee760: 5d                          	pop	rbp
140bee761: c3                          	ret
140bee762: 4c 89 f9                    	mov	rcx, r15
140bee765: 41 b8 00 ca 9a 3b           	mov	r8d, 0x3b9aca00
140bee76b: e8 a0 ea cd 01              	call	0x1428cd210 <.text+0x28cc210>
140bee770: e9 3b fc ff ff              	jmp	0x140bee3b0 <.text+0xbed3b0>
140bee775: c6 85 6f 01 00 00 01        	mov	byte ptr [rbp + 0x16f], 0x1
140bee77c: 4c 89 f1                    	mov	rcx, r14
140bee77f: 31 d2                       	xor	edx, edx
140bee781: e8 0a f4 cd 01              	call	0x1428cdb90 <.text+0x28ccb90>
140bee786: e9 12 fe ff ff              	jmp	0x140bee59d <.text+0xbed59d>
140bee78b: c6 85 6f 01 00 00 01        	mov	byte ptr [rbp + 0x16f], 0x1
140bee792: 4c 8d 05 1f 71 f2 01        	lea	r8, [rip + 0x1f2711f]   # 0x142b158b8
140bee799: 48 89 f9                    	mov	rcx, rdi
140bee79c: e8 9f b0 ce 01              	call	0x1428d9840 <.text+0x28d8840>
140bee7a1: e9 1d 01 00 00              	jmp	0x140bee8c3 <.text+0xbed8c3>
140bee7a6: 48 89 f9                    	mov	rcx, rdi
140bee7a9: 31 d2                       	xor	edx, edx
140bee7ab: e8 e0 f3 cd 01              	call	0x1428cdb90 <.text+0x28ccb90>
140bee7b0: eb 99                       	jmp	0x140bee74b <.text+0xbed74b>
140bee7b2: 89 85 d0 00 00 00           	mov	dword ptr [rbp + 0xd0], eax
140bee7b8: 48 8d 05 61 70 f2 01        	lea	rax, [rip + 0x1f27061]  # 0x142b15820
140bee7bf: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140bee7c4: 48 8d 0d 25 70 f2 01        	lea	rcx, [rip + 0x1f27025]  # 0x142b157f0
140bee7cb: 4c 8d 0d 56 69 f2 01        	lea	r9, [rip + 0x1f26956]   # 0x142b15128
140bee7d2: 4c 8d 85 d0 00 00 00        	lea	r8, [rbp + 0xd0]
140bee7d9: ba 0d 00 00 00              	mov	edx, 0xd
140bee7de: e8 7d af ce 01              	call	0x1428d9760 <.text+0x28d8760>
140bee7e3: 89 85 d0 00 00 00           	mov	dword ptr [rbp + 0xd0], eax
140bee7e9: c6 85 6f 01 00 00 01        	mov	byte ptr [rbp + 0x16f], 0x1
140bee7f0: 48 8d 05 a9 70 f2 01        	lea	rax, [rip + 0x1f270a9]  # 0x142b158a0
140bee7f7: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140bee7fc: 48 8d 0d 85 70 f2 01        	lea	rcx, [rip + 0x1f27085]  # 0x142b15888
140bee803: 4c 8d 0d 1e 69 f2 01        	lea	r9, [rip + 0x1f2691e]   # 0x142b15128
140bee80a: 4c 8d 85 d0 00 00 00        	lea	r8, [rbp + 0xd0]
140bee811: ba 12 00 00 00              	mov	edx, 0x12
140bee816: e8 45 af ce 01              	call	0x1428d9760 <.text+0x28d8760>
140bee81b: e9 a3 00 00 00              	jmp	0x140bee8c3 <.text+0xbed8c3>
140bee820: 0f 10 85 88 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x88]
140bee827: 0f 10 8d 98 00 00 00        	movups	xmm1, xmmword ptr [rbp + 0x98]
140bee82e: 0f 29 8d e0 00 00 00        	movaps	xmmword ptr [rbp + 0xe0], xmm1
140bee835: 0f 29 85 d0 00 00 00        	movaps	xmmword ptr [rbp + 0xd0], xmm0
140bee83c: 48 8d 05 f5 6f f2 01        	lea	rax, [rip + 0x1f26ff5]  # 0x142b15838
140bee843: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140bee848: 48 8d 0d 39 69 f2 01        	lea	rcx, [rip + 0x1f26939]  # 0x142b15188
140bee84f: 4c 8d 0d 82 69 f2 01        	lea	r9, [rip + 0x1f26982]   # 0x142b151d8
140bee856: 4c 8d 85 d0 00 00 00        	lea	r8, [rbp + 0xd0]
140bee85d: ba 2b 00 00 00              	mov	edx, 0x2b
140bee862: e8 f9 ae ce 01              	call	0x1428d9760 <.text+0x28d8760>
140bee867: eb 5a                       	jmp	0x140bee8c3 <.text+0xbed8c3>
140bee869: 48 89 bd 48 01 00 00        	mov	qword ptr [rbp + 0x148], rdi
140bee870: 4c 89 bd 58 01 00 00        	mov	qword ptr [rbp + 0x158], r15
140bee877: 89 85 88 00 00 00           	mov	dword ptr [rbp + 0x88], eax
140bee87d: 48 8d 05 ec 6f f2 01        	lea	rax, [rip + 0x1f26fec]  # 0x142b15870
140bee884: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140bee889: 48 8d 0d c0 6f f2 01        	lea	rcx, [rip + 0x1f26fc0]  # 0x142b15850
140bee890: 4c 8d 0d 91 68 f2 01        	lea	r9, [rip + 0x1f26891]   # 0x142b15128
140bee897: 4c 8d 85 88 00 00 00        	lea	r8, [rbp + 0x88]
140bee89e: ba 1b 00 00 00              	mov	edx, 0x1b
140bee8a3: e8 b8 ae ce 01              	call	0x1428d9760 <.text+0x28d8760>
140bee8a8: eb 19                       	jmp	0x140bee8c3 <.text+0xbed8c3>
140bee8aa: 45 31 e4                    	xor	r12d, r12d
140bee8ad: 4c 8d 05 d4 69 f2 01        	lea	r8, [rip + 0x1f269d4]   # 0x142b15288
140bee8b4: 4c 89 e1                    	mov	rcx, r12
140bee8b7: 48 8b 95 f8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1f8]
140bee8be: e8 20 a7 ce 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
140bee8c3: 0f 0b                       	ud2
140bee8c5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
140bee8d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140bee8d5: 55                          	push	rbp
140bee8d6: 41 57                       	push	r15
140bee8d8: 41 56                       	push	r14
140bee8da: 41 55                       	push	r13
140bee8dc: 41 54                       	push	r12
140bee8de: 56                          	push	rsi
140bee8df: 57                          	push	rdi
140bee8e0: 53                          	push	rbx
140bee8e1: 48 83 ec 28                 	sub	rsp, 0x28
140bee8e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140bee8ec: 48 8b 95 d0 00 00 00        	mov	rdx, qword ptr [rbp + 0xd0]
140bee8f3: 48 85 d2                    	test	rdx, rdx
140bee8f6: 74 12                       	je	0x140bee90a <.text+0xbed90a>
140bee8f8: 48 8b 8d d8 00 00 00        	mov	rcx, qword ptr [rbp + 0xd8]
140bee8ff: 41 b8 01 00 00 00           	mov	r8d, 0x1
140bee905: e8 16 53 a2 ff              	call	0x140613c20 <.text+0x612c20>
140bee90a: 90                          	nop
140bee90b: 48 83 c4 28                 	add	rsp, 0x28
140bee90f: 5b                          	pop	rbx
140bee910: 5f                          	pop	rdi
140bee911: 5e                          	pop	rsi
140bee912: 41 5c                       	pop	r12
140bee914: 41 5d                       	pop	r13
140bee916: 41 5e                       	pop	r14
140bee918: 41 5f                       	pop	r15
140bee91a: 5d                          	pop	rbp
140bee91b: c3                          	ret
140bee91c: 0f 1f 40 00                 	nop	dword ptr [rax]
140bee920: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140bee925: 55                          	push	rbp
140bee926: 41 57                       	push	r15
140bee928: 41 56                       	push	r14
140bee92a: 41 55                       	push	r13
140bee92c: 41 54                       	push	r12
140bee92e: 56                          	push	rsi
140bee92f: 57                          	push	rdi
140bee930: 53                          	push	rbx
140bee931: 48 83 ec 28                 	sub	rsp, 0x28
140bee935: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140bee93c: 48 8b 85 48 01 00 00        	mov	rax, qword ptr [rbp + 0x148]
140bee943: c6 00 00                    	mov	byte ptr [rax], 0x0
140bee946: 48 83 bd 58 01 00 00 00     	cmp	qword ptr [rbp + 0x158], 0x0
140bee94e: 74 19                       	je	0x140bee969 <.text+0xbed969>
140bee950: 41 b8 01 00 00 00           	mov	r8d, 0x1
140bee956: 48 8b 8d 48 01 00 00        	mov	rcx, qword ptr [rbp + 0x148]
140bee95d: 48 8b 95 58 01 00 00        	mov	rdx, qword ptr [rbp + 0x158]
140bee964: e8 b7 52 a2 ff              	call	0x140613c20 <.text+0x612c20>
140bee969: 90                          	nop
140bee96a: 48 83 c4 28                 	add	rsp, 0x28
140bee96e: 5b                          	pop	rbx
140bee96f: 5f                          	pop	rdi
140bee970: 5e                          	pop	rsi
140bee971: 41 5c                       	pop	r12
140bee973: 41 5d                       	pop	r13
140bee975: 41 5e                       	pop	r14
140bee977: 41 5f                       	pop	r15
140bee979: 5d                          	pop	rbp
140bee97a: c3                          	ret
140bee97b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140bee980: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140bee985: 55                          	push	rbp
140bee986: 41 57                       	push	r15
140bee988: 41 56                       	push	r14
140bee98a: 41 55                       	push	r13
140bee98c: 41 54                       	push	r12
140bee98e: 56                          	push	rsi
140bee98f: 57                          	push	rdi
140bee990: 53                          	push	rbx
140bee991: 48 83 ec 28                 	sub	rsp, 0x28
140bee995: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140bee99c: 48 83 bd 60 01 00 00 05     	cmp	qword ptr [rbp + 0x160], 0x5
140bee9a4: 77 35                       	ja	0x140bee9db <.text+0xbed9db>
140bee9a6: b8 32 00 00 00              	mov	eax, 0x32
140bee9ab: 48 8b 8d 60 01 00 00        	mov	rcx, qword ptr [rbp + 0x160]
140bee9b2: 48 0f a3 c8                 	bt	rax, rcx
140bee9b6: 73 23                       	jae	0x140bee9db <.text+0xbed9db>
140bee9b8: 48 83 bd 50 01 00 00 00     	cmp	qword ptr [rbp + 0x150], 0x0
140bee9c0: 74 19                       	je	0x140bee9db <.text+0xbed9db>
140bee9c2: 41 b8 01 00 00 00           	mov	r8d, 0x1
140bee9c8: 48 8b 8d 40 01 00 00        	mov	rcx, qword ptr [rbp + 0x140]
140bee9cf: 48 8b 95 50 01 00 00        	mov	rdx, qword ptr [rbp + 0x150]
140bee9d6: e8 45 52 a2 ff              	call	0x140613c20 <.text+0x612c20>
140bee9db: 90                          	nop
140bee9dc: 48 83 c4 28                 	add	rsp, 0x28
140bee9e0: 5b                          	pop	rbx
140bee9e1: 5f                          	pop	rdi
140bee9e2: 5e                          	pop	rsi
140bee9e3: 41 5c                       	pop	r12
140bee9e5: 41 5d                       	pop	r13
140bee9e7: 41 5e                       	pop	r14
140bee9e9: 41 5f                       	pop	r15
140bee9eb: 5d                          	pop	rbp
140bee9ec: c3                          	ret
140bee9ed: 0f 1f 00                    	nop	dword ptr [rax]
140bee9f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140bee9f5: 55                          	push	rbp
140bee9f6: 41 57                       	push	r15
140bee9f8: 41 56                       	push	r14
140bee9fa: 41 55                       	push	r13
140bee9fc: 41 54                       	push	r12
140bee9fe: 56                          	push	rsi
140bee9ff: 57                          	push	rdi
140beea00: 53                          	push	rbx
140beea01: 48 83 ec 28                 	sub	rsp, 0x28
140beea05: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140beea0c: 80 bd 6f 01 00 00 00        	cmp	byte ptr [rbp + 0x16f], 0x0
140beea13: 75 11                       	jne	0x140beea26 <.text+0xbeda26>
140beea15: 48 83 c4 28                 	add	rsp, 0x28
140beea19: 5b                          	pop	rbx
140beea1a: 5f                          	pop	rdi
140beea1b: 5e                          	pop	rsi
140beea1c: 41 5c                       	pop	r12
140beea1e: 41 5d                       	pop	r13
140beea20: 41 5e                       	pop	r14
140beea22: 41 5f                       	pop	r15
140beea24: 5d                          	pop	rbp
140beea25: c3                          	ret
140beea26: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
140beea2d: e8 de 0b ff ff              	call	0x140bdf610 <.text+0xbde610>
140beea32: eb e1                       	jmp	0x140beea15 <.text+0xbeda15>
140beea34: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
140beea40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140beea45: 55                          	push	rbp
140beea46: 41 57                       	push	r15
140beea48: 41 56                       	push	r14
140beea4a: 41 55                       	push	r13
140beea4c: 41 54                       	push	r12
140beea4e: 56                          	push	rsi
140beea4f: 57                          	push	rdi
140beea50: 53                          	push	rbx
140beea51: 48 83 ec 28                 	sub	rsp, 0x28
140beea55: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140beea5c: 48 8b 8d e0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1e0]
140beea63: e8 c8 31 57 ff              	call	0x140161c30 <.text+0x160c30>
140beea68: 90                          	nop
140beea69: 48 83 c4 28                 	add	rsp, 0x28
140beea6d: 5b                          	pop	rbx
140beea6e: 5f                          	pop	rdi
140beea6f: 5e                          	pop	rsi
140beea70: 41 5c                       	pop	r12
140beea72: 41 5d                       	pop	r13
140beea74: 41 5e                       	pop	r14
140beea76: 41 5f                       	pop	r15
140beea78: 5d                          	pop	rbp
140beea79: c3                          	ret
