
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405047f0:     	pushq	%rbp
1405047f1:     	pushq	%rsi
1405047f2:     	pushq	%rdi
1405047f3:     	subq	$0xc0, %rsp
1405047fa:     	leaq	0x80(%rsp), %rbp
140504802:     	movaps	%xmm6, 0x30(%rbp)
140504806:     	movq	$-0x2, 0x28(%rbp)
14050480e:     	movq	%rdx, %rdi
140504811:     	movq	%rcx, %rsi
140504814:     	movq	0x312787d(%rip), %rax   # 0x14362c098
14050481b:     	cmpb	$0x0, (%rax)
14050481e:     	je	0x1405048b3 <.text+0x5038b3>
140504824:     	movq	(%rsi), %rax
140504827:     	movq	0x8(%rsi), %rcx
14050482b:     	movq	%rax, -0x58(%rbp)
14050482f:     	leaq	-0x3fa9a6(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140504836:     	movq	%rax, -0x50(%rbp)
14050483a:     	movq	%rcx, -0x48(%rbp)
14050483e:     	movq	%rax, -0x40(%rbp)
140504842:     	leaq	0x24e95af(%rip), %rax   # 0x1429eddf8
140504849:     	movq	%rax, -0x28(%rbp)
14050484d:     	movq	$0x2, -0x20(%rbp)
140504855:     	leaq	-0x58(%rbp), %rax
140504859:     	movq	%rax, -0x18(%rbp)
14050485d:     	movq	$0x2, -0x10(%rbp)
140504865:     	movq	$0x0, -0x8(%rbp)
14050486d:     	leaq	0x8(%rbp), %rcx
140504871:     	leaq	-0x28(%rbp), %rdx
140504875:     	callq	0x140d79f90 <.text+0xd78f90>
14050487a:     	movq	0x10(%rbp), %r8
14050487e:     	movq	0x18(%rbp), %r9
140504882:     	leaq	-0x28(%rbp), %rcx
140504886:     	movq	%rdi, %rdx
140504889:     	movq	%r8, 0x20(%rbp)
14050488d:     	callq	0x14068bf00 <.text+0x68af00>
140504892:     	movq	0x10(%rsi), %rax
140504896:     	movss	(%rax), %xmm6
14050489a:     	leaq	-0x28(%rbp), %rcx
14050489e:     	callq	0x140681d30 <.text+0x680d30>
1405048a3:     	movq	%rax, %rcx
1405048a6:     	movaps	%xmm6, %xmm1
1405048a9:     	callq	0x140106350 <.text+0x105350>
1405048ae:     	jmp	0x140504949 <.text+0x503949>
1405048b3:     	movq	0x18(%rsi), %rax
1405048b7:     	movq	(%rsi), %rcx
1405048ba:     	movq	0x8(%rsi), %rdx
1405048be:     	movq	%rax, -0x58(%rbp)
1405048c2:     	leaq	-0x3faa39(%rip), %rax   # 0x140109e90 <.text+0x108e90>
1405048c9:     	movq	%rax, -0x50(%rbp)
1405048cd:     	movq	%rcx, -0x48(%rbp)
1405048d1:     	movq	%rax, -0x40(%rbp)
1405048d5:     	movq	%rdx, -0x38(%rbp)
1405048d9:     	movq	%rax, -0x30(%rbp)
1405048dd:     	leaq	0x24e94dc(%rip), %rax   # 0x1429eddc0
1405048e4:     	movq	%rax, -0x28(%rbp)
1405048e8:     	movq	$0x3, -0x20(%rbp)
1405048f0:     	leaq	-0x58(%rbp), %rax
1405048f4:     	movq	%rax, -0x18(%rbp)
1405048f8:     	movq	$0x3, -0x10(%rbp)
140504900:     	movq	$0x0, -0x8(%rbp)
140504908:     	leaq	0x8(%rbp), %rcx
14050490c:     	leaq	-0x28(%rbp), %rdx
140504910:     	callq	0x140d79f90 <.text+0xd78f90>
140504915:     	movq	0x10(%rbp), %r8
140504919:     	movq	0x18(%rbp), %r9
14050491d:     	leaq	-0x28(%rbp), %rcx
140504921:     	movq	%rdi, %rdx
140504924:     	movq	%r8, 0x20(%rbp)
140504928:     	callq	0x14068bf00 <.text+0x68af00>
14050492d:     	movq	0x10(%rsi), %rax
140504931:     	movss	(%rax), %xmm6
140504935:     	leaq	-0x28(%rbp), %rcx
140504939:     	callq	0x140681d30 <.text+0x680d30>
14050493e:     	movq	%rax, %rcx
140504941:     	movaps	%xmm6, %xmm1
140504944:     	callq	0x140106350 <.text+0x105350>
140504949:     	movq	0x8(%rbp), %rdx
14050494d:     	testq	%rdx, %rdx
140504950:     	je	0x140504961 <.text+0x503961>
140504952:     	movl	$0x1, %r8d
140504958:     	movq	0x20(%rbp), %rcx
14050495c:     	callq	0x140613c20 <.text+0x612c20>
140504961:     	movaps	0x30(%rbp), %xmm6
140504965:     	addq	$0xc0, %rsp
14050496c:     	popq	%rdi
14050496d:     	popq	%rsi
14050496e:     	popq	%rbp
14050496f:     	retq
140504970:     	movq	%rdx, 0x10(%rsp)
140504975:     	pushq	%rbp
140504976:     	pushq	%rsi
140504977:     	pushq	%rdi
140504978:     	subq	$0x30, %rsp
14050497c:     	leaq	0x80(%rdx), %rbp
140504983:     	movaps	%xmm6, 0x20(%rsp)
140504988:     	movq	0x8(%rbp), %rdx
14050498c:     	testq	%rdx, %rdx
14050498f:     	je	0x1405049a0 <.text+0x5039a0>
140504991:     	movl	$0x1, %r8d
140504997:     	movq	0x20(%rbp), %rcx
14050499b:     	callq	0x140613c20 <.text+0x612c20>
1405049a0:     	movaps	0x20(%rsp), %xmm6
1405049a5:     	addq	$0x30, %rsp
1405049a9:     	popq	%rdi
1405049aa:     	popq	%rsi
1405049ab:     	popq	%rbp
1405049ac:     	retq
1405049ad:     	nopl	(%rax)
1405049b0:     	movq	%rdx, 0x10(%rsp)
1405049b5:     	pushq	%rbp
1405049b6:     	pushq	%rsi
1405049b7:     	pushq	%rdi
1405049b8:     	subq	$0x30, %rsp
1405049bc:     	leaq	0x80(%rdx), %rbp
1405049c3:     	movaps	%xmm6, 0x20(%rsp)
1405049c8:     	movq	0x8(%rbp), %rdx
1405049cc:     	testq	%rdx, %rdx
1405049cf:     	je	0x1405049e0 <.text+0x5039e0>
1405049d1:     	movl	$0x1, %r8d
1405049d7:     	movq	0x20(%rbp), %rcx
1405049db:     	callq	0x140613c20 <.text+0x612c20>
1405049e0:     	movaps	0x20(%rsp), %xmm6
1405049e5:     	addq	$0x30, %rsp
1405049e9:     	popq	%rdi
1405049ea:     	popq	%rsi
1405049eb:     	popq	%rbp
1405049ec:     	retq
