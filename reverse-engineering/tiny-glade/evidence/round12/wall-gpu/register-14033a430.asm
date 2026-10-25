
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14033a430:     	pushq	%rbp
14033a431:     	pushq	%rsi
14033a432:     	pushq	%rdi
14033a433:     	subq	$0x1a0, %rsp            # imm = 0x1A0
14033a43a:     	leaq	0x80(%rsp), %rbp
14033a442:     	movq	$-0x2, 0x118(%rbp)
14033a44d:     	movq	%rcx, %rsi
14033a450:     	xorps	%xmm0, %xmm0
14033a453:     	movups	%xmm0, -0x8(%rbp)
14033a457:     	movups	%xmm0, 0x10(%rbp)
14033a45b:     	movups	%xmm0, 0x28(%rbp)
14033a45f:     	movups	%xmm0, 0x40(%rbp)
14033a463:     	movups	%xmm0, 0x58(%rbp)
14033a467:     	movl	$0x0, 0x68(%rbp)
14033a46e:     	movups	%xmm0, 0x90(%rbp)
14033a475:     	movups	%xmm0, 0xa8(%rbp)
14033a47c:     	movups	%xmm0, 0xc0(%rbp)
14033a483:     	movups	%xmm0, 0xd8(%rbp)
14033a48a:     	movups	%xmm0, 0xf0(%rbp)
14033a491:     	movl	$0x0, 0x100(%rbp)
14033a49b:     	movq	$0x0, -0x58(%rbp)
14033a4a3:     	movq	$0x0, -0x28(%rbp)
14033a4ab:     	movq	$0x8, -0x20(%rbp)
14033a4b3:     	movq	$0x0, -0x18(%rbp)
14033a4bb:     	movq	$0x10, -0x10(%rbp)
14033a4c3:     	movq	$0x10, 0x8(%rbp)
14033a4cb:     	movq	$0x10, 0x20(%rbp)
14033a4d3:     	movq	$0x10, 0x38(%rbp)
14033a4db:     	movq	$0x10, 0x50(%rbp)
14033a4e3:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14033a4ed:     	movq	%rax, 0x70(%rbp)
14033a4f1:     	leaq	0x268652d(%rip), %rax   # 0x1429c0a25
14033a4f8:     	movq	%rax, 0x78(%rbp)
14033a4fc:     	movq	$0x3b, 0x80(%rbp)
14033a507:     	movq	$0x10, 0x88(%rbp)
14033a512:     	movq	$0x10, 0xa0(%rbp)
14033a51d:     	movq	$0x10, 0xb8(%rbp)
14033a528:     	movq	$0x10, 0xd0(%rbp)
14033a533:     	movq	$0x10, 0xe8(%rbp)
14033a53e:     	movl	$0x0, 0x108(%rbp)
14033a548:     	movw	$0x1, 0x10c(%rbp)
14033a551:     	movl	$0x0, 0x110(%rbp)
14033a55b:     	movzbl	0x32fa6a7(%rip), %eax   # 0x143634c09
14033a562:     	movl	$0x170, %ecx            # imm = 0x170
14033a567:     	movl	$0x8, %edx
14033a56c:     	callq	0x140613c10 <.text+0x612c10>
14033a571:     	testq	%rax, %rax
14033a574:     	je	0x14033a5ab <.text+0x3395ab>
14033a576:     	movq	%rax, %rdi
14033a579:     	leaq	-0x58(%rbp), %rdx
14033a57d:     	movl	$0x170, %r8d            # imm = 0x170
14033a583:     	movq	%rax, %rcx
14033a586:     	callq	0x142923c00 <.text+0x2922c00>
14033a58b:     	leaq	0x268605e(%rip), %r8    # 0x1429c05f0
14033a592:     	movq	%rsi, %rcx
14033a595:     	movq	%rdi, %rdx
14033a598:     	callq	0x140ce4b10 <.text+0xce3b10>
14033a59d:     	movq	%rsi, %rax
14033a5a0:     	addq	$0x1a0, %rsp            # imm = 0x1A0
14033a5a7:     	popq	%rdi
14033a5a8:     	popq	%rsi
14033a5a9:     	popq	%rbp
14033a5aa:     	retq
14033a5ab:     	movl	$0x8, %ecx
14033a5b0:     	movl	$0x170, %edx            # imm = 0x170
14033a5b5:     	callq	0x1428d9000 <.text+0x28d8000>
14033a5ba:     	ud2
