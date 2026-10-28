
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403b6b40: 55                          	pushq	%rbp
1403b6b41: 41 57                       	pushq	%r15
1403b6b43: 41 56                       	pushq	%r14
1403b6b45: 41 55                       	pushq	%r13
1403b6b47: 41 54                       	pushq	%r12
1403b6b49: 56                          	pushq	%rsi
1403b6b4a: 57                          	pushq	%rdi
1403b6b4b: 53                          	pushq	%rbx
1403b6b4c: 48 81 ec 78 03 00 00        	subq	$0x378, %rsp            # imm = 0x378
1403b6b53: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1403b6b5b: 44 0f 29 a5 e0 02 00 00     	movaps	%xmm12, 0x2e0(%rbp)
1403b6b63: 44 0f 29 9d d0 02 00 00     	movaps	%xmm11, 0x2d0(%rbp)
1403b6b6b: 44 0f 29 95 c0 02 00 00     	movaps	%xmm10, 0x2c0(%rbp)
1403b6b73: 44 0f 29 8d b0 02 00 00     	movaps	%xmm9, 0x2b0(%rbp)
1403b6b7b: 44 0f 29 85 a0 02 00 00     	movaps	%xmm8, 0x2a0(%rbp)
1403b6b83: 0f 29 bd 90 02 00 00        	movaps	%xmm7, 0x290(%rbp)
1403b6b8a: 0f 29 b5 80 02 00 00        	movaps	%xmm6, 0x280(%rbp)
1403b6b91: 48 c7 85 78 02 00 00 fe ff ff ff    	movq	$-0x2, 0x278(%rbp)
1403b6b9c: 4c 89 8d 18 02 00 00        	movq	%r9, 0x218(%rbp)
1403b6ba3: 49 8b 70 10                 	movq	0x10(%r8), %rsi
1403b6ba7: 48 85 f6                    	testq	%rsi, %rsi
1403b6baa: 4c 89 85 20 02 00 00        	movq	%r8, 0x220(%rbp)
1403b6bb1: 48 89 95 28 02 00 00        	movq	%rdx, 0x228(%rbp)
1403b6bb8: 0f 84 2a 09 00 00           	je	0x1403b74e8 <.text+0x3b64e8>
1403b6bbe: 48 89 cb                    	movq	%rcx, %rbx
1403b6bc1: 49 8b 40 08                 	movq	0x8(%r8), %rax
1403b6bc5: f3 44 0f 10 40 14           	movss	0x14(%rax), %xmm8
1403b6bcb: f3 0f 10 70 18              	movss	0x18(%rax), %xmm6
1403b6bd0: 49 89 c5                    	movq	%rax, %r13
1403b6bd3: f3 44 0f 10 48 10           	movss	0x10(%rax), %xmm9
1403b6bd9: 49 89 d4                    	movq	%rdx, %r12
1403b6bdc: f3 44 0f 10 52 1c           	movss	0x1c(%rdx), %xmm10
1403b6be2: f3 0f 10 05 9e ed 56 02     	movss	0x256ed9e(%rip), %xmm0  # 0x142925988
1403b6bea: f3 41 0f 59 c2              	mulss	%xmm10, %xmm0
1403b6bef: e8 6c d2 56 02              	callq	0x142923e60 <.text+0x2922e60>
1403b6bf4: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
1403b6bf9: 48 89 c1                    	movq	%rax, %rcx
1403b6bfc: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
1403b6c00: 0f 28 c8                    	movaps	%xmm0, %xmm1
1403b6c03: f3 0f 5c 0d d1 34 5a 02     	subss	0x25a34d1(%rip), %xmm1  # 0x14295a0dc
1403b6c0b: f3 48 0f 2c d1              	cvttss2si	%xmm1, %rdx
1403b6c10: 48 21 ca                    	andq	%rcx, %rdx
1403b6c13: 48 09 c2                    	orq	%rax, %rdx
1403b6c16: 45 31 f6                    	xorl	%r14d, %r14d
1403b6c19: 0f 57 ff                    	xorps	%xmm7, %xmm7
1403b6c1c: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
1403b6c1f: 49 0f 42 d6                 	cmovbq	%r14, %rdx
1403b6c23: 0f 2e 05 b6 34 5a 02        	ucomiss	0x25a34b6(%rip), %xmm0  # 0x14295a0e0
1403b6c2a: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
1403b6c31: 48 0f 46 c2                 	cmovbeq	%rdx, %rax
1403b6c35: 48 83 f8 03                 	cmpq	$0x3, %rax
1403b6c39: 41 bf 02 00 00 00           	movl	$0x2, %r15d
1403b6c3f: 4c 0f 43 f8                 	cmovaeq	%rax, %r15
1403b6c43: 4c 89 f8                    	movq	%r15, %rax
1403b6c46: 48 ff c8                    	decq	%rax
1403b6c49: 48 89 9d 60 01 00 00        	movq	%rbx, 0x160(%rbp)
1403b6c50: 78 07                       	js	0x1403b6c59 <.text+0x3b5c59>
1403b6c52: f3 4c 0f 2a d8              	cvtsi2ss	%rax, %xmm11
1403b6c57: eb 16                       	jmp	0x1403b6c6f <.text+0x3b5c6f>
1403b6c59: 48 89 c1                    	movq	%rax, %rcx
1403b6c5c: 48 d1 e9                    	shrq	%rcx
1403b6c5f: 83 e0 01                    	andl	$0x1, %eax
1403b6c62: 48 09 c8                    	orq	%rcx, %rax
1403b6c65: f3 4c 0f 2a d8              	cvtsi2ss	%rax, %xmm11
1403b6c6a: f3 45 0f 58 db              	addss	%xmm11, %xmm11
1403b6c6f: 48 8b 9d 60 03 00 00        	movq	0x360(%rbp), %rbx
1403b6c76: 44 0f 14 ce                 	unpcklps	%xmm6, %xmm9            # xmm9 = xmm9[0],xmm6[0],xmm9[1],xmm6[1]
1403b6c7a: f3 44 0f 10 25 fd ec 56 02  	movss	0x256ecfd(%rip), %xmm12 # 0x142925980
1403b6c83: 48 8d 7d c0                 	leaq	-0x40(%rbp), %rdi
1403b6c87: 4c 89 e2                    	movq	%r12, %rdx
1403b6c8a: 4c 8b a5 18 02 00 00        	movq	0x218(%rbp), %r12
1403b6c91: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1403b6ca0: 4d 85 f6                    	testq	%r14, %r14
1403b6ca3: 78 0b                       	js	0x1403b6cb0 <.text+0x3b5cb0>
1403b6ca5: 0f 57 f6                    	xorps	%xmm6, %xmm6
1403b6ca8: f3 49 0f 2a f6              	cvtsi2ss	%r14, %xmm6
1403b6cad: eb 1c                       	jmp	0x1403b6ccb <.text+0x3b5ccb>
1403b6caf: 90                          	nop
1403b6cb0: 4c 89 f0                    	movq	%r14, %rax
1403b6cb3: 48 d1 e8                    	shrq	%rax
1403b6cb6: 44 89 f1                    	movl	%r14d, %ecx
1403b6cb9: 83 e1 01                    	andl	$0x1, %ecx
1403b6cbc: 48 09 c1                    	orq	%rax, %rcx
1403b6cbf: 0f 57 f6                    	xorps	%xmm6, %xmm6
1403b6cc2: f3 48 0f 2a f1              	cvtsi2ss	%rcx, %xmm6
1403b6cc7: f3 0f 58 f6                 	addss	%xmm6, %xmm6
1403b6ccb: f3 41 0f 5e f3              	divss	%xmm11, %xmm6
1403b6cd0: f3 41 0f 59 f2              	mulss	%xmm10, %xmm6
1403b6cd5: 48 89 f9                    	movq	%rdi, %rcx
1403b6cd8: 0f 28 d6                    	movaps	%xmm6, %xmm2
1403b6cdb: e8 c0 d9 8d 00              	callq	0x140c946a0 <.text+0xc936a0>
1403b6ce0: 49 ff c6                    	incq	%r14
1403b6ce3: f3 0f 10 45 c4              	movss	-0x3c(%rbp), %xmm0
1403b6ce8: f3 41 0f 5c c0              	subss	%xmm8, %xmm0
1403b6ced: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1403b6cf1: f3 0f 10 4d c8              	movss	-0x38(%rbp), %xmm1
1403b6cf6: f3 0f 10 55 c0              	movss	-0x40(%rbp), %xmm2
1403b6cfb: 0f 14 d1                    	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
1403b6cfe: 41 0f 5c d1                 	subps	%xmm9, %xmm2
1403b6d02: 0f 59 d2                    	mulps	%xmm2, %xmm2
1403b6d05: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1403b6d09: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
1403b6d0d: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1403b6d11: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403b6d14: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
1403b6d18: 0f 28 c8                    	movaps	%xmm0, %xmm1
1403b6d1b: f3 41 0f 5d cc              	minss	%xmm12, %xmm1
1403b6d20: f3 41 0f c2 c4 01           	cmpltss	%xmm12, %xmm0
1403b6d26: 0f 54 f0                    	andps	%xmm0, %xmm6
1403b6d29: 0f 55 c7                    	andnps	%xmm7, %xmm0
1403b6d2c: 0f 56 c6                    	orps	%xmm6, %xmm0
1403b6d2f: 0f 28 f8                    	movaps	%xmm0, %xmm7
1403b6d32: 44 0f 28 e1                 	movaps	%xmm1, %xmm12
1403b6d36: 4d 39 fe                    	cmpq	%r15, %r14
1403b6d39: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
1403b6d40: 0f 85 5a ff ff ff           	jne	0x1403b6ca0 <.text+0x3b5ca0>
1403b6d46: 48 69 c6 b0 01 00 00        	imulq	$0x1b0, %rsi, %rax      # imm = 0x1B0
1403b6d4d: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1403b6d54: 4d 85 e4                    	testq	%r12, %r12
1403b6d57: 78 07                       	js	0x1403b6d60 <.text+0x3b5d60>
1403b6d59: f3 4d 0f 2a c4              	cvtsi2ss	%r12, %xmm8
1403b6d5e: eb 19                       	jmp	0x1403b6d79 <.text+0x3b5d79>
1403b6d60: 4c 89 e0                    	movq	%r12, %rax
1403b6d63: 48 d1 e8                    	shrq	%rax
1403b6d66: 44 89 e1                    	movl	%r12d, %ecx
1403b6d69: 83 e1 01                    	andl	$0x1, %ecx
1403b6d6c: 48 09 c1                    	orq	%rax, %rcx
1403b6d6f: f3 4c 0f 2a c1              	cvtsi2ss	%rcx, %xmm8
1403b6d74: f3 45 0f 58 c0              	addss	%xmm8, %xmm8
1403b6d79: 4d 89 ee                    	movq	%r13, %r14
1403b6d7c: 4c 01 ad 10 02 00 00        	addq	%r13, 0x210(%rbp)
1403b6d83: 48 8d 7d c8                 	leaq	-0x38(%rbp), %rdi
1403b6d87: 31 f6                       	xorl	%esi, %esi
1403b6d89: 44 0f 28 0d ef 7b 57 02     	movaps	0x2577bef(%rip), %xmm9  # 0x14292e980
1403b6d91: f3 44 0f 10 15 6a eb 56 02  	movss	0x256eb6a(%rip), %xmm10 # 0x142925904
1403b6d9a: f3 0f 10 35 2e eb 56 02     	movss	0x256eb2e(%rip), %xmm6  # 0x1429258d0
1403b6da2: 4c 8d 7d c0                 	leaq	-0x40(%rbp), %r15
1403b6da6: 4c 8d ad 30 02 00 00        	leaq	0x230(%rbp), %r13
1403b6dad: f3 44 0f 10 1d 7a 62 61 02  	movss	0x261627a(%rip), %xmm11 # 0x1429cd030
1403b6db6: 45 0f 57 e4                 	xorps	%xmm12, %xmm12
1403b6dba: 4c 3b b5 10 02 00 00        	cmpq	0x210(%rbp), %r14
1403b6dc1: 0f 84 94 06 00 00           	je	0x1403b745b <.text+0x3b645b>
1403b6dc7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1403b6dd0: 48 85 f6                    	testq	%rsi, %rsi
1403b6dd3: 78 0b                       	js	0x1403b6de0 <.text+0x3b5de0>
1403b6dd5: 0f 57 c9                    	xorps	%xmm1, %xmm1
1403b6dd8: f3 48 0f 2a ce              	cvtsi2ss	%rsi, %xmm1
1403b6ddd: eb 1b                       	jmp	0x1403b6dfa <.text+0x3b5dfa>
1403b6ddf: 90                          	nop
1403b6de0: 48 89 f0                    	movq	%rsi, %rax
1403b6de3: 48 d1 e8                    	shrq	%rax
1403b6de6: 89 f1                       	movl	%esi, %ecx
1403b6de8: 83 e1 01                    	andl	$0x1, %ecx
1403b6deb: 48 09 c1                    	orq	%rax, %rcx
1403b6dee: 0f 57 c9                    	xorps	%xmm1, %xmm1
1403b6df1: f3 48 0f 2a c9              	cvtsi2ss	%rcx, %xmm1
1403b6df6: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1403b6dfa: f3 41 0f 5c c8              	subss	%xmm8, %xmm1
1403b6dff: 41 0f 54 c9                 	andps	%xmm9, %xmm1
1403b6e03: f3 41 0f 58 ca              	addss	%xmm10, %xmm1
1403b6e08: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
1403b6e0c: f3 0f 5e c1                 	divss	%xmm1, %xmm0
1403b6e10: 0f 28 ce                    	movaps	%xmm6, %xmm1
1403b6e13: e8 58 d1 56 02              	callq	0x142923f70 <.text+0x2922f70>
1403b6e18: f3 0f 11 85 6c 02 00 00     	movss	%xmm0, 0x26c(%rbp)
1403b6e20: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1403b6e26: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1403b6e2f: 48 c7 44 24 28 08 00 00 00  	movq	$0x8, 0x28(%rsp)
1403b6e38: 41 b9 1b 00 00 00           	movl	$0x1b, %r9d
1403b6e3e: 4c 89 f9                    	movq	%r15, %rcx
1403b6e41: 48 89 da                    	movq	%rbx, %rdx
1403b6e44: 4c 8d 05 2d 69 61 02        	leaq	0x261692d(%rip), %r8    # 0x1429cd778
1403b6e4b: e8 30 fa ff ff              	callq	0x1403b6880 <.text+0x3b5880>
1403b6e50: 83 7d c0 02                 	cmpl	$0x2, -0x40(%rbp)
1403b6e54: 74 08                       	je	0x1403b6e5e <.text+0x3b5e5e>
1403b6e56: 4c 89 f9                    	movq	%r15, %rcx
1403b6e59: e8 a2 04 6a 00              	callq	0x140a57300 <.text+0xa56300>
1403b6e5e: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1403b6e62: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
1403b6e69: 0f 10 45 c0                 	movups	-0x40(%rbp), %xmm0
1403b6e6d: 0f 10 4d d0                 	movups	-0x30(%rbp), %xmm1
1403b6e71: 0f 10 55 e0                 	movups	-0x20(%rbp), %xmm2
1403b6e75: 0f 29 95 40 01 00 00        	movaps	%xmm2, 0x140(%rbp)
1403b6e7c: 0f 29 8d 30 01 00 00        	movaps	%xmm1, 0x130(%rbp)
1403b6e83: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
1403b6e8a: 4c 39 e6                    	cmpq	%r12, %rsi
1403b6e8d: 0f 85 7d 00 00 00           	jne	0x1403b6f10 <.text+0x3b5f10>
1403b6e93: c6 85 68 01 00 00 00        	movb	$0x0, 0x168(%rbp)
1403b6e9a: 48 89 5d c0                 	movq	%rbx, -0x40(%rbp)
1403b6e9e: 4c 8d a5 6c 02 00 00        	leaq	0x26c(%rbp), %r12
1403b6ea5: 4c 89 65 c8                 	movq	%r12, -0x38(%rbp)
1403b6ea9: 48 b8 00 00 00 00 0a 00 00 00       	movabsq	$0xa00000000, %rax # imm = 0xA00000000
1403b6eb3: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
1403b6eb7: 48 8d 85 68 01 00 00        	leaq	0x168(%rbp), %rax
1403b6ebe: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1403b6ec2: c6 85 75 02 00 00 01        	movb	$0x1, 0x275(%rbp)
1403b6ec9: 4c 89 e9                    	movq	%r13, %rcx
1403b6ecc: 4c 89 fa                    	movq	%r15, %rdx
1403b6ecf: 4c 8d 05 42 db 5e 02        	leaq	0x25edb42(%rip), %r8    # 0x1429a4a18
1403b6ed6: e8 05 86 f0 ff              	callq	0x1402bf4e0 <.text+0x2be4e0>
1403b6edb: 80 bd 68 01 00 00 00        	cmpb	$0x0, 0x168(%rbp)
1403b6ee2: 74 6c                       	je	0x1403b6f50 <.text+0x3b5f50>
1403b6ee4: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403b6eee: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1403b6ef5: c6 85 75 02 00 00 01        	movb	$0x1, 0x275(%rbp)
1403b6efc: 4c 89 e9                    	movq	%r13, %rcx
1403b6eff: e8 1c 2c 25 00              	callq	0x140609b20 <.text+0x608b20>
1403b6f04: eb 66                       	jmp	0x1403b6f6c <.text+0x3b5f6c>
1403b6f06: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1403b6f10: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403b6f1a: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1403b6f21: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1403b6f28: 48 c7 85 e8 00 00 00 02 00 00 00    	movq	$0x2, 0xe8(%rbp)
1403b6f33: 4c 8d a5 6c 02 00 00        	leaq	0x26c(%rbp), %r12
1403b6f3a: 48 c7 85 68 01 00 00 02 00 00 00    	movq	$0x2, 0x168(%rbp)
1403b6f45: e9 9c 02 00 00              	jmp	0x1403b71e6 <.text+0x3b61e6>
1403b6f4a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1403b6f50: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
1403b6f57: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1403b6f5e: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
1403b6f65: 0f 29 85 e0 01 00 00        	movaps	%xmm0, 0x1e0(%rbp)
1403b6f6c: c6 85 68 01 00 00 00        	movb	$0x0, 0x168(%rbp)
1403b6f73: 48 89 5d c0                 	movq	%rbx, -0x40(%rbp)
1403b6f77: 4c 89 65 c8                 	movq	%r12, -0x38(%rbp)
1403b6f7b: 48 b8 00 00 00 00 0a 00 00 00       	movabsq	$0xa00000000, %rax # imm = 0xA00000000
1403b6f85: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
1403b6f89: 48 8d 85 68 01 00 00        	leaq	0x168(%rbp), %rax
1403b6f90: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1403b6f94: c6 85 74 02 00 00 01        	movb	$0x1, 0x274(%rbp)
1403b6f9b: 4c 89 e9                    	movq	%r13, %rcx
1403b6f9e: 4c 89 fa                    	movq	%r15, %rdx
1403b6fa1: 4c 8d 05 70 da 5e 02        	leaq	0x25eda70(%rip), %r8    # 0x1429a4a18
1403b6fa8: e8 33 9c f0 ff              	callq	0x1402c0be0 <.text+0x2bfbe0>
1403b6fad: 80 bd 68 01 00 00 00        	cmpb	$0x0, 0x168(%rbp)
1403b6fb4: 74 2a                       	je	0x1403b6fe0 <.text+0x3b5fe0>
1403b6fb6: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403b6fc0: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1403b6fc7: c6 85 74 02 00 00 01        	movb	$0x1, 0x274(%rbp)
1403b6fce: 4c 89 e9                    	movq	%r13, %rcx
1403b6fd1: e8 4a 2b 25 00              	callq	0x140609b20 <.text+0x608b20>
1403b6fd6: eb 24                       	jmp	0x1403b6ffc <.text+0x3b5ffc>
1403b6fd8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1403b6fe0: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
1403b6fe7: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1403b6fee: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
1403b6ff5: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
1403b6ffc: f3 0f 10 85 6c 02 00 00     	movss	0x26c(%rbp), %xmm0
1403b7004: c6 85 77 02 00 00 01        	movb	$0x1, 0x277(%rbp)
1403b700b: 48 8d 05 de 67 61 02        	leaq	0x26167de(%rip), %rax   # 0x1429cd7f0
1403b7012: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1403b7017: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1403b701d: 48 c7 44 24 30 04 00 00 00  	movq	$0x4, 0x30(%rsp)
1403b7026: 41 b9 28 00 00 00           	movl	$0x28, %r9d
1403b702c: 4c 89 f9                    	movq	%r15, %rcx
1403b702f: 48 89 da                    	movq	%rbx, %rdx
1403b7032: 4c 8d 05 17 68 61 02        	leaq	0x2616817(%rip), %r8    # 0x1429cd850
1403b7039: e8 42 f8 ff ff              	callq	0x1403b6880 <.text+0x3b5880>
1403b703e: 48 83 7d c0 02              	cmpq	$0x2, -0x40(%rbp)
1403b7043: 75 2b                       	jne	0x1403b7070 <.text+0x3b6070>
1403b7045: 0f 10 07                    	movups	(%rdi), %xmm0
1403b7048: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
1403b704c: 0f 10 57 20                 	movups	0x20(%rdi), %xmm2
1403b7050: 0f 29 95 50 02 00 00        	movaps	%xmm2, 0x250(%rbp)
1403b7057: 0f 29 8d 40 02 00 00        	movaps	%xmm1, 0x240(%rbp)
1403b705e: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
1403b7065: b8 02 00 00 00              	movl	$0x2, %eax
1403b706a: eb 64                       	jmp	0x1403b70d0 <.text+0x3b60d0>
1403b706c: 0f 1f 40 00                 	nopl	(%rax)
1403b7070: f3 0f 10 4d f0              	movss	-0x10(%rbp), %xmm1
1403b7075: f3 41 0f 59 cc              	mulss	%xmm12, %xmm1
1403b707a: 4c 89 f9                    	movq	%r15, %rcx
1403b707d: e8 1e 05 6a 00              	callq	0x140a575a0 <.text+0xa565a0>
1403b7082: 4c 89 f9                    	movq	%r15, %rcx
1403b7085: e8 76 02 6a 00              	callq	0x140a57300 <.text+0xa56300>
1403b708a: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
1403b708e: 0f 10 07                    	movups	(%rdi), %xmm0
1403b7091: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
1403b7095: 0f 10 57 20                 	movups	0x20(%rdi), %xmm2
1403b7099: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
1403b70a0: 0f 29 8d 40 02 00 00        	movaps	%xmm1, 0x240(%rbp)
1403b70a7: 0f 29 95 50 02 00 00        	movaps	%xmm2, 0x250(%rbp)
1403b70ae: 48 83 f8 03                 	cmpq	$0x3, %rax
1403b70b2: 75 1c                       	jne	0x1403b70d0 <.text+0x3b60d0>
1403b70b4: 48 c7 85 e8 00 00 00 02 00 00 00    	movq	$0x2, 0xe8(%rbp)
1403b70bf: eb 3d                       	jmp	0x1403b70fe <.text+0x3b60fe>
1403b70c1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1403b70d0: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1403b70d7: 0f 28 85 30 02 00 00        	movaps	0x230(%rbp), %xmm0
1403b70de: 0f 28 8d 40 02 00 00        	movaps	0x240(%rbp), %xmm1
1403b70e5: 0f 28 95 50 02 00 00        	movaps	0x250(%rbp), %xmm2
1403b70ec: 48 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %rax
1403b70f3: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1403b70f7: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1403b70fb: 0f 11 00                    	movups	%xmm0, (%rax)
1403b70fe: f3 0f 10 85 6c 02 00 00     	movss	0x26c(%rbp), %xmm0
1403b7106: c6 85 76 02 00 00 01        	movb	$0x1, 0x276(%rbp)
1403b710d: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1403b7113: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1403b711c: 48 c7 44 24 28 08 00 00 00  	movq	$0x8, 0x28(%rsp)
1403b7125: 41 b9 27 00 00 00           	movl	$0x27, %r9d
1403b712b: 4c 89 f9                    	movq	%r15, %rcx
1403b712e: 48 89 da                    	movq	%rbx, %rdx
1403b7131: 4c 8d 05 40 67 61 02        	leaq	0x2616740(%rip), %r8    # 0x1429cd878
1403b7138: e8 43 f7 ff ff              	callq	0x1403b6880 <.text+0x3b5880>
1403b713d: 48 83 7d c0 02              	cmpq	$0x2, -0x40(%rbp)
1403b7142: 75 2c                       	jne	0x1403b7170 <.text+0x3b6170>
1403b7144: 0f 10 07                    	movups	(%rdi), %xmm0
1403b7147: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
1403b714b: 0f 10 57 20                 	movups	0x20(%rdi), %xmm2
1403b714f: 0f 29 95 50 02 00 00        	movaps	%xmm2, 0x250(%rbp)
1403b7156: 0f 29 8d 40 02 00 00        	movaps	%xmm1, 0x240(%rbp)
1403b715d: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
1403b7164: b8 02 00 00 00              	movl	$0x2, %eax
1403b7169: eb 4d                       	jmp	0x1403b71b8 <.text+0x3b61b8>
1403b716b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1403b7170: f3 0f 10 4d f0              	movss	-0x10(%rbp), %xmm1
1403b7175: f3 41 0f 59 cb              	mulss	%xmm11, %xmm1
1403b717a: 4c 89 f9                    	movq	%r15, %rcx
1403b717d: e8 1e 04 6a 00              	callq	0x140a575a0 <.text+0xa565a0>
1403b7182: 4c 89 f9                    	movq	%r15, %rcx
1403b7185: e8 76 01 6a 00              	callq	0x140a57300 <.text+0xa56300>
1403b718a: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
1403b718e: 0f 10 07                    	movups	(%rdi), %xmm0
1403b7191: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
1403b7195: 0f 10 57 20                 	movups	0x20(%rdi), %xmm2
1403b7199: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
1403b71a0: 0f 29 8d 40 02 00 00        	movaps	%xmm1, 0x240(%rbp)
1403b71a7: 0f 29 95 50 02 00 00        	movaps	%xmm2, 0x250(%rbp)
1403b71ae: 48 83 f8 03                 	cmpq	$0x3, %rax
1403b71b2: 0f 84 82 fd ff ff           	je	0x1403b6f3a <.text+0x3b5f3a>
1403b71b8: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
1403b71bf: 0f 28 85 30 02 00 00        	movaps	0x230(%rbp), %xmm0
1403b71c6: 0f 28 8d 40 02 00 00        	movaps	0x240(%rbp), %xmm1
1403b71cd: 0f 28 95 50 02 00 00        	movaps	0x250(%rbp), %xmm2
1403b71d4: 48 8d 85 70 01 00 00        	leaq	0x170(%rbp), %rax
1403b71db: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1403b71df: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1403b71e3: 0f 11 00                    	movups	%xmm0, (%rax)
1403b71e6: c6 85 71 02 00 00 00        	movb	$0x0, 0x271(%rbp)
1403b71ed: 48 89 5d c0                 	movq	%rbx, -0x40(%rbp)
1403b71f1: 4c 89 65 c8                 	movq	%r12, -0x38(%rbp)
1403b71f5: 48 b8 00 00 00 00 06 00 00 00       	movabsq	$0x600000000, %rax # imm = 0x600000000
1403b71ff: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
1403b7203: 48 8d 85 71 02 00 00        	leaq	0x271(%rbp), %rax
1403b720a: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1403b720e: c6 85 73 02 00 00 01        	movb	$0x1, 0x273(%rbp)
1403b7215: 4c 89 e9                    	movq	%r13, %rcx
1403b7218: 4c 89 fa                    	movq	%r15, %rdx
1403b721b: 4c 8d 05 f6 d7 5e 02        	leaq	0x25ed7f6(%rip), %r8    # 0x1429a4a18
1403b7222: e8 49 93 f0 ff              	callq	0x1402c0570 <.text+0x2bf570>
1403b7227: 80 bd 71 02 00 00 00        	cmpb	$0x0, 0x271(%rbp)
1403b722e: 74 30                       	je	0x1403b7260 <.text+0x3b6260>
1403b7230: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403b723a: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1403b7241: c6 85 73 02 00 00 01        	movb	$0x1, 0x273(%rbp)
1403b7248: 4c 89 e9                    	movq	%r13, %rcx
1403b724b: e8 d0 28 25 00              	callq	0x140609b20 <.text+0x608b20>
1403b7250: eb 2a                       	jmp	0x1403b727c <.text+0x3b627c>
1403b7252: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1403b7260: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
1403b7267: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
1403b726e: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
1403b7275: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
1403b727c: f3 0f 10 85 6c 02 00 00     	movss	0x26c(%rbp), %xmm0
1403b7284: c6 85 72 02 00 00 01        	movb	$0x1, 0x272(%rbp)
1403b728b: 48 8d 05 0e 65 61 02        	leaq	0x261650e(%rip), %rax   # 0x1429cd7a0
1403b7292: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1403b7297: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1403b729d: 48 c7 44 24 30 01 00 00 00  	movq	$0x1, 0x30(%rsp)
1403b72a6: 41 b9 1a 00 00 00           	movl	$0x1a, %r9d
1403b72ac: 4c 89 e9                    	movq	%r13, %rcx
1403b72af: 48 89 da                    	movq	%rbx, %rdx
1403b72b2: 4c 8d 05 ff 64 61 02        	leaq	0x26164ff(%rip), %r8    # 0x1429cd7b8
1403b72b9: e8 c2 f5 ff ff              	callq	0x1403b6880 <.text+0x3b5880>
1403b72be: 83 bd 30 02 00 00 02        	cmpl	$0x2, 0x230(%rbp)
1403b72c5: 74 14                       	je	0x1403b72db <.text+0x3b62db>
1403b72c7: f3 0f 10 8d 60 02 00 00     	movss	0x260(%rbp), %xmm1
1403b72cf: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1403b72d3: 4c 89 e9                    	movq	%r13, %rcx
1403b72d6: e8 c5 02 6a 00              	callq	0x140a575a0 <.text+0xa565a0>
1403b72db: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
1403b72e2: 48 89 87 d0 00 00 00        	movq	%rax, 0xd0(%rdi)
1403b72e9: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
1403b72f0: 0f 10 8d 40 02 00 00        	movups	0x240(%rbp), %xmm1
1403b72f7: 0f 10 95 50 02 00 00        	movups	0x250(%rbp), %xmm2
1403b72fe: 0f 11 97 c0 00 00 00        	movups	%xmm2, 0xc0(%rdi)
1403b7305: 0f 11 8f b0 00 00 00        	movups	%xmm1, 0xb0(%rdi)
1403b730c: 0f 11 87 a0 00 00 00        	movups	%xmm0, 0xa0(%rdi)
1403b7313: 0f 28 85 20 01 00 00        	movaps	0x120(%rbp), %xmm0
1403b731a: 0f 28 8d 30 01 00 00        	movaps	0x130(%rbp), %xmm1
1403b7321: 0f 28 95 40 01 00 00        	movaps	0x140(%rbp), %xmm2
1403b7328: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1403b732c: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
1403b7330: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
1403b7334: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
1403b733b: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1403b733f: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1403b7346: 48 89 87 e8 00 00 00        	movq	%rax, 0xe8(%rdi)
1403b734d: 0f 28 85 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm0
1403b7354: 0f 11 87 d8 00 00 00        	movups	%xmm0, 0xd8(%rdi)
1403b735b: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
1403b7362: 48 89 87 00 01 00 00        	movq	%rax, 0x100(%rdi)
1403b7369: 0f 28 85 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm0
1403b7370: 0f 11 87 f0 00 00 00        	movups	%xmm0, 0xf0(%rdi)
1403b7377: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
1403b737e: 48 89 47 60                 	movq	%rax, 0x60(%rdi)
1403b7382: 0f 10 85 e8 00 00 00        	movups	0xe8(%rbp), %xmm0
1403b7389: 0f 10 8d f8 00 00 00        	movups	0xf8(%rbp), %xmm1
1403b7390: 0f 10 95 08 01 00 00        	movups	0x108(%rbp), %xmm2
1403b7397: 0f 11 57 50                 	movups	%xmm2, 0x50(%rdi)
1403b739b: 0f 11 4f 40                 	movups	%xmm1, 0x40(%rdi)
1403b739f: 0f 11 47 30                 	movups	%xmm0, 0x30(%rdi)
1403b73a3: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
1403b73aa: 48 89 87 98 00 00 00        	movq	%rax, 0x98(%rdi)
1403b73b1: 0f 10 85 68 01 00 00        	movups	0x168(%rbp), %xmm0
1403b73b8: 0f 10 8d 78 01 00 00        	movups	0x178(%rbp), %xmm1
1403b73bf: 0f 10 95 88 01 00 00        	movups	0x188(%rbp), %xmm2
1403b73c6: 0f 11 97 88 00 00 00        	movups	%xmm2, 0x88(%rdi)
1403b73cd: 0f 11 4f 78                 	movups	%xmm1, 0x78(%rdi)
1403b73d1: 0f 11 47 68                 	movups	%xmm0, 0x68(%rdi)
1403b73d5: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
1403b73dc: 48 89 87 18 01 00 00        	movq	%rax, 0x118(%rdi)
1403b73e3: 0f 28 85 a0 01 00 00        	movaps	0x1a0(%rbp), %xmm0
1403b73ea: 0f 11 87 08 01 00 00        	movups	%xmm0, 0x108(%rdi)
1403b73f1: 4c 89 b5 00 02 00 00        	movq	%r14, 0x200(%rbp)
1403b73f8: 49 8d 4e 30                 	leaq	0x30(%r14), %rcx
1403b73fc: 48 89 8d 08 02 00 00        	movq	%rcx, 0x208(%rbp)
1403b7403: e8 f8 ea 23 00              	callq	0x1405f5f00 <.text+0x5f4f00>
1403b7408: 4c 8b b5 00 02 00 00        	movq	0x200(%rbp), %r14
1403b740f: 4d 8d a6 b0 01 00 00        	leaq	0x1b0(%r14), %r12
1403b7416: 48 ff c6                    	incq	%rsi
1403b7419: 41 b8 28 01 00 00           	movl	$0x128, %r8d            # imm = 0x128
1403b741f: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
1403b7426: 4c 89 fa                    	movq	%r15, %rdx
1403b7429: e8 d2 c7 56 02              	callq	0x142923c00 <.text+0x2922c00>
1403b742e: 49 c7 86 58 01 00 00 00 00 00 00    	movq	$0x0, 0x158(%r14)
1403b7439: 41 c7 86 60 01 00 00 00 00 00 00    	movl	$0x0, 0x160(%r14)
1403b7444: 4d 89 e6                    	movq	%r12, %r14
1403b7447: 4c 8b a5 18 02 00 00        	movq	0x218(%rbp), %r12
1403b744e: 4c 3b b5 10 02 00 00        	cmpq	0x210(%rbp), %r14
1403b7455: 0f 85 75 f9 ff ff           	jne	0x1403b6dd0 <.text+0x3b5dd0>
1403b745b: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
1403b7462: 0f 10 00                    	movups	(%rax), %xmm0
1403b7465: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1403b7469: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
1403b7470: 0f 11 49 28                 	movups	%xmm1, 0x28(%rcx)
1403b7474: 0f 11 41 18                 	movups	%xmm0, 0x18(%rcx)
1403b7478: 48 8b 85 20 02 00 00        	movq	0x220(%rbp), %rax
1403b747f: 0f 10 00                    	movups	(%rax), %xmm0
1403b7482: 0f 11 01                    	movups	%xmm0, (%rcx)
1403b7485: 48 8b 40 10                 	movq	0x10(%rax), %rax
1403b7489: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
1403b748d: f3 0f 11 79 40              	movss	%xmm7, 0x40(%rcx)
1403b7492: 4c 89 61 38                 	movq	%r12, 0x38(%rcx)
1403b7496: 48 c7 41 44 00 00 00 00     	movq	$0x0, 0x44(%rcx)
1403b749e: 0f 28 b5 80 02 00 00        	movaps	0x280(%rbp), %xmm6
1403b74a5: 0f 28 bd 90 02 00 00        	movaps	0x290(%rbp), %xmm7
1403b74ac: 44 0f 28 85 a0 02 00 00     	movaps	0x2a0(%rbp), %xmm8
1403b74b4: 44 0f 28 8d b0 02 00 00     	movaps	0x2b0(%rbp), %xmm9
