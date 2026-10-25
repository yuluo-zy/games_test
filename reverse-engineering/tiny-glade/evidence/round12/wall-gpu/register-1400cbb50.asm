
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400cbb50:     	pushq	%rbp
1400cbb51:     	pushq	%rsi
1400cbb52:     	pushq	%rdi
1400cbb53:     	subq	$0x300, %rsp            # imm = 0x300
1400cbb5a:     	leaq	0x80(%rsp), %rbp
1400cbb62:     	movq	$-0x2, 0x278(%rbp)
1400cbb6d:     	movq	%rcx, %rsi
1400cbb70:     	xorps	%xmm0, %xmm0
1400cbb73:     	movups	%xmm0, -0x40(%rbp)
1400cbb77:     	movups	%xmm0, -0x28(%rbp)
1400cbb7b:     	movups	%xmm0, -0x10(%rbp)
1400cbb7f:     	movups	%xmm0, 0x8(%rbp)
1400cbb83:     	movups	%xmm0, 0x20(%rbp)
1400cbb87:     	movl	$0x0, 0x30(%rbp)
1400cbb8e:     	movups	%xmm0, 0x58(%rbp)
1400cbb92:     	movups	%xmm0, 0x70(%rbp)
1400cbb96:     	movups	%xmm0, 0x88(%rbp)
1400cbb9d:     	movups	%xmm0, 0xa0(%rbp)
1400cbba4:     	movups	%xmm0, 0xb8(%rbp)
1400cbbab:     	movl	$0x0, 0xc8(%rbp)
1400cbbb5:     	movq	$0x0, -0x60(%rbp)
1400cbbbd:     	movq	$0x8, -0x58(%rbp)
1400cbbc5:     	movq	$0x0, -0x50(%rbp)
1400cbbcd:     	movq	$0x10, -0x48(%rbp)
1400cbbd5:     	movq	$0x10, -0x30(%rbp)
1400cbbdd:     	movq	$0x10, -0x18(%rbp)
1400cbbe5:     	movq	$0x10, (%rbp)
1400cbbed:     	movq	$0x10, 0x18(%rbp)
1400cbbf5:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1400cbbff:     	movq	%rax, 0x38(%rbp)
1400cbc03:     	leaq	0x286bac1(%rip), %rcx   # 0x1429376cb
1400cbc0a:     	movq	%rcx, 0x40(%rbp)
1400cbc0e:     	movq	$0x67, 0x48(%rbp)
1400cbc16:     	movq	$0x10, 0x50(%rbp)
1400cbc1e:     	movq	$0x10, 0x68(%rbp)
1400cbc26:     	movq	$0x10, 0x80(%rbp)
1400cbc31:     	movq	$0x10, 0x98(%rbp)
1400cbc3c:     	movq	$0x10, 0xb0(%rbp)
1400cbc47:     	movl	$0x0, 0xd0(%rbp)
1400cbc51:     	movw	$0x1, 0xd4(%rbp)
1400cbc5a:     	movq	%rax, 0x1d0(%rbp)
1400cbc61:     	movl	$0x0, 0x270(%rbp)
1400cbc6b:     	movzbl	0x3568f97(%rip), %eax   # 0x143634c09
1400cbc72:     	movl	$0x2d8, %ecx            # imm = 0x2D8
1400cbc77:     	movl	$0x8, %edx
1400cbc7c:     	callq	0x140613c10 <.text+0x612c10>
1400cbc81:     	testq	%rax, %rax
1400cbc84:     	je	0x1400cbcbb <.text+0xcacbb>
1400cbc86:     	movq	%rax, %rdi
1400cbc89:     	leaq	-0x60(%rbp), %rdx
1400cbc8d:     	movl	$0x2d8, %r8d            # imm = 0x2D8
1400cbc93:     	movq	%rax, %rcx
1400cbc96:     	callq	0x142923c00 <.text+0x2922c00>
1400cbc9b:     	leaq	0x286b8e6(%rip), %r8    # 0x142937588
1400cbca2:     	movq	%rsi, %rcx
1400cbca5:     	movq	%rdi, %rdx
1400cbca8:     	callq	0x140ce4b10 <.text+0xce3b10>
1400cbcad:     	movq	%rsi, %rax
1400cbcb0:     	addq	$0x300, %rsp            # imm = 0x300
1400cbcb7:     	popq	%rdi
1400cbcb8:     	popq	%rsi
1400cbcb9:     	popq	%rbp
1400cbcba:     	retq
1400cbcbb:     	movl	$0x8, %ecx
1400cbcc0:     	movl	$0x2d8, %edx            # imm = 0x2D8
1400cbcc5:     	callq	0x1428d9000 <.text+0x28d8000>
1400cbcca:     	ud2
