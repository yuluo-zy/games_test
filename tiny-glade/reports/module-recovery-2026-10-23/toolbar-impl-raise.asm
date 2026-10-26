
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140955650: 55                          	pushq	%rbp
140955651: 41 57                       	pushq	%r15
140955653: 41 56                       	pushq	%r14
140955655: 41 55                       	pushq	%r13
140955657: 41 54                       	pushq	%r12
140955659: 56                          	pushq	%rsi
14095565a: 57                          	pushq	%rdi
14095565b: 53                          	pushq	%rbx
14095565c: 48 81 ec 88 03 00 00        	subq	$0x388, %rsp            # imm = 0x388
140955663: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14095566b: 44 0f 29 bd f0 02 00 00     	movaps	%xmm15, 0x2f0(%rbp)
140955673: 44 0f 29 b5 e0 02 00 00     	movaps	%xmm14, 0x2e0(%rbp)
14095567b: 44 0f 29 ad d0 02 00 00     	movaps	%xmm13, 0x2d0(%rbp)
140955683: 44 0f 29 a5 c0 02 00 00     	movaps	%xmm12, 0x2c0(%rbp)
14095568b: 66 44 0f 7f 9d b0 02 00 00  	movdqa	%xmm11, 0x2b0(%rbp)
140955694: 44 0f 29 95 a0 02 00 00     	movaps	%xmm10, 0x2a0(%rbp)
14095569c: 44 0f 29 8d 90 02 00 00     	movaps	%xmm9, 0x290(%rbp)
1409556a4: 44 0f 29 85 80 02 00 00     	movaps	%xmm8, 0x280(%rbp)
1409556ac: 0f 29 bd 70 02 00 00        	movaps	%xmm7, 0x270(%rbp)
1409556b3: 0f 29 b5 60 02 00 00        	movaps	%xmm6, 0x260(%rbp)
1409556ba: 48 c7 85 58 02 00 00 fe ff ff ff    	movq	$-0x2, 0x258(%rbp)
1409556c5: 4d 8b 39                    	movq	(%r9), %r15
1409556c8: 4c 8b 22                    	movq	(%rdx), %r12
1409556cb: 48 8b 42 08                 	movq	0x8(%rdx), %rax
1409556cf: 4d 8b 1c 24                 	movq	(%r12), %r11
1409556d3: 45 31 d2                    	xorl	%r10d, %r10d
1409556d6: 4c 89 de                    	movq	%r11, %rsi
1409556d9: 48 2b 70 18                 	subq	0x18(%rax), %rsi
1409556dd: 49 0f 42 f2                 	cmovbq	%r10, %rsi
1409556e1: 4c 2b 58 38                 	subq	0x38(%rax), %r11
1409556e5: 4d 0f 42 da                 	cmovbq	%r10, %r11
1409556e9: 4c 8d 34 76                 	leaq	(%rsi,%rsi,2), %r14
1409556ed: 49 c1 e6 03                 	shlq	$0x3, %r14
1409556f1: 4c 03 70 08                 	addq	0x8(%rax), %r14
1409556f5: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1409556f9: 48 29 f2                    	subq	%rsi, %rdx
1409556fc: 49 0f 42 d2                 	cmovbq	%r10, %rdx
140955700: 48 8b 70 30                 	movq	0x30(%rax), %rsi
140955704: bb 08 00 00 00              	movl	$0x8, %ebx
140955709: 4c 0f 42 f3                 	cmovbq	%rbx, %r14
14095570d: 4b 8d 3c 5b                 	leaq	(%r11,%r11,2), %rdi
140955711: 48 c1 e7 03                 	shlq	$0x3, %rdi
140955715: 48 03 78 28                 	addq	0x28(%rax), %rdi
140955719: 4c 29 de                    	subq	%r11, %rsi
14095571c: 4d 8b 59 20                 	movq	0x20(%r9), %r11
140955720: 49 0f 42 f2                 	cmovbq	%r10, %rsi
140955724: 4d 8b 51 30                 	movq	0x30(%r9), %r10
140955728: 4c 89 95 d8 00 00 00        	movq	%r10, 0xd8(%rbp)
14095572f: 48 0f 42 fb                 	cmovbq	%rbx, %rdi
140955733: 48 8b 40 40                 	movq	0x40(%rax), %rax
140955737: 48 29 f0                    	subq	%rsi, %rax
14095573a: 48 29 d0                    	subq	%rdx, %rax
14095573d: 45 8b 51 3c                 	movl	0x3c(%r9), %r10d
140955741: 44 89 95 cc 01 00 00        	movl	%r10d, 0x1cc(%rbp)
140955748: 4d 8b 51 40                 	movq	0x40(%r9), %r10
14095574c: 4c 89 95 38 01 00 00        	movq	%r10, 0x138(%rbp)
140955753: 4d 8b 51 50                 	movq	0x50(%r9), %r10
140955757: 4c 89 95 30 01 00 00        	movq	%r10, 0x130(%rbp)
14095575e: 45 8b 49 5c                 	movl	0x5c(%r9), %r9d
140955762: 44 89 8d d4 01 00 00        	movl	%r9d, 0x1d4(%rbp)
140955769: 4d 8b 48 20                 	movq	0x20(%r8), %r9
14095576d: 4d 8b 50 30                 	movq	0x30(%r8), %r10
140955771: 4c 89 95 78 01 00 00        	movq	%r10, 0x178(%rbp)
140955778: 45 8b 50 3c                 	movl	0x3c(%r8), %r10d
14095577c: 44 89 95 e4 01 00 00        	movl	%r10d, 0x1e4(%rbp)
140955783: 4c 89 a5 c0 00 00 00        	movq	%r12, 0xc0(%rbp)
14095578a: 49 89 04 24                 	movq	%rax, (%r12)
14095578e: 48 8d 04 52                 	leaq	(%rdx,%rdx,2), %rax
140955792: 4d 8d 2c c6                 	leaq	(%r14,%rax,8), %r13
140955796: 48 8d 04 76                 	leaq	(%rsi,%rsi,2), %rax
14095579a: 48 8d 34 c7                 	leaq	(%rdi,%rax,8), %rsi
14095579e: 4c 8b 21                    	movq	(%rcx), %r12
1409557a1: 48 8b 85 a8 03 00 00        	movq	0x3a8(%rbp), %rax
1409557a8: 48 8b 00                    	movq	(%rax), %rax
1409557ab: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1409557b2: 48 8b 85 98 03 00 00        	movq	0x398(%rbp), %rax
1409557b9: 48 8b 00                    	movq	(%rax), %rax
1409557bc: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1409557c3: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
1409557ca: 48 8b 00                    	movq	(%rax), %rax
1409557cd: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1409557d4: 48 8b 85 78 03 00 00        	movq	0x378(%rbp), %rax
1409557db: 48 8b 18                    	movq	(%rax), %rbx
1409557de: 48 8b 85 80 03 00 00        	movq	0x380(%rbp), %rax
1409557e5: 48 8b 08                    	movq	(%rax), %rcx
1409557e8: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1409557ec: 48 89 95 08 01 00 00        	movq	%rdx, 0x108(%rbp)
1409557f3: 8b 40 1c                    	movl	0x1c(%rax), %eax
1409557f6: 89 85 d0 01 00 00           	movl	%eax, 0x1d0(%rbp)
1409557fc: 48 8b 85 88 03 00 00        	movq	0x388(%rbp), %rax
140955803: 48 8b 00                    	movq	(%rax), %rax
140955806: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
14095580d: 48 8b 85 b0 03 00 00        	movq	0x3b0(%rbp), %rax
140955814: 8b 50 1c                    	movl	0x1c(%rax), %edx
140955817: 89 95 d8 01 00 00           	movl	%edx, 0x1d8(%rbp)
14095581d: 48 8b 10                    	movq	(%rax), %rdx
140955820: 48 8b 40 10                 	movq	0x10(%rax), %rax
140955824: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
14095582b: 48 8b 85 b8 03 00 00        	movq	0x3b8(%rbp), %rax
140955832: 44 8b 50 1c                 	movl	0x1c(%rax), %r10d
140955836: 44 89 95 dc 01 00 00        	movl	%r10d, 0x1dc(%rbp)
14095583d: 4c 8b 10                    	movq	(%rax), %r10
140955840: 48 8b 40 10                 	movq	0x10(%rax), %rax
140955844: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
14095584b: 41 8b 40 1c                 	movl	0x1c(%r8), %eax
14095584f: 89 85 e0 01 00 00           	movl	%eax, 0x1e0(%rbp)
140955855: 49 8b 00                    	movq	(%r8), %rax
140955858: 4d 8b 40 10                 	movq	0x10(%r8), %r8
14095585c: 4c 89 85 70 01 00 00        	movq	%r8, 0x170(%rbp)
140955863: 44 0f 28 0d 15 91 fd 01     	movaps	0x1fd9115(%rip), %xmm9  # 0x14292e980
14095586b: 45 0f 57 d2                 	xorps	%xmm10, %xmm10
14095586f: 66 44 0f 6e 35 d0 df 0b 02  	movd	0x20bdfd0(%rip), %xmm14 # 0x142a13848
140955878: f3 44 0f 10 05 83 00 fd 01  	movss	0x1fd0083(%rip), %xmm8  # 0x142925904
140955881: f3 44 0f 10 25 42 c0 07 02  	movss	0x207c042(%rip), %xmm12 # 0x1429d18cc
14095588a: 66 45 0f 76 db              	pcmpeqd	%xmm11, %xmm11
14095588f: f3 44 0f 10 2d bc 00 fd 01  	movss	0x1fd00bc(%rip), %xmm13 # 0x142925954
140955898: f2 44 0f 10 3d 4f 85 12 02  	movsd	0x212854f(%rip), %xmm15 # 0x142a7ddf0
1409558a1: 4c 8d 41 20                 	leaq	0x20(%rcx), %r8
1409558a5: 4c 89 85 90 01 00 00        	movq	%r8, 0x190(%rbp)
1409558ac: 48 89 95 a0 01 00 00        	movq	%rdx, 0x1a0(%rbp)
1409558b3: 48 8d 52 20                 	leaq	0x20(%rdx), %rdx
1409558b7: 48 89 95 58 01 00 00        	movq	%rdx, 0x158(%rbp)
1409558be: 4c 89 95 a8 01 00 00        	movq	%r10, 0x1a8(%rbp)
1409558c5: 49 8d 52 20                 	leaq	0x20(%r10), %rdx
1409558c9: 49 89 f2                    	movq	%rsi, %r10
1409558cc: 48 89 95 60 01 00 00        	movq	%rdx, 0x160(%rbp)
1409558d3: 4c 89 9d d0 00 00 00        	movq	%r11, 0xd0(%rbp)
1409558da: 49 8d 53 30                 	leaq	0x30(%r11), %rdx
1409558de: 48 89 95 f8 00 00 00        	movq	%rdx, 0xf8(%rbp)
1409558e5: 4c 89 bd c8 00 00 00        	movq	%r15, 0xc8(%rbp)
1409558ec: 49 8d 97 90 00 00 00        	leaq	0x90(%r15), %rdx
1409558f3: 48 89 95 00 01 00 00        	movq	%rdx, 0x100(%rbp)
1409558fa: 49 89 cf                    	movq	%rcx, %r15
1409558fd: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
140955901: 48 89 8d 48 01 00 00        	movq	%rcx, 0x148(%rbp)
140955908: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14095590f: 48 8d 40 20                 	leaq	0x20(%rax), %rax
140955913: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
14095591a: 4c 89 8d b8 01 00 00        	movq	%r9, 0x1b8(%rbp)
140955921: 49 8d 41 20                 	leaq	0x20(%r9), %rax
140955925: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
14095592c: 49 8d 44 24 4c              	leaq	0x4c(%r12), %rax
140955931: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140955938: 48 89 9d 30 02 00 00        	movq	%rbx, 0x230(%rbp)
14095593f: 4c 89 bd 40 02 00 00        	movq	%r15, 0x240(%rbp)
140955946: 4c 89 ad 98 01 00 00        	movq	%r13, 0x198(%rbp)
14095594d: 48 89 b5 38 02 00 00        	movq	%rsi, 0x238(%rbp)
140955954: 4d 85 f6                    	testq	%r14, %r14
140955957: 74 17                       	je	0x140955970 <.text+0x954970>
140955959: 4d 39 ee                    	cmpq	%r13, %r14
14095595c: 74 12                       	je	0x140955970 <.text+0x954970>
14095595e: 49 8d 4e 18                 	leaq	0x18(%r14), %rcx
140955962: 4c 89 f0                    	movq	%r14, %rax
140955965: eb 24                       	jmp	0x14095598b <.text+0x95498b>
140955967: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140955970: 48 85 ff                    	testq	%rdi, %rdi
140955973: 0f 84 eb 0a 00 00           	je	0x140956464 <.text+0x955464>
140955979: 4c 39 d7                    	cmpq	%r10, %rdi
14095597c: 0f 84 e2 0a 00 00           	je	0x140956464 <.text+0x955464>
140955982: 48 89 f8                    	movq	%rdi, %rax
140955985: 48 83 c7 18                 	addq	$0x18, %rdi
140955989: 31 c9                       	xorl	%ecx, %ecx
14095598b: 49 89 ce                    	movq	%rcx, %r14
14095598e: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
140955995: 48 ff 01                    	incq	(%rcx)
140955998: 48 8b 30                    	movq	(%rax), %rsi
14095599b: b1 02                       	movb	$0x2, %cl
14095599d: 41 80 7c 24 18 02           	cmpb	$0x2, 0x18(%r12)
1409559a3: 72 0d                       	jb	0x1409559b2 <.text+0x9549b2>
1409559a5: 49 39 74 24 38              	cmpq	%rsi, 0x38(%r12)
1409559aa: 75 06                       	jne	0x1409559b2 <.text+0x9549b2>
1409559ac: 41 0f b6 4c 24 19           	movzbl	0x19(%r12), %ecx
1409559b2: 80 c1 fe                    	addb	$-0x2, %cl
1409559b5: 0f b6 c9                    	movzbl	%cl, %ecx
1409559b8: 80 f9 03                    	cmpb	$0x3, %cl
1409559bb: ba 03 00 00 00              	movl	$0x3, %edx
1409559c0: 0f 43 ca                    	cmovael	%edx, %ecx
1409559c3: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1409559c7: 0f b6 c1                    	movzbl	%cl, %eax
1409559ca: 48 8d 0d 4f f0 15 02        	leaq	0x215f04f(%rip), %rcx   # 0x142ab4a20
1409559d1: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1409559d5: 48 01 c8                    	addq	%rcx, %rax
1409559d8: 48 89 95 48 02 00 00        	movq	%rdx, 0x248(%rbp)
1409559df: ff e0                       	jmpq	*%rax
1409559e1: 48 89 b5 28 02 00 00        	movq	%rsi, 0x228(%rbp)
1409559e8: 4c 89 b5 20 01 00 00        	movq	%r14, 0x120(%rbp)
1409559ef: 48 89 bd 28 01 00 00        	movq	%rdi, 0x128(%rbp)
1409559f6: 4c 89 ee                    	movq	%r13, %rsi
1409559f9: 4c 89 a5 18 01 00 00        	movq	%r12, 0x118(%rbp)
140955a00: 4c 8b a5 d8 00 00 00        	movq	0xd8(%rbp), %r12
140955a07: 44 8b ad cc 01 00 00        	movl	0x1cc(%rbp), %r13d
140955a0e: 45 89 2c 24                 	movl	%r13d, (%r12)
140955a12: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
140955a1a: 48 89 55 c0                 	movq	%rdx, -0x40(%rbp)
140955a1e: 48 c7 45 c8 01 00 00 00     	movq	$0x1, -0x38(%rbp)
140955a26: 48 8d 05 b3 ec 15 02        	leaq	0x215ecb3(%rip), %rax   # 0x142ab46e0
140955a2d: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140955a32: 48 8b 85 90 03 00 00        	movq	0x390(%rbp), %rax
140955a39: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140955a3e: 48 c7 44 24 30 10 00 00 00  	movq	$0x10, 0x30(%rsp)
140955a47: 49 89 d6                    	movq	%rdx, %r14
140955a4a: 41 b8 02 00 00 00           	movl	$0x2, %r8d
140955a50: 4c 8b bd d0 00 00 00        	movq	0xd0(%rbp), %r15
140955a57: 4c 89 f9                    	movq	%r15, %rcx
140955a5a: 48 8d 55 b8                 	leaq	-0x48(%rbp), %rdx
140955a5e: 48 8b 9d c8 00 00 00        	movq	0xc8(%rbp), %rbx
140955a65: 49 89 d9                    	movq	%rbx, %r9
140955a68: e8 83 1d a9 00              	callq	0x1413e77f0 <.text+0x13e67f0>
140955a6d: 45 89 2c 24                 	movl	%r13d, (%r12)
140955a71: 4c 89 f9                    	movq	%r15, %rcx
140955a74: 4c 89 f2                    	movq	%r14, %rdx
140955a77: 49 89 d8                    	movq	%rbx, %r8
140955a7a: e8 81 61 1d 00              	callq	0x140b2bc00 <.text+0xb2ac00>
140955a7f: 45 89 2c 24                 	movl	%r13d, (%r12)
140955a83: 4c 8b a5 18 01 00 00        	movq	0x118(%rbp), %r12
140955a8a: 48 8b 83 90 00 00 00        	movq	0x90(%rbx), %rax
140955a91: 48 85 c0                    	testq	%rax, %rax
140955a94: 48 0f 45 85 00 01 00 00     	cmovneq	0x100(%rbp), %rax
140955a9c: 48 c7 45 b8 01 00 00 00     	movq	$0x1, -0x48(%rbp)
140955aa4: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140955aa8: 48 c7 45 c8 00 00 00 00     	movq	$0x0, -0x38(%rbp)
140955ab0: 48 c7 45 f0 00 00 00 00     	movq	$0x0, -0x10(%rbp)
140955ab8: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
140955abc: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
140955ac3: e8 b8 d7 93 00              	callq	0x141293280 <.text+0x1292280>
140955ac8: 41 80 7c 24 18 00           	cmpb	$0x0, 0x18(%r12)
140955ace: 0f 84 0a 0a 00 00           	je	0x1409564de <.text+0x9554de>
140955ad4: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
140955adb: 49 39 44 24 38              	cmpq	%rax, 0x38(%r12)
140955ae0: 0f 85 f8 09 00 00           	jne	0x1409564de <.text+0x9554de>
140955ae6: 41 83 7c 24 28 02           	cmpl	$0x2, 0x28(%r12)
140955aec: 0f 84 ec 09 00 00           	je	0x1409564de <.text+0x9554de>
140955af2: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
140955af9: 8b 42 08                    	movl	0x8(%rdx), %eax
140955afc: 48 8b 8d 70 03 00 00        	movq	0x370(%rbp), %rcx
140955b03: 89 41 08                    	movl	%eax, 0x8(%rcx)
140955b06: 48 8b 02                    	movq	(%rdx), %rax
140955b09: 48 89 01                    	movq	%rax, (%rcx)
140955b0c: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
140955b13: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
140955b17: 4c 8b bd 40 02 00 00        	movq	0x240(%rbp), %r15
140955b1e: 49 83 7f 18 00              	cmpq	$0x0, 0x18(%r15)
140955b23: 48 8b bd 28 01 00 00        	movq	0x128(%rbp), %rdi
140955b2a: 4c 8b b5 20 01 00 00        	movq	0x120(%rbp), %r14
140955b31: 48 8b 9d 30 02 00 00        	movq	0x230(%rbp), %rbx
140955b38: 0f 84 0e 09 00 00           	je	0x14095644c <.text+0x95544c>
140955b3e: 49 89 f5                    	movq	%rsi, %r13
140955b41: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
140955b48: 48 8d 55 b8                 	leaq	-0x48(%rbp), %rdx
140955b4c: e8 1f b2 c5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140955b51: 49 8b 0f                    	movq	(%r15), %rcx
140955b54: 49 8b 57 08                 	movq	0x8(%r15), %rdx
140955b58: 49 89 d0                    	movq	%rdx, %r8
140955b5b: 49 21 c0                    	andq	%rax, %r8
140955b5e: 48 c1 e8 39                 	shrq	$0x39, %rax
140955b62: 66 0f 6e c0                 	movd	%eax, %xmm0
140955b66: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140955b6a: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140955b6f: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140955b74: 48 8d 41 88                 	leaq	-0x78(%rcx), %rax
140955b78: 31 f6                       	xorl	%esi, %esi
140955b7a: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140955b80: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140955b84: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140955b88: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140955b8d: 45 85 d2                    	testl	%r10d, %r10d
140955b90: 74 3e                       	je	0x140955bd0 <.text+0x954bd0>
140955b92: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140955b97: 4d 01 c3                    	addq	%r8, %r11
140955b9a: 49 21 d3                    	andq	%rdx, %r11
140955b9d: 49 f7 db                    	negq	%r11
140955ba0: 4d 6b db 78                 	imulq	$0x78, %r11, %r11
140955ba4: 4c 8b 8d 48 02 00 00        	movq	0x248(%rbp), %r9
140955bab: 4e 39 0c 18                 	cmpq	%r9, (%rax,%r11)
140955baf: 0f 84 f8 06 00 00           	je	0x1409562ad <.text+0x9552ad>
140955bb5: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140955bb9: 66 45 21 d3                 	andw	%r10w, %r11w
140955bbd: 45 89 da                    	movl	%r11d, %r10d
140955bc0: 75 d0                       	jne	0x140955b92 <.text+0x954b92>
140955bc2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140955bd0: 66 41 0f 74 cb              	pcmpeqb	%xmm11, %xmm1
140955bd5: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140955bda: 45 85 d2                    	testl	%r10d, %r10d
140955bdd: 0f 85 69 08 00 00           	jne	0x14095644c <.text+0x95544c>
140955be3: 49 01 f0                    	addq	%rsi, %r8
140955be6: 49 83 c0 10                 	addq	$0x10, %r8
140955bea: 48 83 c6 10                 	addq	$0x10, %rsi
140955bee: 49 21 d0                    	andq	%rdx, %r8
140955bf1: eb 87                       	jmp	0x140955b7a <.text+0x954b7a>
140955bf3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140955c00: 48 89 b5 28 02 00 00        	movq	%rsi, 0x228(%rbp)
140955c07: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140955c0e: 8b 01                       	movl	(%rcx), %eax
140955c10: 0f 57 d2                    	xorps	%xmm2, %xmm2
140955c13: f3 48 0f 2a d0              	cvtsi2ss	%rax, %xmm2
140955c18: 8b 41 04                    	movl	0x4(%rcx), %eax
140955c1b: 0f 57 db                    	xorps	%xmm3, %xmm3
140955c1e: f3 48 0f 2a d8              	cvtsi2ss	%rax, %xmm3
140955c23: 0f 28 fa                    	movaps	%xmm2, %xmm7
140955c26: f3 0f 5e fb                 	divss	%xmm3, %xmm7
140955c2a: f3 0f 11 95 e8 01 00 00     	movss	%xmm2, 0x1e8(%rbp)
140955c32: f3 0f 11 9d ec 01 00 00     	movss	%xmm3, 0x1ec(%rbp)
140955c3a: f3 0f 11 bd f0 01 00 00     	movss	%xmm7, 0x1f0(%rbp)
140955c42: c7 85 f4 01 00 00 00 00 80 3f       	movl	$0x3f800000, 0x1f4(%rbp) # imm = 0x3F800000
140955c4c: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
140955c53: f3 0f 10 00                 	movss	(%rax), %xmm0
140955c57: f3 0f 10 48 04              	movss	0x4(%rax), %xmm1
140955c5c: e8 6f aa 33 00              	callq	0x140c906d0 <.text+0xc8f6d0>
140955c61: 0f 28 d1                    	movaps	%xmm1, %xmm2
140955c64: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
140955c68: 48 8b b5 70 03 00 00        	movq	0x370(%rbp), %rsi
140955c6f: 8b 46 08                    	movl	0x8(%rsi), %eax
140955c72: 89 45 c0                    	movl	%eax, -0x40(%rbp)
140955c75: 48 8b 06                    	movq	(%rsi), %rax
140955c78: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
140955c7c: 48 8d 45 c4                 	leaq	-0x3c(%rbp), %rax
140955c80: 48 b9 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rcx # imm = 0x3F80000000000000
140955c8a: 48 89 08                    	movq	%rcx, (%rax)
140955c8d: c7 40 08 00 00 00 00        	movl	$0x0, 0x8(%rax)
140955c94: f2 44 0f 11 bd 00 02 00 00  	movsd	%xmm15, 0x200(%rbp)
140955c9d: c6 85 08 02 00 00 00        	movb	$0x0, 0x208(%rbp)
140955ca4: 48 8d 85 00 02 00 00        	leaq	0x200(%rbp), %rax
140955cab: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140955cb0: 4c 8d 6d b8                 	leaq	-0x48(%rbp), %r13
140955cb4: 4c 89 6c 24 28              	movq	%r13, 0x28(%rsp)
140955cb9: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
140955cc0: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140955cc5: 48 8d 8d b4 00 00 00        	leaq	0xb4(%rbp), %rcx
140955ccc: 0f 28 cf                    	movaps	%xmm7, %xmm1
140955ccf: 4c 8d 8d e8 01 00 00        	leaq	0x1e8(%rbp), %r9
140955cd6: e8 c5 27 17 00              	callq	0x140ac84a0 <.text+0xac74a0>
140955cdb: 4c 8b 85 48 02 00 00        	movq	0x248(%rbp), %r8
140955ce2: f3 0f 10 85 b8 00 00 00     	movss	0xb8(%rbp), %xmm0
140955cea: f3 0f 5c 46 04              	subss	0x4(%rsi), %xmm0
140955cef: f3 0f 10 4e 0c              	movss	0xc(%rsi), %xmm1
140955cf4: f3 0f 10 56 18              	movss	0x18(%rsi), %xmm2
140955cf9: 0f 28 d9                    	movaps	%xmm1, %xmm3
140955cfc: f3 0f c2 d9 03              	cmpunordss	%xmm1, %xmm3
140955d01: 0f 28 e3                    	movaps	%xmm3, %xmm4
140955d04: 0f 54 e2                    	andps	%xmm2, %xmm4
140955d07: f3 0f 5f d1                 	maxss	%xmm1, %xmm2
140955d0b: 0f 55 da                    	andnps	%xmm2, %xmm3
140955d0e: 0f 56 dc                    	orps	%xmm4, %xmm3
140955d11: f3 0f 58 d8                 	addss	%xmm0, %xmm3
140955d15: 0f 57 c0                    	xorps	%xmm0, %xmm0
140955d18: f3 0f 5f c3                 	maxss	%xmm3, %xmm0
140955d1c: 41 0f 28 fc                 	movaps	%xmm12, %xmm7
140955d20: f3 0f 5d f8                 	minss	%xmm0, %xmm7
140955d24: 41 0f 2e fc                 	ucomiss	%xmm12, %xmm7
140955d28: 0f 82 ad 00 00 00           	jb	0x140955ddb <.text+0x954ddb>
140955d2e: 4c 89 e9                    	movq	%r13, %rcx
140955d31: 48 89 da                    	movq	%rbx, %rdx
140955d34: e8 f7 c7 13 00              	callq	0x140a92530 <.text+0xa91530>
140955d39: 83 7d b8 02                 	cmpl	$0x2, -0x48(%rbp)
140955d3d: 0f 84 a7 07 00 00           	je	0x1409564ea <.text+0x9554ea>
140955d43: 0f 10 45 b8                 	movups	-0x48(%rbp), %xmm0
140955d47: 0f 10 4d c4                 	movups	-0x3c(%rbp), %xmm1
140955d4b: 0f 11 8d 0c 02 00 00        	movups	%xmm1, 0x20c(%rbp)
140955d52: 0f 29 85 00 02 00 00        	movaps	%xmm0, 0x200(%rbp)
140955d59: 8b 85 dc 01 00 00           	movl	0x1dc(%rbp), %eax
140955d5f: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
140955d66: 89 01                       	movl	%eax, (%rcx)
140955d68: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
140955d6f: 48 8b 58 30                 	movq	0x30(%rax), %rbx
140955d73: 4c 8b 78 40                 	movq	0x40(%rax), %r15
140955d77: 48 3b 58 20                 	cmpq	0x20(%rax), %rbx
140955d7b: 75 13                       	jne	0x140955d90 <.text+0x954d90>
140955d7d: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
140955d84: 48 8d 15 bd 43 2e 02        	leaq	0x22e43bd(%rip), %rdx   # 0x142c3a148
140955d8b: e8 20 cd c0 ff              	callq	0x140562ab0 <.text+0x561ab0>
140955d90: 48 8b 95 a8 01 00 00        	movq	0x1a8(%rbp), %rdx
140955d97: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140955d9b: 48 8d 0c 9b                 	leaq	(%rbx,%rbx,4), %rcx
140955d9f: 4c 89 3c c8                 	movq	%r15, (%rax,%rcx,8)
140955da3: 0f 10 85 0c 02 00 00        	movups	0x20c(%rbp), %xmm0
140955daa: 0f 11 44 c8 14              	movups	%xmm0, 0x14(%rax,%rcx,8)
140955daf: 0f 28 85 00 02 00 00        	movaps	0x200(%rbp), %xmm0
140955db6: 0f 11 44 c8 08              	movups	%xmm0, 0x8(%rax,%rcx,8)
140955dbb: 48 ff c3                    	incq	%rbx
140955dbe: 48 89 5a 30                 	movq	%rbx, 0x30(%rdx)
140955dc2: 48 ff 42 40                 	incq	0x40(%rdx)
140955dc6: 48 8b 9d 30 02 00 00        	movq	0x230(%rbp), %rbx
140955dcd: 4c 8b bd 40 02 00 00        	movq	0x240(%rbp), %r15
140955dd4: 4c 8b 85 48 02 00 00        	movq	0x248(%rbp), %r8
140955ddb: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
140955de2: 8b 8d d0 01 00 00           	movl	0x1d0(%rbp), %ecx
140955de8: 89 08                       	movl	%ecx, (%rax)
140955dea: f3 0f 11 7d b8              	movss	%xmm7, -0x48(%rbp)
140955def: 4c 89 f9                    	movq	%r15, %rcx
140955df2: 4c 89 c2                    	movq	%r8, %rdx
140955df5: 4d 89 e8                    	movq	%r13, %r8
140955df8: e8 c3 4c 15 00              	callq	0x140aaaac0 <.text+0xaa9ac0>
140955dfd: 48 8b 85 70 03 00 00        	movq	0x370(%rbp), %rax
140955e04: f3 0f 11 78 10              	movss	%xmm7, 0x10(%rax)
140955e09: 4c 8b 95 38 02 00 00        	movq	0x238(%rbp), %r10
140955e10: 4c 8b ad 98 01 00 00        	movq	0x198(%rbp), %r13
140955e17: 48 8b b5 28 02 00 00        	movq	0x228(%rbp), %rsi
140955e1e: 41 80 7c 24 18 02           	cmpb	$0x2, 0x18(%r12)
140955e24: 0f 82 2a fb ff ff           	jb	0x140955954 <.text+0x954954>
140955e2a: e9 63 02 00 00              	jmp	0x140956092 <.text+0x955092>
140955e2f: 90                          	nop
140955e30: 48 c7 85 e8 01 00 00 00 00 00 00    	movq	$0x0, 0x1e8(%rbp)
140955e3b: 48 c7 85 f0 01 00 00 08 00 00 00    	movq	$0x8, 0x1f0(%rbp)
140955e46: 48 c7 85 f8 01 00 00 00 00 00 00    	movq	$0x0, 0x1f8(%rbp)
140955e51: 48 8b 85 70 03 00 00        	movq	0x370(%rbp), %rax
140955e58: f3 0f 10 78 0c              	movss	0xc(%rax), %xmm7
140955e5d: 48 89 55 b8                 	movq	%rdx, -0x48(%rbp)
140955e61: 49 83 7f 18 00              	cmpq	$0x0, 0x18(%r15)
140955e66: 0f 84 cb 05 00 00           	je	0x140956437 <.text+0x955437>
140955e6c: 4d 89 d5                    	movq	%r10, %r13
140955e6f: c6 85 57 02 00 00 01        	movb	$0x1, 0x257(%rbp)
140955e76: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
140955e7d: 48 8d 55 b8                 	leaq	-0x48(%rbp), %rdx
140955e81: e8 ea ae c5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140955e86: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
140955e8d: 48 8b 0a                    	movq	(%rdx), %rcx
140955e90: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
140955e94: 49 89 d0                    	movq	%rdx, %r8
140955e97: 49 21 c0                    	andq	%rax, %r8
140955e9a: 48 c1 e8 39                 	shrq	$0x39, %rax
140955e9e: 66 0f 6e c0                 	movd	%eax, %xmm0
140955ea2: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140955ea6: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140955eab: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140955eb0: 48 8d 41 88                 	leaq	-0x78(%rcx), %rax
140955eb4: 45 31 c9                    	xorl	%r9d, %r9d
140955eb7: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140955ebd: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140955ec1: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140955ec5: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140955eca: 45 85 d2                    	testl	%r10d, %r10d
140955ecd: 74 31                       	je	0x140955f00 <.text+0x954f00>
140955ecf: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140955ed4: 4d 01 c3                    	addq	%r8, %r11
140955ed7: 49 21 d3                    	andq	%rdx, %r11
140955eda: 49 f7 db                    	negq	%r11
140955edd: 4d 6b db 78                 	imulq	$0x78, %r11, %r11
140955ee1: 48 8b 9d 48 02 00 00        	movq	0x248(%rbp), %rbx
140955ee8: 4a 39 1c 18                 	cmpq	%rbx, (%rax,%r11)
140955eec: 74 35                       	je	0x140955f23 <.text+0x954f23>
140955eee: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140955ef2: 66 45 21 d3                 	andw	%r10w, %r11w
140955ef6: 45 89 da                    	movl	%r11d, %r10d
140955ef9: 75 d4                       	jne	0x140955ecf <.text+0x954ecf>
140955efb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140955f00: 66 41 0f 74 cb              	pcmpeqb	%xmm11, %xmm1
140955f05: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140955f0a: 45 85 d2                    	testl	%r10d, %r10d
140955f0d: 0f 85 24 05 00 00           	jne	0x140956437 <.text+0x955437>
140955f13: 4d 01 c8                    	addq	%r9, %r8
140955f16: 49 83 c0 10                 	addq	$0x10, %r8
140955f1a: 49 83 c1 10                 	addq	$0x10, %r9
140955f1e: 49 21 d0                    	andq	%rdx, %r8
140955f21: eb 94                       	jmp	0x140955eb7 <.text+0x954eb7>
140955f23: 66 42 0f 6e 44 19 e8        	movd	-0x18(%rcx,%r11), %xmm0
140955f2a: 48 89 5d c0                 	movq	%rbx, -0x40(%rbp)
140955f2e: f3 0f 11 7d c8              	movss	%xmm7, -0x38(%rbp)
140955f33: 66 0f 7e 45 cc              	movd	%xmm0, -0x34(%rbp)
140955f38: 48 b8 05 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffb, %rax # imm = 0x8000000000000005
140955f42: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
140955f46: c6 85 57 02 00 00 01        	movb	$0x1, 0x257(%rbp)
140955f4d: 48 8d 8d e8 01 00 00        	leaq	0x1e8(%rbp), %rcx
140955f54: 48 8d 55 b8                 	leaq	-0x48(%rbp), %rdx
140955f58: e8 e3 cb 12 00              	callq	0x140a82b40 <.text+0xa81b40>
140955f5d: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
140955f64: 8b 8d d4 01 00 00           	movl	0x1d4(%rbp), %ecx
140955f6a: 89 08                       	movl	%ecx, (%rax)
140955f6c: f3 0f 6f 85 e8 01 00 00     	movdqu	0x1e8(%rbp), %xmm0
140955f74: 66 0f 7f 85 00 02 00 00     	movdqa	%xmm0, 0x200(%rbp)
140955f7c: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
140955f83: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
140955f8a: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
140955f91: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
140955f9c: 48 c7 85 08 02 00 00 08 00 00 00    	movq	$0x8, 0x208(%rbp)
140955fa7: 48 c7 85 10 02 00 00 00 00 00 00    	movq	$0x0, 0x210(%rbp)
140955fb2: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140955fb6: 48 8d 04 c2                 	leaq	(%rdx,%rax,8), %rax
140955fba: 48 89 55 b8                 	movq	%rdx, -0x48(%rbp)
140955fbe: 48 89 55 c0                 	movq	%rdx, -0x40(%rbp)
140955fc2: 48 89 4d c8                 	movq	%rcx, -0x38(%rbp)
140955fc6: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
140955fca: 48 8d 5d b8                 	leaq	-0x48(%rbp), %rbx
140955fce: 48 89 d9                    	movq	%rbx, %rcx
140955fd1: e8 7a 0f d4 00              	callq	0x141696f50 <.text+0x1695f50>
140955fd6: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140955fda: 48 89 55 c8                 	movq	%rdx, -0x38(%rbp)
140955fde: 48 c7 45 b8 07 00 00 00     	movq	$0x7, -0x48(%rbp)
140955fe6: c6 85 57 02 00 00 00        	movb	$0x0, 0x257(%rbp)
140955fed: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
140955ff4: 48 89 da                    	movq	%rbx, %rdx
140955ff7: 4c 8d 05 82 e7 15 02        	leaq	0x215e782(%rip), %r8    # 0x142ab4780
140955ffe: e8 bd ee 12 00              	callq	0x140a84ec0 <.text+0xa83ec0>
140956003: 8b 85 d8 01 00 00           	movl	0x1d8(%rbp), %eax
140956009: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
140956010: 89 01                       	movl	%eax, (%rcx)
140956012: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
140956019: 48 8b 58 30                 	movq	0x30(%rax), %rbx
14095601d: 4c 8b 78 40                 	movq	0x40(%rax), %r15
140956021: 48 3b 58 20                 	cmpq	0x20(%rax), %rbx
140956025: 75 1a                       	jne	0x140956041 <.text+0x955041>
140956027: c6 85 57 02 00 00 00        	movb	$0x0, 0x257(%rbp)
14095602e: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
140956035: 48 8d 15 0c 41 2e 02        	leaq	0x22e410c(%rip), %rdx   # 0x142c3a148
14095603c: e8 ef c5 c0 ff              	callq	0x140562630 <.text+0x561630>
140956041: 48 8b 95 a0 01 00 00        	movq	0x1a0(%rbp), %rdx
140956048: 48 8b 42 28                 	movq	0x28(%rdx), %rax
14095604c: 48 89 d9                    	movq	%rbx, %rcx
14095604f: 48 c1 e1 04                 	shlq	$0x4, %rcx
140956053: 4c 89 3c 08                 	movq	%r15, (%rax,%rcx)
140956057: 4c 8b 85 48 02 00 00        	movq	0x248(%rbp), %r8
14095605e: 4c 89 44 08 08              	movq	%r8, 0x8(%rax,%rcx)
140956063: 48 ff c3                    	incq	%rbx
140956066: 48 89 5a 30                 	movq	%rbx, 0x30(%rdx)
14095606a: 48 ff 42 40                 	incq	0x40(%rdx)
14095606e: 48 8b 9d 30 02 00 00        	movq	0x230(%rbp), %rbx
140956075: 4c 8b bd 40 02 00 00        	movq	0x240(%rbp), %r15
14095607c: 4d 89 ea                    	movq	%r13, %r10
14095607f: 4c 8b ad 98 01 00 00        	movq	0x198(%rbp), %r13
140956086: 41 80 7c 24 18 02           	cmpb	$0x2, 0x18(%r12)
14095608c: 0f 82 c2 f8 ff ff           	jb	0x140955954 <.text+0x954954>
140956092: 49 39 74 24 38              	cmpq	%rsi, 0x38(%r12)
140956097: 0f 85 b7 f8 ff ff           	jne	0x140955954 <.text+0x954954>
14095609d: 41 80 7c 24 19 02           	cmpb	$0x2, 0x19(%r12)
1409560a3: 0f 84 ab f8 ff ff           	je	0x140955954 <.text+0x954954>
1409560a9: 41 0f 28 f0                 	movaps	%xmm8, %xmm6
1409560ad: 4d 89 e7                    	movq	%r12, %r15
1409560b0: 48 8b 85 70 03 00 00        	movq	0x370(%rbp), %rax
1409560b7: f3 0f 10 78 10              	movss	0x10(%rax), %xmm7
1409560bc: f3 44 0f 10 58 14           	movss	0x14(%rax), %xmm11
1409560c2: 44 0f 2e df                 	ucomiss	%xmm7, %xmm11
1409560c6: 75 06                       	jne	0x1409560ce <.text+0x9550ce>
1409560c8: 0f 8b b4 01 00 00           	jnp	0x140956282 <.text+0x955282>
1409560ce: f3 0f 10 00                 	movss	(%rax), %xmm0
1409560d2: f3 0f 10 48 08              	movss	0x8(%rax), %xmm1
1409560d7: f3 0f 11 45 b8              	movss	%xmm0, -0x48(%rbp)
1409560dc: f3 0f 11 4d bc              	movss	%xmm1, -0x44(%rbp)
1409560e1: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1409560e8: 48 8d 55 b8                 	leaq	-0x48(%rbp), %rdx
1409560ec: 0f 28 d7                    	movaps	%xmm7, %xmm2
1409560ef: e8 ec 6c 34 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409560f4: 44 0f 28 c7                 	movaps	%xmm7, %xmm8
1409560f8: f3 45 0f 5c c3              	subss	%xmm11, %xmm8
1409560fd: 45 0f 54 c1                 	andps	%xmm9, %xmm8
140956101: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140956108: f3 44 0f 5e 40 28           	divss	0x28(%rax), %xmm8
14095610e: 8b 85 e0 01 00 00           	movl	0x1e0(%rbp), %eax
140956114: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14095611b: 89 01                       	movl	%eax, (%rcx)
14095611d: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
140956124: 4c 8b 60 30                 	movq	0x30(%rax), %r12
140956128: 48 8b 58 40                 	movq	0x40(%rax), %rbx
14095612c: 4c 3b 60 20                 	cmpq	0x20(%rax), %r12
140956130: 75 13                       	jne	0x140956145 <.text+0x955145>
140956132: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
140956139: 48 8d 15 08 40 2e 02        	leaq	0x22e4008(%rip), %rdx   # 0x142c3a148
140956140: e8 eb ca c0 ff              	callq	0x140562c30 <.text+0x561c30>
140956145: 4c 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %r8
14095614c: 49 8b 40 28                 	movq	0x28(%r8), %rax
140956150: 4c 89 e1                    	movq	%r12, %rcx
140956153: 48 c1 e1 05                 	shlq	$0x5, %rcx
140956157: 48 89 1c 08                 	movq	%rbx, (%rax,%rcx)
14095615b: 48 8b 95 48 02 00 00        	movq	0x248(%rbp), %rdx
140956162: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
140956167: 48 8b 95 c0 01 00 00        	movq	0x1c0(%rbp), %rdx
14095616e: 48 89 54 08 10              	movq	%rdx, 0x10(%rax,%rcx)
140956173: 8b 95 c8 01 00 00           	movl	0x1c8(%rbp), %edx
140956179: 89 54 08 18                 	movl	%edx, 0x18(%rax,%rcx)
14095617d: 66 c7 44 08 1c 00 00        	movw	$0x0, 0x1c(%rax,%rcx)
140956184: 49 ff c4                    	incq	%r12
140956187: 4d 89 60 30                 	movq	%r12, 0x30(%r8)
14095618b: 49 ff 40 40                 	incq	0x40(%r8)
14095618f: 0f 57 c0                    	xorps	%xmm0, %xmm0
140956192: 66 41 0f 6f ce              	movdqa	%xmm14, %xmm1
140956197: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
14095619b: e8 00 a3 33 00              	callq	0x140c904a0 <.text+0xc8f4a0>
1409561a0: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
1409561a4: 45 0f 2e da                 	ucomiss	%xmm10, %xmm11
1409561a8: 4c 8b 95 38 02 00 00        	movq	0x238(%rbp), %r10
1409561af: 75 0b                       	jne	0x1409561bc <.text+0x9551bc>
1409561b1: 7a 09                       	jp	0x1409561bc <.text+0x9551bc>
1409561b3: 40 b6 06                    	movb	$0x6, %sil
1409561b6: 41 0f 2e fa                 	ucomiss	%xmm10, %xmm7
1409561ba: 77 2e                       	ja	0x1409561ea <.text+0x9551ea>
1409561bc: 45 0f 2e da                 	ucomiss	%xmm10, %xmm11
1409561c0: 76 0b                       	jbe	0x1409561cd <.text+0x9551cd>
1409561c2: 40 b6 07                    	movb	$0x7, %sil
1409561c5: 41 0f 2e fa                 	ucomiss	%xmm10, %xmm7
1409561c9: 75 02                       	jne	0x1409561cd <.text+0x9551cd>
1409561cb: 7b 1d                       	jnp	0x1409561ea <.text+0x9551ea>
1409561cd: 45 0f 2e e3                 	ucomiss	%xmm11, %xmm12
1409561d1: 0f 86 ab 00 00 00           	jbe	0x140956282 <.text+0x955282>
1409561d7: 40 b6 08                    	movb	$0x8, %sil
1409561da: 41 0f 2e fc                 	ucomiss	%xmm12, %xmm7
1409561de: 0f 85 9e 00 00 00           	jne	0x140956282 <.text+0x955282>
1409561e4: 0f 8a 98 00 00 00           	jp	0x140956282 <.text+0x955282>
1409561ea: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
1409561f1: 8b 8d e4 01 00 00           	movl	0x1e4(%rbp), %ecx
1409561f7: 89 08                       	movl	%ecx, (%rax)
1409561f9: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
140956200: 4c 8b 60 30                 	movq	0x30(%rax), %r12
140956204: 48 8b 58 40                 	movq	0x40(%rax), %rbx
140956208: 4c 3b 60 20                 	cmpq	0x20(%rax), %r12
14095620c: 75 1a                       	jne	0x140956228 <.text+0x955228>
14095620e: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
140956215: 48 8d 15 2c 3f 2e 02        	leaq	0x22e3f2c(%rip), %rdx   # 0x142c3a148
14095621c: e8 8f c8 c0 ff              	callq	0x140562ab0 <.text+0x561ab0>
140956221: 4c 8b 95 38 02 00 00        	movq	0x238(%rbp), %r10
140956228: 0f 57 c0                    	xorps	%xmm0, %xmm0
14095622b: f3 41 0f 5f c0              	maxss	%xmm8, %xmm0
140956230: 0f 28 ce                    	movaps	%xmm6, %xmm1
140956233: f3 0f 5d c8                 	minss	%xmm0, %xmm1
140956237: 4c 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %r8
14095623e: 49 8b 40 28                 	movq	0x28(%r8), %rax
140956242: 4b 8d 0c a4                 	leaq	(%r12,%r12,4), %rcx
140956246: 48 89 1c c8                 	movq	%rbx, (%rax,%rcx,8)
14095624a: 48 8b 95 48 02 00 00        	movq	0x248(%rbp), %rdx
140956251: 48 89 54 c8 08              	movq	%rdx, 0x8(%rax,%rcx,8)
140956256: 48 8b 95 c0 01 00 00        	movq	0x1c0(%rbp), %rdx
14095625d: 48 89 54 c8 10              	movq	%rdx, 0x10(%rax,%rcx,8)
140956262: 8b 95 c8 01 00 00           	movl	0x1c8(%rbp), %edx
140956268: 89 54 c8 18                 	movl	%edx, 0x18(%rax,%rcx,8)
14095626c: f3 0f 11 4c c8 1c           	movss	%xmm1, 0x1c(%rax,%rcx,8)
140956272: 40 88 74 c8 20              	movb	%sil, 0x20(%rax,%rcx,8)
140956277: 49 ff c4                    	incq	%r12
14095627a: 4d 89 60 30                 	movq	%r12, 0x30(%r8)
14095627e: 49 ff 40 40                 	incq	0x40(%r8)
140956282: 48 8b 85 70 03 00 00        	movq	0x370(%rbp), %rax
140956289: f3 0f 11 78 14              	movss	%xmm7, 0x14(%rax)
14095628e: 4d 89 fc                    	movq	%r15, %r12
140956291: 48 8b 9d 30 02 00 00        	movq	0x230(%rbp), %rbx
140956298: 4c 8b bd 40 02 00 00        	movq	0x240(%rbp), %r15
14095629f: 44 0f 28 c6                 	movaps	%xmm6, %xmm8
1409562a3: 66 45 0f 76 db              	pcmpeqd	%xmm11, %xmm11
1409562a8: e9 a7 f6 ff ff              	jmp	0x140955954 <.text+0x954954>
1409562ad: 66 42 0f 6e 44 19 e8        	movd	-0x18(%rcx,%r11), %xmm0
1409562b4: 48 8b 85 70 03 00 00        	movq	0x370(%rbp), %rax
1409562bb: 66 0f 7e 40 0c              	movd	%xmm0, 0xc(%rax)
1409562c0: 66 0f 7e 40 10              	movd	%xmm0, 0x10(%rax)
1409562c5: 66 0f 7e 40 14              	movd	%xmm0, 0x14(%rax)
1409562ca: 4c 89 4d b8                 	movq	%r9, -0x48(%rbp)
1409562ce: 48 83 7b 18 00              	cmpq	$0x0, 0x18(%rbx)
1409562d3: 0f 84 7f 01 00 00           	je	0x140956458 <.text+0x955458>
1409562d9: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
1409562e0: 48 8d 55 b8                 	leaq	-0x48(%rbp), %rdx
1409562e4: e8 87 aa c5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
1409562e9: 48 8b 0b                    	movq	(%rbx), %rcx
1409562ec: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
1409562f0: 49 89 d0                    	movq	%rdx, %r8
1409562f3: 49 21 c0                    	andq	%rax, %r8
1409562f6: 48 c1 e8 39                 	shrq	$0x39, %rax
1409562fa: 66 0f 6e c0                 	movd	%eax, %xmm0
1409562fe: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140956302: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140956307: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14095630c: 48 8d 81 58 fd ff ff        	leaq	-0x2a8(%rcx), %rax
140956313: 31 f6                       	xorl	%esi, %esi
140956315: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
14095631b: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14095631f: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140956323: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140956328: 45 85 d2                    	testl	%r10d, %r10d
14095632b: 74 33                       	je	0x140956360 <.text+0x955360>
14095632d: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140956332: 4d 01 c3                    	addq	%r8, %r11
140956335: 49 21 d3                    	andq	%rdx, %r11
140956338: 49 f7 db                    	negq	%r11
14095633b: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
140956342: 4c 8b 8d 48 02 00 00        	movq	0x248(%rbp), %r9
140956349: 4e 39 0c 18                 	cmpq	%r9, (%rax,%r11)
14095634d: 74 34                       	je	0x140956383 <.text+0x955383>
14095634f: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140956353: 66 45 21 d3                 	andw	%r10w, %r11w
140956357: 45 89 da                    	movl	%r11d, %r10d
14095635a: 75 d1                       	jne	0x14095632d <.text+0x95532d>
14095635c: 0f 1f 40 00                 	nopl	(%rax)
140956360: 66 41 0f 74 cb              	pcmpeqb	%xmm11, %xmm1
140956365: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
14095636a: 45 85 d2                    	testl	%r10d, %r10d
14095636d: 0f 85 e5 00 00 00           	jne	0x140956458 <.text+0x955458>
140956373: 49 01 f0                    	addq	%rsi, %r8
140956376: 49 83 c0 10                 	addq	$0x10, %r8
14095637a: 48 83 c6 10                 	addq	$0x10, %rsi
14095637e: 49 21 d0                    	andq	%rdx, %r8
140956381: eb 92                       	jmp	0x140956315 <.text+0x955315>
140956383: 4a 83 bc 19 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rcx,%r11)
14095638c: 0f 82 34 01 00 00           	jb	0x1409564c6 <.text+0x9554c6>
140956392: f3 42 0f 10 84 19 90 fd ff ff       	movss	-0x270(%rcx,%r11), %xmm0
14095639c: 41 0f 2e c2                 	ucomiss	%xmm10, %xmm0
1409563a0: 0f 86 20 01 00 00           	jbe	0x1409564c6 <.text+0x9554c6>
1409563a6: 4c 01 d9                    	addq	%r11, %rcx
1409563a9: 48 8b 91 a8 fd ff ff        	movq	-0x258(%rcx), %rdx
1409563b0: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
1409563b4: 48 85 d2                    	testq	%rdx, %rdx
1409563b7: 74 4e                       	je	0x140956407 <.text+0x955407>
1409563b9: 48 8b 81 a0 fd ff ff        	movq	-0x260(%rcx), %rax
1409563c0: 48 c1 e2 02                 	shlq	$0x2, %rdx
1409563c4: 48 8d 0c 52                 	leaq	(%rdx,%rdx,2), %rcx
1409563c8: 31 d2                       	xorl	%edx, %edx
1409563ca: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
1409563ce: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
1409563d2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409563e0: f3 0f 10 54 10 04           	movss	0x4(%rax,%rdx), %xmm2
1409563e6: f3 0f c2 c0 03              	cmpunordss	%xmm0, %xmm0
1409563eb: 0f 28 d8                    	movaps	%xmm0, %xmm3
1409563ee: 0f 54 da                    	andps	%xmm2, %xmm3
1409563f1: f3 0f 5d d1                 	minss	%xmm1, %xmm2
1409563f5: 0f 55 c2                    	andnps	%xmm2, %xmm0
1409563f8: 0f 56 c3                    	orps	%xmm3, %xmm0
1409563fb: 48 83 c2 0c                 	addq	$0xc, %rdx
1409563ff: 0f 28 c8                    	movaps	%xmm0, %xmm1
140956402: 48 39 d1                    	cmpq	%rdx, %rcx
140956405: 75 d9                       	jne	0x1409563e0 <.text+0x9553e0>
140956407: f3 41 0f 5f c2              	maxss	%xmm10, %xmm0
14095640c: 48 8b 85 70 03 00 00        	movq	0x370(%rbp), %rax
140956413: f3 0f 11 40 18              	movss	%xmm0, 0x18(%rax)
140956418: 48 8b b5 28 02 00 00        	movq	0x228(%rbp), %rsi
14095641f: 4c 8b 95 38 02 00 00        	movq	0x238(%rbp), %r10
140956426: 41 80 7c 24 18 02           	cmpb	$0x2, 0x18(%r12)
14095642c: 0f 82 22 f5 ff ff           	jb	0x140955954 <.text+0x954954>
140956432: e9 5b fc ff ff              	jmp	0x140956092 <.text+0x955092>
140956437: c6 85 57 02 00 00 01        	movb	$0x1, 0x257(%rbp)
14095643e: 48 8d 0d 23 e3 15 02        	leaq	0x215e323(%rip), %rcx   # 0x142ab4768
140956445: e8 c6 2e f8 01              	callq	0x1428d9310 <.text+0x28d8310>
14095644a: 0f 0b                       	ud2
14095644c: 48 8d 0d b5 e2 15 02        	leaq	0x215e2b5(%rip), %rcx   # 0x142ab4708
140956453: e8 b8 2e f8 01              	callq	0x1428d9310 <.text+0x28d8310>
140956458: 48 8d 0d d9 e2 15 02        	leaq	0x215e2d9(%rip), %rcx   # 0x142ab4738
14095645f: e8 ac 2e f8 01              	callq	0x1428d9310 <.text+0x28d8310>
140956464: 0f 28 b5 60 02 00 00        	movaps	0x260(%rbp), %xmm6
14095646b: 0f 28 bd 70 02 00 00        	movaps	0x270(%rbp), %xmm7
140956472: 44 0f 28 85 80 02 00 00     	movaps	0x280(%rbp), %xmm8
14095647a: 44 0f 28 8d 90 02 00 00     	movaps	0x290(%rbp), %xmm9
140956482: 44 0f 28 95 a0 02 00 00     	movaps	0x2a0(%rbp), %xmm10
14095648a: 44 0f 28 9d b0 02 00 00     	movaps	0x2b0(%rbp), %xmm11
140956492: 44 0f 28 a5 c0 02 00 00     	movaps	0x2c0(%rbp), %xmm12
14095649a: 44 0f 28 ad d0 02 00 00     	movaps	0x2d0(%rbp), %xmm13
1409564a2: 44 0f 28 b5 e0 02 00 00     	movaps	0x2e0(%rbp), %xmm14
1409564aa: 44 0f 28 bd f0 02 00 00     	movaps	0x2f0(%rbp), %xmm15
1409564b2: 48 81 c4 88 03 00 00        	addq	$0x388, %rsp            # imm = 0x388
1409564b9: 5b                          	popq	%rbx
1409564ba: 5f                          	popq	%rdi
1409564bb: 5e                          	popq	%rsi
1409564bc: 41 5c                       	popq	%r12
1409564be: 41 5d                       	popq	%r13
1409564c0: 41 5e                       	popq	%r14
1409564c2: 41 5f                       	popq	%r15
1409564c4: 5d                          	popq	%rbp
1409564c5: c3                          	retq
1409564c6: 48 8d 0d c6 3c 18 02        	leaq	0x2183cc6(%rip), %rcx   # 0x142ada193
1409564cd: 4c 8d 05 4c e2 15 02        	leaq	0x215e24c(%rip), %r8    # 0x142ab4720
1409564d4: ba 32 00 00 00              	movl	$0x32, %edx
1409564d9: e8 52 2f f8 01              	callq	0x1428d9430 <.text+0x28d8430>
1409564de: 48 8d 0d 0b e2 15 02        	leaq	0x215e20b(%rip), %rcx   # 0x142ab46f0
1409564e5: e8 26 2e f8 01              	callq	0x1428d9310 <.text+0x28d8310>
1409564ea: 48 8d 0d 5f e2 15 02        	leaq	0x215e25f(%rip), %rcx   # 0x142ab4750
1409564f1: e8 1a 2e f8 01              	callq	0x1428d9310 <.text+0x28d8310>
1409564f6: cc                          	int3
1409564f7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140956500: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140956505: 55                          	pushq	%rbp
140956506: 41 57                       	pushq	%r15
140956508: 41 56                       	pushq	%r14
14095650a: 41 55                       	pushq	%r13
14095650c: 41 54                       	pushq	%r12
14095650e: 56                          	pushq	%rsi
14095650f: 57                          	pushq	%rdi
140956510: 53                          	pushq	%rbx
140956511: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140956518: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14095651f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140956525: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14095652c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140956532: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140956538: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
14095653f: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140956548: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140956551: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
14095655a: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140956562: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
14095656a: 80 bd 57 02 00 00 00        	cmpb	$0x0, 0x257(%rbp)
140956571: 75 5d                       	jne	0x1409565d0 <.text+0x9555d0>
140956573: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
14095657b: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140956583: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
14095658c: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140956595: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
14095659e: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409565a4: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409565aa: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409565b0: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409565b6: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409565bc: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409565c3: 5b                          	popq	%rbx
1409565c4: 5f                          	popq	%rdi
1409565c5: 5e                          	popq	%rsi
1409565c6: 41 5c                       	popq	%r12
1409565c8: 41 5d                       	popq	%r13
1409565ca: 41 5e                       	popq	%r14
1409565cc: 41 5f                       	popq	%r15
1409565ce: 5d                          	popq	%rbp
1409565cf: c3                          	retq
1409565d0: 48 8d 8d e8 01 00 00        	leaq	0x1e8(%rbp), %rcx
1409565d7: e8 94 1e f4 ff              	callq	0x140898470 <.text+0x897470>
1409565dc: eb 95                       	jmp	0x140956573 <.text+0x955573>
1409565de: 66 90                       	nop
1409565e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409565e5: 55                          	pushq	%rbp
1409565e6: 41 57                       	pushq	%r15
1409565e8: 41 56                       	pushq	%r14
1409565ea: 41 55                       	pushq	%r13
1409565ec: 41 54                       	pushq	%r12
1409565ee: 56                          	pushq	%rsi
1409565ef: 57                          	pushq	%rdi
1409565f0: 53                          	pushq	%rbx
1409565f1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409565f8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409565ff: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140956605: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14095660c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140956612: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140956618: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
14095661f: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140956628: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140956631: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
14095663a: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140956642: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
14095664a: c6 85 57 02 00 00 00        	movb	$0x0, 0x257(%rbp)
140956651: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
140956658: e8 13 1e f4 ff              	callq	0x140898470 <.text+0x897470>
14095665d: c6 85 57 02 00 00 00        	movb	$0x0, 0x257(%rbp)
140956664: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
14095666c: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140956674: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
14095667d: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140956686: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
14095668f: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
140956696: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
14095669c: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409566a2: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
1409566a9: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409566af: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409566b6: 5b                          	popq	%rbx
1409566b7: 5f                          	popq	%rdi
1409566b8: 5e                          	popq	%rsi
1409566b9: 41 5c                       	popq	%r12
1409566bb: 41 5d                       	popq	%r13
1409566bd: 41 5e                       	popq	%r14
1409566bf: 41 5f                       	popq	%r15
1409566c1: 5d                          	popq	%rbp
1409566c2: c3                          	retq
