
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141227380: 3f                          	<unknown>
141227381: 09 d1                       	orl	%edx, %ecx
141227383: c1 e1 06                    	shll	$0x6, %ecx
141227386: 24 3f                       	andb	$0x3f, %al
141227388: 0f b6 c0                    	movzbl	%al, %eax
14122738b: 09 c8                       	orl	%ecx, %eax
14122738d: 83 f8 5c                    	cmpl	$0x5c, %eax
141227390: 74 2e                       	je	0x1412273c0 <.text+0x12263c0>
141227392: 83 f8 2f                    	cmpl	$0x2f, %eax
141227395: 74 29                       	je	0x1412273c0 <.text+0x12263c0>
141227397: 4c 39 ce                    	cmpq	%r9, %rsi
14122739a: 75 84                       	jne	0x141227320 <.text+0x1226320>
14122739c: bb 36 00 00 00              	movl	$0x36, %ebx
1412273a1: 4c 89 ce                    	movq	%r9, %rsi
1412273a4: eb 45                       	jmp	0x1412273eb <.text+0x12263eb>
1412273a6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1412273b0: 31 c0                       	xorl	%eax, %eax
1412273b2: e9 d1 00 00 00              	jmp	0x141227488 <.text+0x1226488>
1412273b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1412273c0: 4c 29 ce                    	subq	%r9, %rsi
1412273c3: 48 ff c6                    	incq	%rsi
1412273c6: 74 18                       	je	0x1412273e0 <.text+0x12263e0>
1412273c8: 48 83 fe 36                 	cmpq	$0x36, %rsi
1412273cc: 72 07                       	jb	0x1412273d5 <.text+0x12263d5>
1412273ce: 74 10                       	je	0x1412273e0 <.text+0x12263e0>
1412273d0: e9 fd 1e 00 00              	jmp	0x1412292d2 <.text+0x12282d2>
1412273d5: 42 80 3c 0e bf              	cmpb	$-0x41, (%rsi,%r9)
1412273da: 0f 8e f2 1e 00 00           	jle	0x1412292d2 <.text+0x12282d2>
1412273e0: bb 36 00 00 00              	movl	$0x36, %ebx
1412273e5: 48 29 f3                    	subq	%rsi, %rbx
1412273e8: 4c 01 ce                    	addq	%r9, %rsi
1412273eb: e8 40 0d 9f ff              	callq	0x140c18130 <.text+0xc17130>
1412273f0: 48 89 c1                    	movq	%rax, %rcx
1412273f3: 48 8b 00                    	movq	(%rax), %rax
1412273f6: 48 85 c0                    	testq	%rax, %rax
1412273f9: 74 15                       	je	0x141227410 <.text+0x1226410>
1412273fb: 83 f8 01                    	cmpl	$0x1, %eax
1412273fe: 0f 85 81 1e 00 00           	jne	0x141229285 <.text+0x1228285>
141227404: 48 83 c1 08                 	addq	$0x8, %rcx
141227408: eb 19                       	jmp	0x141227423 <.text+0x1226423>
14122740a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141227410: 31 d2                       	xorl	%edx, %edx
141227412: e8 a9 a1 6c 01              	callq	0x1428f15c0 <.text+0x28f05c0>
141227417: 48 89 c1                    	movq	%rax, %rcx
14122741a: 48 85 c0                    	testq	%rax, %rax
14122741d: 0f 84 62 1e 00 00           	je	0x141229285 <.text+0x1228285>
141227423: 48 83 39 00                 	cmpq	$0x0, (%rcx)
141227427: 0f 85 cd 1b 00 00           	jne	0x141228ffa <.text+0x1227ffa>
14122742d: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
141227434: 48 89 8d c8 03 00 00        	movq	%rcx, 0x3c8(%rbp)
14122743b: 48 83 c1 08                 	addq	$0x8, %rcx
14122743f: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
141227444: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14122744d: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
141227456: 41 b8 0f 00 00 00           	movl	$0xf, %r8d
14122745c: 48 8d 15 25 62 9f 01        	leaq	0x19f6225(%rip), %rdx   # 0x142c1d688
141227463: 49 89 f1                    	movq	%rsi, %r9
141227466: e8 d5 f8 9e ff              	callq	0x140c16d40 <.text+0xc15d40>
14122746b: 48 8b 8d c8 03 00 00        	movq	0x3c8(%rbp), %rcx
141227472: 48 ff 01                    	incq	(%rcx)
141227475: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
14122747c: b8 01 00 00 00              	movl	$0x1, %eax
141227481: 48 8b 8d 90 03 00 00        	movq	0x390(%rbp), %rcx
141227488: 66 0f 7e f2                 	movd	%xmm6, %edx
14122748c: 89 95 ac 02 00 00           	movl	%edx, 0x2ac(%rbp)
141227492: 0f b7 d2                    	movzwl	%dx, %edx
141227495: 66 44 0f c5 c6 01           	pextrw	$0x1, %xmm6, %r8d
14122749b: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1412274a2: 4c 89 85 c0 02 00 00        	movq	%r8, 0x2c0(%rbp)
1412274a9: 4c 89 c0                    	movq	%r8, %rax
1412274ac: 48 89 95 40 03 00 00        	movq	%rdx, 0x340(%rbp)
1412274b3: 48 0f af c2                 	imulq	%rdx, %rax
1412274b7: 48 89 85 c8 03 00 00        	movq	%rax, 0x3c8(%rbp)
1412274be: 48 39 c1                    	cmpq	%rax, %rcx
1412274c1: 0f 82 df 1a 00 00           	jb	0x141228fa6 <.text+0x1227fa6>
1412274c7: 41 83 fc 02                 	cmpl	$0x2, %r12d
1412274cb: 0f 57 c9                    	xorps	%xmm1, %xmm1
1412274ce: f3 41 0f c2 8f 80 02 00 00 01       	cmpltss	0x280(%r15), %xmm1
1412274d8: 0f 93 c3                    	setae	%bl
1412274db: f3 0f 10 05 8d e3 6f 01     	movss	0x16fe38d(%rip), %xmm0  # 0x142925870
1412274e3: 0f 55 c8                    	andnps	%xmm0, %xmm1
1412274e6: 0f 29 8d f0 01 00 00        	movaps	%xmm1, 0x1f0(%rbp)
1412274ed: 66 41 0f 61 f5              	punpcklwd	%xmm13, %xmm6   # xmm6 = xmm6[0],xmm13[0],xmm6[1],xmm13[1],xmm6[2],xmm13[2],xmm6[3],xmm13[3]
1412274f2: 44 0f 5b ce                 	cvtdq2ps	%xmm6, %xmm9
1412274f6: 40 08 fb                    	orb	%dil, %bl
1412274f9: 44 0f 58 0d cf 71 70 01     	addps	0x17071cf(%rip), %xmm9  # 0x14292e6d0
141227501: f3 41 0f 16 c1              	movshdup	%xmm9, %xmm0            # xmm0 = xmm9[1,1,3,3]
141227506: 0f 29 85 a0 03 00 00        	movaps	%xmm0, 0x3a0(%rbp)
14122750d: 31 ff                       	xorl	%edi, %edi
14122750f: f3 44 0f 11 9d 9c 03 00 00  	movss	%xmm11, 0x39c(%rbp)
141227518: 0f 29 75 e0                 	movaps	%xmm6, -0x20(%rbp)
14122751c: f3 44 0f 11 85 88 03 00 00  	movss	%xmm8, 0x388(%rbp)
141227525: 44 0f 29 8d 90 02 00 00     	movaps	%xmm9, 0x290(%rbp)
14122752d: 48 3b bd 40 03 00 00        	cmpq	0x340(%rbp), %rdi
141227534: 0f 83 36 05 00 00           	jae	0x141227a70 <.text+0x1226a70>
14122753a: 0f 28 f7                    	movaps	%xmm7, %xmm6
14122753d: 0f 57 ff                    	xorps	%xmm7, %xmm7
141227540: f3 4c 0f 2a d7              	cvtsi2ss	%rdi, %xmm10
141227545: f3 45 0f 5e d1              	divss	%xmm9, %xmm10
14122754a: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
14122754e: f3 41 0f 5c c2              	subss	%xmm10, %xmm0
141227553: f3 41 0f 59 c0              	mulss	%xmm8, %xmm0
141227558: f3 44 0f 59 95 80 03 00 00  	mulss	0x380(%rbp), %xmm10
141227561: f3 44 0f 58 d0              	addss	%xmm0, %xmm10
141227566: f3 45 0f 59 d3              	mulss	%xmm11, %xmm10
14122756b: f3 41 0f 10 8f 80 02 00 00  	movss	0x280(%r15), %xmm1
141227574: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
141227578: e8 03 46 e1 fe              	callq	0x14003bb80 <.text+0x3ab80>
14122757d: 49 8b 87 98 00 00 00        	movq	0x98(%r15), %rax
141227584: 48 83 f8 01                 	cmpq	$0x1, %rax
141227588: 0f 84 93 18 00 00           	je	0x141228e21 <.text+0x1227e21>
14122758e: 48 85 c0                    	testq	%rax, %rax
141227591: 0f 84 d5 19 00 00           	je	0x141228f6c <.text+0x1227f6c>
141227597: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
14122759b: 44 0f 28 c9                 	movaps	%xmm1, %xmm9
14122759f: 49 8b 8f 90 00 00 00        	movq	0x90(%r15), %rcx
1412275a6: 48 89 8d 00 02 00 00        	movq	%rcx, 0x200(%rbp)
1412275ad: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1412275b4: 48 8d 05 85 f6 fd ff        	leaq	-0x2097b(%rip), %rax    # 0x141206c40 <.text+0x1205c40>
1412275bb: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1412275c2: 0f 28 c8                    	movaps	%xmm0, %xmm1
1412275c5: f3 41 0f 5f cf              	maxss	%xmm15, %xmm1
1412275ca: 48 8d 85 08 02 00 00        	leaq	0x208(%rbp), %rax
1412275d1: 0f 11 78 10                 	movups	%xmm7, 0x10(%rax)
1412275d5: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
1412275dc: e8 2f 06 fe ff              	callq	0x141207c10 <.text+0x1206c10>
1412275e1: 8b 85 20 02 00 00           	movl	0x220(%rbp), %eax
1412275e7: 4c 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %r9
1412275ee: 49 39 c1                    	cmpq	%rax, %r9
1412275f1: 0f 86 3e 1c 00 00           	jbe	0x141229235 <.text+0x1228235>
1412275f7: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
1412275fb: 4c 39 c9                    	cmpq	%r9, %rcx
1412275fe: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
141227605: 4c 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %r10
14122760c: 0f 83 3a 1c 00 00           	jae	0x14122924c <.text+0x122824c>
141227612: 48 39 c2                    	cmpq	%rax, %rdx
141227615: 0f 86 45 1c 00 00           	jbe	0x141229260 <.text+0x1228260>
14122761b: 48 39 d1                    	cmpq	%rdx, %rcx
14122761e: f3 0f 10 25 de e2 6f 01     	movss	0x16fe2de(%rip), %xmm4  # 0x142925904
141227626: 0f 83 48 1c 00 00           	jae	0x141229274 <.text+0x1228274>
14122762c: f3 0f 10 bd 24 02 00 00     	movss	0x224(%rbp), %xmm7
141227634: 4c 8d 04 40                 	leaq	(%rax,%rax,2), %r8
141227638: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
14122763f: f3 46 0f 10 74 82 04        	movss	0x4(%rdx,%r8,4), %xmm14
141227646: 4c 8d 0c 49                 	leaq	(%rcx,%rcx,2), %r9
14122764a: f3 46 0f 10 6c 8a 04        	movss	0x4(%rdx,%r9,4), %xmm13
141227651: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
141227658: f3 44 0f 10 24 82           	movss	(%rdx,%rax,4), %xmm12
14122765e: f3 44 0f 10 5c 82 04        	movss	0x4(%rdx,%rax,4), %xmm11
141227665: f3 0f 10 05 57 5f 86 01     	movss	0x1865f57(%rip), %xmm0  # 0x142a8d5c4
14122766d: f3 0f 11 85 d0 02 00 00     	movss	%xmm0, 0x2d0(%rbp)
141227675: f6 c3 01                    	testb	$0x1, %bl
141227678: 74 38                       	je	0x1412276b2 <.text+0x12266b2>
14122767a: 49 39 c2                    	cmpq	%rax, %r10
14122767d: 0f 86 13 1c 00 00           	jbe	0x141229296 <.text+0x1228296>
141227683: 4c 39 d1                    	cmpq	%r10, %rcx
141227686: 0f 83 21 1c 00 00           	jae	0x1412292ad <.text+0x12282ad>
14122768c: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
141227693: f3 0f 10 04 82              	movss	(%rdx,%rax,4), %xmm0
141227698: f3 0f 10 4c 82 04           	movss	0x4(%rdx,%rax,4), %xmm1
14122769e: f3 0f 5c c8                 	subss	%xmm0, %xmm1
1412276a2: f3 0f 59 cf                 	mulss	%xmm7, %xmm1
1412276a6: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1412276aa: f3 0f 11 8d d0 02 00 00     	movss	%xmm1, 0x2d0(%rbp)
1412276b2: 41 8b 87 58 02 00 00        	movl	0x258(%r15), %eax
1412276b9: 83 f8 02                    	cmpl	$0x2, %eax
1412276bc: 0f 83 8e 00 00 00           	jae	0x141227750 <.text+0x1226750>
1412276c2: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1412276c6: f3 41 0f 5e 4f 30           	divss	0x30(%r15), %xmm1
1412276cc: f3 0f 11 8d a8 02 00 00     	movss	%xmm1, 0x2a8(%rbp)
1412276d4: 41 0f 2e cf                 	ucomiss	%xmm15, %xmm1
1412276d8: 0f 82 2d 19 00 00           	jb	0x14122900b <.text+0x122800b>
1412276de: 0f 2e e1                    	ucomiss	%xmm1, %xmm4
1412276e1: 0f 82 24 19 00 00           	jb	0x14122900b <.text+0x122800b>
1412276e7: 0f 2e c9                    	ucomiss	%xmm1, %xmm1
1412276ea: 0f 8a 8c 19 00 00           	jp	0x14122907c <.text+0x122807c>
1412276f0: 85 c0                       	testl	%eax, %eax
1412276f2: 0f 84 ff 00 00 00           	je	0x1412277f7 <.text+0x12267f7>
1412276f8: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
1412276ff: 48 8b 41 10                 	movq	0x10(%rcx), %rax
141227703: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
14122770a: 0f 10 01                    	movups	(%rcx), %xmm0
14122770d: 0f 29 85 00 02 00 00        	movaps	%xmm0, 0x200(%rbp)
141227714: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
14122771b: e8 f0 26 a7 ff              	callq	0x140c99e10 <.text+0xc98e10>
141227720: f3 0f 11 85 50 03 00 00     	movss	%xmm0, 0x350(%rbp)
141227728: f3 0f 11 8d 54 03 00 00     	movss	%xmm1, 0x354(%rbp)
141227730: 48 8d 8d 84 02 00 00        	leaq	0x284(%rbp), %rcx
141227737: 48 8d 95 50 03 00 00        	leaq	0x350(%rbp), %rdx
14122773e: e8 7d 56 a7 ff              	callq	0x140c9cdc0 <.text+0xc9bdc0>
141227743: e9 f8 00 00 00              	jmp	0x141227840 <.text+0x1226840>
141227748: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141227750: 49 8b 97 98 00 00 00        	movq	0x98(%r15), %rdx
141227757: 48 39 d1                    	cmpq	%rdx, %rcx
14122775a: 0f 83 61 1b 00 00           	jae	0x1412292c1 <.text+0x12282c1>
141227760: 49 8b 87 90 00 00 00        	movq	0x90(%r15), %rax
141227767: f3 42 0f 10 44 88 08        	movss	0x8(%rax,%r9,4), %xmm0
14122776e: f3 42 0f 5c 44 80 08        	subss	0x8(%rax,%r8,4), %xmm0
141227775: f2 42 0f 10 0c 88           	movsd	(%rax,%r9,4), %xmm1
14122777b: f2 42 0f 10 14 80           	movsd	(%rax,%r8,4), %xmm2
141227781: 0f 5c ca                    	subps	%xmm2, %xmm1
141227784: 0f 28 d1                    	movaps	%xmm1, %xmm2
141227787: 0f 59 d1                    	mulps	%xmm1, %xmm2
14122778a: f3 0f 16 da                 	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
14122778e: f3 0f 58 da                 	addss	%xmm2, %xmm3
141227792: 0f 28 d0                    	movaps	%xmm0, %xmm2
141227795: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
141227799: f3 0f 58 d3                 	addss	%xmm3, %xmm2
14122779d: f3 0f 51 d2                 	sqrtss	%xmm2, %xmm2
1412277a1: 0f 28 dc                    	movaps	%xmm4, %xmm3
1412277a4: f3 0f 5e da                 	divss	%xmm2, %xmm3
1412277a8: f3 0f 59 cb                 	mulss	%xmm3, %xmm1
1412277ac: f3 0f 59 d8                 	mulss	%xmm0, %xmm3
1412277b0: 0f 28 c3                    	movaps	%xmm3, %xmm0
1412277b3: 0f 57 05 36 e1 6f 01        	xorps	0x16fe136(%rip), %xmm0  # 0x1429258f0
1412277ba: f3 0f 59 db                 	mulss	%xmm3, %xmm3
1412277be: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1412277c1: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1412277c5: f3 0f 58 cb                 	addss	%xmm3, %xmm1
1412277c9: f3 0f 51 c9                 	sqrtss	%xmm1, %xmm1
1412277cd: 0f 28 d4                    	movaps	%xmm4, %xmm2
1412277d0: f3 0f 5e d1                 	divss	%xmm1, %xmm2
1412277d4: f3 0f 12 ca                 	movsldup	%xmm2, %xmm1            # xmm1 = xmm2[0,0,2,2]
1412277d8: 0f 59 c8                    	mulps	%xmm0, %xmm1
1412277db: 0f 13 8d 00 02 00 00        	movlps	%xmm1, 0x200(%rbp)
1412277e2: 48 8d 8d 84 02 00 00        	leaq	0x284(%rbp), %rcx
1412277e9: 48 8d 95 00 02 00 00        	leaq	0x200(%rbp), %rdx
1412277f0: e8 cb 55 a7 ff              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1412277f5: eb 49                       	jmp	0x141227840 <.text+0x1226840>
1412277f7: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1412277fe: 0f 10 00                    	movups	(%rax), %xmm0
141227801: 0f 29 85 00 02 00 00        	movaps	%xmm0, 0x200(%rbp)
141227808: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
14122780f: e8 cc 00 a7 ff              	callq	0x140c978e0 <.text+0xc968e0>
141227814: f3 0f 11 85 50 03 00 00     	movss	%xmm0, 0x350(%rbp)
14122781c: f3 0f 11 8d 54 03 00 00     	movss	%xmm1, 0x354(%rbp)
141227824: 48 8d 8d 84 02 00 00        	leaq	0x284(%rbp), %rcx
14122782b: 48 8d 95 50 03 00 00        	leaq	0x350(%rbp), %rdx
141227832: e8 89 55 a7 ff              	callq	0x140c9cdc0 <.text+0xc9bdc0>
141227837: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141227840: 48 8d 77 01                 	leaq	0x1(%rdi), %rsi
141227844: f3 45 0f 5c ee              	subss	%xmm14, %xmm13
141227849: f3 44 0f 59 ef              	mulss	%xmm7, %xmm13
14122784e: f3 45 0f 58 ee              	addss	%xmm14, %xmm13
141227853: f3 45 0f 5c dc              	subss	%xmm12, %xmm11
141227858: f3 44 0f 59 df              	mulss	%xmm7, %xmm11
14122785d: f3 45 0f 58 dc              	addss	%xmm12, %xmm11
141227862: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
141227866: f3 41 0f 58 c3              	addss	%xmm11, %xmm0
14122786b: 45 0f 57 e4                 	xorps	%xmm12, %xmm12
14122786f: f3 45 0f c2 e3 01           	cmpltss	%xmm11, %xmm12
141227875: 41 0f 54 c4                 	andps	%xmm12, %xmm0
141227879: f3 0f 10 0d fb e0 6f 01     	movss	0x16fe0fb(%rip), %xmm1  # 0x14292597c
141227881: 44 0f 55 e1                 	andnps	%xmm1, %xmm12
141227885: 44 0f 56 e0                 	orps	%xmm0, %xmm12
141227889: 48 c1 e7 05                 	shlq	$0x5, %rdi
14122788d: 48 03 bd b8 03 00 00        	addq	0x3b8(%rbp), %rdi
141227894: f3 44 0f 10 9d d0 02 00 00  	movss	0x2d0(%rbp), %xmm11
14122789d: f3 44 0f 58 1d aa 79 85 01  	addss	0x18579aa(%rip), %xmm11 # 0x142a7f250
1412278a6: f3 0f 10 85 9c 03 00 00     	movss	0x39c(%rbp), %xmm0
1412278ae: f3 41 0f 5c c2              	subss	%xmm10, %xmm0
1412278b3: 45 0f 28 f2                 	movaps	%xmm10, %xmm14
1412278b7: f3 45 0f c2 f2 03           	cmpunordss	%xmm10, %xmm14
1412278bd: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
1412278c1: 0f 54 c8                    	andps	%xmm0, %xmm1
1412278c4: f3 41 0f 5d c2              	minss	%xmm10, %xmm0
1412278c9: 44 0f 55 f0                 	andnps	%xmm0, %xmm14
1412278cd: 44 0f 56 f1                 	orps	%xmm1, %xmm14
1412278d1: f3 44 0f 5c ad f0 01 00 00  	subss	0x1f0(%rbp), %xmm13
1412278da: 45 31 f6                    	xorl	%r14d, %r14d
1412278dd: 0f 28 fe                    	movaps	%xmm6, %xmm7
1412278e0: eb 70                       	jmp	0x141227952 <.text+0x1226952>
1412278e2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1412278f0: 0f 2e 8d d0 02 00 00        	ucomiss	0x2d0(%rbp), %xmm1
1412278f7: 0f 97 c0                    	seta	%al
1412278fa: 0f 28 d3                    	movaps	%xmm3, %xmm2
1412278fd: 44 0f 2e e9                 	ucomiss	%xmm1, %xmm13
141227901: 49 8d 4e 01                 	leaq	0x1(%r14), %rcx
141227905: 0f 97 c2                    	seta	%dl
141227908: 08 c2                       	orb	%al, %dl
14122790a: 4c 0f af b5 40 03 00 00     	imulq	0x340(%rbp), %r14
141227912: 49 c1 e6 05                 	shlq	$0x5, %r14
141227916: f3 42 0f 11 04 37           	movss	%xmm0, (%rdi,%r14)
14122791c: f3 42 0f 11 4c 37 04        	movss	%xmm1, 0x4(%rdi,%r14)
141227923: 42 c7 44 37 08 00 00 80 3f  	movl	$0x3f800000, 0x8(%rdi,%r14) # imm = 0x3F800000
14122792c: f3 42 0f 11 54 37 0c        	movss	%xmm2, 0xc(%rdi,%r14)
141227933: 48 8b 85 84 02 00 00        	movq	0x284(%rbp), %rax
14122793a: 4a 89 44 37 10              	movq	%rax, 0x10(%rdi,%r14)
14122793f: 8b 85 8c 02 00 00           	movl	0x28c(%rbp), %eax
141227945: 42 89 44 37 18              	movl	%eax, 0x18(%rdi,%r14)
14122794a: 42 88 54 37 1c              	movb	%dl, 0x1c(%rdi,%r14)
14122794f: 49 89 ce                    	movq	%rcx, %r14
141227952: 4c 3b b5 c0 02 00 00        	cmpq	0x2c0(%rbp), %r14
141227959: 0f 83 d1 00 00 00           	jae	0x141227a30 <.text+0x1226a30>
14122795f: 0f 57 d2                    	xorps	%xmm2, %xmm2
141227962: f3 49 0f 2a d6              	cvtsi2ss	%r14, %xmm2
141227967: f3 0f 5e 95 a0 03 00 00     	divss	0x3a0(%rbp), %xmm2
14122796f: f3 0f 59 d7                 	mulss	%xmm7, %xmm2
141227973: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
141227977: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
14122797b: e8 50 ac a6 ff              	callq	0x140c925d0 <.text+0xc915d0>
141227980: 0f 28 d1                    	movaps	%xmm1, %xmm2
141227983: f3 41 0f 5c d4              	subss	%xmm12, %xmm2
141227988: 80 bd 08 03 00 00 00        	cmpb	$0x0, 0x308(%rbp)
14122798f: 74 2f                       	je	0x1412279c0 <.text+0x12269c0>
141227991: f3 0f 10 9d 78 03 00 00     	movss	0x378(%rbp), %xmm3
141227999: f3 0f 5c d9                 	subss	%xmm1, %xmm3
14122799d: f3 0f 58 1d 5b df 6f 01     	addss	0x16fdf5b(%rip), %xmm3  # 0x142925900
1412279a5: 0f 28 e2                    	movaps	%xmm2, %xmm4
1412279a8: f3 0f c2 e2 03              	cmpunordss	%xmm2, %xmm4
1412279ad: 0f 28 ec                    	movaps	%xmm4, %xmm5
1412279b0: 0f 54 eb                    	andps	%xmm3, %xmm5
1412279b3: f3 0f 5d da                 	minss	%xmm2, %xmm3
1412279b7: 0f 55 e3                    	andnps	%xmm3, %xmm4
1412279ba: 0f 56 e5                    	orps	%xmm5, %xmm4
1412279bd: 0f 28 d4                    	movaps	%xmm4, %xmm2
1412279c0: f6 c3 01                    	testb	$0x1, %bl
1412279c3: 0f 28 fe                    	movaps	%xmm6, %xmm7
1412279c6: 74 58                       	je	0x141227a20 <.text+0x1226a20>
1412279c8: 41 0f 28 e3                 	movaps	%xmm11, %xmm4
1412279cc: f3 0f 5c e1                 	subss	%xmm1, %xmm4
1412279d0: 0f 28 da                    	movaps	%xmm2, %xmm3
1412279d3: f3 0f c2 da 03              	cmpunordss	%xmm2, %xmm3
1412279d8: 0f 28 eb                    	movaps	%xmm3, %xmm5
1412279db: 0f 54 ec                    	andps	%xmm4, %xmm5
1412279de: f3 0f 5d e2                 	minss	%xmm2, %xmm4
1412279e2: 0f 55 dc                    	andnps	%xmm4, %xmm3
1412279e5: 0f 56 dd                    	orps	%xmm5, %xmm3
1412279e8: 41 83 fc 02                 	cmpl	$0x2, %r12d
1412279ec: 0f 85 fe fe ff ff           	jne	0x1412278f0 <.text+0x12268f0>
1412279f2: 41 0f 28 d6                 	movaps	%xmm14, %xmm2
1412279f6: f3 0f 5d d3                 	minss	%xmm3, %xmm2
1412279fa: f3 0f c2 db 03              	cmpunordss	%xmm3, %xmm3
1412279ff: 0f 28 e3                    	movaps	%xmm3, %xmm4
141227a02: 41 0f 54 e6                 	andps	%xmm14, %xmm4
141227a06: 0f 55 da                    	andnps	%xmm2, %xmm3
141227a09: 0f 56 dc                    	orps	%xmm4, %xmm3
141227a0c: e9 df fe ff ff              	jmp	0x1412278f0 <.text+0x12268f0>
141227a11: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141227a20: 31 c0                       	xorl	%eax, %eax
141227a22: e9 d6 fe ff ff              	jmp	0x1412278fd <.text+0x12268fd>
141227a27: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141227a30: 48 89 f7                    	movq	%rsi, %rdi
141227a33: f3 44 0f 10 9d 9c 03 00 00  	movss	0x39c(%rbp), %xmm11
141227a3c: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
141227a40: f3 44 0f 10 25 bb de 6f 01  	movss	0x16fdebb(%rip), %xmm12 # 0x142925904
141227a49: f3 44 0f 10 35 26 56 7a 01  	movss	0x17a5626(%rip), %xmm14 # 0x1429cd078
141227a52: f3 44 0f 10 85 88 03 00 00  	movss	0x388(%rbp), %xmm8
141227a5b: 44 0f 28 8d 90 02 00 00     	movaps	0x290(%rbp), %xmm9
141227a63: 48 3b bd 40 03 00 00        	cmpq	0x340(%rbp), %rdi
141227a6a: 0f 82 ca fa ff ff           	jb	0x14122753a <.text+0x122653a>
141227a70: 4d 85 ed                    	testq	%r13, %r13
141227a73: 0f 84 e9 05 00 00           	je	0x141228062 <.text+0x1227062>
141227a79: f3 0f 10 9d 9c 03 00 00     	movss	0x39c(%rbp), %xmm3
141227a81: 0f 28 eb                    	movaps	%xmm3, %xmm5
141227a84: f3 41 0f 59 e8              	mulss	%xmm8, %xmm5
141227a89: 48 8b 85 00 05 00 00        	movq	0x500(%rbp), %rax
141227a90: f3 0f 10 80 80 02 00 00     	movss	0x280(%rax), %xmm0
141227a98: 0f 28 cb                    	movaps	%xmm3, %xmm1
141227a9b: f3 0f 10 a5 80 03 00 00     	movss	0x380(%rbp), %xmm4
141227aa3: f3 0f 59 cc                 	mulss	%xmm4, %xmm1
141227aa7: 0f 28 d7                    	movaps	%xmm7, %xmm2
141227aaa: f3 0f 58 d0                 	addss	%xmm0, %xmm2
141227aae: 0f 14 ca                    	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
141227ab1: 0f 29 6d 10                 	movaps	%xmm5, 0x10(%rbp)
141227ab5: 0f 14 e8                    	unpcklps	%xmm0, %xmm5            # xmm5 = xmm5[0],xmm0[0],xmm5[1],xmm0[1]
141227ab8: 0f 29 6d f0                 	movaps	%xmm5, -0x10(%rbp)
141227abc: 0f 5c cd                    	subps	%xmm5, %xmm1
141227abf: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
141227ac3: 0f 5e c1                    	divps	%xmm1, %xmm0
141227ac6: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
141227aca: f3 0f 5e bd a0 03 00 00     	divss	0x3a0(%rbp), %xmm7
141227ad2: f3 41 0f 5c e0              	subss	%xmm8, %xmm4
141227ad7: f3 0f 59 e3                 	mulss	%xmm3, %xmm4
141227adb: f3 41 0f 5e e1              	divss	%xmm9, %xmm4
141227ae0: f3 0f 11 a5 80 03 00 00     	movss	%xmm4, 0x380(%rbp)
141227ae8: 48 8b bd 40 03 00 00        	movq	0x340(%rbp), %rdi
141227aef: c1 e7 05                    	shll	$0x5, %edi
141227af2: 48 c7 85 58 01 00 00 00 00 00 00    	movq	$0x0, 0x158(%rbp)
141227afd: 4c 8b bd 48 03 00 00        	movq	0x348(%rbp), %r15
