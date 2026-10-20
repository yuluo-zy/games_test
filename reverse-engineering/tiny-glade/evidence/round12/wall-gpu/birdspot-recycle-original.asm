
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1422f86e0: 41 57                       	pushq	%r15
1422f86e2: 41 56                       	pushq	%r14
1422f86e4: 41 54                       	pushq	%r12
1422f86e6: 56                          	pushq	%rsi
1422f86e7: 57                          	pushq	%rdi
1422f86e8: 53                          	pushq	%rbx
1422f86e9: 48 83 ec 28                 	subq	$0x28, %rsp
1422f86ed: 48 89 d7                    	movq	%rdx, %rdi
1422f86f0: 48 8b 19                    	movq	(%rcx), %rbx
1422f86f3: 48 8d 73 10                 	leaq	0x10(%rbx), %rsi
1422f86f7: b1 01                       	movb	$0x1, %cl
1422f86f9: 31 c0                       	xorl	%eax, %eax
1422f86fb: f0                          	lock
1422f86fc: 0f b0 4b 10                 	cmpxchgb	%cl, 0x10(%rbx)
1422f8700: 0f 85 2a 01 00 00           	jne	0x1422f8830 <.text+0x22f7830>
1422f8706: 48 89 f1                    	movq	%rsi, %rcx
1422f8709: e8 62 95 98 fe              	callq	0x140c81c70 <.text+0xc80c70>
1422f870e: 48 8b 43 20                 	movq	0x20(%rbx), %rax
1422f8712: 4c 8b 4b 28                 	movq	0x28(%rbx), %r9
1422f8716: 48 c7 43 28 00 00 00 00     	movq	$0x0, 0x28(%rbx)
1422f871e: 4d 85 c9                    	testq	%r9, %r9
1422f8721: 0f 84 e9 00 00 00           	je	0x1422f8810 <.text+0x22f7810>
1422f8727: 48 8b 0f                    	movq	(%rdi), %rcx
1422f872a: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
1422f872e: 44 8b 47 14                 	movl	0x14(%rdi), %r8d
1422f8732: 49 c1 e1 03                 	shlq	$0x3, %r9
1422f8736: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
1422f873a: 45 31 d2                    	xorl	%r10d, %r10d
1422f873d: 41 bb ff ff ff ff           	movl	$0xffffffff, %r11d      # imm = 0xFFFFFFFF
1422f8743: eb 18                       	jmp	0x1422f875d <.text+0x22f775d>
1422f8745: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1422f8750: 49 83 c2 18                 	addq	$0x18, %r10
1422f8754: 4d 39 d1                    	cmpq	%r10, %r9
1422f8757: 0f 84 b3 00 00 00           	je	0x1422f8810 <.text+0x22f7810>
1422f875d: 4a 8b 1c 10                 	movq	(%rax,%r10), %rbx
1422f8761: 48 89 df                    	movq	%rbx, %rdi
1422f8764: 48 c1 ef 20                 	shrq	$0x20, %rdi
1422f8768: 74 e6                       	je	0x1422f8750 <.text+0x22f7750>
1422f876a: 89 db                       	movl	%ebx, %ebx
1422f876c: 48 3b 5a 10                 	cmpq	0x10(%rdx), %rbx
1422f8770: 73 de                       	jae	0x1422f8750 <.text+0x22f7750>
1422f8772: 4c 8b 72 08                 	movq	0x8(%rdx), %r14
1422f8776: 48 8d 1c 9b                 	leaq	(%rbx,%rbx,4), %rbx
1422f877a: 41 39 3c 9e                 	cmpl	%edi, (%r14,%rbx,4)
1422f877e: 75 d0                       	jne	0x1422f8750 <.text+0x22f7750>
1422f8780: 49 8d 3c 9e                 	leaq	(%r14,%rbx,4), %rdi
1422f8784: 8b 5f 04                    	movl	0x4(%rdi), %ebx
1422f8787: 4c 39 db                    	cmpq	%r11, %rbx
1422f878a: 74 c4                       	je	0x1422f8750 <.text+0x22f7750>
1422f878c: 48 39 59 38                 	cmpq	%rbx, 0x38(%rcx)
1422f8790: 76 be                       	jbe	0x1422f8750 <.text+0x22f7750>
1422f8792: 4c 8b 71 28                 	movq	0x28(%rcx), %r14
1422f8796: 41 89 df                    	movl	%ebx, %r15d
1422f8799: 41 c1 ef 06                 	shrl	$0x6, %r15d
1422f879d: 4f 8b 34 fe                 	movq	(%r14,%r15,8), %r14
1422f87a1: 49 0f a3 de                 	btq	%rbx, %r14
1422f87a5: 73 a9                       	jae	0x1422f8750 <.text+0x22f7750>
1422f87a7: 8b 5f 0c                    	movl	0xc(%rdi), %ebx
1422f87aa: 8b 7f 10                    	movl	0x10(%rdi), %edi
1422f87ad: 4c 8b b2 a8 01 00 00        	movq	0x1a8(%rdx), %r14
1422f87b4: 4c 8d 3c db                 	leaq	(%rbx,%rbx,8), %r15
1422f87b8: 4c 8b a1 18 01 00 00        	movq	0x118(%rcx), %r12
1422f87bf: 4b 8b 5c fe 18              	movq	0x18(%r14,%r15,8), %rbx
1422f87c4: 4f 8b 74 fe 38              	movq	0x38(%r14,%r15,8), %r14
1422f87c9: 4f 8b 34 e6                 	movq	(%r14,%r12,8), %r14
1422f87cd: 49 f7 d6                    	notq	%r14
1422f87d0: 4f 8d 34 76                 	leaq	(%r14,%r14,2), %r14
1422f87d4: 49 c1 e6 04                 	shlq	$0x4, %r14
1422f87d8: 4e 8b 7c 33 10              	movq	0x10(%rbx,%r14), %r15
1422f87dd: 4c 8d 24 bf                 	leaq	(%rdi,%rdi,4), %r12
1422f87e1: 49 c1 e4 04                 	shlq	$0x4, %r12
1422f87e5: 43 83 7c 27 30 00           	cmpl	$0x0, 0x30(%r15,%r12)
1422f87eb: 0f 85 5f ff ff ff           	jne	0x1422f8750 <.text+0x22f7750>
1422f87f1: 4c 01 f3                    	addq	%r14, %rbx
1422f87f4: 4f 8d 34 27                 	leaq	(%r15,%r12), %r14
1422f87f8: 49 83 c6 30                 	addq	$0x30, %r14
1422f87fc: 48 8b 5b 28                 	movq	0x28(%rbx), %rbx
1422f8800: 44 89 04 bb                 	movl	%r8d, (%rbx,%rdi,4)
1422f8804: 41 c7 06 01 00 00 00        	movl	$0x1, (%r14)
1422f880b: e9 40 ff ff ff              	jmp	0x1422f8750 <.text+0x22f7750>
1422f8810: 48 89 f1                    	movq	%rsi, %rcx
1422f8813: e8 f8 95 98 fe              	callq	0x140c81e10 <.text+0xc80e10>
1422f8818: 31 c9                       	xorl	%ecx, %ecx
1422f881a: b0 01                       	movb	$0x1, %al
1422f881c: f0                          	lock
1422f881d: 0f b0 0e                    	cmpxchgb	%cl, (%rsi)
1422f8820: 75 21                       	jne	0x1422f8843 <.text+0x22f7843>
1422f8822: 48 83 c4 28                 	addq	$0x28, %rsp
1422f8826: 5b                          	popq	%rbx
1422f8827: 5f                          	popq	%rdi
1422f8828: 5e                          	popq	%rsi
1422f8829: 41 5c                       	popq	%r12
1422f882b: 41 5e                       	popq	%r14
1422f882d: 41 5f                       	popq	%r15
1422f882f: c3                          	retq
1422f8830: 48 89 f1                    	movq	%rsi, %rcx
1422f8833: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
1422f8839: e8 d2 49 5d 00              	callq	0x1428cd210 <.text+0x28cc210>
1422f883e: e9 c3 fe ff ff              	jmp	0x1422f8706 <.text+0x22f7706>
1422f8843: 48 89 f1                    	movq	%rsi, %rcx
1422f8846: 31 d2                       	xorl	%edx, %edx
1422f8848: 48 83 c4 28                 	addq	$0x28, %rsp
1422f884c: 5b                          	popq	%rbx
1422f884d: 5f                          	popq	%rdi
1422f884e: 5e                          	popq	%rsi
1422f884f: 41 5c                       	popq	%r12
1422f8851: 41 5e                       	popq	%r14
1422f8853: 41 5f                       	popq	%r15
1422f8855: e9 36 53 5d 00              	jmp	0x1428cdb90 <.text+0x28ccb90>
1422f885a: cc                          	int3
1422f885b: cc                          	int3
1422f885c: cc                          	int3
1422f885d: cc                          	int3
1422f885e: cc                          	int3
1422f885f: cc                          	int3
1422f8860: 56                          	pushq	%rsi
1422f8861: 57                          	pushq	%rdi
1422f8862: 48 83 ec 78                 	subq	$0x78, %rsp
1422f8866: 48 89 d6                    	movq	%rdx, %rsi
1422f8869: 48 89 cf                    	movq	%rcx, %rdi
1422f886c: 48 8b 11                    	movq	(%rcx), %rdx
1422f886f: 0f b6 86 91 00 00 00        	movzbl	0x91(%rsi), %eax
1422f8876: 48 39 56 40                 	cmpq	%rdx, 0x40(%rsi)
1422f887a: 76 63                       	jbe	0x1422f88df <.text+0x22f78df>
1422f887c: 48 8b 4e 30                 	movq	0x30(%rsi), %rcx
1422f8880: 49 89 d0                    	movq	%rdx, %r8
1422f8883: 49 c1 e8 06                 	shrq	$0x6, %r8
1422f8887: 4a 8b 0c c1                 	movq	(%rcx,%r8,8), %rcx
1422f888b: 48 0f a3 d1                 	btq	%rdx, %rcx
1422f888f: 0f 92 c1                    	setb	%cl
