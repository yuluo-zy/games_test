
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140ac98e0: 55                          	pushq	%rbp
140ac98e1: 41 57                       	pushq	%r15
140ac98e3: 41 56                       	pushq	%r14
140ac98e5: 41 55                       	pushq	%r13
140ac98e7: 41 54                       	pushq	%r12
140ac98e9: 56                          	pushq	%rsi
140ac98ea: 57                          	pushq	%rdi
140ac98eb: 53                          	pushq	%rbx
140ac98ec: 48 81 ec 78 04 00 00        	subq	$0x478, %rsp            # imm = 0x478
140ac98f3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140ac98fb: 44 0f 29 bd e0 03 00 00     	movaps	%xmm15, 0x3e0(%rbp)
140ac9903: 44 0f 29 b5 d0 03 00 00     	movaps	%xmm14, 0x3d0(%rbp)
140ac990b: 66 44 0f 7f ad c0 03 00 00  	movdqa	%xmm13, 0x3c0(%rbp)
140ac9914: 66 44 0f 7f a5 b0 03 00 00  	movdqa	%xmm12, 0x3b0(%rbp)
140ac991d: 66 44 0f 7f 9d a0 03 00 00  	movdqa	%xmm11, 0x3a0(%rbp)
140ac9926: 44 0f 29 95 90 03 00 00     	movaps	%xmm10, 0x390(%rbp)
140ac992e: 66 44 0f 7f 8d 80 03 00 00  	movdqa	%xmm9, 0x380(%rbp)
140ac9937: 66 44 0f 7f 85 70 03 00 00  	movdqa	%xmm8, 0x370(%rbp)
140ac9940: 0f 29 bd 60 03 00 00        	movaps	%xmm7, 0x360(%rbp)
140ac9947: 0f 29 b5 50 03 00 00        	movaps	%xmm6, 0x350(%rbp)
140ac994e: 48 c7 85 48 03 00 00 fe ff ff ff    	movq	$-0x2, 0x348(%rbp)
140ac9959: 48 8b 02                    	movq	(%rdx), %rax
140ac995c: 83 38 01                    	cmpl	$0x1, (%rax)
140ac995f: 0f 85 e1 01 00 00           	jne	0x140ac9b46 <.text+0xac8b46>
140ac9965: 4c 89 cb                    	movq	%r9, %rbx
140ac9968: 48 89 8d 10 03 00 00        	movq	%rcx, 0x310(%rbp)
140ac996f: 48 8b 8d a8 04 00 00        	movq	0x4a8(%rbp), %rcx
140ac9976: 48 8b 95 68 04 00 00        	movq	0x468(%rbp), %rdx
140ac997d: 4c 8b 8d 60 04 00 00        	movq	0x460(%rbp), %r9
140ac9984: 4d 8b 11                    	movq	(%r9), %r10
140ac9987: 4c 89 95 68 02 00 00        	movq	%r10, 0x268(%rbp)
140ac998e: 4d 8b 51 20                 	movq	0x20(%r9), %r10
140ac9992: 4c 89 95 b0 00 00 00        	movq	%r10, 0xb0(%rbp)
140ac9999: 4d 8b 49 28                 	movq	0x28(%r9), %r9
140ac999d: 4c 89 4d 00                 	movq	%r9, (%rbp)
140ac99a1: 4c 8b 0a                    	movq	(%rdx), %r9
140ac99a4: 4c 89 8d 30 02 00 00        	movq	%r9, 0x230(%rbp)
140ac99ab: 4c 8b 4a 20                 	movq	0x20(%rdx), %r9
140ac99af: 4c 89 8d b8 00 00 00        	movq	%r9, 0xb8(%rbp)
140ac99b6: 4c 8b 4a 30                 	movq	0x30(%rdx), %r9
140ac99ba: 4c 89 4d 08                 	movq	%r9, 0x8(%rbp)
140ac99be: 8b 52 3c                    	movl	0x3c(%rdx), %edx
140ac99c1: 89 95 08 02 00 00           	movl	%edx, 0x208(%rbp)
140ac99c7: 48 8b 11                    	movq	(%rcx), %rdx
140ac99ca: 48 89 95 c8 00 00 00        	movq	%rdx, 0xc8(%rbp)
140ac99d1: 48 8b 51 20                 	movq	0x20(%rcx), %rdx
140ac99d5: 48 89 95 40 02 00 00        	movq	%rdx, 0x240(%rbp)
140ac99dc: 48 8b 51 30                 	movq	0x30(%rcx), %rdx
140ac99e0: 48 89 55 18                 	movq	%rdx, 0x18(%rbp)
140ac99e4: 8b 51 3c                    	movl	0x3c(%rcx), %edx
140ac99e7: 89 95 10 02 00 00           	movl	%edx, 0x210(%rbp)
140ac99ed: 48 8b 51 40                 	movq	0x40(%rcx), %rdx
140ac99f1: 48 89 95 48 02 00 00        	movq	%rdx, 0x248(%rbp)
140ac99f8: 48 8b 51 50                 	movq	0x50(%rcx), %rdx
140ac99fc: 48 89 55 20                 	movq	%rdx, 0x20(%rbp)
140ac9a00: 8b 51 5c                    	movl	0x5c(%rcx), %edx
140ac9a03: 89 95 14 02 00 00           	movl	%edx, 0x214(%rbp)
140ac9a09: 48 8b 51 60                 	movq	0x60(%rcx), %rdx
140ac9a0d: 48 89 95 38 02 00 00        	movq	%rdx, 0x238(%rbp)
140ac9a14: 48 8b 51 70                 	movq	0x70(%rcx), %rdx
140ac9a18: 48 89 55 10                 	movq	%rdx, 0x10(%rbp)
140ac9a1c: 8b 49 7c                    	movl	0x7c(%rcx), %ecx
140ac9a1f: 89 8d 0c 02 00 00           	movl	%ecx, 0x20c(%rbp)
140ac9a25: 4c 8b 68 08                 	movq	0x8(%rax), %r13
140ac9a29: 4c 89 ad 20 03 00 00        	movq	%r13, 0x320(%rbp)
140ac9a30: 4d 8b 20                    	movq	(%r8), %r12
140ac9a33: 4c 89 ad 00 01 00 00        	movq	%r13, 0x100(%rbp)
140ac9a3a: 49 83 7c 24 18 00           	cmpq	$0x0, 0x18(%r12)
140ac9a40: 0f 84 62 01 00 00           	je	0x140ac9ba8 <.text+0xac8ba8>
140ac9a46: 48 8b bd b0 04 00 00        	movq	0x4b0(%rbp), %rdi
140ac9a4d: 4c 8b b5 98 04 00 00        	movq	0x498(%rbp), %r14
140ac9a54: 48 8b b5 90 04 00 00        	movq	0x490(%rbp), %rsi
140ac9a5b: 4c 8b bd 88 04 00 00        	movq	0x488(%rbp), %r15
140ac9a62: 49 8d 4c 24 20              	leaq	0x20(%r12), %rcx
140ac9a67: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
140ac9a6e: e8 fd 72 ae ff              	callq	0x1405b0d70 <.text+0x5afd70>
140ac9a73: 49 8b 0c 24                 	movq	(%r12), %rcx
140ac9a77: 49 8b 54 24 08              	movq	0x8(%r12), %rdx
140ac9a7c: 49 89 d0                    	movq	%rdx, %r8
140ac9a7f: 49 21 c0                    	andq	%rax, %r8
140ac9a82: 48 c1 e8 39                 	shrq	$0x39, %rax
140ac9a86: 66 0f 6e c0                 	movd	%eax, %xmm0
140ac9a8a: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140ac9a8e: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140ac9a93: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140ac9a98: 48 8d 81 58 fd ff ff        	leaq	-0x2a8(%rcx), %rax
140ac9a9f: 45 31 c9                    	xorl	%r9d, %r9d
140ac9aa2: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140ac9aa6: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
140ac9aac: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140ac9ab0: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140ac9ab4: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
140ac9ab9: 45 85 d2                    	testl	%r10d, %r10d
140ac9abc: 74 28                       	je	0x140ac9ae6 <.text+0xac8ae6>
140ac9abe: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140ac9ac3: 4d 01 c3                    	addq	%r8, %r11
140ac9ac6: 49 21 d3                    	andq	%rdx, %r11
140ac9ac9: 49 f7 db                    	negq	%r11
140ac9acc: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
140ac9ad3: 4e 39 2c 18                 	cmpq	%r13, (%rax,%r11)
140ac9ad7: 74 2f                       	je	0x140ac9b08 <.text+0xac8b08>
140ac9ad9: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140ac9add: 66 45 21 d3                 	andw	%r10w, %r11w
140ac9ae1: 45 89 da                    	movl	%r11d, %r10d
140ac9ae4: 75 d8                       	jne	0x140ac9abe <.text+0xac8abe>
140ac9ae6: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140ac9aea: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140ac9aef: 45 85 d2                    	testl	%r10d, %r10d
140ac9af2: 0f 85 b0 00 00 00           	jne	0x140ac9ba8 <.text+0xac8ba8>
140ac9af8: 4d 01 c8                    	addq	%r9, %r8
140ac9afb: 49 83 c0 10                 	addq	$0x10, %r8
140ac9aff: 49 83 c1 10                 	addq	$0x10, %r9
140ac9b03: 49 21 d0                    	andq	%rdx, %r8
140ac9b06: eb 9e                       	jmp	0x140ac9aa6 <.text+0xac8aa6>
140ac9b08: 4a 83 bc 19 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rcx,%r11)
140ac9b11: 0f 82 66 1c 00 00           	jb	0x140acb77d <.text+0xaca77d>
140ac9b17: f3 42 0f 10 84 19 90 fd ff ff       	movss	-0x270(%rcx,%r11), %xmm0
140ac9b21: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140ac9b25: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140ac9b28: 0f 86 4f 1c 00 00           	jbe	0x140acb77d <.text+0xaca77d>
140ac9b2e: 4a 8d 04 19                 	leaq	(%rcx,%r11), %rax
140ac9b32: 48 89 85 28 03 00 00        	movq	%rax, 0x328(%rbp)
140ac9b39: 83 78 b8 02                 	cmpl	$0x2, -0x48(%rax)
140ac9b3d: 4c 8b ad 10 03 00 00        	movq	0x310(%rbp), %r13
140ac9b44: 73 6e                       	jae	0x140ac9bb4 <.text+0xac8bb4>
140ac9b46: 0f 28 b5 50 03 00 00        	movaps	0x350(%rbp), %xmm6
140ac9b4d: 0f 28 bd 60 03 00 00        	movaps	0x360(%rbp), %xmm7
140ac9b54: 44 0f 28 85 70 03 00 00     	movaps	0x370(%rbp), %xmm8
140ac9b5c: 44 0f 28 8d 80 03 00 00     	movaps	0x380(%rbp), %xmm9
140ac9b64: 44 0f 28 95 90 03 00 00     	movaps	0x390(%rbp), %xmm10
140ac9b6c: 44 0f 28 9d a0 03 00 00     	movaps	0x3a0(%rbp), %xmm11
140ac9b74: 44 0f 28 a5 b0 03 00 00     	movaps	0x3b0(%rbp), %xmm12
140ac9b7c: 44 0f 28 ad c0 03 00 00     	movaps	0x3c0(%rbp), %xmm13
140ac9b84: 44 0f 28 b5 d0 03 00 00     	movaps	0x3d0(%rbp), %xmm14
140ac9b8c: 44 0f 28 bd e0 03 00 00     	movaps	0x3e0(%rbp), %xmm15
140ac9b94: 48 81 c4 78 04 00 00        	addq	$0x478, %rsp            # imm = 0x478
140ac9b9b: 5b                          	popq	%rbx
140ac9b9c: 5f                          	popq	%rdi
140ac9b9d: 5e                          	popq	%rsi
140ac9b9e: 41 5c                       	popq	%r12
140ac9ba0: 41 5d                       	popq	%r13
140ac9ba2: 41 5e                       	popq	%r14
140ac9ba4: 41 5f                       	popq	%r15
140ac9ba6: 5d                          	popq	%rbp
140ac9ba7: c3                          	retq
140ac9ba8: 48 8d 0d 19 6d 01 02        	leaq	0x2016d19(%rip), %rcx   # 0x142ae08c8
140ac9baf: e8 5c f7 e0 01              	callq	0x1428d9310 <.text+0x28d8310>
140ac9bb4: 4a 8d 04 19                 	leaq	(%rcx,%r11), %rax
140ac9bb8: 48 05 60 fd ff ff           	addq	$-0x2a0, %rax           # imm = 0xFD60
140ac9bbe: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
140ac9bc2: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140ac9bc9: 48 8b 88 68 fd ff ff        	movq	-0x298(%rax), %rcx
140ac9bd0: 48 8b 80 70 fd ff ff        	movq	-0x290(%rax), %rax
140ac9bd7: 48 8d 04 c1                 	leaq	(%rcx,%rax,8), %rax
140ac9bdb: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140ac9bdf: 48 8b 85 70 04 00 00        	movq	0x470(%rbp), %rax
140ac9be6: 48 8b 00                    	movq	(%rax), %rax
140ac9be9: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140ac9bed: 48 8b 85 80 04 00 00        	movq	0x480(%rbp), %rax
140ac9bf4: 48 8b 00                    	movq	(%rax), %rax
140ac9bf7: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140ac9bfb: 48 8b 85 a0 04 00 00        	movq	0x4a0(%rbp), %rax
140ac9c02: 48 8b 00                    	movq	(%rax), %rax
140ac9c05: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140ac9c09: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
140ac9c0d: 89 85 1c 02 00 00           	movl	%eax, 0x21c(%rbp)
140ac9c13: 49 8b 07                    	movq	(%r15), %rax
140ac9c16: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
140ac9c1a: 49 8b 47 10                 	movq	0x10(%r15), %rax
140ac9c1e: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
140ac9c22: 48 8b 03                    	movq	(%rbx), %rax
140ac9c25: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140ac9c29: 48 89 55 78                 	movq	%rdx, 0x78(%rbp)
140ac9c2d: 8b 53 1c                    	movl	0x1c(%rbx), %edx
140ac9c30: 89 95 24 02 00 00           	movl	%edx, 0x224(%rbp)
140ac9c36: 41 8b 56 1c                 	movl	0x1c(%r14), %edx
140ac9c3a: 89 95 8c 02 00 00           	movl	%edx, 0x28c(%rbp)
140ac9c40: 49 8b 16                    	movq	(%r14), %rdx
140ac9c43: 4d 8b 46 10                 	movq	0x10(%r14), %r8
140ac9c47: 4c 89 85 e0 00 00 00        	movq	%r8, 0xe0(%rbp)
140ac9c4e: 44 8b 47 1c                 	movl	0x1c(%rdi), %r8d
140ac9c52: 44 89 85 88 02 00 00        	movl	%r8d, 0x288(%rbp)
140ac9c59: 4c 8b 07                    	movq	(%rdi), %r8
140ac9c5c: 4c 8b 4f 10                 	movq	0x10(%rdi), %r9
140ac9c60: 4c 89 8d d0 00 00 00        	movq	%r9, 0xd0(%rbp)
140ac9c67: 44 8b 4e 1c                 	movl	0x1c(%rsi), %r9d
140ac9c6b: 44 89 8d 18 02 00 00        	movl	%r9d, 0x218(%rbp)
140ac9c72: 4c 8b 0e                    	movq	(%rsi), %r9
140ac9c75: 4c 8b 56 10                 	movq	0x10(%rsi), %r10
140ac9c79: 4c 89 55 28                 	movq	%r10, 0x28(%rbp)
140ac9c7d: 48 89 ce                    	movq	%rcx, %rsi
140ac9c80: 66 44 0f 6e 05 97 bc e5 01  	movd	0x1e5bc97(%rip), %xmm8  # 0x142925920
140ac9c89: 66 45 0f 76 c9              	pcmpeqd	%xmm9, %xmm9
140ac9c8e: f3 44 0f 10 15 ed bc e5 01  	movss	0x1e5bced(%rip), %xmm10 # 0x142925984
140ac9c97: 66 44 0f 6f 1d 50 64 01 02  	movdqa	0x2016450(%rip), %xmm11 # 0x142ae00f0
140ac9ca0: 66 44 0f 6e 25 2f bc e5 01  	movd	0x1e5bc2f(%rip), %xmm12 # 0x1429258d8
140ac9ca9: 49 8d 4d 20                 	leaq	0x20(%r13), %rcx
140ac9cad: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
140ac9cb4: 48 89 95 78 02 00 00        	movq	%rdx, 0x278(%rbp)
140ac9cbb: 48 8d 4a 20                 	leaq	0x20(%rdx), %rcx
140ac9cbf: 48 89 8d f8 00 00 00        	movq	%rcx, 0xf8(%rbp)
140ac9cc6: 4c 89 85 70 02 00 00        	movq	%r8, 0x270(%rbp)
140ac9ccd: 49 8d 48 20                 	leaq	0x20(%r8), %rcx
140ac9cd1: 48 89 8d e8 00 00 00        	movq	%rcx, 0xe8(%rbp)
140ac9cd8: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
140ac9cdf: 48 8d 40 20                 	leaq	0x20(%rax), %rax
140ac9ce3: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140ac9ce7: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
140ac9cee: 48 8d 40 04                 	leaq	0x4(%rax), %rax
140ac9cf2: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
140ac9cf9: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
140ac9d00: 48 8d 40 20                 	leaq	0x20(%rax), %rax
140ac9d04: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
140ac9d0b: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
140ac9d12: 48 8d 40 20                 	leaq	0x20(%rax), %rax
140ac9d16: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140ac9d1d: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
140ac9d24: 48 8d 40 20                 	leaq	0x20(%rax), %rax
140ac9d28: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140ac9d2c: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140ac9d33: 48 8d 40 30                 	leaq	0x30(%rax), %rax
140ac9d37: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140ac9d3b: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
140ac9d42: 48 8d 88 90 00 00 00        	leaq	0x90(%rax), %rcx
140ac9d49: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
140ac9d4d: 48 83 c0 20                 	addq	$0x20, %rax
140ac9d51: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140ac9d55: 4c 89 8d c0 00 00 00        	movq	%r9, 0xc0(%rbp)
140ac9d5c: 49 8d 41 20                 	leaq	0x20(%r9), %rax
140ac9d60: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140ac9d64: 4c 8d b5 00 01 00 00        	leaq	0x100(%rbp), %r14
140ac9d6b: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
140ac9d76: eb 4d                       	jmp	0x140ac9dc5 <.text+0xac8dc5>
140ac9d78: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140ac9d80: 4c 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %r8
140ac9d87: 49 8b 40 28                 	movq	0x28(%r8), %rax
140ac9d8b: 48 89 f1                    	movq	%rsi, %rcx
140ac9d8e: 48 c1 e1 05                 	shlq	$0x5, %rcx
140ac9d92: 48 89 3c 08                 	movq	%rdi, (%rax,%rcx)
140ac9d96: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
140ac9d9d: 48 89 54 08 18              	movq	%rdx, 0x18(%rax,%rcx)
140ac9da2: f3 0f 6f 85 00 01 00 00     	movdqu	0x100(%rbp), %xmm0
140ac9daa: f3 0f 7f 44 08 08           	movdqu	%xmm0, 0x8(%rax,%rcx)
140ac9db0: 48 ff c6                    	incq	%rsi
140ac9db3: 49 89 70 30                 	movq	%rsi, 0x30(%r8)
140ac9db7: 49 ff 40 40                 	incq	0x40(%r8)
140ac9dbb: 4c 89 f6                    	movq	%r14, %rsi
140ac9dbe: 4c 8d b5 00 01 00 00        	leaq	0x100(%rbp), %r14
140ac9dc5: 48 3b 75 e0                 	cmpq	-0x20(%rbp), %rsi
140ac9dc9: 0f 84 77 fd ff ff           	je	0x140ac9b46 <.text+0xac8b46>
140ac9dcf: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
140ac9dd6: 48 89 85 08 03 00 00        	movq	%rax, 0x308(%rbp)
140ac9ddd: f2 0f 10 0e                 	movsd	(%rsi), %xmm1
140ac9de1: f2 0f 11 8d 28 02 00 00     	movsd	%xmm1, 0x228(%rbp)
140ac9de9: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
140ac9ded: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
140ac9df1: e8 4a f8 f1 ff              	callq	0x1409e9640 <.text+0x9e8640>
140ac9df6: 0f 28 f0                    	movaps	%xmm0, %xmm6
140ac9df9: 48 8d 8d fc 01 00 00        	leaq	0x1fc(%rbp), %rcx
140ac9e00: 48 8d 95 28 02 00 00        	leaq	0x228(%rbp), %rdx
140ac9e07: 0f 28 d0                    	movaps	%xmm0, %xmm2
140ac9e0a: e8 d1 2f 1d 00              	callq	0x140c9cde0 <.text+0xc9bde0>
140ac9e0f: 48 8d 85 20 03 00 00        	leaq	0x320(%rbp), %rax
140ac9e16: 48 89 85 b0 02 00 00        	movq	%rax, 0x2b0(%rbp)
140ac9e1d: 48 8d 05 5c 7d 00 00        	leaq	0x7d5c(%rip), %rax      # 0x140ad1b80 <.text+0xad0b80>
140ac9e24: 48 89 85 b8 02 00 00        	movq	%rax, 0x2b8(%rbp)
140ac9e2b: 48 8d 85 08 03 00 00        	leaq	0x308(%rbp), %rax
140ac9e32: 48 89 85 c0 02 00 00        	movq	%rax, 0x2c0(%rbp)
140ac9e39: 48 8d 05 80 e6 5f ff        	leaq	-0xa01980(%rip), %rax   # 0x1400c84c0 <.text+0xc74c0>
140ac9e40: 48 89 85 c8 02 00 00        	movq	%rax, 0x2c8(%rbp)
140ac9e47: 48 8d 05 a2 6a 01 02        	leaq	0x2016aa2(%rip), %rax   # 0x142ae08f0
140ac9e4e: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140ac9e55: 48 c7 85 08 01 00 00 02 00 00 00    	movq	$0x2, 0x108(%rbp)
140ac9e60: 48 8d 85 b0 02 00 00        	leaq	0x2b0(%rbp), %rax
140ac9e67: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140ac9e6e: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
140ac9e79: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
140ac9e84: 48 8d bd 90 00 00 00        	leaq	0x90(%rbp), %rdi
140ac9e8b: 48 89 f9                    	movq	%rdi, %rcx
140ac9e8e: 4c 89 f2                    	movq	%r14, %rdx
140ac9e91: e8 fa 00 2b 00              	callq	0x140d79f90 <.text+0xd78f90>
140ac9e96: 8b 85 04 02 00 00           	movl	0x204(%rbp), %eax
140ac9e9c: 48 8d 8d 08 01 00 00        	leaq	0x108(%rbp), %rcx
140ac9ea3: 89 81 90 00 00 00           	movl	%eax, 0x90(%rcx)
140ac9ea9: 48 8b 85 fc 01 00 00        	movq	0x1fc(%rbp), %rax
140ac9eb0: 48 89 81 88 00 00 00        	movq	%rax, 0x88(%rcx)
140ac9eb7: 48 c7 85 00 01 00 00 00 00 00 00    	movq	$0x0, 0x100(%rbp)
140ac9ec2: 48 c7 85 08 01 00 00 08 00 00 00    	movq	$0x8, 0x108(%rbp)
140ac9ecd: 48 c7 85 10 01 00 00 00 00 00 00    	movq	$0x0, 0x110(%rbp)
140ac9ed8: c7 85 18 01 00 00 01 00 00 00       	movl	$0x1, 0x118(%rbp)
140ac9ee2: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
140ac9eed: c7 85 30 01 00 00 02 00 00 00       	movl	$0x2, 0x130(%rbp)
140ac9ef7: c7 85 48 01 00 00 03 00 00 00       	movl	$0x3, 0x148(%rbp)
140ac9f01: c7 85 60 01 00 00 03 00 00 00       	movl	$0x3, 0x160(%rbp)
140ac9f0b: c6 85 78 01 00 00 00        	movb	$0x0, 0x178(%rbp)
140ac9f12: c6 85 88 01 00 00 00        	movb	$0x0, 0x188(%rbp)
140ac9f19: c7 85 8c 01 00 00 00 00 40 3f       	movl	$0x3f400000, 0x18c(%rbp) # imm = 0x3F400000
140ac9f23: 66 c7 85 a8 01 00 00 00 01  	movw	$0x100, 0x1a8(%rbp)     # imm = 0x100
140ac9f2c: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
140ac9f30: 48 89 fa                    	movq	%rdi, %rdx
140ac9f33: 4d 89 f0                    	movq	%r14, %r8
140ac9f36: e8 b5 92 e5 ff              	callq	0x1409231f0 <.text+0x9221f0>
140ac9f3b: 49 89 c4                    	movq	%rax, %r12
140ac9f3e: 48 89 d3                    	movq	%rdx, %rbx
140ac9f41: 80 78 18 00                 	cmpb	$0x0, 0x18(%rax)
140ac9f45: 66 41 0f 6f c0              	movdqa	%xmm8, %xmm0
140ac9f4a: 74 24                       	je	0x140ac9f70 <.text+0xac8f70>
140ac9f4c: 66 41 0f 6f c0              	movdqa	%xmm8, %xmm0
140ac9f51: 49 39 5c 24 38              	cmpq	%rbx, 0x38(%r12)
140ac9f56: 75 18                       	jne	0x140ac9f70 <.text+0xac8f70>
140ac9f58: 41 83 7c 24 28 02           	cmpl	$0x2, 0x28(%r12)
140ac9f5e: 66 41 0f 6f c0              	movdqa	%xmm8, %xmm0
140ac9f63: 74 0b                       	je	0x140ac9f70 <.text+0xac8f70>
140ac9f65: 66 41 0f 6f c4              	movdqa	%xmm12, %xmm0
140ac9f6a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140ac9f70: 66 0f 7e 85 84 02 00 00     	movd	%xmm0, 0x284(%rbp)
140ac9f78: 48 8d 85 fc 01 00 00        	leaq	0x1fc(%rbp), %rax
140ac9f7f: 48 89 85 b0 02 00 00        	movq	%rax, 0x2b0(%rbp)
140ac9f86: 48 8d 05 73 1a 28 00        	leaq	0x281a73(%rip), %rax    # 0x140d4ba00 <.text+0xd4aa00>
140ac9f8d: 48 89 85 b8 02 00 00        	movq	%rax, 0x2b8(%rbp)
140ac9f94: 48 8d 85 84 02 00 00        	leaq	0x284(%rbp), %rax
140ac9f9b: 48 89 85 c0 02 00 00        	movq	%rax, 0x2c0(%rbp)
140ac9fa2: 48 8d 05 e7 28 2c 00        	leaq	0x2c28e7(%rip), %rax    # 0x140d8c890 <.text+0xd8b890>
140ac9fa9: 48 89 85 c8 02 00 00        	movq	%rax, 0x2c8(%rbp)
140ac9fb0: 48 8d 05 69 69 01 02        	leaq	0x2016969(%rip), %rax   # 0x142ae0920
140ac9fb7: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140ac9fbe: 48 c7 85 08 01 00 00 02 00 00 00    	movq	$0x2, 0x108(%rbp)
140ac9fc9: 48 8d 85 b0 02 00 00        	leaq	0x2b0(%rbp), %rax
140ac9fd0: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140ac9fd7: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
140ac9fe2: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
140ac9fed: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140ac9ff1: 4c 89 f2                    	movq	%r14, %rdx
140ac9ff4: e8 97 ff 2a 00              	callq	0x140d79f90 <.text+0xd78f90>
140ac9ff9: 48 8b 7d c0                 	movq	-0x40(%rbp), %rdi
140ac9ffd: 48 8b 45 c8                 	movq	-0x38(%rbp), %rax
140aca001: 48 89 85 38 03 00 00        	movq	%rax, 0x338(%rbp)
140aca008: 4c 8b 7d d0                 	movq	-0x30(%rbp), %r15
140aca00c: 8b 85 04 02 00 00           	movl	0x204(%rbp), %eax
140aca012: 89 85 08 01 00 00           	movl	%eax, 0x108(%rbp)
140aca018: 48 8b 85 fc 01 00 00        	movq	0x1fc(%rbp), %rax
140aca01f: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140aca026: f3 0f 10 bd 84 02 00 00     	movss	0x284(%rbp), %xmm7
140aca02e: 0f b6 05 d4 ab b6 02        	movzbl	0x2b6abd4(%rip), %eax   # 0x143634c09
140aca035: b9 20 00 00 00              	movl	$0x20, %ecx
140aca03a: ba 08 00 00 00              	movl	$0x8, %edx
140aca03f: e8 cc 9b b4 ff              	callq	0x140613c10 <.text+0x612c10>
140aca044: 48 85 c0                    	testq	%rax, %rax
140aca047: 48 89 bd 30 03 00 00        	movq	%rdi, 0x330(%rbp)
140aca04e: 0f 84 7f 17 00 00           	je	0x140acb7d3 <.text+0xaca7d3>
140aca054: 48 89 c1                    	movq	%rax, %rcx
140aca057: 48 83 c6 08                 	addq	$0x8, %rsi
140aca05b: 48 ff 85 a8 00 00 00        	incq	0xa8(%rbp)
140aca062: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140aca069: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140aca071: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
140aca078: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
140aca07c: 8b 85 08 01 00 00           	movl	0x108(%rbp), %eax
140aca082: 89 41 18                    	movl	%eax, 0x18(%rcx)
140aca085: f3 0f 11 79 1c              	movss	%xmm7, 0x1c(%rcx)
140aca08a: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
140aca091: 48 8d 05 a8 68 01 02        	leaq	0x20168a8(%rip), %rax   # 0x142ae0940
140aca098: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
140aca09f: 4d 85 ff                    	testq	%r15, %r15
140aca0a2: 48 89 8d 90 02 00 00        	movq	%rcx, 0x290(%rbp)
140aca0a9: 0f 88 0a 17 00 00           	js	0x140acb7b9 <.text+0xaca7b9>
140aca0af: 74 2f                       	je	0x140aca0e0 <.text+0xac90e0>
140aca0b1: 0f b6 05 51 ab b6 02        	movzbl	0x2b6ab51(%rip), %eax   # 0x143634c09
140aca0b8: 41 be 01 00 00 00           	movl	$0x1, %r14d
140aca0be: ba 01 00 00 00              	movl	$0x1, %edx
140aca0c3: 4c 89 f9                    	movq	%r15, %rcx
140aca0c6: e8 45 9b b4 ff              	callq	0x140613c10 <.text+0x612c10>
140aca0cb: 48 85 c0                    	testq	%rax, %rax
140aca0ce: 0f 84 e8 16 00 00           	je	0x140acb7bc <.text+0xaca7bc>
140aca0d4: 48 89 c7                    	movq	%rax, %rdi
140aca0d7: 4c 8d b5 00 01 00 00        	leaq	0x100(%rbp), %r14
140aca0de: eb 05                       	jmp	0x140aca0e5 <.text+0xac90e5>
140aca0e0: bf 01 00 00 00              	movl	$0x1, %edi
140aca0e5: 48 89 f9                    	movq	%rdi, %rcx
140aca0e8: 48 8b 95 38 03 00 00        	movq	0x338(%rbp), %rdx
140aca0ef: 4d 89 f8                    	movq	%r15, %r8
140aca0f2: e8 09 9b e5 01              	callq	0x142923c00 <.text+0x2922c00>
140aca0f7: 4c 89 bd 00 01 00 00        	movq	%r15, 0x100(%rbp)
140aca0fe: 48 89 bd 08 01 00 00        	movq	%rdi, 0x108(%rbp)
140aca105: 4c 89 bd 10 01 00 00        	movq	%r15, 0x110(%rbp)
140aca10c: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
140aca113: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
140aca11a: 48 8d 05 1f 68 01 02        	leaq	0x201681f(%rip), %rax   # 0x142ae0940
140aca121: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
140aca128: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140aca12c: 4c 89 f2                    	movq	%r14, %rdx
140aca12f: e8 4c 27 9a 00              	callq	0x14146c880 <.text+0x146b880>
140aca134: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
140aca13b: 48 85 d2                    	testq	%rdx, %rdx
140aca13e: 74 12                       	je	0x140aca152 <.text+0xac9152>
140aca140: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140aca146: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
140aca14d: e8 ce 9a b4 ff              	callq	0x140613c20 <.text+0x612c20>
140aca152: b0 02                       	movb	$0x2, %al
140aca154: 41 80 7c 24 18 02           	cmpb	$0x2, 0x18(%r12)
140aca15a: 72 0d                       	jb	0x140aca169 <.text+0xac9169>
140aca15c: 49 39 5c 24 38              	cmpq	%rbx, 0x38(%r12)
140aca161: 75 06                       	jne	0x140aca169 <.text+0xac9169>
140aca163: 41 0f b6 44 24 19           	movzbl	0x19(%r12), %eax
140aca169: 04 fe                       	addb	$-0x2, %al
140aca16b: 0f b6 c0                    	movzbl	%al, %eax
140aca16e: 3c 03                       	cmpb	$0x3, %al
140aca170: b9 03 00 00 00              	movl	$0x3, %ecx
140aca175: 0f 43 c1                    	cmovael	%ecx, %eax
140aca178: 0f b6 c0                    	movzbl	%al, %eax
140aca17b: 48 8d 0d 36 6b 01 02        	leaq	0x2016b36(%rip), %rcx   # 0x142ae0cb8
140aca182: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
140aca186: 48 01 c8                    	addq	%rcx, %rax
140aca189: ff e0                       	jmpq	*%rax
140aca18b: 48 89 b5 98 02 00 00        	movq	%rsi, 0x298(%rbp)
140aca192: 48 8b 5d 08                 	movq	0x8(%rbp), %rbx
140aca196: 44 8b b5 08 02 00 00        	movl	0x208(%rbp), %r14d
140aca19d: 44 89 33                    	movl	%r14d, (%rbx)
140aca1a0: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
140aca1a7: 48 c7 85 00 01 00 00 00 00 00 00    	movq	$0x0, 0x100(%rbp)
140aca1b2: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
140aca1b9: 48 c7 85 10 01 00 00 01 00 00 00    	movq	$0x1, 0x110(%rbp)
140aca1c4: 48 8d 05 a5 67 01 02        	leaq	0x20167a5(%rip), %rax   # 0x142ae0970
140aca1cb: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140aca1d0: 48 8b 85 78 04 00 00        	movq	0x478(%rbp), %rax
140aca1d7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140aca1dc: 48 c7 44 24 30 12 00 00 00  	movq	$0x12, 0x30(%rsp)
140aca1e5: 41 b8 02 00 00 00           	movl	$0x2, %r8d
140aca1eb: 48 8b b5 b8 00 00 00        	movq	0xb8(%rbp), %rsi
140aca1f2: 48 89 f1                    	movq	%rsi, %rcx
140aca1f5: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
140aca1fc: 48 8b bd 30 02 00 00        	movq	0x230(%rbp), %rdi
140aca203: 49 89 f9                    	movq	%rdi, %r9
140aca206: e8 e5 d5 91 00              	callq	0x1413e77f0 <.text+0x13e67f0>
140aca20b: 44 89 33                    	movl	%r14d, (%rbx)
140aca20e: 48 8b 95 20 03 00 00        	movq	0x320(%rbp), %rdx
140aca215: 48 89 f1                    	movq	%rsi, %rcx
140aca218: 49 89 f8                    	movq	%rdi, %r8
140aca21b: e8 e0 19 06 00              	callq	0x140b2bc00 <.text+0xb2ac00>
140aca220: 44 89 33                    	movl	%r14d, (%rbx)
140aca223: 48 8d 9d 00 01 00 00        	leaq	0x100(%rbp), %rbx
140aca22a: 48 8b 87 90 00 00 00        	movq	0x90(%rdi), %rax
140aca231: 48 85 c0                    	testq	%rax, %rax
140aca234: 48 0f 45 45 38              	cmovneq	0x38(%rbp), %rax
140aca239: 48 c7 85 00 01 00 00 01 00 00 00    	movq	$0x1, 0x100(%rbp)
140aca244: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
140aca24b: 48 c7 85 10 01 00 00 00 00 00 00    	movq	$0x0, 0x110(%rbp)
140aca256: 48 c7 85 38 01 00 00 00 00 00 00    	movq	$0x0, 0x138(%rbp)
140aca261: 48 89 d9                    	movq	%rbx, %rcx
140aca264: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
140aca268: e8 13 90 7c 00              	callq	0x141293280 <.text+0x1292280>
140aca26d: 48 c7 85 b0 02 00 00 00 00 00 00    	movq	$0x0, 0x2b0(%rbp)
140aca278: 48 c7 85 b8 02 00 00 04 00 00 00    	movq	$0x4, 0x2b8(%rbp)
140aca283: 48 c7 85 c0 02 00 00 00 00 00 00    	movq	$0x0, 0x2c0(%rbp)
140aca28e: 48 8b b5 20 03 00 00        	movq	0x320(%rbp), %rsi
140aca295: 48 89 b5 00 01 00 00        	movq	%rsi, 0x100(%rbp)
140aca29c: 48 83 7f 18 00              	cmpq	$0x0, 0x18(%rdi)
140aca2a1: 0f 84 d9 03 00 00           	je	0x140aca680 <.text+0xac9680>
140aca2a7: c6 85 46 03 00 00 01        	movb	$0x1, 0x346(%rbp)
140aca2ae: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
140aca2b2: 48 89 da                    	movq	%rbx, %rdx
140aca2b5: e8 b6 6a ae ff              	callq	0x1405b0d70 <.text+0x5afd70>
140aca2ba: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
140aca2c1: 48 8b 0a                    	movq	(%rdx), %rcx
140aca2c4: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
140aca2c8: 49 89 d0                    	movq	%rdx, %r8
140aca2cb: 49 21 c0                    	andq	%rax, %r8
140aca2ce: 48 c1 e8 39                 	shrq	$0x39, %rax
140aca2d2: 66 0f 6e c0                 	movd	%eax, %xmm0
140aca2d6: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140aca2da: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140aca2df: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140aca2e4: 48 8d 81 68 ff ff ff        	leaq	-0x98(%rcx), %rax
140aca2eb: 45 31 c9                    	xorl	%r9d, %r9d
140aca2ee: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140aca2f4: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140aca2f8: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140aca2fc: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140aca301: 45 85 d2                    	testl	%r10d, %r10d
140aca304: 4c 8b ad 10 03 00 00        	movq	0x310(%rbp), %r13
140aca30b: 74 2c                       	je	0x140aca339 <.text+0xac9339>
140aca30d: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140aca312: 4d 01 c3                    	addq	%r8, %r11
140aca315: 49 21 d3                    	andq	%rdx, %r11
140aca318: 49 f7 db                    	negq	%r11
140aca31b: 4d 69 db 98 00 00 00        	imulq	$0x98, %r11, %r11
140aca322: 4a 39 34 18                 	cmpq	%rsi, (%rax,%r11)
140aca326: 0f 84 0d 02 00 00           	je	0x140aca539 <.text+0xac9539>
140aca32c: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140aca330: 66 45 21 d3                 	andw	%r10w, %r11w
140aca334: 45 89 da                    	movl	%r11d, %r10d
140aca337: 75 d4                       	jne	0x140aca30d <.text+0xac930d>
140aca339: 66 41 0f 74 c9              	pcmpeqb	%xmm9, %xmm1
140aca33e: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140aca343: 45 85 d2                    	testl	%r10d, %r10d
140aca346: 0f 85 34 03 00 00           	jne	0x140aca680 <.text+0xac9680>
140aca34c: 4d 01 c8                    	addq	%r9, %r8
140aca34f: 49 83 c0 10                 	addq	$0x10, %r8
140aca353: 49 83 c1 10                 	addq	$0x10, %r9
140aca357: 49 21 d0                    	andq	%rdx, %r8
140aca35a: eb 92                       	jmp	0x140aca2ee <.text+0xac92ee>
140aca35c: 0f 1f 40 00                 	nopl	(%rax)
140aca360: 49 8b 5d 10                 	movq	0x10(%r13), %rbx
140aca364: 48 89 d8                    	movq	%rbx, %rax
140aca367: b9 0c 00 00 00              	movl	$0xc, %ecx
140aca36c: 48 f7 e1                    	mulq	%rcx
140aca36f: 49 89 c6                    	movq	%rax, %r14
140aca372: 0f 80 28 15 00 00           	jo	0x140acb8a0 <.text+0xaca8a0>
140aca378: 41 bf 00 00 00 00           	movl	$0x0, %r15d
140aca37e: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140aca388: 49 39 c6                    	cmpq	%rax, %r14
140aca38b: 0f 87 12 15 00 00           	ja	0x140acb8a3 <.text+0xaca8a3>
140aca391: 49 8b 7d 08                 	movq	0x8(%r13), %rdi
140aca395: 4d 85 f6                    	testq	%r14, %r14
140aca398: 48 89 b5 98 02 00 00        	movq	%rsi, 0x298(%rbp)
140aca39f: 0f 84 b2 03 00 00           	je	0x140aca757 <.text+0xac9757>
140aca3a5: 0f b6 05 5d a8 b6 02        	movzbl	0x2b6a85d(%rip), %eax   # 0x143634c09
140aca3ac: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140aca3b2: ba 04 00 00 00              	movl	$0x4, %edx
140aca3b7: 4c 89 f1                    	movq	%r14, %rcx
140aca3ba: e8 51 98 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140aca3bf: 48 85 c0                    	testq	%rax, %rax
140aca3c2: 0f 84 db 14 00 00           	je	0x140acb8a3 <.text+0xaca8a3>
140aca3c8: 48 89 9d 38 03 00 00        	movq	%rbx, 0x338(%rbp)
140aca3cf: e9 93 03 00 00              	jmp	0x140aca767 <.text+0xac9767>
140aca3d4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140aca3e0: 48 89 b5 98 02 00 00        	movq	%rsi, 0x298(%rbp)
140aca3e7: 48 8b bd 20 03 00 00        	movq	0x320(%rbp), %rdi
140aca3ee: 48 89 bd 00 01 00 00        	movq	%rdi, 0x100(%rbp)
140aca3f5: 48 8b 9d d8 00 00 00        	movq	0xd8(%rbp), %rbx
140aca3fc: 48 83 7b 18 00              	cmpq	$0x0, 0x18(%rbx)
140aca401: 0f 84 6a 13 00 00           	je	0x140acb771 <.text+0xaca771>
140aca407: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140aca40b: 4c 89 f2                    	movq	%r14, %rdx
140aca40e: e8 5d 69 ae ff              	callq	0x1405b0d70 <.text+0x5afd70>
140aca413: 48 8b 33                    	movq	(%rbx), %rsi
140aca416: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140aca41a: 48 89 ca                    	movq	%rcx, %rdx
140aca41d: 48 21 c2                    	andq	%rax, %rdx
140aca420: 48 c1 e8 39                 	shrq	$0x39, %rax
140aca424: 66 0f 6e c0                 	movd	%eax, %xmm0
140aca428: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140aca42c: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140aca431: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140aca436: 48 8d 46 88                 	leaq	-0x78(%rsi), %rax
140aca43a: 45 31 c0                    	xorl	%r8d, %r8d
140aca43d: f3 0f 6f 0c 16              	movdqu	(%rsi,%rdx), %xmm1
140aca442: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140aca446: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140aca44a: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140aca44f: 45 85 c9                    	testl	%r9d, %r9d
140aca452: 74 25                       	je	0x140aca479 <.text+0xac9479>
140aca454: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
140aca459: 49 01 d2                    	addq	%rdx, %r10
140aca45c: 49 21 ca                    	andq	%rcx, %r10
140aca45f: 49 f7 da                    	negq	%r10
140aca462: 4d 6b d2 78                 	imulq	$0x78, %r10, %r10
140aca466: 4a 39 3c 10                 	cmpq	%rdi, (%rax,%r10)
140aca46a: 74 30                       	je	0x140aca49c <.text+0xac949c>
140aca46c: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
140aca470: 66 45 21 ca                 	andw	%r9w, %r10w
140aca474: 45 89 d1                    	movl	%r10d, %r9d
140aca477: 75 db                       	jne	0x140aca454 <.text+0xac9454>
140aca479: 66 41 0f 74 c9              	pcmpeqb	%xmm9, %xmm1
140aca47e: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
140aca483: 45 85 c9                    	testl	%r9d, %r9d
140aca486: 0f 85 e5 12 00 00           	jne	0x140acb771 <.text+0xaca771>
140aca48c: 4c 01 c2                    	addq	%r8, %rdx
140aca48f: 48 83 c2 10                 	addq	$0x10, %rdx
140aca493: 49 83 c0 10                 	addq	$0x10, %r8
140aca497: 48 21 ca                    	andq	%rcx, %rdx
140aca49a: eb a1                       	jmp	0x140aca43d <.text+0xac943d>
140aca49c: 66 45 0f ef ed              	pxor	%xmm13, %xmm13
140aca4a1: 48 8b 85 68 02 00 00        	movq	0x268(%rbp), %rax
140aca4a8: f6 00 01                    	testb	$0x1, (%rax)
140aca4ab: 74 17                       	je	0x140aca4c4 <.text+0xac94c4>
140aca4ad: 48 8b 85 68 02 00 00        	movq	0x268(%rbp), %rax
140aca4b4: 66 44 0f 6e 68 04           	movd	0x4(%rax), %xmm13
140aca4ba: 66 0f 6e 40 0c              	movd	0xc(%rax), %xmm0
140aca4bf: 66 44 0f 62 e8              	punpckldq	%xmm0, %xmm13   # xmm13 = xmm13[0],xmm0[0],xmm13[1],xmm0[1]
140aca4c4: 4c 01 d6                    	addq	%r10, %rsi
140aca4c7: 4c 8b 66 a0                 	movq	-0x60(%rsi), %r12
140aca4cb: 4a 8d 1c e5 00 00 00 00     	leaq	(,%r12,8), %rbx
140aca4d3: 4c 89 e0                    	movq	%r12, %rax
140aca4d6: 48 c1 e8 3d                 	shrq	$0x3d, %rax
140aca4da: 0f 85 76 14 00 00           	jne	0x140acb956 <.text+0xaca956>
140aca4e0: bf 00 00 00 00              	movl	$0x0, %edi
140aca4e5: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140aca4ef: 48 39 c3                    	cmpq	%rax, %rbx
140aca4f2: 0f 87 60 14 00 00           	ja	0x140acb958 <.text+0xaca958>
140aca4f8: f2 41 0f 10 7d 50           	movsd	0x50(%r13), %xmm7
140aca4fe: 4c 8b 76 98                 	movq	-0x68(%rsi), %r14
140aca502: 48 85 db                    	testq	%rbx, %rbx
140aca505: 0f 84 66 05 00 00           	je	0x140acaa71 <.text+0xac9a71>
140aca50b: 0f b6 05 f7 a6 b6 02        	movzbl	0x2b6a6f7(%rip), %eax   # 0x143634c09
140aca512: bf 04 00 00 00              	movl	$0x4, %edi
140aca517: ba 04 00 00 00              	movl	$0x4, %edx
140aca51c: 48 89 d9                    	movq	%rbx, %rcx
140aca51f: e8 ec 96 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140aca524: 48 85 c0                    	testq	%rax, %rax
140aca527: 0f 84 2b 14 00 00           	je	0x140acb958 <.text+0xaca958>
140aca52d: 4c 89 a5 38 03 00 00        	movq	%r12, 0x338(%rbp)
140aca534: e9 48 05 00 00              	jmp	0x140acaa81 <.text+0xac9a81>
140aca539: 4a 8b 74 19 a0              	movq	-0x60(%rcx,%r11), %rsi
140aca53e: 4a 8b 5c 19 b8              	movq	-0x48(%rcx,%r11), %rbx
140aca543: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
140aca547: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140aca54b: f7 d0                       	notl	%eax
140aca54d: 4c 8d 76 10                 	leaq	0x10(%rsi), %r14
140aca551: eb 41                       	jmp	0x140aca594 <.text+0xac9594>
140aca553: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140aca560: 48 8b 85 b8 02 00 00        	movq	0x2b8(%rbp), %rax
140aca567: 4a 8d 0c 6d 00 00 00 00     	leaq	(,%r13,2), %rcx
140aca56f: 4c 01 e9                    	addq	%r13, %rcx
140aca572: 44 89 24 88                 	movl	%r12d, (%rax,%rcx,4)
140aca576: 89 7c 88 04                 	movl	%edi, 0x4(%rax,%rcx,4)
140aca57a: 66 0f 7e 7c 88 08           	movd	%xmm7, 0x8(%rax,%rcx,4)
140aca580: 49 ff c5                    	incq	%r13
140aca583: 4c 89 ad c0 02 00 00        	movq	%r13, 0x2c0(%rbp)
140aca58a: 44 89 f8                    	movl	%r15d, %eax
140aca58d: 4c 8b ad 10 03 00 00        	movq	0x310(%rbp), %r13
140aca594: 48 85 db                    	testq	%rbx, %rbx
140aca597: 0f 84 e3 00 00 00           	je	0x140aca680 <.text+0xac9680>
140aca59d: 66 85 c0                    	testw	%ax, %ax
140aca5a0: 74 0e                       	je	0x140aca5b0 <.text+0xac95b0>
140aca5a2: 44 8d 78 ff                 	leal	-0x1(%rax), %r15d
140aca5a6: 41 21 c7                    	andl	%eax, %r15d
140aca5a9: eb 2f                       	jmp	0x140aca5da <.text+0xac95da>
140aca5ab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140aca5b0: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
140aca5b5: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140aca5b9: 48 81 c6 00 fb ff ff        	addq	$-0x500, %rsi           # imm = 0xFB00
140aca5c0: 49 83 c6 10                 	addq	$0x10, %r14
140aca5c4: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140aca5c9: 74 e5                       	je	0x140aca5b0 <.text+0xac95b0>
140aca5cb: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140aca5d0: 29 c1                       	subl	%eax, %ecx
140aca5d2: f7 d0                       	notl	%eax
140aca5d4: 41 89 c7                    	movl	%eax, %r15d
140aca5d7: 41 21 cf                    	andl	%ecx, %r15d
140aca5da: 48 ff cb                    	decq	%rbx
140aca5dd: f3 0f bc c0                 	tzcntl	%eax, %eax
140aca5e1: 48 f7 d8                    	negq	%rax
140aca5e4: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
140aca5e8: 48 c1 e0 04                 	shlq	$0x4, %rax
140aca5ec: 0f b6 4c 06 da              	movzbl	-0x26(%rsi,%rax), %ecx
140aca5f1: 8d 51 fe                    	leal	-0x2(%rcx), %edx
140aca5f4: 48 ff c9                    	decq	%rcx
140aca5f7: 80 fa 03                    	cmpb	$0x3, %dl
140aca5fa: ba 00 00 00 00              	movl	$0x0, %edx
140aca5ff: 48 0f 43 ca                 	cmovaeq	%rdx, %rcx
140aca603: 48 85 c9                    	testq	%rcx, %rcx
140aca606: 74 18                       	je	0x140aca620 <.text+0xac9620>
140aca608: 44 89 f8                    	movl	%r15d, %eax
140aca60b: 48 83 f9 01                 	cmpq	$0x1, %rcx
140aca60f: 74 83                       	je	0x140aca594 <.text+0xac9594>
140aca611: e9 7f 11 00 00              	jmp	0x140acb795 <.text+0xaca795>
140aca616: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140aca620: 48 01 f0                    	addq	%rsi, %rax
140aca623: f3 0f 10 48 cc              	movss	-0x34(%rax), %xmm1
140aca628: f3 0f 10 50 d4              	movss	-0x2c(%rax), %xmm2
140aca62d: 44 8b 60 b0                 	movl	-0x50(%rax), %r12d
140aca631: c6 85 46 03 00 00 01        	movb	$0x1, 0x346(%rbp)
140aca638: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
140aca63c: e8 1f fa 99 00              	callq	0x14146a060 <.text+0x1469060>
140aca641: 89 c7                       	movl	%eax, %edi
140aca643: 66 0f 6f f8                 	movdqa	%xmm0, %xmm7
140aca647: 4c 8b ad c0 02 00 00        	movq	0x2c0(%rbp), %r13
140aca64e: 4c 3b ad b0 02 00 00        	cmpq	0x2b0(%rbp), %r13
140aca655: 0f 85 05 ff ff ff           	jne	0x140aca560 <.text+0xac9560>
140aca65b: c6 85 46 03 00 00 01        	movb	$0x1, 0x346(%rbp)
140aca662: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140aca669: 48 8d 15 18 63 01 02        	leaq	0x2016318(%rip), %rdx   # 0x142ae0988
140aca670: e8 7b 86 a9 ff              	callq	0x140562cf0 <.text+0x561cf0>
140aca675: e9 e6 fe ff ff              	jmp	0x140aca560 <.text+0xac9560>
140aca67a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140aca680: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
140aca687: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140aca68e: 0f 10 85 b0 02 00 00        	movups	0x2b0(%rbp), %xmm0
140aca695: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
140aca69c: 49 8b 45 00                 	movq	(%r13), %rax
140aca6a0: 48 85 c0                    	testq	%rax, %rax
140aca6a3: 74 17                       	je	0x140aca6bc <.text+0xac96bc>
140aca6a5: 49 8b 4d 08                 	movq	0x8(%r13), %rcx
140aca6a9: 48 c1 e0 02                 	shlq	$0x2, %rax
140aca6ad: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140aca6b1: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140aca6b7: e8 64 95 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140aca6bc: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
140aca6c3: 49 89 45 10                 	movq	%rax, 0x10(%r13)
140aca6c7: 66 0f 6f 85 00 01 00 00     	movdqa	0x100(%rbp), %xmm0
140aca6cf: f3 41 0f 7f 45 00           	movdqu	%xmm0, (%r13)
140aca6d5: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140aca6dc: 48 8b b0 70 fd ff ff        	movq	-0x290(%rax), %rsi
140aca6e3: 48 8d 1c f5 00 00 00 00     	leaq	(,%rsi,8), %rbx
140aca6eb: 48 89 f0                    	movq	%rsi, %rax
140aca6ee: 48 c1 e8 3d                 	shrq	$0x3d, %rax
140aca6f2: 0f 85 d7 11 00 00           	jne	0x140acb8cf <.text+0xaca8cf>
140aca6f8: 41 be 00 00 00 00           	movl	$0x0, %r14d
140aca6fe: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140aca708: 48 39 c3                    	cmpq	%rax, %rbx
140aca70b: 0f 87 c1 11 00 00           	ja	0x140acb8d2 <.text+0xaca8d2>
140aca711: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140aca718: 48 8b b8 68 fd ff ff        	movq	-0x298(%rax), %rdi
140aca71f: 48 85 db                    	testq	%rbx, %rbx
140aca722: 0f 84 c7 01 00 00           	je	0x140aca8ef <.text+0xac98ef>
140aca728: 0f b6 05 da a4 b6 02        	movzbl	0x2b6a4da(%rip), %eax   # 0x143634c09
140aca72f: 41 be 04 00 00 00           	movl	$0x4, %r14d
140aca735: ba 04 00 00 00              	movl	$0x4, %edx
140aca73a: 48 89 d9                    	movq	%rbx, %rcx
140aca73d: e8 ce 94 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140aca742: 48 85 c0                    	testq	%rax, %rax
140aca745: 0f 84 87 11 00 00           	je	0x140acb8d2 <.text+0xaca8d2>
140aca74b: 48 89 b5 38 03 00 00        	movq	%rsi, 0x338(%rbp)
140aca752: e9 a8 01 00 00              	jmp	0x140aca8ff <.text+0xac98ff>
140aca757: b8 04 00 00 00              	movl	$0x4, %eax
140aca75c: 48 c7 85 38 03 00 00 00 00 00 00    	movq	$0x0, 0x338(%rbp)
140aca767: 48 89 85 30 03 00 00        	movq	%rax, 0x330(%rbp)
140aca76e: 48 89 c1                    	movq	%rax, %rcx
140aca771: 48 89 fa                    	movq	%rdi, %rdx
140aca774: 4d 89 f0                    	movq	%r14, %r8
140aca777: e8 84 94 e5 01              	callq	0x142923c00 <.text+0x2922c00>
140aca77c: 48 8b bd 20 03 00 00        	movq	0x320(%rbp), %rdi
140aca783: 8b 85 8c 02 00 00           	movl	0x28c(%rbp), %eax
140aca789: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140aca790: 89 01                       	movl	%eax, (%rcx)
140aca792: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
140aca799: 48 8b 70 30                 	movq	0x30(%rax), %rsi
140aca79d: 4c 8b 70 40                 	movq	0x40(%rax), %r14
140aca7a1: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
140aca7a5: 75 13                       	jne	0x140aca7ba <.text+0xac97ba>
140aca7a7: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140aca7ae: 48 8d 15 93 f9 16 02        	leaq	0x216f993(%rip), %rdx   # 0x142c3a148
140aca7b5: e8 f6 82 a9 ff              	callq	0x140562ab0 <.text+0x561ab0>
140aca7ba: 48 8b 95 78 02 00 00        	movq	0x278(%rbp), %rdx
140aca7c1: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140aca7c5: 48 8d 0c b6                 	leaq	(%rsi,%rsi,4), %rcx
140aca7c9: 4c 8b 85 38 03 00 00        	movq	0x338(%rbp), %r8
140aca7d0: 4c 89 04 c8                 	movq	%r8, (%rax,%rcx,8)
140aca7d4: 4c 8b 85 30 03 00 00        	movq	0x330(%rbp), %r8
140aca7db: 4c 89 44 c8 08              	movq	%r8, 0x8(%rax,%rcx,8)
140aca7e0: 48 89 5c c8 10              	movq	%rbx, 0x10(%rax,%rcx,8)
140aca7e5: 48 89 7c c8 18              	movq	%rdi, 0x18(%rax,%rcx,8)
140aca7ea: 4c 89 74 c8 20              	movq	%r14, 0x20(%rax,%rcx,8)
140aca7ef: 48 ff c6                    	incq	%rsi
140aca7f2: 48 89 72 30                 	movq	%rsi, 0x30(%rdx)
140aca7f6: 48 ff 42 40                 	incq	0x40(%rdx)
140aca7fa: 49 8b 45 18                 	movq	0x18(%r13), %rax
140aca7fe: 48 b9 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
140aca808: 49 89 4d 18                 	movq	%rcx, 0x18(%r13)
140aca80c: 48 89 c1                    	movq	%rax, %rcx
140aca80f: 48 f7 d9                    	negq	%rcx
140aca812: 4c 8d b5 00 01 00 00        	leaq	0x100(%rbp), %r14
140aca819: 0f 80 74 05 00 00           	jo	0x140acad93 <.text+0xac9d93>
140aca81f: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
140aca826: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
140aca82a: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
140aca82e: 0f 10 51 20                 	movups	0x20(%rcx), %xmm2
140aca832: 48 8d 8d b8 02 00 00        	leaq	0x2b8(%rbp), %rcx
140aca839: 0f 11 51 20                 	movups	%xmm2, 0x20(%rcx)
140aca83d: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
140aca841: f3 0f 7f 01                 	movdqu	%xmm0, (%rcx)
140aca845: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
140aca850: 48 c7 85 98 00 00 00 08 00 00 00    	movq	$0x8, 0x98(%rbp)
140aca85b: 48 c7 85 a0 00 00 00 00 00 00 00    	movq	$0x0, 0xa0(%rbp)
140aca866: 48 89 85 b0 02 00 00        	movq	%rax, 0x2b0(%rbp)
140aca86d: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140aca874: 48 8b b0 70 fd ff ff        	movq	-0x290(%rax), %rsi
140aca87b: 48 8d 1c f5 00 00 00 00     	leaq	(,%rsi,8), %rbx
140aca883: 48 89 f0                    	movq	%rsi, %rax
140aca886: 48 c1 e8 3d                 	shrq	$0x3d, %rax
140aca88a: 0f 85 f4 10 00 00           	jne	0x140acb984 <.text+0xaca984>
140aca890: 41 be 00 00 00 00           	movl	$0x0, %r14d
140aca896: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140aca8a0: 48 39 c3                    	cmpq	%rax, %rbx
140aca8a3: 0f 87 de 10 00 00           	ja	0x140acb987 <.text+0xaca987>
140aca8a9: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140aca8b0: 48 8b b8 68 fd ff ff        	movq	-0x298(%rax), %rdi
140aca8b7: 48 85 db                    	testq	%rbx, %rbx
140aca8ba: 0f 84 ad 02 00 00           	je	0x140acab6d <.text+0xac9b6d>
140aca8c0: 0f b6 05 42 a3 b6 02        	movzbl	0x2b6a342(%rip), %eax   # 0x143634c09
140aca8c7: 41 be 04 00 00 00           	movl	$0x4, %r14d
140aca8cd: ba 04 00 00 00              	movl	$0x4, %edx
140aca8d2: 48 89 d9                    	movq	%rbx, %rcx
140aca8d5: e8 36 93 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140aca8da: 48 85 c0                    	testq	%rax, %rax
140aca8dd: 0f 84 a4 10 00 00           	je	0x140acb987 <.text+0xaca987>
140aca8e3: 48 89 b5 38 03 00 00        	movq	%rsi, 0x338(%rbp)
140aca8ea: e9 8e 02 00 00              	jmp	0x140acab7d <.text+0xac9b7d>
140aca8ef: b8 04 00 00 00              	movl	$0x4, %eax
140aca8f4: 48 c7 85 38 03 00 00 00 00 00 00    	movq	$0x0, 0x338(%rbp)
140aca8ff: 48 89 85 30 03 00 00        	movq	%rax, 0x330(%rbp)
140aca906: 48 89 c1                    	movq	%rax, %rcx
140aca909: 48 89 fa                    	movq	%rdi, %rdx
140aca90c: 49 89 d8                    	movq	%rbx, %r8
140aca90f: e8 ec 92 e5 01              	callq	0x142923c00 <.text+0x2922c00>
140aca914: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140aca91b: 4c 8b a0 88 fd ff ff        	movq	-0x278(%rax), %r12
140aca922: 4e 8d 34 a5 00 00 00 00     	leaq	(,%r12,4), %r14
140aca92a: 4c 89 e0                    	movq	%r12, %rax
140aca92d: 48 c1 e8 3e                 	shrq	$0x3e, %rax
140aca931: 0f 85 7e 0f 00 00           	jne	0x140acb8b5 <.text+0xaca8b5>
140aca937: 41 bf 00 00 00 00           	movl	$0x0, %r15d
140aca93d: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140aca947: 49 39 c6                    	cmpq	%rax, %r14
140aca94a: 0f 87 68 0f 00 00           	ja	0x140acb8b8 <.text+0xaca8b8>
140aca950: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140aca957: 48 8b b8 80 fd ff ff        	movq	-0x280(%rax), %rdi
140aca95e: 4d 85 f6                    	testq	%r14, %r14
140aca961: 74 2b                       	je	0x140aca98e <.text+0xac998e>
140aca963: 0f b6 05 9f a2 b6 02        	movzbl	0x2b6a29f(%rip), %eax   # 0x143634c09
140aca96a: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140aca970: ba 04 00 00 00              	movl	$0x4, %edx
140aca975: 4c 89 f1                    	movq	%r14, %rcx
140aca978: e8 93 92 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140aca97d: 48 85 c0                    	testq	%rax, %rax
140aca980: 0f 84 32 0f 00 00           	je	0x140acb8b8 <.text+0xaca8b8>
140aca986: 48 89 c3                    	movq	%rax, %rbx
140aca989: 4d 89 e7                    	movq	%r12, %r15
140aca98c: eb 08                       	jmp	0x140aca996 <.text+0xac9996>
140aca98e: bb 04 00 00 00              	movl	$0x4, %ebx
140aca993: 45 31 ff                    	xorl	%r15d, %r15d
140aca996: 48 89 d9                    	movq	%rbx, %rcx
140aca999: 48 89 fa                    	movq	%rdi, %rdx
140aca99c: 4d 89 f0                    	movq	%r14, %r8
140aca99f: e8 5c 92 e5 01              	callq	0x142923c00 <.text+0x2922c00>
140aca9a4: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140aca9ab: f3 0f 10 b8 90 fd ff ff     	movss	-0x270(%rax), %xmm7
140aca9b3: 49 8b 55 18                 	movq	0x18(%r13), %rdx
140aca9b7: 48 89 d0                    	movq	%rdx, %rax
140aca9ba: 48 f7 d8                    	negq	%rax
140aca9bd: 4c 8d b5 00 01 00 00        	leaq	0x100(%rbp), %r14
140aca9c4: 70 37                       	jo	0x140aca9fd <.text+0xac99fd>
140aca9c6: 73 19                       	jae	0x140aca9e1 <.text+0xac99e1>
140aca9c8: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
140aca9cf: 48 8b 08                    	movq	(%rax), %rcx
140aca9d2: 48 c1 e2 03                 	shlq	$0x3, %rdx
140aca9d6: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140aca9dc: e8 3f 92 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140aca9e1: 49 8b 55 30                 	movq	0x30(%r13), %rdx
140aca9e5: 48 85 d2                    	testq	%rdx, %rdx
140aca9e8: 74 13                       	je	0x140aca9fd <.text+0xac99fd>
140aca9ea: 49 8b 4d 38                 	movq	0x38(%r13), %rcx
140aca9ee: 48 c1 e2 02                 	shlq	$0x2, %rdx
140aca9f2: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140aca9f8: e8 23 92 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140aca9fd: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
140acaa04: 49 89 45 18                 	movq	%rax, 0x18(%r13)
140acaa08: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
140acaa0f: 49 89 45 20                 	movq	%rax, 0x20(%r13)
140acaa13: 49 89 75 28                 	movq	%rsi, 0x28(%r13)
140acaa17: 4d 89 7d 30                 	movq	%r15, 0x30(%r13)
140acaa1b: 49 89 5d 38                 	movq	%rbx, 0x38(%r13)
140acaa1f: 4d 89 65 40                 	movq	%r12, 0x40(%r13)
140acaa23: f3 41 0f 11 7d 48           	movss	%xmm7, 0x48(%r13)
140acaa29: f2 0f 10 85 28 02 00 00     	movsd	0x228(%rbp), %xmm0
140acaa31: 41 c7 45 58 01 00 00 00     	movl	$0x1, 0x58(%r13)
140acaa39: f2 41 0f 11 45 5c           	movsd	%xmm0, 0x5c(%r13)
140acaa3f: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140acaa43: 48 8b 85 68 02 00 00        	movq	0x268(%rbp), %rax
140acaa4a: f6 00 01                    	testb	$0x1, (%rax)
140acaa4d: 74 15                       	je	0x140acaa64 <.text+0xac9a64>
140acaa4f: 48 8b 85 68 02 00 00        	movq	0x268(%rbp), %rax
140acaa56: 66 0f 6e 48 04              	movd	0x4(%rax), %xmm1
140acaa5b: 66 0f 6e 50 0c              	movd	0xc(%rax), %xmm2
140acaa60: 66 0f 62 ca                 	punpckldq	%xmm2, %xmm1    # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
140acaa64: 0f 5c c8                    	subps	%xmm0, %xmm1
140acaa67: 41 0f 13 4d 50              	movlps	%xmm1, 0x50(%r13)
140acaa6c: e9 88 03 00 00              	jmp	0x140acadf9 <.text+0xac9df9>
140acaa71: b8 04 00 00 00              	movl	$0x4, %eax
140acaa76: 48 c7 85 38 03 00 00 00 00 00 00    	movq	$0x0, 0x338(%rbp)
140acaa81: 48 89 85 30 03 00 00        	movq	%rax, 0x330(%rbp)
140acaa88: 48 89 c1                    	movq	%rax, %rcx
140acaa8b: 4c 89 f2                    	movq	%r14, %rdx
140acaa8e: 49 89 d8                    	movq	%rbx, %r8
140acaa91: e8 6a 91 e5 01              	callq	0x142923c00 <.text+0x2922c00>
140acaa96: 48 8b 5e b8                 	movq	-0x48(%rsi), %rbx
140acaa9a: 4c 8d 34 9d 00 00 00 00     	leaq	(,%rbx,4), %r14
140acaaa2: 48 89 d8                    	movq	%rbx, %rax
140acaaa5: 48 c1 e8 3e                 	shrq	$0x3e, %rax
140acaaa9: 0f 85 8d 0e 00 00           	jne	0x140acb93c <.text+0xaca93c>
140acaaaf: 41 bd 00 00 00 00           	movl	$0x0, %r13d
140acaab5: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140acaabf: 49 39 c6                    	cmpq	%rax, %r14
140acaac2: 0f 87 77 0e 00 00           	ja	0x140acb93f <.text+0xaca93f>
140acaac8: 48 8b 7e b0                 	movq	-0x50(%rsi), %rdi
140acaacc: 4d 85 f6                    	testq	%r14, %r14
140acaacf: 74 28                       	je	0x140acaaf9 <.text+0xac9af9>
140acaad1: 0f b6 05 31 a1 b6 02        	movzbl	0x2b6a131(%rip), %eax   # 0x143634c09
140acaad8: 41 bd 04 00 00 00           	movl	$0x4, %r13d
140acaade: ba 04 00 00 00              	movl	$0x4, %edx
140acaae3: 4c 89 f1                    	movq	%r14, %rcx
140acaae6: e8 25 91 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acaaeb: 48 85 c0                    	testq	%rax, %rax
140acaaee: 0f 84 4b 0e 00 00           	je	0x140acb93f <.text+0xaca93f>
140acaaf4: 49 89 c7                    	movq	%rax, %r15
140acaaf7: eb 08                       	jmp	0x140acab01 <.text+0xac9b01>
140acaaf9: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140acaaff: 31 db                       	xorl	%ebx, %ebx
140acab01: 4c 89 f9                    	movq	%r15, %rcx
140acab04: 48 89 fa                    	movq	%rdi, %rdx
140acab07: 4d 89 f0                    	movq	%r14, %r8
140acab0a: e8 f1 90 e5 01              	callq	0x142923c00 <.text+0x2922c00>
140acab0f: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
140acab16: 4c 39 e1                    	cmpq	%r12, %rcx
140acab19: 4c 8b b5 30 03 00 00        	movq	0x330(%rbp), %r14
140acab20: 0f 83 c1 0c 00 00           	jae	0x140acb7e7 <.text+0xaca7e7>
140acab26: 44 0f 5c ef                 	subps	%xmm7, %xmm13
140acab2a: 45 0f 13 2c ce              	movlps	%xmm13, (%r14,%rcx,8)
140acab2f: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140acab36: 83 78 b8 02                 	cmpl	$0x2, -0x48(%rax)
140acab3a: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140acab41: 0f 84 a2 03 00 00           	je	0x140acaee9 <.text+0xac9ee9>
140acab47: 49 8d 44 24 ff              	leaq	-0x1(%r12), %rax
140acab4c: 48 8b 95 08 03 00 00        	movq	0x308(%rbp), %rdx
140acab53: 48 85 d2                    	testq	%rdx, %rdx
140acab56: 0f 84 85 03 00 00           	je	0x140acaee1 <.text+0xac9ee1>
140acab5c: 4d 89 f0                    	movq	%r14, %r8
140acab5f: 48 39 c2                    	cmpq	%rax, %rdx
140acab62: 0f 84 7d 03 00 00           	je	0x140acaee5 <.text+0xac9ee5>
140acab68: e9 7c 03 00 00              	jmp	0x140acaee9 <.text+0xac9ee9>
140acab6d: b8 04 00 00 00              	movl	$0x4, %eax
140acab72: 48 c7 85 38 03 00 00 00 00 00 00    	movq	$0x0, 0x338(%rbp)
140acab7d: 48 89 85 30 03 00 00        	movq	%rax, 0x330(%rbp)
140acab84: 48 89 c1                    	movq	%rax, %rcx
140acab87: 48 89 fa                    	movq	%rdi, %rdx
140acab8a: 49 89 d8                    	movq	%rbx, %r8
140acab8d: e8 6e 90 e5 01              	callq	0x142923c00 <.text+0x2922c00>
140acab92: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140acab99: 4c 8b a0 88 fd ff ff        	movq	-0x278(%rax), %r12
140acaba0: 4a 8d 1c a5 00 00 00 00     	leaq	(,%r12,4), %rbx
140acaba8: 4c 89 e0                    	movq	%r12, %rax
140acabab: 48 c1 e8 3e                 	shrq	$0x3e, %rax
140acabaf: 0f 85 b5 0d 00 00           	jne	0x140acb96a <.text+0xaca96a>
140acabb5: 41 bf 00 00 00 00           	movl	$0x0, %r15d
140acabbb: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140acabc5: 48 39 c3                    	cmpq	%rax, %rbx
140acabc8: 0f 87 9f 0d 00 00           	ja	0x140acb96d <.text+0xaca96d>
140acabce: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140acabd5: 48 8b b8 80 fd ff ff        	movq	-0x280(%rax), %rdi
140acabdc: 48 85 db                    	testq	%rbx, %rbx
140acabdf: 74 2b                       	je	0x140acac0c <.text+0xac9c0c>
140acabe1: 0f b6 05 21 a0 b6 02        	movzbl	0x2b6a021(%rip), %eax   # 0x143634c09
140acabe8: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140acabee: ba 04 00 00 00              	movl	$0x4, %edx
140acabf3: 48 89 d9                    	movq	%rbx, %rcx
140acabf6: e8 15 90 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acabfb: 48 85 c0                    	testq	%rax, %rax
140acabfe: 0f 84 69 0d 00 00           	je	0x140acb96d <.text+0xaca96d>
140acac04: 49 89 c6                    	movq	%rax, %r14
140acac07: 4d 89 e7                    	movq	%r12, %r15
140acac0a: eb 09                       	jmp	0x140acac15 <.text+0xac9c15>
140acac0c: 41 be 04 00 00 00           	movl	$0x4, %r14d
140acac12: 45 31 ff                    	xorl	%r15d, %r15d
140acac15: 4c 89 f1                    	movq	%r14, %rcx
140acac18: 48 89 fa                    	movq	%rdi, %rdx
140acac1b: 49 89 d8                    	movq	%rbx, %r8
140acac1e: e8 dd 8f e5 01              	callq	0x142923c00 <.text+0x2922c00>
140acac23: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140acac2a: 66 0f 6e 80 90 fd ff ff     	movd	-0x270(%rax), %xmm0
140acac32: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
140acac39: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
140acac40: 0f 10 8d b0 02 00 00        	movups	0x2b0(%rbp), %xmm1
140acac47: 0f 10 95 c0 02 00 00        	movups	0x2c0(%rbp), %xmm2
140acac4e: f3 0f 6f 9d d0 02 00 00     	movdqu	0x2d0(%rbp), %xmm3
140acac56: 48 8d 8d 08 01 00 00        	leaq	0x108(%rbp), %rcx
140acac5d: 0f 11 09                    	movups	%xmm1, (%rcx)
140acac60: 0f 11 51 10                 	movups	%xmm2, 0x10(%rcx)
140acac64: f3 0f 7f 59 20              	movdqu	%xmm3, 0x20(%rcx)
140acac69: 48 8b 85 e0 02 00 00        	movq	0x2e0(%rbp), %rax
140acac70: 48 89 41 30                 	movq	%rax, 0x30(%rcx)
140acac74: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
140acac7b: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140acac82: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
140acac89: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
140acac90: 48 89 b5 50 01 00 00        	movq	%rsi, 0x150(%rbp)
140acac97: 4c 89 bd 58 01 00 00        	movq	%r15, 0x158(%rbp)
140acac9e: 4c 89 b5 60 01 00 00        	movq	%r14, 0x160(%rbp)
140acaca5: 4c 89 a5 68 01 00 00        	movq	%r12, 0x168(%rbp)
140acacac: 66 0f 7e 85 70 01 00 00     	movd	%xmm0, 0x170(%rbp)
140acacb4: 48 b8 0c 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff4, %rax # imm = 0x800000000000000C
140acacbe: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140acacc5: c6 85 43 03 00 00 01        	movb	$0x1, 0x343(%rbp)
140acaccc: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
140acacd3: 4c 8d b5 00 01 00 00        	leaq	0x100(%rbp), %r14
140acacda: 4c 89 f2                    	movq	%r14, %rdx
140acacdd: e8 5e 7e fb ff              	callq	0x140a82b40 <.text+0xa81b40>
140acace2: 8b 85 1c 02 00 00           	movl	0x21c(%rbp), %eax
140acace8: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
140acacec: 89 01                       	movl	%eax, (%rcx)
140acacee: f3 0f 6f 85 90 00 00 00     	movdqu	0x90(%rbp), %xmm0
140acacf6: 66 0f 7f 85 b0 02 00 00     	movdqa	%xmm0, 0x2b0(%rbp)
140acacfe: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
140acad05: 48 8b 8d b0 02 00 00        	movq	0x2b0(%rbp), %rcx
140acad0c: 48 8b 95 b8 02 00 00        	movq	0x2b8(%rbp), %rdx
140acad13: 48 c7 85 b0 02 00 00 00 00 00 00    	movq	$0x0, 0x2b0(%rbp)
140acad1e: 48 c7 85 b8 02 00 00 08 00 00 00    	movq	$0x8, 0x2b8(%rbp)
140acad29: 48 c7 85 c0 02 00 00 00 00 00 00    	movq	$0x0, 0x2c0(%rbp)
140acad34: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140acad38: 48 8d 04 c2                 	leaq	(%rdx,%rax,8), %rax
140acad3c: 48 89 95 00 01 00 00        	movq	%rdx, 0x100(%rbp)
140acad43: 48 89 95 08 01 00 00        	movq	%rdx, 0x108(%rbp)
140acad4a: 48 89 8d 10 01 00 00        	movq	%rcx, 0x110(%rbp)
140acad51: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
140acad58: 4c 89 f1                    	movq	%r14, %rcx
140acad5b: e8 f0 c1 bc 00              	callq	0x141696f50 <.text+0x1695f50>
140acad60: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
140acad67: 48 89 95 10 01 00 00        	movq	%rdx, 0x110(%rbp)
140acad6e: 48 c7 85 00 01 00 00 07 00 00 00    	movq	$0x7, 0x100(%rbp)
140acad79: c6 85 43 03 00 00 00        	movb	$0x0, 0x343(%rbp)
140acad80: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
140acad84: 4c 89 f2                    	movq	%r14, %rdx
140acad87: 4c 8d 05 9a 5c 01 02        	leaq	0x2015c9a(%rip), %r8    # 0x142ae0a28
140acad8e: e8 2d a1 fb ff              	callq	0x140a84ec0 <.text+0xa83ec0>
140acad93: 48 8b bd 20 03 00 00        	movq	0x320(%rbp), %rdi
140acad9a: 8b 85 88 02 00 00           	movl	0x288(%rbp), %eax
140acada0: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
140acada7: 89 01                       	movl	%eax, (%rcx)
140acada9: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
140acadb0: 48 8b 70 30                 	movq	0x30(%rax), %rsi
140acadb4: 48 8b 58 40                 	movq	0x40(%rax), %rbx
140acadb8: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
140acadbc: 75 13                       	jne	0x140acadd1 <.text+0xac9dd1>
140acadbe: 48 8b 8d e8 00 00 00        	movq	0xe8(%rbp), %rcx
140acadc5: 48 8d 15 7c f3 16 02        	leaq	0x216f37c(%rip), %rdx   # 0x142c3a148
140acadcc: e8 df 7f a9 ff              	callq	0x140562db0 <.text+0x561db0>
140acadd1: 48 8b 95 70 02 00 00        	movq	0x270(%rbp), %rdx
140acadd8: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140acaddc: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
140acade0: 48 89 1c c8                 	movq	%rbx, (%rax,%rcx,8)
140acade4: 48 89 7c c8 08              	movq	%rdi, 0x8(%rax,%rcx,8)
140acade9: c6 44 c8 10 01              	movb	$0x1, 0x10(%rax,%rcx,8)
140acadee: 48 ff c6                    	incq	%rsi
140acadf1: 48 89 72 30                 	movq	%rsi, 0x30(%rdx)
140acadf5: 48 ff 42 40                 	incq	0x40(%rdx)
140acadf9: 48 8b b5 98 02 00 00        	movq	0x298(%rbp), %rsi
140acae00: 4c 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %r10
140acae07: 49 8b 02                    	movq	(%r10), %rax
140acae0a: 48 89 c1                    	movq	%rax, %rcx
140acae0d: 4c 8b 5d 00                 	movq	(%rbp), %r11
140acae11: 49 2b 4b 18                 	subq	0x18(%r11), %rcx
140acae15: bf 00 00 00 00              	movl	$0x0, %edi
140acae1a: 48 0f 42 cf                 	cmovbq	%rdi, %rcx
140acae1e: 49 2b 43 38                 	subq	0x38(%r11), %rax
140acae22: 48 0f 42 c7                 	cmovbq	%rdi, %rax
140acae26: 49 8b 53 30                 	movq	0x30(%r11), %rdx
140acae2a: 48 29 c2                    	subq	%rax, %rdx
140acae2d: 48 0f 42 d7                 	cmovbq	%rdi, %rdx
140acae31: 41 0f 96 c0                 	setbe	%r8b
140acae35: 4d 8b 4b 10                 	movq	0x10(%r11), %r9
140acae39: 49 29 c9                    	subq	%rcx, %r9
140acae3c: 4c 0f 42 cf                 	cmovbq	%rdi, %r9
140acae40: 49 8b 43 40                 	movq	0x40(%r11), %rax
140acae44: 0f 96 c1                    	setbe	%cl
140acae47: 49 01 d1                    	addq	%rdx, %r9
140acae4a: 48 89 c2                    	movq	%rax, %rdx
140acae4d: 4c 29 ca                    	subq	%r9, %rdx
140acae50: 49 89 12                    	movq	%rdx, (%r10)
140acae53: 44 84 c1                    	testb	%r8b, %cl
140acae56: 0f 85 69 ef ff ff           	jne	0x140ac9dc5 <.text+0xac8dc5>
140acae5c: 49 89 f6                    	movq	%rsi, %r14
140acae5f: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
140acae66: 48 89 01                    	movq	%rax, (%rcx)
140acae69: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
140acae70: 0f b6 58 f6                 	movzbl	-0xa(%rax), %ebx
140acae74: 00 db                       	addb	%bl, %bl
140acae76: 48 8d 8d 08 01 00 00        	leaq	0x108(%rbp), %rcx
140acae7d: 48 8d 95 28 02 00 00        	leaq	0x228(%rbp), %rdx
140acae84: 0f 28 d6                    	movaps	%xmm6, %xmm2
140acae87: e8 54 1f 1d 00              	callq	0x140c9cde0 <.text+0xc9bde0>
140acae8c: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
140acae93: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140acae9a: 88 9d 14 01 00 00           	movb	%bl, 0x114(%rbp)
140acaea0: c6 85 15 01 00 00 00        	movb	$0x0, 0x115(%rbp)
140acaea7: 8b 85 18 02 00 00           	movl	0x218(%rbp), %eax
140acaead: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
140acaeb1: 89 01                       	movl	%eax, (%rcx)
140acaeb3: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140acaeba: 48 8b 70 30                 	movq	0x30(%rax), %rsi
140acaebe: 48 8b 78 40                 	movq	0x40(%rax), %rdi
140acaec2: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
140acaec6: 0f 85 b4 ee ff ff           	jne	0x140ac9d80 <.text+0xac8d80>
140acaecc: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140acaed0: 48 8d 15 71 f2 16 02        	leaq	0x216f271(%rip), %rdx   # 0x142c3a148
140acaed7: e8 54 7d a9 ff              	callq	0x140562c30 <.text+0x561c30>
140acaedc: e9 9f ee ff ff              	jmp	0x140ac9d80 <.text+0xac8d80>
140acaee1: 4d 8d 04 c6                 	leaq	(%r14,%rax,8), %r8
140acaee5: 45 0f 13 28                 	movlps	%xmm13, (%r8)
140acaee9: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
140acaef0: 48 89 85 b0 02 00 00        	movq	%rax, 0x2b0(%rbp)
140acaef7: 4c 89 b5 b8 02 00 00        	movq	%r14, 0x2b8(%rbp)
140acaefe: 4c 89 a5 c0 02 00 00        	movq	%r12, 0x2c0(%rbp)
140acaf05: 48 89 85 00 03 00 00        	movq	%rax, 0x300(%rbp)
140acaf0c: 4c 89 b5 f8 02 00 00        	movq	%r14, 0x2f8(%rbp)
140acaf13: 48 89 9d f0 02 00 00        	movq	%rbx, 0x2f0(%rbp)
140acaf1a: 4c 89 bd e8 02 00 00        	movq	%r15, 0x2e8(%rbp)
140acaf21: c6 85 45 03 00 00 01        	movb	$0x1, 0x345(%rbp)
140acaf28: c6 85 44 03 00 00 00        	movb	$0x0, 0x344(%rbp)
140acaf2f: 48 8d 95 b0 02 00 00        	leaq	0x2b0(%rbp), %rdx
140acaf36: 45 31 c0                    	xorl	%r8d, %r8d
140acaf39: e8 f2 e4 99 00              	callq	0x141469430 <.text+0x1468430>
140acaf3e: 48 8b bd 00 01 00 00        	movq	0x100(%rbp), %rdi
140acaf45: 48 89 f8                    	movq	%rdi, %rax
140acaf48: 48 f7 d8                    	negq	%rax
140acaf4b: 0f 80 db 08 00 00           	jo	0x140acb82c <.text+0xaca82c>
140acaf51: 4c 8b b5 08 01 00 00        	movq	0x108(%rbp), %r14
140acaf58: 4c 8b ad 10 01 00 00        	movq	0x110(%rbp), %r13
140acaf5f: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
140acaf66: 48 89 85 30 03 00 00        	movq	%rax, 0x330(%rbp)
140acaf6d: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
140acaf74: 48 89 85 a0 02 00 00        	movq	%rax, 0x2a0(%rbp)
140acaf7b: 48 8b 85 28 01 00 00        	movq	0x128(%rbp), %rax
140acaf82: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
140acaf86: f3 0f 10 bd 30 01 00 00     	movss	0x130(%rbp), %xmm7
140acaf8e: 8b 85 34 01 00 00           	movl	0x134(%rbp), %eax
140acaf94: 89 85 20 02 00 00           	movl	%eax, 0x220(%rbp)
140acaf9a: 48 85 db                    	testq	%rbx, %rbx
140acaf9d: 74 15                       	je	0x140acafb4 <.text+0xac9fb4>
140acaf9f: 48 c1 e3 02                 	shlq	$0x2, %rbx
140acafa3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acafa9: 4c 89 f9                    	movq	%r15, %rcx
140acafac: 48 89 da                    	movq	%rbx, %rdx
140acafaf: e8 6c 8c b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acafb4: 48 8b 9d 08 03 00 00        	movq	0x308(%rbp), %rbx
140acafbb: 0f b6 05 47 9c b6 02        	movzbl	0x2b69c47(%rip), %eax   # 0x143634c09
140acafc2: 48 85 db                    	testq	%rbx, %rbx
140acafc5: 48 89 bd 58 02 00 00        	movq	%rdi, 0x258(%rbp)
140acafcc: 74 38                       	je	0x140acb006 <.text+0xaca006>
140acafce: b9 10 00 00 00              	movl	$0x10, %ecx
140acafd3: ba 08 00 00 00              	movl	$0x8, %edx
140acafd8: e8 33 8c b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acafdd: 48 85 c0                    	testq	%rax, %rax
140acafe0: 0f 84 0a 09 00 00           	je	0x140acb8f0 <.text+0xaca8f0>
140acafe6: 49 89 c4                    	movq	%rax, %r12
140acafe9: f3 0f 7e 85 08 03 00 00     	movq	0x308(%rbp), %xmm0
140acaff1: 66 0f 70 c0 44              	pshufd	$0x44, %xmm0, %xmm0     # xmm0 = xmm0[0,1,0,1]
140acaff6: 66 41 0f d4 c3              	paddq	%xmm11, %xmm0
140acaffb: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
140acafff: b8 10 00 00 00              	movl	$0x10, %eax
140acb004: eb 2e                       	jmp	0x140acb034 <.text+0xaca034>
140acb006: b9 08 00 00 00              	movl	$0x8, %ecx
140acb00b: ba 08 00 00 00              	movl	$0x8, %edx
140acb010: e8 fb 8b b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acb015: 48 85 c0                    	testq	%rax, %rax
140acb018: 0f 84 80 09 00 00           	je	0x140acb99e <.text+0xaca99e>
140acb01e: 49 89 c4                    	movq	%rax, %r12
140acb021: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
140acb028: 48 ff c0                    	incq	%rax
140acb02b: 49 89 04 24                 	movq	%rax, (%r12)
140acb02f: b8 08 00 00 00              	movl	$0x8, %eax
140acb034: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
140acb03b: 49 8b 04 24                 	movq	(%r12), %rax
140acb03f: 49 39 c5                    	cmpq	%rax, %r13
140acb042: 4c 89 b5 38 03 00 00        	movq	%r14, 0x338(%rbp)
140acb049: 4c 89 a5 90 02 00 00        	movq	%r12, 0x290(%rbp)
140acb050: 76 25                       	jbe	0x140acb077 <.text+0xaca077>
140acb052: f2 41 0f 10 04 c6           	movsd	(%r14,%rax,8), %xmm0
140acb058: 41 0f 5c c5                 	subps	%xmm13, %xmm0
140acb05c: 0f 59 c0                    	mulps	%xmm0, %xmm0
140acb05f: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
140acb063: f3 0f 58 c8                 	addss	%xmm0, %xmm1
140acb067: 0f 57 c0                    	xorps	%xmm0, %xmm0
140acb06a: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
140acb06e: 40 b7 01                    	movb	$0x1, %dil
140acb071: 44 0f 2e d0                 	ucomiss	%xmm0, %xmm10
140acb075: 77 3f                       	ja	0x140acb0b6 <.text+0xaca0b6>
140acb077: 48 85 db                    	testq	%rbx, %rbx
140acb07a: 74 2f                       	je	0x140acb0ab <.text+0xaca0ab>
140acb07c: 49 8b 44 24 08              	movq	0x8(%r12), %rax
140acb081: 49 39 c5                    	cmpq	%rax, %r13
140acb084: 76 25                       	jbe	0x140acb0ab <.text+0xaca0ab>
140acb086: f2 41 0f 10 04 c6           	movsd	(%r14,%rax,8), %xmm0
140acb08c: 41 0f 5c c5                 	subps	%xmm13, %xmm0
140acb090: 0f 59 c0                    	mulps	%xmm0, %xmm0
140acb093: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
140acb097: f3 0f 58 c8                 	addss	%xmm0, %xmm1
140acb09b: 0f 57 c0                    	xorps	%xmm0, %xmm0
140acb09e: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
140acb0a2: 40 b7 01                    	movb	$0x1, %dil
140acb0a5: 44 0f 2e d0                 	ucomiss	%xmm0, %xmm10
140acb0a9: 77 0b                       	ja	0x140acb0b6 <.text+0xaca0b6>
140acb0ab: 4d 85 ed                    	testq	%r13, %r13
140acb0ae: 0f 84 93 01 00 00           	je	0x140acb247 <.text+0xaca247>
140acb0b4: 31 ff                       	xorl	%edi, %edi
140acb0b6: 4b 8d 1c ee                 	leaq	(%r14,%r13,8), %rbx
140acb0ba: f3 41 0f 10 0e              	movss	(%r14), %xmm1
140acb0bf: 66 41 0f 6e 56 04           	movd	0x4(%r14), %xmm2
140acb0c5: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
140acb0cc: 83 38 01                    	cmpl	$0x1, (%rax)
140acb0cf: 0f 85 10 01 00 00           	jne	0x140acb1e5 <.text+0xaca1e5>
140acb0d5: c6 85 47 03 00 00 01        	movb	$0x1, 0x347(%rbp)
140acb0dc: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
140acb0e3: e8 48 f5 1c 00              	callq	0x140c9a630 <.text+0xc99630>
140acb0e8: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
140acb0ef: 84 c0                       	testb	%al, %al
140acb0f1: 74 3e                       	je	0x140acb131 <.text+0xaca131>
140acb0f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140acb100: 4c 8d 71 08                 	leaq	0x8(%rcx), %r14
140acb104: 49 39 de                    	cmpq	%rbx, %r14
140acb107: 0f 84 35 01 00 00           	je	0x140acb242 <.text+0xaca242>
140acb10d: f3 0f 10 49 08              	movss	0x8(%rcx), %xmm1
140acb112: 66 0f 6e 51 0c              	movd	0xc(%rcx), %xmm2
140acb117: c6 85 47 03 00 00 01        	movb	$0x1, 0x347(%rbp)
140acb11e: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
140acb125: e8 06 f5 1c 00              	callq	0x140c9a630 <.text+0xc99630>
140acb12a: 4c 89 f1                    	movq	%r14, %rcx
140acb12d: 84 c0                       	testb	%al, %al
140acb12f: 75 cf                       	jne	0x140acb100 <.text+0xaca100>
140acb131: 48 8b 45 18                 	movq	0x18(%rbp), %rax
140acb135: 8b 8d 10 02 00 00           	movl	0x210(%rbp), %ecx
140acb13b: 89 08                       	movl	%ecx, (%rax)
140acb13d: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
140acb144: 48 8b 70 30                 	movq	0x30(%rax), %rsi
140acb148: 48 8b 78 40                 	movq	0x40(%rax), %rdi
140acb14c: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
140acb150: 75 1a                       	jne	0x140acb16c <.text+0xaca16c>
140acb152: c6 85 47 03 00 00 01        	movb	$0x1, 0x347(%rbp)
140acb159: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
140acb160: 48 8d 15 e1 ef 16 02        	leaq	0x216efe1(%rip), %rdx   # 0x142c3a148
140acb167: e8 04 77 a9 ff              	callq	0x140562870 <.text+0x561870>
140acb16c: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
140acb173: 48 8b 41 28                 	movq	0x28(%rcx), %rax
140acb177: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
140acb17b: 48 ff c6                    	incq	%rsi
140acb17e: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
140acb182: 48 ff 41 40                 	incq	0x40(%rcx)
140acb186: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140acb18a: 8b 8d 14 02 00 00           	movl	0x214(%rbp), %ecx
140acb190: 89 08                       	movl	%ecx, (%rax)
140acb192: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
140acb199: 48 8b 70 30                 	movq	0x30(%rax), %rsi
140acb19d: 48 8b 78 40                 	movq	0x40(%rax), %rdi
140acb1a1: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
140acb1a5: 75 1a                       	jne	0x140acb1c1 <.text+0xaca1c1>
140acb1a7: c6 85 47 03 00 00 01        	movb	$0x1, 0x347(%rbp)
140acb1ae: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
140acb1b5: 48 8d 15 8c ef 16 02        	leaq	0x216ef8c(%rip), %rdx   # 0x142c3a148
140acb1bc: e8 af 76 a9 ff              	callq	0x140562870 <.text+0x561870>
140acb1c1: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
140acb1c8: 48 8b 41 28                 	movq	0x28(%rcx), %rax
140acb1cc: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
140acb1d0: 48 ff c6                    	incq	%rsi
140acb1d3: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
140acb1d7: 48 ff 41 40                 	incq	0x40(%rcx)
140acb1db: 40 b7 01                    	movb	$0x1, %dil
140acb1de: 31 f6                       	xorl	%esi, %esi
140acb1e0: e9 2b 04 00 00              	jmp	0x140acb610 <.text+0xaca610>
140acb1e5: c6 85 47 03 00 00 01        	movb	$0x1, 0x347(%rbp)
140acb1ec: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
140acb1f3: e8 f8 ce 1c 00              	callq	0x140c980f0 <.text+0xc970f0>
140acb1f8: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
140acb1ff: 84 c0                       	testb	%al, %al
140acb201: 0f 84 2a ff ff ff           	je	0x140acb131 <.text+0xaca131>
140acb207: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140acb210: 4c 8d 71 08                 	leaq	0x8(%rcx), %r14
140acb214: 49 39 de                    	cmpq	%rbx, %r14
140acb217: 74 29                       	je	0x140acb242 <.text+0xaca242>
140acb219: f3 0f 10 49 08              	movss	0x8(%rcx), %xmm1
140acb21e: 66 0f 6e 51 0c              	movd	0xc(%rcx), %xmm2
140acb223: c6 85 47 03 00 00 01        	movb	$0x1, 0x347(%rbp)
140acb22a: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
140acb231: e8 ba ce 1c 00              	callq	0x140c980f0 <.text+0xc970f0>
140acb236: 4c 89 f1                    	movq	%r14, %rcx
140acb239: 84 c0                       	testb	%al, %al
140acb23b: 75 d3                       	jne	0x140acb210 <.text+0xaca210>
140acb23d: e9 ef fe ff ff              	jmp	0x140acb131 <.text+0xaca131>
140acb242: 40 84 ff                    	testb	%dil, %dil
140acb245: 75 94                       	jne	0x140acb1db <.text+0xaca1db>
140acb247: 48 8b 85 10 03 00 00        	movq	0x310(%rbp), %rax
140acb24e: 48 8b 58 10                 	movq	0x10(%rax), %rbx
140acb252: 48 89 d8                    	movq	%rbx, %rax
140acb255: b9 0c 00 00 00              	movl	$0xc, %ecx
140acb25a: 48 f7 e1                    	mulq	%rcx
140acb25d: 49 89 c7                    	movq	%rax, %r15
140acb260: 0f 80 b6 07 00 00           	jo	0x140acba1c <.text+0xacaa1c>
140acb266: 41 be 00 00 00 00           	movl	$0x0, %r14d
140acb26c: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140acb276: 49 39 c7                    	cmpq	%rax, %r15
140acb279: 0f 87 a0 07 00 00           	ja	0x140acba1f <.text+0xacaa1f>
140acb27f: 48 8b 85 10 03 00 00        	movq	0x310(%rbp), %rax
140acb286: 48 8b 78 08                 	movq	0x8(%rax), %rdi
140acb28a: 4d 85 ff                    	testq	%r15, %r15
140acb28d: 74 2c                       	je	0x140acb2bb <.text+0xaca2bb>
140acb28f: 0f b6 05 73 99 b6 02        	movzbl	0x2b69973(%rip), %eax   # 0x143634c09
140acb296: 41 be 04 00 00 00           	movl	$0x4, %r14d
140acb29c: ba 04 00 00 00              	movl	$0x4, %edx
140acb2a1: 4c 89 f9                    	movq	%r15, %rcx
140acb2a4: e8 67 89 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acb2a9: 48 85 c0                    	testq	%rax, %rax
140acb2ac: 0f 84 6d 07 00 00           	je	0x140acba1f <.text+0xacaa1f>
140acb2b2: 48 89 9d 18 03 00 00        	movq	%rbx, 0x318(%rbp)
140acb2b9: eb 10                       	jmp	0x140acb2cb <.text+0xaca2cb>
140acb2bb: b8 04 00 00 00              	movl	$0x4, %eax
140acb2c0: 48 c7 85 18 03 00 00 00 00 00 00    	movq	$0x0, 0x318(%rbp)
140acb2cb: 48 89 85 a8 02 00 00        	movq	%rax, 0x2a8(%rbp)
140acb2d2: 48 89 c1                    	movq	%rax, %rcx
140acb2d5: 48 89 fa                    	movq	%rdi, %rdx
140acb2d8: 4d 89 f8                    	movq	%r15, %r8
140acb2db: e8 20 89 e5 01              	callq	0x142923c00 <.text+0x2922c00>
140acb2e0: 4c 8b b5 20 03 00 00        	movq	0x320(%rbp), %r14
140acb2e7: 8b 85 8c 02 00 00           	movl	0x28c(%rbp), %eax
140acb2ed: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140acb2f4: 89 01                       	movl	%eax, (%rcx)
140acb2f6: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
140acb2fd: 48 8b 78 30                 	movq	0x30(%rax), %rdi
140acb301: 4c 8b 78 40                 	movq	0x40(%rax), %r15
140acb305: 48 3b 78 20                 	cmpq	0x20(%rax), %rdi
140acb309: 75 13                       	jne	0x140acb31e <.text+0xaca31e>
140acb30b: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140acb312: 48 8d 15 2f ee 16 02        	leaq	0x216ee2f(%rip), %rdx   # 0x142c3a148
140acb319: e8 92 77 a9 ff              	callq	0x140562ab0 <.text+0x561ab0>
140acb31e: 48 8b 95 78 02 00 00        	movq	0x278(%rbp), %rdx
140acb325: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140acb329: 48 8d 0c bf                 	leaq	(%rdi,%rdi,4), %rcx
140acb32d: 4c 8b 85 18 03 00 00        	movq	0x318(%rbp), %r8
140acb334: 4c 89 04 c8                 	movq	%r8, (%rax,%rcx,8)
140acb338: 4c 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %r8
140acb33f: 4c 89 44 c8 08              	movq	%r8, 0x8(%rax,%rcx,8)
140acb344: 48 89 5c c8 10              	movq	%rbx, 0x10(%rax,%rcx,8)
140acb349: 4c 89 74 c8 18              	movq	%r14, 0x18(%rax,%rcx,8)
140acb34e: 4c 89 7c c8 20              	movq	%r15, 0x20(%rax,%rcx,8)
140acb353: 48 ff c7                    	incq	%rdi
140acb356: 48 89 7a 30                 	movq	%rdi, 0x30(%rdx)
140acb35a: 48 ff 42 40                 	incq	0x40(%rdx)
140acb35e: 48 8b 46 a0                 	movq	-0x60(%rsi), %rax
140acb362: 48 8d 1c c5 00 00 00 00     	leaq	(,%rax,8), %rbx
140acb36a: 48 89 85 18 03 00 00        	movq	%rax, 0x318(%rbp)
140acb371: 48 c1 e8 3d                 	shrq	$0x3d, %rax
140acb375: 0f 85 83 06 00 00           	jne	0x140acb9fe <.text+0xaca9fe>
140acb37b: 41 be 00 00 00 00           	movl	$0x0, %r14d
140acb381: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140acb38b: 48 39 c3                    	cmpq	%rax, %rbx
140acb38e: 0f 87 6d 06 00 00           	ja	0x140acba01 <.text+0xacaa01>
140acb394: 48 8b 7e 98                 	movq	-0x68(%rsi), %rdi
140acb398: 48 85 db                    	testq	%rbx, %rbx
140acb39b: 74 24                       	je	0x140acb3c1 <.text+0xaca3c1>
140acb39d: 0f b6 05 65 98 b6 02        	movzbl	0x2b69865(%rip), %eax   # 0x143634c09
140acb3a4: 41 be 04 00 00 00           	movl	$0x4, %r14d
140acb3aa: ba 04 00 00 00              	movl	$0x4, %edx
140acb3af: 48 89 d9                    	movq	%rbx, %rcx
140acb3b2: e8 59 88 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acb3b7: 48 85 c0                    	testq	%rax, %rax
140acb3ba: 75 15                       	jne	0x140acb3d1 <.text+0xaca3d1>
140acb3bc: e9 40 06 00 00              	jmp	0x140acba01 <.text+0xacaa01>
140acb3c1: b8 04 00 00 00              	movl	$0x4, %eax
140acb3c6: 48 c7 85 18 03 00 00 00 00 00 00    	movq	$0x0, 0x318(%rbp)
140acb3d1: 48 89 85 a8 02 00 00        	movq	%rax, 0x2a8(%rbp)
140acb3d8: 48 89 c1                    	movq	%rax, %rcx
140acb3db: 48 89 fa                    	movq	%rdi, %rdx
140acb3de: 49 89 d8                    	movq	%rbx, %r8
140acb3e1: e8 1a 88 e5 01              	callq	0x142923c00 <.text+0x2922c00>
140acb3e6: 48 8b 5e b8                 	movq	-0x48(%rsi), %rbx
140acb3ea: 4c 8d 34 9d 00 00 00 00     	leaq	(,%rbx,4), %r14
140acb3f2: 48 89 d8                    	movq	%rbx, %rax
140acb3f5: 48 c1 e8 3e                 	shrq	$0x3e, %rax
140acb3f9: 0f 85 e8 05 00 00           	jne	0x140acb9e7 <.text+0xaca9e7>
140acb3ff: 41 bc 00 00 00 00           	movl	$0x0, %r12d
140acb405: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140acb40f: 49 39 c6                    	cmpq	%rax, %r14
140acb412: 0f 87 d2 05 00 00           	ja	0x140acb9ea <.text+0xaca9ea>
140acb418: 48 8b 7e b0                 	movq	-0x50(%rsi), %rdi
140acb41c: 4d 85 f6                    	testq	%r14, %r14
140acb41f: 74 28                       	je	0x140acb449 <.text+0xaca449>
140acb421: 0f b6 05 e1 97 b6 02        	movzbl	0x2b697e1(%rip), %eax   # 0x143634c09
140acb428: 41 bc 04 00 00 00           	movl	$0x4, %r12d
140acb42e: ba 04 00 00 00              	movl	$0x4, %edx
140acb433: 4c 89 f1                    	movq	%r14, %rcx
140acb436: e8 d5 87 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acb43b: 48 85 c0                    	testq	%rax, %rax
140acb43e: 0f 84 a6 05 00 00           	je	0x140acb9ea <.text+0xaca9ea>
140acb444: 49 89 c7                    	movq	%rax, %r15
140acb447: eb 08                       	jmp	0x140acb451 <.text+0xaca451>
140acb449: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140acb44f: 31 db                       	xorl	%ebx, %ebx
140acb451: 4c 89 f9                    	movq	%r15, %rcx
140acb454: 48 89 fa                    	movq	%rdi, %rdx
140acb457: 4d 89 f0                    	movq	%r14, %r8
140acb45a: e8 a1 87 e5 01              	callq	0x142923c00 <.text+0x2922c00>
140acb45f: f3 44 0f 10 6e e4           	movss	-0x1c(%rsi), %xmm13
140acb465: 8b 46 c8                    	movl	-0x38(%rsi), %eax
140acb468: 8d 48 fe                    	leal	-0x2(%rax), %ecx
140acb46b: 83 f9 02                    	cmpl	$0x2, %ecx
140acb46e: bf 02 00 00 00              	movl	$0x2, %edi
140acb473: 0f 43 cf                    	cmovael	%edi, %ecx
140acb476: 85 c9                       	testl	%ecx, %ecx
140acb478: 74 24                       	je	0x140acb49e <.text+0xaca49e>
140acb47a: 83 f9 02                    	cmpl	$0x2, %ecx
140acb47d: 75 1a                       	jne	0x140acb499 <.text+0xaca499>
140acb47f: 48 8b 4e dc                 	movq	-0x24(%rsi), %rcx
140acb483: 48 89 8d 10 01 00 00        	movq	%rcx, 0x110(%rbp)
140acb48a: 0f 10 46 cc                 	movups	-0x34(%rsi), %xmm0
140acb48e: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
140acb495: 89 c7                       	movl	%eax, %edi
140acb497: eb 05                       	jmp	0x140acb49e <.text+0xaca49e>
140acb499: bf 03 00 00 00              	movl	$0x3, %edi
140acb49e: 45 0f 28 ca                 	movaps	%xmm10, %xmm9
140acb4a2: 66 45 0f 6f d3              	movdqa	%xmm11, %xmm10
140acb4a7: f2 44 0f 10 76 e8           	movsd	-0x18(%rsi), %xmm14
140acb4ad: 44 0f b6 76 f8              	movzbl	-0x8(%rsi), %r14d
140acb4b2: 44 0f b6 66 f9              	movzbl	-0x7(%rsi), %r12d
140acb4b7: f3 44 0f 10 7e f0           	movss	-0x10(%rsi), %xmm15
140acb4bd: f3 44 0f 10 5e f4           	movss	-0xc(%rsi), %xmm11
140acb4c3: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
140acb4ca: 48 89 85 c0 02 00 00        	movq	%rax, 0x2c0(%rbp)
140acb4d1: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
140acb4d8: 0f 29 85 b0 02 00 00        	movaps	%xmm0, 0x2b0(%rbp)
140acb4df: 48 8b 95 18 03 00 00        	movq	0x318(%rbp), %rdx
140acb4e6: 48 85 d2                    	testq	%rdx, %rdx
140acb4e9: 74 16                       	je	0x140acb501 <.text+0xaca501>
140acb4eb: 48 c1 e2 03                 	shlq	$0x3, %rdx
140acb4ef: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acb4f5: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
140acb4fc: e8 1f 87 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acb501: 48 85 db                    	testq	%rbx, %rbx
140acb504: 48 8b b5 d8 00 00 00        	movq	0xd8(%rbp), %rsi
140acb50b: 74 15                       	je	0x140acb522 <.text+0xaca522>
140acb50d: 48 c1 e3 02                 	shlq	$0x2, %rbx
140acb511: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acb517: 4c 89 f9                    	movq	%r15, %rcx
140acb51a: 48 89 da                    	movq	%rbx, %rdx
140acb51d: e8 fe 86 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acb522: 48 8b 45 78                 	movq	0x78(%rbp), %rax
140acb526: 8b 8d 24 02 00 00           	movl	0x224(%rbp), %ecx
140acb52c: 89 08                       	movl	%ecx, (%rax)
140acb52e: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
140acb535: 48 8d 8d 08 01 00 00        	leaq	0x108(%rbp), %rcx
140acb53c: 48 89 41 44                 	movq	%rax, 0x44(%rcx)
140acb540: 0f 28 85 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm0
140acb547: 0f 11 41 34                 	movups	%xmm0, 0x34(%rcx)
140acb54b: 48 8b 95 20 03 00 00        	movq	0x320(%rbp), %rdx
140acb552: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
140acb559: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140acb560: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
140acb567: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
140acb56e: 4c 89 ad 10 01 00 00        	movq	%r13, 0x110(%rbp)
140acb575: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
140acb57c: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
140acb583: 48 8b 85 a0 02 00 00        	movq	0x2a0(%rbp), %rax
140acb58a: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
140acb591: 48 8b 45 68                 	movq	0x68(%rbp), %rax
140acb595: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
140acb59c: f3 0f 11 bd 30 01 00 00     	movss	%xmm7, 0x130(%rbp)
140acb5a4: 8b 85 20 02 00 00           	movl	0x220(%rbp), %eax
140acb5aa: 89 85 34 01 00 00           	movl	%eax, 0x134(%rbp)
140acb5b0: 89 bd 38 01 00 00           	movl	%edi, 0x138(%rbp)
140acb5b6: f3 44 0f 11 ad 54 01 00 00  	movss	%xmm13, 0x154(%rbp)
140acb5bf: 44 0f 13 b5 58 01 00 00     	movlps	%xmm14, 0x158(%rbp)
140acb5c7: f3 44 0f 11 bd 60 01 00 00  	movss	%xmm15, 0x160(%rbp)
140acb5d0: f3 44 0f 11 9d 64 01 00 00  	movss	%xmm11, 0x164(%rbp)
140acb5d9: 44 88 b5 68 01 00 00        	movb	%r14b, 0x168(%rbp)
140acb5e0: 44 88 a5 69 01 00 00        	movb	%r12b, 0x169(%rbp)
140acb5e7: c6 85 47 03 00 00 00        	movb	$0x0, 0x347(%rbp)
140acb5ee: 48 89 f1                    	movq	%rsi, %rcx
140acb5f1: 4c 8d 85 00 01 00 00        	leaq	0x100(%rbp), %r8
140acb5f8: e8 c3 e5 fd ff              	callq	0x140aa9bc0 <.text+0xaa8bc0>
140acb5fd: 66 45 0f 6f da              	movdqa	%xmm10, %xmm11
140acb602: 45 0f 28 d1                 	movaps	%xmm9, %xmm10
140acb606: 66 45 0f 76 c9              	pcmpeqd	%xmm9, %xmm9
140acb60b: 40 b6 01                    	movb	$0x1, %sil
140acb60e: 31 ff                       	xorl	%edi, %edi
140acb610: 4c 8b b5 20 03 00 00        	movq	0x320(%rbp), %r14
140acb617: 8b 85 88 02 00 00           	movl	0x288(%rbp), %eax
140acb61d: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
140acb624: 89 01                       	movl	%eax, (%rcx)
140acb626: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
140acb62d: 48 8b 58 30                 	movq	0x30(%rax), %rbx
140acb631: 4c 8b 78 40                 	movq	0x40(%rax), %r15
140acb635: 48 3b 58 20                 	cmpq	0x20(%rax), %rbx
140acb639: 75 1a                       	jne	0x140acb655 <.text+0xaca655>
140acb63b: 40 88 bd 47 03 00 00        	movb	%dil, 0x347(%rbp)
140acb642: 48 8b 8d e8 00 00 00        	movq	0xe8(%rbp), %rcx
140acb649: 48 8d 15 f8 ea 16 02        	leaq	0x216eaf8(%rip), %rdx   # 0x142c3a148
140acb650: e8 5b 77 a9 ff              	callq	0x140562db0 <.text+0x561db0>
140acb655: 48 8b 95 70 02 00 00        	movq	0x270(%rbp), %rdx
140acb65c: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140acb660: 48 8d 0c 5b                 	leaq	(%rbx,%rbx,2), %rcx
140acb664: 4c 89 3c c8                 	movq	%r15, (%rax,%rcx,8)
140acb668: 4c 89 74 c8 08              	movq	%r14, 0x8(%rax,%rcx,8)
140acb66d: c6 44 c8 10 00              	movb	$0x0, 0x10(%rax,%rcx,8)
140acb672: 48 ff c3                    	incq	%rbx
140acb675: 48 89 5a 30                 	movq	%rbx, 0x30(%rdx)
140acb679: 48 ff 42 40                 	incq	0x40(%rdx)
140acb67d: 4c 8b b5 20 03 00 00        	movq	0x320(%rbp), %r14
140acb684: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140acb688: 8b 8d 0c 02 00 00           	movl	0x20c(%rbp), %ecx
140acb68e: 89 08                       	movl	%ecx, (%rax)
140acb690: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
140acb697: 48 8b 58 30                 	movq	0x30(%rax), %rbx
140acb69b: 4c 8b 78 40                 	movq	0x40(%rax), %r15
140acb69f: 48 3b 58 20                 	cmpq	0x20(%rax), %rbx
140acb6a3: 75 17                       	jne	0x140acb6bc <.text+0xaca6bc>
140acb6a5: 40 88 bd 47 03 00 00        	movb	%dil, 0x347(%rbp)
140acb6ac: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
140acb6b0: 48 8d 15 91 ea 16 02        	leaq	0x216ea91(%rip), %rdx   # 0x142c3a148
140acb6b7: e8 f4 70 a9 ff              	callq	0x1405627b0 <.text+0x5617b0>
140acb6bc: 48 8b 95 38 02 00 00        	movq	0x238(%rbp), %rdx
140acb6c3: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140acb6c7: 48 89 d9                    	movq	%rbx, %rcx
140acb6ca: 48 c1 e1 06                 	shlq	$0x6, %rcx
140acb6ce: c6 04 08 02                 	movb	$0x2, (%rax,%rcx)
140acb6d2: 4c 89 74 08 08              	movq	%r14, 0x8(%rax,%rcx)
140acb6d7: 4c 89 7c 08 38              	movq	%r15, 0x38(%rax,%rcx)
140acb6dc: 48 ff c3                    	incq	%rbx
140acb6df: 48 89 5a 30                 	movq	%rbx, 0x30(%rdx)
140acb6e3: 48 ff 42 40                 	incq	0x40(%rdx)
140acb6e7: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140acb6ed: 48 8b 8d 90 02 00 00        	movq	0x290(%rbp), %rcx
140acb6f4: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
140acb6fb: e8 20 85 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acb700: 40 84 ff                    	testb	%dil, %dil
140acb703: 4c 8b ad 10 03 00 00        	movq	0x310(%rbp), %r13
140acb70a: 74 49                       	je	0x140acb755 <.text+0xaca755>
140acb70c: 48 8b 95 58 02 00 00        	movq	0x258(%rbp), %rdx
140acb713: 48 85 d2                    	testq	%rdx, %rdx
140acb716: 74 16                       	je	0x140acb72e <.text+0xaca72e>
140acb718: 48 c1 e2 03                 	shlq	$0x3, %rdx
140acb71c: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acb722: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
140acb729: e8 f2 84 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acb72e: 48 83 bd 30 03 00 00 00     	cmpq	$0x0, 0x330(%rbp)
140acb736: 74 1d                       	je	0x140acb755 <.text+0xaca755>
140acb738: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
140acb73f: 48 c1 e2 02                 	shlq	$0x2, %rdx
140acb743: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acb749: 48 8b 8d a0 02 00 00        	movq	0x2a0(%rbp), %rcx
140acb750: e8 cb 84 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acb755: 40 84 f6                    	testb	%sil, %sil
140acb758: 48 8b b5 98 02 00 00        	movq	0x298(%rbp), %rsi
140acb75f: 4c 8d b5 00 01 00 00        	leaq	0x100(%rbp), %r14
140acb766: 0f 84 59 e6 ff ff           	je	0x140ac9dc5 <.text+0xac8dc5>
140acb76c: e9 8f f6 ff ff              	jmp	0x140acae00 <.text+0xac9e00>
140acb771: 48 8d 0d 68 52 01 02        	leaq	0x2015268(%rip), %rcx   # 0x142ae09e0
140acb778: e8 93 db e0 01              	callq	0x1428d9310 <.text+0x28d8310>
140acb77d: 48 8d 0d 0f ea 00 02        	leaq	0x200ea0f(%rip), %rcx   # 0x142ada193
140acb784: 4c 8d 05 25 51 01 02        	leaq	0x2015125(%rip), %r8    # 0x142ae08b0
140acb78b: ba 32 00 00 00              	movl	$0x32, %edx
140acb790: e8 9b dc e0 01              	callq	0x1428d9430 <.text+0x28d8430>
140acb795: c6 85 46 03 00 00 01        	movb	$0x1, 0x346(%rbp)
140acb79c: 48 8d 0d fd 51 01 02        	leaq	0x20151fd(%rip), %rcx   # 0x142ae09a0
140acb7a3: 4c 8d 05 1e 52 01 02        	leaq	0x201521e(%rip), %r8    # 0x142ae09c8
140acb7aa: ba 28 00 00 00              	movl	$0x28, %edx
140acb7af: e8 7c dc e0 01              	callq	0x1428d9430 <.text+0x28d8430>
140acb7b4: e9 7f 02 00 00              	jmp	0x140acba38 <.text+0xacaa38>
140acb7b9: 45 31 f6                    	xorl	%r14d, %r14d
140acb7bc: 4c 8d 05 5d d7 fc 01        	leaq	0x1fcd75d(%rip), %r8    # 0x142a98f20
140acb7c3: 4c 89 f1                    	movq	%r14, %rcx
140acb7c6: 4c 89 fa                    	movq	%r15, %rdx
140acb7c9: e8 15 d8 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acb7ce: e9 65 02 00 00              	jmp	0x140acba38 <.text+0xacaa38>
140acb7d3: b9 08 00 00 00              	movl	$0x8, %ecx
140acb7d8: ba 20 00 00 00              	movl	$0x20, %edx
140acb7dd: e8 1e d8 e0 01              	callq	0x1428d9000 <.text+0x28d8000>
140acb7e2: e9 51 02 00 00              	jmp	0x140acba38 <.text+0xacaa38>
140acb7e7: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
140acb7ee: 48 89 85 00 03 00 00        	movq	%rax, 0x300(%rbp)
140acb7f5: 4c 89 b5 f8 02 00 00        	movq	%r14, 0x2f8(%rbp)
140acb7fc: 48 89 9d f0 02 00 00        	movq	%rbx, 0x2f0(%rbp)
140acb803: 4c 89 bd e8 02 00 00        	movq	%r15, 0x2e8(%rbp)
140acb80a: c6 85 45 03 00 00 01        	movb	$0x1, 0x345(%rbp)
140acb811: c6 85 44 03 00 00 01        	movb	$0x1, 0x344(%rbp)
140acb818: 4c 8d 05 d9 51 01 02        	leaq	0x20151d9(%rip), %r8    # 0x142ae09f8
140acb81f: 4c 89 e2                    	movq	%r12, %rdx
140acb822: e8 f1 dc e0 01              	callq	0x1428d9518 <.text+0x28d8518>
140acb827: e9 0c 02 00 00              	jmp	0x140acba38 <.text+0xacaa38>
140acb82c: 48 8d 85 08 01 00 00        	leaq	0x108(%rbp), %rax
140acb833: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
140acb837: 66 0f 7f 85 b0 02 00 00     	movdqa	%xmm0, 0x2b0(%rbp)
140acb83f: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
140acb846: 48 89 85 00 03 00 00        	movq	%rax, 0x300(%rbp)
140acb84d: 4c 89 b5 f8 02 00 00        	movq	%r14, 0x2f8(%rbp)
140acb854: 48 89 9d f0 02 00 00        	movq	%rbx, 0x2f0(%rbp)
140acb85b: 4c 89 bd e8 02 00 00        	movq	%r15, 0x2e8(%rbp)
140acb862: c6 85 45 03 00 00 01        	movb	$0x1, 0x345(%rbp)
140acb869: c6 85 44 03 00 00 00        	movb	$0x0, 0x344(%rbp)
140acb870: 48 8d 05 99 51 01 02        	leaq	0x2015199(%rip), %rax   # 0x142ae0a10
140acb877: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140acb87c: 48 8d 0d bd 7a 19 02        	leaq	0x2197abd(%rip), %rcx   # 0x142c63340
140acb883: 4c 8d 0d 96 7a 19 02        	leaq	0x2197a96(%rip), %r9    # 0x142c63320
140acb88a: 4c 8d 85 b0 02 00 00        	leaq	0x2b0(%rbp), %r8
140acb891: ba 2b 00 00 00              	movl	$0x2b, %edx
140acb896: e8 c5 de e0 01              	callq	0x1428d9760 <.text+0x28d8760>
140acb89b: e9 98 01 00 00              	jmp	0x140acba38 <.text+0xacaa38>
140acb8a0: 45 31 ff                    	xorl	%r15d, %r15d
140acb8a3: 4c 8d 05 ce d2 16 02        	leaq	0x216d2ce(%rip), %r8    # 0x142c38b78
140acb8aa: 4c 89 f9                    	movq	%r15, %rcx
140acb8ad: 4c 89 f2                    	movq	%r14, %rdx
140acb8b0: e8 2e d7 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acb8b5: 45 31 ff                    	xorl	%r15d, %r15d
140acb8b8: 4c 8d 05 b9 d2 16 02        	leaq	0x216d2b9(%rip), %r8    # 0x142c38b78
140acb8bf: 4c 89 f9                    	movq	%r15, %rcx
140acb8c2: 4c 89 f2                    	movq	%r14, %rdx
140acb8c5: e8 19 d7 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acb8ca: e9 69 01 00 00              	jmp	0x140acba38 <.text+0xacaa38>
140acb8cf: 45 31 f6                    	xorl	%r14d, %r14d
140acb8d2: c6 85 46 03 00 00 00        	movb	$0x0, 0x346(%rbp)
140acb8d9: 4c 8d 05 98 d2 16 02        	leaq	0x216d298(%rip), %r8    # 0x142c38b78
140acb8e0: 4c 89 f1                    	movq	%r14, %rcx
140acb8e3: 48 89 da                    	movq	%rbx, %rdx
140acb8e6: e8 f8 d6 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acb8eb: e9 48 01 00 00              	jmp	0x140acba38 <.text+0xacaa38>
140acb8f0: 48 89 bd 00 03 00 00        	movq	%rdi, 0x300(%rbp)
140acb8f7: 4c 89 b5 f8 02 00 00        	movq	%r14, 0x2f8(%rbp)
140acb8fe: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
140acb905: 48 89 85 f0 02 00 00        	movq	%rax, 0x2f0(%rbp)
140acb90c: 48 8b 85 a0 02 00 00        	movq	0x2a0(%rbp), %rax
140acb913: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
140acb91a: c6 85 45 03 00 00 01        	movb	$0x1, 0x345(%rbp)
140acb921: c6 85 44 03 00 00 01        	movb	$0x1, 0x344(%rbp)
140acb928: b9 08 00 00 00              	movl	$0x8, %ecx
140acb92d: ba 10 00 00 00              	movl	$0x10, %edx
140acb932: e8 c9 d6 e0 01              	callq	0x1428d9000 <.text+0x28d8000>
140acb937: e9 fc 00 00 00              	jmp	0x140acba38 <.text+0xacaa38>
140acb93c: 45 31 ed                    	xorl	%r13d, %r13d
140acb93f: 4c 8d 05 32 d2 16 02        	leaq	0x216d232(%rip), %r8    # 0x142c38b78
140acb946: 4c 89 e9                    	movq	%r13, %rcx
140acb949: 4c 89 f2                    	movq	%r14, %rdx
140acb94c: e8 92 d6 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acb951: e9 e2 00 00 00              	jmp	0x140acba38 <.text+0xacaa38>
140acb956: 31 ff                       	xorl	%edi, %edi
140acb958: 4c 8d 05 19 d2 16 02        	leaq	0x216d219(%rip), %r8    # 0x142c38b78
140acb95f: 48 89 f9                    	movq	%rdi, %rcx
140acb962: 48 89 da                    	movq	%rbx, %rdx
140acb965: e8 79 d6 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acb96a: 45 31 ff                    	xorl	%r15d, %r15d
140acb96d: 4c 8d 05 04 d2 16 02        	leaq	0x216d204(%rip), %r8    # 0x142c38b78
140acb974: 4c 89 f9                    	movq	%r15, %rcx
140acb977: 48 89 da                    	movq	%rbx, %rdx
140acb97a: e8 64 d6 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acb97f: e9 b4 00 00 00              	jmp	0x140acba38 <.text+0xacaa38>
140acb984: 45 31 f6                    	xorl	%r14d, %r14d
140acb987: 4c 8d 05 ea d1 16 02        	leaq	0x216d1ea(%rip), %r8    # 0x142c38b78
140acb98e: 4c 89 f1                    	movq	%r14, %rcx
140acb991: 48 89 da                    	movq	%rbx, %rdx
140acb994: e8 4a d6 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acb999: e9 9a 00 00 00              	jmp	0x140acba38 <.text+0xacaa38>
140acb99e: 48 89 bd 00 03 00 00        	movq	%rdi, 0x300(%rbp)
140acb9a5: 4c 89 b5 f8 02 00 00        	movq	%r14, 0x2f8(%rbp)
140acb9ac: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
140acb9b3: 48 89 85 f0 02 00 00        	movq	%rax, 0x2f0(%rbp)
140acb9ba: 48 8b 85 a0 02 00 00        	movq	0x2a0(%rbp), %rax
140acb9c1: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
140acb9c8: c6 85 45 03 00 00 01        	movb	$0x1, 0x345(%rbp)
140acb9cf: c6 85 44 03 00 00 01        	movb	$0x1, 0x344(%rbp)
140acb9d6: b9 08 00 00 00              	movl	$0x8, %ecx
140acb9db: ba 08 00 00 00              	movl	$0x8, %edx
140acb9e0: e8 1b d6 e0 01              	callq	0x1428d9000 <.text+0x28d8000>
140acb9e5: eb 51                       	jmp	0x140acba38 <.text+0xacaa38>
140acb9e7: 45 31 e4                    	xorl	%r12d, %r12d
140acb9ea: 4c 8d 05 87 d1 16 02        	leaq	0x216d187(%rip), %r8    # 0x142c38b78
140acb9f1: 4c 89 e1                    	movq	%r12, %rcx
140acb9f4: 4c 89 f2                    	movq	%r14, %rdx
140acb9f7: e8 e7 d5 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acb9fc: eb 3a                       	jmp	0x140acba38 <.text+0xacaa38>
140acb9fe: 45 31 f6                    	xorl	%r14d, %r14d
140acba01: c6 85 47 03 00 00 01        	movb	$0x1, 0x347(%rbp)
140acba08: 4c 8d 05 69 d1 16 02        	leaq	0x216d169(%rip), %r8    # 0x142c38b78
140acba0f: 4c 89 f1                    	movq	%r14, %rcx
140acba12: 48 89 da                    	movq	%rbx, %rdx
140acba15: e8 c9 d5 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acba1a: eb 1c                       	jmp	0x140acba38 <.text+0xacaa38>
140acba1c: 45 31 f6                    	xorl	%r14d, %r14d
140acba1f: c6 85 47 03 00 00 01        	movb	$0x1, 0x347(%rbp)
140acba26: 4c 8d 05 4b d1 16 02        	leaq	0x216d14b(%rip), %r8    # 0x142c38b78
140acba2d: 4c 89 f1                    	movq	%r14, %rcx
140acba30: 4c 89 fa                    	movq	%r15, %rdx
140acba33: e8 ab d5 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acba38: 0f 0b                       	ud2
140acba3a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140acba40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acba45: 55                          	pushq	%rbp
140acba46: 41 57                       	pushq	%r15
140acba48: 41 56                       	pushq	%r14
140acba4a: 41 55                       	pushq	%r13
140acba4c: 41 54                       	pushq	%r12
140acba4e: 56                          	pushq	%rsi
140acba4f: 57                          	pushq	%rdi
140acba50: 53                          	pushq	%rbx
140acba51: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acba58: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acba5f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acba65: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acba6b: 66 44 0f 7f 6c 24 50        	movdqa	%xmm13, 0x50(%rsp)
140acba72: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acba79: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acba80: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acba89: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acba93: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acba9d: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acbaa5: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acbaad: 48 83 bd 30 03 00 00 00     	cmpq	$0x0, 0x330(%rbp)
140acbab5: 74 19                       	je	0x140acbad0 <.text+0xacaad0>
140acbab7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140acbabd: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
140acbac4: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
140acbacb: e8 50 81 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acbad0: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acbad8: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acbae0: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140acbae9: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140acbaf2: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acbafb: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140acbb01: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140acbb07: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140acbb0d: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acbb13: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acbb19: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acbb20: 5b                          	popq	%rbx
140acbb21: 5f                          	popq	%rdi
140acbb22: 5e                          	popq	%rsi
140acbb23: 41 5c                       	popq	%r12
140acbb25: 41 5d                       	popq	%r13
140acbb27: 41 5e                       	popq	%r14
140acbb29: 41 5f                       	popq	%r15
140acbb2b: 5d                          	popq	%rbp
140acbb2c: c3                          	retq
140acbb2d: 0f 1f 00                    	nopl	(%rax)
140acbb30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acbb35: 55                          	pushq	%rbp
140acbb36: 41 57                       	pushq	%r15
140acbb38: 41 56                       	pushq	%r14
140acbb3a: 41 55                       	pushq	%r13
140acbb3c: 41 54                       	pushq	%r12
140acbb3e: 56                          	pushq	%rsi
140acbb3f: 57                          	pushq	%rdi
140acbb40: 53                          	pushq	%rbx
140acbb41: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acbb48: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acbb4f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acbb55: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acbb5b: 66 44 0f 7f 6c 24 50        	movdqa	%xmm13, 0x50(%rsp)
140acbb62: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acbb69: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acbb70: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acbb79: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acbb83: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acbb8d: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acbb95: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acbb9d: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
140acbba4: f0                          	lock
140acbba5: 48 ff 08                    	decq	(%rax)
140acbba8: 75 0c                       	jne	0x140acbbb6 <.text+0xacabb6>
140acbbaa: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140acbbb1: e8 1a 3e b8 ff              	callq	0x14064f9d0 <.text+0x64e9d0>
140acbbb6: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acbbbe: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acbbc6: 66 44 0f 6f 84 24 a0 00 00 00       	movdqa	0xa0(%rsp), %xmm8
140acbbd0: 66 44 0f 6f 8c 24 90 00 00 00       	movdqa	0x90(%rsp), %xmm9
140acbbda: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acbbe3: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
140acbbea: 66 44 0f 6f 64 24 60        	movdqa	0x60(%rsp), %xmm12
140acbbf1: 66 44 0f 6f 6c 24 50        	movdqa	0x50(%rsp), %xmm13
140acbbf8: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acbbfe: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acbc04: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acbc0b: 5b                          	popq	%rbx
140acbc0c: 5f                          	popq	%rdi
140acbc0d: 5e                          	popq	%rsi
140acbc0e: 41 5c                       	popq	%r12
140acbc10: 41 5d                       	popq	%r13
140acbc12: 41 5e                       	popq	%r14
140acbc14: 41 5f                       	popq	%r15
140acbc16: 5d                          	popq	%rbp
140acbc17: c3                          	retq
140acbc18: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140acbc20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acbc25: 55                          	pushq	%rbp
140acbc26: 41 57                       	pushq	%r15
140acbc28: 41 56                       	pushq	%r14
140acbc2a: 41 55                       	pushq	%r13
140acbc2c: 41 54                       	pushq	%r12
140acbc2e: 56                          	pushq	%rsi
140acbc2f: 57                          	pushq	%rdi
140acbc30: 53                          	pushq	%rbx
140acbc31: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acbc38: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acbc3f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acbc45: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acbc4b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140acbc51: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acbc58: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acbc5f: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acbc68: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acbc72: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acbc7c: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acbc84: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acbc8c: 48 83 bd 38 03 00 00 00     	cmpq	$0x0, 0x338(%rbp)
140acbc94: 74 1d                       	je	0x140acbcb3 <.text+0xacacb3>
140acbc96: 48 8b 95 38 03 00 00        	movq	0x338(%rbp), %rdx
140acbc9d: 48 c1 e2 03                 	shlq	$0x3, %rdx
140acbca1: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acbca7: 48 8b 8d 30 03 00 00        	movq	0x330(%rbp), %rcx
140acbcae: e8 6d 7f b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acbcb3: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acbcbb: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acbcc3: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140acbccc: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140acbcd5: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acbcde: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140acbce4: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140acbcea: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140acbcf0: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acbcf6: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acbcfc: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acbd03: 5b                          	popq	%rbx
140acbd04: 5f                          	popq	%rdi
140acbd05: 5e                          	popq	%rsi
140acbd06: 41 5c                       	popq	%r12
140acbd08: 41 5d                       	popq	%r13
140acbd0a: 41 5e                       	popq	%r14
140acbd0c: 41 5f                       	popq	%r15
140acbd0e: 5d                          	popq	%rbp
140acbd0f: c3                          	retq
140acbd10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acbd15: 55                          	pushq	%rbp
140acbd16: 41 57                       	pushq	%r15
140acbd18: 41 56                       	pushq	%r14
140acbd1a: 41 55                       	pushq	%r13
140acbd1c: 41 54                       	pushq	%r12
140acbd1e: 56                          	pushq	%rsi
140acbd1f: 57                          	pushq	%rdi
140acbd20: 53                          	pushq	%rbx
140acbd21: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acbd28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acbd2f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acbd35: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acbd3b: 66 44 0f 7f 6c 24 50        	movdqa	%xmm13, 0x50(%rsp)
140acbd42: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acbd49: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acbd50: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acbd59: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acbd63: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acbd6d: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acbd75: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acbd7d: 48 83 bd 38 03 00 00 00     	cmpq	$0x0, 0x338(%rbp)
140acbd85: 74 21                       	je	0x140acbda8 <.text+0xacada8>
140acbd87: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
140acbd8e: 48 c1 e0 02                 	shlq	$0x2, %rax
140acbd92: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140acbd96: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acbd9c: 48 8b 8d 30 03 00 00        	movq	0x330(%rbp), %rcx
140acbda3: e8 78 7e b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acbda8: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acbdb0: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acbdb8: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140acbdc1: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140acbdca: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acbdd3: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140acbdd9: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140acbddf: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140acbde5: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acbdeb: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acbdf1: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acbdf8: 5b                          	popq	%rbx
140acbdf9: 5f                          	popq	%rdi
140acbdfa: 5e                          	popq	%rsi
140acbdfb: 41 5c                       	popq	%r12
140acbdfd: 41 5d                       	popq	%r13
140acbdff: 41 5e                       	popq	%r14
140acbe01: 41 5f                       	popq	%r15
140acbe03: 5d                          	popq	%rbp
140acbe04: c3                          	retq
140acbe05: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140acbe10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acbe15: 55                          	pushq	%rbp
140acbe16: 41 57                       	pushq	%r15
140acbe18: 41 56                       	pushq	%r14
140acbe1a: 41 55                       	pushq	%r13
140acbe1c: 41 54                       	pushq	%r12
140acbe1e: 56                          	pushq	%rsi
140acbe1f: 57                          	pushq	%rdi
140acbe20: 53                          	pushq	%rbx
140acbe21: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acbe28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acbe2f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acbe35: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acbe3b: 66 44 0f 7f 6c 24 50        	movdqa	%xmm13, 0x50(%rsp)
140acbe42: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acbe49: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acbe50: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acbe59: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acbe63: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acbe6d: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acbe75: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acbe7d: 48 83 bd 38 03 00 00 00     	cmpq	$0x0, 0x338(%rbp)
140acbe85: 74 1d                       	je	0x140acbea4 <.text+0xacaea4>
140acbe87: 48 8b 95 38 03 00 00        	movq	0x338(%rbp), %rdx
140acbe8e: 48 c1 e2 03                 	shlq	$0x3, %rdx
140acbe92: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acbe98: 48 8b 8d 30 03 00 00        	movq	0x330(%rbp), %rcx
140acbe9f: e8 7c 7d b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acbea4: c6 85 46 03 00 00 00        	movb	$0x0, 0x346(%rbp)
140acbeab: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acbeb3: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acbebb: 66 44 0f 6f 84 24 a0 00 00 00       	movdqa	0xa0(%rsp), %xmm8
140acbec5: 66 44 0f 6f 8c 24 90 00 00 00       	movdqa	0x90(%rsp), %xmm9
140acbecf: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acbed8: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
140acbedf: 66 44 0f 6f 64 24 60        	movdqa	0x60(%rsp), %xmm12
140acbee6: 66 44 0f 6f 6c 24 50        	movdqa	0x50(%rsp), %xmm13
140acbeed: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acbef3: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acbef9: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acbf00: 5b                          	popq	%rbx
140acbf01: 5f                          	popq	%rdi
140acbf02: 5e                          	popq	%rsi
140acbf03: 41 5c                       	popq	%r12
140acbf05: 41 5d                       	popq	%r13
140acbf07: 41 5e                       	popq	%r14
140acbf09: 41 5f                       	popq	%r15
140acbf0b: 5d                          	popq	%rbp
140acbf0c: c3                          	retq
140acbf0d: 0f 1f 00                    	nopl	(%rax)
140acbf10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acbf15: 55                          	pushq	%rbp
140acbf16: 41 57                       	pushq	%r15
140acbf18: 41 56                       	pushq	%r14
140acbf1a: 41 55                       	pushq	%r13
140acbf1c: 41 54                       	pushq	%r12
140acbf1e: 56                          	pushq	%rsi
140acbf1f: 57                          	pushq	%rdi
140acbf20: 53                          	pushq	%rbx
140acbf21: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acbf28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acbf2f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acbf35: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acbf3b: 66 44 0f 7f 6c 24 50        	movdqa	%xmm13, 0x50(%rsp)
140acbf42: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acbf49: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acbf50: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acbf59: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acbf63: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acbf6d: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acbf75: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acbf7d: 80 bd 46 03 00 00 00        	cmpb	$0x0, 0x346(%rbp)
140acbf84: 74 26                       	je	0x140acbfac <.text+0xacafac>
140acbf86: 48 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %rax
140acbf8d: 48 85 c0                    	testq	%rax, %rax
140acbf90: 74 1a                       	je	0x140acbfac <.text+0xacafac>
140acbf92: 48 8b 8d b8 02 00 00        	movq	0x2b8(%rbp), %rcx
140acbf99: 48 c1 e0 02                 	shlq	$0x2, %rax
140acbf9d: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140acbfa1: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acbfa7: e8 74 7c b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acbfac: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acbfb4: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acbfbc: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140acbfc5: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140acbfce: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acbfd7: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140acbfdd: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140acbfe3: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140acbfe9: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acbfef: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acbff5: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acbffc: 5b                          	popq	%rbx
140acbffd: 5f                          	popq	%rdi
140acbffe: 5e                          	popq	%rsi
140acbfff: 41 5c                       	popq	%r12
140acc001: 41 5d                       	popq	%r13
140acc003: 41 5e                       	popq	%r14
140acc005: 41 5f                       	popq	%r15
140acc007: 5d                          	popq	%rbp
140acc008: c3                          	retq
140acc009: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140acc010: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acc015: 55                          	pushq	%rbp
140acc016: 41 57                       	pushq	%r15
140acc018: 41 56                       	pushq	%r14
140acc01a: 41 55                       	pushq	%r13
140acc01c: 41 54                       	pushq	%r12
140acc01e: 56                          	pushq	%rsi
140acc01f: 57                          	pushq	%rdi
140acc020: 53                          	pushq	%rbx
140acc021: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acc028: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acc02f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acc035: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acc03b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140acc041: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acc048: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acc04f: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acc058: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acc062: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acc06c: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acc074: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acc07c: 0f b6 9d 45 03 00 00        	movzbl	0x345(%rbp), %ebx
140acc083: 48 8b b5 e8 02 00 00        	movq	0x2e8(%rbp), %rsi
140acc08a: 48 8b bd f0 02 00 00        	movq	0x2f0(%rbp), %rdi
140acc091: 48 8b 95 00 03 00 00        	movq	0x300(%rbp), %rdx
140acc098: 48 85 d2                    	testq	%rdx, %rdx
140acc09b: 74 1f                       	je	0x140acc0bc <.text+0xacb0bc>
140acc09d: f6 85 44 03 00 00 01        	testb	$0x1, 0x344(%rbp)
140acc0a4: 74 16                       	je	0x140acc0bc <.text+0xacb0bc>
140acc0a6: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
140acc0ad: 48 c1 e2 03                 	shlq	$0x3, %rdx
140acc0b1: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acc0b7: e8 64 7b b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acc0bc: f6 c3 01                    	testb	$0x1, %bl
140acc0bf: 74 1a                       	je	0x140acc0db <.text+0xacb0db>
140acc0c1: 48 85 ff                    	testq	%rdi, %rdi
140acc0c4: 74 15                       	je	0x140acc0db <.text+0xacb0db>
140acc0c6: 48 c1 e7 02                 	shlq	$0x2, %rdi
140acc0ca: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acc0d0: 48 89 f1                    	movq	%rsi, %rcx
140acc0d3: 48 89 fa                    	movq	%rdi, %rdx
140acc0d6: e8 45 7b b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acc0db: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acc0e3: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acc0eb: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140acc0f4: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140acc0fd: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acc106: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140acc10c: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140acc112: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140acc118: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acc11e: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acc124: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acc12b: 5b                          	popq	%rbx
140acc12c: 5f                          	popq	%rdi
140acc12d: 5e                          	popq	%rsi
140acc12e: 41 5c                       	popq	%r12
140acc130: 41 5d                       	popq	%r13
140acc132: 41 5e                       	popq	%r14
140acc134: 41 5f                       	popq	%r15
140acc136: 5d                          	popq	%rbp
140acc137: c3                          	retq
140acc138: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140acc140: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acc145: 55                          	pushq	%rbp
140acc146: 41 57                       	pushq	%r15
140acc148: 41 56                       	pushq	%r14
140acc14a: 41 55                       	pushq	%r13
140acc14c: 41 54                       	pushq	%r12
140acc14e: 56                          	pushq	%rsi
140acc14f: 57                          	pushq	%rdi
140acc150: 53                          	pushq	%rbx
140acc151: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acc158: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acc15f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acc165: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acc16b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140acc171: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acc178: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acc17f: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acc188: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acc192: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acc19c: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acc1a4: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acc1ac: 0f b6 9d 47 03 00 00        	movzbl	0x347(%rbp), %ebx
140acc1b3: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140acc1b9: 48 8b 8d 90 02 00 00        	movq	0x290(%rbp), %rcx
140acc1c0: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
140acc1c7: e8 54 7a b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acc1cc: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
140acc1d3: 48 89 85 00 03 00 00        	movq	%rax, 0x300(%rbp)
140acc1da: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
140acc1e1: 48 89 85 f8 02 00 00        	movq	%rax, 0x2f8(%rbp)
140acc1e8: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
140acc1ef: 48 89 85 f0 02 00 00        	movq	%rax, 0x2f0(%rbp)
140acc1f6: 48 8b 85 a0 02 00 00        	movq	0x2a0(%rbp), %rax
140acc1fd: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
140acc204: 88 9d 45 03 00 00           	movb	%bl, 0x345(%rbp)
140acc20a: 88 9d 44 03 00 00           	movb	%bl, 0x344(%rbp)
140acc210: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acc218: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acc220: 66 44 0f 6f 84 24 a0 00 00 00       	movdqa	0xa0(%rsp), %xmm8
140acc22a: 66 44 0f 6f 8c 24 90 00 00 00       	movdqa	0x90(%rsp), %xmm9
140acc234: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acc23d: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
140acc244: 66 44 0f 6f 64 24 60        	movdqa	0x60(%rsp), %xmm12
140acc24b: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140acc251: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acc257: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acc25d: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acc264: 5b                          	popq	%rbx
140acc265: 5f                          	popq	%rdi
140acc266: 5e                          	popq	%rsi
140acc267: 41 5c                       	popq	%r12
140acc269: 41 5d                       	popq	%r13
140acc26b: 41 5e                       	popq	%r14
140acc26d: 41 5f                       	popq	%r15
140acc26f: 5d                          	popq	%rbp
140acc270: c3                          	retq
140acc271: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140acc280: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acc285: 55                          	pushq	%rbp
140acc286: 41 57                       	pushq	%r15
140acc288: 41 56                       	pushq	%r14
140acc28a: 41 55                       	pushq	%r13
140acc28c: 41 54                       	pushq	%r12
140acc28e: 56                          	pushq	%rsi
140acc28f: 57                          	pushq	%rdi
140acc290: 53                          	pushq	%rbx
140acc291: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acc298: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acc29f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acc2a5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acc2ab: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140acc2b1: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acc2b8: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acc2bf: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acc2c8: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acc2d2: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acc2dc: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acc2e4: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acc2ec: 48 83 bd 18 03 00 00 00     	cmpq	$0x0, 0x318(%rbp)
140acc2f4: 74 21                       	je	0x140acc317 <.text+0xacb317>
140acc2f6: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
140acc2fd: 48 c1 e0 02                 	shlq	$0x2, %rax
140acc301: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140acc305: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acc30b: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
140acc312: e8 09 79 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acc317: c6 85 47 03 00 00 01        	movb	$0x1, 0x347(%rbp)
140acc31e: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acc326: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acc32e: 66 44 0f 6f 84 24 a0 00 00 00       	movdqa	0xa0(%rsp), %xmm8
140acc338: 66 44 0f 6f 8c 24 90 00 00 00       	movdqa	0x90(%rsp), %xmm9
140acc342: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acc34b: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
140acc352: 66 44 0f 6f 64 24 60        	movdqa	0x60(%rsp), %xmm12
140acc359: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140acc35f: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acc365: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acc36b: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acc372: 5b                          	popq	%rbx
140acc373: 5f                          	popq	%rdi
140acc374: 5e                          	popq	%rsi
140acc375: 41 5c                       	popq	%r12
140acc377: 41 5d                       	popq	%r13
140acc379: 41 5e                       	popq	%r14
140acc37b: 41 5f                       	popq	%r15
140acc37d: 5d                          	popq	%rbp
140acc37e: c3                          	retq
140acc37f: 90                          	nop
140acc380: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acc385: 55                          	pushq	%rbp
140acc386: 41 57                       	pushq	%r15
140acc388: 41 56                       	pushq	%r14
140acc38a: 41 55                       	pushq	%r13
140acc38c: 41 54                       	pushq	%r12
140acc38e: 56                          	pushq	%rsi
140acc38f: 57                          	pushq	%rdi
140acc390: 53                          	pushq	%rbx
140acc391: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acc398: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acc39f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acc3a5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acc3ab: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140acc3b1: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acc3b8: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acc3bf: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acc3c8: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acc3d2: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acc3dc: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acc3e4: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acc3ec: 48 83 bd 18 03 00 00 00     	cmpq	$0x0, 0x318(%rbp)
140acc3f4: 74 1d                       	je	0x140acc413 <.text+0xacb413>
140acc3f6: 48 8b 95 18 03 00 00        	movq	0x318(%rbp), %rdx
140acc3fd: 48 c1 e2 03                 	shlq	$0x3, %rdx
140acc401: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acc407: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
140acc40e: e8 0d 78 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acc413: c6 85 47 03 00 00 01        	movb	$0x1, 0x347(%rbp)
140acc41a: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acc422: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acc42a: 66 44 0f 6f 84 24 a0 00 00 00       	movdqa	0xa0(%rsp), %xmm8
140acc434: 66 44 0f 6f 8c 24 90 00 00 00       	movdqa	0x90(%rsp), %xmm9
140acc43e: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acc447: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
140acc44e: 66 44 0f 6f 64 24 60        	movdqa	0x60(%rsp), %xmm12
140acc455: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140acc45b: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acc461: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acc467: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acc46e: 5b                          	popq	%rbx
140acc46f: 5f                          	popq	%rdi
140acc470: 5e                          	popq	%rsi
140acc471: 41 5c                       	popq	%r12
140acc473: 41 5d                       	popq	%r13
140acc475: 41 5e                       	popq	%r14
140acc477: 41 5f                       	popq	%r15
140acc479: 5d                          	popq	%rbp
140acc47a: c3                          	retq
140acc47b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140acc480: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acc485: 55                          	pushq	%rbp
140acc486: 41 57                       	pushq	%r15
140acc488: 41 56                       	pushq	%r14
140acc48a: 41 55                       	pushq	%r13
140acc48c: 41 54                       	pushq	%r12
140acc48e: 56                          	pushq	%rsi
140acc48f: 57                          	pushq	%rdi
140acc490: 53                          	pushq	%rbx
140acc491: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acc498: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acc49f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acc4a5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acc4ab: 66 44 0f 7f 6c 24 50        	movdqa	%xmm13, 0x50(%rsp)
140acc4b2: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acc4b9: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acc4c0: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acc4c9: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acc4d3: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acc4dd: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acc4e5: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acc4ed: 48 83 bd 38 03 00 00 00     	cmpq	$0x0, 0x338(%rbp)
140acc4f5: 74 1d                       	je	0x140acc514 <.text+0xacb514>
140acc4f7: 48 8b 95 38 03 00 00        	movq	0x338(%rbp), %rdx
140acc4fe: 48 c1 e2 03                 	shlq	$0x3, %rdx
140acc502: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140acc508: 48 8b 8d 30 03 00 00        	movq	0x330(%rbp), %rcx
140acc50f: e8 0c 77 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acc514: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acc51c: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acc524: 66 44 0f 6f 84 24 a0 00 00 00       	movdqa	0xa0(%rsp), %xmm8
140acc52e: 66 44 0f 6f 8c 24 90 00 00 00       	movdqa	0x90(%rsp), %xmm9
140acc538: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acc541: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
140acc548: 66 44 0f 6f 64 24 60        	movdqa	0x60(%rsp), %xmm12
140acc54f: 66 44 0f 6f 6c 24 50        	movdqa	0x50(%rsp), %xmm13
140acc556: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acc55c: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acc562: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acc569: 5b                          	popq	%rbx
140acc56a: 5f                          	popq	%rdi
140acc56b: 5e                          	popq	%rsi
140acc56c: 41 5c                       	popq	%r12
140acc56e: 41 5d                       	popq	%r13
140acc570: 41 5e                       	popq	%r14
140acc572: 41 5f                       	popq	%r15
140acc574: 5d                          	popq	%rbp
140acc575: c3                          	retq
140acc576: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140acc580: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acc585: 55                          	pushq	%rbp
140acc586: 41 57                       	pushq	%r15
140acc588: 41 56                       	pushq	%r14
140acc58a: 41 55                       	pushq	%r13
140acc58c: 41 54                       	pushq	%r12
140acc58e: 56                          	pushq	%rsi
140acc58f: 57                          	pushq	%rdi
140acc590: 53                          	pushq	%rbx
140acc591: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acc598: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acc59f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acc5a5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acc5ab: 66 44 0f 7f 6c 24 50        	movdqa	%xmm13, 0x50(%rsp)
140acc5b2: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acc5b9: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acc5c0: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acc5c9: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acc5d3: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acc5dd: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acc5e5: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acc5ed: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140acc5f4: e8 f7 5e b0 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
140acc5f9: c6 85 43 03 00 00 01        	movb	$0x1, 0x343(%rbp)
140acc600: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acc608: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acc610: 66 44 0f 6f 84 24 a0 00 00 00       	movdqa	0xa0(%rsp), %xmm8
140acc61a: 66 44 0f 6f 8c 24 90 00 00 00       	movdqa	0x90(%rsp), %xmm9
140acc624: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acc62d: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
140acc634: 66 44 0f 6f 64 24 60        	movdqa	0x60(%rsp), %xmm12
140acc63b: 66 44 0f 6f 6c 24 50        	movdqa	0x50(%rsp), %xmm13
140acc642: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acc648: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acc64e: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acc655: 5b                          	popq	%rbx
140acc656: 5f                          	popq	%rdi
140acc657: 5e                          	popq	%rsi
140acc658: 41 5c                       	popq	%r12
140acc65a: 41 5d                       	popq	%r13
140acc65c: 41 5e                       	popq	%r14
140acc65e: 41 5f                       	popq	%r15
140acc660: 5d                          	popq	%rbp
140acc661: c3                          	retq
140acc662: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140acc670: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acc675: 55                          	pushq	%rbp
140acc676: 41 57                       	pushq	%r15
140acc678: 41 56                       	pushq	%r14
140acc67a: 41 55                       	pushq	%r13
140acc67c: 41 54                       	pushq	%r12
140acc67e: 56                          	pushq	%rsi
140acc67f: 57                          	pushq	%rdi
140acc680: 53                          	pushq	%rbx
140acc681: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acc688: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acc68f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acc695: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acc69b: 66 44 0f 7f 6c 24 50        	movdqa	%xmm13, 0x50(%rsp)
140acc6a2: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acc6a9: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acc6b0: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acc6b9: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acc6c3: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acc6cd: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acc6d5: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acc6dd: c6 85 43 03 00 00 00        	movb	$0x0, 0x343(%rbp)
140acc6e4: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140acc6eb: e8 80 bd dc ff              	callq	0x140898470 <.text+0x897470>
140acc6f0: c6 85 43 03 00 00 00        	movb	$0x0, 0x343(%rbp)
140acc6f7: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acc6ff: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acc707: 66 44 0f 6f 84 24 a0 00 00 00       	movdqa	0xa0(%rsp), %xmm8
140acc711: 66 44 0f 6f 8c 24 90 00 00 00       	movdqa	0x90(%rsp), %xmm9
140acc71b: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acc724: 66 44 0f 6f 5c 24 70        	movdqa	0x70(%rsp), %xmm11
140acc72b: 66 44 0f 6f 64 24 60        	movdqa	0x60(%rsp), %xmm12
140acc732: 66 44 0f 6f 6c 24 50        	movdqa	0x50(%rsp), %xmm13
140acc739: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acc73f: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acc745: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acc74c: 5b                          	popq	%rbx
140acc74d: 5f                          	popq	%rdi
140acc74e: 5e                          	popq	%rsi
140acc74f: 41 5c                       	popq	%r12
140acc751: 41 5d                       	popq	%r13
140acc753: 41 5e                       	popq	%r14
140acc755: 41 5f                       	popq	%r15
140acc757: 5d                          	popq	%rbp
140acc758: c3                          	retq
140acc759: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140acc760: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acc765: 55                          	pushq	%rbp
140acc766: 41 57                       	pushq	%r15
140acc768: 41 56                       	pushq	%r14
140acc76a: 41 55                       	pushq	%r13
140acc76c: 41 54                       	pushq	%r12
140acc76e: 56                          	pushq	%rsi
140acc76f: 57                          	pushq	%rdi
140acc770: 53                          	pushq	%rbx
140acc771: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140acc778: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acc77f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140acc785: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140acc78b: 66 44 0f 7f 6c 24 50        	movdqa	%xmm13, 0x50(%rsp)
140acc792: 66 44 0f 7f 64 24 60        	movdqa	%xmm12, 0x60(%rsp)
140acc799: 66 44 0f 7f 5c 24 70        	movdqa	%xmm11, 0x70(%rsp)
140acc7a0: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140acc7a9: 66 44 0f 7f 8c 24 90 00 00 00       	movdqa	%xmm9, 0x90(%rsp)
140acc7b3: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
140acc7bd: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140acc7c5: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140acc7cd: 80 bd 43 03 00 00 00        	cmpb	$0x0, 0x343(%rbp)
140acc7d4: 75 5d                       	jne	0x140acc833 <.text+0xacb833>
140acc7d6: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140acc7de: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140acc7e6: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140acc7ef: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140acc7f8: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140acc801: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140acc807: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140acc80d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140acc813: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140acc819: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140acc81f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140acc826: 5b                          	popq	%rbx
140acc827: 5f                          	popq	%rdi
140acc828: 5e                          	popq	%rsi
140acc829: 41 5c                       	popq	%r12
140acc82b: 41 5d                       	popq	%r13
140acc82d: 41 5e                       	popq	%r14
140acc82f: 41 5f                       	popq	%r15
140acc831: 5d                          	popq	%rbp
140acc832: c3                          	retq
140acc833: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
140acc83a: e8 31 bc dc ff              	callq	0x140898470 <.text+0x897470>
140acc83f: eb 95                       	jmp	0x140acc7d6 <.text+0xacb7d6>
140acc841: cc                          	int3
140acc842: cc                          	int3
140acc843: cc                          	int3
140acc844: cc                          	int3
140acc845: cc                          	int3
140acc846: cc                          	int3
140acc847: cc                          	int3
140acc848: cc                          	int3
140acc849: cc                          	int3
140acc84a: cc                          	int3
140acc84b: cc                          	int3
140acc84c: cc                          	int3
140acc84d: cc                          	int3
140acc84e: cc                          	int3
140acc84f: cc                          	int3
140acc850: 55                          	pushq	%rbp
140acc851: 41 57                       	pushq	%r15
140acc853: 41 56                       	pushq	%r14
140acc855: 56                          	pushq	%rsi
140acc856: 57                          	pushq	%rdi
140acc857: 53                          	pushq	%rbx
140acc858: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
140acc85f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140acc867: 48 c7 85 80 00 00 00 fe ff ff ff    	movq	$-0x2, 0x80(%rbp)
140acc872: 4c 89 cf                    	movq	%r9, %rdi
140acc875: 4c 89 c3                    	movq	%r8, %rbx
140acc878: 49 89 d6                    	movq	%rdx, %r14
140acc87b: 48 89 ce                    	movq	%rcx, %rsi
140acc87e: 0f b6 05 84 83 b6 02        	movzbl	0x2b68384(%rip), %eax   # 0x143634c09
140acc885: b9 08 00 00 00              	movl	$0x8, %ecx
140acc88a: ba 08 00 00 00              	movl	$0x8, %edx
140acc88f: e8 7c 73 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acc894: 48 85 c0                    	testq	%rax, %rax
140acc897: 0f 84 61 01 00 00           	je	0x140acc9fe <.text+0xacb9fe>
140acc89d: 49 89 c7                    	movq	%rax, %r15
140acc8a0: 0f b6 05 62 83 b6 02        	movzbl	0x2b68362(%rip), %eax   # 0x143634c09
140acc8a7: b9 0b 00 00 00              	movl	$0xb, %ecx
140acc8ac: ba 01 00 00 00              	movl	$0x1, %edx
140acc8b1: e8 5a 73 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acc8b6: 48 85 c0                    	testq	%rax, %rax
140acc8b9: 4c 89 7d 78                 	movq	%r15, 0x78(%rbp)
140acc8bd: 0f 84 4a 01 00 00           	je	0x140acca0d <.text+0xacba0d>
140acc8c3: 48 b9 74 65 6d 70 5f 73 70 68       	movabsq	$0x6870735f706d6574, %rcx # imm = 0x6870735F706D6574
140acc8cd: 48 89 08                    	movq	%rcx, (%rax)
140acc8d0: c7 40 07 68 65 72 65        	movl	$0x65726568, 0x7(%rax)  # imm = 0x65726568
140acc8d7: 48 c7 45 50 0b 00 00 00     	movq	$0xb, 0x50(%rbp)
140acc8df: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
140acc8e3: 48 c7 45 60 0b 00 00 00     	movq	$0xb, 0x60(%rbp)
140acc8eb: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140acc8f5: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140acc8f9: 0f b6 05 09 83 b6 02        	movzbl	0x2b68309(%rip), %eax   # 0x143634c09
140acc900: b9 13 00 00 00              	movl	$0x13, %ecx
140acc905: ba 01 00 00 00              	movl	$0x1, %edx
140acc90a: e8 01 73 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acc90f: 48 85 c0                    	testq	%rax, %rax
140acc912: 0f 84 0d 01 00 00           	je	0x140acca25 <.text+0xacba25>
140acc918: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140acc91f: 8b 15 76 cb fc 01           	movl	0x1fccb76(%rip), %edx   # 0x142a9949b
140acc925: 89 50 0f                    	movl	%edx, 0xf(%rax)
140acc928: 0f 10 05 5d cb fc 01        	movups	0x1fccb5d(%rip), %xmm0  # 0x142a9948c
140acc92f: 0f 11 00                    	movups	%xmm0, (%rax)
140acc932: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
140acc936: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
140acc93a: 0f 10 55 68                 	movups	0x68(%rbp), %xmm2
140acc93e: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
140acc942: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
140acc946: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
140acc94a: f3 41 0f 10 46 0c           	movss	0xc(%r14), %xmm0
140acc950: 0f 28 0d 69 17 e7 01        	movaps	0x1e71769(%rip), %xmm1  # 0x14293e0c0
140acc957: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140acc95b: 66 c7 45 f0 00 01           	movw	$0x100, -0x10(%rbp)     # imm = 0x100
140acc961: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
140acc96b: 48 89 55 f4                 	movq	%rdx, -0xc(%rbp)
140acc96f: c7 45 00 01 00 00 00        	movl	$0x1, (%rbp)
140acc976: f3 0f 11 45 04              	movss	%xmm0, 0x4(%rbp)
140acc97b: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
140acc982: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
140acc98a: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140acc98e: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
140acc996: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
140acc99d: 49 8b 06                    	movq	(%r14), %rax
140acc9a0: 48 89 45 2c                 	movq	%rax, 0x2c(%rbp)
140acc9a4: 41 8b 46 08                 	movl	0x8(%r14), %eax
140acc9a8: 89 45 34                    	movl	%eax, 0x34(%rbp)
140acc9ab: 66 c7 45 38 00 00           	movw	$0x0, 0x38(%rbp)
140acc9b1: c6 45 3a 00                 	movb	$0x0, 0x3a(%rbp)
140acc9b5: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140acc9ba: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
140acc9be: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140acc9c2: 49 89 d8                    	movq	%rbx, %r8
140acc9c5: 49 89 f9                    	movq	%rdi, %r9
140acc9c8: e8 83 13 dd ff              	callq	0x14089dd50 <.text+0x89cd50>
140acc9cd: 48 8b 45 68                 	movq	0x68(%rbp), %rax
140acc9d1: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140acc9d5: 48 89 01                    	movq	%rax, (%rcx)
140acc9d8: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
140acc9df: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
140acc9e3: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
140acc9eb: 48 89 f0                    	movq	%rsi, %rax
140acc9ee: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
140acc9f5: 5b                          	popq	%rbx
140acc9f6: 5f                          	popq	%rdi
140acc9f7: 5e                          	popq	%rsi
140acc9f8: 41 5e                       	popq	%r14
140acc9fa: 41 5f                       	popq	%r15
140acc9fc: 5d                          	popq	%rbp
140acc9fd: c3                          	retq
140acc9fe: b9 08 00 00 00              	movl	$0x8, %ecx
140acca03: ba 08 00 00 00              	movl	$0x8, %edx
140acca08: e8 f3 c5 e0 01              	callq	0x1428d9000 <.text+0x28d8000>
140acca0d: 4c 8d 05 ac 3c 01 02        	leaq	0x2013cac(%rip), %r8    # 0x142ae06c0
140acca14: b9 01 00 00 00              	movl	$0x1, %ecx
140acca19: ba 0b 00 00 00              	movl	$0xb, %edx
140acca1e: e8 c0 c5 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acca23: eb 16                       	jmp	0x140acca3b <.text+0xacba3b>
140acca25: 4c 8d 05 f4 c4 fc 01        	leaq	0x1fcc4f4(%rip), %r8    # 0x142a98f20
140acca2c: b9 01 00 00 00              	movl	$0x1, %ecx
140acca31: ba 13 00 00 00              	movl	$0x13, %edx
140acca36: e8 a8 c5 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acca3b: 0f 0b                       	ud2
140acca3d: 0f 1f 00                    	nopl	(%rax)
140acca40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acca45: 55                          	pushq	%rbp
140acca46: 41 57                       	pushq	%r15
140acca48: 41 56                       	pushq	%r14
140acca4a: 56                          	pushq	%rsi
140acca4b: 57                          	pushq	%rdi
140acca4c: 53                          	pushq	%rbx
140acca4d: 48 83 ec 28                 	subq	$0x28, %rsp
140acca51: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acca58: ba 08 00 00 00              	movl	$0x8, %edx
140acca5d: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140acca63: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140acca67: e8 b4 71 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140acca6c: 90                          	nop
140acca6d: 48 83 c4 28                 	addq	$0x28, %rsp
140acca71: 5b                          	popq	%rbx
140acca72: 5f                          	popq	%rdi
140acca73: 5e                          	popq	%rsi
140acca74: 41 5e                       	popq	%r14
140acca76: 41 5f                       	popq	%r15
140acca78: 5d                          	popq	%rbp
140acca79: c3                          	retq
140acca7a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140acca80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140acca85: 55                          	pushq	%rbp
140acca86: 41 57                       	pushq	%r15
140acca88: 41 56                       	pushq	%r14
140acca8a: 56                          	pushq	%rsi
140acca8b: 57                          	pushq	%rdi
140acca8c: 53                          	pushq	%rbx
140acca8d: 48 83 ec 28                 	subq	$0x28, %rsp
140acca91: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140acca98: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
140acca9c: e8 ff e2 dc ff              	callq	0x14089ada0 <.text+0x899da0>
140accaa1: 90                          	nop
140accaa2: 48 83 c4 28                 	addq	$0x28, %rsp
140accaa6: 5b                          	popq	%rbx
140accaa7: 5f                          	popq	%rdi
140accaa8: 5e                          	popq	%rsi
140accaa9: 41 5e                       	popq	%r14
140accaab: 41 5f                       	popq	%r15
140accaad: 5d                          	popq	%rbp
140accaae: c3                          	retq
140accaaf: cc                          	int3
