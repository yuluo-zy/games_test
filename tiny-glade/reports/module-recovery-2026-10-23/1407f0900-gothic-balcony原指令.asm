
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407f0900: 55                          	pushq	%rbp
1407f0901: 41 57                       	pushq	%r15
1407f0903: 41 56                       	pushq	%r14
1407f0905: 41 55                       	pushq	%r13
1407f0907: 41 54                       	pushq	%r12
1407f0909: 56                          	pushq	%rsi
1407f090a: 57                          	pushq	%rdi
1407f090b: 53                          	pushq	%rbx
1407f090c: 48 81 ec 68 03 00 00        	subq	$0x368, %rsp            # imm = 0x368
1407f0913: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407f091b: 44 0f 29 bd d0 02 00 00     	movaps	%xmm15, 0x2d0(%rbp)
1407f0923: 44 0f 29 b5 c0 02 00 00     	movaps	%xmm14, 0x2c0(%rbp)
1407f092b: 44 0f 29 ad b0 02 00 00     	movaps	%xmm13, 0x2b0(%rbp)
1407f0933: 44 0f 29 a5 a0 02 00 00     	movaps	%xmm12, 0x2a0(%rbp)
1407f093b: 66 44 0f 7f 9d 90 02 00 00  	movdqa	%xmm11, 0x290(%rbp)
1407f0944: 44 0f 29 95 80 02 00 00     	movaps	%xmm10, 0x280(%rbp)
1407f094c: 44 0f 29 8d 70 02 00 00     	movaps	%xmm9, 0x270(%rbp)
1407f0954: 44 0f 29 85 60 02 00 00     	movaps	%xmm8, 0x260(%rbp)
1407f095c: 0f 29 bd 50 02 00 00        	movaps	%xmm7, 0x250(%rbp)
1407f0963: 0f 29 b5 40 02 00 00        	movaps	%xmm6, 0x240(%rbp)
1407f096a: 48 c7 85 38 02 00 00 fe ff ff ff    	movq	$-0x2, 0x238(%rbp)
1407f0975: 4c 89 cb                    	movq	%r9, %rbx
1407f0978: 4c 89 c7                    	movq	%r8, %rdi
1407f097b: 48 89 d6                    	movq	%rdx, %rsi
1407f097e: 48 89 4d 18                 	movq	%rcx, 0x18(%rbp)
1407f0982: e8 79 60 42 00              	callq	0x140c16a00 <.text+0xc15a00>
1407f0987: 88 85 35 02 00 00           	movb	%al, 0x235(%rbp)
1407f098d: 84 c0                       	testb	%al, %al
1407f098f: 0f 84 19 02 00 00           	je	0x1407f0bae <.text+0x7efbae>
1407f0995: 41 be 56 00 00 00           	movl	$0x56, %r14d
1407f099b: b9 56 00 00 00              	movl	$0x56, %ecx
1407f09a0: e8 db fa ff ff              	callq	0x1407f0480 <.text+0x7ef480>
1407f09a5: 4c 8d 3d 9c a7 29 02        	leaq	0x229a79c(%rip), %r15   # 0x142a8b148
1407f09ac: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f09b0: 0f 85 a1 00 00 00           	jne	0x1407f0a57 <.text+0x7efa57>
1407f09b6: 49 89 d1                    	movq	%rdx, %r9
1407f09b9: 48 85 d2                    	testq	%rdx, %rdx
1407f09bc: 74 36                       	je	0x1407f09f4 <.text+0x7ef9f4>
1407f09be: 49 83 f9 56                 	cmpq	$0x56, %r9
1407f09c2: 73 2e                       	jae	0x1407f09f2 <.text+0x7ef9f2>
1407f09c4: 48 8d 05 7d a7 29 02        	leaq	0x229a77d(%rip), %rax   # 0x142a8b148
1407f09cb: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1407f09d0: 7f 22                       	jg	0x1407f09f4 <.text+0x7ef9f4>
1407f09d2: 48 8d 05 3f a8 29 02        	leaq	0x229a83f(%rip), %rax   # 0x142a8b218
1407f09d9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f09de: 48 8d 0d 63 a7 29 02        	leaq	0x229a763(%rip), %rcx   # 0x142a8b148
1407f09e5: ba 56 00 00 00              	movl	$0x56, %edx
1407f09ea: 45 31 c0                    	xorl	%r8d, %r8d
1407f09ed: e8 4e 8f 0e 02              	callq	0x1428d9940 <.text+0x28d8940>
1407f09f2: 75 de                       	jne	0x1407f09d2 <.text+0x7ef9d2>
1407f09f4: 4c 89 c9                    	movq	%r9, %rcx
1407f09f7: e8 84 fa ff ff              	callq	0x1407f0480 <.text+0x7ef480>
1407f09fc: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f0a00: 75 55                       	jne	0x1407f0a57 <.text+0x7efa57>
1407f0a02: 49 89 d0                    	movq	%rdx, %r8
1407f0a05: 49 83 c0 02                 	addq	$0x2, %r8
1407f0a09: 74 39                       	je	0x1407f0a44 <.text+0x7efa44>
1407f0a0b: 49 83 f8 56                 	cmpq	$0x56, %r8
1407f0a0f: 73 31                       	jae	0x1407f0a42 <.text+0x7efa42>
1407f0a11: 48 8d 05 30 a7 29 02        	leaq	0x229a730(%rip), %rax   # 0x142a8b148
1407f0a18: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1407f0a1d: 7f 25                       	jg	0x1407f0a44 <.text+0x7efa44>
1407f0a1f: 48 8d 05 0a a8 29 02        	leaq	0x229a80a(%rip), %rax   # 0x142a8b230
1407f0a26: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f0a2b: 48 8d 0d 16 a7 29 02        	leaq	0x229a716(%rip), %rcx   # 0x142a8b148
1407f0a32: ba 56 00 00 00              	movl	$0x56, %edx
1407f0a37: 41 b9 56 00 00 00           	movl	$0x56, %r9d
1407f0a3d: e8 fe 8e 0e 02              	callq	0x1428d9940 <.text+0x28d8940>
1407f0a42: 75 db                       	jne	0x1407f0a1f <.text+0x7efa1f>
1407f0a44: 41 be 54 00 00 00           	movl	$0x54, %r14d
1407f0a4a: 49 29 d6                    	subq	%rdx, %r14
1407f0a4d: 4c 8d 3d f4 a6 29 02        	leaq	0x229a6f4(%rip), %r15   # 0x142a8b148
1407f0a54: 4d 01 c7                    	addq	%r8, %r15
1407f0a57: 4c 8d 05 74 a8 29 02        	leaq	0x229a874(%rip), %r8    # 0x142a8b2d2
1407f0a5e: 4c 8d 25 29 a8 29 02        	leaq	0x229a829(%rip), %r12   # 0x142a8b28e
1407f0a65: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1407f0a70: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1407f0a75: 85 c0                       	testl	%eax, %eax
1407f0a77: 78 17                       	js	0x1407f0a90 <.text+0x7efa90>
1407f0a79: 49 ff c8                    	decq	%r8
1407f0a7c: 83 f8 5c                    	cmpl	$0x5c, %eax
1407f0a7f: 75 63                       	jne	0x1407f0ae4 <.text+0x7efae4>
1407f0a81: eb 73                       	jmp	0x1407f0af6 <.text+0x7efaf6>
1407f0a83: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407f0a90: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1407f0a95: 80 f9 c0                    	cmpb	$-0x40, %cl
1407f0a98: 7d 23                       	jge	0x1407f0abd <.text+0x7efabd>
1407f0a9a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1407f0a9f: 80 fa c0                    	cmpb	$-0x40, %dl
1407f0aa2: 7d 22                       	jge	0x1407f0ac6 <.text+0x7efac6>
1407f0aa4: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1407f0aa9: 49 83 c0 fc                 	addq	$-0x4, %r8
1407f0aad: 41 83 e1 07                 	andl	$0x7, %r9d
1407f0ab1: 41 c1 e1 06                 	shll	$0x6, %r9d
1407f0ab5: 83 e2 3f                    	andl	$0x3f, %edx
1407f0ab8: 44 09 ca                    	orl	%r9d, %edx
1407f0abb: eb 10                       	jmp	0x1407f0acd <.text+0x7efacd>
1407f0abd: 49 83 c0 fe                 	addq	$-0x2, %r8
1407f0ac1: 83 e1 1f                    	andl	$0x1f, %ecx
1407f0ac4: eb 0f                       	jmp	0x1407f0ad5 <.text+0x7efad5>
1407f0ac6: 49 83 c0 fd                 	addq	$-0x3, %r8
1407f0aca: 83 e2 0f                    	andl	$0xf, %edx
1407f0acd: c1 e2 06                    	shll	$0x6, %edx
1407f0ad0: 83 e1 3f                    	andl	$0x3f, %ecx
1407f0ad3: 09 d1                       	orl	%edx, %ecx
1407f0ad5: c1 e1 06                    	shll	$0x6, %ecx
1407f0ad8: 24 3f                       	andb	$0x3f, %al
1407f0ada: 0f b6 c0                    	movzbl	%al, %eax
1407f0add: 09 c8                       	orl	%ecx, %eax
1407f0adf: 83 f8 5c                    	cmpl	$0x5c, %eax
1407f0ae2: 74 12                       	je	0x1407f0af6 <.text+0x7efaf6>
1407f0ae4: 83 f8 2f                    	cmpl	$0x2f, %eax
1407f0ae7: 74 0d                       	je	0x1407f0af6 <.text+0x7efaf6>
1407f0ae9: 4d 39 e0                    	cmpq	%r12, %r8
1407f0aec: 75 82                       	jne	0x1407f0a70 <.text+0x7efa70>
1407f0aee: 41 bd 44 00 00 00           	movl	$0x44, %r13d
1407f0af4: eb 4d                       	jmp	0x1407f0b43 <.text+0x7efb43>
1407f0af6: 4c 8d 25 91 a7 29 02        	leaq	0x229a791(%rip), %r12   # 0x142a8b28e
1407f0afd: 4d 29 e0                    	subq	%r12, %r8
1407f0b00: 49 ff c0                    	incq	%r8
1407f0b03: 74 32                       	je	0x1407f0b37 <.text+0x7efb37>
1407f0b05: 49 83 f8 44                 	cmpq	$0x44, %r8
1407f0b09: 73 2a                       	jae	0x1407f0b35 <.text+0x7efb35>
1407f0b0b: 43 80 3c 20 bf              	cmpb	$-0x41, (%r8,%r12)
1407f0b10: 7f 25                       	jg	0x1407f0b37 <.text+0x7efb37>
1407f0b12: 48 8d 05 df a6 29 02        	leaq	0x229a6df(%rip), %rax   # 0x142a8b1f8
1407f0b19: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f0b1e: 48 8d 0d 69 a7 29 02        	leaq	0x229a769(%rip), %rcx   # 0x142a8b28e
1407f0b25: ba 44 00 00 00              	movl	$0x44, %edx
1407f0b2a: 41 b9 44 00 00 00           	movl	$0x44, %r9d
1407f0b30: e8 0b 8e 0e 02              	callq	0x1428d9940 <.text+0x28d8940>
1407f0b35: 75 db                       	jne	0x1407f0b12 <.text+0x7efb12>
1407f0b37: 41 bd 44 00 00 00           	movl	$0x44, %r13d
1407f0b3d: 4d 29 c5                    	subq	%r8, %r13
1407f0b40: 4d 01 c4                    	addq	%r8, %r12
1407f0b43: e8 e8 75 42 00              	callq	0x140c18130 <.text+0xc17130>
1407f0b48: 48 89 c1                    	movq	%rax, %rcx
1407f0b4b: 48 8b 00                    	movq	(%rax), %rax
1407f0b4e: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f0b52: 0f 85 88 1f 00 00           	jne	0x1407f2ae0 <.text+0x7f1ae0>
1407f0b58: 48 83 c1 08                 	addq	$0x8, %rcx
1407f0b5c: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407f0b60: 0f 85 c9 1f 00 00           	jne	0x1407f2b2f <.text+0x7f1b2f>
1407f0b66: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407f0b6d: 48 89 8d b0 01 00 00        	movq	%rcx, 0x1b0(%rbp)
1407f0b74: 48 83 c1 08                 	addq	$0x8, %rcx
1407f0b78: 4c 89 6c 24 20              	movq	%r13, 0x20(%rsp)
1407f0b7d: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1407f0b86: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1407f0b8f: 4c 89 fa                    	movq	%r15, %rdx
1407f0b92: 4d 89 f0                    	movq	%r14, %r8
1407f0b95: 4d 89 e1                    	movq	%r12, %r9
1407f0b98: e8 a3 61 42 00              	callq	0x140c16d40 <.text+0xc15d40>
1407f0b9d: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
1407f0ba4: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
1407f0bab: 48 ff 00                    	incq	(%rax)
1407f0bae: 48 89 d9                    	movq	%rbx, %rcx
1407f0bb1: e8 ba 5e 33 00              	callq	0x140b26a70 <.text+0xb25a70>
1407f0bb6: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1407f0bbd: 48 89 95 f8 00 00 00        	movq	%rdx, 0xf8(%rbp)
1407f0bc4: 48 8b 17                    	movq	(%rdi), %rdx
1407f0bc7: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f0bce: e8 2d a5 22 00              	callq	0x140a1b100 <.text+0xa1a100>
1407f0bd3: 48 8b 8d 60 03 00 00        	movq	0x360(%rbp), %rcx
1407f0bda: 48 8b 95 58 03 00 00        	movq	0x358(%rbp), %rdx
1407f0be1: 4c 8b 85 50 03 00 00        	movq	0x350(%rbp), %r8
1407f0be8: 4c 8b b5 e0 01 00 00        	movq	0x1e0(%rbp), %r14
1407f0bef: 48 8b bd e8 01 00 00        	movq	0x1e8(%rbp), %rdi
1407f0bf6: 0f b7 85 f8 01 00 00        	movzwl	0x1f8(%rbp), %eax
1407f0bfd: 4c 8b 8d 00 02 00 00        	movq	0x200(%rbp), %r9
1407f0c04: 45 8b 50 1c                 	movl	0x1c(%r8), %r10d
1407f0c08: 44 89 95 d4 00 00 00        	movl	%r10d, 0xd4(%rbp)
1407f0c0f: 4d 8b 10                    	movq	(%r8), %r10
1407f0c12: 4c 89 55 00                 	movq	%r10, (%rbp)
1407f0c16: 4d 8b 40 10                 	movq	0x10(%r8), %r8
1407f0c1a: 4c 89 45 f8                 	movq	%r8, -0x8(%rbp)
1407f0c1e: 4c 8b 06                    	movq	(%rsi), %r8
1407f0c21: 4c 89 45 08                 	movq	%r8, 0x8(%rbp)
1407f0c25: 49 83 c0 20                 	addq	$0x20, %r8
1407f0c29: 4c 89 45 10                 	movq	%r8, 0x10(%rbp)
1407f0c2d: 48 8b 12                    	movq	(%rdx), %rdx
1407f0c30: 48 89 95 58 01 00 00        	movq	%rdx, 0x158(%rbp)
1407f0c37: 48 8b 09                    	movq	(%rcx), %rcx
1407f0c3a: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
1407f0c3e: 66 45 0f 76 db              	pcmpeqd	%xmm11, %xmm11
1407f0c43: 4c 89 8d d8 00 00 00        	movq	%r9, 0xd8(%rbp)
1407f0c4a: 4d 85 c9                    	testq	%r9, %r9
1407f0c4d: 0f 84 c0 1c 00 00           	je	0x1407f2913 <.text+0x7f1913>
1407f0c53: 66 85 c0                    	testw	%ax, %ax
1407f0c56: 74 18                       	je	0x1407f0c70 <.text+0x7efc70>
1407f0c58: 4d 85 f6                    	testq	%r14, %r14
1407f0c5b: 0f 84 b2 1c 00 00           	je	0x1407f2913 <.text+0x7f1913>
1407f0c61: 8d 48 ff                    	leal	-0x1(%rax), %ecx
1407f0c64: 21 c1                       	andl	%eax, %ecx
1407f0c66: 89 8d 90 01 00 00           	movl	%ecx, 0x190(%rbp)
1407f0c6c: eb 2f                       	jmp	0x1407f0c9d <.text+0x7efc9d>
1407f0c6e: 66 90                       	nop
1407f0c70: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
1407f0c74: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1407f0c78: 49 81 c6 80 f6 ff ff        	addq	$-0x980, %r14           # imm = 0xF680
1407f0c7f: 48 83 c7 10                 	addq	$0x10, %rdi
1407f0c83: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1407f0c88: 74 e6                       	je	0x1407f0c70 <.text+0x7efc70>
1407f0c8a: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1407f0c8f: 29 c1                       	subl	%eax, %ecx
1407f0c91: f7 d0                       	notl	%eax
1407f0c93: 89 c2                       	movl	%eax, %edx
1407f0c95: 21 ca                       	andl	%ecx, %edx
1407f0c97: 89 95 90 01 00 00           	movl	%edx, 0x190(%rbp)
1407f0c9d: f3 0f bc c0                 	tzcntl	%eax, %eax
1407f0ca1: 48 f7 d8                    	negq	%rax
1407f0ca4: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
1407f0cab: 49 8d 14 06                 	leaq	(%r14,%rax), %rdx
1407f0caf: 48 81 c2 70 ff ff ff        	addq	$-0x90, %rdx
1407f0cb6: 49 8b 84 06 68 ff ff ff     	movq	-0x98(%r14,%rax), %rax
1407f0cbe: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
1407f0cc5: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f0ccc: e8 9f a3 22 00              	callq	0x140a1b070 <.text+0xa1a070>
1407f0cd1: 48 ff 8d d8 00 00 00        	decq	0xd8(%rbp)
1407f0cd8: 48 8b b5 e0 01 00 00        	movq	0x1e0(%rbp), %rsi
1407f0cdf: 4c 8b a5 e8 01 00 00        	movq	0x1e8(%rbp), %r12
1407f0ce6: 0f b7 8d f8 01 00 00        	movzwl	0x1f8(%rbp), %ecx
1407f0ced: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407f0cf4: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
1407f0cfb: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
1407f0d02: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
1407f0d09: 48 83 c0 50                 	addq	$0x50, %rax
1407f0d0d: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1407f0d11: 48 83 bd b0 01 00 00 00     	cmpq	$0x0, 0x1b0(%rbp)
1407f0d19: 0f 84 d7 1b 00 00           	je	0x1407f28f6 <.text+0x7f18f6>
1407f0d1f: 66 85 c9                    	testw	%cx, %cx
1407f0d22: 74 1c                       	je	0x1407f0d40 <.text+0x7efd40>
1407f0d24: 8d 41 ff                    	leal	-0x1(%rcx), %eax
1407f0d27: 21 c8                       	andl	%ecx, %eax
1407f0d29: 89 85 dc 01 00 00           	movl	%eax, 0x1dc(%rbp)
1407f0d2f: 48 85 f6                    	testq	%rsi, %rsi
1407f0d32: 75 39                       	jne	0x1407f0d6d <.text+0x7efd6d>
1407f0d34: e9 bd 1b 00 00              	jmp	0x1407f28f6 <.text+0x7f18f6>
1407f0d39: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1407f0d40: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
1407f0d46: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
1407f0d4a: 48 83 c6 c0                 	addq	$-0x40, %rsi
1407f0d4e: 49 83 c4 10                 	addq	$0x10, %r12
1407f0d52: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
1407f0d58: 74 e6                       	je	0x1407f0d40 <.text+0x7efd40>
1407f0d5a: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
1407f0d5f: 29 c8                       	subl	%ecx, %eax
1407f0d61: f7 d1                       	notl	%ecx
1407f0d63: 89 ca                       	movl	%ecx, %edx
1407f0d65: 21 c2                       	andl	%eax, %edx
1407f0d67: 89 95 dc 01 00 00           	movl	%edx, 0x1dc(%rbp)
1407f0d6d: f3 0f bc c1                 	tzcntl	%ecx, %eax
1407f0d71: c1 e0 02                    	shll	$0x2, %eax
1407f0d74: 48 89 f1                    	movq	%rsi, %rcx
1407f0d77: 48 29 c1                    	subq	%rax, %rcx
1407f0d7a: 8b 49 fc                    	movl	-0x4(%rcx), %ecx
1407f0d7d: 89 8d e0 01 00 00           	movl	%ecx, 0x1e0(%rbp)
1407f0d83: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1407f0d8a: 48 83 78 48 00              	cmpq	$0x0, 0x48(%rax)
1407f0d8f: 0f 84 34 1c 00 00           	je	0x1407f29c9 <.text+0x7f19c9>
1407f0d95: 89 8d 90 00 00 00           	movl	%ecx, 0x90(%rbp)
1407f0d9b: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1407f0d9f: 48 8d 95 e0 01 00 00        	leaq	0x1e0(%rbp), %rdx
1407f0da6: e8 25 01 dc ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1407f0dab: 48 ff 8d b0 01 00 00        	decq	0x1b0(%rbp)
1407f0db2: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
1407f0db9: 48 8b 59 30                 	movq	0x30(%rcx), %rbx
1407f0dbd: 48 8b 49 38                 	movq	0x38(%rcx), %rcx
1407f0dc1: 49 89 cb                    	movq	%rcx, %r11
1407f0dc4: 49 21 c3                    	andq	%rax, %r11
1407f0dc7: 48 c1 e8 39                 	shrq	$0x39, %rax
1407f0dcb: 66 0f 6e c0                 	movd	%eax, %xmm0
1407f0dcf: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407f0dd3: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407f0dd8: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f0ddd: 48 8d 43 b0                 	leaq	-0x50(%rbx), %rax
1407f0de1: 45 31 c0                    	xorl	%r8d, %r8d
1407f0de4: 8b 95 90 00 00 00           	movl	0x90(%rbp), %edx
1407f0dea: f3 42 0f 6f 0c 1b           	movdqu	(%rbx,%r11), %xmm1
1407f0df0: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407f0df4: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407f0df8: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1407f0dfd: 45 85 c9                    	testl	%r9d, %r9d
1407f0e00: 74 2e                       	je	0x1407f0e30 <.text+0x7efe30>
1407f0e02: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1407f0e07: 4d 01 da                    	addq	%r11, %r10
1407f0e0a: 49 21 ca                    	andq	%rcx, %r10
1407f0e0d: 49 f7 da                    	negq	%r10
1407f0e10: 4f 8d 3c 92                 	leaq	(%r10,%r10,4), %r15
1407f0e14: 49 c1 e7 04                 	shlq	$0x4, %r15
1407f0e18: 42 3b 14 38                 	cmpl	(%rax,%r15), %edx
1407f0e1c: 74 42                       	je	0x1407f0e60 <.text+0x7efe60>
1407f0e1e: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1407f0e22: 66 45 21 ca                 	andw	%r9w, %r10w
1407f0e26: 45 89 d1                    	movl	%r10d, %r9d
1407f0e29: 75 d7                       	jne	0x1407f0e02 <.text+0x7efe02>
1407f0e2b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1407f0e30: 66 41 0f 74 cb              	pcmpeqb	%xmm11, %xmm1
1407f0e35: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1407f0e3a: 45 85 c9                    	testl	%r9d, %r9d
1407f0e3d: 0f 85 86 1b 00 00           	jne	0x1407f29c9 <.text+0x7f19c9>
1407f0e43: 4d 01 c3                    	addq	%r8, %r11
1407f0e46: 49 83 c3 10                 	addq	$0x10, %r11
1407f0e4a: 49 83 c0 10                 	addq	$0x10, %r8
1407f0e4e: 49 21 cb                    	andq	%rcx, %r11
1407f0e51: eb 97                       	jmp	0x1407f0dea <.text+0x7efdea>
1407f0e53: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407f0e60: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1407f0e67: e8 04 5b 33 00              	callq	0x140b26970 <.text+0xb25970>
1407f0e6c: 8b 8d dc 01 00 00           	movl	0x1dc(%rbp), %ecx
1407f0e72: 3c 09                       	cmpb	$0x9, %al
1407f0e74: 0f 85 97 fe ff ff           	jne	0x1407f0d11 <.text+0x7efd11>
1407f0e7a: 8b 85 d4 00 00 00           	movl	0xd4(%rbp), %eax
1407f0e80: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1407f0e84: 89 01                       	movl	%eax, (%rcx)
1407f0e86: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
1407f0e8d: e8 8e b6 0c 00              	callq	0x1408bc520 <.text+0x8bb520>
1407f0e92: 49 89 d1                    	movq	%rdx, %r9
1407f0e95: 4a 8d 0c 3b                 	leaq	(%rbx,%r15), %rcx
1407f0e99: 48 89 8d c0 01 00 00        	movq	%rcx, 0x1c0(%rbp)
1407f0ea0: 0f b6 49 ff                 	movzbl	-0x1(%rcx), %ecx
1407f0ea4: 88 4c 24 20                 	movb	%cl, 0x20(%rsp)
1407f0ea8: 48 8b 4d 00                 	movq	(%rbp), %rcx
1407f0eac: 8b 95 90 00 00 00           	movl	0x90(%rbp), %edx
1407f0eb2: 49 89 c0                    	movq	%rax, %r8
1407f0eb5: e8 f6 49 03 00              	callq	0x1408258b0 <.text+0x8248b0>
1407f0eba: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
1407f0ec1: 48 8b 95 c0 01 00 00        	movq	0x1c0(%rbp), %rdx
1407f0ec8: 8b 4a e0                    	movl	-0x20(%rdx), %ecx
1407f0ecb: 81 f9 e7 03 00 00           	cmpl	$0x3e7, %ecx            # imm = 0x3E7
1407f0ed1: b8 00 00 00 00              	movl	$0x0, %eax
1407f0ed6: 0f 44 c8                    	cmovel	%eax, %ecx
1407f0ed9: 89 8d 94 01 00 00           	movl	%ecx, 0x194(%rbp)
1407f0edf: 4c 8b 6d 08                 	movq	0x8(%rbp), %r13
1407f0ee3: 49 83 7d 18 00              	cmpq	$0x0, 0x18(%r13)
1407f0ee8: 0f 84 ec 1a 00 00           	je	0x1407f29da <.text+0x7f19da>
1407f0eee: 4a 8d 04 3b                 	leaq	(%rbx,%r15), %rax
1407f0ef2: 48 83 c0 b8                 	addq	$-0x48, %rax
1407f0ef6: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
1407f0efd: 48 8b 9d 88 01 00 00        	movq	0x188(%rbp), %rbx
1407f0f04: 0f b6 42 ff                 	movzbl	-0x1(%rdx), %eax
1407f0f08: 88 85 36 02 00 00           	movb	%al, 0x236(%rbp)
1407f0f0e: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
1407f0f12: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
1407f0f19: e8 52 fe db ff              	callq	0x1405b0d70 <.text+0x5afd70>
1407f0f1e: 4d 8b 7d 00                 	movq	(%r13), %r15
1407f0f22: 49 8b 4d 08                 	movq	0x8(%r13), %rcx
1407f0f26: 48 89 ca                    	movq	%rcx, %rdx
1407f0f29: 48 21 c2                    	andq	%rax, %rdx
1407f0f2c: 48 c1 e8 39                 	shrq	$0x39, %rax
1407f0f30: 66 0f 6e c0                 	movd	%eax, %xmm0
1407f0f34: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407f0f38: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407f0f3d: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f0f42: 49 8d 87 58 fd ff ff        	leaq	-0x2a8(%r15), %rax
1407f0f49: 4c 8b 85 50 01 00 00        	movq	0x150(%rbp), %r8
1407f0f50: 45 31 c9                    	xorl	%r9d, %r9d
1407f0f53: f3 41 0f 6f 0c 17           	movdqu	(%r15,%rdx), %xmm1
1407f0f59: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407f0f5d: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407f0f61: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1407f0f66: 45 85 d2                    	testl	%r10d, %r10d
1407f0f69: 74 28                       	je	0x1407f0f93 <.text+0x7eff93>
1407f0f6b: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1407f0f70: 49 01 d3                    	addq	%rdx, %r11
1407f0f73: 49 21 cb                    	andq	%rcx, %r11
1407f0f76: 49 f7 db                    	negq	%r11
1407f0f79: 4d 69 eb a8 02 00 00        	imulq	$0x2a8, %r11, %r13      # imm = 0x2A8
1407f0f80: 4e 3b 04 28                 	cmpq	(%rax,%r13), %r8
1407f0f84: 74 30                       	je	0x1407f0fb6 <.text+0x7effb6>
1407f0f86: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1407f0f8a: 66 45 21 d3                 	andw	%r10w, %r11w
1407f0f8e: 45 89 da                    	movl	%r11d, %r10d
1407f0f91: 75 d8                       	jne	0x1407f0f6b <.text+0x7eff6b>
1407f0f93: 66 41 0f 74 cb              	pcmpeqb	%xmm11, %xmm1
1407f0f98: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
1407f0f9d: 45 85 d2                    	testl	%r10d, %r10d
1407f0fa0: 0f 85 34 1a 00 00           	jne	0x1407f29da <.text+0x7f19da>
1407f0fa6: 4c 01 ca                    	addq	%r9, %rdx
1407f0fa9: 48 83 c2 10                 	addq	$0x10, %rdx
1407f0fad: 49 83 c1 10                 	addq	$0x10, %r9
1407f0fb1: 48 21 ca                    	andq	%rcx, %rdx
1407f0fb4: eb 9d                       	jmp	0x1407f0f53 <.text+0x7eff53>
1407f0fb6: 4c 89 bd 60 01 00 00        	movq	%r15, 0x160(%rbp)
1407f0fbd: 48 89 9d b8 01 00 00        	movq	%rbx, 0x1b8(%rbp)
1407f0fc4: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
1407f0fcb: 48 8d 15 1e a3 29 02        	leaq	0x229a31e(%rip), %rdx   # 0x142a8b2f0
1407f0fd2: e8 89 22 04 00              	callq	0x140833260 <.text+0x832260>
1407f0fd7: f3 0f 10 38                 	movss	(%rax), %xmm7
1407f0fdb: f3 0f 10 70 04              	movss	0x4(%rax), %xmm6
1407f0fe0: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407f0fe7: 44 8a 78 fc                 	movb	-0x4(%rax), %r15b
1407f0feb: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
1407f0ff2: b1 09                       	movb	$0x9, %cl
1407f0ff4: e8 57 22 04 00              	callq	0x140833250 <.text+0x832250>
1407f0ff9: 48 8b 9d f8 00 00 00        	movq	0xf8(%rbp), %rbx
1407f1000: 44 89 f9                    	movl	%r15d, %ecx
1407f1003: 89 c2                       	movl	%eax, %edx
1407f1005: e8 86 4c 33 00              	callq	0x140b25c90 <.text+0xb24c90>
1407f100a: 88 85 98 01 00 00           	movb	%al, 0x198(%rbp)
1407f1010: 88 95 99 01 00 00           	movb	%dl, 0x199(%rbp)
1407f1016: 48 89 d9                    	movq	%rbx, %rcx
1407f1019: 48 8d 95 98 01 00 00        	leaq	0x198(%rbp), %rdx
1407f1020: e8 6b bf a6 00              	callq	0x14125cf90 <.text+0x125bf90>
1407f1025: 48 85 c0                    	testq	%rax, %rax
1407f1028: 0f 84 70 1b 00 00           	je	0x1407f2b9e <.text+0x7f1b9e>
1407f102e: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
1407f1035: 4a 8d 14 29                 	leaq	(%rcx,%r13), %rdx
1407f1039: 48 81 c2 60 fd ff ff        	addq	$-0x2a0, %rdx           # imm = 0xFD60
1407f1040: 0f 10 40 20                 	movups	0x20(%rax), %xmm0
1407f1044: f3 0f 6f 48 30              	movdqu	0x30(%rax), %xmm1
1407f1049: 66 0f 7f 4d 40              	movdqa	%xmm1, 0x40(%rbp)
1407f104e: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
1407f1052: 66 0f 6e 45 40              	movd	0x40(%rbp), %xmm0
1407f1057: 66 0f 7e 44 24 28           	movd	%xmm0, 0x28(%rsp)
1407f105d: f3 0f 11 74 24 20           	movss	%xmm6, 0x20(%rsp)
1407f1063: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f106a: 4c 8b 85 40 01 00 00        	movq	0x140(%rbp), %r8
1407f1071: 0f 28 df                    	movaps	%xmm7, %xmm3
1407f1074: e8 87 a4 ff ff              	callq	0x1407eb500 <.text+0x7ea500>
1407f1079: f2 44 0f 10 ad f0 01 00 00  	movsd	0x1f0(%rbp), %xmm13
1407f1082: f3 44 0f 10 a5 f8 01 00 00  	movss	0x1f8(%rbp), %xmm12
1407f108b: 48 8d 8d fc 01 00 00        	leaq	0x1fc(%rbp), %rcx
1407f1092: 48 8b 01                    	movq	(%rcx), %rax
1407f1095: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1407f1099: 8b 41 08                    	movl	0x8(%rcx), %eax
1407f109c: 89 45 78                    	movl	%eax, 0x78(%rbp)
1407f109f: 0f 28 85 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm0
1407f10a6: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
1407f10aa: f2 44 0f 10 b5 08 02 00 00  	movsd	0x208(%rbp), %xmm14
1407f10b3: f3 0f 10 bd 10 02 00 00     	movss	0x210(%rbp), %xmm7
1407f10bb: f2 0f 10 85 14 02 00 00     	movsd	0x214(%rbp), %xmm0
1407f10c3: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
1407f10c7: 41 0f 28 c6                 	movaps	%xmm14, %xmm0
1407f10cb: f3 41 0f 5c c5              	subss	%xmm13, %xmm0
1407f10d0: 0f 28 cf                    	movaps	%xmm7, %xmm1
1407f10d3: f3 41 0f 5c cc              	subss	%xmm12, %xmm1
1407f10d8: 0f 28 d0                    	movaps	%xmm0, %xmm2
1407f10db: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
1407f10df: 0f 28 d9                    	movaps	%xmm1, %xmm3
1407f10e2: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
1407f10e6: f3 0f 58 da                 	addss	%xmm2, %xmm3
1407f10ea: 0f 57 d2                    	xorps	%xmm2, %xmm2
1407f10ed: f3 0f 51 d3                 	sqrtss	%xmm3, %xmm2
1407f10f1: f3 0f 10 1d 0b 48 13 02     	movss	0x213480b(%rip), %xmm3  # 0x142925904
1407f10f9: f3 0f 5e da                 	divss	%xmm2, %xmm3
1407f10fd: f3 0f 59 c3                 	mulss	%xmm3, %xmm0
1407f1101: 0f 57 1d e8 47 13 02        	xorps	0x21347e8(%rip), %xmm3  # 0x1429258f0
1407f1108: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
1407f110c: f3 0f 11 9d 98 01 00 00     	movss	%xmm3, 0x198(%rbp)
1407f1114: f3 0f 11 85 9c 01 00 00     	movss	%xmm0, 0x19c(%rbp)
1407f111c: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
1407f1123: 48 8d 95 98 01 00 00        	leaq	0x198(%rbp), %rdx
1407f112a: e8 91 bc 4a 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407f112f: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f1136: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
1407f113d: e8 8e 00 4a 00              	callq	0x140c911d0 <.text+0xc901d0>
1407f1142: 66 0f 6f 85 e0 01 00 00     	movdqa	0x1e0(%rbp), %xmm0
1407f114a: 66 0f 7f 85 00 01 00 00     	movdqa	%xmm0, 0x100(%rbp)
1407f1152: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407f1159: 8a 58 fc                    	movb	-0x4(%rax), %bl
1407f115c: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
1407f1163: b1 09                       	movb	$0x9, %cl
1407f1165: e8 e6 20 04 00              	callq	0x140833250 <.text+0x832250>
1407f116a: 4c 8b bd f8 00 00 00        	movq	0xf8(%rbp), %r15
1407f1171: 89 d9                       	movl	%ebx, %ecx
1407f1173: 89 c2                       	movl	%eax, %edx
1407f1175: e8 16 4b 33 00              	callq	0x140b25c90 <.text+0xb24c90>
1407f117a: 88 85 98 01 00 00           	movb	%al, 0x198(%rbp)
1407f1180: 88 95 99 01 00 00           	movb	%dl, 0x199(%rbp)
1407f1186: 49 83 7f 18 00              	cmpq	$0x0, 0x18(%r15)
1407f118b: 0f 84 66 18 00 00           	je	0x1407f29f7 <.text+0x7f19f7>
1407f1191: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
1407f1195: 48 8d 95 98 01 00 00        	leaq	0x198(%rbp), %rdx
1407f119c: e8 1f 6c a0 00              	callq	0x1411f7dc0 <.text+0x11f6dc0>
1407f11a1: 4c 01 ad 60 01 00 00        	addq	%r13, 0x160(%rbp)
1407f11a8: f3 41 0f 16 c5              	movshdup	%xmm13, %xmm0           # xmm0 = xmm13[1,1,3,3]
1407f11ad: f3 41 0f 16 de              	movshdup	%xmm14, %xmm3           # xmm3 = xmm14[1,1,3,3]
1407f11b2: f3 0f 10 0d b6 46 13 02     	movss	0x21346b6(%rip), %xmm1  # 0x142925870
1407f11ba: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
1407f11be: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
1407f11c2: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407f11c6: 41 0f 16 cd                 	movlhps	%xmm13, %xmm1           # xmm1 = xmm1[0],xmm13[0]
1407f11ca: 41 0f c6 cd e2              	shufps	$0xe2, %xmm13, %xmm1    # xmm1 = xmm1[2,0],xmm13[2,3]
1407f11cf: 0f 28 15 0a 47 13 02        	movaps	0x213470a(%rip), %xmm2  # 0x1429258e0
1407f11d6: 0f 59 ca                    	mulps	%xmm2, %xmm1
1407f11d9: 44 0f 28 d7                 	movaps	%xmm7, %xmm10
1407f11dd: 45 0f 16 d6                 	movlhps	%xmm14, %xmm10          # xmm10 = xmm10[0],xmm14[0]
1407f11e1: 45 0f c6 d6 e2              	shufps	$0xe2, %xmm14, %xmm10   # xmm10 = xmm10[2,0],xmm14[2,3]
1407f11e6: 44 0f 59 d2                 	mulps	%xmm2, %xmm10
1407f11ea: 44 0f 58 d1                 	addps	%xmm1, %xmm10
1407f11ee: f3 0f 58 d8                 	addss	%xmm0, %xmm3
1407f11f2: 0f 29 9d a0 00 00 00        	movaps	%xmm3, 0xa0(%rbp)
1407f11f9: 49 8b 0f                    	movq	(%r15), %rcx
1407f11fc: 49 8b 57 08                 	movq	0x8(%r15), %rdx
1407f1200: 49 89 d0                    	movq	%rdx, %r8
1407f1203: 49 21 c0                    	andq	%rax, %r8
1407f1206: 48 c1 e8 39                 	shrq	$0x39, %rax
1407f120a: 66 0f 6e c0                 	movd	%eax, %xmm0
1407f120e: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407f1212: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407f1217: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f121c: 48 8d 41 b8                 	leaq	-0x48(%rcx), %rax
1407f1220: 44 0f b6 8d 98 01 00 00     	movzbl	0x198(%rbp), %r9d
1407f1228: 44 0f b6 95 99 01 00 00     	movzbl	0x199(%rbp), %r10d
1407f1230: 45 31 ff                    	xorl	%r15d, %r15d
1407f1233: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1407f1239: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407f123d: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407f1241: 66 0f d7 da                 	pmovmskb	%xmm2, %ebx
1407f1245: 85 db                       	testl	%ebx, %ebx
1407f1247: 74 7b                       	je	0x1407f12c4 <.text+0x7f02c4>
1407f1249: 41 80 f9 1a                 	cmpb	$0x1a, %r9b
1407f124d: 75 2e                       	jne	0x1407f127d <.text+0x7f027d>
1407f124f: 4d 89 fd                    	movq	%r15, %r13
1407f1252: f3 44 0f bc fb              	tzcntl	%ebx, %r15d
1407f1257: 4d 01 c7                    	addq	%r8, %r15
1407f125a: 49 21 d7                    	andq	%rdx, %r15
1407f125d: 49 f7 df                    	negq	%r15
1407f1260: 4f 8d 1c ff                 	leaq	(%r15,%r15,8), %r11
1407f1264: 42 80 3c d8 1a              	cmpb	$0x1a, (%rax,%r11,8)
1407f1269: 74 7f                       	je	0x1407f12ea <.text+0x7f02ea>
1407f126b: 44 8d 5b ff                 	leal	-0x1(%rbx), %r11d
1407f126f: 66 41 21 db                 	andw	%bx, %r11w
1407f1273: 44 89 db                    	movl	%r11d, %ebx
1407f1276: 4d 89 ef                    	movq	%r13, %r15
1407f1279: 75 d4                       	jne	0x1407f124f <.text+0x7f024f>
1407f127b: eb 47                       	jmp	0x1407f12c4 <.text+0x7f02c4>
1407f127d: 4c 89 bd d0 01 00 00        	movq	%r15, 0x1d0(%rbp)
1407f1284: f3 44 0f bc fb              	tzcntl	%ebx, %r15d
1407f1289: 4d 01 c7                    	addq	%r8, %r15
1407f128c: 49 21 d7                    	andq	%rdx, %r15
1407f128f: 49 f7 df                    	negq	%r15
1407f1292: 4f 8d 2c ff                 	leaq	(%r15,%r15,8), %r13
1407f1296: 46 0f b6 1c e8              	movzbl	(%rax,%r13,8), %r11d
1407f129b: 41 80 fb 1a                 	cmpb	$0x1a, %r11b
1407f129f: 74 0f                       	je	0x1407f12b0 <.text+0x7f02b0>
1407f12a1: 45 38 d9                    	cmpb	%r11b, %r9b
1407f12a4: 75 0a                       	jne	0x1407f12b0 <.text+0x7f02b0>
1407f12a6: 4e 8d 1c e8                 	leaq	(%rax,%r13,8), %r11
1407f12aa: 45 3a 53 01                 	cmpb	0x1(%r11), %r10b
1407f12ae: 74 3a                       	je	0x1407f12ea <.text+0x7f02ea>
1407f12b0: 44 8d 5b ff                 	leal	-0x1(%rbx), %r11d
1407f12b4: 66 41 21 db                 	andw	%bx, %r11w
1407f12b8: 44 89 db                    	movl	%r11d, %ebx
1407f12bb: 4c 8b bd d0 01 00 00        	movq	0x1d0(%rbp), %r15
1407f12c2: 75 c0                       	jne	0x1407f1284 <.text+0x7f0284>
1407f12c4: 66 41 0f 74 cb              	pcmpeqb	%xmm11, %xmm1
1407f12c9: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
1407f12ce: 45 85 db                    	testl	%r11d, %r11d
1407f12d1: 0f 85 20 17 00 00           	jne	0x1407f29f7 <.text+0x7f19f7>
1407f12d7: 4d 01 f8                    	addq	%r15, %r8
1407f12da: 49 83 c0 10                 	addq	$0x10, %r8
1407f12de: 49 83 c7 10                 	addq	$0x10, %r15
1407f12e2: 49 21 d0                    	andq	%rdx, %r8
1407f12e5: e9 49 ff ff ff              	jmp	0x1407f1233 <.text+0x7f0233>
1407f12ea: 4b 8d 04 ff                 	leaq	(%r15,%r15,8), %rax
1407f12ee: 48 8b 5c c1 d8              	movq	-0x28(%rcx,%rax,8), %rbx
1407f12f3: f0                          	lock
1407f12f4: 48 ff 03                    	incq	(%rbx)
1407f12f7: 0f 8e d0 19 00 00           	jle	0x1407f2ccd <.text+0x7f1ccd>
1407f12fd: 48 89 9d 78 01 00 00        	movq	%rbx, 0x178(%rbp)
1407f1304: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407f130b: 44 8a 78 fc                 	movb	-0x4(%rax), %r15b
1407f130f: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
1407f1316: c6 85 37 02 00 00 01        	movb	$0x1, 0x237(%rbp)
1407f131d: b1 09                       	movb	$0x9, %cl
1407f131f: e8 2c 1f 04 00              	callq	0x140833250 <.text+0x832250>
1407f1324: 4c 8b ad f8 00 00 00        	movq	0xf8(%rbp), %r13
1407f132b: c6 85 37 02 00 00 01        	movb	$0x1, 0x237(%rbp)
1407f1332: 44 89 f9                    	movl	%r15d, %ecx
1407f1335: 89 c2                       	movl	%eax, %edx
1407f1337: e8 54 49 33 00              	callq	0x140b25c90 <.text+0xb24c90>
1407f133c: 88 85 98 01 00 00           	movb	%al, 0x198(%rbp)
1407f1342: 88 95 99 01 00 00           	movb	%dl, 0x199(%rbp)
1407f1348: 49 83 7d 48 00              	cmpq	$0x0, 0x48(%r13)
1407f134d: 0f 84 15 17 00 00           	je	0x1407f2a68 <.text+0x7f1a68>
1407f1353: 49 8d 4d 50                 	leaq	0x50(%r13), %rcx
1407f1357: c6 85 37 02 00 00 01        	movb	$0x1, 0x237(%rbp)
1407f135e: 48 8d 95 98 01 00 00        	leaq	0x198(%rbp), %rdx
1407f1365: e8 56 6a a0 00              	callq	0x1411f7dc0 <.text+0x11f6dc0>
1407f136a: 49 8b 4d 30                 	movq	0x30(%r13), %rcx
1407f136e: 49 8b 55 38                 	movq	0x38(%r13), %rdx
1407f1372: 49 89 d0                    	movq	%rdx, %r8
1407f1375: 49 21 c0                    	andq	%rax, %r8
1407f1378: 48 c1 e8 39                 	shrq	$0x39, %rax
1407f137c: 66 0f 6e c0                 	movd	%eax, %xmm0
1407f1380: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407f1384: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407f1389: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f138e: 48 8d 41 b8                 	leaq	-0x48(%rcx), %rax
1407f1392: 44 0f b6 8d 98 01 00 00     	movzbl	0x198(%rbp), %r9d
1407f139a: 44 0f b6 9d 99 01 00 00     	movzbl	0x199(%rbp), %r11d
1407f13a2: 45 31 ed                    	xorl	%r13d, %r13d
1407f13a5: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1407f13ab: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407f13af: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407f13b3: 66 44 0f d7 fa              	pmovmskb	%xmm2, %r15d
1407f13b8: 45 85 ff                    	testl	%r15d, %r15d
1407f13bb: 0f 84 e0 00 00 00           	je	0x1407f14a1 <.text+0x7f04a1>
1407f13c1: 41 80 f9 1a                 	cmpb	$0x1a, %r9b
1407f13c5: 48 89 9d d0 01 00 00        	movq	%rbx, 0x1d0(%rbp)
1407f13cc: 75 52                       	jne	0x1407f1420 <.text+0x7f0420>
1407f13ce: 44 88 8d 70 01 00 00        	movb	%r9b, 0x170(%rbp)
1407f13d5: 4d 89 e9                    	movq	%r13, %r9
1407f13d8: f3 45 0f bc ef              	tzcntl	%r15d, %r13d
1407f13dd: 4d 01 c5                    	addq	%r8, %r13
1407f13e0: 49 21 d5                    	andq	%rdx, %r13
1407f13e3: 49 f7 dd                    	negq	%r13
1407f13e6: 4e 8d 14 ed 00 00 00 00     	leaq	(,%r13,8), %r10
1407f13ee: 4d 01 ea                    	addq	%r13, %r10
1407f13f1: 42 80 3c d0 1a              	cmpb	$0x1a, (%rax,%r10,8)
1407f13f6: 0f 84 cb 00 00 00           	je	0x1407f14c7 <.text+0x7f04c7>
1407f13fc: 45 8d 57 ff                 	leal	-0x1(%r15), %r10d
1407f1400: 66 45 21 fa                 	andw	%r15w, %r10w
1407f1404: 45 89 d7                    	movl	%r10d, %r15d
1407f1407: 4d 89 cd                    	movq	%r9, %r13
1407f140a: 48 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %rbx
1407f1411: 44 0f b6 8d 70 01 00 00     	movzbl	0x170(%rbp), %r9d
1407f1419: 75 ba                       	jne	0x1407f13d5 <.text+0x7f03d5>
1407f141b: e9 81 00 00 00              	jmp	0x1407f14a1 <.text+0x7f04a1>
1407f1420: 44 88 9d 28 02 00 00        	movb	%r11b, 0x228(%rbp)
1407f1427: 4c 89 ad 18 01 00 00        	movq	%r13, 0x118(%rbp)
1407f142e: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
1407f1435: f3 45 0f bc ef              	tzcntl	%r15d, %r13d
1407f143a: 4d 01 c5                    	addq	%r8, %r13
1407f143d: 49 21 d5                    	andq	%rdx, %r13
1407f1440: 49 f7 dd                    	negq	%r13
1407f1443: 4e 8d 1c ed 00 00 00 00     	leaq	(,%r13,8), %r11
1407f144b: 4d 01 eb                    	addq	%r13, %r11
1407f144e: 46 0f b6 14 d8              	movzbl	(%rax,%r11,8), %r10d
1407f1453: 41 80 fa 1a                 	cmpb	$0x1a, %r10b
1407f1457: 74 1e                       	je	0x1407f1477 <.text+0x7f0477>
1407f1459: 45 38 d1                    	cmpb	%r10b, %r9b
1407f145c: 75 19                       	jne	0x1407f1477 <.text+0x7f0477>
1407f145e: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
1407f1465: 4e 8d 14 d8                 	leaq	(%rax,%r11,8), %r10
1407f1469: 44 0f b6 9d 28 02 00 00     	movzbl	0x228(%rbp), %r11d
1407f1471: 45 3a 5a 01                 	cmpb	0x1(%r10), %r11b
1407f1475: 74 50                       	je	0x1407f14c7 <.text+0x7f04c7>
1407f1477: 45 8d 57 ff                 	leal	-0x1(%r15), %r10d
1407f147b: 66 45 21 fa                 	andw	%r15w, %r10w
1407f147f: 45 89 d7                    	movl	%r10d, %r15d
1407f1482: 44 0f b6 9d 28 02 00 00     	movzbl	0x228(%rbp), %r11d
1407f148a: 4c 8b ad 18 01 00 00        	movq	0x118(%rbp), %r13
1407f1491: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
1407f1498: 48 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %rbx
1407f149f: 75 94                       	jne	0x1407f1435 <.text+0x7f0435>
1407f14a1: 66 41 0f 74 cb              	pcmpeqb	%xmm11, %xmm1
1407f14a6: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
1407f14ab: 45 85 d2                    	testl	%r10d, %r10d
1407f14ae: 0f 85 b4 15 00 00           	jne	0x1407f2a68 <.text+0x7f1a68>
1407f14b4: 4d 01 e8                    	addq	%r13, %r8
1407f14b7: 49 83 c0 10                 	addq	$0x10, %r8
1407f14bb: 49 83 c5 10                 	addq	$0x10, %r13
1407f14bf: 49 21 d0                    	andq	%rdx, %r8
1407f14c2: e9 de fe ff ff              	jmp	0x1407f13a5 <.text+0x7f03a5>
1407f14c7: 4a 8d 04 ed 00 00 00 00     	leaq	(,%r13,8), %rax
1407f14cf: 4c 01 e8                    	addq	%r13, %rax
1407f14d2: 48 8b 44 c1 d8              	movq	-0x28(%rcx,%rax,8), %rax
1407f14d7: f0                          	lock
1407f14d8: 48 ff 00                    	incq	(%rax)
1407f14db: 0f 8e ec 17 00 00           	jle	0x1407f2ccd <.text+0x7f1ccd>
1407f14e1: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
1407f14e8: c6 85 34 02 00 00 01        	movb	$0x1, 0x234(%rbp)
1407f14ef: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f14f6: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
1407f14fa: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
1407f1501: e8 5a 71 4a 00              	callq	0x140c98660 <.text+0xc97660>
1407f1506: 4c 8b bd b8 01 00 00        	movq	0x1b8(%rbp), %r15
1407f150d: 48 8b 95 d0 01 00 00        	movq	0x1d0(%rbp), %rdx
1407f1514: f3 0f 10 85 e8 01 00 00     	movss	0x1e8(%rbp), %xmm0
1407f151c: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407f151f: f3 0f 59 0d 69 f1 16 02     	mulss	0x216f169(%rip), %xmm1  # 0x142960690
1407f1527: 0f 14 c8                    	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
1407f152a: 0f 59 0d af 43 13 02        	mulps	0x21343af(%rip), %xmm1  # 0x1429258e0
1407f1531: 0f 28 c1                    	movaps	%xmm1, %xmm0
1407f1534: 0f c6 c1 d0                 	shufps	$0xd0, %xmm1, %xmm0     # xmm0 = xmm0[0,0],xmm1[1,3]
1407f1538: 0f 28 d9                    	movaps	%xmm1, %xmm3
1407f153b: 0f 14 d9                    	unpcklps	%xmm1, %xmm3            # xmm3 = xmm3[0],xmm1[0],xmm3[1],xmm1[1]
1407f153e: 0f 28 ad 00 01 00 00        	movaps	0x100(%rbp), %xmm5
1407f1545: 0f 28 f5                    	movaps	%xmm5, %xmm6
1407f1548: 0f 59 f5                    	mulps	%xmm5, %xmm6
1407f154b: f3 0f 16 d6                 	movshdup	%xmm6, %xmm2            # xmm2 = xmm6[1,1,3,3]
1407f154f: f3 0f 58 d6                 	addss	%xmm6, %xmm2
1407f1553: 0f 28 e6                    	movaps	%xmm6, %xmm4
1407f1556: 66 0f 15 e6                 	unpckhpd	%xmm6, %xmm4            # xmm4 = xmm4[1],xmm6[1]
1407f155a: f3 0f 58 e2                 	addss	%xmm2, %xmm4
1407f155e: 0f c6 e4 00                 	shufps	$0x0, %xmm4, %xmm4      # xmm4 = xmm4[0,0,0,0]
1407f1562: 0f c6 f6 ff                 	shufps	$0xff, %xmm6, %xmm6     # xmm6 = xmm6[3,3,3,3]
1407f1566: 0f 5c f4                    	subps	%xmm4, %xmm6
1407f1569: 0f 59 de                    	mulps	%xmm6, %xmm3
1407f156c: 0f 28 d5                    	movaps	%xmm5, %xmm2
1407f156f: 0f 59 d0                    	mulps	%xmm0, %xmm2
1407f1572: f3 0f 16 e2                 	movshdup	%xmm2, %xmm4            # xmm4 = xmm2[1,1,3,3]
1407f1576: f3 0f 58 e2                 	addss	%xmm2, %xmm4
1407f157a: 0f 12 d2                    	movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
1407f157d: f3 0f 58 d4                 	addss	%xmm4, %xmm2
1407f1581: f3 0f 58 d2                 	addss	%xmm2, %xmm2
1407f1585: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407f1589: 0f 59 d5                    	mulps	%xmm5, %xmm2
1407f158c: 0f 58 d3                    	addps	%xmm3, %xmm2
1407f158f: 0f 28 dd                    	movaps	%xmm5, %xmm3
1407f1592: 0f c6 dd d2                 	shufps	$0xd2, %xmm5, %xmm3     # xmm3 = xmm3[2,0],xmm5[1,3]
1407f1596: 0f c6 c9 c1                 	shufps	$0xc1, %xmm1, %xmm1     # xmm1 = xmm1[1,0,0,3]
1407f159a: 0f 29 5d 50                 	movaps	%xmm3, 0x50(%rbp)
1407f159e: 0f 59 c3                    	mulps	%xmm3, %xmm0
1407f15a1: 0f 59 cd                    	mulps	%xmm5, %xmm1
1407f15a4: 0f 5c c1                    	subps	%xmm1, %xmm0
1407f15a7: 0f c6 c0 d2                 	shufps	$0xd2, %xmm0, %xmm0     # xmm0 = xmm0[2,0,1,3]
1407f15ab: 44 0f 28 c5                 	movaps	%xmm5, %xmm8
1407f15af: 44 0f 58 c5                 	addps	%xmm5, %xmm8
1407f15b3: 45 0f c6 c0 ff              	shufps	$0xff, %xmm8, %xmm8     # xmm8 = xmm8[3,3,3,3]
1407f15b8: 41 0f 59 c0                 	mulps	%xmm8, %xmm0
1407f15bc: 0f 58 c2                    	addps	%xmm2, %xmm0
1407f15bf: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407f15c3: 0f 14 8d a0 00 00 00        	unpcklps	0xa0(%rbp), %xmm1       # xmm1 = xmm1[0],mem[0],xmm1[1],mem[1]
1407f15ca: 0f 58 c8                    	addps	%xmm0, %xmm1
1407f15cd: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407f15d0: f3 45 0f 16 ca              	movshdup	%xmm10, %xmm9           # xmm9 = xmm10[1,1,3,3]
1407f15d5: f3 41 0f 58 c1              	addss	%xmm9, %xmm0
1407f15da: 0f 13 8d 20 01 00 00        	movlps	%xmm1, 0x120(%rbp)
1407f15e1: f3 0f 11 85 28 01 00 00     	movss	%xmm0, 0x128(%rbp)
1407f15e9: 0f 29 6d c0                 	movaps	%xmm5, -0x40(%rbp)
1407f15ed: c6 85 34 02 00 00 00        	movb	$0x0, 0x234(%rbp)
1407f15f4: 48 8d 45 c0                 	leaq	-0x40(%rbp), %rax
1407f15f8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f15fd: 4c 89 f9                    	movq	%r15, %rcx
1407f1600: 4c 8b 85 28 02 00 00        	movq	0x228(%rbp), %r8
1407f1607: 4c 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %r9
1407f160e: e8 9d 3c 03 00              	callq	0x1408252b0 <.text+0x8242b0>
1407f1613: 49 8b 07                    	movq	(%r15), %rax
1407f1616: 49 0b 47 08                 	orq	0x8(%r15), %rax
1407f161a: 0f 85 1b 15 00 00           	jne	0x1407f2b3b <.text+0x7f1b3b>
1407f1620: 44 0f 29 4d 60              	movaps	%xmm9, 0x60(%rbp)
1407f1625: 49 c7 47 08 00 00 00 00     	movq	$0x0, 0x8(%r15)
1407f162d: 49 c7 07 01 00 00 00        	movq	$0x1, (%r15)
1407f1634: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
1407f163b: 41 0f 29 47 10              	movaps	%xmm0, 0x10(%r15)
1407f1640: f3 44 0f 11 95 68 01 00 00  	movss	%xmm10, 0x168(%rbp)
1407f1649: f3 45 0f 11 57 20           	movss	%xmm10, 0x20(%r15)
1407f164f: 66 0f 6f 85 a0 00 00 00     	movdqa	0xa0(%rbp), %xmm0
1407f1657: 66 41 0f 7e 47 24           	movd	%xmm0, 0x24(%r15)
1407f165d: f3 44 0f 11 8d d0 01 00 00  	movss	%xmm9, 0x1d0(%rbp)
1407f1666: f3 45 0f 11 4f 28           	movss	%xmm9, 0x28(%r15)
1407f166c: 48 b8 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
1407f1676: 49 89 47 2c                 	movq	%rax, 0x2c(%r15)
1407f167a: 41 c7 47 34 00 00 80 3f     	movl	$0x3f800000, 0x34(%r15) # imm = 0x3F800000
1407f1682: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407f1689: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407f168d: fe c8                       	decb	%al
1407f168f: 3c 03                       	cmpb	$0x3, %al
1407f1691: 0f 83 78 15 00 00           	jae	0x1407f2c0f <.text+0x7f1c0f>
1407f1697: 0f b6 c0                    	movzbl	%al, %eax
1407f169a: 48 8d 0d 3f 9f 29 02        	leaq	0x2299f3f(%rip), %rcx   # 0x142a8b5e0
1407f16a1: 48 63 1c 81                 	movslq	(%rcx,%rax,4), %rbx
1407f16a5: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
1407f16ac: 48 8d 15 c5 9c 29 02        	leaq	0x2299cc5(%rip), %rdx   # 0x142a8b378
1407f16b3: e8 a8 1b 04 00              	callq	0x140833260 <.text+0x832260>
1407f16b8: 66 0f 6e 00                 	movd	(%rax), %xmm0
1407f16bc: 66 0f 7e 85 18 01 00 00     	movd	%xmm0, 0x118(%rbp)
1407f16c4: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407f16cb: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407f16cf: 83 f8 01                    	cmpl	$0x1, %eax
1407f16d2: 48 89 9d 70 01 00 00        	movq	%rbx, 0x170(%rbp)
1407f16d9: 0f 84 8c 00 00 00           	je	0x1407f176b <.text+0x7f076b>
1407f16df: 83 f8 02                    	cmpl	$0x2, %eax
1407f16e2: 74 48                       	je	0x1407f172c <.text+0x7f072c>
1407f16e4: 83 f8 03                    	cmpl	$0x3, %eax
1407f16e7: 0f 85 5c 15 00 00           	jne	0x1407f2c49 <.text+0x7f1c49>
1407f16ed: 0f b6 05 15 35 e4 02        	movzbl	0x2e43515(%rip), %eax   # 0x143634c09
1407f16f4: b9 24 00 00 00              	movl	$0x24, %ecx
1407f16f9: ba 04 00 00 00              	movl	$0x4, %edx
1407f16fe: e8 0d 25 e2 ff              	callq	0x140613c10 <.text+0x612c10>
1407f1703: 48 85 c0                    	testq	%rax, %rax
1407f1706: 0f 84 57 15 00 00           	je	0x1407f2c63 <.text+0x7f1c63>
1407f170c: c7 00 c3 f5 a8 be           	movl	$0xbea8f5c3, (%rax)     # imm = 0xBEA8F5C3
1407f1712: f3 44 0f 10 0d b5 97 29 02  	movss	0x22997b5(%rip), %xmm9  # 0x142a8aed0
1407f171b: 44 0f 28 3d 9d 97 29 02     	movaps	0x229979d(%rip), %xmm15 # 0x142a8aec0
1407f1723: 0f 28 05 86 97 29 02        	movaps	0x2299786(%rip), %xmm0  # 0x142a8aeb0
1407f172a: eb 7c                       	jmp	0x1407f17a8 <.text+0x7f07a8>
1407f172c: 0f b6 05 d6 34 e4 02        	movzbl	0x2e434d6(%rip), %eax   # 0x143634c09
1407f1733: b9 24 00 00 00              	movl	$0x24, %ecx
1407f1738: ba 04 00 00 00              	movl	$0x4, %edx
1407f173d: e8 ce 24 e2 ff              	callq	0x140613c10 <.text+0x612c10>
1407f1742: 48 85 c0                    	testq	%rax, %rax
1407f1745: 0f 84 29 15 00 00           	je	0x1407f2c74 <.text+0x7f1c74>
1407f174b: c7 00 85 eb d1 be           	movl	$0xbed1eb85, (%rax)     # imm = 0xBED1EB85
1407f1751: f3 44 0f 10 0d a6 97 29 02  	movss	0x22997a6(%rip), %xmm9  # 0x142a8af00
1407f175a: 44 0f 28 3d 8e 97 29 02     	movaps	0x229978e(%rip), %xmm15 # 0x142a8aef0
1407f1762: 0f 28 05 77 97 29 02        	movaps	0x2299777(%rip), %xmm0  # 0x142a8aee0
1407f1769: eb 3d                       	jmp	0x1407f17a8 <.text+0x7f07a8>
1407f176b: 0f b6 05 97 34 e4 02        	movzbl	0x2e43497(%rip), %eax   # 0x143634c09
1407f1772: b9 24 00 00 00              	movl	$0x24, %ecx
1407f1777: ba 04 00 00 00              	movl	$0x4, %edx
1407f177c: e8 8f 24 e2 ff              	callq	0x140613c10 <.text+0x612c10>
1407f1781: 48 85 c0                    	testq	%rax, %rax
1407f1784: 0f 84 fb 14 00 00           	je	0x1407f2c85 <.text+0x7f1c85>
1407f178a: c7 00 cd cc 4c be           	movl	$0xbe4ccccd, (%rax)     # imm = 0xBE4CCCCD
1407f1790: f3 44 0f 10 0d d3 d9 1e 02  	movss	0x21ed9d3(%rip), %xmm9  # 0x1429df16c
1407f1799: 44 0f 28 3d 7f 97 29 02     	movaps	0x229977f(%rip), %xmm15 # 0x142a8af20
1407f17a1: 0f 28 05 68 97 29 02        	movaps	0x2299768(%rip), %xmm0  # 0x142a8af10
1407f17a8: 44 0f 11 78 04              	movups	%xmm15, 0x4(%rax)
1407f17ad: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
1407f17b4: 0f 11 40 14                 	movups	%xmm0, 0x14(%rax)
1407f17b8: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
1407f17bf: 48 8d 15 ba 9d 29 02        	leaq	0x2299dba(%rip), %rdx   # 0x142a8b580
1407f17c6: e8 95 1a 04 00              	callq	0x140833260 <.text+0x832260>
1407f17cb: 4c 8b bd b8 01 00 00        	movq	0x1b8(%rbp), %r15
1407f17d2: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1407f17d9: 48 8d 99 c8 00 00 00        	leaq	0xc8(%rcx), %rbx
1407f17e0: f3 44 0f 58 48 04           	addss	0x4(%rax), %xmm9
1407f17e6: 66 44 0f 6e 18              	movd	(%rax), %xmm11
1407f17eb: 4d 8b af d8 00 00 00        	movq	0xd8(%r15), %r13
1407f17f2: 4d 3b af c8 00 00 00        	cmpq	0xc8(%r15), %r13
1407f17f9: 75 0f                       	jne	0x1407f180a <.text+0x7f080a>
1407f17fb: 48 89 d9                    	movq	%rbx, %rcx
1407f17fe: 48 8d 15 93 9d 29 02        	leaq	0x2299d93(%rip), %rdx   # 0x142a8b598
1407f1805: e8 f6 1d d7 ff              	callq	0x140563600 <.text+0x562600>
1407f180a: 49 8b 87 d0 00 00 00        	movq	0xd0(%r15), %rax
1407f1811: 4a 8d 0c ad 00 00 00 00     	leaq	(,%r13,4), %rcx
1407f1819: 4c 01 e9                    	addq	%r13, %rcx
1407f181c: 66 44 0f 7e 1c 88           	movd	%xmm11, (%rax,%rcx,4)
1407f1822: f3 44 0f 11 4c 88 04        	movss	%xmm9, 0x4(%rax,%rcx,4)
1407f1829: f3 44 0f 11 7c 88 08        	movss	%xmm15, 0x8(%rax,%rcx,4)
1407f1830: f3 41 0f 16 c7              	movshdup	%xmm15, %xmm0           # xmm0 = xmm15[1,1,3,3]
1407f1835: f3 0f 11 44 88 0c           	movss	%xmm0, 0xc(%rax,%rcx,4)
1407f183b: c6 44 88 10 00              	movb	$0x0, 0x10(%rax,%rcx,4)
1407f1840: 49 ff c5                    	incq	%r13
1407f1843: 4d 89 af d8 00 00 00        	movq	%r13, 0xd8(%r15)
1407f184a: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
1407f1851: f3 44 0f 10 78 0c           	movss	0xc(%rax), %xmm15
1407f1857: f2 44 0f 10 48 10           	movsd	0x10(%rax), %xmm9
1407f185d: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
1407f1864: 48 8d 15 15 9d 29 02        	leaq	0x2299d15(%rip), %rdx   # 0x142a8b580
1407f186b: e8 f0 19 04 00              	callq	0x140833260 <.text+0x832260>
1407f1870: f3 44 0f 58 78 04           	addss	0x4(%rax), %xmm15
1407f1876: 66 44 0f 6e 18              	movd	(%rax), %xmm11
1407f187b: 4d 8b af d8 00 00 00        	movq	0xd8(%r15), %r13
1407f1882: 4d 3b af c8 00 00 00        	cmpq	0xc8(%r15), %r13
1407f1889: 75 0f                       	jne	0x1407f189a <.text+0x7f089a>
1407f188b: 48 89 d9                    	movq	%rbx, %rcx
1407f188e: 48 8d 15 03 9d 29 02        	leaq	0x2299d03(%rip), %rdx   # 0x142a8b598
1407f1895: e8 66 1d d7 ff              	callq	0x140563600 <.text+0x562600>
1407f189a: 49 8b 87 d0 00 00 00        	movq	0xd0(%r15), %rax
1407f18a1: 4a 8d 0c ad 00 00 00 00     	leaq	(,%r13,4), %rcx
1407f18a9: 4c 01 e9                    	addq	%r13, %rcx
1407f18ac: 66 44 0f 7e 1c 88           	movd	%xmm11, (%rax,%rcx,4)
1407f18b2: f3 44 0f 11 7c 88 04        	movss	%xmm15, 0x4(%rax,%rcx,4)
1407f18b9: 44 0f 13 4c 88 08           	movlps	%xmm9, 0x8(%rax,%rcx,4)
1407f18bf: c6 44 88 10 00              	movb	$0x0, 0x10(%rax,%rcx,4)
1407f18c4: 49 ff c5                    	incq	%r13
1407f18c7: 4d 89 af d8 00 00 00        	movq	%r13, 0xd8(%r15)
1407f18ce: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
1407f18d5: f3 44 0f 10 78 18           	movss	0x18(%rax), %xmm15
1407f18db: f2 44 0f 10 48 1c           	movsd	0x1c(%rax), %xmm9
1407f18e1: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
1407f18e8: 48 8d 15 91 9c 29 02        	leaq	0x2299c91(%rip), %rdx   # 0x142a8b580
1407f18ef: e8 6c 19 04 00              	callq	0x140833260 <.text+0x832260>
1407f18f4: f3 44 0f 58 78 04           	addss	0x4(%rax), %xmm15
1407f18fa: 66 44 0f 6e 18              	movd	(%rax), %xmm11
1407f18ff: 4d 8b af d8 00 00 00        	movq	0xd8(%r15), %r13
1407f1906: 4d 3b af c8 00 00 00        	cmpq	0xc8(%r15), %r13
1407f190d: 75 0f                       	jne	0x1407f191e <.text+0x7f091e>
1407f190f: 48 89 d9                    	movq	%rbx, %rcx
1407f1912: 48 8d 15 7f 9c 29 02        	leaq	0x2299c7f(%rip), %rdx   # 0x142a8b598
1407f1919: e8 e2 1c d7 ff              	callq	0x140563600 <.text+0x562600>
1407f191e: 49 8b 87 d0 00 00 00        	movq	0xd0(%r15), %rax
1407f1925: 4a 8d 0c ad 00 00 00 00     	leaq	(,%r13,4), %rcx
1407f192d: 4c 01 e9                    	addq	%r13, %rcx
1407f1930: 66 44 0f 7e 1c 88           	movd	%xmm11, (%rax,%rcx,4)
1407f1936: f3 44 0f 11 7c 88 04        	movss	%xmm15, 0x4(%rax,%rcx,4)
1407f193d: 44 0f 13 4c 88 08           	movlps	%xmm9, 0x8(%rax,%rcx,4)
1407f1943: c6 44 88 10 00              	movb	$0x0, 0x10(%rax,%rcx,4)
1407f1948: 49 ff c5                    	incq	%r13
1407f194b: 4d 89 af d8 00 00 00        	movq	%r13, 0xd8(%r15)
1407f1952: ba 24 00 00 00              	movl	$0x24, %edx
1407f1957: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407f195d: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
1407f1964: e8 b7 22 e2 ff              	callq	0x140613c20 <.text+0x612c20>
1407f1969: 48 81 85 88 01 00 00 b0 00 00 00    	addq	$0xb0, 0x188(%rbp)
1407f1974: 48 8d 85 fc 01 00 00        	leaq	0x1fc(%rbp), %rax
1407f197b: c7 40 ef 00 00 00 00        	movl	$0x0, -0x11(%rax)
1407f1982: 48 c7 40 e7 00 00 00 00     	movq	$0x0, -0x19(%rax)
1407f198a: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
1407f1991: 48 8b 8d e7 01 00 00        	movq	0x1e7(%rbp), %rcx
1407f1998: 48 89 8d 7f 01 00 00        	movq	%rcx, 0x17f(%rbp)
1407f199f: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
1407f19a6: 49 8b 9f c0 00 00 00        	movq	0xc0(%r15), %rbx
1407f19ad: 49 3b 9f b0 00 00 00        	cmpq	0xb0(%r15), %rbx
1407f19b4: 66 45 0f 76 db              	pcmpeqd	%xmm11, %xmm11
1407f19b9: 4c 8b bd 70 01 00 00        	movq	0x170(%rbp), %r15
1407f19c0: 75 13                       	jne	0x1407f19d5 <.text+0x7f09d5>
1407f19c2: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1407f19c9: 48 8d 15 d8 99 29 02        	leaq	0x22999d8(%rip), %rdx   # 0x142a8b3a8
1407f19d0: e8 db 99 03 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407f19d5: f3 0f 10 a5 68 01 00 00     	movss	0x168(%rbp), %xmm4
1407f19dd: 48 8d 05 fc 9b 29 02        	leaq	0x2299bfc(%rip), %rax   # 0x142a8b5e0
1407f19e4: 49 01 c7                    	addq	%rax, %r15
1407f19e7: 4c 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %r9
1407f19ee: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407f19f5: 48 69 cb 60 01 00 00        	imulq	$0x160, %rbx, %rcx      # imm = 0x160
1407f19fc: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407f1a04: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407f1a0d: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407f1a16: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407f1a1e: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
1407f1a25: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407f1a2a: 8b 95 b8 00 00 00           	movl	0xb8(%rbp), %edx
1407f1a30: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407f1a34: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407f1a3c: 0f 10 85 e0 01 00 00        	movups	0x1e0(%rbp), %xmm0
1407f1a43: 0f 10 8d f0 01 00 00        	movups	0x1f0(%rbp), %xmm1
1407f1a4a: f3 0f 6f 95 00 02 00 00     	movdqu	0x200(%rbp), %xmm2
1407f1a52: 0f 10 9d 0c 02 00 00        	movups	0x20c(%rbp), %xmm3
1407f1a59: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407f1a5e: 0f 11 4c 08 54              	movups	%xmm1, 0x54(%rax,%rcx)
1407f1a63: f3 0f 7f 54 08 64           	movdqu	%xmm2, 0x64(%rax,%rcx)
1407f1a69: 0f 11 5c 08 70              	movups	%xmm3, 0x70(%rax,%rcx)
1407f1a6e: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407f1a79: 48 8b 95 bc 00 00 00        	movq	0xbc(%rbp), %rdx
1407f1a80: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407f1a88: 8b 95 c4 00 00 00           	movl	0xc4(%rbp), %edx
1407f1a8e: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407f1a95: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407f1aa0: 48 8b 95 c8 00 00 00        	movq	0xc8(%rbp), %rdx
1407f1aa7: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407f1aaf: 8b 95 d0 00 00 00           	movl	0xd0(%rbp), %edx
1407f1ab5: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407f1abc: 4c 89 bc 08 a0 00 00 00     	movq	%r15, 0xa0(%rax,%rcx)
1407f1ac4: 48 c7 84 08 a8 00 00 00 24 00 00 00 	movq	$0x24, 0xa8(%rax,%rcx)
1407f1ad0: 48 8d 15 b9 98 29 02        	leaq	0x22998b9(%rip), %rdx   # 0x142a8b390
1407f1ad7: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407f1adf: 48 c7 84 08 b8 00 00 00 11 00 00 00 	movq	$0x11, 0xb8(%rax,%rcx)
1407f1aeb: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
1407f1af2: 0f 29 84 08 c0 00 00 00     	movaps	%xmm0, 0xc0(%rax,%rcx)
1407f1afa: f3 0f 11 a5 28 02 00 00     	movss	%xmm4, 0x228(%rbp)
1407f1b02: f3 0f 11 a4 08 d0 00 00 00  	movss	%xmm4, 0xd0(%rax,%rcx)
1407f1b0b: 0f 28 85 a0 00 00 00        	movaps	0xa0(%rbp), %xmm0
1407f1b12: f3 0f 11 84 08 d4 00 00 00  	movss	%xmm0, 0xd4(%rax,%rcx)
1407f1b1b: f3 0f 10 85 d0 01 00 00     	movss	0x1d0(%rbp), %xmm0
1407f1b23: f3 0f 11 84 08 d8 00 00 00  	movss	%xmm0, 0xd8(%rax,%rcx)
1407f1b2c: 48 ba 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rdx # imm = 0x3F8000003F800000
1407f1b36: 48 89 94 08 dc 00 00 00     	movq	%rdx, 0xdc(%rax,%rcx)
1407f1b3e: c7 84 08 e4 00 00 00 00 00 80 3f    	movl	$0x3f800000, 0xe4(%rax,%rcx) # imm = 0x3F800000
1407f1b49: c7 84 08 f0 00 00 00 01 00 00 00    	movl	$0x1, 0xf0(%rax,%rcx)
1407f1b54: 8b 95 94 01 00 00           	movl	0x194(%rbp), %edx
1407f1b5a: 89 94 08 f4 00 00 00        	movl	%edx, 0xf4(%rax,%rcx)
1407f1b61: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407f1b6c: 48 8b 95 a8 01 00 00        	movq	0x1a8(%rbp), %rdx
1407f1b73: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407f1b7b: 0f 10 85 98 01 00 00        	movups	0x198(%rbp), %xmm0
1407f1b82: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407f1b8a: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407f1b92: 0f b6 95 cf 01 00 00        	movzbl	0x1cf(%rbp), %edx
1407f1b99: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407f1ba0: 0f b7 95 cd 01 00 00        	movzwl	0x1cd(%rbp), %edx
1407f1ba7: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407f1baf: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407f1bba: f3 0f 6f 85 20 01 00 00     	movdqu	0x120(%rbp), %xmm0
1407f1bc2: f3 0f 6f 8d 30 01 00 00     	movdqu	0x130(%rbp), %xmm1
1407f1bca: f3 0f 7f 8c 08 2c 01 00 00  	movdqu	%xmm1, 0x12c(%rax,%rcx)
1407f1bd3: f3 0f 7f 84 08 1c 01 00 00  	movdqu	%xmm0, 0x11c(%rax,%rcx)
1407f1bdc: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407f1be4: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1407f1beb: 4c 8b 85 7f 01 00 00        	movq	0x17f(%rbp), %r8
1407f1bf2: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407f1bfa: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407f1c02: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407f1c0a: 0f b6 95 36 02 00 00        	movzbl	0x236(%rbp), %edx
1407f1c11: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407f1c18: 66 c7 84 08 4f 01 00 00 01 09       	movw	$0x901, 0x14f(%rax,%rcx) # imm = 0x901
1407f1c22: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
1407f1c29: 4c 8b 85 e7 00 00 00        	movq	0xe7(%rbp), %r8
1407f1c30: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407f1c38: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407f1c40: 48 ff c3                    	incq	%rbx
1407f1c43: 49 89 99 c0 00 00 00        	movq	%rbx, 0xc0(%r9)
1407f1c4a: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407f1c51: 8b 40 e4                    	movl	-0x1c(%rax), %eax
1407f1c54: 89 85 70 01 00 00           	movl	%eax, 0x170(%rbp)
1407f1c5a: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1407f1c61: e8 6a 69 89 ff              	callq	0x1400885d0 <.text+0x875d0>
1407f1c66: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
1407f1c6a: f3 44 0f 10 7d 40           	movss	0x40(%rbp), %xmm15
1407f1c70: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1407f1c77: 48 8d 48 f9                 	leaq	-0x7(%rax), %rcx
1407f1c7b: e8 f0 39 0d 00              	callq	0x1408c5670 <.text+0x8c4670>
1407f1c80: 41 89 c7                    	movl	%eax, %r15d
1407f1c83: 45 0f 28 cf                 	movaps	%xmm15, %xmm9
1407f1c87: f3 44 0f 59 0d e0 3b 13 02  	mulss	0x2133be0(%rip), %xmm9  # 0x142925870
1407f1c90: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407f1c97: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407f1c9b: fe c8                       	decb	%al
1407f1c9d: 3c 03                       	cmpb	$0x3, %al
1407f1c9f: 0f 83 87 0f 00 00           	jae	0x1407f2c2c <.text+0x7f1c2c>
1407f1ca5: 0f b6 c0                    	movzbl	%al, %eax
1407f1ca8: 48 8d 0d 3d 99 29 02        	leaq	0x229993d(%rip), %rcx   # 0x142a8b5ec
1407f1caf: 48 63 1c 81                 	movslq	(%rcx,%rax,4), %rbx
1407f1cb3: 48 8d 85 fc 01 00 00        	leaq	0x1fc(%rbp), %rax
1407f1cba: c7 40 ef 00 00 00 00        	movl	$0x0, -0x11(%rax)
1407f1cc1: 48 c7 40 e7 00 00 00 00     	movq	$0x0, -0x19(%rax)
1407f1cc9: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
1407f1cd0: 48 8b 8d e7 01 00 00        	movq	0x1e7(%rbp), %rcx
1407f1cd7: 48 89 8d 27 01 00 00        	movq	%rcx, 0x127(%rbp)
1407f1cde: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1407f1ce5: 8b 45 78                    	movl	0x78(%rbp), %eax
1407f1ce8: 89 85 e8 00 00 00           	movl	%eax, 0xe8(%rbp)
1407f1cee: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1407f1cf2: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1407f1cf9: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
1407f1d00: 4c 8b a8 c0 00 00 00        	movq	0xc0(%rax), %r13
1407f1d07: 4c 3b a8 b0 00 00 00        	cmpq	0xb0(%rax), %r13
1407f1d0e: 75 13                       	jne	0x1407f1d23 <.text+0x7f0d23>
1407f1d10: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1407f1d17: 48 8d 15 3a 97 29 02        	leaq	0x229973a(%rip), %rdx   # 0x142a8b458
1407f1d1e: e8 8d 96 03 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407f1d23: 4c 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %r8
1407f1d2a: 49 8b 80 b8 00 00 00        	movq	0xb8(%r8), %rax
1407f1d31: 49 69 cd 60 01 00 00        	imulq	$0x160, %r13, %rcx      # imm = 0x160
1407f1d38: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407f1d40: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407f1d49: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407f1d52: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407f1d5a: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1407f1d61: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407f1d66: 8b 95 80 01 00 00           	movl	0x180(%rbp), %edx
1407f1d6c: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407f1d70: c7 44 08 40 01 00 00 00     	movl	$0x1, 0x40(%rax,%rcx)
1407f1d78: 0f 28 45 d0                 	movaps	-0x30(%rbp), %xmm0
1407f1d7c: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407f1d81: 0f 28 45 e0                 	movaps	-0x20(%rbp), %xmm0
1407f1d85: 41 0f 16 c5                 	movlhps	%xmm13, %xmm0           # xmm0 = xmm0[0],xmm13[0]
1407f1d89: 0f 11 44 08 54              	movups	%xmm0, 0x54(%rax,%rcx)
1407f1d8e: f3 44 0f 11 64 08 64        	movss	%xmm12, 0x64(%rax,%rcx)
1407f1d95: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
1407f1d9c: 48 89 54 08 68              	movq	%rdx, 0x68(%rax,%rcx)
1407f1da1: 8b 95 e8 00 00 00           	movl	0xe8(%rbp), %edx
1407f1da7: 89 54 08 70                 	movl	%edx, 0x70(%rax,%rcx)
1407f1dab: 44 0f 13 74 08 74           	movlps	%xmm14, 0x74(%rax,%rcx)
1407f1db1: f3 0f 11 7c 08 7c           	movss	%xmm7, 0x7c(%rax,%rcx)
1407f1db7: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407f1dc2: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
1407f1dc9: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407f1dd1: 8b 95 b8 00 00 00           	movl	0xb8(%rbp), %edx
1407f1dd7: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407f1dde: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407f1de9: 8b 95 c4 00 00 00           	movl	0xc4(%rbp), %edx
1407f1def: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407f1df6: 48 8b 95 bc 00 00 00        	movq	0xbc(%rbp), %rdx
1407f1dfd: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407f1e05: 48 8d 15 e0 97 29 02        	leaq	0x22997e0(%rip), %rdx   # 0x142a8b5ec
1407f1e0c: 48 01 d3                    	addq	%rdx, %rbx
1407f1e0f: 48 89 9c 08 a0 00 00 00     	movq	%rbx, 0xa0(%rax,%rcx)
1407f1e17: 48 c7 84 08 a8 00 00 00 28 00 00 00 	movq	$0x28, 0xa8(%rax,%rcx)
1407f1e23: 48 8d 15 0e 96 29 02        	leaq	0x229960e(%rip), %rdx   # 0x142a8b438
1407f1e2a: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407f1e32: 48 c7 84 08 b8 00 00 00 19 00 00 00 	movq	$0x19, 0xb8(%rax,%rcx)
1407f1e3e: 66 0f 6f 95 00 01 00 00     	movdqa	0x100(%rbp), %xmm2
1407f1e46: 66 0f 7f 94 08 c0 00 00 00  	movdqa	%xmm2, 0xc0(%rax,%rcx)
1407f1e4f: f3 0f 10 ad 28 02 00 00     	movss	0x228(%rbp), %xmm5
1407f1e57: f3 0f 11 ac 08 d0 00 00 00  	movss	%xmm5, 0xd0(%rax,%rcx)
1407f1e60: 0f 28 9d a0 00 00 00        	movaps	0xa0(%rbp), %xmm3
1407f1e67: f3 0f 11 9c 08 d4 00 00 00  	movss	%xmm3, 0xd4(%rax,%rcx)
1407f1e70: f3 0f 10 a5 d0 01 00 00     	movss	0x1d0(%rbp), %xmm4
1407f1e78: f3 0f 11 a4 08 d8 00 00 00  	movss	%xmm4, 0xd8(%rax,%rcx)
1407f1e81: 49 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %r9 # imm = 0x3F8000003F800000
1407f1e8b: 4c 89 8c 08 dc 00 00 00     	movq	%r9, 0xdc(%rax,%rcx)
1407f1e93: c7 84 08 e4 00 00 00 00 00 80 3f    	movl	$0x3f800000, 0xe4(%rax,%rcx) # imm = 0x3F800000
1407f1e9e: c7 84 08 f0 00 00 00 01 00 00 00    	movl	$0x1, 0xf0(%rax,%rcx)
1407f1ea9: 8b 95 94 01 00 00           	movl	0x194(%rbp), %edx
1407f1eaf: 89 94 08 f4 00 00 00        	movl	%edx, 0xf4(%rax,%rcx)
1407f1eb6: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407f1ec1: 8b 95 70 01 00 00           	movl	0x170(%rbp), %edx
1407f1ec7: 89 94 08 00 01 00 00        	movl	%edx, 0x100(%rax,%rcx)
1407f1ece: f3 0f 10 8d 18 01 00 00     	movss	0x118(%rbp), %xmm1
1407f1ed6: 0f 28 c1                    	movaps	%xmm1, %xmm0
1407f1ed9: f3 41 0f 58 c1              	addss	%xmm9, %xmm0
1407f1ede: f3 41 0f 5c c9              	subss	%xmm9, %xmm1
1407f1ee3: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
1407f1ee7: 89 94 08 04 01 00 00        	movl	%edx, 0x104(%rax,%rcx)
1407f1eee: f3 0f 11 8c 08 08 01 00 00  	movss	%xmm1, 0x108(%rax,%rcx)
1407f1ef7: f3 0f 11 84 08 0c 01 00 00  	movss	%xmm0, 0x10c(%rax,%rcx)
1407f1f00: f3 44 0f 11 bc 08 10 01 00 00       	movss	%xmm15, 0x110(%rax,%rcx)
1407f1f0a: 44 88 bc 08 14 01 00 00     	movb	%r15b, 0x114(%rax,%rcx)
1407f1f12: 0f b6 95 ca 00 00 00        	movzbl	0xca(%rbp), %edx
1407f1f19: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407f1f20: 0f b7 95 c8 00 00 00        	movzwl	0xc8(%rbp), %edx
1407f1f27: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407f1f2f: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407f1f3a: f3 0f 6f 85 e0 01 00 00     	movdqu	0x1e0(%rbp), %xmm0
1407f1f42: f3 0f 6f 8d f0 01 00 00     	movdqu	0x1f0(%rbp), %xmm1
1407f1f4a: f3 0f 7f 84 08 1c 01 00 00  	movdqu	%xmm0, 0x11c(%rax,%rcx)
1407f1f53: f3 0f 7f 8c 08 2c 01 00 00  	movdqu	%xmm1, 0x12c(%rax,%rcx)
1407f1f5c: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407f1f64: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
1407f1f6b: 4c 8b 95 27 01 00 00        	movq	0x127(%rbp), %r10
1407f1f72: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407f1f7a: 4c 89 94 08 44 01 00 00     	movq	%r10, 0x144(%rax,%rcx)
1407f1f82: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407f1f8a: 0f b6 95 36 02 00 00        	movzbl	0x236(%rbp), %edx
1407f1f91: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407f1f98: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407f1fa2: 48 8b 95 98 01 00 00        	movq	0x198(%rbp), %rdx
1407f1fa9: 4c 8b 95 9f 01 00 00        	movq	0x19f(%rbp), %r10
1407f1fb0: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407f1fb8: 4c 89 94 08 58 01 00 00     	movq	%r10, 0x158(%rax,%rcx)
1407f1fc0: 49 ff c5                    	incq	%r13
1407f1fc3: 4d 89 a8 c0 00 00 00        	movq	%r13, 0xc0(%r8)
1407f1fca: 66 0f 7f 95 e0 01 00 00     	movdqa	%xmm2, 0x1e0(%rbp)
1407f1fd2: f3 0f 11 ad f0 01 00 00     	movss	%xmm5, 0x1f0(%rbp)
1407f1fda: f3 0f 11 9d f4 01 00 00     	movss	%xmm3, 0x1f4(%rbp)
1407f1fe2: f3 0f 11 a5 f8 01 00 00     	movss	%xmm4, 0x1f8(%rbp)
1407f1fea: 48 8d 85 fc 01 00 00        	leaq	0x1fc(%rbp), %rax
1407f1ff1: 4c 89 08                    	movq	%r9, (%rax)
1407f1ff4: c7 40 08 00 00 80 3f        	movl	$0x3f800000, 0x8(%rax)  # imm = 0x3F800000
1407f1ffb: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
1407f2002: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
1407f2006: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
1407f200b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f2010: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
1407f2017: 48 8d 95 e0 01 00 00        	leaq	0x1e0(%rbp), %rdx
1407f201e: 44 8b 8d 90 00 00 00        	movl	0x90(%rbp), %r9d
1407f2025: e8 46 3e 00 00              	callq	0x1407f5e70 <.text+0x7f4e70>
1407f202a: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
1407f2031: e8 da 14 04 00              	callq	0x140833510 <.text+0x832510>
1407f2036: 80 78 21 01                 	cmpb	$0x1, 0x21(%rax)
1407f203a: 8b 85 dc 01 00 00           	movl	0x1dc(%rbp), %eax
1407f2040: 89 c1                       	movl	%eax, %ecx
1407f2042: 0f 85 c9 ec ff ff           	jne	0x1407f0d11 <.text+0x7efd11>
1407f2048: f3 0f 10 45 40              	movss	0x40(%rbp), %xmm0
1407f204d: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
1407f2054: f3 44 0f 10 4d 48           	movss	0x48(%rbp), %xmm9
1407f205a: f3 0f 10 7d 4c              	movss	0x4c(%rbp), %xmm7
1407f205f: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
1407f2066: 48 8b 42 08                 	movq	0x8(%rdx), %rax
1407f206a: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1407f206e: 48 01 c9                    	addq	%rcx, %rcx
1407f2071: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1407f2078: 48 89 8d e8 01 00 00        	movq	%rcx, 0x1e8(%rbp)
1407f207f: f2 0f 10 42 50              	movsd	0x50(%rdx), %xmm0
1407f2084: f2 0f 11 85 f0 01 00 00     	movsd	%xmm0, 0x1f0(%rbp)
1407f208c: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f2093: 66 0f 6e 8d 28 02 00 00     	movd	0x228(%rbp), %xmm1
1407f209b: 66 0f 6e 95 d0 01 00 00     	movd	0x1d0(%rbp), %xmm2
1407f20a3: e8 f8 5a a6 ff              	callq	0x140257ba0 <.text+0x256ba0>
1407f20a8: 44 0f 28 e0                 	movaps	%xmm0, %xmm12
1407f20ac: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
1407f20b0: e8 cb bc e8 ff              	callq	0x14067dd80 <.text+0x67cd80>
1407f20b5: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
1407f20b9: 48 8d 50 20                 	leaq	0x20(%rax), %rdx
1407f20bd: 4c 8d 40 10                 	leaq	0x10(%rax), %r8
1407f20c1: 49 89 c1                    	movq	%rax, %r9
1407f20c4: 49 83 c1 28                 	addq	$0x28, %r9
1407f20c8: 80 b8 98 02 00 00 00        	cmpb	$0x0, 0x298(%rax)
1407f20cf: 48 0f 45 ca                 	cmovneq	%rdx, %rcx
1407f20d3: 4d 0f 44 c8                 	cmoveq	%r8, %r9
1407f20d7: 49 8b 01                    	movq	(%r9), %rax
1407f20da: 48 8b 09                    	movq	(%rcx), %rcx
1407f20dd: 48 89 8d e0 01 00 00        	movq	%rcx, 0x1e0(%rbp)
1407f20e4: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
1407f20eb: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
1407f20f3: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f20fa: f3 0f 10 8d 28 02 00 00     	movss	0x228(%rbp), %xmm1
1407f2102: f3 0f 10 95 d0 01 00 00     	movss	0x1d0(%rbp), %xmm2
1407f210a: f3 0f 10 1d 46 95 28 02     	movss	0x2289546(%rip), %xmm3  # 0x142a7b658
1407f2112: e8 d9 2c 22 00              	callq	0x140a14df0 <.text+0xa13df0>
1407f2117: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407f211a: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
1407f211d: f3 44 0f 5f e1              	maxss	%xmm1, %xmm12
1407f2122: f3 41 0f 5c f9              	subss	%xmm9, %xmm7
1407f2127: f3 0f 59 3d b9 7f 16 02     	mulss	0x2167fb9(%rip), %xmm7  # 0x14295a0e8
1407f212f: f3 0f 58 bd a0 00 00 00     	addss	0xa0(%rbp), %xmm7
1407f2137: 76 24                       	jbe	0x1407f215d <.text+0x7f115d>
1407f2139: f3 0f 10 05 43 38 13 02     	movss	0x2133843(%rip), %xmm0  # 0x142925984
1407f2141: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
1407f2144: 0f 93 c3                    	setae	%bl
1407f2147: f3 0f 58 3d 1d d0 1e 02     	addss	0x21ed01d(%rip), %xmm7  # 0x1429df16c
1407f214f: 73 20                       	jae	0x1407f2171 <.text+0x7f1171>
1407f2151: 44 0f 2e e7                 	ucomiss	%xmm7, %xmm12
1407f2155: 0f 86 4f 03 00 00           	jbe	0x1407f24aa <.text+0x7f14aa>
1407f215b: eb 14                       	jmp	0x1407f2171 <.text+0x7f1171>
1407f215d: f3 0f 58 3d 07 d0 1e 02     	addss	0x21ed007(%rip), %xmm7  # 0x1429df16c
1407f2165: 44 0f 2e e7                 	ucomiss	%xmm7, %xmm12
1407f2169: 0f 86 3b 03 00 00           	jbe	0x1407f24aa <.text+0x7f14aa>
1407f216f: 31 db                       	xorl	%ebx, %ebx
1407f2171: 0f 28 c6                    	movaps	%xmm6, %xmm0
1407f2174: 0f 59 05 b5 8d 29 02        	mulps	0x2298db5(%rip), %xmm0  # 0x142a8af30
1407f217b: 0f 28 bd 00 01 00 00        	movaps	0x100(%rbp), %xmm7
1407f2182: 0f 28 cf                    	movaps	%xmm7, %xmm1
1407f2185: 0f 59 0d a4 8d 29 02        	mulps	0x2298da4(%rip), %xmm1  # 0x142a8af30
1407f218c: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1407f2190: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407f2194: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1407f2197: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1407f219b: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1407f219f: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407f21a3: 0f 59 cf                    	mulps	%xmm7, %xmm1
1407f21a6: 0f 58 c8                    	addps	%xmm0, %xmm1
1407f21a9: 0f 28 45 50                 	movaps	0x50(%rbp), %xmm0
1407f21ad: 0f 59 05 7c 8d 29 02        	mulps	0x2298d7c(%rip), %xmm0  # 0x142a8af30
1407f21b4: 0f 59 3d e5 37 13 02        	mulps	0x21337e5(%rip), %xmm7  # 0x1429259a0
1407f21bb: 0f 5c c7                    	subps	%xmm7, %xmm0
1407f21be: 0f c6 c0 d6                 	shufps	$0xd6, %xmm0, %xmm0     # xmm0 = xmm0[2,1,1,3]
1407f21c2: 41 0f 59 c0                 	mulps	%xmm8, %xmm0
1407f21c6: 0f 58 c1                    	addps	%xmm1, %xmm0
1407f21c9: f3 0f 10 8d 68 01 00 00     	movss	0x168(%rbp), %xmm1
1407f21d1: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407f21d5: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407f21d8: 0f 28 55 60                 	movaps	0x60(%rbp), %xmm2
1407f21dc: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407f21e0: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
1407f21e7: 48 8b 42 08                 	movq	0x8(%rdx), %rax
1407f21eb: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1407f21ef: 48 01 c9                    	addq	%rcx, %rcx
1407f21f2: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1407f21f9: 48 89 8d e8 01 00 00        	movq	%rcx, 0x1e8(%rbp)
1407f2200: f2 0f 10 42 50              	movsd	0x50(%rdx), %xmm0
1407f2205: f2 0f 11 85 f0 01 00 00     	movsd	%xmm0, 0x1f0(%rbp)
1407f220d: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f2214: 44 0f 28 e9                 	movaps	%xmm1, %xmm13
1407f2218: 0f 29 55 60                 	movaps	%xmm2, 0x60(%rbp)
1407f221c: e8 7f 59 a6 ff              	callq	0x140257ba0 <.text+0x256ba0>
1407f2221: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407f2225: f3 0f 5c c8                 	subss	%xmm0, %xmm1
1407f2229: 0f 2e 0d 54 37 13 02        	ucomiss	0x2133754(%rip), %xmm1  # 0x142925984
1407f2230: 0f 86 74 02 00 00           	jbe	0x1407f24aa <.text+0x7f14aa>
1407f2236: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
1407f223d: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407f2240: f3 0f 59 15 48 e4 16 02     	mulss	0x216e448(%rip), %xmm2  # 0x142960690
1407f2248: 0f 14 ca                    	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
1407f224b: 0f 59 0d 8e 36 13 02        	mulps	0x213368e(%rip), %xmm1  # 0x1429258e0
1407f2252: 44 0f 28 c9                 	movaps	%xmm1, %xmm9
1407f2256: 44 0f c6 c9 54              	shufps	$0x54, %xmm1, %xmm9     # xmm9 = xmm9[0,1],xmm1[1,1]
1407f225b: 0f 28 ee                    	movaps	%xmm6, %xmm5
1407f225e: 41 0f 59 e9                 	mulps	%xmm9, %xmm5
1407f2262: 0f 28 a5 00 01 00 00        	movaps	0x100(%rbp), %xmm4
1407f2269: 0f 28 d4                    	movaps	%xmm4, %xmm2
1407f226c: 41 0f 59 d1                 	mulps	%xmm9, %xmm2
1407f2270: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
1407f2277: f3 0f 16 da                 	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
1407f227b: f3 0f 58 da                 	addss	%xmm2, %xmm3
1407f227f: 0f 12 d2                    	movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
1407f2282: f3 0f 58 d3                 	addss	%xmm3, %xmm2
1407f2286: f3 0f 58 d2                 	addss	%xmm2, %xmm2
1407f228a: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407f228e: 0f 59 d4                    	mulps	%xmm4, %xmm2
1407f2291: 0f 58 d5                    	addps	%xmm5, %xmm2
1407f2294: 0f c6 c9 d0                 	shufps	$0xd0, %xmm1, %xmm1     # xmm1 = xmm1[0,0,1,3]
1407f2298: 44 0f 59 4d 50              	mulps	0x50(%rbp), %xmm9
1407f229d: 0f 59 cc                    	mulps	%xmm4, %xmm1
1407f22a0: 44 0f 5c c9                 	subps	%xmm1, %xmm9
1407f22a4: 45 0f c6 c9 d6              	shufps	$0xd6, %xmm9, %xmm9     # xmm9 = xmm9[2,1,1,3]
1407f22a9: 45 0f 59 c8                 	mulps	%xmm8, %xmm9
1407f22ad: 44 0f 58 ca                 	addps	%xmm2, %xmm9
1407f22b1: 45 0f 28 f9                 	movaps	%xmm9, %xmm15
1407f22b5: 66 45 0f 15 f9              	unpckhpd	%xmm9, %xmm15           # xmm15 = xmm15[1],xmm9[1]
1407f22ba: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
1407f22be: f3 41 0f 5c c9              	subss	%xmm9, %xmm1
1407f22c3: 0f 28 55 60                 	movaps	0x60(%rbp), %xmm2
1407f22c7: f3 41 0f 5c d7              	subss	%xmm15, %xmm2
1407f22cc: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
1407f22d3: 48 8b 42 08                 	movq	0x8(%rdx), %rax
1407f22d7: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1407f22db: 48 01 c9                    	addq	%rcx, %rcx
1407f22de: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1407f22e5: 48 89 8d e8 01 00 00        	movq	%rcx, 0x1e8(%rbp)
1407f22ec: f2 0f 10 42 50              	movsd	0x50(%rdx), %xmm0
1407f22f1: f2 0f 11 85 f0 01 00 00     	movsd	%xmm0, 0x1f0(%rbp)
1407f22f9: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f2300: e8 9b 58 a6 ff              	callq	0x140257ba0 <.text+0x256ba0>
1407f2305: 44 0f 28 f0                 	movaps	%xmm0, %xmm14
1407f2309: f3 45 0f 58 e9              	addss	%xmm9, %xmm13
1407f230e: 0f 28 55 60                 	movaps	0x60(%rbp), %xmm2
1407f2312: f3 41 0f 58 d7              	addss	%xmm15, %xmm2
1407f2317: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
1407f231e: 48 8b 42 08                 	movq	0x8(%rdx), %rax
1407f2322: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1407f2326: 48 01 c9                    	addq	%rcx, %rcx
1407f2329: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1407f2330: 48 89 8d e8 01 00 00        	movq	%rcx, 0x1e8(%rbp)
1407f2337: f2 0f 10 42 50              	movsd	0x50(%rdx), %xmm0
1407f233c: f2 0f 11 85 f0 01 00 00     	movsd	%xmm0, 0x1f0(%rbp)
1407f2344: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f234b: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
1407f234f: e8 4c 58 a6 ff              	callq	0x140257ba0 <.text+0x256ba0>
1407f2354: 0f 28 9d 40 01 00 00        	movaps	0x140(%rbp), %xmm3
1407f235b: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407f235e: f3 41 0f 5d ce              	minss	%xmm14, %xmm1
1407f2363: f3 45 0f c2 f6 03           	cmpunordss	%xmm14, %xmm14
1407f2369: 41 0f 28 d6                 	movaps	%xmm14, %xmm2
1407f236d: 0f 54 d0                    	andps	%xmm0, %xmm2
1407f2370: 44 0f 55 f1                 	andnps	%xmm1, %xmm14
1407f2374: 44 0f 56 f2                 	orps	%xmm2, %xmm14
1407f2378: 0f 28 c3                    	movaps	%xmm3, %xmm0
1407f237b: f3 41 0f 5d c6              	minss	%xmm14, %xmm0
1407f2380: f3 45 0f c2 f6 03           	cmpunordss	%xmm14, %xmm14
1407f2386: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
1407f238a: 0f 55 c8                    	andnps	%xmm0, %xmm1
1407f238d: 44 0f 54 f3                 	andps	%xmm3, %xmm14
1407f2391: 44 0f 56 f1                 	orps	%xmm1, %xmm14
1407f2395: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
1407f2399: f3 41 0f 5c c6              	subss	%xmm14, %xmm0
1407f239e: f3 0f 10 0d 2a 35 13 02     	movss	0x213352a(%rip), %xmm1  # 0x1429258d0
1407f23a6: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
1407f23a9: 0f 96 c0                    	setbe	%al
1407f23ac: 89 d9                       	movl	%ebx, %ecx
1407f23ae: f6 d1                       	notb	%cl
1407f23b0: 84 c1                       	testb	%al, %cl
1407f23b2: 0f 85 f2 00 00 00           	jne	0x1407f24aa <.text+0x7f14aa>
1407f23b8: f3 0f 10 05 90 d6 28 02     	movss	0x228d690(%rip), %xmm0  # 0x142a7fa50
1407f23c0: 84 db                       	testb	%bl, %bl
1407f23c2: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
1407f23c9: 75 04                       	jne	0x1407f23cf <.text+0x7f13cf>
1407f23cb: 41 0f 28 c6                 	movaps	%xmm14, %xmm0
1407f23cf: 0f 28 9d 00 01 00 00        	movaps	0x100(%rbp), %xmm3
1407f23d6: 0f 28 cb                    	movaps	%xmm3, %xmm1
1407f23d9: 0f 59 0d e0 ac 1d 02        	mulps	0x21dace0(%rip), %xmm1  # 0x1429cd0c0
1407f23e0: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1407f23e4: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407f23e8: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1407f23eb: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1407f23ef: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1407f23f3: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407f23f7: 0f 59 cb                    	mulps	%xmm3, %xmm1
1407f23fa: 0f 59 35 bf ac 1d 02        	mulps	0x21dacbf(%rip), %xmm6  # 0x1429cd0c0
1407f2401: 0f 58 ce                    	addps	%xmm6, %xmm1
1407f2404: 0f 28 55 50                 	movaps	0x50(%rbp), %xmm2
1407f2408: 0f 59 15 b1 ac 1d 02        	mulps	0x21dacb1(%rip), %xmm2  # 0x1429cd0c0
1407f240f: 0f 5c d7                    	subps	%xmm7, %xmm2
1407f2412: 0f c6 d2 d6                 	shufps	$0xd6, %xmm2, %xmm2     # xmm2 = xmm2[2,1,1,3]
1407f2416: 44 0f 59 c2                 	mulps	%xmm2, %xmm8
1407f241a: 44 0f 58 c1                 	addps	%xmm1, %xmm8
1407f241e: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1407f2422: 45 0f 59 c0                 	mulps	%xmm8, %xmm8
1407f2426: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1407f242a: 66 41 0f 15 d0              	unpckhpd	%xmm8, %xmm2            # xmm2 = xmm2[1],xmm8[1]
1407f242f: f3 41 0f 58 d0              	addss	%xmm8, %xmm2
1407f2434: f3 0f 51 d2                 	sqrtss	%xmm2, %xmm2
1407f2438: f3 0f 10 1d c4 34 13 02     	movss	0x21334c4(%rip), %xmm3  # 0x142925904
1407f2440: f3 0f 5e da                 	divss	%xmm2, %xmm3
1407f2444: f3 0f 10 15 04 d6 28 02     	movss	0x228d604(%rip), %xmm2  # 0x142a7fa50
1407f244c: f3 41 0f 5c d6              	subss	%xmm14, %xmm2
1407f2451: 0f c6 c9 e8                 	shufps	$0xe8, %xmm1, %xmm1     # xmm1 = xmm1[0,2,2,3]
1407f2455: f3 0f 12 db                 	movsldup	%xmm3, %xmm3            # xmm3 = xmm3[0,0,2,2]
1407f2459: 0f 59 d9                    	mulps	%xmm1, %xmm3
1407f245c: 44 0f 16 d3                 	movlhps	%xmm3, %xmm10           # xmm10 = xmm10[0],xmm3[0]
1407f2460: 44 0f 11 95 e0 01 00 00     	movups	%xmm10, 0x1e0(%rbp)
1407f2468: f3 44 0f 11 a5 f0 01 00 00  	movss	%xmm12, 0x1f0(%rbp)
1407f2471: f3 0f 11 85 f4 01 00 00     	movss	%xmm0, 0x1f4(%rbp)
1407f2479: f3 0f 5f 15 0f e2 16 02     	maxss	0x216e20f(%rip), %xmm2  # 0x142960690
1407f2481: f3 0f 11 95 f8 01 00 00     	movss	%xmm2, 0x1f8(%rbp)
1407f2489: 66 0f 6f 85 90 00 00 00     	movdqa	0x90(%rbp), %xmm0
1407f2491: 66 0f 7e 85 fc 01 00 00     	movd	%xmm0, 0x1fc(%rbp)
1407f2499: 48 8d 95 e0 01 00 00        	leaq	0x1e0(%rbp), %rdx
1407f24a0: e8 8b 33 03 00              	callq	0x140825830 <.text+0x824830>
1407f24a5: e9 3f 04 00 00              	jmp	0x1407f28e9 <.text+0x7f18e9>
1407f24aa: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
1407f24b1: 48 8b 81 a8 fd ff ff        	movq	-0x258(%rcx), %rax
1407f24b8: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f24bc: 0f 84 38 06 00 00           	je	0x1407f2afa <.text+0x7f1afa>
1407f24c2: 48 85 c0                    	testq	%rax, %rax
1407f24c5: 0f 84 b6 06 00 00           	je	0x1407f2b81 <.text+0x7f1b81>
1407f24cb: 48 8b 89 a0 fd ff ff        	movq	-0x260(%rcx), %rcx
1407f24d2: 48 89 8d e0 01 00 00        	movq	%rcx, 0x1e0(%rbp)
1407f24d9: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
1407f24e0: 48 8d 05 59 47 a1 00        	leaq	0xa14759(%rip), %rax    # 0x141206c40 <.text+0x1205c40>
1407f24e7: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1407f24ee: f3 0f 10 8d 18 01 00 00     	movss	0x118(%rbp), %xmm1
1407f24f6: f3 0f 5f 0d 92 e1 16 02     	maxss	0x216e192(%rip), %xmm1  # 0x142960690
1407f24fe: 48 8d 85 fc 01 00 00        	leaq	0x1fc(%rbp), %rax
1407f2505: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407f2508: 0f 11 40 fc                 	movups	%xmm0, -0x4(%rax)
1407f250c: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f2513: e8 c8 73 a9 00              	callq	0x1412898e0 <.text+0x12888e0>
1407f2518: 8b 8d 00 02 00 00           	movl	0x200(%rbp), %ecx
1407f251e: 4c 8b 85 60 01 00 00        	movq	0x160(%rbp), %r8
1407f2525: 49 8b 90 a8 fd ff ff        	movq	-0x258(%r8), %rdx
1407f252c: 48 39 ca                    	cmpq	%rcx, %rdx
1407f252f: 0f 86 61 07 00 00           	jbe	0x1407f2c96 <.text+0x7f1c96>
1407f2535: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
1407f2539: 48 39 d0                    	cmpq	%rdx, %rax
1407f253c: 0f 83 62 07 00 00           	jae	0x1407f2ca4 <.text+0x7f1ca4>
1407f2542: f3 0f 10 b5 04 02 00 00     	movss	0x204(%rbp), %xmm6
1407f254a: 49 8b 90 a0 fd ff ff        	movq	-0x260(%r8), %rdx
1407f2551: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1407f2555: f3 44 0f 10 44 8a 04        	movss	0x4(%rdx,%rcx,4), %xmm8
1407f255c: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1407f2560: f3 44 0f 10 4c 82 04        	movss	0x4(%rdx,%rax,4), %xmm9
1407f2567: f3 0f 10 3d 95 33 13 02     	movss	0x2133395(%rip), %xmm7  # 0x142925904
1407f256f: f3 0f 5c fe                 	subss	%xmm6, %xmm7
1407f2573: f3 0f 10 04 8a              	movss	(%rdx,%rcx,4), %xmm0
1407f2578: f3 0f 59 c7                 	mulss	%xmm7, %xmm0
1407f257c: f3 0f 10 5c 8a 08           	movss	0x8(%rdx,%rcx,4), %xmm3
1407f2582: f3 0f 59 df                 	mulss	%xmm7, %xmm3
1407f2586: f3 0f 10 0c 82              	movss	(%rdx,%rax,4), %xmm1
1407f258b: f3 0f 59 ce                 	mulss	%xmm6, %xmm1
1407f258f: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407f2593: f3 0f 10 54 82 08           	movss	0x8(%rdx,%rax,4), %xmm2
1407f2599: f3 0f 59 d6                 	mulss	%xmm6, %xmm2
1407f259d: f3 0f 58 d3                 	addss	%xmm3, %xmm2
1407f25a1: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
1407f25a8: 48 8b 42 08                 	movq	0x8(%rdx), %rax
1407f25ac: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1407f25b0: 48 01 c9                    	addq	%rcx, %rcx
1407f25b3: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1407f25ba: 48 89 8d e8 01 00 00        	movq	%rcx, 0x1e8(%rbp)
1407f25c1: f2 0f 10 42 50              	movsd	0x50(%rdx), %xmm0
1407f25c6: f2 0f 11 85 f0 01 00 00     	movsd	%xmm0, 0x1f0(%rbp)
1407f25ce: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f25d5: e8 c6 55 a6 ff              	callq	0x140257ba0 <.text+0x256ba0>
1407f25da: f3 41 0f 59 f8              	mulss	%xmm8, %xmm7
1407f25df: f3 41 0f 59 f1              	mulss	%xmm9, %xmm6
1407f25e4: f3 0f 58 f7                 	addss	%xmm7, %xmm6
1407f25e8: f3 0f 58 05 30 33 13 02     	addss	0x2133330(%rip), %xmm0  # 0x142925920
1407f25f0: 0f 2e f0                    	ucomiss	%xmm0, %xmm6
1407f25f3: 8b 85 dc 01 00 00           	movl	0x1dc(%rbp), %eax
1407f25f9: 89 c1                       	movl	%eax, %ecx
1407f25fb: 0f 86 10 e7 ff ff           	jbe	0x1407f0d11 <.text+0x7efd11>
1407f2601: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407f2608: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407f260c: fe c8                       	decb	%al
1407f260e: 3c 03                       	cmpb	$0x3, %al
1407f2610: 0f 83 9f 06 00 00           	jae	0x1407f2cb5 <.text+0x7f1cb5>
1407f2616: 0f b6 c0                    	movzbl	%al, %eax
1407f2619: 48 8d 0d d8 8f 29 02        	leaq	0x2298fd8(%rip), %rcx   # 0x142a8b5f8
1407f2620: 4c 63 3c 81                 	movslq	(%rcx,%rax,4), %r15
1407f2624: 48 8d 85 fc 01 00 00        	leaq	0x1fc(%rbp), %rax
1407f262b: c7 40 ef 00 00 00 00        	movl	$0x0, -0x11(%rax)
1407f2632: 48 c7 40 e7 00 00 00 00     	movq	$0x0, -0x19(%rax)
1407f263a: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
1407f2641: 48 8b 8d e7 01 00 00        	movq	0x1e7(%rbp), %rcx
1407f2648: 48 89 8d 7f 01 00 00        	movq	%rcx, 0x17f(%rbp)
1407f264f: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
1407f2656: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
1407f265d: 48 8b 98 c0 00 00 00        	movq	0xc0(%rax), %rbx
1407f2664: 48 3b 98 b0 00 00 00        	cmpq	0xb0(%rax), %rbx
1407f266b: 75 13                       	jne	0x1407f2680 <.text+0x7f1680>
1407f266d: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1407f2674: 48 8d 15 95 8e 29 02        	leaq	0x2298e95(%rip), %rdx   # 0x142a8b510
1407f267b: e8 30 8d 03 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407f2680: 48 8d 05 71 8f 29 02        	leaq	0x2298f71(%rip), %rax   # 0x142a8b5f8
1407f2687: 49 01 c7                    	addq	%rax, %r15
1407f268a: 4c 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %r9
1407f2691: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407f2698: 48 69 cb 60 01 00 00        	imulq	$0x160, %rbx, %rcx      # imm = 0x160
1407f269f: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407f26a7: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407f26b0: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407f26b9: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407f26c1: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
1407f26c8: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407f26cd: 8b 95 b8 00 00 00           	movl	0xb8(%rbp), %edx
1407f26d3: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407f26d7: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407f26df: 0f 10 85 e0 01 00 00        	movups	0x1e0(%rbp), %xmm0
1407f26e6: 0f 10 8d f0 01 00 00        	movups	0x1f0(%rbp), %xmm1
1407f26ed: 0f 10 95 00 02 00 00        	movups	0x200(%rbp), %xmm2
1407f26f4: 0f 10 9d 0c 02 00 00        	movups	0x20c(%rbp), %xmm3
1407f26fb: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407f2700: 0f 11 4c 08 54              	movups	%xmm1, 0x54(%rax,%rcx)
1407f2705: 0f 11 54 08 64              	movups	%xmm2, 0x64(%rax,%rcx)
1407f270a: 0f 11 5c 08 70              	movups	%xmm3, 0x70(%rax,%rcx)
1407f270f: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407f271a: 48 8b 95 bc 00 00 00        	movq	0xbc(%rbp), %rdx
1407f2721: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407f2729: 8b 95 c4 00 00 00           	movl	0xc4(%rbp), %edx
1407f272f: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407f2736: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407f2741: 48 8b 95 c8 00 00 00        	movq	0xc8(%rbp), %rdx
1407f2748: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407f2750: 8b 95 d0 00 00 00           	movl	0xd0(%rbp), %edx
1407f2756: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407f275d: 4c 89 bc 08 a0 00 00 00     	movq	%r15, 0xa0(%rax,%rcx)
1407f2765: 48 c7 84 08 a8 00 00 00 2a 00 00 00 	movq	$0x2a, 0xa8(%rax,%rcx)
1407f2771: 48 8d 15 76 8d 29 02        	leaq	0x2298d76(%rip), %rdx   # 0x142a8b4ee
1407f2778: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407f2780: 48 c7 84 08 b8 00 00 00 1e 00 00 00 	movq	$0x1e, 0xb8(%rax,%rcx)
1407f278c: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
1407f2793: 0f 29 84 08 c0 00 00 00     	movaps	%xmm0, 0xc0(%rax,%rcx)
1407f279b: f3 0f 10 85 28 02 00 00     	movss	0x228(%rbp), %xmm0
1407f27a3: f3 0f 11 84 08 d0 00 00 00  	movss	%xmm0, 0xd0(%rax,%rcx)
1407f27ac: 0f 28 85 a0 00 00 00        	movaps	0xa0(%rbp), %xmm0
1407f27b3: f3 0f 11 84 08 d4 00 00 00  	movss	%xmm0, 0xd4(%rax,%rcx)
1407f27bc: f3 0f 10 85 d0 01 00 00     	movss	0x1d0(%rbp), %xmm0
1407f27c4: f3 0f 11 84 08 d8 00 00 00  	movss	%xmm0, 0xd8(%rax,%rcx)
1407f27cd: 48 ba 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rdx # imm = 0x3F8000003F800000
1407f27d7: 48 89 94 08 dc 00 00 00     	movq	%rdx, 0xdc(%rax,%rcx)
1407f27df: c7 84 08 e4 00 00 00 00 00 80 3f    	movl	$0x3f800000, 0xe4(%rax,%rcx) # imm = 0x3F800000
1407f27ea: c7 84 08 f0 00 00 00 01 00 00 00    	movl	$0x1, 0xf0(%rax,%rcx)
1407f27f5: 8b 95 94 01 00 00           	movl	0x194(%rbp), %edx
1407f27fb: 89 94 08 f4 00 00 00        	movl	%edx, 0xf4(%rax,%rcx)
1407f2802: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407f280d: 48 8b 95 a8 01 00 00        	movq	0x1a8(%rbp), %rdx
1407f2814: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407f281c: 0f 10 85 98 01 00 00        	movups	0x198(%rbp), %xmm0
1407f2823: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407f282b: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407f2833: 0f b6 95 cf 01 00 00        	movzbl	0x1cf(%rbp), %edx
1407f283a: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407f2841: 0f b7 95 cd 01 00 00        	movzwl	0x1cd(%rbp), %edx
1407f2848: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407f2850: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407f285b: f3 0f 6f 85 20 01 00 00     	movdqu	0x120(%rbp), %xmm0
1407f2863: 0f 10 8d 30 01 00 00        	movups	0x130(%rbp), %xmm1
1407f286a: 0f 11 8c 08 2c 01 00 00     	movups	%xmm1, 0x12c(%rax,%rcx)
1407f2872: f3 0f 7f 84 08 1c 01 00 00  	movdqu	%xmm0, 0x11c(%rax,%rcx)
1407f287b: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407f2883: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1407f288a: 4c 8b 85 7f 01 00 00        	movq	0x17f(%rbp), %r8
1407f2891: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407f2899: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407f28a1: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407f28a9: 0f b6 95 36 02 00 00        	movzbl	0x236(%rbp), %edx
1407f28b0: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407f28b7: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407f28c1: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
1407f28c8: 4c 8b 85 e7 00 00 00        	movq	0xe7(%rbp), %r8
1407f28cf: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407f28d7: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407f28df: 48 ff c3                    	incq	%rbx
1407f28e2: 49 89 99 c0 00 00 00        	movq	%rbx, 0xc0(%r9)
1407f28e9: 8b 85 dc 01 00 00           	movl	0x1dc(%rbp), %eax
1407f28ef: 89 c1                       	movl	%eax, %ecx
1407f28f1: e9 1b e4 ff ff              	jmp	0x1407f0d11 <.text+0x7efd11>
1407f28f6: 8b 85 90 01 00 00           	movl	0x190(%rbp), %eax
1407f28fc: 4c 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %r9
1407f2903: 4c 89 8d d8 00 00 00        	movq	%r9, 0xd8(%rbp)
1407f290a: 4d 85 c9                    	testq	%r9, %r9
1407f290d: 0f 85 40 e3 ff ff           	jne	0x1407f0c53 <.text+0x7efc53>
1407f2913: 80 bd 35 02 00 00 00        	cmpb	$0x0, 0x235(%rbp)
1407f291a: 74 4b                       	je	0x1407f2967 <.text+0x7f1967>
1407f291c: e8 0f 58 42 00              	callq	0x140c18130 <.text+0xc17130>
1407f2921: 48 89 c1                    	movq	%rax, %rcx
1407f2924: 48 8b 00                    	movq	(%rax), %rax
1407f2927: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f292b: 0f 85 da 01 00 00           	jne	0x1407f2b0b <.text+0x7f1b0b>
1407f2931: 48 83 c1 08                 	addq	$0x8, %rcx
1407f2935: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407f2939: 0f 85 f0 01 00 00           	jne	0x1407f2b2f <.text+0x7f1b2f>
1407f293f: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407f2946: 48 89 8d b0 01 00 00        	movq	%rcx, 0x1b0(%rbp)
1407f294d: 48 83 c1 08                 	addq	$0x8, %rcx
1407f2951: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
1407f2958: e8 93 47 42 00              	callq	0x140c170f0 <.text+0xc160f0>
1407f295d: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
1407f2964: 48 ff 00                    	incq	(%rax)
1407f2967: 0f 28 b5 40 02 00 00        	movaps	0x240(%rbp), %xmm6
1407f296e: 0f 28 bd 50 02 00 00        	movaps	0x250(%rbp), %xmm7
1407f2975: 44 0f 28 85 60 02 00 00     	movaps	0x260(%rbp), %xmm8
1407f297d: 44 0f 28 8d 70 02 00 00     	movaps	0x270(%rbp), %xmm9
1407f2985: 44 0f 28 95 80 02 00 00     	movaps	0x280(%rbp), %xmm10
1407f298d: 44 0f 28 9d 90 02 00 00     	movaps	0x290(%rbp), %xmm11
1407f2995: 44 0f 28 a5 a0 02 00 00     	movaps	0x2a0(%rbp), %xmm12
1407f299d: 44 0f 28 ad b0 02 00 00     	movaps	0x2b0(%rbp), %xmm13
1407f29a5: 44 0f 28 b5 c0 02 00 00     	movaps	0x2c0(%rbp), %xmm14
1407f29ad: 44 0f 28 bd d0 02 00 00     	movaps	0x2d0(%rbp), %xmm15
1407f29b5: 48 81 c4 68 03 00 00        	addq	$0x368, %rsp            # imm = 0x368
1407f29bc: 5b                          	popq	%rbx
1407f29bd: 5f                          	popq	%rdi
1407f29be: 5e                          	popq	%rsi
1407f29bf: 41 5c                       	popq	%r12
1407f29c1: 41 5d                       	popq	%r13
1407f29c3: 41 5e                       	popq	%r14
1407f29c5: 41 5f                       	popq	%r15
1407f29c7: 5d                          	popq	%rbp
1407f29c8: c3                          	retq
1407f29c9: 48 8d 0d 18 e9 42 02        	leaq	0x242e918(%rip), %rcx   # 0x142c212e8
1407f29d0: e8 3b 69 0e 02              	callq	0x1428d9310 <.text+0x28d8310>
1407f29d5: e9 f3 02 00 00              	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f29da: 48 8d 0d c7 85 29 02        	leaq	0x22985c7(%rip), %rcx   # 0x142a8afa8
1407f29e1: 4c 8d 05 f0 88 29 02        	leaq	0x22988f0(%rip), %r8    # 0x142a8b2d8
1407f29e8: ba 16 00 00 00              	movl	$0x16, %edx
1407f29ed: e8 3e 69 0e 02              	callq	0x1428d9330 <.text+0x28d8330>
1407f29f2: e9 d6 02 00 00              	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f29f7: 48 8d 85 98 01 00 00        	leaq	0x198(%rbp), %rax
1407f29fe: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1407f2a05: 48 8d 05 d4 d9 ff ff        	leaq	-0x262c(%rip), %rax     # 0x1407f03e0 <.text+0x7ef3e0>
1407f2a0c: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
1407f2a13: 48 8d 05 3e 86 29 02        	leaq	0x229863e(%rip), %rax   # 0x142a8b058
1407f2a1a: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1407f2a21: 48 c7 85 e8 01 00 00 01 00 00 00    	movq	$0x1, 0x1e8(%rbp)
1407f2a2c: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
1407f2a37: 48 8d 85 20 01 00 00        	leaq	0x120(%rbp), %rax
1407f2a3e: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1407f2a45: 48 c7 85 f8 01 00 00 01 00 00 00    	movq	$0x1, 0x1f8(%rbp)
1407f2a50: 48 8d 15 11 86 29 02        	leaq	0x2298611(%rip), %rdx   # 0x142a8b068
1407f2a57: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f2a5e: e8 2d 69 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f2a63: e9 65 02 00 00              	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2a68: 48 8d 85 98 01 00 00        	leaq	0x198(%rbp), %rax
1407f2a6f: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1407f2a76: 48 8d 05 63 d9 ff ff        	leaq	-0x269d(%rip), %rax     # 0x1407f03e0 <.text+0x7ef3e0>
1407f2a7d: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
1407f2a84: 48 8d 05 cd 85 29 02        	leaq	0x22985cd(%rip), %rax   # 0x142a8b058
1407f2a8b: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1407f2a92: 48 c7 85 e8 01 00 00 01 00 00 00    	movq	$0x1, 0x1e8(%rbp)
1407f2a9d: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
1407f2aa8: 48 8d 85 20 01 00 00        	leaq	0x120(%rbp), %rax
1407f2aaf: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1407f2ab6: 48 c7 85 f8 01 00 00 01 00 00 00    	movq	$0x1, 0x1f8(%rbp)
1407f2ac1: c6 85 37 02 00 00 01        	movb	$0x1, 0x237(%rbp)
1407f2ac8: 48 8d 15 b1 85 29 02        	leaq	0x22985b1(%rip), %rdx   # 0x142a8b080
1407f2acf: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f2ad6: e8 b5 68 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f2adb: e9 ed 01 00 00              	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2ae0: 48 85 c0                    	testq	%rax, %rax
1407f2ae3: 75 3e                       	jne	0x1407f2b23 <.text+0x7f1b23>
1407f2ae5: 31 d2                       	xorl	%edx, %edx
1407f2ae7: e8 94 fa 0f 02              	callq	0x1428f2580 <.text+0x28f1580>
1407f2aec: 48 89 c1                    	movq	%rax, %rcx
1407f2aef: 48 85 c0                    	testq	%rax, %rax
1407f2af2: 0f 85 64 e0 ff ff           	jne	0x1407f0b5c <.text+0x7efb5c>
1407f2af8: eb 29                       	jmp	0x1407f2b23 <.text+0x7f1b23>
1407f2afa: 48 8d 0d d7 61 43 02        	leaq	0x24361d7(%rip), %rcx   # 0x142c28cd8
1407f2b01: e8 0a 68 0e 02              	callq	0x1428d9310 <.text+0x28d8310>
1407f2b06: e9 c2 01 00 00              	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2b0b: 48 85 c0                    	testq	%rax, %rax
1407f2b0e: 75 13                       	jne	0x1407f2b23 <.text+0x7f1b23>
1407f2b10: 31 d2                       	xorl	%edx, %edx
1407f2b12: e8 69 fa 0f 02              	callq	0x1428f2580 <.text+0x28f1580>
1407f2b17: 48 89 c1                    	movq	%rax, %rcx
1407f2b1a: 48 85 c0                    	testq	%rax, %rax
1407f2b1d: 0f 85 12 fe ff ff           	jne	0x1407f2935 <.text+0x7f1935>
1407f2b23: 48 8d 0d 9e e8 42 02        	leaq	0x242e89e(%rip), %rcx   # 0x142c213c8
1407f2b2a: e8 a1 55 0e 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407f2b2f: 48 8d 0d 02 e9 42 02        	leaq	0x242e902(%rip), %rcx   # 0x142c21438
1407f2b36: e8 15 67 0e 02              	callq	0x1428d9250 <.text+0x28d8250>
1407f2b3b: 48 8d 05 e6 eb 29 02        	leaq	0x229ebe6(%rip), %rax   # 0x142a91728
1407f2b42: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1407f2b49: 48 c7 85 e8 01 00 00 01 00 00 00    	movq	$0x1, 0x1e8(%rbp)
1407f2b54: 48 c7 85 f0 01 00 00 08 00 00 00    	movq	$0x8, 0x1f0(%rbp)
1407f2b5f: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407f2b62: 0f 11 85 f8 01 00 00        	movups	%xmm0, 0x1f8(%rbp)
1407f2b69: 48 8d 15 c8 eb 29 02        	leaq	0x229ebc8(%rip), %rdx   # 0x142a91738
1407f2b70: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f2b77: e8 14 68 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f2b7c: e9 4c 01 00 00              	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2b81: 48 8d 0d 08 61 43 02        	leaq	0x2436108(%rip), %rcx   # 0x142c28c90
1407f2b88: 4c 8d 05 31 61 43 02        	leaq	0x2436131(%rip), %r8    # 0x142c28cc0
1407f2b8f: ba 2d 00 00 00              	movl	$0x2d, %edx
1407f2b94: e8 97 68 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f2b99: e9 2f 01 00 00              	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2b9e: 48 8d 85 98 01 00 00        	leaq	0x198(%rbp), %rax
1407f2ba5: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1407f2bac: 48 8d 05 3d a3 a6 00        	leaq	0xa6a33d(%rip), %rax    # 0x14125cef0 <.text+0x125bef0>
1407f2bb3: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
1407f2bba: 48 8d 05 bf f0 42 02        	leaq	0x242f0bf(%rip), %rax   # 0x142c21c80
1407f2bc1: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1407f2bc8: 48 c7 85 e8 01 00 00 01 00 00 00    	movq	$0x1, 0x1e8(%rbp)
1407f2bd3: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
1407f2bde: 48 8d 85 20 01 00 00        	leaq	0x120(%rbp), %rax
1407f2be5: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1407f2bec: 48 c7 85 f8 01 00 00 01 00 00 00    	movq	$0x1, 0x1f8(%rbp)
1407f2bf7: 48 8d 15 aa f0 42 02        	leaq	0x242f0aa(%rip), %rdx   # 0x142c21ca8
1407f2bfe: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1407f2c05: e8 86 67 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f2c0a: e9 be 00 00 00              	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2c0f: 48 8d 0d 12 89 29 02        	leaq	0x2298912(%rip), %rcx   # 0x142a8b528
1407f2c16: 4c 8d 05 93 89 29 02        	leaq	0x2298993(%rip), %r8    # 0x142a8b5b0
1407f2c1d: ba 28 00 00 00              	movl	$0x28, %edx
1407f2c22: e8 09 68 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f2c27: e9 a1 00 00 00              	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2c2c: 48 8d 0d f5 88 29 02        	leaq	0x22988f5(%rip), %rcx   # 0x142a8b528
1407f2c33: 4c 8d 05 2e 89 29 02        	leaq	0x229892e(%rip), %r8    # 0x142a8b568
1407f2c3a: ba 28 00 00 00              	movl	$0x28, %edx
1407f2c3f: e8 ec 67 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f2c44: e9 84 00 00 00              	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2c49: 48 8d 0d d8 88 29 02        	leaq	0x22988d8(%rip), %rcx   # 0x142a8b528
1407f2c50: 4c 8d 05 71 89 29 02        	leaq	0x2298971(%rip), %r8    # 0x142a8b5c8
1407f2c57: ba 28 00 00 00              	movl	$0x28, %edx
1407f2c5c: e8 cf 67 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f2c61: eb 6a                       	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2c63: b9 04 00 00 00              	movl	$0x4, %ecx
1407f2c68: ba 24 00 00 00              	movl	$0x24, %edx
1407f2c6d: e8 8e 63 0e 02              	callq	0x1428d9000 <.text+0x28d8000>
1407f2c72: eb 59                       	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2c74: b9 04 00 00 00              	movl	$0x4, %ecx
1407f2c79: ba 24 00 00 00              	movl	$0x24, %edx
1407f2c7e: e8 7d 63 0e 02              	callq	0x1428d9000 <.text+0x28d8000>
1407f2c83: eb 48                       	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2c85: b9 04 00 00 00              	movl	$0x4, %ecx
1407f2c8a: ba 24 00 00 00              	movl	$0x24, %edx
1407f2c8f: e8 6c 63 0e 02              	callq	0x1428d9000 <.text+0x28d8000>
1407f2c94: eb 37                       	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2c96: 4c 8d 05 4b f9 42 02        	leaq	0x242f94b(%rip), %r8    # 0x142c225e8
1407f2c9d: e8 76 68 0e 02              	callq	0x1428d9518 <.text+0x28d8518>
1407f2ca2: eb 29                       	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2ca4: 4c 8d 05 55 f9 42 02        	leaq	0x242f955(%rip), %r8    # 0x142c22600
1407f2cab: 48 89 c1                    	movq	%rax, %rcx
1407f2cae: e8 65 68 0e 02              	callq	0x1428d9518 <.text+0x28d8518>
1407f2cb3: eb 18                       	jmp	0x1407f2ccd <.text+0x7f1ccd>
1407f2cb5: 48 8d 0d 6c 88 29 02        	leaq	0x229886c(%rip), %rcx   # 0x142a8b528
1407f2cbc: 4c 8d 05 8d 88 29 02        	leaq	0x229888d(%rip), %r8    # 0x142a8b550
1407f2cc3: ba 28 00 00 00              	movl	$0x28, %edx
1407f2cc8: e8 63 67 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f2ccd: 0f 0b                       	ud2
1407f2ccf: 90                          	nop
1407f2cd0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f2cd5: 55                          	pushq	%rbp
1407f2cd6: 41 57                       	pushq	%r15
1407f2cd8: 41 56                       	pushq	%r14
1407f2cda: 41 55                       	pushq	%r13
1407f2cdc: 41 54                       	pushq	%r12
1407f2cde: 56                          	pushq	%rsi
1407f2cdf: 57                          	pushq	%rdi
1407f2ce0: 53                          	pushq	%rbx
1407f2ce1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407f2ce8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f2cef: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407f2cf5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407f2cfb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407f2d01: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407f2d07: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
1407f2d0e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407f2d17: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407f2d20: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407f2d29: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407f2d31: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407f2d39: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
1407f2d40: 48 ff 00                    	incq	(%rax)
1407f2d43: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f2d4b: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407f2d53: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f2d5c: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f2d65: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f2d6e: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407f2d74: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f2d7a: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f2d80: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f2d86: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f2d8c: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f2d93: 5b                          	popq	%rbx
1407f2d94: 5f                          	popq	%rdi
1407f2d95: 5e                          	popq	%rsi
1407f2d96: 41 5c                       	popq	%r12
1407f2d98: 41 5d                       	popq	%r13
1407f2d9a: 41 5e                       	popq	%r14
1407f2d9c: 41 5f                       	popq	%r15
1407f2d9e: 5d                          	popq	%rbp
1407f2d9f: c3                          	retq
1407f2da0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f2da5: 55                          	pushq	%rbp
1407f2da6: 41 57                       	pushq	%r15
1407f2da8: 41 56                       	pushq	%r14
1407f2daa: 41 55                       	pushq	%r13
1407f2dac: 41 54                       	pushq	%r12
1407f2dae: 56                          	pushq	%rsi
1407f2daf: 57                          	pushq	%rdi
1407f2db0: 53                          	pushq	%rbx
1407f2db1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407f2db8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f2dbf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407f2dc5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407f2dcb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407f2dd1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407f2dd7: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
1407f2dde: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407f2de7: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407f2df0: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407f2df9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407f2e01: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407f2e09: 80 bd 35 02 00 00 00        	cmpb	$0x0, 0x235(%rbp)
1407f2e10: 74 3d                       	je	0x1407f2e4f <.text+0x7f1e4f>
1407f2e12: e8 19 53 42 00              	callq	0x140c18130 <.text+0xc17130>
1407f2e17: 48 89 c6                    	movq	%rax, %rsi
1407f2e1a: 48 8b 00                    	movq	(%rax), %rax
1407f2e1d: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f2e21: 0f 85 85 00 00 00           	jne	0x1407f2eac <.text+0x7f1eac>
1407f2e27: 48 83 c6 08                 	addq	$0x8, %rsi
1407f2e2b: 48 83 3e 00                 	cmpq	$0x0, (%rsi)
1407f2e2f: 0f 85 9e 00 00 00           	jne	0x1407f2ed3 <.text+0x7f1ed3>
1407f2e35: 48 c7 06 ff ff ff ff        	movq	$-0x1, (%rsi)
1407f2e3c: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
1407f2e40: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
1407f2e47: e8 a4 42 42 00              	callq	0x140c170f0 <.text+0xc160f0>
1407f2e4c: 48 ff 06                    	incq	(%rsi)
1407f2e4f: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f2e57: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407f2e5f: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f2e68: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f2e71: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f2e7a: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407f2e80: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f2e86: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f2e8c: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f2e92: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f2e98: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f2e9f: 5b                          	popq	%rbx
1407f2ea0: 5f                          	popq	%rdi
1407f2ea1: 5e                          	popq	%rsi
1407f2ea2: 41 5c                       	popq	%r12
1407f2ea4: 41 5d                       	popq	%r13
1407f2ea6: 41 5e                       	popq	%r14
1407f2ea8: 41 5f                       	popq	%r15
1407f2eaa: 5d                          	popq	%rbp
1407f2eab: c3                          	retq
1407f2eac: 48 85 c0                    	testq	%rax, %rax
1407f2eaf: 75 16                       	jne	0x1407f2ec7 <.text+0x7f1ec7>
1407f2eb1: 48 89 f1                    	movq	%rsi, %rcx
1407f2eb4: 31 d2                       	xorl	%edx, %edx
1407f2eb6: e8 c5 f6 0f 02              	callq	0x1428f2580 <.text+0x28f1580>
1407f2ebb: 48 89 c6                    	movq	%rax, %rsi
1407f2ebe: 48 85 c0                    	testq	%rax, %rax
1407f2ec1: 0f 85 64 ff ff ff           	jne	0x1407f2e2b <.text+0x7f1e2b>
1407f2ec7: 48 8d 0d fa e4 42 02        	leaq	0x242e4fa(%rip), %rcx   # 0x142c213c8
1407f2ece: e8 fd 51 0e 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407f2ed3: 48 8d 0d 5e e5 42 02        	leaq	0x242e55e(%rip), %rcx   # 0x142c21438
1407f2eda: e8 71 63 0e 02              	callq	0x1428d9250 <.text+0x28d8250>
1407f2edf: cc                          	int3
1407f2ee0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f2ee5: 55                          	pushq	%rbp
1407f2ee6: 41 57                       	pushq	%r15
1407f2ee8: 41 56                       	pushq	%r14
1407f2eea: 41 55                       	pushq	%r13
1407f2eec: 41 54                       	pushq	%r12
1407f2eee: 56                          	pushq	%rsi
1407f2eef: 57                          	pushq	%rdi
1407f2ef0: 53                          	pushq	%rbx
1407f2ef1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407f2ef8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f2eff: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407f2f05: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407f2f0b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407f2f11: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407f2f17: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
1407f2f1e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407f2f27: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407f2f30: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407f2f39: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407f2f41: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407f2f49: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
1407f2f50: 48 ff 00                    	incq	(%rax)
1407f2f53: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f2f5b: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407f2f63: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f2f6c: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f2f75: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f2f7e: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407f2f84: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f2f8a: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f2f90: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f2f96: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f2f9c: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f2fa3: 5b                          	popq	%rbx
1407f2fa4: 5f                          	popq	%rdi
1407f2fa5: 5e                          	popq	%rsi
1407f2fa6: 41 5c                       	popq	%r12
1407f2fa8: 41 5d                       	popq	%r13
1407f2faa: 41 5e                       	popq	%r14
1407f2fac: 41 5f                       	popq	%r15
1407f2fae: 5d                          	popq	%rbp
1407f2faf: c3                          	retq
1407f2fb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f2fb5: 55                          	pushq	%rbp
1407f2fb6: 41 57                       	pushq	%r15
1407f2fb8: 41 56                       	pushq	%r14
1407f2fba: 41 55                       	pushq	%r13
1407f2fbc: 41 54                       	pushq	%r12
1407f2fbe: 56                          	pushq	%rsi
1407f2fbf: 57                          	pushq	%rdi
1407f2fc0: 53                          	pushq	%rbx
1407f2fc1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407f2fc8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f2fcf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407f2fd5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407f2fdb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407f2fe1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407f2fe7: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
1407f2fee: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407f2ff7: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407f3000: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407f3009: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407f3011: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407f3019: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
1407f3020: 0f b6 9d 34 02 00 00        	movzbl	0x234(%rbp), %ebx
1407f3027: f6 c3 01                    	testb	$0x1, %bl
1407f302a: 74 76                       	je	0x1407f30a2 <.text+0x7f20a2>
1407f302c: f0                          	lock
1407f302d: 48 ff 08                    	decq	(%rax)
1407f3030: 75 0c                       	jne	0x1407f303e <.text+0x7f203e>
1407f3032: 48 8d 8d 98 01 00 00        	leaq	0x198(%rbp), %rcx
1407f3039: e8 e2 95 36 00              	callq	0x140b5c620 <.text+0xb5b620>
1407f303e: 88 9d 37 02 00 00           	movb	%bl, 0x237(%rbp)
1407f3044: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f304c: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407f3054: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f305d: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f3066: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f306f: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
1407f3076: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f307c: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f3082: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f3088: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f308e: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f3095: 5b                          	popq	%rbx
1407f3096: 5f                          	popq	%rdi
1407f3097: 5e                          	popq	%rsi
1407f3098: 41 5c                       	popq	%r12
1407f309a: 41 5d                       	popq	%r13
1407f309c: 41 5e                       	popq	%r14
1407f309e: 41 5f                       	popq	%r15
1407f30a0: 5d                          	popq	%rbp
1407f30a1: c3                          	retq
1407f30a2: 88 9d 37 02 00 00           	movb	%bl, 0x237(%rbp)
1407f30a8: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f30b0: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407f30b8: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f30c1: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f30ca: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f30d3: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
1407f30da: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f30e0: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f30e6: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f30ec: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f30f2: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f30f9: 5b                          	popq	%rbx
1407f30fa: 5f                          	popq	%rdi
1407f30fb: 5e                          	popq	%rsi
1407f30fc: 41 5c                       	popq	%r12
1407f30fe: 41 5d                       	popq	%r13
1407f3100: 41 5e                       	popq	%r14
1407f3102: 41 5f                       	popq	%r15
1407f3104: 5d                          	popq	%rbp
1407f3105: c3                          	retq
1407f3106: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407f3110: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f3115: 55                          	pushq	%rbp
1407f3116: 41 57                       	pushq	%r15
1407f3118: 41 56                       	pushq	%r14
1407f311a: 41 55                       	pushq	%r13
1407f311c: 41 54                       	pushq	%r12
1407f311e: 56                          	pushq	%rsi
1407f311f: 57                          	pushq	%rdi
1407f3120: 53                          	pushq	%rbx
1407f3121: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407f3128: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f312f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407f3135: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407f313b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407f3141: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407f3147: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
1407f314e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407f3157: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407f3160: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407f3169: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407f3171: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407f3179: f6 85 37 02 00 00 01        	testb	$0x1, 0x237(%rbp)
1407f3180: 74 77                       	je	0x1407f31f9 <.text+0x7f21f9>
1407f3182: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
1407f3189: f0                          	lock
1407f318a: 48 ff 08                    	decq	(%rax)
1407f318d: 75 0c                       	jne	0x1407f319b <.text+0x7f219b>
1407f318f: 48 8d 8d 78 01 00 00        	leaq	0x178(%rbp), %rcx
1407f3196: e8 85 94 36 00              	callq	0x140b5c620 <.text+0xb5b620>
1407f319b: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f31a3: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407f31ab: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f31b4: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f31bd: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f31c6: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
1407f31cd: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f31d3: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f31d9: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f31df: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f31e5: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f31ec: 5b                          	popq	%rbx
1407f31ed: 5f                          	popq	%rdi
1407f31ee: 5e                          	popq	%rsi
1407f31ef: 41 5c                       	popq	%r12
1407f31f1: 41 5d                       	popq	%r13
1407f31f3: 41 5e                       	popq	%r14
1407f31f5: 41 5f                       	popq	%r15
1407f31f7: 5d                          	popq	%rbp
1407f31f8: c3                          	retq
1407f31f9: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f3201: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407f3209: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f3212: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f321b: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f3224: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
1407f322b: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f3231: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f3237: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f323d: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f3243: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f324a: 5b                          	popq	%rbx
1407f324b: 5f                          	popq	%rdi
1407f324c: 5e                          	popq	%rsi
1407f324d: 41 5c                       	popq	%r12
1407f324f: 41 5d                       	popq	%r13
1407f3251: 41 5e                       	popq	%r14
1407f3253: 41 5f                       	popq	%r15
1407f3255: 5d                          	popq	%rbp
1407f3256: c3                          	retq
1407f3257: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1407f3260: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f3265: 55                          	pushq	%rbp
1407f3266: 41 57                       	pushq	%r15
1407f3268: 41 56                       	pushq	%r14
1407f326a: 41 55                       	pushq	%r13
1407f326c: 41 54                       	pushq	%r12
1407f326e: 56                          	pushq	%rsi
1407f326f: 57                          	pushq	%rdi
1407f3270: 53                          	pushq	%rbx
1407f3271: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407f3278: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f327f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407f3285: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407f328b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407f3291: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407f3297: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
1407f329e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407f32a7: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407f32b0: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407f32b9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407f32c1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407f32c9: ba 24 00 00 00              	movl	$0x24, %edx
1407f32ce: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407f32d4: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
1407f32db: e8 40 09 e2 ff              	callq	0x140613c20 <.text+0x612c20>
1407f32e0: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f32e8: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407f32f0: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f32f9: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f3302: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f330b: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
1407f3312: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f3318: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f331e: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f3324: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f332a: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f3331: 5b                          	popq	%rbx
1407f3332: 5f                          	popq	%rdi
1407f3333: 5e                          	popq	%rsi
1407f3334: 41 5c                       	popq	%r12
1407f3336: 41 5d                       	popq	%r13
1407f3338: 41 5e                       	popq	%r14
1407f333a: 41 5f                       	popq	%r15
1407f333c: 5d                          	popq	%rbp
1407f333d: c3                          	retq
1407f333e: cc                          	int3
1407f333f: cc                          	int3
