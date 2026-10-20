
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14068bf00:     	pushq	%r15
14068bf02:     	pushq	%r14
14068bf04:     	pushq	%rsi
14068bf05:     	pushq	%rdi
14068bf06:     	pushq	%rbx
14068bf07:     	subq	$0x50, %rsp
14068bf0b:     	movq	%r9, %rdi
14068bf0e:     	movq	%r8, %rbx
14068bf11:     	movq	%rdx, %r14
14068bf14:     	movq	%rcx, %rsi
14068bf17:     	movq	%r8, 0x40(%rsp)
14068bf1c:     	movq	%r9, 0x48(%rsp)
14068bf21:     	leaq	0x40(%rsp), %rcx
14068bf26:     	callq	0x1405bdda0 <.text+0x5bcda0>
14068bf2b:     	movq	%rax, 0x28(%rsp)
14068bf30:     	movabsq	$-0x239fc26ae46c7f4e, %rax # imm = 0xDC603D951B9380B2
14068bf3a:     	movq	%rax, 0x30(%rsp)
14068bf3f:     	movabsq	$-0x3196240f511ec25e, %rax # imm = 0xCE69DBF0AEE13DA2
14068bf49:     	movq	%rax, 0x38(%rsp)
14068bf4e:     	leaq	0x28(%rsp), %rcx
14068bf53:     	callq	0x1405bdf20 <.text+0x5bcf20>
14068bf58:     	movq	0x8(%r14), %rcx
14068bf5c:     	movq	%rcx, 0x28(%rsp)
14068bf61:     	movq	%rax, 0x30(%rsp)
14068bf66:     	leaq	0x28(%rsp), %rcx
14068bf6b:     	callq	0x1405bd910 <.text+0x5bc910>
14068bf70:     	leaq	0x28(%rsp), %r15
14068bf75:     	movq	%r15, %rcx
14068bf78:     	movq	%r14, %rdx
14068bf7b:     	movq	%rax, %r8
14068bf7e:     	callq	0x1406861b0 <.text+0x6851b0>
14068bf83:     	movq	%r15, %rcx
14068bf86:     	xorl	%edx, %edx
14068bf88:     	callq	0x140f9d930 <.text+0xf9c930>
14068bf8d:     	movq	%rax, %rcx
14068bf90:     	movq	%rbx, %rdx
14068bf93:     	movq	%rdi, %r8
14068bf96:     	callq	0x140fac4c0 <.text+0xfab4c0>
14068bf9b:     	movq	0x38(%rsp), %rax
14068bfa0:     	movq	%rax, 0x10(%rsi)
14068bfa4:     	movups	0x28(%rsp), %xmm0
14068bfa9:     	movups	%xmm0, (%rsi)
14068bfac:     	movq	%rsi, %rax
14068bfaf:     	addq	$0x50, %rsp
14068bfb3:     	popq	%rbx
14068bfb4:     	popq	%rdi
14068bfb5:     	popq	%rsi
14068bfb6:     	popq	%r14
14068bfb8:     	popq	%r15
14068bfba:     	retq
