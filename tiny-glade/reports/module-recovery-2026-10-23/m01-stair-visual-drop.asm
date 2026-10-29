
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140893be0: 55                          	pushq	%rbp
140893be1: 56                          	pushq	%rsi
140893be2: 48 83 ec 38                 	subq	$0x38, %rsp
140893be6: 48 8d 6c 24 30              	leaq	0x30(%rsp), %rbp
140893beb: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
140893bf3: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
140893bf7: 48 83 b9 d8 00 00 00 00     	cmpq	$0x0, 0xd8(%rcx)
140893bff: 74 10                       	je	0x140893c11 <.text+0x892c11>
140893c01: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893c05: 48 8d 88 d8 00 00 00        	leaq	0xd8(%rax), %rcx
140893c0c: e8 7f b7 fe ff              	callq	0x14087f390 <.text+0x87e390>
140893c11: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893c15: 48 83 b8 e8 00 00 00 00     	cmpq	$0x0, 0xe8(%rax)
140893c1d: 74 10                       	je	0x140893c2f <.text+0x892c2f>
140893c1f: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893c23: 48 8d 88 e8 00 00 00        	leaq	0xe8(%rax), %rcx
140893c2a: e8 61 b7 fe ff              	callq	0x14087f390 <.text+0x87e390>
140893c2f: 48 8b 75 f8                 	movq	-0x8(%rbp), %rsi
140893c33: 48 8b 16                    	movq	(%rsi), %rdx
140893c36: 48 85 d2                    	testq	%rdx, %rdx
140893c39: 74 13                       	je	0x140893c4e <.text+0x892c4e>
140893c3b: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
140893c3f: 48 c1 e2 03                 	shlq	$0x3, %rdx
140893c43: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140893c49: e8 d2 ff d7 ff              	callq	0x140613c20 <.text+0x612c20>
140893c4e: 48 8b 96 b0 00 00 00        	movq	0xb0(%rsi), %rdx
140893c55: 48 85 d2                    	testq	%rdx, %rdx
140893c58: 74 2a                       	je	0x140893c84 <.text+0x892c84>
140893c5a: 48 8d 04 55 11 00 00 00     	leaq	0x11(,%rdx,2), %rax
140893c62: 48 83 e0 f0                 	andq	$-0x10, %rax
140893c66: 48 01 c2                    	addq	%rax, %rdx
140893c69: 48 83 c2 11                 	addq	$0x11, %rdx
140893c6d: 74 15                       	je	0x140893c84 <.text+0x892c84>
140893c6f: 48 8b 8e a8 00 00 00        	movq	0xa8(%rsi), %rcx
140893c76: 48 29 c1                    	subq	%rax, %rcx
140893c79: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140893c7f: e8 9c ff d7 ff              	callq	0x140613c20 <.text+0x612c20>
140893c84: 48 8b 56 18                 	movq	0x18(%rsi), %rdx
140893c88: 48 85 d2                    	testq	%rdx, %rdx
140893c8b: 74 13                       	je	0x140893ca0 <.text+0x892ca0>
140893c8d: 48 8b 4e 20                 	movq	0x20(%rsi), %rcx
140893c91: 48 c1 e2 04                 	shlq	$0x4, %rdx
140893c95: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140893c9b: e8 80 ff d7 ff              	callq	0x140613c20 <.text+0x612c20>
140893ca0: 48 8b 46 30                 	movq	0x30(%rsi), %rax
140893ca4: 48 85 c0                    	testq	%rax, %rax
140893ca7: 74 17                       	je	0x140893cc0 <.text+0x892cc0>
140893ca9: 48 8b 4e 38                 	movq	0x38(%rsi), %rcx
140893cad: 48 c1 e0 03                 	shlq	$0x3, %rax
140893cb1: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140893cb5: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140893cbb: e8 60 ff d7 ff              	callq	0x140613c20 <.text+0x612c20>
140893cc0: 48 83 7e 68 00              	cmpq	$0x0, 0x68(%rsi)
140893cc5: 74 0d                       	je	0x140893cd4 <.text+0x892cd4>
140893cc7: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893ccb: 48 8d 48 68                 	leaq	0x68(%rax), %rcx
140893ccf: e8 bc b6 fe ff              	callq	0x14087f390 <.text+0x87e390>
140893cd4: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893cd8: 48 83 78 78 00              	cmpq	$0x0, 0x78(%rax)
140893cdd: 74 0d                       	je	0x140893cec <.text+0x892cec>
140893cdf: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893ce3: 48 8d 48 78                 	leaq	0x78(%rax), %rcx
140893ce7: e8 a4 b6 fe ff              	callq	0x14087f390 <.text+0x87e390>
140893cec: 48 8b 75 f8                 	movq	-0x8(%rbp), %rsi
140893cf0: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
140893cf4: 48 85 d2                    	testq	%rdx, %rdx
140893cf7: 74 13                       	je	0x140893d0c <.text+0x892d0c>
140893cf9: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
140893cfd: 48 c1 e2 06                 	shlq	$0x6, %rdx
140893d01: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140893d07: e8 14 ff d7 ff              	callq	0x140613c20 <.text+0x612c20>
140893d0c: 48 8b 86 90 00 00 00        	movq	0x90(%rsi), %rax
140893d13: 48 85 c0                    	testq	%rax, %rax
140893d16: 74 1c                       	je	0x140893d34 <.text+0x892d34>
140893d18: 48 8b 8e 98 00 00 00        	movq	0x98(%rsi), %rcx
140893d1f: 48 6b d0 2c                 	imulq	$0x2c, %rax, %rdx
140893d23: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140893d29: 48 83 c4 38                 	addq	$0x38, %rsp
140893d2d: 5e                          	popq	%rsi
140893d2e: 5d                          	popq	%rbp
140893d2f: e9 ec fe d7 ff              	jmp	0x140613c20 <.text+0x612c20>
140893d34: 90                          	nop
140893d35: 48 83 c4 38                 	addq	$0x38, %rsp
140893d39: 5e                          	popq	%rsi
140893d3a: 5d                          	popq	%rbp
140893d3b: c3                          	retq
140893d3c: 0f 1f 40 00                 	nopl	(%rax)
140893d40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140893d45: 55                          	pushq	%rbp
140893d46: 56                          	pushq	%rsi
140893d47: 48 83 ec 28                 	subq	$0x28, %rsp
140893d4b: 48 8d 6a 30                 	leaq	0x30(%rdx), %rbp
140893d4f: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893d53: 48 83 b8 e8 00 00 00 00     	cmpq	$0x0, 0xe8(%rax)
140893d5b: 74 0c                       	je	0x140893d69 <.text+0x892d69>
140893d5d: 48 8d 88 e8 00 00 00        	leaq	0xe8(%rax), %rcx
140893d64: e8 27 b6 fe ff              	callq	0x14087f390 <.text+0x87e390>
140893d69: 90                          	nop
140893d6a: 48 83 c4 28                 	addq	$0x28, %rsp
140893d6e: 5e                          	popq	%rsi
140893d6f: 5d                          	popq	%rbp
140893d70: c3                          	retq
140893d71: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140893d80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140893d85: 55                          	pushq	%rbp
140893d86: 56                          	pushq	%rsi
140893d87: 48 83 ec 28                 	subq	$0x28, %rsp
140893d8b: 48 8d 6a 30                 	leaq	0x30(%rdx), %rbp
140893d8f: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893d93: 48 8b 10                    	movq	(%rax), %rdx
140893d96: 48 85 d2                    	testq	%rdx, %rdx
140893d99: 74 17                       	je	0x140893db2 <.text+0x892db2>
140893d9b: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893d9f: 48 8b 48 08                 	movq	0x8(%rax), %rcx
140893da3: 48 c1 e2 03                 	shlq	$0x3, %rdx
140893da7: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140893dad: e8 6e fe d7 ff              	callq	0x140613c20 <.text+0x612c20>
140893db2: 48 8b 75 f8                 	movq	-0x8(%rbp), %rsi
140893db6: 48 8d 8e a8 00 00 00        	leaq	0xa8(%rsi), %rcx
140893dbd: 48 8d 96 c8 00 00 00        	leaq	0xc8(%rsi), %rdx
140893dc4: 41 b8 02 00 00 00           	movl	$0x2, %r8d
140893dca: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140893dd0: e8 2b 86 7b ff              	callq	0x14004c400 <.text+0x4b400>
140893dd5: 48 8b 56 18                 	movq	0x18(%rsi), %rdx
140893dd9: 48 85 d2                    	testq	%rdx, %rdx
140893ddc: 74 17                       	je	0x140893df5 <.text+0x892df5>
140893dde: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893de2: 48 8b 48 20                 	movq	0x20(%rax), %rcx
140893de6: 48 c1 e2 04                 	shlq	$0x4, %rdx
140893dea: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140893df0: e8 2b fe d7 ff              	callq	0x140613c20 <.text+0x612c20>
140893df5: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893df9: 48 8b 40 30                 	movq	0x30(%rax), %rax
140893dfd: 48 85 c0                    	testq	%rax, %rax
140893e00: 74 1b                       	je	0x140893e1d <.text+0x892e1d>
140893e02: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140893e06: 48 8b 49 38                 	movq	0x38(%rcx), %rcx
140893e0a: 48 c1 e0 03                 	shlq	$0x3, %rax
140893e0e: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140893e12: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140893e18: e8 03 fe d7 ff              	callq	0x140613c20 <.text+0x612c20>
140893e1d: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893e21: 48 8d 48 48                 	leaq	0x48(%rax), %rcx
140893e25: e8 f6 ba ff ff              	callq	0x14088f920 <.text+0x88e920>
140893e2a: 90                          	nop
140893e2b: 48 83 c4 28                 	addq	$0x28, %rsp
140893e2f: 5e                          	popq	%rsi
140893e30: 5d                          	popq	%rbp
140893e31: c3                          	retq
140893e32: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140893e40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140893e45: 55                          	pushq	%rbp
140893e46: 56                          	pushq	%rsi
140893e47: 48 83 ec 28                 	subq	$0x28, %rsp
140893e4b: 48 8d 6a 30                 	leaq	0x30(%rdx), %rbp
140893e4f: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893e53: 48 83 78 78 00              	cmpq	$0x0, 0x78(%rax)
140893e58: 74 09                       	je	0x140893e63 <.text+0x892e63>
140893e5a: 48 8d 48 78                 	leaq	0x78(%rax), %rcx
140893e5e: e8 2d b5 fe ff              	callq	0x14087f390 <.text+0x87e390>
140893e63: 90                          	nop
140893e64: 48 83 c4 28                 	addq	$0x28, %rsp
140893e68: 5e                          	popq	%rsi
140893e69: 5d                          	popq	%rbp
140893e6a: c3                          	retq
140893e6b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140893e70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140893e75: 55                          	pushq	%rbp
140893e76: 56                          	pushq	%rsi
140893e77: 48 83 ec 28                 	subq	$0x28, %rsp
140893e7b: 48 8d 6a 30                 	leaq	0x30(%rdx), %rbp
140893e7f: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893e83: 48 8b 50 48                 	movq	0x48(%rax), %rdx
140893e87: 48 85 d2                    	testq	%rdx, %rdx
140893e8a: 74 17                       	je	0x140893ea3 <.text+0x892ea3>
140893e8c: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893e90: 48 8b 48 50                 	movq	0x50(%rax), %rcx
140893e94: 48 c1 e2 06                 	shlq	$0x6, %rdx
140893e98: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140893e9e: e8 7d fd d7 ff              	callq	0x140613c20 <.text+0x612c20>
140893ea3: 90                          	nop
140893ea4: 48 83 c4 28                 	addq	$0x28, %rsp
140893ea8: 5e                          	popq	%rsi
140893ea9: 5d                          	popq	%rbp
140893eaa: c3                          	retq
140893eab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140893eb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140893eb5: 55                          	pushq	%rbp
140893eb6: 56                          	pushq	%rsi
140893eb7: 48 83 ec 28                 	subq	$0x28, %rsp
140893ebb: 48 8d 6a 30                 	leaq	0x30(%rdx), %rbp
140893ebf: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140893ec3: 48 8b 80 90 00 00 00        	movq	0x90(%rax), %rax
140893eca: 48 85 c0                    	testq	%rax, %rax
140893ecd: 74 1a                       	je	0x140893ee9 <.text+0x892ee9>
140893ecf: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140893ed3: 48 8b 89 98 00 00 00        	movq	0x98(%rcx), %rcx
140893eda: 48 6b d0 2c                 	imulq	$0x2c, %rax, %rdx
140893ede: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140893ee4: e8 37 fd d7 ff              	callq	0x140613c20 <.text+0x612c20>
140893ee9: 90                          	nop
140893eea: 48 83 c4 28                 	addq	$0x28, %rsp
140893eee: 5e                          	popq	%rsi
140893eef: 5d                          	popq	%rbp
140893ef0: c3                          	retq
