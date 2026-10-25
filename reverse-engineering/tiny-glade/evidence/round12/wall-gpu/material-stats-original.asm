
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a88280:     	pushq	%rsi
140a88281:     	pushq	%rbx
140a88282:     	subq	$0x38, %rsp
140a88286:     	movaps	%xmm6, 0x20(%rsp)
140a8828b:     	movl	%r8d, %ebx
140a8828e:     	movdqa	%xmm1, %xmm6
140a88292:     	movq	%rcx, %rsi
140a88295:     	movq	0x70(%rcx), %rdx
140a88299:     	testq	%rdx, %rdx
140a8829c:     	je	0x140a882b1 <.text+0xa872b1>
140a8829e:     	movq	0x78(%rsi), %rcx
140a882a2:     	shlq	$0x3, %rdx
140a882a6:     	movl	$0x4, %r8d
140a882ac:     	callq	0x140613c20 <.text+0x612c20>
140a882b1:     	movb	$0x0, 0x68(%rsi)
140a882b5:     	movq	$0x0, 0x70(%rsi)
140a882bd:     	movq	$0x4, 0x78(%rsi)
140a882c5:     	xorps	%xmm0, %xmm0
140a882c8:     	movups	%xmm0, 0x80(%rsi)
140a882cf:     	movups	%xmm0, 0x90(%rsi)
140a882d6:     	movups	%xmm0, 0xa0(%rsi)
140a882dd:     	movups	%xmm0, 0xb0(%rsi)
140a882e4:     	movups	%xmm0, 0xbc(%rsi)
140a882eb:     	testb	%bl, %bl
140a882ed:     	je	0x140a882ff <.text+0xa872ff>
140a882ef:     	movl	$0x0, 0x90(%rsi)
140a882f9:     	movd	%xmm6, %eax
140a882fd:     	jmp	0x140a88309 <.text+0xa87309>
140a882ff:     	movd	%xmm6, 0x90(%rsi)
140a88307:     	xorl	%eax, %eax
140a88309:     	movl	%eax, 0x9c(%rsi)
140a8830f:     	movaps	0x20(%rsp), %xmm6
140a88314:     	addq	$0x38, %rsp
140a88318:     	popq	%rbx
140a88319:     	popq	%rsi
140a8831a:     	retq
140a8831b:     	int3
140a8831c:     	int3
140a8831d:     	int3
140a8831e:     	int3
140a8831f:     	int3
140a88320:     	pushq	%rbp
140a88321:     	pushq	%rsi
140a88322:     	pushq	%rdi
140a88323:     	pushq	%rbx
140a88324:     	subq	$0x38, %rsp
140a88328:     	leaq	0x30(%rsp), %rbp
140a8832d:     	movq	$-0x2, (%rbp)
140a88335:     	movq	%rcx, %rdi
140a88338:     	movq	(%rdx), %rax
140a8833b:     	movq	%rax, -0x8(%rbp)
140a8833f:     	movq	0x8(%rdx), %rax
140a88343:     	movq	%rax, -0x10(%rbp)
140a88347:     	movq	0x10(%rdx), %rbx
140a8834b:     	leaq	0x68(%rcx), %rsi
140a8834f:     	movb	$0x1, %cl
140a88351:     	xorl	%eax, %eax
140a88353:     	lock
140a88354:     	cmpxchgb	%cl, 0x68(%rdi)
140a88358:     	jne	0x140a883b1 <.text+0xa873b1>
140a8835a:     	movq	%rsi, %rcx
140a8835d:     	callq	0x140c81c70 <.text+0xc80c70>
140a88362:     	movq	0x70(%rdi), %rdx
140a88366:     	testq	%rdx, %rdx
140a88369:     	je	0x140a8837e <.text+0xa8737e>
140a8836b:     	movq	0x78(%rdi), %rcx
140a8836f:     	shlq	$0x3, %rdx
140a88373:     	movl	$0x4, %r8d
140a88379:     	callq	0x140613c20 <.text+0x612c20>
140a8837e:     	movq	-0x8(%rbp), %rax
140a88382:     	movq	%rax, 0x70(%rdi)
140a88386:     	movq	-0x10(%rbp), %rax
140a8838a:     	movq	%rax, 0x78(%rdi)
140a8838e:     	movq	%rbx, 0x80(%rdi)
140a88395:     	movq	%rsi, %rcx
140a88398:     	callq	0x140c81e10 <.text+0xc80e10>
140a8839d:     	xorl	%ecx, %ecx
140a8839f:     	movb	$0x1, %al
140a883a1:     	lock
140a883a2:     	cmpxchgb	%cl, 0x68(%rdi)
140a883a6:     	jne	0x140a883c1 <.text+0xa873c1>
140a883a8:     	addq	$0x38, %rsp
140a883ac:     	popq	%rbx
140a883ad:     	popq	%rdi
140a883ae:     	popq	%rsi
140a883af:     	popq	%rbp
140a883b0:     	retq
140a883b1:     	movq	%rsi, %rcx
140a883b4:     	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140a883ba:     	callq	0x1428cd210 <.text+0x28cc210>
140a883bf:     	jmp	0x140a8835a <.text+0xa8735a>
140a883c1:     	movq	%rsi, %rcx
140a883c4:     	xorl	%edx, %edx
140a883c6:     	addq	$0x38, %rsp
140a883ca:     	popq	%rbx
140a883cb:     	popq	%rdi
140a883cc:     	popq	%rsi
140a883cd:     	popq	%rbp
140a883ce:     	jmp	0x1428cdb90 <.text+0x28ccb90>
140a883d3:     	nopw	%cs:(%rax,%rax)
140a883e0:     	movq	%rdx, 0x10(%rsp)
140a883e5:     	pushq	%rbp
140a883e6:     	pushq	%rsi
140a883e7:     	pushq	%rdi
140a883e8:     	pushq	%rbx
140a883e9:     	subq	$0x28, %rsp
140a883ed:     	leaq	0x30(%rdx), %rbp
140a883f1:     	cmpq	$0x0, -0x8(%rbp)
140a883f6:     	je	0x140a8840f <.text+0xa8740f>
140a883f8:     	movq	-0x8(%rbp), %rdx
140a883fc:     	shlq	$0x3, %rdx
140a88400:     	movl	$0x4, %r8d
140a88406:     	movq	-0x10(%rbp), %rcx
140a8840a:     	callq	0x140613c20 <.text+0x612c20>
140a8840f:     	nop
140a88410:     	addq	$0x28, %rsp
140a88414:     	popq	%rbx
140a88415:     	popq	%rdi
140a88416:     	popq	%rsi
140a88417:     	popq	%rbp
140a88418:     	retq
140a88419:     	int3
140a8841a:     	int3
140a8841b:     	int3
140a8841c:     	int3
140a8841d:     	int3
140a8841e:     	int3
140a8841f:     	int3
140a88420:     	movl	%edx, 0xb4(%rcx)
140a88426:     	retq
140a88427:     	int3
140a88428:     	int3
140a88429:     	int3
140a8842a:     	int3
140a8842b:     	int3
140a8842c:     	int3
140a8842d:     	int3
140a8842e:     	int3
140a8842f:     	int3
140a88430:     	movl	%edx, 0xbc(%rcx)
140a88436:     	retq
140a88437:     	int3
140a88438:     	int3
140a88439:     	int3
140a8843a:     	int3
140a8843b:     	int3
140a8843c:     	int3
140a8843d:     	int3
140a8843e:     	int3
140a8843f:     	int3
140a88440:     	movl	%edx, 0xac(%rcx)
140a88446:     	retq
140a88447:     	int3
140a88448:     	int3
140a88449:     	int3
140a8844a:     	int3
140a8844b:     	int3
140a8844c:     	int3
140a8844d:     	int3
140a8844e:     	int3
140a8844f:     	int3
140a88450:     	movl	0x94(%rcx), %eax
140a88456:     	nopw	%cs:(%rax,%rax)
140a88460:     	movd	%eax, %xmm0
140a88464:     	addss	%xmm1, %xmm0
140a88468:     	movd	%xmm0, %edx
140a8846c:     	lock
140a8846d:     	cmpxchgl	%edx, 0x94(%rcx)
140a88474:     	jne	0x140a88460 <.text+0xa87460>
140a88476:     	movl	0x9c(%rcx), %eax
140a8847c:     	nopl	(%rax)
140a88480:     	movd	%eax, %xmm0
140a88484:     	subss	%xmm1, %xmm0
140a88488:     	movd	%xmm0, %edx
140a8848c:     	lock
140a8848d:     	cmpxchgl	%edx, 0x9c(%rcx)
140a88494:     	jne	0x140a88480 <.text+0xa87480>
140a88496:     	retq
140a88497:     	int3
140a88498:     	int3
140a88499:     	int3
140a8849a:     	int3
140a8849b:     	int3
140a8849c:     	int3
140a8849d:     	int3
140a8849e:     	int3
140a8849f:     	int3
140a884a0:     	movl	0x98(%rcx), %eax
140a884a6:     	nopw	%cs:(%rax,%rax)
140a884b0:     	movd	%eax, %xmm0
140a884b4:     	addss	%xmm1, %xmm0
140a884b8:     	movd	%xmm0, %edx
140a884bc:     	lock
140a884bd:     	cmpxchgl	%edx, 0x98(%rcx)
140a884c4:     	jne	0x140a884b0 <.text+0xa874b0>
140a884c6:     	movl	0x90(%rcx), %eax
140a884cc:     	nopl	(%rax)
140a884d0:     	movd	%eax, %xmm0
140a884d4:     	subss	%xmm1, %xmm0
140a884d8:     	movd	%xmm0, %edx
140a884dc:     	lock
140a884dd:     	cmpxchgl	%edx, 0x90(%rcx)
140a884e4:     	jne	0x140a884d0 <.text+0xa874d0>
140a884e6:     	retq
140a884e7:     	int3
140a884e8:     	int3
140a884e9:     	int3
140a884ea:     	int3
140a884eb:     	int3
140a884ec:     	int3
140a884ed:     	int3
140a884ee:     	int3
140a884ef:     	int3
140a884f0:     	movl	0xa0(%rcx), %eax
140a884f6:     	nopw	%cs:(%rax,%rax)
140a88500:     	movd	%eax, %xmm0
140a88504:     	addss	%xmm1, %xmm0
140a88508:     	movd	%xmm0, %edx
140a8850c:     	lock
140a8850d:     	cmpxchgl	%edx, 0xa0(%rcx)
140a88514:     	jne	0x140a88500 <.text+0xa87500>
140a88516:     	movl	0x90(%rcx), %eax
140a8851c:     	nopl	(%rax)
140a88520:     	movd	%eax, %xmm0
140a88524:     	subss	%xmm1, %xmm0
140a88528:     	movd	%xmm0, %edx
140a8852c:     	lock
140a8852d:     	cmpxchgl	%edx, 0x90(%rcx)
140a88534:     	jne	0x140a88520 <.text+0xa87520>
140a88536:     	retq
140a88537:     	int3
140a88538:     	int3
140a88539:     	int3
140a8853a:     	int3
140a8853b:     	int3
140a8853c:     	int3
140a8853d:     	int3
140a8853e:     	int3
140a8853f:     	int3
140a88540:     	movl	0xa8(%rcx), %eax
140a88546:     	nopw	%cs:(%rax,%rax)
140a88550:     	movd	%eax, %xmm0
140a88554:     	addss	%xmm1, %xmm0
140a88558:     	movd	%xmm0, %edx
140a8855c:     	lock
140a8855d:     	cmpxchgl	%edx, 0xa8(%rcx)
140a88564:     	jne	0x140a88550 <.text+0xa87550>
140a88566:     	movl	0x90(%rcx), %eax
140a8856c:     	nopl	(%rax)
140a88570:     	movd	%eax, %xmm0
140a88574:     	subss	%xmm1, %xmm0
140a88578:     	movd	%xmm0, %edx
140a8857c:     	lock
140a8857d:     	cmpxchgl	%edx, 0x90(%rcx)
140a88584:     	jne	0x140a88570 <.text+0xa87570>
140a88586:     	retq
140a88587:     	int3
140a88588:     	int3
140a88589:     	int3
140a8858a:     	int3
140a8858b:     	int3
140a8858c:     	int3
140a8858d:     	int3
140a8858e:     	int3
140a8858f:     	int3
140a88590:     	movl	0xa4(%rcx), %eax
140a88596:     	nopw	%cs:(%rax,%rax)
140a885a0:     	movd	%eax, %xmm0
140a885a4:     	addss	%xmm1, %xmm0
140a885a8:     	movd	%xmm0, %edx
140a885ac:     	lock
140a885ad:     	cmpxchgl	%edx, 0xa4(%rcx)
140a885b4:     	jne	0x140a885a0 <.text+0xa875a0>
140a885b6:     	movl	0x90(%rcx), %eax
140a885bc:     	nopl	(%rax)
140a885c0:     	movd	%eax, %xmm0
140a885c4:     	subss	%xmm1, %xmm0
140a885c8:     	movd	%xmm0, %edx
140a885cc:     	lock
140a885cd:     	cmpxchgl	%edx, 0x90(%rcx)
140a885d4:     	jne	0x140a885c0 <.text+0xa875c0>
140a885d6:     	retq
140a885d7:     	int3
140a885d8:     	int3
140a885d9:     	int3
140a885da:     	int3
140a885db:     	int3
140a885dc:     	int3
140a885dd:     	int3
140a885de:     	int3
140a885df:     	int3
140a885e0:     	movl	%edx, 0xb0(%rcx)
140a885e6:     	retq
140a885e7:     	int3
140a885e8:     	int3
140a885e9:     	int3
140a885ea:     	int3
140a885eb:     	int3
140a885ec:     	int3
140a885ed:     	int3
140a885ee:     	int3
140a885ef:     	int3
140a885f0:     	movl	%edx, 0xb8(%rcx)
140a885f6:     	retq
140a885f7:     	int3
140a885f8:     	int3
140a885f9:     	int3
140a885fa:     	int3
140a885fb:     	int3
140a885fc:     	int3
140a885fd:     	int3
140a885fe:     	int3
140a885ff:     	int3
140a88600:     	movl	%edx, 0xc0(%rcx)
140a88606:     	retq
140a88607:     	int3
140a88608:     	int3
140a88609:     	int3
140a8860a:     	int3
140a8860b:     	int3
140a8860c:     	int3
140a8860d:     	int3
140a8860e:     	int3
140a8860f:     	int3
140a88610:     	movl	%edx, 0xc4(%rcx)
140a88616:     	retq
140a88617:     	int3
140a88618:     	int3
140a88619:     	int3
140a8861a:     	int3
140a8861b:     	int3
140a8861c:     	int3
140a8861d:     	int3
140a8861e:     	int3
140a8861f:     	int3
140a88620:     	movl	%edx, 0xc8(%rcx)
140a88626:     	retq
140a88627:     	int3
140a88628:     	int3
140a88629:     	int3
140a8862a:     	int3
140a8862b:     	int3
140a8862c:     	int3
140a8862d:     	int3
140a8862e:     	int3
140a8862f:     	int3
140a88630:     	movq	%rdx, 0x88(%rcx)
140a88637:     	retq
140a88638:     	int3
140a88639:     	int3
140a8863a:     	int3
140a8863b:     	int3
140a8863c:     	int3
140a8863d:     	int3
140a8863e:     	int3
140a8863f:     	int3
140a88640:     	pushq	%rbp
140a88641:     	pushq	%r14
140a88643:     	pushq	%rsi
140a88644:     	pushq	%rdi
140a88645:     	pushq	%rbx
140a88646:     	subq	$0x40, %rsp
140a8864a:     	leaq	0x40(%rsp), %rbp
140a8864f:     	movaps	%xmm6, -0x10(%rbp)
140a88653:     	movq	$-0x2, -0x18(%rbp)
140a8865b:     	movq	%rcx, %rsi
140a8865e:     	addq	$0x68, %rcx
140a88662:     	movb	$0x1, %r14b
140a88665:     	xorl	%eax, %eax
140a88667:     	lock
140a88668:     	cmpxchgb	%r14b, 0x68(%rsi)
140a8866d:     	jne	0x140a88742 <.text+0xa87742>
140a88673:     	movq	%rcx, -0x20(%rbp)
140a88677:     	callq	0x140c81c70 <.text+0xc80c70>
140a8867c:     	movq	0x78(%rsi), %rdi
140a88680:     	movq	0x80(%rsi), %rbx
140a88687:     	xorl	%eax, %eax
140a88689:     	lock
140a8868a:     	cmpxchgb	%r14b, (%rsi)
140a8868e:     	jne	0x140a88758 <.text+0xa87758>
140a88694:     	movq	%rsi, %rcx
140a88697:     	callq	0x140c81c70 <.text+0xc80c70>
140a8869c:     	movq	0x18(%rsi), %rax
140a886a0:     	cmpq	%rbx, %rax
140a886a3:     	cmovbq	%rax, %rbx
140a886a7:     	testq	%rbx, %rbx
140a886aa:     	je	0x140a88705 <.text+0xa87705>
140a886ac:     	movq	0x10(%rsi), %rax
140a886b0:     	xorps	%xmm0, %xmm0
140a886b3:     	xorl	%ecx, %ecx
140a886b5:     	movaps	%xmm0, %xmm6
140a886b8:     	nopl	(%rax,%rax)
140a886c0:     	movsd	(%rax,%rcx,8), %xmm1
140a886c5:     	movsd	(%rdi,%rcx,8), %xmm2
140a886ca:     	leaq	0x1(%rcx), %rdx
140a886ce:     	subps	%xmm2, %xmm1
140a886d1:     	mulps	%xmm1, %xmm1
140a886d4:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
140a886d8:     	addss	%xmm1, %xmm2
140a886dc:     	xorps	%xmm1, %xmm1
140a886df:     	sqrtss	%xmm2, %xmm1
140a886e3:     	cmpunordss	%xmm6, %xmm6
140a886e8:     	movaps	%xmm6, %xmm2
140a886eb:     	andps	%xmm1, %xmm2
140a886ee:     	maxss	%xmm0, %xmm1
140a886f2:     	andnps	%xmm1, %xmm6
140a886f5:     	orps	%xmm2, %xmm6
140a886f8:     	movaps	%xmm6, %xmm0
140a886fb:     	movq	%rdx, %rcx
140a886fe:     	cmpq	%rdx, %rbx
140a88701:     	jne	0x140a886c0 <.text+0xa876c0>
140a88703:     	jmp	0x140a88708 <.text+0xa87708>
140a88705:     	xorps	%xmm6, %xmm6
140a88708:     	movq	%rsi, %rcx
140a8870b:     	callq	0x140c81e10 <.text+0xc80e10>
140a88710:     	xorl	%ecx, %ecx
140a88712:     	movb	$0x1, %al
140a88714:     	lock
140a88715:     	cmpxchgb	%cl, (%rsi)
140a88718:     	jne	0x140a8876b <.text+0xa8776b>
140a8871a:     	movq	-0x20(%rbp), %rsi
140a8871e:     	movq	%rsi, %rcx
140a88721:     	callq	0x140c81e10 <.text+0xc80e10>
140a88726:     	xorl	%ecx, %ecx
140a88728:     	movb	$0x1, %al
140a8872a:     	lock
140a8872b:     	cmpxchgb	%cl, (%rsi)
140a8872e:     	jne	0x140a88777 <.text+0xa87777>
140a88730:     	movaps	%xmm6, %xmm0
140a88733:     	movaps	-0x10(%rbp), %xmm6
140a88737:     	addq	$0x40, %rsp
140a8873b:     	popq	%rbx
140a8873c:     	popq	%rdi
140a8873d:     	popq	%rsi
140a8873e:     	popq	%r14
140a88740:     	popq	%rbp
140a88741:     	retq
140a88742:     	movq	%rcx, %rdi
140a88745:     	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140a8874b:     	callq	0x1428cd210 <.text+0x28cc210>
140a88750:     	movq	%rdi, %rcx
140a88753:     	jmp	0x140a88673 <.text+0xa87673>
140a88758:     	movq	%rsi, %rcx
140a8875b:     	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140a88761:     	callq	0x1428cd210 <.text+0x28cc210>
140a88766:     	jmp	0x140a88694 <.text+0xa87694>
140a8876b:     	movq	%rsi, %rcx
140a8876e:     	xorl	%edx, %edx
140a88770:     	callq	0x1428cdb90 <.text+0x28ccb90>
140a88775:     	jmp	0x140a8871a <.text+0xa8771a>
140a88777:     	movq	%rsi, %rcx
140a8877a:     	xorl	%edx, %edx
140a8877c:     	callq	0x1428cdb90 <.text+0x28ccb90>
140a88781:     	jmp	0x140a88730 <.text+0xa87730>
140a88783:     	nopw	%cs:(%rax,%rax)
140a88790:     	movq	%rdx, 0x10(%rsp)
140a88795:     	pushq	%rbp
140a88796:     	pushq	%r14
140a88798:     	pushq	%rsi
140a88799:     	pushq	%rdi
140a8879a:     	pushq	%rbx
140a8879b:     	subq	$0x30, %rsp
140a8879f:     	leaq	0x40(%rdx), %rbp
140a887a3:     	movaps	%xmm6, 0x20(%rsp)
140a887a8:     	movq	-0x20(%rbp), %rcx
140a887ac:     	callq	0x140161c30 <.text+0x160c30>
140a887b1:     	movaps	0x20(%rsp), %xmm6
140a887b6:     	addq	$0x30, %rsp
140a887ba:     	popq	%rbx
140a887bb:     	popq	%rdi
140a887bc:     	popq	%rsi
140a887bd:     	popq	%r14
140a887bf:     	popq	%rbp
140a887c0:     	retq
140a887c1:     	int3
140a887c2:     	int3
140a887c3:     	int3
140a887c4:     	int3
140a887c5:     	int3
140a887c6:     	int3
140a887c7:     	int3
140a887c8:     	int3
140a887c9:     	int3
140a887ca:     	int3
140a887cb:     	int3
140a887cc:     	int3
140a887cd:     	int3
140a887ce:     	int3
140a887cf:     	int3
140a887d0:     	pushq	%rsi
140a887d1:     	subq	$0x20, %rsp
140a887d5:     	movaps	%xmm7, 0x10(%rsp)
140a887da:     	movaps	%xmm6, (%rsp)
140a887de:     	movl	0x9c(%rcx), %eax
140a887e4:     	movl	0x34(%rcx), %edx
140a887e7:     	movl	0x98(%rcx), %r8d
140a887ee:     	movl	0x30(%rcx), %r9d
140a887f2:     	movd	%eax, %xmm2
140a887f6:     	movd	%r8d, %xmm0
140a887fb:     	punpckldq	%xmm0, %xmm2    # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
140a887ff:     	movd	%edx, %xmm0
140a88803:     	movd	%r9d, %xmm1
140a88808:     	punpckldq	%xmm1, %xmm0    # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
140a8880c:     	subps	%xmm0, %xmm2
140a8880f:     	movaps	0x1ea616a(%rip), %xmm4  # 0x14292e980
140a88816:     	andps	%xmm4, %xmm2
140a88819:     	movl	0x94(%rcx), %eax
140a8881f:     	movl	0x2c(%rcx), %r10d
140a88823:     	movss	0xa0(%rcx), %xmm0
140a8882b:     	subss	0x38(%rcx), %xmm0
140a88830:     	andps	%xmm4, %xmm0
140a88833:     	movl	0xa8(%rcx), %edx
140a88839:     	movl	0x40(%rcx), %r8d
140a8883d:     	movl	0x90(%rcx), %r11d
140a88844:     	movl	0x28(%rcx), %esi
140a88847:     	movl	0xa4(%rcx), %r9d
140a8884e:     	movl	0x3c(%rcx), %ecx
140a88851:     	pshufd	$0xf5, %xmm2, %xmm1     # xmm1 = xmm2[1,1,3,3]
140a88856:     	movaps	%xmm2, %xmm5
140a88859:     	addss	%xmm1, %xmm5
140a8885d:     	movd	%eax, %xmm3
140a88861:     	movd	%r11d, %xmm6
140a88866:     	punpckldq	%xmm6, %xmm3    # xmm3 = xmm3[0],xmm6[0],xmm3[1],xmm6[1]
140a8886a:     	movd	%r10d, %xmm6
140a8886f:     	movd	%esi, %xmm7
140a88873:     	punpckldq	%xmm7, %xmm6    # xmm6 = xmm6[0],xmm7[0],xmm6[1],xmm7[1]
140a88877:     	subps	%xmm6, %xmm3
140a8887a:     	andps	%xmm4, %xmm3
140a8887d:     	movaps	0x205018c(%rip), %xmm6  # 0x142ad8a10
140a88884:     	movss	%xmm5, %xmm6            # xmm6 = xmm5[0],xmm6[1,2,3]
140a88888:     	addps	%xmm3, %xmm6
140a8888b:     	movshdup	%xmm6, %xmm4            # xmm4 = xmm6[1,1,3,3]
140a8888f:     	addss	%xmm4, %xmm6
140a88893:     	addss	%xmm0, %xmm6
140a88897:     	movb	$0x7, %al
140a88899:     	xorps	%xmm5, %xmm5
140a8889c:     	ucomiss	%xmm6, %xmm5
140a8889f:     	jae	0x140a88997 <.text+0xa87997>
140a888a5:     	movd	%r9d, %xmm5
140a888aa:     	movd	%ecx, %xmm6
140a888ae:     	subss	%xmm6, %xmm5
140a888b2:     	movd	%xmm5, %eax
140a888b6:     	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
140a888bb:     	movd	%edx, %xmm5
140a888bf:     	movd	%r8d, %xmm6
140a888c4:     	subss	%xmm6, %xmm5
140a888c8:     	movd	%xmm5, %edx
140a888cc:     	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
140a888d2:     	movd	%xmm3, %r8d
140a888d7:     	movd	%xmm2, %ecx
140a888db:     	movd	%xmm4, %r9d
140a888e0:     	movl	%r9d, %r10d
140a888e3:     	sarl	$0x1f, %r10d
140a888e7:     	shrl	%r10d
140a888ea:     	xorl	%r9d, %r10d
140a888ed:     	xorl	%r11d, %r11d
140a888f0:     	cmpl	%ecx, %r10d
140a888f3:     	cmovgl	%r9d, %ecx
140a888f7:     	setle	%r11b
140a888fb:     	movl	%ecx, %r9d
140a888fe:     	sarl	$0x1f, %r9d
140a88902:     	shrl	%r9d
140a88905:     	xorl	%ecx, %r9d
140a88908:     	cmpl	%r8d, %r9d
140a8890b:     	cmovlel	%r8d, %ecx
140a8890f:     	leal	(%r11,%r11,2), %r8d
140a88913:     	movl	$0x1, %r9d
140a88919:     	cmovgl	%r8d, %r9d
140a8891d:     	movd	%xmm1, %r8d
140a88922:     	movl	%ecx, %r10d
140a88925:     	sarl	$0x1f, %r10d
140a88929:     	shrl	%r10d
140a8892c:     	xorl	%ecx, %r10d
140a8892f:     	cmpl	%r8d, %r10d
140a88932:     	cmovlel	%r8d, %ecx
140a88936:     	movl	$0x2, %r8d
140a8893c:     	cmovgl	%r9d, %r8d
140a88940:     	movd	%xmm0, %r9d
140a88945:     	movl	%ecx, %r10d
140a88948:     	sarl	$0x1f, %r10d
140a8894c:     	shrl	%r10d
140a8894f:     	xorl	%ecx, %r10d
140a88952:     	cmpl	%r9d, %r10d
140a88955:     	cmovlel	%r9d, %ecx
140a88959:     	movl	$0x4, %r9d
140a8895f:     	cmovgl	%r8d, %r9d
140a88963:     	movl	%ecx, %r8d
140a88966:     	sarl	$0x1f, %r8d
140a8896a:     	shrl	%r8d
140a8896d:     	xorl	%ecx, %r8d
140a88970:     	cmpl	%edx, %r8d
140a88973:     	cmovlel	%edx, %ecx
140a88976:     	movl	$0x5, %edx
140a8897b:     	cmovgl	%r9d, %edx
140a8897f:     	movl	%ecx, %r8d
140a88982:     	sarl	$0x1f, %r8d
140a88986:     	shrl	%r8d
140a88989:     	xorl	%ecx, %r8d
140a8898c:     	cmpl	%eax, %r8d
140a8898f:     	movl	$0x6, %eax
140a88994:     	cmovgl	%edx, %eax
140a88997:     	movaps	(%rsp), %xmm6
140a8899b:     	movaps	0x10(%rsp), %xmm7
140a889a0:     	addq	$0x20, %rsp
140a889a4:     	popq	%rsi
140a889a5:     	retq
140a889a6:     	int3
140a889a7:     	int3
140a889a8:     	int3
140a889a9:     	int3
140a889aa:     	int3
140a889ab:     	int3
140a889ac:     	int3
140a889ad:     	int3
140a889ae:     	int3
140a889af:     	int3
140a889b0:     	pushq	%r15
140a889b2:     	pushq	%r14
140a889b4:     	pushq	%r13
140a889b6:     	pushq	%r12
140a889b8:     	pushq	%rsi
140a889b9:     	pushq	%rdi
140a889ba:     	pushq	%rbp
140a889bb:     	pushq	%rbx
140a889bc:     	subq	$0x118, %rsp            # imm = 0x118
140a889c3:     	movaps	%xmm11, 0x100(%rsp)
140a889cc:     	movaps	%xmm10, 0xf0(%rsp)
140a889d5:     	movaps	%xmm9, 0xe0(%rsp)
140a889de:     	movaps	%xmm8, 0xd0(%rsp)
140a889e7:     	movaps	%xmm7, 0xc0(%rsp)
140a889ef:     	movaps	%xmm6, 0xb0(%rsp)
140a889f7:     	movq	%r9, %rbx
140a889fa:     	movq	%r8, %rsi
140a889fd:     	movq	(%rcx), %r15
