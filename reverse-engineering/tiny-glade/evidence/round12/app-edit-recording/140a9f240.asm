
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a9f240:     	pushq	%rbp
140a9f241:     	pushq	%rsi
140a9f242:     	pushq	%rdi
140a9f243:     	subq	$0x1e0, %rsp            # imm = 0x1E0
140a9f24a:     	leaq	0x80(%rsp), %rbp
140a9f252:     	movq	$-0x2, 0x158(%rbp)
140a9f25d:     	movq	%rcx, %rsi
140a9f260:     	xorps	%xmm0, %xmm0
140a9f263:     	movups	%xmm0, 0x38(%rbp)
140a9f267:     	movups	%xmm0, 0x50(%rbp)
140a9f26b:     	movups	%xmm0, 0x68(%rbp)
140a9f26f:     	movups	%xmm0, 0x80(%rbp)
140a9f276:     	movups	%xmm0, 0x98(%rbp)
140a9f27d:     	movl	$0x0, 0xa8(%rbp)
140a9f287:     	movups	%xmm0, 0xd0(%rbp)
140a9f28e:     	movups	%xmm0, 0xe8(%rbp)
140a9f295:     	movups	%xmm0, 0x100(%rbp)
140a9f29c:     	movups	%xmm0, 0x118(%rbp)
140a9f2a3:     	movups	%xmm0, 0x130(%rbp)
140a9f2aa:     	movl	$0x0, 0x140(%rbp)
140a9f2b4:     	movq	$0x0, -0x60(%rbp)
140a9f2bc:     	movq	$0x0, 0x18(%rbp)
140a9f2c4:     	movq	$0x8, 0x20(%rbp)
140a9f2cc:     	movq	$0x0, 0x28(%rbp)
140a9f2d4:     	movq	$0x10, 0x30(%rbp)
140a9f2dc:     	movq	$0x10, 0x48(%rbp)
140a9f2e4:     	movq	$0x10, 0x60(%rbp)
140a9f2ec:     	movq	$0x10, 0x78(%rbp)
140a9f2f4:     	movq	$0x10, 0x90(%rbp)
140a9f2ff:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a9f309:     	movq	%rax, 0xb0(%rbp)
140a9f310:     	leaq	0x203ce83(%rip), %rax   # 0x142adc19a
140a9f317:     	movq	%rax, 0xb8(%rbp)
140a9f31e:     	movq	$0x38, 0xc0(%rbp)
140a9f329:     	movq	$0x10, 0xc8(%rbp)
140a9f334:     	movq	$0x10, 0xe0(%rbp)
140a9f33f:     	movq	$0x10, 0xf8(%rbp)
140a9f34a:     	movq	$0x10, 0x110(%rbp)
140a9f355:     	movq	$0x10, 0x128(%rbp)
140a9f360:     	movl	$0x0, 0x148(%rbp)
140a9f36a:     	movw	$0x1, 0x14c(%rbp)
140a9f373:     	movl	$0x0, 0x150(%rbp)
140a9f37d:     	movzbl	0x2b95885(%rip), %eax   # 0x143634c09
140a9f384:     	movl	$0x1b8, %ecx            # imm = 0x1B8
140a9f389:     	movl	$0x8, %edx
140a9f38e:     	callq	0x140613c10 <.text+0x612c10>
140a9f393:     	testq	%rax, %rax
140a9f396:     	je	0x140a9f3cd <.text+0xa9e3cd>
140a9f398:     	movq	%rax, %rdi
140a9f39b:     	leaq	-0x60(%rbp), %rdx
140a9f39f:     	movl	$0x1b8, %r8d            # imm = 0x1B8
140a9f3a5:     	movq	%rax, %rcx
140a9f3a8:     	callq	0x142923c00 <.text+0x2922c00>
140a9f3ad:     	leaq	0x203cb74(%rip), %r8    # 0x142adbf28
140a9f3b4:     	movq	%rsi, %rcx
140a9f3b7:     	movq	%rdi, %rdx
140a9f3ba:     	callq	0x140ce4b10 <.text+0xce3b10>
140a9f3bf:     	movq	%rsi, %rax
140a9f3c2:     	addq	$0x1e0, %rsp            # imm = 0x1E0
140a9f3c9:     	popq	%rdi
140a9f3ca:     	popq	%rsi
140a9f3cb:     	popq	%rbp
140a9f3cc:     	retq
140a9f3cd:     	movl	$0x8, %ecx
140a9f3d2:     	movl	$0x1b8, %edx            # imm = 0x1B8
140a9f3d7:     	callq	0x1428d9000 <.text+0x28d8000>
140a9f3dc:     	ud2
