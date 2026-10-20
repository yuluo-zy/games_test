
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141304bd0: 55                          	pushq	%rbp
141304bd1: 41 57                       	pushq	%r15
141304bd3: 41 56                       	pushq	%r14
141304bd5: 41 55                       	pushq	%r13
141304bd7: 41 54                       	pushq	%r12
141304bd9: 56                          	pushq	%rsi
141304bda: 57                          	pushq	%rdi
141304bdb: 53                          	pushq	%rbx
141304bdc: 48 81 ec 68 01 00 00        	subq	$0x168, %rsp            # imm = 0x168
141304be3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141304beb: 48 c7 85 e0 00 00 00 fe ff ff ff    	movq	$-0x2, 0xe0(%rbp)
141304bf6: 4c 89 c3                    	movq	%r8, %rbx
141304bf9: 48 89 d7                    	movq	%rdx, %rdi
141304bfc: 48 89 ce                    	movq	%rcx, %rsi
141304bff: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
141304c06: e8 e5 b7 fe ff              	callq	0x1412f03f0 <.text+0x12ef3f0>
141304c0b: 83 bd a0 00 00 00 02        	cmpl	$0x2, 0xa0(%rbp)
141304c12: 75 1c                       	jne	0x141304c30 <.text+0x1303c30>
141304c14: 48 c7 06 00 00 00 00        	movq	$0x0, (%rsi)
141304c1b: 48 c7 46 08 04 00 00 00     	movq	$0x4, 0x8(%rsi)
141304c23: 48 c7 46 10 00 00 00 00     	movq	$0x0, 0x10(%rsi)
141304c2b: e9 c1 02 00 00              	jmp	0x141304ef1 <.text+0x1303ef1>
141304c30: 48 8b 47 20                 	movq	0x20(%rdi), %rax
141304c34: 4c 8b 47 28                 	movq	0x28(%rdi), %r8
141304c38: 80 bf c0 00 00 00 00        	cmpb	$0x0, 0xc0(%rdi)
141304c3f: 74 3f                       	je	0x141304c80 <.text+0x1303c80>
141304c41: 4c 39 c0                    	cmpq	%r8, %rax
141304c44: 74 7e                       	je	0x141304cc4 <.text+0x1303cc4>
141304c46: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
141304c4a: 4c 8b 51 08                 	movq	0x8(%rcx), %r10
141304c4e: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
141304c52: 49 29 c0                    	subq	%rax, %r8
141304c55: 49 c1 e8 02                 	shrq	$0x2, %r8
141304c59: 45 31 c9                    	xorl	%r9d, %r9d
141304c5c: 45 31 db                    	xorl	%r11d, %r11d
141304c5f: 90                          	nop
141304c60: 42 8b 0c 98                 	movl	(%rax,%r11,4), %ecx
141304c64: 48 39 ca                    	cmpq	%rcx, %rdx
141304c67: 0f 86 b7 02 00 00           	jbe	0x141304f24 <.text+0x1303f24>
141304c6d: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
141304c71: 4d 03 4c ca 10              	addq	0x10(%r10,%rcx,8), %r9
141304c76: 49 ff c3                    	incq	%r11
141304c79: 4d 39 d8                    	cmpq	%r11, %r8
141304c7c: 75 e2                       	jne	0x141304c60 <.text+0x1303c60>
141304c7e: eb 47                       	jmp	0x141304cc7 <.text+0x1303cc7>
141304c80: 4c 39 c0                    	cmpq	%r8, %rax
141304c83: 74 3f                       	je	0x141304cc4 <.text+0x1303cc4>
141304c85: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
141304c89: 4c 8b 51 08                 	movq	0x8(%rcx), %r10
141304c8d: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
141304c91: 49 29 c0                    	subq	%rax, %r8
141304c94: 49 c1 e8 02                 	shrq	$0x2, %r8
141304c98: 45 31 c9                    	xorl	%r9d, %r9d
141304c9b: 45 31 db                    	xorl	%r11d, %r11d
141304c9e: 66 90                       	nop
141304ca0: 42 8b 0c 98                 	movl	(%rax,%r11,4), %ecx
141304ca4: 48 39 ca                    	cmpq	%rcx, %rdx
141304ca7: 0f 86 83 02 00 00           	jbe	0x141304f30 <.text+0x1303f30>
141304cad: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
141304cb1: 48 c1 e1 05                 	shlq	$0x5, %rcx
141304cb5: 4d 03 4c 0a 58              	addq	0x58(%r10,%rcx), %r9
141304cba: 49 ff c3                    	incq	%r11
141304cbd: 4d 39 d8                    	cmpq	%r11, %r8
141304cc0: 75 de                       	jne	0x141304ca0 <.text+0x1303ca0>
141304cc2: eb 03                       	jmp	0x141304cc7 <.text+0x1303cc7>
141304cc4: 45 31 c9                    	xorl	%r9d, %r9d
141304cc7: 4c 03 8f b0 00 00 00        	addq	0xb0(%rdi), %r9
141304cce: 4c 2b 8f b8 00 00 00        	subq	0xb8(%rdi), %r9
141304cd5: 49 ff c1                    	incq	%r9
141304cd8: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
141304cdf: 49 0f 45 c1                 	cmovneq	%r9, %rax
141304ce3: 48 83 f8 05                 	cmpq	$0x5, %rax
141304ce7: 41 be 04 00 00 00           	movl	$0x4, %r14d
141304ced: 4c 0f 43 f0                 	cmovaeq	%rax, %r14
141304cf1: 45 31 ed                    	xorl	%r13d, %r13d
141304cf4: b9 28 00 00 00              	movl	$0x28, %ecx
141304cf9: 4c 89 f0                    	movq	%r14, %rax
141304cfc: 48 f7 e1                    	mulq	%rcx
141304cff: 49 89 c7                    	movq	%rax, %r15
141304d02: 0f 80 34 02 00 00           	jo	0x141304f3c <.text+0x1303f3c>
141304d08: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
141304d12: 49 39 c7                    	cmpq	%rax, %r15
141304d15: 0f 87 21 02 00 00           	ja	0x141304f3c <.text+0x1303f3c>
141304d1b: 4d 85 ff                    	testq	%r15, %r15
141304d1e: 74 28                       	je	0x141304d48 <.text+0x1303d48>
141304d20: 0f b6 05 e2 fe 32 02        	movzbl	0x232fee2(%rip), %eax   # 0x143634c09
141304d27: 41 bd 04 00 00 00           	movl	$0x4, %r13d
141304d2d: ba 04 00 00 00              	movl	$0x4, %edx
141304d32: 4c 89 f9                    	movq	%r15, %rcx
141304d35: e8 d6 ee 30 ff              	callq	0x140613c10 <.text+0x612c10>
141304d3a: 48 85 c0                    	testq	%rax, %rax
141304d3d: 0f 84 f9 01 00 00           	je	0x141304f3c <.text+0x1303f3c>
141304d43: 49 89 c4                    	movq	%rax, %r12
141304d46: eb 09                       	jmp	0x141304d51 <.text+0x1303d51>
141304d48: 41 bc 04 00 00 00           	movl	$0x4, %r12d
141304d4e: 45 31 f6                    	xorl	%r14d, %r14d
141304d51: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
141304d58: 49 89 44 24 20              	movq	%rax, 0x20(%r12)
141304d5d: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
141304d64: 0f 10 8d b0 00 00 00        	movups	0xb0(%rbp), %xmm1
141304d6b: 41 0f 11 4c 24 10           	movups	%xmm1, 0x10(%r12)
141304d71: 41 0f 11 04 24              	movups	%xmm0, (%r12)
141304d76: 4c 89 b5 c8 00 00 00        	movq	%r14, 0xc8(%rbp)
141304d7d: 4c 89 a5 d0 00 00 00        	movq	%r12, 0xd0(%rbp)
141304d84: 48 c7 85 d8 00 00 00 01 00 00 00    	movq	$0x1, 0xd8(%rbp)
141304d8f: 48 8d 5d b0                 	leaq	-0x50(%rbp), %rbx
141304d93: 41 b8 c8 00 00 00           	movl	$0xc8, %r8d
141304d99: 48 89 d9                    	movq	%rbx, %rcx
141304d9c: 48 89 fa                    	movq	%rdi, %rdx
141304d9f: e8 5c ee 61 01              	callq	0x142923c00 <.text+0x2922c00>
141304da4: bf 01 00 00 00              	movl	$0x1, %edi
141304da9: 4c 8d 75 78                 	leaq	0x78(%rbp), %r14
141304dad: 49 c7 c5 ff ff ff ff        	movq	$-0x1, %r13
141304db4: 4c 8d bd c8 00 00 00        	leaq	0xc8(%rbp), %r15
141304dbb: eb 33                       	jmp	0x141304df0 <.text+0x1303df0>
141304dbd: 0f 1f 00                    	nopl	(%rax)
141304dc0: 48 8d 04 bf                 	leaq	(%rdi,%rdi,4), %rax
141304dc4: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
141304dcb: 49 89 4c c4 20              	movq	%rcx, 0x20(%r12,%rax,8)
141304dd0: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
141304dd4: 0f 10 8d 88 00 00 00        	movups	0x88(%rbp), %xmm1
141304ddb: 41 0f 11 4c c4 10           	movups	%xmm1, 0x10(%r12,%rax,8)
141304de1: 41 0f 11 04 c4              	movups	%xmm0, (%r12,%rax,8)
141304de6: 48 ff c7                    	incq	%rdi
141304de9: 48 89 bd d8 00 00 00        	movq	%rdi, 0xd8(%rbp)
141304df0: 4c 89 f1                    	movq	%r14, %rcx
141304df3: 48 89 da                    	movq	%rbx, %rdx
141304df6: e8 f5 b5 fe ff              	callq	0x1412f03f0 <.text+0x12ef3f0>
141304dfb: 83 7d 78 02                 	cmpl	$0x2, 0x78(%rbp)
141304dff: 0f 84 d7 00 00 00           	je	0x141304edc <.text+0x1303edc>
141304e05: 48 3b bd c8 00 00 00        	cmpq	0xc8(%rbp), %rdi
141304e0c: 75 b2                       	jne	0x141304dc0 <.text+0x1303dc0>
141304e0e: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
141304e12: 4c 8b 4d d8                 	movq	-0x28(%rbp), %r9
141304e16: 80 7d 70 00                 	cmpb	$0x0, 0x70(%rbp)
141304e1a: 74 44                       	je	0x141304e60 <.text+0x1303e60>
141304e1c: 4c 39 c8                    	cmpq	%r9, %rax
141304e1f: 0f 84 7f 00 00 00           	je	0x141304ea4 <.text+0x1303ea4>
141304e25: 48 8b 4d b8                 	movq	-0x48(%rbp), %rcx
141304e29: 4c 8b 51 08                 	movq	0x8(%rcx), %r10
141304e2d: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
141304e31: 49 29 c1                    	subq	%rax, %r9
141304e34: 49 c1 e9 02                 	shrq	$0x2, %r9
141304e38: 45 31 c0                    	xorl	%r8d, %r8d
141304e3b: 45 31 db                    	xorl	%r11d, %r11d
141304e3e: 66 90                       	nop
141304e40: 42 8b 0c 98                 	movl	(%rax,%r11,4), %ecx
141304e44: 48 39 ca                    	cmpq	%rcx, %rdx
141304e47: 0f 86 bb 00 00 00           	jbe	0x141304f08 <.text+0x1303f08>
141304e4d: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
141304e51: 4d 03 44 ca 10              	addq	0x10(%r10,%rcx,8), %r8
141304e56: 49 ff c3                    	incq	%r11
141304e59: 4d 39 d9                    	cmpq	%r11, %r9
141304e5c: 75 e2                       	jne	0x141304e40 <.text+0x1303e40>
141304e5e: eb 47                       	jmp	0x141304ea7 <.text+0x1303ea7>
141304e60: 4c 39 c8                    	cmpq	%r9, %rax
141304e63: 74 3f                       	je	0x141304ea4 <.text+0x1303ea4>
141304e65: 48 8b 4d c0                 	movq	-0x40(%rbp), %rcx
141304e69: 4c 8b 51 08                 	movq	0x8(%rcx), %r10
141304e6d: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
141304e71: 49 29 c1                    	subq	%rax, %r9
141304e74: 49 c1 e9 02                 	shrq	$0x2, %r9
141304e78: 45 31 c0                    	xorl	%r8d, %r8d
141304e7b: 45 31 db                    	xorl	%r11d, %r11d
141304e7e: 66 90                       	nop
141304e80: 42 8b 0c 98                 	movl	(%rax,%r11,4), %ecx
141304e84: 48 39 ca                    	cmpq	%rcx, %rdx
141304e87: 0f 86 89 00 00 00           	jbe	0x141304f16 <.text+0x1303f16>
141304e8d: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
141304e91: 48 c1 e1 05                 	shlq	$0x5, %rcx
141304e95: 4d 03 44 0a 58              	addq	0x58(%r10,%rcx), %r8
141304e9a: 49 ff c3                    	incq	%r11
141304e9d: 4d 39 d9                    	cmpq	%r11, %r9
141304ea0: 75 de                       	jne	0x141304e80 <.text+0x1303e80>
141304ea2: eb 03                       	jmp	0x141304ea7 <.text+0x1303ea7>
141304ea4: 45 31 c0                    	xorl	%r8d, %r8d
141304ea7: 4c 03 45 60                 	addq	0x60(%rbp), %r8
141304eab: 4c 2b 45 68                 	subq	0x68(%rbp), %r8
141304eaf: 49 ff c0                    	incq	%r8
141304eb2: 4d 0f 44 c5                 	cmoveq	%r13, %r8
141304eb6: 48 c7 44 24 20 28 00 00 00  	movq	$0x28, 0x20(%rsp)
141304ebf: 41 b9 04 00 00 00           	movl	$0x4, %r9d
141304ec5: 4c 89 f9                    	movq	%r15, %rcx
141304ec8: 48 89 fa                    	movq	%rdi, %rdx
141304ecb: e8 90 4b 5a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141304ed0: 4c 8b a5 d0 00 00 00        	movq	0xd0(%rbp), %r12
141304ed7: e9 e4 fe ff ff              	jmp	0x141304dc0 <.text+0x1303dc0>
141304edc: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
141304ee3: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
141304ee7: 0f 10 85 c8 00 00 00        	movups	0xc8(%rbp), %xmm0
141304eee: 0f 11 06                    	movups	%xmm0, (%rsi)
141304ef1: 48 89 f0                    	movq	%rsi, %rax
141304ef4: 48 81 c4 68 01 00 00        	addq	$0x168, %rsp            # imm = 0x168
141304efb: 5b                          	popq	%rbx
141304efc: 5f                          	popq	%rdi
141304efd: 5e                          	popq	%rsi
141304efe: 41 5c                       	popq	%r12
141304f00: 41 5d                       	popq	%r13
141304f02: 41 5e                       	popq	%r14
141304f04: 41 5f                       	popq	%r15
141304f06: 5d                          	popq	%rbp
141304f07: c3                          	retq
141304f08: 4c 8d 05 f9 44 9d 01        	leaq	0x19d44f9(%rip), %r8    # 0x142cd9408
141304f0f: e8 04 46 5d 01              	callq	0x1428d9518 <.text+0x28d8518>
141304f14: eb 0c                       	jmp	0x141304f22 <.text+0x1303f22>
141304f16: 4c 8d 05 03 45 9d 01        	leaq	0x19d4503(%rip), %r8    # 0x142cd9420
141304f1d: e8 f6 45 5d 01              	callq	0x1428d9518 <.text+0x28d8518>
141304f22: 0f 0b                       	ud2
141304f24: 4c 8d 05 dd 44 9d 01        	leaq	0x19d44dd(%rip), %r8    # 0x142cd9408
141304f2b: e8 e8 45 5d 01              	callq	0x1428d9518 <.text+0x28d8518>
141304f30: 4c 8d 05 e9 44 9d 01        	leaq	0x19d44e9(%rip), %r8    # 0x142cd9420
141304f37: e8 dc 45 5d 01              	callq	0x1428d9518 <.text+0x28d8518>
141304f3c: 4c 89 e9                    	movq	%r13, %rcx
141304f3f: 4c 89 fa                    	movq	%r15, %rdx
141304f42: 49 89 d8                    	movq	%rbx, %r8
141304f45: e8 99 40 5d 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
141304f4a: cc                          	int3
141304f4b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141304f50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141304f55: 55                          	pushq	%rbp
141304f56: 41 57                       	pushq	%r15
141304f58: 41 56                       	pushq	%r14
141304f5a: 41 55                       	pushq	%r13
141304f5c: 41 54                       	pushq	%r12
141304f5e: 56                          	pushq	%rsi
141304f5f: 57                          	pushq	%rdi
141304f60: 53                          	pushq	%rbx
141304f61: 48 83 ec 28                 	subq	$0x28, %rsp
141304f65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141304f6c: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
141304f73: 48 85 c0                    	testq	%rax, %rax
141304f76: 74 1a                       	je	0x141304f92 <.text+0x1303f92>
141304f78: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
141304f7f: 48 c1 e0 03                 	shlq	$0x3, %rax
141304f83: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
141304f87: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141304f8d: e8 8e ec 30 ff              	callq	0x140613c20 <.text+0x612c20>
141304f92: 90                          	nop
141304f93: 48 83 c4 28                 	addq	$0x28, %rsp
141304f97: 5b                          	popq	%rbx
141304f98: 5f                          	popq	%rdi
141304f99: 5e                          	popq	%rsi
141304f9a: 41 5c                       	popq	%r12
141304f9c: 41 5d                       	popq	%r13
141304f9e: 41 5e                       	popq	%r14
141304fa0: 41 5f                       	popq	%r15
141304fa2: 5d                          	popq	%rbp
141304fa3: c3                          	retq
