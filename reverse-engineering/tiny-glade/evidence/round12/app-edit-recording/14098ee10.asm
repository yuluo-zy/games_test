
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14098ee10:     	pushq	%rbp
14098ee11:     	pushq	%rsi
14098ee12:     	pushq	%rdi
14098ee13:     	pushq	%rbx
14098ee14:     	subq	$0x228, %rsp            # imm = 0x228
14098ee1b:     	leaq	0x80(%rsp), %rbp
14098ee23:     	movq	$-0x2, 0x1a0(%rbp)
14098ee2e:     	movq	%rcx, %rsi
14098ee31:     	xorps	%xmm0, %xmm0
14098ee34:     	movups	%xmm0, 0x80(%rbp)
14098ee3b:     	movups	%xmm0, 0x98(%rbp)
14098ee42:     	movups	%xmm0, 0xb0(%rbp)
14098ee49:     	movups	%xmm0, 0xc8(%rbp)
14098ee50:     	movups	%xmm0, 0xe0(%rbp)
14098ee57:     	movl	$0x0, 0xf0(%rbp)
14098ee61:     	movups	%xmm0, 0x118(%rbp)
14098ee68:     	movups	%xmm0, 0x130(%rbp)
14098ee6f:     	movups	%xmm0, 0x148(%rbp)
14098ee76:     	movups	%xmm0, 0x160(%rbp)
14098ee7d:     	movups	%xmm0, 0x178(%rbp)
14098ee84:     	movl	$0x0, 0x188(%rbp)
14098ee8e:     	movq	$0x0, 0x18(%rbp)
14098ee96:     	movq	$0x0, 0x60(%rbp)
14098ee9e:     	movq	$0x8, 0x68(%rbp)
14098eea6:     	movq	$0x0, 0x70(%rbp)
14098eeae:     	movq	$0x10, 0x78(%rbp)
14098eeb6:     	movq	$0x10, 0x90(%rbp)
14098eec1:     	movq	$0x10, 0xa8(%rbp)
14098eecc:     	movq	$0x10, 0xc0(%rbp)
14098eed7:     	movq	$0x10, 0xd8(%rbp)
14098eee2:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14098eeec:     	movq	%rax, 0xf8(%rbp)
14098eef3:     	leaq	0x212ada6(%rip), %rax   # 0x142ab9ca0
14098eefa:     	movq	%rax, 0x100(%rbp)
14098ef01:     	movq	$0x39, 0x108(%rbp)
14098ef0c:     	movq	$0x10, 0x110(%rbp)
14098ef17:     	movq	$0x10, 0x128(%rbp)
14098ef22:     	movq	$0x10, 0x140(%rbp)
14098ef2d:     	movq	$0x10, 0x158(%rbp)
14098ef38:     	movq	$0x10, 0x170(%rbp)
14098ef43:     	movl	$0x0, 0x190(%rbp)
14098ef4d:     	movw	$0x1, 0x194(%rbp)
14098ef56:     	movl	$0x0, 0x198(%rbp)
14098ef60:     	movzbl	0x2ca5ca2(%rip), %eax   # 0x143634c09
14098ef67:     	movl	$0x188, %ecx            # imm = 0x188
14098ef6c:     	movl	$0x8, %edx
14098ef71:     	callq	0x140613c10 <.text+0x612c10>
14098ef76:     	testq	%rax, %rax
14098ef79:     	je	0x14098efc0 <.text+0x98dfc0>
14098ef7b:     	movq	%rax, %rdi
14098ef7e:     	leaq	0x18(%rbp), %rdx
14098ef82:     	movl	$0x188, %r8d            # imm = 0x188
14098ef88:     	movq	%rax, %rcx
14098ef8b:     	callq	0x142923c00 <.text+0x2922c00>
14098ef90:     	leaq	0x212a9b1(%rip), %r8    # 0x142ab9948
14098ef97:     	leaq	-0x58(%rbp), %rbx
14098ef9b:     	movq	%rbx, %rcx
14098ef9e:     	movq	%rdi, %rdx
14098efa1:     	callq	0x140ce4b10 <.text+0xce3b10>
14098efa6:     	movq	%rsi, %rcx
14098efa9:     	movq	%rbx, %rdx
14098efac:     	callq	0x14144fae0 <.text+0x144eae0>
14098efb1:     	movq	%rsi, %rax
14098efb4:     	addq	$0x228, %rsp            # imm = 0x228
14098efbb:     	popq	%rbx
14098efbc:     	popq	%rdi
14098efbd:     	popq	%rsi
14098efbe:     	popq	%rbp
14098efbf:     	retq
14098efc0:     	movl	$0x8, %ecx
14098efc5:     	movl	$0x188, %edx            # imm = 0x188
14098efca:     	callq	0x1428d9000 <.text+0x28d8000>
14098efcf:     	ud2
