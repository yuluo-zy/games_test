
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1401453d0:     	pushq	%rbp
1401453d1:     	pushq	%rsi
1401453d2:     	pushq	%rdi
1401453d3:     	pushq	%rbx
1401453d4:     	subq	$0x358, %rsp            # imm = 0x358
1401453db:     	leaq	0x80(%rsp), %rbp
1401453e3:     	movq	$-0x2, 0x2d0(%rbp)
1401453ee:     	movq	%rcx, %rsi
1401453f1:     	movq	(%rcx), %rbx
1401453f4:     	movq	0x8(%rcx), %rax
1401453f8:     	movq	(%rbx), %rcx
1401453fb:     	xorl	%r8d, %r8d
1401453fe:     	cmpq	(%rax), %rcx
140145401:     	movq	%rdx, %rdi
140145404:     	sete	%r8b
140145408:     	leaq	0x280bf99(%rip), %rax   # 0x1429513a8
14014540f:     	leaq	0x280c072(%rip), %rcx   # 0x142951488
140145416:     	cmoveq	%rax, %rcx
14014541a:     	leaq	0x1(,%r8,2), %rax
140145422:     	movq	%rcx, 0x1e0(%rbp)
140145429:     	movq	%rax, 0x1e8(%rbp)
140145430:     	leaq	0x1e0(%rbp), %rax
140145437:     	movq	%rax, 0x1f0(%rbp)
14014543e:     	leaq	-0x3b5b5(%rip), %rax    # 0x140109e90 <.text+0x108e90>
140145445:     	movq	%rax, 0x1f8(%rbp)
14014544c:     	movq	%rbx, 0x200(%rbp)
140145453:     	leaq	0xc48266(%rip), %rax    # 0x140d8d6c0 <.text+0xd8c6c0>
14014545a:     	movq	%rax, 0x208(%rbp)
140145461:     	leaq	0x280c028(%rip), %rax   # 0x142951490
140145468:     	movq	%rax, 0x150(%rbp)
14014546f:     	movq	$0x2, 0x158(%rbp)
14014547a:     	leaq	0x1f0(%rbp), %rax
140145481:     	movq	%rax, 0x160(%rbp)
140145488:     	movq	$0x2, 0x168(%rbp)
140145493:     	movq	$0x0, 0x170(%rbp)
14014549e:     	leaq	-0x28(%rbp), %rcx
1401454a2:     	leaq	0x150(%rbp), %rdx
1401454a9:     	callq	0x140d79f90 <.text+0xd78f90>
1401454ae:     	movb	$0x0, 0x8(%rbp)
1401454b2:     	movl	$0x0, 0xd(%rbp)
1401454b9:     	movl	$0x0, 0x10(%rbp)
1401454c0:     	movq	$0x3, -0x58(%rbp)
1401454c8:     	movq	$0x6, -0x40(%rbp)
1401454d0:     	movl	$0x0, -0x10(%rbp)
1401454d7:     	movl	$0x0, -0x8(%rbp)
1401454de:     	movq	$0x0, (%rbp)
1401454e6:     	movw	$0x200, 0x18(%rbp)      # imm = 0x200
1401454ec:     	movb	$0x2, 0x1a(%rbp)
1401454f0:     	movb	$0x2, 0x1d(%rbp)
1401454f4:     	leaq	0x150(%rbp), %rcx
1401454fb:     	leaq	-0x58(%rbp), %rdx
1401454ff:     	movq	%rdi, %r8
140145502:     	callq	0x140c28d40 <.text+0xc27d40>
140145507:     	movq	0x160(%rbp), %rax
14014550e:     	lock
14014550f:     	decq	(%rax)
140145512:     	jne	0x140145520 <.text+0x144520>
140145514:     	leaq	0x160(%rbp), %rcx
14014551b:     	callq	0x140c41170 <.text+0xc40170>
140145520:     	movq	$0x4, 0x150(%rbp)
14014552b:     	movb	$0x0, 0x2cf(%rbp)
140145532:     	leaq	0x280bf77(%rip), %rdx   # 0x1429514b0
140145539:     	leaq	-0x58(%rbp), %rcx
14014553d:     	movl	$0x7, %r8d
140145543:     	callq	0x1400568d0 <.text+0x558d0>
140145548:     	movups	0x8(%rbp), %xmm0
14014554c:     	movaps	%xmm0, 0x250(%rbp)
140145553:     	movups	-0x8(%rbp), %xmm0
140145557:     	movaps	%xmm0, 0x240(%rbp)
14014555e:     	movups	-0x18(%rbp), %xmm0
140145562:     	movaps	%xmm0, 0x230(%rbp)
140145569:     	movups	-0x58(%rbp), %xmm0
14014556d:     	movups	-0x48(%rbp), %xmm1
140145571:     	movups	-0x38(%rbp), %xmm2
140145575:     	movups	-0x28(%rbp), %xmm3
140145579:     	movaps	%xmm3, 0x220(%rbp)
140145580:     	movaps	%xmm2, 0x210(%rbp)
140145587:     	movaps	%xmm1, 0x200(%rbp)
14014558e:     	movaps	%xmm0, 0x1f0(%rbp)
140145595:     	movb	$0x1, 0x2cf(%rbp)
14014559c:     	leaq	-0x58(%rbp), %rcx
1401455a0:     	leaq	0x150(%rbp), %rdx
1401455a7:     	leaq	0x1f0(%rbp), %r8
1401455ae:     	callq	0x140c46bd0 <.text+0xc45bd0>
1401455b3:     	leaq	0x268(%rbp), %rcx
1401455ba:     	leaq	-0x58(%rbp), %rdx
1401455be:     	movq	%rdi, %r8
1401455c1:     	callq	0x140c46d20 <.text+0xc45d20>
1401455c6:     	cmpb	$0x0, 0x2bc(%rbp)
1401455cd:     	je	0x1401455fc <.text+0x1445fc>
1401455cf:     	movq	0x278(%rbp), %rax
1401455d6:     	lock
1401455d7:     	decq	(%rax)
1401455da:     	jne	0x1401455e8 <.text+0x1445e8>
1401455dc:     	leaq	0x278(%rbp), %rcx
1401455e3:     	callq	0x140c41170 <.text+0xc40170>
1401455e8:     	movq	(%rbx), %rax
1401455eb:     	movq	0x10(%rsi), %rcx
1401455ef:     	movq	$0x2, (%rcx)
1401455f6:     	movq	%rax, 0x8(%rcx)
1401455fa:     	jmp	0x140145636 <.text+0x144636>
1401455fc:     	movb	$0x0, -0x58(%rbp)
140145600:     	cmpb	$0x0, 0x2bb(%rbp)
140145607:     	je	0x14014561d <.text+0x14461d>
140145609:     	leaq	0x278(%rbp), %rcx
140145610:     	leaq	-0x58(%rbp), %rdx
140145614:     	callq	0x140042870 <.text+0x41870>
140145619:     	testb	%al, %al
14014561b:     	jne	0x1401455cf <.text+0x1445cf>
14014561d:     	movq	0x278(%rbp), %rax
140145624:     	lock
140145625:     	decq	(%rax)
140145628:     	jne	0x140145636 <.text+0x144636>
14014562a:     	leaq	0x278(%rbp), %rcx
140145631:     	callq	0x140c41170 <.text+0xc40170>
140145636:     	movq	$0x4, 0x150(%rbp)
140145641:     	movb	$0x0, 0x2ce(%rbp)
140145648:     	leaq	0x280be68(%rip), %rdx   # 0x1429514b7
14014564f:     	leaq	-0x58(%rbp), %rcx
140145653:     	movl	$0x6, %r8d
140145659:     	callq	0x1400568d0 <.text+0x558d0>
14014565e:     	movups	0x8(%rbp), %xmm0
140145662:     	movaps	%xmm0, 0x250(%rbp)
140145669:     	movups	-0x8(%rbp), %xmm0
14014566d:     	movaps	%xmm0, 0x240(%rbp)
140145674:     	movups	-0x18(%rbp), %xmm0
140145678:     	movaps	%xmm0, 0x230(%rbp)
14014567f:     	movups	-0x58(%rbp), %xmm0
140145683:     	movups	-0x48(%rbp), %xmm1
140145687:     	movups	-0x38(%rbp), %xmm2
14014568b:     	movups	-0x28(%rbp), %xmm3
14014568f:     	movaps	%xmm3, 0x220(%rbp)
140145696:     	movaps	%xmm2, 0x210(%rbp)
14014569d:     	movaps	%xmm1, 0x200(%rbp)
1401456a4:     	movaps	%xmm0, 0x1f0(%rbp)
1401456ab:     	movb	$0x1, 0x2ce(%rbp)
1401456b2:     	leaq	-0x58(%rbp), %rcx
1401456b6:     	leaq	0x150(%rbp), %rdx
1401456bd:     	leaq	0x1f0(%rbp), %r8
1401456c4:     	callq	0x140c46bd0 <.text+0xc45bd0>
1401456c9:     	leaq	0x268(%rbp), %rcx
1401456d0:     	leaq	-0x58(%rbp), %rdx
1401456d4:     	movq	%rdi, %r8
1401456d7:     	callq	0x140c46d20 <.text+0xc45d20>
1401456dc:     	cmpb	$0x0, 0x2bc(%rbp)
1401456e3:     	je	0x140145712 <.text+0x144712>
1401456e5:     	movq	0x278(%rbp), %rax
1401456ec:     	lock
1401456ed:     	decq	(%rax)
1401456f0:     	jne	0x1401456fe <.text+0x1446fe>
1401456f2:     	leaq	0x278(%rbp), %rcx
1401456f9:     	callq	0x140c41170 <.text+0xc40170>
1401456fe:     	movq	(%rbx), %rax
140145701:     	movq	0x10(%rsi), %rcx
140145705:     	movq	$0x3, (%rcx)
14014570c:     	movq	%rax, 0x8(%rcx)
140145710:     	jmp	0x14014574c <.text+0x14474c>
140145712:     	movb	$0x0, -0x58(%rbp)
140145716:     	cmpb	$0x0, 0x2bb(%rbp)
14014571d:     	je	0x140145733 <.text+0x144733>
14014571f:     	leaq	0x278(%rbp), %rcx
140145726:     	leaq	-0x58(%rbp), %rdx
14014572a:     	callq	0x140042870 <.text+0x41870>
14014572f:     	testb	%al, %al
140145731:     	jne	0x1401456e5 <.text+0x1446e5>
140145733:     	movq	0x278(%rbp), %rax
14014573a:     	lock
14014573b:     	decq	(%rax)
14014573e:     	jne	0x14014574c <.text+0x14474c>
140145740:     	leaq	0x278(%rbp), %rcx
140145747:     	callq	0x140c41170 <.text+0xc40170>
14014574c:     	nop
14014574d:     	addq	$0x358, %rsp            # imm = 0x358
140145754:     	popq	%rbx
140145755:     	popq	%rdi
140145756:     	popq	%rsi
140145757:     	popq	%rbp
140145758:     	retq
140145759:     	nopl	(%rax)
140145760:     	movq	%rdx, 0x10(%rsp)
140145765:     	pushq	%rbp
140145766:     	pushq	%rsi
140145767:     	pushq	%rdi
140145768:     	pushq	%rbx
140145769:     	subq	$0x28, %rsp
14014576d:     	leaq	0x80(%rdx), %rbp
140145774:     	cmpb	$0x0, 0x2cf(%rbp)
14014577b:     	jne	0x140145793 <.text+0x144793>
14014577d:     	cmpq	$0x4, 0x150(%rbp)
140145785:     	je	0x140145793 <.text+0x144793>
140145787:     	leaq	0x150(%rbp), %rcx
14014578e:     	callq	0x1405ea2f0 <.text+0x5e92f0>
140145793:     	nop
140145794:     	addq	$0x28, %rsp
140145798:     	popq	%rbx
140145799:     	popq	%rdi
14014579a:     	popq	%rsi
14014579b:     	popq	%rbp
14014579c:     	retq
14014579d:     	nopl	(%rax)
1401457a0:     	movq	%rdx, 0x10(%rsp)
1401457a5:     	pushq	%rbp
1401457a6:     	pushq	%rsi
1401457a7:     	pushq	%rdi
1401457a8:     	pushq	%rbx
1401457a9:     	subq	$0x28, %rsp
1401457ad:     	leaq	0x80(%rdx), %rbp
1401457b4:     	leaq	0x278(%rbp), %rcx
1401457bb:     	movq	0x278(%rbp), %rax
1401457c2:     	lock
1401457c3:     	decq	(%rax)
1401457c6:     	jne	0x1401457cd <.text+0x1447cd>
1401457c8:     	callq	0x140c41170 <.text+0xc40170>
1401457cd:     	nop
1401457ce:     	addq	$0x28, %rsp
1401457d2:     	popq	%rbx
1401457d3:     	popq	%rdi
1401457d4:     	popq	%rsi
1401457d5:     	popq	%rbp
1401457d6:     	retq
1401457d7:     	nopw	(%rax,%rax)
1401457e0:     	movq	%rdx, 0x10(%rsp)
1401457e5:     	pushq	%rbp
1401457e6:     	pushq	%rsi
1401457e7:     	pushq	%rdi
1401457e8:     	pushq	%rbx
1401457e9:     	subq	$0x28, %rsp
1401457ed:     	leaq	0x80(%rdx), %rbp
1401457f4:     	cmpb	$0x0, 0x2ce(%rbp)
1401457fb:     	jne	0x140145813 <.text+0x144813>
1401457fd:     	cmpq	$0x4, 0x150(%rbp)
140145805:     	je	0x140145813 <.text+0x144813>
140145807:     	leaq	0x150(%rbp), %rcx
14014580e:     	callq	0x1405ea2f0 <.text+0x5e92f0>
140145813:     	nop
140145814:     	addq	$0x28, %rsp
140145818:     	popq	%rbx
140145819:     	popq	%rdi
14014581a:     	popq	%rsi
14014581b:     	popq	%rbp
14014581c:     	retq
14014581d:     	nopl	(%rax)
140145820:     	movq	%rdx, 0x10(%rsp)
140145825:     	pushq	%rbp
140145826:     	pushq	%rsi
140145827:     	pushq	%rdi
140145828:     	pushq	%rbx
140145829:     	subq	$0x28, %rsp
14014582d:     	leaq	0x80(%rdx), %rbp
140145834:     	leaq	0x278(%rbp), %rcx
14014583b:     	movq	0x278(%rbp), %rax
140145842:     	lock
140145843:     	decq	(%rax)
140145846:     	jne	0x14014584d <.text+0x14484d>
140145848:     	callq	0x140c41170 <.text+0xc40170>
14014584d:     	nop
14014584e:     	addq	$0x28, %rsp
140145852:     	popq	%rbx
140145853:     	popq	%rdi
140145854:     	popq	%rsi
140145855:     	popq	%rbp
140145856:     	retq
