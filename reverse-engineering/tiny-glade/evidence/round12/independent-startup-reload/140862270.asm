
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140862270: 55                          	pushq	%rbp
140862271: 41 57                       	pushq	%r15
140862273: 41 56                       	pushq	%r14
140862275: 41 55                       	pushq	%r13
140862277: 41 54                       	pushq	%r12
140862279: 56                          	pushq	%rsi
14086227a: 57                          	pushq	%rdi
14086227b: 53                          	pushq	%rbx
14086227c: 48 81 ec 28 03 00 00        	subq	$0x328, %rsp            # imm = 0x328
140862283: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14086228b: 44 0f 29 b5 90 02 00 00     	movaps	%xmm14, 0x290(%rbp)
140862293: 44 0f 29 ad 80 02 00 00     	movaps	%xmm13, 0x280(%rbp)
14086229b: 44 0f 29 a5 70 02 00 00     	movaps	%xmm12, 0x270(%rbp)
1408622a3: 44 0f 29 9d 60 02 00 00     	movaps	%xmm11, 0x260(%rbp)
1408622ab: 44 0f 29 95 50 02 00 00     	movaps	%xmm10, 0x250(%rbp)
1408622b3: 44 0f 29 8d 40 02 00 00     	movaps	%xmm9, 0x240(%rbp)
1408622bb: 44 0f 29 85 30 02 00 00     	movaps	%xmm8, 0x230(%rbp)
1408622c3: 66 0f 7f bd 20 02 00 00     	movdqa	%xmm7, 0x220(%rbp)
1408622cb: 66 0f 7f b5 10 02 00 00     	movdqa	%xmm6, 0x210(%rbp)
1408622d3: 48 c7 85 08 02 00 00 fe ff ff ff    	movq	$-0x2, 0x208(%rbp)
1408622de: 4c 89 cb                    	movq	%r9, %rbx
1408622e1: 49 89 d6                    	movq	%rdx, %r14
1408622e4: 49 8b 00                    	movq	(%r8), %rax
1408622e7: 49 8b 50 08                 	movq	0x8(%r8), %rdx
1408622eb: 4c 8b 00                    	movq	(%rax), %r8
1408622ee: 45 31 c9                    	xorl	%r9d, %r9d
1408622f1: 4d 89 c2                    	movq	%r8, %r10
1408622f4: 4c 2b 52 18                 	subq	0x18(%rdx), %r10
1408622f8: 4d 0f 42 d1                 	cmovbq	%r9, %r10
1408622fc: 4c 2b 42 38                 	subq	0x38(%rdx), %r8
140862300: 4d 0f 42 c1                 	cmovbq	%r9, %r8
140862304: 4c 8b 5a 30                 	movq	0x30(%rdx), %r11
140862308: 4d 29 c3                    	subq	%r8, %r11
14086230b: 4d 0f 42 d9                 	cmovbq	%r9, %r11
14086230f: 41 0f 97 c0                 	seta	%r8b
140862313: 48 8b 72 10                 	movq	0x10(%rdx), %rsi
140862317: 4c 29 d6                    	subq	%r10, %rsi
14086231a: 49 0f 42 f1                 	cmovbq	%r9, %rsi
14086231e: 48 8b 52 40                 	movq	0x40(%rdx), %rdx
140862322: 41 0f 97 c1                 	seta	%r9b
140862326: 4c 01 de                    	addq	%r11, %rsi
140862329: 49 89 d2                    	movq	%rdx, %r10
14086232c: 49 29 f2                    	subq	%rsi, %r10
14086232f: 4c 89 10                    	movq	%r10, (%rax)
140862332: 45 08 c1                    	orb	%r8b, %r9b
140862335: 0f 84 ad 0a 00 00           	je	0x140862de8 <.text+0x861de8>
14086233b: 48 89 10                    	movq	%rdx, (%rax)
14086233e: 48 8b 05 63 a0 dc 02        	movq	0x2dca063(%rip), %rax   # 0x14362c3a8
140862345: 48 8b 00                    	movq	(%rax), %rax
140862348: 48 83 f8 02                 	cmpq	$0x2, %rax
14086234c: 0f 86 c3 00 00 00           	jbe	0x140862415 <.text+0x861415>
140862352: 48 8d 05 e7 3d 23 02        	leaq	0x2233de7(%rip), %rax   # 0x142a96140
140862359: 48 89 ce                    	movq	%rcx, %rsi
14086235c: 48 89 c1                    	movq	%rax, %rcx
14086235f: e8 1c ba e1 ff              	callq	0x14067dd80 <.text+0x67cd80>
140862364: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
140862368: 8b 40 10                    	movl	0x10(%rax), %eax
14086236b: 48 c7 85 08 01 00 00 03 00 00 00    	movq	$0x3, 0x108(%rbp)
140862376: 48 8d 0d db 3d 23 02        	leaq	0x2233ddb(%rip), %rcx   # 0x142a96158
14086237d: 48 89 8d 10 01 00 00        	movq	%rcx, 0x110(%rbp)
140862384: 48 c7 85 18 01 00 00 2c 00 00 00    	movq	$0x2c, 0x118(%rbp)
14086238f: 48 8d 15 62 3d 23 02        	leaq	0x2233d62(%rip), %rdx   # 0x142a960f8
140862396: 48 89 95 28 01 00 00        	movq	%rdx, 0x128(%rbp)
14086239d: 48 c7 85 30 01 00 00 01 00 00 00    	movq	$0x1, 0x130(%rbp)
1408623a8: 48 c7 85 38 01 00 00 08 00 00 00    	movq	$0x8, 0x138(%rbp)
1408623b3: 48 c7 85 d8 00 00 00 00 00 00 00    	movq	$0x0, 0xd8(%rbp)
1408623be: 66 0f ef c9                 	pxor	%xmm1, %xmm1
1408623c2: f3 0f 7f 8d 40 01 00 00     	movdqu	%xmm1, 0x140(%rbp)
1408623ca: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
1408623d1: 48 c7 85 e8 00 00 00 2c 00 00 00    	movq	$0x2c, 0xe8(%rbp)
1408623dc: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
1408623e7: f3 0f 7f 85 f8 00 00 00     	movdqu	%xmm0, 0xf8(%rbp)
1408623ef: c7 85 20 01 00 00 01 00 00 00       	movl	$0x1, 0x120(%rbp)
1408623f9: 89 85 24 01 00 00           	movl	%eax, 0x124(%rbp)
1408623ff: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
140862406: 48 8d 95 d8 00 00 00        	leaq	0xd8(%rbp), %rdx
14086240d: e8 1e 7f 4e 00              	callq	0x140d4a330 <.text+0xd49330>
140862412: 48 89 f1                    	movq	%rsi, %rcx
140862415: 48 8b 01                    	movq	(%rcx), %rax
140862418: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14086241c: 0f b6 40 58                 	movzbl	0x58(%rax), %eax
140862420: c1 e0 03                    	shll	$0x3, %eax
140862423: 48 8d 0d 66 42 23 02        	leaq	0x2234266(%rip), %rcx   # 0x142a96690
14086242a: 4c 8b 0c 08                 	movq	(%rax,%rcx), %r9
14086242e: 48 8d 0d 83 42 23 02        	leaq	0x2234283(%rip), %rcx   # 0x142a966b8
140862435: 48 8b 04 08                 	movq	(%rax,%rcx), %rax
140862439: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14086243e: 48 8d 15 d5 d0 28 02        	leaq	0x228d0d5(%rip), %rdx   # 0x142aef51a
140862445: 48 8d bd d8 00 00 00        	leaq	0xd8(%rbp), %rdi
14086244c: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
140862452: 48 89 f9                    	movq	%rdi, %rcx
140862455: e8 b6 03 50 00              	callq	0x140d62810 <.text+0xd61810>
14086245a: 48 89 f9                    	movq	%rdi, %rcx
14086245d: e8 6e 4a 51 00              	callq	0x140d76ed0 <.text+0xd75ed0>
140862462: 49 89 d0                    	movq	%rdx, %r8
140862465: 48 c7 44 24 20 0a 00 00 00  	movq	$0xa, 0x20(%rsp)
14086246e: 4c 8d 0d 0f 3d 23 02        	leaq	0x2233d0f(%rip), %r9    # 0x142a96184
140862475: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
14086247c: 48 89 c2                    	movq	%rax, %rdx
14086247f: e8 8c 03 50 00              	callq	0x140d62810 <.text+0xd61810>
140862484: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
14086248b: e8 40 4a 51 00              	callq	0x140d76ed0 <.text+0xd75ed0>
140862490: 49 89 d0                    	movq	%rdx, %r8
140862493: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140862497: 48 89 c2                    	movq	%rax, %rdx
14086249a: e8 a1 6c 4f 00              	callq	0x140d59140 <.text+0xd58140>
14086249f: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
1408624a6: 48 85 d2                    	testq	%rdx, %rdx
1408624a9: 74 12                       	je	0x1408624bd <.text+0x8614bd>
1408624ab: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
1408624b2: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408624b8: e8 63 17 db ff              	callq	0x140613c20 <.text+0x612c20>
1408624bd: 48 8b 45 50                 	movq	0x50(%rbp), %rax
1408624c1: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1408624c8: 48 f7 d8                    	negq	%rax
1408624cb: 0f 80 8e 09 00 00           	jo	0x140862e5f <.text+0x861e5f>
1408624d1: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
1408624d5: 4c 8b 45 60                 	movq	0x60(%rbp), %r8
1408624d9: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
1408624e0: 48 89 95 e8 01 00 00        	movq	%rdx, 0x1e8(%rbp)
1408624e7: e8 c4 3d ba 00              	callq	0x1414062b0 <.text+0x14052b0>
1408624ec: 48 8b b5 58 01 00 00        	movq	0x158(%rbp), %rsi
1408624f3: 48 89 f0                    	movq	%rsi, %rax
1408624f6: 48 f7 d8                    	negq	%rax
1408624f9: 0f 80 9b 09 00 00           	jo	0x140862e9a <.text+0x861e9a>
1408624ff: 4c 89 75 20                 	movq	%r14, 0x20(%rbp)
140862503: 4c 8b b5 60 01 00 00        	movq	0x160(%rbp), %r14
14086250a: 48 8b bd 68 01 00 00        	movq	0x168(%rbp), %rdi
140862511: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
140862518: 48 85 d2                    	testq	%rdx, %rdx
14086251b: 74 12                       	je	0x14086252f <.text+0x86152f>
14086251d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140862523: 48 8b 8d e8 01 00 00        	movq	0x1e8(%rbp), %rcx
14086252a: e8 f1 16 db ff              	callq	0x140613c20 <.text+0x612c20>
14086252f: 4c 8b bd 10 03 00 00        	movq	0x310(%rbp), %r15
140862536: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
14086253d: 48 85 d2                    	testq	%rdx, %rdx
140862540: 74 12                       	je	0x140862554 <.text+0x861554>
140862542: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140862549: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14086254f: e8 cc 16 db ff              	callq	0x140613c20 <.text+0x612c20>
140862554: 48 8d 3c bf                 	leaq	(%rdi,%rdi,4), %rdi
140862558: 48 c1 e7 04                 	shlq	$0x4, %rdi
14086255c: 4c 01 f7                    	addq	%r14, %rdi
14086255f: 4c 89 b5 a0 00 00 00        	movq	%r14, 0xa0(%rbp)
140862566: 48 89 b5 b0 00 00 00        	movq	%rsi, 0xb0(%rbp)
14086256d: 48 89 bd b8 00 00 00        	movq	%rdi, 0xb8(%rbp)
140862574: 48 8d b5 60 01 00 00        	leaq	0x160(%rbp), %rsi
14086257b: 8b 43 1c                    	movl	0x1c(%rbx), %eax
14086257e: 89 85 ac 01 00 00           	movl	%eax, 0x1ac(%rbp)
140862584: 48 8b 03                    	movq	(%rbx), %rax
140862587: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
14086258e: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140862592: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140862596: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
14086259a: 89 85 a8 01 00 00           	movl	%eax, 0x1a8(%rbp)
1408625a0: 49 8b 07                    	movq	(%r15), %rax
1408625a3: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
1408625a7: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
1408625ab: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1408625b2: 48 83 c0 20                 	addq	$0x20, %rax
1408625b6: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1408625ba: 45 31 ed                    	xorl	%r13d, %r13d
1408625bd: 66 0f 6f 35 bb 37 23 02     	movdqa	0x22337bb(%rip), %xmm6  # 0x142a95d80
1408625c5: 66 0f 6e 3d c3 37 23 02     	movd	0x22337c3(%rip), %xmm7  # 0x142a95d90
1408625cd: 4c 8d 3d 46 cf 28 02        	leaq	0x228cf46(%rip), %r15   # 0x142aef51a
1408625d4: 44 0f 28 05 a4 c3 0c 02     	movaps	0x20cc3a4(%rip), %xmm8  # 0x14292e980
1408625dc: f3 44 0f 10 0d 2f 12 22 02  	movss	0x222122f(%rip), %xmm9  # 0x142a83814
1408625e5: f2 44 0f 10 15 b2 37 23 02  	movsd	0x22337b2(%rip), %xmm10 # 0x142a95da0
1408625ee: f2 44 0f 10 1d b9 37 23 02  	movsd	0x22337b9(%rip), %xmm11 # 0x142a95db0
1408625f7: 45 0f 57 e4                 	xorps	%xmm12, %xmm12
1408625fb: 49 39 fe                    	cmpq	%rdi, %r14
1408625fe: 0f 84 ce 07 00 00           	je	0x140862dd2 <.text+0x861dd2>
140862604: 49 83 c6 50                 	addq	$0x50, %r14
140862608: eb 1b                       	jmp	0x140862625 <.text+0x861625>
14086260a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140862610: 48 8d 43 b0                 	leaq	-0x50(%rbx), %rax
140862614: 4c 8d 73 50                 	leaq	0x50(%rbx), %r14
140862618: 48 83 c0 50                 	addq	$0x50, %rax
14086261c: 48 39 f8                    	cmpq	%rdi, %rax
14086261f: 0f 84 b0 07 00 00           	je	0x140862dd5 <.text+0x861dd5>
140862625: 4c 89 f3                    	movq	%r14, %rbx
140862628: 49 8b 56 b0                 	movq	-0x50(%r14), %rdx
14086262c: 49 8b 46 f8                 	movq	-0x8(%r14), %rax
140862630: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
140862637: 41 0f 10 46 e8              	movups	-0x18(%r14), %xmm0
14086263c: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
140862643: 41 0f 10 46 d8              	movups	-0x28(%r14), %xmm0
140862648: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
14086264c: 41 0f 10 46 c8              	movups	-0x38(%r14), %xmm0
140862651: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
140862655: 41 0f 10 46 b8              	movups	-0x48(%r14), %xmm0
14086265a: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
14086265e: 48 89 d0                    	movq	%rdx, %rax
140862661: 48 f7 d8                    	negq	%rax
140862664: 0f 80 6b 07 00 00           	jo	0x140862dd5 <.text+0x861dd5>
14086266a: 48 89 95 58 01 00 00        	movq	%rdx, 0x158(%rbp)
140862671: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
140862678: 48 89 46 40                 	movq	%rax, 0x40(%rsi)
14086267c: 66 0f 6f 45 50              	movdqa	0x50(%rbp), %xmm0
140862681: 66 0f 6f 4d 60              	movdqa	0x60(%rbp), %xmm1
140862686: 0f 28 55 70                 	movaps	0x70(%rbp), %xmm2
14086268a: 0f 28 9d 80 00 00 00        	movaps	0x80(%rbp), %xmm3
140862691: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140862695: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140862699: f3 0f 7f 4e 10              	movdqu	%xmm1, 0x10(%rsi)
14086269e: f3 0f 7f 06                 	movdqu	%xmm0, (%rsi)
1408626a2: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
1408626a9: 4c 8b b5 68 01 00 00        	movq	0x168(%rbp), %r14
1408626b0: 49 83 fe 12                 	cmpq	$0x12, %r14
1408626b4: 0f 85 86 00 00 00           	jne	0x140862740 <.text+0x861740>
1408626ba: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
1408626be: 66 0f 74 c6                 	pcmpeqb	%xmm6, %xmm0
1408626c2: 0f b7 41 10                 	movzwl	0x10(%rcx), %eax
1408626c6: 66 0f 6e c8                 	movd	%eax, %xmm1
1408626ca: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
1408626ce: 66 0f db c8                 	pand	%xmm0, %xmm1
1408626d2: 66 0f d7 c1                 	pmovmskb	%xmm1, %eax
1408626d6: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1408626db: 75 63                       	jne	0x140862740 <.text+0x861740>
1408626dd: 48 85 d2                    	testq	%rdx, %rdx
1408626e0: 74 0b                       	je	0x1408626ed <.text+0x8616ed>
1408626e2: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408626e8: e8 33 15 db ff              	callq	0x140613c20 <.text+0x612c20>
1408626ed: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1408626f4: 48 85 d2                    	testq	%rdx, %rdx
1408626f7: 74 12                       	je	0x14086270b <.text+0x86170b>
1408626f9: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
140862700: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140862706: e8 15 15 db ff              	callq	0x140613c20 <.text+0x612c20>
14086270b: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
140862712: 48 89 d0                    	movq	%rdx, %rax
140862715: 48 f7 d8                    	negq	%rax
140862718: 0f 80 f2 fe ff ff           	jo	0x140862610 <.text+0x861610>
14086271e: 0f 83 ec fe ff ff           	jae	0x140862610 <.text+0x861610>
140862724: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
14086272b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140862731: e8 ea 14 db ff              	callq	0x140613c20 <.text+0x612c20>
140862736: e9 d5 fe ff ff              	jmp	0x140862610 <.text+0x861610>
14086273b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140862740: 4c 89 fa                    	movq	%r15, %rdx
140862743: 4d 89 ef                    	movq	%r13, %r15
140862746: 48 89 8d b8 01 00 00        	movq	%rcx, 0x1b8(%rbp)
14086274d: 48 89 9d a8 00 00 00        	movq	%rbx, 0xa8(%rbp)
140862754: 48 8b 9d 78 01 00 00        	movq	0x178(%rbp), %rbx
14086275b: 4c 8b a5 80 01 00 00        	movq	0x180(%rbp), %r12
140862762: 48 8b 45 18                 	movq	0x18(%rbp), %rax
140862766: 0f b6 40 58                 	movzbl	0x58(%rax), %eax
14086276a: c1 e0 03                    	shll	$0x3, %eax
14086276d: 48 8d 0d 1c 3f 23 02        	leaq	0x2233f1c(%rip), %rcx   # 0x142a96690
140862774: 4c 8b 0c 08                 	movq	(%rax,%rcx), %r9
140862778: 48 8d 0d 39 3f 23 02        	leaq	0x2233f39(%rip), %rcx   # 0x142a966b8
14086277f: 48 8b 04 08                 	movq	(%rax,%rcx), %rax
140862783: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140862788: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
14086278e: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
140862795: 48 89 cf                    	movq	%rcx, %rdi
140862798: e8 73 00 50 00              	callq	0x140d62810 <.text+0xd61810>
14086279d: 4c 8d 6d b0                 	leaq	-0x50(%rbp), %r13
1408627a1: 4c 89 e9                    	movq	%r13, %rcx
1408627a4: 48 89 da                    	movq	%rbx, %rdx
1408627a7: 4d 89 e0                    	movq	%r12, %r8
1408627aa: 49 89 f9                    	movq	%rdi, %r9
1408627ad: e8 2e 41 a8 00              	callq	0x1412e68e0 <.text+0x12e58e0>
1408627b2: 4c 89 6d 10                 	movq	%r13, 0x10(%rbp)
1408627b6: 4c 89 e9                    	movq	%r13, %rcx
1408627b9: e8 12 47 51 00              	callq	0x140d76ed0 <.text+0xd75ed0>
1408627be: 49 89 d0                    	movq	%rdx, %r8
1408627c1: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1408627c8: 48 89 c2                    	movq	%rax, %rdx
1408627cb: e8 70 69 4f 00              	callq	0x140d59140 <.text+0xd58140>
1408627d0: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1408627d7: 48 8b 95 c8 01 00 00        	movq	0x1c8(%rbp), %rdx
1408627de: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1408627e5: 48 f7 d8                    	negq	%rax
1408627e8: 0f 80 1f 07 00 00           	jo	0x140862f0d <.text+0x861f0d>
1408627ee: 4d 89 fd                    	movq	%r15, %r13
1408627f1: 4c 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %r8
1408627f8: 48 89 f9                    	movq	%rdi, %rcx
1408627fb: 48 89 95 d0 00 00 00        	movq	%rdx, 0xd0(%rbp)
140862802: e8 e9 34 ba 00              	callq	0x141405cf0 <.text+0x1404cf0>
140862807: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
14086280e: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
140862815: 48 f7 d8                    	negq	%rax
140862818: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
14086281f: 0f 80 ad 06 00 00           	jo	0x140862ed2 <.text+0x861ed2>
140862825: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
14086282c: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
140862833: 48 8b bd e8 00 00 00        	movq	0xe8(%rbp), %rdi
14086283a: 4c 89 f2                    	movq	%r14, %rdx
14086283d: e8 3e 38 cf 00              	callq	0x141556080 <.text+0x1555080>
140862842: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140862849: 48 89 95 f8 01 00 00        	movq	%rdx, 0x1f8(%rbp)
140862850: 8b 85 a0 01 00 00           	movl	0x1a0(%rbp), %eax
140862856: 89 85 b8 01 00 00           	movl	%eax, 0x1b8(%rbp)
14086285c: 48 8d 3c 7f                 	leaq	(%rdi,%rdi,2), %rdi
140862860: 48 c1 e7 04                 	shlq	$0x4, %rdi
140862864: 4c 8b bd b0 01 00 00        	movq	0x1b0(%rbp), %r15
14086286b: 4c 01 ff                    	addq	%r15, %rdi
14086286e: 66 90                       	nop
140862870: 49 39 ff                    	cmpq	%rdi, %r15
140862873: 0f 84 67 04 00 00           	je	0x140862ce0 <.text+0x861ce0>
140862879: 4c 89 f8                    	movq	%r15, %rax
14086287c: 49 83 c7 30                 	addq	$0x30, %r15
140862880: f3 0f 10 00                 	movss	(%rax), %xmm0
140862884: 41 0f 54 c0                 	andps	%xmm8, %xmm0
140862888: 44 0f 2e c8                 	ucomiss	%xmm0, %xmm9
14086288c: 76 e2                       	jbe	0x140862870 <.text+0x861870>
14086288e: f3 0f 10 40 08              	movss	0x8(%rax), %xmm0
140862893: 41 0f 54 c0                 	andps	%xmm8, %xmm0
140862897: 44 0f 2e c8                 	ucomiss	%xmm0, %xmm9
14086289b: 76 d3                       	jbe	0x140862870 <.text+0x861870>
14086289d: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
1408628a4: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
1408628a8: e8 f3 1d 4b 00              	callq	0x140d146a0 <.text+0xd136a0>
1408628ad: 48 8b 9d e0 00 00 00        	movq	0xe0(%rbp), %rbx
1408628b4: 48 85 db                    	testq	%rbx, %rbx
1408628b7: 75 07                       	jne	0x1408628c0 <.text+0x8618c0>
1408628b9: 48 8b 9d e8 00 00 00        	movq	0xe8(%rbp), %rbx
1408628c0: 4c 8b b5 f8 00 00 00        	movq	0xf8(%rbp), %r14
1408628c7: 48 8b 03                    	movq	(%rbx), %rax
1408628ca: 4c 8b 63 10                 	movq	0x10(%rbx), %r12
1408628ce: 4c 29 e0                    	subq	%r12, %rax
1408628d1: 48 83 f8 2f                 	cmpq	$0x2f, %rax
1408628d5: 0f 86 71 03 00 00           	jbe	0x140862c4c <.text+0x861c4c>
1408628db: 49 8d 95 a1 86 01 00        	leaq	0x186a1(%r13), %rdx
1408628e2: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1408628e6: 48 8d 0d 03 4c a7 00        	leaq	0xa74c03(%rip), %rcx    # 0x1412d74f0 <.text+0x12d64f0>
1408628ed: 4a 89 0c 20                 	movq	%rcx, (%rax,%r12)
1408628f1: 48 8d 0d 38 24 d0 00        	leaq	0xd02438(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1408628f8: 4a 89 4c 20 08              	movq	%rcx, 0x8(%rax,%r12)
1408628fd: 4e 89 74 20 10              	movq	%r14, 0x10(%rax,%r12)
140862902: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
140862906: 4a 89 54 20 18              	movq	%rdx, 0x18(%rax,%r12)
14086290b: 42 c6 44 20 20 01           	movb	$0x1, 0x20(%rax,%r12)
140862911: 42 c6 44 20 28 00           	movb	$0x0, 0x28(%rax,%r12)
140862917: 49 83 c4 30                 	addq	$0x30, %r12
14086291b: 4c 89 63 10                 	movq	%r12, 0x10(%rbx)
14086291f: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140862926: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
14086292d: 0f 10 85 d8 00 00 00        	movups	0xd8(%rbp), %xmm0
140862934: f3 0f 6f 8d e8 00 00 00     	movdqu	0xe8(%rbp), %xmm1
14086293c: 66 0f 7f 8d d0 01 00 00     	movdqa	%xmm1, 0x1d0(%rbp)
140862944: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
14086294b: 48 8b 95 f0 01 00 00        	movq	0x1f0(%rbp), %rdx
140862952: 4c 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %r8
140862959: f0                          	lock
14086295a: 48 ff 02                    	incq	(%rdx)
14086295d: 0f 8e 17 06 00 00           	jle	0x140862f7a <.text+0x861f7a>
140862963: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14086296a: 4c 8d 0d 7f 38 23 02        	leaq	0x223387f(%rip), %r9    # 0x142a961f0
140862971: e8 7a c1 a7 00              	callq	0x1412deaf0 <.text+0x12ddaf0>
140862976: 48 8b 9d c8 01 00 00        	movq	0x1c8(%rbp), %rbx
14086297d: 48 85 db                    	testq	%rbx, %rbx
140862980: 75 07                       	jne	0x140862989 <.text+0x861989>
140862982: 48 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %rbx
140862989: 4c 8b b5 e0 01 00 00        	movq	0x1e0(%rbp), %r14
140862990: 48 8b 03                    	movq	(%rbx), %rax
140862993: 4c 8b 63 10                 	movq	0x10(%rbx), %r12
140862997: 4c 29 e0                    	subq	%r12, %rax
14086299a: 48 83 f8 2f                 	cmpq	$0x2f, %rax
14086299e: 0f 86 cd 02 00 00           	jbe	0x140862c71 <.text+0x861c71>
1408629a4: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1408629a8: 48 8d 0d 21 3e a7 00        	leaq	0xa73e21(%rip), %rcx    # 0x1412d67d0 <.text+0x12d57d0>
1408629af: 4a 89 0c 20                 	movq	%rcx, (%rax,%r12)
1408629b3: 48 8d 0d 76 23 d0 00        	leaq	0xd02376(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1408629ba: 4a 89 4c 20 08              	movq	%rcx, 0x8(%rax,%r12)
1408629bf: 4e 89 74 20 10              	movq	%r14, 0x10(%rax,%r12)
1408629c4: f2 46 0f 11 54 20 18        	movsd	%xmm10, 0x18(%rax,%r12)
1408629cb: 42 c7 44 20 20 00 00 80 3f  	movl	$0x3f800000, 0x20(%rax,%r12) # imm = 0x3F800000
1408629d4: 42 c6 44 20 24 00           	movb	$0x0, 0x24(%rax,%r12)
1408629da: 42 c6 44 20 28 00           	movb	$0x0, 0x28(%rax,%r12)
1408629e0: 49 83 c4 30                 	addq	$0x30, %r12
1408629e4: 4c 89 63 10                 	movq	%r12, 0x10(%rbx)
1408629e8: 48 8b 9d c8 01 00 00        	movq	0x1c8(%rbp), %rbx
1408629ef: 48 85 db                    	testq	%rbx, %rbx
1408629f2: 75 07                       	jne	0x1408629fb <.text+0x8619fb>
1408629f4: 48 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %rbx
1408629fb: f3 45 0f 10 6f d0           	movss	-0x30(%r15), %xmm13
140862a01: f3 45 0f 10 77 d8           	movss	-0x28(%r15), %xmm14
140862a07: 4c 8b b5 e0 01 00 00        	movq	0x1e0(%rbp), %r14
140862a0e: 48 8b 03                    	movq	(%rbx), %rax
140862a11: 4c 8b 63 10                 	movq	0x10(%rbx), %r12
140862a15: 4c 29 e0                    	subq	%r12, %rax
140862a18: 48 83 f8 27                 	cmpq	$0x27, %rax
140862a1c: 0f 86 74 02 00 00           	jbe	0x140862c96 <.text+0x861c96>
140862a22: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140862a26: 48 8d 0d 73 87 a7 00        	leaq	0xa78773(%rip), %rcx    # 0x1412db1a0 <.text+0x12da1a0>
140862a2d: 4a 89 0c 20                 	movq	%rcx, (%rax,%r12)
140862a31: 48 8d 0d f8 22 d0 00        	leaq	0xd022f8(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140862a38: 4a 89 4c 20 08              	movq	%rcx, 0x8(%rax,%r12)
140862a3d: 4e 89 74 20 10              	movq	%r14, 0x10(%rax,%r12)
140862a42: f3 46 0f 11 6c 20 18        	movss	%xmm13, 0x18(%rax,%r12)
140862a49: f3 46 0f 11 74 20 1c        	movss	%xmm14, 0x1c(%rax,%r12)
140862a50: 42 c6 44 20 20 00           	movb	$0x0, 0x20(%rax,%r12)
140862a56: 49 83 c4 28                 	addq	$0x28, %r12
140862a5a: 4c 89 63 10                 	movq	%r12, 0x10(%rbx)
140862a5e: f2 45 0f 10 6f d0           	movsd	-0x30(%r15), %xmm13
140862a64: 45 0f 58 eb                 	addps	%xmm11, %xmm13
140862a68: 48 8b 9d c8 01 00 00        	movq	0x1c8(%rbp), %rbx
140862a6f: 48 85 db                    	testq	%rbx, %rbx
140862a72: 75 07                       	jne	0x140862a7b <.text+0x861a7b>
140862a74: 48 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %rbx
140862a7b: f3 45 0f 10 77 d8           	movss	-0x28(%r15), %xmm14
140862a81: f3 45 0f 58 f4              	addss	%xmm12, %xmm14
140862a86: 4c 8b b5 e0 01 00 00        	movq	0x1e0(%rbp), %r14
140862a8d: 48 8b 03                    	movq	(%rbx), %rax
140862a90: 4c 8b 63 10                 	movq	0x10(%rbx), %r12
140862a94: 4c 29 e0                    	subq	%r12, %rax
140862a97: 48 83 f8 27                 	cmpq	$0x27, %rax
140862a9b: 0f 86 1a 02 00 00           	jbe	0x140862cbb <.text+0x861cbb>
140862aa1: 49 8d 47 d0                 	leaq	-0x30(%r15), %rax
140862aa5: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140862aa9: 48 8d 15 00 6b a7 00        	leaq	0xa76b00(%rip), %rdx    # 0x1412d95b0 <.text+0x12d85b0>
140862ab0: 4a 89 14 21                 	movq	%rdx, (%rcx,%r12)
140862ab4: 48 8d 15 75 22 d0 00        	leaq	0xd02275(%rip), %rdx    # 0x141564d30 <.text+0x1563d30>
140862abb: 4a 89 54 21 08              	movq	%rdx, 0x8(%rcx,%r12)
140862ac0: 4e 89 74 21 10              	movq	%r14, 0x10(%rcx,%r12)
140862ac5: 46 0f 13 6c 21 18           	movlps	%xmm13, 0x18(%rcx,%r12)
140862acb: f3 46 0f 11 74 21 20        	movss	%xmm14, 0x20(%rcx,%r12)
140862ad2: 42 c6 44 21 24 00           	movb	$0x0, 0x24(%rcx,%r12)
140862ad8: 49 83 c4 28                 	addq	$0x28, %r12
140862adc: 4c 89 63 10                 	movq	%r12, 0x10(%rbx)
140862ae0: 48 8b 9d e0 01 00 00        	movq	0x1e0(%rbp), %rbx
140862ae7: 0f 10 00                    	movups	(%rax), %xmm0
140862aea: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
140862aee: 48 8b 40 10                 	movq	0x10(%rax), %rax
140862af2: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140862af6: 41 0f 10 47 ec              	movups	-0x14(%r15), %xmm0
140862afb: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140862aff: 41 8b 47 fc                 	movl	-0x4(%r15), %eax
140862b03: 89 45 00                    	movl	%eax, (%rbp)
140862b06: 4c 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %r8
140862b0d: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
140862b14: f0                          	lock
140862b15: 49 ff 00                    	incq	(%r8)
140862b18: 0f 8e 5c 04 00 00           	jle	0x140862f7a <.text+0x861f7a>
140862b1e: 8b 8d a8 01 00 00           	movl	0x1a8(%rbp), %ecx
140862b24: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
140862b28: 89 0a                       	movl	%ecx, (%rdx)
140862b2a: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
140862b31: 48 8b 4a 40                 	movq	0x40(%rdx), %rcx
140862b35: 48 89 8d 20 01 00 00        	movq	%rcx, 0x120(%rbp)
140862b3c: 4c 89 45 48                 	movq	%r8, 0x48(%rbp)
140862b40: 4c 89 85 d8 00 00 00        	movq	%r8, 0xd8(%rbp)
140862b47: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
140862b4e: 48 89 9d e8 00 00 00        	movq	%rbx, 0xe8(%rbp)
140862b55: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
140862b59: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140862b60: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
140862b64: 0f 28 45 d0                 	movaps	-0x30(%rbp), %xmm0
140862b68: 0f 11 01                    	movups	%xmm0, (%rcx)
140862b6b: 8b 85 b8 01 00 00           	movl	0x1b8(%rbp), %eax
140862b71: 89 85 08 01 00 00           	movl	%eax, 0x108(%rbp)
140862b77: 8b 45 00                    	movl	(%rbp), %eax
140862b7a: 89 41 2c                    	movl	%eax, 0x2c(%rcx)
140862b7d: 0f 28 45 f0                 	movaps	-0x10(%rbp), %xmm0
140862b81: 0f 11 41 1c                 	movups	%xmm0, 0x1c(%rcx)
140862b85: 4c 8b 62 30                 	movq	0x30(%rdx), %r12
140862b89: 4c 3b 62 20                 	cmpq	0x20(%rdx), %r12
140862b8d: 75 10                       	jne	0x140862b9f <.text+0x861b9f>
140862b8f: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140862b93: 48 8d 15 ae 75 3d 02        	leaq	0x23d75ae(%rip), %rdx   # 0x142c3a148
140862b9a: e8 a1 09 d0 ff              	callq	0x140563540 <.text+0x562540>
140862b9f: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
140862ba6: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140862baa: 4b 8d 0c a4                 	leaq	(%r12,%r12,4), %rcx
140862bae: 48 c1 e1 04                 	shlq	$0x4, %rcx
140862bb2: 0f 10 85 d8 00 00 00        	movups	0xd8(%rbp), %xmm0
140862bb9: f3 0f 6f 8d e8 00 00 00     	movdqu	0xe8(%rbp), %xmm1
140862bc1: 0f 10 95 f8 00 00 00        	movups	0xf8(%rbp), %xmm2
140862bc8: 0f 10 9d 08 01 00 00        	movups	0x108(%rbp), %xmm3
140862bcf: 0f 10 a5 18 01 00 00        	movups	0x118(%rbp), %xmm4
140862bd6: 0f 11 64 08 40              	movups	%xmm4, 0x40(%rax,%rcx)
140862bdb: 0f 11 5c 08 30              	movups	%xmm3, 0x30(%rax,%rcx)
140862be0: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
140862be5: f3 0f 7f 4c 08 10           	movdqu	%xmm1, 0x10(%rax,%rcx)
140862beb: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
140862bef: 49 ff c4                    	incq	%r12
140862bf2: 4c 89 62 30                 	movq	%r12, 0x30(%rdx)
140862bf6: 48 ff 42 40                 	incq	0x40(%rdx)
140862bfa: 8b 85 ac 01 00 00           	movl	0x1ac(%rbp), %eax
140862c00: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
140862c04: 89 01                       	movl	%eax, (%rcx)
140862c06: 48 8b 8d e8 01 00 00        	movq	0x1e8(%rbp), %rcx
140862c0d: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
140862c11: 49 89 d8                    	movq	%rbx, %r8
140862c14: e8 57 9d 68 00              	callq	0x140eec970 <.text+0xeeb970>
140862c19: 49 8d 4d 01                 	leaq	0x1(%r13), %rcx
140862c1d: 4c 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %r8
140862c24: 49 8b 50 30                 	movq	0x30(%r8), %rdx
140862c28: 49 81 c5 a2 86 01 00        	addq	$0x186a2, %r13          # imm = 0x186A2
140862c2f: 49 39 d5                    	cmpq	%rdx, %r13
140862c32: 4c 0f 46 ea                 	cmovbeq	%rdx, %r13
140862c36: 4d 89 68 30                 	movq	%r13, 0x30(%r8)
140862c3a: 48 c1 e8 20                 	shrq	$0x20, %rax
140862c3e: 49 89 cd                    	movq	%rcx, %r13
140862c41: 0f 84 29 fc ff ff           	je	0x140862870 <.text+0x861870>
140862c47: e9 f6 01 00 00              	jmp	0x140862e42 <.text+0x861e42>
140862c4c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140862c55: 41 b8 30 00 00 00           	movl	$0x30, %r8d
140862c5b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140862c61: 48 89 d9                    	movq	%rbx, %rcx
140862c64: 4c 89 e2                    	movq	%r12, %rdx
140862c67: e8 f4 6d 04 02              	callq	0x1428a9a60 <.text+0x28a8a60>
140862c6c: e9 6a fc ff ff              	jmp	0x1408628db <.text+0x8618db>
140862c71: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140862c7a: 41 b8 30 00 00 00           	movl	$0x30, %r8d
140862c80: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140862c86: 48 89 d9                    	movq	%rbx, %rcx
140862c89: 4c 89 e2                    	movq	%r12, %rdx
140862c8c: e8 cf 6d 04 02              	callq	0x1428a9a60 <.text+0x28a8a60>
140862c91: e9 0e fd ff ff              	jmp	0x1408629a4 <.text+0x8619a4>
140862c96: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140862c9f: 41 b8 28 00 00 00           	movl	$0x28, %r8d
140862ca5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140862cab: 48 89 d9                    	movq	%rbx, %rcx
140862cae: 4c 89 e2                    	movq	%r12, %rdx
140862cb1: e8 aa 6d 04 02              	callq	0x1428a9a60 <.text+0x28a8a60>
140862cb6: e9 67 fd ff ff              	jmp	0x140862a22 <.text+0x861a22>
140862cbb: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140862cc4: 41 b8 28 00 00 00           	movl	$0x28, %r8d
140862cca: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140862cd0: 48 89 d9                    	movq	%rbx, %rcx
140862cd3: 4c 89 e2                    	movq	%r12, %rdx
140862cd6: e8 85 6d 04 02              	callq	0x1428a9a60 <.text+0x28a8a60>
140862cdb: e9 c1 fd ff ff              	jmp	0x140862aa1 <.text+0x861aa1>
140862ce0: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
140862ce7: f0                          	lock
140862ce8: 48 ff 08                    	decq	(%rax)
140862ceb: 75 0c                       	jne	0x140862cf9 <.text+0x861cf9>
140862ced: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140862cf4: e8 37 be de ff              	callq	0x14064eb30 <.text+0x64db30>
140862cf9: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
140862d00: 48 85 c0                    	testq	%rax, %rax
140862d03: 4c 8d 3d 10 c8 28 02        	leaq	0x228c810(%rip), %r15   # 0x142aef51a
140862d0a: 74 1a                       	je	0x140862d26 <.text+0x861d26>
140862d0c: 48 c1 e0 04                 	shlq	$0x4, %rax
140862d10: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140862d14: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140862d1a: 48 8b 8d b0 01 00 00        	movq	0x1b0(%rbp), %rcx
140862d21: e8 fa 0e db ff              	callq	0x140613c20 <.text+0x612c20>
140862d26: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
140862d2d: 48 85 d2                    	testq	%rdx, %rdx
140862d30: 74 12                       	je	0x140862d44 <.text+0x861d44>
140862d32: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140862d38: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
140862d3f: e8 dc 0e db ff              	callq	0x140613c20 <.text+0x612c20>
140862d44: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
140862d48: 48 85 d2                    	testq	%rdx, %rdx
140862d4b: 74 0f                       	je	0x140862d5c <.text+0x861d5c>
140862d4d: 48 8b 4d b8                 	movq	-0x48(%rbp), %rcx
140862d51: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140862d57: e8 c4 0e db ff              	callq	0x140613c20 <.text+0x612c20>
140862d5c: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
140862d63: 48 85 d2                    	testq	%rdx, %rdx
140862d66: 74 12                       	je	0x140862d7a <.text+0x861d7a>
140862d68: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
140862d6f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140862d75: e8 a6 0e db ff              	callq	0x140613c20 <.text+0x612c20>
140862d7a: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
140862d81: 48 85 d2                    	testq	%rdx, %rdx
140862d84: 74 12                       	je	0x140862d98 <.text+0x861d98>
140862d86: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
140862d8d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140862d93: e8 88 0e db ff              	callq	0x140613c20 <.text+0x612c20>
140862d98: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
140862d9f: 48 89 d0                    	movq	%rdx, %rax
140862da2: 48 f7 d8                    	negq	%rax
140862da5: 70 14                       	jo	0x140862dbb <.text+0x861dbb>
140862da7: 73 12                       	jae	0x140862dbb <.text+0x861dbb>
140862da9: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
140862db0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140862db6: e8 65 0e db ff              	callq	0x140613c20 <.text+0x612c20>
140862dbb: 4c 8b b5 a8 00 00 00        	movq	0xa8(%rbp), %r14
140862dc2: 48 8b bd b8 00 00 00        	movq	0xb8(%rbp), %rdi
140862dc9: 49 39 fe                    	cmpq	%rdi, %r14
140862dcc: 0f 85 32 f8 ff ff           	jne	0x140862604 <.text+0x861604>
140862dd2: 4c 89 f3                    	movq	%r14, %rbx
140862dd5: 48 89 9d a8 00 00 00        	movq	%rbx, 0xa8(%rbp)
140862ddc: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
140862de3: e8 48 64 01 00              	callq	0x140879230 <.text+0x878230>
140862de8: 0f 28 b5 10 02 00 00        	movaps	0x210(%rbp), %xmm6
140862def: 0f 28 bd 20 02 00 00        	movaps	0x220(%rbp), %xmm7
140862df6: 44 0f 28 85 30 02 00 00     	movaps	0x230(%rbp), %xmm8
140862dfe: 44 0f 28 8d 40 02 00 00     	movaps	0x240(%rbp), %xmm9
140862e06: 44 0f 28 95 50 02 00 00     	movaps	0x250(%rbp), %xmm10
140862e0e: 44 0f 28 9d 60 02 00 00     	movaps	0x260(%rbp), %xmm11
140862e16: 44 0f 28 a5 70 02 00 00     	movaps	0x270(%rbp), %xmm12
140862e1e: 44 0f 28 ad 80 02 00 00     	movaps	0x280(%rbp), %xmm13
140862e26: 44 0f 28 b5 90 02 00 00     	movaps	0x290(%rbp), %xmm14
140862e2e: 48 81 c4 28 03 00 00        	addq	$0x328, %rsp            # imm = 0x328
140862e35: 5b                          	popq	%rbx
140862e36: 5f                          	popq	%rdi
140862e37: 5e                          	popq	%rsi
140862e38: 41 5c                       	popq	%r12
140862e3a: 41 5d                       	popq	%r13
140862e3c: 41 5e                       	popq	%r14
140862e3e: 41 5f                       	popq	%r15
140862e40: 5d                          	popq	%rbp
140862e41: c3                          	retq
140862e42: 48 8d 0d bf 33 23 02        	leaq	0x22333bf(%rip), %rcx   # 0x142a96208
140862e49: 4c 8d 05 d8 33 23 02        	leaq	0x22333d8(%rip), %r8    # 0x142a96228
140862e50: ba 20 00 00 00              	movl	$0x20, %edx
140862e55: e8 d6 65 07 02              	callq	0x1428d9430 <.text+0x28d8430>
140862e5a: e9 1b 01 00 00              	jmp	0x140862f7a <.text+0x861f7a>
140862e5f: 48 8b 45 58                 	movq	0x58(%rbp), %rax
140862e63: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
140862e6a: 48 8d 05 1f 33 23 02        	leaq	0x223331f(%rip), %rax   # 0x142a96190
140862e71: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140862e76: 48 8d 0d 07 33 23 02        	leaq	0x2233307(%rip), %rcx   # 0x142a96184
140862e7d: 4c 8d 0d 1c 30 23 02        	leaq	0x223301c(%rip), %r9    # 0x142a95ea0
140862e84: 4c 8d 85 58 01 00 00        	leaq	0x158(%rbp), %r8
140862e8b: ba 0a 00 00 00              	movl	$0xa, %edx
140862e90: e8 cb 68 07 02              	callq	0x1428d9760 <.text+0x28d8760>
140862e95: e9 e0 00 00 00              	jmp	0x140862f7a <.text+0x861f7a>
140862e9a: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
140862ea1: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140862ea5: 48 8d 05 14 33 23 02        	leaq	0x2233314(%rip), %rax   # 0x142a961c0
140862eac: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140862eb1: 48 8d 0d f0 32 23 02        	leaq	0x22332f0(%rip), %rcx   # 0x142a961a8
140862eb8: 4c 8d 0d 01 30 23 02        	leaq	0x2233001(%rip), %r9    # 0x142a95ec0
140862ebf: 4c 8d 45 50                 	leaq	0x50(%rbp), %r8
140862ec3: ba 11 00 00 00              	movl	$0x11, %edx
140862ec8: e8 93 68 07 02              	callq	0x1428d9760 <.text+0x28d8760>
140862ecd: e9 a8 00 00 00              	jmp	0x140862f7a <.text+0x861f7a>
140862ed2: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140862ed9: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
140862ee0: 48 8d 05 f1 32 23 02        	leaq	0x22332f1(%rip), %rax   # 0x142a961d8
140862ee7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140862eec: 48 8d 0d 0d 30 23 02        	leaq	0x223300d(%rip), %rcx   # 0x142a95f00
140862ef3: 4c 8d 0d c6 2f 23 02        	leaq	0x2232fc6(%rip), %r9    # 0x142a95ec0
140862efa: 4c 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %r8
140862f01: ba 2b 00 00 00              	movl	$0x2b, %edx
140862f06: e8 55 68 07 02              	callq	0x1428d9760 <.text+0x28d8760>
140862f0b: eb 6d                       	jmp	0x140862f7a <.text+0x861f7a>
140862f0d: 48 89 55 08                 	movq	%rdx, 0x8(%rbp)
140862f11: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
140862f15: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140862f1c: 48 8d 05 cd b9 7c ff        	leaq	-0x834633(%rip), %rax   # 0x14002e8f0 <.text+0x2d8f0>
140862f23: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
140862f2a: 48 8d 05 1f 33 23 02        	leaq	0x223331f(%rip), %rax   # 0x142a96250
140862f31: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
140862f38: 48 c7 85 e0 00 00 00 01 00 00 00    	movq	$0x1, 0xe0(%rbp)
140862f43: 48 c7 85 f8 00 00 00 00 00 00 00    	movq	$0x0, 0xf8(%rbp)
140862f4e: 48 8d 85 f0 01 00 00        	leaq	0x1f0(%rbp), %rax
140862f55: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
140862f5c: 48 c7 85 f0 00 00 00 01 00 00 00    	movq	$0x1, 0xf0(%rbp)
140862f67: 48 8d 15 f2 32 23 02        	leaq	0x22332f2(%rip), %rdx   # 0x142a96260
140862f6e: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
140862f75: e8 16 64 07 02              	callq	0x1428d9390 <.text+0x28d8390>
140862f7a: 0f 0b                       	ud2
140862f7c: 0f 1f 40 00                 	nopl	(%rax)
140862f80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140862f85: 55                          	pushq	%rbp
140862f86: 41 57                       	pushq	%r15
140862f88: 41 56                       	pushq	%r14
140862f8a: 41 55                       	pushq	%r13
140862f8c: 41 54                       	pushq	%r12
140862f8e: 56                          	pushq	%rsi
140862f8f: 57                          	pushq	%rdi
140862f90: 53                          	pushq	%rbx
140862f91: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140862f98: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140862f9f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140862fa5: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140862fab: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140862fb1: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140862fb7: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140862fbd: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140862fc3: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140862fcc: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140862fd5: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140862fde: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140862fe5: 48 85 d2                    	testq	%rdx, %rdx
140862fe8: 74 12                       	je	0x140862ffc <.text+0x861ffc>
140862fea: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140862ff1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140862ff7: e8 24 0c db ff              	callq	0x140613c20 <.text+0x612c20>
140862ffc: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140863004: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14086300c: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140863015: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
14086301b: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140863021: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140863027: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14086302d: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140863033: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140863039: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140863040: 5b                          	popq	%rbx
140863041: 5f                          	popq	%rdi
140863042: 5e                          	popq	%rsi
140863043: 41 5c                       	popq	%r12
140863045: 41 5d                       	popq	%r13
140863047: 41 5e                       	popq	%r14
140863049: 41 5f                       	popq	%r15
14086304b: 5d                          	popq	%rbp
14086304c: c3                          	retq
14086304d: 0f 1f 00                    	nopl	(%rax)
140863050: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140863055: 55                          	pushq	%rbp
140863056: 41 57                       	pushq	%r15
140863058: 41 56                       	pushq	%r14
14086305a: 41 55                       	pushq	%r13
14086305c: 41 54                       	pushq	%r12
14086305e: 56                          	pushq	%rsi
14086305f: 57                          	pushq	%rdi
140863060: 53                          	pushq	%rbx
140863061: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140863068: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086306f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140863075: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
14086307b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140863081: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140863087: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14086308d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140863093: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14086309c: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
1408630a5: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
1408630ae: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
1408630b5: 48 85 d2                    	testq	%rdx, %rdx
1408630b8: 74 12                       	je	0x1408630cc <.text+0x8620cc>
1408630ba: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
1408630c1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408630c7: e8 54 0b db ff              	callq	0x140613c20 <.text+0x612c20>
1408630cc: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
1408630d5: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
1408630de: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
1408630e7: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
1408630ed: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
1408630f3: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
1408630f9: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1408630ff: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140863105: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
14086310b: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140863112: 5b                          	popq	%rbx
140863113: 5f                          	popq	%rdi
140863114: 5e                          	popq	%rsi
140863115: 41 5c                       	popq	%r12
140863117: 41 5d                       	popq	%r13
140863119: 41 5e                       	popq	%r14
14086311b: 41 5f                       	popq	%r15
14086311d: 5d                          	popq	%rbp
14086311e: c3                          	retq
14086311f: 90                          	nop
140863120: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140863125: 55                          	pushq	%rbp
140863126: 41 57                       	pushq	%r15
140863128: 41 56                       	pushq	%r14
14086312a: 41 55                       	pushq	%r13
14086312c: 41 54                       	pushq	%r12
14086312e: 56                          	pushq	%rsi
14086312f: 57                          	pushq	%rdi
140863130: 53                          	pushq	%rbx
140863131: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140863138: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086313f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140863145: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
14086314b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140863151: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140863157: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14086315d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140863163: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14086316c: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140863175: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14086317e: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
140863185: e8 76 48 d8 ff              	callq	0x1405e7a00 <.text+0x5e6a00>
14086318a: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140863193: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
14086319c: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
1408631a5: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
1408631ab: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
1408631b1: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
1408631b7: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1408631bd: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
1408631c3: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
1408631c9: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1408631d0: 5b                          	popq	%rbx
1408631d1: 5f                          	popq	%rdi
1408631d2: 5e                          	popq	%rsi
1408631d3: 41 5c                       	popq	%r12
1408631d5: 41 5d                       	popq	%r13
1408631d7: 41 5e                       	popq	%r14
1408631d9: 41 5f                       	popq	%r15
1408631db: 5d                          	popq	%rbp
1408631dc: c3                          	retq
1408631dd: 0f 1f 00                    	nopl	(%rax)
1408631e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408631e5: 55                          	pushq	%rbp
1408631e6: 41 57                       	pushq	%r15
1408631e8: 41 56                       	pushq	%r14
1408631ea: 41 55                       	pushq	%r13
1408631ec: 41 54                       	pushq	%r12
1408631ee: 56                          	pushq	%rsi
1408631ef: 57                          	pushq	%rdi
1408631f0: 53                          	pushq	%rbx
1408631f1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
1408631f8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408631ff: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140863205: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
14086320b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140863211: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140863217: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14086321d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140863223: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14086322c: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140863235: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14086323e: 48 83 bd 00 02 00 00 00     	cmpq	$0x0, 0x200(%rbp)
140863246: 74 19                       	je	0x140863261 <.text+0x862261>
140863248: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14086324e: 48 8b 8d e8 01 00 00        	movq	0x1e8(%rbp), %rcx
140863255: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
14086325c: e8 bf 09 db ff              	callq	0x140613c20 <.text+0x612c20>
140863261: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
14086326a: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
140863273: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14086327c: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140863282: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140863288: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14086328e: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140863294: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
14086329a: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
1408632a0: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1408632a7: 5b                          	popq	%rbx
1408632a8: 5f                          	popq	%rdi
1408632a9: 5e                          	popq	%rsi
1408632aa: 41 5c                       	popq	%r12
1408632ac: 41 5d                       	popq	%r13
1408632ae: 41 5e                       	popq	%r14
1408632b0: 41 5f                       	popq	%r15
1408632b2: 5d                          	popq	%rbp
1408632b3: c3                          	retq
1408632b4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1408632c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408632c5: 55                          	pushq	%rbp
1408632c6: 41 57                       	pushq	%r15
1408632c8: 41 56                       	pushq	%r14
1408632ca: 41 55                       	pushq	%r13
1408632cc: 41 54                       	pushq	%r12
1408632ce: 56                          	pushq	%rsi
1408632cf: 57                          	pushq	%rdi
1408632d0: 53                          	pushq	%rbx
1408632d1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
1408632d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408632df: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
1408632e5: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
1408632eb: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
1408632f1: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
1408632f7: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
1408632fd: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140863303: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14086330c: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140863315: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14086331e: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
140863322: 48 85 d2                    	testq	%rdx, %rdx
140863325: 74 0f                       	je	0x140863336 <.text+0x862336>
140863327: 48 8b 4d b8                 	movq	-0x48(%rbp), %rcx
14086332b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140863331: e8 ea 08 db ff              	callq	0x140613c20 <.text+0x612c20>
140863336: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
14086333f: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
140863348: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140863351: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140863357: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14086335d: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140863363: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140863369: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
14086336f: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140863375: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14086337c: 5b                          	popq	%rbx
14086337d: 5f                          	popq	%rdi
14086337e: 5e                          	popq	%rsi
14086337f: 41 5c                       	popq	%r12
140863381: 41 5d                       	popq	%r13
140863383: 41 5e                       	popq	%r14
140863385: 41 5f                       	popq	%r15
140863387: 5d                          	popq	%rbp
140863388: c3                          	retq
140863389: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140863390: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140863395: 55                          	pushq	%rbp
140863396: 41 57                       	pushq	%r15
140863398: 41 56                       	pushq	%r14
14086339a: 41 55                       	pushq	%r13
14086339c: 41 54                       	pushq	%r12
14086339e: 56                          	pushq	%rsi
14086339f: 57                          	pushq	%rdi
1408633a0: 53                          	pushq	%rbx
1408633a1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
1408633a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408633af: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
1408633b5: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
1408633bb: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
1408633c1: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
1408633c7: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
1408633cd: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
1408633d3: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
1408633dc: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
1408633e5: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
1408633ee: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
1408633f2: e8 09 46 d8 ff              	callq	0x1405e7a00 <.text+0x5e6a00>
1408633f7: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140863400: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
140863409: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140863412: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140863418: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14086341e: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140863424: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14086342a: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140863430: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140863436: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14086343d: 5b                          	popq	%rbx
14086343e: 5f                          	popq	%rdi
14086343f: 5e                          	popq	%rsi
140863440: 41 5c                       	popq	%r12
140863442: 41 5d                       	popq	%r13
140863444: 41 5e                       	popq	%r14
140863446: 41 5f                       	popq	%r15
140863448: 5d                          	popq	%rbp
140863449: c3                          	retq
14086344a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140863450: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140863455: 55                          	pushq	%rbp
140863456: 41 57                       	pushq	%r15
140863458: 41 56                       	pushq	%r14
14086345a: 41 55                       	pushq	%r13
14086345c: 41 54                       	pushq	%r12
14086345e: 56                          	pushq	%rsi
14086345f: 57                          	pushq	%rdi
140863460: 53                          	pushq	%rbx
140863461: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140863468: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086346f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140863475: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
14086347b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140863481: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140863487: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14086348d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140863493: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14086349c: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
1408634a5: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
1408634ae: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
1408634b5: 48 85 d2                    	testq	%rdx, %rdx
1408634b8: 74 12                       	je	0x1408634cc <.text+0x8624cc>
1408634ba: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408634c0: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
1408634c7: e8 54 07 db ff              	callq	0x140613c20 <.text+0x612c20>
1408634cc: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
1408634d5: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
1408634de: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
1408634e7: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
1408634ed: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
1408634f3: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
1408634f9: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1408634ff: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140863505: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
14086350b: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140863512: 5b                          	popq	%rbx
140863513: 5f                          	popq	%rdi
140863514: 5e                          	popq	%rsi
140863515: 41 5c                       	popq	%r12
140863517: 41 5d                       	popq	%r13
140863519: 41 5e                       	popq	%r14
14086351b: 41 5f                       	popq	%r15
14086351d: 5d                          	popq	%rbp
14086351e: c3                          	retq
14086351f: 90                          	nop
140863520: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140863525: 55                          	pushq	%rbp
140863526: 41 57                       	pushq	%r15
140863528: 41 56                       	pushq	%r14
14086352a: 41 55                       	pushq	%r13
14086352c: 41 54                       	pushq	%r12
14086352e: 56                          	pushq	%rsi
14086352f: 57                          	pushq	%rdi
140863530: 53                          	pushq	%rbx
140863531: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140863538: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086353f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140863545: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
14086354b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140863551: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140863557: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14086355d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140863563: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14086356c: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140863575: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14086357e: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
140863585: 48 85 c0                    	testq	%rax, %rax
140863588: 74 1a                       	je	0x1408635a4 <.text+0x8625a4>
14086358a: 48 c1 e0 04                 	shlq	$0x4, %rax
14086358e: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140863592: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140863598: 48 8b 8d b0 01 00 00        	movq	0x1b0(%rbp), %rcx
14086359f: e8 7c 06 db ff              	callq	0x140613c20 <.text+0x612c20>
1408635a4: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
1408635ad: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
1408635b6: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
1408635bf: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
1408635c5: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
1408635cb: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
1408635d1: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1408635d7: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
1408635dd: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
1408635e3: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1408635ea: 5b                          	popq	%rbx
1408635eb: 5f                          	popq	%rdi
1408635ec: 5e                          	popq	%rsi
1408635ed: 41 5c                       	popq	%r12
1408635ef: 41 5d                       	popq	%r13
1408635f1: 41 5e                       	popq	%r14
1408635f3: 41 5f                       	popq	%r15
1408635f5: 5d                          	popq	%rbp
1408635f6: c3                          	retq
1408635f7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140863600: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140863605: 55                          	pushq	%rbp
140863606: 41 57                       	pushq	%r15
140863608: 41 56                       	pushq	%r14
14086360a: 41 55                       	pushq	%r13
14086360c: 41 54                       	pushq	%r12
14086360e: 56                          	pushq	%rsi
14086360f: 57                          	pushq	%rdi
140863610: 53                          	pushq	%rbx
140863611: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140863618: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086361f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140863625: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
14086362b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140863631: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140863637: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14086363d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140863643: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14086364c: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140863655: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14086365e: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
140863665: f0                          	lock
140863666: 48 ff 08                    	decq	(%rax)
140863669: 75 0c                       	jne	0x140863677 <.text+0x862677>
14086366b: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140863672: e8 b9 b4 de ff              	callq	0x14064eb30 <.text+0x64db30>
140863677: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140863680: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
140863689: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140863692: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140863698: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14086369e: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
1408636a4: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1408636aa: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
1408636b0: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
1408636b6: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1408636bd: 5b                          	popq	%rbx
1408636be: 5f                          	popq	%rdi
1408636bf: 5e                          	popq	%rsi
1408636c0: 41 5c                       	popq	%r12
1408636c2: 41 5d                       	popq	%r13
1408636c4: 41 5e                       	popq	%r14
1408636c6: 41 5f                       	popq	%r15
1408636c8: 5d                          	popq	%rbp
1408636c9: c3                          	retq
1408636ca: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1408636d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408636d5: 55                          	pushq	%rbp
1408636d6: 41 57                       	pushq	%r15
1408636d8: 41 56                       	pushq	%r14
1408636da: 41 55                       	pushq	%r13
1408636dc: 41 54                       	pushq	%r12
1408636de: 56                          	pushq	%rsi
1408636df: 57                          	pushq	%rdi
1408636e0: 53                          	pushq	%rbx
1408636e1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
1408636e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408636ef: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
1408636f5: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
1408636fb: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140863701: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140863707: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14086370d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140863713: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14086371c: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140863725: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14086372e: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140863732: f0                          	lock
140863733: 48 ff 08                    	decq	(%rax)
140863736: 75 0c                       	jne	0x140863744 <.text+0x862744>
140863738: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
14086373f: e8 ec b3 de ff              	callq	0x14064eb30 <.text+0x64db30>
140863744: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
14086374d: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
140863756: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14086375f: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140863765: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14086376b: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140863771: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140863777: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
14086377d: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140863783: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14086378a: 5b                          	popq	%rbx
14086378b: 5f                          	popq	%rdi
14086378c: 5e                          	popq	%rsi
14086378d: 41 5c                       	popq	%r12
14086378f: 41 5d                       	popq	%r13
140863791: 41 5e                       	popq	%r14
140863793: 41 5f                       	popq	%r15
140863795: 5d                          	popq	%rbp
140863796: c3                          	retq
140863797: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1408637a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408637a5: 55                          	pushq	%rbp
1408637a6: 41 57                       	pushq	%r15
1408637a8: 41 56                       	pushq	%r14
1408637aa: 41 55                       	pushq	%r13
1408637ac: 41 54                       	pushq	%r12
1408637ae: 56                          	pushq	%rsi
1408637af: 57                          	pushq	%rdi
1408637b0: 53                          	pushq	%rbx
1408637b1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
1408637b8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408637bf: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
1408637c5: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
1408637cb: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
1408637d1: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
1408637d7: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
1408637dd: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
1408637e3: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
1408637ec: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
1408637f5: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
1408637fe: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140863802: e8 99 50 d8 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
140863807: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140863810: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
140863819: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140863822: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140863828: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14086382e: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140863834: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14086383a: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140863840: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140863846: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14086384d: 5b                          	popq	%rbx
14086384e: 5f                          	popq	%rdi
14086384f: 5e                          	popq	%rsi
140863850: 41 5c                       	popq	%r12
140863852: 41 5d                       	popq	%r13
140863854: 41 5e                       	popq	%r14
140863856: 41 5f                       	popq	%r15
140863858: 5d                          	popq	%rbp
140863859: c3                          	retq
14086385a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140863860: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140863865: 55                          	pushq	%rbp
140863866: 41 57                       	pushq	%r15
140863868: 41 56                       	pushq	%r14
14086386a: 41 55                       	pushq	%r13
14086386c: 41 54                       	pushq	%r12
14086386e: 56                          	pushq	%rsi
14086386f: 57                          	pushq	%rdi
140863870: 53                          	pushq	%rbx
140863871: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140863878: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086387f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140863885: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
14086388b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140863891: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140863897: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14086389d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
1408638a3: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
1408638ac: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
1408638b5: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
1408638be: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
1408638c5: e8 a6 fc df ff              	callq	0x140663570 <.text+0x662570>
1408638ca: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
1408638d1: e8 5a 59 01 00              	callq	0x140879230 <.text+0x878230>
1408638d6: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
1408638de: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
1408638e6: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
1408638ef: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
1408638f5: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
1408638fb: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140863901: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140863907: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
14086390d: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140863913: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14086391a: 5b                          	popq	%rbx
14086391b: 5f                          	popq	%rdi
14086391c: 5e                          	popq	%rsi
14086391d: 41 5c                       	popq	%r12
14086391f: 41 5d                       	popq	%r13
140863921: 41 5e                       	popq	%r14
140863923: 41 5f                       	popq	%r15
140863925: 5d                          	popq	%rbp
140863926: c3                          	retq
140863927: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140863930: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140863935: 55                          	pushq	%rbp
140863936: 41 57                       	pushq	%r15
140863938: 41 56                       	pushq	%r14
14086393a: 41 55                       	pushq	%r13
14086393c: 41 54                       	pushq	%r12
14086393e: 56                          	pushq	%rsi
14086393f: 57                          	pushq	%rdi
140863940: 53                          	pushq	%rbx
140863941: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140863948: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086394f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140863955: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
14086395b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140863961: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140863967: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14086396d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140863973: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14086397c: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140863985: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14086398e: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
140863995: e8 06 4f d8 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
14086399a: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
1408639a3: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
1408639ac: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
1408639b5: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
1408639bb: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
1408639c1: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
1408639c7: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1408639cd: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
1408639d3: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
1408639d9: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1408639e0: 5b                          	popq	%rbx
1408639e1: 5f                          	popq	%rdi
1408639e2: 5e                          	popq	%rsi
1408639e3: 41 5c                       	popq	%r12
1408639e5: 41 5d                       	popq	%r13
1408639e7: 41 5e                       	popq	%r14
1408639e9: 41 5f                       	popq	%r15
1408639eb: 5d                          	popq	%rbp
1408639ec: c3                          	retq
