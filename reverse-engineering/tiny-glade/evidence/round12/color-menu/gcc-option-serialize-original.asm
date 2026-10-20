
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14158b780:     	pushq	%r15
14158b782:     	pushq	%r14
14158b784:     	pushq	%r12
14158b786:     	pushq	%rsi
14158b787:     	pushq	%rdi
14158b788:     	pushq	%rbx
14158b789:     	subq	$0x28, %rsp
14158b78d:     	movq	%r9, %rsi
14158b790:     	movq	%r8, %rbx
14158b793:     	movq	(%rcx), %rdi
14158b796:     	cmpb	$0x1, 0x8(%rcx)
14158b79a:     	je	0x14158b7bc <.text+0x158a7bc>
14158b79c:     	movq	(%rdi), %r14
14158b79f:     	movq	0x10(%r14), %rax
14158b7a3:     	cmpq	%rax, (%r14)
14158b7a6:     	je	0x14158b860 <.text+0x158a860>
14158b7ac:     	movq	0x8(%r14), %r8
14158b7b0:     	movb	$0x2c, (%r8,%rax)
14158b7b5:     	incq	%rax
14158b7b8:     	movq	%rax, 0x10(%r14)
14158b7bc:     	movb	$0x2, 0x8(%rcx)
14158b7c0:     	movq	%rdi, %rcx
14158b7c3:     	movq	%rdx, %r8
14158b7c6:     	movq	%rbx, %r9
14158b7c9:     	callq	0x141580d20 <.text+0x157fd20>
14158b7ce:     	movq	(%rdi), %rbx
14158b7d1:     	movq	0x10(%rbx), %rdx
14158b7d5:     	cmpq	%rdx, (%rbx)
14158b7d8:     	je	0x14158b83a <.text+0x158a83a>
14158b7da:     	movq	0x8(%rbx), %rax
14158b7de:     	movb	$0x3a, (%rax,%rdx)
14158b7e2:     	incq	%rdx
14158b7e5:     	movq	%rdx, 0x10(%rbx)
14158b7e9:     	testb	$0x1, (%rsi)
14158b7ec:     	je	0x14158b804 <.text+0x158a804>
14158b7ee:     	addq	$0x4, %rsi
14158b7f2:     	movq	%rsi, %rcx
14158b7f5:     	movq	%rdi, %rdx
14158b7f8:     	callq	0x1408e5bc0 <.text+0x8e4bc0>
14158b7fd:     	testq	%rax, %rax
14158b800:     	jne	0x14158b82c <.text+0x158a82c>
14158b802:     	jmp	0x14158b82a <.text+0x158a82a>
14158b804:     	movq	(%rdi), %rsi
14158b807:     	movq	(%rsi), %rax
14158b80a:     	movq	0x10(%rsi), %rdx
14158b80e:     	subq	%rdx, %rax
14158b811:     	cmpq	$0x3, %rax
14158b815:     	jbe	0x14158b895 <.text+0x158a895>
14158b817:     	movq	0x8(%rsi), %rax
14158b81b:     	movl	$0x6c6c756e, (%rax,%rdx) # imm = 0x6C6C756E
14158b822:     	addq	$0x4, %rdx
14158b826:     	movq	%rdx, 0x10(%rsi)
14158b82a:     	xorl	%eax, %eax
14158b82c:     	addq	$0x28, %rsp
14158b830:     	popq	%rbx
14158b831:     	popq	%rdi
14158b832:     	popq	%rsi
14158b833:     	popq	%r12
14158b835:     	popq	%r14
14158b837:     	popq	%r15
14158b839:     	retq
14158b83a:     	movq	$0x1, 0x20(%rsp)
14158b843:     	movl	$0x1, %r8d
14158b849:     	movl	$0x1, %r9d
14158b84f:     	movq	%rbx, %rcx
14158b852:     	callq	0x1428a9a60 <.text+0x28a8a60>
14158b857:     	movq	0x10(%rbx), %rdx
14158b85b:     	jmp	0x14158b7da <.text+0x158a7da>
14158b860:     	movq	$0x1, 0x20(%rsp)
14158b869:     	movl	$0x1, %r8d
14158b86f:     	movl	$0x1, %r9d
14158b875:     	movq	%rcx, %r15
14158b878:     	movq	%r14, %rcx
14158b87b:     	movq	%rdx, %r12
14158b87e:     	movq	%rax, %rdx
14158b881:     	callq	0x1428a9a60 <.text+0x28a8a60>
14158b886:     	movq	%r15, %rcx
14158b889:     	movq	%r12, %rdx
14158b88c:     	movq	0x10(%r14), %rax
14158b890:     	jmp	0x14158b7ac <.text+0x158a7ac>
14158b895:     	movq	$0x1, 0x20(%rsp)
14158b89e:     	movl	$0x4, %r8d
14158b8a4:     	movl	$0x1, %r9d
14158b8aa:     	movq	%rsi, %rcx
14158b8ad:     	callq	0x1428a9a60 <.text+0x28a8a60>
14158b8b2:     	movq	0x10(%rsi), %rdx
14158b8b6:     	jmp	0x14158b817 <.text+0x158a817>
14158b8bb:     	int3
14158b8bc:     	int3
14158b8bd:     	int3
14158b8be:     	int3
14158b8bf:     	int3
14158b8c0:     	pushq	%r15
14158b8c2:     	pushq	%r14
14158b8c4:     	pushq	%r12
14158b8c6:     	pushq	%rsi
14158b8c7:     	pushq	%rdi
14158b8c8:     	pushq	%rbx
14158b8c9:     	subq	$0x28, %rsp
14158b8cd:     	movq	%r9, %rsi
14158b8d0:     	movq	%r8, %rbx
14158b8d3:     	movq	(%rcx), %rdi
14158b8d6:     	cmpb	$0x1, 0x8(%rcx)
14158b8da:     	je	0x14158b8f8 <.text+0x158a8f8>
14158b8dc:     	movq	(%rdi), %r14
14158b8df:     	movq	0x10(%r14), %rax
14158b8e3:     	cmpq	%rax, (%r14)
14158b8e6:     	je	0x14158b960 <.text+0x158a960>
14158b8e8:     	movq	0x8(%r14), %r8
14158b8ec:     	movb	$0x2c, (%r8,%rax)
14158b8f1:     	incq	%rax
14158b8f4:     	movq	%rax, 0x10(%r14)
14158b8f8:     	movb	$0x2, 0x8(%rcx)
14158b8fc:     	movq	%rdi, %rcx
14158b8ff:     	movq	%rdx, %r8
14158b902:     	movq	%rbx, %r9
14158b905:     	callq	0x141580d20 <.text+0x157fd20>
14158b90a:     	movq	(%rdi), %rbx
14158b90d:     	movq	0x10(%rbx), %rdx
14158b911:     	cmpq	%rdx, (%rbx)
14158b914:     	je	0x14158b93d <.text+0x158a93d>
14158b916:     	movq	0x8(%rbx), %rax
14158b91a:     	movb	$0x3a, (%rax,%rdx)
14158b91e:     	incq	%rdx
14158b921:     	movq	%rdx, 0x10(%rbx)
14158b925:     	movq	%rsi, %rcx
14158b928:     	movq	%rdi, %rdx
14158b92b:     	addq	$0x28, %rsp
14158b92f:     	popq	%rbx
14158b930:     	popq	%rdi
14158b931:     	popq	%rsi
14158b932:     	popq	%r12
14158b934:     	popq	%r14
14158b936:     	popq	%r15
14158b938:     	jmp	0x140a48eb0 <.text+0xa47eb0>
14158b93d:     	movq	$0x1, 0x20(%rsp)
14158b946:     	movl	$0x1, %r8d
14158b94c:     	movl	$0x1, %r9d
14158b952:     	movq	%rbx, %rcx
14158b955:     	callq	0x1428a9a60 <.text+0x28a8a60>
14158b95a:     	movq	0x10(%rbx), %rdx
14158b95e:     	jmp	0x14158b916 <.text+0x158a916>
14158b960:     	movq	$0x1, 0x20(%rsp)
14158b969:     	movl	$0x1, %r8d
14158b96f:     	movl	$0x1, %r9d
14158b975:     	movq	%rcx, %r15
14158b978:     	movq	%r14, %rcx
14158b97b:     	movq	%rdx, %r12
14158b97e:     	movq	%rax, %rdx
14158b981:     	callq	0x1428a9a60 <.text+0x28a8a60>
14158b986:     	movq	%r15, %rcx
14158b989:     	movq	%r12, %rdx
14158b98c:     	movq	0x10(%r14), %rax
14158b990:     	jmp	0x14158b8e8 <.text+0x158a8e8>
14158b995:     	int3
14158b996:     	int3
14158b997:     	int3
14158b998:     	int3
14158b999:     	int3
14158b99a:     	int3
14158b99b:     	int3
14158b99c:     	int3
14158b99d:     	int3
14158b99e:     	int3
14158b99f:     	int3
14158b9a0:     	pushq	%r15
14158b9a2:     	pushq	%r14
14158b9a4:     	pushq	%r12
14158b9a6:     	pushq	%rsi
14158b9a7:     	pushq	%rdi
14158b9a8:     	pushq	%rbx
14158b9a9:     	subq	$0x58, %rsp
14158b9ad:     	movq	%r9, %rdi
14158b9b0:     	movq	%r8, %rbx
14158b9b3:     	movq	(%rcx), %rsi
14158b9b6:     	cmpb	$0x1, 0x8(%rcx)
14158b9ba:     	je	0x14158b9dc <.text+0x158a9dc>
14158b9bc:     	movq	(%rsi), %r14
14158b9bf:     	movq	0x10(%r14), %rax
14158b9c3:     	cmpq	%rax, (%r14)
14158b9c6:     	je	0x14158bafa <.text+0x158aafa>
14158b9cc:     	movq	0x8(%r14), %r8
14158b9d0:     	movb	$0x2c, (%r8,%rax)
14158b9d5:     	incq	%rax
14158b9d8:     	movq	%rax, 0x10(%r14)
14158b9dc:     	movb	$0x2, 0x8(%rcx)
14158b9e0:     	movq	%rsi, %rcx
14158b9e3:     	movq	%rdx, %r8
14158b9e6:     	movq	%rbx, %r9
14158b9e9:     	callq	0x141580d20 <.text+0x157fd20>
14158b9ee:     	movq	(%rsi), %rbx
14158b9f1:     	movq	0x10(%rbx), %rdx
14158b9f5:     	cmpq	%rdx, (%rbx)
14158b9f8:     	je	0x14158bab1 <.text+0x158aab1>
14158b9fe:     	movq	0x8(%rbx), %rax
14158ba02:     	movb	$0x3a, (%rax,%rdx)
14158ba06:     	incq	%rdx
14158ba09:     	movq	%rdx, 0x10(%rbx)
14158ba0d:     	movzbl	(%rdi), %eax
14158ba10:     	movq	(%rsi), %rsi
14158ba13:     	cmpq	$0x64, %rax
14158ba17:     	jb	0x14158ba44 <.text+0x158aa44>
14158ba19:     	movzbl	%al, %ecx
14158ba1c:     	leal	(%rcx,%rcx,4), %edx
14158ba1f:     	leal	(%rcx,%rdx,8), %ecx
14158ba22:     	shrl	$0xc, %ecx
14158ba25:     	imull	$0x64, %ecx, %edx
14158ba28:     	subb	%dl, %al
14158ba2a:     	movzbl	%al, %eax
14158ba2d:     	leaq	0x1542b17(%rip), %rdx   # 0x142ace54b
14158ba34:     	movzwl	(%rdx,%rax,2), %eax
14158ba38:     	movw	%ax, 0x31(%rsp)
14158ba3d:     	xorl	%r14d, %r14d
14158ba40:     	movl	%ecx, %eax
14158ba42:     	jmp	0x14158ba4e <.text+0x158aa4e>
14158ba44:     	movl	$0x2, %r14d
14158ba4a:     	cmpb	$0xa, %al
14158ba4c:     	jae	0x14158ba57 <.text+0x158aa57>
14158ba4e:     	orb	$0x30, %al
14158ba50:     	movb	%al, 0x30(%rsp,%r14)
14158ba55:     	jmp	0x14158ba6d <.text+0x158aa6d>
14158ba57:     	leaq	0x1542aed(%rip), %rcx   # 0x142ace54b
14158ba5e:     	movzwl	(%rcx,%rax,2), %eax
14158ba62:     	movw	%ax, 0x31(%rsp)
14158ba67:     	movl	$0x1, %r14d
14158ba6d:     	movq	%r14, %rdi
14158ba70:     	xorq	$0x3, %rdi
14158ba74:     	movq	(%rsi), %rax
14158ba77:     	movq	0x10(%rsi), %rbx
14158ba7b:     	subq	%rbx, %rax
14158ba7e:     	cmpq	%rdi, %rax
