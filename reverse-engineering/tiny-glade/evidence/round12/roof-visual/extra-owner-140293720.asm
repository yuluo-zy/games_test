
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140293720:     	pushq	%rbp
140293721:     	pushq	%rsi
140293722:     	pushq	%rdi
140293723:     	subq	$0x300, %rsp            # imm = 0x300
14029372a:     	leaq	0x80(%rsp), %rbp
140293732:     	movq	$-0x2, 0x278(%rbp)
14029373d:     	movq	%rcx, %rsi
140293740:     	xorps	%xmm0, %xmm0
140293743:     	movups	%xmm0, -0x38(%rbp)
140293747:     	movups	%xmm0, -0x20(%rbp)
14029374b:     	movups	%xmm0, -0x8(%rbp)
14029374f:     	movups	%xmm0, 0x10(%rbp)
140293753:     	movups	%xmm0, 0x28(%rbp)
140293757:     	movl	$0x0, 0x38(%rbp)
14029375e:     	movups	%xmm0, 0x60(%rbp)
140293762:     	movups	%xmm0, 0x78(%rbp)
140293766:     	movups	%xmm0, 0x90(%rbp)
14029376d:     	movups	%xmm0, 0xa8(%rbp)
140293774:     	movups	%xmm0, 0xc0(%rbp)
14029377b:     	movl	$0x0, 0xd0(%rbp)
140293785:     	movq	$0x0, -0x58(%rbp)
14029378d:     	movq	$0x8, -0x50(%rbp)
140293795:     	movq	$0x0, -0x48(%rbp)
14029379d:     	movq	$0x10, -0x40(%rbp)
1402937a5:     	movq	$0x10, -0x28(%rbp)
1402937ad:     	movq	$0x10, -0x10(%rbp)
1402937b5:     	movq	$0x10, 0x8(%rbp)
1402937bd:     	movq	$0x10, 0x20(%rbp)
1402937c5:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1402937cf:     	movq	%rax, 0x40(%rbp)
1402937d3:     	leaq	0x27176b9(%rip), %rcx   # 0x1429aae93
1402937da:     	movq	%rcx, 0x48(%rbp)
1402937de:     	movq	$0x3b, 0x50(%rbp)
1402937e6:     	movq	$0x10, 0x58(%rbp)
1402937ee:     	movq	$0x10, 0x70(%rbp)
1402937f6:     	movq	$0x10, 0x88(%rbp)
140293801:     	movq	$0x10, 0xa0(%rbp)
14029380c:     	movq	$0x10, 0xb8(%rbp)
140293817:     	movl	$0x0, 0xd8(%rbp)
140293821:     	movw	$0x1, 0xdc(%rbp)
14029382a:     	movq	%rax, 0x248(%rbp)
140293831:     	movl	$0x0, 0x270(%rbp)
14029383b:     	movzbl	0x33a13c7(%rip), %eax   # 0x143634c09
140293842:     	movl	$0x2d0, %ecx            # imm = 0x2D0
140293847:     	movl	$0x8, %edx
14029384c:     	callq	0x140613c10 <.text+0x612c10>
140293851:     	testq	%rax, %rax
140293854:     	je	0x14029388b <.text+0x29288b>
140293856:     	movq	%rax, %rdi
140293859:     	leaq	-0x58(%rbp), %rdx
14029385d:     	movl	$0x2d0, %r8d            # imm = 0x2D0
140293863:     	movq	%rax, %rcx
140293866:     	callq	0x142923c00 <.text+0x2922c00>
14029386b:     	leaq	0x27174be(%rip), %r8    # 0x1429aad30
140293872:     	movq	%rsi, %rcx
140293875:     	movq	%rdi, %rdx
140293878:     	callq	0x140ce4b10 <.text+0xce3b10>
14029387d:     	movq	%rsi, %rax
140293880:     	addq	$0x300, %rsp            # imm = 0x300
140293887:     	popq	%rdi
140293888:     	popq	%rsi
140293889:     	popq	%rbp
14029388a:     	retq
14029388b:     	movl	$0x8, %ecx
140293890:     	movl	$0x2d0, %edx            # imm = 0x2D0
140293895:     	callq	0x1428d9000 <.text+0x28d8000>
14029389a:     	ud2
14029389c:     	nopl	(%rax)
1402938a0:     	movq	%rdx, 0x10(%rsp)
1402938a5:     	pushq	%rbp
1402938a6:     	pushq	%rsi
1402938a7:     	pushq	%rdi
1402938a8:     	subq	$0x20, %rsp
1402938ac:     	leaq	0x80(%rdx), %rbp
1402938b3:     	leaq	-0x58(%rbp), %rcx
1402938b7:     	callq	0x1405ddcb0 <.text+0x5dccb0>
1402938bc:     	nop
1402938bd:     	addq	$0x20, %rsp
1402938c1:     	popq	%rdi
1402938c2:     	popq	%rsi
1402938c3:     	popq	%rbp
1402938c4:     	retq
