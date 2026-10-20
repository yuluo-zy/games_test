
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409a2290:     	pushq	%rbp
1409a2291:     	pushq	%r15
1409a2293:     	pushq	%r14
1409a2295:     	pushq	%r13
1409a2297:     	pushq	%r12
1409a2299:     	pushq	%rsi
1409a229a:     	pushq	%rdi
1409a229b:     	pushq	%rbx
1409a229c:     	subq	$0x258, %rsp            # imm = 0x258
1409a22a3:     	leaq	0x80(%rsp), %rbp
1409a22ab:     	movaps	%xmm11, 0x1c0(%rbp)
1409a22b3:     	movaps	%xmm10, 0x1b0(%rbp)
1409a22bb:     	movaps	%xmm9, 0x1a0(%rbp)
1409a22c3:     	movaps	%xmm8, 0x190(%rbp)
1409a22cb:     	movdqa	%xmm7, 0x180(%rbp)
1409a22d3:     	movaps	%xmm6, 0x170(%rbp)
1409a22da:     	movq	$-0x2, 0x168(%rbp)
1409a22e5:     	movq	%r8, 0xb8(%rbp)
1409a22ec:     	movq	(%rcx), %rbx
1409a22ef:     	movq	0x8(%rcx), %r8
1409a22f3:     	movq	(%rbx), %r11
1409a22f6:     	xorl	%r10d, %r10d
1409a22f9:     	movq	%r11, %rcx
1409a22fc:     	subq	0x18(%r8), %rcx
1409a2300:     	cmovbq	%r10, %rcx
1409a2304:     	subq	0x38(%r8), %r11
1409a2308:     	movq	%rdx, 0xc0(%rbp)
1409a230f:     	cmovbq	%r10, %r11
1409a2313:     	leaq	(%rcx,%rcx,2), %rax
1409a2317:     	shlq	$0x3, %rax
1409a231b:     	addq	0x8(%r8), %rax
1409a231f:     	movq	0x10(%r8), %rdx
1409a2323:     	movq	0x30(%r8), %rsi
1409a2327:     	subq	%rcx, %rdx
1409a232a:     	cmovbq	%r10, %rdx
1409a232e:     	movl	$0x8, %edi
1409a2333:     	cmovbq	%rdi, %rax
1409a2337:     	leaq	(%r11,%r11,2), %rcx
1409a233b:     	shlq	$0x3, %rcx
1409a233f:     	addq	0x28(%r8), %rcx
1409a2343:     	subq	%r11, %rsi
1409a2346:     	cmovbq	%r10, %rsi
1409a234a:     	cmovbq	%rdi, %rcx
1409a234e:     	movq	0x248(%rbp), %r10
1409a2355:     	movq	0x40(%r8), %r8
1409a2359:     	subq	%rsi, %r8
1409a235c:     	subq	%rdx, %r8
1409a235f:     	movq	%rbx, 0x118(%rbp)
1409a2366:     	movq	%r8, (%rbx)
1409a2369:     	leaq	(%rdx,%rdx,2), %rdx
1409a236d:     	leaq	(%rax,%rdx,8), %rdx
1409a2371:     	movq	%rdx, 0xe8(%rbp)
1409a2378:     	leaq	(%rsi,%rsi,2), %rdx
1409a237c:     	leaq	(%rcx,%rdx,8), %rdx
1409a2380:     	movq	%rdx, 0xf0(%rbp)
1409a2387:     	movl	0x1c(%r9), %edx
1409a238b:     	movl	%edx, 0x15c(%rbp)
1409a2391:     	movq	(%r9), %rdx
1409a2394:     	movq	%rdx, 0xc8(%rbp)
1409a239b:     	movq	0x10(%r9), %rdx
1409a239f:     	movq	%rdx, 0xd0(%rbp)
1409a23a6:     	movl	0x1c(%r10), %edx
1409a23aa:     	movl	%edx, 0x160(%rbp)
1409a23b0:     	movq	(%r10), %rdx
1409a23b3:     	movq	%rdx, 0xe0(%rbp)
1409a23ba:     	movq	0x10(%r10), %rdx
1409a23be:     	movq	%rdx, 0xd8(%rbp)
1409a23c5:     	movq	0x240(%rbp), %rdx
1409a23cc:     	movq	(%rdx), %r8
1409a23cf:     	movq	0x250(%rbp), %rdx
1409a23d6:     	movl	0x1c(%rdx), %r9d
1409a23da:     	movl	%r9d, 0xf8(%rbp)
1409a23e1:     	movq	(%rdx), %r9
1409a23e4:     	movq	0x10(%rdx), %rdx
1409a23e8:     	movq	%rdx, 0xa0(%rbp)
1409a23ef:     	pcmpeqd	%xmm7, %xmm7
1409a23f3:     	xorps	%xmm8, %xmm8
1409a23f7:     	movss	0x1f83471(%rip), %xmm6  # 0x142925870
1409a23ff:     	movss	0x1f834fc(%rip), %xmm9  # 0x142925904
1409a2408:     	movq	%r8, 0x108(%rbp)
1409a240f:     	leaq	0x20(%r8), %rdx
1409a2413:     	movq	%rdx, 0xa8(%rbp)
1409a241a:     	movq	%r9, 0x100(%rbp)
1409a2421:     	leaq	0x20(%r9), %rdx
1409a2425:     	movq	%rdx, 0xb0(%rbp)
1409a242c:     	jmp	0x1409a2476 <.text+0x9a1476>
1409a242e:     	nop
1409a2430:     	movq	0x100(%rbp), %rdx
1409a2437:     	movq	0x28(%rdx), %rax
1409a243b:     	movq	%rsi, %rcx
1409a243e:     	shlq	$0x5, %rcx
1409a2442:     	movq	%rdi, (%rax,%rcx)
1409a2446:     	movb	$0x10, 0x8(%rax,%rcx)
1409a244b:     	movzbl	0x110(%rbp), %r8d
1409a2453:     	movb	%r8b, 0x9(%rax,%rcx)
1409a2458:     	movlps	%xmm10, 0x10(%rax,%rcx)
1409a245e:     	movss	%xmm11, 0x18(%rax,%rcx)
1409a2465:     	incq	%rsi
1409a2468:     	movq	%rsi, 0x30(%rdx)
1409a246c:     	incq	0x40(%rdx)
1409a2470:     	movq	%r15, %rcx
1409a2473:     	movq	%r12, %rax
1409a2476:     	testq	%rax, %rax
1409a2479:     	je	0x1409a2490 <.text+0x9a1490>
1409a247b:     	cmpq	0xe8(%rbp), %rax
1409a2482:     	je	0x1409a2490 <.text+0x9a1490>
1409a2484:     	leaq	0x18(%rax), %r12
1409a2488:     	movq	%rcx, %r15
1409a248b:     	jmp	0x1409a24b0 <.text+0x9a14b0>
1409a248d:     	nopl	(%rax)
1409a2490:     	testq	%rcx, %rcx
1409a2493:     	je	0x1409a27bd <.text+0x9a17bd>
1409a2499:     	cmpq	0xf0(%rbp), %rcx
1409a24a0:     	je	0x1409a27bd <.text+0x9a17bd>
1409a24a6:     	leaq	0x18(%rcx), %r15
1409a24aa:     	xorl	%r12d, %r12d
1409a24ad:     	movq	%rcx, %rax
1409a24b0:     	movq	0x118(%rbp), %rcx
1409a24b7:     	incq	(%rcx)
1409a24ba:     	movzbl	0x10(%rax), %esi
1409a24be:     	movzbl	0x11(%rax), %edx
1409a24c2:     	movq	0x8(%rax), %rbx
1409a24c6:     	movzbl	0x13(%rax), %edi
1409a24ca:     	movzbl	0x12(%rax), %r14d
1409a24cf:     	movl	0x15c(%rbp), %eax
1409a24d5:     	movq	0xd0(%rbp), %rcx
1409a24dc:     	movl	%eax, (%rcx)
1409a24de:     	movb	%dl, 0x110(%rbp)
1409a24e4:     	movb	%dl, -0x58(%rbp)
1409a24e7:     	movq	0xc8(%rbp), %rcx
1409a24ee:     	movq	%rbx, %rdx
1409a24f1:     	leaq	-0x58(%rbp), %r13
1409a24f5:     	movq	%r13, %r8
1409a24f8:     	callq	0x140aaa350 <.text+0xaa9350>
1409a24fd:     	testb	%r14b, %r14b
1409a2500:     	movq	%r13, %r14
1409a2503:     	je	0x1409a2602 <.text+0x9a1602>
1409a2509:     	movq	$0x0, 0x140(%rbp)
1409a2514:     	movq	$0x8, 0x148(%rbp)
1409a251f:     	movq	$0x0, 0x150(%rbp)
1409a252a:     	movq	%rbx, -0x50(%rbp)
1409a252e:     	movb	%sil, -0x48(%rbp)
1409a2532:     	movzbl	0x110(%rbp), %eax
1409a2539:     	movb	%al, -0x47(%rbp)
1409a253c:     	movabsq	$-0x7ffffffffffffff8, %rax # imm = 0x8000000000000008
1409a2546:     	movq	%rax, -0x58(%rbp)
1409a254a:     	movb	$0x1, 0x165(%rbp)
1409a2551:     	leaq	0x140(%rbp), %rcx
1409a2558:     	movq	%r14, %rdx
1409a255b:     	callq	0x140a82b40 <.text+0xa81b40>
1409a2560:     	movl	0x160(%rbp), %eax
1409a2566:     	movq	0xd8(%rbp), %rcx
1409a256d:     	movl	%eax, (%rcx)
1409a256f:     	movdqu	0x140(%rbp), %xmm0
1409a2577:     	movdqa	%xmm0, 0x120(%rbp)
1409a257f:     	movq	0x150(%rbp), %rax
1409a2586:     	movq	0x120(%rbp), %rcx
1409a258d:     	movq	0x128(%rbp), %rdx
1409a2594:     	movq	$0x0, 0x120(%rbp)
1409a259f:     	movq	$0x8, 0x128(%rbp)
1409a25aa:     	movq	$0x0, 0x130(%rbp)
1409a25b5:     	leaq	(%rax,%rax,2), %rax
1409a25b9:     	leaq	(%rdx,%rax,8), %rax
1409a25bd:     	movq	%rdx, -0x58(%rbp)
1409a25c1:     	movq	%rdx, -0x50(%rbp)
1409a25c5:     	movq	%rcx, -0x48(%rbp)
1409a25c9:     	movq	%rax, -0x40(%rbp)
1409a25cd:     	movq	%r14, %rcx
1409a25d0:     	callq	0x141696f50 <.text+0x1695f50>
1409a25d5:     	movq	%rax, -0x50(%rbp)
1409a25d9:     	movq	%rdx, -0x48(%rbp)
1409a25dd:     	movq	$0x7, -0x58(%rbp)
1409a25e5:     	movb	$0x0, 0x165(%rbp)
1409a25ec:     	movq	0xe0(%rbp), %rcx
1409a25f3:     	movq	%r14, %rdx
1409a25f6:     	leaq	0x211a343(%rip), %r8    # 0x142abc940
1409a25fd:     	callq	0x140a84ec0 <.text+0xa83ec0>
1409a2602:     	movq	%r15, %rcx
1409a2605:     	movq	%r12, %rax
1409a2608:     	testb	%dil, %dil
1409a260b:     	je	0x1409a2476 <.text+0x9a1476>
1409a2611:     	movq	%rbx, -0x58(%rbp)
1409a2615:     	movq	0x108(%rbp), %rax
1409a261c:     	cmpq	$0x0, 0x18(%rax)
1409a2621:     	je	0x1409a26d0 <.text+0x9a16d0>
1409a2627:     	movq	0xa8(%rbp), %rcx
1409a262e:     	movq	%r14, %rdx
1409a2631:     	callq	0x1405b0d70 <.text+0x5afd70>
1409a2636:     	movq	0x108(%rbp), %rcx
1409a263d:     	movq	(%rcx), %rdi
1409a2640:     	movq	0x8(%rcx), %rcx
1409a2644:     	movq	%rcx, %rdx
1409a2647:     	andq	%rax, %rdx
1409a264a:     	shrq	$0x39, %rax
1409a264e:     	movd	%eax, %xmm0
1409a2652:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1409a2656:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1409a265b:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1409a2660:     	leaq	-0x2a8(%rdi), %rax
1409a2667:     	xorl	%r8d, %r8d
1409a266a:     	movdqu	(%rdi,%rdx), %xmm1
1409a266f:     	movdqa	%xmm1, %xmm2
1409a2673:     	pcmpeqb	%xmm0, %xmm2
1409a2677:     	pmovmskb	%xmm2, %r9d
1409a267c:     	testl	%r9d, %r9d
1409a267f:     	je	0x1409a26b0 <.text+0x9a16b0>
1409a2681:     	tzcntl	%r9d, %r10d
1409a2686:     	addq	%rdx, %r10
1409a2689:     	andq	%rcx, %r10
1409a268c:     	negq	%r10
1409a268f:     	imulq	$0x2a8, %r10, %r10      # imm = 0x2A8
1409a2696:     	cmpq	%rbx, (%rax,%r10)
1409a269a:     	je	0x1409a26dd <.text+0x9a16dd>
1409a269c:     	leal	-0x1(%r9), %r10d
1409a26a0:     	andw	%r9w, %r10w
1409a26a4:     	movl	%r10d, %r9d
1409a26a7:     	jne	0x1409a2681 <.text+0x9a1681>
1409a26a9:     	nopl	(%rax)
1409a26b0:     	pcmpeqb	%xmm7, %xmm1
1409a26b4:     	pmovmskb	%xmm1, %r9d
1409a26b9:     	testl	%r9d, %r9d
1409a26bc:     	jne	0x1409a26d0 <.text+0x9a16d0>
1409a26be:     	addq	%r8, %rdx
1409a26c1:     	addq	$0x10, %rdx
1409a26c5:     	addq	$0x10, %r8
1409a26c9:     	andq	%rcx, %rdx
1409a26cc:     	jmp	0x1409a266a <.text+0x9a166a>
1409a26ce:     	nop
1409a26d0:     	xorps	%xmm10, %xmm10
1409a26d4:     	xorps	%xmm11, %xmm11
1409a26d8:     	jmp	0x1409a277d <.text+0x9a177d>
1409a26dd:     	cmpq	$0x2, -0x290(%rdi,%r10)
1409a26e6:     	jb	0x1409a2ccd <.text+0x9a1ccd>
1409a26ec:     	movss	-0x270(%rdi,%r10), %xmm0
1409a26f6:     	ucomiss	%xmm8, %xmm0
1409a26fa:     	jbe	0x1409a2ccd <.text+0x9a1ccd>
1409a2700:     	addq	%r10, %rdi
1409a2703:     	leaq	-0x268(%rdi), %rcx
1409a270a:     	movaps	%xmm6, %xmm1
1409a270d:     	callq	0x141467ce0 <.text+0x1466ce0>
1409a2712:     	movl	%eax, %ecx
1409a2714:     	movq	-0x258(%rdi), %rdx
1409a271b:     	cmpq	%rcx, %rdx
1409a271e:     	jbe	0x1409a2cf4 <.text+0x9a1cf4>
1409a2724:     	leaq	0x1(%rcx), %rax
1409a2728:     	cmpq	%rdx, %rax
1409a272b:     	jae	0x1409a2ce5 <.text+0x9a1ce5>
1409a2731:     	movq	-0x260(%rdi), %rdx
1409a2738:     	leaq	(%rcx,%rcx,2), %rcx
1409a273c:     	leaq	(%rax,%rax,2), %rax
1409a2740:     	movaps	%xmm9, %xmm1
1409a2744:     	subss	%xmm0, %xmm1
1409a2748:     	movsldup	%xmm1, %xmm2            # xmm2 = xmm1[0,0,2,2]
1409a274c:     	mulss	0x8(%rdx,%rcx,4), %xmm1
1409a2752:     	movsd	(%rdx,%rcx,4), %xmm3
1409a2757:     	movsd	(%rdx,%rax,4), %xmm4
1409a275c:     	mulps	%xmm2, %xmm3
1409a275f:     	movsldup	%xmm0, %xmm10           # xmm10 = xmm0[0,0,2,2]
1409a2764:     	mulps	%xmm4, %xmm10
1409a2768:     	addps	%xmm3, %xmm10
1409a276c:     	movss	0x8(%rdx,%rax,4), %xmm11
1409a2773:     	mulss	%xmm0, %xmm11
1409a2778:     	addss	%xmm1, %xmm11
1409a277d:     	movl	0xf8(%rbp), %eax
1409a2783:     	movq	0xa0(%rbp), %rcx
1409a278a:     	movl	%eax, (%rcx)
1409a278c:     	movq	0x100(%rbp), %rax
1409a2793:     	movq	0x30(%rax), %rsi
1409a2797:     	movq	0x40(%rax), %rdi
1409a279b:     	cmpq	0x20(%rax), %rsi
1409a279f:     	jne	0x1409a2430 <.text+0x9a1430>
1409a27a5:     	movq	0xb0(%rbp), %rcx
1409a27ac:     	leaq	0x2297995(%rip), %rdx   # 0x142c3a148
1409a27b3:     	callq	0x140562c30 <.text+0x561c30>
1409a27b8:     	jmp	0x1409a2430 <.text+0x9a1430>
1409a27bd:     	movq	0xb8(%rbp), %rax
1409a27c4:     	movq	(%rax), %r15
1409a27c7:     	movq	0x8(%rax), %rcx
1409a27cb:     	movq	(%r15), %r8
1409a27ce:     	xorl	%edx, %edx
1409a27d0:     	movq	%r8, %r9
1409a27d3:     	subq	0x18(%rcx), %r9
1409a27d7:     	cmovbq	%rdx, %r9
1409a27db:     	subq	0x38(%rcx), %r8
1409a27df:     	cmovbq	%rdx, %r8
1409a27e3:     	movq	%r9, %rax
1409a27e6:     	shlq	$0x5, %rax
1409a27ea:     	addq	0x8(%rcx), %rax
1409a27ee:     	movq	0x10(%rcx), %r10
1409a27f2:     	movq	0x30(%rcx), %r11
1409a27f6:     	subq	%r9, %r10
1409a27f9:     	cmovbq	%rdx, %r10
1409a27fd:     	movl	$0x8, %r9d
1409a2803:     	cmovbq	%r9, %rax
1409a2807:     	movq	%r8, %rdi
1409a280a:     	shlq	$0x5, %rdi
1409a280e:     	addq	0x28(%rcx), %rdi
1409a2812:     	subq	%r8, %r11
1409a2815:     	cmovbq	%rdx, %r11
1409a2819:     	cmovbq	%r9, %rdi
1409a281d:     	movq	0x40(%rcx), %rcx
1409a2821:     	subq	%r11, %rcx
1409a2824:     	subq	%r10, %rcx
1409a2827:     	movq	%rcx, (%r15)
1409a282a:     	shlq	$0x5, %r10
1409a282e:     	addq	%rax, %r10
1409a2831:     	movq	%r10, 0x110(%rbp)
1409a2838:     	shlq	$0x5, %r11
1409a283c:     	addq	%rdi, %r11
1409a283f:     	movq	%r11, 0x118(%rbp)
1409a2846:     	leaq	-0x58(%rbp), %rbx
1409a284a:     	nopw	(%rax,%rax)
1409a2850:     	testq	%rax, %rax
1409a2853:     	je	0x1409a2870 <.text+0x9a1870>
1409a2855:     	cmpq	0x110(%rbp), %rax
1409a285c:     	je	0x1409a2870 <.text+0x9a1870>
1409a285e:     	leaq	0x20(%rax), %r13
1409a2862:     	movq	%rdi, %r14
1409a2865:     	jmp	0x1409a2893 <.text+0x9a1893>
1409a2867:     	nopw	(%rax,%rax)
1409a2870:     	testq	%rdi, %rdi
1409a2873:     	je	0x1409a29d1 <.text+0x9a19d1>
1409a2879:     	cmpq	0x118(%rbp), %rdi
1409a2880:     	je	0x1409a29d1 <.text+0x9a19d1>
1409a2886:     	leaq	0x20(%rdi), %r14
1409a288a:     	xorl	%r13d, %r13d
1409a288d:     	movq	%rdi, %rax
1409a2890:     	movq	%r14, %rdi
1409a2893:     	incq	(%r15)
1409a2896:     	movsd	0x10(%rax), %xmm6
1409a289b:     	movshdup	%xmm6, %xmm0            # xmm0 = xmm6[1,1,3,3]
1409a289f:     	movq	0x8(%rax), %r12
1409a28a3:     	movzbl	0x18(%rax), %esi
1409a28a7:     	movl	0x15c(%rbp), %eax
1409a28ad:     	movq	0xd0(%rbp), %rcx
1409a28b4:     	movl	%eax, (%rcx)
1409a28b6:     	movss	%xmm0, -0x58(%rbp)
1409a28bb:     	movq	0xc8(%rbp), %rcx
1409a28c2:     	movq	%r12, %rdx
1409a28c5:     	movq	%rbx, %r8
1409a28c8:     	callq	0x140aaa530 <.text+0xaa9530>
1409a28cd:     	movq	%r13, %rax
1409a28d0:     	testb	%sil, %sil
1409a28d3:     	je	0x1409a2850 <.text+0x9a1850>
1409a28d9:     	movq	$0x0, 0x140(%rbp)
1409a28e4:     	movq	$0x8, 0x148(%rbp)
1409a28ef:     	movq	$0x0, 0x150(%rbp)
1409a28fa:     	movq	%r12, -0x50(%rbp)
1409a28fe:     	movlps	%xmm6, -0x48(%rbp)
1409a2902:     	movabsq	$-0x7ffffffffffffff7, %rax # imm = 0x8000000000000009
1409a290c:     	movq	%rax, -0x58(%rbp)
1409a2910:     	movb	$0x1, 0x166(%rbp)
1409a2917:     	leaq	0x140(%rbp), %rcx
1409a291e:     	movq	%rbx, %rdx
1409a2921:     	callq	0x140a82b40 <.text+0xa81b40>
1409a2926:     	movl	0x160(%rbp), %eax
1409a292c:     	movq	0xd8(%rbp), %rcx
1409a2933:     	movl	%eax, (%rcx)
1409a2935:     	movups	0x140(%rbp), %xmm0
1409a293c:     	movaps	%xmm0, 0x120(%rbp)
1409a2943:     	movq	0x150(%rbp), %rax
1409a294a:     	movq	0x120(%rbp), %rcx
1409a2951:     	movq	0x128(%rbp), %rdx
1409a2958:     	movq	$0x0, 0x120(%rbp)
1409a2963:     	movq	$0x8, 0x128(%rbp)
1409a296e:     	movq	$0x0, 0x130(%rbp)
1409a2979:     	leaq	(%rax,%rax,2), %rax
1409a297d:     	leaq	(%rdx,%rax,8), %rax
1409a2981:     	movq	%rdx, -0x58(%rbp)
1409a2985:     	movq	%rdx, -0x50(%rbp)
1409a2989:     	movq	%rcx, -0x48(%rbp)
1409a298d:     	movq	%rax, -0x40(%rbp)
1409a2991:     	movq	%rbx, %rcx
1409a2994:     	callq	0x141696f50 <.text+0x1695f50>
1409a2999:     	movq	%rax, -0x50(%rbp)
1409a299d:     	movq	%rdx, -0x48(%rbp)
1409a29a1:     	movq	$0x7, -0x58(%rbp)
1409a29a9:     	movb	$0x0, 0x166(%rbp)
1409a29b0:     	movq	0xe0(%rbp), %rcx
1409a29b7:     	movq	%rbx, %rdx
1409a29ba:     	leaq	0x2119f67(%rip), %r8    # 0x142abc928
1409a29c1:     	callq	0x140a84ec0 <.text+0xa83ec0>
1409a29c6:     	movq	%r13, %rax
1409a29c9:     	movq	%r14, %rdi
1409a29cc:     	jmp	0x1409a2850 <.text+0x9a1850>
1409a29d1:     	movq	0xc0(%rbp), %rax
1409a29d8:     	movq	(%rax), %rsi
1409a29db:     	movq	0x8(%rax), %rcx
1409a29df:     	movq	(%rsi), %r8
1409a29e2:     	xorl	%edx, %edx
1409a29e4:     	movq	%r8, %r9
1409a29e7:     	subq	0x18(%rcx), %r9
1409a29eb:     	cmovbq	%rdx, %r9
1409a29ef:     	subq	0x38(%rcx), %r8
1409a29f3:     	cmovbq	%rdx, %r8
1409a29f7:     	leaq	(%r9,%r9,2), %rax
1409a29fb:     	shlq	$0x3, %rax
1409a29ff:     	addq	0x8(%rcx), %rax
1409a2a03:     	movq	0x10(%rcx), %r10
1409a2a07:     	subq	%r9, %r10
1409a2a0a:     	cmovbq	%rdx, %r10
1409a2a0e:     	movl	$0x8, %r9d
1409a2a14:     	cmovbq	%r9, %rax
1409a2a18:     	leaq	(%r8,%r8,2), %rdi
1409a2a1c:     	shlq	$0x3, %rdi
1409a2a20:     	addq	0x28(%rcx), %rdi
1409a2a24:     	movq	0x30(%rcx), %r11
1409a2a28:     	subq	%r8, %r11
1409a2a2b:     	cmovbq	%rdx, %r11
1409a2a2f:     	cmovbq	%r9, %rdi
1409a2a33:     	movq	0x40(%rcx), %rcx
1409a2a37:     	subq	%r11, %rcx
1409a2a3a:     	subq	%r10, %rcx
1409a2a3d:     	movq	%rsi, 0xe8(%rbp)
1409a2a44:     	movq	%rcx, (%rsi)
1409a2a47:     	leaq	(%r10,%r10,2), %rcx
1409a2a4b:     	leaq	(%rax,%rcx,8), %rcx
1409a2a4f:     	movq	%rcx, 0xf0(%rbp)
1409a2a56:     	leaq	(%r11,%r11,2), %rcx
1409a2a5a:     	leaq	(%rdi,%rcx,8), %rcx
1409a2a5e:     	movq	%rcx, 0xf8(%rbp)
1409a2a65:     	movq	0xd0(%rbp), %r15
1409a2a6c:     	movq	0xc8(%rbp), %r13
1409a2a73:     	nopw	%cs:(%rax,%rax)
1409a2a80:     	testq	%rax, %rax
1409a2a83:     	je	0x1409a2ab0 <.text+0x9a1ab0>
1409a2a85:     	cmpq	0xf0(%rbp), %rax
1409a2a8c:     	je	0x1409a2ab0 <.text+0x9a1ab0>
1409a2a8e:     	leaq	0x18(%rax), %rbx
1409a2a92:     	movq	%rdi, 0x118(%rbp)
1409a2a99:     	movq	%rdi, 0x108(%rbp)
1409a2aa0:     	jmp	0x1409a2add <.text+0x9a1add>
1409a2aa2:     	nopw	%cs:(%rax,%rax)
1409a2ab0:     	testq	%rdi, %rdi
1409a2ab3:     	je	0x1409a2c8b <.text+0x9a1c8b>
1409a2ab9:     	cmpq	0xf8(%rbp), %rdi
1409a2ac0:     	je	0x1409a2c8b <.text+0x9a1c8b>
1409a2ac6:     	leaq	0x18(%rdi), %rcx
1409a2aca:     	xorl	%ebx, %ebx
1409a2acc:     	movq	%rdi, %rax
1409a2acf:     	movq	%rcx, 0x108(%rbp)
1409a2ad6:     	movq	%rcx, 0x118(%rbp)
1409a2add:     	movq	0xe8(%rbp), %rcx
1409a2ae4:     	incq	(%rcx)
1409a2ae7:     	movzbl	0x10(%rax), %ecx
1409a2aeb:     	movb	%cl, 0x100(%rbp)
1409a2af1:     	movzbl	0x11(%rax), %esi
1409a2af5:     	movq	0x8(%rax), %r12
1409a2af9:     	movzbl	0x12(%rax), %eax
1409a2afd:     	movb	%al, 0x110(%rbp)
1409a2b03:     	movl	0x15c(%rbp), %r14d
1409a2b0a:     	movl	%r14d, (%r15)
1409a2b0d:     	movb	%sil, -0x58(%rbp)
1409a2b11:     	movq	%r13, %rcx
1409a2b14:     	movq	%r12, %rdx
1409a2b17:     	leaq	-0x58(%rbp), %rdi
1409a2b1b:     	movq	%rdi, %r8
1409a2b1e:     	callq	0x140aa99e0 <.text+0xaa89e0>
1409a2b23:     	movl	%r14d, (%r15)
1409a2b26:     	movq	%rdi, %r14
1409a2b29:     	movb	%sil, -0x58(%rbp)
1409a2b2d:     	movq	%r13, %rcx
1409a2b30:     	movq	%r12, %rdx
1409a2b33:     	movq	%rdi, %r8
1409a2b36:     	callq	0x140aa9eb0 <.text+0xaa8eb0>
1409a2b3b:     	movq	%rbx, %rax
1409a2b3e:     	cmpb	$0x1, 0x110(%rbp)
1409a2b45:     	movq	0x118(%rbp), %rdi
1409a2b4c:     	jne	0x1409a2a80 <.text+0x9a1a80>
1409a2b52:     	movq	$0x0, 0x140(%rbp)
1409a2b5d:     	movq	$0x8, 0x148(%rbp)
1409a2b68:     	movq	$0x0, 0x150(%rbp)
1409a2b73:     	movq	%r12, -0x50(%rbp)
1409a2b77:     	movb	0x100(%rbp), %dil
1409a2b7e:     	movb	%dil, -0x48(%rbp)
1409a2b82:     	movb	%sil, -0x47(%rbp)
1409a2b86:     	movabsq	$-0x7ffffffffffffff6, %rax # imm = 0x800000000000000A
1409a2b90:     	movq	%rax, -0x58(%rbp)
1409a2b94:     	movb	$0x1, 0x167(%rbp)
1409a2b9b:     	leaq	0x140(%rbp), %rcx
1409a2ba2:     	movq	%r14, %rdx
1409a2ba5:     	callq	0x140a82b40 <.text+0xa81b40>
1409a2baa:     	movq	%r12, -0x50(%rbp)
1409a2bae:     	movb	%dil, -0x48(%rbp)
1409a2bb2:     	movb	%sil, -0x47(%rbp)
1409a2bb6:     	movabsq	$-0x7ffffffffffffff9, %rax # imm = 0x8000000000000007
1409a2bc0:     	movq	%rax, -0x58(%rbp)
1409a2bc4:     	movb	$0x1, 0x167(%rbp)
1409a2bcb:     	leaq	0x140(%rbp), %rcx
1409a2bd2:     	movq	%r14, %rdx
1409a2bd5:     	callq	0x140a82b40 <.text+0xa81b40>
1409a2bda:     	movl	0x160(%rbp), %eax
1409a2be0:     	movq	0xd8(%rbp), %rcx
1409a2be7:     	movl	%eax, (%rcx)
1409a2be9:     	movdqu	0x140(%rbp), %xmm0
1409a2bf1:     	movdqa	%xmm0, 0x120(%rbp)
1409a2bf9:     	movq	0x150(%rbp), %rax
1409a2c00:     	movq	0x120(%rbp), %rcx
1409a2c07:     	movq	0x128(%rbp), %rdx
1409a2c0e:     	movq	$0x0, 0x120(%rbp)
1409a2c19:     	movq	$0x8, 0x128(%rbp)
1409a2c24:     	movq	$0x0, 0x130(%rbp)
1409a2c2f:     	leaq	(%rax,%rax,2), %rax
1409a2c33:     	leaq	(%rdx,%rax,8), %rax
1409a2c37:     	movq	%rdx, -0x58(%rbp)
1409a2c3b:     	movq	%rdx, -0x50(%rbp)
1409a2c3f:     	movq	%rcx, -0x48(%rbp)
1409a2c43:     	movq	%rax, -0x40(%rbp)
1409a2c47:     	movq	%r14, %rcx
1409a2c4a:     	callq	0x141696f50 <.text+0x1695f50>
1409a2c4f:     	movq	%rax, -0x50(%rbp)
1409a2c53:     	movq	%rdx, -0x48(%rbp)
1409a2c57:     	movq	$0x7, -0x58(%rbp)
1409a2c5f:     	movb	$0x0, 0x167(%rbp)
1409a2c66:     	movq	0xe0(%rbp), %rcx
1409a2c6d:     	movq	%r14, %rdx
1409a2c70:     	leaq	0x2119c99(%rip), %r8    # 0x142abc910
1409a2c77:     	callq	0x140a84ec0 <.text+0xa83ec0>
1409a2c7c:     	movq	%rbx, %rax
1409a2c7f:     	movq	0x108(%rbp), %rdi
1409a2c86:     	jmp	0x1409a2a80 <.text+0x9a1a80>
1409a2c8b:     	movaps	0x170(%rbp), %xmm6
1409a2c92:     	movaps	0x180(%rbp), %xmm7
1409a2c99:     	movaps	0x190(%rbp), %xmm8
1409a2ca1:     	movaps	0x1a0(%rbp), %xmm9
1409a2ca9:     	movaps	0x1b0(%rbp), %xmm10
1409a2cb1:     	movaps	0x1c0(%rbp), %xmm11
1409a2cb9:     	addq	$0x258, %rsp            # imm = 0x258
1409a2cc0:     	popq	%rbx
1409a2cc1:     	popq	%rdi
1409a2cc2:     	popq	%rsi
1409a2cc3:     	popq	%r12
1409a2cc5:     	popq	%r13
1409a2cc7:     	popq	%r14
1409a2cc9:     	popq	%r15
1409a2ccb:     	popq	%rbp
1409a2ccc:     	retq
1409a2ccd:     	leaq	0x21374bf(%rip), %rcx   # 0x142ada193
1409a2cd4:     	leaq	0x2119c7d(%rip), %r8    # 0x142abc958
1409a2cdb:     	movl	$0x32, %edx
1409a2ce0:     	callq	0x1428d9430 <.text+0x28d8430>
1409a2ce5:     	leaq	0x22c0984(%rip), %r8    # 0x142c63670
1409a2cec:     	movq	%rax, %rcx
1409a2cef:     	callq	0x1428d9518 <.text+0x28d8518>
1409a2cf4:     	leaq	0x22c095d(%rip), %r8    # 0x142c63658
1409a2cfb:     	callq	0x1428d9518 <.text+0x28d8518>
1409a2d00:     	int3
1409a2d01:     	nopw	%cs:(%rax,%rax)
1409a2d10:     	movq	%rdx, 0x10(%rsp)
1409a2d15:     	pushq	%rbp
1409a2d16:     	pushq	%r15
1409a2d18:     	pushq	%r14
1409a2d1a:     	pushq	%r13
1409a2d1c:     	pushq	%r12
1409a2d1e:     	pushq	%rsi
1409a2d1f:     	pushq	%rdi
1409a2d20:     	pushq	%rbx
1409a2d21:     	subq	$0x88, %rsp
1409a2d28:     	leaq	0x80(%rdx), %rbp
1409a2d2f:     	movaps	%xmm11, 0x20(%rsp)
1409a2d35:     	movaps	%xmm10, 0x30(%rsp)
1409a2d3b:     	movaps	%xmm9, 0x40(%rsp)
