
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405b4430:     	pushq	%rbp
1405b4431:     	pushq	%rsi
1405b4432:     	pushq	%rdi
1405b4433:     	subq	$0x1a0, %rsp            # imm = 0x1A0
1405b443a:     	leaq	0x80(%rsp), %rbp
1405b4442:     	movq	$-0x2, 0x118(%rbp)
1405b444d:     	movq	%rcx, %rsi
1405b4450:     	xorps	%xmm0, %xmm0
1405b4453:     	movups	%xmm0, -0x8(%rbp)
1405b4457:     	movups	%xmm0, 0x10(%rbp)
1405b445b:     	movups	%xmm0, 0x28(%rbp)
1405b445f:     	movups	%xmm0, 0x40(%rbp)
1405b4463:     	movups	%xmm0, 0x58(%rbp)
1405b4467:     	movl	$0x0, 0x68(%rbp)
1405b446e:     	movups	%xmm0, 0x90(%rbp)
1405b4475:     	movups	%xmm0, 0xa8(%rbp)
1405b447c:     	movups	%xmm0, 0xc0(%rbp)
1405b4483:     	movups	%xmm0, 0xd8(%rbp)
1405b448a:     	movups	%xmm0, 0xf0(%rbp)
1405b4491:     	movl	$0x0, 0x100(%rbp)
1405b449b:     	movq	$0x0, -0x60(%rbp)
1405b44a3:     	movq	$0x0, -0x28(%rbp)
1405b44ab:     	movq	$0x8, -0x20(%rbp)
1405b44b3:     	movq	$0x0, -0x18(%rbp)
1405b44bb:     	movq	$0x10, -0x10(%rbp)
1405b44c3:     	movq	$0x10, 0x8(%rbp)
1405b44cb:     	movq	$0x10, 0x20(%rbp)
1405b44d3:     	movq	$0x10, 0x38(%rbp)
1405b44db:     	movq	$0x10, 0x50(%rbp)
1405b44e3:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1405b44ed:     	movq	%rax, 0x70(%rbp)
1405b44f1:     	leaq	0x244d29c(%rip), %rax   # 0x142a01794
1405b44f8:     	movq	%rax, 0x78(%rbp)
1405b44fc:     	movq	$0x45, 0x80(%rbp)
1405b4507:     	movq	$0x10, 0x88(%rbp)
1405b4512:     	movq	$0x10, 0xa0(%rbp)
1405b451d:     	movq	$0x10, 0xb8(%rbp)
1405b4528:     	movq	$0x10, 0xd0(%rbp)
1405b4533:     	movq	$0x10, 0xe8(%rbp)
1405b453e:     	movl	$0x0, 0x108(%rbp)
1405b4548:     	movw	$0x1, 0x10c(%rbp)
1405b4551:     	movl	$0x0, 0x110(%rbp)
1405b455b:     	movzbl	0x30806a7(%rip), %eax   # 0x143634c09
1405b4562:     	movl	$0x178, %ecx            # imm = 0x178
1405b4567:     	movl	$0x8, %edx
1405b456c:     	callq	0x140613c10 <.text+0x612c10>
1405b4571:     	testq	%rax, %rax
1405b4574:     	je	0x1405b45ab <.text+0x5b35ab>
1405b4576:     	movq	%rax, %rdi
1405b4579:     	leaq	-0x60(%rbp), %rdx
1405b457d:     	movl	$0x178, %r8d            # imm = 0x178
1405b4583:     	movq	%rax, %rcx
1405b4586:     	callq	0x142923c00 <.text+0x2922c00>
1405b458b:     	leaq	0x244d036(%rip), %r8    # 0x142a015c8
1405b4592:     	movq	%rsi, %rcx
1405b4595:     	movq	%rdi, %rdx
1405b4598:     	callq	0x140ce4b10 <.text+0xce3b10>
1405b459d:     	movq	%rsi, %rax
1405b45a0:     	addq	$0x1a0, %rsp            # imm = 0x1A0
1405b45a7:     	popq	%rdi
1405b45a8:     	popq	%rsi
1405b45a9:     	popq	%rbp
1405b45aa:     	retq
1405b45ab:     	movl	$0x8, %ecx
1405b45b0:     	movl	$0x178, %edx            # imm = 0x178
1405b45b5:     	callq	0x1428d9000 <.text+0x28d8000>
1405b45ba:     	ud2
