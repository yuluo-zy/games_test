
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408d0490: 55                          	push	rbp
1408d0491: 41 57                       	push	r15
1408d0493: 41 56                       	push	r14
1408d0495: 41 55                       	push	r13
1408d0497: 41 54                       	push	r12
1408d0499: 56                          	push	rsi
1408d049a: 57                          	push	rdi
1408d049b: 53                          	push	rbx
1408d049c: 48 81 ec a8 04 00 00        	sub	rsp, 0x4a8
1408d04a3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1408d04ab: 44 0f 29 bd 10 04 00 00     	movaps	xmmword ptr [rbp + 0x410], xmm15
1408d04b3: 44 0f 29 b5 00 04 00 00     	movaps	xmmword ptr [rbp + 0x400], xmm14
1408d04bb: 44 0f 29 ad f0 03 00 00     	movaps	xmmword ptr [rbp + 0x3f0], xmm13
1408d04c3: 44 0f 29 a5 e0 03 00 00     	movaps	xmmword ptr [rbp + 0x3e0], xmm12
1408d04cb: 66 44 0f 7f 9d d0 03 00 00  	movdqa	xmmword ptr [rbp + 0x3d0], xmm11
1408d04d4: 66 44 0f 7f 95 c0 03 00 00  	movdqa	xmmword ptr [rbp + 0x3c0], xmm10
1408d04dd: 44 0f 29 8d b0 03 00 00     	movaps	xmmword ptr [rbp + 0x3b0], xmm9
1408d04e5: 44 0f 29 85 a0 03 00 00     	movaps	xmmword ptr [rbp + 0x3a0], xmm8
1408d04ed: 0f 29 bd 90 03 00 00        	movaps	xmmword ptr [rbp + 0x390], xmm7
1408d04f4: 0f 29 b5 80 03 00 00        	movaps	xmmword ptr [rbp + 0x380], xmm6
1408d04fb: 48 c7 85 78 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x378], -0x2
1408d0506: 4d 89 cd                    	mov	r13, r9
1408d0509: 4d 89 c6                    	mov	r14, r8
1408d050c: 48 89 95 e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], rdx
1408d0513: 48 89 ce                    	mov	rsi, rcx
1408d0516: e8 e5 64 34 00              	call	0x140c16a00 <.text+0xc15a00>
1408d051b: 84 c0                       	test	al, al
1408d051d: 4c 89 b5 b0 00 00 00        	mov	qword ptr [rbp + 0xb0], r14
1408d0524: 4c 89 ad e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], r13
1408d052b: 74 5f                       	je	0x1408d058c <.text+0x8cf58c>
1408d052d: 4c 8d 3d 1e 31 1d 02        	lea	r15, [rip + 0x21d311e]  # 0x142aa3652
1408d0534: bf 40 00 00 00              	mov	edi, 0x40
1408d0539: ba 40 00 00 00              	mov	edx, 0x40
1408d053e: 4c 89 f9                    	mov	rcx, r15
1408d0541: e8 da a6 ff ff              	call	0x1408cac20 <.text+0x8c9c20>
1408d0546: 48 83 f8 01                 	cmp	rax, 0x1
1408d054a: 0f 85 b1 00 00 00           	jne	0x1408d0601 <.text+0x8cf601>
1408d0550: 49 89 d1                    	mov	r9, rdx
1408d0553: 48 85 d2                    	test	rdx, rdx
1408d0556: 74 3d                       	je	0x1408d0595 <.text+0x8cf595>
1408d0558: 49 83 f9 40                 	cmp	r9, 0x40
1408d055c: 73 35                       	jae	0x1408d0593 <.text+0x8cf593>
1408d055e: 48 8d 05 ed 30 1d 02        	lea	rax, [rip + 0x21d30ed]  # 0x142aa3652
1408d0565: 41 80 3c 01 bf              	cmp	byte ptr [r9 + rax], -0x41
1408d056a: 7f 29                       	jg	0x1408d0595 <.text+0x8cf595>
1408d056c: 48 8d 05 ed 31 1d 02        	lea	rax, [rip + 0x21d31ed]  # 0x142aa3760
1408d0573: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408d0578: 48 8d 0d d3 30 1d 02        	lea	rcx, [rip + 0x21d30d3]  # 0x142aa3652
1408d057f: ba 40 00 00 00              	mov	edx, 0x40
1408d0584: 45 31 c0                    	xor	r8d, r8d
1408d0587: e8 b4 93 00 02              	call	0x1428d9940 <.text+0x28d8940>
1408d058c: 31 c0                       	xor	eax, eax
1408d058e: e9 bb 01 00 00              	jmp	0x1408d074e <.text+0x8cf74e>
1408d0593: 75 d7                       	jne	0x1408d056c <.text+0x8cf56c>
1408d0595: 4c 8d 3d b6 30 1d 02        	lea	r15, [rip + 0x21d30b6]  # 0x142aa3652
1408d059c: 4c 89 f9                    	mov	rcx, r15
1408d059f: 4c 89 ca                    	mov	rdx, r9
1408d05a2: e8 79 a6 ff ff              	call	0x1408cac20 <.text+0x8c9c20>
1408d05a7: 48 83 f8 01                 	cmp	rax, 0x1
1408d05ab: 75 54                       	jne	0x1408d0601 <.text+0x8cf601>
1408d05ad: 49 89 d0                    	mov	r8, rdx
1408d05b0: 49 83 c0 02                 	add	r8, 0x2
1408d05b4: 74 39                       	je	0x1408d05ef <.text+0x8cf5ef>
1408d05b6: 49 83 f8 40                 	cmp	r8, 0x40
1408d05ba: 73 31                       	jae	0x1408d05ed <.text+0x8cf5ed>
1408d05bc: 48 8d 05 8f 30 1d 02        	lea	rax, [rip + 0x21d308f]  # 0x142aa3652
1408d05c3: 41 80 3c 00 bf              	cmp	byte ptr [r8 + rax], -0x41
1408d05c8: 7f 25                       	jg	0x1408d05ef <.text+0x8cf5ef>
1408d05ca: 48 8d 05 a7 31 1d 02        	lea	rax, [rip + 0x21d31a7]  # 0x142aa3778
1408d05d1: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408d05d6: 48 8d 0d 75 30 1d 02        	lea	rcx, [rip + 0x21d3075]  # 0x142aa3652
1408d05dd: ba 40 00 00 00              	mov	edx, 0x40
1408d05e2: 41 b9 40 00 00 00           	mov	r9d, 0x40
1408d05e8: e8 53 93 00 02              	call	0x1428d9940 <.text+0x28d8940>
1408d05ed: 75 db                       	jne	0x1408d05ca <.text+0x8cf5ca>
1408d05ef: bf 3e 00 00 00              	mov	edi, 0x3e
1408d05f4: 48 29 d7                    	sub	rdi, rdx
1408d05f7: 4c 8d 3d 54 30 1d 02        	lea	r15, [rip + 0x21d3054]  # 0x142aa3652
1408d05fe: 4d 01 c7                    	add	r15, r8
1408d0601: 4c 8d 05 e2 32 1d 02        	lea	r8, [rip + 0x21d32e2]   # 0x142aa38ea
1408d0608: 4c 8d 25 a9 32 1d 02        	lea	r12, [rip + 0x21d32a9]  # 0x142aa38b8
1408d060f: 90                          	nop
1408d0610: 41 0f be 40 ff              	movsx	eax, byte ptr [r8 - 0x1]
1408d0615: 85 c0                       	test	eax, eax
1408d0617: 78 17                       	js	0x1408d0630 <.text+0x8cf630>
1408d0619: 49 ff c8                    	dec	r8
1408d061c: 83 f8 5c                    	cmp	eax, 0x5c
1408d061f: 75 63                       	jne	0x1408d0684 <.text+0x8cf684>
1408d0621: eb 72                       	jmp	0x1408d0695 <.text+0x8cf695>
1408d0623: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1408d0630: 41 0f b6 48 fe              	movzx	ecx, byte ptr [r8 - 0x2]
1408d0635: 80 f9 c0                    	cmp	cl, -0x40
1408d0638: 7d 23                       	jge	0x1408d065d <.text+0x8cf65d>
1408d063a: 41 0f b6 50 fd              	movzx	edx, byte ptr [r8 - 0x3]
1408d063f: 80 fa c0                    	cmp	dl, -0x40
1408d0642: 7d 22                       	jge	0x1408d0666 <.text+0x8cf666>
1408d0644: 45 0f b6 48 fc              	movzx	r9d, byte ptr [r8 - 0x4]
1408d0649: 49 83 c0 fc                 	add	r8, -0x4
1408d064d: 41 83 e1 07                 	and	r9d, 0x7
1408d0651: 41 c1 e1 06                 	shl	r9d, 0x6
1408d0655: 83 e2 3f                    	and	edx, 0x3f
1408d0658: 44 09 ca                    	or	edx, r9d
1408d065b: eb 10                       	jmp	0x1408d066d <.text+0x8cf66d>
1408d065d: 49 83 c0 fe                 	add	r8, -0x2
1408d0661: 83 e1 1f                    	and	ecx, 0x1f
1408d0664: eb 0f                       	jmp	0x1408d0675 <.text+0x8cf675>
1408d0666: 49 83 c0 fd                 	add	r8, -0x3
1408d066a: 83 e2 0f                    	and	edx, 0xf
1408d066d: c1 e2 06                    	shl	edx, 0x6
1408d0670: 83 e1 3f                    	and	ecx, 0x3f
1408d0673: 09 d1                       	or	ecx, edx
1408d0675: c1 e1 06                    	shl	ecx, 0x6
1408d0678: 24 3f                       	and	al, 0x3f
1408d067a: 0f b6 c0                    	movzx	eax, al
1408d067d: 09 c8                       	or	eax, ecx
1408d067f: 83 f8 5c                    	cmp	eax, 0x5c
1408d0682: 74 11                       	je	0x1408d0695 <.text+0x8cf695>
1408d0684: 83 f8 2f                    	cmp	eax, 0x2f
1408d0687: 74 0c                       	je	0x1408d0695 <.text+0x8cf695>
1408d0689: 4d 39 e0                    	cmp	r8, r12
1408d068c: 75 82                       	jne	0x1408d0610 <.text+0x8cf610>
1408d068e: bb 32 00 00 00              	mov	ebx, 0x32
1408d0693: eb 4c                       	jmp	0x1408d06e1 <.text+0x8cf6e1>
1408d0695: 4c 8d 25 1c 32 1d 02        	lea	r12, [rip + 0x21d321c]  # 0x142aa38b8
1408d069c: 4d 29 e0                    	sub	r8, r12
1408d069f: 49 ff c0                    	inc	r8
1408d06a2: 74 32                       	je	0x1408d06d6 <.text+0x8cf6d6>
1408d06a4: 49 83 f8 32                 	cmp	r8, 0x32
1408d06a8: 73 2a                       	jae	0x1408d06d4 <.text+0x8cf6d4>
1408d06aa: 43 80 3c 20 bf              	cmp	byte ptr [r8 + r12], -0x41
1408d06af: 7f 25                       	jg	0x1408d06d6 <.text+0x8cf6d6>
1408d06b1: 48 8d 05 88 30 1d 02        	lea	rax, [rip + 0x21d3088]  # 0x142aa3740
1408d06b8: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408d06bd: 48 8d 0d f4 31 1d 02        	lea	rcx, [rip + 0x21d31f4]  # 0x142aa38b8
1408d06c4: ba 32 00 00 00              	mov	edx, 0x32
1408d06c9: 41 b9 32 00 00 00           	mov	r9d, 0x32
1408d06cf: e8 6c 92 00 02              	call	0x1428d9940 <.text+0x28d8940>
1408d06d4: 75 db                       	jne	0x1408d06b1 <.text+0x8cf6b1>
1408d06d6: bb 32 00 00 00              	mov	ebx, 0x32
1408d06db: 4c 29 c3                    	sub	rbx, r8
1408d06de: 4d 01 c4                    	add	r12, r8
1408d06e1: e8 4a 7a 34 00              	call	0x140c18130 <.text+0xc17130>
1408d06e6: 48 89 c1                    	mov	rcx, rax
1408d06e9: 48 8b 00                    	mov	rax, qword ptr [rax]
1408d06ec: 48 83 f8 01                 	cmp	rax, 0x1
1408d06f0: 0f 85 6c 55 00 00           	jne	0x1408d5c62 <.text+0x8d4c62>
1408d06f6: 48 83 c1 08                 	add	rcx, 0x8
1408d06fa: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
1408d06fe: 0f 85 93 55 00 00           	jne	0x1408d5c97 <.text+0x8d4c97>
1408d0704: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
1408d070b: 48 89 8d 70 03 00 00        	mov	qword ptr [rbp + 0x370], rcx
1408d0712: 48 83 c1 08                 	add	rcx, 0x8
1408d0716: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1408d071b: 48 c7 44 24 30 00 00 00 00  	mov	qword ptr [rsp + 0x30], 0x0
1408d0724: 48 c7 44 24 28 01 00 00 00  	mov	qword ptr [rsp + 0x28], 0x1
1408d072d: 4c 89 fa                    	mov	rdx, r15
1408d0730: 49 89 f8                    	mov	r8, rdi
1408d0733: 4d 89 e1                    	mov	r9, r12
1408d0736: e8 05 66 34 00              	call	0x140c16d40 <.text+0xc15d40>
1408d073b: 48 8b 8d 70 03 00 00        	mov	rcx, qword ptr [rbp + 0x370]
1408d0742: 48 ff 01                    	inc	qword ptr [rcx]
1408d0745: 48 89 45 38                 	mov	qword ptr [rbp + 0x38], rax
1408d0749: b8 01 00 00 00              	mov	eax, 0x1
1408d074e: 48 8b 8d c0 04 00 00        	mov	rcx, qword ptr [rbp + 0x4c0]
1408d0755: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
1408d0759: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
1408d075c: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1408d0760: 89 02                       	mov	dword ptr [rdx], eax
1408d0762: 48 8b 16                    	mov	rdx, qword ptr [rsi]
1408d0765: 4c 8b 09                    	mov	r9, qword ptr [rcx]
1408d0768: 4c 8b 71 08                 	mov	r14, qword ptr [rcx + 0x8]
1408d076c: 44 8b 41 14                 	mov	r8d, dword ptr [rcx + 0x14]
1408d0770: 4d 8b a1 f8 00 00 00        	mov	r12, qword ptr [r9 + 0xf8]
1408d0777: 44 89 c0                    	mov	eax, r8d
1408d077a: 2b 41 10                    	sub	eax, dword ptr [rcx + 0x10]
1408d077d: 3d 00 b0 33 c2              	cmp	eax, 0xc233b000
1408d0782: 41 ba 00 b0 33 c2           	mov	r10d, 0xc233b000
1408d0788: 44 0f 42 d0                 	cmovb	r10d, eax
1408d078c: 49 8b 81 00 01 00 00        	mov	rax, qword ptr [r9 + 0x100]
1408d0793: 49 8d 1c 84                 	lea	rbx, [r12 + 4*rax]
1408d0797: 45 0f 57 c0                 	xorps	xmm8, xmm8
1408d079b: 44 0f 28 0d 3d 51 05 02     	movaps	xmm9, xmmword ptr [rip + 0x205513d] # 0x1429258e0
1408d07a3: f3 44 0f 10 15 c4 50 05 02  	movss	xmm10, dword ptr [rip + 0x20550c4] # 0x142925870
1408d07ac: 66 45 0f 76 db              	pcmpeqd	xmm11, xmm11
1408d07b1: 41 0f b6 89 08 01 00 00     	movzx	ecx, byte ptr [r9 + 0x108]
1408d07b9: 48 8d 82 88 3e 00 00        	lea	rax, [rdx + 0x3e88]
1408d07c0: 48 89 85 60 02 00 00        	mov	qword ptr [rbp + 0x260], rax
1408d07c7: 48 89 95 70 03 00 00        	mov	qword ptr [rbp + 0x370], rdx
1408d07ce: 48 8d 82 a8 3e 00 00        	lea	rax, [rdx + 0x3ea8]
1408d07d5: 48 89 85 28 03 00 00        	mov	qword ptr [rbp + 0x328], rax
1408d07dc: 41 bb 08 00 00 00           	mov	r11d, 0x8
1408d07e2: 45 31 ff                    	xor	r15d, r15d
1408d07e5: 31 f6                       	xor	esi, esi
1408d07e7: 45 31 ed                    	xor	r13d, r13d
1408d07ea: 48 c7 85 60 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x360], 0x0
1408d07f5: 48 c7 85 58 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x358], 0x0
1408d0800: b8 08 00 00 00              	mov	eax, 0x8
1408d0805: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1408d080c: 4c 89 8d 20 03 00 00        	mov	qword ptr [rbp + 0x320], r9
1408d0813: 44 89 85 10 03 00 00        	mov	dword ptr [rbp + 0x310], r8d
1408d081a: 44 89 95 08 03 00 00        	mov	dword ptr [rbp + 0x308], r10d
1408d0821: 88 8d a8 00 00 00           	mov	byte ptr [rbp + 0xa8], cl
1408d0827: 84 c9                       	test	cl, cl
1408d0829: 0f 85 8f 01 00 00           	jne	0x1408d09be <.text+0x8cf9be>
1408d082f: 90                          	nop
1408d0830: 49 39 f7                    	cmp	r15, rsi
1408d0833: 0f 85 50 01 00 00           	jne	0x1408d0989 <.text+0x8cf989>
1408d0839: 49 39 dc                    	cmp	r12, rbx
1408d083c: 0f 84 60 07 00 00           	je	0x1408d0fa2 <.text+0x8cffa2>
1408d0842: 41 8b 0c 24                 	mov	ecx, dword ptr [r12]
1408d0846: 49 83 c4 04                 	add	r12, 0x4
1408d084a: 49 8b 86 00 01 00 00        	mov	rax, qword ptr [r14 + 0x100]
1408d0851: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
1408d0855: 48 c1 e1 05                 	shl	rcx, 0x5
1408d0859: 48 83 7c 08 58 00           	cmp	qword ptr [rax + rcx + 0x58], 0x0
1408d085f: 74 0f                       	je	0x1408d0870 <.text+0x8cf870>
1408d0861: 48 8d 34 08                 	lea	rsi, [rax + rcx]
1408d0865: 48 8d 3c 08                 	lea	rdi, [rax + rcx]
1408d0869: 48 83 c7 58                 	add	rdi, 0x58
1408d086d: eb 30                       	jmp	0x1408d089f <.text+0x8cf89f>
1408d086f: 90                          	nop
1408d0870: 49 39 dc                    	cmp	r12, rbx
1408d0873: 0f 84 29 07 00 00           	je	0x1408d0fa2 <.text+0x8cffa2>
1408d0879: 41 8b 04 24                 	mov	eax, dword ptr [r12]
1408d087d: 49 83 c4 04                 	add	r12, 0x4
1408d0881: 49 8b b6 00 01 00 00        	mov	rsi, qword ptr [r14 + 0x100]
1408d0888: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
1408d088c: 48 c1 e0 05                 	shl	rax, 0x5
1408d0890: 48 83 7c 06 58 00           	cmp	qword ptr [rsi + rax + 0x58], 0x0
1408d0896: 74 d8                       	je	0x1408d0870 <.text+0x8cf870>
1408d0898: 48 01 c6                    	add	rsi, rax
1408d089b: 48 8d 7e 58                 	lea	rdi, [rsi + 0x58]
1408d089f: 8b 86 94 00 00 00           	mov	eax, dword ptr [rsi + 0x94]
1408d08a5: 4d 8b 86 a8 01 00 00        	mov	r8, qword ptr [r14 + 0x1a8]
1408d08ac: 48 8d 04 c0                 	lea	rax, [rax + 8*rax]
1408d08b0: 49 8d 0c c0                 	lea	rcx, [r8 + 8*rax]
1408d08b4: 49 8b 91 10 01 00 00        	mov	rdx, qword ptr [r9 + 0x110]
1408d08bb: 49 8b 44 c0 40              	mov	rax, qword ptr [r8 + 8*rax + 0x40]
1408d08c0: 48 39 d0                    	cmp	rax, rdx
1408d08c3: 76 3b                       	jbe	0x1408d0900 <.text+0x8cf900>
1408d08c5: 4c 8b 41 38                 	mov	r8, qword ptr [rcx + 0x38]
1408d08c9: 49 8b 14 d0                 	mov	rdx, qword ptr [r8 + 8*rdx]
1408d08cd: 48 85 d2                    	test	rdx, rdx
1408d08d0: 74 2e                       	je	0x1408d0900 <.text+0x8cf900>
1408d08d2: 4c 8b 41 18                 	mov	r8, qword ptr [rcx + 0x18]
1408d08d6: 48 f7 d2                    	not	rdx
1408d08d9: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
1408d08dd: 48 c1 e2 04                 	shl	rdx, 0x4
1408d08e1: 49 8b 54 10 10              	mov	rdx, qword ptr [r8 + rdx + 0x10]
1408d08e6: 48 89 95 58 03 00 00        	mov	qword ptr [rbp + 0x358], rdx
1408d08ed: 49 8b 91 18 01 00 00        	mov	rdx, qword ptr [r9 + 0x118]
1408d08f4: 48 39 d0                    	cmp	rax, rdx
1408d08f7: 77 1e                       	ja	0x1408d0917 <.text+0x8cf917>
1408d08f9: eb 55                       	jmp	0x1408d0950 <.text+0x8cf950>
1408d08fb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1408d0900: 48 c7 85 58 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x358], 0x0
1408d090b: 49 8b 91 18 01 00 00        	mov	rdx, qword ptr [r9 + 0x118]
1408d0912: 48 39 d0                    	cmp	rax, rdx
1408d0915: 76 39                       	jbe	0x1408d0950 <.text+0x8cf950>
1408d0917: 48 8b 41 38                 	mov	rax, qword ptr [rcx + 0x38]
1408d091b: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1408d091f: 48 85 c0                    	test	rax, rax
1408d0922: 74 2c                       	je	0x1408d0950 <.text+0x8cf950>
1408d0924: 48 8b 51 18                 	mov	rdx, qword ptr [rcx + 0x18]
1408d0928: 48 f7 d0                    	not	rax
1408d092b: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1408d092f: 48 c1 e0 04                 	shl	rax, 0x4
1408d0933: 48 8b 44 02 10              	mov	rax, qword ptr [rdx + rax + 0x10]
1408d0938: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
1408d093f: eb 1a                       	jmp	0x1408d095b <.text+0x8cf95b>
1408d0941: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1408d0950: 48 c7 85 60 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x360], 0x0
1408d095b: 49 8b 91 20 01 00 00        	mov	rdx, qword ptr [r9 + 0x120]
1408d0962: e8 09 fe 42 00              	call	0x140d00770 <.text+0xcff770>
1408d0967: 49 89 c5                    	mov	r13, rax
1408d096a: 4c 8b 5e 50                 	mov	r11, qword ptr [rsi + 0x50]
1408d096e: 48 8b 37                    	mov	rsi, qword ptr [rdi]
1408d0971: 45 31 ff                    	xor	r15d, r15d
1408d0974: 4c 8b 8d 20 03 00 00        	mov	r9, qword ptr [rbp + 0x320]
1408d097b: 44 8b 85 10 03 00 00        	mov	r8d, dword ptr [rbp + 0x310]
1408d0982: 44 8b 95 08 03 00 00        	mov	r10d, dword ptr [rbp + 0x308]
1408d0989: 4c 89 f8                    	mov	rax, r15
1408d098c: 48 c1 e0 04                 	shl	rax, 0x4
1408d0990: 41 8b 4c 03 08              	mov	ecx, dword ptr [r11 + rax + 0x8]
1408d0995: 44 89 c2                    	mov	edx, r8d
1408d0998: 41 2b 54 8d 00              	sub	edx, dword ptr [r13 + 4*rcx]
1408d099d: 44 39 d2                    	cmp	edx, r10d
1408d09a0: 72 0d                       	jb	0x1408d09af <.text+0x8cf9af>
1408d09a2: 49 ff c7                    	inc	r15
1408d09a5: 49 39 f7                    	cmp	r15, rsi
1408d09a8: 75 df                       	jne	0x1408d0989 <.text+0x8cf989>
1408d09aa: e9 8a fe ff ff              	jmp	0x1408d0839 <.text+0x8cf839>
1408d09af: 4c 89 9d 90 01 00 00        	mov	qword ptr [rbp + 0x190], r11
1408d09b6: 4c 01 d8                    	add	rax, r11
1408d09b9: e9 a7 01 00 00              	jmp	0x1408d0b65 <.text+0x8cfb65>
1408d09be: 4c 89 9d 90 01 00 00        	mov	qword ptr [rbp + 0x190], r11
1408d09c5: 48 8b 95 58 02 00 00        	mov	rdx, qword ptr [rbp + 0x258]
1408d09cc: 49 39 f7                    	cmp	r15, rsi
1408d09cf: 0f 85 67 01 00 00           	jne	0x1408d0b3c <.text+0x8cfb3c>
1408d09d5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1408d09e0: 49 39 dc                    	cmp	r12, rbx
1408d09e3: 0f 84 b9 05 00 00           	je	0x1408d0fa2 <.text+0x8cffa2>
1408d09e9: 41 8b 0c 24                 	mov	ecx, dword ptr [r12]
1408d09ed: 49 83 c4 04                 	add	r12, 0x4
1408d09f1: 49 8b 86 a8 01 00 00        	mov	rax, qword ptr [r14 + 0x1a8]
1408d09f8: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1408d09fc: 48 83 7c c8 10 00           	cmp	qword ptr [rax + 8*rcx + 0x10], 0x0
1408d0a02: 74 2c                       	je	0x1408d0a30 <.text+0x8cfa30>
1408d0a04: 4c 8d 3c c8                 	lea	r15, [rax + 8*rcx]
1408d0a08: 48 8d 34 c8                 	lea	rsi, [rax + 8*rcx]
1408d0a0c: 48 83 c6 10                 	add	rsi, 0x10
1408d0a10: 49 8b 89 10 01 00 00        	mov	rcx, qword ptr [r9 + 0x110]
1408d0a17: 49 8b 47 40                 	mov	rax, qword ptr [r15 + 0x40]
1408d0a1b: 48 39 c8                    	cmp	rax, rcx
1408d0a1e: 0f 86 8c 00 00 00           	jbe	0x1408d0ab0 <.text+0x8cfab0>
1408d0a24: eb 46                       	jmp	0x1408d0a6c <.text+0x8cfa6c>
1408d0a26: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1408d0a30: 49 39 dc                    	cmp	r12, rbx
1408d0a33: 0f 84 69 05 00 00           	je	0x1408d0fa2 <.text+0x8cffa2>
1408d0a39: 41 8b 0c 24                 	mov	ecx, dword ptr [r12]
1408d0a3d: 49 83 c4 04                 	add	r12, 0x4
1408d0a41: 49 8b 86 a8 01 00 00        	mov	rax, qword ptr [r14 + 0x1a8]
1408d0a48: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1408d0a4c: 48 83 7c c8 10 00           	cmp	qword ptr [rax + 8*rcx + 0x10], 0x0
1408d0a52: 74 dc                       	je	0x1408d0a30 <.text+0x8cfa30>
1408d0a54: 4c 8d 3c c8                 	lea	r15, [rax + 8*rcx]
1408d0a58: 49 8d 77 10                 	lea	rsi, [r15 + 0x10]
1408d0a5c: 49 8b 89 10 01 00 00        	mov	rcx, qword ptr [r9 + 0x110]
1408d0a63: 49 8b 47 40                 	mov	rax, qword ptr [r15 + 0x40]
1408d0a67: 48 39 c8                    	cmp	rax, rcx
1408d0a6a: 76 44                       	jbe	0x1408d0ab0 <.text+0x8cfab0>
1408d0a6c: 49 8b 57 38                 	mov	rdx, qword ptr [r15 + 0x38]
1408d0a70: 48 8b 0c ca                 	mov	rcx, qword ptr [rdx + 8*rcx]
1408d0a74: 48 85 c9                    	test	rcx, rcx
1408d0a77: 74 37                       	je	0x1408d0ab0 <.text+0x8cfab0>
1408d0a79: 49 8b 57 18                 	mov	rdx, qword ptr [r15 + 0x18]
1408d0a7d: 48 f7 d1                    	not	rcx
1408d0a80: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1408d0a84: 48 c1 e1 04                 	shl	rcx, 0x4
1408d0a88: 48 8b 4c 0a 10              	mov	rcx, qword ptr [rdx + rcx + 0x10]
1408d0a8d: 48 89 8d 58 03 00 00        	mov	qword ptr [rbp + 0x358], rcx
1408d0a94: 49 8b 89 18 01 00 00        	mov	rcx, qword ptr [r9 + 0x118]
1408d0a9b: 48 39 c8                    	cmp	rax, rcx
1408d0a9e: 77 27                       	ja	0x1408d0ac7 <.text+0x8cfac7>
1408d0aa0: eb 5e                       	jmp	0x1408d0b00 <.text+0x8cfb00>
1408d0aa2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1408d0ab0: 48 c7 85 58 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x358], 0x0
1408d0abb: 49 8b 89 18 01 00 00        	mov	rcx, qword ptr [r9 + 0x118]
1408d0ac2: 48 39 c8                    	cmp	rax, rcx
1408d0ac5: 76 39                       	jbe	0x1408d0b00 <.text+0x8cfb00>
1408d0ac7: 49 8b 47 38                 	mov	rax, qword ptr [r15 + 0x38]
1408d0acb: 48 8b 04 c8                 	mov	rax, qword ptr [rax + 8*rcx]
1408d0acf: 48 85 c0                    	test	rax, rax
1408d0ad2: 74 2c                       	je	0x1408d0b00 <.text+0x8cfb00>
1408d0ad4: 49 8b 4f 18                 	mov	rcx, qword ptr [r15 + 0x18]
1408d0ad8: 48 f7 d0                    	not	rax
1408d0adb: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1408d0adf: 48 c1 e0 04                 	shl	rax, 0x4
1408d0ae3: 48 8b 44 01 10              	mov	rax, qword ptr [rcx + rax + 0x10]
1408d0ae8: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
1408d0aef: eb 1a                       	jmp	0x1408d0b0b <.text+0x8cfb0b>
1408d0af1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1408d0b00: 48 c7 85 60 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x360], 0x0
1408d0b0b: 49 8b 91 20 01 00 00        	mov	rdx, qword ptr [r9 + 0x120]
1408d0b12: 4c 89 f9                    	mov	rcx, r15
1408d0b15: e8 56 fc 42 00              	call	0x140d00770 <.text+0xcff770>
1408d0b1a: 49 89 c5                    	mov	r13, rax
1408d0b1d: 49 8b 57 08                 	mov	rdx, qword ptr [r15 + 0x8]
1408d0b21: 48 8b 36                    	mov	rsi, qword ptr [rsi]
1408d0b24: 45 31 ff                    	xor	r15d, r15d
1408d0b27: 4c 8b 8d 20 03 00 00        	mov	r9, qword ptr [rbp + 0x320]
1408d0b2e: 44 8b 85 10 03 00 00        	mov	r8d, dword ptr [rbp + 0x310]
1408d0b35: 44 8b 95 08 03 00 00        	mov	r10d, dword ptr [rbp + 0x308]
1408d0b3c: 44 89 f9                    	mov	ecx, r15d
1408d0b3f: 44 89 c0                    	mov	eax, r8d
1408d0b42: 41 2b 44 8d 00              	sub	eax, dword ptr [r13 + 4*rcx]
1408d0b47: 44 39 d0                    	cmp	eax, r10d
1408d0b4a: 72 0e                       	jb	0x1408d0b5a <.text+0x8cfb5a>
1408d0b4c: 49 ff c7                    	inc	r15
1408d0b4f: 49 39 f7                    	cmp	r15, rsi
1408d0b52: 0f 84 88 fe ff ff           	je	0x1408d09e0 <.text+0x8cf9e0>
1408d0b58: eb e2                       	jmp	0x1408d0b3c <.text+0x8cfb3c>
1408d0b5a: 48 89 95 58 02 00 00        	mov	qword ptr [rbp + 0x258], rdx
1408d0b61: 4a 8d 04 fa                 	lea	rax, [rdx + 8*r15]
1408d0b65: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1408d0b69: 48 89 ca                    	mov	rdx, rcx
1408d0b6c: 48 c1 e2 04                 	shl	rdx, 0x4
1408d0b70: 48 03 95 58 03 00 00        	add	rdx, qword ptr [rbp + 0x358]
1408d0b77: 48 89 95 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rdx
1408d0b7e: 48 c1 e1 05                 	shl	rcx, 0x5
1408d0b82: 48 03 8d 60 03 00 00        	add	rcx, qword ptr [rbp + 0x360]
1408d0b89: 48 8b 00                    	mov	rax, qword ptr [rax]
1408d0b8c: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1408d0b93: f3 0f 10 79 50              	movss	xmm7, dword ptr [rcx + 0x50]
1408d0b98: f3 0f 10 71 54              	movss	xmm6, dword ptr [rcx + 0x54]
1408d0b9d: f3 0f 10 41 5c              	movss	xmm0, dword ptr [rcx + 0x5c]
1408d0ba2: f3 0f 5c 41 58              	subss	xmm0, dword ptr [rcx + 0x58]
1408d0ba7: 44 0f 28 e0                 	movaps	xmm12, xmm0
1408d0bab: f3 45 0f 59 e0              	mulss	xmm12, xmm8
1408d0bb0: 44 0f 14 e0                 	unpcklps	xmm12, xmm0             # xmm12 = xmm12[0],xmm0[0],xmm12[1],xmm0[1]
1408d0bb4: 45 0f 59 e1                 	mulps	xmm12, xmm9
1408d0bb8: 44 0f 13 a5 70 02 00 00     	movlps	qword ptr [rbp + 0x270], xmm12
1408d0bc0: f3 44 0f 11 a5 78 02 00 00  	movss	dword ptr [rbp + 0x278], xmm12
1408d0bc9: 8b 85 78 02 00 00           	mov	eax, dword ptr [rbp + 0x278]
1408d0bcf: 48 8d 8d bc 01 00 00        	lea	rcx, [rbp + 0x1bc]
1408d0bd6: 89 41 08                    	mov	dword ptr [rcx + 0x8], eax
1408d0bd9: 48 8b 85 70 02 00 00        	mov	rax, qword ptr [rbp + 0x270]
1408d0be0: 48 89 01                    	mov	qword ptr [rcx], rax
1408d0be3: f3 0f 11 bd b0 01 00 00     	movss	dword ptr [rbp + 0x1b0], xmm7
1408d0beb: f3 0f 11 85 b4 01 00 00     	movss	dword ptr [rbp + 0x1b4], xmm0
1408d0bf3: f3 0f 11 b5 b8 01 00 00     	movss	dword ptr [rbp + 0x1b8], xmm6
1408d0bfb: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d0c02: e8 39 99 b8 00              	call	0x14145a540 <.text+0x1459540>
1408d0c07: 48 89 85 a8 02 00 00        	mov	qword ptr [rbp + 0x2a8], rax
1408d0c0e: f3 41 0f 59 fa              	mulss	xmm7, xmm10
1408d0c13: f3 41 0f 59 f2              	mulss	xmm6, xmm10
1408d0c18: f3 41 0f 16 cc              	movshdup	xmm1, xmm12             # xmm1 = xmm12[1,1,3,3]
1408d0c1d: 0f 28 c7                    	movaps	xmm0, xmm7
1408d0c20: 0f 28 d6                    	movaps	xmm2, xmm6
1408d0c23: e8 58 ad de 00              	call	0x1416bb980 <.text+0x16ba980>
1408d0c28: 48 89 d7                    	mov	rdi, rdx
1408d0c2b: 48 89 85 f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rax
1408d0c32: 48 89 85 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rax
1408d0c39: 48 89 95 b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rdx
1408d0c40: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d0c47: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
1408d0c4e: e8 dd 0e 3c 00              	call	0x140c91b30 <.text+0xc90b30>
1408d0c53: 0f 28 85 b0 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x1b0]
1408d0c5a: 0f 28 8d c0 01 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x1c0]
1408d0c61: 48 8d 85 f8 00 00 00        	lea	rax, [rbp + 0xf8]
1408d0c68: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
1408d0c6c: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
1408d0c6f: 48 c7 85 b0 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x1
1408d0c7a: 48 c7 85 b8 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b8], 0x1
1408d0c85: 48 c7 85 c0 01 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x1c0], 0x4
1408d0c90: 48 c7 85 c8 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1c8], 0x0
1408d0c9b: 48 c7 85 d0 01 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x1d0], 0x4
1408d0ca6: 48 c7 85 d8 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1d8], 0x0
1408d0cb1: 48 8b 85 f0 02 00 00        	mov	rax, qword ptr [rbp + 0x2f0]
1408d0cb8: 48 89 85 e0 01 00 00        	mov	qword ptr [rbp + 0x1e0], rax
1408d0cbf: 48 89 bd e8 01 00 00        	mov	qword ptr [rbp + 0x1e8], rdi
1408d0cc6: 48 c7 85 f0 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1f0], 0x0
1408d0cd1: 48 c7 85 00 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x200], 0x0
1408d0cdc: 48 8b 85 10 01 00 00        	mov	rax, qword ptr [rbp + 0x110]
1408d0ce3: 48 8d 8d bc 01 00 00        	lea	rcx, [rbp + 0x1bc]
1408d0cea: 48 89 41 6c                 	mov	qword ptr [rcx + 0x6c], rax
1408d0cee: f3 0f 6f 85 f0 00 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0xf0]
1408d0cf6: f3 0f 6f 8d 00 01 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0x100]
1408d0cfe: f3 0f 7f 49 5c              	movdqu	xmmword ptr [rcx + 0x5c], xmm1
1408d0d03: f3 0f 7f 41 4c              	movdqu	xmmword ptr [rcx + 0x4c], xmm0
1408d0d08: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
1408d0d0f: 48 89 85 30 02 00 00        	mov	qword ptr [rbp + 0x230], rax
1408d0d16: 0f b6 05 ec 3e d6 02        	movzx	eax, byte ptr [rip + 0x2d63eec] # 0x143634c09
1408d0d1d: b9 90 00 00 00              	mov	ecx, 0x90
1408d0d22: ba 10 00 00 00              	mov	edx, 0x10
1408d0d27: e8 e4 2e d4 ff              	call	0x140613c10 <.text+0x612c10>
1408d0d2c: 48 85 c0                    	test	rax, rax
1408d0d2f: 0f 84 e2 4f 00 00           	je	0x1408d5d17 <.text+0x8d4d17>
1408d0d35: 41 b8 90 00 00 00           	mov	r8d, 0x90
1408d0d3b: 48 89 c1                    	mov	rcx, rax
1408d0d3e: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d0d45: 48 89 c7                    	mov	rdi, rax
1408d0d48: e8 b3 2e 05 02              	call	0x142923c00 <.text+0x2922c00>
1408d0d4d: 48 89 bd f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rdi
1408d0d54: 48 89 bd d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rdi
1408d0d5b: c6 85 b8 00 00 00 0c        	mov	byte ptr [rbp + 0xb8], 0xc
1408d0d62: 48 8b 85 90 02 00 00        	mov	rax, qword ptr [rbp + 0x290]
1408d0d69: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
1408d0d70: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
1408d0d77: c6 85 4f 03 00 00 01        	mov	byte ptr [rbp + 0x34f], 0x1
1408d0d7e: 48 8b 8d 28 03 00 00        	mov	rcx, qword ptr [rbp + 0x328]
1408d0d85: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d0d8c: e8 df ff cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d0d91: 48 89 85 a8 02 00 00        	mov	qword ptr [rbp + 0x2a8], rax
1408d0d98: 49 ff c7                    	inc	r15
1408d0d9b: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d0da2: 48 8b 8a 88 3e 00 00        	mov	rcx, qword ptr [rdx + 0x3e88]
1408d0da9: 48 8b 92 90 3e 00 00        	mov	rdx, qword ptr [rdx + 0x3e90]
1408d0db0: 49 89 d0                    	mov	r8, rdx
1408d0db3: 4c 8b 8d a8 02 00 00        	mov	r9, qword ptr [rbp + 0x2a8]
1408d0dba: 4d 21 c8                    	and	r8, r9
1408d0dbd: 49 c1 e9 39                 	shr	r9, 0x39
1408d0dc1: 66 41 0f 6e c1              	movd	xmm0, r9d
1408d0dc6: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d0dca: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d0dcf: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d0dd4: 4c 8d 49 e0                 	lea	r9, [rcx - 0x20]
1408d0dd8: 45 31 db                    	xor	r11d, r11d
1408d0ddb: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d0de1: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d0de5: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d0de9: 66 0f d7 c2                 	pmovmskb	eax, xmm2
1408d0ded: 85 c0                       	test	eax, eax
1408d0def: 74 47                       	je	0x1408d0e38 <.text+0x8cfe38>
1408d0df1: 4c 89 9d 68 02 00 00        	mov	qword ptr [rbp + 0x268], r11
1408d0df8: f3 0f bc f8                 	tzcnt	edi, eax
1408d0dfc: 4c 01 c7                    	add	rdi, r8
1408d0dff: 48 21 d7                    	and	rdi, rdx
1408d0e02: 49 89 fa                    	mov	r10, rdi
1408d0e05: 49 c1 e2 05                 	shl	r10, 0x5
1408d0e09: 49 89 d3                    	mov	r11, rdx
1408d0e0c: 4c 89 ca                    	mov	rdx, r9
1408d0e0f: 4d 29 d1                    	sub	r9, r10
1408d0e12: 4c 8b 95 90 02 00 00        	mov	r10, qword ptr [rbp + 0x290]
1408d0e19: 4d 39 11                    	cmp	qword ptr [r9], r10
1408d0e1c: 74 42                       	je	0x1408d0e60 <.text+0x8cfe60>
1408d0e1e: 44 8d 48 ff                 	lea	r9d, [rax - 0x1]
1408d0e22: 66 41 21 c1                 	and	r9w, ax
1408d0e26: 44 89 c8                    	mov	eax, r9d
1408d0e29: 49 89 d1                    	mov	r9, rdx
1408d0e2c: 4c 89 da                    	mov	rdx, r11
1408d0e2f: 4c 8b 9d 68 02 00 00        	mov	r11, qword ptr [rbp + 0x268]
1408d0e36: 75 c0                       	jne	0x1408d0df8 <.text+0x8cfdf8>
1408d0e38: 4d 89 ca                    	mov	r10, r9
1408d0e3b: 66 41 0f 74 cb              	pcmpeqb	xmm1, xmm11
1408d0e40: 66 44 0f d7 c9              	pmovmskb	r9d, xmm1
1408d0e45: 45 85 c9                    	test	r9d, r9d
1408d0e48: 75 42                       	jne	0x1408d0e8c <.text+0x8cfe8c>
1408d0e4a: 4d 01 d8                    	add	r8, r11
1408d0e4d: 49 83 c0 10                 	add	r8, 0x10
1408d0e51: 49 83 c3 10                 	add	r11, 0x10
1408d0e55: 49 21 d0                    	and	r8, rdx
1408d0e58: 4d 89 d1                    	mov	r9, r10
1408d0e5b: e9 7b ff ff ff              	jmp	0x1408d0ddb <.text+0x8cfddb>
1408d0e60: 48 f7 df                    	neg	rdi
1408d0e63: 48 c1 e7 05                 	shl	rdi, 0x5
1408d0e67: 48 01 f9                    	add	rcx, rdi
1408d0e6a: 48 89 8d b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rcx
1408d0e71: 48 8b 85 60 02 00 00        	mov	rax, qword ptr [rbp + 0x260]
1408d0e78: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d0e7f: 48 c7 85 b0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2b0], 0x0
1408d0e8a: eb 3f                       	jmp	0x1408d0ecb <.text+0x8cfecb>
1408d0e8c: 48 8b 8d 70 03 00 00        	mov	rcx, qword ptr [rbp + 0x370]
1408d0e93: 48 83 b9 98 3e 00 00 00     	cmp	qword ptr [rcx + 0x3e98], 0x0
1408d0e9b: 48 8b 95 90 02 00 00        	mov	rdx, qword ptr [rbp + 0x290]
1408d0ea2: 0f 84 cc 00 00 00           	je	0x1408d0f74 <.text+0x8cff74>
1408d0ea8: 48 8b 8d 60 02 00 00        	mov	rcx, qword ptr [rbp + 0x260]
1408d0eaf: 48 89 8d b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rcx
1408d0eb6: 48 89 95 b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rdx
1408d0ebd: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
1408d0ec4: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d0ecb: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
1408d0ed2: 0f 10 85 b8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xb8]
1408d0ed9: f3 0f 6f 8d c8 00 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0xc8]
1408d0ee1: 66 0f 7f 8d 00 01 00 00     	movdqa	xmmword ptr [rbp + 0x100], xmm1
1408d0ee9: 0f 29 85 f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm0
1408d0ef0: 8b 41 18                    	mov	eax, dword ptr [rcx + 0x18]
1408d0ef3: 89 85 b8 01 00 00           	mov	dword ptr [rbp + 0x1b8], eax
1408d0ef9: 48 8b 41 10                 	mov	rax, qword ptr [rcx + 0x10]
1408d0efd: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
1408d0f04: 66 0f 6f 01                 	movdqa	xmm0, xmmword ptr [rcx]
1408d0f08: 66 0f 7f 85 c0 01 00 00     	movdqa	xmmword ptr [rbp + 0x1c0], xmm0
1408d0f10: c6 85 4f 03 00 00 00        	mov	byte ptr [rbp + 0x34f], 0x0
1408d0f17: 48 8d 85 b0 01 00 00        	lea	rax, [rbp + 0x1b0]
1408d0f1e: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408d0f23: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1408d0f2a: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1408d0f31: 4c 8b 85 70 03 00 00        	mov	r8, qword ptr [rbp + 0x370]
1408d0f38: 4c 8d 8d f0 00 00 00        	lea	r9, [rbp + 0xf0]
1408d0f3f: e8 0c fc 26 00              	call	0x140b40b50 <.text+0xb3fb50>
1408d0f44: 4c 8b 8d 20 03 00 00        	mov	r9, qword ptr [rbp + 0x320]
1408d0f4b: 44 8b 85 10 03 00 00        	mov	r8d, dword ptr [rbp + 0x310]
1408d0f52: 44 8b 95 08 03 00 00        	mov	r10d, dword ptr [rbp + 0x308]
1408d0f59: 0f b6 8d a8 00 00 00        	movzx	ecx, byte ptr [rbp + 0xa8]
1408d0f60: 4c 8b 9d 90 01 00 00        	mov	r11, qword ptr [rbp + 0x190]
1408d0f67: 84 c9                       	test	cl, cl
1408d0f69: 0f 84 c1 f8 ff ff           	je	0x1408d0830 <.text+0x8cf830>
1408d0f6f: e9 4a fa ff ff              	jmp	0x1408d09be <.text+0x8cf9be>
1408d0f74: c6 85 4f 03 00 00 01        	mov	byte ptr [rbp + 0x34f], 0x1
1408d0f7b: ba 01 00 00 00              	mov	edx, 0x1
1408d0f80: 48 8b 8d 60 02 00 00        	mov	rcx, qword ptr [rbp + 0x260]
1408d0f87: 4c 8b 85 28 03 00 00        	mov	r8, qword ptr [rbp + 0x328]
1408d0f8e: 41 b1 01                    	mov	r9b, 0x1
1408d0f91: e8 0a 8d fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d0f96: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
1408d0f9d: e9 06 ff ff ff              	jmp	0x1408d0ea8 <.text+0x8cfea8>
1408d0fa2: 4c 8b 85 c8 04 00 00        	mov	r8, qword ptr [rbp + 0x4c8]
1408d0fa9: 49 8b 08                    	mov	rcx, qword ptr [r8]
1408d0fac: 49 8b 40 10                 	mov	rax, qword ptr [r8 + 0x10]
1408d0fb0: 48 8b 09                    	mov	rcx, qword ptr [rcx]
1408d0fb3: 48 39 48 40                 	cmp	qword ptr [rax + 0x40], rcx
1408d0fb7: 0f 86 a6 00 00 00           	jbe	0x1408d1063 <.text+0x8d0063>
1408d0fbd: 48 8b 50 38                 	mov	rdx, qword ptr [rax + 0x38]
1408d0fc1: 48 8b 0c ca                 	mov	rcx, qword ptr [rdx + 8*rcx]
1408d0fc5: 48 85 c9                    	test	rcx, rcx
1408d0fc8: 0f 84 95 00 00 00           	je	0x1408d1063 <.text+0x8d0063>
1408d0fce: 48 8b 40 08                 	mov	rax, qword ptr [rax + 0x8]
1408d0fd2: 48 f7 d1                    	not	rcx
1408d0fd5: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1408d0fd9: 4d 8b 68 08                 	mov	r13, qword ptr [r8 + 0x8]
1408d0fdd: 4d 8b 45 00                 	mov	r8, qword ptr [r13]
1408d0fe1: 31 d2                       	xor	edx, edx
1408d0fe3: 4d 89 c1                    	mov	r9, r8
1408d0fe6: 4c 2b 4c c8 18              	sub	r9, qword ptr [rax + 8*rcx + 0x18]
1408d0feb: 4c 0f 42 ca                 	cmovb	r9, rdx
1408d0fef: 4c 2b 44 c8 38              	sub	r8, qword ptr [rax + 8*rcx + 0x38]
1408d0ff4: 4c 0f 42 c2                 	cmovb	r8, rdx
1408d0ff8: 4c 89 cb                    	mov	rbx, r9
1408d0ffb: 48 c1 e3 04                 	shl	rbx, 0x4
1408d0fff: 48 03 5c c8 08              	add	rbx, qword ptr [rax + 8*rcx + 0x8]
1408d1004: 4c 8b 54 c8 10              	mov	r10, qword ptr [rax + 8*rcx + 0x10]
1408d1009: 4c 8b 5c c8 30              	mov	r11, qword ptr [rax + 8*rcx + 0x30]
1408d100e: 4d 29 ca                    	sub	r10, r9
1408d1011: 4c 0f 42 d2                 	cmovb	r10, rdx
1408d1015: 41 b9 08 00 00 00           	mov	r9d, 0x8
1408d101b: 49 0f 42 d9                 	cmovb	rbx, r9
1408d101f: 4d 89 c7                    	mov	r15, r8
1408d1022: 49 c1 e7 04                 	shl	r15, 0x4
1408d1026: 4c 03 7c c8 28              	add	r15, qword ptr [rax + 8*rcx + 0x28]
1408d102b: 4d 29 c3                    	sub	r11, r8
1408d102e: 4c 0f 42 da                 	cmovb	r11, rdx
1408d1032: 4d 0f 42 f9                 	cmovb	r15, r9
1408d1036: 48 8b 44 c8 40              	mov	rax, qword ptr [rax + 8*rcx + 0x40]
1408d103b: 4c 29 d8                    	sub	rax, r11
1408d103e: 4c 29 d0                    	sub	rax, r10
1408d1041: 49 89 45 00                 	mov	qword ptr [r13], rax
1408d1045: 49 c1 e2 04                 	shl	r10, 0x4
1408d1049: 49 01 da                    	add	r10, rbx
1408d104c: 4c 89 95 20 03 00 00        	mov	qword ptr [rbp + 0x320], r10
1408d1053: 49 c1 e3 04                 	shl	r11, 0x4
1408d1057: 4d 01 fb                    	add	r11, r15
1408d105a: 4c 89 9d 10 03 00 00        	mov	qword ptr [rbp + 0x310], r11
1408d1061: eb 03                       	jmp	0x1408d1066 <.text+0x8d0066>
1408d1063: 45 31 ed                    	xor	r13d, r13d
1408d1066: 31 f6                       	xor	esi, esi
1408d1068: 66 0f 76 f6                 	pcmpeqd	xmm6, xmm6
1408d106c: e9 c8 00 00 00              	jmp	0x1408d1139 <.text+0x8d0139>
1408d1071: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1408d1080: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d1086: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d108a: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d108e: 66 0f d7 da                 	pmovmskb	ebx, xmm2
1408d1092: 85 db                       	test	ebx, ebx
1408d1094: 74 2a                       	je	0x1408d10c0 <.text+0x8d00c0>
1408d1096: f3 0f bc c3                 	tzcnt	eax, ebx
1408d109a: 4c 01 c0                    	add	rax, r8
1408d109d: 48 21 d0                    	and	rax, rdx
1408d10a0: 49 89 c1                    	mov	r9, rax
1408d10a3: 49 c1 e1 05                 	shl	r9, 0x5
1408d10a7: 4d 89 e2                    	mov	r10, r12
1408d10aa: 4d 29 ca                    	sub	r10, r9
1408d10ad: 4d 39 32                    	cmp	qword ptr [r10], r14
1408d10b0: 74 2e                       	je	0x1408d10e0 <.text+0x8d00e0>
1408d10b2: 8d 43 ff                    	lea	eax, [rbx - 0x1]
1408d10b5: 66 21 d8                    	and	ax, bx
1408d10b8: 89 c3                       	mov	ebx, eax
1408d10ba: 75 da                       	jne	0x1408d1096 <.text+0x8d0096>
1408d10bc: 0f 1f 40 00                 	nop	dword ptr [rax]
1408d10c0: 66 0f 74 ce                 	pcmpeqb	xmm1, xmm6
1408d10c4: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d10c8: 85 c0                       	test	eax, eax
1408d10ca: 75 44                       	jne	0x1408d1110 <.text+0x8d0110>
1408d10cc: 49 01 f8                    	add	r8, rdi
1408d10cf: 49 83 c0 10                 	add	r8, 0x10
1408d10d3: 48 83 c7 10                 	add	rdi, 0x10
1408d10d7: 49 21 d0                    	and	r8, rdx
1408d10da: eb a4                       	jmp	0x1408d1080 <.text+0x8d0080>
1408d10dc: 0f 1f 40 00                 	nop	dword ptr [rax]
1408d10e0: 49 c1 f9 05                 	sar	r9, 0x5
1408d10e4: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d10e8: 49 21 d0                    	and	r8, rdx
1408d10eb: f3 42 0f 6f 04 01           	movdqu	xmm0, xmmword ptr [rcx + r8]
1408d10f1: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d10f5: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d10f9: 85 d2                       	test	edx, edx
1408d10fb: 74 53                       	je	0x1408d1150 <.text+0x8d0150>
1408d10fd: 66 0f bd d2                 	bsr	dx, dx
1408d1101: 83 f2 0f                    	xor	edx, 0xf
1408d1104: eb 4e                       	jmp	0x1408d1154 <.text+0x8d0154>
1408d1106: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1408d1110: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d1117: 48 83 b8 98 3e 00 00 00     	cmp	qword ptr [rax + 0x3e98], 0x0
1408d111f: 4c 89 fb                    	mov	rbx, r15
1408d1122: 4d 89 df                    	mov	r15, r11
1408d1125: 4c 8b b5 60 03 00 00        	mov	r14, qword ptr [rbp + 0x360]
1408d112c: 48 8b bd 58 03 00 00        	mov	rdi, qword ptr [rbp + 0x358]
1408d1133: 0f 84 e0 00 00 00           	je	0x1408d1219 <.text+0x8d0219>
1408d1139: 4c 89 f0                    	mov	rax, r14
1408d113c: 48 89 f9                    	mov	rcx, rdi
1408d113f: 48 85 f6                    	test	rsi, rsi
1408d1142: 0f 85 10 01 00 00           	jne	0x1408d1258 <.text+0x8d0258>
1408d1148: e9 f3 00 00 00              	jmp	0x1408d1240 <.text+0x8d0240>
1408d114d: 0f 1f 00                    	nop	dword ptr [rax]
1408d1150: 66 ba 10 00                 	mov	dx, 0x10
1408d1154: 4c 89 fb                    	mov	rbx, r15
1408d1157: 4d 89 df                    	mov	r15, r11
1408d115a: 4c 8b b5 60 03 00 00        	mov	r14, qword ptr [rbp + 0x360]
1408d1161: f3 42 0f 6f 04 09           	movdqu	xmm0, xmmword ptr [rcx + r9]
1408d1167: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d116b: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d1170: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d1177: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d117c: 44 01 d2                    	add	edx, r10d
1408d117f: 41 b2 80                    	mov	r10b, -0x80
1408d1182: 66 83 fa 0f                 	cmp	dx, 0xf
1408d1186: 48 8b bd 58 03 00 00        	mov	rdi, qword ptr [rbp + 0x358]
1408d118d: 77 11                       	ja	0x1408d11a0 <.text+0x8d01a0>
1408d118f: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d1196: 48 ff 82 98 3e 00 00        	inc	qword ptr [rdx + 0x3e98]
1408d119d: 41 b2 ff                    	mov	r10b, -0x1
1408d11a0: 48 f7 d8                    	neg	rax
1408d11a3: 48 c1 e0 05                 	shl	rax, 0x5
1408d11a7: 48 01 c8                    	add	rax, rcx
1408d11aa: 46 88 14 09                 	mov	byte ptr [rcx + r9], r10b
1408d11ae: 46 88 54 01 10              	mov	byte ptr [rcx + r8 + 0x10], r10b
1408d11b3: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d11ba: 48 ff 8a a0 3e 00 00        	dec	qword ptr [rdx + 0x3ea0]
1408d11c1: 8b 48 f8                    	mov	ecx, dword ptr [rax - 0x8]
1408d11c4: 89 8d 00 01 00 00           	mov	dword ptr [rbp + 0x100], ecx
1408d11ca: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d11cf: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d11d7: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d11de: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d11e5: e8 a6 83 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d11ea: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d11f1: 0f 84 42 ff ff ff           	je	0x1408d1139 <.text+0x8d0139>
1408d11f7: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d11fe: f0                          	lock
1408d11ff: 48 ff 08                    	dec	qword ptr [rax]
1408d1202: 0f 85 31 ff ff ff           	jne	0x1408d1139 <.text+0x8d0139>
1408d1208: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d120f: e8 0c b4 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d1214: e9 20 ff ff ff              	jmp	0x1408d1139 <.text+0x8d0139>
1408d1219: ba 01 00 00 00              	mov	edx, 0x1
1408d121e: 48 8b 8d 60 02 00 00        	mov	rcx, qword ptr [rbp + 0x260]
1408d1225: 4c 8b 85 28 03 00 00        	mov	r8, qword ptr [rbp + 0x328]
1408d122c: 41 b1 01                    	mov	r9b, 0x1
1408d122f: e8 6c 8a fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d1234: e9 00 ff ff ff              	jmp	0x1408d1139 <.text+0x8d0139>
1408d1239: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1408d1240: 4c 89 ee                    	mov	rsi, r13
1408d1243: 41 bd 00 00 00 00           	mov	r13d, 0x0
1408d1249: 4c 89 f8                    	mov	rax, r15
1408d124c: 48 89 d9                    	mov	rcx, rbx
1408d124f: 48 85 f6                    	test	rsi, rsi
1408d1252: 0f 84 b6 00 00 00           	je	0x1408d130e <.text+0x8d030e>
1408d1258: 48 85 c9                    	test	rcx, rcx
1408d125b: 74 23                       	je	0x1408d1280 <.text+0x8d0280>
1408d125d: 48 8d 79 10                 	lea	rdi, [rcx + 0x10]
1408d1261: 48 3b 8d 20 03 00 00        	cmp	rcx, qword ptr [rbp + 0x320]
1408d1268: 74 16                       	je	0x1408d1280 <.text+0x8d0280>
1408d126a: 49 89 c4                    	mov	r12, rax
1408d126d: 48 89 c8                    	mov	rax, rcx
1408d1270: eb 22                       	jmp	0x1408d1294 <.text+0x8d0294>
1408d1272: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1408d1280: 48 85 c0                    	test	rax, rax
1408d1283: 74 bb                       	je	0x1408d1240 <.text+0x8d0240>
1408d1285: 48 3b 85 10 03 00 00        	cmp	rax, qword ptr [rbp + 0x310]
1408d128c: 74 b2                       	je	0x1408d1240 <.text+0x8d0240>
1408d128e: 4c 8d 60 10                 	lea	r12, [rax + 0x10]
1408d1292: 31 ff                       	xor	edi, edi
1408d1294: 48 ff 06                    	inc	qword ptr [rsi]
1408d1297: 4c 8b 70 08                 	mov	r14, qword ptr [rax + 0x8]
1408d129b: 4c 89 f0                    	mov	rax, r14
1408d129e: 48 c1 e8 20                 	shr	rax, 0x20
1408d12a2: 74 9c                       	je	0x1408d1240 <.text+0x8d0240>
1408d12a4: 4c 89 b5 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], r14
1408d12ab: 48 8b 8d 28 03 00 00        	mov	rcx, qword ptr [rbp + 0x328]
1408d12b2: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d12b9: e8 b2 fa cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d12be: 48 89 bd 58 03 00 00        	mov	qword ptr [rbp + 0x358], rdi
1408d12c5: 4c 89 a5 60 03 00 00        	mov	qword ptr [rbp + 0x360], r12
1408d12cc: 4d 89 fb                    	mov	r11, r15
1408d12cf: 49 89 df                    	mov	r15, rbx
1408d12d2: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d12d9: 48 8b 8a 88 3e 00 00        	mov	rcx, qword ptr [rdx + 0x3e88]
1408d12e0: 48 8b 92 90 3e 00 00        	mov	rdx, qword ptr [rdx + 0x3e90]
1408d12e7: 49 89 d0                    	mov	r8, rdx
1408d12ea: 49 21 c0                    	and	r8, rax
1408d12ed: 48 c1 e8 39                 	shr	rax, 0x39
1408d12f1: 66 0f 6e c0                 	movd	xmm0, eax
1408d12f5: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d12f9: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d12fe: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d1303: 4c 8d 61 e0                 	lea	r12, [rcx - 0x20]
1408d1307: 31 ff                       	xor	edi, edi
1408d1309: e9 72 fd ff ff              	jmp	0x1408d1080 <.text+0x8d0080>
1408d130e: 4c 8b 9d e8 02 00 00        	mov	r11, qword ptr [rbp + 0x2e8]
1408d1315: 49 8b 0b                    	mov	rcx, qword ptr [r11]
1408d1318: 4d 8b 6b 08                 	mov	r13, qword ptr [r11 + 0x8]
1408d131c: 45 8b 43 14                 	mov	r8d, dword ptr [r11 + 0x14]
1408d1320: 4c 8b b9 f8 00 00 00        	mov	r15, qword ptr [rcx + 0xf8]
1408d1327: 48 8b 81 00 01 00 00        	mov	rax, qword ptr [rcx + 0x100]
1408d132e: 4d 8d 34 87                 	lea	r14, [r15 + 4*rax]
1408d1332: 48 89 8d 58 03 00 00        	mov	qword ptr [rbp + 0x358], rcx
1408d1339: 44 0f b6 89 08 01 00 00     	movzx	r9d, byte ptr [rcx + 0x108]
1408d1341: 48 8b 85 b8 04 00 00        	mov	rax, qword ptr [rbp + 0x4b8]
1408d1348: 48 8b 08                    	mov	rcx, qword ptr [rax]
1408d134b: 48 8b 85 b0 04 00 00        	mov	rax, qword ptr [rbp + 0x4b0]
1408d1352: 4c 8b 10                    	mov	r10, qword ptr [rax]
1408d1355: 48 8b 85 a8 04 00 00        	mov	rax, qword ptr [rbp + 0x4a8]
1408d135c: 48 8b 10                    	mov	rdx, qword ptr [rax]
1408d135f: 44 89 c0                    	mov	eax, r8d
1408d1362: 41 2b 43 10                 	sub	eax, dword ptr [r11 + 0x10]
1408d1366: 3d 00 b0 33 c2              	cmp	eax, 0xc233b000
1408d136b: 41 bb 00 b0 33 c2           	mov	r11d, 0xc233b000
1408d1371: 44 0f 42 d8                 	cmovb	r11d, eax
1408d1375: 48 89 95 a8 02 00 00        	mov	qword ptr [rbp + 0x2a8], rdx
1408d137c: 48 8d 42 20                 	lea	rax, [rdx + 0x20]
1408d1380: 48 89 85 68 02 00 00        	mov	qword ptr [rbp + 0x268], rax
1408d1387: 48 8b 95 e0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e0]
1408d138e: 48 8b 02                    	mov	rax, qword ptr [rdx]
1408d1391: 48 89 85 88 00 00 00        	mov	qword ptr [rbp + 0x88], rax
1408d1398: 48 8b 42 08                 	mov	rax, qword ptr [rdx + 0x8]
1408d139c: 48 89 45 28                 	mov	qword ptr [rbp + 0x28], rax
1408d13a0: 66 45 0f 76 d2              	pcmpeqd	xmm10, xmm10
1408d13a5: 66 45 0f ef db              	pxor	xmm11, xmm11
1408d13aa: 44 0f 28 35 0e cd 06 02     	movaps	xmm14, xmmword ptr [rip + 0x206cd0e] # 0x14293e0c0
1408d13b2: 44 0f 28 2d d6 1e 1d 02     	movaps	xmm13, xmmword ptr [rip + 0x21d1ed6] # 0x142aa3290
1408d13ba: 66 0f 6e 35 ae 44 05 02     	movd	xmm6, dword ptr [rip + 0x20544ae] # 0x142925870
1408d13c2: f2 44 0f 10 0d 35 c9 10 02  	movsd	xmm9, qword ptr [rip + 0x210c935] # 0x1429ddd00
1408d13cb: f3 44 0f 10 05 ac 1e 1d 02  	movss	xmm8, dword ptr [rip + 0x21d1eac] # 0x142aa3280
1408d13d4: f3 44 0f 10 25 73 de 1a 02  	movss	xmm12, dword ptr [rip + 0x21ade73] # 0x142a7f250
1408d13dd: f3 44 0f 10 3d 9e 45 05 02  	movss	xmm15, dword ptr [rip + 0x205459e] # 0x142925984
1408d13e6: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d13ed: 48 8d 90 58 3e 00 00        	lea	rdx, [rax + 0x3e58]
1408d13f4: 48 89 95 a0 02 00 00        	mov	qword ptr [rbp + 0x2a0], rdx
1408d13fb: 4c 89 95 90 02 00 00        	mov	qword ptr [rbp + 0x290], r10
1408d1402: 49 8d 52 20                 	lea	rdx, [r10 + 0x20]
1408d1406: 48 89 95 98 01 00 00        	mov	qword ptr [rbp + 0x198], rdx
1408d140d: 48 8d 90 28 3e 00 00        	lea	rdx, [rax + 0x3e28]
1408d1414: 48 89 95 98 02 00 00        	mov	qword ptr [rbp + 0x298], rdx
1408d141b: 48 8d 90 c8 3d 00 00        	lea	rdx, [rax + 0x3dc8]
1408d1422: 48 89 95 f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rdx
1408d1429: 48 8d 51 20                 	lea	rdx, [rcx + 0x20]
1408d142d: 48 89 95 a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rdx
1408d1434: 48 8d 90 78 3e 00 00        	lea	rdx, [rax + 0x3e78]
1408d143b: 48 89 95 08 03 00 00        	mov	qword ptr [rbp + 0x308], rdx
1408d1442: 48 89 8d 60 02 00 00        	mov	qword ptr [rbp + 0x260], rcx
1408d1449: 48 8d 49 50                 	lea	rcx, [rcx + 0x50]
1408d144d: 48 89 8d f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rcx
1408d1454: 48 8d 88 e8 3d 00 00        	lea	rcx, [rax + 0x3de8]
1408d145b: 48 89 8d e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rcx
1408d1462: 48 05 48 3e 00 00           	add	rax, 0x3e48
1408d1468: 48 89 85 28 03 00 00        	mov	qword ptr [rbp + 0x328], rax
1408d146f: b8 08 00 00 00              	mov	eax, 0x8
1408d1474: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1408d147b: 48 c7 85 60 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x360], 0x0
1408d1486: be 08 00 00 00              	mov	esi, 0x8
1408d148b: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
1408d1496: 45 31 e4                    	xor	r12d, r12d
1408d1499: 31 ff                       	xor	edi, edi
1408d149b: 31 db                       	xor	ebx, ebx
1408d149d: 44 89 85 10 03 00 00        	mov	dword ptr [rbp + 0x310], r8d
1408d14a4: 44 88 8d 90 01 00 00        	mov	byte ptr [rbp + 0x190], r9b
1408d14ab: 44 89 9d e8 02 00 00        	mov	dword ptr [rbp + 0x2e8], r11d
1408d14b2: 45 84 c9                    	test	r9b, r9b
1408d14b5: 0f 84 95 01 00 00           	je	0x1408d1650 <.text+0x8d0650>
1408d14bb: 48 89 b5 e8 00 00 00        	mov	qword ptr [rbp + 0xe8], rsi
1408d14c2: 48 39 fb                    	cmp	rbx, rdi
1408d14c5: 0f 85 60 01 00 00           	jne	0x1408d162b <.text+0x8d062b>
1408d14cb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1408d14d0: 4d 39 f7                    	cmp	r15, r14
1408d14d3: 4c 8b 85 58 03 00 00        	mov	r8, qword ptr [rbp + 0x358]
1408d14da: 0f 84 cb 29 00 00           	je	0x1408d3eab <.text+0x8d2eab>
1408d14e0: 41 8b 0f                    	mov	ecx, dword ptr [r15]
1408d14e3: 49 83 c7 04                 	add	r15, 0x4
1408d14e7: 49 8b 85 a8 01 00 00        	mov	rax, qword ptr [r13 + 0x1a8]
1408d14ee: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1408d14f2: 48 83 7c c8 10 00           	cmp	qword ptr [rax + 8*rcx + 0x10], 0x0
1408d14f8: 74 26                       	je	0x1408d1520 <.text+0x8d0520>
1408d14fa: 48 8d 3c c8                 	lea	rdi, [rax + 8*rcx]
1408d14fe: 48 8d 34 c8                 	lea	rsi, [rax + 8*rcx]
1408d1502: 48 83 c6 10                 	add	rsi, 0x10
1408d1506: 49 8b 88 10 01 00 00        	mov	rcx, qword ptr [r8 + 0x110]
1408d150d: 48 8b 47 40                 	mov	rax, qword ptr [rdi + 0x40]
1408d1511: 48 39 c8                    	cmp	rax, rcx
1408d1514: 0f 86 86 00 00 00           	jbe	0x1408d15a0 <.text+0x8d05a0>
1408d151a: eb 3f                       	jmp	0x1408d155b <.text+0x8d055b>
1408d151c: 0f 1f 40 00                 	nop	dword ptr [rax]
1408d1520: 4d 39 f7                    	cmp	r15, r14
1408d1523: 0f 84 82 29 00 00           	je	0x1408d3eab <.text+0x8d2eab>
1408d1529: 41 8b 0f                    	mov	ecx, dword ptr [r15]
1408d152c: 49 83 c7 04                 	add	r15, 0x4
1408d1530: 49 8b 85 a8 01 00 00        	mov	rax, qword ptr [r13 + 0x1a8]
1408d1537: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1408d153b: 48 83 7c c8 10 00           	cmp	qword ptr [rax + 8*rcx + 0x10], 0x0
1408d1541: 74 dd                       	je	0x1408d1520 <.text+0x8d0520>
1408d1543: 48 8d 3c c8                 	lea	rdi, [rax + 8*rcx]
1408d1547: 48 8d 77 10                 	lea	rsi, [rdi + 0x10]
1408d154b: 49 8b 88 10 01 00 00        	mov	rcx, qword ptr [r8 + 0x110]
1408d1552: 48 8b 47 40                 	mov	rax, qword ptr [rdi + 0x40]
1408d1556: 48 39 c8                    	cmp	rax, rcx
1408d1559: 76 45                       	jbe	0x1408d15a0 <.text+0x8d05a0>
1408d155b: 48 8b 57 38                 	mov	rdx, qword ptr [rdi + 0x38]
1408d155f: 48 8b 0c ca                 	mov	rcx, qword ptr [rdx + 8*rcx]
1408d1563: 48 85 c9                    	test	rcx, rcx
1408d1566: 74 38                       	je	0x1408d15a0 <.text+0x8d05a0>
1408d1568: 48 8b 57 18                 	mov	rdx, qword ptr [rdi + 0x18]
1408d156c: 48 f7 d1                    	not	rcx
1408d156f: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1408d1573: 48 c1 e1 04                 	shl	rcx, 0x4
1408d1577: 48 8b 4c 0a 10              	mov	rcx, qword ptr [rdx + rcx + 0x10]
1408d157c: 48 89 8d 60 03 00 00        	mov	qword ptr [rbp + 0x360], rcx
1408d1583: 49 8b 88 18 01 00 00        	mov	rcx, qword ptr [r8 + 0x118]
1408d158a: 48 39 c8                    	cmp	rax, rcx
1408d158d: 77 28                       	ja	0x1408d15b7 <.text+0x8d05b7>
1408d158f: eb 5f                       	jmp	0x1408d15f0 <.text+0x8d05f0>
1408d1591: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1408d15a0: 48 c7 85 60 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x360], 0x0
1408d15ab: 49 8b 88 18 01 00 00        	mov	rcx, qword ptr [r8 + 0x118]
1408d15b2: 48 39 c8                    	cmp	rax, rcx
1408d15b5: 76 39                       	jbe	0x1408d15f0 <.text+0x8d05f0>
1408d15b7: 48 8b 47 38                 	mov	rax, qword ptr [rdi + 0x38]
1408d15bb: 48 8b 04 c8                 	mov	rax, qword ptr [rax + 8*rcx]
1408d15bf: 48 85 c0                    	test	rax, rax
1408d15c2: 74 2c                       	je	0x1408d15f0 <.text+0x8d05f0>
1408d15c4: 48 8b 4f 18                 	mov	rcx, qword ptr [rdi + 0x18]
1408d15c8: 48 f7 d0                    	not	rax
1408d15cb: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1408d15cf: 48 c1 e0 04                 	shl	rax, 0x4
1408d15d3: 48 8b 44 01 10              	mov	rax, qword ptr [rcx + rax + 0x10]
1408d15d8: 48 89 85 20 03 00 00        	mov	qword ptr [rbp + 0x320], rax
1408d15df: eb 1a                       	jmp	0x1408d15fb <.text+0x8d05fb>
1408d15e1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1408d15f0: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
1408d15fb: 49 8b 90 20 01 00 00        	mov	rdx, qword ptr [r8 + 0x120]
1408d1602: 48 89 f9                    	mov	rcx, rdi
1408d1605: e8 66 f1 42 00              	call	0x140d00770 <.text+0xcff770>
1408d160a: 49 89 c4                    	mov	r12, rax
1408d160d: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1408d1611: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1408d1618: 48 8b 3e                    	mov	rdi, qword ptr [rsi]
1408d161b: 31 db                       	xor	ebx, ebx
1408d161d: 44 8b 85 10 03 00 00        	mov	r8d, dword ptr [rbp + 0x310]
1408d1624: 44 8b 9d e8 02 00 00        	mov	r11d, dword ptr [rbp + 0x2e8]
1408d162b: 89 d9                       	mov	ecx, ebx
1408d162d: 44 89 c0                    	mov	eax, r8d
1408d1630: 41 2b 04 8c                 	sub	eax, dword ptr [r12 + 4*rcx]
1408d1634: 44 39 d8                    	cmp	eax, r11d
1408d1637: 0f 82 93 01 00 00           	jb	0x1408d17d0 <.text+0x8d07d0>
1408d163d: 48 ff c3                    	inc	rbx
1408d1640: 48 39 fb                    	cmp	rbx, rdi
1408d1643: 0f 84 87 fe ff ff           	je	0x1408d14d0 <.text+0x8d04d0>
1408d1649: eb e0                       	jmp	0x1408d162b <.text+0x8d062b>
1408d164b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1408d1650: 48 39 fb                    	cmp	rbx, rdi
1408d1653: 0f 85 58 01 00 00           	jne	0x1408d17b1 <.text+0x8d07b1>
1408d1659: 4d 39 f7                    	cmp	r15, r14
1408d165c: 4c 8b 8d 58 03 00 00        	mov	r9, qword ptr [rbp + 0x358]
1408d1663: 0f 84 42 28 00 00           	je	0x1408d3eab <.text+0x8d2eab>
1408d1669: 41 8b 0f                    	mov	ecx, dword ptr [r15]
1408d166c: 49 83 c7 04                 	add	r15, 0x4
1408d1670: 49 8b 85 00 01 00 00        	mov	rax, qword ptr [r13 + 0x100]
1408d1677: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
1408d167b: 48 c1 e1 05                 	shl	rcx, 0x5
1408d167f: 48 83 7c 08 58 00           	cmp	qword ptr [rax + rcx + 0x58], 0x0
1408d1685: 74 19                       	je	0x1408d16a0 <.text+0x8d06a0>
1408d1687: 48 8d 34 08                 	lea	rsi, [rax + rcx]
1408d168b: 48 8d 3c 08                 	lea	rdi, [rax + rcx]
1408d168f: 48 83 c7 58                 	add	rdi, 0x58
1408d1693: eb 39                       	jmp	0x1408d16ce <.text+0x8d06ce>
1408d1695: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1408d16a0: 4d 39 f7                    	cmp	r15, r14
1408d16a3: 0f 84 02 28 00 00           	je	0x1408d3eab <.text+0x8d2eab>
1408d16a9: 41 8b 07                    	mov	eax, dword ptr [r15]
1408d16ac: 49 83 c7 04                 	add	r15, 0x4
1408d16b0: 49 8b b5 00 01 00 00        	mov	rsi, qword ptr [r13 + 0x100]
1408d16b7: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
1408d16bb: 48 c1 e0 05                 	shl	rax, 0x5
1408d16bf: 48 83 7c 06 58 00           	cmp	qword ptr [rsi + rax + 0x58], 0x0
1408d16c5: 74 d9                       	je	0x1408d16a0 <.text+0x8d06a0>
1408d16c7: 48 01 c6                    	add	rsi, rax
1408d16ca: 48 8d 7e 58                 	lea	rdi, [rsi + 0x58]
1408d16ce: 8b 86 94 00 00 00           	mov	eax, dword ptr [rsi + 0x94]
1408d16d4: 4d 8b 85 a8 01 00 00        	mov	r8, qword ptr [r13 + 0x1a8]
1408d16db: 48 8d 04 c0                 	lea	rax, [rax + 8*rax]
1408d16df: 49 8d 0c c0                 	lea	rcx, [r8 + 8*rax]
1408d16e3: 49 8b 91 10 01 00 00        	mov	rdx, qword ptr [r9 + 0x110]
1408d16ea: 49 8b 44 c0 40              	mov	rax, qword ptr [r8 + 8*rax + 0x40]
1408d16ef: 48 39 d0                    	cmp	rax, rdx
1408d16f2: 76 3c                       	jbe	0x1408d1730 <.text+0x8d0730>
1408d16f4: 4c 8b 41 38                 	mov	r8, qword ptr [rcx + 0x38]
1408d16f8: 49 8b 14 d0                 	mov	rdx, qword ptr [r8 + 8*rdx]
1408d16fc: 48 85 d2                    	test	rdx, rdx
1408d16ff: 74 2f                       	je	0x1408d1730 <.text+0x8d0730>
1408d1701: 4c 8b 41 18                 	mov	r8, qword ptr [rcx + 0x18]
1408d1705: 48 f7 d2                    	not	rdx
1408d1708: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
1408d170c: 48 c1 e2 04                 	shl	rdx, 0x4
1408d1710: 49 8b 54 10 10              	mov	rdx, qword ptr [r8 + rdx + 0x10]
1408d1715: 48 89 95 60 03 00 00        	mov	qword ptr [rbp + 0x360], rdx
1408d171c: 49 8b 91 18 01 00 00        	mov	rdx, qword ptr [r9 + 0x118]
1408d1723: 48 39 d0                    	cmp	rax, rdx
1408d1726: 77 1f                       	ja	0x1408d1747 <.text+0x8d0747>
1408d1728: eb 56                       	jmp	0x1408d1780 <.text+0x8d0780>
1408d172a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1408d1730: 48 c7 85 60 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x360], 0x0
1408d173b: 49 8b 91 18 01 00 00        	mov	rdx, qword ptr [r9 + 0x118]
1408d1742: 48 39 d0                    	cmp	rax, rdx
1408d1745: 76 39                       	jbe	0x1408d1780 <.text+0x8d0780>
1408d1747: 48 8b 41 38                 	mov	rax, qword ptr [rcx + 0x38]
1408d174b: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1408d174f: 48 85 c0                    	test	rax, rax
1408d1752: 74 2c                       	je	0x1408d1780 <.text+0x8d0780>
1408d1754: 48 8b 51 18                 	mov	rdx, qword ptr [rcx + 0x18]
1408d1758: 48 f7 d0                    	not	rax
1408d175b: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1408d175f: 48 c1 e0 04                 	shl	rax, 0x4
1408d1763: 48 8b 44 02 10              	mov	rax, qword ptr [rdx + rax + 0x10]
1408d1768: 48 89 85 20 03 00 00        	mov	qword ptr [rbp + 0x320], rax
1408d176f: eb 1a                       	jmp	0x1408d178b <.text+0x8d078b>
1408d1771: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1408d1780: 48 c7 85 20 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x320], 0x0
1408d178b: 49 8b 91 20 01 00 00        	mov	rdx, qword ptr [r9 + 0x120]
1408d1792: e8 d9 ef 42 00              	call	0x140d00770 <.text+0xcff770>
1408d1797: 49 89 c4                    	mov	r12, rax
1408d179a: 48 8b 76 50                 	mov	rsi, qword ptr [rsi + 0x50]
1408d179e: 48 8b 3f                    	mov	rdi, qword ptr [rdi]
1408d17a1: 31 db                       	xor	ebx, ebx
1408d17a3: 44 8b 85 10 03 00 00        	mov	r8d, dword ptr [rbp + 0x310]
1408d17aa: 44 8b 9d e8 02 00 00        	mov	r11d, dword ptr [rbp + 0x2e8]
1408d17b1: 48 89 d8                    	mov	rax, rbx
1408d17b4: 48 c1 e0 04                 	shl	rax, 0x4
1408d17b8: 8b 4c 06 08                 	mov	ecx, dword ptr [rsi + rax + 0x8]
1408d17bc: 44 89 c2                    	mov	edx, r8d
1408d17bf: 41 2b 14 8c                 	sub	edx, dword ptr [r12 + 4*rcx]
1408d17c3: 44 39 da                    	cmp	edx, r11d
1408d17c6: 72 1c                       	jb	0x1408d17e4 <.text+0x8d07e4>
1408d17c8: 48 ff c3                    	inc	rbx
1408d17cb: e9 80 fe ff ff              	jmp	0x1408d1650 <.text+0x8d0650>
1408d17d0: 48 89 bd 00 03 00 00        	mov	qword ptr [rbp + 0x300], rdi
1408d17d7: 48 8b 85 58 02 00 00        	mov	rax, qword ptr [rbp + 0x258]
1408d17de: 48 8d 04 d8                 	lea	rax, [rax + 8*rbx]
1408d17e2: eb 11                       	jmp	0x1408d17f5 <.text+0x8d07f5>
1408d17e4: 48 89 bd 00 03 00 00        	mov	qword ptr [rbp + 0x300], rdi
1408d17eb: 48 89 b5 e8 00 00 00        	mov	qword ptr [rbp + 0xe8], rsi
1408d17f2: 48 01 f0                    	add	rax, rsi
1408d17f5: 48 6b d1 68                 	imul	rdx, rcx, 0x68
1408d17f9: 48 03 95 60 03 00 00        	add	rdx, qword ptr [rbp + 0x360]
1408d1800: 48 89 95 a8 01 00 00        	mov	qword ptr [rbp + 0x1a8], rdx
1408d1807: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1408d180b: 48 c1 e1 04                 	shl	rcx, 0x4
1408d180f: 48 03 8d 20 03 00 00        	add	rcx, qword ptr [rbp + 0x320]
1408d1816: 48 8b 00                    	mov	rax, qword ptr [rax]
1408d1819: 48 89 85 30 03 00 00        	mov	qword ptr [rbp + 0x330], rax
1408d1820: 48 89 8d 38 03 00 00        	mov	qword ptr [rbp + 0x338], rcx
1408d1827: 48 8b 39                    	mov	rdi, qword ptr [rcx]
1408d182a: 48 89 bd b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rdi
1408d1831: 48 8b b5 60 02 00 00        	mov	rsi, qword ptr [rbp + 0x260]
1408d1838: 48 83 7e 18 00              	cmp	qword ptr [rsi + 0x18], 0x0
1408d183d: 0f 84 f4 43 00 00           	je	0x1408d5c37 <.text+0x8d4c37>
1408d1843: 48 ff c3                    	inc	rbx
1408d1846: 48 8b 8d a8 00 00 00        	mov	rcx, qword ptr [rbp + 0xa8]
1408d184d: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d1854: e8 17 f5 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d1859: 4c 8b 0e                    	mov	r9, qword ptr [rsi]
1408d185c: 48 8b 76 08                 	mov	rsi, qword ptr [rsi + 0x8]
1408d1860: 48 89 f1                    	mov	rcx, rsi
1408d1863: 48 21 c1                    	and	rcx, rax
1408d1866: 48 c1 e8 39                 	shr	rax, 0x39
1408d186a: 66 0f 6e c0                 	movd	xmm0, eax
1408d186e: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d1872: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d1877: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d187c: 4d 8d 91 58 fd ff ff        	lea	r10, [r9 - 0x2a8]
1408d1883: 31 c0                       	xor	eax, eax
1408d1885: f3 41 0f 6f 0c 09           	movdqu	xmm1, xmmword ptr [r9 + rcx]
1408d188b: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d188f: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d1893: 66 0f d7 d2                 	pmovmskb	edx, xmm2
1408d1897: 85 d2                       	test	edx, edx
1408d1899: 74 28                       	je	0x1408d18c3 <.text+0x8d08c3>
1408d189b: f3 44 0f bc c2              	tzcnt	r8d, edx
1408d18a0: 49 01 c8                    	add	r8, rcx
1408d18a3: 49 21 f0                    	and	r8, rsi
1408d18a6: 49 f7 d8                    	neg	r8
1408d18a9: 4d 69 c0 a8 02 00 00        	imul	r8, r8, 0x2a8
1408d18b0: 4b 39 3c 02                 	cmp	qword ptr [r10 + r8], rdi
1408d18b4: 74 2e                       	je	0x1408d18e4 <.text+0x8d08e4>
1408d18b6: 44 8d 42 ff                 	lea	r8d, [rdx - 0x1]
1408d18ba: 66 41 21 d0                 	and	r8w, dx
1408d18be: 44 89 c2                    	mov	edx, r8d
1408d18c1: 75 d8                       	jne	0x1408d189b <.text+0x8d089b>
1408d18c3: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d18c8: 66 0f d7 d1                 	pmovmskb	edx, xmm1
1408d18cc: 85 d2                       	test	edx, edx
1408d18ce: 0f 85 63 43 00 00           	jne	0x1408d5c37 <.text+0x8d4c37>
1408d18d4: 48 01 c1                    	add	rcx, rax
1408d18d7: 48 83 c1 10                 	add	rcx, 0x10
1408d18db: 48 83 c0 10                 	add	rax, 0x10
1408d18df: 48 21 f1                    	and	rcx, rsi
1408d18e2: eb a1                       	jmp	0x1408d1885 <.text+0x8d0885>
1408d18e4: 4b 83 bc 01 70 fd ff ff 02  	cmp	qword ptr [r9 + r8 - 0x290], 0x2
1408d18ed: 0f 82 b0 43 00 00           	jb	0x1408d5ca3 <.text+0x8d4ca3>
1408d18f3: f3 43 0f 10 84 01 90 fd ff ff       	movss	xmm0, dword ptr [r9 + r8 - 0x270]
1408d18fd: 41 0f 2e c3                 	ucomiss	xmm0, xmm11
1408d1901: 0f 86 9c 43 00 00           	jbe	0x1408d5ca3 <.text+0x8d4ca3>
1408d1907: 4d 01 c8                    	add	r8, r9
1408d190a: 41 80 78 f7 01              	cmp	byte ptr [r8 - 0x9], 0x1
1408d190f: 0f 85 e5 00 00 00           	jne	0x1408d19fa <.text+0x8d09fa>
1408d1915: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1408d191c: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
1408d1923: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1408d192a: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d1931: e8 3a f4 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d1936: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d193d: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d1944: 4c 8b 9a c8 3d 00 00        	mov	r11, qword ptr [rdx + 0x3dc8]
1408d194b: 4c 8b 92 d0 3d 00 00        	mov	r10, qword ptr [rdx + 0x3dd0]
1408d1952: 4d 89 d0                    	mov	r8, r10
1408d1955: 49 21 c0                    	and	r8, rax
1408d1958: 48 c1 e8 39                 	shr	rax, 0x39
1408d195c: 66 0f 6e c0                 	movd	xmm0, eax
1408d1960: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d1964: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d1969: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d196e: 4d 8d 4b e0                 	lea	r9, [r11 - 0x20]
1408d1972: 31 c9                       	xor	ecx, ecx
1408d1974: f3 43 0f 6f 0c 03           	movdqu	xmm1, xmmword ptr [r11 + r8]
1408d197a: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d197e: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d1982: 66 0f d7 f2                 	pmovmskb	esi, xmm2
1408d1986: 85 f6                       	test	esi, esi
1408d1988: 74 4c                       	je	0x1408d19d6 <.text+0x8d09d6>
1408d198a: 48 89 8d 38 03 00 00        	mov	qword ptr [rbp + 0x338], rcx
1408d1991: f3 0f bc c6                 	tzcnt	eax, esi
1408d1995: 4c 01 c0                    	add	rax, r8
1408d1998: 4c 21 d0                    	and	rax, r10
1408d199b: 4c 89 ca                    	mov	rdx, r9
1408d199e: 49 89 c1                    	mov	r9, rax
1408d19a1: 49 c1 e1 05                 	shl	r9, 0x5
1408d19a5: 48 89 d7                    	mov	rdi, rdx
1408d19a8: 4c 29 cf                    	sub	rdi, r9
1408d19ab: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1408d19b2: 48 39 0f                    	cmp	qword ptr [rdi], rcx
1408d19b5: 0f 84 2b 02 00 00           	je	0x1408d1be6 <.text+0x8d0be6>
1408d19bb: 8d 46 ff                    	lea	eax, [rsi - 0x1]
1408d19be: 66 21 f0                    	and	ax, si
1408d19c1: 89 c6                       	mov	esi, eax
1408d19c3: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d19ca: 48 8b 8d 38 03 00 00        	mov	rcx, qword ptr [rbp + 0x338]
1408d19d1: 49 89 d1                    	mov	r9, rdx
1408d19d4: 75 bb                       	jne	0x1408d1991 <.text+0x8d0991>
1408d19d6: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d19db: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d19df: 85 c0                       	test	eax, eax
1408d19e1: 0f 85 34 02 00 00           	jne	0x1408d1c1b <.text+0x8d0c1b>
1408d19e7: 49 01 c8                    	add	r8, rcx
1408d19ea: 49 83 c0 10                 	add	r8, 0x10
1408d19ee: 48 83 c1 10                 	add	rcx, 0x10
1408d19f2: 4d 21 d0                    	and	r8, r10
1408d19f5: e9 7a ff ff ff              	jmp	0x1408d1974 <.text+0x8d0974>
1408d19fa: 4c 89 95 40 03 00 00        	mov	qword ptr [rbp + 0x340], r10
1408d1a01: 4c 89 8d 90 00 00 00        	mov	qword ptr [rbp + 0x90], r9
1408d1a08: 48 8b 85 38 03 00 00        	mov	rax, qword ptr [rbp + 0x338]
1408d1a0f: 48 8b 38                    	mov	rdi, qword ptr [rax]
1408d1a12: 48 89 bd b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rdi
1408d1a19: 48 8b 8d a8 00 00 00        	mov	rcx, qword ptr [rbp + 0xa8]
1408d1a20: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d1a27: e8 44 f3 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d1a2c: 48 89 f1                    	mov	rcx, rsi
1408d1a2f: 48 21 c1                    	and	rcx, rax
1408d1a32: 48 c1 e8 39                 	shr	rax, 0x39
1408d1a36: 66 0f 6e c0                 	movd	xmm0, eax
1408d1a3a: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d1a3e: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d1a43: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d1a48: 31 c0                       	xor	eax, eax
1408d1a4a: 4c 8b 8d 90 00 00 00        	mov	r9, qword ptr [rbp + 0x90]
1408d1a51: f3 41 0f 6f 0c 09           	movdqu	xmm1, xmmword ptr [r9 + rcx]
1408d1a57: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d1a5b: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d1a5f: 66 0f d7 d2                 	pmovmskb	edx, xmm2
1408d1a63: 85 d2                       	test	edx, edx
1408d1a65: 4c 8b 95 40 03 00 00        	mov	r10, qword ptr [rbp + 0x340]
1408d1a6c: 74 28                       	je	0x1408d1a96 <.text+0x8d0a96>
1408d1a6e: f3 44 0f bc c2              	tzcnt	r8d, edx
1408d1a73: 49 01 c8                    	add	r8, rcx
1408d1a76: 49 21 f0                    	and	r8, rsi
1408d1a79: 49 f7 d8                    	neg	r8
1408d1a7c: 4d 69 c0 a8 02 00 00        	imul	r8, r8, 0x2a8
1408d1a83: 4b 39 3c 02                 	cmp	qword ptr [r10 + r8], rdi
1408d1a87: 74 2e                       	je	0x1408d1ab7 <.text+0x8d0ab7>
1408d1a89: 44 8d 42 ff                 	lea	r8d, [rdx - 0x1]
1408d1a8d: 66 41 21 d0                 	and	r8w, dx
1408d1a91: 44 89 c2                    	mov	edx, r8d
1408d1a94: 75 d8                       	jne	0x1408d1a6e <.text+0x8d0a6e>
1408d1a96: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d1a9b: 66 0f d7 d1                 	pmovmskb	edx, xmm1
1408d1a9f: 85 d2                       	test	edx, edx
1408d1aa1: 0f 85 df 41 00 00           	jne	0x1408d5c86 <.text+0x8d4c86>
1408d1aa7: 48 01 c1                    	add	rcx, rax
1408d1aaa: 48 83 c1 10                 	add	rcx, 0x10
1408d1aae: 48 83 c0 10                 	add	rax, 0x10
1408d1ab2: 48 21 f1                    	and	rcx, rsi
1408d1ab5: eb 93                       	jmp	0x1408d1a4a <.text+0x8d0a4a>
1408d1ab7: 4b 83 bc 01 70 fd ff ff 02  	cmp	qword ptr [r9 + r8 - 0x290], 0x2
1408d1ac0: 0f 82 17 42 00 00           	jb	0x1408d5cdd <.text+0x8d4cdd>
1408d1ac6: f3 43 0f 10 84 01 90 fd ff ff       	movss	xmm0, dword ptr [r9 + r8 - 0x270]
1408d1ad0: 41 0f 2e c3                 	ucomiss	xmm0, xmm11
1408d1ad4: 0f 86 03 42 00 00           	jbe	0x1408d5cdd <.text+0x8d4cdd>
1408d1ada: 4d 01 c1                    	add	r9, r8
1408d1add: 41 8b 41 b8                 	mov	eax, dword ptr [r9 - 0x48]
1408d1ae1: 89 85 84 00 00 00           	mov	dword ptr [rbp + 0x84], eax
1408d1ae7: 4c 89 8d 90 00 00 00        	mov	qword ptr [rbp + 0x90], r9
1408d1aee: 41 0f b6 41 f6              	movzx	eax, byte ptr [r9 - 0xa]
1408d1af3: 88 85 df 02 00 00           	mov	byte ptr [rbp + 0x2df], al
1408d1af9: 3c 01                       	cmp	al, 0x1
1408d1afb: 48 8b b5 30 03 00 00        	mov	rsi, qword ptr [rbp + 0x330]
1408d1b02: 0f 85 48 01 00 00           	jne	0x1408d1c50 <.text+0x8d0c50>
1408d1b08: 48 8b 85 90 02 00 00        	mov	rax, qword ptr [rbp + 0x290]
1408d1b0f: 48 83 78 18 00              	cmp	qword ptr [rax + 0x18], 0x0
1408d1b14: 0f 84 ab 0c 00 00           	je	0x1408d27c5 <.text+0x8d17c5>
1408d1b1a: 48 8b 8d 98 01 00 00        	mov	rcx, qword ptr [rbp + 0x198]
1408d1b21: 48 8b 95 38 03 00 00        	mov	rdx, qword ptr [rbp + 0x338]
1408d1b28: e8 43 f2 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d1b2d: 48 8b 95 90 02 00 00        	mov	rdx, qword ptr [rbp + 0x290]
1408d1b34: 48 8b 0a                    	mov	rcx, qword ptr [rdx]
1408d1b37: 4c 8b 5a 08                 	mov	r11, qword ptr [rdx + 0x8]
1408d1b3b: 4d 89 d8                    	mov	r8, r11
1408d1b3e: 49 21 c0                    	and	r8, rax
1408d1b41: 48 c1 e8 39                 	shr	rax, 0x39
1408d1b45: 66 0f 6e c0                 	movd	xmm0, eax
1408d1b49: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d1b4d: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d1b52: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d1b57: 4c 8d 49 e0                 	lea	r9, [rcx - 0x20]
1408d1b5b: 31 ff                       	xor	edi, edi
1408d1b5d: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d1b63: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d1b67: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d1b6b: 66 44 0f d7 d2              	pmovmskb	r10d, xmm2
1408d1b70: 45 85 d2                    	test	r10d, r10d
1408d1b73: 74 4d                       	je	0x1408d1bc2 <.text+0x8d0bc2>
1408d1b75: 48 8b 85 38 03 00 00        	mov	rax, qword ptr [rbp + 0x338]
1408d1b7c: 48 8b 10                    	mov	rdx, qword ptr [rax]
1408d1b7f: 48 89 bd 40 03 00 00        	mov	qword ptr [rbp + 0x340], rdi
1408d1b86: f3 41 0f bc f2              	tzcnt	esi, r10d
1408d1b8b: 4c 01 c6                    	add	rsi, r8
1408d1b8e: 4c 21 de                    	and	rsi, r11
1408d1b91: 48 89 f7                    	mov	rdi, rsi
1408d1b94: 48 c1 e7 05                 	shl	rdi, 0x5
1408d1b98: 4c 89 c8                    	mov	rax, r9
1408d1b9b: 48 29 f8                    	sub	rax, rdi
1408d1b9e: 48 3b 10                    	cmp	rdx, qword ptr [rax]
1408d1ba1: 0f 84 01 06 00 00           	je	0x1408d21a8 <.text+0x8d11a8>
1408d1ba7: 41 8d 42 ff                 	lea	eax, [r10 - 0x1]
1408d1bab: 66 44 21 d0                 	and	ax, r10w
1408d1baf: 41 89 c2                    	mov	r10d, eax
1408d1bb2: 48 8b b5 30 03 00 00        	mov	rsi, qword ptr [rbp + 0x330]
1408d1bb9: 48 8b bd 40 03 00 00        	mov	rdi, qword ptr [rbp + 0x340]
1408d1bc0: 75 c4                       	jne	0x1408d1b86 <.text+0x8d0b86>
1408d1bc2: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d1bc7: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d1bcb: 85 c0                       	test	eax, eax
1408d1bcd: 0f 85 f2 0b 00 00           	jne	0x1408d27c5 <.text+0x8d17c5>
1408d1bd3: 49 01 f8                    	add	r8, rdi
1408d1bd6: 49 83 c0 10                 	add	r8, 0x10
1408d1bda: 48 83 c7 10                 	add	rdi, 0x10
1408d1bde: 4d 21 d8                    	and	r8, r11
1408d1be1: e9 77 ff ff ff              	jmp	0x1408d1b5d <.text+0x8d0b5d>
1408d1be6: 49 c1 f9 05                 	sar	r9, 0x5
1408d1bea: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d1bee: 4d 21 d0                    	and	r8, r10
1408d1bf1: f3 43 0f 6f 04 03           	movdqu	xmm0, xmmword ptr [r11 + r8]
1408d1bf7: 66 41 0f 74 c2              	pcmpeqb	xmm0, xmm10
1408d1bfc: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d1c00: 85 d2                       	test	edx, edx
1408d1c02: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d1c09: 0f 84 1f 01 00 00           	je	0x1408d1d2e <.text+0x8d0d2e>
1408d1c0f: 66 0f bd d2                 	bsr	dx, dx
1408d1c13: 83 f2 0f                    	xor	edx, 0xf
1408d1c16: e9 17 01 00 00              	jmp	0x1408d1d32 <.text+0x8d0d32>
1408d1c1b: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d1c22: 48 83 b8 d8 3d 00 00 00     	cmp	qword ptr [rax + 0x3dd8], 0x0
1408d1c2a: 0f 85 a9 01 00 00           	jne	0x1408d1dd9 <.text+0x8d0dd9>
1408d1c30: ba 01 00 00 00              	mov	edx, 0x1
1408d1c35: 48 8b 8d f0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f0]
1408d1c3c: 4c 8b 85 e0 02 00 00        	mov	r8, qword ptr [rbp + 0x2e0]
1408d1c43: 41 b1 01                    	mov	r9b, 0x1
1408d1c46: e8 55 80 fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d1c4b: e9 89 01 00 00              	jmp	0x1408d1dd9 <.text+0x8d0dd9>
1408d1c50: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
1408d1c57: 48 83 78 18 00              	cmp	qword ptr [rax + 0x18], 0x0
1408d1c5c: 0f 84 63 0b 00 00           	je	0x1408d27c5 <.text+0x8d17c5>
1408d1c62: 48 8b 8d 68 02 00 00        	mov	rcx, qword ptr [rbp + 0x268]
1408d1c69: 48 8b 95 38 03 00 00        	mov	rdx, qword ptr [rbp + 0x338]
1408d1c70: e8 fb f0 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d1c75: 48 8b 95 a8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2a8]
1408d1c7c: 48 8b 0a                    	mov	rcx, qword ptr [rdx]
1408d1c7f: 4c 8b 5a 08                 	mov	r11, qword ptr [rdx + 0x8]
1408d1c83: 4d 89 d8                    	mov	r8, r11
1408d1c86: 49 21 c0                    	and	r8, rax
1408d1c89: 48 c1 e8 39                 	shr	rax, 0x39
1408d1c8d: 66 0f 6e c0                 	movd	xmm0, eax
1408d1c91: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d1c95: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d1c9a: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d1c9f: 4c 8d 49 e0                 	lea	r9, [rcx - 0x20]
1408d1ca3: 31 ff                       	xor	edi, edi
1408d1ca5: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d1cab: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d1caf: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d1cb3: 66 44 0f d7 d2              	pmovmskb	r10d, xmm2
1408d1cb8: 45 85 d2                    	test	r10d, r10d
1408d1cbb: 74 4d                       	je	0x1408d1d0a <.text+0x8d0d0a>
1408d1cbd: 48 8b 85 38 03 00 00        	mov	rax, qword ptr [rbp + 0x338]
1408d1cc4: 48 8b 10                    	mov	rdx, qword ptr [rax]
1408d1cc7: 48 89 bd 40 03 00 00        	mov	qword ptr [rbp + 0x340], rdi
1408d1cce: f3 41 0f bc f2              	tzcnt	esi, r10d
1408d1cd3: 4c 01 c6                    	add	rsi, r8
1408d1cd6: 4c 21 de                    	and	rsi, r11
1408d1cd9: 48 89 f7                    	mov	rdi, rsi
1408d1cdc: 48 c1 e7 05                 	shl	rdi, 0x5
1408d1ce0: 4c 89 c8                    	mov	rax, r9
1408d1ce3: 48 29 f8                    	sub	rax, rdi
1408d1ce6: 48 3b 10                    	cmp	rdx, qword ptr [rax]
1408d1ce9: 0f 84 51 05 00 00           	je	0x1408d2240 <.text+0x8d1240>
1408d1cef: 41 8d 42 ff                 	lea	eax, [r10 - 0x1]
1408d1cf3: 66 44 21 d0                 	and	ax, r10w
1408d1cf7: 41 89 c2                    	mov	r10d, eax
1408d1cfa: 48 8b b5 30 03 00 00        	mov	rsi, qword ptr [rbp + 0x330]
1408d1d01: 48 8b bd 40 03 00 00        	mov	rdi, qword ptr [rbp + 0x340]
1408d1d08: 75 c4                       	jne	0x1408d1cce <.text+0x8d0cce>
1408d1d0a: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d1d0f: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d1d13: 85 c0                       	test	eax, eax
1408d1d15: 0f 85 aa 0a 00 00           	jne	0x1408d27c5 <.text+0x8d17c5>
1408d1d1b: 49 01 f8                    	add	r8, rdi
1408d1d1e: 49 83 c0 10                 	add	r8, 0x10
1408d1d22: 48 83 c7 10                 	add	rdi, 0x10
1408d1d26: 4d 21 d8                    	and	r8, r11
1408d1d29: e9 77 ff ff ff              	jmp	0x1408d1ca5 <.text+0x8d0ca5>
1408d1d2e: 66 ba 10 00                 	mov	dx, 0x10
1408d1d32: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d1d39: f3 43 0f 6f 04 0b           	movdqu	xmm0, xmmword ptr [r11 + r9]
1408d1d3f: 66 41 0f 74 c2              	pcmpeqb	xmm0, xmm10
1408d1d44: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d1d49: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d1d50: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d1d55: 44 01 d2                    	add	edx, r10d
1408d1d58: 41 b2 80                    	mov	r10b, -0x80
1408d1d5b: 66 83 fa 0f                 	cmp	dx, 0xf
1408d1d5f: 77 11                       	ja	0x1408d1d72 <.text+0x8d0d72>
1408d1d61: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d1d68: 48 ff 82 d8 3d 00 00        	inc	qword ptr [rdx + 0x3dd8]
1408d1d6f: 41 b2 ff                    	mov	r10b, -0x1
1408d1d72: 48 f7 d8                    	neg	rax
1408d1d75: 48 c1 e0 05                 	shl	rax, 0x5
1408d1d79: 4c 01 d8                    	add	rax, r11
1408d1d7c: 47 88 14 0b                 	mov	byte ptr [r11 + r9], r10b
1408d1d80: 47 88 54 03 10              	mov	byte ptr [r11 + r8 + 0x10], r10b
1408d1d85: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d1d8c: 48 ff 8a e0 3d 00 00        	dec	qword ptr [rdx + 0x3de0]
1408d1d93: 44 8b 40 f8                 	mov	r8d, dword ptr [rax - 0x8]
1408d1d97: 44 89 85 00 01 00 00        	mov	dword ptr [rbp + 0x100], r8d
1408d1d9e: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d1da3: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d1dab: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d1db2: e8 d9 77 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d1db7: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d1dbe: 74 19                       	je	0x1408d1dd9 <.text+0x8d0dd9>
1408d1dc0: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d1dc7: f0                          	lock
1408d1dc8: 48 ff 08                    	dec	qword ptr [rax]
1408d1dcb: 75 0c                       	jne	0x1408d1dd9 <.text+0x8d0dd9>
1408d1dcd: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d1dd4: e8 47 a8 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d1dd9: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1408d1de0: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
1408d1de7: 48 8b 8d 28 03 00 00        	mov	rcx, qword ptr [rbp + 0x328]
1408d1dee: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d1df5: e8 76 ef cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d1dfa: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d1e01: 4c 8b 9a 28 3e 00 00        	mov	r11, qword ptr [rdx + 0x3e28]
1408d1e08: 4c 8b 92 30 3e 00 00        	mov	r10, qword ptr [rdx + 0x3e30]
1408d1e0f: 4d 89 d0                    	mov	r8, r10
1408d1e12: 49 21 c0                    	and	r8, rax
1408d1e15: 48 c1 e8 39                 	shr	rax, 0x39
1408d1e19: 66 0f 6e c0                 	movd	xmm0, eax
1408d1e1d: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d1e21: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d1e26: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d1e2b: 4d 8d 4b e0                 	lea	r9, [r11 - 0x20]
1408d1e2f: 31 c9                       	xor	ecx, ecx
1408d1e31: f3 43 0f 6f 0c 03           	movdqu	xmm1, xmmword ptr [r11 + r8]
1408d1e37: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d1e3b: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d1e3f: 66 0f d7 f2                 	pmovmskb	esi, xmm2
1408d1e43: 85 f6                       	test	esi, esi
1408d1e45: 74 48                       	je	0x1408d1e8f <.text+0x8d0e8f>
1408d1e47: 48 89 8d 38 03 00 00        	mov	qword ptr [rbp + 0x338], rcx
1408d1e4e: f3 0f bc c6                 	tzcnt	eax, esi
1408d1e52: 4c 01 c0                    	add	rax, r8
1408d1e55: 4c 21 d0                    	and	rax, r10
1408d1e58: 4c 89 ca                    	mov	rdx, r9
1408d1e5b: 49 89 c1                    	mov	r9, rax
1408d1e5e: 49 c1 e1 05                 	shl	r9, 0x5
1408d1e62: 48 89 d7                    	mov	rdi, rdx
1408d1e65: 4c 29 cf                    	sub	rdi, r9
1408d1e68: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1408d1e6f: 48 39 0f                    	cmp	qword ptr [rdi], rcx
1408d1e72: 74 38                       	je	0x1408d1eac <.text+0x8d0eac>
1408d1e74: 8d 46 ff                    	lea	eax, [rsi - 0x1]
1408d1e77: 66 21 f0                    	and	ax, si
1408d1e7a: 89 c6                       	mov	esi, eax
1408d1e7c: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d1e83: 48 8b 8d 38 03 00 00        	mov	rcx, qword ptr [rbp + 0x338]
1408d1e8a: 49 89 d1                    	mov	r9, rdx
1408d1e8d: 75 bf                       	jne	0x1408d1e4e <.text+0x8d0e4e>
1408d1e8f: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d1e94: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d1e98: 85 c0                       	test	eax, eax
1408d1e9a: 75 3e                       	jne	0x1408d1eda <.text+0x8d0eda>
1408d1e9c: 49 01 c8                    	add	r8, rcx
1408d1e9f: 49 83 c0 10                 	add	r8, 0x10
1408d1ea3: 48 83 c1 10                 	add	rcx, 0x10
1408d1ea7: 4d 21 d0                    	and	r8, r10
1408d1eaa: eb 85                       	jmp	0x1408d1e31 <.text+0x8d0e31>
1408d1eac: 49 c1 f9 05                 	sar	r9, 0x5
1408d1eb0: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d1eb4: 4d 21 d0                    	and	r8, r10
1408d1eb7: f3 43 0f 6f 04 03           	movdqu	xmm0, xmmword ptr [r11 + r8]
1408d1ebd: 66 41 0f 74 c2              	pcmpeqb	xmm0, xmm10
1408d1ec2: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d1ec6: 85 d2                       	test	edx, edx
1408d1ec8: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d1ecf: 74 3e                       	je	0x1408d1f0f <.text+0x8d0f0f>
1408d1ed1: 66 0f bd d2                 	bsr	dx, dx
1408d1ed5: 83 f2 0f                    	xor	edx, 0xf
1408d1ed8: eb 39                       	jmp	0x1408d1f13 <.text+0x8d0f13>
1408d1eda: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d1ee1: 48 83 b8 38 3e 00 00 00     	cmp	qword ptr [rax + 0x3e38], 0x0
1408d1ee9: 0f 85 cb 00 00 00           	jne	0x1408d1fba <.text+0x8d0fba>
1408d1eef: ba 01 00 00 00              	mov	edx, 0x1
1408d1ef4: 48 8b 8d 98 02 00 00        	mov	rcx, qword ptr [rbp + 0x298]
1408d1efb: 4c 8b 85 28 03 00 00        	mov	r8, qword ptr [rbp + 0x328]
1408d1f02: 41 b1 01                    	mov	r9b, 0x1
1408d1f05: e8 96 7d fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d1f0a: e9 ab 00 00 00              	jmp	0x1408d1fba <.text+0x8d0fba>
1408d1f0f: 66 ba 10 00                 	mov	dx, 0x10
1408d1f13: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d1f1a: f3 43 0f 6f 04 0b           	movdqu	xmm0, xmmword ptr [r11 + r9]
1408d1f20: 66 41 0f 74 c2              	pcmpeqb	xmm0, xmm10
1408d1f25: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d1f2a: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d1f31: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d1f36: 44 01 d2                    	add	edx, r10d
1408d1f39: 41 b2 80                    	mov	r10b, -0x80
1408d1f3c: 66 83 fa 0f                 	cmp	dx, 0xf
1408d1f40: 77 11                       	ja	0x1408d1f53 <.text+0x8d0f53>
1408d1f42: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d1f49: 48 ff 82 38 3e 00 00        	inc	qword ptr [rdx + 0x3e38]
1408d1f50: 41 b2 ff                    	mov	r10b, -0x1
1408d1f53: 48 f7 d8                    	neg	rax
1408d1f56: 48 c1 e0 05                 	shl	rax, 0x5
1408d1f5a: 4c 01 d8                    	add	rax, r11
1408d1f5d: 47 88 14 0b                 	mov	byte ptr [r11 + r9], r10b
1408d1f61: 47 88 54 03 10              	mov	byte ptr [r11 + r8 + 0x10], r10b
1408d1f66: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d1f6d: 48 ff 8a 40 3e 00 00        	dec	qword ptr [rdx + 0x3e40]
1408d1f74: 44 8b 40 f8                 	mov	r8d, dword ptr [rax - 0x8]
1408d1f78: 44 89 85 00 01 00 00        	mov	dword ptr [rbp + 0x100], r8d
1408d1f7f: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d1f84: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d1f8c: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d1f93: e8 f8 75 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d1f98: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d1f9f: 74 19                       	je	0x1408d1fba <.text+0x8d0fba>
1408d1fa1: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d1fa8: f0                          	lock
1408d1fa9: 48 ff 08                    	dec	qword ptr [rax]
1408d1fac: 75 0c                       	jne	0x1408d1fba <.text+0x8d0fba>
1408d1fae: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d1fb5: e8 66 a6 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d1fba: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1408d1fc1: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
1408d1fc8: 48 8b 8d 08 03 00 00        	mov	rcx, qword ptr [rbp + 0x308]
1408d1fcf: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d1fd6: e8 95 ed cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d1fdb: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d1fe2: 4c 8b 9a 58 3e 00 00        	mov	r11, qword ptr [rdx + 0x3e58]
1408d1fe9: 4c 8b 92 60 3e 00 00        	mov	r10, qword ptr [rdx + 0x3e60]
1408d1ff0: 4d 89 d0                    	mov	r8, r10
1408d1ff3: 49 21 c0                    	and	r8, rax
1408d1ff6: 48 c1 e8 39                 	shr	rax, 0x39
1408d1ffa: 66 0f 6e c0                 	movd	xmm0, eax
1408d1ffe: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d2002: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d2007: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d200c: 4d 8d 4b e0                 	lea	r9, [r11 - 0x20]
1408d2010: 31 c9                       	xor	ecx, ecx
1408d2012: f3 43 0f 6f 0c 03           	movdqu	xmm1, xmmword ptr [r11 + r8]
1408d2018: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d201c: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d2020: 66 0f d7 f2                 	pmovmskb	esi, xmm2
1408d2024: 85 f6                       	test	esi, esi
1408d2026: 74 48                       	je	0x1408d2070 <.text+0x8d1070>
1408d2028: 48 89 8d 38 03 00 00        	mov	qword ptr [rbp + 0x338], rcx
1408d202f: f3 0f bc c6                 	tzcnt	eax, esi
1408d2033: 4c 01 c0                    	add	rax, r8
1408d2036: 4c 21 d0                    	and	rax, r10
1408d2039: 4c 89 ca                    	mov	rdx, r9
1408d203c: 49 89 c1                    	mov	r9, rax
1408d203f: 49 c1 e1 05                 	shl	r9, 0x5
1408d2043: 48 89 d7                    	mov	rdi, rdx
1408d2046: 4c 29 cf                    	sub	rdi, r9
1408d2049: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1408d2050: 48 39 0f                    	cmp	qword ptr [rdi], rcx
1408d2053: 74 38                       	je	0x1408d208d <.text+0x8d108d>
1408d2055: 8d 46 ff                    	lea	eax, [rsi - 0x1]
1408d2058: 66 21 f0                    	and	ax, si
1408d205b: 89 c6                       	mov	esi, eax
1408d205d: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d2064: 48 8b 8d 38 03 00 00        	mov	rcx, qword ptr [rbp + 0x338]
1408d206b: 49 89 d1                    	mov	r9, rdx
1408d206e: 75 bf                       	jne	0x1408d202f <.text+0x8d102f>
1408d2070: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d2075: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d2079: 85 c0                       	test	eax, eax
1408d207b: 75 3e                       	jne	0x1408d20bb <.text+0x8d10bb>
1408d207d: 49 01 c8                    	add	r8, rcx
1408d2080: 49 83 c0 10                 	add	r8, 0x10
1408d2084: 48 83 c1 10                 	add	rcx, 0x10
1408d2088: 4d 21 d0                    	and	r8, r10
1408d208b: eb 85                       	jmp	0x1408d2012 <.text+0x8d1012>
1408d208d: 49 c1 f9 05                 	sar	r9, 0x5
1408d2091: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d2095: 4d 21 d0                    	and	r8, r10
1408d2098: f3 43 0f 6f 04 03           	movdqu	xmm0, xmmword ptr [r11 + r8]
1408d209e: 66 41 0f 74 c2              	pcmpeqb	xmm0, xmm10
1408d20a3: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d20a7: 85 d2                       	test	edx, edx
1408d20a9: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d20b0: 74 3e                       	je	0x1408d20f0 <.text+0x8d10f0>
1408d20b2: 66 0f bd d2                 	bsr	dx, dx
1408d20b6: 83 f2 0f                    	xor	edx, 0xf
1408d20b9: eb 39                       	jmp	0x1408d20f4 <.text+0x8d10f4>
1408d20bb: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d20c2: 48 83 b8 68 3e 00 00 00     	cmp	qword ptr [rax + 0x3e68], 0x0
1408d20ca: 0f 85 5e 14 00 00           	jne	0x1408d352e <.text+0x8d252e>
1408d20d0: ba 01 00 00 00              	mov	edx, 0x1
1408d20d5: 48 8b 8d a0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2a0]
1408d20dc: 4c 8b 85 08 03 00 00        	mov	r8, qword ptr [rbp + 0x308]
1408d20e3: 41 b1 01                    	mov	r9b, 0x1
1408d20e6: e8 b5 7b fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d20eb: e9 3e 14 00 00              	jmp	0x1408d352e <.text+0x8d252e>
1408d20f0: 66 ba 10 00                 	mov	dx, 0x10
1408d20f4: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d20fb: f3 43 0f 6f 04 0b           	movdqu	xmm0, xmmword ptr [r11 + r9]
1408d2101: 66 41 0f 74 c2              	pcmpeqb	xmm0, xmm10
1408d2106: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d210b: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d2112: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d2117: 44 01 d2                    	add	edx, r10d
1408d211a: 41 b2 80                    	mov	r10b, -0x80
1408d211d: 66 83 fa 0f                 	cmp	dx, 0xf
1408d2121: 77 11                       	ja	0x1408d2134 <.text+0x8d1134>
1408d2123: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d212a: 48 ff 82 68 3e 00 00        	inc	qword ptr [rdx + 0x3e68]
1408d2131: 41 b2 ff                    	mov	r10b, -0x1
1408d2134: 48 f7 d8                    	neg	rax
1408d2137: 48 c1 e0 05                 	shl	rax, 0x5
1408d213b: 4c 01 d8                    	add	rax, r11
1408d213e: 47 88 14 0b                 	mov	byte ptr [r11 + r9], r10b
1408d2142: 47 88 54 03 10              	mov	byte ptr [r11 + r8 + 0x10], r10b
1408d2147: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d214e: 48 ff 8a 70 3e 00 00        	dec	qword ptr [rdx + 0x3e70]
1408d2155: 44 8b 40 f8                 	mov	r8d, dword ptr [rax - 0x8]
1408d2159: 44 89 85 00 01 00 00        	mov	dword ptr [rbp + 0x100], r8d
1408d2160: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d2165: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d216d: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d2174: e8 17 74 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d2179: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d2180: 0f 84 a8 13 00 00           	je	0x1408d352e <.text+0x8d252e>
1408d2186: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d218d: f0                          	lock
1408d218e: 48 ff 08                    	dec	qword ptr [rax]
1408d2191: 0f 85 97 13 00 00           	jne	0x1408d352e <.text+0x8d252e>
1408d2197: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d219e: e8 7d a4 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d21a3: e9 86 13 00 00              	jmp	0x1408d352e <.text+0x8d252e>
1408d21a8: 48 f7 de                    	neg	rsi
1408d21ab: 48 c7 85 78 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x178], 0x0
1408d21b6: 48 c7 85 80 01 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x180], 0x4
1408d21c1: 48 c7 85 88 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x188], 0x0
1408d21cc: 48 c7 85 40 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x240], 0x0
1408d21d7: 48 c7 85 48 02 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x248], 0x4
1408d21e2: 48 c7 85 50 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x250], 0x0
1408d21ed: 48 c1 e6 05                 	shl	rsi, 0x5
1408d21f1: 48 8b 7c 31 f0              	mov	rdi, qword ptr [rcx + rsi - 0x10]
1408d21f6: 48 6b 44 31 f8 68           	imul	rax, qword ptr [rcx + rsi - 0x8], 0x68
1408d21fc: 48 01 f8                    	add	rax, rdi
1408d21ff: 48 89 85 40 03 00 00        	mov	qword ptr [rbp + 0x340], rax
1408d2206: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1408d2210: 48 3b bd 40 03 00 00        	cmp	rdi, qword ptr [rbp + 0x340]
1408d2217: 48 89 bd a0 01 00 00        	mov	qword ptr [rbp + 0x1a0], rdi
1408d221e: 0f 84 bc 00 00 00           	je	0x1408d22e0 <.text+0x8d12e0>
1408d2224: 48 89 f9                    	mov	rcx, rdi
1408d2227: 48 83 c7 68                 	add	rdi, 0x68
1408d222b: 48 8d 95 40 02 00 00        	lea	rdx, [rbp + 0x240]
1408d2232: 4c 8d 85 78 01 00 00        	lea	r8, [rbp + 0x178]
1408d2239: e8 92 5c 00 00              	call	0x1408d7ed0 <.text+0x8d6ed0>
1408d223e: eb d0                       	jmp	0x1408d2210 <.text+0x8d1210>
1408d2240: 48 f7 de                    	neg	rsi
1408d2243: 48 c7 85 78 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x178], 0x0
1408d224e: 48 c7 85 80 01 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x180], 0x4
1408d2259: 48 c7 85 88 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x188], 0x0
1408d2264: 48 c7 85 40 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x240], 0x0
1408d226f: 48 c7 85 48 02 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x248], 0x4
1408d227a: 48 c7 85 50 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x250], 0x0
1408d2285: 48 c1 e6 05                 	shl	rsi, 0x5
1408d2289: 48 8b 7c 31 f0              	mov	rdi, qword ptr [rcx + rsi - 0x10]
1408d228e: 48 6b 44 31 f8 68           	imul	rax, qword ptr [rcx + rsi - 0x8], 0x68
1408d2294: 48 01 f8                    	add	rax, rdi
1408d2297: 48 89 85 40 03 00 00        	mov	qword ptr [rbp + 0x340], rax
1408d229e: 48 8b b5 38 03 00 00        	mov	rsi, qword ptr [rbp + 0x338]
1408d22a5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1408d22b0: 48 3b bd 40 03 00 00        	cmp	rdi, qword ptr [rbp + 0x340]
1408d22b7: 48 89 bd a0 01 00 00        	mov	qword ptr [rbp + 0x1a0], rdi
1408d22be: 0f 84 66 02 00 00           	je	0x1408d252a <.text+0x8d152a>
1408d22c4: 48 89 f9                    	mov	rcx, rdi
1408d22c7: 48 83 c7 68                 	add	rdi, 0x68
1408d22cb: 48 8d 95 40 02 00 00        	lea	rdx, [rbp + 0x240]
1408d22d2: 4c 8d 85 78 01 00 00        	lea	r8, [rbp + 0x178]
1408d22d9: e8 f2 5b 00 00              	call	0x1408d7ed0 <.text+0x8d6ed0>
1408d22de: eb d0                       	jmp	0x1408d22b0 <.text+0x8d12b0>
1408d22e0: 48 83 bd 88 01 00 00 00     	cmp	qword ptr [rbp + 0x188], 0x0
1408d22e8: 0f 84 84 04 00 00           	je	0x1408d2772 <.text+0x8d1772>
1408d22ee: 48 8b 85 88 01 00 00        	mov	rax, qword ptr [rbp + 0x188]
1408d22f5: 48 89 85 80 02 00 00        	mov	qword ptr [rbp + 0x280], rax
1408d22fc: 0f 10 85 78 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x178]
1408d2303: 0f 29 85 70 02 00 00        	movaps	xmmword ptr [rbp + 0x270], xmm0
1408d230a: 48 8b 85 50 02 00 00        	mov	rax, qword ptr [rbp + 0x250]
1408d2311: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d2318: f3 0f 6f 85 40 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x240]
1408d2320: 66 0f 7f 85 b0 02 00 00     	movdqa	xmmword ptr [rbp + 0x2b0], xmm0
1408d2328: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1408d2332: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
1408d2339: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
1408d2340: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
1408d2347: 4c 8d 85 b0 02 00 00        	lea	r8, [rbp + 0x2b0]
1408d234e: 4c 8d 8d b0 01 00 00        	lea	r9, [rbp + 0x1b0]
1408d2355: e8 76 d4 ff ff              	call	0x1408cf7d0 <.text+0x8ce7d0>
1408d235a: 48 c7 85 b0 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x1
1408d2365: 48 c7 85 b8 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b8], 0x1
1408d2370: 0f 28 85 60 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x160]
1408d2377: 48 8d 85 c0 01 00 00        	lea	rax, [rbp + 0x1c0]
1408d237e: 0f 29 40 70                 	movaps	xmmword ptr [rax + 0x70], xmm0
1408d2382: 0f 28 85 50 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x150]
1408d2389: 0f 29 40 60                 	movaps	xmmword ptr [rax + 0x60], xmm0
1408d238d: 0f 28 85 40 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x140]
1408d2394: 0f 29 40 50                 	movaps	xmmword ptr [rax + 0x50], xmm0
1408d2398: 0f 28 85 30 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x130]
1408d239f: 0f 29 40 40                 	movaps	xmmword ptr [rax + 0x40], xmm0
1408d23a3: 66 0f 6f 85 f0 00 00 00     	movdqa	xmm0, xmmword ptr [rbp + 0xf0]
1408d23ab: 66 0f 6f 8d 00 01 00 00     	movdqa	xmm1, xmmword ptr [rbp + 0x100]
1408d23b3: 66 0f 6f 95 10 01 00 00     	movdqa	xmm2, xmmword ptr [rbp + 0x110]
1408d23bb: 0f 28 9d 20 01 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x120]
1408d23c2: 0f 29 58 30                 	movaps	xmmword ptr [rax + 0x30], xmm3
1408d23c6: 66 0f 7f 50 20              	movdqa	xmmword ptr [rax + 0x20], xmm2
1408d23cb: 66 0f 7f 48 10              	movdqa	xmmword ptr [rax + 0x10], xmm1
1408d23d0: 66 0f 7f 00                 	movdqa	xmmword ptr [rax], xmm0
1408d23d4: 0f b6 05 2e 28 d6 02        	movzx	eax, byte ptr [rip + 0x2d6282e] # 0x143634c09
1408d23db: b9 90 00 00 00              	mov	ecx, 0x90
1408d23e0: ba 10 00 00 00              	mov	edx, 0x10
1408d23e5: e8 26 18 d4 ff              	call	0x140613c10 <.text+0x612c10>
1408d23ea: 48 85 c0                    	test	rax, rax
1408d23ed: 0f 84 ae 39 00 00           	je	0x1408d5da1 <.text+0x8d4da1>
1408d23f3: 41 b8 90 00 00 00           	mov	r8d, 0x90
1408d23f9: 48 89 c1                    	mov	rcx, rax
1408d23fc: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d2403: 48 89 fa                    	mov	rdx, rdi
1408d2406: 48 89 c6                    	mov	rsi, rax
1408d2409: e8 f2 17 05 02              	call	0x142923c00 <.text+0x2922c00>
1408d240e: 48 8b 85 38 03 00 00        	mov	rax, qword ptr [rbp + 0x338]
1408d2415: 48 8b 00                    	mov	rax, qword ptr [rax]
1408d2418: 48 89 b5 a0 00 00 00        	mov	qword ptr [rbp + 0xa0], rsi
1408d241f: 48 89 b5 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rsi
1408d2426: c6 85 b8 00 00 00 09        	mov	byte ptr [rbp + 0xb8], 0x9
1408d242d: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
1408d2434: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1408d243b: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
1408d2442: c6 85 55 03 00 00 01        	mov	byte ptr [rbp + 0x355], 0x1
1408d2449: 48 8b 8d 08 03 00 00        	mov	rcx, qword ptr [rbp + 0x308]
1408d2450: 48 89 fa                    	mov	rdx, rdi
1408d2453: e8 18 e9 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d2458: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d245f: 48 8b 8a 58 3e 00 00        	mov	rcx, qword ptr [rdx + 0x3e58]
1408d2466: 48 8b 92 60 3e 00 00        	mov	rdx, qword ptr [rdx + 0x3e60]
1408d246d: 49 89 d0                    	mov	r8, rdx
1408d2470: 49 21 c0                    	and	r8, rax
1408d2473: 49 89 c1                    	mov	r9, rax
1408d2476: 49 c1 e9 39                 	shr	r9, 0x39
1408d247a: 66 41 0f 6e c1              	movd	xmm0, r9d
1408d247f: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d2483: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d2488: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d248d: 4c 8d 49 e0                 	lea	r9, [rcx - 0x20]
1408d2491: 31 f6                       	xor	esi, esi
1408d2493: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d2499: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d249d: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d24a1: 66 44 0f d7 da              	pmovmskb	r11d, xmm2
1408d24a6: 45 85 db                    	test	r11d, r11d
1408d24a9: 74 53                       	je	0x1408d24fe <.text+0x8d14fe>
1408d24ab: 48 89 b5 98 00 00 00        	mov	qword ptr [rbp + 0x98], rsi
1408d24b2: f3 41 0f bc f3              	tzcnt	esi, r11d
1408d24b7: 4c 01 c6                    	add	rsi, r8
1408d24ba: 48 21 d6                    	and	rsi, rdx
1408d24bd: 48 89 f7                    	mov	rdi, rsi
1408d24c0: 48 c1 e7 05                 	shl	rdi, 0x5
1408d24c4: 49 89 d2                    	mov	r10, rdx
1408d24c7: 4c 89 ca                    	mov	rdx, r9
1408d24ca: 49 29 f9                    	sub	r9, rdi
1408d24cd: 48 8b bd 30 03 00 00        	mov	rdi, qword ptr [rbp + 0x330]
1408d24d4: 49 39 39                    	cmp	qword ptr [r9], rdi
1408d24d7: 0f 84 d2 13 00 00           	je	0x1408d38af <.text+0x8d28af>
1408d24dd: 45 8d 4b ff                 	lea	r9d, [r11 - 0x1]
1408d24e1: 66 45 21 d9                 	and	r9w, r11w
1408d24e5: 45 89 cb                    	mov	r11d, r9d
1408d24e8: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d24ef: 48 8b b5 98 00 00 00        	mov	rsi, qword ptr [rbp + 0x98]
1408d24f6: 49 89 d1                    	mov	r9, rdx
1408d24f9: 4c 89 d2                    	mov	rdx, r10
1408d24fc: 75 b4                       	jne	0x1408d24b2 <.text+0x8d14b2>
1408d24fe: 4d 89 ca                    	mov	r10, r9
1408d2501: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d2506: 66 44 0f d7 c9              	pmovmskb	r9d, xmm1
1408d250b: 45 85 c9                    	test	r9d, r9d
1408d250e: 0f 85 64 17 00 00           	jne	0x1408d3c78 <.text+0x8d2c78>
1408d2514: 49 01 f0                    	add	r8, rsi
1408d2517: 49 83 c0 10                 	add	r8, 0x10
1408d251b: 48 83 c6 10                 	add	rsi, 0x10
1408d251f: 49 21 d0                    	and	r8, rdx
1408d2522: 4d 89 d1                    	mov	r9, r10
1408d2525: e9 69 ff ff ff              	jmp	0x1408d2493 <.text+0x8d1493>
1408d252a: 48 83 bd 88 01 00 00 00     	cmp	qword ptr [rbp + 0x188], 0x0
1408d2532: 0f 84 3a 02 00 00           	je	0x1408d2772 <.text+0x8d1772>
1408d2538: 48 8b 85 88 01 00 00        	mov	rax, qword ptr [rbp + 0x188]
1408d253f: 48 89 85 80 02 00 00        	mov	qword ptr [rbp + 0x280], rax
1408d2546: 0f 10 85 78 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x178]
1408d254d: 0f 29 85 70 02 00 00        	movaps	xmmword ptr [rbp + 0x270], xmm0
1408d2554: 48 8b 85 50 02 00 00        	mov	rax, qword ptr [rbp + 0x250]
1408d255b: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d2562: f3 0f 6f 85 40 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x240]
1408d256a: 66 0f 7f 85 b0 02 00 00     	movdqa	xmmword ptr [rbp + 0x2b0], xmm0
1408d2572: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1408d257c: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
1408d2583: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
1408d258a: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
1408d2591: 4c 8d 85 b0 02 00 00        	lea	r8, [rbp + 0x2b0]
1408d2598: 4c 8d 8d b0 01 00 00        	lea	r9, [rbp + 0x1b0]
1408d259f: e8 2c d2 ff ff              	call	0x1408cf7d0 <.text+0x8ce7d0>
1408d25a4: 48 c7 85 b0 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x1
1408d25af: 48 c7 85 b8 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b8], 0x1
1408d25ba: 0f 28 85 60 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x160]
1408d25c1: 48 8d 85 c0 01 00 00        	lea	rax, [rbp + 0x1c0]
1408d25c8: 0f 29 40 70                 	movaps	xmmword ptr [rax + 0x70], xmm0
1408d25cc: 0f 28 85 50 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x150]
1408d25d3: 0f 29 40 60                 	movaps	xmmword ptr [rax + 0x60], xmm0
1408d25d7: 0f 28 85 40 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x140]
1408d25de: 0f 29 40 50                 	movaps	xmmword ptr [rax + 0x50], xmm0
1408d25e2: 0f 28 85 30 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x130]
1408d25e9: 0f 29 40 40                 	movaps	xmmword ptr [rax + 0x40], xmm0
1408d25ed: 66 0f 6f 85 f0 00 00 00     	movdqa	xmm0, xmmword ptr [rbp + 0xf0]
1408d25f5: 66 0f 6f 8d 00 01 00 00     	movdqa	xmm1, xmmword ptr [rbp + 0x100]
1408d25fd: 66 0f 6f 95 10 01 00 00     	movdqa	xmm2, xmmword ptr [rbp + 0x110]
1408d2605: 0f 28 9d 20 01 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x120]
1408d260c: 0f 29 58 30                 	movaps	xmmword ptr [rax + 0x30], xmm3
1408d2610: 66 0f 7f 50 20              	movdqa	xmmword ptr [rax + 0x20], xmm2
1408d2615: 66 0f 7f 48 10              	movdqa	xmmword ptr [rax + 0x10], xmm1
1408d261a: 66 0f 7f 00                 	movdqa	xmmword ptr [rax], xmm0
1408d261e: 0f b6 05 e4 25 d6 02        	movzx	eax, byte ptr [rip + 0x2d625e4] # 0x143634c09
1408d2625: b9 90 00 00 00              	mov	ecx, 0x90
1408d262a: ba 10 00 00 00              	mov	edx, 0x10
1408d262f: e8 dc 15 d4 ff              	call	0x140613c10 <.text+0x612c10>
1408d2634: 48 85 c0                    	test	rax, rax
1408d2637: 0f 84 50 37 00 00           	je	0x1408d5d8d <.text+0x8d4d8d>
1408d263d: 41 b8 90 00 00 00           	mov	r8d, 0x90
1408d2643: 48 89 c1                    	mov	rcx, rax
1408d2646: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d264d: 48 89 c7                    	mov	rdi, rax
1408d2650: e8 ab 15 05 02              	call	0x142923c00 <.text+0x2922c00>
1408d2655: 48 8b 06                    	mov	rax, qword ptr [rsi]
1408d2658: 48 89 bd 98 00 00 00        	mov	qword ptr [rbp + 0x98], rdi
1408d265f: 48 89 bd d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rdi
1408d2666: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d266d: c6 85 b8 00 00 00 09        	mov	byte ptr [rbp + 0xb8], 0x9
1408d2674: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
1408d267b: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1408d2682: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
1408d2689: c6 85 54 03 00 00 01        	mov	byte ptr [rbp + 0x354], 0x1
1408d2690: 48 8b 8d 08 03 00 00        	mov	rcx, qword ptr [rbp + 0x308]
1408d2697: 48 89 fa                    	mov	rdx, rdi
1408d269a: e8 d1 e6 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d269f: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d26a6: 48 8b 8a 58 3e 00 00        	mov	rcx, qword ptr [rdx + 0x3e58]
1408d26ad: 48 8b 92 60 3e 00 00        	mov	rdx, qword ptr [rdx + 0x3e60]
1408d26b4: 49 89 d0                    	mov	r8, rdx
1408d26b7: 49 21 c0                    	and	r8, rax
1408d26ba: 49 89 c1                    	mov	r9, rax
1408d26bd: 49 c1 e9 39                 	shr	r9, 0x39
1408d26c1: 66 41 0f 6e c1              	movd	xmm0, r9d
1408d26c6: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d26ca: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d26cf: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d26d4: 4c 8d 51 e0                 	lea	r10, [rcx - 0x20]
1408d26d8: 48 c7 85 a0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xa0], 0x0
1408d26e3: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d26e9: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d26ed: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d26f1: 66 44 0f d7 da              	pmovmskb	r11d, xmm2
1408d26f6: 45 85 db                    	test	r11d, r11d
1408d26f9: 74 43                       	je	0x1408d273e <.text+0x8d173e>
1408d26fb: f3 41 0f bc f3              	tzcnt	esi, r11d
1408d2700: 4c 01 c6                    	add	rsi, r8
1408d2703: 48 21 d6                    	and	rsi, rdx
1408d2706: 48 89 f7                    	mov	rdi, rsi
1408d2709: 48 c1 e7 05                 	shl	rdi, 0x5
1408d270d: 4d 89 d1                    	mov	r9, r10
1408d2710: 49 29 f9                    	sub	r9, rdi
1408d2713: 48 8b bd 30 03 00 00        	mov	rdi, qword ptr [rbp + 0x330]
1408d271a: 49 39 39                    	cmp	qword ptr [r9], rdi
1408d271d: 0f 84 c2 11 00 00           	je	0x1408d38e5 <.text+0x8d28e5>
1408d2723: 45 8d 4b ff                 	lea	r9d, [r11 - 0x1]
1408d2727: 66 45 21 d9                 	and	r9w, r11w
1408d272b: 45 89 cb                    	mov	r11d, r9d
1408d272e: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d2735: 48 8b b5 38 03 00 00        	mov	rsi, qword ptr [rbp + 0x338]
1408d273c: 75 bd                       	jne	0x1408d26fb <.text+0x8d16fb>
1408d273e: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d2743: 66 44 0f d7 c9              	pmovmskb	r9d, xmm1
1408d2748: 45 85 c9                    	test	r9d, r9d
1408d274b: 0f 85 ce 15 00 00           	jne	0x1408d3d1f <.text+0x8d2d1f>
1408d2751: 4c 8b 8d a0 00 00 00        	mov	r9, qword ptr [rbp + 0xa0]
1408d2758: 4d 01 c8                    	add	r8, r9
1408d275b: 49 83 c0 10                 	add	r8, 0x10
1408d275f: 49 83 c1 10                 	add	r9, 0x10
1408d2763: 4c 89 8d a0 00 00 00        	mov	qword ptr [rbp + 0xa0], r9
1408d276a: 49 21 d0                    	and	r8, rdx
1408d276d: e9 71 ff ff ff              	jmp	0x1408d26e3 <.text+0x8d16e3>
1408d2772: 48 8b 85 40 02 00 00        	mov	rax, qword ptr [rbp + 0x240]
1408d2779: 48 85 c0                    	test	rax, rax
1408d277c: 48 8b b5 30 03 00 00        	mov	rsi, qword ptr [rbp + 0x330]
1408d2783: 74 1a                       	je	0x1408d279f <.text+0x8d179f>
1408d2785: 48 8b 8d 48 02 00 00        	mov	rcx, qword ptr [rbp + 0x248]
1408d278c: 48 c1 e0 02                 	shl	rax, 0x2
1408d2790: 48 8d 14 c0                 	lea	rdx, [rax + 8*rax]
1408d2794: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d279a: e8 81 14 d4 ff              	call	0x140613c20 <.text+0x612c20>
1408d279f: 48 8b 85 78 01 00 00        	mov	rax, qword ptr [rbp + 0x178]
1408d27a6: 48 85 c0                    	test	rax, rax
1408d27a9: 74 1a                       	je	0x1408d27c5 <.text+0x8d17c5>
1408d27ab: 48 8b 8d 80 01 00 00        	mov	rcx, qword ptr [rbp + 0x180]
1408d27b2: 48 c1 e0 02                 	shl	rax, 0x2
1408d27b6: 48 8d 14 c0                 	lea	rdx, [rax + 8*rax]
1408d27ba: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d27c0: e8 5b 14 d4 ff              	call	0x140613c20 <.text+0x612c20>
1408d27c5: 48 89 b5 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rsi
1408d27cc: 48 8b 8d 08 03 00 00        	mov	rcx, qword ptr [rbp + 0x308]
1408d27d3: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d27da: e8 91 e5 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d27df: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d27e6: 4c 8b 9a 58 3e 00 00        	mov	r11, qword ptr [rdx + 0x3e58]
1408d27ed: 4c 8b 92 60 3e 00 00        	mov	r10, qword ptr [rdx + 0x3e60]
1408d27f4: 4d 89 d0                    	mov	r8, r10
1408d27f7: 49 21 c0                    	and	r8, rax
1408d27fa: 48 c1 e8 39                 	shr	rax, 0x39
1408d27fe: 66 0f 6e c0                 	movd	xmm0, eax
1408d2802: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d2806: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d280b: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d2810: 49 8d 7b e0                 	lea	rdi, [r11 - 0x20]
1408d2814: 31 c9                       	xor	ecx, ecx
1408d2816: f3 43 0f 6f 0c 03           	movdqu	xmm1, xmmword ptr [r11 + r8]
1408d281c: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d2820: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d2824: 66 0f d7 f2                 	pmovmskb	esi, xmm2
1408d2828: 85 f6                       	test	esi, esi
1408d282a: 74 3e                       	je	0x1408d286a <.text+0x8d186a>
1408d282c: 48 89 8d 40 03 00 00        	mov	qword ptr [rbp + 0x340], rcx
1408d2833: f3 0f bc c6                 	tzcnt	eax, esi
1408d2837: 4c 01 c0                    	add	rax, r8
1408d283a: 4c 21 d0                    	and	rax, r10
1408d283d: 49 89 c1                    	mov	r9, rax
1408d2840: 49 c1 e1 05                 	shl	r9, 0x5
1408d2844: 48 89 fa                    	mov	rdx, rdi
1408d2847: 4c 29 cf                    	sub	rdi, r9
1408d284a: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1408d2851: 48 39 0f                    	cmp	qword ptr [rdi], rcx
1408d2854: 74 31                       	je	0x1408d2887 <.text+0x8d1887>
1408d2856: 8d 46 ff                    	lea	eax, [rsi - 0x1]
1408d2859: 66 21 f0                    	and	ax, si
1408d285c: 89 c6                       	mov	esi, eax
1408d285e: 48 8b 8d 40 03 00 00        	mov	rcx, qword ptr [rbp + 0x340]
1408d2865: 48 89 d7                    	mov	rdi, rdx
1408d2868: 75 c9                       	jne	0x1408d2833 <.text+0x8d1833>
1408d286a: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d286f: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d2873: 85 c0                       	test	eax, eax
1408d2875: 75 3e                       	jne	0x1408d28b5 <.text+0x8d18b5>
1408d2877: 49 01 c8                    	add	r8, rcx
1408d287a: 49 83 c0 10                 	add	r8, 0x10
1408d287e: 48 83 c1 10                 	add	rcx, 0x10
1408d2882: 4d 21 d0                    	and	r8, r10
1408d2885: eb 8f                       	jmp	0x1408d2816 <.text+0x8d1816>
1408d2887: 49 c1 f9 05                 	sar	r9, 0x5
1408d288b: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d288f: 4d 21 d0                    	and	r8, r10
1408d2892: f3 43 0f 6f 04 03           	movdqu	xmm0, xmmword ptr [r11 + r8]
1408d2898: 66 41 0f 74 c2              	pcmpeqb	xmm0, xmm10
1408d289d: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d28a1: 85 d2                       	test	edx, edx
1408d28a3: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d28aa: 74 3e                       	je	0x1408d28ea <.text+0x8d18ea>
1408d28ac: 66 0f bd d2                 	bsr	dx, dx
1408d28b0: 83 f2 0f                    	xor	edx, 0xf
1408d28b3: eb 39                       	jmp	0x1408d28ee <.text+0x8d18ee>
1408d28b5: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d28bc: 48 83 b8 68 3e 00 00 00     	cmp	qword ptr [rax + 0x3e68], 0x0
1408d28c4: 0f 85 c4 00 00 00           	jne	0x1408d298e <.text+0x8d198e>
1408d28ca: ba 01 00 00 00              	mov	edx, 0x1
1408d28cf: 48 8b 8d a0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2a0]
1408d28d6: 4c 8b 85 08 03 00 00        	mov	r8, qword ptr [rbp + 0x308]
1408d28dd: 41 b1 01                    	mov	r9b, 0x1
1408d28e0: e8 bb 73 fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d28e5: e9 a4 00 00 00              	jmp	0x1408d298e <.text+0x8d198e>
1408d28ea: 66 ba 10 00                 	mov	dx, 0x10
1408d28ee: f3 43 0f 6f 04 0b           	movdqu	xmm0, xmmword ptr [r11 + r9]
1408d28f4: 66 41 0f 74 c2              	pcmpeqb	xmm0, xmm10
1408d28f9: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d28fe: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d2905: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d290a: 44 01 d2                    	add	edx, r10d
1408d290d: 41 b2 80                    	mov	r10b, -0x80
1408d2910: 66 83 fa 0f                 	cmp	dx, 0xf
1408d2914: 77 11                       	ja	0x1408d2927 <.text+0x8d1927>
1408d2916: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d291d: 48 ff 82 68 3e 00 00        	inc	qword ptr [rdx + 0x3e68]
1408d2924: 41 b2 ff                    	mov	r10b, -0x1
1408d2927: 48 f7 d8                    	neg	rax
1408d292a: 48 c1 e0 05                 	shl	rax, 0x5
1408d292e: 4c 01 d8                    	add	rax, r11
1408d2931: 47 88 14 0b                 	mov	byte ptr [r11 + r9], r10b
1408d2935: 47 88 54 03 10              	mov	byte ptr [r11 + r8 + 0x10], r10b
1408d293a: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d2941: 48 ff 8a 70 3e 00 00        	dec	qword ptr [rdx + 0x3e70]
1408d2948: 44 8b 40 f8                 	mov	r8d, dword ptr [rax - 0x8]
1408d294c: 44 89 85 00 01 00 00        	mov	dword ptr [rbp + 0x100], r8d
1408d2953: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d2958: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d2960: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d2967: e8 24 6c 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d296c: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d2973: 74 19                       	je	0x1408d298e <.text+0x8d198e>
1408d2975: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d297c: f0                          	lock
1408d297d: 48 ff 08                    	dec	qword ptr [rax]
1408d2980: 75 0c                       	jne	0x1408d298e <.text+0x8d198e>
1408d2982: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d2989: e8 92 9c 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d298e: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d2995: 48 8b b5 38 03 00 00        	mov	rsi, qword ptr [rbp + 0x338]
1408d299c: 48 8b 36                    	mov	rsi, qword ptr [rsi]
1408d299f: 48 89 b5 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rsi
1408d29a6: 48 8b 85 60 02 00 00        	mov	rax, qword ptr [rbp + 0x260]
1408d29ad: 48 83 78 48 00              	cmp	qword ptr [rax + 0x48], 0x0
1408d29b2: 0f 84 82 01 00 00           	je	0x1408d2b3a <.text+0x8d1b3a>
1408d29b8: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
1408d29bf: 48 89 fa                    	mov	rdx, rdi
1408d29c2: e8 a9 e3 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d29c7: 48 8b 95 60 02 00 00        	mov	rdx, qword ptr [rbp + 0x260]
1408d29ce: 48 8b 4a 30                 	mov	rcx, qword ptr [rdx + 0x30]
1408d29d2: 48 8b 52 38                 	mov	rdx, qword ptr [rdx + 0x38]
1408d29d6: 49 89 d0                    	mov	r8, rdx
1408d29d9: 49 21 c0                    	and	r8, rax
1408d29dc: 48 c1 e8 39                 	shr	rax, 0x39
1408d29e0: 66 0f 6e c0                 	movd	xmm0, eax
1408d29e4: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d29e8: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d29ed: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d29f2: 48 8d 41 f0                 	lea	rax, [rcx - 0x10]
1408d29f6: 45 31 db                    	xor	r11d, r11d
1408d29f9: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d29ff: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d2a03: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d2a07: 66 44 0f d7 d2              	pmovmskb	r10d, xmm2
1408d2a0c: 45 85 d2                    	test	r10d, r10d
1408d2a0f: 74 4e                       	je	0x1408d2a5f <.text+0x8d1a5f>
1408d2a11: 4c 89 9d 40 03 00 00        	mov	qword ptr [rbp + 0x340], r11
1408d2a18: 49 89 c9                    	mov	r9, rcx
1408d2a1b: f3 45 0f bc da              	tzcnt	r11d, r10d
1408d2a20: 4d 01 c3                    	add	r11, r8
1408d2a23: 49 21 d3                    	and	r11, rdx
1408d2a26: 4c 89 df                    	mov	rdi, r11
1408d2a29: 48 c1 e7 04                 	shl	rdi, 0x4
1408d2a2d: 48 89 d1                    	mov	rcx, rdx
1408d2a30: 48 89 c2                    	mov	rdx, rax
1408d2a33: 48 29 f8                    	sub	rax, rdi
1408d2a36: 48 39 30                    	cmp	qword ptr [rax], rsi
1408d2a39: 74 4e                       	je	0x1408d2a89 <.text+0x8d1a89>
1408d2a3b: 41 8d 42 ff                 	lea	eax, [r10 - 0x1]
1408d2a3f: 66 44 21 d0                 	and	ax, r10w
1408d2a43: 41 89 c2                    	mov	r10d, eax
1408d2a46: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d2a4d: 4c 8b 9d 40 03 00 00        	mov	r11, qword ptr [rbp + 0x340]
1408d2a54: 48 89 d0                    	mov	rax, rdx
1408d2a57: 48 89 ca                    	mov	rdx, rcx
1408d2a5a: 4c 89 c9                    	mov	rcx, r9
1408d2a5d: 75 b9                       	jne	0x1408d2a18 <.text+0x8d1a18>
1408d2a5f: 49 89 c1                    	mov	r9, rax
1408d2a62: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d2a67: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d2a6b: 85 c0                       	test	eax, eax
1408d2a6d: 0f 85 c7 00 00 00           	jne	0x1408d2b3a <.text+0x8d1b3a>
1408d2a73: 4d 01 d8                    	add	r8, r11
1408d2a76: 49 83 c0 10                 	add	r8, 0x10
1408d2a7a: 49 83 c3 10                 	add	r11, 0x10
1408d2a7e: 49 21 d0                    	and	r8, rdx
1408d2a81: 4c 89 c8                    	mov	rax, r9
1408d2a84: e9 70 ff ff ff              	jmp	0x1408d29f9 <.text+0x8d19f9>
1408d2a89: 49 f7 db                    	neg	r11
1408d2a8c: 49 c1 e3 04                 	shl	r11, 0x4
1408d2a90: 4b 8b 4c 19 f8              	mov	rcx, qword ptr [r9 + r11 - 0x8]
1408d2a95: 48 89 c8                    	mov	rax, rcx
1408d2a98: 48 c1 e8 20                 	shr	rax, 0x20
1408d2a9c: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d2aa3: 0f 84 91 00 00 00           	je	0x1408d2b3a <.text+0x8d1b3a>
1408d2aa9: 89 c9                       	mov	ecx, ecx
1408d2aab: 4c 8b 55 28                 	mov	r10, qword ptr [rbp + 0x28]
1408d2aaf: 49 3b 4a 10                 	cmp	rcx, qword ptr [r10 + 0x10]
1408d2ab3: 4c 8b 8d 88 00 00 00        	mov	r9, qword ptr [rbp + 0x88]
1408d2aba: 73 7e                       	jae	0x1408d2b3a <.text+0x8d1b3a>
1408d2abc: 49 8b 52 08                 	mov	rdx, qword ptr [r10 + 0x8]
1408d2ac0: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
1408d2ac4: 39 04 8a                    	cmp	dword ptr [rdx + 4*rcx], eax
1408d2ac7: 75 71                       	jne	0x1408d2b3a <.text+0x8d1b3a>
1408d2ac9: 48 8d 04 8a                 	lea	rax, [rdx + 4*rcx]
1408d2acd: 8b 48 04                    	mov	ecx, dword ptr [rax + 0x4]
1408d2ad0: ba ff ff ff ff              	mov	edx, 0xffffffff
1408d2ad5: 48 39 d1                    	cmp	rcx, rdx
1408d2ad8: 74 60                       	je	0x1408d2b3a <.text+0x8d1b3a>
1408d2ada: 49 39 49 38                 	cmp	qword ptr [r9 + 0x38], rcx
1408d2ade: 76 5a                       	jbe	0x1408d2b3a <.text+0x8d1b3a>
1408d2ae0: 49 8b 51 28                 	mov	rdx, qword ptr [r9 + 0x28]
1408d2ae4: 41 89 c8                    	mov	r8d, ecx
1408d2ae7: 41 c1 e8 06                 	shr	r8d, 0x6
1408d2aeb: 4a 8b 14 c2                 	mov	rdx, qword ptr [rdx + 8*r8]
1408d2aef: 48 0f a3 ca                 	bt	rdx, rcx
1408d2af3: 73 45                       	jae	0x1408d2b3a <.text+0x8d1b3a>
1408d2af5: 8b 48 0c                    	mov	ecx, dword ptr [rax + 0xc]
1408d2af8: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1408d2afb: 49 8b 92 a8 01 00 00        	mov	rdx, qword ptr [r10 + 0x1a8]
1408d2b02: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1408d2b06: 4d 8b 81 10 01 00 00        	mov	r8, qword ptr [r9 + 0x110]
1408d2b0d: 4c 8b 4c ca 18              	mov	r9, qword ptr [rdx + 8*rcx + 0x18]
1408d2b12: 48 8b 4c ca 38              	mov	rcx, qword ptr [rdx + 8*rcx + 0x38]
1408d2b17: 4a 8b 0c c1                 	mov	rcx, qword ptr [rcx + 8*r8]
1408d2b1b: 48 f7 d1                    	not	rcx
1408d2b1e: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1408d2b22: 48 c1 e1 04                 	shl	rcx, 0x4
1408d2b26: 49 8b 4c 09 10              	mov	rcx, qword ptr [r9 + rcx + 0x10]
1408d2b2b: 48 6b c0 58                 	imul	rax, rax, 0x58
1408d2b2f: 44 0f 2e 7c 01 30           	ucomiss	xmm15, dword ptr [rcx + rax + 0x30]
1408d2b35: 0f 93 c0                    	setae	al
1408d2b38: eb 02                       	jmp	0x1408d2b3c <.text+0x8d1b3c>
1408d2b3a: 31 c0                       	xor	eax, eax
1408d2b3c: 48 8b 95 a8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1a8]
1408d2b43: 48 83 7a 10 00              	cmp	qword ptr [rdx + 0x10], 0x0
1408d2b48: 0f 84 c6 02 00 00           	je	0x1408d2e14 <.text+0x8d1e14>
1408d2b4e: 41 0f 28 c0                 	movaps	xmm0, xmm8
1408d2b52: 83 bd 84 00 00 00 02        	cmp	dword ptr [rbp + 0x84], 0x2
1408d2b59: 72 03                       	jb	0x1408d2b5e <.text+0x8d1b5e>
1408d2b5b: 0f 57 c0                    	xorps	xmm0, xmm0
1408d2b5e: 41 0f 28 cc                 	movaps	xmm1, xmm12
1408d2b62: 84 c0                       	test	al, al
1408d2b64: 75 03                       	jne	0x1408d2b69 <.text+0x8d1b69>
1408d2b66: 0f 57 c9                    	xorps	xmm1, xmm1
1408d2b69: c7 85 f0 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xf0], 0x0
1408d2b73: c7 85 fc 00 00 00 01 00 00 00       	mov	dword ptr [rbp + 0xfc], 0x1
1408d2b7d: f3 0f 11 85 00 01 00 00     	movss	dword ptr [rbp + 0x100], xmm0
1408d2b85: c7 85 04 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x104], 0x0
1408d2b8f: f3 0f 11 8d 08 01 00 00     	movss	dword ptr [rbp + 0x108], xmm1
1408d2b97: c6 85 0c 01 00 00 01        	mov	byte ptr [rbp + 0x10c], 0x1
1408d2b9e: 48 89 f9                    	mov	rcx, rdi
1408d2ba1: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d2ba8: e8 c3 61 01 00              	call	0x1408e8d70 <.text+0x8e7d70>
1408d2bad: 0f 10 85 b0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b0]
1408d2bb4: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1408d2bb8: 48 8b 85 c0 01 00 00        	mov	rax, qword ptr [rbp + 0x1c0]
1408d2bbf: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1408d2bc3: 48 8d 8d c8 01 00 00        	lea	rcx, [rbp + 0x1c8]
1408d2bca: 0f 10 01                    	movups	xmm0, xmmword ptr [rcx]
1408d2bcd: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
1408d2bd1: 48 8b 41 10                 	mov	rax, qword ptr [rcx + 0x10]
1408d2bd5: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1408d2bd9: 48 8b 41 28                 	mov	rax, qword ptr [rcx + 0x28]
1408d2bdd: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d2be4: 0f 10 41 18                 	movups	xmm0, xmmword ptr [rcx + 0x18]
1408d2be8: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1408d2bef: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
1408d2bf6: 48 89 ea                    	mov	rdx, rbp
1408d2bf9: 4c 8d 45 e0                 	lea	r8, [rbp - 0x20]
1408d2bfd: 4c 8d 8d b0 02 00 00        	lea	r9, [rbp + 0x2b0]
1408d2c04: e8 c7 cb ff ff              	call	0x1408cf7d0 <.text+0x8ce7d0>
1408d2c09: 48 c7 85 b0 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x1
1408d2c14: 48 c7 85 b8 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b8], 0x1
1408d2c1f: 0f 28 85 60 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x160]
1408d2c26: 48 8d 85 c0 01 00 00        	lea	rax, [rbp + 0x1c0]
1408d2c2d: 0f 29 40 70                 	movaps	xmmword ptr [rax + 0x70], xmm0
1408d2c31: 0f 28 85 50 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x150]
1408d2c38: 0f 29 40 60                 	movaps	xmmword ptr [rax + 0x60], xmm0
1408d2c3c: 0f 28 85 40 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x140]
1408d2c43: 0f 29 40 50                 	movaps	xmmword ptr [rax + 0x50], xmm0
1408d2c47: 0f 28 85 30 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x130]
1408d2c4e: 0f 29 40 40                 	movaps	xmmword ptr [rax + 0x40], xmm0
1408d2c52: 66 0f 6f 85 f0 00 00 00     	movdqa	xmm0, xmmword ptr [rbp + 0xf0]
1408d2c5a: 66 0f 6f 8d 00 01 00 00     	movdqa	xmm1, xmmword ptr [rbp + 0x100]
1408d2c62: 66 0f 6f 95 10 01 00 00     	movdqa	xmm2, xmmword ptr [rbp + 0x110]
1408d2c6a: 0f 28 9d 20 01 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x120]
1408d2c71: 0f 29 58 30                 	movaps	xmmword ptr [rax + 0x30], xmm3
1408d2c75: 66 0f 7f 50 20              	movdqa	xmmword ptr [rax + 0x20], xmm2
1408d2c7a: 66 0f 7f 48 10              	movdqa	xmmword ptr [rax + 0x10], xmm1
1408d2c7f: 66 0f 7f 00                 	movdqa	xmmword ptr [rax], xmm0
1408d2c83: 0f b6 05 7f 1f d6 02        	movzx	eax, byte ptr [rip + 0x2d61f7f] # 0x143634c09
1408d2c8a: b9 90 00 00 00              	mov	ecx, 0x90
1408d2c8f: ba 10 00 00 00              	mov	edx, 0x10
1408d2c94: e8 77 0f d4 ff              	call	0x140613c10 <.text+0x612c10>
1408d2c99: 48 85 c0                    	test	rax, rax
1408d2c9c: 0f 84 9d 30 00 00           	je	0x1408d5d3f <.text+0x8d4d3f>
1408d2ca2: 41 b8 90 00 00 00           	mov	r8d, 0x90
1408d2ca8: 48 89 c1                    	mov	rcx, rax
1408d2cab: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d2cb2: 48 89 fa                    	mov	rdx, rdi
1408d2cb5: 48 89 c6                    	mov	rsi, rax
1408d2cb8: e8 43 0f 05 02              	call	0x142923c00 <.text+0x2922c00>
1408d2cbd: 0f b6 8d df 02 00 00        	movzx	ecx, byte ptr [rbp + 0x2df]
1408d2cc4: c0 e1 03                    	shl	cl, 0x3
1408d2cc7: 48 8b 85 38 03 00 00        	mov	rax, qword ptr [rbp + 0x338]
1408d2cce: 48 8b 00                    	mov	rax, qword ptr [rax]
1408d2cd1: 48 89 b5 40 03 00 00        	mov	qword ptr [rbp + 0x340], rsi
1408d2cd8: 48 89 b5 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rsi
1408d2cdf: 88 8d b8 00 00 00           	mov	byte ptr [rbp + 0xb8], cl
1408d2ce5: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
1408d2cec: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1408d2cf3: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
1408d2cfa: c6 85 53 03 00 00 01        	mov	byte ptr [rbp + 0x353], 0x1
1408d2d01: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1408d2d08: 48 89 fa                    	mov	rdx, rdi
1408d2d0b: e8 60 e0 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d2d10: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d2d17: 48 8b 8a c8 3d 00 00        	mov	rcx, qword ptr [rdx + 0x3dc8]
1408d2d1e: 48 8b 92 d0 3d 00 00        	mov	rdx, qword ptr [rdx + 0x3dd0]
1408d2d25: 49 89 d0                    	mov	r8, rdx
1408d2d28: 49 21 c0                    	and	r8, rax
1408d2d2b: 49 89 c1                    	mov	r9, rax
1408d2d2e: 49 c1 e9 39                 	shr	r9, 0x39
1408d2d32: 66 41 0f 6e c1              	movd	xmm0, r9d
1408d2d37: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d2d3b: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d2d40: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d2d45: 4c 8d 49 e0                 	lea	r9, [rcx - 0x20]
1408d2d49: 31 f6                       	xor	esi, esi
1408d2d4b: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d2d51: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d2d55: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d2d59: 66 44 0f d7 da              	pmovmskb	r11d, xmm2
1408d2d5e: 45 85 db                    	test	r11d, r11d
1408d2d61: 74 4f                       	je	0x1408d2db2 <.text+0x8d1db2>
1408d2d63: 48 89 b5 a0 01 00 00        	mov	qword ptr [rbp + 0x1a0], rsi
1408d2d6a: f3 41 0f bc f3              	tzcnt	esi, r11d
1408d2d6f: 4c 01 c6                    	add	rsi, r8
1408d2d72: 48 21 d6                    	and	rsi, rdx
1408d2d75: 48 89 f7                    	mov	rdi, rsi
1408d2d78: 48 c1 e7 05                 	shl	rdi, 0x5
1408d2d7c: 49 89 d2                    	mov	r10, rdx
1408d2d7f: 4c 89 ca                    	mov	rdx, r9
1408d2d82: 49 29 f9                    	sub	r9, rdi
1408d2d85: 48 8b bd 30 03 00 00        	mov	rdi, qword ptr [rbp + 0x330]
1408d2d8c: 49 39 39                    	cmp	qword ptr [r9], rdi
1408d2d8f: 74 4d                       	je	0x1408d2dde <.text+0x8d1dde>
1408d2d91: 45 8d 4b ff                 	lea	r9d, [r11 - 0x1]
1408d2d95: 66 45 21 d9                 	and	r9w, r11w
1408d2d99: 45 89 cb                    	mov	r11d, r9d
1408d2d9c: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d2da3: 48 8b b5 a0 01 00 00        	mov	rsi, qword ptr [rbp + 0x1a0]
1408d2daa: 49 89 d1                    	mov	r9, rdx
1408d2dad: 4c 89 d2                    	mov	rdx, r10
1408d2db0: 75 b8                       	jne	0x1408d2d6a <.text+0x8d1d6a>
1408d2db2: 4d 89 ca                    	mov	r10, r9
1408d2db5: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d2dba: 66 44 0f d7 c9              	pmovmskb	r9d, xmm1
1408d2dbf: 45 85 c9                    	test	r9d, r9d
1408d2dc2: 0f 85 5f 01 00 00           	jne	0x1408d2f27 <.text+0x8d1f27>
1408d2dc8: 49 01 f0                    	add	r8, rsi
1408d2dcb: 49 83 c0 10                 	add	r8, 0x10
1408d2dcf: 48 83 c6 10                 	add	rsi, 0x10
1408d2dd3: 49 21 d0                    	and	r8, rdx
1408d2dd6: 4d 89 d1                    	mov	r9, r10
1408d2dd9: e9 6d ff ff ff              	jmp	0x1408d2d4b <.text+0x8d1d4b>
1408d2dde: 48 f7 de                    	neg	rsi
1408d2de1: 48 c1 e6 05                 	shl	rsi, 0x5
1408d2de5: 48 01 f1                    	add	rcx, rsi
1408d2de8: 48 89 8d b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rcx
1408d2def: 48 8b 85 f0 02 00 00        	mov	rax, qword ptr [rbp + 0x2f0]
1408d2df6: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d2dfd: 48 c7 85 b0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2b0], 0x0
1408d2e08: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d2e0f: e9 4b 01 00 00              	jmp	0x1408d2f5f <.text+0x8d1f5f>
1408d2e14: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1408d2e1b: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
1408d2e22: 48 8b 8d 28 03 00 00        	mov	rcx, qword ptr [rbp + 0x328]
1408d2e29: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d2e30: e8 3b df cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d2e35: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d2e3c: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d2e43: 4c 8b 9a 28 3e 00 00        	mov	r11, qword ptr [rdx + 0x3e28]
1408d2e4a: 4c 8b 92 30 3e 00 00        	mov	r10, qword ptr [rdx + 0x3e30]
1408d2e51: 4d 89 d0                    	mov	r8, r10
1408d2e54: 49 21 c0                    	and	r8, rax
1408d2e57: 48 c1 e8 39                 	shr	rax, 0x39
1408d2e5b: 66 0f 6e c0                 	movd	xmm0, eax
1408d2e5f: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d2e63: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d2e68: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d2e6d: 4d 8d 4b e0                 	lea	r9, [r11 - 0x20]
1408d2e71: 31 c9                       	xor	ecx, ecx
1408d2e73: f3 43 0f 6f 0c 03           	movdqu	xmm1, xmmword ptr [r11 + r8]
1408d2e79: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d2e7d: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d2e81: 66 0f d7 f2                 	pmovmskb	esi, xmm2
1408d2e85: 85 f6                       	test	esi, esi
1408d2e87: 74 48                       	je	0x1408d2ed1 <.text+0x8d1ed1>
1408d2e89: 48 89 8d 40 03 00 00        	mov	qword ptr [rbp + 0x340], rcx
1408d2e90: f3 0f bc c6                 	tzcnt	eax, esi
1408d2e94: 4c 01 c0                    	add	rax, r8
1408d2e97: 4c 21 d0                    	and	rax, r10
1408d2e9a: 4c 89 ca                    	mov	rdx, r9
1408d2e9d: 49 89 c1                    	mov	r9, rax
1408d2ea0: 49 c1 e1 05                 	shl	r9, 0x5
1408d2ea4: 48 89 d7                    	mov	rdi, rdx
1408d2ea7: 4c 29 cf                    	sub	rdi, r9
1408d2eaa: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1408d2eb1: 48 39 0f                    	cmp	qword ptr [rdi], rcx
1408d2eb4: 74 3c                       	je	0x1408d2ef2 <.text+0x8d1ef2>
1408d2eb6: 8d 46 ff                    	lea	eax, [rsi - 0x1]
1408d2eb9: 66 21 f0                    	and	ax, si
1408d2ebc: 89 c6                       	mov	esi, eax
1408d2ebe: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d2ec5: 48 8b 8d 40 03 00 00        	mov	rcx, qword ptr [rbp + 0x340]
1408d2ecc: 49 89 d1                    	mov	r9, rdx
1408d2ecf: 75 bf                       	jne	0x1408d2e90 <.text+0x8d1e90>
1408d2ed1: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d2ed6: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d2eda: 85 c0                       	test	eax, eax
1408d2edc: 0f 85 6e 06 00 00           	jne	0x1408d3550 <.text+0x8d2550>
1408d2ee2: 49 01 c8                    	add	r8, rcx
1408d2ee5: 49 83 c0 10                 	add	r8, 0x10
1408d2ee9: 48 83 c1 10                 	add	rcx, 0x10
1408d2eed: 4d 21 d0                    	and	r8, r10
1408d2ef0: eb 81                       	jmp	0x1408d2e73 <.text+0x8d1e73>
1408d2ef2: 49 c1 f9 05                 	sar	r9, 0x5
1408d2ef6: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d2efa: 4d 21 d0                    	and	r8, r10
1408d2efd: f3 43 0f 6f 04 03           	movdqu	xmm0, xmmword ptr [r11 + r8]
1408d2f03: 66 41 0f 74 c2              	pcmpeqb	xmm0, xmm10
1408d2f08: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d2f0c: 85 d2                       	test	edx, edx
1408d2f0e: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d2f15: 0f 84 dc 08 00 00           	je	0x1408d37f7 <.text+0x8d27f7>
1408d2f1b: 66 0f bd d2                 	bsr	dx, dx
1408d2f1f: 83 f2 0f                    	xor	edx, 0xf
1408d2f22: e9 d4 08 00 00              	jmp	0x1408d37fb <.text+0x8d27fb>
1408d2f27: 48 8b 8d 70 03 00 00        	mov	rcx, qword ptr [rbp + 0x370]
1408d2f2e: 48 83 b9 d8 3d 00 00 00     	cmp	qword ptr [rcx + 0x3dd8], 0x0
1408d2f36: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1408d2f3d: 0f 84 83 0e 00 00           	je	0x1408d3dc6 <.text+0x8d2dc6>
1408d2f43: 48 8b 95 f0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2f0]
1408d2f4a: 48 89 95 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rdx
1408d2f51: 48 89 8d b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rcx
1408d2f58: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d2f5f: f3 0f 6f 85 b8 00 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0xb8]
1408d2f67: f3 0f 6f 8d c8 00 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0xc8]
1408d2f6f: 66 0f 7f 8d 00 01 00 00     	movdqa	xmmword ptr [rbp + 0x100], xmm1
1408d2f77: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d2f7f: c7 85 b8 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x1b8], 0x0
1408d2f89: 48 c7 85 b0 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x0
1408d2f94: 44 0f 29 b5 c0 01 00 00     	movaps	xmmword ptr [rbp + 0x1c0], xmm14
1408d2f9c: c6 85 53 03 00 00 00        	mov	byte ptr [rbp + 0x353], 0x0
1408d2fa3: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1408d2fa8: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1408d2faf: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1408d2fb6: 4c 8b 85 70 03 00 00        	mov	r8, qword ptr [rbp + 0x370]
1408d2fbd: 4c 8d 8d f0 00 00 00        	lea	r9, [rbp + 0xf0]
1408d2fc4: e8 87 db 26 00              	call	0x140b40b50 <.text+0xb3fb50>
1408d2fc9: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d2fd0: 48 8b 8d 38 03 00 00        	mov	rcx, qword ptr [rbp + 0x338]
1408d2fd7: 48 8b 95 90 00 00 00        	mov	rdx, qword ptr [rbp + 0x90]
1408d2fde: f3 0f 10 42 dc              	movss	xmm0, dword ptr [rdx - 0x24]
1408d2fe3: 41 0f 2e c3                 	ucomiss	xmm0, xmm11
1408d2fe7: 0f 86 60 03 00 00           	jbe	0x1408d334d <.text+0x8d234d>
1408d2fed: 8b 42 b8                    	mov	eax, dword ptr [rdx - 0x48]
1408d2ff0: 83 f8 01                    	cmp	eax, 0x1
1408d2ff3: 0f 87 54 03 00 00           	ja	0x1408d334d <.text+0x8d234d>
1408d2ff9: 48 8b 09                    	mov	rcx, qword ptr [rcx]
1408d2ffc: 48 89 8d 38 03 00 00        	mov	qword ptr [rbp + 0x338], rcx
1408d3003: f3 0f 10 7a e0              	movss	xmm7, dword ptr [rdx - 0x20]
1408d3008: 48 83 c2 bc                 	add	rdx, -0x44
1408d300c: a8 01                       	test	al, 0x1
1408d300e: 0f 84 71 05 00 00           	je	0x1408d3585 <.text+0x8d2585>
1408d3014: 48 89 d6                    	mov	rsi, rdx
1408d3017: 48 89 d1                    	mov	rcx, rdx
1408d301a: 31 d2                       	xor	edx, edx
1408d301c: e8 df 65 3c 00              	call	0x140c99600 <.text+0xc98600>
1408d3021: f3 0f 11 85 40 02 00 00     	movss	dword ptr [rbp + 0x240], xmm0
1408d3029: 66 0f 7e 8d 44 02 00 00     	movd	dword ptr [rbp + 0x244], xmm1
1408d3031: 48 8d 8d b8 00 00 00        	lea	rcx, [rbp + 0xb8]
1408d3038: 48 8d 95 40 02 00 00        	lea	rdx, [rbp + 0x240]
1408d303f: 0f 28 d7                    	movaps	xmm2, xmm7
1408d3042: e8 99 9d 3c 00              	call	0x140c9cde0 <.text+0xc9bde0>
1408d3047: ba 01 00 00 00              	mov	edx, 0x1
1408d304c: 48 89 f1                    	mov	rcx, rsi
1408d304f: e8 ac 65 3c 00              	call	0x140c99600 <.text+0xc98600>
1408d3054: f3 0f 11 85 70 02 00 00     	movss	dword ptr [rbp + 0x270], xmm0
1408d305c: 66 0f 7e 8d 74 02 00 00     	movd	dword ptr [rbp + 0x274], xmm1
1408d3064: 48 8d 8d c4 00 00 00        	lea	rcx, [rbp + 0xc4]
1408d306b: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
1408d3072: 0f 28 d7                    	movaps	xmm2, xmm7
1408d3075: e8 66 9d 3c 00              	call	0x140c9cde0 <.text+0xc9bde0>
1408d307a: ba 02 00 00 00              	mov	edx, 0x2
1408d307f: 48 89 f1                    	mov	rcx, rsi
1408d3082: e8 79 65 3c 00              	call	0x140c99600 <.text+0xc98600>
1408d3087: f3 0f 11 85 f0 00 00 00     	movss	dword ptr [rbp + 0xf0], xmm0
1408d308f: 66 0f 7e 8d f4 00 00 00     	movd	dword ptr [rbp + 0xf4], xmm1
1408d3097: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1408d309e: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d30a5: 0f 28 d7                    	movaps	xmm2, xmm7
1408d30a8: e8 33 9d 3c 00              	call	0x140c9cde0 <.text+0xc9bde0>
1408d30ad: ba 03 00 00 00              	mov	edx, 0x3
1408d30b2: 48 89 f1                    	mov	rcx, rsi
1408d30b5: e8 46 65 3c 00              	call	0x140c99600 <.text+0xc98600>
1408d30ba: f3 0f 11 85 b0 01 00 00     	movss	dword ptr [rbp + 0x1b0], xmm0
1408d30c2: 66 0f 7e 8d b4 01 00 00     	movd	dword ptr [rbp + 0x1b4], xmm1
1408d30ca: 48 8d 8d dc 00 00 00        	lea	rcx, [rbp + 0xdc]
1408d30d1: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d30d8: 0f 28 d7                    	movaps	xmm2, xmm7
1408d30db: e8 00 9d 3c 00              	call	0x140c9cde0 <.text+0xc9bde0>
1408d30e0: 48 89 f1                    	mov	rcx, rsi
1408d30e3: 31 d2                       	xor	edx, edx
1408d30e5: e8 16 65 3c 00              	call	0x140c99600 <.text+0xc98600>
1408d30ea: ba 01 00 00 00              	mov	edx, 0x1
1408d30ef: 48 89 f1                    	mov	rcx, rsi
1408d30f2: e8 09 65 3c 00              	call	0x140c99600 <.text+0xc98600>
1408d30f7: ba 02 00 00 00              	mov	edx, 0x2
1408d30fc: 48 89 f1                    	mov	rcx, rsi
1408d30ff: e8 fc 64 3c 00              	call	0x140c99600 <.text+0xc98600>
1408d3104: ba 03 00 00 00              	mov	edx, 0x3
1408d3109: 48 89 f1                    	mov	rcx, rsi
1408d310c: e8 ef 64 3c 00              	call	0x140c99600 <.text+0xc98600>
1408d3111: 0f b6 05 f1 1a d6 02        	movzx	eax, byte ptr [rip + 0x2d61af1] # 0x143634c09
1408d3118: b9 18 00 00 00              	mov	ecx, 0x18
1408d311d: ba 04 00 00 00              	mov	edx, 0x4
1408d3122: e8 e9 0a d4 ff              	call	0x140613c10 <.text+0x612c10>
1408d3127: 48 85 c0                    	test	rax, rax
1408d312a: 0f 84 85 2c 00 00           	je	0x1408d5db5 <.text+0x8d4db5>
1408d3130: 48 89 c6                    	mov	rsi, rax
1408d3133: 0f 28 05 66 01 1d 02        	movaps	xmm0, xmmword ptr [rip + 0x21d0166] # 0x142aa32a0
1408d313a: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
1408d313d: 48 b8 02 00 00 00 03 00 00 00       	movabs	rax, 0x300000002
1408d3147: 48 89 46 10                 	mov	qword ptr [rsi + 0x10], rax
1408d314b: 48 c7 85 78 01 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x178], 0x2
1408d3156: 48 89 b5 80 01 00 00        	mov	qword ptr [rbp + 0x180], rsi
1408d315d: 48 c7 85 88 01 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x188], 0x2
1408d3168: 0f b6 05 9a 1a d6 02        	movzx	eax, byte ptr [rip + 0x2d61a9a] # 0x143634c09
1408d316f: b9 30 00 00 00              	mov	ecx, 0x30
1408d3174: ba 04 00 00 00              	mov	edx, 0x4
1408d3179: e8 92 0a d4 ff              	call	0x140613c10 <.text+0x612c10>
1408d317e: 48 85 c0                    	test	rax, rax
1408d3181: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d3188: 48 89 b5 40 03 00 00        	mov	qword ptr [rbp + 0x340], rsi
1408d318f: 0f 84 31 2c 00 00           	je	0x1408d5dc6 <.text+0x8d4dc6>
1408d3195: 0f 10 85 b8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xb8]
1408d319c: f3 0f 6f 8d c8 00 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0xc8]
1408d31a4: 0f 10 95 d8 00 00 00        	movups	xmm2, xmmword ptr [rbp + 0xd8]
1408d31ab: 0f 11 50 20                 	movups	xmmword ptr [rax + 0x20], xmm2
1408d31af: f3 0f 7f 48 10              	movdqu	xmmword ptr [rax + 0x10], xmm1
1408d31b4: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
1408d31b7: 48 c7 85 40 02 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x240], 0x4
1408d31c2: 48 89 85 a0 01 00 00        	mov	qword ptr [rbp + 0x1a0], rax
1408d31c9: 48 89 85 48 02 00 00        	mov	qword ptr [rbp + 0x248], rax
1408d31d0: 48 c7 85 50 02 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x250], 0x4
1408d31db: 0f b6 05 27 1a d6 02        	movzx	eax, byte ptr [rip + 0x2d61a27] # 0x143634c09
1408d31e2: b9 30 00 00 00              	mov	ecx, 0x30
1408d31e7: ba 04 00 00 00              	mov	edx, 0x4
1408d31ec: e8 1f 0a d4 ff              	call	0x140613c10 <.text+0x612c10>
1408d31f1: 48 85 c0                    	test	rax, rax
1408d31f4: 0f 84 eb 2b 00 00           	je	0x1408d5de5 <.text+0x8d4de5>
1408d31fa: 44 0f 11 28                 	movups	xmmword ptr [rax], xmm13
1408d31fe: 0f 28 05 ab 00 1d 02        	movaps	xmm0, xmmword ptr [rip + 0x21d00ab] # 0x142aa32b0
1408d3205: 0f 11 40 10                 	movups	xmmword ptr [rax + 0x10], xmm0
1408d3209: 0f 28 05 b0 00 1d 02        	movaps	xmm0, xmmword ptr [rip + 0x21d00b0] # 0x142aa32c0
1408d3210: 0f 11 40 20                 	movups	xmmword ptr [rax + 0x20], xmm0
1408d3214: 48 c7 85 70 02 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x270], 0x4
1408d321f: 48 89 85 78 02 00 00        	mov	qword ptr [rbp + 0x278], rax
1408d3226: 48 c7 85 80 02 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x280], 0x4
1408d3231: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1408d323b: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
1408d3242: c6 85 1d 03 00 00 00        	mov	byte ptr [rbp + 0x31d], 0x0
1408d3249: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1408d324e: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
1408d3255: 48 8d 95 78 01 00 00        	lea	rdx, [rbp + 0x178]
1408d325c: 4c 8d 85 40 02 00 00        	lea	r8, [rbp + 0x240]
1408d3263: 4c 8d 8d 70 02 00 00        	lea	r9, [rbp + 0x270]
1408d326a: e8 d1 bb ff ff              	call	0x1408cee40 <.text+0x8cde40>
1408d326f: 48 8b b5 e8 00 00 00        	mov	rsi, qword ptr [rbp + 0xe8]
1408d3276: 48 c7 85 b0 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x1
1408d3281: 48 c7 85 b8 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b8], 0x1
1408d328c: 0f 28 85 60 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x160]
1408d3293: 48 8d 85 c0 01 00 00        	lea	rax, [rbp + 0x1c0]
1408d329a: 0f 29 40 70                 	movaps	xmmword ptr [rax + 0x70], xmm0
1408d329e: 0f 28 85 50 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x150]
1408d32a5: 0f 29 40 60                 	movaps	xmmword ptr [rax + 0x60], xmm0
1408d32a9: 0f 28 85 40 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x140]
1408d32b0: 0f 29 40 50                 	movaps	xmmword ptr [rax + 0x50], xmm0
1408d32b4: 0f 28 85 30 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x130]
1408d32bb: 0f 29 40 40                 	movaps	xmmword ptr [rax + 0x40], xmm0
1408d32bf: 0f 28 85 f0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xf0]
1408d32c6: 66 0f 6f 8d 00 01 00 00     	movdqa	xmm1, xmmword ptr [rbp + 0x100]
1408d32ce: 66 0f 6f 95 10 01 00 00     	movdqa	xmm2, xmmword ptr [rbp + 0x110]
1408d32d6: 0f 28 9d 20 01 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x120]
1408d32dd: 0f 29 58 30                 	movaps	xmmword ptr [rax + 0x30], xmm3
1408d32e1: 66 0f 7f 50 20              	movdqa	xmmword ptr [rax + 0x20], xmm2
1408d32e6: 66 0f 7f 48 10              	movdqa	xmmword ptr [rax + 0x10], xmm1
1408d32eb: 0f 29 00                    	movaps	xmmword ptr [rax], xmm0
1408d32ee: 0f b6 05 14 19 d6 02        	movzx	eax, byte ptr [rip + 0x2d61914] # 0x143634c09
1408d32f5: b9 90 00 00 00              	mov	ecx, 0x90
1408d32fa: ba 10 00 00 00              	mov	edx, 0x10
1408d32ff: e8 0c 09 d4 ff              	call	0x140613c10 <.text+0x612c10>
1408d3304: 48 85 c0                    	test	rax, rax
1408d3307: 0f 84 f7 2a 00 00           	je	0x1408d5e04 <.text+0x8d4e04>
1408d330d: 41 b8 90 00 00 00           	mov	r8d, 0x90
1408d3313: 48 89 c1                    	mov	rcx, rax
1408d3316: 48 89 fa                    	mov	rdx, rdi
1408d3319: 48 89 85 40 03 00 00        	mov	qword ptr [rbp + 0x340], rax
1408d3320: e8 db 08 05 02              	call	0x142923c00 <.text+0x2922c00>
1408d3325: 48 8b 85 40 03 00 00        	mov	rax, qword ptr [rbp + 0x340]
1408d332c: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
1408d3333: c6 85 b0 02 00 00 01        	mov	byte ptr [rbp + 0x2b0], 0x1
1408d333a: 48 8b 85 38 03 00 00        	mov	rax, qword ptr [rbp + 0x338]
1408d3341: 48 89 85 b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rax
1408d3348: e9 40 07 00 00              	jmp	0x1408d3a8d <.text+0x8d2a8d>
1408d334d: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1408d3354: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
1408d335b: 48 8b 8d 28 03 00 00        	mov	rcx, qword ptr [rbp + 0x328]
1408d3362: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d3369: e8 02 da cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d336e: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d3375: 4c 8b 9a 28 3e 00 00        	mov	r11, qword ptr [rdx + 0x3e28]
1408d337c: 4c 8b 92 30 3e 00 00        	mov	r10, qword ptr [rdx + 0x3e30]
1408d3383: 4d 89 d0                    	mov	r8, r10
1408d3386: 49 21 c0                    	and	r8, rax
1408d3389: 48 c1 e8 39                 	shr	rax, 0x39
1408d338d: 66 0f 6e c0                 	movd	xmm0, eax
1408d3391: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d3395: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d339a: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d339f: 4d 8d 4b e0                 	lea	r9, [r11 - 0x20]
1408d33a3: 31 c9                       	xor	ecx, ecx
1408d33a5: f3 43 0f 6f 0c 03           	movdqu	xmm1, xmmword ptr [r11 + r8]
1408d33ab: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d33af: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d33b3: 66 0f d7 f2                 	pmovmskb	esi, xmm2
1408d33b7: 85 f6                       	test	esi, esi
1408d33b9: 74 48                       	je	0x1408d3403 <.text+0x8d2403>
1408d33bb: 48 89 8d 38 03 00 00        	mov	qword ptr [rbp + 0x338], rcx
1408d33c2: f3 0f bc c6                 	tzcnt	eax, esi
1408d33c6: 4c 01 c0                    	add	rax, r8
1408d33c9: 4c 21 d0                    	and	rax, r10
1408d33cc: 4c 89 ca                    	mov	rdx, r9
1408d33cf: 49 89 c1                    	mov	r9, rax
1408d33d2: 49 c1 e1 05                 	shl	r9, 0x5
1408d33d6: 48 89 d7                    	mov	rdi, rdx
1408d33d9: 4c 29 cf                    	sub	rdi, r9
1408d33dc: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1408d33e3: 48 39 0f                    	cmp	qword ptr [rdi], rcx
1408d33e6: 74 38                       	je	0x1408d3420 <.text+0x8d2420>
1408d33e8: 8d 46 ff                    	lea	eax, [rsi - 0x1]
1408d33eb: 66 21 f0                    	and	ax, si
1408d33ee: 89 c6                       	mov	esi, eax
1408d33f0: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d33f7: 48 8b 8d 38 03 00 00        	mov	rcx, qword ptr [rbp + 0x338]
1408d33fe: 49 89 d1                    	mov	r9, rdx
1408d3401: 75 bf                       	jne	0x1408d33c2 <.text+0x8d23c2>
1408d3403: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d3408: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d340c: 85 c0                       	test	eax, eax
1408d340e: 75 3e                       	jne	0x1408d344e <.text+0x8d244e>
1408d3410: 49 01 c8                    	add	r8, rcx
1408d3413: 49 83 c0 10                 	add	r8, 0x10
1408d3417: 48 83 c1 10                 	add	rcx, 0x10
1408d341b: 4d 21 d0                    	and	r8, r10
1408d341e: eb 85                       	jmp	0x1408d33a5 <.text+0x8d23a5>
1408d3420: 49 c1 f9 05                 	sar	r9, 0x5
1408d3424: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d3428: 4d 21 d0                    	and	r8, r10
1408d342b: f3 43 0f 6f 04 03           	movdqu	xmm0, xmmword ptr [r11 + r8]
1408d3431: 66 41 0f 74 c2              	pcmpeqb	xmm0, xmm10
1408d3436: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d343a: 85 d2                       	test	edx, edx
1408d343c: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d3443: 74 3e                       	je	0x1408d3483 <.text+0x8d2483>
1408d3445: 66 0f bd d2                 	bsr	dx, dx
1408d3449: 83 f2 0f                    	xor	edx, 0xf
1408d344c: eb 39                       	jmp	0x1408d3487 <.text+0x8d2487>
1408d344e: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d3455: 48 83 b8 38 3e 00 00 00     	cmp	qword ptr [rax + 0x3e38], 0x0
1408d345d: 0f 85 cb 00 00 00           	jne	0x1408d352e <.text+0x8d252e>
1408d3463: ba 01 00 00 00              	mov	edx, 0x1
1408d3468: 48 8b 8d 98 02 00 00        	mov	rcx, qword ptr [rbp + 0x298]
1408d346f: 4c 8b 85 28 03 00 00        	mov	r8, qword ptr [rbp + 0x328]
1408d3476: 41 b1 01                    	mov	r9b, 0x1
1408d3479: e8 22 68 fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d347e: e9 ab 00 00 00              	jmp	0x1408d352e <.text+0x8d252e>
1408d3483: 66 ba 10 00                 	mov	dx, 0x10
1408d3487: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d348e: f3 43 0f 6f 04 0b           	movdqu	xmm0, xmmword ptr [r11 + r9]
1408d3494: 66 41 0f 74 c2              	pcmpeqb	xmm0, xmm10
1408d3499: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d349e: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d34a5: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d34aa: 44 01 d2                    	add	edx, r10d
1408d34ad: 41 b2 80                    	mov	r10b, -0x80
1408d34b0: 66 83 fa 0f                 	cmp	dx, 0xf
1408d34b4: 77 11                       	ja	0x1408d34c7 <.text+0x8d24c7>
1408d34b6: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d34bd: 48 ff 82 38 3e 00 00        	inc	qword ptr [rdx + 0x3e38]
1408d34c4: 41 b2 ff                    	mov	r10b, -0x1
1408d34c7: 48 f7 d8                    	neg	rax
1408d34ca: 48 c1 e0 05                 	shl	rax, 0x5
1408d34ce: 4c 01 d8                    	add	rax, r11
1408d34d1: 47 88 14 0b                 	mov	byte ptr [r11 + r9], r10b
1408d34d5: 47 88 54 03 10              	mov	byte ptr [r11 + r8 + 0x10], r10b
1408d34da: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d34e1: 48 ff 8a 40 3e 00 00        	dec	qword ptr [rdx + 0x3e40]
1408d34e8: 44 8b 40 f8                 	mov	r8d, dword ptr [rax - 0x8]
1408d34ec: 44 89 85 00 01 00 00        	mov	dword ptr [rbp + 0x100], r8d
1408d34f3: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d34f8: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d3500: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d3507: e8 84 60 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d350c: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d3513: 74 19                       	je	0x1408d352e <.text+0x8d252e>
1408d3515: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d351c: f0                          	lock
1408d351d: 48 ff 08                    	dec	qword ptr [rax]
1408d3520: 75 0c                       	jne	0x1408d352e <.text+0x8d252e>
1408d3522: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d3529: e8 f2 90 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d352e: 44 8b 85 10 03 00 00        	mov	r8d, dword ptr [rbp + 0x310]
1408d3535: 44 0f b6 8d 90 01 00 00     	movzx	r9d, byte ptr [rbp + 0x190]
1408d353d: 44 8b 9d e8 02 00 00        	mov	r11d, dword ptr [rbp + 0x2e8]
1408d3544: 48 8b b5 e8 00 00 00        	mov	rsi, qword ptr [rbp + 0xe8]
1408d354b: e9 62 df ff ff              	jmp	0x1408d14b2 <.text+0x8d04b2>
1408d3550: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d3557: 48 83 b8 38 3e 00 00 00     	cmp	qword ptr [rax + 0x3e38], 0x0
1408d355f: 0f 85 6b fa ff ff           	jne	0x1408d2fd0 <.text+0x8d1fd0>
1408d3565: ba 01 00 00 00              	mov	edx, 0x1
1408d356a: 48 8b 8d 98 02 00 00        	mov	rcx, qword ptr [rbp + 0x298]
1408d3571: 4c 8b 85 28 03 00 00        	mov	r8, qword ptr [rbp + 0x328]
1408d3578: 41 b1 01                    	mov	r9b, 0x1
1408d357b: e8 20 67 fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d3580: e9 4b fa ff ff              	jmp	0x1408d2fd0 <.text+0x8d1fd0>
1408d3585: 48 8d 4d 44                 	lea	rcx, [rbp + 0x44]
1408d3589: 48 89 d7                    	mov	rdi, rdx
1408d358c: 0f 28 d7                    	movaps	xmm2, xmm7
1408d358f: e8 4c 98 3c 00              	call	0x140c9cde0 <.text+0xc9bde0>
1408d3594: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d359b: 48 89 fa                    	mov	rdx, rdi
1408d359e: 66 0f 6f d6                 	movdqa	xmm2, xmm6
1408d35a2: 0f 28 df                    	movaps	xmm3, xmm7
1408d35a5: e8 a6 46 3c 00              	call	0x140c97c50 <.text+0xc96c50>
1408d35aa: 48 8d 4d 68                 	lea	rcx, [rbp + 0x68]
1408d35ae: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d35b5: 4c 8d 05 d4 ff 1c 02        	lea	r8, [rip + 0x21cffd4]   # 0x142aa3590
1408d35bc: e8 ff 94 a2 00              	call	0x1412fcac0 <.text+0x12fbac0>
1408d35c1: 45 0f 28 fe                 	movaps	xmm15, xmm14
1408d35c5: c6 85 52 03 00 00 01        	mov	byte ptr [rbp + 0x352], 0x1
1408d35cc: 48 8d 75 c0                 	lea	rsi, [rbp - 0x40]
1408d35d0: 48 89 f1                    	mov	rcx, rsi
1408d35d3: 48 89 fa                    	mov	rdx, rdi
1408d35d6: 66 0f 6f d6                 	movdqa	xmm2, xmm6
1408d35da: e8 e1 45 3c 00              	call	0x140c97bc0 <.text+0xc96bc0>
1408d35df: c6 85 52 03 00 00 01        	mov	byte ptr [rbp + 0x352], 0x1
1408d35e6: 48 8d 4d 50                 	lea	rcx, [rbp + 0x50]
1408d35ea: 48 89 f2                    	mov	rdx, rsi
1408d35ed: 4c 8d 05 9c ff 1c 02        	lea	r8, [rip + 0x21cff9c]   # 0x142aa3590
1408d35f4: e8 57 10 a3 00              	call	0x141304650 <.text+0x1303650>
1408d35f9: 48 8b 75 78                 	mov	rsi, qword ptr [rbp + 0x78]
1408d35fd: 48 3b 75 68                 	cmp	rsi, qword ptr [rbp + 0x68]
1408d3601: 75 17                       	jne	0x1408d361a <.text+0x8d261a>
1408d3603: c6 85 6f 03 00 00 01        	mov	byte ptr [rbp + 0x36f], 0x1
1408d360a: 48 8d 4d 68                 	lea	rcx, [rbp + 0x68]
1408d360e: 48 8d 15 e3 0c 1d 02        	lea	rdx, [rip + 0x21d0ce3]  # 0x142aa42f8
1408d3615: e8 66 bb 2c 00              	call	0x140b9f180 <.text+0xb9e180>
1408d361a: 48 8b 7d 70                 	mov	rdi, qword ptr [rbp + 0x70]
1408d361e: 48 85 f6                    	test	rsi, rsi
1408d3621: 74 18                       	je	0x1408d363b <.text+0x8d263b>
1408d3623: 48 8d 4f 0c                 	lea	rcx, [rdi + 0xc]
1408d3627: 48 8d 04 b5 00 00 00 00     	lea	rax, [4*rsi]
1408d362f: 4c 8d 04 40                 	lea	r8, [rax + 2*rax]
1408d3633: 48 89 fa                    	mov	rdx, rdi
1408d3636: e8 d5 05 05 02              	call	0x142923c10 <.text+0x2922c10>
1408d363b: 8b 45 4c                    	mov	eax, dword ptr [rbp + 0x4c]
1408d363e: 89 47 08                    	mov	dword ptr [rdi + 0x8], eax
1408d3641: 48 8b 45 44                 	mov	rax, qword ptr [rbp + 0x44]
1408d3645: 48 89 07                    	mov	qword ptr [rdi], rax
1408d3648: 48 ff c6                    	inc	rsi
1408d364b: 48 89 75 78                 	mov	qword ptr [rbp + 0x78], rsi
1408d364f: f3 0f 10 7d 44              	movss	xmm7, dword ptr [rbp + 0x44]
1408d3654: f3 44 0f 10 75 4c           	movss	xmm14, dword ptr [rbp + 0x4c]
1408d365a: 48 8b 75 60                 	mov	rsi, qword ptr [rbp + 0x60]
1408d365e: 48 3b 75 50                 	cmp	rsi, qword ptr [rbp + 0x50]
1408d3662: 75 17                       	jne	0x1408d367b <.text+0x8d267b>
1408d3664: c6 85 6f 03 00 00 01        	mov	byte ptr [rbp + 0x36f], 0x1
1408d366b: 48 8d 4d 50                 	lea	rcx, [rbp + 0x50]
1408d366f: 48 8d 15 9a 0c 1d 02        	lea	rdx, [rip + 0x21d0c9a]  # 0x142aa4310
1408d3676: e8 15 e9 2a 00              	call	0x140b81f90 <.text+0xb80f90>
1408d367b: 48 8b 7d 58                 	mov	rdi, qword ptr [rbp + 0x58]
1408d367f: 48 85 f6                    	test	rsi, rsi
1408d3682: 74 14                       	je	0x1408d3698 <.text+0x8d2698>
1408d3684: 48 8d 4f 08                 	lea	rcx, [rdi + 0x8]
1408d3688: 4c 8d 04 f5 00 00 00 00     	lea	r8, [8*rsi]
1408d3690: 48 89 fa                    	mov	rdx, rdi
1408d3693: e8 78 05 05 02              	call	0x142923c10 <.text+0x2922c10>
1408d3698: f3 0f 11 3f                 	movss	dword ptr [rdi], xmm7
1408d369c: f3 44 0f 11 77 04           	movss	dword ptr [rdi + 0x4], xmm14
1408d36a2: 48 ff c6                    	inc	rsi
1408d36a5: 48 89 75 60                 	mov	qword ptr [rbp + 0x60], rsi
1408d36a9: 48 8b 75 78                 	mov	rsi, qword ptr [rbp + 0x78]
1408d36ad: 48 c7 85 b0 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x0
1408d36b8: 48 c7 85 c8 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1c8], 0x1
1408d36c3: 48 89 b5 d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rsi
1408d36ca: c6 85 6f 03 00 00 01        	mov	byte ptr [rbp + 0x36f], 0x1
1408d36d1: 48 8d 8d 78 01 00 00        	lea	rcx, [rbp + 0x178]
1408d36d8: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d36df: 4c 8d 05 aa fe 1c 02        	lea	r8, [rip + 0x21cfeaa]   # 0x142aa3590
1408d36e6: e8 a5 f3 a2 00              	call	0x141302a90 <.text+0x1301a90>
1408d36eb: 48 8b 85 88 01 00 00        	mov	rax, qword ptr [rbp + 0x188]
1408d36f2: 48 89 85 50 02 00 00        	mov	qword ptr [rbp + 0x250], rax
1408d36f9: 0f 10 85 78 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x178]
1408d3700: 0f 29 85 40 02 00 00        	movaps	xmmword ptr [rbp + 0x240], xmm0
1408d3707: 48 8b 45 78                 	mov	rax, qword ptr [rbp + 0x78]
1408d370b: 48 89 85 80 02 00 00        	mov	qword ptr [rbp + 0x280], rax
1408d3712: 0f 10 45 68                 	movups	xmm0, xmmword ptr [rbp + 0x68]
1408d3716: 0f 29 85 70 02 00 00        	movaps	xmmword ptr [rbp + 0x270], xmm0
1408d371d: 48 85 f6                    	test	rsi, rsi
1408d3720: 0f 84 fc 01 00 00           	je	0x1408d3922 <.text+0x8d2922>
1408d3726: 48 8d 04 b5 00 00 00 00     	lea	rax, [4*rsi]
1408d372e: 48 8d 0c 40                 	lea	rcx, [rax + 2*rax]
1408d3732: 0f b6 05 d0 14 d6 02        	movzx	eax, byte ptr [rip + 0x2d614d0] # 0x143634c09
1408d3739: ba 04 00 00 00              	mov	edx, 0x4
1408d373e: 48 89 cf                    	mov	rdi, rcx
1408d3741: e8 ca 04 d4 ff              	call	0x140613c10 <.text+0x612c10>
1408d3746: 48 85 c0                    	test	rax, rax
1408d3749: 0f 84 d7 26 00 00           	je	0x1408d5e26 <.text+0x8d4e26>
1408d374f: 41 b9 01 00 00 00           	mov	r9d, 0x1
1408d3755: 48 89 c1                    	mov	rcx, rax
1408d3758: 48 83 fe 01                 	cmp	rsi, 0x1
1408d375c: 45 0f 28 f7                 	movaps	xmm14, xmm15
1408d3760: f3 44 0f 10 3d 1b 22 05 02  	movss	xmm15, dword ptr [rip + 0x205221b] # 0x142925984
1408d3769: 74 7b                       	je	0x1408d37e6 <.text+0x8d27e6>
1408d376b: ba 01 00 00 00              	mov	edx, 0x1
1408d3770: 48 89 c1                    	mov	rcx, rax
1408d3773: 48 83 fe 04                 	cmp	rsi, 0x4
1408d3777: 76 52                       	jbe	0x1408d37cb <.text+0x8d27cb>
1408d3779: 4c 8d 56 ff                 	lea	r10, [rsi - 0x1]
1408d377d: 4c 89 95 40 03 00 00        	mov	qword ptr [rbp + 0x340], r10
1408d3784: 49 83 e2 fc                 	and	r10, -0x4
1408d3788: 4b 8d 0c 52                 	lea	rcx, [r10 + 2*r10]
1408d378c: 48 8d 0c 88                 	lea	rcx, [rax + 4*rcx]
1408d3790: 49 8d 52 01                 	lea	rdx, [r10 + 0x1]
1408d3794: 49 89 c1                    	mov	r9, rax
1408d3797: 4d 89 d3                    	mov	r11, r10
1408d379a: 49 b8 00 00 80 bf 00 00 00 80       	movabs	r8, -0x7fffffff40800000
1408d37a4: 4d 89 41 10                 	mov	qword ptr [r9 + 0x10], r8
1408d37a8: 45 0f 11 29                 	movups	xmmword ptr [r9], xmm13
1408d37ac: 4d 89 41 28                 	mov	qword ptr [r9 + 0x28], r8
1408d37b0: 45 0f 11 69 18              	movups	xmmword ptr [r9 + 0x18], xmm13
1408d37b5: 49 83 c1 30                 	add	r9, 0x30
1408d37b9: 49 83 c3 fc                 	add	r11, -0x4
1408d37bd: 75 e5                       	jne	0x1408d37a4 <.text+0x8d27a4>
1408d37bf: 49 89 f1                    	mov	r9, rsi
1408d37c2: 4c 39 95 40 03 00 00        	cmp	qword ptr [rbp + 0x340], r10
1408d37c9: 74 1b                       	je	0x1408d37e6 <.text+0x8d27e6>
1408d37cb: 48 ff c2                    	inc	rdx
1408d37ce: f2 44 0f 11 09              	movsd	qword ptr [rcx], xmm9
1408d37d3: c7 41 08 00 00 00 80        	mov	dword ptr [rcx + 0x8], 0x80000000
1408d37da: 48 83 c1 0c                 	add	rcx, 0xc
1408d37de: 48 39 d6                    	cmp	rsi, rdx
1408d37e1: 75 e8                       	jne	0x1408d37cb <.text+0x8d27cb>
1408d37e3: 49 89 f1                    	mov	r9, rsi
1408d37e6: f2 44 0f 11 09              	movsd	qword ptr [rcx], xmm9
1408d37eb: c7 41 08 00 00 00 80        	mov	dword ptr [rcx + 0x8], 0x80000000
1408d37f2: e9 40 01 00 00              	jmp	0x1408d3937 <.text+0x8d2937>
1408d37f7: 66 ba 10 00                 	mov	dx, 0x10
1408d37fb: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d3802: f3 43 0f 6f 04 0b           	movdqu	xmm0, xmmword ptr [r11 + r9]
1408d3808: 66 41 0f 74 c2              	pcmpeqb	xmm0, xmm10
1408d380d: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d3812: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d3819: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d381e: 44 01 d2                    	add	edx, r10d
1408d3821: 41 b2 80                    	mov	r10b, -0x80
1408d3824: 66 83 fa 0f                 	cmp	dx, 0xf
1408d3828: 77 11                       	ja	0x1408d383b <.text+0x8d283b>
1408d382a: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d3831: 48 ff 82 38 3e 00 00        	inc	qword ptr [rdx + 0x3e38]
1408d3838: 41 b2 ff                    	mov	r10b, -0x1
1408d383b: 48 f7 d8                    	neg	rax
1408d383e: 48 c1 e0 05                 	shl	rax, 0x5
1408d3842: 4c 01 d8                    	add	rax, r11
1408d3845: 47 88 14 0b                 	mov	byte ptr [r11 + r9], r10b
1408d3849: 47 88 54 03 10              	mov	byte ptr [r11 + r8 + 0x10], r10b
1408d384e: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d3855: 48 ff 8a 40 3e 00 00        	dec	qword ptr [rdx + 0x3e40]
1408d385c: 44 8b 40 f8                 	mov	r8d, dword ptr [rax - 0x8]
1408d3860: 44 89 85 00 01 00 00        	mov	dword ptr [rbp + 0x100], r8d
1408d3867: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d386c: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d3874: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d387b: e8 10 5d 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d3880: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d3887: 0f 84 43 f7 ff ff           	je	0x1408d2fd0 <.text+0x8d1fd0>
1408d388d: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d3894: f0                          	lock
1408d3895: 48 ff 08                    	dec	qword ptr [rax]
1408d3898: 0f 85 32 f7 ff ff           	jne	0x1408d2fd0 <.text+0x8d1fd0>
1408d389e: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d38a5: e8 76 8d 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d38aa: e9 21 f7 ff ff              	jmp	0x1408d2fd0 <.text+0x8d1fd0>
1408d38af: 48 f7 de                    	neg	rsi
1408d38b2: 48 c1 e6 05                 	shl	rsi, 0x5
1408d38b6: 48 01 f1                    	add	rcx, rsi
1408d38b9: 48 89 8d b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rcx
1408d38c0: 48 8b 85 a0 02 00 00        	mov	rax, qword ptr [rbp + 0x2a0]
1408d38c7: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d38ce: 48 c7 85 b0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2b0], 0x0
1408d38d9: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d38e0: e9 cb 03 00 00              	jmp	0x1408d3cb0 <.text+0x8d2cb0>
1408d38e5: 48 f7 de                    	neg	rsi
1408d38e8: 48 c1 e6 05                 	shl	rsi, 0x5
1408d38ec: 48 01 f1                    	add	rcx, rsi
1408d38ef: 48 89 8d b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rcx
1408d38f6: 48 8b 85 a0 02 00 00        	mov	rax, qword ptr [rbp + 0x2a0]
1408d38fd: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d3904: 48 c7 85 b0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2b0], 0x0
1408d390f: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d3916: 48 8b b5 38 03 00 00        	mov	rsi, qword ptr [rbp + 0x338]
1408d391d: e9 35 04 00 00              	jmp	0x1408d3d57 <.text+0x8d2d57>
1408d3922: b8 04 00 00 00              	mov	eax, 0x4
1408d3927: 45 31 c9                    	xor	r9d, r9d
1408d392a: 45 0f 28 f7                 	movaps	xmm14, xmm15
1408d392e: f3 44 0f 10 3d 4d 20 05 02  	movss	xmm15, dword ptr [rip + 0x205204d] # 0x142925984
1408d3937: 4c 89 8d b8 00 00 00        	mov	qword ptr [rbp + 0xb8], r9
1408d393e: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
1408d3945: 48 89 b5 c8 00 00 00        	mov	qword ptr [rbp + 0xc8], rsi
1408d394c: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1408d3956: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
1408d395d: c6 85 1e 03 00 00 00        	mov	byte ptr [rbp + 0x31e], 0x0
1408d3964: 48 8d 85 b0 01 00 00        	lea	rax, [rbp + 0x1b0]
1408d396b: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408d3970: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
1408d3977: 48 8d 95 40 02 00 00        	lea	rdx, [rbp + 0x240]
1408d397e: 4c 8d 85 70 02 00 00        	lea	r8, [rbp + 0x270]
1408d3985: 4c 8d 8d b8 00 00 00        	lea	r9, [rbp + 0xb8]
1408d398c: e8 af b4 ff ff              	call	0x1408cee40 <.text+0x8cde40>
1408d3991: 48 c7 85 b0 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x1
1408d399c: 48 c7 85 b8 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b8], 0x1
1408d39a7: 0f 28 85 60 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x160]
1408d39ae: 48 8d 85 c0 01 00 00        	lea	rax, [rbp + 0x1c0]
1408d39b5: 0f 29 40 70                 	movaps	xmmword ptr [rax + 0x70], xmm0
1408d39b9: 0f 28 85 50 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x150]
1408d39c0: 0f 29 40 60                 	movaps	xmmword ptr [rax + 0x60], xmm0
1408d39c4: 0f 28 85 40 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x140]
1408d39cb: 0f 29 40 50                 	movaps	xmmword ptr [rax + 0x50], xmm0
1408d39cf: 0f 28 85 30 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x130]
1408d39d6: 0f 29 40 40                 	movaps	xmmword ptr [rax + 0x40], xmm0
1408d39da: 0f 28 85 f0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xf0]
1408d39e1: 66 0f 6f 8d 00 01 00 00     	movdqa	xmm1, xmmword ptr [rbp + 0x100]
1408d39e9: 66 0f 6f 95 10 01 00 00     	movdqa	xmm2, xmmword ptr [rbp + 0x110]
1408d39f1: 0f 28 9d 20 01 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x120]
1408d39f8: 0f 29 58 30                 	movaps	xmmword ptr [rax + 0x30], xmm3
1408d39fc: 66 0f 7f 50 20              	movdqa	xmmword ptr [rax + 0x20], xmm2
1408d3a01: 66 0f 7f 48 10              	movdqa	xmmword ptr [rax + 0x10], xmm1
1408d3a06: 0f 29 00                    	movaps	xmmword ptr [rax], xmm0
1408d3a09: 0f b6 05 f9 11 d6 02        	movzx	eax, byte ptr [rip + 0x2d611f9] # 0x143634c09
1408d3a10: b9 90 00 00 00              	mov	ecx, 0x90
1408d3a15: ba 10 00 00 00              	mov	edx, 0x10
1408d3a1a: e8 f1 01 d4 ff              	call	0x140613c10 <.text+0x612c10>
1408d3a1f: 48 85 c0                    	test	rax, rax
1408d3a22: 48 8b b5 e8 00 00 00        	mov	rsi, qword ptr [rbp + 0xe8]
1408d3a29: 0f 84 e6 23 00 00           	je	0x1408d5e15 <.text+0x8d4e15>
1408d3a2f: 41 b8 90 00 00 00           	mov	r8d, 0x90
1408d3a35: 48 89 c1                    	mov	rcx, rax
1408d3a38: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d3a3f: 48 89 c7                    	mov	rdi, rax
1408d3a42: e8 b9 01 05 02              	call	0x142923c00 <.text+0x2922c00>
1408d3a47: 48 89 bd 40 03 00 00        	mov	qword ptr [rbp + 0x340], rdi
1408d3a4e: 48 89 bd c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rdi
1408d3a55: c6 85 b0 02 00 00 01        	mov	byte ptr [rbp + 0x2b0], 0x1
1408d3a5c: 48 8b 85 38 03 00 00        	mov	rax, qword ptr [rbp + 0x338]
1408d3a63: 48 89 85 b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rax
1408d3a6a: 48 8b 55 50                 	mov	rdx, qword ptr [rbp + 0x50]
1408d3a6e: 48 85 d2                    	test	rdx, rdx
1408d3a71: 74 13                       	je	0x1408d3a86 <.text+0x8d2a86>
1408d3a73: 48 8b 4d 58                 	mov	rcx, qword ptr [rbp + 0x58]
1408d3a77: 48 c1 e2 03                 	shl	rdx, 0x3
1408d3a7b: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d3a81: e8 9a 01 d4 ff              	call	0x140613c20 <.text+0x612c20>
1408d3a86: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d3a8d: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1408d3a94: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
1408d3a9b: c6 85 50 03 00 00 01        	mov	byte ptr [rbp + 0x350], 0x1
1408d3aa2: 48 8b 8d 28 03 00 00        	mov	rcx, qword ptr [rbp + 0x328]
1408d3aa9: 48 89 fa                    	mov	rdx, rdi
1408d3aac: e8 bf d2 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d3ab1: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d3ab8: 48 8b 8a 28 3e 00 00        	mov	rcx, qword ptr [rdx + 0x3e28]
1408d3abf: 48 8b 92 30 3e 00 00        	mov	rdx, qword ptr [rdx + 0x3e30]
1408d3ac6: 49 89 d0                    	mov	r8, rdx
1408d3ac9: 49 21 c0                    	and	r8, rax
1408d3acc: 49 89 c1                    	mov	r9, rax
1408d3acf: 49 c1 e9 39                 	shr	r9, 0x39
1408d3ad3: 66 41 0f 6e c1              	movd	xmm0, r9d
1408d3ad8: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d3adc: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d3ae1: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d3ae6: 4c 8d 51 e0                 	lea	r10, [rcx - 0x20]
1408d3aea: 48 c7 85 38 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x338], 0x0
1408d3af5: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d3afb: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d3aff: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d3b03: 66 44 0f d7 da              	pmovmskb	r11d, xmm2
1408d3b08: 45 85 db                    	test	r11d, r11d
1408d3b0b: 74 3f                       	je	0x1408d3b4c <.text+0x8d2b4c>
1408d3b0d: f3 41 0f bc f3              	tzcnt	esi, r11d
1408d3b12: 4c 01 c6                    	add	rsi, r8
1408d3b15: 48 21 d6                    	and	rsi, rdx
1408d3b18: 48 89 f7                    	mov	rdi, rsi
1408d3b1b: 48 c1 e7 05                 	shl	rdi, 0x5
1408d3b1f: 4d 89 d1                    	mov	r9, r10
1408d3b22: 49 29 f9                    	sub	r9, rdi
1408d3b25: 48 8b bd 30 03 00 00        	mov	rdi, qword ptr [rbp + 0x330]
1408d3b2c: 49 39 39                    	cmp	qword ptr [r9], rdi
1408d3b2f: 74 4b                       	je	0x1408d3b7c <.text+0x8d2b7c>
1408d3b31: 45 8d 4b ff                 	lea	r9d, [r11 - 0x1]
1408d3b35: 66 45 21 d9                 	and	r9w, r11w
1408d3b39: 45 89 cb                    	mov	r11d, r9d
1408d3b3c: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d3b43: 48 8b b5 e8 00 00 00        	mov	rsi, qword ptr [rbp + 0xe8]
1408d3b4a: 75 c1                       	jne	0x1408d3b0d <.text+0x8d2b0d>
1408d3b4c: 66 41 0f 74 ca              	pcmpeqb	xmm1, xmm10
1408d3b51: 66 44 0f d7 c9              	pmovmskb	r9d, xmm1
1408d3b56: 45 85 c9                    	test	r9d, r9d
1408d3b59: 75 5b                       	jne	0x1408d3bb6 <.text+0x8d2bb6>
1408d3b5b: 4c 8b 8d 38 03 00 00        	mov	r9, qword ptr [rbp + 0x338]
1408d3b62: 4d 01 c8                    	add	r8, r9
1408d3b65: 49 83 c0 10                 	add	r8, 0x10
1408d3b69: 49 83 c1 10                 	add	r9, 0x10
1408d3b6d: 4c 89 8d 38 03 00 00        	mov	qword ptr [rbp + 0x338], r9
1408d3b74: 49 21 d0                    	and	r8, rdx
1408d3b77: e9 79 ff ff ff              	jmp	0x1408d3af5 <.text+0x8d2af5>
1408d3b7c: 48 f7 de                    	neg	rsi
1408d3b7f: 48 c1 e6 05                 	shl	rsi, 0x5
1408d3b83: 48 01 f1                    	add	rcx, rsi
1408d3b86: 48 89 8d c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rcx
1408d3b8d: 48 8b 85 98 02 00 00        	mov	rax, qword ptr [rbp + 0x298]
1408d3b94: 48 89 85 c8 00 00 00        	mov	qword ptr [rbp + 0xc8], rax
1408d3b9b: 48 c7 85 b8 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xb8], 0x0
1408d3ba6: 48 8d bd b0 01 00 00        	lea	rdi, [rbp + 0x1b0]
1408d3bad: 48 8b b5 e8 00 00 00        	mov	rsi, qword ptr [rbp + 0xe8]
1408d3bb4: eb 38                       	jmp	0x1408d3bee <.text+0x8d2bee>
1408d3bb6: 48 8b 8d 70 03 00 00        	mov	rcx, qword ptr [rbp + 0x370]
1408d3bbd: 48 83 b9 38 3e 00 00 00     	cmp	qword ptr [rcx + 0x3e38], 0x0
1408d3bc5: 0f 84 2f 02 00 00           	je	0x1408d3dfa <.text+0x8d2dfa>
1408d3bcb: 48 8b 8d 98 02 00 00        	mov	rcx, qword ptr [rbp + 0x298]
1408d3bd2: 48 89 8d b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rcx
1408d3bd9: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1408d3be0: 48 89 8d c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rcx
1408d3be7: 48 89 85 c8 00 00 00        	mov	qword ptr [rbp + 0xc8], rax
1408d3bee: 0f 10 85 b0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2b0]
1408d3bf5: f3 0f 6f 8d c0 02 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0x2c0]
1408d3bfd: 66 0f 7f 8d 00 01 00 00     	movdqa	xmmword ptr [rbp + 0x100], xmm1
1408d3c05: 0f 29 85 f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm0
1408d3c0c: c7 85 b8 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x1b8], 0x0
1408d3c16: 48 c7 85 b0 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x0
1408d3c21: 44 0f 29 b5 c0 01 00 00     	movaps	xmmword ptr [rbp + 0x1c0], xmm14
1408d3c29: c6 85 50 03 00 00 00        	mov	byte ptr [rbp + 0x350], 0x0
1408d3c30: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1408d3c35: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1408d3c3c: 48 8d 95 b8 00 00 00        	lea	rdx, [rbp + 0xb8]
1408d3c43: 4c 8b 85 70 03 00 00        	mov	r8, qword ptr [rbp + 0x370]
1408d3c4a: 4c 8d 8d f0 00 00 00        	lea	r9, [rbp + 0xf0]
1408d3c51: e8 fa ce 26 00              	call	0x140b40b50 <.text+0xb3fb50>
1408d3c56: 44 8b 85 10 03 00 00        	mov	r8d, dword ptr [rbp + 0x310]
1408d3c5d: 44 0f b6 8d 90 01 00 00     	movzx	r9d, byte ptr [rbp + 0x190]
1408d3c65: 44 8b 9d e8 02 00 00        	mov	r11d, dword ptr [rbp + 0x2e8]
1408d3c6c: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
1408d3c73: e9 3a d8 ff ff              	jmp	0x1408d14b2 <.text+0x8d04b2>
1408d3c78: 48 8b 8d 70 03 00 00        	mov	rcx, qword ptr [rbp + 0x370]
1408d3c7f: 48 83 b9 68 3e 00 00 00     	cmp	qword ptr [rcx + 0x3e68], 0x0
1408d3c87: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1408d3c8e: 0f 84 a8 01 00 00           	je	0x1408d3e3c <.text+0x8d2e3c>
1408d3c94: 48 8b 95 a0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2a0]
1408d3c9b: 48 89 95 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rdx
1408d3ca2: 48 89 8d b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rcx
1408d3ca9: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d3cb0: f3 0f 6f 85 b8 00 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0xb8]
1408d3cb8: f3 0f 6f 8d c8 00 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0xc8]
1408d3cc0: 66 0f 7f 8d 00 01 00 00     	movdqa	xmmword ptr [rbp + 0x100], xmm1
1408d3cc8: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d3cd0: c7 85 b8 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x1b8], 0x0
1408d3cda: 48 c7 85 b0 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x0
1408d3ce5: 44 0f 29 b5 c0 01 00 00     	movaps	xmmword ptr [rbp + 0x1c0], xmm14
1408d3ced: c6 85 55 03 00 00 00        	mov	byte ptr [rbp + 0x355], 0x0
1408d3cf4: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1408d3cf9: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1408d3d00: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1408d3d07: 4c 8b 85 70 03 00 00        	mov	r8, qword ptr [rbp + 0x370]
1408d3d0e: 4c 8d 8d f0 00 00 00        	lea	r9, [rbp + 0xf0]
1408d3d15: e8 36 ce 26 00              	call	0x140b40b50 <.text+0xb3fb50>
1408d3d1a: e9 76 ec ff ff              	jmp	0x1408d2995 <.text+0x8d1995>
1408d3d1f: 48 8b 8d 70 03 00 00        	mov	rcx, qword ptr [rbp + 0x370]
1408d3d26: 48 83 b9 68 3e 00 00 00     	cmp	qword ptr [rcx + 0x3e68], 0x0
1408d3d2e: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1408d3d35: 0f 84 35 01 00 00           	je	0x1408d3e70 <.text+0x8d2e70>
1408d3d3b: 48 8b 95 a0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2a0]
1408d3d42: 48 89 95 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rdx
1408d3d49: 48 89 8d b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rcx
1408d3d50: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d3d57: f3 0f 6f 85 b8 00 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0xb8]
1408d3d5f: f3 0f 6f 8d c8 00 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0xc8]
1408d3d67: 66 0f 7f 8d 00 01 00 00     	movdqa	xmmword ptr [rbp + 0x100], xmm1
1408d3d6f: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d3d77: c7 85 b8 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x1b8], 0x0
1408d3d81: 48 c7 85 b0 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x0
1408d3d8c: 44 0f 29 b5 c0 01 00 00     	movaps	xmmword ptr [rbp + 0x1c0], xmm14
1408d3d94: c6 85 54 03 00 00 00        	mov	byte ptr [rbp + 0x354], 0x0
1408d3d9b: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1408d3da0: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1408d3da7: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1408d3dae: 4c 8b 85 70 03 00 00        	mov	r8, qword ptr [rbp + 0x370]
1408d3db5: 4c 8d 8d f0 00 00 00        	lea	r9, [rbp + 0xf0]
1408d3dbc: e8 8f cd 26 00              	call	0x140b40b50 <.text+0xb3fb50>
1408d3dc1: e9 d6 eb ff ff              	jmp	0x1408d299c <.text+0x8d199c>
1408d3dc6: c6 85 53 03 00 00 01        	mov	byte ptr [rbp + 0x353], 0x1
1408d3dcd: ba 01 00 00 00              	mov	edx, 0x1
1408d3dd2: 48 8b 8d f0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f0]
1408d3dd9: 4c 8b 85 e0 02 00 00        	mov	r8, qword ptr [rbp + 0x2e0]
1408d3de0: 41 b1 01                    	mov	r9b, 0x1
1408d3de3: 48 89 c6                    	mov	rsi, rax
1408d3de6: e8 b5 5e fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d3deb: 48 89 f0                    	mov	rax, rsi
1408d3dee: 48 8b 8d b0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b0]
1408d3df5: e9 49 f1 ff ff              	jmp	0x1408d2f43 <.text+0x8d1f43>
1408d3dfa: c6 85 50 03 00 00 01        	mov	byte ptr [rbp + 0x350], 0x1
1408d3e01: ba 01 00 00 00              	mov	edx, 0x1
1408d3e06: 48 8b 8d 98 02 00 00        	mov	rcx, qword ptr [rbp + 0x298]
1408d3e0d: 4c 8b 85 28 03 00 00        	mov	r8, qword ptr [rbp + 0x328]
1408d3e14: 41 b1 01                    	mov	r9b, 0x1
1408d3e17: 48 89 c6                    	mov	rsi, rax
1408d3e1a: e8 81 5e fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d3e1f: 48 89 f0                    	mov	rax, rsi
1408d3e22: 48 8b b5 e8 00 00 00        	mov	rsi, qword ptr [rbp + 0xe8]
1408d3e29: 48 8b 8d b0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b0]
1408d3e30: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1408d3e37: e9 8f fd ff ff              	jmp	0x1408d3bcb <.text+0x8d2bcb>
1408d3e3c: c6 85 55 03 00 00 01        	mov	byte ptr [rbp + 0x355], 0x1
1408d3e43: ba 01 00 00 00              	mov	edx, 0x1
1408d3e48: 48 8b 8d a0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2a0]
1408d3e4f: 4c 8b 85 08 03 00 00        	mov	r8, qword ptr [rbp + 0x308]
1408d3e56: 41 b1 01                    	mov	r9b, 0x1
1408d3e59: 48 89 c6                    	mov	rsi, rax
1408d3e5c: e8 3f 5e fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d3e61: 48 89 f0                    	mov	rax, rsi
1408d3e64: 48 8b 8d b0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b0]
1408d3e6b: e9 24 fe ff ff              	jmp	0x1408d3c94 <.text+0x8d2c94>
1408d3e70: c6 85 54 03 00 00 01        	mov	byte ptr [rbp + 0x354], 0x1
1408d3e77: ba 01 00 00 00              	mov	edx, 0x1
1408d3e7c: 48 8b 8d a0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2a0]
1408d3e83: 4c 8b 85 08 03 00 00        	mov	r8, qword ptr [rbp + 0x308]
1408d3e8a: 41 b1 01                    	mov	r9b, 0x1
1408d3e8d: 48 89 c6                    	mov	rsi, rax
1408d3e90: e8 0b 5e fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d3e95: 48 89 f0                    	mov	rax, rsi
1408d3e98: 48 8b b5 38 03 00 00        	mov	rsi, qword ptr [rbp + 0x338]
1408d3e9f: 48 8b 8d b0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b0]
1408d3ea6: e9 90 fe ff ff              	jmp	0x1408d3d3b <.text+0x8d2d3b>
1408d3eab: 4c 8b 85 b0 00 00 00        	mov	r8, qword ptr [rbp + 0xb0]
1408d3eb2: 49 8b 08                    	mov	rcx, qword ptr [r8]
1408d3eb5: 49 8b 40 10                 	mov	rax, qword ptr [r8 + 0x10]
1408d3eb9: 48 8b 09                    	mov	rcx, qword ptr [rcx]
1408d3ebc: 48 39 48 40                 	cmp	qword ptr [rax + 0x40], rcx
1408d3ec0: 0f 86 a6 00 00 00           	jbe	0x1408d3f6c <.text+0x8d2f6c>
1408d3ec6: 48 8b 50 38                 	mov	rdx, qword ptr [rax + 0x38]
1408d3eca: 48 8b 0c ca                 	mov	rcx, qword ptr [rdx + 8*rcx]
1408d3ece: 48 85 c9                    	test	rcx, rcx
1408d3ed1: 0f 84 95 00 00 00           	je	0x1408d3f6c <.text+0x8d2f6c>
1408d3ed7: 48 8b 40 08                 	mov	rax, qword ptr [rax + 0x8]
1408d3edb: 48 f7 d1                    	not	rcx
1408d3ede: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1408d3ee2: 4d 8b 60 08                 	mov	r12, qword ptr [r8 + 0x8]
1408d3ee6: 4d 8b 04 24                 	mov	r8, qword ptr [r12]
1408d3eea: 31 d2                       	xor	edx, edx
1408d3eec: 4d 89 c1                    	mov	r9, r8
1408d3eef: 4c 2b 4c c8 18              	sub	r9, qword ptr [rax + 8*rcx + 0x18]
1408d3ef4: 4c 0f 42 ca                 	cmovb	r9, rdx
1408d3ef8: 4c 2b 44 c8 38              	sub	r8, qword ptr [rax + 8*rcx + 0x38]
1408d3efd: 4c 0f 42 c2                 	cmovb	r8, rdx
1408d3f01: 4d 89 cd                    	mov	r13, r9
1408d3f04: 49 c1 e5 04                 	shl	r13, 0x4
1408d3f08: 4c 03 6c c8 08              	add	r13, qword ptr [rax + 8*rcx + 0x8]
1408d3f0d: 4c 8b 5c c8 10              	mov	r11, qword ptr [rax + 8*rcx + 0x10]
1408d3f12: 48 8b 74 c8 30              	mov	rsi, qword ptr [rax + 8*rcx + 0x30]
1408d3f17: 4d 29 cb                    	sub	r11, r9
1408d3f1a: 4c 0f 42 da                 	cmovb	r11, rdx
1408d3f1e: 41 b9 08 00 00 00           	mov	r9d, 0x8
1408d3f24: 4d 0f 42 e9                 	cmovb	r13, r9
1408d3f28: 4c 89 c7                    	mov	rdi, r8
1408d3f2b: 48 c1 e7 04                 	shl	rdi, 0x4
1408d3f2f: 48 03 7c c8 28              	add	rdi, qword ptr [rax + 8*rcx + 0x28]
1408d3f34: 4c 29 c6                    	sub	rsi, r8
1408d3f37: 48 0f 42 f2                 	cmovb	rsi, rdx
1408d3f3b: 49 0f 42 f9                 	cmovb	rdi, r9
1408d3f3f: 48 8b 44 c8 40              	mov	rax, qword ptr [rax + 8*rcx + 0x40]
1408d3f44: 48 29 f0                    	sub	rax, rsi
1408d3f47: 4c 29 d8                    	sub	rax, r11
1408d3f4a: 49 89 04 24                 	mov	qword ptr [r12], rax
1408d3f4e: 49 c1 e3 04                 	shl	r11, 0x4
1408d3f52: 4d 01 eb                    	add	r11, r13
1408d3f55: 4c 89 9d 20 03 00 00        	mov	qword ptr [rbp + 0x320], r11
1408d3f5c: 48 c1 e6 04                 	shl	rsi, 0x4
1408d3f60: 48 01 fe                    	add	rsi, rdi
1408d3f63: 48 89 b5 10 03 00 00        	mov	qword ptr [rbp + 0x310], rsi
1408d3f6a: eb 03                       	jmp	0x1408d3f6f <.text+0x8d2f6f>
1408d3f6c: 45 31 e4                    	xor	r12d, r12d
1408d3f6f: 45 31 f6                    	xor	r14d, r14d
1408d3f72: 66 0f 76 f6                 	pcmpeqd	xmm6, xmm6
1408d3f76: e9 bb 00 00 00              	jmp	0x1408d4036 <.text+0x8d3036>
1408d3f7b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1408d3f80: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d3f86: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d3f8a: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d3f8e: 66 0f d7 da                 	pmovmskb	ebx, xmm2
1408d3f92: 85 db                       	test	ebx, ebx
1408d3f94: 74 2a                       	je	0x1408d3fc0 <.text+0x8d2fc0>
1408d3f96: f3 0f bc c3                 	tzcnt	eax, ebx
1408d3f9a: 4c 01 c0                    	add	rax, r8
1408d3f9d: 48 21 d0                    	and	rax, rdx
1408d3fa0: 49 89 c1                    	mov	r9, rax
1408d3fa3: 49 c1 e1 05                 	shl	r9, 0x5
1408d3fa7: 4c 89 d7                    	mov	rdi, r10
1408d3faa: 4c 29 cf                    	sub	rdi, r9
1408d3fad: 4c 39 3f                    	cmp	qword ptr [rdi], r15
1408d3fb0: 74 2e                       	je	0x1408d3fe0 <.text+0x8d2fe0>
1408d3fb2: 8d 43 ff                    	lea	eax, [rbx - 0x1]
1408d3fb5: 66 21 d8                    	and	ax, bx
1408d3fb8: 89 c3                       	mov	ebx, eax
1408d3fba: 75 da                       	jne	0x1408d3f96 <.text+0x8d2f96>
1408d3fbc: 0f 1f 40 00                 	nop	dword ptr [rax]
1408d3fc0: 66 0f 74 ce                 	pcmpeqb	xmm1, xmm6
1408d3fc4: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d3fc8: 85 c0                       	test	eax, eax
1408d3fca: 75 44                       	jne	0x1408d4010 <.text+0x8d3010>
1408d3fcc: 4d 01 d8                    	add	r8, r11
1408d3fcf: 49 83 c0 10                 	add	r8, 0x10
1408d3fd3: 49 83 c3 10                 	add	r11, 0x10
1408d3fd7: 49 21 d0                    	and	r8, rdx
1408d3fda: eb a4                       	jmp	0x1408d3f80 <.text+0x8d2f80>
1408d3fdc: 0f 1f 40 00                 	nop	dword ptr [rax]
1408d3fe0: 49 c1 f9 05                 	sar	r9, 0x5
1408d3fe4: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d3fe8: 49 21 d0                    	and	r8, rdx
1408d3feb: f3 42 0f 6f 04 01           	movdqu	xmm0, xmmword ptr [rcx + r8]
1408d3ff1: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d3ff5: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d3ff9: 85 d2                       	test	edx, edx
1408d3ffb: 74 53                       	je	0x1408d4050 <.text+0x8d3050>
1408d3ffd: 66 0f bd d2                 	bsr	dx, dx
1408d4001: 83 f2 0f                    	xor	edx, 0xf
1408d4004: eb 4e                       	jmp	0x1408d4054 <.text+0x8d3054>
1408d4006: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1408d4010: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d4017: 48 83 b8 68 3e 00 00 00     	cmp	qword ptr [rax + 0x3e68], 0x0
1408d401f: 0f 84 e0 00 00 00           	je	0x1408d4105 <.text+0x8d3105>
1408d4025: 48 89 f7                    	mov	rdi, rsi
1408d4028: 48 8b 8d 58 03 00 00        	mov	rcx, qword ptr [rbp + 0x358]
1408d402f: 48 8b 85 60 03 00 00        	mov	rax, qword ptr [rbp + 0x360]
1408d4036: 4d 85 f6                    	test	r14, r14
1408d4039: 0f 85 09 01 00 00           	jne	0x1408d4148 <.text+0x8d3148>
1408d403f: e9 ec 00 00 00              	jmp	0x1408d4130 <.text+0x8d3130>
1408d4044: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1408d4050: 66 ba 10 00                 	mov	dx, 0x10
1408d4054: f3 42 0f 6f 04 09           	movdqu	xmm0, xmmword ptr [rcx + r9]
1408d405a: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d405e: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d4063: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d406a: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d406f: 44 01 d2                    	add	edx, r10d
1408d4072: 41 b2 80                    	mov	r10b, -0x80
1408d4075: 66 83 fa 0f                 	cmp	dx, 0xf
1408d4079: 77 11                       	ja	0x1408d408c <.text+0x8d308c>
1408d407b: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d4082: 48 ff 82 68 3e 00 00        	inc	qword ptr [rdx + 0x3e68]
1408d4089: 41 b2 ff                    	mov	r10b, -0x1
1408d408c: 48 f7 d8                    	neg	rax
1408d408f: 48 c1 e0 05                 	shl	rax, 0x5
1408d4093: 48 01 c8                    	add	rax, rcx
1408d4096: 46 88 14 09                 	mov	byte ptr [rcx + r9], r10b
1408d409a: 46 88 54 01 10              	mov	byte ptr [rcx + r8 + 0x10], r10b
1408d409f: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d40a6: 48 ff 8a 70 3e 00 00        	dec	qword ptr [rdx + 0x3e70]
1408d40ad: 8b 48 f8                    	mov	ecx, dword ptr [rax - 0x8]
1408d40b0: 89 8d 00 01 00 00           	mov	dword ptr [rbp + 0x100], ecx
1408d40b6: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d40bb: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d40c3: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d40ca: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d40d1: e8 ba 54 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d40d6: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d40dd: 0f 84 42 ff ff ff           	je	0x1408d4025 <.text+0x8d3025>
1408d40e3: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d40ea: f0                          	lock
1408d40eb: 48 ff 08                    	dec	qword ptr [rax]
1408d40ee: 0f 85 31 ff ff ff           	jne	0x1408d4025 <.text+0x8d3025>
1408d40f4: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d40fb: e8 20 85 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d4100: e9 20 ff ff ff              	jmp	0x1408d4025 <.text+0x8d3025>
1408d4105: ba 01 00 00 00              	mov	edx, 0x1
1408d410a: 48 8b 8d a0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2a0]
1408d4111: 4c 8b 85 08 03 00 00        	mov	r8, qword ptr [rbp + 0x308]
1408d4118: 41 b1 01                    	mov	r9b, 0x1
1408d411b: e8 80 5b fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d4120: e9 00 ff ff ff              	jmp	0x1408d4025 <.text+0x8d3025>
1408d4125: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1408d4130: 4d 89 e6                    	mov	r14, r12
1408d4133: 41 bc 00 00 00 00           	mov	r12d, 0x0
1408d4139: 48 89 f8                    	mov	rax, rdi
1408d413c: 4c 89 e9                    	mov	rcx, r13
1408d413f: 4d 85 f6                    	test	r14, r14
1408d4142: 0f 84 5f 04 00 00           	je	0x1408d45a7 <.text+0x8d35a7>
1408d4148: 48 85 c9                    	test	rcx, rcx
1408d414b: 74 23                       	je	0x1408d4170 <.text+0x8d3170>
1408d414d: 48 8d 51 10                 	lea	rdx, [rcx + 0x10]
1408d4151: 48 3b 8d 20 03 00 00        	cmp	rcx, qword ptr [rbp + 0x320]
1408d4158: 74 16                       	je	0x1408d4170 <.text+0x8d3170>
1408d415a: 49 89 c0                    	mov	r8, rax
1408d415d: 48 89 c8                    	mov	rax, rcx
1408d4160: eb 22                       	jmp	0x1408d4184 <.text+0x8d3184>
1408d4162: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1408d4170: 48 85 c0                    	test	rax, rax
1408d4173: 74 bb                       	je	0x1408d4130 <.text+0x8d3130>
1408d4175: 48 3b 85 10 03 00 00        	cmp	rax, qword ptr [rbp + 0x310]
1408d417c: 74 b2                       	je	0x1408d4130 <.text+0x8d3130>
1408d417e: 4c 8d 40 10                 	lea	r8, [rax + 0x10]
1408d4182: 31 d2                       	xor	edx, edx
1408d4184: 49 ff 06                    	inc	qword ptr [r14]
1408d4187: 4c 8b 78 08                 	mov	r15, qword ptr [rax + 0x8]
1408d418b: 4c 89 f8                    	mov	rax, r15
1408d418e: 48 c1 e8 20                 	shr	rax, 0x20
1408d4192: 74 9c                       	je	0x1408d4130 <.text+0x8d3130>
1408d4194: 4c 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], r8
1408d419b: 48 89 95 58 03 00 00        	mov	qword ptr [rbp + 0x358], rdx
1408d41a2: 48 89 fe                    	mov	rsi, rdi
1408d41a5: 4c 89 bd f0 00 00 00        	mov	qword ptr [rbp + 0xf0], r15
1408d41ac: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1408d41b3: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d41ba: e8 b1 cb cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d41bf: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d41c6: 48 8b 8a c8 3d 00 00        	mov	rcx, qword ptr [rdx + 0x3dc8]
1408d41cd: 48 8b 92 d0 3d 00 00        	mov	rdx, qword ptr [rdx + 0x3dd0]
1408d41d4: 49 89 d0                    	mov	r8, rdx
1408d41d7: 49 21 c0                    	and	r8, rax
1408d41da: 48 c1 e8 39                 	shr	rax, 0x39
1408d41de: 66 0f 6e c0                 	movd	xmm0, eax
1408d41e2: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d41e6: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d41eb: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d41f0: 48 8d 79 e0                 	lea	rdi, [rcx - 0x20]
1408d41f4: 45 31 db                    	xor	r11d, r11d
1408d41f7: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d41fd: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d4201: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d4205: 66 0f d7 da                 	pmovmskb	ebx, xmm2
1408d4209: 85 db                       	test	ebx, ebx
1408d420b: 74 33                       	je	0x1408d4240 <.text+0x8d3240>
1408d420d: f3 0f bc c3                 	tzcnt	eax, ebx
1408d4211: 4c 01 c0                    	add	rax, r8
1408d4214: 48 21 d0                    	and	rax, rdx
1408d4217: 49 89 c1                    	mov	r9, rax
1408d421a: 49 c1 e1 05                 	shl	r9, 0x5
1408d421e: 49 89 fa                    	mov	r10, rdi
1408d4221: 4d 29 ca                    	sub	r10, r9
1408d4224: 4d 39 3a                    	cmp	qword ptr [r10], r15
1408d4227: 74 37                       	je	0x1408d4260 <.text+0x8d3260>
1408d4229: 8d 43 ff                    	lea	eax, [rbx - 0x1]
1408d422c: 66 21 d8                    	and	ax, bx
1408d422f: 89 c3                       	mov	ebx, eax
1408d4231: 75 da                       	jne	0x1408d420d <.text+0x8d320d>
1408d4233: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1408d4240: 66 0f 74 ce                 	pcmpeqb	xmm1, xmm6
1408d4244: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d4248: 85 c0                       	test	eax, eax
1408d424a: 75 44                       	jne	0x1408d4290 <.text+0x8d3290>
1408d424c: 4d 01 d8                    	add	r8, r11
1408d424f: 49 83 c0 10                 	add	r8, 0x10
1408d4253: 49 83 c3 10                 	add	r11, 0x10
1408d4257: 49 21 d0                    	and	r8, rdx
1408d425a: eb 9b                       	jmp	0x1408d41f7 <.text+0x8d31f7>
1408d425c: 0f 1f 40 00                 	nop	dword ptr [rax]
1408d4260: 49 c1 f9 05                 	sar	r9, 0x5
1408d4264: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d4268: 49 21 d0                    	and	r8, rdx
1408d426b: f3 42 0f 6f 04 01           	movdqu	xmm0, xmmword ptr [rcx + r8]
1408d4271: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d4275: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d4279: 85 d2                       	test	edx, edx
1408d427b: 74 53                       	je	0x1408d42d0 <.text+0x8d32d0>
1408d427d: 66 0f bd d2                 	bsr	dx, dx
1408d4281: 83 f2 0f                    	xor	edx, 0xf
1408d4284: eb 4e                       	jmp	0x1408d42d4 <.text+0x8d32d4>
1408d4286: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1408d4290: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d4297: 48 83 b8 d8 3d 00 00 00     	cmp	qword ptr [rax + 0x3dd8], 0x0
1408d429f: 0f 85 db 00 00 00           	jne	0x1408d4380 <.text+0x8d3380>
1408d42a5: ba 01 00 00 00              	mov	edx, 0x1
1408d42aa: 48 8b 8d f0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f0]
1408d42b1: 4c 8b 85 e0 02 00 00        	mov	r8, qword ptr [rbp + 0x2e0]
1408d42b8: 41 b1 01                    	mov	r9b, 0x1
1408d42bb: e8 e0 59 fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d42c0: e9 bb 00 00 00              	jmp	0x1408d4380 <.text+0x8d3380>
1408d42c5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1408d42d0: 66 ba 10 00                 	mov	dx, 0x10
1408d42d4: f3 42 0f 6f 04 09           	movdqu	xmm0, xmmword ptr [rcx + r9]
1408d42da: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d42de: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d42e3: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d42ea: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d42ef: 44 01 d2                    	add	edx, r10d
1408d42f2: 41 b2 80                    	mov	r10b, -0x80
1408d42f5: 66 83 fa 0f                 	cmp	dx, 0xf
1408d42f9: 77 11                       	ja	0x1408d430c <.text+0x8d330c>
1408d42fb: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d4302: 48 ff 82 d8 3d 00 00        	inc	qword ptr [rdx + 0x3dd8]
1408d4309: 41 b2 ff                    	mov	r10b, -0x1
1408d430c: 48 f7 d8                    	neg	rax
1408d430f: 48 c1 e0 05                 	shl	rax, 0x5
1408d4313: 48 01 c8                    	add	rax, rcx
1408d4316: 46 88 14 09                 	mov	byte ptr [rcx + r9], r10b
1408d431a: 46 88 54 01 10              	mov	byte ptr [rcx + r8 + 0x10], r10b
1408d431f: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d4326: 48 ff 8a e0 3d 00 00        	dec	qword ptr [rdx + 0x3de0]
1408d432d: 8b 48 f8                    	mov	ecx, dword ptr [rax - 0x8]
1408d4330: 89 8d 00 01 00 00           	mov	dword ptr [rbp + 0x100], ecx
1408d4336: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d433b: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d4343: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d434a: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d4351: e8 3a 52 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d4356: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d435d: 74 21                       	je	0x1408d4380 <.text+0x8d3380>
1408d435f: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d4366: f0                          	lock
1408d4367: 48 ff 08                    	dec	qword ptr [rax]
1408d436a: 75 14                       	jne	0x1408d4380 <.text+0x8d3380>
1408d436c: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d4373: e8 a8 82 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d4378: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1408d4380: 4c 89 bd f0 00 00 00        	mov	qword ptr [rbp + 0xf0], r15
1408d4387: 48 8b 8d 28 03 00 00        	mov	rcx, qword ptr [rbp + 0x328]
1408d438e: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d4395: e8 d6 c9 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d439a: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d43a1: 48 8b 8a 28 3e 00 00        	mov	rcx, qword ptr [rdx + 0x3e28]
1408d43a8: 48 8b 92 30 3e 00 00        	mov	rdx, qword ptr [rdx + 0x3e30]
1408d43af: 49 89 d0                    	mov	r8, rdx
1408d43b2: 49 21 c0                    	and	r8, rax
1408d43b5: 48 c1 e8 39                 	shr	rax, 0x39
1408d43b9: 66 0f 6e c0                 	movd	xmm0, eax
1408d43bd: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d43c1: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d43c6: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d43cb: 4c 8d 51 e0                 	lea	r10, [rcx - 0x20]
1408d43cf: 45 31 db                    	xor	r11d, r11d
1408d43d2: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d43d8: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d43dc: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d43e0: 66 0f d7 da                 	pmovmskb	ebx, xmm2
1408d43e4: 85 db                       	test	ebx, ebx
1408d43e6: 74 28                       	je	0x1408d4410 <.text+0x8d3410>
1408d43e8: f3 0f bc c3                 	tzcnt	eax, ebx
1408d43ec: 4c 01 c0                    	add	rax, r8
1408d43ef: 48 21 d0                    	and	rax, rdx
1408d43f2: 49 89 c1                    	mov	r9, rax
1408d43f5: 49 c1 e1 05                 	shl	r9, 0x5
1408d43f9: 4c 89 d7                    	mov	rdi, r10
1408d43fc: 4c 29 cf                    	sub	rdi, r9
1408d43ff: 4c 39 3f                    	cmp	qword ptr [rdi], r15
1408d4402: 74 2c                       	je	0x1408d4430 <.text+0x8d3430>
1408d4404: 8d 43 ff                    	lea	eax, [rbx - 0x1]
1408d4407: 66 21 d8                    	and	ax, bx
1408d440a: 89 c3                       	mov	ebx, eax
1408d440c: 75 da                       	jne	0x1408d43e8 <.text+0x8d33e8>
1408d440e: 66 90                       	nop
1408d4410: 66 0f 74 ce                 	pcmpeqb	xmm1, xmm6
1408d4414: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d4418: 85 c0                       	test	eax, eax
1408d441a: 75 44                       	jne	0x1408d4460 <.text+0x8d3460>
1408d441c: 4d 01 d8                    	add	r8, r11
1408d441f: 49 83 c0 10                 	add	r8, 0x10
1408d4423: 49 83 c3 10                 	add	r11, 0x10
1408d4427: 49 21 d0                    	and	r8, rdx
1408d442a: eb a6                       	jmp	0x1408d43d2 <.text+0x8d33d2>
1408d442c: 0f 1f 40 00                 	nop	dword ptr [rax]
1408d4430: 49 c1 f9 05                 	sar	r9, 0x5
1408d4434: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d4438: 49 21 d0                    	and	r8, rdx
1408d443b: f3 42 0f 6f 04 01           	movdqu	xmm0, xmmword ptr [rcx + r8]
1408d4441: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d4445: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d4449: 85 d2                       	test	edx, edx
1408d444b: 74 53                       	je	0x1408d44a0 <.text+0x8d34a0>
1408d444d: 66 0f bd d2                 	bsr	dx, dx
1408d4451: 83 f2 0f                    	xor	edx, 0xf
1408d4454: eb 4e                       	jmp	0x1408d44a4 <.text+0x8d34a4>
1408d4456: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1408d4460: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d4467: 48 83 b8 38 3e 00 00 00     	cmp	qword ptr [rax + 0x3e38], 0x0
1408d446f: 0f 85 db 00 00 00           	jne	0x1408d4550 <.text+0x8d3550>
1408d4475: ba 01 00 00 00              	mov	edx, 0x1
1408d447a: 48 8b 8d 98 02 00 00        	mov	rcx, qword ptr [rbp + 0x298]
1408d4481: 4c 8b 85 28 03 00 00        	mov	r8, qword ptr [rbp + 0x328]
1408d4488: 41 b1 01                    	mov	r9b, 0x1
1408d448b: e8 10 58 fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d4490: e9 bb 00 00 00              	jmp	0x1408d4550 <.text+0x8d3550>
1408d4495: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1408d44a0: 66 ba 10 00                 	mov	dx, 0x10
1408d44a4: f3 42 0f 6f 04 09           	movdqu	xmm0, xmmword ptr [rcx + r9]
1408d44aa: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d44ae: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d44b3: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d44ba: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d44bf: 44 01 d2                    	add	edx, r10d
1408d44c2: 41 b2 80                    	mov	r10b, -0x80
1408d44c5: 66 83 fa 0f                 	cmp	dx, 0xf
1408d44c9: 77 11                       	ja	0x1408d44dc <.text+0x8d34dc>
1408d44cb: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d44d2: 48 ff 82 38 3e 00 00        	inc	qword ptr [rdx + 0x3e38]
1408d44d9: 41 b2 ff                    	mov	r10b, -0x1
1408d44dc: 48 f7 d8                    	neg	rax
1408d44df: 48 c1 e0 05                 	shl	rax, 0x5
1408d44e3: 48 01 c8                    	add	rax, rcx
1408d44e6: 46 88 14 09                 	mov	byte ptr [rcx + r9], r10b
1408d44ea: 46 88 54 01 10              	mov	byte ptr [rcx + r8 + 0x10], r10b
1408d44ef: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d44f6: 48 ff 8a 40 3e 00 00        	dec	qword ptr [rdx + 0x3e40]
1408d44fd: 8b 48 f8                    	mov	ecx, dword ptr [rax - 0x8]
1408d4500: 89 8d 00 01 00 00           	mov	dword ptr [rbp + 0x100], ecx
1408d4506: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d450b: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d4513: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d451a: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d4521: e8 6a 50 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d4526: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d452d: 74 21                       	je	0x1408d4550 <.text+0x8d3550>
1408d452f: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d4536: f0                          	lock
1408d4537: 48 ff 08                    	dec	qword ptr [rax]
1408d453a: 75 14                       	jne	0x1408d4550 <.text+0x8d3550>
1408d453c: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d4543: e8 d8 80 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d4548: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1408d4550: 4c 89 bd f0 00 00 00        	mov	qword ptr [rbp + 0xf0], r15
1408d4557: 48 8b 8d 08 03 00 00        	mov	rcx, qword ptr [rbp + 0x308]
1408d455e: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d4565: e8 06 c8 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d456a: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d4571: 48 8b 8a 58 3e 00 00        	mov	rcx, qword ptr [rdx + 0x3e58]
1408d4578: 48 8b 92 60 3e 00 00        	mov	rdx, qword ptr [rdx + 0x3e60]
1408d457f: 49 89 d0                    	mov	r8, rdx
1408d4582: 49 21 c0                    	and	r8, rax
1408d4585: 48 c1 e8 39                 	shr	rax, 0x39
1408d4589: 66 0f 6e c0                 	movd	xmm0, eax
1408d458d: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d4591: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d4596: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d459b: 4c 8d 51 e0                 	lea	r10, [rcx - 0x20]
1408d459f: 45 31 db                    	xor	r11d, r11d
1408d45a2: e9 d9 f9 ff ff              	jmp	0x1408d3f80 <.text+0x8d2f80>
1408d45a7: 48 8b 95 a0 04 00 00        	mov	rdx, qword ptr [rbp + 0x4a0]
1408d45ae: 48 8b 0a                    	mov	rcx, qword ptr [rdx]
1408d45b1: 48 8b 5a 08                 	mov	rbx, qword ptr [rdx + 0x8]
1408d45b5: 44 8b 42 14                 	mov	r8d, dword ptr [rdx + 0x14]
1408d45b9: 4c 8b b9 f8 00 00 00        	mov	r15, qword ptr [rcx + 0xf8]
1408d45c0: 48 8b 81 00 01 00 00        	mov	rax, qword ptr [rcx + 0x100]
1408d45c7: 4d 8d 24 87                 	lea	r12, [r15 + 4*rax]
1408d45cb: 48 89 8d 20 03 00 00        	mov	qword ptr [rbp + 0x320], rcx
1408d45d2: 44 0f b6 89 08 01 00 00     	movzx	r9d, byte ptr [rcx + 0x108]
1408d45da: 48 8b 8d 70 03 00 00        	mov	rcx, qword ptr [rbp + 0x370]
1408d45e1: 48 8d 81 f8 3d 00 00        	lea	rax, [rcx + 0x3df8]
1408d45e8: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1408d45ef: 44 89 85 58 03 00 00        	mov	dword ptr [rbp + 0x358], r8d
1408d45f6: 44 89 c0                    	mov	eax, r8d
1408d45f9: 2b 42 10                    	sub	eax, dword ptr [rdx + 0x10]
1408d45fc: 3d 00 b0 33 c2              	cmp	eax, 0xc233b000
1408d4601: ba 00 b0 33 c2              	mov	edx, 0xc233b000
1408d4606: 0f 42 d0                    	cmovb	edx, eax
1408d4609: 89 95 60 03 00 00           	mov	dword ptr [rbp + 0x360], edx
1408d460f: 48 8d 81 18 3e 00 00        	lea	rax, [rcx + 0x3e18]
1408d4616: 48 89 85 10 03 00 00        	mov	qword ptr [rbp + 0x310], rax
1408d461d: b8 08 00 00 00              	mov	eax, 0x8
1408d4622: 48 89 85 28 03 00 00        	mov	qword ptr [rbp + 0x328], rax
1408d4629: 45 31 ed                    	xor	r13d, r13d
1408d462c: 66 0f 76 f6                 	pcmpeqd	xmm6, xmm6
1408d4630: be 08 00 00 00              	mov	esi, 0x8
1408d4635: 31 ff                       	xor	edi, edi
1408d4637: 45 31 f6                    	xor	r14d, r14d
1408d463a: 44 88 8d 08 03 00 00        	mov	byte ptr [rbp + 0x308], r9b
1408d4641: 45 84 c9                    	test	r9b, r9b
1408d4644: 0f 85 f6 00 00 00           	jne	0x1408d4740 <.text+0x8d3740>
1408d464a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1408d4650: 49 39 fe                    	cmp	r14, rdi
1408d4653: 0f 85 a7 00 00 00           	jne	0x1408d4700 <.text+0x8d3700>
1408d4659: 4d 39 e7                    	cmp	r15, r12
1408d465c: 0f 84 c9 03 00 00           	je	0x1408d4a2b <.text+0x8d3a2b>
1408d4662: 41 8b 0f                    	mov	ecx, dword ptr [r15]
1408d4665: 49 83 c7 04                 	add	r15, 0x4
1408d4669: 48 8b 83 00 01 00 00        	mov	rax, qword ptr [rbx + 0x100]
1408d4670: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
1408d4674: 48 c1 e1 05                 	shl	rcx, 0x5
1408d4678: 48 83 7c 08 58 00           	cmp	qword ptr [rax + rcx + 0x58], 0x0
1408d467e: 74 10                       	je	0x1408d4690 <.text+0x8d3690>
1408d4680: 48 8d 3c 08                 	lea	rdi, [rax + rcx]
1408d4684: 4c 8d 34 08                 	lea	r14, [rax + rcx]
1408d4688: 49 83 c6 58                 	add	r14, 0x58
1408d468c: eb 30                       	jmp	0x1408d46be <.text+0x8d36be>
1408d468e: 66 90                       	nop
1408d4690: 4d 39 e7                    	cmp	r15, r12
1408d4693: 0f 84 92 03 00 00           	je	0x1408d4a2b <.text+0x8d3a2b>
1408d4699: 41 8b 07                    	mov	eax, dword ptr [r15]
1408d469c: 49 83 c7 04                 	add	r15, 0x4
1408d46a0: 48 8b bb 00 01 00 00        	mov	rdi, qword ptr [rbx + 0x100]
1408d46a7: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
1408d46ab: 48 c1 e0 05                 	shl	rax, 0x5
1408d46af: 48 83 7c 07 58 00           	cmp	qword ptr [rdi + rax + 0x58], 0x0
1408d46b5: 74 d9                       	je	0x1408d4690 <.text+0x8d3690>
1408d46b7: 48 01 c7                    	add	rdi, rax
1408d46ba: 4c 8d 77 58                 	lea	r14, [rdi + 0x58]
1408d46be: 8b 87 94 00 00 00           	mov	eax, dword ptr [rdi + 0x94]
1408d46c4: 48 8d 0c c0                 	lea	rcx, [rax + 8*rax]
1408d46c8: 48 c1 e1 03                 	shl	rcx, 0x3
1408d46cc: 48 03 8b a8 01 00 00        	add	rcx, qword ptr [rbx + 0x1a8]
1408d46d3: 48 8b 85 20 03 00 00        	mov	rax, qword ptr [rbp + 0x320]
1408d46da: 48 8b 90 18 01 00 00        	mov	rdx, qword ptr [rax + 0x118]
1408d46e1: e8 5a c0 42 00              	call	0x140d00740 <.text+0xcff740>
1408d46e6: 49 89 c5                    	mov	r13, rax
1408d46e9: 48 8b 4f 50                 	mov	rcx, qword ptr [rdi + 0x50]
1408d46ed: 49 8b 3e                    	mov	rdi, qword ptr [r14]
1408d46f0: 45 31 f6                    	xor	r14d, r14d
1408d46f3: eb 0e                       	jmp	0x1408d4703 <.text+0x8d3703>
1408d46f5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1408d4700: 48 89 f1                    	mov	rcx, rsi
1408d4703: 4c 89 f0                    	mov	rax, r14
1408d4706: 48 c1 e0 04                 	shl	rax, 0x4
1408d470a: 48 89 ce                    	mov	rsi, rcx
1408d470d: 8b 4c 01 08                 	mov	ecx, dword ptr [rcx + rax + 0x8]
1408d4711: 8b 95 58 03 00 00           	mov	edx, dword ptr [rbp + 0x358]
1408d4717: 41 2b 54 8d 00              	sub	edx, dword ptr [r13 + 4*rcx]
1408d471c: 3b 95 60 03 00 00           	cmp	edx, dword ptr [rbp + 0x360]
1408d4722: 72 0d                       	jb	0x1408d4731 <.text+0x8d3731>
1408d4724: 49 ff c6                    	inc	r14
1408d4727: 49 39 fe                    	cmp	r14, rdi
1408d472a: 75 d4                       	jne	0x1408d4700 <.text+0x8d3700>
1408d472c: e9 28 ff ff ff              	jmp	0x1408d4659 <.text+0x8d3659>
1408d4731: 48 01 f0                    	add	rax, rsi
1408d4734: e9 cb 00 00 00              	jmp	0x1408d4804 <.text+0x8d3804>
1408d4739: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1408d4740: 49 39 fe                    	cmp	r14, rdi
1408d4743: 0f 85 8c 00 00 00           	jne	0x1408d47d5 <.text+0x8d37d5>
1408d4749: 4d 39 e7                    	cmp	r15, r12
1408d474c: 0f 84 d9 02 00 00           	je	0x1408d4a2b <.text+0x8d3a2b>
1408d4752: 41 8b 0f                    	mov	ecx, dword ptr [r15]
1408d4755: 49 83 c7 04                 	add	r15, 0x4
1408d4759: 48 8b 83 a8 01 00 00        	mov	rax, qword ptr [rbx + 0x1a8]
1408d4760: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1408d4764: 48 83 7c c8 10 00           	cmp	qword ptr [rax + 8*rcx + 0x10], 0x0
1408d476a: 74 14                       	je	0x1408d4780 <.text+0x8d3780>
1408d476c: 48 8d 3c c8                 	lea	rdi, [rax + 8*rcx]
1408d4770: 4c 8d 34 c8                 	lea	r14, [rax + 8*rcx]
1408d4774: 49 83 c6 10                 	add	r14, 0x10
1408d4778: eb 31                       	jmp	0x1408d47ab <.text+0x8d37ab>
1408d477a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1408d4780: 4d 39 e7                    	cmp	r15, r12
1408d4783: 0f 84 a2 02 00 00           	je	0x1408d4a2b <.text+0x8d3a2b>
1408d4789: 41 8b 0f                    	mov	ecx, dword ptr [r15]
1408d478c: 49 83 c7 04                 	add	r15, 0x4
1408d4790: 48 8b 83 a8 01 00 00        	mov	rax, qword ptr [rbx + 0x1a8]
1408d4797: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1408d479b: 48 83 7c c8 10 00           	cmp	qword ptr [rax + 8*rcx + 0x10], 0x0
1408d47a1: 74 dd                       	je	0x1408d4780 <.text+0x8d3780>
1408d47a3: 48 8d 3c c8                 	lea	rdi, [rax + 8*rcx]
1408d47a7: 4c 8d 77 10                 	lea	r14, [rdi + 0x10]
1408d47ab: 48 8b 85 20 03 00 00        	mov	rax, qword ptr [rbp + 0x320]
1408d47b2: 48 8b 90 18 01 00 00        	mov	rdx, qword ptr [rax + 0x118]
1408d47b9: 48 89 f9                    	mov	rcx, rdi
1408d47bc: e8 7f bf 42 00              	call	0x140d00740 <.text+0xcff740>
1408d47c1: 49 89 c5                    	mov	r13, rax
1408d47c4: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1408d47c8: 48 89 85 28 03 00 00        	mov	qword ptr [rbp + 0x328], rax
1408d47cf: 49 8b 3e                    	mov	rdi, qword ptr [r14]
1408d47d2: 45 31 f6                    	xor	r14d, r14d
1408d47d5: 44 89 f0                    	mov	eax, r14d
1408d47d8: 8b 8d 58 03 00 00           	mov	ecx, dword ptr [rbp + 0x358]
1408d47de: 41 2b 4c 85 00              	sub	ecx, dword ptr [r13 + 4*rax]
1408d47e3: 3b 8d 60 03 00 00           	cmp	ecx, dword ptr [rbp + 0x360]
1408d47e9: 72 0e                       	jb	0x1408d47f9 <.text+0x8d37f9>
1408d47eb: 49 ff c6                    	inc	r14
1408d47ee: 49 39 fe                    	cmp	r14, rdi
1408d47f1: 0f 84 52 ff ff ff           	je	0x1408d4749 <.text+0x8d3749>
1408d47f7: eb dc                       	jmp	0x1408d47d5 <.text+0x8d37d5>
1408d47f9: 48 8b 85 28 03 00 00        	mov	rax, qword ptr [rbp + 0x328]
1408d4800: 4a 8d 04 f0                 	lea	rax, [rax + 8*r14]
1408d4804: 48 8b 08                    	mov	rcx, qword ptr [rax]
1408d4807: 48 89 c8                    	mov	rax, rcx
1408d480a: 48 c1 e8 20                 	shr	rax, 0x20
1408d480e: 0f 84 17 02 00 00           	je	0x1408d4a2b <.text+0x8d3a2b>
1408d4814: 48 89 8d e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rcx
1408d481b: 48 89 8d f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rcx
1408d4822: 48 8b 8d 10 03 00 00        	mov	rcx, qword ptr [rbp + 0x310]
1408d4829: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d4830: e8 3b c5 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d4835: 49 ff c6                    	inc	r14
1408d4838: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d483f: 48 8b 8a f8 3d 00 00        	mov	rcx, qword ptr [rdx + 0x3df8]
1408d4846: 48 8b 92 00 3e 00 00        	mov	rdx, qword ptr [rdx + 0x3e00]
1408d484d: 49 89 d3                    	mov	r11, rdx
1408d4850: 49 21 c3                    	and	r11, rax
1408d4853: 48 c1 e8 39                 	shr	rax, 0x39
1408d4857: 66 0f 6e c0                 	movd	xmm0, eax
1408d485b: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d485f: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d4864: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d4869: 4c 8d 51 e0                 	lea	r10, [rcx - 0x20]
1408d486d: 45 31 c9                    	xor	r9d, r9d
1408d4870: f3 42 0f 6f 0c 19           	movdqu	xmm1, xmmword ptr [rcx + r11]
1408d4876: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d487a: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d487e: 66 44 0f d7 c2              	pmovmskb	r8d, xmm2
1408d4883: 45 85 c0                    	test	r8d, r8d
1408d4886: 74 64                       	je	0x1408d48ec <.text+0x8d38ec>
1408d4888: 48 89 8d e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], rcx
1408d488f: 4c 89 8d 58 02 00 00        	mov	qword ptr [rbp + 0x258], r9
1408d4896: 4c 89 9d b0 00 00 00        	mov	qword ptr [rbp + 0xb0], r11
1408d489d: f3 41 0f bc c0              	tzcnt	eax, r8d
1408d48a2: 4c 01 d8                    	add	rax, r11
1408d48a5: 48 21 d0                    	and	rax, rdx
1408d48a8: 49 89 c1                    	mov	r9, rax
1408d48ab: 49 c1 e1 05                 	shl	r9, 0x5
1408d48af: 4d 89 d3                    	mov	r11, r10
1408d48b2: 4d 29 ca                    	sub	r10, r9
1408d48b5: 48 89 d1                    	mov	rcx, rdx
1408d48b8: 48 8b 95 e0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e0]
1408d48bf: 49 39 12                    	cmp	qword ptr [r10], rdx
1408d48c2: 74 47                       	je	0x1408d490b <.text+0x8d390b>
1408d48c4: 41 8d 40 ff                 	lea	eax, [r8 - 0x1]
1408d48c8: 66 44 21 c0                 	and	ax, r8w
1408d48cc: 41 89 c0                    	mov	r8d, eax
1408d48cf: 48 89 ca                    	mov	rdx, rcx
1408d48d2: 48 8b 8d e8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e8]
1408d48d9: 4d 89 da                    	mov	r10, r11
1408d48dc: 4c 8b 8d 58 02 00 00        	mov	r9, qword ptr [rbp + 0x258]
1408d48e3: 4c 8b 9d b0 00 00 00        	mov	r11, qword ptr [rbp + 0xb0]
1408d48ea: 75 b1                       	jne	0x1408d489d <.text+0x8d389d>
1408d48ec: 66 0f 74 ce                 	pcmpeqb	xmm1, xmm6
1408d48f0: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d48f4: 85 c0                       	test	eax, eax
1408d48f6: 75 40                       	jne	0x1408d4938 <.text+0x8d3938>
1408d48f8: 4d 01 cb                    	add	r11, r9
1408d48fb: 49 83 c3 10                 	add	r11, 0x10
1408d48ff: 49 83 c1 10                 	add	r9, 0x10
1408d4903: 49 21 d3                    	and	r11, rdx
1408d4906: e9 65 ff ff ff              	jmp	0x1408d4870 <.text+0x8d3870>
1408d490b: 49 c1 f9 05                 	sar	r9, 0x5
1408d490f: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d4913: 49 21 c8                    	and	r8, rcx
1408d4916: 48 8b 8d e8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e8]
1408d491d: f3 42 0f 6f 04 01           	movdqu	xmm0, xmmword ptr [rcx + r8]
1408d4923: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d4927: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d492b: 85 d2                       	test	edx, edx
1408d492d: 74 3e                       	je	0x1408d496d <.text+0x8d396d>
1408d492f: 66 0f bd d2                 	bsr	dx, dx
1408d4933: 83 f2 0f                    	xor	edx, 0xf
1408d4936: eb 39                       	jmp	0x1408d4971 <.text+0x8d3971>
1408d4938: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d493f: 48 83 b8 08 3e 00 00 00     	cmp	qword ptr [rax + 0x3e08], 0x0
1408d4947: 0f 85 c8 00 00 00           	jne	0x1408d4a15 <.text+0x8d3a15>
1408d494d: ba 01 00 00 00              	mov	edx, 0x1
1408d4952: 48 8b 8d 90 02 00 00        	mov	rcx, qword ptr [rbp + 0x290]
1408d4959: 4c 8b 85 10 03 00 00        	mov	r8, qword ptr [rbp + 0x310]
1408d4960: 41 b1 01                    	mov	r9b, 0x1
1408d4963: e8 38 53 fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d4968: e9 a8 00 00 00              	jmp	0x1408d4a15 <.text+0x8d3a15>
1408d496d: 66 ba 10 00                 	mov	dx, 0x10
1408d4971: f3 42 0f 6f 04 09           	movdqu	xmm0, xmmword ptr [rcx + r9]
1408d4977: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d497b: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d4980: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d4987: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d498c: 44 01 d2                    	add	edx, r10d
1408d498f: 41 b2 80                    	mov	r10b, -0x80
1408d4992: 66 83 fa 0f                 	cmp	dx, 0xf
1408d4996: 77 11                       	ja	0x1408d49a9 <.text+0x8d39a9>
1408d4998: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d499f: 48 ff 82 08 3e 00 00        	inc	qword ptr [rdx + 0x3e08]
1408d49a6: 41 b2 ff                    	mov	r10b, -0x1
1408d49a9: 48 f7 d8                    	neg	rax
1408d49ac: 48 c1 e0 05                 	shl	rax, 0x5
1408d49b0: 48 01 c8                    	add	rax, rcx
1408d49b3: 46 88 14 09                 	mov	byte ptr [rcx + r9], r10b
1408d49b7: 46 88 54 01 10              	mov	byte ptr [rcx + r8 + 0x10], r10b
1408d49bc: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d49c3: 48 ff 8a 10 3e 00 00        	dec	qword ptr [rdx + 0x3e10]
1408d49ca: 8b 48 f8                    	mov	ecx, dword ptr [rax - 0x8]
1408d49cd: 89 8d 00 01 00 00           	mov	dword ptr [rbp + 0x100], ecx
1408d49d3: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d49d8: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d49e0: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d49e7: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d49ee: e8 9d 4b 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d49f3: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d49fa: 74 19                       	je	0x1408d4a15 <.text+0x8d3a15>
1408d49fc: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d4a03: f0                          	lock
1408d4a04: 48 ff 08                    	dec	qword ptr [rax]
1408d4a07: 75 0c                       	jne	0x1408d4a15 <.text+0x8d3a15>
1408d4a09: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d4a10: e8 0b 7c 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d4a15: 44 0f b6 8d 08 03 00 00     	movzx	r9d, byte ptr [rbp + 0x308]
1408d4a1d: 45 84 c9                    	test	r9b, r9b
1408d4a20: 0f 84 2a fc ff ff           	je	0x1408d4650 <.text+0x8d3650>
1408d4a26: e9 15 fd ff ff              	jmp	0x1408d4740 <.text+0x8d3740>
1408d4a2b: 48 8b 8d 90 04 00 00        	mov	rcx, qword ptr [rbp + 0x490]
1408d4a32: 48 8b 39                    	mov	rdi, qword ptr [rcx]
1408d4a35: 4c 8b 61 08                 	mov	r12, qword ptr [rcx + 0x8]
1408d4a39: 8b 51 14                    	mov	edx, dword ptr [rcx + 0x14]
1408d4a3c: 4c 8b bf f8 00 00 00        	mov	r15, qword ptr [rdi + 0xf8]
1408d4a43: 48 8b 87 00 01 00 00        	mov	rax, qword ptr [rdi + 0x100]
1408d4a4a: 4d 8d 2c 87                 	lea	r13, [r15 + 4*rax]
1408d4a4e: 44 0f b6 87 08 01 00 00     	movzx	r8d, byte ptr [rdi + 0x108]
1408d4a56: 89 95 60 03 00 00           	mov	dword ptr [rbp + 0x360], edx
1408d4a5c: 89 d0                       	mov	eax, edx
1408d4a5e: 2b 41 10                    	sub	eax, dword ptr [rcx + 0x10]
1408d4a61: 3d 00 b0 33 c2              	cmp	eax, 0xc233b000
1408d4a66: b9 00 b0 33 c2              	mov	ecx, 0xc233b000
1408d4a6b: 0f 42 c8                    	cmovb	ecx, eax
1408d4a6e: 89 8d 20 03 00 00           	mov	dword ptr [rbp + 0x320], ecx
1408d4a74: b8 08 00 00 00              	mov	eax, 0x8
1408d4a79: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1408d4a80: 48 c7 85 e0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2e0], 0x0
1408d4a8b: 66 0f 76 f6                 	pcmpeqd	xmm6, xmm6
1408d4a8f: 0f 57 ff                    	xorps	xmm7, xmm7
1408d4a92: f3 44 0f 10 05 69 0e 05 02  	movss	xmm8, dword ptr [rip + 0x2050e69] # 0x142925904
1408d4a9b: 48 c7 85 e8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2e8], 0x0
1408d4aa6: 48 89 85 b0 00 00 00        	mov	qword ptr [rbp + 0xb0], rax
1408d4aad: 45 31 f6                    	xor	r14d, r14d
1408d4ab0: 31 db                       	xor	ebx, ebx
1408d4ab2: 48 c7 85 28 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x328], 0x0
1408d4abd: c7 85 58 03 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x358], 0x0
1408d4ac7: 48 c7 85 08 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x308], 0x0
1408d4ad2: 31 f6                       	xor	esi, esi
1408d4ad4: 44 88 85 90 01 00 00        	mov	byte ptr [rbp + 0x190], r8b
1408d4adb: 45 84 c0                    	test	r8b, r8b
1408d4ade: 0f 84 3c 02 00 00           	je	0x1408d4d20 <.text+0x8d3d20>
1408d4ae4: 4c 8b 85 58 02 00 00        	mov	r8, qword ptr [rbp + 0x258]
1408d4aeb: eb 06                       	jmp	0x1408d4af3 <.text+0x8d3af3>
1408d4aed: 0f 1f 00                    	nop	dword ptr [rax]
1408d4af0: 48 ff c3                    	inc	rbx
1408d4af3: 4c 39 f3                    	cmp	rbx, r14
1408d4af6: 0f 85 c1 01 00 00           	jne	0x1408d4cbd <.text+0x8d3cbd>
1408d4afc: 4d 39 ef                    	cmp	r15, r13
1408d4aff: 0f 84 0d 0d 00 00           	je	0x1408d5812 <.text+0x8d4812>
1408d4b05: 41 8b 0f                    	mov	ecx, dword ptr [r15]
1408d4b08: 49 83 c7 04                 	add	r15, 0x4
1408d4b0c: 49 8b 84 24 a8 01 00 00     	mov	rax, qword ptr [r12 + 0x1a8]
1408d4b14: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1408d4b18: 48 83 7c c8 10 00           	cmp	qword ptr [rax + 8*rcx + 0x10], 0x0
1408d4b1e: 74 30                       	je	0x1408d4b50 <.text+0x8d3b50>
1408d4b20: 4c 8d 34 c8                 	lea	r14, [rax + 8*rcx]
1408d4b24: 48 8d 34 c8                 	lea	rsi, [rax + 8*rcx]
1408d4b28: 48 83 c6 10                 	add	rsi, 0x10
1408d4b2c: 48 8b 8f 10 01 00 00        	mov	rcx, qword ptr [rdi + 0x110]
1408d4b33: 49 8b 46 40                 	mov	rax, qword ptr [r14 + 0x40]
1408d4b37: 48 39 c8                    	cmp	rax, rcx
1408d4b3a: 0f 86 90 00 00 00           	jbe	0x1408d4bd0 <.text+0x8d3bd0>
1408d4b40: eb 4a                       	jmp	0x1408d4b8c <.text+0x8d3b8c>
1408d4b42: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1408d4b50: 4d 39 ef                    	cmp	r15, r13
1408d4b53: 0f 84 b9 0c 00 00           	je	0x1408d5812 <.text+0x8d4812>
1408d4b59: 41 8b 0f                    	mov	ecx, dword ptr [r15]
1408d4b5c: 49 83 c7 04                 	add	r15, 0x4
1408d4b60: 49 8b 84 24 a8 01 00 00     	mov	rax, qword ptr [r12 + 0x1a8]
1408d4b68: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1408d4b6c: 48 83 7c c8 10 00           	cmp	qword ptr [rax + 8*rcx + 0x10], 0x0
1408d4b72: 74 dc                       	je	0x1408d4b50 <.text+0x8d3b50>
1408d4b74: 4c 8d 34 c8                 	lea	r14, [rax + 8*rcx]
1408d4b78: 49 8d 76 10                 	lea	rsi, [r14 + 0x10]
1408d4b7c: 48 8b 8f 10 01 00 00        	mov	rcx, qword ptr [rdi + 0x110]
1408d4b83: 49 8b 46 40                 	mov	rax, qword ptr [r14 + 0x40]
1408d4b87: 48 39 c8                    	cmp	rax, rcx
1408d4b8a: 76 44                       	jbe	0x1408d4bd0 <.text+0x8d3bd0>
1408d4b8c: 49 8b 56 38                 	mov	rdx, qword ptr [r14 + 0x38]
1408d4b90: 48 8b 0c ca                 	mov	rcx, qword ptr [rdx + 8*rcx]
1408d4b94: 48 85 c9                    	test	rcx, rcx
1408d4b97: 74 37                       	je	0x1408d4bd0 <.text+0x8d3bd0>
1408d4b99: 49 8b 56 18                 	mov	rdx, qword ptr [r14 + 0x18]
1408d4b9d: 48 f7 d1                    	not	rcx
1408d4ba0: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1408d4ba4: 48 c1 e1 04                 	shl	rcx, 0x4
1408d4ba8: 48 8b 4c 0a 10              	mov	rcx, qword ptr [rdx + rcx + 0x10]
1408d4bad: 48 89 8d e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rcx
1408d4bb4: 48 8b 8f 18 01 00 00        	mov	rcx, qword ptr [rdi + 0x118]
1408d4bbb: 48 39 c8                    	cmp	rax, rcx
1408d4bbe: 77 27                       	ja	0x1408d4be7 <.text+0x8d3be7>
1408d4bc0: eb 5e                       	jmp	0x1408d4c20 <.text+0x8d3c20>
1408d4bc2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1408d4bd0: 48 c7 85 e0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2e0], 0x0
1408d4bdb: 48 8b 8f 18 01 00 00        	mov	rcx, qword ptr [rdi + 0x118]
1408d4be2: 48 39 c8                    	cmp	rax, rcx
1408d4be5: 76 39                       	jbe	0x1408d4c20 <.text+0x8d3c20>
1408d4be7: 49 8b 56 38                 	mov	rdx, qword ptr [r14 + 0x38]
1408d4beb: 48 8b 0c ca                 	mov	rcx, qword ptr [rdx + 8*rcx]
1408d4bef: 48 85 c9                    	test	rcx, rcx
1408d4bf2: 74 2c                       	je	0x1408d4c20 <.text+0x8d3c20>
1408d4bf4: 49 8b 56 18                 	mov	rdx, qword ptr [r14 + 0x18]
1408d4bf8: 48 f7 d1                    	not	rcx
1408d4bfb: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1408d4bff: 48 c1 e1 04                 	shl	rcx, 0x4
1408d4c03: 48 8b 4c 0a 10              	mov	rcx, qword ptr [rdx + rcx + 0x10]
1408d4c08: 48 89 8d e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], rcx
1408d4c0f: 48 8b 97 20 01 00 00        	mov	rdx, qword ptr [rdi + 0x120]
1408d4c16: 48 39 d0                    	cmp	rax, rdx
1408d4c19: 76 55                       	jbe	0x1408d4c70 <.text+0x8d3c70>
1408d4c1b: eb 1a                       	jmp	0x1408d4c37 <.text+0x8d3c37>
1408d4c1d: 0f 1f 00                    	nop	dword ptr [rax]
1408d4c20: 48 c7 85 e8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2e8], 0x0
1408d4c2b: 48 8b 97 20 01 00 00        	mov	rdx, qword ptr [rdi + 0x120]
1408d4c32: 48 39 d0                    	cmp	rax, rdx
1408d4c35: 76 39                       	jbe	0x1408d4c70 <.text+0x8d3c70>
1408d4c37: 49 8b 4e 38                 	mov	rcx, qword ptr [r14 + 0x38]
1408d4c3b: 48 83 3c d1 00              	cmp	qword ptr [rcx + 8*rdx], 0x0
1408d4c40: 74 2e                       	je	0x1408d4c70 <.text+0x8d3c70>
1408d4c42: 4c 89 f1                    	mov	rcx, r14
1408d4c45: e8 26 bb 42 00              	call	0x140d00770 <.text+0xcff770>
1408d4c4a: 48 89 85 28 03 00 00        	mov	qword ptr [rbp + 0x328], rax
1408d4c51: 49 8b 46 40                 	mov	rax, qword ptr [r14 + 0x40]
1408d4c55: b1 01                       	mov	cl, 0x1
1408d4c57: 89 8d 58 03 00 00           	mov	dword ptr [rbp + 0x358], ecx
1408d4c5d: 48 8b 97 28 01 00 00        	mov	rdx, qword ptr [rdi + 0x128]
1408d4c64: 48 39 d0                    	cmp	rax, rdx
1408d4c67: 76 47                       	jbe	0x1408d4cb0 <.text+0x8d3cb0>
1408d4c69: eb 1b                       	jmp	0x1408d4c86 <.text+0x8d3c86>
1408d4c6b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1408d4c70: c7 85 58 03 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x358], 0x0
1408d4c7a: 48 8b 97 28 01 00 00        	mov	rdx, qword ptr [rdi + 0x128]
1408d4c81: 48 39 d0                    	cmp	rax, rdx
1408d4c84: 76 2a                       	jbe	0x1408d4cb0 <.text+0x8d3cb0>
1408d4c86: 49 8b 46 38                 	mov	rax, qword ptr [r14 + 0x38]
1408d4c8a: 48 83 3c d0 00              	cmp	qword ptr [rax + 8*rdx], 0x0
1408d4c8f: 74 1f                       	je	0x1408d4cb0 <.text+0x8d3cb0>
1408d4c91: 4c 89 f1                    	mov	rcx, r14
1408d4c94: e8 d7 ba 42 00              	call	0x140d00770 <.text+0xcff770>
1408d4c99: 48 89 85 08 03 00 00        	mov	qword ptr [rbp + 0x308], rax
1408d4ca0: b0 01                       	mov	al, 0x1
1408d4ca2: eb 0e                       	jmp	0x1408d4cb2 <.text+0x8d3cb2>
1408d4ca4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1408d4cb0: 31 c0                       	xor	eax, eax
1408d4cb2: 4d 8b 46 08                 	mov	r8, qword ptr [r14 + 0x8]
1408d4cb6: 4c 8b 36                    	mov	r14, qword ptr [rsi]
1408d4cb9: 31 db                       	xor	ebx, ebx
1408d4cbb: 89 c6                       	mov	esi, eax
1408d4cbd: f6 85 58 03 00 00 01        	test	byte ptr [rbp + 0x358], 0x1
1408d4cc4: 74 1a                       	je	0x1408d4ce0 <.text+0x8d3ce0>
1408d4cc6: 89 d9                       	mov	ecx, ebx
1408d4cc8: 8b 85 60 03 00 00           	mov	eax, dword ptr [rbp + 0x360]
1408d4cce: 48 8b 95 28 03 00 00        	mov	rdx, qword ptr [rbp + 0x328]
1408d4cd5: 2b 04 8a                    	sub	eax, dword ptr [rdx + 4*rcx]
1408d4cd8: 3b 85 20 03 00 00           	cmp	eax, dword ptr [rbp + 0x320]
1408d4cde: 72 28                       	jb	0x1408d4d08 <.text+0x8d3d08>
1408d4ce0: 40 f6 c6 01                 	test	sil, 0x1
1408d4ce4: 0f 84 06 fe ff ff           	je	0x1408d4af0 <.text+0x8d3af0>
1408d4cea: 89 d9                       	mov	ecx, ebx
1408d4cec: 8b 85 60 03 00 00           	mov	eax, dword ptr [rbp + 0x360]
1408d4cf2: 48 8b 95 08 03 00 00        	mov	rdx, qword ptr [rbp + 0x308]
1408d4cf9: 2b 04 8a                    	sub	eax, dword ptr [rdx + 4*rcx]
1408d4cfc: 3b 85 20 03 00 00           	cmp	eax, dword ptr [rbp + 0x320]
1408d4d02: 0f 83 e8 fd ff ff           	jae	0x1408d4af0 <.text+0x8d3af0>
1408d4d08: 89 b5 f0 02 00 00           	mov	dword ptr [rbp + 0x2f0], esi
1408d4d0e: 4c 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], r8
1408d4d15: 49 8d 04 d8                 	lea	rax, [r8 + 8*rbx]
1408d4d19: e9 72 02 00 00              	jmp	0x1408d4f90 <.text+0x8d3f90>
1408d4d1e: 66 90                       	nop
1408d4d20: 4c 8b 85 b0 00 00 00        	mov	r8, qword ptr [rbp + 0xb0]
1408d4d27: eb 0a                       	jmp	0x1408d4d33 <.text+0x8d3d33>
1408d4d29: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1408d4d30: 48 ff c3                    	inc	rbx
1408d4d33: 4c 39 f3                    	cmp	rbx, r14
1408d4d36: 0f 85 e1 01 00 00           	jne	0x1408d4f1d <.text+0x8d3f1d>
1408d4d3c: 4d 39 ef                    	cmp	r15, r13
1408d4d3f: 0f 84 cd 0a 00 00           	je	0x1408d5812 <.text+0x8d4812>
1408d4d45: 41 8b 0f                    	mov	ecx, dword ptr [r15]
1408d4d48: 49 83 c7 04                 	add	r15, 0x4
1408d4d4c: 49 8b 84 24 00 01 00 00     	mov	rax, qword ptr [r12 + 0x100]
1408d4d54: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
1408d4d58: 48 c1 e1 05                 	shl	rcx, 0x5
1408d4d5c: 48 83 7c 08 58 00           	cmp	qword ptr [rax + rcx + 0x58], 0x0
1408d4d62: 74 1c                       	je	0x1408d4d80 <.text+0x8d3d80>
1408d4d64: 48 8d 34 08                 	lea	rsi, [rax + rcx]
1408d4d68: 48 8d 1c 08                 	lea	rbx, [rax + rcx]
1408d4d6c: 48 83 c3 58                 	add	rbx, 0x58
1408d4d70: eb 3d                       	jmp	0x1408d4daf <.text+0x8d3daf>
1408d4d72: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1408d4d80: 4d 39 ef                    	cmp	r15, r13
1408d4d83: 0f 84 89 0a 00 00           	je	0x1408d5812 <.text+0x8d4812>
1408d4d89: 41 8b 07                    	mov	eax, dword ptr [r15]
1408d4d8c: 49 83 c7 04                 	add	r15, 0x4
1408d4d90: 49 8b b4 24 00 01 00 00     	mov	rsi, qword ptr [r12 + 0x100]
1408d4d98: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
1408d4d9c: 48 c1 e0 05                 	shl	rax, 0x5
1408d4da0: 48 83 7c 06 58 00           	cmp	qword ptr [rsi + rax + 0x58], 0x0
1408d4da6: 74 d8                       	je	0x1408d4d80 <.text+0x8d3d80>
1408d4da8: 48 01 c6                    	add	rsi, rax
1408d4dab: 48 8d 5e 58                 	lea	rbx, [rsi + 0x58]
1408d4daf: 8b 86 94 00 00 00           	mov	eax, dword ptr [rsi + 0x94]
1408d4db5: 49 8b 94 24 a8 01 00 00     	mov	rdx, qword ptr [r12 + 0x1a8]
1408d4dbd: 48 8d 04 c0                 	lea	rax, [rax + 8*rax]
1408d4dc1: 4c 8d 34 c2                 	lea	r14, [rdx + 8*rax]
1408d4dc5: 48 8b 8f 10 01 00 00        	mov	rcx, qword ptr [rdi + 0x110]
1408d4dcc: 48 8b 44 c2 40              	mov	rax, qword ptr [rdx + 8*rax + 0x40]
1408d4dd1: 48 39 c8                    	cmp	rax, rcx
1408d4dd4: 76 3a                       	jbe	0x1408d4e10 <.text+0x8d3e10>
1408d4dd6: 49 8b 56 38                 	mov	rdx, qword ptr [r14 + 0x38]
1408d4dda: 48 8b 0c ca                 	mov	rcx, qword ptr [rdx + 8*rcx]
1408d4dde: 48 85 c9                    	test	rcx, rcx
1408d4de1: 74 2d                       	je	0x1408d4e10 <.text+0x8d3e10>
1408d4de3: 49 8b 56 18                 	mov	rdx, qword ptr [r14 + 0x18]
1408d4de7: 48 f7 d1                    	not	rcx
1408d4dea: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1408d4dee: 48 c1 e1 04                 	shl	rcx, 0x4
1408d4df2: 48 8b 4c 0a 10              	mov	rcx, qword ptr [rdx + rcx + 0x10]
1408d4df7: 48 89 8d e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rcx
1408d4dfe: 48 8b 8f 18 01 00 00        	mov	rcx, qword ptr [rdi + 0x118]
1408d4e05: 48 39 c8                    	cmp	rax, rcx
1408d4e08: 77 1d                       	ja	0x1408d4e27 <.text+0x8d3e27>
1408d4e0a: eb 64                       	jmp	0x1408d4e70 <.text+0x8d3e70>
1408d4e0c: 0f 1f 40 00                 	nop	dword ptr [rax]
1408d4e10: 48 c7 85 e0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2e0], 0x0
1408d4e1b: 48 8b 8f 18 01 00 00        	mov	rcx, qword ptr [rdi + 0x118]
1408d4e22: 48 39 c8                    	cmp	rax, rcx
1408d4e25: 76 49                       	jbe	0x1408d4e70 <.text+0x8d3e70>
1408d4e27: 49 8b 46 38                 	mov	rax, qword ptr [r14 + 0x38]
1408d4e2b: 48 8b 04 c8                 	mov	rax, qword ptr [rax + 8*rcx]
1408d4e2f: 48 85 c0                    	test	rax, rax
1408d4e32: 74 3c                       	je	0x1408d4e70 <.text+0x8d3e70>
1408d4e34: 49 8b 4e 18                 	mov	rcx, qword ptr [r14 + 0x18]
1408d4e38: 48 f7 d0                    	not	rax
1408d4e3b: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1408d4e3f: 48 c1 e0 04                 	shl	rax, 0x4
1408d4e43: 48 8b 44 01 10              	mov	rax, qword ptr [rcx + rax + 0x10]
1408d4e48: 48 89 85 e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], rax
1408d4e4f: 48 8b 97 20 01 00 00        	mov	rdx, qword ptr [rdi + 0x120]
1408d4e56: 48 8b 86 88 00 00 00        	mov	rax, qword ptr [rsi + 0x88]
1408d4e5d: 48 39 d0                    	cmp	rax, rdx
1408d4e60: 76 6e                       	jbe	0x1408d4ed0 <.text+0x8d3ed0>
1408d4e62: eb 2a                       	jmp	0x1408d4e8e <.text+0x8d3e8e>
1408d4e64: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1408d4e70: 48 c7 85 e8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2e8], 0x0
1408d4e7b: 48 8b 97 20 01 00 00        	mov	rdx, qword ptr [rdi + 0x120]
1408d4e82: 48 8b 86 88 00 00 00        	mov	rax, qword ptr [rsi + 0x88]
1408d4e89: 48 39 d0                    	cmp	rax, rdx
1408d4e8c: 76 42                       	jbe	0x1408d4ed0 <.text+0x8d3ed0>
1408d4e8e: 48 8b 8e 80 00 00 00        	mov	rcx, qword ptr [rsi + 0x80]
1408d4e95: 48 83 3c d1 00              	cmp	qword ptr [rcx + 8*rdx], 0x0
1408d4e9a: 74 34                       	je	0x1408d4ed0 <.text+0x8d3ed0>
1408d4e9c: 4c 89 f1                    	mov	rcx, r14
1408d4e9f: e8 cc b8 42 00              	call	0x140d00770 <.text+0xcff770>
1408d4ea4: 48 89 85 28 03 00 00        	mov	qword ptr [rbp + 0x328], rax
1408d4eab: 48 8b 86 88 00 00 00        	mov	rax, qword ptr [rsi + 0x88]
1408d4eb2: b1 01                       	mov	cl, 0x1
1408d4eb4: 89 8d 58 03 00 00           	mov	dword ptr [rbp + 0x358], ecx
1408d4eba: 48 8b 97 28 01 00 00        	mov	rdx, qword ptr [rdi + 0x128]
1408d4ec1: 48 39 d0                    	cmp	rax, rdx
1408d4ec4: 76 4a                       	jbe	0x1408d4f10 <.text+0x8d3f10>
1408d4ec6: eb 1e                       	jmp	0x1408d4ee6 <.text+0x8d3ee6>
1408d4ec8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1408d4ed0: c7 85 58 03 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x358], 0x0
1408d4eda: 48 8b 97 28 01 00 00        	mov	rdx, qword ptr [rdi + 0x128]
1408d4ee1: 48 39 d0                    	cmp	rax, rdx
1408d4ee4: 76 2a                       	jbe	0x1408d4f10 <.text+0x8d3f10>
1408d4ee6: 48 8b 86 80 00 00 00        	mov	rax, qword ptr [rsi + 0x80]
1408d4eed: 48 83 3c d0 00              	cmp	qword ptr [rax + 8*rdx], 0x0
1408d4ef2: 74 1c                       	je	0x1408d4f10 <.text+0x8d3f10>
1408d4ef4: 4c 89 f1                    	mov	rcx, r14
1408d4ef7: e8 74 b8 42 00              	call	0x140d00770 <.text+0xcff770>
1408d4efc: 48 89 85 08 03 00 00        	mov	qword ptr [rbp + 0x308], rax
1408d4f03: b0 01                       	mov	al, 0x1
1408d4f05: eb 0b                       	jmp	0x1408d4f12 <.text+0x8d3f12>
1408d4f07: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1408d4f10: 31 c0                       	xor	eax, eax
1408d4f12: 4c 8b 46 50                 	mov	r8, qword ptr [rsi + 0x50]
1408d4f16: 4c 8b 33                    	mov	r14, qword ptr [rbx]
1408d4f19: 31 db                       	xor	ebx, ebx
1408d4f1b: 89 c6                       	mov	esi, eax
1408d4f1d: 48 89 d8                    	mov	rax, rbx
1408d4f20: 48 c1 e0 04                 	shl	rax, 0x4
1408d4f24: 41 8b 4c 00 08              	mov	ecx, dword ptr [r8 + rax + 0x8]
1408d4f29: f6 85 58 03 00 00 01        	test	byte ptr [rbp + 0x358], 0x1
1408d4f30: 74 1f                       	je	0x1408d4f51 <.text+0x8d3f51>
1408d4f32: 8b 95 60 03 00 00           	mov	edx, dword ptr [rbp + 0x360]
1408d4f38: 4d 89 c1                    	mov	r9, r8
1408d4f3b: 4c 8b 85 28 03 00 00        	mov	r8, qword ptr [rbp + 0x328]
1408d4f42: 41 2b 14 88                 	sub	edx, dword ptr [r8 + 4*rcx]
1408d4f46: 4d 89 c8                    	mov	r8, r9
1408d4f49: 3b 95 20 03 00 00           	cmp	edx, dword ptr [rbp + 0x320]
1408d4f4f: 72 2f                       	jb	0x1408d4f80 <.text+0x8d3f80>
1408d4f51: 40 f6 c6 01                 	test	sil, 0x1
1408d4f55: 0f 84 d5 fd ff ff           	je	0x1408d4d30 <.text+0x8d3d30>
1408d4f5b: 89 c9                       	mov	ecx, ecx
1408d4f5d: 8b 95 60 03 00 00           	mov	edx, dword ptr [rbp + 0x360]
1408d4f63: 4d 89 c1                    	mov	r9, r8
1408d4f66: 4c 8b 85 08 03 00 00        	mov	r8, qword ptr [rbp + 0x308]
1408d4f6d: 41 2b 14 88                 	sub	edx, dword ptr [r8 + 4*rcx]
1408d4f71: 4d 89 c8                    	mov	r8, r9
1408d4f74: 3b 95 20 03 00 00           	cmp	edx, dword ptr [rbp + 0x320]
1408d4f7a: 0f 83 b0 fd ff ff           	jae	0x1408d4d30 <.text+0x8d3d30>
1408d4f80: 89 b5 f0 02 00 00           	mov	dword ptr [rbp + 0x2f0], esi
1408d4f86: 4c 89 85 b0 00 00 00        	mov	qword ptr [rbp + 0xb0], r8
1408d4f8d: 4c 01 c0                    	add	rax, r8
1408d4f90: 48 8d 14 c9                 	lea	rdx, [rcx + 8*rcx]
1408d4f94: 4c 8b 85 e0 02 00 00        	mov	r8, qword ptr [rbp + 0x2e0]
1408d4f9b: 49 8d 14 d0                 	lea	rdx, [r8 + 8*rdx]
1408d4f9f: 48 89 95 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rdx
1408d4fa6: 48 6b c9 58                 	imul	rcx, rcx, 0x58
1408d4faa: 48 03 8d e8 02 00 00        	add	rcx, qword ptr [rbp + 0x2e8]
1408d4fb1: 4c 8b 10                    	mov	r10, qword ptr [rax]
1408d4fb4: 48 ff c3                    	inc	rbx
1408d4fb7: 48 8b 31                    	mov	rsi, qword ptr [rcx]
1408d4fba: 48 89 b5 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rsi
1408d4fc1: 48 8b 85 60 02 00 00        	mov	rax, qword ptr [rbp + 0x260]
1408d4fc8: 48 83 78 18 00              	cmp	qword ptr [rax + 0x18], 0x0
1408d4fcd: 4c 89 95 a8 02 00 00        	mov	qword ptr [rbp + 0x2a8], r10
1408d4fd4: 0f 84 f0 00 00 00           	je	0x1408d50ca <.text+0x8d40ca>
1408d4fda: 48 89 8d 68 02 00 00        	mov	qword ptr [rbp + 0x268], rcx
1408d4fe1: 48 8b 8d a8 00 00 00        	mov	rcx, qword ptr [rbp + 0xa8]
1408d4fe8: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d4fef: e8 7c bd cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d4ff4: 48 8b 95 60 02 00 00        	mov	rdx, qword ptr [rbp + 0x260]
1408d4ffb: 48 8b 0a                    	mov	rcx, qword ptr [rdx]
1408d4ffe: 48 8b 52 08                 	mov	rdx, qword ptr [rdx + 0x8]
1408d5002: 49 89 d0                    	mov	r8, rdx
1408d5005: 49 21 c0                    	and	r8, rax
1408d5008: 48 c1 e8 39                 	shr	rax, 0x39
1408d500c: 66 0f 6e c0                 	movd	xmm0, eax
1408d5010: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d5014: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d5019: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d501e: 48 8d 81 58 fd ff ff        	lea	rax, [rcx - 0x2a8]
1408d5025: 45 31 c9                    	xor	r9d, r9d
1408d5028: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d502e: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d5032: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d5036: 66 44 0f d7 d2              	pmovmskb	r10d, xmm2
1408d503b: 45 85 d2                    	test	r10d, r10d
1408d503e: 74 28                       	je	0x1408d5068 <.text+0x8d4068>
1408d5040: f3 45 0f bc da              	tzcnt	r11d, r10d
1408d5045: 4d 01 c3                    	add	r11, r8
1408d5048: 49 21 d3                    	and	r11, rdx
1408d504b: 49 f7 db                    	neg	r11
1408d504e: 4d 69 db a8 02 00 00        	imul	r11, r11, 0x2a8
1408d5055: 4a 39 34 18                 	cmp	qword ptr [rax + r11], rsi
1408d5059: 74 32                       	je	0x1408d508d <.text+0x8d408d>
1408d505b: 45 8d 5a ff                 	lea	r11d, [r10 - 0x1]
1408d505f: 66 45 21 d3                 	and	r11w, r10w
1408d5063: 45 89 da                    	mov	r10d, r11d
1408d5066: 75 d8                       	jne	0x1408d5040 <.text+0x8d4040>
1408d5068: 66 0f 74 ce                 	pcmpeqb	xmm1, xmm6
1408d506c: 66 44 0f d7 d1              	pmovmskb	r10d, xmm1
1408d5071: 45 85 d2                    	test	r10d, r10d
1408d5074: 4c 8b 95 a8 02 00 00        	mov	r10, qword ptr [rbp + 0x2a8]
1408d507b: 75 4d                       	jne	0x1408d50ca <.text+0x8d40ca>
1408d507d: 4d 01 c8                    	add	r8, r9
1408d5080: 49 83 c0 10                 	add	r8, 0x10
1408d5084: 49 83 c1 10                 	add	r9, 0x10
1408d5088: 49 21 d0                    	and	r8, rdx
1408d508b: eb 9b                       	jmp	0x1408d5028 <.text+0x8d4028>
1408d508d: 4a 83 bc 19 70 fd ff ff 02  	cmp	qword ptr [rcx + r11 - 0x290], 0x2
1408d5096: 0f 82 24 0c 00 00           	jb	0x1408d5cc0 <.text+0x8d4cc0>
1408d509c: f3 42 0f 10 84 19 90 fd ff ff       	movss	xmm0, dword ptr [rcx + r11 - 0x270]
1408d50a6: 0f 2e c7                    	ucomiss	xmm0, xmm7
1408d50a9: 0f 86 11 0c 00 00           	jbe	0x1408d5cc0 <.text+0x8d4cc0>
1408d50af: 4c 01 d9                    	add	rcx, r11
1408d50b2: 80 79 f7 00                 	cmp	byte ptr [rcx - 0x9], 0x0
1408d50b6: 4c 8b 95 a8 02 00 00        	mov	r10, qword ptr [rbp + 0x2a8]
1408d50bd: 48 8b b5 f8 02 00 00        	mov	rsi, qword ptr [rbp + 0x2f8]
1408d50c4: 0f 84 48 01 00 00           	je	0x1408d5212 <.text+0x8d4212>
1408d50ca: 4c 89 95 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], r10
1408d50d1: 48 8b 8d 10 03 00 00        	mov	rcx, qword ptr [rbp + 0x310]
1408d50d8: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d50df: e8 8c bc cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d50e4: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d50eb: 48 8b 8a f8 3d 00 00        	mov	rcx, qword ptr [rdx + 0x3df8]
1408d50f2: 48 8b 92 00 3e 00 00        	mov	rdx, qword ptr [rdx + 0x3e00]
1408d50f9: 49 89 d0                    	mov	r8, rdx
1408d50fc: 49 21 c0                    	and	r8, rax
1408d50ff: 48 c1 e8 39                 	shr	rax, 0x39
1408d5103: 66 0f 6e c0                 	movd	xmm0, eax
1408d5107: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d510b: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d5110: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d5115: 4c 8d 51 e0                 	lea	r10, [rcx - 0x20]
1408d5119: 45 31 c9                    	xor	r9d, r9d
1408d511c: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d5122: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d5126: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d512a: 66 0f d7 f2                 	pmovmskb	esi, xmm2
1408d512e: 85 f6                       	test	esi, esi
1408d5130: 74 52                       	je	0x1408d5184 <.text+0x8d4184>
1408d5132: 48 89 8d f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rcx
1408d5139: 4c 89 8d 68 02 00 00        	mov	qword ptr [rbp + 0x268], r9
1408d5140: f3 0f bc c6                 	tzcnt	eax, esi
1408d5144: 4c 01 c0                    	add	rax, r8
1408d5147: 48 21 d0                    	and	rax, rdx
1408d514a: 49 89 c1                    	mov	r9, rax
1408d514d: 49 c1 e1 05                 	shl	r9, 0x5
1408d5151: 48 89 d1                    	mov	rcx, rdx
1408d5154: 4c 89 d2                    	mov	rdx, r10
1408d5157: 4d 29 ca                    	sub	r10, r9
1408d515a: 4c 8b 9d a8 02 00 00        	mov	r11, qword ptr [rbp + 0x2a8]
1408d5161: 4d 39 1a                    	cmp	qword ptr [r10], r11
1408d5164: 74 43                       	je	0x1408d51a9 <.text+0x8d41a9>
1408d5166: 8d 46 ff                    	lea	eax, [rsi - 0x1]
1408d5169: 66 21 f0                    	and	ax, si
1408d516c: 89 c6                       	mov	esi, eax
1408d516e: 49 89 d2                    	mov	r10, rdx
1408d5171: 48 89 ca                    	mov	rdx, rcx
1408d5174: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
1408d517b: 4c 8b 8d 68 02 00 00        	mov	r9, qword ptr [rbp + 0x268]
1408d5182: 75 bc                       	jne	0x1408d5140 <.text+0x8d4140>
1408d5184: 66 0f 74 ce                 	pcmpeqb	xmm1, xmm6
1408d5188: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d518c: 85 c0                       	test	eax, eax
1408d518e: 8b b5 f0 02 00 00           	mov	esi, dword ptr [rbp + 0x2f0]
1408d5194: 75 47                       	jne	0x1408d51dd <.text+0x8d41dd>
1408d5196: 4d 01 c8                    	add	r8, r9
1408d5199: 49 83 c0 10                 	add	r8, 0x10
1408d519d: 49 83 c1 10                 	add	r9, 0x10
1408d51a1: 49 21 d0                    	and	r8, rdx
1408d51a4: e9 73 ff ff ff              	jmp	0x1408d511c <.text+0x8d411c>
1408d51a9: 49 c1 f9 05                 	sar	r9, 0x5
1408d51ad: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d51b1: 49 21 c8                    	and	r8, rcx
1408d51b4: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
1408d51bb: f3 42 0f 6f 04 01           	movdqu	xmm0, xmmword ptr [rcx + r8]
1408d51c1: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d51c5: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d51c9: 85 d2                       	test	edx, edx
1408d51cb: 0f 84 2e 01 00 00           	je	0x1408d52ff <.text+0x8d42ff>
1408d51d1: 66 0f bd d2                 	bsr	dx, dx
1408d51d5: 83 f2 0f                    	xor	edx, 0xf
1408d51d8: e9 26 01 00 00              	jmp	0x1408d5303 <.text+0x8d4303>
1408d51dd: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d51e4: 48 83 b8 08 3e 00 00 00     	cmp	qword ptr [rax + 0x3e08], 0x0
1408d51ec: 0f 85 bb 01 00 00           	jne	0x1408d53ad <.text+0x8d43ad>
1408d51f2: ba 01 00 00 00              	mov	edx, 0x1
1408d51f7: 48 8b 8d 90 02 00 00        	mov	rcx, qword ptr [rbp + 0x290]
1408d51fe: 4c 8b 85 10 03 00 00        	mov	r8, qword ptr [rbp + 0x310]
1408d5205: 41 b1 01                    	mov	r9b, 0x1
1408d5208: e8 93 4a fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d520d: e9 9b 01 00 00              	jmp	0x1408d53ad <.text+0x8d43ad>
1408d5212: 48 8b 46 10                 	mov	rax, qword ptr [rsi + 0x10]
1408d5216: 48 85 c0                    	test	rax, rax
1408d5219: 0f 84 db 0a 00 00           	je	0x1408d5cfa <.text+0x8d4cfa>
1408d521f: 48 8b 8d 68 02 00 00        	mov	rcx, qword ptr [rbp + 0x268]
1408d5226: 66 0f 6e 41 4c              	movd	xmm0, dword ptr [rcx + 0x4c]
1408d522b: 48 8b 4e 08                 	mov	rcx, qword ptr [rsi + 0x8]
1408d522f: 48 8d 04 c0                 	lea	rax, [rax + 8*rax]
1408d5233: 48 8d 04 81                 	lea	rax, [rcx + 4*rax]
1408d5237: 48 89 8d b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rcx
1408d523e: 48 89 85 b8 01 00 00        	mov	qword ptr [rbp + 0x1b8], rax
1408d5245: 66 0f 7e 85 c0 01 00 00     	movd	dword ptr [rbp + 0x1c0], xmm0
1408d524d: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1408d5254: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d525b: 4c 8d 05 2e e3 1c 02        	lea	r8, [rip + 0x21ce32e]   # 0x142aa3590
1408d5262: e8 69 8f 4f 00              	call	0x140dce1d0 <.text+0xdcd1d0>
1408d5267: 4c 8b 46 28                 	mov	r8, qword ptr [rsi + 0x28]
1408d526b: 4c 89 c0                    	mov	rax, r8
1408d526e: b9 24 00 00 00              	mov	ecx, 0x24
1408d5273: 48 f7 e1                    	mul	rcx
1408d5276: 49 89 c1                    	mov	r9, rax
1408d5279: 0f 80 f1 0a 00 00           	jo	0x1408d5d70 <.text+0x8d4d70>
1408d527f: b9 00 00 00 00              	mov	ecx, 0x0
1408d5284: 48 b8 fc ff ff ff ff ff ff 7f       	movabs	rax, 0x7ffffffffffffffc
1408d528e: 49 39 c1                    	cmp	r9, rax
1408d5291: 0f 87 db 0a 00 00           	ja	0x1408d5d72 <.text+0x8d4d72>
1408d5297: 48 8b 56 20                 	mov	rdx, qword ptr [rsi + 0x20]
1408d529b: 4d 85 c9                    	test	r9, r9
1408d529e: 48 89 b5 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rsi
1408d52a5: 4c 89 85 98 01 00 00        	mov	qword ptr [rbp + 0x198], r8
1408d52ac: 0f 84 08 01 00 00           	je	0x1408d53ba <.text+0x8d43ba>
1408d52b2: 48 89 95 a0 02 00 00        	mov	qword ptr [rbp + 0x2a0], rdx
1408d52b9: 0f b6 05 49 f9 d5 02        	movzx	eax, byte ptr [rip + 0x2d5f949] # 0x143634c09
1408d52c0: ba 04 00 00 00              	mov	edx, 0x4
1408d52c5: 4c 89 c9                    	mov	rcx, r9
1408d52c8: 4c 89 8d 98 02 00 00        	mov	qword ptr [rbp + 0x298], r9
1408d52cf: e8 3c e9 d3 ff              	call	0x140613c10 <.text+0x612c10>
1408d52d4: b9 04 00 00 00              	mov	ecx, 0x4
1408d52d9: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
1408d52e0: 4c 8b 8d 98 02 00 00        	mov	r9, qword ptr [rbp + 0x298]
1408d52e7: 48 85 c0                    	test	rax, rax
1408d52ea: 0f 84 82 0a 00 00           	je	0x1408d5d72 <.text+0x8d4d72>
1408d52f0: 48 89 d1                    	mov	rcx, rdx
1408d52f3: 48 8b 95 a0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2a0]
1408d52fa: e9 c2 00 00 00              	jmp	0x1408d53c1 <.text+0x8d43c1>
1408d52ff: 66 ba 10 00                 	mov	dx, 0x10
1408d5303: 8b b5 f0 02 00 00           	mov	esi, dword ptr [rbp + 0x2f0]
1408d5309: f3 42 0f 6f 04 09           	movdqu	xmm0, xmmword ptr [rcx + r9]
1408d530f: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d5313: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d5318: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d531f: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d5324: 44 01 d2                    	add	edx, r10d
1408d5327: 41 b2 80                    	mov	r10b, -0x80
1408d532a: 66 83 fa 0f                 	cmp	dx, 0xf
1408d532e: 77 11                       	ja	0x1408d5341 <.text+0x8d4341>
1408d5330: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d5337: 48 ff 82 08 3e 00 00        	inc	qword ptr [rdx + 0x3e08]
1408d533e: 41 b2 ff                    	mov	r10b, -0x1
1408d5341: 48 f7 d8                    	neg	rax
1408d5344: 48 c1 e0 05                 	shl	rax, 0x5
1408d5348: 48 01 c8                    	add	rax, rcx
1408d534b: 46 88 14 09                 	mov	byte ptr [rcx + r9], r10b
1408d534f: 46 88 54 01 10              	mov	byte ptr [rcx + r8 + 0x10], r10b
1408d5354: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d535b: 48 ff 8a 10 3e 00 00        	dec	qword ptr [rdx + 0x3e10]
1408d5362: 8b 48 f8                    	mov	ecx, dword ptr [rax - 0x8]
1408d5365: 89 8d 00 01 00 00           	mov	dword ptr [rbp + 0x100], ecx
1408d536b: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d5370: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d5378: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d537f: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d5386: e8 05 42 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d538b: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d5392: 74 19                       	je	0x1408d53ad <.text+0x8d43ad>
1408d5394: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d539b: f0                          	lock
1408d539c: 48 ff 08                    	dec	qword ptr [rax]
1408d539f: 75 0c                       	jne	0x1408d53ad <.text+0x8d43ad>
1408d53a1: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d53a8: e8 73 72 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d53ad: 44 0f b6 85 90 01 00 00     	movzx	r8d, byte ptr [rbp + 0x190]
1408d53b5: e9 21 f7 ff ff              	jmp	0x1408d4adb <.text+0x8d3adb>
1408d53ba: b8 04 00 00 00              	mov	eax, 0x4
1408d53bf: 31 c9                       	xor	ecx, ecx
1408d53c1: 48 89 8d 98 02 00 00        	mov	qword ptr [rbp + 0x298], rcx
1408d53c8: 48 89 c1                    	mov	rcx, rax
1408d53cb: 4d 89 c8                    	mov	r8, r9
1408d53ce: 48 89 c6                    	mov	rsi, rax
1408d53d1: e8 2a e8 04 02              	call	0x142923c00 <.text+0x2922c00>
1408d53d6: 48 8b 85 98 02 00 00        	mov	rax, qword ptr [rbp + 0x298]
1408d53dd: 48 89 85 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rax
1408d53e4: 48 89 b5 88 00 00 00        	mov	qword ptr [rbp + 0x88], rsi
1408d53eb: 48 89 b5 b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rsi
1408d53f2: 48 8b 85 98 01 00 00        	mov	rax, qword ptr [rbp + 0x198]
1408d53f9: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d5400: 4c 8b 85 f8 02 00 00        	mov	r8, qword ptr [rbp + 0x2f8]
1408d5407: 4d 8b 50 40                 	mov	r10, qword ptr [r8 + 0x40]
1408d540b: 4c 89 d0                    	mov	rax, r10
1408d540e: b9 18 00 00 00              	mov	ecx, 0x18
1408d5413: 48 f7 e1                    	mul	rcx
1408d5416: 49 89 c1                    	mov	r9, rax
1408d5419: 0f 80 34 09 00 00           	jo	0x1408d5d53 <.text+0x8d4d53>
1408d541f: b9 00 00 00 00              	mov	ecx, 0x0
1408d5424: 48 b8 fc ff ff ff ff ff ff 7f       	movabs	rax, 0x7ffffffffffffffc
1408d542e: 49 39 c1                    	cmp	r9, rax
1408d5431: 0f 87 1e 09 00 00           	ja	0x1408d5d55 <.text+0x8d4d55>
1408d5437: 49 8b 70 38                 	mov	rsi, qword ptr [r8 + 0x38]
1408d543b: 4d 85 c9                    	test	r9, r9
1408d543e: 4c 89 95 98 01 00 00        	mov	qword ptr [rbp + 0x198], r10
1408d5445: 74 40                       	je	0x1408d5487 <.text+0x8d4487>
1408d5447: 0f b6 05 bb f7 d5 02        	movzx	eax, byte ptr [rip + 0x2d5f7bb] # 0x143634c09
1408d544e: ba 04 00 00 00              	mov	edx, 0x4
1408d5453: 4c 89 c9                    	mov	rcx, r9
1408d5456: 4c 89 8d a0 02 00 00        	mov	qword ptr [rbp + 0x2a0], r9
1408d545d: e8 ae e7 d3 ff              	call	0x140613c10 <.text+0x612c10>
1408d5462: b9 04 00 00 00              	mov	ecx, 0x4
1408d5467: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
1408d546e: 4c 8b 8d a0 02 00 00        	mov	r9, qword ptr [rbp + 0x2a0]
1408d5475: 48 85 c0                    	test	rax, rax
1408d5478: 0f 84 d7 08 00 00           	je	0x1408d5d55 <.text+0x8d4d55>
1408d547e: 48 89 95 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rdx
1408d5485: eb 10                       	jmp	0x1408d5497 <.text+0x8d4497>
1408d5487: b8 04 00 00 00              	mov	eax, 0x4
1408d548c: 48 c7 85 f8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2f8], 0x0
1408d5497: 48 89 c1                    	mov	rcx, rax
1408d549a: 48 89 f2                    	mov	rdx, rsi
1408d549d: 4d 89 c8                    	mov	r8, r9
1408d54a0: 48 89 c6                    	mov	rsi, rax
1408d54a3: e8 58 e7 04 02              	call	0x142923c00 <.text+0x2922c00>
1408d54a8: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
1408d54af: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
1408d54b6: 48 89 b5 b8 01 00 00        	mov	qword ptr [rbp + 0x1b8], rsi
1408d54bd: 48 8b 85 98 01 00 00        	mov	rax, qword ptr [rbp + 0x198]
1408d54c4: 48 89 85 c0 01 00 00        	mov	qword ptr [rbp + 0x1c0], rax
1408d54cb: c6 85 1f 03 00 00 00        	mov	byte ptr [rbp + 0x31f], 0x0
1408d54d2: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
1408d54d9: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
1408d54e0: 4c 8d 85 b0 02 00 00        	lea	r8, [rbp + 0x2b0]
1408d54e7: 4c 8d 8d b0 01 00 00        	lea	r9, [rbp + 0x1b0]
1408d54ee: e8 dd a2 ff ff              	call	0x1408cf7d0 <.text+0x8ce7d0>
1408d54f3: 48 c7 85 b0 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x1
1408d54fe: 48 c7 85 b8 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1b8], 0x1
1408d5509: 0f 28 85 60 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x160]
1408d5510: 48 8d 85 c0 01 00 00        	lea	rax, [rbp + 0x1c0]
1408d5517: 0f 29 40 70                 	movaps	xmmword ptr [rax + 0x70], xmm0
1408d551b: 0f 28 85 50 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x150]
1408d5522: 0f 29 40 60                 	movaps	xmmword ptr [rax + 0x60], xmm0
1408d5526: 0f 28 85 40 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x140]
1408d552d: 0f 29 40 50                 	movaps	xmmword ptr [rax + 0x50], xmm0
1408d5531: 0f 28 85 30 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x130]
1408d5538: 0f 29 40 40                 	movaps	xmmword ptr [rax + 0x40], xmm0
1408d553c: 66 0f 6f 85 f0 00 00 00     	movdqa	xmm0, xmmword ptr [rbp + 0xf0]
1408d5544: 66 0f 6f 8d 00 01 00 00     	movdqa	xmm1, xmmword ptr [rbp + 0x100]
1408d554c: 66 0f 6f 95 10 01 00 00     	movdqa	xmm2, xmmword ptr [rbp + 0x110]
1408d5554: 0f 28 9d 20 01 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x120]
1408d555b: 0f 29 58 30                 	movaps	xmmword ptr [rax + 0x30], xmm3
1408d555f: 66 0f 7f 50 20              	movdqa	xmmword ptr [rax + 0x20], xmm2
1408d5564: 66 0f 7f 48 10              	movdqa	xmmword ptr [rax + 0x10], xmm1
1408d5569: 66 0f 7f 00                 	movdqa	xmmword ptr [rax], xmm0
1408d556d: 0f b6 05 95 f6 d5 02        	movzx	eax, byte ptr [rip + 0x2d5f695] # 0x143634c09
1408d5574: b9 90 00 00 00              	mov	ecx, 0x90
1408d5579: ba 10 00 00 00              	mov	edx, 0x10
1408d557e: e8 8d e6 d3 ff              	call	0x140613c10 <.text+0x612c10>
1408d5583: 48 85 c0                    	test	rax, rax
1408d5586: 0f 84 9f 07 00 00           	je	0x1408d5d2b <.text+0x8d4d2b>
1408d558c: 41 b8 90 00 00 00           	mov	r8d, 0x90
1408d5592: 48 89 c1                    	mov	rcx, rax
1408d5595: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d559c: 48 89 c6                    	mov	rsi, rax
1408d559f: e8 5c e6 04 02              	call	0x142923c00 <.text+0x2922c00>
1408d55a4: 48 8b 95 68 02 00 00        	mov	rdx, qword ptr [rbp + 0x268]
1408d55ab: f3 0f 10 42 34              	movss	xmm0, dword ptr [rdx + 0x34]
1408d55b0: 41 0f 2e c0                 	ucomiss	xmm0, xmm8
1408d55b4: 0f 9b c0                    	setnp	al
1408d55b7: 0f 94 c1                    	sete	cl
1408d55ba: 20 c1                       	and	cl, al
1408d55bc: 44 0f 2e 7a 30              	ucomiss	xmm15, dword ptr [rdx + 0x30]
1408d55c1: 48 8b 02                    	mov	rax, qword ptr [rdx]
1408d55c4: 0f 92 c2                    	setb	dl
1408d55c7: 48 89 b5 68 02 00 00        	mov	qword ptr [rbp + 0x268], rsi
1408d55ce: 48 89 b5 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rsi
1408d55d5: c6 85 b8 00 00 00 02        	mov	byte ptr [rbp + 0xb8], 0x2
1408d55dc: 0f 93 85 b9 00 00 00        	setae	byte ptr [rbp + 0xb9]
1408d55e3: 20 ca                       	and	dl, cl
1408d55e5: 88 95 ba 00 00 00           	mov	byte ptr [rbp + 0xba], dl
1408d55eb: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
1408d55f2: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
1408d55f9: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
1408d5600: c6 85 56 03 00 00 01        	mov	byte ptr [rbp + 0x356], 0x1
1408d5607: 48 8b 8d 10 03 00 00        	mov	rcx, qword ptr [rbp + 0x310]
1408d560e: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1408d5615: e8 56 b7 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d561a: 48 89 85 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rax
1408d5621: 8b b5 f0 02 00 00           	mov	esi, dword ptr [rbp + 0x2f0]
1408d5627: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d562e: 48 8b 8a f8 3d 00 00        	mov	rcx, qword ptr [rdx + 0x3df8]
1408d5635: 48 8b 92 00 3e 00 00        	mov	rdx, qword ptr [rdx + 0x3e00]
1408d563c: 49 89 d0                    	mov	r8, rdx
1408d563f: 4c 8b 8d f8 02 00 00        	mov	r9, qword ptr [rbp + 0x2f8]
1408d5646: 4d 21 c8                    	and	r8, r9
1408d5649: 49 c1 e9 39                 	shr	r9, 0x39
1408d564d: 66 41 0f 6e c1              	movd	xmm0, r9d
1408d5652: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d5656: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d565b: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d5660: 4c 8d 49 e0                 	lea	r9, [rcx - 0x20]
1408d5664: 45 31 db                    	xor	r11d, r11d
1408d5667: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d566d: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d5671: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d5675: 66 0f d7 c2                 	pmovmskb	eax, xmm2
1408d5679: 85 c0                       	test	eax, eax
1408d567b: 74 4d                       	je	0x1408d56ca <.text+0x8d46ca>
1408d567d: 4c 89 9d 98 01 00 00        	mov	qword ptr [rbp + 0x198], r11
1408d5684: f3 0f bc f0                 	tzcnt	esi, eax
1408d5688: 4c 01 c6                    	add	rsi, r8
1408d568b: 48 21 d6                    	and	rsi, rdx
1408d568e: 49 89 f2                    	mov	r10, rsi
1408d5691: 49 c1 e2 05                 	shl	r10, 0x5
1408d5695: 49 89 d3                    	mov	r11, rdx
1408d5698: 4c 89 ca                    	mov	rdx, r9
1408d569b: 4d 29 d1                    	sub	r9, r10
1408d569e: 4c 8b 95 a8 02 00 00        	mov	r10, qword ptr [rbp + 0x2a8]
1408d56a5: 4d 39 11                    	cmp	qword ptr [r9], r10
1408d56a8: 74 47                       	je	0x1408d56f1 <.text+0x8d46f1>
1408d56aa: 44 8d 48 ff                 	lea	r9d, [rax - 0x1]
1408d56ae: 66 41 21 c1                 	and	r9w, ax
1408d56b2: 44 89 c8                    	mov	eax, r9d
1408d56b5: 8b b5 f0 02 00 00           	mov	esi, dword ptr [rbp + 0x2f0]
1408d56bb: 49 89 d1                    	mov	r9, rdx
1408d56be: 4c 89 da                    	mov	rdx, r11
1408d56c1: 4c 8b 9d 98 01 00 00        	mov	r11, qword ptr [rbp + 0x198]
1408d56c8: 75 ba                       	jne	0x1408d5684 <.text+0x8d4684>
1408d56ca: 4d 89 ca                    	mov	r10, r9
1408d56cd: 66 0f 74 ce                 	pcmpeqb	xmm1, xmm6
1408d56d1: 66 44 0f d7 c9              	pmovmskb	r9d, xmm1
1408d56d6: 45 85 c9                    	test	r9d, r9d
1408d56d9: 75 48                       	jne	0x1408d5723 <.text+0x8d4723>
1408d56db: 4d 01 d8                    	add	r8, r11
1408d56de: 49 83 c0 10                 	add	r8, 0x10
1408d56e2: 49 83 c3 10                 	add	r11, 0x10
1408d56e6: 49 21 d0                    	and	r8, rdx
1408d56e9: 4d 89 d1                    	mov	r9, r10
1408d56ec: e9 76 ff ff ff              	jmp	0x1408d5667 <.text+0x8d4667>
1408d56f1: 48 f7 de                    	neg	rsi
1408d56f4: 48 c1 e6 05                 	shl	rsi, 0x5
1408d56f8: 48 01 f1                    	add	rcx, rsi
1408d56fb: 48 89 8d b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rcx
1408d5702: 48 8b 85 90 02 00 00        	mov	rax, qword ptr [rbp + 0x290]
1408d5709: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d5710: 48 c7 85 b0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2b0], 0x0
1408d571b: 8b b5 f0 02 00 00           	mov	esi, dword ptr [rbp + 0x2f0]
1408d5721: eb 3f                       	jmp	0x1408d5762 <.text+0x8d4762>
1408d5723: 48 8b 8d 70 03 00 00        	mov	rcx, qword ptr [rbp + 0x370]
1408d572a: 48 83 b9 08 3e 00 00 00     	cmp	qword ptr [rcx + 0x3e08], 0x0
1408d5732: 48 8b 95 a8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2a8]
1408d5739: 0f 84 9f 00 00 00           	je	0x1408d57de <.text+0x8d47de>
1408d573f: 48 8b 8d 90 02 00 00        	mov	rcx, qword ptr [rbp + 0x290]
1408d5746: 48 89 8d b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rcx
1408d574d: 48 89 95 b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rdx
1408d5754: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
1408d575b: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
1408d5762: 0f 10 85 b8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xb8]
1408d5769: f3 0f 6f 8d c8 00 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0xc8]
1408d5771: 66 0f 7f 8d 00 01 00 00     	movdqa	xmmword ptr [rbp + 0x100], xmm1
1408d5779: 0f 29 85 f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm0
1408d5780: c7 85 b8 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x1b8], 0x0
1408d578a: 48 c7 85 b0 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1b0], 0x0
1408d5795: 44 0f 29 b5 c0 01 00 00     	movaps	xmmword ptr [rbp + 0x1c0], xmm14
1408d579d: c6 85 56 03 00 00 00        	mov	byte ptr [rbp + 0x356], 0x0
1408d57a4: 48 8d 85 b0 01 00 00        	lea	rax, [rbp + 0x1b0]
1408d57ab: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408d57b0: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1408d57b7: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1408d57be: 4c 8b 85 70 03 00 00        	mov	r8, qword ptr [rbp + 0x370]
1408d57c5: 4c 8d 8d f0 00 00 00        	lea	r9, [rbp + 0xf0]
1408d57cc: e8 7f b3 26 00              	call	0x140b40b50 <.text+0xb3fb50>
1408d57d1: 44 0f b6 85 90 01 00 00     	movzx	r8d, byte ptr [rbp + 0x190]
1408d57d9: e9 fd f2 ff ff              	jmp	0x1408d4adb <.text+0x8d3adb>
1408d57de: c6 85 56 03 00 00 01        	mov	byte ptr [rbp + 0x356], 0x1
1408d57e5: ba 01 00 00 00              	mov	edx, 0x1
1408d57ea: 48 8b 8d 90 02 00 00        	mov	rcx, qword ptr [rbp + 0x290]
1408d57f1: 4c 8b 85 10 03 00 00        	mov	r8, qword ptr [rbp + 0x310]
1408d57f8: 41 b1 01                    	mov	r9b, 0x1
1408d57fb: e8 a0 44 fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d5800: 8b b5 f0 02 00 00           	mov	esi, dword ptr [rbp + 0x2f0]
1408d5806: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
1408d580d: e9 2d ff ff ff              	jmp	0x1408d573f <.text+0x8d473f>
1408d5812: 4c 8b 85 98 04 00 00        	mov	r8, qword ptr [rbp + 0x498]
1408d5819: 49 8b 08                    	mov	rcx, qword ptr [r8]
1408d581c: 49 8b 40 10                 	mov	rax, qword ptr [r8 + 0x10]
1408d5820: 48 8b 09                    	mov	rcx, qword ptr [rcx]
1408d5823: 48 39 48 40                 	cmp	qword ptr [rax + 0x40], rcx
1408d5827: 0f 86 a6 00 00 00           	jbe	0x1408d58d3 <.text+0x8d48d3>
1408d582d: 48 8b 50 38                 	mov	rdx, qword ptr [rax + 0x38]
1408d5831: 48 8b 0c ca                 	mov	rcx, qword ptr [rdx + 8*rcx]
1408d5835: 48 85 c9                    	test	rcx, rcx
1408d5838: 0f 84 95 00 00 00           	je	0x1408d58d3 <.text+0x8d48d3>
1408d583e: 48 8b 40 08                 	mov	rax, qword ptr [rax + 0x8]
1408d5842: 48 f7 d1                    	not	rcx
1408d5845: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
1408d5849: 4d 8b 60 08                 	mov	r12, qword ptr [r8 + 0x8]
1408d584d: 4d 8b 04 24                 	mov	r8, qword ptr [r12]
1408d5851: 31 d2                       	xor	edx, edx
1408d5853: 4d 89 c1                    	mov	r9, r8
1408d5856: 4c 2b 4c c8 18              	sub	r9, qword ptr [rax + 8*rcx + 0x18]
1408d585b: 4c 0f 42 ca                 	cmovb	r9, rdx
1408d585f: 4c 2b 44 c8 38              	sub	r8, qword ptr [rax + 8*rcx + 0x38]
1408d5864: 4c 0f 42 c2                 	cmovb	r8, rdx
1408d5868: 4c 89 cf                    	mov	rdi, r9
1408d586b: 48 c1 e7 04                 	shl	rdi, 0x4
1408d586f: 48 03 7c c8 08              	add	rdi, qword ptr [rax + 8*rcx + 0x8]
1408d5874: 4c 8b 54 c8 10              	mov	r10, qword ptr [rax + 8*rcx + 0x10]
1408d5879: 4c 8b 5c c8 30              	mov	r11, qword ptr [rax + 8*rcx + 0x30]
1408d587e: 4d 29 ca                    	sub	r10, r9
1408d5881: 4c 0f 42 d2                 	cmovb	r10, rdx
1408d5885: 41 b9 08 00 00 00           	mov	r9d, 0x8
1408d588b: 49 0f 42 f9                 	cmovb	rdi, r9
1408d588f: 4d 89 c6                    	mov	r14, r8
1408d5892: 49 c1 e6 04                 	shl	r14, 0x4
1408d5896: 4c 03 74 c8 28              	add	r14, qword ptr [rax + 8*rcx + 0x28]
1408d589b: 4d 29 c3                    	sub	r11, r8
1408d589e: 4c 0f 42 da                 	cmovb	r11, rdx
1408d58a2: 4d 0f 42 f1                 	cmovb	r14, r9
1408d58a6: 48 8b 44 c8 40              	mov	rax, qword ptr [rax + 8*rcx + 0x40]
1408d58ab: 4c 29 d8                    	sub	rax, r11
1408d58ae: 4c 29 d0                    	sub	rax, r10
1408d58b1: 49 89 04 24                 	mov	qword ptr [r12], rax
1408d58b5: 49 c1 e2 04                 	shl	r10, 0x4
1408d58b9: 49 01 fa                    	add	r10, rdi
1408d58bc: 4c 89 95 60 03 00 00        	mov	qword ptr [rbp + 0x360], r10
1408d58c3: 49 c1 e3 04                 	shl	r11, 0x4
1408d58c7: 4d 01 f3                    	add	r11, r14
1408d58ca: 4c 89 9d 20 03 00 00        	mov	qword ptr [rbp + 0x320], r11
1408d58d1: eb 03                       	jmp	0x1408d58d6 <.text+0x8d48d6>
1408d58d3: 45 31 e4                    	xor	r12d, r12d
1408d58d6: 31 f6                       	xor	esi, esi
1408d58d8: 66 0f 76 f6                 	pcmpeqd	xmm6, xmm6
1408d58dc: 4c 89 b5 28 03 00 00        	mov	qword ptr [rbp + 0x328], r14
1408d58e3: e9 ce 00 00 00              	jmp	0x1408d59b6 <.text+0x8d49b6>
1408d58e8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1408d58f0: f3 42 0f 6f 0c 01           	movdqu	xmm1, xmmword ptr [rcx + r8]
1408d58f6: 66 0f 6f d1                 	movdqa	xmm2, xmm1
1408d58fa: 66 0f 74 d0                 	pcmpeqb	xmm2, xmm0
1408d58fe: 66 0f d7 fa                 	pmovmskb	edi, xmm2
1408d5902: 85 ff                       	test	edi, edi
1408d5904: 74 3a                       	je	0x1408d5940 <.text+0x8d4940>
1408d5906: 4d 89 fd                    	mov	r13, r15
1408d5909: 4d 89 cf                    	mov	r15, r9
1408d590c: f3 0f bc c7                 	tzcnt	eax, edi
1408d5910: 4c 01 c0                    	add	rax, r8
1408d5913: 48 21 d0                    	and	rax, rdx
1408d5916: 49 89 c1                    	mov	r9, rax
1408d5919: 49 c1 e1 05                 	shl	r9, 0x5
1408d591d: 4d 89 f2                    	mov	r10, r14
1408d5920: 4d 29 ca                    	sub	r10, r9
1408d5923: 49 39 1a                    	cmp	qword ptr [r10], rbx
1408d5926: 74 38                       	je	0x1408d5960 <.text+0x8d4960>
1408d5928: 8d 47 ff                    	lea	eax, [rdi - 0x1]
1408d592b: 66 21 f8                    	and	ax, di
1408d592e: 89 c7                       	mov	edi, eax
1408d5930: 4d 89 f9                    	mov	r9, r15
1408d5933: 4d 89 ef                    	mov	r15, r13
1408d5936: 75 ce                       	jne	0x1408d5906 <.text+0x8d4906>
1408d5938: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1408d5940: 66 0f 74 ce                 	pcmpeqb	xmm1, xmm6
1408d5944: 66 0f d7 c1                 	pmovmskb	eax, xmm1
1408d5948: 85 c0                       	test	eax, eax
1408d594a: 75 44                       	jne	0x1408d5990 <.text+0x8d4990>
1408d594c: 4d 01 c8                    	add	r8, r9
1408d594f: 49 83 c0 10                 	add	r8, 0x10
1408d5953: 49 83 c1 10                 	add	r9, 0x10
1408d5957: 49 21 d0                    	and	r8, rdx
1408d595a: eb 94                       	jmp	0x1408d58f0 <.text+0x8d48f0>
1408d595c: 0f 1f 40 00                 	nop	dword ptr [rax]
1408d5960: 49 c1 f9 05                 	sar	r9, 0x5
1408d5964: 4d 8d 41 f0                 	lea	r8, [r9 - 0x10]
1408d5968: 49 21 d0                    	and	r8, rdx
1408d596b: f3 42 0f 6f 04 01           	movdqu	xmm0, xmmword ptr [rcx + r8]
1408d5971: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d5975: 66 0f d7 d0                 	pmovmskb	edx, xmm0
1408d5979: 85 d2                       	test	edx, edx
1408d597b: 74 53                       	je	0x1408d59d0 <.text+0x8d49d0>
1408d597d: 66 0f bd d2                 	bsr	dx, dx
1408d5981: 83 f2 0f                    	xor	edx, 0xf
1408d5984: eb 4e                       	jmp	0x1408d59d4 <.text+0x8d49d4>
1408d5986: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1408d5990: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d5997: 48 83 b8 08 3e 00 00 00     	cmp	qword ptr [rax + 0x3e08], 0x0
1408d599f: 4c 89 df                    	mov	rdi, r11
1408d59a2: 4c 8b b5 28 03 00 00        	mov	r14, qword ptr [rbp + 0x328]
1408d59a9: 48 8b 9d 58 03 00 00        	mov	rbx, qword ptr [rbp + 0x358]
1408d59b0: 0f 84 e3 00 00 00           	je	0x1408d5a99 <.text+0x8d4a99>
1408d59b6: 4c 89 f9                    	mov	rcx, r15
1408d59b9: 48 89 d8                    	mov	rax, rbx
1408d59bc: 48 85 f6                    	test	rsi, rsi
1408d59bf: 0f 85 13 01 00 00           	jne	0x1408d5ad8 <.text+0x8d4ad8>
1408d59c5: e9 f6 00 00 00              	jmp	0x1408d5ac0 <.text+0x8d4ac0>
1408d59ca: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1408d59d0: 66 ba 10 00                 	mov	dx, 0x10
1408d59d4: 4c 89 df                    	mov	rdi, r11
1408d59d7: 4c 8b b5 28 03 00 00        	mov	r14, qword ptr [rbp + 0x328]
1408d59de: 4d 89 ef                    	mov	r15, r13
1408d59e1: f3 42 0f 6f 04 09           	movdqu	xmm0, xmmword ptr [rcx + r9]
1408d59e7: 66 0f 74 c6                 	pcmpeqb	xmm0, xmm6
1408d59eb: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
1408d59f0: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
1408d59f7: f3 45 0f bc d2              	tzcnt	r10d, r10d
1408d59fc: 44 01 d2                    	add	edx, r10d
1408d59ff: 41 b2 80                    	mov	r10b, -0x80
1408d5a02: 66 83 fa 0f                 	cmp	dx, 0xf
1408d5a06: 48 8b 9d 58 03 00 00        	mov	rbx, qword ptr [rbp + 0x358]
1408d5a0d: 77 11                       	ja	0x1408d5a20 <.text+0x8d4a20>
1408d5a0f: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d5a16: 48 ff 82 08 3e 00 00        	inc	qword ptr [rdx + 0x3e08]
1408d5a1d: 41 b2 ff                    	mov	r10b, -0x1
1408d5a20: 48 f7 d8                    	neg	rax
1408d5a23: 48 c1 e0 05                 	shl	rax, 0x5
1408d5a27: 48 01 c8                    	add	rax, rcx
1408d5a2a: 46 88 14 09                 	mov	byte ptr [rcx + r9], r10b
1408d5a2e: 46 88 54 01 10              	mov	byte ptr [rcx + r8 + 0x10], r10b
1408d5a33: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d5a3a: 48 ff 8a 10 3e 00 00        	dec	qword ptr [rdx + 0x3e10]
1408d5a41: 8b 48 f8                    	mov	ecx, dword ptr [rax - 0x8]
1408d5a44: 89 8d 00 01 00 00           	mov	dword ptr [rbp + 0x100], ecx
1408d5a4a: f3 0f 6f 40 e8              	movdqu	xmm0, xmmword ptr [rax - 0x18]
1408d5a4f: 66 0f 7f 85 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm0
1408d5a57: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1408d5a5e: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
1408d5a65: e8 26 3b 00 00              	call	0x1408d9590 <.text+0x8d8590>
1408d5a6a: 80 bd d0 01 00 00 11        	cmp	byte ptr [rbp + 0x1d0], 0x11
1408d5a71: 0f 84 3f ff ff ff           	je	0x1408d59b6 <.text+0x8d49b6>
1408d5a77: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
1408d5a7e: f0                          	lock
1408d5a7f: 48 ff 08                    	dec	qword ptr [rax]
1408d5a82: 0f 85 2e ff ff ff           	jne	0x1408d59b6 <.text+0x8d49b6>
1408d5a88: 48 8d 8d e8 01 00 00        	lea	rcx, [rbp + 0x1e8]
1408d5a8f: e8 8c 6b 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d5a94: e9 1d ff ff ff              	jmp	0x1408d59b6 <.text+0x8d49b6>
1408d5a99: ba 01 00 00 00              	mov	edx, 0x1
1408d5a9e: 48 8b 8d 90 02 00 00        	mov	rcx, qword ptr [rbp + 0x290]
1408d5aa5: 4c 8b 85 10 03 00 00        	mov	r8, qword ptr [rbp + 0x310]
1408d5aac: 41 b1 01                    	mov	r9b, 0x1
1408d5aaf: e8 ec 41 fd 01              	call	0x1428a9ca0 <.text+0x28a8ca0>
1408d5ab4: e9 fd fe ff ff              	jmp	0x1408d59b6 <.text+0x8d49b6>
1408d5ab9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1408d5ac0: 4c 89 e6                    	mov	rsi, r12
1408d5ac3: 41 bc 00 00 00 00           	mov	r12d, 0x0
1408d5ac9: 48 89 f9                    	mov	rcx, rdi
1408d5acc: 4c 89 f0                    	mov	rax, r14
1408d5acf: 48 85 f6                    	test	rsi, rsi
1408d5ad2: 0f 84 ae 00 00 00           	je	0x1408d5b86 <.text+0x8d4b86>
1408d5ad8: 48 85 c9                    	test	rcx, rcx
1408d5adb: 74 23                       	je	0x1408d5b00 <.text+0x8d4b00>
1408d5add: 4c 8d 79 10                 	lea	r15, [rcx + 0x10]
1408d5ae1: 48 3b 8d 60 03 00 00        	cmp	rcx, qword ptr [rbp + 0x360]
1408d5ae8: 74 16                       	je	0x1408d5b00 <.text+0x8d4b00>
1408d5aea: 49 89 c5                    	mov	r13, rax
1408d5aed: 48 89 c8                    	mov	rax, rcx
1408d5af0: eb 23                       	jmp	0x1408d5b15 <.text+0x8d4b15>
1408d5af2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1408d5b00: 48 85 c0                    	test	rax, rax
1408d5b03: 74 bb                       	je	0x1408d5ac0 <.text+0x8d4ac0>
1408d5b05: 48 3b 85 20 03 00 00        	cmp	rax, qword ptr [rbp + 0x320]
1408d5b0c: 74 b2                       	je	0x1408d5ac0 <.text+0x8d4ac0>
1408d5b0e: 4c 8d 68 10                 	lea	r13, [rax + 0x10]
1408d5b12: 45 31 ff                    	xor	r15d, r15d
1408d5b15: 48 ff 06                    	inc	qword ptr [rsi]
1408d5b18: 48 8b 58 08                 	mov	rbx, qword ptr [rax + 0x8]
1408d5b1c: 48 89 d8                    	mov	rax, rbx
1408d5b1f: 48 c1 e8 20                 	shr	rax, 0x20
1408d5b23: 74 9b                       	je	0x1408d5ac0 <.text+0x8d4ac0>
1408d5b25: 48 89 9d f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rbx
1408d5b2c: 48 8b 8d 10 03 00 00        	mov	rcx, qword ptr [rbp + 0x310]
1408d5b33: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
1408d5b3a: e8 31 b2 cd ff              	call	0x1405b0d70 <.text+0x5afd70>
1408d5b3f: 4c 89 ad 58 03 00 00        	mov	qword ptr [rbp + 0x358], r13
1408d5b46: 49 89 fb                    	mov	r11, rdi
1408d5b49: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1408d5b50: 48 8b 8a f8 3d 00 00        	mov	rcx, qword ptr [rdx + 0x3df8]
1408d5b57: 48 8b 92 00 3e 00 00        	mov	rdx, qword ptr [rdx + 0x3e00]
1408d5b5e: 49 89 d0                    	mov	r8, rdx
1408d5b61: 49 21 c0                    	and	r8, rax
1408d5b64: 48 c1 e8 39                 	shr	rax, 0x39
1408d5b68: 66 0f 6e c0                 	movd	xmm0, eax
1408d5b6c: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408d5b70: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408d5b75: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1408d5b7a: 4c 8d 71 e0                 	lea	r14, [rcx - 0x20]
1408d5b7e: 45 31 c9                    	xor	r9d, r9d
1408d5b81: e9 6a fd ff ff              	jmp	0x1408d58f0 <.text+0x8d48f0>
1408d5b86: 48 83 7d 30 00              	cmp	qword ptr [rbp + 0x30], 0x0
1408d5b8b: 74 48                       	je	0x1408d5bd5 <.text+0x8d4bd5>
1408d5b8d: e8 9e 25 34 00              	call	0x140c18130 <.text+0xc17130>
1408d5b92: 48 89 c1                    	mov	rcx, rax
1408d5b95: 48 8b 00                    	mov	rax, qword ptr [rax]
1408d5b98: 48 83 f8 01                 	cmp	rax, 0x1
1408d5b9c: 0f 85 a6 00 00 00           	jne	0x1408d5c48 <.text+0x8d4c48>
1408d5ba2: 48 83 c1 08                 	add	rcx, 0x8
1408d5ba6: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
1408d5baa: 0f 85 e7 00 00 00           	jne	0x1408d5c97 <.text+0x8d4c97>
1408d5bb0: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
1408d5bb7: 48 89 8d 70 03 00 00        	mov	qword ptr [rbp + 0x370], rcx
1408d5bbe: 48 83 c1 08                 	add	rcx, 0x8
1408d5bc2: 48 8b 55 38                 	mov	rdx, qword ptr [rbp + 0x38]
1408d5bc6: e8 25 15 34 00              	call	0x140c170f0 <.text+0xc160f0>
1408d5bcb: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d5bd2: 48 ff 00                    	inc	qword ptr [rax]
1408d5bd5: 0f 28 b5 80 03 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x380]
1408d5bdc: 0f 28 bd 90 03 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x390]
1408d5be3: 44 0f 28 85 a0 03 00 00     	movaps	xmm8, xmmword ptr [rbp + 0x3a0]
1408d5beb: 44 0f 28 8d b0 03 00 00     	movaps	xmm9, xmmword ptr [rbp + 0x3b0]
1408d5bf3: 44 0f 28 95 c0 03 00 00     	movaps	xmm10, xmmword ptr [rbp + 0x3c0]
1408d5bfb: 44 0f 28 9d d0 03 00 00     	movaps	xmm11, xmmword ptr [rbp + 0x3d0]
1408d5c03: 44 0f 28 a5 e0 03 00 00     	movaps	xmm12, xmmword ptr [rbp + 0x3e0]
1408d5c0b: 44 0f 28 ad f0 03 00 00     	movaps	xmm13, xmmword ptr [rbp + 0x3f0]
1408d5c13: 44 0f 28 b5 00 04 00 00     	movaps	xmm14, xmmword ptr [rbp + 0x400]
1408d5c1b: 44 0f 28 bd 10 04 00 00     	movaps	xmm15, xmmword ptr [rbp + 0x410]
1408d5c23: 48 81 c4 a8 04 00 00        	add	rsp, 0x4a8
1408d5c2a: 5b                          	pop	rbx
1408d5c2b: 5f                          	pop	rdi
1408d5c2c: 5e                          	pop	rsi
1408d5c2d: 41 5c                       	pop	r12
1408d5c2f: 41 5d                       	pop	r13
1408d5c31: 41 5e                       	pop	r14
1408d5c33: 41 5f                       	pop	r15
1408d5c35: 5d                          	pop	rbp
1408d5c36: c3                          	ret
1408d5c37: 48 8d 0d 6a e5 1c 02        	lea	rcx, [rip + 0x21ce56a]  # 0x142aa41a8
1408d5c3e: e8 cd 36 00 02              	call	0x1428d9310 <.text+0x28d8310>
1408d5c43: e9 f9 01 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5c48: 48 85 c0                    	test	rax, rax
1408d5c4b: 75 2d                       	jne	0x1408d5c7a <.text+0x8d4c7a>
1408d5c4d: 31 d2                       	xor	edx, edx
1408d5c4f: e8 7c 2b 02 02              	call	0x1428f87d0 <.text+0x28f77d0>
1408d5c54: 48 89 c1                    	mov	rcx, rax
1408d5c57: 48 85 c0                    	test	rax, rax
1408d5c5a: 0f 85 46 ff ff ff           	jne	0x1408d5ba6 <.text+0x8d4ba6>
1408d5c60: eb 18                       	jmp	0x1408d5c7a <.text+0x8d4c7a>
1408d5c62: 48 85 c0                    	test	rax, rax
1408d5c65: 75 13                       	jne	0x1408d5c7a <.text+0x8d4c7a>
1408d5c67: 31 d2                       	xor	edx, edx
1408d5c69: e8 62 2b 02 02              	call	0x1428f87d0 <.text+0x28f77d0>
1408d5c6e: 48 89 c1                    	mov	rcx, rax
1408d5c71: 48 85 c0                    	test	rax, rax
1408d5c74: 0f 85 80 aa ff ff           	jne	0x1408d06fa <.text+0x8cf6fa>
1408d5c7a: 48 8d 0d 1f 4e 36 02        	lea	rcx, [rip + 0x2364e1f]  # 0x142c3aaa0
1408d5c81: e8 4a 24 00 02              	call	0x1428d80d0 <.text+0x28d70d0>
1408d5c86: 48 8d 0d 4b e5 1c 02        	lea	rcx, [rip + 0x21ce54b]  # 0x142aa41d8
1408d5c8d: e8 7e 36 00 02              	call	0x1428d9310 <.text+0x28d8310>
1408d5c92: e9 aa 01 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5c97: 48 8d 0d d2 4e 36 02        	lea	rcx, [rip + 0x2364ed2]  # 0x142c3ab70
1408d5c9e: e8 ad 35 00 02              	call	0x1428d9250 <.text+0x28d8250>
1408d5ca3: 48 8d 0d e9 44 20 02        	lea	rcx, [rip + 0x22044e9]  # 0x142ada193
1408d5caa: 4c 8d 05 df e4 1c 02        	lea	r8, [rip + 0x21ce4df]   # 0x142aa4190
1408d5cb1: ba 32 00 00 00              	mov	edx, 0x32
1408d5cb6: e8 75 37 00 02              	call	0x1428d9430 <.text+0x28d8430>
1408d5cbb: e9 81 01 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5cc0: 48 8d 0d cc 44 20 02        	lea	rcx, [rip + 0x22044cc]  # 0x142ada193
1408d5cc7: 4c 8d 05 aa e4 1c 02        	lea	r8, [rip + 0x21ce4aa]   # 0x142aa4178
1408d5cce: ba 32 00 00 00              	mov	edx, 0x32
1408d5cd3: e8 58 37 00 02              	call	0x1428d9430 <.text+0x28d8430>
1408d5cd8: e9 64 01 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5cdd: 48 8d 0d af 44 20 02        	lea	rcx, [rip + 0x22044af]  # 0x142ada193
1408d5ce4: 4c 8d 05 d5 e4 1c 02        	lea	r8, [rip + 0x21ce4d5]   # 0x142aa41c0
1408d5ceb: ba 32 00 00 00              	mov	edx, 0x32
1408d5cf0: e8 3b 37 00 02              	call	0x1428d9430 <.text+0x28d8430>
1408d5cf5: e9 47 01 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5cfa: 48 8d 0d 57 fc 1c 02        	lea	rcx, [rip + 0x21cfc57]  # 0x142aa5958
1408d5d01: 4c 8d 05 c0 fc 1c 02        	lea	r8, [rip + 0x21cfcc0]   # 0x142aa59c8
1408d5d08: ba 2c 00 00 00              	mov	edx, 0x2c
1408d5d0d: e8 1e 37 00 02              	call	0x1428d9430 <.text+0x28d8430>
1408d5d12: e9 2a 01 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5d17: b9 10 00 00 00              	mov	ecx, 0x10
1408d5d1c: ba 90 00 00 00              	mov	edx, 0x90
1408d5d21: e8 da 32 00 02              	call	0x1428d9000 <.text+0x28d8000>
1408d5d26: e9 16 01 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5d2b: b9 10 00 00 00              	mov	ecx, 0x10
1408d5d30: ba 90 00 00 00              	mov	edx, 0x90
1408d5d35: e8 c6 32 00 02              	call	0x1428d9000 <.text+0x28d8000>
1408d5d3a: e9 02 01 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5d3f: b9 10 00 00 00              	mov	ecx, 0x10
1408d5d44: ba 90 00 00 00              	mov	edx, 0x90
1408d5d49: e8 b2 32 00 02              	call	0x1428d9000 <.text+0x28d8000>
1408d5d4e: e9 ee 00 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5d53: 31 c9                       	xor	ecx, ecx
1408d5d55: c6 85 1f 03 00 00 01        	mov	byte ptr [rbp + 0x31f], 0x1
1408d5d5c: 4c 8d 05 15 2e 36 02        	lea	r8, [rip + 0x2362e15]   # 0x142c38b78
1408d5d63: 4c 89 ca                    	mov	rdx, r9
1408d5d66: e8 78 32 00 02              	call	0x1428d8fe3 <.text+0x28d7fe3>
1408d5d6b: e9 d1 00 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5d70: 31 c9                       	xor	ecx, ecx
1408d5d72: c6 85 57 03 00 00 01        	mov	byte ptr [rbp + 0x357], 0x1
1408d5d79: 4c 8d 05 f8 2d 36 02        	lea	r8, [rip + 0x2362df8]   # 0x142c38b78
1408d5d80: 4c 89 ca                    	mov	rdx, r9
1408d5d83: e8 5b 32 00 02              	call	0x1428d8fe3 <.text+0x28d7fe3>
1408d5d88: e9 b4 00 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5d8d: b9 10 00 00 00              	mov	ecx, 0x10
1408d5d92: ba 90 00 00 00              	mov	edx, 0x90
1408d5d97: e8 64 32 00 02              	call	0x1428d9000 <.text+0x28d8000>
1408d5d9c: e9 a0 00 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5da1: b9 10 00 00 00              	mov	ecx, 0x10
1408d5da6: ba 90 00 00 00              	mov	edx, 0x90
1408d5dab: e8 50 32 00 02              	call	0x1428d9000 <.text+0x28d8000>
1408d5db0: e9 8c 00 00 00              	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5db5: b9 04 00 00 00              	mov	ecx, 0x4
1408d5dba: ba 18 00 00 00              	mov	edx, 0x18
1408d5dbf: e8 3c 32 00 02              	call	0x1428d9000 <.text+0x28d8000>
1408d5dc4: eb 7b                       	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5dc6: c6 85 51 03 00 00 01        	mov	byte ptr [rbp + 0x351], 0x1
1408d5dcd: 4c 8d 05 24 d8 1c 02        	lea	r8, [rip + 0x21cd824]   # 0x142aa35f8
1408d5dd4: b9 04 00 00 00              	mov	ecx, 0x4
1408d5dd9: ba 30 00 00 00              	mov	edx, 0x30
1408d5dde: e8 00 32 00 02              	call	0x1428d8fe3 <.text+0x28d7fe3>
1408d5de3: eb 5c                       	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5de5: c6 85 1d 03 00 00 01        	mov	byte ptr [rbp + 0x31d], 0x1
1408d5dec: 4c 8d 05 4d e5 1c 02        	lea	r8, [rip + 0x21ce54d]   # 0x142aa4340
1408d5df3: b9 04 00 00 00              	mov	ecx, 0x4
1408d5df8: ba 30 00 00 00              	mov	edx, 0x30
1408d5dfd: e8 e1 31 00 02              	call	0x1428d8fe3 <.text+0x28d7fe3>
1408d5e02: eb 3d                       	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5e04: b9 10 00 00 00              	mov	ecx, 0x10
1408d5e09: ba 90 00 00 00              	mov	edx, 0x90
1408d5e0e: e8 ed 31 00 02              	call	0x1428d9000 <.text+0x28d8000>
1408d5e13: eb 2c                       	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5e15: b9 10 00 00 00              	mov	ecx, 0x10
1408d5e1a: ba 90 00 00 00              	mov	edx, 0x90
1408d5e1f: e8 dc 31 00 02              	call	0x1428d9000 <.text+0x28d8000>
1408d5e24: eb 1b                       	jmp	0x1408d5e41 <.text+0x8d4e41>
1408d5e26: c6 85 1e 03 00 00 01        	mov	byte ptr [rbp + 0x31e], 0x1
1408d5e2d: 4c 8d 05 f4 e4 1c 02        	lea	r8, [rip + 0x21ce4f4]   # 0x142aa4328
1408d5e34: b9 04 00 00 00              	mov	ecx, 0x4
1408d5e39: 48 89 fa                    	mov	rdx, rdi
1408d5e3c: e8 a2 31 00 02              	call	0x1428d8fe3 <.text+0x28d7fe3>
1408d5e41: 0f 0b                       	ud2
1408d5e43: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1408d5e50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d5e55: 55                          	push	rbp
1408d5e56: 41 57                       	push	r15
1408d5e58: 41 56                       	push	r14
1408d5e5a: 41 55                       	push	r13
1408d5e5c: 41 54                       	push	r12
1408d5e5e: 56                          	push	rsi
1408d5e5f: 57                          	push	rdi
1408d5e60: 53                          	push	rbx
1408d5e61: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d5e68: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d5e6f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d5e75: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d5e7b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d5e81: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d5e87: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d5e8d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d5e97: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d5ea0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d5ea9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d5eb1: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d5eba: c6 85 6f 03 00 00 00        	mov	byte ptr [rbp + 0x36f], 0x0
1408d5ec1: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1408d5ec8: e8 13 bd d2 ff              	call	0x140601be0 <.text+0x600be0>
1408d5ecd: c6 85 6f 03 00 00 00        	mov	byte ptr [rbp + 0x36f], 0x0
1408d5ed4: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d5edd: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d5ee5: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d5eee: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d5ef7: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d5f01: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d5f07: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d5f0d: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d5f13: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d5f19: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d5f1f: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d5f26: 5b                          	pop	rbx
1408d5f27: 5f                          	pop	rdi
1408d5f28: 5e                          	pop	rsi
1408d5f29: 41 5c                       	pop	r12
1408d5f2b: 41 5d                       	pop	r13
1408d5f2d: 41 5e                       	pop	r14
1408d5f2f: 41 5f                       	pop	r15
1408d5f31: 5d                          	pop	rbp
1408d5f32: c3                          	ret
1408d5f33: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1408d5f40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d5f45: 55                          	push	rbp
1408d5f46: 41 57                       	push	r15
1408d5f48: 41 56                       	push	r14
1408d5f4a: 41 55                       	push	r13
1408d5f4c: 41 54                       	push	r12
1408d5f4e: 56                          	push	rsi
1408d5f4f: 57                          	push	rdi
1408d5f50: 53                          	push	rbx
1408d5f51: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d5f58: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d5f5f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d5f65: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d5f6b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d5f71: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d5f77: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d5f7d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d5f87: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d5f90: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d5f99: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d5fa1: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d5faa: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1408d5fb1: e8 2a bc d2 ff              	call	0x140601be0 <.text+0x600be0>
1408d5fb6: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d5fbe: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d5fc6: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d5fcf: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d5fd8: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d5fe1: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d5fe7: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d5fed: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d5ff3: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d5ff9: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d5fff: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d6006: 5b                          	pop	rbx
1408d6007: 5f                          	pop	rdi
1408d6008: 5e                          	pop	rsi
1408d6009: 41 5c                       	pop	r12
1408d600b: 41 5d                       	pop	r13
1408d600d: 41 5e                       	pop	r14
1408d600f: 41 5f                       	pop	r15
1408d6011: 5d                          	pop	rbp
1408d6012: c3                          	ret
1408d6013: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1408d6020: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d6025: 55                          	push	rbp
1408d6026: 41 57                       	push	r15
1408d6028: 41 56                       	push	r14
1408d602a: 41 55                       	push	r13
1408d602c: 41 54                       	push	r12
1408d602e: 56                          	push	rsi
1408d602f: 57                          	push	rdi
1408d6030: 53                          	push	rbx
1408d6031: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d6038: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d603f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d6045: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d604b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d6051: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d6057: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d605d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d6067: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d6070: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d6079: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d6081: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d608a: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1408d6091: e8 4a bb d2 ff              	call	0x140601be0 <.text+0x600be0>
1408d6096: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d609f: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d60a7: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d60b0: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d60b9: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d60c3: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d60c9: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d60cf: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d60d5: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d60db: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d60e1: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d60e8: 5b                          	pop	rbx
1408d60e9: 5f                          	pop	rdi
1408d60ea: 5e                          	pop	rsi
1408d60eb: 41 5c                       	pop	r12
1408d60ed: 41 5d                       	pop	r13
1408d60ef: 41 5e                       	pop	r14
1408d60f1: 41 5f                       	pop	r15
1408d60f3: 5d                          	pop	rbp
1408d60f4: c3                          	ret
1408d60f5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1408d6100: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d6105: 55                          	push	rbp
1408d6106: 41 57                       	push	r15
1408d6108: 41 56                       	push	r14
1408d610a: 41 55                       	push	r13
1408d610c: 41 54                       	push	r12
1408d610e: 56                          	push	rsi
1408d610f: 57                          	push	rdi
1408d6110: 53                          	push	rbx
1408d6111: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d6118: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d611f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d6125: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d612b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d6131: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d6137: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d613d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d6147: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d6150: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d6159: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d6161: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d616a: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1408d6171: e8 6a ba d2 ff              	call	0x140601be0 <.text+0x600be0>
1408d6176: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d617f: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d6187: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6190: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d6199: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d61a3: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d61a9: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d61af: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d61b5: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d61bb: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d61c1: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d61c8: 5b                          	pop	rbx
1408d61c9: 5f                          	pop	rdi
1408d61ca: 5e                          	pop	rsi
1408d61cb: 41 5c                       	pop	r12
1408d61cd: 41 5d                       	pop	r13
1408d61cf: 41 5e                       	pop	r14
1408d61d1: 41 5f                       	pop	r15
1408d61d3: 5d                          	pop	rbp
1408d61d4: c3                          	ret
1408d61d5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1408d61e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d61e5: 55                          	push	rbp
1408d61e6: 41 57                       	push	r15
1408d61e8: 41 56                       	push	r14
1408d61ea: 41 55                       	push	r13
1408d61ec: 41 54                       	push	r12
1408d61ee: 56                          	push	rsi
1408d61ef: 57                          	push	rdi
1408d61f0: 53                          	push	rbx
1408d61f1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d61f8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d61ff: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d6205: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d620b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d6211: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d6217: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d621d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d6227: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d6230: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d6239: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d6241: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d624a: 80 bd 1e 03 00 00 00        	cmp	byte ptr [rbp + 0x31e], 0x0
1408d6251: 0f 84 b2 00 00 00           	je	0x1408d6309 <.text+0x8d5309>
1408d6257: 48 8b 85 70 02 00 00        	mov	rax, qword ptr [rbp + 0x270]
1408d625e: 48 85 c0                    	test	rax, rax
1408d6261: 74 1a                       	je	0x1408d627d <.text+0x8d527d>
1408d6263: 48 8b 8d 78 02 00 00        	mov	rcx, qword ptr [rbp + 0x278]
1408d626a: 48 c1 e0 02                 	shl	rax, 0x2
1408d626e: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
1408d6272: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d6278: e8 a3 d9 d3 ff              	call	0x140613c20 <.text+0x612c20>
1408d627d: 48 8b 85 40 02 00 00        	mov	rax, qword ptr [rbp + 0x240]
1408d6284: 48 85 c0                    	test	rax, rax
1408d6287: 74 1a                       	je	0x1408d62a3 <.text+0x8d52a3>
1408d6289: 48 8b 8d 48 02 00 00        	mov	rcx, qword ptr [rbp + 0x248]
1408d6290: 48 c1 e0 02                 	shl	rax, 0x2
1408d6294: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
1408d6298: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d629e: e8 7d d9 d3 ff              	call	0x140613c20 <.text+0x612c20>
1408d62a3: c6 85 6f 03 00 00 00        	mov	byte ptr [rbp + 0x36f], 0x0
1408d62aa: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d62b3: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d62bb: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d62c4: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d62cd: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d62d7: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d62dd: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d62e3: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d62e9: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d62ef: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d62f5: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d62fc: 5b                          	pop	rbx
1408d62fd: 5f                          	pop	rdi
1408d62fe: 5e                          	pop	rsi
1408d62ff: 41 5c                       	pop	r12
1408d6301: 41 5d                       	pop	r13
1408d6303: 41 5e                       	pop	r14
1408d6305: 41 5f                       	pop	r15
1408d6307: 5d                          	pop	rbp
1408d6308: c3                          	ret
1408d6309: c6 85 6f 03 00 00 00        	mov	byte ptr [rbp + 0x36f], 0x0
1408d6310: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d6319: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d6321: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d632a: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d6333: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d633d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d6343: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d6349: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d634f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d6355: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d635b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d6362: 5b                          	pop	rbx
1408d6363: 5f                          	pop	rdi
1408d6364: 5e                          	pop	rsi
1408d6365: 41 5c                       	pop	r12
1408d6367: 41 5d                       	pop	r13
1408d6369: 41 5e                       	pop	r14
1408d636b: 41 5f                       	pop	r15
1408d636d: 5d                          	pop	rbp
1408d636e: c3                          	ret
1408d636f: 90                          	nop
1408d6370: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d6375: 55                          	push	rbp
1408d6376: 41 57                       	push	r15
1408d6378: 41 56                       	push	r14
1408d637a: 41 55                       	push	r13
1408d637c: 41 54                       	push	r12
1408d637e: 56                          	push	rsi
1408d637f: 57                          	push	rdi
1408d6380: 53                          	push	rbx
1408d6381: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d6388: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d638f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d6395: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d639b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d63a1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d63a7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d63ad: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d63b7: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d63c0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d63c9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d63d1: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d63da: 0f b6 9d 1d 03 00 00        	movzx	ebx, byte ptr [rbp + 0x31d]
1408d63e1: f6 c3 01                    	test	bl, 0x1
1408d63e4: 74 7c                       	je	0x1408d6462 <.text+0x8d5462>
1408d63e6: ba 30 00 00 00              	mov	edx, 0x30
1408d63eb: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d63f1: 48 8b 8d a0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1a0]
1408d63f8: e8 23 d8 d3 ff              	call	0x140613c20 <.text+0x612c20>
1408d63fd: 88 9d 51 03 00 00           	mov	byte ptr [rbp + 0x351], bl
1408d6403: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d640c: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d6414: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d641d: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d6426: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d6430: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d6436: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d643c: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d6442: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d6448: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d644e: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d6455: 5b                          	pop	rbx
1408d6456: 5f                          	pop	rdi
1408d6457: 5e                          	pop	rsi
1408d6458: 41 5c                       	pop	r12
1408d645a: 41 5d                       	pop	r13
1408d645c: 41 5e                       	pop	r14
1408d645e: 41 5f                       	pop	r15
1408d6460: 5d                          	pop	rbp
1408d6461: c3                          	ret
1408d6462: 88 9d 51 03 00 00           	mov	byte ptr [rbp + 0x351], bl
1408d6468: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d6471: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d6479: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6482: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d648b: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d6495: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d649b: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d64a1: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d64a7: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d64ad: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d64b3: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d64ba: 5b                          	pop	rbx
1408d64bb: 5f                          	pop	rdi
1408d64bc: 5e                          	pop	rsi
1408d64bd: 41 5c                       	pop	r12
1408d64bf: 41 5d                       	pop	r13
1408d64c1: 41 5e                       	pop	r14
1408d64c3: 41 5f                       	pop	r15
1408d64c5: 5d                          	pop	rbp
1408d64c6: c3                          	ret
1408d64c7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1408d64d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d64d5: 55                          	push	rbp
1408d64d6: 41 57                       	push	r15
1408d64d8: 41 56                       	push	r14
1408d64da: 41 55                       	push	r13
1408d64dc: 41 54                       	push	r12
1408d64de: 56                          	push	rsi
1408d64df: 57                          	push	rdi
1408d64e0: 53                          	push	rbx
1408d64e1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d64e8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d64ef: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d64f5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d64fb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d6501: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d6507: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d650d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d6517: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d6520: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d6529: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d6531: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d653a: 80 bd 55 03 00 00 00        	cmp	byte ptr [rbp + 0x355], 0x0
1408d6541: 74 78                       	je	0x1408d65bb <.text+0x8d55bb>
1408d6543: 48 8b 85 a0 00 00 00        	mov	rax, qword ptr [rbp + 0xa0]
1408d654a: f0                          	lock
1408d654b: 48 ff 08                    	dec	qword ptr [rax]
1408d654e: 75 0c                       	jne	0x1408d655c <.text+0x8d555c>
1408d6550: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1408d6557: e8 c4 60 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d655c: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d6565: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d656d: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6576: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d657f: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d6589: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d658f: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d6595: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d659b: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d65a1: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d65a7: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d65ae: 5b                          	pop	rbx
1408d65af: 5f                          	pop	rdi
1408d65b0: 5e                          	pop	rsi
1408d65b1: 41 5c                       	pop	r12
1408d65b3: 41 5d                       	pop	r13
1408d65b5: 41 5e                       	pop	r14
1408d65b7: 41 5f                       	pop	r15
1408d65b9: 5d                          	pop	rbp
1408d65ba: c3                          	ret
1408d65bb: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d65c4: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d65cc: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d65d5: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d65de: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d65e8: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d65ee: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d65f4: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d65fa: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d6600: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d6606: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d660d: 5b                          	pop	rbx
1408d660e: 5f                          	pop	rdi
1408d660f: 5e                          	pop	rsi
1408d6610: 41 5c                       	pop	r12
1408d6612: 41 5d                       	pop	r13
1408d6614: 41 5e                       	pop	r14
1408d6616: 41 5f                       	pop	r15
1408d6618: 5d                          	pop	rbp
1408d6619: c3                          	ret
1408d661a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1408d6620: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d6625: 55                          	push	rbp
1408d6626: 41 57                       	push	r15
1408d6628: 41 56                       	push	r14
1408d662a: 41 55                       	push	r13
1408d662c: 41 54                       	push	r12
1408d662e: 56                          	push	rsi
1408d662f: 57                          	push	rdi
1408d6630: 53                          	push	rbx
1408d6631: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d6638: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d663f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d6645: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d664b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d6651: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d6657: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d665d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d6667: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d6670: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d6679: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d6681: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d668a: 80 bd 54 03 00 00 00        	cmp	byte ptr [rbp + 0x354], 0x0
1408d6691: 74 78                       	je	0x1408d670b <.text+0x8d570b>
1408d6693: 48 8b 85 98 00 00 00        	mov	rax, qword ptr [rbp + 0x98]
1408d669a: f0                          	lock
1408d669b: 48 ff 08                    	dec	qword ptr [rax]
1408d669e: 75 0c                       	jne	0x1408d66ac <.text+0x8d56ac>
1408d66a0: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1408d66a7: e8 74 5f 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d66ac: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d66b5: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d66bd: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d66c6: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d66cf: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d66d9: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d66df: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d66e5: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d66eb: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d66f1: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d66f7: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d66fe: 5b                          	pop	rbx
1408d66ff: 5f                          	pop	rdi
1408d6700: 5e                          	pop	rsi
1408d6701: 41 5c                       	pop	r12
1408d6703: 41 5d                       	pop	r13
1408d6705: 41 5e                       	pop	r14
1408d6707: 41 5f                       	pop	r15
1408d6709: 5d                          	pop	rbp
1408d670a: c3                          	ret
1408d670b: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d6714: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d671c: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6725: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d672e: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d6738: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d673e: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d6744: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d674a: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d6750: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d6756: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d675d: 5b                          	pop	rbx
1408d675e: 5f                          	pop	rdi
1408d675f: 5e                          	pop	rsi
1408d6760: 41 5c                       	pop	r12
1408d6762: 41 5d                       	pop	r13
1408d6764: 41 5e                       	pop	r14
1408d6766: 41 5f                       	pop	r15
1408d6768: 5d                          	pop	rbp
1408d6769: c3                          	ret
1408d676a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1408d6770: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d6775: 55                          	push	rbp
1408d6776: 41 57                       	push	r15
1408d6778: 41 56                       	push	r14
1408d677a: 41 55                       	push	r13
1408d677c: 41 54                       	push	r12
1408d677e: 56                          	push	rsi
1408d677f: 57                          	push	rdi
1408d6780: 53                          	push	rbx
1408d6781: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d6788: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d678f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d6795: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d679b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d67a1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d67a7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d67ad: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d67b7: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d67c0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d67c9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d67d1: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d67da: f6 85 51 03 00 00 01        	test	byte ptr [rbp + 0x351], 0x1
1408d67e1: 74 74                       	je	0x1408d6857 <.text+0x8d5857>
1408d67e3: ba 18 00 00 00              	mov	edx, 0x18
1408d67e8: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d67ee: 48 8b 8d 40 03 00 00        	mov	rcx, qword ptr [rbp + 0x340]
1408d67f5: e8 26 d4 d3 ff              	call	0x140613c20 <.text+0x612c20>
1408d67fa: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d6802: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d680a: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6813: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d681c: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d6825: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d682b: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d6831: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d6837: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d683d: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d6843: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d684a: 5b                          	pop	rbx
1408d684b: 5f                          	pop	rdi
1408d684c: 5e                          	pop	rsi
1408d684d: 41 5c                       	pop	r12
1408d684f: 41 5d                       	pop	r13
1408d6851: 41 5e                       	pop	r14
1408d6853: 41 5f                       	pop	r15
1408d6855: 5d                          	pop	rbp
1408d6856: c3                          	ret
1408d6857: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d685f: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d6867: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6870: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d6879: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d6882: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d6888: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d688e: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d6894: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d689a: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d68a0: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d68a7: 5b                          	pop	rbx
1408d68a8: 5f                          	pop	rdi
1408d68a9: 5e                          	pop	rsi
1408d68aa: 41 5c                       	pop	r12
1408d68ac: 41 5d                       	pop	r13
1408d68ae: 41 5e                       	pop	r14
1408d68b0: 41 5f                       	pop	r15
1408d68b2: 5d                          	pop	rbp
1408d68b3: c3                          	ret
1408d68b4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1408d68c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d68c5: 55                          	push	rbp
1408d68c6: 41 57                       	push	r15
1408d68c8: 41 56                       	push	r14
1408d68ca: 41 55                       	push	r13
1408d68cc: 41 54                       	push	r12
1408d68ce: 56                          	push	rsi
1408d68cf: 57                          	push	rdi
1408d68d0: 53                          	push	rbx
1408d68d1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d68d8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d68df: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d68e5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d68eb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d68f1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d68f7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d68fd: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d6907: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d6910: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d6919: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d6921: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d692a: 80 bd 50 03 00 00 00        	cmp	byte ptr [rbp + 0x350], 0x0
1408d6931: 74 76                       	je	0x1408d69a9 <.text+0x8d59a9>
1408d6933: 48 8b 85 40 03 00 00        	mov	rax, qword ptr [rbp + 0x340]
1408d693a: f0                          	lock
1408d693b: 48 ff 08                    	dec	qword ptr [rax]
1408d693e: 75 0c                       	jne	0x1408d694c <.text+0x8d594c>
1408d6940: 48 8d 8d c8 02 00 00        	lea	rcx, [rbp + 0x2c8]
1408d6947: e8 d4 5c 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d694c: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d6954: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d695c: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6965: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d696e: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d6977: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d697d: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d6983: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d6989: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d698f: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d6995: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d699c: 5b                          	pop	rbx
1408d699d: 5f                          	pop	rdi
1408d699e: 5e                          	pop	rsi
1408d699f: 41 5c                       	pop	r12
1408d69a1: 41 5d                       	pop	r13
1408d69a3: 41 5e                       	pop	r14
1408d69a5: 41 5f                       	pop	r15
1408d69a7: 5d                          	pop	rbp
1408d69a8: c3                          	ret
1408d69a9: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d69b1: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d69b9: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d69c2: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d69cb: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d69d4: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d69da: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d69e0: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d69e6: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d69ec: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d69f2: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d69f9: 5b                          	pop	rbx
1408d69fa: 5f                          	pop	rdi
1408d69fb: 5e                          	pop	rsi
1408d69fc: 41 5c                       	pop	r12
1408d69fe: 41 5d                       	pop	r13
1408d6a00: 41 5e                       	pop	r14
1408d6a02: 41 5f                       	pop	r15
1408d6a04: 5d                          	pop	rbp
1408d6a05: c3                          	ret
1408d6a06: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1408d6a10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d6a15: 55                          	push	rbp
1408d6a16: 41 57                       	push	r15
1408d6a18: 41 56                       	push	r14
1408d6a1a: 41 55                       	push	r13
1408d6a1c: 41 54                       	push	r12
1408d6a1e: 56                          	push	rsi
1408d6a1f: 57                          	push	rdi
1408d6a20: 53                          	push	rbx
1408d6a21: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d6a28: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d6a2f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d6a35: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d6a3b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d6a41: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d6a47: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d6a4d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d6a57: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d6a60: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d6a69: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d6a71: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d6a7a: 0f b6 9d 6f 03 00 00        	movzx	ebx, byte ptr [rbp + 0x36f]
1408d6a81: 48 8b 55 50                 	mov	rdx, qword ptr [rbp + 0x50]
1408d6a85: 48 85 d2                    	test	rdx, rdx
1408d6a88: 74 13                       	je	0x1408d6a9d <.text+0x8d5a9d>
1408d6a8a: 48 8b 4d 58                 	mov	rcx, qword ptr [rbp + 0x58]
1408d6a8e: 48 c1 e2 03                 	shl	rdx, 0x3
1408d6a92: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d6a98: e8 83 d1 d3 ff              	call	0x140613c20 <.text+0x612c20>
1408d6a9d: 80 e3 01                    	and	bl, 0x1
1408d6aa0: 88 9d 52 03 00 00           	mov	byte ptr [rbp + 0x352], bl
1408d6aa6: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d6aaf: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d6ab7: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6ac0: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d6ac9: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d6ad3: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d6ad9: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d6adf: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d6ae5: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d6aeb: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d6af1: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d6af8: 5b                          	pop	rbx
1408d6af9: 5f                          	pop	rdi
1408d6afa: 5e                          	pop	rsi
1408d6afb: 41 5c                       	pop	r12
1408d6afd: 41 5d                       	pop	r13
1408d6aff: 41 5e                       	pop	r14
1408d6b01: 41 5f                       	pop	r15
1408d6b03: 5d                          	pop	rbp
1408d6b04: c3                          	ret
1408d6b05: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1408d6b10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d6b15: 55                          	push	rbp
1408d6b16: 41 57                       	push	r15
1408d6b18: 41 56                       	push	r14
1408d6b1a: 41 55                       	push	r13
1408d6b1c: 41 54                       	push	r12
1408d6b1e: 56                          	push	rsi
1408d6b1f: 57                          	push	rdi
1408d6b20: 53                          	push	rbx
1408d6b21: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d6b28: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d6b2f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d6b35: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d6b3b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d6b41: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d6b47: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d6b4d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d6b57: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d6b60: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d6b69: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d6b71: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d6b7a: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1408d6b81: e8 5a b0 d2 ff              	call	0x140601be0 <.text+0x600be0>
1408d6b86: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d6b8e: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d6b96: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6b9f: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d6ba8: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d6bb1: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d6bb7: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d6bbd: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d6bc3: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d6bc9: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d6bcf: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d6bd6: 5b                          	pop	rbx
1408d6bd7: 5f                          	pop	rdi
1408d6bd8: 5e                          	pop	rsi
1408d6bd9: 41 5c                       	pop	r12
1408d6bdb: 41 5d                       	pop	r13
1408d6bdd: 41 5e                       	pop	r14
1408d6bdf: 41 5f                       	pop	r15
1408d6be1: 5d                          	pop	rbp
1408d6be2: c3                          	ret
1408d6be3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1408d6bf0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d6bf5: 55                          	push	rbp
1408d6bf6: 41 57                       	push	r15
1408d6bf8: 41 56                       	push	r14
1408d6bfa: 41 55                       	push	r13
1408d6bfc: 41 54                       	push	r12
1408d6bfe: 56                          	push	rsi
1408d6bff: 57                          	push	rdi
1408d6c00: 53                          	push	rbx
1408d6c01: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d6c08: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d6c0f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d6c15: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d6c1b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d6c21: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d6c27: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d6c2d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d6c37: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d6c40: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d6c49: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d6c51: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d6c5a: 80 bd 52 03 00 00 00        	cmp	byte ptr [rbp + 0x352], 0x0
1408d6c61: 74 7d                       	je	0x1408d6ce0 <.text+0x8d5ce0>
1408d6c63: 48 8b 45 68                 	mov	rax, qword ptr [rbp + 0x68]
1408d6c67: 48 85 c0                    	test	rax, rax
1408d6c6a: 74 17                       	je	0x1408d6c83 <.text+0x8d5c83>
1408d6c6c: 48 8b 4d 70                 	mov	rcx, qword ptr [rbp + 0x70]
1408d6c70: 48 c1 e0 02                 	shl	rax, 0x2
1408d6c74: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
1408d6c78: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d6c7e: e8 9d cf d3 ff              	call	0x140613c20 <.text+0x612c20>
1408d6c83: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d6c8b: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d6c93: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6c9c: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d6ca5: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d6cae: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d6cb4: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d6cba: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d6cc0: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d6cc6: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d6ccc: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d6cd3: 5b                          	pop	rbx
1408d6cd4: 5f                          	pop	rdi
1408d6cd5: 5e                          	pop	rsi
1408d6cd6: 41 5c                       	pop	r12
1408d6cd8: 41 5d                       	pop	r13
1408d6cda: 41 5e                       	pop	r14
1408d6cdc: 41 5f                       	pop	r15
1408d6cde: 5d                          	pop	rbp
1408d6cdf: c3                          	ret
1408d6ce0: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d6ce8: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d6cf0: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6cf9: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d6d02: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d6d0b: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d6d11: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d6d17: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d6d1d: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d6d23: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d6d29: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d6d30: 5b                          	pop	rbx
1408d6d31: 5f                          	pop	rdi
1408d6d32: 5e                          	pop	rsi
1408d6d33: 41 5c                       	pop	r12
1408d6d35: 41 5d                       	pop	r13
1408d6d37: 41 5e                       	pop	r14
1408d6d39: 41 5f                       	pop	r15
1408d6d3b: 5d                          	pop	rbp
1408d6d3c: c3                          	ret
1408d6d3d: 0f 1f 00                    	nop	dword ptr [rax]
1408d6d40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d6d45: 55                          	push	rbp
1408d6d46: 41 57                       	push	r15
1408d6d48: 41 56                       	push	r14
1408d6d4a: 41 55                       	push	r13
1408d6d4c: 41 54                       	push	r12
1408d6d4e: 56                          	push	rsi
1408d6d4f: 57                          	push	rdi
1408d6d50: 53                          	push	rbx
1408d6d51: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d6d58: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d6d5f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d6d65: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d6d6b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d6d71: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d6d77: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d6d7d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d6d87: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d6d90: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d6d99: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d6da1: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d6daa: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1408d6db1: e8 2a ae d2 ff              	call	0x140601be0 <.text+0x600be0>
1408d6db6: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d6dbe: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d6dc6: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6dcf: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d6dd8: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d6de1: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d6de7: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d6ded: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d6df3: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d6df9: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d6dff: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d6e06: 5b                          	pop	rbx
1408d6e07: 5f                          	pop	rdi
1408d6e08: 5e                          	pop	rsi
1408d6e09: 41 5c                       	pop	r12
1408d6e0b: 41 5d                       	pop	r13
1408d6e0d: 41 5e                       	pop	r14
1408d6e0f: 41 5f                       	pop	r15
1408d6e11: 5d                          	pop	rbp
1408d6e12: c3                          	ret
1408d6e13: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1408d6e20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d6e25: 55                          	push	rbp
1408d6e26: 41 57                       	push	r15
1408d6e28: 41 56                       	push	r14
1408d6e2a: 41 55                       	push	r13
1408d6e2c: 41 54                       	push	r12
1408d6e2e: 56                          	push	rsi
1408d6e2f: 57                          	push	rdi
1408d6e30: 53                          	push	rbx
1408d6e31: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d6e38: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d6e3f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d6e45: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d6e4b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d6e51: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d6e57: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d6e5d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d6e67: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d6e70: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d6e79: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d6e81: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d6e8a: 80 bd 53 03 00 00 00        	cmp	byte ptr [rbp + 0x353], 0x0
1408d6e91: 74 76                       	je	0x1408d6f09 <.text+0x8d5f09>
1408d6e93: 48 8b 85 40 03 00 00        	mov	rax, qword ptr [rbp + 0x340]
1408d6e9a: f0                          	lock
1408d6e9b: 48 ff 08                    	dec	qword ptr [rax]
1408d6e9e: 75 0c                       	jne	0x1408d6eac <.text+0x8d5eac>
1408d6ea0: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1408d6ea7: e8 74 57 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d6eac: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d6eb4: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d6ebc: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6ec5: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d6ece: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d6ed7: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d6edd: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d6ee3: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d6ee9: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d6eef: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d6ef5: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d6efc: 5b                          	pop	rbx
1408d6efd: 5f                          	pop	rdi
1408d6efe: 5e                          	pop	rsi
1408d6eff: 41 5c                       	pop	r12
1408d6f01: 41 5d                       	pop	r13
1408d6f03: 41 5e                       	pop	r14
1408d6f05: 41 5f                       	pop	r15
1408d6f07: 5d                          	pop	rbp
1408d6f08: c3                          	ret
1408d6f09: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d6f11: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d6f19: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d6f22: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d6f2b: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d6f34: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d6f3a: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d6f40: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d6f46: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d6f4c: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d6f52: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d6f59: 5b                          	pop	rbx
1408d6f5a: 5f                          	pop	rdi
1408d6f5b: 5e                          	pop	rsi
1408d6f5c: 41 5c                       	pop	r12
1408d6f5e: 41 5d                       	pop	r13
1408d6f60: 41 5e                       	pop	r14
1408d6f62: 41 5f                       	pop	r15
1408d6f64: 5d                          	pop	rbp
1408d6f65: c3                          	ret
1408d6f66: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1408d6f70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d6f75: 55                          	push	rbp
1408d6f76: 41 57                       	push	r15
1408d6f78: 41 56                       	push	r14
1408d6f7a: 41 55                       	push	r13
1408d6f7c: 41 54                       	push	r12
1408d6f7e: 56                          	push	rsi
1408d6f7f: 57                          	push	rdi
1408d6f80: 53                          	push	rbx
1408d6f81: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d6f88: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d6f8f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d6f95: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d6f9b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d6fa1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d6fa7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d6fad: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d6fb7: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d6fc0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d6fc9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d6fd1: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d6fda: 80 bd 56 03 00 00 00        	cmp	byte ptr [rbp + 0x356], 0x0
1408d6fe1: 74 76                       	je	0x1408d7059 <.text+0x8d6059>
1408d6fe3: 48 8b 85 68 02 00 00        	mov	rax, qword ptr [rbp + 0x268]
1408d6fea: f0                          	lock
1408d6feb: 48 ff 08                    	dec	qword ptr [rax]
1408d6fee: 75 0c                       	jne	0x1408d6ffc <.text+0x8d5ffc>
1408d6ff0: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1408d6ff7: e8 24 56 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d6ffc: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7004: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d700c: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7015: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d701e: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7027: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d702d: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7033: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7039: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d703f: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7045: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d704c: 5b                          	pop	rbx
1408d704d: 5f                          	pop	rdi
1408d704e: 5e                          	pop	rsi
1408d704f: 41 5c                       	pop	r12
1408d7051: 41 5d                       	pop	r13
1408d7053: 41 5e                       	pop	r14
1408d7055: 41 5f                       	pop	r15
1408d7057: 5d                          	pop	rbp
1408d7058: c3                          	ret
1408d7059: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7061: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d7069: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7072: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d707b: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7084: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d708a: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7090: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7096: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d709c: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d70a2: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d70a9: 5b                          	pop	rbx
1408d70aa: 5f                          	pop	rdi
1408d70ab: 5e                          	pop	rsi
1408d70ac: 41 5c                       	pop	r12
1408d70ae: 41 5d                       	pop	r13
1408d70b0: 41 5e                       	pop	r14
1408d70b2: 41 5f                       	pop	r15
1408d70b4: 5d                          	pop	rbp
1408d70b5: c3                          	ret
1408d70b6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1408d70c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d70c5: 55                          	push	rbp
1408d70c6: 41 57                       	push	r15
1408d70c8: 41 56                       	push	r14
1408d70ca: 41 55                       	push	r13
1408d70cc: 41 54                       	push	r12
1408d70ce: 56                          	push	rsi
1408d70cf: 57                          	push	rdi
1408d70d0: 53                          	push	rbx
1408d70d1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d70d8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d70df: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d70e5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d70eb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d70f1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d70f7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d70fd: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d7107: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d7110: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d7119: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d7121: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d712a: 0f b6 9d 1f 03 00 00        	movzx	ebx, byte ptr [rbp + 0x31f]
1408d7131: f6 c3 01                    	test	bl, 0x1
1408d7134: 0f 84 90 00 00 00           	je	0x1408d71ca <.text+0x8d61ca>
1408d713a: 48 83 bd 98 02 00 00 00     	cmp	qword ptr [rbp + 0x298], 0x0
1408d7142: 74 21                       	je	0x1408d7165 <.text+0x8d6165>
1408d7144: 48 8b 85 98 02 00 00        	mov	rax, qword ptr [rbp + 0x298]
1408d714b: 48 c1 e0 02                 	shl	rax, 0x2
1408d714f: 48 8d 14 c0                 	lea	rdx, [rax + 8*rax]
1408d7153: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d7159: 48 8b 8d 88 00 00 00        	mov	rcx, qword ptr [rbp + 0x88]
1408d7160: e8 bb ca d3 ff              	call	0x140613c20 <.text+0x612c20>
1408d7165: 88 9d 57 03 00 00           	mov	byte ptr [rbp + 0x357], bl
1408d716b: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d7174: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d717c: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7185: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d718e: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d7198: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d719e: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d71a4: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d71aa: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d71b0: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d71b6: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d71bd: 5b                          	pop	rbx
1408d71be: 5f                          	pop	rdi
1408d71bf: 5e                          	pop	rsi
1408d71c0: 41 5c                       	pop	r12
1408d71c2: 41 5d                       	pop	r13
1408d71c4: 41 5e                       	pop	r14
1408d71c6: 41 5f                       	pop	r15
1408d71c8: 5d                          	pop	rbp
1408d71c9: c3                          	ret
1408d71ca: 88 9d 57 03 00 00           	mov	byte ptr [rbp + 0x357], bl
1408d71d0: 66 0f 6f b4 24 c0 00 00 00  	movdqa	xmm6, xmmword ptr [rsp + 0xc0]
1408d71d9: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d71e1: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d71ea: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d71f3: 66 44 0f 6f 94 24 80 00 00 00       	movdqa	xmm10, xmmword ptr [rsp + 0x80]
1408d71fd: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d7203: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7209: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d720f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7215: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d721b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d7222: 5b                          	pop	rbx
1408d7223: 5f                          	pop	rdi
1408d7224: 5e                          	pop	rsi
1408d7225: 41 5c                       	pop	r12
1408d7227: 41 5d                       	pop	r13
1408d7229: 41 5e                       	pop	r14
1408d722b: 41 5f                       	pop	r15
1408d722d: 5d                          	pop	rbp
1408d722e: c3                          	ret
1408d722f: 90                          	nop
1408d7230: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d7235: 55                          	push	rbp
1408d7236: 41 57                       	push	r15
1408d7238: 41 56                       	push	r14
1408d723a: 41 55                       	push	r13
1408d723c: 41 54                       	push	r12
1408d723e: 56                          	push	rsi
1408d723f: 57                          	push	rdi
1408d7240: 53                          	push	rbx
1408d7241: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d7248: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d724f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d7255: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d725b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d7261: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d7267: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d726d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d7277: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d7280: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d7289: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d7291: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d729a: f6 85 57 03 00 00 01        	test	byte ptr [rbp + 0x357], 0x1
1408d72a1: 0f 84 83 00 00 00           	je	0x1408d732a <.text+0x8d632a>
1408d72a7: 48 8b 85 70 02 00 00        	mov	rax, qword ptr [rbp + 0x270]
1408d72ae: 48 85 c0                    	test	rax, rax
1408d72b1: 74 1a                       	je	0x1408d72cd <.text+0x8d62cd>
1408d72b3: 48 8b 8d 78 02 00 00        	mov	rcx, qword ptr [rbp + 0x278]
1408d72ba: 48 c1 e0 02                 	shl	rax, 0x2
1408d72be: 48 8d 14 c0                 	lea	rdx, [rax + 8*rax]
1408d72c2: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d72c8: e8 53 c9 d3 ff              	call	0x140613c20 <.text+0x612c20>
1408d72cd: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d72d5: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d72dd: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d72e6: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d72ef: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d72f8: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d72fe: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7304: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d730a: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7310: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7316: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d731d: 5b                          	pop	rbx
1408d731e: 5f                          	pop	rdi
1408d731f: 5e                          	pop	rsi
1408d7320: 41 5c                       	pop	r12
1408d7322: 41 5d                       	pop	r13
1408d7324: 41 5e                       	pop	r14
1408d7326: 41 5f                       	pop	r15
1408d7328: 5d                          	pop	rbp
1408d7329: c3                          	ret
1408d732a: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7332: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d733a: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7343: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d734c: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7355: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d735b: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7361: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7367: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d736d: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7373: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d737a: 5b                          	pop	rbx
1408d737b: 5f                          	pop	rdi
1408d737c: 5e                          	pop	rsi
1408d737d: 41 5c                       	pop	r12
1408d737f: 41 5d                       	pop	r13
1408d7381: 41 5e                       	pop	r14
1408d7383: 41 5f                       	pop	r15
1408d7385: 5d                          	pop	rbp
1408d7386: c3                          	ret
1408d7387: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1408d7390: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d7395: 55                          	push	rbp
1408d7396: 41 57                       	push	r15
1408d7398: 41 56                       	push	r14
1408d739a: 41 55                       	push	r13
1408d739c: 41 54                       	push	r12
1408d739e: 56                          	push	rsi
1408d739f: 57                          	push	rdi
1408d73a0: 53                          	push	rbx
1408d73a1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d73a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d73af: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d73b5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d73bb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d73c1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d73c7: 66 44 0f 7f 5c 24 70        	movdqa	xmmword ptr [rsp + 0x70], xmm11
1408d73ce: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
1408d73d7: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d73e0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d73e9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d73f1: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1408d73f9: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1408d7400: e8 db a7 d2 ff              	call	0x140601be0 <.text+0x600be0>
1408d7405: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d740d: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d7415: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d741e: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d7427: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7430: 66 44 0f 6f 5c 24 70        	movdqa	xmm11, xmmword ptr [rsp + 0x70]
1408d7437: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d743d: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7443: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7449: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d744f: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d7456: 5b                          	pop	rbx
1408d7457: 5f                          	pop	rdi
1408d7458: 5e                          	pop	rsi
1408d7459: 41 5c                       	pop	r12
1408d745b: 41 5d                       	pop	r13
1408d745d: 41 5e                       	pop	r14
1408d745f: 41 5f                       	pop	r15
1408d7461: 5d                          	pop	rbp
1408d7462: c3                          	ret
1408d7463: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1408d7470: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d7475: 55                          	push	rbp
1408d7476: 41 57                       	push	r15
1408d7478: 41 56                       	push	r14
1408d747a: 41 55                       	push	r13
1408d747c: 41 54                       	push	r12
1408d747e: 56                          	push	rsi
1408d747f: 57                          	push	rdi
1408d7480: 53                          	push	rbx
1408d7481: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d7488: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d748f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d7495: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d749b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d74a1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d74a7: 66 44 0f 7f 5c 24 70        	movdqa	xmmword ptr [rsp + 0x70], xmm11
1408d74ae: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
1408d74b7: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d74c0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d74c9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d74d1: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1408d74d9: 48 8b 85 f0 02 00 00        	mov	rax, qword ptr [rbp + 0x2f0]
1408d74e0: f0                          	lock
1408d74e1: 48 ff 08                    	dec	qword ptr [rax]
1408d74e4: 75 0c                       	jne	0x1408d74f2 <.text+0x8d64f2>
1408d74e6: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1408d74ed: e8 de 84 d7 ff              	call	0x14064f9d0 <.text+0x64e9d0>
1408d74f2: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d74fa: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d7502: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d750b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d7514: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d751d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d7523: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7529: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d752f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7535: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d753b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d7542: 5b                          	pop	rbx
1408d7543: 5f                          	pop	rdi
1408d7544: 5e                          	pop	rsi
1408d7545: 41 5c                       	pop	r12
1408d7547: 41 5d                       	pop	r13
1408d7549: 41 5e                       	pop	r14
1408d754b: 41 5f                       	pop	r15
1408d754d: 5d                          	pop	rbp
1408d754e: c3                          	ret
1408d754f: 90                          	nop
1408d7550: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d7555: 55                          	push	rbp
1408d7556: 41 57                       	push	r15
1408d7558: 41 56                       	push	r14
1408d755a: 41 55                       	push	r13
1408d755c: 41 54                       	push	r12
1408d755e: 56                          	push	rsi
1408d755f: 57                          	push	rdi
1408d7560: 53                          	push	rbx
1408d7561: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d7568: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d756f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d7575: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d757b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d7581: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d7587: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d758d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d7597: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d75a0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d75a9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d75b1: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d75ba: 48 8b 85 a0 01 00 00        	mov	rax, qword ptr [rbp + 0x1a0]
1408d75c1: 48 3b 85 40 03 00 00        	cmp	rax, qword ptr [rbp + 0x340]
1408d75c8: 75 5d                       	jne	0x1408d7627 <.text+0x8d6627>
1408d75ca: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d75d2: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d75da: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d75e3: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d75ec: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d75f5: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d75fb: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7601: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7607: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d760d: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7613: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d761a: 5b                          	pop	rbx
1408d761b: 5f                          	pop	rdi
1408d761c: 5e                          	pop	rsi
1408d761d: 41 5c                       	pop	r12
1408d761f: 41 5d                       	pop	r13
1408d7621: 41 5e                       	pop	r14
1408d7623: 41 5f                       	pop	r15
1408d7625: 5d                          	pop	rbp
1408d7626: c3                          	ret
1408d7627: 48 8b 85 40 02 00 00        	mov	rax, qword ptr [rbp + 0x240]
1408d762e: 48 85 c0                    	test	rax, rax
1408d7631: 74 1a                       	je	0x1408d764d <.text+0x8d664d>
1408d7633: 48 8b 8d 48 02 00 00        	mov	rcx, qword ptr [rbp + 0x248]
1408d763a: 48 c1 e0 02                 	shl	rax, 0x2
1408d763e: 48 8d 14 c0                 	lea	rdx, [rax + 8*rax]
1408d7642: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d7648: e8 d3 c5 d3 ff              	call	0x140613c20 <.text+0x612c20>
1408d764d: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7655: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d765d: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7666: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d766f: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7678: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d767e: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7684: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d768a: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7690: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7696: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d769d: 5b                          	pop	rbx
1408d769e: 5f                          	pop	rdi
1408d769f: 5e                          	pop	rsi
1408d76a0: 41 5c                       	pop	r12
1408d76a2: 41 5d                       	pop	r13
1408d76a4: 41 5e                       	pop	r14
1408d76a6: 41 5f                       	pop	r15
1408d76a8: 5d                          	pop	rbp
1408d76a9: c3                          	ret
1408d76aa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1408d76b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d76b5: 55                          	push	rbp
1408d76b6: 41 57                       	push	r15
1408d76b8: 41 56                       	push	r14
1408d76ba: 41 55                       	push	r13
1408d76bc: 41 54                       	push	r12
1408d76be: 56                          	push	rsi
1408d76bf: 57                          	push	rdi
1408d76c0: 53                          	push	rbx
1408d76c1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d76c8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d76cf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d76d5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d76db: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d76e1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d76e7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d76ed: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
1408d76f6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d76ff: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d7708: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d7710: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1408d7718: 48 8b 85 a0 01 00 00        	mov	rax, qword ptr [rbp + 0x1a0]
1408d771f: 48 3b 85 40 03 00 00        	cmp	rax, qword ptr [rbp + 0x340]
1408d7726: 75 5d                       	jne	0x1408d7785 <.text+0x8d6785>
1408d7728: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7730: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d7738: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7741: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d774a: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7753: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d7759: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d775f: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7765: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d776b: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7771: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d7778: 5b                          	pop	rbx
1408d7779: 5f                          	pop	rdi
1408d777a: 5e                          	pop	rsi
1408d777b: 41 5c                       	pop	r12
1408d777d: 41 5d                       	pop	r13
1408d777f: 41 5e                       	pop	r14
1408d7781: 41 5f                       	pop	r15
1408d7783: 5d                          	pop	rbp
1408d7784: c3                          	ret
1408d7785: 48 8b 85 78 01 00 00        	mov	rax, qword ptr [rbp + 0x178]
1408d778c: 48 85 c0                    	test	rax, rax
1408d778f: 74 1a                       	je	0x1408d77ab <.text+0x8d67ab>
1408d7791: 48 8b 8d 80 01 00 00        	mov	rcx, qword ptr [rbp + 0x180]
1408d7798: 48 c1 e0 02                 	shl	rax, 0x2
1408d779c: 48 8d 14 c0                 	lea	rdx, [rax + 8*rax]
1408d77a0: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d77a6: e8 75 c4 d3 ff              	call	0x140613c20 <.text+0x612c20>
1408d77ab: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d77b3: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d77bb: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d77c4: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d77cd: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d77d6: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d77dc: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d77e2: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d77e8: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d77ee: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d77f4: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d77fb: 5b                          	pop	rbx
1408d77fc: 5f                          	pop	rdi
1408d77fd: 5e                          	pop	rsi
1408d77fe: 41 5c                       	pop	r12
1408d7800: 41 5d                       	pop	r13
1408d7802: 41 5e                       	pop	r14
1408d7804: 41 5f                       	pop	r15
1408d7806: 5d                          	pop	rbp
1408d7807: c3                          	ret
1408d7808: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1408d7810: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d7815: 55                          	push	rbp
1408d7816: 41 57                       	push	r15
1408d7818: 41 56                       	push	r14
1408d781a: 41 55                       	push	r13
1408d781c: 41 54                       	push	r12
1408d781e: 56                          	push	rsi
1408d781f: 57                          	push	rdi
1408d7820: 53                          	push	rbx
1408d7821: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d7828: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d782f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d7835: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d783b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d7841: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d7847: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d784d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d7857: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d7860: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d7869: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d7871: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d787a: 48 8b 85 a0 01 00 00        	mov	rax, qword ptr [rbp + 0x1a0]
1408d7881: 48 3b 85 40 03 00 00        	cmp	rax, qword ptr [rbp + 0x340]
1408d7888: 75 5d                       	jne	0x1408d78e7 <.text+0x8d68e7>
1408d788a: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7892: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d789a: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d78a3: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d78ac: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d78b5: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d78bb: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d78c1: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d78c7: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d78cd: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d78d3: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d78da: 5b                          	pop	rbx
1408d78db: 5f                          	pop	rdi
1408d78dc: 5e                          	pop	rsi
1408d78dd: 41 5c                       	pop	r12
1408d78df: 41 5d                       	pop	r13
1408d78e1: 41 5e                       	pop	r14
1408d78e3: 41 5f                       	pop	r15
1408d78e5: 5d                          	pop	rbp
1408d78e6: c3                          	ret
1408d78e7: 48 8b 85 40 02 00 00        	mov	rax, qword ptr [rbp + 0x240]
1408d78ee: 48 85 c0                    	test	rax, rax
1408d78f1: 74 1a                       	je	0x1408d790d <.text+0x8d690d>
1408d78f3: 48 8b 8d 48 02 00 00        	mov	rcx, qword ptr [rbp + 0x248]
1408d78fa: 48 c1 e0 02                 	shl	rax, 0x2
1408d78fe: 48 8d 14 c0                 	lea	rdx, [rax + 8*rax]
1408d7902: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d7908: e8 13 c3 d3 ff              	call	0x140613c20 <.text+0x612c20>
1408d790d: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7915: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d791d: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7926: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d792f: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7938: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d793e: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7944: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d794a: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7950: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7956: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d795d: 5b                          	pop	rbx
1408d795e: 5f                          	pop	rdi
1408d795f: 5e                          	pop	rsi
1408d7960: 41 5c                       	pop	r12
1408d7962: 41 5d                       	pop	r13
1408d7964: 41 5e                       	pop	r14
1408d7966: 41 5f                       	pop	r15
1408d7968: 5d                          	pop	rbp
1408d7969: c3                          	ret
1408d796a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1408d7970: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d7975: 55                          	push	rbp
1408d7976: 41 57                       	push	r15
1408d7978: 41 56                       	push	r14
1408d797a: 41 55                       	push	r13
1408d797c: 41 54                       	push	r12
1408d797e: 56                          	push	rsi
1408d797f: 57                          	push	rdi
1408d7980: 53                          	push	rbx
1408d7981: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d7988: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d798f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d7995: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d799b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d79a1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d79a7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d79ad: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
1408d79b6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d79bf: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d79c8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d79d0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1408d79d8: 48 8b 85 a0 01 00 00        	mov	rax, qword ptr [rbp + 0x1a0]
1408d79df: 48 3b 85 40 03 00 00        	cmp	rax, qword ptr [rbp + 0x340]
1408d79e6: 75 5d                       	jne	0x1408d7a45 <.text+0x8d6a45>
1408d79e8: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d79f0: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d79f8: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7a01: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d7a0a: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7a13: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d7a19: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7a1f: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7a25: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7a2b: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7a31: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d7a38: 5b                          	pop	rbx
1408d7a39: 5f                          	pop	rdi
1408d7a3a: 5e                          	pop	rsi
1408d7a3b: 41 5c                       	pop	r12
1408d7a3d: 41 5d                       	pop	r13
1408d7a3f: 41 5e                       	pop	r14
1408d7a41: 41 5f                       	pop	r15
1408d7a43: 5d                          	pop	rbp
1408d7a44: c3                          	ret
1408d7a45: 48 8b 85 78 01 00 00        	mov	rax, qword ptr [rbp + 0x178]
1408d7a4c: 48 85 c0                    	test	rax, rax
1408d7a4f: 74 1a                       	je	0x1408d7a6b <.text+0x8d6a6b>
1408d7a51: 48 8b 8d 80 01 00 00        	mov	rcx, qword ptr [rbp + 0x180]
1408d7a58: 48 c1 e0 02                 	shl	rax, 0x2
1408d7a5c: 48 8d 14 c0                 	lea	rdx, [rax + 8*rax]
1408d7a60: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408d7a66: e8 b5 c1 d3 ff              	call	0x140613c20 <.text+0x612c20>
1408d7a6b: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7a73: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d7a7b: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7a84: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d7a8d: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7a96: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d7a9c: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7aa2: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7aa8: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7aae: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7ab4: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d7abb: 5b                          	pop	rbx
1408d7abc: 5f                          	pop	rdi
1408d7abd: 5e                          	pop	rsi
1408d7abe: 41 5c                       	pop	r12
1408d7ac0: 41 5d                       	pop	r13
1408d7ac2: 41 5e                       	pop	r14
1408d7ac4: 41 5f                       	pop	r15
1408d7ac6: 5d                          	pop	rbp
1408d7ac7: c3                          	ret
1408d7ac8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1408d7ad0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d7ad5: 55                          	push	rbp
1408d7ad6: 41 57                       	push	r15
1408d7ad8: 41 56                       	push	r14
1408d7ada: 41 55                       	push	r13
1408d7adc: 41 54                       	push	r12
1408d7ade: 56                          	push	rsi
1408d7adf: 57                          	push	rdi
1408d7ae0: 53                          	push	rbx
1408d7ae1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d7ae8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d7aef: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d7af5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d7afb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d7b01: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d7b07: 66 44 0f 7f 5c 24 70        	movdqa	xmmword ptr [rsp + 0x70], xmm11
1408d7b0e: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d7b18: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d7b21: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d7b2a: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d7b32: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1408d7b3a: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d7b41: 48 ff 00                    	inc	qword ptr [rax]
1408d7b44: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7b4c: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d7b54: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7b5d: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d7b66: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7b6f: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d7b75: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7b7b: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7b81: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7b87: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7b8d: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d7b94: 5b                          	pop	rbx
1408d7b95: 5f                          	pop	rdi
1408d7b96: 5e                          	pop	rsi
1408d7b97: 41 5c                       	pop	r12
1408d7b99: 41 5d                       	pop	r13
1408d7b9b: 41 5e                       	pop	r14
1408d7b9d: 41 5f                       	pop	r15
1408d7b9f: 5d                          	pop	rbp
1408d7ba0: c3                          	ret
1408d7ba1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1408d7bb0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d7bb5: 55                          	push	rbp
1408d7bb6: 41 57                       	push	r15
1408d7bb8: 41 56                       	push	r14
1408d7bba: 41 55                       	push	r13
1408d7bbc: 41 54                       	push	r12
1408d7bbe: 56                          	push	rsi
1408d7bbf: 57                          	push	rdi
1408d7bc0: 53                          	push	rbx
1408d7bc1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d7bc8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d7bcf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d7bd5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d7bdb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d7be1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d7be7: 66 44 0f 7f 5c 24 70        	movdqa	xmmword ptr [rsp + 0x70], xmm11
1408d7bee: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
1408d7bf7: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d7c00: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d7c09: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d7c11: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1408d7c19: 80 bd 4f 03 00 00 00        	cmp	byte ptr [rbp + 0x34f], 0x0
1408d7c20: 74 76                       	je	0x1408d7c98 <.text+0x8d6c98>
1408d7c22: 48 8b 85 f0 02 00 00        	mov	rax, qword ptr [rbp + 0x2f0]
1408d7c29: f0                          	lock
1408d7c2a: 48 ff 08                    	dec	qword ptr [rax]
1408d7c2d: 75 0c                       	jne	0x1408d7c3b <.text+0x8d6c3b>
1408d7c2f: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1408d7c36: e8 e5 49 28 00              	call	0x140b5c620 <.text+0xb5b620>
1408d7c3b: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7c43: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d7c4b: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7c54: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d7c5d: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7c66: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d7c6c: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7c72: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7c78: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7c7e: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7c84: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d7c8b: 5b                          	pop	rbx
1408d7c8c: 5f                          	pop	rdi
1408d7c8d: 5e                          	pop	rsi
1408d7c8e: 41 5c                       	pop	r12
1408d7c90: 41 5d                       	pop	r13
1408d7c92: 41 5e                       	pop	r14
1408d7c94: 41 5f                       	pop	r15
1408d7c96: 5d                          	pop	rbp
1408d7c97: c3                          	ret
1408d7c98: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7ca0: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d7ca8: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7cb1: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d7cba: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7cc3: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d7cc9: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7ccf: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7cd5: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7cdb: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7ce1: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d7ce8: 5b                          	pop	rbx
1408d7ce9: 5f                          	pop	rdi
1408d7cea: 5e                          	pop	rsi
1408d7ceb: 41 5c                       	pop	r12
1408d7ced: 41 5d                       	pop	r13
1408d7cef: 41 5e                       	pop	r14
1408d7cf1: 41 5f                       	pop	r15
1408d7cf3: 5d                          	pop	rbp
1408d7cf4: c3                          	ret
1408d7cf5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1408d7d00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d7d05: 55                          	push	rbp
1408d7d06: 41 57                       	push	r15
1408d7d08: 41 56                       	push	r14
1408d7d0a: 41 55                       	push	r13
1408d7d0c: 41 54                       	push	r12
1408d7d0e: 56                          	push	rsi
1408d7d0f: 57                          	push	rdi
1408d7d10: 53                          	push	rbx
1408d7d11: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d7d18: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d7d1f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d7d25: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d7d2b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d7d31: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d7d37: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1408d7d3d: 66 44 0f 7f 94 24 80 00 00 00       	movdqa	xmmword ptr [rsp + 0x80], xmm10
1408d7d47: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d7d50: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d7d59: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d7d61: 66 0f 7f b4 24 c0 00 00 00  	movdqa	xmmword ptr [rsp + 0xc0], xmm6
1408d7d6a: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
1408d7d71: 48 ff 00                    	inc	qword ptr [rax]
1408d7d74: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7d7c: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d7d84: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7d8d: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d7d96: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7d9f: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d7da5: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7dab: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7db1: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7db7: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7dbd: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d7dc4: 5b                          	pop	rbx
1408d7dc5: 5f                          	pop	rdi
1408d7dc6: 5e                          	pop	rsi
1408d7dc7: 41 5c                       	pop	r12
1408d7dc9: 41 5d                       	pop	r13
1408d7dcb: 41 5e                       	pop	r14
1408d7dcd: 41 5f                       	pop	r15
1408d7dcf: 5d                          	pop	rbp
1408d7dd0: c3                          	ret
1408d7dd1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1408d7de0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408d7de5: 55                          	push	rbp
1408d7de6: 41 57                       	push	r15
1408d7de8: 41 56                       	push	r14
1408d7dea: 41 55                       	push	r13
1408d7dec: 41 54                       	push	r12
1408d7dee: 56                          	push	rsi
1408d7def: 57                          	push	rdi
1408d7df0: 53                          	push	rbx
1408d7df1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1408d7df8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408d7dff: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1408d7e05: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
1408d7e0b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1408d7e11: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1408d7e17: 66 44 0f 7f 5c 24 70        	movdqa	xmmword ptr [rsp + 0x70], xmm11
1408d7e1e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
1408d7e27: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1408d7e30: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1408d7e39: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1408d7e41: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1408d7e49: 48 83 7d 30 00              	cmp	qword ptr [rbp + 0x30], 0x0
1408d7e4e: 74 20                       	je	0x1408d7e70 <.text+0x8d6e70>
1408d7e50: 48 8d 45 38                 	lea	rax, [rbp + 0x38]
1408d7e54: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
1408d7e58: 48 8d 45 20                 	lea	rax, [rbp + 0x20]
1408d7e5c: 48 89 45 18                 	mov	qword ptr [rbp + 0x18], rax
1408d7e60: 48 8d 0d 79 b8 1c 02        	lea	rcx, [rip + 0x21cb879]  # 0x142aa36e0
1408d7e67: 48 8d 55 18                 	lea	rdx, [rbp + 0x18]
1408d7e6b: e8 60 74 a5 00              	call	0x14132f2d0 <.text+0x132e2d0>
1408d7e70: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1408d7e78: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1408d7e80: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1408d7e89: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1408d7e92: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1408d7e9b: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1408d7ea1: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1408d7ea7: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1408d7ead: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1408d7eb3: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1408d7eb9: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1408d7ec0: 5b                          	pop	rbx
1408d7ec1: 5f                          	pop	rdi
1408d7ec2: 5e                          	pop	rsi
1408d7ec3: 41 5c                       	pop	r12
1408d7ec5: 41 5d                       	pop	r13
1408d7ec7: 41 5e                       	pop	r14
1408d7ec9: 41 5f                       	pop	r15
1408d7ecb: 5d                          	pop	rbp
1408d7ecc: c3                          	ret
