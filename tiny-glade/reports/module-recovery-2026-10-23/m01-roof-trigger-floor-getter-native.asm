
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408e4560:     	subq	$0x28, %rsp
1408e4564:     	movq	0x10(%rdx), %rax
1408e4568:     	testq	%rax, %rax
1408e456b:     	je	0x1408e458d <.text+0x8e358d>
1408e456d:     	movq	0x8(%rdx), %rdx
1408e4571:     	leaq	(%rax,%rax,8), %rax
1408e4575:     	leaq	(%rdx,%rax,4), %rax
1408e4579:     	movq	%rdx, (%rcx)
1408e457c:     	movq	%rax, 0x8(%rcx)
1408e4580:     	movss	%xmm2, 0x10(%rcx)
1408e4585:     	movq	%rcx, %rax
1408e4588:     	addq	$0x28, %rsp
1408e458c:     	retq
1408e458d:     	leaq	0x21c13c4(%rip), %rcx   # 0x142aa5958
1408e4594:     	leaq	0x21c142d(%rip), %r8    # 0x142aa59c8
1408e459b:     	movl	$0x2c, %edx
1408e45a0:     	callq	0x1428d9430 <.text+0x28d8430>
1408e45a5:     	int3
