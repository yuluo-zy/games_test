
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140500a20:     	pushq	%r15
140500a22:     	pushq	%r14
140500a24:     	pushq	%r13
140500a26:     	pushq	%r12
140500a28:     	pushq	%rsi
140500a29:     	pushq	%rdi
140500a2a:     	pushq	%rbp
140500a2b:     	pushq	%rbx
140500a2c:     	subq	$0xd8, %rsp
140500a33:     	movq	%r9, %rsi
140500a36:     	movq	%rcx, %rbx
140500a39:     	movq	%rdx, 0x28(%rsp)
140500a3e:     	movq	%r8, 0x30(%rsp)
140500a43:     	leaq	0x10(%rcx), %rdi
140500a47:     	movq	%rdi, %rcx
140500a4a:     	callq	0x1406868b0 <.text+0x6858b0>
140500a4f:     	movq	%rax, %r12
140500a52:     	leaq	0x28(%rsp), %r15
140500a57:     	movq	%r15, %rcx
140500a5a:     	callq	0x140683c10 <.text+0x682c10>
140500a5f:     	movq	%rax, 0x38(%rsp)
140500a64:     	leaq	0x38(%rsp), %r14
140500a69:     	movq	%r14, %rcx
140500a6c:     	callq	0x1400885d0 <.text+0x875d0>
140500a71:     	movq	%rax, 0x58(%rsp)
140500a76:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
140500a80:     	movq	%r13, 0x60(%rsp)
140500a85:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
140500a8f:     	movq	%rbp, 0x68(%rsp)
140500a94:     	leaq	0x58(%rsp), %rcx
140500a99:     	callq	0x1405bdf20 <.text+0x5bcf20>
140500a9e:     	movq	%r12, 0x58(%rsp)
140500aa3:     	movq	%rax, 0x60(%rsp)
140500aa8:     	leaq	0x58(%rsp), %rcx
140500aad:     	callq	0x1405bd910 <.text+0x5bc910>
140500ab2:     	movq	%rax, %r12
140500ab5:     	movq	%rdi, %rcx
140500ab8:     	movq	%rax, %rdx
140500abb:     	callq	0x140686450 <.text+0x685450>
140500ac0:     	testb	%al, %al
140500ac2:     	je	0x140500adc <.text+0x4ffadc>
140500ac4:     	leaq	0x58(%rsp), %rbx
140500ac9:     	movq	%rbx, %rcx
140500acc:     	movq	%rdi, %rdx
140500acf:     	movq	%r12, %r8
140500ad2:     	callq	0x140686530 <.text+0x685530>
140500ad7:     	jmp	0x140500c1c <.text+0x4ffc1c>
140500adc:     	movq	0x312b8c5(%rip), %rax   # 0x14362c3a8
140500ae3:     	movq	(%rax), %rax
140500ae6:     	testq	%rax, %rax
140500ae9:     	je	0x140500bc4 <.text+0x4ffbc4>
140500aef:     	movq	%rbx, 0x38(%rsp)
140500af4:     	leaq	0x32da5(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
140500afb:     	movq	%rax, 0x40(%rsp)
140500b00:     	movq	%r15, 0x48(%rsp)
140500b05:     	movq	%rax, 0x50(%rsp)
140500b0a:     	leaq	0x24ed247(%rip), %rcx   # 0x1429edd58
140500b11:     	callq	0x14067dd80 <.text+0x67cd80>
140500b16:     	movups	(%rax), %xmm0
140500b19:     	movl	0x10(%rax), %eax
140500b1c:     	movq	$0x1, 0x88(%rsp)
140500b28:     	leaq	0x24ed1f9(%rip), %rcx   # 0x1429edd28
140500b2f:     	movq	%rcx, 0x90(%rsp)
140500b37:     	movq	$0x14, 0x98(%rsp)
140500b43:     	leaq	0x24ed14e(%rip), %rdx   # 0x1429edc98
140500b4a:     	movq	%rdx, 0xa8(%rsp)
140500b52:     	movq	$0x2, 0xb0(%rsp)
140500b5e:     	movq	%r14, 0xb8(%rsp)
140500b66:     	movq	$0x2, 0xc0(%rsp)
140500b72:     	movq	$0x0, 0xc8(%rsp)
140500b7e:     	movq	$0x0, 0x58(%rsp)
140500b87:     	movq	%rcx, 0x60(%rsp)
140500b8c:     	movq	$0x14, 0x68(%rsp)
140500b95:     	movq	$0x0, 0x70(%rsp)
140500b9e:     	movups	%xmm0, 0x78(%rsp)
140500ba3:     	movl	$0x1, 0xa0(%rsp)
140500bae:     	movl	%eax, 0xa4(%rsp)
140500bb5:     	leaq	0x27(%rsp), %rcx
140500bba:     	leaq	0x58(%rsp), %rdx
140500bbf:     	callq	0x140d4a330 <.text+0xd49330>
140500bc4:     	movups	0x28(%rsp), %xmm0
140500bc9:     	movq	0x18(%rbx), %rbx
140500bcd:     	movups	%xmm0, 0x38(%rsp)
140500bd2:     	leaq	0x38(%rsp), %rcx
140500bd7:     	callq	0x140683c10 <.text+0x682c10>
140500bdc:     	movq	%rax, 0x58(%rsp)
140500be1:     	movq	%r13, 0x60(%rsp)
140500be6:     	movq	%rbp, 0x68(%rsp)
140500beb:     	leaq	0x58(%rsp), %rcx
140500bf0:     	callq	0x1405bdf20 <.text+0x5bcf20>
140500bf5:     	movq	%rbx, 0x58(%rsp)
140500bfa:     	movq	%rax, 0x60(%rsp)
140500bff:     	leaq	0x58(%rsp), %rcx
140500c04:     	callq	0x1405bd910 <.text+0x5bc910>
140500c09:     	leaq	0x58(%rsp), %rbx
140500c0e:     	movq	%rbx, %rcx
140500c11:     	movq	%rdi, %rdx
140500c14:     	movq	%rax, %r8
140500c17:     	callq	0x1406861b0 <.text+0x6851b0>
140500c1c:     	movss	0x24ecf6c(%rip), %xmm1  # 0x1429edb90
140500c24:     	movq	%rbx, %rcx
140500c27:     	movq	%rsi, %r8
140500c2a:     	callq	0x1403d07f0 <.text+0x3cf7f0>
140500c2f:     	nop
140500c30:     	addq	$0xd8, %rsp
140500c37:     	popq	%rbx
140500c38:     	popq	%rbp
140500c39:     	popq	%rdi
140500c3a:     	popq	%rsi
140500c3b:     	popq	%r12
140500c3d:     	popq	%r13
140500c3f:     	popq	%r14
140500c41:     	popq	%r15
140500c43:     	retq
