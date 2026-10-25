
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141475530:     	pushq	%r15
141475532:     	pushq	%r14
141475534:     	pushq	%r13
141475536:     	pushq	%r12
141475538:     	pushq	%rsi
141475539:     	pushq	%rdi
14147553a:     	pushq	%rbp
14147553b:     	pushq	%rbx
14147553c:     	subq	$0x38, %rsp
141475540:     	movq	%rdx, %rsi
141475543:     	movq	0x8(%rcx), %rdi
141475547:     	movq	0x10(%rcx), %r12
14147554b:     	movq	0x18(%rcx), %rax
14147554f:     	movq	%rax, 0x28(%rsp)
141475554:     	movq	0x20(%rcx), %r14
141475558:     	movq	0x28(%rcx), %r15
14147555c:     	movq	0x58(%rcx), %r13
141475560:     	movq	0x60(%rcx), %rbp
141475564:     	movzbl	0x68(%rcx), %ebx
141475568:     	cmpq	%r13, %rbp
14147556b:     	jne	0x141475772 <.text+0x1474772>
141475571:     	cmpq	%r15, %r14
141475574:     	je	0x141475761 <.text+0x1474761>
14147557a:     	testb	$0x1, %bl
14147557d:     	je	0x14147563d <.text+0x147463d>
141475583:     	jmp	0x14147559d <.text+0x147459d>
141475585:     	nopw	%cs:(%rax,%rax)
141475590:     	addq	$0x4, %r14
141475594:     	cmpq	%r15, %r14
141475597:     	je	0x141475761 <.text+0x1474761>
14147559d:     	movl	(%r14), %ecx
1414755a0:     	movq	0x8(%rdi), %rax
1414755a4:     	leaq	(%rcx,%rcx,8), %rcx
1414755a8:     	movq	0x10(%rax,%rcx,8), %r12
1414755ad:     	testq	%r12, %r12
1414755b0:     	je	0x141475590 <.text+0x1474590>
1414755b2:     	leaq	(%rax,%rcx,8), %rax
1414755b6:     	movq	0x28(%rsp), %rcx
1414755bb:     	movq	0x110(%rcx), %rcx
1414755c2:     	cmpq	%rcx, 0x40(%rax)
1414755c6:     	jbe	0x1414755f0 <.text+0x14745f0>
1414755c8:     	movq	0x38(%rax), %rdx
1414755cc:     	movq	(%rdx,%rcx,8), %rcx
1414755d0:     	testq	%rcx, %rcx
1414755d3:     	je	0x1414755f0 <.text+0x14745f0>
1414755d5:     	movq	0x18(%rax), %rax
1414755d9:     	notq	%rcx
1414755dc:     	leaq	(%rcx,%rcx,2), %rcx
1414755e0:     	shlq	$0x4, %rcx
1414755e4:     	movq	0x10(%rax,%rcx), %r13
1414755e9:     	jmp	0x1414755f3 <.text+0x14745f3>
1414755eb:     	nopl	(%rax,%rax)
1414755f0:     	xorl	%r13d, %r13d
1414755f3:     	xorl	%eax, %eax
1414755f5:     	nopw	%cs:(%rax,%rax)
141475600:     	leaq	0x1(%rax), %rbx
141475604:     	movl	%eax, %eax
141475606:     	shlq	$0x4, %rax
14147560a:     	movq	(%r13,%rax), %rdx
14147560f:     	movq	%rsi, %rcx
141475612:     	callq	0x1402ca2f0 <.text+0x2c92f0>
141475617:     	movq	%rbx, %rax
14147561a:     	cmpq	%rbx, %r12
14147561d:     	jne	0x141475600 <.text+0x1474600>
14147561f:     	jmp	0x141475590 <.text+0x1474590>
141475624:     	nopw	%cs:(%rax,%rax)
141475630:     	addq	$0x4, %r14
141475634:     	cmpq	%r15, %r14
141475637:     	je	0x141475761 <.text+0x1474761>
14147563d:     	movl	(%r14), %ecx
141475640:     	movq	0x8(%r12), %rax
141475645:     	leaq	(%rcx,%rcx,4), %rcx
141475649:     	shlq	$0x5, %rcx
14147564d:     	movl	0x94(%rax,%rcx), %edx
141475654:     	movq	0x8(%rdi), %r8
141475658:     	leaq	(%rdx,%rdx,8), %r9
14147565c:     	leaq	(%r8,%r9,8), %rdx
141475660:     	movq	0x58(%rax,%rcx), %r13
141475665:     	cmpq	%r13, 0x10(%r8,%r9,8)
14147566a:     	jne	0x1414756b0 <.text+0x14746b0>
14147566c:     	testq	%r13, %r13
14147566f:     	je	0x141475630 <.text+0x1474630>
141475671:     	movq	0x28(%rsp), %rax
141475676:     	movq	0x110(%rax), %rax
14147567d:     	cmpq	%rax, 0x40(%rdx)
141475681:     	jbe	0x1414756f0 <.text+0x14746f0>
141475683:     	movq	0x38(%rdx), %rcx
141475687:     	movq	(%rcx,%rax,8), %rax
14147568b:     	testq	%rax, %rax
14147568e:     	je	0x1414756f0 <.text+0x14746f0>
141475690:     	movq	0x18(%rdx), %rcx
141475694:     	notq	%rax
141475697:     	leaq	(%rax,%rax,2), %rax
14147569b:     	shlq	$0x4, %rax
14147569f:     	movq	0x10(%rcx,%rax), %rbp
1414756a4:     	jmp	0x1414756f2 <.text+0x14746f2>
1414756a6:     	nopw	%cs:(%rax,%rax)
1414756b0:     	testq	%r13, %r13
1414756b3:     	je	0x141475630 <.text+0x1474630>
1414756b9:     	movq	0x28(%rsp), %r8
1414756be:     	movq	0x110(%r8), %r8
1414756c5:     	cmpq	%r8, 0x40(%rdx)
1414756c9:     	jbe	0x141475730 <.text+0x1474730>
1414756cb:     	movq	0x38(%rdx), %r9
1414756cf:     	movq	(%r9,%r8,8), %r8
1414756d3:     	testq	%r8, %r8
1414756d6:     	je	0x141475730 <.text+0x1474730>
1414756d8:     	movq	0x18(%rdx), %rdx
1414756dc:     	notq	%r8
1414756df:     	leaq	(%r8,%r8,2), %r8
1414756e3:     	shlq	$0x4, %r8
1414756e7:     	movq	0x10(%rdx,%r8), %rbp
1414756ec:     	jmp	0x141475732 <.text+0x1474732>
1414756ee:     	nop
1414756f0:     	xorl	%ebp, %ebp
1414756f2:     	xorl	%eax, %eax
1414756f4:     	nopw	%cs:(%rax,%rax)
141475700:     	leaq	0x1(%rax), %rbx
141475704:     	movl	%eax, %eax
141475706:     	shlq	$0x4, %rax
14147570a:     	movq	(%rbp,%rax), %rdx
14147570f:     	movq	%rsi, %rcx
141475712:     	callq	0x1402ca2f0 <.text+0x2c92f0>
141475717:     	movq	%rbx, %rax
14147571a:     	cmpq	%rbx, %r13
14147571d:     	jne	0x141475700 <.text+0x1474700>
14147571f:     	jmp	0x141475630 <.text+0x1474630>
141475724:     	nopw	%cs:(%rax,%rax)
141475730:     	xorl	%ebp, %ebp
141475732:     	addq	%rcx, %rax
141475735:     	movq	0x50(%rax), %rbx
141475739:     	addq	$0x8, %rbx
14147573d:     	nopl	(%rax)
141475740:     	movl	(%rbx), %eax
141475742:     	shlq	$0x4, %rax
141475746:     	movq	(%rbp,%rax), %rdx
14147574b:     	movq	%rsi, %rcx
14147574e:     	callq	0x1402ca2f0 <.text+0x2c92f0>
141475753:     	addq	$0x10, %rbx
141475757:     	decq	%r13
14147575a:     	jne	0x141475740 <.text+0x1474740>
14147575c:     	jmp	0x141475630 <.text+0x1474630>
141475761:     	addq	$0x38, %rsp
141475765:     	popq	%rbx
141475766:     	popq	%rbp
141475767:     	popq	%rdi
141475768:     	popq	%rsi
141475769:     	popq	%r12
14147576b:     	popq	%r13
14147576d:     	popq	%r14
14147576f:     	popq	%r15
141475771:     	retq
141475772:     	movq	0x50(%rcx), %rax
141475776:     	movq	%rax, 0x20(%rsp)
14147577b:     	testb	$0x1, %bl
14147577e:     	jne	0x1414757bc <.text+0x14747bc>
141475780:     	movq	0x40(%rcx), %rax
141475784:     	movq	%rax, 0x30(%rsp)
141475789:     	jmp	0x14147587c <.text+0x147487c>
14147578e:     	movq	$0x0, 0x20(%rsp)
141475797:     	xorl	%ebp, %ebp
141475799:     	movl	%ebp, %eax
14147579b:     	shlq	$0x4, %rax
14147579f:     	incq	%rbp
1414757a2:     	movq	0x20(%rsp), %rcx
1414757a7:     	movq	(%rcx,%rax), %rdx
1414757ab:     	movq	%rsi, %rcx
1414757ae:     	callq	0x1402ca2f0 <.text+0x2c92f0>
1414757b3:     	cmpq	%r13, %rbp
1414757b6:     	je	0x141475571 <.text+0x1474571>
1414757bc:     	cmpq	%r13, %rbp
1414757bf:     	jne	0x141475799 <.text+0x1474799>
1414757c1:     	nopw	%cs:(%rax,%rax)
1414757d0:     	cmpq	%r15, %r14
1414757d3:     	je	0x141475571 <.text+0x1474571>
1414757d9:     	movl	(%r14), %ecx
1414757dc:     	addq	$0x4, %r14
1414757e0:     	movq	0x8(%rdi), %rax
1414757e4:     	leaq	(%rcx,%rcx,8), %rcx
1414757e8:     	movq	0x10(%rax,%rcx,8), %r13
1414757ed:     	testq	%r13, %r13
1414757f0:     	je	0x1414757d0 <.text+0x14747d0>
1414757f2:     	leaq	(%rax,%rcx,8), %rax
1414757f6:     	movq	0x28(%rsp), %rcx
1414757fb:     	movq	0x110(%rcx), %rcx
141475802:     	cmpq	%rcx, 0x40(%rax)
141475806:     	jbe	0x14147578e <.text+0x147478e>
141475808:     	movq	0x38(%rax), %rdx
14147580c:     	movq	(%rdx,%rcx,8), %rcx
141475810:     	testq	%rcx, %rcx
141475813:     	je	0x14147578e <.text+0x147478e>
141475819:     	movq	0x18(%rax), %rax
14147581d:     	notq	%rcx
141475820:     	leaq	(%rcx,%rcx,2), %rcx
141475824:     	shlq	$0x4, %rcx
141475828:     	movq	0x10(%rax,%rcx), %rax
14147582d:     	movq	%rax, 0x20(%rsp)
141475832:     	jmp	0x141475797 <.text+0x1474797>
141475837:     	movq	$0x0, 0x20(%rsp)
141475840:     	movq	0x50(%rax), %rax
141475844:     	movq	%rax, 0x30(%rsp)
141475849:     	xorl	%ebp, %ebp
14147584b:     	movq	%rbp, %rax
14147584e:     	shlq	$0x4, %rax
141475852:     	movq	0x30(%rsp), %rcx
141475857:     	movl	0x8(%rcx,%rax), %eax
14147585b:     	shlq	$0x4, %rax
14147585f:     	incq	%rbp
141475862:     	movq	0x20(%rsp), %rcx
141475867:     	movq	(%rcx,%rax), %rdx
14147586b:     	movq	%rsi, %rcx
14147586e:     	callq	0x1402ca2f0 <.text+0x2c92f0>
141475873:     	cmpq	%r13, %rbp
141475876:     	je	0x141475571 <.text+0x1474571>
14147587c:     	cmpq	%r13, %rbp
14147587f:     	jne	0x14147584b <.text+0x147484b>
141475881:     	nopw	%cs:(%rax,%rax)
141475890:     	cmpq	%r15, %r14
141475893:     	je	0x141475571 <.text+0x1474571>
141475899:     	movl	(%r14), %ecx
14147589c:     	addq	$0x4, %r14
1414758a0:     	movq	0x8(%r12), %rax
1414758a5:     	leaq	(%rcx,%rcx,4), %rcx
1414758a9:     	shlq	$0x5, %rcx
1414758ad:     	movq	0x58(%rax,%rcx), %r13
1414758b2:     	testq	%r13, %r13
1414758b5:     	je	0x141475890 <.text+0x1474890>
1414758b7:     	addq	%rcx, %rax
1414758ba:     	movl	0x94(%rax), %ecx
1414758c0:     	movq	0x8(%rdi), %rdx
1414758c4:     	leaq	(%rcx,%rcx,8), %r8
1414758c8:     	movq	0x28(%rsp), %rcx
1414758cd:     	movq	0x110(%rcx), %rcx
1414758d4:     	cmpq	%rcx, 0x40(%rdx,%r8,8)
1414758d9:     	jbe	0x141475837 <.text+0x1474837>
1414758df:     	leaq	(%rdx,%r8,8), %rdx
1414758e3:     	movq	0x38(%rdx), %r8
1414758e7:     	movq	(%r8,%rcx,8), %rcx
1414758eb:     	testq	%rcx, %rcx
1414758ee:     	je	0x141475837 <.text+0x1474837>
1414758f4:     	movq	0x18(%rdx), %rdx
1414758f8:     	notq	%rcx
1414758fb:     	leaq	(%rcx,%rcx,2), %rcx
1414758ff:     	shlq	$0x4, %rcx
141475903:     	movq	0x10(%rdx,%rcx), %rcx
141475908:     	movq	%rcx, 0x20(%rsp)
14147590d:     	jmp	0x141475840 <.text+0x1474840>
