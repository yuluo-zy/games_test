
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1414c9d40:     	pushq	%r15
1414c9d42:     	pushq	%r14
1414c9d44:     	pushq	%r13
1414c9d46:     	pushq	%r12
1414c9d48:     	pushq	%rsi
1414c9d49:     	pushq	%rdi
1414c9d4a:     	pushq	%rbx
1414c9d4b:     	subq	$0x90, %rsp
1414c9d52:     	testb	$0x1, (%rcx)
1414c9d55:     	je	0x1414c9fa3 <.text+0x14c8fa3>
1414c9d5b:     	movq	%rcx, %rsi
1414c9d5e:     	movq	0x5a0(%rdx), %rax
1414c9d65:     	movq	%rax, 0x30(%rsp)
1414c9d6a:     	cmpq	%rax, 0x18(%rcx)
1414c9d6e:     	jne	0x1414c9ea4 <.text+0x14c8ea4>
1414c9d74:     	movq	0x108(%rdx), %rax
1414c9d7b:     	movl	0x158(%rsi), %ecx
1414c9d81:     	movl	%eax, 0x158(%rsi)
1414c9d87:     	cmpq	%rcx, %rax
1414c9d8a:     	jb	0x1414c9eee <.text+0x14c8eee>
1414c9d90:     	movl	$0x1, %ebx
1414c9d95:     	lock
1414c9d96:     	xaddl	%ebx, 0x5b0(%rdx)
1414c9d9d:     	movq	0x8(%rsi), %rax
1414c9da1:     	cmpq	%rax, 0x218(%rdx)
1414c9da8:     	jbe	0x1414c9efd <.text+0x14c8efd>
1414c9dae:     	movq	0x210(%rdx), %rcx
1414c9db5:     	movq	(%rcx,%rax,8), %rcx
1414c9db9:     	testq	%rcx, %rcx
1414c9dbc:     	je	0x1414c9efd <.text+0x14c8efd>
1414c9dc2:     	movq	0x1e0(%rdx), %rax
1414c9dc9:     	notq	%rcx
1414c9dcc:     	leaq	(%rcx,%rcx,2), %rcx
1414c9dd0:     	shlq	$0x5, %rcx
1414c9dd4:     	cmpq	$0x0, 0x20(%rax,%rcx)
1414c9dda:     	je	0x1414c9efd <.text+0x14c8efd>
1414c9de0:     	addq	%rcx, %rax
1414c9de3:     	movq	0x10(%rax), %r14
1414c9de7:     	movl	0x5b0(%rdx), %ecx
1414c9ded:     	movq	0x10(%rsi), %rcx
1414c9df1:     	cmpq	%rcx, 0x218(%rdx)
1414c9df8:     	jbe	0x1414c9f1b <.text+0x14c8f1b>
1414c9dfe:     	movq	0x210(%rdx), %r8
1414c9e05:     	movq	(%r8,%rcx,8), %rcx
1414c9e09:     	testq	%rcx, %rcx
1414c9e0c:     	je	0x1414c9f1b <.text+0x14c8f1b>
1414c9e12:     	movq	0x1e0(%rdx), %r15
1414c9e19:     	notq	%rcx
1414c9e1c:     	leaq	(%rcx,%rcx,2), %rcx
1414c9e20:     	shlq	$0x5, %rcx
1414c9e24:     	cmpq	$0x0, 0x20(%r15,%rcx)
1414c9e2a:     	je	0x1414c9f1b <.text+0x14c8f1b>
1414c9e30:     	addq	%rcx, %r15
1414c9e33:     	movq	0x10(%r15), %rdi
1414c9e37:     	movl	0x5b0(%rdx), %ecx
1414c9e3d:     	movl	%ebx, 0x5c(%rax)
1414c9e40:     	movq	0x30(%r14), %r12
1414c9e44:     	movq	0x40(%r14), %r13
1414c9e48:     	cmpq	0x20(%r14), %r12
1414c9e4c:     	jne	0x1414c9e5e <.text+0x14c8e5e>
1414c9e4e:     	leaq	0x20(%r14), %rcx
1414c9e52:     	leaq	0x17702ef(%rip), %rdx   # 0x142c3a148
1414c9e59:     	callq	0x140562630 <.text+0x561630>
1414c9e5e:     	movq	0x28(%r14), %rax
1414c9e62:     	movq	%r12, %rcx
1414c9e65:     	shlq	$0x4, %rcx
1414c9e69:     	movq	%r13, (%rax,%rcx)
1414c9e6d:     	movb	$0x0, 0x8(%rax,%rcx)
1414c9e72:     	incq	%r12
1414c9e75:     	movq	%r12, 0x30(%r14)
1414c9e79:     	incq	0x40(%r14)
1414c9e7d:     	movl	%ebx, 0x5c(%r15)
1414c9e81:     	movq	%rdi, %rcx
1414c9e84:     	callq	0x140a43840 <.text+0xa42840>
1414c9e89:     	movl	%ebx, 0x150(%rsi)
1414c9e8f:     	xorl	%eax, %eax
1414c9e91:     	addq	$0x90, %rsp
1414c9e98:     	popq	%rbx
1414c9e99:     	popq	%rdi
1414c9e9a:     	popq	%rsi
1414c9e9b:     	popq	%r12
1414c9e9d:     	popq	%r13
1414c9e9f:     	popq	%r14
1414c9ea1:     	popq	%r15
1414c9ea3:     	retq
1414c9ea4:     	leaq	0x178dab5(%rip), %rax   # 0x142c57960
1414c9eab:     	movq	%rax, 0x60(%rsp)
1414c9eb0:     	movq	$0x1, 0x68(%rsp)
1414c9eb9:     	movq	$0x8, 0x70(%rsp)
1414c9ec2:     	xorps	%xmm0, %xmm0
1414c9ec5:     	movups	%xmm0, 0x78(%rsp)
1414c9eca:     	addq	$0x18, %rsi
1414c9ece:     	leaq	0x178da9b(%rip), %rax   # 0x142c57970
1414c9ed5:     	movq	%rax, 0x20(%rsp)
1414c9eda:     	leaq	0x30(%rsp), %r8
1414c9edf:     	leaq	0x60(%rsp), %r9
1414c9ee4:     	xorl	%ecx, %ecx
1414c9ee6:     	movq	%rsi, %rdx
1414c9ee9:     	callq	0x1428ce0e1 <.text+0x28cd0e1>
1414c9eee:     	leaq	0x178da93(%rip), %r8    # 0x142c57988
1414c9ef5:     	movq	%rax, %rdx
1414c9ef8:     	callq	0x1428d9830 <.text+0x28d8830>
1414c9efd:     	addq	$0xb8, %rsi
1414c9f04:     	leaq	0x17a1033(%rip), %rax   # 0x142c6af3e
1414c9f0b:     	movq	%rax, 0x50(%rsp)
1414c9f10:     	movq	$0x55, 0x58(%rsp)
1414c9f19:     	jmp	0x1414c9f37 <.text+0x14c8f37>
1414c9f1b:     	addq	$0xb8, %rsi
1414c9f22:     	leaq	0x179ed87(%rip), %rax   # 0x142c68cb0
1414c9f29:     	movq	%rax, 0x50(%rsp)
1414c9f2e:     	movq	$0x35, 0x58(%rsp)
1414c9f37:     	movq	%rsi, 0x30(%rsp)
1414c9f3c:     	leaq	-0x148e473(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
1414c9f43:     	movq	%rax, 0x38(%rsp)
1414c9f48:     	leaq	0x50(%rsp), %rax
1414c9f4d:     	movq	%rax, 0x40(%rsp)
1414c9f52:     	leaq	-0x13c00c9(%rip), %rax  # 0x140109e90 <.text+0x108e90>
