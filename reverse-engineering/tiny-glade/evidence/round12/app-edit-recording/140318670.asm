
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140318670:     	pushq	%rbp
140318671:     	pushq	%rsi
140318672:     	pushq	%rdi
140318673:     	subq	$0x1a0, %rsp            # imm = 0x1A0
14031867a:     	leaq	0x80(%rsp), %rbp
140318682:     	movq	$-0x2, 0x118(%rbp)
14031868d:     	movq	%rcx, %rsi
140318690:     	xorps	%xmm0, %xmm0
140318693:     	movups	%xmm0, -0x8(%rbp)
140318697:     	movups	%xmm0, 0x10(%rbp)
14031869b:     	movups	%xmm0, 0x28(%rbp)
14031869f:     	movups	%xmm0, 0x40(%rbp)
1403186a3:     	movups	%xmm0, 0x58(%rbp)
1403186a7:     	movl	$0x0, 0x68(%rbp)
1403186ae:     	movups	%xmm0, 0x90(%rbp)
1403186b5:     	movups	%xmm0, 0xa8(%rbp)
1403186bc:     	movups	%xmm0, 0xc0(%rbp)
1403186c3:     	movups	%xmm0, 0xd8(%rbp)
1403186ca:     	movups	%xmm0, 0xf0(%rbp)
1403186d1:     	movl	$0x0, 0x100(%rbp)
1403186db:     	movq	$0x0, -0x58(%rbp)
1403186e3:     	movq	$0x0, -0x28(%rbp)
1403186eb:     	movq	$0x8, -0x20(%rbp)
1403186f3:     	movq	$0x0, -0x18(%rbp)
1403186fb:     	movq	$0x10, -0x10(%rbp)
140318703:     	movq	$0x10, 0x8(%rbp)
14031870b:     	movq	$0x10, 0x20(%rbp)
140318713:     	movq	$0x10, 0x38(%rbp)
14031871b:     	movq	$0x10, 0x50(%rbp)
140318723:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14031872d:     	movq	%rax, 0x70(%rbp)
140318731:     	leaq	0x26a3395(%rip), %rax   # 0x1429bbacd
140318738:     	movq	%rax, 0x78(%rbp)
14031873c:     	movq	$0x51, 0x80(%rbp)
140318747:     	movq	$0x10, 0x88(%rbp)
140318752:     	movq	$0x10, 0xa0(%rbp)
14031875d:     	movq	$0x10, 0xb8(%rbp)
140318768:     	movq	$0x10, 0xd0(%rbp)
140318773:     	movq	$0x10, 0xe8(%rbp)
14031877e:     	movl	$0x0, 0x108(%rbp)
140318788:     	movw	$0x1, 0x10c(%rbp)
140318791:     	movl	$0x0, 0x110(%rbp)
14031879b:     	movzbl	0x331c467(%rip), %eax   # 0x143634c09
1403187a2:     	movl	$0x170, %ecx            # imm = 0x170
1403187a7:     	movl	$0x8, %edx
1403187ac:     	callq	0x140613c10 <.text+0x612c10>
1403187b1:     	testq	%rax, %rax
1403187b4:     	je	0x1403187eb <.text+0x3177eb>
1403187b6:     	movq	%rax, %rdi
1403187b9:     	leaq	-0x58(%rbp), %rdx
1403187bd:     	movl	$0x170, %r8d            # imm = 0x170
1403187c3:     	movq	%rax, %rcx
1403187c6:     	callq	0x142923c00 <.text+0x2922c00>
1403187cb:     	leaq	0x26a31ee(%rip), %r8    # 0x1429bb9c0
1403187d2:     	movq	%rsi, %rcx
1403187d5:     	movq	%rdi, %rdx
1403187d8:     	callq	0x140ce4b10 <.text+0xce3b10>
1403187dd:     	movq	%rsi, %rax
1403187e0:     	addq	$0x1a0, %rsp            # imm = 0x1A0
1403187e7:     	popq	%rdi
1403187e8:     	popq	%rsi
1403187e9:     	popq	%rbp
1403187ea:     	retq
1403187eb:     	movl	$0x8, %ecx
1403187f0:     	movl	$0x170, %edx            # imm = 0x170
1403187f5:     	callq	0x1428d9000 <.text+0x28d8000>
1403187fa:     	ud2
