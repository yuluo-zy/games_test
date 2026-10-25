
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1422979b0:     	pushq	%r15
1422979b2:     	pushq	%r14
1422979b4:     	pushq	%r13
1422979b6:     	pushq	%r12
1422979b8:     	pushq	%rsi
1422979b9:     	pushq	%rdi
1422979ba:     	pushq	%rbp
1422979bb:     	pushq	%rbx
1422979bc:     	subq	$0x208, %rsp            # imm = 0x208
1422979c3:     	movaps	%xmm14, 0x1f0(%rsp)
1422979cc:     	movaps	%xmm13, 0x1e0(%rsp)
1422979d5:     	movaps	%xmm12, 0x1d0(%rsp)
1422979de:     	movaps	%xmm11, 0x1c0(%rsp)
1422979e7:     	movaps	%xmm10, 0x1b0(%rsp)
1422979f0:     	movaps	%xmm9, 0x1a0(%rsp)
1422979f9:     	movaps	%xmm8, 0x190(%rsp)
142297a02:     	movaps	%xmm7, 0x180(%rsp)
142297a0a:     	movaps	%xmm6, 0x170(%rsp)
142297a12:     	movq	%rdx, %rbx
142297a15:     	movq	(%rcx), %r15
142297a18:     	movq	0x8(%rcx), %rdi
142297a1c:     	movl	0x10(%rcx), %r9d
142297a20:     	movl	0x14(%rcx), %eax
142297a23:     	leaq	0x1a0(%rdi), %rsi
142297a2a:     	leaq	0xf8(%rdi), %r14
142297a31:     	leaq	0xb8(%rsp), %rcx
142297a39:     	movl	%eax, 0x20(%rsp)
142297a3d:     	movq	%rdi, %rdx
142297a40:     	movq	%r15, %r8
142297a43:     	callq	0x1425f25a0 <.text+0x25f15a0>
142297a48:     	movq	%rdi, 0x98(%rsp)
142297a50:     	movq	%rsi, 0xa0(%rsp)
142297a58:     	movq	%r14, 0xa8(%rsp)
142297a60:     	movq	%r15, 0x38(%rsp)
142297a65:     	movq	%r15, 0xb0(%rsp)
142297a6d:     	movq	0xb8(%rsp), %r15
142297a75:     	movq	0xc0(%rsp), %r12
142297a7d:     	movq	0xd8(%rsp), %rax
142297a85:     	movq	%rax, 0x88(%rsp)
142297a8d:     	movq	0xe8(%rsp), %r14
142297a95:     	movq	0xf0(%rsp), %rax
142297a9d:     	movq	%rax, 0x78(%rsp)
142297aa2:     	movq	0xf8(%rsp), %rax
142297aaa:     	movq	%rax, 0x68(%rsp)
142297aaf:     	movq	0x108(%rsp), %rax
142297ab7:     	movq	%rax, 0x60(%rsp)
142297abc:     	movl	0x114(%rsp), %eax
142297ac3:     	movl	%eax, 0x34(%rsp)
142297ac7:     	movq	0x118(%rsp), %rax
142297acf:     	movq	%rax, 0x58(%rsp)
142297ad4:     	movq	0x128(%rsp), %rax
142297adc:     	movq	%rax, 0x50(%rsp)
142297ae1:     	movl	0x134(%rsp), %eax
142297ae8:     	movl	%eax, 0x30(%rsp)
142297aec:     	movq	0x138(%rsp), %rax
142297af4:     	movq	%rax, 0x48(%rsp)
142297af9:     	movq	0x148(%rsp), %rax
142297b01:     	movq	%rax, 0x40(%rsp)
142297b06:     	movl	0x154(%rsp), %eax
142297b0d:     	movl	%eax, 0x84(%rsp)
142297b14:     	movq	(%rbx), %rax
142297b17:     	movq	%rax, 0x90(%rsp)
142297b1f:     	movss	0x69cb6d(%rip), %xmm7   # 0x142934694
142297b27:     	movss	0x7560d0(%rip), %xmm8   # 0x1429edc00
142297b30:     	movss	0x68ddcb(%rip), %xmm9   # 0x142925904
142297b39:     	xorps	%xmm10, %xmm10
142297b3d:     	movss	0x77f172(%rip), %xmm11  # 0x142a16cb8
142297b46:     	movss	0x68ddd1(%rip), %xmm12  # 0x142925920
142297b4f:     	movss	0x735510(%rip), %xmm13  # 0x1429cd068
142297b58:     	movzbl	0x168(%rsp), %eax
142297b60:     	movb	%al, 0x2b(%rsp)
142297b64:     	movzbl	0x160(%rsp), %eax
142297b6c:     	movl	%eax, 0x2c(%rsp)
142297b70:     	jmp	0x142297bdd <.text+0x2296bdd>
142297b72:     	nopw	%cs:(%rax,%rax)
142297b80:     	addss	0x1c(%rbp), %xmm0
142297b85:     	xorps	%xmm1, %xmm1
142297b88:     	maxss	%xmm0, %xmm1
142297b8c:     	movaps	%xmm9, %xmm0
142297b90:     	minss	%xmm1, %xmm0
142297b94:     	movss	%xmm0, 0x1c(%rbp)
142297b99:     	movl	0x30(%rsp), %eax
142297b9d:     	movl	%eax, (%rcx)
142297b9f:     	movss	0x1c(%rbp), %xmm1
142297ba4:     	leaq	0xda3e4d(%rip), %rcx    # 0x14303b9f8
142297bab:     	callq	0x140c95ef0 <.text+0xc94ef0>
142297bb0:     	movaps	%xmm9, %xmm1
142297bb4:     	subss	%xmm0, %xmm1
142297bb8:     	mulss	%xmm10, %xmm1
142297bbd:     	mulss	%xmm13, %xmm0
142297bc2:     	addss	%xmm1, %xmm0
142297bc6:     	addss	%xmm9, %xmm0
142297bcb:     	movsd	0x8(%r13), %xmm1
142297bd1:     	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
142297bd5:     	mulps	%xmm1, %xmm0
142297bd8:     	movlps	%xmm0, 0x8(%r13)
142297bdd:     	testb	$0x1, 0x2b(%rsp)
142297be2:     	je	0x142297c50 <.text+0x2296c50>
142297be4:     	movq	0x78(%rsp), %r8
142297be9:     	cmpq	%r14, %r8
142297bec:     	jne	0x142297d4f <.text+0x2296d4f>
142297bf2:     	nopw	%cs:(%rax,%rax)
142297c00:     	cmpq	%r12, %r15
142297c03:     	je	0x142297f87 <.text+0x2296f87>
142297c09:     	movl	(%r15), %ecx
142297c0c:     	addq	$0x4, %r15
142297c10:     	movq	0x1a8(%rdi), %rax
142297c17:     	leaq	(%rcx,%rcx,8), %rcx
142297c1b:     	movq	0x10(%rax,%rcx,8), %r14
142297c20:     	testq	%r14, %r14
142297c23:     	je	0x142297c00 <.text+0x2296c00>
142297c25:     	leaq	(%rax,%rcx,8), %rcx
142297c29:     	movq	0x38(%rcx), %rax
142297c2d:     	movq	0x38(%rsp), %rdx
142297c32:     	movq	0x128(%rdx), %rdx
142297c39:     	cmpq	%rdx, 0x40(%rcx)
142297c3d:     	jbe	0x142297cbd <.text+0x2296cbd>
142297c3f:     	cmpq	$0x0, (%rax,%rdx,8)
142297c44:     	setne	%dl
142297c47:     	movl	%edx, 0x2c(%rsp)
142297c4b:     	jmp	0x142297cc5 <.text+0x2296cc5>
142297c4d:     	nopl	(%rax)
142297c50:     	movq	0x78(%rsp), %r8
142297c55:     	cmpq	%r14, %r8
142297c58:     	jne	0x142297e0c <.text+0x2296e0c>
142297c5e:     	nop
142297c60:     	cmpq	%r12, %r15
142297c63:     	je	0x142297f87 <.text+0x2296f87>
142297c69:     	movl	(%r15), %ecx
142297c6c:     	addq	$0x4, %r15
142297c70:     	movq	0x100(%rdi), %rax
142297c77:     	leaq	(%rcx,%rcx,4), %rcx
142297c7b:     	shlq	$0x5, %rcx
142297c7f:     	movq	0x58(%rax,%rcx), %r14
142297c84:     	testq	%r14, %r14
142297c87:     	je	0x142297c60 <.text+0x2296c60>
142297c89:     	addq	%rcx, %rax
142297c8c:     	movq	0x38(%rsp), %rcx
142297c91:     	movq	0x128(%rcx), %rcx
142297c98:     	cmpq	%rcx, 0x88(%rax)
142297c9f:     	jbe	0x142297d57 <.text+0x2296d57>
142297ca5:     	movq	0x80(%rax), %rdx
142297cac:     	cmpq	$0x0, (%rdx,%rcx,8)
142297cb1:     	setne	%cl
142297cb4:     	movl	%ecx, 0x2c(%rsp)
142297cb8:     	jmp	0x142297d5f <.text+0x2296d5f>
142297cbd:     	movl	$0x0, 0x2c(%rsp)
142297cc5:     	movq	0x18(%rcx), %rcx
142297cc9:     	movq	0x38(%rsp), %r9
142297cce:     	movq	0x110(%r9), %rdx
142297cd5:     	movq	0x118(%r9), %r8
142297cdc:     	movq	(%rax,%rdx,8), %rdx
142297ce0:     	notq	%rdx
142297ce3:     	leaq	(%rdx,%rdx,2), %rdx
142297ce7:     	shlq	$0x4, %rdx
142297ceb:     	movq	0x10(%rcx,%rdx), %r10
142297cf0:     	movq	%r10, 0x68(%rsp)
142297cf5:     	movq	0x28(%rcx,%rdx), %rdx
142297cfa:     	movq	%rdx, 0x60(%rsp)
142297cff:     	movq	(%rax,%r8,8), %rdx
142297d03:     	notq	%rdx
142297d06:     	leaq	(%rdx,%rdx,2), %rdx
142297d0a:     	shlq	$0x4, %rdx
142297d0e:     	movq	0x10(%rcx,%rdx), %r8
142297d13:     	movq	%r8, 0x58(%rsp)
142297d18:     	movq	0x28(%rcx,%rdx), %rdx
142297d1d:     	movq	%rdx, 0x50(%rsp)
142297d22:     	movq	0x120(%r9), %rdx
142297d29:     	movq	(%rax,%rdx,8), %rax
142297d2d:     	notq	%rax
142297d30:     	leaq	(%rax,%rax,2), %rax
142297d34:     	shlq	$0x4, %rax
142297d38:     	movq	0x10(%rcx,%rax), %rdx
142297d3d:     	movq	%rdx, 0x48(%rsp)
142297d42:     	movq	0x28(%rcx,%rax), %rax
142297d47:     	movq	%rax, 0x40(%rsp)
142297d4c:     	xorl	%r8d, %r8d
142297d4f:     	movl	%r8d, %r13d
142297d52:     	jmp	0x142297e20 <.text+0x2296e20>
142297d57:     	movl	$0x0, 0x2c(%rsp)
142297d5f:     	movl	0x94(%rax), %ecx
142297d65:     	movq	0x1a8(%rdi), %rdx
142297d6c:     	leaq	(%rcx,%rcx,8), %rcx
142297d70:     	movq	0x18(%rdx,%rcx,8), %r8
142297d75:     	movq	0x38(%rdx,%rcx,8), %rcx
142297d7a:     	movq	0x38(%rsp), %r10
142297d7f:     	movq	0x110(%r10), %rdx
142297d86:     	movq	0x118(%r10), %r9
142297d8d:     	movq	(%rcx,%rdx,8), %rdx
142297d91:     	notq	%rdx
142297d94:     	leaq	(%rdx,%rdx,2), %rdx
142297d98:     	shlq	$0x4, %rdx
142297d9c:     	movq	0x10(%r8,%rdx), %r11
142297da1:     	movq	%r11, 0x68(%rsp)
142297da6:     	movq	0x28(%r8,%rdx), %rdx
142297dab:     	movq	%rdx, 0x60(%rsp)
142297db0:     	movq	(%rcx,%r9,8), %rdx
142297db4:     	notq	%rdx
142297db7:     	leaq	(%rdx,%rdx,2), %rdx
142297dbb:     	shlq	$0x4, %rdx
142297dbf:     	movq	0x10(%r8,%rdx), %r9
142297dc4:     	movq	%r9, 0x58(%rsp)
142297dc9:     	movq	0x28(%r8,%rdx), %rdx
142297dce:     	movq	%rdx, 0x50(%rsp)
142297dd3:     	movq	0x120(%r10), %rdx
142297dda:     	movq	(%rcx,%rdx,8), %rcx
142297dde:     	notq	%rcx
142297de1:     	leaq	(%rcx,%rcx,2), %rcx
142297de5:     	shlq	$0x4, %rcx
142297de9:     	movq	0x10(%r8,%rcx), %rdx
142297dee:     	movq	%rdx, 0x48(%rsp)
142297df3:     	movq	0x28(%r8,%rcx), %rcx
142297df8:     	movq	%rcx, 0x40(%rsp)
142297dfd:     	movq	0x50(%rax), %rax
142297e01:     	movq	%rax, 0x88(%rsp)
142297e09:     	xorl	%r8d, %r8d
142297e0c:     	movq	%r8, %rax
142297e0f:     	shlq	$0x4, %rax
142297e13:     	movq	0x88(%rsp), %rcx
142297e1b:     	movl	0x8(%rcx,%rax), %r13d
142297e20:     	leaq	(,%r13,8), %rax
142297e28:     	addq	%r13, %rax
142297e2b:     	movq	0x68(%rsp), %rcx
142297e30:     	leaq	(%rcx,%rax,4), %rbp
142297e34:     	movq	0x60(%rsp), %rax
142297e39:     	leaq	(%rax,%r13,4), %rsi
142297e3d:     	movq	0x50(%rsp), %rax
142297e42:     	leaq	(%rax,%r13,4), %rax
142297e46:     	movq	%rax, 0x70(%rsp)
142297e4b:     	movq	0x48(%rsp), %rax
142297e50:     	leaq	(%rax,%r13,8), %rax
142297e54:     	movq	0x40(%rsp), %rcx
142297e59:     	leaq	(%rcx,%r13,4), %rcx
142297e5d:     	shlq	$0x4, %r13
142297e61:     	addq	0x58(%rsp), %r13
142297e66:     	movq	0x90(%rsp), %rdx
142297e6e:     	movss	0x28(%rdx), %xmm0
142297e73:     	movl	0x34(%rsp), %edx
142297e77:     	movl	%edx, (%rsi)
142297e79:     	movaps	%xmm0, %xmm1
142297e7c:     	mulss	%xmm7, %xmm1
142297e80:     	divss	%xmm8, %xmm0
142297e85:     	testb	$0x1, 0x2c(%rsp)
142297e8a:     	jne	0x142297e8f <.text+0x2296e8f>
142297e8c:     	movaps	%xmm1, %xmm0
142297e8f:     	movl	0x2c(%rsp), %ebx
142297e93:     	movl	%ebx, %edx
142297e95:     	notb	%dl
142297e97:     	incq	%r8
142297e9a:     	movq	%r8, 0x78(%rsp)
142297e9f:     	addss	0x18(%rbp), %xmm0
142297ea4:     	movzbl	%dl, %edx
142297ea7:     	andl	$0x1, %edx
142297eaa:     	movss	%xmm0, 0x18(%rbp)
142297eaf:     	movl	0x84(%rsp), %r8d
142297eb7:     	movl	%r8d, (%rcx)
142297eba:     	movq	%rdx, (%rax)
142297ebd:     	movss	0x18(%rbp), %xmm0
142297ec2:     	minss	%xmm9, %xmm0
142297ec7:     	movl	0x34(%rsp), %eax
142297ecb:     	movl	%eax, (%rsi)
142297ecd:     	movss	%xmm0, 0x18(%rbp)
142297ed2:     	xorps	%xmm1, %xmm1
142297ed5:     	maxss	%xmm0, %xmm1
142297ed9:     	movaps	%xmm9, %xmm6
142297edd:     	minss	%xmm1, %xmm6
142297ee1:     	movsd	(%rbp), %xmm14
142297ee7:     	leaq	0xda3b02(%rip), %rcx    # 0x14303b9f0
142297eee:     	movaps	%xmm6, %xmm1
142297ef1:     	callq	0x140c95ef0 <.text+0xc94ef0>
142297ef6:     	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
142297efa:     	mulps	%xmm14, %xmm0
142297efe:     	movl	0x30(%rsp), %eax
142297f02:     	movq	0x70(%rsp), %rcx
142297f07:     	movl	%eax, (%rcx)
142297f09:     	movlps	%xmm0, 0x8(%r13)
142297f0e:     	testb	$0x1, %bl
142297f11:     	jne	0x142297bdd <.text+0x2296bdd>
142297f17:     	leaq	0xda3ad2(%rip), %rcx    # 0x14303b9f0
142297f1e:     	movaps	%xmm6, %xmm1
142297f21:     	callq	0x140c95ef0 <.text+0xc94ef0>
142297f26:     	movq	0x70(%rsp), %rcx
142297f2b:     	movaps	%xmm9, %xmm1
142297f2f:     	subss	%xmm0, %xmm1
142297f33:     	mulss	%xmm11, %xmm1
142297f38:     	addss	%xmm0, %xmm1
142297f3c:     	movaps	%xmm9, %xmm0
142297f40:     	subss	%xmm1, %xmm0
142297f44:     	movsd	0x8(%rbp), %xmm2
142297f49:     	movsd	0x10(%rbp), %xmm3
142297f4e:     	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
142297f52:     	mulps	%xmm3, %xmm0
142297f55:     	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
142297f59:     	mulps	%xmm2, %xmm1
142297f5c:     	addps	%xmm0, %xmm1
142297f5f:     	movl	0x30(%rsp), %eax
142297f63:     	movl	%eax, (%rcx)
142297f65:     	movlps	%xmm1, (%r13)
142297f6a:     	cmpb	$0x0, 0x20(%rbp)
142297f6e:     	movl	0x34(%rsp), %eax
142297f72:     	movl	%eax, (%rsi)
142297f74:     	movaps	%xmm12, %xmm0
142297f78:     	jne	0x142297b80 <.text+0x2296b80>
142297f7e:     	movaps	%xmm8, %xmm0
142297f82:     	jmp	0x142297b80 <.text+0x2296b80>
142297f87:     	movaps	0x170(%rsp), %xmm6
142297f8f:     	movaps	0x180(%rsp), %xmm7
142297f97:     	movaps	0x190(%rsp), %xmm8
142297fa0:     	movaps	0x1a0(%rsp), %xmm9
142297fa9:     	movaps	0x1b0(%rsp), %xmm10
142297fb2:     	movaps	0x1c0(%rsp), %xmm11
142297fbb:     	movaps	0x1d0(%rsp), %xmm12
142297fc4:     	movaps	0x1e0(%rsp), %xmm13
142297fcd:     	movaps	0x1f0(%rsp), %xmm14
142297fd6:     	addq	$0x208, %rsp            # imm = 0x208
142297fdd:     	popq	%rbx
142297fde:     	popq	%rbp
142297fdf:     	popq	%rdi
142297fe0:     	popq	%rsi
142297fe1:     	popq	%r12
142297fe3:     	popq	%r13
142297fe5:     	popq	%r14
142297fe7:     	popq	%r15
142297fe9:     	retq
142297fea:     	int3
142297feb:     	int3
142297fec:     	int3
142297fed:     	int3
142297fee:     	int3
142297fef:     	int3
