
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fbe1a0:     	pushq	%r15
140fbe1a2:     	pushq	%r14
140fbe1a4:     	pushq	%r13
140fbe1a6:     	pushq	%r12
140fbe1a8:     	pushq	%rsi
140fbe1a9:     	pushq	%rdi
140fbe1aa:     	pushq	%rbx
140fbe1ab:     	subq	$0x100, %rsp            # imm = 0x100
140fbe1b2:     	movq	%rcx, %rsi
140fbe1b5:     	movq	%rdx, 0xf0(%rsp)
140fbe1bd:     	movq	%r8, 0xf8(%rsp)
140fbe1c5:     	movq	0x18(%rcx), %r12
140fbe1c9:     	leaq	0xf0(%rsp), %rdi
140fbe1d1:     	movq	%rdi, %rcx
140fbe1d4:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fbe1d9:     	movq	%rax, 0x40(%rsp)
140fbe1de:     	movabsq	$0x41223169ff28813b, %r14 # imm = 0x41223169FF28813B
140fbe1e8:     	movq	%r14, 0x48(%rsp)
140fbe1ed:     	movabsq	$-0x58648d975d569727, %r15 # imm = 0xA79B7268A2A968D9
140fbe1f7:     	movq	%r15, 0x50(%rsp)
140fbe1fc:     	leaq	0x40(%rsp), %rcx
140fbe201:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fbe206:     	movq	%r12, 0x40(%rsp)
140fbe20b:     	movq	%rax, 0x48(%rsp)
140fbe210:     	leaq	0x40(%rsp), %rcx
140fbe215:     	callq	0x1405bd910 <.text+0x5bc910>
140fbe21a:     	movq	%rax, 0x40(%rsp)
140fbe21f:     	movq	0x10(%rsi), %r13
140fbe223:     	cmpq	$0x0, 0x1b8(%r13)
140fbe22b:     	je	0x140fbe2ee <.text+0xfbd2ee>
140fbe231:     	movq	%rax, %rbx
140fbe234:     	leaq	0x1c0(%r13), %rcx
140fbe23b:     	leaq	0x40(%rsp), %rdx
140fbe240:     	callq	0x1405b0d70 <.text+0x5afd70>
140fbe245:     	movq	0x1a0(%r13), %rcx
140fbe24c:     	movq	0x1a8(%r13), %rdx
140fbe253:     	movq	%rdx, %r8
140fbe256:     	andq	%rax, %r8
140fbe259:     	shrq	$0x39, %rax
140fbe25d:     	movd	%eax, %xmm0
140fbe261:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fbe265:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fbe26a:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fbe26f:     	leaq	-0xc0(%rcx), %rax
140fbe276:     	xorl	%r9d, %r9d
140fbe279:     	pcmpeqd	%xmm1, %xmm1
140fbe27d:     	movdqu	(%rcx,%r8), %xmm2
140fbe283:     	movdqa	%xmm2, %xmm3
140fbe287:     	pcmpeqb	%xmm0, %xmm3
140fbe28b:     	pmovmskb	%xmm3, %r10d
140fbe290:     	testl	%r10d, %r10d
140fbe293:     	je	0x140fbe2d0 <.text+0xfbd2d0>
140fbe295:     	tzcntl	%r10d, %r11d
140fbe29a:     	addq	%r8, %r11
140fbe29d:     	andq	%rdx, %r11
140fbe2a0:     	negq	%r11
140fbe2a3:     	leaq	(%r11,%r11,2), %r11
140fbe2a7:     	shlq	$0x6, %r11
140fbe2ab:     	cmpq	%rbx, (%rax,%r11)
140fbe2af:     	je	0x140fbe622 <.text+0xfbd622>
140fbe2b5:     	leal	-0x1(%r10), %r11d
140fbe2b9:     	andw	%r10w, %r11w
140fbe2bd:     	movl	%r11d, %r10d
140fbe2c0:     	jne	0x140fbe295 <.text+0xfbd295>
140fbe2c2:     	nopw	%cs:(%rax,%rax)
140fbe2d0:     	pcmpeqb	%xmm1, %xmm2
140fbe2d4:     	pmovmskb	%xmm2, %r10d
140fbe2d9:     	testl	%r10d, %r10d
140fbe2dc:     	jne	0x140fbe2ee <.text+0xfbd2ee>
140fbe2de:     	addq	%r9, %r8
140fbe2e1:     	addq	$0x10, %r8
140fbe2e5:     	addq	$0x10, %r9
140fbe2e9:     	andq	%rdx, %r8
140fbe2ec:     	jmp	0x140fbe27d <.text+0xfbd27d>
140fbe2ee:     	leaq	0x10(%rsi), %rbx
140fbe2f2:     	movq	0x266e0af(%rip), %rax   # 0x14362c3a8
140fbe2f9:     	movq	(%rax), %rax
140fbe2fc:     	testq	%rax, %rax
140fbe2ff:     	je	0x140fbe3e2 <.text+0xfbd3e2>
140fbe305:     	movq	%rsi, 0x20(%rsp)
140fbe30a:     	leaq	-0xa8aa71(%rip), %rax   # 0x1405338a0 <.text+0x5328a0>
140fbe311:     	movq	%rax, 0x28(%rsp)
140fbe316:     	movq	%rdi, 0x30(%rsp)
140fbe31b:     	movq	%rax, 0x38(%rsp)
140fbe320:     	leaq	0x1bf5b69(%rip), %rcx   # 0x142bb3e90
140fbe327:     	callq	0x14067dd80 <.text+0x67cd80>
140fbe32c:     	movdqu	(%rax), %xmm0
140fbe330:     	movl	0x10(%rax), %eax
140fbe333:     	movq	$0x1, 0x70(%rsp)
140fbe33c:     	leaq	0x1bf5acd(%rip), %rcx   # 0x142bb3e10
140fbe343:     	movq	%rcx, 0x78(%rsp)
140fbe348:     	movq	$0x14, 0x80(%rsp)
140fbe354:     	leaq	0x1bf5ae5(%rip), %rdx   # 0x142bb3e40
140fbe35b:     	movq	%rdx, 0x90(%rsp)
140fbe363:     	movq	$0x2, 0x98(%rsp)
140fbe36f:     	leaq	0x20(%rsp), %rdx
140fbe374:     	movq	%rdx, 0xa0(%rsp)
140fbe37c:     	movq	$0x2, 0xa8(%rsp)
140fbe388:     	movq	$0x0, 0xb0(%rsp)
140fbe394:     	movq	$0x0, 0x40(%rsp)
140fbe39d:     	movq	%rcx, 0x48(%rsp)
140fbe3a2:     	movq	$0x14, 0x50(%rsp)
140fbe3ab:     	movq	$0x0, 0x58(%rsp)
140fbe3b4:     	movdqu	%xmm0, 0x60(%rsp)
140fbe3ba:     	movl	$0x1, 0x88(%rsp)
140fbe3c5:     	movl	%eax, 0x8c(%rsp)
140fbe3cc:     	leaq	0xd8(%rsp), %rcx
140fbe3d4:     	leaq	0x40(%rsp), %rdx
140fbe3d9:     	callq	0x140d4a330 <.text+0xd49330>
140fbe3de:     	movq	0x18(%rsi), %r12
140fbe3e2:     	movups	0xf0(%rsp), %xmm0
140fbe3ea:     	movups	%xmm0, 0x20(%rsp)
140fbe3ef:     	leaq	0x20(%rsp), %rcx
140fbe3f4:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fbe3f9:     	movq	%rax, 0x40(%rsp)
140fbe3fe:     	movq	%r14, 0x48(%rsp)
140fbe403:     	movq	%r15, 0x50(%rsp)
140fbe408:     	leaq	0x40(%rsp), %rcx
140fbe40d:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fbe412:     	movq	%r12, 0x40(%rsp)
140fbe417:     	movq	%rax, 0x48(%rsp)
140fbe41c:     	leaq	0x40(%rsp), %rcx
140fbe421:     	callq	0x1405bd910 <.text+0x5bc910>
140fbe426:     	leaq	0xd8(%rsp), %rdi
140fbe42e:     	movq	%rdi, %rcx
140fbe431:     	movq	%rbx, %rdx
140fbe434:     	movq	%rax, %r8
140fbe437:     	callq	0x1406861b0 <.text+0x6851b0>
140fbe43c:     	leaq	0x1bf5a9d(%rip), %rsi   # 0x142bb3ee0
140fbe443:     	movq	%rsi, 0xc8(%rsp)
140fbe44b:     	movq	$0xc, 0xd0(%rsp)
140fbe457:     	leaq	0xc8(%rsp), %rcx
140fbe45f:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fbe464:     	movq	%rax, 0x40(%rsp)
140fbe469:     	movabsq	$-0x239fc26ae46c7f4e, %rax # imm = 0xDC603D951B9380B2
140fbe473:     	movq	%rax, 0x48(%rsp)
140fbe478:     	movabsq	$-0x3196240f511ec25e, %rax # imm = 0xCE69DBF0AEE13DA2
140fbe482:     	movq	%rax, 0x50(%rsp)
140fbe487:     	leaq	0x40(%rsp), %rcx
140fbe48c:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fbe491:     	movq	0xe0(%rsp), %rcx
140fbe499:     	movq	%rcx, 0x40(%rsp)
140fbe49e:     	movq	%rax, 0x48(%rsp)
140fbe4a3:     	leaq	0x40(%rsp), %rcx
140fbe4a8:     	callq	0x1405bd910 <.text+0x5bc910>
140fbe4ad:     	leaq	0x40(%rsp), %rbx
140fbe4b2:     	movq	%rbx, %rcx
140fbe4b5:     	movq	%rdi, %rdx
140fbe4b8:     	movq	%rax, %r8
140fbe4bb:     	callq	0x1406861b0 <.text+0x6851b0>
140fbe4c0:     	movq	%rbx, %rcx
140fbe4c3:     	xorl	%edx, %edx
140fbe4c5:     	callq	0x140f9d930 <.text+0xf9c930>
140fbe4ca:     	movl	$0xc, %r8d
140fbe4d0:     	movq	%rax, %rcx
140fbe4d3:     	movq	%rsi, %rdx
140fbe4d6:     	callq	0x140fac4c0 <.text+0xfab4c0>
140fbe4db:     	movq	0x50(%rsp), %rax
140fbe4e0:     	movq	%rax, 0x30(%rsp)
140fbe4e5:     	movups	0x40(%rsp), %xmm0
140fbe4ea:     	movaps	%xmm0, 0x20(%rsp)
140fbe4ef:     	leaq	0x20(%rsp), %rcx
140fbe4f4:     	movb	$0x1, %dl
140fbe4f6:     	callq	0x140f9d930 <.text+0xf9c930>
140fbe4fb:     	movd	0x1a0eb6d(%rip), %xmm1  # 0x1429cd070
140fbe503:     	movq	%rax, %rcx
140fbe506:     	callq	0x140fac150 <.text+0xfab150>
140fbe50b:     	movq	0x30(%rsp), %rax
140fbe510:     	movq	%rax, 0x50(%rsp)
140fbe515:     	movaps	0x20(%rsp), %xmm0
140fbe51a:     	movaps	%xmm0, 0x40(%rsp)
140fbe51f:     	leaq	0x40(%rsp), %rcx
140fbe524:     	movb	$0x1, %dl
140fbe526:     	callq	0x140f9d930 <.text+0xf9c930>
140fbe52b:     	movq	%rax, %rcx
140fbe52e:     	movw	$0x320, %dx             # imm = 0x320
140fbe532:     	callq	0x140b90ea0 <.text+0xb8fea0>
140fbe537:     	movdqa	0x40(%rsp), %xmm0
140fbe53d:     	movdqa	%xmm0, 0x20(%rsp)
140fbe543:     	movq	0x50(%rsp), %rax
140fbe548:     	movq	%rax, 0x30(%rsp)
140fbe54d:     	movq	0x20(%rsp), %rsi
140fbe552:     	cmpq	$0x0, 0x1b8(%rsi)
140fbe55a:     	je	0x140fbe822 <.text+0xfbd822>
140fbe560:     	leaq	0x28(%rsp), %rdx
140fbe565:     	leaq	0x1c0(%rsi), %rcx
140fbe56c:     	callq	0x1405b0d70 <.text+0x5afd70>
140fbe571:     	movq	0x1a0(%rsi), %rcx
140fbe578:     	movq	0x1a8(%rsi), %rdx
140fbe57f:     	movq	%rdx, %r8
140fbe582:     	andq	%rax, %r8
140fbe585:     	shrq	$0x39, %rax
140fbe589:     	movd	%eax, %xmm0
140fbe58d:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fbe591:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fbe596:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fbe59b:     	leaq	-0xc0(%rcx), %rax
140fbe5a2:     	xorl	%r9d, %r9d
140fbe5a5:     	movq	0x28(%rsp), %r10
140fbe5aa:     	pcmpeqd	%xmm1, %xmm1
140fbe5ae:     	movdqu	(%rcx,%r8), %xmm2
140fbe5b4:     	movdqa	%xmm2, %xmm3
140fbe5b8:     	pcmpeqb	%xmm0, %xmm3
140fbe5bc:     	pmovmskb	%xmm3, %r11d
140fbe5c1:     	testl	%r11d, %r11d
140fbe5c4:     	je	0x140fbe600 <.text+0xfbd600>
140fbe5c6:     	tzcntl	%r11d, %esi
140fbe5cb:     	addq	%r8, %rsi
140fbe5ce:     	andq	%rdx, %rsi
140fbe5d1:     	negq	%rsi
140fbe5d4:     	leaq	(%rsi,%rsi,2), %rsi
140fbe5d8:     	shlq	$0x6, %rsi
140fbe5dc:     	cmpq	(%rax,%rsi), %r10
140fbe5e0:     	je	0x140fbe80a <.text+0xfbd80a>
140fbe5e6:     	leal	-0x1(%r11), %esi
140fbe5ea:     	andw	%r11w, %si
140fbe5ee:     	movl	%esi, %r11d
140fbe5f1:     	jne	0x140fbe5c6 <.text+0xfbd5c6>
140fbe5f3:     	nopw	%cs:(%rax,%rax)
140fbe600:     	pcmpeqb	%xmm1, %xmm2
140fbe604:     	pmovmskb	%xmm2, %r11d
140fbe609:     	testl	%r11d, %r11d
140fbe60c:     	jne	0x140fbe822 <.text+0xfbd822>
140fbe612:     	addq	%r9, %r8
140fbe615:     	addq	$0x10, %r8
140fbe619:     	addq	$0x10, %r9
140fbe61d:     	andq	%rdx, %r8
140fbe620:     	jmp	0x140fbe5ae <.text+0xfbd5ae>
140fbe622:     	movq	%r13, 0xd8(%rsp)
140fbe62a:     	movq	%rbx, 0xe0(%rsp)
140fbe632:     	movq	$0x0, 0xe8(%rsp)
140fbe63e:     	leaq	0x1bf589b(%rip), %rsi   # 0x142bb3ee0
140fbe645:     	movq	%rsi, 0xc8(%rsp)
140fbe64d:     	movq	$0xc, 0xd0(%rsp)
140fbe659:     	leaq	0xc8(%rsp), %rcx
140fbe661:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fbe666:     	movq	%rax, 0x40(%rsp)
140fbe66b:     	movabsq	$-0x239fc26ae46c7f4e, %rax # imm = 0xDC603D951B9380B2
140fbe675:     	movq	%rax, 0x48(%rsp)
140fbe67a:     	movabsq	$-0x3196240f511ec25e, %rax # imm = 0xCE69DBF0AEE13DA2
140fbe684:     	movq	%rax, 0x50(%rsp)
140fbe689:     	leaq	0x40(%rsp), %rcx
140fbe68e:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fbe693:     	movq	%rbx, 0x40(%rsp)
140fbe698:     	movq	%rax, 0x48(%rsp)
140fbe69d:     	leaq	0x40(%rsp), %rcx
140fbe6a2:     	callq	0x1405bd910 <.text+0x5bc910>
140fbe6a7:     	leaq	0x40(%rsp), %rdi
140fbe6ac:     	leaq	0xd8(%rsp), %rdx
140fbe6b4:     	movq	%rdi, %rcx
140fbe6b7:     	movq	%rax, %r8
140fbe6ba:     	callq	0x1406861b0 <.text+0x6851b0>
140fbe6bf:     	movq	%rdi, %rcx
140fbe6c2:     	xorl	%edx, %edx
140fbe6c4:     	callq	0x140f9d930 <.text+0xf9c930>
140fbe6c9:     	movl	$0xc, %r8d
140fbe6cf:     	movq	%rax, %rcx
140fbe6d2:     	movq	%rsi, %rdx
140fbe6d5:     	callq	0x140fac4c0 <.text+0xfab4c0>
140fbe6da:     	movq	0x50(%rsp), %rax
140fbe6df:     	movq	%rax, 0x30(%rsp)
140fbe6e4:     	movups	0x40(%rsp), %xmm0
140fbe6e9:     	movaps	%xmm0, 0x20(%rsp)
140fbe6ee:     	leaq	0x20(%rsp), %rcx
140fbe6f3:     	movb	$0x1, %dl
140fbe6f5:     	callq	0x140f9d930 <.text+0xf9c930>
140fbe6fa:     	movd	0x1a0e96e(%rip), %xmm1  # 0x1429cd070
140fbe702:     	movq	%rax, %rcx
140fbe705:     	callq	0x140fac150 <.text+0xfab150>
140fbe70a:     	movq	0x30(%rsp), %rax
140fbe70f:     	movq	%rax, 0x50(%rsp)
140fbe714:     	movaps	0x20(%rsp), %xmm0
140fbe719:     	movaps	%xmm0, 0x40(%rsp)
140fbe71e:     	leaq	0x40(%rsp), %rcx
140fbe723:     	movb	$0x1, %dl
140fbe725:     	callq	0x140f9d930 <.text+0xf9c930>
140fbe72a:     	movq	%rax, %rcx
140fbe72d:     	movw	$0x320, %dx             # imm = 0x320
140fbe731:     	callq	0x140b90ea0 <.text+0xb8fea0>
140fbe736:     	movaps	0x40(%rsp), %xmm0
140fbe73b:     	movaps	%xmm0, 0x20(%rsp)
140fbe740:     	movq	0x50(%rsp), %rax
140fbe745:     	movq	%rax, 0x30(%rsp)
140fbe74a:     	movq	0x20(%rsp), %rsi
140fbe74f:     	cmpq	$0x0, 0x1b8(%rsi)
140fbe757:     	je	0x140fbe822 <.text+0xfbd822>
140fbe75d:     	leaq	0x28(%rsp), %rdx
140fbe762:     	leaq	0x1c0(%rsi), %rcx
140fbe769:     	callq	0x1405b0d70 <.text+0x5afd70>
140fbe76e:     	movq	0x1a0(%rsi), %rcx
140fbe775:     	movq	0x1a8(%rsi), %rdx
140fbe77c:     	movq	%rdx, %r8
140fbe77f:     	andq	%rax, %r8
140fbe782:     	shrq	$0x39, %rax
140fbe786:     	movd	%eax, %xmm0
140fbe78a:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fbe78e:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fbe793:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fbe798:     	leaq	-0xc0(%rcx), %rax
140fbe79f:     	xorl	%r9d, %r9d
140fbe7a2:     	movq	0x28(%rsp), %r10
140fbe7a7:     	pcmpeqd	%xmm1, %xmm1
140fbe7ab:     	movdqu	(%rcx,%r8), %xmm2
140fbe7b1:     	movdqa	%xmm2, %xmm3
140fbe7b5:     	pcmpeqb	%xmm0, %xmm3
140fbe7b9:     	pmovmskb	%xmm3, %r11d
140fbe7be:     	testl	%r11d, %r11d
140fbe7c1:     	je	0x140fbe7ec <.text+0xfbd7ec>
140fbe7c3:     	tzcntl	%r11d, %esi
140fbe7c8:     	addq	%r8, %rsi
140fbe7cb:     	andq	%rdx, %rsi
140fbe7ce:     	negq	%rsi
140fbe7d1:     	leaq	(%rsi,%rsi,2), %rsi
140fbe7d5:     	shlq	$0x6, %rsi
140fbe7d9:     	cmpq	(%rax,%rsi), %r10
140fbe7dd:     	je	0x140fbe80a <.text+0xfbd80a>
140fbe7df:     	leal	-0x1(%r11), %esi
140fbe7e3:     	andw	%r11w, %si
140fbe7e7:     	movl	%esi, %r11d
140fbe7ea:     	jne	0x140fbe7c3 <.text+0xfbd7c3>
140fbe7ec:     	pcmpeqb	%xmm1, %xmm2
140fbe7f0:     	pmovmskb	%xmm2, %r11d
140fbe7f5:     	testl	%r11d, %r11d
140fbe7f8:     	jne	0x140fbe822 <.text+0xfbd822>
140fbe7fa:     	addq	%r9, %r8
140fbe7fd:     	addq	$0x10, %r8
140fbe801:     	addq	$0x10, %r9
140fbe805:     	andq	%rdx, %r8
140fbe808:     	jmp	0x140fbe7ab <.text+0xfbd7ab>
140fbe80a:     	orb	$0x2, -0x1(%rcx,%rsi)
140fbe80f:     	addq	$0x100, %rsp            # imm = 0x100
140fbe816:     	popq	%rbx
140fbe817:     	popq	%rdi
140fbe818:     	popq	%rsi
140fbe819:     	popq	%r12
140fbe81b:     	popq	%r13
140fbe81d:     	popq	%r14
140fbe81f:     	popq	%r15
140fbe821:     	retq
140fbe822:     	leaq	0x1a597c7(%rip), %rcx   # 0x142a17ff0
140fbe829:     	callq	0x1428d9310 <.text+0x28d8310>
140fbe82e:     	int3
