
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1402b5c00: 55                          	pushq	%rbp
1402b5c01: 56                          	pushq	%rsi
1402b5c02: 57                          	pushq	%rdi
1402b5c03: 53                          	pushq	%rbx
1402b5c04: 48 83 ec 68                 	subq	$0x68, %rsp
1402b5c08: 48 8d 6c 24 60              	leaq	0x60(%rsp), %rbp
1402b5c0d: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
1402b5c15: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1402b5c19: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1402b5c1d: 48 85 c0                    	testq	%rax, %rax
1402b5c20: 0f 84 34 01 00 00           	je	0x1402b5d5a <.text+0x2b4d5a>
1402b5c26: 48 c7 41 10 00 00 00 00     	movq	$0x0, 0x10(%rcx)
1402b5c2e: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
1402b5c32: 48 8b 71 08                 	movq	0x8(%rcx), %rsi
1402b5c36: ba 01 00 00 00              	movl	$0x1, %edx
1402b5c3b: 83 7e 50 02                 	cmpl	$0x2, 0x50(%rsi)
1402b5c3f: 48 89 f1                    	movq	%rsi, %rcx
1402b5c42: 74 2c                       	je	0x1402b5c70 <.text+0x2b4c70>
1402b5c44: 31 db                       	xorl	%ebx, %ebx
1402b5c46: 48 89 f0                    	movq	%rsi, %rax
1402b5c49: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1402b5c50: 48 39 55 e8                 	cmpq	%rdx, -0x18(%rbp)
1402b5c54: 0f 84 f1 00 00 00           	je	0x1402b5d4b <.text+0x2b4d4b>
1402b5c5a: 48 ff c2                    	incq	%rdx
1402b5c5d: 48 8d 88 98 00 00 00        	leaq	0x98(%rax), %rcx
1402b5c64: 83 b8 e8 00 00 00 02        	cmpl	$0x2, 0xe8(%rax)
1402b5c6b: 48 89 c8                    	movq	%rcx, %rax
1402b5c6e: 75 e0                       	jne	0x1402b5c50 <.text+0x2b4c50>
1402b5c70: 48 89 55 f0                 	movq	%rdx, -0x10(%rbp)
1402b5c74: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
1402b5c78: e8 63 1c 34 00              	callq	0x1405f78e0 <.text+0x5f68e0>
1402b5c7d: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1402b5c81: 0f b6 81 96 00 00 00        	movzbl	0x96(%rcx), %eax
1402b5c88: 04 fb                       	addb	$-0x5, %al
1402b5c8a: 3c 02                       	cmpb	$0x2, %al
1402b5c8c: 48 8b 7d f0                 	movq	-0x10(%rbp), %rdi
1402b5c90: 72 1d                       	jb	0x1402b5caf <.text+0x2b4caf>
1402b5c92: 48 83 c1 70                 	addq	$0x70, %rcx
1402b5c96: 48 c7 45 e0 01 00 00 00     	movq	$0x1, -0x20(%rbp)
1402b5c9e: 48 89 7d d8                 	movq	%rdi, -0x28(%rbp)
1402b5ca2: e8 09 c8 92 00              	callq	0x140be24b0 <.text+0xbe14b0>
1402b5ca7: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
1402b5cab: 48 8b 70 08                 	movq	0x8(%rax), %rsi
1402b5caf: bb 01 00 00 00              	movl	$0x1, %ebx
1402b5cb4: eb 28                       	jmp	0x1402b5cde <.text+0x2b4cde>
1402b5cb6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1402b5cc0: 48 89 f8                    	movq	%rdi, %rax
1402b5cc3: 48 29 d8                    	subq	%rbx, %rax
1402b5cc6: 48 69 c8 98 00 00 00        	imulq	$0x98, %rax, %rcx
1402b5ccd: 48 01 f1                    	addq	%rsi, %rcx
1402b5cd0: 41 b8 98 00 00 00           	movl	$0x98, %r8d
1402b5cd6: e8 25 df 66 02              	callq	0x142923c00 <.text+0x2922c00>
1402b5cdb: 48 ff c7                    	incq	%rdi
1402b5cde: 48 3b 7d e8                 	cmpq	-0x18(%rbp), %rdi
1402b5ce2: 74 67                       	je	0x1402b5d4b <.text+0x2b4d4b>
1402b5ce4: 48 69 c7 98 00 00 00        	imulq	$0x98, %rdi, %rax
1402b5ceb: 48 8d 14 06                 	leaq	(%rsi,%rax), %rdx
1402b5cef: 83 7c 06 50 02              	cmpl	$0x2, 0x50(%rsi,%rax)
1402b5cf4: 75 ca                       	jne	0x1402b5cc0 <.text+0x2b4cc0>
1402b5cf6: 48 ff c7                    	incq	%rdi
1402b5cf9: 48 89 7d f0                 	movq	%rdi, -0x10(%rbp)
1402b5cfd: 48 ff c3                    	incq	%rbx
1402b5d00: 48 89 5d f8                 	movq	%rbx, -0x8(%rbp)
1402b5d04: 48 89 55 c8                 	movq	%rdx, -0x38(%rbp)
1402b5d08: 48 89 d1                    	movq	%rdx, %rcx
1402b5d0b: e8 d0 1b 34 00              	callq	0x1405f78e0 <.text+0x5f68e0>
1402b5d10: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
1402b5d14: 0f b6 81 96 00 00 00        	movzbl	0x96(%rcx), %eax
1402b5d1b: 04 fb                       	addb	$-0x5, %al
1402b5d1d: 3c 02                       	cmpb	$0x2, %al
1402b5d1f: 73 0f                       	jae	0x1402b5d30 <.text+0x2b4d30>
1402b5d21: 48 8b 7d f0                 	movq	-0x10(%rbp), %rdi
1402b5d25: 48 8b 5d f8                 	movq	-0x8(%rbp), %rbx
1402b5d29: eb b3                       	jmp	0x1402b5cde <.text+0x2b4cde>
1402b5d2b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1402b5d30: 48 83 c1 70                 	addq	$0x70, %rcx
1402b5d34: 48 8b 5d f8                 	movq	-0x8(%rbp), %rbx
1402b5d38: 48 89 5d e0                 	movq	%rbx, -0x20(%rbp)
1402b5d3c: 48 8b 7d f0                 	movq	-0x10(%rbp), %rdi
1402b5d40: 48 89 7d d8                 	movq	%rdi, -0x28(%rbp)
1402b5d44: e8 67 c7 92 00              	callq	0x140be24b0 <.text+0xbe14b0>
1402b5d49: eb 93                       	jmp	0x1402b5cde <.text+0x2b4cde>
1402b5d4b: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
1402b5d4f: 48 29 d9                    	subq	%rbx, %rcx
1402b5d52: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
1402b5d56: 48 89 48 10                 	movq	%rcx, 0x10(%rax)
1402b5d5a: 48 83 c4 68                 	addq	$0x68, %rsp
1402b5d5e: 5b                          	popq	%rbx
1402b5d5f: 5f                          	popq	%rdi
1402b5d60: 5e                          	popq	%rsi
1402b5d61: 5d                          	popq	%rbp
1402b5d62: c3                          	retq
1402b5d63: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1402b5d70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1402b5d75: 55                          	pushq	%rbp
1402b5d76: 56                          	pushq	%rsi
1402b5d77: 57                          	pushq	%rdi
1402b5d78: 53                          	pushq	%rbx
1402b5d79: 48 83 ec 28                 	subq	$0x28, %rsp
1402b5d7d: 48 8d 6a 60                 	leaq	0x60(%rdx), %rbp
1402b5d81: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
1402b5d85: 0f b6 80 96 00 00 00        	movzbl	0x96(%rax), %eax
1402b5d8c: 04 fb                       	addb	$-0x5, %al
1402b5d8e: 3c 02                       	cmpb	$0x2, %al
1402b5d90: 72 1d                       	jb	0x1402b5daf <.text+0x2b4daf>
1402b5d92: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1402b5d96: 48 83 c1 70                 	addq	$0x70, %rcx
1402b5d9a: 48 c7 45 e0 01 00 00 00     	movq	$0x1, -0x20(%rbp)
1402b5da2: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1402b5da6: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1402b5daa: e8 01 c7 92 00              	callq	0x140be24b0 <.text+0xbe14b0>
1402b5daf: 48 c7 45 e0 01 00 00 00     	movq	$0x1, -0x20(%rbp)
1402b5db7: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1402b5dbb: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1402b5dbf: 48 83 c4 28                 	addq	$0x28, %rsp
1402b5dc3: 5b                          	popq	%rbx
1402b5dc4: 5f                          	popq	%rdi
1402b5dc5: 5e                          	popq	%rsi
1402b5dc6: 5d                          	popq	%rbp
1402b5dc7: c3                          	retq
1402b5dc8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1402b5dd0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1402b5dd5: 55                          	pushq	%rbp
1402b5dd6: 56                          	pushq	%rsi
1402b5dd7: 57                          	pushq	%rdi
1402b5dd8: 53                          	pushq	%rbx
1402b5dd9: 48 83 ec 28                 	subq	$0x28, %rsp
1402b5ddd: 48 8d 6a 60                 	leaq	0x60(%rdx), %rbp
1402b5de1: 48 8b 75 e0                 	movq	-0x20(%rbp), %rsi
1402b5de5: 48 85 f6                    	testq	%rsi, %rsi
1402b5de8: 74 36                       	je	0x1402b5e20 <.text+0x2b4e20>
1402b5dea: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
1402b5dee: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
1402b5df2: 4c 8b 41 08                 	movq	0x8(%rcx), %r8
1402b5df6: 48 69 d0 98 00 00 00        	imulq	$0x98, %rax, %rdx
1402b5dfd: 4c 01 c2                    	addq	%r8, %rdx
1402b5e00: 4c 8b 4d e8                 	movq	-0x18(%rbp), %r9
1402b5e04: 49 29 c1                    	subq	%rax, %r9
1402b5e07: 48 29 f0                    	subq	%rsi, %rax
1402b5e0a: 48 69 c8 98 00 00 00        	imulq	$0x98, %rax, %rcx
1402b5e11: 4c 01 c1                    	addq	%r8, %rcx
1402b5e14: 4d 69 c1 98 00 00 00        	imulq	$0x98, %r9, %r8
1402b5e1b: e8 f0 dd 66 02              	callq	0x142923c10 <.text+0x2922c10>
1402b5e20: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
1402b5e24: 48 29 f1                    	subq	%rsi, %rcx
1402b5e27: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
1402b5e2b: 48 89 48 10                 	movq	%rcx, 0x10(%rax)
1402b5e2f: 48 83 c4 28                 	addq	$0x28, %rsp
1402b5e33: 5b                          	popq	%rbx
1402b5e34: 5f                          	popq	%rdi
1402b5e35: 5e                          	popq	%rsi
1402b5e36: 5d                          	popq	%rbp
1402b5e37: c3                          	retq
1402b5e38: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1402b5e40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1402b5e45: 55                          	pushq	%rbp
1402b5e46: 56                          	pushq	%rsi
1402b5e47: 57                          	pushq	%rdi
1402b5e48: 53                          	pushq	%rbx
1402b5e49: 48 83 ec 28                 	subq	$0x28, %rsp
1402b5e4d: 48 8d 6a 60                 	leaq	0x60(%rdx), %rbp
1402b5e51: 48 8b 45 c8                 	movq	-0x38(%rbp), %rax
1402b5e55: 0f b6 80 96 00 00 00        	movzbl	0x96(%rax), %eax
1402b5e5c: 04 fb                       	addb	$-0x5, %al
1402b5e5e: 3c 02                       	cmpb	$0x2, %al
1402b5e60: 72 1d                       	jb	0x1402b5e7f <.text+0x2b4e7f>
1402b5e62: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
1402b5e66: 48 83 c1 70                 	addq	$0x70, %rcx
1402b5e6a: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
1402b5e6e: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
1402b5e72: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1402b5e76: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1402b5e7a: e8 31 c6 92 00              	callq	0x140be24b0 <.text+0xbe14b0>
1402b5e7f: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
1402b5e83: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
1402b5e87: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1402b5e8b: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1402b5e8f: 48 83 c4 28                 	addq	$0x28, %rsp
1402b5e93: 5b                          	popq	%rbx
1402b5e94: 5f                          	popq	%rdi
1402b5e95: 5e                          	popq	%rsi
1402b5e96: 5d                          	popq	%rbp
1402b5e97: c3                          	retq
