
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1406818c0:     	pushq	%rsi
1406818c1:     	subq	$0x30, %rsp
1406818c5:     	movq	%rcx, %rsi
1406818c8:     	movq	%rdx, 0x28(%rsp)
1406818cd:     	movq	$0x0, (%rcx)
1406818d4:     	movq	$0x4, 0x8(%rcx)
1406818dc:     	xorps	%xmm0, %xmm0
1406818df:     	movups	%xmm0, 0x10(%rcx)
1406818e3:     	movq	$0x4, 0x20(%rcx)
1406818eb:     	movq	$0x0, 0x28(%rcx)
1406818f3:     	movq	$0x1, 0x30(%rcx)
1406818fb:     	movq	$0x0, 0x38(%rcx)
140681903:     	movb	$0x28, 0x44(%rcx)
140681907:     	movl	$0x3f800000, 0x40(%rcx) # imm = 0x3F800000
14068190e:     	lock
14068190f:     	decq	(%rdx)
140681912:     	jne	0x14068191e <.text+0x68091e>
140681914:     	leaq	0x28(%rsp), %rcx
140681919:     	callq	0x140b8e120 <.text+0xb8d120>
14068191e:     	movq	%rsi, %rax
140681921:     	addq	$0x30, %rsp
140681925:     	popq	%rsi
140681926:     	retq
140681927:     	int3
140681928:     	int3
140681929:     	int3
14068192a:     	int3
14068192b:     	int3
14068192c:     	int3
14068192d:     	int3
14068192e:     	int3
14068192f:     	int3
140681930:     	pushq	%rsi
140681931:     	subq	$0x20, %rsp
140681935:     	movq	%rdx, %rsi
140681938:     	movzbl	0x44(%rcx), %edx
14068193c:     	cmpb	$0x28, %dl
14068193f:     	je	0x140681954 <.text+0x680954>
140681941:     	movq	0x8(%rsi), %rcx
140681945:     	callq	0x140946620 <.text+0x945620>
14068194a:     	testb	%al, %al
14068194c:     	je	0x140681954 <.text+0x680954>
14068194e:     	movq	(%rsi), %rax
140681951:     	orb	$0x20, (%rax)
140681954:     	addq	$0x20, %rsp
140681958:     	popq	%rsi
140681959:     	retq
14068195a:     	int3
14068195b:     	int3
14068195c:     	int3
14068195d:     	int3
14068195e:     	int3
14068195f:     	int3
140681960:     	pushq	%rbp
140681961:     	pushq	%r15
140681963:     	pushq	%r14
140681965:     	pushq	%r13
140681967:     	pushq	%r12
140681969:     	pushq	%rsi
14068196a:     	pushq	%rdi
14068196b:     	pushq	%rbx
14068196c:     	subq	$0x98, %rsp
140681973:     	leaq	0x80(%rsp), %rbp
14068197b:     	movq	$-0x2, 0x10(%rbp)
140681983:     	movq	%r8, %rdi
140681986:     	movq	%rdx, %rbx
140681989:     	movq	%rcx, -0x38(%rbp)
14068198d:     	movq	0x18(%r8), %rax
140681991:     	movzbl	(%rax), %esi
140681994:     	movq	%rdx, %rcx
140681997:     	callq	0x140b92590 <.text+0xb91590>
14068199c:     	leaq	-0x30(%rbp), %r14
1406819a0:     	movq	%r14, %rcx
1406819a3:     	callq	0x140a82b20 <.text+0xa81b20>
1406819a8:     	movl	%esi, %r8d
1406819ab:     	andl	$0x1, %r8d
1406819af:     	shll	$0x5, %r8d
1406819b3:     	addl	$0xffffff20, %r8d       # imm = 0xFFFFFF20
1406819ba:     	leaq	-0x58(%rbp), %r15
1406819be:     	movq	%r15, %rcx
1406819c1:     	movq	%r14, %rdx
1406819c4:     	callq	0x140b920b0 <.text+0xb910b0>
1406819c9:     	movss	0x22b2bd7(%rip), %xmm2  # 0x1429345a8
1406819d1:     	movq	%r14, %rcx
1406819d4:     	movq	%r15, %rdx
1406819d7:     	callq	0x140b92160 <.text+0xb91160>
1406819dc:     	movq	0x10(%rdi), %r8
1406819e0:     	leaq	0x23957b1(%rip), %r9    # 0x142a17198
1406819e7:     	movq	%rbx, %rcx
1406819ea:     	movq	%r14, %rdx
1406819ed:     	movq	%r8, -0x40(%rbp)
1406819f1:     	callq	0x140b92340 <.text+0xb91340>
1406819f6:     	movq	-0x28(%rbp), %r14
1406819fa:     	movq	-0x20(%rbp), %r15
1406819fe:     	movabsq	$0x7ffffffffffffffe, %r12 # imm = 0x7FFFFFFFFFFFFFFE
140681a08:     	movabsq	$-0x7ffffffffffffffe, %r13 # imm = 0x8000000000000002
140681a12:     	testq	%r15, %r15
140681a15:     	je	0x140681a5d <.text+0x680a5d>
140681a17:     	leaq	0x8(%r14), %rdi
140681a1b:     	jmp	0x140681a29 <.text+0x680a29>
140681a1d:     	nopl	(%rax)
140681a20:     	addq	$0x20, %rdi
140681a24:     	decq	%r15
140681a27:     	je	0x140681a5d <.text+0x680a5d>
140681a29:     	movq	-0x8(%rdi), %rdx
140681a2d:     	leaq	(%rdx,%r12), %rcx
140681a31:     	cmpq	$0x3, %rcx
140681a35:     	setb	%al
140681a38:     	cmpq	$0x1, %rcx
140681a3c:     	setne	%cl
140681a3f:     	cmpq	%r13, %rdx
140681a42:     	jl	0x140681a20 <.text+0x680a20>
140681a44:     	andb	%cl, %al
140681a46:     	jne	0x140681a20 <.text+0x680a20>
140681a48:     	testq	%rdx, %rdx
140681a4b:     	je	0x140681a20 <.text+0x680a20>
140681a4d:     	movq	(%rdi), %rcx
140681a50:     	movl	$0x1, %r8d
140681a56:     	callq	0x140613c20 <.text+0x612c20>
140681a5b:     	jmp	0x140681a20 <.text+0x680a20>
140681a5d:     	movq	-0x30(%rbp), %rdx
140681a61:     	testq	%rdx, %rdx
140681a64:     	je	0x140681a78 <.text+0x680a78>
140681a66:     	shlq	$0x5, %rdx
140681a6a:     	movl	$0x8, %r8d
140681a70:     	movq	%r14, %rcx
140681a73:     	callq	0x140613c20 <.text+0x612c20>
140681a78:     	leaq	0x18(%rbx), %r14
140681a7c:     	movq	%r14, %rcx
140681a7f:     	callq	0x140b92590 <.text+0xb91590>
140681a84:     	leaq	-0x30(%rbp), %r15
140681a88:     	movq	%r15, %rcx
140681a8b:     	callq	0x140a82b20 <.text+0xa81b20>
140681a90:     	movq	0x30(%rbx), %r8
140681a94:     	movq	0x38(%rbx), %r9
140681a98:     	movb	$0x1, 0x20(%rsp)
140681a9d:     	leaq	-0x10(%rbp), %rcx
140681aa1:     	movq	%r15, %rdx
140681aa4:     	callq	0x140fad6a0 <.text+0xfac6a0>
140681aa9:     	testb	$0x1, %sil
140681aad:     	jne	0x140681b39 <.text+0x680b39>
140681ab3:     	movss	0x40(%rbx), %xmm0
140681ab8:     	movss	0x22a3e44(%rip), %xmm1  # 0x142925904
140681ac0:     	ucomiss	%xmm0, %xmm1
140681ac3:     	jbe	0x140681b39 <.text+0x680b39>
140681ac5:     	mulss	0x231ed07(%rip), %xmm0  # 0x1429a07d4
140681acd:     	xorps	%xmm1, %xmm1
140681ad0:     	maxss	%xmm0, %xmm1
140681ad4:     	movss	0x22b9194(%rip), %xmm0  # 0x14293ac70
140681adc:     	minss	%xmm1, %xmm0
140681ae0:     	cvttss2si	%xmm0, %rax
140681ae5:     	xorps	%xmm1, %xmm1
140681ae8:     	xorl	%ecx, %ecx
140681aea:     	ucomiss	%xmm1, %xmm0
140681aed:     	cmovael	%eax, %ecx
140681af0:     	movq	(%rbp), %rax
140681af4:     	movq	%rax, -0x20(%rbp)
140681af8:     	ucomiss	0x231ecd1(%rip), %xmm0  # 0x1429a07d0
140681aff:     	movl	$0xffffffff, %r8d       # imm = 0xFFFFFFFF
140681b05:     	cmovbel	%ecx, %r8d
140681b09:     	movups	-0x10(%rbp), %xmm0
140681b0d:     	movaps	%xmm0, -0x30(%rbp)
140681b11:     	orl	$0xffffff00, %r8d       # imm = 0xFFFFFF00
140681b18:     	movb	$0x0, 0xf(%rbp)
140681b1c:     	leaq	-0x58(%rbp), %rcx
140681b20:     	leaq	-0x30(%rbp), %rdx
140681b24:     	callq	0x140b920b0 <.text+0xb910b0>
140681b29:     	movups	-0x58(%rbp), %xmm0
140681b2d:     	movaps	%xmm0, -0x10(%rbp)
140681b31:     	movq	-0x48(%rbp), %rax
140681b35:     	movq	%rax, (%rbp)
140681b39:     	movb	$0x1, 0xf(%rbp)
140681b3d:     	leaq	0x2395654(%rip), %r9    # 0x142a17198
140681b44:     	leaq	-0x10(%rbp), %rdx
140681b48:     	movq	%r14, %rcx
140681b4b:     	movq	-0x40(%rbp), %r8
140681b4f:     	callq	0x140b92340 <.text+0xb91340>
140681b54:     	xorps	%xmm0, %xmm0
140681b57:     	movq	-0x38(%rbp), %r14
140681b5b:     	movups	%xmm0, (%r14)
140681b5f:     	movq	-0x8(%rbp), %rdi
140681b63:     	movq	(%rbp), %rsi
140681b67:     	testq	%rsi, %rsi
140681b6a:     	je	0x140681bbd <.text+0x680bbd>
140681b6c:     	leaq	0x8(%rdi), %rbx
140681b70:     	jmp	0x140681b89 <.text+0x680b89>
140681b72:     	nopw	%cs:(%rax,%rax)
140681b80:     	addq	$0x20, %rbx
140681b84:     	decq	%rsi
140681b87:     	je	0x140681bbd <.text+0x680bbd>
140681b89:     	movq	-0x8(%rbx), %rdx
140681b8d:     	leaq	(%rdx,%r12), %rcx
140681b91:     	cmpq	$0x3, %rcx
140681b95:     	setb	%al
140681b98:     	cmpq	$0x1, %rcx
140681b9c:     	setne	%cl
140681b9f:     	cmpq	%r13, %rdx
140681ba2:     	jl	0x140681b80 <.text+0x680b80>
140681ba4:     	andb	%cl, %al
140681ba6:     	jne	0x140681b80 <.text+0x680b80>
140681ba8:     	testq	%rdx, %rdx
140681bab:     	je	0x140681b80 <.text+0x680b80>
140681bad:     	movq	(%rbx), %rcx
140681bb0:     	movl	$0x1, %r8d
140681bb6:     	callq	0x140613c20 <.text+0x612c20>
140681bbb:     	jmp	0x140681b80 <.text+0x680b80>
140681bbd:     	movq	-0x10(%rbp), %rdx
140681bc1:     	testq	%rdx, %rdx
140681bc4:     	je	0x140681bd8 <.text+0x680bd8>
140681bc6:     	shlq	$0x5, %rdx
140681bca:     	movl	$0x8, %r8d
140681bd0:     	movq	%rdi, %rcx
140681bd3:     	callq	0x140613c20 <.text+0x612c20>
140681bd8:     	movq	%r14, %rax
140681bdb:     	addq	$0x98, %rsp
140681be2:     	popq	%rbx
140681be3:     	popq	%rdi
140681be4:     	popq	%rsi
140681be5:     	popq	%r12
140681be7:     	popq	%r13
140681be9:     	popq	%r14
140681beb:     	popq	%r15
140681bed:     	popq	%rbp
140681bee:     	retq
140681bef:     	nop
140681bf0:     	movq	%rdx, 0x10(%rsp)
140681bf5:     	pushq	%rbp
140681bf6:     	pushq	%r15
140681bf8:     	pushq	%r14
140681bfa:     	pushq	%r13
140681bfc:     	pushq	%r12
140681bfe:     	pushq	%rsi
140681bff:     	pushq	%rdi
140681c00:     	pushq	%rbx
140681c01:     	subq	$0x28, %rsp
140681c05:     	leaq	0x80(%rdx), %rbp
140681c0c:     	cmpb	$0x0, 0xf(%rbp)
140681c10:     	jne	0x140681c23 <.text+0x680c23>
140681c12:     	addq	$0x28, %rsp
140681c16:     	popq	%rbx
140681c17:     	popq	%rdi
140681c18:     	popq	%rsi
140681c19:     	popq	%r12
140681c1b:     	popq	%r13
140681c1d:     	popq	%r14
140681c1f:     	popq	%r15
140681c21:     	popq	%rbp
140681c22:     	retq
140681c23:     	leaq	-0x10(%rbp), %rcx
140681c27:     	callq	0x1405f11f0 <.text+0x5f01f0>
140681c2c:     	jmp	0x140681c12 <.text+0x680c12>
140681c2e:     	nop
140681c30:     	movq	%rdx, 0x10(%rsp)
140681c35:     	pushq	%rbp
140681c36:     	pushq	%r15
140681c38:     	pushq	%r14
140681c3a:     	pushq	%r13
140681c3c:     	pushq	%r12
140681c3e:     	pushq	%rsi
140681c3f:     	pushq	%rdi
140681c40:     	pushq	%rbx
140681c41:     	subq	$0x28, %rsp
140681c45:     	leaq	0x80(%rdx), %rbp
140681c4c:     	leaq	-0x30(%rbp), %rcx
140681c50:     	callq	0x1405f11f0 <.text+0x5f01f0>
140681c55:     	nop
140681c56:     	addq	$0x28, %rsp
140681c5a:     	popq	%rbx
140681c5b:     	popq	%rdi
140681c5c:     	popq	%rsi
140681c5d:     	popq	%r12
140681c5f:     	popq	%r13
140681c61:     	popq	%r14
140681c63:     	popq	%r15
140681c65:     	popq	%rbp
140681c66:     	retq
140681c67:     	int3
140681c68:     	int3
140681c69:     	int3
140681c6a:     	int3
140681c6b:     	int3
140681c6c:     	int3
140681c6d:     	int3
140681c6e:     	int3
140681c6f:     	int3
140681c70:     	pushq	%r14
140681c72:     	pushq	%rsi
140681c73:     	pushq	%rdi
140681c74:     	pushq	%rbx
140681c75:     	subq	$0x48, %rsp
140681c79:     	movaps	%xmm6, 0x30(%rsp)
140681c7e:     	movq	%rcx, %rsi
140681c81:     	movss	0x8(%rdx), %xmm0
140681c86:     	movss	0xc(%rdx), %xmm1
140681c8b:     	movss	0x22a3bdd(%rip), %xmm2  # 0x142925870
140681c93:     	movss	0x10(%rdx), %xmm3
140681c98:     	mulss	%xmm2, %xmm3
140681c9c:     	mulss	0x14(%rdx), %xmm2
140681ca1:     	movaps	%xmm0, %xmm4
140681ca4:     	subss	%xmm3, %xmm4
140681ca8:     	movaps	%xmm1, %xmm5
140681cab:     	subss	%xmm2, %xmm5
140681caf:     	addss	%xmm0, %xmm3
140681cb3:     	addss	%xmm1, %xmm2
140681cb7:     	movq	(%rdx), %rdi
140681cba:     	movd	%xmm4, %ebx
140681cbe:     	movd	%xmm5, %eax
140681cc2:     	shlq	$0x20, %rax
140681cc6:     	orq	%rax, %rbx
140681cc9:     	subss	%xmm4, %xmm3
140681ccd:     	subss	%xmm5, %xmm2
140681cd1:     	movd	%xmm3, %r14d
140681cd6:     	movd	%xmm2, %eax
140681cda:     	shlq	$0x20, %rax
140681cde:     	orq	%rax, %r14
140681ce1:     	movss	0x18(%rdx), %xmm6
140681ce6:     	movq	%rcx, 0x20(%rsp)
140681ceb:     	movq	%rdi, %rcx
140681cee:     	movq	%rbx, %rdx
140681cf1:     	movq	%r14, %r8
140681cf4:     	movaps	%xmm6, %xmm3
140681cf7:     	callq	0x140b8f510 <.text+0xb8e510>
140681cfc:     	addq	$0x18, %rsi
140681d00:     	movq	%rsi, 0x20(%rsp)
140681d05:     	movq	%rdi, %rcx
140681d08:     	movq	%rbx, %rdx
140681d0b:     	movq	%r14, %r8
140681d0e:     	movaps	%xmm6, %xmm3
140681d11:     	callq	0x140b8f510 <.text+0xb8e510>
140681d16:     	movaps	0x30(%rsp), %xmm6
140681d1b:     	addq	$0x48, %rsp
140681d1f:     	popq	%rbx
140681d20:     	popq	%rdi
140681d21:     	popq	%rsi
140681d22:     	popq	%r14
140681d24:     	retq
