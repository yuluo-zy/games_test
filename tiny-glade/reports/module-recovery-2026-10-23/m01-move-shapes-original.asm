
B版本内容/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14097a060: 55                          	pushq	%rbp
14097a061: 41 57                       	pushq	%r15
14097a063: 41 56                       	pushq	%r14
14097a065: 41 55                       	pushq	%r13
14097a067: 41 54                       	pushq	%r12
14097a069: 56                          	pushq	%rsi
14097a06a: 57                          	pushq	%rdi
14097a06b: 53                          	pushq	%rbx
14097a06c: 48 81 ec 98 04 00 00        	subq	$0x498, %rsp            # imm = 0x498
14097a073: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14097a07b: 44 0f 29 bd 00 04 00 00     	movaps	%xmm15, 0x400(%rbp)
14097a083: 44 0f 29 b5 f0 03 00 00     	movaps	%xmm14, 0x3f0(%rbp)
14097a08b: 44 0f 29 ad e0 03 00 00     	movaps	%xmm13, 0x3e0(%rbp)
14097a093: 44 0f 29 a5 d0 03 00 00     	movaps	%xmm12, 0x3d0(%rbp)
14097a09b: 44 0f 29 9d c0 03 00 00     	movaps	%xmm11, 0x3c0(%rbp)
14097a0a3: 44 0f 29 95 b0 03 00 00     	movaps	%xmm10, 0x3b0(%rbp)
14097a0ab: 66 44 0f 7f 8d a0 03 00 00  	movdqa	%xmm9, 0x3a0(%rbp)
14097a0b4: 66 44 0f 7f 85 90 03 00 00  	movdqa	%xmm8, 0x390(%rbp)
14097a0bd: 66 0f 7f bd 80 03 00 00     	movdqa	%xmm7, 0x380(%rbp)
14097a0c5: 0f 29 b5 70 03 00 00        	movaps	%xmm6, 0x370(%rbp)
14097a0cc: 48 c7 85 68 03 00 00 fe ff ff ff    	movq	$-0x2, 0x368(%rbp)
14097a0d7: 4c 8b 22                    	movq	(%rdx), %r12
14097a0da: 4c 8b 72 20                 	movq	0x20(%rdx), %r14
14097a0de: 4c 8b 7a 30                 	movq	0x30(%rdx), %r15
14097a0e2: 44 8b 6a 3c                 	movl	0x3c(%rdx), %r13d
14097a0e6: 48 8b 42 40                 	movq	0x40(%rdx), %rax
14097a0ea: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
14097a0f1: 48 8b 42 48                 	movq	0x48(%rdx), %rax
14097a0f5: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
14097a0fc: 49 8b 00                    	movq	(%r8), %rax
14097a0ff: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
14097a106: 4d 8b 50 20                 	movq	0x20(%r8), %r10
14097a10a: 4d 8b 58 40                 	movq	0x40(%r8), %r11
14097a10e: 49 8b 40 48                 	movq	0x48(%r8), %rax
14097a112: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14097a116: 49 8b 40 68                 	movq	0x68(%r8), %rax
14097a11a: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14097a11e: 49 8b 01                    	movq	(%r9), %rax
14097a121: 48 89 85 d8 02 00 00        	movq	%rax, 0x2d8(%rbp)
14097a128: 49 8b 41 10                 	movq	0x10(%r9), %rax
14097a12c: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
14097a133: 41 8b 41 1c                 	movl	0x1c(%r9), %eax
14097a137: 89 85 70 02 00 00           	movl	%eax, 0x270(%rbp)
14097a13d: 49 8b 41 20                 	movq	0x20(%r9), %rax
14097a141: 48 89 85 b0 02 00 00        	movq	%rax, 0x2b0(%rbp)
14097a148: 49 8b 41 30                 	movq	0x30(%r9), %rax
14097a14c: 48 89 85 88 02 00 00        	movq	%rax, 0x288(%rbp)
14097a153: 41 8b 41 3c                 	movl	0x3c(%r9), %eax
14097a157: 89 85 b8 02 00 00           	movl	%eax, 0x2b8(%rbp)
14097a15d: 48 8b 85 80 04 00 00        	movq	0x480(%rbp), %rax
14097a164: 48 8b 18                    	movq	(%rax), %rbx
14097a167: 48 8b 40 20                 	movq	0x20(%rax), %rax
14097a16b: 48 89 85 58 03 00 00        	movq	%rax, 0x358(%rbp)
14097a172: 48 8b 95 a0 04 00 00        	movq	0x4a0(%rbp), %rdx
14097a179: 0f 10 42 40                 	movups	0x40(%rdx), %xmm0
14097a17d: f3 0f 6f 4a 50              	movdqu	0x50(%rdx), %xmm1
14097a182: 66 0f 7f 4d 20              	movdqa	%xmm1, 0x20(%rbp)
14097a187: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
14097a18b: 48 8b 01                    	movq	(%rcx), %rax
14097a18e: 4c 8b 42 20                 	movq	0x20(%rdx), %r8
14097a192: 4c 8b 4a 30                 	movq	0x30(%rdx), %r9
14097a196: 8b 4a 3c                    	movl	0x3c(%rdx), %ecx
14097a199: 89 8d 48 01 00 00           	movl	%ecx, 0x148(%rbp)
14097a19f: 8b 08                       	movl	(%rax), %ecx
14097a1a1: 83 f9 05                    	cmpl	$0x5, %ecx
14097a1a4: 0f 87 74 21 00 00           	ja	0x14097c31e <.text+0x97b31e>
14097a1aa: 48 8b bd a8 04 00 00        	movq	0x4a8(%rbp), %rdi
14097a1b1: 48 8b 95 c0 04 00 00        	movq	0x4c0(%rbp), %rdx
14097a1b8: 48 8b 77 20                 	movq	0x20(%rdi), %rsi
14097a1bc: 48 89 75 48                 	movq	%rsi, 0x48(%rbp)
14097a1c0: 48 8b 77 30                 	movq	0x30(%rdi), %rsi
14097a1c4: 48 89 75 50                 	movq	%rsi, 0x50(%rbp)
14097a1c8: 8b 77 3c                    	movl	0x3c(%rdi), %esi
14097a1cb: 89 b5 e0 00 00 00           	movl	%esi, 0xe0(%rbp)
14097a1d1: 48 8b 77 40                 	movq	0x40(%rdi), %rsi
14097a1d5: 48 89 b5 b0 00 00 00        	movq	%rsi, 0xb0(%rbp)
14097a1dc: 48 8b 32                    	movq	(%rdx), %rsi
14097a1df: 48 89 b5 30 01 00 00        	movq	%rsi, 0x130(%rbp)
14097a1e6: 48 8b 72 20                 	movq	0x20(%rdx), %rsi
14097a1ea: 48 89 b5 f8 00 00 00        	movq	%rsi, 0xf8(%rbp)
14097a1f1: 48 8b 72 30                 	movq	0x30(%rdx), %rsi
14097a1f5: 48 89 75 60                 	movq	%rsi, 0x60(%rbp)
14097a1f9: 8b 72 3c                    	movl	0x3c(%rdx), %esi
14097a1fc: 89 b5 e8 00 00 00           	movl	%esi, 0xe8(%rbp)
14097a202: 48 8b 72 40                 	movq	0x40(%rdx), %rsi
14097a206: 48 89 b5 f0 00 00 00        	movq	%rsi, 0xf0(%rbp)
14097a20d: 48 8b 72 50                 	movq	0x50(%rdx), %rsi
14097a211: 48 89 75 58                 	movq	%rsi, 0x58(%rbp)
14097a215: 8b 72 5c                    	movl	0x5c(%rdx), %esi
14097a218: 89 b5 e4 00 00 00           	movl	%esi, 0xe4(%rbp)
14097a21e: 48 8b 52 60                 	movq	0x60(%rdx), %rdx
14097a222: 48 8b 77 50                 	movq	0x50(%rdi), %rsi
14097a226: 48 89 b5 b8 00 00 00        	movq	%rsi, 0xb8(%rbp)
14097a22d: 8b 77 5c                    	movl	0x5c(%rdi), %esi
14097a230: 89 b5 4c 01 00 00           	movl	%esi, 0x14c(%rbp)
14097a236: 48 8b 77 60                 	movq	0x60(%rdi), %rsi
14097a23a: 48 89 b5 40 03 00 00        	movq	%rsi, 0x340(%rbp)
14097a241: 48 8b 77 70                 	movq	0x70(%rdi), %rsi
14097a245: 8b 7f 7c                    	movl	0x7c(%rdi), %edi
14097a248: 83 f9 02                    	cmpl	$0x2, %ecx
14097a24b: 74 09                       	je	0x14097a256 <.text+0x979256>
14097a24d: 83 f9 01                    	cmpl	$0x1, %ecx
14097a250: 0f 85 c8 20 00 00           	jne	0x14097c31e <.text+0x97b31e>
14097a256: 48 89 95 38 01 00 00        	movq	%rdx, 0x138(%rbp)
14097a25d: 44 89 ad 7c 02 00 00        	movl	%r13d, 0x27c(%rbp)
14097a264: 4c 89 b5 58 01 00 00        	movq	%r14, 0x158(%rbp)
14097a26b: 4c 89 8d 08 01 00 00        	movq	%r9, 0x108(%rbp)
14097a272: 4c 89 95 60 01 00 00        	movq	%r10, 0x160(%rbp)
14097a279: 4c 89 85 c0 00 00 00        	movq	%r8, 0xc0(%rbp)
14097a280: 4c 8b 68 08                 	movq	0x8(%rax), %r13
14097a284: 4c 89 ad 50 03 00 00        	movq	%r13, 0x350(%rbp)
14097a28b: 4c 89 ad 70 01 00 00        	movq	%r13, 0x170(%rbp)
14097a292: 49 83 7c 24 18 00           	cmpq	$0x0, 0x18(%r12)
14097a298: 0f 84 cc 03 00 00           	je	0x14097a66a <.text+0x97966a>
14097a29e: 4d 8d 74 24 20              	leaq	0x20(%r12), %r14
14097a2a3: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097a2aa: 4c 89 f1                    	movq	%r14, %rcx
14097a2ad: 4c 89 9d 00 01 00 00        	movq	%r11, 0x100(%rbp)
14097a2b4: e8 b7 6a c3 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14097a2b9: 49 8b 0c 24                 	movq	(%r12), %rcx
14097a2bd: 49 8b 54 24 08              	movq	0x8(%r12), %rdx
14097a2c2: 49 89 d0                    	movq	%rdx, %r8
14097a2c5: 49 21 c0                    	andq	%rax, %r8
14097a2c8: 48 c1 e8 39                 	shrq	$0x39, %rax
14097a2cc: 66 0f 6e c0                 	movd	%eax, %xmm0
14097a2d0: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14097a2d4: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14097a2d9: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14097a2de: 48 8d 81 58 fd ff ff        	leaq	-0x2a8(%rcx), %rax
14097a2e5: 45 31 c9                    	xorl	%r9d, %r9d
14097a2e8: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14097a2ec: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
14097a2f2: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14097a2f6: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14097a2fa: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
14097a2ff: 45 85 d2                    	testl	%r10d, %r10d
14097a302: 74 28                       	je	0x14097a32c <.text+0x97932c>
14097a304: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14097a309: 4d 01 c3                    	addq	%r8, %r11
14097a30c: 49 21 d3                    	andq	%rdx, %r11
14097a30f: 49 f7 db                    	negq	%r11
14097a312: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
14097a319: 4e 39 2c 18                 	cmpq	%r13, (%rax,%r11)
14097a31d: 74 2f                       	je	0x14097a34e <.text+0x97934e>
14097a31f: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
14097a323: 66 45 21 d3                 	andw	%r10w, %r11w
14097a327: 45 89 da                    	movl	%r11d, %r10d
14097a32a: 75 d8                       	jne	0x14097a304 <.text+0x979304>
14097a32c: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14097a330: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14097a335: 45 85 d2                    	testl	%r10d, %r10d
14097a338: 0f 85 2c 03 00 00           	jne	0x14097a66a <.text+0x97966a>
14097a33e: 4d 01 c8                    	addq	%r9, %r8
14097a341: 49 83 c0 10                 	addq	$0x10, %r8
14097a345: 49 83 c1 10                 	addq	$0x10, %r9
14097a349: 49 21 d0                    	andq	%rdx, %r8
14097a34c: eb 9e                       	jmp	0x14097a2ec <.text+0x9792ec>
14097a34e: 89 bd ec 00 00 00           	movl	%edi, 0xec(%rbp)
14097a354: 4a 83 bc 19 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rcx,%r11)
14097a35d: 0f 82 1d 20 00 00           	jb	0x14097c380 <.text+0x97b380>
14097a363: f3 42 0f 10 84 19 90 fd ff ff       	movss	-0x270(%rcx,%r11), %xmm0
14097a36d: 66 0f ef c9                 	pxor	%xmm1, %xmm1
14097a371: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
14097a374: 0f 86 06 20 00 00           	jbe	0x14097c380 <.text+0x97b380>
14097a37a: 4c 89 ad 70 01 00 00        	movq	%r13, 0x170(%rbp)
14097a381: 49 83 7c 24 18 00           	cmpq	$0x0, 0x18(%r12)
14097a387: 0f 84 e9 02 00 00           	je	0x14097a676 <.text+0x979676>
14097a38d: 4a 8d 3c 19                 	leaq	(%rcx,%r11), %rdi
14097a391: 4a 8d 04 19                 	leaq	(%rcx,%r11), %rax
14097a395: 48 05 60 fd ff ff           	addq	$-0x2a0, %rax           # imm = 0xFD60
14097a39b: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
14097a3a2: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097a3a9: 4c 89 f1                    	movq	%r14, %rcx
14097a3ac: e8 bf 69 c3 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14097a3b1: 49 8b 0c 24                 	movq	(%r12), %rcx
14097a3b5: 49 8b 54 24 08              	movq	0x8(%r12), %rdx
14097a3ba: 49 89 d0                    	movq	%rdx, %r8
14097a3bd: 49 21 c0                    	andq	%rax, %r8
14097a3c0: 48 c1 e8 39                 	shrq	$0x39, %rax
14097a3c4: 66 0f 6e c0                 	movd	%eax, %xmm0
14097a3c8: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14097a3cc: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14097a3d1: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14097a3d6: 48 8d 81 58 fd ff ff        	leaq	-0x2a8(%rcx), %rax
14097a3dd: 45 31 c9                    	xorl	%r9d, %r9d
14097a3e0: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14097a3e4: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
14097a3ea: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14097a3ee: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14097a3f2: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
14097a3f7: 45 85 d2                    	testl	%r10d, %r10d
14097a3fa: 74 28                       	je	0x14097a424 <.text+0x979424>
14097a3fc: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14097a401: 4d 01 c3                    	addq	%r8, %r11
14097a404: 49 21 d3                    	andq	%rdx, %r11
14097a407: 49 f7 db                    	negq	%r11
14097a40a: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
14097a411: 4e 39 2c 18                 	cmpq	%r13, (%rax,%r11)
14097a415: 74 2f                       	je	0x14097a446 <.text+0x979446>
14097a417: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
14097a41b: 66 45 21 d3                 	andw	%r10w, %r11w
14097a41f: 45 89 da                    	movl	%r11d, %r10d
14097a422: 75 d8                       	jne	0x14097a3fc <.text+0x9793fc>
14097a424: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14097a428: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14097a42d: 45 85 d2                    	testl	%r10d, %r10d
14097a430: 0f 85 40 02 00 00           	jne	0x14097a676 <.text+0x979676>
14097a436: 4d 01 c8                    	addq	%r9, %r8
14097a439: 49 83 c0 10                 	addq	$0x10, %r8
14097a43d: 49 83 c1 10                 	addq	$0x10, %r9
14097a441: 49 21 d0                    	andq	%rdx, %r8
14097a444: eb 9e                       	jmp	0x14097a3e4 <.text+0x9793e4>
14097a446: 4a 83 bc 19 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rcx,%r11)
14097a44f: 0f 82 43 1f 00 00           	jb	0x14097c398 <.text+0x97b398>
14097a455: f3 42 0f 10 84 19 90 fd ff ff       	movss	-0x270(%rcx,%r11), %xmm0
14097a45f: 66 0f ef c9                 	pxor	%xmm1, %xmm1
14097a463: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
14097a466: 0f 86 2c 1f 00 00           	jbe	0x14097c398 <.text+0x97b398>
14097a46c: 4c 01 d9                    	addq	%r11, %rcx
14097a46f: 8b 41 b8                    	movl	-0x48(%rcx), %eax
14097a472: 89 85 60 03 00 00           	movl	%eax, 0x360(%rbp)
14097a478: 83 f8 01                    	cmpl	$0x1, %eax
14097a47b: 0f 87 9d 1e 00 00           	ja	0x14097c31e <.text+0x97b31e>
14097a481: 4c 89 bd c8 00 00 00        	movq	%r15, 0xc8(%rbp)
14097a488: 44 8b 79 bc                 	movl	-0x44(%rcx), %r15d
14097a48c: 49 89 fe                    	movq	%rdi, %r14
14097a48f: 8b 79 c0                    	movl	-0x40(%rcx), %edi
14097a492: 44 8b 41 c4                 	movl	-0x3c(%rcx), %r8d
14097a496: 8b 51 c8                    	movl	-0x38(%rcx), %edx
14097a499: 48 8b 41 cc                 	movq	-0x34(%rcx), %rax
14097a49d: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
14097a4a4: 41 80 7e f8 00              	cmpb	$0x0, -0x8(%r14)
14097a4a9: 44 89 85 38 03 00 00        	movl	%r8d, 0x338(%rbp)
14097a4b0: 89 95 34 03 00 00           	movl	%edx, 0x334(%rbp)
14097a4b6: 4c 89 b5 28 03 00 00        	movq	%r14, 0x328(%rbp)
14097a4bd: 0f 84 3d 04 00 00           	je	0x14097a900 <.text+0x979900>
14097a4c3: f6 85 60 03 00 00 01        	testb	$0x1, 0x360(%rbp)
14097a4ca: 44 89 f8                    	movl	%r15d, %eax
14097a4cd: 41 0f 45 c0                 	cmovnel	%r8d, %eax
14097a4d1: 66 0f 6e c8                 	movd	%eax, %xmm1
14097a4d5: 89 f9                       	movl	%edi, %ecx
14097a4d7: 0f 45 ca                    	cmovnel	%edx, %ecx
14097a4da: 66 0f 6e d1                 	movd	%ecx, %xmm2
14097a4de: 89 85 90 00 00 00           	movl	%eax, 0x90(%rbp)
14097a4e4: 89 8d 94 00 00 00           	movl	%ecx, 0x94(%rbp)
14097a4ea: 48 8b 85 b8 04 00 00        	movq	0x4b8(%rbp), %rax
14097a4f1: 48 8b 08                    	movq	(%rax), %rcx
14097a4f4: e8 47 f1 06 00              	callq	0x1409e9640 <.text+0x9e8640>
14097a4f9: f3 41 0f 10 56 e0           	movss	-0x20(%r14), %xmm2
14097a4ff: 0f 28 ca                    	movaps	%xmm2, %xmm1
14097a502: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
14097a506: f3 0f c2 c0 03              	cmpunordss	%xmm0, %xmm0
14097a50b: 0f 54 d0                    	andps	%xmm0, %xmm2
14097a50e: 0f 55 c1                    	andnps	%xmm1, %xmm0
14097a511: 0f 56 d0                    	orps	%xmm0, %xmm2
14097a514: f3 0f 58 15 80 a1 fb 01     	addss	0x1fba180(%rip), %xmm2  # 0x14293469c
14097a51c: 48 8d 8d 14 01 00 00        	leaq	0x114(%rbp), %rcx
14097a523: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
14097a52a: e8 b1 28 32 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14097a52f: 48 8d 85 50 03 00 00        	leaq	0x350(%rbp), %rax
14097a536: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
14097a53d: 48 8d 05 1c 71 00 00        	leaq	0x711c(%rip), %rax      # 0x140981660 <.text+0x980660>
14097a544: 48 89 85 98 02 00 00        	movq	%rax, 0x298(%rbp)
14097a54b: 48 8d 05 16 c7 13 02        	leaq	0x213c716(%rip), %rax   # 0x142ab6c68
14097a552: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
14097a559: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
14097a564: 48 8d 85 90 02 00 00        	leaq	0x290(%rbp), %rax
14097a56b: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
14097a572: 48 c7 85 88 01 00 00 01 00 00 00    	movq	$0x1, 0x188(%rbp)
14097a57d: 48 c7 85 90 01 00 00 00 00 00 00    	movq	$0x0, 0x190(%rbp)
14097a588: 4c 8d b5 e0 02 00 00        	leaq	0x2e0(%rbp), %r14
14097a58f: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097a596: 4c 89 f1                    	movq	%r14, %rcx
14097a599: e8 f2 f9 3f 00              	callq	0x140d79f90 <.text+0xd78f90>
14097a59e: 8b 85 1c 01 00 00           	movl	0x11c(%rbp), %eax
14097a5a4: 89 85 08 02 00 00           	movl	%eax, 0x208(%rbp)
14097a5aa: 48 8b 85 14 01 00 00        	movq	0x114(%rbp), %rax
14097a5b1: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
14097a5b8: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
14097a5c3: 48 c7 85 78 01 00 00 08 00 00 00    	movq	$0x8, 0x178(%rbp)
14097a5ce: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
14097a5d9: c7 85 88 01 00 00 01 00 00 00       	movl	$0x1, 0x188(%rbp)
14097a5e3: 48 c7 85 90 01 00 00 00 00 00 00    	movq	$0x0, 0x190(%rbp)
14097a5ee: c7 85 a0 01 00 00 00 00 00 00       	movl	$0x0, 0x1a0(%rbp)
14097a5f8: c7 85 b8 01 00 00 00 00 00 00       	movl	$0x0, 0x1b8(%rbp)
14097a602: c7 85 d0 01 00 00 00 00 00 00       	movl	$0x0, 0x1d0(%rbp)
14097a60c: c6 85 e8 01 00 00 00        	movb	$0x0, 0x1e8(%rbp)
14097a613: c6 85 f8 01 00 00 00        	movb	$0x0, 0x1f8(%rbp)
14097a61a: c7 85 fc 01 00 00 cd cc cc 3e       	movl	$0x3ecccccd, 0x1fc(%rbp) # imm = 0x3ECCCCCD
14097a624: 66 c7 85 18 02 00 00 00 01  	movw	$0x100, 0x218(%rbp)     # imm = 0x100
14097a62d: 4c 8d 85 70 01 00 00        	leaq	0x170(%rbp), %r8
14097a634: 48 89 d9                    	movq	%rbx, %rcx
14097a637: 4c 89 f2                    	movq	%r14, %rdx
14097a63a: e8 b1 8b fa ff              	callq	0x1409231f0 <.text+0x9221f0>
14097a63f: 49 89 c6                    	movq	%rax, %r14
14097a642: f3 0f 10 05 8e b2 fa 01     	movss	0x1fab28e(%rip), %xmm0  # 0x1429258d8
14097a64a: 80 78 18 00                 	cmpb	$0x0, 0x18(%rax)
14097a64e: 74 32                       	je	0x14097a682 <.text+0x979682>
14097a650: 49 39 56 38                 	cmpq	%rdx, 0x38(%r14)
14097a654: 75 2c                       	jne	0x14097a682 <.text+0x979682>
14097a656: 49 89 d5                    	movq	%rdx, %r13
14097a659: 41 83 7e 28 02              	cmpl	$0x2, 0x28(%r14)
14097a65e: 74 25                       	je	0x14097a685 <.text+0x979685>
14097a660: f3 0f 10 05 08 b3 fa 01     	movss	0x1fab308(%rip), %xmm0  # 0x142925970
14097a668: eb 1b                       	jmp	0x14097a685 <.text+0x979685>
14097a66a: 48 8d 0d 9f c5 13 02        	leaq	0x213c59f(%rip), %rcx   # 0x142ab6c10
14097a671: e8 9a ec f5 01              	callq	0x1428d9310 <.text+0x28d8310>
14097a676: 48 8d 0d c3 c5 13 02        	leaq	0x213c5c3(%rip), %rcx   # 0x142ab6c40
14097a67d: e8 8e ec f5 01              	callq	0x1428d9310 <.text+0x28d8310>
14097a682: 49 89 d5                    	movq	%rdx, %r13
14097a685: c7 85 70 01 00 00 00 00 00 00       	movl	$0x0, 0x170(%rbp)
14097a68f: f3 0f 11 85 80 01 00 00     	movss	%xmm0, 0x180(%rbp)
14097a697: 48 8b 85 14 01 00 00        	movq	0x114(%rbp), %rax
14097a69e: 48 89 85 84 01 00 00        	movq	%rax, 0x184(%rbp)
14097a6a5: 8b 85 1c 01 00 00           	movl	0x11c(%rbp), %eax
14097a6ab: 89 85 8c 01 00 00           	movl	%eax, 0x18c(%rbp)
14097a6b1: c7 85 90 01 00 00 00 00 80 3f       	movl	$0x3f800000, 0x190(%rbp) # imm = 0x3F800000
14097a6bb: c6 85 94 01 00 00 01        	movb	$0x1, 0x194(%rbp)
14097a6c2: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14097a6c9: 48 8b 95 58 03 00 00        	movq	0x358(%rbp), %rdx
14097a6d0: e8 4b 7e 18 00              	callq	0x140b02520 <.text+0xb01520>
14097a6d5: 41 80 7e 18 02              	cmpb	$0x2, 0x18(%r14)
14097a6da: 0f 82 20 02 00 00           	jb	0x14097a900 <.text+0x979900>
14097a6e0: 41 0f b6 46 19              	movzbl	0x19(%r14), %eax
14097a6e5: 3c 01                       	cmpb	$0x1, %al
14097a6e7: 0f 87 13 02 00 00           	ja	0x14097a900 <.text+0x979900>
14097a6ed: a8 01                       	testb	$0x1, %al
14097a6ef: 0f 84 0b 02 00 00           	je	0x14097a900 <.text+0x979900>
14097a6f5: 4d 39 6e 38                 	cmpq	%r13, 0x38(%r14)
14097a6f9: 0f 85 01 02 00 00           	jne	0x14097a900 <.text+0x979900>
14097a6ff: 44 89 bd 4c 03 00 00        	movl	%r15d, 0x34c(%rbp)
14097a706: 89 bd 3c 03 00 00           	movl	%edi, 0x33c(%rbp)
14097a70c: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
14097a713: 8b 85 7c 02 00 00           	movl	0x27c(%rbp), %eax
14097a719: 89 01                       	movl	%eax, (%rcx)
14097a71b: 48 8b 95 50 03 00 00        	movq	0x350(%rbp), %rdx
14097a722: c6 85 70 01 00 00 00        	movb	$0x0, 0x170(%rbp)
14097a729: 4c 8d 85 70 01 00 00        	leaq	0x170(%rbp), %r8
14097a730: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
14097a737: e8 c4 f0 12 00              	callq	0x140aa9800 <.text+0xaa8800>
14097a73c: 48 c7 85 90 02 00 00 00 00 00 00    	movq	$0x0, 0x290(%rbp)
14097a747: 48 c7 85 98 02 00 00 08 00 00 00    	movq	$0x8, 0x298(%rbp)
14097a752: 48 c7 85 a0 02 00 00 00 00 00 00    	movq	$0x0, 0x2a0(%rbp)
14097a75d: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
14097a764: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14097a76b: 66 c7 85 80 01 00 00 01 00  	movw	$0x1, 0x180(%rbp)
14097a774: 48 b8 0b 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff5, %rax # imm = 0x800000000000000B
14097a77e: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
14097a785: c6 85 67 03 00 00 01        	movb	$0x1, 0x367(%rbp)
14097a78c: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
14097a793: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097a79a: e8 a1 83 10 00              	callq	0x140a82b40 <.text+0xa81b40>
14097a79f: 48 8b 95 90 04 00 00        	movq	0x490(%rbp), %rdx
14097a7a6: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14097a7a9: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14097a7ad: 89 01                       	movl	%eax, (%rcx)
14097a7af: 4c 8b 32                    	movq	(%rdx), %r14
14097a7b2: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
14097a7b9: 0f 29 85 e0 02 00 00        	movaps	%xmm0, 0x2e0(%rbp)
14097a7c0: 48 8b 85 a0 02 00 00        	movq	0x2a0(%rbp), %rax
14097a7c7: 48 8b 8d e0 02 00 00        	movq	0x2e0(%rbp), %rcx
14097a7ce: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
14097a7d5: 48 c7 85 e0 02 00 00 00 00 00 00    	movq	$0x0, 0x2e0(%rbp)
14097a7e0: 48 c7 85 e8 02 00 00 08 00 00 00    	movq	$0x8, 0x2e8(%rbp)
14097a7eb: 48 c7 85 f0 02 00 00 00 00 00 00    	movq	$0x0, 0x2f0(%rbp)
14097a7f6: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14097a7fa: 48 8d 04 c2                 	leaq	(%rdx,%rax,8), %rax
14097a7fe: 48 89 95 70 01 00 00        	movq	%rdx, 0x170(%rbp)
14097a805: 48 89 95 78 01 00 00        	movq	%rdx, 0x178(%rbp)
14097a80c: 48 89 8d 80 01 00 00        	movq	%rcx, 0x180(%rbp)
14097a813: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
14097a81a: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14097a821: e8 2a c7 d1 00              	callq	0x141696f50 <.text+0x1695f50>
14097a826: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14097a82d: 48 89 95 80 01 00 00        	movq	%rdx, 0x180(%rbp)
14097a834: 48 c7 85 70 01 00 00 07 00 00 00    	movq	$0x7, 0x170(%rbp)
14097a83f: c6 85 67 03 00 00 00        	movb	$0x0, 0x367(%rbp)
14097a846: 4c 8d 05 2b c4 13 02        	leaq	0x213c42b(%rip), %r8    # 0x142ab6c78
14097a84d: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097a854: 4c 89 f1                    	movq	%r14, %rcx
14097a857: e8 64 a6 10 00              	callq	0x140a84ec0 <.text+0xa83ec0>
14097a85c: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
14097a863: 80 78 f9 02                 	cmpb	$0x2, -0x7(%rax)
14097a867: 40 0f 93 c7                 	setae	%dil
14097a86b: 8b 85 1c 01 00 00           	movl	0x11c(%rbp), %eax
14097a871: 89 85 7f 01 00 00           	movl	%eax, 0x17f(%rbp)
14097a877: 48 8b 85 14 01 00 00        	movq	0x114(%rbp), %rax
14097a87e: 48 89 85 77 01 00 00        	movq	%rax, 0x177(%rbp)
14097a885: 8b 45 2c                    	movl	0x2c(%rbp), %eax
14097a888: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
14097a88c: 89 01                       	movl	%eax, (%rcx)
14097a88e: 4c 8b 7d 10                 	movq	0x10(%rbp), %r15
14097a892: 4d 8b 77 30                 	movq	0x30(%r15), %r14
14097a896: 4d 8b 6f 40                 	movq	0x40(%r15), %r13
14097a89a: 4d 3b 77 20                 	cmpq	0x20(%r15), %r14
14097a89e: 75 17                       	jne	0x14097a8b7 <.text+0x9798b7>
14097a8a0: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
14097a8a4: c6 85 67 03 00 00 00        	movb	$0x0, 0x367(%rbp)
14097a8ab: 48 8d 15 96 f8 2b 02        	leaq	0x22bf896(%rip), %rdx   # 0x142c3a148
14097a8b2: e8 79 83 be ff              	callq	0x140562c30 <.text+0x561c30>
14097a8b7: 40 00 ff                    	addb	%dil, %dil
14097a8ba: 40 80 cf 11                 	orb	$0x11, %dil
14097a8be: 49 8b 47 28                 	movq	0x28(%r15), %rax
14097a8c2: 4c 89 f1                    	movq	%r14, %rcx
14097a8c5: 48 c1 e1 05                 	shlq	$0x5, %rcx
14097a8c9: 4c 89 2c 08                 	movq	%r13, (%rax,%rcx)
14097a8cd: 40 88 7c 08 08              	movb	%dil, 0x8(%rax,%rcx)
14097a8d2: 8b 95 7f 01 00 00           	movl	0x17f(%rbp), %edx
14097a8d8: 89 54 08 18                 	movl	%edx, 0x18(%rax,%rcx)
14097a8dc: 0f 10 85 70 01 00 00        	movups	0x170(%rbp), %xmm0
14097a8e3: 0f 11 44 08 09              	movups	%xmm0, 0x9(%rax,%rcx)
14097a8e8: 49 ff c6                    	incq	%r14
14097a8eb: 4d 89 77 30                 	movq	%r14, 0x30(%r15)
14097a8ef: 49 ff 47 40                 	incq	0x40(%r15)
14097a8f3: 8b bd 3c 03 00 00           	movl	0x33c(%rbp), %edi
14097a8f9: 44 8b bd 4c 03 00 00        	movl	0x34c(%rbp), %r15d
14097a900: 66 0f 6e b5 38 03 00 00     	movd	0x338(%rbp), %xmm6
14097a908: 66 45 0f 6e c7              	movd	%r15d, %xmm8
14097a90d: 66 44 0f 6f d6              	movdqa	%xmm6, %xmm10
14097a912: f6 85 60 03 00 00 01        	testb	$0x1, 0x360(%rbp)
14097a919: 44 8b ad 34 03 00 00        	movl	0x334(%rbp), %r13d
14097a920: 75 05                       	jne	0x14097a927 <.text+0x979927>
14097a922: 66 45 0f 6f d0              	movdqa	%xmm8, %xmm10
14097a927: 66 41 0f 6e fd              	movd	%r13d, %xmm7
14097a92c: 66 44 0f 6e cf              	movd	%edi, %xmm9
14097a931: 66 44 0f 6f df              	movdqa	%xmm7, %xmm11
14097a936: 75 05                       	jne	0x14097a93d <.text+0x97993d>
14097a938: 66 45 0f 6f d9              	movdqa	%xmm9, %xmm11
14097a93d: 66 44 0f 7e 95 a8 00 00 00  	movd	%xmm10, 0xa8(%rbp)
14097a946: 66 44 0f 7e 9d ac 00 00 00  	movd	%xmm11, 0xac(%rbp)
14097a94f: 48 8b 85 b8 04 00 00        	movq	0x4b8(%rbp), %rax
14097a956: 4c 8b 28                    	movq	(%rax), %r13
14097a959: 4c 89 e9                    	movq	%r13, %rcx
14097a95c: 66 41 0f 6f ca              	movdqa	%xmm10, %xmm1
14097a961: 66 41 0f 6f d3              	movdqa	%xmm11, %xmm2
14097a966: e8 d5 ec 06 00              	callq	0x1409e9640 <.text+0x9e8640>
14097a96b: 44 0f 28 e0                 	movaps	%xmm0, %xmm12
14097a96f: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
14097a976: f3 0f 10 40 e0              	movss	-0x20(%rax), %xmm0
14097a97b: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
14097a97f: f3 41 0f c2 d4 03           	cmpunordss	%xmm12, %xmm2
14097a985: 0f 28 ca                    	movaps	%xmm2, %xmm1
14097a988: 0f 54 d0                    	andps	%xmm0, %xmm2
14097a98b: f3 41 0f 5f c4              	maxss	%xmm12, %xmm0
14097a990: 0f 55 c8                    	andnps	%xmm0, %xmm1
14097a993: 0f 56 d1                    	orps	%xmm1, %xmm2
14097a996: 48 8d 8d c0 02 00 00        	leaq	0x2c0(%rbp), %rcx
14097a99d: 48 8d 95 a8 00 00 00        	leaq	0xa8(%rbp), %rdx
14097a9a4: e8 37 24 32 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14097a9a9: f6 85 60 03 00 00 01        	testb	$0x1, 0x360(%rbp)
14097a9b0: 4c 8b b5 58 01 00 00        	movq	0x158(%rbp), %r14
14097a9b7: 0f 84 02 01 00 00           	je	0x14097aabf <.text+0x979abf>
14097a9bd: 44 89 bd e0 02 00 00        	movl	%r15d, 0x2e0(%rbp)
14097a9c4: 89 bd e4 02 00 00           	movl	%edi, 0x2e4(%rbp)
14097a9ca: 8b 85 38 03 00 00           	movl	0x338(%rbp), %eax
14097a9d0: 89 85 e8 02 00 00           	movl	%eax, 0x2e8(%rbp)
14097a9d6: 8b 85 34 03 00 00           	movl	0x334(%rbp), %eax
14097a9dc: 89 85 ec 02 00 00           	movl	%eax, 0x2ec(%rbp)
14097a9e2: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
14097a9e9: 48 89 85 f0 02 00 00        	movq	%rax, 0x2f0(%rbp)
14097a9f0: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14097a9f7: 48 8d 95 e0 02 00 00        	leaq	0x2e0(%rbp), %rdx
14097a9fe: e8 6d eb 31 00              	callq	0x140c99570 <.text+0xc98570>
14097aa03: f3 0f 10 9d 70 01 00 00     	movss	0x170(%rbp), %xmm3
14097aa0b: f3 0f 10 a5 74 01 00 00     	movss	0x174(%rbp), %xmm4
14097aa13: f2 0f 10 8d 78 01 00 00     	movsd	0x178(%rbp), %xmm1
14097aa1b: f3 44 0f 10 ad 80 01 00 00  	movss	0x180(%rbp), %xmm13
14097aa24: f2 0f 10 95 84 01 00 00     	movsd	0x184(%rbp), %xmm2
14097aa2c: f3 0f 10 85 8c 01 00 00     	movss	0x18c(%rbp), %xmm0
14097aa34: f3 0f 10 ad 90 01 00 00     	movss	0x190(%rbp), %xmm5
14097aa3c: 45 0f 57 f6                 	xorps	%xmm14, %xmm14
14097aa40: 44 0f 2e f5                 	ucomiss	%xmm5, %xmm14
14097aa44: 0f 83 81 00 00 00           	jae	0x14097aacb <.text+0x979acb>
14097aa4a: f3 41 0f 58 dd              	addss	%xmm13, %xmm3
14097aa4f: f3 0f 58 2d ad ae fa 01     	addss	0x1faaead(%rip), %xmm5  # 0x142925904
14097aa57: 44 0f 2e f3                 	ucomiss	%xmm3, %xmm14
14097aa5b: 0f 83 c8 00 00 00           	jae	0x14097ab29 <.text+0x979b29>
14097aa61: 0f 16 dc                    	movlhps	%xmm4, %xmm3            # xmm3 = xmm3[0],xmm4[0]
14097aa64: 0f c6 e2 30                 	shufps	$0x30, %xmm2, %xmm4     # xmm4 = xmm4[0,0],xmm2[3,0]
14097aa68: 44 0f 28 ea                 	movaps	%xmm2, %xmm13
14097aa6c: 44 0f c6 ec 84              	shufps	$0x84, %xmm4, %xmm13    # xmm13 = xmm13[0,1],xmm4[0,2]
14097aa71: 0f c6 d3 24                 	shufps	$0x24, %xmm3, %xmm2     # xmm2 = xmm2[0,1],xmm3[2,0]
14097aa75: 0f 16 c1                    	movlhps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0]
14097aa78: 0f 28 d8                    	movaps	%xmm0, %xmm3
14097aa7b: 0f c6 d9 d8                 	shufps	$0xd8, %xmm1, %xmm3     # xmm3 = xmm3[0,2],xmm1[1,3]
14097aa7f: 44 0f 5c eb                 	subps	%xmm3, %xmm13
14097aa83: 0f c6 e9 d4                 	shufps	$0xd4, %xmm1, %xmm5     # xmm5 = xmm5[0,1],xmm1[1,3]
14097aa87: 0f c6 c5 28                 	shufps	$0x28, %xmm5, %xmm0     # xmm0 = xmm0[0,2],xmm5[2,0]
14097aa8b: 0f 58 c2                    	addps	%xmm2, %xmm0
14097aa8e: 0f 28 c8                    	movaps	%xmm0, %xmm1
14097aa91: 41 0f c6 cd 23              	shufps	$0x23, %xmm13, %xmm1    # xmm1 = xmm1[3,0],xmm13[2,0]
14097aa96: 44 0f c6 e9 24              	shufps	$0x24, %xmm1, %xmm13    # xmm13 = xmm13[0,1],xmm1[2,0]
14097aa9b: 0f c6 c0 ff                 	shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
14097aa9f: 0f 57 c9                    	xorps	%xmm1, %xmm1
14097aaa2: f3 0f 51 c8                 	sqrtss	%xmm0, %xmm1
14097aaa6: f3 0f 10 05 c2 ad fa 01     	movss	0x1faadc2(%rip), %xmm0  # 0x142925870
14097aaae: f3 0f 5e c1                 	divss	%xmm1, %xmm0
14097aab2: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14097aab6: 41 0f 59 c5                 	mulps	%xmm13, %xmm0
14097aaba: e9 05 01 00 00              	jmp	0x14097abc4 <.text+0x979bc4>
14097aabf: 0f 28 05 fa 35 fc 01        	movaps	0x1fc35fa(%rip), %xmm0  # 0x14293e0c0
14097aac6: e9 f9 00 00 00              	jmp	0x14097abc4 <.text+0x979bc4>
14097aacb: f3 44 0f 5c eb              	subss	%xmm3, %xmm13
14097aad0: f3 0f 10 1d 2c ae fa 01     	movss	0x1faae2c(%rip), %xmm3  # 0x142925904
14097aad8: f3 0f 5c dd                 	subss	%xmm5, %xmm3
14097aadc: 45 0f 2e f5                 	ucomiss	%xmm13, %xmm14
14097aae0: 0f 83 8f 00 00 00           	jae	0x14097ab75 <.text+0x979b75>
14097aae6: 41 0f 14 e5                 	unpcklps	%xmm13, %xmm4           # xmm4 = xmm4[0],xmm13[0],xmm4[1],xmm13[1]
14097aaea: 0f 16 e2                    	movlhps	%xmm2, %xmm4            # xmm4 = xmm4[0],xmm2[0]
14097aaed: 0f 28 d1                    	movaps	%xmm1, %xmm2
14097aaf0: 0f c6 d1 25                 	shufps	$0x25, %xmm1, %xmm2     # xmm2 = xmm2[1,1],xmm1[2,0]
14097aaf4: 0f c6 d9 d4                 	shufps	$0xd4, %xmm1, %xmm3     # xmm3 = xmm3[0,1],xmm1[1,3]
14097aaf8: 0f 16 c1                    	movlhps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0]
14097aafb: 0f c6 d8 82                 	shufps	$0x82, %xmm0, %xmm3     # xmm3 = xmm3[2,0],xmm0[0,2]
14097aaff: 0f 58 dc                    	addps	%xmm4, %xmm3
14097ab02: 0f 5c e2                    	subps	%xmm2, %xmm4
14097ab05: 0f c6 e3 23                 	shufps	$0x23, %xmm3, %xmm4     # xmm4 = xmm4[3,0],xmm3[2,0]
14097ab09: f3 0f 16 c3                 	movshdup	%xmm3, %xmm0            # xmm0 = xmm3[1,1,3,3]
14097ab0d: 0f c6 dc 24                 	shufps	$0x24, %xmm4, %xmm3     # xmm3 = xmm3[0,1],xmm4[2,0]
14097ab11: 0f 57 c9                    	xorps	%xmm1, %xmm1
14097ab14: f3 0f 51 c8                 	sqrtss	%xmm0, %xmm1
14097ab18: f3 0f 10 05 50 ad fa 01     	movss	0x1faad50(%rip), %xmm0  # 0x142925870
14097ab20: f3 0f 5e c1                 	divss	%xmm1, %xmm0
14097ab24: e9 94 00 00 00              	jmp	0x14097abbd <.text+0x979bbd>
14097ab29: 44 0f 28 ea                 	movaps	%xmm2, %xmm13
14097ab2d: 44 0f c6 ea e1              	shufps	$0xe1, %xmm2, %xmm13    # xmm13 = xmm13[1,0],xmm2[2,3]
14097ab32: 0f 16 e5                    	movlhps	%xmm5, %xmm4            # xmm4 = xmm4[0],xmm5[0]
14097ab35: 0f c6 d4 21                 	shufps	$0x21, %xmm4, %xmm2     # xmm2 = xmm2[1,0],xmm4[2,0]
14097ab39: 0f 16 c1                    	movlhps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0]
14097ab3c: 0f 28 e0                    	movaps	%xmm0, %xmm4
14097ab3f: 0f c6 e1 42                 	shufps	$0x42, %xmm1, %xmm4     # xmm4 = xmm4[2,0],xmm1[0,1]
14097ab43: 41 0f 58 e5                 	addps	%xmm13, %xmm4
14097ab47: 0f c6 d9 d4                 	shufps	$0xd4, %xmm1, %xmm3     # xmm3 = xmm3[0,1],xmm1[1,3]
14097ab4b: 0f c6 c3 82                 	shufps	$0x82, %xmm3, %xmm0     # xmm0 = xmm0[2,0],xmm3[0,2]
14097ab4f: 0f 5c d0                    	subps	%xmm0, %xmm2
14097ab52: 0f c6 e2 e4                 	shufps	$0xe4, %xmm2, %xmm4     # xmm4 = xmm4[0,1],xmm2[2,3]
14097ab56: 0f 12 d2                    	movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
14097ab59: 0f 57 c9                    	xorps	%xmm1, %xmm1
14097ab5c: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
14097ab60: f3 0f 10 05 08 ad fa 01     	movss	0x1faad08(%rip), %xmm0  # 0x142925870
14097ab68: f3 0f 5e c1                 	divss	%xmm1, %xmm0
14097ab6c: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14097ab70: 0f 59 c4                    	mulps	%xmm4, %xmm0
14097ab73: eb 4f                       	jmp	0x14097abc4 <.text+0x979bc4>
14097ab75: 0f 14 dc                    	unpcklps	%xmm4, %xmm3            # xmm3 = xmm3[0],xmm4[0],xmm3[1],xmm4[1]
14097ab78: 0f 28 e3                    	movaps	%xmm3, %xmm4
14097ab7b: 0f 16 e1                    	movlhps	%xmm1, %xmm4            # xmm4 = xmm4[0],xmm1[0]
14097ab7e: 0f 28 ea                    	movaps	%xmm2, %xmm5
14097ab81: 0f 16 e9                    	movlhps	%xmm1, %xmm5            # xmm5 = xmm5[0],xmm1[0]
14097ab84: 0f c6 dd 24                 	shufps	$0x24, %xmm5, %xmm3     # xmm3 = xmm3[0,1],xmm5[2,0]
14097ab88: 0f 28 e9                    	movaps	%xmm1, %xmm5
14097ab8b: f3 41 0f 10 ed              	movss	%xmm13, %xmm5           # xmm5 = xmm13[0],xmm5[1,2,3]
14097ab90: 0f c6 ca 55                 	shufps	$0x55, %xmm2, %xmm1     # xmm1 = xmm1[1,1],xmm2[1,1]
14097ab94: 0f 58 cc                    	addps	%xmm4, %xmm1
14097ab97: 0f c6 c2 d4                 	shufps	$0xd4, %xmm2, %xmm0     # xmm0 = xmm0[0,1],xmm2[1,3]
14097ab9b: 0f c6 e8 24                 	shufps	$0x24, %xmm0, %xmm5     # xmm5 = xmm5[0,1],xmm0[2,0]
14097ab9f: 0f 5c dd                    	subps	%xmm5, %xmm3
14097aba2: 0f 57 d2                    	xorps	%xmm2, %xmm2
14097aba5: f3 0f 51 d3                 	sqrtss	%xmm3, %xmm2
14097aba9: 0f c6 d9 9c                 	shufps	$0x9c, %xmm1, %xmm3     # xmm3 = xmm3[0,3],xmm1[1,2]
14097abad: 0f c6 db 78                 	shufps	$0x78, %xmm3, %xmm3     # xmm3 = xmm3[0,2,3,1]
14097abb1: f3 0f 10 05 b7 ac fa 01     	movss	0x1faacb7(%rip), %xmm0  # 0x142925870
14097abb9: f3 0f 5e c2                 	divss	%xmm2, %xmm0
14097abbd: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14097abc1: 0f 59 c3                    	mulps	%xmm3, %xmm0
14097abc4: 4c 8b 85 98 04 00 00        	movq	0x498(%rbp), %r8
14097abcb: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
14097abd2: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
14097abd9: 8b 8d c8 02 00 00           	movl	0x2c8(%rbp), %ecx
14097abdf: 89 8d 78 01 00 00           	movl	%ecx, 0x178(%rbp)
14097abe5: 48 8b 8d c0 02 00 00        	movq	0x2c0(%rbp), %rcx
14097abec: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
14097abf3: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
14097abf7: 48 8b 8d 58 03 00 00        	movq	0x358(%rbp), %rcx
14097abfe: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
14097ac03: f3 44 0f 11 64 24 30        	movss	%xmm12, 0x30(%rsp)
14097ac0a: 48 89 54 24 28              	movq	%rdx, 0x28(%rsp)
14097ac0f: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14097ac14: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14097ac1b: 48 89 ea                    	movq	%rbp, %rdx
14097ac1e: 49 89 d9                    	movq	%rbx, %r9
14097ac21: e8 5a 26 00 00              	callq	0x14097d280 <.text+0x97c280>
14097ac26: b1 02                       	movb	$0x2, %cl
14097ac28: 80 78 18 02                 	cmpb	$0x2, 0x18(%rax)
14097ac2c: 72 0a                       	jb	0x14097ac38 <.text+0x979c38>
14097ac2e: 48 39 50 38                 	cmpq	%rdx, 0x38(%rax)
14097ac32: 75 04                       	jne	0x14097ac38 <.text+0x979c38>
14097ac34: 0f b6 48 19                 	movzbl	0x19(%rax), %ecx
14097ac38: 80 c1 fe                    	addb	$-0x2, %cl
14097ac3b: 0f b6 c1                    	movzbl	%cl, %eax
14097ac3e: 3c 03                       	cmpb	$0x3, %al
14097ac40: b9 03 00 00 00              	movl	$0x3, %ecx
14097ac45: 0f 42 c8                    	cmovbl	%eax, %ecx
14097ac48: 0f b6 c1                    	movzbl	%cl, %eax
14097ac4b: 48 8d 0d c2 c8 13 02        	leaq	0x213c8c2(%rip), %rcx   # 0x142ab7514
14097ac52: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
14097ac56: 48 01 c8                    	addq	%rcx, %rax
14097ac59: ff e0                       	jmpq	*%rax
14097ac5b: 44 89 bd 4c 03 00 00        	movl	%r15d, 0x34c(%rbp)
14097ac62: 89 bd 3c 03 00 00           	movl	%edi, 0x33c(%rbp)
14097ac68: 48 8b 9d 88 02 00 00        	movq	0x288(%rbp), %rbx
14097ac6f: 44 8b b5 b8 02 00 00        	movl	0x2b8(%rbp), %r14d
14097ac76: 44 89 33                    	movl	%r14d, (%rbx)
14097ac79: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
14097ac80: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
14097ac8b: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14097ac92: 48 c7 85 80 01 00 00 01 00 00 00    	movq	$0x1, 0x180(%rbp)
14097ac9d: 48 8d 05 ec bf 13 02        	leaq	0x213bfec(%rip), %rax   # 0x142ab6c90
14097aca4: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14097aca9: 48 8b 85 b0 04 00 00        	movq	0x4b0(%rbp), %rax
14097acb0: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14097acb5: 48 c7 44 24 30 0a 00 00 00  	movq	$0xa, 0x30(%rsp)
14097acbe: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097acc5: 41 b8 02 00 00 00           	movl	$0x2, %r8d
14097accb: 48 8b bd b0 02 00 00        	movq	0x2b0(%rbp), %rdi
14097acd2: 48 89 f9                    	movq	%rdi, %rcx
14097acd5: 48 8b b5 d8 02 00 00        	movq	0x2d8(%rbp), %rsi
14097acdc: 49 89 f1                    	movq	%rsi, %r9
14097acdf: e8 0c cb a6 00              	callq	0x1413e77f0 <.text+0x13e67f0>
14097ace4: 44 89 33                    	movl	%r14d, (%rbx)
14097ace7: 48 8b 95 50 03 00 00        	movq	0x350(%rbp), %rdx
14097acee: 48 89 f9                    	movq	%rdi, %rcx
14097acf1: 49 89 f0                    	movq	%rsi, %r8
14097acf4: e8 07 0f 1b 00              	callq	0x140b2bc00 <.text+0xb2ac00>
14097acf9: 44 89 33                    	movl	%r14d, (%rbx)
14097acfc: 48 83 c7 30                 	addq	$0x30, %rdi
14097ad00: 48 8d 86 90 00 00 00        	leaq	0x90(%rsi), %rax
14097ad07: 48 8b 8e 90 00 00 00        	movq	0x90(%rsi), %rcx
14097ad0e: 48 85 c9                    	testq	%rcx, %rcx
14097ad11: 48 0f 44 c1                 	cmoveq	%rcx, %rax
14097ad15: 48 c7 85 70 01 00 00 01 00 00 00    	movq	$0x1, 0x170(%rbp)
14097ad20: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14097ad27: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
14097ad32: 48 c7 85 a8 01 00 00 00 00 00 00    	movq	$0x0, 0x1a8(%rbp)
14097ad3d: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14097ad44: 48 89 fa                    	movq	%rdi, %rdx
14097ad47: e8 34 85 91 00              	callq	0x141293280 <.text+0x1292280>
14097ad4c: 48 c7 85 e0 02 00 00 00 00 00 00    	movq	$0x0, 0x2e0(%rbp)
14097ad57: 48 c7 85 e8 02 00 00 04 00 00 00    	movq	$0x4, 0x2e8(%rbp)
14097ad62: 48 c7 85 f0 02 00 00 00 00 00 00    	movq	$0x0, 0x2f0(%rbp)
14097ad6d: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
14097ad74: 8b 8d 70 02 00 00           	movl	0x270(%rbp), %ecx
14097ad7a: 89 08                       	movl	%ecx, (%rax)
14097ad7c: 48 8b 95 50 03 00 00        	movq	0x350(%rbp), %rdx
14097ad83: 48 89 f1                    	movq	%rsi, %rcx
14097ad86: e8 95 36 0a 00              	callq	0x140a1e420 <.text+0xa1d420>
14097ad8b: 48 85 c0                    	testq	%rax, %rax
14097ad8e: 0f 84 69 02 00 00           	je	0x14097affd <.text+0x979ffd>
14097ad94: 48 8b 58 30                 	movq	0x30(%rax), %rbx
14097ad98: 4c 8b 70 48                 	movq	0x48(%rax), %r14
14097ad9c: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
14097ada0: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14097ada4: f7 d0                       	notl	%eax
14097ada6: 4c 8d 7b 10                 	leaq	0x10(%rbx), %r15
14097adaa: 45 31 e4                    	xorl	%r12d, %r12d
14097adad: eb 31                       	jmp	0x14097ade0 <.text+0x979de0>
14097adaf: f3 41 0f 5c f2              	subss	%xmm10, %xmm6
14097adb4: f3 41 0f 5c fb              	subss	%xmm11, %xmm7
14097adb9: 48 8b 85 e8 02 00 00        	movq	0x2e8(%rbp), %rax
14097adc0: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
14097adc4: 89 3c 88                    	movl	%edi, (%rax,%rcx,4)
14097adc7: f3 0f 11 74 88 04           	movss	%xmm6, 0x4(%rax,%rcx,4)
14097adcd: f3 0f 11 7c 88 08           	movss	%xmm7, 0x8(%rax,%rcx,4)
14097add3: 48 ff c6                    	incq	%rsi
14097add6: 48 89 b5 f0 02 00 00        	movq	%rsi, 0x2f0(%rbp)
14097addd: 44 89 e8                    	movl	%r13d, %eax
14097ade0: 4d 85 f6                    	testq	%r14, %r14
14097ade3: 0f 84 14 02 00 00           	je	0x14097affd <.text+0x979ffd>
14097ade9: 66 85 c0                    	testw	%ax, %ax
14097adec: 74 12                       	je	0x14097ae00 <.text+0x979e00>
14097adee: 44 8d 68 ff                 	leal	-0x1(%rax), %r13d
14097adf2: 41 21 c5                    	andl	%eax, %r13d
14097adf5: eb 33                       	jmp	0x14097ae2a <.text+0x979e2a>
14097adf7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14097ae00: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
14097ae05: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14097ae09: 48 81 c3 00 fb ff ff        	addq	$-0x500, %rbx           # imm = 0xFB00
14097ae10: 49 83 c7 10                 	addq	$0x10, %r15
14097ae14: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
14097ae19: 74 e5                       	je	0x14097ae00 <.text+0x979e00>
14097ae1b: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14097ae20: 29 c1                       	subl	%eax, %ecx
14097ae22: f7 d0                       	notl	%eax
14097ae24: 41 89 c5                    	movl	%eax, %r13d
14097ae27: 41 21 cd                    	andl	%ecx, %r13d
14097ae2a: 49 ff ce                    	decq	%r14
14097ae2d: f3 0f bc c0                 	tzcntl	%eax, %eax
14097ae31: 48 f7 d8                    	negq	%rax
14097ae34: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
14097ae38: 48 c1 e0 04                 	shlq	$0x4, %rax
14097ae3c: 0f b6 4c 03 da              	movzbl	-0x26(%rbx,%rax), %ecx
14097ae41: 8d 51 fe                    	leal	-0x2(%rcx), %edx
14097ae44: 48 ff c9                    	decq	%rcx
14097ae47: 80 fa 03                    	cmpb	$0x3, %dl
14097ae4a: 49 0f 43 cc                 	cmovaeq	%r12, %rcx
14097ae4e: 48 85 c9                    	testq	%rcx, %rcx
14097ae51: 74 0e                       	je	0x14097ae61 <.text+0x979e61>
14097ae53: 44 89 e8                    	movl	%r13d, %eax
14097ae56: 48 83 f9 01                 	cmpq	$0x1, %rcx
14097ae5a: 74 84                       	je	0x14097ade0 <.text+0x979de0>
14097ae5c: e9 e5 1a 00 00              	jmp	0x14097c946 <.text+0x97b946>
14097ae61: 48 01 d8                    	addq	%rbx, %rax
14097ae64: f3 0f 10 70 cc              	movss	-0x34(%rax), %xmm6
14097ae69: 66 0f 6e 78 d4              	movd	-0x2c(%rax), %xmm7
14097ae6e: 8b 78 b0                    	movl	-0x50(%rax), %edi
14097ae71: 48 8b b5 f0 02 00 00        	movq	0x2f0(%rbp), %rsi
14097ae78: 48 3b b5 e0 02 00 00        	cmpq	0x2e0(%rbp), %rsi
14097ae7f: 0f 85 2a ff ff ff           	jne	0x14097adaf <.text+0x979daf>
14097ae85: 48 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %rcx
14097ae8c: 48 8d 15 0d be 13 02        	leaq	0x213be0d(%rip), %rdx   # 0x142ab6ca0
14097ae93: e8 58 7e be ff              	callq	0x140562cf0 <.text+0x561cf0>
14097ae98: e9 12 ff ff ff              	jmp	0x14097adaf <.text+0x979daf>
14097ae9d: 48 89 b5 b0 02 00 00        	movq	%rsi, 0x2b0(%rbp)
14097aea4: 48 8b b5 50 03 00 00        	movq	0x350(%rbp), %rsi
14097aeab: 48 89 b5 70 01 00 00        	movq	%rsi, 0x170(%rbp)
14097aeb2: 49 83 7e 18 00              	cmpq	$0x0, 0x18(%r14)
14097aeb7: 0f 84 ef 07 00 00           	je	0x14097b6ac <.text+0x97a6ac>
14097aebd: 49 8d 4e 20                 	leaq	0x20(%r14), %rcx
14097aec1: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097aec8: e8 a3 5e c3 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14097aecd: 49 8b 0e                    	movq	(%r14), %rcx
14097aed0: 49 8b 56 08                 	movq	0x8(%r14), %rdx
14097aed4: 49 89 d0                    	movq	%rdx, %r8
14097aed7: 49 21 c0                    	andq	%rax, %r8
14097aeda: 48 c1 e8 39                 	shrq	$0x39, %rax
14097aede: 66 0f 6e c0                 	movd	%eax, %xmm0
14097aee2: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14097aee6: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14097aeeb: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14097aef0: 48 8d 41 88                 	leaq	-0x78(%rcx), %rax
14097aef4: 45 31 c9                    	xorl	%r9d, %r9d
14097aef7: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14097aefb: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
14097af01: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14097af05: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14097af09: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
14097af0e: 45 85 db                    	testl	%r11d, %r11d
14097af11: 74 43                       	je	0x14097af56 <.text+0x979f56>
14097af13: 89 bd 3c 03 00 00           	movl	%edi, 0x33c(%rbp)
14097af19: 44 89 bd 4c 03 00 00        	movl	%r15d, 0x34c(%rbp)
14097af20: f3 45 0f bc d3              	tzcntl	%r11d, %r10d
14097af25: 4d 01 c2                    	addq	%r8, %r10
14097af28: 49 21 d2                    	andq	%rdx, %r10
14097af2b: 49 f7 da                    	negq	%r10
14097af2e: 4d 6b d2 78                 	imulq	$0x78, %r10, %r10
14097af32: 4a 39 34 10                 	cmpq	%rsi, (%rax,%r10)
14097af36: 0f 84 b0 01 00 00           	je	0x14097b0ec <.text+0x97a0ec>
14097af3c: 45 8d 53 ff                 	leal	-0x1(%r11), %r10d
14097af40: 66 45 21 da                 	andw	%r11w, %r10w
14097af44: 45 89 d3                    	movl	%r10d, %r11d
14097af47: 8b bd 3c 03 00 00           	movl	0x33c(%rbp), %edi
14097af4d: 44 8b bd 4c 03 00 00        	movl	0x34c(%rbp), %r15d
14097af54: 75 ca                       	jne	0x14097af20 <.text+0x979f20>
14097af56: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14097af5a: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14097af5f: 45 85 d2                    	testl	%r10d, %r10d
14097af62: 0f 85 44 07 00 00           	jne	0x14097b6ac <.text+0x97a6ac>
14097af68: 4d 01 c8                    	addq	%r9, %r8
14097af6b: 49 83 c0 10                 	addq	$0x10, %r8
14097af6f: 49 83 c1 10                 	addq	$0x10, %r9
14097af73: 49 21 d0                    	andq	%rdx, %r8
14097af76: eb 83                       	jmp	0x14097aefb <.text+0x979efb>
14097af78: 41 89 fc                    	movl	%edi, %r12d
14097af7b: 48 8b 85 88 04 00 00        	movq	0x488(%rbp), %rax
14097af82: 48 8b 70 10                 	movq	0x10(%rax), %rsi
14097af86: 45 31 f6                    	xorl	%r14d, %r14d
14097af89: b9 0c 00 00 00              	movl	$0xc, %ecx
14097af8e: 48 89 f0                    	movq	%rsi, %rax
14097af91: 48 f7 e1                    	mulq	%rcx
14097af94: 48 89 c7                    	movq	%rax, %rdi
14097af97: 0f 80 c3 19 00 00           	jo	0x14097c960 <.text+0x97b960>
14097af9d: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14097afa7: 48 39 c7                    	cmpq	%rax, %rdi
14097afaa: 0f 87 b0 19 00 00           	ja	0x14097c960 <.text+0x97b960>
14097afb0: 45 89 fd                    	movl	%r15d, %r13d
14097afb3: 4c 8b bd 50 03 00 00        	movq	0x350(%rbp), %r15
14097afba: 48 8b 85 88 04 00 00        	movq	0x488(%rbp), %rax
14097afc1: 48 8b 58 08                 	movq	0x8(%rax), %rbx
14097afc5: 48 85 ff                    	testq	%rdi, %rdi
14097afc8: 0f 84 90 01 00 00           	je	0x14097b15e <.text+0x97a15e>
14097afce: 0f b6 05 34 9c cb 02        	movzbl	0x2cb9c34(%rip), %eax   # 0x143634c09
14097afd5: 41 be 04 00 00 00           	movl	$0x4, %r14d
14097afdb: ba 04 00 00 00              	movl	$0x4, %edx
14097afe0: 48 89 f9                    	movq	%rdi, %rcx
14097afe3: e8 28 8c c9 ff              	callq	0x140613c10 <.text+0x612c10>
14097afe8: 48 85 c0                    	testq	%rax, %rax
14097afeb: 0f 84 6f 19 00 00           	je	0x14097c960 <.text+0x97b960>
14097aff1: 48 89 b5 58 03 00 00        	movq	%rsi, 0x358(%rbp)
14097aff8: e9 71 01 00 00              	jmp	0x14097b16e <.text+0x97a16e>
14097affd: 48 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %rax
14097b004: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
14097b00b: 0f 10 85 e0 02 00 00        	movups	0x2e0(%rbp), %xmm0
14097b012: 0f 29 85 70 01 00 00        	movaps	%xmm0, 0x170(%rbp)
14097b019: 48 8b b5 88 04 00 00        	movq	0x488(%rbp), %rsi
14097b020: 48 8b 06                    	movq	(%rsi), %rax
14097b023: 48 85 c0                    	testq	%rax, %rax
14097b026: 74 17                       	je	0x14097b03f <.text+0x97a03f>
14097b028: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
14097b02c: 48 c1 e0 02                 	shlq	$0x2, %rax
14097b030: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14097b034: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14097b03a: e8 e1 8b c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097b03f: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
14097b046: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14097b04a: 0f 28 85 70 01 00 00        	movaps	0x170(%rbp), %xmm0
14097b051: 0f 11 06                    	movups	%xmm0, (%rsi)
14097b054: 8b 85 60 03 00 00           	movl	0x360(%rbp), %eax
14097b05a: 89 46 20                    	movl	%eax, 0x20(%rsi)
14097b05d: 8b 85 4c 03 00 00           	movl	0x34c(%rbp), %eax
14097b063: 89 46 24                    	movl	%eax, 0x24(%rsi)
14097b066: 8b 85 3c 03 00 00           	movl	0x33c(%rbp), %eax
14097b06c: 89 46 28                    	movl	%eax, 0x28(%rsi)
14097b06f: 8b 85 38 03 00 00           	movl	0x338(%rbp), %eax
14097b075: 89 46 2c                    	movl	%eax, 0x2c(%rsi)
14097b078: 8b 85 34 03 00 00           	movl	0x334(%rbp), %eax
14097b07e: 89 46 30                    	movl	%eax, 0x30(%rsi)
14097b081: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
14097b088: 48 89 46 34                 	movq	%rax, 0x34(%rsi)
14097b08c: f3 0f 10 85 c4 02 00 00     	movss	0x2c4(%rbp), %xmm0
14097b094: f3 0f 11 46 3c              	movss	%xmm0, 0x3c(%rsi)
14097b099: 48 8b 85 10 03 00 00        	movq	0x310(%rbp), %rax
14097b0a0: f3 0f 5c 40 04              	subss	0x4(%rax), %xmm0
14097b0a5: f3 0f 5e 40 10              	divss	0x10(%rax), %xmm0
14097b0aa: f3 0f 10 08                 	movss	(%rax), %xmm1
14097b0ae: f3 0f 10 50 08              	movss	0x8(%rax), %xmm2
14097b0b3: 0f 14 ca                    	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
14097b0b6: f3 0f 10 50 0c              	movss	0xc(%rax), %xmm2
14097b0bb: f3 0f 10 58 14              	movss	0x14(%rax), %xmm3
14097b0c0: 0f 14 d3                    	unpcklps	%xmm3, %xmm2            # xmm2 = xmm2[0],xmm3[0],xmm2[1],xmm3[1]
14097b0c3: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
14097b0c7: 0f 59 c2                    	mulps	%xmm2, %xmm0
14097b0ca: 0f 58 c1                    	addps	%xmm1, %xmm0
14097b0cd: f3 0f 10 8d c0 02 00 00     	movss	0x2c0(%rbp), %xmm1
14097b0d5: f3 0f 10 95 c8 02 00 00     	movss	0x2c8(%rbp), %xmm2
14097b0dd: 0f 14 ca                    	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
14097b0e0: 0f 5c c1                    	subps	%xmm1, %xmm0
14097b0e3: 0f 13 46 18                 	movlps	%xmm0, 0x18(%rsi)
14097b0e7: e9 8a 03 00 00              	jmp	0x14097b476 <.text+0x97a476>
14097b0ec: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
14097b0f3: 48 c1 e8 20                 	shrq	$0x20, %rax
14097b0f7: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
14097b0fe: 48 8b bd 28 03 00 00        	movq	0x328(%rbp), %rdi
14097b105: f3 0f 10 47 e0              	movss	-0x20(%rdi), %xmm0
14097b10a: 66 0f ef c9                 	pxor	%xmm1, %xmm1
14097b10e: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
14097b111: 4c 8b bd 88 04 00 00        	movq	0x488(%rbp), %r15
14097b118: 0f 85 9a 05 00 00           	jne	0x14097b6b8 <.text+0x97a6b8>
14097b11e: 0f 8a 94 05 00 00           	jp	0x14097b6b8 <.text+0x97a6b8>
14097b124: 0f 57 c0                    	xorps	%xmm0, %xmm0
14097b127: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
14097b12e: f6 00 01                    	testb	$0x1, (%rax)
14097b131: 4c 8b 9d c8 04 00 00        	movq	0x4c8(%rbp), %r11
14097b138: 48 8b 95 d0 04 00 00        	movq	0x4d0(%rbp), %rdx
14097b13f: 0f 84 bb 05 00 00           	je	0x14097b700 <.text+0x97a700>
14097b145: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
14097b14c: f3 0f 10 40 04              	movss	0x4(%rax), %xmm0
14097b151: f3 0f 10 48 0c              	movss	0xc(%rax), %xmm1
14097b156: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
14097b159: e9 a2 05 00 00              	jmp	0x14097b700 <.text+0x97a700>
14097b15e: b8 04 00 00 00              	movl	$0x4, %eax
14097b163: 48 c7 85 58 03 00 00 00 00 00 00    	movq	$0x0, 0x358(%rbp)
14097b16e: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
14097b175: 48 89 c1                    	movq	%rax, %rcx
14097b178: 48 89 da                    	movq	%rbx, %rdx
14097b17b: 49 89 f8                    	movq	%rdi, %r8
14097b17e: e8 7d 8a fa 01              	callq	0x142923c00 <.text+0x2922c00>
14097b183: 48 8b 95 a8 04 00 00        	movq	0x4a8(%rbp), %rdx
14097b18a: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14097b18d: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14097b191: 89 01                       	movl	%eax, (%rcx)
14097b193: 48 8b 3a                    	movq	(%rdx), %rdi
14097b196: 48 8b 5f 30                 	movq	0x30(%rdi), %rbx
14097b19a: 4c 8b 77 40                 	movq	0x40(%rdi), %r14
14097b19e: 48 3b 5f 20                 	cmpq	0x20(%rdi), %rbx
14097b1a2: 75 10                       	jne	0x14097b1b4 <.text+0x97a1b4>
14097b1a4: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
14097b1a8: 48 8d 15 99 ef 2b 02        	leaq	0x22bef99(%rip), %rdx   # 0x142c3a148
14097b1af: e8 3c 78 be ff              	callq	0x1405629f0 <.text+0x5619f0>
14097b1b4: 48 8b 47 28                 	movq	0x28(%rdi), %rax
14097b1b8: 48 8d 0c 5b                 	leaq	(%rbx,%rbx,2), %rcx
14097b1bc: 48 c1 e1 04                 	shlq	$0x4, %rcx
14097b1c0: 48 8b 95 58 03 00 00        	movq	0x358(%rbp), %rdx
14097b1c7: 48 89 14 08                 	movq	%rdx, (%rax,%rcx)
14097b1cb: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
14097b1d2: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
14097b1d7: 48 89 74 08 10              	movq	%rsi, 0x10(%rax,%rcx)
14097b1dc: 4c 89 7c 08 18              	movq	%r15, 0x18(%rax,%rcx)
14097b1e1: 66 44 0f 7e 54 08 20        	movd	%xmm10, 0x20(%rax,%rcx)
14097b1e8: 66 44 0f 7e 5c 08 24        	movd	%xmm11, 0x24(%rax,%rcx)
14097b1ef: 4c 89 74 08 28              	movq	%r14, 0x28(%rax,%rcx)
14097b1f4: 48 ff c3                    	incq	%rbx
14097b1f7: 48 89 5f 30                 	movq	%rbx, 0x30(%rdi)
14097b1fb: 48 ff 47 40                 	incq	0x40(%rdi)
14097b1ff: 48 8b bd 50 03 00 00        	movq	0x350(%rbp), %rdi
14097b206: 48 8b 45 50                 	movq	0x50(%rbp), %rax
14097b20a: 8b 8d e0 00 00 00           	movl	0xe0(%rbp), %ecx
14097b210: 89 08                       	movl	%ecx, (%rax)
14097b212: 4c 8b 75 48                 	movq	0x48(%rbp), %r14
14097b216: 49 8b 76 30                 	movq	0x30(%r14), %rsi
14097b21a: 49 8b 5e 40                 	movq	0x40(%r14), %rbx
14097b21e: 49 3b 76 20                 	cmpq	0x20(%r14), %rsi
14097b222: 75 10                       	jne	0x14097b234 <.text+0x97a234>
14097b224: 49 8d 4e 20                 	leaq	0x20(%r14), %rcx
14097b228: 48 8d 15 19 ef 2b 02        	leaq	0x22bef19(%rip), %rdx   # 0x142c3a148
14097b22f: e8 fc 73 be ff              	callq	0x140562630 <.text+0x561630>
14097b234: 49 8b 46 28                 	movq	0x28(%r14), %rax
14097b238: 48 89 f1                    	movq	%rsi, %rcx
14097b23b: 48 c1 e1 04                 	shlq	$0x4, %rcx
14097b23f: 48 89 1c 08                 	movq	%rbx, (%rax,%rcx)
14097b243: 48 89 7c 08 08              	movq	%rdi, 0x8(%rax,%rcx)
14097b248: 48 ff c6                    	incq	%rsi
14097b24b: 49 89 76 30                 	movq	%rsi, 0x30(%r14)
14097b24f: 49 ff 46 40                 	incq	0x40(%r14)
14097b253: 48 8b 85 88 04 00 00        	movq	0x488(%rbp), %rax
14097b25a: 83 78 20 02                 	cmpl	$0x2, 0x20(%rax)
14097b25e: 48 8b 9d b0 00 00 00        	movq	0xb0(%rbp), %rbx
14097b265: 48 8b bd 28 03 00 00        	movq	0x328(%rbp), %rdi
14097b26c: 0f 84 70 01 00 00           	je	0x14097b3e2 <.text+0x97a3e2>
14097b272: 48 83 c0 20                 	addq	$0x20, %rax
14097b276: 0f 10 00                    	movups	(%rax), %xmm0
14097b279: 0f 10 48 0c                 	movups	0xc(%rax), %xmm1
14097b27d: 0f 11 8d 8c 01 00 00        	movups	%xmm1, 0x18c(%rbp)
14097b284: 0f 11 85 80 01 00 00        	movups	%xmm0, 0x180(%rbp)
14097b28b: 48 c7 85 90 02 00 00 00 00 00 00    	movq	$0x0, 0x290(%rbp)
14097b296: 48 c7 85 98 02 00 00 08 00 00 00    	movq	$0x8, 0x298(%rbp)
14097b2a1: 48 c7 85 a0 02 00 00 00 00 00 00    	movq	$0x0, 0x2a0(%rbp)
14097b2ac: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
14097b2b3: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14097b2ba: 8b 85 60 03 00 00           	movl	0x360(%rbp), %eax
14097b2c0: 89 85 9c 01 00 00           	movl	%eax, 0x19c(%rbp)
14097b2c6: 44 89 ad a0 01 00 00        	movl	%r13d, 0x1a0(%rbp)
14097b2cd: 44 89 a5 a4 01 00 00        	movl	%r12d, 0x1a4(%rbp)
14097b2d4: 8b 85 38 03 00 00           	movl	0x338(%rbp), %eax
14097b2da: 89 85 a8 01 00 00           	movl	%eax, 0x1a8(%rbp)
14097b2e0: 8b 85 34 03 00 00           	movl	0x334(%rbp), %eax
14097b2e6: 89 85 ac 01 00 00           	movl	%eax, 0x1ac(%rbp)
14097b2ec: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
14097b2f3: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14097b2fa: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
14097b304: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
14097b30b: c6 85 66 03 00 00 01        	movb	$0x1, 0x366(%rbp)
14097b312: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
14097b319: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097b320: e8 1b 78 10 00              	callq	0x140a82b40 <.text+0xa81b40>
14097b325: 48 8b 95 90 04 00 00        	movq	0x490(%rbp), %rdx
14097b32c: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14097b32f: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14097b333: 89 01                       	movl	%eax, (%rcx)
14097b335: 48 8b 32                    	movq	(%rdx), %rsi
14097b338: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
14097b33f: 0f 29 85 e0 02 00 00        	movaps	%xmm0, 0x2e0(%rbp)
14097b346: 48 8b 85 a0 02 00 00        	movq	0x2a0(%rbp), %rax
14097b34d: 48 8b 8d e0 02 00 00        	movq	0x2e0(%rbp), %rcx
14097b354: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
14097b35b: 48 c7 85 e0 02 00 00 00 00 00 00    	movq	$0x0, 0x2e0(%rbp)
14097b366: 48 c7 85 e8 02 00 00 08 00 00 00    	movq	$0x8, 0x2e8(%rbp)
14097b371: 48 c7 85 f0 02 00 00 00 00 00 00    	movq	$0x0, 0x2f0(%rbp)
14097b37c: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14097b380: 48 8d 04 c2                 	leaq	(%rdx,%rax,8), %rax
14097b384: 48 89 95 70 01 00 00        	movq	%rdx, 0x170(%rbp)
14097b38b: 48 89 95 78 01 00 00        	movq	%rdx, 0x178(%rbp)
14097b392: 48 89 8d 80 01 00 00        	movq	%rcx, 0x180(%rbp)
14097b399: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
14097b3a0: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14097b3a7: e8 a4 bb d1 00              	callq	0x141696f50 <.text+0x1695f50>
14097b3ac: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14097b3b3: 48 89 95 80 01 00 00        	movq	%rdx, 0x180(%rbp)
14097b3ba: 48 c7 85 70 01 00 00 07 00 00 00    	movq	$0x7, 0x170(%rbp)
14097b3c5: c6 85 66 03 00 00 00        	movb	$0x0, 0x366(%rbp)
14097b3cc: 4c 8d 05 45 b9 13 02        	leaq	0x213b945(%rip), %r8    # 0x142ab6d18
14097b3d3: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097b3da: 48 89 f1                    	movq	%rsi, %rcx
14097b3dd: e8 de 9a 10 00              	callq	0x140a84ec0 <.text+0xa83ec0>
14097b3e2: f3 0f 10 77 e0              	movss	-0x20(%rdi), %xmm6
14097b3e7: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
14097b3ee: 8b 8d 4c 01 00 00           	movl	0x14c(%rbp), %ecx
14097b3f4: 89 08                       	movl	%ecx, (%rax)
14097b3f6: 48 8b 73 30                 	movq	0x30(%rbx), %rsi
14097b3fa: 48 8b 7b 40                 	movq	0x40(%rbx), %rdi
14097b3fe: 48 3b 73 20                 	cmpq	0x20(%rbx), %rsi
14097b402: 75 10                       	jne	0x14097b414 <.text+0x97a414>
14097b404: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
14097b408: 48 8d 15 39 ed 2b 02        	leaq	0x22bed39(%rip), %rdx   # 0x142c3a148
14097b40f: e8 dc 75 be ff              	callq	0x1405629f0 <.text+0x5619f0>
14097b414: 48 8b 43 28                 	movq	0x28(%rbx), %rax
14097b418: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
14097b41c: 48 c1 e1 04                 	shlq	$0x4, %rcx
14097b420: 8b 95 60 03 00 00           	movl	0x360(%rbp), %edx
14097b426: 89 14 08                    	movl	%edx, (%rax,%rcx)
14097b429: 44 89 6c 08 04              	movl	%r13d, 0x4(%rax,%rcx)
14097b42e: 44 89 64 08 08              	movl	%r12d, 0x8(%rax,%rcx)
14097b433: 8b 95 38 03 00 00           	movl	0x338(%rbp), %edx
14097b439: 89 54 08 0c                 	movl	%edx, 0xc(%rax,%rcx)
14097b43d: 8b 95 34 03 00 00           	movl	0x334(%rbp), %edx
14097b443: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
14097b447: 48 8b 95 20 03 00 00        	movq	0x320(%rbp), %rdx
14097b44e: 48 89 54 08 14              	movq	%rdx, 0x14(%rax,%rcx)
14097b453: f3 0f 11 74 08 1c           	movss	%xmm6, 0x1c(%rax,%rcx)
14097b459: c7 44 08 20 00 00 c0 3f     	movl	$0x3fc00000, 0x20(%rax,%rcx) # imm = 0x3FC00000
14097b461: c6 44 08 24 01              	movb	$0x1, 0x24(%rax,%rcx)
14097b466: 48 89 7c 08 28              	movq	%rdi, 0x28(%rax,%rcx)
14097b46b: 48 ff c6                    	incq	%rsi
14097b46e: 48 89 73 30                 	movq	%rsi, 0x30(%rbx)
14097b472: 48 ff 43 40                 	incq	0x40(%rbx)
14097b476: 31 db                       	xorl	%ebx, %ebx
14097b478: 4c 8b 9d 00 01 00 00        	movq	0x100(%rbp), %r11
14097b47f: 49 8b 03                    	movq	(%r11), %rax
14097b482: 31 c9                       	xorl	%ecx, %ecx
14097b484: 48 89 c2                    	movq	%rax, %rdx
14097b487: 48 8b 75 38                 	movq	0x38(%rbp), %rsi
14097b48b: 48 2b 56 18                 	subq	0x18(%rsi), %rdx
14097b48f: 48 0f 42 d1                 	cmovbq	%rcx, %rdx
14097b493: 48 2b 46 38                 	subq	0x38(%rsi), %rax
14097b497: 48 0f 42 c1                 	cmovbq	%rcx, %rax
14097b49b: 4c 8b 46 30                 	movq	0x30(%rsi), %r8
14097b49f: 49 29 c0                    	subq	%rax, %r8
14097b4a2: 4c 0f 42 c1                 	cmovbq	%rcx, %r8
14097b4a6: 41 0f 96 c1                 	setbe	%r9b
14097b4aa: 4c 8b 56 10                 	movq	0x10(%rsi), %r10
14097b4ae: 49 29 d2                    	subq	%rdx, %r10
14097b4b1: 4c 0f 42 d1                 	cmovbq	%rcx, %r10
14097b4b5: 48 8b 46 40                 	movq	0x40(%rsi), %rax
14097b4b9: 0f 96 c1                    	setbe	%cl
14097b4bc: 4d 01 c2                    	addq	%r8, %r10
14097b4bf: 48 89 c2                    	movq	%rax, %rdx
14097b4c2: 4c 29 d2                    	subq	%r10, %rdx
14097b4c5: 49 89 13                    	movq	%rdx, (%r11)
14097b4c8: 44 84 c9                    	testb	%r9b, %cl
14097b4cb: 0f 85 4d 0e 00 00           	jne	0x14097c31e <.text+0x97b31e>
14097b4d1: 49 89 03                    	movq	%rax, (%r11)
14097b4d4: 4c 8b b5 50 03 00 00        	movq	0x350(%rbp), %r14
14097b4db: 48 8b 95 a0 04 00 00        	movq	0x4a0(%rbp), %rdx
14097b4e2: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14097b4e5: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14097b4e9: 89 01                       	movl	%eax, (%rcx)
14097b4eb: 48 8b 32                    	movq	(%rdx), %rsi
14097b4ee: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
14097b4f2: 4c 8b 7e 40                 	movq	0x40(%rsi), %r15
14097b4f6: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
14097b4fa: 75 10                       	jne	0x14097b50c <.text+0x97a50c>
14097b4fc: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
14097b500: 48 8d 15 41 ec 2b 02        	leaq	0x22bec41(%rip), %rdx   # 0x142c3a148
14097b507: e8 24 77 be ff              	callq	0x140562c30 <.text+0x561c30>
14097b50c: 48 8b 46 28                 	movq	0x28(%rsi), %rax
14097b510: 48 89 f9                    	movq	%rdi, %rcx
14097b513: 48 c1 e1 05                 	shlq	$0x5, %rcx
14097b517: 4c 89 3c 08                 	movq	%r15, (%rax,%rcx)
14097b51b: 4c 89 74 08 08              	movq	%r14, 0x8(%rax,%rcx)
14097b520: 48 8b 95 c0 02 00 00        	movq	0x2c0(%rbp), %rdx
14097b527: 48 89 54 08 10              	movq	%rdx, 0x10(%rax,%rcx)
14097b52c: 8b 95 c8 02 00 00           	movl	0x2c8(%rbp), %edx
14097b532: 89 54 08 18                 	movl	%edx, 0x18(%rax,%rcx)
14097b536: 66 c7 44 08 1c 04 00        	movw	$0x4, 0x1c(%rax,%rcx)
14097b53d: 48 ff c7                    	incq	%rdi
14097b540: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
14097b544: 48 ff 46 40                 	incq	0x40(%rsi)
14097b548: 48 8b bd 50 03 00 00        	movq	0x350(%rbp), %rdi
14097b54f: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
14097b556: 66 0f 6e 50 1c              	movd	0x1c(%rax), %xmm2
14097b55b: 66 45 0f ef c0              	pxor	%xmm8, %xmm8
14097b560: 66 0f 6e 0d 88 b0 13 02     	movd	0x213b088(%rip), %xmm1  # 0x142ab65f0
14097b568: 0f 57 c0                    	xorps	%xmm0, %xmm0
14097b56b: e8 30 4f 31 00              	callq	0x140c904a0 <.text+0xc8f4a0>
14097b570: f3 44 0f 5f c0              	maxss	%xmm0, %xmm8
14097b575: f3 0f 10 35 87 a3 fa 01     	movss	0x1faa387(%rip), %xmm6  # 0x142925904
14097b57d: 0f 28 fe                    	movaps	%xmm6, %xmm7
14097b580: f3 41 0f 5d f8              	minss	%xmm8, %xmm7
14097b585: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
14097b58c: 8b 8d 48 01 00 00           	movl	0x148(%rbp), %ecx
14097b592: 89 08                       	movl	%ecx, (%rax)
14097b594: 4c 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %r8
14097b59b: 4d 8b 70 30                 	movq	0x30(%r8), %r14
14097b59f: 4d 8b 78 40                 	movq	0x40(%r8), %r15
14097b5a3: 49 8d 70 20                 	leaq	0x20(%r8), %rsi
14097b5a7: 4d 3b 70 20                 	cmpq	0x20(%r8), %r14
14097b5ab: 75 15                       	jne	0x14097b5c2 <.text+0x97a5c2>
14097b5ad: 48 8d 15 94 eb 2b 02        	leaq	0x22beb94(%rip), %rdx   # 0x142c3a148
14097b5b4: 48 89 f1                    	movq	%rsi, %rcx
14097b5b7: 4d 89 c4                    	movq	%r8, %r12
14097b5ba: e8 f1 74 be ff              	callq	0x140562ab0 <.text+0x561ab0>
14097b5bf: 4d 89 e0                    	movq	%r12, %r8
14097b5c2: 49 8b 40 28                 	movq	0x28(%r8), %rax
14097b5c6: 4b 8d 0c b6                 	leaq	(%r14,%r14,4), %rcx
14097b5ca: 4c 89 3c c8                 	movq	%r15, (%rax,%rcx,8)
14097b5ce: 48 89 7c c8 08              	movq	%rdi, 0x8(%rax,%rcx,8)
14097b5d3: 48 8b 95 c0 02 00 00        	movq	0x2c0(%rbp), %rdx
14097b5da: 48 89 54 c8 10              	movq	%rdx, 0x10(%rax,%rcx,8)
14097b5df: 8b 95 c8 02 00 00           	movl	0x2c8(%rbp), %edx
14097b5e5: 89 54 c8 18                 	movl	%edx, 0x18(%rax,%rcx,8)
14097b5e9: f3 0f 11 7c c8 1c           	movss	%xmm7, 0x1c(%rax,%rcx,8)
14097b5ef: c6 44 c8 20 04              	movb	$0x4, 0x20(%rax,%rcx,8)
14097b5f4: 49 ff c6                    	incq	%r14
14097b5f7: 4d 89 70 30                 	movq	%r14, 0x30(%r8)
14097b5fb: 49 ff 40 40                 	incq	0x40(%r8)
14097b5ff: 84 db                       	testb	%bl, %bl
14097b601: 0f 84 17 0d 00 00           	je	0x14097c31e <.text+0x97b31e>
14097b607: 4d 89 c7                    	movq	%r8, %r15
14097b60a: 48 8b bd 50 03 00 00        	movq	0x350(%rbp), %rdi
14097b611: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
14097b618: 66 0f 6e 50 1c              	movd	0x1c(%rax), %xmm2
14097b61d: 0f 57 ff                    	xorps	%xmm7, %xmm7
14097b620: 66 0f 6e 0d c8 af 13 02     	movd	0x213afc8(%rip), %xmm1  # 0x142ab65f0
14097b628: 0f 57 c0                    	xorps	%xmm0, %xmm0
14097b62b: e8 70 4e 31 00              	callq	0x140c904a0 <.text+0xc8f4a0>
14097b630: f3 0f 5f f8                 	maxss	%xmm0, %xmm7
14097b634: f3 0f 5d f7                 	minss	%xmm7, %xmm6
14097b638: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
14097b63f: 8b 8d 48 01 00 00           	movl	0x148(%rbp), %ecx
14097b645: 89 08                       	movl	%ecx, (%rax)
14097b647: 4d 89 f8                    	movq	%r15, %r8
14097b64a: 49 8b 5f 30                 	movq	0x30(%r15), %rbx
14097b64e: 4d 8b 77 40                 	movq	0x40(%r15), %r14
14097b652: 49 3b 5f 20                 	cmpq	0x20(%r15), %rbx
14097b656: 75 12                       	jne	0x14097b66a <.text+0x97a66a>
14097b658: 48 8d 15 e9 ea 2b 02        	leaq	0x22beae9(%rip), %rdx   # 0x142c3a148
14097b65f: 48 89 f1                    	movq	%rsi, %rcx
14097b662: e8 49 74 be ff              	callq	0x140562ab0 <.text+0x561ab0>
14097b667: 4d 89 f8                    	movq	%r15, %r8
14097b66a: 49 8b 40 28                 	movq	0x28(%r8), %rax
14097b66e: 48 8d 0c 9b                 	leaq	(%rbx,%rbx,4), %rcx
14097b672: 4c 89 34 c8                 	movq	%r14, (%rax,%rcx,8)
14097b676: 48 89 7c c8 08              	movq	%rdi, 0x8(%rax,%rcx,8)
14097b67b: 48 8b 95 c0 02 00 00        	movq	0x2c0(%rbp), %rdx
14097b682: 48 89 54 c8 10              	movq	%rdx, 0x10(%rax,%rcx,8)
14097b687: 8b 95 c8 02 00 00           	movl	0x2c8(%rbp), %edx
14097b68d: 89 54 c8 18                 	movl	%edx, 0x18(%rax,%rcx,8)
14097b691: f3 0f 11 74 c8 1c           	movss	%xmm6, 0x1c(%rax,%rcx,8)
14097b697: c6 44 c8 20 05              	movb	$0x5, 0x20(%rax,%rcx,8)
14097b69c: 48 ff c3                    	incq	%rbx
14097b69f: 49 89 58 30                 	movq	%rbx, 0x30(%r8)
14097b6a3: 49 ff 40 40                 	incq	0x40(%r8)
14097b6a7: e9 72 0c 00 00              	jmp	0x14097c31e <.text+0x97b31e>
14097b6ac: 48 8d 0d 1d b6 13 02        	leaq	0x213b61d(%rip), %rcx   # 0x142ab6cd0
14097b6b3: e8 58 dc f5 01              	callq	0x1428d9310 <.text+0x28d8310>
14097b6b8: f3 41 0f 10 47 3c           	movss	0x3c(%r15), %xmm0
14097b6be: 48 8b 85 10 03 00 00        	movq	0x310(%rbp), %rax
14097b6c5: f3 0f 5c 40 04              	subss	0x4(%rax), %xmm0
14097b6ca: f3 0f 5e 40 10              	divss	0x10(%rax), %xmm0
14097b6cf: f3 0f 10 08                 	movss	(%rax), %xmm1
14097b6d3: f3 0f 10 50 08              	movss	0x8(%rax), %xmm2
14097b6d8: 0f 14 ca                    	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
14097b6db: f3 0f 10 50 0c              	movss	0xc(%rax), %xmm2
14097b6e0: f3 0f 10 58 14              	movss	0x14(%rax), %xmm3
14097b6e5: 0f 14 d3                    	unpcklps	%xmm3, %xmm2            # xmm2 = xmm2[0],xmm3[0],xmm2[1],xmm3[1]
14097b6e8: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
14097b6ec: 0f 59 c2                    	mulps	%xmm2, %xmm0
14097b6ef: 0f 58 c1                    	addps	%xmm1, %xmm0
14097b6f2: 4c 8b 9d c8 04 00 00        	movq	0x4c8(%rbp), %r11
14097b6f9: 48 8b 95 d0 04 00 00        	movq	0x4d0(%rbp), %rdx
14097b700: 4a 8d 04 11                 	leaq	(%rcx,%r10), %rax
14097b704: 48 89 85 88 02 00 00        	movq	%rax, 0x288(%rbp)
14097b70b: f3 46 0f 10 64 11 e4        	movss	-0x1c(%rcx,%r10), %xmm12
14097b712: f2 41 0f 10 4f 18           	movsd	0x18(%r15), %xmm1
14097b718: 0f 5c c1                    	subps	%xmm1, %xmm0
14097b71b: 0f 13 85 d0 02 00 00        	movlps	%xmm0, 0x2d0(%rbp)
14097b722: 48 8b 0a                    	movq	(%rdx), %rcx
14097b725: 48 8b 01                    	movq	(%rcx), %rax
14097b728: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
14097b72c: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
14097b730: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
14097b735: 41 f7 d0                    	notl	%r8d
14097b738: 48 8d 50 10                 	leaq	0x10(%rax), %rdx
14097b73c: 48 8b b5 50 03 00 00        	movq	0x350(%rbp), %rsi
14097b743: 48 83 c1 ff                 	addq	$-0x1, %rcx
14097b747: 0f 92 c3                    	setb	%bl
14097b74a: 73 5f                       	jae	0x14097b7ab <.text+0x97a7ab>
14097b74c: 66 45 85 c0                 	testw	%r8w, %r8w
14097b750: 74 0e                       	je	0x14097b760 <.text+0x97a760>
14097b752: 45 8d 48 ff                 	leal	-0x1(%r8), %r9d
14097b756: 45 21 c1                    	andl	%r8d, %r9d
14097b759: eb 33                       	jmp	0x14097b78e <.text+0x97a78e>
14097b75b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14097b760: 66 0f 6f 02                 	movdqa	(%rdx), %xmm0
14097b764: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
14097b769: 48 05 80 fd ff ff           	addq	$-0x280, %rax           # imm = 0xFD80
14097b76f: 48 83 c2 10                 	addq	$0x10, %rdx
14097b773: 41 81 f8 ff ff 00 00        	cmpl	$0xffff, %r8d           # imm = 0xFFFF
14097b77a: 74 e4                       	je	0x14097b760 <.text+0x97a760>
14097b77c: 41 ba fe ff ff ff           	movl	$0xfffffffe, %r10d      # imm = 0xFFFFFFFE
14097b782: 45 29 c2                    	subl	%r8d, %r10d
14097b785: 41 f7 d0                    	notl	%r8d
14097b788: 45 89 c1                    	movl	%r8d, %r9d
14097b78b: 45 21 d1                    	andl	%r10d, %r9d
14097b78e: f3 45 0f bc c0              	tzcntl	%r8d, %r8d
14097b793: 49 f7 d8                    	negq	%r8
14097b796: 4f 8d 14 80                 	leaq	(%r8,%r8,4), %r10
14097b79a: 4a 39 74 d0 d8              	cmpq	%rsi, -0x28(%rax,%r10,8)
14097b79f: 74 0a                       	je	0x14097b7ab <.text+0x97a7ab>
14097b7a1: 45 89 c8                    	movl	%r9d, %r8d
14097b7a4: 4a 39 74 d0 e0              	cmpq	%rsi, -0x20(%rax,%r10,8)
14097b7a9: 75 98                       	jne	0x14097b743 <.text+0x97a743>
14097b7ab: 49 8b 03                    	movq	(%r11), %rax
14097b7ae: 80 38 00                    	cmpb	$0x0, (%rax)
14097b7b1: 4c 89 b5 58 01 00 00        	movq	%r14, 0x158(%rbp)
14097b7b8: 0f 85 8c 01 00 00           	jne	0x14097b94a <.text+0x97a94a>
14097b7be: 48 89 b5 70 01 00 00        	movq	%rsi, 0x170(%rbp)
14097b7c5: 49 83 7c 24 48 00           	cmpq	$0x0, 0x48(%r12)
14097b7cb: 0f 84 79 01 00 00           	je	0x14097b94a <.text+0x97a94a>
14097b7d1: 49 8d 4c 24 50              	leaq	0x50(%r12), %rcx
14097b7d6: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097b7dd: e8 8e 55 c3 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14097b7e2: 49 8b 4c 24 30              	movq	0x30(%r12), %rcx
14097b7e7: 49 8b 54 24 38              	movq	0x38(%r12), %rdx
14097b7ec: 49 89 d0                    	movq	%rdx, %r8
14097b7ef: 49 21 c0                    	andq	%rax, %r8
14097b7f2: 48 c1 e8 39                 	shrq	$0x39, %rax
14097b7f6: 66 0f 6e c0                 	movd	%eax, %xmm0
14097b7fa: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14097b7fe: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14097b803: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14097b808: 48 8d 41 f0                 	leaq	-0x10(%rcx), %rax
14097b80c: 45 31 c9                    	xorl	%r9d, %r9d
14097b80f: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14097b813: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
14097b819: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14097b81d: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14097b821: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
14097b826: 45 85 d2                    	testl	%r10d, %r10d
14097b829: 74 31                       	je	0x14097b85c <.text+0x97a85c>
14097b82b: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14097b830: 4d 01 c3                    	addq	%r8, %r11
14097b833: 49 21 d3                    	andq	%rdx, %r11
14097b836: 4c 89 df                    	movq	%r11, %rdi
14097b839: 48 c1 e7 04                 	shlq	$0x4, %rdi
14097b83d: 49 89 c6                    	movq	%rax, %r14
14097b840: 49 29 fe                    	subq	%rdi, %r14
14097b843: 49 39 36                    	cmpq	%rsi, (%r14)
14097b846: 74 36                       	je	0x14097b87e <.text+0x97a87e>
14097b848: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
14097b84c: 66 45 21 d3                 	andw	%r10w, %r11w
14097b850: 45 89 da                    	movl	%r11d, %r10d
14097b853: 48 8b bd 28 03 00 00        	movq	0x328(%rbp), %rdi
14097b85a: 75 cf                       	jne	0x14097b82b <.text+0x97a82b>
14097b85c: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14097b860: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14097b865: 45 85 d2                    	testl	%r10d, %r10d
14097b868: 0f 85 dc 00 00 00           	jne	0x14097b94a <.text+0x97a94a>
14097b86e: 4d 01 c8                    	addq	%r9, %r8
14097b871: 49 83 c0 10                 	addq	$0x10, %r8
14097b875: 49 83 c1 10                 	addq	$0x10, %r9
14097b879: 49 21 d0                    	andq	%rdx, %r8
14097b87c: eb 95                       	jmp	0x14097b813 <.text+0x97a813>
14097b87e: 49 f7 db                    	negq	%r11
14097b881: 49 c1 e3 04                 	shlq	$0x4, %r11
14097b885: 4a 8b 4c 19 f8              	movq	-0x8(%rcx,%r11), %rcx
14097b88a: 48 89 c8                    	movq	%rcx, %rax
14097b88d: 48 c1 e8 20                 	shrq	$0x20, %rax
14097b891: 48 8b bd 28 03 00 00        	movq	0x328(%rbp), %rdi
14097b898: 0f 84 ac 00 00 00           	je	0x14097b94a <.text+0x97a94a>
14097b89e: 89 c9                       	movl	%ecx, %ecx
14097b8a0: 4c 8b 95 28 01 00 00        	movq	0x128(%rbp), %r10
14097b8a7: 49 3b 4a 10                 	cmpq	0x10(%r10), %rcx
14097b8ab: 0f 83 99 00 00 00           	jae	0x14097b94a <.text+0x97a94a>
14097b8b1: 49 8b 52 08                 	movq	0x8(%r10), %rdx
14097b8b5: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14097b8b9: 39 04 8a                    	cmpl	%eax, (%rdx,%rcx,4)
14097b8bc: 0f 85 88 00 00 00           	jne	0x14097b94a <.text+0x97a94a>
14097b8c2: 48 8d 04 8a                 	leaq	(%rdx,%rcx,4), %rax
14097b8c6: 8b 48 04                    	movl	0x4(%rax), %ecx
14097b8c9: ba ff ff ff ff              	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
14097b8ce: 48 39 d1                    	cmpq	%rdx, %rcx
14097b8d1: 74 77                       	je	0x14097b94a <.text+0x97a94a>
14097b8d3: 4c 8b 8d 50 01 00 00        	movq	0x150(%rbp), %r9
14097b8da: 49 39 49 38                 	cmpq	%rcx, 0x38(%r9)
14097b8de: 76 6a                       	jbe	0x14097b94a <.text+0x97a94a>
14097b8e0: 49 8b 51 28                 	movq	0x28(%r9), %rdx
14097b8e4: 41 89 c8                    	movl	%ecx, %r8d
14097b8e7: 41 c1 e8 06                 	shrl	$0x6, %r8d
14097b8eb: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
14097b8ef: 48 0f a3 ca                 	btq	%rcx, %rdx
14097b8f3: 0f 83 eb 0d 00 00           	jae	0x14097c6e4 <.text+0x97b6e4>
14097b8f9: 8b 48 0c                    	movl	0xc(%rax), %ecx
14097b8fc: 8b 40 10                    	movl	0x10(%rax), %eax
14097b8ff: 49 8b 92 a8 01 00 00        	movq	0x1a8(%r10), %rdx
14097b906: 4c 8d 04 c9                 	leaq	(%rcx,%rcx,8), %r8
14097b90a: 49 8b 89 10 01 00 00        	movq	0x110(%r9), %rcx
14097b911: 4a 39 4c c2 40              	cmpq	%rcx, 0x40(%rdx,%r8,8)
14097b916: 0f 86 94 0a 00 00           	jbe	0x14097c3b0 <.text+0x97b3b0>
14097b91c: 4a 8d 14 c2                 	leaq	(%rdx,%r8,8), %rdx
14097b920: 4c 8b 42 38                 	movq	0x38(%rdx), %r8
14097b924: 49 8b 0c c8                 	movq	(%r8,%rcx,8), %rcx
14097b928: 48 85 c9                    	testq	%rcx, %rcx
14097b92b: 0f 84 7f 0a 00 00           	je	0x14097c3b0 <.text+0x97b3b0>
14097b931: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14097b935: 48 f7 d1                    	notq	%rcx
14097b938: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14097b93c: 48 c1 e1 04                 	shlq	$0x4, %rcx
14097b940: 48 8b 74 0a 10              	movq	0x10(%rdx,%rcx), %rsi
14097b945: e9 68 0a 00 00              	jmp	0x14097c3b2 <.text+0x97b3b2>
14097b94a: 31 f6                       	xorl	%esi, %esi
14097b94c: 48 8b 95 40 03 00 00        	movq	0x340(%rbp), %rdx
14097b953: 41 88 77 40                 	movb	%sil, 0x40(%r15)
14097b957: 66 0f 6e 85 d0 02 00 00     	movd	0x2d0(%rbp), %xmm0
14097b95f: 66 0f 6e 8d d4 02 00 00     	movd	0x2d4(%rbp), %xmm1
14097b967: 0f b6 87 68 fe ff ff        	movzbl	-0x198(%rdi), %eax
14097b96e: f6 85 60 03 00 00 01        	testb	$0x1, 0x360(%rbp)
14097b975: 74 69                       	je	0x14097b9e0 <.text+0x97a9e0>
14097b977: a8 01                       	testb	$0x1, %al
14097b979: 8b 85 3c 03 00 00           	movl	0x33c(%rbp), %eax
14097b97f: 8b 8d 4c 03 00 00           	movl	0x34c(%rbp), %ecx
14097b985: 4c 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %r8
14097b98c: 0f 84 f2 0f 00 00           	je	0x14097c984 <.text+0x97b984>
14097b992: f3 44 0f 10 57 e0           	movss	-0x20(%rdi), %xmm10
14097b998: 66 0f 7e 8d 10 03 00 00     	movd	%xmm1, 0x310(%rbp)
14097b9a0: 66 0f 7e 85 58 03 00 00     	movd	%xmm0, 0x358(%rbp)
14097b9a8: 44 8b 8f 6c fe ff ff        	movl	-0x194(%rdi), %r9d
14097b9af: 41 b7 01                    	movb	$0x1, %r15b
14097b9b2: 41 ba 01 00 00 00           	movl	$0x1, %r10d
14097b9b8: 4c 8b b5 50 03 00 00        	movq	0x350(%rbp), %r14
14097b9bf: 89 85 80 02 00 00           	movl	%eax, 0x280(%rbp)
14097b9c5: 89 8d d8 02 00 00           	movl	%ecx, 0x2d8(%rbp)
14097b9cb: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
14097b9d2: 89 c7                       	movl	%eax, %edi
14097b9d4: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
14097b9db: 41 89 c3                    	movl	%eax, %r11d
14097b9de: eb 51                       	jmp	0x14097ba31 <.text+0x97aa31>
14097b9e0: a8 01                       	testb	$0x1, %al
14097b9e2: 4c 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %r8
14097b9e9: 0f 84 95 0f 00 00           	je	0x14097c984 <.text+0x97b984>
14097b9ef: 44 8b 9f 6c fe ff ff        	movl	-0x194(%rdi), %r11d
14097b9f6: 8b 7f e0                    	movl	-0x20(%rdi), %edi
14097b9f9: 66 0f 7e 8d 80 02 00 00     	movd	%xmm1, 0x280(%rbp)
14097ba01: 66 0f 7e 85 d8 02 00 00     	movd	%xmm0, 0x2d8(%rbp)
14097ba09: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14097ba0f: 4c 8b 95 50 03 00 00        	movq	0x350(%rbp), %r10
14097ba16: 45 31 ff                    	xorl	%r15d, %r15d
14097ba19: 8b 85 34 03 00 00           	movl	0x334(%rbp), %eax
14097ba1f: 89 85 10 03 00 00           	movl	%eax, 0x310(%rbp)
14097ba25: 8b 85 38 03 00 00           	movl	0x338(%rbp), %eax
14097ba2b: 89 85 58 03 00 00           	movl	%eax, 0x358(%rbp)
14097ba31: 8b 85 ec 00 00 00           	movl	0xec(%rbp), %eax
14097ba37: 41 89 00                    	movl	%eax, (%r8)
14097ba3a: 4c 8b 62 30                 	movq	0x30(%rdx), %r12
14097ba3e: 48 8b 72 40                 	movq	0x40(%rdx), %rsi
14097ba42: 4c 3b 62 20                 	cmpq	0x20(%rdx), %r12
14097ba46: 75 4b                       	jne	0x14097ba93 <.text+0x97aa93>
14097ba48: 48 8d 4a 20                 	leaq	0x20(%rdx), %rcx
14097ba4c: 48 8d 15 f5 e6 2b 02        	leaq	0x22be6f5(%rip), %rdx   # 0x142c3a148
14097ba53: 4c 89 b5 b0 02 00 00        	movq	%r14, 0x2b0(%rbp)
14097ba5a: 45 89 ce                    	movl	%r9d, %r14d
14097ba5d: 44 89 bd b8 02 00 00        	movl	%r15d, 0x2b8(%rbp)
14097ba64: 4d 89 d7                    	movq	%r10, %r15
14097ba67: 41 89 fd                    	movl	%edi, %r13d
14097ba6a: 44 89 df                    	movl	%r11d, %edi
14097ba6d: e8 3e 6d be ff              	callq	0x1405627b0 <.text+0x5617b0>
14097ba72: 41 89 fb                    	movl	%edi, %r11d
14097ba75: 44 89 ef                    	movl	%r13d, %edi
14097ba78: 4d 89 fa                    	movq	%r15, %r10
14097ba7b: 44 8b bd b8 02 00 00        	movl	0x2b8(%rbp), %r15d
14097ba82: 45 89 f1                    	movl	%r14d, %r9d
14097ba85: 4c 8b b5 b0 02 00 00        	movq	0x2b0(%rbp), %r14
14097ba8c: 48 8b 95 40 03 00 00        	movq	0x340(%rbp), %rdx
14097ba93: 4c 89 e0                    	movq	%r12, %rax
14097ba96: 48 c1 e0 06                 	shlq	$0x6, %rax
14097ba9a: 48 8b 4a 28                 	movq	0x28(%rdx), %rcx
14097ba9e: 44 88 3c 01                 	movb	%r15b, (%rcx,%rax)
14097baa2: c6 44 01 01 00              	movb	$0x0, 0x1(%rcx,%rax)
14097baa7: 44 8b 85 d8 02 00 00        	movl	0x2d8(%rbp), %r8d
14097baae: 44 89 44 01 04              	movl	%r8d, 0x4(%rcx,%rax)
14097bab3: 44 8b ad 80 02 00 00        	movl	0x280(%rbp), %r13d
14097baba: 44 89 6c 01 08              	movl	%r13d, 0x8(%rcx,%rax)
14097babf: 44 8b 85 58 03 00 00        	movl	0x358(%rbp), %r8d
14097bac6: 44 89 44 01 0c              	movl	%r8d, 0xc(%rcx,%rax)
14097bacb: 44 8b 85 10 03 00 00        	movl	0x310(%rbp), %r8d
14097bad2: 44 89 44 01 10              	movl	%r8d, 0x10(%rcx,%rax)
14097bad7: 89 7c 01 14                 	movl	%edi, 0x14(%rcx,%rax)
14097badb: 44 89 5c 01 18              	movl	%r11d, 0x18(%rcx,%rax)
14097bae0: f3 44 0f 11 54 01 1c        	movss	%xmm10, 0x1c(%rcx,%rax)
14097bae7: 44 89 4c 01 20              	movl	%r9d, 0x20(%rcx,%rax)
14097baec: c7 44 01 24 00 00 00 00     	movl	$0x0, 0x24(%rcx,%rax)
14097baf4: 4c 89 54 01 28              	movq	%r10, 0x28(%rcx,%rax)
14097baf9: 4c 89 74 01 30              	movq	%r14, 0x30(%rcx,%rax)
14097bafe: 48 89 74 01 38              	movq	%rsi, 0x38(%rcx,%rax)
14097bb03: 49 ff c4                    	incq	%r12
14097bb06: 4c 89 62 30                 	movq	%r12, 0x30(%rdx)
14097bb0a: 48 ff 42 40                 	incq	0x40(%rdx)
14097bb0e: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
14097bb15: f3 44 0f 10 50 e0           	movss	-0x20(%rax), %xmm10
14097bb1b: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
14097bb22: 8b 8d 4c 01 00 00           	movl	0x14c(%rbp), %ecx
14097bb28: 89 08                       	movl	%ecx, (%rax)
14097bb2a: 4c 8b b5 b0 00 00 00        	movq	0xb0(%rbp), %r14
14097bb31: 49 8b 76 30                 	movq	0x30(%r14), %rsi
14097bb35: 49 8b 7e 40                 	movq	0x40(%r14), %rdi
14097bb39: 49 3b 76 20                 	cmpq	0x20(%r14), %rsi
14097bb3d: 75 10                       	jne	0x14097bb4f <.text+0x97ab4f>
14097bb3f: 49 8d 4e 20                 	leaq	0x20(%r14), %rcx
14097bb43: 48 8d 15 fe e5 2b 02        	leaq	0x22be5fe(%rip), %rdx   # 0x142c3a148
14097bb4a: e8 a1 6e be ff              	callq	0x1405629f0 <.text+0x5619f0>
14097bb4f: 49 8b 46 28                 	movq	0x28(%r14), %rax
14097bb53: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
14097bb57: 48 c1 e1 04                 	shlq	$0x4, %rcx
14097bb5b: 8b 95 60 03 00 00           	movl	0x360(%rbp), %edx
14097bb61: 89 14 08                    	movl	%edx, (%rax,%rcx)
14097bb64: 44 8b a5 d8 02 00 00        	movl	0x2d8(%rbp), %r12d
14097bb6b: 44 89 64 08 04              	movl	%r12d, 0x4(%rax,%rcx)
14097bb70: 44 89 6c 08 08              	movl	%r13d, 0x8(%rax,%rcx)
14097bb75: 44 8b 95 58 03 00 00        	movl	0x358(%rbp), %r10d
14097bb7c: 44 89 54 08 0c              	movl	%r10d, 0xc(%rax,%rcx)
14097bb81: 44 8b 8d 10 03 00 00        	movl	0x310(%rbp), %r9d
14097bb88: 44 89 4c 08 10              	movl	%r9d, 0x10(%rax,%rcx)
14097bb8d: 4c 8b 85 20 03 00 00        	movq	0x320(%rbp), %r8
14097bb94: 4c 89 44 08 14              	movq	%r8, 0x14(%rax,%rcx)
14097bb99: f3 44 0f 11 54 08 1c        	movss	%xmm10, 0x1c(%rax,%rcx)
14097bba0: c7 44 08 20 00 00 c0 3f     	movl	$0x3fc00000, 0x20(%rax,%rcx) # imm = 0x3FC00000
14097bba8: c6 44 08 24 00              	movb	$0x0, 0x24(%rax,%rcx)
14097bbad: 48 89 7c 08 28              	movq	%rdi, 0x28(%rax,%rcx)
14097bbb2: 48 ff c6                    	incq	%rsi
14097bbb5: 49 89 76 30                 	movq	%rsi, 0x30(%r14)
14097bbb9: 49 ff 46 40                 	incq	0x40(%r14)
14097bbbd: 89 95 70 01 00 00           	movl	%edx, 0x170(%rbp)
14097bbc3: 44 89 a5 74 01 00 00        	movl	%r12d, 0x174(%rbp)
14097bbca: 44 89 ad 78 01 00 00        	movl	%r13d, 0x178(%rbp)
14097bbd1: 44 89 95 7c 01 00 00        	movl	%r10d, 0x17c(%rbp)
14097bbd8: 44 89 8d 80 01 00 00        	movl	%r9d, 0x180(%rbp)
14097bbdf: 4c 89 85 84 01 00 00        	movq	%r8, 0x184(%rbp)
14097bbe6: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097bbed: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14097bbf4: e8 77 6d 15 00              	callq	0x140ad2970 <.text+0xad1970>
14097bbf9: 84 c0                       	testb	%al, %al
14097bbfb: 4c 8b b5 88 04 00 00        	movq	0x488(%rbp), %r14
14097bc02: 44 8b bd 3c 03 00 00        	movl	0x33c(%rbp), %r15d
14097bc09: 88 85 b0 02 00 00           	movb	%al, 0x2b0(%rbp)
14097bc0f: 74 0c                       	je	0x14097bc1d <.text+0x97ac1d>
14097bc11: 44 8b bd 58 03 00 00        	movl	0x358(%rbp), %r15d
14097bc18: e9 e8 03 00 00              	jmp	0x14097c005 <.text+0x97b005>
14097bc1d: 48 8b 45 60                 	movq	0x60(%rbp), %rax
14097bc21: 8b 8d e8 00 00 00           	movl	0xe8(%rbp), %ecx
14097bc27: 89 08                       	movl	%ecx, (%rax)
14097bc29: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
14097bc30: 48 8b 71 30                 	movq	0x30(%rcx), %rsi
14097bc34: 48 8b 79 40                 	movq	0x40(%rcx), %rdi
14097bc38: 48 3b 71 20                 	cmpq	0x20(%rcx), %rsi
14097bc3c: 75 1e                       	jne	0x14097bc5c <.text+0x97ac5c>
14097bc3e: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
14097bc45: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14097bc49: 48 8d 15 f8 e4 2b 02        	leaq	0x22be4f8(%rip), %rdx   # 0x142c3a148
14097bc50: e8 1b 6c be ff              	callq	0x140562870 <.text+0x561870>
14097bc55: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
14097bc5c: 48 8b 41 28                 	movq	0x28(%rcx), %rax
14097bc60: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
14097bc64: 48 ff c6                    	incq	%rsi
14097bc67: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
14097bc6b: 48 ff 41 40                 	incq	0x40(%rcx)
14097bc6f: 48 8b 45 58                 	movq	0x58(%rbp), %rax
14097bc73: 8b 8d e4 00 00 00           	movl	0xe4(%rbp), %ecx
14097bc79: 89 08                       	movl	%ecx, (%rax)
14097bc7b: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
14097bc82: 48 8b 71 30                 	movq	0x30(%rcx), %rsi
14097bc86: 48 8b 79 40                 	movq	0x40(%rcx), %rdi
14097bc8a: 48 3b 71 20                 	cmpq	0x20(%rcx), %rsi
14097bc8e: 75 1e                       	jne	0x14097bcae <.text+0x97acae>
14097bc90: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
14097bc97: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14097bc9b: 48 8d 15 a6 e4 2b 02        	leaq	0x22be4a6(%rip), %rdx   # 0x142c3a148
14097bca2: e8 c9 6b be ff              	callq	0x140562870 <.text+0x561870>
14097bca7: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
14097bcae: 48 8b 41 28                 	movq	0x28(%rcx), %rax
14097bcb2: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
14097bcb6: 48 ff c6                    	incq	%rsi
14097bcb9: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
14097bcbd: 48 ff 41 40                 	incq	0x40(%rcx)
14097bcc1: 48 8b 95 30 01 00 00        	movq	0x130(%rbp), %rdx
14097bcc8: f6 02 01                    	testb	$0x1, (%rdx)
14097bccb: 0f 84 e2 00 00 00           	je	0x14097bdb3 <.text+0x97adb3>
14097bcd1: 8b 85 60 03 00 00           	movl	0x360(%rbp), %eax
14097bcd7: 89 85 e0 02 00 00           	movl	%eax, 0x2e0(%rbp)
14097bcdd: 44 89 a5 e4 02 00 00        	movl	%r12d, 0x2e4(%rbp)
14097bce4: 44 89 ad e8 02 00 00        	movl	%r13d, 0x2e8(%rbp)
14097bceb: 8b 8d 58 03 00 00           	movl	0x358(%rbp), %ecx
14097bcf1: 89 8d ec 02 00 00           	movl	%ecx, 0x2ec(%rbp)
14097bcf7: 8b 8d 10 03 00 00           	movl	0x310(%rbp), %ecx
14097bcfd: 89 8d f0 02 00 00           	movl	%ecx, 0x2f0(%rbp)
14097bd03: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
14097bd0a: 48 89 8d f4 02 00 00        	movq	%rcx, 0x2f4(%rbp)
14097bd11: 89 85 70 01 00 00           	movl	%eax, 0x170(%rbp)
14097bd17: 8b 85 4c 03 00 00           	movl	0x34c(%rbp), %eax
14097bd1d: 89 85 74 01 00 00           	movl	%eax, 0x174(%rbp)
14097bd23: 44 89 bd 78 01 00 00        	movl	%r15d, 0x178(%rbp)
14097bd2a: 8b 85 38 03 00 00           	movl	0x338(%rbp), %eax
14097bd30: 89 85 7c 01 00 00           	movl	%eax, 0x17c(%rbp)
14097bd36: 8b 85 34 03 00 00           	movl	0x334(%rbp), %eax
14097bd3c: 89 85 80 01 00 00           	movl	%eax, 0x180(%rbp)
14097bd42: 48 89 8d 84 01 00 00        	movq	%rcx, 0x184(%rbp)
14097bd49: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
14097bd50: 4c 8d 85 70 01 00 00        	leaq	0x170(%rbp), %r8
14097bd57: 4c 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %r9
14097bd5e: e8 7d 6f 15 00              	callq	0x140ad2ce0 <.text+0xad1ce0>
14097bd63: 8b 85 90 02 00 00           	movl	0x290(%rbp), %eax
14097bd69: 89 85 60 03 00 00           	movl	%eax, 0x360(%rbp)
14097bd6f: 8b 85 94 02 00 00           	movl	0x294(%rbp), %eax
14097bd75: 89 85 d8 02 00 00           	movl	%eax, 0x2d8(%rbp)
14097bd7b: 44 8b ad 98 02 00 00        	movl	0x298(%rbp), %r13d
14097bd82: 44 8b bd 9c 02 00 00        	movl	0x29c(%rbp), %r15d
14097bd89: 8b 85 a0 02 00 00           	movl	0x2a0(%rbp), %eax
14097bd8f: 89 85 10 03 00 00           	movl	%eax, 0x310(%rbp)
14097bd95: 48 8b 8d a4 02 00 00        	movq	0x2a4(%rbp), %rcx
14097bd9c: 48 89 8d 20 03 00 00        	movq	%rcx, 0x320(%rbp)
14097bda3: 48 c1 e9 20                 	shrq	$0x20, %rcx
14097bda7: 48 89 8d 70 02 00 00        	movq	%rcx, 0x270(%rbp)
14097bdae: e9 52 02 00 00              	jmp	0x14097c005 <.text+0x97b005>
14097bdb3: f6 85 60 03 00 00 01        	testb	$0x1, 0x360(%rbp)
14097bdba: 0f 84 f2 00 00 00           	je	0x14097beb2 <.text+0x97aeb2>
14097bdc0: 48 8d 72 04                 	leaq	0x4(%rdx), %rsi
14097bdc4: 8b 85 4c 03 00 00           	movl	0x34c(%rbp), %eax
14097bdca: 89 85 70 01 00 00           	movl	%eax, 0x170(%rbp)
14097bdd0: 44 89 bd 74 01 00 00        	movl	%r15d, 0x174(%rbp)
14097bdd7: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
14097bdde: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
14097bde5: f2 0f 10 42 04              	movsd	0x4(%rdx), %xmm0
14097bdea: f2 0f 11 85 78 01 00 00     	movsd	%xmm0, 0x178(%rbp)
14097bdf2: f2 0f 10 8d d0 02 00 00     	movsd	0x2d0(%rbp), %xmm1
14097bdfa: 44 0f 28 c1                 	movaps	%xmm1, %xmm8
14097bdfe: 44 0f 5c c0                 	subps	%xmm0, %xmm8
14097be02: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
14097be06: 41 0f 59 c0                 	mulps	%xmm8, %xmm0
14097be0a: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
14097be0e: f3 0f 58 d0                 	addss	%xmm0, %xmm2
14097be12: 0f 57 c0                    	xorps	%xmm0, %xmm0
14097be15: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
14097be19: f3 0f 10 15 e3 9a fa 01     	movss	0x1fa9ae3(%rip), %xmm2  # 0x142925904
14097be21: f3 0f 5e d0                 	divss	%xmm0, %xmm2
14097be25: f3 45 0f 16 c8              	movshdup	%xmm8, %xmm9            # xmm9 = xmm8[1,1,3,3]
14097be2a: f3 44 0f 59 c2              	mulss	%xmm2, %xmm8
14097be2f: f3 44 0f 59 ca              	mulss	%xmm2, %xmm9
14097be34: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14097be38: 48 8d bd 70 01 00 00        	leaq	0x170(%rbp), %rdi
14097be3f: 48 89 f9                    	movq	%rdi, %rcx
14097be42: e8 69 ee 31 00              	callq	0x140c9acb0 <.text+0xc99cb0>
14097be47: 48 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %rcx
14097be4e: 48 89 fa                    	movq	%rdi, %rdx
14097be51: 49 89 c0                    	movq	%rax, %r8
14097be54: e8 17 e6 31 00              	callq	0x140c9a470 <.text+0xc99470>
14097be59: f3 0f 10 95 e0 02 00 00     	movss	0x2e0(%rbp), %xmm2
14097be61: f3 0f 10 9d e4 02 00 00     	movss	0x2e4(%rbp), %xmm3
14097be69: f3 44 0f 10 95 e8 02 00 00  	movss	0x2e8(%rbp), %xmm10
14097be72: 66 44 0f 6e 9d ec 02 00 00  	movd	0x2ec(%rbp), %xmm11
14097be7b: f3 44 0f 11 4c 24 28        	movss	%xmm9, 0x28(%rsp)
14097be82: f3 44 0f 11 44 24 20        	movss	%xmm8, 0x20(%rsp)
14097be89: 48 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %rcx
14097be90: 48 89 f2                    	movq	%rsi, %rdx
14097be93: e8 e8 ba 31 00              	callq	0x140c97980 <.text+0xc96980>
14097be98: 80 bd e0 02 00 00 00        	cmpb	$0x0, 0x2e0(%rbp)
14097be9f: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
14097bea3: 0f 85 a5 00 00 00           	jne	0x14097bf4e <.text+0x97af4e>
14097bea9: 45 0f 57 f6                 	xorps	%xmm14, %xmm14
14097bead: e9 a5 00 00 00              	jmp	0x14097bf57 <.text+0x97af57>
14097beb2: f2 0f 10 95 d0 02 00 00     	movsd	0x2d0(%rbp), %xmm2
14097beba: f2 0f 10 42 04              	movsd	0x4(%rdx), %xmm0
14097bebf: 0f 5c d0                    	subps	%xmm0, %xmm2
14097bec2: 0f 28 ca                    	movaps	%xmm2, %xmm1
14097bec5: 0f 59 ca                    	mulps	%xmm2, %xmm1
14097bec8: f3 0f 16 d9                 	movshdup	%xmm1, %xmm3            # xmm3 = xmm1[1,1,3,3]
14097becc: f3 0f 58 d9                 	addss	%xmm1, %xmm3
14097bed0: 0f 57 c9                    	xorps	%xmm1, %xmm1
14097bed3: f3 0f 51 cb                 	sqrtss	%xmm3, %xmm1
14097bed7: f3 0f 10 1d 25 9a fa 01     	movss	0x1fa9a25(%rip), %xmm3  # 0x142925904
14097bedf: f3 0f 5e d9                 	divss	%xmm1, %xmm3
14097bee3: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
14097bee7: f3 0f 59 d3                 	mulss	%xmm3, %xmm2
14097beeb: f3 0f 59 cb                 	mulss	%xmm3, %xmm1
14097beef: f3 0f 10 62 0c              	movss	0xc(%rdx), %xmm4
14097bef4: f3 0f 5c e6                 	subss	%xmm6, %xmm4
14097bef8: f3 0f 59 d4                 	mulss	%xmm4, %xmm2
14097befc: f3 0f 59 e1                 	mulss	%xmm1, %xmm4
14097bf00: f3 0f 58 d0                 	addss	%xmm0, %xmm2
14097bf04: f3 0f 16 d8                 	movshdup	%xmm0, %xmm3            # xmm3 = xmm0[1,1,3,3]
14097bf08: f3 0f 58 dc                 	addss	%xmm4, %xmm3
14097bf0c: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
14097bf13: f3 0f 10 40 28              	movss	0x28(%rax), %xmm0
14097bf18: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
14097bf1e: c7 44 24 28 00 00 c8 42     	movl	$0x42c80000, 0x28(%rsp) # imm = 0x42C80000
14097bf26: 66 41 0f 6f c0              	movdqa	%xmm8, %xmm0
14097bf2b: 66 41 0f 6f c9              	movdqa	%xmm9, %xmm1
14097bf30: e8 3b 46 31 00              	callq	0x140c90570 <.text+0xc8f570>
14097bf35: 66 0f 7e 85 d8 02 00 00     	movd	%xmm0, 0x2d8(%rbp)
14097bf3d: 66 41 0f 7e cd              	movd	%xmm1, %r13d
14097bf42: 44 8b bd 58 03 00 00        	movl	0x358(%rbp), %r15d
14097bf49: e9 b7 00 00 00              	jmp	0x14097c005 <.text+0x97b005>
14097bf4e: f3 44 0f 10 b5 e8 02 00 00  	movss	0x2e8(%rbp), %xmm14
14097bf57: f3 44 0f 11 4c 24 28        	movss	%xmm9, 0x28(%rsp)
14097bf5e: f3 44 0f 11 44 24 20        	movss	%xmm8, 0x20(%rsp)
14097bf65: 48 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %rcx
14097bf6c: 48 89 f2                    	movq	%rsi, %rdx
14097bf6f: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
14097bf73: 66 41 0f 6f db              	movdqa	%xmm11, %xmm3
14097bf78: e8 03 ba 31 00              	callq	0x140c97980 <.text+0xc96980>
14097bf7d: 80 bd e0 02 00 00 00        	cmpb	$0x0, 0x2e0(%rbp)
14097bf84: 74 09                       	je	0x14097bf8f <.text+0x97af8f>
14097bf86: f3 44 0f 10 ad e8 02 00 00  	movss	0x2e8(%rbp), %xmm13
14097bf8f: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
14097bf96: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
14097bf9d: 41 0f 28 c6                 	movaps	%xmm14, %xmm0
14097bfa1: f3 41 0f c2 c6 03           	cmpunordss	%xmm14, %xmm0
14097bfa7: 0f 28 c8                    	movaps	%xmm0, %xmm1
14097bfaa: 41 0f 54 cd                 	andps	%xmm13, %xmm1
14097bfae: f3 45 0f 5d ee              	minss	%xmm14, %xmm13
14097bfb3: 41 0f 55 c5                 	andnps	%xmm13, %xmm0
14097bfb7: 0f 56 c1                    	orps	%xmm1, %xmm0
14097bfba: f3 44 0f 59 c0              	mulss	%xmm0, %xmm8
14097bfbf: f3 44 0f 58 40 04           	addss	0x4(%rax), %xmm8
14097bfc5: f3 44 0f 59 c8              	mulss	%xmm0, %xmm9
14097bfca: f3 44 0f 58 48 08           	addss	0x8(%rax), %xmm9
14097bfd0: f3 0f 10 41 28              	movss	0x28(%rcx), %xmm0
14097bfd5: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
14097bfdb: c7 44 24 28 00 00 c8 42     	movl	$0x42c80000, 0x28(%rsp) # imm = 0x42C80000
14097bfe3: 66 0f 6f c6                 	movdqa	%xmm6, %xmm0
14097bfe7: 66 0f 6f cf                 	movdqa	%xmm7, %xmm1
14097bfeb: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
14097bfef: 41 0f 28 d9                 	movaps	%xmm9, %xmm3
14097bff3: e8 78 45 31 00              	callq	0x140c90570 <.text+0xc8f570>
14097bff8: 66 41 0f 7e c7              	movd	%xmm0, %r15d
14097bffd: 66 0f 7e 8d 10 03 00 00     	movd	%xmm1, 0x310(%rbp)
14097c005: 49 8b 76 10                 	movq	0x10(%r14), %rsi
14097c009: 45 31 e4                    	xorl	%r12d, %r12d
14097c00c: b9 0c 00 00 00              	movl	$0xc, %ecx
14097c011: 48 89 f0                    	movq	%rsi, %rax
14097c014: 48 f7 e1                    	mulq	%rcx
14097c017: 48 89 c7                    	movq	%rax, %rdi
14097c01a: 0f 80 52 09 00 00           	jo	0x14097c972 <.text+0x97b972>
14097c020: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14097c02a: 48 39 c7                    	cmpq	%rax, %rdi
14097c02d: 0f 87 3f 09 00 00           	ja	0x14097c972 <.text+0x97b972>
14097c033: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
14097c03a: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
14097c041: 4d 8b 76 08                 	movq	0x8(%r14), %r14
14097c045: 48 85 ff                    	testq	%rdi, %rdi
14097c048: 74 2c                       	je	0x14097c076 <.text+0x97b076>
14097c04a: 0f b6 05 b8 8b cb 02        	movzbl	0x2cb8bb8(%rip), %eax   # 0x143634c09
14097c051: 41 bc 04 00 00 00           	movl	$0x4, %r12d
14097c057: ba 04 00 00 00              	movl	$0x4, %edx
14097c05c: 48 89 f9                    	movq	%rdi, %rcx
14097c05f: e8 ac 7b c9 ff              	callq	0x140613c10 <.text+0x612c10>
14097c064: 48 85 c0                    	testq	%rax, %rax
14097c067: 0f 84 05 09 00 00           	je	0x14097c972 <.text+0x97b972>
14097c06d: 48 89 b5 b8 02 00 00        	movq	%rsi, 0x2b8(%rbp)
14097c074: eb 10                       	jmp	0x14097c086 <.text+0x97b086>
14097c076: b8 04 00 00 00              	movl	$0x4, %eax
14097c07b: 48 c7 85 b8 02 00 00 00 00 00 00    	movq	$0x0, 0x2b8(%rbp)
14097c086: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
14097c08d: 48 89 c1                    	movq	%rax, %rcx
14097c090: 4c 89 f2                    	movq	%r14, %rdx
14097c093: 49 89 f8                    	movq	%rdi, %r8
14097c096: e8 65 7b fa 01              	callq	0x142923c00 <.text+0x2922c00>
14097c09b: f6 85 60 03 00 00 01        	testb	$0x1, 0x360(%rbp)
14097c0a2: 8b bd d8 02 00 00           	movl	0x2d8(%rbp), %edi
14097c0a8: 44 89 bd 58 03 00 00        	movl	%r15d, 0x358(%rbp)
14097c0af: 41 0f 45 ff                 	cmovnel	%r15d, %edi
14097c0b3: 44 89 ad 80 02 00 00        	movl	%r13d, 0x280(%rbp)
14097c0ba: 45 89 ee                    	movl	%r13d, %r14d
14097c0bd: 44 0f 45 b5 10 03 00 00     	cmovnel	0x310(%rbp), %r14d
14097c0c5: 48 8b 95 a8 04 00 00        	movq	0x4a8(%rbp), %rdx
14097c0cc: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14097c0cf: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14097c0d3: 89 01                       	movl	%eax, (%rcx)
14097c0d5: 4c 8b 22                    	movq	(%rdx), %r12
14097c0d8: 4d 8b 6c 24 30              	movq	0x30(%r12), %r13
14097c0dd: 4d 8b 7c 24 40              	movq	0x40(%r12), %r15
14097c0e2: 4d 3b 6c 24 20              	cmpq	0x20(%r12), %r13
14097c0e7: 75 11                       	jne	0x14097c0fa <.text+0x97b0fa>
14097c0e9: 49 8d 4c 24 20              	leaq	0x20(%r12), %rcx
14097c0ee: 48 8d 15 53 e0 2b 02        	leaq	0x22be053(%rip), %rdx   # 0x142c3a148
14097c0f5: e8 f6 68 be ff              	callq	0x1405629f0 <.text+0x5619f0>
14097c0fa: 4a 8d 04 6d 00 00 00 00     	leaq	(,%r13,2), %rax
14097c102: 4c 01 e8                    	addq	%r13, %rax
14097c105: 48 c1 e0 04                 	shlq	$0x4, %rax
14097c109: 49 8b 4c 24 28              	movq	0x28(%r12), %rcx
14097c10e: 48 8b 95 b8 02 00 00        	movq	0x2b8(%rbp), %rdx
14097c115: 48 89 14 01                 	movq	%rdx, (%rcx,%rax)
14097c119: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
14097c120: 48 89 54 01 08              	movq	%rdx, 0x8(%rcx,%rax)
14097c125: 48 89 74 01 10              	movq	%rsi, 0x10(%rcx,%rax)
14097c12a: 48 8b 95 28 01 00 00        	movq	0x128(%rbp), %rdx
14097c131: 48 89 54 01 18              	movq	%rdx, 0x18(%rcx,%rax)
14097c136: 89 7c 01 20                 	movl	%edi, 0x20(%rcx,%rax)
14097c13a: 44 89 74 01 24              	movl	%r14d, 0x24(%rcx,%rax)
14097c13f: 4c 89 7c 01 28              	movq	%r15, 0x28(%rcx,%rax)
14097c144: 49 ff c5                    	incq	%r13
14097c147: 4d 89 6c 24 30              	movq	%r13, 0x30(%r12)
14097c14c: 49 ff 44 24 40              	incq	0x40(%r12)
14097c151: 48 8b 85 88 02 00 00        	movq	0x288(%rbp), %rax
14097c158: f2 0f 10 70 e8              	movsd	-0x18(%rax), %xmm6
14097c15d: 66 0f 6e 78 f0              	movd	-0x10(%rax), %xmm7
14097c162: 66 44 0f 6e 40 f4           	movd	-0xc(%rax), %xmm8
14097c168: 0f b6 78 f8                 	movzbl	-0x8(%rax), %edi
14097c16c: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
14097c173: 8b 8d 7c 02 00 00           	movl	0x27c(%rbp), %ecx
14097c179: 89 08                       	movl	%ecx, (%rax)
14097c17b: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
14097c182: 48 89 85 88 02 00 00        	movq	%rax, 0x288(%rbp)
14097c189: f6 85 60 03 00 00 01        	testb	$0x1, 0x360(%rbp)
14097c190: 74 64                       	je	0x14097c1f6 <.text+0x97b1f6>
14097c192: 44 8b a5 d8 02 00 00        	movl	0x2d8(%rbp), %r12d
14097c199: 44 89 a5 70 01 00 00        	movl	%r12d, 0x170(%rbp)
14097c1a0: 44 8b bd 80 02 00 00        	movl	0x280(%rbp), %r15d
14097c1a7: 44 89 bd 74 01 00 00        	movl	%r15d, 0x174(%rbp)
14097c1ae: 8b b5 58 03 00 00           	movl	0x358(%rbp), %esi
14097c1b4: 89 b5 78 01 00 00           	movl	%esi, 0x178(%rbp)
14097c1ba: 44 8b ad 10 03 00 00        	movl	0x310(%rbp), %r13d
14097c1c1: 44 89 ad 7c 01 00 00        	movl	%r13d, 0x17c(%rbp)
14097c1c8: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
14097c1cf: 48 c1 e0 20                 	shlq	$0x20, %rax
14097c1d3: 44 8b b5 20 03 00 00        	movl	0x320(%rbp), %r14d
14097c1da: 49 09 c6                    	orq	%rax, %r14
14097c1dd: 4c 89 b5 80 01 00 00        	movq	%r14, 0x180(%rbp)
14097c1e4: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
14097c1e8: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097c1ef: e8 fc ca 31 00              	callq	0x140c98cf0 <.text+0xc97cf0>
14097c1f4: eb 63                       	jmp	0x14097c259 <.text+0x97b259>
14097c1f6: 44 8b a5 d8 02 00 00        	movl	0x2d8(%rbp), %r12d
14097c1fd: 44 89 a5 70 01 00 00        	movl	%r12d, 0x170(%rbp)
14097c204: 44 8b bd 80 02 00 00        	movl	0x280(%rbp), %r15d
14097c20b: 44 89 bd 74 01 00 00        	movl	%r15d, 0x174(%rbp)
14097c212: 8b b5 58 03 00 00           	movl	0x358(%rbp), %esi
14097c218: 89 b5 78 01 00 00           	movl	%esi, 0x178(%rbp)
14097c21e: 44 8b ad 10 03 00 00        	movl	0x310(%rbp), %r13d
14097c225: 44 89 ad 7c 01 00 00        	movl	%r13d, 0x17c(%rbp)
14097c22c: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
14097c230: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097c237: f3 0f 10 15 31 96 fa 01     	movss	0x1fa9631(%rip), %xmm2  # 0x142925870
14097c23f: e8 ac ba 31 00              	callq	0x140c97cf0 <.text+0xc96cf0>
14097c244: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
14097c24b: 48 c1 e0 20                 	shlq	$0x20, %rax
14097c24f: 44 8b b5 20 03 00 00        	movl	0x320(%rbp), %r14d
14097c256: 49 09 c6                    	orq	%rax, %r14
14097c259: 31 c0                       	xorl	%eax, %eax
14097c25b: 80 3d 2e 8b cb 02 00        	cmpb	$0x0, 0x2cb8b2e(%rip)   # 0x143634d90
14097c262: 0f 94 c0                    	sete	%al
14097c265: 8d 04 80                    	leal	(%rax,%rax,4), %eax
14097c268: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
14097c26c: 48 89 8d a0 01 00 00        	movq	%rcx, 0x1a0(%rbp)
14097c273: 0f 10 45 c8                 	movups	-0x38(%rbp), %xmm0
14097c277: 0f 10 4d d8                 	movups	-0x28(%rbp), %xmm1
14097c27b: 0f 10 55 e8                 	movups	-0x18(%rbp), %xmm2
14097c27f: 0f 29 95 90 01 00 00        	movaps	%xmm2, 0x190(%rbp)
14097c286: 0f 29 8d 80 01 00 00        	movaps	%xmm1, 0x180(%rbp)
14097c28d: 0f 29 85 70 01 00 00        	movaps	%xmm0, 0x170(%rbp)
14097c294: 8b 8d 60 03 00 00           	movl	0x360(%rbp), %ecx
14097c29a: 89 8d a8 01 00 00           	movl	%ecx, 0x1a8(%rbp)
14097c2a0: 44 89 a5 ac 01 00 00        	movl	%r12d, 0x1ac(%rbp)
14097c2a7: 44 89 bd b0 01 00 00        	movl	%r15d, 0x1b0(%rbp)
14097c2ae: 89 b5 b4 01 00 00           	movl	%esi, 0x1b4(%rbp)
14097c2b4: 44 89 ad b8 01 00 00        	movl	%r13d, 0x1b8(%rbp)
14097c2bb: 4c 89 b5 bc 01 00 00        	movq	%r14, 0x1bc(%rbp)
14097c2c2: f3 44 0f 11 a5 c4 01 00 00  	movss	%xmm12, 0x1c4(%rbp)
14097c2cb: 0f 13 b5 c8 01 00 00        	movlps	%xmm6, 0x1c8(%rbp)
14097c2d2: 66 0f 7e bd d0 01 00 00     	movd	%xmm7, 0x1d0(%rbp)
14097c2da: 66 44 0f 7e 85 d4 01 00 00  	movd	%xmm8, 0x1d4(%rbp)
14097c2e3: 40 88 bd d8 01 00 00        	movb	%dil, 0x1d8(%rbp)
14097c2ea: 88 85 d9 01 00 00           	movb	%al, 0x1d9(%rbp)
14097c2f0: 4c 8d 85 70 01 00 00        	leaq	0x170(%rbp), %r8
14097c2f7: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
14097c2fe: 48 8b 95 88 02 00 00        	movq	0x288(%rbp), %rdx
14097c305: e8 b6 d8 12 00              	callq	0x140aa9bc0 <.text+0xaa8bc0>
14097c30a: 80 bd b0 02 00 00 00        	cmpb	$0x0, 0x2b0(%rbp)
14097c311: 4c 8b 9d 00 01 00 00        	movq	0x100(%rbp), %r11
14097c318: 0f 85 61 f1 ff ff           	jne	0x14097b47f <.text+0x97a47f>
14097c31e: 0f 28 b5 70 03 00 00        	movaps	0x370(%rbp), %xmm6
14097c325: 0f 28 bd 80 03 00 00        	movaps	0x380(%rbp), %xmm7
14097c32c: 44 0f 28 85 90 03 00 00     	movaps	0x390(%rbp), %xmm8
14097c334: 44 0f 28 8d a0 03 00 00     	movaps	0x3a0(%rbp), %xmm9
14097c33c: 44 0f 28 95 b0 03 00 00     	movaps	0x3b0(%rbp), %xmm10
14097c344: 44 0f 28 9d c0 03 00 00     	movaps	0x3c0(%rbp), %xmm11
14097c34c: 44 0f 28 a5 d0 03 00 00     	movaps	0x3d0(%rbp), %xmm12
14097c354: 44 0f 28 ad e0 03 00 00     	movaps	0x3e0(%rbp), %xmm13
14097c35c: 44 0f 28 b5 f0 03 00 00     	movaps	0x3f0(%rbp), %xmm14
14097c364: 44 0f 28 bd 00 04 00 00     	movaps	0x400(%rbp), %xmm15
14097c36c: 48 81 c4 98 04 00 00        	addq	$0x498, %rsp            # imm = 0x498
14097c373: 5b                          	popq	%rbx
14097c374: 5f                          	popq	%rdi
14097c375: 5e                          	popq	%rsi
14097c376: 41 5c                       	popq	%r12
14097c378: 41 5d                       	popq	%r13
14097c37a: 41 5e                       	popq	%r14
14097c37c: 41 5f                       	popq	%r15
14097c37e: 5d                          	popq	%rbp
14097c37f: c3                          	retq
14097c380: 48 8d 0d 0c de 15 02        	leaq	0x215de0c(%rip), %rcx   # 0x142ada193
14097c387: 4c 8d 05 6a a8 13 02        	leaq	0x213a86a(%rip), %r8    # 0x142ab6bf8
14097c38e: ba 32 00 00 00              	movl	$0x32, %edx
14097c393: e8 98 d0 f5 01              	callq	0x1428d9430 <.text+0x28d8430>
14097c398: 48 8d 0d f4 dd 15 02        	leaq	0x215ddf4(%rip), %rcx   # 0x142ada193
14097c39f: 4c 8d 05 82 a8 13 02        	leaq	0x213a882(%rip), %r8    # 0x142ab6c28
14097c3a6: ba 32 00 00 00              	movl	$0x32, %edx
14097c3ab: e8 80 d0 f5 01              	callq	0x1428d9430 <.text+0x28d8430>
14097c3b0: 31 f6                       	xorl	%esi, %esi
14097c3b2: 48 6b f8 58                 	imulq	$0x58, %rax, %rdi
14097c3b6: f3 0f 10 8d d0 02 00 00     	movss	0x2d0(%rbp), %xmm1
14097c3be: f3 0f 10 95 d4 02 00 00     	movss	0x2d4(%rbp), %xmm2
14097c3c6: 4c 89 e9                    	movq	%r13, %rcx
14097c3c9: e8 72 d2 06 00              	callq	0x1409e9640 <.text+0x9e8640>
14097c3ce: 66 44 0f 6f d0              	movdqa	%xmm0, %xmm10
14097c3d3: 48 8b 85 88 02 00 00        	movq	0x288(%rbp), %rax
14097c3da: f3 44 0f 58 50 e4           	addss	-0x1c(%rax), %xmm10
14097c3e0: f3 0f 10 05 9c 95 fa 01     	movss	0x1fa959c(%rip), %xmm0  # 0x142925984
14097c3e8: 0f 2e 44 3e 30              	ucomiss	0x30(%rsi,%rdi), %xmm0
14097c3ed: 73 10                       	jae	0x14097c3ff <.text+0x97b3ff>
14097c3ef: 48 01 fe                    	addq	%rdi, %rsi
14097c3f2: 48 89 f1                    	movq	%rsi, %rcx
14097c3f5: e8 66 6e f6 ff              	callq	0x1408e3260 <.text+0x8e2260>
14097c3fa: f3 44 0f 58 d0              	addss	%xmm0, %xmm10
14097c3ff: 48 8d 4d 6c                 	leaq	0x6c(%rbp), %rcx
14097c403: 48 8d 95 d0 02 00 00        	leaq	0x2d0(%rbp), %rdx
14097c40a: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
14097c40e: e8 cd 09 32 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14097c413: f3 0f 10 45 6c              	movss	0x6c(%rbp), %xmm0
14097c418: f3 44 0f 10 6d 70           	movss	0x70(%rbp), %xmm13
14097c41e: f3 0f 10 4d 74              	movss	0x74(%rbp), %xmm1
14097c423: f3 0f 10 15 bd dc fd 01     	movss	0x1fddcbd(%rip), %xmm2  # 0x14295a0e8
14097c42b: f3 41 0f 58 d5              	addss	%xmm13, %xmm2
14097c430: f3 0f 11 85 90 00 00 00     	movss	%xmm0, 0x90(%rbp)
14097c438: f3 0f 11 95 94 00 00 00     	movss	%xmm2, 0x94(%rbp)
14097c440: f3 0f 11 8d 98 00 00 00     	movss	%xmm1, 0x98(%rbp)
14097c448: f2 0f 10 05 b0 18 06 02     	movsd	0x20618b0(%rip), %xmm0  # 0x1429ddd00
14097c450: f2 0f 11 85 9c 00 00 00     	movsd	%xmm0, 0x9c(%rbp)
14097c458: c7 85 a4 00 00 00 00 00 00 80       	movl	$0x80000000, 0xa4(%rbp) # imm = 0x80000000
14097c462: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
14097c469: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14097c46e: c7 44 24 30 02 02 00 00     	movl	$0x202, 0x30(%rsp)      # imm = 0x202
14097c476: c7 44 24 28 ff ff 7f 7f     	movl	$0x7f7fffff, 0x28(%rsp) # imm = 0x7F7FFFFF
14097c47e: 48 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %rcx
14097c485: 4c 8d 85 90 00 00 00        	leaq	0x90(%rbp), %r8
14097c48c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14097c492: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
14097c496: e8 e5 1c f5 ff              	callq	0x1408ce180 <.text+0x8cd180>
14097c49b: 0f b6 85 e0 02 00 00        	movzbl	0x2e0(%rbp), %eax
14097c4a2: 3c 11                       	cmpb	$0x11, %al
14097c4a4: 0f 84 3a 02 00 00           	je	0x14097c6e4 <.text+0x97b6e4>
14097c4aa: fe c8                       	decb	%al
14097c4ac: 3c 02                       	cmpb	$0x2, %al
14097c4ae: 0f 83 30 02 00 00           	jae	0x14097c6e4 <.text+0x97b6e4>
14097c4b4: 66 44 0f 6e 9d f8 02 00 00  	movd	0x2f8(%rbp), %xmm11
14097c4bd: f3 44 0f 10 b5 00 03 00 00  	movss	0x300(%rbp), %xmm14
14097c4c6: 48 8b b5 e8 02 00 00        	movq	0x2e8(%rbp), %rsi
14097c4cd: 4c 8d 05 14 a8 13 02        	leaq	0x213a814(%rip), %r8    # 0x142ab6ce8
14097c4d4: 4c 89 e1                    	movq	%r12, %rcx
14097c4d7: 48 89 f2                    	movq	%rsi, %rdx
14097c4da: e8 11 5a 11 00              	callq	0x140a91ef0 <.text+0xa90ef0>
14097c4df: 48 89 c7                    	movq	%rax, %rdi
14097c4e2: 4c 89 e1                    	movq	%r12, %rcx
14097c4e5: 48 89 f2                    	movq	%rsi, %rdx
14097c4e8: e8 c3 5d 11 00              	callq	0x140a922b0 <.text+0xa912b0>
14097c4ed: 48 89 c1                    	movq	%rax, %rcx
14097c4f0: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
14097c4f7: 4c 8b 85 28 01 00 00        	movq	0x128(%rbp), %r8
14097c4fe: e8 ed a0 ff ff              	callq	0x1409765f0 <.text+0x9755f0>
14097c503: 48 89 c6                    	movq	%rax, %rsi
14097c506: 48 85 ff                    	testq	%rdi, %rdi
14097c509: 0f 94 c0                    	sete	%al
14097c50c: 48 85 f6                    	testq	%rsi, %rsi
14097c50f: 0f 94 c1                    	sete	%cl
14097c512: 08 c1                       	orb	%al, %cl
14097c514: 0f 85 ca 01 00 00           	jne	0x14097c6e4 <.text+0x97b6e4>
14097c51a: f3 44 0f 10 7e 4c           	movss	0x4c(%rsi), %xmm15
14097c520: 48 89 f1                    	movq	%rsi, %rcx
14097c523: e8 38 6d f6 ff              	callq	0x1408e3260 <.text+0x8e2260>
14097c528: 44 0f 28 d0                 	movaps	%xmm0, %xmm10
14097c52c: f3 45 0f 58 d7              	addss	%xmm15, %xmm10
14097c531: f3 0f 10 87 80 02 00 00     	movss	0x280(%rdi), %xmm0
14097c539: f3 0f 11 85 88 00 00 00     	movss	%xmm0, 0x88(%rbp)
14097c541: f3 44 0f 11 95 8c 00 00 00  	movss	%xmm10, 0x8c(%rbp)
14097c54a: 48 8b 85 88 02 00 00        	movq	0x288(%rbp), %rax
14097c551: 66 0f 6e 40 e8              	movd	-0x18(%rax), %xmm0
14097c556: 66 0f 7e 85 80 00 00 00     	movd	%xmm0, 0x80(%rbp)
14097c55e: f3 44 0f 11 ad 84 00 00 00  	movss	%xmm13, 0x84(%rbp)
14097c567: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
14097c56e: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
14097c575: e8 16 66 de 00              	callq	0x141762b90 <.text+0x1761b90>
14097c57a: 84 c0                       	testb	%al, %al
14097c57c: 0f 84 62 01 00 00           	je	0x14097c6e4 <.text+0x97b6e4>
14097c582: 66 44 0f 7e 5d 78           	movd	%xmm11, 0x78(%rbp)
14097c588: f3 44 0f 11 75 7c           	movss	%xmm14, 0x7c(%rbp)
14097c58e: 48 8d bd 70 01 00 00        	leaq	0x170(%rbp), %rdi
14097c595: 48 8d 55 78                 	leaq	0x78(%rbp), %rdx
14097c599: 48 89 f9                    	movq	%rdi, %rcx
14097c59c: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
14097c5a0: e8 3b 08 32 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14097c5a5: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
14097c5ac: f3 44 0f 10 15 4f 93 fa 01  	movss	0x1fa934f(%rip), %xmm10 # 0x142925904
14097c5b5: 48 89 f2                    	movq	%rsi, %rdx
14097c5b8: 49 89 f8                    	movq	%rdi, %r8
14097c5bb: 41 0f 28 da                 	movaps	%xmm10, %xmm3
14097c5bf: e8 ec 47 00 00              	callq	0x140980db0 <.text+0x97fdb0>
14097c5c4: 8b 85 90 02 00 00           	movl	0x290(%rbp), %eax
14097c5ca: 85 c0                       	testl	%eax, %eax
14097c5cc: 0f 84 12 01 00 00           	je	0x14097c6e4 <.text+0x97b6e4>
14097c5d2: 83 f8 01                    	cmpl	$0x1, %eax
14097c5d5: 0f 85 1e 01 00 00           	jne	0x14097c6f9 <.text+0x97b6f9>
14097c5db: f3 44 0f 10 95 94 02 00 00  	movss	0x294(%rbp), %xmm10
14097c5e4: f3 44 0f 10 b5 98 02 00 00  	movss	0x298(%rbp), %xmm14
14097c5ed: f3 44 0f 10 9d 9c 02 00 00  	movss	0x29c(%rbp), %xmm11
14097c5f6: f3 44 0f 11 95 d0 02 00 00  	movss	%xmm10, 0x2d0(%rbp)
14097c5ff: f3 44 0f 11 9d d4 02 00 00  	movss	%xmm11, 0x2d4(%rbp)
14097c608: e8 93 a0 ff ff              	callq	0x1409766a0 <.text+0x9756a0>
14097c60d: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
14097c614: f3 44 0f 11 95 20 01 00 00  	movss	%xmm10, 0x120(%rbp)
14097c61d: f3 44 0f 11 9d 24 01 00 00  	movss	%xmm11, 0x124(%rbp)
14097c626: 41 0f 28 d6                 	movaps	%xmm14, %xmm2
14097c62a: f3 41 0f c2 d6 03           	cmpunordss	%xmm14, %xmm2
14097c630: 0f 28 c2                    	movaps	%xmm2, %xmm0
14097c633: 41 0f 54 c5                 	andps	%xmm13, %xmm0
14097c637: f3 45 0f 5f ee              	maxss	%xmm14, %xmm13
14097c63c: 41 0f 55 d5                 	andnps	%xmm13, %xmm2
14097c640: 0f 56 d0                    	orps	%xmm0, %xmm2
14097c643: 48 8d 8d d4 00 00 00        	leaq	0xd4(%rbp), %rcx
14097c64a: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
14097c651: e8 8a 07 32 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14097c656: f3 44 0f 11 95 40 01 00 00  	movss	%xmm10, 0x140(%rbp)
14097c65f: f3 44 0f 11 9d 44 01 00 00  	movss	%xmm11, 0x144(%rbp)
14097c668: f3 0f 10 95 c4 02 00 00     	movss	0x2c4(%rbp), %xmm2
14097c670: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14097c677: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
14097c67e: e8 5d 07 32 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14097c683: 8b 85 dc 00 00 00           	movl	0xdc(%rbp), %eax
14097c689: 48 8b 8d 10 03 00 00        	movq	0x310(%rbp), %rcx
14097c690: 89 41 08                    	movl	%eax, 0x8(%rcx)
14097c693: 48 8b 85 d4 00 00 00        	movq	0xd4(%rbp), %rax
14097c69a: 48 89 01                    	movq	%rax, (%rcx)
14097c69d: 8b 85 78 01 00 00           	movl	0x178(%rbp), %eax
14097c6a3: 89 41 14                    	movl	%eax, 0x14(%rcx)
14097c6a6: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14097c6ad: 48 89 41 0c                 	movq	%rax, 0xc(%rcx)
14097c6b1: 48 c7 85 70 01 00 00 02 00 00 00    	movq	$0x2, 0x170(%rbp)
14097c6bc: 48 89 8d 78 01 00 00        	movq	%rcx, 0x178(%rbp)
14097c6c3: 48 c7 85 80 01 00 00 02 00 00 00    	movq	$0x2, 0x180(%rbp)
14097c6ce: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14097c6d8: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
14097c6df: e9 e1 01 00 00              	jmp	0x14097c8c5 <.text+0x97b8c5>
14097c6e4: 31 f6                       	xorl	%esi, %esi
14097c6e6: 4c 8b bd 88 04 00 00        	movq	0x488(%rbp), %r15
14097c6ed: 48 8b bd 28 03 00 00        	movq	0x328(%rbp), %rdi
14097c6f4: e9 53 f2 ff ff              	jmp	0x14097b94c <.text+0x97a94c>
14097c6f9: f3 44 0f 10 9d 94 02 00 00  	movss	0x294(%rbp), %xmm11
14097c702: e8 99 9f ff ff              	callq	0x1409766a0 <.text+0x9756a0>
14097c707: 44 0f 10 b5 98 02 00 00     	movups	0x298(%rbp), %xmm14
14097c70f: f2 0f 10 05 d9 d7 10 02     	movsd	0x210d7d9(%rip), %xmm0  # 0x142a89ef0
14097c717: 41 0f 58 c6                 	addps	%xmm14, %xmm0
14097c71b: f2 0f 10 8d a0 02 00 00     	movsd	0x2a0(%rbp), %xmm1
14097c723: 0f 29 8d 10 03 00 00        	movaps	%xmm1, 0x310(%rbp)
14097c72a: 0f 11 00                    	movups	%xmm0, (%rax)
14097c72d: f2 44 0f 10 bd a8 02 00 00  	movsd	0x2a8(%rbp), %xmm15
14097c736: f3 0f 10 05 2a 09 05 02     	movss	0x205092a(%rip), %xmm0  # 0x1429cd068
14097c73e: 41 0f 58 c7                 	addps	%xmm15, %xmm0
14097c742: 0f 13 40 10                 	movlps	%xmm0, 0x10(%rax)
14097c746: 48 c7 85 70 01 00 00 02 00 00 00    	movq	$0x2, 0x170(%rbp)
14097c751: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14097c758: 48 c7 85 80 01 00 00 02 00 00 00    	movq	$0x2, 0x180(%rbp)
14097c763: 48 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rsi # imm = 0x8000000000000000
14097c76d: 48 89 b5 88 01 00 00        	movq	%rsi, 0x188(%rbp)
14097c774: c7 85 a0 01 00 00 00 00 00 00       	movl	$0x0, 0x1a0(%rbp)
14097c77e: c7 85 b0 01 00 00 00 00 40 40       	movl	$0x40400000, 0x1b0(%rbp) # imm = 0x40400000
14097c788: 66 c7 85 b4 01 00 00 01 01  	movw	$0x101, 0x1b4(%rbp)     # imm = 0x101
14097c791: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14097c798: 48 8b 95 58 03 00 00        	movq	0x358(%rbp), %rdx
14097c79f: e8 cc 87 18 00              	callq	0x140b04f70 <.text+0xb03f70>
14097c7a4: 41 0f 28 cb                 	movaps	%xmm11, %xmm1
14097c7a8: f3 45 0f 5c d3              	subss	%xmm11, %xmm10
14097c7ad: f3 41 0f 16 c6              	movshdup	%xmm14, %xmm0           # xmm0 = xmm14[1,1,3,3]
14097c7b2: f3 41 0f 59 c2              	mulss	%xmm10, %xmm0
14097c7b7: 45 0f 28 df                 	movaps	%xmm15, %xmm11
14097c7bb: f3 44 0f 59 d9              	mulss	%xmm1, %xmm11
14097c7c0: f3 44 0f 58 d8              	addss	%xmm0, %xmm11
14097c7c5: 45 0f c6 f6 e8              	shufps	$0xe8, %xmm14, %xmm14   # xmm14 = xmm14[0,2,2,3]
14097c7ca: f3 41 0f 12 c2              	movsldup	%xmm10, %xmm0           # xmm0 = xmm10[0,0,2,2]
14097c7cf: 41 0f 59 c6                 	mulps	%xmm14, %xmm0
14097c7d3: 0f 28 95 10 03 00 00        	movaps	0x310(%rbp), %xmm2
14097c7da: 44 0f c6 fa 11              	shufps	$0x11, %xmm2, %xmm15    # xmm15 = xmm15[1,0],xmm2[1,0]
14097c7df: 44 0f c6 fa 42              	shufps	$0x42, %xmm2, %xmm15    # xmm15 = xmm15[2,0],xmm2[0,1]
14097c7e4: f3 44 0f 12 d1              	movsldup	%xmm1, %xmm10           # xmm10 = xmm1[0,0,2,2]
14097c7e9: 45 0f 59 d7                 	mulps	%xmm15, %xmm10
14097c7ed: 44 0f 58 d0                 	addps	%xmm0, %xmm10
14097c7f1: 44 0f 13 95 d0 02 00 00     	movlps	%xmm10, 0x2d0(%rbp)
14097c7f9: e8 a2 9e ff ff              	callq	0x1409766a0 <.text+0x9756a0>
14097c7fe: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
14097c805: 44 0f 13 95 20 01 00 00     	movlps	%xmm10, 0x120(%rbp)
14097c80d: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
14097c811: f3 41 0f 5f c3              	maxss	%xmm11, %xmm0
14097c816: f3 45 0f c2 db 03           	cmpunordss	%xmm11, %xmm11
14097c81c: 41 0f 28 cb                 	movaps	%xmm11, %xmm1
14097c820: 0f 55 c8                    	andnps	%xmm0, %xmm1
14097c823: 45 0f 54 dd                 	andps	%xmm13, %xmm11
14097c827: 44 0f 56 d9                 	orps	%xmm1, %xmm11
14097c82b: 48 8d 8d d4 00 00 00        	leaq	0xd4(%rbp), %rcx
14097c832: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
14097c839: 41 0f 28 d3                 	movaps	%xmm11, %xmm2
14097c83d: e8 9e 05 32 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14097c842: f3 44 0f 11 95 40 01 00 00  	movss	%xmm10, 0x140(%rbp)
14097c84b: f3 41 0f 16 c2              	movshdup	%xmm10, %xmm0           # xmm0 = xmm10[1,1,3,3]
14097c850: f3 0f 11 85 44 01 00 00     	movss	%xmm0, 0x144(%rbp)
14097c858: f3 0f 10 95 c4 02 00 00     	movss	0x2c4(%rbp), %xmm2
14097c860: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14097c867: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
14097c86e: e8 6d 05 32 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14097c873: 8b 85 dc 00 00 00           	movl	0xdc(%rbp), %eax
14097c879: 48 8b 8d 10 03 00 00        	movq	0x310(%rbp), %rcx
14097c880: 89 41 08                    	movl	%eax, 0x8(%rcx)
14097c883: 48 8b 85 d4 00 00 00        	movq	0xd4(%rbp), %rax
14097c88a: 48 89 01                    	movq	%rax, (%rcx)
14097c88d: 8b 85 78 01 00 00           	movl	0x178(%rbp), %eax
14097c893: 89 41 14                    	movl	%eax, 0x14(%rcx)
14097c896: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14097c89d: 48 89 41 0c                 	movq	%rax, 0xc(%rcx)
14097c8a1: 48 c7 85 70 01 00 00 02 00 00 00    	movq	$0x2, 0x170(%rbp)
14097c8ac: 48 89 8d 78 01 00 00        	movq	%rcx, 0x178(%rbp)
14097c8b3: 48 c7 85 80 01 00 00 02 00 00 00    	movq	$0x2, 0x180(%rbp)
14097c8be: 48 89 b5 88 01 00 00        	movq	%rsi, 0x188(%rbp)
14097c8c5: c7 85 a0 01 00 00 00 00 00 00       	movl	$0x0, 0x1a0(%rbp)
14097c8cf: c7 85 b0 01 00 00 00 00 40 40       	movl	$0x40400000, 0x1b0(%rbp) # imm = 0x40400000
14097c8d9: 66 c7 85 b4 01 00 00 01 01  	movw	$0x101, 0x1b4(%rbp)     # imm = 0x101
14097c8e2: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14097c8e9: 48 8b 95 58 03 00 00        	movq	0x358(%rbp), %rdx
14097c8f0: e8 7b 86 18 00              	callq	0x140b04f70 <.text+0xb03f70>
14097c8f5: 40 b6 01                    	movb	$0x1, %sil
14097c8f8: 48 8b 85 88 04 00 00        	movq	0x488(%rbp), %rax
14097c8ff: 80 78 40 00                 	cmpb	$0x0, 0x40(%rax)
14097c903: 0f 85 dd fd ff ff           	jne	0x14097c6e6 <.text+0x97b6e6>
14097c909: c6 85 70 01 00 00 03        	movb	$0x3, 0x170(%rbp)
14097c910: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
14097c917: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14097c91e: 8b 85 c8 02 00 00           	movl	0x2c8(%rbp), %eax
14097c924: 89 85 80 01 00 00           	movl	%eax, 0x180(%rbp)
14097c92a: 4c 8d 05 cf a3 13 02        	leaq	0x213a3cf(%rip), %r8    # 0x142ab6d00
14097c931: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14097c935: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14097c93c: e8 bf a9 9a 00              	callq	0x141327300 <.text+0x1326300>
14097c941: e9 a0 fd ff ff              	jmp	0x14097c6e6 <.text+0x97b6e6>
14097c946: 48 8d 0d f4 9e 13 02        	leaq	0x2139ef4(%rip), %rcx   # 0x142ab6841
14097c94d: 4c 8d 05 64 a3 13 02        	leaq	0x213a364(%rip), %r8    # 0x142ab6cb8
14097c954: ba 28 00 00 00              	movl	$0x28, %edx
14097c959: e8 d2 ca f5 01              	callq	0x1428d9430 <.text+0x28d8430>
14097c95e: 0f 0b                       	ud2
14097c960: 4c 8d 05 11 c2 2b 02        	leaq	0x22bc211(%rip), %r8    # 0x142c38b78
14097c967: 4c 89 f1                    	movq	%r14, %rcx
14097c96a: 48 89 fa                    	movq	%rdi, %rdx
14097c96d: e8 71 c6 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14097c972: 4c 8d 05 ff c1 2b 02        	leaq	0x22bc1ff(%rip), %r8    # 0x142c38b78
14097c979: 4c 89 e1                    	movq	%r12, %rcx
14097c97c: 48 89 fa                    	movq	%rdi, %rdx
14097c97f: e8 5f c6 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14097c984: 48 8d 05 15 e5 16 02        	leaq	0x216e515(%rip), %rax   # 0x142aeaea0
14097c98b: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
14097c992: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
14097c99d: 48 c7 85 80 01 00 00 08 00 00 00    	movq	$0x8, 0x180(%rbp)
14097c9a8: 66 0f ef c0                 	pxor	%xmm0, %xmm0
14097c9ac: f3 0f 7f 85 88 01 00 00     	movdqu	%xmm0, 0x188(%rbp)
14097c9b4: 48 8d 15 f5 e4 16 02        	leaq	0x216e4f5(%rip), %rdx   # 0x142aeaeb0
14097c9bb: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14097c9c2: e8 c9 c9 f5 01              	callq	0x1428d9390 <.text+0x28d8390>
14097c9c7: cc                          	int3
14097c9c8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14097c9d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097c9d5: 55                          	pushq	%rbp
14097c9d6: 41 57                       	pushq	%r15
14097c9d8: 41 56                       	pushq	%r14
14097c9da: 41 55                       	pushq	%r13
14097c9dc: 41 54                       	pushq	%r12
14097c9de: 56                          	pushq	%rsi
14097c9df: 57                          	pushq	%rdi
14097c9e0: 53                          	pushq	%rbx
14097c9e1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
14097c9e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097c9ef: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
14097c9f5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
14097c9fb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
14097ca01: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
14097ca07: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
14097ca10: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
14097ca19: 66 44 0f 7f 8c 24 a0 00 00 00       	movdqa	%xmm9, 0xa0(%rsp)
14097ca23: 66 44 0f 7f 84 24 b0 00 00 00       	movdqa	%xmm8, 0xb0(%rsp)
14097ca2d: 66 0f 7f bc 24 c0 00 00 00  	movdqa	%xmm7, 0xc0(%rsp)
14097ca36: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
14097ca3f: ba 18 00 00 00              	movl	$0x18, %edx
14097ca44: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14097ca4a: 48 8b 8d 10 03 00 00        	movq	0x310(%rbp), %rcx
14097ca51: e8 ca 71 c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097ca56: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
14097ca5e: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
14097ca66: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
14097ca6f: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
14097ca78: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
14097ca81: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
14097ca8a: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
14097ca90: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
14097ca96: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
14097ca9c: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
14097caa2: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
14097caa9: 5b                          	popq	%rbx
14097caaa: 5f                          	popq	%rdi
14097caab: 5e                          	popq	%rsi
14097caac: 41 5c                       	popq	%r12
14097caae: 41 5d                       	popq	%r13
14097cab0: 41 5e                       	popq	%r14
14097cab2: 41 5f                       	popq	%r15
14097cab4: 5d                          	popq	%rbp
14097cab5: c3                          	retq
14097cab6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14097cac0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097cac5: 55                          	pushq	%rbp
14097cac6: 41 57                       	pushq	%r15
14097cac8: 41 56                       	pushq	%r14
14097caca: 41 55                       	pushq	%r13
14097cacc: 41 54                       	pushq	%r12
14097cace: 56                          	pushq	%rsi
14097cacf: 57                          	pushq	%rdi
14097cad0: 53                          	pushq	%rbx
14097cad1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
14097cad8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097cadf: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
14097cae5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
14097caeb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
14097caf1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
14097caf7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
14097cb00: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
14097cb09: 66 44 0f 7f 8c 24 a0 00 00 00       	movdqa	%xmm9, 0xa0(%rsp)
14097cb13: 66 44 0f 7f 84 24 b0 00 00 00       	movdqa	%xmm8, 0xb0(%rsp)
14097cb1d: 66 0f 7f bc 24 c0 00 00 00  	movdqa	%xmm7, 0xc0(%rsp)
14097cb26: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
14097cb2f: ba 18 00 00 00              	movl	$0x18, %edx
14097cb34: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14097cb3a: 48 8b 8d 10 03 00 00        	movq	0x310(%rbp), %rcx
14097cb41: e8 da 70 c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097cb46: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
14097cb4e: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
14097cb56: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
14097cb5f: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
14097cb68: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
14097cb71: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
14097cb7a: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
14097cb80: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
14097cb86: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
14097cb8c: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
14097cb92: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
14097cb99: 5b                          	popq	%rbx
14097cb9a: 5f                          	popq	%rdi
14097cb9b: 5e                          	popq	%rsi
14097cb9c: 41 5c                       	popq	%r12
14097cb9e: 41 5d                       	popq	%r13
14097cba0: 41 5e                       	popq	%r14
14097cba2: 41 5f                       	popq	%r15
14097cba4: 5d                          	popq	%rbp
14097cba5: c3                          	retq
14097cba6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14097cbb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097cbb5: 55                          	pushq	%rbp
14097cbb6: 41 57                       	pushq	%r15
14097cbb8: 41 56                       	pushq	%r14
14097cbba: 41 55                       	pushq	%r13
14097cbbc: 41 54                       	pushq	%r12
14097cbbe: 56                          	pushq	%rsi
14097cbbf: 57                          	pushq	%rdi
14097cbc0: 53                          	pushq	%rbx
14097cbc1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
14097cbc8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097cbcf: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
14097cbd5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
14097cbdb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
14097cbe1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
14097cbe7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
14097cbf0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
14097cbf9: 66 44 0f 7f 8c 24 a0 00 00 00       	movdqa	%xmm9, 0xa0(%rsp)
14097cc03: 66 44 0f 7f 84 24 b0 00 00 00       	movdqa	%xmm8, 0xb0(%rsp)
14097cc0d: 66 0f 7f bc 24 c0 00 00 00  	movdqa	%xmm7, 0xc0(%rsp)
14097cc16: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
14097cc1e: c6 85 67 03 00 00 00        	movb	$0x0, 0x367(%rbp)
14097cc25: 48 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %rcx
14097cc2c: e8 3f b8 f1 ff              	callq	0x140898470 <.text+0x897470>
14097cc31: c6 85 67 03 00 00 00        	movb	$0x0, 0x367(%rbp)
14097cc38: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
14097cc40: 66 0f 6f bc 24 c0 00 00 00  	movdqa	0xc0(%rsp), %xmm7
14097cc49: 66 44 0f 6f 84 24 b0 00 00 00       	movdqa	0xb0(%rsp), %xmm8
14097cc53: 66 44 0f 6f 8c 24 a0 00 00 00       	movdqa	0xa0(%rsp), %xmm9
14097cc5d: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
14097cc66: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
14097cc6f: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
14097cc75: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
14097cc7b: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
14097cc81: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
14097cc87: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
14097cc8e: 5b                          	popq	%rbx
14097cc8f: 5f                          	popq	%rdi
14097cc90: 5e                          	popq	%rsi
14097cc91: 41 5c                       	popq	%r12
14097cc93: 41 5d                       	popq	%r13
14097cc95: 41 5e                       	popq	%r14
14097cc97: 41 5f                       	popq	%r15
14097cc99: 5d                          	popq	%rbp
14097cc9a: c3                          	retq
14097cc9b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14097cca0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097cca5: 55                          	pushq	%rbp
14097cca6: 41 57                       	pushq	%r15
14097cca8: 41 56                       	pushq	%r14
14097ccaa: 41 55                       	pushq	%r13
14097ccac: 41 54                       	pushq	%r12
14097ccae: 56                          	pushq	%rsi
14097ccaf: 57                          	pushq	%rdi
14097ccb0: 53                          	pushq	%rbx
14097ccb1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
14097ccb8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097ccbf: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
14097ccc5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
14097cccb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
14097ccd1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
14097ccd7: 66 44 0f 7f 9c 24 80 00 00 00       	movdqa	%xmm11, 0x80(%rsp)
14097cce1: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
14097ccea: 66 44 0f 7f 8c 24 a0 00 00 00       	movdqa	%xmm9, 0xa0(%rsp)
14097ccf4: 66 44 0f 7f 84 24 b0 00 00 00       	movdqa	%xmm8, 0xb0(%rsp)
14097ccfe: 66 0f 7f bc 24 c0 00 00 00  	movdqa	%xmm7, 0xc0(%rsp)
14097cd07: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
14097cd0f: 48 83 bd b8 02 00 00 00     	cmpq	$0x0, 0x2b8(%rbp)
14097cd17: 74 21                       	je	0x14097cd3a <.text+0x97bd3a>
14097cd19: 48 8b 85 b8 02 00 00        	movq	0x2b8(%rbp), %rax
14097cd20: 48 c1 e0 02                 	shlq	$0x2, %rax
14097cd24: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14097cd28: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14097cd2e: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
14097cd35: e8 e6 6e c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097cd3a: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
14097cd42: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
14097cd4a: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
14097cd53: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
14097cd5c: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
14097cd65: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
14097cd6e: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
14097cd74: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
14097cd7a: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
14097cd80: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
14097cd86: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
14097cd8d: 5b                          	popq	%rbx
14097cd8e: 5f                          	popq	%rdi
14097cd8f: 5e                          	popq	%rsi
14097cd90: 41 5c                       	popq	%r12
14097cd92: 41 5d                       	popq	%r13
14097cd94: 41 5e                       	popq	%r14
14097cd96: 41 5f                       	popq	%r15
14097cd98: 5d                          	popq	%rbp
14097cd99: c3                          	retq
14097cd9a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14097cda0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097cda5: 55                          	pushq	%rbp
14097cda6: 41 57                       	pushq	%r15
14097cda8: 41 56                       	pushq	%r14
14097cdaa: 41 55                       	pushq	%r13
14097cdac: 41 54                       	pushq	%r12
14097cdae: 56                          	pushq	%rsi
14097cdaf: 57                          	pushq	%rdi
14097cdb0: 53                          	pushq	%rbx
14097cdb1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
14097cdb8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097cdbf: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
14097cdc5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
14097cdcb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
14097cdd1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
14097cdd7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
14097cde0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
14097cde9: 66 44 0f 7f 8c 24 a0 00 00 00       	movdqa	%xmm9, 0xa0(%rsp)
14097cdf3: 66 44 0f 7f 84 24 b0 00 00 00       	movdqa	%xmm8, 0xb0(%rsp)
14097cdfd: 66 0f 7f bc 24 c0 00 00 00  	movdqa	%xmm7, 0xc0(%rsp)
14097ce06: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
14097ce0e: 80 bd 67 03 00 00 00        	cmpb	$0x0, 0x367(%rbp)
14097ce15: 75 60                       	jne	0x14097ce77 <.text+0x97be77>
14097ce17: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
14097ce1f: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
14097ce27: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
14097ce30: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
14097ce39: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
14097ce42: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
14097ce4b: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
14097ce51: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
14097ce57: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
14097ce5d: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
14097ce63: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
14097ce6a: 5b                          	popq	%rbx
14097ce6b: 5f                          	popq	%rdi
14097ce6c: 5e                          	popq	%rsi
14097ce6d: 41 5c                       	popq	%r12
14097ce6f: 41 5d                       	popq	%r13
14097ce71: 41 5e                       	popq	%r14
14097ce73: 41 5f                       	popq	%r15
14097ce75: 5d                          	popq	%rbp
14097ce76: c3                          	retq
14097ce77: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
14097ce7e: e8 ed b5 f1 ff              	callq	0x140898470 <.text+0x897470>
14097ce83: eb 92                       	jmp	0x14097ce17 <.text+0x97be17>
14097ce85: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14097ce90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097ce95: 55                          	pushq	%rbp
14097ce96: 41 57                       	pushq	%r15
14097ce98: 41 56                       	pushq	%r14
14097ce9a: 41 55                       	pushq	%r13
14097ce9c: 41 54                       	pushq	%r12
14097ce9e: 56                          	pushq	%rsi
14097ce9f: 57                          	pushq	%rdi
14097cea0: 53                          	pushq	%rbx
14097cea1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
14097cea8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097ceaf: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
14097ceb5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
14097cebb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
14097cec1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
14097cec7: 66 44 0f 7f 9c 24 80 00 00 00       	movdqa	%xmm11, 0x80(%rsp)
14097ced1: 66 44 0f 7f 94 24 90 00 00 00       	movdqa	%xmm10, 0x90(%rsp)
14097cedb: 66 44 0f 7f 8c 24 a0 00 00 00       	movdqa	%xmm9, 0xa0(%rsp)
14097cee5: 66 44 0f 7f 84 24 b0 00 00 00       	movdqa	%xmm8, 0xb0(%rsp)
14097ceef: 66 0f 7f bc 24 c0 00 00 00  	movdqa	%xmm7, 0xc0(%rsp)
14097cef8: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
14097cf01: c6 85 66 03 00 00 00        	movb	$0x0, 0x366(%rbp)
14097cf08: 48 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %rcx
14097cf0f: e8 5c b5 f1 ff              	callq	0x140898470 <.text+0x897470>
14097cf14: c6 85 66 03 00 00 00        	movb	$0x0, 0x366(%rbp)
14097cf1b: 66 0f 6f b4 24 d0 00 00 00  	movdqa	0xd0(%rsp), %xmm6
14097cf24: 66 0f 6f bc 24 c0 00 00 00  	movdqa	0xc0(%rsp), %xmm7
14097cf2d: 66 44 0f 6f 84 24 b0 00 00 00       	movdqa	0xb0(%rsp), %xmm8
14097cf37: 66 44 0f 6f 8c 24 a0 00 00 00       	movdqa	0xa0(%rsp), %xmm9
14097cf41: 66 44 0f 6f 94 24 90 00 00 00       	movdqa	0x90(%rsp), %xmm10
14097cf4b: 66 44 0f 6f 9c 24 80 00 00 00       	movdqa	0x80(%rsp), %xmm11
14097cf55: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
14097cf5b: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
14097cf61: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
14097cf67: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
14097cf6d: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
14097cf74: 5b                          	popq	%rbx
14097cf75: 5f                          	popq	%rdi
14097cf76: 5e                          	popq	%rsi
14097cf77: 41 5c                       	popq	%r12
14097cf79: 41 5d                       	popq	%r13
14097cf7b: 41 5e                       	popq	%r14
14097cf7d: 41 5f                       	popq	%r15
14097cf7f: 5d                          	popq	%rbp
14097cf80: c3                          	retq
14097cf81: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14097cf90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097cf95: 55                          	pushq	%rbp
14097cf96: 41 57                       	pushq	%r15
14097cf98: 41 56                       	pushq	%r14
14097cf9a: 41 55                       	pushq	%r13
14097cf9c: 41 54                       	pushq	%r12
14097cf9e: 56                          	pushq	%rsi
14097cf9f: 57                          	pushq	%rdi
14097cfa0: 53                          	pushq	%rbx
14097cfa1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
14097cfa8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097cfaf: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
14097cfb5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
14097cfbb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
14097cfc1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
14097cfc7: 66 44 0f 7f 9c 24 80 00 00 00       	movdqa	%xmm11, 0x80(%rsp)
14097cfd1: 66 44 0f 7f 94 24 90 00 00 00       	movdqa	%xmm10, 0x90(%rsp)
14097cfdb: 66 44 0f 7f 8c 24 a0 00 00 00       	movdqa	%xmm9, 0xa0(%rsp)
14097cfe5: 66 44 0f 7f 84 24 b0 00 00 00       	movdqa	%xmm8, 0xb0(%rsp)
14097cfef: 66 0f 7f bc 24 c0 00 00 00  	movdqa	%xmm7, 0xc0(%rsp)
14097cff8: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
14097d001: 48 83 bd 58 03 00 00 00     	cmpq	$0x0, 0x358(%rbp)
14097d009: 74 21                       	je	0x14097d02c <.text+0x97c02c>
14097d00b: 48 8b 85 58 03 00 00        	movq	0x358(%rbp), %rax
14097d012: 48 c1 e0 02                 	shlq	$0x2, %rax
14097d016: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14097d01a: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14097d020: 48 8b 8d 10 03 00 00        	movq	0x310(%rbp), %rcx
14097d027: e8 f4 6b c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097d02c: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
14097d034: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
14097d03c: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
14097d045: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
14097d04e: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
14097d057: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
14097d060: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
14097d066: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
14097d06c: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
14097d072: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
14097d078: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
14097d07f: 5b                          	popq	%rbx
14097d080: 5f                          	popq	%rdi
14097d081: 5e                          	popq	%rsi
14097d082: 41 5c                       	popq	%r12
14097d084: 41 5d                       	popq	%r13
14097d086: 41 5e                       	popq	%r14
14097d088: 41 5f                       	popq	%r15
14097d08a: 5d                          	popq	%rbp
14097d08b: c3                          	retq
14097d08c: 0f 1f 40 00                 	nopl	(%rax)
14097d090: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097d095: 55                          	pushq	%rbp
14097d096: 41 57                       	pushq	%r15
14097d098: 41 56                       	pushq	%r14
14097d09a: 41 55                       	pushq	%r13
14097d09c: 41 54                       	pushq	%r12
14097d09e: 56                          	pushq	%rsi
14097d09f: 57                          	pushq	%rdi
14097d0a0: 53                          	pushq	%rbx
14097d0a1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
14097d0a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097d0af: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
14097d0b5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
14097d0bb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
14097d0c1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
14097d0c7: 66 44 0f 7f 9c 24 80 00 00 00       	movdqa	%xmm11, 0x80(%rsp)
14097d0d1: 66 44 0f 7f 94 24 90 00 00 00       	movdqa	%xmm10, 0x90(%rsp)
14097d0db: 66 44 0f 7f 8c 24 a0 00 00 00       	movdqa	%xmm9, 0xa0(%rsp)
14097d0e5: 66 44 0f 7f 84 24 b0 00 00 00       	movdqa	%xmm8, 0xb0(%rsp)
14097d0ef: 66 0f 7f bc 24 c0 00 00 00  	movdqa	%xmm7, 0xc0(%rsp)
14097d0f8: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
14097d101: 80 bd 66 03 00 00 00        	cmpb	$0x0, 0x366(%rbp)
14097d108: 75 60                       	jne	0x14097d16a <.text+0x97c16a>
14097d10a: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
14097d112: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
14097d11a: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
14097d123: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
14097d12c: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
14097d135: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
14097d13e: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
14097d144: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
14097d14a: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
14097d150: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
14097d156: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
14097d15d: 5b                          	popq	%rbx
14097d15e: 5f                          	popq	%rdi
14097d15f: 5e                          	popq	%rsi
14097d160: 41 5c                       	popq	%r12
14097d162: 41 5d                       	popq	%r13
14097d164: 41 5e                       	popq	%r14
14097d166: 41 5f                       	popq	%r15
14097d168: 5d                          	popq	%rbp
14097d169: c3                          	retq
14097d16a: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
14097d171: e8 fa b2 f1 ff              	callq	0x140898470 <.text+0x897470>
14097d176: eb 92                       	jmp	0x14097d10a <.text+0x97c10a>
14097d178: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14097d180: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097d185: 55                          	pushq	%rbp
14097d186: 41 57                       	pushq	%r15
14097d188: 41 56                       	pushq	%r14
14097d18a: 41 55                       	pushq	%r13
14097d18c: 41 54                       	pushq	%r12
14097d18e: 56                          	pushq	%rsi
14097d18f: 57                          	pushq	%rdi
14097d190: 53                          	pushq	%rbx
14097d191: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
14097d198: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097d19f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
14097d1a5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
14097d1ab: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
14097d1b1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
14097d1b7: 66 44 0f 7f 9c 24 80 00 00 00       	movdqa	%xmm11, 0x80(%rsp)
14097d1c1: 66 44 0f 7f 94 24 90 00 00 00       	movdqa	%xmm10, 0x90(%rsp)
14097d1cb: 66 44 0f 7f 8c 24 a0 00 00 00       	movdqa	%xmm9, 0xa0(%rsp)
14097d1d5: 66 44 0f 7f 84 24 b0 00 00 00       	movdqa	%xmm8, 0xb0(%rsp)
14097d1df: 66 0f 7f bc 24 c0 00 00 00  	movdqa	%xmm7, 0xc0(%rsp)
14097d1e8: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
14097d1f0: 48 8b 85 e0 02 00 00        	movq	0x2e0(%rbp), %rax
14097d1f7: 48 85 c0                    	testq	%rax, %rax
14097d1fa: 74 1a                       	je	0x14097d216 <.text+0x97c216>
14097d1fc: 48 8b 8d e8 02 00 00        	movq	0x2e8(%rbp), %rcx
14097d203: 48 c1 e0 02                 	shlq	$0x2, %rax
14097d207: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14097d20b: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14097d211: e8 0a 6a c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097d216: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
14097d21e: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
14097d226: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
14097d22f: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
14097d238: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
14097d241: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
14097d24a: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
14097d250: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
14097d256: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
14097d25c: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
14097d262: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
14097d269: 5b                          	popq	%rbx
14097d26a: 5f                          	popq	%rdi
14097d26b: 5e                          	popq	%rsi
14097d26c: 41 5c                       	popq	%r12
14097d26e: 41 5d                       	popq	%r13
14097d270: 41 5e                       	popq	%r14
14097d272: 41 5f                       	popq	%r15
14097d274: 5d                          	popq	%rbp
14097d275: c3                          	retq
