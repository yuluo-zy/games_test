
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14146eb31: 48 89 b5 b0 01 00 00        	movq	%rsi, 0x1b0(%rbp)
14146eb38: 48 8d 8d b8 01 00 00        	leaq	0x1b8(%rbp), %rcx
14146eb3f: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
14146eb43: 41 b8 a8 01 00 00           	movl	$0x1a8, %r8d            # imm = 0x1A8
14146eb49: e8 b2 50 4b 01              	callq	0x142923c00 <.text+0x2922c00>
14146eb4e: 48 0f ba a5 10 09 00 00 30  	btq	$0x30, 0x910(%rbp)
14146eb57: 0f 83 3f 01 00 00           	jae	0x14146ec9c <.text+0x146dc9c>
14146eb5d: 48 8b 05 44 d8 1b 02        	movq	0x21bd844(%rip), %rax   # 0x14362c3a8
14146eb64: 48 8b 00                    	movq	(%rax), %rax
14146eb67: 48 83 f8 03                 	cmpq	$0x3, %rax
14146eb6b: 0f 82 c4 00 00 00           	jb	0x14146ec35 <.text+0x146dc35>
14146eb71: c6 85 73 09 00 00 01        	movb	$0x1, 0x973(%rbp)
14146eb78: 48 8d 0d 21 5f 7f 01        	leaq	0x17f5f21(%rip), %rcx   # 0x142c64aa0
14146eb7f: e8 fc f1 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
14146eb84: 0f 10 00                    	movups	(%rax), %xmm0
14146eb87: 8b 40 10                    	movl	0x10(%rax), %eax
14146eb8a: 48 c7 85 e0 05 00 00 03 00 00 00    	movq	$0x3, 0x5e0(%rbp)
14146eb95: 48 8d 0d ac 55 7f 01        	leaq	0x17f55ac(%rip), %rcx   # 0x142c64148
14146eb9c: 48 89 8d e8 05 00 00        	movq	%rcx, 0x5e8(%rbp)
14146eba3: 48 c7 85 f0 05 00 00 26 00 00 00    	movq	$0x26, 0x5f0(%rbp)
14146ebae: 48 8d 15 db 5e 7f 01        	leaq	0x17f5edb(%rip), %rdx   # 0x142c64a90
14146ebb5: 48 89 95 00 06 00 00        	movq	%rdx, 0x600(%rbp)
14146ebbc: 48 c7 85 08 06 00 00 01 00 00 00    	movq	$0x1, 0x608(%rbp)
14146ebc7: 48 c7 85 10 06 00 00 08 00 00 00    	movq	$0x8, 0x610(%rbp)
14146ebd2: 48 c7 85 b0 05 00 00 00 00 00 00    	movq	$0x0, 0x5b0(%rbp)
14146ebdd: 0f 57 c9                    	xorps	%xmm1, %xmm1
14146ebe0: 0f 11 8d 18 06 00 00        	movups	%xmm1, 0x618(%rbp)
14146ebe7: 48 89 8d b8 05 00 00        	movq	%rcx, 0x5b8(%rbp)
14146ebee: 48 c7 85 c0 05 00 00 26 00 00 00    	movq	$0x26, 0x5c0(%rbp)
14146ebf9: 48 c7 85 c8 05 00 00 00 00 00 00    	movq	$0x0, 0x5c8(%rbp)
14146ec04: 0f 11 85 d0 05 00 00        	movups	%xmm0, 0x5d0(%rbp)
14146ec0b: c7 85 f8 05 00 00 01 00 00 00       	movl	$0x1, 0x5f8(%rbp)
14146ec15: 89 85 fc 05 00 00           	movl	%eax, 0x5fc(%rbp)
14146ec1b: c6 85 73 09 00 00 01        	movb	$0x1, 0x973(%rbp)
14146ec22: 48 8d 8d 72 09 00 00        	leaq	0x972(%rbp), %rcx
14146ec29: 48 8d 95 b0 05 00 00        	leaq	0x5b0(%rbp), %rdx
14146ec30: e8 fb b6 8d ff              	callq	0x140d4a330 <.text+0xd49330>
14146ec35: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
14146ec3c: 48 85 c9                    	testq	%rcx, %rcx
14146ec3f: 74 5b                       	je	0x14146ec9c <.text+0x146dc9c>
14146ec41: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
14146ec48: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14146ec4c: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
14146ec50: eb 09                       	jmp	0x14146ec5b <.text+0x146dc5b>
14146ec52: 48 83 c0 28                 	addq	$0x28, %rax
14146ec56: 48 39 c8                    	cmpq	%rcx, %rax
14146ec59: 74 41                       	je	0x14146ec9c <.text+0x146dc9c>
14146ec5b: 4c 8b 40 20                 	movq	0x20(%rax), %r8
14146ec5f: 4d 85 c0                    	testq	%r8, %r8
14146ec62: 74 ee                       	je	0x14146ec52 <.text+0x146dc52>
14146ec64: 48 8b 50 18                 	movq	0x18(%rax), %rdx
14146ec68: 4d 6b c0 58                 	imulq	$0x58, %r8, %r8
14146ec6c: 45 31 c9                    	xorl	%r9d, %r9d
14146ec6f: eb 18                       	jmp	0x14146ec89 <.text+0x146dc89>
14146ec71: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14146ec80: 49 83 c1 58                 	addq	$0x58, %r9
14146ec84: 4d 39 c8                    	cmpq	%r9, %r8
14146ec87: 74 c9                       	je	0x14146ec52 <.text+0x146dc52>
14146ec89: 42 80 7c 0a 52 3a           	cmpb	$0x3a, 0x52(%rdx,%r9)
14146ec8f: 75 ef                       	jne	0x14146ec80 <.text+0x146dc80>
14146ec91: 42 c7 44 0a 30 e7 03 00 00  	movl	$0x3e7, 0x30(%rdx,%r9)  # imm = 0x3E7
14146ec9a: eb e4                       	jmp	0x14146ec80 <.text+0x146dc80>
14146ec9c: 48 0f ba a5 10 09 00 00 28  	btq	$0x28, 0x910(%rbp)
14146eca5: 0f 83 f3 0a 00 00           	jae	0x14146f79e <.text+0x146e79e>
14146ecab: 48 8b 05 f6 d6 1b 02        	movq	0x21bd6f6(%rip), %rax   # 0x14362c3a8
14146ecb2: 48 8b 00                    	movq	(%rax), %rax
14146ecb5: 48 83 f8 03                 	cmpq	$0x3, %rax
14146ecb9: 0f 82 c4 00 00 00           	jb	0x14146ed83 <.text+0x146dd83>
14146ecbf: c6 85 73 09 00 00 01        	movb	$0x1, 0x973(%rbp)
14146ecc6: 48 8d 0d 33 5e 7f 01        	leaq	0x17f5e33(%rip), %rcx   # 0x142c64b00
14146eccd: e8 ae f0 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
14146ecd2: 0f 10 00                    	movups	(%rax), %xmm0
14146ecd5: 8b 40 10                    	movl	0x10(%rax), %eax
14146ecd8: 48 c7 85 e0 05 00 00 03 00 00 00    	movq	$0x3, 0x5e0(%rbp)
14146ece3: 48 8d 0d 5e 54 7f 01        	leaq	0x17f545e(%rip), %rcx   # 0x142c64148
14146ecea: 48 89 8d e8 05 00 00        	movq	%rcx, 0x5e8(%rbp)
14146ecf1: 48 c7 85 f0 05 00 00 26 00 00 00    	movq	$0x26, 0x5f0(%rbp)
14146ecfc: 48 8d 15 ed 5d 7f 01        	leaq	0x17f5ded(%rip), %rdx   # 0x142c64af0
14146ed03: 48 89 95 00 06 00 00        	movq	%rdx, 0x600(%rbp)
14146ed0a: 48 c7 85 08 06 00 00 01 00 00 00    	movq	$0x1, 0x608(%rbp)
14146ed15: 48 c7 85 10 06 00 00 08 00 00 00    	movq	$0x8, 0x610(%rbp)
14146ed20: 48 c7 85 b0 05 00 00 00 00 00 00    	movq	$0x0, 0x5b0(%rbp)
14146ed2b: 0f 57 c9                    	xorps	%xmm1, %xmm1
14146ed2e: 0f 11 8d 18 06 00 00        	movups	%xmm1, 0x618(%rbp)
14146ed35: 48 89 8d b8 05 00 00        	movq	%rcx, 0x5b8(%rbp)
14146ed3c: 48 c7 85 c0 05 00 00 26 00 00 00    	movq	$0x26, 0x5c0(%rbp)
14146ed47: 48 c7 85 c8 05 00 00 00 00 00 00    	movq	$0x0, 0x5c8(%rbp)
14146ed52: 0f 11 85 d0 05 00 00        	movups	%xmm0, 0x5d0(%rbp)
14146ed59: c7 85 f8 05 00 00 01 00 00 00       	movl	$0x1, 0x5f8(%rbp)
14146ed63: 89 85 fc 05 00 00           	movl	%eax, 0x5fc(%rbp)
14146ed69: c6 85 73 09 00 00 01        	movb	$0x1, 0x973(%rbp)
14146ed70: 48 8d 8d 72 09 00 00        	leaq	0x972(%rbp), %rcx
14146ed77: 48 8d 95 b0 05 00 00        	leaq	0x5b0(%rbp), %rdx
14146ed7e: e8 ad b5 8d ff              	callq	0x140d4a330 <.text+0xd49330>
14146ed83: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
14146ed8a: 48 85 c0                    	testq	%rax, %rax
14146ed8d: 0f 84 45 09 00 00           	je	0x14146f6d8 <.text+0x146e6d8>
14146ed93: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
14146ed9a: 48 c1 e0 04                 	shlq	$0x4, %rax
14146ed9e: 31 d2                       	xorl	%edx, %edx
14146eda0: 4c 8d 05 01 5f 7f 01        	leaq	0x17f5f01(%rip), %r8    # 0x142c64ca8
14146eda7: eb 15                       	jmp	0x14146edbe <.text+0x146ddbe>
14146eda9: 45 89 d1                    	movl	%r10d, %r9d
14146edac: 44 89 4c 11 08              	movl	%r9d, 0x8(%rcx,%rdx)
14146edb1: 48 83 c2 10                 	addq	$0x10, %rdx
14146edb5: 48 39 d0                    	cmpq	%rdx, %rax
14146edb8: 0f 84 1a 09 00 00           	je	0x14146f6d8 <.text+0x146e6d8>
14146edbe: 44 8b 54 11 08              	movl	0x8(%rcx,%rdx), %r10d
14146edc3: 49 83 fa 08                 	cmpq	$0x8, %r10
14146edc7: 77 12                       	ja	0x14146eddb <.text+0x146dddb>
14146edc9: 4f 63 0c 90                 	movslq	(%r8,%r10,4), %r9
14146edcd: 4d 01 c1                    	addq	%r8, %r9
14146edd0: 41 ff e1                    	jmpq	*%r9
14146edd3: 41 b9 02 00 00 00           	movl	$0x2, %r9d
14146edd9: eb d1                       	jmp	0x14146edac <.text+0x146ddac>
14146eddb: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14146ede1: 41 81 fa e7 03 00 00        	cmpl	$0x3e7, %r10d           # imm = 0x3E7
14146ede8: 74 bf                       	je	0x14146eda9 <.text+0x146dda9>
14146edea: eb c0                       	jmp	0x14146edac <.text+0x146ddac>
14146edec: 41 b9 03 00 00 00           	movl	$0x3, %r9d
14146edf2: eb b8                       	jmp	0x14146edac <.text+0x146ddac>
14146edf4: 41 b9 05 00 00 00           	movl	$0x5, %r9d
14146edfa: eb b0                       	jmp	0x14146edac <.text+0x146ddac>
14146edfc: 41 b9 08 00 00 00           	movl	$0x8, %r9d
14146ee02: eb a8                       	jmp	0x14146edac <.text+0x146ddac>
14146ee04: 41 b9 04 00 00 00           	movl	$0x4, %r9d
14146ee0a: eb a0                       	jmp	0x14146edac <.text+0x146ddac>
