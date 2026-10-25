
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141740460:     	pushq	%rbp
141740461:     	pushq	%rsi
141740462:     	pushq	%rdi
141740463:     	subq	$0x1d0, %rsp            # imm = 0x1D0
14174046a:     	leaq	0x80(%rsp), %rbp
141740472:     	movq	$-0x2, 0x148(%rbp)
14174047d:     	movq	%rcx, %rsi
141740480:     	xorps	%xmm0, %xmm0
141740483:     	movups	%xmm0, 0x28(%rbp)
141740487:     	movups	%xmm0, 0x40(%rbp)
14174048b:     	movups	%xmm0, 0x58(%rbp)
14174048f:     	movups	%xmm0, 0x70(%rbp)
141740493:     	movups	%xmm0, 0x88(%rbp)
14174049a:     	movl	$0x0, 0x98(%rbp)
1417404a4:     	movups	%xmm0, 0xc0(%rbp)
1417404ab:     	movups	%xmm0, 0xd8(%rbp)
1417404b2:     	movups	%xmm0, 0xf0(%rbp)
1417404b9:     	movups	%xmm0, 0x108(%rbp)
1417404c0:     	movups	%xmm0, 0x120(%rbp)
1417404c7:     	movl	$0x0, 0x130(%rbp)
1417404d1:     	movq	$0x0, -0x58(%rbp)
1417404d9:     	movq	$0x0, 0x8(%rbp)
1417404e1:     	movq	$0x8, 0x10(%rbp)
1417404e9:     	movq	$0x0, 0x18(%rbp)
1417404f1:     	movq	$0x10, 0x20(%rbp)
1417404f9:     	movq	$0x10, 0x38(%rbp)
141740501:     	movq	$0x10, 0x50(%rbp)
141740509:     	movq	$0x10, 0x68(%rbp)
141740511:     	movq	$0x10, 0x80(%rbp)
14174051c:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141740526:     	movq	%rax, 0xa0(%rbp)
14174052d:     	leaq	0x158f629(%rip), %rax   # 0x142ccfb5d
141740534:     	movq	%rax, 0xa8(%rbp)
14174053b:     	movq	$0x3c, 0xb0(%rbp)
141740546:     	movq	$0x10, 0xb8(%rbp)
141740551:     	movq	$0x10, 0xd0(%rbp)
14174055c:     	movq	$0x10, 0xe8(%rbp)
141740567:     	movq	$0x10, 0x100(%rbp)
141740572:     	movq	$0x10, 0x118(%rbp)
14174057d:     	movl	$0x0, 0x138(%rbp)
141740587:     	movw	$0x1, 0x13c(%rbp)
141740590:     	movl	$0x0, 0x140(%rbp)
14174059a:     	movzbl	0x1ef4668(%rip), %eax   # 0x143634c09
1417405a1:     	movl	$0x1a0, %ecx            # imm = 0x1A0
1417405a6:     	movl	$0x8, %edx
1417405ab:     	callq	0x140613c10 <.text+0x612c10>
1417405b0:     	testq	%rax, %rax
1417405b3:     	je	0x1417405ea <.text+0x173f5ea>
1417405b5:     	movq	%rax, %rdi
1417405b8:     	leaq	-0x58(%rbp), %rdx
1417405bc:     	movl	$0x1a0, %r8d            # imm = 0x1A0
1417405c2:     	movq	%rax, %rcx
1417405c5:     	callq	0x142923c00 <.text+0x2922c00>
1417405ca:     	leaq	0x158f28f(%rip), %r8    # 0x142ccf860
1417405d1:     	movq	%rsi, %rcx
1417405d4:     	movq	%rdi, %rdx
1417405d7:     	callq	0x140ce4b10 <.text+0xce3b10>
1417405dc:     	movq	%rsi, %rax
1417405df:     	addq	$0x1d0, %rsp            # imm = 0x1D0
1417405e6:     	popq	%rdi
1417405e7:     	popq	%rsi
1417405e8:     	popq	%rbp
1417405e9:     	retq
1417405ea:     	movl	$0x8, %ecx
1417405ef:     	movl	$0x1a0, %edx            # imm = 0x1A0
1417405f4:     	callq	0x1428d9000 <.text+0x28d8000>
1417405f9:     	ud2
