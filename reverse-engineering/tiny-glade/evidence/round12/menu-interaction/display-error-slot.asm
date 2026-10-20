
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fc69f0:     	pushq	%rbp
140fc69f1:     	pushq	%rsi
140fc69f2:     	pushq	%rdi
140fc69f3:     	subq	$0xa0, %rsp
140fc69fa:     	leaq	0x80(%rsp), %rbp
140fc6a02:     	movq	$-0x2, 0x18(%rbp)
140fc6a0a:     	movq	%rdx, %rdi
140fc6a0d:     	movq	%rcx, (%rbp)
140fc6a11:     	leaq	-0xebcb88(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140fc6a18:     	movq	%rax, 0x8(%rbp)
140fc6a1c:     	leaq	0x1bed96d(%rip), %rax   # 0x142bb4390
140fc6a23:     	movq	%rax, -0x30(%rbp)
140fc6a27:     	movq	$0x1, -0x28(%rbp)
140fc6a2f:     	movq	%rbp, %rax
140fc6a32:     	movq	%rax, -0x20(%rbp)
140fc6a36:     	movq	$0x1, -0x18(%rbp)
140fc6a3e:     	movq	$0x0, -0x10(%rbp)
140fc6a46:     	leaq	-0x50(%rbp), %rcx
140fc6a4a:     	leaq	-0x30(%rbp), %rdx
140fc6a4e:     	callq	0x140d79f90 <.text+0xd78f90>
140fc6a53:     	movq	-0x48(%rbp), %rax
140fc6a57:     	movq	-0x40(%rbp), %rsi
140fc6a5b:     	movq	%rax, -0x38(%rbp)
140fc6a5f:     	movq	%rax, -0x60(%rbp)
140fc6a63:     	movq	%rsi, -0x58(%rbp)
140fc6a67:     	leaq	-0x60(%rbp), %rcx
140fc6a6b:     	callq	0x1405bdda0 <.text+0x5bcda0>
140fc6a70:     	movq	%rax, -0x30(%rbp)
140fc6a74:     	movabsq	$-0x239fc26ae46c7f4e, %rax # imm = 0xDC603D951B9380B2
140fc6a7e:     	movq	%rax, -0x28(%rbp)
140fc6a82:     	movabsq	$-0x3196240f511ec25e, %rax # imm = 0xCE69DBF0AEE13DA2
140fc6a8c:     	movq	%rax, -0x20(%rbp)
140fc6a90:     	leaq	-0x30(%rbp), %rcx
140fc6a94:     	callq	0x1405bdf20 <.text+0x5bcf20>
140fc6a99:     	movq	0x8(%rdi), %rcx
140fc6a9d:     	movq	%rcx, -0x30(%rbp)
140fc6aa1:     	movq	%rax, -0x28(%rbp)
140fc6aa5:     	leaq	-0x30(%rbp), %rcx
140fc6aa9:     	callq	0x1405bd910 <.text+0x5bc910>
140fc6aae:     	leaq	-0x30(%rbp), %rcx
140fc6ab2:     	movq	%rdi, %rdx
140fc6ab5:     	movq	%rax, %r8
140fc6ab8:     	callq	0x1406861b0 <.text+0x6851b0>
140fc6abd:     	leaq	-0x30(%rbp), %rcx
140fc6ac1:     	xorl	%edx, %edx
140fc6ac3:     	callq	0x140f9d930 <.text+0xf9c930>
140fc6ac8:     	movq	%rax, %rcx
140fc6acb:     	movq	-0x38(%rbp), %rdx
140fc6acf:     	movq	%rsi, %r8
140fc6ad2:     	callq	0x140fac4c0 <.text+0xfab4c0>
140fc6ad7:     	movq	-0x20(%rbp), %rax
140fc6adb:     	movq	%rax, 0x10(%rbp)
140fc6adf:     	movups	-0x30(%rbp), %xmm0
140fc6ae3:     	movaps	%xmm0, (%rbp)
140fc6ae7:     	movq	%rbp, %rcx
140fc6aea:     	movb	$0x1, %dl
140fc6aec:     	callq	0x140f9d930 <.text+0xf9c930>
140fc6af1:     	movl	$0xffffffe0, 0x258(%rax) # imm = 0xFFFFFFE0
140fc6afb:     	movq	0x10(%rbp), %rax
140fc6aff:     	movq	%rax, -0x20(%rbp)
140fc6b03:     	movaps	(%rbp), %xmm0
140fc6b07:     	movaps	%xmm0, -0x30(%rbp)
140fc6b0b:     	leaq	-0x30(%rbp), %rcx
140fc6b0f:     	movb	$0x1, %dl
140fc6b11:     	callq	0x140f9d930 <.text+0xf9c930>
140fc6b16:     	movq	%rax, %rcx
140fc6b19:     	xorl	%edx, %edx
140fc6b1b:     	callq	0x140b90f40 <.text+0xb8ff40>
140fc6b20:     	movq	-0x20(%rbp), %rax
140fc6b24:     	movq	%rax, 0x10(%rbp)
140fc6b28:     	movaps	-0x30(%rbp), %xmm0
140fc6b2c:     	movaps	%xmm0, (%rbp)
140fc6b30:     	movq	%rbp, %rcx
140fc6b33:     	movb	$0x1, %dl
140fc6b35:     	callq	0x140f9d930 <.text+0xf9c930>
140fc6b3a:     	movss	0x1bec67e(%rip), %xmm1  # 0x142bb31c0
140fc6b42:     	movq	%rax, %rcx
140fc6b45:     	callq	0x140b90ec0 <.text+0xb8fec0>
140fc6b4a:     	movq	0x10(%rbp), %rax
140fc6b4e:     	movq	%rax, -0x20(%rbp)
140fc6b52:     	movaps	(%rbp), %xmm0
140fc6b56:     	movaps	%xmm0, -0x30(%rbp)
140fc6b5a:     	leaq	-0x30(%rbp), %rcx
140fc6b5e:     	movb	$0x1, %dl
140fc6b60:     	callq	0x140f9d930 <.text+0xf9c930>
140fc6b65:     	movss	0x1a06533(%rip), %xmm1  # 0x1429cd0a0
140fc6b6d:     	movq	%rax, %rcx
140fc6b70:     	callq	0x140fac150 <.text+0xfab150>
140fc6b75:     	movq	-0x50(%rbp), %rdx
140fc6b79:     	testq	%rdx, %rdx
140fc6b7c:     	je	0x140fc6b8d <.text+0xfc5b8d>
140fc6b7e:     	movl	$0x1, %r8d
140fc6b84:     	movq	-0x38(%rbp), %rcx
140fc6b88:     	callq	0x140613c20 <.text+0x612c20>
140fc6b8d:     	nop
140fc6b8e:     	addq	$0xa0, %rsp
140fc6b95:     	popq	%rdi
140fc6b96:     	popq	%rsi
140fc6b97:     	popq	%rbp
140fc6b98:     	retq
140fc6b99:     	nopl	(%rax)
140fc6ba0:     	movq	%rdx, 0x10(%rsp)
140fc6ba5:     	pushq	%rbp
140fc6ba6:     	pushq	%rsi
140fc6ba7:     	pushq	%rdi
140fc6ba8:     	subq	$0x20, %rsp
140fc6bac:     	leaq	0x80(%rdx), %rbp
140fc6bb3:     	movq	-0x50(%rbp), %rdx
140fc6bb7:     	testq	%rdx, %rdx
140fc6bba:     	je	0x140fc6bcb <.text+0xfc5bcb>
140fc6bbc:     	movl	$0x1, %r8d
140fc6bc2:     	movq	-0x38(%rbp), %rcx
140fc6bc6:     	callq	0x140613c20 <.text+0x612c20>
140fc6bcb:     	nop
140fc6bcc:     	addq	$0x20, %rsp
140fc6bd0:     	popq	%rdi
140fc6bd1:     	popq	%rsi
140fc6bd2:     	popq	%rbp
140fc6bd3:     	retq
