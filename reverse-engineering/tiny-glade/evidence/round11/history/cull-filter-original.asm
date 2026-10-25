
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415635a0: 41 57                       	pushq	%r15
1415635a2: 41 56                       	pushq	%r14
1415635a4: 41 55                       	pushq	%r13
1415635a6: 41 54                       	pushq	%r12
1415635a8: 56                          	pushq	%rsi
1415635a9: 57                          	pushq	%rdi
1415635aa: 55                          	pushq	%rbp
1415635ab: 53                          	pushq	%rbx
1415635ac: 48 83 ec 58                 	subq	$0x58, %rsp
1415635b0: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
1415635b6: 4c 8b 71 20                 	movq	0x20(%rcx), %r14
1415635ba: 4d 85 f6                    	testq	%r14, %r14
1415635bd: 0f 84 92 01 00 00           	je	0x141563755 <.text+0x1562755>
1415635c3: 48 89 ce                    	movq	%rcx, %rsi
1415635c6: 4c 8b 39                    	movq	(%rcx), %r15
1415635c9: 4c 8b 61 08                 	movq	0x8(%rcx), %r12
1415635cd: 4c 8b 2a                    	movq	(%rdx), %r13
1415635d0: 49 8d 7d 20                 	leaq	0x20(%r13), %rdi
1415635d4: 0f b7 41 18                 	movzwl	0x18(%rcx), %eax
1415635d8: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1415635dc: e9 b0 00 00 00              	jmp	0x141563691 <.text+0x1562691>
1415635e1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1415635f0: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1415635f6: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1415635fa: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1415635fe: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
141563603: 45 85 db                    	testl	%r11d, %r11d
141563606: 74 48                       	je	0x141563650 <.text+0x1562650>
141563608: 48 89 f2                    	movq	%rsi, %rdx
14156360b: 49 89 fa                    	movq	%rdi, %r10
14156360e: f3 41 0f bc fb              	tzcntl	%r11d, %edi
141563613: 4c 01 c7                    	addq	%r8, %rdi
141563616: 48 23 7c 24 30              	andq	0x30(%rsp), %rdi
14156361b: 48 f7 df                    	negq	%rdi
14156361e: 48 6b ff 58                 	imulq	$0x58, %rdi, %rdi
141563622: 44 89 ce                    	movl	%r9d, %esi
141563625: 4c 8b 4c 24 38              	movq	0x38(%rsp), %r9
14156362a: 41 3b 34 39                 	cmpl	(%r9,%rdi), %esi
14156362e: 74 50                       	je	0x141563680 <.text+0x1562680>
141563630: 41 89 f1                    	movl	%esi, %r9d
141563633: 41 8d 7b ff                 	leal	-0x1(%r11), %edi
141563637: 66 44 21 df                 	andw	%r11w, %di
14156363b: 41 89 fb                    	movl	%edi, %r11d
14156363e: 4c 89 d7                    	movq	%r10, %rdi
141563641: 48 89 d6                    	movq	%rdx, %rsi
141563644: 75 c2                       	jne	0x141563608 <.text+0x1562608>
141563646: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141563650: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
141563654: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
141563659: 45 85 db                    	testl	%r11d, %r11d
14156365c: 0f 85 ec 00 00 00           	jne	0x14156374e <.text+0x156274e>
141563662: 49 01 c0                    	addq	%rax, %r8
141563665: 49 83 c0 10                 	addq	$0x10, %r8
141563669: 48 83 c0 10                 	addq	$0x10, %rax
14156366d: 4c 23 44 24 30              	andq	0x30(%rsp), %r8
141563672: e9 79 ff ff ff              	jmp	0x1415635f0 <.text+0x15625f0>
141563677: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141563680: 89 d8                       	movl	%ebx, %eax
141563682: 4d 85 f6                    	testq	%r14, %r14
141563685: 4c 89 d7                    	movq	%r10, %rdi
141563688: 48 89 d6                    	movq	%rdx, %rsi
14156368b: 0f 84 c4 00 00 00           	je	0x141563755 <.text+0x1562755>
141563691: 66 85 c0                    	testw	%ax, %ax
141563694: 74 1a                       	je	0x1415636b0 <.text+0x15626b0>
141563696: 8d 58 ff                    	leal	-0x1(%rax), %ebx
141563699: 21 c3                       	andl	%eax, %ebx
14156369b: 66 89 5e 18                 	movw	%bx, 0x18(%rsi)
14156369f: 49 ff ce                    	decq	%r14
1415636a2: 4c 89 76 20                 	movq	%r14, 0x20(%rsi)
1415636a6: 4d 85 ff                    	testq	%r15, %r15
1415636a9: 75 40                       	jne	0x1415636eb <.text+0x15626eb>
1415636ab: e9 a5 00 00 00              	jmp	0x141563755 <.text+0x1562755>
1415636b0: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
1415636b6: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1415636ba: 49 81 c7 80 fa ff ff        	addq	$-0x580, %r15           # imm = 0xFA80
1415636c1: 49 83 c4 10                 	addq	$0x10, %r12
1415636c5: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1415636ca: 74 e4                       	je	0x1415636b0 <.text+0x15626b0>
1415636cc: 4c 89 66 08                 	movq	%r12, 0x8(%rsi)
1415636d0: 4c 89 3e                    	movq	%r15, (%rsi)
1415636d3: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1415636d8: 29 c1                       	subl	%eax, %ecx
1415636da: f7 d0                       	notl	%eax
1415636dc: 89 c3                       	movl	%eax, %ebx
1415636de: 21 cb                       	andl	%ecx, %ebx
1415636e0: 66 89 5e 18                 	movw	%bx, 0x18(%rsi)
1415636e4: 49 ff ce                    	decq	%r14
1415636e7: 4c 89 76 20                 	movq	%r14, 0x20(%rsi)
1415636eb: f3 0f bc c0                 	tzcntl	%eax, %eax
1415636ef: 48 f7 d8                    	negq	%rax
1415636f2: 48 6b c0 58                 	imulq	$0x58, %rax, %rax
1415636f6: 41 8b 6c 07 a8              	movl	-0x58(%r15,%rax), %ebp
1415636fb: 89 6c 24 2c                 	movl	%ebp, 0x2c(%rsp)
1415636ff: 49 83 7d 18 00              	cmpq	$0x0, 0x18(%r13)
141563704: 74 48                       	je	0x14156374e <.text+0x156274e>
141563706: 48 89 f9                    	movq	%rdi, %rcx
141563709: 48 8d 54 24 2c              	leaq	0x2c(%rsp), %rdx
14156370e: e8 bd d7 04 ff              	callq	0x1405b0ed0 <.text+0x5afed0>
141563713: 49 8b 4d 00                 	movq	(%r13), %rcx
141563717: 4d 8b 45 08                 	movq	0x8(%r13), %r8
14156371b: 4c 89 44 24 30              	movq	%r8, 0x30(%rsp)
141563720: 49 21 c0                    	andq	%rax, %r8
141563723: 48 c1 e8 39                 	shrq	$0x39, %rax
141563727: 66 0f 6e c0                 	movd	%eax, %xmm0
14156372b: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14156372f: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141563734: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141563739: 48 8d 41 a8                 	leaq	-0x58(%rcx), %rax
14156373d: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
141563742: 44 8b 4c 24 2c              	movl	0x2c(%rsp), %r9d
141563747: 31 c0                       	xorl	%eax, %eax
141563749: e9 a2 fe ff ff              	jmp	0x1415635f0 <.text+0x15625f0>
14156374e: b8 01 00 00 00              	movl	$0x1, %eax
141563753: eb 02                       	jmp	0x141563757 <.text+0x1562757>
141563755: 31 c0                       	xorl	%eax, %eax
141563757: 89 ea                       	movl	%ebp, %edx
141563759: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
14156375e: 48 83 c4 58                 	addq	$0x58, %rsp
141563762: 5b                          	popq	%rbx
141563763: 5d                          	popq	%rbp
141563764: 5f                          	popq	%rdi
141563765: 5e                          	popq	%rsi
141563766: 41 5c                       	popq	%r12
141563768: 41 5d                       	popq	%r13
14156376a: 41 5e                       	popq	%r14
14156376c: 41 5f                       	popq	%r15
14156376e: c3                          	retq
