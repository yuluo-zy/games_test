
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141da6cd0: 55                          	pushq	%rbp
141da6cd1: 56                          	pushq	%rsi
141da6cd2: 57                          	pushq	%rdi
141da6cd3: 48 81 ec d0 02 00 00        	subq	$0x2d0, %rsp            # imm = 0x2D0
141da6cda: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141da6ce2: 48 c7 85 48 02 00 00 fe ff ff ff    	movq	$-0x2, 0x248(%rbp)
141da6ced: 48 89 ce                    	movq	%rcx, %rsi
141da6cf0: 0f 57 c0                    	xorps	%xmm0, %xmm0
141da6cf3: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
141da6cf7: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
141da6cfb: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
141da6cff: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
141da6d03: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
141da6d07: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
141da6d0e: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
141da6d12: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
141da6d16: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
141da6d1d: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
141da6d24: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
141da6d2b: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
141da6d35: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
141da6d3d: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
141da6d45: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
141da6d4d: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
141da6d55: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
141da6d5d: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
141da6d65: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
141da6d6d: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
141da6d75: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141da6d7f: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
141da6d83: 48 8d 0d e6 61 1c 01        	leaq	0x11c61e6(%rip), %rcx   # 0x142f6cf70
141da6d8a: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
141da6d8e: 48 c7 45 48 2b 00 00 00     	movq	$0x2b, 0x48(%rbp)
141da6d96: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
141da6d9e: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
141da6da6: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
141da6db1: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
141da6dbc: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
141da6dc7: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
141da6dd1: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
141da6dda: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
141da6de1: c7 85 40 02 00 00 00 00 00 00       	movl	$0x0, 0x240(%rbp)
141da6deb: 0f b6 05 17 de 88 01        	movzbl	0x188de17(%rip), %eax   # 0x143634c09
141da6df2: b9 a8 02 00 00              	movl	$0x2a8, %ecx            # imm = 0x2A8
141da6df7: ba 08 00 00 00              	movl	$0x8, %edx
141da6dfc: e8 0f ce 86 fe              	callq	0x140613c10 <.text+0x612c10>
141da6e01: 48 85 c0                    	testq	%rax, %rax
141da6e04: 74 35                       	je	0x141da6e3b <.text+0x1da5e3b>
141da6e06: 48 89 c7                    	movq	%rax, %rdi
141da6e09: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
141da6e0d: 41 b8 a8 02 00 00           	movl	$0x2a8, %r8d            # imm = 0x2A8
141da6e13: 48 89 c1                    	movq	%rax, %rcx
141da6e16: e8 e5 cd b7 00              	callq	0x142923c00 <.text+0x2922c00>
141da6e1b: 4c 8d 05 96 60 1c 01        	leaq	0x11c6096(%rip), %r8    # 0x142f6ceb8
141da6e22: 48 89 f1                    	movq	%rsi, %rcx
141da6e25: 48 89 fa                    	movq	%rdi, %rdx
141da6e28: e8 e3 dc f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141da6e2d: 48 89 f0                    	movq	%rsi, %rax
141da6e30: 48 81 c4 d0 02 00 00        	addq	$0x2d0, %rsp            # imm = 0x2D0
141da6e37: 5f                          	popq	%rdi
141da6e38: 5e                          	popq	%rsi
141da6e39: 5d                          	popq	%rbp
141da6e3a: c3                          	retq
141da6e3b: b9 08 00 00 00              	movl	$0x8, %ecx
141da6e40: ba a8 02 00 00              	movl	$0x2a8, %edx            # imm = 0x2A8
141da6e45: e8 b6 21 b3 00              	callq	0x1428d9000 <.text+0x28d8000>
141da6e4a: 0f 0b                       	ud2
141da6e4c: 0f 1f 40 00                 	nopl	(%rax)
141da6e50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141da6e55: 55                          	pushq	%rbp
141da6e56: 56                          	pushq	%rsi
141da6e57: 57                          	pushq	%rdi
141da6e58: 48 83 ec 20                 	subq	$0x20, %rsp
141da6e5c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141da6e63: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
141da6e67: e8 44 85 82 fe              	callq	0x1405cf3b0 <.text+0x5ce3b0>
141da6e6c: 90                          	nop
141da6e6d: 48 83 c4 20                 	addq	$0x20, %rsp
141da6e71: 5f                          	popq	%rdi
141da6e72: 5e                          	popq	%rsi
141da6e73: 5d                          	popq	%rbp
141da6e74: c3                          	retq
