
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b3c930:     	pushq	%r15
140b3c932:     	pushq	%r14
140b3c934:     	pushq	%r13
140b3c936:     	pushq	%r12
140b3c938:     	pushq	%rsi
140b3c939:     	pushq	%rdi
140b3c93a:     	pushq	%rbx
140b3c93b:     	subq	$0x80, %rsp
140b3c942:     	movl	$0x1, %ebx
140b3c947:     	lock
140b3c948:     	xaddl	%ebx, 0x5b0(%rdx)
140b3c94f:     	testb	$0x1, (%rcx)
140b3c952:     	je	0x140b3cb05 <.text+0xb3bb05>
140b3c958:     	movq	%rcx, %rsi
140b3c95b:     	movq	0x8(%rcx), %rax
140b3c95f:     	cmpq	%rax, 0x218(%rdx)
140b3c966:     	jbe	0x140b3ca62 <.text+0xb3ba62>
140b3c96c:     	movq	0x210(%rdx), %rcx
140b3c973:     	movq	(%rcx,%rax,8), %rcx
140b3c977:     	testq	%rcx, %rcx
140b3c97a:     	je	0x140b3ca62 <.text+0xb3ba62>
140b3c980:     	movq	0x1e0(%rdx), %rax
140b3c987:     	notq	%rcx
140b3c98a:     	leaq	(%rcx,%rcx,2), %rcx
140b3c98e:     	shlq	$0x5, %rcx
140b3c992:     	cmpq	$0x0, 0x20(%rax,%rcx)
140b3c998:     	je	0x140b3ca62 <.text+0xb3ba62>
140b3c99e:     	addq	%rcx, %rax
140b3c9a1:     	movq	0x10(%rax), %r14
140b3c9a5:     	movl	0x5b0(%rdx), %ecx
140b3c9ab:     	movq	0x10(%rsi), %rcx
140b3c9af:     	cmpq	%rcx, 0x218(%rdx)
140b3c9b6:     	jbe	0x140b3ca80 <.text+0xb3ba80>
140b3c9bc:     	movq	0x210(%rdx), %r8
140b3c9c3:     	movq	(%r8,%rcx,8), %rcx
140b3c9c7:     	testq	%rcx, %rcx
140b3c9ca:     	je	0x140b3ca80 <.text+0xb3ba80>
140b3c9d0:     	movq	0x1e0(%rdx), %r15
140b3c9d7:     	notq	%rcx
140b3c9da:     	leaq	(%rcx,%rcx,2), %rcx
140b3c9de:     	shlq	$0x5, %rcx
140b3c9e2:     	cmpq	$0x0, 0x20(%r15,%rcx)
140b3c9e8:     	je	0x140b3ca80 <.text+0xb3ba80>
140b3c9ee:     	addq	%rcx, %r15
140b3c9f1:     	movq	0x10(%r15), %rdi
140b3c9f5:     	movl	0x5b0(%rdx), %ecx
140b3c9fb:     	movl	%ebx, 0x5c(%rax)
140b3c9fe:     	movq	0x30(%r14), %r12
140b3ca02:     	movq	0x40(%r14), %r13
140b3ca06:     	cmpq	0x20(%r14), %r12
140b3ca0a:     	jne	0x140b3ca1c <.text+0xb3ba1c>
140b3ca0c:     	leaq	0x20(%r14), %rcx
140b3ca10:     	leaq	0x20fd731(%rip), %rdx   # 0x142c3a148
140b3ca17:     	callq	0x140562630 <.text+0x561630>
140b3ca1c:     	movq	0x28(%r14), %rax
140b3ca20:     	movq	%r12, %rcx
140b3ca23:     	shlq	$0x4, %rcx
140b3ca27:     	movq	%r13, (%rax,%rcx)
140b3ca2b:     	movb	$0x0, 0x8(%rax,%rcx)
140b3ca30:     	incq	%r12
140b3ca33:     	movq	%r12, 0x30(%r14)
140b3ca37:     	incq	0x40(%r14)
140b3ca3b:     	movl	%ebx, 0x5c(%r15)
140b3ca3f:     	movq	%rdi, %rcx
140b3ca42:     	callq	0x140a43840 <.text+0xa42840>
140b3ca47:     	movl	%ebx, 0x150(%rsi)
140b3ca4d:     	xorl	%eax, %eax
140b3ca4f:     	addq	$0x80, %rsp
140b3ca56:     	popq	%rbx
140b3ca57:     	popq	%rdi
140b3ca58:     	popq	%rsi
140b3ca59:     	popq	%r12
140b3ca5b:     	popq	%r13
140b3ca5d:     	popq	%r14
140b3ca5f:     	popq	%r15
140b3ca61:     	retq
140b3ca62:     	addq	$0xb8, %rsi
140b3ca69:     	leaq	0x1fb1c5c(%rip), %rax   # 0x142aee6cc
140b3ca70:     	movq	%rax, 0x20(%rsp)
140b3ca75:     	movq	$0x55, 0x28(%rsp)
140b3ca7e:     	jmp	0x140b3ca9c <.text+0xb3ba9c>
140b3ca80:     	addq	$0xb8, %rsi
140b3ca87:     	leaq	0x1fb1ce7(%rip), %rax   # 0x142aee775
140b3ca8e:     	movq	%rax, 0x20(%rsp)
140b3ca93:     	movq	$0x35, 0x28(%rsp)
140b3ca9c:     	movq	%rsi, 0x30(%rsp)
140b3caa1:     	leaq	-0xb00fd8(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140b3caa8:     	movq	%rax, 0x38(%rsp)
140b3caad:     	leaq	0x20(%rsp), %rax
140b3cab2:     	movq	%rax, 0x40(%rsp)
140b3cab7:     	leaq	-0xa32c2e(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140b3cabe:     	movq	%rax, 0x48(%rsp)
140b3cac3:     	leaq	0x1fb1a9e(%rip), %rax   # 0x142aee568
140b3caca:     	movq	%rax, 0x50(%rsp)
140b3cacf:     	movq	$0x2, 0x58(%rsp)
140b3cad8:     	movq	$0x0, 0x70(%rsp)
140b3cae1:     	leaq	0x30(%rsp), %rax
140b3cae6:     	movq	%rax, 0x60(%rsp)
140b3caeb:     	movq	$0x2, 0x68(%rsp)
140b3caf4:     	leaq	0x1fb1c2d(%rip), %rdx   # 0x142aee728
140b3cafb:     	leaq	0x50(%rsp), %rcx
140b3cb00:     	callq	0x1428d9390 <.text+0x28d8390>
140b3cb05:     	leaq	0x1fb1d7c(%rip), %rcx   # 0x142aee888
140b3cb0c:     	leaq	0x1fb1dd5(%rip), %r8    # 0x142aee8e8
140b3cb13:     	movl	$0x59, %edx
