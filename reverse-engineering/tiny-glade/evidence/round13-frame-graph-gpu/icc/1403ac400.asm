
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403ac400:     	pushq	%rbp
1403ac401:     	pushq	%r15
1403ac403:     	pushq	%r14
1403ac405:     	pushq	%rsi
1403ac406:     	pushq	%rdi
1403ac407:     	pushq	%rbx
1403ac408:     	subq	$0x1a8, %rsp            # imm = 0x1A8
1403ac40f:     	leaq	0x80(%rsp), %rbp
1403ac417:     	movq	$-0x2, 0x120(%rbp)
1403ac422:     	movq	%rcx, %rsi
1403ac425:     	movq	(%rdx), %rdx
1403ac428:     	leaq	0x70(%rbp), %rcx
1403ac42c:     	callq	0x140697b10 <.text+0x696b10>
1403ac431:     	cmpl	$0x1, 0x70(%rbp)
1403ac435:     	je	0x1403ac944 <.text+0x3ab944>
1403ac43b:     	movq	0xd4(%rbp), %rax
1403ac442:     	movq	%rax, 0x10(%rbp)
1403ac446:     	movups	0xc4(%rbp), %xmm0
1403ac44d:     	movaps	%xmm0, (%rbp)
1403ac451:     	movups	0xb4(%rbp), %xmm0
1403ac458:     	movaps	%xmm0, -0x10(%rbp)
1403ac45c:     	movups	0x74(%rbp), %xmm0
1403ac460:     	movups	0x84(%rbp), %xmm1
1403ac467:     	movups	0x94(%rbp), %xmm2
1403ac46e:     	movups	0xa4(%rbp), %xmm3
1403ac475:     	movaps	%xmm3, -0x20(%rbp)
1403ac479:     	movaps	%xmm2, -0x30(%rbp)
1403ac47d:     	movaps	%xmm1, -0x40(%rbp)
1403ac481:     	movaps	%xmm0, -0x50(%rbp)
1403ac485:     	leaq	-0x28(%rbp), %rdx
1403ac489:     	leaq	0x70(%rbp), %rdi
1403ac48d:     	movl	$0x20, %r8d
1403ac493:     	movq	%rdi, %rcx
1403ac496:     	callq	0x140695be0 <.text+0x694be0>
1403ac49b:     	movq	%rdi, %rcx
1403ac49e:     	callq	0x140d76ed0 <.text+0xd75ed0>
1403ac4a3:     	movq	%rdx, %r8
1403ac4a6:     	leaq	0x48(%rbp), %rcx
1403ac4aa:     	movq	%rax, %rdx
1403ac4ad:     	callq	0x140d77340 <.text+0xd76340>
1403ac4b2:     	movq	0x50(%rbp), %r15
1403ac4b6:     	movq	0x58(%rbp), %rbx
1403ac4ba:     	testq	%rbx, %rbx
1403ac4bd:     	js	0x1403ac976 <.text+0x3ab976>
1403ac4c3:     	je	0x1403ac4ec <.text+0x3ab4ec>
1403ac4c5:     	movzbl	0x328873d(%rip), %eax   # 0x143634c09
1403ac4cc:     	movl	$0x1, %edi
1403ac4d1:     	movl	$0x1, %edx
1403ac4d6:     	movq	%rbx, %rcx
1403ac4d9:     	callq	0x140613c10 <.text+0x612c10>
1403ac4de:     	testq	%rax, %rax
1403ac4e1:     	je	0x1403ac978 <.text+0x3ab978>
1403ac4e7:     	movq	%rax, %r14
1403ac4ea:     	jmp	0x1403ac4f2 <.text+0x3ab4f2>
1403ac4ec:     	movl	$0x1, %r14d
1403ac4f2:     	movq	%r14, %rcx
1403ac4f5:     	movq	%r15, %rdx
1403ac4f8:     	movq	%rbx, %r8
1403ac4fb:     	callq	0x142923c00 <.text+0x2922c00>
1403ac500:     	movq	0x48(%rbp), %rdx
1403ac504:     	leaq	(,%rdx,2), %rax
1403ac50c:     	testq	%rax, %rax
1403ac50f:     	jne	0x1403ac73d <.text+0x3ab73d>
1403ac515:     	movq	0x70(%rbp), %rdx
1403ac519:     	testq	%rdx, %rdx
1403ac51c:     	je	0x1403ac52d <.text+0x3ab52d>
1403ac51e:     	movq	0x78(%rbp), %rcx
1403ac522:     	movl	$0x1, %r8d
1403ac528:     	callq	0x140613c20 <.text+0x612c20>
1403ac52d:     	cmpq	$0xf, %rbx
1403ac531:     	movq	%rbx, 0xf0(%rbp)
1403ac538:     	movq	%r14, 0xe0(%rbp)
1403ac53f:     	ja	0x1403ac565 <.text+0x3ab565>
1403ac541:     	testq	%rbx, %rbx
1403ac544:     	je	0x1403ac590 <.text+0x3ab590>
1403ac546:     	xorl	%edx, %edx
1403ac548:     	nopl	(%rax,%rax)
1403ac550:     	cmpb	$0x0, (%r14,%rdx)
1403ac555:     	je	0x1403ac904 <.text+0x3ab904>
1403ac55b:     	incq	%rdx
1403ac55e:     	cmpq	%rdx, %rbx
1403ac561:     	jne	0x1403ac550 <.text+0x3ab550>
1403ac563:     	jmp	0x1403ac590 <.text+0x3ab590>
1403ac565:     	movb	$0x0, 0x11f(%rbp)
1403ac56c:     	xorl	%ecx, %ecx
1403ac56e:     	movq	%r14, %rdx
1403ac571:     	movq	%rbx, %r8
1403ac574:     	callq	0x140d88a40 <.text+0xd87a40>
1403ac579:     	testq	%rax, %rax
1403ac57c:     	movq	0xf0(%rbp), %rbx
1403ac583:     	movq	0xe0(%rbp), %r14
1403ac58a:     	jne	0x1403ac904 <.text+0x3ab904>
1403ac590:     	movq	%rbx, 0x70(%rbp)
1403ac594:     	movq	%r14, 0x78(%rbp)
1403ac598:     	movq	%rbx, 0x80(%rbp)
1403ac59f:     	movb	$0x1, 0x11f(%rbp)
1403ac5a6:     	leaq	0x70(%rbp), %rcx
1403ac5aa:     	callq	0x140d79950 <.text+0xd78950>
1403ac5af:     	movq	%rax, %r14
1403ac5b2:     	movq	%rdx, %rbx
1403ac5b5:     	movq	%rax, %rcx
1403ac5b8:     	xorl	%edx, %edx
1403ac5ba:     	xorl	%r8d, %r8d
1403ac5bd:     	xorl	%r9d, %r9d
1403ac5c0:     	callq	0x142923470 <.text+0x2922470>
1403ac5c5:     	movq	%rax, %rdi
1403ac5c8:     	movzbl	0x328863a(%rip), %eax   # 0x143634c09
1403ac5cf:     	movl	$0x800, %ecx            # imm = 0x800
1403ac5d4:     	movl	$0x1, %edx
1403ac5d9:     	callq	0x140613c40 <.text+0x612c40>
1403ac5de:     	testq	%rax, %rax
1403ac5e1:     	movq	%rbx, 0x110(%rbp)
1403ac5e8:     	movq	%r14, 0x108(%rbp)
1403ac5ef:     	je	0x1403ac993 <.text+0x3ab993>
1403ac5f5:     	movq	%rax, %r15
1403ac5f8:     	movl	$0x800, 0xec(%rbp)      # imm = 0x800
1403ac602:     	leaq	0xec(%rbp), %rdx
1403ac609:     	movq	%rdi, %rcx
1403ac60c:     	movq	%rax, %r8
1403ac60f:     	callq	0x142923490 <.text+0x2922490>
1403ac614:     	movl	%eax, 0xe0(%rbp)
1403ac61a:     	testl	%eax, %eax
1403ac61c:     	movq	%r15, 0xf0(%rbp)
1403ac623:     	je	0x1403ac69f <.text+0x3ab69f>
1403ac625:     	movl	0xec(%rbp), %eax
1403ac62b:     	cmpl	$0x800, %eax            # imm = 0x800
1403ac630:     	movl	$0x800, %ecx            # imm = 0x800
1403ac635:     	cmovbl	%eax, %ecx
1403ac638:     	movq	$0x800, 0x70(%rbp)      # imm = 0x800
1403ac640:     	movq	%r15, 0x78(%rbp)
1403ac644:     	movq	%rcx, 0x80(%rbp)
1403ac64b:     	leaq	0x48(%rbp), %rcx
1403ac64f:     	leaq	0x70(%rbp), %rdx
1403ac653:     	callq	0x140d79ce0 <.text+0xd78ce0>
1403ac658:     	cmpl	$0x2, 0x48(%rbp)
1403ac65c:     	jne	0x1403ac75d <.text+0x3ab75d>
1403ac662:     	movq	0x50(%rbp), %rdx
1403ac666:     	movq	0x58(%rbp), %r8
1403ac66a:     	leaq	0x20(%rbp), %rcx
1403ac66e:     	callq	0x140d79b10 <.text+0xd78b10>
1403ac673:     	cmpb	$0x2, 0x38(%rbp)
1403ac677:     	jne	0x1403ac83f <.text+0x3ab83f>
1403ac67d:     	movq	0x20(%rbp), %r14
1403ac681:     	movq	0x28(%rbp), %rbx
1403ac685:     	movq	%r14, %rax
1403ac688:     	negq	%rax
1403ac68b:     	jo	0x1403ac8d4 <.text+0x3ab8d4>
1403ac691:     	movq	0x30(%rbp), %r15
1403ac695:     	movq	%rdi, %rcx
1403ac698:     	callq	0x142923480 <.text+0x2922480>
1403ac69d:     	jmp	0x1403ac6fc <.text+0x3ab6fc>
1403ac69f:     	leaq	0x261f2b2(%rip), %rax   # 0x1429cb958
1403ac6a6:     	movq	%rax, 0x70(%rbp)
1403ac6aa:     	movq	$0x1, 0x78(%rbp)
1403ac6b2:     	movq	$0x8, 0x80(%rbp)
1403ac6bd:     	xorps	%xmm0, %xmm0
1403ac6c0:     	movups	%xmm0, 0x88(%rbp)
1403ac6c7:     	leaq	0x70(%rbp), %rcx
1403ac6cb:     	callq	0x1428a2020 <.text+0x28a1020>
1403ac6d0:     	movq	%rax, %rbx
1403ac6d3:     	movq	%rdi, %rcx
1403ac6d6:     	callq	0x142923480 <.text+0x2922480>
1403ac6db:     	movl	$0x800, %edx            # imm = 0x800
1403ac6e0:     	movl	$0x1, %r8d
1403ac6e6:     	movq	0xf0(%rbp), %rcx
1403ac6ed:     	callq	0x140613c20 <.text+0x612c20>
1403ac6f2:     	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
1403ac6fc:     	movq	0x110(%rbp), %rdx
1403ac703:     	movq	0x108(%rbp), %rcx
1403ac70a:     	movb	$0x0, (%rcx)
1403ac70d:     	testq	%rdx, %rdx
1403ac710:     	je	0x1403ac71d <.text+0x3ab71d>
1403ac712:     	movl	$0x1, %r8d
1403ac718:     	callq	0x140613c20 <.text+0x612c20>
1403ac71d:     	movq	%r14, (%rsi)
1403ac720:     	movq	%rbx, 0x8(%rsi)
1403ac724:     	movq	%r15, 0x10(%rsi)
1403ac728:     	movb	$0x1, 0x18(%rsi)
1403ac72c:     	movl	0x41(%rbp), %eax
1403ac72f:     	movl	0x44(%rbp), %ecx
1403ac732:     	movl	%eax, 0x19(%rsi)
1403ac735:     	movl	%ecx, 0x1c(%rsi)
1403ac738:     	jmp	0x1403ac82c <.text+0x3ab82c>
1403ac73d:     	movl	$0x1, %r8d
1403ac743:     	movq	%r15, %rcx
1403ac746:     	callq	0x140613c20 <.text+0x612c20>
1403ac74b:     	movq	0x70(%rbp), %rdx
1403ac74f:     	testq	%rdx, %rdx
1403ac752:     	jne	0x1403ac51e <.text+0x3ab51e>
1403ac758:     	jmp	0x1403ac52d <.text+0x3ab52d>
1403ac75d:     	movq	0x58(%rbp), %rax
1403ac761:     	movq	%rax, 0x100(%rbp)
1403ac768:     	movq	0x60(%rbp), %rax
1403ac76c:     	movq	%rax, 0xf8(%rbp)
1403ac773:     	movq	0x68(%rbp), %rdi
1403ac777:     	movb	$0x0, 0x11e(%rbp)
1403ac77e:     	leaq	0x70(%rbp), %rcx
1403ac782:     	callq	0x140d56700 <.text+0xd55700>
1403ac787:     	movups	0x70(%rbp), %xmm0
1403ac78b:     	movups	0x80(%rbp), %xmm1
1403ac792:     	movups	0x90(%rbp), %xmm2
1403ac799:     	movaps	%xmm2, -0x30(%rbp)
1403ac79d:     	movaps	%xmm1, -0x40(%rbp)
1403ac7a1:     	movaps	%xmm0, -0x50(%rbp)
1403ac7a5:     	movups	0x48(%rbp), %xmm0
1403ac7a9:     	movups	%xmm0, 0x80(%rbp)
1403ac7b0:     	movq	0x100(%rbp), %rax
1403ac7b7:     	movq	%rax, 0x90(%rbp)
1403ac7be:     	movq	0xf8(%rbp), %rax
1403ac7c5:     	movq	%rax, 0x98(%rbp)
1403ac7cc:     	movq	%rdi, 0xa0(%rbp)
1403ac7d3:     	leaq	0x261f18e(%rip), %rax   # 0x1429cb968
1403ac7da:     	movq	%rax, 0x70(%rbp)
1403ac7de:     	movq	$0x18, 0x78(%rbp)
1403ac7e6:     	movb	$0x1, 0x11e(%rbp)
1403ac7ed:     	leaq	0x70(%rbp), %rcx
1403ac7f1:     	leaq	-0x50(%rbp), %rdx
1403ac7f5:     	callq	0x1428a2bc0 <.text+0x28a1bc0>
1403ac7fa:     	movq	%rax, 0x8(%rsi)
1403ac7fe:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403ac808:     	movq	%rax, (%rsi)
1403ac80b:     	movq	0x110(%rbp), %rdx
1403ac812:     	movq	0x108(%rbp), %rcx
1403ac819:     	movb	$0x0, (%rcx)
1403ac81c:     	testq	%rdx, %rdx
1403ac81f:     	je	0x1403ac82c <.text+0x3ab82c>
1403ac821:     	movl	$0x1, %r8d
1403ac827:     	callq	0x140613c20 <.text+0x612c20>
1403ac82c:     	movq	%rsi, %rax
1403ac82f:     	addq	$0x1a8, %rsp            # imm = 0x1A8
1403ac836:     	popq	%rbx
1403ac837:     	popq	%rdi
1403ac838:     	popq	%rsi
1403ac839:     	popq	%r14
1403ac83b:     	popq	%r15
1403ac83d:     	popq	%rbp
1403ac83e:     	retq
1403ac83f:     	movq	0x20(%rbp), %rax
1403ac843:     	movq	%rax, 0x100(%rbp)
1403ac84a:     	movq	0x28(%rbp), %rax
1403ac84e:     	movq	%rax, 0xf8(%rbp)
1403ac855:     	movb	$0x1, 0x11d(%rbp)
1403ac85c:     	leaq	0x70(%rbp), %rcx
1403ac860:     	callq	0x140d56700 <.text+0xd55700>
1403ac865:     	movups	0x70(%rbp), %xmm0
1403ac869:     	movups	0x80(%rbp), %xmm1
1403ac870:     	movups	0x90(%rbp), %xmm2
1403ac877:     	movaps	%xmm2, -0x30(%rbp)
1403ac87b:     	movaps	%xmm1, -0x40(%rbp)
1403ac87f:     	movaps	%xmm0, -0x50(%rbp)
1403ac883:     	movq	0x100(%rbp), %rax
1403ac88a:     	movq	%rax, 0x80(%rbp)
1403ac891:     	movq	0xf8(%rbp), %rax
1403ac898:     	movq	%rax, 0x88(%rbp)
1403ac89f:     	movups	0x30(%rbp), %xmm0
1403ac8a3:     	movups	%xmm0, 0x90(%rbp)
1403ac8aa:     	leaq	0x261f0b7(%rip), %rax   # 0x1429cb968
1403ac8b1:     	movq	%rax, 0x70(%rbp)
1403ac8b5:     	movq	$0x18, 0x78(%rbp)
1403ac8bd:     	movb	$0x0, 0x11d(%rbp)
1403ac8c4:     	leaq	0x70(%rbp), %rcx
1403ac8c8:     	leaq	-0x50(%rbp), %rdx
1403ac8cc:     	callq	0x1428a3960 <.text+0x28a2960>
1403ac8d1:     	movq	%rax, %rbx
1403ac8d4:     	movq	0x110(%rbp), %rdx
1403ac8db:     	movq	0x108(%rbp), %rcx
1403ac8e2:     	movq	%rbx, 0x8(%rsi)
1403ac8e6:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403ac8f0:     	movq	%rax, (%rsi)
1403ac8f3:     	movb	$0x0, (%rcx)
1403ac8f6:     	testq	%rdx, %rdx
1403ac8f9:     	jne	0x1403ac821 <.text+0x3ab821>
1403ac8ff:     	jmp	0x1403ac82c <.text+0x3ab82c>
1403ac904:     	movq	%rbx, 0x70(%rbp)
1403ac908:     	movq	%r14, 0x78(%rbp)
1403ac90c:     	movq	%rbx, 0x80(%rbp)
1403ac913:     	movq	%rdx, 0x88(%rbp)
1403ac91a:     	leaq	0x261efdf(%rip), %rax   # 0x1429cb900
1403ac921:     	movq	%rax, 0x20(%rsp)
1403ac926:     	leaq	0x261edbb(%rip), %rcx   # 0x1429cb6e8
1403ac92d:     	leaq	0x261ed94(%rip), %r9    # 0x1429cb6c8
1403ac934:     	leaq	0x70(%rbp), %r8
1403ac938:     	movl	$0x2b, %edx
1403ac93d:     	callq	0x1428d9760 <.text+0x28d8760>
1403ac942:     	jmp	0x1403ac9a9 <.text+0x3ab9a9>
1403ac944:     	movq	0x78(%rbp), %rax
1403ac948:     	movq	%rax, -0x50(%rbp)
1403ac94c:     	leaq	0x261eefd(%rip), %rax   # 0x1429cb850
1403ac953:     	movq	%rax, 0x20(%rsp)
1403ac958:     	leaq	0x261ed89(%rip), %rcx   # 0x1429cb6e8
1403ac95f:     	leaq	0x261edb2(%rip), %r9    # 0x1429cb718
1403ac966:     	leaq	-0x50(%rbp), %r8
1403ac96a:     	movl	$0x2b, %edx
1403ac96f:     	callq	0x1428d9760 <.text+0x28d8760>
1403ac974:     	jmp	0x1403ac9a9 <.text+0x3ab9a9>
1403ac976:     	xorl	%edi, %edi
1403ac978:     	movq	%r15, 0x110(%rbp)
1403ac97f:     	leaq	0x261ee02(%rip), %r8    # 0x1429cb788
1403ac986:     	movq	%rdi, %rcx
1403ac989:     	movq	%rbx, %rdx
1403ac98c:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1403ac991:     	jmp	0x1403ac9a9 <.text+0x3ab9a9>
1403ac993:     	leaq	0x261ef7e(%rip), %r8    # 0x1429cb918
1403ac99a:     	movl	$0x1, %ecx
1403ac99f:     	movl	$0x800, %edx            # imm = 0x800
1403ac9a4:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1403ac9a9:     	ud2
