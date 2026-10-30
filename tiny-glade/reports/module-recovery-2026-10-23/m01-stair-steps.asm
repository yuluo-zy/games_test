
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407da0b0: 55                          	pushq	%rbp
1407da0b1: 41 57                       	pushq	%r15
1407da0b3: 41 56                       	pushq	%r14
1407da0b5: 41 55                       	pushq	%r13
1407da0b7: 41 54                       	pushq	%r12
1407da0b9: 56                          	pushq	%rsi
1407da0ba: 57                          	pushq	%rdi
1407da0bb: 53                          	pushq	%rbx
1407da0bc: 48 81 ec 88 04 00 00        	subq	$0x488, %rsp            # imm = 0x488
1407da0c3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407da0cb: 44 0f 29 bd f0 03 00 00     	movaps	%xmm15, 0x3f0(%rbp)
1407da0d3: 44 0f 29 b5 e0 03 00 00     	movaps	%xmm14, 0x3e0(%rbp)
1407da0db: 44 0f 29 ad d0 03 00 00     	movaps	%xmm13, 0x3d0(%rbp)
1407da0e3: 44 0f 29 a5 c0 03 00 00     	movaps	%xmm12, 0x3c0(%rbp)
1407da0eb: 44 0f 29 9d b0 03 00 00     	movaps	%xmm11, 0x3b0(%rbp)
1407da0f3: 44 0f 29 95 a0 03 00 00     	movaps	%xmm10, 0x3a0(%rbp)
1407da0fb: 44 0f 29 8d 90 03 00 00     	movaps	%xmm9, 0x390(%rbp)
1407da103: 44 0f 29 85 80 03 00 00     	movaps	%xmm8, 0x380(%rbp)
1407da10b: 0f 29 bd 70 03 00 00        	movaps	%xmm7, 0x370(%rbp)
1407da112: 0f 29 b5 60 03 00 00        	movaps	%xmm6, 0x360(%rbp)
1407da119: 48 c7 85 58 03 00 00 fe ff ff ff    	movq	$-0x2, 0x358(%rbp)
1407da124: 48 8b 85 78 04 00 00        	movq	0x478(%rbp), %rax
1407da12b: 48 83 f8 01                 	cmpq	$0x1, %rax
1407da12f: 0f 84 8a 11 00 00           	je	0x1407db2bf <.text+0x7da2bf>
1407da135: 48 85 c0                    	testq	%rax, %rax
1407da138: 0f 84 97 11 00 00           	je	0x1407db2d5 <.text+0x7da2d5>
1407da13e: 4c 89 ce                    	movq	%r9, %rsi
1407da141: 4c 89 c3                    	movq	%r8, %rbx
1407da144: 48 89 d7                    	movq	%rdx, %rdi
1407da147: 49 89 ce                    	movq	%rcx, %r14
1407da14a: 4c 8b a5 c8 04 00 00        	movq	0x4c8(%rbp), %r12
1407da151: 4c 8b bd b8 04 00 00        	movq	0x4b8(%rbp), %r15
1407da158: f3 0f 10 b5 b0 04 00 00     	movss	0x4b0(%rbp), %xmm6
1407da160: 4c 8b ad 70 04 00 00        	movq	0x470(%rbp), %r13
1407da167: f3 41 0f 10 45 08           	movss	0x8(%r13), %xmm0
1407da16d: f3 41 0f 10 4d 00           	movss	(%r13), %xmm1
1407da173: f3 41 0f 5c 4d 0c           	subss	0xc(%r13), %xmm1
1407da179: f3 41 0f 5c 45 14           	subss	0x14(%r13), %xmm0
1407da17f: f3 41 0f 10 55 04           	movss	0x4(%r13), %xmm2
1407da185: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1407da189: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1407da18d: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407da191: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407da194: f3 0f 51 c8                 	sqrtss	%xmm0, %xmm1
1407da198: f3 0f 11 95 54 03 00 00     	movss	%xmm2, 0x354(%rbp)
1407da1a0: f3 41 0f 5c 55 10           	subss	0x10(%r13), %xmm2
1407da1a6: 0f 54 15 d3 47 15 02        	andps	0x21547d3(%rip), %xmm2  # 0x14292e980
1407da1ad: 0f 29 95 e0 01 00 00        	movaps	%xmm2, 0x1e0(%rbp)
1407da1b4: f3 0f 10 05 30 38 20 02     	movss	0x2203830(%rip), %xmm0  # 0x1429dd9ec
1407da1bc: f3 0f 5e c1                 	divss	%xmm1, %xmm0
1407da1c0: e8 1b 9d 14 02              	callq	0x142923ee0 <.text+0x2922ee0>
1407da1c5: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
1407da1ca: 48 89 c1                    	movq	%rax, %rcx
1407da1cd: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
1407da1d1: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407da1d4: f3 0f 5c 0d 00 ff 17 02     	subss	0x217ff00(%rip), %xmm1  # 0x14295a0dc
1407da1dc: f3 48 0f 2c d1              	cvttss2si	%xmm1, %rdx
1407da1e1: 48 21 ca                    	andq	%rcx, %rdx
1407da1e4: 48 09 c2                    	orq	%rax, %rdx
1407da1e7: 31 c0                       	xorl	%eax, %eax
1407da1e9: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407da1ec: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
1407da1ef: 48 0f 42 d0                 	cmovbq	%rax, %rdx
1407da1f3: 0f 2e 05 e6 fe 17 02        	ucomiss	0x217fee6(%rip), %xmm0  # 0x14295a0e0
1407da1fa: 48 c7 c1 ff ff ff ff        	movq	$-0x1, %rcx
1407da201: 48 0f 47 d1                 	cmovaq	%rcx, %rdx
1407da205: 48 89 95 58 02 00 00        	movq	%rdx, 0x258(%rbp)
1407da20c: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407da20f: 80 bd a8 04 00 00 00        	cmpb	$0x0, 0x4a8(%rbp)
1407da216: 74 08                       	je	0x1407da220 <.text+0x7d9220>
1407da218: f3 0f 10 05 30 b7 14 02     	movss	0x214b730(%rip), %xmm0  # 0x142925950
1407da220: 4c 8b 8d 78 04 00 00        	movq	0x478(%rbp), %r9
1407da227: 4b 8d 0c 49                 	leaq	(%r9,%r9,2), %rcx
1407da22b: 48 8d 14 8d 00 00 00 00     	leaq	(,%rcx,4), %rdx
1407da233: 4c 01 ea                    	addq	%r13, %rdx
1407da236: 4c 89 6d e0                 	movq	%r13, -0x20(%rbp)
1407da23a: 48 89 95 50 02 00 00        	movq	%rdx, 0x250(%rbp)
1407da241: 48 89 55 e8                 	movq	%rdx, -0x18(%rbp)
1407da245: 4c 89 75 f0                 	movq	%r14, -0x10(%rbp)
1407da249: 49 8d 14 be                 	leaq	(%r14,%rdi,4), %rdx
1407da24d: 4c 39 cf                    	cmpq	%r9, %rdi
1407da250: 49 0f 43 f9                 	cmovaeq	%r9, %rdi
1407da254: 48 89 55 f8                 	movq	%rdx, -0x8(%rbp)
1407da258: 48 8d 14 f3                 	leaq	(%rbx,%rsi,8), %rdx
1407da25c: 48 39 fe                    	cmpq	%rdi, %rsi
1407da25f: 48 0f 43 f7                 	cmovaeq	%rdi, %rsi
1407da263: 48 c7 45 00 00 00 00 00     	movq	$0x0, (%rbp)
1407da26b: 48 89 7d 08                 	movq	%rdi, 0x8(%rbp)
1407da26f: 4c 89 4d 10                 	movq	%r9, 0x10(%rbp)
1407da273: 48 89 5d 18                 	movq	%rbx, 0x18(%rbp)
1407da277: 4c 8b 95 c0 04 00 00        	movq	0x4c0(%rbp), %r10
1407da27e: 4f 8d 04 a2                 	leaq	(%r10,%r12,4), %r8
1407da282: 49 39 f4                    	cmpq	%rsi, %r12
1407da285: 4c 0f 43 e6                 	cmovaeq	%rsi, %r12
1407da289: 48 89 55 20                 	movq	%rdx, 0x20(%rbp)
1407da28d: 48 c7 45 28 00 00 00 00     	movq	$0x0, 0x28(%rbp)
1407da295: 48 89 75 30                 	movq	%rsi, 0x30(%rbp)
1407da299: 48 89 7d 38                 	movq	%rdi, 0x38(%rbp)
1407da29d: 4c 89 55 40                 	movq	%r10, 0x40(%rbp)
1407da2a1: 4c 89 45 48                 	movq	%r8, 0x48(%rbp)
1407da2a5: 48 c7 45 50 00 00 00 00     	movq	$0x0, 0x50(%rbp)
1407da2ad: 4c 89 65 58                 	movq	%r12, 0x58(%rbp)
1407da2b1: 48 89 75 60                 	movq	%rsi, 0x60(%rbp)
1407da2b5: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
1407da2bd: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
1407da2c8: 49 8d 4c 8d f4              	leaq	-0xc(%r13,%rcx,4), %rcx
1407da2cd: 48 89 8d 48 02 00 00        	movq	%rcx, 0x248(%rbp)
1407da2d4: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
1407da2db: 48 83 c1 02                 	addq	$0x2, %rcx
1407da2df: 4c 89 ca                    	movq	%r9, %rdx
1407da2e2: 48 29 ca                    	subq	%rcx, %rdx
1407da2e5: 48 0f 42 d0                 	cmovbq	%rax, %rdx
1407da2e9: 48 89 95 90 01 00 00        	movq	%rdx, 0x190(%rbp)
1407da2f0: 49 83 c1 fe                 	addq	$-0x2, %r9
1407da2f4: 4c 89 8d 98 01 00 00        	movq	%r9, 0x198(%rbp)
1407da2fb: 49 8b 47 08                 	movq	0x8(%r15), %rax
1407da2ff: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407da306: 49 8b 47 10                 	movq	0x10(%r15), %rax
1407da30a: 48 01 c0                    	addq	%rax, %rax
1407da30d: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
1407da314: f2 41 0f 10 4f 50           	movsd	0x50(%r15), %xmm1
1407da31a: 0f 29 8d b0 00 00 00        	movaps	%xmm1, 0xb0(%rbp)
1407da321: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1407da325: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
1407da32c: f3 0f 12 c6                 	movsldup	%xmm6, %xmm0            # xmm0 = xmm6[0,0,2,2]
1407da330: 0f 29 85 d0 00 00 00        	movaps	%xmm0, 0xd0(%rbp)
1407da337: 4c 8d a5 d0 02 00 00        	leaq	0x2d0(%rbp), %r12
1407da33e: f3 0f 10 3d 2a b5 14 02     	movss	0x214b52a(%rip), %xmm7  # 0x142925870
1407da346: eb 10                       	jmp	0x1407da358 <.text+0x7d9358>
1407da348: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1407da350: f3 0f 10 3d 18 b5 14 02     	movss	0x214b518(%rip), %xmm7  # 0x142925870
1407da358: 4c 89 e1                    	movq	%r12, %rcx
1407da35b: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
1407da35f: e8 ac 34 ac 00              	callq	0x14129d810 <.text+0x129c810>
1407da364: 4c 8b b5 d0 02 00 00        	movq	0x2d0(%rbp), %r14
1407da36b: 4d 85 f6                    	testq	%r14, %r14
1407da36e: 0f 84 d1 0e 00 00           	je	0x1407db245 <.text+0x7da245>
1407da374: 4c 89 ad 30 03 00 00        	movq	%r13, 0x330(%rbp)
1407da37b: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
1407da382: 48 89 8d 48 03 00 00        	movq	%rcx, 0x348(%rbp)
1407da389: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
1407da38d: 4c 8b bd d8 02 00 00        	movq	0x2d8(%rbp), %r15
1407da394: 4c 8b ad e0 02 00 00        	movq	0x2e0(%rbp), %r13
1407da39b: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
1407da3a2: 48 8b bd f0 02 00 00        	movq	0x2f0(%rbp), %rdi
1407da3a9: 48 8b 9d f8 02 00 00        	movq	0x2f8(%rbp), %rbx
1407da3b0: 48 8b b5 00 03 00 00        	movq	0x300(%rbp), %rsi
1407da3b7: 4c 8b 85 08 03 00 00        	movq	0x308(%rbp), %r8
1407da3be: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
1407da3c5: f3 0f 10 57 04              	movss	0x4(%rdi), %xmm2
1407da3ca: f3 45 0f 10 6e 04           	movss	0x4(%r14), %xmm13
1407da3d0: 44 0f 28 e2                 	movaps	%xmm2, %xmm12
1407da3d4: f3 45 0f 5c e5              	subss	%xmm13, %xmm12
1407da3d9: 44 0f 54 25 9f 45 15 02     	andps	0x215459f(%rip), %xmm12 # 0x14292e980
1407da3e1: f3 44 0f 5f 25 1e b5 14 02  	maxss	0x214b51e(%rip), %xmm12 # 0x142925908
1407da3ea: 0f b7 8d a0 04 00 00        	movzwl	0x4a0(%rbp), %ecx
1407da3f1: f7 c1 00 01 00 00           	testl	$0x100, %ecx            # imm = 0x100
1407da3f7: 75 47                       	jne	0x1407da440 <.text+0x7d9440>
1407da3f9: 48 83 bd 48 02 00 00 00     	cmpq	$0x0, 0x248(%rbp)
1407da401: 0f 84 a0 0e 00 00           	je	0x1407db2a7 <.text+0x7da2a7>
1407da407: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
1407da40e: f3 0f 10 40 f8              	movss	-0x8(%rax), %xmm0
1407da413: 0f 2e 85 54 03 00 00        	ucomiss	0x354(%rbp), %xmm0
1407da41a: 76 74                       	jbe	0x1407da490 <.text+0x7d9490>
1407da41c: 48 8b 85 48 03 00 00        	movq	0x348(%rbp), %rax
1407da423: 48 3b 85 90 01 00 00        	cmpq	0x190(%rbp), %rax
1407da42a: 0f 97 c0                    	seta	%al
1407da42d: f6 c1 01                    	testb	$0x1, %cl
1407da430: 74 2f                       	je	0x1407da461 <.text+0x7d9461>
1407da432: eb 7c                       	jmp	0x1407da4b0 <.text+0x7d94b0>
1407da434: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1407da440: f6 c1 01                    	testb	$0x1, %cl
1407da443: 75 76                       	jne	0x1407da4bb <.text+0x7d94bb>
1407da445: 48 83 bd 48 02 00 00 00     	cmpq	$0x0, 0x248(%rbp)
1407da44d: 0f 84 60 0e 00 00           	je	0x1407db2b3 <.text+0x7da2b3>
1407da453: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
1407da45a: f3 0f 10 40 f8              	movss	-0x8(%rax), %xmm0
1407da45f: 31 c0                       	xorl	%eax, %eax
1407da461: f3 0f 10 8d 54 03 00 00     	movss	0x354(%rbp), %xmm1
1407da469: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
1407da46c: 76 42                       	jbe	0x1407da4b0 <.text+0x7d94b0>
1407da46e: 0f 28 85 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm0
1407da475: 48 8b 8d 48 03 00 00        	movq	0x348(%rbp), %rcx
1407da47c: 48 3b 8d 58 02 00 00        	cmpq	0x258(%rbp), %rcx
1407da483: 73 2b                       	jae	0x1407da4b0 <.text+0x7d94b0>
1407da485: eb 37                       	jmp	0x1407da4be <.text+0x7d94be>
1407da487: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1407da490: f6 c1 01                    	testb	$0x1, %cl
1407da493: 75 26                       	jne	0x1407da4bb <.text+0x7d94bb>
1407da495: 31 c0                       	xorl	%eax, %eax
1407da497: f3 0f 10 8d 54 03 00 00     	movss	0x354(%rbp), %xmm1
1407da49f: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
1407da4a2: 77 ca                       	ja	0x1407da46e <.text+0x7d946e>
1407da4a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1407da4b0: 0f 28 85 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm0
1407da4b7: 84 c0                       	testb	%al, %al
1407da4b9: 75 03                       	jne	0x1407da4be <.text+0x7d94be>
1407da4bb: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407da4be: f3 44 0f 58 e0              	addss	%xmm0, %xmm12
1407da4c3: 80 bd d0 04 00 00 00        	cmpb	$0x0, 0x4d0(%rbp)
1407da4ca: 48 89 95 60 02 00 00        	movq	%rdx, 0x260(%rbp)
1407da4d1: 4c 89 85 70 02 00 00        	movq	%r8, 0x270(%rbp)
1407da4d8: 74 05                       	je	0x1407da4df <.text+0x7d94df>
1407da4da: f3 44 0f 59 e7              	mulss	%xmm7, %xmm12
1407da4df: 41 0f 28 f5                 	movaps	%xmm13, %xmm6
1407da4e3: f3 41 0f c2 f5 03           	cmpunordss	%xmm13, %xmm6
1407da4e9: 0f 28 c6                    	movaps	%xmm6, %xmm0
1407da4ec: 0f 54 c2                    	andps	%xmm2, %xmm0
1407da4ef: 0f 29 95 40 01 00 00        	movaps	%xmm2, 0x140(%rbp)
1407da4f6: 0f 28 ca                    	movaps	%xmm2, %xmm1
1407da4f9: f3 41 0f 5f cd              	maxss	%xmm13, %xmm1
1407da4fe: 0f 55 f1                    	andnps	%xmm1, %xmm6
1407da501: 0f 56 f0                    	orps	%xmm0, %xmm6
1407da504: f3 41 0f 10 06              	movss	(%r14), %xmm0
1407da509: f3 0f 59 c7                 	mulss	%xmm7, %xmm0
1407da50d: f3 41 0f 10 5e 08           	movss	0x8(%r14), %xmm3
1407da513: f3 0f 59 df                 	mulss	%xmm7, %xmm3
1407da517: f3 0f 10 0f                 	movss	(%rdi), %xmm1
1407da51b: f3 0f 59 cf                 	mulss	%xmm7, %xmm1
1407da51f: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407da523: f3 0f 10 57 08              	movss	0x8(%rdi), %xmm2
1407da528: f3 0f 59 d7                 	mulss	%xmm7, %xmm2
1407da52c: f3 0f 58 d3                 	addss	%xmm3, %xmm2
1407da530: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1407da537: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
1407da53e: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
1407da545: 48 89 85 d8 02 00 00        	movq	%rax, 0x2d8(%rbp)
1407da54c: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
1407da553: 0f 13 85 e0 02 00 00        	movlps	%xmm0, 0x2e0(%rbp)
1407da55a: 4c 89 e1                    	movq	%r12, %rcx
1407da55d: e8 3e d6 a7 ff              	callq	0x140257ba0 <.text+0x256ba0>
1407da562: f3 0f 58 05 b6 b3 14 02     	addss	0x214b3b6(%rip), %xmm0  # 0x142925920
1407da56a: 44 0f 28 c6                 	movaps	%xmm6, %xmm8
1407da56e: f3 44 0f c2 c6 03           	cmpunordss	%xmm6, %xmm8
1407da574: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1407da578: 0f 54 c8                    	andps	%xmm0, %xmm1
1407da57b: f3 0f 5f c6                 	maxss	%xmm6, %xmm0
1407da57f: 44 0f 55 c0                 	andnps	%xmm0, %xmm8
1407da583: 44 0f 56 c1                 	orps	%xmm1, %xmm8
1407da587: f3 41 0f 10 06              	movss	(%r14), %xmm0
1407da58c: f3 41 0f 10 4e 08           	movss	0x8(%r14), %xmm1
1407da592: f3 0f 11 85 d0 02 00 00     	movss	%xmm0, 0x2d0(%rbp)
1407da59a: f3 0f 11 8d d4 02 00 00     	movss	%xmm1, 0x2d4(%rbp)
1407da5a2: 48 8d 8d fc 01 00 00        	leaq	0x1fc(%rbp), %rcx
1407da5a9: 4c 89 e2                    	movq	%r12, %rdx
1407da5ac: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1407da5b0: e8 2b 28 4c 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1407da5b5: f3 0f 10 07                 	movss	(%rdi), %xmm0
1407da5b9: f3 0f 10 4f 08              	movss	0x8(%rdi), %xmm1
1407da5be: f3 0f 11 85 d0 02 00 00     	movss	%xmm0, 0x2d0(%rbp)
1407da5c6: f3 0f 11 8d d4 02 00 00     	movss	%xmm1, 0x2d4(%rbp)
1407da5ce: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1407da5d5: 4c 89 e2                    	movq	%r12, %rdx
1407da5d8: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1407da5dc: e8 ff 27 4c 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1407da5e1: 45 0f 28 f4                 	movaps	%xmm12, %xmm14
1407da5e5: f3 44 0f 59 35 fa fa 17 02  	mulss	0x217fafa(%rip), %xmm14 # 0x14295a0e8
1407da5ee: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
1407da5f2: 45 0f 28 fe                 	movaps	%xmm14, %xmm15
1407da5f6: f3 45 0f 59 f9              	mulss	%xmm9, %xmm15
1407da5fb: f3 0f 10 b5 fc 01 00 00     	movss	0x1fc(%rbp), %xmm6
1407da603: f3 41 0f 58 f7              	addss	%xmm15, %xmm6
1407da608: f3 0f 10 85 00 02 00 00     	movss	0x200(%rbp), %xmm0
1407da610: f3 41 0f 58 c6              	addss	%xmm14, %xmm0
1407da615: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
1407da61c: f3 0f 10 85 04 02 00 00     	movss	0x204(%rbp), %xmm0
1407da624: f3 41 0f 58 c7              	addss	%xmm15, %xmm0
1407da629: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
1407da630: f3 41 0f 10 3f              	movss	(%r15), %xmm7
1407da635: f3 41 0f 10 45 00           	movss	(%r13), %xmm0
1407da63b: f3 41 0f 10 4d 04           	movss	0x4(%r13), %xmm1
1407da641: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407da644: 44 0f 28 05 a4 b2 14 02     	movaps	0x214b2a4(%rip), %xmm8  # 0x1429258f0
1407da64c: 41 0f 57 d0                 	xorps	%xmm8, %xmm2
1407da650: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1407da654: 0f 14 d0                    	unpcklps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
1407da657: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1407da65b: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407da65f: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
1407da663: f3 44 0f 10 15 98 b2 14 02  	movss	0x214b298(%rip), %xmm10 # 0x142925904
1407da66c: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407da670: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1407da674: f3 0f 12 c1                 	movsldup	%xmm1, %xmm0            # xmm0 = xmm1[0,0,2,2]
1407da678: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407da67b: 0f 13 85 08 02 00 00        	movlps	%xmm0, 0x208(%rbp)
1407da682: 4c 89 e1                    	movq	%r12, %rcx
1407da685: 48 8d bd 08 02 00 00        	leaq	0x208(%rbp), %rdi
1407da68c: 48 89 fa                    	movq	%rdi, %rdx
1407da68f: e8 2c 27 4c 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407da694: f3 0f 10 85 d0 02 00 00     	movss	0x2d0(%rbp), %xmm0
1407da69c: 0f 29 85 a0 02 00 00        	movaps	%xmm0, 0x2a0(%rbp)
1407da6a3: f3 0f 10 85 d4 02 00 00     	movss	0x2d4(%rbp), %xmm0
1407da6ab: 0f 29 85 80 02 00 00        	movaps	%xmm0, 0x280(%rbp)
1407da6b2: f3 0f 10 85 d8 02 00 00     	movss	0x2d8(%rbp), %xmm0
1407da6ba: 0f 29 85 90 02 00 00        	movaps	%xmm0, 0x290(%rbp)
1407da6c1: f3 41 0f 10 45 00           	movss	(%r13), %xmm0
1407da6c7: f3 41 0f 10 4d 04           	movss	0x4(%r13), %xmm1
1407da6cd: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407da6d0: 41 0f 57 d0                 	xorps	%xmm8, %xmm2
1407da6d4: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1407da6d8: 0f 14 d0                    	unpcklps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
1407da6db: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1407da6df: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407da6e3: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
1407da6e7: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407da6eb: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1407da6ef: f3 0f 12 c1                 	movsldup	%xmm1, %xmm0            # xmm0 = xmm1[0,0,2,2]
1407da6f3: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407da6f6: 0f 13 85 08 02 00 00        	movlps	%xmm0, 0x208(%rbp)
1407da6fd: 4c 89 e1                    	movq	%r12, %rcx
1407da700: 48 89 fa                    	movq	%rdi, %rdx
1407da703: e8 b8 26 4c 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407da708: f3 0f 10 85 d0 02 00 00     	movss	0x2d0(%rbp), %xmm0
1407da710: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1407da717: f3 0f 10 85 d4 02 00 00     	movss	0x2d4(%rbp), %xmm0
1407da71f: 0f 29 85 e0 00 00 00        	movaps	%xmm0, 0xe0(%rbp)
1407da726: f3 0f 10 85 d8 02 00 00     	movss	0x2d8(%rbp), %xmm0
1407da72e: 0f 29 85 d0 01 00 00        	movaps	%xmm0, 0x1d0(%rbp)
1407da735: f3 44 0f 10 9d f0 01 00 00  	movss	0x1f0(%rbp), %xmm11
1407da73e: f3 44 0f 58 b5 f4 01 00 00  	addss	0x1f4(%rbp), %xmm14
1407da747: f3 45 0f 58 df              	addss	%xmm15, %xmm11
1407da74c: f3 44 0f 58 bd f8 01 00 00  	addss	0x1f8(%rbp), %xmm15
1407da755: f3 0f 10 03                 	movss	(%rbx), %xmm0
1407da759: 0f 14 f8                    	unpcklps	%xmm0, %xmm7            # xmm7 = xmm7[0],xmm0[0],xmm7[1],xmm0[1]
1407da75c: 0f 5c bd c0 00 00 00        	subps	0xc0(%rbp), %xmm7
1407da763: f3 0f 10 06                 	movss	(%rsi), %xmm0
1407da767: f3 0f 10 4e 04              	movss	0x4(%rsi), %xmm1
1407da76c: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407da76f: 41 0f 57 d0                 	xorps	%xmm8, %xmm2
1407da773: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1407da777: 0f 14 d0                    	unpcklps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
1407da77a: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1407da77e: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407da782: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
1407da786: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407da78a: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1407da78e: f3 0f 12 c1                 	movsldup	%xmm1, %xmm0            # xmm0 = xmm1[0,0,2,2]
1407da792: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407da795: 0f 13 85 08 02 00 00        	movlps	%xmm0, 0x208(%rbp)
1407da79c: 4c 89 e1                    	movq	%r12, %rcx
1407da79f: 48 89 fa                    	movq	%rdi, %rdx
1407da7a2: e8 19 26 4c 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407da7a7: f3 0f 10 85 d0 02 00 00     	movss	0x2d0(%rbp), %xmm0
1407da7af: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
1407da7b6: f3 0f 10 85 d4 02 00 00     	movss	0x2d4(%rbp), %xmm0
1407da7be: 0f 29 85 c0 02 00 00        	movaps	%xmm0, 0x2c0(%rbp)
1407da7c5: f3 0f 10 85 d8 02 00 00     	movss	0x2d8(%rbp), %xmm0
1407da7cd: 0f 29 85 b0 02 00 00        	movaps	%xmm0, 0x2b0(%rbp)
1407da7d4: f3 0f 10 06                 	movss	(%rsi), %xmm0
1407da7d8: f3 0f 10 4e 04              	movss	0x4(%rsi), %xmm1
1407da7dd: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407da7e0: 41 0f 57 d0                 	xorps	%xmm8, %xmm2
1407da7e4: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1407da7e8: 0f 14 d0                    	unpcklps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
1407da7eb: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1407da7ef: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407da7f3: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
1407da7f7: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407da7fb: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1407da7ff: f3 0f 12 c1                 	movsldup	%xmm1, %xmm0            # xmm0 = xmm1[0,0,2,2]
1407da803: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407da806: 0f 13 85 08 02 00 00        	movlps	%xmm0, 0x208(%rbp)
1407da80d: 4c 89 e1                    	movq	%r12, %rcx
1407da810: 48 89 fa                    	movq	%rdi, %rdx
1407da813: e8 a8 25 4c 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407da818: f3 0f 10 85 d0 02 00 00     	movss	0x2d0(%rbp), %xmm0
1407da820: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
1407da827: f3 0f 10 85 d4 02 00 00     	movss	0x2d4(%rbp), %xmm0
1407da82f: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
1407da836: f3 0f 10 85 d8 02 00 00     	movss	0x2d8(%rbp), %xmm0
1407da83e: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
1407da845: 0f 28 8d d0 00 00 00        	movaps	0xd0(%rbp), %xmm1
1407da84c: 0f 28 c1                    	movaps	%xmm1, %xmm0
1407da84f: 0f 5f c7                    	maxps	%xmm7, %xmm0
1407da852: 0f c2 ff 03                 	cmpunordps	%xmm7, %xmm7
1407da856: 0f 54 cf                    	andps	%xmm7, %xmm1
1407da859: 0f 55 f8                    	andnps	%xmm0, %xmm7
1407da85c: 0f 56 f9                    	orps	%xmm1, %xmm7
1407da85f: 0f 28 c7                    	movaps	%xmm7, %xmm0
1407da862: 0f 59 05 77 b0 14 02        	mulps	0x214b077(%rip), %xmm0  # 0x1429258e0
1407da869: f3 44 0f 16 c0              	movshdup	%xmm0, %xmm8            # xmm8 = xmm0[1,1,3,3]
1407da86e: 44 0f 58 c0                 	addps	%xmm0, %xmm8
1407da872: f3 0f 10 05 76 3e 15 02     	movss	0x2153e76(%rip), %xmm0  # 0x14292e6f0
1407da87a: f3 0f 10 0d 12 9e 15 02     	movss	0x2159e12(%rip), %xmm1  # 0x142934694
1407da882: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1407da886: e8 15 5c 4b 00              	callq	0x140c904a0 <.text+0xc8f4a0>
1407da88b: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407da88e: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
1407da892: f3 44 0f 5d d1              	minss	%xmm1, %xmm10
1407da897: 45 0f 2e c1                 	ucomiss	%xmm9, %xmm8
1407da89b: 44 0f 29 ad 30 01 00 00     	movaps	%xmm13, 0x130(%rbp)
1407da8a3: 72 2b                       	jb	0x1407da8d0 <.text+0x7d98d0>
1407da8a5: ba 01 00 00 00              	movl	$0x1, %edx
1407da8aa: f3 0f 10 05 c6 b0 14 02     	movss	0x214b0c6(%rip), %xmm0  # 0x142925978
1407da8b2: 41 0f 2e c0                 	ucomiss	%xmm8, %xmm0
1407da8b6: 76 18                       	jbe	0x1407da8d0 <.text+0x7d98d0>
1407da8b8: 4c 8b b5 80 04 00 00        	movq	0x480(%rbp), %r14
1407da8bf: 4c 8b ad 30 03 00 00        	movq	0x330(%rbp), %r13
1407da8c6: e9 40 01 00 00              	jmp	0x1407daa0b <.text+0x7d9a0b>
1407da8cb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1407da8d0: 44 0f 2e 05 a0 b0 14 02     	ucomiss	0x214b0a0(%rip), %xmm8  # 0x142925978
1407da8d8: 4c 8b b5 80 04 00 00        	movq	0x480(%rbp), %r14
1407da8df: 4c 8b ad 30 03 00 00        	movq	0x330(%rbp), %r13
1407da8e6: 72 48                       	jb	0x1407da930 <.text+0x7d9930>
1407da8e8: f3 0f 10 05 38 28 1f 02     	movss	0x21f2838(%rip), %xmm0  # 0x1429cd128
1407da8f0: 41 0f 2e c0                 	ucomiss	%xmm8, %xmm0
1407da8f4: 76 3a                       	jbe	0x1407da930 <.text+0x7d9930>
1407da8f6: 49 8b 0e                    	movq	(%r14), %rcx
1407da8f9: 48 b8 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rax # imm = 0xA0761D6478BD642F
1407da903: 48 01 c1                    	addq	%rax, %rcx
1407da906: 49 89 0e                    	movq	%rcx, (%r14)
1407da909: 48 89 c8                    	movq	%rcx, %rax
1407da90c: 48 ba db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %rdx # imm = 0xE7037ED1A0B428DB
1407da916: 48 31 d0                    	xorq	%rdx, %rax
1407da919: 48 f7 e1                    	mulq	%rcx
1407da91c: 48 31 c2                    	xorq	%rax, %rdx
1407da91f: 48 c1 ea 3f                 	shrq	$0x3f, %rdx
1407da923: 48 ff c2                    	incq	%rdx
1407da926: e9 e0 00 00 00              	jmp	0x1407daa0b <.text+0x7d9a0b>
1407da92b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1407da930: f3 44 0f 10 0d cb af 14 02  	movss	0x214afcb(%rip), %xmm9  # 0x142925904
1407da939: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1407da93d: f3 41 0f 5c c2              	subss	%xmm10, %xmm0
1407da942: f3 0f 59 05 e6 26 1f 02     	mulss	0x21f26e6(%rip), %xmm0  # 0x1429cd030
1407da94a: 45 0f 28 ea                 	movaps	%xmm10, %xmm13
1407da94e: f3 44 0f 59 2d 59 b0 14 02  	mulss	0x214b059(%rip), %xmm13 # 0x1429259b0
1407da957: f3 44 0f 58 e8              	addss	%xmm0, %xmm13
1407da95c: 4c 89 f1                    	movq	%r14, %rcx
1407da95f: e8 1c 4a 4d 00              	callq	0x140caf380 <.text+0xcae380>
1407da964: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407da967: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
1407da96b: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1407da96f: f3 0f 5d c1                 	minss	%xmm1, %xmm0
1407da973: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407da976: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1407da97a: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1407da97e: f3 44 0f 10 0d f1 26 1f 02  	movss	0x21f26f1(%rip), %xmm9  # 0x1429cd078
1407da987: f3 44 0f 5c c8              	subss	%xmm0, %xmm9
1407da98c: f3 44 0f 59 c9              	mulss	%xmm1, %xmm9
1407da991: f3 45 0f 59 cd              	mulss	%xmm13, %xmm9
1407da996: f3 44 0f 58 0d c5 5d 1c 02  	addss	0x21c5dc5(%rip), %xmm9  # 0x1429a0764
1407da99f: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1407da9a3: f3 41 0f 5e c1              	divss	%xmm9, %xmm0
1407da9a8: e8 b3 94 14 02              	callq	0x142923e60 <.text+0x2922e60>
1407da9ad: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
1407da9b2: 48 89 c1                    	movq	%rax, %rcx
1407da9b5: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
1407da9b9: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407da9bc: f3 0f 5c 0d 18 f7 17 02     	subss	0x217f718(%rip), %xmm1  # 0x14295a0dc
1407da9c4: f3 48 0f 2c d1              	cvttss2si	%xmm1, %rdx
1407da9c9: 48 21 ca                    	andq	%rcx, %rdx
1407da9cc: 48 09 c2                    	orq	%rax, %rdx
1407da9cf: 0f 2e 05 ba 5c 18 02        	ucomiss	0x2185cba(%rip), %xmm0  # 0x142960690
1407da9d6: b8 00 00 00 00              	movl	$0x0, %eax
1407da9db: 48 0f 42 d0                 	cmovbq	%rax, %rdx
1407da9df: 0f 2e 05 fa f6 17 02        	ucomiss	0x217f6fa(%rip), %xmm0  # 0x14295a0e0
1407da9e6: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
1407da9ed: 48 0f 47 d0                 	cmovaq	%rax, %rdx
1407da9f1: 48 83 fa 01                 	cmpq	$0x1, %rdx
1407da9f5: 48 83 d2 00                 	adcq	$0x0, %rdx
1407da9f9: f3 44 0f 59 0d d6 ae 14 02  	mulss	0x214aed6(%rip), %xmm9  # 0x1429258d8
1407daa02: f3 44 0f 5f 0d 3d 39 2a 02  	maxss	0x22a393d(%rip), %xmm9  # 0x142a7e348
1407daa0b: 48 ff c2                    	incq	%rdx
1407daa0e: f3 45 0f 5e c8              	divss	%xmm8, %xmm9
1407daa13: 48 8d 05 8e d9 2a 02        	leaq	0x22ad98e(%rip), %rax   # 0x142a883a8
1407daa1a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407daa1f: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
1407daa26: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
1407daa2a: 4d 89 f1                    	movq	%r14, %r9
1407daa2d: e8 1e 59 4b 00              	callq	0x140c90350 <.text+0xc8f350>
1407daa32: 4c 89 f1                    	movq	%r14, %rcx
1407daa35: e8 46 49 4d 00              	callq	0x140caf380 <.text+0xcae380>
1407daa3a: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
1407daa3e: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
1407daa45: f3 0f 10 00                 	movss	(%rax), %xmm0
1407daa49: f3 0f 10 0d 1f ae 14 02     	movss	0x214ae1f(%rip), %xmm1  # 0x142925870
1407daa51: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
1407daa55: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
1407daa5c: f3 44 0f 10 28              	movss	(%rax), %xmm13
1407daa61: f3 44 0f 59 e9              	mulss	%xmm1, %xmm13
1407daa66: f3 44 0f 58 e8              	addss	%xmm0, %xmm13
1407daa6b: f3 0f 10 05 11 af 14 02     	movss	0x214af11(%rip), %xmm0  # 0x142925984
1407daa73: 41 0f 2e c5                 	ucomiss	%xmm13, %xmm0
1407daa77: 76 34                       	jbe	0x1407daaad <.text+0x7d9aad>
1407daa79: 80 bd d0 04 00 00 00        	cmpb	$0x0, 0x4d0(%rbp)
1407daa80: 74 2b                       	je	0x1407daaad <.text+0x7d9aad>
1407daa82: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
1407daa89: 48 85 d2                    	testq	%rdx, %rdx
1407daa8c: 0f 84 be f8 ff ff           	je	0x1407da350 <.text+0x7d9350>
1407daa92: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
1407daa99: 48 c1 e2 02                 	shlq	$0x2, %rdx
1407daa9d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407daaa3: e8 78 91 e3 ff              	callq	0x140613c20 <.text+0x612c20>
1407daaa8: e9 a3 f8 ff ff              	jmp	0x1407da350 <.text+0x7d9350>
1407daaad: 0f 28 85 a0 02 00 00        	movaps	0x2a0(%rbp), %xmm0
1407daab4: 0f 14 85 f0 00 00 00        	unpcklps	0xf0(%rbp), %xmm0       # xmm0 = xmm0[0],mem[0],xmm0[1],mem[1]
1407daabb: 0f 28 8d 80 02 00 00        	movaps	0x280(%rbp), %xmm1
1407daac2: 0f 14 8d e0 00 00 00        	unpcklps	0xe0(%rbp), %xmm1       # xmm1 = xmm1[0],mem[0],xmm1[1],mem[1]
1407daac9: 0f 28 e1                    	movaps	%xmm1, %xmm4
1407daacc: 0f 28 8d 90 02 00 00        	movaps	0x290(%rbp), %xmm1
1407daad3: 0f 14 8d d0 01 00 00        	unpcklps	0x1d0(%rbp), %xmm1      # xmm1 = xmm1[0],mem[0],xmm1[1],mem[1]
1407daada: 0f 28 ad a0 01 00 00        	movaps	0x1a0(%rbp), %xmm5
1407daae1: 0f 14 ad 20 01 00 00        	unpcklps	0x120(%rbp), %xmm5      # xmm5 = xmm5[0],mem[0],xmm5[1],mem[1]
1407daae8: 0f 28 95 c0 02 00 00        	movaps	0x2c0(%rbp), %xmm2
1407daaef: 0f 14 95 10 01 00 00        	unpcklps	0x110(%rbp), %xmm2      # xmm2 = xmm2[0],mem[0],xmm2[1],mem[1]
1407daaf6: 0f 29 95 c0 02 00 00        	movaps	%xmm2, 0x2c0(%rbp)
1407daafd: 0f 28 9d b0 02 00 00        	movaps	0x2b0(%rbp), %xmm3
1407dab04: 0f 14 9d 00 01 00 00        	unpcklps	0x100(%rbp), %xmm3      # xmm3 = xmm3[0],mem[0],xmm3[1],mem[1]
1407dab0b: 66 0f 70 d7 a0              	pshufd	$0xa0, %xmm7, %xmm2     # xmm2 = xmm7[0,0,2,2]
1407dab10: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407dab13: 0f 59 e2                    	mulps	%xmm2, %xmm4
1407dab16: 0f 29 a5 80 02 00 00        	movaps	%xmm4, 0x280(%rbp)
1407dab1d: 0f 59 ca                    	mulps	%xmm2, %xmm1
1407dab20: f3 0f 12 d6                 	movsldup	%xmm6, %xmm2            # xmm2 = xmm6[0,0,2,2]
1407dab24: 0f 28 25 b5 ad 14 02        	movaps	0x214adb5(%rip), %xmm4  # 0x1429258e0
1407dab2b: 0f 59 c4                    	mulps	%xmm4, %xmm0
1407dab2e: 0f 5c f0                    	subps	%xmm0, %xmm6
1407dab31: 0f 58 c2                    	addps	%xmm2, %xmm0
1407dab34: 0f c6 c6 01                 	shufps	$0x1, %xmm6, %xmm0      # xmm0 = xmm0[1,0],xmm6[0,0]
1407dab38: 0f c6 c6 e2                 	shufps	$0xe2, %xmm6, %xmm0     # xmm0 = xmm0[2,0],xmm6[2,3]
1407dab3c: 0f 29 85 a0 02 00 00        	movaps	%xmm0, 0x2a0(%rbp)
1407dab43: 0f 28 b5 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm6
1407dab4a: f3 0f 12 d6                 	movsldup	%xmm6, %xmm2            # xmm2 = xmm6[0,0,2,2]
1407dab4e: 0f 28 a5 80 02 00 00        	movaps	0x280(%rbp), %xmm4
1407dab55: 0f 59 25 84 ad 14 02        	mulps	0x214ad84(%rip), %xmm4  # 0x1429258e0
1407dab5c: 0f 5c f4                    	subps	%xmm4, %xmm6
1407dab5f: 0f 58 e2                    	addps	%xmm2, %xmm4
1407dab62: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
1407dab69: f3 0f 12 d0                 	movsldup	%xmm0, %xmm2            # xmm2 = xmm0[0,0,2,2]
1407dab6d: 0f 29 95 30 03 00 00        	movaps	%xmm2, 0x330(%rbp)
1407dab74: 0f c6 e6 01                 	shufps	$0x1, %xmm6, %xmm4      # xmm4 = xmm4[1,0],xmm6[0,0]
1407dab78: 0f c6 e6 e2                 	shufps	$0xe2, %xmm6, %xmm4     # xmm4 = xmm4[2,0],xmm6[2,3]
1407dab7c: f3 41 0f 12 d3              	movsldup	%xmm11, %xmm2           # xmm2 = xmm11[0,0,2,2]
1407dab81: 0f 29 95 70 02 00 00        	movaps	%xmm2, 0x270(%rbp)
1407dab88: 0f 28 15 51 ad 14 02        	movaps	0x214ad51(%rip), %xmm2  # 0x1429258e0
1407dab8f: 0f 59 ca                    	mulps	%xmm2, %xmm1
1407dab92: 0f 5c c1                    	subps	%xmm1, %xmm0
1407dab95: 0f 58 8d 30 03 00 00        	addps	0x330(%rbp), %xmm1
1407dab9c: f3 41 0f 12 f6              	movsldup	%xmm14, %xmm6           # xmm6 = xmm14[0,0,2,2]
1407daba1: 0f 29 b5 60 02 00 00        	movaps	%xmm6, 0x260(%rbp)
1407daba8: 0f c6 c8 01                 	shufps	$0x1, %xmm0, %xmm1      # xmm1 = xmm1[1,0],xmm0[0,0]
1407dabac: 0f c6 c8 e2                 	shufps	$0xe2, %xmm0, %xmm1     # xmm1 = xmm1[2,0],xmm0[2,3]
1407dabb0: f3 41 0f 12 c7              	movsldup	%xmm15, %xmm0           # xmm0 = xmm15[0,0,2,2]
1407dabb5: 0f 29 85 30 03 00 00        	movaps	%xmm0, 0x330(%rbp)
1407dabbc: 66 0f 70 ff f5              	pshufd	$0xf5, %xmm7, %xmm7     # xmm7 = xmm7[1,1,3,3]
1407dabc1: 0f 59 ef                    	mulps	%xmm7, %xmm5
1407dabc4: 0f 59 ea                    	mulps	%xmm2, %xmm5
1407dabc7: 0f 28 85 c0 02 00 00        	movaps	0x2c0(%rbp), %xmm0
1407dabce: 0f 59 c7                    	mulps	%xmm7, %xmm0
1407dabd1: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407dabd4: 0f 59 df                    	mulps	%xmm7, %xmm3
1407dabd7: 0f 59 da                    	mulps	%xmm2, %xmm3
1407dabda: 44 0f 5c dd                 	subps	%xmm5, %xmm11
1407dabde: 0f 58 ad 70 02 00 00        	addps	0x270(%rbp), %xmm5
1407dabe5: 41 0f c6 eb 01              	shufps	$0x1, %xmm11, %xmm5     # xmm5 = xmm5[1,0],xmm11[0,0]
1407dabea: 41 0f c6 eb e2              	shufps	$0xe2, %xmm11, %xmm5    # xmm5 = xmm5[2,0],xmm11[2,3]
1407dabef: 0f 28 f8                    	movaps	%xmm0, %xmm7
1407dabf2: 44 0f 5c f0                 	subps	%xmm0, %xmm14
1407dabf6: 0f 58 bd 60 02 00 00        	addps	0x260(%rbp), %xmm7
1407dabfd: 41 0f c6 fe 01              	shufps	$0x1, %xmm14, %xmm7     # xmm7 = xmm7[1,0],xmm14[0,0]
1407dac02: 41 0f c6 fe e2              	shufps	$0xe2, %xmm14, %xmm7    # xmm7 = xmm7[2,0],xmm14[2,3]
1407dac07: 44 0f 5c fb                 	subps	%xmm3, %xmm15
1407dac0b: 0f 58 9d 30 03 00 00        	addps	0x330(%rbp), %xmm3
1407dac12: 41 0f c6 df 01              	shufps	$0x1, %xmm15, %xmm3     # xmm3 = xmm3[1,0],xmm15[0,0]
1407dac17: 41 0f c6 df e2              	shufps	$0xe2, %xmm15, %xmm3    # xmm3 = xmm3[2,0],xmm15[2,3]
1407dac1c: 0f 29 8d 90 02 00 00        	movaps	%xmm1, 0x290(%rbp)
1407dac23: 0f 28 c1                    	movaps	%xmm1, %xmm0
1407dac26: 44 0f 28 f2                 	movaps	%xmm2, %xmm14
1407dac2a: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407dac2d: 0f 29 9d b0 02 00 00        	movaps	%xmm3, 0x2b0(%rbp)
1407dac34: 0f 59 da                    	mulps	%xmm2, %xmm3
1407dac37: 0f 58 d8                    	addps	%xmm0, %xmm3
1407dac3a: 44 0f 28 bd a0 02 00 00     	movaps	0x2a0(%rbp), %xmm15
1407dac42: 41 0f 28 c7                 	movaps	%xmm15, %xmm0
1407dac46: 0f 14 c4                    	unpcklps	%xmm4, %xmm0            # xmm0 = xmm0[0],xmm4[0],xmm0[1],xmm4[1]
1407dac49: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407dac4c: 0f 28 d5                    	movaps	%xmm5, %xmm2
1407dac4f: 0f 14 d7                    	unpcklps	%xmm7, %xmm2            # xmm2 = xmm2[0],xmm7[0],xmm2[1],xmm7[1]
1407dac52: 41 0f 59 d6                 	mulps	%xmm14, %xmm2
1407dac56: 0f 58 d0                    	addps	%xmm0, %xmm2
1407dac59: 0f 28 f4                    	movaps	%xmm4, %xmm6
1407dac5c: 41 0f c6 e7 11              	shufps	$0x11, %xmm15, %xmm4    # xmm4 = xmm4[1,0],xmm15[1,0]
1407dac61: 45 0f 28 df                 	movaps	%xmm15, %xmm11
1407dac65: 41 0f c6 e7 e2              	shufps	$0xe2, %xmm15, %xmm4    # xmm4 = xmm4[2,0],xmm15[2,3]
1407dac6a: 41 0f 59 e6                 	mulps	%xmm14, %xmm4
1407dac6e: 44 0f 28 ff                 	movaps	%xmm7, %xmm15
1407dac72: 0f 28 cf                    	movaps	%xmm7, %xmm1
1407dac75: 0f c6 cd 11                 	shufps	$0x11, %xmm5, %xmm1     # xmm1 = xmm1[1,0],xmm5[1,0]
1407dac79: 0f 28 fd                    	movaps	%xmm5, %xmm7
1407dac7c: 0f c6 cd e2                 	shufps	$0xe2, %xmm5, %xmm1     # xmm1 = xmm1[2,0],xmm5[2,3]
1407dac80: 41 0f 59 ce                 	mulps	%xmm14, %xmm1
1407dac84: 0f 58 cc                    	addps	%xmm4, %xmm1
1407dac87: 0f 29 95 60 02 00 00        	movaps	%xmm2, 0x260(%rbp)
1407dac8e: 0f 29 8d 70 02 00 00        	movaps	%xmm1, 0x270(%rbp)
1407dac95: f3 0f 5c d1                 	subss	%xmm1, %xmm2
1407dac99: f3 0f 16 c3                 	movshdup	%xmm3, %xmm0            # xmm0 = xmm3[1,1,3,3]
1407dac9d: 0f 29 9d 30 03 00 00        	movaps	%xmm3, 0x330(%rbp)
1407daca4: 0f 28 cb                    	movaps	%xmm3, %xmm1
1407daca7: f3 0f 5c c8                 	subss	%xmm0, %xmm1
1407dacab: 0f 28 c2                    	movaps	%xmm2, %xmm0
1407dacae: f3 0f 59 c2                 	mulss	%xmm2, %xmm0
1407dacb2: 0f 28 d9                    	movaps	%xmm1, %xmm3
1407dacb5: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
1407dacb9: f3 0f 58 d8                 	addss	%xmm0, %xmm3
1407dacbd: f3 0f 51 db                 	sqrtss	%xmm3, %xmm3
1407dacc1: f3 0f 10 05 3b ac 14 02     	movss	0x214ac3b(%rip), %xmm0  # 0x142925904
1407dacc9: f3 0f 5e c3                 	divss	%xmm3, %xmm0
1407daccd: 66 0f 7e c0                 	movd	%xmm0, %eax
1407dacd1: 85 c0                       	testl	%eax, %eax
1407dacd3: 0f 99 c1                    	setns	%cl
1407dacd6: 89 c2                       	movl	%eax, %edx
1407dacd8: 81 e2 ff ff ff 7f           	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
1407dacde: 81 c2 00 00 80 ff           	addl	$0xff800000, %edx       # imm = 0xFF800000
1407dace4: 81 fa 00 00 00 7f           	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
1407dacea: 0f 92 c2                    	setb	%dl
1407daced: 20 ca                       	andb	%cl, %dl
1407dacef: ff c8                       	decl	%eax
1407dacf1: 3d ff ff 7f 00              	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
1407dacf6: 0f 92 c0                    	setb	%al
1407dacf9: 08 d0                       	orb	%dl, %al
1407dacfb: 75 08                       	jne	0x1407dad05 <.text+0x7d9d05>
1407dacfd: 0f 57 d2                    	xorps	%xmm2, %xmm2
1407dad00: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407dad03: eb 0f                       	jmp	0x1407dad14 <.text+0x7d9d14>
1407dad05: 0f 57 0d e4 ab 14 02        	xorps	0x214abe4(%rip), %xmm1  # 0x1429258f0
1407dad0c: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1407dad10: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
1407dad14: 48 8d 8d 08 02 00 00        	leaq	0x208(%rbp), %rcx
1407dad1b: e8 b0 61 4b 00              	callq	0x140c90ed0 <.text+0xc8fed0>
1407dad20: 0f 28 ad 30 01 00 00        	movaps	0x130(%rbp), %xmm5
1407dad27: f3 0f 5c ad 40 01 00 00     	subss	0x140(%rbp), %xmm5
1407dad2f: 0f 54 2d 4a 3c 15 02        	andps	0x2153c4a(%rip), %xmm5  # 0x14292e980
1407dad36: 44 0f 5c df                 	subps	%xmm7, %xmm11
1407dad3a: 41 0f 5c f7                 	subps	%xmm15, %xmm6
1407dad3e: 0f 28 bd 90 02 00 00        	movaps	0x290(%rbp), %xmm7
1407dad45: 0f 5c bd b0 02 00 00        	subps	0x2b0(%rbp), %xmm7
1407dad4c: 45 0f 59 db                 	mulps	%xmm11, %xmm11
1407dad50: 0f 59 f6                    	mulps	%xmm6, %xmm6
1407dad53: 41 0f 58 f3                 	addps	%xmm11, %xmm6
1407dad57: 0f 59 ff                    	mulps	%xmm7, %xmm7
1407dad5a: 0f 58 fe                    	addps	%xmm6, %xmm7
1407dad5d: 44 0f 51 df                 	sqrtps	%xmm7, %xmm11
1407dad61: 48 8b 85 48 03 00 00        	movq	0x348(%rbp), %rax
1407dad68: 48 85 c0                    	testq	%rax, %rax
1407dad6b: 0f 94 c1                    	sete	%cl
1407dad6e: 48 3b 85 98 01 00 00        	cmpq	0x198(%rbp), %rax
1407dad75: 0f 94 c0                    	sete	%al
1407dad78: 08 c8                       	orb	%cl, %al
1407dad7a: 48 8b 95 38 02 00 00        	movq	0x238(%rbp), %rdx
1407dad81: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
1407dad88: 48 8d 3c 8a                 	leaq	(%rdx,%rcx,4), %rdi
1407dad8c: f3 44 0f 59 2d 17 ca 2a 02  	mulss	0x22aca17(%rip), %xmm13 # 0x142a877ac
1407dad95: f2 0f 10 85 08 02 00 00     	movsd	0x208(%rbp), %xmm0
1407dad9d: 0f 29 85 80 02 00 00        	movaps	%xmm0, 0x280(%rbp)
1407dada4: f3 0f 59 2d 0c bf 23 02     	mulss	0x223bf0c(%rip), %xmm5  # 0x142a16cb8
1407dadac: f3 0f 10 25 50 ab 14 02     	movss	0x214ab50(%rip), %xmm4  # 0x142925904
1407dadb4: 0f 28 c4                    	movaps	%xmm4, %xmm0
1407dadb7: f3 41 0f 5c c2              	subss	%xmm10, %xmm0
1407dadbc: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407dadbf: f3 0f 10 1d 59 ab 14 02     	movss	0x214ab59(%rip), %xmm3  # 0x142925920
1407dadc7: f3 0f 59 cb                 	mulss	%xmm3, %xmm1
1407dadcb: 41 0f 28 fa                 	movaps	%xmm10, %xmm7
1407dadcf: f3 0f 59 3d 01 ab 14 02     	mulss	0x214ab01(%rip), %xmm7  # 0x1429258d8
1407dadd7: f3 0f 58 f9                 	addss	%xmm1, %xmm7
1407daddb: 0f 28 cd                    	movaps	%xmm5, %xmm1
1407dadde: f3 0f 5d cf                 	minss	%xmm7, %xmm1
1407dade2: f3 0f c2 ff 03              	cmpunordss	%xmm7, %xmm7
1407dade7: 0f 28 d7                    	movaps	%xmm7, %xmm2
1407dadea: 0f 55 d1                    	andnps	%xmm1, %xmm2
1407daded: f3 0f 10 8d 10 02 00 00     	movss	0x210(%rbp), %xmm1
1407dadf5: f3 0f 11 8d 90 02 00 00     	movss	%xmm1, 0x290(%rbp)
1407dadfd: 0f 54 fd                    	andps	%xmm5, %xmm7
1407dae00: 0f 56 fa                    	orps	%xmm2, %xmm7
1407dae03: f3 0f 59 05 65 5f 15 02     	mulss	0x2155f65(%rip), %xmm0  # 0x142930d70
1407dae0b: f3 44 0f 59 d3              	mulss	%xmm3, %xmm10
1407dae10: f3 44 0f 58 d0              	addss	%xmm0, %xmm10
1407dae15: 0f 28 c5                    	movaps	%xmm5, %xmm0
1407dae18: f3 41 0f 5d c2              	minss	%xmm10, %xmm0
1407dae1d: f3 45 0f c2 d2 03           	cmpunordss	%xmm10, %xmm10
1407dae23: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407dae27: 0f 55 c8                    	andnps	%xmm0, %xmm1
1407dae2a: f2 0f 10 85 14 02 00 00     	movsd	0x214(%rbp), %xmm0
1407dae32: 0f 29 85 a0 02 00 00        	movaps	%xmm0, 0x2a0(%rbp)
1407dae39: 44 0f 54 d5                 	andps	%xmm5, %xmm10
1407dae3d: f3 0f 10 85 1c 02 00 00     	movss	0x21c(%rbp), %xmm0
1407dae45: f3 0f 11 85 b0 02 00 00     	movss	%xmm0, 0x2b0(%rbp)
1407dae4d: 44 0f 56 d1                 	orps	%xmm1, %xmm10
1407dae51: 0f 28 c4                    	movaps	%xmm4, %xmm0
1407dae54: f3 41 0f 5c c1              	subss	%xmm9, %xmm0
1407dae59: f3 41 0f 59 c2              	mulss	%xmm10, %xmm0
1407dae5e: f2 0f 10 8d 20 02 00 00     	movsd	0x220(%rbp), %xmm1
1407dae66: 0f 29 8d c0 02 00 00        	movaps	%xmm1, 0x2c0(%rbp)
1407dae6d: f3 41 0f 59 f9              	mulss	%xmm9, %xmm7
1407dae72: f3 0f 10 8d 28 02 00 00     	movss	0x228(%rbp), %xmm1
1407dae7a: f3 0f 11 8d a0 01 00 00     	movss	%xmm1, 0x1a0(%rbp)
1407dae82: f3 0f 58 f8                 	addss	%xmm0, %xmm7
1407dae86: f3 41 0f 12 c0              	movsldup	%xmm8, %xmm0            # xmm0 = xmm8[0,0,2,2]
1407dae8b: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
1407dae92: 0f b6 c0                    	movzbl	%al, %eax
1407dae95: 66 0f 6e c0                 	movd	%eax, %xmm0
1407dae99: 66 0f 70 c0 44              	pshufd	$0x44, %xmm0, %xmm0     # xmm0 = xmm0[0,1,0,1]
1407dae9e: 45 31 ff                    	xorl	%r15d, %r15d
1407daea1: b1 01                       	movb	$0x1, %cl
1407daea3: 66 0f 70 c0 88              	pshufd	$0x88, %xmm0, %xmm0     # xmm0 = xmm0[0,2,0,2]
1407daea8: 66 0f 72 f0 1f              	pslld	$0x1f, %xmm0
1407daead: 66 0f 72 e0 1f              	psrad	$0x1f, %xmm0
1407daeb2: 66 0f 7f 85 c0 01 00 00     	movdqa	%xmm0, 0x1c0(%rbp)
1407daeba: 48 89 95 d0 01 00 00        	movq	%rdx, 0x1d0(%rbp)
1407daec1: 48 89 d0                    	movq	%rdx, %rax
1407daec4: 48 39 f8                    	cmpq	%rdi, %rax
1407daec7: 0f 84 49 03 00 00           	je	0x1407db216 <.text+0x7da216>
1407daecd: 0f 1f 00                    	nopl	(%rax)
1407daed0: 48 8d 70 04                 	leaq	0x4(%rax), %rsi
1407daed4: f6 c1 01                    	testb	$0x1, %cl
1407daed7: 74 17                       	je	0x1407daef0 <.text+0x7d9ef0>
1407daed9: 48 39 fe                    	cmpq	%rdi, %rsi
1407daedc: 0f 84 34 03 00 00           	je	0x1407db216 <.text+0x7da216>
1407daee2: 48 8d 58 08                 	leaq	0x8(%rax), %rbx
1407daee6: 49 89 c5                    	movq	%rax, %r13
1407daee9: eb 0b                       	jmp	0x1407daef6 <.text+0x7d9ef6>
1407daeeb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1407daef0: 48 89 f3                    	movq	%rsi, %rbx
1407daef3: 48 89 c6                    	movq	%rax, %rsi
1407daef6: f3 41 0f 10 75 00           	movss	(%r13), %xmm6
1407daefc: f3 44 0f 10 06              	movss	(%rsi), %xmm8
1407daf01: 4c 89 f1                    	movq	%r14, %rcx
1407daf04: e8 77 44 4d 00              	callq	0x140caf380 <.text+0xcae380>
1407daf09: f3 0f 10 2d f3 a9 14 02     	movss	0x214a9f3(%rip), %xmm5  # 0x142925904
1407daf11: 0f 28 dd                    	movaps	%xmm5, %xmm3
1407daf14: f3 41 0f 5c d8              	subss	%xmm8, %xmm3
1407daf19: 0f 28 e5                    	movaps	%xmm5, %xmm4
1407daf1c: f3 0f 5c e6                 	subss	%xmm6, %xmm4
1407daf20: 0f 28 cc                    	movaps	%xmm4, %xmm1
1407daf23: 41 0f 14 c8                 	unpcklps	%xmm8, %xmm1            # xmm1 = xmm1[0],xmm8[0],xmm1[1],xmm8[1]
1407daf27: 44 0f 28 8d 30 03 00 00     	movaps	0x330(%rbp), %xmm9
1407daf2f: 41 0f 59 c9                 	mulps	%xmm9, %xmm1
1407daf33: 0f 28 d3                    	movaps	%xmm3, %xmm2
1407daf36: 0f 14 d6                    	unpcklps	%xmm6, %xmm2            # xmm2 = xmm2[0],xmm6[0],xmm2[1],xmm6[1]
1407daf39: 0f c6 d2 e1                 	shufps	$0xe1, %xmm2, %xmm2     # xmm2 = xmm2[1,0,2,3]
1407daf3d: 45 0f c6 c9 e1              	shufps	$0xe1, %xmm9, %xmm9     # xmm9 = xmm9[1,0,2,3]
1407daf42: 44 0f 59 ca                 	mulps	%xmm2, %xmm9
1407daf46: 44 0f 58 c9                 	addps	%xmm1, %xmm9
1407daf4a: f3 41 0f 59 c5              	mulss	%xmm13, %xmm0
1407daf4f: f3 41 0f 16 d1              	movshdup	%xmm9, %xmm2            # xmm2 = xmm9[1,1,3,3]
1407daf54: 0f 28 cd                    	movaps	%xmm5, %xmm1
1407daf57: f3 0f 5c c8                 	subss	%xmm0, %xmm1
1407daf5b: 41 0f 28 e9                 	movaps	%xmm9, %xmm5
1407daf5f: f3 0f 5c ea                 	subss	%xmm2, %xmm5
1407daf63: f3 0f 10 05 05 a9 14 02     	movss	0x214a905(%rip), %xmm0  # 0x142925870
1407daf6b: f3 44 0f 59 c8              	mulss	%xmm0, %xmm9
1407daf70: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
1407daf74: f3 41 0f 58 d1              	addss	%xmm9, %xmm2
1407daf79: f3 0f 12 c3                 	movsldup	%xmm3, %xmm0            # xmm0 = xmm3[0,0,2,2]
1407daf7d: f3 0f 12 dc                 	movsldup	%xmm4, %xmm3            # xmm3 = xmm4[0,0,2,2]
1407daf81: f3 41 0f 12 e0              	movsldup	%xmm8, %xmm4            # xmm4 = xmm8[0,0,2,2]
1407daf86: 44 0f 28 85 70 02 00 00     	movaps	0x270(%rbp), %xmm8
1407daf8e: 41 0f 59 e0                 	mulps	%xmm8, %xmm4
1407daf92: f3 0f 12 f6                 	movsldup	%xmm6, %xmm6            # xmm6 = xmm6[0,0,2,2]
1407daf96: 41 0f 59 f0                 	mulps	%xmm8, %xmm6
1407daf9a: 44 0f 28 85 60 02 00 00     	movaps	0x260(%rbp), %xmm8
1407dafa2: 41 0f 59 c0                 	mulps	%xmm8, %xmm0
1407dafa6: 0f 58 c4                    	addps	%xmm4, %xmm0
1407dafa9: 41 0f 59 d8                 	mulps	%xmm8, %xmm3
1407dafad: 0f 58 de                    	addps	%xmm6, %xmm3
1407dafb0: 0f 28 e3                    	movaps	%xmm3, %xmm4
1407dafb3: 0f 5c e0                    	subps	%xmm0, %xmm4
1407dafb6: 0f 59 e4                    	mulps	%xmm4, %xmm4
1407dafb9: f3 0f 16 f4                 	movshdup	%xmm4, %xmm6            # xmm6 = xmm4[1,1,3,3]
1407dafbd: f3 0f 58 f4                 	addss	%xmm4, %xmm6
1407dafc1: f3 0f 59 ed                 	mulss	%xmm5, %xmm5
1407dafc5: f3 0f 58 f5                 	addss	%xmm5, %xmm6
1407dafc9: 0f 57 e4                    	xorps	%xmm4, %xmm4
1407dafcc: f3 0f 51 e6                 	sqrtss	%xmm6, %xmm4
1407dafd0: f3 0f 59 25 d8 c7 2a 02     	mulss	0x22ac7d8(%rip), %xmm4  # 0x142a877b0
1407dafd8: f3 0f 59 e1                 	mulss	%xmm1, %xmm4
1407dafdc: 0f 28 2d fd a8 14 02        	movaps	0x214a8fd(%rip), %xmm5  # 0x1429258e0
1407dafe3: 0f 59 dd                    	mulps	%xmm5, %xmm3
1407dafe6: 0f 59 c5                    	mulps	%xmm5, %xmm0
1407dafe9: 0f 58 c3                    	addps	%xmm3, %xmm0
1407dafec: 0f 13 85 70 01 00 00        	movlps	%xmm0, 0x170(%rbp)
1407daff3: f3 0f 11 95 78 01 00 00     	movss	%xmm2, 0x178(%rbp)
1407daffb: f3 41 0f 10 75 00           	movss	(%r13), %xmm6
1407db001: f3 44 0f 10 36              	movss	(%rsi), %xmm14
1407db006: f3 0f 10 85 90 02 00 00     	movss	0x290(%rbp), %xmm0
1407db00e: f3 0f 59 c4                 	mulss	%xmm4, %xmm0
1407db012: f3 0f 12 d4                 	movsldup	%xmm4, %xmm2            # xmm2 = xmm4[0,0,2,2]
1407db016: 0f 59 95 80 02 00 00        	mulps	0x280(%rbp), %xmm2
1407db01d: 0f 13 95 60 01 00 00        	movlps	%xmm2, 0x160(%rbp)
1407db024: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
1407db028: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
1407db02c: f3 0f 11 85 68 01 00 00     	movss	%xmm0, 0x168(%rbp)
1407db034: f3 0f 10 85 b0 02 00 00     	movss	0x2b0(%rbp), %xmm0
1407db03c: f3 0f 59 c2                 	mulss	%xmm2, %xmm0
1407db040: f3 0f 12 d2                 	movsldup	%xmm2, %xmm2            # xmm2 = xmm2[0,0,2,2]
1407db044: 0f 59 95 a0 02 00 00        	mulps	0x2a0(%rbp), %xmm2
1407db04b: 0f 13 95 50 01 00 00        	movlps	%xmm2, 0x150(%rbp)
1407db052: f3 0f 11 85 58 01 00 00     	movss	%xmm0, 0x158(%rbp)
1407db05a: f3 0f 10 85 a0 01 00 00     	movss	0x1a0(%rbp), %xmm0
1407db062: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
1407db066: f3 0f 12 c9                 	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
1407db06a: 0f 59 8d c0 02 00 00        	mulps	0x2c0(%rbp), %xmm1
1407db071: 0f 13 8d d0 02 00 00        	movlps	%xmm1, 0x2d0(%rbp)
1407db078: f3 0f 11 85 d8 02 00 00     	movss	%xmm0, 0x2d8(%rbp)
1407db080: 48 8d 85 70 01 00 00        	leaq	0x170(%rbp), %rax
1407db087: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407db08c: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1407db090: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
1407db097: 4c 8d 85 50 01 00 00        	leaq	0x150(%rbp), %r8
1407db09e: 4d 89 e1                    	movq	%r12, %r9
1407db0a1: e8 5a 24 0c 00              	callq	0x14089d500 <.text+0x89c500>
1407db0a6: f3 45 0f 10 7d 00           	movss	(%r13), %xmm15
1407db0ac: 4c 89 f1                    	movq	%r14, %rcx
1407db0af: e8 cc 42 4d 00              	callq	0x140caf380 <.text+0xcae380>
1407db0b4: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
1407db0b8: f3 44 0f 58 3d 27 f0 17 02  	addss	0x217f027(%rip), %xmm15 # 0x14295a0e8
1407db0c1: f3 44 0f 59 0d a6 5c 15 02  	mulss	0x2155ca6(%rip), %xmm9  # 0x142930d70
1407db0ca: f3 44 0f 10 16              	movss	(%rsi), %xmm10
1407db0cf: 4c 89 f1                    	movq	%r14, %rcx
1407db0d2: e8 a9 42 4d 00              	callq	0x140caf380 <.text+0xcae380>
1407db0d7: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
1407db0db: 8b 8d 90 04 00 00           	movl	0x490(%rbp), %ecx
1407db0e1: e8 6a d6 07 00              	callq	0x140858750 <.text+0x857750>
1407db0e6: f3 0f 10 1d 16 a8 14 02     	movss	0x214a816(%rip), %xmm3  # 0x142925904
1407db0ee: 0f 28 c3                    	movaps	%xmm3, %xmm0
1407db0f1: f3 0f 5c c6                 	subss	%xmm6, %xmm0
1407db0f5: 0f 28 cb                    	movaps	%xmm3, %xmm1
1407db0f8: f3 41 0f 5c ce              	subss	%xmm14, %xmm1
1407db0fd: 0f 14 ce                    	unpcklps	%xmm6, %xmm1            # xmm1 = xmm1[0],xmm6[0],xmm1[1],xmm6[1]
1407db100: 0f c6 c9 e1                 	shufps	$0xe1, %xmm1, %xmm1     # xmm1 = xmm1[1,0,2,3]
1407db104: 41 0f 28 d3                 	movaps	%xmm11, %xmm2
1407db108: 41 0f c6 d3 e1              	shufps	$0xe1, %xmm11, %xmm2    # xmm2 = xmm2[1,0],xmm11[2,3]
1407db10d: 0f 59 d1                    	mulps	%xmm1, %xmm2
1407db110: 41 0f 14 c6                 	unpcklps	%xmm14, %xmm0           # xmm0 = xmm0[0],xmm14[0],xmm0[1],xmm14[1]
1407db114: 41 0f 59 c3                 	mulps	%xmm11, %xmm0
1407db118: 0f 58 c2                    	addps	%xmm2, %xmm0
1407db11b: 0f 28 15 5e 38 15 02        	movaps	0x215385e(%rip), %xmm2  # 0x14292e980
1407db122: 44 0f 54 fa                 	andps	%xmm2, %xmm15
1407db126: f3 45 0f 58 ff              	addss	%xmm15, %xmm15
1407db12b: 0f 28 cb                    	movaps	%xmm3, %xmm1
1407db12e: f3 41 0f 5c cf              	subss	%xmm15, %xmm1
1407db133: f3 0f 59 cf                 	mulss	%xmm7, %xmm1
1407db137: f3 44 0f 5c c9              	subss	%xmm1, %xmm9
1407db13c: f3 44 0f 59 05 2b 5c 15 02  	mulss	0x2155c2b(%rip), %xmm8  # 0x142930d70
1407db145: f3 44 0f 58 15 9a ef 17 02  	addss	0x217ef9a(%rip), %xmm10 # 0x14295a0e8
1407db14e: 44 0f 54 d2                 	andps	%xmm2, %xmm10
1407db152: f3 45 0f 58 d2              	addss	%xmm10, %xmm10
1407db157: 0f 28 cb                    	movaps	%xmm3, %xmm1
1407db15a: f3 41 0f 5c ca              	subss	%xmm10, %xmm1
1407db15f: f3 0f 59 cf                 	mulss	%xmm7, %xmm1
1407db163: f3 44 0f 5c c1              	subss	%xmm1, %xmm8
1407db168: 45 0f 14 c1                 	unpcklps	%xmm9, %xmm8            # xmm8 = xmm8[0],xmm9[0],xmm8[1],xmm9[1]
1407db16c: 48 8b 8d 48 03 00 00        	movq	0x348(%rbp), %rcx
1407db173: 41 8d 0c 0f                 	leal	(%r15,%rcx), %ecx
1407db177: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
1407db17b: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
1407db17f: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
1407db183: f3 0f 10 26                 	movss	(%rsi), %xmm4
1407db187: f3 41 0f 10 6d 00           	movss	(%r13), %xmm5
1407db18d: 0f 14 ec                    	unpcklps	%xmm4, %xmm5            # xmm5 = xmm5[0],xmm4[0],xmm5[1],xmm4[1]
1407db190: 0f 59 ad b0 01 00 00        	mulps	0x1b0(%rbp), %xmm5
1407db197: 0f 29 9d f0 02 00 00        	movaps	%xmm3, 0x2f0(%rbp)
1407db19e: 0f 29 95 e0 02 00 00        	movaps	%xmm2, 0x2e0(%rbp)
1407db1a5: 0f 29 8d d0 02 00 00        	movaps	%xmm1, 0x2d0(%rbp)
1407db1ac: f2 41 0f 12 c8              	movddup	%xmm8, %xmm1            # xmm1 = xmm8[0,0]
1407db1b1: 66 0f 28 95 c0 01 00 00     	movapd	0x1c0(%rbp), %xmm2
1407db1b9: 66 0f 55 d1                 	andnpd	%xmm1, %xmm2
1407db1bd: 66 0f 29 95 00 03 00 00     	movapd	%xmm2, 0x300(%rbp)
1407db1c5: 0f 13 85 10 03 00 00        	movlps	%xmm0, 0x310(%rbp)
1407db1cc: c7 85 18 03 00 00 00 00 00 00       	movl	$0x0, 0x318(%rbp)
1407db1d6: 89 8d 1c 03 00 00           	movl	%ecx, 0x31c(%rbp)
1407db1dc: 89 85 20 03 00 00           	movl	%eax, 0x320(%rbp)
1407db1e2: 0f 13 ad 24 03 00 00        	movlps	%xmm5, 0x324(%rbp)
1407db1e9: c7 85 2c 03 00 00 60 00 00 00       	movl	$0x60, 0x32c(%rbp)
1407db1f3: 48 8b 8d 88 04 00 00        	movq	0x488(%rbp), %rcx
1407db1fa: 4c 89 e2                    	movq	%r12, %rdx
1407db1fd: e8 ce 1b ab 00              	callq	0x14128cdd0 <.text+0x128bdd0>
1407db202: 41 ff c7                    	incl	%r15d
1407db205: 31 c9                       	xorl	%ecx, %ecx
1407db207: 48 89 d8                    	movq	%rbx, %rax
1407db20a: 49 89 f5                    	movq	%rsi, %r13
1407db20d: 48 39 f8                    	cmpq	%rdi, %rax
1407db210: 0f 85 ba fc ff ff           	jne	0x1407daed0 <.text+0x7d9ed0>
1407db216: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
1407db21d: 48 85 d2                    	testq	%rdx, %rdx
1407db220: 74 16                       	je	0x1407db238 <.text+0x7da238>
1407db222: 48 c1 e2 02                 	shlq	$0x2, %rdx
1407db226: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407db22c: 48 8b 8d d0 01 00 00        	movq	0x1d0(%rbp), %rcx
1407db233: e8 e8 89 e3 ff              	callq	0x140613c20 <.text+0x612c20>
1407db238: f3 0f 10 3d 30 a6 14 02     	movss	0x214a630(%rip), %xmm7  # 0x142925870
1407db240: e9 13 f1 ff ff              	jmp	0x1407da358 <.text+0x7d9358>
1407db245: 0f 28 b5 60 03 00 00        	movaps	0x360(%rbp), %xmm6
1407db24c: 0f 28 bd 70 03 00 00        	movaps	0x370(%rbp), %xmm7
1407db253: 44 0f 28 85 80 03 00 00     	movaps	0x380(%rbp), %xmm8
1407db25b: 44 0f 28 8d 90 03 00 00     	movaps	0x390(%rbp), %xmm9
1407db263: 44 0f 28 95 a0 03 00 00     	movaps	0x3a0(%rbp), %xmm10
1407db26b: 44 0f 28 9d b0 03 00 00     	movaps	0x3b0(%rbp), %xmm11
1407db273: 44 0f 28 a5 c0 03 00 00     	movaps	0x3c0(%rbp), %xmm12
1407db27b: 44 0f 28 ad d0 03 00 00     	movaps	0x3d0(%rbp), %xmm13
1407db283: 44 0f 28 b5 e0 03 00 00     	movaps	0x3e0(%rbp), %xmm14
1407db28b: 44 0f 28 bd f0 03 00 00     	movaps	0x3f0(%rbp), %xmm15
1407db293: 48 81 c4 88 04 00 00        	addq	$0x488, %rsp            # imm = 0x488
1407db29a: 5b                          	popq	%rbx
1407db29b: 5f                          	popq	%rdi
1407db29c: 5e                          	popq	%rsi
1407db29d: 41 5c                       	popq	%r12
1407db29f: 41 5d                       	popq	%r13
1407db2a1: 41 5e                       	popq	%r14
1407db2a3: 41 5f                       	popq	%r15
1407db2a5: 5d                          	popq	%rbp
1407db2a6: c3                          	retq
1407db2a7: 48 8d 0d ca d0 2a 02        	leaq	0x22ad0ca(%rip), %rcx   # 0x142a88378
1407db2ae: e8 5d e0 0f 02              	callq	0x1428d9310 <.text+0x28d8310>
1407db2b3: 48 8d 0d d6 d0 2a 02        	leaq	0x22ad0d6(%rip), %rcx   # 0x142a88390
