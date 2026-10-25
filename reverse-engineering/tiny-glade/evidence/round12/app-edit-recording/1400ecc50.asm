
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400ecc50:     	pushq	%rbp
1400ecc51:     	pushq	%rsi
1400ecc52:     	pushq	%rdi
1400ecc53:     	subq	$0x3e0, %rsp            # imm = 0x3E0
1400ecc5a:     	leaq	0x80(%rsp), %rbp
1400ecc62:     	movq	$-0x2, 0x358(%rbp)
1400ecc6d:     	movq	%rcx, %rsi
1400ecc70:     	xorps	%xmm0, %xmm0
1400ecc73:     	movups	%xmm0, -0x40(%rbp)
1400ecc77:     	movups	%xmm0, -0x28(%rbp)
1400ecc7b:     	movups	%xmm0, -0x10(%rbp)
1400ecc7f:     	movups	%xmm0, 0x8(%rbp)
1400ecc83:     	movups	%xmm0, 0x20(%rbp)
1400ecc87:     	movl	$0x0, 0x30(%rbp)
1400ecc8e:     	movups	%xmm0, 0x58(%rbp)
1400ecc92:     	movups	%xmm0, 0x70(%rbp)
1400ecc96:     	movups	%xmm0, 0x88(%rbp)
1400ecc9d:     	movups	%xmm0, 0xa0(%rbp)
1400ecca4:     	movups	%xmm0, 0xb8(%rbp)
1400eccab:     	movl	$0x0, 0xc8(%rbp)
1400eccb5:     	movq	$0x0, -0x60(%rbp)
1400eccbd:     	movq	$0x8, -0x58(%rbp)
1400eccc5:     	movq	$0x0, -0x50(%rbp)
1400ecccd:     	movq	$0x10, -0x48(%rbp)
1400eccd5:     	movq	$0x10, -0x30(%rbp)
1400eccdd:     	movq	$0x10, -0x18(%rbp)
1400ecce5:     	movq	$0x10, (%rbp)
1400ecced:     	movq	$0x10, 0x18(%rbp)
1400eccf5:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1400eccff:     	movq	%rax, 0x38(%rbp)
1400ecd03:     	leaq	0x2850728(%rip), %rcx   # 0x14293d432
1400ecd0a:     	movq	%rcx, 0x40(%rbp)
1400ecd0e:     	movq	$0x4d, 0x48(%rbp)
1400ecd16:     	movq	$0x10, 0x50(%rbp)
1400ecd1e:     	movq	$0x10, 0x68(%rbp)
1400ecd26:     	movq	$0x10, 0x80(%rbp)
1400ecd31:     	movq	$0x10, 0x98(%rbp)
1400ecd3c:     	movq	$0x10, 0xb0(%rbp)
1400ecd47:     	movl	$0x0, 0xd0(%rbp)
1400ecd51:     	movw	$0x1, 0xd4(%rbp)
1400ecd5a:     	movq	%rax, 0x1c8(%rbp)
1400ecd61:     	movl	$0x0, 0x350(%rbp)
1400ecd6b:     	movzbl	0x3547e97(%rip), %eax   # 0x143634c09
1400ecd72:     	movl	$0x3b8, %ecx            # imm = 0x3B8
1400ecd77:     	movl	$0x8, %edx
1400ecd7c:     	callq	0x140613c10 <.text+0x612c10>
1400ecd81:     	testq	%rax, %rax
1400ecd84:     	je	0x1400ecdbb <.text+0xebdbb>
1400ecd86:     	movq	%rax, %rdi
1400ecd89:     	leaq	-0x60(%rbp), %rdx
1400ecd8d:     	movl	$0x3b8, %r8d            # imm = 0x3B8
1400ecd93:     	movq	%rax, %rcx
1400ecd96:     	callq	0x142923c00 <.text+0x2922c00>
1400ecd9b:     	leaq	0x285034e(%rip), %r8    # 0x14293d0f0
1400ecda2:     	movq	%rsi, %rcx
1400ecda5:     	movq	%rdi, %rdx
1400ecda8:     	callq	0x140ce4b10 <.text+0xce3b10>
1400ecdad:     	movq	%rsi, %rax
1400ecdb0:     	addq	$0x3e0, %rsp            # imm = 0x3E0
1400ecdb7:     	popq	%rdi
1400ecdb8:     	popq	%rsi
1400ecdb9:     	popq	%rbp
1400ecdba:     	retq
1400ecdbb:     	movl	$0x8, %ecx
1400ecdc0:     	movl	$0x3b8, %edx            # imm = 0x3B8
1400ecdc5:     	callq	0x1428d9000 <.text+0x28d8000>
1400ecdca:     	ud2
