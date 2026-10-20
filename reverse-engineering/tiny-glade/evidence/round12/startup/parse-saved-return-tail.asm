
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141473b00: 00 0f                       	addb	%cl, (%rdi)
141473b02: 29 b4 24 c0 00 00 00        	subl	%esi, 0xc0(%rsp)
141473b09: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
141473b10: 48 85 d2                    	testq	%rdx, %rdx
141473b13: 74 16                       	je	0x141473b2b <.text+0x1472b2b>
141473b15: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
141473b1c: 48 c1 e2 04                 	shlq	$0x4, %rdx
141473b20: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141473b26: e8 f5 00 1a ff              	callq	0x140613c20 <.text+0x612c20>
141473b2b: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473b33: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473b3b: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473b44: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473b4d: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473b56: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473b5c: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473b62: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473b68: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
141473b6f: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473b75: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473b7c: 5b                          	popq	%rbx
141473b7d: 5f                          	popq	%rdi
141473b7e: 5e                          	popq	%rsi
141473b7f: 41 5c                       	popq	%r12
141473b81: 41 5d                       	popq	%r13
141473b83: 41 5e                       	popq	%r14
141473b85: 41 5f                       	popq	%r15
141473b87: 5d                          	popq	%rbp
141473b88: c3                          	retq
141473b89: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141473b90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473b95: 55                          	pushq	%rbp
141473b96: 41 57                       	pushq	%r15
141473b98: 41 56                       	pushq	%r14
141473b9a: 41 55                       	pushq	%r13
141473b9c: 41 54                       	pushq	%r12
141473b9e: 56                          	pushq	%rsi
141473b9f: 57                          	pushq	%rdi
141473ba0: 53                          	pushq	%rbx
141473ba1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473ba8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141473baf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473bb5: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
141473bbc: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473bc2: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473bc8: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141473bce: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473bd7: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473be0: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141473be9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473bf1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473bf9: 48 83 bd 20 03 00 00 ff     	cmpq	$-0x1, 0x320(%rbp)
141473c01: 74 24                       	je	0x141473c27 <.text+0x1472c27>
141473c03: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
141473c0a: f0                          	lock
141473c0b: 48 ff 08                    	decq	(%rax)
141473c0e: 75 17                       	jne	0x141473c27 <.text+0x1472c27>
141473c10: ba 40 00 00 00              	movl	$0x40, %edx
141473c15: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141473c1b: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
141473c22: e8 f9 ff 19 ff              	callq	0x140613c20 <.text+0x612c20>
141473c27: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473c2e: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473c36: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473c3e: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473c47: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473c50: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473c59: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473c5f: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473c65: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473c6b: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141473c71: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473c77: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473c7e: 5b                          	popq	%rbx
141473c7f: 5f                          	popq	%rdi
141473c80: 5e                          	popq	%rsi
141473c81: 41 5c                       	popq	%r12
141473c83: 41 5d                       	popq	%r13
141473c85: 41 5e                       	popq	%r14
141473c87: 41 5f                       	popq	%r15
141473c89: 5d                          	popq	%rbp
141473c8a: c3                          	retq
141473c8b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141473c90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473c95: 55                          	pushq	%rbp
141473c96: 41 57                       	pushq	%r15
141473c98: 41 56                       	pushq	%r14
141473c9a: 41 55                       	pushq	%r13
141473c9c: 41 54                       	pushq	%r12
141473c9e: 56                          	pushq	%rsi
141473c9f: 57                          	pushq	%rdi
141473ca0: 53                          	pushq	%rbx
141473ca1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473ca8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141473caf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473cb5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141473cbb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473cc1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473cc7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141473ccd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473cd6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473cdf: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
141473ce9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473cf1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473cf9: 80 bd 35 03 00 00 00        	cmpb	$0x0, 0x335(%rbp)
141473d00: 74 77                       	je	0x141473d79 <.text+0x1472d79>
141473d02: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473d09: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
141473d10: e8 1b 5e 40 ff              	callq	0x140879b30 <.text+0x878b30>
141473d15: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473d1c: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473d24: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473d2c: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473d35: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473d3e: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473d47: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473d4d: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473d53: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473d59: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141473d5f: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473d65: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473d6c: 5b                          	popq	%rbx
141473d6d: 5f                          	popq	%rdi
141473d6e: 5e                          	popq	%rsi
141473d6f: 41 5c                       	popq	%r12
141473d71: 41 5d                       	popq	%r13
141473d73: 41 5e                       	popq	%r14
141473d75: 41 5f                       	popq	%r15
141473d77: 5d                          	popq	%rbp
141473d78: c3                          	retq
141473d79: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473d80: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473d88: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473d90: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473d99: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473da2: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473dab: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473db1: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473db7: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473dbd: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141473dc3: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473dc9: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473dd0: 5b                          	popq	%rbx
141473dd1: 5f                          	popq	%rdi
141473dd2: 5e                          	popq	%rsi
141473dd3: 41 5c                       	popq	%r12
141473dd5: 41 5d                       	popq	%r13
141473dd7: 41 5e                       	popq	%r14
141473dd9: 41 5f                       	popq	%r15
141473ddb: 5d                          	popq	%rbp
141473ddc: c3                          	retq
141473ddd: 0f 1f 00                    	nopl	(%rax)
141473de0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473de5: 55                          	pushq	%rbp
141473de6: 41 57                       	pushq	%r15
141473de8: 41 56                       	pushq	%r14
141473dea: 41 55                       	pushq	%r13
141473dec: 41 54                       	pushq	%r12
141473dee: 56                          	pushq	%rsi
141473def: 57                          	pushq	%rdi
141473df0: 53                          	pushq	%rbx
141473df1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473df8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141473dff: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473e05: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
141473e0c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473e12: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473e18: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141473e1e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473e27: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473e30: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141473e39: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473e41: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473e49: 48 b8 03 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffd, %rax # imm = 0x8000000000000003
141473e53: 48 8b 95 08 03 00 00        	movq	0x308(%rbp), %rdx
141473e5a: 48 89 42 10                 	movq	%rax, 0x10(%rdx)
141473e5e: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
141473e65: 48 89 42 18                 	movq	%rax, 0x18(%rdx)
141473e69: 8b 85 58 01 00 00           	movl	0x158(%rbp), %eax
141473e6f: 89 42 20                    	movl	%eax, 0x20(%rdx)
141473e72: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
141473e79: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
141473e80: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
141473e84: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
141473e8b: 0f 11 01                    	movups	%xmm0, (%rcx)
141473e8e: 8b 85 20 03 00 00           	movl	0x320(%rbp), %eax
141473e94: 89 42 3c                    	movl	%eax, 0x3c(%rdx)
141473e97: 0f 10 85 70 02 00 00        	movups	0x270(%rbp), %xmm0
141473e9e: 0f 11 42 40                 	movups	%xmm0, 0x40(%rdx)
141473ea2: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
141473ea9: 48 89 42 50                 	movq	%rax, 0x50(%rdx)
141473ead: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473eb4: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473ebc: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473ec4: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473ecd: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473ed6: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473edf: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473ee5: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473eeb: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473ef1: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141473ef7: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473efd: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473f04: 5b                          	popq	%rbx
141473f05: 5f                          	popq	%rdi
141473f06: 5e                          	popq	%rsi
141473f07: 41 5c                       	popq	%r12
141473f09: 41 5d                       	popq	%r13
141473f0b: 41 5e                       	popq	%r14
141473f0d: 41 5f                       	popq	%r15
141473f0f: 5d                          	popq	%rbp
141473f10: c3                          	retq
141473f11: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141473f20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473f25: 55                          	pushq	%rbp
141473f26: 41 57                       	pushq	%r15
141473f28: 41 56                       	pushq	%r14
141473f2a: 41 55                       	pushq	%r13
141473f2c: 41 54                       	pushq	%r12
141473f2e: 56                          	pushq	%rsi
141473f2f: 57                          	pushq	%rdi
141473f30: 53                          	pushq	%rbx
141473f31: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473f38: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141473f3f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473f45: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
141473f4c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473f52: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473f58: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141473f5e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473f67: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473f70: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141473f79: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473f81: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473f89: 80 bd 37 03 00 00 00        	cmpb	$0x0, 0x337(%rbp)
141473f90: 74 6a                       	je	0x141473ffc <.text+0x1472ffc>
141473f92: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
141473f99: e8 12 b9 41 ff              	callq	0x14088f8b0 <.text+0x88e8b0>
141473f9e: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473fa6: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473fae: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473fb7: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473fc0: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473fc9: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473fcf: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473fd5: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473fdb: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
141473fe2: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473fe8: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473fef: 5b                          	popq	%rbx
141473ff0: 5f                          	popq	%rdi
141473ff1: 5e                          	popq	%rsi
141473ff2: 41 5c                       	popq	%r12
141473ff4: 41 5d                       	popq	%r13
141473ff6: 41 5e                       	popq	%r14
141473ff8: 41 5f                       	popq	%r15
141473ffa: 5d                          	popq	%rbp
141473ffb: c3                          	retq
141473ffc: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141474004: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
14147400c: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141474015: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
14147401e: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141474027: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
14147402d: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141474033: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141474039: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
141474040: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141474046: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
14147404d: 5b                          	popq	%rbx
14147404e: 5f                          	popq	%rdi
14147404f: 5e                          	popq	%rsi
141474050: 41 5c                       	popq	%r12
141474052: 41 5d                       	popq	%r13
141474054: 41 5e                       	popq	%r14
141474056: 41 5f                       	popq	%r15
141474058: 5d                          	popq	%rbp
141474059: c3                          	retq
14147405a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141474060: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141474065: 55                          	pushq	%rbp
141474066: 41 57                       	pushq	%r15
141474068: 41 56                       	pushq	%r14
14147406a: 41 55                       	pushq	%r13
14147406c: 41 54                       	pushq	%r12
14147406e: 56                          	pushq	%rsi
14147406f: 57                          	pushq	%rdi
141474070: 53                          	pushq	%rbx
141474071: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141474078: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147407f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141474085: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147408c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141474092: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141474098: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147409e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1414740a7: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1414740b0: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1414740b9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1414740c1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1414740c9: 48 8b 95 40 01 00 00        	movq	0x140(%rbp), %rdx
1414740d0: 48 85 d2                    	testq	%rdx, %rdx
1414740d3: 74 16                       	je	0x1414740eb <.text+0x14730eb>
1414740d5: 48 c1 e2 05                 	shlq	$0x5, %rdx
1414740d9: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1414740df: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
1414740e6: e8 35 fb 19 ff              	callq	0x140613c20 <.text+0x612c20>
1414740eb: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1414740f3: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1414740fb: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141474104: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
14147410d: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141474116: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
14147411c: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141474122: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141474128: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
14147412f: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141474135: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
14147413c: 5b                          	popq	%rbx
14147413d: 5f                          	popq	%rdi
14147413e: 5e                          	popq	%rsi
14147413f: 41 5c                       	popq	%r12
141474141: 41 5d                       	popq	%r13
141474143: 41 5e                       	popq	%r14
141474145: 41 5f                       	popq	%r15
141474147: 5d                          	popq	%rbp
141474148: c3                          	retq
141474149: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141474150: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141474155: 55                          	pushq	%rbp
141474156: 41 57                       	pushq	%r15
141474158: 41 56                       	pushq	%r14
14147415a: 41 55                       	pushq	%r13
14147415c: 41 54                       	pushq	%r12
14147415e: 56                          	pushq	%rsi
14147415f: 57                          	pushq	%rdi
141474160: 53                          	pushq	%rbx
141474161: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141474168: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147416f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141474175: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147417c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141474182: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141474188: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147418e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141474197: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1414741a0: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1414741a9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1414741b1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1414741b9: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
1414741c0: e8 9b 0b 42 ff              	callq	0x140894d60 <.text+0x893d60>
1414741c5: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1414741cd: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1414741d5: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1414741de: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1414741e7: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1414741f0: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1414741f6: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1414741fc: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141474202: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
141474209: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
14147420f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141474216: 5b                          	popq	%rbx
141474217: 5f                          	popq	%rdi
141474218: 5e                          	popq	%rsi
141474219: 41 5c                       	popq	%r12
14147421b: 41 5d                       	popq	%r13
14147421d: 41 5e                       	popq	%r14
14147421f: 41 5f                       	popq	%r15
141474221: 5d                          	popq	%rbp
141474222: c3                          	retq
141474223: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141474230: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141474235: 55                          	pushq	%rbp
141474236: 41 57                       	pushq	%r15
141474238: 41 56                       	pushq	%r14
14147423a: 41 55                       	pushq	%r13
14147423c: 41 54                       	pushq	%r12
14147423e: 56                          	pushq	%rsi
14147423f: 57                          	pushq	%rdi
141474240: 53                          	pushq	%rbx
141474241: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141474248: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147424f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141474255: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147425c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141474262: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141474268: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147426e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141474277: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141474280: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141474289: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141474291: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141474299: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1414742a0: e8 9b a8 41 ff              	callq	0x14088eb40 <.text+0x88db40>
1414742a5: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1414742ad: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1414742b5: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1414742be: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1414742c7: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1414742d0: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1414742d6: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1414742dc: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1414742e2: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
1414742e9: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1414742ef: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1414742f6: 5b                          	popq	%rbx
1414742f7: 5f                          	popq	%rdi
1414742f8: 5e                          	popq	%rsi
1414742f9: 41 5c                       	popq	%r12
1414742fb: 41 5d                       	popq	%r13
1414742fd: 41 5e                       	popq	%r14
1414742ff: 41 5f                       	popq	%r15
141474301: 5d                          	popq	%rbp
141474302: c3                          	retq
141474303: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141474310: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141474315: 55                          	pushq	%rbp
141474316: 41 57                       	pushq	%r15
141474318: 41 56                       	pushq	%r14
14147431a: 41 55                       	pushq	%r13
14147431c: 41 54                       	pushq	%r12
14147431e: 56                          	pushq	%rsi
14147431f: 57                          	pushq	%rdi
141474320: 53                          	pushq	%rbx
141474321: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141474328: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147432f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141474335: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
14147433b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141474341: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141474347: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147434d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141474356: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
14147435f: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
141474369: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141474371: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141474379: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
141474380: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141474387: 48 89 08                    	movq	%rcx, (%rax)
14147438a: c6 85 35 03 00 00 00        	movb	$0x0, 0x335(%rbp)
141474391: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141474399: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1414743a1: 66 44 0f 6f 84 24 a0 00 00 00       	movdqa	0xa0(%rsp), %xmm8
1414743ab: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1414743b4: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1414743bd: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1414743c3: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1414743c9: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1414743cf: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1414743d5: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1414743db: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1414743e2: 5b                          	popq	%rbx
1414743e3: 5f                          	popq	%rdi
1414743e4: 5e                          	popq	%rsi
1414743e5: 41 5c                       	popq	%r12
1414743e7: 41 5d                       	popq	%r13
1414743e9: 41 5e                       	popq	%r14
1414743eb: 41 5f                       	popq	%r15
1414743ed: 5d                          	popq	%rbp
1414743ee: c3                          	retq
1414743ef: 90                          	nop
1414743f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414743f5: 55                          	pushq	%rbp
1414743f6: 41 57                       	pushq	%r15
1414743f8: 41 56                       	pushq	%r14
1414743fa: 41 55                       	pushq	%r13
1414743fc: 41 54                       	pushq	%r12
1414743fe: 56                          	pushq	%rsi
1414743ff: 57                          	pushq	%rdi
141474400: 53                          	pushq	%rbx
141474401: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141474408: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147440f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141474415: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147441c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141474422: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141474428: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147442e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141474437: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141474440: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141474449: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141474451: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141474459: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
141474460: e8 cb 87 8d ff              	callq	0x140d4cc30 <.text+0xd4bc30>
141474465: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
14147446d: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141474475: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
14147447e: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141474487: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141474490: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141474496: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
14147449c: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1414744a2: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
1414744a9: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1414744af: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1414744b6: 5b                          	popq	%rbx
1414744b7: 5f                          	popq	%rdi
1414744b8: 5e                          	popq	%rsi
1414744b9: 41 5c                       	popq	%r12
1414744bb: 41 5d                       	popq	%r13
1414744bd: 41 5e                       	popq	%r14
1414744bf: 41 5f                       	popq	%r15
1414744c1: 5d                          	popq	%rbp
1414744c2: c3                          	retq
