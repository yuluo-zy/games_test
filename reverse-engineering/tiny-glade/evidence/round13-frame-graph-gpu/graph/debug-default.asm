
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14093fce0:     	pushq	%rbp
14093fce1:     	pushq	%r14
14093fce3:     	pushq	%rsi
14093fce4:     	pushq	%rdi
14093fce5:     	pushq	%rbx
14093fce6:     	subq	$0xd0, %rsp
14093fced:     	leaq	0x80(%rsp), %rbp
14093fcf5:     	movq	$-0x2, 0x48(%rbp)
14093fcfd:     	movq	%rcx, %rsi
14093fd00:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14093fd0a:     	movq	%rax, -0x60(%rbp)
14093fd0e:     	movq	%rax, -0x40(%rbp)
14093fd12:     	movq	%rax, -0x20(%rbp)
14093fd16:     	movq	%rax, (%rbp)
14093fd1a:     	movq	%rax, 0x20(%rbp)
14093fd1e:     	movq	$0x0, 0x40(%rbp)
14093fd26:     	callq	0x140d4d740 <.text+0xd4c740>
14093fd2b:     	movq	%rax, %r14
14093fd2e:     	testb	$0x1, (%rax)
14093fd31:     	je	0x14093fd44 <.text+0x93ed44>
14093fd33:     	movq	%r14, %rax
14093fd36:     	addq	$0x8, %rax
14093fd3a:     	movq	0x8(%r14), %rdi
14093fd3e:     	movq	0x10(%r14), %rbx
14093fd42:     	jmp	0x14093fd65 <.text+0x93ed65>
14093fd44:     	callq	0x140d64e00 <.text+0xd63e00>
14093fd49:     	movq	%rax, %rdi
14093fd4c:     	movq	%rdx, %rbx
14093fd4f:     	movq	%r14, %rax
14093fd52:     	addq	$0x8, %rax
14093fd56:     	movq	$0x1, (%r14)
14093fd5d:     	movq	%rdi, 0x8(%r14)
14093fd61:     	movq	%rdx, 0x10(%r14)
14093fd65:     	leaq	0x1(%rdi), %rcx
14093fd69:     	movq	%rcx, (%rax)
14093fd6c:     	movb	$0x0, 0x102(%rsi)
14093fd73:     	movl	$0x0, 0xf0(%rsi)
14093fd7d:     	movb	$0x0, 0xf4(%rsi)
14093fd84:     	movw	$0x0, 0xf5(%rsi)
14093fd8d:     	movsd	0x21725db(%rip), %xmm0  # 0x142ab2370
14093fd95:     	movsd	%xmm0, 0xe8(%rsi)
14093fd9d:     	movq	$0x0, (%rsi)
14093fda4:     	movb	$0x1, 0xf7(%rsi)
14093fdab:     	leaq	0x10(%rsi), %rcx
14093fdaf:     	leaq	-0x60(%rbp), %rdx
14093fdb3:     	movl	$0xa8, %r8d
14093fdb9:     	callq	0x142923c00 <.text+0x2922c00>
14093fdbe:     	movb	$0x0, 0xf8(%rsi)
14093fdc5:     	movw	$0x0, 0x103(%rsi)
14093fdce:     	movups	0x217271b(%rip), %xmm0  # 0x142ab24f0
14093fdd5:     	movups	%xmm0, 0xb8(%rsi)
14093fddc:     	movups	0x217271d(%rip), %xmm0  # 0x142ab2500
14093fde3:     	movups	%xmm0, 0xc8(%rsi)
14093fdea:     	movq	%rdi, 0xd8(%rsi)
14093fdf1:     	movq	%rbx, 0xe0(%rsi)
14093fdf8:     	movq	$0x0, 0xf9(%rsi)
14093fe03:     	movb	$0x0, 0x101(%rsi)
14093fe0a:     	movq	%rsi, %rax
14093fe0d:     	addq	$0xd0, %rsp
14093fe14:     	popq	%rbx
14093fe15:     	popq	%rdi
14093fe16:     	popq	%rsi
14093fe17:     	popq	%r14
14093fe19:     	popq	%rbp
14093fe1a:     	retq
14093fe1b:     	nopl	(%rax,%rax)
14093fe20:     	movq	%rdx, 0x10(%rsp)
14093fe25:     	pushq	%rbp
14093fe26:     	pushq	%r14
14093fe28:     	pushq	%rsi
14093fe29:     	pushq	%rdi
14093fe2a:     	pushq	%rbx
14093fe2b:     	subq	$0x20, %rsp
14093fe2f:     	leaq	0x80(%rdx), %rbp
14093fe36:     	leaq	-0x60(%rbp), %rcx
14093fe3a:     	callq	0x14088a940 <.text+0x889940>
14093fe3f:     	nop
14093fe40:     	addq	$0x20, %rsp
14093fe44:     	popq	%rbx
14093fe45:     	popq	%rdi
14093fe46:     	popq	%rsi
14093fe47:     	popq	%r14
14093fe49:     	popq	%rbp
14093fe4a:     	retq
