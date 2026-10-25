
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416934d0: 55                          	push	rbp
1416934d1: 41 57                       	push	r15
1416934d3: 41 56                       	push	r14
1416934d5: 41 55                       	push	r13
1416934d7: 41 54                       	push	r12
1416934d9: 56                          	push	rsi
1416934da: 57                          	push	rdi
1416934db: 53                          	push	rbx
1416934dc: 48 81 ec 28 01 00 00        	sub	rsp, 0x128
1416934e3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1416934eb: 48 c7 85 a0 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xa0], -0x2
1416934f6: 48 89 ce                    	mov	rsi, rcx
1416934f9: 48 8b 01                    	mov	rax, qword ptr [rcx]
1416934fc: 48 8b 49 08                 	mov	rcx, qword ptr [rcx + 0x8]
141693500: 48 39 c8                    	cmp	rax, rcx
141693503: 0f 87 9f 03 00 00           	ja	0x1416938a8 <.text+0x16928a8>
141693509: 48 89 d7                    	mov	rdi, rdx
14169350c: 48 8b 1e                    	mov	rbx, qword ptr [rsi]
14169350f: 4c 8b 76 08                 	mov	r14, qword ptr [rsi + 0x8]
141693513: e8 e8 34 58 ff              	call	0x140c16a00 <.text+0xc15a00>
141693518: 84 c0                       	test	al, al
14169351a: 88 85 98 00 00 00           	mov	byte ptr [rbp + 0x98], al
141693520: 74 5d                       	je	0x14169357f <.text+0x169257f>
141693522: 41 bd 43 00 00 00           	mov	r13d, 0x43
141693528: b9 43 00 00 00              	mov	ecx, 0x43
14169352d: e8 ae be ff ff              	call	0x14168f3e0 <.text+0x168e3e0>
141693532: 4c 8d 25 3a 31 5f 01        	lea	r12, [rip + 0x15f313a]  # 0x142c86673
141693539: 48 83 f8 01                 	cmp	rax, 0x1
14169353d: 0f 85 a8 00 00 00           	jne	0x1416935eb <.text+0x16925eb>
141693543: 49 89 d1                    	mov	r9, rdx
141693546: 48 85 d2                    	test	rdx, rdx
141693549: 74 3d                       	je	0x141693588 <.text+0x1692588>
14169354b: 49 83 f9 43                 	cmp	r9, 0x43
14169354f: 73 35                       	jae	0x141693586 <.text+0x1692586>
141693551: 48 8d 05 1b 31 5f 01        	lea	rax, [rip + 0x15f311b]  # 0x142c86673
141693558: 41 80 3c 01 bf              	cmp	byte ptr [r9 + rax], -0x41
14169355d: 7f 29                       	jg	0x141693588 <.text+0x1692588>
14169355f: 48 8d 05 da 31 5f 01        	lea	rax, [rip + 0x15f31da]  # 0x142c86740
141693566: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14169356b: 48 8d 0d 01 31 5f 01        	lea	rcx, [rip + 0x15f3101]  # 0x142c86673
141693572: ba 43 00 00 00              	mov	edx, 0x43
141693577: 45 31 c0                    	xor	r8d, r8d
14169357a: e8 c1 63 24 01              	call	0x1428d9940 <.text+0x28d8940>
14169357f: 31 c0                       	xor	eax, eax
141693581: e9 36 02 00 00              	jmp	0x1416937bc <.text+0x16927bc>
141693586: 75 d7                       	jne	0x14169355f <.text+0x169255f>
141693588: 4c 89 c9                    	mov	rcx, r9
14169358b: e8 50 be ff ff              	call	0x14168f3e0 <.text+0x168e3e0>
141693590: 48 83 f8 01                 	cmp	rax, 0x1
141693594: 75 55                       	jne	0x1416935eb <.text+0x16925eb>
141693596: 49 89 d0                    	mov	r8, rdx
141693599: 49 83 c0 02                 	add	r8, 0x2
14169359d: 74 39                       	je	0x1416935d8 <.text+0x16925d8>
14169359f: 49 83 f8 43                 	cmp	r8, 0x43
1416935a3: 73 31                       	jae	0x1416935d6 <.text+0x16925d6>
1416935a5: 48 8d 05 c7 30 5f 01        	lea	rax, [rip + 0x15f30c7]  # 0x142c86673
1416935ac: 41 80 3c 00 bf              	cmp	byte ptr [r8 + rax], -0x41
1416935b1: 7f 25                       	jg	0x1416935d8 <.text+0x16925d8>
1416935b3: 48 8d 05 9e 31 5f 01        	lea	rax, [rip + 0x15f319e]  # 0x142c86758
1416935ba: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1416935bf: 48 8d 0d ad 30 5f 01        	lea	rcx, [rip + 0x15f30ad]  # 0x142c86673
1416935c6: ba 43 00 00 00              	mov	edx, 0x43
1416935cb: 41 b9 43 00 00 00           	mov	r9d, 0x43
1416935d1: e8 6a 63 24 01              	call	0x1428d9940 <.text+0x28d8940>
1416935d6: 75 db                       	jne	0x1416935b3 <.text+0x16925b3>
1416935d8: 41 bd 41 00 00 00           	mov	r13d, 0x41
1416935de: 49 29 d5                    	sub	r13, rdx
1416935e1: 4c 8d 25 8b 30 5f 01        	lea	r12, [rip + 0x15f308b]  # 0x142c86673
1416935e8: 4d 01 c4                    	add	r12, r8
1416935eb: 4c 89 6d 28                 	mov	qword ptr [rbp + 0x28], r13
1416935ef: 4c 8d 05 f3 31 5f 01        	lea	r8, [rip + 0x15f31f3]   # 0x142c867e9
1416935f6: 4c 8d 2d bb 31 5f 01        	lea	r13, [rip + 0x15f31bb]  # 0x142c867b8
1416935fd: 0f 1f 00                    	nop	dword ptr [rax]
141693600: 41 0f be 40 ff              	movsx	eax, byte ptr [r8 - 0x1]
141693605: 85 c0                       	test	eax, eax
141693607: 78 17                       	js	0x141693620 <.text+0x1692620>
141693609: 49 ff c8                    	dec	r8
14169360c: 83 f8 5c                    	cmp	eax, 0x5c
14169360f: 75 63                       	jne	0x141693674 <.text+0x1692674>
141693611: eb 73                       	jmp	0x141693686 <.text+0x1692686>
141693613: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
141693620: 41 0f b6 48 fe              	movzx	ecx, byte ptr [r8 - 0x2]
141693625: 80 f9 c0                    	cmp	cl, -0x40
141693628: 7d 23                       	jge	0x14169364d <.text+0x169264d>
14169362a: 41 0f b6 50 fd              	movzx	edx, byte ptr [r8 - 0x3]
14169362f: 80 fa c0                    	cmp	dl, -0x40
141693632: 7d 22                       	jge	0x141693656 <.text+0x1692656>
141693634: 45 0f b6 48 fc              	movzx	r9d, byte ptr [r8 - 0x4]
141693639: 49 83 c0 fc                 	add	r8, -0x4
14169363d: 41 83 e1 07                 	and	r9d, 0x7
141693641: 41 c1 e1 06                 	shl	r9d, 0x6
141693645: 83 e2 3f                    	and	edx, 0x3f
141693648: 44 09 ca                    	or	edx, r9d
14169364b: eb 10                       	jmp	0x14169365d <.text+0x169265d>
14169364d: 49 83 c0 fe                 	add	r8, -0x2
141693651: 83 e1 1f                    	and	ecx, 0x1f
141693654: eb 0f                       	jmp	0x141693665 <.text+0x1692665>
141693656: 49 83 c0 fd                 	add	r8, -0x3
14169365a: 83 e2 0f                    	and	edx, 0xf
14169365d: c1 e2 06                    	shl	edx, 0x6
141693660: 83 e1 3f                    	and	ecx, 0x3f
141693663: 09 d1                       	or	ecx, edx
141693665: c1 e1 06                    	shl	ecx, 0x6
141693668: 24 3f                       	and	al, 0x3f
14169366a: 0f b6 c0                    	movzx	eax, al
14169366d: 09 c8                       	or	eax, ecx
14169366f: 83 f8 5c                    	cmp	eax, 0x5c
141693672: 74 12                       	je	0x141693686 <.text+0x1692686>
141693674: 83 f8 2f                    	cmp	eax, 0x2f
141693677: 74 0d                       	je	0x141693686 <.text+0x1692686>
141693679: 4d 39 e8                    	cmp	r8, r13
14169367c: 75 82                       	jne	0x141693600 <.text+0x1692600>
14169367e: 41 bf 31 00 00 00           	mov	r15d, 0x31
141693684: eb 4d                       	jmp	0x1416936d3 <.text+0x16926d3>
141693686: 4c 8d 2d 2b 31 5f 01        	lea	r13, [rip + 0x15f312b]  # 0x142c867b8
14169368d: 4d 29 e8                    	sub	r8, r13
141693690: 49 ff c0                    	inc	r8
141693693: 74 32                       	je	0x1416936c7 <.text+0x16926c7>
141693695: 49 83 f8 31                 	cmp	r8, 0x31
141693699: 73 2a                       	jae	0x1416936c5 <.text+0x16926c5>
14169369b: 43 80 3c 28 bf              	cmp	byte ptr [r8 + r13], -0x41
1416936a0: 7f 25                       	jg	0x1416936c7 <.text+0x16926c7>
1416936a2: 48 8d 05 77 30 5f 01        	lea	rax, [rip + 0x15f3077]  # 0x142c86720
1416936a9: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1416936ae: 48 8d 0d 03 31 5f 01        	lea	rcx, [rip + 0x15f3103]  # 0x142c867b8
1416936b5: ba 31 00 00 00              	mov	edx, 0x31
1416936ba: 41 b9 31 00 00 00           	mov	r9d, 0x31
1416936c0: e8 7b 62 24 01              	call	0x1428d9940 <.text+0x28d8940>
1416936c5: 75 db                       	jne	0x1416936a2 <.text+0x16926a2>
1416936c7: 41 bf 31 00 00 00           	mov	r15d, 0x31
1416936cd: 4d 29 c7                    	sub	r15, r8
1416936d0: 4d 01 c5                    	add	r13, r8
1416936d3: 48 89 b5 80 00 00 00        	mov	qword ptr [rbp + 0x80], rsi
1416936da: 48 8d 05 4f 04 00 00        	lea	rax, [rip + 0x44f]      # 0x141693b30 <.text+0x1692b30>
1416936e1: 48 89 85 88 00 00 00        	mov	qword ptr [rbp + 0x88], rax
1416936e8: 48 8d 05 79 31 5f 01        	lea	rax, [rip + 0x15f3179]  # 0x142c86868
1416936ef: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
1416936f3: 48 c7 45 38 01 00 00 00     	mov	qword ptr [rbp + 0x38], 0x1
1416936fb: 48 8d 85 80 00 00 00        	lea	rax, [rbp + 0x80]
141693702: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
141693706: 48 c7 45 48 01 00 00 00     	mov	qword ptr [rbp + 0x48], 0x1
14169370e: 48 c7 45 50 00 00 00 00     	mov	qword ptr [rbp + 0x50], 0x0
141693716: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
14169371a: 48 8d 55 30                 	lea	rdx, [rbp + 0x30]
14169371e: e8 6d 68 6e ff              	call	0x140d79f90 <.text+0xd78f90>
141693723: e8 08 4a 58 ff              	call	0x140c18130 <.text+0xc17130>
141693728: 48 89 c1                    	mov	rcx, rax
14169372b: 48 8b 00                    	mov	rax, qword ptr [rax]
14169372e: 48 85 c0                    	test	rax, rax
141693731: 74 0f                       	je	0x141693742 <.text+0x1692742>
141693733: 83 f8 01                    	cmp	eax, 0x1
141693736: 0f 85 e8 01 00 00           	jne	0x141693924 <.text+0x1692924>
14169373c: 48 83 c1 08                 	add	rcx, 0x8
141693740: eb 13                       	jmp	0x141693755 <.text+0x1692755>
141693742: 31 d2                       	xor	edx, edx
141693744: e8 87 50 26 01              	call	0x1428f87d0 <.text+0x28f77d0>
141693749: 48 89 c1                    	mov	rcx, rax
14169374c: 48 85 c0                    	test	rax, rax
14169374f: 0f 84 cf 01 00 00           	je	0x141693924 <.text+0x1692924>
141693755: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
141693759: 0f 85 a8 01 00 00           	jne	0x141693907 <.text+0x1692907>
14169375f: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
141693766: 48 89 8d 90 00 00 00        	mov	qword ptr [rbp + 0x90], rcx
14169376d: 48 83 c1 08                 	add	rcx, 0x8
141693771: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
141693775: 0f 11 44 24 28              	movups	xmmword ptr [rsp + 0x28], xmm0
14169377a: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
14169377f: 4c 89 e2                    	mov	rdx, r12
141693782: 4c 8b 45 28                 	mov	r8, qword ptr [rbp + 0x28]
141693786: 4d 89 e9                    	mov	r9, r13
141693789: e8 b2 35 58 ff              	call	0x140c16d40 <.text+0xc15d40>
14169378e: 49 89 c7                    	mov	r15, rax
141693791: 48 8b 85 90 00 00 00        	mov	rax, qword ptr [rbp + 0x90]
141693798: 48 ff 00                    	inc	qword ptr [rax]
14169379b: 48 8b 55 d0                 	mov	rdx, qword ptr [rbp - 0x30]
14169379f: 48 85 d2                    	test	rdx, rdx
1416937a2: 74 0f                       	je	0x1416937b3 <.text+0x16927b3>
1416937a4: 48 8b 4d d8                 	mov	rcx, qword ptr [rbp - 0x28]
1416937a8: 41 b8 01 00 00 00           	mov	r8d, 0x1
1416937ae: e8 6d 04 f8 fe              	call	0x140613c20 <.text+0x612c20>
1416937b3: 4c 89 7d c8                 	mov	qword ptr [rbp - 0x38], r15
1416937b7: b8 01 00 00 00              	mov	eax, 0x1
1416937bc: 49 ff c6                    	inc	r14
1416937bf: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
1416937c3: 49 39 de                    	cmp	r14, rbx
1416937c6: 0f 82 10 01 00 00           	jb	0x1416938dc <.text+0x16928dc>
1416937cc: 48 8b 56 18                 	mov	rdx, qword ptr [rsi + 0x18]
1416937d0: 49 39 d6                    	cmp	r14, rdx
1416937d3: 0f 87 1a 01 00 00           	ja	0x1416938f3 <.text+0x16928f3>
1416937d9: 4c 89 f0                    	mov	rax, r14
1416937dc: 48 29 d8                    	sub	rax, rbx
1416937df: 74 6c                       	je	0x14169384d <.text+0x169284d>
1416937e1: 49 c1 e6 04                 	shl	r14, 0x4
1416937e5: 4c 3b 76 38                 	cmp	r14, qword ptr [rsi + 0x38]
1416937e9: 0f 87 46 01 00 00           	ja	0x141693935 <.text+0x1692935>
1416937ef: 48 c1 e3 04                 	shl	rbx, 0x4
1416937f3: 4c 8b 4e 10                 	mov	r9, qword ptr [rsi + 0x10]
1416937f7: 49 01 d9                    	add	r9, rbx
1416937fa: 4c 8d 46 20                 	lea	r8, [rsi + 0x20]
1416937fe: 48 c1 e0 04                 	shl	rax, 0x4
141693802: 48 c7 85 80 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x80], 0x0
14169380d: 48 89 85 88 00 00 00        	mov	qword ptr [rbp + 0x88], rax
141693814: 48 89 5c 24 38              	mov	qword ptr [rsp + 0x38], rbx
141693819: 48 8d 8d 80 00 00 00        	lea	rcx, [rbp + 0x80]
141693820: 48 89 4c 24 28              	mov	qword ptr [rsp + 0x28], rcx
141693825: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14169382a: 48 c7 44 24 30 01 00 00 00  	mov	qword ptr [rsp + 0x30], 0x1
141693833: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
141693837: 48 89 fa                    	mov	rdx, rdi
14169383a: e8 61 b7 55 ff              	call	0x140beefa0 <.text+0xbedfa0>
14169383f: 48 8b 45 30                 	mov	rax, qword ptr [rbp + 0x30]
141693843: 48 83 f8 0b                 	cmp	rax, 0xb
141693847: 0f 85 ef 00 00 00           	jne	0x14169393c <.text+0x169293c>
14169384d: 48 c7 06 ff ff ff ff        	mov	qword ptr [rsi], -0x1
141693854: 48 c7 46 08 00 00 00 00     	mov	qword ptr [rsi + 0x8], 0x0
14169385c: 80 bd 98 00 00 00 00        	cmp	byte ptr [rbp + 0x98], 0x0
141693863: 74 43                       	je	0x1416938a8 <.text+0x16928a8>
141693865: e8 c6 48 58 ff              	call	0x140c18130 <.text+0xc17130>
14169386a: 48 89 c1                    	mov	rcx, rax
14169386d: 48 8b 00                    	mov	rax, qword ptr [rax]
141693870: 48 83 f8 01                 	cmp	rax, 0x1
141693874: 75 46                       	jne	0x1416938bc <.text+0x16928bc>
141693876: 48 83 c1 08                 	add	rcx, 0x8
14169387a: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
14169387e: 0f 85 94 00 00 00           	jne	0x141693918 <.text+0x1692918>
141693884: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
14169388b: 48 89 8d 98 00 00 00        	mov	qword ptr [rbp + 0x98], rcx
141693892: 48 83 c1 08                 	add	rcx, 0x8
141693896: 4c 89 fa                    	mov	rdx, r15
141693899: e8 52 38 58 ff              	call	0x140c170f0 <.text+0xc160f0>
14169389e: 48 8b 85 98 00 00 00        	mov	rax, qword ptr [rbp + 0x98]
1416938a5: 48 ff 00                    	inc	qword ptr [rax]
1416938a8: 48 81 c4 28 01 00 00        	add	rsp, 0x128
1416938af: 5b                          	pop	rbx
1416938b0: 5f                          	pop	rdi
1416938b1: 5e                          	pop	rsi
1416938b2: 41 5c                       	pop	r12
1416938b4: 41 5d                       	pop	r13
1416938b6: 41 5e                       	pop	r14
1416938b8: 41 5f                       	pop	r15
1416938ba: 5d                          	pop	rbp
1416938bb: c3                          	ret
1416938bc: 48 85 c0                    	test	rax, rax
1416938bf: 75 0f                       	jne	0x1416938d0 <.text+0x16928d0>
1416938c1: 31 d2                       	xor	edx, edx
1416938c3: e8 08 4f 26 01              	call	0x1428f87d0 <.text+0x28f77d0>
1416938c8: 48 89 c1                    	mov	rcx, rax
1416938cb: 48 85 c0                    	test	rax, rax
1416938ce: 75 aa                       	jne	0x14169387a <.text+0x169287a>
1416938d0: 48 8d 0d c9 71 5a 01        	lea	rcx, [rip + 0x15a71c9]  # 0x142c3aaa0
1416938d7: e8 f4 47 24 01              	call	0x1428d80d0 <.text+0x28d70d0>
1416938dc: 4c 8d 05 95 2f 5f 01        	lea	r8, [rip + 0x15f2f95]   # 0x142c86878
1416938e3: 48 89 d9                    	mov	rcx, rbx
1416938e6: 4c 89 f2                    	mov	rdx, r14
1416938e9: e8 62 5f 24 01              	call	0x1428d9850 <.text+0x28d8850>
1416938ee: e9 c5 00 00 00              	jmp	0x1416939b8 <.text+0x16929b8>
1416938f3: 4c 8d 05 7e 2f 5f 01        	lea	r8, [rip + 0x15f2f7e]   # 0x142c86878
1416938fa: 4c 89 f1                    	mov	rcx, r14
1416938fd: e8 3e 5f 24 01              	call	0x1428d9840 <.text+0x28d8840>
141693902: e9 b1 00 00 00              	jmp	0x1416939b8 <.text+0x16929b8>
141693907: 48 8d 0d 62 72 5a 01        	lea	rcx, [rip + 0x15a7262]  # 0x142c3ab70
14169390e: e8 3d 59 24 01              	call	0x1428d9250 <.text+0x28d8250>
141693913: e9 a0 00 00 00              	jmp	0x1416939b8 <.text+0x16929b8>
141693918: 48 8d 0d 51 72 5a 01        	lea	rcx, [rip + 0x15a7251]  # 0x142c3ab70
14169391f: e8 2c 59 24 01              	call	0x1428d9250 <.text+0x28d8250>
141693924: 48 8d 0d 75 71 5a 01        	lea	rcx, [rip + 0x15a7175]  # 0x142c3aaa0
14169392b: e8 a0 47 24 01              	call	0x1428d80d0 <.text+0x28d70d0>
141693930: e9 83 00 00 00              	jmp	0x1416939b8 <.text+0x16929b8>
141693935: b8 0b 00 00 00              	mov	eax, 0xb
14169393a: eb 28                       	jmp	0x141693964 <.text+0x1692964>
14169393c: 48 8b 4d 78                 	mov	rcx, qword ptr [rbp + 0x78]
141693940: 48 89 4d 10                 	mov	qword ptr [rbp + 0x10], rcx
141693944: 0f 10 45 38                 	movups	xmm0, xmmword ptr [rbp + 0x38]
141693948: 0f 10 4d 48                 	movups	xmm1, xmmword ptr [rbp + 0x48]
14169394c: 0f 10 55 58                 	movups	xmm2, xmmword ptr [rbp + 0x58]
141693950: 0f 10 5d 68                 	movups	xmm3, xmmword ptr [rbp + 0x68]
141693954: 0f 29 5d 00                 	movaps	xmmword ptr [rbp], xmm3
141693958: 0f 29 55 f0                 	movaps	xmmword ptr [rbp - 0x10], xmm2
14169395c: 0f 29 4d e0                 	movaps	xmmword ptr [rbp - 0x20], xmm1
141693960: 0f 29 45 d0                 	movaps	xmmword ptr [rbp - 0x30], xmm0
141693964: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
141693968: 0f 28 45 d0                 	movaps	xmm0, xmmword ptr [rbp - 0x30]
14169396c: 0f 28 4d e0                 	movaps	xmm1, xmmword ptr [rbp - 0x20]
141693970: 0f 28 55 f0                 	movaps	xmm2, xmmword ptr [rbp - 0x10]
141693974: 0f 28 5d 00                 	movaps	xmm3, xmmword ptr [rbp]
141693978: 0f 11 45 38                 	movups	xmmword ptr [rbp + 0x38], xmm0
14169397c: 0f 11 4d 48                 	movups	xmmword ptr [rbp + 0x48], xmm1
141693980: 0f 11 55 58                 	movups	xmmword ptr [rbp + 0x58], xmm2
141693984: 0f 11 5d 68                 	movups	xmmword ptr [rbp + 0x68], xmm3
141693988: 48 8b 45 10                 	mov	rax, qword ptr [rbp + 0x10]
14169398c: 48 89 45 78                 	mov	qword ptr [rbp + 0x78], rax
141693990: 48 8d 05 f9 2e 5f 01        	lea	rax, [rip + 0x15f2ef9]  # 0x142c86890
141693997: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14169399c: 48 8d 0d a5 2c 5f 01        	lea	rcx, [rip + 0x15f2ca5]  # 0x142c86648
1416939a3: 4c 8d 0d 7e 2c 5f 01        	lea	r9, [rip + 0x15f2c7e]   # 0x142c86628
1416939aa: 4c 8d 45 30                 	lea	r8, [rbp + 0x30]
1416939ae: ba 2b 00 00 00              	mov	edx, 0x2b
1416939b3: e8 a8 5d 24 01              	call	0x1428d9760 <.text+0x28d8760>
1416939b8: 0f 0b                       	ud2
1416939ba: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1416939c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1416939c5: 55                          	push	rbp
1416939c6: 41 57                       	push	r15
1416939c8: 41 56                       	push	r14
1416939ca: 41 55                       	push	r13
1416939cc: 41 54                       	push	r12
1416939ce: 56                          	push	rsi
1416939cf: 57                          	push	rdi
1416939d0: 53                          	push	rbx
1416939d1: 48 83 ec 48                 	sub	rsp, 0x48
1416939d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1416939dc: 48 8b 85 90 00 00 00        	mov	rax, qword ptr [rbp + 0x90]
1416939e3: 48 ff 00                    	inc	qword ptr [rax]
1416939e6: 48 83 c4 48                 	add	rsp, 0x48
1416939ea: 5b                          	pop	rbx
1416939eb: 5f                          	pop	rdi
1416939ec: 5e                          	pop	rsi
1416939ed: 41 5c                       	pop	r12
1416939ef: 41 5d                       	pop	r13
1416939f1: 41 5e                       	pop	r14
1416939f3: 41 5f                       	pop	r15
1416939f5: 5d                          	pop	rbp
1416939f6: c3                          	ret
1416939f7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
141693a00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141693a05: 55                          	push	rbp
141693a06: 41 57                       	push	r15
141693a08: 41 56                       	push	r14
141693a0a: 41 55                       	push	r13
141693a0c: 41 54                       	push	r12
141693a0e: 56                          	push	rsi
141693a0f: 57                          	push	rdi
141693a10: 53                          	push	rbx
141693a11: 48 83 ec 48                 	sub	rsp, 0x48
141693a15: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141693a1c: 48 8b 55 d0                 	mov	rdx, qword ptr [rbp - 0x30]
141693a20: 48 85 d2                    	test	rdx, rdx
141693a23: 74 0f                       	je	0x141693a34 <.text+0x1692a34>
141693a25: 48 8b 4d d8                 	mov	rcx, qword ptr [rbp - 0x28]
141693a29: 41 b8 01 00 00 00           	mov	r8d, 0x1
141693a2f: e8 ec 01 f8 fe              	call	0x140613c20 <.text+0x612c20>
141693a34: 90                          	nop
141693a35: 48 83 c4 48                 	add	rsp, 0x48
141693a39: 5b                          	pop	rbx
141693a3a: 5f                          	pop	rdi
141693a3b: 5e                          	pop	rsi
141693a3c: 41 5c                       	pop	r12
141693a3e: 41 5d                       	pop	r13
141693a40: 41 5e                       	pop	r14
141693a42: 41 5f                       	pop	r15
141693a44: 5d                          	pop	rbp
141693a45: c3                          	ret
141693a46: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
141693a50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141693a55: 55                          	push	rbp
141693a56: 41 57                       	push	r15
141693a58: 41 56                       	push	r14
141693a5a: 41 55                       	push	r13
141693a5c: 41 54                       	push	r12
141693a5e: 56                          	push	rsi
141693a5f: 57                          	push	rdi
141693a60: 53                          	push	rbx
141693a61: 48 83 ec 48                 	sub	rsp, 0x48
141693a65: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141693a6c: 80 bd 98 00 00 00 00        	cmp	byte ptr [rbp + 0x98], 0x0
141693a73: 74 20                       	je	0x141693a95 <.text+0x1692a95>
141693a75: 48 8d 45 c8                 	lea	rax, [rbp - 0x38]
141693a79: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
141693a7d: 48 8d 45 20                 	lea	rax, [rbp + 0x20]
141693a81: 48 89 45 18                 	mov	qword ptr [rbp + 0x18], rax
141693a85: 48 8d 0d 34 2c 5f 01        	lea	rcx, [rip + 0x15f2c34]  # 0x142c866c0
141693a8c: 48 8d 55 18                 	lea	rdx, [rbp + 0x18]
141693a90: e8 3b b8 c9 ff              	call	0x14132f2d0 <.text+0x132e2d0>
141693a95: 90                          	nop
141693a96: 48 83 c4 48                 	add	rsp, 0x48
141693a9a: 5b                          	pop	rbx
141693a9b: 5f                          	pop	rdi
141693a9c: 5e                          	pop	rsi
141693a9d: 41 5c                       	pop	r12
141693a9f: 41 5d                       	pop	r13
141693aa1: 41 5e                       	pop	r14
141693aa3: 41 5f                       	pop	r15
141693aa5: 5d                          	pop	rbp
141693aa6: c3                          	ret
141693aa7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
141693ab0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141693ab5: 55                          	push	rbp
141693ab6: 41 57                       	push	r15
141693ab8: 41 56                       	push	r14
141693aba: 41 55                       	push	r13
141693abc: 41 54                       	push	r12
141693abe: 56                          	push	rsi
141693abf: 57                          	push	rdi
141693ac0: 53                          	push	rbx
141693ac1: 48 83 ec 48                 	sub	rsp, 0x48
141693ac5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141693acc: 48 8b 85 98 00 00 00        	mov	rax, qword ptr [rbp + 0x98]
141693ad3: 48 ff 00                    	inc	qword ptr [rax]
141693ad6: 48 83 c4 48                 	add	rsp, 0x48
141693ada: 5b                          	pop	rbx
141693adb: 5f                          	pop	rdi
141693adc: 5e                          	pop	rsi
141693add: 41 5c                       	pop	r12
141693adf: 41 5d                       	pop	r13
141693ae1: 41 5e                       	pop	r14
141693ae3: 41 5f                       	pop	r15
141693ae5: 5d                          	pop	rbp
141693ae6: c3                          	ret
141693ae7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
141693af0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141693af5: 55                          	push	rbp
141693af6: 41 57                       	push	r15
141693af8: 41 56                       	push	r14
141693afa: 41 55                       	push	r13
141693afc: 41 54                       	push	r12
141693afe: 56                          	push	rsi
141693aff: 57                          	push	rdi
141693b00: 53                          	push	rbx
141693b01: 48 83 ec 48                 	sub	rsp, 0x48
141693b05: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141693b0c: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
141693b10: e8 fb dc 11 ff              	call	0x1407b1810 <.text+0x7b0810>
141693b15: 90                          	nop
141693b16: 48 83 c4 48                 	add	rsp, 0x48
141693b1a: 5b                          	pop	rbx
141693b1b: 5f                          	pop	rdi
141693b1c: 5e                          	pop	rsi
141693b1d: 41 5c                       	pop	r12
141693b1f: 41 5d                       	pop	r13
141693b21: 41 5e                       	pop	r14
141693b23: 41 5f                       	pop	r15
141693b25: 5d                          	pop	rbp
141693b26: c3                          	ret
