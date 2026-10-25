
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fbb410:     	pushq	%r15
140fbb412:     	pushq	%r14
140fbb414:     	pushq	%r13
140fbb416:     	pushq	%r12
140fbb418:     	pushq	%rsi
140fbb419:     	pushq	%rdi
140fbb41a:     	pushq	%rbx
140fbb41b:     	subq	$0x100, %rsp            # imm = 0x100
140fbb422:     	movq	%rcx, %rsi
140fbb425:     	movq	%rdx, 0xf0(%rsp)
140fbb42d:     	movq	%r8, 0xf8(%rsp)
140fbb435:     	movq	0x18(%rcx), %r12
140fbb439:     	leaq	0xf0(%rsp), %rdi
140fbb441:     	movq	%rdi, %rcx
140fbb444:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fbb449:     	movq	%rax, 0x48(%rsp)
140fbb44e:     	movabsq	$0x41223169ff28813b, %r14 # imm = 0x41223169FF28813B
140fbb458:     	movq	%r14, 0x50(%rsp)
140fbb45d:     	movabsq	$-0x58648d975d569727, %r15 # imm = 0xA79B7268A2A968D9
140fbb467:     	movq	%r15, 0x58(%rsp)
140fbb46c:     	leaq	0x48(%rsp), %rcx
140fbb471:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fbb476:     	movq	%r12, 0x48(%rsp)
140fbb47b:     	movq	%rax, 0x50(%rsp)
140fbb480:     	leaq	0x48(%rsp), %rcx
140fbb485:     	callq	0x1405bd910 <.text+0x5bc910>
140fbb48a:     	movq	%rax, 0x48(%rsp)
140fbb48f:     	movq	0x10(%rsi), %r13
140fbb493:     	cmpq	$0x0, 0x1b8(%r13)
140fbb49b:     	je	0x140fbb55e <.text+0xfba55e>
140fbb4a1:     	movq	%rax, %rbx
140fbb4a4:     	leaq	0x1c0(%r13), %rcx
140fbb4ab:     	leaq	0x48(%rsp), %rdx
140fbb4b0:     	callq	0x1405b0d70 <.text+0x5afd70>
140fbb4b5:     	movq	0x1a0(%r13), %rcx
140fbb4bc:     	movq	0x1a8(%r13), %rdx
140fbb4c3:     	movq	%rdx, %r8
140fbb4c6:     	andq	%rax, %r8
140fbb4c9:     	shrq	$0x39, %rax
140fbb4cd:     	movd	%eax, %xmm0
140fbb4d1:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140fbb4d5:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140fbb4da:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140fbb4df:     	leaq	-0xc0(%rcx), %rax
140fbb4e6:     	xorl	%r9d, %r9d
140fbb4e9:     	pcmpeqd	%xmm1, %xmm1
140fbb4ed:     	movdqu	(%rcx,%r8), %xmm2
140fbb4f3:     	movdqa	%xmm2, %xmm3
140fbb4f7:     	pcmpeqb	%xmm0, %xmm3
140fbb4fb:     	pmovmskb	%xmm3, %r10d
140fbb500:     	testl	%r10d, %r10d
140fbb503:     	je	0x140fbb540 <.text+0xfba540>
140fbb505:     	tzcntl	%r10d, %r11d
140fbb50a:     	addq	%r8, %r11
140fbb50d:     	andq	%rdx, %r11
140fbb510:     	negq	%r11
140fbb513:     	leaq	(%r11,%r11,2), %r11
140fbb517:     	shlq	$0x6, %r11
140fbb51b:     	cmpq	%rbx, (%rax,%r11)
140fbb51f:     	je	0x140fbb73e <.text+0xfba73e>
140fbb525:     	leal	-0x1(%r10), %r11d
140fbb529:     	andw	%r10w, %r11w
140fbb52d:     	movl	%r11d, %r10d
140fbb530:     	jne	0x140fbb505 <.text+0xfba505>
140fbb532:     	nopw	%cs:(%rax,%rax)
140fbb540:     	pcmpeqb	%xmm1, %xmm2
140fbb544:     	pmovmskb	%xmm2, %r10d
140fbb549:     	testl	%r10d, %r10d
140fbb54c:     	jne	0x140fbb55e <.text+0xfba55e>
140fbb54e:     	addq	%r9, %r8
140fbb551:     	addq	$0x10, %r8
140fbb555:     	addq	$0x10, %r9
140fbb559:     	andq	%rdx, %r8
140fbb55c:     	jmp	0x140fbb4ed <.text+0xfba4ed>
140fbb55e:     	leaq	0x10(%rsi), %rbx
140fbb562:     	movq	0x2670e3f(%rip), %rax   # 0x14362c3a8
140fbb569:     	movq	(%rax), %rax
140fbb56c:     	testq	%rax, %rax
140fbb56f:     	je	0x140fbb655 <.text+0xfba655>
140fbb575:     	movq	%rsi, 0x20(%rsp)
140fbb57a:     	leaq	-0xa87ce1(%rip), %rax   # 0x1405338a0 <.text+0x5328a0>
140fbb581:     	movq	%rax, 0x28(%rsp)
140fbb586:     	movq	%rdi, 0x30(%rsp)
140fbb58b:     	movq	%rax, 0x38(%rsp)
140fbb590:     	leaq	0x1bf88f9(%rip), %rcx   # 0x142bb3e90
140fbb597:     	callq	0x14067dd80 <.text+0x67cd80>
140fbb59c:     	movdqu	(%rax), %xmm0
140fbb5a0:     	movl	0x10(%rax), %eax
140fbb5a3:     	movq	$0x1, 0x78(%rsp)
140fbb5ac:     	leaq	0x1bf885d(%rip), %rcx   # 0x142bb3e10
140fbb5b3:     	movq	%rcx, 0x80(%rsp)
140fbb5bb:     	movq	$0x14, 0x88(%rsp)
140fbb5c7:     	leaq	0x1bf8872(%rip), %rdx   # 0x142bb3e40
140fbb5ce:     	movq	%rdx, 0x98(%rsp)
140fbb5d6:     	movq	$0x2, 0xa0(%rsp)
140fbb5e2:     	leaq	0x20(%rsp), %rdx
140fbb5e7:     	movq	%rdx, 0xa8(%rsp)
140fbb5ef:     	movq	$0x2, 0xb0(%rsp)
140fbb5fb:     	movq	$0x0, 0xb8(%rsp)
140fbb607:     	movq	$0x0, 0x48(%rsp)
140fbb610:     	movq	%rcx, 0x50(%rsp)
140fbb615:     	movq	$0x14, 0x58(%rsp)
140fbb61e:     	movq	$0x0, 0x60(%rsp)
140fbb627:     	movdqu	%xmm0, 0x68(%rsp)
140fbb62d:     	movl	$0x1, 0x90(%rsp)
140fbb638:     	movl	%eax, 0x94(%rsp)
140fbb63f:     	leaq	0xd8(%rsp), %rcx
140fbb647:     	leaq	0x48(%rsp), %rdx
140fbb64c:     	callq	0x140d4a330 <.text+0xd49330>
140fbb651:     	movq	0x18(%rsi), %r12
140fbb655:     	movdqu	0xf0(%rsp), %xmm0
140fbb65e:     	movdqu	%xmm0, 0x20(%rsp)
140fbb664:     	leaq	0x20(%rsp), %rcx
140fbb669:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fbb66e:     	movq	%rax, 0x48(%rsp)
140fbb673:     	movq	%r14, 0x50(%rsp)
140fbb678:     	movq	%r15, 0x58(%rsp)
140fbb67d:     	leaq	0x48(%rsp), %rcx
140fbb682:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fbb687:     	movq	%r12, 0x48(%rsp)
140fbb68c:     	movq	%rax, 0x50(%rsp)
140fbb691:     	leaq	0x48(%rsp), %rcx
140fbb696:     	callq	0x1405bd910 <.text+0x5bc910>
140fbb69b:     	leaq	0xd8(%rsp), %r14
140fbb6a3:     	movq	%r14, %rcx
140fbb6a6:     	movq	%rbx, %rdx
140fbb6a9:     	movq	%rax, %r8
140fbb6ac:     	callq	0x1406861b0 <.text+0x6851b0>
140fbb6b1:     	leaq	0x1bf8837(%rip), %rcx   # 0x142bb3eef
140fbb6b8:     	movl	$0x13, %edx
140fbb6bd:     	callq	0x140b9d480 <.text+0xb9c480>
140fbb6c2:     	movq	%rax, %rsi
140fbb6c5:     	movq	%rdx, %rdi
140fbb6c8:     	movq	%rax, 0xc8(%rsp)
140fbb6d0:     	movq	%rdx, 0xd0(%rsp)
140fbb6d8:     	leaq	0xc8(%rsp), %rcx
140fbb6e0:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fbb6e5:     	movq	%rax, 0x48(%rsp)
140fbb6ea:     	movabsq	$-0x239fc26ae46c7f4e, %rax # imm = 0xDC603D951B9380B2
140fbb6f4:     	movq	%rax, 0x50(%rsp)
140fbb6f9:     	movabsq	$-0x3196240f511ec25e, %rax # imm = 0xCE69DBF0AEE13DA2
140fbb703:     	movq	%rax, 0x58(%rsp)
140fbb708:     	leaq	0x48(%rsp), %rcx
140fbb70d:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fbb712:     	movq	0xe0(%rsp), %rcx
140fbb71a:     	movq	%rcx, 0x48(%rsp)
140fbb71f:     	movq	%rax, 0x50(%rsp)
140fbb724:     	leaq	0x48(%rsp), %rcx
140fbb729:     	callq	0x1405bd910 <.text+0x5bc910>
140fbb72e:     	leaq	0x48(%rsp), %rbx
140fbb733:     	movq	%rbx, %rcx
140fbb736:     	movq	%r14, %rdx
140fbb739:     	jmp	0x140fbb7df <.text+0xfba7df>
140fbb73e:     	movq	%r13, 0xd8(%rsp)
140fbb746:     	movq	%rbx, 0xe0(%rsp)
140fbb74e:     	movq	$0x0, 0xe8(%rsp)
140fbb75a:     	leaq	0x1bf878e(%rip), %rcx   # 0x142bb3eef
140fbb761:     	movl	$0x13, %edx
140fbb766:     	callq	0x140b9d480 <.text+0xb9c480>
140fbb76b:     	movq	%rax, %rsi
140fbb76e:     	movq	%rdx, %rdi
140fbb771:     	movq	%rax, 0xc8(%rsp)
140fbb779:     	movq	%rdx, 0xd0(%rsp)
140fbb781:     	leaq	0xc8(%rsp), %rcx
140fbb789:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fbb78e:     	movq	%rax, 0x48(%rsp)
140fbb793:     	movabsq	$-0x239fc26ae46c7f4e, %rax # imm = 0xDC603D951B9380B2
140fbb79d:     	movq	%rax, 0x50(%rsp)
140fbb7a2:     	movabsq	$-0x3196240f511ec25e, %rax # imm = 0xCE69DBF0AEE13DA2
140fbb7ac:     	movq	%rax, 0x58(%rsp)
140fbb7b1:     	leaq	0x48(%rsp), %rcx
140fbb7b6:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fbb7bb:     	movq	%rbx, 0x48(%rsp)
140fbb7c0:     	movq	%rax, 0x50(%rsp)
140fbb7c5:     	leaq	0x48(%rsp), %rcx
140fbb7ca:     	callq	0x1405bd910 <.text+0x5bc910>
140fbb7cf:     	leaq	0x48(%rsp), %rbx
140fbb7d4:     	leaq	0xd8(%rsp), %rdx
140fbb7dc:     	movq	%rbx, %rcx
140fbb7df:     	movq	%rax, %r8
140fbb7e2:     	callq	0x1406861b0 <.text+0x6851b0>
140fbb7e7:     	movq	%rbx, %rcx
140fbb7ea:     	xorl	%edx, %edx
140fbb7ec:     	callq	0x140f9d930 <.text+0xf9c930>
140fbb7f1:     	movq	%rax, %rcx
140fbb7f4:     	movq	%rsi, %rdx
140fbb7f7:     	movq	%rdi, %r8
140fbb7fa:     	callq	0x140fac4c0 <.text+0xfab4c0>
140fbb7ff:     	movq	0x58(%rsp), %rax
140fbb804:     	movq	%rax, 0x30(%rsp)
140fbb809:     	movups	0x48(%rsp), %xmm0
140fbb80e:     	movaps	%xmm0, 0x20(%rsp)
140fbb813:     	leaq	0x20(%rsp), %rcx
140fbb818:     	movb	$0x1, %dl
140fbb81a:     	callq	0x140f9d930 <.text+0xf9c930>
140fbb81f:     	movl	$0x7b5a51a0, 0x258(%rax) # imm = 0x7B5A51A0
140fbb829:     	addq	$0x100, %rsp            # imm = 0x100
140fbb830:     	popq	%rbx
140fbb831:     	popq	%rdi
140fbb832:     	popq	%rsi
140fbb833:     	popq	%r12
140fbb835:     	popq	%r13
140fbb837:     	popq	%r14
140fbb839:     	popq	%r15
140fbb83b:     	retq
