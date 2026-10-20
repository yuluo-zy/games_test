
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b22db0: 55                          	pushq	%rbp
140b22db1: 41 57                       	pushq	%r15
140b22db3: 41 56                       	pushq	%r14
140b22db5: 56                          	pushq	%rsi
140b22db6: 57                          	pushq	%rdi
140b22db7: 53                          	pushq	%rbx
140b22db8: 48 81 ec d8 02 00 00        	subq	$0x2d8, %rsp            # imm = 0x2D8
140b22dbf: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b22dc7: 48 c7 85 50 02 00 00 fe ff ff ff    	movq	$-0x2, 0x250(%rbp)
140b22dd2: 48 89 ce                    	movq	%rcx, %rsi
140b22dd5: 48 c7 85 28 02 00 00 00 00 00 00    	movq	$0x0, 0x228(%rbp)
140b22de0: 48 c7 85 30 02 00 00 08 00 00 00    	movq	$0x8, 0x230(%rbp)
140b22deb: 48 c7 85 38 02 00 00 00 00 00 00    	movq	$0x0, 0x238(%rbp)
140b22df6: 48 8b 39                    	movq	(%rcx), %rdi
140b22df9: 48 83 7f 30 00              	cmpq	$0x0, 0x30(%rdi)
140b22dfe: 48 b8 9f d7 b8 11 29 55 79 e0       	movabsq	$-0x1f86aad6ee472861, %rax # imm = 0xE079552911B8D79F
140b22e08: 48 b9 56 dd 40 97 20 82 c7 7e       	movabsq	$0x7ec782209740dd56, %rcx # imm = 0x7EC782209740DD56
140b22e12: 0f 84 92 00 00 00           	je	0x140b22eaa <.text+0xb21eaa>
140b22e18: 48 8b 57 18                 	movq	0x18(%rdi), %rdx
140b22e1c: 4c 8b 47 20                 	movq	0x20(%rdi), %r8
140b22e20: 4d 89 c1                    	movq	%r8, %r9
140b22e23: 49 21 c1                    	andq	%rax, %r9
140b22e26: 45 31 d2                    	xorl	%r10d, %r10d
140b22e29: 66 0f 6f 05 bf d8 e2 01     	movdqa	0x1e2d8bf(%rip), %xmm0  # 0x1429506f0
140b22e31: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140b22e35: f3 42 0f 6f 14 0a           	movdqu	(%rdx,%r9), %xmm2
140b22e3b: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140b22e3f: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140b22e43: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
140b22e48: 45 85 db                    	testl	%r11d, %r11d
140b22e4b: 75 30                       	jne	0x140b22e7d <.text+0xb21e7d>
140b22e4d: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140b22e51: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
140b22e56: 45 85 db                    	testl	%r11d, %r11d
140b22e59: 75 4f                       	jne	0x140b22eaa <.text+0xb21eaa>
140b22e5b: 4d 01 d1                    	addq	%r10, %r9
140b22e5e: 49 83 c1 10                 	addq	$0x10, %r9
140b22e62: 49 83 c2 10                 	addq	$0x10, %r10
140b22e66: 4d 21 c1                    	andq	%r8, %r9
140b22e69: eb ca                       	jmp	0x140b22e35 <.text+0xb21e35>
140b22e6b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b22e70: 41 8d 5b ff                 	leal	-0x1(%r11), %ebx
140b22e74: 66 44 21 db                 	andw	%r11w, %bx
140b22e78: 41 89 db                    	movl	%ebx, %r11d
140b22e7b: 74 d0                       	je	0x140b22e4d <.text+0xb21e4d>
140b22e7d: f3 41 0f bc db              	tzcntl	%r11d, %ebx
140b22e82: 4c 01 cb                    	addq	%r9, %rbx
140b22e85: 4c 21 c3                    	andq	%r8, %rbx
140b22e88: 48 f7 db                    	negq	%rbx
140b22e8b: 48 8d 1c 5b                 	leaq	(%rbx,%rbx,2), %rbx
140b22e8f: 48 39 4c da e8              	cmpq	%rcx, -0x18(%rdx,%rbx,8)
140b22e94: 75 da                       	jne	0x140b22e70 <.text+0xb21e70>
140b22e96: 48 39 44 da f0              	cmpq	%rax, -0x10(%rdx,%rbx,8)
140b22e9b: 75 d3                       	jne	0x140b22e70 <.text+0xb21e70>
140b22e9d: 48 8d 04 da                 	leaq	(%rdx,%rbx,8), %rax
140b22ea1: 48 8b 58 f8                 	movq	-0x8(%rax), %rbx
140b22ea5: e9 b6 03 00 00              	jmp	0x140b23260 <.text+0xb22260>
140b22eaa: 0f b6 57 60                 	movzbl	0x60(%rdi), %edx
140b22eae: 48 8b 97 80 00 00 00        	movq	0x80(%rdi), %rdx
140b22eb5: 4c 8b 8f 88 00 00 00        	movq	0x88(%rdi), %r9
140b22ebc: 4d 89 ca                    	movq	%r9, %r10
140b22ebf: 49 21 c2                    	andq	%rax, %r10
140b22ec2: 45 31 db                    	xorl	%r11d, %r11d
140b22ec5: 66 0f 6f 05 23 d8 e2 01     	movdqa	0x1e2d823(%rip), %xmm0  # 0x1429506f0
140b22ecd: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140b22ed1: f3 42 0f 6f 14 12           	movdqu	(%rdx,%r10), %xmm2
140b22ed7: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140b22edb: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140b22edf: 66 0f d7 db                 	pmovmskb	%xmm3, %ebx
140b22ee3: 85 db                       	testl	%ebx, %ebx
140b22ee5: 75 36                       	jne	0x140b22f1d <.text+0xb21f1d>
140b22ee7: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140b22eeb: 66 44 0f d7 c2              	pmovmskb	%xmm2, %r8d
140b22ef0: 45 85 c0                    	testl	%r8d, %r8d
140b22ef3: 0f 85 ee 00 00 00           	jne	0x140b22fe7 <.text+0xb21fe7>
140b22ef9: 4d 01 da                    	addq	%r11, %r10
140b22efc: 49 83 c2 10                 	addq	$0x10, %r10
140b22f00: 49 83 c3 10                 	addq	$0x10, %r11
140b22f04: 4d 21 ca                    	andq	%r9, %r10
140b22f07: eb c8                       	jmp	0x140b22ed1 <.text+0xb21ed1>
140b22f09: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140b22f10: 44 8d 43 ff                 	leal	-0x1(%rbx), %r8d
140b22f14: 66 41 21 d8                 	andw	%bx, %r8w
140b22f18: 44 89 c3                    	movl	%r8d, %ebx
140b22f1b: 74 ca                       	je	0x140b22ee7 <.text+0xb21ee7>
140b22f1d: f3 44 0f bc f3              	tzcntl	%ebx, %r14d
140b22f22: 4d 01 d6                    	addq	%r10, %r14
140b22f25: 4d 21 ce                    	andq	%r9, %r14
140b22f28: 4d 89 f0                    	movq	%r14, %r8
140b22f2b: 49 f7 d8                    	negq	%r8
140b22f2e: 4e 8d 3c f5 00 00 00 00     	leaq	(,%r14,8), %r15
140b22f36: 49 c1 e0 07                 	shlq	$0x7, %r8
140b22f3a: 4d 29 f8                    	subq	%r15, %r8
140b22f3d: 4a 39 8c 02 78 ff ff ff     	cmpq	%rcx, -0x88(%rdx,%r8)
140b22f45: 75 c9                       	jne	0x140b22f10 <.text+0xb21f10>
140b22f47: 4a 39 44 02 80              	cmpq	%rax, -0x80(%rdx,%r8)
140b22f4c: 75 c2                       	jne	0x140b22f10 <.text+0xb21f10>
140b22f4e: 4d 89 f2                    	movq	%r14, %r10
140b22f51: 49 c1 e2 07                 	shlq	$0x7, %r10
140b22f55: 4f 8d 1c f2                 	leaq	(%r10,%r14,8), %r11
140b22f59: 49 c1 fb 03                 	sarq	$0x3, %r11
140b22f5d: 49 ba f1 f0 f0 f0 f0 f0 f0 f0       	movabsq	$-0xf0f0f0f0f0f0f0f, %r10 # imm = 0xF0F0F0F0F0F0F0F1
140b22f67: 4d 0f af d3                 	imulq	%r11, %r10
140b22f6b: 4d 8d 5a f0                 	leaq	-0x10(%r10), %r11
140b22f6f: 4d 21 cb                    	andq	%r9, %r11
140b22f72: f3 42 0f 6f 04 1a           	movdqu	(%rdx,%r11), %xmm0
140b22f78: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140b22f7c: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
140b22f80: 66 0f d7 d8                 	pmovmskb	%xmm0, %ebx
140b22f84: f3 42 0f 6f 04 12           	movdqu	(%rdx,%r10), %xmm0
140b22f8a: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
140b22f8e: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
140b22f93: 85 db                       	testl	%ebx, %ebx
140b22f95: 74 09                       	je	0x140b22fa0 <.text+0xb21fa0>
140b22f97: 66 0f bd db                 	bsrw	%bx, %bx
140b22f9b: 83 f3 0f                    	xorl	$0xf, %ebx
140b22f9e: eb 04                       	jmp	0x140b22fa4 <.text+0xb21fa4>
140b22fa0: 66 bb 10 00                 	movw	$0x10, %bx
140b22fa4: 49 01 d0                    	addq	%rdx, %r8
140b22fa7: 41 81 c9 00 00 01 00        	orl	$0x10000, %r9d          # imm = 0x10000
140b22fae: f3 45 0f bc c9              	tzcntl	%r9d, %r9d
140b22fb3: 44 01 cb                    	addl	%r9d, %ebx
140b22fb6: 41 b1 80                    	movb	$-0x80, %r9b
140b22fb9: 66 83 fb 0f                 	cmpw	$0xf, %bx
140b22fbd: 77 0a                       	ja	0x140b22fc9 <.text+0xb21fc9>
140b22fbf: 48 ff 87 90 00 00 00        	incq	0x90(%rdi)
140b22fc6: 41 b1 ff                    	movb	$-0x1, %r9b
140b22fc9: 46 88 0c 12                 	movb	%r9b, (%rdx,%r10)
140b22fcd: 46 88 4c 1a 10              	movb	%r9b, 0x10(%rdx,%r11)
140b22fd2: 48 ff 8f 98 00 00 00        	decq	0x98(%rdi)
140b22fd9: 49 8b 50 88                 	movq	-0x78(%r8), %rdx
140b22fdd: 48 83 fa 02                 	cmpq	$0x2, %rdx
140b22fe1: 0f 85 04 02 00 00           	jne	0x140b231eb <.text+0xb221eb>
140b22fe7: 48 8b 56 08                 	movq	0x8(%rsi), %rdx
140b22feb: 48 8b 1a                    	movq	(%rdx), %rbx
140b22fee: 4c 8d 73 01                 	leaq	0x1(%rbx), %r14
140b22ff2: 4c 89 32                    	movq	%r14, (%rdx)
140b22ff5: 48 ba 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
140b22fff: 48 89 95 00 02 00 00        	movq	%rdx, 0x200(%rbp)
140b23006: 48 8d 15 d7 38 11 02        	leaq	0x21138d7(%rip), %rdx   # 0x142c368e4
140b2300d: 48 89 95 08 02 00 00        	movq	%rdx, 0x208(%rbp)
140b23014: 48 c7 85 10 02 00 00 2a 00 00 00    	movq	$0x2a, 0x210(%rbp)
140b2301f: 66 c7 85 20 02 00 00 00 01  	movw	$0x100, 0x220(%rbp)     # imm = 0x100
140b23028: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
140b23033: 48 89 8d d0 01 00 00        	movq	%rcx, 0x1d0(%rbp)
140b2303a: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
140b23041: 48 c7 85 f0 01 00 00 01 00 00 00    	movq	$0x1, 0x1f0(%rbp)
140b2304c: 48 c7 85 f8 01 00 00 00 00 00 00    	movq	$0x0, 0x1f8(%rbp)
140b23057: 48 c7 85 18 02 00 00 00 00 00 00    	movq	$0x0, 0x218(%rbp)
140b23062: c6 85 22 02 00 00 01        	movb	$0x1, 0x222(%rbp)
140b23069: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
140b23074: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140b2307b: 4c 8d 85 c8 01 00 00        	leaq	0x1c8(%rbp), %r8
140b23082: 48 89 da                    	movq	%rbx, %rdx
140b23085: e8 e6 c7 21 00              	callq	0x140d3f870 <.text+0xd3e870>
140b2308a: 4c 39 77 10                 	cmpq	%r14, 0x10(%rdi)
140b2308e: 73 19                       	jae	0x140b230a9 <.text+0xb220a9>
140b23090: c6 85 4f 02 00 00 01        	movb	$0x1, 0x24f(%rbp)
140b23097: 4c 8d 05 8a 69 fc 01        	leaq	0x1fc698a(%rip), %r8    # 0x142ae9a28
140b2309e: 48 89 f9                    	movq	%rdi, %rcx
140b230a1: 4c 89 f2                    	movq	%r14, %rdx
140b230a4: e8 97 12 7d 00              	callq	0x1412f4340 <.text+0x12f3340>
140b230a9: 4c 8b 7f 08                 	movq	0x8(%rdi), %r15
140b230ad: 4c 69 f3 d0 00 00 00        	imulq	$0xd0, %rbx, %r14
140b230b4: 4b 8d 04 37                 	leaq	(%r15,%r14), %rax
140b230b8: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
140b230bf: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140b230c3: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
140b230ca: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
140b230d0: e8 2b 0b e0 01              	callq	0x142923c00 <.text+0x2922c00>
140b230d5: 43 83 3c 37 02              	cmpl	$0x2, (%r15,%r14)
140b230da: 74 0c                       	je	0x140b230e8 <.text+0xb220e8>
140b230dc: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
140b230e3: e8 68 da ac ff              	callq	0x1405f0b50 <.text+0x5efb50>
140b230e8: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140b230ec: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
140b230f2: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
140b230f9: e8 02 0b e0 01              	callq	0x142923c00 <.text+0x2922c00>
140b230fe: 48 83 c7 18                 	addq	$0x18, %rdi
140b23102: 48 ba 56 dd 40 97 20 82 c7 7e       	movabsq	$0x7ec782209740dd56, %rdx # imm = 0x7EC782209740DD56
140b2310c: 49 b8 9f d7 b8 11 29 55 79 e0       	movabsq	$-0x1f86aad6ee472861, %r8 # imm = 0xE079552911B8D79F
140b23116: 48 89 f9                    	movq	%rdi, %rcx
140b23119: 49 89 d9                    	movq	%rbx, %r9
140b2311c: e8 ff fa 55 ff              	callq	0x140082c20 <.text+0x81c20>
140b23121: 0f 10 05 28 66 fc 01        	movups	0x1fc6628(%rip), %xmm0  # 0x142ae9750
140b23128: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
140b2312f: f3 0f 6f 05 09 66 fc 01     	movdqu	0x1fc6609(%rip), %xmm0  # 0x142ae9740
140b23137: 66 0f 7f 85 80 00 00 00     	movdqa	%xmm0, 0x80(%rbp)
140b2313f: c6 85 4e 02 00 00 01        	movb	$0x1, 0x24e(%rbp)
140b23146: 48 8d 85 28 02 00 00        	leaq	0x228(%rbp), %rax
140b2314d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b23152: 4c 8d 85 80 00 00 00        	leaq	0x80(%rbp), %r8
140b23159: 48 89 d9                    	movq	%rbx, %rcx
140b2315c: 48 89 f2                    	movq	%rsi, %rdx
140b2315f: 45 31 c9                    	xorl	%r9d, %r9d
140b23162: e8 09 62 46 00              	callq	0x140f89370 <.text+0xf88370>
140b23167: 48 8b 06                    	movq	(%rsi), %rax
140b2316a: 48 8b 40 08                 	movq	0x8(%rax), %rax
140b2316e: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
140b23175: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
140b2317c: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
140b23180: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140b23184: 4a 8d 4c 30 60              	leaq	0x60(%rax,%r14), %rcx
140b23189: 4a 8d 94 30 80 00 00 00     	leaq	0x80(%rax,%r14), %rdx
140b23191: 41 b8 20 00 00 00           	movl	$0x20, %r8d
140b23197: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140b2319d: 48 89 8d 40 02 00 00        	movq	%rcx, 0x240(%rbp)
140b231a4: e8 97 e1 52 ff              	callq	0x140051340 <.text+0x50340>
140b231a9: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
140b231ae: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
140b231b3: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
140b231ba: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
140b231bf: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
140b231c3: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
140b231ca: 48 85 d2                    	testq	%rdx, %rdx
140b231cd: 0f 84 8d 00 00 00           	je	0x140b23260 <.text+0xb22260>
140b231d3: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
140b231da: 48 c1 e2 03                 	shlq	$0x3, %rdx
140b231de: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140b231e4: e8 37 0a af ff              	callq	0x140613c20 <.text+0x612c20>
140b231e9: eb 75                       	jmp	0x140b23260 <.text+0xb22260>
140b231eb: 41 0f 10 40 f0              	movups	-0x10(%r8), %xmm0
140b231f0: 0f 11 85 b8 01 00 00        	movups	%xmm0, 0x1b8(%rbp)
140b231f7: 41 0f 10 40 e0              	movups	-0x20(%r8), %xmm0
140b231fc: 0f 11 85 a8 01 00 00        	movups	%xmm0, 0x1a8(%rbp)
140b23203: 41 0f 10 40 d0              	movups	-0x30(%r8), %xmm0
140b23208: 0f 11 85 98 01 00 00        	movups	%xmm0, 0x198(%rbp)
140b2320f: f3 41 0f 6f 40 90           	movdqu	-0x70(%r8), %xmm0
140b23215: f3 41 0f 6f 48 a0           	movdqu	-0x60(%r8), %xmm1
140b2321b: f3 41 0f 6f 50 b0           	movdqu	-0x50(%r8), %xmm2
140b23221: f3 41 0f 6f 58 c0           	movdqu	-0x40(%r8), %xmm3
140b23227: f3 0f 7f 9d 88 01 00 00     	movdqu	%xmm3, 0x188(%rbp)
140b2322f: f3 0f 7f 95 78 01 00 00     	movdqu	%xmm2, 0x178(%rbp)
140b23237: f3 0f 7f 8d 68 01 00 00     	movdqu	%xmm1, 0x168(%rbp)
140b2323f: f3 0f 7f 85 58 01 00 00     	movdqu	%xmm0, 0x158(%rbp)
140b23247: 48 89 95 50 01 00 00        	movq	%rdx, 0x150(%rbp)
140b2324e: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
140b23255: 48 89 f2                    	movq	%rsi, %rdx
140b23258: e8 e3 c7 21 00              	callq	0x140d3fa40 <.text+0xd3ea40>
140b2325d: 48 89 c3                    	movq	%rax, %rbx
140b23260: 48 89 d8                    	movq	%rbx, %rax
140b23263: 48 81 c4 d8 02 00 00        	addq	$0x2d8, %rsp            # imm = 0x2D8
140b2326a: 5b                          	popq	%rbx
140b2326b: 5f                          	popq	%rdi
140b2326c: 5e                          	popq	%rsi
140b2326d: 41 5e                       	popq	%r14
140b2326f: 41 5f                       	popq	%r15
140b23271: 5d                          	popq	%rbp
140b23272: c3                          	retq
140b23273: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140b23280: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b23285: 55                          	pushq	%rbp
140b23286: 41 57                       	pushq	%r15
140b23288: 41 56                       	pushq	%r14
140b2328a: 56                          	pushq	%rsi
140b2328b: 57                          	pushq	%rdi
140b2328c: 53                          	pushq	%rbx
140b2328d: 48 83 ec 28                 	subq	$0x28, %rsp
140b23291: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b23298: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140b2329c: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
140b232a2: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
140b232a9: e8 52 09 e0 01              	callq	0x142923c00 <.text+0x2922c00>
140b232ae: c6 85 4f 02 00 00 00        	movb	$0x0, 0x24f(%rbp)
140b232b5: 48 83 c4 28                 	addq	$0x28, %rsp
140b232b9: 5b                          	popq	%rbx
140b232ba: 5f                          	popq	%rdi
140b232bb: 5e                          	popq	%rsi
140b232bc: 41 5e                       	popq	%r14
140b232be: 41 5f                       	popq	%r15
140b232c0: 5d                          	popq	%rbp
140b232c1: c3                          	retq
140b232c2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140b232d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b232d5: 55                          	pushq	%rbp
140b232d6: 41 57                       	pushq	%r15
140b232d8: 41 56                       	pushq	%r14
140b232da: 56                          	pushq	%rsi
140b232db: 57                          	pushq	%rdi
140b232dc: 53                          	pushq	%rbx
140b232dd: 48 83 ec 28                 	subq	$0x28, %rsp
140b232e1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b232e8: 80 bd 4f 02 00 00 00        	cmpb	$0x0, 0x24f(%rbp)
140b232ef: 74 0c                       	je	0x140b232fd <.text+0xb222fd>
140b232f1: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140b232f8: e8 53 d8 ac ff              	callq	0x1405f0b50 <.text+0x5efb50>
140b232fd: 90                          	nop
140b232fe: 48 83 c4 28                 	addq	$0x28, %rsp
140b23302: 5b                          	popq	%rbx
140b23303: 5f                          	popq	%rdi
140b23304: 5e                          	popq	%rsi
140b23305: 41 5e                       	popq	%r14
140b23307: 41 5f                       	popq	%r15
140b23309: 5d                          	popq	%rbp
140b2330a: c3                          	retq
140b2330b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b23310: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b23315: 55                          	pushq	%rbp
140b23316: 41 57                       	pushq	%r15
140b23318: 41 56                       	pushq	%r14
140b2331a: 56                          	pushq	%rsi
140b2331b: 57                          	pushq	%rdi
140b2331c: 53                          	pushq	%rbx
140b2331d: 48 83 ec 28                 	subq	$0x28, %rsp
140b23321: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b23328: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
140b2332d: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
140b23332: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
140b23339: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
140b2333e: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
140b23342: c6 85 4e 02 00 00 00        	movb	$0x0, 0x24e(%rbp)
140b23349: 48 83 c4 28                 	addq	$0x28, %rsp
140b2334d: 5b                          	popq	%rbx
140b2334e: 5f                          	popq	%rdi
140b2334f: 5e                          	popq	%rsi
140b23350: 41 5e                       	popq	%r14
140b23352: 41 5f                       	popq	%r15
140b23354: 5d                          	popq	%rbp
140b23355: c3                          	retq
140b23356: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140b23360: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b23365: 55                          	pushq	%rbp
140b23366: 41 57                       	pushq	%r15
140b23368: 41 56                       	pushq	%r14
140b2336a: 56                          	pushq	%rsi
140b2336b: 57                          	pushq	%rdi
140b2336c: 53                          	pushq	%rbx
140b2336d: 48 83 ec 28                 	subq	$0x28, %rsp
140b23371: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b23378: 80 bd 4e 02 00 00 00        	cmpb	$0x0, 0x24e(%rbp)
140b2337f: 74 1f                       	je	0x140b233a0 <.text+0xb223a0>
140b23381: 48 8d 95 a0 00 00 00        	leaq	0xa0(%rbp), %rdx
140b23388: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140b2338f: 41 b8 20 00 00 00           	movl	$0x20, %r8d
140b23395: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140b2339b: e8 a0 df 52 ff              	callq	0x140051340 <.text+0x50340>
140b233a0: 90                          	nop
140b233a1: 48 83 c4 28                 	addq	$0x28, %rsp
140b233a5: 5b                          	popq	%rbx
140b233a6: 5f                          	popq	%rdi
140b233a7: 5e                          	popq	%rsi
140b233a8: 41 5e                       	popq	%r14
140b233aa: 41 5f                       	popq	%r15
140b233ac: 5d                          	popq	%rbp
140b233ad: c3                          	retq
140b233ae: 66 90                       	nop
140b233b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b233b5: 55                          	pushq	%rbp
140b233b6: 41 57                       	pushq	%r15
140b233b8: 41 56                       	pushq	%r14
140b233ba: 56                          	pushq	%rsi
140b233bb: 57                          	pushq	%rdi
140b233bc: 53                          	pushq	%rbx
140b233bd: 48 83 ec 28                 	subq	$0x28, %rsp
140b233c1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b233c8: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
140b233cf: 48 85 d2                    	testq	%rdx, %rdx
140b233d2: 74 16                       	je	0x140b233ea <.text+0xb223ea>
140b233d4: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
140b233db: 48 c1 e2 03                 	shlq	$0x3, %rdx
140b233df: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140b233e5: e8 36 08 af ff              	callq	0x140613c20 <.text+0x612c20>
140b233ea: 90                          	nop
140b233eb: 48 83 c4 28                 	addq	$0x28, %rsp
140b233ef: 5b                          	popq	%rbx
140b233f0: 5f                          	popq	%rdi
140b233f1: 5e                          	popq	%rsi
140b233f2: 41 5e                       	popq	%r14
140b233f4: 41 5f                       	popq	%r15
140b233f6: 5d                          	popq	%rbp
140b233f7: c3                          	retq
