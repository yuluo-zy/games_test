
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fba410:     	pushq	%r15
140fba412:     	pushq	%r14
140fba414:     	pushq	%r13
140fba416:     	pushq	%r12
140fba418:     	pushq	%rsi
140fba419:     	pushq	%rdi
140fba41a:     	pushq	%rbp
140fba41b:     	pushq	%rbx
140fba41c:     	subq	$0xe8, %rsp
140fba423:     	movq	%r9, %rsi
140fba426:     	movq	%rcx, %rdi
140fba429:     	movq	%rdx, 0xc0(%rsp)
140fba431:     	movq	%r8, 0xc8(%rsp)
140fba439:     	movq	0x18(%rcx), %r13
140fba43d:     	leaq	0xc0(%rsp), %rbx
140fba445:     	movq	%rbx, %rcx
140fba448:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fba44d:     	movq	%rax, 0x40(%rsp)
140fba452:     	movabsq	$0x41223169ff28813b, %r15 # imm = 0x41223169FF28813B
140fba45c:     	movq	%r15, 0x48(%rsp)
140fba461:     	movabsq	$-0x58648d975d569727, %r12 # imm = 0xA79B7268A2A968D9
140fba46b:     	movq	%r12, 0x50(%rsp)
140fba470:     	leaq	0x40(%rsp), %rcx
140fba475:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fba47a:     	movq	%r13, 0x40(%rsp)
140fba47f:     	movq	%rax, 0x48(%rsp)
140fba484:     	leaq	0x40(%rsp), %rcx
140fba489:     	callq	0x1405bd910 <.text+0x5bc910>
140fba48e:     	movq	%rax, 0x40(%rsp)
140fba493:     	movq	0x10(%rdi), %rbp
140fba497:     	cmpq	$0x0, 0x1b8(%rbp)
140fba49f:     	je	0x140fba55e <.text+0xfb955e>
140fba4a5:     	movq	%rax, %r14
140fba4a8:     	leaq	0x1c0(%rbp), %rcx
140fba4af:     	leaq	0x40(%rsp), %rdx
140fba4b4:     	callq	0x1405b0d70 <.text+0x5afd70>
140fba4b9:     	movq	0x1a0(%rbp), %rcx
140fba4c0:     	movq	0x1a8(%rbp), %rdx
140fba4c7:     	movq	%rdx, %r8
140fba4ca:     	andq	%rax, %r8
140fba4cd:     	shrq	$0x39, %rax
140fba4d1:     	movd	%eax, %xmm0
140fba4d5:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fba4d9:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fba4de:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fba4e3:     	leaq	-0xc0(%rcx), %rax
140fba4ea:     	xorl	%r9d, %r9d
140fba4ed:     	pcmpeqd	%xmm1, %xmm1
140fba4f1:     	movdqu	(%rcx,%r8), %xmm2
140fba4f7:     	movdqa	%xmm2, %xmm3
140fba4fb:     	pcmpeqb	%xmm0, %xmm3
140fba4ff:     	pmovmskb	%xmm3, %r10d
140fba504:     	testl	%r10d, %r10d
140fba507:     	je	0x140fba540 <.text+0xfb9540>
140fba509:     	tzcntl	%r10d, %r11d
140fba50e:     	addq	%r8, %r11
140fba511:     	andq	%rdx, %r11
140fba514:     	negq	%r11
140fba517:     	leaq	(%r11,%r11,2), %r11
140fba51b:     	shlq	$0x6, %r11
140fba51f:     	cmpq	%r14, (%rax,%r11)
140fba523:     	je	0x140fba7c2 <.text+0xfb97c2>
140fba529:     	leal	-0x1(%r10), %r11d
140fba52d:     	andw	%r10w, %r11w
140fba531:     	movl	%r11d, %r10d
140fba534:     	jne	0x140fba509 <.text+0xfb9509>
140fba536:     	nopw	%cs:(%rax,%rax)
140fba540:     	pcmpeqb	%xmm1, %xmm2
140fba544:     	pmovmskb	%xmm2, %r10d
140fba549:     	testl	%r10d, %r10d
140fba54c:     	jne	0x140fba55e <.text+0xfb955e>
140fba54e:     	addq	%r9, %r8
140fba551:     	addq	$0x10, %r8
140fba555:     	addq	$0x10, %r9
140fba559:     	andq	%rdx, %r8
140fba55c:     	jmp	0x140fba4f1 <.text+0xfb94f1>
140fba55e:     	leaq	0x10(%rdi), %r14
140fba562:     	movq	0x2671e3f(%rip), %rax   # 0x14362c3a8
140fba569:     	movq	(%rax), %rax
140fba56c:     	testq	%rax, %rax
140fba56f:     	je	0x140fba652 <.text+0xfb9652>
140fba575:     	movq	%rdi, 0x20(%rsp)
140fba57a:     	leaq	-0xa86ce1(%rip), %rax   # 0x1405338a0 <.text+0x5328a0>
140fba581:     	movq	%rax, 0x28(%rsp)
140fba586:     	movq	%rbx, 0x30(%rsp)
140fba58b:     	movq	%rax, 0x38(%rsp)
140fba590:     	leaq	0x1bf98f9(%rip), %rcx   # 0x142bb3e90
140fba597:     	callq	0x14067dd80 <.text+0x67cd80>
140fba59c:     	movdqu	(%rax), %xmm0
140fba5a0:     	movl	0x10(%rax), %eax
140fba5a3:     	movq	$0x1, 0x70(%rsp)
140fba5ac:     	leaq	0x1bf985d(%rip), %rcx   # 0x142bb3e10
140fba5b3:     	movq	%rcx, 0x78(%rsp)
140fba5b8:     	movq	$0x14, 0x80(%rsp)
140fba5c4:     	leaq	0x1bf9875(%rip), %rdx   # 0x142bb3e40
140fba5cb:     	movq	%rdx, 0x90(%rsp)
140fba5d3:     	movq	$0x2, 0x98(%rsp)
140fba5df:     	leaq	0x20(%rsp), %rdx
140fba5e4:     	movq	%rdx, 0xa0(%rsp)
140fba5ec:     	movq	$0x2, 0xa8(%rsp)
140fba5f8:     	movq	$0x0, 0xb0(%rsp)
140fba604:     	movq	$0x0, 0x40(%rsp)
140fba60d:     	movq	%rcx, 0x48(%rsp)
140fba612:     	movq	$0x14, 0x50(%rsp)
140fba61b:     	movq	$0x0, 0x58(%rsp)
140fba624:     	movdqu	%xmm0, 0x60(%rsp)
140fba62a:     	movl	$0x1, 0x88(%rsp)
140fba635:     	movl	%eax, 0x8c(%rsp)
140fba63c:     	leaq	0xd0(%rsp), %rcx
140fba644:     	leaq	0x40(%rsp), %rdx
140fba649:     	callq	0x140d4a330 <.text+0xd49330>
140fba64e:     	movq	0x18(%rdi), %r13
140fba652:     	movups	0xc0(%rsp), %xmm0
140fba65a:     	movups	%xmm0, 0x20(%rsp)
140fba65f:     	leaq	0x20(%rsp), %rcx
140fba664:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fba669:     	movq	%rax, 0x40(%rsp)
140fba66e:     	movq	%r15, 0x48(%rsp)
140fba673:     	movq	%r12, 0x50(%rsp)
140fba678:     	leaq	0x40(%rsp), %rcx
140fba67d:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fba682:     	movq	%r13, 0x40(%rsp)
140fba687:     	movq	%rax, 0x48(%rsp)
140fba68c:     	leaq	0x40(%rsp), %rcx
140fba691:     	callq	0x1405bd910 <.text+0x5bc910>
140fba696:     	leaq	0xd0(%rsp), %rdi
140fba69e:     	movq	%rdi, %rcx
140fba6a1:     	movq	%r14, %rdx
140fba6a4:     	movq	%rax, %r8
140fba6a7:     	callq	0x1406861b0 <.text+0x6851b0>
140fba6ac:     	leaq	0x1bf9c95(%rip), %r8    # 0x142bb4348
140fba6b3:     	leaq	0x40(%rsp), %rbx
140fba6b8:     	movl	$0x9, %r9d
140fba6be:     	movq	%rbx, %rcx
140fba6c1:     	movq	%rdi, %rdx
140fba6c4:     	callq	0x1406811c0 <.text+0x6801c0>
140fba6c9:     	movq	%rbx, %rcx
140fba6cc:     	movb	$0x1, %dl
140fba6ce:     	callq	0x140f9de80 <.text+0xf9ce80>
140fba6d3:     	movb	$0x12, 0x44(%rax)
140fba6d7:     	movdqu	0x40(%rsp), %xmm0
140fba6dd:     	movdqa	%xmm0, 0x20(%rsp)
140fba6e3:     	movq	0x50(%rsp), %rax
140fba6e8:     	movq	%rax, 0x30(%rsp)
140fba6ed:     	movq	0x20(%rsp), %rdi
140fba6f2:     	cmpq	$0x0, 0x1b8(%rdi)
140fba6fa:     	je	0x140fba91d <.text+0xfb991d>
140fba700:     	leaq	0x28(%rsp), %rdx
140fba705:     	leaq	0x1c0(%rdi), %rcx
140fba70c:     	callq	0x1405b0d70 <.text+0x5afd70>
140fba711:     	movq	0x1a0(%rdi), %rcx
140fba718:     	movq	0x1a8(%rdi), %rdx
140fba71f:     	movq	%rdx, %r8
140fba722:     	andq	%rax, %r8
140fba725:     	shrq	$0x39, %rax
140fba729:     	movd	%eax, %xmm0
140fba72d:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fba731:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fba736:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fba73b:     	leaq	-0xc0(%rcx), %rax
140fba742:     	xorl	%r9d, %r9d
140fba745:     	movq	0x28(%rsp), %r10
140fba74a:     	pcmpeqd	%xmm1, %xmm1
140fba74e:     	movdqu	(%rcx,%r8), %xmm2
140fba754:     	movdqa	%xmm2, %xmm3
140fba758:     	pcmpeqb	%xmm0, %xmm3
140fba75c:     	pmovmskb	%xmm3, %r11d
140fba761:     	testl	%r11d, %r11d
140fba764:     	je	0x140fba7a0 <.text+0xfb97a0>
140fba766:     	tzcntl	%r11d, %edi
140fba76b:     	addq	%r8, %rdi
140fba76e:     	andq	%rdx, %rdi
140fba771:     	negq	%rdi
140fba774:     	leaq	(%rdi,%rdi,2), %rdi
140fba778:     	shlq	$0x6, %rdi
140fba77c:     	cmpq	(%rax,%rdi), %r10
140fba780:     	je	0x140fba8e2 <.text+0xfb98e2>
140fba786:     	leal	-0x1(%r11), %edi
140fba78a:     	andw	%r11w, %di
140fba78e:     	movl	%edi, %r11d
140fba791:     	jne	0x140fba766 <.text+0xfb9766>
140fba793:     	nopw	%cs:(%rax,%rax)
140fba7a0:     	pcmpeqb	%xmm1, %xmm2
140fba7a4:     	pmovmskb	%xmm2, %r11d
140fba7a9:     	testl	%r11d, %r11d
140fba7ac:     	jne	0x140fba91d <.text+0xfb991d>
140fba7b2:     	addq	%r9, %r8
140fba7b5:     	addq	$0x10, %r8
140fba7b9:     	addq	$0x10, %r9
140fba7bd:     	andq	%rdx, %r8
140fba7c0:     	jmp	0x140fba74e <.text+0xfb974e>
140fba7c2:     	movq	%rbp, 0xd0(%rsp)
140fba7ca:     	movq	%r14, 0xd8(%rsp)
140fba7d2:     	movq	$0x0, 0xe0(%rsp)
140fba7de:     	leaq	0x1bf9b63(%rip), %r8    # 0x142bb4348
140fba7e5:     	leaq	0x40(%rsp), %rdi
140fba7ea:     	leaq	0xd0(%rsp), %rdx
140fba7f2:     	movl	$0x9, %r9d
140fba7f8:     	movq	%rdi, %rcx
140fba7fb:     	callq	0x1406811c0 <.text+0x6801c0>
140fba800:     	movq	%rdi, %rcx
140fba803:     	movb	$0x1, %dl
140fba805:     	callq	0x140f9de80 <.text+0xf9ce80>
140fba80a:     	movb	$0x12, 0x44(%rax)
140fba80e:     	movups	0x40(%rsp), %xmm0
140fba813:     	movaps	%xmm0, 0x20(%rsp)
140fba818:     	movq	0x50(%rsp), %rax
140fba81d:     	movq	%rax, 0x30(%rsp)
140fba822:     	movq	0x20(%rsp), %rdi
140fba827:     	cmpq	$0x0, 0x1b8(%rdi)
140fba82f:     	je	0x140fba91d <.text+0xfb991d>
140fba835:     	leaq	0x28(%rsp), %rdx
140fba83a:     	leaq	0x1c0(%rdi), %rcx
140fba841:     	callq	0x1405b0d70 <.text+0x5afd70>
140fba846:     	movq	0x1a0(%rdi), %rcx
140fba84d:     	movq	0x1a8(%rdi), %rdx
140fba854:     	movq	%rdx, %r8
140fba857:     	andq	%rax, %r8
140fba85a:     	shrq	$0x39, %rax
140fba85e:     	movd	%eax, %xmm0
140fba862:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fba866:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fba86b:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fba870:     	leaq	-0xc0(%rcx), %rax
140fba877:     	xorl	%r9d, %r9d
140fba87a:     	movq	0x28(%rsp), %r10
140fba87f:     	pcmpeqd	%xmm1, %xmm1
140fba883:     	movdqu	(%rcx,%r8), %xmm2
140fba889:     	movdqa	%xmm2, %xmm3
140fba88d:     	pcmpeqb	%xmm0, %xmm3
140fba891:     	pmovmskb	%xmm3, %r11d
140fba896:     	testl	%r11d, %r11d
140fba899:     	je	0x140fba8c4 <.text+0xfb98c4>
140fba89b:     	tzcntl	%r11d, %edi
140fba8a0:     	addq	%r8, %rdi
140fba8a3:     	andq	%rdx, %rdi
140fba8a6:     	negq	%rdi
140fba8a9:     	leaq	(%rdi,%rdi,2), %rdi
140fba8ad:     	shlq	$0x6, %rdi
140fba8b1:     	cmpq	(%rax,%rdi), %r10
140fba8b5:     	je	0x140fba8e2 <.text+0xfb98e2>
140fba8b7:     	leal	-0x1(%r11), %edi
140fba8bb:     	andw	%r11w, %di
140fba8bf:     	movl	%edi, %r11d
140fba8c2:     	jne	0x140fba89b <.text+0xfb989b>
140fba8c4:     	pcmpeqb	%xmm1, %xmm2
140fba8c8:     	pmovmskb	%xmm2, %r11d
140fba8cd:     	testl	%r11d, %r11d
140fba8d0:     	jne	0x140fba91d <.text+0xfb991d>
140fba8d2:     	addq	%r9, %r8
140fba8d5:     	addq	$0x10, %r8
140fba8d9:     	addq	$0x10, %r9
140fba8dd:     	andq	%rdx, %r8
140fba8e0:     	jmp	0x140fba883 <.text+0xfb9883>
140fba8e2:     	testb	$0x30, -0x2(%rcx,%rdi)
140fba8e7:     	je	0x140fba908 <.text+0xfb9908>
140fba8e9:     	movl	0x1c(%rsi), %eax
140fba8ec:     	movq	0x10(%rsi), %rcx
140fba8f0:     	movl	%eax, (%rcx)
140fba8f2:     	movq	(%rsi), %rcx
140fba8f5:     	movq	$0x2, 0x40(%rsp)
140fba8fe:     	leaq	0x40(%rsp), %rdx
140fba903:     	callq	0x14093fad0 <.text+0x93ead0>
140fba908:     	nop
140fba909:     	addq	$0xe8, %rsp
140fba910:     	popq	%rbx
140fba911:     	popq	%rbp
140fba912:     	popq	%rdi
140fba913:     	popq	%rsi
140fba914:     	popq	%r12
140fba916:     	popq	%r13
140fba918:     	popq	%r14
140fba91a:     	popq	%r15
140fba91c:     	retq
140fba91d:     	leaq	0x1a5c8e4(%rip), %rcx   # 0x142a17208
140fba924:     	leaq	0x1a5d6ad(%rip), %r8    # 0x142a17fd8
140fba92b:     	movl	$0x16, %edx
140fba930:     	callq	0x1428d9330 <.text+0x28d8330>
140fba935:     	int3
140fba936:     	int3
140fba937:     	int3
140fba938:     	int3
140fba939:     	int3
140fba93a:     	int3
140fba93b:     	int3
140fba93c:     	int3
140fba93d:     	int3
140fba93e:     	int3
140fba93f:     	int3
140fba940:     	pushq	%r15
140fba942:     	pushq	%r14
140fba944:     	pushq	%r13
140fba946:     	pushq	%r12
140fba948:     	pushq	%rsi
140fba949:     	pushq	%rdi
140fba94a:     	pushq	%rbp
140fba94b:     	pushq	%rbx
140fba94c:     	subq	$0xd8, %rsp
140fba953:     	movq	%r9, %rsi
140fba956:     	movq	%rcx, %rdi
140fba959:     	movq	%rdx, 0xa8(%rsp)
140fba961:     	movq	%r8, 0xb0(%rsp)
140fba969:     	movq	0x18(%rcx), %r13
140fba96d:     	leaq	0xa8(%rsp), %rbx
140fba975:     	movq	%rbx, %rcx
140fba978:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fba97d:     	movq	%rax, 0x28(%rsp)
140fba982:     	movabsq	$0x41223169ff28813b, %r15 # imm = 0x41223169FF28813B
140fba98c:     	movq	%r15, 0x30(%rsp)
140fba991:     	movabsq	$-0x58648d975d569727, %r12 # imm = 0xA79B7268A2A968D9
140fba99b:     	movq	%r12, 0x38(%rsp)
