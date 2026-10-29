
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409781d0: 55                          	pushq	%rbp
1409781d1: 41 57                       	pushq	%r15
1409781d3: 41 56                       	pushq	%r14
1409781d5: 41 55                       	pushq	%r13
1409781d7: 41 54                       	pushq	%r12
1409781d9: 56                          	pushq	%rsi
1409781da: 57                          	pushq	%rdi
1409781db: 53                          	pushq	%rbx
1409781dc: 48 81 ec f8 02 00 00        	subq	$0x2f8, %rsp            # imm = 0x2F8
1409781e3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409781eb: 44 0f 29 95 60 02 00 00     	movaps	%xmm10, 0x260(%rbp)
1409781f3: 44 0f 29 8d 50 02 00 00     	movaps	%xmm9, 0x250(%rbp)
1409781fb: 44 0f 29 85 40 02 00 00     	movaps	%xmm8, 0x240(%rbp)
140978203: 0f 29 bd 30 02 00 00        	movaps	%xmm7, 0x230(%rbp)
14097820a: 0f 29 b5 20 02 00 00        	movaps	%xmm6, 0x220(%rbp)
140978211: 48 c7 85 18 02 00 00 fe ff ff ff    	movq	$-0x2, 0x218(%rbp)
14097821c: 4c 89 8d 00 02 00 00        	movq	%r9, 0x200(%rbp)
140978223: 48 89 8d 08 02 00 00        	movq	%rcx, 0x208(%rbp)
14097822a: 48 8b 0a                    	movq	(%rdx), %rcx
14097822d: 83 39 01                    	cmpl	$0x1, (%rcx)
140978230: 0f 85 08 16 00 00           	jne	0x14097983e <.text+0x97883e>
140978236: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
14097823d: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
140978244: 4c 8b 8d e0 02 00 00        	movq	0x2e0(%rbp), %r9
14097824b: 4d 8b 11                    	movq	(%r9), %r10
14097824e: 4c 89 95 e8 01 00 00        	movq	%r10, 0x1e8(%rbp)
140978255: 4d 8b 51 20                 	movq	0x20(%r9), %r10
140978259: 4c 89 95 38 01 00 00        	movq	%r10, 0x138(%rbp)
140978260: 4d 8b 49 28                 	movq	0x28(%r9), %r9
140978264: 4c 89 4d f0                 	movq	%r9, -0x10(%rbp)
140978268: 4c 8b 32                    	movq	(%rdx), %r14
14097826b: 4c 8b 4a 10                 	movq	0x10(%rdx), %r9
14097826f: 4c 89 8d 70 01 00 00        	movq	%r9, 0x170(%rbp)
140978276: 44 8b 4a 1c                 	movl	0x1c(%rdx), %r9d
14097827a: 44 89 8d 78 01 00 00        	movl	%r9d, 0x178(%rbp)
140978281: 4c 8b 7a 20                 	movq	0x20(%rdx), %r15
140978285: 4c 8b 4a 30                 	movq	0x30(%rdx), %r9
140978289: 4c 89 8d 80 01 00 00        	movq	%r9, 0x180(%rbp)
140978290: 8b 52 3c                    	movl	0x3c(%rdx), %edx
140978293: 89 95 88 01 00 00           	movl	%edx, 0x188(%rbp)
140978299: 48 8b 10                    	movq	(%rax), %rdx
14097829c: 48 89 55 f8                 	movq	%rdx, -0x8(%rbp)
1409782a0: 4c 8b 69 08                 	movq	0x8(%rcx), %r13
1409782a4: 4d 8b 20                    	movq	(%r8), %r12
1409782a7: 4c 89 6d 20                 	movq	%r13, 0x20(%rbp)
1409782ab: 49 83 7c 24 18 00           	cmpq	$0x0, 0x18(%r12)
1409782b1: 48 8b 48 20                 	movq	0x20(%rax), %rcx
1409782b5: 48 89 8d 68 01 00 00        	movq	%rcx, 0x168(%rbp)
1409782bc: 48 8b 48 30                 	movq	0x30(%rax), %rcx
1409782c0: 48 89 4d 10                 	movq	%rcx, 0x10(%rbp)
1409782c4: 8b 48 3c                    	movl	0x3c(%rax), %ecx
1409782c7: 89 8d 5c 01 00 00           	movl	%ecx, 0x15c(%rbp)
1409782cd: 48 8b 48 40                 	movq	0x40(%rax), %rcx
1409782d1: 48 89 8d f0 01 00 00        	movq	%rcx, 0x1f0(%rbp)
1409782d8: 48 8b 48 50                 	movq	0x50(%rax), %rcx
1409782dc: 48 89 4d 08                 	movq	%rcx, 0x8(%rbp)
1409782e0: 8b 40 5c                    	movl	0x5c(%rax), %eax
1409782e3: 89 85 58 01 00 00           	movl	%eax, 0x158(%rbp)
1409782e9: 48 8b bd 38 03 00 00        	movq	0x338(%rbp), %rdi
1409782f0: 48 8b 47 20                 	movq	0x20(%rdi), %rax
1409782f4: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
1409782fb: 48 8b 47 30                 	movq	0x30(%rdi), %rax
1409782ff: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140978306: 8b 47 3c                    	movl	0x3c(%rdi), %eax
140978309: 89 85 d4 01 00 00           	movl	%eax, 0x1d4(%rbp)
14097830f: 48 8b 47 40                 	movq	0x40(%rdi), %rax
140978313: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14097831a: 48 8b 47 50                 	movq	0x50(%rdi), %rax
14097831e: 48 89 45 00                 	movq	%rax, (%rbp)
140978322: 8b 47 5c                    	movl	0x5c(%rdi), %eax
140978325: 89 85 54 01 00 00           	movl	%eax, 0x154(%rbp)
14097832b: 0f 84 24 05 00 00           	je	0x140978855 <.text+0x977855>
140978331: 48 8b 9d 20 03 00 00        	movq	0x320(%rbp), %rbx
140978338: 48 8b b5 18 03 00 00        	movq	0x318(%rbp), %rsi
14097833f: 49 8d 4c 24 20              	leaq	0x20(%r12), %rcx
140978344: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
140978348: e8 23 8a c3 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14097834d: 49 8b 0c 24                 	movq	(%r12), %rcx
140978351: 49 8b 54 24 08              	movq	0x8(%r12), %rdx
140978356: 49 89 d0                    	movq	%rdx, %r8
140978359: 49 21 c0                    	andq	%rax, %r8
14097835c: 48 c1 e8 39                 	shrq	$0x39, %rax
140978360: 66 0f 6e c0                 	movd	%eax, %xmm0
140978364: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140978368: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14097836d: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140978372: 48 8d 81 58 fd ff ff        	leaq	-0x2a8(%rcx), %rax
140978379: 45 31 c9                    	xorl	%r9d, %r9d
14097837c: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140978380: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
140978386: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14097838a: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14097838e: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
140978393: 45 85 d2                    	testl	%r10d, %r10d
140978396: 74 28                       	je	0x1409783c0 <.text+0x9773c0>
140978398: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14097839d: 4d 01 c3                    	addq	%r8, %r11
1409783a0: 49 21 d3                    	andq	%rdx, %r11
1409783a3: 49 f7 db                    	negq	%r11
1409783a6: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
1409783ad: 4e 39 2c 18                 	cmpq	%r13, (%rax,%r11)
1409783b1: 74 2f                       	je	0x1409783e2 <.text+0x9773e2>
1409783b3: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1409783b7: 66 45 21 d3                 	andw	%r10w, %r11w
1409783bb: 45 89 da                    	movl	%r11d, %r10d
1409783be: 75 d8                       	jne	0x140978398 <.text+0x977398>
1409783c0: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1409783c4: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1409783c9: 45 85 d2                    	testl	%r10d, %r10d
1409783cc: 0f 85 83 04 00 00           	jne	0x140978855 <.text+0x977855>
1409783d2: 4d 01 c8                    	addq	%r9, %r8
1409783d5: 49 83 c0 10                 	addq	$0x10, %r8
1409783d9: 49 83 c1 10                 	addq	$0x10, %r9
1409783dd: 49 21 d0                    	andq	%rdx, %r8
1409783e0: eb 9e                       	jmp	0x140978380 <.text+0x977380>
1409783e2: 4a 83 bc 19 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rcx,%r11)
1409783eb: 0f 82 87 14 00 00           	jb	0x140979878 <.text+0x978878>
1409783f1: f3 42 0f 10 84 19 90 fd ff ff       	movss	-0x270(%rcx,%r11), %xmm0
1409783fb: 66 0f ef c9                 	pxor	%xmm1, %xmm1
1409783ff: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140978402: 0f 86 70 14 00 00           	jbe	0x140979878 <.text+0x978878>
140978408: 4e 8d 24 19                 	leaq	(%rcx,%r11), %r12
14097840c: 41 83 7c 24 b8 02           	cmpl	$0x2, -0x48(%r12)
140978412: 0f 82 26 14 00 00           	jb	0x14097983e <.text+0x97883e>
140978418: 4c 89 ad e0 01 00 00        	movq	%r13, 0x1e0(%rbp)
14097841f: 4e 8d 2c 19                 	leaq	(%rcx,%r11), %r13
140978423: 49 81 c5 60 fd ff ff        	addq	$-0x2a0, %r13           # imm = 0xFD60
14097842a: f3 0f 10 0d 3e d4 fa 01     	movss	0x1fad43e(%rip), %xmm1  # 0x142925870
140978432: 4c 89 e9                    	movq	%r13, %rcx
140978435: e8 a6 f8 ae 00              	callq	0x141467ce0 <.text+0x1466ce0>
14097843a: 89 c1                       	movl	%eax, %ecx
14097843c: 49 8b 94 24 70 fd ff ff     	movq	-0x290(%r12), %rdx
140978444: 48 39 ca                    	cmpq	%rcx, %rdx
140978447: 0f 86 67 14 00 00           	jbe	0x1409798b4 <.text+0x9788b4>
14097844d: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
140978451: 48 39 d0                    	cmpq	%rdx, %rax
140978454: 0f 83 66 14 00 00           	jae	0x1409798c0 <.text+0x9788c0>
14097845a: 49 8b 84 24 68 fd ff ff     	movq	-0x298(%r12), %rax
140978462: f3 44 0f 10 05 99 d4 fa 01  	movss	0x1fad499(%rip), %xmm8  # 0x142925904
14097846b: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14097846f: f3 0f 5c c8                 	subss	%xmm0, %xmm1
140978473: f2 0f 10 14 c8              	movsd	(%rax,%rcx,8), %xmm2
140978478: f2 0f 10 5c c8 08           	movsd	0x8(%rax,%rcx,8), %xmm3
14097847e: f3 0f 12 c9                 	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
140978482: 0f 59 ca                    	mulps	%xmm2, %xmm1
140978485: f3 0f 12 f8                 	movsldup	%xmm0, %xmm7            # xmm7 = xmm0[0,0,2,2]
140978489: 0f 59 fb                    	mulps	%xmm3, %xmm7
14097848c: 0f 58 f9                    	addps	%xmm1, %xmm7
14097848f: 0f 5c da                    	subps	%xmm2, %xmm3
140978492: f3 0f 16 c3                 	movshdup	%xmm3, %xmm0            # xmm0 = xmm3[1,1,3,3]
140978496: 0f 28 35 53 d4 fa 01        	movaps	0x1fad453(%rip), %xmm6  # 0x1429258f0
14097849d: 0f 57 c6                    	xorps	%xmm6, %xmm0
1409784a0: 0f 14 c3                    	unpcklps	%xmm3, %xmm0            # xmm0 = xmm0[0],xmm3[0],xmm0[1],xmm3[1]
1409784a3: 0f 59 db                    	mulps	%xmm3, %xmm3
1409784a6: f3 0f 16 cb                 	movshdup	%xmm3, %xmm1            # xmm1 = xmm3[1,1,3,3]
1409784aa: f3 0f 58 cb                 	addss	%xmm3, %xmm1
1409784ae: f3 0f 51 c9                 	sqrtss	%xmm1, %xmm1
1409784b2: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1409784b6: f3 0f 5e d1                 	divss	%xmm1, %xmm2
1409784ba: f3 0f 12 ca                 	movsldup	%xmm2, %xmm1            # xmm1 = xmm2[0,0,2,2]
1409784be: 0f 59 c1                    	mulps	%xmm1, %xmm0
1409784c1: 0f 13 85 30 01 00 00        	movlps	%xmm0, 0x130(%rbp)
1409784c8: 48 8b 13                    	movq	(%rbx), %rdx
1409784cb: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
1409784cf: e8 ac e8 31 00              	callq	0x140c96d80 <.text+0xc95d80>
1409784d4: f3 0f 10 45 20              	movss	0x20(%rbp), %xmm0
1409784d9: f3 0f 10 4d 28              	movss	0x28(%rbp), %xmm1
1409784de: 0f 28 d0                    	movaps	%xmm0, %xmm2
1409784e1: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
1409784e5: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1409784e8: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1409784ec: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1409784f0: f3 0f 51 c9                 	sqrtss	%xmm1, %xmm1
1409784f4: f3 44 0f 5e c1              	divss	%xmm1, %xmm8
1409784f9: f2 0f 10 95 30 01 00 00     	movsd	0x130(%rbp), %xmm2
140978501: f3 41 0f 12 c8              	movsldup	%xmm8, %xmm1            # xmm1 = xmm8[0,0,2,2]
140978506: 0f 59 c8                    	mulps	%xmm0, %xmm1
140978509: 0f 59 ca                    	mulps	%xmm2, %xmm1
14097850c: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
140978510: f3 0f 58 c1                 	addss	%xmm1, %xmm0
140978514: 0f 57 c9                    	xorps	%xmm1, %xmm1
140978517: 31 c0                       	xorl	%eax, %eax
140978519: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
14097851c: 0f 97 c0                    	seta	%al
14097851f: 0f 57 f2                    	xorps	%xmm2, %xmm6
140978522: 66 0f 6e c0                 	movd	%eax, %xmm0
140978526: 66 0f 70 c8 50              	pshufd	$0x50, %xmm0, %xmm1     # xmm1 = xmm0[0,0,1,1]
14097852b: 66 0f 72 f1 1f              	pslld	$0x1f, %xmm1
140978530: 66 0f 72 e1 1f              	psrad	$0x1f, %xmm1
140978535: 0f 54 d1                    	andps	%xmm1, %xmm2
140978538: 0f 55 ce                    	andnps	%xmm6, %xmm1
14097853b: 0f 56 ca                    	orps	%xmm2, %xmm1
14097853e: 0f 58 cf                    	addps	%xmm7, %xmm1
140978541: 0f 13 4d e8                 	movlps	%xmm1, -0x18(%rbp)
140978545: 48 8b 0e                    	movq	(%rsi), %rcx
140978548: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14097854c: e8 ef 10 07 00              	callq	0x1409e9640 <.text+0x9e8640>
140978551: 66 0f 6f f0                 	movdqa	%xmm0, %xmm6
140978555: 4c 89 a5 f8 01 00 00        	movq	%r12, 0x1f8(%rbp)
14097855c: f3 41 0f 10 44 24 e0        	movss	-0x20(%r12), %xmm0
140978563: 66 0f 6f d6                 	movdqa	%xmm6, %xmm2
140978567: f3 0f c2 d6 03              	cmpunordss	%xmm6, %xmm2
14097856c: 0f 28 ca                    	movaps	%xmm2, %xmm1
14097856f: 0f 54 d0                    	andps	%xmm0, %xmm2
140978572: f3 0f 5f c6                 	maxss	%xmm6, %xmm0
140978576: 0f 55 c8                    	andnps	%xmm0, %xmm1
140978579: 0f 56 d1                    	orps	%xmm1, %xmm2
14097857c: 48 8d 8d 48 01 00 00        	leaq	0x148(%rbp), %rcx
140978583: 48 8d 55 e8                 	leaq	-0x18(%rbp), %rdx
140978587: e8 54 48 32 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14097858c: 48 8d 75 20                 	leaq	0x20(%rbp), %rsi
140978590: 48 8d 95 30 01 00 00        	leaq	0x130(%rbp), %rdx
140978597: 48 89 f1                    	movq	%rsi, %rcx
14097859a: e8 21 48 32 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
14097859f: 48 8d 5d c0                 	leaq	-0x40(%rbp), %rbx
1409785a3: 48 89 d9                    	movq	%rbx, %rcx
1409785a6: 48 89 f2                    	movq	%rsi, %rdx
1409785a9: e8 22 8c 31 00              	callq	0x140c911d0 <.text+0xc901d0>
1409785ae: 48 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %rax
1409785b5: 4c 8b 08                    	movq	(%rax), %r9
1409785b8: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
1409785bf: 48 8b 00                    	movq	(%rax), %rax
1409785c2: 8b 8d 50 01 00 00           	movl	0x150(%rbp), %ecx
1409785c8: 89 4d 28                    	movl	%ecx, 0x28(%rbp)
1409785cb: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
1409785d2: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
1409785d6: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1409785db: f3 0f 11 74 24 30           	movss	%xmm6, 0x30(%rsp)
1409785e1: 4c 89 6c 24 28              	movq	%r13, 0x28(%rsp)
1409785e6: 4c 8b a5 e0 01 00 00        	movq	0x1e0(%rbp), %r12
1409785ed: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
1409785f2: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
1409785f6: 48 89 da                    	movq	%rbx, %rdx
1409785f9: 4c 8b 85 10 03 00 00        	movq	0x310(%rbp), %r8
140978600: e8 7b 4c 00 00              	callq	0x14097d280 <.text+0x97c280>
140978605: b1 02                       	movb	$0x2, %cl
140978607: 80 78 18 02                 	cmpb	$0x2, 0x18(%rax)
14097860b: 72 0a                       	jb	0x140978617 <.text+0x977617>
14097860d: 48 39 50 38                 	cmpq	%rdx, 0x38(%rax)
140978611: 75 04                       	jne	0x140978617 <.text+0x977617>
140978613: 0f b6 48 19                 	movzbl	0x19(%rax), %ecx
140978617: 80 c1 fe                    	addb	$-0x2, %cl
14097861a: 0f b6 c1                    	movzbl	%cl, %eax
14097861d: 3c 03                       	cmpb	$0x3, %al
14097861f: b9 03 00 00 00              	movl	$0x3, %ecx
140978624: 0f 42 c8                    	cmovbl	%eax, %ecx
140978627: 0f b6 c1                    	movzbl	%cl, %eax
14097862a: 48 8d 0d d3 ee 13 02        	leaq	0x213eed3(%rip), %rcx   # 0x142ab7504
140978631: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
140978635: 48 01 c8                    	addq	%rcx, %rax
140978638: ff e0                       	jmpq	*%rax
14097863a: 48 8b b5 80 01 00 00        	movq	0x180(%rbp), %rsi
140978641: 8b bd 88 01 00 00           	movl	0x188(%rbp), %edi
140978647: 89 3e                       	movl	%edi, (%rsi)
140978649: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
140978651: 4c 89 65 28                 	movq	%r12, 0x28(%rbp)
140978655: 48 c7 45 30 01 00 00 00     	movq	$0x1, 0x30(%rbp)
14097865d: 48 8d 05 b4 e4 13 02        	leaq	0x213e4b4(%rip), %rax   # 0x142ab6b18
140978664: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140978669: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
140978670: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140978675: 48 c7 44 24 30 12 00 00 00  	movq	$0x12, 0x30(%rsp)
14097867e: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
140978682: 41 b8 02 00 00 00           	movl	$0x2, %r8d
140978688: 4c 89 f9                    	movq	%r15, %rcx
14097868b: 4d 89 f1                    	movq	%r14, %r9
14097868e: e8 5d f1 a6 00              	callq	0x1413e77f0 <.text+0x13e67f0>
140978693: 89 3e                       	movl	%edi, (%rsi)
140978695: 4c 89 f9                    	movq	%r15, %rcx
140978698: 4c 89 e2                    	movq	%r12, %rdx
14097869b: 4d 89 f0                    	movq	%r14, %r8
14097869e: e8 5d 35 1b 00              	callq	0x140b2bc00 <.text+0xb2ac00>
1409786a3: 89 3e                       	movl	%edi, (%rsi)
1409786a5: 49 83 c7 30                 	addq	$0x30, %r15
1409786a9: 49 8d 86 90 00 00 00        	leaq	0x90(%r14), %rax
1409786b0: 49 8b 8e 90 00 00 00        	movq	0x90(%r14), %rcx
1409786b7: 48 85 c9                    	testq	%rcx, %rcx
1409786ba: 48 0f 44 c1                 	cmoveq	%rcx, %rax
1409786be: 48 c7 45 20 01 00 00 00     	movq	$0x1, 0x20(%rbp)
1409786c6: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
1409786ca: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
1409786d2: 48 c7 45 58 00 00 00 00     	movq	$0x0, 0x58(%rbp)
1409786da: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
1409786de: 4c 89 fa                    	movq	%r15, %rdx
1409786e1: e8 9a ab 91 00              	callq	0x141293280 <.text+0x1292280>
1409786e6: 48 c7 85 90 01 00 00 00 00 00 00    	movq	$0x0, 0x190(%rbp)
1409786f1: 48 c7 85 98 01 00 00 04 00 00 00    	movq	$0x4, 0x198(%rbp)
1409786fc: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
140978707: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14097870e: 8b 8d 78 01 00 00           	movl	0x178(%rbp), %ecx
140978714: 89 08                       	movl	%ecx, (%rax)
140978716: c6 85 16 02 00 00 01        	movb	$0x1, 0x216(%rbp)
14097871d: 4c 89 f1                    	movq	%r14, %rcx
140978720: 4c 89 e2                    	movq	%r12, %rdx
140978723: e8 f8 5c 0a 00              	callq	0x140a1e420 <.text+0xa1d420>
140978728: 48 85 c0                    	testq	%rax, %rax
14097872b: 4c 8b a5 f8 01 00 00        	movq	0x1f8(%rbp), %r12
140978732: 0f 84 55 02 00 00           	je	0x14097898d <.text+0x97798d>
140978738: 48 8b 70 30                 	movq	0x30(%rax), %rsi
14097873c: 48 8b 78 48                 	movq	0x48(%rax), %rdi
140978740: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
140978744: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140978748: f7 d0                       	notl	%eax
14097874a: 4c 8d 7e 10                 	leaq	0x10(%rsi), %r15
14097874e: 45 31 ed                    	xorl	%r13d, %r13d
140978751: eb 3b                       	jmp	0x14097878e <.text+0x97778e>
140978753: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140978760: 41 0f 14 f0                 	unpcklps	%xmm8, %xmm6            # xmm6 = xmm6[0],xmm8[0],xmm6[1],xmm8[1]
140978764: 0f 5c f7                    	subps	%xmm7, %xmm6
140978767: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
14097876e: 4b 8d 0c 76                 	leaq	(%r14,%r14,2), %rcx
140978772: 44 89 24 88                 	movl	%r12d, (%rax,%rcx,4)
140978776: 0f 13 74 88 04              	movlps	%xmm6, 0x4(%rax,%rcx,4)
14097877b: 49 ff c6                    	incq	%r14
14097877e: 4c 89 b5 a0 01 00 00        	movq	%r14, 0x1a0(%rbp)
140978785: 89 d8                       	movl	%ebx, %eax
140978787: 4c 8b a5 f8 01 00 00        	movq	0x1f8(%rbp), %r12
14097878e: 48 85 ff                    	testq	%rdi, %rdi
140978791: 0f 84 f6 01 00 00           	je	0x14097898d <.text+0x97798d>
140978797: 66 85 c0                    	testw	%ax, %ax
14097879a: 74 14                       	je	0x1409787b0 <.text+0x9777b0>
14097879c: 8d 58 ff                    	leal	-0x1(%rax), %ebx
14097879f: 21 c3                       	andl	%eax, %ebx
1409787a1: eb 35                       	jmp	0x1409787d8 <.text+0x9777d8>
1409787a3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1409787b0: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
1409787b5: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1409787b9: 48 81 c6 00 fb ff ff        	addq	$-0x500, %rsi           # imm = 0xFB00
1409787c0: 49 83 c7 10                 	addq	$0x10, %r15
1409787c4: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1409787c9: 74 e5                       	je	0x1409787b0 <.text+0x9777b0>
1409787cb: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1409787d0: 29 c1                       	subl	%eax, %ecx
1409787d2: f7 d0                       	notl	%eax
1409787d4: 89 c3                       	movl	%eax, %ebx
1409787d6: 21 cb                       	andl	%ecx, %ebx
1409787d8: 48 ff cf                    	decq	%rdi
1409787db: f3 0f bc c0                 	tzcntl	%eax, %eax
1409787df: 48 f7 d8                    	negq	%rax
1409787e2: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
1409787e6: 48 c1 e0 04                 	shlq	$0x4, %rax
1409787ea: 0f b6 4c 06 da              	movzbl	-0x26(%rsi,%rax), %ecx
1409787ef: 8d 51 fe                    	leal	-0x2(%rcx), %edx
1409787f2: 48 ff c9                    	decq	%rcx
1409787f5: 80 fa 03                    	cmpb	$0x3, %dl
1409787f8: 49 0f 43 cd                 	cmovaeq	%r13, %rcx
1409787fc: 48 85 c9                    	testq	%rcx, %rcx
1409787ff: 74 0f                       	je	0x140978810 <.text+0x977810>
140978801: 89 d8                       	movl	%ebx, %eax
140978803: 48 83 f9 01                 	cmpq	$0x1, %rcx
140978807: 74 85                       	je	0x14097878e <.text+0x97778e>
140978809: e9 82 10 00 00              	jmp	0x140979890 <.text+0x978890>
14097880e: 66 90                       	nop
140978810: 48 01 f0                    	addq	%rsi, %rax
140978813: 44 8b 60 b0                 	movl	-0x50(%rax), %r12d
140978817: f3 44 0f 10 40 d4           	movss	-0x2c(%rax), %xmm8
14097881d: f3 0f 10 70 cc              	movss	-0x34(%rax), %xmm6
140978822: 4c 8b b5 a0 01 00 00        	movq	0x1a0(%rbp), %r14
140978829: 4c 3b b5 90 01 00 00        	cmpq	0x190(%rbp), %r14
140978830: 0f 85 2a ff ff ff           	jne	0x140978760 <.text+0x977760>
140978836: c6 85 16 02 00 00 01        	movb	$0x1, 0x216(%rbp)
14097883d: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
140978844: 48 8d 15 e5 e2 13 02        	leaq	0x213e2e5(%rip), %rdx   # 0x142ab6b30
14097884b: e8 a0 a4 be ff              	callq	0x140562cf0 <.text+0x561cf0>
140978850: e9 0b ff ff ff              	jmp	0x140978760 <.text+0x977760>
140978855: 48 8d 0d a4 e2 13 02        	leaq	0x213e2a4(%rip), %rcx   # 0x142ab6b00
14097885c: e8 af 0a f6 01              	callq	0x1428d9310 <.text+0x28d8310>
140978861: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
140978868: 4c 8b 30                    	movq	(%rax), %r14
14097886b: 4c 89 65 20                 	movq	%r12, 0x20(%rbp)
14097886f: 49 83 7e 18 00              	cmpq	$0x0, 0x18(%r14)
140978874: 0f 84 cd 05 00 00           	je	0x140978e47 <.text+0x977e47>
14097887a: 49 8d 4e 20                 	leaq	0x20(%r14), %rcx
14097887e: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
140978882: e8 e9 84 c3 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140978887: 49 8b 1e                    	movq	(%r14), %rbx
14097888a: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
14097888e: 48 89 ca                    	movq	%rcx, %rdx
140978891: 48 21 c2                    	andq	%rax, %rdx
140978894: 48 c1 e8 39                 	shrq	$0x39, %rax
140978898: 66 0f 6e c0                 	movd	%eax, %xmm0
14097889c: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1409788a0: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1409788a5: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1409788aa: 48 8d 43 88                 	leaq	-0x78(%rbx), %rax
1409788ae: 45 31 c0                    	xorl	%r8d, %r8d
1409788b1: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1409788b5: f3 0f 6f 14 13              	movdqu	(%rbx,%rdx), %xmm2
1409788ba: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1409788be: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1409788c2: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1409788c7: 45 85 d2                    	testl	%r10d, %r10d
1409788ca: 74 29                       	je	0x1409788f5 <.text+0x9778f5>
1409788cc: f3 45 0f bc ca              	tzcntl	%r10d, %r9d
1409788d1: 49 01 d1                    	addq	%rdx, %r9
1409788d4: 49 21 c9                    	andq	%rcx, %r9
1409788d7: 49 f7 d9                    	negq	%r9
1409788da: 4d 6b c9 78                 	imulq	$0x78, %r9, %r9
1409788de: 4e 39 24 08                 	cmpq	%r12, (%rax,%r9)
1409788e2: 0f 84 69 01 00 00           	je	0x140978a51 <.text+0x977a51>
1409788e8: 45 8d 4a ff                 	leal	-0x1(%r10), %r9d
1409788ec: 66 45 21 d1                 	andw	%r10w, %r9w
1409788f0: 45 89 ca                    	movl	%r9d, %r10d
1409788f3: 75 d7                       	jne	0x1409788cc <.text+0x9778cc>
1409788f5: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1409788f9: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1409788fe: 45 85 c9                    	testl	%r9d, %r9d
140978901: 0f 85 40 05 00 00           	jne	0x140978e47 <.text+0x977e47>
140978907: 4c 01 c2                    	addq	%r8, %rdx
14097890a: 48 83 c2 10                 	addq	$0x10, %rdx
14097890e: 49 83 c0 10                 	addq	$0x10, %r8
140978912: 48 21 ca                    	andq	%rcx, %rdx
140978915: eb 9e                       	jmp	0x1409788b5 <.text+0x9778b5>
140978917: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
14097891e: 48 8b 58 10                 	movq	0x10(%rax), %rbx
140978922: 31 f6                       	xorl	%esi, %esi
140978924: b9 0c 00 00 00              	movl	$0xc, %ecx
140978929: 48 89 d8                    	movq	%rbx, %rax
14097892c: 48 f7 e1                    	mulq	%rcx
14097892f: 49 89 c6                    	movq	%rax, %r14
140978932: 0f 80 97 0f 00 00           	jo	0x1409798cf <.text+0x9788cf>
140978938: 49 bd fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %r13 # imm = 0x7FFFFFFFFFFFFFFC
140978942: 4d 39 ee                    	cmpq	%r13, %r14
140978945: 0f 87 84 0f 00 00           	ja	0x1409798cf <.text+0x9788cf>
14097894b: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
140978952: 4c 8b 78 08                 	movq	0x8(%rax), %r15
140978956: 4d 85 f6                    	testq	%r14, %r14
140978959: 0f 84 16 02 00 00           	je	0x140978b75 <.text+0x977b75>
14097895f: 0f b6 05 a3 c2 cb 02        	movzbl	0x2cbc2a3(%rip), %eax   # 0x143634c09
140978966: be 04 00 00 00              	movl	$0x4, %esi
14097896b: ba 04 00 00 00              	movl	$0x4, %edx
140978970: 4c 89 f1                    	movq	%r14, %rcx
140978973: e8 98 b2 c9 ff              	callq	0x140613c10 <.text+0x612c10>
140978978: 48 85 c0                    	testq	%rax, %rax
14097897b: 0f 84 4e 0f 00 00           	je	0x1409798cf <.text+0x9788cf>
140978981: 48 89 9d 00 02 00 00        	movq	%rbx, 0x200(%rbp)
140978988: e9 f8 01 00 00              	jmp	0x140978b85 <.text+0x977b85>
14097898d: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
140978994: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140978998: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
14097899f: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
1409789a3: 48 8b b5 08 02 00 00        	movq	0x208(%rbp), %rsi
1409789aa: 48 8b 06                    	movq	(%rsi), %rax
1409789ad: 48 85 c0                    	testq	%rax, %rax
1409789b0: 74 17                       	je	0x1409789c9 <.text+0x9779c9>
1409789b2: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
1409789b6: 48 c1 e0 02                 	shlq	$0x2, %rax
1409789ba: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409789be: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409789c4: e8 57 b2 c9 ff              	callq	0x140613c20 <.text+0x612c20>
1409789c9: 48 8b 45 30                 	movq	0x30(%rbp), %rax
1409789cd: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1409789d1: 0f 28 45 20                 	movaps	0x20(%rbp), %xmm0
1409789d5: 0f 11 06                    	movups	%xmm0, (%rsi)
1409789d8: 49 8b bc 24 70 fd ff ff     	movq	-0x290(%r12), %rdi
1409789e0: 48 8d 1c fd 00 00 00 00     	leaq	(,%rdi,8), %rbx
1409789e8: 45 31 ff                    	xorl	%r15d, %r15d
1409789eb: 48 89 f8                    	movq	%rdi, %rax
1409789ee: 48 c1 e8 3d                 	shrq	$0x3d, %rax
1409789f2: 0f 85 e9 0e 00 00           	jne	0x1409798e1 <.text+0x9788e1>
1409789f8: 48 be fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rsi # imm = 0x7FFFFFFFFFFFFFFC
140978a02: 48 39 f3                    	cmpq	%rsi, %rbx
140978a05: 0f 87 d6 0e 00 00           	ja	0x1409798e1 <.text+0x9788e1>
140978a0b: 4d 8b b4 24 68 fd ff ff     	movq	-0x298(%r12), %r14
140978a13: 48 85 db                    	testq	%rbx, %rbx
140978a16: 0f 84 cb 02 00 00           	je	0x140978ce7 <.text+0x977ce7>
140978a1c: 0f b6 05 e6 c1 cb 02        	movzbl	0x2cbc1e6(%rip), %eax   # 0x143634c09
140978a23: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140978a29: ba 04 00 00 00              	movl	$0x4, %edx
140978a2e: 48 89 d9                    	movq	%rbx, %rcx
140978a31: e8 da b1 c9 ff              	callq	0x140613c10 <.text+0x612c10>
140978a36: 48 85 c0                    	testq	%rax, %rax
140978a39: 0f 84 a2 0e 00 00           	je	0x1409798e1 <.text+0x9788e1>
140978a3f: 48 89 c1                    	movq	%rax, %rcx
140978a42: 49 89 f7                    	movq	%rsi, %r15
140978a45: 48 89 bd 00 02 00 00        	movq	%rdi, 0x200(%rbp)
140978a4c: e9 a9 02 00 00              	jmp	0x140978cfa <.text+0x977cfa>
140978a51: 4a 8b 4c 0b a0              	movq	-0x60(%rbx,%r9), %rcx
140978a56: 48 85 c9                    	testq	%rcx, %rcx
140978a59: 0f 84 1a 0f 00 00           	je	0x140979979 <.text+0x978979>
140978a5f: 4c 89 75 18                 	movq	%r14, 0x18(%rbp)
140978a63: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
140978a67: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
140978a6e: f6 00 01                    	testb	$0x1, (%rax)
140978a71: 74 10                       	je	0x140978a83 <.text+0x977a83>
140978a73: 66 44 0f 6e 40 04           	movd	0x4(%rax), %xmm8
140978a79: 66 0f 6e 40 0c              	movd	0xc(%rax), %xmm0
140978a7e: 66 44 0f 62 c0              	punpckldq	%xmm0, %xmm8    # xmm8 = xmm8[0],xmm0[0],xmm8[1],xmm0[1]
140978a83: 4c 8d 2c cd 00 00 00 00     	leaq	(,%rcx,8), %r13
140978a8b: 45 31 ff                    	xorl	%r15d, %r15d
140978a8e: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
140978a95: 48 89 c8                    	movq	%rcx, %rax
140978a98: 48 c1 e8 3d                 	shrq	$0x3d, %rax
140978a9c: 0f 85 74 0e 00 00           	jne	0x140979916 <.text+0x978916>
140978aa2: 48 be fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rsi # imm = 0x7FFFFFFFFFFFFFFC
140978aac: 49 39 f5                    	cmpq	%rsi, %r13
140978aaf: 0f 87 61 0e 00 00           	ja	0x140979916 <.text+0x978916>
140978ab5: 4c 01 cb                    	addq	%r9, %rbx
140978ab8: 4c 8b 63 98                 	movq	-0x68(%rbx), %r12
140978abc: f2 41 0f 10 34 24           	movsd	(%r12), %xmm6
140978ac2: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
140978ac9: f2 44 0f 10 48 50           	movsd	0x50(%rax), %xmm9
140978acf: 0f b6 05 33 c1 cb 02        	movzbl	0x2cbc133(%rip), %eax   # 0x143634c09
140978ad6: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140978adc: ba 04 00 00 00              	movl	$0x4, %edx
140978ae1: 4c 89 e9                    	movq	%r13, %rcx
140978ae4: e8 27 b1 c9 ff              	callq	0x140613c10 <.text+0x612c10>
140978ae9: 48 85 c0                    	testq	%rax, %rax
140978aec: 0f 84 24 0e 00 00           	je	0x140979916 <.text+0x978916>
140978af2: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
140978af9: 48 89 c1                    	movq	%rax, %rcx
140978afc: 4c 89 e2                    	movq	%r12, %rdx
140978aff: 4c 89 ad 88 01 00 00        	movq	%r13, 0x188(%rbp)
140978b06: 4d 89 e8                    	movq	%r13, %r8
140978b09: e8 f2 b0 fa 01              	callq	0x142923c00 <.text+0x2922c00>
140978b0e: 4c 8b 73 b8                 	movq	-0x48(%rbx), %r14
140978b12: 4e 8d 2c b5 00 00 00 00     	leaq	(,%r14,4), %r13
140978b1a: 45 31 ff                    	xorl	%r15d, %r15d
140978b1d: 4c 89 f0                    	movq	%r14, %rax
140978b20: 48 c1 e8 3e                 	shrq	$0x3e, %rax
140978b24: 0f 85 fe 0d 00 00           	jne	0x140979928 <.text+0x978928>
140978b2a: 49 39 f5                    	cmpq	%rsi, %r13
140978b2d: 0f 87 f5 0d 00 00           	ja	0x140979928 <.text+0x978928>
140978b33: 4c 8b 63 b0                 	movq	-0x50(%rbx), %r12
140978b37: 4d 85 ed                    	testq	%r13, %r13
140978b3a: 48 89 9d e8 01 00 00        	movq	%rbx, 0x1e8(%rbp)
140978b41: 0f 84 0c 03 00 00           	je	0x140978e53 <.text+0x977e53>
140978b47: 0f b6 05 bb c0 cb 02        	movzbl	0x2cbc0bb(%rip), %eax   # 0x143634c09
140978b4e: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140978b54: ba 04 00 00 00              	movl	$0x4, %edx
140978b59: 4c 89 e9                    	movq	%r13, %rcx
140978b5c: e8 af b0 c9 ff              	callq	0x140613c10 <.text+0x612c10>
140978b61: 48 85 c0                    	testq	%rax, %rax
140978b64: 0f 84 be 0d 00 00           	je	0x140979928 <.text+0x978928>
140978b6a: 48 89 f3                    	movq	%rsi, %rbx
140978b6d: 4c 89 f6                    	movq	%r14, %rsi
140978b70: e9 ec 02 00 00              	jmp	0x140978e61 <.text+0x977e61>
140978b75: b8 04 00 00 00              	movl	$0x4, %eax
140978b7a: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
140978b85: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
140978b8c: 48 89 c1                    	movq	%rax, %rcx
140978b8f: 4c 89 fa                    	movq	%r15, %rdx
140978b92: 4d 89 f0                    	movq	%r14, %r8
140978b95: e8 66 b0 fa 01              	callq	0x142923c00 <.text+0x2922c00>
140978b9a: 8b 47 1c                    	movl	0x1c(%rdi), %eax
140978b9d: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
140978ba1: 89 01                       	movl	%eax, (%rcx)
140978ba3: 48 8b 37                    	movq	(%rdi), %rsi
140978ba6: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
140978baa: 4c 8b 76 40                 	movq	0x40(%rsi), %r14
140978bae: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
140978bb2: 75 10                       	jne	0x140978bc4 <.text+0x977bc4>
140978bb4: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
140978bb8: 48 8d 15 89 15 2c 02        	leaq	0x22c1589(%rip), %rdx   # 0x142c3a148
140978bbf: e8 2c 9e be ff              	callq	0x1405629f0 <.text+0x5619f0>
140978bc4: 48 8b 46 28                 	movq	0x28(%rsi), %rax
140978bc8: 48 8d 0c 7f                 	leaq	(%rdi,%rdi,2), %rcx
140978bcc: 48 c1 e1 04                 	shlq	$0x4, %rcx
140978bd0: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
140978bd7: 48 89 14 08                 	movq	%rdx, (%rax,%rcx)
140978bdb: 48 8b 95 e8 01 00 00        	movq	0x1e8(%rbp), %rdx
140978be2: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
140978be7: 48 89 5c 08 10              	movq	%rbx, 0x10(%rax,%rcx)
140978bec: 4c 89 64 08 18              	movq	%r12, 0x18(%rax,%rcx)
140978bf1: 0f 13 7c 08 20              	movlps	%xmm7, 0x20(%rax,%rcx)
140978bf6: 4c 89 74 08 28              	movq	%r14, 0x28(%rax,%rcx)
140978bfb: 48 ff c7                    	incq	%rdi
140978bfe: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
140978c02: 48 ff 46 40                 	incq	0x40(%rsi)
140978c06: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
140978c0d: 48 8b 42 18                 	movq	0x18(%rdx), %rax
140978c11: 48 b9 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
140978c1b: 48 89 4a 18                 	movq	%rcx, 0x18(%rdx)
140978c1f: 48 89 c1                    	movq	%rax, %rcx
140978c22: 48 f7 d9                    	negq	%rcx
140978c25: 0f 80 97 06 00 00           	jo	0x1409792c2 <.text+0x9782c2>
140978c2b: 0f 10 42 20                 	movups	0x20(%rdx), %xmm0
140978c2f: 0f 10 4a 30                 	movups	0x30(%rdx), %xmm1
140978c33: 0f 10 52 40                 	movups	0x40(%rdx), %xmm2
140978c37: 0f 11 95 b8 01 00 00        	movups	%xmm2, 0x1b8(%rbp)
140978c3e: 0f 11 8d a8 01 00 00        	movups	%xmm1, 0x1a8(%rbp)
140978c45: 0f 11 85 98 01 00 00        	movups	%xmm0, 0x198(%rbp)
140978c4c: 48 c7 85 18 01 00 00 00 00 00 00    	movq	$0x0, 0x118(%rbp)
140978c57: 48 c7 85 20 01 00 00 08 00 00 00    	movq	$0x8, 0x120(%rbp)
140978c62: 48 c7 85 28 01 00 00 00 00 00 00    	movq	$0x0, 0x128(%rbp)
140978c6d: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
140978c74: 4c 8b a5 f8 01 00 00        	movq	0x1f8(%rbp), %r12
140978c7b: 49 8b b4 24 70 fd ff ff     	movq	-0x290(%r12), %rsi
140978c83: 48 8d 3c f5 00 00 00 00     	leaq	(,%rsi,8), %rdi
140978c8b: 45 31 f6                    	xorl	%r14d, %r14d
140978c8e: 48 89 f0                    	movq	%rsi, %rax
140978c91: 48 c1 e8 3d                 	shrq	$0x3d, %rax
140978c95: 0f 85 b6 0c 00 00           	jne	0x140979951 <.text+0x978951>
140978c9b: 4c 39 ef                    	cmpq	%r13, %rdi
140978c9e: 0f 87 ad 0c 00 00           	ja	0x140979951 <.text+0x978951>
140978ca4: 49 8b 9c 24 68 fd ff ff     	movq	-0x298(%r12), %rbx
140978cac: 48 85 ff                    	testq	%rdi, %rdi
140978caf: 0f 84 28 04 00 00           	je	0x1409790dd <.text+0x9780dd>
140978cb5: 0f b6 05 4d bf cb 02        	movzbl	0x2cbbf4d(%rip), %eax   # 0x143634c09
140978cbc: 41 be 04 00 00 00           	movl	$0x4, %r14d
140978cc2: ba 04 00 00 00              	movl	$0x4, %edx
140978cc7: 48 89 f9                    	movq	%rdi, %rcx
140978cca: e8 41 af c9 ff              	callq	0x140613c10 <.text+0x612c10>
140978ccf: 48 85 c0                    	testq	%rax, %rax
140978cd2: 0f 84 79 0c 00 00           	je	0x140979951 <.text+0x978951>
140978cd8: 4d 89 ee                    	movq	%r13, %r14
140978cdb: 48 89 b5 08 02 00 00        	movq	%rsi, 0x208(%rbp)
140978ce2: e9 09 04 00 00              	jmp	0x1409790f0 <.text+0x9780f0>
140978ce7: 49 89 f7                    	movq	%rsi, %r15
140978cea: b9 04 00 00 00              	movl	$0x4, %ecx
140978cef: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
140978cfa: 48 89 8d d8 01 00 00        	movq	%rcx, 0x1d8(%rbp)
140978d01: 4c 89 f2                    	movq	%r14, %rdx
140978d04: 49 89 d8                    	movq	%rbx, %r8
140978d07: e8 f4 ae fa 01              	callq	0x142923c00 <.text+0x2922c00>
140978d0c: 4d 8b ac 24 88 fd ff ff     	movq	-0x278(%r12), %r13
140978d14: 4e 8d 34 ad 00 00 00 00     	leaq	(,%r13,4), %r14
140978d1c: 31 f6                       	xorl	%esi, %esi
140978d1e: 4c 89 e8                    	movq	%r13, %rax
140978d21: 48 c1 e8 3e                 	shrq	$0x3e, %rax
140978d25: 0f 85 d4 0b 00 00           	jne	0x1409798ff <.text+0x9788ff>
140978d2b: 4d 39 fe                    	cmpq	%r15, %r14
140978d2e: 0f 87 cb 0b 00 00           	ja	0x1409798ff <.text+0x9788ff>
140978d34: 4d 8b bc 24 80 fd ff ff     	movq	-0x280(%r12), %r15
140978d3c: 4d 85 f6                    	testq	%r14, %r14
140978d3f: 74 2a                       	je	0x140978d6b <.text+0x977d6b>
140978d41: 0f b6 05 c1 be cb 02        	movzbl	0x2cbbec1(%rip), %eax   # 0x143634c09
140978d48: be 04 00 00 00              	movl	$0x4, %esi
140978d4d: ba 04 00 00 00              	movl	$0x4, %edx
140978d52: 4c 89 f1                    	movq	%r14, %rcx
140978d55: e8 b6 ae c9 ff              	callq	0x140613c10 <.text+0x612c10>
140978d5a: 48 85 c0                    	testq	%rax, %rax
140978d5d: 0f 84 9c 0b 00 00           	je	0x1409798ff <.text+0x9788ff>
140978d63: 48 89 c3                    	movq	%rax, %rbx
140978d66: 4c 89 ee                    	movq	%r13, %rsi
140978d69: eb 07                       	jmp	0x140978d72 <.text+0x977d72>
140978d6b: bb 04 00 00 00              	movl	$0x4, %ebx
140978d70: 31 f6                       	xorl	%esi, %esi
140978d72: 48 89 d9                    	movq	%rbx, %rcx
140978d75: 4c 89 fa                    	movq	%r15, %rdx
140978d78: 4d 89 f0                    	movq	%r14, %r8
140978d7b: e8 80 ae fa 01              	callq	0x142923c00 <.text+0x2922c00>
140978d80: f3 41 0f 10 b4 24 90 fd ff ff       	movss	-0x270(%r12), %xmm6
140978d8a: 4c 8b b5 08 02 00 00        	movq	0x208(%rbp), %r14
140978d91: 49 8b 56 18                 	movq	0x18(%r14), %rdx
140978d95: 48 89 d0                    	movq	%rdx, %rax
140978d98: 48 f7 d8                    	negq	%rax
140978d9b: 70 31                       	jo	0x140978dce <.text+0x977dce>
140978d9d: 73 13                       	jae	0x140978db2 <.text+0x977db2>
140978d9f: 49 8b 4e 20                 	movq	0x20(%r14), %rcx
140978da3: 48 c1 e2 03                 	shlq	$0x3, %rdx
140978da7: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140978dad: e8 6e ae c9 ff              	callq	0x140613c20 <.text+0x612c20>
140978db2: 49 8b 56 30                 	movq	0x30(%r14), %rdx
140978db6: 48 85 d2                    	testq	%rdx, %rdx
140978db9: 74 13                       	je	0x140978dce <.text+0x977dce>
140978dbb: 49 8b 4e 38                 	movq	0x38(%r14), %rcx
140978dbf: 48 c1 e2 02                 	shlq	$0x2, %rdx
140978dc3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140978dc9: e8 52 ae c9 ff              	callq	0x140613c20 <.text+0x612c20>
140978dce: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
140978dd5: 49 89 46 18                 	movq	%rax, 0x18(%r14)
140978dd9: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
140978de0: 49 89 46 20                 	movq	%rax, 0x20(%r14)
140978de4: 49 89 7e 28                 	movq	%rdi, 0x28(%r14)
140978de8: 49 89 76 30                 	movq	%rsi, 0x30(%r14)
140978dec: 49 89 5e 38                 	movq	%rbx, 0x38(%r14)
140978df0: 4d 89 6e 40                 	movq	%r13, 0x40(%r14)
140978df4: f3 41 0f 11 76 48           	movss	%xmm6, 0x48(%r14)
140978dfa: 0f 57 c0                    	xorps	%xmm0, %xmm0
140978dfd: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
140978e04: f6 00 01                    	testb	$0x1, (%rax)
140978e07: 74 0d                       	je	0x140978e16 <.text+0x977e16>
140978e09: f3 0f 10 40 04              	movss	0x4(%rax), %xmm0
140978e0e: f3 0f 10 48 0c              	movss	0xc(%rax), %xmm1
140978e13: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
140978e16: 49 83 bc 24 70 fd ff ff 00  	cmpq	$0x0, -0x290(%r12)
140978e1f: 4c 89 e0                    	movq	%r12, %rax
140978e22: 4c 8b a5 e0 01 00 00        	movq	0x1e0(%rbp), %r12
140978e29: 0f 84 0d 0b 00 00           	je	0x14097993c <.text+0x97893c>
140978e2f: 48 8b 80 68 fd ff ff        	movq	-0x298(%rax), %rax
140978e36: f2 0f 10 08                 	movsd	(%rax), %xmm1
140978e3a: 0f 5c c1                    	subps	%xmm1, %xmm0
140978e3d: 41 0f 13 46 50              	movlps	%xmm0, 0x50(%r14)
140978e42: e9 2b 09 00 00              	jmp	0x140979772 <.text+0x978772>
140978e47: 48 8d 0d 2a dd 13 02        	leaq	0x213dd2a(%rip), %rcx   # 0x142ab6b78
140978e4e: e8 bd 04 f6 01              	callq	0x1428d9310 <.text+0x28d8310>
140978e53: 48 89 f3                    	movq	%rsi, %rbx
140978e56: b8 04 00 00 00              	movl	$0x4, %eax
140978e5b: 4c 89 f6                    	movq	%r14, %rsi
140978e5e: 45 31 f6                    	xorl	%r14d, %r14d
140978e61: 49 89 c7                    	movq	%rax, %r15
140978e64: 48 89 c1                    	movq	%rax, %rcx
140978e67: 4c 89 e2                    	movq	%r12, %rdx
140978e6a: 4d 89 e8                    	movq	%r13, %r8
140978e6d: e8 8e ad fa 01              	callq	0x142923c00 <.text+0x2922c00>
140978e72: 45 0f 5c c1                 	subps	%xmm9, %xmm8
140978e76: 44 0f 5c c6                 	subps	%xmm6, %xmm8
140978e7a: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
140978e81: f3 0f 10 70 c0              	movss	-0x40(%rax), %xmm6
140978e86: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
140978e8d: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
140978e94: 4c 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %r9
140978e9b: 4c 89 8d 98 01 00 00        	movq	%r9, 0x198(%rbp)
140978ea2: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
140978ea9: 4c 89 b5 78 01 00 00        	movq	%r14, 0x178(%rbp)
140978eb0: 4c 89 b5 a8 01 00 00        	movq	%r14, 0x1a8(%rbp)
140978eb7: 4c 89 bd 80 01 00 00        	movq	%r15, 0x180(%rbp)
140978ebe: 4c 89 bd b0 01 00 00        	movq	%r15, 0x1b0(%rbp)
140978ec5: 48 89 b5 b8 01 00 00        	movq	%rsi, 0x1b8(%rbp)
140978ecc: f3 0f 11 b5 c0 01 00 00     	movss	%xmm6, 0x1c0(%rbp)
140978ed4: 4d 8d 3c c1                 	leaq	(%r9,%rax,8), %r15
140978ed8: 48 ff c8                    	decq	%rax
140978edb: 48 b9 ff ff ff ff ff ff ff 1f       	movabsq	$0x1fffffffffffffff, %rcx # imm = 0x1FFFFFFFFFFFFFFF
140978ee5: 48 21 c1                    	andq	%rax, %rcx
140978ee8: 4c 89 c8                    	movq	%r9, %rax
140978eeb: 48 83 f9 03                 	cmpq	$0x3, %rcx
140978eef: 49 89 f6                    	movq	%rsi, %r14
140978ef2: 48 89 de                    	movq	%rbx, %rsi
140978ef5: 72 59                       	jb	0x140978f50 <.text+0x977f50>
140978ef7: 48 ff c1                    	incq	%rcx
140978efa: 48 89 ca                    	movq	%rcx, %rdx
140978efd: 48 83 e2 fc                 	andq	$-0x4, %rdx
140978f01: 49 8d 04 d1                 	leaq	(%r9,%rdx,8), %rax
140978f05: f3 41 0f 12 c0              	movsldup	%xmm8, %xmm0            # xmm0 = xmm8[0,0,2,2]
140978f0a: f3 41 0f 16 c8              	movshdup	%xmm8, %xmm1            # xmm1 = xmm8[1,1,3,3]
140978f0f: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
140978f12: 45 31 c0                    	xorl	%r8d, %r8d
140978f15: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140978f20: 43 0f 10 0c c1              	movups	(%r9,%r8,8), %xmm1
140978f25: 43 0f 10 54 c1 10           	movups	0x10(%r9,%r8,8), %xmm2
140978f2b: 0f 58 c8                    	addps	%xmm0, %xmm1
140978f2e: 43 0f 11 0c c1              	movups	%xmm1, (%r9,%r8,8)
140978f33: 0f 58 d0                    	addps	%xmm0, %xmm2
140978f36: 43 0f 11 54 c1 10           	movups	%xmm2, 0x10(%r9,%r8,8)
140978f3c: 49 83 c0 04                 	addq	$0x4, %r8
140978f40: 4c 39 c2                    	cmpq	%r8, %rdx
140978f43: 75 db                       	jne	0x140978f20 <.text+0x977f20>
140978f45: 48 39 d1                    	cmpq	%rdx, %rcx
140978f48: 74 1d                       	je	0x140978f67 <.text+0x977f67>
140978f4a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140978f50: f2 0f 10 00                 	movsd	(%rax), %xmm0
140978f54: 41 0f 58 c0                 	addps	%xmm8, %xmm0
140978f58: 0f 13 00                    	movlps	%xmm0, (%rax)
140978f5b: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
140978f5f: 48 89 c8                    	movq	%rcx, %rax
140978f62: 4c 39 f9                    	cmpq	%r15, %rcx
140978f65: 75 e9                       	jne	0x140978f50 <.text+0x977f50>
140978f67: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140978f6b: 4c 8d 68 04                 	leaq	0x4(%rax), %r13
140978f6f: f3 41 0f 10 09              	movss	(%r9), %xmm1
140978f74: f3 41 0f 10 51 04           	movss	0x4(%r9), %xmm2
140978f7a: 83 38 01                    	cmpl	$0x1, (%rax)
140978f7d: 75 54                       	jne	0x140978fd3 <.text+0x977fd3>
140978f7f: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
140978f86: 4c 89 e9                    	movq	%r13, %rcx
140978f89: e8 a2 16 32 00              	callq	0x140c9a630 <.text+0xc99630>
140978f8e: 48 8b 9d e8 01 00 00        	movq	0x1e8(%rbp), %rbx
140978f95: 84 c0                       	testb	%al, %al
140978f97: 0f 84 84 00 00 00           	je	0x140979021 <.text+0x978021>
140978f9d: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
140978fa4: 4c 8d 61 08                 	leaq	0x8(%rcx), %r12
140978fa8: 4d 39 fc                    	cmpq	%r15, %r12
140978fab: 0f 84 6b 03 00 00           	je	0x14097931c <.text+0x97831c>
140978fb1: f3 0f 10 49 08              	movss	0x8(%rcx), %xmm1
140978fb6: f3 0f 10 51 0c              	movss	0xc(%rcx), %xmm2
140978fbb: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
140978fc2: 4c 89 e9                    	movq	%r13, %rcx
140978fc5: e8 66 16 32 00              	callq	0x140c9a630 <.text+0xc99630>
140978fca: 4c 89 e1                    	movq	%r12, %rcx
140978fcd: 84 c0                       	testb	%al, %al
140978fcf: 75 d3                       	jne	0x140978fa4 <.text+0x977fa4>
140978fd1: eb 4e                       	jmp	0x140979021 <.text+0x978021>
140978fd3: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
140978fda: 4c 89 e9                    	movq	%r13, %rcx
140978fdd: e8 0e f1 31 00              	callq	0x140c980f0 <.text+0xc970f0>
140978fe2: 48 8b 9d e8 01 00 00        	movq	0x1e8(%rbp), %rbx
140978fe9: 84 c0                       	testb	%al, %al
140978feb: 74 34                       	je	0x140979021 <.text+0x978021>
140978fed: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
140978ff4: 4c 8d 61 08                 	leaq	0x8(%rcx), %r12
140978ff8: 4d 39 fc                    	cmpq	%r15, %r12
140978ffb: 0f 84 1b 03 00 00           	je	0x14097931c <.text+0x97831c>
140979001: f3 0f 10 49 08              	movss	0x8(%rcx), %xmm1
140979006: f3 0f 10 51 0c              	movss	0xc(%rcx), %xmm2
14097900b: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
140979012: 4c 89 e9                    	movq	%r13, %rcx
140979015: e8 d6 f0 31 00              	callq	0x140c980f0 <.text+0xc970f0>
14097901a: 4c 89 e1                    	movq	%r12, %rcx
14097901d: 84 c0                       	testb	%al, %al
14097901f: 75 d3                       	jne	0x140978ff4 <.text+0x977ff4>
140979021: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140979025: 8b 8d 5c 01 00 00           	movl	0x15c(%rbp), %ecx
14097902b: 89 08                       	movl	%ecx, (%rax)
14097902d: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140979034: 48 8b 70 30                 	movq	0x30(%rax), %rsi
140979038: 48 8b 78 40                 	movq	0x40(%rax), %rdi
14097903c: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
140979040: 75 1e                       	jne	0x140979060 <.text+0x978060>
140979042: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140979049: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14097904d: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
140979054: 48 8d 15 ed 10 2c 02        	leaq	0x22c10ed(%rip), %rdx   # 0x142c3a148
14097905b: e8 10 98 be ff              	callq	0x140562870 <.text+0x561870>
140979060: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
140979067: 48 8b 41 28                 	movq	0x28(%rcx), %rax
14097906b: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
14097906f: 48 ff c6                    	incq	%rsi
140979072: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
140979076: 48 ff 41 40                 	incq	0x40(%rcx)
14097907a: 48 8b 45 08                 	movq	0x8(%rbp), %rax
14097907e: 8b 8d 58 01 00 00           	movl	0x158(%rbp), %ecx
140979084: 89 08                       	movl	%ecx, (%rax)
140979086: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
14097908d: 48 8b 70 30                 	movq	0x30(%rax), %rsi
140979091: 48 8b 78 40                 	movq	0x40(%rax), %rdi
140979095: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
140979099: 75 1e                       	jne	0x1409790b9 <.text+0x9780b9>
14097909b: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1409790a2: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
1409790a6: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
1409790ad: 48 8d 15 94 10 2c 02        	leaq	0x22c1094(%rip), %rdx   # 0x142c3a148
1409790b4: e8 b7 97 be ff              	callq	0x140562870 <.text+0x561870>
1409790b9: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
1409790c0: 48 8b 41 28                 	movq	0x28(%rcx), %rax
1409790c4: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
1409790c8: 48 ff c6                    	incq	%rsi
1409790cb: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
1409790cf: 48 ff 41 40                 	incq	0x40(%rcx)
1409790d3: 40 b7 01                    	movb	$0x1, %dil
1409790d6: 31 f6                       	xorl	%esi, %esi
1409790d8: e9 67 05 00 00              	jmp	0x140979644 <.text+0x978644>
1409790dd: 4d 89 ee                    	movq	%r13, %r14
1409790e0: b8 04 00 00 00              	movl	$0x4, %eax
1409790e5: 48 c7 85 08 02 00 00 00 00 00 00    	movq	$0x0, 0x208(%rbp)
1409790f0: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1409790f7: 48 89 c1                    	movq	%rax, %rcx
1409790fa: 48 89 da                    	movq	%rbx, %rdx
1409790fd: 49 89 f8                    	movq	%rdi, %r8
140979100: e8 fb aa fa 01              	callq	0x142923c00 <.text+0x2922c00>
140979105: 4d 8b ac 24 88 fd ff ff     	movq	-0x278(%r12), %r13
14097910d: 4a 8d 3c ad 00 00 00 00     	leaq	(,%r13,4), %rdi
140979115: 45 31 ff                    	xorl	%r15d, %r15d
140979118: 4c 89 e8                    	movq	%r13, %rax
14097911b: 48 c1 e8 3e                 	shrq	$0x3e, %rax
14097911f: 0f 85 40 08 00 00           	jne	0x140979965 <.text+0x978965>
140979125: 4c 39 f7                    	cmpq	%r14, %rdi
140979128: 0f 87 37 08 00 00           	ja	0x140979965 <.text+0x978965>
14097912e: 4d 8b b4 24 80 fd ff ff     	movq	-0x280(%r12), %r14
140979136: 48 85 ff                    	testq	%rdi, %rdi
140979139: 74 2b                       	je	0x140979166 <.text+0x978166>
14097913b: 0f b6 05 c7 ba cb 02        	movzbl	0x2cbbac7(%rip), %eax   # 0x143634c09
140979142: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140979148: ba 04 00 00 00              	movl	$0x4, %edx
14097914d: 48 89 f9                    	movq	%rdi, %rcx
140979150: e8 bb aa c9 ff              	callq	0x140613c10 <.text+0x612c10>
140979155: 48 85 c0                    	testq	%rax, %rax
140979158: 0f 84 07 08 00 00           	je	0x140979965 <.text+0x978965>
14097915e: 48 89 c3                    	movq	%rax, %rbx
140979161: 4d 89 ef                    	movq	%r13, %r15
140979164: eb 08                       	jmp	0x14097916e <.text+0x97816e>
140979166: bb 04 00 00 00              	movl	$0x4, %ebx
14097916b: 45 31 ff                    	xorl	%r15d, %r15d
14097916e: 48 89 d9                    	movq	%rbx, %rcx
140979171: 4c 89 f2                    	movq	%r14, %rdx
140979174: 49 89 f8                    	movq	%rdi, %r8
140979177: e8 84 aa fa 01              	callq	0x142923c00 <.text+0x2922c00>
14097917c: f3 41 0f 10 84 24 90 fd ff ff       	movss	-0x270(%r12), %xmm0
140979186: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
14097918d: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
140979194: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
14097919b: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14097919f: 0f 10 8d 90 01 00 00        	movups	0x190(%rbp), %xmm1
1409791a6: 0f 10 95 a0 01 00 00        	movups	0x1a0(%rbp), %xmm2
1409791ad: 0f 10 9d b0 01 00 00        	movups	0x1b0(%rbp), %xmm3
1409791b4: 0f 11 5d 48                 	movups	%xmm3, 0x48(%rbp)
1409791b8: 0f 11 55 38                 	movups	%xmm2, 0x38(%rbp)
1409791bc: 0f 11 4d 28                 	movups	%xmm1, 0x28(%rbp)
1409791c0: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
1409791c7: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1409791cb: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1409791d2: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1409791d6: 48 89 75 70                 	movq	%rsi, 0x70(%rbp)
1409791da: 4c 89 7d 78                 	movq	%r15, 0x78(%rbp)
1409791de: 48 89 9d 80 00 00 00        	movq	%rbx, 0x80(%rbp)
1409791e5: 4c 89 ad 88 00 00 00        	movq	%r13, 0x88(%rbp)
1409791ec: f3 0f 11 85 90 00 00 00     	movss	%xmm0, 0x90(%rbp)
1409791f4: 48 b8 0c 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff4, %rax # imm = 0x800000000000000C
1409791fe: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140979202: c6 85 15 02 00 00 01        	movb	$0x1, 0x215(%rbp)
140979209: 48 8d 8d 18 01 00 00        	leaq	0x118(%rbp), %rcx
140979210: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
140979214: e8 27 99 10 00              	callq	0x140a82b40 <.text+0xa81b40>
140979219: 48 8b 95 f8 02 00 00        	movq	0x2f8(%rbp), %rdx
140979220: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140979223: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140979227: 89 01                       	movl	%eax, (%rcx)
140979229: 48 8b 3a                    	movq	(%rdx), %rdi
14097922c: 0f 10 85 18 01 00 00        	movups	0x118(%rbp), %xmm0
140979233: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
14097923a: 48 8b 85 28 01 00 00        	movq	0x128(%rbp), %rax
140979241: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
140979248: 48 8b 95 98 01 00 00        	movq	0x198(%rbp), %rdx
14097924f: 48 c7 85 90 01 00 00 00 00 00 00    	movq	$0x0, 0x190(%rbp)
14097925a: 48 c7 85 98 01 00 00 08 00 00 00    	movq	$0x8, 0x198(%rbp)
140979265: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
140979270: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140979274: 48 8d 04 c2                 	leaq	(%rdx,%rax,8), %rax
140979278: 48 89 55 20                 	movq	%rdx, 0x20(%rbp)
14097927c: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
140979280: 48 89 4d 30                 	movq	%rcx, 0x30(%rbp)
140979284: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
140979288: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14097928c: e8 bf dc d1 00              	callq	0x141696f50 <.text+0x1695f50>
140979291: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140979295: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
140979299: 48 c7 45 20 07 00 00 00     	movq	$0x7, 0x20(%rbp)
1409792a1: c6 85 15 02 00 00 00        	movb	$0x0, 0x215(%rbp)
1409792a8: 4c 8d 05 f9 d8 13 02        	leaq	0x213d8f9(%rip), %r8    # 0x142ab6ba8
1409792af: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
1409792b3: 48 89 f9                    	movq	%rdi, %rcx
1409792b6: e8 05 bc 10 00              	callq	0x140a84ec0 <.text+0xa83ec0>
1409792bb: 4c 8b a5 e0 01 00 00        	movq	0x1e0(%rbp), %r12
1409792c2: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
1409792c9: 8b 8d d4 01 00 00           	movl	0x1d4(%rbp), %ecx
1409792cf: 89 08                       	movl	%ecx, (%rax)
1409792d1: 48 8b 9d d8 01 00 00        	movq	0x1d8(%rbp), %rbx
1409792d8: 48 8b 73 30                 	movq	0x30(%rbx), %rsi
1409792dc: 48 8b 7b 40                 	movq	0x40(%rbx), %rdi
1409792e0: 48 3b 73 20                 	cmpq	0x20(%rbx), %rsi
1409792e4: 75 10                       	jne	0x1409792f6 <.text+0x9782f6>
1409792e6: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
1409792ea: 48 8d 15 57 0e 2c 02        	leaq	0x22c0e57(%rip), %rdx   # 0x142c3a148
1409792f1: e8 ba 9a be ff              	callq	0x140562db0 <.text+0x561db0>
1409792f6: 48 8b 43 28                 	movq	0x28(%rbx), %rax
1409792fa: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
1409792fe: 48 89 3c c8                 	movq	%rdi, (%rax,%rcx,8)
140979302: 4c 89 64 c8 08              	movq	%r12, 0x8(%rax,%rcx,8)
140979307: c6 44 c8 10 01              	movb	$0x1, 0x10(%rax,%rcx,8)
14097930c: 48 ff c6                    	incq	%rsi
14097930f: 48 89 73 30                 	movq	%rsi, 0x30(%rbx)
140979313: 48 ff 43 40                 	incq	0x40(%rbx)
140979317: e9 56 04 00 00              	jmp	0x140979772 <.text+0x978772>
14097931c: 4c 8b 85 08 02 00 00        	movq	0x208(%rbp), %r8
140979323: 4d 8b 68 10                 	movq	0x10(%r8), %r13
140979327: 31 c9                       	xorl	%ecx, %ecx
140979329: ba 0c 00 00 00              	movl	$0xc, %edx
14097932e: 4c 89 e8                    	movq	%r13, %rax
140979331: 48 f7 e2                    	mulq	%rdx
140979334: 49 89 c4                    	movq	%rax, %r12
140979337: 0f 80 48 06 00 00           	jo	0x140979985 <.text+0x978985>
14097933d: 49 39 f4                    	cmpq	%rsi, %r12
140979340: 0f 87 3f 06 00 00           	ja	0x140979985 <.text+0x978985>
140979346: 4d 8b 78 08                 	movq	0x8(%r8), %r15
14097934a: 4d 85 e4                    	testq	%r12, %r12
14097934d: 74 2b                       	je	0x14097937a <.text+0x97837a>
14097934f: 0f b6 05 b3 b8 cb 02        	movzbl	0x2cbb8b3(%rip), %eax   # 0x143634c09
140979356: ba 04 00 00 00              	movl	$0x4, %edx
14097935b: 4c 89 e1                    	movq	%r12, %rcx
14097935e: e8 ad a8 c9 ff              	callq	0x140613c10 <.text+0x612c10>
140979363: b9 04 00 00 00              	movl	$0x4, %ecx
140979368: 48 85 c0                    	testq	%rax, %rax
14097936b: 0f 84 14 06 00 00           	je	0x140979985 <.text+0x978985>
140979371: 4c 89 ad 08 02 00 00        	movq	%r13, 0x208(%rbp)
140979378: eb 10                       	jmp	0x14097938a <.text+0x97838a>
14097937a: b8 04 00 00 00              	movl	$0x4, %eax
14097937f: 48 c7 85 08 02 00 00 00 00 00 00    	movq	$0x0, 0x208(%rbp)
14097938a: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140979391: 48 89 c1                    	movq	%rax, %rcx
140979394: 4c 89 fa                    	movq	%r15, %rdx
140979397: 4d 89 e0                    	movq	%r12, %r8
14097939a: e8 61 a8 fa 01              	callq	0x142923c00 <.text+0x2922c00>
14097939f: 8b 47 1c                    	movl	0x1c(%rdi), %eax
1409793a2: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
1409793a6: 89 01                       	movl	%eax, (%rcx)
1409793a8: 48 8b 3f                    	movq	(%rdi), %rdi
1409793ab: 4c 8b 7f 30                 	movq	0x30(%rdi), %r15
1409793af: 4c 8b 67 40                 	movq	0x40(%rdi), %r12
1409793b3: 4c 3b 7f 20                 	cmpq	0x20(%rdi), %r15
1409793b7: 75 10                       	jne	0x1409793c9 <.text+0x9783c9>
1409793b9: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
1409793bd: 48 8d 15 84 0d 2c 02        	leaq	0x22c0d84(%rip), %rdx   # 0x142c3a148
1409793c4: e8 27 96 be ff              	callq	0x1405629f0 <.text+0x5619f0>
1409793c9: 48 8b 47 28                 	movq	0x28(%rdi), %rax
1409793cd: 4b 8d 0c 7f                 	leaq	(%r15,%r15,2), %rcx
1409793d1: 48 c1 e1 04                 	shlq	$0x4, %rcx
1409793d5: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
1409793dc: 48 89 14 08                 	movq	%rdx, (%rax,%rcx)
1409793e0: 48 8b 95 f0 01 00 00        	movq	0x1f0(%rbp), %rdx
1409793e7: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
1409793ec: 4c 89 6c 08 10              	movq	%r13, 0x10(%rax,%rcx)
1409793f1: 48 8b 95 e0 01 00 00        	movq	0x1e0(%rbp), %rdx
1409793f8: 48 89 54 08 18              	movq	%rdx, 0x18(%rax,%rcx)
1409793fd: 0f 13 7c 08 20              	movlps	%xmm7, 0x20(%rax,%rcx)
140979402: 4c 89 64 08 28              	movq	%r12, 0x28(%rax,%rcx)
140979407: 49 ff c7                    	incq	%r15
14097940a: 4c 89 7f 30                 	movq	%r15, 0x30(%rdi)
14097940e: 48 ff 47 40                 	incq	0x40(%rdi)
140979412: 48 8b 43 a0                 	movq	-0x60(%rbx), %rax
140979416: 48 8d 3c c5 00 00 00 00     	leaq	(,%rax,8), %rdi
14097941e: 45 31 ff                    	xorl	%r15d, %r15d
140979421: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
140979428: 48 c1 e8 3d                 	shrq	$0x3d, %rax
14097942c: 75 2d                       	jne	0x14097945b <.text+0x97845b>
14097942e: 48 39 f7                    	cmpq	%rsi, %rdi
140979431: 77 28                       	ja	0x14097945b <.text+0x97845b>
140979433: 4c 8b 63 98                 	movq	-0x68(%rbx), %r12
140979437: 48 85 ff                    	testq	%rdi, %rdi
14097943a: 74 3d                       	je	0x140979479 <.text+0x978479>
14097943c: 0f b6 05 c6 b7 cb 02        	movzbl	0x2cbb7c6(%rip), %eax   # 0x143634c09
140979443: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140979449: ba 04 00 00 00              	movl	$0x4, %edx
14097944e: 48 89 f9                    	movq	%rdi, %rcx
140979451: e8 ba a7 c9 ff              	callq	0x140613c10 <.text+0x612c10>
140979456: 48 85 c0                    	testq	%rax, %rax
140979459: 75 2e                       	jne	0x140979489 <.text+0x978489>
14097945b: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
140979462: 4c 8d 05 0f f7 2b 02        	leaq	0x22bf70f(%rip), %r8    # 0x142c38b78
140979469: 4c 89 f9                    	movq	%r15, %rcx
14097946c: 48 89 fa                    	movq	%rdi, %rdx
14097946f: e8 6f fb f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140979474: e9 36 05 00 00              	jmp	0x1409799af <.text+0x9789af>
140979479: b8 04 00 00 00              	movl	$0x4, %eax
14097947e: 48 c7 85 08 02 00 00 00 00 00 00    	movq	$0x0, 0x208(%rbp)
140979489: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140979490: 48 89 c1                    	movq	%rax, %rcx
140979493: 4c 89 e2                    	movq	%r12, %rdx
140979496: 49 89 f8                    	movq	%rdi, %r8
140979499: e8 62 a7 fa 01              	callq	0x142923c00 <.text+0x2922c00>
14097949e: 48 8b 7b b8                 	movq	-0x48(%rbx), %rdi
1409794a2: 4c 8d 24 bd 00 00 00 00     	leaq	(,%rdi,4), %r12
1409794aa: 45 31 ed                    	xorl	%r13d, %r13d
1409794ad: 48 89 f8                    	movq	%rdi, %rax
1409794b0: 48 c1 e8 3e                 	shrq	$0x3e, %rax
1409794b4: 0f 85 e3 04 00 00           	jne	0x14097999d <.text+0x97899d>
1409794ba: 49 39 f4                    	cmpq	%rsi, %r12
1409794bd: 0f 87 da 04 00 00           	ja	0x14097999d <.text+0x97899d>
1409794c3: 4c 8b 7b b0                 	movq	-0x50(%rbx), %r15
1409794c7: 4d 85 e4                    	testq	%r12, %r12
1409794ca: 74 28                       	je	0x1409794f4 <.text+0x9784f4>
1409794cc: 0f b6 05 36 b7 cb 02        	movzbl	0x2cbb736(%rip), %eax   # 0x143634c09
1409794d3: 41 bd 04 00 00 00           	movl	$0x4, %r13d
1409794d9: ba 04 00 00 00              	movl	$0x4, %edx
1409794de: 4c 89 e1                    	movq	%r12, %rcx
1409794e1: e8 2a a7 c9 ff              	callq	0x140613c10 <.text+0x612c10>
1409794e6: 48 85 c0                    	testq	%rax, %rax
1409794e9: 0f 84 ae 04 00 00           	je	0x14097999d <.text+0x97899d>
1409794ef: 48 89 c6                    	movq	%rax, %rsi
1409794f2: eb 07                       	jmp	0x1409794fb <.text+0x9784fb>
1409794f4: be 04 00 00 00              	movl	$0x4, %esi
1409794f9: 31 ff                       	xorl	%edi, %edi
1409794fb: 48 89 f1                    	movq	%rsi, %rcx
1409794fe: 4c 89 fa                    	movq	%r15, %rdx
140979501: 4d 89 e0                    	movq	%r12, %r8
140979504: e8 f7 a6 fa 01              	callq	0x142923c00 <.text+0x2922c00>
140979509: 8b 43 c8                    	movl	-0x38(%rbx), %eax
14097950c: 8d 48 fe                    	leal	-0x2(%rax), %ecx
14097950f: 83 f9 02                    	cmpl	$0x2, %ecx
140979512: 41 bf 02 00 00 00           	movl	$0x2, %r15d
140979518: 41 0f 43 cf                 	cmovael	%r15d, %ecx
14097951c: 85 c9                       	testl	%ecx, %ecx
14097951e: 74 20                       	je	0x140979540 <.text+0x978540>
140979520: 83 f9 02                    	cmpl	$0x2, %ecx
140979523: 75 15                       	jne	0x14097953a <.text+0x97853a>
140979525: 48 8b 4b dc                 	movq	-0x24(%rbx), %rcx
140979529: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
14097952d: 0f 10 43 cc                 	movups	-0x34(%rbx), %xmm0
140979531: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
140979535: 41 89 c7                    	movl	%eax, %r15d
140979538: eb 06                       	jmp	0x140979540 <.text+0x978540>
14097953a: 41 bf 03 00 00 00           	movl	$0x3, %r15d
140979540: f3 0f 10 7b e4              	movss	-0x1c(%rbx), %xmm7
140979545: f2 44 0f 10 43 e8           	movsd	-0x18(%rbx), %xmm8
14097954b: 44 0f b6 63 f8              	movzbl	-0x8(%rbx), %r12d
140979550: 44 0f b6 6b f9              	movzbl	-0x7(%rbx), %r13d
140979555: f3 44 0f 10 4b f0           	movss	-0x10(%rbx), %xmm9
14097955b: f3 44 0f 10 53 f4           	movss	-0xc(%rbx), %xmm10
140979561: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
140979568: 48 85 d2                    	testq	%rdx, %rdx
14097956b: 74 16                       	je	0x140979583 <.text+0x978583>
14097956d: 48 c1 e2 03                 	shlq	$0x3, %rdx
140979571: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140979577: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
14097957e: e8 9d a6 c9 ff              	callq	0x140613c20 <.text+0x612c20>
140979583: 48 85 ff                    	testq	%rdi, %rdi
140979586: 74 15                       	je	0x14097959d <.text+0x97859d>
140979588: 48 c1 e7 02                 	shlq	$0x2, %rdi
14097958c: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140979592: 48 89 f1                    	movq	%rsi, %rcx
140979595: 48 89 fa                    	movq	%rdi, %rdx
140979598: e8 83 a6 c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097959d: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
1409795a4: 8b 41 1c                    	movl	0x1c(%rcx), %eax
1409795a7: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
1409795ab: 89 01                       	movl	%eax, (%rcx)
1409795ad: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
1409795b4: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
1409795b8: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1409795bf: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
1409795c3: 48 89 4d 30                 	movq	%rcx, 0x30(%rbp)
1409795c7: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
1409795ce: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
1409795d2: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1409795d9: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1409795dd: 4c 89 75 48                 	movq	%r14, 0x48(%rbp)
1409795e1: f3 0f 11 75 50              	movss	%xmm6, 0x50(%rbp)
1409795e6: 44 89 7d 58                 	movl	%r15d, 0x58(%rbp)
1409795ea: 0f 28 45 d0                 	movaps	-0x30(%rbp), %xmm0
1409795ee: 0f 11 45 5c                 	movups	%xmm0, 0x5c(%rbp)
1409795f2: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
1409795f6: 48 89 45 6c                 	movq	%rax, 0x6c(%rbp)
1409795fa: f3 0f 11 7d 74              	movss	%xmm7, 0x74(%rbp)
1409795ff: 44 0f 13 45 78              	movlps	%xmm8, 0x78(%rbp)
140979604: f3 44 0f 11 8d 80 00 00 00  	movss	%xmm9, 0x80(%rbp)
14097960d: f3 44 0f 11 95 84 00 00 00  	movss	%xmm10, 0x84(%rbp)
140979616: 44 88 a5 88 00 00 00        	movb	%r12b, 0x88(%rbp)
14097961d: 44 88 ad 89 00 00 00        	movb	%r13b, 0x89(%rbp)
140979624: c6 85 17 02 00 00 00        	movb	$0x0, 0x217(%rbp)
14097962b: 4c 8d 45 20                 	leaq	0x20(%rbp), %r8
14097962f: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
140979633: 48 8b 95 e0 01 00 00        	movq	0x1e0(%rbp), %rdx
14097963a: e8 81 05 13 00              	callq	0x140aa9bc0 <.text+0xaa8bc0>
14097963f: 40 b6 01                    	movb	$0x1, %sil
140979642: 31 ff                       	xorl	%edi, %edi
140979644: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
14097964b: 8b 8d d4 01 00 00           	movl	0x1d4(%rbp), %ecx
140979651: 89 08                       	movl	%ecx, (%rax)
140979653: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
14097965a: 48 8b 58 30                 	movq	0x30(%rax), %rbx
14097965e: 4c 8b 70 40                 	movq	0x40(%rax), %r14
140979662: 48 3b 58 20                 	cmpq	0x20(%rax), %rbx
140979666: 75 1e                       	jne	0x140979686 <.text+0x978686>
140979668: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
14097966f: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
140979673: 40 88 bd 17 02 00 00        	movb	%dil, 0x217(%rbp)
14097967a: 48 8d 15 c7 0a 2c 02        	leaq	0x22c0ac7(%rip), %rdx   # 0x142c3a148
140979681: e8 2a 97 be ff              	callq	0x140562db0 <.text+0x561db0>
140979686: 4c 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %r8
14097968d: 49 8b 40 28                 	movq	0x28(%r8), %rax
140979691: 48 8d 0c 5b                 	leaq	(%rbx,%rbx,2), %rcx
140979695: 4c 89 34 c8                 	movq	%r14, (%rax,%rcx,8)
140979699: 48 8b 95 e0 01 00 00        	movq	0x1e0(%rbp), %rdx
1409796a0: 48 89 54 c8 08              	movq	%rdx, 0x8(%rax,%rcx,8)
1409796a5: c6 44 c8 10 00              	movb	$0x0, 0x10(%rax,%rcx,8)
1409796aa: 48 ff c3                    	incq	%rbx
1409796ad: 49 89 58 30                 	movq	%rbx, 0x30(%r8)
1409796b1: 49 ff 40 40                 	incq	0x40(%r8)
1409796b5: 48 8b 45 00                 	movq	(%rbp), %rax
1409796b9: 8b 8d 54 01 00 00           	movl	0x154(%rbp), %ecx
1409796bf: 89 08                       	movl	%ecx, (%rax)
1409796c1: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1409796c8: 48 8b 58 30                 	movq	0x30(%rax), %rbx
1409796cc: 4c 8b 70 40                 	movq	0x40(%rax), %r14
1409796d0: 48 3b 58 20                 	cmpq	0x20(%rax), %rbx
1409796d4: 75 1e                       	jne	0x1409796f4 <.text+0x9786f4>
1409796d6: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1409796dd: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
1409796e1: 40 88 bd 17 02 00 00        	movb	%dil, 0x217(%rbp)
1409796e8: 48 8d 15 59 0a 2c 02        	leaq	0x22c0a59(%rip), %rdx   # 0x142c3a148
1409796ef: e8 bc 90 be ff              	callq	0x1405627b0 <.text+0x5617b0>
1409796f4: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
1409796fb: 48 8b 42 28                 	movq	0x28(%rdx), %rax
1409796ff: 48 89 d9                    	movq	%rbx, %rcx
140979702: 48 c1 e1 06                 	shlq	$0x6, %rcx
140979706: c6 04 08 02                 	movb	$0x2, (%rax,%rcx)
14097970a: 4c 8b a5 e0 01 00 00        	movq	0x1e0(%rbp), %r12
140979711: 4c 89 64 08 08              	movq	%r12, 0x8(%rax,%rcx)
140979716: 4c 89 74 08 38              	movq	%r14, 0x38(%rax,%rcx)
14097971b: 48 ff c3                    	incq	%rbx
14097971e: 48 89 5a 30                 	movq	%rbx, 0x30(%rdx)
140979722: 48 ff 42 40                 	incq	0x40(%rdx)
140979726: 40 84 ff                    	testb	%dil, %dil
140979729: 48 8b bd 78 01 00 00        	movq	0x178(%rbp), %rdi
140979730: 74 37                       	je	0x140979769 <.text+0x978769>
140979732: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140979738: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
14097973f: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
140979746: e8 d5 a4 c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097974b: 48 85 ff                    	testq	%rdi, %rdi
14097974e: 74 19                       	je	0x140979769 <.text+0x978769>
140979750: 48 c1 e7 02                 	shlq	$0x2, %rdi
140979754: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14097975a: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
140979761: 48 89 fa                    	movq	%rdi, %rdx
140979764: e8 b7 a4 c9 ff              	callq	0x140613c20 <.text+0x612c20>
140979769: 40 84 f6                    	testb	%sil, %sil
14097976c: 0f 84 cc 00 00 00           	je	0x14097983e <.text+0x97883e>
140979772: 4c 8b 9d 38 01 00 00        	movq	0x138(%rbp), %r11
140979779: 49 8b 03                    	movq	(%r11), %rax
14097977c: 31 c9                       	xorl	%ecx, %ecx
14097977e: 48 89 c2                    	movq	%rax, %rdx
140979781: 48 8b 75 f0                 	movq	-0x10(%rbp), %rsi
140979785: 48 2b 56 18                 	subq	0x18(%rsi), %rdx
140979789: 48 0f 42 d1                 	cmovbq	%rcx, %rdx
14097978d: 48 2b 46 38                 	subq	0x38(%rsi), %rax
140979791: 48 0f 42 c1                 	cmovbq	%rcx, %rax
140979795: 4c 8b 46 30                 	movq	0x30(%rsi), %r8
140979799: 49 29 c0                    	subq	%rax, %r8
14097979c: 4c 0f 42 c1                 	cmovbq	%rcx, %r8
1409797a0: 41 0f 96 c1                 	setbe	%r9b
1409797a4: 4c 8b 56 10                 	movq	0x10(%rsi), %r10
1409797a8: 49 29 d2                    	subq	%rdx, %r10
1409797ab: 4c 0f 42 d1                 	cmovbq	%rcx, %r10
1409797af: 48 8b 46 40                 	movq	0x40(%rsi), %rax
1409797b3: 0f 96 c1                    	setbe	%cl
1409797b6: 4d 01 c2                    	addq	%r8, %r10
1409797b9: 48 89 c2                    	movq	%rax, %rdx
1409797bc: 4c 29 d2                    	subq	%r10, %rdx
1409797bf: 49 89 13                    	movq	%rdx, (%r11)
1409797c2: 44 84 c9                    	testb	%r9b, %cl
1409797c5: 75 77                       	jne	0x14097983e <.text+0x97883e>
1409797c7: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
1409797ce: 48 89 01                    	movq	%rax, (%rcx)
1409797d1: 48 8b 95 00 03 00 00        	movq	0x300(%rbp), %rdx
1409797d8: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409797db: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409797df: 89 01                       	movl	%eax, (%rcx)
1409797e1: 48 8b 32                    	movq	(%rdx), %rsi
1409797e4: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
1409797e8: 48 8b 5e 40                 	movq	0x40(%rsi), %rbx
1409797ec: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
1409797f0: 75 10                       	jne	0x140979802 <.text+0x978802>
1409797f2: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1409797f6: 48 8d 15 4b 09 2c 02        	leaq	0x22c094b(%rip), %rdx   # 0x142c3a148
1409797fd: e8 2e 94 be ff              	callq	0x140562c30 <.text+0x561c30>
140979802: 48 8b 46 28                 	movq	0x28(%rsi), %rax
140979806: 48 89 f9                    	movq	%rdi, %rcx
140979809: 48 c1 e1 05                 	shlq	$0x5, %rcx
14097980d: 48 89 1c 08                 	movq	%rbx, (%rax,%rcx)
140979811: 4c 89 64 08 08              	movq	%r12, 0x8(%rax,%rcx)
140979816: 48 8b 95 48 01 00 00        	movq	0x148(%rbp), %rdx
14097981d: 48 89 54 08 10              	movq	%rdx, 0x10(%rax,%rcx)
140979822: 8b 95 50 01 00 00           	movl	0x150(%rbp), %edx
140979828: 89 54 08 18                 	movl	%edx, 0x18(%rax,%rcx)
14097982c: 66 c7 44 08 1c 01 00        	movw	$0x1, 0x1c(%rax,%rcx)
140979833: 48 ff c7                    	incq	%rdi
140979836: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
14097983a: 48 ff 46 40                 	incq	0x40(%rsi)
14097983e: 0f 28 b5 20 02 00 00        	movaps	0x220(%rbp), %xmm6
140979845: 0f 28 bd 30 02 00 00        	movaps	0x230(%rbp), %xmm7
14097984c: 44 0f 28 85 40 02 00 00     	movaps	0x240(%rbp), %xmm8
140979854: 44 0f 28 8d 50 02 00 00     	movaps	0x250(%rbp), %xmm9
14097985c: 44 0f 28 95 60 02 00 00     	movaps	0x260(%rbp), %xmm10
140979864: 48 81 c4 f8 02 00 00        	addq	$0x2f8, %rsp            # imm = 0x2F8
14097986b: 5b                          	popq	%rbx
14097986c: 5f                          	popq	%rdi
14097986d: 5e                          	popq	%rsi
14097986e: 41 5c                       	popq	%r12
140979870: 41 5d                       	popq	%r13
140979872: 41 5e                       	popq	%r14
140979874: 41 5f                       	popq	%r15
140979876: 5d                          	popq	%rbp
140979877: c3                          	retq
140979878: 48 8d 0d 14 09 16 02        	leaq	0x2160914(%rip), %rcx   # 0x142ada193
14097987f: 4c 8d 05 62 d2 13 02        	leaq	0x213d262(%rip), %r8    # 0x142ab6ae8
140979886: ba 32 00 00 00              	movl	$0x32, %edx
14097988b: e8 a0 fb f5 01              	callq	0x1428d9430 <.text+0x28d8430>
140979890: c6 85 16 02 00 00 01        	movb	$0x1, 0x216(%rbp)
140979897: 48 8d 0d a3 cf 13 02        	leaq	0x213cfa3(%rip), %rcx   # 0x142ab6841
14097989e: 4c 8d 05 a3 d2 13 02        	leaq	0x213d2a3(%rip), %r8    # 0x142ab6b48
1409798a5: ba 28 00 00 00              	movl	$0x28, %edx
1409798aa: e8 81 fb f5 01              	callq	0x1428d9430 <.text+0x28d8430>
1409798af: e9 fb 00 00 00              	jmp	0x1409799af <.text+0x9789af>
1409798b4: 4c 8d 05 9d 9d 2e 02        	leaq	0x22e9d9d(%rip), %r8    # 0x142c63658
1409798bb: e8 58 fc f5 01              	callq	0x1428d9518 <.text+0x28d8518>
1409798c0: 4c 8d 05 a9 9d 2e 02        	leaq	0x22e9da9(%rip), %r8    # 0x142c63670
1409798c7: 48 89 c1                    	movq	%rax, %rcx
1409798ca: e8 49 fc f5 01              	callq	0x1428d9518 <.text+0x28d8518>
1409798cf: 4c 8d 05 a2 f2 2b 02        	leaq	0x22bf2a2(%rip), %r8    # 0x142c38b78
1409798d6: 48 89 f1                    	movq	%rsi, %rcx
1409798d9: 4c 89 f2                    	movq	%r14, %rdx
1409798dc: e8 02 f7 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409798e1: c6 85 16 02 00 00 00        	movb	$0x0, 0x216(%rbp)
1409798e8: 4c 8d 05 89 f2 2b 02        	leaq	0x22bf289(%rip), %r8    # 0x142c38b78
1409798ef: 4c 89 f9                    	movq	%r15, %rcx
1409798f2: 48 89 da                    	movq	%rbx, %rdx
1409798f5: e8 e9 f6 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409798fa: e9 b0 00 00 00              	jmp	0x1409799af <.text+0x9789af>
1409798ff: 4c 8d 05 72 f2 2b 02        	leaq	0x22bf272(%rip), %r8    # 0x142c38b78
140979906: 48 89 f1                    	movq	%rsi, %rcx
140979909: 4c 89 f2                    	movq	%r14, %rdx
14097990c: e8 d2 f6 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140979911: e9 99 00 00 00              	jmp	0x1409799af <.text+0x9789af>
140979916: 4c 8d 05 5b f2 2b 02        	leaq	0x22bf25b(%rip), %r8    # 0x142c38b78
14097991d: 4c 89 f9                    	movq	%r15, %rcx
140979920: 4c 89 ea                    	movq	%r13, %rdx
140979923: e8 bb f6 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140979928: 4c 8d 05 49 f2 2b 02        	leaq	0x22bf249(%rip), %r8    # 0x142c38b78
14097992f: 4c 89 f9                    	movq	%r15, %rcx
140979932: 4c 89 ea                    	movq	%r13, %rdx
140979935: e8 a9 f6 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14097993a: eb 73                       	jmp	0x1409799af <.text+0x9789af>
14097993c: c6 85 16 02 00 00 00        	movb	$0x0, 0x216(%rbp)
140979943: 48 8d 0d 16 d2 13 02        	leaq	0x213d216(%rip), %rcx   # 0x142ab6b60
14097994a: e8 c1 f9 f5 01              	callq	0x1428d9310 <.text+0x28d8310>
14097994f: eb 5e                       	jmp	0x1409799af <.text+0x9789af>
140979951: 4c 8d 05 20 f2 2b 02        	leaq	0x22bf220(%rip), %r8    # 0x142c38b78
140979958: 4c 89 f1                    	movq	%r14, %rcx
14097995b: 48 89 fa                    	movq	%rdi, %rdx
14097995e: e8 80 f6 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140979963: eb 4a                       	jmp	0x1409799af <.text+0x9789af>
140979965: 4c 8d 05 0c f2 2b 02        	leaq	0x22bf20c(%rip), %r8    # 0x142c38b78
14097996c: 4c 89 f9                    	movq	%r15, %rcx
14097996f: 48 89 fa                    	movq	%rdi, %rdx
140979972: e8 6c f6 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140979977: eb 36                       	jmp	0x1409799af <.text+0x9789af>
140979979: 48 8d 0d 10 d2 13 02        	leaq	0x213d210(%rip), %rcx   # 0x142ab6b90
140979980: e8 8b f9 f5 01              	callq	0x1428d9310 <.text+0x28d8310>
140979985: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
14097998c: 4c 8d 05 e5 f1 2b 02        	leaq	0x22bf1e5(%rip), %r8    # 0x142c38b78
140979993: 4c 89 e2                    	movq	%r12, %rdx
140979996: e8 48 f6 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14097999b: eb 12                       	jmp	0x1409799af <.text+0x9789af>
14097999d: 4c 8d 05 d4 f1 2b 02        	leaq	0x22bf1d4(%rip), %r8    # 0x142c38b78
1409799a4: 4c 89 e9                    	movq	%r13, %rcx
1409799a7: 4c 89 e2                    	movq	%r12, %rdx
1409799aa: e8 34 f6 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409799af: 0f 0b                       	ud2
1409799b1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409799c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409799c5: 55                          	pushq	%rbp
1409799c6: 41 57                       	pushq	%r15
1409799c8: 41 56                       	pushq	%r14
1409799ca: 41 55                       	pushq	%r13
1409799cc: 41 54                       	pushq	%r12
1409799ce: 56                          	pushq	%rsi
1409799cf: 57                          	pushq	%rdi
1409799d0: 53                          	pushq	%rbx
1409799d1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1409799d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409799df: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
1409799e5: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
1409799eb: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
1409799f1: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
1409799f6: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
1409799fe: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140979a04: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
140979a0b: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
140979a12: e8 09 a2 c9 ff              	callq	0x140613c20 <.text+0x612c20>
140979a17: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
140979a1f: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
140979a24: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
140979a2a: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
140979a30: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
140979a36: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
140979a3d: 5b                          	popq	%rbx
140979a3e: 5f                          	popq	%rdi
140979a3f: 5e                          	popq	%rsi
140979a40: 41 5c                       	popq	%r12
140979a42: 41 5d                       	popq	%r13
140979a44: 41 5e                       	popq	%r14
140979a46: 41 5f                       	popq	%r15
140979a48: 5d                          	popq	%rbp
140979a49: c3                          	retq
140979a4a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140979a50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140979a55: 55                          	pushq	%rbp
140979a56: 41 57                       	pushq	%r15
140979a58: 41 56                       	pushq	%r14
140979a5a: 41 55                       	pushq	%r13
140979a5c: 41 54                       	pushq	%r12
140979a5e: 56                          	pushq	%rsi
140979a5f: 57                          	pushq	%rdi
140979a60: 53                          	pushq	%rbx
140979a61: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140979a68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140979a6f: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
140979a75: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
140979a7b: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
140979a81: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
140979a86: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
140979a8e: 48 83 bd 00 02 00 00 00     	cmpq	$0x0, 0x200(%rbp)
140979a96: 74 21                       	je	0x140979ab9 <.text+0x978ab9>
140979a98: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
140979a9f: 48 c1 e0 02                 	shlq	$0x2, %rax
140979aa3: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140979aa7: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140979aad: 48 8b 8d e8 01 00 00        	movq	0x1e8(%rbp), %rcx
140979ab4: e8 67 a1 c9 ff              	callq	0x140613c20 <.text+0x612c20>
140979ab9: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
140979ac1: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
140979ac6: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
140979acc: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
140979ad2: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
140979ad8: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
140979adf: 5b                          	popq	%rbx
140979ae0: 5f                          	popq	%rdi
140979ae1: 5e                          	popq	%rsi
140979ae2: 41 5c                       	popq	%r12
140979ae4: 41 5d                       	popq	%r13
140979ae6: 41 5e                       	popq	%r14
140979ae8: 41 5f                       	popq	%r15
140979aea: 5d                          	popq	%rbp
140979aeb: c3                          	retq
140979aec: 0f 1f 40 00                 	nopl	(%rax)
140979af0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140979af5: 55                          	pushq	%rbp
140979af6: 41 57                       	pushq	%r15
140979af8: 41 56                       	pushq	%r14
140979afa: 41 55                       	pushq	%r13
140979afc: 41 54                       	pushq	%r12
140979afe: 56                          	pushq	%rsi
140979aff: 57                          	pushq	%rdi
140979b00: 53                          	pushq	%rbx
140979b01: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140979b08: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140979b0f: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
140979b15: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
140979b1b: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
140979b21: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
140979b26: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
140979b2e: 48 83 bd 00 02 00 00 00     	cmpq	$0x0, 0x200(%rbp)
140979b36: 74 1d                       	je	0x140979b55 <.text+0x978b55>
140979b38: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
140979b3f: 48 c1 e2 03                 	shlq	$0x3, %rdx
140979b43: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140979b49: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
140979b50: e8 cb a0 c9 ff              	callq	0x140613c20 <.text+0x612c20>
140979b55: c6 85 16 02 00 00 00        	movb	$0x0, 0x216(%rbp)
140979b5c: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
140979b64: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
140979b69: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
140979b6f: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
140979b75: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
140979b7b: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
140979b82: 5b                          	popq	%rbx
140979b83: 5f                          	popq	%rdi
140979b84: 5e                          	popq	%rsi
140979b85: 41 5c                       	popq	%r12
140979b87: 41 5d                       	popq	%r13
140979b89: 41 5e                       	popq	%r14
140979b8b: 41 5f                       	popq	%r15
140979b8d: 5d                          	popq	%rbp
140979b8e: c3                          	retq
140979b8f: 90                          	nop
140979b90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140979b95: 55                          	pushq	%rbp
140979b96: 41 57                       	pushq	%r15
140979b98: 41 56                       	pushq	%r14
140979b9a: 41 55                       	pushq	%r13
140979b9c: 41 54                       	pushq	%r12
140979b9e: 56                          	pushq	%rsi
140979b9f: 57                          	pushq	%rdi
140979ba0: 53                          	pushq	%rbx
140979ba1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140979ba8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140979baf: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
140979bb5: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
140979bbb: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
140979bc1: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
140979bc6: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
140979bce: 80 bd 16 02 00 00 00        	cmpb	$0x0, 0x216(%rbp)
140979bd5: 74 26                       	je	0x140979bfd <.text+0x978bfd>
140979bd7: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
140979bde: 48 85 c0                    	testq	%rax, %rax
140979be1: 74 1a                       	je	0x140979bfd <.text+0x978bfd>
140979be3: 48 8b 8d 98 01 00 00        	movq	0x198(%rbp), %rcx
140979bea: 48 c1 e0 02                 	shlq	$0x2, %rax
140979bee: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140979bf2: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140979bf8: e8 23 a0 c9 ff              	callq	0x140613c20 <.text+0x612c20>
140979bfd: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
140979c05: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
140979c0a: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
140979c10: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
140979c16: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
140979c1c: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
140979c23: 5b                          	popq	%rbx
140979c24: 5f                          	popq	%rdi
140979c25: 5e                          	popq	%rsi
140979c26: 41 5c                       	popq	%r12
140979c28: 41 5d                       	popq	%r13
140979c2a: 41 5e                       	popq	%r14
140979c2c: 41 5f                       	popq	%r15
140979c2e: 5d                          	popq	%rbp
140979c2f: c3                          	retq
140979c30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140979c35: 55                          	pushq	%rbp
140979c36: 41 57                       	pushq	%r15
140979c38: 41 56                       	pushq	%r14
140979c3a: 41 55                       	pushq	%r13
140979c3c: 41 54                       	pushq	%r12
140979c3e: 56                          	pushq	%rsi
140979c3f: 57                          	pushq	%rdi
140979c40: 53                          	pushq	%rbx
140979c41: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140979c48: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140979c4f: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
140979c55: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
140979c5b: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
140979c61: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
140979c66: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
140979c6e: 48 83 bd 08 02 00 00 00     	cmpq	$0x0, 0x208(%rbp)
140979c76: 74 21                       	je	0x140979c99 <.text+0x978c99>
140979c78: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
140979c7f: 48 c1 e0 02                 	shlq	$0x2, %rax
140979c83: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140979c87: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140979c8d: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
140979c94: e8 87 9f c9 ff              	callq	0x140613c20 <.text+0x612c20>
140979c99: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
140979ca0: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
140979ca8: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
140979cad: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
140979cb3: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
140979cb9: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
140979cbf: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
140979cc6: 5b                          	popq	%rbx
140979cc7: 5f                          	popq	%rdi
140979cc8: 5e                          	popq	%rsi
140979cc9: 41 5c                       	popq	%r12
140979ccb: 41 5d                       	popq	%r13
140979ccd: 41 5e                       	popq	%r14
140979ccf: 41 5f                       	popq	%r15
140979cd1: 5d                          	popq	%rbp
140979cd2: c3                          	retq
140979cd3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140979ce0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140979ce5: 55                          	pushq	%rbp
140979ce6: 41 57                       	pushq	%r15
140979ce8: 41 56                       	pushq	%r14
140979cea: 41 55                       	pushq	%r13
140979cec: 41 54                       	pushq	%r12
140979cee: 56                          	pushq	%rsi
140979cef: 57                          	pushq	%rdi
140979cf0: 53                          	pushq	%rbx
140979cf1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140979cf8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140979cff: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
140979d05: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
140979d0b: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
140979d11: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
140979d16: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
140979d1e: 48 83 bd 08 02 00 00 00     	cmpq	$0x0, 0x208(%rbp)
140979d26: 74 1d                       	je	0x140979d45 <.text+0x978d45>
140979d28: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
140979d2f: 48 c1 e2 03                 	shlq	$0x3, %rdx
140979d33: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140979d39: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
140979d40: e8 db 9e c9 ff              	callq	0x140613c20 <.text+0x612c20>
140979d45: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
140979d4c: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
140979d54: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
140979d59: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
140979d5f: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
140979d65: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
140979d6b: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
140979d72: 5b                          	popq	%rbx
140979d73: 5f                          	popq	%rdi
140979d74: 5e                          	popq	%rsi
140979d75: 41 5c                       	popq	%r12
140979d77: 41 5d                       	popq	%r13
140979d79: 41 5e                       	popq	%r14
140979d7b: 41 5f                       	popq	%r15
140979d7d: 5d                          	popq	%rbp
140979d7e: c3                          	retq
140979d7f: 90                          	nop
140979d80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140979d85: 55                          	pushq	%rbp
140979d86: 41 57                       	pushq	%r15
140979d88: 41 56                       	pushq	%r14
140979d8a: 41 55                       	pushq	%r13
140979d8c: 41 54                       	pushq	%r12
140979d8e: 56                          	pushq	%rsi
140979d8f: 57                          	pushq	%rdi
140979d90: 53                          	pushq	%rbx
140979d91: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140979d98: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140979d9f: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
140979da5: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
140979dab: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
140979db1: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
140979db6: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
140979dbe: f6 85 17 02 00 00 01        	testb	$0x1, 0x217(%rbp)
140979dc5: 75 33                       	jne	0x140979dfa <.text+0x978dfa>
140979dc7: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
140979dcf: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
140979dd4: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
140979dda: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
140979de0: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
140979de6: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
140979ded: 5b                          	popq	%rbx
140979dee: 5f                          	popq	%rdi
140979def: 5e                          	popq	%rsi
140979df0: 41 5c                       	popq	%r12
140979df2: 41 5d                       	popq	%r13
140979df4: 41 5e                       	popq	%r14
140979df6: 41 5f                       	popq	%r15
140979df8: 5d                          	popq	%rbp
140979df9: c3                          	retq
140979dfa: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
140979e01: e8 ea 86 c5 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
140979e06: eb bf                       	jmp	0x140979dc7 <.text+0x978dc7>
140979e08: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140979e10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140979e15: 55                          	pushq	%rbp
140979e16: 41 57                       	pushq	%r15
140979e18: 41 56                       	pushq	%r14
140979e1a: 41 55                       	pushq	%r13
140979e1c: 41 54                       	pushq	%r12
140979e1e: 56                          	pushq	%rsi
140979e1f: 57                          	pushq	%rdi
140979e20: 53                          	pushq	%rbx
140979e21: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140979e28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140979e2f: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
140979e35: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
140979e3b: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
140979e41: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
140979e46: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
140979e4e: 48 83 bd 08 02 00 00 00     	cmpq	$0x0, 0x208(%rbp)
140979e56: 74 1d                       	je	0x140979e75 <.text+0x978e75>
140979e58: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
140979e5f: 48 c1 e2 03                 	shlq	$0x3, %rdx
140979e63: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140979e69: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
140979e70: e8 ab 9d c9 ff              	callq	0x140613c20 <.text+0x612c20>
140979e75: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
140979e7d: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
140979e82: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
140979e88: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
140979e8e: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
140979e94: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
140979e9b: 5b                          	popq	%rbx
140979e9c: 5f                          	popq	%rdi
140979e9d: 5e                          	popq	%rsi
140979e9e: 41 5c                       	popq	%r12
140979ea0: 41 5d                       	popq	%r13
140979ea2: 41 5e                       	popq	%r14
140979ea4: 41 5f                       	popq	%r15
140979ea6: 5d                          	popq	%rbp
140979ea7: c3                          	retq
140979ea8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140979eb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140979eb5: 55                          	pushq	%rbp
140979eb6: 41 57                       	pushq	%r15
140979eb8: 41 56                       	pushq	%r14
140979eba: 41 55                       	pushq	%r13
140979ebc: 41 54                       	pushq	%r12
140979ebe: 56                          	pushq	%rsi
140979ebf: 57                          	pushq	%rdi
140979ec0: 53                          	pushq	%rbx
140979ec1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140979ec8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140979ecf: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
140979ed5: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
140979edb: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
140979ee1: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
140979ee6: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
140979eee: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
140979ef5: e8 f6 85 c5 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
140979efa: c6 85 15 02 00 00 01        	movb	$0x1, 0x215(%rbp)
140979f01: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
140979f09: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
140979f0e: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
140979f14: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
140979f1a: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
140979f20: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
140979f27: 5b                          	popq	%rbx
140979f28: 5f                          	popq	%rdi
140979f29: 5e                          	popq	%rsi
140979f2a: 41 5c                       	popq	%r12
140979f2c: 41 5d                       	popq	%r13
140979f2e: 41 5e                       	popq	%r14
140979f30: 41 5f                       	popq	%r15
140979f32: 5d                          	popq	%rbp
140979f33: c3                          	retq
140979f34: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140979f40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140979f45: 55                          	pushq	%rbp
140979f46: 41 57                       	pushq	%r15
140979f48: 41 56                       	pushq	%r14
140979f4a: 41 55                       	pushq	%r13
140979f4c: 41 54                       	pushq	%r12
140979f4e: 56                          	pushq	%rsi
140979f4f: 57                          	pushq	%rdi
140979f50: 53                          	pushq	%rbx
140979f51: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140979f58: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140979f5f: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
140979f65: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
140979f6b: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
140979f71: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
140979f76: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
140979f7e: c6 85 15 02 00 00 00        	movb	$0x0, 0x215(%rbp)
140979f85: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
140979f8c: e8 df e4 f1 ff              	callq	0x140898470 <.text+0x897470>
140979f91: c6 85 15 02 00 00 00        	movb	$0x0, 0x215(%rbp)
140979f98: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
140979fa0: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
140979fa5: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
140979fab: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
140979fb1: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
140979fb7: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
140979fbe: 5b                          	popq	%rbx
140979fbf: 5f                          	popq	%rdi
140979fc0: 5e                          	popq	%rsi
140979fc1: 41 5c                       	popq	%r12
140979fc3: 41 5d                       	popq	%r13
140979fc5: 41 5e                       	popq	%r14
140979fc7: 41 5f                       	popq	%r15
140979fc9: 5d                          	popq	%rbp
140979fca: c3                          	retq
140979fcb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140979fd0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140979fd5: 55                          	pushq	%rbp
140979fd6: 41 57                       	pushq	%r15
140979fd8: 41 56                       	pushq	%r14
140979fda: 41 55                       	pushq	%r13
140979fdc: 41 54                       	pushq	%r12
140979fde: 56                          	pushq	%rsi
140979fdf: 57                          	pushq	%rdi
140979fe0: 53                          	pushq	%rbx
140979fe1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140979fe8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140979fef: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
140979ff5: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
140979ffb: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
14097a001: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
14097a006: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
14097a00e: 80 bd 15 02 00 00 00        	cmpb	$0x0, 0x215(%rbp)
14097a015: 75 33                       	jne	0x14097a04a <.text+0x97904a>
14097a017: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14097a01f: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
14097a024: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14097a02a: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14097a030: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
14097a036: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14097a03d: 5b                          	popq	%rbx
14097a03e: 5f                          	popq	%rdi
14097a03f: 5e                          	popq	%rsi
14097a040: 41 5c                       	popq	%r12
14097a042: 41 5d                       	popq	%r13
14097a044: 41 5e                       	popq	%r14
14097a046: 41 5f                       	popq	%r15
14097a048: 5d                          	popq	%rbp
14097a049: c3                          	retq
14097a04a: 48 8d 8d 18 01 00 00        	leaq	0x118(%rbp), %rcx
14097a051: e8 1a e4 f1 ff              	callq	0x140898470 <.text+0x897470>
14097a056: eb bf                       	jmp	0x14097a017 <.text+0x979017>
14097a058: cc                          	int3
14097a059: cc                          	int3
14097a05a: cc                          	int3
14097a05b: cc                          	int3
14097a05c: cc                          	int3
14097a05d: cc                          	int3
14097a05e: cc                          	int3
14097a05f: cc                          	int3
