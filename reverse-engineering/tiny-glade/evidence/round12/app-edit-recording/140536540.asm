
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140536540:     	pushq	%rbp
140536541:     	pushq	%rsi
140536542:     	pushq	%rdi
140536543:     	subq	$0x350, %rsp            # imm = 0x350
14053654a:     	leaq	0x80(%rsp), %rbp
140536552:     	movq	$-0x2, 0x2c8(%rbp)
14053655d:     	movq	%rcx, %rsi
140536560:     	xorps	%xmm0, %xmm0
140536563:     	movups	%xmm0, -0x40(%rbp)
140536567:     	movups	%xmm0, -0x28(%rbp)
14053656b:     	movups	%xmm0, -0x10(%rbp)
14053656f:     	movups	%xmm0, 0x8(%rbp)
140536573:     	movups	%xmm0, 0x20(%rbp)
140536577:     	movl	$0x0, 0x30(%rbp)
14053657e:     	movups	%xmm0, 0x58(%rbp)
140536582:     	movups	%xmm0, 0x70(%rbp)
140536586:     	movups	%xmm0, 0x88(%rbp)
14053658d:     	movups	%xmm0, 0xa0(%rbp)
140536594:     	movups	%xmm0, 0xb8(%rbp)
14053659b:     	movl	$0x0, 0xc8(%rbp)
1405365a5:     	movq	$0x0, -0x60(%rbp)
1405365ad:     	movq	$0x8, -0x58(%rbp)
1405365b5:     	movq	$0x0, -0x50(%rbp)
1405365bd:     	movq	$0x10, -0x48(%rbp)
1405365c5:     	movq	$0x10, -0x30(%rbp)
1405365cd:     	movq	$0x10, -0x18(%rbp)
1405365d5:     	movq	$0x10, (%rbp)
1405365dd:     	movq	$0x10, 0x18(%rbp)
1405365e5:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1405365ef:     	movq	%rax, 0x38(%rbp)
1405365f3:     	leaq	0x24bf723(%rip), %rcx   # 0x1429f5d1d
1405365fa:     	movq	%rcx, 0x40(%rbp)
1405365fe:     	movq	$0x3d, 0x48(%rbp)
140536606:     	movq	$0x10, 0x50(%rbp)
14053660e:     	movq	$0x10, 0x68(%rbp)
140536616:     	movq	$0x10, 0x80(%rbp)
140536621:     	movq	$0x10, 0x98(%rbp)
14053662c:     	movq	$0x10, 0xb0(%rbp)
140536637:     	movl	$0x0, 0xd0(%rbp)
140536641:     	movw	$0x1, 0xd4(%rbp)
14053664a:     	movq	%rax, 0x1e0(%rbp)
140536651:     	movl	$0x0, 0x2c0(%rbp)
14053665b:     	movzbl	0x30fe5a7(%rip), %eax   # 0x143634c09
140536662:     	movl	$0x328, %ecx            # imm = 0x328
140536667:     	movl	$0x8, %edx
14053666c:     	callq	0x140613c10 <.text+0x612c10>
140536671:     	testq	%rax, %rax
140536674:     	je	0x1405366ab <.text+0x5356ab>
140536676:     	movq	%rax, %rdi
140536679:     	leaq	-0x60(%rbp), %rdx
14053667d:     	movl	$0x328, %r8d            # imm = 0x328
140536683:     	movq	%rax, %rcx
140536686:     	callq	0x142923c00 <.text+0x2922c00>
14053668b:     	leaq	0x24bf5b6(%rip), %r8    # 0x1429f5c48
140536692:     	movq	%rsi, %rcx
140536695:     	movq	%rdi, %rdx
140536698:     	callq	0x140ce4b10 <.text+0xce3b10>
14053669d:     	movq	%rsi, %rax
1405366a0:     	addq	$0x350, %rsp            # imm = 0x350
1405366a7:     	popq	%rdi
1405366a8:     	popq	%rsi
1405366a9:     	popq	%rbp
1405366aa:     	retq
1405366ab:     	movl	$0x8, %ecx
1405366b0:     	movl	$0x328, %edx            # imm = 0x328
1405366b5:     	callq	0x1428d9000 <.text+0x28d8000>
1405366ba:     	ud2
