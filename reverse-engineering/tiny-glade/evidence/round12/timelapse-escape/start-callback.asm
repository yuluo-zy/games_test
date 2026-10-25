
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140af9e00:     	pushq	%rsi
140af9e01:     	pushq	%rdi
140af9e02:     	pushq	%rbx
140af9e03:     	subq	$0x90, %rsp
140af9e0a:     	testb	%cl, %cl
140af9e0c:     	je	0x140af9e5f <.text+0xaf8e5f>
140af9e0e:     	movl	0x1c(%rdx), %eax
140af9e11:     	movq	0x10(%rdx), %rcx
140af9e15:     	movl	%eax, (%rcx)
140af9e17:     	movq	(%rdx), %rsi
140af9e1a:     	movq	0x30(%rsi), %rdi
140af9e1e:     	movq	0x40(%rsi), %rbx
140af9e22:     	cmpq	0x20(%rsi), %rdi
140af9e26:     	jne	0x140af9e38 <.text+0xaf8e38>
140af9e28:     	leaq	0x20(%rsi), %rcx
140af9e2c:     	leaq	0x2140315(%rip), %rdx   # 0x142c3a148
140af9e33:     	callq	0x140562c30 <.text+0x561c30>
140af9e38:     	movq	0x28(%rsi), %rax
140af9e3c:     	movq	%rdi, %rcx
140af9e3f:     	shlq	$0x5, %rcx
140af9e43:     	movl	$0x4, (%rax,%rcx)
140af9e4a:     	movq	%rbx, 0x18(%rax,%rcx)
140af9e4f:     	incq	%rdi
140af9e52:     	movq	%rdi, 0x30(%rsi)
140af9e56:     	incq	0x40(%rsi)
140af9e5a:     	jmp	0x140af9f24 <.text+0xaf8f24>
140af9e5f:     	movq	(%r8), %rax
140af9e62:     	movq	0x8(%r8), %rcx
140af9e66:     	movq	0x10(%r8), %rdx
140af9e6a:     	movq	0x18(%r8), %r9
140af9e6e:     	movq	0x20(%r8), %rdi
140af9e72:     	movq	0x28(%r8), %rbx
140af9e76:     	movq	0x30(%r8), %rsi
140af9e7a:     	movq	(%rcx), %rcx
140af9e7d:     	movups	(%rcx), %xmm0
140af9e80:     	movaps	%xmm0, 0x20(%rsp)
140af9e85:     	movl	0x1c(%rax), %ecx
140af9e88:     	movq	0x10(%rax), %r8
140af9e8c:     	movl	%ecx, (%r8)
140af9e8f:     	movq	(%rax), %rax
140af9e92:     	movaps	0x20(%rsp), %xmm0
140af9e97:     	movups	%xmm0, (%rax)
140af9e9a:     	movl	0x1c(%rdx), %eax
140af9e9d:     	movq	0x10(%rdx), %rcx
140af9ea1:     	movl	%eax, (%rcx)
140af9ea3:     	movq	(%rdx), %rcx
140af9ea6:     	movq	(%r9), %rax
140af9ea9:     	movups	0x60(%rax), %xmm0
140af9ead:     	movups	%xmm0, 0x28(%rsp)
140af9eb2:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140af9ebc:     	movq	%rax, 0x20(%rsp)
140af9ec1:     	movq	$0x0, 0x38(%rsp)
140af9eca:     	movb	$0x1, 0x40(%rsp)
140af9ecf:     	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
140af9ed9:     	movq	%rax, 0x58(%rsp)
140af9ede:     	leaq	0x2160f53(%rip), %r8    # 0x142c5ae38
140af9ee5:     	leaq	0x20(%rsp), %rdx
140af9eea:     	callq	0x140b59be0 <.text+0xb58be0>
140af9eef:     	movq	(%rbx), %rax
140af9ef2:     	movss	(%rax), %xmm0
140af9ef6:     	mulss	0x1e2ba8a(%rip), %xmm0  # 0x142925988
140af9efe:     	movl	0x1c(%rdi), %eax
140af9f01:     	movq	0x10(%rdi), %rcx
140af9f05:     	movl	%eax, (%rcx)
140af9f07:     	movq	(%rdi), %rax
140af9f0a:     	movl	$0x1, (%rax)
140af9f10:     	movss	%xmm0, 0x4(%rax)
140af9f15:     	movl	0x1c(%rsi), %eax
140af9f18:     	movq	0x10(%rsi), %rcx
140af9f1c:     	movl	%eax, (%rcx)
140af9f1e:     	movq	(%rsi), %rax
140af9f21:     	movb	$0x0, (%rax)
140af9f24:     	addq	$0x90, %rsp
140af9f2b:     	popq	%rbx
140af9f2c:     	popq	%rdi
140af9f2d:     	popq	%rsi
140af9f2e:     	retq
140af9f2f:     	int3
140af9f30:     	pushq	%rsi
140af9f31:     	pushq	%rdi
140af9f32:     	pushq	%rbx
140af9f33:     	subq	$0x30, %rsp
140af9f37:     	testb	%cl, %cl
140af9f39:     	je	0x140af9f8c <.text+0xaf8f8c>
140af9f3b:     	movl	0x1c(%rdx), %eax
140af9f3e:     	movq	0x10(%rdx), %rcx
140af9f42:     	movl	%eax, (%rcx)
140af9f44:     	movq	(%rdx), %rsi
140af9f47:     	movq	0x30(%rsi), %rdi
140af9f4b:     	movq	0x40(%rsi), %rbx
140af9f4f:     	cmpq	0x20(%rsi), %rdi
140af9f53:     	jne	0x140af9f65 <.text+0xaf8f65>
140af9f55:     	leaq	0x20(%rsi), %rcx
140af9f59:     	leaq	0x21401e8(%rip), %rdx   # 0x142c3a148
140af9f60:     	callq	0x140562c30 <.text+0x561c30>
140af9f65:     	movq	0x28(%rsi), %rax
140af9f69:     	movq	%rdi, %rcx
140af9f6c:     	shlq	$0x5, %rcx
140af9f70:     	movl	$0x4, (%rax,%rcx)
140af9f77:     	movq	%rbx, 0x18(%rax,%rcx)
140af9f7c:     	incq	%rdi
140af9f7f:     	movq	%rdi, 0x30(%rsi)
140af9f83:     	incq	0x40(%rsi)
140af9f87:     	jmp	0x140afa050 <.text+0xaf9050>
140af9f8c:     	movl	0x1c(%r8), %eax
140af9f90:     	movq	0x10(%r8), %rcx
140af9f94:     	movl	%eax, (%rcx)
140af9f96:     	movq	(%r8), %rcx
140af9f99:     	cmpq	$0x5, 0x10(%rcx)
140af9f9e:     	jb	0x140afa050 <.text+0xaf9050>
140af9fa4:     	movq	0x8(%rcx), %r8
140af9fa8:     	movq	0x90(%r8), %r10
140af9faf:     	testq	%r10, %r10
140af9fb2:     	je	0x140afa050 <.text+0xaf9050>
140af9fb8:     	cmpb	$0x4, 0x18(%rcx)
140af9fbc:     	jne	0x140af9fdb <.text+0xaf8fdb>
140af9fbe:     	movq	0x98(%r8), %rax
