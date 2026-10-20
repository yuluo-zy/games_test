
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14143b290:     	pushq	%rbp
14143b291:     	pushq	%rsi
14143b292:     	pushq	%rdi
14143b293:     	subq	$0x190, %rsp            # imm = 0x190
14143b29a:     	leaq	0x80(%rsp), %rbp
14143b2a2:     	movq	$-0x2, 0x108(%rbp)
14143b2ad:     	movq	%rcx, %rsi
14143b2b0:     	xorps	%xmm0, %xmm0
14143b2b3:     	movups	%xmm0, -0x18(%rbp)
14143b2b7:     	movups	%xmm0, (%rbp)
14143b2bb:     	movups	%xmm0, 0x18(%rbp)
14143b2bf:     	movups	%xmm0, 0x30(%rbp)
14143b2c3:     	movups	%xmm0, 0x48(%rbp)
14143b2c7:     	movl	$0x0, 0x58(%rbp)
14143b2ce:     	movups	%xmm0, 0x80(%rbp)
14143b2d5:     	movups	%xmm0, 0x98(%rbp)
14143b2dc:     	movups	%xmm0, 0xb0(%rbp)
14143b2e3:     	movups	%xmm0, 0xc8(%rbp)
14143b2ea:     	movups	%xmm0, 0xe0(%rbp)
14143b2f1:     	movl	$0x0, 0xf0(%rbp)
14143b2fb:     	movq	$0x0, -0x60(%rbp)
14143b303:     	movq	$0x0, -0x38(%rbp)
14143b30b:     	movq	$0x8, -0x30(%rbp)
14143b313:     	movq	$0x0, -0x28(%rbp)
14143b31b:     	movq	$0x10, -0x20(%rbp)
14143b323:     	movq	$0x10, -0x8(%rbp)
14143b32b:     	movq	$0x10, 0x10(%rbp)
14143b333:     	movq	$0x10, 0x28(%rbp)
14143b33b:     	movq	$0x10, 0x40(%rbp)
14143b343:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14143b34d:     	movq	%rax, 0x60(%rbp)
14143b351:     	leaq	0x1823bf2(%rip), %rax   # 0x142c5ef4a
14143b358:     	movq	%rax, 0x68(%rbp)
14143b35c:     	movq	$0x4d, 0x70(%rbp)
14143b364:     	movq	$0x10, 0x78(%rbp)
14143b36c:     	movq	$0x10, 0x90(%rbp)
14143b377:     	movq	$0x10, 0xa8(%rbp)
14143b382:     	movq	$0x10, 0xc0(%rbp)
14143b38d:     	movq	$0x10, 0xd8(%rbp)
14143b398:     	movl	$0x0, 0xf8(%rbp)
14143b3a2:     	movw	$0x1, 0xfc(%rbp)
14143b3ab:     	movl	$0x0, 0x100(%rbp)
14143b3b5:     	movzbl	0x21f984d(%rip), %eax   # 0x143634c09
14143b3bc:     	movl	$0x168, %ecx            # imm = 0x168
14143b3c1:     	movl	$0x8, %edx
14143b3c6:     	callq	0x140613c10 <.text+0x612c10>
14143b3cb:     	testq	%rax, %rax
14143b3ce:     	je	0x14143b405 <.text+0x143a405>
14143b3d0:     	movq	%rax, %rdi
14143b3d3:     	leaq	-0x60(%rbp), %rdx
14143b3d7:     	movl	$0x168, %r8d            # imm = 0x168
14143b3dd:     	movq	%rax, %rcx
14143b3e0:     	callq	0x142923c00 <.text+0x2922c00>
14143b3e5:     	leaq	0x1823814(%rip), %r8    # 0x142c5ec00
14143b3ec:     	movq	%rsi, %rcx
14143b3ef:     	movq	%rdi, %rdx
14143b3f2:     	callq	0x140ce4b10 <.text+0xce3b10>
14143b3f7:     	movq	%rsi, %rax
14143b3fa:     	addq	$0x190, %rsp            # imm = 0x190
14143b401:     	popq	%rdi
14143b402:     	popq	%rsi
14143b403:     	popq	%rbp
14143b404:     	retq
14143b405:     	movl	$0x8, %ecx
14143b40a:     	movl	$0x168, %edx            # imm = 0x168
14143b40f:     	callq	0x1428d9000 <.text+0x28d8000>
14143b414:     	ud2
