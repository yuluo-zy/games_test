
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141697ab0: 55                          	pushq	%rbp
141697ab1: 41 57                       	pushq	%r15
141697ab3: 41 56                       	pushq	%r14
141697ab5: 41 55                       	pushq	%r13
141697ab7: 41 54                       	pushq	%r12
141697ab9: 56                          	pushq	%rsi
141697aba: 57                          	pushq	%rdi
141697abb: 53                          	pushq	%rbx
141697abc: 48 81 ec 08 04 00 00        	subq	$0x408, %rsp            # imm = 0x408
141697ac3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141697acb: 66 44 0f 7f 9d 70 03 00 00  	movdqa	%xmm11, 0x370(%rbp)
141697ad4: 66 44 0f 7f 95 60 03 00 00  	movdqa	%xmm10, 0x360(%rbp)
141697add: 44 0f 29 8d 50 03 00 00     	movaps	%xmm9, 0x350(%rbp)
141697ae5: 66 44 0f 7f 85 40 03 00 00  	movdqa	%xmm8, 0x340(%rbp)
141697aee: 66 0f 7f bd 30 03 00 00     	movdqa	%xmm7, 0x330(%rbp)
141697af6: 66 0f 7f b5 20 03 00 00     	movdqa	%xmm6, 0x320(%rbp)
141697afe: 48 c7 85 18 03 00 00 fe ff ff ff    	movq	$-0x2, 0x318(%rbp)
141697b09: 4c 89 ce                    	movq	%r9, %rsi
141697b0c: 4c 89 c3                    	movq	%r8, %rbx
141697b0f: 49 89 d2                    	movq	%rdx, %r10
141697b12: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
141697b19: 4c 8d 82 e0 00 00 00        	leaq	0xe0(%rdx), %r8
141697b20: 4c 8d 8a 18 01 00 00        	leaq	0x118(%rdx), %r9
141697b27: 8b 82 f8 02 00 00           	movl	0x2f8(%rdx), %eax
141697b2d: 0f 10 42 70                 	movups	0x70(%rdx), %xmm0
141697b31: 0f 29 85 d0 01 00 00        	movaps	%xmm0, 0x1d0(%rbp)
141697b38: f3 0f 6f 42 30              	movdqu	0x30(%rdx), %xmm0
141697b3d: f3 0f 6f 4a 40              	movdqu	0x40(%rdx), %xmm1
141697b42: f3 0f 6f 52 50              	movdqu	0x50(%rdx), %xmm2
141697b47: 0f 10 5a 60                 	movups	0x60(%rdx), %xmm3
141697b4b: 0f 29 9d c0 01 00 00        	movaps	%xmm3, 0x1c0(%rbp)
141697b52: 66 0f 7f 95 b0 01 00 00     	movdqa	%xmm2, 0x1b0(%rbp)
141697b5a: 66 0f 7f 8d a0 01 00 00     	movdqa	%xmm1, 0x1a0(%rbp)
141697b62: 66 0f 7f 85 90 01 00 00     	movdqa	%xmm0, 0x190(%rbp)
141697b6a: c6 85 14 03 00 00 01        	movb	$0x1, 0x314(%rbp)
141697b71: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
141697b78: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
141697b7d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141697b82: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
141697b86: 48 89 da                    	movq	%rbx, %rdx
141697b89: 4c 89 85 70 01 00 00        	movq	%r8, 0x170(%rbp)
141697b90: 4c 89 95 f0 02 00 00        	movq	%r10, 0x2f0(%rbp)
141697b97: e8 f4 f2 05 00              	callq	0x1416f6e90 <.text+0x16f5e90>
141697b9c: 83 7d c0 0b                 	cmpl	$0xb, -0x40(%rbp)
141697ba0: 0f 85 6c 1b 00 00           	jne	0x141699712 <.text+0x1698712>
141697ba6: 48 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %rax
141697bad: 48 8b 90 10 01 00 00        	movq	0x110(%rax), %rdx
141697bb4: c6 85 14 03 00 00 01        	movb	$0x1, 0x314(%rbp)
141697bbb: 48 89 d9                    	movq	%rbx, %rcx
141697bbe: e8 ad fc 05 00              	callq	0x1416f7870 <.text+0x16f6870>
141697bc3: 48 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %rcx
141697bca: 48 8b 41 20                 	movq	0x20(%rcx), %rax
141697bce: 48 8b 49 28                 	movq	0x28(%rcx), %rcx
141697bd2: 48 89 ca                    	movq	%rcx, %rdx
141697bd5: 48 c1 e2 06                 	shlq	$0x6, %rdx
141697bd9: 48 8d 0c 8a                 	leaq	(%rdx,%rcx,4), %rcx
141697bdd: 48 01 c1                    	addq	%rax, %rcx
141697be0: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
141697be7: 48 89 8d 98 01 00 00        	movq	%rcx, 0x198(%rbp)
141697bee: 48 89 9d a0 01 00 00        	movq	%rbx, 0x1a0(%rbp)
141697bf5: c6 85 14 03 00 00 01        	movb	$0x1, 0x314(%rbp)
141697bfc: 4c 8d 05 8d f2 5e 01        	leaq	0x15ef28d(%rip), %r8    # 0x142c86e90
141697c03: 48 8d 8d e8 00 00 00        	leaq	0xe8(%rbp), %rcx
141697c0a: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
141697c11: e8 9a 47 c6 ff              	callq	0x1412fc3b0 <.text+0x12fb3b0>
141697c16: 4c 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %r9
141697c1d: 49 83 79 10 00              	cmpq	$0x0, 0x10(%r9)
141697c22: 74 2b                       	je	0x141697c4f <.text+0x1696c4f>
141697c24: 48 8b 13                    	movq	(%rbx), %rdx
141697c27: 49 8d 89 c0 00 00 00        	leaq	0xc0(%r9), %rcx
141697c2e: c6 85 07 03 00 00 01        	movb	$0x1, 0x307(%rbp)
141697c35: e8 36 ee 53 ff              	callq	0x140bd6a70 <.text+0xbd5a70>
141697c3a: 4c 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %r9
141697c41: 41 c6 81 d8 00 00 00 01     	movb	$0x1, 0xd8(%r9)
141697c49: 49 8b 41 10                 	movq	0x10(%r9), %rax
141697c4d: eb 0a                       	jmp	0x141697c59 <.text+0x1696c59>
141697c4f: 41 c6 81 d8 00 00 00 02     	movb	$0x2, 0xd8(%r9)
141697c57: 31 c0                       	xorl	%eax, %eax
141697c59: 49 8d 89 80 00 00 00        	leaq	0x80(%r9), %rcx
141697c60: 48 89 8d 80 01 00 00        	movq	%rcx, 0x180(%rbp)
141697c67: 49 8b 09                    	movq	(%r9), %rcx
141697c6a: 49 8b 51 08                 	movq	0x8(%r9), %rdx
141697c6e: 48 69 c0 b8 00 00 00        	imulq	$0xb8, %rax, %rax
141697c75: 48 01 d0                    	addq	%rdx, %rax
141697c78: 48 89 95 50 01 00 00        	movq	%rdx, 0x150(%rbp)
141697c7f: 48 89 95 58 01 00 00        	movq	%rdx, 0x158(%rbp)
141697c86: 48 89 8d 60 01 00 00        	movq	%rcx, 0x160(%rbp)
141697c8d: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
141697c94: 48 8b 0e                    	movq	(%rsi), %rcx
141697c97: 48 89 8d c8 00 00 00        	movq	%rcx, 0xc8(%rbp)
141697c9e: 4c 8b 46 18                 	movq	0x18(%rsi), %r8
141697ca2: 49 8d 89 f8 00 00 00        	leaq	0xf8(%r9), %rcx
141697ca9: 48 89 8d 88 01 00 00        	movq	%rcx, 0x188(%rbp)
141697cb0: 4c 89 85 00 01 00 00        	movq	%r8, 0x100(%rbp)
141697cb7: 49 8d 48 20                 	leaq	0x20(%r8), %rcx
141697cbb: 48 89 8d d0 00 00 00        	movq	%rcx, 0xd0(%rbp)
141697cc2: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
141697cc6: 66 0f 6f 3d b2 53 33 01     	movdqa	0x13353b2(%rip), %xmm7  # 0x1429cd080
141697cce: 66 44 0f 6f 05 b9 53 33 01  	movdqa	0x13353b9(%rip), %xmm8  # 0x1429cd090
141697cd7: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
141697cdb: 49 8d 89 c0 00 00 00        	leaq	0xc0(%r9), %rcx
141697ce2: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
141697ce9: 48 c7 85 e8 02 00 00 00 00 00 00    	movq	$0x0, 0x2e8(%rbp)
141697cf4: 4c 8d 75 c0                 	leaq	-0x40(%rbp), %r14
141697cf8: 31 f6                       	xorl	%esi, %esi
141697cfa: 48 89 9d 08 03 00 00        	movq	%rbx, 0x308(%rbp)
141697d01: eb 22                       	jmp	0x141697d25 <.text+0x1696d25>
141697d03: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141697d10: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
141697d17: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
141697d1e: 40 b6 01                    	movb	$0x1, %sil
141697d21: 4c 8d 75 c0                 	leaq	-0x40(%rbp), %r14
141697d25: 48 39 c2                    	cmpq	%rax, %rdx
141697d28: 0f 84 4e 16 00 00           	je	0x14169937c <.text+0x169837c>
141697d2e: 4c 8d ba b8 00 00 00        	leaq	0xb8(%rdx), %r15
141697d35: 4c 89 bd 58 01 00 00        	movq	%r15, 0x158(%rbp)
141697d3c: 48 8b 3a                    	movq	(%rdx), %rdi
141697d3f: 48 83 c2 08                 	addq	$0x8, %rdx
141697d43: 41 b8 b0 00 00 00           	movl	$0xb0, %r8d
141697d49: 4c 89 f1                    	movq	%r14, %rcx
141697d4c: e8 af be 28 01              	callq	0x142923c00 <.text+0x2922c00>
141697d51: 48 83 ff 02                 	cmpq	$0x2, %rdi
141697d55: 0f 84 28 16 00 00           	je	0x141699383 <.text+0x1698383>
141697d5b: 48 89 bd 90 01 00 00        	movq	%rdi, 0x190(%rbp)
141697d62: 41 b8 b0 00 00 00           	movl	$0xb0, %r8d
141697d68: 48 8d 8d 98 01 00 00        	leaq	0x198(%rbp), %rcx
141697d6f: 4c 89 f2                    	movq	%r14, %rdx
141697d72: e8 89 be 28 01              	callq	0x142923c00 <.text+0x2922c00>
141697d77: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
141697d7e: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
141697d85: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141697d8c: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141697d93: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141697d9a: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
141697da1: 48 8d 95 50 02 00 00        	leaq	0x250(%rbp), %rdx
141697da8: e8 d3 15 2d ff              	callq	0x140969380 <.text+0x968380>
141697dad: 48 89 c3                    	movq	%rax, %rbx
141697db0: 48 85 c0                    	testq	%rax, %rax
141697db3: 0f 84 c1 18 00 00           	je	0x14169967a <.text+0x169867a>
141697db9: 44 0f b6 bd 40 02 00 00     	movzbl	0x240(%rbp), %r15d
141697dc1: 80 3d d0 ce f9 01 00        	cmpb	$0x0, 0x1f9ced0(%rip)   # 0x143634c98
141697dc8: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141697dcf: 0f 84 25 01 00 00           	je	0x141697efa <.text+0x1696efa>
141697dd5: 48 8b 05 cc 45 f9 01        	movq	0x1f945cc(%rip), %rax   # 0x14362c3a8
141697ddc: 48 8b 00                    	movq	(%rax), %rax
141697ddf: 48 83 f8 03                 	cmpq	$0x3, %rax
141697de3: 0f 82 11 01 00 00           	jb	0x141697efa <.text+0x1696efa>
141697de9: 48 8d 43 48                 	leaq	0x48(%rbx), %rax
141697ded: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
141697df4: 48 8d 05 a5 ba e9 fe        	leaq	-0x116455b(%rip), %rax  # 0x1405338a0 <.text+0x5328a0>
141697dfb: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
141697e02: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141697e09: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141697e10: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141697e17: 48 8d 0d b2 fb 5e 01        	leaq	0x15efbb2(%rip), %rcx   # 0x142c879d0
141697e1e: e8 5d 5f fe fe              	callq	0x14067dd80 <.text+0x67cd80>
141697e23: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
141697e27: 8b 40 10                    	movl	0x10(%rax), %eax
141697e2a: 48 c7 85 80 02 00 00 03 00 00 00    	movq	$0x3, 0x280(%rbp)
141697e35: 48 8d 0d ac fb 5e 01        	leaq	0x15efbac(%rip), %rcx   # 0x142c879e8
141697e3c: 48 89 8d 88 02 00 00        	movq	%rcx, 0x288(%rbp)
141697e43: 48 c7 85 90 02 00 00 2f 00 00 00    	movq	$0x2f, 0x290(%rbp)
141697e4e: 48 8d 15 6b fb 5e 01        	leaq	0x15efb6b(%rip), %rdx   # 0x142c879c0
141697e55: 48 89 95 a0 02 00 00        	movq	%rdx, 0x2a0(%rbp)
141697e5c: 48 c7 85 a8 02 00 00 01 00 00 00    	movq	$0x1, 0x2a8(%rbp)
141697e67: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
141697e6e: 48 89 95 b0 02 00 00        	movq	%rdx, 0x2b0(%rbp)
141697e75: 48 c7 85 b8 02 00 00 01 00 00 00    	movq	$0x1, 0x2b8(%rbp)
141697e80: 48 c7 85 c0 02 00 00 00 00 00 00    	movq	$0x0, 0x2c0(%rbp)
141697e8b: 48 c7 85 50 02 00 00 00 00 00 00    	movq	$0x0, 0x250(%rbp)
141697e96: 48 89 8d 58 02 00 00        	movq	%rcx, 0x258(%rbp)
141697e9d: 48 c7 85 60 02 00 00 2f 00 00 00    	movq	$0x2f, 0x260(%rbp)
141697ea8: 48 c7 85 68 02 00 00 00 00 00 00    	movq	$0x0, 0x268(%rbp)
141697eb3: f3 0f 7f 85 70 02 00 00     	movdqu	%xmm0, 0x270(%rbp)
141697ebb: c7 85 98 02 00 00 01 00 00 00       	movl	$0x1, 0x298(%rbp)
141697ec5: 89 85 9c 02 00 00           	movl	%eax, 0x29c(%rbp)
141697ecb: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141697ed2: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141697ed9: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141697ee0: 48 8d 8d 06 03 00 00        	leaq	0x306(%rbp), %rcx
141697ee7: 48 8d 95 50 02 00 00        	leaq	0x250(%rbp), %rdx
141697eee: e8 3d 24 6b ff              	callq	0x140d4a330 <.text+0xd49330>
141697ef3: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141697efa: 48 8b 11                    	movq	(%rcx), %rdx
141697efd: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
141697f01: 4c 8b 43 48                 	movq	0x48(%rbx), %r8
141697f05: 4c 8b 4b 50                 	movq	0x50(%rbx), %r9
141697f09: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141697f10: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141697f17: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141697f1e: e8 8d 8a 55 ff              	callq	0x140bf09b0 <.text+0xbef9b0>
141697f23: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
141697f2a: 4c 8b 70 08                 	movq	0x8(%rax), %r14
141697f2e: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141697f35: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141697f3c: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141697f43: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
141697f4a: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
141697f51: e8 aa eb 53 ff              	callq	0x140bd6b00 <.text+0xbd5b00>
141697f56: 44 8b 85 38 02 00 00        	movl	0x238(%rbp), %r8d
141697f5d: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141697f64: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141697f6b: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141697f72: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
141697f79: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
141697f80: e8 db e7 54 ff              	callq	0x140be6760 <.text+0xbe5760>
141697f85: 41 c1 e7 06                 	shll	$0x6, %r15d
141697f89: 4a 8d 0c 3b                 	leaq	(%rbx,%r15), %rcx
141697f8d: 48 83 c1 58                 	addq	$0x58, %rcx
141697f91: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141697f98: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141697f9f: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141697fa6: 4c 89 f2                    	movq	%r14, %rdx
141697fa9: 4c 8d 85 50 02 00 00        	leaq	0x250(%rbp), %r8
141697fb0: e8 4b 0b 2a ff              	callq	0x140938b00 <.text+0x937b00>
141697fb5: 49 89 c4                    	movq	%rax, %r12
141697fb8: 48 39 85 c0 00 00 00        	cmpq	%rax, 0xc0(%rbp)
141697fbf: 0f 94 c0                    	sete	%al
141697fc2: 40 84 c6                    	testb	%al, %sil
141697fc5: 75 32                       	jne	0x141697ff9 <.text+0x1696ff9>
141697fc7: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
141697fce: 48 8b 10                    	movq	(%rax), %rdx
141697fd1: 48 8b 48 08                 	movq	0x8(%rax), %rcx
141697fd5: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141697fdc: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141697fe3: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141697fea: 4d 89 e0                    	movq	%r12, %r8
141697fed: e8 3e 61 55 ff              	callq	0x140bee130 <.text+0xbed130>
141697ff2: 4c 89 a5 c0 00 00 00        	movq	%r12, 0xc0(%rbp)
141697ff9: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
141698000: 48 89 8d 78 01 00 00        	movq	%rcx, 0x178(%rbp)
141698007: 48 83 f9 01                 	cmpq	$0x1, %rcx
14169800b: 0f 85 ef 00 00 00           	jne	0x141698100 <.text+0x1697100>
141698011: 48 8b b5 98 01 00 00        	movq	0x198(%rbp), %rsi
141698018: 48 89 b5 50 02 00 00        	movq	%rsi, 0x250(%rbp)
14169801f: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
141698026: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
14169802b: 0f 84 25 13 00 00           	je	0x141699356 <.text+0x1698356>
141698031: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698038: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169803f: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141698046: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
14169804d: 48 8d 95 50 02 00 00        	leaq	0x250(%rbp), %rdx
141698054: e8 17 8d f1 fe              	callq	0x1405b0d70 <.text+0x5afd70>
141698059: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
141698060: 4c 8b 19                    	movq	(%rcx), %r11
141698063: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
141698067: 48 89 ca                    	movq	%rcx, %rdx
14169806a: 48 21 c2                    	andq	%rax, %rdx
14169806d: 48 c1 e8 39                 	shrq	$0x39, %rax
141698071: 66 0f 6e c0                 	movd	%eax, %xmm0
141698075: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141698079: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14169807e: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141698083: 49 8d 43 b0                 	leaq	-0x50(%r11), %rax
141698087: 45 31 c0                    	xorl	%r8d, %r8d
14169808a: f3 41 0f 6f 0c 13           	movdqu	(%r11,%rdx), %xmm1
141698090: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
141698094: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
141698098: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
14169809d: 45 85 c9                    	testl	%r9d, %r9d
1416980a0: 74 2e                       	je	0x1416980d0 <.text+0x16970d0>
1416980a2: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1416980a7: 49 01 d2                    	addq	%rdx, %r10
1416980aa: 49 21 ca                    	andq	%rcx, %r10
1416980ad: 49 f7 da                    	negq	%r10
1416980b0: 4f 8d 14 92                 	leaq	(%r10,%r10,4), %r10
1416980b4: 49 c1 e2 04                 	shlq	$0x4, %r10
1416980b8: 4a 39 34 10                 	cmpq	%rsi, (%rax,%r10)
1416980bc: 74 52                       	je	0x141698110 <.text+0x1697110>
1416980be: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1416980c2: 66 45 21 ca                 	andw	%r9w, %r10w
1416980c6: 45 89 d1                    	movl	%r10d, %r9d
1416980c9: 75 d7                       	jne	0x1416980a2 <.text+0x16970a2>
1416980cb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1416980d0: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1416980d4: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1416980d9: 45 85 c9                    	testl	%r9d, %r9d
1416980dc: 0f 85 74 12 00 00           	jne	0x141699356 <.text+0x1698356>
1416980e2: 4c 01 c2                    	addq	%r8, %rdx
1416980e5: 48 83 c2 10                 	addq	$0x10, %rdx
1416980e9: 49 83 c0 10                 	addq	$0x10, %r8
1416980ed: 48 21 ca                    	andq	%rcx, %rdx
1416980f0: eb 98                       	jmp	0x14169808a <.text+0x169708a>
1416980f2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141698100: 80 3d 91 cb f9 01 00        	cmpb	$0x0, 0x1f9cb91(%rip)   # 0x143634c98
141698107: 75 54                       	jne	0x14169815d <.text+0x169715d>
141698109: e9 c6 02 00 00              	jmp	0x1416983d4 <.text+0x16973d4>
14169810e: 66 90                       	nop
141698110: 4b 8b 44 13 f8              	movq	-0x8(%r11,%r10), %rax
141698115: 48 89 85 d8 02 00 00        	movq	%rax, 0x2d8(%rbp)
14169811c: 0f b6 85 90 01 00 00        	movzbl	0x190(%rbp), %eax
141698123: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
14169812a: 89 d1                       	movl	%edx, %ecx
14169812c: 20 c1                       	andb	%al, %cl
14169812e: f6 c1 01                    	testb	$0x1, %cl
141698131: 0f 84 9c 06 00 00           	je	0x1416987d3 <.text+0x16977d3>
141698137: 48 8b 85 d0 02 00 00        	movq	0x2d0(%rbp), %rax
14169813e: 48 3b 85 98 01 00 00        	cmpq	0x198(%rbp), %rax
141698145: 0f 85 92 06 00 00           	jne	0x1416987dd <.text+0x16977dd>
14169814b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141698150: 80 3d 41 cb f9 01 00        	cmpb	$0x0, 0x1f9cb41(%rip)   # 0x143634c98
141698157: 0f 84 77 02 00 00           	je	0x1416983d4 <.text+0x16973d4>
14169815d: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
141698164: 48 8b 8d b0 01 00 00        	movq	0x1b0(%rbp), %rcx
14169816b: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
141698172: 4c 8b 85 08 02 00 00        	movq	0x208(%rbp), %r8
141698179: 49 01 d0                    	addq	%rdx, %r8
14169817c: 49 ff c0                    	incq	%r8
14169817f: 66 0f 6f 02                 	movdqa	(%rdx), %xmm0
141698183: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
141698188: 41 f7 d1                    	notl	%r9d
14169818b: 4c 8b 95 18 02 00 00        	movq	0x218(%rbp), %r10
141698192: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
141698196: 48 89 95 68 02 00 00        	movq	%rdx, 0x268(%rbp)
14169819d: 48 83 c2 10                 	addq	$0x10, %rdx
1416981a1: 48 89 95 70 02 00 00        	movq	%rdx, 0x270(%rbp)
1416981a8: 4c 89 85 78 02 00 00        	movq	%r8, 0x278(%rbp)
1416981af: 66 44 89 8d 80 02 00 00     	movw	%r9w, 0x280(%rbp)
1416981b7: 4c 89 95 88 02 00 00        	movq	%r10, 0x288(%rbp)
1416981be: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1416981c5: 48 89 8d 58 02 00 00        	movq	%rcx, 0x258(%rbp)
1416981cc: 48 8d 85 e8 00 00 00        	leaq	0xe8(%rbp), %rax
1416981d3: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
1416981da: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416981e1: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416981e8: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
1416981ef: 31 c9                       	xorl	%ecx, %ecx
1416981f1: e8 2a 19 a4 fe              	callq	0x1400d9b20 <.text+0xd8b20>
1416981f6: 48 85 c0                    	testq	%rax, %rax
1416981f9: 0f 84 a1 14 00 00           	je	0x1416996a0 <.text+0x16986a0>
1416981ff: 0f 10 00                    	movups	(%rax), %xmm0
141698202: 48 ff 00                    	incq	(%rax)
141698205: 0f 10 0d 34 76 5c 01        	movups	0x15c7634(%rip), %xmm1  # 0x142c5f840
14169820c: 0f 29 8d 20 01 00 00        	movaps	%xmm1, 0x120(%rbp)
141698213: 0f 10 0d 16 76 5c 01        	movups	0x15c7616(%rip), %xmm1  # 0x142c5f830
14169821a: 0f 29 8d 10 01 00 00        	movaps	%xmm1, 0x110(%rbp)
141698221: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
141698228: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
14169822f: 48 8d 95 50 02 00 00        	leaq	0x250(%rbp), %rdx
141698236: e8 a5 ac e8 ff              	callq	0x141522ee0 <.text+0x1521ee0>
14169823b: 66 0f 6f 85 10 01 00 00     	movdqa	0x110(%rbp), %xmm0
141698243: 66 0f 6f 8d 20 01 00 00     	movdqa	0x120(%rbp), %xmm1
14169824b: 66 0f 6f 95 30 01 00 00     	movdqa	0x130(%rbp), %xmm2
141698253: 66 0f 7f 95 90 00 00 00     	movdqa	%xmm2, 0x90(%rbp)
14169825b: 66 0f 7f 8d 80 00 00 00     	movdqa	%xmm1, 0x80(%rbp)
141698263: 66 0f 7f 45 70              	movdqa	%xmm0, 0x70(%rbp)
141698268: 48 8b 05 39 41 f9 01        	movq	0x1f94139(%rip), %rax   # 0x14362c3a8
14169826f: 48 8b 00                    	movq	(%rax), %rax
141698272: 48 83 f8 03                 	cmpq	$0x3, %rax
141698276: 0f 82 27 01 00 00           	jb	0x1416983a3 <.text+0x16973a3>
14169827c: 48 8d 45 70                 	leaq	0x70(%rbp), %rax
141698280: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
141698287: 48 8d 05 72 31 db ff        	leaq	-0x24ce8e(%rip), %rax   # 0x14144b400 <.text+0x144a400>
14169828e: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
141698295: 48 c7 85 10 01 00 00 02 00 00 00    	movq	$0x2, 0x110(%rbp)
1416982a0: 48 c7 85 20 01 00 00 02 00 00 00    	movq	$0x2, 0x120(%rbp)
1416982ab: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
1416982b6: 48 b8 20 00 00 00 04 00 00 00       	movabsq	$0x400000020, %rax # imm = 0x400000020
1416982c0: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
1416982c7: c6 85 40 01 00 00 03        	movb	$0x3, 0x140(%rbp)
1416982ce: 48 8d 0d 7b f7 5e 01        	leaq	0x15ef77b(%rip), %rcx   # 0x142c87a50
1416982d5: e8 a6 5a fe fe              	callq	0x14067dd80 <.text+0x67cd80>
1416982da: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1416982de: 8b 40 10                    	movl	0x10(%rax), %eax
1416982e1: 48 c7 85 80 02 00 00 03 00 00 00    	movq	$0x3, 0x280(%rbp)
1416982ec: 48 8d 0d f5 f6 5e 01        	leaq	0x15ef6f5(%rip), %rcx   # 0x142c879e8
1416982f3: 48 89 8d 88 02 00 00        	movq	%rcx, 0x288(%rbp)
1416982fa: 48 c7 85 90 02 00 00 2f 00 00 00    	movq	$0x2f, 0x290(%rbp)
141698305: 48 8d 15 34 f7 5e 01        	leaq	0x15ef734(%rip), %rdx   # 0x142c87a40
14169830c: 48 89 95 a0 02 00 00        	movq	%rdx, 0x2a0(%rbp)
141698313: 48 c7 85 a8 02 00 00 01 00 00 00    	movq	$0x1, 0x2a8(%rbp)
14169831e: 48 8d 95 a0 00 00 00        	leaq	0xa0(%rbp), %rdx
141698325: 48 89 95 b0 02 00 00        	movq	%rdx, 0x2b0(%rbp)
14169832c: 48 c7 85 b8 02 00 00 01 00 00 00    	movq	$0x1, 0x2b8(%rbp)
141698337: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
14169833e: 48 89 95 c0 02 00 00        	movq	%rdx, 0x2c0(%rbp)
141698345: 48 c7 85 c8 02 00 00 01 00 00 00    	movq	$0x1, 0x2c8(%rbp)
141698350: 48 c7 85 50 02 00 00 00 00 00 00    	movq	$0x0, 0x250(%rbp)
14169835b: 48 89 8d 58 02 00 00        	movq	%rcx, 0x258(%rbp)
141698362: 48 c7 85 60 02 00 00 2f 00 00 00    	movq	$0x2f, 0x260(%rbp)
14169836d: 48 c7 85 68 02 00 00 00 00 00 00    	movq	$0x0, 0x268(%rbp)
141698378: f3 0f 7f 85 70 02 00 00     	movdqu	%xmm0, 0x270(%rbp)
141698380: c7 85 98 02 00 00 01 00 00 00       	movl	$0x1, 0x298(%rbp)
14169838a: 89 85 9c 02 00 00           	movl	%eax, 0x29c(%rbp)
141698390: 48 8d 8d 06 03 00 00        	leaq	0x306(%rbp), %rcx
141698397: 48 8d 95 50 02 00 00        	leaq	0x250(%rbp), %rdx
14169839e: e8 8d 1f 6b ff              	callq	0x140d4a330 <.text+0xd49330>
1416983a3: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416983aa: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416983b1: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
1416983b8: 41 b8 20 00 00 00           	movl	$0x20, %r8d
1416983be: 41 b9 10 00 00 00           	movl	$0x10, %r9d
1416983c4: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1416983c8: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
1416983cf: e8 0c 83 9b fe              	callq	0x1400506e0 <.text+0x4f6e0>
1416983d4: 48 8b b5 00 02 00 00        	movq	0x200(%rbp), %rsi
1416983db: 48 8b 9d 18 02 00 00        	movq	0x218(%rbp), %rbx
1416983e2: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
1416983e6: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1416983ea: f7 d0                       	notl	%eax
1416983ec: 4c 8d 76 10                 	leaq	0x10(%rsi), %r14
1416983f0: 4c 8b 85 08 03 00 00        	movq	0x308(%rbp), %r8
1416983f7: 48 83 eb 01                 	subq	$0x1, %rbx
1416983fb: 0f 82 8e 00 00 00           	jb	0x14169848f <.text+0x169748f>
141698401: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141698410: 66 85 c0                    	testw	%ax, %ax
141698413: 74 0b                       	je	0x141698420 <.text+0x1697420>
141698415: 8d 78 ff                    	leal	-0x1(%rax), %edi
141698418: 21 c7                       	andl	%eax, %edi
14169841a: eb 2c                       	jmp	0x141698448 <.text+0x1697448>
14169841c: 0f 1f 40 00                 	nopl	(%rax)
141698420: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
141698425: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
141698429: 48 81 c6 80 fb ff ff        	addq	$-0x480, %rsi           # imm = 0xFB80
141698430: 49 83 c6 10                 	addq	$0x10, %r14
141698434: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
141698439: 74 e5                       	je	0x141698420 <.text+0x1697420>
14169843b: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
141698440: 29 c1                       	subl	%eax, %ecx
141698442: f7 d0                       	notl	%eax
141698444: 89 c7                       	movl	%eax, %edi
141698446: 21 cf                       	andl	%ecx, %edi
141698448: f3 0f bc c0                 	tzcntl	%eax, %eax
14169844c: 48 f7 d8                    	negq	%rax
14169844f: 4c 8d 3c c0                 	leaq	(%rax,%rax,8), %r15
141698453: 49 8b 10                    	movq	(%r8), %rdx
141698456: 4a 8d 0c fe                 	leaq	(%rsi,%r15,8), %rcx
14169845a: 48 83 c1 e0                 	addq	$-0x20, %rcx
14169845e: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698465: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169846c: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141698473: e8 f8 e5 53 ff              	callq	0x140bd6a70 <.text+0xbd5a70>
141698478: 4a 8d 04 fe                 	leaq	(%rsi,%r15,8), %rax
14169847c: c6 40 f8 01                 	movb	$0x1, -0x8(%rax)
141698480: 89 f8                       	movl	%edi, %eax
141698482: 4c 8b 85 08 03 00 00        	movq	0x308(%rbp), %r8
141698489: 48 83 eb 01                 	subq	$0x1, %rbx
14169848d: 73 81                       	jae	0x141698410 <.text+0x1697410>
14169848f: 49 8b 10                    	movq	(%r8), %rdx
141698492: 49 8b 48 08                 	movq	0x8(%r8), %rcx
141698496: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
14169849d: 4c 8b 85 18 02 00 00        	movq	0x218(%rbp), %r8
1416984a4: 4c 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %r9
1416984ab: 4c 8b 95 a8 01 00 00        	movq	0x1a8(%rbp), %r10
1416984b2: 4d 8d 1c c2                 	leaq	(%r10,%rax,8), %r11
1416984b6: 48 8b b5 00 02 00 00        	movq	0x200(%rbp), %rsi
1416984bd: 48 8b bd 08 02 00 00        	movq	0x208(%rbp), %rdi
1416984c4: 48 8d 7c 3e 01              	leaq	0x1(%rsi,%rdi), %rdi
1416984c9: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
1416984cd: 66 0f d7 d8                 	pmovmskb	%xmm0, %ebx
1416984d1: f7 d3                       	notl	%ebx
1416984d3: 4c 01 c0                    	addq	%r8, %rax
1416984d6: 48 89 b5 78 02 00 00        	movq	%rsi, 0x278(%rbp)
1416984dd: 48 83 c6 10                 	addq	$0x10, %rsi
1416984e1: 48 89 b5 80 02 00 00        	movq	%rsi, 0x280(%rbp)
1416984e8: 48 89 bd 88 02 00 00        	movq	%rdi, 0x288(%rbp)
1416984ef: 66 89 9d 90 02 00 00        	movw	%bx, 0x290(%rbp)
1416984f6: 4c 89 85 98 02 00 00        	movq	%r8, 0x298(%rbp)
1416984fd: 4c 89 95 50 02 00 00        	movq	%r10, 0x250(%rbp)
141698504: 4c 89 95 58 02 00 00        	movq	%r10, 0x258(%rbp)
14169850b: 4c 89 8d 60 02 00 00        	movq	%r9, 0x260(%rbp)
141698512: 4c 89 9d 68 02 00 00        	movq	%r11, 0x268(%rbp)
141698519: 4c 8d 85 e8 00 00 00        	leaq	0xe8(%rbp), %r8
141698520: 4c 89 85 70 02 00 00        	movq	%r8, 0x270(%rbp)
141698527: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169852e: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698535: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169853c: 4c 8d 85 50 02 00 00        	leaq	0x250(%rbp), %r8
141698543: 4c 89 44 24 28              	movq	%r8, 0x28(%rsp)
141698548: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14169854d: 4d 89 e0                    	movq	%r12, %r8
141698550: 45 31 c9                    	xorl	%r9d, %r9d
141698553: e8 b8 d3 00 00              	callq	0x1416a5910 <.text+0x16a4910>
141698558: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169855f: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698566: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169856d: 4c 89 e1                    	movq	%r12, %rcx
141698570: e8 0b 58 fe fe              	callq	0x14067dd80 <.text+0x67cd80>
141698575: 48 8b 40 18                 	movq	0x18(%rax), %rax
141698579: 48 83 78 50 02              	cmpq	$0x2, 0x50(%rax)
14169857e: 72 78                       	jb	0x1416985f8 <.text+0x16975f8>
141698580: 48 8b 40 48                 	movq	0x48(%rax), %rax
141698584: 48 83 78 48 00              	cmpq	$0x0, 0x48(%rax)
141698589: 74 6d                       	je	0x1416985f8 <.text+0x16975f8>
14169858b: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
141698592: 48 8b 10                    	movq	(%rax), %rdx
141698595: 48 8b 48 08                 	movq	0x8(%rax), %rcx
141698599: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1416985a0: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1416985a7: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
1416985ae: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
1416985b5: 48 c7 85 60 02 00 00 00 00 00 00    	movq	$0x0, 0x260(%rbp)
1416985c0: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416985c7: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416985ce: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
1416985d5: 48 8d 85 50 02 00 00        	leaq	0x250(%rbp), %rax
1416985dc: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1416985e1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1416985ea: 4d 89 e0                    	movq	%r12, %r8
1416985ed: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1416985f3: e8 88 cd 00 00              	callq	0x1416a5380 <.text+0x16a4380>
1416985f8: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416985ff: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698606: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169860d: 4c 89 e1                    	movq	%r12, %rcx
141698610: e8 6b 57 fe fe              	callq	0x14067dd80 <.text+0x67cd80>
141698615: 48 8b 40 18                 	movq	0x18(%rax), %rax
141698619: 48 83 78 50 03              	cmpq	$0x3, 0x50(%rax)
14169861e: 72 48                       	jb	0x141698668 <.text+0x1697668>
141698620: 48 8b 40 48                 	movq	0x48(%rax), %rax
141698624: 48 83 78 78 00              	cmpq	$0x0, 0x78(%rax)
141698629: 74 3d                       	je	0x141698668 <.text+0x1697668>
14169862b: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
141698632: 48 8b 10                    	movq	(%rax), %rdx
141698635: 48 8b 48 08                 	movq	0x8(%rax), %rcx
141698639: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698640: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698647: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169864e: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
141698655: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14169865a: 4d 89 e0                    	movq	%r12, %r8
14169865d: 41 b9 02 00 00 00           	movl	$0x2, %r9d
141698663: e8 e8 5a 55 ff              	callq	0x140bee150 <.text+0xbed150>
141698668: 4c 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %r9
14169866f: 4d 85 c9                    	testq	%r9, %r9
141698672: 74 2b                       	je	0x14169869f <.text+0x169769f>
141698674: 4c 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %r8
14169867b: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698682: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698689: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698690: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141698697: 4c 89 e2                    	movq	%r12, %rdx
14169869a: e8 b1 f3 05 00              	callq	0x1416f7a50 <.text+0x16f6a50>
14169869f: f6 85 39 02 00 00 01        	testb	$0x1, 0x239(%rbp)
1416986a6: 74 28                       	je	0x1416986d0 <.text+0x16976d0>
1416986a8: 0f b6 95 41 02 00 00        	movzbl	0x241(%rbp), %edx
1416986af: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416986b6: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416986bd: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
1416986c4: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
1416986cb: e8 50 f2 05 00              	callq	0x1416f7920 <.text+0x16f6920>
1416986d0: 0f b6 85 38 02 00 00        	movzbl	0x238(%rbp), %eax
1416986d7: 83 e0 03                    	andl	$0x3, %eax
1416986da: 74 34                       	je	0x141698710 <.text+0x1697710>
1416986dc: 83 f8 03                    	cmpl	$0x3, %eax
1416986df: 0f 84 e1 0f 00 00           	je	0x1416996c6 <.text+0x16986c6>
1416986e5: 66 0f 6e 8d 3c 02 00 00     	movd	0x23c(%rbp), %xmm1
1416986ed: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416986f4: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416986fb: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698702: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141698709: e8 52 f2 05 00              	callq	0x1416f7960 <.text+0x16f6960>
14169870e: eb 40                       	jmp	0x141698750 <.text+0x1697750>
141698710: 80 bd 3b 02 00 00 00        	cmpb	$0x0, 0x23b(%rbp)
141698717: 79 37                       	jns	0x141698750 <.text+0x1697750>
141698719: 66 0f 6e 8d 3c 02 00 00     	movd	0x23c(%rbp), %xmm1
141698721: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698728: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169872f: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698736: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
14169873d: e8 1e f2 05 00              	callq	0x1416f7960 <.text+0x16f6960>
141698742: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141698750: 8b 85 d0 01 00 00           	movl	0x1d0(%rbp), %eax
141698756: 48 8d 0d 03 f4 5e 01        	leaq	0x15ef403(%rip), %rcx   # 0x142c87b60
14169875d: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
141698761: 48 01 c8                    	addq	%rcx, %rax
141698764: ff e0                       	jmpq	*%rax
141698766: 8b 9d d4 01 00 00           	movl	0x1d4(%rbp), %ebx
14169876c: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
141698773: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698777: 80 b8 b9 0a 00 00 01        	cmpb	$0x1, 0xab9(%rax)
14169877e: 75 2f                       	jne	0x1416987af <.text+0x16977af>
141698780: 49 8b 0f                    	movq	(%r15), %rcx
141698783: 48 8b 51 48                 	movq	0x48(%rcx), %rdx
141698787: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169878e: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698795: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169879c: 48 89 c1                    	movq	%rax, %rcx
14169879f: e8 ec 58 55 ff              	callq	0x140bee090 <.text+0xbed090>
1416987a4: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
1416987ab: 49 8b 47 08                 	movq	0x8(%r15), %rax
1416987af: f0                          	lock
1416987b0: 48 ff 80 a8 0a 00 00        	incq	0xaa8(%rax)
1416987b7: 49 8b 0f                    	movq	(%r15), %rcx
1416987ba: 48 8b 49 48                 	movq	0x48(%rcx), %rcx
1416987be: c7 44 24 20 00 00 00 00     	movl	$0x0, 0x20(%rsp)
1416987c6: 89 da                       	movl	%ebx, %edx
1416987c8: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416987ce: e9 cd 04 00 00              	jmp	0x141698ca0 <.text+0x1697ca0>
1416987d3: 08 d0                       	orb	%dl, %al
1416987d5: a8 01                       	testb	$0x1, %al
1416987d7: 0f 84 73 f9 ff ff           	je	0x141698150 <.text+0x1697150>
1416987dd: 4d 01 d3                    	addq	%r10, %r11
1416987e0: 49 8b 4b c0                 	movq	-0x40(%r11), %rcx
1416987e4: 4c 89 9d e0 00 00 00        	movq	%r11, 0xe0(%rbp)
1416987eb: 4d 6b 73 c8 38              	imulq	$0x38, -0x38(%r11), %r14
1416987f0: 49 01 ce                    	addq	%rcx, %r14
1416987f3: 4c 89 a5 f8 02 00 00        	movq	%r12, 0x2f8(%rbp)
1416987fa: 4c 89 b5 e0 02 00 00        	movq	%r14, 0x2e0(%rbp)
141698801: e9 0c 01 00 00              	jmp	0x141698912 <.text+0x1697912>
141698806: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141698810: 48 21 d6                    	andq	%rdx, %rsi
141698813: f3 44 0f 6f 1c 37           	movdqu	(%rdi,%rsi), %xmm11
141698819: 66 41 0f 6f c3              	movdqa	%xmm11, %xmm0
14169881e: 66 41 0f 74 c2              	pcmpeqb	%xmm10, %xmm0
141698823: 66 44 0f d7 e0              	pmovmskb	%xmm0, %r12d
141698828: 45 85 e4                    	testl	%r12d, %r12d
14169882b: 74 73                       	je	0x1416988a0 <.text+0x16978a0>
14169882d: 48 89 95 b0 00 00 00        	movq	%rdx, 0xb0(%rbp)
141698834: 4c 89 85 b8 00 00 00        	movq	%r8, 0xb8(%rbp)
14169883b: f3 41 0f bc c4              	tzcntl	%r12d, %eax
141698840: 48 01 f0                    	addq	%rsi, %rax
141698843: 48 21 d0                    	andq	%rdx, %rax
141698846: 48 f7 d8                    	negq	%rax
141698849: 4c 8d 34 80                 	leaq	(%rax,%rax,4), %r14
14169884d: 49 c1 e6 04                 	shlq	$0x4, %r14
141698851: 4e 39 6c 33 10              	cmpq	%r13, 0x10(%rbx,%r14)
141698856: 75 1b                       	jne	0x141698873 <.text+0x1697873>
141698858: 4a 8d 04 33                 	leaq	(%rbx,%r14), %rax
14169885c: 48 8b 50 08                 	movq	0x8(%rax), %rdx
141698860: 48 8b 8d d0 02 00 00        	movq	0x2d0(%rbp), %rcx
141698867: 4d 89 e8                    	movq	%r13, %r8
14169886a: e8 81 b3 28 01              	callq	0x142923bf0 <.text+0x2922bf0>
14169886f: 85 c0                       	testl	%eax, %eax
141698871: 74 5d                       	je	0x1416988d0 <.text+0x16978d0>
141698873: 41 8d 44 24 ff              	leal	-0x1(%r12), %eax
141698878: 66 44 21 e0                 	andw	%r12w, %ax
14169887c: 41 89 c4                    	movl	%eax, %r12d
14169887f: 4c 8b b5 e0 02 00 00        	movq	0x2e0(%rbp), %r14
141698886: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
14169888d: 4c 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %r8
141698894: 75 a5                       	jne	0x14169883b <.text+0x169783b>
141698896: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1416988a0: 66 44 0f 74 de              	pcmpeqb	%xmm6, %xmm11
1416988a5: 66 41 0f d7 c3              	pmovmskb	%xmm11, %eax
1416988aa: 85 c0                       	testl	%eax, %eax
1416988ac: 0f 85 5e 02 00 00           	jne	0x141698b10 <.text+0x1697b10>
1416988b2: 4c 01 c6                    	addq	%r8, %rsi
1416988b5: 48 83 c6 10                 	addq	$0x10, %rsi
1416988b9: 49 83 c0 10                 	addq	$0x10, %r8
1416988bd: e9 4e ff ff ff              	jmp	0x141698810 <.text+0x1697810>
1416988c2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1416988d0: 42 8b 54 37 f8              	movl	-0x8(%rdi,%r14), %edx
1416988d5: 49 83 c7 10                 	addq	$0x10, %r15
1416988d9: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416988e0: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416988e7: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
1416988ee: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
1416988f5: 4d 89 f8                    	movq	%r15, %r8
1416988f8: e8 83 f0 05 00              	callq	0x1416f7980 <.text+0x16f6980>
1416988fd: 4c 8b a5 f8 02 00 00        	movq	0x2f8(%rbp), %r12
141698904: 4c 8b b5 e0 02 00 00        	movq	0x2e0(%rbp), %r14
14169890b: 48 8b 8d e8 02 00 00        	movq	0x2e8(%rbp), %rcx
141698912: 4c 39 f1                    	cmpq	%r14, %rcx
141698915: 0f 84 08 02 00 00           	je	0x141698b23 <.text+0x1697b23>
14169891b: 49 89 cf                    	movq	%rcx, %r15
14169891e: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698925: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169892c: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141698933: 4c 89 e1                    	movq	%r12, %rcx
141698936: e8 45 54 fe fe              	callq	0x14067dd80 <.text+0x67cd80>
14169893b: 49 8d 4f 38                 	leaq	0x38(%r15), %rcx
14169893f: 48 8b 58 18                 	movq	0x18(%rax), %rbx
141698943: 48 83 bb d0 00 00 00 00     	cmpq	$0x0, 0xd0(%rbx)
14169894b: 74 c5                       	je	0x141698912 <.text+0x1697912>
14169894d: 48 89 8d e8 02 00 00        	movq	%rcx, 0x2e8(%rbp)
141698954: 49 8b 17                    	movq	(%r15), %rdx
141698957: 4d 8b 6f 08                 	movq	0x8(%r15), %r13
14169895b: 48 83 c2 10                 	addq	$0x10, %rdx
14169895f: f3 0f 6f 83 d8 00 00 00     	movdqu	0xd8(%rbx), %xmm0
141698967: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
14169896c: 66 0f ef cf                 	pxor	%xmm7, %xmm1
141698970: 66 0f 7f 8d 50 02 00 00     	movdqa	%xmm1, 0x250(%rbp)
141698978: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
14169897d: 66 41 0f ef c8              	pxor	%xmm8, %xmm1
141698982: 66 0f 7f 8d 60 02 00 00     	movdqa	%xmm1, 0x260(%rbp)
14169898a: 66 0f 7f 85 70 02 00 00     	movdqa	%xmm0, 0x270(%rbp)
141698992: 48 8d 85 80 02 00 00        	leaq	0x280(%rbp), %rax
141698999: 44 0f 29 08                 	movaps	%xmm9, (%rax)
14169899d: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
1416989a5: 48 8d b5 50 02 00 00        	leaq	0x250(%rbp), %rsi
1416989ac: 48 89 f1                    	movq	%rsi, %rcx
1416989af: 48 89 95 d0 02 00 00        	movq	%rdx, 0x2d0(%rbp)
1416989b6: 4d 89 e8                    	movq	%r13, %r8
1416989b9: e8 a2 d7 a6 fe              	callq	0x140106160 <.text+0x105160>
1416989be: c6 85 10 01 00 00 ff        	movb	$-0x1, 0x110(%rbp)
1416989c5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416989cb: 48 89 f1                    	movq	%rsi, %rcx
1416989ce: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
1416989d5: e8 86 d7 a6 fe              	callq	0x140106160 <.text+0x105160>
1416989da: 4c 8b 8d 80 02 00 00        	movq	0x280(%rbp), %r9
1416989e1: 49 c1 e1 38                 	shlq	$0x38, %r9
1416989e5: 4c 0b 8d 88 02 00 00        	orq	0x288(%rbp), %r9
1416989ec: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
1416989f3: 48 8b 85 68 02 00 00        	movq	0x268(%rbp), %rax
1416989fa: 4c 31 c8                    	xorq	%r9, %rax
1416989fd: 4c 8b 85 50 02 00 00        	movq	0x250(%rbp), %r8
141698a04: 49 01 c8                    	addq	%rcx, %r8
141698a07: 48 8b 95 58 02 00 00        	movq	0x258(%rbp), %rdx
141698a0e: 48 c1 c1 0d                 	rolq	$0xd, %rcx
141698a12: 48 01 c2                    	addq	%rax, %rdx
141698a15: 4c 31 c1                    	xorq	%r8, %rcx
141698a18: 48 c1 c0 10                 	rolq	$0x10, %rax
141698a1c: 48 31 d0                    	xorq	%rdx, %rax
141698a1f: 49 c1 c0 20                 	rolq	$0x20, %r8
141698a23: 48 01 ca                    	addq	%rcx, %rdx
141698a26: 49 01 c0                    	addq	%rax, %r8
141698a29: 48 c1 c1 11                 	rolq	$0x11, %rcx
141698a2d: 48 31 d1                    	xorq	%rdx, %rcx
141698a30: 48 c1 c0 15                 	rolq	$0x15, %rax
141698a34: 48 c1 c2 20                 	rolq	$0x20, %rdx
141698a38: 4c 31 c0                    	xorq	%r8, %rax
141698a3b: 4d 31 c8                    	xorq	%r9, %r8
141698a3e: 48 81 f2 ff 00 00 00        	xorq	$0xff, %rdx
141698a45: 49 01 c8                    	addq	%rcx, %r8
141698a48: 48 01 c2                    	addq	%rax, %rdx
141698a4b: 48 c1 c1 0d                 	rolq	$0xd, %rcx
141698a4f: 4c 31 c1                    	xorq	%r8, %rcx
141698a52: 48 c1 c0 10                 	rolq	$0x10, %rax
141698a56: 49 c1 c0 20                 	rolq	$0x20, %r8
141698a5a: 48 31 d0                    	xorq	%rdx, %rax
141698a5d: 48 01 ca                    	addq	%rcx, %rdx
141698a60: 48 c1 c1 11                 	rolq	$0x11, %rcx
141698a64: 49 01 c0                    	addq	%rax, %r8
141698a67: 48 31 d1                    	xorq	%rdx, %rcx
141698a6a: 48 c1 c0 15                 	rolq	$0x15, %rax
141698a6e: 4c 31 c0                    	xorq	%r8, %rax
141698a71: 48 c1 c2 20                 	rolq	$0x20, %rdx
141698a75: 49 01 c8                    	addq	%rcx, %r8
141698a78: 48 01 c2                    	addq	%rax, %rdx
141698a7b: 48 c1 c1 0d                 	rolq	$0xd, %rcx
141698a7f: 4c 31 c1                    	xorq	%r8, %rcx
141698a82: 48 c1 c0 10                 	rolq	$0x10, %rax
141698a86: 49 c1 c0 20                 	rolq	$0x20, %r8
141698a8a: 48 31 d0                    	xorq	%rdx, %rax
141698a8d: 48 01 ca                    	addq	%rcx, %rdx
141698a90: 48 c1 c1 11                 	rolq	$0x11, %rcx
141698a94: 49 01 c0                    	addq	%rax, %r8
141698a97: 48 31 d1                    	xorq	%rdx, %rcx
141698a9a: 48 c1 c0 15                 	rolq	$0x15, %rax
141698a9e: 4c 31 c0                    	xorq	%r8, %rax
141698aa1: 48 c1 c2 20                 	rolq	$0x20, %rdx
141698aa5: 49 01 c8                    	addq	%rcx, %r8
141698aa8: 48 01 c2                    	addq	%rax, %rdx
141698aab: 48 c1 c1 0d                 	rolq	$0xd, %rcx
141698aaf: 4c 31 c1                    	xorq	%r8, %rcx
141698ab2: 48 c1 c0 10                 	rolq	$0x10, %rax
141698ab6: 48 31 d0                    	xorq	%rdx, %rax
141698ab9: 48 01 ca                    	addq	%rcx, %rdx
141698abc: 48 c1 c1 11                 	rolq	$0x11, %rcx
141698ac0: 48 c1 c0 15                 	rolq	$0x15, %rax
141698ac4: 48 89 d6                    	movq	%rdx, %rsi
141698ac7: 48 c1 c6 20                 	rolq	$0x20, %rsi
141698acb: 48 31 ce                    	xorq	%rcx, %rsi
141698ace: 48 31 c6                    	xorq	%rax, %rsi
141698ad1: 48 31 d6                    	xorq	%rdx, %rsi
141698ad4: 48 89 f0                    	movq	%rsi, %rax
141698ad7: 48 c1 e8 39                 	shrq	$0x39, %rax
141698adb: 48 8b bb b8 00 00 00        	movq	0xb8(%rbx), %rdi
141698ae2: 48 8b 93 c0 00 00 00        	movq	0xc0(%rbx), %rdx
141698ae9: 66 0f 6e c0                 	movd	%eax, %xmm0
141698aed: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141698af1: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141698af6: 66 44 0f 70 d0 00           	pshufd	$0x0, %xmm0, %xmm10     # xmm10 = xmm0[0,0,0,0]
141698afc: 48 8d 5f b0                 	leaq	-0x50(%rdi), %rbx
141698b00: 45 31 c0                    	xorl	%r8d, %r8d
141698b03: e9 08 fd ff ff              	jmp	0x141698810 <.text+0x1697810>
141698b08: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141698b10: 4c 8b a5 f8 02 00 00        	movq	0x2f8(%rbp), %r12
141698b17: 48 8b 8d e8 02 00 00        	movq	0x2e8(%rbp), %rcx
141698b1e: e9 ef fd ff ff              	jmp	0x141698912 <.text+0x1697912>
141698b23: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
141698b2a: 48 83 c2 d0                 	addq	$-0x30, %rdx
141698b2e: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698b35: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698b3c: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141698b43: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141698b4a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141698b50: e8 9b ee 05 00              	callq	0x1416f79f0 <.text+0x16f69f0>
141698b55: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
141698b5c: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
141698b63: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
141698b6a: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
141698b71: 80 3d 20 c1 f9 01 00        	cmpb	$0x0, 0x1f9c120(%rip)   # 0x143634c98
141698b78: 0f 85 df f5 ff ff           	jne	0x14169815d <.text+0x169715d>
141698b7e: e9 51 f8 ff ff              	jmp	0x1416983d4 <.text+0x16973d4>
141698b83: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
141698b8a: 48 89 85 e0 02 00 00        	movq	%rax, 0x2e0(%rbp)
141698b91: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
141698b98: 48 89 85 f8 02 00 00        	movq	%rax, 0x2f8(%rbp)
141698b9f: 80 bd 78 01 00 00 00        	cmpb	$0x0, 0x178(%rbp)
141698ba6: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
141698bad: 0f 84 3e 0c 00 00           	je	0x1416997f1 <.text+0x16987f1>
141698bb3: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
141698bba: 48 85 c0                    	testq	%rax, %rax
141698bbd: 0f 84 0f 05 00 00           	je	0x1416990d2 <.text+0x16980d2>
141698bc3: 48 8b bd f8 02 00 00        	movq	0x2f8(%rbp), %rdi
141698bca: 48 8d 34 c7                 	leaq	(%rdi,%rax,8), %rsi
141698bce: eb 3c                       	jmp	0x141698c0c <.text+0x1697c0c>
141698bd0: f0                          	lock
141698bd1: 48 ff 80 a8 0a 00 00        	incq	0xaa8(%rax)
141698bd8: 49 8b 0f                    	movq	(%r15), %rcx
141698bdb: 48 8b 49 48                 	movq	0x48(%rcx), %rcx
141698bdf: 44 89 74 24 28              	movl	%r14d, 0x28(%rsp)
141698be4: c7 44 24 20 00 00 00 00     	movl	$0x0, 0x20(%rsp)
141698bec: 48 8b 95 d8 02 00 00        	movq	0x2d8(%rbp), %rdx
141698bf3: 41 89 d8                    	movl	%ebx, %r8d
141698bf6: 45 31 c9                    	xorl	%r9d, %r9d
141698bf9: ff 90 48 03 00 00           	callq	*0x348(%rax)
141698bff: 48 83 c7 08                 	addq	$0x8, %rdi
141698c03: 48 39 f7                    	cmpq	%rsi, %rdi
141698c06: 0f 84 c6 04 00 00           	je	0x1416990d2 <.text+0x16980d2>
141698c0c: 44 8b 37                    	movl	(%rdi), %r14d
141698c0f: 8b 5f 04                    	movl	0x4(%rdi), %ebx
141698c12: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698c16: 80 b8 b9 0a 00 00 01        	cmpb	$0x1, 0xab9(%rax)
141698c1d: 75 b1                       	jne	0x141698bd0 <.text+0x1697bd0>
141698c1f: 49 8b 0f                    	movq	(%r15), %rcx
141698c22: 48 8b 51 48                 	movq	0x48(%rcx), %rdx
141698c26: 48 89 c1                    	movq	%rax, %rcx
141698c29: e8 62 54 55 ff              	callq	0x140bee090 <.text+0xbed090>
141698c2e: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698c32: eb 9c                       	jmp	0x141698bd0 <.text+0x1697bd0>
141698c34: 8b 9d d4 01 00 00           	movl	0x1d4(%rbp), %ebx
141698c3a: 44 8b b5 d8 01 00 00        	movl	0x1d8(%rbp), %r14d
141698c41: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
141698c48: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698c4c: 80 b8 b9 0a 00 00 01        	cmpb	$0x1, 0xab9(%rax)
141698c53: 75 2f                       	jne	0x141698c84 <.text+0x1697c84>
141698c55: 49 8b 0f                    	movq	(%r15), %rcx
141698c58: 48 8b 51 48                 	movq	0x48(%rcx), %rdx
141698c5c: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698c63: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698c6a: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698c71: 48 89 c1                    	movq	%rax, %rcx
141698c74: e8 17 54 55 ff              	callq	0x140bee090 <.text+0xbed090>
141698c79: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
141698c80: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698c84: f0                          	lock
141698c85: 48 ff 80 a8 0a 00 00        	incq	0xaa8(%rax)
141698c8c: 49 8b 0f                    	movq	(%r15), %rcx
141698c8f: 48 8b 49 48                 	movq	0x48(%rcx), %rcx
141698c93: c7 44 24 20 00 00 00 00     	movl	$0x0, 0x20(%rsp)
141698c9b: 89 da                       	movl	%ebx, %edx
141698c9d: 45 89 f0                    	movl	%r14d, %r8d
141698ca0: 45 31 c9                    	xorl	%r9d, %r9d
141698ca3: ff 90 40 03 00 00           	callq	*0x340(%rax)
141698ca9: e9 88 00 00 00              	jmp	0x141698d36 <.text+0x1697d36>
141698cae: 80 bd 78 01 00 00 00        	cmpb	$0x0, 0x178(%rbp)
141698cb5: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
141698cbc: 0f 84 4c 0b 00 00           	je	0x14169980e <.text+0x169880e>
141698cc2: 8b 9d d4 01 00 00           	movl	0x1d4(%rbp), %ebx
141698cc8: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698ccc: 80 b8 b9 0a 00 00 01        	cmpb	$0x1, 0xab9(%rax)
141698cd3: 75 2f                       	jne	0x141698d04 <.text+0x1697d04>
141698cd5: 49 8b 0f                    	movq	(%r15), %rcx
141698cd8: 48 8b 51 48                 	movq	0x48(%rcx), %rdx
141698cdc: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698ce3: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698cea: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698cf1: 48 89 c1                    	movq	%rax, %rcx
141698cf4: e8 97 53 55 ff              	callq	0x140bee090 <.text+0xbed090>
141698cf9: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
141698d00: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698d04: f0                          	lock
141698d05: 48 ff 80 a8 0a 00 00        	incq	0xaa8(%rax)
141698d0c: 49 8b 0f                    	movq	(%r15), %rcx
141698d0f: 48 8b 49 48                 	movq	0x48(%rcx), %rcx
141698d13: c7 44 24 28 00 00 00 00     	movl	$0x0, 0x28(%rsp)
141698d1b: c7 44 24 20 00 00 00 00     	movl	$0x0, 0x20(%rsp)
141698d23: 48 8b 95 d8 02 00 00        	movq	0x2d8(%rbp), %rdx
141698d2a: 41 89 d8                    	movl	%ebx, %r8d
141698d2d: 45 31 c9                    	xorl	%r9d, %r9d
141698d30: ff 90 48 03 00 00           	callq	*0x348(%rax)
141698d36: 40 b6 01                    	movb	$0x1, %sil
141698d39: b0 01                       	movb	$0x1, %al
141698d3b: e9 a5 05 00 00              	jmp	0x1416992e5 <.text+0x16982e5>
141698d40: 44 8b ad d4 01 00 00        	movl	0x1d4(%rbp), %r13d
141698d47: 48 8b 9d d8 01 00 00        	movq	0x1d8(%rbp), %rbx
141698d4e: 4c 8b b5 e0 01 00 00        	movq	0x1e0(%rbp), %r14
141698d55: 48 8b b5 e8 01 00 00        	movq	0x1e8(%rbp), %rsi
141698d5c: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141698d63: 4c 8b 79 08                 	movq	0x8(%rcx), %r15
141698d67: 41 80 bf b9 0a 00 00 00     	cmpb	$0x0, 0xab9(%r15)
141698d6f: 74 2f                       	je	0x141698da0 <.text+0x1697da0>
141698d71: 48 8b 01                    	movq	(%rcx), %rax
141698d74: 48 8b 50 48                 	movq	0x48(%rax), %rdx
141698d78: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698d7f: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698d86: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698d8d: 4c 89 f9                    	movq	%r15, %rcx
141698d90: e8 fb 52 55 ff              	callq	0x140bee090 <.text+0xbed090>
141698d95: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141698d9c: 4c 8b 79 08                 	movq	0x8(%rcx), %r15
141698da0: f0                          	lock
141698da1: 49 ff 87 a8 0a 00 00        	incq	0xaa8(%r15)
141698da8: 48 8b 01                    	movq	(%rcx), %rax
141698dab: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
141698daf: 4c 8b 60 48                 	movq	0x48(%rax), %r12
141698db3: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698dba: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698dc1: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698dc8: 44 89 ea                    	movl	%r13d, %edx
141698dcb: e8 00 ec 54 ff              	callq	0x140be79d0 <.text+0xbe69d0>
141698dd0: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698dd7: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698dde: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698de5: 4c 89 f9                    	movq	%r15, %rcx
141698de8: 48 89 c2                    	movq	%rax, %rdx
141698deb: 45 31 c0                    	xorl	%r8d, %r8d
141698dee: e8 7d 66 55 ff              	callq	0x140bef470 <.text+0xbee470>
141698df3: a8 01                       	testb	$0x1, %al
141698df5: 0f 84 88 0a 00 00           	je	0x141699883 <.text+0x1698883>
141698dfb: 89 74 24 20                 	movl	%esi, 0x20(%rsp)
141698dff: 4c 89 e1                    	movq	%r12, %rcx
141698e02: 49 89 d8                    	movq	%rbx, %r8
141698e05: 45 89 f1                    	movl	%r14d, %r9d
141698e08: 41 ff 97 58 03 00 00        	callq	*0x358(%r15)
141698e0f: e9 c5 04 00 00              	jmp	0x1416992d9 <.text+0x16982d9>
141698e14: 48 8d 8d d8 01 00 00        	leaq	0x1d8(%rbp), %rcx
141698e1b: 48 8b 41 10                 	movq	0x10(%rcx), %rax
141698e1f: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
141698e26: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
141698e2a: 66 0f 7f 85 50 02 00 00     	movdqa	%xmm0, 0x250(%rbp)
141698e32: 8b 9d d4 01 00 00           	movl	0x1d4(%rbp), %ebx
141698e38: 48 8b b5 08 03 00 00        	movq	0x308(%rbp), %rsi
141698e3f: 48 89 f1                    	movq	%rsi, %rcx
141698e42: 48 8d 95 50 02 00 00        	leaq	0x250(%rbp), %rdx
141698e49: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141698e4f: e8 9c eb 05 00              	callq	0x1416f79f0 <.text+0x16f69f0>
141698e54: 48 8b 46 08                 	movq	0x8(%rsi), %rax
141698e58: 80 b8 b9 0a 00 00 01        	cmpb	$0x1, 0xab9(%rax)
141698e5f: 75 13                       	jne	0x141698e74 <.text+0x1697e74>
141698e61: 48 8b 0e                    	movq	(%rsi), %rcx
141698e64: 48 8b 51 48                 	movq	0x48(%rcx), %rdx
141698e68: 48 89 c1                    	movq	%rax, %rcx
141698e6b: e8 20 52 55 ff              	callq	0x140bee090 <.text+0xbed090>
141698e70: 48 8b 46 08                 	movq	0x8(%rsi), %rax
141698e74: f0                          	lock
141698e75: 48 ff 80 a8 0a 00 00        	incq	0xaa8(%rax)
141698e7c: 48 8b 0e                    	movq	(%rsi), %rcx
141698e7f: 48 8b 49 48                 	movq	0x48(%rcx), %rcx
141698e83: c7 44 24 28 00 00 00 00     	movl	$0x0, 0x28(%rsp)
141698e8b: c7 44 24 20 00 00 00 00     	movl	$0x0, 0x20(%rsp)
141698e93: 89 da                       	movl	%ebx, %edx
141698e95: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141698e9b: 45 31 c9                    	xorl	%r9d, %r9d
141698e9e: ff 90 48 03 00 00           	callq	*0x348(%rax)
141698ea4: c6 85 17 03 00 00 00        	movb	$0x0, 0x317(%rbp)
141698eab: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698eb2: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698eb9: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
141698ec0: e8 eb 95 54 ff              	callq	0x140be24b0 <.text+0xbe14b0>
141698ec5: b0 01                       	movb	$0x1, %al
141698ec7: 31 f6                       	xorl	%esi, %esi
141698ec9: e9 10 04 00 00              	jmp	0x1416992de <.text+0x16982de>
141698ece: 44 8b ad d4 01 00 00        	movl	0x1d4(%rbp), %r13d
141698ed5: 48 8b 9d d8 01 00 00        	movq	0x1d8(%rbp), %rbx
141698edc: 4c 8b b5 e0 01 00 00        	movq	0x1e0(%rbp), %r14
141698ee3: 48 8b b5 e8 01 00 00        	movq	0x1e8(%rbp), %rsi
141698eea: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141698ef1: 4c 8b 79 08                 	movq	0x8(%rcx), %r15
141698ef5: 41 80 bf b9 0a 00 00 00     	cmpb	$0x0, 0xab9(%r15)
141698efd: 74 2f                       	je	0x141698f2e <.text+0x1697f2e>
141698eff: 48 8b 01                    	movq	(%rcx), %rax
141698f02: 48 8b 50 48                 	movq	0x48(%rax), %rdx
141698f06: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698f0d: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698f14: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698f1b: 4c 89 f9                    	movq	%r15, %rcx
141698f1e: e8 6d 51 55 ff              	callq	0x140bee090 <.text+0xbed090>
141698f23: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141698f2a: 4c 8b 79 08                 	movq	0x8(%rcx), %r15
141698f2e: f0                          	lock
141698f2f: 49 ff 87 a8 0a 00 00        	incq	0xaa8(%r15)
141698f36: 48 8b 01                    	movq	(%rcx), %rax
141698f39: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
141698f3d: 4c 8b 60 48                 	movq	0x48(%rax), %r12
141698f41: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698f48: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698f4f: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698f56: 44 89 ea                    	movl	%r13d, %edx
141698f59: e8 72 ea 54 ff              	callq	0x140be79d0 <.text+0xbe69d0>
141698f5e: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698f65: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698f6c: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698f73: 4c 89 f9                    	movq	%r15, %rcx
141698f76: 48 89 c2                    	movq	%rax, %rdx
141698f79: 45 31 c0                    	xorl	%r8d, %r8d
141698f7c: e8 ef 64 55 ff              	callq	0x140bef470 <.text+0xbee470>
141698f81: a8 01                       	testb	$0x1, %al
141698f83: 0f 84 42 08 00 00           	je	0x1416997cb <.text+0x16987cb>
141698f89: 89 74 24 20                 	movl	%esi, 0x20(%rsp)
141698f8d: 4c 89 e1                    	movq	%r12, %rcx
141698f90: 49 89 d8                    	movq	%rbx, %r8
141698f93: 45 89 f1                    	movl	%r14d, %r9d
141698f96: 41 ff 97 50 03 00 00        	callq	*0x350(%r15)
141698f9d: e9 37 03 00 00              	jmp	0x1416992d9 <.text+0x16982d9>
141698fa2: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
141698fa9: 48 8b 58 08                 	movq	0x8(%rax), %rbx
141698fad: 80 bb d8 0a 00 00 00        	cmpb	$0x0, 0xad8(%rbx)
141698fb4: 0f 84 5c 06 00 00           	je	0x141699616 <.text+0x1698616>
141698fba: 44 8b ad d4 01 00 00        	movl	0x1d4(%rbp), %r13d
141698fc1: 44 8b a5 d8 01 00 00        	movl	0x1d8(%rbp), %r12d
141698fc8: 48 8b bd e0 01 00 00        	movq	0x1e0(%rbp), %rdi
141698fcf: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
141698fd6: 48 89 85 d8 02 00 00        	movq	%rax, 0x2d8(%rbp)
141698fdd: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
141698fe4: 48 89 85 f8 02 00 00        	movq	%rax, 0x2f8(%rbp)
141698feb: 48 8b b5 f8 01 00 00        	movq	0x1f8(%rbp), %rsi
141698ff2: 80 bb b9 0a 00 00 00        	cmpb	$0x0, 0xab9(%rbx)
141698ff9: 0f 84 03 01 00 00           	je	0x141699102 <.text+0x1698102>
141698fff: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
141699006: 48 8b 00                    	movq	(%rax), %rax
141699009: 48 8b 50 48                 	movq	0x48(%rax), %rdx
14169900d: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699014: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169901b: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699022: 48 89 d9                    	movq	%rbx, %rcx
141699025: e8 66 50 55 ff              	callq	0x140bee090 <.text+0xbed090>
14169902a: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141699031: 48 8b 59 08                 	movq	0x8(%rcx), %rbx
141699035: e9 cf 00 00 00              	jmp	0x141699109 <.text+0x1698109>
14169903a: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
141699041: 48 8b 58 08                 	movq	0x8(%rax), %rbx
141699045: 80 bb d8 0a 00 00 00        	cmpb	$0x0, 0xad8(%rbx)
14169904c: 0f 84 f6 05 00 00           	je	0x141699648 <.text+0x1698648>
141699052: 44 8b ad d4 01 00 00        	movl	0x1d4(%rbp), %r13d
141699059: 44 8b a5 d8 01 00 00        	movl	0x1d8(%rbp), %r12d
141699060: 48 8b bd e0 01 00 00        	movq	0x1e0(%rbp), %rdi
141699067: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
14169906e: 48 89 85 d8 02 00 00        	movq	%rax, 0x2d8(%rbp)
141699075: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
14169907c: 48 89 85 f8 02 00 00        	movq	%rax, 0x2f8(%rbp)
141699083: 48 8b b5 f8 01 00 00        	movq	0x1f8(%rbp), %rsi
14169908a: 80 bb b9 0a 00 00 00        	cmpb	$0x0, 0xab9(%rbx)
141699091: 0f 84 59 01 00 00           	je	0x1416991f0 <.text+0x16981f0>
141699097: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
14169909e: 48 8b 00                    	movq	(%rax), %rax
1416990a1: 48 8b 50 48                 	movq	0x48(%rax), %rdx
1416990a5: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416990ac: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416990b3: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
1416990ba: 48 89 d9                    	movq	%rbx, %rcx
1416990bd: e8 ce 4f 55 ff              	callq	0x140bee090 <.text+0xbed090>
1416990c2: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
1416990c9: 48 8b 59 08                 	movq	0x8(%rcx), %rbx
1416990cd: e9 25 01 00 00              	jmp	0x1416991f7 <.text+0x16981f7>
1416990d2: 40 b6 01                    	movb	$0x1, %sil
1416990d5: 48 8b 95 e0 02 00 00        	movq	0x2e0(%rbp), %rdx
1416990dc: 48 85 d2                    	testq	%rdx, %rdx
1416990df: 0f 84 6d 02 00 00           	je	0x141699352 <.text+0x1698352>
1416990e5: 48 c1 e2 03                 	shlq	$0x3, %rdx
1416990e9: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1416990ef: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1416990f6: e8 25 ab f7 fe              	callq	0x140613c20 <.text+0x612c20>
1416990fb: 31 c0                       	xorl	%eax, %eax
1416990fd: e9 e3 01 00 00              	jmp	0x1416992e5 <.text+0x16982e5>
141699102: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141699109: f0                          	lock
14169910a: 48 ff 83 a8 0a 00 00        	incq	0xaa8(%rbx)
141699111: 48 8b 01                    	movq	(%rcx), %rax
141699114: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
141699118: 4c 8b 78 48                 	movq	0x48(%rax), %r15
14169911c: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699123: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169912a: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699131: 44 89 ea                    	movl	%r13d, %edx
141699134: e8 97 e8 54 ff              	callq	0x140be79d0 <.text+0xbe69d0>
141699139: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699140: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699147: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169914e: 48 89 d9                    	movq	%rbx, %rcx
141699151: 48 89 c2                    	movq	%rax, %rdx
141699154: 45 31 c0                    	xorl	%r8d, %r8d
141699157: e8 14 63 55 ff              	callq	0x140bef470 <.text+0xbee470>
14169915c: a8 01                       	testb	$0x1, %al
14169915e: 0f 84 1b 06 00 00           	je	0x14169977f <.text+0x169877f>
141699164: 49 89 d5                    	movq	%rdx, %r13
141699167: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
14169916e: 4c 8b 70 08                 	movq	0x8(%rax), %r14
141699172: 48 8b 48 10                 	movq	0x10(%rax), %rcx
141699176: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169917d: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699184: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169918b: 44 89 e2                    	movl	%r12d, %edx
14169918e: e8 3d e8 54 ff              	callq	0x140be79d0 <.text+0xbe69d0>
141699193: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169919a: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416991a1: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
1416991a8: 4c 89 f1                    	movq	%r14, %rcx
1416991ab: 48 89 c2                    	movq	%rax, %rdx
1416991ae: 45 31 c0                    	xorl	%r8d, %r8d
1416991b1: e8 ba 62 55 ff              	callq	0x140bef470 <.text+0xbee470>
1416991b6: a8 01                       	testb	$0x1, %al
1416991b8: 0f 84 e7 05 00 00           	je	0x1416997a5 <.text+0x16987a5>
1416991be: 49 89 d1                    	movq	%rdx, %r9
1416991c1: 48 8b 85 d8 02 00 00        	movq	0x2d8(%rbp), %rax
1416991c8: 89 44 24 30                 	movl	%eax, 0x30(%rsp)
1416991cc: 89 74 24 28                 	movl	%esi, 0x28(%rsp)
1416991d0: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1416991d7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416991dc: 4c 89 f9                    	movq	%r15, %rcx
1416991df: 4c 89 ea                    	movq	%r13, %rdx
1416991e2: 49 89 f8                    	movq	%rdi, %r8
1416991e5: ff 93 b8 04 00 00           	callq	*0x4b8(%rbx)
1416991eb: e9 e9 00 00 00              	jmp	0x1416992d9 <.text+0x16982d9>
1416991f0: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
1416991f7: f0                          	lock
1416991f8: 48 ff 83 a8 0a 00 00        	incq	0xaa8(%rbx)
1416991ff: 48 8b 01                    	movq	(%rcx), %rax
141699202: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
141699206: 4c 8b 78 48                 	movq	0x48(%rax), %r15
14169920a: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699211: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699218: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169921f: 44 89 ea                    	movl	%r13d, %edx
141699222: e8 a9 e7 54 ff              	callq	0x140be79d0 <.text+0xbe69d0>
141699227: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169922e: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699235: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169923c: 48 89 d9                    	movq	%rbx, %rcx
14169923f: 48 89 c2                    	movq	%rax, %rdx
141699242: 45 31 c0                    	xorl	%r8d, %r8d
141699245: e8 26 62 55 ff              	callq	0x140bef470 <.text+0xbee470>
14169924a: a8 01                       	testb	$0x1, %al
14169924c: 0f 84 eb 05 00 00           	je	0x14169983d <.text+0x169883d>
141699252: 49 89 d5                    	movq	%rdx, %r13
141699255: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
14169925c: 4c 8b 70 08                 	movq	0x8(%rax), %r14
141699260: 48 8b 48 10                 	movq	0x10(%rax), %rcx
141699264: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169926b: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699272: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699279: 44 89 e2                    	movl	%r12d, %edx
14169927c: e8 4f e7 54 ff              	callq	0x140be79d0 <.text+0xbe69d0>
141699281: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699288: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169928f: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699296: 4c 89 f1                    	movq	%r14, %rcx
141699299: 48 89 c2                    	movq	%rax, %rdx
14169929c: 45 31 c0                    	xorl	%r8d, %r8d
14169929f: e8 cc 61 55 ff              	callq	0x140bef470 <.text+0xbee470>
1416992a4: a8 01                       	testb	$0x1, %al
1416992a6: 0f 84 b4 05 00 00           	je	0x141699860 <.text+0x1698860>
1416992ac: 49 89 d1                    	movq	%rdx, %r9
1416992af: 48 8b 85 d8 02 00 00        	movq	0x2d8(%rbp), %rax
1416992b6: 89 44 24 30                 	movl	%eax, 0x30(%rsp)
1416992ba: 89 74 24 28                 	movl	%esi, 0x28(%rsp)
1416992be: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1416992c5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416992ca: 4c 89 f9                    	movq	%r15, %rcx
1416992cd: 4c 89 ea                    	movq	%r13, %rdx
1416992d0: 49 89 f8                    	movq	%rdi, %r8
1416992d3: ff 93 c0 04 00 00           	callq	*0x4c0(%rbx)
1416992d9: 40 b6 01                    	movb	$0x1, %sil
1416992dc: b0 01                       	movb	$0x1, %al
1416992de: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
1416992e5: 49 8b 17                    	movq	(%r15), %rdx
1416992e8: 49 8b 4f 08                 	movq	0x8(%r15), %rcx
1416992ec: 40 88 b5 17 03 00 00        	movb	%sil, 0x317(%rbp)
1416992f3: 88 85 16 03 00 00           	movb	%al, 0x316(%rbp)
1416992f9: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699300: e8 3b 78 55 ff              	callq	0x140bf0b40 <.text+0xbefb40>
141699305: 41 b8 48 00 00 00           	movl	$0x48, %r8d
14169930b: 41 b9 10 00 00 00           	movl	$0x10, %r9d
141699311: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
141699318: 48 8d 95 20 02 00 00        	leaq	0x220(%rbp), %rdx
14169931f: e8 ec 59 9b fe              	callq	0x14004ed10 <.text+0x4dd10>
141699324: 48 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %rdx
14169932b: 48 85 d2                    	testq	%rdx, %rdx
14169932e: 48 8b 9d 08 03 00 00        	movq	0x308(%rbp), %rbx
141699335: 0f 84 d5 e9 ff ff           	je	0x141697d10 <.text+0x1696d10>
14169933b: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
141699342: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141699348: e8 d3 a8 f7 fe              	callq	0x140613c20 <.text+0x612c20>
14169934d: e9 be e9 ff ff              	jmp	0x141697d10 <.text+0x1696d10>
141699352: 31 c0                       	xorl	%eax, %eax
141699354: eb 8f                       	jmp	0x1416992e5 <.text+0x16982e5>
141699356: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169935d: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699364: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
14169936b: 48 8d 0d a6 e6 5e 01        	leaq	0x15ee6a6(%rip), %rcx   # 0x142c87a18
141699372: e8 99 ff 23 01              	callq	0x1428d9310 <.text+0x28d8310>
141699377: e9 28 05 00 00              	jmp	0x1416998a4 <.text+0x16988a4>
14169937c: 4c 8b bd 58 01 00 00        	movq	0x158(%rbp), %r15
141699383: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14169938a: 4c 29 f8                    	subq	%r15, %rax
14169938d: 48 c1 e8 03                 	shrq	$0x3, %rax
141699391: 48 b9 a7 37 bd e9 4d 6f 7a d3       	movabsq	$-0x2c8590b21642c859, %rcx # imm = 0xD37A6F4DE9BD37A7
14169939b: 48 0f af c8                 	imulq	%rax, %rcx
14169939f: 48 89 8d d0 02 00 00        	movq	%rcx, 0x2d0(%rbp)
1416993a6: 48 c7 85 e8 02 00 00 00 00 00 00    	movq	$0x0, 0x2e8(%rbp)
1416993b1: 4c 89 bd f8 02 00 00        	movq	%r15, 0x2f8(%rbp)
1416993b8: 4c 89 f9                    	movq	%r15, %rcx
1416993bb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1416993c0: 48 8b 85 e8 02 00 00        	movq	0x2e8(%rbp), %rax
1416993c7: 48 39 85 d0 02 00 00        	cmpq	%rax, 0x2d0(%rbp)
1416993ce: 74 1b                       	je	0x1416993eb <.text+0x16983eb>
1416993d0: 48 ff c0                    	incq	%rax
1416993d3: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
1416993da: 48 8d b1 b8 00 00 00        	leaq	0xb8(%rcx), %rsi
1416993e1: e8 fa 9b f6 fe              	callq	0x140602fe0 <.text+0x601fe0>
1416993e6: 48 89 f1                    	movq	%rsi, %rcx
1416993e9: eb d5                       	jmp	0x1416993c0 <.text+0x16983c0>
1416993eb: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1416993f2: 48 85 c0                    	testq	%rax, %rax
1416993f5: 74 19                       	je	0x141699410 <.text+0x1698410>
1416993f7: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
1416993fe: 48 69 d0 b8 00 00 00        	imulq	$0xb8, %rax, %rdx
141699405: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14169940b: e8 10 a8 f7 fe              	callq	0x140613c20 <.text+0x612c20>
141699410: c6 85 07 03 00 00 00        	movb	$0x0, 0x307(%rbp)
141699417: 48 89 d9                    	movq	%rbx, %rcx
14169941a: e8 31 e4 05 00              	callq	0x1416f7850 <.text+0x16f6850>
14169941f: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
141699426: 48 c7 00 0b 00 00 00        	movq	$0xb, (%rax)
14169942d: 48 8b b5 f0 00 00 00        	movq	0xf0(%rbp), %rsi
141699434: 48 8b bd f8 00 00 00        	movq	0xf8(%rbp), %rdi
14169943b: 48 85 ff                    	testq	%rdi, %rdi
14169943e: 74 45                       	je	0x141699485 <.text+0x1698485>
141699440: 48 8d 5e 10                 	leaq	0x10(%rsi), %rbx
141699444: eb 13                       	jmp	0x141699459 <.text+0x1698459>
141699446: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141699450: 48 83 c3 20                 	addq	$0x20, %rbx
141699454: 48 ff cf                    	decq	%rdi
141699457: 74 2c                       	je	0x141699485 <.text+0x1698485>
141699459: 8b 43 f0                    	movl	-0x10(%rbx), %eax
14169945c: 83 e0 fe                    	andl	$-0x2, %eax
14169945f: 83 f8 02                    	cmpl	$0x2, %eax
141699462: 75 ec                       	jne	0x141699450 <.text+0x1698450>
141699464: 48 8b 43 f8                 	movq	-0x8(%rbx), %rax
141699468: 48 85 c0                    	testq	%rax, %rax
14169946b: 74 e3                       	je	0x141699450 <.text+0x1698450>
14169946d: 48 8b 0b                    	movq	(%rbx), %rcx
141699470: 48 c1 e0 03                 	shlq	$0x3, %rax
141699474: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
141699478: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14169947e: e8 9d a7 f7 fe              	callq	0x140613c20 <.text+0x612c20>
141699483: eb cb                       	jmp	0x141699450 <.text+0x1698450>
141699485: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
14169948c: 48 85 d2                    	testq	%rdx, %rdx
14169948f: 74 12                       	je	0x1416994a3 <.text+0x16984a3>
141699491: 48 c1 e2 05                 	shlq	$0x5, %rdx
141699495: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14169949b: 48 89 f1                    	movq	%rsi, %rcx
14169949e: e8 7d a7 f7 fe              	callq	0x140613c20 <.text+0x612c20>
1416994a3: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
1416994aa: e8 01 90 54 ff              	callq	0x140be24b0 <.text+0xbe14b0>
1416994af: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1416994b6: 0f 10 40 50                 	movups	0x50(%rax), %xmm0
1416994ba: 0f 29 85 e0 01 00 00        	movaps	%xmm0, 0x1e0(%rbp)
1416994c1: 0f 10 40 40                 	movups	0x40(%rax), %xmm0
1416994c5: 0f 29 85 d0 01 00 00        	movaps	%xmm0, 0x1d0(%rbp)
1416994cc: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1416994d0: f3 0f 6f 48 10              	movdqu	0x10(%rax), %xmm1
1416994d5: f3 0f 6f 50 20              	movdqu	0x20(%rax), %xmm2
1416994da: 0f 10 58 30                 	movups	0x30(%rax), %xmm3
1416994de: 0f 29 9d c0 01 00 00        	movaps	%xmm3, 0x1c0(%rbp)
1416994e5: 66 0f 7f 95 b0 01 00 00     	movdqa	%xmm2, 0x1b0(%rbp)
1416994ed: 66 0f 7f 8d a0 01 00 00     	movdqa	%xmm1, 0x1a0(%rbp)
1416994f5: 66 0f 7f 85 90 01 00 00     	movdqa	%xmm0, 0x190(%rbp)
1416994fd: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
141699504: e8 a7 8f 54 ff              	callq	0x140be24b0 <.text+0xbe14b0>
141699509: 8b 85 90 01 00 00           	movl	0x190(%rbp), %eax
14169950f: 83 e0 fe                    	andl	$-0x2, %eax
141699512: 83 f8 02                    	cmpl	$0x2, %eax
141699515: 75 26                       	jne	0x14169953d <.text+0x169853d>
141699517: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
14169951e: 48 85 c0                    	testq	%rax, %rax
141699521: 74 1a                       	je	0x14169953d <.text+0x169853d>
141699523: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
14169952a: 48 c1 e0 03                 	shlq	$0x3, %rax
14169952e: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
141699532: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141699538: e8 e3 a6 f7 fe              	callq	0x140613c20 <.text+0x612c20>
14169953d: 8b 85 b0 01 00 00           	movl	0x1b0(%rbp), %eax
141699543: 83 e0 fe                    	andl	$-0x2, %eax
141699546: 83 f8 02                    	cmpl	$0x2, %eax
141699549: 75 26                       	jne	0x141699571 <.text+0x1698571>
14169954b: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
141699552: 48 85 c0                    	testq	%rax, %rax
141699555: 74 1a                       	je	0x141699571 <.text+0x1698571>
141699557: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
14169955e: 48 c1 e0 03                 	shlq	$0x3, %rax
141699562: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
141699566: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14169956c: e8 af a6 f7 fe              	callq	0x140613c20 <.text+0x612c20>
141699571: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
141699578: e8 33 8f 54 ff              	callq	0x140be24b0 <.text+0xbe14b0>
14169957d: 48 8b b5 f0 02 00 00        	movq	0x2f0(%rbp), %rsi
141699584: 83 be f8 02 00 00 00        	cmpl	$0x0, 0x2f8(%rsi)
14169958b: 74 0a                       	je	0x141699597 <.text+0x1698597>
14169958d: c7 86 f8 02 00 00 00 00 00 00       	movl	$0x0, 0x2f8(%rsi)
141699597: 48 8b 46 18                 	movq	0x18(%rsi), %rax
14169959b: 48 85 c0                    	testq	%rax, %rax
14169959e: 74 1a                       	je	0x1416995ba <.text+0x16985ba>
1416995a0: 48 8b 4e 20                 	movq	0x20(%rsi), %rcx
1416995a4: 48 89 c2                    	movq	%rax, %rdx
1416995a7: 48 c1 e2 06                 	shlq	$0x6, %rdx
1416995ab: 48 8d 14 82                 	leaq	(%rdx,%rax,4), %rdx
1416995af: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1416995b5: e8 66 a6 f7 fe              	callq	0x140613c20 <.text+0x612c20>
1416995ba: ba 00 03 00 00              	movl	$0x300, %edx            # imm = 0x300
1416995bf: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1416995c5: 48 89 f1                    	movq	%rsi, %rcx
1416995c8: e8 53 a6 f7 fe              	callq	0x140613c20 <.text+0x612c20>
1416995cd: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
1416995d4: 0f 28 b5 20 03 00 00        	movaps	0x320(%rbp), %xmm6
1416995db: 0f 28 bd 30 03 00 00        	movaps	0x330(%rbp), %xmm7
1416995e2: 44 0f 28 85 40 03 00 00     	movaps	0x340(%rbp), %xmm8
1416995ea: 44 0f 28 8d 50 03 00 00     	movaps	0x350(%rbp), %xmm9
1416995f2: 44 0f 28 95 60 03 00 00     	movaps	0x360(%rbp), %xmm10
1416995fa: 44 0f 28 9d 70 03 00 00     	movaps	0x370(%rbp), %xmm11
141699602: 48 81 c4 08 04 00 00        	addq	$0x408, %rsp            # imm = 0x408
141699609: 5b                          	popq	%rbx
14169960a: 5f                          	popq	%rdi
14169960b: 5e                          	popq	%rsi
14169960c: 41 5c                       	popq	%r12
14169960e: 41 5d                       	popq	%r13
141699610: 41 5e                       	popq	%r14
141699612: 41 5f                       	popq	%r15
141699614: 5d                          	popq	%rbp
141699615: c3                          	retq
141699616: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169961d: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699624: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169962b: 48 8d 0d de e0 5e 01        	leaq	0x15ee0de(%rip), %rcx   # 0x142c87710
141699632: 4c 8d 05 1f e1 5e 01        	leaq	0x15ee11f(%rip), %r8    # 0x142c87758
141699639: ba 41 00 00 00              	movl	$0x41, %edx
14169963e: e8 ed fd 23 01              	callq	0x1428d9430 <.text+0x28d8430>
141699643: e9 5c 02 00 00              	jmp	0x1416998a4 <.text+0x16988a4>
141699648: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169964f: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699656: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169965d: 48 8d 0d ac e0 5e 01        	leaq	0x15ee0ac(%rip), %rcx   # 0x142c87710
141699664: 4c 8d 05 4d e1 5e 01        	leaq	0x15ee14d(%rip), %r8    # 0x142c877b8
14169966b: ba 41 00 00 00              	movl	$0x41, %edx
141699670: e8 bb fd 23 01              	callq	0x1428d9430 <.text+0x28d8430>
141699675: e9 2a 02 00 00              	jmp	0x1416998a4 <.text+0x16988a4>
14169967a: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699681: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699688: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
14169968f: 48 8d 0d 0a e3 5e 01        	leaq	0x15ee30a(%rip), %rcx   # 0x142c879a0
141699696: e8 75 fc 23 01              	callq	0x1428d9310 <.text+0x28d8310>
14169969b: e9 04 02 00 00              	jmp	0x1416998a4 <.text+0x16988a4>
1416996a0: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416996a7: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416996ae: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
1416996b5: 48 8d 0d e4 13 5a 01        	leaq	0x15a13e4(%rip), %rcx   # 0x142c3aaa0
1416996bc: e8 0f ea 23 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1416996c1: e9 de 01 00 00              	jmp	0x1416998a4 <.text+0x16988a4>
1416996c6: c6 85 50 02 00 00 03        	movb	$0x3, 0x250(%rbp)
1416996cd: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416996d4: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416996db: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
1416996e2: 48 8d 05 6f e1 5e 01        	leaq	0x15ee16f(%rip), %rax   # 0x142c87858
1416996e9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416996ee: 48 8d 0d 7b e1 5e 01        	leaq	0x15ee17b(%rip), %rcx   # 0x142c87870
1416996f5: 4c 8d 0d d4 d7 5e 01        	leaq	0x15ed7d4(%rip), %r9    # 0x142c86ed0
1416996fc: 4c 8d 85 50 02 00 00        	leaq	0x250(%rbp), %r8
141699703: ba 48 00 00 00              	movl	$0x48, %edx
141699708: e8 53 00 24 01              	callq	0x1428d9760 <.text+0x28d8760>
14169970d: e9 92 01 00 00              	jmp	0x1416998a4 <.text+0x16988a4>
141699712: 0f 10 45 00                 	movups	(%rbp), %xmm0
141699716: 0f 29 85 d0 01 00 00        	movaps	%xmm0, 0x1d0(%rbp)
14169971d: f3 0f 6f 45 c0              	movdqu	-0x40(%rbp), %xmm0
141699722: f3 0f 6f 4d d0              	movdqu	-0x30(%rbp), %xmm1
141699727: f3 0f 6f 55 e0              	movdqu	-0x20(%rbp), %xmm2
14169972c: 0f 10 5d f0                 	movups	-0x10(%rbp), %xmm3
141699730: 0f 29 9d c0 01 00 00        	movaps	%xmm3, 0x1c0(%rbp)
141699737: 66 0f 7f 95 b0 01 00 00     	movdqa	%xmm2, 0x1b0(%rbp)
14169973f: 66 0f 7f 8d a0 01 00 00     	movdqa	%xmm1, 0x1a0(%rbp)
141699747: 66 0f 7f 85 90 01 00 00     	movdqa	%xmm0, 0x190(%rbp)
14169974f: 48 8d 05 32 e2 5e 01        	leaq	0x15ee232(%rip), %rax   # 0x142c87988
141699756: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14169975b: 48 8d 0d d6 e1 5e 01        	leaq	0x15ee1d6(%rip), %rcx   # 0x142c87938
141699762: 4c 8d 0d 87 d7 5e 01        	leaq	0x15ed787(%rip), %r9    # 0x142c86ef0
141699769: 4c 8d 85 90 01 00 00        	leaq	0x190(%rbp), %r8
141699770: ba 11 00 00 00              	movl	$0x11, %edx
141699775: e8 e6 ff 23 01              	callq	0x1428d9760 <.text+0x28d8760>
14169977a: e9 25 01 00 00              	jmp	0x1416998a4 <.text+0x16988a4>
14169977f: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699786: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169978d: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699794: 48 8d 0d d5 df 5e 01        	leaq	0x15edfd5(%rip), %rcx   # 0x142c87770
14169979b: e8 70 fb 23 01              	callq	0x1428d9310 <.text+0x28d8310>
1416997a0: e9 ff 00 00 00              	jmp	0x1416998a4 <.text+0x16988a4>
1416997a5: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416997ac: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416997b3: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
1416997ba: 48 8d 0d c7 df 5e 01        	leaq	0x15edfc7(%rip), %rcx   # 0x142c87788
1416997c1: e8 4a fb 23 01              	callq	0x1428d9310 <.text+0x28d8310>
1416997c6: e9 d9 00 00 00              	jmp	0x1416998a4 <.text+0x16988a4>
1416997cb: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416997d2: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416997d9: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
1416997e0: 48 8d 0d b1 de 5e 01        	leaq	0x15edeb1(%rip), %rcx   # 0x142c87698
1416997e7: e8 24 fb 23 01              	callq	0x1428d9310 <.text+0x28d8310>
1416997ec: e9 b3 00 00 00              	jmp	0x1416998a4 <.text+0x16988a4>
1416997f1: 48 8d 0d 70 e2 5e 01        	leaq	0x15ee270(%rip), %rcx   # 0x142c87a68
1416997f8: 4c 8d 05 b1 e2 5e 01        	leaq	0x15ee2b1(%rip), %r8    # 0x142c87ab0
1416997ff: ba 2c 00 00 00              	movl	$0x2c, %edx
141699804: e8 27 fb 23 01              	callq	0x1428d9330 <.text+0x28d8330>
141699809: e9 96 00 00 00              	jmp	0x1416998a4 <.text+0x16988a4>
14169980e: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699815: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169981c: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699823: 48 8d 0d 3e e2 5e 01        	leaq	0x15ee23e(%rip), %rcx   # 0x142c87a68
14169982a: 4c 8d 05 67 e2 5e 01        	leaq	0x15ee267(%rip), %r8    # 0x142c87a98
141699831: ba 2c 00 00 00              	movl	$0x2c, %edx
141699836: e8 f5 fa 23 01              	callq	0x1428d9330 <.text+0x28d8330>
14169983b: eb 67                       	jmp	0x1416998a4 <.text+0x16988a4>
14169983d: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699844: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169984b: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699852: 48 8d 0d 77 df 5e 01        	leaq	0x15edf77(%rip), %rcx   # 0x142c877d0
141699859: e8 b2 fa 23 01              	callq	0x1428d9310 <.text+0x28d8310>
14169985e: eb 44                       	jmp	0x1416998a4 <.text+0x16988a4>
141699860: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699867: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169986e: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699875: 48 8d 0d 6c df 5e 01        	leaq	0x15edf6c(%rip), %rcx   # 0x142c877e8
14169987c: e8 8f fa 23 01              	callq	0x1428d9310 <.text+0x28d8310>
141699881: eb 21                       	jmp	0x1416998a4 <.text+0x16988a4>
141699883: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169988a: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699891: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699898: 48 8d 0d 01 df 5e 01        	leaq	0x15edf01(%rip), %rcx   # 0x142c877a0
14169989f: e8 6c fa 23 01              	callq	0x1428d9310 <.text+0x28d8310>
1416998a4: 0f 0b                       	ud2
1416998a6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1416998b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416998b5: 55                          	pushq	%rbp
1416998b6: 41 57                       	pushq	%r15
1416998b8: 41 56                       	pushq	%r14
1416998ba: 41 55                       	pushq	%r13
1416998bc: 41 54                       	pushq	%r12
1416998be: 56                          	pushq	%rsi
1416998bf: 57                          	pushq	%rdi
1416998c0: 53                          	pushq	%rbx
1416998c1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1416998c8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416998cf: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
1416998d6: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
1416998dd: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
1416998e3: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
1416998ea: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
1416998f0: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
1416998f9: 0f b6 9d 14 03 00 00        	movzbl	0x314(%rbp), %ebx
141699900: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
141699907: e8 a4 8b 54 ff              	callq	0x140be24b0 <.text+0xbe14b0>
14169990c: 48 8b b5 f0 02 00 00        	movq	0x2f0(%rbp), %rsi
141699913: 48 8d 8e 80 00 00 00        	leaq	0x80(%rsi), %rcx
14169991a: e8 31 82 ac fe              	callq	0x140161b50 <.text+0x160b50>
14169991f: 48 8d 8e f8 00 00 00        	leaq	0xf8(%rsi), %rcx
141699926: e8 85 8b 54 ff              	callq	0x140be24b0 <.text+0xbe14b0>
14169992b: 83 be f8 02 00 00 00        	cmpl	$0x0, 0x2f8(%rsi)
141699932: 74 0a                       	je	0x14169993e <.text+0x169893e>
141699934: c7 86 f8 02 00 00 00 00 00 00       	movl	$0x0, 0x2f8(%rsi)
14169993e: f6 c3 01                    	testb	$0x1, %bl
141699941: 74 08                       	je	0x14169994b <.text+0x169894b>
141699943: 48 89 f1                    	movq	%rsi, %rcx
141699946: e8 c5 cd 1d ff              	callq	0x140876710 <.text+0x875710>
14169994b: 48 8b 46 18                 	movq	0x18(%rsi), %rax
14169994f: 48 85 c0                    	testq	%rax, %rax
141699952: 74 1a                       	je	0x14169996e <.text+0x169896e>
141699954: 48 8b 4e 20                 	movq	0x20(%rsi), %rcx
141699958: 48 89 c2                    	movq	%rax, %rdx
14169995b: 48 c1 e2 06                 	shlq	$0x6, %rdx
14169995f: 48 8d 14 82                 	leaq	(%rdx,%rax,4), %rdx
141699963: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141699969: e8 b2 a2 f7 fe              	callq	0x140613c20 <.text+0x612c20>
14169996e: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
141699976: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
14169997b: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
141699981: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141699987: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
14169998d: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
141699993: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14169999a: 5b                          	popq	%rbx
14169999b: 5f                          	popq	%rdi
14169999c: 5e                          	popq	%rsi
14169999d: 41 5c                       	popq	%r12
14169999f: 41 5d                       	popq	%r13
1416999a1: 41 5e                       	popq	%r14
1416999a3: 41 5f                       	popq	%r15
1416999a5: 5d                          	popq	%rbp
1416999a6: c3                          	retq
1416999a7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1416999b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416999b5: 55                          	pushq	%rbp
1416999b6: 41 57                       	pushq	%r15
1416999b8: 41 56                       	pushq	%r14
1416999ba: 41 55                       	pushq	%r13
1416999bc: 41 54                       	pushq	%r12
1416999be: 56                          	pushq	%rsi
1416999bf: 57                          	pushq	%rdi
1416999c0: 53                          	pushq	%rbx
1416999c1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1416999c8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416999cf: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
1416999d6: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
1416999dd: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
1416999e3: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
1416999ea: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
1416999f0: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
1416999f9: c6 85 14 03 00 00 01        	movb	$0x1, 0x314(%rbp)
141699a00: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
141699a07: e8 64 50 13 ff              	callq	0x1407cea70 <.text+0x7cda70>
141699a0c: c6 85 14 03 00 00 01        	movb	$0x1, 0x314(%rbp)
141699a13: 66 0f 6f b4 24 80 00 00 00  	movdqa	0x80(%rsp), %xmm6
141699a1c: 66 0f 6f 7c 24 70           	movdqa	0x70(%rsp), %xmm7
141699a22: 66 44 0f 6f 44 24 60        	movdqa	0x60(%rsp), %xmm8
141699a29: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141699a2f: 66 44 0f 6f 54 24 40        	movdqa	0x40(%rsp), %xmm10
141699a36: 66 44 0f 6f 5c 24 30        	movdqa	0x30(%rsp), %xmm11
141699a3d: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141699a44: 5b                          	popq	%rbx
141699a45: 5f                          	popq	%rdi
141699a46: 5e                          	popq	%rsi
141699a47: 41 5c                       	popq	%r12
141699a49: 41 5d                       	popq	%r13
141699a4b: 41 5e                       	popq	%r14
141699a4d: 41 5f                       	popq	%r15
141699a4f: 5d                          	popq	%rbp
141699a50: c3                          	retq
141699a51: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141699a60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141699a65: 55                          	pushq	%rbp
141699a66: 41 57                       	pushq	%r15
141699a68: 41 56                       	pushq	%r14
141699a6a: 41 55                       	pushq	%r13
141699a6c: 41 54                       	pushq	%r12
141699a6e: 56                          	pushq	%rsi
141699a6f: 57                          	pushq	%rdi
141699a70: 53                          	pushq	%rbx
141699a71: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141699a78: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141699a7f: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
141699a86: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
141699a8d: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141699a93: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
141699a9a: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
141699aa0: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
141699aa9: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
141699ab0: e8 1b 16 1e ff              	callq	0x14087b0d0 <.text+0x87a0d0>
141699ab5: c6 85 07 03 00 00 00        	movb	$0x0, 0x307(%rbp)
141699abc: 66 0f 6f b4 24 80 00 00 00  	movdqa	0x80(%rsp), %xmm6
141699ac5: 66 0f 6f 7c 24 70           	movdqa	0x70(%rsp), %xmm7
141699acb: 66 44 0f 6f 44 24 60        	movdqa	0x60(%rsp), %xmm8
141699ad2: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141699ad8: 66 44 0f 6f 54 24 40        	movdqa	0x40(%rsp), %xmm10
141699adf: 66 44 0f 6f 5c 24 30        	movdqa	0x30(%rsp), %xmm11
141699ae6: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141699aed: 5b                          	popq	%rbx
141699aee: 5f                          	popq	%rdi
141699aef: 5e                          	popq	%rsi
141699af0: 41 5c                       	popq	%r12
141699af2: 41 5d                       	popq	%r13
141699af4: 41 5e                       	popq	%r14
141699af6: 41 5f                       	popq	%r15
141699af8: 5d                          	popq	%rbp
141699af9: c3                          	retq
141699afa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141699b00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141699b05: 55                          	pushq	%rbp
141699b06: 41 57                       	pushq	%r15
141699b08: 41 56                       	pushq	%r14
141699b0a: 41 55                       	pushq	%r13
141699b0c: 41 54                       	pushq	%r12
141699b0e: 56                          	pushq	%rsi
141699b0f: 57                          	pushq	%rdi
141699b10: 53                          	pushq	%rbx
141699b11: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141699b18: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141699b1f: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
141699b25: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
141699b2b: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141699b31: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
141699b37: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
141699b3c: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
141699b44: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
141699b4b: 48 85 c0                    	testq	%rax, %rax
141699b4e: 74 19                       	je	0x141699b69 <.text+0x1698b69>
141699b50: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
141699b57: 48 69 d0 b8 00 00 00        	imulq	$0xb8, %rax, %rdx
141699b5e: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141699b64: e8 b7 a0 f7 fe              	callq	0x140613c20 <.text+0x612c20>
141699b69: c6 85 07 03 00 00 00        	movb	$0x0, 0x307(%rbp)
141699b70: 66 0f 6f b4 24 80 00 00 00  	movdqa	0x80(%rsp), %xmm6
141699b79: 66 0f 6f 7c 24 70           	movdqa	0x70(%rsp), %xmm7
141699b7f: 66 44 0f 6f 44 24 60        	movdqa	0x60(%rsp), %xmm8
141699b86: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141699b8c: 66 44 0f 6f 54 24 40        	movdqa	0x40(%rsp), %xmm10
141699b93: 66 44 0f 6f 5c 24 30        	movdqa	0x30(%rsp), %xmm11
141699b9a: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141699ba1: 5b                          	popq	%rbx
141699ba2: 5f                          	popq	%rdi
141699ba3: 5e                          	popq	%rsi
141699ba4: 41 5c                       	popq	%r12
141699ba6: 41 5d                       	popq	%r13
141699ba8: 41 5e                       	popq	%r14
141699baa: 41 5f                       	popq	%r15
141699bac: 5d                          	popq	%rbp
141699bad: c3                          	retq
141699bae: 66 90                       	nop
141699bb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141699bb5: 55                          	pushq	%rbp
141699bb6: 41 57                       	pushq	%r15
141699bb8: 41 56                       	pushq	%r14
141699bba: 41 55                       	pushq	%r13
141699bbc: 41 54                       	pushq	%r12
141699bbe: 56                          	pushq	%rsi
141699bbf: 57                          	pushq	%rdi
141699bc0: 53                          	pushq	%rbx
141699bc1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141699bc8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141699bcf: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
141699bd6: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
141699bdd: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141699be3: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
141699bea: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
141699bf0: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
141699bf9: 48 8b bd d0 02 00 00        	movq	0x2d0(%rbp), %rdi
141699c00: 48 8b 9d e8 02 00 00        	movq	0x2e8(%rbp), %rbx
141699c07: 48 39 df                    	cmpq	%rbx, %rdi
141699c0a: 48 8b b5 f8 02 00 00        	movq	0x2f8(%rbp), %rsi
141699c11: 74 24                       	je	0x141699c37 <.text+0x1698c37>
141699c13: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141699c20: 48 69 cb b8 00 00 00        	imulq	$0xb8, %rbx, %rcx
141699c27: 48 01 f1                    	addq	%rsi, %rcx
141699c2a: e8 b1 93 f6 fe              	callq	0x140602fe0 <.text+0x601fe0>
141699c2f: 48 ff c3                    	incq	%rbx
141699c32: 48 39 fb                    	cmpq	%rdi, %rbx
141699c35: 75 e9                       	jne	0x141699c20 <.text+0x1698c20>
141699c37: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
141699c3f: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
141699c44: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
141699c4a: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141699c50: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
141699c56: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
141699c5c: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141699c63: 5b                          	popq	%rbx
141699c64: 5f                          	popq	%rdi
141699c65: 5e                          	popq	%rsi
141699c66: 41 5c                       	popq	%r12
141699c68: 41 5d                       	popq	%r13
141699c6a: 41 5e                       	popq	%r14
141699c6c: 41 5f                       	popq	%r15
141699c6e: 5d                          	popq	%rbp
141699c6f: c3                          	retq
141699c70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141699c75: 55                          	pushq	%rbp
141699c76: 41 57                       	pushq	%r15
141699c78: 41 56                       	pushq	%r14
141699c7a: 41 55                       	pushq	%r13
141699c7c: 41 54                       	pushq	%r12
141699c7e: 56                          	pushq	%rsi
141699c7f: 57                          	pushq	%rdi
141699c80: 53                          	pushq	%rbx
141699c81: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141699c88: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141699c8f: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
141699c96: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
141699c9d: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141699ca3: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
141699caa: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
141699cb0: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
141699cb9: 48 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %rax
141699cc0: 83 b8 f8 02 00 00 00        	cmpl	$0x0, 0x2f8(%rax)
141699cc7: 74 11                       	je	0x141699cda <.text+0x1698cda>
141699cc9: 48 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %rax
141699cd0: c7 80 f8 02 00 00 00 00 00 00       	movl	$0x0, 0x2f8(%rax)
141699cda: 48 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %rax
141699ce1: 48 8b 40 18                 	movq	0x18(%rax), %rax
141699ce5: 48 85 c0                    	testq	%rax, %rax
141699ce8: 74 21                       	je	0x141699d0b <.text+0x1698d0b>
141699cea: 48 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %rcx
141699cf1: 48 8b 49 20                 	movq	0x20(%rcx), %rcx
141699cf5: 48 89 c2                    	movq	%rax, %rdx
141699cf8: 48 c1 e2 06                 	shlq	$0x6, %rdx
141699cfc: 48 8d 14 82                 	leaq	(%rdx,%rax,4), %rdx
141699d00: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141699d06: e8 15 9f f7 fe              	callq	0x140613c20 <.text+0x612c20>
141699d0b: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
141699d13: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
141699d18: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
141699d1e: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141699d24: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
141699d2a: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
141699d30: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141699d37: 5b                          	popq	%rbx
141699d38: 5f                          	popq	%rdi
141699d39: 5e                          	popq	%rsi
141699d3a: 41 5c                       	popq	%r12
141699d3c: 41 5d                       	popq	%r13
141699d3e: 41 5e                       	popq	%r14
141699d40: 41 5f                       	popq	%r15
141699d42: 5d                          	popq	%rbp
141699d43: c3                          	retq
141699d44: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141699d50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141699d55: 55                          	pushq	%rbp
141699d56: 41 57                       	pushq	%r15
141699d58: 41 56                       	pushq	%r14
141699d5a: 41 55                       	pushq	%r13
141699d5c: 41 54                       	pushq	%r12
141699d5e: 56                          	pushq	%rsi
141699d5f: 57                          	pushq	%rdi
141699d60: 53                          	pushq	%rbx
141699d61: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141699d68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141699d6f: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
141699d76: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
141699d7d: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141699d83: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
141699d8a: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
141699d90: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
141699d99: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699da0: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699da7: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141699dae: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
141699db5: 41 b8 20 00 00 00           	movl	$0x20, %r8d
141699dbb: 41 b9 10 00 00 00           	movl	$0x10, %r9d
141699dc1: 48 8d 95 30 01 00 00        	leaq	0x130(%rbp), %rdx
141699dc8: e8 13 69 9b fe              	callq	0x1400506e0 <.text+0x4f6e0>
141699dcd: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699dd4: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699ddb: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141699de2: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
141699dea: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
141699def: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
141699df5: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141699dfb: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
141699e01: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
141699e07: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141699e0e: 5b                          	popq	%rbx
141699e0f: 5f                          	popq	%rdi
141699e10: 5e                          	popq	%rsi
141699e11: 41 5c                       	popq	%r12
141699e13: 41 5d                       	popq	%r13
141699e15: 41 5e                       	popq	%r14
141699e17: 41 5f                       	popq	%r15
141699e19: 5d                          	popq	%rbp
141699e1a: c3                          	retq
141699e1b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141699e20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141699e25: 55                          	pushq	%rbp
141699e26: 41 57                       	pushq	%r15
141699e28: 41 56                       	pushq	%r14
141699e2a: 41 55                       	pushq	%r13
141699e2c: 41 54                       	pushq	%r12
141699e2e: 56                          	pushq	%rsi
141699e2f: 57                          	pushq	%rdi
141699e30: 53                          	pushq	%rbx
141699e31: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141699e38: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141699e3f: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
141699e46: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
141699e4d: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141699e53: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
141699e5a: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
141699e60: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
141699e69: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
141699e6d: 41 b8 20 00 00 00           	movl	$0x20, %r8d
141699e73: 41 b9 10 00 00 00           	movl	$0x10, %r9d
141699e79: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
141699e80: e8 5b 68 9b fe              	callq	0x1400506e0 <.text+0x4f6e0>
141699e85: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699e8c: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699e93: c6 85 15 03 00 00 01        	movb	$0x1, 0x315(%rbp)
141699e9a: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
141699ea2: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
141699ea7: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
141699ead: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141699eb3: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
141699eb9: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
141699ebf: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141699ec6: 5b                          	popq	%rbx
141699ec7: 5f                          	popq	%rdi
141699ec8: 5e                          	popq	%rsi
141699ec9: 41 5c                       	popq	%r12
141699ecb: 41 5d                       	popq	%r13
141699ecd: 41 5e                       	popq	%r14
141699ecf: 41 5f                       	popq	%r15
141699ed1: 5d                          	popq	%rbp
141699ed2: c3                          	retq
141699ed3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141699ee0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141699ee5: 55                          	pushq	%rbp
141699ee6: 41 57                       	pushq	%r15
141699ee8: 41 56                       	pushq	%r14
141699eea: 41 55                       	pushq	%r13
141699eec: 41 54                       	pushq	%r12
141699eee: 56                          	pushq	%rsi
141699eef: 57                          	pushq	%rdi
141699ef0: 53                          	pushq	%rbx
141699ef1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141699ef8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141699eff: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
141699f06: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
141699f0d: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141699f13: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
141699f1a: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
141699f20: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
141699f29: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
141699f30: e8 7b 85 54 ff              	callq	0x140be24b0 <.text+0xbe14b0>
141699f35: c6 85 17 03 00 00 00        	movb	$0x0, 0x317(%rbp)
141699f3c: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699f43: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699f4a: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
141699f52: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
141699f57: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
141699f5d: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141699f63: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
141699f69: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
141699f6f: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141699f76: 5b                          	popq	%rbx
141699f77: 5f                          	popq	%rdi
141699f78: 5e                          	popq	%rsi
141699f79: 41 5c                       	popq	%r12
141699f7b: 41 5d                       	popq	%r13
141699f7d: 41 5e                       	popq	%r14
141699f7f: 41 5f                       	popq	%r15
141699f81: 5d                          	popq	%rbp
141699f82: c3                          	retq
141699f83: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141699f90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141699f95: 55                          	pushq	%rbp
141699f96: 41 57                       	pushq	%r15
141699f98: 41 56                       	pushq	%r14
141699f9a: 41 55                       	pushq	%r13
141699f9c: 41 54                       	pushq	%r12
141699f9e: 56                          	pushq	%rsi
141699f9f: 57                          	pushq	%rdi
141699fa0: 53                          	pushq	%rbx
141699fa1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141699fa8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141699faf: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
141699fb6: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
141699fbd: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141699fc3: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
141699fca: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
141699fd0: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
141699fd9: 48 83 bd e0 02 00 00 00     	cmpq	$0x0, 0x2e0(%rbp)
141699fe1: 74 21                       	je	0x14169a004 <.text+0x1699004>
141699fe3: 48 8b 85 e0 02 00 00        	movq	0x2e0(%rbp), %rax
141699fea: 48 8d 14 c5 00 00 00 00     	leaq	(,%rax,8), %rdx
141699ff2: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141699ff8: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
141699fff: e8 1c 9c f7 fe              	callq	0x140613c20 <.text+0x612c20>
14169a004: 66 0f 6f b4 24 80 00 00 00  	movdqa	0x80(%rsp), %xmm6
14169a00d: 66 0f 6f 7c 24 70           	movdqa	0x70(%rsp), %xmm7
14169a013: 66 44 0f 6f 44 24 60        	movdqa	0x60(%rsp), %xmm8
14169a01a: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14169a020: 66 44 0f 6f 54 24 40        	movdqa	0x40(%rsp), %xmm10
14169a027: 66 44 0f 6f 5c 24 30        	movdqa	0x30(%rsp), %xmm11
14169a02e: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14169a035: 5b                          	popq	%rbx
14169a036: 5f                          	popq	%rdi
14169a037: 5e                          	popq	%rsi
14169a038: 41 5c                       	popq	%r12
14169a03a: 41 5d                       	popq	%r13
14169a03c: 41 5e                       	popq	%r14
14169a03e: 41 5f                       	popq	%r15
14169a040: 5d                          	popq	%rbp
14169a041: c3                          	retq
14169a042: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14169a050: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14169a055: 55                          	pushq	%rbp
14169a056: 41 57                       	pushq	%r15
14169a058: 41 56                       	pushq	%r14
14169a05a: 41 55                       	pushq	%r13
14169a05c: 41 54                       	pushq	%r12
14169a05e: 56                          	pushq	%rsi
14169a05f: 57                          	pushq	%rdi
14169a060: 53                          	pushq	%rbx
14169a061: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
14169a068: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14169a06f: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
14169a076: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
14169a07d: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
14169a083: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
14169a08a: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
14169a090: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
14169a099: 80 bd 78 01 00 00 00        	cmpb	$0x0, 0x178(%rbp)
14169a0a0: 74 4e                       	je	0x14169a0f0 <.text+0x16990f0>
14169a0a2: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169a0a9: c6 85 16 03 00 00 00        	movb	$0x0, 0x316(%rbp)
14169a0b0: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169a0b7: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14169a0bf: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
14169a0c4: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14169a0ca: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14169a0d0: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
14169a0d6: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
14169a0dc: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14169a0e3: 5b                          	popq	%rbx
14169a0e4: 5f                          	popq	%rdi
14169a0e5: 5e                          	popq	%rsi
14169a0e6: 41 5c                       	popq	%r12
14169a0e8: 41 5d                       	popq	%r13
14169a0ea: 41 5e                       	popq	%r14
14169a0ec: 41 5f                       	popq	%r15
14169a0ee: 5d                          	popq	%rbp
14169a0ef: c3                          	retq
14169a0f0: 48 83 bd e0 02 00 00 00     	cmpq	$0x0, 0x2e0(%rbp)
14169a0f8: 74 1d                       	je	0x14169a117 <.text+0x1699117>
14169a0fa: 48 8b 95 e0 02 00 00        	movq	0x2e0(%rbp), %rdx
14169a101: 48 c1 e2 03                 	shlq	$0x3, %rdx
14169a105: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14169a10b: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
14169a112: e8 09 9b f7 fe              	callq	0x140613c20 <.text+0x612c20>
14169a117: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169a11e: c6 85 16 03 00 00 00        	movb	$0x0, 0x316(%rbp)
14169a125: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169a12c: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14169a134: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
14169a139: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14169a13f: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14169a145: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
14169a14b: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
14169a151: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14169a158: 5b                          	popq	%rbx
14169a159: 5f                          	popq	%rdi
14169a15a: 5e                          	popq	%rsi
14169a15b: 41 5c                       	popq	%r12
14169a15d: 41 5d                       	popq	%r13
14169a15f: 41 5e                       	popq	%r14
14169a161: 41 5f                       	popq	%r15
14169a163: 5d                          	popq	%rbp
14169a164: c3                          	retq
14169a165: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14169a170: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14169a175: 55                          	pushq	%rbp
14169a176: 41 57                       	pushq	%r15
14169a178: 41 56                       	pushq	%r14
14169a17a: 41 55                       	pushq	%r13
14169a17c: 41 54                       	pushq	%r12
14169a17e: 56                          	pushq	%rsi
14169a17f: 57                          	pushq	%rdi
14169a180: 53                          	pushq	%rbx
14169a181: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
14169a188: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14169a18f: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
14169a196: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
14169a19d: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
14169a1a3: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
14169a1aa: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
14169a1b0: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
14169a1b9: 48 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %rdx
14169a1c0: 48 85 d2                    	testq	%rdx, %rdx
14169a1c3: 74 12                       	je	0x14169a1d7 <.text+0x16991d7>
14169a1c5: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
14169a1cc: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14169a1d2: e8 49 9a f7 fe              	callq	0x140613c20 <.text+0x612c20>
14169a1d7: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14169a1df: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
14169a1e4: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14169a1ea: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14169a1f0: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
14169a1f6: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
14169a1fc: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14169a203: 5b                          	popq	%rbx
14169a204: 5f                          	popq	%rdi
14169a205: 5e                          	popq	%rsi
14169a206: 41 5c                       	popq	%r12
14169a208: 41 5d                       	popq	%r13
14169a20a: 41 5e                       	popq	%r14
14169a20c: 41 5f                       	popq	%r15
14169a20e: 5d                          	popq	%rbp
14169a20f: c3                          	retq
14169a210: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14169a215: 55                          	pushq	%rbp
14169a216: 41 57                       	pushq	%r15
14169a218: 41 56                       	pushq	%r14
14169a21a: 41 55                       	pushq	%r13
14169a21c: 41 54                       	pushq	%r12
14169a21e: 56                          	pushq	%rsi
14169a21f: 57                          	pushq	%rdi
14169a220: 53                          	pushq	%rbx
14169a221: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
14169a228: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14169a22f: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
14169a236: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
14169a23d: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
14169a243: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
14169a24a: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
14169a250: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
14169a259: 0f b6 9d 16 03 00 00        	movzbl	0x316(%rbp), %ebx
14169a260: 0f b6 b5 17 03 00 00        	movzbl	0x317(%rbp), %esi
14169a267: 80 bd 15 03 00 00 00        	cmpb	$0x0, 0x315(%rbp)
14169a26e: 74 22                       	je	0x14169a292 <.text+0x1699292>
14169a270: 48 8b 95 a0 01 00 00        	movq	0x1a0(%rbp), %rdx
14169a277: 48 85 d2                    	testq	%rdx, %rdx
14169a27a: 74 16                       	je	0x14169a292 <.text+0x1699292>
14169a27c: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
14169a283: 48 c1 e2 03                 	shlq	$0x3, %rdx
14169a287: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14169a28d: e8 8e 99 f7 fe              	callq	0x140613c20 <.text+0x612c20>
14169a292: 41 b8 48 00 00 00           	movl	$0x48, %r8d
14169a298: 41 b9 10 00 00 00           	movl	$0x10, %r9d
14169a29e: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
14169a2a5: 48 8d 95 20 02 00 00        	leaq	0x220(%rbp), %rdx
14169a2ac: e8 5f 4a 9b fe              	callq	0x14004ed10 <.text+0x4dd10>
14169a2b1: 8b 85 d0 01 00 00           	movl	0x1d0(%rbp), %eax
14169a2b7: 83 f8 01                    	cmpl	$0x1, %eax
14169a2ba: 74 2e                       	je	0x14169a2ea <.text+0x16992ea>
14169a2bc: 83 f8 04                    	cmpl	$0x4, %eax
14169a2bf: 75 3b                       	jne	0x14169a2fc <.text+0x16992fc>
14169a2c1: f6 c3 01                    	testb	$0x1, %bl
14169a2c4: 74 36                       	je	0x14169a2fc <.text+0x16992fc>
14169a2c6: 48 8b 95 d8 01 00 00        	movq	0x1d8(%rbp), %rdx
14169a2cd: 48 85 d2                    	testq	%rdx, %rdx
14169a2d0: 74 2a                       	je	0x14169a2fc <.text+0x16992fc>
14169a2d2: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
14169a2d9: 48 c1 e2 03                 	shlq	$0x3, %rdx
14169a2dd: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14169a2e3: e8 38 99 f7 fe              	callq	0x140613c20 <.text+0x612c20>
14169a2e8: eb 12                       	jmp	0x14169a2fc <.text+0x16992fc>
14169a2ea: 40 f6 c6 01                 	testb	$0x1, %sil
14169a2ee: 74 0c                       	je	0x14169a2fc <.text+0x16992fc>
14169a2f0: 48 8d 8d d8 01 00 00        	leaq	0x1d8(%rbp), %rcx
14169a2f7: e8 b4 81 54 ff              	callq	0x140be24b0 <.text+0xbe14b0>
14169a2fc: 48 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %rdx
14169a303: 48 85 d2                    	testq	%rdx, %rdx
14169a306: 74 12                       	je	0x14169a31a <.text+0x169931a>
14169a308: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
14169a30f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14169a315: e8 06 99 f7 fe              	callq	0x140613c20 <.text+0x612c20>
14169a31a: 66 0f 6f b4 24 80 00 00 00  	movdqa	0x80(%rsp), %xmm6
14169a323: 66 0f 6f 7c 24 70           	movdqa	0x70(%rsp), %xmm7
14169a329: 66 44 0f 6f 44 24 60        	movdqa	0x60(%rsp), %xmm8
14169a330: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14169a336: 66 44 0f 6f 54 24 40        	movdqa	0x40(%rsp), %xmm10
14169a33d: 66 44 0f 6f 5c 24 30        	movdqa	0x30(%rsp), %xmm11
14169a344: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14169a34b: 5b                          	popq	%rbx
14169a34c: 5f                          	popq	%rdi
14169a34d: 5e                          	popq	%rsi
14169a34e: 41 5c                       	popq	%r12
14169a350: 41 5d                       	popq	%r13
14169a352: 41 5e                       	popq	%r14
14169a354: 41 5f                       	popq	%r15
14169a356: 5d                          	popq	%rbp
14169a357: c3                          	retq
14169a358: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14169a360: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14169a365: 55                          	pushq	%rbp
14169a366: 41 57                       	pushq	%r15
14169a368: 41 56                       	pushq	%r14
14169a36a: 41 55                       	pushq	%r13
14169a36c: 41 54                       	pushq	%r12
14169a36e: 56                          	pushq	%rsi
14169a36f: 57                          	pushq	%rdi
14169a370: 53                          	pushq	%rbx
14169a371: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
14169a378: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14169a37f: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
14169a385: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
14169a38b: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
14169a391: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
14169a397: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
14169a39c: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
14169a3a4: ba 00 03 00 00              	movl	$0x300, %edx            # imm = 0x300
14169a3a9: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14169a3af: 48 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %rcx
14169a3b6: e8 65 98 f7 fe              	callq	0x140613c20 <.text+0x612c20>
14169a3bb: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14169a3c3: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
14169a3c8: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14169a3ce: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14169a3d4: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
14169a3da: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
14169a3e0: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14169a3e7: 5b                          	popq	%rbx
14169a3e8: 5f                          	popq	%rdi
14169a3e9: 5e                          	popq	%rsi
14169a3ea: 41 5c                       	popq	%r12
14169a3ec: 41 5d                       	popq	%r13
14169a3ee: 41 5e                       	popq	%r14
14169a3f0: 41 5f                       	popq	%r15
14169a3f2: 5d                          	popq	%rbp
14169a3f3: c3                          	retq
14169a3f4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14169a400: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14169a405: 55                          	pushq	%rbp
14169a406: 41 57                       	pushq	%r15
14169a408: 41 56                       	pushq	%r14
14169a40a: 41 55                       	pushq	%r13
14169a40c: 41 54                       	pushq	%r12
14169a40e: 56                          	pushq	%rsi
14169a40f: 57                          	pushq	%rdi
14169a410: 53                          	pushq	%rbx
14169a411: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
14169a418: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14169a41f: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
14169a426: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
14169a42d: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
14169a433: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
14169a43a: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
14169a440: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
14169a449: 0f b6 9d 07 03 00 00        	movzbl	0x307(%rbp), %ebx
14169a450: 48 8d 8d e8 00 00 00        	leaq	0xe8(%rbp), %rcx
14169a457: e8 b4 c3 1f ff              	callq	0x140896810 <.text+0x895810>
14169a45c: 88 9d 14 03 00 00           	movb	%bl, 0x314(%rbp)
14169a462: 66 0f 6f b4 24 80 00 00 00  	movdqa	0x80(%rsp), %xmm6
14169a46b: 66 0f 6f 7c 24 70           	movdqa	0x70(%rsp), %xmm7
14169a471: 66 44 0f 6f 44 24 60        	movdqa	0x60(%rsp), %xmm8
14169a478: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14169a47e: 66 44 0f 6f 54 24 40        	movdqa	0x40(%rsp), %xmm10
14169a485: 66 44 0f 6f 5c 24 30        	movdqa	0x30(%rsp), %xmm11
14169a48c: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14169a493: 5b                          	popq	%rbx
14169a494: 5f                          	popq	%rdi
14169a495: 5e                          	popq	%rsi
14169a496: 41 5c                       	popq	%r12
14169a498: 41 5d                       	popq	%r13
14169a49a: 41 5e                       	popq	%r14
14169a49c: 41 5f                       	popq	%r15
14169a49e: 5d                          	popq	%rbp
14169a49f: c3                          	retq
14169a4a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14169a4a5: 55                          	pushq	%rbp
14169a4a6: 41 57                       	pushq	%r15
14169a4a8: 41 56                       	pushq	%r14
14169a4aa: 41 55                       	pushq	%r13
14169a4ac: 41 54                       	pushq	%r12
14169a4ae: 56                          	pushq	%rsi
14169a4af: 57                          	pushq	%rdi
14169a4b0: 53                          	pushq	%rbx
14169a4b1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
14169a4b8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14169a4bf: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
14169a4c6: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
14169a4cd: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
14169a4d3: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
14169a4da: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
14169a4e0: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
14169a4e9: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
14169a4f0: e8 5b 76 ac fe              	callq	0x140161b50 <.text+0x160b50>
14169a4f5: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14169a4fd: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
14169a502: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14169a508: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14169a50e: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
14169a514: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
14169a51a: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14169a521: 5b                          	popq	%rbx
14169a522: 5f                          	popq	%rdi
14169a523: 5e                          	popq	%rsi
14169a524: 41 5c                       	popq	%r12
14169a526: 41 5d                       	popq	%r13
14169a528: 41 5e                       	popq	%r14
14169a52a: 41 5f                       	popq	%r15
14169a52c: 5d                          	popq	%rbp
14169a52d: c3                          	retq
14169a52e: 66 90                       	nop
14169a530: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14169a535: 55                          	pushq	%rbp
14169a536: 41 57                       	pushq	%r15
14169a538: 41 56                       	pushq	%r14
14169a53a: 41 55                       	pushq	%r13
14169a53c: 41 54                       	pushq	%r12
14169a53e: 56                          	pushq	%rsi
14169a53f: 57                          	pushq	%rdi
14169a540: 53                          	pushq	%rbx
14169a541: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
14169a548: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14169a54f: 66 44 0f 7f 5c 24 30        	movdqa	%xmm11, 0x30(%rsp)
14169a556: 66 44 0f 7f 54 24 40        	movdqa	%xmm10, 0x40(%rsp)
14169a55d: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
14169a563: 66 44 0f 7f 44 24 60        	movdqa	%xmm8, 0x60(%rsp)
14169a56a: 66 0f 7f 7c 24 70           	movdqa	%xmm7, 0x70(%rsp)
14169a570: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
14169a579: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
14169a580: e8 1b c6 f6 fe              	callq	0x140606ba0 <.text+0x605ba0>
14169a585: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14169a58d: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
14169a592: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14169a598: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14169a59e: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
14169a5a4: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
14169a5aa: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14169a5b1: 5b                          	popq	%rbx
14169a5b2: 5f                          	popq	%rdi
14169a5b3: 5e                          	popq	%rsi
14169a5b4: 41 5c                       	popq	%r12
14169a5b6: 41 5d                       	popq	%r13
14169a5b8: 41 5e                       	popq	%r14
14169a5ba: 41 5f                       	popq	%r15
14169a5bc: 5d                          	popq	%rbp
14169a5bd: c3                          	retq
14169a5be: 66 90                       	nop
14169a5c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14169a5c5: 55                          	pushq	%rbp
14169a5c6: 41 57                       	pushq	%r15
14169a5c8: 41 56                       	pushq	%r14
14169a5ca: 41 55                       	pushq	%r13
14169a5cc: 41 54                       	pushq	%r12
14169a5ce: 56                          	pushq	%rsi
14169a5cf: 57                          	pushq	%rdi
14169a5d0: 53                          	pushq	%rbx
14169a5d1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
14169a5d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14169a5df: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
14169a5e5: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
14169a5eb: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
14169a5f1: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
14169a5f7: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
14169a5fc: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
14169a604: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
14169a60b: e8 a0 7e 54 ff              	callq	0x140be24b0 <.text+0xbe14b0>
14169a610: 66 0f 6f b4 24 80 00 00 00  	movdqa	0x80(%rsp), %xmm6
14169a619: 66 0f 6f 7c 24 70           	movdqa	0x70(%rsp), %xmm7
14169a61f: 66 44 0f 6f 44 24 60        	movdqa	0x60(%rsp), %xmm8
14169a626: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14169a62c: 66 44 0f 6f 54 24 40        	movdqa	0x40(%rsp), %xmm10
14169a633: 66 44 0f 6f 5c 24 30        	movdqa	0x30(%rsp), %xmm11
14169a63a: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14169a641: 5b                          	popq	%rbx
14169a642: 5f                          	popq	%rdi
14169a643: 5e                          	popq	%rsi
14169a644: 41 5c                       	popq	%r12
14169a646: 41 5d                       	popq	%r13
14169a648: 41 5e                       	popq	%r14
14169a64a: 41 5f                       	popq	%r15
14169a64c: 5d                          	popq	%rbp
14169a64d: c3                          	retq
