
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140942c80: 55                          	pushq	%rbp
140942c81: 41 56                       	pushq	%r14
140942c83: 56                          	pushq	%rsi
140942c84: 57                          	pushq	%rdi
140942c85: 53                          	pushq	%rbx
140942c86: 48 81 ec 10 01 00 00        	subq	$0x110, %rsp            # imm = 0x110
140942c8d: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140942c95: 48 c7 85 88 00 00 00 fe ff ff ff    	movq	$-0x2, 0x88(%rbp)
140942ca0: 48 89 d7                    	movq	%rdx, %rdi
140942ca3: 48 89 ce                    	movq	%rcx, %rsi
140942ca6: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140942caa: e8 d1 8e ab 00              	callq	0x1413fbb80 <.text+0x13fab80>
140942caf: c6 85 87 00 00 00 01        	movb	$0x1, 0x87(%rbp)
140942cb6: 48 8b 0d b3 96 ce 02        	movq	0x2ce96b3(%rip), %rcx   # 0x14362c370
140942cbd: 4c 8d 05 9c d0 31 02        	leaq	0x231d09c(%rip), %r8    # 0x142c5fd60
140942cc4: 48 8d 95 87 00 00 00        	leaq	0x87(%rbp), %rdx
140942ccb: e8 d0 eb 1d 00              	callq	0x140b218a0 <.text+0xb208a0>
140942cd0: 49 89 d0                    	movq	%rdx, %r8
140942cd3: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140942cd7: 48 89 c2                    	movq	%rax, %rdx
140942cda: e8 21 03 b1 00              	callq	0x141453000 <.text+0x1452000>
140942cdf: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
140942ce3: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140942ce7: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140942ceb: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140942cef: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140942cf3: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140942cf7: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140942cfb: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
140942cff: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
140942d03: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
140942d07: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
140942d0b: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
140942d0f: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140942d13: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
140942d17: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140942d1b: 48 89 f1                    	movq	%rsi, %rcx
140942d1e: e8 5d 45 22 00              	callq	0x140b67280 <.text+0xb66280>
140942d23: 4c 8d 05 56 fb 16 02        	leaq	0x216fb56(%rip), %r8    # 0x142ab2880
140942d2a: 48 8d 5d 10                 	leaq	0x10(%rbp), %rbx
140942d2e: 48 89 d9                    	movq	%rbx, %rcx
140942d31: b2 02                       	movb	$0x2, %dl
140942d33: e8 98 aa 16 00              	callq	0x140aad7d0 <.text+0xaac7d0>
140942d38: 48 89 f1                    	movq	%rsi, %rcx
140942d3b: 48 89 da                    	movq	%rbx, %rdx
140942d3e: e8 3d 45 22 00              	callq	0x140b67280 <.text+0xb66280>
140942d43: 4c 8d 05 4e fb 16 02        	leaq	0x216fb4e(%rip), %r8    # 0x142ab2898
140942d4a: 48 89 d9                    	movq	%rbx, %rcx
140942d4d: b2 0a                       	movb	$0xa, %dl
140942d4f: e8 dc ca 17 00              	callq	0x140abf830 <.text+0xabe830>
140942d54: 48 89 f1                    	movq	%rsi, %rcx
140942d57: 48 89 da                    	movq	%rbx, %rdx
140942d5a: e8 21 45 22 00              	callq	0x140b67280 <.text+0xb66280>
140942d5f: 4c 8d 05 4a fb 16 02        	leaq	0x216fb4a(%rip), %r8    # 0x142ab28b0
140942d66: 48 89 d9                    	movq	%rbx, %rcx
140942d69: b2 03                       	movb	$0x3, %dl
140942d6b: e8 70 93 d4 00              	callq	0x14168c0e0 <.text+0x168b0e0>
140942d70: 48 89 f1                    	movq	%rsi, %rcx
140942d73: 48 89 da                    	movq	%rbx, %rdx
140942d76: e8 05 45 22 00              	callq	0x140b67280 <.text+0xb66280>
140942d7b: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140942d7f: e8 4c 3d 04 00              	callq	0x140986ad0 <.text+0x985ad0>
140942d84: c6 85 87 00 00 00 05        	movb	$0x5, 0x87(%rbp)
140942d8b: 48 8b 0d de 95 ce 02        	movq	0x2ce95de(%rip), %rcx   # 0x14362c370
140942d92: 4c 8d 05 c7 cf 31 02        	leaq	0x231cfc7(%rip), %r8    # 0x142c5fd60
140942d99: 48 8d 95 87 00 00 00        	leaq	0x87(%rbp), %rdx
140942da0: e8 fb ea 1d 00              	callq	0x140b218a0 <.text+0xb208a0>
140942da5: 49 89 d0                    	movq	%rdx, %r8
140942da8: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140942dac: 48 89 c2                    	movq	%rax, %rdx
140942daf: e8 4c 02 b1 00              	callq	0x141453000 <.text+0x1452000>
140942db4: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
140942db8: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140942dbc: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140942dc0: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140942dc4: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140942dc8: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140942dcc: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140942dd0: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
140942dd4: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
140942dd8: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
140942ddc: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
140942de0: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
140942de4: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140942de8: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
140942dec: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140942df0: 48 89 f1                    	movq	%rsi, %rcx
140942df3: e8 88 44 22 00              	callq	0x140b67280 <.text+0xb66280>
140942df8: 48 89 d9                    	movq	%rbx, %rcx
140942dfb: e8 a0 70 c1 00              	callq	0x141559ea0 <.text+0x1558ea0>
140942e00: 4c 8d 0d c1 fa 16 02        	leaq	0x216fac1(%rip), %r9    # 0x142ab28c8
140942e07: 4c 8d 75 a0                 	leaq	-0x60(%rbp), %r14
140942e0b: 4c 89 f1                    	movq	%r14, %rcx
140942e0e: 48 89 da                    	movq	%rbx, %rdx
140942e11: 45 31 c0                    	xorl	%r8d, %r8d
140942e14: e8 d7 fd b0 00              	callq	0x141452bf0 <.text+0x1451bf0>
140942e19: 48 89 f1                    	movq	%rsi, %rcx
140942e1c: 4c 89 f2                    	movq	%r14, %rdx
140942e1f: e8 5c 44 22 00              	callq	0x140b67280 <.text+0xb66280>
140942e24: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140942e28: e8 e3 24 cb 00              	callq	0x1415f5310 <.text+0x15f4310>
140942e2d: c6 85 87 00 00 00 2c        	movb	$0x2c, 0x87(%rbp)
140942e34: 48 8b 0d 35 95 ce 02        	movq	0x2ce9535(%rip), %rcx   # 0x14362c370
140942e3b: 4c 8d 05 1e cf 31 02        	leaq	0x231cf1e(%rip), %r8    # 0x142c5fd60
140942e42: 48 8d 95 87 00 00 00        	leaq	0x87(%rbp), %rdx
140942e49: e8 52 ea 1d 00              	callq	0x140b218a0 <.text+0xb208a0>
140942e4e: 49 89 d0                    	movq	%rdx, %r8
140942e51: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140942e55: 48 89 c2                    	movq	%rax, %rdx
140942e58: e8 a3 01 b1 00              	callq	0x141453000 <.text+0x1452000>
140942e5d: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
140942e61: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140942e65: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140942e69: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140942e6d: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140942e71: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140942e75: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140942e79: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
140942e7d: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
140942e81: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
140942e85: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
140942e89: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
140942e8d: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140942e91: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
140942e95: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140942e99: 48 89 f1                    	movq	%rsi, %rcx
140942e9c: e8 df 43 22 00              	callq	0x140b67280 <.text+0xb66280>
140942ea1: 80 bf e7 00 00 00 01        	cmpb	$0x1, 0xe7(%rdi)
140942ea8: 75 30                       	jne	0x140942eda <.text+0x941eda>
140942eaa: 48 8d 7d 10                 	leaq	0x10(%rbp), %rdi
140942eae: 48 89 f9                    	movq	%rdi, %rcx
140942eb1: e8 0a 91 ab 00              	callq	0x1413fbfc0 <.text+0x13fafc0>
140942eb6: 4c 8d 0d 23 fa 16 02        	leaq	0x216fa23(%rip), %r9    # 0x142ab28e0
140942ebd: 48 8d 5d a0                 	leaq	-0x60(%rbp), %rbx
140942ec1: 48 89 d9                    	movq	%rbx, %rcx
140942ec4: 48 89 fa                    	movq	%rdi, %rdx
140942ec7: 45 31 c0                    	xorl	%r8d, %r8d
140942eca: e8 f1 a5 d5 00              	callq	0x14169d4c0 <.text+0x169c4c0>
140942ecf: 48 89 f1                    	movq	%rsi, %rcx
140942ed2: 48 89 da                    	movq	%rbx, %rdx
140942ed5: e8 a6 43 22 00              	callq	0x140b67280 <.text+0xb66280>
140942eda: 48 89 f0                    	movq	%rsi, %rax
140942edd: 48 81 c4 10 01 00 00        	addq	$0x110, %rsp            # imm = 0x110
140942ee4: 5b                          	popq	%rbx
140942ee5: 5f                          	popq	%rdi
140942ee6: 5e                          	popq	%rsi
140942ee7: 41 5e                       	popq	%r14
140942ee9: 5d                          	popq	%rbp
140942eea: c3                          	retq
140942eeb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140942ef0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140942ef5: 55                          	pushq	%rbp
140942ef6: 41 56                       	pushq	%r14
140942ef8: 56                          	pushq	%rsi
140942ef9: 57                          	pushq	%rdi
140942efa: 53                          	pushq	%rbx
140942efb: 48 83 ec 20                 	subq	$0x20, %rsp
140942eff: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140942f06: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140942f0a: e8 71 69 6e ff              	callq	0x140029880 <.text+0x28880>
140942f0f: 90                          	nop
140942f10: 48 83 c4 20                 	addq	$0x20, %rsp
140942f14: 5b                          	popq	%rbx
140942f15: 5f                          	popq	%rdi
140942f16: 5e                          	popq	%rsi
140942f17: 41 5e                       	popq	%r14
140942f19: 5d                          	popq	%rbp
140942f1a: c3                          	retq
140942f1b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140942f20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140942f25: 55                          	pushq	%rbp
140942f26: 41 56                       	pushq	%r14
140942f28: 56                          	pushq	%rsi
140942f29: 57                          	pushq	%rdi
140942f2a: 53                          	pushq	%rbx
140942f2b: 48 83 ec 20                 	subq	$0x20, %rsp
140942f2f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140942f36: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140942f3a: e8 41 69 6e ff              	callq	0x140029880 <.text+0x28880>
140942f3f: 90                          	nop
140942f40: 48 83 c4 20                 	addq	$0x20, %rsp
140942f44: 5b                          	popq	%rbx
140942f45: 5f                          	popq	%rdi
140942f46: 5e                          	popq	%rsi
140942f47: 41 5e                       	popq	%r14
140942f49: 5d                          	popq	%rbp
140942f4a: c3                          	retq
140942f4b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140942f50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140942f55: 55                          	pushq	%rbp
140942f56: 41 56                       	pushq	%r14
140942f58: 56                          	pushq	%rsi
140942f59: 57                          	pushq	%rdi
140942f5a: 53                          	pushq	%rbx
140942f5b: 48 83 ec 20                 	subq	$0x20, %rsp
140942f5f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140942f66: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140942f6a: e8 11 69 6e ff              	callq	0x140029880 <.text+0x28880>
140942f6f: 90                          	nop
140942f70: 48 83 c4 20                 	addq	$0x20, %rsp
140942f74: 5b                          	popq	%rbx
140942f75: 5f                          	popq	%rdi
140942f76: 5e                          	popq	%rsi
140942f77: 41 5e                       	popq	%r14
140942f79: 5d                          	popq	%rbp
140942f7a: c3                          	retq
