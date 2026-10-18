
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416a3af0: 00 f3                       	addb	%dh, %bl
1416a3af2: 0f bc c0                    	bsfl	%eax, %eax
1416a3af5: 48 f7 d8                    	negq	%rax
1416a3af8: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
1416a3afc: 48 c1 e0 04                 	shlq	$0x4, %rax
1416a3b00: 4d 8d 24 06                 	leaq	(%r14,%rax), %r12
1416a3b04: 49 8d 34 06                 	leaq	(%r14,%rax), %rsi
1416a3b08: 48 83 c6 c8                 	addq	$-0x38, %rsi
1416a3b0c: 49 8b 44 06 c8              	movq	-0x38(%r14,%rax), %rax
1416a3b11: 48 b9 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
1416a3b1b: 48 31 c8                    	xorq	%rcx, %rax
1416a3b1e: 48 89 95 60 03 00 00        	movq	%rdx, 0x360(%rbp)
1416a3b25: 74 49                       	je	0x1416a3b70 <.text+0x16a2b70>
1416a3b27: 48 83 f8 01                 	cmpq	$0x1, %rax
1416a3b2b: 0f 85 02 0a 00 00           	jne	0x1416a4533 <.text+0x16a3533>
1416a3b31: 49 8d 7c 24 d0              	leaq	-0x30(%r12), %rdi
1416a3b36: c6 85 7f 03 00 00 01        	movb	$0x1, 0x37f(%rbp)
1416a3b3d: 48 89 f9                    	movq	%rdi, %rcx
1416a3b40: e8 3b 98 a1 00              	callq	0x1420bd380 <.text+0x20bc380>
1416a3b45: 41 8b 4c 24 d8              	movl	-0x28(%r12), %ecx
1416a3b4a: 48 0f af c1                 	imulq	%rcx, %rax
1416a3b4e: c6 85 7f 03 00 00 01        	movb	$0x1, 0x37f(%rbp)
1416a3b55: 48 89 f9                    	movq	%rdi, %rcx
1416a3b58: 48 89 c2                    	movq	%rax, %rdx
1416a3b5b: e8 f0 2c 54 ff              	callq	0x140be6850 <.text+0xbe5850>
1416a3b60: eb 34                       	jmp	0x1416a3b96 <.text+0x16a2b96>
1416a3b62: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1416a3b70: 49 8d 7c 24 d0              	leaq	-0x30(%r12), %rdi
1416a3b75: c6 85 7f 03 00 00 01        	movb	$0x1, 0x37f(%rbp)
1416a3b7c: 48 89 f9                    	movq	%rdi, %rcx
1416a3b7f: e8 fc 97 a1 00              	callq	0x1420bd380 <.text+0x20bc380>
1416a3b84: c6 85 7f 03 00 00 01        	movb	$0x1, 0x37f(%rbp)
1416a3b8b: 48 89 f9                    	movq	%rdi, %rcx
1416a3b8e: 48 89 c2                    	movq	%rax, %rdx
1416a3b91: e8 ba 2c 54 ff              	callq	0x140be6850 <.text+0xbe5850>
1416a3b96: 89 c7                       	movl	%eax, %edi
1416a3b98: c6 85 7f 03 00 00 01        	movb	$0x1, 0x37f(%rbp)
1416a3b9f: 48 89 f1                    	movq	%rsi, %rcx
1416a3ba2: e8 59 9e a1 00              	callq	0x1420bda00 <.text+0x20bca00>
1416a3ba7: a8 01                       	testb	$0x1, %al
1416a3ba9: 0f 84 60 09 00 00           	je	0x1416a450f <.text+0x16a350f>
1416a3baf: 48 89 d6                    	movq	%rdx, %rsi
1416a3bb2: 41 8b 5c 24 f8              	movl	-0x8(%r12), %ebx
1416a3bb7: 4c 8b ad 58 03 00 00        	movq	0x358(%rbp), %r13
1416a3bbe: 4c 3b ad 48 03 00 00        	cmpq	0x348(%rbp), %r13
1416a3bc5: 75 1a                       	jne	0x1416a3be1 <.text+0x16a2be1>
1416a3bc7: c6 85 7f 03 00 00 01        	movb	$0x1, 0x37f(%rbp)
1416a3bce: 48 8d 8d 48 03 00 00        	leaq	0x348(%rbp), %rcx
1416a3bd5: 48 8d 15 d4 4f 5e 01        	leaq	0x15e4fd4(%rip), %rdx   # 0x142c88bb0
1416a3bdc: e8 ef 97 fe fe              	callq	0x14068d3d0 <.text+0x68c3d0>
1416a3be1: 48 89 d8                    	movq	%rbx, %rax
1416a3be4: 48 c1 e0 20                 	shlq	$0x20, %rax
1416a3be8: 48 09 d8                    	orq	%rbx, %rax
1416a3beb: 48 8b 8d 50 03 00 00        	movq	0x350(%rbp), %rcx
1416a3bf2: 4c 89 ea                    	movq	%r13, %rdx
1416a3bf5: 48 c1 e2 04                 	shlq	$0x4, %rdx
1416a3bf9: 48 89 04 11                 	movq	%rax, (%rcx,%rdx)
1416a3bfd: 89 7c 11 08                 	movl	%edi, 0x8(%rcx,%rdx)
1416a3c01: c7 44 11 0c 00 00 00 00     	movl	$0x0, 0xc(%rcx,%rdx)
1416a3c09: 49 ff c5                    	incq	%r13
1416a3c0c: 4c 89 ad 58 03 00 00        	movq	%r13, 0x358(%rbp)
1416a3c13: 45 8b 64 24 f8              	movl	-0x8(%r12), %r12d
1416a3c18: 48 8b bd b0 02 00 00        	movq	0x2b0(%rbp), %rdi
1416a3c1f: 48 3b bd a0 02 00 00        	cmpq	0x2a0(%rbp), %rdi
