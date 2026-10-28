
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b2cb30: 55                          	pushq	%rbp
140b2cb31: 41 57                       	pushq	%r15
140b2cb33: 41 56                       	pushq	%r14
140b2cb35: 41 54                       	pushq	%r12
140b2cb37: 56                          	pushq	%rsi
140b2cb38: 57                          	pushq	%rdi
140b2cb39: 53                          	pushq	%rbx
140b2cb3a: 48 81 ec 60 02 00 00        	subq	$0x260, %rsp            # imm = 0x260
140b2cb41: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b2cb49: 48 c7 85 d8 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1d8(%rbp)
140b2cb54: 4d 89 c7                    	movq	%r8, %r15
140b2cb57: 48 89 d7                    	movq	%rdx, %rdi
140b2cb5a: 48 89 ce                    	movq	%rcx, %rsi
140b2cb5d: 44 88 8d 90 01 00 00        	movb	%r9b, 0x190(%rbp)
140b2cb64: 4c 8d b2 c0 00 00 00        	leaq	0xc0(%rdx), %r14
140b2cb6b: 48 8d 9a 50 01 00 00        	leaq	0x150(%rdx), %rbx
140b2cb72: 48 8d 4a 60                 	leaq	0x60(%rdx), %rcx
140b2cb76: 4c 8d a2 90 00 00 00        	leaq	0x90(%rdx), %r12
140b2cb7d: e8 de cd 04 00              	callq	0x140b79960 <.text+0xb78960>
140b2cb82: 4c 89 e1                    	movq	%r12, %rcx
140b2cb85: e8 d6 cd 04 00              	callq	0x140b79960 <.text+0xb78960>
140b2cb8a: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
140b2cb8e: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140b2cb94: 4c 89 f2                    	movq	%r14, %rdx
140b2cb97: e8 64 70 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2cb9c: 48 c7 87 c0 00 00 00 00 00 00 00    	movq	$0x0, 0xc0(%rdi)
140b2cba7: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140b2cbab: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140b2cbb1: 48 89 da                    	movq	%rbx, %rdx
140b2cbb4: e8 47 70 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2cbb9: 48 c7 87 50 01 00 00 00 00 00 00    	movq	$0x0, 0x150(%rdi)
140b2cbc4: 48 8b 07                    	movq	(%rdi), %rax
140b2cbc7: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
140b2cbcb: 48 01 c1                    	addq	%rax, %rcx
140b2cbce: 48 ff c1                    	incq	%rcx
140b2cbd1: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
140b2cbd5: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
140b2cbd9: f7 d2                       	notl	%edx
140b2cbdb: 4c 8b 47 18                 	movq	0x18(%rdi), %r8
140b2cbdf: 0f 10 47 10                 	movups	0x10(%rdi), %xmm0
140b2cbe3: 0f 11 85 08 01 00 00        	movups	%xmm0, 0x108(%rbp)
140b2cbea: 0f 10 07                    	movups	(%rdi), %xmm0
140b2cbed: 0f 11 85 f8 00 00 00        	movups	%xmm0, 0xf8(%rbp)
140b2cbf4: 0f 10 05 c5 7f fc 01        	movups	0x1fc7fc5(%rip), %xmm0  # 0x142af4bc0
140b2cbfb: 0f 11 47 10                 	movups	%xmm0, 0x10(%rdi)
140b2cbff: f3 0f 6f 05 a9 7f fc 01     	movdqu	0x1fc7fa9(%rip), %xmm0  # 0x142af4bb0
140b2cc07: f3 0f 7f 07                 	movdqu	%xmm0, (%rdi)
140b2cc0b: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
140b2cc12: 48 83 c0 10                 	addq	$0x10, %rax
140b2cc16: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
140b2cc1d: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
140b2cc24: 66 89 95 e8 00 00 00        	movw	%dx, 0xe8(%rbp)
140b2cc2b: 4c 89 85 f0 00 00 00        	movq	%r8, 0xf0(%rbp)
140b2cc32: 48 89 bd 18 01 00 00        	movq	%rdi, 0x118(%rbp)
140b2cc39: 48 c7 45 a0 01 00 00 00     	movq	$0x1, -0x60(%rbp)
140b2cc41: 48 c7 45 38 01 00 00 00     	movq	$0x1, 0x38(%rbp)
140b2cc49: 4c 89 bd 20 01 00 00        	movq	%r15, 0x120(%rbp)
140b2cc50: 48 8d 85 90 01 00 00        	leaq	0x190(%rbp), %rax
140b2cc57: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
140b2cc5e: 48 8d 8d 30 01 00 00        	leaq	0x130(%rbp), %rcx
140b2cc65: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140b2cc69: e8 b2 cf 91 00              	callq	0x141449c20 <.text+0x1448c20>
140b2cc6e: 48 83 7f 18 00              	cmpq	$0x0, 0x18(%rdi)
140b2cc73: 0f 85 9d 01 00 00           	jne	0x140b2ce16 <.text+0xb2be16>
140b2cc79: 49 83 3e 00                 	cmpq	$0x0, (%r14)
140b2cc7d: 0f 85 b4 01 00 00           	jne	0x140b2ce37 <.text+0xb2be37>
140b2cc83: 48 83 3b 00                 	cmpq	$0x0, (%rbx)
140b2cc87: 0f 85 cb 01 00 00           	jne	0x140b2ce58 <.text+0xb2be58>
140b2cc8d: 48 83 7f 48 00              	cmpq	$0x0, 0x48(%rdi)
140b2cc92: 74 3e                       	je	0x140b2ccd2 <.text+0xb2bcd2>
140b2cc94: 48 8b 5f 38                 	movq	0x38(%rdi), %rbx
140b2cc98: 48 85 db                    	testq	%rbx, %rbx
140b2cc9b: 74 0f                       	je	0x140b2ccac <.text+0xb2bcac>
140b2cc9d: 48 8b 4f 30                 	movq	0x30(%rdi), %rcx
140b2cca1: 4c 8d 43 11                 	leaq	0x11(%rbx), %r8
140b2cca5: b2 ff                       	movb	$-0x1, %dl
140b2cca7: e8 74 6f df 01              	callq	0x142923c20 <.text+0x2922c20>
140b2ccac: 48 c7 47 48 00 00 00 00     	movq	$0x0, 0x48(%rdi)
140b2ccb4: 48 8d 43 01                 	leaq	0x1(%rbx), %rax
140b2ccb8: 48 89 c1                    	movq	%rax, %rcx
140b2ccbb: 48 c1 e9 03                 	shrq	$0x3, %rcx
140b2ccbf: 48 83 e0 f8                 	andq	$-0x8, %rax
140b2ccc3: 48 29 c8                    	subq	%rcx, %rax
140b2ccc6: 48 83 fb 08                 	cmpq	$0x8, %rbx
140b2ccca: 48 0f 42 c3                 	cmovbq	%rbx, %rax
140b2ccce: 48 89 47 40                 	movq	%rax, 0x40(%rdi)
140b2ccd2: c6 87 e0 01 00 00 00        	movb	$0x0, 0x1e0(%rdi)
140b2ccd9: 4c 8b 8d 30 01 00 00        	movq	0x130(%rbp), %r9
140b2cce0: 4c 8b 95 38 01 00 00        	movq	0x138(%rbp), %r10
140b2cce7: 4c 8b 85 48 01 00 00        	movq	0x148(%rbp), %r8
140b2ccee: 49 8d 4a 01                 	leaq	0x1(%r10), %rcx
140b2ccf2: 66 41 0f 6f 01              	movdqa	(%r9), %xmm0
140b2ccf7: 4d 85 d2                    	testq	%r10, %r10
140b2ccfa: 74 20                       	je	0x140b2cd1c <.text+0xb2bd1c>
140b2ccfc: ba 70 00 00 00              	movl	$0x70, %edx
140b2cd01: 48 89 c8                    	movq	%rcx, %rax
140b2cd04: 48 f7 e2                    	mulq	%rdx
140b2cd07: 49 8d 14 02                 	leaq	(%r10,%rax), %rdx
140b2cd0b: 48 83 c2 11                 	addq	$0x11, %rdx
140b2cd0f: 4d 89 ca                    	movq	%r9, %r10
140b2cd12: 49 29 c2                    	subq	%rax, %r10
140b2cd15: b8 10 00 00 00              	movl	$0x10, %eax
140b2cd1a: eb 02                       	jmp	0x140b2cd1e <.text+0xb2bd1e>
140b2cd1c: 31 c0                       	xorl	%eax, %eax
140b2cd1e: 4d 8d 59 10                 	leaq	0x10(%r9), %r11
140b2cd22: 66 0f d7 f8                 	pmovmskb	%xmm0, %edi
140b2cd26: f7 d7                       	notl	%edi
140b2cd28: 4c 01 c9                    	addq	%r9, %rcx
140b2cd2b: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
140b2cd32: 48 89 95 98 01 00 00        	movq	%rdx, 0x198(%rbp)
140b2cd39: 4c 89 95 a0 01 00 00        	movq	%r10, 0x1a0(%rbp)
140b2cd40: 4c 89 8d a8 01 00 00        	movq	%r9, 0x1a8(%rbp)
140b2cd47: 4c 89 9d b0 01 00 00        	movq	%r11, 0x1b0(%rbp)
140b2cd4e: 48 89 8d b8 01 00 00        	movq	%rcx, 0x1b8(%rbp)
140b2cd55: 66 89 bd c0 01 00 00        	movw	%di, 0x1c0(%rbp)
140b2cd5c: 4c 89 85 c8 01 00 00        	movq	%r8, 0x1c8(%rbp)
140b2cd63: c6 85 d6 01 00 00 01        	movb	$0x1, 0x1d6(%rbp)
140b2cd6a: 31 c9                       	xorl	%ecx, %ecx
140b2cd6c: e8 af cd 5a ff              	callq	0x1400d9b20 <.text+0xd8b20>
140b2cd71: 48 85 c0                    	testq	%rax, %rax
140b2cd74: 0f 84 ff 00 00 00           	je	0x140b2ce79 <.text+0xb2be79>
140b2cd7a: 0f 10 00                    	movups	(%rax), %xmm0
140b2cd7d: 48 ff 00                    	incq	(%rax)
140b2cd80: 0f 10 0d b9 2a 13 02        	movups	0x2132ab9(%rip), %xmm1  # 0x142c5f840
140b2cd87: 0f 29 8d 70 01 00 00        	movaps	%xmm1, 0x170(%rbp)
140b2cd8e: 0f 10 0d 9b 2a 13 02        	movups	0x2132a9b(%rip), %xmm1  # 0x142c5f830
140b2cd95: 0f 29 8d 60 01 00 00        	movaps	%xmm1, 0x160(%rbp)
140b2cd9c: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
140b2cda3: f3 0f 6f 85 90 01 00 00     	movdqu	0x190(%rbp), %xmm0
140b2cdab: 0f 10 8d a0 01 00 00        	movups	0x1a0(%rbp), %xmm1
140b2cdb2: 0f 10 95 b0 01 00 00        	movups	0x1b0(%rbp), %xmm2
140b2cdb9: 0f 10 9d c0 01 00 00        	movups	0x1c0(%rbp), %xmm3
140b2cdc0: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
140b2cdc4: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
140b2cdc8: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140b2cdcc: 66 0f 7f 45 a0              	movdqa	%xmm0, -0x60(%rbp)
140b2cdd1: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140b2cdd8: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140b2cddc: e8 df 69 9f 00              	callq	0x1415237c0 <.text+0x15227c0>
140b2cde1: 0f 28 85 60 01 00 00        	movaps	0x160(%rbp), %xmm0
140b2cde8: 0f 28 8d 70 01 00 00        	movaps	0x170(%rbp), %xmm1
140b2cdef: 0f 28 95 80 01 00 00        	movaps	0x180(%rbp), %xmm2
140b2cdf6: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140b2cdfa: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140b2cdfe: 0f 11 06                    	movups	%xmm0, (%rsi)
140b2ce01: 48 89 f0                    	movq	%rsi, %rax
140b2ce04: 48 81 c4 60 02 00 00        	addq	$0x260, %rsp            # imm = 0x260
140b2ce0b: 5b                          	popq	%rbx
140b2ce0c: 5f                          	popq	%rdi
140b2ce0d: 5e                          	popq	%rsi
140b2ce0e: 41 5c                       	popq	%r12
140b2ce10: 41 5e                       	popq	%r14
140b2ce12: 41 5f                       	popq	%r15
140b2ce14: 5d                          	popq	%rbp
140b2ce15: c3                          	retq
140b2ce16: c6 85 d7 01 00 00 01        	movb	$0x1, 0x1d7(%rbp)
140b2ce1d: 48 8d 0d 44 e8 fb 01        	leaq	0x1fbe844(%rip), %rcx   # 0x142aeb668
140b2ce24: 4c 8d 05 6d e8 fb 01        	leaq	0x1fbe86d(%rip), %r8    # 0x142aeb698
140b2ce2b: ba 2c 00 00 00              	movl	$0x2c, %edx
140b2ce30: e8 fb c5 da 01              	callq	0x1428d9430 <.text+0x28d8430>
140b2ce35: eb 55                       	jmp	0x140b2ce8c <.text+0xb2be8c>
140b2ce37: c6 85 d7 01 00 00 01        	movb	$0x1, 0x1d7(%rbp)
140b2ce3e: 48 8d 0d 6b e8 fb 01        	leaq	0x1fbe86b(%rip), %rcx   # 0x142aeb6b0
140b2ce45: 4c 8d 05 94 e8 fb 01        	leaq	0x1fbe894(%rip), %r8    # 0x142aeb6e0
140b2ce4c: ba 2e 00 00 00              	movl	$0x2e, %edx
140b2ce51: e8 da c5 da 01              	callq	0x1428d9430 <.text+0x28d8430>
140b2ce56: eb 34                       	jmp	0x140b2ce8c <.text+0xb2be8c>
140b2ce58: c6 85 d7 01 00 00 01        	movb	$0x1, 0x1d7(%rbp)
140b2ce5f: 48 8d 0d 92 e8 fb 01        	leaq	0x1fbe892(%rip), %rcx   # 0x142aeb6f8
140b2ce66: 4c 8d 05 bb e8 fb 01        	leaq	0x1fbe8bb(%rip), %r8    # 0x142aeb728
140b2ce6d: ba 2c 00 00 00              	movl	$0x2c, %edx
140b2ce72: e8 b9 c5 da 01              	callq	0x1428d9430 <.text+0x28d8430>
140b2ce77: eb 13                       	jmp	0x140b2ce8c <.text+0xb2be8c>
140b2ce79: c6 85 d6 01 00 00 01        	movb	$0x1, 0x1d6(%rbp)
140b2ce80: 48 8d 0d 19 dc 10 02        	leaq	0x210dc19(%rip), %rcx   # 0x142c3aaa0
