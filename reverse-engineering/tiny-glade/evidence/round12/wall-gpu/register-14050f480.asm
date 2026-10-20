
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14050f480:     	pushq	%rbp
14050f481:     	pushq	%rsi
14050f482:     	pushq	%rdi
14050f483:     	subq	$0x2f0, %rsp            # imm = 0x2F0
14050f48a:     	leaq	0x80(%rsp), %rbp
14050f492:     	movq	$-0x2, 0x268(%rbp)
14050f49d:     	movq	%rcx, %rsi
14050f4a0:     	xorps	%xmm0, %xmm0
14050f4a3:     	movups	%xmm0, -0x40(%rbp)
14050f4a7:     	movups	%xmm0, -0x28(%rbp)
14050f4ab:     	movups	%xmm0, -0x10(%rbp)
14050f4af:     	movups	%xmm0, 0x8(%rbp)
14050f4b3:     	movups	%xmm0, 0x20(%rbp)
14050f4b7:     	movl	$0x0, 0x30(%rbp)
14050f4be:     	movups	%xmm0, 0x58(%rbp)
14050f4c2:     	movups	%xmm0, 0x70(%rbp)
14050f4c6:     	movups	%xmm0, 0x88(%rbp)
14050f4cd:     	movups	%xmm0, 0xa0(%rbp)
14050f4d4:     	movups	%xmm0, 0xb8(%rbp)
14050f4db:     	movl	$0x0, 0xc8(%rbp)
14050f4e5:     	movq	$0x0, -0x60(%rbp)
14050f4ed:     	movq	$0x8, -0x58(%rbp)
14050f4f5:     	movq	$0x0, -0x50(%rbp)
14050f4fd:     	movq	$0x10, -0x48(%rbp)
14050f505:     	movq	$0x10, -0x30(%rbp)
14050f50d:     	movq	$0x10, -0x18(%rbp)
14050f515:     	movq	$0x10, (%rbp)
14050f51d:     	movq	$0x10, 0x18(%rbp)
14050f525:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14050f52f:     	movq	%rax, 0x38(%rbp)
14050f533:     	leaq	0x24e104b(%rip), %rcx   # 0x1429f0585
14050f53a:     	movq	%rcx, 0x40(%rbp)
14050f53e:     	movq	$0x41, 0x48(%rbp)
14050f546:     	movq	$0x10, 0x50(%rbp)
14050f54e:     	movq	$0x10, 0x68(%rbp)
14050f556:     	movq	$0x10, 0x80(%rbp)
14050f561:     	movq	$0x10, 0x98(%rbp)
14050f56c:     	movq	$0x10, 0xb0(%rbp)
14050f577:     	movl	$0x0, 0xd0(%rbp)
14050f581:     	movw	$0x1, 0xd4(%rbp)
14050f58a:     	movq	%rax, 0x1d0(%rbp)
14050f591:     	movl	$0x0, 0x260(%rbp)
14050f59b:     	movzbl	0x3125667(%rip), %eax   # 0x143634c09
14050f5a2:     	movl	$0x2c8, %ecx            # imm = 0x2C8
14050f5a7:     	movl	$0x8, %edx
14050f5ac:     	callq	0x140613c10 <.text+0x612c10>
14050f5b1:     	testq	%rax, %rax
14050f5b4:     	je	0x14050f5eb <.text+0x50e5eb>
14050f5b6:     	movq	%rax, %rdi
14050f5b9:     	leaq	-0x60(%rbp), %rdx
14050f5bd:     	movl	$0x2c8, %r8d            # imm = 0x2C8
14050f5c3:     	movq	%rax, %rcx
14050f5c6:     	callq	0x142923c00 <.text+0x2922c00>
14050f5cb:     	leaq	0x24e0cee(%rip), %r8    # 0x1429f02c0
14050f5d2:     	movq	%rsi, %rcx
14050f5d5:     	movq	%rdi, %rdx
14050f5d8:     	callq	0x140ce4b10 <.text+0xce3b10>
14050f5dd:     	movq	%rsi, %rax
14050f5e0:     	addq	$0x2f0, %rsp            # imm = 0x2F0
14050f5e7:     	popq	%rdi
14050f5e8:     	popq	%rsi
14050f5e9:     	popq	%rbp
14050f5ea:     	retq
14050f5eb:     	movl	$0x8, %ecx
14050f5f0:     	movl	$0x2c8, %edx            # imm = 0x2C8
14050f5f5:     	callq	0x1428d9000 <.text+0x28d8000>
14050f5fa:     	ud2
