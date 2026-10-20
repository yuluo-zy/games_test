
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14031ea20: 55                          	pushq	%rbp
14031ea21: 41 57                       	pushq	%r15
14031ea23: 41 56                       	pushq	%r14
14031ea25: 56                          	pushq	%rsi
14031ea26: 57                          	pushq	%rdi
14031ea27: 53                          	pushq	%rbx
14031ea28: 48 83 ec 68                 	subq	$0x68, %rsp
14031ea2c: 48 8d 6c 24 60              	leaq	0x60(%rsp), %rbp
14031ea31: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
14031ea39: 48 89 ce                    	movq	%rcx, %rsi
14031ea3c: e8 bf 7f 8f 00              	callq	0x140c16a00 <.text+0xc15a00>
14031ea41: 84 c0                       	testb	%al, %al
14031ea43: 74 5f                       	je	0x14031eaa4 <.text+0x31daa4>
14031ea45: 48 8d 1d c2 db 69 02        	leaq	0x269dbc2(%rip), %rbx   # 0x1429bc60e
14031ea4c: bf 31 00 00 00              	movl	$0x31, %edi
14031ea51: ba 31 00 00 00              	movl	$0x31, %edx
14031ea56: 48 89 d9                    	movq	%rbx, %rcx
14031ea59: e8 32 c0 ff ff              	callq	0x14031aa90 <.text+0x319a90>
14031ea5e: 48 83 f8 01                 	cmpq	$0x1, %rax
14031ea62: 0f 85 b1 00 00 00           	jne	0x14031eb19 <.text+0x31db19>
14031ea68: 49 89 d1                    	movq	%rdx, %r9
14031ea6b: 48 85 d2                    	testq	%rdx, %rdx
14031ea6e: 74 3d                       	je	0x14031eaad <.text+0x31daad>
14031ea70: 49 83 f9 31                 	cmpq	$0x31, %r9
14031ea74: 73 35                       	jae	0x14031eaab <.text+0x31daab>
14031ea76: 48 8d 05 91 db 69 02        	leaq	0x269db91(%rip), %rax   # 0x1429bc60e
14031ea7d: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
14031ea82: 7f 29                       	jg	0x14031eaad <.text+0x31daad>
14031ea84: 48 8d 05 6d dc 69 02        	leaq	0x269dc6d(%rip), %rax   # 0x1429bc6f8
14031ea8b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14031ea90: 48 8d 0d 77 db 69 02        	leaq	0x269db77(%rip), %rcx   # 0x1429bc60e
14031ea97: ba 31 00 00 00              	movl	$0x31, %edx
14031ea9c: 45 31 c0                    	xorl	%r8d, %r8d
14031ea9f: e8 9c ae 5b 02              	callq	0x1428d9940 <.text+0x28d8940>
14031eaa4: 31 c0                       	xorl	%eax, %eax
14031eaa6: e9 bf 01 00 00              	jmp	0x14031ec6a <.text+0x31dc6a>
14031eaab: 75 d7                       	jne	0x14031ea84 <.text+0x31da84>
14031eaad: 48 8d 1d 5a db 69 02        	leaq	0x269db5a(%rip), %rbx   # 0x1429bc60e
14031eab4: 48 89 d9                    	movq	%rbx, %rcx
14031eab7: 4c 89 ca                    	movq	%r9, %rdx
14031eaba: e8 d1 bf ff ff              	callq	0x14031aa90 <.text+0x319a90>
14031eabf: 48 83 f8 01                 	cmpq	$0x1, %rax
14031eac3: 75 54                       	jne	0x14031eb19 <.text+0x31db19>
14031eac5: 49 89 d0                    	movq	%rdx, %r8
14031eac8: 49 83 c0 02                 	addq	$0x2, %r8
14031eacc: 74 39                       	je	0x14031eb07 <.text+0x31db07>
14031eace: 49 83 f8 31                 	cmpq	$0x31, %r8
14031ead2: 73 31                       	jae	0x14031eb05 <.text+0x31db05>
14031ead4: 48 8d 05 33 db 69 02        	leaq	0x269db33(%rip), %rax   # 0x1429bc60e
14031eadb: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14031eae0: 7f 25                       	jg	0x14031eb07 <.text+0x31db07>
14031eae2: 48 8d 05 27 dc 69 02        	leaq	0x269dc27(%rip), %rax   # 0x1429bc710
14031eae9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14031eaee: 48 8d 0d 19 db 69 02        	leaq	0x269db19(%rip), %rcx   # 0x1429bc60e
14031eaf5: ba 31 00 00 00              	movl	$0x31, %edx
14031eafa: 41 b9 31 00 00 00           	movl	$0x31, %r9d
14031eb00: e8 3b ae 5b 02              	callq	0x1428d9940 <.text+0x28d8940>
14031eb05: 75 db                       	jne	0x14031eae2 <.text+0x31dae2>
14031eb07: bf 2f 00 00 00              	movl	$0x2f, %edi
14031eb0c: 48 29 d7                    	subq	%rdx, %rdi
14031eb0f: 48 8d 1d f8 da 69 02        	leaq	0x269daf8(%rip), %rbx   # 0x1429bc60e
14031eb16: 4c 01 c3                    	addq	%r8, %rbx
14031eb19: 4c 8d 05 43 dc 69 02        	leaq	0x269dc43(%rip), %r8    # 0x1429bc763
14031eb20: 4c 8d 35 19 dc 69 02        	leaq	0x269dc19(%rip), %r14   # 0x1429bc740
14031eb27: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14031eb30: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
14031eb35: 85 c0                       	testl	%eax, %eax
14031eb37: 78 17                       	js	0x14031eb50 <.text+0x31db50>
14031eb39: 49 ff c8                    	decq	%r8
14031eb3c: 83 f8 5c                    	cmpl	$0x5c, %eax
14031eb3f: 75 63                       	jne	0x14031eba4 <.text+0x31dba4>
14031eb41: eb 73                       	jmp	0x14031ebb6 <.text+0x31dbb6>
14031eb43: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14031eb50: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
14031eb55: 80 f9 c0                    	cmpb	$-0x40, %cl
14031eb58: 7d 23                       	jge	0x14031eb7d <.text+0x31db7d>
14031eb5a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
14031eb5f: 80 fa c0                    	cmpb	$-0x40, %dl
14031eb62: 7d 22                       	jge	0x14031eb86 <.text+0x31db86>
14031eb64: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
14031eb69: 49 83 c0 fc                 	addq	$-0x4, %r8
14031eb6d: 41 83 e1 07                 	andl	$0x7, %r9d
14031eb71: 41 c1 e1 06                 	shll	$0x6, %r9d
14031eb75: 83 e2 3f                    	andl	$0x3f, %edx
14031eb78: 44 09 ca                    	orl	%r9d, %edx
14031eb7b: eb 10                       	jmp	0x14031eb8d <.text+0x31db8d>
14031eb7d: 49 83 c0 fe                 	addq	$-0x2, %r8
14031eb81: 83 e1 1f                    	andl	$0x1f, %ecx
14031eb84: eb 0f                       	jmp	0x14031eb95 <.text+0x31db95>
14031eb86: 49 83 c0 fd                 	addq	$-0x3, %r8
14031eb8a: 83 e2 0f                    	andl	$0xf, %edx
14031eb8d: c1 e2 06                    	shll	$0x6, %edx
14031eb90: 83 e1 3f                    	andl	$0x3f, %ecx
14031eb93: 09 d1                       	orl	%edx, %ecx
14031eb95: c1 e1 06                    	shll	$0x6, %ecx
14031eb98: 24 3f                       	andb	$0x3f, %al
14031eb9a: 0f b6 c0                    	movzbl	%al, %eax
14031eb9d: 09 c8                       	orl	%ecx, %eax
14031eb9f: 83 f8 5c                    	cmpl	$0x5c, %eax
14031eba2: 74 12                       	je	0x14031ebb6 <.text+0x31dbb6>
14031eba4: 83 f8 2f                    	cmpl	$0x2f, %eax
14031eba7: 74 0d                       	je	0x14031ebb6 <.text+0x31dbb6>
14031eba9: 4d 39 f0                    	cmpq	%r14, %r8
14031ebac: 75 82                       	jne	0x14031eb30 <.text+0x31db30>
14031ebae: 41 bf 23 00 00 00           	movl	$0x23, %r15d
14031ebb4: eb 4d                       	jmp	0x14031ec03 <.text+0x31dc03>
14031ebb6: 4c 8d 35 83 db 69 02        	leaq	0x269db83(%rip), %r14   # 0x1429bc740
14031ebbd: 4d 29 f0                    	subq	%r14, %r8
14031ebc0: 49 ff c0                    	incq	%r8
14031ebc3: 74 32                       	je	0x14031ebf7 <.text+0x31dbf7>
14031ebc5: 49 83 f8 23                 	cmpq	$0x23, %r8
14031ebc9: 73 2a                       	jae	0x14031ebf5 <.text+0x31dbf5>
14031ebcb: 43 80 3c 30 bf              	cmpb	$-0x41, (%r8,%r14)
14031ebd0: 7f 25                       	jg	0x14031ebf7 <.text+0x31dbf7>
14031ebd2: 48 8d 05 ff da 69 02        	leaq	0x269daff(%rip), %rax   # 0x1429bc6d8
14031ebd9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14031ebde: 48 8d 0d 5b db 69 02        	leaq	0x269db5b(%rip), %rcx   # 0x1429bc740
14031ebe5: ba 23 00 00 00              	movl	$0x23, %edx
14031ebea: 41 b9 23 00 00 00           	movl	$0x23, %r9d
14031ebf0: e8 4b ad 5b 02              	callq	0x1428d9940 <.text+0x28d8940>
14031ebf5: 75 db                       	jne	0x14031ebd2 <.text+0x31dbd2>
14031ebf7: 41 bf 23 00 00 00           	movl	$0x23, %r15d
14031ebfd: 4d 29 c7                    	subq	%r8, %r15
14031ec00: 4d 01 c6                    	addq	%r8, %r14
14031ec03: e8 28 95 8f 00              	callq	0x140c18130 <.text+0xc17130>
14031ec08: 48 89 c1                    	movq	%rax, %rcx
14031ec0b: 48 8b 00                    	movq	(%rax), %rax
14031ec0e: 48 83 f8 01                 	cmpq	$0x1, %rax
14031ec12: 0f 85 79 01 00 00           	jne	0x14031ed91 <.text+0x31dd91>
14031ec18: 48 83 c1 08                 	addq	$0x8, %rcx
14031ec1c: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14031ec20: 0f 85 8f 01 00 00           	jne	0x14031edb5 <.text+0x31ddb5>
14031ec26: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14031ec2d: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
14031ec31: 48 83 c1 08                 	addq	$0x8, %rcx
14031ec35: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
14031ec3a: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14031ec43: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14031ec4c: 48 89 da                    	movq	%rbx, %rdx
14031ec4f: 49 89 f8                    	movq	%rdi, %r8
14031ec52: 4d 89 f1                    	movq	%r14, %r9
14031ec55: e8 e6 80 8f 00              	callq	0x140c16d40 <.text+0xc15d40>
14031ec5a: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
14031ec5e: 48 ff 01                    	incq	(%rcx)
14031ec61: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14031ec65: b8 01 00 00 00              	movl	$0x1, %eax
14031ec6a: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
14031ec6e: b1 01                       	movb	$0x1, %cl
14031ec70: 31 c0                       	xorl	%eax, %eax
14031ec72: f0                          	lock
14031ec73: 0f b0 0d d6 c3 30 03        	cmpxchgb	%cl, 0x330c3d6(%rip)    # 0x14362b050
14031ec7a: 0f 85 41 01 00 00           	jne	0x14031edc1 <.text+0x31ddc1>
14031ec80: 48 8d 0d c9 c3 30 03        	leaq	0x330c3c9(%rip), %rcx   # 0x14362b050
14031ec87: e8 e4 2f 96 00              	callq	0x140c81c70 <.text+0xc80c70>
14031ec8c: 4c 8b 35 cd c3 30 03        	movq	0x330c3cd(%rip), %r14   # 0x14362b060
14031ec93: 48 69 1d ca c3 30 03 98 00 00 00    	imulq	$0x98, 0x330c3ca(%rip), %rbx # 0x14362b068
14031ec9e: 4c 01 f3                    	addq	%r14, %rbx
14031eca1: eb 0d                       	jmp	0x14031ecb0 <.text+0x31dcb0>
14031eca3: 8b 47 54                    	movl	0x54(%rdi), %eax
14031eca6: 85 c0                       	testl	%eax, %eax
14031eca8: 74 46                       	je	0x14031ecf0 <.text+0x31dcf0>
14031ecaa: ff c8                       	decl	%eax
14031ecac: 89 47 54                    	movl	%eax, 0x54(%rdi)
14031ecaf: 90                          	nop
14031ecb0: 49 39 de                    	cmpq	%rbx, %r14
14031ecb3: 74 4e                       	je	0x14031ed03 <.text+0x31dd03>
14031ecb5: 4c 89 f7                    	movq	%r14, %rdi
14031ecb8: 49 81 c6 98 00 00 00        	addq	$0x98, %r14
14031ecbf: 80 bf 96 00 00 00 06        	cmpb	$0x6, 0x96(%rdi)
14031ecc6: 74 e8                       	je	0x14031ecb0 <.text+0x31dcb0>
14031ecc8: 8b 47 50                    	movl	0x50(%rdi), %eax
14031eccb: 83 f8 02                    	cmpl	$0x2, %eax
14031ecce: 74 e0                       	je	0x14031ecb0 <.text+0x31dcb0>
14031ecd0: 83 f8 01                    	cmpl	$0x1, %eax
14031ecd3: 75 ce                       	jne	0x14031eca3 <.text+0x31dca3>
14031ecd5: 48 89 f1                    	movq	%rsi, %rcx
14031ecd8: e8 a3 9c 8d 00              	callq	0x140bf8980 <.text+0xbf7980>
14031ecdd: 48 89 f9                    	movq	%rdi, %rcx
14031ece0: b2 01                       	movb	$0x1, %dl
14031ece2: e8 f9 05 00 00              	callq	0x14031f2e0 <.text+0x31e2e0>
14031ece7: c7 47 50 02 00 00 00        	movl	$0x2, 0x50(%rdi)
14031ecee: eb c0                       	jmp	0x14031ecb0 <.text+0x31dcb0>
14031ecf0: 48 89 f9                    	movq	%rdi, %rcx
14031ecf3: 31 d2                       	xorl	%edx, %edx
14031ecf5: e8 e6 05 00 00              	callq	0x14031f2e0 <.text+0x31e2e0>
14031ecfa: c7 47 50 02 00 00 00        	movl	$0x2, 0x50(%rdi)
14031ed01: eb ad                       	jmp	0x14031ecb0 <.text+0x31dcb0>
14031ed03: 48 8d 0d 4e c3 30 03        	leaq	0x330c34e(%rip), %rcx   # 0x14362b058
14031ed0a: e8 f1 6e f9 ff              	callq	0x1402b5c00 <.text+0x2b4c00>
14031ed0f: 48 8d 0d 3a c3 30 03        	leaq	0x330c33a(%rip), %rcx   # 0x14362b050
14031ed16: e8 f5 30 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031ed1b: 31 c9                       	xorl	%ecx, %ecx
14031ed1d: b0 01                       	movb	$0x1, %al
14031ed1f: f0                          	lock
14031ed20: 0f b0 0d 29 c3 30 03        	cmpxchgb	%cl, 0x330c329(%rip)    # 0x14362b050
14031ed27: 0f 85 ab 00 00 00           	jne	0x14031edd8 <.text+0x31ddd8>
14031ed2d: 48 83 7d e8 00              	cmpq	$0x0, -0x18(%rbp)
14031ed32: 74 3a                       	je	0x14031ed6e <.text+0x31dd6e>
14031ed34: e8 f7 93 8f 00              	callq	0x140c18130 <.text+0xc17130>
14031ed39: 48 89 c1                    	movq	%rax, %rcx
14031ed3c: 48 8b 00                    	movq	(%rax), %rax
14031ed3f: 48 83 f8 01                 	cmpq	$0x1, %rax
14031ed43: 75 36                       	jne	0x14031ed7b <.text+0x31dd7b>
14031ed45: 48 83 c1 08                 	addq	$0x8, %rcx
14031ed49: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14031ed4d: 75 66                       	jne	0x14031edb5 <.text+0x31ddb5>
14031ed4f: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14031ed56: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
14031ed5a: 48 83 c1 08                 	addq	$0x8, %rcx
14031ed5e: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
14031ed62: e8 89 83 8f 00              	callq	0x140c170f0 <.text+0xc160f0>
14031ed67: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14031ed6b: 48 ff 00                    	incq	(%rax)
14031ed6e: 48 83 c4 68                 	addq	$0x68, %rsp
14031ed72: 5b                          	popq	%rbx
14031ed73: 5f                          	popq	%rdi
14031ed74: 5e                          	popq	%rsi
14031ed75: 41 5e                       	popq	%r14
14031ed77: 41 5f                       	popq	%r15
14031ed79: 5d                          	popq	%rbp
14031ed7a: c3                          	retq
14031ed7b: 48 85 c0                    	testq	%rax, %rax
14031ed7e: 75 29                       	jne	0x14031eda9 <.text+0x31dda9>
14031ed80: 31 d2                       	xorl	%edx, %edx
14031ed82: e8 b9 28 58 02              	callq	0x1428a1640 <.text+0x28a0640>
14031ed87: 48 89 c1                    	movq	%rax, %rcx
14031ed8a: 48 85 c0                    	testq	%rax, %rax
14031ed8d: 75 ba                       	jne	0x14031ed49 <.text+0x31dd49>
14031ed8f: eb 18                       	jmp	0x14031eda9 <.text+0x31dda9>
14031ed91: 48 85 c0                    	testq	%rax, %rax
14031ed94: 75 13                       	jne	0x14031eda9 <.text+0x31dda9>
14031ed96: 31 d2                       	xorl	%edx, %edx
14031ed98: e8 a3 28 58 02              	callq	0x1428a1640 <.text+0x28a0640>
14031ed9d: 48 89 c1                    	movq	%rax, %rcx
14031eda0: 48 85 c0                    	testq	%rax, %rax
14031eda3: 0f 85 73 fe ff ff           	jne	0x14031ec1c <.text+0x31dc1c>
14031eda9: 48 8d 0d 60 3c 61 02        	leaq	0x2613c60(%rip), %rcx   # 0x142932a10
14031edb0: e8 1b 93 5b 02              	callq	0x1428d80d0 <.text+0x28d70d0>
14031edb5: 48 8d 0d fc 3c 61 02        	leaq	0x2613cfc(%rip), %rcx   # 0x142932ab8
14031edbc: e8 8f a4 5b 02              	callq	0x1428d9250 <.text+0x28d8250>
14031edc1: 48 8d 0d 88 c2 30 03        	leaq	0x330c288(%rip), %rcx   # 0x14362b050
14031edc8: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
14031edce: e8 3d e4 5a 02              	callq	0x1428cd210 <.text+0x28cc210>
14031edd3: e9 a8 fe ff ff              	jmp	0x14031ec80 <.text+0x31dc80>
14031edd8: 48 8d 0d 71 c2 30 03        	leaq	0x330c271(%rip), %rcx   # 0x14362b050
14031eddf: 31 d2                       	xorl	%edx, %edx
14031ede1: e8 aa ed 5a 02              	callq	0x1428cdb90 <.text+0x28ccb90>
14031ede6: e9 42 ff ff ff              	jmp	0x14031ed2d <.text+0x31dd2d>
14031edeb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14031edf0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031edf5: 55                          	pushq	%rbp
14031edf6: 41 57                       	pushq	%r15
14031edf8: 41 56                       	pushq	%r14
14031edfa: 56                          	pushq	%rsi
14031edfb: 57                          	pushq	%rdi
14031edfc: 53                          	pushq	%rbx
14031edfd: 48 83 ec 38                 	subq	$0x38, %rsp
14031ee01: 48 8d 6a 60                 	leaq	0x60(%rdx), %rbp
14031ee05: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14031ee09: 48 ff 00                    	incq	(%rax)
14031ee0c: 48 83 c4 38                 	addq	$0x38, %rsp
14031ee10: 5b                          	popq	%rbx
14031ee11: 5f                          	popq	%rdi
14031ee12: 5e                          	popq	%rsi
14031ee13: 41 5e                       	popq	%r14
14031ee15: 41 5f                       	popq	%r15
14031ee17: 5d                          	popq	%rbp
14031ee18: c3                          	retq
14031ee19: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14031ee20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031ee25: 55                          	pushq	%rbp
14031ee26: 41 57                       	pushq	%r15
14031ee28: 41 56                       	pushq	%r14
14031ee2a: 56                          	pushq	%rsi
14031ee2b: 57                          	pushq	%rdi
14031ee2c: 53                          	pushq	%rbx
14031ee2d: 48 83 ec 38                 	subq	$0x38, %rsp
14031ee31: 48 8d 6a 60                 	leaq	0x60(%rdx), %rbp
14031ee35: 48 83 7d e8 00              	cmpq	$0x0, -0x18(%rbp)
14031ee3a: 74 20                       	je	0x14031ee5c <.text+0x31de5c>
14031ee3c: 48 8d 45 f0                 	leaq	-0x10(%rbp), %rax
14031ee40: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
14031ee44: 48 8d 45 e0                 	leaq	-0x20(%rbp), %rax
14031ee48: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
14031ee4c: 48 8d 0d 25 d8 69 02        	leaq	0x269d825(%rip), %rcx   # 0x1429bc678
14031ee53: 48 8d 55 d8                 	leaq	-0x28(%rbp), %rdx
14031ee57: e8 24 37 d8 ff              	callq	0x1400a2580 <.text+0xa1580>
14031ee5c: 90                          	nop
14031ee5d: 48 83 c4 38                 	addq	$0x38, %rsp
14031ee61: 5b                          	popq	%rbx
14031ee62: 5f                          	popq	%rdi
14031ee63: 5e                          	popq	%rsi
14031ee64: 41 5e                       	popq	%r14
14031ee66: 41 5f                       	popq	%r15
14031ee68: 5d                          	popq	%rbp
14031ee69: c3                          	retq
14031ee6a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14031ee70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031ee75: 55                          	pushq	%rbp
14031ee76: 41 57                       	pushq	%r15
14031ee78: 41 56                       	pushq	%r14
14031ee7a: 56                          	pushq	%rsi
14031ee7b: 57                          	pushq	%rdi
14031ee7c: 53                          	pushq	%rbx
14031ee7d: 48 83 ec 38                 	subq	$0x38, %rsp
14031ee81: 48 8d 6a 60                 	leaq	0x60(%rdx), %rbp
14031ee85: 48 8d 0d c4 c1 30 03        	leaq	0x330c1c4(%rip), %rcx   # 0x14362b050
14031ee8c: e8 9f 2d e4 ff              	callq	0x140161c30 <.text+0x160c30>
14031ee91: 90                          	nop
14031ee92: 48 83 c4 38                 	addq	$0x38, %rsp
14031ee96: 5b                          	popq	%rbx
14031ee97: 5f                          	popq	%rdi
14031ee98: 5e                          	popq	%rsi
14031ee99: 41 5e                       	popq	%r14
14031ee9b: 41 5f                       	popq	%r15
14031ee9d: 5d                          	popq	%rbp
14031ee9e: c3                          	retq
14031ee9f: 90                          	nop
14031eea0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031eea5: 55                          	pushq	%rbp
14031eea6: 41 57                       	pushq	%r15
14031eea8: 41 56                       	pushq	%r14
14031eeaa: 56                          	pushq	%rsi
14031eeab: 57                          	pushq	%rdi
14031eeac: 53                          	pushq	%rbx
14031eead: 48 83 ec 38                 	subq	$0x38, %rsp
14031eeb1: 48 8d 6a 60                 	leaq	0x60(%rdx), %rbp
14031eeb5: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14031eeb9: 48 ff 00                    	incq	(%rax)
14031eebc: 48 83 c4 38                 	addq	$0x38, %rsp
14031eec0: 5b                          	popq	%rbx
14031eec1: 5f                          	popq	%rdi
14031eec2: 5e                          	popq	%rsi
14031eec3: 41 5e                       	popq	%r14
14031eec5: 41 5f                       	popq	%r15
14031eec7: 5d                          	popq	%rbp
14031eec8: c3                          	retq
14031eec9: cc                          	int3
14031eeca: cc                          	int3
14031eecb: cc                          	int3
14031eecc: cc                          	int3
14031eecd: cc                          	int3
14031eece: cc                          	int3
14031eecf: cc                          	int3
