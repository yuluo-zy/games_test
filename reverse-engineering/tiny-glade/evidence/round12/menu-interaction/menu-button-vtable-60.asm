
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14067e510:     	pushq	%rsi
14067e511:     	pushq	%rdi
14067e512:     	subq	$0x28, %rsp
14067e516:     	movq	%rdx, %rsi
14067e519:     	movq	%rcx, %rdi
14067e51c:     	movq	(%r8), %rdx
14067e51f:     	movq	%rsi, %rcx
14067e522:     	callq	0x140b90f60 <.text+0xb8ff60>
14067e527:     	movq	%rsi, %rcx
14067e52a:     	callq	0x140b90f50 <.text+0xb8ff50>
14067e52f:     	movss	(%rax), %xmm0
14067e533:     	movss	0x4(%rax), %xmm1
14067e538:     	movl	$0x0, (%rdi)
14067e53e:     	movss	%xmm0, 0x4(%rdi)
14067e543:     	movl	$0x0, 0x8(%rdi)
14067e54a:     	movss	%xmm1, 0xc(%rdi)
14067e54f:     	movq	%rdi, %rax
14067e552:     	addq	$0x28, %rsp
14067e556:     	popq	%rdi
14067e557:     	popq	%rsi
14067e558:     	retq
