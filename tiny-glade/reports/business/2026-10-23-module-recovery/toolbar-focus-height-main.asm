
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a623e0: 55                          	pushq	%rbp
140a623e1: 41 57                       	pushq	%r15
140a623e3: 41 56                       	pushq	%r14
140a623e5: 41 55                       	pushq	%r13
140a623e7: 41 54                       	pushq	%r12
140a623e9: 56                          	pushq	%rsi
140a623ea: 57                          	pushq	%rdi
140a623eb: 53                          	pushq	%rbx
140a623ec: 48 81 ec a8 02 00 00        	subq	$0x2a8, %rsp            # imm = 0x2A8
140a623f3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a623fb: 44 0f 29 bd 10 02 00 00     	movaps	%xmm15, 0x210(%rbp)
140a62403: 44 0f 29 b5 00 02 00 00     	movaps	%xmm14, 0x200(%rbp)
140a6240b: 44 0f 29 ad f0 01 00 00     	movaps	%xmm13, 0x1f0(%rbp)
140a62413: 44 0f 29 a5 e0 01 00 00     	movaps	%xmm12, 0x1e0(%rbp)
140a6241b: 44 0f 29 9d d0 01 00 00     	movaps	%xmm11, 0x1d0(%rbp)
140a62423: 44 0f 29 95 c0 01 00 00     	movaps	%xmm10, 0x1c0(%rbp)
140a6242b: 44 0f 29 8d b0 01 00 00     	movaps	%xmm9, 0x1b0(%rbp)
140a62433: 44 0f 29 85 a0 01 00 00     	movaps	%xmm8, 0x1a0(%rbp)
140a6243b: 0f 29 bd 90 01 00 00        	movaps	%xmm7, 0x190(%rbp)
140a62442: 0f 29 b5 80 01 00 00        	movaps	%xmm6, 0x180(%rbp)
140a62449: 48 c7 85 78 01 00 00 fe ff ff ff    	movq	$-0x2, 0x178(%rbp)
140a62454: 4c 8b ad c8 02 00 00        	movq	0x2c8(%rbp), %r13
140a6245b: 48 8b 01                    	movq	(%rcx), %rax
140a6245e: 8b 08                       	movl	(%rax), %ecx
140a62460: 83 f9 05                    	cmpl	$0x5, %ecx
140a62463: 0f 87 dc 00 00 00           	ja	0x140a62545 <.text+0xa61545>
140a62469: 4c 89 ce                    	movq	%r9, %rsi
140a6246c: 4c 89 c7                    	movq	%r8, %rdi
140a6246f: 83 f9 01                    	cmpl	$0x1, %ecx
140a62472: 74 09                       	je	0x140a6247d <.text+0xa6147d>
140a62474: 83 f9 02                    	cmpl	$0x2, %ecx
140a62477: 0f 85 c8 00 00 00           	jne	0x140a62545 <.text+0xa61545>
140a6247d: 4c 8b 70 08                 	movq	0x8(%rax), %r14
140a62481: 4c 89 b5 60 01 00 00        	movq	%r14, 0x160(%rbp)
140a62488: 4c 8b 22                    	movq	(%rdx), %r12
140a6248b: 4c 89 75 70                 	movq	%r14, 0x70(%rbp)
140a6248f: 49 83 7c 24 18 00           	cmpq	$0x0, 0x18(%r12)
140a62495: 0f 84 f8 01 00 00           	je	0x140a62693 <.text+0xa61693>
140a6249b: 49 8d 5c 24 20              	leaq	0x20(%r12), %rbx
140a624a0: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a624a4: 48 89 d9                    	movq	%rbx, %rcx
140a624a7: e8 c4 e8 b4 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140a624ac: 49 8b 0c 24                 	movq	(%r12), %rcx
140a624b0: 49 8b 54 24 08              	movq	0x8(%r12), %rdx
140a624b5: 49 89 d0                    	movq	%rdx, %r8
140a624b8: 49 21 c0                    	andq	%rax, %r8
140a624bb: 48 c1 e8 39                 	shrq	$0x39, %rax
140a624bf: 66 0f 6e c0                 	movd	%eax, %xmm0
140a624c3: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a624c7: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a624cc: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a624d1: 48 8d 81 58 fd ff ff        	leaq	-0x2a8(%rcx), %rax
140a624d8: 45 31 c9                    	xorl	%r9d, %r9d
140a624db: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140a624df: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
140a624e5: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140a624e9: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140a624ed: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
140a624f2: 45 85 d2                    	testl	%r10d, %r10d
140a624f5: 74 2c                       	je	0x140a62523 <.text+0xa61523>
140a624f7: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140a624fc: 4d 01 c3                    	addq	%r8, %r11
140a624ff: 49 21 d3                    	andq	%rdx, %r11
140a62502: 49 f7 db                    	negq	%r11
140a62505: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
140a6250c: 4e 39 34 18                 	cmpq	%r14, (%rax,%r11)
140a62510: 0f 84 99 00 00 00           	je	0x140a625af <.text+0xa615af>
140a62516: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140a6251a: 66 45 21 d3                 	andw	%r10w, %r11w
140a6251e: 45 89 da                    	movl	%r11d, %r10d
140a62521: 75 d4                       	jne	0x140a624f7 <.text+0xa614f7>
140a62523: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140a62527: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140a6252c: 45 85 d2                    	testl	%r10d, %r10d
140a6252f: 0f 85 5e 01 00 00           	jne	0x140a62693 <.text+0xa61693>
140a62535: 4d 01 c8                    	addq	%r9, %r8
140a62538: 49 83 c0 10                 	addq	$0x10, %r8
140a6253c: 49 83 c1 10                 	addq	$0x10, %r9
140a62540: 49 21 d0                    	andq	%rdx, %r8
140a62543: eb 9a                       	jmp	0x140a624df <.text+0xa614df>
140a62545: 41 c7 45 00 00 00 00 00     	movl	$0x0, (%r13)
140a6254d: 0f 28 b5 80 01 00 00        	movaps	0x180(%rbp), %xmm6
140a62554: 0f 28 bd 90 01 00 00        	movaps	0x190(%rbp), %xmm7
140a6255b: 44 0f 28 85 a0 01 00 00     	movaps	0x1a0(%rbp), %xmm8
140a62563: 44 0f 28 8d b0 01 00 00     	movaps	0x1b0(%rbp), %xmm9
140a6256b: 44 0f 28 95 c0 01 00 00     	movaps	0x1c0(%rbp), %xmm10
140a62573: 44 0f 28 9d d0 01 00 00     	movaps	0x1d0(%rbp), %xmm11
140a6257b: 44 0f 28 a5 e0 01 00 00     	movaps	0x1e0(%rbp), %xmm12
140a62583: 44 0f 28 ad f0 01 00 00     	movaps	0x1f0(%rbp), %xmm13
140a6258b: 44 0f 28 b5 00 02 00 00     	movaps	0x200(%rbp), %xmm14
140a62593: 44 0f 28 bd 10 02 00 00     	movaps	0x210(%rbp), %xmm15
140a6259b: 48 81 c4 a8 02 00 00        	addq	$0x2a8, %rsp            # imm = 0x2A8
140a625a2: 5b                          	popq	%rbx
140a625a3: 5f                          	popq	%rdi
140a625a4: 5e                          	popq	%rsi
140a625a5: 41 5c                       	popq	%r12
140a625a7: 41 5d                       	popq	%r13
140a625a9: 41 5e                       	popq	%r14
140a625ab: 41 5f                       	popq	%r15
140a625ad: 5d                          	popq	%rbp
140a625ae: c3                          	retq
140a625af: 4a 83 bc 19 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rcx,%r11)
140a625b8: 0f 82 e4 15 00 00           	jb	0x140a63ba2 <.text+0xa62ba2>
140a625be: f3 42 0f 10 84 19 90 fd ff ff       	movss	-0x270(%rcx,%r11), %xmm0
140a625c8: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140a625cc: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140a625cf: 0f 86 cd 15 00 00           	jbe	0x140a63ba2 <.text+0xa62ba2>
140a625d5: 4c 01 d9                    	addq	%r11, %rcx
140a625d8: 83 79 b8 01                 	cmpl	$0x1, -0x48(%rcx)
140a625dc: 0f 87 6b ff ff ff           	ja	0x140a6254d <.text+0xa6154d>
140a625e2: 4c 89 75 70                 	movq	%r14, 0x70(%rbp)
140a625e6: 49 83 7c 24 18 00           	cmpq	$0x0, 0x18(%r12)
140a625ec: 0f 84 cc 03 00 00           	je	0x140a629be <.text+0xa619be>
140a625f2: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a625f6: 48 89 d9                    	movq	%rbx, %rcx
140a625f9: e8 72 e7 b4 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140a625fe: 4d 8b 3c 24                 	movq	(%r12), %r15
140a62602: 49 8b 4c 24 08              	movq	0x8(%r12), %rcx
140a62607: 48 89 ca                    	movq	%rcx, %rdx
140a6260a: 48 21 c2                    	andq	%rax, %rdx
140a6260d: 48 c1 e8 39                 	shrq	$0x39, %rax
140a62611: 66 0f 6e c0                 	movd	%eax, %xmm0
140a62615: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a62619: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a6261e: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a62623: 49 8d 87 58 fd ff ff        	leaq	-0x2a8(%r15), %rax
140a6262a: 45 31 c0                    	xorl	%r8d, %r8d
140a6262d: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140a62631: f3 41 0f 6f 14 17           	movdqu	(%r15,%rdx), %xmm2
140a62637: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140a6263b: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140a6263f: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
140a62644: 45 85 c9                    	testl	%r9d, %r9d
140a62647: 74 28                       	je	0x140a62671 <.text+0xa61671>
140a62649: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
140a6264e: 49 01 d2                    	addq	%rdx, %r10
140a62651: 49 21 ca                    	andq	%rcx, %r10
140a62654: 49 f7 da                    	negq	%r10
140a62657: 4d 69 d2 a8 02 00 00        	imulq	$0x2a8, %r10, %r10      # imm = 0x2A8
140a6265e: 4e 39 34 10                 	cmpq	%r14, (%rax,%r10)
140a62662: 74 3b                       	je	0x140a6269f <.text+0xa6169f>
140a62664: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
140a62668: 66 45 21 ca                 	andw	%r9w, %r10w
140a6266c: 45 89 d1                    	movl	%r10d, %r9d
140a6266f: 75 d8                       	jne	0x140a62649 <.text+0xa61649>
140a62671: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140a62675: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140a6267a: 45 85 c9                    	testl	%r9d, %r9d
140a6267d: 0f 85 3b 03 00 00           	jne	0x140a629be <.text+0xa619be>
140a62683: 4c 01 c2                    	addq	%r8, %rdx
140a62686: 48 83 c2 10                 	addq	$0x10, %rdx
140a6268a: 49 83 c0 10                 	addq	$0x10, %r8
140a6268e: 48 21 ca                    	andq	%rcx, %rdx
140a62691: eb 9e                       	jmp	0x140a62631 <.text+0xa61631>
140a62693: 48 8d 0d 06 1b 07 02        	leaq	0x2071b06(%rip), %rcx   # 0x142ad41a0
140a6269a: e8 71 6c e7 01              	callq	0x1428d9310 <.text+0x28d8310>
140a6269f: 4b 83 bc 17 70 fd ff ff 02  	cmpq	$0x2, -0x290(%r15,%r10)
140a626a8: 0f 82 0c 15 00 00           	jb	0x140a63bba <.text+0xa62bba>
140a626ae: f3 43 0f 10 84 17 90 fd ff ff       	movss	-0x270(%r15,%r10), %xmm0
140a626b8: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140a626bc: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140a626bf: 0f 86 f5 14 00 00           	jbe	0x140a63bba <.text+0xa62bba>
140a626c5: 4d 01 d7                    	addq	%r10, %r15
140a626c8: 49 8b 8f a8 fd ff ff        	movq	-0x258(%r15), %rcx
140a626cf: 48 85 c9                    	testq	%rcx, %rcx
140a626d2: 0f 84 32 15 00 00           	je	0x140a63c0a <.text+0xa62c0a>
140a626d8: 49 8b 87 a0 fd ff ff        	movq	-0x260(%r15), %rax
140a626df: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140a626e3: 48 8d 0c 88                 	leaq	(%rax,%rcx,4), %rcx
140a626e7: 48 8b 36                    	movq	(%rsi), %rsi
140a626ea: f3 44 0f 10 00              	movss	(%rax), %xmm8
140a626ef: 48 8b 17                    	movq	(%rdi), %rdx
140a626f2: 8b 12                       	movl	(%rdx), %edx
140a626f4: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a626f7: f3 48 0f 2a c2              	cvtsi2ss	%rdx, %xmm0
140a626fc: f3 0f 11 85 58 01 00 00     	movss	%xmm0, 0x158(%rbp)
140a62704: f3 44 0f 10 70 08           	movss	0x8(%rax), %xmm14
140a6270a: 0f 28 06                    	movaps	(%rsi), %xmm0
140a6270d: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
140a62711: 0f 28 46 10                 	movaps	0x10(%rsi), %xmm0
140a62715: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
140a62719: 0f 28 5e 20                 	movaps	0x20(%rsi), %xmm3
140a6271d: 0f 28 66 30                 	movaps	0x30(%rsi), %xmm4
140a62721: 0f 28 ae 80 00 00 00        	movaps	0x80(%rsi), %xmm5
140a62728: 0f 28 b6 90 00 00 00        	movaps	0x90(%rsi), %xmm6
140a6272f: 0f 28 be a0 00 00 00        	movaps	0xa0(%rsi), %xmm7
140a62736: 44 0f 28 8e b0 00 00 00     	movaps	0xb0(%rsi), %xmm9
140a6273e: f3 44 0f 10 15 c9 ec 01 02  	movss	0x201ecc9(%rip), %xmm10 # 0x142a81410
140a62747: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
140a6274b: f3 0f 10 05 1d 31 ec 01     	movss	0x1ec311d(%rip), %xmm0  # 0x142925870
140a62753: f3 0f 11 85 50 01 00 00     	movss	%xmm0, 0x150(%rbp)
140a6275b: eb 54                       	jmp	0x140a627b1 <.text+0xa617b1>
140a6275d: 0f 1f 00                    	nopl	(%rax)
140a62760: 41 0f 28 d5                 	movaps	%xmm13, %xmm2
140a62764: f3 41 0f c2 d4 01           	cmpltss	%xmm12, %xmm2
140a6276a: 0f 54 c2                    	andps	%xmm2, %xmm0
140a6276d: 41 0f 55 d0                 	andnps	%xmm8, %xmm2
140a62771: 0f 56 d0                    	orps	%xmm0, %xmm2
140a62774: 44 0f 28 c2                 	movaps	%xmm2, %xmm8
140a62778: f3 45 0f 5f e5              	maxss	%xmm13, %xmm12
140a6277d: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
140a62781: f3 41 0f c2 c2 03           	cmpunordss	%xmm10, %xmm0
140a62787: 0f 28 d0                    	movaps	%xmm0, %xmm2
140a6278a: 0f 54 d1                    	andps	%xmm1, %xmm2
140a6278d: f3 41 0f 5d ca              	minss	%xmm10, %xmm1
140a62792: 0f 55 c1                    	andnps	%xmm1, %xmm0
140a62795: 0f 56 c2                    	orps	%xmm2, %xmm0
140a62798: 48 83 c0 0c                 	addq	$0xc, %rax
140a6279c: 45 0f 28 ec                 	movaps	%xmm12, %xmm13
140a627a0: 44 0f 28 d0                 	movaps	%xmm0, %xmm10
140a627a4: 45 0f 28 f3                 	movaps	%xmm11, %xmm14
140a627a8: 48 39 c8                    	cmpq	%rcx, %rax
140a627ab: 0f 84 ae 00 00 00           	je	0x140a6285f <.text+0xa6185f>
140a627b1: f3 0f 10 48 04              	movss	0x4(%rax), %xmm1
140a627b6: f3 44 0f 10 58 08           	movss	0x8(%rax), %xmm11
140a627bc: f3 0f 10 00                 	movss	(%rax), %xmm0
140a627c0: 44 0f 28 e0                 	movaps	%xmm0, %xmm12
140a627c4: 44 0f c6 e0 00              	shufps	$0x0, %xmm0, %xmm12     # xmm12 = xmm12[0,0],xmm0[0,0]
140a627c9: 44 0f 59 e5                 	mulps	%xmm5, %xmm12
140a627cd: 44 0f 28 f9                 	movaps	%xmm1, %xmm15
140a627d1: 44 0f c6 f9 00              	shufps	$0x0, %xmm1, %xmm15     # xmm15 = xmm15[0,0],xmm1[0,0]
140a627d6: 44 0f 59 fe                 	mulps	%xmm6, %xmm15
140a627da: 45 0f 58 fc                 	addps	%xmm12, %xmm15
140a627de: 45 0f 28 e3                 	movaps	%xmm11, %xmm12
140a627e2: 45 0f c6 e3 00              	shufps	$0x0, %xmm11, %xmm12    # xmm12 = xmm12[0,0],xmm11[0,0]
140a627e7: 44 0f 59 e7                 	mulps	%xmm7, %xmm12
140a627eb: 45 0f 58 e7                 	addps	%xmm15, %xmm12
140a627ef: 45 0f 58 e1                 	addps	%xmm9, %xmm12
140a627f3: 45 0f 28 fc                 	movaps	%xmm12, %xmm15
140a627f7: 45 0f c6 fc 00              	shufps	$0x0, %xmm12, %xmm15    # xmm15 = xmm15[0,0],xmm12[0,0]
140a627fc: 44 0f 59 7d 20              	mulps	0x20(%rbp), %xmm15
140a62801: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
140a62805: 41 0f c6 d4 55              	shufps	$0x55, %xmm12, %xmm2    # xmm2 = xmm2[1,1],xmm12[1,1]
140a6280a: 0f 59 55 60                 	mulps	0x60(%rbp), %xmm2
140a6280e: 41 0f 58 d7                 	addps	%xmm15, %xmm2
140a62812: 45 0f c6 e4 aa              	shufps	$0xaa, %xmm12, %xmm12   # xmm12 = xmm12[2,2,2,2]
140a62817: 44 0f 59 e3                 	mulps	%xmm3, %xmm12
140a6281b: 44 0f 58 e2                 	addps	%xmm2, %xmm12
140a6281f: 44 0f 58 e4                 	addps	%xmm4, %xmm12
140a62823: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
140a62827: 41 0f c6 d4 ff              	shufps	$0xff, %xmm12, %xmm2    # xmm2 = xmm2[3,3],xmm12[3,3]
140a6282c: f3 44 0f 5e e2              	divss	%xmm2, %xmm12
140a62831: f3 0f 10 15 37 30 ec 01     	movss	0x1ec3037(%rip), %xmm2  # 0x142925870
140a62839: f3 44 0f 59 e2              	mulss	%xmm2, %xmm12
140a6283e: f3 44 0f 58 e2              	addss	%xmm2, %xmm12
140a62843: f3 44 0f 59 a5 58 01 00 00  	mulss	0x158(%rbp), %xmm12
140a6284c: 45 0f 2e e5                 	ucomiss	%xmm13, %xmm12
140a62850: 0f 87 0a ff ff ff           	ja	0x140a62760 <.text+0xa61760>
140a62856: 45 0f 28 de                 	movaps	%xmm14, %xmm11
140a6285a: e9 01 ff ff ff              	jmp	0x140a62760 <.text+0xa61760>
140a6285f: 41 f6 87 68 fe ff ff 01     	testb	$0x1, -0x198(%r15)
140a62867: 0f 84 65 13 00 00           	je	0x140a63bd2 <.text+0xa62bd2>
140a6286d: f3 41 0f 10 8f 6c fe ff ff  	movss	-0x194(%r15), %xmm1
140a62876: f3 0f 58 0d a2 09 05 02     	addss	0x20509a2(%rip), %xmm1  # 0x142ab3220
140a6287e: 0f 28 f8                    	movaps	%xmm0, %xmm7
140a62881: f3 0f c2 f8 03              	cmpunordss	%xmm0, %xmm7
140a62886: 0f 28 d7                    	movaps	%xmm7, %xmm2
140a62889: 0f 54 d1                    	andps	%xmm1, %xmm2
140a6288c: f3 0f 5d c8                 	minss	%xmm0, %xmm1
140a62890: 0f 55 f9                    	andnps	%xmm1, %xmm7
140a62893: 0f 56 fa                    	orps	%xmm2, %xmm7
140a62896: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a6289a: 48 89 f2                    	movq	%rsi, %rdx
140a6289d: e8 4e 45 23 00              	callq	0x140c96df0 <.text+0xc95df0>
140a628a2: f3 0f 10 45 70              	movss	0x70(%rbp), %xmm0
140a628a7: f3 0f 10 4d 78              	movss	0x78(%rbp), %xmm1
140a628ac: f3 0f 11 85 30 01 00 00     	movss	%xmm0, 0x130(%rbp)
140a628b4: f3 0f 11 8d 34 01 00 00     	movss	%xmm1, 0x134(%rbp)
140a628bc: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a628c0: 48 8d 95 30 01 00 00        	leaq	0x130(%rbp), %rdx
140a628c7: 0f 57 d2                    	xorps	%xmm2, %xmm2
140a628ca: e8 11 a5 23 00              	callq	0x140c9cde0 <.text+0xc9bde0>
140a628cf: f3 0f 10 45 78              	movss	0x78(%rbp), %xmm0
140a628d4: 0f 28 c8                    	movaps	%xmm0, %xmm1
140a628d7: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
140a628db: f2 0f 10 55 70              	movsd	0x70(%rbp), %xmm2
140a628e0: 0f 28 da                    	movaps	%xmm2, %xmm3
140a628e3: 0f 59 da                    	mulps	%xmm2, %xmm3
140a628e6: f3 0f 16 e3                 	movshdup	%xmm3, %xmm4            # xmm4 = xmm3[1,1,3,3]
140a628ea: f3 0f 58 e3                 	addss	%xmm3, %xmm4
140a628ee: f3 0f 58 e1                 	addss	%xmm1, %xmm4
140a628f2: 0f 57 c9                    	xorps	%xmm1, %xmm1
140a628f5: f3 0f 51 cc                 	sqrtss	%xmm4, %xmm1
140a628f9: f3 0f 10 35 03 30 ec 01     	movss	0x1ec3003(%rip), %xmm6  # 0x142925904
140a62901: 44 0f 28 d6                 	movaps	%xmm6, %xmm10
140a62905: f3 44 0f 5e d1              	divss	%xmm1, %xmm10
140a6290a: f3 45 0f 12 ca              	movsldup	%xmm10, %xmm9           # xmm9 = xmm10[0,0,2,2]
140a6290f: 44 0f 59 ca                 	mulps	%xmm2, %xmm9
140a62913: f3 44 0f 59 d0              	mulss	%xmm0, %xmm10
140a62918: 44 0f 59 0d 30 0d 07 02     	mulps	0x2070d30(%rip), %xmm9  # 0x142ad3650
140a62920: f3 44 0f 59 15 a7 2f ec 01  	mulss	0x1ec2fa7(%rip), %xmm10 # 0x1429258d0
140a62929: f3 44 0f 11 85 30 01 00 00  	movss	%xmm8, 0x130(%rbp)
140a62932: f3 44 0f 11 9d 34 01 00 00  	movss	%xmm11, 0x134(%rbp)
140a6293b: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a6293f: 48 8d 95 30 01 00 00        	leaq	0x130(%rbp), %rdx
140a62946: 0f 28 d7                    	movaps	%xmm7, %xmm2
140a62949: e8 92 a4 23 00              	callq	0x140c9cde0 <.text+0xc9bde0>
140a6294e: f3 0f 10 45 78              	movss	0x78(%rbp), %xmm0
140a62953: f3 41 0f 58 c2              	addss	%xmm10, %xmm0
140a62958: f2 0f 10 4d 70              	movsd	0x70(%rbp), %xmm1
140a6295d: 41 0f 58 c9                 	addps	%xmm9, %xmm1
140a62961: 0f 13 4d 08                 	movlps	%xmm1, 0x8(%rbp)
140a62965: f3 0f 11 45 10              	movss	%xmm0, 0x10(%rbp)
140a6296a: 41 83 7d 00 00              	cmpl	$0x0, (%r13)
140a6296f: 74 59                       	je	0x140a629ca <.text+0xa619ca>
140a62971: 41 8b 45 0c                 	movl	0xc(%r13), %eax
140a62975: 89 45 78                    	movl	%eax, 0x78(%rbp)
140a62978: 49 8b 45 04                 	movq	0x4(%r13), %rax
140a6297c: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a62980: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
140a62987: 48 8b 00                    	movq	(%rax), %rax
140a6298a: f3 0f 10 58 28              	movss	0x28(%rax), %xmm3
140a6298f: c7 44 24 20 00 00 a0 41     	movl	$0x41a00000, 0x20(%rsp) # imm = 0x41A00000
140a62997: 48 8d 8d 30 01 00 00        	leaq	0x130(%rbp), %rcx
140a6299e: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a629a2: 4c 8d 45 08                 	leaq	0x8(%rbp), %r8
140a629a6: e8 55 dc 22 00              	callq	0x140c90600 <.text+0xc8f600>
140a629ab: 8b 85 38 01 00 00           	movl	0x138(%rbp), %eax
140a629b1: 41 89 45 0c                 	movl	%eax, 0xc(%r13)
140a629b5: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
140a629bc: eb 17                       	jmp	0x140a629d5 <.text+0xa619d5>
140a629be: 48 8d 0d 0b 18 07 02        	leaq	0x207180b(%rip), %rcx   # 0x142ad41d0
140a629c5: e8 46 69 e7 01              	callq	0x1428d9310 <.text+0x28d8310>
140a629ca: 8b 45 10                    	movl	0x10(%rbp), %eax
140a629cd: 41 89 45 0c                 	movl	%eax, 0xc(%r13)
140a629d1: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140a629d5: 49 89 45 04                 	movq	%rax, 0x4(%r13)
140a629d9: 41 c7 45 00 01 00 00 00     	movl	$0x1, (%r13)
140a629e1: f3 41 0f 10 7d 04           	movss	0x4(%r13), %xmm7
140a629e7: 41 8b 7d 08                 	movl	0x8(%r13), %edi
140a629eb: f3 45 0f 10 45 0c           	movss	0xc(%r13), %xmm8
140a629f1: f3 0f 11 bd 30 01 00 00     	movss	%xmm7, 0x130(%rbp)
140a629f9: f3 44 0f 11 85 34 01 00 00  	movss	%xmm8, 0x134(%rbp)
140a62a02: 41 f6 87 68 fe ff ff 01     	testb	$0x1, -0x198(%r15)
140a62a0a: 0f 84 c2 11 00 00           	je	0x140a63bd2 <.text+0xa62bd2>
140a62a10: f3 41 0f 10 97 6c fe ff ff  	movss	-0x194(%r15), %xmm2
140a62a19: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140a62a1d: 48 8d 95 30 01 00 00        	leaq	0x130(%rbp), %rdx
140a62a24: e8 b7 a3 23 00              	callq	0x140c9cde0 <.text+0xc9bde0>
140a62a29: 0f b6 05 d9 21 bd 02        	movzbl	0x2bd21d9(%rip), %eax   # 0x143634c09
140a62a30: b9 18 00 00 00              	movl	$0x18, %ecx
140a62a35: ba 04 00 00 00              	movl	$0x4, %edx
140a62a3a: e8 d1 11 bb ff              	callq	0x140613c10 <.text+0x612c10>
140a62a3f: 48 85 c0                    	testq	%rax, %rax
140a62a42: 0f 84 d2 11 00 00           	je	0x140a63c1a <.text+0xa62c1a>
140a62a48: f3 0f 11 38                 	movss	%xmm7, (%rax)
140a62a4c: 89 7d 20                    	movl	%edi, 0x20(%rbp)
140a62a4f: 89 78 04                    	movl	%edi, 0x4(%rax)
140a62a52: f3 44 0f 11 40 08           	movss	%xmm8, 0x8(%rax)
140a62a58: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140a62a5c: 48 89 48 0c                 	movq	%rcx, 0xc(%rax)
140a62a60: 8b 4d 58                    	movl	0x58(%rbp), %ecx
140a62a63: 89 48 14                    	movl	%ecx, 0x14(%rax)
140a62a66: 48 c7 45 70 02 00 00 00     	movq	$0x2, 0x70(%rbp)
140a62a6e: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
140a62a72: 48 c7 85 80 00 00 00 02 00 00 00    	movq	$0x2, 0x80(%rbp)
140a62a7d: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a62a87: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140a62a8e: c7 85 a0 00 00 00 00 00 00 00       	movl	$0x0, 0xa0(%rbp)
140a62a98: c7 85 b0 00 00 00 00 00 80 40       	movl	$0x40800000, 0xb0(%rbp) # imm = 0x40800000
140a62aa2: 66 c7 85 b4 00 00 00 01 00  	movw	$0x1, 0xb4(%rbp)
140a62aab: 48 8b 85 b8 02 00 00        	movq	0x2b8(%rbp), %rax
140a62ab2: 48 8b 10                    	movq	(%rax), %rdx
140a62ab5: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a62ab9: 48 89 95 58 01 00 00        	movq	%rdx, 0x158(%rbp)
140a62ac0: e8 ab 24 0a 00              	callq	0x140b04f70 <.text+0xb03f70>
140a62ac5: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
140a62acc: 48 8b 38                    	movq	(%rax), %rdi
140a62acf: 4c 8d ad 60 01 00 00        	leaq	0x160(%rbp), %r13
140a62ad6: 4c 89 6d 38                 	movq	%r13, 0x38(%rbp)
140a62ada: 48 8d 05 df 23 0d 00        	leaq	0xd23df(%rip), %rax     # 0x140b34ec0 <.text+0xb33ec0>
140a62ae1: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140a62ae5: 48 8d 05 24 17 07 02        	leaq	0x2071724(%rip), %rax   # 0x142ad4210
140a62aec: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a62af0: 48 c7 45 78 01 00 00 00     	movq	$0x1, 0x78(%rbp)
140a62af8: 48 8d 45 38                 	leaq	0x38(%rbp), %rax
140a62afc: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
140a62b03: 48 c7 85 88 00 00 00 01 00 00 00    	movq	$0x1, 0x88(%rbp)
140a62b0e: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
140a62b19: 4c 8d a5 30 01 00 00        	leaq	0x130(%rbp), %r12
140a62b20: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a62b24: 4c 89 e1                    	movq	%r12, %rcx
140a62b27: e8 64 74 31 00              	callq	0x140d79f90 <.text+0xd78f90>
140a62b2c: 8b 45 58                    	movl	0x58(%rbp), %eax
140a62b2f: 89 85 08 01 00 00           	movl	%eax, 0x108(%rbp)
140a62b35: 48 8b 45 50                 	movq	0x50(%rbp), %rax
140a62b39: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140a62b40: 48 8b 45 f9                 	movq	-0x7(%rbp), %rax
140a62b44: 48 8b 4d 00                 	movq	(%rbp), %rcx
140a62b48: 48 89 85 e9 00 00 00        	movq	%rax, 0xe9(%rbp)
140a62b4f: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
140a62b56: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
140a62b5e: 48 c7 45 78 08 00 00 00     	movq	$0x8, 0x78(%rbp)
140a62b66: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
140a62b71: c7 85 88 00 00 00 01 00 00 00       	movl	$0x1, 0x88(%rbp)
140a62b7b: 48 c7 85 90 00 00 00 01 00 00 00    	movq	$0x1, 0x90(%rbp)
140a62b86: c7 85 a0 00 00 00 02 00 00 00       	movl	$0x2, 0xa0(%rbp)
140a62b90: c7 85 b8 00 00 00 03 00 00 00       	movl	$0x3, 0xb8(%rbp)
140a62b9a: c7 85 d0 00 00 00 03 00 00 00       	movl	$0x3, 0xd0(%rbp)
140a62ba4: c6 85 e8 00 00 00 00        	movb	$0x0, 0xe8(%rbp)
140a62bab: c6 85 f8 00 00 00 00        	movb	$0x0, 0xf8(%rbp)
140a62bb2: c7 85 fc 00 00 00 00 00 00 3f       	movl	$0x3f000000, 0xfc(%rbp) # imm = 0x3F000000
140a62bbc: 66 c7 85 18 01 00 00 00 01  	movw	$0x100, 0x118(%rbp)     # imm = 0x100
140a62bc5: 4c 8d 45 70                 	leaq	0x70(%rbp), %r8
140a62bc9: 48 89 7d f0                 	movq	%rdi, -0x10(%rbp)
140a62bcd: 48 89 f9                    	movq	%rdi, %rcx
140a62bd0: 4c 89 e2                    	movq	%r12, %rdx
140a62bd3: e8 18 06 ec ff              	callq	0x1409231f0 <.text+0x9221f0>
140a62bd8: 49 89 c6                    	movq	%rax, %r14
140a62bdb: 80 78 18 00                 	cmpb	$0x0, 0x18(%rax)
140a62bdf: 48 89 55 18                 	movq	%rdx, 0x18(%rbp)
140a62be3: 0f 84 02 05 00 00           	je	0x140a630eb <.text+0xa620eb>
140a62be9: 49 39 56 38                 	cmpq	%rdx, 0x38(%r14)
140a62bed: 0f 85 e0 04 00 00           	jne	0x140a630d3 <.text+0xa620d3>
140a62bf3: 41 83 7e 28 02              	cmpl	$0x2, 0x28(%r14)
140a62bf8: 0f 84 d5 04 00 00           	je	0x140a630d3 <.text+0xa620d3>
140a62bfe: f3 41 0f 10 47 e4           	movss	-0x1c(%r15), %xmm0
140a62c04: 0f 2e 05 25 a4 f6 01        	ucomiss	0x1f6a425(%rip), %xmm0  # 0x1429cd030
140a62c0b: 0f 86 98 02 00 00           	jbe	0x140a62ea9 <.text+0xa61ea9>
140a62c11: f3 44 0f 10 65 50           	movss	0x50(%rbp), %xmm12
140a62c17: f3 44 0f 10 75 54           	movss	0x54(%rbp), %xmm14
140a62c1d: f3 44 0f 10 6d 58           	movss	0x58(%rbp), %xmm13
140a62c23: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a62c27: 48 89 f2                    	movq	%rsi, %rdx
140a62c2a: e8 c1 41 23 00              	callq	0x140c96df0 <.text+0xc95df0>
140a62c2f: f3 0f 10 45 70              	movss	0x70(%rbp), %xmm0
140a62c34: f3 0f 10 55 78              	movss	0x78(%rbp), %xmm2
140a62c39: 0f 28 0d b0 2c ec 01        	movaps	0x1ec2cb0(%rip), %xmm1  # 0x1429258f0
140a62c40: 0f 57 ca                    	xorps	%xmm2, %xmm1
140a62c43: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
140a62c47: 0f 28 d8                    	movaps	%xmm0, %xmm3
140a62c4a: f3 0f 59 d8                 	mulss	%xmm0, %xmm3
140a62c4e: f3 0f 58 da                 	addss	%xmm2, %xmm3
140a62c52: f3 0f 51 db                 	sqrtss	%xmm3, %xmm3
140a62c56: 0f 28 d6                    	movaps	%xmm6, %xmm2
140a62c59: f3 0f 5e d3                 	divss	%xmm3, %xmm2
140a62c5d: f3 0f 59 ca                 	mulss	%xmm2, %xmm1
140a62c61: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
140a62c65: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a62c69: e8 62 e3 22 00              	callq	0x140c90fd0 <.text+0xc8ffd0>
140a62c6e: 44 0f 28 7d 70              	movaps	0x70(%rbp), %xmm15
140a62c73: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
140a62c7a: 48 8b 00                    	movq	(%rax), %rax
140a62c7d: 48 83 c0 2c                 	addq	$0x2c, %rax
140a62c81: 4c 89 ad 30 01 00 00        	movq	%r13, 0x130(%rbp)
140a62c88: 48 8d 0d a1 3c 00 00        	leaq	0x3ca1(%rip), %rcx      # 0x140a66930 <.text+0xa65930>
140a62c8f: 48 89 8d 38 01 00 00        	movq	%rcx, 0x138(%rbp)
140a62c96: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140a62c9d: 48 8d 05 ec e5 5c ff        	leaq	-0xa31a14(%rip), %rax   # 0x140031290 <.text+0x30290>
140a62ca4: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
140a62cab: 48 8d 05 86 15 07 02        	leaq	0x2071586(%rip), %rax   # 0x142ad4238
140a62cb2: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a62cb6: 48 c7 45 78 02 00 00 00     	movq	$0x2, 0x78(%rbp)
140a62cbe: 4c 89 a5 80 00 00 00        	movq	%r12, 0x80(%rbp)
140a62cc5: 48 c7 85 88 00 00 00 02 00 00 00    	movq	$0x2, 0x88(%rbp)
140a62cd0: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
140a62cdb: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140a62cdf: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a62ce3: e8 a8 72 31 00              	callq	0x140d79f90 <.text+0xd78f90>
140a62ce8: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
140a62cec: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
140a62cf3: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
140a62cf7: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
140a62cfb: 4c 8b 6d e8                 	movq	-0x18(%rbp), %r13
140a62cff: 0f b6 05 03 1f bd 02        	movzbl	0x2bd1f03(%rip), %eax   # 0x143634c09
140a62d06: b9 30 00 00 00              	movl	$0x30, %ecx
140a62d0b: ba 10 00 00 00              	movl	$0x10, %edx
140a62d10: e8 fb 0e bb ff              	callq	0x140613c10 <.text+0x612c10>
140a62d15: 48 85 c0                    	testq	%rax, %rax
140a62d18: 0f 84 6e 0f 00 00           	je	0x140a63c8c <.text+0xa62c8c>
140a62d1e: 48 89 c7                    	movq	%rax, %rdi
140a62d21: f3 41 0f 16 c9              	movshdup	%xmm9, %xmm1            # xmm1 = xmm9[1,1,3,3]
140a62d26: f3 0f 10 05 62 ae f8 01     	movss	0x1f8ae62(%rip), %xmm0  # 0x1429edb90
140a62d2e: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
140a62d32: 41 0f 28 d7                 	movaps	%xmm15, %xmm2
140a62d36: 41 0f c6 d7 ff              	shufps	$0xff, %xmm15, %xmm2    # xmm2 = xmm2[3,3],xmm15[3,3]
140a62d3b: 0f 59 15 1e 09 07 02        	mulps	0x207091e(%rip), %xmm2  # 0x142ad3660
140a62d42: 41 0f 28 df                 	movaps	%xmm15, %xmm3
140a62d46: 41 0f c6 df 00              	shufps	$0x0, %xmm15, %xmm3     # xmm3 = xmm3[0,0],xmm15[0,0]
140a62d4b: 0f 59 1d 1e 09 07 02        	mulps	0x207091e(%rip), %xmm3  # 0x142ad3670
140a62d52: 0f 59 1d 17 a4 f6 01        	mulps	0x1f6a417(%rip), %xmm3  # 0x1429cd170
140a62d59: 41 0f 28 e7                 	movaps	%xmm15, %xmm4
140a62d5d: 41 0f c6 e7 55              	shufps	$0x55, %xmm15, %xmm4    # xmm4 = xmm4[1,1],xmm15[1,1]
140a62d62: 0f 59 25 17 09 07 02        	mulps	0x2070917(%rip), %xmm4  # 0x142ad3680
140a62d69: f3 41 0f 58 ce              	addss	%xmm14, %xmm1
140a62d6e: 0f 59 25 0b a4 f6 01        	mulps	0x1f6a40b(%rip), %xmm4  # 0x1429cd180
140a62d75: 45 0f c6 ff aa              	shufps	$0xaa, %xmm15, %xmm15   # xmm15 = xmm15[2,2,2,2]
140a62d7a: 44 0f 59 3d 0e 09 07 02     	mulps	0x207090e(%rip), %xmm15 # 0x142ad3690
140a62d82: 44 0f 59 3d 06 a4 f6 01     	mulps	0x1f6a406(%rip), %xmm15 # 0x1429cd190
140a62d8a: 0f 58 da                    	addps	%xmm2, %xmm3
140a62d8d: 44 0f 58 fc                 	addps	%xmm4, %xmm15
140a62d91: 44 0f 58 fb                 	addps	%xmm3, %xmm15
140a62d95: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
140a62d99: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
140a62d9d: f3 41 0f 58 d5              	addss	%xmm13, %xmm2
140a62da2: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
140a62da7: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
140a62daa: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140a62db1: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140a62db9: 44 0f 29 78 10              	movaps	%xmm15, 0x10(%rax)
140a62dbe: 0f 28 0d db 08 07 02        	movaps	0x20708db(%rip), %xmm1  # 0x142ad36a0
140a62dc5: f3 41 0f 10 cc              	movss	%xmm12, %xmm1           # xmm1 = xmm12[0],xmm1[1,2,3]
140a62dca: 0f 58 c8                    	addps	%xmm0, %xmm1
140a62dcd: 0f 13 48 20                 	movlps	%xmm1, 0x20(%rax)
140a62dd1: f3 0f 11 50 28              	movss	%xmm2, 0x28(%rax)
140a62dd6: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a62dda: 48 8d 05 77 14 07 02        	leaq	0x2071477(%rip), %rax   # 0x142ad4258
140a62de1: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
140a62de5: 4d 85 ed                    	testq	%r13, %r13
140a62de8: 0f 88 b2 0e 00 00           	js	0x140a63ca0 <.text+0xa62ca0>
140a62dee: 74 38                       	je	0x140a62e28 <.text+0xa61e28>
140a62df0: 48 89 bd 70 01 00 00        	movq	%rdi, 0x170(%rbp)
140a62df7: 0f b6 05 0b 1e bd 02        	movzbl	0x2bd1e0b(%rip), %eax   # 0x143634c09
140a62dfe: bf 01 00 00 00              	movl	$0x1, %edi
140a62e03: ba 01 00 00 00              	movl	$0x1, %edx
140a62e08: 4c 89 eb                    	movq	%r13, %rbx
140a62e0b: 4c 89 e9                    	movq	%r13, %rcx
140a62e0e: e8 fd 0d bb ff              	callq	0x140613c10 <.text+0x612c10>
140a62e13: 48 85 c0                    	testq	%rax, %rax
140a62e16: 0f 84 90 0e 00 00           	je	0x140a63cac <.text+0xa62cac>
140a62e1c: 49 89 c5                    	movq	%rax, %r13
140a62e1f: 48 8b bd 70 01 00 00        	movq	0x170(%rbp), %rdi
140a62e26: eb 09                       	jmp	0x140a62e31 <.text+0xa61e31>
140a62e28: 4c 89 eb                    	movq	%r13, %rbx
140a62e2b: 41 bd 01 00 00 00           	movl	$0x1, %r13d
140a62e31: 4c 89 e9                    	movq	%r13, %rcx
140a62e34: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140a62e38: 49 89 d8                    	movq	%rbx, %r8
140a62e3b: e8 c0 0d ec 01              	callq	0x142923c00 <.text+0x2922c00>
140a62e40: 48 89 5d 70                 	movq	%rbx, 0x70(%rbp)
140a62e44: 4c 89 6d 78                 	movq	%r13, 0x78(%rbp)
140a62e48: 48 89 9d 80 00 00 00        	movq	%rbx, 0x80(%rbp)
140a62e4f: 48 89 bd 88 00 00 00        	movq	%rdi, 0x88(%rbp)
140a62e56: 48 8d 05 fb 13 07 02        	leaq	0x20713fb(%rip), %rax   # 0x142ad4258
140a62e5d: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
140a62e64: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a62e68: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
140a62e6f: e8 0c 9a a0 00              	callq	0x14146c880 <.text+0x146b880>
140a62e74: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140a62e7b: 48 85 c0                    	testq	%rax, %rax
140a62e7e: 4c 8d ad 60 01 00 00        	leaq	0x160(%rbp), %r13
140a62e85: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
140a62e89: 74 18                       	je	0x140a62ea3 <.text+0xa61ea3>
140a62e8b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a62e91: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
140a62e95: 48 89 d7                    	movq	%rdx, %rdi
140a62e98: 48 89 c2                    	movq	%rax, %rdx
140a62e9b: e8 80 0d bb ff              	callq	0x140613c20 <.text+0x612c20>
140a62ea0: 48 89 fa                    	movq	%rdi, %rdx
140a62ea3: f3 41 0f 10 47 e4           	movss	-0x1c(%r15), %xmm0
140a62ea9: f3 0f 10 0d 4f af 01 02     	movss	0x201af4f(%rip), %xmm1  # 0x142a7de00
140a62eb1: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
140a62eb4: 0f 86 19 02 00 00           	jbe	0x140a630d3 <.text+0xa620d3>
140a62eba: f2 44 0f 10 65 50           	movsd	0x50(%rbp), %xmm12
140a62ec0: f3 44 0f 10 6d 58           	movss	0x58(%rbp), %xmm13
140a62ec6: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a62eca: 48 89 f2                    	movq	%rsi, %rdx
140a62ecd: e8 1e 3f 23 00              	callq	0x140c96df0 <.text+0xc95df0>
140a62ed2: f3 0f 10 45 70              	movss	0x70(%rbp), %xmm0
140a62ed7: f3 0f 10 55 78              	movss	0x78(%rbp), %xmm2
140a62edc: 0f 28 0d 0d 2a ec 01        	movaps	0x1ec2a0d(%rip), %xmm1  # 0x1429258f0
140a62ee3: 0f 57 ca                    	xorps	%xmm2, %xmm1
140a62ee6: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
140a62eea: 0f 28 d8                    	movaps	%xmm0, %xmm3
140a62eed: f3 0f 59 d8                 	mulss	%xmm0, %xmm3
140a62ef1: f3 0f 58 da                 	addss	%xmm2, %xmm3
140a62ef5: f3 0f 51 db                 	sqrtss	%xmm3, %xmm3
140a62ef9: 0f 28 d6                    	movaps	%xmm6, %xmm2
140a62efc: f3 0f 5e d3                 	divss	%xmm3, %xmm2
140a62f00: f3 0f 59 ca                 	mulss	%xmm2, %xmm1
140a62f04: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
140a62f08: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a62f0c: e8 bf e0 22 00              	callq	0x140c90fd0 <.text+0xc8ffd0>
140a62f11: 44 0f 28 5d 70              	movaps	0x70(%rbp), %xmm11
140a62f16: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
140a62f1d: 48 8b 00                    	movq	(%rax), %rax
140a62f20: 48 83 c0 2c                 	addq	$0x2c, %rax
140a62f24: 4c 89 ad 30 01 00 00        	movq	%r13, 0x130(%rbp)
140a62f2b: 48 8d 0d fe 39 00 00        	leaq	0x39fe(%rip), %rcx      # 0x140a66930 <.text+0xa65930>
140a62f32: 48 89 8d 38 01 00 00        	movq	%rcx, 0x138(%rbp)
140a62f39: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140a62f40: 48 8d 05 49 e3 5c ff        	leaq	-0xa31cb7(%rip), %rax   # 0x140031290 <.text+0x30290>
140a62f47: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
140a62f4e: 48 8d 05 43 13 07 02        	leaq	0x2071343(%rip), %rax   # 0x142ad4298
140a62f55: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a62f59: 48 c7 45 78 02 00 00 00     	movq	$0x2, 0x78(%rbp)
140a62f61: 4c 89 a5 80 00 00 00        	movq	%r12, 0x80(%rbp)
140a62f68: 48 c7 85 88 00 00 00 02 00 00 00    	movq	$0x2, 0x88(%rbp)
140a62f73: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
140a62f7e: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140a62f82: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a62f86: e8 05 70 31 00              	callq	0x140d79f90 <.text+0xd78f90>
140a62f8b: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
140a62f8f: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
140a62f96: 48 8b 45 c8                 	movq	-0x38(%rbp), %rax
140a62f9a: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
140a62f9e: 4c 8b 6d d0                 	movq	-0x30(%rbp), %r13
140a62fa2: 0f b6 05 60 1c bd 02        	movzbl	0x2bd1c60(%rip), %eax   # 0x143634c09
140a62fa9: b9 30 00 00 00              	movl	$0x30, %ecx
140a62fae: ba 10 00 00 00              	movl	$0x10, %edx
140a62fb3: e8 58 0c bb ff              	callq	0x140613c10 <.text+0x612c10>
140a62fb8: 48 85 c0                    	testq	%rax, %rax
140a62fbb: 0f 84 02 0d 00 00           	je	0x140a63cc3 <.text+0xa62cc3>
140a62fc1: f3 0f 10 05 c7 ab f8 01     	movss	0x1f8abc7(%rip), %xmm0  # 0x1429edb90
140a62fc9: f3 41 0f 59 c2              	mulss	%xmm10, %xmm0
140a62fce: f3 41 0f 58 c5              	addss	%xmm13, %xmm0
140a62fd3: 0f 57 c9                    	xorps	%xmm1, %xmm1
140a62fd6: f3 0f 58 c8                 	addss	%xmm0, %xmm1
140a62fda: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140a62fe1: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140a62fe9: 44 0f 29 58 10              	movaps	%xmm11, 0x10(%rax)
140a62fee: 0f 28 05 bb 06 07 02        	movaps	0x20706bb(%rip), %xmm0  # 0x142ad36b0
140a62ff5: 41 0f 59 c1                 	mulps	%xmm9, %xmm0
140a62ff9: 41 0f 58 c4                 	addps	%xmm12, %xmm0
140a62ffd: 0f 58 05 fc fb 01 02        	addps	0x201fbfc(%rip), %xmm0  # 0x142a82c00
140a63004: 0f 13 40 20                 	movlps	%xmm0, 0x20(%rax)
140a63008: f3 0f 11 48 28              	movss	%xmm1, 0x28(%rax)
140a6300d: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a63011: 48 8d 1d a0 12 07 02        	leaq	0x20712a0(%rip), %rbx   # 0x142ad42b8
140a63018: 48 89 5d 78                 	movq	%rbx, 0x78(%rbp)
140a6301c: 4d 85 ed                    	testq	%r13, %r13
140a6301f: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
140a63026: 0f 88 ab 0c 00 00           	js	0x140a63cd7 <.text+0xa62cd7>
140a6302c: 74 2e                       	je	0x140a6305c <.text+0xa6205c>
140a6302e: 0f b6 05 d4 1b bd 02        	movzbl	0x2bd1bd4(%rip), %eax   # 0x143634c09
140a63035: b8 01 00 00 00              	movl	$0x1, %eax
140a6303a: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
140a63041: ba 01 00 00 00              	movl	$0x1, %edx
140a63046: 4c 89 e9                    	movq	%r13, %rcx
140a63049: e8 c2 0b bb ff              	callq	0x140613c10 <.text+0x612c10>
140a6304e: 48 85 c0                    	testq	%rax, %rax
140a63051: 0f 84 8b 0c 00 00           	je	0x140a63ce2 <.text+0xa62ce2>
140a63057: 48 89 c7                    	movq	%rax, %rdi
140a6305a: eb 05                       	jmp	0x140a63061 <.text+0xa62061>
140a6305c: bf 01 00 00 00              	movl	$0x1, %edi
140a63061: 48 89 f9                    	movq	%rdi, %rcx
140a63064: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140a63068: 4d 89 e8                    	movq	%r13, %r8
140a6306b: e8 90 0b ec 01              	callq	0x142923c00 <.text+0x2922c00>
140a63070: 4c 89 6d 70                 	movq	%r13, 0x70(%rbp)
140a63074: 48 89 7d 78                 	movq	%rdi, 0x78(%rbp)
140a63078: 4c 89 ad 80 00 00 00        	movq	%r13, 0x80(%rbp)
140a6307f: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
140a63086: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140a6308d: 48 89 9d 90 00 00 00        	movq	%rbx, 0x90(%rbp)
140a63094: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a63098: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
140a6309f: e8 dc 97 a0 00              	callq	0x14146c880 <.text+0x146b880>
140a630a4: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140a630ab: 48 85 c0                    	testq	%rax, %rax
140a630ae: 4c 8d ad 60 01 00 00        	leaq	0x160(%rbp), %r13
140a630b5: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
140a630b9: 74 18                       	je	0x140a630d3 <.text+0xa620d3>
140a630bb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a630c1: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
140a630c5: 48 89 d7                    	movq	%rdx, %rdi
140a630c8: 48 89 c2                    	movq	%rax, %rdx
140a630cb: e8 50 0b bb ff              	callq	0x140613c20 <.text+0x612c20>
140a630d0: 48 89 fa                    	movq	%rdi, %rdx
140a630d3: 41 80 7e 18 00              	cmpb	$0x0, 0x18(%r14)
140a630d8: 74 11                       	je	0x140a630eb <.text+0xa620eb>
140a630da: 49 39 56 38                 	cmpq	%rdx, 0x38(%r14)
140a630de: 75 0b                       	jne	0x140a630eb <.text+0xa620eb>
140a630e0: 41 83 7e 28 02              	cmpl	$0x2, 0x28(%r14)
140a630e5: 40 0f 95 c7                 	setne	%dil
140a630e9: eb 02                       	jmp	0x140a630ed <.text+0xa620ed>
140a630eb: 31 ff                       	xorl	%edi, %edi
140a630ed: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
140a630f4: 48 8b 00                    	movq	(%rax), %rax
140a630f7: 48 83 c0 2c                 	addq	$0x2c, %rax
140a630fb: 4c 89 ad 30 01 00 00        	movq	%r13, 0x130(%rbp)
140a63102: 48 8d 0d 27 38 00 00        	leaq	0x3827(%rip), %rcx      # 0x140a66930 <.text+0xa65930>
140a63109: 48 89 8d 38 01 00 00        	movq	%rcx, 0x138(%rbp)
140a63110: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
140a63117: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140a6311e: 48 8d 05 6b e1 5c ff        	leaq	-0xa31e95(%rip), %rax   # 0x140031290 <.text+0x30290>
140a63125: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
140a6312c: 48 8d 05 c5 11 07 02        	leaq	0x20711c5(%rip), %rax   # 0x142ad42f8
140a63133: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a63137: 48 c7 45 78 02 00 00 00     	movq	$0x2, 0x78(%rbp)
140a6313f: 4c 89 a5 80 00 00 00        	movq	%r12, 0x80(%rbp)
140a63146: 48 c7 85 88 00 00 00 02 00 00 00    	movq	$0x2, 0x88(%rbp)
140a63151: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
140a6315c: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140a63160: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a63164: e8 27 6e 31 00              	callq	0x140d79f90 <.text+0xd78f90>
140a63169: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
140a6316d: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
140a63174: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
140a63178: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
140a6317c: 4c 8b 65 b8                 	movq	-0x48(%rbp), %r12
140a63180: 0f b6 05 82 1a bd 02        	movzbl	0x2bd1a82(%rip), %eax   # 0x143634c09
140a63187: b9 20 00 00 00              	movl	$0x20, %ecx
140a6318c: ba 08 00 00 00              	movl	$0x8, %edx
140a63191: e8 7a 0a bb ff              	callq	0x140613c10 <.text+0x612c10>
140a63196: 48 85 c0                    	testq	%rax, %rax
140a63199: 0f 84 8a 0a 00 00           	je	0x140a63c29 <.text+0xa62c29>
140a6319f: 40 84 ff                    	testb	%dil, %dil
140a631a2: 75 10                       	jne	0x140a631b4 <.text+0xa621b4>
140a631a4: f3 0f 10 05 cc d5 f3 01     	movss	0x1f3d5cc(%rip), %xmm0  # 0x1429a0778
140a631ac: f3 0f 11 85 50 01 00 00     	movss	%xmm0, 0x150(%rbp)
140a631b4: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140a631bb: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140a631c3: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140a631c7: 48 89 48 10                 	movq	%rcx, 0x10(%rax)
140a631cb: 8b 4d 58                    	movl	0x58(%rbp), %ecx
140a631ce: 89 48 18                    	movl	%ecx, 0x18(%rax)
140a631d1: f3 0f 10 85 50 01 00 00     	movss	0x150(%rbp), %xmm0
140a631d9: f3 0f 11 40 1c              	movss	%xmm0, 0x1c(%rax)
140a631de: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a631e2: 48 8d 1d 2f 11 07 02        	leaq	0x207112f(%rip), %rbx   # 0x142ad4318
140a631e9: 48 89 5d 78                 	movq	%rbx, 0x78(%rbp)
140a631ed: 4d 85 e4                    	testq	%r12, %r12
140a631f0: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
140a631f7: 0f 88 40 0a 00 00           	js	0x140a63c3d <.text+0xa62c3d>
140a631fd: 74 2f                       	je	0x140a6322e <.text+0xa6222e>
140a631ff: 0f b6 05 03 1a bd 02        	movzbl	0x2bd1a03(%rip), %eax   # 0x143634c09
140a63206: 41 bd 01 00 00 00           	movl	$0x1, %r13d
140a6320c: ba 01 00 00 00              	movl	$0x1, %edx
140a63211: 4c 89 e1                    	movq	%r12, %rcx
140a63214: e8 f7 09 bb ff              	callq	0x140613c10 <.text+0x612c10>
140a63219: 48 85 c0                    	testq	%rax, %rax
140a6321c: 0f 84 1e 0a 00 00           	je	0x140a63c40 <.text+0xa62c40>
140a63222: 48 89 c7                    	movq	%rax, %rdi
140a63225: 4c 8d ad 60 01 00 00        	leaq	0x160(%rbp), %r13
140a6322c: eb 05                       	jmp	0x140a63233 <.text+0xa62233>
140a6322e: bf 01 00 00 00              	movl	$0x1, %edi
140a63233: 48 89 f9                    	movq	%rdi, %rcx
140a63236: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140a6323a: 4d 89 e0                    	movq	%r12, %r8
140a6323d: e8 be 09 ec 01              	callq	0x142923c00 <.text+0x2922c00>
140a63242: 4c 89 65 70                 	movq	%r12, 0x70(%rbp)
140a63246: 48 89 7d 78                 	movq	%rdi, 0x78(%rbp)
140a6324a: 4c 89 a5 80 00 00 00        	movq	%r12, 0x80(%rbp)
140a63251: 48 8b 85 28 01 00 00        	movq	0x128(%rbp), %rax
140a63258: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140a6325f: 48 89 9d 90 00 00 00        	movq	%rbx, 0x90(%rbp)
140a63266: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a6326a: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
140a63271: e8 0a 96 a0 00              	callq	0x14146c880 <.text+0x146b880>
140a63276: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
140a6327d: 48 85 d2                    	testq	%rdx, %rdx
140a63280: 8b 5d 20                    	movl	0x20(%rbp), %ebx
140a63283: 74 0f                       	je	0x140a63294 <.text+0xa62294>
140a63285: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a6328b: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
140a6328f: e8 8c 09 bb ff              	callq	0x140613c20 <.text+0x612c20>
140a63294: b0 02                       	movb	$0x2, %al
140a63296: 41 80 7e 18 02              	cmpb	$0x2, 0x18(%r14)
140a6329b: 72 0f                       	jb	0x140a632ac <.text+0xa622ac>
140a6329d: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
140a632a1: 49 39 4e 38                 	cmpq	%rcx, 0x38(%r14)
140a632a5: 75 05                       	jne	0x140a632ac <.text+0xa622ac>
140a632a7: 41 0f b6 46 19              	movzbl	0x19(%r14), %eax
140a632ac: 04 fe                       	addb	$-0x2, %al
140a632ae: 0f b6 c0                    	movzbl	%al, %eax
140a632b1: 3c 03                       	cmpb	$0x3, %al
140a632b3: b9 03 00 00 00              	movl	$0x3, %ecx
140a632b8: 0f 42 c8                    	cmovbl	%eax, %ecx
140a632bb: 0f b6 c1                    	movzbl	%cl, %eax
140a632be: 48 8d 0d 1f 16 07 02        	leaq	0x207161f(%rip), %rcx   # 0x142ad48e4
140a632c5: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
140a632c9: 48 01 c8                    	addq	%rcx, %rax
140a632cc: ff e0                       	jmpq	*%rax
140a632ce: 48 8b bd 60 01 00 00        	movq	0x160(%rbp), %rdi
140a632d5: 48 8b 95 a0 02 00 00        	movq	0x2a0(%rbp), %rdx
140a632dc: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140a632df: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140a632e3: 89 01                       	movl	%eax, (%rcx)
140a632e5: 48 8b 1a                    	movq	(%rdx), %rbx
140a632e8: 4c 8b 73 30                 	movq	0x30(%rbx), %r14
140a632ec: 4c 8b 63 40                 	movq	0x40(%rbx), %r12
140a632f0: 4c 3b 73 20                 	cmpq	0x20(%rbx), %r14
140a632f4: 75 10                       	jne	0x140a63306 <.text+0xa62306>
140a632f6: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
140a632fa: 48 8d 15 47 6e 1d 02        	leaq	0x21d6e47(%rip), %rdx   # 0x142c3a148
140a63301: e8 2a f3 af ff              	callq	0x140562630 <.text+0x561630>
140a63306: 48 8b 43 28                 	movq	0x28(%rbx), %rax
140a6330a: 4c 89 f1                    	movq	%r14, %rcx
140a6330d: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a63311: 4c 89 24 08                 	movq	%r12, (%rax,%rcx)
140a63315: 48 89 7c 08 08              	movq	%rdi, 0x8(%rax,%rcx)
140a6331a: 49 ff c6                    	incq	%r14
140a6331d: 4c 89 73 30                 	movq	%r14, 0x30(%rbx)
140a63321: 48 ff 43 40                 	incq	0x40(%rbx)
140a63325: 48 8b 95 a8 02 00 00        	movq	0x2a8(%rbp), %rdx
140a6332c: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140a6332f: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140a63333: 89 01                       	movl	%eax, (%rcx)
140a63335: 48 8b 3a                    	movq	(%rdx), %rdi
140a63338: 4c 8b 77 30                 	movq	0x30(%rdi), %r14
140a6333c: 48 8b 5f 40                 	movq	0x40(%rdi), %rbx
140a63340: 4c 3b 77 20                 	cmpq	0x20(%rdi), %r14
140a63344: 75 10                       	jne	0x140a63356 <.text+0xa62356>
140a63346: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
140a6334a: 48 8d 15 f7 6d 1d 02        	leaq	0x21d6df7(%rip), %rdx   # 0x142c3a148
140a63351: e8 1a f5 af ff              	callq	0x140562870 <.text+0x561870>
140a63356: 48 8d 47 40                 	leaq	0x40(%rdi), %rax
140a6335a: 48 8b 4f 28                 	movq	0x28(%rdi), %rcx
140a6335e: 48 83 c7 30                 	addq	$0x30, %rdi
140a63362: 4a 89 1c f1                 	movq	%rbx, (%rcx,%r14,8)
140a63366: e9 8e 00 00 00              	jmp	0x140a633f9 <.text+0xa623f9>
140a6336b: 48 8b 9d 60 01 00 00        	movq	0x160(%rbp), %rbx
140a63372: 48 8b 95 a0 02 00 00        	movq	0x2a0(%rbp), %rdx
140a63379: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140a6337c: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140a63380: 89 01                       	movl	%eax, (%rcx)
140a63382: 48 8b 3a                    	movq	(%rdx), %rdi
140a63385: 4c 8b 77 30                 	movq	0x30(%rdi), %r14
140a63389: 4c 8b 67 40                 	movq	0x40(%rdi), %r12
140a6338d: 4c 3b 77 20                 	cmpq	0x20(%rdi), %r14
140a63391: 75 4a                       	jne	0x140a633dd <.text+0xa623dd>
140a63393: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
140a63397: 48 8d 15 aa 6d 1d 02        	leaq	0x21d6daa(%rip), %rdx   # 0x142c3a148
140a6339e: e8 8d f2 af ff              	callq	0x140562630 <.text+0x561630>
140a633a3: eb 38                       	jmp	0x140a633dd <.text+0xa623dd>
140a633a5: 48 8b 9d 60 01 00 00        	movq	0x160(%rbp), %rbx
140a633ac: 48 8b 95 98 02 00 00        	movq	0x298(%rbp), %rdx
140a633b3: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140a633b6: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140a633ba: 89 01                       	movl	%eax, (%rcx)
140a633bc: 48 8b 3a                    	movq	(%rdx), %rdi
140a633bf: 4c 8b 77 30                 	movq	0x30(%rdi), %r14
140a633c3: 4c 8b 67 40                 	movq	0x40(%rdi), %r12
140a633c7: 4c 3b 77 20                 	cmpq	0x20(%rdi), %r14
140a633cb: 75 10                       	jne	0x140a633dd <.text+0xa623dd>
140a633cd: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
140a633d1: 48 8d 15 70 6d 1d 02        	leaq	0x21d6d70(%rip), %rdx   # 0x142c3a148
140a633d8: e8 53 f2 af ff              	callq	0x140562630 <.text+0x561630>
140a633dd: 48 8d 47 40                 	leaq	0x40(%rdi), %rax
140a633e1: 48 8b 4f 28                 	movq	0x28(%rdi), %rcx
140a633e5: 48 83 c7 30                 	addq	$0x30, %rdi
140a633e9: 4c 89 f2                    	movq	%r14, %rdx
140a633ec: 48 c1 e2 04                 	shlq	$0x4, %rdx
140a633f0: 4c 89 24 11                 	movq	%r12, (%rcx,%rdx)
140a633f4: 48 89 5c 11 08              	movq	%rbx, 0x8(%rcx,%rdx)
140a633f9: 8b 5d 20                    	movl	0x20(%rbp), %ebx
140a633fc: 49 ff c6                    	incq	%r14
140a633ff: 4c 89 37                    	movq	%r14, (%rdi)
140a63402: 48 ff 00                    	incq	(%rax)
140a63405: 4c 89 6d 38                 	movq	%r13, 0x38(%rbp)
140a63409: 48 8d 05 b0 1a 0d 00        	leaq	0xd1ab0(%rip), %rax     # 0x140b34ec0 <.text+0xb33ec0>
140a63410: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140a63414: 48 8d 05 3d 0f 07 02        	leaq	0x2070f3d(%rip), %rax   # 0x142ad4358
140a6341b: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a6341f: 48 c7 45 78 01 00 00 00     	movq	$0x1, 0x78(%rbp)
140a63427: 48 8d 45 38                 	leaq	0x38(%rbp), %rax
140a6342b: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
140a63432: 48 c7 85 88 00 00 00 01 00 00 00    	movq	$0x1, 0x88(%rbp)
140a6343d: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
140a63448: 4c 8d b5 30 01 00 00        	leaq	0x130(%rbp), %r14
140a6344f: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a63453: 4c 89 f1                    	movq	%r14, %rcx
140a63456: e8 35 6b 31 00              	callq	0x140d79f90 <.text+0xd78f90>
140a6345b: 48 8b 45 f9                 	movq	-0x7(%rbp), %rax
140a6345f: 48 8b 4d 00                 	movq	(%rbp), %rcx
140a63463: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
140a6346a: 48 89 85 e9 00 00 00        	movq	%rax, 0xe9(%rbp)
140a63471: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
140a63479: 48 c7 45 78 08 00 00 00     	movq	$0x8, 0x78(%rbp)
140a63481: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
140a6348c: c7 85 88 00 00 00 01 00 00 00       	movl	$0x1, 0x88(%rbp)
140a63496: 48 c7 85 90 00 00 00 01 00 00 00    	movq	$0x1, 0x90(%rbp)
140a634a1: c7 85 a0 00 00 00 02 00 00 00       	movl	$0x2, 0xa0(%rbp)
140a634ab: c7 85 b8 00 00 00 03 00 00 00       	movl	$0x3, 0xb8(%rbp)
140a634b5: c7 85 d0 00 00 00 03 00 00 00       	movl	$0x3, 0xd0(%rbp)
140a634bf: c6 85 e8 00 00 00 00        	movb	$0x0, 0xe8(%rbp)
140a634c6: c6 85 f8 00 00 00 00        	movb	$0x0, 0xf8(%rbp)
140a634cd: c7 85 fc 00 00 00 00 00 00 3f       	movl	$0x3f000000, 0xfc(%rbp) # imm = 0x3F000000
140a634d7: f3 0f 11 bd 00 01 00 00     	movss	%xmm7, 0x100(%rbp)
140a634df: 89 9d 04 01 00 00           	movl	%ebx, 0x104(%rbp)
140a634e5: f3 44 0f 11 85 08 01 00 00  	movss	%xmm8, 0x108(%rbp)
140a634ee: 66 c7 85 18 01 00 00 00 01  	movw	$0x100, 0x118(%rbp)     # imm = 0x100
140a634f7: 4c 8d 45 70                 	leaq	0x70(%rbp), %r8
140a634fb: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
140a634ff: 4c 89 f2                    	movq	%r14, %rdx
140a63502: e8 e9 fc eb ff              	callq	0x1409231f0 <.text+0x9221f0>
140a63507: 49 89 d5                    	movq	%rdx, %r13
140a6350a: f3 44 0f 10 1d 65 d2 f3 01  	movss	0x1f3d265(%rip), %xmm11 # 0x1429a0778
140a63513: 80 78 18 00                 	cmpb	$0x0, 0x18(%rax)
140a63517: 0f 84 24 02 00 00           	je	0x140a63741 <.text+0xa62741>
140a6351d: 4c 39 68 38                 	cmpq	%r13, 0x38(%rax)
140a63521: 0f 85 98 04 00 00           	jne	0x140a639bf <.text+0xa629bf>
140a63527: 83 78 28 02                 	cmpl	$0x2, 0x28(%rax)
140a6352b: 0f 84 8e 04 00 00           	je	0x140a639bf <.text+0xa629bf>
140a63531: f3 41 0f 10 47 e0           	movss	-0x20(%r15), %xmm0
140a63537: 0f 57 c9                    	xorps	%xmm1, %xmm1
140a6353a: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140a6353d: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
140a63541: 0f 86 79 02 00 00           	jbe	0x140a637c0 <.text+0xa627c0>
140a63547: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a6354b: 48 89 f2                    	movq	%rsi, %rdx
140a6354e: e8 9d 38 23 00              	callq	0x140c96df0 <.text+0xc95df0>
140a63553: f3 0f 10 45 70              	movss	0x70(%rbp), %xmm0
140a63558: f3 0f 10 55 78              	movss	0x78(%rbp), %xmm2
140a6355d: 0f 28 0d 8c 23 ec 01        	movaps	0x1ec238c(%rip), %xmm1  # 0x1429258f0
140a63564: 0f 57 ca                    	xorps	%xmm2, %xmm1
140a63567: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
140a6356b: 0f 28 d8                    	movaps	%xmm0, %xmm3
140a6356e: f3 0f 59 d8                 	mulss	%xmm0, %xmm3
140a63572: f3 0f 58 da                 	addss	%xmm2, %xmm3
140a63576: f3 0f 51 db                 	sqrtss	%xmm3, %xmm3
140a6357a: 0f 28 d6                    	movaps	%xmm6, %xmm2
140a6357d: f3 0f 5e d3                 	divss	%xmm3, %xmm2
140a63581: f3 0f 59 ca                 	mulss	%xmm2, %xmm1
140a63585: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
140a63589: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a6358d: e8 3e da 22 00              	callq	0x140c90fd0 <.text+0xc8ffd0>
140a63592: 44 0f 28 65 70              	movaps	0x70(%rbp), %xmm12
140a63597: 48 8d 85 60 01 00 00        	leaq	0x160(%rbp), %rax
140a6359e: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
140a635a5: 48 8d 05 84 33 00 00        	leaq	0x3384(%rip), %rax      # 0x140a66930 <.text+0xa65930>
140a635ac: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
140a635b3: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140a635ba: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140a635c1: 48 8d 05 c8 dc 5c ff        	leaq	-0xa32338(%rip), %rax   # 0x140031290 <.text+0x30290>
140a635c8: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
140a635cf: 48 8d 05 aa 0d 07 02        	leaq	0x2070daa(%rip), %rax   # 0x142ad4380
140a635d6: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a635da: 48 c7 45 78 02 00 00 00     	movq	$0x2, 0x78(%rbp)
140a635e2: 4c 89 b5 80 00 00 00        	movq	%r14, 0x80(%rbp)
140a635e9: 48 c7 85 88 00 00 00 02 00 00 00    	movq	$0x2, 0x88(%rbp)
140a635f4: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
140a635ff: 48 8d 4d 38                 	leaq	0x38(%rbp), %rcx
140a63603: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a63607: e8 84 69 31 00              	callq	0x140d79f90 <.text+0xd78f90>
140a6360c: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140a63610: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
140a63617: 4c 8b 65 48                 	movq	0x48(%rbp), %r12
140a6361b: 0f b6 05 e7 15 bd 02        	movzbl	0x2bd15e7(%rip), %eax   # 0x143634c09
140a63622: b9 30 00 00 00              	movl	$0x30, %ecx
140a63627: ba 10 00 00 00              	movl	$0x10, %edx
140a6362c: e8 df 05 bb ff              	callq	0x140613c10 <.text+0x612c10>
140a63631: 48 85 c0                    	testq	%rax, %rax
140a63634: 0f 84 c0 06 00 00           	je	0x140a63cfa <.text+0xa62cfa>
140a6363a: f3 41 0f 16 d1              	movshdup	%xmm9, %xmm2            # xmm2 = xmm9[1,1,3,3]
140a6363f: f3 0f 10 05 49 a5 f8 01     	movss	0x1f8a549(%rip), %xmm0  # 0x1429edb90
140a63647: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
140a6364b: f3 0f 10 4d 20              	movss	0x20(%rbp), %xmm1
140a63650: 41 0f 28 dc                 	movaps	%xmm12, %xmm3
140a63654: 41 0f c6 dc ff              	shufps	$0xff, %xmm12, %xmm3    # xmm3 = xmm3[3,3],xmm12[3,3]
140a63659: 0f 59 1d 00 00 07 02        	mulps	0x2070000(%rip), %xmm3  # 0x142ad3660
140a63660: 41 0f 28 e4                 	movaps	%xmm12, %xmm4
140a63664: 41 0f c6 e4 00              	shufps	$0x0, %xmm12, %xmm4     # xmm4 = xmm4[0,0],xmm12[0,0]
140a63669: 0f 59 25 00 00 07 02        	mulps	0x2070000(%rip), %xmm4  # 0x142ad3670
140a63670: 0f 59 25 f9 9a f6 01        	mulps	0x1f69af9(%rip), %xmm4  # 0x1429cd170
140a63677: 41 0f 28 ec                 	movaps	%xmm12, %xmm5
140a6367b: 41 0f c6 ec 55              	shufps	$0x55, %xmm12, %xmm5    # xmm5 = xmm5[1,1],xmm12[1,1]
140a63680: 0f 59 2d f9 ff 06 02        	mulps	0x206fff9(%rip), %xmm5  # 0x142ad3680
140a63687: f3 0f 58 ca                 	addss	%xmm2, %xmm1
140a6368b: 0f 59 2d ee 9a f6 01        	mulps	0x1f69aee(%rip), %xmm5  # 0x1429cd180
140a63692: 45 0f c6 e4 aa              	shufps	$0xaa, %xmm12, %xmm12   # xmm12 = xmm12[2,2,2,2]
140a63697: 44 0f 59 25 f1 ff 06 02     	mulps	0x206fff1(%rip), %xmm12 # 0x142ad3690
140a6369f: 44 0f 59 25 e9 9a f6 01     	mulps	0x1f69ae9(%rip), %xmm12 # 0x1429cd190
140a636a7: 0f 58 e3                    	addps	%xmm3, %xmm4
140a636aa: 44 0f 58 e5                 	addps	%xmm5, %xmm12
140a636ae: 44 0f 58 e4                 	addps	%xmm4, %xmm12
140a636b2: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
140a636b6: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
140a636ba: f3 41 0f 58 d0              	addss	%xmm8, %xmm2
140a636bf: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
140a636c4: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
140a636c7: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140a636ce: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140a636d6: 44 0f 29 60 10              	movaps	%xmm12, 0x10(%rax)
140a636db: 0f 28 0d be ff 06 02        	movaps	0x206ffbe(%rip), %xmm1  # 0x142ad36a0
140a636e2: f3 0f 10 cf                 	movss	%xmm7, %xmm1            # xmm1 = xmm7[0],xmm1[1,2,3]
140a636e6: 0f 58 c8                    	addps	%xmm0, %xmm1
140a636e9: 0f 13 48 20                 	movlps	%xmm1, 0x20(%rax)
140a636ed: f3 0f 11 50 28              	movss	%xmm2, 0x28(%rax)
140a636f2: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a636f6: 48 8d 1d a3 0c 07 02        	leaq	0x2070ca3(%rip), %rbx   # 0x142ad43a0
140a636fd: 48 89 5d 78                 	movq	%rbx, 0x78(%rbp)
140a63701: 4d 85 e4                    	testq	%r12, %r12
140a63704: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
140a6370b: 0f 88 fa 05 00 00           	js	0x140a63d0b <.text+0xa62d0b>
140a63711: 74 3a                       	je	0x140a6374d <.text+0xa6274d>
140a63713: 0f b6 05 ef 14 bd 02        	movzbl	0x2bd14ef(%rip), %eax   # 0x143634c09
140a6371a: b8 01 00 00 00              	movl	$0x1, %eax
140a6371f: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
140a63726: ba 01 00 00 00              	movl	$0x1, %edx
140a6372b: 4c 89 e1                    	movq	%r12, %rcx
140a6372e: e8 dd 04 bb ff              	callq	0x140613c10 <.text+0x612c10>
140a63733: 48 85 c0                    	testq	%rax, %rax
140a63736: 0f 84 da 05 00 00           	je	0x140a63d16 <.text+0xa62d16>
140a6373c: 48 89 c7                    	movq	%rax, %rdi
140a6373f: eb 11                       	jmp	0x140a63752 <.text+0xa62752>
140a63741: 4c 8b bd b0 02 00 00        	movq	0x2b0(%rbp), %r15
140a63748: e9 9b 02 00 00              	jmp	0x140a639e8 <.text+0xa629e8>
140a6374d: bf 01 00 00 00              	movl	$0x1, %edi
140a63752: 48 89 f9                    	movq	%rdi, %rcx
140a63755: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
140a6375c: 4d 89 e0                    	movq	%r12, %r8
140a6375f: e8 9c 04 ec 01              	callq	0x142923c00 <.text+0x2922c00>
140a63764: 4c 89 65 70                 	movq	%r12, 0x70(%rbp)
140a63768: 48 89 7d 78                 	movq	%rdi, 0x78(%rbp)
140a6376c: 4c 89 a5 80 00 00 00        	movq	%r12, 0x80(%rbp)
140a63773: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
140a6377a: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140a63781: 48 89 9d 90 00 00 00        	movq	%rbx, 0x90(%rbp)
140a63788: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a6378c: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
140a63793: e8 e8 90 a0 00              	callq	0x14146c880 <.text+0x146b880>
140a63798: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
140a6379c: 48 85 d2                    	testq	%rdx, %rdx
140a6379f: 8b 5d 20                    	movl	0x20(%rbp), %ebx
140a637a2: 74 12                       	je	0x140a637b6 <.text+0xa627b6>
140a637a4: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a637aa: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
140a637b1: e8 6a 04 bb ff              	callq	0x140613c20 <.text+0x612c20>
140a637b6: f3 41 0f 10 47 e0           	movss	-0x20(%r15), %xmm0
140a637bc: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140a637c0: f3 0f 10 0d 04 e1 f6 01     	movss	0x1f6e104(%rip), %xmm1  # 0x1429d18cc
140a637c8: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
140a637cb: 0f 86 ee 01 00 00           	jbe	0x140a639bf <.text+0xa629bf>
140a637d1: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a637d5: 48 89 f2                    	movq	%rsi, %rdx
140a637d8: e8 13 36 23 00              	callq	0x140c96df0 <.text+0xc95df0>
140a637dd: f3 0f 10 45 70              	movss	0x70(%rbp), %xmm0
140a637e2: f3 0f 10 55 78              	movss	0x78(%rbp), %xmm2
140a637e7: 0f 28 0d 02 21 ec 01        	movaps	0x1ec2102(%rip), %xmm1  # 0x1429258f0
140a637ee: 0f 57 ca                    	xorps	%xmm2, %xmm1
140a637f1: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
140a637f5: 0f 28 d8                    	movaps	%xmm0, %xmm3
140a637f8: f3 0f 59 d8                 	mulss	%xmm0, %xmm3
140a637fc: f3 0f 58 da                 	addss	%xmm2, %xmm3
140a63800: 0f 57 d2                    	xorps	%xmm2, %xmm2
140a63803: f3 0f 51 d3                 	sqrtss	%xmm3, %xmm2
140a63807: f3 0f 5e f2                 	divss	%xmm2, %xmm6
140a6380b: f3 0f 59 ce                 	mulss	%xmm6, %xmm1
140a6380f: f3 0f 59 f0                 	mulss	%xmm0, %xmm6
140a63813: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a63817: 0f 28 d6                    	movaps	%xmm6, %xmm2
140a6381a: e8 b1 d7 22 00              	callq	0x140c90fd0 <.text+0xc8ffd0>
140a6381f: 0f 28 75 70                 	movaps	0x70(%rbp), %xmm6
140a63823: 48 8d 85 60 01 00 00        	leaq	0x160(%rbp), %rax
140a6382a: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
140a63831: 48 8d 05 f8 30 00 00        	leaq	0x30f8(%rip), %rax      # 0x140a66930 <.text+0xa65930>
140a63838: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
140a6383f: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140a63846: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140a6384d: 48 8d 05 3c da 5c ff        	leaq	-0xa325c4(%rip), %rax   # 0x140031290 <.text+0x30290>
140a63854: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
140a6385b: 48 8d 05 86 0b 07 02        	leaq	0x2070b86(%rip), %rax   # 0x142ad43e8
140a63862: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a63866: 48 c7 45 78 02 00 00 00     	movq	$0x2, 0x78(%rbp)
140a6386e: 4c 89 b5 80 00 00 00        	movq	%r14, 0x80(%rbp)
140a63875: 48 c7 85 88 00 00 00 02 00 00 00    	movq	$0x2, 0x88(%rbp)
140a63880: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
140a6388b: 48 8d 4d 38                 	leaq	0x38(%rbp), %rcx
140a6388f: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a63893: e8 f8 66 31 00              	callq	0x140d79f90 <.text+0xd78f90>
140a63898: 48 8b 75 40                 	movq	0x40(%rbp), %rsi
140a6389c: 4c 8b 7d 48                 	movq	0x48(%rbp), %r15
140a638a0: 0f b6 05 62 13 bd 02        	movzbl	0x2bd1362(%rip), %eax   # 0x143634c09
140a638a7: b9 30 00 00 00              	movl	$0x30, %ecx
140a638ac: ba 10 00 00 00              	movl	$0x10, %edx
140a638b1: e8 5a 03 bb ff              	callq	0x140613c10 <.text+0x612c10>
140a638b6: 48 85 c0                    	testq	%rax, %rax
140a638b9: 0f 84 6f 04 00 00           	je	0x140a63d2e <.text+0xa62d2e>
140a638bf: 49 89 c4                    	movq	%rax, %r12
140a638c2: f3 44 0f 59 15 c5 a2 f8 01  	mulss	0x1f8a2c5(%rip), %xmm10 # 0x1429edb90
140a638cb: f3 45 0f 58 d0              	addss	%xmm8, %xmm10
140a638d0: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a638d3: 44 0f 59 0d d5 fd 06 02     	mulps	0x206fdd5(%rip), %xmm9  # 0x142ad36b0
140a638db: f3 41 0f 58 c2              	addss	%xmm10, %xmm0
140a638e0: 66 0f 6e cb                 	movd	%ebx, %xmm1
140a638e4: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140a638eb: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140a638f3: 0f 29 70 10                 	movaps	%xmm6, 0x10(%rax)
140a638f7: 0f 28 d7                    	movaps	%xmm7, %xmm2
140a638fa: 0f 14 d1                    	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
140a638fd: 41 0f 58 d1                 	addps	%xmm9, %xmm2
140a63901: 0f 58 15 f8 f2 01 02        	addps	0x201f2f8(%rip), %xmm2  # 0x142a82c00
140a63908: 0f 13 50 20                 	movlps	%xmm2, 0x20(%rax)
140a6390c: f3 0f 11 40 28              	movss	%xmm0, 0x28(%rax)
140a63911: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a63915: 48 8d 1d ec 0a 07 02        	leaq	0x2070aec(%rip), %rbx   # 0x142ad4408
140a6391c: 48 89 5d 78                 	movq	%rbx, 0x78(%rbp)
140a63920: 4d 85 ff                    	testq	%r15, %r15
140a63923: 0f 88 16 04 00 00           	js	0x140a63d3f <.text+0xa62d3f>
140a63929: 74 36                       	je	0x140a63961 <.text+0xa62961>
140a6392b: 4c 89 a5 50 01 00 00        	movq	%r12, 0x150(%rbp)
140a63932: 0f b6 05 d0 12 bd 02        	movzbl	0x2bd12d0(%rip), %eax   # 0x143634c09
140a63939: 41 bc 01 00 00 00           	movl	$0x1, %r12d
140a6393f: ba 01 00 00 00              	movl	$0x1, %edx
140a63944: 4c 89 f9                    	movq	%r15, %rcx
140a63947: e8 c4 02 bb ff              	callq	0x140613c10 <.text+0x612c10>
140a6394c: 48 85 c0                    	testq	%rax, %rax
140a6394f: 0f 84 f4 03 00 00           	je	0x140a63d49 <.text+0xa62d49>
140a63955: 48 89 c7                    	movq	%rax, %rdi
140a63958: 4c 8b a5 50 01 00 00        	movq	0x150(%rbp), %r12
140a6395f: eb 05                       	jmp	0x140a63966 <.text+0xa62966>
140a63961: bf 01 00 00 00              	movl	$0x1, %edi
140a63966: 48 89 f9                    	movq	%rdi, %rcx
140a63969: 48 89 f2                    	movq	%rsi, %rdx
140a6396c: 4d 89 f8                    	movq	%r15, %r8
140a6396f: e8 8c 02 ec 01              	callq	0x142923c00 <.text+0x2922c00>
140a63974: 4c 89 7d 70                 	movq	%r15, 0x70(%rbp)
140a63978: 48 89 7d 78                 	movq	%rdi, 0x78(%rbp)
140a6397c: 4c 89 bd 80 00 00 00        	movq	%r15, 0x80(%rbp)
140a63983: 4c 89 a5 88 00 00 00        	movq	%r12, 0x88(%rbp)
140a6398a: 48 89 9d 90 00 00 00        	movq	%rbx, 0x90(%rbp)
140a63991: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a63995: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
140a6399c: e8 df 8e a0 00              	callq	0x14146c880 <.text+0x146b880>
140a639a1: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
140a639a5: 48 85 d2                    	testq	%rdx, %rdx
140a639a8: 8b 5d 20                    	movl	0x20(%rbp), %ebx
140a639ab: 74 0e                       	je	0x140a639bb <.text+0xa629bb>
140a639ad: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a639b3: 48 89 f1                    	movq	%rsi, %rcx
140a639b6: e8 65 02 bb ff              	callq	0x140613c20 <.text+0x612c20>
140a639bb: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140a639bf: 80 78 18 00                 	cmpb	$0x0, 0x18(%rax)
140a639c3: 4c 8b bd b0 02 00 00        	movq	0x2b0(%rbp), %r15
140a639ca: 74 1c                       	je	0x140a639e8 <.text+0xa629e8>
140a639cc: 4c 39 68 38                 	cmpq	%r13, 0x38(%rax)
140a639d0: 75 16                       	jne	0x140a639e8 <.text+0xa629e8>
140a639d2: 31 c9                       	xorl	%ecx, %ecx
140a639d4: 83 78 28 02                 	cmpl	$0x2, 0x28(%rax)
140a639d8: 0f 94 c1                    	sete	%cl
140a639db: 48 8d 05 de fc 06 02        	leaq	0x206fcde(%rip), %rax   # 0x142ad36c0
140a639e2: f3 44 0f 10 1c 88           	movss	(%rax,%rcx,4), %xmm11
140a639e8: 48 8d 85 60 01 00 00        	leaq	0x160(%rbp), %rax
140a639ef: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
140a639f6: 48 8d 05 33 2f 00 00        	leaq	0x2f33(%rip), %rax      # 0x140a66930 <.text+0xa65930>
140a639fd: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
140a63a04: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140a63a0b: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140a63a12: 48 8d 05 77 d8 5c ff        	leaq	-0xa32789(%rip), %rax   # 0x140031290 <.text+0x30290>
140a63a19: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
140a63a20: 48 8d 05 21 0a 07 02        	leaq	0x2070a21(%rip), %rax   # 0x142ad4448
140a63a27: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a63a2b: 48 c7 45 78 02 00 00 00     	movq	$0x2, 0x78(%rbp)
140a63a33: 4c 89 b5 80 00 00 00        	movq	%r14, 0x80(%rbp)
140a63a3a: 48 c7 85 88 00 00 00 02 00 00 00    	movq	$0x2, 0x88(%rbp)
140a63a45: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
140a63a50: 48 8d 4d 38                 	leaq	0x38(%rbp), %rcx
140a63a54: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a63a58: e8 33 65 31 00              	callq	0x140d79f90 <.text+0xd78f90>
140a63a5d: 48 8b 75 40                 	movq	0x40(%rbp), %rsi
140a63a61: 4c 8b 75 48                 	movq	0x48(%rbp), %r14
140a63a65: 0f b6 05 9d 11 bd 02        	movzbl	0x2bd119d(%rip), %eax   # 0x143634c09
140a63a6c: b9 20 00 00 00              	movl	$0x20, %ecx
140a63a71: ba 08 00 00 00              	movl	$0x8, %edx
140a63a76: e8 95 01 bb ff              	callq	0x140613c10 <.text+0x612c10>
140a63a7b: 48 85 c0                    	testq	%rax, %rax
140a63a7e: 0f 84 d3 01 00 00           	je	0x140a63c57 <.text+0xa62c57>
140a63a84: 49 89 c4                    	movq	%rax, %r12
140a63a87: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140a63a8e: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140a63a96: f3 0f 11 78 10              	movss	%xmm7, 0x10(%rax)
140a63a9b: 89 58 14                    	movl	%ebx, 0x14(%rax)
140a63a9e: f3 44 0f 11 40 18           	movss	%xmm8, 0x18(%rax)
140a63aa4: f3 44 0f 11 58 1c           	movss	%xmm11, 0x1c(%rax)
140a63aaa: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a63aae: 48 8d 1d b3 09 07 02        	leaq	0x20709b3(%rip), %rbx   # 0x142ad4468
140a63ab5: 48 89 5d 78                 	movq	%rbx, 0x78(%rbp)
140a63ab9: 4d 85 f6                    	testq	%r14, %r14
140a63abc: 0f 88 a9 01 00 00           	js	0x140a63c6b <.text+0xa62c6b>
140a63ac2: 74 2f                       	je	0x140a63af3 <.text+0xa62af3>
140a63ac4: 0f b6 05 3e 11 bd 02        	movzbl	0x2bd113e(%rip), %eax   # 0x143634c09
140a63acb: 41 bf 01 00 00 00           	movl	$0x1, %r15d
140a63ad1: ba 01 00 00 00              	movl	$0x1, %edx
140a63ad6: 4c 89 f1                    	movq	%r14, %rcx
140a63ad9: e8 32 01 bb ff              	callq	0x140613c10 <.text+0x612c10>
140a63ade: 48 85 c0                    	testq	%rax, %rax
140a63ae1: 0f 84 87 01 00 00           	je	0x140a63c6e <.text+0xa62c6e>
140a63ae7: 48 89 c7                    	movq	%rax, %rdi
140a63aea: 4c 8b bd b0 02 00 00        	movq	0x2b0(%rbp), %r15
140a63af1: eb 05                       	jmp	0x140a63af8 <.text+0xa62af8>
140a63af3: bf 01 00 00 00              	movl	$0x1, %edi
140a63af8: 48 89 f9                    	movq	%rdi, %rcx
140a63afb: 48 89 f2                    	movq	%rsi, %rdx
140a63afe: 4d 89 f0                    	movq	%r14, %r8
140a63b01: e8 fa 00 ec 01              	callq	0x142923c00 <.text+0x2922c00>
140a63b06: 4c 89 75 70                 	movq	%r14, 0x70(%rbp)
140a63b0a: 48 89 7d 78                 	movq	%rdi, 0x78(%rbp)
140a63b0e: 4c 89 b5 80 00 00 00        	movq	%r14, 0x80(%rbp)
140a63b15: 4c 89 a5 88 00 00 00        	movq	%r12, 0x88(%rbp)
140a63b1c: 48 89 9d 90 00 00 00        	movq	%rbx, 0x90(%rbp)
140a63b23: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
140a63b27: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
140a63b2e: e8 4d 8d a0 00              	callq	0x14146c880 <.text+0x146b880>
140a63b33: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
140a63b37: 48 85 d2                    	testq	%rdx, %rdx
140a63b3a: 74 0e                       	je	0x140a63b4a <.text+0xa62b4a>
140a63b3c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a63b42: 48 89 f1                    	movq	%rsi, %rcx
140a63b45: e8 d6 00 bb ff              	callq	0x140613c20 <.text+0x612c20>
140a63b4a: 48 8b bd 60 01 00 00        	movq	0x160(%rbp), %rdi
140a63b51: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
140a63b55: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
140a63b59: 89 01                       	movl	%eax, (%rcx)
140a63b5b: 49 8b 37                    	movq	(%r15), %rsi
140a63b5e: 48 8b 5e 30                 	movq	0x30(%rsi), %rbx
140a63b62: 4c 8b 76 40                 	movq	0x40(%rsi), %r14
140a63b66: 48 3b 5e 20                 	cmpq	0x20(%rsi), %rbx
140a63b6a: 75 10                       	jne	0x140a63b7c <.text+0xa62b7c>
140a63b6c: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
140a63b70: 48 8d 15 d1 65 1d 02        	leaq	0x21d65d1(%rip), %rdx   # 0x142c3a148
140a63b77: e8 34 f2 af ff              	callq	0x140562db0 <.text+0x561db0>
140a63b7c: 48 8b 46 28                 	movq	0x28(%rsi), %rax
140a63b80: 48 8d 0c 5b                 	leaq	(%rbx,%rbx,2), %rcx
140a63b84: 4c 89 2c c8                 	movq	%r13, (%rax,%rcx,8)
140a63b88: 48 89 7c c8 08              	movq	%rdi, 0x8(%rax,%rcx,8)
140a63b8d: 4c 89 74 c8 10              	movq	%r14, 0x10(%rax,%rcx,8)
140a63b92: 48 ff c3                    	incq	%rbx
140a63b95: 48 89 5e 30                 	movq	%rbx, 0x30(%rsi)
140a63b99: 48 ff 46 40                 	incq	0x40(%rsi)
140a63b9d: e9 ab e9 ff ff              	jmp	0x140a6254d <.text+0xa6154d>
140a63ba2: 48 8d 0d ea 65 07 02        	leaq	0x20765ea(%rip), %rcx   # 0x142ada193
140a63ba9: 4c 8d 05 d8 05 07 02        	leaq	0x20705d8(%rip), %r8    # 0x142ad4188
140a63bb0: ba 32 00 00 00              	movl	$0x32, %edx
140a63bb5: e8 76 58 e7 01              	callq	0x1428d9430 <.text+0x28d8430>
140a63bba: 48 8d 0d d2 65 07 02        	leaq	0x20765d2(%rip), %rcx   # 0x142ada193
140a63bc1: 4c 8d 05 f0 05 07 02        	leaq	0x20705f0(%rip), %r8    # 0x142ad41b8
140a63bc8: ba 32 00 00 00              	movl	$0x32, %edx
140a63bcd: e8 5e 58 e7 01              	callq	0x1428d9430 <.text+0x28d8430>
140a63bd2: 48 8d 05 c7 72 08 02        	leaq	0x20872c7(%rip), %rax   # 0x142aeaea0
140a63bd9: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140a63bdd: 48 c7 45 78 01 00 00 00     	movq	$0x1, 0x78(%rbp)
140a63be5: 48 c7 85 80 00 00 00 08 00 00 00    	movq	$0x8, 0x80(%rbp)
140a63bf0: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a63bf3: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
140a63bfa: 48 8d 15 af 72 08 02        	leaq	0x20872af(%rip), %rdx   # 0x142aeaeb0
140a63c01: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a63c05: e8 86 57 e7 01              	callq	0x1428d9390 <.text+0x28d8390>
140a63c0a: 4c 8d 05 d7 05 07 02        	leaq	0x20705d7(%rip), %r8    # 0x142ad41e8
140a63c11: 31 c9                       	xorl	%ecx, %ecx
140a63c13: 31 d2                       	xorl	%edx, %edx
140a63c15: e8 fe 58 e7 01              	callq	0x1428d9518 <.text+0x28d8518>
140a63c1a: b9 04 00 00 00              	movl	$0x4, %ecx
140a63c1f: ba 18 00 00 00              	movl	$0x18, %edx
140a63c24: e8 d7 53 e7 01              	callq	0x1428d9000 <.text+0x28d8000>
140a63c29: b9 08 00 00 00              	movl	$0x8, %ecx
140a63c2e: ba 20 00 00 00              	movl	$0x20, %edx
140a63c33: e8 c8 53 e7 01              	callq	0x1428d9000 <.text+0x28d8000>
140a63c38: e9 1e 01 00 00              	jmp	0x140a63d5b <.text+0xa62d5b>
140a63c3d: 45 31 ed                    	xorl	%r13d, %r13d
140a63c40: 4c 8d 05 d9 52 03 02        	leaq	0x20352d9(%rip), %r8    # 0x142a98f20
140a63c47: 4c 89 e9                    	movq	%r13, %rcx
140a63c4a: 4c 89 e2                    	movq	%r12, %rdx
140a63c4d: e8 91 53 e7 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a63c52: e9 04 01 00 00              	jmp	0x140a63d5b <.text+0xa62d5b>
140a63c57: b9 08 00 00 00              	movl	$0x8, %ecx
140a63c5c: ba 20 00 00 00              	movl	$0x20, %edx
140a63c61: e8 9a 53 e7 01              	callq	0x1428d9000 <.text+0x28d8000>
140a63c66: e9 f0 00 00 00              	jmp	0x140a63d5b <.text+0xa62d5b>
140a63c6b: 45 31 ff                    	xorl	%r15d, %r15d
140a63c6e: 4c 89 a5 58 01 00 00        	movq	%r12, 0x158(%rbp)
140a63c75: 4c 8d 05 a4 52 03 02        	leaq	0x20352a4(%rip), %r8    # 0x142a98f20
140a63c7c: 4c 89 f9                    	movq	%r15, %rcx
140a63c7f: 4c 89 f2                    	movq	%r14, %rdx
140a63c82: e8 5c 53 e7 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a63c87: e9 cf 00 00 00              	jmp	0x140a63d5b <.text+0xa62d5b>
140a63c8c: b9 10 00 00 00              	movl	$0x10, %ecx
140a63c91: ba 30 00 00 00              	movl	$0x30, %edx
140a63c96: e8 65 53 e7 01              	callq	0x1428d9000 <.text+0x28d8000>
140a63c9b: e9 bb 00 00 00              	jmp	0x140a63d5b <.text+0xa62d5b>
140a63ca0: 4c 89 eb                    	movq	%r13, %rbx
140a63ca3: 48 89 bd 70 01 00 00        	movq	%rdi, 0x170(%rbp)
140a63caa: 31 ff                       	xorl	%edi, %edi
140a63cac: 4c 8d 05 6d 52 03 02        	leaq	0x203526d(%rip), %r8    # 0x142a98f20
140a63cb3: 48 89 f9                    	movq	%rdi, %rcx
140a63cb6: 48 89 da                    	movq	%rbx, %rdx
140a63cb9: e8 25 53 e7 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a63cbe: e9 98 00 00 00              	jmp	0x140a63d5b <.text+0xa62d5b>
140a63cc3: b9 10 00 00 00              	movl	$0x10, %ecx
140a63cc8: ba 30 00 00 00              	movl	$0x30, %edx
140a63ccd: e8 2e 53 e7 01              	callq	0x1428d9000 <.text+0x28d8000>
140a63cd2: e9 84 00 00 00              	jmp	0x140a63d5b <.text+0xa62d5b>
140a63cd7: 48 c7 85 28 01 00 00 00 00 00 00    	movq	$0x0, 0x128(%rbp)
140a63ce2: 4c 8d 05 37 52 03 02        	leaq	0x2035237(%rip), %r8    # 0x142a98f20
140a63ce9: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
140a63cf0: 4c 89 ea                    	movq	%r13, %rdx
140a63cf3: e8 eb 52 e7 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a63cf8: eb 61                       	jmp	0x140a63d5b <.text+0xa62d5b>
140a63cfa: b9 10 00 00 00              	movl	$0x10, %ecx
140a63cff: ba 30 00 00 00              	movl	$0x30, %edx
140a63d04: e8 f7 52 e7 01              	callq	0x1428d9000 <.text+0x28d8000>
140a63d09: eb 50                       	jmp	0x140a63d5b <.text+0xa62d5b>
140a63d0b: 48 c7 85 28 01 00 00 00 00 00 00    	movq	$0x0, 0x128(%rbp)
140a63d16: 4c 8d 05 03 52 03 02        	leaq	0x2035203(%rip), %r8    # 0x142a98f20
140a63d1d: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
140a63d24: 4c 89 e2                    	movq	%r12, %rdx
140a63d27: e8 b7 52 e7 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a63d2c: eb 2d                       	jmp	0x140a63d5b <.text+0xa62d5b>
140a63d2e: b9 10 00 00 00              	movl	$0x10, %ecx
140a63d33: ba 30 00 00 00              	movl	$0x30, %edx
140a63d38: e8 c3 52 e7 01              	callq	0x1428d9000 <.text+0x28d8000>
140a63d3d: eb 1c                       	jmp	0x140a63d5b <.text+0xa62d5b>
140a63d3f: 4c 89 a5 50 01 00 00        	movq	%r12, 0x150(%rbp)
140a63d46: 45 31 e4                    	xorl	%r12d, %r12d
140a63d49: 4c 8d 05 d0 51 03 02        	leaq	0x20351d0(%rip), %r8    # 0x142a98f20
140a63d50: 4c 89 e1                    	movq	%r12, %rcx
140a63d53: 4c 89 fa                    	movq	%r15, %rdx
140a63d56: e8 88 52 e7 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a63d5b: 0f 0b                       	ud2
140a63d5d: 0f 1f 00                    	nopl	(%rax)
140a63d60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a63d65: 55                          	pushq	%rbp
140a63d66: 41 57                       	pushq	%r15
140a63d68: 41 56                       	pushq	%r14
140a63d6a: 41 55                       	pushq	%r13
140a63d6c: 41 54                       	pushq	%r12
140a63d6e: 56                          	pushq	%rsi
140a63d6f: 57                          	pushq	%rdi
140a63d70: 53                          	pushq	%rbx
140a63d71: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a63d78: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a63d7f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
140a63d85: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
140a63d8b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
140a63d91: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
140a63d97: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
140a63d9d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140a63da3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
140a63dac: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
140a63db5: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
140a63dbd: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140a63dc5: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
140a63dcc: f0                          	lock
140a63dcd: 48 ff 08                    	decq	(%rax)
140a63dd0: 75 09                       	jne	0x140a63ddb <.text+0xa62ddb>
140a63dd2: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a63dd6: e8 f5 bb be ff              	callq	0x14064f9d0 <.text+0x64e9d0>
140a63ddb: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140a63de3: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
140a63deb: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
140a63df4: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
140a63dfd: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140a63e03: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
140a63e09: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
140a63e0f: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
140a63e15: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
140a63e1b: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
140a63e21: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a63e28: 5b                          	popq	%rbx
140a63e29: 5f                          	popq	%rdi
140a63e2a: 5e                          	popq	%rsi
140a63e2b: 41 5c                       	popq	%r12
140a63e2d: 41 5d                       	popq	%r13
140a63e2f: 41 5e                       	popq	%r14
140a63e31: 41 5f                       	popq	%r15
140a63e33: 5d                          	popq	%rbp
140a63e34: c3                          	retq
140a63e35: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a63e40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a63e45: 55                          	pushq	%rbp
140a63e46: 41 57                       	pushq	%r15
140a63e48: 41 56                       	pushq	%r14
140a63e4a: 41 55                       	pushq	%r13
140a63e4c: 41 54                       	pushq	%r12
140a63e4e: 56                          	pushq	%rsi
140a63e4f: 57                          	pushq	%rdi
140a63e50: 53                          	pushq	%rbx
140a63e51: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a63e58: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a63e5f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
140a63e65: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
140a63e6b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
140a63e71: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
140a63e77: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
140a63e7d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140a63e83: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
140a63e8c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
140a63e95: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
140a63e9d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140a63ea5: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
140a63eac: f0                          	lock
140a63ead: 48 ff 08                    	decq	(%rax)
140a63eb0: 75 09                       	jne	0x140a63ebb <.text+0xa62ebb>
140a63eb2: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a63eb6: e8 15 bb be ff              	callq	0x14064f9d0 <.text+0x64e9d0>
140a63ebb: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140a63ec3: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
140a63ecb: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
140a63ed4: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
140a63edd: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140a63ee3: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
140a63ee9: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
140a63eef: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
140a63ef5: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
140a63efb: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
140a63f01: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a63f08: 5b                          	popq	%rbx
140a63f09: 5f                          	popq	%rdi
140a63f0a: 5e                          	popq	%rsi
140a63f0b: 41 5c                       	popq	%r12
140a63f0d: 41 5d                       	popq	%r13
140a63f0f: 41 5e                       	popq	%r14
140a63f11: 41 5f                       	popq	%r15
140a63f13: 5d                          	popq	%rbp
140a63f14: c3                          	retq
140a63f15: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a63f20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a63f25: 55                          	pushq	%rbp
140a63f26: 41 57                       	pushq	%r15
140a63f28: 41 56                       	pushq	%r14
140a63f2a: 41 55                       	pushq	%r13
140a63f2c: 41 54                       	pushq	%r12
140a63f2e: 56                          	pushq	%rsi
140a63f2f: 57                          	pushq	%rdi
140a63f30: 53                          	pushq	%rbx
140a63f31: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a63f38: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a63f3f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
140a63f45: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
140a63f4b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
140a63f51: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
140a63f57: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
140a63f5d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140a63f63: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
140a63f6c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
140a63f75: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
140a63f7d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140a63f85: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
140a63f8c: f0                          	lock
140a63f8d: 48 ff 08                    	decq	(%rax)
140a63f90: 75 09                       	jne	0x140a63f9b <.text+0xa62f9b>
140a63f92: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a63f96: e8 35 ba be ff              	callq	0x14064f9d0 <.text+0x64e9d0>
140a63f9b: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140a63fa3: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
140a63fab: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
140a63fb4: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
140a63fbd: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140a63fc3: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
140a63fc9: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
140a63fcf: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
140a63fd5: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
140a63fdb: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
140a63fe1: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a63fe8: 5b                          	popq	%rbx
140a63fe9: 5f                          	popq	%rdi
140a63fea: 5e                          	popq	%rsi
140a63feb: 41 5c                       	popq	%r12
140a63fed: 41 5d                       	popq	%r13
140a63fef: 41 5e                       	popq	%r14
140a63ff1: 41 5f                       	popq	%r15
140a63ff3: 5d                          	popq	%rbp
140a63ff4: c3                          	retq
140a63ff5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a64000: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a64005: 55                          	pushq	%rbp
140a64006: 41 57                       	pushq	%r15
140a64008: 41 56                       	pushq	%r14
140a6400a: 41 55                       	pushq	%r13
140a6400c: 41 54                       	pushq	%r12
140a6400e: 56                          	pushq	%rsi
140a6400f: 57                          	pushq	%rdi
140a64010: 53                          	pushq	%rbx
140a64011: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a64018: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a6401f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
140a64025: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
140a6402b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
140a64031: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
140a64037: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
140a6403d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140a64043: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
140a6404c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
140a64055: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
140a6405d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140a64065: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
140a6406c: f0                          	lock
140a6406d: 48 ff 08                    	decq	(%rax)
140a64070: 75 09                       	jne	0x140a6407b <.text+0xa6307b>
140a64072: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a64076: e8 55 b9 be ff              	callq	0x14064f9d0 <.text+0x64e9d0>
140a6407b: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140a64083: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
140a6408b: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
140a64094: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
140a6409d: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140a640a3: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
140a640a9: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
140a640af: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
140a640b5: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
140a640bb: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
140a640c1: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a640c8: 5b                          	popq	%rbx
140a640c9: 5f                          	popq	%rdi
140a640ca: 5e                          	popq	%rsi
140a640cb: 41 5c                       	popq	%r12
140a640cd: 41 5d                       	popq	%r13
140a640cf: 41 5e                       	popq	%r14
140a640d1: 41 5f                       	popq	%r15
140a640d3: 5d                          	popq	%rbp
140a640d4: c3                          	retq
140a640d5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a640e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a640e5: 55                          	pushq	%rbp
140a640e6: 41 57                       	pushq	%r15
140a640e8: 41 56                       	pushq	%r14
140a640ea: 41 55                       	pushq	%r13
140a640ec: 41 54                       	pushq	%r12
140a640ee: 56                          	pushq	%rsi
140a640ef: 57                          	pushq	%rdi
140a640f0: 53                          	pushq	%rbx
140a640f1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a640f8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a640ff: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
140a64105: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
140a6410b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
140a64111: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
140a64117: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
140a6411d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140a64123: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
140a6412c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
140a64135: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
140a6413d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140a64145: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
140a64149: 48 85 d2                    	testq	%rdx, %rdx
140a6414c: 74 0f                       	je	0x140a6415d <.text+0xa6315d>
140a6414e: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140a64152: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a64158: e8 c3 fa ba ff              	callq	0x140613c20 <.text+0x612c20>
140a6415d: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140a64165: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
140a6416d: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
140a64176: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
140a6417f: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140a64185: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
140a6418b: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
140a64191: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
140a64197: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
140a6419d: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
140a641a3: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a641aa: 5b                          	popq	%rbx
140a641ab: 5f                          	popq	%rdi
140a641ac: 5e                          	popq	%rsi
140a641ad: 41 5c                       	popq	%r12
140a641af: 41 5d                       	popq	%r13
140a641b1: 41 5e                       	popq	%r14
140a641b3: 41 5f                       	popq	%r15
140a641b5: 5d                          	popq	%rbp
140a641b6: c3                          	retq
140a641b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a641c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a641c5: 55                          	pushq	%rbp
140a641c6: 41 57                       	pushq	%r15
140a641c8: 41 56                       	pushq	%r14
140a641ca: 41 55                       	pushq	%r13
140a641cc: 41 54                       	pushq	%r12
140a641ce: 56                          	pushq	%rsi
140a641cf: 57                          	pushq	%rdi
140a641d0: 53                          	pushq	%rbx
140a641d1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a641d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a641df: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
140a641e5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
140a641eb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
140a641f1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
140a641f7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
140a641fd: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140a64203: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
140a6420c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
140a64215: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
140a6421d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140a64225: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
140a64229: 48 85 d2                    	testq	%rdx, %rdx
140a6422c: 74 0f                       	je	0x140a6423d <.text+0xa6323d>
140a6422e: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140a64232: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a64238: e8 e3 f9 ba ff              	callq	0x140613c20 <.text+0x612c20>
140a6423d: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140a64245: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
140a6424d: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
140a64256: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
140a6425f: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140a64265: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
140a6426b: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
140a64271: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
140a64277: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
140a6427d: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
140a64283: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a6428a: 5b                          	popq	%rbx
140a6428b: 5f                          	popq	%rdi
140a6428c: 5e                          	popq	%rsi
140a6428d: 41 5c                       	popq	%r12
140a6428f: 41 5d                       	popq	%r13
140a64291: 41 5e                       	popq	%r14
140a64293: 41 5f                       	popq	%r15
140a64295: 5d                          	popq	%rbp
140a64296: c3                          	retq
140a64297: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a642a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a642a5: 55                          	pushq	%rbp
140a642a6: 41 57                       	pushq	%r15
140a642a8: 41 56                       	pushq	%r14
140a642aa: 41 55                       	pushq	%r13
140a642ac: 41 54                       	pushq	%r12
140a642ae: 56                          	pushq	%rsi
140a642af: 57                          	pushq	%rdi
140a642b0: 53                          	pushq	%rbx
140a642b1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a642b8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a642bf: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
140a642c5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
140a642cb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
140a642d1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
140a642d7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
140a642dd: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140a642e3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
140a642ec: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
140a642f5: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
140a642fd: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140a64305: 48 83 bd 68 01 00 00 00     	cmpq	$0x0, 0x168(%rbp)
140a6430d: 74 16                       	je	0x140a64325 <.text+0xa63325>
140a6430f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a64315: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
140a64319: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
140a64320: e8 fb f8 ba ff              	callq	0x140613c20 <.text+0x612c20>
140a64325: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140a6432d: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
140a64335: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
140a6433e: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
140a64347: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140a6434d: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
140a64353: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
140a64359: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
140a6435f: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
140a64365: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
140a6436b: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a64372: 5b                          	popq	%rbx
140a64373: 5f                          	popq	%rdi
140a64374: 5e                          	popq	%rsi
140a64375: 41 5c                       	popq	%r12
140a64377: 41 5d                       	popq	%r13
140a64379: 41 5e                       	popq	%r14
140a6437b: 41 5f                       	popq	%r15
140a6437d: 5d                          	popq	%rbp
140a6437e: c3                          	retq
140a6437f: 90                          	nop
140a64380: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a64385: 55                          	pushq	%rbp
140a64386: 41 57                       	pushq	%r15
140a64388: 41 56                       	pushq	%r14
140a6438a: 41 55                       	pushq	%r13
140a6438c: 41 54                       	pushq	%r12
140a6438e: 56                          	pushq	%rsi
140a6438f: 57                          	pushq	%rdi
140a64390: 53                          	pushq	%rbx
140a64391: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a64398: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a6439f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
140a643a5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
140a643ab: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
140a643b1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
140a643b7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
140a643bd: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140a643c3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
140a643cc: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
140a643d5: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
140a643dd: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140a643e5: 48 83 bd 68 01 00 00 00     	cmpq	$0x0, 0x168(%rbp)
140a643ed: 74 16                       	je	0x140a64405 <.text+0xa63405>
140a643ef: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a643f5: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
140a643f9: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
140a64400: e8 1b f8 ba ff              	callq	0x140613c20 <.text+0x612c20>
140a64405: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140a6440d: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
140a64415: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
140a6441e: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
140a64427: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140a6442d: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
140a64433: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
140a64439: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
140a6443f: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
140a64445: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
140a6444b: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a64452: 5b                          	popq	%rbx
140a64453: 5f                          	popq	%rdi
140a64454: 5e                          	popq	%rsi
140a64455: 41 5c                       	popq	%r12
140a64457: 41 5d                       	popq	%r13
140a64459: 41 5e                       	popq	%r14
140a6445b: 41 5f                       	popq	%r15
140a6445d: 5d                          	popq	%rbp
140a6445e: c3                          	retq
140a6445f: 90                          	nop
140a64460: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a64465: 55                          	pushq	%rbp
140a64466: 41 57                       	pushq	%r15
140a64468: 41 56                       	pushq	%r14
140a6446a: 41 55                       	pushq	%r13
140a6446c: 41 54                       	pushq	%r12
140a6446e: 56                          	pushq	%rsi
140a6446f: 57                          	pushq	%rdi
140a64470: 53                          	pushq	%rbx
140a64471: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a64478: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a6447f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
140a64485: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
140a6448b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
140a64491: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
140a64497: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
140a6449d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140a644a3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
140a644ac: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
140a644b5: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
140a644bd: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140a644c5: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
140a644cc: f0                          	lock
140a644cd: 48 ff 08                    	decq	(%rax)
140a644d0: 75 09                       	jne	0x140a644db <.text+0xa634db>
140a644d2: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a644d6: e8 f5 b4 be ff              	callq	0x14064f9d0 <.text+0x64e9d0>
140a644db: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140a644e3: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
140a644eb: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
140a644f4: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
140a644fd: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140a64503: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
140a64509: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
140a6450f: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
140a64515: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
140a6451b: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
140a64521: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a64528: 5b                          	popq	%rbx
140a64529: 5f                          	popq	%rdi
140a6452a: 5e                          	popq	%rsi
140a6452b: 41 5c                       	popq	%r12
140a6452d: 41 5d                       	popq	%r13
140a6452f: 41 5e                       	popq	%r14
140a64531: 41 5f                       	popq	%r15
140a64533: 5d                          	popq	%rbp
140a64534: c3                          	retq
140a64535: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a64540: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a64545: 55                          	pushq	%rbp
140a64546: 41 57                       	pushq	%r15
140a64548: 41 56                       	pushq	%r14
140a6454a: 41 55                       	pushq	%r13
140a6454c: 41 54                       	pushq	%r12
140a6454e: 56                          	pushq	%rsi
140a6454f: 57                          	pushq	%rdi
140a64550: 53                          	pushq	%rbx
140a64551: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a64558: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a6455f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
140a64565: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
140a6456b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
140a64571: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
140a64577: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
140a6457d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140a64583: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
140a6458c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
140a64595: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
140a6459d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140a645a5: 48 8b 85 28 01 00 00        	movq	0x128(%rbp), %rax
140a645ac: f0                          	lock
140a645ad: 48 ff 08                    	decq	(%rax)
140a645b0: 75 09                       	jne	0x140a645bb <.text+0xa635bb>
140a645b2: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a645b6: e8 15 b4 be ff              	callq	0x14064f9d0 <.text+0x64e9d0>
140a645bb: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140a645c3: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
140a645cb: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
140a645d4: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
140a645dd: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140a645e3: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
140a645e9: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
140a645ef: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
140a645f5: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
140a645fb: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
140a64601: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a64608: 5b                          	popq	%rbx
140a64609: 5f                          	popq	%rdi
140a6460a: 5e                          	popq	%rsi
140a6460b: 41 5c                       	popq	%r12
140a6460d: 41 5d                       	popq	%r13
140a6460f: 41 5e                       	popq	%r14
140a64611: 41 5f                       	popq	%r15
140a64613: 5d                          	popq	%rbp
140a64614: c3                          	retq
140a64615: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a64620: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a64625: 55                          	pushq	%rbp
140a64626: 41 57                       	pushq	%r15
140a64628: 41 56                       	pushq	%r14
140a6462a: 41 55                       	pushq	%r13
140a6462c: 41 54                       	pushq	%r12
140a6462e: 56                          	pushq	%rsi
140a6462f: 57                          	pushq	%rdi
140a64630: 53                          	pushq	%rbx
140a64631: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a64638: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a6463f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
140a64645: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
140a6464b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
140a64651: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
140a64657: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
140a6465d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140a64663: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
140a6466c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
140a64675: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
140a6467d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140a64685: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
140a64689: 48 85 d2                    	testq	%rdx, %rdx
140a6468c: 74 0f                       	je	0x140a6469d <.text+0xa6369d>
140a6468e: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140a64692: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a64698: e8 83 f5 ba ff              	callq	0x140613c20 <.text+0x612c20>
140a6469d: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140a646a5: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
140a646ad: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
140a646b6: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
140a646bf: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140a646c5: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
140a646cb: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
140a646d1: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
140a646d7: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
140a646dd: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
140a646e3: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a646ea: 5b                          	popq	%rbx
140a646eb: 5f                          	popq	%rdi
140a646ec: 5e                          	popq	%rsi
140a646ed: 41 5c                       	popq	%r12
140a646ef: 41 5d                       	popq	%r13
140a646f1: 41 5e                       	popq	%r14
140a646f3: 41 5f                       	popq	%r15
140a646f5: 5d                          	popq	%rbp
140a646f6: c3                          	retq
140a646f7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a64700: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a64705: 55                          	pushq	%rbp
140a64706: 41 57                       	pushq	%r15
140a64708: 41 56                       	pushq	%r14
140a6470a: 41 55                       	pushq	%r13
140a6470c: 41 54                       	pushq	%r12
140a6470e: 56                          	pushq	%rsi
140a6470f: 57                          	pushq	%rdi
140a64710: 53                          	pushq	%rbx
140a64711: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a64718: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a6471f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
140a64725: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
140a6472b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
140a64731: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
140a64737: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
140a6473d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140a64743: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
140a6474c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
140a64755: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
140a6475d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140a64765: 48 83 bd 70 01 00 00 00     	cmpq	$0x0, 0x170(%rbp)
140a6476d: 74 16                       	je	0x140a64785 <.text+0xa63785>
140a6476f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a64775: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
140a64779: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
140a64780: e8 9b f4 ba ff              	callq	0x140613c20 <.text+0x612c20>
140a64785: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140a6478d: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
140a64795: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
140a6479e: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
140a647a7: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140a647ad: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
140a647b3: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
140a647b9: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
140a647bf: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
140a647c5: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
140a647cb: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a647d2: 5b                          	popq	%rbx
140a647d3: 5f                          	popq	%rdi
140a647d4: 5e                          	popq	%rsi
140a647d5: 41 5c                       	popq	%r12
140a647d7: 41 5d                       	popq	%r13
140a647d9: 41 5e                       	popq	%r14
140a647db: 41 5f                       	popq	%r15
140a647dd: 5d                          	popq	%rbp
140a647de: c3                          	retq
