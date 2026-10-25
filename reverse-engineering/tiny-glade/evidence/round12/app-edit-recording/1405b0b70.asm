
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405b0b70:     	pushq	%rbp
1405b0b71:     	pushq	%rsi
1405b0b72:     	pushq	%rdi
1405b0b73:     	subq	$0x1e0, %rsp            # imm = 0x1E0
1405b0b7a:     	leaq	0x80(%rsp), %rbp
1405b0b82:     	movq	$-0x2, 0x158(%rbp)
1405b0b8d:     	movq	%rcx, %rsi
1405b0b90:     	xorps	%xmm0, %xmm0
1405b0b93:     	movups	%xmm0, 0x38(%rbp)
1405b0b97:     	movups	%xmm0, 0x50(%rbp)
1405b0b9b:     	movups	%xmm0, 0x68(%rbp)
1405b0b9f:     	movups	%xmm0, 0x80(%rbp)
1405b0ba6:     	movups	%xmm0, 0x98(%rbp)
1405b0bad:     	movl	$0x0, 0xa8(%rbp)
1405b0bb7:     	movups	%xmm0, 0xd0(%rbp)
1405b0bbe:     	movups	%xmm0, 0xe8(%rbp)
1405b0bc5:     	movups	%xmm0, 0x100(%rbp)
1405b0bcc:     	movups	%xmm0, 0x118(%rbp)
1405b0bd3:     	movups	%xmm0, 0x130(%rbp)
1405b0bda:     	movl	$0x0, 0x140(%rbp)
1405b0be4:     	movq	$0x0, -0x60(%rbp)
1405b0bec:     	movq	$0x0, 0x18(%rbp)
1405b0bf4:     	movq	$0x8, 0x20(%rbp)
1405b0bfc:     	movq	$0x0, 0x28(%rbp)
1405b0c04:     	movq	$0x10, 0x30(%rbp)
1405b0c0c:     	movq	$0x10, 0x48(%rbp)
1405b0c14:     	movq	$0x10, 0x60(%rbp)
1405b0c1c:     	movq	$0x10, 0x78(%rbp)
1405b0c24:     	movq	$0x10, 0x90(%rbp)
1405b0c2f:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1405b0c39:     	movq	%rax, 0xb0(%rbp)
1405b0c40:     	leaq	0x244ff97(%rip), %rax   # 0x142a00bde
1405b0c47:     	movq	%rax, 0xb8(%rbp)
1405b0c4e:     	movq	$0x3f, 0xc0(%rbp)
1405b0c59:     	movq	$0x10, 0xc8(%rbp)
1405b0c64:     	movq	$0x10, 0xe0(%rbp)
1405b0c6f:     	movq	$0x10, 0xf8(%rbp)
1405b0c7a:     	movq	$0x10, 0x110(%rbp)
1405b0c85:     	movq	$0x10, 0x128(%rbp)
1405b0c90:     	movl	$0x0, 0x148(%rbp)
1405b0c9a:     	movw	$0x1, 0x14c(%rbp)
1405b0ca3:     	movl	$0x0, 0x150(%rbp)
1405b0cad:     	movzbl	0x3083f55(%rip), %eax   # 0x143634c09
1405b0cb4:     	movl	$0x1b8, %ecx            # imm = 0x1B8
1405b0cb9:     	movl	$0x8, %edx
1405b0cbe:     	callq	0x140613c10 <.text+0x612c10>
1405b0cc3:     	testq	%rax, %rax
1405b0cc6:     	je	0x1405b0cfd <.text+0x5afcfd>
1405b0cc8:     	movq	%rax, %rdi
1405b0ccb:     	leaq	-0x60(%rbp), %rdx
1405b0ccf:     	movl	$0x1b8, %r8d            # imm = 0x1B8
1405b0cd5:     	movq	%rax, %rcx
1405b0cd8:     	callq	0x142923c00 <.text+0x2922c00>
1405b0cdd:     	leaq	0x244fddc(%rip), %r8    # 0x142a00ac0
1405b0ce4:     	movq	%rsi, %rcx
1405b0ce7:     	movq	%rdi, %rdx
1405b0cea:     	callq	0x140ce4b10 <.text+0xce3b10>
1405b0cef:     	movq	%rsi, %rax
1405b0cf2:     	addq	$0x1e0, %rsp            # imm = 0x1E0
1405b0cf9:     	popq	%rdi
1405b0cfa:     	popq	%rsi
1405b0cfb:     	popq	%rbp
1405b0cfc:     	retq
1405b0cfd:     	movl	$0x8, %ecx
1405b0d02:     	movl	$0x1b8, %edx            # imm = 0x1B8
1405b0d07:     	callq	0x1428d9000 <.text+0x28d8000>
1405b0d0c:     	ud2
