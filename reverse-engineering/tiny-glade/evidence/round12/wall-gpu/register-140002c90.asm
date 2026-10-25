
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140002c90:     	pushq	%rbp
140002c91:     	pushq	%rsi
140002c92:     	pushq	%rdi
140002c93:     	subq	$0x470, %rsp            # imm = 0x470
140002c9a:     	leaq	0x80(%rsp), %rbp
140002ca2:     	movq	$-0x2, 0x3e8(%rbp)
140002cad:     	movq	%rcx, %rsi
140002cb0:     	xorps	%xmm0, %xmm0
140002cb3:     	movups	%xmm0, -0x38(%rbp)
140002cb7:     	movups	%xmm0, -0x20(%rbp)
140002cbb:     	movups	%xmm0, -0x8(%rbp)
140002cbf:     	movups	%xmm0, 0x10(%rbp)
140002cc3:     	movups	%xmm0, 0x28(%rbp)
140002cc7:     	movl	$0x0, 0x38(%rbp)
140002cce:     	movups	%xmm0, 0x60(%rbp)
140002cd2:     	movups	%xmm0, 0x78(%rbp)
140002cd6:     	movups	%xmm0, 0x90(%rbp)
140002cdd:     	movups	%xmm0, 0xa8(%rbp)
140002ce4:     	movups	%xmm0, 0xc0(%rbp)
140002ceb:     	movl	$0x0, 0xd0(%rbp)
140002cf5:     	movq	$0x0, -0x58(%rbp)
140002cfd:     	movq	$0x8, -0x50(%rbp)
140002d05:     	movq	$0x0, -0x48(%rbp)
140002d0d:     	movq	$0x10, -0x40(%rbp)
140002d15:     	movq	$0x10, -0x28(%rbp)
140002d1d:     	movq	$0x10, -0x10(%rbp)
140002d25:     	movq	$0x10, 0x8(%rbp)
140002d2d:     	movq	$0x10, 0x20(%rbp)
140002d35:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140002d3f:     	movq	%rax, 0x40(%rbp)
140002d43:     	leaq	0x2922aeb(%rip), %rcx   # 0x142925835
140002d4a:     	movq	%rcx, 0x48(%rbp)
140002d4e:     	movq	$0x39, 0x50(%rbp)
140002d56:     	movq	$0x10, 0x58(%rbp)
140002d5e:     	movq	$0x10, 0x70(%rbp)
140002d66:     	movq	$0x10, 0x88(%rbp)
140002d71:     	movq	$0x10, 0xa0(%rbp)
140002d7c:     	movq	$0x10, 0xb8(%rbp)
140002d87:     	movl	$0x0, 0xd8(%rbp)
140002d91:     	movw	$0x1, 0xdc(%rbp)
140002d9a:     	movq	%rax, 0xe0(%rbp)
140002da1:     	movl	$0x0, 0x3e0(%rbp)
140002dab:     	movzbl	0x3631e57(%rip), %eax   # 0x143634c09
140002db2:     	movl	$0x440, %ecx            # imm = 0x440
140002db7:     	movl	$0x8, %edx
140002dbc:     	callq	0x140613c10 <.text+0x612c10>
140002dc1:     	testq	%rax, %rax
140002dc4:     	je	0x140002dfb <.text+0x1dfb>
140002dc6:     	movq	%rax, %rdi
140002dc9:     	leaq	-0x58(%rbp), %rdx
140002dcd:     	movl	$0x440, %r8d            # imm = 0x440
140002dd3:     	movq	%rax, %rcx
140002dd6:     	callq	0x142923c00 <.text+0x2922c00>
140002ddb:     	leaq	0x292297e(%rip), %r8    # 0x142925760
140002de2:     	movq	%rsi, %rcx
140002de5:     	movq	%rdi, %rdx
140002de8:     	callq	0x140ce4b10 <.text+0xce3b10>
140002ded:     	movq	%rsi, %rax
140002df0:     	addq	$0x470, %rsp            # imm = 0x470
140002df7:     	popq	%rdi
140002df8:     	popq	%rsi
140002df9:     	popq	%rbp
140002dfa:     	retq
140002dfb:     	movl	$0x8, %ecx
140002e00:     	movl	$0x440, %edx            # imm = 0x440
140002e05:     	callq	0x1428d9000 <.text+0x28d8000>
140002e0a:     	ud2
