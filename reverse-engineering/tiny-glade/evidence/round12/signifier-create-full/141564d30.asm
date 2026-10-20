
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141564d30:     	pushq	%rbp
141564d31:     	subq	$0xc0, %rsp
141564d38:     	leaq	0x80(%rsp), %rbp
141564d40:     	movq	$-0x2, 0x38(%rbp)
141564d48:     	movq	%rcx, 0x30(%rbp)
141564d4c:     	movl	(%rdx), %eax
141564d4e:     	leaq	0x1710f73(%rip), %rcx   # 0x142c75cc8
141564d55:     	movslq	(%rcx,%rax,4), %rax
141564d59:     	addq	%rcx, %rax
141564d5c:     	jmpq	*%rax
141564d5e:     	leaq	0x1710a0b(%rip), %rax   # 0x142c75770
141564d65:     	movq	%rax, 0x20(%rbp)
141564d69:     	movl	$0x6, %eax
141564d6e:     	jmp	0x141564da4 <.text+0x1563da4>
141564d70:     	leaq	0x1710a0c(%rip), %rax   # 0x142c75783
141564d77:     	movq	%rax, 0x20(%rbp)
141564d7b:     	movl	$0x7, %eax
141564d80:     	jmp	0x141564da4 <.text+0x1563da4>
141564d82:     	leaq	0x1710a01(%rip), %rax   # 0x142c7578a
141564d89:     	movq	%rax, 0x20(%rbp)
141564d8d:     	movl	$0x8, %eax
141564d92:     	jmp	0x141564da4 <.text+0x1563da4>
141564d94:     	leaq	0x17109db(%rip), %rax   # 0x142c75776
141564d9b:     	movq	%rax, 0x20(%rbp)
141564d9f:     	movl	$0xd, %eax
141564da4:     	movq	%rax, 0x28(%rbp)
141564da8:     	movq	0x10(%rdx), %rcx
141564dac:     	movq	0x18(%rdx), %rax
141564db0:     	movq	%rcx, 0x18(%rbp)
141564db4:     	movq	%rcx, (%rbp)
141564db8:     	movq	%rax, 0x8(%rbp)
141564dbc:     	leaq	0x20(%rbp), %rax
141564dc0:     	movq	%rax, -0x30(%rbp)
141564dc4:     	leaq	-0x145af3b(%rip), %rax  # 0x140109e90 <.text+0x108e90>
141564dcb:     	movq	%rax, -0x28(%rbp)
141564dcf:     	movq	%rbp, %rcx
141564dd2:     	movq	%rcx, -0x20(%rbp)
141564dd6:     	movq	%rax, -0x18(%rbp)
141564dda:     	leaq	0x30(%rbp), %rax
141564dde:     	movq	%rax, -0x10(%rbp)
141564de2:     	leaq	-0x8813a9(%rip), %rax   # 0x140ce3a40 <.text+0xce2a40>
141564de9:     	movq	%rax, -0x8(%rbp)
141564ded:     	leaq	0x17109bc(%rip), %rax   # 0x142c757b0
141564df4:     	movq	%rax, -0x60(%rbp)
141564df8:     	movq	$0x3, -0x58(%rbp)
141564e00:     	movq	$0x0, -0x40(%rbp)
141564e08:     	leaq	-0x30(%rbp), %rax
141564e0c:     	movq	%rax, -0x50(%rbp)
141564e10:     	movq	$0x3, -0x48(%rbp)
141564e18:     	movq	%rdx, 0x10(%rbp)
141564e1c:     	leaq	0x1710935(%rip), %rdx   # 0x142c75758
141564e23:     	leaq	-0x60(%rbp), %rcx
141564e27:     	callq	0x1428d9390 <.text+0x28d8390>
141564e2c:     	ud2
