
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14234b010: 55                          	pushq	%rbp
14234b011: 41 57                       	pushq	%r15
14234b013: 41 56                       	pushq	%r14
14234b015: 41 55                       	pushq	%r13
14234b017: 41 54                       	pushq	%r12
14234b019: 56                          	pushq	%rsi
14234b01a: 57                          	pushq	%rdi
14234b01b: 53                          	pushq	%rbx
14234b01c: 48 81 ec 48 03 00 00        	subq	$0x348, %rsp            # imm = 0x348
14234b023: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14234b02b: 44 0f 29 95 b0 02 00 00     	movaps	%xmm10, 0x2b0(%rbp)
14234b033: 44 0f 29 8d a0 02 00 00     	movaps	%xmm9, 0x2a0(%rbp)
14234b03b: 44 0f 29 85 90 02 00 00     	movaps	%xmm8, 0x290(%rbp)
14234b043: 0f 29 bd 80 02 00 00        	movaps	%xmm7, 0x280(%rbp)
14234b04a: 0f 29 b5 70 02 00 00        	movaps	%xmm6, 0x270(%rbp)
14234b051: 48 c7 85 68 02 00 00 fe ff ff ff    	movq	$-0x2, 0x268(%rbp)
14234b05c: 4c 89 8d 40 02 00 00        	movq	%r9, 0x240(%rbp)
14234b063: 4c 89 85 18 02 00 00        	movq	%r8, 0x218(%rbp)
14234b06a: 49 89 cf                    	movq	%rcx, %r15
14234b06d: 48 8b 85 60 03 00 00        	movq	0x360(%rbp), %rax
14234b074: 4c 8b 22                    	movq	(%rdx), %r12
14234b077: 4c 8b 42 20                 	movq	0x20(%rdx), %r8
14234b07b: 48 8b 4a 40                 	movq	0x40(%rdx), %rcx
14234b07f: 48 89 8d 90 01 00 00        	movq	%rcx, 0x190(%rbp)
14234b086: 48 8b 4a 60                 	movq	0x60(%rdx), %rcx
14234b08a: 48 89 8d a0 01 00 00        	movq	%rcx, 0x1a0(%rbp)
14234b091: 48 8b 9a 80 00 00 00        	movq	0x80(%rdx), %rbx
14234b098: 48 8b 8a a0 00 00 00        	movq	0xa0(%rdx), %rcx
14234b09f: 48 89 8d 10 02 00 00        	movq	%rcx, 0x210(%rbp)
14234b0a6: 48 8b 8a c0 00 00 00        	movq	0xc0(%rdx), %rcx
14234b0ad: 48 89 8d 20 02 00 00        	movq	%rcx, 0x220(%rbp)
14234b0b4: 48 8b 8a e0 00 00 00        	movq	0xe0(%rdx), %rcx
14234b0bb: 48 89 8d c8 01 00 00        	movq	%rcx, 0x1c8(%rbp)
14234b0c2: 48 8b 08                    	movq	(%rax), %rcx
14234b0c5: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
14234b0cc: 48 8b 40 20                 	movq	0x20(%rax), %rax
14234b0d0: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14234b0d7: 48 8b 8d 58 03 00 00        	movq	0x358(%rbp), %rcx
14234b0de: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
14234b0e5: 48 8b 51 20                 	movq	0x20(%rcx), %rdx
14234b0e9: 48 89 95 d0 01 00 00        	movq	%rdx, 0x1d0(%rbp)
14234b0f0: 48 8b 51 30                 	movq	0x30(%rcx), %rdx
14234b0f4: 48 89 95 d8 01 00 00        	movq	%rdx, 0x1d8(%rbp)
14234b0fb: 8b 51 3c                    	movl	0x3c(%rcx), %edx
14234b0fe: 89 95 f4 01 00 00           	movl	%edx, 0x1f4(%rbp)
14234b104: 48 8b 51 40                 	movq	0x40(%rcx), %rdx
14234b108: 48 89 95 08 02 00 00        	movq	%rdx, 0x208(%rbp)
14234b10f: 48 8b 51 48                 	movq	0x48(%rcx), %rdx
14234b113: 48 89 95 00 02 00 00        	movq	%rdx, 0x200(%rbp)
14234b11a: 48 8b 71 68                 	movq	0x68(%rcx), %rsi
14234b11e: 48 8b 79 70                 	movq	0x70(%rcx), %rdi
14234b122: 48 8b 91 90 00 00 00        	movq	0x90(%rcx), %rdx
14234b129: 48 89 95 b0 01 00 00        	movq	%rdx, 0x1b0(%rbp)
14234b130: 48 8b 91 a0 00 00 00        	movq	0xa0(%rcx), %rdx
14234b137: 48 89 95 b8 01 00 00        	movq	%rdx, 0x1b8(%rbp)
14234b13e: 8b 91 ac 00 00 00           	movl	0xac(%rcx), %edx
14234b144: 89 95 f0 01 00 00           	movl	%edx, 0x1f0(%rbp)
14234b14a: 4c 8b a9 b0 00 00 00        	movq	0xb0(%rcx), %r13
14234b151: 48 8b 91 c0 00 00 00        	movq	0xc0(%rcx), %rdx
14234b158: 48 89 95 e0 01 00 00        	movq	%rdx, 0x1e0(%rbp)
14234b15f: 8b 91 cc 00 00 00           	movl	0xcc(%rcx), %edx
14234b165: 89 95 3c 02 00 00           	movl	%edx, 0x23c(%rbp)
14234b16b: 48 8b 91 f0 00 00 00        	movq	0xf0(%rcx), %rdx
14234b172: 48 89 95 30 02 00 00        	movq	%rdx, 0x230(%rbp)
14234b179: 48 8b 89 f8 00 00 00        	movq	0xf8(%rcx), %rcx
14234b180: 48 89 8d 88 01 00 00        	movq	%rcx, 0x188(%rbp)
14234b187: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14234b18a: 48 8b 50 10                 	movq	0x10(%rax), %rdx
14234b18e: 89 0a                       	movl	%ecx, (%rdx)
14234b190: 4c 8b 30                    	movq	(%rax), %r14
14234b193: 4c 89 85 c0 01 00 00        	movq	%r8, 0x1c0(%rbp)
14234b19a: 4c 89 c1                    	movq	%r8, %rcx
14234b19d: e8 1e 4a 5f fe              	callq	0x14093fbc0 <.text+0x93ebc0>
14234b1a2: 84 c0                       	testb	%al, %al
14234b1a4: 74 0a                       	je	0x14234b1b0 <.text+0x234a1b0>
14234b1a6: 41 c6 46 0a 00              	movb	$0x0, 0xa(%r14)
14234b1ab: 41 c6 46 0d 01              	movb	$0x1, 0xd(%r14)
14234b1b0: 41 8b 04 24                 	movl	(%r12), %eax
14234b1b4: 83 f8 0d                    	cmpl	$0xd, %eax
14234b1b7: 74 3e                       	je	0x14234b1f7 <.text+0x234a1f7>
14234b1b9: 83 f8 13                    	cmpl	$0x13, %eax
14234b1bc: 75 2e                       	jne	0x14234b1ec <.text+0x234a1ec>
14234b1be: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
14234b1c5: 8b 88 00 01 00 00           	movl	0x100(%rax), %ecx
14234b1cb: f3 0f 10 88 04 01 00 00     	movss	0x104(%rax), %xmm1
14234b1d3: e8 b8 48 5f fe              	callq	0x14093fa90 <.text+0x93ea90>
14234b1d8: 84 c0                       	testb	%al, %al
14234b1da: 74 1b                       	je	0x14234b1f7 <.text+0x234a1f7>
14234b1dc: 41 0f b6 46 0a              	movzbl	0xa(%r14), %eax
14234b1e1: 89 c1                       	movl	%eax, %ecx
14234b1e3: 80 f1 01                    	xorb	$0x1, %cl
14234b1e6: 41 88 4e 0a                 	movb	%cl, 0xa(%r14)
14234b1ea: eb 07                       	jmp	0x14234b1f3 <.text+0x234a1f3>
14234b1ec: 41 c6 46 0a 00              	movb	$0x0, 0xa(%r14)
14234b1f1: b0 01                       	movb	$0x1, %al
14234b1f3: 41 88 46 0d                 	movb	%al, 0xd(%r14)
14234b1f7: 48 89 9d 98 01 00 00        	movq	%rbx, 0x198(%rbp)
14234b1fe: 48 8b 0e                    	movq	(%rsi), %rcx
14234b201: 31 c0                       	xorl	%eax, %eax
14234b203: 48 89 ca                    	movq	%rcx, %rdx
14234b206: 48 2b 57 18                 	subq	0x18(%rdi), %rdx
14234b20a: 48 0f 42 d0                 	cmovbq	%rax, %rdx
14234b20e: 48 2b 4f 38                 	subq	0x38(%rdi), %rcx
14234b212: 48 0f 42 c8                 	cmovbq	%rax, %rcx
14234b216: 4c 8d 04 92                 	leaq	(%rdx,%rdx,4), %r8
14234b21a: 49 c1 e0 03                 	shlq	$0x3, %r8
14234b21e: 4c 03 47 08                 	addq	0x8(%rdi), %r8
14234b222: 4c 8d 0c 89                 	leaq	(%rcx,%rcx,4), %r9
14234b226: 49 c1 e1 03                 	shlq	$0x3, %r9
14234b22a: 4c 03 4f 28                 	addq	0x28(%rdi), %r9
14234b22e: 4c 8b 57 10                 	movq	0x10(%rdi), %r10
14234b232: 49 29 d2                    	subq	%rdx, %r10
14234b235: 4c 0f 42 d0                 	cmovbq	%rax, %r10
14234b239: 48 8b 57 30                 	movq	0x30(%rdi), %rdx
14234b23d: 41 bb 08 00 00 00           	movl	$0x8, %r11d
14234b243: 4d 0f 42 c3                 	cmovbq	%r11, %r8
14234b247: 4b 8d 1c 92                 	leaq	(%r10,%r10,4), %rbx
14234b24b: 4c 89 b5 48 02 00 00        	movq	%r14, 0x248(%rbp)
14234b252: 41 0f 96 c6                 	setbe	%r14b
14234b256: 4d 8d 44 d8 d8              	leaq	-0x28(%r8,%rbx,8), %r8
14234b25b: 4c 0f 46 c0                 	cmovbeq	%rax, %r8
14234b25f: 48 29 ca                    	subq	%rcx, %rdx
14234b262: 48 0f 42 d0                 	cmovbq	%rax, %rdx
14234b266: 4d 0f 42 cb                 	cmovbq	%r11, %r9
14234b26a: 48 8d 0c 92                 	leaq	(%rdx,%rdx,4), %rcx
14234b26e: 41 0f 96 c3                 	setbe	%r11b
14234b272: 49 8d 5c c9 d8              	leaq	-0x28(%r9,%rcx,8), %rbx
14234b277: 49 0f 46 d8                 	cmovbeq	%r8, %rbx
14234b27b: 4c 01 d2                    	addq	%r10, %rdx
14234b27e: 45 84 de                    	testb	%r11b, %r14b
14234b281: 48 0f 44 d0                 	cmoveq	%rax, %rdx
14234b285: 48 8b 47 40                 	movq	0x40(%rdi), %rax
14234b289: 48 29 d0                    	subq	%rdx, %rax
14234b28c: 48 89 06                    	movq	%rax, (%rsi)
14234b28f: 45 84 de                    	testb	%r11b, %r14b
14234b292: 4c 8b b5 48 02 00 00        	movq	0x248(%rbp), %r14
14234b299: 0f 85 e7 00 00 00           	jne	0x14234b386 <.text+0x234a386>
14234b29f: 48 8b b5 40 03 00 00        	movq	0x340(%rbp), %rsi
14234b2a6: 83 3b 13                    	cmpl	$0x13, (%rbx)
14234b2a9: 75 7a                       	jne	0x14234b325 <.text+0x234a325>
14234b2ab: 83 7b 10 0d                 	cmpl	$0xd, 0x10(%rbx)
14234b2af: 74 74                       	je	0x14234b325 <.text+0x234a325>
14234b2b1: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
14234b2b5: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
14234b2b9: 89 01                       	movl	%eax, (%rcx)
14234b2bb: 49 8b 07                    	movq	(%r15), %rax
14234b2be: c6 40 01 01                 	movb	$0x1, 0x1(%rax)
14234b2c2: 4c 89 f1                    	movq	%r14, %rcx
14234b2c5: e8 a6 52 52 fe              	callq	0x140870570 <.text+0x86f570>
14234b2ca: 41 0f b6 4e 09              	movzbl	0x9(%r14), %ecx
14234b2cf: e8 2c 32 5c fe              	callq	0x14090e500 <.text+0x90d500>
14234b2d4: 4c 8b 85 40 02 00 00        	movq	0x240(%rbp), %r8
14234b2db: 41 8b 48 1c                 	movl	0x1c(%r8), %ecx
14234b2df: 49 8b 50 10                 	movq	0x10(%r8), %rdx
14234b2e3: 89 0a                       	movl	%ecx, (%rdx)
14234b2e5: 49 8b 08                    	movq	(%r8), %rcx
14234b2e8: 88 81 03 01 00 00           	movb	%al, 0x103(%rcx)
14234b2ee: 8b 46 1c                    	movl	0x1c(%rsi), %eax
14234b2f1: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
14234b2f5: 89 01                       	movl	%eax, (%rcx)
14234b2f7: 48 8b 0e                    	movq	(%rsi), %rcx
14234b2fa: b2 02                       	movb	$0x2, %dl
14234b2fc: e8 8f 37 56 fe              	callq	0x1408aea90 <.text+0x8ada90>
14234b301: 41 80 7e 0b 00              	cmpb	$0x0, 0xb(%r14)
14234b306: 75 1d                       	jne	0x14234b325 <.text+0x234a325>
14234b308: 48 8b 95 18 02 00 00        	movq	0x218(%rbp), %rdx
14234b30f: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14234b312: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14234b316: 89 01                       	movl	%eax, (%rcx)
14234b318: 48 8b 0a                    	movq	(%rdx), %rcx
14234b31b: e8 d0 33 5c fe              	callq	0x14090e6f0 <.text+0x90d6f0>
14234b320: 41 c6 46 0b 01              	movb	$0x1, 0xb(%r14)
14234b325: 83 7b 10 13                 	cmpl	$0x13, 0x10(%rbx)
14234b329: 75 5b                       	jne	0x14234b386 <.text+0x234a386>
14234b32b: 83 3b 0d                    	cmpl	$0xd, (%rbx)
14234b32e: 74 56                       	je	0x14234b386 <.text+0x234a386>
14234b330: 48 8b bd 48 03 00 00        	movq	0x348(%rbp), %rdi
14234b337: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
14234b33e: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14234b341: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14234b345: 89 01                       	movl	%eax, (%rcx)
14234b347: 48 8b 02                    	movq	(%rdx), %rax
14234b34a: c6 80 03 01 00 00 00        	movb	$0x0, 0x103(%rax)
14234b351: 8b 46 1c                    	movl	0x1c(%rsi), %eax
14234b354: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
14234b358: 89 01                       	movl	%eax, (%rcx)
14234b35a: 48 8b 0e                    	movq	(%rsi), %rcx
14234b35d: b2 01                       	movb	$0x1, %dl
14234b35f: e8 2c 37 56 fe              	callq	0x1408aea90 <.text+0x8ada90>
14234b364: 8b 47 1c                    	movl	0x1c(%rdi), %eax
14234b367: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
14234b36b: 89 01                       	movl	%eax, (%rcx)
14234b36d: 48 8b 0f                    	movq	(%rdi), %rcx
14234b370: e8 cb cd 55 fe              	callq	0x1408a8140 <.text+0x8a7140>
14234b375: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
14234b379: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
14234b37d: 89 01                       	movl	%eax, (%rcx)
14234b37f: 49 8b 07                    	movq	(%r15), %rax
14234b382: c6 40 01 00                 	movb	$0x0, 0x1(%rax)
14234b386: 49 8b 07                    	movq	(%r15), %rax
14234b389: 48 8b 9d 40 02 00 00        	movq	0x240(%rbp), %rbx
14234b390: 48 8b 0b                    	movq	(%rbx), %rcx
14234b393: 48 89 8d f8 01 00 00        	movq	%rcx, 0x1f8(%rbp)
14234b39a: 66 0f ef db                 	pxor	%xmm3, %xmm3
14234b39e: 80 78 01 01                 	cmpb	$0x1, 0x1(%rax)
14234b3a2: 48 8b b5 08 02 00 00        	movq	0x208(%rbp), %rsi
14234b3a9: 75 18                       	jne	0x14234b3c3 <.text+0x234a3c3>
14234b3ab: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
14234b3b2: 80 b9 03 01 00 00 03        	cmpb	$0x3, 0x103(%rcx)
14234b3b9: 74 08                       	je	0x14234b3c3 <.text+0x234a3c3>
14234b3bb: 66 0f 6e 1d 41 a5 5d 00     	movd	0x5da541(%rip), %xmm3   # 0x142925904
14234b3c3: 4c 89 a5 a8 01 00 00        	movq	%r12, 0x1a8(%rbp)
14234b3ca: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
14234b3d1: 4c 89 bd 80 01 00 00        	movq	%r15, 0x180(%rbp)
14234b3d8: 48 8d 15 c4 3d d0 00        	leaq	0xd03dc4(%rip), %rdx    # 0x14304f1a3
14234b3df: 41 b8 11 00 00 00           	movl	$0x11, %r8d
14234b3e5: 48 8b 8d 20 02 00 00        	movq	0x220(%rbp), %rcx
14234b3ec: e8 2f ab 70 fe              	callq	0x140a55f20 <.text+0xa54f20>
14234b3f1: 48 8b 0e                    	movq	(%rsi), %rcx
14234b3f4: 31 c0                       	xorl	%eax, %eax
14234b3f6: 48 89 ca                    	movq	%rcx, %rdx
14234b3f9: 4c 8b 85 00 02 00 00        	movq	0x200(%rbp), %r8
14234b400: 49 2b 50 18                 	subq	0x18(%r8), %rdx
14234b404: 48 0f 42 d0                 	cmovbq	%rax, %rdx
14234b408: 49 2b 48 38                 	subq	0x38(%r8), %rcx
14234b40c: 48 0f 42 c8                 	cmovbq	%rax, %rcx
14234b410: 49 89 d7                    	movq	%rdx, %r15
14234b413: 49 c1 e7 04                 	shlq	$0x4, %r15
14234b417: 4d 03 78 08                 	addq	0x8(%r8), %r15
14234b41b: 4d 8b 60 10                 	movq	0x10(%r8), %r12
14234b41f: 4d 8b 48 30                 	movq	0x30(%r8), %r9
14234b423: 49 29 d4                    	subq	%rdx, %r12
14234b426: 4c 0f 42 e0                 	cmovbq	%rax, %r12
14234b42a: ba 08 00 00 00              	movl	$0x8, %edx
14234b42f: 4c 0f 42 fa                 	cmovbq	%rdx, %r15
14234b433: 48 89 cf                    	movq	%rcx, %rdi
14234b436: 48 c1 e7 04                 	shlq	$0x4, %rdi
14234b43a: 49 03 78 28                 	addq	0x28(%r8), %rdi
14234b43e: 49 29 c9                    	subq	%rcx, %r9
14234b441: 4c 0f 42 c8                 	cmovbq	%rax, %r9
14234b445: 48 0f 42 fa                 	cmovbq	%rdx, %rdi
14234b449: 49 8b 40 40                 	movq	0x40(%r8), %rax
14234b44d: 4c 29 c8                    	subq	%r9, %rax
14234b450: 4c 29 e0                    	subq	%r12, %rax
14234b453: 48 89 06                    	movq	%rax, (%rsi)
14234b456: 49 c1 e4 04                 	shlq	$0x4, %r12
14234b45a: 4d 01 fc                    	addq	%r15, %r12
14234b45d: 49 c1 e1 04                 	shlq	$0x4, %r9
14234b461: 49 01 f9                    	addq	%rdi, %r9
14234b464: 4c 89 8d 00 02 00 00        	movq	%r9, 0x200(%rbp)
14234b46b: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
14234b472: 8b 41 1c                    	movl	0x1c(%rcx), %eax
14234b475: 89 85 54 02 00 00           	movl	%eax, 0x254(%rbp)
14234b47b: 48 8b 01                    	movq	(%rcx), %rax
14234b47e: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
14234b485: 48 8b 41 10                 	movq	0x10(%rcx), %rax
14234b489: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
14234b490: 8b 43 1c                    	movl	0x1c(%rbx), %eax
14234b493: 89 85 18 02 00 00           	movl	%eax, 0x218(%rbp)
14234b499: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14234b49d: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
14234b4a4: 49 8d 45 20                 	leaq	0x20(%r13), %rax
14234b4a8: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
14234b4af: eb 69                       	jmp	0x14234b51a <.text+0x234a51a>
14234b4b1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14234b4c0: 49 8b 45 28                 	movq	0x28(%r13), %rax
14234b4c4: 48 89 f1                    	movq	%rsi, %rcx
14234b4c7: 48 c1 e1 04                 	shlq	$0x4, %rcx
14234b4cb: 4c 89 34 08                 	movq	%r14, (%rax,%rcx)
14234b4cf: c6 44 08 08 01              	movb	$0x1, 0x8(%rax,%rcx)
14234b4d4: 48 ff c6                    	incq	%rsi
14234b4d7: 49 89 75 30                 	movq	%rsi, 0x30(%r13)
14234b4db: 49 ff 45 40                 	incq	0x40(%r13)
14234b4df: 4c 8b b5 48 02 00 00        	movq	0x248(%rbp), %r14
14234b4e6: 48 8b b5 08 02 00 00        	movq	0x208(%rbp), %rsi
14234b4ed: 0f b6 4b 08                 	movzbl	0x8(%rbx), %ecx
14234b4f1: e8 0a 30 5c fe              	callq	0x14090e500 <.text+0x90d500>
14234b4f6: 8b 8d 18 02 00 00           	movl	0x218(%rbp), %ecx
14234b4fc: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
14234b503: 89 0a                       	movl	%ecx, (%rdx)
14234b505: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
14234b50c: 88 81 03 01 00 00           	movb	%al, 0x103(%rcx)
14234b512: 0f b6 43 08                 	movzbl	0x8(%rbx), %eax
14234b516: 41 88 46 09                 	movb	%al, 0x9(%r14)
14234b51a: 4d 85 ff                    	testq	%r15, %r15
14234b51d: 74 11                       	je	0x14234b530 <.text+0x234a530>
14234b51f: 4d 39 e7                    	cmpq	%r12, %r15
14234b522: 74 0c                       	je	0x14234b530 <.text+0x234a530>
14234b524: 49 8d 47 10                 	leaq	0x10(%r15), %rax
14234b528: 4c 89 fb                    	movq	%r15, %rbx
14234b52b: eb 22                       	jmp	0x14234b54f <.text+0x234a54f>
14234b52d: 0f 1f 00                    	nopl	(%rax)
14234b530: 48 85 ff                    	testq	%rdi, %rdi
14234b533: 0f 84 1a 01 00 00           	je	0x14234b653 <.text+0x234a653>
14234b539: 48 3b bd 00 02 00 00        	cmpq	0x200(%rbp), %rdi
14234b540: 0f 84 0d 01 00 00           	je	0x14234b653 <.text+0x234a653>
14234b546: 48 89 fb                    	movq	%rdi, %rbx
14234b549: 48 83 c7 10                 	addq	$0x10, %rdi
14234b54d: 31 c0                       	xorl	%eax, %eax
14234b54f: 48 ff 06                    	incq	(%rsi)
14234b552: 49 89 c7                    	movq	%rax, %r15
14234b555: 0f b6 43 08                 	movzbl	0x8(%rbx), %eax
14234b559: 3c 04                       	cmpb	$0x4, %al
14234b55b: 74 bd                       	je	0x14234b51a <.text+0x234a51a>
14234b55d: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
14234b564: 80 b9 03 01 00 00 03        	cmpb	$0x3, 0x103(%rcx)
14234b56b: 75 43                       	jne	0x14234b5b0 <.text+0x234a5b0>
14234b56d: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
14234b574: 8b 8d 3c 02 00 00           	movl	0x23c(%rbp), %ecx
14234b57a: 89 08                       	movl	%ecx, (%rax)
14234b57c: 49 8b 75 30                 	movq	0x30(%r13), %rsi
14234b580: 4d 8b 75 40                 	movq	0x40(%r13), %r14
14234b584: 49 3b 75 20                 	cmpq	0x20(%r13), %rsi
14234b588: 0f 85 32 ff ff ff           	jne	0x14234b4c0 <.text+0x234a4c0>
14234b58e: 48 8b 8d 20 02 00 00        	movq	0x220(%rbp), %rcx
14234b595: 48 8d 15 d4 9c 86 00        	leaq	0x869cd4(%rip), %rdx    # 0x142bb5270
14234b59c: e8 8f 70 21 fe              	callq	0x140562630 <.text+0x561630>
14234b5a1: e9 1a ff ff ff              	jmp	0x14234b4c0 <.text+0x234a4c0>
14234b5a6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14234b5b0: 3c 02                       	cmpb	$0x2, %al
14234b5b2: 0f 85 35 ff ff ff           	jne	0x14234b4ed <.text+0x234a4ed>
14234b5b8: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
14234b5bf: 8b 8d 3c 02 00 00           	movl	0x23c(%rbp), %ecx
14234b5c5: 89 08                       	movl	%ecx, (%rax)
14234b5c7: 49 8b 75 30                 	movq	0x30(%r13), %rsi
14234b5cb: 4d 8b 75 40                 	movq	0x40(%r13), %r14
14234b5cf: 49 3b 75 20                 	cmpq	0x20(%r13), %rsi
14234b5d3: 75 13                       	jne	0x14234b5e8 <.text+0x234a5e8>
14234b5d5: 48 8b 8d 20 02 00 00        	movq	0x220(%rbp), %rcx
14234b5dc: 48 8d 15 8d 9c 86 00        	leaq	0x869c8d(%rip), %rdx    # 0x142bb5270
14234b5e3: e8 48 70 21 fe              	callq	0x140562630 <.text+0x561630>
14234b5e8: 49 8b 45 28                 	movq	0x28(%r13), %rax
14234b5ec: 48 89 f1                    	movq	%rsi, %rcx
14234b5ef: 48 c1 e1 04                 	shlq	$0x4, %rcx
14234b5f3: 4c 89 34 08                 	movq	%r14, (%rax,%rcx)
14234b5f7: c6 44 08 08 00              	movb	$0x0, 0x8(%rax,%rcx)
14234b5fc: 48 ff c6                    	incq	%rsi
14234b5ff: 49 89 75 30                 	movq	%rsi, 0x30(%r13)
14234b603: 49 ff 45 40                 	incq	0x40(%r13)
14234b607: 4c 8b b5 48 02 00 00        	movq	0x248(%rbp), %r14
14234b60e: 41 80 7e 0c 00              	cmpb	$0x0, 0xc(%r14)
14234b613: 48 8b b5 08 02 00 00        	movq	0x208(%rbp), %rsi
14234b61a: 0f 85 cd fe ff ff           	jne	0x14234b4ed <.text+0x234a4ed>
14234b620: 8b 85 54 02 00 00           	movl	0x254(%rbp), %eax
14234b626: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
14234b62d: 89 01                       	movl	%eax, (%rcx)
14234b62f: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234b636: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
14234b63d: 4c 8b 85 78 01 00 00        	movq	0x178(%rbp), %r8
14234b644: e8 c7 2e 5c fe              	callq	0x14090e510 <.text+0x90d510>
14234b649: 41 c6 46 0c 01              	movb	$0x1, 0xc(%r14)
14234b64e: e9 9a fe ff ff              	jmp	0x14234b4ed <.text+0x234a4ed>
14234b653: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
14234b65a: 80 78 01 00                 	cmpb	$0x0, 0x1(%rax)
14234b65e: 74 09                       	je	0x14234b669 <.text+0x234a669>
14234b660: 0f b6 30                    	movzbl	(%rax), %esi
14234b663: 40 80 f6 01                 	xorb	$0x1, %sil
14234b667: eb 02                       	jmp	0x14234b66b <.text+0x234a66b>
14234b669: 31 f6                       	xorl	%esi, %esi
14234b66b: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
14234b672: 48 8b 00                    	movq	(%rax), %rax
14234b675: 45 31 d2                    	xorl	%r10d, %r10d
14234b678: 48 89 c1                    	movq	%rax, %rcx
14234b67b: 4c 8b bd 88 01 00 00        	movq	0x188(%rbp), %r15
14234b682: 49 2b 4f 18                 	subq	0x18(%r15), %rcx
14234b686: 49 0f 42 ca                 	cmovbq	%r10, %rcx
14234b68a: 49 2b 47 38                 	subq	0x38(%r15), %rax
14234b68e: 49 0f 42 c2                 	cmovbq	%r10, %rax
14234b692: 4d 8b 4f 08                 	movq	0x8(%r15), %r9
14234b696: 4d 8b 47 30                 	movq	0x30(%r15), %r8
14234b69a: 4c 89 c2                    	movq	%r8, %rdx
14234b69d: 48 29 c2                    	subq	%rax, %rdx
14234b6a0: 49 0f 42 d2                 	cmovbq	%r10, %rdx
14234b6a4: 49 8b 5f 10                 	movq	0x10(%r15), %rbx
14234b6a8: 41 0f 96 c6                 	setbe	%r14b
14234b6ac: 48 89 df                    	movq	%rbx, %rdi
14234b6af: 48 29 cf                    	subq	%rcx, %rdi
14234b6b2: 49 0f 42 fa                 	cmovbq	%r10, %rdi
14234b6b6: 4d 8b 5f 28                 	movq	0x28(%r15), %r11
14234b6ba: 4d 8b 57 40                 	movq	0x40(%r15), %r10
14234b6be: 41 0f 96 c7                 	setbe	%r15b
14234b6c2: 4d 89 d5                    	movq	%r10, %r13
14234b6c5: 49 29 d5                    	subq	%rdx, %r13
14234b6c8: 49 29 fd                    	subq	%rdi, %r13
14234b6cb: 4c 8b a5 30 02 00 00        	movq	0x230(%rbp), %r12
14234b6d2: 4d 89 2c 24                 	movq	%r13, (%r12)
14234b6d6: 4c 8b ad 30 02 00 00        	movq	0x230(%rbp), %r13
14234b6dd: 45 84 f7                    	testb	%r14b, %r15b
14234b6e0: 4c 8b bd 80 01 00 00        	movq	0x180(%rbp), %r15
14234b6e7: 4c 8b a5 e8 01 00 00        	movq	0x1e8(%rbp), %r12
14234b6ee: 0f 85 bc 00 00 00           	jne	0x14234b7b0 <.text+0x234a7b0>
14234b6f4: 49 89 c6                    	movq	%rax, %r14
14234b6f7: 49 c1 e6 04                 	shlq	$0x4, %r14
14234b6fb: 4d 01 f3                    	addq	%r14, %r11
14234b6fe: 48 c1 e2 04                 	shlq	$0x4, %rdx
14234b702: 49 89 ce                    	movq	%rcx, %r14
14234b705: 49 c1 e6 04                 	shlq	$0x4, %r14
14234b709: 4d 01 f1                    	addq	%r14, %r9
14234b70c: 48 c1 e7 04                 	shlq	$0x4, %rdi
14234b710: 45 31 f6                    	xorl	%r14d, %r14d
14234b713: 48 39 cb                    	cmpq	%rcx, %rbx
14234b716: b9 08 00 00 00              	movl	$0x8, %ecx
14234b71b: 4c 0f 42 c9                 	cmovbq	%rcx, %r9
14234b71f: 4d 8d 4c 39 f0              	leaq	-0x10(%r9,%rdi), %r9
14234b724: 4d 0f 47 f1                 	cmovaq	%r9, %r14
14234b728: 49 39 c0                    	cmpq	%rax, %r8
14234b72b: 4c 0f 42 d9                 	cmovbq	%rcx, %r11
14234b72f: 49 8d 44 13 f0              	leaq	-0x10(%r11,%rdx), %rax
14234b734: 49 0f 46 c6                 	cmovbeq	%r14, %rax
14234b738: 4d 89 55 00                 	movq	%r10, (%r13)
14234b73c: 80 78 08 01                 	cmpb	$0x1, 0x8(%rax)
14234b740: 75 6e                       	jne	0x14234b7b0 <.text+0x234a7b0>
14234b742: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
14234b749: c6 40 0a 00                 	movb	$0x0, 0xa(%rax)
14234b74d: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
14234b751: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
14234b755: 89 01                       	movl	%eax, (%rcx)
14234b757: 41 c6 44 24 01 01           	movb	$0x1, 0x1(%r12)
14234b75d: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
14234b764: 8b 8d f0 01 00 00           	movl	0x1f0(%rbp), %ecx
14234b76a: 89 08                       	movl	%ecx, (%rax)
14234b76c: 4c 8b b5 b0 01 00 00        	movq	0x1b0(%rbp), %r14
14234b773: 49 8b 7e 30                 	movq	0x30(%r14), %rdi
14234b777: 49 8b 5e 40                 	movq	0x40(%r14), %rbx
14234b77b: 49 3b 7e 20                 	cmpq	0x20(%r14), %rdi
14234b77f: 75 10                       	jne	0x14234b791 <.text+0x234a791>
14234b781: 49 8d 4e 20                 	leaq	0x20(%r14), %rcx
14234b785: 48 8d 15 e4 9a 86 00        	leaq	0x869ae4(%rip), %rdx    # 0x142bb5270
14234b78c: e8 1f 76 21 fe              	callq	0x140562db0 <.text+0x561db0>
14234b791: 49 8b 46 28                 	movq	0x28(%r14), %rax
14234b795: 48 8d 0c 7f                 	leaq	(%rdi,%rdi,2), %rcx
14234b799: c7 04 c8 13 00 00 00        	movl	$0x13, (%rax,%rcx,8)
14234b7a0: 48 89 5c c8 10              	movq	%rbx, 0x10(%rax,%rcx,8)
14234b7a5: 48 ff c7                    	incq	%rdi
14234b7a8: 49 89 7e 30                 	movq	%rdi, 0x30(%r14)
14234b7ac: 49 ff 46 40                 	incq	0x40(%r14)
14234b7b0: 41 0f b6 44 24 01           	movzbl	0x1(%r12), %eax
14234b7b6: 3c 01                       	cmpb	$0x1, %al
14234b7b8: 0f 85 46 09 00 00           	jne	0x14234c104 <.text+0x234b104>
14234b7be: 40 84 f6                    	testb	%sil, %sil
14234b7c1: 0f 84 a6 00 00 00           	je	0x14234b86d <.text+0x234a86d>
14234b7c7: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
14234b7ce: 83 38 0d                    	cmpl	$0xd, (%rax)
14234b7d1: 75 56                       	jne	0x14234b829 <.text+0x234a829>
14234b7d3: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
14234b7da: 8b 8d f4 01 00 00           	movl	0x1f4(%rbp), %ecx
14234b7e0: 89 08                       	movl	%ecx, (%rax)
14234b7e2: 48 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %rbx
14234b7e9: 48 8b 73 30                 	movq	0x30(%rbx), %rsi
14234b7ed: 48 8b 7b 40                 	movq	0x40(%rbx), %rdi
14234b7f1: 48 3b 73 20                 	cmpq	0x20(%rbx), %rsi
14234b7f5: 75 10                       	jne	0x14234b807 <.text+0x234a807>
14234b7f7: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
14234b7fb: 48 8d 15 6e 9a 86 00        	leaq	0x869a6e(%rip), %rdx    # 0x142bb5270
14234b802: e8 29 6e 21 fe              	callq	0x140562630 <.text+0x561630>
14234b807: 48 8b 43 28                 	movq	0x28(%rbx), %rax
14234b80b: 48 89 f1                    	movq	%rsi, %rcx
14234b80e: 48 c1 e1 04                 	shlq	$0x4, %rcx
14234b812: 48 89 3c 08                 	movq	%rdi, (%rax,%rcx)
14234b816: c7 44 08 08 00 03 00 03     	movl	$0x3000300, 0x8(%rax,%rcx) # imm = 0x3000300
14234b81e: 48 ff c6                    	incq	%rsi
14234b821: 48 89 73 30                 	movq	%rsi, 0x30(%rbx)
14234b825: 48 ff 43 40                 	incq	0x40(%rbx)
14234b829: 48 8b 95 58 03 00 00        	movq	0x358(%rbp), %rdx
14234b830: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14234b833: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14234b837: 89 01                       	movl	%eax, (%rcx)
14234b839: 48 8b 32                    	movq	(%rdx), %rsi
14234b83c: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
14234b840: 48 8b 5e 40                 	movq	0x40(%rsi), %rbx
14234b844: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
14234b848: 75 10                       	jne	0x14234b85a <.text+0x234a85a>
14234b84a: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
14234b84e: 48 8d 15 53 be 87 00        	leaq	0x87be53(%rip), %rdx    # 0x142bc76a8
14234b855: e8 16 70 21 fe              	callq	0x140562870 <.text+0x561870>
14234b85a: 48 8b 46 28                 	movq	0x28(%rsi), %rax
14234b85e: 48 89 1c f8                 	movq	%rbx, (%rax,%rdi,8)
14234b862: 48 ff c7                    	incq	%rdi
14234b865: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
14234b869: 48 ff 46 40                 	incq	0x40(%rsi)
14234b86d: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
14234b874: f3 0f 10 80 60 01 00 00     	movss	0x160(%rax), %xmm0
14234b87c: 0f 57 c9                    	xorps	%xmm1, %xmm1
14234b87f: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
14234b882: 4c 8b b5 48 02 00 00        	movq	0x248(%rbp), %r14
14234b889: 75 02                       	jne	0x14234b88d <.text+0x234a88d>
14234b88b: 7b 7d                       	jnp	0x14234b90a <.text+0x234a90a>
14234b88d: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234b894: e8 97 59 5c fe              	callq	0x140911230 <.text+0x910230>
14234b899: 3c 04                       	cmpb	$0x4, %al
14234b89b: 74 6d                       	je	0x14234b90a <.text+0x234a90a>
14234b89d: 0f b6 c0                    	movzbl	%al, %eax
14234b8a0: 83 f8 01                    	cmpl	$0x1, %eax
14234b8a3: 74 65                       	je	0x14234b90a <.text+0x234a90a>
14234b8a5: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
14234b8ac: f3 44 0f 10 80 60 01 00 00  	movss	0x160(%rax), %xmm8
14234b8b5: 8b 85 54 02 00 00           	movl	0x254(%rbp), %eax
14234b8bb: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
14234b8c2: 89 01                       	movl	%eax, (%rcx)
14234b8c4: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234b8cb: e8 90 59 5c fe              	callq	0x140911260 <.text+0x910260>
14234b8d0: f3 0f 10 78 0c              	movss	0xc(%rax), %xmm7
14234b8d5: f3 0f 10 70 10              	movss	0x10(%rax), %xmm6
14234b8da: 0f 2e f7                    	ucomiss	%xmm7, %xmm6
14234b8dd: 0f 82 99 08 00 00           	jb	0x14234c17c <.text+0x234b17c>
14234b8e3: 48 89 c7                    	movq	%rax, %rdi
14234b8e6: f3 44 0f 59 05 1d 38 d0 00  	mulss	0xd0381d(%rip), %xmm8   # 0x14304f10c
14234b8ef: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
14234b8f3: e8 b8 85 5d 00              	callq	0x142923eb0 <.text+0x2922eb0>
14234b8f8: f3 0f 59 47 08              	mulss	0x8(%rdi), %xmm0
14234b8fd: f3 0f 5f f8                 	maxss	%xmm0, %xmm7
14234b901: f3 0f 5d f7                 	minss	%xmm7, %xmm6
14234b905: f3 0f 11 77 08              	movss	%xmm6, 0x8(%rdi)
14234b90a: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
14234b911: 48 8b 38                    	movq	(%rax), %rdi
14234b914: c7 45 14 ff ff 7f 7f        	movl	$0x7f7fffff, 0x14(%rbp) # imm = 0x7F7FFFFF
14234b91b: c6 45 10 05                 	movb	$0x5, 0x10(%rbp)
14234b91f: c6 45 b0 00                 	movb	$0x0, -0x50(%rbp)
14234b923: 48 8d 9d c0 00 00 00        	leaq	0xc0(%rbp), %rbx
14234b92a: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
14234b92e: 4c 8d 45 b0                 	leaq	-0x50(%rbp), %r8
14234b932: 48 89 d9                    	movq	%rbx, %rcx
14234b935: e8 d6 d4 65 fe              	callq	0x1409a8e10 <.text+0x9a7e10>
14234b93a: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14234b93e: 48 89 da                    	movq	%rbx, %rdx
14234b941: 45 31 c0                    	xorl	%r8d, %r8d
14234b944: e8 97 d5 65 fe              	callq	0x1409a8ee0 <.text+0x9a7ee0>
14234b949: c6 85 67 02 00 00 01        	movb	$0x1, 0x267(%rbp)
14234b950: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14234b954: e8 57 d6 65 fe              	callq	0x1409a8fb0 <.text+0x9a7fb0>
14234b959: c6 85 67 02 00 00 00        	movb	$0x0, 0x267(%rbp)
14234b960: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
14234b967: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
14234b96b: 4c 8d 45 b0                 	leaq	-0x50(%rbp), %r8
14234b96f: e8 0c d5 65 fe              	callq	0x1409a8e80 <.text+0x9a7e80>
14234b974: 48 8d 15 39 38 d0 00        	leaq	0xd03839(%rip), %rdx    # 0x14304f1b4
14234b97b: 4c 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %r9
14234b982: 41 b8 13 00 00 00           	movl	$0x13, %r8d
14234b988: 48 89 f9                    	movq	%rdi, %rcx
14234b98b: e8 40 7d 5d fe              	callq	0x1409236d0 <.text+0x9226d0>
14234b990: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
14234b994: 48 89 55 b8                 	movq	%rdx, -0x48(%rbp)
14234b998: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234b99f: e8 8c 58 5c fe              	callq	0x140911230 <.text+0x910230>
14234b9a4: 89 c3                       	movl	%eax, %ebx
14234b9a6: 41 c7 46 04 ff ff ff ff     	movl	$0xffffffff, 0x4(%r14)  # imm = 0xFFFFFFFF
14234b9ae: 48 8d 15 c3 a3 d0 00        	leaq	0xd0a3c3(%rip), %rdx    # 0x143055d78
14234b9b5: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
14234b9bc: e8 bf 78 05 00              	callq	0x1423a3280 <.text+0x23a2280>
14234b9c1: 48 85 c0                    	testq	%rax, %rax
14234b9c4: 0f 84 c4 07 00 00           	je	0x14234c18e <.text+0x234b18e>
14234b9ca: 48 89 c7                    	movq	%rax, %rdi
14234b9cd: 48 8d 05 6c f0 c4 00        	leaq	0xc4f06c(%rip), %rax    # 0x142f9aa40
14234b9d4: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14234b9d8: 48 c7 45 18 57 00 00 00     	movq	$0x57, 0x18(%rbp)
14234b9e0: 48 83 7f 18 57              	cmpq	$0x57, 0x18(%rdi)
14234b9e5: 0f 85 61 07 00 00           	jne	0x14234c14c <.text+0x234b14c>
14234b9eb: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
14234b9ef: 48 8d 0d 4a f0 c4 00        	leaq	0xc4f04a(%rip), %rcx    # 0x142f9aa40
14234b9f6: 41 b8 57 00 00 00           	movl	$0x57, %r8d
14234b9fc: e8 ef 81 5d 00              	callq	0x142923bf0 <.text+0x2922bf0>
14234ba01: 85 c0                       	testl	%eax, %eax
14234ba03: 0f 85 43 07 00 00           	jne	0x14234c14c <.text+0x234b14c>
14234ba09: 48 8b 0f                    	movq	(%rdi), %rcx
14234ba0c: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14234ba10: ff 50 18                    	callq	*0x18(%rax)
14234ba13: 48 85 c0                    	testq	%rax, %rax
14234ba16: 0f 84 72 07 00 00           	je	0x14234c18e <.text+0x234b18e>
14234ba1c: 48 83 78 08 00              	cmpq	$0x0, 0x8(%rax)
14234ba21: 0f 84 73 07 00 00           	je	0x14234c19a <.text+0x234b19a>
14234ba27: 0f b6 f3                    	movzbl	%bl, %esi
14234ba2a: 48 8b 38                    	movq	(%rax), %rdi
14234ba2d: 8b 5f 0c                    	movl	0xc(%rdi), %ebx
14234ba30: 8b 85 54 02 00 00           	movl	0x254(%rbp), %eax
14234ba36: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
14234ba3d: 89 01                       	movl	%eax, (%rcx)
14234ba3f: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234ba46: e8 f5 57 5c fe              	callq	0x140911240 <.text+0x910240>
14234ba4b: 8b 08                       	movl	(%rax), %ecx
14234ba4d: 83 f9 02                    	cmpl	$0x2, %ecx
14234ba50: 73 47                       	jae	0x14234ba99 <.text+0x234aa99>
14234ba52: 39 f3                       	cmpl	%esi, %ebx
14234ba54: 0f 95 c2                    	setne	%dl
14234ba57: 85 db                       	testl	%ebx, %ebx
14234ba59: 41 0f 94 c0                 	sete	%r8b
14234ba5d: 41 08 d0                    	orb	%dl, %r8b
14234ba60: 74 0a                       	je	0x14234ba6c <.text+0x234aa6c>
14234ba62: 85 c9                       	testl	%ecx, %ecx
14234ba64: 75 33                       	jne	0x14234ba99 <.text+0x234aa99>
14234ba66: 41 89 76 04                 	movl	%esi, 0x4(%r14)
14234ba6a: eb 2d                       	jmp	0x14234ba99 <.text+0x234aa99>
14234ba6c: 8b 4f 08                    	movl	0x8(%rdi), %ecx
14234ba6f: 89 8d c8 00 00 00           	movl	%ecx, 0xc8(%rbp)
14234ba75: 48 8b 0f                    	movq	(%rdi), %rcx
14234ba78: 48 89 8d c0 00 00 00        	movq	%rcx, 0xc0(%rbp)
14234ba7f: c7 00 01 00 00 00           	movl	$0x1, (%rax)
14234ba85: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
14234ba8c: 48 89 48 04                 	movq	%rcx, 0x4(%rax)
14234ba90: 8b 8d c8 00 00 00           	movl	0xc8(%rbp), %ecx
14234ba96: 89 48 0c                    	movl	%ecx, 0xc(%rax)
14234ba99: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
14234baa0: f3 44 0f 10 10              	movss	(%rax), %xmm10
14234baa5: f3 44 0f 10 48 04           	movss	0x4(%rax), %xmm9
14234baab: 48 8b 9d 98 01 00 00        	movq	0x198(%rbp), %rbx
14234bab2: 48 89 d9                    	movq	%rbx, %rcx
14234bab5: e8 56 e1 5f fe              	callq	0x140949c10 <.text+0x948c10>
14234baba: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
14234babe: 0f 28 f9                    	movaps	%xmm1, %xmm7
14234bac1: 48 89 d9                    	movq	%rbx, %rcx
14234bac4: e8 57 e1 5f fe              	callq	0x140949c20 <.text+0x948c20>
14234bac9: 0f 28 f0                    	movaps	%xmm0, %xmm6
14234bacc: 8b 85 54 02 00 00           	movl	0x254(%rbp), %eax
14234bad2: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
14234bad9: 89 01                       	movl	%eax, (%rcx)
14234badb: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234bae2: e8 59 57 5c fe              	callq	0x140911240 <.text+0x910240>
14234bae7: 48 89 c7                    	movq	%rax, %rdi
14234baea: 8b 08                       	movl	(%rax), %ecx
14234baec: 83 c1 fe                    	addl	$-0x2, %ecx
14234baef: 83 f9 02                    	cmpl	$0x2, %ecx
14234baf2: b8 02 00 00 00              	movl	$0x2, %eax
14234baf7: 0f 42 c1                    	cmovbl	%ecx, %eax
14234bafa: 85 c0                       	testl	%eax, %eax
14234bafc: 74 72                       	je	0x14234bb70 <.text+0x234ab70>
14234bafe: f3 45 0f 5e d0              	divss	%xmm8, %xmm10
14234bb03: f3 44 0f 5e cf              	divss	%xmm7, %xmm9
14234bb08: f3 45 0f 58 d2              	addss	%xmm10, %xmm10
14234bb0d: f3 45 0f 58 c9              	addss	%xmm9, %xmm9
14234bb12: f3 0f 10 05 ca e5 60 00     	movss	0x60e5ca(%rip), %xmm0   # 0x14295a0e4
14234bb1a: f3 44 0f 58 d0              	addss	%xmm0, %xmm10
14234bb1f: f3 44 0f 58 c8              	addss	%xmm0, %xmm9
14234bb24: f3 41 0f 59 f2              	mulss	%xmm10, %xmm6
14234bb29: 44 0f 57 0d bf 9d 5d 00     	xorps	0x5d9dbf(%rip), %xmm9   # 0x1429258f0
14234bb31: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14234bb35: 83 f8 01                    	cmpl	$0x1, %eax
14234bb38: 75 4d                       	jne	0x14234bb87 <.text+0x234ab87>
14234bb3a: e8 d1 23 5e fe              	callq	0x14092df10 <.text+0x92cf10>
14234bb3f: 04 fe                       	addb	$-0x2, %al
14234bb41: 0f b6 c0                    	movzbl	%al, %eax
14234bb44: 3c 03                       	cmpb	$0x3, %al
14234bb46: b9 03 00 00 00              	movl	$0x3, %ecx
14234bb4b: 0f 42 c8                    	cmovbl	%eax, %ecx
14234bb4e: fe c9                       	decb	%cl
14234bb50: 80 f9 02                    	cmpb	$0x2, %cl
14234bb53: 73 72                       	jae	0x14234bbc7 <.text+0x234abc7>
14234bb55: f3 0f 11 77 04              	movss	%xmm6, 0x4(%rdi)
14234bb5a: f3 44 0f 11 4f 08           	movss	%xmm9, 0x8(%rdi)
14234bb60: 41 80 7e 0d 00              	cmpb	$0x0, 0xd(%r14)
14234bb65: 0f 85 28 01 00 00           	jne	0x14234bc93 <.text+0x234ac93>
14234bb6b: e9 42 04 00 00              	jmp	0x14234bfb2 <.text+0x234afb2>
14234bb70: 0f 57 f6                    	xorps	%xmm6, %xmm6
14234bb73: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
14234bb77: 41 80 7e 0d 00              	cmpb	$0x0, 0xd(%r14)
14234bb7c: 0f 85 11 01 00 00           	jne	0x14234bc93 <.text+0x234ac93>
14234bb82: e9 2b 04 00 00              	jmp	0x14234bfb2 <.text+0x234afb2>
14234bb87: e8 84 23 5e fe              	callq	0x14092df10 <.text+0x92cf10>
14234bb8c: 04 fe                       	addb	$-0x2, %al
14234bb8e: 0f b6 c0                    	movzbl	%al, %eax
14234bb91: 3c 03                       	cmpb	$0x3, %al
14234bb93: b9 03 00 00 00              	movl	$0x3, %ecx
14234bb98: 0f 42 c8                    	cmovbl	%eax, %ecx
14234bb9b: fe c9                       	decb	%cl
14234bb9d: 80 f9 02                    	cmpb	$0x2, %cl
14234bba0: 73 40                       	jae	0x14234bbe2 <.text+0x234abe2>
14234bba2: 41 89 76 04                 	movl	%esi, 0x4(%r14)
14234bba6: f3 0f 11 77 10              	movss	%xmm6, 0x10(%rdi)
14234bbab: f3 44 0f 11 4f 14           	movss	%xmm9, 0x14(%rdi)
14234bbb1: c7 07 00 00 00 00           	movl	$0x0, (%rdi)
14234bbb7: 41 80 7e 0d 00              	cmpb	$0x0, 0xd(%r14)
14234bbbc: 0f 85 d1 00 00 00           	jne	0x14234bc93 <.text+0x234ac93>
14234bbc2: e9 eb 03 00 00              	jmp	0x14234bfb2 <.text+0x234afb2>
14234bbc7: f3 0f 10 77 04              	movss	0x4(%rdi), %xmm6
14234bbcc: f3 44 0f 10 4f 08           	movss	0x8(%rdi), %xmm9
14234bbd2: 41 80 7e 0d 00              	cmpb	$0x0, 0xd(%r14)
14234bbd7: 0f 85 b6 00 00 00           	jne	0x14234bc93 <.text+0x234ac93>
14234bbdd: e9 d0 03 00 00              	jmp	0x14234bfb2 <.text+0x234afb2>
14234bbe2: f6 07 01                    	testb	$0x1, (%rdi)
14234bbe5: 0f 84 92 00 00 00           	je	0x14234bc7d <.text+0x234ac7d>
14234bbeb: 0f 10 47 04                 	movups	0x4(%rdi), %xmm0
14234bbef: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14234bbf3: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
14234bbfa: 0f 59 80 80 00 00 00        	mulps	0x80(%rax), %xmm0
14234bc01: 0f 10 4f 08                 	movups	0x8(%rdi), %xmm1
14234bc05: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
14234bc09: 0f 59 88 90 00 00 00        	mulps	0x90(%rax), %xmm1
14234bc10: 0f 10 7f 0c                 	movups	0xc(%rdi), %xmm7
14234bc14: 0f c6 ff 00                 	shufps	$0x0, %xmm7, %xmm7      # xmm7 = xmm7[0,0,0,0]
14234bc18: 0f 59 b8 a0 00 00 00        	mulps	0xa0(%rax), %xmm7
14234bc1f: 0f 58 c8                    	addps	%xmm0, %xmm1
14234bc22: 0f 58 f9                    	addps	%xmm1, %xmm7
14234bc25: 0f 58 b8 b0 00 00 00        	addps	0xb0(%rax), %xmm7
14234bc2c: 0f 28 c7                    	movaps	%xmm7, %xmm0
14234bc2f: 0f c6 c7 00                 	shufps	$0x0, %xmm7, %xmm0      # xmm0 = xmm0[0,0],xmm7[0,0]
14234bc33: 0f 59 00                    	mulps	(%rax), %xmm0
14234bc36: 0f 28 cf                    	movaps	%xmm7, %xmm1
14234bc39: 0f c6 cf 55                 	shufps	$0x55, %xmm7, %xmm1     # xmm1 = xmm1[1,1],xmm7[1,1]
14234bc3d: 0f 59 48 10                 	mulps	0x10(%rax), %xmm1
14234bc41: 0f c6 ff aa                 	shufps	$0xaa, %xmm7, %xmm7     # xmm7 = xmm7[2,2,2,2]
14234bc45: 0f 59 78 20                 	mulps	0x20(%rax), %xmm7
14234bc49: 0f 58 c8                    	addps	%xmm0, %xmm1
14234bc4c: 0f 58 f9                    	addps	%xmm1, %xmm7
14234bc4f: 0f 58 78 30                 	addps	0x30(%rax), %xmm7
14234bc53: 0f 28 c7                    	movaps	%xmm7, %xmm0
14234bc56: 0f c6 c7 ff                 	shufps	$0xff, %xmm7, %xmm0     # xmm0 = xmm0[3,3],xmm7[3,3]
14234bc5a: 0f 5e f8                    	divps	%xmm0, %xmm7
14234bc5d: f3 44 0f 16 cf              	movshdup	%xmm7, %xmm9            # xmm9 = xmm7[1,1,3,3]
14234bc62: 48 89 d9                    	movq	%rbx, %rcx
14234bc65: e8 b6 df 5f fe              	callq	0x140949c20 <.text+0x948c20>
14234bc6a: 0f 28 f0                    	movaps	%xmm0, %xmm6
14234bc6d: f3 0f 59 f7                 	mulss	%xmm7, %xmm6
14234bc71: 41 80 7e 0d 00              	cmpb	$0x0, 0xd(%r14)
14234bc76: 75 1b                       	jne	0x14234bc93 <.text+0x234ac93>
14234bc78: e9 35 03 00 00              	jmp	0x14234bfb2 <.text+0x234afb2>
14234bc7d: f3 0f 10 77 10              	movss	0x10(%rdi), %xmm6
14234bc82: f3 44 0f 10 4f 14           	movss	0x14(%rdi), %xmm9
14234bc88: 41 80 7e 0d 00              	cmpb	$0x0, 0xd(%r14)
14234bc8d: 0f 84 1f 03 00 00           	je	0x14234bfb2 <.text+0x234afb2>
14234bc93: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
14234bc9a: e8 41 3f 5f fe              	callq	0x14093fbe0 <.text+0x93ebe0>
14234bc9f: 84 c0                       	testb	%al, %al
14234bca1: 0f 84 0b 03 00 00           	je	0x14234bfb2 <.text+0x234afb2>
14234bca7: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234bcae: e8 7d 55 5c fe              	callq	0x140911230 <.text+0x910230>
14234bcb3: 04 fd                       	addb	$-0x3, %al
14234bcb5: 3c 02                       	cmpb	$0x2, %al
14234bcb7: 0f 82 f5 02 00 00           	jb	0x14234bfb2 <.text+0x234afb2>
14234bcbd: 48 89 d9                    	movq	%rbx, %rcx
14234bcc0: e8 5b df 5f fe              	callq	0x140949c20 <.text+0x948c20>
14234bcc5: 0f 28 f8                    	movaps	%xmm0, %xmm7
14234bcc8: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234bccf: e8 6c 55 5c fe              	callq	0x140911240 <.text+0x910240>
14234bcd4: f3 44 0f 10 40 18           	movss	0x18(%rax), %xmm8
14234bcda: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
14234bce1: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14234bce4: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14234bce8: 89 01                       	movl	%eax, (%rcx)
14234bcea: 48 8b 3a                    	movq	(%rdx), %rdi
14234bced: 48 8d 05 04 35 d0 00        	leaq	0xd03504(%rip), %rax    # 0x14304f1f8
14234bcf4: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14234bcfb: 48 c7 85 c8 00 00 00 0c 00 00 00    	movq	$0xc, 0xc8(%rbp)
14234bd06: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
14234bd0d: e8 8e 20 27 fe              	callq	0x1405bdda0 <.text+0x5bcda0>
14234bd12: 48 bb 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rbx # imm = 0xB1695320004FB443
14234bd1c: 48 31 c3                    	xorq	%rax, %rbx
14234bd1f: 48 89 f9                    	movq	%rdi, %rcx
14234bd22: 48 89 da                    	movq	%rbx, %rdx
14234bd25: e8 96 81 33 fe              	callq	0x140683ec0 <.text+0x682ec0>
14234bd2a: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
14234bd2e: f6 c2 01                    	testb	$0x1, %dl
14234bd31: 75 7e                       	jne	0x14234bdb1 <.text+0x234adb1>
14234bd33: f3 0f 58 f7                 	addss	%xmm7, %xmm6
14234bd37: f3 0f 10 05 c5 9b 5d 00     	movss	0x5d9bc5(%rip), %xmm0   # 0x142925904
14234bd3f: f3 41 0f 5c c1              	subss	%xmm9, %xmm0
14234bd44: 0f 14 f0                    	unpcklps	%xmm0, %xmm6            # xmm6 = xmm6[0],xmm0[0],xmm6[1],xmm0[1]
14234bd47: 0f 59 35 92 9b 5d 00        	mulps	0x5d9b92(%rip), %xmm6   # 0x1429258e0
14234bd4e: 0f 59 35 8b c1 c3 00        	mulps	0xc3c18b(%rip), %xmm6   # 0x142f87ee0
14234bd55: f3 44 0f 59 05 d2 f8 60 00  	mulss	0x60f8d2(%rip), %xmm8   # 0x14295b630
14234bd5e: 48 8b 77 10                 	movq	0x10(%rdi), %rsi
14234bd62: 48 3b 37                    	cmpq	(%rdi), %rsi
14234bd65: 75 0f                       	jne	0x14234bd76 <.text+0x234ad76>
14234bd67: 48 8d 15 5a 32 86 00        	leaq	0x86325a(%rip), %rdx    # 0x142baefc8
14234bd6e: 48 89 f9                    	movq	%rdi, %rcx
14234bd71: e8 5a 5c 2e fe              	callq	0x1406319d0 <.text+0x6309d0>
14234bd76: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14234bd7a: 48 6b ce 38                 	imulq	$0x38, %rsi, %rcx
14234bd7e: c7 04 08 04 00 00 00        	movl	$0x4, (%rax,%rcx)
14234bd85: f3 0f 7e c6                 	movq	%xmm6, %xmm0            # xmm0 = xmm6[0],zero
14234bd89: f3 0f 7f 44 08 04           	movdqu	%xmm0, 0x4(%rax,%rcx)
14234bd8f: c7 44 08 14 01 00 00 00     	movl	$0x1, 0x14(%rax,%rcx)
14234bd97: f3 44 0f 11 44 08 18        	movss	%xmm8, 0x18(%rax,%rcx)
14234bd9e: f3 44 0f 11 44 08 1c        	movss	%xmm8, 0x1c(%rax,%rcx)
14234bda5: 48 89 5c 08 30              	movq	%rbx, 0x30(%rax,%rcx)
14234bdaa: 48 ff c6                    	incq	%rsi
14234bdad: 48 89 77 10                 	movq	%rsi, 0x10(%rdi)
14234bdb1: 48 89 7d 10                 	movq	%rdi, 0x10(%rbp)
14234bdb5: 48 89 5d 18                 	movq	%rbx, 0x18(%rbp)
14234bdb9: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
14234bdc1: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234bdc8: e8 73 54 5c fe              	callq	0x140911240 <.text+0x910240>
14234bdcd: 83 38 02                    	cmpl	$0x2, (%rax)
14234bdd0: 48 8d 05 2d 34 d0 00        	leaq	0xd0342d(%rip), %rax    # 0x14304f204
14234bdd7: 4c 8d 05 3f 34 d0 00        	leaq	0xd0343f(%rip), %r8     # 0x14304f21d
14234bdde: 4c 0f 42 c0                 	cmovbq	%rax, %r8
14234bde2: b8 19 00 00 00              	movl	$0x19, %eax
14234bde7: 41 b9 12 00 00 00           	movl	$0x12, %r9d
14234bded: 4c 0f 42 c8                 	cmovbq	%rax, %r9
14234bdf1: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
14234bdfa: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
14234be01: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
14234be05: e8 66 2b c5 fe              	callq	0x140f9e970 <.text+0xf9d970>
14234be0a: 48 8b b5 d0 00 00 00        	movq	0xd0(%rbp), %rsi
14234be11: 48 83 be b8 01 00 00 00     	cmpq	$0x0, 0x1b8(%rsi)
14234be19: 0f 84 7f 01 00 00           	je	0x14234bf9e <.text+0x234af9e>
14234be1f: 48 8d bd d8 00 00 00        	leaq	0xd8(%rbp), %rdi
14234be26: 48 8d 8e c0 01 00 00        	leaq	0x1c0(%rsi), %rcx
14234be2d: 48 89 fa                    	movq	%rdi, %rdx
14234be30: e8 3b 4f 26 fe              	callq	0x1405b0d70 <.text+0x5afd70>
14234be35: 48 8b 8e a0 01 00 00        	movq	0x1a0(%rsi), %rcx
14234be3c: 48 8b 96 a8 01 00 00        	movq	0x1a8(%rsi), %rdx
14234be43: 49 89 d0                    	movq	%rdx, %r8
14234be46: 49 21 c0                    	andq	%rax, %r8
14234be49: 48 c1 e8 39                 	shrq	$0x39, %rax
14234be4d: 66 0f 6e c0                 	movd	%eax, %xmm0
14234be51: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14234be55: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14234be5a: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14234be5f: 48 8d 81 40 ff ff ff        	leaq	-0xc0(%rcx), %rax
14234be66: 45 31 c9                    	xorl	%r9d, %r9d
14234be69: 4c 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %r10
14234be70: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14234be74: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
14234be7a: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14234be7e: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14234be82: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
14234be87: 45 85 db                    	testl	%r11d, %r11d
14234be8a: 74 29                       	je	0x14234beb5 <.text+0x234aeb5>
14234be8c: f3 41 0f bc f3              	tzcntl	%r11d, %esi
14234be91: 4c 01 c6                    	addq	%r8, %rsi
14234be94: 48 21 d6                    	andq	%rdx, %rsi
14234be97: 48 f7 de                    	negq	%rsi
14234be9a: 48 8d 34 76                 	leaq	(%rsi,%rsi,2), %rsi
14234be9e: 48 c1 e6 06                 	shlq	$0x6, %rsi
14234bea2: 4c 3b 14 30                 	cmpq	(%rax,%rsi), %r10
14234bea6: 74 2f                       	je	0x14234bed7 <.text+0x234aed7>
14234bea8: 41 8d 73 ff                 	leal	-0x1(%r11), %esi
14234beac: 66 44 21 de                 	andw	%r11w, %si
14234beb0: 41 89 f3                    	movl	%esi, %r11d
14234beb3: 75 d7                       	jne	0x14234be8c <.text+0x234ae8c>
14234beb5: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14234beb9: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
14234bebe: 45 85 db                    	testl	%r11d, %r11d
14234bec1: 0f 85 d7 00 00 00           	jne	0x14234bf9e <.text+0x234af9e>
14234bec7: 4d 01 c8                    	addq	%r9, %r8
14234beca: 49 83 c0 10                 	addq	$0x10, %r8
14234bece: 49 83 c1 10                 	addq	$0x10, %r9
14234bed2: 49 21 d0                    	andq	%rdx, %r8
14234bed5: eb 9d                       	jmp	0x14234be74 <.text+0x234ae74>
14234bed7: 80 4c 31 ff 02              	orb	$0x2, -0x1(%rcx,%rsi)
14234bedc: 48 8b b5 d0 00 00 00        	movq	0xd0(%rbp), %rsi
14234bee3: 48 83 be b8 01 00 00 00     	cmpq	$0x0, 0x1b8(%rsi)
14234beeb: 0f 84 ad 00 00 00           	je	0x14234bf9e <.text+0x234af9e>
14234bef1: 48 8d 8e c0 01 00 00        	leaq	0x1c0(%rsi), %rcx
14234bef8: 48 89 fa                    	movq	%rdi, %rdx
14234befb: e8 70 4e 26 fe              	callq	0x1405b0d70 <.text+0x5afd70>
14234bf00: 48 8b 8e a0 01 00 00        	movq	0x1a0(%rsi), %rcx
14234bf07: 48 8b 96 a8 01 00 00        	movq	0x1a8(%rsi), %rdx
14234bf0e: 49 89 d0                    	movq	%rdx, %r8
14234bf11: 49 21 c0                    	andq	%rax, %r8
14234bf14: 48 c1 e8 39                 	shrq	$0x39, %rax
14234bf18: 66 0f 6e c0                 	movd	%eax, %xmm0
14234bf1c: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14234bf20: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14234bf25: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14234bf2a: 48 8d 81 40 ff ff ff        	leaq	-0xc0(%rcx), %rax
14234bf31: 45 31 c9                    	xorl	%r9d, %r9d
14234bf34: 4c 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %r10
14234bf3b: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14234bf3f: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
14234bf45: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14234bf49: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14234bf4d: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
14234bf52: 45 85 db                    	testl	%r11d, %r11d
14234bf55: 74 29                       	je	0x14234bf80 <.text+0x234af80>
14234bf57: f3 41 0f bc f3              	tzcntl	%r11d, %esi
14234bf5c: 4c 01 c6                    	addq	%r8, %rsi
14234bf5f: 48 21 d6                    	andq	%rdx, %rsi
14234bf62: 48 f7 de                    	negq	%rsi
14234bf65: 48 8d 34 76                 	leaq	(%rsi,%rsi,2), %rsi
14234bf69: 48 c1 e6 06                 	shlq	$0x6, %rsi
14234bf6d: 4c 3b 14 30                 	cmpq	(%rax,%rsi), %r10
14234bf71: 74 37                       	je	0x14234bfaa <.text+0x234afaa>
14234bf73: 41 8d 73 ff                 	leal	-0x1(%r11), %esi
14234bf77: 66 44 21 de                 	andw	%r11w, %si
14234bf7b: 41 89 f3                    	movl	%esi, %r11d
14234bf7e: 75 d7                       	jne	0x14234bf57 <.text+0x234af57>
14234bf80: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14234bf84: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
14234bf89: 45 85 db                    	testl	%r11d, %r11d
14234bf8c: 75 10                       	jne	0x14234bf9e <.text+0x234af9e>
14234bf8e: 4d 01 c8                    	addq	%r9, %r8
14234bf91: 49 83 c0 10                 	addq	$0x10, %r8
14234bf95: 49 83 c1 10                 	addq	$0x10, %r9
14234bf99: 49 21 d0                    	andq	%rdx, %r8
14234bf9c: eb a1                       	jmp	0x14234bf3f <.text+0x234af3f>
14234bf9e: 48 8d 0d 4b c0 6c 00        	leaq	0x6cc04b(%rip), %rcx    # 0x142a17ff0
14234bfa5: e8 66 d3 58 00              	callq	0x1428d9310 <.text+0x28d8310>
14234bfaa: c7 44 31 f8 00 00 80 3f     	movl	$0x3f800000, -0x8(%rcx,%rsi) # imm = 0x3F800000
14234bfb2: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234bfb9: e8 72 52 5c fe              	callq	0x140911230 <.text+0x910230>
14234bfbe: 3c 03                       	cmpb	$0x3, %al
14234bfc0: 0f 85 d8 00 00 00           	jne	0x14234c09e <.text+0x234b09e>
14234bfc6: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
14234bfcd: e8 ae 96 57 fe              	callq	0x1408c5680 <.text+0x8c4680>
14234bfd2: 84 c0                       	testb	%al, %al
14234bfd4: 0f 84 c4 00 00 00           	je	0x14234c09e <.text+0x234b09e>
14234bfda: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
14234bfe1: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14234bfe4: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14234bfe8: 89 01                       	movl	%eax, (%rcx)
14234bfea: 48 8b 3a                    	movq	(%rdx), %rdi
14234bfed: 48 8d 05 3b 32 d0 00        	leaq	0xd0323b(%rip), %rax    # 0x14304f22f
14234bff4: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14234bffb: 48 c7 85 c8 00 00 00 0c 00 00 00    	movq	$0xc, 0xc8(%rbp)
14234c006: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
14234c00d: e8 8e 1d 27 fe              	callq	0x1405bdda0 <.text+0x5bcda0>
14234c012: 48 bb 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rbx # imm = 0xB1695320004FB443
14234c01c: 48 31 c3                    	xorq	%rax, %rbx
14234c01f: 48 89 f9                    	movq	%rdi, %rcx
14234c022: 48 89 da                    	movq	%rbx, %rdx
14234c025: e8 96 7e 33 fe              	callq	0x140683ec0 <.text+0x682ec0>
14234c02a: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
14234c02e: f6 c2 01                    	testb	$0x1, %dl
14234c031: 75 33                       	jne	0x14234c066 <.text+0x234b066>
14234c033: 48 8b 77 10                 	movq	0x10(%rdi), %rsi
14234c037: 48 3b 37                    	cmpq	(%rdi), %rsi
14234c03a: 75 0f                       	jne	0x14234c04b <.text+0x234b04b>
14234c03c: 48 8d 15 85 2f 86 00        	leaq	0x862f85(%rip), %rdx    # 0x142baefc8
14234c043: 48 89 f9                    	movq	%rdi, %rcx
14234c046: e8 85 59 2e fe              	callq	0x1406319d0 <.text+0x6309d0>
14234c04b: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14234c04f: 48 6b ce 38                 	imulq	$0x38, %rsi, %rcx
14234c053: c7 04 08 01 00 00 00        	movl	$0x1, (%rax,%rcx)
14234c05a: 48 89 5c 08 30              	movq	%rbx, 0x30(%rax,%rcx)
14234c05f: 48 ff c6                    	incq	%rsi
14234c062: 48 89 77 10                 	movq	%rsi, 0x10(%rdi)
14234c066: 48 89 7d 10                 	movq	%rdi, 0x10(%rbp)
14234c06a: 48 89 5d 18                 	movq	%rbx, 0x18(%rbp)
14234c06e: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
14234c076: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
14234c07f: 4c 8d 05 b5 31 d0 00        	leaq	0xd031b5(%rip), %r8     # 0x14304f23b
14234c086: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
14234c08d: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
14234c091: 41 b9 12 00 00 00           	movl	$0x12, %r9d
14234c097: e8 d4 28 c5 fe              	callq	0x140f9e970 <.text+0xf9d970>
14234c09c: eb 60                       	jmp	0x14234c0fe <.text+0x234b0fe>
14234c09e: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234c0a5: e8 86 51 5c fe              	callq	0x140911230 <.text+0x910230>
14234c0aa: 3c 03                       	cmpb	$0x3, %al
14234c0ac: 75 50                       	jne	0x14234c0fe <.text+0x234b0fe>
14234c0ae: 48 8b 05 73 ff 2d 01        	movq	0x12dff73(%rip), %rax   # 0x14362c028
14234c0b5: 80 38 01                    	cmpb	$0x1, (%rax)
14234c0b8: 75 44                       	jne	0x14234c0fe <.text+0x234b0fe>
14234c0ba: 48 8b 95 68 03 00 00        	movq	0x368(%rbp), %rdx
14234c0c1: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14234c0c4: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14234c0c8: 89 01                       	movl	%eax, (%rcx)
14234c0ca: 48 8b 32                    	movq	(%rdx), %rsi
14234c0cd: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
14234c0d1: 48 8b 5e 40                 	movq	0x40(%rsi), %rbx
14234c0d5: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
14234c0d9: 75 10                       	jne	0x14234c0eb <.text+0x234b0eb>
14234c0db: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
14234c0df: 48 8d 15 8a 91 86 00        	leaq	0x86918a(%rip), %rdx    # 0x142bb5270
14234c0e6: e8 85 67 21 fe              	callq	0x140562870 <.text+0x561870>
14234c0eb: 48 8b 46 28                 	movq	0x28(%rsi), %rax
14234c0ef: 48 89 1c f8                 	movq	%rbx, (%rax,%rdi,8)
14234c0f3: 48 ff c7                    	incq	%rdi
14234c0f6: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
14234c0fa: 48 ff 46 40                 	incq	0x40(%rsi)
14234c0fe: 41 0f b6 44 24 01           	movzbl	0x1(%r12), %eax
14234c104: 41 8b 4f 1c                 	movl	0x1c(%r15), %ecx
14234c108: 49 8b 57 10                 	movq	0x10(%r15), %rdx
14234c10c: 89 0a                       	movl	%ecx, (%rdx)
14234c10e: 41 88 04 24                 	movb	%al, (%r12)
14234c112: 0f 28 b5 70 02 00 00        	movaps	0x270(%rbp), %xmm6
14234c119: 0f 28 bd 80 02 00 00        	movaps	0x280(%rbp), %xmm7
14234c120: 44 0f 28 85 90 02 00 00     	movaps	0x290(%rbp), %xmm8
14234c128: 44 0f 28 8d a0 02 00 00     	movaps	0x2a0(%rbp), %xmm9
14234c130: 44 0f 28 95 b0 02 00 00     	movaps	0x2b0(%rbp), %xmm10
14234c138: 48 81 c4 48 03 00 00        	addq	$0x348, %rsp            # imm = 0x348
14234c13f: 5b                          	popq	%rbx
14234c140: 5f                          	popq	%rdi
14234c141: 5e                          	popq	%rsi
14234c142: 41 5c                       	popq	%r12
14234c144: 41 5d                       	popq	%r13
14234c146: 41 5e                       	popq	%r14
14234c148: 41 5f                       	popq	%r15
14234c14a: 5d                          	popq	%rbp
14234c14b: c3                          	retq
14234c14c: 48 c7 85 c0 00 00 00 00 00 00 00    	movq	$0x0, 0xc0(%rbp)
14234c157: 48 83 c7 10                 	addq	$0x10, %rdi
14234c15b: 48 8d 05 86 9c d0 00        	leaq	0xd09c86(%rip), %rax    # 0x143055de8
14234c162: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14234c167: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
14234c16b: 4c 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %r9
14234c172: 31 c9                       	xorl	%ecx, %ecx
14234c174: 49 89 f8                    	movq	%rdi, %r8
14234c177: e8 c5 e9 57 00              	callq	0x1428cab41 <.text+0x28c9b41>
14234c17c: 4c 8d 05 dd 2f d0 00        	leaq	0xd02fdd(%rip), %r8     # 0x14304f160
14234c183: 0f 28 c7                    	movaps	%xmm7, %xmm0
14234c186: 0f 28 ce                    	movaps	%xmm6, %xmm1
14234c189: e8 f2 f2 a3 fe              	callq	0x140d8b480 <.text+0xd8a480>
14234c18e: 48 8d 0d 33 30 d0 00        	leaq	0xd03033(%rip), %rcx    # 0x14304f1c8
14234c195: e8 76 d1 58 00              	callq	0x1428d9310 <.text+0x28d8310>
14234c19a: 4c 8d 05 3f 30 d0 00        	leaq	0xd0303f(%rip), %r8     # 0x14304f1e0
14234c1a1: 31 c9                       	xorl	%ecx, %ecx
14234c1a3: 31 d2                       	xorl	%edx, %edx
14234c1a5: e8 6e d3 58 00              	callq	0x1428d9518 <.text+0x28d8518>
14234c1aa: cc                          	int3
14234c1ab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14234c1b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14234c1b5: 55                          	pushq	%rbp
14234c1b6: 41 57                       	pushq	%r15
14234c1b8: 41 56                       	pushq	%r14
14234c1ba: 41 55                       	pushq	%r13
14234c1bc: 41 54                       	pushq	%r12
14234c1be: 56                          	pushq	%rsi
14234c1bf: 57                          	pushq	%rdi
14234c1c0: 53                          	pushq	%rbx
14234c1c1: 48 83 ec 78                 	subq	$0x78, %rsp
14234c1c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234c1cc: 44 0f 29 54 24 20           	movaps	%xmm10, 0x20(%rsp)
14234c1d2: 44 0f 29 4c 24 30           	movaps	%xmm9, 0x30(%rsp)
14234c1d8: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
14234c1de: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
14234c1e3: 0f 29 74 24 60              	movaps	%xmm6, 0x60(%rsp)
14234c1e8: 80 bd 67 02 00 00 00        	cmpb	$0x0, 0x267(%rbp)
14234c1ef: 75 2d                       	jne	0x14234c21e <.text+0x234b21e>
14234c1f1: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
14234c1f6: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
14234c1fb: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
14234c201: 44 0f 28 4c 24 30           	movaps	0x30(%rsp), %xmm9
14234c207: 44 0f 28 54 24 20           	movaps	0x20(%rsp), %xmm10
14234c20d: 48 83 c4 78                 	addq	$0x78, %rsp
14234c211: 5b                          	popq	%rbx
14234c212: 5f                          	popq	%rdi
14234c213: 5e                          	popq	%rsi
14234c214: 41 5c                       	popq	%r12
14234c216: 41 5d                       	popq	%r13
14234c218: 41 5e                       	popq	%r14
14234c21a: 41 5f                       	popq	%r15
14234c21c: 5d                          	popq	%rbp
14234c21d: c3                          	retq
14234c21e: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14234c222: e8 49 92 54 fe              	callq	0x140895470 <.text+0x894470>
14234c227: eb c8                       	jmp	0x14234c1f1 <.text+0x234b1f1>
14234c229: cc                          	int3
14234c22a: cc                          	int3
14234c22b: cc                          	int3
14234c22c: cc                          	int3
14234c22d: cc                          	int3
14234c22e: cc                          	int3
14234c22f: cc                          	int3
