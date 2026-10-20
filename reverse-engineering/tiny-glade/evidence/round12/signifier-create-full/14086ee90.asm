
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14086ee90:     	pushq	%rsi
14086ee91:     	pushq	%rdi
14086ee92:     	subq	$0xa8, %rsp
14086ee99:     	movl	$0x1, %edi
14086ee9e:     	lock
14086ee9f:     	xaddl	%edi, 0x5b0(%rdx)
14086eea6:     	xorl	%eax, %eax
14086eea8:     	cmpq	0x230(%rcx), %rax
14086eeaf:     	jo	0x14086f020 <.text+0x86e020>
14086eeb5:     	movq	%rcx, %rsi
14086eeb8:     	movq	0x268(%rcx), %rax
14086eebf:     	cmpq	%rax, 0x218(%rdx)
14086eec6:     	jbe	0x14086ef89 <.text+0x86df89>
14086eecc:     	movq	0x210(%rdx), %rcx
14086eed3:     	movq	(%rcx,%rax,8), %rcx
14086eed7:     	testq	%rcx, %rcx
14086eeda:     	je	0x14086ef89 <.text+0x86df89>
14086eee0:     	movq	0x1e0(%rdx), %rax
14086eee7:     	notq	%rcx
14086eeea:     	leaq	(%rcx,%rcx,2), %rcx
14086eeee:     	shlq	$0x5, %rcx
14086eef2:     	cmpq	$0x0, 0x20(%rax,%rcx)
14086eef8:     	je	0x14086ef89 <.text+0x86df89>
14086eefe:     	addq	%rcx, %rax
14086ef01:     	leaq	0x140(%rsi), %rcx
14086ef08:     	leaq	0x270(%rsi), %r9
14086ef0f:     	movq	0x10(%rax), %r8
14086ef13:     	leaq	0x58(%rax), %r10
14086ef17:     	addq	$0x5c, %rax
14086ef1b:     	movl	0x130(%rsi), %r11d
14086ef22:     	movq	%r8, 0x20(%rsp)
14086ef27:     	movq	%r10, 0x28(%rsp)
14086ef2c:     	movq	%rax, 0x30(%rsp)
14086ef31:     	movl	%r11d, 0x38(%rsp)
14086ef36:     	movl	%edi, 0x3c(%rsp)
14086ef3a:     	leaq	0x40(%rsp), %rax
14086ef3f:     	movq	%rcx, 0x40(%rsp)
14086ef44:     	movq	%rdx, 0x48(%rsp)
14086ef49:     	movl	%r11d, 0x50(%rsp)
14086ef4e:     	movl	%edi, 0x54(%rsp)
14086ef52:     	leaq	0x58(%rsp), %r8
14086ef57:     	movq	%rdx, 0x58(%rsp)
14086ef5c:     	movq	$0x0, 0x60(%rsp)
14086ef65:     	movq	%r9, 0x68(%rsp)
14086ef6a:     	leaq	0x20(%rsp), %rcx
14086ef6f:     	movq	%rax, %rdx
14086ef72:     	callq	0x140929160 <.text+0x928160>
14086ef77:     	movl	%edi, 0x130(%rsi)
14086ef7d:     	xorl	%eax, %eax
14086ef7f:     	addq	$0xa8, %rsp
14086ef86:     	popq	%rdi
14086ef87:     	popq	%rsi
14086ef88:     	retq
14086ef89:     	addq	$0x98, %rsi
14086ef90:     	leaq	0x2228218(%rip), %rax   # 0x142a971af
14086ef97:     	movq	%rax, 0x78(%rsp)
14086ef9c:     	movq	$0x37, 0x80(%rsp)
14086efa8:     	movq	%rsi, 0x88(%rsp)
14086efb0:     	leaq	-0x8334e7(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
14086efb7:     	movq	%rax, 0x90(%rsp)
14086efbf:     	leaq	0x78(%rsp), %rax
14086efc4:     	movq	%rax, 0x98(%rsp)
14086efcc:     	leaq	-0x765143(%rip), %rax   # 0x140109e90 <.text+0x108e90>
14086efd3:     	movq	%rax, 0xa0(%rsp)
14086efdb:     	leaq	0x2227fb6(%rip), %rax   # 0x142a96f98
14086efe2:     	movq	%rax, 0x20(%rsp)
14086efe7:     	movq	$0x2, 0x28(%rsp)
14086eff0:     	movq	$0x0, 0x40(%rsp)
14086eff9:     	leaq	0x88(%rsp), %rax
14086f001:     	movq	%rax, 0x30(%rsp)
14086f006:     	movq	$0x2, 0x38(%rsp)
14086f00f:     	leaq	0x222800a(%rip), %rdx   # 0x142a97020
14086f016:     	leaq	0x20(%rsp), %rcx
14086f01b:     	callq	0x1428d9390 <.text+0x28d8390>
14086f020:     	leaq	0x2228889(%rip), %rcx   # 0x142a978b0
14086f027:     	leaq	0x22288e2(%rip), %r8    # 0x142a97910
14086f02e:     	movl	$0x59, %edx
14086f033:     	callq	0x1428d9330 <.text+0x28d8330>
14086f038:     	int3
