
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b2fc00: 55                          	pushq	%rbp
140b2fc01: 41 57                       	pushq	%r15
140b2fc03: 41 56                       	pushq	%r14
140b2fc05: 41 55                       	pushq	%r13
140b2fc07: 41 54                       	pushq	%r12
140b2fc09: 56                          	pushq	%rsi
140b2fc0a: 57                          	pushq	%rdi
140b2fc0b: 53                          	pushq	%rbx
140b2fc0c: 48 81 ec c8 01 00 00        	subq	$0x1c8, %rsp            # imm = 0x1C8
140b2fc13: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b2fc1b: 44 0f 29 8d 30 01 00 00     	movaps	%xmm9, 0x130(%rbp)
140b2fc23: 44 0f 29 85 20 01 00 00     	movaps	%xmm8, 0x120(%rbp)
140b2fc2b: 0f 29 bd 10 01 00 00        	movaps	%xmm7, 0x110(%rbp)
140b2fc32: 66 0f 7f b5 00 01 00 00     	movdqa	%xmm6, 0x100(%rbp)
140b2fc3a: 48 c7 85 f8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xf8(%rbp)
140b2fc45: 4c 89 ce                    	movq	%r9, %rsi
140b2fc48: 4c 89 45 f8                 	movq	%r8, -0x8(%rbp)
140b2fc4c: 49 89 d6                    	movq	%rdx, %r14
140b2fc4f: 48 89 cb                    	movq	%rcx, %rbx
140b2fc52: e8 a9 6d 0e 00              	callq	0x140c16a00 <.text+0xc15a00>
140b2fc57: 84 c0                       	testb	%al, %al
140b2fc59: 88 85 f7 00 00 00           	movb	%al, 0xf7(%rbp)
140b2fc5f: 74 5f                       	je	0x140b2fcc0 <.text+0xb2ecc0>
140b2fc61: 4c 8d 3d 6c b6 fb 01        	leaq	0x1fbb66c(%rip), %r15   # 0x142aeb2d4
140b2fc68: bf 5d 00 00 00              	movl	$0x5d, %edi
140b2fc6d: ba 5d 00 00 00              	movl	$0x5d, %edx
140b2fc72: 4c 89 f9                    	movq	%r15, %rcx
140b2fc75: e8 f6 b1 ff ff              	callq	0x140b2ae70 <.text+0xb29e70>
140b2fc7a: 48 83 f8 01                 	cmpq	$0x1, %rax
140b2fc7e: 0f 85 b1 00 00 00           	jne	0x140b2fd35 <.text+0xb2ed35>
140b2fc84: 49 89 d1                    	movq	%rdx, %r9
140b2fc87: 48 85 d2                    	testq	%rdx, %rdx
140b2fc8a: 74 3d                       	je	0x140b2fcc9 <.text+0xb2ecc9>
140b2fc8c: 49 83 f9 5d                 	cmpq	$0x5d, %r9
140b2fc90: 73 35                       	jae	0x140b2fcc7 <.text+0xb2ecc7>
140b2fc92: 48 8d 05 3b b6 fb 01        	leaq	0x1fbb63b(%rip), %rax   # 0x142aeb2d4
140b2fc99: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
140b2fc9e: 7f 29                       	jg	0x140b2fcc9 <.text+0xb2ecc9>
140b2fca0: 48 8d 05 41 b7 fb 01        	leaq	0x1fbb741(%rip), %rax   # 0x142aeb3e8
140b2fca7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b2fcac: 48 8d 0d 21 b6 fb 01        	leaq	0x1fbb621(%rip), %rcx   # 0x142aeb2d4
140b2fcb3: ba 5d 00 00 00              	movl	$0x5d, %edx
140b2fcb8: 45 31 c0                    	xorl	%r8d, %r8d
140b2fcbb: e8 80 9c da 01              	callq	0x1428d9940 <.text+0x28d8940>
140b2fcc0: 31 c0                       	xorl	%eax, %eax
140b2fcc2: e9 dc 01 00 00              	jmp	0x140b2fea3 <.text+0xb2eea3>
140b2fcc7: 75 d7                       	jne	0x140b2fca0 <.text+0xb2eca0>
140b2fcc9: 4c 8d 3d 04 b6 fb 01        	leaq	0x1fbb604(%rip), %r15   # 0x142aeb2d4
140b2fcd0: 4c 89 f9                    	movq	%r15, %rcx
140b2fcd3: 4c 89 ca                    	movq	%r9, %rdx
140b2fcd6: e8 95 b1 ff ff              	callq	0x140b2ae70 <.text+0xb29e70>
140b2fcdb: 48 83 f8 01                 	cmpq	$0x1, %rax
140b2fcdf: 75 54                       	jne	0x140b2fd35 <.text+0xb2ed35>
140b2fce1: 49 89 d0                    	movq	%rdx, %r8
140b2fce4: 49 83 c0 02                 	addq	$0x2, %r8
140b2fce8: 74 39                       	je	0x140b2fd23 <.text+0xb2ed23>
140b2fcea: 49 83 f8 5d                 	cmpq	$0x5d, %r8
140b2fcee: 73 31                       	jae	0x140b2fd21 <.text+0xb2ed21>
140b2fcf0: 48 8d 05 dd b5 fb 01        	leaq	0x1fbb5dd(%rip), %rax   # 0x142aeb2d4
140b2fcf7: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140b2fcfc: 7f 25                       	jg	0x140b2fd23 <.text+0xb2ed23>
140b2fcfe: 48 8d 05 fb b6 fb 01        	leaq	0x1fbb6fb(%rip), %rax   # 0x142aeb400
140b2fd05: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b2fd0a: 48 8d 0d c3 b5 fb 01        	leaq	0x1fbb5c3(%rip), %rcx   # 0x142aeb2d4
140b2fd11: ba 5d 00 00 00              	movl	$0x5d, %edx
140b2fd16: 41 b9 5d 00 00 00           	movl	$0x5d, %r9d
140b2fd1c: e8 1f 9c da 01              	callq	0x1428d9940 <.text+0x28d8940>
140b2fd21: 75 db                       	jne	0x140b2fcfe <.text+0xb2ecfe>
140b2fd23: bf 5b 00 00 00              	movl	$0x5b, %edi
140b2fd28: 48 29 d7                    	subq	%rdx, %rdi
140b2fd2b: 4c 8d 3d a2 b5 fb 01        	leaq	0x1fbb5a2(%rip), %r15   # 0x142aeb2d4
140b2fd32: 4d 01 c7                    	addq	%r8, %r15
140b2fd35: 48 8d 05 ec bb fb 01        	leaq	0x1fbbbec(%rip), %rax   # 0x142aeb928
140b2fd3c: 4c 8d 25 95 bb fb 01        	leaq	0x1fbbb95(%rip), %r12   # 0x142aeb8d8
140b2fd43: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140b2fd50: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
140b2fd54: 85 c9                       	testl	%ecx, %ecx
140b2fd56: 78 18                       	js	0x140b2fd70 <.text+0xb2ed70>
140b2fd58: 48 ff c8                    	decq	%rax
140b2fd5b: 83 f9 5c                    	cmpl	$0x5c, %ecx
140b2fd5e: 75 69                       	jne	0x140b2fdc9 <.text+0xb2edc9>
140b2fd60: eb 7d                       	jmp	0x140b2fddf <.text+0xb2eddf>
140b2fd62: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140b2fd70: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
140b2fd74: 80 fa c0                    	cmpb	$-0x40, %dl
140b2fd77: 7d 25                       	jge	0x140b2fd9e <.text+0xb2ed9e>
140b2fd79: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
140b2fd7e: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
140b2fd82: 7d 23                       	jge	0x140b2fda7 <.text+0xb2eda7>
140b2fd84: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
140b2fd89: 48 83 c0 fc                 	addq	$-0x4, %rax
140b2fd8d: 41 83 e1 07                 	andl	$0x7, %r9d
140b2fd91: 41 c1 e1 06                 	shll	$0x6, %r9d
140b2fd95: 41 83 e0 3f                 	andl	$0x3f, %r8d
140b2fd99: 45 09 c8                    	orl	%r9d, %r8d
140b2fd9c: eb 11                       	jmp	0x140b2fdaf <.text+0xb2edaf>
140b2fd9e: 48 83 c0 fe                 	addq	$-0x2, %rax
140b2fda2: 83 e2 1f                    	andl	$0x1f, %edx
140b2fda5: eb 12                       	jmp	0x140b2fdb9 <.text+0xb2edb9>
140b2fda7: 48 83 c0 fd                 	addq	$-0x3, %rax
140b2fdab: 41 83 e0 0f                 	andl	$0xf, %r8d
140b2fdaf: 41 c1 e0 06                 	shll	$0x6, %r8d
140b2fdb3: 83 e2 3f                    	andl	$0x3f, %edx
140b2fdb6: 44 09 c2                    	orl	%r8d, %edx
140b2fdb9: c1 e2 06                    	shll	$0x6, %edx
140b2fdbc: 80 e1 3f                    	andb	$0x3f, %cl
140b2fdbf: 0f b6 c9                    	movzbl	%cl, %ecx
140b2fdc2: 09 d1                       	orl	%edx, %ecx
140b2fdc4: 83 f9 5c                    	cmpl	$0x5c, %ecx
140b2fdc7: 74 16                       	je	0x140b2fddf <.text+0xb2eddf>
140b2fdc9: 83 f9 2f                    	cmpl	$0x2f, %ecx
140b2fdcc: 74 11                       	je	0x140b2fddf <.text+0xb2eddf>
140b2fdce: 4c 39 e0                    	cmpq	%r12, %rax
140b2fdd1: 0f 85 79 ff ff ff           	jne	0x140b2fd50 <.text+0xb2ed50>
140b2fdd7: 41 bd 50 00 00 00           	movl	$0x50, %r13d
140b2fddd: eb 50                       	jmp	0x140b2fe2f <.text+0xb2ee2f>
140b2fddf: 4c 8d 25 f2 ba fb 01        	leaq	0x1fbbaf2(%rip), %r12   # 0x142aeb8d8
140b2fde6: 4c 29 e0                    	subq	%r12, %rax
140b2fde9: 49 89 c0                    	movq	%rax, %r8
140b2fdec: 49 ff c0                    	incq	%r8
140b2fdef: 74 32                       	je	0x140b2fe23 <.text+0xb2ee23>
140b2fdf1: 49 83 f8 50                 	cmpq	$0x50, %r8
140b2fdf5: 73 2a                       	jae	0x140b2fe21 <.text+0xb2ee21>
140b2fdf7: 43 80 3c 20 bf              	cmpb	$-0x41, (%r8,%r12)
140b2fdfc: 7f 25                       	jg	0x140b2fe23 <.text+0xb2ee23>
140b2fdfe: 48 8d 05 c3 b5 fb 01        	leaq	0x1fbb5c3(%rip), %rax   # 0x142aeb3c8
140b2fe05: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b2fe0a: 48 8d 0d c7 ba fb 01        	leaq	0x1fbbac7(%rip), %rcx   # 0x142aeb8d8
140b2fe11: ba 50 00 00 00              	movl	$0x50, %edx
140b2fe16: 41 b9 50 00 00 00           	movl	$0x50, %r9d
140b2fe1c: e8 1f 9b da 01              	callq	0x1428d9940 <.text+0x28d8940>
140b2fe21: 75 db                       	jne	0x140b2fdfe <.text+0xb2edfe>
140b2fe23: 41 bd 4f 00 00 00           	movl	$0x4f, %r13d
140b2fe29: 49 29 c5                    	subq	%rax, %r13
140b2fe2c: 4d 01 c4                    	addq	%r8, %r12
140b2fe2f: e8 fc 82 0e 00              	callq	0x140c18130 <.text+0xc17130>
140b2fe34: 48 89 c1                    	movq	%rax, %rcx
140b2fe37: 48 8b 00                    	movq	(%rax), %rax
140b2fe3a: 48 83 f8 01                 	cmpq	$0x1, %rax
140b2fe3e: 0f 85 a9 08 00 00           	jne	0x140b306ed <.text+0xb2f6ed>
140b2fe44: 48 83 c1 08                 	addq	$0x8, %rcx
140b2fe48: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140b2fe4c: 0f 85 38 09 00 00           	jne	0x140b3078a <.text+0xb2f78a>
140b2fe52: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140b2fe59: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
140b2fe60: 48 83 c1 08                 	addq	$0x8, %rcx
140b2fe64: 4c 89 6c 24 20              	movq	%r13, 0x20(%rsp)
140b2fe69: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140b2fe72: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140b2fe7b: 4c 89 fa                    	movq	%r15, %rdx
140b2fe7e: 49 89 f8                    	movq	%rdi, %r8
140b2fe81: 4d 89 e1                    	movq	%r12, %r9
140b2fe84: e8 b7 6e 0e 00              	callq	0x140c16d40 <.text+0xc15d40>
140b2fe89: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140b2fe90: 48 ff 01                    	incq	(%rcx)
140b2fe93: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
140b2fe9a: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
140b2fe9e: b8 01 00 00 00              	movl	$0x1, %eax
140b2fea3: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140b2fea7: 48 8b 03                    	movq	(%rbx), %rax
140b2feaa: 8b 08                       	movl	(%rax), %ecx
140b2feac: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
140b2feb0: 83 f9 05                    	cmpl	$0x5, %ecx
140b2feb3: 0f 87 9d 07 00 00           	ja	0x140b30656 <.text+0xb2f656>
140b2feb9: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
140b2fec0: 4c 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %r9
140b2fec7: 48 8b 95 d0 01 00 00        	movq	0x1d0(%rbp), %rdx
140b2fece: 4d 8b 01                    	movq	(%r9), %r8
140b2fed1: 49 8b 79 08                 	movq	0x8(%r9), %rdi
140b2fed5: 4d 8b 08                    	movq	(%r8), %r9
140b2fed8: 45 31 db                    	xorl	%r11d, %r11d
140b2fedb: 4c 89 cb                    	movq	%r9, %rbx
140b2fede: 48 2b 5f 18                 	subq	0x18(%rdi), %rbx
140b2fee2: 49 0f 42 db                 	cmovbq	%r11, %rbx
140b2fee6: 4c 2b 4f 38                 	subq	0x38(%rdi), %r9
140b2feea: 4d 0f 42 cb                 	cmovbq	%r11, %r9
140b2feee: 4c 8b 95 c8 01 00 00        	movq	0x1c8(%rbp), %r10
140b2fef5: 4c 8b 7f 10                 	movq	0x10(%rdi), %r15
140b2fef9: 4c 8b 67 30                 	movq	0x30(%rdi), %r12
140b2fefd: 4d 29 cc                    	subq	%r9, %r12
140b2ff00: 4d 0f 42 e3                 	cmovbq	%r11, %r12
140b2ff04: 41 0f 97 c5                 	seta	%r13b
140b2ff08: 49 29 df                    	subq	%rbx, %r15
140b2ff0b: 4c 8b 8d b0 01 00 00        	movq	0x1b0(%rbp), %r9
140b2ff12: 4d 0f 42 fb                 	cmovbq	%r11, %r15
140b2ff16: 48 8b 40 08                 	movq	0x8(%rax), %rax
140b2ff1a: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140b2ff1e: 48 8b 47 40                 	movq	0x40(%rdi), %rax
140b2ff22: 0f 97 c3                    	seta	%bl
140b2ff25: 4d 01 e7                    	addq	%r12, %r15
140b2ff28: 44 08 eb                    	orb	%r13b, %bl
140b2ff2b: 4d 0f 45 fb                 	cmovneq	%r11, %r15
140b2ff2f: 4c 29 f8                    	subq	%r15, %rax
140b2ff32: 49 89 00                    	movq	%rax, (%r8)
140b2ff35: 4d 8b 36                    	movq	(%r14), %r14
140b2ff38: 4d 8b 26                    	movq	(%r14), %r12
140b2ff3b: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
140b2ff41: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140b2ff45: f7 d0                       	notl	%eax
140b2ff47: 4d 8b 02                    	movq	(%r10), %r8
140b2ff4a: 4c 89 45 08                 	movq	%r8, 0x8(%rbp)
140b2ff4e: 4d 8b 42 08                 	movq	0x8(%r10), %r8
140b2ff52: 4c 89 45 00                 	movq	%r8, (%rbp)
140b2ff56: 4c 8b 06                    	movq	(%rsi), %r8
140b2ff59: 4c 89 45 18                 	movq	%r8, 0x18(%rbp)
140b2ff5d: 88 9d ee 00 00 00           	movb	%bl, 0xee(%rbp)
140b2ff63: 80 f3 01                    	xorb	$0x1, %bl
140b2ff66: 88 9d ef 00 00 00           	movb	%bl, 0xef(%rbp)
140b2ff6c: 48 8b 09                    	movq	(%rcx), %rcx
140b2ff6f: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
140b2ff73: 48 8b 0a                    	movq	(%rdx), %rcx
140b2ff76: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
140b2ff7a: 41 8b 49 1c                 	movl	0x1c(%r9), %ecx
140b2ff7e: 89 8d a0 00 00 00           	movl	%ecx, 0xa0(%rbp)
140b2ff84: 49 8b 11                    	movq	(%r9), %rdx
140b2ff87: 49 8b 49 10                 	movq	0x10(%r9), %rcx
140b2ff8b: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
140b2ff8f: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140b2ff96: 44 8b 41 1c                 	movl	0x1c(%rcx), %r8d
140b2ff9a: 44 89 85 cc 00 00 00        	movl	%r8d, 0xcc(%rbp)
140b2ffa1: 4c 8b 01                    	movq	(%rcx), %r8
140b2ffa4: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
140b2ffa8: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
140b2ffac: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
140b2ffb3: 44 8b 49 1c                 	movl	0x1c(%rcx), %r9d
140b2ffb7: 44 89 8d a4 00 00 00        	movl	%r9d, 0xa4(%rbp)
140b2ffbe: 4c 8b 09                    	movq	(%rcx), %r9
140b2ffc1: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
140b2ffc5: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
140b2ffc9: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
140b2ffcd: 0f 57 ff                    	xorps	%xmm7, %xmm7
140b2ffd0: f3 44 0f 10 05 ab 59 df 01  	movss	0x1df59ab(%rip), %xmm8  # 0x142925984
140b2ffd9: f3 44 0f 10 0d 22 59 df 01  	movss	0x1df5922(%rip), %xmm9  # 0x142925904
140b2ffe2: 49 8b 4e 18                 	movq	0x18(%r14), %rcx
140b2ffe6: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
140b2ffed: 4d 8d 6c 24 10              	leaq	0x10(%r12), %r13
140b2fff2: 49 8d 4e 50                 	leaq	0x50(%r14), %rcx
140b2fff6: 48 89 4d f0                 	movq	%rcx, -0x10(%rbp)
140b2fffa: 49 8d 4e 20                 	leaq	0x20(%r14), %rcx
140b2fffe: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
140b30002: 48 8d 4a 40                 	leaq	0x40(%rdx), %rcx
140b30006: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
140b3000a: 48 8d 4a 20                 	leaq	0x20(%rdx), %rcx
140b3000e: 48 89 8d 90 00 00 00        	movq	%rcx, 0x90(%rbp)
140b30015: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
140b30019: 48 8d 4a 30                 	leaq	0x30(%rdx), %rcx
140b3001d: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
140b30021: 49 8d 48 40                 	leaq	0x40(%r8), %rcx
140b30025: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
140b3002c: 49 8d 48 20                 	leaq	0x20(%r8), %rcx
140b30030: 48 89 8d c0 00 00 00        	movq	%rcx, 0xc0(%rbp)
140b30037: 4c 89 45 60                 	movq	%r8, 0x60(%rbp)
140b3003b: 49 8d 48 30                 	leaq	0x30(%r8), %rcx
140b3003f: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
140b30046: 49 8d 49 40                 	leaq	0x40(%r9), %rcx
140b3004a: 48 89 8d 80 00 00 00        	movq	%rcx, 0x80(%rbp)
140b30051: 49 8d 49 20                 	leaq	0x20(%r9), %rcx
140b30055: 48 89 8d 98 00 00 00        	movq	%rcx, 0x98(%rbp)
140b3005c: 4c 89 4d 48                 	movq	%r9, 0x48(%rbp)
140b30060: 49 8d 49 30                 	leaq	0x30(%r9), %rcx
140b30064: 48 89 8d 88 00 00 00        	movq	%rcx, 0x88(%rbp)
140b3006b: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
140b30072: 48 85 d2                    	testq	%rdx, %rdx
140b30075: 0f 84 db 05 00 00           	je	0x140b30656 <.text+0xb2f656>
140b3007b: 66 85 c0                    	testw	%ax, %ax
140b3007e: 74 10                       	je	0x140b30090 <.text+0xb2f090>
140b30080: 8d 48 ff                    	leal	-0x1(%rax), %ecx
140b30083: 21 c1                       	andl	%eax, %ecx
140b30085: 89 8d f0 00 00 00           	movl	%ecx, 0xf0(%rbp)
140b3008b: eb 35                       	jmp	0x140b300c2 <.text+0xb2f0c2>
140b3008d: 0f 1f 00                    	nopl	(%rax)
140b30090: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
140b30096: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140b3009a: 49 81 c4 80 d5 ff ff        	addq	$-0x2a80, %r12          # imm = 0xD580
140b300a1: 49 83 c5 10                 	addq	$0x10, %r13
140b300a5: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140b300aa: 74 e4                       	je	0x140b30090 <.text+0xb2f090>
140b300ac: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140b300b1: 29 c1                       	subl	%eax, %ecx
140b300b3: f7 d0                       	notl	%eax
140b300b5: 41 89 c0                    	movl	%eax, %r8d
140b300b8: 41 21 c8                    	andl	%ecx, %r8d
140b300bb: 44 89 85 f0 00 00 00        	movl	%r8d, 0xf0(%rbp)
140b300c2: 48 ff ca                    	decq	%rdx
140b300c5: 48 89 95 e0 00 00 00        	movq	%rdx, 0xe0(%rbp)
140b300cc: f3 0f bc c0                 	tzcntl	%eax, %eax
140b300d0: 48 f7 d8                    	negq	%rax
140b300d3: 48 69 c0 a8 02 00 00        	imulq	$0x2a8, %rax, %rax      # imm = 0x2A8
140b300da: 49 8b b4 04 58 fd ff ff     	movq	-0x2a8(%r12,%rax), %rsi
140b300e2: 48 89 b5 d0 00 00 00        	movq	%rsi, 0xd0(%rbp)
140b300e9: 49 83 7e 48 00              	cmpq	$0x0, 0x48(%r14)
140b300ee: 0f 84 9c 01 00 00           	je	0x140b30290 <.text+0xb2f290>
140b300f4: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
140b300f8: 48 8d 95 d0 00 00 00        	leaq	0xd0(%rbp), %rdx
140b300ff: e8 6c 0c a8 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140b30104: 49 8b 4e 30                 	movq	0x30(%r14), %rcx
140b30108: 49 8b 56 38                 	movq	0x38(%r14), %rdx
140b3010c: 49 89 d0                    	movq	%rdx, %r8
140b3010f: 49 21 c0                    	andq	%rax, %r8
140b30112: 48 c1 e8 39                 	shrq	$0x39, %rax
140b30116: 66 0f 6e c0                 	movd	%eax, %xmm0
140b3011a: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b3011e: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b30123: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140b30128: 48 8d 41 f0                 	leaq	-0x10(%rcx), %rax
140b3012c: 45 31 c9                    	xorl	%r9d, %r9d
140b3012f: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140b30135: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140b30139: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140b3013d: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140b30142: 45 85 d2                    	testl	%r10d, %r10d
140b30145: 74 39                       	je	0x140b30180 <.text+0xb2f180>
140b30147: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140b3014c: 4d 01 c3                    	addq	%r8, %r11
140b3014f: 49 21 d3                    	andq	%rdx, %r11
140b30152: 4c 89 df                    	movq	%r11, %rdi
140b30155: 48 c1 e7 04                 	shlq	$0x4, %rdi
140b30159: 48 89 c3                    	movq	%rax, %rbx
140b3015c: 48 29 fb                    	subq	%rdi, %rbx
140b3015f: 48 39 33                    	cmpq	%rsi, (%rbx)
140b30162: 74 4c                       	je	0x140b301b0 <.text+0xb2f1b0>
140b30164: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140b30168: 66 45 21 d3                 	andw	%r10w, %r11w
140b3016c: 45 89 da                    	movl	%r11d, %r10d
140b3016f: 75 d6                       	jne	0x140b30147 <.text+0xb2f147>
140b30171: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140b30180: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140b30184: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140b30189: 45 85 d2                    	testl	%r10d, %r10d
140b3018c: 0f 85 fe 00 00 00           	jne	0x140b30290 <.text+0xb2f290>
140b30192: 4d 01 c8                    	addq	%r9, %r8
140b30195: 49 83 c0 10                 	addq	$0x10, %r8
140b30199: 49 83 c1 10                 	addq	$0x10, %r9
140b3019d: 49 21 d0                    	andq	%rdx, %r8
140b301a0: eb 8d                       	jmp	0x140b3012f <.text+0xb2f12f>
140b301a2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140b301b0: 49 f7 db                    	negq	%r11
140b301b3: 49 c1 e3 04                 	shlq	$0x4, %r11
140b301b7: 4a 8b 44 19 f8              	movq	-0x8(%rcx,%r11), %rax
140b301bc: 48 89 c1                    	movq	%rax, %rcx
140b301bf: 48 c1 e9 20                 	shrq	$0x20, %rcx
140b301c3: 0f 84 c7 00 00 00           	je	0x140b30290 <.text+0xb2f290>
140b301c9: 41 89 c0                    	movl	%eax, %r8d
140b301cc: 48 8b 7d 00                 	movq	(%rbp), %rdi
140b301d0: 4c 3b 47 10                 	cmpq	0x10(%rdi), %r8
140b301d4: 0f 83 f6 05 00 00           	jae	0x140b307d0 <.text+0xb2f7d0>
140b301da: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
140b301de: 48 89 c1                    	movq	%rax, %rcx
140b301e1: 48 c1 e9 20                 	shrq	$0x20, %rcx
140b301e5: 4f 8d 0c 80                 	leaq	(%r8,%r8,4), %r9
140b301e9: 42 39 0c 8a                 	cmpl	%ecx, (%rdx,%r9,4)
140b301ed: 0f 85 dd 05 00 00           	jne	0x140b307d0 <.text+0xb2f7d0>
140b301f3: 4e 8d 0c 8a                 	leaq	(%rdx,%r9,4), %r9
140b301f7: 41 8b 51 04                 	movl	0x4(%r9), %edx
140b301fb: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
140b30201: 4c 39 d2                    	cmpq	%r10, %rdx
140b30204: 4c 8b 5d 08                 	movq	0x8(%rbp), %r11
140b30208: 0f 84 c2 05 00 00           	je	0x140b307d0 <.text+0xb2f7d0>
140b3020e: 45 31 c0                    	xorl	%r8d, %r8d
140b30211: 49 39 53 38                 	cmpq	%rdx, 0x38(%r11)
140b30215: 0f 86 c5 05 00 00           	jbe	0x140b307e0 <.text+0xb2f7e0>
140b3021b: 49 8b 4b 28                 	movq	0x28(%r11), %rcx
140b3021f: 41 89 d2                    	movl	%edx, %r10d
140b30222: 41 c1 ea 06                 	shrl	$0x6, %r10d
140b30226: 4a 8b 0c d1                 	movq	(%rcx,%r10,8), %rcx
140b3022a: 48 0f a3 d1                 	btq	%rdx, %rcx
140b3022e: 0f 83 ac 05 00 00           	jae	0x140b307e0 <.text+0xb2f7e0>
140b30234: 41 8b 41 0c                 	movl	0xc(%r9), %eax
140b30238: 41 8b 49 10                 	movl	0x10(%r9), %ecx
140b3023c: 48 8b 97 a8 01 00 00        	movq	0x1a8(%rdi), %rdx
140b30243: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
140b30247: 4d 8b 83 10 01 00 00        	movq	0x110(%r11), %r8
140b3024e: 4c 8b 4c c2 18              	movq	0x18(%rdx,%rax,8), %r9
140b30253: 48 8b 44 c2 38              	movq	0x38(%rdx,%rax,8), %rax
140b30258: 4a 8b 04 c0                 	movq	(%rax,%r8,8), %rax
140b3025c: 48 f7 d0                    	notq	%rax
140b3025f: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140b30263: 48 c1 e0 04                 	shlq	$0x4, %rax
140b30267: 49 8b 44 01 10              	movq	0x10(%r9,%rax), %rax
140b3026c: f3 0f 10 04 c8              	movss	(%rax,%rcx,8), %xmm0
140b30271: 41 0f 2e c1                 	ucomiss	%xmm9, %xmm0
140b30275: 8b 85 f0 00 00 00           	movl	0xf0(%rbp), %eax
140b3027b: 0f 85 ea fd ff ff           	jne	0x140b3006b <.text+0xb2f06b>
140b30281: 0f 8a e4 fd ff ff           	jp	0x140b3006b <.text+0xb2f06b>
140b30287: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140b30290: 48 89 b5 d0 00 00 00        	movq	%rsi, 0xd0(%rbp)
140b30297: 49 83 7e 18 00              	cmpq	$0x0, 0x18(%r14)
140b3029c: 0f 84 3a 04 00 00           	je	0x140b306dc <.text+0xb2f6dc>
140b302a2: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
140b302a6: 48 8d 95 d0 00 00 00        	leaq	0xd0(%rbp), %rdx
140b302ad: e8 be 0a a8 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140b302b2: 49 8b 3e                    	movq	(%r14), %rdi
140b302b5: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
140b302b9: 48 89 ca                    	movq	%rcx, %rdx
140b302bc: 48 21 c2                    	andq	%rax, %rdx
140b302bf: 48 c1 e8 39                 	shrq	$0x39, %rax
140b302c3: 66 0f 6e c0                 	movd	%eax, %xmm0
140b302c7: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b302cb: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b302d0: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140b302d5: 48 8d 87 58 fd ff ff        	leaq	-0x2a8(%rdi), %rax
140b302dc: 45 31 c0                    	xorl	%r8d, %r8d
140b302df: f3 0f 6f 0c 17              	movdqu	(%rdi,%rdx), %xmm1
140b302e4: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140b302e8: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140b302ec: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140b302f1: 45 85 c9                    	testl	%r9d, %r9d
140b302f4: 74 2a                       	je	0x140b30320 <.text+0xb2f320>
140b302f6: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
140b302fb: 49 01 d2                    	addq	%rdx, %r10
140b302fe: 49 21 ca                    	andq	%rcx, %r10
140b30301: 49 f7 da                    	negq	%r10
140b30304: 49 69 da a8 02 00 00        	imulq	$0x2a8, %r10, %rbx      # imm = 0x2A8
140b3030b: 48 39 34 18                 	cmpq	%rsi, (%rax,%rbx)
140b3030f: 74 3f                       	je	0x140b30350 <.text+0xb2f350>
140b30311: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
140b30315: 66 45 21 ca                 	andw	%r9w, %r10w
140b30319: 45 89 d1                    	movl	%r10d, %r9d
140b3031c: 75 d8                       	jne	0x140b302f6 <.text+0xb2f2f6>
140b3031e: 66 90                       	nop
140b30320: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140b30324: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
140b30329: 45 85 c9                    	testl	%r9d, %r9d
140b3032c: 0f 85 aa 03 00 00           	jne	0x140b306dc <.text+0xb2f6dc>
140b30332: 4c 01 c2                    	addq	%r8, %rdx
140b30335: 48 83 c2 10                 	addq	$0x10, %rdx
140b30339: 49 83 c0 10                 	addq	$0x10, %r8
140b3033d: 48 21 ca                    	andq	%rcx, %rdx
140b30340: eb 9d                       	jmp	0x140b302df <.text+0xb2f2df>
140b30342: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140b30350: 48 83 bc 1f 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rdi,%rbx)
140b30359: 0f 82 37 04 00 00           	jb	0x140b30796 <.text+0xb2f796>
140b3035f: f3 0f 10 84 1f 90 fd ff ff  	movss	-0x270(%rdi,%rbx), %xmm0
140b30368: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
140b3036b: 0f 86 25 04 00 00           	jbe	0x140b30796 <.text+0xb2f796>
140b30371: 48 8d 0c 1f                 	leaq	(%rdi,%rbx), %rcx
140b30375: 80 79 f7 00                 	cmpb	$0x0, -0x9(%rcx)
140b30379: 8b 85 f0 00 00 00           	movl	0xf0(%rbp), %eax
140b3037f: 0f 85 e6 fc ff ff           	jne	0x140b3006b <.text+0xb2f06b>
140b30385: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
140b30389: 8d 42 ff                    	leal	-0x1(%rdx), %eax
140b3038c: 83 f8 02                    	cmpl	$0x2, %eax
140b3038f: 73 25                       	jae	0x140b303b6 <.text+0xb2f3b6>
140b30391: 8b 85 f0 00 00 00           	movl	0xf0(%rbp), %eax
140b30397: 48 39 75 10                 	cmpq	%rsi, 0x10(%rbp)
140b3039b: 0f 85 ca fc ff ff           	jne	0x140b3006b <.text+0xb2f06b>
140b303a1: 41 b7 01                    	movb	$0x1, %r15b
140b303a4: 83 79 b8 02                 	cmpl	$0x2, -0x48(%rcx)
140b303a8: 8b 85 f0 00 00 00           	movl	0xf0(%rbp), %eax
140b303ae: 0f 82 b7 fc ff ff           	jb	0x140b3006b <.text+0xb2f06b>
140b303b4: eb 0b                       	jmp	0x140b303c1 <.text+0xb2f3c1>
140b303b6: 85 d2                       	testl	%edx, %edx
140b303b8: 0f 85 7e 03 00 00           	jne	0x140b3073c <.text+0xb2f73c>
140b303be: 45 31 ff                    	xorl	%r15d, %r15d
140b303c1: 44 89 f8                    	movl	%r15d, %eax
140b303c4: 34 01                       	xorb	$0x1, %al
140b303c6: 48 89 75 50                 	movq	%rsi, 0x50(%rbp)
140b303ca: 44 88 7d 58                 	movb	%r15b, 0x58(%rbp)
140b303ce: 88 45 59                    	movb	%al, 0x59(%rbp)
140b303d1: 4c 89 f1                    	movq	%r14, %rcx
140b303d4: 48 89 f2                    	movq	%rsi, %rdx
140b303d7: 4c 8b 45 f8                 	movq	-0x8(%rbp), %r8
140b303db: e8 c0 1f f6 ff              	callq	0x140a923a0 <.text+0xa913a0>
140b303e0: 48 85 c0                    	testq	%rax, %rax
140b303e3: 74 13                       	je	0x140b303f8 <.text+0xb2f3f8>
140b303e5: 44 0f 2e 40 30              	ucomiss	0x30(%rax), %xmm8
140b303ea: 0f 93 c1                    	setae	%cl
140b303ed: 80 78 3d 00                 	cmpb	$0x0, 0x3d(%rax)
140b303f1: 0f 95 c0                    	setne	%al
140b303f4: 20 c8                       	andb	%cl, %al
140b303f6: eb 02                       	jmp	0x140b303fa <.text+0xb2f3fa>
140b303f8: 31 c0                       	xorl	%eax, %eax
140b303fa: 48 01 df                    	addq	%rbx, %rdi
140b303fd: 48 81 c7 60 fd ff ff        	addq	$-0x2a0, %rdi           # imm = 0xFD60
140b30404: 88 44 24 20                 	movb	%al, 0x20(%rsp)
140b30408: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b3040c: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
140b30410: 49 89 f8                    	movq	%rdi, %r8
140b30413: 44 0f b6 8d ee 00 00 00     	movzbl	0xee(%rbp), %r9d
140b3041b: e8 b0 62 e2 ff              	callq	0x1409566d0 <.text+0x9556d0>
140b30420: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
140b30427: 48 89 95 d8 00 00 00        	movq	%rdx, 0xd8(%rbp)
140b3042e: 44 0a bd ef 00 00 00        	orb	0xef(%rbp), %r15b
140b30435: 0f 84 26 01 00 00           	je	0x140b30561 <.text+0xb2f561>
140b3043b: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140b3043f: 44 8b 48 2c                 	movl	0x2c(%rax), %r9d
140b30443: 48 8b 45 28                 	movq	0x28(%rbp), %rax
140b30447: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b3044c: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b30450: 48 89 fa                    	movq	%rdi, %rdx
140b30453: 4c 8d 85 d0 00 00 00        	leaq	0xd0(%rbp), %r8
140b3045a: e8 71 68 e2 ff              	callq	0x140956cd0 <.text+0x955cd0>
140b3045f: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
140b30466: b0 02                       	movb	$0x2, %al
140b30468: 80 79 18 02                 	cmpb	$0x2, 0x18(%rcx)
140b3046c: 72 11                       	jb	0x140b3047f <.text+0xb2f47f>
140b3046e: 48 8b 51 38                 	movq	0x38(%rcx), %rdx
140b30472: 48 3b 95 d8 00 00 00        	cmpq	0xd8(%rbp), %rdx
140b30479: 75 04                       	jne	0x140b3047f <.text+0xb2f47f>
140b3047b: 0f b6 41 19                 	movzbl	0x19(%rcx), %eax
140b3047f: 04 fe                       	addb	$-0x2, %al
140b30481: 0f b6 c0                    	movzbl	%al, %eax
140b30484: 3c 03                       	cmpb	$0x3, %al
140b30486: b9 03 00 00 00              	movl	$0x3, %ecx
140b3048b: 0f 43 c1                    	cmovael	%ecx, %eax
140b3048e: 0f b6 c0                    	movzbl	%al, %eax
140b30491: 48 8d 0d f8 b5 fb 01        	leaq	0x1fbb5f8(%rip), %rcx   # 0x142aeba90
140b30498: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
140b3049c: 48 01 c8                    	addq	%rcx, %rax
140b3049f: ff e0                       	jmpq	*%rax
140b304a1: 8b 85 cc 00 00 00           	movl	0xcc(%rbp), %eax
140b304a7: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b304ab: 89 01                       	movl	%eax, (%rcx)
140b304ad: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
140b304b4: 48 8b 18                    	movq	(%rax), %rbx
140b304b7: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140b304be: 48 8b 38                    	movq	(%rax), %rdi
140b304c1: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140b304c8: 48 3b 38                    	cmpq	(%rax), %rdi
140b304cb: 75 13                       	jne	0x140b304e0 <.text+0xb2f4e0>
140b304cd: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
140b304d4: 48 8d 15 6d 9c 10 02        	leaq	0x2109c6d(%rip), %rdx   # 0x142c3a148
140b304db: e8 50 21 a3 ff              	callq	0x140562630 <.text+0x561630>
140b304e0: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b304e4: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140b304e8: 48 89 f9                    	movq	%rdi, %rcx
140b304eb: 48 c1 e1 04                 	shlq	$0x4, %rcx
140b304ef: 48 89 1c 08                 	movq	%rbx, (%rax,%rcx)
140b304f3: 48 89 74 08 08              	movq	%rsi, 0x8(%rax,%rcx)
140b304f8: 48 ff c7                    	incq	%rdi
140b304fb: 48 89 7a 30                 	movq	%rdi, 0x30(%rdx)
140b304ff: 48 ff 42 40                 	incq	0x40(%rdx)
140b30503: 8b 85 a4 00 00 00           	movl	0xa4(%rbp), %eax
140b30509: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140b3050d: 89 01                       	movl	%eax, (%rcx)
140b3050f: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
140b30516: 48 8b 30                    	movq	(%rax), %rsi
140b30519: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
140b30520: 48 8b 38                    	movq	(%rax), %rdi
140b30523: 48 8b 85 98 00 00 00        	movq	0x98(%rbp), %rax
140b3052a: 48 3b 38                    	cmpq	(%rax), %rdi
140b3052d: 75 13                       	jne	0x140b30542 <.text+0xb2f542>
140b3052f: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
140b30536: 48 8d 15 0b 9c 10 02        	leaq	0x2109c0b(%rip), %rdx   # 0x142c3a148
140b3053d: e8 2e 23 a3 ff              	callq	0x140562870 <.text+0x561870>
140b30542: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140b30546: 48 8b 40 28                 	movq	0x28(%rax), %rax
140b3054a: 48 89 34 f8                 	movq	%rsi, (%rax,%rdi,8)
140b3054e: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
140b30555: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
140b3055c: e9 e1 00 00 00              	jmp	0x140b30642 <.text+0xb2f642>
140b30561: 80 78 18 02                 	cmpb	$0x2, 0x18(%rax)
140b30565: 72 10                       	jb	0x140b30577 <.text+0xb2f577>
140b30567: 48 39 50 38                 	cmpq	%rdx, 0x38(%rax)
140b3056b: 75 0a                       	jne	0x140b30577 <.text+0xb2f577>
140b3056d: 80 78 19 02                 	cmpb	$0x2, 0x19(%rax)
140b30571: 0f 85 c4 fe ff ff           	jne	0x140b3043b <.text+0xb2f43b>
140b30577: 8b 85 f0 00 00 00           	movl	0xf0(%rbp), %eax
140b3057d: e9 e9 fa ff ff              	jmp	0x140b3006b <.text+0xb2f06b>
140b30582: 8b 85 cc 00 00 00           	movl	0xcc(%rbp), %eax
140b30588: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b3058c: 89 01                       	movl	%eax, (%rcx)
140b3058e: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
140b30595: 48 8b 18                    	movq	(%rax), %rbx
140b30598: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140b3059f: 48 8b 38                    	movq	(%rax), %rdi
140b305a2: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140b305a9: 48 3b 38                    	cmpq	(%rax), %rdi
140b305ac: 75 13                       	jne	0x140b305c1 <.text+0xb2f5c1>
140b305ae: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
140b305b5: 48 8d 15 8c 9b 10 02        	leaq	0x2109b8c(%rip), %rdx   # 0x142c3a148
140b305bc: e8 6f 20 a3 ff              	callq	0x140562630 <.text+0x561630>
140b305c1: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140b305c5: 48 8b 40 28                 	movq	0x28(%rax), %rax
140b305c9: 48 89 f9                    	movq	%rdi, %rcx
140b305cc: 48 c1 e1 04                 	shlq	$0x4, %rcx
140b305d0: 48 89 1c 08                 	movq	%rbx, (%rax,%rcx)
140b305d4: 48 89 74 08 08              	movq	%rsi, 0x8(%rax,%rcx)
140b305d9: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140b305e0: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
140b305e7: eb 59                       	jmp	0x140b30642 <.text+0xb2f642>
140b305e9: 8b 85 a0 00 00 00           	movl	0xa0(%rbp), %eax
140b305ef: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140b305f3: 89 01                       	movl	%eax, (%rcx)
140b305f5: 48 8b 45 70                 	movq	0x70(%rbp), %rax
140b305f9: 48 8b 18                    	movq	(%rax), %rbx
140b305fc: 48 8b 45 78                 	movq	0x78(%rbp), %rax
140b30600: 48 8b 38                    	movq	(%rax), %rdi
140b30603: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
140b3060a: 48 3b 38                    	cmpq	(%rax), %rdi
140b3060d: 75 13                       	jne	0x140b30622 <.text+0xb2f622>
140b3060f: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
140b30616: 48 8d 15 2b 9b 10 02        	leaq	0x2109b2b(%rip), %rdx   # 0x142c3a148
140b3061d: e8 0e 20 a3 ff              	callq	0x140562630 <.text+0x561630>
140b30622: 48 8b 45 30                 	movq	0x30(%rbp), %rax
140b30626: 48 8b 40 28                 	movq	0x28(%rax), %rax
140b3062a: 48 89 f9                    	movq	%rdi, %rcx
140b3062d: 48 c1 e1 04                 	shlq	$0x4, %rcx
140b30631: 48 89 1c 08                 	movq	%rbx, (%rax,%rcx)
140b30635: 48 89 74 08 08              	movq	%rsi, 0x8(%rax,%rcx)
140b3063a: 48 8b 45 78                 	movq	0x78(%rbp), %rax
140b3063e: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
140b30642: 48 ff c7                    	incq	%rdi
140b30645: 48 89 38                    	movq	%rdi, (%rax)
140b30648: 48 ff 01                    	incq	(%rcx)
140b3064b: 8b 85 f0 00 00 00           	movl	0xf0(%rbp), %eax
140b30651: e9 15 fa ff ff              	jmp	0x140b3006b <.text+0xb2f06b>
140b30656: 80 bd f7 00 00 00 00        	cmpb	$0x0, 0xf7(%rbp)
140b3065d: 74 4b                       	je	0x140b306aa <.text+0xb2f6aa>
140b3065f: e8 cc 7a 0e 00              	callq	0x140c18130 <.text+0xc17130>
140b30664: 48 89 c1                    	movq	%rax, %rcx
140b30667: 48 8b 00                    	movq	(%rax), %rax
140b3066a: 48 83 f8 01                 	cmpq	$0x1, %rax
140b3066e: 0f 85 9a 00 00 00           	jne	0x140b3070e <.text+0xb2f70e>
140b30674: 48 83 c1 08                 	addq	$0x8, %rcx
140b30678: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
140b3067f: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140b30683: 0f 85 01 01 00 00           	jne	0x140b3078a <.text+0xb2f78a>
140b30689: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140b30690: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
140b30697: 48 83 c1 08                 	addq	$0x8, %rcx
140b3069b: e8 50 6a 0e 00              	callq	0x140c170f0 <.text+0xc160f0>
140b306a0: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140b306a7: 48 ff 00                    	incq	(%rax)
140b306aa: 0f 28 b5 00 01 00 00        	movaps	0x100(%rbp), %xmm6
140b306b1: 0f 28 bd 10 01 00 00        	movaps	0x110(%rbp), %xmm7
140b306b8: 44 0f 28 85 20 01 00 00     	movaps	0x120(%rbp), %xmm8
140b306c0: 44 0f 28 8d 30 01 00 00     	movaps	0x130(%rbp), %xmm9
140b306c8: 48 81 c4 c8 01 00 00        	addq	$0x1c8, %rsp            # imm = 0x1C8
140b306cf: 5b                          	popq	%rbx
140b306d0: 5f                          	popq	%rdi
140b306d1: 5e                          	popq	%rsi
140b306d2: 41 5c                       	popq	%r12
140b306d4: 41 5d                       	popq	%r13
140b306d6: 41 5e                       	popq	%r14
140b306d8: 41 5f                       	popq	%r15
140b306da: 5d                          	popq	%rbp
140b306db: c3                          	retq
140b306dc: 48 8d 0d 75 b2 fb 01        	leaq	0x1fbb275(%rip), %rcx   # 0x142aeb958
140b306e3: e8 28 8c da 01              	callq	0x1428d9310 <.text+0x28d8310>
140b306e8: e9 32 01 00 00              	jmp	0x140b3081f <.text+0xb2f81f>
140b306ed: 48 85 c0                    	testq	%rax, %rax
140b306f0: 0f 85 ce 00 00 00           	jne	0x140b307c4 <.text+0xb2f7c4>
140b306f6: 31 d2                       	xorl	%edx, %edx
140b306f8: e8 d3 80 dc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140b306fd: 48 89 c1                    	movq	%rax, %rcx
140b30700: 48 85 c0                    	testq	%rax, %rax
140b30703: 0f 85 3f f7 ff ff           	jne	0x140b2fe48 <.text+0xb2ee48>
140b30709: e9 b6 00 00 00              	jmp	0x140b307c4 <.text+0xb2f7c4>
140b3070e: 48 85 c0                    	testq	%rax, %rax
140b30711: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
140b30718: 0f 85 a6 00 00 00           	jne	0x140b307c4 <.text+0xb2f7c4>
140b3071e: 48 89 c6                    	movq	%rax, %rsi
140b30721: 31 d2                       	xorl	%edx, %edx
140b30723: e8 a8 80 dc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140b30728: 48 89 f2                    	movq	%rsi, %rdx
140b3072b: 48 89 c1                    	movq	%rax, %rcx
140b3072e: 48 85 c0                    	testq	%rax, %rax
140b30731: 0f 85 48 ff ff ff           	jne	0x140b3067f <.text+0xb2f67f>
140b30737: e9 88 00 00 00              	jmp	0x140b307c4 <.text+0xb2f7c4>
140b3073c: 80 bd f7 00 00 00 00        	cmpb	$0x0, 0xf7(%rbp)
140b30743: 48 8b b5 a8 00 00 00        	movq	0xa8(%rbp), %rsi
140b3074a: 0f 84 5a ff ff ff           	je	0x140b306aa <.text+0xb2f6aa>
140b30750: e8 db 79 0e 00              	callq	0x140c18130 <.text+0xc17130>
140b30755: 48 89 c1                    	movq	%rax, %rcx
140b30758: 48 8b 00                    	movq	(%rax), %rax
140b3075b: 48 83 f8 01                 	cmpq	$0x1, %rax
140b3075f: 75 4f                       	jne	0x140b307b0 <.text+0xb2f7b0>
140b30761: 48 83 c1 08                 	addq	$0x8, %rcx
140b30765: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140b30769: 75 1f                       	jne	0x140b3078a <.text+0xb2f78a>
140b3076b: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140b30772: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
140b30779: 48 83 c1 08                 	addq	$0x8, %rcx
140b3077d: 48 89 f2                    	movq	%rsi, %rdx
140b30780: e8 6b 69 0e 00              	callq	0x140c170f0 <.text+0xc160f0>
140b30785: e9 16 ff ff ff              	jmp	0x140b306a0 <.text+0xb2f6a0>
140b3078a: 48 8d 0d df a3 10 02        	leaq	0x210a3df(%rip), %rcx   # 0x142c3ab70
140b30791: e8 ba 8a da 01              	callq	0x1428d9250 <.text+0x28d8250>
140b30796: 48 8d 0d f6 99 fa 01        	leaq	0x1fa99f6(%rip), %rcx   # 0x142ada193
140b3079d: 4c 8d 05 9c b1 fb 01        	leaq	0x1fbb19c(%rip), %r8    # 0x142aeb940
140b307a4: ba 32 00 00 00              	movl	$0x32, %edx
140b307a9: e8 82 8c da 01              	callq	0x1428d9430 <.text+0x28d8430>
140b307ae: eb 6f                       	jmp	0x140b3081f <.text+0xb2f81f>
140b307b0: 48 85 c0                    	testq	%rax, %rax
140b307b3: 75 0f                       	jne	0x140b307c4 <.text+0xb2f7c4>
140b307b5: 31 d2                       	xorl	%edx, %edx
140b307b7: e8 14 80 dc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140b307bc: 48 89 c1                    	movq	%rax, %rcx
140b307bf: 48 85 c0                    	testq	%rax, %rax
140b307c2: 75 a1                       	jne	0x140b30765 <.text+0xb2f765>
140b307c4: 48 8d 0d d5 a2 10 02        	leaq	0x210a2d5(%rip), %rcx   # 0x142c3aaa0
140b307cb: e8 00 79 da 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140b307d0: 48 c1 e1 20                 	shlq	$0x20, %rcx
140b307d4: 4c 09 c1                    	orq	%r8, %rcx
140b307d7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b307dd: 48 89 c8                    	movq	%rcx, %rax
140b307e0: 44 89 85 d0 00 00 00        	movl	%r8d, 0xd0(%rbp)
140b307e7: 89 95 d4 00 00 00           	movl	%edx, 0xd4(%rbp)
140b307ed: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
140b307f4: 48 8d 05 2d b1 fb 01        	leaq	0x1fbb12d(%rip), %rax   # 0x142aeb928
140b307fb: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b30800: 48 8d 0d d9 a9 fb 01        	leaq	0x1fba9d9(%rip), %rcx   # 0x142aeb1e0
140b30807: 4c 8d 0d b2 a9 fb 01        	leaq	0x1fba9b2(%rip), %r9    # 0x142aeb1c0
140b3080e: 4c 8d 85 d0 00 00 00        	leaq	0xd0(%rbp), %r8
140b30815: ba 2b 00 00 00              	movl	$0x2b, %edx
140b3081a: e8 41 8f da 01              	callq	0x1428d9760 <.text+0x28d8760>
140b3081f: 0f 0b                       	ud2
140b30821: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140b30830: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b30835: 55                          	pushq	%rbp
140b30836: 41 57                       	pushq	%r15
140b30838: 41 56                       	pushq	%r14
140b3083a: 41 55                       	pushq	%r13
140b3083c: 41 54                       	pushq	%r12
140b3083e: 56                          	pushq	%rsi
140b3083f: 57                          	pushq	%rdi
140b30840: 53                          	pushq	%rbx
140b30841: 48 83 ec 78                 	subq	$0x78, %rsp
140b30845: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3084c: 44 0f 29 4c 24 30           	movaps	%xmm9, 0x30(%rsp)
140b30852: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
140b30858: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
140b3085d: 66 0f 7f 74 24 60           	movdqa	%xmm6, 0x60(%rsp)
140b30863: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140b3086a: 48 ff 00                    	incq	(%rax)
140b3086d: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
140b30872: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
140b30877: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
140b3087d: 44 0f 28 4c 24 30           	movaps	0x30(%rsp), %xmm9
140b30883: 48 83 c4 78                 	addq	$0x78, %rsp
140b30887: 5b                          	popq	%rbx
140b30888: 5f                          	popq	%rdi
140b30889: 5e                          	popq	%rsi
140b3088a: 41 5c                       	popq	%r12
140b3088c: 41 5d                       	popq	%r13
140b3088e: 41 5e                       	popq	%r14
140b30890: 41 5f                       	popq	%r15
140b30892: 5d                          	popq	%rbp
140b30893: c3                          	retq
140b30894: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140b308a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b308a5: 55                          	pushq	%rbp
140b308a6: 41 57                       	pushq	%r15
140b308a8: 41 56                       	pushq	%r14
140b308aa: 41 55                       	pushq	%r13
140b308ac: 41 54                       	pushq	%r12
140b308ae: 56                          	pushq	%rsi
140b308af: 57                          	pushq	%rdi
140b308b0: 53                          	pushq	%rbx
140b308b1: 48 83 ec 78                 	subq	$0x78, %rsp
140b308b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b308bc: 44 0f 29 4c 24 30           	movaps	%xmm9, 0x30(%rsp)
140b308c2: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
140b308c8: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
140b308cd: 66 0f 7f 74 24 60           	movdqa	%xmm6, 0x60(%rsp)
140b308d3: 80 bd f7 00 00 00 00        	cmpb	$0x0, 0xf7(%rbp)
140b308da: 74 20                       	je	0x140b308fc <.text+0xb2f8fc>
140b308dc: 48 8d 45 c8                 	leaq	-0x38(%rbp), %rax
140b308e0: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
140b308e4: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
140b308e8: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
140b308ec: 48 8d 0d 75 aa fb 01        	leaq	0x1fbaa75(%rip), %rcx   # 0x142aeb368
140b308f3: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
140b308f7: e8 d4 e9 7f 00              	callq	0x14132f2d0 <.text+0x132e2d0>
140b308fc: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
140b30901: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
140b30906: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
140b3090c: 44 0f 28 4c 24 30           	movaps	0x30(%rsp), %xmm9
140b30912: 48 83 c4 78                 	addq	$0x78, %rsp
140b30916: 5b                          	popq	%rbx
140b30917: 5f                          	popq	%rdi
140b30918: 5e                          	popq	%rsi
140b30919: 41 5c                       	popq	%r12
140b3091b: 41 5d                       	popq	%r13
140b3091d: 41 5e                       	popq	%r14
140b3091f: 41 5f                       	popq	%r15
140b30921: 5d                          	popq	%rbp
140b30922: c3                          	retq
140b30923: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140b30930: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b30935: 55                          	pushq	%rbp
140b30936: 41 57                       	pushq	%r15
140b30938: 41 56                       	pushq	%r14
140b3093a: 41 55                       	pushq	%r13
140b3093c: 41 54                       	pushq	%r12
140b3093e: 56                          	pushq	%rsi
140b3093f: 57                          	pushq	%rdi
140b30940: 53                          	pushq	%rbx
140b30941: 48 83 ec 78                 	subq	$0x78, %rsp
140b30945: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3094c: 44 0f 29 4c 24 30           	movaps	%xmm9, 0x30(%rsp)
140b30952: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
140b30958: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
140b3095d: 66 0f 7f 74 24 60           	movdqa	%xmm6, 0x60(%rsp)
140b30963: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140b3096a: 48 ff 00                    	incq	(%rax)
140b3096d: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
140b30972: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
140b30977: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
140b3097d: 44 0f 28 4c 24 30           	movaps	0x30(%rsp), %xmm9
140b30983: 48 83 c4 78                 	addq	$0x78, %rsp
140b30987: 5b                          	popq	%rbx
140b30988: 5f                          	popq	%rdi
140b30989: 5e                          	popq	%rsi
140b3098a: 41 5c                       	popq	%r12
140b3098c: 41 5d                       	popq	%r13
140b3098e: 41 5e                       	popq	%r14
140b30990: 41 5f                       	popq	%r15
140b30992: 5d                          	popq	%rbp
140b30993: c3                          	retq
140b30994: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140b309a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b309a5: 55                          	pushq	%rbp
140b309a6: 41 57                       	pushq	%r15
140b309a8: 41 56                       	pushq	%r14
140b309aa: 41 55                       	pushq	%r13
140b309ac: 41 54                       	pushq	%r12
140b309ae: 56                          	pushq	%rsi
140b309af: 57                          	pushq	%rdi
140b309b0: 53                          	pushq	%rbx
140b309b1: 48 83 ec 78                 	subq	$0x78, %rsp
140b309b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b309bc: 44 0f 29 4c 24 30           	movaps	%xmm9, 0x30(%rsp)
140b309c2: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
140b309c8: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
140b309cd: 66 0f 7f 74 24 60           	movdqa	%xmm6, 0x60(%rsp)
140b309d3: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140b309da: 48 ff 00                    	incq	(%rax)
140b309dd: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
140b309e2: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
140b309e7: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
140b309ed: 44 0f 28 4c 24 30           	movaps	0x30(%rsp), %xmm9
140b309f3: 48 83 c4 78                 	addq	$0x78, %rsp
140b309f7: 5b                          	popq	%rbx
140b309f8: 5f                          	popq	%rdi
140b309f9: 5e                          	popq	%rsi
140b309fa: 41 5c                       	popq	%r12
140b309fc: 41 5d                       	popq	%r13
140b309fe: 41 5e                       	popq	%r14
140b30a00: 41 5f                       	popq	%r15
140b30a02: 5d                          	popq	%rbp
140b30a03: c3                          	retq
140b30a04: cc                          	int3
140b30a05: cc                          	int3
140b30a06: cc                          	int3
140b30a07: cc                          	int3
140b30a08: cc                          	int3
140b30a09: cc                          	int3
140b30a0a: cc                          	int3
140b30a0b: cc                          	int3
140b30a0c: cc                          	int3
140b30a0d: cc                          	int3
140b30a0e: cc                          	int3
140b30a0f: cc                          	int3
140b30a10: 48 83 ec 48                 	subq	$0x48, %rsp
140b30a14: 48 89 d0                    	movq	%rdx, %rax
140b30a17: 48 89 4c 24 40              	movq	%rcx, 0x40(%rsp)
140b30a1c: 48 8d 0d 4d af fb 01        	leaq	0x1fbaf4d(%rip), %rcx   # 0x142aeb970
140b30a23: 48 89 4c 24 30              	movq	%rcx, 0x30(%rsp)
140b30a28: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
140b30a2d: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
140b30a32: 48 c7 44 24 20 05 00 00 00  	movq	$0x5, 0x20(%rsp)
140b30a3b: 48 8d 15 4e af fb 01        	leaq	0x1fbaf4e(%rip), %rdx   # 0x142aeb990
140b30a42: 4c 8d 0d 67 af fb 01        	leaq	0x1fbaf67(%rip), %r9    # 0x142aeb9b0
140b30a49: 41 b8 20 00 00 00           	movl	$0x20, %r8d
140b30a4f: 48 89 c1                    	movq	%rax, %rcx
140b30a52: e8 99 61 25 00              	callq	0x140d86bf0 <.text+0xd85bf0>
140b30a57: 90                          	nop
140b30a58: 48 83 c4 48                 	addq	$0x48, %rsp
140b30a5c: c3                          	retq
140b30a5d: cc                          	int3
140b30a5e: cc                          	int3
140b30a5f: cc                          	int3
