
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b7e440:     	pushq	%rbp
140b7e441:     	pushq	%r15
140b7e443:     	pushq	%r14
140b7e445:     	pushq	%r13
140b7e447:     	pushq	%r12
140b7e449:     	pushq	%rsi
140b7e44a:     	pushq	%rdi
140b7e44b:     	pushq	%rbx
140b7e44c:     	subq	$0x2f8, %rsp            # imm = 0x2F8
140b7e453:     	leaq	0x80(%rsp), %rbp
140b7e45b:     	movdqa	%xmm6, 0x260(%rbp)
140b7e463:     	movq	$-0x2, 0x258(%rbp)
140b7e46e:     	movq	%rcx, %rsi
140b7e471:     	movq	$0x0, 0x230(%rbp)
140b7e47c:     	movq	$0x8, 0x238(%rbp)
140b7e487:     	movq	$0x0, 0x240(%rbp)
140b7e492:     	movq	(%rcx), %rdi
140b7e495:     	cmpq	$0x0, 0x30(%rdi)
140b7e49a:     	movabsq	$0x7e9bc519d53d8528, %r12 # imm = 0x7E9BC519D53D8528
140b7e4a4:     	movabsq	$0x74091cff3827aa9e, %r15 # imm = 0x74091CFF3827AA9E
140b7e4ae:     	je	0x140b7e54a <.text+0xb7d54a>
140b7e4b4:     	movq	0x18(%rdi), %rax
140b7e4b8:     	movq	0x20(%rdi), %rcx
140b7e4bc:     	movq	%rcx, %rdx
140b7e4bf:     	andq	%r12, %rdx
140b7e4c2:     	xorl	%r8d, %r8d
140b7e4c5:     	movdqa	0x1de3193(%rip), %xmm0  # 0x142961660
140b7e4cd:     	pcmpeqd	%xmm1, %xmm1
140b7e4d1:     	movdqu	(%rax,%rdx), %xmm2
140b7e4d6:     	movdqa	%xmm2, %xmm3
140b7e4da:     	pcmpeqb	%xmm0, %xmm3
140b7e4de:     	pmovmskb	%xmm3, %r9d
140b7e4e3:     	testl	%r9d, %r9d
140b7e4e6:     	jne	0x140b7e51d <.text+0xb7d51d>
140b7e4e8:     	pcmpeqb	%xmm1, %xmm2
140b7e4ec:     	pmovmskb	%xmm2, %r9d
140b7e4f1:     	testl	%r9d, %r9d
140b7e4f4:     	jne	0x140b7e54a <.text+0xb7d54a>
140b7e4f6:     	addq	%r8, %rdx
140b7e4f9:     	addq	$0x10, %rdx
140b7e4fd:     	addq	$0x10, %r8
140b7e501:     	andq	%rcx, %rdx
140b7e504:     	jmp	0x140b7e4d1 <.text+0xb7d4d1>
140b7e506:     	nopw	%cs:(%rax,%rax)
140b7e510:     	leal	-0x1(%r9), %r10d
140b7e514:     	andw	%r9w, %r10w
140b7e518:     	movl	%r10d, %r9d
140b7e51b:     	je	0x140b7e4e8 <.text+0xb7d4e8>
140b7e51d:     	tzcntl	%r9d, %r10d
140b7e522:     	addq	%rdx, %r10
140b7e525:     	andq	%rcx, %r10
140b7e528:     	negq	%r10
140b7e52b:     	leaq	(%r10,%r10,2), %r10
140b7e52f:     	cmpq	%r15, -0x18(%rax,%r10,8)
140b7e534:     	jne	0x140b7e510 <.text+0xb7d510>
140b7e536:     	cmpq	%r12, -0x10(%rax,%r10,8)
140b7e53b:     	jne	0x140b7e510 <.text+0xb7d510>
140b7e53d:     	leaq	(%rax,%r10,8), %rax
140b7e541:     	movq	-0x8(%rax), %rbx
140b7e545:     	jmp	0x140b7e9b3 <.text+0xb7d9b3>
140b7e54a:     	movzbl	0x60(%rdi), %eax
140b7e54e:     	movq	0x80(%rdi), %rax
140b7e555:     	movq	0x88(%rdi), %rdx
140b7e55c:     	movq	%rdx, %r8
140b7e55f:     	andq	%r12, %r8
140b7e562:     	xorl	%r9d, %r9d
140b7e565:     	movdqa	0x1de30f3(%rip), %xmm6  # 0x142961660
140b7e56d:     	pcmpeqd	%xmm0, %xmm0
140b7e571:     	movdqu	(%rax,%r8), %xmm1
140b7e577:     	movdqa	%xmm1, %xmm2
140b7e57b:     	pcmpeqb	%xmm6, %xmm2
140b7e57f:     	pmovmskb	%xmm2, %r10d
140b7e584:     	testl	%r10d, %r10d
140b7e587:     	jne	0x140b7e5bd <.text+0xb7d5bd>
140b7e589:     	pcmpeqb	%xmm0, %xmm1
140b7e58d:     	pmovmskb	%xmm1, %ecx
140b7e591:     	testl	%ecx, %ecx
140b7e593:     	jne	0x140b7e688 <.text+0xb7d688>
140b7e599:     	addq	%r9, %r8
140b7e59c:     	addq	$0x10, %r8
140b7e5a0:     	addq	$0x10, %r9
140b7e5a4:     	andq	%rdx, %r8
140b7e5a7:     	jmp	0x140b7e571 <.text+0xb7d571>
140b7e5a9:     	nopl	(%rax)
140b7e5b0:     	leal	-0x1(%r10), %ecx
140b7e5b4:     	andw	%r10w, %cx
140b7e5b8:     	movl	%ecx, %r10d
140b7e5bb:     	je	0x140b7e589 <.text+0xb7d589>
140b7e5bd:     	tzcntl	%r10d, %r11d
140b7e5c2:     	addq	%r8, %r11
140b7e5c5:     	andq	%rdx, %r11
140b7e5c8:     	movq	%r11, %rcx
140b7e5cb:     	negq	%rcx
140b7e5ce:     	leaq	(,%r11,8), %rbx
140b7e5d6:     	shlq	$0x7, %rcx
140b7e5da:     	subq	%rbx, %rcx
140b7e5dd:     	cmpq	%r15, -0x88(%rax,%rcx)
140b7e5e5:     	jne	0x140b7e5b0 <.text+0xb7d5b0>
140b7e5e7:     	cmpq	%r12, -0x80(%rax,%rcx)
140b7e5ec:     	jne	0x140b7e5b0 <.text+0xb7d5b0>
140b7e5ee:     	movq	%r11, %r8
140b7e5f1:     	shlq	$0x7, %r8
140b7e5f5:     	leaq	(%r8,%r11,8), %r9
140b7e5f9:     	sarq	$0x3, %r9
140b7e5fd:     	movabsq	$-0xf0f0f0f0f0f0f0f, %r8 # imm = 0xF0F0F0F0F0F0F0F1
140b7e607:     	imulq	%r9, %r8
140b7e60b:     	leaq	-0x10(%r8), %r9
140b7e60f:     	andq	%rdx, %r9
140b7e612:     	movdqu	(%rax,%r9), %xmm0
140b7e618:     	pcmpeqd	%xmm1, %xmm1
140b7e61c:     	pcmpeqb	%xmm1, %xmm0
140b7e620:     	pmovmskb	%xmm0, %r10d
140b7e625:     	movdqu	(%rax,%r8), %xmm0
140b7e62b:     	pcmpeqb	%xmm1, %xmm0
140b7e62f:     	pmovmskb	%xmm0, %edx
140b7e633:     	testl	%r10d, %r10d
140b7e636:     	je	0x140b7e643 <.text+0xb7d643>
140b7e638:     	bsrw	%r10w, %r10w
140b7e63d:     	xorl	$0xf, %r10d
140b7e641:     	jmp	0x140b7e648 <.text+0xb7d648>
140b7e643:     	movw	$0x10, %r10w
140b7e648:     	addq	%rax, %rcx
140b7e64b:     	orl	$0x10000, %edx          # imm = 0x10000
140b7e651:     	tzcntl	%edx, %edx
140b7e655:     	addl	%edx, %r10d
140b7e658:     	movb	$-0x80, %dl
140b7e65a:     	cmpw	$0xf, %r10w
140b7e65f:     	ja	0x140b7e66a <.text+0xb7d66a>
140b7e661:     	incq	0x90(%rdi)
140b7e668:     	movb	$-0x1, %dl
140b7e66a:     	movb	%dl, (%rax,%r8)
140b7e66e:     	movb	%dl, 0x10(%rax,%r9)
140b7e673:     	decq	0x98(%rdi)
140b7e67a:     	movq	-0x78(%rcx), %rax
140b7e67e:     	cmpq	$0x2, %rax
140b7e682:     	jne	0x140b7e945 <.text+0xb7d945>
140b7e688:     	movq	0x8(%rsi), %rax
140b7e68c:     	movq	(%rax), %rbx
140b7e68f:     	leaq	0x1(%rbx), %r14
140b7e693:     	movq	%r14, (%rax)
140b7e696:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140b7e6a0:     	movq	%rax, 0x208(%rbp)
140b7e6a7:     	leaq	0x20b83c0(%rip), %rax   # 0x142c36a6e
140b7e6ae:     	movq	%rax, 0x210(%rbp)
140b7e6b5:     	movq	$0x31, 0x218(%rbp)
140b7e6c0:     	movw	$0x100, 0x228(%rbp)     # imm = 0x100
140b7e6c9:     	movq	$0x1, 0x1d0(%rbp)
140b7e6d4:     	movq	%r15, 0x1d8(%rbp)
140b7e6db:     	movq	%r12, 0x1e0(%rbp)
140b7e6e2:     	movq	$0x4, 0x1f8(%rbp)
140b7e6ed:     	movq	$0x30, 0x200(%rbp)
140b7e6f8:     	movq	$0x0, 0x220(%rbp)
140b7e703:     	movb	$0x1, 0x22a(%rbp)
140b7e70a:     	movq	$0x0, 0x1e8(%rbp)
140b7e715:     	leaq	0x80(%rbp), %rcx
140b7e71c:     	leaq	0x1d0(%rbp), %r8
140b7e723:     	movq	%rbx, %rdx
140b7e726:     	callq	0x140d3f870 <.text+0xd3e870>
140b7e72b:     	cmpq	%r14, 0x10(%rdi)
140b7e72f:     	jae	0x140b7e74a <.text+0xb7d74a>
140b7e731:     	movb	$0x1, 0x257(%rbp)
140b7e738:     	leaq	0x1f77161(%rip), %r8    # 0x142af58a0
140b7e73f:     	movq	%rdi, %rcx
