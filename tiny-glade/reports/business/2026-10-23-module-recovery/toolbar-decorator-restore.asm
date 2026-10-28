
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b2c4b0: 55                          	pushq	%rbp
140b2c4b1: 41 57                       	pushq	%r15
140b2c4b3: 41 56                       	pushq	%r14
140b2c4b5: 41 55                       	pushq	%r13
140b2c4b7: 41 54                       	pushq	%r12
140b2c4b9: 56                          	pushq	%rsi
140b2c4ba: 57                          	pushq	%rdi
140b2c4bb: 53                          	pushq	%rbx
140b2c4bc: 48 81 ec 68 03 00 00        	subq	$0x368, %rsp            # imm = 0x368
140b2c4c3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b2c4cb: 48 c7 85 e0 02 00 00 fe ff ff ff    	movq	$-0x2, 0x2e0(%rbp)
140b2c4d6: 48 89 95 d0 02 00 00        	movq	%rdx, 0x2d0(%rbp)
140b2c4dd: 49 89 ce                    	movq	%rcx, %r14
140b2c4e0: 48 8d 99 c0 00 00 00        	leaq	0xc0(%rcx), %rbx
140b2c4e7: 4c 8d b9 50 01 00 00        	leaq	0x150(%rcx), %r15
140b2c4ee: 48 83 c1 60                 	addq	$0x60, %rcx
140b2c4f2: 49 8d be 90 00 00 00        	leaq	0x90(%r14), %rdi
140b2c4f9: e8 62 d4 04 00              	callq	0x140b79960 <.text+0xb78960>
140b2c4fe: 48 89 f9                    	movq	%rdi, %rcx
140b2c501: e8 5a d4 04 00              	callq	0x140b79960 <.text+0xb78960>
140b2c506: 4c 8d 6d 30                 	leaq	0x30(%rbp), %r13
140b2c50a: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140b2c510: 4c 89 e9                    	movq	%r13, %rcx
140b2c513: 48 89 da                    	movq	%rbx, %rdx
140b2c516: e8 e5 76 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2c51b: 49 c7 86 c0 00 00 00 00 00 00 00    	movq	$0x0, 0xc0(%r14)
140b2c526: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
140b2c52d: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140b2c533: 4c 89 fa                    	movq	%r15, %rdx
140b2c536: 49 89 cf                    	movq	%rcx, %r15
140b2c539: e8 c2 76 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2c53e: 49 c7 86 50 01 00 00 00 00 00 00    	movq	$0x0, 0x150(%r14)
140b2c549: 49 8b 06                    	movq	(%r14), %rax
140b2c54c: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
140b2c550: 48 01 c1                    	addq	%rax, %rcx
140b2c553: 48 ff c1                    	incq	%rcx
140b2c556: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
140b2c55a: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
140b2c55e: f7 d2                       	notl	%edx
140b2c560: 4d 8b 46 18                 	movq	0x18(%r14), %r8
140b2c564: 41 0f 10 06                 	movups	(%r14), %xmm0
140b2c568: 41 0f 10 4e 10              	movups	0x10(%r14), %xmm1
140b2c56d: 0f 11 8d b8 02 00 00        	movups	%xmm1, 0x2b8(%rbp)
140b2c574: 0f 11 85 a8 02 00 00        	movups	%xmm0, 0x2a8(%rbp)
140b2c57b: 0f 10 05 2e 86 fc 01        	movups	0x1fc862e(%rip), %xmm0  # 0x142af4bb0
140b2c582: 41 0f 11 06                 	movups	%xmm0, (%r14)
140b2c586: f3 0f 6f 05 32 86 fc 01     	movdqu	0x1fc8632(%rip), %xmm0  # 0x142af4bc0
140b2c58e: f3 41 0f 7f 46 10           	movdqu	%xmm0, 0x10(%r14)
140b2c594: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
140b2c59b: 48 83 c0 10                 	addq	$0x10, %rax
140b2c59f: 48 89 85 88 02 00 00        	movq	%rax, 0x288(%rbp)
140b2c5a6: 48 89 8d 90 02 00 00        	movq	%rcx, 0x290(%rbp)
140b2c5ad: 66 89 95 98 02 00 00        	movw	%dx, 0x298(%rbp)
140b2c5b4: 4c 89 85 a0 02 00 00        	movq	%r8, 0x2a0(%rbp)
140b2c5bb: 4c 89 b5 c8 02 00 00        	movq	%r14, 0x2c8(%rbp)
140b2c5c2: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140b2c5c9: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140b2c5cf: 4c 89 ea                    	movq	%r13, %rdx
140b2c5d2: e8 29 76 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2c5d7: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
140b2c5de: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140b2c5e4: 4c 89 fa                    	movq	%r15, %rdx
140b2c5e7: e8 14 76 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2c5ec: 48 c7 85 50 01 00 00 01 00 00 00    	movq	$0x1, 0x150(%rbp)
140b2c5f7: 48 c7 85 e8 01 00 00 01 00 00 00    	movq	$0x1, 0x1e8(%rbp)
140b2c602: b8 01 00 00 00              	movl	$0x1, %eax
140b2c607: 4c 8d 75 a8                 	leaq	-0x58(%rbp), %r14
140b2c60b: bf 50 00 00 00              	movl	$0x50, %edi
140b2c610: eb 15                       	jmp	0x140b2c627 <.text+0xb2b627>
140b2c612: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140b2c620: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
140b2c627: 48 83 f8 02                 	cmpq	$0x2, %rax
140b2c62b: 0f 84 9c 02 00 00           	je	0x140b2c8cd <.text+0xb2b8cd>
140b2c631: 4c 8b a5 80 02 00 00        	movq	0x280(%rbp), %r12
140b2c638: 4d 85 e4                    	testq	%r12, %r12
140b2c63b: 0f 84 0f 02 00 00           	je	0x140b2c850 <.text+0xb2b850>
140b2c641: 4c 8b ad a0 02 00 00        	movq	0x2a0(%rbp), %r13
140b2c648: 4d 85 ed                    	testq	%r13, %r13
140b2c64b: 0f 84 76 01 00 00           	je	0x140b2c7c7 <.text+0xb2b7c7>
140b2c651: 0f b7 85 98 02 00 00        	movzwl	0x298(%rbp), %eax
140b2c658: 66 85 c0                    	testw	%ax, %ax
140b2c65b: 74 07                       	je	0x140b2c664 <.text+0xb2b664>
140b2c65d: 8d 48 ff                    	leal	-0x1(%rax), %ecx
140b2c660: 21 c1                       	andl	%eax, %ecx
140b2c662: eb 41                       	jmp	0x140b2c6a5 <.text+0xb2b6a5>
140b2c664: 48 8b 8d 88 02 00 00        	movq	0x288(%rbp), %rcx
140b2c66b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b2c670: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
140b2c674: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140b2c678: 49 81 c4 80 f6 ff ff        	addq	$-0x980, %r12           # imm = 0xF680
140b2c67f: 48 83 c1 10                 	addq	$0x10, %rcx
140b2c683: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140b2c688: 74 e6                       	je	0x140b2c670 <.text+0xb2b670>
140b2c68a: 48 89 8d 88 02 00 00        	movq	%rcx, 0x288(%rbp)
140b2c691: 4c 89 a5 80 02 00 00        	movq	%r12, 0x280(%rbp)
140b2c698: ba fe ff ff ff              	movl	$0xfffffffe, %edx       # imm = 0xFFFFFFFE
140b2c69d: 29 c2                       	subl	%eax, %edx
140b2c69f: f7 d0                       	notl	%eax
140b2c6a1: 89 c1                       	movl	%eax, %ecx
140b2c6a3: 21 d1                       	andl	%edx, %ecx
140b2c6a5: 66 89 8d 98 02 00 00        	movw	%cx, 0x298(%rbp)
140b2c6ac: 49 ff cd                    	decq	%r13
140b2c6af: 4c 89 ad a0 02 00 00        	movq	%r13, 0x2a0(%rbp)
140b2c6b6: f3 0f bc c0                 	tzcntl	%eax, %eax
140b2c6ba: 48 f7 d8                    	negq	%rax
140b2c6bd: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
140b2c6c4: 49 8b bc 04 68 ff ff ff     	movq	-0x98(%r12,%rax), %rdi
140b2c6cc: 49 8b 9c 04 70 ff ff ff     	movq	-0x90(%r12,%rax), %rbx
140b2c6d4: 49 8d 14 04                 	leaq	(%r12,%rax), %rdx
140b2c6d8: 48 81 c2 78 ff ff ff        	addq	$-0x88, %rdx
140b2c6df: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140b2c6e5: 4c 8d bd c0 00 00 00        	leaq	0xc0(%rbp), %r15
140b2c6ec: 4c 89 f9                    	movq	%r15, %rcx
140b2c6ef: e8 0c 75 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2c6f4: 48 85 db                    	testq	%rbx, %rbx
140b2c6f7: 74 24                       	je	0x140b2c71d <.text+0xb2b71d>
140b2c6f9: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140b2c6ff: 4c 8d 6d 30                 	leaq	0x30(%rbp), %r13
140b2c703: 4c 89 e9                    	movq	%r13, %rcx
140b2c706: 4c 89 fa                    	movq	%r15, %rdx
140b2c709: e8 f2 74 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2c70e: 45 31 e4                    	xorl	%r12d, %r12d
140b2c711: 48 89 bd d8 02 00 00        	movq	%rdi, 0x2d8(%rbp)
140b2c718: e9 70 01 00 00              	jmp	0x140b2c88d <.text+0xb2b88d>
140b2c71d: 4d 85 ed                    	testq	%r13, %r13
140b2c720: 4c 8d 75 a8                 	leaq	-0x58(%rbp), %r14
140b2c724: bf 50 00 00 00              	movl	$0x50, %edi
140b2c729: 0f 84 98 00 00 00           	je	0x140b2c7c7 <.text+0xb2b7c7>
140b2c72f: 0f b7 85 98 02 00 00        	movzwl	0x298(%rbp), %eax
140b2c736: 48 8b 9d 88 02 00 00        	movq	0x288(%rbp), %rbx
140b2c73d: 0f 1f 00                    	nopl	(%rax)
140b2c740: 66 85 c0                    	testw	%ax, %ax
140b2c743: 74 0b                       	je	0x140b2c750 <.text+0xb2b750>
140b2c745: 44 8d 70 ff                 	leal	-0x1(%rax), %r14d
140b2c749: 41 21 c6                    	andl	%eax, %r14d
140b2c74c: eb 39                       	jmp	0x140b2c787 <.text+0xb2b787>
140b2c74e: 66 90                       	nop
140b2c750: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
140b2c754: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140b2c758: 49 81 c4 80 f6 ff ff        	addq	$-0x980, %r12           # imm = 0xF680
140b2c75f: 48 83 c3 10                 	addq	$0x10, %rbx
140b2c763: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140b2c768: 74 e6                       	je	0x140b2c750 <.text+0xb2b750>
140b2c76a: 48 89 9d 88 02 00 00        	movq	%rbx, 0x288(%rbp)
140b2c771: 4c 89 a5 80 02 00 00        	movq	%r12, 0x280(%rbp)
140b2c778: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140b2c77d: 29 c1                       	subl	%eax, %ecx
140b2c77f: f7 d0                       	notl	%eax
140b2c781: 41 89 c6                    	movl	%eax, %r14d
140b2c784: 41 21 ce                    	andl	%ecx, %r14d
140b2c787: 66 44 89 b5 98 02 00 00     	movw	%r14w, 0x298(%rbp)
140b2c78f: 49 ff cd                    	decq	%r13
140b2c792: 4c 89 ad a0 02 00 00        	movq	%r13, 0x2a0(%rbp)
140b2c799: f3 0f bc c0                 	tzcntl	%eax, %eax
140b2c79d: 48 f7 d8                    	negq	%rax
140b2c7a0: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
140b2c7a7: 49 8d 0c 04                 	leaq	(%r12,%rax), %rcx
140b2c7ab: 48 81 c1 70 ff ff ff        	addq	$-0x90, %rcx
140b2c7b2: e8 d9 82 ad ff              	callq	0x140604a90 <.text+0x603a90>
140b2c7b7: 44 89 f0                    	movl	%r14d, %eax
140b2c7ba: 4d 85 ed                    	testq	%r13, %r13
140b2c7bd: 4c 8d 75 a8                 	leaq	-0x58(%rbp), %r14
140b2c7c1: 0f 85 79 ff ff ff           	jne	0x140b2c740 <.text+0xb2b740>
140b2c7c7: 48 8b 9d b0 02 00 00        	movq	0x2b0(%rbp), %rbx
140b2c7ce: 48 85 db                    	testq	%rbx, %rbx
140b2c7d1: 74 2e                       	je	0x140b2c801 <.text+0xb2b801>
140b2c7d3: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
140b2c7da: 4c 8d 43 11                 	leaq	0x11(%rbx), %r8
140b2c7de: b2 ff                       	movb	$-0x1, %dl
140b2c7e0: e8 3b 74 df 01              	callq	0x142923c20 <.text+0x2922c20>
140b2c7e5: 48 8d 43 01                 	leaq	0x1(%rbx), %rax
140b2c7e9: 48 89 c1                    	movq	%rax, %rcx
140b2c7ec: 48 c1 e9 03                 	shrq	$0x3, %rcx
140b2c7f0: 48 83 e0 f8                 	andq	$-0x8, %rax
140b2c7f4: 48 29 c8                    	subq	%rcx, %rax
140b2c7f7: 48 83 fb 08                 	cmpq	$0x8, %rbx
140b2c7fb: 48 0f 42 c3                 	cmovbq	%rbx, %rax
140b2c7ff: eb 02                       	jmp	0x140b2c803 <.text+0xb2b803>
140b2c801: 31 c0                       	xorl	%eax, %eax
140b2c803: 4c 8d 6d 30                 	leaq	0x30(%rbp), %r13
140b2c807: 48 c7 85 c0 02 00 00 00 00 00 00    	movq	$0x0, 0x2c0(%rbp)
140b2c812: 48 89 85 b8 02 00 00        	movq	%rax, 0x2b8(%rbp)
140b2c819: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
140b2c820: 48 8d 8d a8 02 00 00        	leaq	0x2a8(%rbp), %rcx
140b2c827: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
140b2c82b: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
140b2c82f: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140b2c833: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
140b2c837: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
140b2c83e: 48 c7 85 80 02 00 00 00 00 00 00    	movq	$0x0, 0x280(%rbp)
140b2c849: 4c 8d bd c0 00 00 00        	leaq	0xc0(%rbp), %r15
140b2c850: a8 01                       	testb	$0x1, %al
140b2c852: 74 5c                       	je	0x140b2c8b0 <.text+0xb2b8b0>
140b2c854: 48 8b 9d f0 01 00 00        	movq	0x1f0(%rbp), %rbx
140b2c85b: 48 c7 85 f0 01 00 00 00 00 00 00    	movq	$0x0, 0x1f0(%rbp)
140b2c866: 48 85 db                    	testq	%rbx, %rbx
140b2c869: 74 45                       	je	0x140b2c8b0 <.text+0xb2b8b0>
140b2c86b: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140b2c871: 4c 89 e9                    	movq	%r13, %rcx
140b2c874: 48 8d 95 f8 01 00 00        	leaq	0x1f8(%rbp), %rdx
140b2c87b: e8 80 73 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2c880: 41 bc 01 00 00 00           	movl	$0x1, %r12d
140b2c886: 48 8b bd d8 02 00 00        	movq	0x2d8(%rbp), %rdi
140b2c88d: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140b2c893: 4c 8d 75 a8                 	leaq	-0x58(%rbp), %r14
140b2c897: 4c 89 f1                    	movq	%r14, %rcx
140b2c89a: 4c 89 ea                    	movq	%r13, %rdx
140b2c89d: e8 5e 73 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2c8a2: 48 89 fe                    	movq	%rdi, %rsi
140b2c8a5: bf 50 00 00 00              	movl	$0x50, %edi
140b2c8aa: eb 64                       	jmp	0x140b2c910 <.text+0xb2b910>
140b2c8ac: 0f 1f 40 00                 	nopl	(%rax)
140b2c8b0: 48 83 f8 02                 	cmpq	$0x2, %rax
140b2c8b4: 74 0c                       	je	0x140b2c8c2 <.text+0xb2b8c2>
140b2c8b6: 48 8d 8d e8 01 00 00        	leaq	0x1e8(%rbp), %rcx
140b2c8bd: e8 fe e8 d5 ff              	callq	0x14088b1c0 <.text+0x88a1c0>
140b2c8c2: 48 c7 85 e8 01 00 00 02 00 00 00    	movq	$0x2, 0x1e8(%rbp)
140b2c8cd: 83 bd 50 01 00 00 01        	cmpl	$0x1, 0x150(%rbp)
140b2c8d4: 0f 85 2b 01 00 00           	jne	0x140b2ca05 <.text+0xb2ba05>
140b2c8da: 48 8b 9d 58 01 00 00        	movq	0x158(%rbp), %rbx
140b2c8e1: 48 c7 85 58 01 00 00 00 00 00 00    	movq	$0x0, 0x158(%rbp)
140b2c8ec: 48 85 db                    	testq	%rbx, %rbx
140b2c8ef: 0f 84 10 01 00 00           	je	0x140b2ca05 <.text+0xb2ba05>
140b2c8f5: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140b2c8fb: 4c 89 f1                    	movq	%r14, %rcx
140b2c8fe: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
140b2c905: e8 f6 72 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2c90a: 41 bc 02 00 00 00           	movl	$0x2, %r12d
140b2c910: 48 89 9d c0 00 00 00        	movq	%rbx, 0xc0(%rbp)
140b2c917: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140b2c91d: 48 8d 8d c8 00 00 00        	leaq	0xc8(%rbp), %rcx
140b2c924: 4c 89 f2                    	movq	%r14, %rdx
140b2c927: e8 d4 72 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2c92c: 48 8b 8d d0 02 00 00        	movq	0x2d0(%rbp), %rcx
140b2c933: 4c 89 e2                    	movq	%r12, %rdx
140b2c936: 49 89 f0                    	movq	%rsi, %r8
140b2c939: e8 f2 05 ef ff              	callq	0x140a1cf30 <.text+0xa1bf30>
140b2c93e: 48 89 55 40                 	movq	%rdx, 0x40(%rbp)
140b2c942: 4c 89 65 30                 	movq	%r12, 0x30(%rbp)
140b2c946: 48 89 75 38                 	movq	%rsi, 0x38(%rbp)
140b2c94a: 48 89 c1                    	movq	%rax, %rcx
140b2c94d: 4c 89 fa                    	movq	%r15, %rdx
140b2c950: 4d 89 e8                    	movq	%r13, %r8
140b2c953: e8 f8 df a2 00              	callq	0x14155a950 <.text+0x1559950>
140b2c958: 4c 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %r9
140b2c95f: 4d 85 c9                    	testq	%r9, %r9
140b2c962: 74 28                       	je	0x140b2c98c <.text+0xb2b98c>
140b2c964: 49 8d 41 01                 	leaq	0x1(%r9), %rax
140b2c968: 48 f7 e7                    	mulq	%rdi
140b2c96b: 49 01 c1                    	addq	%rax, %r9
140b2c96e: 49 83 c1 11                 	addq	$0x11, %r9
140b2c972: 74 18                       	je	0x140b2c98c <.text+0xb2b98c>
140b2c974: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
140b2c97b: 48 29 c1                    	subq	%rax, %rcx
140b2c97e: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b2c984: 4c 89 ca                    	movq	%r9, %rdx
140b2c987: e8 94 72 ae ff              	callq	0x140613c20 <.text+0x612c20>
140b2c98c: 48 8b 95 c8 00 00 00        	movq	0xc8(%rbp), %rdx
140b2c993: 48 85 d2                    	testq	%rdx, %rdx
140b2c996: 74 2a                       	je	0x140b2c9c2 <.text+0xb2b9c2>
140b2c998: 48 8d 04 95 13 00 00 00     	leaq	0x13(,%rdx,4), %rax
140b2c9a0: 48 83 e0 f0                 	andq	$-0x10, %rax
140b2c9a4: 48 01 c2                    	addq	%rax, %rdx
140b2c9a7: 48 83 c2 11                 	addq	$0x11, %rdx
140b2c9ab: 74 15                       	je	0x140b2c9c2 <.text+0xb2b9c2>
140b2c9ad: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
140b2c9b4: 48 29 c1                    	subq	%rax, %rcx
140b2c9b7: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b2c9bd: e8 5e 72 ae ff              	callq	0x140613c20 <.text+0x612c20>
140b2c9c2: 48 8b 95 28 01 00 00        	movq	0x128(%rbp), %rdx
140b2c9c9: 48 85 d2                    	testq	%rdx, %rdx
140b2c9cc: 0f 84 4e fc ff ff           	je	0x140b2c620 <.text+0xb2b620>
140b2c9d2: 48 8d 04 95 13 00 00 00     	leaq	0x13(,%rdx,4), %rax
140b2c9da: 48 83 e0 f0                 	andq	$-0x10, %rax
140b2c9de: 48 01 c2                    	addq	%rax, %rdx
140b2c9e1: 48 83 c2 11                 	addq	$0x11, %rdx
140b2c9e5: 0f 84 35 fc ff ff           	je	0x140b2c620 <.text+0xb2b620>
140b2c9eb: 48 8b 8d 20 01 00 00        	movq	0x120(%rbp), %rcx
140b2c9f2: 48 29 c1                    	subq	%rax, %rcx
140b2c9f5: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b2c9fb: e8 20 72 ae ff              	callq	0x140613c20 <.text+0x612c20>
140b2ca00: e9 1b fc ff ff              	jmp	0x140b2c620 <.text+0xb2b620>
140b2ca05: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
140b2ca0c: e8 8f b4 d6 ff              	callq	0x140897ea0 <.text+0x896ea0>
140b2ca11: 90                          	nop
140b2ca12: 48 81 c4 68 03 00 00        	addq	$0x368, %rsp            # imm = 0x368
140b2ca19: 5b                          	popq	%rbx
140b2ca1a: 5f                          	popq	%rdi
140b2ca1b: 5e                          	popq	%rsi
140b2ca1c: 41 5c                       	popq	%r12
140b2ca1e: 41 5d                       	popq	%r13
140b2ca20: 41 5e                       	popq	%r14
140b2ca22: 41 5f                       	popq	%r15
140b2ca24: 5d                          	popq	%rbp
140b2ca25: c3                          	retq
140b2ca26: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140b2ca30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b2ca35: 55                          	pushq	%rbp
140b2ca36: 41 57                       	pushq	%r15
140b2ca38: 41 56                       	pushq	%r14
140b2ca3a: 41 55                       	pushq	%r13
140b2ca3c: 41 54                       	pushq	%r12
140b2ca3e: 56                          	pushq	%rsi
140b2ca3f: 57                          	pushq	%rdi
140b2ca40: 53                          	pushq	%rbx
140b2ca41: 48 83 ec 28                 	subq	$0x28, %rsp
140b2ca45: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b2ca4c: 48 c7 85 80 02 00 00 00 00 00 00    	movq	$0x0, 0x280(%rbp)
140b2ca57: 48 83 c4 28                 	addq	$0x28, %rsp
140b2ca5b: 5b                          	popq	%rbx
140b2ca5c: 5f                          	popq	%rdi
140b2ca5d: 5e                          	popq	%rsi
140b2ca5e: 41 5c                       	popq	%r12
140b2ca60: 41 5d                       	popq	%r13
140b2ca62: 41 5e                       	popq	%r14
140b2ca64: 41 5f                       	popq	%r15
140b2ca66: 5d                          	popq	%rbp
140b2ca67: c3                          	retq
140b2ca68: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140b2ca70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b2ca75: 55                          	pushq	%rbp
140b2ca76: 41 57                       	pushq	%r15
140b2ca78: 41 56                       	pushq	%r14
140b2ca7a: 41 55                       	pushq	%r13
140b2ca7c: 41 54                       	pushq	%r12
140b2ca7e: 56                          	pushq	%rsi
140b2ca7f: 57                          	pushq	%rdi
140b2ca80: 53                          	pushq	%rbx
140b2ca81: 48 83 ec 28                 	subq	$0x28, %rsp
140b2ca85: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b2ca8c: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
140b2ca93: e8 08 b4 d6 ff              	callq	0x140897ea0 <.text+0x896ea0>
140b2ca98: 90                          	nop
140b2ca99: 48 83 c4 28                 	addq	$0x28, %rsp
140b2ca9d: 5b                          	popq	%rbx
140b2ca9e: 5f                          	popq	%rdi
140b2ca9f: 5e                          	popq	%rsi
140b2caa0: 41 5c                       	popq	%r12
140b2caa2: 41 5d                       	popq	%r13
140b2caa4: 41 5e                       	popq	%r14
140b2caa6: 41 5f                       	popq	%r15
140b2caa8: 5d                          	popq	%rbp
140b2caa9: c3                          	retq
140b2caaa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140b2cab0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b2cab5: 55                          	pushq	%rbp
140b2cab6: 41 57                       	pushq	%r15
140b2cab8: 41 56                       	pushq	%r14
140b2caba: 41 55                       	pushq	%r13
140b2cabc: 41 54                       	pushq	%r12
140b2cabe: 56                          	pushq	%rsi
140b2cabf: 57                          	pushq	%rdi
140b2cac0: 53                          	pushq	%rbx
140b2cac1: 48 83 ec 28                 	subq	$0x28, %rsp
140b2cac5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b2cacc: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
140b2cad3: e8 b8 7f ad ff              	callq	0x140604a90 <.text+0x603a90>
140b2cad8: 90                          	nop
140b2cad9: 48 83 c4 28                 	addq	$0x28, %rsp
140b2cadd: 5b                          	popq	%rbx
140b2cade: 5f                          	popq	%rdi
140b2cadf: 5e                          	popq	%rsi
140b2cae0: 41 5c                       	popq	%r12
140b2cae2: 41 5d                       	popq	%r13
140b2cae4: 41 5e                       	popq	%r14
140b2cae6: 41 5f                       	popq	%r15
140b2cae8: 5d                          	popq	%rbp
140b2cae9: c3                          	retq
140b2caea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140b2caf0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b2caf5: 55                          	pushq	%rbp
140b2caf6: 41 57                       	pushq	%r15
140b2caf8: 41 56                       	pushq	%r14
140b2cafa: 41 55                       	pushq	%r13
140b2cafc: 41 54                       	pushq	%r12
140b2cafe: 56                          	pushq	%rsi
140b2caff: 57                          	pushq	%rdi
140b2cb00: 53                          	pushq	%rbx
140b2cb01: 48 83 ec 28                 	subq	$0x28, %rsp
140b2cb05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b2cb0c: 48 c7 85 e8 01 00 00 02 00 00 00    	movq	$0x2, 0x1e8(%rbp)
140b2cb17: 48 83 c4 28                 	addq	$0x28, %rsp
140b2cb1b: 5b                          	popq	%rbx
140b2cb1c: 5f                          	popq	%rdi
140b2cb1d: 5e                          	popq	%rsi
140b2cb1e: 41 5c                       	popq	%r12
140b2cb20: 41 5d                       	popq	%r13
140b2cb22: 41 5e                       	popq	%r14
140b2cb24: 41 5f                       	popq	%r15
140b2cb26: 5d                          	popq	%rbp
140b2cb27: c3                          	retq
140b2cb28: cc                          	int3
140b2cb29: cc                          	int3
140b2cb2a: cc                          	int3
140b2cb2b: cc                          	int3
140b2cb2c: cc                          	int3
140b2cb2d: cc                          	int3
140b2cb2e: cc                          	int3
140b2cb2f: cc                          	int3
