
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a65c30:     	pushq	%rbp
140a65c31:     	pushq	%rsi
140a65c32:     	pushq	%rdi
140a65c33:     	subq	$0xd0, %rsp
140a65c3a:     	leaq	0x80(%rsp), %rbp
140a65c42:     	movq	$-0x2, 0x48(%rbp)
140a65c4a:     	movq	%r8, %rax
140a65c4d:     	movq	%rdx, %rdi
140a65c50:     	movq	%rcx, %rsi
140a65c53:     	movq	$0x3, 0x20(%rsp)
140a65c5c:     	leaq	0x206e9d5(%rip), %r8    # 0x142ad4638
140a65c63:     	leaq	-0x10(%rbp), %rcx
140a65c67:     	movl	$0x1a, %r9d
140a65c6d:     	movq	%rax, %rdx
140a65c70:     	callq	0x1413132c0 <.text+0x13122c0>
140a65c75:     	movzbl	-0x10(%rbp), %eax
140a65c79:     	cmpb	$0x32, %al
140a65c7b:     	jne	0x140a65d47 <.text+0xa64d47>
140a65c81:     	movq	0x8(%rbp), %rax
140a65c85:     	movups	-0x8(%rbp), %xmm0
140a65c89:     	movaps	%xmm0, -0x30(%rbp)
140a65c8d:     	movq	%rax, -0x20(%rbp)
140a65c91:     	movb	$0x1, 0x47(%rbp)
140a65c95:     	movq	%rdi, 0x20(%rsp)
140a65c9a:     	leaq	0x206e9b1(%rip), %r8    # 0x142ad4652
140a65ca1:     	leaq	-0x10(%rbp), %rcx
140a65ca5:     	leaq	-0x30(%rbp), %rdx
140a65ca9:     	movl	$0xd, %r9d
140a65caf:     	callq	0x141310810 <.text+0x130f810>
140a65cb4:     	movzbl	-0x10(%rbp), %eax
140a65cb8:     	cmpb	$0x32, %al
140a65cba:     	jne	0x140a65d83 <.text+0xa64d83>
140a65cc0:     	leaq	0x8(%rdi), %rax
140a65cc4:     	movb	$0x1, 0x47(%rbp)
140a65cc8:     	movq	%rax, 0x20(%rsp)
140a65ccd:     	leaq	0x206e98b(%rip), %r8    # 0x142ad465f
140a65cd4:     	leaq	-0x10(%rbp), %rcx
140a65cd8:     	leaq	-0x30(%rbp), %rdx
140a65cdc:     	movl	$0x11, %r9d
140a65ce2:     	callq	0x141310810 <.text+0x130f810>
140a65ce7:     	movzbl	-0x10(%rbp), %eax
140a65ceb:     	cmpb	$0x32, %al
140a65ced:     	jne	0x140a65d83 <.text+0xa64d83>
140a65cf3:     	addq	$0x10, %rdi
140a65cf7:     	movb	$0x1, 0x47(%rbp)
140a65cfb:     	movq	%rdi, 0x20(%rsp)
140a65d00:     	leaq	0x206e969(%rip), %r8    # 0x142ad4670
140a65d07:     	leaq	-0x10(%rbp), %rcx
140a65d0b:     	leaq	-0x30(%rbp), %rdx
140a65d0f:     	movl	$0xb, %r9d
140a65d15:     	callq	0x141310eb0 <.text+0x130feb0>
140a65d1a:     	movzbl	-0x10(%rbp), %eax
140a65d1e:     	cmpb	$0x32, %al
140a65d20:     	jne	0x140a65d83 <.text+0xa64d83>
140a65d22:     	movq	-0x20(%rbp), %rax
140a65d26:     	movq	%rax, (%rbp)
140a65d2a:     	movaps	-0x30(%rbp), %xmm0
140a65d2e:     	movaps	%xmm0, -0x10(%rbp)
140a65d32:     	movb	$0x0, 0x47(%rbp)
140a65d36:     	leaq	-0x10(%rbp), %rdx
140a65d3a:     	movq	%rsi, %rcx
140a65d3d:     	callq	0x141312a40 <.text+0x1311a40>
140a65d42:     	jmp	0x140a65dcc <.text+0xa64dcc>
140a65d47:     	movups	-0xf(%rbp), %xmm0
140a65d4b:     	movups	(%rbp), %xmm1
140a65d4f:     	movups	0x10(%rbp), %xmm2
140a65d53:     	movups	0x20(%rbp), %xmm3
140a65d57:     	movups	%xmm1, -0x41(%rbp)
140a65d5b:     	movaps	%xmm0, -0x50(%rbp)
140a65d5f:     	movq	0x30(%rbp), %rcx
140a65d63:     	movq	%rcx, 0x40(%rsi)
140a65d67:     	movups	%xmm3, 0x30(%rsi)
140a65d6b:     	movups	%xmm2, 0x20(%rsi)
140a65d6f:     	movups	-0x41(%rbp), %xmm0
140a65d73:     	movups	%xmm0, 0x10(%rsi)
140a65d77:     	movaps	-0x50(%rbp), %xmm0
140a65d7b:     	movups	%xmm0, 0x1(%rsi)
140a65d7f:     	movb	%al, (%rsi)
140a65d81:     	jmp	0x140a65dcc <.text+0xa64dcc>
140a65d83:     	movq	0x30(%rbp), %rcx
140a65d87:     	movq	%rcx, 0x40(%rsi)
140a65d8b:     	movups	-0xf(%rbp), %xmm0
140a65d8f:     	movups	0x1(%rbp), %xmm1
140a65d93:     	movups	0x11(%rbp), %xmm2
140a65d97:     	movups	0x21(%rbp), %xmm3
140a65d9b:     	movups	%xmm3, 0x31(%rsi)
140a65d9f:     	movups	%xmm2, 0x21(%rsi)
140a65da3:     	movups	%xmm1, 0x11(%rsi)
140a65da7:     	movups	%xmm0, 0x1(%rsi)
140a65dab:     	movb	%al, (%rsi)
140a65dad:     	movq	-0x28(%rbp), %rax
140a65db1:     	cmpl	$0x1, (%rax)
140a65db4:     	jne	0x140a65dcc <.text+0xa64dcc>
140a65db6:     	movq	0x8(%rax), %rcx
140a65dba:     	incq	%rcx
140a65dbd:     	movq	$-0x1, %rdx
140a65dc4:     	cmovneq	%rcx, %rdx
140a65dc8:     	movq	%rdx, 0x8(%rax)
140a65dcc:     	movq	%rsi, %rax
140a65dcf:     	addq	$0xd0, %rsp
140a65dd6:     	popq	%rdi
140a65dd7:     	popq	%rsi
140a65dd8:     	popq	%rbp
140a65dd9:     	retq
140a65dda:     	nopw	(%rax,%rax)
140a65de0:     	movq	%rdx, 0x10(%rsp)
140a65de5:     	pushq	%rbp
140a65de6:     	pushq	%rsi
140a65de7:     	pushq	%rdi
140a65de8:     	subq	$0x30, %rsp
140a65dec:     	leaq	0x80(%rdx), %rbp
140a65df3:     	cmpb	$0x0, 0x47(%rbp)
140a65df7:     	je	0x140a65e18 <.text+0xa64e18>
140a65df9:     	movq	-0x28(%rbp), %rax
140a65dfd:     	cmpl	$0x1, (%rax)
140a65e00:     	jne	0x140a65e18 <.text+0xa64e18>
140a65e02:     	movq	0x8(%rax), %rcx
140a65e06:     	incq	%rcx
140a65e09:     	movq	$-0x1, %rdx
140a65e10:     	cmovneq	%rcx, %rdx
140a65e14:     	movq	%rdx, 0x8(%rax)
140a65e18:     	addq	$0x30, %rsp
140a65e1c:     	popq	%rdi
140a65e1d:     	popq	%rsi
140a65e1e:     	popq	%rbp
140a65e1f:     	retq
140a65e20:     	pushq	%rbp
140a65e21:     	pushq	%rsi
140a65e22:     	pushq	%rdi
140a65e23:     	subq	$0xd0, %rsp
140a65e2a:     	leaq	0x80(%rsp), %rbp
140a65e32:     	movq	$-0x2, 0x48(%rbp)
140a65e3a:     	movq	%r8, %rax
140a65e3d:     	movq	%rdx, %rdi
140a65e40:     	movq	%rcx, %rsi
140a65e43:     	movq	$0x2, 0x20(%rsp)
140a65e4c:     	leaq	0x206e828(%rip), %r8    # 0x142ad467b
140a65e53:     	leaq	-0x10(%rbp), %rcx
140a65e57:     	movl	$0xa, %r9d
140a65e5d:     	movq	%rax, %rdx
140a65e60:     	callq	0x1413132c0 <.text+0x13122c0>
140a65e65:     	movzbl	-0x10(%rbp), %eax
140a65e69:     	cmpb	$0x32, %al
140a65e6b:     	jne	0x140a65f04 <.text+0xa64f04>
140a65e71:     	movq	0x8(%rbp), %rax
140a65e75:     	movups	-0x8(%rbp), %xmm0
140a65e79:     	movaps	%xmm0, -0x30(%rbp)
140a65e7d:     	movq	%rax, -0x20(%rbp)
140a65e81:     	movb	$0x1, 0x47(%rbp)
140a65e85:     	movq	%rdi, 0x20(%rsp)
140a65e8a:     	leaq	0x206e7f4(%rip), %r8    # 0x142ad4685
140a65e91:     	leaq	-0x10(%rbp), %rcx
140a65e95:     	leaq	-0x30(%rbp), %rdx
140a65e99:     	movl	$0x7, %r9d
140a65e9f:     	callq	0x141310eb0 <.text+0x130feb0>
140a65ea4:     	movzbl	-0x10(%rbp), %eax
140a65ea8:     	cmpb	$0x32, %al
140a65eaa:     	jne	0x140a65f40 <.text+0xa64f40>
140a65eb0:     	addq	$0x4, %rdi
140a65eb4:     	movb	$0x1, 0x47(%rbp)
140a65eb8:     	movq	%rdi, 0x20(%rsp)
140a65ebd:     	leaq	0x206e7c8(%rip), %r8    # 0x142ad468c
140a65ec4:     	leaq	-0x10(%rbp), %rcx
140a65ec8:     	leaq	-0x30(%rbp), %rdx
140a65ecc:     	movl	$0x10, %r9d
140a65ed2:     	callq	0x141310eb0 <.text+0x130feb0>
140a65ed7:     	movzbl	-0x10(%rbp), %eax
140a65edb:     	cmpb	$0x32, %al
140a65edd:     	jne	0x140a65f40 <.text+0xa64f40>
140a65edf:     	movq	-0x20(%rbp), %rax
140a65ee3:     	movq	%rax, (%rbp)
140a65ee7:     	movaps	-0x30(%rbp), %xmm0
140a65eeb:     	movaps	%xmm0, -0x10(%rbp)
140a65eef:     	movb	$0x0, 0x47(%rbp)
140a65ef3:     	leaq	-0x10(%rbp), %rdx
140a65ef7:     	movq	%rsi, %rcx
140a65efa:     	callq	0x141312a40 <.text+0x1311a40>
140a65eff:     	jmp	0x140a65f89 <.text+0xa64f89>
140a65f04:     	movups	-0xf(%rbp), %xmm0
140a65f08:     	movups	(%rbp), %xmm1
140a65f0c:     	movups	0x10(%rbp), %xmm2
140a65f10:     	movups	0x20(%rbp), %xmm3
140a65f14:     	movups	%xmm1, -0x41(%rbp)
140a65f18:     	movaps	%xmm0, -0x50(%rbp)
140a65f1c:     	movq	0x30(%rbp), %rcx
140a65f20:     	movq	%rcx, 0x40(%rsi)
140a65f24:     	movups	%xmm3, 0x30(%rsi)
140a65f28:     	movups	%xmm2, 0x20(%rsi)
140a65f2c:     	movups	-0x41(%rbp), %xmm0
140a65f30:     	movups	%xmm0, 0x10(%rsi)
140a65f34:     	movaps	-0x50(%rbp), %xmm0
140a65f38:     	movups	%xmm0, 0x1(%rsi)
140a65f3c:     	movb	%al, (%rsi)
140a65f3e:     	jmp	0x140a65f89 <.text+0xa64f89>
140a65f40:     	movq	0x30(%rbp), %rcx
140a65f44:     	movq	%rcx, 0x40(%rsi)
140a65f48:     	movups	-0xf(%rbp), %xmm0
140a65f4c:     	movups	0x1(%rbp), %xmm1
140a65f50:     	movups	0x11(%rbp), %xmm2
140a65f54:     	movups	0x21(%rbp), %xmm3
140a65f58:     	movups	%xmm3, 0x31(%rsi)
140a65f5c:     	movups	%xmm2, 0x21(%rsi)
140a65f60:     	movups	%xmm1, 0x11(%rsi)
140a65f64:     	movups	%xmm0, 0x1(%rsi)
140a65f68:     	movb	%al, (%rsi)
140a65f6a:     	movq	-0x28(%rbp), %rax
140a65f6e:     	cmpl	$0x1, (%rax)
140a65f71:     	jne	0x140a65f89 <.text+0xa64f89>
140a65f73:     	movq	0x8(%rax), %rcx
140a65f77:     	incq	%rcx
140a65f7a:     	movq	$-0x1, %rdx
140a65f81:     	cmovneq	%rcx, %rdx
140a65f85:     	movq	%rdx, 0x8(%rax)
140a65f89:     	movq	%rsi, %rax
140a65f8c:     	addq	$0xd0, %rsp
140a65f93:     	popq	%rdi
140a65f94:     	popq	%rsi
140a65f95:     	popq	%rbp
140a65f96:     	retq
140a65f97:     	nopw	(%rax,%rax)
140a65fa0:     	movq	%rdx, 0x10(%rsp)
140a65fa5:     	pushq	%rbp
140a65fa6:     	pushq	%rsi
140a65fa7:     	pushq	%rdi
140a65fa8:     	subq	$0x30, %rsp
140a65fac:     	leaq	0x80(%rdx), %rbp
140a65fb3:     	cmpb	$0x0, 0x47(%rbp)
140a65fb7:     	je	0x140a65fd8 <.text+0xa64fd8>
140a65fb9:     	movq	-0x28(%rbp), %rax
140a65fbd:     	cmpl	$0x1, (%rax)
140a65fc0:     	jne	0x140a65fd8 <.text+0xa64fd8>
140a65fc2:     	movq	0x8(%rax), %rcx
140a65fc6:     	incq	%rcx
140a65fc9:     	movq	$-0x1, %rdx
140a65fd0:     	cmovneq	%rcx, %rdx
140a65fd4:     	movq	%rdx, 0x8(%rax)
140a65fd8:     	addq	$0x30, %rsp
140a65fdc:     	popq	%rdi
140a65fdd:     	popq	%rsi
140a65fde:     	popq	%rbp
140a65fdf:     	retq
