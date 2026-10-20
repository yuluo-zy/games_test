
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140929a90: 55                          	pushq	%rbp
140929a91: 41 57                       	pushq	%r15
140929a93: 41 56                       	pushq	%r14
140929a95: 41 55                       	pushq	%r13
140929a97: 41 54                       	pushq	%r12
140929a99: 56                          	pushq	%rsi
140929a9a: 57                          	pushq	%rdi
140929a9b: 53                          	pushq	%rbx
140929a9c: 48 81 ec f8 00 00 00        	subq	$0xf8, %rsp
140929aa3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140929aab: 0f 29 75 60                 	movaps	%xmm6, 0x60(%rbp)
140929aaf: 48 c7 45 58 fe ff ff ff     	movq	$-0x2, 0x58(%rbp)
140929ab7: 48 89 d7                    	movq	%rdx, %rdi
140929aba: 48 89 cb                    	movq	%rcx, %rbx
140929abd: e8 3e cf 2e 00              	callq	0x140c16a00 <.text+0xc15a00>
140929ac2: 84 c0                       	testb	%al, %al
140929ac4: 88 45 57                    	movb	%al, 0x57(%rbp)
140929ac7: 74 5f                       	je	0x140929b28 <.text+0x928b28>
140929ac9: 4c 8d 35 95 4b 18 02        	leaq	0x2184b95(%rip), %r14   # 0x142aae665
140929ad0: be 50 00 00 00              	movl	$0x50, %esi
140929ad5: ba 50 00 00 00              	movl	$0x50, %edx
140929ada: 4c 89 f1                    	movq	%r14, %rcx
140929add: e8 3e 9e ff ff              	callq	0x140923920 <.text+0x922920>
140929ae2: 48 83 f8 01                 	cmpq	$0x1, %rax
140929ae6: 0f 85 b1 00 00 00           	jne	0x140929b9d <.text+0x928b9d>
140929aec: 49 89 d1                    	movq	%rdx, %r9
140929aef: 48 85 d2                    	testq	%rdx, %rdx
140929af2: 74 3d                       	je	0x140929b31 <.text+0x928b31>
140929af4: 49 83 f9 50                 	cmpq	$0x50, %r9
140929af8: 73 35                       	jae	0x140929b2f <.text+0x928b2f>
140929afa: 48 8d 05 64 4b 18 02        	leaq	0x2184b64(%rip), %rax   # 0x142aae665
140929b01: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
140929b06: 7f 29                       	jg	0x140929b31 <.text+0x928b31>
140929b08: 48 8d 05 29 4c 18 02        	leaq	0x2184c29(%rip), %rax   # 0x142aae738
140929b0f: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140929b14: 48 8d 0d 4a 4b 18 02        	leaq	0x2184b4a(%rip), %rcx   # 0x142aae665
140929b1b: ba 50 00 00 00              	movl	$0x50, %edx
140929b20: 45 31 c0                    	xorl	%r8d, %r8d
140929b23: e8 18 fe fa 01              	callq	0x1428d9940 <.text+0x28d8940>
140929b28: 31 c0                       	xorl	%eax, %eax
140929b2a: e9 cb 01 00 00              	jmp	0x140929cfa <.text+0x928cfa>
140929b2f: 75 d7                       	jne	0x140929b08 <.text+0x928b08>
140929b31: 4c 8d 35 2d 4b 18 02        	leaq	0x2184b2d(%rip), %r14   # 0x142aae665
140929b38: 4c 89 f1                    	movq	%r14, %rcx
140929b3b: 4c 89 ca                    	movq	%r9, %rdx
140929b3e: e8 dd 9d ff ff              	callq	0x140923920 <.text+0x922920>
140929b43: 48 83 f8 01                 	cmpq	$0x1, %rax
140929b47: 75 54                       	jne	0x140929b9d <.text+0x928b9d>
140929b49: 49 89 d0                    	movq	%rdx, %r8
140929b4c: 49 83 c0 02                 	addq	$0x2, %r8
140929b50: 74 39                       	je	0x140929b8b <.text+0x928b8b>
140929b52: 49 83 f8 50                 	cmpq	$0x50, %r8
140929b56: 73 31                       	jae	0x140929b89 <.text+0x928b89>
140929b58: 48 8d 05 06 4b 18 02        	leaq	0x2184b06(%rip), %rax   # 0x142aae665
140929b5f: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140929b64: 7f 25                       	jg	0x140929b8b <.text+0x928b8b>
140929b66: 48 8d 05 e3 4b 18 02        	leaq	0x2184be3(%rip), %rax   # 0x142aae750
140929b6d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140929b72: 48 8d 0d ec 4a 18 02        	leaq	0x2184aec(%rip), %rcx   # 0x142aae665
140929b79: ba 50 00 00 00              	movl	$0x50, %edx
140929b7e: 41 b9 50 00 00 00           	movl	$0x50, %r9d
140929b84: e8 b7 fd fa 01              	callq	0x1428d9940 <.text+0x28d8940>
140929b89: 75 db                       	jne	0x140929b66 <.text+0x928b66>
140929b8b: be 4e 00 00 00              	movl	$0x4e, %esi
140929b90: 48 29 d6                    	subq	%rdx, %rsi
140929b93: 4c 8d 35 cb 4a 18 02        	leaq	0x2184acb(%rip), %r14   # 0x142aae665
140929b9a: 4d 01 c6                    	addq	%r8, %r14
140929b9d: 48 8d 05 e2 59 18 02        	leaq	0x21859e2(%rip), %rax   # 0x142aaf586
140929ba4: 4c 8d 3d 91 59 18 02        	leaq	0x2185991(%rip), %r15   # 0x142aaf53c
140929bab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140929bb0: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
140929bb4: 85 c9                       	testl	%ecx, %ecx
140929bb6: 78 18                       	js	0x140929bd0 <.text+0x928bd0>
140929bb8: 48 ff c8                    	decq	%rax
140929bbb: 83 f9 5c                    	cmpl	$0x5c, %ecx
140929bbe: 75 69                       	jne	0x140929c29 <.text+0x928c29>
140929bc0: eb 7d                       	jmp	0x140929c3f <.text+0x928c3f>
140929bc2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140929bd0: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
140929bd4: 80 fa c0                    	cmpb	$-0x40, %dl
140929bd7: 7d 25                       	jge	0x140929bfe <.text+0x928bfe>
140929bd9: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
140929bde: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
140929be2: 7d 23                       	jge	0x140929c07 <.text+0x928c07>
140929be4: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
140929be9: 48 83 c0 fc                 	addq	$-0x4, %rax
140929bed: 41 83 e1 07                 	andl	$0x7, %r9d
140929bf1: 41 c1 e1 06                 	shll	$0x6, %r9d
140929bf5: 41 83 e0 3f                 	andl	$0x3f, %r8d
140929bf9: 45 09 c8                    	orl	%r9d, %r8d
140929bfc: eb 11                       	jmp	0x140929c0f <.text+0x928c0f>
140929bfe: 48 83 c0 fe                 	addq	$-0x2, %rax
140929c02: 83 e2 1f                    	andl	$0x1f, %edx
140929c05: eb 12                       	jmp	0x140929c19 <.text+0x928c19>
140929c07: 48 83 c0 fd                 	addq	$-0x3, %rax
140929c0b: 41 83 e0 0f                 	andl	$0xf, %r8d
140929c0f: 41 c1 e0 06                 	shll	$0x6, %r8d
140929c13: 83 e2 3f                    	andl	$0x3f, %edx
140929c16: 44 09 c2                    	orl	%r8d, %edx
140929c19: c1 e2 06                    	shll	$0x6, %edx
140929c1c: 80 e1 3f                    	andb	$0x3f, %cl
140929c1f: 0f b6 c9                    	movzbl	%cl, %ecx
140929c22: 09 d1                       	orl	%edx, %ecx
140929c24: 83 f9 5c                    	cmpl	$0x5c, %ecx
140929c27: 74 16                       	je	0x140929c3f <.text+0x928c3f>
140929c29: 83 f9 2f                    	cmpl	$0x2f, %ecx
140929c2c: 74 11                       	je	0x140929c3f <.text+0x928c3f>
140929c2e: 4c 39 f8                    	cmpq	%r15, %rax
140929c31: 0f 85 79 ff ff ff           	jne	0x140929bb0 <.text+0x928bb0>
140929c37: 41 bc 4a 00 00 00           	movl	$0x4a, %r12d
140929c3d: eb 50                       	jmp	0x140929c8f <.text+0x928c8f>
140929c3f: 4c 8d 3d f6 58 18 02        	leaq	0x21858f6(%rip), %r15   # 0x142aaf53c
140929c46: 4c 29 f8                    	subq	%r15, %rax
140929c49: 49 89 c0                    	movq	%rax, %r8
140929c4c: 49 ff c0                    	incq	%r8
140929c4f: 74 32                       	je	0x140929c83 <.text+0x928c83>
140929c51: 49 83 f8 4a                 	cmpq	$0x4a, %r8
140929c55: 73 2a                       	jae	0x140929c81 <.text+0x928c81>
140929c57: 43 80 3c 38 bf              	cmpb	$-0x41, (%r8,%r15)
140929c5c: 7f 25                       	jg	0x140929c83 <.text+0x928c83>
140929c5e: 48 8d 05 b3 4a 18 02        	leaq	0x2184ab3(%rip), %rax   # 0x142aae718
140929c65: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140929c6a: 48 8d 0d cb 58 18 02        	leaq	0x21858cb(%rip), %rcx   # 0x142aaf53c
140929c71: ba 4a 00 00 00              	movl	$0x4a, %edx
140929c76: 41 b9 4a 00 00 00           	movl	$0x4a, %r9d
140929c7c: e8 bf fc fa 01              	callq	0x1428d9940 <.text+0x28d8940>
140929c81: 75 db                       	jne	0x140929c5e <.text+0x928c5e>
140929c83: 41 bc 49 00 00 00           	movl	$0x49, %r12d
140929c89: 49 29 c4                    	subq	%rax, %r12
140929c8c: 4d 01 c7                    	addq	%r8, %r15
140929c8f: e8 9c e4 2e 00              	callq	0x140c18130 <.text+0xc17130>
140929c94: 48 89 c1                    	movq	%rax, %rcx
140929c97: 48 8b 00                    	movq	(%rax), %rax
140929c9a: 48 83 f8 01                 	cmpq	$0x1, %rax
140929c9e: 0f 85 27 03 00 00           	jne	0x140929fcb <.text+0x928fcb>
140929ca4: 48 83 c1 08                 	addq	$0x8, %rcx
140929ca8: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140929cac: 0f 85 53 03 00 00           	jne	0x14092a005 <.text+0x929005>
140929cb2: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140929cb9: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
140929cbd: 48 83 c1 08                 	addq	$0x8, %rcx
140929cc1: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
140929cc6: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140929ccf: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140929cd8: 4c 89 f2                    	movq	%r14, %rdx
140929cdb: 49 89 f0                    	movq	%rsi, %r8
140929cde: 4d 89 f9                    	movq	%r15, %r9
140929ce1: e8 5a d0 2e 00              	callq	0x140c16d40 <.text+0xc15d40>
140929ce6: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
140929cea: 48 ff 01                    	incq	(%rcx)
140929ced: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140929cf1: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140929cf5: b8 01 00 00 00              	movl	$0x1, %eax
140929cfa: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140929cfe: 48 8b 0b                    	movq	(%rbx), %rcx
140929d01: 4c 8b 7b 08                 	movq	0x8(%rbx), %r15
140929d05: 4c 8b a1 f8 00 00 00        	movq	0xf8(%rcx), %r12
140929d0c: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
140929d13: 4d 8d 2c 84                 	leaq	(%r12,%rax,4), %r13
140929d17: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
140929d1b: 0f b6 b1 08 01 00 00        	movzbl	0x108(%rcx), %esi
140929d22: 48 8b 07                    	movq	(%rdi), %rax
140929d25: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140929d29: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140929d2d: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
140929d31: 48 8b 47 18                 	movq	0x18(%rdi), %rax
140929d35: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140929d39: 48 8b 47 10                 	movq	0x10(%rdi), %rax
140929d3d: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140929d41: b8 08 00 00 00              	movl	$0x8, %eax
140929d46: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140929d4a: 31 db                       	xorl	%ebx, %ebx
140929d4c: 0f 57 f6                    	xorps	%xmm6, %xmm6
140929d4f: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140929d53: 45 31 f6                    	xorl	%r14d, %r14d
140929d56: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
140929d5e: 31 ff                       	xorl	%edi, %edi
140929d60: eb 45                       	jmp	0x140929da7 <.text+0x928da7>
140929d62: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140929d70: 49 ff c6                    	incq	%r14
140929d73: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140929d77: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140929d7b: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140929d7f: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140929d83: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
140929d87: 48 8b 45 18                 	movq	0x18(%rbp), %rax
140929d8b: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
140929d8f: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140929d93: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
140929d97: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140929d9b: 48 8d 15 e6 57 18 02        	leaq	0x21857e6(%rip), %rdx   # 0x142aaf588
140929da2: e8 c9 a9 3e 00              	callq	0x140d14770 <.text+0xd13770>
140929da7: 40 84 f6                    	testb	%sil, %sil
140929daa: 0f 84 80 00 00 00           	je	0x140929e30 <.text+0x928e30>
140929db0: 49 39 de                    	cmpq	%rbx, %r14
140929db3: 0f 85 3a 01 00 00           	jne	0x140929ef3 <.text+0x928ef3>
140929db9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140929dc0: 4d 39 ec                    	cmpq	%r13, %r12
140929dc3: 0f 84 aa 01 00 00           	je	0x140929f73 <.text+0x928f73>
140929dc9: 41 8b 0c 24                 	movl	(%r12), %ecx
140929dcd: 49 83 c4 04                 	addq	$0x4, %r12
140929dd1: 49 8b 87 a8 01 00 00        	movq	0x1a8(%r15), %rax
140929dd8: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
140929ddc: 48 8b 5c c8 10              	movq	0x10(%rax,%rcx,8), %rbx
140929de1: 48 85 db                    	testq	%rbx, %rbx
140929de4: 74 da                       	je	0x140929dc0 <.text+0x928dc0>
140929de6: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
140929dea: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140929dee: 48 8b 89 18 01 00 00        	movq	0x118(%rcx), %rcx
140929df5: 48 39 48 40                 	cmpq	%rcx, 0x40(%rax)
140929df9: 0f 86 e7 00 00 00           	jbe	0x140929ee6 <.text+0x928ee6>
140929dff: 48 8b 50 38                 	movq	0x38(%rax), %rdx
140929e03: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
140929e07: 48 85 c9                    	testq	%rcx, %rcx
140929e0a: 0f 84 d6 00 00 00           	je	0x140929ee6 <.text+0x928ee6>
140929e10: 48 8b 50 18                 	movq	0x18(%rax), %rdx
140929e14: 48 f7 d1                    	notq	%rcx
140929e17: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140929e1b: 48 c1 e1 04                 	shlq	$0x4, %rcx
140929e1f: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
140929e24: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
140929e28: 40 b7 01                    	movb	$0x1, %dil
140929e2b: e9 b8 00 00 00              	jmp	0x140929ee8 <.text+0x928ee8>
140929e30: 49 39 de                    	cmpq	%rbx, %r14
140929e33: 0f 85 a7 00 00 00           	jne	0x140929ee0 <.text+0x928ee0>
140929e39: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140929e40: 4d 39 ec                    	cmpq	%r13, %r12
140929e43: 0f 84 2a 01 00 00           	je	0x140929f73 <.text+0x928f73>
140929e49: 41 8b 0c 24                 	movl	(%r12), %ecx
140929e4d: 49 83 c4 04                 	addq	$0x4, %r12
140929e51: 49 8b 87 00 01 00 00        	movq	0x100(%r15), %rax
140929e58: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
140929e5c: 48 c1 e1 05                 	shlq	$0x5, %rcx
140929e60: 48 8b 5c 08 58              	movq	0x58(%rax,%rcx), %rbx
140929e65: 48 85 db                    	testq	%rbx, %rbx
140929e68: 74 d6                       	je	0x140929e40 <.text+0x928e40>
140929e6a: 48 01 c8                    	addq	%rcx, %rax
140929e6d: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140929e71: 48 8b 89 18 01 00 00        	movq	0x118(%rcx), %rcx
140929e78: 48 39 88 88 00 00 00        	cmpq	%rcx, 0x88(%rax)
140929e7f: 0f 86 85 00 00 00           	jbe	0x140929f0a <.text+0x928f0a>
140929e85: 48 8b 90 80 00 00 00        	movq	0x80(%rax), %rdx
140929e8c: 48 83 3c ca 00              	cmpq	$0x0, (%rdx,%rcx,8)
140929e91: 74 77                       	je	0x140929f0a <.text+0x928f0a>
140929e93: 44 8b 80 94 00 00 00        	movl	0x94(%rax), %r8d
140929e9a: 49 8b 97 a8 01 00 00        	movq	0x1a8(%r15), %rdx
140929ea1: 4f 8d 04 c0                 	leaq	(%r8,%r8,8), %r8
140929ea5: 40 b7 01                    	movb	$0x1, %dil
140929ea8: 4a 39 4c c2 40              	cmpq	%rcx, 0x40(%rdx,%r8,8)
140929ead: 76 5f                       	jbe	0x140929f0e <.text+0x928f0e>
140929eaf: 4e 8b 4c c2 38              	movq	0x38(%rdx,%r8,8), %r9
140929eb4: 49 8b 0c c9                 	movq	(%r9,%rcx,8), %rcx
140929eb8: 48 85 c9                    	testq	%rcx, %rcx
140929ebb: 74 51                       	je	0x140929f0e <.text+0x928f0e>
140929ebd: 4a 8d 14 c2                 	leaq	(%rdx,%r8,8), %rdx
140929ec1: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
140929ec5: 48 f7 d1                    	notq	%rcx
140929ec8: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140929ecc: 48 c1 e1 04                 	shlq	$0x4, %rcx
140929ed0: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
140929ed5: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
140929ed9: eb 3b                       	jmp	0x140929f16 <.text+0x928f16>
140929edb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140929ee0: 48 8b 45 28                 	movq	0x28(%rbp), %rax
140929ee4: eb 37                       	jmp	0x140929f1d <.text+0x928f1d>
140929ee6: 31 ff                       	xorl	%edi, %edi
140929ee8: 48 8b 40 08                 	movq	0x8(%rax), %rax
140929eec: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140929ef0: 45 31 f6                    	xorl	%r14d, %r14d
140929ef3: 48 8b 45 30                 	movq	0x30(%rbp), %rax
140929ef7: 4a 8b 04 f0                 	movq	(%rax,%r14,8), %rax
140929efb: 40 f6 c7 01                 	testb	$0x1, %dil
140929eff: 0f 84 6b fe ff ff           	je	0x140929d70 <.text+0x928d70>
140929f05: 44 89 f1                    	movl	%r14d, %ecx
140929f08: eb 35                       	jmp	0x140929f3f <.text+0x928f3f>
140929f0a: 31 ff                       	xorl	%edi, %edi
140929f0c: eb 08                       	jmp	0x140929f16 <.text+0x928f16>
140929f0e: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
140929f16: 48 8b 40 50                 	movq	0x50(%rax), %rax
140929f1a: 45 31 f6                    	xorl	%r14d, %r14d
140929f1d: 4c 89 f1                    	movq	%r14, %rcx
140929f20: 48 c1 e1 04                 	shlq	$0x4, %rcx
140929f24: 48 89 c2                    	movq	%rax, %rdx
140929f27: 48 8b 04 08                 	movq	(%rax,%rcx), %rax
140929f2b: 40 f6 c7 01                 	testb	$0x1, %dil
140929f2f: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
140929f33: 0f 84 37 fe ff ff           	je	0x140929d70 <.text+0x928d70>
140929f39: 48 01 d1                    	addq	%rdx, %rcx
140929f3c: 8b 49 08                    	movl	0x8(%rcx), %ecx
140929f3f: 48 8d 14 c9                 	leaq	(%rcx,%rcx,8), %rdx
140929f43: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140929f47: 48 03 4d 40                 	addq	0x40(%rbp), %rcx
140929f4b: 48 01 d1                    	addq	%rdx, %rcx
140929f4e: 49 ff c6                    	incq	%r14
140929f51: 48 85 c9                    	testq	%rcx, %rcx
140929f54: 0f 84 19 fe ff ff           	je	0x140929d73 <.text+0x928d73>
140929f5a: f3 0f 10 41 14              	movss	0x14(%rcx), %xmm0
140929f5f: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
140929f62: 0f 85 3f fe ff ff           	jne	0x140929da7 <.text+0x928da7>
140929f68: 0f 8b 05 fe ff ff           	jnp	0x140929d73 <.text+0x928d73>
140929f6e: e9 34 fe ff ff              	jmp	0x140929da7 <.text+0x928da7>
140929f73: 80 7d 57 00                 	cmpb	$0x0, 0x57(%rbp)
140929f77: 74 3a                       	je	0x140929fb3 <.text+0x928fb3>
140929f79: e8 b2 e1 2e 00              	callq	0x140c18130 <.text+0xc17130>
140929f7e: 48 89 c1                    	movq	%rax, %rcx
140929f81: 48 8b 00                    	movq	(%rax), %rax
140929f84: 48 83 f8 01                 	cmpq	$0x1, %rax
140929f88: 75 5b                       	jne	0x140929fe5 <.text+0x928fe5>
140929f8a: 48 83 c1 08                 	addq	$0x8, %rcx
140929f8e: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140929f92: 75 71                       	jne	0x14092a005 <.text+0x929005>
140929f94: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140929f9b: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
140929f9f: 48 83 c1 08                 	addq	$0x8, %rcx
140929fa3: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
140929fa7: e8 44 d1 2e 00              	callq	0x140c170f0 <.text+0xc160f0>
140929fac: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140929fb0: 48 ff 00                    	incq	(%rax)
140929fb3: 0f 28 75 60                 	movaps	0x60(%rbp), %xmm6
140929fb7: 48 81 c4 f8 00 00 00        	addq	$0xf8, %rsp
140929fbe: 5b                          	popq	%rbx
140929fbf: 5f                          	popq	%rdi
140929fc0: 5e                          	popq	%rsi
140929fc1: 41 5c                       	popq	%r12
140929fc3: 41 5d                       	popq	%r13
140929fc5: 41 5e                       	popq	%r14
140929fc7: 41 5f                       	popq	%r15
140929fc9: 5d                          	popq	%rbp
140929fca: c3                          	retq
140929fcb: 48 85 c0                    	testq	%rax, %rax
140929fce: 75 29                       	jne	0x140929ff9 <.text+0x928ff9>
140929fd0: 31 d2                       	xorl	%edx, %edx
140929fd2: e8 f9 e7 fc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140929fd7: 48 89 c1                    	movq	%rax, %rcx
140929fda: 48 85 c0                    	testq	%rax, %rax
140929fdd: 0f 85 c5 fc ff ff           	jne	0x140929ca8 <.text+0x928ca8>
140929fe3: eb 14                       	jmp	0x140929ff9 <.text+0x928ff9>
140929fe5: 48 85 c0                    	testq	%rax, %rax
140929fe8: 75 0f                       	jne	0x140929ff9 <.text+0x928ff9>
140929fea: 31 d2                       	xorl	%edx, %edx
140929fec: e8 df e7 fc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140929ff1: 48 89 c1                    	movq	%rax, %rcx
140929ff4: 48 85 c0                    	testq	%rax, %rax
140929ff7: 75 95                       	jne	0x140929f8e <.text+0x928f8e>
140929ff9: 48 8d 0d a0 0a 31 02        	leaq	0x2310aa0(%rip), %rcx   # 0x142c3aaa0
14092a000: e8 cb e0 fa 01              	callq	0x1428d80d0 <.text+0x28d70d0>
14092a005: 48 8d 0d 64 0b 31 02        	leaq	0x2310b64(%rip), %rcx   # 0x142c3ab70
14092a00c: e8 3f f2 fa 01              	callq	0x1428d9250 <.text+0x28d8250>
14092a011: cc                          	int3
14092a012: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14092a020: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14092a025: 55                          	pushq	%rbp
14092a026: 41 57                       	pushq	%r15
14092a028: 41 56                       	pushq	%r14
14092a02a: 41 55                       	pushq	%r13
14092a02c: 41 54                       	pushq	%r12
14092a02e: 56                          	pushq	%rsi
14092a02f: 57                          	pushq	%rdi
14092a030: 53                          	pushq	%rbx
14092a031: 48 83 ec 48                 	subq	$0x48, %rsp
14092a035: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14092a03c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
14092a041: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14092a045: 48 ff 00                    	incq	(%rax)
14092a048: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14092a04d: 48 83 c4 48                 	addq	$0x48, %rsp
14092a051: 5b                          	popq	%rbx
14092a052: 5f                          	popq	%rdi
14092a053: 5e                          	popq	%rsi
14092a054: 41 5c                       	popq	%r12
14092a056: 41 5d                       	popq	%r13
14092a058: 41 5e                       	popq	%r14
14092a05a: 41 5f                       	popq	%r15
14092a05c: 5d                          	popq	%rbp
14092a05d: c3                          	retq
14092a05e: 66 90                       	nop
14092a060: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14092a065: 55                          	pushq	%rbp
14092a066: 41 57                       	pushq	%r15
14092a068: 41 56                       	pushq	%r14
14092a06a: 41 55                       	pushq	%r13
14092a06c: 41 54                       	pushq	%r12
14092a06e: 56                          	pushq	%rsi
14092a06f: 57                          	pushq	%rdi
14092a070: 53                          	pushq	%rbx
14092a071: 48 83 ec 48                 	subq	$0x48, %rsp
14092a075: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14092a07c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
14092a081: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14092a085: 48 ff 00                    	incq	(%rax)
14092a088: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14092a08d: 48 83 c4 48                 	addq	$0x48, %rsp
14092a091: 5b                          	popq	%rbx
14092a092: 5f                          	popq	%rdi
14092a093: 5e                          	popq	%rsi
14092a094: 41 5c                       	popq	%r12
14092a096: 41 5d                       	popq	%r13
14092a098: 41 5e                       	popq	%r14
14092a09a: 41 5f                       	popq	%r15
14092a09c: 5d                          	popq	%rbp
14092a09d: c3                          	retq
14092a09e: 66 90                       	nop
14092a0a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14092a0a5: 55                          	pushq	%rbp
14092a0a6: 41 57                       	pushq	%r15
14092a0a8: 41 56                       	pushq	%r14
14092a0aa: 41 55                       	pushq	%r13
14092a0ac: 41 54                       	pushq	%r12
14092a0ae: 56                          	pushq	%rsi
14092a0af: 57                          	pushq	%rdi
14092a0b0: 53                          	pushq	%rbx
14092a0b1: 48 83 ec 48                 	subq	$0x48, %rsp
14092a0b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14092a0bc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
14092a0c1: 80 7d 57 00                 	cmpb	$0x0, 0x57(%rbp)
14092a0c5: 74 1f                       	je	0x14092a0e6 <.text+0x9290e6>
14092a0c7: 48 8d 45 f0                 	leaq	-0x10(%rbp), %rax
14092a0cb: 48 89 45 00                 	movq	%rax, (%rbp)
14092a0cf: 48 89 e8                    	movq	%rbp, %rax
14092a0d2: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
14092a0d6: 48 8d 0d db 45 18 02        	leaq	0x21845db(%rip), %rcx   # 0x142aae6b8
14092a0dd: 48 8d 55 f8                 	leaq	-0x8(%rbp), %rdx
14092a0e1: e8 ea 51 a0 00              	callq	0x14132f2d0 <.text+0x132e2d0>
14092a0e6: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14092a0eb: 48 83 c4 48                 	addq	$0x48, %rsp
14092a0ef: 5b                          	popq	%rbx
14092a0f0: 5f                          	popq	%rdi
14092a0f1: 5e                          	popq	%rsi
14092a0f2: 41 5c                       	popq	%r12
14092a0f4: 41 5d                       	popq	%r13
14092a0f6: 41 5e                       	popq	%r14
14092a0f8: 41 5f                       	popq	%r15
14092a0fa: 5d                          	popq	%rbp
14092a0fb: c3                          	retq
