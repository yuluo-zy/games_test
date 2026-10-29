
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141fe6a40:     	pushq	%r15
141fe6a42:     	pushq	%r14
141fe6a44:     	pushq	%r13
141fe6a46:     	pushq	%r12
141fe6a48:     	pushq	%rsi
141fe6a49:     	pushq	%rdi
141fe6a4a:     	pushq	%rbp
141fe6a4b:     	pushq	%rbx
141fe6a4c:     	subq	$0x108, %rsp            # imm = 0x108
141fe6a53:     	movaps	%xmm8, 0xf0(%rsp)
141fe6a5c:     	movaps	%xmm7, 0xe0(%rsp)
141fe6a64:     	movaps	%xmm6, 0xd0(%rsp)
141fe6a6c:     	movq	(%rdx), %r13
141fe6a6f:     	movl	(%r8), %ebx
141fe6a72:     	movl	%ebx, 0x4c(%rsp)
141fe6a76:     	movq	(%r13), %rdi
141fe6a7a:     	cmpq	$0x0, 0x48(%rdi)
141fe6a7f:     	je	0x141fe6bf9 <.text+0x1fe5bf9>
141fe6a85:     	movq	%rcx, %rsi
141fe6a88:     	leaq	0x50(%rdi), %rcx
141fe6a8c:     	leaq	0x4c(%rsp), %rdx
141fe6a91:     	callq	0x1405b0ed0 <.text+0x5afed0>
141fe6a96:     	movq	0x30(%rdi), %r14
141fe6a9a:     	movq	0x38(%rdi), %rcx
141fe6a9e:     	movq	%rcx, %rdx
141fe6aa1:     	andq	%rax, %rdx
141fe6aa4:     	shrq	$0x39, %rax
141fe6aa8:     	movd	%eax, %xmm0
141fe6aac:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141fe6ab0:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141fe6ab5:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141fe6aba:     	leaq	-0x50(%r14), %rax
141fe6abe:     	xorl	%r8d, %r8d
141fe6ac1:     	pcmpeqd	%xmm1, %xmm1
141fe6ac5:     	movdqu	(%r14,%rdx), %xmm2
141fe6acb:     	movdqa	%xmm2, %xmm3
141fe6acf:     	pcmpeqb	%xmm0, %xmm3
141fe6ad3:     	pmovmskb	%xmm3, %r9d
141fe6ad8:     	testl	%r9d, %r9d
141fe6adb:     	je	0x141fe6b10 <.text+0x1fe5b10>
141fe6add:     	tzcntl	%r9d, %r10d
141fe6ae2:     	addq	%rdx, %r10
141fe6ae5:     	andq	%rcx, %r10
141fe6ae8:     	negq	%r10
141fe6aeb:     	leaq	(%r10,%r10,4), %rdi
141fe6aef:     	shlq	$0x4, %rdi
141fe6af3:     	cmpl	(%rax,%rdi), %ebx
141fe6af6:     	je	0x141fe6b32 <.text+0x1fe5b32>
141fe6af8:     	leal	-0x1(%r9), %r10d
141fe6afc:     	andw	%r9w, %r10w
141fe6b00:     	movl	%r10d, %r9d
141fe6b03:     	jne	0x141fe6add <.text+0x1fe5add>
141fe6b05:     	nopw	%cs:(%rax,%rax)
141fe6b10:     	pcmpeqb	%xmm1, %xmm2
141fe6b14:     	pmovmskb	%xmm2, %r9d
141fe6b19:     	testl	%r9d, %r9d
141fe6b1c:     	jne	0x141fe6bf9 <.text+0x1fe5bf9>
141fe6b22:     	addq	%r8, %rdx
141fe6b25:     	addq	$0x10, %rdx
141fe6b29:     	addq	$0x10, %r8
141fe6b2d:     	andq	%rcx, %rdx
141fe6b30:     	jmp	0x141fe6ac5 <.text+0x1fe5ac5>
141fe6b32:     	movq	0x8(%r13), %r15
141fe6b36:     	movq	%r15, %rcx
141fe6b39:     	movl	%ebx, 0x48(%rsp)
141fe6b3d:     	movl	%ebx, %edx
141fe6b3f:     	callq	0x140b26970 <.text+0xb25970>
141fe6b44:     	movl	%eax, %ebp
141fe6b46:     	movzbl	-0x4(%r14,%rdi), %ebx
141fe6b4c:     	movq	(%r15), %rdx
141fe6b4f:     	movl	%eax, %ecx
141fe6b51:     	callq	0x140833250 <.text+0x832250>
141fe6b56:     	movq	0x8(%r15), %r12
141fe6b5a:     	movl	%ebx, %ecx
141fe6b5c:     	movl	%eax, %edx
141fe6b5e:     	callq	0x140b25c90 <.text+0xb24c90>
141fe6b63:     	movb	%al, 0x30(%rsp)
141fe6b67:     	movb	%dl, 0x31(%rsp)
141fe6b6b:     	leaq	0x30(%rsp), %rdx
141fe6b70:     	movq	%r12, %rcx
141fe6b73:     	callq	0x14125cf90 <.text+0x125bf90>
141fe6b78:     	testq	%rax, %rax
141fe6b7b:     	je	0x141fe6e88 <.text+0x1fe5e88>
141fe6b81:     	leaq	(%r14,%rdi), %rdx
141fe6b85:     	movq	0x10(%r13), %r12
141fe6b89:     	movzbl	0x46(%r12), %ecx
141fe6b8f:     	leal	-0x38(%rcx), %r8d
141fe6b93:     	cmpb	$0x8, %r8b
141fe6b97:     	movzbl	%r8b, %r8d
141fe6b9b:     	movl	$0x6, %r10d
141fe6ba1:     	cmovael	%r10d, %r8d
141fe6ba5:     	cmpb	$0x6, %r8b
141fe6ba9:     	sete	%r11b
141fe6bad:     	movzbl	-0x2(%rdx), %ebx
141fe6bb1:     	leal	-0x38(%rbx), %r9d
141fe6bb5:     	cmpb	$0x8, %r9b
141fe6bb9:     	movzbl	%r9b, %r9d
141fe6bbd:     	cmovael	%r10d, %r9d
141fe6bc1:     	cmpb	%cl, %bl
141fe6bc3:     	setne	%r10b
141fe6bc7:     	testb	%r10b, %r11b
141fe6bca:     	jne	0x141fe6bee <.text+0x1fe5bee>
141fe6bcc:     	cmpb	%r8b, %r9b
141fe6bcf:     	jne	0x141fe6bee <.text+0x1fe5bee>
141fe6bd1:     	movss	0x30(%rax), %xmm8
141fe6bd7:     	movzbl	0x44(%r12), %eax
141fe6bdd:     	movzbl	-0x4(%rdx), %ebx
141fe6be1:     	addq	%rax, %rbx
141fe6be4:     	callq	0x140833130 <.text+0x832130>
141fe6be9:     	cmpq	%rax, %rbx
141fe6bec:     	jbe	0x141fe6c11 <.text+0x1fe5c11>
141fe6bee:     	movl	$0x4, (%rsi)
141fe6bf4:     	jmp	0x141fe6e58 <.text+0x1fe5e58>
141fe6bf9:     	leaq	0xfe5100(%rip), %rcx    # 0x142fcbd00
141fe6c00:     	leaq	0xfe51c9(%rip), %r8     # 0x142fcbdd0
141fe6c07:     	movl	$0x16, %edx
141fe6c0c:     	callq	0x1428d9330 <.text+0x28d8330>
141fe6c11:     	movq	0x20(%r12), %rax
141fe6c16:     	movq	%rax, 0xa0(%rsp)
141fe6c1e:     	movups	(%r12), %xmm0
141fe6c23:     	movdqu	0x10(%r12), %xmm1
141fe6c2a:     	movdqa	%xmm1, 0x90(%rsp)
141fe6c33:     	movaps	%xmm0, 0x80(%rsp)
141fe6c3b:     	movzwl	0x45(%r12), %eax
141fe6c41:     	movw	%ax, 0x2c(%rsp)
141fe6c46:     	movzbl	0x47(%r12), %eax
141fe6c4c:     	movb	%al, 0x2b(%rsp)
141fe6c50:     	movsd	0x28(%r12), %xmm6
141fe6c57:     	movl	0x40(%r12), %eax
141fe6c5c:     	movl	%eax, 0xc0(%rsp)
141fe6c63:     	movups	0x30(%r12), %xmm0
141fe6c69:     	movaps	%xmm0, 0xb0(%rsp)
141fe6c71:     	movq	(%r15), %rdx
141fe6c74:     	movl	%ebp, %ecx
141fe6c76:     	callq	0x140833250 <.text+0x832250>
141fe6c7b:     	movq	0x8(%r15), %r12
141fe6c7f:     	movl	%ebx, %ecx
141fe6c81:     	movl	%eax, %edx
141fe6c83:     	callq	0x140b25c90 <.text+0xb24c90>
141fe6c88:     	movb	%al, 0x2e(%rsp)
141fe6c8c:     	movb	%dl, 0x2f(%rsp)
141fe6c90:     	leaq	0x2e(%rsp), %r15
141fe6c95:     	movq	%r12, %rcx
141fe6c98:     	movq	%r15, %rdx
141fe6c9b:     	callq	0x14125cf90 <.text+0x125bf90>
141fe6ca0:     	testq	%rax, %rax
141fe6ca3:     	je	0x141fe6ee9 <.text+0x1fe5ee9>
141fe6ca9:     	movss	0x30(%rax), %xmm7
141fe6cae:     	subss	%xmm8, %xmm7
141fe6cb3:     	mulss	0x93ebb5(%rip), %xmm7   # 0x142925870
141fe6cbb:     	addq	%rdi, %r14
141fe6cbe:     	addq	$-0x48, %r14
141fe6cc2:     	movq	0x18(%r13), %rdx
141fe6cc6:     	testb	$0x1, 0x8(%rdx)
141fe6cca:     	je	0x141fe6d39 <.text+0x1fe5d39>
141fe6ccc:     	movq	0x20(%r13), %rdi
141fe6cd0:     	movl	0x8(%rdi), %eax
141fe6cd3:     	movl	%eax, 0x38(%rsp)
141fe6cd7:     	movq	(%rdi), %rax
141fe6cda:     	movq	%rax, 0x30(%rsp)
141fe6cdf:     	leaq	0x50(%rsp), %r15
141fe6ce4:     	leaq	0x30(%rsp), %r8
141fe6ce9:     	movq	%r15, %rcx
141fe6cec:     	callq	0x1408e4750 <.text+0x8e3750>
141fe6cf1:     	movq	%r15, %rcx
141fe6cf4:     	callq	0x1411ff740 <.text+0x11fe740>
141fe6cf9:     	movups	(%rax), %xmm0
141fe6cfc:     	movaps	%xmm0, 0x30(%rsp)
141fe6d01:     	movq	0x10(%rax), %rax
141fe6d05:     	movq	%rax, 0x40(%rsp)
141fe6d0a:     	movd	(%rdi), %xmm1
141fe6d0e:     	movd	0x8(%rdi), %xmm2
141fe6d13:     	leaq	0x30(%rsp), %r15
141fe6d18:     	movq	%r15, %rcx
141fe6d1b:     	callq	0x140c9acb0 <.text+0xc99cb0>
141fe6d20:     	movq	%r15, %rcx
141fe6d23:     	movq	%rax, %rdx
141fe6d26:     	callq	0x140c9ca50 <.text+0xc9ba50>
141fe6d2b:     	divss	%xmm0, %xmm7
141fe6d2f:     	mulss	0x93ec19(%rip), %xmm7   # 0x142925950
141fe6d37:     	jmp	0x141fe6d6a <.text+0x1fe5d6a>
141fe6d39:     	movq	0x20(%r13), %rax
141fe6d3d:     	movl	0x8(%rax), %ecx
141fe6d40:     	movl	%ecx, 0x38(%rsp)
141fe6d44:     	movq	(%rax), %rax
141fe6d47:     	movq	%rax, 0x30(%rsp)
141fe6d4c:     	leaq	0x50(%rsp), %r15
141fe6d51:     	leaq	0x30(%rsp), %r8
141fe6d56:     	movq	%r15, %rcx
141fe6d59:     	callq	0x1408e4750 <.text+0x8e3750>
141fe6d5e:     	movq	%r15, %rcx
141fe6d61:     	callq	0x1411ff640 <.text+0x11fe640>
141fe6d66:     	divss	%xmm0, %xmm7
141fe6d6a:     	movzwl	0x2c(%rsp), %edi
141fe6d6f:     	movzbl	0x2b(%rsp), %ebp
141fe6d74:     	leaq	0xfe506d(%rip), %rdx    # 0x142fcbde8
141fe6d7b:     	movq	%r14, %rcx
141fe6d7e:     	callq	0x1408333b0 <.text+0x8323b0>
141fe6d83:     	movq	%rax, %r15
141fe6d86:     	leaq	0xfe5073(%rip), %rdx    # 0x142fcbe00
141fe6d8d:     	movq	%r14, %rcx
141fe6d90:     	callq	0x1408333b0 <.text+0x8323b0>
141fe6d95:     	movss	(%rax), %xmm1
141fe6d99:     	movss	0x4(%rax), %xmm0
141fe6d9e:     	movq	0x28(%r13), %rax
141fe6da2:     	movss	(%rax), %xmm2
141fe6da6:     	movaps	0x93eb43(%rip), %xmm3   # 0x1429258f0
141fe6dad:     	xorps	%xmm7, %xmm3
141fe6db0:     	movaps	%xmm1, %xmm4
141fe6db3:     	cmpltss	%xmm2, %xmm4
141fe6db8:     	andps	%xmm4, %xmm7
141fe6dbb:     	andnps	%xmm3, %xmm4
141fe6dbe:     	orps	%xmm7, %xmm4
141fe6dc1:     	addss	%xmm1, %xmm4
141fe6dc5:     	movss	0x8(%r15), %xmm1
141fe6dcb:     	movaps	%xmm0, %xmm3
141fe6dce:     	subss	0x4(%rax), %xmm3
141fe6dd3:     	movl	$0x0, (%rsi)
141fe6dd9:     	movss	%xmm1, 0x4(%rsi)
141fe6dde:     	movss	%xmm4, 0x8(%rsi)
141fe6de3:     	subss	%xmm2, %xmm4
141fe6de7:     	mulss	%xmm4, %xmm4
141fe6deb:     	mulss	%xmm3, %xmm3
141fe6def:     	addss	%xmm4, %xmm3
141fe6df3:     	xorps	%xmm1, %xmm1
141fe6df6:     	sqrtss	%xmm3, %xmm1
141fe6dfa:     	movss	%xmm0, 0xc(%rsi)
141fe6dff:     	movaps	0x80(%rsp), %xmm0
141fe6e07:     	movdqa	0x90(%rsp), %xmm2
141fe6e10:     	movdqu	%xmm2, 0x30(%rsi)
141fe6e15:     	movups	%xmm0, 0x20(%rsi)
141fe6e19:     	movq	0xa0(%rsp), %rax
141fe6e21:     	movq	%rax, 0x40(%rsi)
141fe6e25:     	movlps	%xmm6, 0x48(%rsi)
141fe6e29:     	movdqa	0xb0(%rsp), %xmm0
141fe6e32:     	movdqu	%xmm0, 0x50(%rsi)
141fe6e37:     	movl	0xc0(%rsp), %eax
141fe6e3e:     	movl	%eax, 0x60(%rsi)
141fe6e41:     	movb	%bl, 0x64(%rsi)
141fe6e44:     	movw	%di, 0x65(%rsi)
141fe6e48:     	movb	%bpl, 0x67(%rsi)
141fe6e4c:     	movl	0x48(%rsp), %eax
141fe6e50:     	movl	%eax, 0x68(%rsi)
141fe6e53:     	movss	%xmm1, 0x6c(%rsi)
141fe6e58:     	movq	%rsi, %rax
141fe6e5b:     	movaps	0xd0(%rsp), %xmm6
141fe6e63:     	movaps	0xe0(%rsp), %xmm7
141fe6e6b:     	movaps	0xf0(%rsp), %xmm8
141fe6e74:     	addq	$0x108, %rsp            # imm = 0x108
141fe6e7b:     	popq	%rbx
141fe6e7c:     	popq	%rbp
141fe6e7d:     	popq	%rdi
141fe6e7e:     	popq	%rsi
141fe6e7f:     	popq	%r12
141fe6e81:     	popq	%r13
141fe6e83:     	popq	%r14
141fe6e85:     	popq	%r15
141fe6e87:     	retq
141fe6e88:     	leaq	0x30(%rsp), %rax
141fe6e8d:     	movq	%rax, 0x80(%rsp)
141fe6e95:     	leaq	-0xd89fac(%rip), %rax   # 0x14125cef0 <.text+0x125bef0>
141fe6e9c:     	movq	%rax, 0x88(%rsp)
141fe6ea4:     	leaq	0xc3add5(%rip), %rax    # 0x142c21c80
141fe6eab:     	movq	%rax, 0x50(%rsp)
141fe6eb0:     	movq	$0x1, 0x58(%rsp)
141fe6eb9:     	movq	$0x0, 0x70(%rsp)
141fe6ec2:     	leaq	0x80(%rsp), %rax
141fe6eca:     	movq	%rax, 0x60(%rsp)
141fe6ecf:     	movq	$0x1, 0x68(%rsp)
141fe6ed8:     	leaq	0xc3adb1(%rip), %rdx    # 0x142c21c90
141fe6edf:     	leaq	0x50(%rsp), %rcx
141fe6ee4:     	callq	0x1428d9390 <.text+0x28d8390>
141fe6ee9:     	movq	%r15, 0x30(%rsp)
141fe6eee:     	leaq	-0xd8a005(%rip), %rax   # 0x14125cef0 <.text+0x125bef0>
141fe6ef5:     	movq	%rax, 0x38(%rsp)
141fe6efa:     	leaq	0xc3ad7f(%rip), %rax    # 0x142c21c80
141fe6f01:     	movq	%rax, 0x50(%rsp)
141fe6f06:     	movq	$0x1, 0x58(%rsp)
141fe6f0f:     	movq	$0x0, 0x70(%rsp)
141fe6f18:     	leaq	0x30(%rsp), %rax
141fe6f1d:     	jmp	0x141fe6eca <.text+0x1fe5eca>
141fe6f1f:     	int3
141fe6f20:     	pushq	%r14
141fe6f22:     	pushq	%rsi
141fe6f23:     	pushq	%rdi
141fe6f24:     	pushq	%rbp
141fe6f25:     	pushq	%rbx
141fe6f26:     	subq	$0xe0, %rsp
141fe6f2d:     	movaps	%xmm8, 0xd0(%rsp)
141fe6f36:     	movaps	%xmm7, 0xc0(%rsp)
141fe6f3e:     	movaps	%xmm6, 0xb0(%rsp)
141fe6f46:     	movq	(%rcx), %r14
141fe6f49:     	movl	(%rdx), %edx
141fe6f4b:     	movq	(%r14), %rax
141fe6f4e:     	movq	(%rax), %rcx
141fe6f51:     	xorl	%eax, %eax
141fe6f53:     	cmpl	%edx, (%rcx)
141fe6f55:     	je	0x141fe71ec <.text+0x1fe61ec>
141fe6f5b:     	movzbl	0x22(%r8), %ecx
141fe6f60:     	leal	-0x2(%rcx), %r9d
141fe6f64:     	decq	%rcx
141fe6f67:     	xorl	%eax, %eax
141fe6f69:     	cmpb	$0x3, %r9b
141fe6f6d:     	cmovaeq	%rax, %rcx
141fe6f71:     	leaq	0xfe4eb8(%rip), %r9     # 0x142fcbe30
141fe6f78:     	movslq	(%r9,%rcx,4), %rcx
141fe6f7c:     	addq	%r9, %rcx
141fe6f7f:     	jmpq	*%rcx
141fe6f81:     	movq	0x8(%r14), %rcx
141fe6f85:     	movzbl	(%rcx), %ecx
141fe6f88:     	movzbl	0x46(%r8), %r9d
141fe6f8d:     	leal	-0x38(%rcx), %r10d
141fe6f91:     	cmpb	$0x8, %r10b
141fe6f95:     	movzbl	%r10b, %r10d
141fe6f99:     	movl	$0x6, %r11d
141fe6f9f:     	cmovael	%r11d, %r10d
141fe6fa3:     	leal	-0x38(%r9), %esi
141fe6fa7:     	cmpb	$0x8, %sil
141fe6fab:     	movzbl	%sil, %esi
141fe6faf:     	cmovael	%r11d, %esi
141fe6fb3:     	cmpb	%sil, %r10b
141fe6fb6:     	jne	0x141fe71ec <.text+0x1fe61ec>
141fe6fbc:     	cmpb	$0x6, %r10b
141fe6fc0:     	sete	%r10b
141fe6fc4:     	cmpb	%r9b, %cl
141fe6fc7:     	setne	%cl
141fe6fca:     	testb	%r10b, %cl
141fe6fcd:     	jne	0x141fe71ec <.text+0x1fe61ec>
141fe6fd3:     	movq	0x10(%r14), %rax
141fe6fd7:     	leaq	0xfe4dc2(%rip), %r9     # 0x142fcbda0
141fe6fde:     	leaq	0x80(%rsp), %rcx
141fe6fe6:     	movl	%edx, %esi
141fe6fe8:     	movq	%rax, %rdx
141fe6feb:     	movq	%r8, %rbx
141fe6fee:     	movl	%esi, %r8d
141fe6ff1:     	callq	0x140a1aa20 <.text+0xa19a20>
141fe6ff6:     	movq	0x18(%r14), %rdi
141fe6ffa:     	movq	%rdi, %rcx
141fe6ffd:     	movl	%esi, %ebp
141fe6fff:     	movl	%esi, %edx
141fe7001:     	callq	0x140b26970 <.text+0xb25970>
141fe7006:     	movzbl	0x44(%rbx), %ebx
141fe700a:     	movq	(%rdi), %rdx
141fe700d:     	movl	%eax, %ecx
141fe700f:     	callq	0x140833250 <.text+0x832250>
141fe7014:     	movq	0x8(%rdi), %rdi
141fe7018:     	movl	%ebx, %ecx
141fe701a:     	movl	%eax, %edx
141fe701c:     	callq	0x140b25c90 <.text+0xb24c90>
141fe7021:     	movb	%al, 0x30(%rsp)
141fe7025:     	movb	%dl, 0x31(%rsp)
141fe7029:     	leaq	0x30(%rsp), %rsi
141fe702e:     	movq	%rdi, %rcx
141fe7031:     	movq	%rsi, %rdx
141fe7034:     	callq	0x14125cf90 <.text+0x125bf90>
141fe7039:     	testq	%rax, %rax
141fe703c:     	je	0x141fe722b <.text+0x1fe622b>
141fe7042:     	movups	0x30(%rax), %xmm1
141fe7046:     	movss	0x3c(%rax), %xmm2
141fe704b:     	subss	0x38(%rax), %xmm2
141fe7050:     	xorps	%xmm4, %xmm4
141fe7053:     	mulss	%xmm2, %xmm4
141fe7057:     	movaps	0x80(%rsp), %xmm3
141fe705f:     	movaps	%xmm3, %xmm0
141fe7062:     	mulps	%xmm3, %xmm0
141fe7065:     	movshdup	%xmm0, %xmm5            # xmm5 = xmm0[1,1,3,3]
141fe7069:     	addss	%xmm0, %xmm5
141fe706d:     	movaps	%xmm0, %xmm6
141fe7070:     	unpckhpd	%xmm0, %xmm6            # xmm6 = xmm6[1],xmm0[1]
141fe7074:     	addss	%xmm5, %xmm6
141fe7078:     	shufps	$0x0, %xmm6, %xmm6      # xmm6 = xmm6[0,0,0,0]
141fe707c:     	shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
141fe7080:     	subps	%xmm6, %xmm0
141fe7083:     	movaps	0xfe4c56(%rip), %xmm6   # 0x142fcbce0
141fe708a:     	movaps	%xmm0, %xmm5
141fe708d:     	mulps	%xmm6, %xmm5
141fe7090:     	movaps	%xmm3, %xmm7
141fe7093:     	mulps	%xmm6, %xmm7
141fe7096:     	movshdup	%xmm7, %xmm8            # xmm8 = xmm7[1,1,3,3]
141fe709b:     	addss	%xmm7, %xmm8
141fe70a0:     	movhlps	%xmm7, %xmm7            # xmm7 = xmm7[1,1]
141fe70a3:     	addss	%xmm8, %xmm7
141fe70a8:     	addss	%xmm7, %xmm7
141fe70ac:     	shufps	$0x0, %xmm7, %xmm7      # xmm7 = xmm7[0,0,0,0]
141fe70b0:     	mulps	%xmm3, %xmm7
141fe70b3:     	addps	%xmm5, %xmm7
141fe70b6:     	movaps	%xmm3, %xmm5
141fe70b9:     	shufps	$0xd2, %xmm3, %xmm5     # xmm5 = xmm5[2,0],xmm3[1,3]
141fe70bd:     	mulps	%xmm5, %xmm6
141fe70c0:     	movaps	0xfe4c28(%rip), %xmm8   # 0x142fcbcf0
141fe70c8:     	mulps	%xmm3, %xmm8
141fe70cc:     	addps	%xmm6, %xmm8
141fe70d0:     	shufps	$0xd2, %xmm8, %xmm8     # xmm8 = xmm8[2,0,1,3]
141fe70d5:     	movaps	%xmm3, %xmm6
141fe70d8:     	addps	%xmm3, %xmm6
141fe70db:     	shufps	$0xff, %xmm6, %xmm6     # xmm6 = xmm6[3,3,3,3]
141fe70df:     	mulps	%xmm6, %xmm8
141fe70e3:     	addps	%xmm7, %xmm8
141fe70e7:     	movsd	0x90(%rsp), %xmm7
141fe70f0:     	unpcklps	%xmm2, %xmm4            # xmm4 = xmm4[0],xmm2[0],xmm4[1],xmm2[1]
141fe70f3:     	mulps	0x93e7e6(%rip), %xmm4   # 0x1429258e0
141fe70fa:     	addps	%xmm4, %xmm7
141fe70fd:     	addss	0x98(%rsp), %xmm4
141fe7106:     	subps	%xmm8, %xmm7
141fe710a:     	movhlps	%xmm8, %xmm8            # xmm8 = xmm8[1,1]
141fe710e:     	subss	%xmm8, %xmm4
141fe7113:     	movlps	%xmm7, 0x30(%rsp)
141fe7118:     	movss	%xmm4, 0x38(%rsp)
141fe711e:     	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
141fe7121:     	movss	0x93e847(%rip), %xmm2   # 0x142925970
141fe7129:     	addps	%xmm1, %xmm2
141fe712c:     	movlps	%xmm2, 0x40(%rsp)
141fe7131:     	movl	$0x3e99999a, 0x48(%rsp) # imm = 0x3E99999A
141fe7139:     	movss	0x93e7c3(%rip), %xmm1   # 0x142925904
141fe7141:     	mulps	%xmm1, %xmm0
141fe7144:     	mulps	%xmm3, %xmm1
141fe7147:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
141fe714b:     	addss	%xmm1, %xmm2
141fe714f:     	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
141fe7152:     	addss	%xmm2, %xmm1
141fe7156:     	addss	%xmm1, %xmm1
141fe715a:     	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
141fe715e:     	mulps	%xmm3, %xmm1
141fe7161:     	addps	%xmm0, %xmm1
141fe7164:     	xorps	%xmm0, %xmm0
141fe7167:     	mulps	%xmm5, %xmm0
141fe716a:     	mulps	0xad59bf(%rip), %xmm3   # 0x142abcb30
141fe7171:     	addps	%xmm0, %xmm3
141fe7174:     	shufps	$0xd6, %xmm3, %xmm3     # xmm3 = xmm3[2,1,1,3]
141fe7178:     	mulps	%xmm6, %xmm3
141fe717b:     	addps	%xmm1, %xmm3
141fe717e:     	movaps	%xmm3, %xmm0
141fe7181:     	unpckhpd	%xmm3, %xmm0            # xmm0 = xmm0[1],xmm3[1]
141fe7185:     	movaps	%xmm3, %xmm1
141fe7188:     	mulss	%xmm3, %xmm1
141fe718c:     	movaps	%xmm0, %xmm2
141fe718f:     	mulss	%xmm0, %xmm2
141fe7193:     	addss	%xmm1, %xmm2
141fe7197:     	xorps	%xmm1, %xmm1
141fe719a:     	sqrtss	%xmm2, %xmm1
141fe719e:     	movss	0x93e75e(%rip), %xmm2   # 0x142925904
141fe71a6:     	divss	%xmm1, %xmm2
141fe71aa:     	mulss	%xmm2, %xmm3
141fe71ae:     	mulss	%xmm0, %xmm2
141fe71b2:     	leaq	0xfe4bff(%rip), %rax    # 0x142fcbdb8
141fe71b9:     	movq	%rax, 0x28(%rsp)
141fe71be:     	movss	%xmm2, 0x20(%rsp)
141fe71c4:     	leaq	0x50(%rsp), %rsi
141fe71c9:     	leaq	0x30(%rsp), %rdx
141fe71ce:     	leaq	0x40(%rsp), %r8
141fe71d3:     	movq	%rsi, %rcx
141fe71d6:     	callq	0x140c98380 <.text+0xc97380>
141fe71db:     	movq	0x28(%r14), %rcx
141fe71df:     	movq	%rsi, %rdx
141fe71e2:     	callq	0x140c98580 <.text+0xc97580>
141fe71e7:     	movzbl	%al, %eax
141fe71ea:     	movl	%ebp, %edx
141fe71ec:     	movaps	0xb0(%rsp), %xmm6
141fe71f4:     	movaps	0xc0(%rsp), %xmm7
141fe71fc:     	movaps	0xd0(%rsp), %xmm8
141fe7205:     	addq	$0xe0, %rsp
141fe720c:     	popq	%rbx
141fe720d:     	popq	%rbp
141fe720e:     	popq	%rdi
141fe720f:     	popq	%rsi
141fe7210:     	popq	%r14
141fe7212:     	retq
141fe7213:     	leaq	0xfe4b56(%rip), %rcx    # 0x142fcbd70
141fe721a:     	leaq	0xfe4b67(%rip), %r8     # 0x142fcbd88
141fe7221:     	movl	$0x13, %edx
141fe7226:     	callq	0x1428d9430 <.text+0x28d8430>
141fe722b:     	movq	%rsi, 0x40(%rsp)
141fe7230:     	leaq	-0xd8a347(%rip), %rax   # 0x14125cef0 <.text+0x125bef0>
141fe7237:     	movq	%rax, 0x48(%rsp)
141fe723c:     	leaq	0xc3aa3d(%rip), %rax    # 0x142c21c80
141fe7243:     	movq	%rax, 0x50(%rsp)
141fe7248:     	movq	$0x1, 0x58(%rsp)
141fe7251:     	movq	$0x0, 0x70(%rsp)
141fe725a:     	leaq	0x40(%rsp), %rax
141fe725f:     	movq	%rax, 0x60(%rsp)
141fe7264:     	movq	$0x1, 0x68(%rsp)
141fe726d:     	leaq	0xc3aa1c(%rip), %rdx    # 0x142c21c90
141fe7274:     	leaq	0x50(%rsp), %rcx
141fe7279:     	callq	0x1428d9390 <.text+0x28d8390>
141fe727e:     	int3
141fe727f:     	int3
141fe7280:     	pushq	%r15
141fe7282:     	pushq	%r14
141fe7284:     	pushq	%r13
141fe7286:     	pushq	%r12
141fe7288:     	pushq	%rsi
141fe7289:     	pushq	%rdi
141fe728a:     	pushq	%rbp
141fe728b:     	pushq	%rbx
141fe728c:     	subq	$0xb8, %rsp
141fe7293:     	movq	%rcx, %rbx
141fe7296:     	movq	0x138(%rsp), %rsi
141fe729e:     	movq	0x148(%rsp), %r12
141fe72a6:     	movq	0x128(%rsp), %r13
141fe72ae:     	movq	0x140(%rsp), %rbp
141fe72b6:     	movq	0x120(%rsp), %r15
141fe72be:     	movl	0x8(%rdx), %eax
141fe72c1:     	movl	%eax, 0x38(%rsp)
141fe72c5:     	movq	(%rdx), %rax
141fe72c8:     	movq	%rax, 0x30(%rsp)
141fe72cd:     	movss	%xmm2, 0x40(%rsp)
141fe72d3:     	movss	%xmm3, 0x44(%rsp)
141fe72d9:     	movl	$0x3dcccccd, 0x48(%rsp) # imm = 0x3DCCCCCD
141fe72e1:     	movaps	0x10(%rdx), %xmm3
141fe72e5:     	movaps	%xmm3, %xmm1
141fe72e8:     	mulps	%xmm3, %xmm1
141fe72eb:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
141fe72ef:     	addss	%xmm1, %xmm0
141fe72f3:     	movaps	%xmm1, %xmm2
141fe72f6:     	unpckhpd	%xmm1, %xmm2            # xmm2 = xmm2[1],xmm1[1]
141fe72fa:     	addss	%xmm0, %xmm2
141fe72fe:     	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
141fe7302:     	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
141fe7306:     	subps	%xmm2, %xmm1
141fe7309:     	movss	0x93e5f3(%rip), %xmm0   # 0x142925904
141fe7311:     	mulps	%xmm0, %xmm1
141fe7314:     	mulps	%xmm3, %xmm0
141fe7317:     	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
141fe731b:     	addss	%xmm0, %xmm2
141fe731f:     	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
141fe7322:     	addss	%xmm2, %xmm0
141fe7326:     	addss	%xmm0, %xmm0
141fe732a:     	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141fe732e:     	mulps	%xmm3, %xmm0
141fe7331:     	addps	%xmm1, %xmm0
141fe7334:     	movaps	%xmm3, %xmm1
141fe7337:     	shufps	$0xc5, %xmm3, %xmm1     # xmm1 = xmm1[1,1],xmm3[0,3]
141fe733b:     	xorps	%xmm2, %xmm2
141fe733e:     	mulps	%xmm1, %xmm2
141fe7341:     	movaps	%xmm3, %xmm1
141fe7344:     	shufps	$0xd6, %xmm3, %xmm1     # xmm1 = xmm1[2,1],xmm3[1,3]
141fe7348:     	mulps	0xa9b0b1(%rip), %xmm1   # 0x142a82400
141fe734f:     	subps	%xmm1, %xmm2
141fe7352:     	addps	%xmm3, %xmm3
141fe7355:     	shufps	$0xff, %xmm3, %xmm3     # xmm3 = xmm3[3,3,3,3]
141fe7359:     	mulps	%xmm2, %xmm3
141fe735c:     	addps	%xmm0, %xmm3
141fe735f:     	movaps	%xmm3, %xmm0
141fe7362:     	unpckhpd	%xmm3, %xmm0            # xmm0 = xmm0[1],xmm3[1]
141fe7366:     	movaps	%xmm3, %xmm1
141fe7369:     	mulss	%xmm3, %xmm1
141fe736d:     	movaps	%xmm0, %xmm2
141fe7370:     	mulss	%xmm0, %xmm2
141fe7374:     	addss	%xmm1, %xmm2
141fe7378:     	xorps	%xmm1, %xmm1
141fe737b:     	sqrtss	%xmm2, %xmm1
141fe737f:     	movss	0x93e57d(%rip), %xmm2   # 0x142925904
141fe7387:     	divss	%xmm1, %xmm2
141fe738b:     	mulss	%xmm2, %xmm3
141fe738f:     	mulss	%xmm0, %xmm2
141fe7393:     	leaq	0xfe49be(%rip), %rax    # 0x142fcbd58
141fe739a:     	movq	%rax, 0x28(%rsp)
141fe739f:     	movss	%xmm2, 0x20(%rsp)
141fe73a5:     	leaq	0x98(%rsp), %r14
141fe73ad:     	leaq	0x30(%rsp), %rdi
141fe73b2:     	leaq	0x40(%rsp), %r8
141fe73b7:     	movq	%r14, %rcx
141fe73ba:     	movq	%rdi, %rdx
141fe73bd:     	callq	0x140c98380 <.text+0xc97380>
141fe73c2:     	movq	0x30(%r15), %rax
141fe73c6:     	movq	0x38(%r15), %rcx
141fe73ca:     	addq	%rax, %rcx
141fe73cd:     	incq	%rcx
141fe73d0:     	movdqa	(%rax), %xmm0
141fe73d4:     	pmovmskb	%xmm0, %edx
141fe73d8:     	notl	%edx
141fe73da:     	movw	%dx, 0x58(%rsp)
141fe73df:     	movq	%rax, 0x40(%rsp)
141fe73e4:     	addq	$0x10, %rax
141fe73e8:     	movq	%rax, 0x48(%rsp)
141fe73ed:     	movq	%rcx, 0x50(%rsp)
141fe73f2:     	movq	0x48(%r15), %rax
141fe73f6:     	movq	%rax, 0x60(%rsp)
141fe73fb:     	addq	$0x4e, %rbx
141fe73ff:     	leaq	0x68(%rsp), %r15
141fe7404:     	leaq	0x130(%rsp), %rax
141fe740c:     	movq	%rax, 0x68(%rsp)
141fe7411:     	movq	%rbx, 0x70(%rsp)
141fe7416:     	movq	%rbp, 0x78(%rsp)
141fe741b:     	movq	%r13, 0x80(%rsp)
141fe7423:     	movq	%r12, 0x88(%rsp)
141fe742b:     	movq	%r14, 0x90(%rsp)
141fe7433:     	movq	%r15, 0x30(%rsp)
141fe7438:     	jmp	0x141fe7489 <.text+0x1fe6489>
141fe743a:     	nopw	(%rax,%rax)
141fe7440:     	leal	-0x1(%rdx), %r8d
141fe7444:     	andl	%edx, %r8d
141fe7447:     	movw	%r8w, 0x58(%rsp)
141fe744d:     	decq	%rcx
141fe7450:     	movq	%rcx, 0x60(%rsp)
141fe7455:     	testq	%rax, %rax
141fe7458:     	je	0x141fe7542 <.text+0x1fe6542>
141fe745e:     	tzcntl	%edx, %ecx
141fe7462:     	negq	%rcx
141fe7465:     	leaq	(%rcx,%rcx,4), %rcx
141fe7469:     	shlq	$0x4, %rcx
141fe746d:     	leaq	(%rax,%rcx), %rdx
141fe7471:     	addq	$-0x50, %rdx
141fe7475:     	leaq	(%rax,%rcx), %r8
141fe7479:     	addq	$-0x48, %r8
141fe747d:     	movq	%rdi, %rcx
141fe7480:     	callq	0x141fe6f20 <.text+0x1fe5f20>
141fe7485:     	testl	%eax, %eax
141fe7487:     	jne	0x141fe7500 <.text+0x1fe6500>
141fe7489:     	movq	0x60(%rsp), %rcx
141fe748e:     	testq	%rcx, %rcx
141fe7491:     	je	0x141fe7542 <.text+0x1fe6542>
141fe7497:     	movzwl	0x58(%rsp), %edx
141fe749c:     	movq	0x40(%rsp), %rax
141fe74a1:     	testw	%dx, %dx
141fe74a4:     	jne	0x141fe7440 <.text+0x1fe6440>
141fe74a6:     	movq	0x48(%rsp), %r8
141fe74ab:     	nopl	(%rax,%rax)
141fe74b0:     	movdqa	(%r8), %xmm0
141fe74b5:     	pmovmskb	%xmm0, %edx
141fe74b9:     	addq	$-0x500, %rax           # imm = 0xFB00
141fe74bf:     	addq	$0x10, %r8
141fe74c3:     	cmpl	$0xffff, %edx           # imm = 0xFFFF
141fe74c9:     	je	0x141fe74b0 <.text+0x1fe64b0>
141fe74cb:     	movq	%r8, 0x48(%rsp)
141fe74d0:     	movq	%rax, 0x40(%rsp)
141fe74d5:     	movl	$0xfffffffe, %r8d       # imm = 0xFFFFFFFE
141fe74db:     	subl	%edx, %r8d
141fe74de:     	notl	%edx
141fe74e0:     	movl	%edx, %r9d
141fe74e3:     	andl	%r8d, %r9d
141fe74e6:     	movw	%r9w, 0x58(%rsp)
141fe74ec:     	decq	%rcx
141fe74ef:     	movq	%rcx, 0x60(%rsp)
141fe74f4:     	jmp	0x141fe745e <.text+0x1fe645e>
141fe74f9:     	nopl	(%rax)
141fe7500:     	movq	0x10(%rsi), %rbx
141fe7504:     	cmpq	(%rsi), %rbx
141fe7507:     	je	0x141fe751c <.text+0x1fe651c>
141fe7509:     	movq	0x8(%rsi), %rax
141fe750d:     	movl	%edx, (%rax,%rbx,4)
141fe7510:     	incq	%rbx
141fe7513:     	movq	%rbx, 0x10(%rsi)
141fe7517:     	jmp	0x141fe7433 <.text+0x1fe6433>
141fe751c:     	movq	$0x4, 0x20(%rsp)
141fe7525:     	movl	$0x1, %r8d
141fe752b:     	movl	$0x4, %r9d
141fe7531:     	movq	%rsi, %rcx
141fe7534:     	movl	%edx, %ebp
141fe7536:     	movq	%rbx, %rdx
141fe7539:     	callq	0x1428a7f20 <.text+0x28a6f20>
141fe753e:     	movl	%ebp, %edx
141fe7540:     	jmp	0x141fe7509 <.text+0x1fe6509>
141fe7542:     	addq	$0xb8, %rsp
141fe7549:     	popq	%rbx
141fe754a:     	popq	%rbp
141fe754b:     	popq	%rdi
141fe754c:     	popq	%rsi
141fe754d:     	popq	%r12
141fe754f:     	popq	%r13
141fe7551:     	popq	%r14
141fe7553:     	popq	%r15
141fe7555:     	retq
141fe7556:     	int3
141fe7557:     	int3
141fe7558:     	int3
141fe7559:     	int3
141fe755a:     	int3
141fe755b:     	int3
141fe755c:     	int3
141fe755d:     	int3
141fe755e:     	int3
141fe755f:     	int3
