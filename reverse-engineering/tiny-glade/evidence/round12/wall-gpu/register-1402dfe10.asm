
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1402dfe10:     	pushq	%rbp
1402dfe11:     	pushq	%rsi
1402dfe12:     	pushq	%rdi
1402dfe13:     	subq	$0x1b0, %rsp            # imm = 0x1B0
1402dfe1a:     	leaq	0x80(%rsp), %rbp
1402dfe22:     	movq	$-0x2, 0x128(%rbp)
1402dfe2d:     	movq	%rcx, %rsi
1402dfe30:     	xorps	%xmm0, %xmm0
1402dfe33:     	movups	%xmm0, 0x8(%rbp)
1402dfe37:     	movups	%xmm0, 0x20(%rbp)
1402dfe3b:     	movups	%xmm0, 0x38(%rbp)
1402dfe3f:     	movups	%xmm0, 0x50(%rbp)
1402dfe43:     	movups	%xmm0, 0x68(%rbp)
1402dfe47:     	movl	$0x0, 0x78(%rbp)
1402dfe4e:     	movups	%xmm0, 0xa0(%rbp)
1402dfe55:     	movups	%xmm0, 0xb8(%rbp)
1402dfe5c:     	movups	%xmm0, 0xd0(%rbp)
1402dfe63:     	movups	%xmm0, 0xe8(%rbp)
1402dfe6a:     	movups	%xmm0, 0x100(%rbp)
1402dfe71:     	movl	$0x0, 0x110(%rbp)
1402dfe7b:     	movq	$0x0, -0x58(%rbp)
1402dfe83:     	movq	$0x0, -0x18(%rbp)
1402dfe8b:     	movq	$0x8, -0x10(%rbp)
1402dfe93:     	movq	$0x0, -0x8(%rbp)
1402dfe9b:     	movq	$0x10, (%rbp)
1402dfea3:     	movq	$0x10, 0x18(%rbp)
1402dfeab:     	movq	$0x10, 0x30(%rbp)
1402dfeb3:     	movq	$0x10, 0x48(%rbp)
1402dfebb:     	movq	$0x10, 0x60(%rbp)
1402dfec3:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1402dfecd:     	movq	%rax, 0x80(%rbp)
1402dfed4:     	leaq	0x26d3e21(%rip), %rax   # 0x1429b3cfc
1402dfedb:     	movq	%rax, 0x88(%rbp)
1402dfee2:     	movq	$0x51, 0x90(%rbp)
1402dfeed:     	movq	$0x10, 0x98(%rbp)
1402dfef8:     	movq	$0x10, 0xb0(%rbp)
1402dff03:     	movq	$0x10, 0xc8(%rbp)
1402dff0e:     	movq	$0x10, 0xe0(%rbp)
1402dff19:     	movq	$0x10, 0xf8(%rbp)
1402dff24:     	movl	$0x0, 0x118(%rbp)
1402dff2e:     	movw	$0x1, 0x11c(%rbp)
1402dff37:     	movl	$0x0, 0x120(%rbp)
1402dff41:     	movzbl	0x3354cc1(%rip), %eax   # 0x143634c09
1402dff48:     	movl	$0x180, %ecx            # imm = 0x180
1402dff4d:     	movl	$0x8, %edx
1402dff52:     	callq	0x140613c10 <.text+0x612c10>
1402dff57:     	testq	%rax, %rax
1402dff5a:     	je	0x1402dff91 <.text+0x2def91>
1402dff5c:     	movq	%rax, %rdi
1402dff5f:     	leaq	-0x58(%rbp), %rdx
1402dff63:     	movl	$0x180, %r8d            # imm = 0x180
1402dff69:     	movq	%rax, %rcx
1402dff6c:     	callq	0x142923c00 <.text+0x2922c00>
1402dff71:     	leaq	0x26d3c10(%rip), %r8    # 0x1429b3b88
1402dff78:     	movq	%rsi, %rcx
1402dff7b:     	movq	%rdi, %rdx
1402dff7e:     	callq	0x140ce4b10 <.text+0xce3b10>
1402dff83:     	movq	%rsi, %rax
1402dff86:     	addq	$0x1b0, %rsp            # imm = 0x1B0
1402dff8d:     	popq	%rdi
1402dff8e:     	popq	%rsi
1402dff8f:     	popq	%rbp
1402dff90:     	retq
1402dff91:     	movl	$0x8, %ecx
1402dff96:     	movl	$0x180, %edx            # imm = 0x180
1402dff9b:     	callq	0x1428d9000 <.text+0x28d8000>
1402dffa0:     	ud2
