
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a8a310:     	pushq	%r15
140a8a312:     	pushq	%r14
140a8a314:     	pushq	%r13
140a8a316:     	pushq	%r12
140a8a318:     	pushq	%rsi
140a8a319:     	pushq	%rdi
140a8a31a:     	pushq	%rbp
140a8a31b:     	pushq	%rbx
140a8a31c:     	subq	$0x68, %rsp
140a8a320:     	movq	%r9, 0x20(%rsp)
140a8a325:     	movq	(%rcx), %rax
140a8a328:     	movq	0x8(%rcx), %r15
140a8a32c:     	movq	(%rax), %rcx
140a8a32f:     	xorl	%edi, %edi
140a8a331:     	movq	%rcx, %r10
140a8a334:     	subq	0x18(%r15), %r10
140a8a338:     	cmovbq	%rdi, %r10
140a8a33c:     	subq	0x38(%r15), %rcx
140a8a340:     	cmovbq	%rdi, %rcx
140a8a344:     	movq	0x10(%r15), %rbx
140a8a348:     	movq	0x30(%r15), %r9
140a8a34c:     	movq	%r9, %r11
140a8a34f:     	subq	%rcx, %r11
140a8a352:     	cmovbq	%rdi, %r11
140a8a356:     	setbe	%bpl
140a8a35a:     	movq	%rbx, %r14
140a8a35d:     	subq	%r10, %r14
140a8a360:     	cmovbq	%rdi, %r14
140a8a364:     	setbe	%r13b
140a8a368:     	movq	0x40(%r15), %rsi
140a8a36c:     	movq	%rsi, %rdi
140a8a36f:     	subq	%r11, %rdi
140a8a372:     	subq	%r14, %rdi
140a8a375:     	movq	0x8(%r15), %r12
140a8a379:     	movq	0x28(%r15), %r15
140a8a37d:     	movq	%rdi, (%rax)
140a8a380:     	testb	%bpl, %r13b
140a8a383:     	jne	0x140a8a442 <.text+0xa89442>
140a8a389:     	movq	%rcx, %rdi
140a8a38c:     	shlq	$0x4, %rdi
140a8a390:     	addq	%rdi, %r15
140a8a393:     	shlq	$0x4, %r11
140a8a397:     	movq	%r10, %rdi
140a8a39a:     	shlq	$0x4, %rdi
140a8a39e:     	addq	%rdi, %r12
140a8a3a1:     	shlq	$0x4, %r14
140a8a3a5:     	xorl	%edi, %edi
140a8a3a7:     	cmpq	%r10, %rbx
140a8a3aa:     	movl	$0x8, %r10d
140a8a3b0:     	cmovbq	%r10, %r12
140a8a3b4:     	leaq	-0x10(%r12,%r14), %rbx
140a8a3b9:     	cmovbeq	%rdi, %rbx
140a8a3bd:     	cmpq	%rcx, %r9
140a8a3c0:     	cmovbq	%r10, %r15
140a8a3c4:     	leaq	-0x10(%r15,%r11), %rcx
140a8a3c9:     	cmovbeq	%rbx, %rcx
140a8a3cd:     	movq	%rsi, (%rax)
140a8a3d0:     	addq	$0x8, %rcx
140a8a3d4:     	movq	%rcx, 0x28(%rsp)
140a8a3d9:     	movq	(%rdx), %rax
140a8a3dc:     	cmpq	0x30(%rax), %rdi
140a8a3e0:     	movq	0xd8(%rsp), %rax
140a8a3e8:     	movq	%rax, 0x30(%rsp)
140a8a3ed:     	movq	0xd0(%rsp), %rax
140a8a3f5:     	movq	%rax, 0x38(%rsp)
140a8a3fa:     	movq	%r8, 0x40(%rsp)
140a8a3ff:     	movq	0x20(%rsp), %rax
140a8a404:     	movq	%rax, 0x48(%rsp)
140a8a409:     	movq	0xe8(%rsp), %rax
140a8a411:     	movq	%rax, 0x50(%rsp)
140a8a416:     	leaq	0x28(%rsp), %rax
140a8a41b:     	movq	%rax, 0x58(%rsp)
140a8a420:     	movq	0xf0(%rsp), %rax
140a8a428:     	movq	%rax, 0x60(%rsp)
140a8a42d:     	movq	0xe0(%rsp), %rdx
140a8a435:     	setno	%cl
140a8a438:     	leaq	0x30(%rsp), %r8
140a8a43d:     	callq	0x140af9e00 <.text+0xaf8e00>
140a8a442:     	nop
140a8a443:     	addq	$0x68, %rsp
140a8a447:     	popq	%rbx
140a8a448:     	popq	%rbp
140a8a449:     	popq	%rdi
140a8a44a:     	popq	%rsi
140a8a44b:     	popq	%r12
140a8a44d:     	popq	%r13
140a8a44f:     	popq	%r14
140a8a451:     	popq	%r15
140a8a453:     	retq
140a8a454:     	int3
140a8a455:     	int3
140a8a456:     	int3
140a8a457:     	int3
140a8a458:     	int3
140a8a459:     	int3
140a8a45a:     	int3
140a8a45b:     	int3
140a8a45c:     	int3
140a8a45d:     	int3
140a8a45e:     	int3
140a8a45f:     	int3
