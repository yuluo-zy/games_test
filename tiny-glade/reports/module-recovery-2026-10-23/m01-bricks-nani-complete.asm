
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142076230: 55                          	push	rbp
142076231: 41 57                       	push	r15
142076233: 41 56                       	push	r14
142076235: 41 55                       	push	r13
142076237: 41 54                       	push	r12
142076239: 56                          	push	rsi
14207623a: 57                          	push	rdi
14207623b: 53                          	push	rbx
14207623c: 48 81 ec c8 01 00 00        	sub	rsp, 0x1c8
142076243: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14207624b: 44 0f 29 bd 30 01 00 00     	movaps	xmmword ptr [rbp + 0x130], xmm15
142076253: 44 0f 29 b5 20 01 00 00     	movaps	xmmword ptr [rbp + 0x120], xmm14
14207625b: 44 0f 29 ad 10 01 00 00     	movaps	xmmword ptr [rbp + 0x110], xmm13
142076263: 44 0f 29 a5 00 01 00 00     	movaps	xmmword ptr [rbp + 0x100], xmm12
14207626b: 44 0f 29 9d f0 00 00 00     	movaps	xmmword ptr [rbp + 0xf0], xmm11
142076273: 44 0f 29 95 e0 00 00 00     	movaps	xmmword ptr [rbp + 0xe0], xmm10
14207627b: 44 0f 29 8d d0 00 00 00     	movaps	xmmword ptr [rbp + 0xd0], xmm9
142076283: 44 0f 29 85 c0 00 00 00     	movaps	xmmword ptr [rbp + 0xc0], xmm8
14207628b: 0f 29 bd b0 00 00 00        	movaps	xmmword ptr [rbp + 0xb0], xmm7
142076292: 0f 29 b5 a0 00 00 00        	movaps	xmmword ptr [rbp + 0xa0], xmm6
142076299: 48 c7 85 98 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x98], -0x2
1420762a4: 48 89 d6                    	mov	rsi, rdx
1420762a7: 48 89 cf                    	mov	rdi, rcx
1420762aa: e8 51 07 ba fe              	call	0x140c16a00 <.text+0xc15a00>
1420762af: 84 c0                       	test	al, al
1420762b1: 74 5f                       	je	0x142076312 <.text+0x2075312>
1420762b3: 4c 8d 35 41 cd f5 00        	lea	r14, [rip + 0xf5cd41]   # 0x142fd2ffb
1420762ba: bb 4a 00 00 00              	mov	ebx, 0x4a
1420762bf: ba 4a 00 00 00              	mov	edx, 0x4a
1420762c4: 4c 89 f1                    	mov	rcx, r14
1420762c7: e8 a4 8b ff ff              	call	0x14206ee70 <.text+0x206de70>
1420762cc: 48 83 f8 01                 	cmp	rax, 0x1
1420762d0: 0f 85 b1 00 00 00           	jne	0x142076387 <.text+0x2075387>
1420762d6: 49 89 d1                    	mov	r9, rdx
1420762d9: 48 85 d2                    	test	rdx, rdx
1420762dc: 74 3d                       	je	0x14207631b <.text+0x207531b>
1420762de: 49 83 f9 4a                 	cmp	r9, 0x4a
1420762e2: 73 35                       	jae	0x142076319 <.text+0x2075319>
1420762e4: 48 8d 05 10 cd f5 00        	lea	rax, [rip + 0xf5cd10]   # 0x142fd2ffb
1420762eb: 41 80 3c 01 bf              	cmp	byte ptr [r9 + rax], -0x41
1420762f0: 7f 29                       	jg	0x14207631b <.text+0x207531b>
1420762f2: 48 8d 05 df d0 f5 00        	lea	rax, [rip + 0xf5d0df]   # 0x142fd33d8
1420762f9: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1420762fe: 48 8d 0d f6 cc f5 00        	lea	rcx, [rip + 0xf5ccf6]   # 0x142fd2ffb
142076305: ba 4a 00 00 00              	mov	edx, 0x4a
14207630a: 45 31 c0                    	xor	r8d, r8d
14207630d: e8 2e 36 86 00              	call	0x1428d9940 <.text+0x28d8940>
142076312: 31 c0                       	xor	eax, eax
142076314: e9 c1 01 00 00              	jmp	0x1420764da <.text+0x20754da>
142076319: 75 d7                       	jne	0x1420762f2 <.text+0x20752f2>
14207631b: 4c 8d 35 d9 cc f5 00        	lea	r14, [rip + 0xf5ccd9]   # 0x142fd2ffb
142076322: 4c 89 f1                    	mov	rcx, r14
142076325: 4c 89 ca                    	mov	rdx, r9
142076328: e8 43 8b ff ff              	call	0x14206ee70 <.text+0x206de70>
14207632d: 48 83 f8 01                 	cmp	rax, 0x1
142076331: 75 54                       	jne	0x142076387 <.text+0x2075387>
142076333: 49 89 d0                    	mov	r8, rdx
142076336: 49 83 c0 02                 	add	r8, 0x2
14207633a: 74 39                       	je	0x142076375 <.text+0x2075375>
14207633c: 49 83 f8 4a                 	cmp	r8, 0x4a
142076340: 73 31                       	jae	0x142076373 <.text+0x2075373>
142076342: 48 8d 05 b2 cc f5 00        	lea	rax, [rip + 0xf5ccb2]   # 0x142fd2ffb
142076349: 41 80 3c 00 bf              	cmp	byte ptr [r8 + rax], -0x41
14207634e: 7f 25                       	jg	0x142076375 <.text+0x2075375>
142076350: 48 8d 05 99 d0 f5 00        	lea	rax, [rip + 0xf5d099]   # 0x142fd33f0
142076357: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14207635c: 48 8d 0d 98 cc f5 00        	lea	rcx, [rip + 0xf5cc98]   # 0x142fd2ffb
142076363: ba 4a 00 00 00              	mov	edx, 0x4a
142076368: 41 b9 4a 00 00 00           	mov	r9d, 0x4a
14207636e: e8 cd 35 86 00              	call	0x1428d9940 <.text+0x28d8940>
142076373: 75 db                       	jne	0x142076350 <.text+0x2075350>
142076375: bb 48 00 00 00              	mov	ebx, 0x48
14207637a: 48 29 d3                    	sub	rbx, rdx
14207637d: 4c 8d 35 77 cc f5 00        	lea	r14, [rip + 0xf5cc77]   # 0x142fd2ffb
142076384: 4d 01 c6                    	add	r14, r8
142076387: 4c 8d 05 d0 d0 f5 00        	lea	r8, [rip + 0xf5d0d0]    # 0x142fd345e
14207638e: 4c 8d 3d 9b d0 f5 00        	lea	r15, [rip + 0xf5d09b]   # 0x142fd3430
142076395: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1420763a0: 41 0f be 40 ff              	movsx	eax, byte ptr [r8 - 0x1]
1420763a5: 85 c0                       	test	eax, eax
1420763a7: 78 17                       	js	0x1420763c0 <.text+0x20753c0>
1420763a9: 49 ff c8                    	dec	r8
1420763ac: 83 f8 5c                    	cmp	eax, 0x5c
1420763af: 75 63                       	jne	0x142076414 <.text+0x2075414>
1420763b1: eb 73                       	jmp	0x142076426 <.text+0x2075426>
1420763b3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1420763c0: 41 0f b6 48 fe              	movzx	ecx, byte ptr [r8 - 0x2]
1420763c5: 80 f9 c0                    	cmp	cl, -0x40
1420763c8: 7d 23                       	jge	0x1420763ed <.text+0x20753ed>
1420763ca: 41 0f b6 50 fd              	movzx	edx, byte ptr [r8 - 0x3]
1420763cf: 80 fa c0                    	cmp	dl, -0x40
1420763d2: 7d 22                       	jge	0x1420763f6 <.text+0x20753f6>
1420763d4: 45 0f b6 48 fc              	movzx	r9d, byte ptr [r8 - 0x4]
1420763d9: 49 83 c0 fc                 	add	r8, -0x4
1420763dd: 41 83 e1 07                 	and	r9d, 0x7
1420763e1: 41 c1 e1 06                 	shl	r9d, 0x6
1420763e5: 83 e2 3f                    	and	edx, 0x3f
1420763e8: 44 09 ca                    	or	edx, r9d
1420763eb: eb 10                       	jmp	0x1420763fd <.text+0x20753fd>
1420763ed: 49 83 c0 fe                 	add	r8, -0x2
1420763f1: 83 e1 1f                    	and	ecx, 0x1f
1420763f4: eb 0f                       	jmp	0x142076405 <.text+0x2075405>
1420763f6: 49 83 c0 fd                 	add	r8, -0x3
1420763fa: 83 e2 0f                    	and	edx, 0xf
1420763fd: c1 e2 06                    	shl	edx, 0x6
142076400: 83 e1 3f                    	and	ecx, 0x3f
142076403: 09 d1                       	or	ecx, edx
142076405: c1 e1 06                    	shl	ecx, 0x6
142076408: 24 3f                       	and	al, 0x3f
14207640a: 0f b6 c0                    	movzx	eax, al
14207640d: 09 c8                       	or	eax, ecx
14207640f: 83 f8 5c                    	cmp	eax, 0x5c
142076412: 74 12                       	je	0x142076426 <.text+0x2075426>
142076414: 83 f8 2f                    	cmp	eax, 0x2f
142076417: 74 0d                       	je	0x142076426 <.text+0x2075426>
142076419: 4d 39 f8                    	cmp	r8, r15
14207641c: 75 82                       	jne	0x1420763a0 <.text+0x20753a0>
14207641e: 41 bc 2e 00 00 00           	mov	r12d, 0x2e
142076424: eb 4d                       	jmp	0x142076473 <.text+0x2075473>
142076426: 4c 8d 3d 03 d0 f5 00        	lea	r15, [rip + 0xf5d003]   # 0x142fd3430
14207642d: 4d 29 f8                    	sub	r8, r15
142076430: 49 ff c0                    	inc	r8
142076433: 74 32                       	je	0x142076467 <.text+0x2075467>
142076435: 49 83 f8 2e                 	cmp	r8, 0x2e
142076439: 73 2a                       	jae	0x142076465 <.text+0x2075465>
14207643b: 43 80 3c 38 bf              	cmp	byte ptr [r8 + r15], -0x41
142076440: 7f 25                       	jg	0x142076467 <.text+0x2075467>
142076442: 48 8d 05 6f cf f5 00        	lea	rax, [rip + 0xf5cf6f]   # 0x142fd33b8
142076449: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14207644e: 48 8d 0d db cf f5 00        	lea	rcx, [rip + 0xf5cfdb]   # 0x142fd3430
142076455: ba 2e 00 00 00              	mov	edx, 0x2e
14207645a: 41 b9 2e 00 00 00           	mov	r9d, 0x2e
142076460: e8 db 34 86 00              	call	0x1428d9940 <.text+0x28d8940>
142076465: 75 db                       	jne	0x142076442 <.text+0x2075442>
142076467: 41 bc 2e 00 00 00           	mov	r12d, 0x2e
14207646d: 4d 29 c4                    	sub	r12, r8
142076470: 4d 01 c7                    	add	r15, r8
142076473: e8 b8 1c ba fe              	call	0x140c18130 <.text+0xc17130>
142076478: 48 89 c1                    	mov	rcx, rax
14207647b: 48 8b 00                    	mov	rax, qword ptr [rax]
14207647e: 48 83 f8 01                 	cmp	rax, 0x1
142076482: 0f 85 18 06 00 00           	jne	0x142076aa0 <.text+0x2075aa0>
142076488: 48 83 c1 08                 	add	rcx, 0x8
14207648c: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
142076490: 0f 85 2e 06 00 00           	jne	0x142076ac4 <.text+0x2075ac4>
142076496: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
14207649d: 48 89 4d 60                 	mov	qword ptr [rbp + 0x60], rcx
1420764a1: 48 83 c1 08                 	add	rcx, 0x8
1420764a5: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
1420764aa: 48 c7 44 24 30 00 00 00 00  	mov	qword ptr [rsp + 0x30], 0x0
1420764b3: 48 c7 44 24 28 01 00 00 00  	mov	qword ptr [rsp + 0x28], 0x1
1420764bc: 4c 89 f2                    	mov	rdx, r14
1420764bf: 49 89 d8                    	mov	r8, rbx
1420764c2: 4d 89 f9                    	mov	r9, r15
1420764c5: e8 76 08 ba fe              	call	0x140c16d40 <.text+0xc15d40>
1420764ca: 48 8b 4d 60                 	mov	rcx, qword ptr [rbp + 0x60]
1420764ce: 48 ff 01                    	inc	qword ptr [rcx]
1420764d1: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
1420764d5: b8 01 00 00 00              	mov	eax, 0x1
1420764da: 48 89 45 38                 	mov	qword ptr [rbp + 0x38], rax
1420764de: 48 8b 0e                    	mov	rcx, qword ptr [rsi]
1420764e1: b2 06                       	mov	dl, 0x6
1420764e3: e8 68 b7 91 fe              	call	0x140991c50 <.text+0x990c50>
1420764e8: 48 89 55 30                 	mov	qword ptr [rbp + 0x30], rdx
1420764ec: 48 89 45 50                 	mov	qword ptr [rbp + 0x50], rax
1420764f0: 48 8b 0f                    	mov	rcx, qword ptr [rdi]
1420764f3: 4c 8b 7f 08                 	mov	r15, qword ptr [rdi + 0x8]
1420764f7: 4c 8b a1 f8 00 00 00        	mov	r12, qword ptr [rcx + 0xf8]
1420764fe: 48 8b 81 00 01 00 00        	mov	rax, qword ptr [rcx + 0x100]
142076505: 4d 8d 2c 84                 	lea	r13, [r12 + 4*rax]
142076509: 48 89 4d 58                 	mov	qword ptr [rbp + 0x58], rcx
14207650d: 0f b6 81 08 01 00 00        	movzx	eax, byte ptr [rcx + 0x108]
142076514: 88 85 97 00 00 00           	mov	byte ptr [rbp + 0x97], al
14207651a: b8 08 00 00 00              	mov	eax, 0x8
14207651f: 48 89 45 48                 	mov	qword ptr [rbp + 0x48], rax
142076523: 48 c7 45 78 00 00 00 00     	mov	qword ptr [rbp + 0x78], 0x0
14207652b: 48 c7 85 80 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x80], 0x0
142076536: 48 c7 85 88 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x88], 0x0
142076541: 31 f6                       	xor	esi, esi
142076543: 45 31 f6                    	xor	r14d, r14d
142076546: 80 bd 97 00 00 00 00        	cmp	byte ptr [rbp + 0x97], 0x0
14207654d: 0f 85 4d 01 00 00           	jne	0x1420766a0 <.text+0x20756a0>
142076553: 49 39 f6                    	cmp	r14, rsi
142076556: 0f 85 2f 01 00 00           	jne	0x14207668b <.text+0x207568b>
14207655c: 0f 1f 40 00                 	nop	dword ptr [rax]
142076560: 4d 39 ec                    	cmp	r12, r13
142076563: 0f 84 48 04 00 00           	je	0x1420769b1 <.text+0x20759b1>
142076569: 41 8b 0c 24                 	mov	ecx, dword ptr [r12]
14207656d: 49 83 c4 04                 	add	r12, 0x4
142076571: 49 8b 87 00 01 00 00        	mov	rax, qword ptr [r15 + 0x100]
142076578: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
14207657c: 48 c1 e1 05                 	shl	rcx, 0x5
142076580: 48 8b 74 08 58              	mov	rsi, qword ptr [rax + rcx + 0x58]
142076585: 48 85 f6                    	test	rsi, rsi
142076588: 74 d6                       	je	0x142076560 <.text+0x2075560>
14207658a: 48 01 c8                    	add	rax, rcx
14207658d: 8b 88 94 00 00 00           	mov	ecx, dword ptr [rax + 0x94]
142076593: 49 8b 97 a8 01 00 00        	mov	rdx, qword ptr [r15 + 0x1a8]
14207659a: 4c 8d 0c c9                 	lea	r9, [rcx + 8*rcx]
14207659e: 4a 8d 0c ca                 	lea	rcx, [rdx + 8*r9]
1420765a2: 4c 8b 55 58                 	mov	r10, qword ptr [rbp + 0x58]
1420765a6: 4d 8b 82 10 01 00 00        	mov	r8, qword ptr [r10 + 0x110]
1420765ad: 4a 8b 54 ca 40              	mov	rdx, qword ptr [rdx + 8*r9 + 0x40]
1420765b2: 4c 39 c2                    	cmp	rdx, r8
1420765b5: 76 33                       	jbe	0x1420765ea <.text+0x20755ea>
1420765b7: 4c 8b 49 38                 	mov	r9, qword ptr [rcx + 0x38]
1420765bb: 4f 8b 04 c1                 	mov	r8, qword ptr [r9 + 8*r8]
1420765bf: 4d 85 c0                    	test	r8, r8
1420765c2: 74 26                       	je	0x1420765ea <.text+0x20755ea>
1420765c4: 4c 8b 49 18                 	mov	r9, qword ptr [rcx + 0x18]
1420765c8: 49 f7 d0                    	not	r8
1420765cb: 4f 8d 04 40                 	lea	r8, [r8 + 2*r8]
1420765cf: 49 c1 e0 04                 	shl	r8, 0x4
1420765d3: 4f 8b 44 01 10              	mov	r8, qword ptr [r9 + r8 + 0x10]
1420765d8: 4c 89 45 78                 	mov	qword ptr [rbp + 0x78], r8
1420765dc: 4d 8b 82 18 01 00 00        	mov	r8, qword ptr [r10 + 0x118]
1420765e3: 4c 39 c2                    	cmp	rdx, r8
1420765e6: 77 16                       	ja	0x1420765fe <.text+0x20755fe>
1420765e8: eb 4a                       	jmp	0x142076634 <.text+0x2075634>
1420765ea: 48 c7 45 78 00 00 00 00     	mov	qword ptr [rbp + 0x78], 0x0
1420765f2: 4d 8b 82 18 01 00 00        	mov	r8, qword ptr [r10 + 0x118]
1420765f9: 4c 39 c2                    	cmp	rdx, r8
1420765fc: 76 36                       	jbe	0x142076634 <.text+0x2075634>
1420765fe: 4c 8b 49 38                 	mov	r9, qword ptr [rcx + 0x38]
142076602: 4f 8b 04 c1                 	mov	r8, qword ptr [r9 + 8*r8]
142076606: 4d 85 c0                    	test	r8, r8
142076609: 74 29                       	je	0x142076634 <.text+0x2075634>
14207660b: 4c 8b 49 18                 	mov	r9, qword ptr [rcx + 0x18]
14207660f: 49 f7 d0                    	not	r8
142076612: 4f 8d 04 40                 	lea	r8, [r8 + 2*r8]
142076616: 49 c1 e0 04                 	shl	r8, 0x4
14207661a: 4f 8b 44 01 10              	mov	r8, qword ptr [r9 + r8 + 0x10]
14207661f: 4c 89 85 80 00 00 00        	mov	qword ptr [rbp + 0x80], r8
142076626: 4d 8b 82 20 01 00 00        	mov	r8, qword ptr [r10 + 0x120]
14207662d: 4c 39 c2                    	cmp	rdx, r8
142076630: 76 43                       	jbe	0x142076675 <.text+0x2075675>
142076632: eb 17                       	jmp	0x14207664b <.text+0x207564b>
142076634: 48 c7 85 80 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x80], 0x0
14207663f: 4d 8b 82 20 01 00 00        	mov	r8, qword ptr [r10 + 0x120]
142076646: 4c 39 c2                    	cmp	rdx, r8
142076649: 76 2a                       	jbe	0x142076675 <.text+0x2075675>
14207664b: 48 8b 51 38                 	mov	rdx, qword ptr [rcx + 0x38]
14207664f: 4a 8b 14 c2                 	mov	rdx, qword ptr [rdx + 8*r8]
142076653: 48 85 d2                    	test	rdx, rdx
142076656: 74 1d                       	je	0x142076675 <.text+0x2075675>
142076658: 48 8b 49 18                 	mov	rcx, qword ptr [rcx + 0x18]
14207665c: 48 f7 d2                    	not	rdx
14207665f: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
142076663: 48 c1 e2 04                 	shl	rdx, 0x4
142076667: 48 8b 4c 11 10              	mov	rcx, qword ptr [rcx + rdx + 0x10]
14207666c: 48 89 8d 88 00 00 00        	mov	qword ptr [rbp + 0x88], rcx
142076673: eb 0b                       	jmp	0x142076680 <.text+0x2075680>
142076675: 48 c7 85 88 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x88], 0x0
142076680: 48 8b 40 50                 	mov	rax, qword ptr [rax + 0x50]
142076684: 48 89 45 48                 	mov	qword ptr [rbp + 0x48], rax
142076688: 45 31 f6                    	xor	r14d, r14d
14207668b: 4c 89 f0                    	mov	rax, r14
14207668e: 48 c1 e0 04                 	shl	rax, 0x4
142076692: 48 8b 4d 48                 	mov	rcx, qword ptr [rbp + 0x48]
142076696: 8b 44 01 08                 	mov	eax, dword ptr [rcx + rax + 0x8]
14207669a: e9 1e 01 00 00              	jmp	0x1420767bd <.text+0x20757bd>
14207669f: 90                          	nop
1420766a0: 49 39 f6                    	cmp	r14, rsi
1420766a3: 0f 85 11 01 00 00           	jne	0x1420767ba <.text+0x20757ba>
1420766a9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1420766b0: 4d 39 ec                    	cmp	r12, r13
1420766b3: 0f 84 f8 02 00 00           	je	0x1420769b1 <.text+0x20759b1>
1420766b9: 41 8b 0c 24                 	mov	ecx, dword ptr [r12]
1420766bd: 49 83 c4 04                 	add	r12, 0x4
1420766c1: 49 8b 87 a8 01 00 00        	mov	rax, qword ptr [r15 + 0x1a8]
1420766c8: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1420766cc: 48 8b 74 c8 10              	mov	rsi, qword ptr [rax + 8*rcx + 0x10]
1420766d1: 48 85 f6                    	test	rsi, rsi
1420766d4: 74 da                       	je	0x1420766b0 <.text+0x20756b0>
1420766d6: 48 8d 04 c8                 	lea	rax, [rax + 8*rcx]
1420766da: 4c 8b 4d 58                 	mov	r9, qword ptr [rbp + 0x58]
1420766de: 49 8b 91 10 01 00 00        	mov	rdx, qword ptr [r9 + 0x110]
1420766e5: 48 8b 48 40                 	mov	rcx, qword ptr [rax + 0x40]
1420766e9: 48 39 d1                    	cmp	rcx, rdx
1420766ec: 76 33                       	jbe	0x142076721 <.text+0x2075721>
1420766ee: 4c 8b 40 38                 	mov	r8, qword ptr [rax + 0x38]
1420766f2: 49 8b 14 d0                 	mov	rdx, qword ptr [r8 + 8*rdx]
1420766f6: 48 85 d2                    	test	rdx, rdx
1420766f9: 74 26                       	je	0x142076721 <.text+0x2075721>
1420766fb: 4c 8b 40 18                 	mov	r8, qword ptr [rax + 0x18]
1420766ff: 48 f7 d2                    	not	rdx
142076702: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
142076706: 48 c1 e2 04                 	shl	rdx, 0x4
14207670a: 49 8b 54 10 10              	mov	rdx, qword ptr [r8 + rdx + 0x10]
14207670f: 48 89 55 78                 	mov	qword ptr [rbp + 0x78], rdx
142076713: 49 8b 91 18 01 00 00        	mov	rdx, qword ptr [r9 + 0x118]
14207671a: 48 39 d1                    	cmp	rcx, rdx
14207671d: 77 16                       	ja	0x142076735 <.text+0x2075735>
14207671f: eb 4a                       	jmp	0x14207676b <.text+0x207576b>
142076721: 48 c7 45 78 00 00 00 00     	mov	qword ptr [rbp + 0x78], 0x0
142076729: 49 8b 91 18 01 00 00        	mov	rdx, qword ptr [r9 + 0x118]
142076730: 48 39 d1                    	cmp	rcx, rdx
142076733: 76 36                       	jbe	0x14207676b <.text+0x207576b>
142076735: 4c 8b 40 38                 	mov	r8, qword ptr [rax + 0x38]
142076739: 49 8b 14 d0                 	mov	rdx, qword ptr [r8 + 8*rdx]
14207673d: 48 85 d2                    	test	rdx, rdx
142076740: 74 29                       	je	0x14207676b <.text+0x207576b>
142076742: 4c 8b 40 18                 	mov	r8, qword ptr [rax + 0x18]
142076746: 48 f7 d2                    	not	rdx
142076749: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
14207674d: 48 c1 e2 04                 	shl	rdx, 0x4
142076751: 49 8b 54 10 10              	mov	rdx, qword ptr [r8 + rdx + 0x10]
142076756: 48 89 95 80 00 00 00        	mov	qword ptr [rbp + 0x80], rdx
14207675d: 49 8b 91 20 01 00 00        	mov	rdx, qword ptr [r9 + 0x120]
142076764: 48 39 d1                    	cmp	rcx, rdx
142076767: 76 43                       	jbe	0x1420767ac <.text+0x20757ac>
142076769: eb 17                       	jmp	0x142076782 <.text+0x2075782>
14207676b: 48 c7 85 80 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x80], 0x0
142076776: 49 8b 91 20 01 00 00        	mov	rdx, qword ptr [r9 + 0x120]
14207677d: 48 39 d1                    	cmp	rcx, rdx
142076780: 76 2a                       	jbe	0x1420767ac <.text+0x20757ac>
142076782: 48 8b 48 38                 	mov	rcx, qword ptr [rax + 0x38]
142076786: 48 8b 0c d1                 	mov	rcx, qword ptr [rcx + 8*rdx]
14207678a: 48 85 c9                    	test	rcx, rcx
14207678d: 74 1d                       	je	0x1420767ac <.text+0x20757ac>
14207678f: 48 8b 40 18                 	mov	rax, qword ptr [rax + 0x18]
142076793: 48 f7 d1                    	not	rcx
142076796: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
14207679a: 48 c1 e1 04                 	shl	rcx, 0x4
14207679e: 48 8b 44 08 10              	mov	rax, qword ptr [rax + rcx + 0x10]
1420767a3: 48 89 85 88 00 00 00        	mov	qword ptr [rbp + 0x88], rax
1420767aa: eb 0b                       	jmp	0x1420767b7 <.text+0x20757b7>
1420767ac: 48 c7 85 88 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x88], 0x0
1420767b7: 45 31 f6                    	xor	r14d, r14d
1420767ba: 44 89 f0                    	mov	eax, r14d
1420767bd: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1420767c1: 48 8b 4d 78                 	mov	rcx, qword ptr [rbp + 0x78]
1420767c5: 48 8d 1c 81                 	lea	rbx, [rcx + 4*rax]
1420767c9: 48 8b 8d 88 00 00 00        	mov	rcx, qword ptr [rbp + 0x88]
1420767d0: 48 8d 3c c1                 	lea	rdi, [rcx + 8*rax]
1420767d4: 48 c1 e0 04                 	shl	rax, 0x4
1420767d8: 48 03 85 80 00 00 00        	add	rax, qword ptr [rbp + 0x80]
1420767df: 8b 48 18                    	mov	ecx, dword ptr [rax + 0x18]
1420767e2: 89 4d 28                    	mov	dword ptr [rbp + 0x28], ecx
1420767e5: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
1420767e9: 48 89 4d 20                 	mov	qword ptr [rbp + 0x20], rcx
1420767ed: f3 0f 10 70 1c              	movss	xmm6, dword ptr [rax + 0x1c]
1420767f2: f3 44 0f 10 58 20           	movss	xmm11, dword ptr [rax + 0x20]
1420767f8: f3 44 0f 10 50 24           	movss	xmm10, dword ptr [rax + 0x24]
1420767fe: 0f 10 40 1c                 	movups	xmm0, xmmword ptr [rax + 0x1c]
142076802: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
142076806: 44 0f 10 68 20              	movups	xmm13, xmmword ptr [rax + 0x20]
14207680b: 0f 28 38                    	movaps	xmm7, xmmword ptr [rax]
14207680e: 0f 28 40 10                 	movaps	xmm0, xmmword ptr [rax + 0x10]
142076812: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
142076816: f3 0f 10 47 08              	movss	xmm0, dword ptr [rdi + 0x8]
14207681b: f3 0f 10 4f 0c              	movss	xmm1, dword ptr [rdi + 0xc]
142076820: e8 cb a5 c1 fe              	call	0x140c90df0 <.text+0xc8fdf0>
142076825: 0f 28 cf                    	movaps	xmm1, xmm7
142076828: 66 0f 15 cf                 	unpckhpd	xmm1, xmm7              # xmm1 = xmm1[1],xmm7[1]
14207682c: 0f 28 c1                    	movaps	xmm0, xmm1
14207682f: f3 0f 58 c1                 	addss	xmm0, xmm1
142076833: 0f 28 e7                    	movaps	xmm4, xmm7
142076836: 0f c6 e7 ff                 	shufps	xmm4, xmm7, 0xff        # xmm4 = xmm4[3,3],xmm7[3,3]
14207683a: f3 0f 12 d0                 	movsldup	xmm2, xmm0              # xmm2 = xmm0[0,0,2,2]
14207683e: 0f 59 d7                    	mulps	xmm2, xmm7
142076841: 44 0f 28 f7                 	movaps	xmm14, xmm7
142076845: 44 0f 58 f7                 	addps	xmm14, xmm7
142076849: 0f 28 ec                    	movaps	xmm5, xmm4
14207684c: 41 0f 59 ee                 	mulps	xmm5, xmm14
142076850: 0f c6 ed e1                 	shufps	xmm5, xmm5, 0xe1        # xmm5 = xmm5[1,0,2,3]
142076854: 0f 28 da                    	movaps	xmm3, xmm2
142076857: 0f 58 dd                    	addps	xmm3, xmm5
14207685a: 0f 5c d5                    	subps	xmm2, xmm5
14207685d: 44 0f 28 cf                 	movaps	xmm9, xmm7
142076861: 45 0f 59 ce                 	mulps	xmm9, xmm14
142076865: f3 45 0f 16 c1              	movshdup	xmm8, xmm9              # xmm8 = xmm9[1,1,3,3]
14207686a: 41 0f 28 e9                 	movaps	xmm5, xmm9
14207686e: f3 41 0f 58 e8              	addss	xmm5, xmm8
142076873: f3 44 0f 10 25 88 f0 8a 00  	movss	xmm12, dword ptr [rip + 0x8af088] # 0x142925904
14207687c: 45 0f 28 fc                 	movaps	xmm15, xmm12
142076880: f3 44 0f 5c fd              	subss	xmm15, xmm5
142076885: f3 41 0f 16 ee              	movshdup	xmm5, xmm14             # xmm5 = xmm14[1,1,3,3]
14207688a: f3 0f 59 ef                 	mulss	xmm5, xmm7
14207688e: f3 0f 59 e0                 	mulss	xmm4, xmm0
142076892: f3 0f 59 c1                 	mulss	xmm0, xmm1
142076896: f3 44 0f 58 c8              	addss	xmm9, xmm0
14207689b: f3 41 0f 58 c0              	addss	xmm0, xmm8
1420768a0: 41 0f 28 cc                 	movaps	xmm1, xmm12
1420768a4: f3 0f 5c c8                 	subss	xmm1, xmm0
1420768a8: 0f 28 c5                    	movaps	xmm0, xmm5
1420768ab: f3 0f 5c c4                 	subss	xmm0, xmm4
1420768af: f3 0f 58 e5                 	addss	xmm4, xmm5
1420768b3: 0f 14 cc                    	unpcklps	xmm1, xmm4              # xmm1 = xmm1[0],xmm4[0],xmm1[1],xmm4[1]
1420768b6: 0f 16 ca                    	movlhps	xmm1, xmm2              # xmm1 = xmm1[0],xmm2[0]
1420768b9: 0f c6 d3 01                 	shufps	xmm2, xmm3, 0x1         # xmm2 = xmm2[1,0],xmm3[0,0]
1420768bd: 44 0f c6 fb 30              	shufps	xmm15, xmm3, 0x30       # xmm15 = xmm15[0,0],xmm3[3,0]
1420768c2: 41 0f c6 d7 82              	shufps	xmm2, xmm15, 0x82       # xmm2 = xmm2[2,0],xmm15[0,2]
1420768c7: 41 0f 28 e5                 	movaps	xmm4, xmm13
1420768cb: 41 0f c6 e5 55              	shufps	xmm4, xmm13, 0x55       # xmm4 = xmm4[1,1],xmm13[1,1]
1420768d0: 0f 59 d4                    	mulps	xmm2, xmm4
1420768d3: f3 45 0f 5c e1              	subss	xmm12, xmm9
1420768d8: 41 0f 14 c4                 	unpcklps	xmm0, xmm12             # xmm0 = xmm0[0],xmm12[0],xmm0[1],xmm12[1]
1420768dc: 0f c6 c3 d4                 	shufps	xmm0, xmm3, 0xd4        # xmm0 = xmm0[0,1],xmm3[1,3]
1420768e0: 45 0f c6 ed 00              	shufps	xmm13, xmm13, 0x0       # xmm13 = xmm13[0,0,0,0]
1420768e5: 41 0f 59 c5                 	mulps	xmm0, xmm13
1420768e9: 0f 28 5d 00                 	movaps	xmm3, xmmword ptr [rbp]
1420768ed: 0f c6 db 00                 	shufps	xmm3, xmm3, 0x0         # xmm3 = xmm3[0,0,0,0]
1420768f1: 0f 59 cb                    	mulps	xmm1, xmm3
1420768f4: 0f 28 2d 85 80 8b 00        	movaps	xmm5, xmmword ptr [rip + 0x8b8085] # 0x14292e980
1420768fb: 44 0f 54 dd                 	andps	xmm11, xmm5
1420768ff: 44 0f 54 d5                 	andps	xmm10, xmm5
142076903: 41 0f 28 db                 	movaps	xmm3, xmm11
142076907: f3 41 0f c2 db 03           	cmpunordss	xmm3, xmm11
14207690d: 0f 28 e3                    	movaps	xmm4, xmm3
142076910: 41 0f 54 e2                 	andps	xmm4, xmm10
142076914: f3 45 0f 5f d3              	maxss	xmm10, xmm11
142076919: 0f 54 f5                    	andps	xmm6, xmm5
14207691c: 41 0f 55 da                 	andnps	xmm3, xmm10
142076920: 0f 56 dc                    	orps	xmm3, xmm4
142076923: 0f 28 e3                    	movaps	xmm4, xmm3
142076926: f3 0f 5f e6                 	maxss	xmm4, xmm6
14207692a: f3 0f c2 f6 03              	cmpunordss	xmm6, xmm6
14207692f: 0f 28 ee                    	movaps	xmm5, xmm6
142076932: 0f 54 eb                    	andps	xmm5, xmm3
142076935: 0f 55 f4                    	andnps	xmm6, xmm4
142076938: 0f 56 f5                    	orps	xmm6, xmm5
14207693b: 8b 0f                       	mov	ecx, dword ptr [rdi]
14207693d: 8b 57 14                    	mov	edx, dword ptr [rdi + 0x14]
142076940: c1 e2 1f                    	shl	edx, 0x1f
142076943: 0b 57 04                    	or	edx, dword ptr [rdi + 0x4]
142076946: 0f 28 d9                    	movaps	xmm3, xmm1
142076949: f2 0f 10 d8                 	movsd	xmm3, xmm0              # xmm3 = xmm0[0],xmm3[1]
14207694d: 0f c6 cb 24                 	shufps	xmm1, xmm3, 0x24        # xmm1 = xmm1[0,1],xmm3[2,0]
142076951: f3 0f 10 5f 10              	movss	xmm3, dword ptr [rdi + 0x10]
142076956: 0f 29 4d c0                 	movaps	xmmword ptr [rbp - 0x40], xmm1
14207695a: 0f c6 c2 49                 	shufps	xmm0, xmm2, 0x49        # xmm0 = xmm0[1,2],xmm2[0,1]
14207695e: 0f 29 45 d0                 	movaps	xmmword ptr [rbp - 0x30], xmm0
142076962: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
142076966: 0f c6 d0 02                 	shufps	xmm2, xmm0, 0x2         # xmm2 = xmm2[2,0],xmm0[0,0]
14207696a: 0f c6 d0 98                 	shufps	xmm2, xmm0, 0x98        # xmm2 = xmm2[0,2],xmm0[1,2]
14207696e: 0f 29 55 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm2
142076972: 89 45 f0                    	mov	dword ptr [rbp - 0x10], eax
142076975: 89 4d f4                    	mov	dword ptr [rbp - 0xc], ecx
142076978: 89 55 f8                    	mov	dword ptr [rbp - 0x8], edx
14207697b: f3 0f 11 5d fc              	movss	dword ptr [rbp - 0x4], xmm3
142076980: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
142076984: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142076989: 48 8b 4d 30                 	mov	rcx, qword ptr [rbp + 0x30]
14207698d: 48 89 da                    	mov	rdx, rbx
142076990: 4c 8d 45 20                 	lea	r8, [rbp + 0x20]
142076994: 0f 28 de                    	movaps	xmm3, xmm6
142076997: e8 44 78 4f ff              	call	0x14156e1e0 <.text+0x156d1e0>
14207699c: 49 ff c6                    	inc	r14
14207699f: 80 bd 97 00 00 00 00        	cmp	byte ptr [rbp + 0x97], 0x0
1420769a6: 0f 84 a7 fb ff ff           	je	0x142076553 <.text+0x2075553>
1420769ac: e9 ef fc ff ff              	jmp	0x1420766a0 <.text+0x20756a0>
1420769b1: 48 8b 75 50                 	mov	rsi, qword ptr [rbp + 0x50]
1420769b5: 48 89 f1                    	mov	rcx, rsi
1420769b8: e8 53 b4 c0 fe              	call	0x140c81e10 <.text+0xc80e10>
1420769bd: 48 89 f1                    	mov	rcx, rsi
1420769c0: 48 83 c9 01                 	or	rcx, 0x1
1420769c4: e8 47 b4 c0 fe              	call	0x140c81e10 <.text+0xc80e10>
1420769c9: 31 c9                       	xor	ecx, ecx
1420769cb: b8 08 00 00 00              	mov	eax, 0x8
1420769d0: f0                          	lock
1420769d1: 48 0f b1 0e                 	cmpxchg	qword ptr [rsi], rcx
1420769d5: 0f 85 f5 00 00 00           	jne	0x142076ad0 <.text+0x2075ad0>
1420769db: 48 83 7d 38 00              	cmp	qword ptr [rbp + 0x38], 0x0
1420769e0: 74 42                       	je	0x142076a24 <.text+0x2075a24>
1420769e2: e8 49 17 ba fe              	call	0x140c18130 <.text+0xc17130>
1420769e7: 48 89 c1                    	mov	rcx, rax
1420769ea: 48 8b 00                    	mov	rax, qword ptr [rax]
1420769ed: 48 83 f8 01                 	cmp	rax, 0x1
1420769f1: 0f 85 8f 00 00 00           	jne	0x142076a86 <.text+0x2075a86>
1420769f7: 48 83 c1 08                 	add	rcx, 0x8
1420769fb: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
1420769ff: 0f 85 bf 00 00 00           	jne	0x142076ac4 <.text+0x2075ac4>
142076a05: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
142076a0c: 48 89 4d 60                 	mov	qword ptr [rbp + 0x60], rcx
142076a10: 48 83 c1 08                 	add	rcx, 0x8
142076a14: 48 8b 55 40                 	mov	rdx, qword ptr [rbp + 0x40]
142076a18: e8 d3 06 ba fe              	call	0x140c170f0 <.text+0xc160f0>
142076a1d: 48 8b 45 60                 	mov	rax, qword ptr [rbp + 0x60]
142076a21: 48 ff 00                    	inc	qword ptr [rax]
142076a24: 0f 28 b5 a0 00 00 00        	movaps	xmm6, xmmword ptr [rbp + 0xa0]
142076a2b: 0f 28 bd b0 00 00 00        	movaps	xmm7, xmmword ptr [rbp + 0xb0]
142076a32: 44 0f 28 85 c0 00 00 00     	movaps	xmm8, xmmword ptr [rbp + 0xc0]
142076a3a: 44 0f 28 8d d0 00 00 00     	movaps	xmm9, xmmword ptr [rbp + 0xd0]
142076a42: 44 0f 28 95 e0 00 00 00     	movaps	xmm10, xmmword ptr [rbp + 0xe0]
142076a4a: 44 0f 28 9d f0 00 00 00     	movaps	xmm11, xmmword ptr [rbp + 0xf0]
142076a52: 44 0f 28 a5 00 01 00 00     	movaps	xmm12, xmmword ptr [rbp + 0x100]
142076a5a: 44 0f 28 ad 10 01 00 00     	movaps	xmm13, xmmword ptr [rbp + 0x110]
142076a62: 44 0f 28 b5 20 01 00 00     	movaps	xmm14, xmmword ptr [rbp + 0x120]
142076a6a: 44 0f 28 bd 30 01 00 00     	movaps	xmm15, xmmword ptr [rbp + 0x130]
142076a72: 48 81 c4 c8 01 00 00        	add	rsp, 0x1c8
142076a79: 5b                          	pop	rbx
142076a7a: 5f                          	pop	rdi
142076a7b: 5e                          	pop	rsi
142076a7c: 41 5c                       	pop	r12
142076a7e: 41 5d                       	pop	r13
142076a80: 41 5e                       	pop	r14
142076a82: 41 5f                       	pop	r15
142076a84: 5d                          	pop	rbp
142076a85: c3                          	ret
142076a86: 48 85 c0                    	test	rax, rax
142076a89: 75 2d                       	jne	0x142076ab8 <.text+0x2075ab8>
142076a8b: 31 d2                       	xor	edx, edx
142076a8d: e8 3e 1d 88 00              	call	0x1428f87d0 <.text+0x28f77d0>
142076a92: 48 89 c1                    	mov	rcx, rax
142076a95: 48 85 c0                    	test	rax, rax
142076a98: 0f 85 5d ff ff ff           	jne	0x1420769fb <.text+0x20759fb>
142076a9e: eb 18                       	jmp	0x142076ab8 <.text+0x2075ab8>
142076aa0: 48 85 c0                    	test	rax, rax
142076aa3: 75 13                       	jne	0x142076ab8 <.text+0x2075ab8>
142076aa5: 31 d2                       	xor	edx, edx
142076aa7: e8 24 1d 88 00              	call	0x1428f87d0 <.text+0x28f77d0>
142076aac: 48 89 c1                    	mov	rcx, rax
142076aaf: 48 85 c0                    	test	rax, rax
142076ab2: 0f 85 d4 f9 ff ff           	jne	0x14207648c <.text+0x207548c>
142076ab8: 48 8d 0d e1 3f bc 00        	lea	rcx, [rip + 0xbc3fe1]   # 0x142c3aaa0
142076abf: e8 0c 16 86 00              	call	0x1428d80d0 <.text+0x28d70d0>
142076ac4: 48 8d 0d a5 40 bc 00        	lea	rcx, [rip + 0xbc40a5]   # 0x142c3ab70
142076acb: e8 80 27 86 00              	call	0x1428d9250 <.text+0x28d8250>
142076ad0: 48 89 f1                    	mov	rcx, rsi
142076ad3: 31 d2                       	xor	edx, edx
142076ad5: e8 86 54 85 00              	call	0x1428cbf60 <.text+0x28caf60>
142076ada: e9 fc fe ff ff              	jmp	0x1420769db <.text+0x20759db>
142076adf: 90                          	nop
142076ae0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142076ae5: 55                          	push	rbp
142076ae6: 41 57                       	push	r15
142076ae8: 41 56                       	push	r14
142076aea: 41 55                       	push	r13
142076aec: 41 54                       	push	r12
142076aee: 56                          	push	rsi
142076aef: 57                          	push	rdi
142076af0: 53                          	push	rbx
142076af1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
142076af8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142076aff: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
142076b05: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
142076b0b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
142076b11: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
142076b17: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
142076b1d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
142076b26: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
142076b2f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
142076b38: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
142076b40: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
142076b48: 48 8b 45 60                 	mov	rax, qword ptr [rbp + 0x60]
142076b4c: 48 ff 00                    	inc	qword ptr [rax]
142076b4f: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
142076b57: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
142076b5f: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
142076b68: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
142076b71: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
142076b7a: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
142076b80: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
142076b86: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
142076b8c: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
142076b92: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
142076b98: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
142076b9f: 5b                          	pop	rbx
142076ba0: 5f                          	pop	rdi
142076ba1: 5e                          	pop	rsi
142076ba2: 41 5c                       	pop	r12
142076ba4: 41 5d                       	pop	r13
142076ba6: 41 5e                       	pop	r14
142076ba8: 41 5f                       	pop	r15
142076baa: 5d                          	pop	rbp
142076bab: c3                          	ret
142076bac: 0f 1f 40 00                 	nop	dword ptr [rax]
142076bb0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142076bb5: 55                          	push	rbp
142076bb6: 41 57                       	push	r15
142076bb8: 41 56                       	push	r14
142076bba: 41 55                       	push	r13
142076bbc: 41 54                       	push	r12
142076bbe: 56                          	push	rsi
142076bbf: 57                          	push	rdi
142076bc0: 53                          	push	rbx
142076bc1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
142076bc8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142076bcf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
142076bd5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
142076bdb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
142076be1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
142076be7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
142076bed: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
142076bf6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
142076bff: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
142076c08: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
142076c10: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
142076c18: 48 8b 45 60                 	mov	rax, qword ptr [rbp + 0x60]
142076c1c: 48 ff 00                    	inc	qword ptr [rax]
142076c1f: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
142076c27: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
142076c2f: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
142076c38: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
142076c41: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
142076c4a: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
142076c50: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
142076c56: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
142076c5c: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
142076c62: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
142076c68: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
142076c6f: 5b                          	pop	rbx
142076c70: 5f                          	pop	rdi
142076c71: 5e                          	pop	rsi
142076c72: 41 5c                       	pop	r12
142076c74: 41 5d                       	pop	r13
142076c76: 41 5e                       	pop	r14
142076c78: 41 5f                       	pop	r15
142076c7a: 5d                          	pop	rbp
142076c7b: c3                          	ret
142076c7c: 0f 1f 40 00                 	nop	dword ptr [rax]
142076c80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142076c85: 55                          	push	rbp
142076c86: 41 57                       	push	r15
142076c88: 41 56                       	push	r14
142076c8a: 41 55                       	push	r13
142076c8c: 41 54                       	push	r12
142076c8e: 56                          	push	rsi
142076c8f: 57                          	push	rdi
142076c90: 53                          	push	rbx
142076c91: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
142076c98: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142076c9f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
142076ca5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
142076cab: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
142076cb1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
142076cb7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
142076cbd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
142076cc6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
142076ccf: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
142076cd8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
142076ce0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
142076ce8: 48 8b 4d 50                 	mov	rcx, qword ptr [rbp + 0x50]
142076cec: e8 df a5 fc fd              	call	0x1400412d0 <.text+0x402d0>
142076cf1: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
142076cf9: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
142076d01: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
142076d0a: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
142076d13: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
142076d1c: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
142076d22: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
142076d28: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
142076d2e: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
142076d34: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
142076d3a: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
142076d41: 5b                          	pop	rbx
142076d42: 5f                          	pop	rdi
142076d43: 5e                          	pop	rsi
142076d44: 41 5c                       	pop	r12
142076d46: 41 5d                       	pop	r13
142076d48: 41 5e                       	pop	r14
142076d4a: 41 5f                       	pop	r15
142076d4c: 5d                          	pop	rbp
142076d4d: c3                          	ret
142076d4e: 66 90                       	nop
142076d50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142076d55: 55                          	push	rbp
142076d56: 41 57                       	push	r15
142076d58: 41 56                       	push	r14
142076d5a: 41 55                       	push	r13
142076d5c: 41 54                       	push	r12
142076d5e: 56                          	push	rsi
142076d5f: 57                          	push	rdi
142076d60: 53                          	push	rbx
142076d61: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
142076d68: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142076d6f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
142076d75: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
142076d7b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
142076d81: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
142076d87: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
142076d8d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
142076d96: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
142076d9f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
142076da8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
142076db0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
142076db8: 48 83 7d 38 00              	cmp	qword ptr [rbp + 0x38], 0x0
142076dbd: 74 20                       	je	0x142076ddf <.text+0x2075ddf>
142076dbf: 48 8d 45 40                 	lea	rax, [rbp + 0x40]
142076dc3: 48 89 45 18                 	mov	qword ptr [rbp + 0x18], rax
142076dc7: 48 8d 45 18                 	lea	rax, [rbp + 0x18]
142076dcb: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
142076dcf: 48 8d 0d 82 c5 f5 00        	lea	rcx, [rip + 0xf5c582]   # 0x142fd3358
142076dd6: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
142076dda: e8 f1 84 2b ff              	call	0x14132f2d0 <.text+0x132e2d0>
142076ddf: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
142076de7: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
142076def: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
142076df8: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
142076e01: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
142076e0a: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
142076e10: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
142076e16: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
142076e1c: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
142076e22: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
142076e28: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
142076e2f: 5b                          	pop	rbx
142076e30: 5f                          	pop	rdi
142076e31: 5e                          	pop	rsi
142076e32: 41 5c                       	pop	r12
142076e34: 41 5d                       	pop	r13
142076e36: 41 5e                       	pop	r14
142076e38: 41 5f                       	pop	r15
142076e3a: 5d                          	pop	rbp
142076e3b: c3                          	ret


/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142076e40: 55                          	push	rbp
142076e41: 41 57                       	push	r15
142076e43: 41 56                       	push	r14
142076e45: 41 55                       	push	r13
142076e47: 41 54                       	push	r12
142076e49: 56                          	push	rsi
142076e4a: 57                          	push	rdi
142076e4b: 53                          	push	rbx
142076e4c: 48 81 ec 58 02 00 00        	sub	rsp, 0x258
142076e53: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
142076e5b: 44 0f 29 bd c0 01 00 00     	movaps	xmmword ptr [rbp + 0x1c0], xmm15
142076e63: 44 0f 29 b5 b0 01 00 00     	movaps	xmmword ptr [rbp + 0x1b0], xmm14
142076e6b: 44 0f 29 ad a0 01 00 00     	movaps	xmmword ptr [rbp + 0x1a0], xmm13
142076e73: 44 0f 29 a5 90 01 00 00     	movaps	xmmword ptr [rbp + 0x190], xmm12
142076e7b: 44 0f 29 9d 80 01 00 00     	movaps	xmmword ptr [rbp + 0x180], xmm11
142076e83: 44 0f 29 95 70 01 00 00     	movaps	xmmword ptr [rbp + 0x170], xmm10
142076e8b: 44 0f 29 8d 60 01 00 00     	movaps	xmmword ptr [rbp + 0x160], xmm9
142076e93: 44 0f 29 85 50 01 00 00     	movaps	xmmword ptr [rbp + 0x150], xmm8
142076e9b: 0f 29 bd 40 01 00 00        	movaps	xmmword ptr [rbp + 0x140], xmm7
142076ea2: 0f 29 b5 30 01 00 00        	movaps	xmmword ptr [rbp + 0x130], xmm6
142076ea9: 48 c7 85 28 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x128], -0x2
142076eb4: 48 89 d6                    	mov	rsi, rdx
142076eb7: 48 89 cf                    	mov	rdi, rcx
142076eba: e8 41 fb b9 fe              	call	0x140c16a00 <.text+0xc15a00>
142076ebf: 84 c0                       	test	al, al
142076ec1: 74 5f                       	je	0x142076f22 <.text+0x2075f22>
142076ec3: 4c 8d 35 43 c4 f5 00        	lea	r14, [rip + 0xf5c443]   # 0x142fd330d
142076eca: bb 45 00 00 00              	mov	ebx, 0x45
142076ecf: ba 45 00 00 00              	mov	edx, 0x45
142076ed4: 4c 89 f1                    	mov	rcx, r14
142076ed7: e8 94 7f ff ff              	call	0x14206ee70 <.text+0x206de70>
142076edc: 48 83 f8 01                 	cmp	rax, 0x1
142076ee0: 0f 85 b1 00 00 00           	jne	0x142076f97 <.text+0x2075f97>
142076ee6: 49 89 d1                    	mov	r9, rdx
142076ee9: 48 85 d2                    	test	rdx, rdx
142076eec: 74 3d                       	je	0x142076f2b <.text+0x2075f2b>
142076eee: 49 83 f9 45                 	cmp	r9, 0x45
142076ef2: 73 35                       	jae	0x142076f29 <.text+0x2075f29>
142076ef4: 48 8d 05 12 c4 f5 00        	lea	rax, [rip + 0xf5c412]   # 0x142fd330d
142076efb: 41 80 3c 01 bf              	cmp	byte ptr [r9 + rax], -0x41
142076f00: 7f 29                       	jg	0x142076f2b <.text+0x2075f2b>
142076f02: 48 8d 05 cf c4 f5 00        	lea	rax, [rip + 0xf5c4cf]   # 0x142fd33d8
142076f09: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142076f0e: 48 8d 0d f8 c3 f5 00        	lea	rcx, [rip + 0xf5c3f8]   # 0x142fd330d
142076f15: ba 45 00 00 00              	mov	edx, 0x45
142076f1a: 45 31 c0                    	xor	r8d, r8d
142076f1d: e8 1e 2a 86 00              	call	0x1428d9940 <.text+0x28d8940>
142076f22: 31 c0                       	xor	eax, eax
142076f24: e9 ca 01 00 00              	jmp	0x1420770f3 <.text+0x20760f3>
142076f29: 75 d7                       	jne	0x142076f02 <.text+0x2075f02>
142076f2b: 4c 8d 35 db c3 f5 00        	lea	r14, [rip + 0xf5c3db]   # 0x142fd330d
142076f32: 4c 89 f1                    	mov	rcx, r14
142076f35: 4c 89 ca                    	mov	rdx, r9
142076f38: e8 33 7f ff ff              	call	0x14206ee70 <.text+0x206de70>
142076f3d: 48 83 f8 01                 	cmp	rax, 0x1
142076f41: 75 54                       	jne	0x142076f97 <.text+0x2075f97>
142076f43: 49 89 d0                    	mov	r8, rdx
142076f46: 49 83 c0 02                 	add	r8, 0x2
142076f4a: 74 39                       	je	0x142076f85 <.text+0x2075f85>
142076f4c: 49 83 f8 45                 	cmp	r8, 0x45
142076f50: 73 31                       	jae	0x142076f83 <.text+0x2075f83>
142076f52: 48 8d 05 b4 c3 f5 00        	lea	rax, [rip + 0xf5c3b4]   # 0x142fd330d
142076f59: 41 80 3c 00 bf              	cmp	byte ptr [r8 + rax], -0x41
142076f5e: 7f 25                       	jg	0x142076f85 <.text+0x2075f85>
142076f60: 48 8d 05 89 c4 f5 00        	lea	rax, [rip + 0xf5c489]   # 0x142fd33f0
142076f67: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142076f6c: 48 8d 0d 9a c3 f5 00        	lea	rcx, [rip + 0xf5c39a]   # 0x142fd330d
142076f73: ba 45 00 00 00              	mov	edx, 0x45
142076f78: 41 b9 45 00 00 00           	mov	r9d, 0x45
142076f7e: e8 bd 29 86 00              	call	0x1428d9940 <.text+0x28d8940>
142076f83: 75 db                       	jne	0x142076f60 <.text+0x2075f60>
142076f85: bb 43 00 00 00              	mov	ebx, 0x43
142076f8a: 48 29 d3                    	sub	rbx, rdx
142076f8d: 4c 8d 35 79 c3 f5 00        	lea	r14, [rip + 0xf5c379]   # 0x142fd330d
142076f94: 4d 01 c6                    	add	r14, r8
142076f97: 4c 8d 05 c0 c4 f5 00        	lea	r8, [rip + 0xf5c4c0]    # 0x142fd345e
142076f9e: 4c 8d 3d 8b c4 f5 00        	lea	r15, [rip + 0xf5c48b]   # 0x142fd3430
142076fa5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
142076fb0: 41 0f be 40 ff              	movsx	eax, byte ptr [r8 - 0x1]
142076fb5: 85 c0                       	test	eax, eax
142076fb7: 78 17                       	js	0x142076fd0 <.text+0x2075fd0>
142076fb9: 49 ff c8                    	dec	r8
142076fbc: 83 f8 5c                    	cmp	eax, 0x5c
142076fbf: 75 63                       	jne	0x142077024 <.text+0x2076024>
142076fc1: eb 73                       	jmp	0x142077036 <.text+0x2076036>
142076fc3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
142076fd0: 41 0f b6 48 fe              	movzx	ecx, byte ptr [r8 - 0x2]
142076fd5: 80 f9 c0                    	cmp	cl, -0x40
142076fd8: 7d 23                       	jge	0x142076ffd <.text+0x2075ffd>
142076fda: 41 0f b6 50 fd              	movzx	edx, byte ptr [r8 - 0x3]
142076fdf: 80 fa c0                    	cmp	dl, -0x40
142076fe2: 7d 22                       	jge	0x142077006 <.text+0x2076006>
142076fe4: 45 0f b6 48 fc              	movzx	r9d, byte ptr [r8 - 0x4]
142076fe9: 49 83 c0 fc                 	add	r8, -0x4
142076fed: 41 83 e1 07                 	and	r9d, 0x7
142076ff1: 41 c1 e1 06                 	shl	r9d, 0x6
142076ff5: 83 e2 3f                    	and	edx, 0x3f
142076ff8: 44 09 ca                    	or	edx, r9d
142076ffb: eb 10                       	jmp	0x14207700d <.text+0x207600d>
142076ffd: 49 83 c0 fe                 	add	r8, -0x2
142077001: 83 e1 1f                    	and	ecx, 0x1f
142077004: eb 0f                       	jmp	0x142077015 <.text+0x2076015>
142077006: 49 83 c0 fd                 	add	r8, -0x3
14207700a: 83 e2 0f                    	and	edx, 0xf
14207700d: c1 e2 06                    	shl	edx, 0x6
142077010: 83 e1 3f                    	and	ecx, 0x3f
142077013: 09 d1                       	or	ecx, edx
142077015: c1 e1 06                    	shl	ecx, 0x6
142077018: 24 3f                       	and	al, 0x3f
14207701a: 0f b6 c0                    	movzx	eax, al
14207701d: 09 c8                       	or	eax, ecx
14207701f: 83 f8 5c                    	cmp	eax, 0x5c
142077022: 74 12                       	je	0x142077036 <.text+0x2076036>
142077024: 83 f8 2f                    	cmp	eax, 0x2f
142077027: 74 0d                       	je	0x142077036 <.text+0x2076036>
142077029: 4d 39 f8                    	cmp	r8, r15
14207702c: 75 82                       	jne	0x142076fb0 <.text+0x2075fb0>
14207702e: 41 bc 2e 00 00 00           	mov	r12d, 0x2e
142077034: eb 4d                       	jmp	0x142077083 <.text+0x2076083>
142077036: 4c 8d 3d f3 c3 f5 00        	lea	r15, [rip + 0xf5c3f3]   # 0x142fd3430
14207703d: 4d 29 f8                    	sub	r8, r15
142077040: 49 ff c0                    	inc	r8
142077043: 74 32                       	je	0x142077077 <.text+0x2076077>
142077045: 49 83 f8 2e                 	cmp	r8, 0x2e
142077049: 73 2a                       	jae	0x142077075 <.text+0x2076075>
14207704b: 43 80 3c 38 bf              	cmp	byte ptr [r8 + r15], -0x41
142077050: 7f 25                       	jg	0x142077077 <.text+0x2076077>
142077052: 48 8d 05 5f c3 f5 00        	lea	rax, [rip + 0xf5c35f]   # 0x142fd33b8
142077059: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14207705e: 48 8d 0d cb c3 f5 00        	lea	rcx, [rip + 0xf5c3cb]   # 0x142fd3430
142077065: ba 2e 00 00 00              	mov	edx, 0x2e
14207706a: 41 b9 2e 00 00 00           	mov	r9d, 0x2e
142077070: e8 cb 28 86 00              	call	0x1428d9940 <.text+0x28d8940>
142077075: 75 db                       	jne	0x142077052 <.text+0x2076052>
142077077: 41 bc 2e 00 00 00           	mov	r12d, 0x2e
14207707d: 4d 29 c4                    	sub	r12, r8
142077080: 4d 01 c7                    	add	r15, r8
142077083: e8 a8 10 ba fe              	call	0x140c18130 <.text+0xc17130>
142077088: 48 89 c1                    	mov	rcx, rax
14207708b: 48 8b 00                    	mov	rax, qword ptr [rax]
14207708e: 48 83 f8 01                 	cmp	rax, 0x1
142077092: 0f 85 c2 08 00 00           	jne	0x14207795a <.text+0x207695a>
142077098: 48 83 c1 08                 	add	rcx, 0x8
14207709c: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
1420770a0: 0f 85 d8 08 00 00           	jne	0x14207797e <.text+0x207697e>
1420770a6: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
1420770ad: 48 89 8d 18 01 00 00        	mov	qword ptr [rbp + 0x118], rcx
1420770b4: 48 83 c1 08                 	add	rcx, 0x8
1420770b8: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
1420770bd: 48 c7 44 24 30 00 00 00 00  	mov	qword ptr [rsp + 0x30], 0x0
1420770c6: 48 c7 44 24 28 01 00 00 00  	mov	qword ptr [rsp + 0x28], 0x1
1420770cf: 4c 89 f2                    	mov	rdx, r14
1420770d2: 49 89 d8                    	mov	r8, rbx
1420770d5: 4d 89 f9                    	mov	r9, r15
1420770d8: e8 63 fc b9 fe              	call	0x140c16d40 <.text+0xc15d40>
1420770dd: 48 8b 8d 18 01 00 00        	mov	rcx, qword ptr [rbp + 0x118]
1420770e4: 48 ff 01                    	inc	qword ptr [rcx]
1420770e7: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
1420770ee: b8 01 00 00 00              	mov	eax, 0x1
1420770f3: 48 89 85 b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rax
1420770fa: 48 8b 0e                    	mov	rcx, qword ptr [rsi]
1420770fd: b2 07                       	mov	dl, 0x7
1420770ff: e8 4c ab 91 fe              	call	0x140991c50 <.text+0x990c50>
142077104: 48 89 95 80 00 00 00        	mov	qword ptr [rbp + 0x80], rdx
14207710b: 48 89 85 d8 00 00 00        	mov	qword ptr [rbp + 0xd8], rax
142077112: 48 8b 0f                    	mov	rcx, qword ptr [rdi]
142077115: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
142077119: 48 89 85 18 01 00 00        	mov	qword ptr [rbp + 0x118], rax
142077120: 48 8b b1 f8 00 00 00        	mov	rsi, qword ptr [rcx + 0xf8]
142077127: 48 8b 81 00 01 00 00        	mov	rax, qword ptr [rcx + 0x100]
14207712e: 4c 8d 2c 86                 	lea	r13, [rsi + 4*rax]
142077132: 48 89 8d 10 01 00 00        	mov	qword ptr [rbp + 0x110], rcx
142077139: 0f b6 81 08 01 00 00        	movzx	eax, byte ptr [rcx + 0x108]
142077140: 88 85 27 01 00 00           	mov	byte ptr [rbp + 0x127], al
142077146: b8 08 00 00 00              	mov	eax, 0x8
14207714b: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
142077152: 48 c7 85 f0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xf0], 0x0
14207715d: 48 c7 85 f8 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xf8], 0x0
142077168: 48 c7 85 00 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x100], 0x0
142077173: 48 c7 85 08 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x108], 0x0
14207717e: 31 db                       	xor	ebx, ebx
142077180: 48 c7 85 e8 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xe8], 0x0
14207718b: 80 bd 27 01 00 00 00        	cmp	byte ptr [rbp + 0x127], 0x0
142077192: 0f 85 f8 01 00 00           	jne	0x142077390 <.text+0x2076390>
142077198: 48 8b 85 e8 00 00 00        	mov	rax, qword ptr [rbp + 0xe8]
14207719f: 48 39 d8                    	cmp	rax, rbx
1420771a2: 0f 85 c1 01 00 00           	jne	0x142077369 <.text+0x2076369>
1420771a8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1420771b0: 4c 39 ee                    	cmp	rsi, r13
1420771b3: 0f 84 a3 06 00 00           	je	0x14207785c <.text+0x207685c>
1420771b9: 8b 0e                       	mov	ecx, dword ptr [rsi]
1420771bb: 48 83 c6 04                 	add	rsi, 0x4
1420771bf: 48 8b 85 18 01 00 00        	mov	rax, qword ptr [rbp + 0x118]
1420771c6: 48 8b 80 00 01 00 00        	mov	rax, qword ptr [rax + 0x100]
1420771cd: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
1420771d1: 48 c1 e1 05                 	shl	rcx, 0x5
1420771d5: 48 8b 5c 08 58              	mov	rbx, qword ptr [rax + rcx + 0x58]
1420771da: 48 85 db                    	test	rbx, rbx
1420771dd: 74 d1                       	je	0x1420771b0 <.text+0x20761b0>
1420771df: 48 01 c8                    	add	rax, rcx
1420771e2: 8b 88 94 00 00 00           	mov	ecx, dword ptr [rax + 0x94]
1420771e8: 48 8b 95 18 01 00 00        	mov	rdx, qword ptr [rbp + 0x118]
1420771ef: 48 8b 92 a8 01 00 00        	mov	rdx, qword ptr [rdx + 0x1a8]
1420771f6: 4c 8d 0c c9                 	lea	r9, [rcx + 8*rcx]
1420771fa: 4a 8d 0c ca                 	lea	rcx, [rdx + 8*r9]
1420771fe: 4c 8b 85 10 01 00 00        	mov	r8, qword ptr [rbp + 0x110]
142077205: 4d 8b 80 10 01 00 00        	mov	r8, qword ptr [r8 + 0x110]
14207720c: 4a 8b 54 ca 40              	mov	rdx, qword ptr [rdx + 8*r9 + 0x40]
142077211: 4c 39 c2                    	cmp	rdx, r8
142077214: 76 3d                       	jbe	0x142077253 <.text+0x2076253>
142077216: 4c 8b 49 38                 	mov	r9, qword ptr [rcx + 0x38]
14207721a: 4f 8b 04 c1                 	mov	r8, qword ptr [r9 + 8*r8]
14207721e: 4d 85 c0                    	test	r8, r8
142077221: 74 30                       	je	0x142077253 <.text+0x2076253>
142077223: 4c 8b 49 18                 	mov	r9, qword ptr [rcx + 0x18]
142077227: 49 f7 d0                    	not	r8
14207722a: 4f 8d 04 40                 	lea	r8, [r8 + 2*r8]
14207722e: 49 c1 e0 04                 	shl	r8, 0x4
142077232: 4f 8b 44 01 10              	mov	r8, qword ptr [r9 + r8 + 0x10]
142077237: 4c 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], r8
14207723e: 4c 8b 85 10 01 00 00        	mov	r8, qword ptr [rbp + 0x110]
142077245: 4d 8b 80 18 01 00 00        	mov	r8, qword ptr [r8 + 0x118]
14207724c: 4c 39 c2                    	cmp	rdx, r8
14207724f: 77 20                       	ja	0x142077271 <.text+0x2076271>
142077251: eb 5b                       	jmp	0x1420772ae <.text+0x20762ae>
142077253: 48 c7 85 f0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xf0], 0x0
14207725e: 4c 8b 85 10 01 00 00        	mov	r8, qword ptr [rbp + 0x110]
142077265: 4d 8b 80 18 01 00 00        	mov	r8, qword ptr [r8 + 0x118]
14207726c: 4c 39 c2                    	cmp	rdx, r8
14207726f: 76 3d                       	jbe	0x1420772ae <.text+0x20762ae>
142077271: 4c 8b 49 38                 	mov	r9, qword ptr [rcx + 0x38]
142077275: 4f 8b 04 c1                 	mov	r8, qword ptr [r9 + 8*r8]
142077279: 4d 85 c0                    	test	r8, r8
14207727c: 74 30                       	je	0x1420772ae <.text+0x20762ae>
14207727e: 4c 8b 49 18                 	mov	r9, qword ptr [rcx + 0x18]
142077282: 49 f7 d0                    	not	r8
142077285: 4f 8d 04 40                 	lea	r8, [r8 + 2*r8]
142077289: 49 c1 e0 04                 	shl	r8, 0x4
14207728d: 4f 8b 44 01 10              	mov	r8, qword ptr [r9 + r8 + 0x10]
142077292: 4c 89 85 f8 00 00 00        	mov	qword ptr [rbp + 0xf8], r8
142077299: 4c 8b 85 10 01 00 00        	mov	r8, qword ptr [rbp + 0x110]
1420772a0: 4d 8b 80 20 01 00 00        	mov	r8, qword ptr [r8 + 0x120]
1420772a7: 4c 39 c2                    	cmp	rdx, r8
1420772aa: 76 5d                       	jbe	0x142077309 <.text+0x2076309>
1420772ac: eb 1e                       	jmp	0x1420772cc <.text+0x20762cc>
1420772ae: 48 c7 85 f8 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xf8], 0x0
1420772b9: 4c 8b 85 10 01 00 00        	mov	r8, qword ptr [rbp + 0x110]
1420772c0: 4d 8b 80 20 01 00 00        	mov	r8, qword ptr [r8 + 0x120]
1420772c7: 4c 39 c2                    	cmp	rdx, r8
1420772ca: 76 3d                       	jbe	0x142077309 <.text+0x2076309>
1420772cc: 4c 8b 49 38                 	mov	r9, qword ptr [rcx + 0x38]
1420772d0: 4f 8b 04 c1                 	mov	r8, qword ptr [r9 + 8*r8]
1420772d4: 4d 85 c0                    	test	r8, r8
1420772d7: 74 30                       	je	0x142077309 <.text+0x2076309>
1420772d9: 4c 8b 49 18                 	mov	r9, qword ptr [rcx + 0x18]
1420772dd: 49 f7 d0                    	not	r8
1420772e0: 4f 8d 04 40                 	lea	r8, [r8 + 2*r8]
1420772e4: 49 c1 e0 04                 	shl	r8, 0x4
1420772e8: 4f 8b 44 01 10              	mov	r8, qword ptr [r9 + r8 + 0x10]
1420772ed: 4c 89 85 00 01 00 00        	mov	qword ptr [rbp + 0x100], r8
1420772f4: 4c 8b 85 10 01 00 00        	mov	r8, qword ptr [rbp + 0x110]
1420772fb: 4d 8b 80 28 01 00 00        	mov	r8, qword ptr [r8 + 0x128]
142077302: 4c 39 c2                    	cmp	rdx, r8
142077305: 76 4a                       	jbe	0x142077351 <.text+0x2076351>
142077307: eb 1e                       	jmp	0x142077327 <.text+0x2076327>
142077309: 48 c7 85 00 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x100], 0x0
142077314: 4c 8b 85 10 01 00 00        	mov	r8, qword ptr [rbp + 0x110]
14207731b: 4d 8b 80 28 01 00 00        	mov	r8, qword ptr [r8 + 0x128]
142077322: 4c 39 c2                    	cmp	rdx, r8
142077325: 76 2a                       	jbe	0x142077351 <.text+0x2076351>
142077327: 48 8b 51 38                 	mov	rdx, qword ptr [rcx + 0x38]
14207732b: 4a 8b 14 c2                 	mov	rdx, qword ptr [rdx + 8*r8]
14207732f: 48 85 d2                    	test	rdx, rdx
142077332: 74 1d                       	je	0x142077351 <.text+0x2076351>
142077334: 48 8b 49 18                 	mov	rcx, qword ptr [rcx + 0x18]
142077338: 48 f7 d2                    	not	rdx
14207733b: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
14207733f: 48 c1 e2 04                 	shl	rdx, 0x4
142077343: 48 8b 4c 11 10              	mov	rcx, qword ptr [rcx + rdx + 0x10]
142077348: 48 89 8d 08 01 00 00        	mov	qword ptr [rbp + 0x108], rcx
14207734f: eb 0b                       	jmp	0x14207735c <.text+0x207635c>
142077351: 48 c7 85 08 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x108], 0x0
14207735c: 48 8b 40 50                 	mov	rax, qword ptr [rax + 0x50]
142077360: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
142077367: 31 c0                       	xor	eax, eax
142077369: 48 89 85 e8 00 00 00        	mov	qword ptr [rbp + 0xe8], rax
142077370: 48 c1 e0 04                 	shl	rax, 0x4
142077374: 48 8b 8d d0 00 00 00        	mov	rcx, qword ptr [rbp + 0xd0]
14207737b: 8b 54 01 08                 	mov	edx, dword ptr [rcx + rax + 0x8]
14207737f: e9 b3 01 00 00              	jmp	0x142077537 <.text+0x2076537>
142077384: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
142077390: 48 8b 85 e8 00 00 00        	mov	rax, qword ptr [rbp + 0xe8]
142077397: 48 39 d8                    	cmp	rax, rbx
14207739a: 0f 85 8e 01 00 00           	jne	0x14207752e <.text+0x207652e>
1420773a0: 4c 39 ee                    	cmp	rsi, r13
1420773a3: 0f 84 b3 04 00 00           	je	0x14207785c <.text+0x207685c>
1420773a9: 8b 0e                       	mov	ecx, dword ptr [rsi]
1420773ab: 48 83 c6 04                 	add	rsi, 0x4
1420773af: 48 8b 85 18 01 00 00        	mov	rax, qword ptr [rbp + 0x118]
1420773b6: 48 8b 80 a8 01 00 00        	mov	rax, qword ptr [rax + 0x1a8]
1420773bd: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1420773c1: 48 8b 5c c8 10              	mov	rbx, qword ptr [rax + 8*rcx + 0x10]
1420773c6: 48 85 db                    	test	rbx, rbx
1420773c9: 74 d5                       	je	0x1420773a0 <.text+0x20763a0>
1420773cb: 48 8d 04 c8                 	lea	rax, [rax + 8*rcx]
1420773cf: 48 8b 8d 10 01 00 00        	mov	rcx, qword ptr [rbp + 0x110]
1420773d6: 48 8b 91 10 01 00 00        	mov	rdx, qword ptr [rcx + 0x110]
1420773dd: 48 8b 48 40                 	mov	rcx, qword ptr [rax + 0x40]
1420773e1: 48 39 d1                    	cmp	rcx, rdx
1420773e4: 76 3d                       	jbe	0x142077423 <.text+0x2076423>
1420773e6: 4c 8b 40 38                 	mov	r8, qword ptr [rax + 0x38]
1420773ea: 49 8b 14 d0                 	mov	rdx, qword ptr [r8 + 8*rdx]
1420773ee: 48 85 d2                    	test	rdx, rdx
1420773f1: 74 30                       	je	0x142077423 <.text+0x2076423>
1420773f3: 4c 8b 40 18                 	mov	r8, qword ptr [rax + 0x18]
1420773f7: 48 f7 d2                    	not	rdx
1420773fa: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
1420773fe: 48 c1 e2 04                 	shl	rdx, 0x4
142077402: 49 8b 54 10 10              	mov	rdx, qword ptr [r8 + rdx + 0x10]
142077407: 48 89 95 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rdx
14207740e: 48 8b 95 10 01 00 00        	mov	rdx, qword ptr [rbp + 0x110]
142077415: 48 8b 92 18 01 00 00        	mov	rdx, qword ptr [rdx + 0x118]
14207741c: 48 39 d1                    	cmp	rcx, rdx
14207741f: 77 20                       	ja	0x142077441 <.text+0x2076441>
142077421: eb 5b                       	jmp	0x14207747e <.text+0x207647e>
142077423: 48 c7 85 f0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xf0], 0x0
14207742e: 48 8b 95 10 01 00 00        	mov	rdx, qword ptr [rbp + 0x110]
142077435: 48 8b 92 18 01 00 00        	mov	rdx, qword ptr [rdx + 0x118]
14207743c: 48 39 d1                    	cmp	rcx, rdx
14207743f: 76 3d                       	jbe	0x14207747e <.text+0x207647e>
142077441: 4c 8b 40 38                 	mov	r8, qword ptr [rax + 0x38]
142077445: 49 8b 14 d0                 	mov	rdx, qword ptr [r8 + 8*rdx]
142077449: 48 85 d2                    	test	rdx, rdx
14207744c: 74 30                       	je	0x14207747e <.text+0x207647e>
14207744e: 4c 8b 40 18                 	mov	r8, qword ptr [rax + 0x18]
142077452: 48 f7 d2                    	not	rdx
142077455: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
142077459: 48 c1 e2 04                 	shl	rdx, 0x4
14207745d: 49 8b 54 10 10              	mov	rdx, qword ptr [r8 + rdx + 0x10]
142077462: 48 89 95 f8 00 00 00        	mov	qword ptr [rbp + 0xf8], rdx
142077469: 48 8b 95 10 01 00 00        	mov	rdx, qword ptr [rbp + 0x110]
142077470: 48 8b 92 20 01 00 00        	mov	rdx, qword ptr [rdx + 0x120]
142077477: 48 39 d1                    	cmp	rcx, rdx
14207747a: 76 5d                       	jbe	0x1420774d9 <.text+0x20764d9>
14207747c: eb 1e                       	jmp	0x14207749c <.text+0x207649c>
14207747e: 48 c7 85 f8 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xf8], 0x0
142077489: 48 8b 95 10 01 00 00        	mov	rdx, qword ptr [rbp + 0x110]
142077490: 48 8b 92 20 01 00 00        	mov	rdx, qword ptr [rdx + 0x120]
142077497: 48 39 d1                    	cmp	rcx, rdx
14207749a: 76 3d                       	jbe	0x1420774d9 <.text+0x20764d9>
14207749c: 4c 8b 40 38                 	mov	r8, qword ptr [rax + 0x38]
1420774a0: 49 8b 14 d0                 	mov	rdx, qword ptr [r8 + 8*rdx]
1420774a4: 48 85 d2                    	test	rdx, rdx
1420774a7: 74 30                       	je	0x1420774d9 <.text+0x20764d9>
1420774a9: 4c 8b 40 18                 	mov	r8, qword ptr [rax + 0x18]
1420774ad: 48 f7 d2                    	not	rdx
1420774b0: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
1420774b4: 48 c1 e2 04                 	shl	rdx, 0x4
1420774b8: 49 8b 54 10 10              	mov	rdx, qword ptr [r8 + rdx + 0x10]
1420774bd: 48 89 95 00 01 00 00        	mov	qword ptr [rbp + 0x100], rdx
1420774c4: 48 8b 95 10 01 00 00        	mov	rdx, qword ptr [rbp + 0x110]
1420774cb: 48 8b 92 28 01 00 00        	mov	rdx, qword ptr [rdx + 0x128]
1420774d2: 48 39 d1                    	cmp	rcx, rdx
1420774d5: 76 4a                       	jbe	0x142077521 <.text+0x2076521>
1420774d7: eb 1e                       	jmp	0x1420774f7 <.text+0x20764f7>
1420774d9: 48 c7 85 00 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x100], 0x0
1420774e4: 48 8b 95 10 01 00 00        	mov	rdx, qword ptr [rbp + 0x110]
1420774eb: 48 8b 92 28 01 00 00        	mov	rdx, qword ptr [rdx + 0x128]
1420774f2: 48 39 d1                    	cmp	rcx, rdx
1420774f5: 76 2a                       	jbe	0x142077521 <.text+0x2076521>
1420774f7: 48 8b 48 38                 	mov	rcx, qword ptr [rax + 0x38]
1420774fb: 48 8b 0c d1                 	mov	rcx, qword ptr [rcx + 8*rdx]
1420774ff: 48 85 c9                    	test	rcx, rcx
142077502: 74 1d                       	je	0x142077521 <.text+0x2076521>
142077504: 48 8b 40 18                 	mov	rax, qword ptr [rax + 0x18]
142077508: 48 f7 d1                    	not	rcx
14207750b: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
14207750f: 48 c1 e1 04                 	shl	rcx, 0x4
142077513: 48 8b 44 08 10              	mov	rax, qword ptr [rax + rcx + 0x10]
142077518: 48 89 85 08 01 00 00        	mov	qword ptr [rbp + 0x108], rax
14207751f: eb 0b                       	jmp	0x14207752c <.text+0x207652c>
142077521: 48 c7 85 08 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x108], 0x0
14207752c: 31 c0                       	xor	eax, eax
14207752e: 48 89 85 e8 00 00 00        	mov	qword ptr [rbp + 0xe8], rax
142077535: 89 c2                       	mov	edx, eax
142077537: 48 8d 04 52                 	lea	rax, [rdx + 2*rdx]
14207753b: 48 8b 8d f0 00 00 00        	mov	rcx, qword ptr [rbp + 0xf0]
142077542: 48 8d 0c 81                 	lea	rcx, [rcx + 4*rax]
142077546: 48 89 8d c8 00 00 00        	mov	qword ptr [rbp + 0xc8], rcx
14207754d: 48 8b 8d 00 01 00 00        	mov	rcx, qword ptr [rbp + 0x100]
142077554: 48 8d 0c c1                 	lea	rcx, [rcx + 8*rax]
142077558: 48 c1 e0 04                 	shl	rax, 0x4
14207755c: 48 03 85 f8 00 00 00        	add	rax, qword ptr [rbp + 0xf8]
142077563: 48 6b d2 3c                 	imul	rdx, rdx, 0x3c
142077567: 48 03 95 08 01 00 00        	add	rdx, qword ptr [rbp + 0x108]
14207756e: 44 8b 42 20                 	mov	r8d, dword ptr [rdx + 0x20]
142077572: 44 89 85 b0 00 00 00        	mov	dword ptr [rbp + 0xb0], r8d
142077579: 4c 8b 42 18                 	mov	r8, qword ptr [rdx + 0x18]
14207757d: 4c 89 85 a8 00 00 00        	mov	qword ptr [rbp + 0xa8], r8
142077584: 44 8b 42 2c                 	mov	r8d, dword ptr [rdx + 0x2c]
142077588: 44 89 85 a0 00 00 00        	mov	dword ptr [rbp + 0xa0], r8d
14207758f: 4c 8b 42 24                 	mov	r8, qword ptr [rdx + 0x24]
142077593: 4c 89 85 98 00 00 00        	mov	qword ptr [rbp + 0x98], r8
14207759a: 4c 8b 42 30                 	mov	r8, qword ptr [rdx + 0x30]
14207759e: 4c 89 85 88 00 00 00        	mov	qword ptr [rbp + 0x88], r8
1420775a5: 44 8b 42 38                 	mov	r8d, dword ptr [rdx + 0x38]
1420775a9: 44 89 85 90 00 00 00        	mov	dword ptr [rbp + 0x90], r8d
1420775b0: f3 44 0f 10 1a              	movss	xmm11, dword ptr [rdx]
1420775b5: f3 44 0f 10 62 04           	movss	xmm12, dword ptr [rdx + 0x4]
1420775bb: f3 44 0f 10 4a 08           	movss	xmm9, dword ptr [rdx + 0x8]
1420775c1: f3 44 0f 10 52 0c           	movss	xmm10, dword ptr [rdx + 0xc]
1420775c7: f3 0f 10 42 10              	movss	xmm0, dword ptr [rdx + 0x10]
1420775cc: f3 0f 11 85 e0 00 00 00     	movss	dword ptr [rbp + 0xe0], xmm0
1420775d4: f3 0f 10 42 14              	movss	xmm0, dword ptr [rdx + 0x14]
1420775d9: f3 0f 11 85 e4 00 00 00     	movss	dword ptr [rbp + 0xe4], xmm0
1420775e1: 8b 50 18                    	mov	edx, dword ptr [rax + 0x18]
1420775e4: 89 55 78                    	mov	dword ptr [rbp + 0x78], edx
1420775e7: 48 8b 50 10                 	mov	rdx, qword ptr [rax + 0x10]
1420775eb: 48 89 55 70                 	mov	qword ptr [rbp + 0x70], rdx
1420775ef: f3 0f 10 70 1c              	movss	xmm6, dword ptr [rax + 0x1c]
1420775f4: f3 44 0f 10 70 20           	movss	xmm14, dword ptr [rax + 0x20]
1420775fa: f3 44 0f 10 40 24           	movss	xmm8, dword ptr [rax + 0x24]
142077600: 0f 10 40 1c                 	movups	xmm0, xmmword ptr [rax + 0x1c]
142077604: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
142077608: 44 0f 10 78 20              	movups	xmm15, xmmword ptr [rax + 0x20]
14207760d: 0f 28 38                    	movaps	xmm7, xmmword ptr [rax]
142077610: 0f 28 40 10                 	movaps	xmm0, xmmword ptr [rax + 0x10]
142077614: 0f 29 45 30                 	movaps	xmmword ptr [rbp + 0x30], xmm0
142077618: f3 44 0f 10 68 08           	movss	xmm13, dword ptr [rax + 0x8]
14207761e: f2 0f 10 01                 	movsd	xmm0, qword ptr [rcx]
142077622: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
142077626: 0f b6 79 14                 	movzx	edi, byte ptr [rcx + 0x14]
14207762a: f3 0f 10 41 08              	movss	xmm0, dword ptr [rcx + 0x8]
14207762f: f3 0f 10 49 0c              	movss	xmm1, dword ptr [rcx + 0xc]
142077634: e8 b7 97 c1 fe              	call	0x140c90df0 <.text+0xc8fdf0>
142077639: 41 89 c6                    	mov	r14d, eax
14207763c: 41 0f 28 c3                 	movaps	xmm0, xmm11
142077640: 41 0f 28 cc                 	movaps	xmm1, xmm12
142077644: e8 a7 97 c1 fe              	call	0x140c90df0 <.text+0xc8fdf0>
142077649: 41 89 c7                    	mov	r15d, eax
14207764c: 41 0f 28 c1                 	movaps	xmm0, xmm9
142077650: 41 0f 28 ca                 	movaps	xmm1, xmm10
142077654: e8 97 97 c1 fe              	call	0x140c90df0 <.text+0xc8fdf0>
142077659: 41 89 c4                    	mov	r12d, eax
14207765c: f3 0f 10 85 e0 00 00 00     	movss	xmm0, dword ptr [rbp + 0xe0]
142077664: f3 0f 10 8d e4 00 00 00     	movss	xmm1, dword ptr [rbp + 0xe4]
14207766c: e8 7f 97 c1 fe              	call	0x140c90df0 <.text+0xc8fdf0>
142077671: 0f 28 05 08 73 8b 00        	movaps	xmm0, xmmword ptr [rip + 0x8b7308] # 0x14292e980
142077678: 0f 54 f0                    	andps	xmm6, xmm0
14207767b: 44 0f 54 f0                 	andps	xmm14, xmm0
14207767f: 44 0f 54 c0                 	andps	xmm8, xmm0
142077683: 41 0f 28 c6                 	movaps	xmm0, xmm14
142077687: f3 41 0f c2 c6 03           	cmpunordss	xmm0, xmm14
14207768d: 0f 28 c8                    	movaps	xmm1, xmm0
142077690: 41 0f 54 c8                 	andps	xmm1, xmm8
142077694: f3 45 0f 5f c6              	maxss	xmm8, xmm14
142077699: 41 0f 55 c0                 	andnps	xmm0, xmm8
14207769d: 0f 56 c1                    	orps	xmm0, xmm1
1420776a0: 0f 28 c8                    	movaps	xmm1, xmm0
1420776a3: f3 0f 5f ce                 	maxss	xmm1, xmm6
1420776a7: f3 0f c2 f6 03              	cmpunordss	xmm6, xmm6
1420776ac: 0f 28 d6                    	movaps	xmm2, xmm6
1420776af: 0f 54 d0                    	andps	xmm2, xmm0
1420776b2: 0f 55 f1                    	andnps	xmm6, xmm1
1420776b5: 0f 56 f2                    	orps	xmm6, xmm2
1420776b8: 0f 28 d7                    	movaps	xmm2, xmm7
1420776bb: f3 0f 58 d7                 	addss	xmm2, xmm7
1420776bf: f3 0f 16 df                 	movshdup	xmm3, xmm7              # xmm3 = xmm7[1,1,3,3]
1420776c3: 0f 28 c3                    	movaps	xmm0, xmm3
1420776c6: f3 0f 58 c3                 	addss	xmm0, xmm3
1420776ca: 45 0f 28 cd                 	movaps	xmm9, xmm13
1420776ce: f3 45 0f 58 cd              	addss	xmm9, xmm13
1420776d3: 0f 28 e7                    	movaps	xmm4, xmm7
1420776d6: f3 0f 59 e2                 	mulss	xmm4, xmm2
1420776da: 0f 28 cf                    	movaps	xmm1, xmm7
1420776dd: f3 0f 59 c8                 	mulss	xmm1, xmm0
1420776e1: 44 0f 28 c7                 	movaps	xmm8, xmm7
1420776e5: f3 45 0f 59 c1              	mulss	xmm8, xmm9
1420776ea: 0f 28 eb                    	movaps	xmm5, xmm3
1420776ed: f3 0f 59 e8                 	mulss	xmm5, xmm0
1420776f1: f3 41 0f 59 d9              	mulss	xmm3, xmm9
1420776f6: f3 45 0f 59 e9              	mulss	xmm13, xmm9
1420776fb: 0f c6 ff ff                 	shufps	xmm7, xmm7, 0xff        # xmm7 = xmm7[3,3,3,3]
1420776ff: f3 0f 59 d7                 	mulss	xmm2, xmm7
142077703: f3 0f 59 c7                 	mulss	xmm0, xmm7
142077707: f3 41 0f 59 f9              	mulss	xmm7, xmm9
14207770c: 44 0f 28 d5                 	movaps	xmm10, xmm5
142077710: f3 45 0f 58 d5              	addss	xmm10, xmm13
142077715: f3 44 0f 10 1d e6 e1 8a 00  	movss	xmm11, dword ptr [rip + 0x8ae1e6] # 0x142925904
14207771e: 45 0f 28 cb                 	movaps	xmm9, xmm11
142077722: f3 45 0f 5c ca              	subss	xmm9, xmm10
142077727: 44 0f 28 d1                 	movaps	xmm10, xmm1
14207772b: f3 44 0f 58 d7              	addss	xmm10, xmm7
142077730: 45 0f 14 ca                 	unpcklps	xmm9, xmm10             # xmm9 = xmm9[0],xmm10[0],xmm9[1],xmm10[1]
142077734: 45 0f 28 d0                 	movaps	xmm10, xmm8
142077738: f3 44 0f 5c d0              	subss	xmm10, xmm0
14207773d: 45 0f 16 ca                 	movlhps	xmm9, xmm10             # xmm9 = xmm9[0],xmm10[0]
142077741: f3 0f 5c cf                 	subss	xmm1, xmm7
142077745: f3 44 0f 58 ec              	addss	xmm13, xmm4
14207774a: 41 0f 28 fb                 	movaps	xmm7, xmm11
14207774e: f3 41 0f 5c fd              	subss	xmm7, xmm13
142077753: 0f 14 cf                    	unpcklps	xmm1, xmm7              # xmm1 = xmm1[0],xmm7[0],xmm1[1],xmm7[1]
142077756: 0f 28 fb                    	movaps	xmm7, xmm3
142077759: f3 0f 58 fa                 	addss	xmm7, xmm2
14207775d: 0f 16 cf                    	movlhps	xmm1, xmm7              # xmm1 = xmm1[0],xmm7[0]
142077760: f3 41 0f 58 c0              	addss	xmm0, xmm8
142077765: f3 0f 5c da                 	subss	xmm3, xmm2
142077769: 0f 14 c3                    	unpcklps	xmm0, xmm3              # xmm0 = xmm0[0],xmm3[0],xmm0[1],xmm3[1]
14207776c: f3 0f 58 ec                 	addss	xmm5, xmm4
142077770: 41 0f 28 d3                 	movaps	xmm2, xmm11
142077774: f3 0f 5c d5                 	subss	xmm2, xmm5
142077778: 0f 16 c2                    	movlhps	xmm0, xmm2              # xmm0 = xmm0[0],xmm2[0]
14207777b: 0f 28 5d 50                 	movaps	xmm3, xmmword ptr [rbp + 0x50]
14207777f: 0f c6 db 00                 	shufps	xmm3, xmm3, 0x0         # xmm3 = xmm3[0,0,0,0]
142077783: 41 0f 59 d9                 	mulps	xmm3, xmm9
142077787: 41 0f 28 d7                 	movaps	xmm2, xmm15
14207778b: 41 0f c6 d7 00              	shufps	xmm2, xmm15, 0x0        # xmm2 = xmm2[0,0],xmm15[0,0]
142077790: 0f 59 d1                    	mulps	xmm2, xmm1
142077793: 45 0f c6 ff 55              	shufps	xmm15, xmm15, 0x55      # xmm15 = xmm15[1,1,1,1]
142077798: 44 0f 59 f8                 	mulps	xmm15, xmm0
14207779c: 0f 28 c3                    	movaps	xmm0, xmm3
14207779f: f2 0f 10 c2                 	movsd	xmm0, xmm2              # xmm0 = xmm2[0],xmm0[1]
1420777a3: 0f c6 d8 24                 	shufps	xmm3, xmm0, 0x24        # xmm3 = xmm3[0,1],xmm0[2,0]
1420777a7: 0f 29 5d c0                 	movaps	xmmword ptr [rbp - 0x40], xmm3
1420777ab: 41 0f c6 d7 49              	shufps	xmm2, xmm15, 0x49       # xmm2 = xmm2[1,2],xmm15[0,1]
1420777b0: 0f 29 55 d0                 	movaps	xmmword ptr [rbp - 0x30], xmm2
1420777b4: 0f 28 45 30                 	movaps	xmm0, xmmword ptr [rbp + 0x30]
1420777b8: 44 0f c6 f8 02              	shufps	xmm15, xmm0, 0x2        # xmm15 = xmm15[2,0],xmm0[0,0]
1420777bd: 44 0f c6 f8 98              	shufps	xmm15, xmm0, 0x98       # xmm15 = xmm15[0,2],xmm0[1,2]
1420777c2: 44 0f 29 7d e0              	movaps	xmmword ptr [rbp - 0x20], xmm15
1420777c7: 0f 28 45 40                 	movaps	xmm0, xmmword ptr [rbp + 0x40]
1420777cb: 0f 13 45 f0                 	movlps	qword ptr [rbp - 0x10], xmm0
1420777cf: 44 89 75 f8                 	mov	dword ptr [rbp - 0x8], r14d
1420777d3: 89 7d fc                    	mov	dword ptr [rbp - 0x4], edi
1420777d6: 8b 8d b0 00 00 00           	mov	ecx, dword ptr [rbp + 0xb0]
1420777dc: 48 8d 55 00                 	lea	rdx, [rbp]
1420777e0: 89 4a 08                    	mov	dword ptr [rdx + 0x8], ecx
1420777e3: 48 8b 8d a8 00 00 00        	mov	rcx, qword ptr [rbp + 0xa8]
1420777ea: 48 89 0a                    	mov	qword ptr [rdx], rcx
1420777ed: 44 89 7d 0c                 	mov	dword ptr [rbp + 0xc], r15d
1420777f1: 8b 8d a0 00 00 00           	mov	ecx, dword ptr [rbp + 0xa0]
1420777f7: 89 4a 18                    	mov	dword ptr [rdx + 0x18], ecx
1420777fa: 48 8b 8d 98 00 00 00        	mov	rcx, qword ptr [rbp + 0x98]
142077801: 48 89 4a 10                 	mov	qword ptr [rdx + 0x10], rcx
142077805: 44 89 65 1c                 	mov	dword ptr [rbp + 0x1c], r12d
142077809: 8b 8d 90 00 00 00           	mov	ecx, dword ptr [rbp + 0x90]
14207780f: 89 4a 28                    	mov	dword ptr [rdx + 0x28], ecx
142077812: 48 8b 8d 88 00 00 00        	mov	rcx, qword ptr [rbp + 0x88]
142077819: 48 89 4a 20                 	mov	qword ptr [rdx + 0x20], rcx
14207781d: 89 45 2c                    	mov	dword ptr [rbp + 0x2c], eax
142077820: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
142077824: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142077829: 48 8b 8d 80 00 00 00        	mov	rcx, qword ptr [rbp + 0x80]
142077830: 48 8b 95 c8 00 00 00        	mov	rdx, qword ptr [rbp + 0xc8]
142077837: 4c 8d 45 70                 	lea	r8, [rbp + 0x70]
14207783b: 0f 28 de                    	movaps	xmm3, xmm6
14207783e: e8 8d 73 4f ff              	call	0x14156ebd0 <.text+0x156dbd0>
142077843: 48 ff 85 e8 00 00 00        	inc	qword ptr [rbp + 0xe8]
14207784a: 80 bd 27 01 00 00 00        	cmp	byte ptr [rbp + 0x127], 0x0
142077851: 0f 84 41 f9 ff ff           	je	0x142077198 <.text+0x2076198>
142077857: e9 34 fb ff ff              	jmp	0x142077390 <.text+0x2076390>
14207785c: 48 8b b5 d8 00 00 00        	mov	rsi, qword ptr [rbp + 0xd8]
142077863: 48 89 f1                    	mov	rcx, rsi
142077866: e8 a5 a5 c0 fe              	call	0x140c81e10 <.text+0xc80e10>
14207786b: 48 89 f1                    	mov	rcx, rsi
14207786e: 48 83 c9 01                 	or	rcx, 0x1
142077872: e8 99 a5 c0 fe              	call	0x140c81e10 <.text+0xc80e10>
142077877: 31 c9                       	xor	ecx, ecx
142077879: b8 08 00 00 00              	mov	eax, 0x8
14207787e: f0                          	lock
14207787f: 48 0f b1 0e                 	cmpxchg	qword ptr [rsi], rcx
142077883: 0f 85 01 01 00 00           	jne	0x14207798a <.text+0x207698a>
142077889: 48 83 bd b8 00 00 00 00     	cmp	qword ptr [rbp + 0xb8], 0x0
142077891: 74 4b                       	je	0x1420778de <.text+0x20768de>
142077893: e8 98 08 ba fe              	call	0x140c18130 <.text+0xc17130>
142077898: 48 89 c1                    	mov	rcx, rax
14207789b: 48 8b 00                    	mov	rax, qword ptr [rax]
14207789e: 48 83 f8 01                 	cmp	rax, 0x1
1420778a2: 0f 85 98 00 00 00           	jne	0x142077940 <.text+0x2076940>
1420778a8: 48 83 c1 08                 	add	rcx, 0x8
1420778ac: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
1420778b0: 0f 85 c8 00 00 00           	jne	0x14207797e <.text+0x207697e>
1420778b6: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
1420778bd: 48 89 8d 18 01 00 00        	mov	qword ptr [rbp + 0x118], rcx
1420778c4: 48 83 c1 08                 	add	rcx, 0x8
1420778c8: 48 8b 95 c0 00 00 00        	mov	rdx, qword ptr [rbp + 0xc0]
1420778cf: e8 1c f8 b9 fe              	call	0x140c170f0 <.text+0xc160f0>
1420778d4: 48 8b 85 18 01 00 00        	mov	rax, qword ptr [rbp + 0x118]
1420778db: 48 ff 00                    	inc	qword ptr [rax]
1420778de: 0f 28 b5 30 01 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x130]
1420778e5: 0f 28 bd 40 01 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x140]
1420778ec: 44 0f 28 85 50 01 00 00     	movaps	xmm8, xmmword ptr [rbp + 0x150]
1420778f4: 44 0f 28 8d 60 01 00 00     	movaps	xmm9, xmmword ptr [rbp + 0x160]
1420778fc: 44 0f 28 95 70 01 00 00     	movaps	xmm10, xmmword ptr [rbp + 0x170]
142077904: 44 0f 28 9d 80 01 00 00     	movaps	xmm11, xmmword ptr [rbp + 0x180]
14207790c: 44 0f 28 a5 90 01 00 00     	movaps	xmm12, xmmword ptr [rbp + 0x190]
142077914: 44 0f 28 ad a0 01 00 00     	movaps	xmm13, xmmword ptr [rbp + 0x1a0]
14207791c: 44 0f 28 b5 b0 01 00 00     	movaps	xmm14, xmmword ptr [rbp + 0x1b0]
142077924: 44 0f 28 bd c0 01 00 00     	movaps	xmm15, xmmword ptr [rbp + 0x1c0]
14207792c: 48 81 c4 58 02 00 00        	add	rsp, 0x258
142077933: 5b                          	pop	rbx
142077934: 5f                          	pop	rdi
142077935: 5e                          	pop	rsi
142077936: 41 5c                       	pop	r12
142077938: 41 5d                       	pop	r13
14207793a: 41 5e                       	pop	r14
14207793c: 41 5f                       	pop	r15
14207793e: 5d                          	pop	rbp
14207793f: c3                          	ret
142077940: 48 85 c0                    	test	rax, rax
142077943: 75 2d                       	jne	0x142077972 <.text+0x2076972>
142077945: 31 d2                       	xor	edx, edx
142077947: e8 84 0e 88 00              	call	0x1428f87d0 <.text+0x28f77d0>
14207794c: 48 89 c1                    	mov	rcx, rax
14207794f: 48 85 c0                    	test	rax, rax
142077952: 0f 85 54 ff ff ff           	jne	0x1420778ac <.text+0x20768ac>
142077958: eb 18                       	jmp	0x142077972 <.text+0x2076972>
14207795a: 48 85 c0                    	test	rax, rax
14207795d: 75 13                       	jne	0x142077972 <.text+0x2076972>
14207795f: 31 d2                       	xor	edx, edx
142077961: e8 6a 0e 88 00              	call	0x1428f87d0 <.text+0x28f77d0>
142077966: 48 89 c1                    	mov	rcx, rax
142077969: 48 85 c0                    	test	rax, rax
14207796c: 0f 85 2a f7 ff ff           	jne	0x14207709c <.text+0x207609c>
142077972: 48 8d 0d 27 31 bc 00        	lea	rcx, [rip + 0xbc3127]   # 0x142c3aaa0
142077979: e8 52 07 86 00              	call	0x1428d80d0 <.text+0x28d70d0>
14207797e: 48 8d 0d eb 31 bc 00        	lea	rcx, [rip + 0xbc31eb]   # 0x142c3ab70
142077985: e8 c6 18 86 00              	call	0x1428d9250 <.text+0x28d8250>
14207798a: 48 89 f1                    	mov	rcx, rsi
14207798d: 31 d2                       	xor	edx, edx
14207798f: e8 cc 45 85 00              	call	0x1428cbf60 <.text+0x28caf60>
142077994: e9 f0 fe ff ff              	jmp	0x142077889 <.text+0x2076889>
142077999: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1420779a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1420779a5: 55                          	push	rbp
1420779a6: 41 57                       	push	r15
1420779a8: 41 56                       	push	r14
1420779aa: 41 55                       	push	r13
1420779ac: 41 54                       	push	r12
1420779ae: 56                          	push	rsi
1420779af: 57                          	push	rdi
1420779b0: 53                          	push	rbx
1420779b1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1420779b8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1420779bf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1420779c5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1420779cb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1420779d1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1420779d7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1420779dd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
1420779e6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1420779ef: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1420779f8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
142077a00: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
142077a08: 48 8b 85 18 01 00 00        	mov	rax, qword ptr [rbp + 0x118]
142077a0f: 48 ff 00                    	inc	qword ptr [rax]
142077a12: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
142077a1a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
142077a22: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
142077a2b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
142077a34: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
142077a3d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
142077a43: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
142077a49: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
142077a4f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
142077a55: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
142077a5b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
142077a62: 5b                          	pop	rbx
142077a63: 5f                          	pop	rdi
142077a64: 5e                          	pop	rsi
142077a65: 41 5c                       	pop	r12
142077a67: 41 5d                       	pop	r13
142077a69: 41 5e                       	pop	r14
142077a6b: 41 5f                       	pop	r15
142077a6d: 5d                          	pop	rbp
142077a6e: c3                          	ret
142077a6f: 90                          	nop
142077a70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142077a75: 55                          	push	rbp
142077a76: 41 57                       	push	r15
142077a78: 41 56                       	push	r14
142077a7a: 41 55                       	push	r13
142077a7c: 41 54                       	push	r12
142077a7e: 56                          	push	rsi
142077a7f: 57                          	push	rdi
142077a80: 53                          	push	rbx
142077a81: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
142077a88: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142077a8f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
142077a95: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
142077a9b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
142077aa1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
142077aa7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
142077aad: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
142077ab6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
142077abf: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
142077ac8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
142077ad0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
142077ad8: 48 8b 85 18 01 00 00        	mov	rax, qword ptr [rbp + 0x118]
142077adf: 48 ff 00                    	inc	qword ptr [rax]
142077ae2: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
142077aea: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
142077af2: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
142077afb: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
142077b04: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
142077b0d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
142077b13: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
142077b19: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
142077b1f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
142077b25: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
142077b2b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
142077b32: 5b                          	pop	rbx
142077b33: 5f                          	pop	rdi
142077b34: 5e                          	pop	rsi
142077b35: 41 5c                       	pop	r12
142077b37: 41 5d                       	pop	r13
142077b39: 41 5e                       	pop	r14
142077b3b: 41 5f                       	pop	r15
142077b3d: 5d                          	pop	rbp
142077b3e: c3                          	ret
142077b3f: 90                          	nop
142077b40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142077b45: 55                          	push	rbp
142077b46: 41 57                       	push	r15
142077b48: 41 56                       	push	r14
142077b4a: 41 55                       	push	r13
142077b4c: 41 54                       	push	r12
142077b4e: 56                          	push	rsi
142077b4f: 57                          	push	rdi
142077b50: 53                          	push	rbx
142077b51: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
142077b58: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142077b5f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
142077b65: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
142077b6b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
142077b71: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
142077b77: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
142077b7d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
142077b86: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
142077b8f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
142077b98: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
142077ba0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
142077ba8: 48 8b 8d d8 00 00 00        	mov	rcx, qword ptr [rbp + 0xd8]
142077baf: e8 1c 97 fc fd              	call	0x1400412d0 <.text+0x402d0>
142077bb4: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
142077bbc: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
142077bc4: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
142077bcd: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
142077bd6: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
142077bdf: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
142077be5: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
142077beb: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
142077bf1: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
142077bf7: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
142077bfd: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
142077c04: 5b                          	pop	rbx
142077c05: 5f                          	pop	rdi
142077c06: 5e                          	pop	rsi
142077c07: 41 5c                       	pop	r12
142077c09: 41 5d                       	pop	r13
142077c0b: 41 5e                       	pop	r14
142077c0d: 41 5f                       	pop	r15
142077c0f: 5d                          	pop	rbp
142077c10: c3                          	ret
142077c11: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
142077c20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142077c25: 55                          	push	rbp
142077c26: 41 57                       	push	r15
142077c28: 41 56                       	push	r14
142077c2a: 41 55                       	push	r13
142077c2c: 41 54                       	push	r12
142077c2e: 56                          	push	rsi
142077c2f: 57                          	push	rdi
142077c30: 53                          	push	rbx
142077c31: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
142077c38: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142077c3f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
142077c45: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
142077c4b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
142077c51: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
142077c57: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
142077c5d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
142077c66: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
142077c6f: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
142077c78: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
142077c80: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
142077c88: 48 83 bd b8 00 00 00 00     	cmp	qword ptr [rbp + 0xb8], 0x0
142077c90: 74 23                       	je	0x142077cb5 <.text+0x2076cb5>
142077c92: 48 8d 85 c0 00 00 00        	lea	rax, [rbp + 0xc0]
142077c99: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
142077c9d: 48 8d 45 68                 	lea	rax, [rbp + 0x68]
142077ca1: 48 89 45 60                 	mov	qword ptr [rbp + 0x60], rax
142077ca5: 48 8d 0d ac b6 f5 00        	lea	rcx, [rip + 0xf5b6ac]   # 0x142fd3358
142077cac: 48 8d 55 60                 	lea	rdx, [rbp + 0x60]
142077cb0: e8 1b 76 2b ff              	call	0x14132f2d0 <.text+0x132e2d0>
142077cb5: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
142077cbd: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
142077cc5: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
142077cce: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
142077cd7: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
142077ce0: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
142077ce6: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
142077cec: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
142077cf2: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
142077cf8: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
142077cfe: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
142077d05: 5b                          	pop	rbx
142077d06: 5f                          	pop	rdi
142077d07: 5e                          	pop	rsi
142077d08: 41 5c                       	pop	r12
142077d0a: 41 5d                       	pop	r13
142077d0c: 41 5e                       	pop	r14
142077d0e: 41 5f                       	pop	r15
142077d10: 5d                          	pop	rbp
142077d11: c3                          	ret
