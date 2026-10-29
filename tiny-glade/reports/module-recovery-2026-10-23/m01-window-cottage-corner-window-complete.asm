
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407f66b0: 55                          	pushq	%rbp
1407f66b1: 41 57                       	pushq	%r15
1407f66b3: 41 56                       	pushq	%r14
1407f66b5: 41 55                       	pushq	%r13
1407f66b7: 41 54                       	pushq	%r12
1407f66b9: 56                          	pushq	%rsi
1407f66ba: 57                          	pushq	%rdi
1407f66bb: 53                          	pushq	%rbx
1407f66bc: 48 81 ec 48 03 00 00        	subq	$0x348, %rsp            # imm = 0x348
1407f66c3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407f66cb: 44 0f 29 bd b0 02 00 00     	movaps	%xmm15, 0x2b0(%rbp)
1407f66d3: 44 0f 29 b5 a0 02 00 00     	movaps	%xmm14, 0x2a0(%rbp)
1407f66db: 44 0f 29 ad 90 02 00 00     	movaps	%xmm13, 0x290(%rbp)
1407f66e3: 44 0f 29 a5 80 02 00 00     	movaps	%xmm12, 0x280(%rbp)
1407f66eb: 44 0f 29 9d 70 02 00 00     	movaps	%xmm11, 0x270(%rbp)
1407f66f3: 44 0f 29 95 60 02 00 00     	movaps	%xmm10, 0x260(%rbp)
1407f66fb: 44 0f 29 8d 50 02 00 00     	movaps	%xmm9, 0x250(%rbp)
1407f6703: 44 0f 29 85 40 02 00 00     	movaps	%xmm8, 0x240(%rbp)
1407f670b: 0f 29 bd 30 02 00 00        	movaps	%xmm7, 0x230(%rbp)
1407f6712: 0f 29 b5 20 02 00 00        	movaps	%xmm6, 0x220(%rbp)
1407f6719: 48 c7 85 18 02 00 00 fe ff ff ff    	movq	$-0x2, 0x218(%rbp)
1407f6724: 4d 89 cd                    	movq	%r9, %r13
1407f6727: 4d 89 c7                    	movq	%r8, %r15
1407f672a: 48 89 d3                    	movq	%rdx, %rbx
1407f672d: 49 89 ce                    	movq	%rcx, %r14
1407f6730: e8 cb 02 42 00              	callq	0x140c16a00 <.text+0xc15a00>
1407f6735: 88 85 16 02 00 00           	movb	%al, 0x216(%rbp)
1407f673b: 84 c0                       	testb	%al, %al
1407f673d: 0f 84 20 02 00 00           	je	0x1407f6963 <.text+0x7f5963>
1407f6743: 41 bc 5a 00 00 00           	movl	$0x5a, %r12d
1407f6749: b9 5a 00 00 00              	movl	$0x5a, %ecx
1407f674e: e8 dd fa ff ff              	callq	0x1407f6230 <.text+0x7f5230>
1407f6753: 48 8d 3d 46 57 29 02        	leaq	0x2295746(%rip), %rdi   # 0x142a8bea0
1407f675a: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f675e: 0f 85 a1 00 00 00           	jne	0x1407f6805 <.text+0x7f5805>
1407f6764: 49 89 d1                    	movq	%rdx, %r9
1407f6767: 48 85 d2                    	testq	%rdx, %rdx
1407f676a: 74 36                       	je	0x1407f67a2 <.text+0x7f57a2>
1407f676c: 49 83 f9 5a                 	cmpq	$0x5a, %r9
1407f6770: 73 2e                       	jae	0x1407f67a0 <.text+0x7f57a0>
1407f6772: 48 8d 05 27 57 29 02        	leaq	0x2295727(%rip), %rax   # 0x142a8bea0
1407f6779: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1407f677e: 7f 22                       	jg	0x1407f67a2 <.text+0x7f57a2>
1407f6780: 48 8d 05 f1 57 29 02        	leaq	0x22957f1(%rip), %rax   # 0x142a8bf78
1407f6787: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f678c: 48 8d 0d 0d 57 29 02        	leaq	0x229570d(%rip), %rcx   # 0x142a8bea0
1407f6793: ba 5a 00 00 00              	movl	$0x5a, %edx
1407f6798: 45 31 c0                    	xorl	%r8d, %r8d
1407f679b: e8 a0 31 0e 02              	callq	0x1428d9940 <.text+0x28d8940>
1407f67a0: 75 de                       	jne	0x1407f6780 <.text+0x7f5780>
1407f67a2: 4c 89 c9                    	movq	%r9, %rcx
1407f67a5: e8 86 fa ff ff              	callq	0x1407f6230 <.text+0x7f5230>
1407f67aa: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f67ae: 75 55                       	jne	0x1407f6805 <.text+0x7f5805>
1407f67b0: 49 89 d0                    	movq	%rdx, %r8
1407f67b3: 49 83 c0 02                 	addq	$0x2, %r8
1407f67b7: 74 39                       	je	0x1407f67f2 <.text+0x7f57f2>
1407f67b9: 49 83 f8 5a                 	cmpq	$0x5a, %r8
1407f67bd: 73 31                       	jae	0x1407f67f0 <.text+0x7f57f0>
1407f67bf: 48 8d 05 da 56 29 02        	leaq	0x22956da(%rip), %rax   # 0x142a8bea0
1407f67c6: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1407f67cb: 7f 25                       	jg	0x1407f67f2 <.text+0x7f57f2>
1407f67cd: 48 8d 05 bc 57 29 02        	leaq	0x22957bc(%rip), %rax   # 0x142a8bf90
1407f67d4: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f67d9: 48 8d 0d c0 56 29 02        	leaq	0x22956c0(%rip), %rcx   # 0x142a8bea0
1407f67e0: ba 5a 00 00 00              	movl	$0x5a, %edx
1407f67e5: 41 b9 5a 00 00 00           	movl	$0x5a, %r9d
1407f67eb: e8 50 31 0e 02              	callq	0x1428d9940 <.text+0x28d8940>
1407f67f0: 75 db                       	jne	0x1407f67cd <.text+0x7f57cd>
1407f67f2: 41 bc 58 00 00 00           	movl	$0x58, %r12d
1407f67f8: 49 29 d4                    	subq	%rdx, %r12
1407f67fb: 48 8d 3d 9e 56 29 02        	leaq	0x229569e(%rip), %rdi   # 0x142a8bea0
1407f6802: 4c 01 c7                    	addq	%r8, %rdi
1407f6805: 4c 89 ad 60 01 00 00        	movq	%r13, 0x160(%rbp)
1407f680c: 4c 8d 05 21 58 29 02        	leaq	0x2295821(%rip), %r8    # 0x142a8c034
1407f6813: 4c 8d 2d d4 57 29 02        	leaq	0x22957d4(%rip), %r13   # 0x142a8bfee
1407f681a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1407f6820: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1407f6825: 85 c0                       	testl	%eax, %eax
1407f6827: 78 17                       	js	0x1407f6840 <.text+0x7f5840>
1407f6829: 49 ff c8                    	decq	%r8
1407f682c: 83 f8 5c                    	cmpl	$0x5c, %eax
1407f682f: 75 63                       	jne	0x1407f6894 <.text+0x7f5894>
1407f6831: eb 72                       	jmp	0x1407f68a5 <.text+0x7f58a5>
1407f6833: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407f6840: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1407f6845: 80 f9 c0                    	cmpb	$-0x40, %cl
1407f6848: 7d 23                       	jge	0x1407f686d <.text+0x7f586d>
1407f684a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1407f684f: 80 fa c0                    	cmpb	$-0x40, %dl
1407f6852: 7d 22                       	jge	0x1407f6876 <.text+0x7f5876>
1407f6854: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1407f6859: 49 83 c0 fc                 	addq	$-0x4, %r8
1407f685d: 41 83 e1 07                 	andl	$0x7, %r9d
1407f6861: 41 c1 e1 06                 	shll	$0x6, %r9d
1407f6865: 83 e2 3f                    	andl	$0x3f, %edx
1407f6868: 44 09 ca                    	orl	%r9d, %edx
1407f686b: eb 10                       	jmp	0x1407f687d <.text+0x7f587d>
1407f686d: 49 83 c0 fe                 	addq	$-0x2, %r8
1407f6871: 83 e1 1f                    	andl	$0x1f, %ecx
1407f6874: eb 0f                       	jmp	0x1407f6885 <.text+0x7f5885>
1407f6876: 49 83 c0 fd                 	addq	$-0x3, %r8
1407f687a: 83 e2 0f                    	andl	$0xf, %edx
1407f687d: c1 e2 06                    	shll	$0x6, %edx
1407f6880: 83 e1 3f                    	andl	$0x3f, %ecx
1407f6883: 09 d1                       	orl	%edx, %ecx
1407f6885: c1 e1 06                    	shll	$0x6, %ecx
1407f6888: 24 3f                       	andb	$0x3f, %al
1407f688a: 0f b6 c0                    	movzbl	%al, %eax
1407f688d: 09 c8                       	orl	%ecx, %eax
1407f688f: 83 f8 5c                    	cmpl	$0x5c, %eax
1407f6892: 74 11                       	je	0x1407f68a5 <.text+0x7f58a5>
1407f6894: 83 f8 2f                    	cmpl	$0x2f, %eax
1407f6897: 74 0c                       	je	0x1407f68a5 <.text+0x7f58a5>
1407f6899: 4d 39 e8                    	cmpq	%r13, %r8
1407f689c: 75 82                       	jne	0x1407f6820 <.text+0x7f5820>
1407f689e: be 46 00 00 00              	movl	$0x46, %esi
1407f68a3: eb 4c                       	jmp	0x1407f68f1 <.text+0x7f58f1>
1407f68a5: 4c 8d 2d 42 57 29 02        	leaq	0x2295742(%rip), %r13   # 0x142a8bfee
1407f68ac: 4d 29 e8                    	subq	%r13, %r8
1407f68af: 49 ff c0                    	incq	%r8
1407f68b2: 74 32                       	je	0x1407f68e6 <.text+0x7f58e6>
1407f68b4: 49 83 f8 46                 	cmpq	$0x46, %r8
1407f68b8: 73 2a                       	jae	0x1407f68e4 <.text+0x7f58e4>
1407f68ba: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
1407f68bf: 7f 25                       	jg	0x1407f68e6 <.text+0x7f58e6>
1407f68c1: 48 8d 05 90 56 29 02        	leaq	0x2295690(%rip), %rax   # 0x142a8bf58
1407f68c8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f68cd: 48 8d 0d 1a 57 29 02        	leaq	0x229571a(%rip), %rcx   # 0x142a8bfee
1407f68d4: ba 46 00 00 00              	movl	$0x46, %edx
1407f68d9: 41 b9 46 00 00 00           	movl	$0x46, %r9d
1407f68df: e8 5c 30 0e 02              	callq	0x1428d9940 <.text+0x28d8940>
1407f68e4: 75 db                       	jne	0x1407f68c1 <.text+0x7f58c1>
1407f68e6: be 46 00 00 00              	movl	$0x46, %esi
1407f68eb: 4c 29 c6                    	subq	%r8, %rsi
1407f68ee: 4d 01 c5                    	addq	%r8, %r13
1407f68f1: e8 3a 18 42 00              	callq	0x140c18130 <.text+0xc17130>
1407f68f6: 48 89 c1                    	movq	%rax, %rcx
1407f68f9: 48 8b 00                    	movq	(%rax), %rax
1407f68fc: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f6900: 0f 85 98 1b 00 00           	jne	0x1407f849e <.text+0x7f749e>
1407f6906: 48 83 c1 08                 	addq	$0x8, %rcx
1407f690a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407f690e: 0f 85 e5 1b 00 00           	jne	0x1407f84f9 <.text+0x7f74f9>
1407f6914: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407f691b: 48 89 8d 30 01 00 00        	movq	%rcx, 0x130(%rbp)
1407f6922: 48 83 c1 08                 	addq	$0x8, %rcx
1407f6926: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
1407f692b: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1407f6934: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1407f693d: 48 89 fa                    	movq	%rdi, %rdx
1407f6940: 4d 89 e0                    	movq	%r12, %r8
1407f6943: 4d 89 e9                    	movq	%r13, %r9
1407f6946: e8 f5 03 42 00              	callq	0x140c16d40 <.text+0xc15d40>
1407f694b: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
1407f6952: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
1407f6959: 48 ff 00                    	incq	(%rax)
1407f695c: 4c 8b ad 60 01 00 00        	movq	0x160(%rbp), %r13
1407f6963: 4c 89 f1                    	movq	%r14, %rcx
1407f6966: e8 05 01 33 00              	callq	0x140b26a70 <.text+0xb25a70>
1407f696b: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1407f6972: 48 89 95 28 01 00 00        	movq	%rdx, 0x128(%rbp)
1407f6979: 48 8b 13                    	movq	(%rbx), %rdx
1407f697c: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f6983: e8 78 47 22 00              	callq	0x140a1b100 <.text+0xa1a100>
1407f6988: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
1407f698f: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
1407f6996: 4c 8b b5 c0 01 00 00        	movq	0x1c0(%rbp), %r14
1407f699d: 48 8b b5 c8 01 00 00        	movq	0x1c8(%rbp), %rsi
1407f69a4: 0f b7 85 d8 01 00 00        	movzwl	0x1d8(%rbp), %eax
1407f69ab: 4c 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %r8
1407f69b2: 4d 8b 4d 00                 	movq	(%r13), %r9
1407f69b6: 4c 89 4d 30                 	movq	%r9, 0x30(%rbp)
1407f69ba: 4d 8b 0f                    	movq	(%r15), %r9
1407f69bd: 4c 89 4d 38                 	movq	%r9, 0x38(%rbp)
1407f69c1: 44 8b 4a 1c                 	movl	0x1c(%rdx), %r9d
1407f69c5: 44 89 8d a4 00 00 00        	movl	%r9d, 0xa4(%rbp)
1407f69cc: 4c 8b 0a                    	movq	(%rdx), %r9
1407f69cf: 4c 89 4d 48                 	movq	%r9, 0x48(%rbp)
1407f69d3: 48 8b 52 10                 	movq	0x10(%rdx), %rdx
1407f69d7: 48 89 55 40                 	movq	%rdx, 0x40(%rbp)
1407f69db: 48 8b 09                    	movq	(%rcx), %rcx
1407f69de: 48 89 4d 50                 	movq	%rcx, 0x50(%rbp)
1407f69e2: 4c 89 85 a8 00 00 00        	movq	%r8, 0xa8(%rbp)
1407f69e9: 4d 85 c0                    	testq	%r8, %r8
1407f69ec: 0f 84 fc 18 00 00           	je	0x1407f82ee <.text+0x7f72ee>
1407f69f2: 66 85 c0                    	testw	%ax, %ax
1407f69f5: 74 19                       	je	0x1407f6a10 <.text+0x7f5a10>
1407f69f7: 4d 85 f6                    	testq	%r14, %r14
1407f69fa: 0f 84 ee 18 00 00           	je	0x1407f82ee <.text+0x7f72ee>
1407f6a00: 8d 48 ff                    	leal	-0x1(%rax), %ecx
1407f6a03: 21 c1                       	andl	%eax, %ecx
1407f6a05: 89 8d 5c 01 00 00           	movl	%ecx, 0x15c(%rbp)
1407f6a0b: eb 30                       	jmp	0x1407f6a3d <.text+0x7f5a3d>
1407f6a0d: 0f 1f 00                    	nopl	(%rax)
1407f6a10: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
1407f6a14: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1407f6a18: 49 81 c6 80 f6 ff ff        	addq	$-0x980, %r14           # imm = 0xF680
1407f6a1f: 48 83 c6 10                 	addq	$0x10, %rsi
1407f6a23: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1407f6a28: 74 e6                       	je	0x1407f6a10 <.text+0x7f5a10>
1407f6a2a: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1407f6a2f: 29 c1                       	subl	%eax, %ecx
1407f6a31: f7 d0                       	notl	%eax
1407f6a33: 89 c2                       	movl	%eax, %edx
1407f6a35: 21 ca                       	andl	%ecx, %edx
1407f6a37: 89 95 5c 01 00 00           	movl	%edx, 0x15c(%rbp)
1407f6a3d: f3 0f bc c0                 	tzcntl	%eax, %eax
1407f6a41: 48 f7 d8                    	negq	%rax
1407f6a44: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
1407f6a4b: 49 8d 14 06                 	leaq	(%r14,%rax), %rdx
1407f6a4f: 48 81 c2 70 ff ff ff        	addq	$-0x90, %rdx
1407f6a56: 49 8b 84 06 68 ff ff ff     	movq	-0x98(%r14,%rax), %rax
1407f6a5e: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1407f6a65: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f6a6c: e8 ff 45 22 00              	callq	0x140a1b070 <.text+0xa1a070>
1407f6a71: 48 ff 8d a8 00 00 00        	decq	0xa8(%rbp)
1407f6a78: 4c 8b a5 c0 01 00 00        	movq	0x1c0(%rbp), %r12
1407f6a7f: 4c 8b ad c8 01 00 00        	movq	0x1c8(%rbp), %r13
1407f6a86: 0f b7 8d d8 01 00 00        	movzwl	0x1d8(%rbp), %ecx
1407f6a8d: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
1407f6a94: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1407f6a9b: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
1407f6aa2: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
1407f6aa9: 48 83 c0 50                 	addq	$0x50, %rax
1407f6aad: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
1407f6ab1: 48 83 bd 60 01 00 00 00     	cmpq	$0x0, 0x160(%rbp)
1407f6ab9: 0f 84 12 18 00 00           	je	0x1407f82d1 <.text+0x7f72d1>
1407f6abf: 66 85 c9                    	testw	%cx, %cx
1407f6ac2: 74 1c                       	je	0x1407f6ae0 <.text+0x7f5ae0>
1407f6ac4: 8d 41 ff                    	leal	-0x1(%rcx), %eax
1407f6ac7: 21 c8                       	andl	%ecx, %eax
1407f6ac9: 89 85 74 01 00 00           	movl	%eax, 0x174(%rbp)
1407f6acf: 4d 85 e4                    	testq	%r12, %r12
1407f6ad2: 75 39                       	jne	0x1407f6b0d <.text+0x7f5b0d>
1407f6ad4: e9 f8 17 00 00              	jmp	0x1407f82d1 <.text+0x7f72d1>
1407f6ad9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1407f6ae0: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
1407f6ae6: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
1407f6aea: 49 83 c4 c0                 	addq	$-0x40, %r12
1407f6aee: 49 83 c5 10                 	addq	$0x10, %r13
1407f6af2: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
1407f6af8: 74 e6                       	je	0x1407f6ae0 <.text+0x7f5ae0>
1407f6afa: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
1407f6aff: 29 c8                       	subl	%ecx, %eax
1407f6b01: f7 d1                       	notl	%ecx
1407f6b03: 89 ca                       	movl	%ecx, %edx
1407f6b05: 21 c2                       	andl	%eax, %edx
1407f6b07: 89 95 74 01 00 00           	movl	%edx, 0x174(%rbp)
1407f6b0d: f3 0f bc c1                 	tzcntl	%ecx, %eax
1407f6b11: c1 e0 02                    	shll	$0x2, %eax
1407f6b14: 4c 89 e1                    	movq	%r12, %rcx
1407f6b17: 48 29 c1                    	subq	%rax, %rcx
1407f6b1a: 8b 41 fc                    	movl	-0x4(%rcx), %eax
1407f6b1d: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
1407f6b24: 89 85 c0 01 00 00           	movl	%eax, 0x1c0(%rbp)
1407f6b2a: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
1407f6b31: 48 83 78 48 00              	cmpq	$0x0, 0x48(%rax)
1407f6b36: 0f 84 68 18 00 00           	je	0x1407f83a4 <.text+0x7f73a4>
1407f6b3c: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
1407f6b40: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407f6b47: e8 84 a3 db ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1407f6b4c: 48 ff 8d 60 01 00 00        	decq	0x160(%rbp)
1407f6b53: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
1407f6b5a: 4c 8b 79 30                 	movq	0x30(%rcx), %r15
1407f6b5e: 48 8b 49 38                 	movq	0x38(%rcx), %rcx
1407f6b62: 49 89 cb                    	movq	%rcx, %r11
1407f6b65: 49 21 c3                    	andq	%rax, %r11
1407f6b68: 48 c1 e8 39                 	shrq	$0x39, %rax
1407f6b6c: 66 0f 6e c0                 	movd	%eax, %xmm0
1407f6b70: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407f6b74: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407f6b79: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f6b7e: 49 8d 47 b0                 	leaq	-0x50(%r15), %rax
1407f6b82: 45 31 c0                    	xorl	%r8d, %r8d
1407f6b85: f3 43 0f 6f 0c 1f           	movdqu	(%r15,%r11), %xmm1
1407f6b8b: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407f6b8f: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407f6b93: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1407f6b98: 45 85 c9                    	testl	%r9d, %r9d
1407f6b9b: 74 33                       	je	0x1407f6bd0 <.text+0x7f5bd0>
1407f6b9d: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1407f6ba2: 4d 01 da                    	addq	%r11, %r10
1407f6ba5: 49 21 ca                    	andq	%rcx, %r10
1407f6ba8: 49 f7 da                    	negq	%r10
1407f6bab: 4b 8d 3c 92                 	leaq	(%r10,%r10,4), %rdi
1407f6baf: 48 c1 e7 04                 	shlq	$0x4, %rdi
1407f6bb3: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
1407f6bba: 3b 14 38                    	cmpl	(%rax,%rdi), %edx
1407f6bbd: 74 41                       	je	0x1407f6c00 <.text+0x7f5c00>
1407f6bbf: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1407f6bc3: 66 45 21 ca                 	andw	%r9w, %r10w
1407f6bc7: 45 89 d1                    	movl	%r10d, %r9d
1407f6bca: 75 d1                       	jne	0x1407f6b9d <.text+0x7f5b9d>
1407f6bcc: 0f 1f 40 00                 	nopl	(%rax)
1407f6bd0: 66 0f 74 0d b8 ed 12 02     	pcmpeqb	0x212edb8(%rip), %xmm1  # 0x142925990
1407f6bd8: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1407f6bdd: 45 85 c9                    	testl	%r9d, %r9d
1407f6be0: 0f 85 be 17 00 00           	jne	0x1407f83a4 <.text+0x7f73a4>
1407f6be6: 4d 01 c3                    	addq	%r8, %r11
1407f6be9: 49 83 c3 10                 	addq	$0x10, %r11
1407f6bed: 49 83 c0 10                 	addq	$0x10, %r8
1407f6bf1: 49 21 cb                    	andq	%rcx, %r11
1407f6bf4: eb 8f                       	jmp	0x1407f6b85 <.text+0x7f5b85>
1407f6bf6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407f6c00: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
1407f6c07: e8 64 fd 32 00              	callq	0x140b26970 <.text+0xb25970>
1407f6c0c: 8b 8d 74 01 00 00           	movl	0x174(%rbp), %ecx
1407f6c12: 3c 03                       	cmpb	$0x3, %al
1407f6c14: 0f 85 97 fe ff ff           	jne	0x1407f6ab1 <.text+0x7f5ab1>
1407f6c1a: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
1407f6c1e: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
1407f6c25: 4c 8d 05 0c 54 29 02        	leaq	0x229540c(%rip), %r8    # 0x142a8c038
1407f6c2c: e8 bf b2 29 00              	callq	0x140a91ef0 <.text+0xa90ef0>
1407f6c31: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
1407f6c38: 48 83 bd b8 01 00 00 00     	cmpq	$0x0, 0x1b8(%rbp)
1407f6c40: 0f 84 41 19 00 00           	je	0x1407f8587 <.text+0x7f7587>
1407f6c46: 49 8d 1c 3f                 	leaq	(%r15,%rdi), %rbx
1407f6c4a: 0f b6 43 ff                 	movzbl	-0x1(%rbx), %eax
1407f6c4e: 88 85 38 01 00 00           	movb	%al, 0x138(%rbp)
1407f6c54: 8b 43 e0                    	movl	-0x20(%rbx), %eax
1407f6c57: 89 85 c8 00 00 00           	movl	%eax, 0xc8(%rbp)
1407f6c5d: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1407f6c61: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
1407f6c65: 4c 8b 85 68 01 00 00        	movq	0x168(%rbp), %r8
1407f6c6c: 4c 8d 0d f5 53 29 02        	leaq	0x22953f5(%rip), %r9    # 0x142a8c068
1407f6c73: e8 a8 3d 22 00              	callq	0x140a1aa20 <.text+0xa19a20>
1407f6c78: 4c 89 b5 88 00 00 00        	movq	%r14, 0x88(%rbp)
1407f6c7f: 48 89 9d 08 02 00 00        	movq	%rbx, 0x208(%rbp)
1407f6c86: 44 0f b6 73 fc              	movzbl	-0x4(%rbx), %r14d
1407f6c8b: 4d 85 f6                    	testq	%r14, %r14
1407f6c8e: 74 28                       	je	0x1407f6cb8 <.text+0x7f5cb8>
1407f6c90: 44 89 f0                    	movl	%r14d, %eax
1407f6c93: c1 e0 04                    	shll	$0x4, %eax
1407f6c96: 48 8d 1c 40                 	leaq	(%rax,%rax,2), %rbx
1407f6c9a: 0f b6 05 68 df e3 02        	movzbl	0x2e3df68(%rip), %eax   # 0x143634c09
1407f6ca1: ba 10 00 00 00              	movl	$0x10, %edx
1407f6ca6: 48 89 d9                    	movq	%rbx, %rcx
1407f6ca9: e8 62 cf e1 ff              	callq	0x140613c10 <.text+0x612c10>
1407f6cae: 48 85 c0                    	testq	%rax, %rax
1407f6cb1: 75 0a                       	jne	0x1407f6cbd <.text+0x7f5cbd>
1407f6cb3: e9 e0 18 00 00              	jmp	0x1407f8598 <.text+0x7f7598>
1407f6cb8: b8 10 00 00 00              	movl	$0x10, %eax
1407f6cbd: 4c 89 b5 40 01 00 00        	movq	%r14, 0x140(%rbp)
1407f6cc4: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
1407f6ccb: 48 c7 85 50 01 00 00 00 00 00 00    	movq	$0x0, 0x150(%rbp)
1407f6cd6: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f6cdd: 48 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %rdx
1407f6ce4: e8 a7 3a 33 00              	callq	0x140b2a790 <.text+0xb29790>
1407f6ce9: 4c 8b b5 88 00 00 00        	movq	0x88(%rbp), %r14
1407f6cf0: 49 8d 14 3f                 	leaq	(%r15,%rdi), %rdx
1407f6cf4: 48 83 c2 b8                 	addq	$-0x48, %rdx
1407f6cf8: 83 bd c0 01 00 00 01        	cmpl	$0x1, 0x1c0(%rbp)
1407f6cff: 48 89 95 d0 00 00 00        	movq	%rdx, 0xd0(%rbp)
1407f6d06: 0f 85 bb 08 00 00           	jne	0x1407f75c7 <.text+0x7f65c7>
1407f6d0c: 48 8d 8d c4 01 00 00        	leaq	0x1c4(%rbp), %rcx
1407f6d13: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1407f6d17: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1407f6d1e: 0f 10 01                    	movups	(%rcx), %xmm0
1407f6d21: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
1407f6d28: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
1407f6d2f: 80 78 da 02                 	cmpb	$0x2, -0x26(%rax)
1407f6d33: 0f 83 31 18 00 00           	jae	0x1407f856a <.text+0x7f756a>
1407f6d39: 0f b6 48 d8                 	movzbl	-0x28(%rax), %ecx
1407f6d3d: e8 ae e9 32 00              	callq	0x140b256f0 <.text+0xb246f0>
1407f6d42: a8 01                       	testb	$0x1, %al
1407f6d44: 0f 84 f5 18 00 00           	je	0x1407f863f <.text+0x7f763f>
1407f6d4a: 48 89 d3                    	movq	%rdx, %rbx
1407f6d4d: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1407f6d54: e8 e7 33 4a 00              	callq	0x140c9a140 <.text+0xc99140>
1407f6d59: 0f 28 f8                    	movaps	%xmm0, %xmm7
1407f6d5c: 66 0f 6f f1                 	movdqa	%xmm1, %xmm6
1407f6d60: 48 8d 8d ec 00 00 00        	leaq	0xec(%rbp), %rcx
1407f6d67: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1407f6d6e: 49 89 d8                    	movq	%rbx, %r8
1407f6d71: e8 9a 34 4a 00              	callq	0x140c9a210 <.text+0xc99210>
1407f6d76: f2 44 0f 10 bd ec 00 00 00  	movsd	0xec(%rbp), %xmm15
1407f6d7f: f2 44 0f 10 9d f4 00 00 00  	movsd	0xf4(%rbp), %xmm11
1407f6d88: f3 0f 11 bd 80 00 00 00     	movss	%xmm7, 0x80(%rbp)
1407f6d90: f3 0f 11 b5 84 00 00 00     	movss	%xmm6, 0x84(%rbp)
1407f6d98: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
1407f6d9f: 44 8a 78 fc                 	movb	-0x4(%rax), %r15b
1407f6da3: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
1407f6daa: b1 03                       	movb	$0x3, %cl
1407f6dac: e8 9f c4 03 00              	callq	0x140833250 <.text+0x832250>
1407f6db1: 48 8b 9d 28 01 00 00        	movq	0x128(%rbp), %rbx
1407f6db8: 44 89 f9                    	movl	%r15d, %ecx
1407f6dbb: 89 c2                       	movl	%eax, %edx
1407f6dbd: e8 ce ee 32 00              	callq	0x140b25c90 <.text+0xb24c90>
1407f6dc2: 88 85 90 01 00 00           	movb	%al, 0x190(%rbp)
1407f6dc8: 88 95 91 01 00 00           	movb	%dl, 0x191(%rbp)
1407f6dce: 48 89 d9                    	movq	%rbx, %rcx
1407f6dd1: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1407f6dd8: e8 b3 61 a6 00              	callq	0x14125cf90 <.text+0x125bf90>
1407f6ddd: 48 85 c0                    	testq	%rax, %rax
1407f6de0: 0f 84 cb 17 00 00           	je	0x1407f85b1 <.text+0x7f75b1>
1407f6de6: f3 0f 10 40 30              	movss	0x30(%rax), %xmm0
1407f6deb: f3 0f 5e 05 01 79 13 02     	divss	0x2137901(%rip), %xmm0  # 0x14292e6f4
1407f6df3: 0f 28 4d f0                 	movaps	-0x10(%rbp), %xmm1
1407f6df7: 66 0f 6e 75 f4              	movd	-0xc(%rbp), %xmm6
1407f6dfc: f3 44 0f 12 e0              	movsldup	%xmm0, %xmm12           # xmm12 = xmm0[0,0,2,2]
1407f6e01: 45 0f 28 c7                 	movaps	%xmm15, %xmm8
1407f6e05: 45 0f 59 c4                 	mulps	%xmm12, %xmm8
1407f6e09: f3 44 0f 10 6d f8           	movss	-0x8(%rbp), %xmm13
1407f6e0f: 44 0f 16 e9                 	movlhps	%xmm1, %xmm13           # xmm13 = xmm13[0],xmm1[0]
1407f6e13: 44 0f c6 e9 e2              	shufps	$0xe2, %xmm1, %xmm13    # xmm13 = xmm13[2,0],xmm1[2,3]
1407f6e18: 45 0f 58 c5                 	addps	%xmm13, %xmm8
1407f6e1c: 45 0f 59 e3                 	mulps	%xmm11, %xmm12
1407f6e20: 45 0f 58 e5                 	addps	%xmm13, %xmm12
1407f6e24: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1407f6e28: 0f 28 15 b1 ea 12 02        	movaps	0x212eab1(%rip), %xmm2  # 0x1429258e0
1407f6e2f: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407f6e32: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407f6e36: 0f 59 ca                    	mulps	%xmm2, %xmm1
1407f6e39: 0f 58 c8                    	addps	%xmm0, %xmm1
1407f6e3c: 0f 13 8d 78 01 00 00        	movlps	%xmm1, 0x178(%rbp)
1407f6e43: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1407f6e4a: 48 8d 95 78 01 00 00        	leaq	0x178(%rbp), %rdx
1407f6e51: 66 0f 6f d6                 	movdqa	%xmm6, %xmm2
1407f6e55: e8 86 5f 4a 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1407f6e5a: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
1407f6e61: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
1407f6e68: e8 53 5f 4a 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407f6e6d: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f6e74: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
1407f6e7b: e8 50 a3 49 00              	callq	0x140c911d0 <.text+0xc901d0>
1407f6e80: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
1407f6e87: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407f6e8b: 48 ff c8                    	decq	%rax
1407f6e8e: 48 83 f8 03                 	cmpq	$0x3, %rax
1407f6e92: 0f 83 44 16 00 00           	jae	0x1407f84dc <.text+0x7f74dc>
1407f6e98: 44 0f 28 95 c0 01 00 00     	movaps	0x1c0(%rbp), %xmm10
1407f6ea0: f2 0f 10 bd 90 01 00 00     	movsd	0x190(%rbp), %xmm7
1407f6ea8: f3 44 0f 10 8d 98 01 00 00  	movss	0x198(%rbp), %xmm9
1407f6eb1: 44 0f 59 2d 87 4d 29 02     	mulps	0x2294d87(%rip), %xmm13 # 0x142a8bc40
1407f6eb9: 44 0f 59 05 8f 4d 29 02     	mulps	0x2294d8f(%rip), %xmm8  # 0x142a8bc50
1407f6ec1: 45 0f 58 c5                 	addps	%xmm13, %xmm8
1407f6ec5: 44 0f 13 85 10 01 00 00     	movlps	%xmm8, 0x110(%rbp)
1407f6ecd: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1407f6ed4: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
1407f6edb: 66 0f 6f d6                 	movdqa	%xmm6, %xmm2
1407f6edf: e8 fc 5e 4a 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1407f6ee4: f3 41 0f 16 c7              	movshdup	%xmm15, %xmm0           # xmm0 = xmm15[1,1,3,3]
1407f6ee9: 0f 57 05 00 ea 12 02        	xorps	0x212ea00(%rip), %xmm0  # 0x1429258f0
1407f6ef0: f3 0f 11 85 78 01 00 00     	movss	%xmm0, 0x178(%rbp)
1407f6ef8: f3 44 0f 11 bd 7c 01 00 00  	movss	%xmm15, 0x17c(%rbp)
1407f6f01: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
1407f6f08: 48 8d 95 78 01 00 00        	leaq	0x178(%rbp), %rdx
1407f6f0f: e8 ac 5e 4a 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407f6f14: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f6f1b: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
1407f6f22: e8 a9 a2 49 00              	callq	0x140c911d0 <.text+0xc901d0>
1407f6f27: 44 0f 28 85 c0 01 00 00     	movaps	0x1c0(%rbp), %xmm8
1407f6f2f: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
1407f6f36: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407f6f3d: 8b 85 98 01 00 00           	movl	0x198(%rbp), %eax
1407f6f43: 89 85 c8 01 00 00           	movl	%eax, 0x1c8(%rbp)
1407f6f49: 48 8b bd 50 01 00 00        	movq	0x150(%rbp), %rdi
1407f6f50: 48 3b bd 40 01 00 00        	cmpq	0x140(%rbp), %rdi
1407f6f57: 75 13                       	jne	0x1407f6f6c <.text+0x7f5f6c>
1407f6f59: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1407f6f60: 48 8d 15 71 51 29 02        	leaq	0x2295171(%rip), %rdx   # 0x142a8c0d8
1407f6f67: e8 04 45 03 00              	callq	0x14082b470 <.text+0x82a470>
1407f6f6c: 48 8b 85 48 01 00 00        	movq	0x148(%rbp), %rax
1407f6f73: 48 8d 0c 7f                 	leaq	(%rdi,%rdi,2), %rcx
1407f6f77: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407f6f7b: 44 0f 29 04 08              	movaps	%xmm8, (%rax,%rcx)
1407f6f80: 48 8b 95 c0 01 00 00        	movq	0x1c0(%rbp), %rdx
1407f6f87: 48 89 54 08 10              	movq	%rdx, 0x10(%rax,%rcx)
1407f6f8c: 8b 95 c8 01 00 00           	movl	0x1c8(%rbp), %edx
1407f6f92: 89 54 08 18                 	movl	%edx, 0x18(%rax,%rcx)
1407f6f96: 48 ba 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rdx # imm = 0x3F8000003F800000
1407f6fa0: 48 89 54 08 1c              	movq	%rdx, 0x1c(%rax,%rcx)
1407f6fa5: c7 44 08 24 00 00 80 3f     	movl	$0x3f800000, 0x24(%rax,%rcx) # imm = 0x3F800000
1407f6fad: 48 ff c7                    	incq	%rdi
1407f6fb0: 48 89 bd 50 01 00 00        	movq	%rdi, 0x150(%rbp)
1407f6fb7: 44 0f 59 25 91 4c 29 02     	mulps	0x2294c91(%rip), %xmm12 # 0x142a8bc50
1407f6fbf: 45 0f 58 ec                 	addps	%xmm12, %xmm13
1407f6fc3: 44 0f 13 ad 10 01 00 00     	movlps	%xmm13, 0x110(%rbp)
1407f6fcb: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1407f6fd2: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
1407f6fd9: 66 0f 6f d6                 	movdqa	%xmm6, %xmm2
1407f6fdd: e8 fe 5d 4a 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1407f6fe2: f3 41 0f 16 c3              	movshdup	%xmm11, %xmm0           # xmm0 = xmm11[1,1,3,3]
1407f6fe7: 0f 57 05 02 e9 12 02        	xorps	0x212e902(%rip), %xmm0  # 0x1429258f0
1407f6fee: f3 0f 11 85 78 01 00 00     	movss	%xmm0, 0x178(%rbp)
1407f6ff6: f3 44 0f 11 9d 7c 01 00 00  	movss	%xmm11, 0x17c(%rbp)
1407f6fff: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
1407f7006: 48 8d 95 78 01 00 00        	leaq	0x178(%rbp), %rdx
1407f700d: e8 ae 5d 4a 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407f7012: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f7019: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
1407f7020: e8 ab a1 49 00              	callq	0x140c911d0 <.text+0xc901d0>
1407f7025: 0f 28 b5 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm6
1407f702c: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
1407f7033: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407f703a: 8b 85 98 01 00 00           	movl	0x198(%rbp), %eax
1407f7040: 89 85 c8 01 00 00           	movl	%eax, 0x1c8(%rbp)
1407f7046: 48 8b bd 50 01 00 00        	movq	0x150(%rbp), %rdi
1407f704d: 48 3b bd 40 01 00 00        	cmpq	0x140(%rbp), %rdi
1407f7054: 75 13                       	jne	0x1407f7069 <.text+0x7f6069>
1407f7056: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1407f705d: 48 8d 15 8c 50 29 02        	leaq	0x229508c(%rip), %rdx   # 0x142a8c0f0
1407f7064: e8 07 44 03 00              	callq	0x14082b470 <.text+0x82a470>
1407f7069: 48 8b 85 48 01 00 00        	movq	0x148(%rbp), %rax
1407f7070: 48 8d 0c 7f                 	leaq	(%rdi,%rdi,2), %rcx
1407f7074: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407f7078: 0f 29 34 08                 	movaps	%xmm6, (%rax,%rcx)
1407f707c: 48 8b 95 c0 01 00 00        	movq	0x1c0(%rbp), %rdx
1407f7083: 48 89 54 08 10              	movq	%rdx, 0x10(%rax,%rcx)
1407f7088: 8b 95 c8 01 00 00           	movl	0x1c8(%rbp), %edx
1407f708e: 89 54 08 18                 	movl	%edx, 0x18(%rax,%rcx)
1407f7092: 48 ba 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rdx # imm = 0x3F8000003F800000
1407f709c: 48 89 54 08 1c              	movq	%rdx, 0x1c(%rax,%rcx)
1407f70a1: c7 44 08 24 00 00 80 3f     	movl	$0x3f800000, 0x24(%rax,%rcx) # imm = 0x3F800000
1407f70a9: 48 ff c7                    	incq	%rdi
1407f70ac: 48 89 bd 50 01 00 00        	movq	%rdi, 0x150(%rbp)
1407f70b3: 8b 85 a4 00 00 00           	movl	0xa4(%rbp), %eax
1407f70b9: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
1407f70bd: 89 01                       	movl	%eax, (%rcx)
1407f70bf: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
1407f70c6: e8 55 54 0c 00              	callq	0x1408bc520 <.text+0x8bb520>
1407f70cb: 49 89 d1                    	movq	%rdx, %r9
1407f70ce: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
1407f70d5: 0f b6 49 ff                 	movzbl	-0x1(%rcx), %ecx
1407f70d9: 88 4c 24 20                 	movb	%cl, 0x20(%rsp)
1407f70dd: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
1407f70e1: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
1407f70e8: 49 89 c0                    	movq	%rax, %r8
1407f70eb: e8 c0 e7 02 00              	callq	0x1408258b0 <.text+0x8248b0>
1407f70f0: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1407f70f7: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
1407f70fe: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407f7102: fe c8                       	decb	%al
1407f7104: 3c 03                       	cmpb	$0x3, %al
1407f7106: 0f 83 16 15 00 00           	jae	0x1407f8622 <.text+0x7f7622>
1407f710c: 0f b6 c0                    	movzbl	%al, %eax
1407f710f: 48 8d 0d 32 52 29 02        	leaq	0x2295232(%rip), %rcx   # 0x142a8c348
1407f7116: 4c 63 3c 81                 	movslq	(%rcx,%rax,4), %r15
1407f711a: 45 0f 28 c2                 	movaps	%xmm10, %xmm8
1407f711e: 44 0f 59 05 4a 4b 29 02     	mulps	0x2294b4a(%rip), %xmm8  # 0x142a8bc70
1407f7126: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
1407f712d: 48 8d 82 b0 00 00 00        	leaq	0xb0(%rdx), %rax
1407f7134: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
1407f713b: 48 8d 85 c4 01 00 00        	leaq	0x1c4(%rbp), %rax
1407f7142: c7 40 07 00 00 00 00        	movl	$0x0, 0x7(%rax)
1407f7149: 48 c7 40 ff 00 00 00 00     	movq	$0x0, -0x1(%rax)
1407f7151: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407f7158: 48 8b 8d c7 01 00 00        	movq	0x1c7(%rbp), %rcx
1407f715f: 48 89 8d 7f 01 00 00        	movq	%rcx, 0x17f(%rbp)
1407f7166: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
1407f716d: 48 8b 9a c0 00 00 00        	movq	0xc0(%rdx), %rbx
1407f7174: 48 3b 9a b0 00 00 00        	cmpq	0xb0(%rdx), %rbx
1407f717b: 75 13                       	jne	0x1407f7190 <.text+0x7f6190>
1407f717d: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
1407f7184: 48 8d 15 0d 50 29 02        	leaq	0x229500d(%rip), %rdx   # 0x142a8c198
1407f718b: e8 20 42 03 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407f7190: 48 8d 05 b1 51 29 02        	leaq	0x22951b1(%rip), %rax   # 0x142a8c348
1407f7197: 49 01 c7                    	addq	%rax, %r15
1407f719a: 41 0f 28 f2                 	movaps	%xmm10, %xmm6
1407f719e: 41 0f 59 f2                 	mulps	%xmm10, %xmm6
1407f71a2: f3 0f 16 c6                 	movshdup	%xmm6, %xmm0            # xmm0 = xmm6[1,1,3,3]
1407f71a6: f3 0f 58 c6                 	addss	%xmm6, %xmm0
1407f71aa: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407f71ad: 66 0f 15 ce                 	unpckhpd	%xmm6, %xmm1            # xmm1 = xmm1[1],xmm6[1]
1407f71b1: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407f71b5: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407f71b9: 0f c6 f6 ff                 	shufps	$0xff, %xmm6, %xmm6     # xmm6 = xmm6[3,3,3,3]
1407f71bd: 0f 5c f1                    	subps	%xmm1, %xmm6
1407f71c0: 0f 28 c6                    	movaps	%xmm6, %xmm0
1407f71c3: 0f 28 1d 96 4a 29 02        	movaps	0x2294a96(%rip), %xmm3  # 0x142a8bc60
1407f71ca: 0f 59 c3                    	mulps	%xmm3, %xmm0
1407f71cd: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407f71d1: 0f 59 cb                    	mulps	%xmm3, %xmm1
1407f71d4: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1407f71d8: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407f71dc: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1407f71df: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1407f71e3: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1407f71e7: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407f71eb: 41 0f 59 ca                 	mulps	%xmm10, %xmm1
1407f71ef: 0f 58 c8                    	addps	%xmm0, %xmm1
1407f71f2: 45 0f 28 da                 	movaps	%xmm10, %xmm11
1407f71f6: 45 0f c6 da d2              	shufps	$0xd2, %xmm10, %xmm11   # xmm11 = xmm11[2,0],xmm10[1,3]
1407f71fb: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
1407f71ff: 0f 59 c3                    	mulps	%xmm3, %xmm0
1407f7202: 41 0f 58 c0                 	addps	%xmm8, %xmm0
1407f7206: 0f c6 c0 d2                 	shufps	$0xd2, %xmm0, %xmm0     # xmm0 = xmm0[2,0,1,3]
1407f720a: 45 0f 28 fa                 	movaps	%xmm10, %xmm15
1407f720e: 45 0f 58 fa                 	addps	%xmm10, %xmm15
1407f7212: 45 0f c6 ff ff              	shufps	$0xff, %xmm15, %xmm15   # xmm15 = xmm15[3,3,3,3]
1407f7217: 41 0f 59 c7                 	mulps	%xmm15, %xmm0
1407f721b: 0f 58 c1                    	addps	%xmm1, %xmm0
1407f721e: 0f 58 f8                    	addps	%xmm0, %xmm7
1407f7221: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407f7224: f3 44 0f 58 c8              	addss	%xmm0, %xmm9
1407f7229: 4c 8b 95 00 02 00 00        	movq	0x200(%rbp), %r10
1407f7230: 49 8b 82 b8 00 00 00        	movq	0xb8(%r10), %rax
1407f7237: 48 69 cb 60 01 00 00        	imulq	$0x160, %rbx, %rcx      # imm = 0x160
1407f723e: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407f7246: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407f724f: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407f7258: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407f7260: 48 8b 55 5c                 	movq	0x5c(%rbp), %rdx
1407f7264: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407f7269: 8b 55 64                    	movl	0x64(%rbp), %edx
1407f726c: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407f7270: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407f7278: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1407f727f: 0f 10 8d d0 01 00 00        	movups	0x1d0(%rbp), %xmm1
1407f7286: f3 0f 6f 95 e0 01 00 00     	movdqu	0x1e0(%rbp), %xmm2
1407f728e: 0f 10 9d ec 01 00 00        	movups	0x1ec(%rbp), %xmm3
1407f7295: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407f729a: 0f 11 4c 08 54              	movups	%xmm1, 0x54(%rax,%rcx)
1407f729f: f3 0f 7f 54 08 64           	movdqu	%xmm2, 0x64(%rax,%rcx)
1407f72a5: 0f 11 5c 08 70              	movups	%xmm3, 0x70(%rax,%rcx)
1407f72aa: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407f72b5: 48 8b 55 68                 	movq	0x68(%rbp), %rdx
1407f72b9: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407f72c1: 8b 55 70                    	movl	0x70(%rbp), %edx
1407f72c4: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407f72cb: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407f72d6: 48 8b 55 74                 	movq	0x74(%rbp), %rdx
1407f72da: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407f72e2: 8b 55 7c                    	movl	0x7c(%rbp), %edx
1407f72e5: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407f72ec: 4c 89 bc 08 a0 00 00 00     	movq	%r15, 0xa0(%rax,%rcx)
1407f72f4: 48 c7 84 08 a8 00 00 00 24 00 00 00 	movq	$0x24, 0xa8(%rax,%rcx)
1407f7300: 48 8d 15 6d 4e 29 02        	leaq	0x2294e6d(%rip), %rdx   # 0x142a8c174
1407f7307: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407f730f: 48 c7 84 08 b8 00 00 00 1e 00 00 00 	movq	$0x1e, 0xb8(%rax,%rcx)
1407f731b: 44 0f 29 94 08 c0 00 00 00  	movaps	%xmm10, 0xc0(%rax,%rcx)
1407f7324: 0f 13 bc 08 d0 00 00 00     	movlps	%xmm7, 0xd0(%rax,%rcx)
1407f732c: f3 44 0f 11 8c 08 d8 00 00 00       	movss	%xmm9, 0xd8(%rax,%rcx)
1407f7336: 49 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %r9 # imm = 0x3F8000003F800000
1407f7340: 4c 89 8c 08 dc 00 00 00     	movq	%r9, 0xdc(%rax,%rcx)
1407f7348: c7 84 08 e4 00 00 00 00 00 80 3f    	movl	$0x3f800000, 0xe4(%rax,%rcx) # imm = 0x3F800000
1407f7353: c7 84 08 f0 00 00 00 01 00 00 00    	movl	$0x1, 0xf0(%rax,%rcx)
1407f735e: 8b 95 c8 00 00 00           	movl	0xc8(%rbp), %edx
1407f7364: 89 94 08 f4 00 00 00        	movl	%edx, 0xf4(%rax,%rcx)
1407f736b: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407f7376: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
1407f737d: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407f7385: 48 8b 95 a0 01 00 00        	movq	0x1a0(%rbp), %rdx
1407f738c: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407f7394: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407f739c: 0f b6 95 b7 01 00 00        	movzbl	0x1b7(%rbp), %edx
1407f73a3: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407f73aa: 0f b7 95 b5 01 00 00        	movzwl	0x1b5(%rbp), %edx
1407f73b1: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407f73b9: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407f73c4: f3 0f 6f 85 e0 00 00 00     	movdqu	0xe0(%rbp), %xmm0
1407f73cc: f3 0f 6f 8d f0 00 00 00     	movdqu	0xf0(%rbp), %xmm1
1407f73d4: f3 0f 7f 8c 08 2c 01 00 00  	movdqu	%xmm1, 0x12c(%rax,%rcx)
1407f73dd: f3 0f 7f 84 08 1c 01 00 00  	movdqu	%xmm0, 0x11c(%rax,%rcx)
1407f73e6: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407f73ee: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1407f73f5: 4c 8b 85 7f 01 00 00        	movq	0x17f(%rbp), %r8
1407f73fc: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407f7404: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407f740c: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407f7414: 0f b6 95 38 01 00 00        	movzbl	0x138(%rbp), %edx
1407f741b: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407f7422: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407f742c: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
1407f7433: 4c 8b 85 17 01 00 00        	movq	0x117(%rbp), %r8
1407f743a: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407f7442: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407f744a: 48 ff c3                    	incq	%rbx
1407f744d: 49 89 9a c0 00 00 00        	movq	%rbx, 0xc0(%r10)
1407f7454: 49 8b 02                    	movq	(%r10), %rax
1407f7457: 49 0b 42 08                 	orq	0x8(%r10), %rax
1407f745b: 0f 85 a4 10 00 00           	jne	0x1407f8505 <.text+0x7f7505>
1407f7461: 49 c7 42 08 00 00 00 00     	movq	$0x0, 0x8(%r10)
1407f7469: 49 c7 02 01 00 00 00        	movq	$0x1, (%r10)
1407f7470: 45 0f 29 52 10              	movaps	%xmm10, 0x10(%r10)
1407f7475: 0f 29 bd b0 00 00 00        	movaps	%xmm7, 0xb0(%rbp)
1407f747c: 41 0f 13 7a 20              	movlps	%xmm7, 0x20(%r10)
1407f7481: f3 45 0f 11 4a 28           	movss	%xmm9, 0x28(%r10)
1407f7487: 4d 89 4a 2c                 	movq	%r9, 0x2c(%r10)
1407f748b: 41 c7 42 34 00 00 80 3f     	movl	$0x3f800000, 0x34(%r10) # imm = 0x3F800000
1407f7493: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
1407f749a: 8a 58 fc                    	movb	-0x4(%rax), %bl
1407f749d: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
1407f74a4: b1 03                       	movb	$0x3, %cl
1407f74a6: e8 a5 bd 03 00              	callq	0x140833250 <.text+0x832250>
1407f74ab: 48 8b bd 28 01 00 00        	movq	0x128(%rbp), %rdi
1407f74b2: 89 d9                       	movl	%ebx, %ecx
1407f74b4: 89 c2                       	movl	%eax, %edx
1407f74b6: e8 d5 e7 32 00              	callq	0x140b25c90 <.text+0xb24c90>
1407f74bb: 88 85 90 01 00 00           	movb	%al, 0x190(%rbp)
1407f74c1: 88 95 91 01 00 00           	movb	%dl, 0x191(%rbp)
1407f74c7: 48 83 7f 18 00              	cmpq	$0x0, 0x18(%rdi)
1407f74cc: 0f 84 e3 0e 00 00           	je	0x1407f83b5 <.text+0x7f73b5>
1407f74d2: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
1407f74d6: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1407f74dd: e8 de 08 a0 00              	callq	0x1411f7dc0 <.text+0x11f6dc0>
1407f74e2: 48 8b 0f                    	movq	(%rdi), %rcx
1407f74e5: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
1407f74e9: 49 89 d0                    	movq	%rdx, %r8
1407f74ec: 49 21 c0                    	andq	%rax, %r8
1407f74ef: 48 c1 e8 39                 	shrq	$0x39, %rax
1407f74f3: 66 0f 6e c0                 	movd	%eax, %xmm0
1407f74f7: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407f74fb: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407f7500: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f7505: 48 8d 41 b8                 	leaq	-0x48(%rcx), %rax
1407f7509: 44 0f b6 8d 90 01 00 00     	movzbl	0x190(%rbp), %r9d
1407f7511: 44 0f b6 95 91 01 00 00     	movzbl	0x191(%rbp), %r10d
1407f7519: 45 31 db                    	xorl	%r11d, %r11d
1407f751c: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1407f7522: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407f7526: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407f752a: 66 0f d7 fa                 	pmovmskb	%xmm2, %edi
1407f752e: 85 ff                       	testl	%edi, %edi
1407f7530: 0f 28 bd b0 00 00 00        	movaps	0xb0(%rbp), %xmm7
1407f7537: 74 67                       	je	0x1407f75a0 <.text+0x7f65a0>
1407f7539: 41 80 f9 1a                 	cmpb	$0x1a, %r9b
1407f753d: 75 28                       	jne	0x1407f7567 <.text+0x7f6567>
1407f753f: f3 0f bc df                 	tzcntl	%edi, %ebx
1407f7543: 4c 01 c3                    	addq	%r8, %rbx
1407f7546: 48 21 d3                    	andq	%rdx, %rbx
1407f7549: 48 f7 db                    	negq	%rbx
1407f754c: 4c 8d 34 db                 	leaq	(%rbx,%rbx,8), %r14
1407f7550: 42 80 3c f0 1a              	cmpb	$0x1a, (%rax,%r14,8)
1407f7555: 0f 84 cb 02 00 00           	je	0x1407f7826 <.text+0x7f6826>
1407f755b: 8d 5f ff                    	leal	-0x1(%rdi), %ebx
1407f755e: 66 21 fb                    	andw	%di, %bx
1407f7561: 89 df                       	movl	%ebx, %edi
1407f7563: 75 da                       	jne	0x1407f753f <.text+0x7f653f>
1407f7565: eb 39                       	jmp	0x1407f75a0 <.text+0x7f65a0>
1407f7567: f3 0f bc df                 	tzcntl	%edi, %ebx
1407f756b: 4c 01 c3                    	addq	%r8, %rbx
1407f756e: 48 21 d3                    	andq	%rdx, %rbx
1407f7571: 48 f7 db                    	negq	%rbx
1407f7574: 4c 8d 34 db                 	leaq	(%rbx,%rbx,8), %r14
1407f7578: 46 0f b6 3c f0              	movzbl	(%rax,%r14,8), %r15d
1407f757d: 41 80 ff 1a                 	cmpb	$0x1a, %r15b
1407f7581: 74 13                       	je	0x1407f7596 <.text+0x7f6596>
1407f7583: 45 38 f9                    	cmpb	%r15b, %r9b
1407f7586: 75 0e                       	jne	0x1407f7596 <.text+0x7f6596>
1407f7588: 4e 8d 34 f0                 	leaq	(%rax,%r14,8), %r14
1407f758c: 45 3a 56 01                 	cmpb	0x1(%r14), %r10b
1407f7590: 0f 84 90 02 00 00           	je	0x1407f7826 <.text+0x7f6826>
1407f7596: 8d 5f ff                    	leal	-0x1(%rdi), %ebx
1407f7599: 66 21 fb                    	andw	%di, %bx
1407f759c: 89 df                       	movl	%ebx, %edi
1407f759e: 75 c7                       	jne	0x1407f7567 <.text+0x7f6567>
1407f75a0: 66 0f 74 0d e8 e3 12 02     	pcmpeqb	0x212e3e8(%rip), %xmm1  # 0x142925990
1407f75a8: 66 0f d7 f9                 	pmovmskb	%xmm1, %edi
1407f75ac: 85 ff                       	testl	%edi, %edi
1407f75ae: 0f 85 01 0e 00 00           	jne	0x1407f83b5 <.text+0x7f73b5>
1407f75b4: 4d 01 d8                    	addq	%r11, %r8
1407f75b7: 49 83 c0 10                 	addq	$0x10, %r8
1407f75bb: 49 83 c3 10                 	addq	$0x10, %r11
1407f75bf: 49 21 d0                    	andq	%rdx, %r8
1407f75c2: e9 55 ff ff ff              	jmp	0x1407f751c <.text+0x7f651c>
1407f75c7: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
1407f75ce: 80 78 da 02                 	cmpb	$0x2, -0x26(%rax)
1407f75d2: 0f 83 75 0f 00 00           	jae	0x1407f854d <.text+0x7f754d>
1407f75d8: f2 0f 10 32                 	movsd	(%rdx), %xmm6
1407f75dc: f2 0f 11 b5 10 01 00 00     	movsd	%xmm6, 0x110(%rbp)
1407f75e4: f2 0f 11 b5 78 01 00 00     	movsd	%xmm6, 0x178(%rbp)
1407f75ec: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
1407f75f3: e8 48 31 33 00              	callq	0x140b2a740 <.text+0xb29740>
1407f75f8: 89 85 c0 01 00 00           	movl	%eax, 0x1c0(%rbp)
1407f75fe: 66 0f 7e 85 c4 01 00 00     	movd	%xmm0, 0x1c4(%rbp)
1407f7606: f3 0f 10 8d 78 01 00 00     	movss	0x178(%rbp), %xmm1
1407f760e: f3 0f 58 0d e2 40 1f 02     	addss	0x21f40e2(%rip), %xmm1  # 0x1429eb6f8
1407f7616: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f761d: e8 8e a8 49 00              	callq	0x140c91eb0 <.text+0xc90eb0>
1407f7622: 66 0f 7e 85 78 01 00 00     	movd	%xmm0, 0x178(%rbp)
1407f762a: 0f 13 b5 90 01 00 00        	movlps	%xmm6, 0x190(%rbp)
1407f7631: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
1407f7638: e8 03 31 33 00              	callq	0x140b2a740 <.text+0xb29740>
1407f763d: 89 85 c0 01 00 00           	movl	%eax, 0x1c0(%rbp)
1407f7643: 66 0f 7e 85 c4 01 00 00     	movd	%xmm0, 0x1c4(%rbp)
1407f764b: f3 0f 10 8d 90 01 00 00     	movss	0x190(%rbp), %xmm1
1407f7653: f3 0f 58 0d 29 e3 12 02     	addss	0x212e329(%rip), %xmm1  # 0x142925984
1407f765b: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f7662: e8 49 a8 49 00              	callq	0x140c91eb0 <.text+0xc90eb0>
1407f7667: 66 0f 7e 85 90 01 00 00     	movd	%xmm0, 0x190(%rbp)
1407f766f: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f7676: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
1407f767d: 4c 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %r8
1407f7684: e8 07 aa 49 00              	callq	0x140c92090 <.text+0xc91090>
1407f7689: f3 0f 10 b5 c0 01 00 00     	movss	0x1c0(%rbp), %xmm6
1407f7691: f3 0f 10 bd c8 01 00 00     	movss	0x1c8(%rbp), %xmm7
1407f7699: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f76a0: 48 8d 95 78 01 00 00        	leaq	0x178(%rbp), %rdx
1407f76a7: 4c 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %r8
1407f76ae: e8 dd a9 49 00              	callq	0x140c92090 <.text+0xc91090>
1407f76b3: f3 44 0f 10 bd c0 01 00 00  	movss	0x1c0(%rbp), %xmm15
1407f76bc: f3 44 0f 10 85 c8 01 00 00  	movss	0x1c8(%rbp), %xmm8
1407f76c5: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f76cc: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1407f76d3: 4c 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %r8
1407f76da: e8 b1 a9 49 00              	callq	0x140c92090 <.text+0xc91090>
1407f76df: 45 0f 14 f8                 	unpcklps	%xmm8, %xmm15           # xmm15 = xmm15[0],xmm8[0],xmm15[1],xmm8[1]
1407f76e3: 0f 14 f7                    	unpcklps	%xmm7, %xmm6            # xmm6 = xmm6[0],xmm7[0],xmm6[1],xmm7[1]
1407f76e6: 44 0f 5c fe                 	subps	%xmm6, %xmm15
1407f76ea: 41 0f 28 c7                 	movaps	%xmm15, %xmm0
1407f76ee: 41 0f 59 c7                 	mulps	%xmm15, %xmm0
1407f76f2: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407f76f6: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407f76fa: f3 0f 51 c9                 	sqrtss	%xmm1, %xmm1
1407f76fe: f3 0f 10 1d fe e1 12 02     	movss	0x212e1fe(%rip), %xmm3  # 0x142925904
1407f7706: 0f 28 c3                    	movaps	%xmm3, %xmm0
1407f7709: f3 0f 5e c1                 	divss	%xmm1, %xmm0
1407f770d: 66 0f 7e c0                 	movd	%xmm0, %eax
1407f7711: 85 c0                       	testl	%eax, %eax
1407f7713: 0f 99 c1                    	setns	%cl
1407f7716: 89 c2                       	movl	%eax, %edx
1407f7718: 81 e2 ff ff ff 7f           	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
1407f771e: 81 c2 00 00 80 ff           	addl	$0xff800000, %edx       # imm = 0xFF800000
1407f7724: 81 fa 00 00 00 7f           	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
1407f772a: 0f 92 c2                    	setb	%dl
1407f772d: 20 ca                       	andb	%cl, %dl
1407f772f: ff c8                       	decl	%eax
1407f7731: 3d ff ff 7f 00              	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
1407f7736: 0f 92 c0                    	setb	%al
1407f7739: 08 d0                       	orb	%dl, %al
1407f773b: 0f 84 5d 0b 00 00           	je	0x1407f829e <.text+0x7f729e>
1407f7741: f3 44 0f 10 9d c0 01 00 00  	movss	0x1c0(%rbp), %xmm11
1407f774a: f3 0f 10 8d c8 01 00 00     	movss	0x1c8(%rbp), %xmm1
1407f7752: 44 0f 14 d9                 	unpcklps	%xmm1, %xmm11           # xmm11 = xmm11[0],xmm1[0],xmm11[1],xmm1[1]
1407f7756: 44 0f 5c de                 	subps	%xmm6, %xmm11
1407f775a: 41 0f 28 cb                 	movaps	%xmm11, %xmm1
1407f775e: 41 0f 59 cb                 	mulps	%xmm11, %xmm1
1407f7762: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1407f7766: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407f776a: f3 0f 51 d2                 	sqrtss	%xmm2, %xmm2
1407f776e: 0f 28 cb                    	movaps	%xmm3, %xmm1
1407f7771: f3 0f 5e ca                 	divss	%xmm2, %xmm1
1407f7775: 66 0f 7e c8                 	movd	%xmm1, %eax
1407f7779: 85 c0                       	testl	%eax, %eax
1407f777b: 0f 98 c1                    	sets	%cl
1407f777e: 89 c2                       	movl	%eax, %edx
1407f7780: 81 e2 ff ff ff 7f           	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
1407f7786: 81 c2 00 00 80 ff           	addl	$0xff800000, %edx       # imm = 0xFF800000
1407f778c: 81 fa 00 00 00 7f           	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
1407f7792: 0f 93 c2                    	setae	%dl
1407f7795: 08 ca                       	orb	%cl, %dl
1407f7797: ff c8                       	decl	%eax
1407f7799: 3d ff ff 7f 00              	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
1407f779e: 0f 93 c0                    	setae	%al
1407f77a1: 84 d0                       	testb	%dl, %al
1407f77a3: 0f 85 f5 0a 00 00           	jne	0x1407f829e <.text+0x7f729e>
1407f77a9: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1407f77ad: 44 0f 59 f8                 	mulps	%xmm0, %xmm15
1407f77b1: f3 0f 12 c1                 	movsldup	%xmm1, %xmm0            # xmm0 = xmm1[0,0,2,2]
1407f77b5: 44 0f 59 d8                 	mulps	%xmm0, %xmm11
1407f77b9: 41 0f 28 f3                 	movaps	%xmm11, %xmm6
1407f77bd: 41 0f 5c f7                 	subps	%xmm15, %xmm6
1407f77c1: 0f 28 c6                    	movaps	%xmm6, %xmm0
1407f77c4: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407f77c7: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407f77cb: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407f77cf: f3 0f 51 c9                 	sqrtss	%xmm1, %xmm1
1407f77d3: 0f 28 c3                    	movaps	%xmm3, %xmm0
1407f77d6: f3 0f 5e c1                 	divss	%xmm1, %xmm0
1407f77da: 66 0f 7e c0                 	movd	%xmm0, %eax
1407f77de: 85 c0                       	testl	%eax, %eax
1407f77e0: 0f 98 c1                    	sets	%cl
1407f77e3: 89 c2                       	movl	%eax, %edx
1407f77e5: 81 e2 ff ff ff 7f           	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
1407f77eb: 81 c2 00 00 80 ff           	addl	$0xff800000, %edx       # imm = 0xFF800000
1407f77f1: 81 fa 00 00 00 7f           	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
1407f77f7: 0f 93 c2                    	setae	%dl
1407f77fa: 08 ca                       	orb	%cl, %dl
1407f77fc: ff c8                       	decl	%eax
1407f77fe: 3d ff ff 7f 00              	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
1407f7803: 0f 93 c0                    	setae	%al
1407f7806: 84 d0                       	testb	%dl, %al
1407f7808: 0f 85 90 0a 00 00           	jne	0x1407f829e <.text+0x7f729e>
1407f780e: f3 0f 16 fe                 	movshdup	%xmm6, %xmm7            # xmm7 = xmm6[1,1,3,3]
1407f7812: 0f 57 3d d7 e0 12 02        	xorps	0x212e0d7(%rip), %xmm7  # 0x1429258f0
1407f7819: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
1407f781d: f3 0f 59 f0                 	mulss	%xmm0, %xmm6
1407f7821: e9 62 f5 ff ff              	jmp	0x1407f6d88 <.text+0x7f5d88>
1407f7826: 48 8d 04 db                 	leaq	(%rbx,%rbx,8), %rax
1407f782a: 48 8b 44 c1 d8              	movq	-0x28(%rcx,%rax,8), %rax
1407f782f: f0                          	lock
1407f7830: 48 ff 00                    	incq	(%rax)
1407f7833: 0f 8e c8 0e 00 00           	jle	0x1407f8701 <.text+0x7f7701>
1407f7839: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
1407f7840: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407f7847: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
1407f784e: 8a 58 fc                    	movb	-0x4(%rax), %bl
1407f7851: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
1407f7858: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
1407f785f: b1 03                       	movb	$0x3, %cl
1407f7861: e8 ea b9 03 00              	callq	0x140833250 <.text+0x832250>
1407f7866: 48 8b bd 28 01 00 00        	movq	0x128(%rbp), %rdi
1407f786d: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
1407f7874: 89 d9                       	movl	%ebx, %ecx
1407f7876: 89 c2                       	movl	%eax, %edx
1407f7878: e8 13 e4 32 00              	callq	0x140b25c90 <.text+0xb24c90>
1407f787d: 88 85 78 01 00 00           	movb	%al, 0x178(%rbp)
1407f7883: 88 95 79 01 00 00           	movb	%dl, 0x179(%rbp)
1407f7889: 48 83 7f 48 00              	cmpq	$0x0, 0x48(%rdi)
1407f788e: 0f 84 92 0b 00 00           	je	0x1407f8426 <.text+0x7f7426>
1407f7894: 48 8d 4f 50                 	leaq	0x50(%rdi), %rcx
1407f7898: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
1407f789f: 48 8d 95 78 01 00 00        	leaq	0x178(%rbp), %rdx
1407f78a6: e8 15 05 a0 00              	callq	0x1411f7dc0 <.text+0x11f6dc0>
1407f78ab: 48 8b 4f 30                 	movq	0x30(%rdi), %rcx
1407f78af: 48 8b 57 38                 	movq	0x38(%rdi), %rdx
1407f78b3: 49 89 d0                    	movq	%rdx, %r8
1407f78b6: 49 21 c0                    	andq	%rax, %r8
1407f78b9: 48 c1 e8 39                 	shrq	$0x39, %rax
1407f78bd: 66 0f 6e c0                 	movd	%eax, %xmm0
1407f78c1: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407f78c5: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407f78ca: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f78cf: 48 8d 41 b8                 	leaq	-0x48(%rcx), %rax
1407f78d3: 44 0f b6 8d 78 01 00 00     	movzbl	0x178(%rbp), %r9d
1407f78db: 44 0f b6 95 79 01 00 00     	movzbl	0x179(%rbp), %r10d
1407f78e3: 45 31 db                    	xorl	%r11d, %r11d
1407f78e6: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1407f78ec: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407f78f0: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407f78f4: 66 0f d7 fa                 	pmovmskb	%xmm2, %edi
1407f78f8: 85 ff                       	testl	%edi, %edi
1407f78fa: 74 5f                       	je	0x1407f795b <.text+0x7f695b>
1407f78fc: 41 80 f9 1a                 	cmpb	$0x1a, %r9b
1407f7900: 75 24                       	jne	0x1407f7926 <.text+0x7f6926>
1407f7902: f3 0f bc df                 	tzcntl	%edi, %ebx
1407f7906: 4c 01 c3                    	addq	%r8, %rbx
1407f7909: 48 21 d3                    	andq	%rdx, %rbx
1407f790c: 48 f7 db                    	negq	%rbx
1407f790f: 4c 8d 34 db                 	leaq	(%rbx,%rbx,8), %r14
1407f7913: 42 80 3c f0 1a              	cmpb	$0x1a, (%rax,%r14,8)
1407f7918: 74 68                       	je	0x1407f7982 <.text+0x7f6982>
1407f791a: 8d 5f ff                    	leal	-0x1(%rdi), %ebx
1407f791d: 66 21 fb                    	andw	%di, %bx
1407f7920: 89 df                       	movl	%ebx, %edi
1407f7922: 75 de                       	jne	0x1407f7902 <.text+0x7f6902>
1407f7924: eb 35                       	jmp	0x1407f795b <.text+0x7f695b>
1407f7926: f3 0f bc df                 	tzcntl	%edi, %ebx
1407f792a: 4c 01 c3                    	addq	%r8, %rbx
1407f792d: 48 21 d3                    	andq	%rdx, %rbx
1407f7930: 48 f7 db                    	negq	%rbx
1407f7933: 4c 8d 34 db                 	leaq	(%rbx,%rbx,8), %r14
1407f7937: 46 0f b6 3c f0              	movzbl	(%rax,%r14,8), %r15d
1407f793c: 41 80 ff 1a                 	cmpb	$0x1a, %r15b
1407f7940: 74 0f                       	je	0x1407f7951 <.text+0x7f6951>
1407f7942: 45 38 f9                    	cmpb	%r15b, %r9b
1407f7945: 75 0a                       	jne	0x1407f7951 <.text+0x7f6951>
1407f7947: 4e 8d 34 f0                 	leaq	(%rax,%r14,8), %r14
1407f794b: 45 3a 56 01                 	cmpb	0x1(%r14), %r10b
1407f794f: 74 31                       	je	0x1407f7982 <.text+0x7f6982>
1407f7951: 8d 5f ff                    	leal	-0x1(%rdi), %ebx
1407f7954: 66 21 fb                    	andw	%di, %bx
1407f7957: 89 df                       	movl	%ebx, %edi
1407f7959: 75 cb                       	jne	0x1407f7926 <.text+0x7f6926>
1407f795b: 66 0f 74 0d 2d e0 12 02     	pcmpeqb	0x212e02d(%rip), %xmm1  # 0x142925990
1407f7963: 66 0f d7 f9                 	pmovmskb	%xmm1, %edi
1407f7967: 85 ff                       	testl	%edi, %edi
1407f7969: 0f 85 b7 0a 00 00           	jne	0x1407f8426 <.text+0x7f7426>
1407f796f: 4d 01 d8                    	addq	%r11, %r8
1407f7972: 49 83 c0 10                 	addq	$0x10, %r8
1407f7976: 49 83 c3 10                 	addq	$0x10, %r11
1407f797a: 49 21 d0                    	andq	%rdx, %r8
1407f797d: e9 64 ff ff ff              	jmp	0x1407f78e6 <.text+0x7f68e6>
1407f7982: 48 8d 04 db                 	leaq	(%rbx,%rbx,8), %rax
1407f7986: 4c 8b 44 c1 d8              	movq	-0x28(%rcx,%rax,8), %r8
1407f798b: f0                          	lock
1407f798c: 49 ff 00                    	incq	(%r8)
1407f798f: 0f 8e 6c 0d 00 00           	jle	0x1407f8701 <.text+0x7f7701>
1407f7995: 0f 13 bd c0 01 00 00        	movlps	%xmm7, 0x1c0(%rbp)
1407f799c: f3 44 0f 11 8d c8 01 00 00  	movss	%xmm9, 0x1c8(%rbp)
1407f79a5: 44 0f 29 55 10              	movaps	%xmm10, 0x10(%rbp)
1407f79aa: c6 85 17 02 00 00 00        	movb	$0x0, 0x217(%rbp)
1407f79b1: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
1407f79b5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f79ba: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
1407f79c1: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
1407f79c8: 4c 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %r9
1407f79cf: e8 dc d8 02 00              	callq	0x1408252b0 <.text+0x8242b0>
1407f79d4: 4c 8b b5 88 00 00 00        	movq	0x88(%rbp), %r14
1407f79db: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
1407f79e2: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
1407f79e9: 83 b8 58 02 00 00 02        	cmpl	$0x2, 0x258(%rax)
1407f79f0: 0f 83 e9 02 00 00           	jae	0x1407f7cdf <.text+0x7f6cdf>
1407f79f6: 44 0f b6 7a fc              	movzbl	-0x4(%rdx), %r15d
1407f79fb: 41 8d 47 ff                 	leal	-0x1(%r15), %eax
1407f79ff: 3c 03                       	cmpb	$0x3, %al
1407f7a01: 0f 83 b7 0c 00 00           	jae	0x1407f86be <.text+0x7f76be>
1407f7a07: 0f b6 c0                    	movzbl	%al, %eax
1407f7a0a: 48 8d 0d 43 49 29 02        	leaq	0x2294943(%rip), %rcx   # 0x142a8c354
1407f7a11: 48 63 1c 81                 	movslq	(%rcx,%rax,4), %rbx
1407f7a15: 48 8d 85 c4 01 00 00        	leaq	0x1c4(%rbp), %rax
1407f7a1c: c7 40 07 00 00 00 00        	movl	$0x0, 0x7(%rax)
1407f7a23: 48 c7 40 ff 00 00 00 00     	movq	$0x0, -0x1(%rax)
1407f7a2b: 48 8d 4a e8                 	leaq	-0x18(%rdx), %rcx
1407f7a2f: 48 8d 15 fa 47 29 02        	leaq	0x22947fa(%rip), %rdx   # 0x142a8c230
1407f7a36: e8 75 19 04 00              	callq	0x1408393b0 <.text+0x8383b0>
1407f7a3b: 8b 00                       	movl	(%rax), %eax
1407f7a3d: 89 85 d8 00 00 00           	movl	%eax, 0xd8(%rbp)
1407f7a43: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407f7a4a: 48 8b 8d c7 01 00 00        	movq	0x1c7(%rbp), %rcx
1407f7a51: 48 89 8d 7f 01 00 00        	movq	%rcx, 0x17f(%rbp)
1407f7a58: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
1407f7a5f: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407f7a66: 48 8b b8 c0 00 00 00        	movq	0xc0(%rax), %rdi
1407f7a6d: 48 3b b8 b0 00 00 00        	cmpq	0xb0(%rax), %rdi
1407f7a74: 75 13                       	jne	0x1407f7a89 <.text+0x7f6a89>
1407f7a76: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
1407f7a7d: 48 8d 15 c4 47 29 02        	leaq	0x22947c4(%rip), %rdx   # 0x142a8c248
1407f7a84: e8 27 39 03 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407f7a89: 48 8d 05 c4 48 29 02        	leaq	0x22948c4(%rip), %rax   # 0x142a8c354
1407f7a90: 48 01 c3                    	addq	%rax, %rbx
1407f7a93: 49 69 d7 e8 03 00 00        	imulq	$0x3e8, %r15, %rdx      # imm = 0x3E8
1407f7a9a: 48 03 95 68 01 00 00        	addq	0x168(%rbp), %rdx
1407f7aa1: 4c 8b 8d 00 02 00 00        	movq	0x200(%rbp), %r9
1407f7aa8: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407f7aaf: 48 69 cf 60 01 00 00        	imulq	$0x160, %rdi, %rcx      # imm = 0x160
1407f7ab6: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407f7abe: 48 c7 44 08 10 01 00 00 00  	movq	$0x1, 0x10(%rax,%rcx)
1407f7ac7: 48 89 54 08 18              	movq	%rdx, 0x18(%rax,%rcx)
1407f7acc: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407f7ad5: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407f7add: 48 8b 55 5c                 	movq	0x5c(%rbp), %rdx
1407f7ae1: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407f7ae6: 8b 55 64                    	movl	0x64(%rbp), %edx
1407f7ae9: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407f7aed: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407f7af5: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1407f7afc: 0f 10 8d d0 01 00 00        	movups	0x1d0(%rbp), %xmm1
1407f7b03: f3 0f 6f 95 e0 01 00 00     	movdqu	0x1e0(%rbp), %xmm2
1407f7b0b: 0f 10 9d ec 01 00 00        	movups	0x1ec(%rbp), %xmm3
1407f7b12: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407f7b17: 0f 11 4c 08 54              	movups	%xmm1, 0x54(%rax,%rcx)
1407f7b1c: f3 0f 7f 54 08 64           	movdqu	%xmm2, 0x64(%rax,%rcx)
1407f7b22: 0f 11 5c 08 70              	movups	%xmm3, 0x70(%rax,%rcx)
1407f7b27: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407f7b32: 48 8b 55 68                 	movq	0x68(%rbp), %rdx
1407f7b36: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407f7b3e: 8b 55 70                    	movl	0x70(%rbp), %edx
1407f7b41: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407f7b48: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407f7b53: 48 8b 55 74                 	movq	0x74(%rbp), %rdx
1407f7b57: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407f7b5f: 8b 55 7c                    	movl	0x7c(%rbp), %edx
1407f7b62: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407f7b69: 48 89 9c 08 a0 00 00 00     	movq	%rbx, 0xa0(%rax,%rcx)
1407f7b71: 48 c7 84 08 a8 00 00 00 2a 00 00 00 	movq	$0x2a, 0xa8(%rax,%rcx)
1407f7b7d: 48 8d 15 c8 96 42 02        	leaq	0x24296c8(%rip), %rdx   # 0x142c2124c
1407f7b84: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407f7b8c: 48 c7 84 08 b8 00 00 00 24 00 00 00 	movq	$0x24, 0xb8(%rax,%rcx)
1407f7b98: 44 0f 29 94 08 c0 00 00 00  	movaps	%xmm10, 0xc0(%rax,%rcx)
1407f7ba1: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
1407f7ba8: 0f 13 84 08 d0 00 00 00     	movlps	%xmm0, 0xd0(%rax,%rcx)
1407f7bb0: f3 44 0f 11 8c 08 d8 00 00 00       	movss	%xmm9, 0xd8(%rax,%rcx)
1407f7bba: 48 ba 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rdx # imm = 0x3F8000003F800000
1407f7bc4: 48 89 94 08 dc 00 00 00     	movq	%rdx, 0xdc(%rax,%rcx)
1407f7bcc: c7 84 08 e4 00 00 00 00 00 80 3f    	movl	$0x3f800000, 0xe4(%rax,%rcx) # imm = 0x3F800000
1407f7bd7: c7 84 08 f0 00 00 00 01 00 00 00    	movl	$0x1, 0xf0(%rax,%rcx)
1407f7be2: 8b 95 d8 00 00 00           	movl	0xd8(%rbp), %edx
1407f7be8: 89 94 08 f4 00 00 00        	movl	%edx, 0xf4(%rax,%rcx)
1407f7bef: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407f7bfa: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
1407f7c01: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407f7c09: 48 8b 95 a0 01 00 00        	movq	0x1a0(%rbp), %rdx
1407f7c10: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407f7c18: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407f7c20: 0f b6 95 b7 01 00 00        	movzbl	0x1b7(%rbp), %edx
1407f7c27: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407f7c2e: 0f b7 95 b5 01 00 00        	movzwl	0x1b5(%rbp), %edx
1407f7c35: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407f7c3d: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407f7c48: f3 0f 6f 85 e0 00 00 00     	movdqu	0xe0(%rbp), %xmm0
1407f7c50: f3 0f 6f 8d f0 00 00 00     	movdqu	0xf0(%rbp), %xmm1
1407f7c58: f3 0f 7f 8c 08 2c 01 00 00  	movdqu	%xmm1, 0x12c(%rax,%rcx)
1407f7c61: f3 0f 7f 84 08 1c 01 00 00  	movdqu	%xmm0, 0x11c(%rax,%rcx)
1407f7c6a: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407f7c72: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1407f7c79: 4c 8b 85 7f 01 00 00        	movq	0x17f(%rbp), %r8
1407f7c80: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407f7c88: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407f7c90: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407f7c98: 0f b6 95 38 01 00 00        	movzbl	0x138(%rbp), %edx
1407f7c9f: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407f7ca6: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407f7cb0: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
1407f7cb7: 4c 8b 85 17 01 00 00        	movq	0x117(%rbp), %r8
1407f7cbe: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407f7cc6: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407f7cce: 48 ff c7                    	incq	%rdi
1407f7cd1: 49 89 b9 c0 00 00 00        	movq	%rdi, 0xc0(%r9)
1407f7cd8: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
1407f7cdf: 44 8a 7a fc                 	movb	-0x4(%rdx), %r15b
1407f7ce3: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
1407f7cea: b1 03                       	movb	$0x3, %cl
1407f7cec: e8 5f b5 03 00              	callq	0x140833250 <.text+0x832250>
1407f7cf1: 48 8b 9d 28 01 00 00        	movq	0x128(%rbp), %rbx
1407f7cf8: 44 89 f9                    	movl	%r15d, %ecx
1407f7cfb: 89 c2                       	movl	%eax, %edx
1407f7cfd: e8 8e df 32 00              	callq	0x140b25c90 <.text+0xb24c90>
1407f7d02: 88 85 90 01 00 00           	movb	%al, 0x190(%rbp)
1407f7d08: 88 95 91 01 00 00           	movb	%dl, 0x191(%rbp)
1407f7d0e: 48 89 d9                    	movq	%rbx, %rcx
1407f7d11: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1407f7d18: e8 73 52 a6 00              	callq	0x14125cf90 <.text+0x125bf90>
1407f7d1d: 48 85 c0                    	testq	%rax, %rax
1407f7d20: 0f 84 2a 09 00 00           	je	0x1407f8650 <.text+0x7f7650>
1407f7d26: f3 0f 10 78 30              	movss	0x30(%rax), %xmm7
1407f7d2b: f3 44 0f 10 60 3c           	movss	0x3c(%rax), %xmm12
1407f7d31: f3 44 0f 5c 60 38           	subss	0x38(%rax), %xmm12
1407f7d37: 44 0f 28 c7                 	movaps	%xmm7, %xmm8
1407f7d3b: f3 44 0f 5e 05 b0 69 13 02  	divss	0x21369b0(%rip), %xmm8  # 0x14292e6f4
1407f7d44: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
1407f7d4b: 48 8d 15 0e 45 29 02        	leaq	0x229450e(%rip), %rdx   # 0x142a8c260
1407f7d52: e8 09 b5 03 00              	callq	0x140833260 <.text+0x832260>
1407f7d57: f2 44 0f 10 30              	movsd	(%rax), %xmm14
1407f7d5c: 45 0f 28 e8                 	movaps	%xmm8, %xmm13
1407f7d60: f3 45 0f 58 e8              	addss	%xmm8, %xmm13
1407f7d65: f3 44 0f 58 2d 12 3f 29 02  	addss	0x2293f12(%rip), %xmm13 # 0x142a8bc80
1407f7d6e: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407f7d75: 48 8b b8 d8 00 00 00        	movq	0xd8(%rax), %rdi
1407f7d7c: 48 3b b8 c8 00 00 00        	cmpq	0xc8(%rax), %rdi
1407f7d83: 75 1a                       	jne	0x1407f7d9f <.text+0x7f6d9f>
1407f7d85: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407f7d8c: 48 8d 88 c8 00 00 00        	leaq	0xc8(%rax), %rcx
1407f7d93: 48 8d 15 de 44 29 02        	leaq	0x22944de(%rip), %rdx   # 0x142a8c278
1407f7d9a: e8 61 b8 d6 ff              	callq	0x140563600 <.text+0x562600>
1407f7d9f: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
1407f7da6: 48 8b 82 d0 00 00 00        	movq	0xd0(%rdx), %rax
1407f7dad: 48 8d 0c bf                 	leaq	(%rdi,%rdi,4), %rcx
1407f7db1: 44 0f 13 34 88              	movlps	%xmm14, (%rax,%rcx,4)
1407f7db6: f3 44 0f 11 6c 88 08        	movss	%xmm13, 0x8(%rax,%rcx,4)
1407f7dbd: f3 44 0f 11 64 88 0c        	movss	%xmm12, 0xc(%rax,%rcx,4)
1407f7dc4: c6 44 88 10 00              	movb	$0x0, 0x10(%rax,%rcx,4)
1407f7dc9: 48 ff c7                    	incq	%rdi
1407f7dcc: 48 89 ba d8 00 00 00        	movq	%rdi, 0xd8(%rdx)
1407f7dd3: 4c 8b 85 48 01 00 00        	movq	0x148(%rbp), %r8
1407f7dda: 4c 8b 8d 50 01 00 00        	movq	0x150(%rbp), %r9
1407f7de1: f3 44 0f 11 44 24 58        	movss	%xmm8, 0x58(%rsp)
1407f7de8: 48 8d 85 20 01 00 00        	leaq	0x120(%rbp), %rax
1407f7def: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1407f7df4: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
1407f7df9: 8b 85 c8 00 00 00           	movl	0xc8(%rbp), %eax
1407f7dff: 89 44 24 28                 	movl	%eax, 0x28(%rsp)
1407f7e03: 0f b6 85 38 01 00 00        	movzbl	0x138(%rbp), %eax
1407f7e0a: 88 44 24 20                 	movb	%al, 0x20(%rsp)
1407f7e0e: c7 44 24 50 01 00 00 00     	movl	$0x1, 0x50(%rsp)
1407f7e16: c7 44 24 48 00 00 00 00     	movl	$0x0, 0x48(%rsp)
1407f7e1e: c7 44 24 40 00 00 00 00     	movl	$0x0, 0x40(%rsp)
1407f7e26: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
1407f7e2d: 48 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %rdx
1407f7e34: e8 57 7b ff ff              	callq	0x1407ef990 <.text+0x7ee990>
1407f7e39: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
1407f7e40: 83 b8 58 02 00 00 02        	cmpl	$0x2, 0x258(%rax)
1407f7e47: 0f 83 51 04 00 00           	jae	0x1407f829e <.text+0x7f729e>
1407f7e4d: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
1407f7e54: 8b 4a e4                    	movl	-0x1c(%rdx), %ecx
1407f7e57: 69 c1 b6 aa aa aa           	imull	$0xaaaaaab6, %ecx, %eax # imm = 0xAAAAAAB6
1407f7e5d: 05 4b 88 56 55              	addl	$0x5556884b, %eax       # imm = 0x5556884B
1407f7e62: 3d 55 55 55 55              	cmpl	$0x55555555, %eax       # imm = 0x55555555
1407f7e67: 0f 87 1b 01 00 00           	ja	0x1407f7f88 <.text+0x7f6f88>
1407f7e6d: 0f b6 42 fc                 	movzbl	-0x4(%rdx), %eax
1407f7e71: 83 f8 03                    	cmpl	$0x3, %eax
1407f7e74: 0f 83 0e 01 00 00           	jae	0x1407f7f88 <.text+0x7f6f88>
1407f7e7a: 83 f8 01                    	cmpl	$0x1, %eax
1407f7e7d: 48 8b 9d 68 01 00 00        	movq	0x168(%rbp), %rbx
1407f7e84: 74 0d                       	je	0x1407f7e93 <.text+0x7f6e93>
1407f7e86: 83 f8 02                    	cmpl	$0x2, %eax
1407f7e89: 0f 85 5a 08 00 00           	jne	0x1407f86e9 <.text+0x7f76e9>
1407f7e8f: 44 0f 28 c7                 	movaps	%xmm7, %xmm8
1407f7e93: 0f b6 05 6f cd e3 02        	movzbl	0x2e3cd6f(%rip), %eax   # 0x143634c09
1407f7e9a: b9 50 00 00 00              	movl	$0x50, %ecx
1407f7e9f: ba 10 00 00 00              	movl	$0x10, %edx
1407f7ea4: e8 67 bd e1 ff              	callq	0x140613c10 <.text+0x612c10>
1407f7ea9: 48 85 c0                    	testq	%rax, %rax
1407f7eac: 0f 84 26 08 00 00           	je	0x1407f86d8 <.text+0x7f76d8>
1407f7eb2: 48 89 c7                    	movq	%rax, %rdi
1407f7eb5: 0f 28 0d d4 3d 29 02        	movaps	0x2293dd4(%rip), %xmm1  # 0x142a8bc90
1407f7ebc: 44 0f 59 d9                 	mulps	%xmm1, %xmm11
1407f7ec0: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
1407f7ec4: 0f 59 05 d5 3d 29 02        	mulps	0x2293dd5(%rip), %xmm0  # 0x142a8bca0
1407f7ecb: 41 0f 58 c3                 	addps	%xmm11, %xmm0
1407f7ecf: 0f c6 c0 d2                 	shufps	$0xd2, %xmm0, %xmm0     # xmm0 = xmm0[2,0,1,3]
1407f7ed3: 44 0f 59 f8                 	mulps	%xmm0, %xmm15
1407f7ed7: 0f 59 f1                    	mulps	%xmm1, %xmm6
1407f7eda: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
1407f7ede: 0f 59 c1                    	mulps	%xmm1, %xmm0
1407f7ee1: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407f7ee5: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407f7ee9: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407f7eec: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407f7ef0: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1407f7ef4: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f7ef8: 41 0f 59 c2                 	mulps	%xmm10, %xmm0
1407f7efc: 0f 58 c6                    	addps	%xmm6, %xmm0
1407f7eff: 41 0f 58 c7                 	addps	%xmm15, %xmm0
1407f7f03: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407f7f06: 66 0f 15 c8                 	unpckhpd	%xmm0, %xmm1            # xmm1 = xmm1[1],xmm0[1]
1407f7f0a: f3 41 0f 58 c9              	addss	%xmm9, %xmm1
1407f7f0f: 44 0f 29 10                 	movaps	%xmm10, (%rax)
1407f7f13: 0f 58 85 b0 00 00 00        	addps	0xb0(%rbp), %xmm0
1407f7f1a: 0f 13 40 10                 	movlps	%xmm0, 0x10(%rax)
1407f7f1e: f3 0f 11 48 18              	movss	%xmm1, 0x18(%rax)
1407f7f23: 48 b8 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
1407f7f2d: 48 89 47 1c                 	movq	%rax, 0x1c(%rdi)
1407f7f31: c7 47 24 00 00 80 3f        	movl	$0x3f800000, 0x24(%rdi) # imm = 0x3F800000
1407f7f38: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
1407f7f3f: 48 89 47 30                 	movq	%rax, 0x30(%rdi)
1407f7f43: f3 44 0f 11 47 38           	movss	%xmm8, 0x38(%rdi)
1407f7f49: f3 44 0f 11 67 3c           	movss	%xmm12, 0x3c(%rdi)
1407f7f4f: 89 5f 40                    	movl	%ebx, 0x40(%rdi)
1407f7f52: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407f7f59: 48 8b 88 10 01 00 00        	movq	0x110(%rax), %rcx
1407f7f60: 48 85 c9                    	testq	%rcx, %rcx
1407f7f63: 74 10                       	je	0x1407f7f75 <.text+0x7f6f75>
1407f7f65: ba 50 00 00 00              	movl	$0x50, %edx
1407f7f6a: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1407f7f70: e8 ab bc e1 ff              	callq	0x140613c20 <.text+0x612c20>
1407f7f75: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407f7f7c: 48 89 b8 10 01 00 00        	movq	%rdi, 0x110(%rax)
1407f7f83: e9 16 03 00 00              	jmp	0x1407f829e <.text+0x7f729e>
1407f7f88: e8 b3 e2 49 00              	callq	0x140c96240 <.text+0xc95240>
1407f7f8d: 48 89 c7                    	movq	%rax, %rdi
1407f7f90: 48 b8 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rax # imm = 0xA0761D6478BD642F
1407f7f9a: 48 01 c7                    	addq	%rax, %rdi
1407f7f9d: 48 89 f8                    	movq	%rdi, %rax
1407f7fa0: 48 b9 db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %rcx # imm = 0xE7037ED1A0B428DB
1407f7faa: 48 31 c8                    	xorq	%rcx, %rax
1407f7fad: 48 f7 e7                    	mulq	%rdi
1407f7fb0: 31 d0                       	xorl	%edx, %eax
1407f7fb2: c1 e8 1e                    	shrl	$0x1e, %eax
1407f7fb5: 83 f8 01                    	cmpl	$0x1, %eax
1407f7fb8: 0f 85 e0 02 00 00           	jne	0x1407f829e <.text+0x7f729e>
1407f7fbe: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
1407f7fc5: 0f b6 50 fc                 	movzbl	-0x4(%rax), %edx
1407f7fc9: 44 0f 29 95 c0 01 00 00     	movaps	%xmm10, 0x1c0(%rbp)
1407f7fd1: 66 0f 6f 85 b0 00 00 00     	movdqa	0xb0(%rbp), %xmm0
1407f7fd9: 66 0f d6 85 d0 01 00 00     	movq	%xmm0, 0x1d0(%rbp)
1407f7fe1: f3 44 0f 11 8d d8 01 00 00  	movss	%xmm9, 0x1d8(%rbp)
1407f7fea: 48 8d 85 c4 01 00 00        	leaq	0x1c4(%rbp), %rax
1407f7ff1: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
1407f7ffb: 48 89 48 18                 	movq	%rcx, 0x18(%rax)
1407f7fff: c7 40 20 00 00 80 3f        	movl	$0x3f800000, 0x20(%rax) # imm = 0x3F800000
1407f8006: 48 8b 45 50                 	movq	0x50(%rbp), %rax
1407f800a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f800f: 48 8d 8d 78 01 00 00        	leaq	0x178(%rbp), %rcx
1407f8016: 41 b0 03                    	movb	$0x3, %r8b
1407f8019: 4c 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %r9
1407f8020: e8 8b 77 ff ff              	callq	0x1407ef7b0 <.text+0x7ee7b0>
1407f8025: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1407f802c: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1407f8033: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
1407f803a: 48 85 c0                    	testq	%rax, %rax
1407f803d: 0f 84 35 02 00 00           	je	0x1407f8278 <.text+0x7f7278>
1407f8043: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1407f8047: 48 c1 e0 04                 	shlq	$0x4, %rax
1407f804b: 4c 8b bd b0 00 00 00        	movq	0xb0(%rbp), %r15
1407f8052: 4c 01 f8                    	addq	%r15, %rax
1407f8055: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
1407f805c: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407f8063: 48 05 f8 00 00 00           	addq	$0xf8, %rax
1407f8069: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
1407f8070: 45 31 c9                    	xorl	%r9d, %r9d
1407f8073: e9 95 00 00 00              	jmp	0x1407f810d <.text+0x7f710d>
1407f8078: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1407f8080: 49 83 c7 30                 	addq	$0x30, %r15
1407f8084: 4c 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %r9
1407f808b: 49 ff c1                    	incq	%r9
1407f808e: 4c 8b 85 00 02 00 00        	movq	0x200(%rbp), %r8
1407f8095: 49 8b 80 00 01 00 00        	movq	0x100(%r8), %rax
1407f809c: 48 8d 0c 9b                 	leaq	(%rbx,%rbx,4), %rcx
1407f80a0: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407f80a4: c6 04 08 01                 	movb	$0x1, (%rax,%rcx)
1407f80a8: 48 8b 95 38 01 00 00        	movq	0x138(%rbp), %rdx
1407f80af: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
1407f80b4: 66 0f 6f 85 e0 00 00 00     	movdqa	0xe0(%rbp), %xmm0
1407f80bc: 66 0f 6f 8d f0 00 00 00     	movdqa	0xf0(%rbp), %xmm1
1407f80c4: 66 0f 6f 95 00 01 00 00     	movdqa	0x100(%rbp), %xmm2
1407f80cc: 66 0f 7f 44 08 10           	movdqa	%xmm0, 0x10(%rax,%rcx)
1407f80d2: 66 0f 7f 4c 08 20           	movdqa	%xmm1, 0x20(%rax,%rcx)
1407f80d8: 66 0f 7f 54 08 30           	movdqa	%xmm2, 0x30(%rax,%rcx)
1407f80de: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
1407f80e5: 48 89 54 08 40              	movq	%rdx, 0x40(%rax,%rcx)
1407f80ea: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
1407f80f1: 48 89 54 08 48              	movq	%rdx, 0x48(%rax,%rcx)
1407f80f6: 48 ff c3                    	incq	%rbx
1407f80f9: 49 89 98 08 01 00 00        	movq	%rbx, 0x108(%r8)
1407f8100: 4c 3b bd c8 00 00 00        	cmpq	0xc8(%rbp), %r15
1407f8107: 0f 84 6b 01 00 00           	je	0x1407f8278 <.text+0x7f7278>
1407f810d: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
1407f8112: 66 41 0f 6f 4f 10           	movdqa	0x10(%r15), %xmm1
1407f8118: 66 41 0f 6f 57 20           	movdqa	0x20(%r15), %xmm2
1407f811e: 66 0f 7f 95 e0 01 00 00     	movdqa	%xmm2, 0x1e0(%rbp)
1407f8126: 66 0f 7f 8d d0 01 00 00     	movdqa	%xmm1, 0x1d0(%rbp)
1407f812e: 66 0f 7f 85 c0 01 00 00     	movdqa	%xmm0, 0x1c0(%rbp)
1407f8136: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
1407f813d: 89 85 a0 01 00 00           	movl	%eax, 0x1a0(%rbp)
1407f8143: 48 8d 05 46 41 29 02        	leaq	0x2294146(%rip), %rax   # 0x142a8c290
1407f814a: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407f8151: 48 c7 85 98 01 00 00 0a 00 00 00    	movq	$0xa, 0x198(%rbp)
1407f815c: 4c 89 8d b8 01 00 00        	movq	%r9, 0x1b8(%rbp)
1407f8163: 4c 89 8d a8 01 00 00        	movq	%r9, 0x1a8(%rbp)
1407f816a: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1407f8171: e8 fa 48 a9 00              	callq	0x14128ca70 <.text+0x128ba70>
1407f8176: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1407f817d: 48 b8 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rax # imm = 0xA0761D6478BD642F
1407f8187: 4c 8d 04 07                 	leaq	(%rdi,%rax), %r8
1407f818b: 4c 89 c0                    	movq	%r8, %rax
1407f818e: 48 b9 db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %rcx # imm = 0xE7037ED1A0B428DB
1407f8198: 48 31 c8                    	xorq	%rcx, %rax
1407f819b: 49 f7 e0                    	mulq	%r8
1407f819e: 48 89 d1                    	movq	%rdx, %rcx
1407f81a1: 48 31 c1                    	xorq	%rax, %rcx
1407f81a4: 48 89 c8                    	movq	%rcx, %rax
1407f81a7: ba 06 00 00 00              	movl	$0x6, %edx
1407f81ac: 48 f7 e2                    	mulq	%rdx
1407f81af: 48 01 c9                    	addq	%rcx, %rcx
1407f81b2: 48 8d 04 49                 	leaq	(%rcx,%rcx,2), %rax
1407f81b6: 48 83 f8 03                 	cmpq	$0x3, %rax
1407f81ba: 77 4f                       	ja	0x1407f820b <.text+0x7f720b>
1407f81bc: 48 b8 5e c8 7a f1 c8 3a ec 40       	movabsq	$0x40ec3ac8f17ac85e, %rax # imm = 0x40EC3AC8F17AC85E
1407f81c6: 48 01 c7                    	addq	%rax, %rdi
1407f81c9: 48 b9 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rcx # imm = 0xA0761D6478BD642F
1407f81d3: 49 b9 db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %r9 # imm = 0xE7037ED1A0B428DB
1407f81dd: 0f 1f 00                    	nopl	(%rax)
1407f81e0: 48 89 f8                    	movq	%rdi, %rax
1407f81e3: 4c 31 c8                    	xorq	%r9, %rax
1407f81e6: 48 f7 e7                    	mulq	%rdi
1407f81e9: 49 01 c8                    	addq	%rcx, %r8
1407f81ec: 48 31 c2                    	xorq	%rax, %rdx
1407f81ef: 48 8d 04 12                 	leaq	(%rdx,%rdx), %rax
1407f81f3: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1407f81f7: 48 01 cf                    	addq	%rcx, %rdi
1407f81fa: 48 83 f8 04                 	cmpq	$0x4, %rax
1407f81fe: 72 e0                       	jb	0x1407f81e0 <.text+0x7f71e0>
1407f8200: 48 89 d0                    	movq	%rdx, %rax
1407f8203: b9 06 00 00 00              	movl	$0x6, %ecx
1407f8208: 48 f7 e1                    	mulq	%rcx
1407f820b: 4c 89 c7                    	movq	%r8, %rdi
1407f820e: 48 89 95 38 01 00 00        	movq	%rdx, 0x138(%rbp)
1407f8215: 66 0f 6f 85 c0 01 00 00     	movdqa	0x1c0(%rbp), %xmm0
1407f821d: 66 0f 6f 8d d0 01 00 00     	movdqa	0x1d0(%rbp), %xmm1
1407f8225: 66 0f 6f 95 e0 01 00 00     	movdqa	0x1e0(%rbp), %xmm2
1407f822d: 66 0f 7f 95 00 01 00 00     	movdqa	%xmm2, 0x100(%rbp)
1407f8235: 66 0f 7f 8d f0 00 00 00     	movdqa	%xmm1, 0xf0(%rbp)
1407f823d: 66 0f 7f 85 e0 00 00 00     	movdqa	%xmm0, 0xe0(%rbp)
1407f8245: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407f824c: 48 8b 98 08 01 00 00        	movq	0x108(%rax), %rbx
1407f8253: 48 3b 98 f8 00 00 00        	cmpq	0xf8(%rax), %rbx
1407f825a: 0f 85 20 fe ff ff           	jne	0x1407f8080 <.text+0x7f7080>
1407f8260: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
1407f8267: 48 8d 15 32 40 29 02        	leaq	0x2294032(%rip), %rdx   # 0x142a8c2a0
1407f826e: e8 8d 47 f9 ff              	callq	0x14078ca00 <.text+0x78ba00>
1407f8273: e9 08 fe ff ff              	jmp	0x1407f8080 <.text+0x7f7080>
1407f8278: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
1407f827f: 48 85 c0                    	testq	%rax, %rax
1407f8282: 74 1a                       	je	0x1407f829e <.text+0x7f729e>
1407f8284: 48 c1 e0 04                 	shlq	$0x4, %rax
1407f8288: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1407f828c: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1407f8292: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
1407f8299: e8 82 b9 e1 ff              	callq	0x140613c20 <.text+0x612c20>
1407f829e: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
1407f82a5: 48 85 c0                    	testq	%rax, %rax
1407f82a8: 74 1a                       	je	0x1407f82c4 <.text+0x7f72c4>
1407f82aa: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
1407f82b1: 48 c1 e0 04                 	shlq	$0x4, %rax
1407f82b5: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1407f82b9: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1407f82bf: e8 5c b9 e1 ff              	callq	0x140613c20 <.text+0x612c20>
1407f82c4: 8b 85 74 01 00 00           	movl	0x174(%rbp), %eax
1407f82ca: 89 c1                       	movl	%eax, %ecx
1407f82cc: e9 e0 e7 ff ff              	jmp	0x1407f6ab1 <.text+0x7f5ab1>
1407f82d1: 8b 85 5c 01 00 00           	movl	0x15c(%rbp), %eax
1407f82d7: 4c 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %r8
1407f82de: 4c 89 85 a8 00 00 00        	movq	%r8, 0xa8(%rbp)
1407f82e5: 4d 85 c0                    	testq	%r8, %r8
1407f82e8: 0f 85 04 e7 ff ff           	jne	0x1407f69f2 <.text+0x7f59f2>
1407f82ee: 80 bd 16 02 00 00 00        	cmpb	$0x0, 0x216(%rbp)
1407f82f5: 74 4b                       	je	0x1407f8342 <.text+0x7f7342>
1407f82f7: e8 34 fe 41 00              	callq	0x140c18130 <.text+0xc17130>
1407f82fc: 48 89 c1                    	movq	%rax, %rcx
1407f82ff: 48 8b 00                    	movq	(%rax), %rax
1407f8302: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f8306: 0f 85 ac 01 00 00           	jne	0x1407f84b8 <.text+0x7f74b8>
1407f830c: 48 83 c1 08                 	addq	$0x8, %rcx
1407f8310: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407f8314: 0f 85 df 01 00 00           	jne	0x1407f84f9 <.text+0x7f74f9>
1407f831a: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407f8321: 48 89 8d 60 01 00 00        	movq	%rcx, 0x160(%rbp)
1407f8328: 48 83 c1 08                 	addq	$0x8, %rcx
1407f832c: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
1407f8333: e8 b8 ed 41 00              	callq	0x140c170f0 <.text+0xc160f0>
1407f8338: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1407f833f: 48 ff 00                    	incq	(%rax)
1407f8342: 0f 28 b5 20 02 00 00        	movaps	0x220(%rbp), %xmm6
1407f8349: 0f 28 bd 30 02 00 00        	movaps	0x230(%rbp), %xmm7
1407f8350: 44 0f 28 85 40 02 00 00     	movaps	0x240(%rbp), %xmm8
1407f8358: 44 0f 28 8d 50 02 00 00     	movaps	0x250(%rbp), %xmm9
1407f8360: 44 0f 28 95 60 02 00 00     	movaps	0x260(%rbp), %xmm10
1407f8368: 44 0f 28 9d 70 02 00 00     	movaps	0x270(%rbp), %xmm11
1407f8370: 44 0f 28 a5 80 02 00 00     	movaps	0x280(%rbp), %xmm12
1407f8378: 44 0f 28 ad 90 02 00 00     	movaps	0x290(%rbp), %xmm13
1407f8380: 44 0f 28 b5 a0 02 00 00     	movaps	0x2a0(%rbp), %xmm14
1407f8388: 44 0f 28 bd b0 02 00 00     	movaps	0x2b0(%rbp), %xmm15
1407f8390: 48 81 c4 48 03 00 00        	addq	$0x348, %rsp            # imm = 0x348
1407f8397: 5b                          	popq	%rbx
1407f8398: 5f                          	popq	%rdi
1407f8399: 5e                          	popq	%rsi
1407f839a: 41 5c                       	popq	%r12
1407f839c: 41 5d                       	popq	%r13
1407f839e: 41 5e                       	popq	%r14
1407f83a0: 41 5f                       	popq	%r15
1407f83a2: 5d                          	popq	%rbp
1407f83a3: c3                          	retq
1407f83a4: 48 8d 0d 3d 8f 42 02        	leaq	0x2428f3d(%rip), %rcx   # 0x142c212e8
1407f83ab: e8 60 0f 0e 02              	callq	0x1428d9310 <.text+0x28d8310>
1407f83b0: e9 4c 03 00 00              	jmp	0x1407f8701 <.text+0x7f7701>
1407f83b5: 48 8d 85 90 01 00 00        	leaq	0x190(%rbp), %rax
1407f83bc: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1407f83c3: 48 8d 05 c6 dd ff ff        	leaq	-0x223a(%rip), %rax     # 0x1407f6190 <.text+0x7f5190>
1407f83ca: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1407f83d1: 48 8d 05 d8 39 29 02        	leaq	0x22939d8(%rip), %rax   # 0x142a8bdb0
1407f83d8: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407f83df: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
1407f83ea: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
1407f83f5: 48 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %rax
1407f83fc: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1407f8403: 48 c7 85 d8 01 00 00 01 00 00 00    	movq	$0x1, 0x1d8(%rbp)
1407f840e: 48 8d 15 ab 39 29 02        	leaq	0x22939ab(%rip), %rdx   # 0x142a8bdc0
1407f8415: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f841c: e8 6f 0f 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f8421: e9 db 02 00 00              	jmp	0x1407f8701 <.text+0x7f7701>
1407f8426: 48 8d 85 78 01 00 00        	leaq	0x178(%rbp), %rax
1407f842d: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1407f8434: 48 8d 05 55 dd ff ff        	leaq	-0x22ab(%rip), %rax     # 0x1407f6190 <.text+0x7f5190>
1407f843b: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1407f8442: 48 8d 05 67 39 29 02        	leaq	0x2293967(%rip), %rax   # 0x142a8bdb0
1407f8449: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407f8450: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
1407f845b: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
1407f8466: 48 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %rax
1407f846d: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1407f8474: 48 c7 85 d8 01 00 00 01 00 00 00    	movq	$0x1, 0x1d8(%rbp)
1407f847f: c6 85 17 02 00 00 01        	movb	$0x1, 0x217(%rbp)
1407f8486: 48 8d 15 4b 39 29 02        	leaq	0x229394b(%rip), %rdx   # 0x142a8bdd8
1407f848d: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f8494: e8 f7 0e 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f8499: e9 63 02 00 00              	jmp	0x1407f8701 <.text+0x7f7701>
1407f849e: 48 85 c0                    	testq	%rax, %rax
1407f84a1: 75 2d                       	jne	0x1407f84d0 <.text+0x7f74d0>
1407f84a3: 31 d2                       	xorl	%edx, %edx
1407f84a5: e8 d6 a0 0f 02              	callq	0x1428f2580 <.text+0x28f1580>
1407f84aa: 48 89 c1                    	movq	%rax, %rcx
1407f84ad: 48 85 c0                    	testq	%rax, %rax
1407f84b0: 0f 85 54 e4 ff ff           	jne	0x1407f690a <.text+0x7f590a>
1407f84b6: eb 18                       	jmp	0x1407f84d0 <.text+0x7f74d0>
1407f84b8: 48 85 c0                    	testq	%rax, %rax
1407f84bb: 75 13                       	jne	0x1407f84d0 <.text+0x7f74d0>
1407f84bd: 31 d2                       	xorl	%edx, %edx
1407f84bf: e8 bc a0 0f 02              	callq	0x1428f2580 <.text+0x28f1580>
1407f84c4: 48 89 c1                    	movq	%rax, %rcx
1407f84c7: 48 85 c0                    	testq	%rax, %rax
1407f84ca: 0f 85 40 fe ff ff           	jne	0x1407f8310 <.text+0x7f7310>
1407f84d0: 48 8d 0d f1 8e 42 02        	leaq	0x2428ef1(%rip), %rcx   # 0x142c213c8
1407f84d7: e8 f4 fb 0d 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407f84dc: 48 8d 0d b5 3b 29 02        	leaq	0x2293bb5(%rip), %rcx   # 0x142a8c098
1407f84e3: 4c 8d 05 d6 3b 29 02        	leaq	0x2293bd6(%rip), %r8    # 0x142a8c0c0
1407f84ea: ba 28 00 00 00              	movl	$0x28, %edx
1407f84ef: e8 3c 0f 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f84f4: e9 08 02 00 00              	jmp	0x1407f8701 <.text+0x7f7701>
1407f84f9: 48 8d 0d 38 8f 42 02        	leaq	0x2428f38(%rip), %rcx   # 0x142c21438
1407f8500: e8 4b 0d 0e 02              	callq	0x1428d9250 <.text+0x28d8250>
1407f8505: 48 8d 05 1c 92 29 02        	leaq	0x229921c(%rip), %rax   # 0x142a91728
1407f850c: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407f8513: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
1407f851e: 48 c7 85 d0 01 00 00 08 00 00 00    	movq	$0x8, 0x1d0(%rbp)
1407f8529: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1407f852d: f3 0f 7f 85 d8 01 00 00     	movdqu	%xmm0, 0x1d8(%rbp)
1407f8535: 48 8d 15 fc 91 29 02        	leaq	0x22991fc(%rip), %rdx   # 0x142a91738
1407f853c: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f8543: e8 48 0e 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f8548: e9 b4 01 00 00              	jmp	0x1407f8701 <.text+0x7f7701>
1407f854d: 48 8d 0d 44 3b 29 02        	leaq	0x2293b44(%rip), %rcx   # 0x142a8c098
1407f8554: 4c 8d 05 d5 3d 29 02        	leaq	0x2293dd5(%rip), %r8    # 0x142a8c330
1407f855b: ba 28 00 00 00              	movl	$0x28, %edx
1407f8560: e8 cb 0e 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f8565: e9 97 01 00 00              	jmp	0x1407f8701 <.text+0x7f7701>
1407f856a: 48 8d 0d 27 3b 29 02        	leaq	0x2293b27(%rip), %rcx   # 0x142a8c098
1407f8571: 4c 8d 05 a0 3d 29 02        	leaq	0x2293da0(%rip), %r8    # 0x142a8c318
1407f8578: ba 28 00 00 00              	movl	$0x28, %edx
1407f857d: e8 ae 0e 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f8582: e9 7a 01 00 00              	jmp	0x1407f8701 <.text+0x7f7701>
1407f8587: 48 8d 0d c2 3a 29 02        	leaq	0x2293ac2(%rip), %rcx   # 0x142a8c050
1407f858e: e8 7d 0d 0e 02              	callq	0x1428d9310 <.text+0x28d8310>
1407f8593: e9 69 01 00 00              	jmp	0x1407f8701 <.text+0x7f7701>
1407f8598: 4c 8d 05 e1 3a 29 02        	leaq	0x2293ae1(%rip), %r8    # 0x142a8c080
1407f859f: b9 10 00 00 00              	movl	$0x10, %ecx
1407f85a4: 48 89 da                    	movq	%rbx, %rdx
1407f85a7: e8 37 0a 0e 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1407f85ac: e9 50 01 00 00              	jmp	0x1407f8701 <.text+0x7f7701>
1407f85b1: 48 8d 85 90 01 00 00        	leaq	0x190(%rbp), %rax
1407f85b8: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1407f85bf: 48 8d 05 2a 49 a6 00        	leaq	0xa6492a(%rip), %rax    # 0x14125cef0 <.text+0x125bef0>
1407f85c6: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1407f85cd: 48 8d 05 ac 96 42 02        	leaq	0x24296ac(%rip), %rax   # 0x142c21c80
1407f85d4: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407f85db: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
1407f85e6: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
1407f85f1: 48 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %rax
1407f85f8: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1407f85ff: 48 c7 85 d8 01 00 00 01 00 00 00    	movq	$0x1, 0x1d8(%rbp)
1407f860a: 48 8d 15 97 96 42 02        	leaq	0x2429697(%rip), %rdx   # 0x142c21ca8
1407f8611: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f8618: e8 73 0d 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f861d: e9 df 00 00 00              	jmp	0x1407f8701 <.text+0x7f7701>
1407f8622: 48 8d 0d 6f 3a 29 02        	leaq	0x2293a6f(%rip), %rcx   # 0x142a8c098
1407f8629: 4c 8d 05 b8 3c 29 02        	leaq	0x2293cb8(%rip), %r8    # 0x142a8c2e8
1407f8630: ba 28 00 00 00              	movl	$0x28, %edx
1407f8635: e8 f6 0d 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f863a: e9 c2 00 00 00              	jmp	0x1407f8701 <.text+0x7f7701>
1407f863f: 48 8d 0d ba 3c 29 02        	leaq	0x2293cba(%rip), %rcx   # 0x142a8c300
1407f8646: e8 c5 0c 0e 02              	callq	0x1428d9310 <.text+0x28d8310>
1407f864b: e9 b1 00 00 00              	jmp	0x1407f8701 <.text+0x7f7701>
1407f8650: 48 8d 85 90 01 00 00        	leaq	0x190(%rbp), %rax
1407f8657: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1407f865e: 48 8d 05 8b 48 a6 00        	leaq	0xa6488b(%rip), %rax    # 0x14125cef0 <.text+0x125bef0>
1407f8665: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1407f866c: 48 8d 05 0d 96 42 02        	leaq	0x242960d(%rip), %rax   # 0x142c21c80
1407f8673: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407f867a: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
1407f8685: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
1407f8690: 48 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %rax
1407f8697: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1407f869e: 48 c7 85 d8 01 00 00 01 00 00 00    	movq	$0x1, 0x1d8(%rbp)
1407f86a9: 48 8d 15 f8 95 42 02        	leaq	0x24295f8(%rip), %rdx   # 0x142c21ca8
1407f86b0: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407f86b7: e8 d4 0c 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f86bc: eb 43                       	jmp	0x1407f8701 <.text+0x7f7701>
1407f86be: 48 8d 0d d3 39 29 02        	leaq	0x22939d3(%rip), %rcx   # 0x142a8c098
1407f86c5: 4c 8d 05 04 3c 29 02        	leaq	0x2293c04(%rip), %r8    # 0x142a8c2d0
1407f86cc: ba 28 00 00 00              	movl	$0x28, %edx
1407f86d1: e8 5a 0d 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f86d6: eb 29                       	jmp	0x1407f8701 <.text+0x7f7701>
1407f86d8: b9 10 00 00 00              	movl	$0x10, %ecx
1407f86dd: ba 50 00 00 00              	movl	$0x50, %edx
1407f86e2: e8 19 09 0e 02              	callq	0x1428d9000 <.text+0x28d8000>
1407f86e7: eb 18                       	jmp	0x1407f8701 <.text+0x7f7701>
1407f86e9: 48 8d 0d a8 39 29 02        	leaq	0x22939a8(%rip), %rcx   # 0x142a8c098
1407f86f0: 4c 8d 05 c1 3b 29 02        	leaq	0x2293bc1(%rip), %r8    # 0x142a8c2b8
1407f86f7: ba 28 00 00 00              	movl	$0x28, %edx
1407f86fc: e8 2f 0d 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f8701: 0f 0b                       	ud2
1407f8703: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407f8710: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f8715: 55                          	pushq	%rbp
1407f8716: 41 57                       	pushq	%r15
1407f8718: 41 56                       	pushq	%r14
1407f871a: 41 55                       	pushq	%r13
1407f871c: 41 54                       	pushq	%r12
1407f871e: 56                          	pushq	%rsi
1407f871f: 57                          	pushq	%rdi
1407f8720: 53                          	pushq	%rbx
1407f8721: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407f8728: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f872f: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407f8735: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407f873b: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407f8744: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407f874d: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407f8756: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407f875f: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407f8768: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407f8771: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407f8779: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407f8781: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
1407f8788: 48 ff 00                    	incq	(%rax)
1407f878b: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407f8793: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407f879b: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407f87a4: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407f87ad: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407f87b6: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407f87bf: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407f87c8: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407f87d1: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407f87d7: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407f87dd: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407f87e4: 5b                          	popq	%rbx
1407f87e5: 5f                          	popq	%rdi
1407f87e6: 5e                          	popq	%rsi
1407f87e7: 41 5c                       	popq	%r12
1407f87e9: 41 5d                       	popq	%r13
1407f87eb: 41 5e                       	popq	%r14
1407f87ed: 41 5f                       	popq	%r15
1407f87ef: 5d                          	popq	%rbp
1407f87f0: c3                          	retq
1407f87f1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1407f8800: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f8805: 55                          	pushq	%rbp
1407f8806: 41 57                       	pushq	%r15
1407f8808: 41 56                       	pushq	%r14
1407f880a: 41 55                       	pushq	%r13
1407f880c: 41 54                       	pushq	%r12
1407f880e: 56                          	pushq	%rsi
1407f880f: 57                          	pushq	%rdi
1407f8810: 53                          	pushq	%rbx
1407f8811: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407f8818: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f881f: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407f8825: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407f882b: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407f8834: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407f883d: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407f8846: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407f884f: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407f8858: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407f8861: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407f8869: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407f8871: 80 bd 16 02 00 00 00        	cmpb	$0x0, 0x216(%rbp)
1407f8878: 74 3d                       	je	0x1407f88b7 <.text+0x7f78b7>
1407f887a: e8 b1 f8 41 00              	callq	0x140c18130 <.text+0xc17130>
1407f887f: 48 89 c6                    	movq	%rax, %rsi
1407f8882: 48 8b 00                    	movq	(%rax), %rax
1407f8885: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f8889: 0f 85 8e 00 00 00           	jne	0x1407f891d <.text+0x7f791d>
1407f888f: 48 83 c6 08                 	addq	$0x8, %rsi
1407f8893: 48 83 3e 00                 	cmpq	$0x0, (%rsi)
1407f8897: 0f 85 a7 00 00 00           	jne	0x1407f8944 <.text+0x7f7944>
1407f889d: 48 c7 06 ff ff ff ff        	movq	$-0x1, (%rsi)
1407f88a4: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
1407f88a8: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
1407f88af: e8 3c e8 41 00              	callq	0x140c170f0 <.text+0xc160f0>
1407f88b4: 48 ff 06                    	incq	(%rsi)
1407f88b7: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407f88bf: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407f88c7: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407f88d0: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407f88d9: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407f88e2: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407f88eb: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407f88f4: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407f88fd: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407f8903: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407f8909: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407f8910: 5b                          	popq	%rbx
1407f8911: 5f                          	popq	%rdi
1407f8912: 5e                          	popq	%rsi
1407f8913: 41 5c                       	popq	%r12
1407f8915: 41 5d                       	popq	%r13
1407f8917: 41 5e                       	popq	%r14
1407f8919: 41 5f                       	popq	%r15
1407f891b: 5d                          	popq	%rbp
1407f891c: c3                          	retq
1407f891d: 48 85 c0                    	testq	%rax, %rax
1407f8920: 75 16                       	jne	0x1407f8938 <.text+0x7f7938>
1407f8922: 48 89 f1                    	movq	%rsi, %rcx
1407f8925: 31 d2                       	xorl	%edx, %edx
1407f8927: e8 54 9c 0f 02              	callq	0x1428f2580 <.text+0x28f1580>
1407f892c: 48 89 c6                    	movq	%rax, %rsi
1407f892f: 48 85 c0                    	testq	%rax, %rax
1407f8932: 0f 85 5b ff ff ff           	jne	0x1407f8893 <.text+0x7f7893>
1407f8938: 48 8d 0d 89 8a 42 02        	leaq	0x2428a89(%rip), %rcx   # 0x142c213c8
1407f893f: e8 8c f7 0d 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407f8944: 48 8d 0d ed 8a 42 02        	leaq	0x2428aed(%rip), %rcx   # 0x142c21438
1407f894b: e8 00 09 0e 02              	callq	0x1428d9250 <.text+0x28d8250>
1407f8950: cc                          	int3
1407f8951: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1407f8960: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f8965: 55                          	pushq	%rbp
1407f8966: 41 57                       	pushq	%r15
1407f8968: 41 56                       	pushq	%r14
1407f896a: 41 55                       	pushq	%r13
1407f896c: 41 54                       	pushq	%r12
1407f896e: 56                          	pushq	%rsi
1407f896f: 57                          	pushq	%rdi
1407f8970: 53                          	pushq	%rbx
1407f8971: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407f8978: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f897f: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407f8985: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407f898b: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407f8994: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407f899d: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407f89a6: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407f89af: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407f89b8: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407f89c1: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407f89c9: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407f89d1: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1407f89d8: 48 ff 00                    	incq	(%rax)
1407f89db: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407f89e3: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407f89eb: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407f89f4: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407f89fd: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407f8a06: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407f8a0f: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407f8a18: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407f8a21: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407f8a27: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407f8a2d: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407f8a34: 5b                          	popq	%rbx
1407f8a35: 5f                          	popq	%rdi
1407f8a36: 5e                          	popq	%rsi
1407f8a37: 41 5c                       	popq	%r12
1407f8a39: 41 5d                       	popq	%r13
1407f8a3b: 41 5e                       	popq	%r14
1407f8a3d: 41 5f                       	popq	%r15
1407f8a3f: 5d                          	popq	%rbp
1407f8a40: c3                          	retq
1407f8a41: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1407f8a50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f8a55: 55                          	pushq	%rbp
1407f8a56: 41 57                       	pushq	%r15
1407f8a58: 41 56                       	pushq	%r14
1407f8a5a: 41 55                       	pushq	%r13
1407f8a5c: 41 54                       	pushq	%r12
1407f8a5e: 56                          	pushq	%rsi
1407f8a5f: 57                          	pushq	%rdi
1407f8a60: 53                          	pushq	%rbx
1407f8a61: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407f8a68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f8a6f: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407f8a75: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407f8a7b: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407f8a84: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407f8a8d: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407f8a96: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407f8a9f: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407f8aa8: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407f8ab1: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407f8ab9: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407f8ac1: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
1407f8ac8: 48 85 c0                    	testq	%rax, %rax
1407f8acb: 74 1a                       	je	0x1407f8ae7 <.text+0x7f7ae7>
1407f8acd: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
1407f8ad4: 48 c1 e0 04                 	shlq	$0x4, %rax
1407f8ad8: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1407f8adc: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1407f8ae2: e8 39 b1 e1 ff              	callq	0x140613c20 <.text+0x612c20>
1407f8ae7: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407f8aef: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407f8af7: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407f8b00: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407f8b09: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407f8b12: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407f8b1b: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407f8b24: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407f8b2d: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407f8b33: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407f8b39: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407f8b40: 5b                          	popq	%rbx
1407f8b41: 5f                          	popq	%rdi
1407f8b42: 5e                          	popq	%rsi
1407f8b43: 41 5c                       	popq	%r12
1407f8b45: 41 5d                       	popq	%r13
1407f8b47: 41 5e                       	popq	%r14
1407f8b49: 41 5f                       	popq	%r15
1407f8b4b: 5d                          	popq	%rbp
1407f8b4c: c3                          	retq
1407f8b4d: 0f 1f 00                    	nopl	(%rax)
1407f8b50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f8b55: 55                          	pushq	%rbp
1407f8b56: 41 57                       	pushq	%r15
1407f8b58: 41 56                       	pushq	%r14
1407f8b5a: 41 55                       	pushq	%r13
1407f8b5c: 41 54                       	pushq	%r12
1407f8b5e: 56                          	pushq	%rsi
1407f8b5f: 57                          	pushq	%rdi
1407f8b60: 53                          	pushq	%rbx
1407f8b61: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407f8b68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f8b6f: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407f8b75: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407f8b7b: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407f8b84: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407f8b8d: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407f8b96: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407f8b9f: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407f8ba8: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407f8bb1: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407f8bb9: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407f8bc1: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
1407f8bc8: 48 85 c0                    	testq	%rax, %rax
1407f8bcb: 74 1a                       	je	0x1407f8be7 <.text+0x7f7be7>
1407f8bcd: 48 c1 e0 04                 	shlq	$0x4, %rax
1407f8bd1: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1407f8bd5: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1407f8bdb: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
1407f8be2: e8 39 b0 e1 ff              	callq	0x140613c20 <.text+0x612c20>
1407f8be7: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407f8bef: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407f8bf7: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407f8c00: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407f8c09: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407f8c12: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407f8c1b: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407f8c24: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407f8c2d: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407f8c33: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407f8c39: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407f8c40: 5b                          	popq	%rbx
1407f8c41: 5f                          	popq	%rdi
1407f8c42: 5e                          	popq	%rsi
1407f8c43: 41 5c                       	popq	%r12
1407f8c45: 41 5d                       	popq	%r13
1407f8c47: 41 5e                       	popq	%r14
1407f8c49: 41 5f                       	popq	%r15
1407f8c4b: 5d                          	popq	%rbp
1407f8c4c: c3                          	retq
1407f8c4d: 0f 1f 00                    	nopl	(%rax)
1407f8c50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f8c55: 55                          	pushq	%rbp
1407f8c56: 41 57                       	pushq	%r15
1407f8c58: 41 56                       	pushq	%r14
1407f8c5a: 41 55                       	pushq	%r13
1407f8c5c: 41 54                       	pushq	%r12
1407f8c5e: 56                          	pushq	%rsi
1407f8c5f: 57                          	pushq	%rdi
1407f8c60: 53                          	pushq	%rbx
1407f8c61: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407f8c68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f8c6f: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407f8c75: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407f8c7b: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407f8c84: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407f8c8d: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407f8c96: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407f8c9f: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407f8ca8: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407f8cb1: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407f8cb9: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407f8cc1: 80 bd 17 02 00 00 00        	cmpb	$0x0, 0x217(%rbp)
1407f8cc8: 74 7f                       	je	0x1407f8d49 <.text+0x7f7d49>
1407f8cca: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
1407f8cd1: f0                          	lock
1407f8cd2: 48 ff 08                    	decq	(%rax)
1407f8cd5: 75 0c                       	jne	0x1407f8ce3 <.text+0x7f7ce3>
1407f8cd7: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1407f8cde: e8 3d 39 36 00              	callq	0x140b5c620 <.text+0xb5b620>
1407f8ce3: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407f8ceb: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407f8cf3: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407f8cfc: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407f8d05: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407f8d0e: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407f8d17: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407f8d20: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407f8d29: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407f8d2f: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407f8d35: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407f8d3c: 5b                          	popq	%rbx
1407f8d3d: 5f                          	popq	%rdi
1407f8d3e: 5e                          	popq	%rsi
1407f8d3f: 41 5c                       	popq	%r12
1407f8d41: 41 5d                       	popq	%r13
1407f8d43: 41 5e                       	popq	%r14
1407f8d45: 41 5f                       	popq	%r15
1407f8d47: 5d                          	popq	%rbp
1407f8d48: c3                          	retq
1407f8d49: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407f8d51: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407f8d59: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407f8d62: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407f8d6b: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407f8d74: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407f8d7d: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407f8d86: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407f8d8f: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407f8d95: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407f8d9b: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407f8da2: 5b                          	popq	%rbx
1407f8da3: 5f                          	popq	%rdi
1407f8da4: 5e                          	popq	%rsi
1407f8da5: 41 5c                       	popq	%r12
1407f8da7: 41 5d                       	popq	%r13
1407f8da9: 41 5e                       	popq	%r14
1407f8dab: 41 5f                       	popq	%r15
1407f8dad: 5d                          	popq	%rbp
1407f8dae: c3                          	retq
