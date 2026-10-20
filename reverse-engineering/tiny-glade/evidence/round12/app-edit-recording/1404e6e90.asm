
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404e6e90:     	pushq	%rbp
1404e6e91:     	pushq	%rsi
1404e6e92:     	pushq	%rdi
1404e6e93:     	subq	$0x1c0, %rsp            # imm = 0x1C0
1404e6e9a:     	leaq	0x80(%rsp), %rbp
1404e6ea2:     	movq	$-0x2, 0x138(%rbp)
1404e6ead:     	movq	%rcx, %rsi
1404e6eb0:     	xorps	%xmm0, %xmm0
1404e6eb3:     	movups	%xmm0, 0x18(%rbp)
1404e6eb7:     	movups	%xmm0, 0x30(%rbp)
1404e6ebb:     	movups	%xmm0, 0x48(%rbp)
1404e6ebf:     	movups	%xmm0, 0x60(%rbp)
1404e6ec3:     	movups	%xmm0, 0x78(%rbp)
1404e6ec7:     	movl	$0x0, 0x88(%rbp)
1404e6ed1:     	movups	%xmm0, 0xb0(%rbp)
1404e6ed8:     	movups	%xmm0, 0xc8(%rbp)
1404e6edf:     	movups	%xmm0, 0xe0(%rbp)
1404e6ee6:     	movups	%xmm0, 0xf8(%rbp)
1404e6eed:     	movups	%xmm0, 0x110(%rbp)
1404e6ef4:     	movl	$0x0, 0x120(%rbp)
1404e6efe:     	movq	$0x0, -0x58(%rbp)
1404e6f06:     	movq	$0x0, -0x8(%rbp)
1404e6f0e:     	movq	$0x8, (%rbp)
1404e6f16:     	movq	$0x0, 0x8(%rbp)
1404e6f1e:     	movq	$0x10, 0x10(%rbp)
1404e6f26:     	movq	$0x10, 0x28(%rbp)
1404e6f2e:     	movq	$0x10, 0x40(%rbp)
1404e6f36:     	movq	$0x10, 0x58(%rbp)
1404e6f3e:     	movq	$0x10, 0x70(%rbp)
1404e6f46:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1404e6f50:     	movq	%rax, 0x90(%rbp)
1404e6f57:     	leaq	0x2504733(%rip), %rax   # 0x1429eb691
1404e6f5e:     	movq	%rax, 0x98(%rbp)
1404e6f65:     	movq	$0x65, 0xa0(%rbp)
1404e6f70:     	movq	$0x10, 0xa8(%rbp)
1404e6f7b:     	movq	$0x10, 0xc0(%rbp)
1404e6f86:     	movq	$0x10, 0xd8(%rbp)
1404e6f91:     	movq	$0x10, 0xf0(%rbp)
1404e6f9c:     	movq	$0x10, 0x108(%rbp)
1404e6fa7:     	movl	$0x0, 0x128(%rbp)
1404e6fb1:     	movw	$0x1, 0x12c(%rbp)
1404e6fba:     	movl	$0x0, 0x130(%rbp)
1404e6fc4:     	movzbl	0x314dc3e(%rip), %eax   # 0x143634c09
1404e6fcb:     	movl	$0x190, %ecx            # imm = 0x190
1404e6fd0:     	movl	$0x8, %edx
1404e6fd5:     	callq	0x140613c10 <.text+0x612c10>
1404e6fda:     	testq	%rax, %rax
1404e6fdd:     	je	0x1404e7014 <.text+0x4e6014>
1404e6fdf:     	movq	%rax, %rdi
1404e6fe2:     	leaq	-0x58(%rbp), %rdx
1404e6fe6:     	movl	$0x190, %r8d            # imm = 0x190
1404e6fec:     	movq	%rax, %rcx
1404e6fef:     	callq	0x142923c00 <.text+0x2922c00>
1404e6ff4:     	leaq	0x2504365(%rip), %r8    # 0x1429eb360
1404e6ffb:     	movq	%rsi, %rcx
1404e6ffe:     	movq	%rdi, %rdx
1404e7001:     	callq	0x140ce4b10 <.text+0xce3b10>
1404e7006:     	movq	%rsi, %rax
1404e7009:     	addq	$0x1c0, %rsp            # imm = 0x1C0
1404e7010:     	popq	%rdi
1404e7011:     	popq	%rsi
1404e7012:     	popq	%rbp
1404e7013:     	retq
1404e7014:     	movl	$0x8, %ecx
1404e7019:     	movl	$0x190, %edx            # imm = 0x190
1404e701e:     	callq	0x1428d9000 <.text+0x28d8000>
1404e7023:     	ud2
