
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b59be0:     	pushq	%rbp
140b59be1:     	pushq	%r15
140b59be3:     	pushq	%r14
140b59be5:     	pushq	%r13
140b59be7:     	pushq	%r12
140b59be9:     	pushq	%rsi
140b59bea:     	pushq	%rdi
140b59beb:     	pushq	%rbx
140b59bec:     	subq	$0x1b8, %rsp            # imm = 0x1B8
140b59bf3:     	leaq	0x80(%rsp), %rbp
140b59bfb:     	movq	$-0x2, 0x130(%rbp)
140b59c06:     	movups	0x60(%rdx), %xmm0
140b59c0a:     	movaps	%xmm0, 0x90(%rbp)
140b59c11:     	movups	0x50(%rdx), %xmm0
140b59c15:     	movaps	%xmm0, 0x80(%rbp)
140b59c1c:     	movups	0x40(%rdx), %xmm0
140b59c20:     	movaps	%xmm0, 0x70(%rbp)
140b59c24:     	movups	(%rdx), %xmm0
140b59c27:     	movups	0x10(%rdx), %xmm1
140b59c2b:     	movups	0x20(%rdx), %xmm2
140b59c2f:     	movups	0x30(%rdx), %xmm3
140b59c33:     	movaps	%xmm3, 0x60(%rbp)
140b59c37:     	movaps	%xmm2, 0x50(%rbp)
140b59c3b:     	movaps	%xmm1, 0x40(%rbp)
140b59c3f:     	movaps	%xmm0, 0x30(%rbp)
140b59c43:     	movq	$0x0, 0xa0(%rbp)
140b59c4e:     	movq	$0x2, 0xa8(%rbp)
140b59c59:     	movq	0x18(%rcx), %rsi
140b59c5d:     	cmpq	$-0x3, %rsi
140b59c61:     	ja	0x140b5a0e7 <.text+0xb590e7>
140b59c67:     	leaq	0x2(%rsi), %rax
140b59c6b:     	movq	(%rcx), %rbx
140b59c6e:     	cmpq	%rbx, %rax
140b59c71:     	jbe	0x140b59cea <.text+0xb58cea>
140b59c73:     	movq	%rbx, %rax
140b59c76:     	subq	%rsi, %rax
140b59c79:     	cmpq	$0x1, %rax
140b59c7d:     	jbe	0x140b5a0fa <.text+0xb590fa>
140b59c83:     	movq	%rbx, %r14
140b59c86:     	movq	0x10(%rcx), %rdi
140b59c8a:     	cmpq	%rax, %rdi
140b59c8d:     	jbe	0x140b59d0d <.text+0xb58d0d>
140b59c8f:     	movq	%rbx, %r8
140b59c92:     	subq	%rdi, %r8
140b59c95:     	movq	%rsi, %r9
140b59c98:     	subq	%r8, %r9
140b59c9b:     	cmpq	%r9, %r8
140b59c9e:     	setbe	%al
140b59ca1:     	movq	%r14, %rdx
140b59ca4:     	subq	%rbx, %rdx
140b59ca7:     	cmpq	%r9, %rdx
140b59caa:     	setb	%dl
140b59cad:     	orb	%al, %dl
140b59caf:     	cmpb	$0x1, %dl
140b59cb2:     	jne	0x140b59cf0 <.text+0xb58cf0>
140b59cb4:     	movq	%r14, %r15
140b59cb7:     	subq	%r8, %r15
140b59cba:     	movq	0x8(%rcx), %r9
140b59cbe:     	imulq	$0x38, %rdi, %rdx
140b59cc2:     	addq	%r9, %rdx
140b59cc5:     	imulq	$0x38, %r15, %rax
140b59cc9:     	addq	%r9, %rax
140b59ccc:     	imulq	$0x38, %r8, %r8
140b59cd0:     	movq	%rcx, %rdi
140b59cd3:     	movq	%rax, %rcx
140b59cd6:     	callq	0x142923c10 <.text+0x2922c10>
140b59cdb:     	movq	%rdi, %rcx
140b59cde:     	movq	%r15, 0x10(%rdi)
140b59ce2:     	movq	%r14, %rbx
140b59ce5:     	movq	%r15, %rdi
140b59ce8:     	jmp	0x140b59d10 <.text+0xb58d10>
140b59cea:     	movq	0x10(%rcx), %rdi
140b59cee:     	jmp	0x140b59d10 <.text+0xb58d10>
140b59cf0:     	movq	0x8(%rcx), %rdx
140b59cf4:     	imulq	$0x38, %rbx, %rax
140b59cf8:     	addq	%rdx, %rax
140b59cfb:     	imulq	$0x38, %r9, %r8
140b59cff:     	movq	%rcx, %rbx
140b59d02:     	movq	%rax, %rcx
140b59d05:     	callq	0x142923c00 <.text+0x2922c00>
140b59d0a:     	movq	%rbx, %rcx
140b59d0d:     	movq	%r14, %rbx
140b59d10:     	addq	%rsi, %rdi
140b59d13:     	xorl	%eax, %eax
140b59d15:     	cmpq	%rbx, %rdi
140b59d18:     	cmovaeq	%rbx, %rax
140b59d1c:     	subq	%rax, %rdi
140b59d1f:     	movaps	0xa0(%rbp), %xmm0
140b59d26:     	movaps	%xmm0, 0x20(%rbp)
140b59d2a:     	movaps	0x90(%rbp), %xmm0
140b59d31:     	movaps	%xmm0, 0x10(%rbp)
140b59d35:     	movaps	0x80(%rbp), %xmm0
140b59d3c:     	movaps	%xmm0, (%rbp)
140b59d40:     	movaps	0x70(%rbp), %xmm0
140b59d44:     	movaps	%xmm0, -0x10(%rbp)
140b59d48:     	movaps	0x30(%rbp), %xmm0
140b59d4c:     	movaps	0x40(%rbp), %xmm1
140b59d50:     	movaps	0x50(%rbp), %xmm2
140b59d54:     	movaps	0x60(%rbp), %xmm3
140b59d58:     	movaps	%xmm3, -0x20(%rbp)
140b59d5c:     	movaps	%xmm2, -0x30(%rbp)
140b59d60:     	movaps	%xmm1, -0x40(%rbp)
140b59d64:     	movaps	%xmm0, -0x50(%rbp)
140b59d68:     	movq	%rbx, %rax
140b59d6b:     	subq	%rdi, %rax
140b59d6e:     	cmpq	$0x1, %rax
140b59d72:     	ja	0x140b59e43 <.text+0xb58e43>
140b59d78:     	cmpq	%rdi, %rbx
140b59d7b:     	je	0x140b59db3 <.text+0xb58db3>
140b59d7d:     	movq	0x20(%rbp), %rdx
140b59d81:     	cmpq	%rdx, 0x28(%rbp)
140b59d85:     	je	0x140b59db3 <.text+0xb58db3>
140b59d87:     	imulq	$0x38, %rdi, %rax
140b59d8b:     	addq	0x8(%rcx), %rax
140b59d8f:     	leaq	0x1(%rdx), %r8
140b59d93:     	movq	%r8, 0x20(%rbp)
140b59d97:     	imulq	$0x38, %rdx, %r8
140b59d9b:     	movq	-0x50(%rbp,%r8), %rdx
140b59da0:     	movabsq	$-0x7fffffffffffffec, %r9 # imm = 0x8000000000000014
140b59daa:     	cmpq	%r9, %rdx
140b59dad:     	jne	0x140b5a0b8 <.text+0xb590b8>
140b59db3:     	xorl	%eax, %eax
140b59db5:     	movaps	0x20(%rbp), %xmm0
140b59db9:     	movaps	%xmm0, 0x120(%rbp)
140b59dc0:     	movaps	0x10(%rbp), %xmm0
140b59dc4:     	movaps	%xmm0, 0x110(%rbp)
140b59dcb:     	movaps	(%rbp), %xmm0
140b59dcf:     	movaps	%xmm0, 0x100(%rbp)
140b59dd6:     	movaps	-0x10(%rbp), %xmm0
140b59dda:     	movaps	%xmm0, 0xf0(%rbp)
140b59de1:     	movaps	-0x50(%rbp), %xmm0
140b59de5:     	movaps	-0x40(%rbp), %xmm1
140b59de9:     	movaps	-0x30(%rbp), %xmm2
140b59ded:     	movaps	-0x20(%rbp), %xmm3
140b59df1:     	movaps	%xmm3, 0xe0(%rbp)
140b59df8:     	movaps	%xmm2, 0xd0(%rbp)
140b59dff:     	movaps	%xmm1, 0xc0(%rbp)
140b59e06:     	movaps	%xmm0, 0xb0(%rbp)
140b59e0d:     	movq	0x120(%rbp), %rdx
140b59e14:     	movq	0x128(%rbp), %r8
140b59e1b:     	movq	%r8, %r10
140b59e1e:     	subq	%rdx, %r10
140b59e21:     	je	0x140b5a09d <.text+0xb5909d>
140b59e27:     	movq	0x8(%rcx), %r9
140b59e2b:     	leaq	0x1(%rdx), %r11
140b59e2f:     	cmpq	%r11, %r8
140b59e32:     	jne	0x140b59ee6 <.text+0xb58ee6>
140b59e38:     	xorl	%r11d, %r11d
140b59e3b:     	movq	%rdx, %rdi
140b59e3e:     	jmp	0x140b59f7e <.text+0xb58f7e>
140b59e43:     	movaps	0xa0(%rbp), %xmm0
140b59e4a:     	movaps	%xmm0, 0x120(%rbp)
140b59e51:     	movaps	0x90(%rbp), %xmm0
140b59e58:     	movaps	%xmm0, 0x110(%rbp)
140b59e5f:     	movaps	0x80(%rbp), %xmm0
140b59e66:     	movaps	%xmm0, 0x100(%rbp)
140b59e6d:     	movaps	0x70(%rbp), %xmm0
140b59e71:     	movaps	%xmm0, 0xf0(%rbp)
140b59e78:     	movaps	0x30(%rbp), %xmm0
140b59e7c:     	movaps	0x40(%rbp), %xmm1
