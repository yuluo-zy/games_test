
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14054e960:     	pushq	%rbp
14054e961:     	pushq	%rsi
14054e962:     	pushq	%rdi
14054e963:     	pushq	%rbx
14054e964:     	subq	$0x528, %rsp            # imm = 0x528
14054e96b:     	leaq	0x80(%rsp), %rbp
14054e973:     	movq	$-0x2, 0x4a0(%rbp)
14054e97e:     	movq	%rcx, %rsi
14054e981:     	xorps	%xmm0, %xmm0
14054e984:     	movups	%xmm0, 0x38(%rbp)
14054e988:     	movups	%xmm0, 0x50(%rbp)
14054e98c:     	movups	%xmm0, 0x68(%rbp)
14054e990:     	movups	%xmm0, 0x80(%rbp)
14054e997:     	movups	%xmm0, 0x98(%rbp)
14054e99e:     	movl	$0x0, 0xa8(%rbp)
14054e9a8:     	movups	%xmm0, 0xd0(%rbp)
14054e9af:     	movups	%xmm0, 0xe8(%rbp)
14054e9b6:     	movups	%xmm0, 0x100(%rbp)
14054e9bd:     	movups	%xmm0, 0x118(%rbp)
14054e9c4:     	movups	%xmm0, 0x130(%rbp)
14054e9cb:     	movl	$0x0, 0x140(%rbp)
14054e9d5:     	movq	$0x0, 0x18(%rbp)
14054e9dd:     	movq	$0x8, 0x20(%rbp)
14054e9e5:     	movq	$0x0, 0x28(%rbp)
14054e9ed:     	movq	$0x10, 0x30(%rbp)
14054e9f5:     	movq	$0x10, 0x48(%rbp)
14054e9fd:     	movq	$0x10, 0x60(%rbp)
14054ea05:     	movq	$0x10, 0x78(%rbp)
14054ea0d:     	movq	$0x10, 0x90(%rbp)
14054ea18:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14054ea22:     	movq	%rax, 0xb0(%rbp)
14054ea29:     	leaq	0x24aa9b6(%rip), %rcx   # 0x1429f93e6
14054ea30:     	movq	%rcx, 0xb8(%rbp)
14054ea37:     	movq	$0x47, 0xc0(%rbp)
14054ea42:     	movq	$0x10, 0xc8(%rbp)
14054ea4d:     	movq	$0x10, 0xe0(%rbp)
14054ea58:     	movq	$0x10, 0xf8(%rbp)
14054ea63:     	movq	$0x10, 0x110(%rbp)
14054ea6e:     	movq	$0x10, 0x128(%rbp)
14054ea79:     	movl	$0x0, 0x148(%rbp)
14054ea83:     	movw	$0x1, 0x14c(%rbp)
14054ea8c:     	movq	%rax, 0x240(%rbp)
14054ea93:     	movl	$0x0, 0x498(%rbp)
14054ea9d:     	movzbl	0x30e6165(%rip), %eax   # 0x143634c09
14054eaa4:     	movl	$0x488, %ecx            # imm = 0x488
14054eaa9:     	movl	$0x8, %edx
14054eaae:     	callq	0x140613c10 <.text+0x612c10>
14054eab3:     	testq	%rax, %rax
14054eab6:     	je	0x14054eafd <.text+0x54dafd>
14054eab8:     	movq	%rax, %rdi
14054eabb:     	leaq	0x18(%rbp), %rdx
14054eabf:     	movl	$0x488, %r8d            # imm = 0x488
14054eac5:     	movq	%rax, %rcx
14054eac8:     	callq	0x142923c00 <.text+0x2922c00>
14054eacd:     	leaq	0x24aa7ec(%rip), %r8    # 0x1429f92c0
14054ead4:     	leaq	-0x58(%rbp), %rbx
14054ead8:     	movq	%rbx, %rcx
14054eadb:     	movq	%rdi, %rdx
14054eade:     	callq	0x140ce4b10 <.text+0xce3b10>
14054eae3:     	movq	%rsi, %rcx
14054eae6:     	movq	%rbx, %rdx
14054eae9:     	callq	0x1400d2a70 <.text+0xd1a70>
14054eaee:     	movq	%rsi, %rax
14054eaf1:     	addq	$0x528, %rsp            # imm = 0x528
14054eaf8:     	popq	%rbx
14054eaf9:     	popq	%rdi
14054eafa:     	popq	%rsi
14054eafb:     	popq	%rbp
14054eafc:     	retq
14054eafd:     	movl	$0x8, %ecx
14054eb02:     	movl	$0x488, %edx            # imm = 0x488
14054eb07:     	callq	0x1428d9000 <.text+0x28d8000>
14054eb0c:     	ud2
