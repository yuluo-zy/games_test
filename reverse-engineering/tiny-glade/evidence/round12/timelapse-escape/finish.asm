
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a8a460:     	pushq	%r14
140a8a462:     	pushq	%rsi
140a8a463:     	pushq	%rdi
140a8a464:     	pushq	%rbx
140a8a465:     	subq	$0x58, %rsp
140a8a469:     	movq	(%rcx), %rax
140a8a46c:     	movq	0x8(%rcx), %rcx
140a8a470:     	movq	(%rax), %r10
140a8a473:     	xorl	%r11d, %r11d
140a8a476:     	movq	%r10, %rsi
140a8a479:     	subq	0x18(%rcx), %rsi
140a8a47d:     	cmovbq	%r11, %rsi
140a8a481:     	subq	0x38(%rcx), %r10
140a8a485:     	cmovbq	%r11, %r10
140a8a489:     	movq	0x30(%rcx), %rdi
140a8a48d:     	subq	%r10, %rdi
140a8a490:     	cmovbq	%r11, %rdi
140a8a494:     	setbe	%r10b
140a8a498:     	movq	0x10(%rcx), %rbx
140a8a49c:     	subq	%rsi, %rbx
140a8a49f:     	cmovbq	%r11, %rbx
140a8a4a3:     	movq	0x40(%rcx), %rsi
140a8a4a7:     	setbe	%cl
140a8a4aa:     	addq	%rdi, %rbx
140a8a4ad:     	andb	%r10b, %cl
140a8a4b0:     	cmoveq	%r11, %rbx
140a8a4b4:     	subq	%rbx, %rsi
140a8a4b7:     	movq	%rsi, (%rax)
140a8a4ba:     	movl	0x1c(%r9), %eax
140a8a4be:     	movq	0x10(%r9), %r10
140a8a4c2:     	movl	%eax, (%r10)
140a8a4c5:     	movq	(%r9), %rdi
140a8a4c8:     	movzbl	(%rdi), %r9d
140a8a4cc:     	cmpb	$0x2, %r9b
140a8a4d0:     	je	0x140a8a5ba <.text+0xa895ba>
140a8a4d6:     	movq	0xa0(%rsp), %rax
140a8a4de:     	testb	%cl, %cl
140a8a4e0:     	je	0x140a8a4e7 <.text+0xa894e7>
140a8a4e2:     	movq	(%rax), %rsi
140a8a4e5:     	jmp	0x140a8a555 <.text+0xa89555>
140a8a4e7:     	movq	(%rax), %rsi
140a8a4ea:     	testb	$0x1, %r9b
140a8a4ee:     	jne	0x140a8a555 <.text+0xa89555>
140a8a4f0:     	movq	0xa8(%rsp), %rcx
140a8a4f8:     	movl	0x1c(%rax), %r9d
140a8a4fc:     	movq	0x10(%rax), %rax
140a8a500:     	movl	%r9d, (%rax)
140a8a503:     	movq	(%rcx), %rax
140a8a506:     	movups	0x60(%rax), %xmm0
140a8a50a:     	movups	%xmm0, 0x28(%rsp)
140a8a50f:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a8a519:     	movq	%rax, 0x20(%rsp)
140a8a51e:     	movq	$0x0, 0x38(%rsp)
140a8a527:     	movb	$0x0, 0x40(%rsp)
140a8a52c:     	leaq	0x21d0905(%rip), %rax   # 0x142c5ae38
140a8a533:     	leaq	0x20(%rsp), %r9
140a8a538:     	movq	%rsi, %rcx
140a8a53b:     	movq	%rdx, %rbx
140a8a53e:     	movq	%r9, %rdx
140a8a541:     	movq	%r8, %r14
140a8a544:     	movq	%rax, %r8
140a8a547:     	callq	0x140b59900 <.text+0xb58900>
140a8a54c:     	movq	%rbx, %rdx
140a8a54f:     	movq	%r14, %r8
140a8a552:     	movb	$0x1, (%rdi)
140a8a555:     	cmpq	$0x0, 0x18(%rsi)
140a8a55a:     	jne	0x140a8a5ba <.text+0xa895ba>
140a8a55c:     	movl	0x1c(%r8), %eax
140a8a560:     	movq	0x10(%r8), %rcx
140a8a564:     	movl	%eax, (%rcx)
140a8a566:     	movq	(%r8), %rax
140a8a569:     	movl	$0x0, (%rax)
140a8a56f:     	movzbl	(%rdi), %ebx
140a8a572:     	movl	0x1c(%rdx), %eax
140a8a575:     	movq	0x10(%rdx), %rcx
140a8a579:     	movl	%eax, (%rcx)
140a8a57b:     	movq	(%rdx), %rsi
140a8a57e:     	movq	0x30(%rsi), %rdi
140a8a582:     	movq	0x40(%rsi), %r14
140a8a586:     	cmpq	0x20(%rsi), %rdi
140a8a58a:     	jne	0x140a8a59c <.text+0xa8959c>
140a8a58c:     	leaq	0x20(%rsi), %rcx
140a8a590:     	leaq	0x21afbb1(%rip), %rdx   # 0x142c3a148
140a8a597:     	callq	0x140562630 <.text+0x561630>
140a8a59c:     	movq	0x28(%rsi), %rax
140a8a5a0:     	movq	%rdi, %rcx
140a8a5a3:     	shlq	$0x4, %rcx
140a8a5a7:     	movq	%r14, (%rax,%rcx)
140a8a5ab:     	movb	%bl, 0x8(%rax,%rcx)
140a8a5af:     	incq	%rdi
140a8a5b2:     	movq	%rdi, 0x30(%rsi)
140a8a5b6:     	incq	0x40(%rsi)
140a8a5ba:     	addq	$0x58, %rsp
140a8a5be:     	popq	%rbx
140a8a5bf:     	popq	%rdi
140a8a5c0:     	popq	%rsi
140a8a5c1:     	popq	%r14
140a8a5c3:     	retq
140a8a5c4:     	int3
140a8a5c5:     	int3
140a8a5c6:     	int3
140a8a5c7:     	int3
140a8a5c8:     	int3
140a8a5c9:     	int3
140a8a5ca:     	int3
140a8a5cb:     	int3
140a8a5cc:     	int3
140a8a5cd:     	int3
140a8a5ce:     	int3
140a8a5cf:     	int3
