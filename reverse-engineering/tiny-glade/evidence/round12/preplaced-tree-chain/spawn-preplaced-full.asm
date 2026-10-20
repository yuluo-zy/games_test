
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140862270:     	pushq	%rbp
140862271:     	pushq	%r15
140862273:     	pushq	%r14
140862275:     	pushq	%r13
140862277:     	pushq	%r12
140862279:     	pushq	%rsi
14086227a:     	pushq	%rdi
14086227b:     	pushq	%rbx
14086227c:     	subq	$0x328, %rsp            # imm = 0x328
140862283:     	leaq	0x80(%rsp), %rbp
14086228b:     	movaps	%xmm14, 0x290(%rbp)
140862293:     	movaps	%xmm13, 0x280(%rbp)
14086229b:     	movaps	%xmm12, 0x270(%rbp)
1408622a3:     	movaps	%xmm11, 0x260(%rbp)
1408622ab:     	movaps	%xmm10, 0x250(%rbp)
1408622b3:     	movaps	%xmm9, 0x240(%rbp)
1408622bb:     	movaps	%xmm8, 0x230(%rbp)
1408622c3:     	movdqa	%xmm7, 0x220(%rbp)
1408622cb:     	movdqa	%xmm6, 0x210(%rbp)
1408622d3:     	movq	$-0x2, 0x208(%rbp)
1408622de:     	movq	%r9, %rbx
1408622e1:     	movq	%rdx, %r14
1408622e4:     	movq	(%r8), %rax
1408622e7:     	movq	0x8(%r8), %rdx
1408622eb:     	movq	(%rax), %r8
1408622ee:     	xorl	%r9d, %r9d
1408622f1:     	movq	%r8, %r10
1408622f4:     	subq	0x18(%rdx), %r10
1408622f8:     	cmovbq	%r9, %r10
1408622fc:     	subq	0x38(%rdx), %r8
140862300:     	cmovbq	%r9, %r8
140862304:     	movq	0x30(%rdx), %r11
140862308:     	subq	%r8, %r11
14086230b:     	cmovbq	%r9, %r11
14086230f:     	seta	%r8b
140862313:     	movq	0x10(%rdx), %rsi
140862317:     	subq	%r10, %rsi
14086231a:     	cmovbq	%r9, %rsi
14086231e:     	movq	0x40(%rdx), %rdx
140862322:     	seta	%r9b
140862326:     	addq	%r11, %rsi
140862329:     	movq	%rdx, %r10
14086232c:     	subq	%rsi, %r10
14086232f:     	movq	%r10, (%rax)
140862332:     	orb	%r8b, %r9b
140862335:     	je	0x140862de8 <.text+0x861de8>
14086233b:     	movq	%rdx, (%rax)
14086233e:     	movq	0x2dca063(%rip), %rax   # 0x14362c3a8
140862345:     	movq	(%rax), %rax
140862348:     	cmpq	$0x2, %rax
14086234c:     	jbe	0x140862415 <.text+0x861415>
140862352:     	leaq	0x2233de7(%rip), %rax   # 0x142a96140
140862359:     	movq	%rcx, %rsi
14086235c:     	movq	%rax, %rcx
14086235f:     	callq	0x14067dd80 <.text+0x67cd80>
140862364:     	movdqu	(%rax), %xmm0
140862368:     	movl	0x10(%rax), %eax
14086236b:     	movq	$0x3, 0x108(%rbp)
140862376:     	leaq	0x2233ddb(%rip), %rcx   # 0x142a96158
14086237d:     	movq	%rcx, 0x110(%rbp)
140862384:     	movq	$0x2c, 0x118(%rbp)
14086238f:     	leaq	0x2233d62(%rip), %rdx   # 0x142a960f8
140862396:     	movq	%rdx, 0x128(%rbp)
14086239d:     	movq	$0x1, 0x130(%rbp)
1408623a8:     	movq	$0x8, 0x138(%rbp)
1408623b3:     	movq	$0x0, 0xd8(%rbp)
1408623be:     	pxor	%xmm1, %xmm1
1408623c2:     	movdqu	%xmm1, 0x140(%rbp)
1408623ca:     	movq	%rcx, 0xe0(%rbp)
1408623d1:     	movq	$0x2c, 0xe8(%rbp)
1408623dc:     	movq	$0x0, 0xf0(%rbp)
1408623e7:     	movdqu	%xmm0, 0xf8(%rbp)
1408623ef:     	movl	$0x1, 0x120(%rbp)
1408623f9:     	movl	%eax, 0x124(%rbp)
1408623ff:     	leaq	0x158(%rbp), %rcx
140862406:     	leaq	0xd8(%rbp), %rdx
14086240d:     	callq	0x140d4a330 <.text+0xd49330>
140862412:     	movq	%rsi, %rcx
140862415:     	movq	(%rcx), %rax
140862418:     	movq	%rax, 0x18(%rbp)
14086241c:     	movzbl	0x58(%rax), %eax
140862420:     	shll	$0x3, %eax
140862423:     	leaq	0x2234266(%rip), %rcx   # 0x142a96690
14086242a:     	movq	(%rax,%rcx), %r9
14086242e:     	leaq	0x2234283(%rip), %rcx   # 0x142a966b8
140862435:     	movq	(%rax,%rcx), %rax
140862439:     	movq	%rax, 0x20(%rsp)
14086243e:     	leaq	0x228d0d5(%rip), %rdx   # 0x142aef51a
140862445:     	leaq	0xd8(%rbp), %rdi
14086244c:     	movl	$0xc, %r8d
140862452:     	movq	%rdi, %rcx
140862455:     	callq	0x140d62810 <.text+0xd61810>
14086245a:     	movq	%rdi, %rcx
14086245d:     	callq	0x140d76ed0 <.text+0xd75ed0>
140862462:     	movq	%rdx, %r8
140862465:     	movq	$0xa, 0x20(%rsp)
14086246e:     	leaq	0x2233d0f(%rip), %r9    # 0x142a96184
140862475:     	leaq	0x158(%rbp), %rcx
14086247c:     	movq	%rax, %rdx
14086247f:     	callq	0x140d62810 <.text+0xd61810>
140862484:     	leaq	0x158(%rbp), %rcx
14086248b:     	callq	0x140d76ed0 <.text+0xd75ed0>
140862490:     	movq	%rdx, %r8
140862493:     	leaq	0x50(%rbp), %rcx
140862497:     	movq	%rax, %rdx
14086249a:     	callq	0x140d59140 <.text+0xd58140>
14086249f:     	movq	0x158(%rbp), %rdx
1408624a6:     	testq	%rdx, %rdx
1408624a9:     	je	0x1408624bd <.text+0x8614bd>
1408624ab:     	movq	0x160(%rbp), %rcx
1408624b2:     	movl	$0x1, %r8d
1408624b8:     	callq	0x140613c20 <.text+0x612c20>
1408624bd:     	movq	0x50(%rbp), %rax
1408624c1:     	movq	%rax, 0x200(%rbp)
1408624c8:     	negq	%rax
1408624cb:     	jo	0x140862e5f <.text+0x861e5f>
1408624d1:     	movq	0x58(%rbp), %rdx
1408624d5:     	movq	0x60(%rbp), %r8
1408624d9:     	leaq	0x158(%rbp), %rcx
1408624e0:     	movq	%rdx, 0x1e8(%rbp)
1408624e7:     	callq	0x1414062b0 <.text+0x14052b0>
1408624ec:     	movq	0x158(%rbp), %rsi
1408624f3:     	movq	%rsi, %rax
1408624f6:     	negq	%rax
1408624f9:     	jo	0x140862e9a <.text+0x861e9a>
1408624ff:     	movq	%r14, 0x20(%rbp)
140862503:     	movq	0x160(%rbp), %r14
14086250a:     	movq	0x168(%rbp), %rdi
140862511:     	movq	0x200(%rbp), %rdx
140862518:     	testq	%rdx, %rdx
14086251b:     	je	0x14086252f <.text+0x86152f>
14086251d:     	movl	$0x1, %r8d
140862523:     	movq	0x1e8(%rbp), %rcx
14086252a:     	callq	0x140613c20 <.text+0x612c20>
14086252f:     	movq	0x310(%rbp), %r15
140862536:     	movq	0xd8(%rbp), %rdx
14086253d:     	testq	%rdx, %rdx
140862540:     	je	0x140862554 <.text+0x861554>
140862542:     	movq	0xe0(%rbp), %rcx
140862549:     	movl	$0x1, %r8d
14086254f:     	callq	0x140613c20 <.text+0x612c20>
140862554:     	leaq	(%rdi,%rdi,4), %rdi
140862558:     	shlq	$0x4, %rdi
14086255c:     	addq	%r14, %rdi
14086255f:     	movq	%r14, 0xa0(%rbp)
140862566:     	movq	%rsi, 0xb0(%rbp)
14086256d:     	movq	%rdi, 0xb8(%rbp)
140862574:     	leaq	0x160(%rbp), %rsi
14086257b:     	movl	0x1c(%rbx), %eax
14086257e:     	movl	%eax, 0x1ac(%rbp)
140862584:     	movq	(%rbx), %rax
140862587:     	movq	%rax, 0x1e8(%rbp)
14086258e:     	movq	0x10(%rbx), %rax
140862592:     	movq	%rax, 0x30(%rbp)
140862596:     	movl	0x1c(%r15), %eax
14086259a:     	movl	%eax, 0x1a8(%rbp)
1408625a0:     	movq	(%r15), %rax
1408625a3:     	movq	0x10(%r15), %rcx
1408625a7:     	movq	%rcx, 0x28(%rbp)
1408625ab:     	movq	%rax, 0x200(%rbp)
1408625b2:     	addq	$0x20, %rax
1408625b6:     	movq	%rax, 0x40(%rbp)
1408625ba:     	xorl	%r13d, %r13d
1408625bd:     	movdqa	0x22337bb(%rip), %xmm6  # 0x142a95d80
1408625c5:     	movd	0x22337c3(%rip), %xmm7  # 0x142a95d90
1408625cd:     	leaq	0x228cf46(%rip), %r15   # 0x142aef51a
1408625d4:     	movaps	0x20cc3a4(%rip), %xmm8  # 0x14292e980
1408625dc:     	movss	0x222122f(%rip), %xmm9  # 0x142a83814
1408625e5:     	movsd	0x22337b2(%rip), %xmm10 # 0x142a95da0
1408625ee:     	movsd	0x22337b9(%rip), %xmm11 # 0x142a95db0
1408625f7:     	xorps	%xmm12, %xmm12
1408625fb:     	cmpq	%rdi, %r14
1408625fe:     	je	0x140862dd2 <.text+0x861dd2>
140862604:     	addq	$0x50, %r14
140862608:     	jmp	0x140862625 <.text+0x861625>
14086260a:     	nopw	(%rax,%rax)
140862610:     	leaq	-0x50(%rbx), %rax
140862614:     	leaq	0x50(%rbx), %r14
140862618:     	addq	$0x50, %rax
14086261c:     	cmpq	%rdi, %rax
14086261f:     	je	0x140862dd5 <.text+0x861dd5>
140862625:     	movq	%r14, %rbx
140862628:     	movq	-0x50(%r14), %rdx
14086262c:     	movq	-0x8(%r14), %rax
140862630:     	movq	%rax, 0x90(%rbp)
140862637:     	movups	-0x18(%r14), %xmm0
14086263c:     	movaps	%xmm0, 0x80(%rbp)
140862643:     	movups	-0x28(%r14), %xmm0
140862648:     	movaps	%xmm0, 0x70(%rbp)
14086264c:     	movups	-0x38(%r14), %xmm0
140862651:     	movaps	%xmm0, 0x60(%rbp)
140862655:     	movups	-0x48(%r14), %xmm0
14086265a:     	movaps	%xmm0, 0x50(%rbp)
14086265e:     	movq	%rdx, %rax
140862661:     	negq	%rax
140862664:     	jo	0x140862dd5 <.text+0x861dd5>
14086266a:     	movq	%rdx, 0x158(%rbp)
140862671:     	movq	0x90(%rbp), %rax
140862678:     	movq	%rax, 0x40(%rsi)
14086267c:     	movdqa	0x50(%rbp), %xmm0
140862681:     	movdqa	0x60(%rbp), %xmm1
140862686:     	movaps	0x70(%rbp), %xmm2
14086268a:     	movaps	0x80(%rbp), %xmm3
140862691:     	movups	%xmm3, 0x30(%rsi)
140862695:     	movups	%xmm2, 0x20(%rsi)
140862699:     	movdqu	%xmm1, 0x10(%rsi)
14086269e:     	movdqu	%xmm0, (%rsi)
1408626a2:     	movq	0x160(%rbp), %rcx
1408626a9:     	movq	0x168(%rbp), %r14
1408626b0:     	cmpq	$0x12, %r14
1408626b4:     	jne	0x140862740 <.text+0x861740>
1408626ba:     	movdqu	(%rcx), %xmm0
1408626be:     	pcmpeqb	%xmm6, %xmm0
1408626c2:     	movzwl	0x10(%rcx), %eax
1408626c6:     	movd	%eax, %xmm1
1408626ca:     	pcmpeqb	%xmm7, %xmm1
1408626ce:     	pand	%xmm0, %xmm1
1408626d2:     	pmovmskb	%xmm1, %eax
1408626d6:     	cmpl	$0xffff, %eax           # imm = 0xFFFF
1408626db:     	jne	0x140862740 <.text+0x861740>
1408626dd:     	testq	%rdx, %rdx
1408626e0:     	je	0x1408626ed <.text+0x8616ed>
1408626e2:     	movl	$0x1, %r8d
1408626e8:     	callq	0x140613c20 <.text+0x612c20>
1408626ed:     	movq	0x170(%rbp), %rdx
1408626f4:     	testq	%rdx, %rdx
1408626f7:     	je	0x14086270b <.text+0x86170b>
1408626f9:     	movq	0x178(%rbp), %rcx
140862700:     	movl	$0x1, %r8d
140862706:     	callq	0x140613c20 <.text+0x612c20>
14086270b:     	movq	0x188(%rbp), %rdx
140862712:     	movq	%rdx, %rax
140862715:     	negq	%rax
140862718:     	jo	0x140862610 <.text+0x861610>
14086271e:     	jae	0x140862610 <.text+0x861610>
140862724:     	movq	0x190(%rbp), %rcx
14086272b:     	movl	$0x1, %r8d
140862731:     	callq	0x140613c20 <.text+0x612c20>
140862736:     	jmp	0x140862610 <.text+0x861610>
14086273b:     	nopl	(%rax,%rax)
140862740:     	movq	%r15, %rdx
140862743:     	movq	%r13, %r15
140862746:     	movq	%rcx, 0x1b8(%rbp)
14086274d:     	movq	%rbx, 0xa8(%rbp)
140862754:     	movq	0x178(%rbp), %rbx
14086275b:     	movq	0x180(%rbp), %r12
140862762:     	movq	0x18(%rbp), %rax
140862766:     	movzbl	0x58(%rax), %eax
14086276a:     	shll	$0x3, %eax
14086276d:     	leaq	0x2233f1c(%rip), %rcx   # 0x142a96690
140862774:     	movq	(%rax,%rcx), %r9
140862778:     	leaq	0x2233f39(%rip), %rcx   # 0x142a966b8
14086277f:     	movq	(%rax,%rcx), %rax
140862783:     	movq	%rax, 0x20(%rsp)
140862788:     	movl	$0xc, %r8d
14086278e:     	leaq	0xd8(%rbp), %rcx
140862795:     	movq	%rcx, %rdi
140862798:     	callq	0x140d62810 <.text+0xd61810>
14086279d:     	leaq	-0x50(%rbp), %r13
1408627a1:     	movq	%r13, %rcx
1408627a4:     	movq	%rbx, %rdx
1408627a7:     	movq	%r12, %r8
1408627aa:     	movq	%rdi, %r9
1408627ad:     	callq	0x1412e68e0 <.text+0x12e58e0>
1408627b2:     	movq	%r13, 0x10(%rbp)
1408627b6:     	movq	%r13, %rcx
1408627b9:     	callq	0x140d76ed0 <.text+0xd75ed0>
1408627be:     	movq	%rdx, %r8
1408627c1:     	leaq	0x1c0(%rbp), %rcx
1408627c8:     	movq	%rax, %rdx
1408627cb:     	callq	0x140d59140 <.text+0xd58140>
1408627d0:     	movq	0x1c0(%rbp), %rax
1408627d7:     	movq	0x1c8(%rbp), %rdx
1408627de:     	movq	%rax, 0xc0(%rbp)
1408627e5:     	negq	%rax
1408627e8:     	jo	0x140862f0d <.text+0x861f0d>
1408627ee:     	movq	%r15, %r13
1408627f1:     	movq	0x1d0(%rbp), %r8
1408627f8:     	movq	%rdi, %rcx
1408627fb:     	movq	%rdx, 0xd0(%rbp)
140862802:     	callq	0x141405cf0 <.text+0x1404cf0>
140862807:     	movq	0xd8(%rbp), %rax
14086280e:     	movq	%rax, 0xc8(%rbp)
140862815:     	negq	%rax
140862818:     	movq	0x1b8(%rbp), %rcx
14086281f:     	jo	0x140862ed2 <.text+0x861ed2>
140862825:     	movq	0xe0(%rbp), %rax
14086282c:     	movq	%rax, 0x1b0(%rbp)
140862833:     	movq	0xe8(%rbp), %rdi
14086283a:     	movq	%r14, %rdx
14086283d:     	callq	0x141556080 <.text+0x1555080>
140862842:     	movq	%rax, 0x1f0(%rbp)
140862849:     	movq	%rdx, 0x1f8(%rbp)
140862850:     	movl	0x1a0(%rbp), %eax
140862856:     	movl	%eax, 0x1b8(%rbp)
14086285c:     	leaq	(%rdi,%rdi,2), %rdi
140862860:     	shlq	$0x4, %rdi
140862864:     	movq	0x1b0(%rbp), %r15
14086286b:     	addq	%r15, %rdi
14086286e:     	nop
140862870:     	cmpq	%rdi, %r15
140862873:     	je	0x140862ce0 <.text+0x861ce0>
140862879:     	movq	%r15, %rax
14086287c:     	addq	$0x30, %r15
140862880:     	movss	(%rax), %xmm0
140862884:     	andps	%xmm8, %xmm0
140862888:     	ucomiss	%xmm0, %xmm9
14086288c:     	jbe	0x140862870 <.text+0x861870>
14086288e:     	movss	0x8(%rax), %xmm0
140862893:     	andps	%xmm8, %xmm0
140862897:     	ucomiss	%xmm0, %xmm9
14086289b:     	jbe	0x140862870 <.text+0x861870>
14086289d:     	leaq	0xd8(%rbp), %rcx
1408628a4:     	movq	0x20(%rbp), %rdx
1408628a8:     	callq	0x140d146a0 <.text+0xd136a0>
1408628ad:     	movq	0xe0(%rbp), %rbx
1408628b4:     	testq	%rbx, %rbx
1408628b7:     	jne	0x1408628c0 <.text+0x8618c0>
1408628b9:     	movq	0xe8(%rbp), %rbx
1408628c0:     	movq	0xf8(%rbp), %r14
1408628c7:     	movq	(%rbx), %rax
1408628ca:     	movq	0x10(%rbx), %r12
1408628ce:     	subq	%r12, %rax
1408628d1:     	cmpq	$0x2f, %rax
1408628d5:     	jbe	0x140862c4c <.text+0x861c4c>
1408628db:     	leaq	0x186a1(%r13), %rdx
1408628e2:     	movq	0x8(%rbx), %rax
1408628e6:     	leaq	0xa74c03(%rip), %rcx    # 0x1412d74f0 <.text+0x12d64f0>
1408628ed:     	movq	%rcx, (%rax,%r12)
1408628f1:     	leaq	0xd02438(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1408628f8:     	movq	%rcx, 0x8(%rax,%r12)
1408628fd:     	movq	%r14, 0x10(%rax,%r12)
140862902:     	movq	%rdx, 0x38(%rbp)
140862906:     	movq	%rdx, 0x18(%rax,%r12)
14086290b:     	movb	$0x1, 0x20(%rax,%r12)
140862911:     	movb	$0x0, 0x28(%rax,%r12)
140862917:     	addq	$0x30, %r12
14086291b:     	movq	%r12, 0x10(%rbx)
14086291f:     	movq	0xf8(%rbp), %rax
140862926:     	movq	%rax, 0x1e0(%rbp)
14086292d:     	movups	0xd8(%rbp), %xmm0
140862934:     	movdqu	0xe8(%rbp), %xmm1
14086293c:     	movdqa	%xmm1, 0x1d0(%rbp)
140862944:     	movaps	%xmm0, 0x1c0(%rbp)
14086294b:     	movq	0x1f0(%rbp), %rdx
140862952:     	movq	0x1f8(%rbp), %r8
140862959:     	lock
14086295a:     	incq	(%rdx)
14086295d:     	jle	0x140862f7a <.text+0x861f7a>
140862963:     	leaq	0x1c0(%rbp), %rcx
14086296a:     	leaq	0x223387f(%rip), %r9    # 0x142a961f0
140862971:     	callq	0x1412deaf0 <.text+0x12ddaf0>
140862976:     	movq	0x1c8(%rbp), %rbx
14086297d:     	testq	%rbx, %rbx
140862980:     	jne	0x140862989 <.text+0x861989>
140862982:     	movq	0x1d0(%rbp), %rbx
140862989:     	movq	0x1e0(%rbp), %r14
140862990:     	movq	(%rbx), %rax
140862993:     	movq	0x10(%rbx), %r12
140862997:     	subq	%r12, %rax
14086299a:     	cmpq	$0x2f, %rax
14086299e:     	jbe	0x140862c71 <.text+0x861c71>
1408629a4:     	movq	0x8(%rbx), %rax
1408629a8:     	leaq	0xa73e21(%rip), %rcx    # 0x1412d67d0 <.text+0x12d57d0>
1408629af:     	movq	%rcx, (%rax,%r12)
1408629b3:     	leaq	0xd02376(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1408629ba:     	movq	%rcx, 0x8(%rax,%r12)
1408629bf:     	movq	%r14, 0x10(%rax,%r12)
1408629c4:     	movsd	%xmm10, 0x18(%rax,%r12)
1408629cb:     	movl	$0x3f800000, 0x20(%rax,%r12) # imm = 0x3F800000
1408629d4:     	movb	$0x0, 0x24(%rax,%r12)
1408629da:     	movb	$0x0, 0x28(%rax,%r12)
1408629e0:     	addq	$0x30, %r12
1408629e4:     	movq	%r12, 0x10(%rbx)
1408629e8:     	movq	0x1c8(%rbp), %rbx
1408629ef:     	testq	%rbx, %rbx
1408629f2:     	jne	0x1408629fb <.text+0x8619fb>
1408629f4:     	movq	0x1d0(%rbp), %rbx
1408629fb:     	movss	-0x30(%r15), %xmm13
140862a01:     	movss	-0x28(%r15), %xmm14
140862a07:     	movq	0x1e0(%rbp), %r14
140862a0e:     	movq	(%rbx), %rax
140862a11:     	movq	0x10(%rbx), %r12
140862a15:     	subq	%r12, %rax
140862a18:     	cmpq	$0x27, %rax
140862a1c:     	jbe	0x140862c96 <.text+0x861c96>
140862a22:     	movq	0x8(%rbx), %rax
140862a26:     	leaq	0xa78773(%rip), %rcx    # 0x1412db1a0 <.text+0x12da1a0>
140862a2d:     	movq	%rcx, (%rax,%r12)
140862a31:     	leaq	0xd022f8(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140862a38:     	movq	%rcx, 0x8(%rax,%r12)
140862a3d:     	movq	%r14, 0x10(%rax,%r12)
140862a42:     	movss	%xmm13, 0x18(%rax,%r12)
140862a49:     	movss	%xmm14, 0x1c(%rax,%r12)
140862a50:     	movb	$0x0, 0x20(%rax,%r12)
140862a56:     	addq	$0x28, %r12
140862a5a:     	movq	%r12, 0x10(%rbx)
140862a5e:     	movsd	-0x30(%r15), %xmm13
140862a64:     	addps	%xmm11, %xmm13
140862a68:     	movq	0x1c8(%rbp), %rbx
140862a6f:     	testq	%rbx, %rbx
140862a72:     	jne	0x140862a7b <.text+0x861a7b>
140862a74:     	movq	0x1d0(%rbp), %rbx
140862a7b:     	movss	-0x28(%r15), %xmm14
140862a81:     	addss	%xmm12, %xmm14
140862a86:     	movq	0x1e0(%rbp), %r14
140862a8d:     	movq	(%rbx), %rax
140862a90:     	movq	0x10(%rbx), %r12
140862a94:     	subq	%r12, %rax
140862a97:     	cmpq	$0x27, %rax
140862a9b:     	jbe	0x140862cbb <.text+0x861cbb>
140862aa1:     	leaq	-0x30(%r15), %rax
140862aa5:     	movq	0x8(%rbx), %rcx
140862aa9:     	leaq	0xa76b00(%rip), %rdx    # 0x1412d95b0 <.text+0x12d85b0>
140862ab0:     	movq	%rdx, (%rcx,%r12)
140862ab4:     	leaq	0xd02275(%rip), %rdx    # 0x141564d30 <.text+0x1563d30>
140862abb:     	movq	%rdx, 0x8(%rcx,%r12)
140862ac0:     	movq	%r14, 0x10(%rcx,%r12)
140862ac5:     	movlps	%xmm13, 0x18(%rcx,%r12)
140862acb:     	movss	%xmm14, 0x20(%rcx,%r12)
140862ad2:     	movb	$0x0, 0x24(%rcx,%r12)
140862ad8:     	addq	$0x28, %r12
140862adc:     	movq	%r12, 0x10(%rbx)
140862ae0:     	movq	0x1e0(%rbp), %rbx
140862ae7:     	movups	(%rax), %xmm0
140862aea:     	movaps	%xmm0, -0x30(%rbp)
140862aee:     	movq	0x10(%rax), %rax
140862af2:     	movq	%rax, -0x20(%rbp)
140862af6:     	movups	-0x14(%r15), %xmm0
140862afb:     	movaps	%xmm0, -0x10(%rbp)
140862aff:     	movl	-0x4(%r15), %eax
140862b03:     	movl	%eax, (%rbp)
140862b06:     	movq	0x1f0(%rbp), %r8
140862b0d:     	movq	0x1f8(%rbp), %rax
140862b14:     	lock
140862b15:     	incq	(%r8)
140862b18:     	jle	0x140862f7a <.text+0x861f7a>
140862b1e:     	movl	0x1a8(%rbp), %ecx
140862b24:     	movq	0x28(%rbp), %rdx
140862b28:     	movl	%ecx, (%rdx)
140862b2a:     	movq	0x200(%rbp), %rdx
140862b31:     	movq	0x40(%rdx), %rcx
140862b35:     	movq	%rcx, 0x120(%rbp)
140862b3c:     	movq	%r8, 0x48(%rbp)
140862b40:     	movq	%r8, 0xd8(%rbp)
140862b47:     	movq	%rax, 0xe0(%rbp)
140862b4e:     	movq	%rbx, 0xe8(%rbp)
140862b55:     	movq	-0x20(%rbp), %rax
140862b59:     	leaq	0xf0(%rbp), %rcx
140862b60:     	movq	%rax, 0x10(%rcx)
140862b64:     	movaps	-0x30(%rbp), %xmm0
140862b68:     	movups	%xmm0, (%rcx)
140862b6b:     	movl	0x1b8(%rbp), %eax
140862b71:     	movl	%eax, 0x108(%rbp)
140862b77:     	movl	(%rbp), %eax
140862b7a:     	movl	%eax, 0x2c(%rcx)
140862b7d:     	movaps	-0x10(%rbp), %xmm0
140862b81:     	movups	%xmm0, 0x1c(%rcx)
140862b85:     	movq	0x30(%rdx), %r12
140862b89:     	cmpq	0x20(%rdx), %r12
140862b8d:     	jne	0x140862b9f <.text+0x861b9f>
140862b8f:     	movq	0x40(%rbp), %rcx
140862b93:     	leaq	0x23d75ae(%rip), %rdx   # 0x142c3a148
140862b9a:     	callq	0x140563540 <.text+0x562540>
140862b9f:     	movq	0x200(%rbp), %rdx
140862ba6:     	movq	0x28(%rdx), %rax
140862baa:     	leaq	(%r12,%r12,4), %rcx
140862bae:     	shlq	$0x4, %rcx
140862bb2:     	movups	0xd8(%rbp), %xmm0
140862bb9:     	movdqu	0xe8(%rbp), %xmm1
140862bc1:     	movups	0xf8(%rbp), %xmm2
140862bc8:     	movups	0x108(%rbp), %xmm3
140862bcf:     	movups	0x118(%rbp), %xmm4
140862bd6:     	movups	%xmm4, 0x40(%rax,%rcx)
140862bdb:     	movups	%xmm3, 0x30(%rax,%rcx)
140862be0:     	movups	%xmm2, 0x20(%rax,%rcx)
140862be5:     	movdqu	%xmm1, 0x10(%rax,%rcx)
140862beb:     	movups	%xmm0, (%rax,%rcx)
140862bef:     	incq	%r12
140862bf2:     	movq	%r12, 0x30(%rdx)
140862bf6:     	incq	0x40(%rdx)
140862bfa:     	movl	0x1ac(%rbp), %eax
140862c00:     	movq	0x30(%rbp), %rcx
140862c04:     	movl	%eax, (%rcx)
140862c06:     	movq	0x1e8(%rbp), %rcx
140862c0d:     	movq	0x38(%rbp), %rdx
140862c11:     	movq	%rbx, %r8
140862c14:     	callq	0x140eec970 <.text+0xeeb970>
140862c19:     	leaq	0x1(%r13), %rcx
140862c1d:     	movq	0x1e8(%rbp), %r8
140862c24:     	movq	0x30(%r8), %rdx
140862c28:     	addq	$0x186a2, %r13          # imm = 0x186A2
140862c2f:     	cmpq	%rdx, %r13
140862c32:     	cmovbeq	%rdx, %r13
140862c36:     	movq	%r13, 0x30(%r8)
140862c3a:     	shrq	$0x20, %rax
140862c3e:     	movq	%rcx, %r13
140862c41:     	je	0x140862870 <.text+0x861870>
140862c47:     	jmp	0x140862e42 <.text+0x861e42>
140862c4c:     	movq	$0x1, 0x20(%rsp)
140862c55:     	movl	$0x30, %r8d
140862c5b:     	movl	$0x1, %r9d
140862c61:     	movq	%rbx, %rcx
140862c64:     	movq	%r12, %rdx
140862c67:     	callq	0x1428a9a60 <.text+0x28a8a60>
140862c6c:     	jmp	0x1408628db <.text+0x8618db>
140862c71:     	movq	$0x1, 0x20(%rsp)
140862c7a:     	movl	$0x30, %r8d
140862c80:     	movl	$0x1, %r9d
140862c86:     	movq	%rbx, %rcx
140862c89:     	movq	%r12, %rdx
140862c8c:     	callq	0x1428a9a60 <.text+0x28a8a60>
140862c91:     	jmp	0x1408629a4 <.text+0x8619a4>
140862c96:     	movq	$0x1, 0x20(%rsp)
140862c9f:     	movl	$0x28, %r8d
140862ca5:     	movl	$0x1, %r9d
140862cab:     	movq	%rbx, %rcx
140862cae:     	movq	%r12, %rdx
140862cb1:     	callq	0x1428a9a60 <.text+0x28a8a60>
140862cb6:     	jmp	0x140862a22 <.text+0x861a22>
140862cbb:     	movq	$0x1, 0x20(%rsp)
140862cc4:     	movl	$0x28, %r8d
140862cca:     	movl	$0x1, %r9d
140862cd0:     	movq	%rbx, %rcx
140862cd3:     	movq	%r12, %rdx
140862cd6:     	callq	0x1428a9a60 <.text+0x28a8a60>
140862cdb:     	jmp	0x140862aa1 <.text+0x861aa1>
140862ce0:     	movq	0x1f0(%rbp), %rax
140862ce7:     	lock
140862ce8:     	decq	(%rax)
140862ceb:     	jne	0x140862cf9 <.text+0x861cf9>
140862ced:     	leaq	0x1f0(%rbp), %rcx
140862cf4:     	callq	0x14064eb30 <.text+0x64db30>
140862cf9:     	movq	0xc8(%rbp), %rax
140862d00:     	testq	%rax, %rax
140862d03:     	leaq	0x228c810(%rip), %r15   # 0x142aef51a
140862d0a:     	je	0x140862d26 <.text+0x861d26>
140862d0c:     	shlq	$0x4, %rax
140862d10:     	leaq	(%rax,%rax,2), %rdx
140862d14:     	movl	$0x4, %r8d
140862d1a:     	movq	0x1b0(%rbp), %rcx
140862d21:     	callq	0x140613c20 <.text+0x612c20>
140862d26:     	movq	0xc0(%rbp), %rdx
140862d2d:     	testq	%rdx, %rdx
140862d30:     	je	0x140862d44 <.text+0x861d44>
140862d32:     	movl	$0x1, %r8d
140862d38:     	movq	0xd0(%rbp), %rcx
140862d3f:     	callq	0x140613c20 <.text+0x612c20>
140862d44:     	movq	-0x50(%rbp), %rdx
140862d48:     	testq	%rdx, %rdx
140862d4b:     	je	0x140862d5c <.text+0x861d5c>
140862d4d:     	movq	-0x48(%rbp), %rcx
140862d51:     	movl	$0x1, %r8d
140862d57:     	callq	0x140613c20 <.text+0x612c20>
140862d5c:     	movq	0x158(%rbp), %rdx
140862d63:     	testq	%rdx, %rdx
140862d66:     	je	0x140862d7a <.text+0x861d7a>
140862d68:     	movq	0x160(%rbp), %rcx
140862d6f:     	movl	$0x1, %r8d
140862d75:     	callq	0x140613c20 <.text+0x612c20>
140862d7a:     	movq	0x170(%rbp), %rdx
140862d81:     	testq	%rdx, %rdx
140862d84:     	je	0x140862d98 <.text+0x861d98>
140862d86:     	movq	0x178(%rbp), %rcx
140862d8d:     	movl	$0x1, %r8d
140862d93:     	callq	0x140613c20 <.text+0x612c20>
140862d98:     	movq	0x188(%rbp), %rdx
140862d9f:     	movq	%rdx, %rax
140862da2:     	negq	%rax
140862da5:     	jo	0x140862dbb <.text+0x861dbb>
140862da7:     	jae	0x140862dbb <.text+0x861dbb>
140862da9:     	movq	0x190(%rbp), %rcx
140862db0:     	movl	$0x1, %r8d
140862db6:     	callq	0x140613c20 <.text+0x612c20>
140862dbb:     	movq	0xa8(%rbp), %r14
140862dc2:     	movq	0xb8(%rbp), %rdi
140862dc9:     	cmpq	%rdi, %r14
140862dcc:     	jne	0x140862604 <.text+0x861604>
140862dd2:     	movq	%r14, %rbx
140862dd5:     	movq	%rbx, 0xa8(%rbp)
140862ddc:     	leaq	0xa0(%rbp), %rcx
140862de3:     	callq	0x140879230 <.text+0x878230>
140862de8:     	movaps	0x210(%rbp), %xmm6
140862def:     	movaps	0x220(%rbp), %xmm7
140862df6:     	movaps	0x230(%rbp), %xmm8
140862dfe:     	movaps	0x240(%rbp), %xmm9
140862e06:     	movaps	0x250(%rbp), %xmm10
140862e0e:     	movaps	0x260(%rbp), %xmm11
140862e16:     	movaps	0x270(%rbp), %xmm12
140862e1e:     	movaps	0x280(%rbp), %xmm13
140862e26:     	movaps	0x290(%rbp), %xmm14
140862e2e:     	addq	$0x328, %rsp            # imm = 0x328
140862e35:     	popq	%rbx
140862e36:     	popq	%rdi
140862e37:     	popq	%rsi
140862e38:     	popq	%r12
140862e3a:     	popq	%r13
140862e3c:     	popq	%r14
140862e3e:     	popq	%r15
140862e40:     	popq	%rbp
140862e41:     	retq
140862e42:     	leaq	0x22333bf(%rip), %rcx   # 0x142a96208
140862e49:     	leaq	0x22333d8(%rip), %r8    # 0x142a96228
140862e50:     	movl	$0x20, %edx
140862e55:     	callq	0x1428d9430 <.text+0x28d8430>
140862e5a:     	jmp	0x140862f7a <.text+0x861f7a>
140862e5f:     	movq	0x58(%rbp), %rax
140862e63:     	movq	%rax, 0x158(%rbp)
140862e6a:     	leaq	0x223331f(%rip), %rax   # 0x142a96190
140862e71:     	movq	%rax, 0x20(%rsp)
140862e76:     	leaq	0x2233307(%rip), %rcx   # 0x142a96184
140862e7d:     	leaq	0x223301c(%rip), %r9    # 0x142a95ea0
140862e84:     	leaq	0x158(%rbp), %r8
140862e8b:     	movl	$0xa, %edx
140862e90:     	callq	0x1428d9760 <.text+0x28d8760>
140862e95:     	jmp	0x140862f7a <.text+0x861f7a>
140862e9a:     	movq	0x160(%rbp), %rax
140862ea1:     	movq	%rax, 0x50(%rbp)
140862ea5:     	leaq	0x2233314(%rip), %rax   # 0x142a961c0
140862eac:     	movq	%rax, 0x20(%rsp)
140862eb1:     	leaq	0x22332f0(%rip), %rcx   # 0x142a961a8
140862eb8:     	leaq	0x2233001(%rip), %r9    # 0x142a95ec0
140862ebf:     	leaq	0x50(%rbp), %r8
140862ec3:     	movl	$0x11, %edx
140862ec8:     	callq	0x1428d9760 <.text+0x28d8760>
140862ecd:     	jmp	0x140862f7a <.text+0x861f7a>
140862ed2:     	movq	0xe0(%rbp), %rax
140862ed9:     	movq	%rax, 0x1c0(%rbp)
140862ee0:     	leaq	0x22332f1(%rip), %rax   # 0x142a961d8
140862ee7:     	movq	%rax, 0x20(%rsp)
140862eec:     	leaq	0x223300d(%rip), %rcx   # 0x142a95f00
140862ef3:     	leaq	0x2232fc6(%rip), %r9    # 0x142a95ec0
140862efa:     	leaq	0x1c0(%rbp), %r8
140862f01:     	movl	$0x2b, %edx
140862f06:     	callq	0x1428d9760 <.text+0x28d8760>
140862f0b:     	jmp	0x140862f7a <.text+0x861f7a>
140862f0d:     	movq	%rdx, 0x8(%rbp)
140862f11:     	leaq	0x10(%rbp), %rax
140862f15:     	movq	%rax, 0x1f0(%rbp)
140862f1c:     	leaq	-0x834633(%rip), %rax   # 0x14002e8f0 <.text+0x2d8f0>
140862f23:     	movq	%rax, 0x1f8(%rbp)
140862f2a:     	leaq	0x223331f(%rip), %rax   # 0x142a96250
140862f31:     	movq	%rax, 0xd8(%rbp)
140862f38:     	movq	$0x1, 0xe0(%rbp)
140862f43:     	movq	$0x0, 0xf8(%rbp)
140862f4e:     	leaq	0x1f0(%rbp), %rax
140862f55:     	movq	%rax, 0xe8(%rbp)
140862f5c:     	movq	$0x1, 0xf0(%rbp)
140862f67:     	leaq	0x22332f2(%rip), %rdx   # 0x142a96260
140862f6e:     	leaq	0xd8(%rbp), %rcx
140862f75:     	callq	0x1428d9390 <.text+0x28d8390>
140862f7a:     	ud2
140862f7c:     	nopl	(%rax)
140862f80:     	movq	%rdx, 0x10(%rsp)
140862f85:     	pushq	%rbp
140862f86:     	pushq	%r15
140862f88:     	pushq	%r14
140862f8a:     	pushq	%r13
140862f8c:     	pushq	%r12
140862f8e:     	pushq	%rsi
140862f8f:     	pushq	%rdi
140862f90:     	pushq	%rbx
140862f91:     	subq	$0xb8, %rsp
140862f98:     	leaq	0x80(%rdx), %rbp
140862f9f:     	movaps	%xmm14, 0x20(%rsp)
140862fa5:     	movaps	%xmm13, 0x30(%rsp)
140862fab:     	movaps	%xmm12, 0x40(%rsp)
140862fb1:     	movaps	%xmm11, 0x50(%rsp)
140862fb7:     	movaps	%xmm10, 0x60(%rsp)
140862fbd:     	movaps	%xmm9, 0x70(%rsp)
140862fc3:     	movaps	%xmm8, 0x80(%rsp)
140862fcc:     	movdqa	%xmm7, 0x90(%rsp)
140862fd5:     	movdqa	%xmm6, 0xa0(%rsp)
140862fde:     	movq	0xd8(%rbp), %rdx
140862fe5:     	testq	%rdx, %rdx
140862fe8:     	je	0x140862ffc <.text+0x861ffc>
140862fea:     	movq	0xe0(%rbp), %rcx
140862ff1:     	movl	$0x1, %r8d
140862ff7:     	callq	0x140613c20 <.text+0x612c20>
140862ffc:     	movaps	0xa0(%rsp), %xmm6
140863004:     	movaps	0x90(%rsp), %xmm7
14086300c:     	movaps	0x80(%rsp), %xmm8
140863015:     	movaps	0x70(%rsp), %xmm9
14086301b:     	movaps	0x60(%rsp), %xmm10
140863021:     	movaps	0x50(%rsp), %xmm11
140863027:     	movaps	0x40(%rsp), %xmm12
14086302d:     	movaps	0x30(%rsp), %xmm13
140863033:     	movaps	0x20(%rsp), %xmm14
140863039:     	addq	$0xb8, %rsp
140863040:     	popq	%rbx
140863041:     	popq	%rdi
140863042:     	popq	%rsi
140863043:     	popq	%r12
140863045:     	popq	%r13
140863047:     	popq	%r14
140863049:     	popq	%r15
14086304b:     	popq	%rbp
14086304c:     	retq
14086304d:     	nopl	(%rax)
140863050:     	movq	%rdx, 0x10(%rsp)
140863055:     	pushq	%rbp
140863056:     	pushq	%r15
140863058:     	pushq	%r14
14086305a:     	pushq	%r13
14086305c:     	pushq	%r12
14086305e:     	pushq	%rsi
14086305f:     	pushq	%rdi
140863060:     	pushq	%rbx
140863061:     	subq	$0xb8, %rsp
140863068:     	leaq	0x80(%rdx), %rbp
14086306f:     	movaps	%xmm14, 0x20(%rsp)
140863075:     	movaps	%xmm13, 0x30(%rsp)
14086307b:     	movaps	%xmm12, 0x40(%rsp)
140863081:     	movaps	%xmm11, 0x50(%rsp)
140863087:     	movaps	%xmm10, 0x60(%rsp)
14086308d:     	movaps	%xmm9, 0x70(%rsp)
140863093:     	movaps	%xmm8, 0x80(%rsp)
14086309c:     	movdqa	%xmm7, 0x90(%rsp)
1408630a5:     	movdqa	%xmm6, 0xa0(%rsp)
1408630ae:     	movq	0x158(%rbp), %rdx
1408630b5:     	testq	%rdx, %rdx
1408630b8:     	je	0x1408630cc <.text+0x8620cc>
1408630ba:     	movq	0x160(%rbp), %rcx
1408630c1:     	movl	$0x1, %r8d
1408630c7:     	callq	0x140613c20 <.text+0x612c20>
1408630cc:     	movdqa	0xa0(%rsp), %xmm6
1408630d5:     	movdqa	0x90(%rsp), %xmm7
1408630de:     	movaps	0x80(%rsp), %xmm8
1408630e7:     	movaps	0x70(%rsp), %xmm9
1408630ed:     	movaps	0x60(%rsp), %xmm10
1408630f3:     	movaps	0x50(%rsp), %xmm11
1408630f9:     	movaps	0x40(%rsp), %xmm12
1408630ff:     	movaps	0x30(%rsp), %xmm13
140863105:     	movaps	0x20(%rsp), %xmm14
14086310b:     	addq	$0xb8, %rsp
140863112:     	popq	%rbx
140863113:     	popq	%rdi
140863114:     	popq	%rsi
140863115:     	popq	%r12
140863117:     	popq	%r13
140863119:     	popq	%r14
14086311b:     	popq	%r15
14086311d:     	popq	%rbp
14086311e:     	retq
14086311f:     	nop
140863120:     	movq	%rdx, 0x10(%rsp)
140863125:     	pushq	%rbp
140863126:     	pushq	%r15
140863128:     	pushq	%r14
14086312a:     	pushq	%r13
14086312c:     	pushq	%r12
14086312e:     	pushq	%rsi
14086312f:     	pushq	%rdi
140863130:     	pushq	%rbx
140863131:     	subq	$0xb8, %rsp
140863138:     	leaq	0x80(%rdx), %rbp
14086313f:     	movaps	%xmm14, 0x20(%rsp)
140863145:     	movaps	%xmm13, 0x30(%rsp)
14086314b:     	movaps	%xmm12, 0x40(%rsp)
140863151:     	movaps	%xmm11, 0x50(%rsp)
140863157:     	movaps	%xmm10, 0x60(%rsp)
14086315d:     	movaps	%xmm9, 0x70(%rsp)
140863163:     	movaps	%xmm8, 0x80(%rsp)
14086316c:     	movdqa	%xmm7, 0x90(%rsp)
140863175:     	movdqa	%xmm6, 0xa0(%rsp)
14086317e:     	leaq	0x158(%rbp), %rcx
140863185:     	callq	0x1405e7a00 <.text+0x5e6a00>
14086318a:     	movdqa	0xa0(%rsp), %xmm6
140863193:     	movdqa	0x90(%rsp), %xmm7
14086319c:     	movaps	0x80(%rsp), %xmm8
1408631a5:     	movaps	0x70(%rsp), %xmm9
1408631ab:     	movaps	0x60(%rsp), %xmm10
1408631b1:     	movaps	0x50(%rsp), %xmm11
1408631b7:     	movaps	0x40(%rsp), %xmm12
1408631bd:     	movaps	0x30(%rsp), %xmm13
1408631c3:     	movaps	0x20(%rsp), %xmm14
1408631c9:     	addq	$0xb8, %rsp
1408631d0:     	popq	%rbx
1408631d1:     	popq	%rdi
1408631d2:     	popq	%rsi
1408631d3:     	popq	%r12
1408631d5:     	popq	%r13
1408631d7:     	popq	%r14
1408631d9:     	popq	%r15
1408631db:     	popq	%rbp
1408631dc:     	retq
1408631dd:     	nopl	(%rax)
1408631e0:     	movq	%rdx, 0x10(%rsp)
1408631e5:     	pushq	%rbp
1408631e6:     	pushq	%r15
1408631e8:     	pushq	%r14
1408631ea:     	pushq	%r13
1408631ec:     	pushq	%r12
1408631ee:     	pushq	%rsi
1408631ef:     	pushq	%rdi
1408631f0:     	pushq	%rbx
1408631f1:     	subq	$0xb8, %rsp
1408631f8:     	leaq	0x80(%rdx), %rbp
1408631ff:     	movaps	%xmm14, 0x20(%rsp)
140863205:     	movaps	%xmm13, 0x30(%rsp)
14086320b:     	movaps	%xmm12, 0x40(%rsp)
140863211:     	movaps	%xmm11, 0x50(%rsp)
140863217:     	movaps	%xmm10, 0x60(%rsp)
14086321d:     	movaps	%xmm9, 0x70(%rsp)
140863223:     	movaps	%xmm8, 0x80(%rsp)
14086322c:     	movdqa	%xmm7, 0x90(%rsp)
140863235:     	movdqa	%xmm6, 0xa0(%rsp)
14086323e:     	cmpq	$0x0, 0x200(%rbp)
140863246:     	je	0x140863261 <.text+0x862261>
140863248:     	movl	$0x1, %r8d
14086324e:     	movq	0x1e8(%rbp), %rcx
140863255:     	movq	0x200(%rbp), %rdx
14086325c:     	callq	0x140613c20 <.text+0x612c20>
140863261:     	movdqa	0xa0(%rsp), %xmm6
14086326a:     	movdqa	0x90(%rsp), %xmm7
140863273:     	movaps	0x80(%rsp), %xmm8
14086327c:     	movaps	0x70(%rsp), %xmm9
140863282:     	movaps	0x60(%rsp), %xmm10
140863288:     	movaps	0x50(%rsp), %xmm11
14086328e:     	movaps	0x40(%rsp), %xmm12
140863294:     	movaps	0x30(%rsp), %xmm13
14086329a:     	movaps	0x20(%rsp), %xmm14
1408632a0:     	addq	$0xb8, %rsp
1408632a7:     	popq	%rbx
1408632a8:     	popq	%rdi
1408632a9:     	popq	%rsi
1408632aa:     	popq	%r12
1408632ac:     	popq	%r13
1408632ae:     	popq	%r14
1408632b0:     	popq	%r15
1408632b2:     	popq	%rbp
1408632b3:     	retq
1408632b4:     	nopw	%cs:(%rax,%rax)
1408632c0:     	movq	%rdx, 0x10(%rsp)
1408632c5:     	pushq	%rbp
1408632c6:     	pushq	%r15
1408632c8:     	pushq	%r14
1408632ca:     	pushq	%r13
1408632cc:     	pushq	%r12
1408632ce:     	pushq	%rsi
1408632cf:     	pushq	%rdi
1408632d0:     	pushq	%rbx
1408632d1:     	subq	$0xb8, %rsp
1408632d8:     	leaq	0x80(%rdx), %rbp
1408632df:     	movaps	%xmm14, 0x20(%rsp)
1408632e5:     	movaps	%xmm13, 0x30(%rsp)
1408632eb:     	movaps	%xmm12, 0x40(%rsp)
1408632f1:     	movaps	%xmm11, 0x50(%rsp)
1408632f7:     	movaps	%xmm10, 0x60(%rsp)
1408632fd:     	movaps	%xmm9, 0x70(%rsp)
140863303:     	movaps	%xmm8, 0x80(%rsp)
14086330c:     	movdqa	%xmm7, 0x90(%rsp)
140863315:     	movdqa	%xmm6, 0xa0(%rsp)
14086331e:     	movq	-0x50(%rbp), %rdx
140863322:     	testq	%rdx, %rdx
140863325:     	je	0x140863336 <.text+0x862336>
140863327:     	movq	-0x48(%rbp), %rcx
14086332b:     	movl	$0x1, %r8d
140863331:     	callq	0x140613c20 <.text+0x612c20>
140863336:     	movdqa	0xa0(%rsp), %xmm6
14086333f:     	movdqa	0x90(%rsp), %xmm7
140863348:     	movaps	0x80(%rsp), %xmm8
140863351:     	movaps	0x70(%rsp), %xmm9
140863357:     	movaps	0x60(%rsp), %xmm10
14086335d:     	movaps	0x50(%rsp), %xmm11
140863363:     	movaps	0x40(%rsp), %xmm12
140863369:     	movaps	0x30(%rsp), %xmm13
14086336f:     	movaps	0x20(%rsp), %xmm14
140863375:     	addq	$0xb8, %rsp
14086337c:     	popq	%rbx
14086337d:     	popq	%rdi
14086337e:     	popq	%rsi
14086337f:     	popq	%r12
140863381:     	popq	%r13
140863383:     	popq	%r14
140863385:     	popq	%r15
140863387:     	popq	%rbp
140863388:     	retq
140863389:     	nopl	(%rax)
140863390:     	movq	%rdx, 0x10(%rsp)
140863395:     	pushq	%rbp
140863396:     	pushq	%r15
140863398:     	pushq	%r14
14086339a:     	pushq	%r13
14086339c:     	pushq	%r12
14086339e:     	pushq	%rsi
14086339f:     	pushq	%rdi
1408633a0:     	pushq	%rbx
1408633a1:     	subq	$0xb8, %rsp
1408633a8:     	leaq	0x80(%rdx), %rbp
1408633af:     	movaps	%xmm14, 0x20(%rsp)
1408633b5:     	movaps	%xmm13, 0x30(%rsp)
1408633bb:     	movaps	%xmm12, 0x40(%rsp)
1408633c1:     	movaps	%xmm11, 0x50(%rsp)
1408633c7:     	movaps	%xmm10, 0x60(%rsp)
1408633cd:     	movaps	%xmm9, 0x70(%rsp)
1408633d3:     	movaps	%xmm8, 0x80(%rsp)
1408633dc:     	movdqa	%xmm7, 0x90(%rsp)
1408633e5:     	movdqa	%xmm6, 0xa0(%rsp)
1408633ee:     	leaq	0x8(%rbp), %rcx
1408633f2:     	callq	0x1405e7a00 <.text+0x5e6a00>
1408633f7:     	movdqa	0xa0(%rsp), %xmm6
140863400:     	movdqa	0x90(%rsp), %xmm7
140863409:     	movaps	0x80(%rsp), %xmm8
140863412:     	movaps	0x70(%rsp), %xmm9
140863418:     	movaps	0x60(%rsp), %xmm10
14086341e:     	movaps	0x50(%rsp), %xmm11
140863424:     	movaps	0x40(%rsp), %xmm12
14086342a:     	movaps	0x30(%rsp), %xmm13
140863430:     	movaps	0x20(%rsp), %xmm14
140863436:     	addq	$0xb8, %rsp
14086343d:     	popq	%rbx
14086343e:     	popq	%rdi
14086343f:     	popq	%rsi
140863440:     	popq	%r12
140863442:     	popq	%r13
140863444:     	popq	%r14
140863446:     	popq	%r15
140863448:     	popq	%rbp
140863449:     	retq
14086344a:     	nopw	(%rax,%rax)
140863450:     	movq	%rdx, 0x10(%rsp)
140863455:     	pushq	%rbp
140863456:     	pushq	%r15
140863458:     	pushq	%r14
14086345a:     	pushq	%r13
14086345c:     	pushq	%r12
14086345e:     	pushq	%rsi
14086345f:     	pushq	%rdi
140863460:     	pushq	%rbx
140863461:     	subq	$0xb8, %rsp
140863468:     	leaq	0x80(%rdx), %rbp
14086346f:     	movaps	%xmm14, 0x20(%rsp)
140863475:     	movaps	%xmm13, 0x30(%rsp)
14086347b:     	movaps	%xmm12, 0x40(%rsp)
140863481:     	movaps	%xmm11, 0x50(%rsp)
140863487:     	movaps	%xmm10, 0x60(%rsp)
14086348d:     	movaps	%xmm9, 0x70(%rsp)
140863493:     	movaps	%xmm8, 0x80(%rsp)
14086349c:     	movdqa	%xmm7, 0x90(%rsp)
1408634a5:     	movdqa	%xmm6, 0xa0(%rsp)
1408634ae:     	movq	0xc0(%rbp), %rdx
1408634b5:     	testq	%rdx, %rdx
1408634b8:     	je	0x1408634cc <.text+0x8624cc>
1408634ba:     	movl	$0x1, %r8d
1408634c0:     	movq	0xd0(%rbp), %rcx
1408634c7:     	callq	0x140613c20 <.text+0x612c20>
1408634cc:     	movdqa	0xa0(%rsp), %xmm6
1408634d5:     	movdqa	0x90(%rsp), %xmm7
1408634de:     	movaps	0x80(%rsp), %xmm8
1408634e7:     	movaps	0x70(%rsp), %xmm9
1408634ed:     	movaps	0x60(%rsp), %xmm10
1408634f3:     	movaps	0x50(%rsp), %xmm11
1408634f9:     	movaps	0x40(%rsp), %xmm12
1408634ff:     	movaps	0x30(%rsp), %xmm13
140863505:     	movaps	0x20(%rsp), %xmm14
14086350b:     	addq	$0xb8, %rsp
140863512:     	popq	%rbx
140863513:     	popq	%rdi
140863514:     	popq	%rsi
140863515:     	popq	%r12
140863517:     	popq	%r13
140863519:     	popq	%r14
14086351b:     	popq	%r15
14086351d:     	popq	%rbp
14086351e:     	retq
14086351f:     	nop
140863520:     	movq	%rdx, 0x10(%rsp)
140863525:     	pushq	%rbp
140863526:     	pushq	%r15
140863528:     	pushq	%r14
14086352a:     	pushq	%r13
14086352c:     	pushq	%r12
14086352e:     	pushq	%rsi
14086352f:     	pushq	%rdi
140863530:     	pushq	%rbx
140863531:     	subq	$0xb8, %rsp
140863538:     	leaq	0x80(%rdx), %rbp
14086353f:     	movaps	%xmm14, 0x20(%rsp)
140863545:     	movaps	%xmm13, 0x30(%rsp)
14086354b:     	movaps	%xmm12, 0x40(%rsp)
140863551:     	movaps	%xmm11, 0x50(%rsp)
140863557:     	movaps	%xmm10, 0x60(%rsp)
14086355d:     	movaps	%xmm9, 0x70(%rsp)
140863563:     	movaps	%xmm8, 0x80(%rsp)
14086356c:     	movdqa	%xmm7, 0x90(%rsp)
140863575:     	movdqa	%xmm6, 0xa0(%rsp)
14086357e:     	movq	0xc8(%rbp), %rax
140863585:     	testq	%rax, %rax
140863588:     	je	0x1408635a4 <.text+0x8625a4>
14086358a:     	shlq	$0x4, %rax
14086358e:     	leaq	(%rax,%rax,2), %rdx
140863592:     	movl	$0x4, %r8d
140863598:     	movq	0x1b0(%rbp), %rcx
14086359f:     	callq	0x140613c20 <.text+0x612c20>
1408635a4:     	movdqa	0xa0(%rsp), %xmm6
1408635ad:     	movdqa	0x90(%rsp), %xmm7
1408635b6:     	movaps	0x80(%rsp), %xmm8
1408635bf:     	movaps	0x70(%rsp), %xmm9
1408635c5:     	movaps	0x60(%rsp), %xmm10
1408635cb:     	movaps	0x50(%rsp), %xmm11
1408635d1:     	movaps	0x40(%rsp), %xmm12
1408635d7:     	movaps	0x30(%rsp), %xmm13
1408635dd:     	movaps	0x20(%rsp), %xmm14
1408635e3:     	addq	$0xb8, %rsp
1408635ea:     	popq	%rbx
1408635eb:     	popq	%rdi
1408635ec:     	popq	%rsi
1408635ed:     	popq	%r12
1408635ef:     	popq	%r13
1408635f1:     	popq	%r14
1408635f3:     	popq	%r15
1408635f5:     	popq	%rbp
1408635f6:     	retq
1408635f7:     	nopw	(%rax,%rax)
140863600:     	movq	%rdx, 0x10(%rsp)
140863605:     	pushq	%rbp
140863606:     	pushq	%r15
140863608:     	pushq	%r14
14086360a:     	pushq	%r13
14086360c:     	pushq	%r12
14086360e:     	pushq	%rsi
14086360f:     	pushq	%rdi
140863610:     	pushq	%rbx
140863611:     	subq	$0xb8, %rsp
140863618:     	leaq	0x80(%rdx), %rbp
14086361f:     	movaps	%xmm14, 0x20(%rsp)
140863625:     	movaps	%xmm13, 0x30(%rsp)
14086362b:     	movaps	%xmm12, 0x40(%rsp)
140863631:     	movaps	%xmm11, 0x50(%rsp)
140863637:     	movaps	%xmm10, 0x60(%rsp)
14086363d:     	movaps	%xmm9, 0x70(%rsp)
140863643:     	movaps	%xmm8, 0x80(%rsp)
14086364c:     	movdqa	%xmm7, 0x90(%rsp)
140863655:     	movdqa	%xmm6, 0xa0(%rsp)
14086365e:     	movq	0x1f0(%rbp), %rax
140863665:     	lock
140863666:     	decq	(%rax)
140863669:     	jne	0x140863677 <.text+0x862677>
14086366b:     	leaq	0x1f0(%rbp), %rcx
140863672:     	callq	0x14064eb30 <.text+0x64db30>
140863677:     	movdqa	0xa0(%rsp), %xmm6
140863680:     	movdqa	0x90(%rsp), %xmm7
140863689:     	movaps	0x80(%rsp), %xmm8
140863692:     	movaps	0x70(%rsp), %xmm9
140863698:     	movaps	0x60(%rsp), %xmm10
14086369e:     	movaps	0x50(%rsp), %xmm11
1408636a4:     	movaps	0x40(%rsp), %xmm12
1408636aa:     	movaps	0x30(%rsp), %xmm13
1408636b0:     	movaps	0x20(%rsp), %xmm14
1408636b6:     	addq	$0xb8, %rsp
1408636bd:     	popq	%rbx
1408636be:     	popq	%rdi
1408636bf:     	popq	%rsi
1408636c0:     	popq	%r12
1408636c2:     	popq	%r13
1408636c4:     	popq	%r14
1408636c6:     	popq	%r15
1408636c8:     	popq	%rbp
1408636c9:     	retq
1408636ca:     	nopw	(%rax,%rax)
1408636d0:     	movq	%rdx, 0x10(%rsp)
1408636d5:     	pushq	%rbp
1408636d6:     	pushq	%r15
1408636d8:     	pushq	%r14
1408636da:     	pushq	%r13
1408636dc:     	pushq	%r12
1408636de:     	pushq	%rsi
1408636df:     	pushq	%rdi
1408636e0:     	pushq	%rbx
1408636e1:     	subq	$0xb8, %rsp
1408636e8:     	leaq	0x80(%rdx), %rbp
1408636ef:     	movaps	%xmm14, 0x20(%rsp)
1408636f5:     	movaps	%xmm13, 0x30(%rsp)
1408636fb:     	movaps	%xmm12, 0x40(%rsp)
140863701:     	movaps	%xmm11, 0x50(%rsp)
140863707:     	movaps	%xmm10, 0x60(%rsp)
14086370d:     	movaps	%xmm9, 0x70(%rsp)
140863713:     	movaps	%xmm8, 0x80(%rsp)
14086371c:     	movdqa	%xmm7, 0x90(%rsp)
140863725:     	movdqa	%xmm6, 0xa0(%rsp)
14086372e:     	movq	0x48(%rbp), %rax
140863732:     	lock
140863733:     	decq	(%rax)
140863736:     	jne	0x140863744 <.text+0x862744>
140863738:     	leaq	0xd8(%rbp), %rcx
14086373f:     	callq	0x14064eb30 <.text+0x64db30>
140863744:     	movdqa	0xa0(%rsp), %xmm6
14086374d:     	movdqa	0x90(%rsp), %xmm7
140863756:     	movaps	0x80(%rsp), %xmm8
14086375f:     	movaps	0x70(%rsp), %xmm9
140863765:     	movaps	0x60(%rsp), %xmm10
14086376b:     	movaps	0x50(%rsp), %xmm11
140863771:     	movaps	0x40(%rsp), %xmm12
140863777:     	movaps	0x30(%rsp), %xmm13
14086377d:     	movaps	0x20(%rsp), %xmm14
140863783:     	addq	$0xb8, %rsp
14086378a:     	popq	%rbx
14086378b:     	popq	%rdi
14086378c:     	popq	%rsi
14086378d:     	popq	%r12
14086378f:     	popq	%r13
140863791:     	popq	%r14
140863793:     	popq	%r15
140863795:     	popq	%rbp
140863796:     	retq
140863797:     	nopw	(%rax,%rax)
1408637a0:     	movq	%rdx, 0x10(%rsp)
1408637a5:     	pushq	%rbp
1408637a6:     	pushq	%r15
1408637a8:     	pushq	%r14
1408637aa:     	pushq	%r13
1408637ac:     	pushq	%r12
1408637ae:     	pushq	%rsi
1408637af:     	pushq	%rdi
1408637b0:     	pushq	%rbx
1408637b1:     	subq	$0xb8, %rsp
1408637b8:     	leaq	0x80(%rdx), %rbp
1408637bf:     	movaps	%xmm14, 0x20(%rsp)
1408637c5:     	movaps	%xmm13, 0x30(%rsp)
1408637cb:     	movaps	%xmm12, 0x40(%rsp)
1408637d1:     	movaps	%xmm11, 0x50(%rsp)
1408637d7:     	movaps	%xmm10, 0x60(%rsp)
1408637dd:     	movaps	%xmm9, 0x70(%rsp)
1408637e3:     	movaps	%xmm8, 0x80(%rsp)
1408637ec:     	movdqa	%xmm7, 0x90(%rsp)
1408637f5:     	movdqa	%xmm6, 0xa0(%rsp)
1408637fe:     	leaq	0x50(%rbp), %rcx
140863802:     	callq	0x1405e88a0 <.text+0x5e78a0>
140863807:     	movdqa	0xa0(%rsp), %xmm6
140863810:     	movdqa	0x90(%rsp), %xmm7
140863819:     	movaps	0x80(%rsp), %xmm8
140863822:     	movaps	0x70(%rsp), %xmm9
140863828:     	movaps	0x60(%rsp), %xmm10
14086382e:     	movaps	0x50(%rsp), %xmm11
140863834:     	movaps	0x40(%rsp), %xmm12
14086383a:     	movaps	0x30(%rsp), %xmm13
140863840:     	movaps	0x20(%rsp), %xmm14
140863846:     	addq	$0xb8, %rsp
14086384d:     	popq	%rbx
14086384e:     	popq	%rdi
14086384f:     	popq	%rsi
140863850:     	popq	%r12
140863852:     	popq	%r13
140863854:     	popq	%r14
140863856:     	popq	%r15
140863858:     	popq	%rbp
140863859:     	retq
14086385a:     	nopw	(%rax,%rax)
140863860:     	movq	%rdx, 0x10(%rsp)
140863865:     	pushq	%rbp
140863866:     	pushq	%r15
140863868:     	pushq	%r14
14086386a:     	pushq	%r13
14086386c:     	pushq	%r12
14086386e:     	pushq	%rsi
14086386f:     	pushq	%rdi
140863870:     	pushq	%rbx
140863871:     	subq	$0xb8, %rsp
140863878:     	leaq	0x80(%rdx), %rbp
14086387f:     	movaps	%xmm14, 0x20(%rsp)
140863885:     	movaps	%xmm13, 0x30(%rsp)
14086388b:     	movaps	%xmm12, 0x40(%rsp)
140863891:     	movaps	%xmm11, 0x50(%rsp)
140863897:     	movaps	%xmm10, 0x60(%rsp)
14086389d:     	movaps	%xmm9, 0x70(%rsp)
1408638a3:     	movaps	%xmm8, 0x80(%rsp)
1408638ac:     	movdqa	%xmm7, 0x90(%rsp)
1408638b5:     	movdqa	%xmm6, 0xa0(%rsp)
1408638be:     	leaq	0x158(%rbp), %rcx
1408638c5:     	callq	0x140663570 <.text+0x662570>
1408638ca:     	leaq	0xa0(%rbp), %rcx
1408638d1:     	callq	0x140879230 <.text+0x878230>
1408638d6:     	movaps	0xa0(%rsp), %xmm6
1408638de:     	movaps	0x90(%rsp), %xmm7
1408638e6:     	movaps	0x80(%rsp), %xmm8
1408638ef:     	movaps	0x70(%rsp), %xmm9
1408638f5:     	movaps	0x60(%rsp), %xmm10
1408638fb:     	movaps	0x50(%rsp), %xmm11
140863901:     	movaps	0x40(%rsp), %xmm12
140863907:     	movaps	0x30(%rsp), %xmm13
14086390d:     	movaps	0x20(%rsp), %xmm14
140863913:     	addq	$0xb8, %rsp
14086391a:     	popq	%rbx
14086391b:     	popq	%rdi
14086391c:     	popq	%rsi
14086391d:     	popq	%r12
14086391f:     	popq	%r13
140863921:     	popq	%r14
140863923:     	popq	%r15
140863925:     	popq	%rbp
140863926:     	retq
140863927:     	nopw	(%rax,%rax)
140863930:     	movq	%rdx, 0x10(%rsp)
140863935:     	pushq	%rbp
140863936:     	pushq	%r15
140863938:     	pushq	%r14
14086393a:     	pushq	%r13
14086393c:     	pushq	%r12
14086393e:     	pushq	%rsi
14086393f:     	pushq	%rdi
140863940:     	pushq	%rbx
140863941:     	subq	$0xb8, %rsp
140863948:     	leaq	0x80(%rdx), %rbp
14086394f:     	movaps	%xmm14, 0x20(%rsp)
140863955:     	movaps	%xmm13, 0x30(%rsp)
14086395b:     	movaps	%xmm12, 0x40(%rsp)
140863961:     	movaps	%xmm11, 0x50(%rsp)
140863967:     	movaps	%xmm10, 0x60(%rsp)
14086396d:     	movaps	%xmm9, 0x70(%rsp)
140863973:     	movaps	%xmm8, 0x80(%rsp)
14086397c:     	movdqa	%xmm7, 0x90(%rsp)
140863985:     	movdqa	%xmm6, 0xa0(%rsp)
14086398e:     	leaq	0x1c0(%rbp), %rcx
140863995:     	callq	0x1405e88a0 <.text+0x5e78a0>
14086399a:     	movdqa	0xa0(%rsp), %xmm6
1408639a3:     	movdqa	0x90(%rsp), %xmm7
1408639ac:     	movaps	0x80(%rsp), %xmm8
1408639b5:     	movaps	0x70(%rsp), %xmm9
1408639bb:     	movaps	0x60(%rsp), %xmm10
1408639c1:     	movaps	0x50(%rsp), %xmm11
1408639c7:     	movaps	0x40(%rsp), %xmm12
1408639cd:     	movaps	0x30(%rsp), %xmm13
1408639d3:     	movaps	0x20(%rsp), %xmm14
1408639d9:     	addq	$0xb8, %rsp
1408639e0:     	popq	%rbx
1408639e1:     	popq	%rdi
1408639e2:     	popq	%rsi
1408639e3:     	popq	%r12
1408639e5:     	popq	%r13
1408639e7:     	popq	%r14
1408639e9:     	popq	%r15
1408639eb:     	popq	%rbp
1408639ec:     	retq
