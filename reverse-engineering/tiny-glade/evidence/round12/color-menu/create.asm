
reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1425f27c0:     	pushq	%rbp
1425f27c1:     	pushq	%r15
1425f27c3:     	pushq	%r14
1425f27c5:     	pushq	%r13
1425f27c7:     	pushq	%r12
1425f27c9:     	pushq	%rsi
1425f27ca:     	pushq	%rdi
1425f27cb:     	pushq	%rbx
1425f27cc:     	subq	$0x2f8, %rsp            # imm = 0x2F8
1425f27d3:     	leaq	0x80(%rsp), %rbp
1425f27db:     	movaps	%xmm15, 0x260(%rbp)
1425f27e3:     	movaps	%xmm14, 0x250(%rbp)
1425f27eb:     	movaps	%xmm13, 0x240(%rbp)
1425f27f3:     	movaps	%xmm12, 0x230(%rbp)
1425f27fb:     	movaps	%xmm11, 0x220(%rbp)
1425f2803:     	movaps	%xmm10, 0x210(%rbp)
1425f280b:     	movaps	%xmm9, 0x200(%rbp)
1425f2813:     	movaps	%xmm8, 0x1f0(%rbp)
1425f281b:     	movaps	%xmm7, 0x1e0(%rbp)
1425f2822:     	movaps	%xmm6, 0x1d0(%rbp)
1425f2829:     	movq	$-0x2, 0x1c8(%rbp)
1425f2834:     	movaps	%xmm3, %xmm6
1425f2837:     	movaps	%xmm2, %xmm7
1425f283a:     	movaps	%xmm1, %xmm8
1425f283e:     	movq	%rcx, %r14
1425f2841:     	movq	0x300(%rbp), %rax
1425f2848:     	movq	0x10(%rax), %rbx
1425f284c:     	testq	%rbx, %rbx
1425f284f:     	je	0x1425f288a <.text+0x25f188a>
1425f2851:     	movq	0x8(%rax), %rax
1425f2855:     	leaq	(,%rbx,8), %rcx
1425f285d:     	leaq	(%rcx,%rcx,4), %rcx
1425f2861:     	xorl	%edx, %edx
1425f2863:     	nopw	%cs:(%rax,%rax)
1425f2870:     	xorl	%r8d, %r8d
1425f2873:     	cmpb	$0x5, 0x18(%rax,%rdx)
1425f2878:     	sete	%r8b
1425f287c:     	subq	%r8, %rbx
1425f287f:     	addq	$0x28, %rdx
1425f2883:     	cmpq	%rdx, %rcx
1425f2886:     	jne	0x1425f2870 <.text+0x25f1870>
1425f2888:     	jmp	0x1425f288c <.text+0x25f188c>
1425f288a:     	xorl	%ebx, %ebx
1425f288c:     	leaq	(,%rbx,8), %rdi
1425f2894:     	xorl	%esi, %esi
1425f2896:     	movq	%rbx, %rax
1425f2899:     	shrq	$0x3d, %rax
1425f289d:     	jne	0x1425f3755 <.text+0x25f2755>
1425f28a3:     	movabsq	$0x7ffffffffffffff8, %rax # imm = 0x7FFFFFFFFFFFFFF8
1425f28ad:     	cmpq	%rax, %rdi
1425f28b0:     	ja	0x1425f3755 <.text+0x25f2755>
1425f28b6:     	testq	%rdi, %rdi
1425f28b9:     	je	0x1425f28e2 <.text+0x25f18e2>
1425f28bb:     	movzbl	0x1042347(%rip), %eax   # 0x143634c09
1425f28c2:     	movl	$0x8, %esi
1425f28c7:     	movl	$0x8, %edx
1425f28cc:     	movq	%rdi, %rcx
1425f28cf:     	callq	0x140613c10 <.text+0x612c10>
1425f28d4:     	testq	%rax, %rax
1425f28d7:     	je	0x1425f3755 <.text+0x25f2755>
1425f28dd:     	movq	%rbx, %rcx
1425f28e0:     	jmp	0x1425f28e9 <.text+0x25f18e9>
1425f28e2:     	movl	$0x8, %eax
1425f28e7:     	xorl	%ecx, %ecx
1425f28e9:     	movss	0x2e8(%rbp), %xmm13
1425f28f2:     	movss	0x2e0(%rbp), %xmm12
1425f28fb:     	movq	%rcx, 0x160(%rbp)
1425f2902:     	movq	%rax, 0x168(%rbp)
1425f2909:     	cvtsi2ss	%rbx, %xmm0
1425f290e:     	movq	$0x0, 0x170(%rbp)
1425f2919:     	addss	0x3677c3(%rip), %xmm0   # 0x14295a0e4
1425f2921:     	movaps	%xmm13, %xmm1
1425f2925:     	addss	0x2f0(%rbp), %xmm1
1425f292d:     	mulss	%xmm0, %xmm1
1425f2931:     	divss	%xmm6, %xmm1
1425f2935:     	movss	0x3677aa(%rip), %xmm9   # 0x14295a0e8
1425f293e:     	mulss	%xmm1, %xmm9
1425f2943:     	addss	0x4b0914(%rip), %xmm9   # 0x142aa3260
1425f294c:     	addss	%xmm12, %xmm9
1425f2951:     	movaps	%xmm9, %xmm0
1425f2955:     	callq	0x1425f2460 <.text+0x25f1460>
1425f295a:     	movaps	%xmm0, %xmm10
1425f295e:     	movaps	%xmm1, %xmm11
1425f2962:     	leaq	0xb4450f(%rip), %r8     # 0x143136e78
1425f2969:     	leaq	-0x50(%rbp), %rcx
1425f296d:     	movq	0x300(%rbp), %rdx
1425f2974:     	callq	0x141c5a430 <.text+0x1c59430>
1425f2979:     	movq	-0x50(%rbp), %rax
1425f297d:     	movq	-0x40(%rbp), %rdi
1425f2981:     	cmpq	%rdi, %rax
1425f2984:     	jbe	0x1425f29d9 <.text+0x25f19d9>
1425f2986:     	movq	-0x48(%rbp), %rcx
1425f298a:     	shlq	$0x3, %rax
1425f298e:     	leaq	(%rax,%rax,4), %rdx
1425f2992:     	testq	%rdi, %rdi
1425f2995:     	je	0x1425f29df <.text+0x25f19df>
1425f2997:     	leaq	(,%rdi,8), %rax
1425f299f:     	leaq	(%rax,%rax,4), %rsi
1425f29a3:     	movl	$0x8, %r8d
1425f29a9:     	movq	%rsi, %r9
1425f29ac:     	callq	0x140613c30 <.text+0x612c30>
1425f29b1:     	movq	%rax, %rbx
1425f29b4:     	testq	%rax, %rax
1425f29b7:     	movq	0x300(%rbp), %r8
1425f29be:     	jne	0x1425f29f6 <.text+0x25f19f6>
1425f29c0:     	leaq	0xb444c9(%rip), %r8     # 0x143136e90
1425f29c7:     	movl	$0x8, %ecx
1425f29cc:     	movq	%rsi, %rdx
1425f29cf:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1425f29d4:     	jmp	0x1425f3767 <.text+0x25f2767>
1425f29d9:     	movq	-0x48(%rbp), %rbx
1425f29dd:     	jmp	0x1425f29ef <.text+0x25f19ef>
1425f29df:     	movl	$0x8, %ebx
1425f29e4:     	movl	$0x8, %r8d
1425f29ea:     	callq	0x140613c20 <.text+0x612c20>
1425f29ef:     	movq	0x300(%rbp), %r8
1425f29f6:     	movq	%r14, 0x128(%rbp)
1425f29fd:     	movq	0x318(%rbp), %rcx
1425f2a04:     	movq	0x310(%rbp), %rax
1425f2a0b:     	movq	0x308(%rbp), %rdx
1425f2a12:     	movss	%xmm6, 0xdc(%rbp)
1425f2a1a:     	movss	%xmm8, 0xa0(%rbp)
1425f2a23:     	movss	%xmm7, 0xa4(%rbp)
1425f2a2b:     	movss	%xmm13, 0xe0(%rbp)
1425f2a34:     	movq	$0x0, 0x60(%rbp)
1425f2a3c:     	movq	$0x8, 0x68(%rbp)
1425f2a44:     	movq	$0x0, 0x70(%rbp)
1425f2a4c:     	movups	(%rdx), %xmm0
1425f2a4f:     	movups	%xmm0, 0x80(%rbp)
1425f2a56:     	movups	(%rax), %xmm0
1425f2a59:     	movups	0x10(%rax), %xmm1
1425f2a5d:     	movups	%xmm0, 0xb8(%rbp)
1425f2a64:     	movups	%xmm1, 0xc8(%rbp)
1425f2a6b:     	movl	0x20(%rax), %edx
1425f2a6e:     	movl	%edx, 0xd8(%rbp)
1425f2a74:     	movl	$0x0, 0x78(%rbp)
1425f2a7b:     	movss	%xmm10, 0xa8(%rbp)
1425f2a84:     	movss	%xmm11, 0xac(%rbp)
1425f2a8d:     	movq	(%rcx), %rdx
1425f2a90:     	movq	%rdx, 0xe4(%rbp)
1425f2a97:     	movl	0x8(%rcx), %ecx
1425f2a9a:     	movl	%ecx, 0xec(%rbp)
1425f2aa0:     	movq	$0x0, 0x50(%rbp)
1425f2aa8:     	movq	$0x0, 0xb0(%rbp)
1425f2ab3:     	movq	%rbx, 0x90(%rbp)
1425f2aba:     	movq	%rdi, 0x98(%rbp)
1425f2ac1:     	movq	0x8(%r8), %rdi
1425f2ac5:     	movq	0x10(%r8), %rcx
1425f2ac9:     	movq	$0x0, 0x10(%r8)
1425f2ad1:     	leaq	(%rcx,%rcx,4), %rdx
1425f2ad5:     	leaq	(%rdi,%rdx,8), %rdx
1425f2ad9:     	movq	%rdx, 0x130(%rbp)
1425f2ae0:     	movq	%rdx, 0xf8(%rbp)
1425f2ae7:     	movq	%r8, 0x100(%rbp)
1425f2aee:     	movq	%rcx, 0x108(%rbp)
1425f2af5:     	movq	$0x0, 0x110(%rbp)
1425f2b00:     	testq	%rcx, %rcx
1425f2b03:     	je	0x1425f35d0 <.text+0x25f25d0>
1425f2b09:     	mulss	%xmm6, %xmm9
1425f2b0e:     	movzbl	0x1c(%rax), %edx
1425f2b12:     	movb	%dl, 0x1bc(%rbp)
1425f2b18:     	movss	0x8(%rax), %xmm0
1425f2b1d:     	movss	%xmm0, 0x148(%rbp)
1425f2b25:     	movzbl	0x10(%rax), %edx
1425f2b29:     	movb	%dl, 0x1bd(%rbp)
1425f2b2f:     	movl	0x14(%rax), %edx
1425f2b32:     	movl	%edx, 0x158(%rbp)
1425f2b38:     	movzbl	0x11(%rax), %edx
1425f2b3c:     	movb	%dl, 0x1bf(%rbp)
1425f2b42:     	movl	0x18(%rax), %edx
1425f2b45:     	movl	%edx, 0x15c(%rbp)
1425f2b4b:     	movss	0x4(%rax), %xmm0
1425f2b50:     	movss	%xmm0, 0x154(%rbp)
1425f2b58:     	movl	0xc(%rax), %edx
1425f2b5b:     	movl	%edx, 0x14c(%rbp)
1425f2b61:     	movzbl	0x1d(%rax), %edx
1425f2b65:     	movb	%dl, 0x1be(%rbp)
1425f2b6b:     	movl	(%rax), %eax
1425f2b6d:     	movl	%eax, 0x150(%rbp)
1425f2b73:     	movss	0x3fb015(%rip), %xmm0   # 0x1429edb90
1425f2b7b:     	mulss	%xmm13, %xmm0
1425f2b80:     	movss	%xmm0, 0x140(%rbp)
1425f2b88:     	movss	0x332dc0(%rip), %xmm0   # 0x142925950
1425f2b90:     	mulss	%xmm13, %xmm0
1425f2b95:     	movss	%xmm0, 0x144(%rbp)
1425f2b9d:     	xorps	%xmm0, %xmm0
1425f2ba0:     	cmpltss	%xmm12, %xmm0
1425f2ba6:     	movss	0x4c0671(%rip), %xmm13  # 0x142ab3220
1425f2baf:     	movss	0x332d4c(%rip), %xmm12  # 0x142925904
1425f2bb8:     	divss	0x2f8(%rbp), %xmm12
1425f2bc1:     	andps	%xmm0, %xmm13
1425f2bc5:     	movsldup	%xmm6, %xmm15           # xmm15 = xmm6[0,0,2,2]
1425f2bca:     	movaps	%xmm8, %xmm14
1425f2bce:     	unpcklps	%xmm7, %xmm14           # xmm14 = xmm14[0],xmm7[0],xmm14[1],xmm7[1]
1425f2bd2:     	addq	$0x28, %rdi
1425f2bd6:     	shlq	$0x3, %rcx
1425f2bda:     	leaq	(%rcx,%rcx,4), %rcx
1425f2bde:     	movl	$0x1, %r15d
1425f2be4:     	movl	$0x0, 0x1b8(%rbp)
1425f2bee:     	movq	%rdi, %rax
1425f2bf1:     	jmp	0x1425f2c31 <.text+0x25f1c31>
1425f2bf3:     	nopw	%cs:(%rax,%rax)
1425f2c00:     	movq	0x68(%rbp), %rax
1425f2c04:     	movq	%r12, (%rax,%rsi,8)
1425f2c08:     	incq	%rsi
1425f2c0b:     	movq	%rsi, 0x70(%rbp)
1425f2c0f:     	addss	0x2f0(%rbp), %xmm11
1425f2c18:     	addss	%xmm11, %xmm9
1425f2c1d:     	incq	%r15
1425f2c20:     	leaq	0x28(%rdi), %rax
1425f2c24:     	addq	$-0x28, %rbx
1425f2c28:     	movq	%rbx, %rcx
1425f2c2b:     	je	0x1425f35d0 <.text+0x25f25d0>
1425f2c31:     	movq	%rax, %rdi
1425f2c34:     	movq	%rax, 0xf0(%rbp)
1425f2c3b:     	movq	-0x28(%rax), %rax
1425f2c3f:     	movq	%rax, 0x178(%rbp)
1425f2c46:     	negq	%rax
1425f2c49:     	jo	0x1425f35d0 <.text+0x25f25d0>
1425f2c4f:     	movq	%rcx, 0x120(%rbp)
1425f2c56:     	leaq	-0x1(%r15), %r13
1425f2c5a:     	movq	-0x20(%rdi), %rax
1425f2c5e:     	movq	%rax, 0x138(%rbp)
1425f2c65:     	movq	-0x18(%rdi), %r14
1425f2c69:     	movzbl	-0x10(%rdi), %ebx
1425f2c6d:     	movzbl	-0xf(%rdi), %eax
1425f2c71:     	movb	%al, 0x1c6(%rbp)
1425f2c77:     	movzwl	-0xe(%rdi), %eax
1425f2c7b:     	movw	%ax, 0x1b6(%rbp)
1425f2c82:     	movl	-0xc(%rdi), %eax
1425f2c85:     	movl	%eax, 0x1c0(%rbp)
1425f2c8b:     	movl	-0x8(%rdi), %eax
1425f2c8e:     	movl	%eax, 0x184(%rbp)
1425f2c94:     	movl	-0x4(%rdi), %r12d
1425f2c98:     	movq	%r15, 0x118(%rbp)
1425f2c9f:     	leal	-0x3(%rbx), %eax
1425f2ca2:     	cmpb	$0xc, %al
1425f2ca4:     	movzbl	%al, %eax
1425f2ca7:     	movl	$0x8, %ecx
1425f2cac:     	cmovael	%ecx, %eax
1425f2caf:     	movzbl	%al, %esi
1425f2cb2:     	cmpb	$0x2, %al
1425f2cb4:     	jne	0x1425f2d00 <.text+0x25f1d00>
1425f2cb6:     	subss	0x144(%rbp), %xmm9
1425f2cbf:     	movaps	%xmm9, %xmm11
1425f2cc3:     	divss	%xmm6, %xmm11
1425f2cc8:     	movaps	%xmm11, %xmm0
1425f2ccc:     	callq	0x142923e80 <.text+0x2922e80>
1425f2cd1:     	movaps	%xmm0, %xmm10
1425f2cd5:     	movaps	%xmm11, %xmm0
1425f2cd9:     	callq	0x142923fd0 <.text+0x2922fd0>
1425f2cde:     	unpcklps	%xmm0, %xmm10           # xmm10 = xmm10[0],xmm0[0],xmm10[1],xmm0[1]
1425f2ce2:     	xorl	%eax, %eax
1425f2ce4:     	movss	0x140(%rbp), %xmm11
1425f2ced:     	jmp	0x1425f2e60 <.text+0x25f1e60>
1425f2cf2:     	nopw	%cs:(%rax,%rax)
1425f2d00:     	movaps	%xmm9, %xmm11
1425f2d04:     	divss	%xmm6, %xmm11
1425f2d09:     	movaps	%xmm11, %xmm0
1425f2d0d:     	callq	0x142923e80 <.text+0x2922e80>
1425f2d12:     	movaps	%xmm0, %xmm10
1425f2d16:     	movaps	%xmm11, %xmm0
1425f2d1a:     	callq	0x142923fd0 <.text+0x2922fd0>
1425f2d1f:     	unpcklps	%xmm0, %xmm10           # xmm10 = xmm10[0],xmm0[0],xmm10[1],xmm0[1]
1425f2d23:     	leaq	0xb441f6(%rip), %rcx    # 0x143136f20
1425f2d2a:     	movslq	(%rcx,%rsi,4), %rax
1425f2d2e:     	addq	%rcx, %rax
1425f2d31:     	jmpq	*%rax
1425f2d33:     	xorl	%eax, %eax
1425f2d35:     	movss	0x2e8(%rbp), %xmm11
1425f2d3e:     	jmp	0x1425f2e60 <.text+0x25f1e60>
1425f2d43:     	movzbl	0x1c6(%rbp), %eax
1425f2d4a:     	cmpb	%al, 0x1bc(%rbp)
1425f2d50:     	jne	0x1425f2d33 <.text+0x25f1d33>
1425f2d52:     	jmp	0x1425f2e2d <.text+0x25f1e2d>
1425f2d57:     	movzbl	0x1bd(%rbp), %eax
1425f2d5e:     	cmpb	%bl, %al
1425f2d60:     	jne	0x1425f2d33 <.text+0x25f1d33>
1425f2d62:     	cmpb	$0x2, %al
1425f2d64:     	jne	0x1425f2e2d <.text+0x25f1e2d>
1425f2d6a:     	xorl	%eax, %eax
1425f2d6c:     	movl	0x1c0(%rbp), %ecx
1425f2d72:     	cmpl	%ecx, 0x158(%rbp)
1425f2d78:     	jne	0x1425f2d35 <.text+0x25f1d35>
1425f2d7a:     	movzbl	0x1c6(%rbp), %ecx
1425f2d81:     	cmpb	%cl, 0x1bf(%rbp)
1425f2d87:     	jne	0x1425f2d35 <.text+0x25f1d35>
1425f2d89:     	movl	0x184(%rbp), %ecx
1425f2d8f:     	cmpl	%ecx, 0x15c(%rbp)
1425f2d95:     	je	0x1425f2e2d <.text+0x25f1e2d>
1425f2d9b:     	jmp	0x1425f2d35 <.text+0x25f1d35>
1425f2d9d:     	movzbl	0x1c6(%rbp), %eax
1425f2da4:     	cmpb	%al, 0x1be(%rbp)
1425f2daa:     	jne	0x1425f2d33 <.text+0x25f1d33>
1425f2dac:     	jmp	0x1425f2e2d <.text+0x25f1e2d>
1425f2dae:     	movb	$0x1, 0x1c7(%rbp)
1425f2db5:     	movzbl	0x1c6(%rbp), %ecx
1425f2dbc:     	callq	0x140b06770 <.text+0xb05770>
1425f2dc1:     	movss	0x154(%rbp), %xmm1
1425f2dc9:     	subss	%xmm0, %xmm1
1425f2dcd:     	andps	0x33bbac(%rip), %xmm1   # 0x14292e980
1425f2dd4:     	movss	0x3da284(%rip), %xmm0   # 0x1429cd060
1425f2ddc:     	ucomiss	%xmm1, %xmm0
1425f2ddf:     	jbe	0x1425f2d33 <.text+0x25f1d33>
1425f2de5:     	jmp	0x1425f2e2d <.text+0x25f1e2d>
1425f2de7:     	movl	0x1c0(%rbp), %eax
1425f2ded:     	cmpl	%eax, 0x14c(%rbp)
1425f2df3:     	jne	0x1425f2d33 <.text+0x25f1d33>
1425f2df9:     	jmp	0x1425f2e2d <.text+0x25f1e2d>
1425f2dfb:     	movss	0x1c0(%rbp), %xmm0
1425f2e03:     	movss	0x148(%rbp), %xmm1
1425f2e0b:     	ucomiss	%xmm0, %xmm1
1425f2e0e:     	jne	0x1425f2d33 <.text+0x25f1d33>
1425f2e14:     	jnp	0x1425f2e2d <.text+0x25f1e2d>
1425f2e16:     	jmp	0x1425f2d33 <.text+0x25f1d33>
1425f2e1b:     	movl	0x1c0(%rbp), %eax
1425f2e21:     	cmpl	%eax, 0x150(%rbp)
1425f2e27:     	jne	0x1425f2d33 <.text+0x25f1d33>
1425f2e2d:     	movb	$0x1, %al
1425f2e2f:     	testb	$0x1, 0x1b8(%rbp)
1425f2e36:     	jne	0x1425f2e41 <.text+0x25f1e41>
1425f2e38:     	movq	%r13, 0xb0(%rbp)
1425f2e3f:     	movb	$0x1, %al
1425f2e41:     	movss	0x2e8(%rbp), %xmm11
1425f2e4a:     	movb	$0x1, %cl
1425f2e4c:     	movl	%ecx, 0x1b8(%rbp)
1425f2e52:     	nopw	%cs:(%rax,%rax)
1425f2e60:     	movq	0x178(%rbp), %rcx
1425f2e67:     	movq	%rcx, -0x50(%rbp)
1425f2e6b:     	movq	0x138(%rbp), %rcx
1425f2e72:     	movq	%rcx, -0x48(%rbp)
1425f2e76:     	movq	%r14, -0x40(%rbp)
1425f2e7a:     	xorl	%ecx, %ecx
1425f2e7c:     	cmpb	$0x5, %bl
1425f2e7f:     	sete	%dl
1425f2e82:     	testq	%r13, %r13
1425f2e85:     	js	0x1425f2ea0 <.text+0x25f1ea0>
1425f2e87:     	xorps	%xmm0, %xmm0
1425f2e8a:     	cvtsi2ss	%r13, %xmm0
1425f2e8f:     	jmp	0x1425f2eb9 <.text+0x25f1eb9>
1425f2e91:     	nopw	%cs:(%rax,%rax)
1425f2ea0:     	movq	%r13, %r8
1425f2ea3:     	shrq	%r8
1425f2ea6:     	andl	$0x1, %r13d
1425f2eaa:     	orq	%r8, %r13
1425f2ead:     	xorps	%xmm0, %xmm0
1425f2eb0:     	cvtsi2ss	%r13, %xmm0
1425f2eb5:     	addss	%xmm0, %xmm0
1425f2eb9:     	mulps	%xmm15, %xmm10
1425f2ebd:     	addps	%xmm14, %xmm10
1425f2ec1:     	movb	%dl, %cl
1425f2ec3:     	leaq	0xb43d46(%rip), %rdx    # 0x143136c10
1425f2eca:     	movss	(%rdx,%rcx,4), %xmm1
1425f2ecf:     	mulss	%xmm11, %xmm1
1425f2ed4:     	mulss	0x3f881c(%rip), %xmm0   # 0x1429eb6f8
1425f2edc:     	addss	%xmm13, %xmm0
1425f2ee1:     	mulss	%xmm12, %xmm0
1425f2ee6:     	movss	%xmm8, -0x30(%rbp)
1425f2eec:     	movss	%xmm7, -0x2c(%rbp)
1425f2ef1:     	movq	$0x0, -0x28(%rbp)
1425f2ef9:     	movl	%r12d, -0x38(%rbp)
1425f2efd:     	movss	%xmm1, -0x20(%rbp)
1425f2f02:     	movss	%xmm1, -0x1c(%rbp)
1425f2f07:     	movlps	%xmm10, -0x18(%rbp)
1425f2f0c:     	movss	%xmm8, -0x10(%rbp)
1425f2f12:     	movss	%xmm7, -0xc(%rbp)
1425f2f17:     	movss	%xmm0, -0x8(%rbp)
1425f2f1c:     	movl	$0x0, -0x4(%rbp)
1425f2f23:     	movb	$0x0, (%rbp)
1425f2f27:     	movb	%al, 0x1(%rbp)
1425f2f2a:     	movq	$0x1, 0x8(%rbp)
1425f2f32:     	movb	$0x0, 0x1c7(%rbp)
1425f2f39:     	leaq	0x188(%rbp), %rcx
1425f2f40:     	movq	0x320(%rbp), %rdx
1425f2f47:     	leaq	-0x50(%rbp), %r8
1425f2f4b:     	leaq	0xb43f6e(%rip), %r9     # 0x143136ec0
1425f2f52:     	callq	0x1422bf090 <.text+0x22be090>
1425f2f57:     	leaq	0xb43ff2(%rip), %rcx    # 0x143136f50
1425f2f5e:     	movslq	(%rcx,%rsi,4), %rax
1425f2f62:     	addq	%rcx, %rax
1425f2f65:     	jmpq	*%rax
1425f2f67:     	movq	0x190(%rbp), %rbx
1425f2f6e:     	testq	%rbx, %rbx
1425f2f71:     	jne	0x1425f2f7a <.text+0x25f1f7a>
1425f2f73:     	movq	0x198(%rbp), %rbx
1425f2f7a:     	movq	0x1a8(%rbp), %r12
1425f2f81:     	movq	(%rbx), %rax
1425f2f84:     	movq	0x10(%rbx), %rsi
1425f2f88:     	subq	%rsi, %rax
1425f2f8b:     	cmpq	$0x1f, %rax
1425f2f8f:     	jbe	0x1425f34c8 <.text+0x25f24c8>
1425f2f95:     	movq	0x8(%rbx), %rax
1425f2f99:     	leaq	-0x335760(%rip), %rcx   # 0x1422bd840 <.text+0x22bc840>
1425f2fa0:     	jmp	0x1425f3057 <.text+0x25f2057>
1425f2fa5:     	movq	0x190(%rbp), %rbx
1425f2fac:     	testq	%rbx, %rbx
1425f2faf:     	jne	0x1425f2fb8 <.text+0x25f1fb8>
1425f2fb1:     	movq	0x198(%rbp), %rbx
1425f2fb8:     	movq	0x1a8(%rbp), %r12
1425f2fbf:     	movq	(%rbx), %rax
1425f2fc2:     	movq	0x10(%rbx), %rsi
1425f2fc6:     	subq	%rsi, %rax
1425f2fc9:     	cmpq	$0x1f, %rax
1425f2fcd:     	jbe	0x1425f33c0 <.text+0x25f23c0>
1425f2fd3:     	movq	0x8(%rbx), %rax
1425f2fd7:     	leaq	-0x3347ce(%rip), %rcx   # 0x1422be810 <.text+0x22bd810>
1425f2fde:     	jmp	0x1425f3057 <.text+0x25f2057>
1425f2fe0:     	movq	0x190(%rbp), %rbx
1425f2fe7:     	testq	%rbx, %rbx
1425f2fea:     	jne	0x1425f2ff3 <.text+0x25f1ff3>
1425f2fec:     	movq	0x198(%rbp), %rbx
1425f2ff3:     	movq	0x1a8(%rbp), %r12
1425f2ffa:     	movq	(%rbx), %rax
1425f2ffd:     	movq	0x10(%rbx), %rsi
1425f3001:     	subq	%rsi, %rax
1425f3004:     	cmpq	$0x1f, %rax
1425f3008:     	jbe	0x1425f33ec <.text+0x25f23ec>
1425f300e:     	movq	0x8(%rbx), %rax
1425f3012:     	leaq	-0x335209(%rip), %rcx   # 0x1422bde10 <.text+0x22bce10>
1425f3019:     	jmp	0x1425f31e6 <.text+0x25f21e6>
1425f301e:     	movq	0x190(%rbp), %rbx
1425f3025:     	testq	%rbx, %rbx
1425f3028:     	jne	0x1425f3031 <.text+0x25f2031>
1425f302a:     	movq	0x198(%rbp), %rbx
1425f3031:     	movq	0x1a8(%rbp), %r12
1425f3038:     	movq	(%rbx), %rax
1425f303b:     	movq	0x10(%rbx), %rsi
1425f303f:     	subq	%rsi, %rax
1425f3042:     	cmpq	$0x1f, %rax
1425f3046:     	jbe	0x1425f3418 <.text+0x25f2418>
1425f304c:     	movq	0x8(%rbx), %rax
1425f3050:     	leaq	-0x334ba7(%rip), %rcx   # 0x1422be4b0 <.text+0x22bd4b0>
1425f3057:     	movq	%rcx, (%rax,%rsi)
1425f305b:     	leaq	-0x333692(%rip), %rcx   # 0x1422bf9d0 <.text+0x22be9d0>
1425f3062:     	movq	%rcx, 0x8(%rax,%rsi)
1425f3067:     	movq	%r12, 0x10(%rax,%rsi)
1425f306c:     	movl	0x1c0(%rbp), %ecx
1425f3072:     	movl	%ecx, 0x18(%rax,%rsi)
1425f3076:     	movb	$0x0, 0x1c(%rax,%rsi)
1425f307b:     	jmp	0x1425f3200 <.text+0x25f2200>
1425f3080:     	movq	0x190(%rbp), %rbx
1425f3087:     	testq	%rbx, %rbx
1425f308a:     	jne	0x1425f3093 <.text+0x25f2093>
1425f308c:     	movq	0x198(%rbp), %rbx
1425f3093:     	movq	0x1a8(%rbp), %r12
1425f309a:     	movq	(%rbx), %rax
1425f309d:     	movq	0x10(%rbx), %rsi
1425f30a1:     	subq	%rsi, %rax
1425f30a4:     	cmpq	$0x1f, %rax
1425f30a8:     	jbe	0x1425f3444 <.text+0x25f2444>
1425f30ae:     	movq	0x8(%rbx), %rax
1425f30b2:     	leaq	-0x3359e9(%rip), %rcx   # 0x1422bd6d0 <.text+0x22bc6d0>
1425f30b9:     	movq	%rcx, (%rax,%rsi)
1425f30bd:     	leaq	-0x3336f4(%rip), %rcx   # 0x1422bf9d0 <.text+0x22be9d0>
1425f30c4:     	movq	%rcx, 0x8(%rax,%rsi)
1425f30c9:     	movq	%r12, 0x10(%rax,%rsi)
1425f30ce:     	movb	$0x0, 0x18(%rax,%rsi)
1425f30d3:     	addq	$0x20, %rsi
1425f30d7:     	movq	%rsi, 0x10(%rbx)
1425f30db:     	movq	0x70(%rbp), %rsi
1425f30df:     	cmpq	0x60(%rbp), %rsi
1425f30e3:     	movq	0x120(%rbp), %rbx
1425f30ea:     	jne	0x1425f2c00 <.text+0x25f1c00>
1425f30f0:     	jmp	0x1425f33a4 <.text+0x25f23a4>
1425f30f5:     	movq	0x190(%rbp), %rbx
1425f30fc:     	testq	%rbx, %rbx
1425f30ff:     	jne	0x1425f3108 <.text+0x25f2108>
1425f3101:     	movq	0x198(%rbp), %rbx
1425f3108:     	movq	0x1a8(%rbp), %r12
1425f310f:     	movq	(%rbx), %rax
1425f3112:     	movq	0x10(%rbx), %rsi
1425f3116:     	subq	%rsi, %rax
1425f3119:     	cmpq	$0x1f, %rax
1425f311d:     	jbe	0x1425f3470 <.text+0x25f2470>
1425f3123:     	movq	0x8(%rbx), %rax
1425f3127:     	leaq	-0x334d6e(%rip), %rcx   # 0x1422be3c0 <.text+0x22bd3c0>
1425f312e:     	jmp	0x1425f31e6 <.text+0x25f21e6>
1425f3133:     	movq	0x190(%rbp), %rbx
1425f313a:     	testq	%rbx, %rbx
1425f313d:     	jne	0x1425f3146 <.text+0x25f2146>
1425f313f:     	movq	0x198(%rbp), %rbx
1425f3146:     	movq	0x1a8(%rbp), %r12
1425f314d:     	movq	(%rbx), %rax
1425f3150:     	movq	0x10(%rbx), %rsi
1425f3154:     	subq	%rsi, %rax
1425f3157:     	cmpq	$0x1f, %rax
1425f315b:     	jbe	0x1425f349c <.text+0x25f249c>
1425f3161:     	movq	0x8(%rbx), %rax
1425f3165:     	leaq	-0x3351ec(%rip), %rcx   # 0x1422bdf80 <.text+0x22bcf80>
1425f316c:     	jmp	0x1425f31e6 <.text+0x25f21e6>
1425f316e:     	movq	0x190(%rbp), %r13
1425f3175:     	testq	%r13, %r13
1425f3178:     	jne	0x1425f3181 <.text+0x25f2181>
1425f317a:     	movq	0x198(%rbp), %r13
1425f3181:     	movq	0x1a8(%rbp), %r12
1425f3188:     	movq	(%r13), %rax
1425f318c:     	movq	0x10(%r13), %rsi
1425f3190:     	subq	%rsi, %rax
1425f3193:     	cmpq	$0x1f, %rax
1425f3197:     	jbe	0x1425f34f4 <.text+0x25f24f4>
1425f319d:     	movq	0x8(%r13), %rax
1425f31a1:     	leaq	-0x335bc8(%rip), %rcx   # 0x1422bd5e0 <.text+0x22bc5e0>
1425f31a8:     	jmp	0x1425f3297 <.text+0x25f2297>
1425f31ad:     	movq	0x190(%rbp), %rbx
1425f31b4:     	testq	%rbx, %rbx
1425f31b7:     	jne	0x1425f31c0 <.text+0x25f21c0>
1425f31b9:     	movq	0x198(%rbp), %rbx
1425f31c0:     	movq	0x1a8(%rbp), %r12
1425f31c7:     	movq	(%rbx), %rax
1425f31ca:     	movq	0x10(%rbx), %rsi
1425f31ce:     	subq	%rsi, %rax
1425f31d1:     	cmpq	$0x1f, %rax
1425f31d5:     	jbe	0x1425f3520 <.text+0x25f2520>
1425f31db:     	movq	0x8(%rbx), %rax
1425f31df:     	leaq	-0x335176(%rip), %rcx   # 0x1422be070 <.text+0x22bd070>
1425f31e6:     	movq	%rcx, (%rax,%rsi)
1425f31ea:     	leaq	-0x333821(%rip), %rcx   # 0x1422bf9d0 <.text+0x22be9d0>
1425f31f1:     	movq	%rcx, 0x8(%rax,%rsi)
1425f31f6:     	movq	%r12, 0x10(%rax,%rsi)
1425f31fb:     	movb	$0x0, 0x18(%rax,%rsi)
1425f3200:     	addq	$0x20, %rsi
1425f3204:     	movq	%rsi, 0x10(%rbx)
1425f3208:     	movq	0x170(%rbp), %rsi
1425f320f:     	cmpq	0x160(%rbp), %rsi
1425f3216:     	je	0x1425f3360 <.text+0x25f2360>
1425f321c:     	jmp	0x1425f337a <.text+0x25f237a>
1425f3221:     	movq	0x190(%rbp), %r13
1425f3228:     	testq	%r13, %r13
1425f322b:     	jne	0x1425f3234 <.text+0x25f2234>
1425f322d:     	movq	0x198(%rbp), %r13
1425f3234:     	movq	0x1a8(%rbp), %r12
1425f323b:     	movq	(%r13), %rax
1425f323f:     	movq	0x10(%r13), %rsi
1425f3243:     	subq	%rsi, %rax
1425f3246:     	cmpq	$0x1f, %rax
1425f324a:     	jbe	0x1425f354c <.text+0x25f254c>
1425f3250:     	movq	0x8(%r13), %rax
1425f3254:     	leaq	-0x334b3b(%rip), %rcx   # 0x1422be720 <.text+0x22bd720>
1425f325b:     	jmp	0x1425f3297 <.text+0x25f2297>
1425f325d:     	movq	0x190(%rbp), %r13
1425f3264:     	testq	%r13, %r13
1425f3267:     	jne	0x1425f3270 <.text+0x25f2270>
1425f3269:     	movq	0x198(%rbp), %r13
1425f3270:     	movq	0x1a8(%rbp), %r12
1425f3277:     	movq	(%r13), %rax
1425f327b:     	movq	0x10(%r13), %rsi
1425f327f:     	subq	%rsi, %rax
1425f3282:     	cmpq	$0x1f, %rax
1425f3286:     	jbe	0x1425f3578 <.text+0x25f2578>
1425f328c:     	movq	0x8(%r13), %rax
1425f3290:     	leaq	-0x335967(%rip), %rcx   # 0x1422bd930 <.text+0x22bc930>
1425f3297:     	movq	%rcx, (%rax,%rsi)
1425f329b:     	leaq	-0x3338d2(%rip), %rcx   # 0x1422bf9d0 <.text+0x22be9d0>
1425f32a2:     	movq	%rcx, 0x8(%rax,%rsi)
1425f32a7:     	movq	%r12, 0x10(%rax,%rsi)
1425f32ac:     	movb	$0x0, 0x18(%rax,%rsi)
1425f32b1:     	movzbl	0x1c6(%rbp), %ecx
1425f32b8:     	movb	%cl, 0x19(%rax,%rsi)
1425f32bc:     	addq	$0x20, %rsi
1425f32c0:     	jmp	0x1425f334c <.text+0x25f234c>
1425f32c5:     	movq	0x190(%rbp), %r13
1425f32cc:     	testq	%r13, %r13
1425f32cf:     	jne	0x1425f32d8 <.text+0x25f22d8>
1425f32d1:     	movq	0x198(%rbp), %r13
1425f32d8:     	movq	0x1a8(%rbp), %r12
1425f32df:     	movq	(%r13), %rax
1425f32e3:     	movq	0x10(%r13), %rsi
1425f32e7:     	subq	%rsi, %rax
1425f32ea:     	cmpq	$0x27, %rax
1425f32ee:     	jbe	0x1425f35a4 <.text+0x25f25a4>
1425f32f4:     	movq	0x8(%r13), %rax
1425f32f8:     	leaq	-0x3356df(%rip), %rcx   # 0x1422bdc20 <.text+0x22bcc20>
1425f32ff:     	movq	%rcx, (%rax,%rsi)
1425f3303:     	leaq	-0x33393a(%rip), %rcx   # 0x1422bf9d0 <.text+0x22be9d0>
1425f330a:     	movq	%rcx, 0x8(%rax,%rsi)
1425f330f:     	movq	%r12, 0x10(%rax,%rsi)
1425f3314:     	movb	%bl, 0x18(%rax,%rsi)
1425f3318:     	movzbl	0x1c6(%rbp), %ecx
1425f331f:     	movb	%cl, 0x19(%rax,%rsi)
1425f3323:     	movzwl	0x1b6(%rbp), %ecx
1425f332a:     	movw	%cx, 0x1a(%rax,%rsi)
1425f332f:     	movl	0x1c0(%rbp), %ecx
1425f3335:     	movl	%ecx, 0x1c(%rax,%rsi)
1425f3339:     	movl	0x184(%rbp), %ecx
1425f333f:     	movl	%ecx, 0x20(%rax,%rsi)
1425f3343:     	movb	$0x0, 0x24(%rax,%rsi)
1425f3348:     	addq	$0x28, %rsi
1425f334c:     	movq	%rsi, 0x10(%r13)
1425f3350:     	movq	0x170(%rbp), %rsi
1425f3357:     	cmpq	0x160(%rbp), %rsi
1425f335e:     	jne	0x1425f337a <.text+0x25f237a>
1425f3360:     	movb	$0x0, 0x1c7(%rbp)
1425f3367:     	leaq	0x160(%rbp), %rcx
1425f336e:     	leaq	0xb43b63(%rip), %rdx    # 0x143136ed8
1425f3375:     	callq	0x140b81df0 <.text+0xb80df0>
1425f337a:     	movq	0x168(%rbp), %rax
1425f3381:     	movq	%r12, (%rax,%rsi,8)
1425f3385:     	incq	%rsi
1425f3388:     	movq	%rsi, 0x170(%rbp)
1425f338f:     	movq	0x70(%rbp), %rsi
1425f3393:     	cmpq	0x60(%rbp), %rsi
1425f3397:     	movq	0x120(%rbp), %rbx
1425f339e:     	jne	0x1425f2c00 <.text+0x25f1c00>
1425f33a4:     	movb	$0x0, 0x1c7(%rbp)
1425f33ab:     	leaq	0x60(%rbp), %rcx
1425f33af:     	leaq	0xb43b3a(%rip), %rdx    # 0x143136ef0
1425f33b6:     	callq	0x140b81df0 <.text+0xb80df0>
1425f33bb:     	jmp	0x1425f2c00 <.text+0x25f1c00>
1425f33c0:     	movb	$0x0, 0x1c7(%rbp)
1425f33c7:     	movq	$0x1, 0x20(%rsp)
1425f33d0:     	movl	$0x20, %r8d
1425f33d6:     	movl	$0x1, %r9d
1425f33dc:     	movq	%rbx, %rcx
1425f33df:     	movq	%rsi, %rdx
1425f33e2:     	callq	0x1429107f0 <.text+0x290f7f0>
1425f33e7:     	jmp	0x1425f2fd3 <.text+0x25f1fd3>
1425f33ec:     	movb	$0x0, 0x1c7(%rbp)
1425f33f3:     	movq	$0x1, 0x20(%rsp)
1425f33fc:     	movl	$0x20, %r8d
1425f3402:     	movl	$0x1, %r9d
1425f3408:     	movq	%rbx, %rcx
1425f340b:     	movq	%rsi, %rdx
1425f340e:     	callq	0x1429107f0 <.text+0x290f7f0>
1425f3413:     	jmp	0x1425f300e <.text+0x25f200e>
1425f3418:     	movb	$0x0, 0x1c7(%rbp)
1425f341f:     	movq	$0x1, 0x20(%rsp)
1425f3428:     	movl	$0x20, %r8d
1425f342e:     	movl	$0x1, %r9d
1425f3434:     	movq	%rbx, %rcx
1425f3437:     	movq	%rsi, %rdx
1425f343a:     	callq	0x1429107f0 <.text+0x290f7f0>
1425f343f:     	jmp	0x1425f304c <.text+0x25f204c>
1425f3444:     	movb	$0x0, 0x1c7(%rbp)
1425f344b:     	movq	$0x1, 0x20(%rsp)
1425f3454:     	movl	$0x20, %r8d
1425f345a:     	movl	$0x1, %r9d
1425f3460:     	movq	%rbx, %rcx
1425f3463:     	movq	%rsi, %rdx
1425f3466:     	callq	0x1429107f0 <.text+0x290f7f0>
1425f346b:     	jmp	0x1425f30ae <.text+0x25f20ae>
1425f3470:     	movb	$0x0, 0x1c7(%rbp)
1425f3477:     	movq	$0x1, 0x20(%rsp)
1425f3480:     	movl	$0x20, %r8d
1425f3486:     	movl	$0x1, %r9d
1425f348c:     	movq	%rbx, %rcx
1425f348f:     	movq	%rsi, %rdx
1425f3492:     	callq	0x1429107f0 <.text+0x290f7f0>
1425f3497:     	jmp	0x1425f3123 <.text+0x25f2123>
1425f349c:     	movb	$0x0, 0x1c7(%rbp)
1425f34a3:     	movq	$0x1, 0x20(%rsp)
1425f34ac:     	movl	$0x20, %r8d
1425f34b2:     	movl	$0x1, %r9d
1425f34b8:     	movq	%rbx, %rcx
1425f34bb:     	movq	%rsi, %rdx
1425f34be:     	callq	0x1429107f0 <.text+0x290f7f0>
1425f34c3:     	jmp	0x1425f3161 <.text+0x25f2161>
1425f34c8:     	movb	$0x0, 0x1c7(%rbp)
1425f34cf:     	movq	$0x1, 0x20(%rsp)
1425f34d8:     	movl	$0x20, %r8d
1425f34de:     	movl	$0x1, %r9d
1425f34e4:     	movq	%rbx, %rcx
1425f34e7:     	movq	%rsi, %rdx
1425f34ea:     	callq	0x1429107f0 <.text+0x290f7f0>
1425f34ef:     	jmp	0x1425f2f95 <.text+0x25f1f95>
1425f34f4:     	movb	$0x0, 0x1c7(%rbp)
1425f34fb:     	movq	$0x1, 0x20(%rsp)
1425f3504:     	movl	$0x20, %r8d
1425f350a:     	movl	$0x1, %r9d
1425f3510:     	movq	%r13, %rcx
1425f3513:     	movq	%rsi, %rdx
1425f3516:     	callq	0x1429107f0 <.text+0x290f7f0>
1425f351b:     	jmp	0x1425f319d <.text+0x25f219d>
1425f3520:     	movb	$0x0, 0x1c7(%rbp)
1425f3527:     	movq	$0x1, 0x20(%rsp)
1425f3530:     	movl	$0x20, %r8d
1425f3536:     	movl	$0x1, %r9d
1425f353c:     	movq	%rbx, %rcx
1425f353f:     	movq	%rsi, %rdx
1425f3542:     	callq	0x1429107f0 <.text+0x290f7f0>
1425f3547:     	jmp	0x1425f31db <.text+0x25f21db>
1425f354c:     	movb	$0x0, 0x1c7(%rbp)
1425f3553:     	movq	$0x1, 0x20(%rsp)
1425f355c:     	movl	$0x20, %r8d
1425f3562:     	movl	$0x1, %r9d
1425f3568:     	movq	%r13, %rcx
1425f356b:     	movq	%rsi, %rdx
1425f356e:     	callq	0x1429107f0 <.text+0x290f7f0>
1425f3573:     	jmp	0x1425f3250 <.text+0x25f2250>
1425f3578:     	movb	$0x0, 0x1c7(%rbp)
1425f357f:     	movq	$0x1, 0x20(%rsp)
1425f3588:     	movl	$0x20, %r8d
1425f358e:     	movl	$0x1, %r9d
1425f3594:     	movq	%r13, %rcx
1425f3597:     	movq	%rsi, %rdx
1425f359a:     	callq	0x1429107f0 <.text+0x290f7f0>
1425f359f:     	jmp	0x1425f328c <.text+0x25f228c>
1425f35a4:     	movb	$0x0, 0x1c7(%rbp)
1425f35ab:     	movq	$0x1, 0x20(%rsp)
1425f35b4:     	movl	$0x28, %r8d
1425f35ba:     	movl	$0x1, %r9d
1425f35c0:     	movq	%r13, %rcx
1425f35c3:     	movq	%rsi, %rdx
1425f35c6:     	callq	0x1429107f0 <.text+0x290f7f0>
1425f35cb:     	jmp	0x1425f32f4 <.text+0x25f22f4>
1425f35d0:     	movq	0x130(%rbp), %r8
1425f35d7:     	movq	%r8, %rax
1425f35da:     	subq	%rdi, %rax
1425f35dd:     	movabsq	$-0x3333333333333333, %rcx # imm = 0xCCCCCCCCCCCCCCCD
1425f35e7:     	mulq	%rcx
1425f35ea:     	cmpq	%rdi, %r8
1425f35ed:     	movq	0x128(%rbp), %r15
1425f35f4:     	je	0x1425f3632 <.text+0x25f2632>
1425f35f6:     	movq	%rdx, %rbx
1425f35f9:     	shrq	$0x5, %rbx
1425f35fd:     	addq	$0x8, %rdi
1425f3601:     	jmp	0x1425f3619 <.text+0x25f2619>
1425f3603:     	nopw	%cs:(%rax,%rax)
1425f3610:     	addq	$0x28, %rdi
1425f3614:     	decq	%rbx
1425f3617:     	je	0x1425f3632 <.text+0x25f2632>
1425f3619:     	movq	-0x8(%rdi), %rdx
1425f361d:     	testq	%rdx, %rdx
1425f3620:     	je	0x1425f3610 <.text+0x25f2610>
1425f3622:     	movq	(%rdi), %rcx
1425f3625:     	movl	$0x1, %r8d
1425f362b:     	callq	0x140613c20 <.text+0x612c20>
1425f3630:     	jmp	0x1425f3610 <.text+0x25f2610>
1425f3632:     	leaq	-0x50(%rbp), %rsi
1425f3636:     	leaq	0x50(%rbp), %rdx
1425f363a:     	movl	$0xa0, %r8d
1425f3640:     	movq	%rsi, %rcx
1425f3643:     	callq	0x142923c00 <.text+0x2922c00>
1425f3648:     	movb	$0x0, 0x1c5(%rbp)
1425f364f:     	leaq	0xb43852(%rip), %r9     # 0x143136ea8
1425f3656:     	leaq	0xf0(%rbp), %rcx
1425f365d:     	movq	0x320(%rbp), %rdx
1425f3664:     	movq	%rsi, %r8
1425f3667:     	callq	0x1422bf2c0 <.text+0x22be2c0>
1425f366c:     	movq	0x110(%rbp), %rax
1425f3673:     	movq	0x170(%rbp), %rcx
1425f367a:     	movq	%rcx, 0x10(%r15)
1425f367e:     	movups	0x160(%rbp), %xmm0
1425f3685:     	movups	%xmm0, (%r15)
1425f3689:     	movq	%rax, 0x18(%r15)
1425f368d:     	movq	0x300(%rbp), %r14
1425f3694:     	movq	0x8(%r14), %rdi
1425f3698:     	movq	0x10(%r14), %rsi
1425f369c:     	testq	%rsi, %rsi
1425f369f:     	je	0x1425f36d2 <.text+0x25f26d2>
1425f36a1:     	leaq	0x8(%rdi), %rbx
1425f36a5:     	jmp	0x1425f36b9 <.text+0x25f26b9>
1425f36a7:     	nopw	(%rax,%rax)
1425f36b0:     	addq	$0x28, %rbx
1425f36b4:     	decq	%rsi
1425f36b7:     	je	0x1425f36d2 <.text+0x25f26d2>
1425f36b9:     	movq	-0x8(%rbx), %rdx
1425f36bd:     	testq	%rdx, %rdx
1425f36c0:     	je	0x1425f36b0 <.text+0x25f26b0>
1425f36c2:     	movq	(%rbx), %rcx
1425f36c5:     	movl	$0x1, %r8d
1425f36cb:     	callq	0x140613c20 <.text+0x612c20>
1425f36d0:     	jmp	0x1425f36b0 <.text+0x25f26b0>
1425f36d2:     	movq	(%r14), %rax
1425f36d5:     	testq	%rax, %rax
1425f36d8:     	je	0x1425f36f0 <.text+0x25f26f0>
1425f36da:     	shlq	$0x3, %rax
1425f36de:     	leaq	(%rax,%rax,4), %rdx
1425f36e2:     	movl	$0x8, %r8d
1425f36e8:     	movq	%rdi, %rcx
1425f36eb:     	callq	0x140613c20 <.text+0x612c20>
1425f36f0:     	movq	%r15, %rax
1425f36f3:     	movaps	0x1d0(%rbp), %xmm6
1425f36fa:     	movaps	0x1e0(%rbp), %xmm7
1425f3701:     	movaps	0x1f0(%rbp), %xmm8
1425f3709:     	movaps	0x200(%rbp), %xmm9
1425f3711:     	movaps	0x210(%rbp), %xmm10
1425f3719:     	movaps	0x220(%rbp), %xmm11
1425f3721:     	movaps	0x230(%rbp), %xmm12
1425f3729:     	movaps	0x240(%rbp), %xmm13
1425f3731:     	movaps	0x250(%rbp), %xmm14
1425f3739:     	movaps	0x260(%rbp), %xmm15
1425f3741:     	addq	$0x2f8, %rsp            # imm = 0x2F8
1425f3748:     	popq	%rbx
1425f3749:     	popq	%rdi
1425f374a:     	popq	%rsi
1425f374b:     	popq	%r12
1425f374d:     	popq	%r13
1425f374f:     	popq	%r14
1425f3751:     	popq	%r15
1425f3753:     	popq	%rbp
1425f3754:     	retq
1425f3755:     	leaq	0xb43704(%rip), %r8     # 0x143136e60
1425f375c:     	movq	%rsi, %rcx
1425f375f:     	movq	%rdi, %rdx
1425f3762:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1425f3767:     	ud2
1425f3769:     	nopl	(%rax)
1425f3770:     	movq	%rdx, 0x10(%rsp)
1425f3775:     	pushq	%rbp
1425f3776:     	pushq	%r15
1425f3778:     	pushq	%r14
1425f377a:     	pushq	%r13
1425f377c:     	pushq	%r12
1425f377e:     	pushq	%rsi
1425f377f:     	pushq	%rdi
1425f3780:     	pushq	%rbx
1425f3781:     	subq	$0xc8, %rsp
1425f3788:     	leaq	0x80(%rdx), %rbp
1425f378f:     	movaps	%xmm15, 0x20(%rsp)
1425f3795:     	movaps	%xmm14, 0x30(%rsp)
1425f379b:     	movaps	%xmm13, 0x40(%rsp)
1425f37a1:     	movaps	%xmm12, 0x50(%rsp)
1425f37a7:     	movaps	%xmm11, 0x60(%rsp)
1425f37ad:     	movaps	%xmm10, 0x70(%rsp)
1425f37b3:     	movaps	%xmm9, 0x80(%rsp)
1425f37bc:     	movaps	%xmm8, 0x90(%rsp)
1425f37c5:     	movaps	%xmm7, 0xa0(%rsp)
1425f37cd:     	movaps	%xmm6, 0xb0(%rsp)
1425f37d5:     	movq	0x300(%rbp), %rcx
1425f37dc:     	callq	0x14014a860 <.text+0x149860>
1425f37e1:     	movaps	0xb0(%rsp), %xmm6
1425f37e9:     	movaps	0xa0(%rsp), %xmm7
1425f37f1:     	movaps	0x90(%rsp), %xmm8
1425f37fa:     	movaps	0x80(%rsp), %xmm9
1425f3803:     	movaps	0x70(%rsp), %xmm10
1425f3809:     	movaps	0x60(%rsp), %xmm11
1425f380f:     	movaps	0x50(%rsp), %xmm12
1425f3815:     	movaps	0x40(%rsp), %xmm13
1425f381b:     	movaps	0x30(%rsp), %xmm14
1425f3821:     	movaps	0x20(%rsp), %xmm15
1425f3827:     	addq	$0xc8, %rsp
1425f382e:     	popq	%rbx
1425f382f:     	popq	%rdi
1425f3830:     	popq	%rsi
1425f3831:     	popq	%r12
1425f3833:     	popq	%r13
1425f3835:     	popq	%r14
1425f3837:     	popq	%r15
1425f3839:     	popq	%rbp
1425f383a:     	retq
1425f383b:     	nopl	(%rax,%rax)
1425f3840:     	movq	%rdx, 0x10(%rsp)
1425f3845:     	pushq	%rbp
1425f3846:     	pushq	%r15
1425f3848:     	pushq	%r14
1425f384a:     	pushq	%r13
1425f384c:     	pushq	%r12
1425f384e:     	pushq	%rsi
1425f384f:     	pushq	%rdi
1425f3850:     	pushq	%rbx
1425f3851:     	subq	$0xc8, %rsp
1425f3858:     	leaq	0x80(%rdx), %rbp
1425f385f:     	movaps	%xmm15, 0x20(%rsp)
1425f3865:     	movaps	%xmm14, 0x30(%rsp)
1425f386b:     	movaps	%xmm13, 0x40(%rsp)
1425f3871:     	movaps	%xmm12, 0x50(%rsp)
1425f3877:     	movaps	%xmm11, 0x60(%rsp)
1425f387d:     	movaps	%xmm10, 0x70(%rsp)
1425f3883:     	movaps	%xmm9, 0x80(%rsp)
1425f388c:     	movaps	%xmm8, 0x90(%rsp)
1425f3895:     	movaps	%xmm7, 0xa0(%rsp)
1425f389d:     	movaps	%xmm6, 0xb0(%rsp)
1425f38a5:     	movq	0x160(%rbp), %rdx
1425f38ac:     	testq	%rdx, %rdx
1425f38af:     	je	0x1425f38c7 <.text+0x25f28c7>
1425f38b1:     	movq	0x168(%rbp), %rcx
1425f38b8:     	shlq	$0x3, %rdx
1425f38bc:     	movl	$0x8, %r8d
1425f38c2:     	callq	0x140613c20 <.text+0x612c20>
1425f38c7:     	movaps	0xb0(%rsp), %xmm6
1425f38cf:     	movaps	0xa0(%rsp), %xmm7
1425f38d7:     	movaps	0x90(%rsp), %xmm8
1425f38e0:     	movaps	0x80(%rsp), %xmm9
1425f38e9:     	movaps	0x70(%rsp), %xmm10
1425f38ef:     	movaps	0x60(%rsp), %xmm11
1425f38f5:     	movaps	0x50(%rsp), %xmm12
1425f38fb:     	movaps	0x40(%rsp), %xmm13
1425f3901:     	movaps	0x30(%rsp), %xmm14
1425f3907:     	movaps	0x20(%rsp), %xmm15
1425f390d:     	addq	$0xc8, %rsp
1425f3914:     	popq	%rbx
1425f3915:     	popq	%rdi
1425f3916:     	popq	%rsi
1425f3917:     	popq	%r12
1425f3919:     	popq	%r13
1425f391b:     	popq	%r14
1425f391d:     	popq	%r15
1425f391f:     	popq	%rbp
1425f3920:     	retq
1425f3921:     	nopw	%cs:(%rax,%rax)
1425f3930:     	movq	%rdx, 0x10(%rsp)
1425f3935:     	pushq	%rbp
1425f3936:     	pushq	%r15
1425f3938:     	pushq	%r14
1425f393a:     	pushq	%r13
1425f393c:     	pushq	%r12
1425f393e:     	pushq	%rsi
1425f393f:     	pushq	%rdi
1425f3940:     	pushq	%rbx
1425f3941:     	subq	$0xc8, %rsp
1425f3948:     	leaq	0x80(%rdx), %rbp
1425f394f:     	movaps	%xmm15, 0x20(%rsp)
1425f3955:     	movaps	%xmm14, 0x30(%rsp)
1425f395b:     	movaps	%xmm13, 0x40(%rsp)
1425f3961:     	movaps	%xmm12, 0x50(%rsp)
1425f3967:     	movaps	%xmm11, 0x60(%rsp)
1425f396d:     	movaps	%xmm10, 0x70(%rsp)
1425f3973:     	movaps	%xmm9, 0x80(%rsp)
1425f397c:     	movaps	%xmm8, 0x90(%rsp)
1425f3985:     	movaps	%xmm7, 0xa0(%rsp)
1425f398d:     	movaps	%xmm6, 0xb0(%rsp)
1425f3995:     	leaq	-0x50(%rbp), %rcx
1425f3999:     	callq	0x14014a860 <.text+0x149860>
1425f399e:     	movaps	0xb0(%rsp), %xmm6
1425f39a6:     	movaps	0xa0(%rsp), %xmm7
1425f39ae:     	movaps	0x90(%rsp), %xmm8
1425f39b7:     	movaps	0x80(%rsp), %xmm9
1425f39c0:     	movaps	0x70(%rsp), %xmm10
1425f39c6:     	movaps	0x60(%rsp), %xmm11
1425f39cc:     	movaps	0x50(%rsp), %xmm12
1425f39d2:     	movaps	0x40(%rsp), %xmm13
1425f39d8:     	movaps	0x30(%rsp), %xmm14
1425f39de:     	movaps	0x20(%rsp), %xmm15
1425f39e4:     	addq	$0xc8, %rsp
1425f39eb:     	popq	%rbx
1425f39ec:     	popq	%rdi
1425f39ed:     	popq	%rsi
1425f39ee:     	popq	%r12
1425f39f0:     	popq	%r13
1425f39f2:     	popq	%r14
1425f39f4:     	popq	%r15
1425f39f6:     	popq	%rbp
1425f39f7:     	retq
1425f39f8:     	nopl	(%rax,%rax)
1425f3a00:     	movq	%rdx, 0x10(%rsp)
1425f3a05:     	pushq	%rbp
1425f3a06:     	pushq	%r15
1425f3a08:     	pushq	%r14
1425f3a0a:     	pushq	%r13
1425f3a0c:     	pushq	%r12
1425f3a0e:     	pushq	%rsi
1425f3a0f:     	pushq	%rdi
1425f3a10:     	pushq	%rbx
1425f3a11:     	subq	$0xc8, %rsp
1425f3a18:     	leaq	0x80(%rdx), %rbp
1425f3a1f:     	movaps	%xmm15, 0x20(%rsp)
1425f3a25:     	movaps	%xmm14, 0x30(%rsp)
1425f3a2b:     	movaps	%xmm13, 0x40(%rsp)
1425f3a31:     	movaps	%xmm12, 0x50(%rsp)
1425f3a37:     	movaps	%xmm11, 0x60(%rsp)
1425f3a3d:     	movaps	%xmm10, 0x70(%rsp)
1425f3a43:     	movaps	%xmm9, 0x80(%rsp)
1425f3a4c:     	movaps	%xmm8, 0x90(%rsp)
1425f3a55:     	movaps	%xmm7, 0xa0(%rsp)
1425f3a5d:     	movaps	%xmm6, 0xb0(%rsp)
1425f3a65:     	leaq	0xf0(%rbp), %rcx
1425f3a6c:     	callq	0x140ec30e0 <.text+0xec20e0>
1425f3a71:     	movb	$0x1, 0x1c5(%rbp)
1425f3a78:     	movaps	0xb0(%rsp), %xmm6
1425f3a80:     	movaps	0xa0(%rsp), %xmm7
1425f3a88:     	movaps	0x90(%rsp), %xmm8
1425f3a91:     	movaps	0x80(%rsp), %xmm9
1425f3a9a:     	movaps	0x70(%rsp), %xmm10
1425f3aa0:     	movaps	0x60(%rsp), %xmm11
1425f3aa6:     	movaps	0x50(%rsp), %xmm12
1425f3aac:     	movaps	0x40(%rsp), %xmm13
1425f3ab2:     	movaps	0x30(%rsp), %xmm14
1425f3ab8:     	movaps	0x20(%rsp), %xmm15
1425f3abe:     	addq	$0xc8, %rsp
1425f3ac5:     	popq	%rbx
1425f3ac6:     	popq	%rdi
1425f3ac7:     	popq	%rsi
1425f3ac8:     	popq	%r12
1425f3aca:     	popq	%r13
1425f3acc:     	popq	%r14
1425f3ace:     	popq	%r15
1425f3ad0:     	popq	%rbp
1425f3ad1:     	retq
1425f3ad2:     	nopw	%cs:(%rax,%rax)
1425f3ae0:     	movq	%rdx, 0x10(%rsp)
1425f3ae5:     	pushq	%rbp
1425f3ae6:     	pushq	%r15
1425f3ae8:     	pushq	%r14
1425f3aea:     	pushq	%r13
1425f3aec:     	pushq	%r12
1425f3aee:     	pushq	%rsi
1425f3aef:     	pushq	%rdi
1425f3af0:     	pushq	%rbx
1425f3af1:     	subq	$0xc8, %rsp
1425f3af8:     	leaq	0x80(%rdx), %rbp
1425f3aff:     	movaps	%xmm15, 0x20(%rsp)
1425f3b05:     	movaps	%xmm14, 0x30(%rsp)
1425f3b0b:     	movaps	%xmm13, 0x40(%rsp)
1425f3b11:     	movaps	%xmm12, 0x50(%rsp)
1425f3b17:     	movaps	%xmm11, 0x60(%rsp)
1425f3b1d:     	movaps	%xmm10, 0x70(%rsp)
1425f3b23:     	movaps	%xmm9, 0x80(%rsp)
1425f3b2c:     	movaps	%xmm8, 0x90(%rsp)
1425f3b35:     	movaps	%xmm7, 0xa0(%rsp)
1425f3b3d:     	movaps	%xmm6, 0xb0(%rsp)
1425f3b45:     	cmpb	$0x0, 0x1c7(%rbp)
1425f3b4c:     	je	0x1425f3bcb <.text+0x25f2bcb>
1425f3b4e:     	cmpq	$0x0, 0x178(%rbp)
1425f3b56:     	je	0x1425f3b71 <.text+0x25f2b71>
1425f3b58:     	movl	$0x1, %r8d
1425f3b5e:     	movq	0x138(%rbp), %rcx
1425f3b65:     	movq	0x178(%rbp), %rdx
1425f3b6c:     	callq	0x140613c20 <.text+0x612c20>
1425f3b71:     	movaps	0xb0(%rsp), %xmm6
1425f3b79:     	movaps	0xa0(%rsp), %xmm7
1425f3b81:     	movaps	0x90(%rsp), %xmm8
1425f3b8a:     	movaps	0x80(%rsp), %xmm9
1425f3b93:     	movaps	0x70(%rsp), %xmm10
1425f3b99:     	movaps	0x60(%rsp), %xmm11
1425f3b9f:     	movaps	0x50(%rsp), %xmm12
1425f3ba5:     	movaps	0x40(%rsp), %xmm13
1425f3bab:     	movaps	0x30(%rsp), %xmm14
1425f3bb1:     	movaps	0x20(%rsp), %xmm15
1425f3bb7:     	addq	$0xc8, %rsp
1425f3bbe:     	popq	%rbx
1425f3bbf:     	popq	%rdi
1425f3bc0:     	popq	%rsi
1425f3bc1:     	popq	%r12
1425f3bc3:     	popq	%r13
1425f3bc5:     	popq	%r14
1425f3bc7:     	popq	%r15
1425f3bc9:     	popq	%rbp
1425f3bca:     	retq
1425f3bcb:     	movaps	0xb0(%rsp), %xmm6
1425f3bd3:     	movaps	0xa0(%rsp), %xmm7
1425f3bdb:     	movaps	0x90(%rsp), %xmm8
1425f3be4:     	movaps	0x80(%rsp), %xmm9
1425f3bed:     	movaps	0x70(%rsp), %xmm10
1425f3bf3:     	movaps	0x60(%rsp), %xmm11
1425f3bf9:     	movaps	0x50(%rsp), %xmm12
1425f3bff:     	movaps	0x40(%rsp), %xmm13
1425f3c05:     	movaps	0x30(%rsp), %xmm14
1425f3c0b:     	movaps	0x20(%rsp), %xmm15
1425f3c11:     	addq	$0xc8, %rsp
1425f3c18:     	popq	%rbx
1425f3c19:     	popq	%rdi
1425f3c1a:     	popq	%rsi
1425f3c1b:     	popq	%r12
1425f3c1d:     	popq	%r13
1425f3c1f:     	popq	%r14
1425f3c21:     	popq	%r15
1425f3c23:     	popq	%rbp
1425f3c24:     	retq
1425f3c25:     	nopw	%cs:(%rax,%rax)
1425f3c30:     	movq	%rdx, 0x10(%rsp)
1425f3c35:     	pushq	%rbp
1425f3c36:     	pushq	%r15
1425f3c38:     	pushq	%r14
1425f3c3a:     	pushq	%r13
1425f3c3c:     	pushq	%r12
1425f3c3e:     	pushq	%rsi
1425f3c3f:     	pushq	%rdi
1425f3c40:     	pushq	%rbx
1425f3c41:     	subq	$0xc8, %rsp
1425f3c48:     	leaq	0x80(%rdx), %rbp
1425f3c4f:     	movaps	%xmm15, 0x20(%rsp)
1425f3c55:     	movaps	%xmm14, 0x30(%rsp)
1425f3c5b:     	movaps	%xmm13, 0x40(%rsp)
1425f3c61:     	movaps	%xmm12, 0x50(%rsp)
1425f3c67:     	movaps	%xmm11, 0x60(%rsp)
1425f3c6d:     	movaps	%xmm10, 0x70(%rsp)
1425f3c73:     	movaps	%xmm9, 0x80(%rsp)
1425f3c7c:     	movaps	%xmm8, 0x90(%rsp)
1425f3c85:     	movaps	%xmm7, 0xa0(%rsp)
1425f3c8d:     	movaps	%xmm6, 0xb0(%rsp)
1425f3c95:     	cmpb	$0x0, 0x1c5(%rbp)
1425f3c9c:     	je	0x1425f3d01 <.text+0x25f2d01>
1425f3c9e:     	leaq	0x50(%rbp), %rcx
1425f3ca2:     	callq	0x140ec4530 <.text+0xec3530>
1425f3ca7:     	movaps	0xb0(%rsp), %xmm6
1425f3caf:     	movaps	0xa0(%rsp), %xmm7
1425f3cb7:     	movaps	0x90(%rsp), %xmm8
1425f3cc0:     	movaps	0x80(%rsp), %xmm9
1425f3cc9:     	movaps	0x70(%rsp), %xmm10
1425f3ccf:     	movaps	0x60(%rsp), %xmm11
1425f3cd5:     	movaps	0x50(%rsp), %xmm12
1425f3cdb:     	movaps	0x40(%rsp), %xmm13
1425f3ce1:     	movaps	0x30(%rsp), %xmm14
1425f3ce7:     	movaps	0x20(%rsp), %xmm15
1425f3ced:     	addq	$0xc8, %rsp
1425f3cf4:     	popq	%rbx
1425f3cf5:     	popq	%rdi
1425f3cf6:     	popq	%rsi
1425f3cf7:     	popq	%r12
1425f3cf9:     	popq	%r13
1425f3cfb:     	popq	%r14
1425f3cfd:     	popq	%r15
1425f3cff:     	popq	%rbp
1425f3d00:     	retq
1425f3d01:     	movaps	0xb0(%rsp), %xmm6
1425f3d09:     	movaps	0xa0(%rsp), %xmm7
1425f3d11:     	movaps	0x90(%rsp), %xmm8
1425f3d1a:     	movaps	0x80(%rsp), %xmm9
1425f3d23:     	movaps	0x70(%rsp), %xmm10
1425f3d29:     	movaps	0x60(%rsp), %xmm11
1425f3d2f:     	movaps	0x50(%rsp), %xmm12
1425f3d35:     	movaps	0x40(%rsp), %xmm13
1425f3d3b:     	movaps	0x30(%rsp), %xmm14
1425f3d41:     	movaps	0x20(%rsp), %xmm15
1425f3d47:     	addq	$0xc8, %rsp
1425f3d4e:     	popq	%rbx
1425f3d4f:     	popq	%rdi
1425f3d50:     	popq	%rsi
1425f3d51:     	popq	%r12
1425f3d53:     	popq	%r13
1425f3d55:     	popq	%r14
1425f3d57:     	popq	%r15
1425f3d59:     	popq	%rbp
1425f3d5a:     	retq
