
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14145a710:     	pushq	%rsi
14145a711:     	pushq	%rdi
14145a712:     	pushq	%rbx
14145a713:     	subq	$0x70, %rsp
14145a717:     	movq	%rcx, %rdi
14145a71a:     	movabsq	$0x736f6d6570736575, %rax # imm = 0x736F6D6570736575
14145a724:     	movq	%rax, 0x28(%rsp)
14145a729:     	movabsq	$0x6c7967656e657261, %rax # imm = 0x6C7967656E657261
14145a733:     	movq	%rax, 0x30(%rsp)
14145a738:     	movabsq	$0x646f72616e646f6d, %rax # imm = 0x646F72616E646F6D
14145a742:     	movq	%rax, 0x38(%rsp)
14145a747:     	movabsq	$0x7465646279746573, %rax # imm = 0x7465646279746573
14145a751:     	movq	%rax, 0x40(%rsp)
14145a756:     	xorps	%xmm0, %xmm0
14145a759:     	movups	%xmm0, 0x48(%rsp)
14145a75e:     	movups	%xmm0, 0x58(%rsp)
14145a763:     	movq	$0x0, 0x68(%rsp)
14145a76c:     	movq	(%rcx), %rax
14145a76f:     	movq	%rax, 0x20(%rsp)
14145a774:     	leaq	0x28(%rsp), %rbx
14145a779:     	leaq	0x20(%rsp), %rdx
14145a77e:     	movl	$0x8, %r8d
14145a784:     	movq	%rbx, %rcx
14145a787:     	callq	0x140106160 <.text+0x105160>
14145a78c:     	movq	0x8(%rdi), %rsi
14145a790:     	movq	0x10(%rdi), %rdi
14145a794:     	xorl	%eax, %eax
14145a796:     	testq	%rsi, %rsi
14145a799:     	setne	%al
14145a79c:     	movq	%rax, 0x20(%rsp)
14145a7a1:     	leaq	0x20(%rsp), %rdx
14145a7a6:     	movl	$0x8, %r8d
14145a7ac:     	movq	%rbx, %rcx
14145a7af:     	callq	0x140106160 <.text+0x105160>
14145a7b4:     	testq	%rsi, %rsi
14145a7b7:     	je	0x14145a7e4 <.text+0x14597e4>
14145a7b9:     	leaq	0x28(%rsp), %rbx
14145a7be:     	movq	%rbx, %rcx
14145a7c1:     	movq	%rsi, %rdx
14145a7c4:     	movq	%rdi, %r8
14145a7c7:     	callq	0x140106160 <.text+0x105160>
14145a7cc:     	movb	$-0x1, 0x20(%rsp)
14145a7d1:     	leaq	0x20(%rsp), %rdx
14145a7d6:     	movl	$0x1, %r8d
14145a7dc:     	movq	%rbx, %rcx
14145a7df:     	callq	0x140106160 <.text+0x105160>
14145a7e4:     	movq	0x58(%rsp), %r9
14145a7e9:     	shlq	$0x38, %r9
14145a7ed:     	orq	0x60(%rsp), %r9
14145a7f2:     	movq	0x38(%rsp), %rdx
14145a7f7:     	movq	0x40(%rsp), %rcx
14145a7fc:     	xorq	%r9, %rcx
14145a7ff:     	movq	0x28(%rsp), %rax
14145a804:     	addq	%rdx, %rax
14145a807:     	movq	0x30(%rsp), %r8
14145a80c:     	rolq	$0xd, %rdx
14145a810:     	addq	%rcx, %r8
14145a813:     	xorq	%rax, %rdx
14145a816:     	rolq	$0x10, %rcx
14145a81a:     	xorq	%r8, %rcx
14145a81d:     	rolq	$0x20, %rax
14145a821:     	addq	%rdx, %r8
14145a824:     	addq	%rcx, %rax
14145a827:     	rolq	$0x11, %rdx
14145a82b:     	xorq	%r8, %rdx
14145a82e:     	rolq	$0x15, %rcx
14145a832:     	rolq	$0x20, %r8
14145a836:     	xorq	%rax, %rcx
14145a839:     	xorq	%r9, %rax
14145a83c:     	xorq	$0xff, %r8
14145a843:     	addq	%rdx, %rax
14145a846:     	addq	%rcx, %r8
14145a849:     	rolq	$0xd, %rdx
14145a84d:     	xorq	%rax, %rdx
14145a850:     	rolq	$0x10, %rcx
14145a854:     	rolq	$0x20, %rax
14145a858:     	xorq	%r8, %rcx
14145a85b:     	addq	%rdx, %r8
14145a85e:     	rolq	$0x11, %rdx
14145a862:     	addq	%rcx, %rax
14145a865:     	xorq	%r8, %rdx
14145a868:     	rolq	$0x15, %rcx
14145a86c:     	xorq	%rax, %rcx
14145a86f:     	rolq	$0x20, %r8
14145a873:     	addq	%rdx, %rax
14145a876:     	addq	%rcx, %r8
14145a879:     	rolq	$0xd, %rdx
14145a87d:     	xorq	%rax, %rdx
14145a880:     	rolq	$0x10, %rcx
14145a884:     	rolq	$0x20, %rax
14145a888:     	xorq	%r8, %rcx
14145a88b:     	addq	%rdx, %r8
14145a88e:     	rolq	$0x11, %rdx
14145a892:     	addq	%rcx, %rax
14145a895:     	xorq	%r8, %rdx
14145a898:     	rolq	$0x15, %rcx
14145a89c:     	xorq	%rax, %rcx
14145a89f:     	rolq	$0x20, %r8
14145a8a3:     	addq	%rdx, %rax
14145a8a6:     	addq	%rcx, %r8
14145a8a9:     	rolq	$0xd, %rdx
14145a8ad:     	xorq	%rax, %rdx
14145a8b0:     	rolq	$0x10, %rcx
14145a8b4:     	xorq	%r8, %rcx
14145a8b7:     	addq	%rdx, %r8
14145a8ba:     	rolq	$0x11, %rdx
14145a8be:     	rolq	$0x15, %rcx
14145a8c2:     	movq	%r8, %rax
14145a8c5:     	rolq	$0x20, %rax
14145a8c9:     	xorq	%rdx, %rax
14145a8cc:     	xorq	%rcx, %rax
14145a8cf:     	xorq	%r8, %rax
14145a8d2:     	addq	$0x70, %rsp
14145a8d6:     	popq	%rbx
14145a8d7:     	popq	%rdi
14145a8d8:     	popq	%rsi
14145a8d9:     	retq
