
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140504b40:     	pushq	%rbp
140504b41:     	pushq	%r15
140504b43:     	pushq	%r14
140504b45:     	pushq	%rsi
140504b46:     	pushq	%rdi
140504b47:     	pushq	%rbx
140504b48:     	subq	$0xc8, %rsp
140504b4f:     	leaq	0x80(%rsp), %rbp
140504b57:     	movq	$-0x2, 0x40(%rbp)
140504b5f:     	movq	%r8, %rdi
140504b62:     	movq	%rdx, %rsi
140504b65:     	movq	%rcx, %rbx
140504b68:     	leaq	0x24e9365(%rip), %rcx   # 0x1429eded4
140504b6f:     	movl	$0xd, %edx
140504b74:     	callq	0x140b9d480 <.text+0xb9c480>
140504b79:     	movq	%rdx, %r8
140504b7c:     	leaq	-0x40(%rbp), %rcx
140504b80:     	movq	%rax, %rdx
140504b83:     	callq	0x140d7ad40 <.text+0xd79d40>
140504b88:     	movq	-0x38(%rbp), %r8
140504b8c:     	movq	-0x30(%rbp), %r9
140504b90:     	leaq	0x20(%rbp), %rcx
140504b94:     	movq	%rdi, %rdx
140504b97:     	movq	%r8, 0x38(%rbp)
140504b9b:     	callq	0x14068bfc0 <.text+0x68afc0>
140504ba0:     	leaq	0x20(%rbp), %rcx
140504ba4:     	callq	0x14067dd80 <.text+0x67cd80>
140504ba9:     	movq	%rax, %rcx
140504bac:     	callq	0x140681e50 <.text+0x680e50>
140504bb1:     	movq	%rax, %rcx
140504bb4:     	callq	0x140683c50 <.text+0x682c50>
140504bb9:     	movq	0x30(%rbp), %rax
140504bbd:     	movq	%rax, -0x50(%rbp)
140504bc1:     	movups	0x20(%rbp), %xmm0
140504bc5:     	movaps	%xmm0, -0x60(%rbp)
140504bc9:     	leaq	0x20(%rbp), %rcx
140504bcd:     	leaq	-0x60(%rbp), %rdx
140504bd1:     	callq	0x14067df20 <.text+0x67cf20>
140504bd6:     	leaq	0x20(%rbp), %rcx
140504bda:     	callq	0x140681d50 <.text+0x680d50>
140504bdf:     	movq	%rax, %rcx
140504be2:     	callq	0x140683c80 <.text+0x682c80>
140504be7:     	movl	%eax, %r14d
140504bea:     	movq	-0x40(%rbp), %rdx
140504bee:     	testq	%rdx, %rdx
140504bf1:     	je	0x140504c02 <.text+0x503c02>
140504bf3:     	movl	$0x1, %r8d
140504bf9:     	movq	0x38(%rbp), %rcx
140504bfd:     	callq	0x140613c20 <.text+0x612c20>
140504c02:     	testb	%r14b, %r14b
140504c05:     	je	0x140504c6d <.text+0x503c6d>
140504c07:     	movl	0x1c(%rbx), %eax
140504c0a:     	movq	0x10(%rbx), %rcx
140504c0e:     	movl	%eax, (%rcx)
140504c10:     	movq	(%rbx), %rbx
140504c13:     	movq	0x30(%rbx), %r14
140504c17:     	movq	0x40(%rbx), %r15
140504c1b:     	cmpq	0x20(%rbx), %r14
140504c1f:     	jne	0x140504c31 <.text+0x503c31>
140504c21:     	leaq	0x20(%rbx), %rcx
140504c25:     	leaq	0x24588a4(%rip), %rdx   # 0x14295d4d0
140504c2c:     	callq	0x140562630 <.text+0x561630>
140504c31:     	movq	0x28(%rbx), %rax
140504c35:     	movq	%r14, %rcx
140504c38:     	shlq	$0x4, %rcx
140504c3c:     	movq	%r15, (%rax,%rcx)
140504c40:     	movb	$0xb, 0x8(%rax,%rcx)
140504c45:     	incq	%r14
140504c48:     	movq	%r14, 0x30(%rbx)
140504c4c:     	incq	0x40(%rbx)
140504c50:     	movl	0x1c(%rsi), %eax
140504c53:     	movq	0x10(%rsi), %rcx
140504c57:     	movl	%eax, (%rcx)
140504c59:     	movq	(%rsi), %rcx
140504c5c:     	movq	$0x2, -0x40(%rbp)
140504c64:     	leaq	-0x40(%rbp), %rdx
140504c68:     	callq	0x14093fad0 <.text+0x93ead0>
140504c6d:     	leaq	0x24e926d(%rip), %rcx   # 0x1429edee1
140504c74:     	movl	$0x8, %edx
140504c79:     	callq	0x140b9d480 <.text+0xb9c480>
140504c7e:     	movq	%rdx, %r8
140504c81:     	leaq	-0x40(%rbp), %rcx
140504c85:     	movq	%rax, %rdx
140504c88:     	callq	0x140d7ad40 <.text+0xd79d40>
140504c8d:     	movq	-0x38(%rbp), %r8
140504c91:     	movq	-0x30(%rbp), %r9
140504c95:     	leaq	0x20(%rbp), %rcx
140504c99:     	movq	%rdi, %rdx
140504c9c:     	movq	%r8, 0x38(%rbp)
140504ca0:     	callq	0x14068bfc0 <.text+0x68afc0>
140504ca5:     	leaq	-0x60(%rbp), %rcx
140504ca9:     	leaq	0x20(%rbp), %rdx
140504cad:     	callq	0x14067df20 <.text+0x67cf20>
140504cb2:     	leaq	0x20(%rbp), %rcx
140504cb6:     	leaq	-0x60(%rbp), %rdx
140504cba:     	movb	$0x6, %r8b
140504cbd:     	callq	0x14067e5b0 <.text+0x67d5b0>
140504cc2:     	leaq	0x20(%rbp), %rcx
140504cc6:     	callq	0x140681d50 <.text+0x680d50>
140504ccb:     	movq	%rax, %rcx
140504cce:     	callq	0x140683c80 <.text+0x682c80>
140504cd3:     	movl	%eax, %ebx
140504cd5:     	movq	-0x40(%rbp), %rdx
140504cd9:     	testq	%rdx, %rdx
140504cdc:     	je	0x140504ced <.text+0x503ced>
140504cde:     	movl	$0x1, %r8d
140504ce4:     	movq	0x38(%rbp), %rcx
140504ce8:     	callq	0x140613c20 <.text+0x612c20>
140504ced:     	testb	%bl, %bl
140504cef:     	je	0x140504d0e <.text+0x503d0e>
140504cf1:     	movl	0x1c(%rsi), %eax
140504cf4:     	movq	0x10(%rsi), %rcx
140504cf8:     	movl	%eax, (%rcx)
140504cfa:     	movq	(%rsi), %rcx
140504cfd:     	movq	$0x5, -0x40(%rbp)
140504d05:     	leaq	-0x40(%rbp), %rdx
140504d09:     	callq	0x14093fad0 <.text+0x93ead0>
140504d0e:     	leaq	0x24e91d4(%rip), %rcx   # 0x1429edee9
140504d15:     	movl	$0x9, %edx
140504d1a:     	callq	0x140b9d480 <.text+0xb9c480>
140504d1f:     	movq	%rdx, %r8
140504d22:     	leaq	-0x40(%rbp), %rcx
140504d26:     	movq	%rax, %rdx
140504d29:     	callq	0x140d7ad40 <.text+0xd79d40>
140504d2e:     	movq	-0x38(%rbp), %r8
140504d32:     	movq	-0x30(%rbp), %r9
140504d36:     	leaq	0x20(%rbp), %rcx
140504d3a:     	movq	%rdi, %rdx
140504d3d:     	movq	%r8, 0x38(%rbp)
140504d41:     	callq	0x14068bfc0 <.text+0x68afc0>
140504d46:     	leaq	-0x60(%rbp), %rcx
140504d4a:     	leaq	0x20(%rbp), %rdx
140504d4e:     	callq	0x14067df20 <.text+0x67cf20>
140504d53:     	leaq	0x20(%rbp), %rcx
140504d57:     	leaq	-0x60(%rbp), %rdx
140504d5b:     	movb	$0x6, %r8b
140504d5e:     	callq	0x14067e5b0 <.text+0x67d5b0>
140504d63:     	leaq	0x20(%rbp), %rcx
140504d67:     	callq	0x140681d50 <.text+0x680d50>
140504d6c:     	movq	%rax, %rcx
140504d6f:     	callq	0x140683c80 <.text+0x682c80>
140504d74:     	movl	%eax, %ebx
140504d76:     	movq	-0x40(%rbp), %rdx
140504d7a:     	testq	%rdx, %rdx
140504d7d:     	je	0x140504d8e <.text+0x503d8e>
140504d7f:     	movl	$0x1, %r8d
140504d85:     	movq	0x38(%rbp), %rcx
140504d89:     	callq	0x140613c20 <.text+0x612c20>
140504d8e:     	testb	%bl, %bl
140504d90:     	je	0x140504e26 <.text+0x503e26>
140504d96:     	movl	0x1c(%rsi), %eax
140504d99:     	movq	0x10(%rsi), %rcx
140504d9d:     	movl	%eax, (%rcx)
140504d9f:     	movq	(%rsi), %rsi
140504da2:     	movzbl	0x312fe60(%rip), %eax   # 0x143634c09
140504da9:     	movl	$0x48, %ecx
140504dae:     	movl	$0x8, %edx
140504db3:     	callq	0x140613c10 <.text+0x612c10>
140504db8:     	testq	%rax, %rax
140504dbb:     	je	0x140504e37 <.text+0x503e37>
140504dbd:     	xorps	%xmm0, %xmm0
140504dc0:     	movups	%xmm0, -0x23(%rbp)
140504dc4:     	movups	%xmm0, -0x2c(%rbp)
140504dc8:     	movups	%xmm0, -0x3c(%rbp)
140504dcc:     	movq	$0x1, (%rax)
140504dd3:     	movq	$0x1, 0x8(%rax)
140504ddb:     	movb	$0x0, 0x10(%rax)
140504ddf:     	movl	$0x0, 0x14(%rax)
140504de6:     	movups	-0x40(%rbp), %xmm0
140504dea:     	movups	-0x30(%rbp), %xmm1
140504dee:     	movups	-0x23(%rbp), %xmm2
140504df2:     	movups	%xmm2, 0x35(%rax)
140504df6:     	movups	%xmm1, 0x28(%rax)
140504dfa:     	movups	%xmm0, 0x18(%rax)
140504dfe:     	movq	$0x0, -0x40(%rbp)
140504e06:     	movq	$0x0, -0x30(%rbp)
140504e0e:     	movq	%rax, -0x10(%rbp)
140504e12:     	movb	$0x0, -0x8(%rbp)
140504e16:     	movb	$0x0, 0x9(%rbp)
140504e1a:     	leaq	-0x40(%rbp), %rdx
140504e1e:     	movq	%rsi, %rcx
140504e21:     	callq	0x14093fad0 <.text+0x93ead0>
140504e26:     	nop
140504e27:     	addq	$0xc8, %rsp
140504e2e:     	popq	%rbx
140504e2f:     	popq	%rdi
140504e30:     	popq	%rsi
140504e31:     	popq	%r14
140504e33:     	popq	%r15
140504e35:     	popq	%rbp
140504e36:     	retq
140504e37:     	movl	$0x8, %ecx
140504e3c:     	movl	$0x48, %edx
140504e41:     	callq	0x1428d9000 <.text+0x28d8000>
140504e46:     	int3
140504e47:     	nopw	(%rax,%rax)
140504e50:     	movq	%rdx, 0x10(%rsp)
140504e55:     	pushq	%rbp
140504e56:     	pushq	%r15
140504e58:     	pushq	%r14
140504e5a:     	pushq	%rsi
140504e5b:     	pushq	%rdi
140504e5c:     	pushq	%rbx
140504e5d:     	subq	$0x28, %rsp
140504e61:     	leaq	0x80(%rdx), %rbp
140504e68:     	movq	-0x40(%rbp), %rdx
140504e6c:     	testq	%rdx, %rdx
140504e6f:     	je	0x140504e80 <.text+0x503e80>
140504e71:     	movl	$0x1, %r8d
140504e77:     	movq	0x38(%rbp), %rcx
140504e7b:     	callq	0x140613c20 <.text+0x612c20>
140504e80:     	nop
140504e81:     	addq	$0x28, %rsp
140504e85:     	popq	%rbx
140504e86:     	popq	%rdi
140504e87:     	popq	%rsi
140504e88:     	popq	%r14
140504e8a:     	popq	%r15
140504e8c:     	popq	%rbp
140504e8d:     	retq
140504e8e:     	nop
140504e90:     	movq	%rdx, 0x10(%rsp)
140504e95:     	pushq	%rbp
140504e96:     	pushq	%r15
140504e98:     	pushq	%r14
140504e9a:     	pushq	%rsi
140504e9b:     	pushq	%rdi
140504e9c:     	pushq	%rbx
140504e9d:     	subq	$0x28, %rsp
140504ea1:     	leaq	0x80(%rdx), %rbp
140504ea8:     	movq	-0x40(%rbp), %rdx
140504eac:     	testq	%rdx, %rdx
140504eaf:     	je	0x140504ec0 <.text+0x503ec0>
140504eb1:     	movl	$0x1, %r8d
140504eb7:     	movq	0x38(%rbp), %rcx
140504ebb:     	callq	0x140613c20 <.text+0x612c20>
140504ec0:     	nop
140504ec1:     	addq	$0x28, %rsp
140504ec5:     	popq	%rbx
140504ec6:     	popq	%rdi
140504ec7:     	popq	%rsi
140504ec8:     	popq	%r14
140504eca:     	popq	%r15
140504ecc:     	popq	%rbp
140504ecd:     	retq
140504ece:     	nop
140504ed0:     	movq	%rdx, 0x10(%rsp)
140504ed5:     	pushq	%rbp
140504ed6:     	pushq	%r15
140504ed8:     	pushq	%r14
140504eda:     	pushq	%rsi
140504edb:     	pushq	%rdi
140504edc:     	pushq	%rbx
140504edd:     	subq	$0x28, %rsp
140504ee1:     	leaq	0x80(%rdx), %rbp
140504ee8:     	movq	-0x40(%rbp), %rdx
140504eec:     	testq	%rdx, %rdx
140504eef:     	je	0x140504f00 <.text+0x503f00>
140504ef1:     	movl	$0x1, %r8d
140504ef7:     	movq	0x38(%rbp), %rcx
140504efb:     	callq	0x140613c20 <.text+0x612c20>
140504f00:     	nop
140504f01:     	addq	$0x28, %rsp
140504f05:     	popq	%rbx
140504f06:     	popq	%rdi
140504f07:     	popq	%rsi
140504f08:     	popq	%r14
140504f0a:     	popq	%r15
140504f0c:     	popq	%rbp
140504f0d:     	retq
