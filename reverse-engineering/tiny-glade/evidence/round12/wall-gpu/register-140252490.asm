
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140252490:     	pushq	%rbp
140252491:     	pushq	%rsi
140252492:     	pushq	%rdi
140252493:     	subq	$0x1a0, %rsp            # imm = 0x1A0
14025249a:     	leaq	0x80(%rsp), %rbp
1402524a2:     	movq	$-0x2, 0x118(%rbp)
1402524ad:     	movq	%rcx, %rsi
1402524b0:     	xorps	%xmm0, %xmm0
1402524b3:     	movups	%xmm0, -0x8(%rbp)
1402524b7:     	movups	%xmm0, 0x10(%rbp)
1402524bb:     	movups	%xmm0, 0x28(%rbp)
1402524bf:     	movups	%xmm0, 0x40(%rbp)
1402524c3:     	movups	%xmm0, 0x58(%rbp)
1402524c7:     	movl	$0x0, 0x68(%rbp)
1402524ce:     	movups	%xmm0, 0x90(%rbp)
1402524d5:     	movups	%xmm0, 0xa8(%rbp)
1402524dc:     	movups	%xmm0, 0xc0(%rbp)
1402524e3:     	movups	%xmm0, 0xd8(%rbp)
1402524ea:     	movups	%xmm0, 0xf0(%rbp)
1402524f1:     	movl	$0x0, 0x100(%rbp)
1402524fb:     	movq	$0x0, -0x58(%rbp)
140252503:     	movq	$0x0, -0x28(%rbp)
14025250b:     	movq	$0x8, -0x20(%rbp)
140252513:     	movq	$0x0, -0x18(%rbp)
14025251b:     	movq	$0x10, -0x10(%rbp)
140252523:     	movq	$0x10, 0x8(%rbp)
14025252b:     	movq	$0x10, 0x20(%rbp)
140252533:     	movq	$0x10, 0x38(%rbp)
14025253b:     	movq	$0x10, 0x50(%rbp)
140252543:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14025254d:     	movq	%rax, 0x70(%rbp)
140252551:     	leaq	0x27509c5(%rip), %rax   # 0x1429a2f1d
140252558:     	movq	%rax, 0x78(%rbp)
14025255c:     	movq	$0x4f, 0x80(%rbp)
140252567:     	movq	$0x10, 0x88(%rbp)
140252572:     	movq	$0x10, 0xa0(%rbp)
14025257d:     	movq	$0x10, 0xb8(%rbp)
140252588:     	movq	$0x10, 0xd0(%rbp)
140252593:     	movq	$0x10, 0xe8(%rbp)
14025259e:     	movl	$0x0, 0x108(%rbp)
1402525a8:     	movw	$0x1, 0x10c(%rbp)
1402525b1:     	movl	$0x0, 0x110(%rbp)
1402525bb:     	movzbl	0x33e2647(%rip), %eax   # 0x143634c09
1402525c2:     	movl	$0x170, %ecx            # imm = 0x170
1402525c7:     	movl	$0x8, %edx
1402525cc:     	callq	0x140613c10 <.text+0x612c10>
1402525d1:     	testq	%rax, %rax
1402525d4:     	je	0x14025260b <.text+0x25160b>
1402525d6:     	movq	%rax, %rdi
1402525d9:     	leaq	-0x58(%rbp), %rdx
1402525dd:     	movl	$0x170, %r8d            # imm = 0x170
1402525e3:     	movq	%rax, %rcx
1402525e6:     	callq	0x142923c00 <.text+0x2922c00>
1402525eb:     	leaq	0x27506ee(%rip), %r8    # 0x1429a2ce0
1402525f2:     	movq	%rsi, %rcx
1402525f5:     	movq	%rdi, %rdx
1402525f8:     	callq	0x140ce4b10 <.text+0xce3b10>
1402525fd:     	movq	%rsi, %rax
140252600:     	addq	$0x1a0, %rsp            # imm = 0x1A0
140252607:     	popq	%rdi
140252608:     	popq	%rsi
140252609:     	popq	%rbp
14025260a:     	retq
14025260b:     	movl	$0x8, %ecx
140252610:     	movl	$0x170, %edx            # imm = 0x170
140252615:     	callq	0x1428d9000 <.text+0x28d8000>
14025261a:     	ud2
