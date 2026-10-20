
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1402ad500:     	pushq	%rbp
1402ad501:     	pushq	%rsi
1402ad502:     	pushq	%rdi
1402ad503:     	subq	$0x190, %rsp            # imm = 0x190
1402ad50a:     	leaq	0x80(%rsp), %rbp
1402ad512:     	movq	$-0x2, 0x108(%rbp)
1402ad51d:     	movq	%rcx, %rsi
1402ad520:     	xorps	%xmm0, %xmm0
1402ad523:     	movups	%xmm0, -0x18(%rbp)
1402ad527:     	movups	%xmm0, (%rbp)
1402ad52b:     	movups	%xmm0, 0x18(%rbp)
1402ad52f:     	movups	%xmm0, 0x30(%rbp)
1402ad533:     	movups	%xmm0, 0x48(%rbp)
1402ad537:     	movl	$0x0, 0x58(%rbp)
1402ad53e:     	movups	%xmm0, 0x80(%rbp)
1402ad545:     	movups	%xmm0, 0x98(%rbp)
1402ad54c:     	movups	%xmm0, 0xb0(%rbp)
1402ad553:     	movups	%xmm0, 0xc8(%rbp)
1402ad55a:     	movups	%xmm0, 0xe0(%rbp)
1402ad561:     	movl	$0x0, 0xf0(%rbp)
1402ad56b:     	movq	$0x0, -0x60(%rbp)
1402ad573:     	movq	$0x0, -0x38(%rbp)
1402ad57b:     	movq	$0x8, -0x30(%rbp)
1402ad583:     	movq	$0x0, -0x28(%rbp)
1402ad58b:     	movq	$0x10, -0x20(%rbp)
1402ad593:     	movq	$0x10, -0x8(%rbp)
1402ad59b:     	movq	$0x10, 0x10(%rbp)
1402ad5a3:     	movq	$0x10, 0x28(%rbp)
1402ad5ab:     	movq	$0x10, 0x40(%rbp)
1402ad5b3:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1402ad5bd:     	movq	%rax, 0x60(%rbp)
1402ad5c1:     	leaq	0x26ffac4(%rip), %rax   # 0x1429ad08c
1402ad5c8:     	movq	%rax, 0x68(%rbp)
1402ad5cc:     	movq	$0x5d, 0x70(%rbp)
1402ad5d4:     	movq	$0x10, 0x78(%rbp)
1402ad5dc:     	movq	$0x10, 0x90(%rbp)
1402ad5e7:     	movq	$0x10, 0xa8(%rbp)
1402ad5f2:     	movq	$0x10, 0xc0(%rbp)
1402ad5fd:     	movq	$0x10, 0xd8(%rbp)
1402ad608:     	movl	$0x0, 0xf8(%rbp)
1402ad612:     	movw	$0x1, 0xfc(%rbp)
1402ad61b:     	movl	$0x0, 0x100(%rbp)
1402ad625:     	movzbl	0x33875dd(%rip), %eax   # 0x143634c09
1402ad62c:     	movl	$0x168, %ecx            # imm = 0x168
1402ad631:     	movl	$0x8, %edx
1402ad636:     	callq	0x140613c10 <.text+0x612c10>
1402ad63b:     	testq	%rax, %rax
1402ad63e:     	je	0x1402ad675 <.text+0x2ac675>
1402ad640:     	movq	%rax, %rdi
1402ad643:     	leaq	-0x60(%rbp), %rdx
1402ad647:     	movl	$0x168, %r8d            # imm = 0x168
1402ad64d:     	movq	%rax, %rcx
1402ad650:     	callq	0x142923c00 <.text+0x2922c00>
1402ad655:     	leaq	0x26ff8bc(%rip), %r8    # 0x1429acf18
1402ad65c:     	movq	%rsi, %rcx
1402ad65f:     	movq	%rdi, %rdx
1402ad662:     	callq	0x140ce4b10 <.text+0xce3b10>
1402ad667:     	movq	%rsi, %rax
1402ad66a:     	addq	$0x190, %rsp            # imm = 0x190
1402ad671:     	popq	%rdi
1402ad672:     	popq	%rsi
1402ad673:     	popq	%rbp
1402ad674:     	retq
1402ad675:     	movl	$0x8, %ecx
1402ad67a:     	movl	$0x168, %edx            # imm = 0x168
1402ad67f:     	callq	0x1428d9000 <.text+0x28d8000>
1402ad684:     	ud2
