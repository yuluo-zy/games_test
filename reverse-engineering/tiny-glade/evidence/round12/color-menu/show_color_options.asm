
reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14229a7f0:     	pushq	%rbp
14229a7f1:     	pushq	%r15
14229a7f3:     	pushq	%r14
14229a7f5:     	pushq	%r13
14229a7f7:     	pushq	%r12
14229a7f9:     	pushq	%rsi
14229a7fa:     	pushq	%rdi
14229a7fb:     	pushq	%rbx
14229a7fc:     	subq	$0x338, %rsp            # imm = 0x338
14229a803:     	leaq	0x80(%rsp), %rbp
14229a80b:     	movaps	%xmm15, 0x2a0(%rbp)
14229a813:     	movaps	%xmm14, 0x290(%rbp)
14229a81b:     	movaps	%xmm13, 0x280(%rbp)
14229a823:     	movaps	%xmm12, 0x270(%rbp)
14229a82b:     	movaps	%xmm11, 0x260(%rbp)
14229a833:     	movaps	%xmm10, 0x250(%rbp)
14229a83b:     	movaps	%xmm9, 0x240(%rbp)
14229a843:     	movaps	%xmm8, 0x230(%rbp)
14229a84b:     	movaps	%xmm7, 0x220(%rbp)
14229a852:     	movaps	%xmm6, 0x210(%rbp)
14229a859:     	movq	$-0x2, 0x208(%rbp)
14229a864:     	movq	(%rcx), %rdi
14229a867:     	movq	0x8(%rcx), %rcx
14229a86b:     	movq	(%rdi), %r11
14229a86e:     	xorl	%r10d, %r10d
14229a871:     	movq	%r11, %rax
14229a874:     	subq	0x18(%rcx), %rax
14229a878:     	cmovbq	%r10, %rax
14229a87c:     	subq	0x38(%rcx), %r11
14229a880:     	cmovbq	%r10, %r11
14229a884:     	movq	%rax, %r12
14229a887:     	shlq	$0x5, %r12
14229a88b:     	addq	0x8(%rcx), %r12
14229a88f:     	movq	0x10(%rcx), %rbx
14229a893:     	subq	%rax, %rbx
14229a896:     	cmovbq	%r10, %rbx
14229a89a:     	movq	0x320(%rbp), %rax
14229a8a1:     	movl	$0x8, %esi
14229a8a6:     	cmovbq	%rsi, %r12
14229a8aa:     	movq	%r11, %r15
14229a8ad:     	shlq	$0x5, %r15
14229a8b1:     	addq	0x28(%rcx), %r15
14229a8b5:     	movq	0x30(%rcx), %r14
14229a8b9:     	subq	%r11, %r14
14229a8bc:     	cmovbq	%r10, %r14
14229a8c0:     	movq	0x340(%rbp), %r10
14229a8c7:     	movq	0x338(%rbp), %r11
14229a8ce:     	cmovbq	%rsi, %r15
14229a8d2:     	movq	0x40(%rcx), %rcx
14229a8d6:     	subq	%r14, %rcx
14229a8d9:     	subq	%rbx, %rcx
14229a8dc:     	movq	0x328(%rbp), %rsi
14229a8e3:     	movq	%rdi, 0x48(%rbp)
14229a8e7:     	movq	%rcx, (%rdi)
14229a8ea:     	shlq	$0x5, %rbx
14229a8ee:     	addq	%r12, %rbx
14229a8f1:     	movq	%rbx, 0x8(%rbp)
14229a8f5:     	shlq	$0x5, %r14
14229a8f9:     	movq	%r15, 0x50(%rbp)
14229a8fd:     	addq	%r15, %r14
14229a900:     	movq	%r14, 0x20(%rbp)
14229a904:     	movq	%r12, %r15
14229a907:     	movq	(%rsi), %rcx
14229a90a:     	movq	%rcx, 0x200(%rbp)
14229a911:     	movq	(%rdx), %rsi
14229a914:     	movq	(%r9), %rcx
14229a917:     	movq	%rcx, 0x90(%rbp)
14229a91e:     	movq	(%r11), %rcx
14229a921:     	movq	0x8(%r11), %rdx
14229a925:     	movq	%rdx, 0x128(%rbp)
14229a92c:     	movq	(%r10), %rdx
14229a92f:     	movq	%rdx, (%rbp)
14229a933:     	movq	(%r8), %rdx
14229a936:     	movq	%rdx, 0x28(%rbp)
14229a93a:     	movq	(%rax), %rdx
14229a93d:     	movq	%rdx, 0x88(%rbp)
14229a944:     	movq	0x8(%rax), %rax
14229a948:     	movq	%rax, 0x1b0(%rbp)
14229a94f:     	movq	0x348(%rbp), %rax
14229a956:     	movq	(%rax), %rax
14229a959:     	movq	%rax, 0x1c8(%rbp)
14229a960:     	movq	0x368(%rbp), %rax
14229a967:     	movq	(%rax), %rax
14229a96a:     	movq	%rax, 0x58(%rbp)
14229a96e:     	movq	0x360(%rbp), %rax
14229a975:     	movq	(%rax), %rax
14229a978:     	movq	%rax, 0x60(%rbp)
14229a97c:     	movq	0x350(%rbp), %rax
14229a983:     	movl	0x1c(%rax), %edx
14229a986:     	movl	%edx, 0x74(%rbp)
14229a989:     	movq	(%rax), %rdx
14229a98c:     	movq	%rdx, 0x10(%rbp)
14229a990:     	movq	0x10(%rax), %rax
14229a994:     	movq	%rax, 0x18(%rbp)
14229a998:     	movq	0x330(%rbp), %rax
14229a99f:     	movq	(%rax), %rax
14229a9a2:     	movq	%rax, 0x1f8(%rbp)
14229a9a9:     	movss	0x736f1a(%rip), %xmm13  # 0x1429d18cc
14229a9b2:     	movss	0x68af49(%rip), %xmm14  # 0x142925904
14229a9bb:     	movabsq	$0x73726f6c6f635f72, %rdi # imm = 0x73726F6C6F635F72
14229a9c5:     	movq	%rsi, 0x178(%rbp)
14229a9cc:     	movq	%rcx, 0x108(%rbp)
14229a9d3:     	jmp	0x14229a9f8 <.text+0x22999f8>
14229a9d5:     	nopw	%cs:(%rax,%rax)
14229a9e0:     	movq	0x1e0(%rbp), %r15
14229a9e7:     	movq	0x178(%rbp), %rsi
14229a9ee:     	movabsq	$0x73726f6c6f635f72, %rdi # imm = 0x73726F6C6F635F72
14229a9f8:     	testq	%r15, %r15
14229a9fb:     	je	0x14229aa30 <.text+0x2299a30>
14229a9fd:     	cmpq	0x8(%rbp), %r15
14229aa01:     	je	0x14229aa30 <.text+0x2299a30>
14229aa03:     	movq	0x48(%rbp), %rax
14229aa07:     	incq	(%rax)
14229aa0a:     	leaq	0x20(%r15), %rdx
14229aa0e:     	movzbl	(%r15), %eax
14229aa12:     	leaq	0xda2057(%rip), %rcx    # 0x14303ca70
14229aa19:     	movslq	(%rcx,%rax,4), %rax
14229aa1d:     	addq	%rcx, %rax
14229aa20:     	jmpq	*%rax
14229aa22:     	nopw	%cs:(%rax,%rax)
14229aa30:     	movq	0x50(%rbp), %rcx
14229aa34:     	testq	%rcx, %rcx
14229aa37:     	je	0x1422a0db7 <.text+0x229fdb7>
14229aa3d:     	cmpq	0x20(%rbp), %rcx
14229aa41:     	je	0x1422a0db7 <.text+0x229fdb7>
14229aa47:     	movq	0x48(%rbp), %rax
14229aa4b:     	incq	(%rax)
14229aa4e:     	movzbl	(%rcx), %eax
14229aa51:     	movq	%rcx, %r15
14229aa54:     	addq	$0x20, %rcx
14229aa58:     	leaq	0xda2011(%rip), %rdx    # 0x14303ca70
14229aa5f:     	movslq	(%rdx,%rax,4), %rax
14229aa63:     	addq	%rdx, %rax
14229aa66:     	xorl	%edx, %edx
14229aa68:     	movq	%rcx, 0x50(%rbp)
14229aa6c:     	jmpq	*%rax
14229aa6e:     	movq	%rdx, 0x1e0(%rbp)
14229aa75:     	leaq	0x110(%rbp), %rcx
14229aa7c:     	callq	0x1422acce0 <.text+0x22abce0>
14229aa81:     	jmp	0x14229e9e0 <.text+0x229d9e0>
14229aa86:     	movq	%rdx, 0x1e0(%rbp)
14229aa8d:     	movzbl	0x139a175(%rip), %eax   # 0x143634c09
14229aa94:     	movl	$0x168, %ecx            # imm = 0x168
14229aa99:     	movl	$0x8, %edx
14229aa9e:     	callq	0x140613c10 <.text+0x612c10>
14229aaa3:     	testq	%rax, %rax
14229aaa6:     	je	0x1422a0fdc <.text+0x229ffdc>
14229aaac:     	movq	%rax, %rsi
14229aaaf:     	movzbl	0x139a153(%rip), %eax   # 0x143634c09
14229aab6:     	movl	$0x16, %ecx
14229aabb:     	movl	$0x1, %edx
14229aac0:     	callq	0x140613c10 <.text+0x612c10>
14229aac5:     	testq	%rax, %rax
14229aac8:     	je	0x1422a1998 <.text+0x22a0998>
14229aace:     	movq	%rax, %rdi
14229aad1:     	movups	0xda1b58(%rip), %xmm0   # 0x14303c630
14229aad8:     	movups	%xmm0, (%rax)
14229aadb:     	movabsq	$0x776f6c6c65795f67, %rax # imm = 0x776F6C6C65795F67
14229aae5:     	movq	%rax, 0xe(%rdi)
14229aae9:     	movzbl	0x139a119(%rip), %eax   # 0x143634c09
14229aaf0:     	movl	$0x16, %ecx
14229aaf5:     	movl	$0x1, %edx
14229aafa:     	callq	0x140613c10 <.text+0x612c10>
14229aaff:     	testq	%rax, %rax
14229ab02:     	je	0x1422a147e <.text+0x22a047e>
14229ab08:     	movq	%rax, %rbx
14229ab0b:     	movups	0xda1b34(%rip), %xmm0   # 0x14303c646
14229ab12:     	movups	%xmm0, (%rax)
14229ab15:     	movabsq	$0x6465722f73776f64, %rax # imm = 0x6465722F73776F64
14229ab1f:     	movq	%rax, 0xe(%rbx)
14229ab23:     	movzbl	0x139a0df(%rip), %eax   # 0x143634c09
14229ab2a:     	movl	$0x19, %ecx
14229ab2f:     	movl	$0x1, %edx
14229ab34:     	callq	0x140613c10 <.text+0x612c10>
14229ab39:     	testq	%rax, %rax
14229ab3c:     	je	0x1422a14a7 <.text+0x22a04a7>
14229ab42:     	movq	%rax, %r14
14229ab45:     	movups	0xda18d0(%rip), %xmm0   # 0x14303c41c
14229ab4c:     	movups	%xmm0, 0x9(%rax)
14229ab50:     	movups	0xda18bc(%rip), %xmm0   # 0x14303c413
14229ab57:     	movups	%xmm0, (%rax)
14229ab5a:     	movzbl	0x139a0a8(%rip), %eax   # 0x143634c09
14229ab61:     	movl	$0x17, %ecx
14229ab66:     	movl	$0x1, %edx
14229ab6b:     	callq	0x140613c10 <.text+0x612c10>
14229ab70:     	testq	%rax, %rax
14229ab73:     	je	0x1422a14d7 <.text+0x22a04d7>
14229ab79:     	movq	%rax, %r12
14229ab7c:     	movups	0xda1ad9(%rip), %xmm0   # 0x14303c65c
14229ab83:     	movups	%xmm0, (%rax)
14229ab86:     	movabsq	$0x65756c622f73776f, %rax # imm = 0x65756C622F73776F
14229ab90:     	movq	%rax, 0xf(%r12)
14229ab95:     	movzbl	0x139a06d(%rip), %eax   # 0x143634c09
14229ab9c:     	movl	$0x1b, %ecx
14229aba1:     	movl	$0x1, %edx
14229aba6:     	callq	0x140613c10 <.text+0x612c10>
14229abab:     	testq	%rax, %rax
14229abae:     	je	0x1422a150e <.text+0x22a050e>
14229abb4:     	movq	%rax, %r13
14229abb7:     	movups	0xda1773(%rip), %xmm0   # 0x14303c331
14229abbe:     	movups	%xmm0, 0xb(%rax)
14229abc2:     	movups	0xda175d(%rip), %xmm0   # 0x14303c326
14229abc9:     	movups	%xmm0, (%rax)
14229abcc:     	movzbl	0x139a036(%rip), %eax   # 0x143634c09
14229abd3:     	movl	$0x5, %ecx
14229abd8:     	movl	$0x1, %edx
14229abdd:     	callq	0x140613c10 <.text+0x612c10>
14229abe2:     	testq	%rax, %rax
14229abe5:     	je	0x1422a154c <.text+0x22a054c>
14229abeb:     	movzbl	0xe9c165(%rip), %ecx    # 0x143136d57
14229abf2:     	movb	%cl, 0x4(%rax)
14229abf5:     	movl	0xe9c158(%rip), %ecx    # 0x143136d53
14229abfb:     	movq	%rax, 0x1e8(%rbp)
14229ac02:     	movl	%ecx, (%rax)
14229ac04:     	movzbl	0x1399ffe(%rip), %eax   # 0x143634c09
14229ac0b:     	movl	$0x12, %ecx
14229ac10:     	movl	$0x1, %edx
14229ac15:     	callq	0x140613c10 <.text+0x612c10>
14229ac1a:     	testq	%rax, %rax
14229ac1d:     	je	0x1422a1591 <.text+0x22a0591>
14229ac23:     	movq	%r15, 0x1f0(%rbp)
14229ac2a:     	movups	0xda1a42(%rip), %xmm0   # 0x14303c673
14229ac31:     	movups	%xmm0, (%rax)
14229ac34:     	movq	%rax, 0x1d8(%rbp)
14229ac3b:     	movw	$0x315f, 0x10(%rax)     # imm = 0x315F
14229ac41:     	movzbl	0x1399fc1(%rip), %eax   # 0x143634c09
14229ac48:     	movl	$0x12, %ecx
14229ac4d:     	movl	$0x1, %edx
14229ac52:     	callq	0x140613c10 <.text+0x612c10>
14229ac57:     	testq	%rax, %rax
14229ac5a:     	je	0x1422a17f3 <.text+0x22a07f3>
14229ac60:     	movups	0xda1a1e(%rip), %xmm0   # 0x14303c685
14229ac67:     	movups	%xmm0, (%rax)
14229ac6a:     	movw	$0x325f, 0x10(%rax)     # imm = 0x325F
14229ac70:     	movq	%rax, %r15
14229ac73:     	movzbl	0x1399f8f(%rip), %eax   # 0x143634c09
14229ac7a:     	movl	$0x12, %ecx
14229ac7f:     	movl	$0x1, %edx
14229ac84:     	callq	0x140613c10 <.text+0x612c10>
14229ac89:     	testq	%rax, %rax
14229ac8c:     	je	0x1422a12b7 <.text+0x22a02b7>
14229ac92:     	movups	0xda19fe(%rip), %xmm0   # 0x14303c697
14229ac99:     	movups	%xmm0, (%rax)
14229ac9c:     	movw	$0x335f, 0x10(%rax)     # imm = 0x335F
14229aca2:     	movq	$0x16, (%rsi)
14229aca9:     	movq	%rdi, 0x8(%rsi)
14229acad:     	movq	$0x16, 0x10(%rsi)
14229acb5:     	movb	$0x3, 0x18(%rsi)
14229acb9:     	movl	$0x0, 0x1c(%rsi)
14229acc0:     	movl	$0x0, 0x24(%rsi)
14229acc7:     	movq	$0x16, 0x28(%rsi)
14229accf:     	movq	%rbx, 0x30(%rsi)
14229acd3:     	movq	$0x16, 0x38(%rsi)
14229acdb:     	movb	$0x3, 0x40(%rsi)
14229acdf:     	movl	$0x4, 0x44(%rsi)
14229ace6:     	movl	$0x0, 0x4c(%rsi)
14229aced:     	movq	$0x19, 0x50(%rsi)
14229acf5:     	movq	%r14, 0x58(%rsi)
14229acf9:     	movq	$0x19, 0x60(%rsi)
14229ad01:     	movb	$0x3, 0x68(%rsi)
14229ad05:     	movl	$0x3, 0x6c(%rsi)
14229ad0c:     	movl	$0x0, 0x74(%rsi)
14229ad13:     	movq	$0x17, 0x78(%rsi)
14229ad1b:     	movq	%r12, 0x80(%rsi)
14229ad22:     	movq	$0x17, 0x88(%rsi)
14229ad2d:     	movb	$0x3, 0x90(%rsi)
14229ad34:     	movl	$0x1, 0x94(%rsi)
14229ad3e:     	movl	$0x0, 0x9c(%rsi)
14229ad48:     	movq	$0x1b, 0xa0(%rsi)
14229ad53:     	movq	%r13, 0xa8(%rsi)
14229ad5a:     	movq	$0x1b, 0xb0(%rsi)
14229ad65:     	movb	$0x3, 0xb8(%rsi)
14229ad6c:     	movl	$0x2, 0xbc(%rsi)
14229ad76:     	movl	$0x0, 0xc4(%rsi)
14229ad80:     	movq	$0x5, 0xc8(%rsi)
14229ad8b:     	movq	0x1e8(%rbp), %rcx
14229ad92:     	movq	%rcx, 0xd0(%rsi)
14229ad99:     	movq	$0x5, 0xd8(%rsi)
14229ada4:     	movb	$0x5, 0xe0(%rsi)
14229adab:     	movl	$0x0, 0xec(%rsi)
14229adb5:     	movq	$0x12, 0xf0(%rsi)
14229adc0:     	movq	0x1d8(%rbp), %rcx
14229adc7:     	movq	%rcx, 0xf8(%rsi)
14229adce:     	movq	$0x12, 0x100(%rsi)
14229add9:     	movw	$0x9, 0x108(%rsi)
14229ade2:     	movl	$0x0, 0x114(%rsi)
14229adec:     	movq	$0x12, 0x118(%rsi)
14229adf7:     	movq	%r15, 0x120(%rsi)
14229adfe:     	movq	$0x12, 0x128(%rsi)
14229ae09:     	movw	$0x109, 0x130(%rsi)     # imm = 0x109
14229ae12:     	movl	$0x0, 0x13c(%rsi)
14229ae1c:     	movq	$0x12, 0x140(%rsi)
14229ae27:     	movq	%rax, 0x148(%rsi)
14229ae2e:     	movq	$0x12, 0x150(%rsi)
14229ae39:     	movw	$0x209, 0x158(%rsi)     # imm = 0x209
14229ae42:     	movl	$0x0, 0x164(%rsi)
14229ae4c:     	movq	$0x9, 0x110(%rbp)
14229ae57:     	movq	%rsi, 0x118(%rbp)
14229ae5e:     	movq	$0x9, 0x120(%rbp)
14229ae69:     	jmp	0x14229c615 <.text+0x229b615>
14229ae6e:     	movq	%rdx, 0x1e0(%rbp)
14229ae75:     	movl	0x4(%r15), %edi
14229ae79:     	movq	0x358(%rbp), %rcx
14229ae80:     	callq	0x140b26a70 <.text+0xb25a70>
14229ae85:     	movq	%rax, 0x180(%rbp)
14229ae8c:     	movq	%rdx, 0x188(%rbp)
14229ae93:     	leaq	0x180(%rbp), %rcx
14229ae9a:     	movl	%edi, %edx
14229ae9c:     	callq	0x140b26970 <.text+0xb25970>
14229aea1:     	movl	%eax, %ebx
14229aea3:     	movq	0x200(%rbp), %rcx
14229aeaa:     	movl	%edi, %edx
14229aeac:     	callq	0x140a1c930 <.text+0xa1b930>
14229aeb1:     	testq	%rax, %rax
14229aeb4:     	je	0x14229dc4c <.text+0x229cc4c>
14229aeba:     	addq	$0x30, %rax
14229aebe:     	movq	%rax, %rcx
14229aec1:     	leaq	0xda1b30(%rip), %rdx    # 0x14303c9f8
14229aec8:     	callq	0x1408392d0 <.text+0x8382d0>
14229aecd:     	movzbl	0x4(%rax), %esi
14229aed1:     	movzbl	0x1399d31(%rip), %eax   # 0x143634c09
14229aed8:     	cmpb	$0x14, %bl
14229aedb:     	jne	0x14229dc5f <.text+0x229cc5f>
14229aee1:     	movl	$0x28, %ecx
14229aee6:     	movl	$0x8, %edx
14229aeeb:     	callq	0x140613c10 <.text+0x612c10>
14229aef0:     	testq	%rax, %rax
14229aef3:     	je	0x1422a1f7a <.text+0x22a0f7a>
14229aef9:     	movq	%rax, %rdi
14229aefc:     	movzbl	0x1399d06(%rip), %eax   # 0x143634c09
14229af03:     	testb	%sil, %sil
14229af06:     	je	0x14229e329 <.text+0x229d329>
14229af0c:     	movl	$0x1f, %ecx
14229af11:     	movl	$0x1, %edx
14229af16:     	callq	0x140613c10 <.text+0x612c10>
14229af1b:     	testq	%rax, %rax
14229af1e:     	je	0x1422a2a72 <.text+0x22a1a72>
14229af24:     	movups	0xe9be94(%rip), %xmm0   # 0x143136dbf
14229af2b:     	movups	%xmm0, 0xf(%rax)
14229af2f:     	movups	0xe9be7a(%rip), %xmm0   # 0x143136db0
14229af36:     	movups	%xmm0, (%rax)
14229af39:     	movl	$0x1f, %ecx
14229af3e:     	jmp	0x14229e365 <.text+0x229d365>
14229af43:     	movq	%rdx, 0x1e0(%rbp)
14229af4a:     	movq	0x8(%r15), %rax
14229af4e:     	movq	%rax, 0x80(%rbp)
14229af55:     	leaq	0x80(%rbp), %rbx
14229af5c:     	movq	%rbx, %rcx
14229af5f:     	callq	0x1400885d0 <.text+0x875d0>
14229af64:     	movq	0x10(%rsi), %rdx
14229af68:     	cmpq	%rdx, %rax
14229af6b:     	jae	0x1422a18b4 <.text+0x22a08b4>
14229af71:     	movq	0x8(%rsi), %rcx
14229af75:     	movl	(%rcx,%rax,4), %r13d
14229af79:     	movl	%r13d, 0x70(%rbp)
14229af7d:     	movq	%rbx, %rcx
14229af80:     	callq	0x1400885d0 <.text+0x875d0>
14229af85:     	movq	0x90(%rbp), %rcx
14229af8c:     	movq	0x10(%rcx), %rdx
14229af90:     	cmpq	%rdx, %rax
14229af93:     	jae	0x1422a139a <.text+0x22a039a>
14229af99:     	movq	0x8(%rcx), %rcx
14229af9d:     	movl	(%rcx,%rax,4), %ebx
14229afa0:     	movl	%ebx, 0x6c(%rbp)
14229afa3:     	movq	0x80(%rbp), %rdx
14229afaa:     	movq	0x1f8(%rbp), %rcx
14229afb1:     	callq	0x140a922b0 <.text+0xa912b0>
14229afb6:     	movq	%rax, %rcx
14229afb9:     	shrq	$0x20, %rcx
14229afbd:     	je	0x14229c621 <.text+0x229b621>
14229afc3:     	movl	%eax, %eax
14229afc5:     	movq	0x128(%rbp), %rdx
14229afcc:     	cmpq	0x10(%rdx), %rax
14229afd0:     	jae	0x14229c621 <.text+0x229b621>
14229afd6:     	movq	0x128(%rbp), %rdx
14229afdd:     	movq	0x8(%rdx), %rdx
14229afe1:     	leaq	(%rax,%rax,4), %rax
14229afe5:     	cmpl	%ecx, (%rdx,%rax,4)
14229afe8:     	jne	0x14229c621 <.text+0x229b621>
14229afee:     	leaq	(%rdx,%rax,4), %rax
14229aff2:     	movl	0x4(%rax), %ecx
14229aff5:     	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
14229affa:     	cmpq	%rdx, %rcx
14229affd:     	je	0x14229c621 <.text+0x229b621>
14229b003:     	movq	0x108(%rbp), %rdx
14229b00a:     	cmpq	%rcx, 0x38(%rdx)
14229b00e:     	jbe	0x14229c621 <.text+0x229b621>
14229b014:     	movq	0x108(%rbp), %rdx
14229b01b:     	movq	0x28(%rdx), %rdx
14229b01f:     	movl	%ecx, %r8d
14229b022:     	shrl	$0x6, %r8d
14229b026:     	movq	(%rdx,%r8,8), %rdx
14229b02a:     	btq	%rcx, %rdx
14229b02e:     	jae	0x14229c621 <.text+0x229b621>
14229b034:     	movl	0xc(%rax), %ecx
14229b037:     	movl	0x10(%rax), %eax
14229b03a:     	movq	0x128(%rbp), %rdx
14229b041:     	movq	0x1a8(%rdx), %rdx
14229b048:     	leaq	(%rcx,%rcx,8), %rcx
14229b04c:     	movq	0x108(%rbp), %r8
14229b053:     	movq	0x110(%r8), %r8
14229b05a:     	movq	0x18(%rdx,%rcx,8), %r9
14229b05f:     	movq	0x38(%rdx,%rcx,8), %rcx
14229b064:     	movq	(%rcx,%r8,8), %rcx
14229b068:     	notq	%rcx
14229b06b:     	leaq	(%rcx,%rcx,2), %rcx
14229b06f:     	shlq	$0x4, %rcx
14229b073:     	movq	0x10(%r9,%rcx), %rdx
14229b078:     	imulq	$0x58, %rax, %rax
14229b07c:     	movzbl	0x40(%rdx,%rax), %ecx
14229b081:     	testl	%ecx, %ecx
14229b083:     	je	0x1422a0b4d <.text+0x229fb4d>
14229b089:     	cmpl	$0x2, %ecx
14229b08c:     	jne	0x14229c621 <.text+0x229b621>
14229b092:     	addq	%rax, %rdx
14229b095:     	leaq	0x44(%rdx), %rcx
14229b099:     	movzbl	0x41(%rdx), %eax
14229b09d:     	movl	0x48(%rdx), %edi
14229b0a0:     	movl	(%rcx), %r12d
14229b0a3:     	testb	%al, %al
14229b0a5:     	je	0x1422a0b99 <.text+0x229fb99>
14229b0ab:     	movzbl	0x1399b57(%rip), %eax   # 0x143634c09
14229b0b2:     	movl	$0x190, %ecx            # imm = 0x190
14229b0b7:     	movl	$0x8, %edx
14229b0bc:     	callq	0x140613c10 <.text+0x612c10>
14229b0c1:     	testq	%rax, %rax
14229b0c4:     	je	0x1422a0f87 <.text+0x229ff87>
14229b0ca:     	movq	%rax, %r14
14229b0cd:     	movl	%r13d, 0x1e8(%rbp)
14229b0d4:     	movzbl	0x1399b2e(%rip), %eax   # 0x143634c09
14229b0db:     	movl	$0x18, %ecx
14229b0e0:     	movl	$0x1, %edx
14229b0e5:     	callq	0x140613c10 <.text+0x612c10>
14229b0ea:     	testq	%rax, %rax
14229b0ed:     	je	0x1422a2d82 <.text+0x22a1d82>
14229b0f3:     	movq	%rax, %rsi
14229b0f6:     	movups	0xda12cb(%rip), %xmm0   # 0x14303c3c8
14229b0fd:     	movups	%xmm0, (%rax)
14229b100:     	movabsq	$0x65746968772f7377, %rax # imm = 0x65746968772F7377
14229b10a:     	movq	%rax, 0x10(%rsi)
14229b10e:     	movzbl	0x1399af4(%rip), %eax   # 0x143634c09
14229b115:     	movl	$0x17, %ecx
14229b11a:     	movl	$0x1, %edx
14229b11f:     	callq	0x140613c10 <.text+0x612c10>
14229b124:     	testq	%rax, %rax
14229b127:     	je	0x1422a2da4 <.text+0x22a1da4>
14229b12d:     	movq	%rax, %r13
14229b130:     	movups	0xda12a9(%rip), %xmm0   # 0x14303c3e0
14229b137:     	movups	%xmm0, (%rax)
14229b13a:     	movabsq	$0x646f6f772f73776f, %rax # imm = 0x646F6F772F73776F
14229b144:     	movq	%rax, 0xf(%r13)
14229b148:     	movzbl	0x1399aba(%rip), %eax   # 0x143634c09
14229b14f:     	movl	$0x1c, %ecx
14229b154:     	movl	$0x1, %edx
14229b159:     	callq	0x140613c10 <.text+0x612c10>
14229b15e:     	testq	%rax, %rax
14229b161:     	je	0x1422a2dcd <.text+0x22a1dcd>
14229b167:     	movups	0xda1295(%rip), %xmm0   # 0x14303c403
14229b16e:     	movups	%xmm0, 0xc(%rax)
14229b172:     	movups	0xda127e(%rip), %xmm0   # 0x14303c3f7
14229b179:     	movq	%rax, 0x1f0(%rbp)
14229b180:     	movups	%xmm0, (%rax)
14229b183:     	movzbl	0x1399a7f(%rip), %eax   # 0x143634c09
14229b18a:     	movl	$0x14, %ecx
14229b18f:     	movl	$0x1, %edx
14229b194:     	callq	0x140613c10 <.text+0x612c10>
14229b199:     	testq	%rax, %rax
14229b19c:     	je	0x1422a2d52 <.text+0x22a1d52>
14229b1a2:     	movups	0xda11c6(%rip), %xmm0   # 0x14303c36f
14229b1a9:     	movups	%xmm0, (%rax)
14229b1ac:     	movq	%rax, 0x1d8(%rbp)
14229b1b3:     	movl	$0x6b726164, 0x10(%rax) # imm = 0x6B726164
14229b1ba:     	movzbl	0x1399a48(%rip), %eax   # 0x143634c09
14229b1c1:     	movl	$0x13, %ecx
14229b1c6:     	movl	$0x1, %edx
14229b1cb:     	callq	0x140613c10 <.text+0x612c10>
14229b1d0:     	testq	%rax, %rax
14229b1d3:     	je	0x1422a2d22 <.text+0x22a1d22>
14229b1d9:     	movups	0xda10d6(%rip), %xmm0   # 0x14303c2b6
14229b1e0:     	movups	%xmm0, (%rax)
14229b1e3:     	movq	%rax, 0x1c0(%rbp)
14229b1ea:     	movl	$0x6465725f, 0xf(%rax)  # imm = 0x6465725F
14229b1f1:     	movzbl	0x1399a11(%rip), %eax   # 0x143634c09
14229b1f8:     	movl	$0xf, %ecx
14229b1fd:     	movl	$0x1, %edx
14229b202:     	callq	0x140613c10 <.text+0x612c10>
14229b207:     	testq	%rax, %rax
14229b20a:     	je	0x1422a2cf2 <.text+0x22a1cf2>
14229b210:     	movq	%rax, %rcx
14229b213:     	movabsq	$0x6b6e69702f6e6f63, %rax # imm = 0x6B6E69702F6E6F63
14229b21d:     	movq	%rax, 0x7(%rcx)
14229b221:     	movabsq	$0x63695f726f6c6f63, %rax # imm = 0x63695F726F6C6F63
14229b22b:     	movq	%rcx, 0x150(%rbp)
14229b232:     	movq	%rax, (%rcx)
14229b235:     	movzbl	0x13999cd(%rip), %eax   # 0x143634c09
14229b23c:     	movl	$0x19, %ecx
14229b241:     	movl	$0x1, %edx
14229b246:     	callq	0x140613c10 <.text+0x612c10>
14229b24b:     	testq	%rax, %rax
14229b24e:     	je	0x1422a2cc2 <.text+0x22a1cc2>
14229b254:     	movups	0xda11c1(%rip), %xmm0   # 0x14303c41c
14229b25b:     	movups	%xmm0, 0x9(%rax)
14229b25f:     	movups	0xda11ad(%rip), %xmm0   # 0x14303c413
14229b266:     	movq	%rax, 0x148(%rbp)
14229b26d:     	movups	%xmm0, (%rax)
14229b270:     	movzbl	0x1399992(%rip), %eax   # 0x143634c09
14229b277:     	movl	$0x14, %ecx
14229b27c:     	movl	$0x1, %edx
14229b281:     	callq	0x140613c10 <.text+0x612c10>
14229b286:     	testq	%rax, %rax
14229b289:     	je	0x1422a2e09 <.text+0x22a1e09>
14229b28f:     	movl	%r12d, 0xd0(%rbp)
14229b296:     	movups	0xda1042(%rip), %xmm0   # 0x14303c2df
14229b29d:     	movups	%xmm0, (%rax)
14229b2a0:     	movq	%rax, 0x100(%rbp)
14229b2a7:     	movl	$0x65756c62, 0x10(%rax) # imm = 0x65756C62
14229b2ae:     	movzbl	0x1399954(%rip), %eax   # 0x143634c09
14229b2b5:     	movl	$0x1b, %ecx
14229b2ba:     	movl	$0x1, %edx
14229b2bf:     	callq	0x140613c10 <.text+0x612c10>
14229b2c4:     	testq	%rax, %rax
14229b2c7:     	je	0x1422a2e36 <.text+0x22a1e36>
14229b2cd:     	movups	0xda105d(%rip), %xmm0   # 0x14303c331
14229b2d4:     	movups	%xmm0, 0xb(%rax)
14229b2d8:     	movups	0xda1047(%rip), %xmm0   # 0x14303c326
14229b2df:     	movups	%xmm0, (%rax)
14229b2e2:     	movq	%rax, %r12
14229b2e5:     	movzbl	0x139991d(%rip), %eax   # 0x143634c09
14229b2ec:     	movl	$0x15, %ecx
14229b2f1:     	movl	$0x1, %edx
14229b2f6:     	callq	0x140613c10 <.text+0x612c10>
14229b2fb:     	testq	%rax, %rax
14229b2fe:     	je	0x1422a2e63 <.text+0x22a1e63>
14229b304:     	movq	$0x18, (%r14)
14229b30b:     	movq	%rsi, 0x8(%r14)
14229b30f:     	movq	$0x18, 0x10(%r14)
14229b317:     	movw	$0x102, 0x18(%r14)      # imm = 0x102
14229b31e:     	movl	$0x4, 0x1c(%r14)
14229b326:     	movl	%edi, 0x20(%r14)
14229b32a:     	movl	$0x0, 0x24(%r14)
14229b332:     	movq	$0x17, 0x28(%r14)
14229b33a:     	movq	%r13, 0x30(%r14)
14229b33e:     	movq	$0x17, 0x38(%r14)
14229b346:     	movw	$0x102, 0x40(%r14)      # imm = 0x102
14229b34d:     	movl	$0x0, 0x44(%r14)
14229b355:     	movl	%edi, 0x48(%r14)
14229b359:     	movl	$0x0, 0x4c(%r14)
14229b361:     	movq	$0x1c, 0x50(%r14)
14229b369:     	movq	0x1f0(%rbp), %rcx
14229b370:     	movq	%rcx, 0x58(%r14)
14229b374:     	movq	$0x1c, 0x60(%r14)
14229b37c:     	movw	$0x102, 0x68(%r14)      # imm = 0x102
14229b383:     	movl	$0x1, 0x6c(%r14)
14229b38b:     	movl	%edi, 0x70(%r14)
14229b38f:     	movl	$0x0, 0x74(%r14)
14229b397:     	movq	$0x14, 0x78(%r14)
14229b39f:     	movq	0x1d8(%rbp), %rcx
14229b3a6:     	movq	%rcx, 0x80(%r14)
14229b3ad:     	movq	$0x14, 0x88(%r14)
14229b3b8:     	movw	$0x102, 0x90(%r14)      # imm = 0x102
14229b3c2:     	movl	$0x7, 0x94(%r14)
14229b3cd:     	movl	%edi, 0x98(%r14)
14229b3d4:     	movl	$0x0, 0x9c(%r14)
14229b3df:     	movq	$0x13, 0xa0(%r14)
14229b3ea:     	movq	0x1c0(%rbp), %rcx
14229b3f1:     	movq	%rcx, 0xa8(%r14)
14229b3f8:     	movq	$0x13, 0xb0(%r14)
14229b403:     	movw	$0x102, 0xb8(%r14)      # imm = 0x102
14229b40d:     	movl	$0x3, 0xbc(%r14)
14229b418:     	movl	%edi, 0xc0(%r14)
14229b41f:     	movl	$0x0, 0xc4(%r14)
14229b42a:     	movq	$0xf, 0xc8(%r14)
14229b435:     	movq	0x150(%rbp), %rcx
14229b43c:     	movq	%rcx, 0xd0(%r14)
14229b443:     	movq	$0xf, 0xd8(%r14)
14229b44e:     	movw	$0x102, 0xe0(%r14)      # imm = 0x102
14229b458:     	movl	$0x9, 0xe4(%r14)
14229b463:     	movl	%edi, 0xe8(%r14)
14229b46a:     	movl	$0x0, 0xec(%r14)
14229b475:     	movq	$0x19, 0xf0(%r14)
14229b480:     	movq	0x148(%rbp), %rcx
14229b487:     	movq	%rcx, 0xf8(%r14)
14229b48e:     	movq	$0x19, 0x100(%r14)
14229b499:     	movw	$0x102, 0x108(%r14)     # imm = 0x102
14229b4a3:     	movl	$0x8, 0x10c(%r14)
14229b4ae:     	movl	%edi, 0x110(%r14)
14229b4b5:     	movl	$0x0, 0x114(%r14)
14229b4c0:     	movq	$0x14, 0x118(%r14)
14229b4cb:     	movq	0x100(%rbp), %rcx
14229b4d2:     	movq	%rcx, 0x120(%r14)
14229b4d9:     	movq	$0x14, 0x128(%r14)
14229b4e4:     	movw	$0x102, 0x130(%r14)     # imm = 0x102
14229b4ee:     	movl	$0x2, 0x134(%r14)
14229b4f9:     	movl	%edi, 0x138(%r14)
14229b500:     	movl	$0x0, 0x13c(%r14)
14229b50b:     	movq	$0x1b, 0x140(%r14)
14229b516:     	movq	%r12, 0x148(%r14)
14229b51d:     	movups	0xda0e1d(%rip), %xmm0   # 0x14303c341
14229b524:     	movups	%xmm0, (%rax)
14229b527:     	movabsq	$0x6e656572675f666f, %rcx # imm = 0x6E656572675F666F
14229b531:     	movq	%rcx, 0xd(%rax)
14229b535:     	movq	$0x1b, 0x150(%r14)
14229b540:     	movw	$0x102, 0x158(%r14)     # imm = 0x102
14229b54a:     	movl	$0x6, 0x15c(%r14)
14229b555:     	movl	%edi, 0x160(%r14)
14229b55c:     	movl	$0x0, 0x164(%r14)
14229b567:     	movq	$0x15, 0x168(%r14)
14229b572:     	movq	%rax, 0x170(%r14)
14229b579:     	movq	$0x15, 0x178(%r14)
14229b584:     	movw	$0x102, 0x180(%r14)     # imm = 0x102
14229b58e:     	movl	$0x5, 0x184(%r14)
14229b599:     	movl	%edi, 0x188(%r14)
14229b5a0:     	movq	%r14, %rax
14229b5a3:     	addq	$0x18c, %rax            # imm = 0x18C
14229b5a9:     	movl	$0xa, %ecx
14229b5ae:     	movl	0xd0(%rbp), %esi
14229b5b4:     	jmp	0x14229c890 <.text+0x229b890>
14229b5b9:     	movq	%rdx, 0x1e0(%rbp)
14229b5c0:     	movl	0x4(%r15), %r12d
14229b5c4:     	movq	0x200(%rbp), %rcx
14229b5cb:     	movl	%r12d, %edx
14229b5ce:     	callq	0x140a1c930 <.text+0xa1b930>
14229b5d3:     	testq	%rax, %rax
14229b5d6:     	je	0x14229de23 <.text+0x229ce23>
14229b5dc:     	movzbl	0x22(%rax), %eax
14229b5e0:     	addb	$-0x2, %al
14229b5e2:     	cmpb	$0x3, %al
14229b5e4:     	setae	%cl
14229b5e7:     	movzbl	%al, %eax
14229b5ea:     	incl	%eax
14229b5ec:     	testl	$0x1fd, %eax            # imm = 0x1FD
14229b5f1:     	sete	%sil
14229b5f5:     	orb	%cl, %sil
14229b5f8:     	jmp	0x14229de25 <.text+0x229ce25>
14229b5fd:     	movq	%rdx, 0x1e0(%rbp)
14229b604:     	movl	0x4(%r15), %edi
14229b608:     	movzbl	0x1(%r15), %ebx
14229b60d:     	leaq	0x180(%rbp), %rcx
14229b614:     	callq	0x1422acce0 <.text+0x22abce0>
14229b619:     	testb	%bl, %bl
14229b61b:     	je	0x14229b70b <.text+0x229a70b>
14229b621:     	movq	0x358(%rbp), %rcx
14229b628:     	callq	0x140b26a70 <.text+0xb25a70>
14229b62d:     	movq	%rax, 0xd8(%rbp)
14229b634:     	movq	%rdx, 0xe0(%rbp)
14229b63b:     	leaq	0xd8(%rbp), %rcx
14229b642:     	movl	%edi, %edx
14229b644:     	callq	0x140b26970 <.text+0xb25970>
14229b649:     	addb	$-0x9, %al
14229b64b:     	cmpb	$0x2, %al
14229b64d:     	jb	0x14229b70b <.text+0x229a70b>
14229b653:     	movzbl	0x13995af(%rip), %eax   # 0x143634c09
14229b65a:     	movl	$0x1b, %ecx
14229b65f:     	movl	$0x1, %edx
14229b664:     	callq	0x140613c10 <.text+0x612c10>
14229b669:     	testq	%rax, %rax
14229b66c:     	je	0x1422a2a35 <.text+0x22a1a35>
14229b672:     	movups	0xda0e29(%rip), %xmm0   # 0x14303c4a2
14229b679:     	movups	%xmm0, 0xb(%rax)
14229b67d:     	movups	0xda0e13(%rip), %xmm0   # 0x14303c497
14229b684:     	movq	%rax, 0x1f0(%rbp)
14229b68b:     	movups	%xmm0, (%rax)
14229b68e:     	movq	0x190(%rbp), %rsi
14229b695:     	cmpq	0x180(%rbp), %rsi
14229b69c:     	jne	0x14229b6b1 <.text+0x229a6b1>
14229b69e:     	leaq	0x180(%rbp), %rcx
14229b6a5:     	leaq	0xda0e0c(%rip), %rdx    # 0x14303c4b8
14229b6ac:     	callq	0x140562ab0 <.text+0x561ab0>
14229b6b1:     	movq	0x188(%rbp), %rdi
14229b6b8:     	testq	%rsi, %rsi
14229b6bb:     	je	0x14229b6d5 <.text+0x229a6d5>
14229b6bd:     	leaq	0x28(%rdi), %rcx
14229b6c1:     	leaq	(,%rsi,8), %rax
14229b6c9:     	leaq	(%rax,%rax,4), %r8
14229b6cd:     	movq	%rdi, %rdx
14229b6d0:     	callq	0x142923c10 <.text+0x2922c10>
14229b6d5:     	movq	$0x1b, (%rdi)
14229b6dc:     	movq	0x1f0(%rbp), %rax
14229b6e3:     	movq	%rax, 0x8(%rdi)
14229b6e7:     	movq	$0x1b, 0x10(%rdi)
14229b6ef:     	movb	$0x3, 0x18(%rdi)
14229b6f3:     	movl	$0x3e7, 0x1c(%rdi)      # imm = 0x3E7
14229b6fa:     	movl	$0x0, 0x24(%rdi)
14229b701:     	incq	%rsi
14229b704:     	movq	%rsi, 0x190(%rbp)
14229b70b:     	movq	0x190(%rbp), %rax
14229b712:     	movq	%rax, 0x120(%rbp)
14229b719:     	movups	0x180(%rbp), %xmm0
14229b720:     	jmp	0x14229e9d6 <.text+0x229d9d6>
14229b725:     	movq	%rdx, 0x1e0(%rbp)
14229b72c:     	movq	0x8(%r15), %rdx
14229b730:     	movq	%rdx, 0x40(%rbp)
14229b734:     	movq	0x139095d(%rip), %rax   # 0x14362c098
14229b73b:     	cmpb	$0x0, (%rax)
14229b73e:     	je	0x14229cc37 <.text+0x229bc37>
14229b744:     	movzbl	0x13994be(%rip), %eax   # 0x143634c09
14229b74b:     	movl	$0x280, %ecx            # imm = 0x280
14229b750:     	movl	$0x8, %edx
14229b755:     	callq	0x140613c10 <.text+0x612c10>
14229b75a:     	testq	%rax, %rax
14229b75d:     	je	0x1422a2409 <.text+0x22a1409>
14229b763:     	movq	%rax, %rbx
14229b766:     	movzbl	0x139949c(%rip), %eax   # 0x143634c09
14229b76d:     	movl	$0x16, %ecx
14229b772:     	movl	$0x1, %edx
14229b777:     	callq	0x140613c10 <.text+0x612c10>
14229b77c:     	testq	%rax, %rax
14229b77f:     	je	0x1422a1f58 <.text+0x22a0f58>
14229b785:     	movq	%rax, %r14
14229b788:     	movups	0xda0a0f(%rip), %xmm0   # 0x14303c19e
14229b78f:     	movups	%xmm0, (%rax)
14229b792:     	movabsq	$0x796572675f736b63, %rax # imm = 0x796572675F736B63
14229b79c:     	movq	%rax, 0xe(%r14)
14229b7a0:     	movzbl	0x1399462(%rip), %eax   # 0x143634c09
14229b7a7:     	movl	$0xe, %ecx
14229b7ac:     	movl	$0x1, %edx
14229b7b1:     	callq	0x140613c10 <.text+0x612c10>
14229b7b6:     	testq	%rax, %rax
14229b7b9:     	je	0x1422a1d0f <.text+0x22a0d0f>
14229b7bf:     	movq	%rax, %r12
14229b7c2:     	movq	%rdi, 0x6(%rax)
14229b7c6:     	movabsq	$0x5f72657473616c70, %rsi # imm = 0x5F72657473616C70
14229b7d0:     	movq	%rsi, (%rax)
14229b7d3:     	movzbl	0x139942f(%rip), %eax   # 0x143634c09
14229b7da:     	movl	$0xe, %ecx
14229b7df:     	movl	$0x1, %edx
14229b7e4:     	callq	0x140613c10 <.text+0x612c10>
14229b7e9:     	testq	%rax, %rax
14229b7ec:     	je	0x1422a265a <.text+0x22a165a>
14229b7f2:     	movq	%rax, %r13
14229b7f5:     	movq	%rdi, 0x6(%rax)
14229b7f9:     	movq	%rsi, (%rax)
14229b7fc:     	movzbl	0x1399406(%rip), %eax   # 0x143634c09
14229b803:     	movl	$0x11, %ecx
14229b808:     	movl	$0x1, %edx
14229b80d:     	callq	0x140613c10 <.text+0x612c10>
14229b812:     	testq	%rax, %rax
14229b815:     	je	0x1422a2577 <.text+0x22a1577>
14229b81b:     	movq	%rax, %rsi
14229b81e:     	movups	0xda0a6b(%rip), %xmm6   # 0x14303c290
14229b825:     	movups	%xmm6, (%rax)
14229b828:     	movb	$0x64, 0x10(%rax)
14229b82c:     	movzbl	0x13993d6(%rip), %eax   # 0x143634c09
14229b833:     	movl	$0x11, %ecx
14229b838:     	movl	$0x1, %edx
14229b83d:     	callq	0x140613c10 <.text+0x612c10>
14229b842:     	testq	%rax, %rax
14229b845:     	je	0x1422a245d <.text+0x22a145d>
14229b84b:     	movq	%rax, %rdi
14229b84e:     	movups	%xmm6, (%rax)
14229b851:     	movb	$0x64, 0x10(%rax)
14229b855:     	movzbl	0x13993ad(%rip), %eax   # 0x143634c09
14229b85c:     	movl	$0x11, %ecx
14229b861:     	movl	$0x1, %edx
14229b866:     	callq	0x140613c10 <.text+0x612c10>
14229b86b:     	testq	%rax, %rax
14229b86e:     	je	0x1422a22d1 <.text+0x22a12d1>
14229b874:     	movups	%xmm6, (%rax)
14229b877:     	movq	%rax, 0x1f0(%rbp)
14229b87e:     	movb	$0x64, 0x10(%rax)
14229b882:     	movzbl	0x1399380(%rip), %eax   # 0x143634c09
14229b889:     	movl	$0x11, %ecx
14229b88e:     	movl	$0x1, %edx
14229b893:     	callq	0x140613c10 <.text+0x612c10>
14229b898:     	testq	%rax, %rax
14229b89b:     	je	0x1422a1e83 <.text+0x22a0e83>
14229b8a1:     	movups	%xmm6, (%rax)
14229b8a4:     	movq	%rax, 0x1e8(%rbp)
14229b8ab:     	movb	$0x64, 0x10(%rax)
14229b8af:     	movzbl	0x1399353(%rip), %eax   # 0x143634c09
14229b8b6:     	movl	$0x11, %ecx
14229b8bb:     	movl	$0x1, %edx
14229b8c0:     	callq	0x140613c10 <.text+0x612c10>
14229b8c5:     	testq	%rax, %rax
14229b8c8:     	je	0x1422a1d7d <.text+0x22a0d7d>
14229b8ce:     	movups	%xmm6, (%rax)
14229b8d1:     	movq	%rax, 0x1d8(%rbp)
14229b8d8:     	movb	$0x64, 0x10(%rax)
14229b8dc:     	movzbl	0x1399326(%rip), %eax   # 0x143634c09
14229b8e3:     	movl	$0x11, %ecx
14229b8e8:     	movl	$0x1, %edx
14229b8ed:     	callq	0x140613c10 <.text+0x612c10>
14229b8f2:     	testq	%rax, %rax
14229b8f5:     	je	0x1422a2088 <.text+0x22a1088>
14229b8fb:     	movups	%xmm6, (%rax)
14229b8fe:     	movq	%rax, 0x1c0(%rbp)
14229b905:     	movb	$0x64, 0x10(%rax)
14229b909:     	movzbl	0x13992f9(%rip), %eax   # 0x143634c09
14229b910:     	movl	$0x11, %ecx
14229b915:     	movl	$0x1, %edx
14229b91a:     	callq	0x140613c10 <.text+0x612c10>
14229b91f:     	testq	%rax, %rax
14229b922:     	je	0x1422a268a <.text+0x22a168a>
14229b928:     	movups	%xmm6, (%rax)
14229b92b:     	movq	%rax, 0x150(%rbp)
14229b932:     	movb	$0x64, 0x10(%rax)
14229b936:     	movzbl	0x13992cc(%rip), %eax   # 0x143634c09
14229b93d:     	movl	$0x11, %ecx
14229b942:     	movl	$0x1, %edx
14229b947:     	callq	0x140613c10 <.text+0x612c10>
14229b94c:     	testq	%rax, %rax
14229b94f:     	je	0x1422a25ae <.text+0x22a15ae>
14229b955:     	movups	%xmm6, (%rax)
14229b958:     	movq	%rax, 0x148(%rbp)
14229b95f:     	movb	$0x64, 0x10(%rax)
14229b963:     	movzbl	0x139929f(%rip), %eax   # 0x143634c09
14229b96a:     	movl	$0x11, %ecx
14229b96f:     	movl	$0x1, %edx
14229b974:     	callq	0x140613c10 <.text+0x612c10>
14229b979:     	testq	%rax, %rax
14229b97c:     	je	0x1422a249b <.text+0x22a149b>
14229b982:     	movups	%xmm6, (%rax)
14229b985:     	movq	%rax, 0x100(%rbp)
14229b98c:     	movb	$0x64, 0x10(%rax)
14229b990:     	movzbl	0x1399272(%rip), %eax   # 0x143634c09
14229b997:     	movl	$0x5, %ecx
14229b99c:     	movl	$0x1, %edx
14229b9a1:     	callq	0x140613c10 <.text+0x612c10>
14229b9a6:     	testq	%rax, %rax
14229b9a9:     	je	0x1422a2217 <.text+0x22a1217>
14229b9af:     	movzbl	0xe9b3a1(%rip), %ecx    # 0x143136d57
14229b9b6:     	movb	%cl, 0x4(%rax)
14229b9b9:     	movl	0xe9b394(%rip), %ecx    # 0x143136d53
14229b9bf:     	movq	%rax, 0xd0(%rbp)
14229b9c6:     	movl	%ecx, (%rax)
14229b9c8:     	movzbl	0x139923a(%rip), %eax   # 0x143634c09
14229b9cf:     	movl	$0x11, %ecx
14229b9d4:     	movl	$0x1, %edx
14229b9d9:     	callq	0x140613c10 <.text+0x612c10>
14229b9de:     	testq	%rax, %rax
14229b9e1:     	je	0x1422a2179 <.text+0x22a1179>
14229b9e7:     	movups	%xmm6, (%rax)
14229b9ea:     	movq	%rax, 0xc8(%rbp)
14229b9f1:     	movb	$0x64, 0x10(%rax)
14229b9f5:     	movzbl	0x139920d(%rip), %eax   # 0x143634c09
14229b9fc:     	movl	$0x10, %ecx
14229ba01:     	movl	$0x1, %edx
14229ba06:     	callq	0x140613c10 <.text+0x612c10>
14229ba0b:     	testq	%rax, %rax
14229ba0e:     	je	0x1422a20cd <.text+0x22a10cd>
14229ba14:     	movups	0xda0822(%rip), %xmm0   # 0x14303c23d
14229ba1b:     	movups	%xmm0, (%rax)
14229ba1e:     	movq	%rax, 0xa8(%rbp)
14229ba25:     	movzbl	0x13991dd(%rip), %eax   # 0x143634c09
14229ba2c:     	movl	$0x13, %ecx
14229ba31:     	movl	$0x1, %edx
14229ba36:     	callq	0x140613c10 <.text+0x612c10>
14229ba3b:     	testq	%rax, %rax
14229ba3e:     	je	0x1422a1fce <.text+0x22a0fce>
14229ba44:     	movq	$0x16, (%rbx)
14229ba4b:     	movq	%r14, 0x8(%rbx)
14229ba4f:     	movq	$0x16, 0x10(%rbx)
14229ba57:     	movb	$0x3, 0x18(%rbx)
14229ba5b:     	movl	$0x3, 0x1c(%rbx)
14229ba62:     	movl	$0x0, 0x24(%rbx)
14229ba69:     	movq	$0xe, 0x28(%rbx)
14229ba71:     	movq	%r12, 0x30(%rbx)
14229ba75:     	movq	$0xe, 0x38(%rbx)
14229ba7d:     	movb	$0x3, 0x40(%rbx)
14229ba81:     	movl	$0x5, 0x44(%rbx)
14229ba88:     	movl	$0x1, 0x4c(%rbx)
14229ba8f:     	movq	$0xe, 0x50(%rbx)
14229ba97:     	movq	%r13, 0x58(%rbx)
14229ba9b:     	movq	$0xe, 0x60(%rbx)
14229baa3:     	movb	$0x3, 0x68(%rbx)
14229baa7:     	movl	$0x8, 0x6c(%rbx)
14229baae:     	movl	$0x4, 0x74(%rbx)
14229bab5:     	movq	$0x11, 0x78(%rbx)
14229babd:     	movq	%rsi, 0x80(%rbx)
14229bac4:     	movq	$0x11, 0x88(%rbx)
14229bacf:     	movb	$0x3, 0x90(%rbx)
14229bad6:     	movl	$0x309, 0x94(%rbx)      # imm = 0x309
14229bae0:     	movl	$0x0, 0x9c(%rbx)
14229baea:     	movq	$0x11, 0xa0(%rbx)
14229baf5:     	movq	%rdi, 0xa8(%rbx)
14229bafc:     	movq	$0x11, 0xb0(%rbx)
14229bb07:     	movb	$0x3, 0xb8(%rbx)
14229bb0e:     	movl	$0x309, 0xbc(%rbx)      # imm = 0x309
14229bb18:     	movl	$0x0, 0xc4(%rbx)
14229bb22:     	movq	$0x11, 0xc8(%rbx)
14229bb2d:     	movq	0x1f0(%rbp), %rcx
14229bb34:     	movq	%rcx, 0xd0(%rbx)
14229bb3b:     	movq	$0x11, 0xd8(%rbx)
14229bb46:     	movb	$0x3, 0xe0(%rbx)
14229bb4d:     	movl	$0x309, 0xe4(%rbx)      # imm = 0x309
14229bb57:     	movl	$0x0, 0xec(%rbx)
14229bb61:     	movq	$0x11, 0xf0(%rbx)
14229bb6c:     	movq	0x1e8(%rbp), %rcx
14229bb73:     	movq	%rcx, 0xf8(%rbx)
14229bb7a:     	movq	$0x11, 0x100(%rbx)
14229bb85:     	movb	$0x3, 0x108(%rbx)
14229bb8c:     	movl	$0x309, 0x10c(%rbx)     # imm = 0x309
14229bb96:     	movl	$0x0, 0x114(%rbx)
14229bba0:     	movq	$0x11, 0x118(%rbx)
14229bbab:     	movq	0x1d8(%rbp), %rcx
14229bbb2:     	movq	%rcx, 0x120(%rbx)
14229bbb9:     	movq	$0x11, 0x128(%rbx)
14229bbc4:     	movb	$0x3, 0x130(%rbx)
14229bbcb:     	movl	$0x309, 0x134(%rbx)     # imm = 0x309
14229bbd5:     	movl	$0x0, 0x13c(%rbx)
14229bbdf:     	movq	$0x11, 0x140(%rbx)
14229bbea:     	movq	0x1c0(%rbp), %rcx
14229bbf1:     	movq	%rcx, 0x148(%rbx)
14229bbf8:     	movq	$0x11, 0x150(%rbx)
14229bc03:     	movb	$0x3, 0x158(%rbx)
14229bc0a:     	movl	$0x309, 0x15c(%rbx)     # imm = 0x309
14229bc14:     	movl	$0x0, 0x164(%rbx)
14229bc1e:     	movq	$0x11, 0x168(%rbx)
14229bc29:     	movq	0x150(%rbp), %rcx
14229bc30:     	movq	%rcx, 0x170(%rbx)
14229bc37:     	movq	$0x11, 0x178(%rbx)
14229bc42:     	movb	$0x3, 0x180(%rbx)
14229bc49:     	movl	$0x309, 0x184(%rbx)     # imm = 0x309
14229bc53:     	movl	$0x0, 0x18c(%rbx)
14229bc5d:     	movq	$0x11, 0x190(%rbx)
14229bc68:     	movq	0x148(%rbp), %rcx
14229bc6f:     	movq	%rcx, 0x198(%rbx)
14229bc76:     	movq	$0x11, 0x1a0(%rbx)
14229bc81:     	movb	$0x3, 0x1a8(%rbx)
14229bc88:     	movl	$0x309, 0x1ac(%rbx)     # imm = 0x309
14229bc92:     	movl	$0x0, 0x1b4(%rbx)
14229bc9c:     	movq	$0x11, 0x1b8(%rbx)
14229bca7:     	movq	0x100(%rbp), %rcx
14229bcae:     	movq	%rcx, 0x1c0(%rbx)
14229bcb5:     	movq	$0x11, 0x1c8(%rbx)
14229bcc0:     	movb	$0x3, 0x1d0(%rbx)
14229bcc7:     	movl	$0x309, 0x1d4(%rbx)     # imm = 0x309
14229bcd1:     	movl	$0x0, 0x1dc(%rbx)
14229bcdb:     	movq	$0x5, 0x1e0(%rbx)
14229bce6:     	movq	0xd0(%rbp), %rcx
14229bced:     	movq	%rcx, 0x1e8(%rbx)
14229bcf4:     	movq	$0x5, 0x1f0(%rbx)
14229bcff:     	movb	$0x5, 0x1f8(%rbx)
14229bd06:     	movl	$0x0, 0x204(%rbx)
14229bd10:     	movq	$0x11, 0x208(%rbx)
14229bd1b:     	movq	0xc8(%rbp), %rcx
14229bd22:     	movq	%rcx, 0x210(%rbx)
14229bd29:     	movq	$0x11, 0x218(%rbx)
14229bd34:     	movb	$0x3, 0x220(%rbx)
14229bd3b:     	movl	$0x309, 0x224(%rbx)     # imm = 0x309
14229bd45:     	movl	$0x0, 0x22c(%rbx)
14229bd4f:     	movq	$0x10, 0x230(%rbx)
14229bd5a:     	movq	0xa8(%rbp), %rcx
14229bd61:     	movq	%rcx, 0x238(%rbx)
14229bd68:     	movups	0xda04de(%rip), %xmm0   # 0x14303c24d
14229bd6f:     	movups	%xmm0, (%rax)
14229bd72:     	movl	$0x7472696b, 0xf(%rax)  # imm = 0x7472696B
14229bd79:     	movq	$0x10, 0x240(%rbx)
14229bd84:     	movw	$0x104, 0x248(%rbx)     # imm = 0x104
14229bd8d:     	movl	$0x0, 0x254(%rbx)
14229bd97:     	movq	$0x13, 0x258(%rbx)
14229bda2:     	movq	%rax, 0x260(%rbx)
14229bda9:     	movq	$0x13, 0x268(%rbx)
14229bdb4:     	movw	$0x4, 0x270(%rbx)
14229bdbd:     	movl	$0x0, 0x27c(%rbx)
14229bdc7:     	movq	$0x10, 0x110(%rbp)
14229bdd2:     	movq	%rbx, 0x118(%rbp)
14229bdd9:     	movq	$0x10, 0x120(%rbp)
14229bde4:     	jmp	0x14229e9e0 <.text+0x229d9e0>
14229bde9:     	movq	%rdx, 0x1e0(%rbp)
14229bdf0:     	movzbl	0x1398e12(%rip), %eax   # 0x143634c09
14229bdf7:     	movq	0x139029a(%rip), %rax   # 0x14362c098
14229bdfe:     	cmpb	$0x0, (%rax)
14229be01:     	je	0x14229d754 <.text+0x229c754>
14229be07:     	movl	$0x168, %ecx            # imm = 0x168
14229be0c:     	movl	$0x8, %edx
14229be11:     	callq	0x140613c10 <.text+0x612c10>
14229be16:     	testq	%rax, %rax
14229be19:     	je	0x1422a0fdc <.text+0x229ffdc>
14229be1f:     	movq	%rax, %rsi
14229be22:     	movzbl	0x1398de0(%rip), %eax   # 0x143634c09
14229be29:     	movl	$0x13, %ecx
14229be2e:     	movl	$0x1, %edx
14229be33:     	callq	0x140613c10 <.text+0x612c10>
14229be38:     	testq	%rax, %rax
14229be3b:     	je	0x1422a25f3 <.text+0x22a15f3>
14229be41:     	movq	%rax, %rdi
14229be44:     	movups	0xda046b(%rip), %xmm0   # 0x14303c2b6
14229be4b:     	movups	%xmm0, (%rax)
14229be4e:     	movl	$0x6465725f, 0xf(%rax)  # imm = 0x6465725F
14229be55:     	movzbl	0x1398dad(%rip), %eax   # 0x143634c09
14229be5c:     	movl	$0x15, %ecx
14229be61:     	movl	$0x1, %edx
14229be66:     	callq	0x140613c10 <.text+0x612c10>
14229be6b:     	testq	%rax, %rax
14229be6e:     	je	0x1422a2509 <.text+0x22a1509>
14229be74:     	movq	%rax, %rbx
14229be77:     	movups	0xda04c3(%rip), %xmm0   # 0x14303c341
14229be7e:     	movups	%xmm0, (%rax)
14229be81:     	movabsq	$0x6e656572675f666f, %rax # imm = 0x6E656572675F666F
14229be8b:     	movq	%rax, 0xd(%rbx)
14229be8f:     	movzbl	0x1398d73(%rip), %eax   # 0x143634c09
14229be96:     	movl	$0x11, %ecx
14229be9b:     	movl	$0x1, %edx
14229bea0:     	callq	0x140613c10 <.text+0x612c10>
14229bea5:     	testq	%rax, %rax
14229bea8:     	je	0x1422a2356 <.text+0x22a1356>
14229beae:     	movq	%rax, %r14
14229beb1:     	movups	0xda03d8(%rip), %xmm6   # 0x14303c290
14229beb8:     	movups	%xmm6, (%rax)
14229bebb:     	movb	$0x64, 0x10(%rax)
14229bebf:     	movzbl	0x1398d43(%rip), %eax   # 0x143634c09
14229bec6:     	movl	$0x11, %ecx
14229becb:     	movl	$0x1, %edx
14229bed0:     	callq	0x140613c10 <.text+0x612c10>
14229bed5:     	testq	%rax, %rax
14229bed8:     	je	0x1422a21e0 <.text+0x22a11e0>
14229bede:     	movq	%rax, %r12
14229bee1:     	movups	%xmm6, (%rax)
14229bee4:     	movb	$0x64, 0x10(%rax)
14229bee8:     	movzbl	0x1398d1a(%rip), %eax   # 0x143634c09
14229beef:     	movl	$0x11, %ecx
14229bef4:     	movl	$0x1, %edx
14229bef9:     	callq	0x140613c10 <.text+0x612c10>
14229befe:     	testq	%rax, %rax
14229bf01:     	je	0x1422a213b <.text+0x22a113b>
14229bf07:     	movq	%rax, %r13
14229bf0a:     	movups	%xmm6, (%rax)
14229bf0d:     	movb	$0x64, 0x10(%rax)
14229bf11:     	movzbl	0x1398cf1(%rip), %eax   # 0x143634c09
14229bf18:     	movl	$0x11, %ecx
14229bf1d:     	movl	$0x1, %edx
14229bf22:     	callq	0x140613c10 <.text+0x612c10>
14229bf27:     	testq	%rax, %rax
14229bf2a:     	je	0x1422a2043 <.text+0x22a1043>
14229bf30:     	movups	%xmm6, (%rax)
14229bf33:     	movq	%rax, 0x1f0(%rbp)
14229bf3a:     	movb	$0x64, 0x10(%rax)
14229bf3e:     	movzbl	0x1398cc4(%rip), %eax   # 0x143634c09
14229bf45:     	movl	$0x11, %ecx
14229bf4a:     	movl	$0x1, %edx
14229bf4f:     	callq	0x140613c10 <.text+0x612c10>
14229bf54:     	testq	%rax, %rax
14229bf57:     	je	0x1422a1df9 <.text+0x22a0df9>
14229bf5d:     	movups	%xmm6, (%rax)
14229bf60:     	movq	%rax, 0x1e8(%rbp)
14229bf67:     	movb	$0x64, 0x10(%rax)
14229bf6b:     	movzbl	0x1398c97(%rip), %eax   # 0x143634c09
14229bf72:     	movl	$0x11, %ecx
14229bf77:     	movl	$0x1, %edx
14229bf7c:     	callq	0x140613c10 <.text+0x612c10>
14229bf81:     	testq	%rax, %rax
14229bf84:     	je	0x1422a23c4 <.text+0x22a13c4>
14229bf8a:     	movups	%xmm6, (%rax)
14229bf8d:     	movb	$0x64, 0x10(%rax)
14229bf91:     	movq	%rax, 0x1d8(%rbp)
14229bf98:     	movzbl	0x1398c6a(%rip), %eax   # 0x143634c09
14229bf9f:     	movl	$0x11, %ecx
14229bfa4:     	movl	$0x1, %edx
14229bfa9:     	callq	0x140613c10 <.text+0x612c10>
14229bfae:     	testq	%rax, %rax
14229bfb1:     	je	0x1422a1f89 <.text+0x22a0f89>
14229bfb7:     	movups	%xmm6, (%rax)
14229bfba:     	movb	$0x64, 0x10(%rax)
14229bfbe:     	movq	$0x13, (%rsi)
14229bfc5:     	movq	%rdi, 0x8(%rsi)
14229bfc9:     	movq	$0x13, 0x10(%rsi)
14229bfd1:     	movb	$0x3, 0x18(%rsi)
14229bfd5:     	movl	$0x0, 0x1c(%rsi)
14229bfdc:     	movl	$0x0, 0x24(%rsi)
14229bfe3:     	movq	$0x15, 0x28(%rsi)
14229bfeb:     	movq	%rbx, 0x30(%rsi)
14229bfef:     	movq	$0x15, 0x38(%rsi)
14229bff7:     	movb	$0x3, 0x40(%rsi)
14229bffb:     	movl	$0x4, 0x44(%rsi)
14229c002:     	movl	$0x0, 0x4c(%rsi)
14229c009:     	movq	$0x11, 0x50(%rsi)
14229c011:     	movq	%r14, 0x58(%rsi)
14229c015:     	movq	$0x11, 0x60(%rsi)
14229c01d:     	movb	$0x3, 0x68(%rsi)
14229c021:     	movl	$0x309, 0x6c(%rsi)      # imm = 0x309
14229c028:     	movl	$0x0, 0x74(%rsi)
14229c02f:     	movq	$0x11, 0x78(%rsi)
14229c037:     	movq	%r12, 0x80(%rsi)
14229c03e:     	movq	$0x11, 0x88(%rsi)
14229c049:     	movb	$0x3, 0x90(%rsi)
14229c050:     	movl	$0x309, 0x94(%rsi)      # imm = 0x309
14229c05a:     	movl	$0x0, 0x9c(%rsi)
14229c064:     	movq	$0x11, 0xa0(%rsi)
14229c06f:     	movq	%r13, 0xa8(%rsi)
14229c076:     	movq	$0x11, 0xb0(%rsi)
14229c081:     	movb	$0x3, 0xb8(%rsi)
14229c088:     	movl	$0x309, 0xbc(%rsi)      # imm = 0x309
14229c092:     	movl	$0x0, 0xc4(%rsi)
14229c09c:     	movq	$0x11, 0xc8(%rsi)
14229c0a7:     	movq	0x1f0(%rbp), %rcx
14229c0ae:     	movq	%rcx, 0xd0(%rsi)
14229c0b5:     	movq	$0x11, 0xd8(%rsi)
14229c0c0:     	movb	$0x3, 0xe0(%rsi)
14229c0c7:     	movl	$0x309, 0xe4(%rsi)      # imm = 0x309
14229c0d1:     	movl	$0x0, 0xec(%rsi)
14229c0db:     	movq	$0x11, 0xf0(%rsi)
14229c0e6:     	movq	0x1e8(%rbp), %rcx
14229c0ed:     	movq	%rcx, 0xf8(%rsi)
14229c0f4:     	movq	$0x11, 0x100(%rsi)
14229c0ff:     	movb	$0x3, 0x108(%rsi)
14229c106:     	movl	$0x309, 0x10c(%rsi)     # imm = 0x309
14229c110:     	movl	$0x0, 0x114(%rsi)
14229c11a:     	movq	$0x11, 0x118(%rsi)
14229c125:     	movq	0x1d8(%rbp), %rcx
14229c12c:     	movq	%rcx, 0x120(%rsi)
14229c133:     	movq	$0x11, 0x128(%rsi)
14229c13e:     	movb	$0x3, 0x130(%rsi)
14229c145:     	movl	$0x309, 0x134(%rsi)     # imm = 0x309
14229c14f:     	movl	$0x0, 0x13c(%rsi)
14229c159:     	movq	$0x11, 0x140(%rsi)
14229c164:     	movq	%rax, 0x148(%rsi)
14229c16b:     	movq	$0x11, 0x150(%rsi)
14229c176:     	movb	$0x3, 0x158(%rsi)
14229c17d:     	movl	$0x309, 0x15c(%rsi)     # imm = 0x309
14229c187:     	movl	$0x0, 0x164(%rsi)
14229c191:     	movq	$0x9, 0x110(%rbp)
14229c19c:     	movq	%rsi, 0x118(%rbp)
14229c1a3:     	movq	$0x9, 0x120(%rbp)
14229c1ae:     	jmp	0x14229e9e0 <.text+0x229d9e0>
14229c1b3:     	movq	%rdx, 0x1e0(%rbp)
14229c1ba:     	movzbl	0x1398a48(%rip), %eax   # 0x143634c09
14229c1c1:     	movl	$0x190, %ecx            # imm = 0x190
14229c1c6:     	movl	$0x8, %edx
14229c1cb:     	callq	0x140613c10 <.text+0x612c10>
14229c1d0:     	testq	%rax, %rax
14229c1d3:     	je	0x1422a0f87 <.text+0x229ff87>
14229c1d9:     	movq	%rax, %rsi
14229c1dc:     	movzbl	0x1398a26(%rip), %eax   # 0x143634c09
14229c1e3:     	movl	$0x17, %ecx
14229c1e8:     	movl	$0x1, %edx
14229c1ed:     	callq	0x140613c10 <.text+0x612c10>
14229c1f2:     	testq	%rax, %rax
14229c1f5:     	je	0x1422a13a9 <.text+0x22a03a9>
14229c1fb:     	movq	%rax, %rdi
14229c1fe:     	movups	0xd9ff53(%rip), %xmm0   # 0x14303c158
14229c205:     	movups	%xmm0, (%rax)
14229c208:     	movabsq	$0x65676965625f736b, %rax # imm = 0x65676965625F736B
14229c212:     	movq	%rax, 0xf(%rdi)
14229c216:     	movzbl	0x13989ec(%rip), %eax   # 0x143634c09
14229c21d:     	movl	$0x18, %ecx
14229c222:     	movl	$0x1, %edx
14229c227:     	callq	0x140613c10 <.text+0x612c10>
14229c22c:     	testq	%rax, %rax
14229c22f:     	je	0x1422a1303 <.text+0x22a0303>
14229c235:     	movq	%rax, %rbx
14229c238:     	movups	0xd9ff30(%rip), %xmm0   # 0x14303c16f
14229c23f:     	movups	%xmm0, (%rax)
14229c242:     	movabsq	$0x776f6c6c65795f73, %rax # imm = 0x776F6C6C65795F73
14229c24c:     	movq	%rax, 0x10(%rbx)
14229c250:     	movzbl	0x13989b2(%rip), %eax   # 0x143634c09
14229c257:     	movl	$0x17, %ecx
14229c25c:     	movl	$0x1, %edx
14229c261:     	callq	0x140613c10 <.text+0x612c10>
14229c266:     	testq	%rax, %rax
14229c269:     	je	0x1422a1968 <.text+0x22a0968>
14229c26f:     	movq	%rax, %r14
14229c272:     	movups	0xd9ff0e(%rip), %xmm0   # 0x14303c187
14229c279:     	movups	%xmm0, (%rax)
14229c27c:     	movabsq	$0x746867696c5f736b, %rax # imm = 0x746867696C5F736B
14229c286:     	movq	%rax, 0xf(%r14)
14229c28a:     	movzbl	0x1398978(%rip), %eax   # 0x143634c09
14229c291:     	movl	$0x16, %ecx
14229c296:     	movl	$0x1, %edx
14229c29b:     	callq	0x140613c10 <.text+0x612c10>
14229c2a0:     	testq	%rax, %rax
14229c2a3:     	je	0x1422a187d <.text+0x22a087d>
14229c2a9:     	movq	%rax, %r12
14229c2ac:     	movups	0xd9feeb(%rip), %xmm0   # 0x14303c19e
14229c2b3:     	movups	%xmm0, (%rax)
14229c2b6:     	movabsq	$0x796572675f736b63, %rax # imm = 0x796572675F736B63
14229c2c0:     	movq	%rax, 0xe(%r12)
14229c2c5:     	movzbl	0x139893d(%rip), %eax   # 0x143634c09
14229c2cc:     	movl	$0x16, %ecx
14229c2d1:     	movl	$0x1, %edx
14229c2d6:     	callq	0x140613c10 <.text+0x612c10>
14229c2db:     	testq	%rax, %rax
14229c2de:     	je	0x1422a19ba <.text+0x22a09ba>
14229c2e4:     	movq	%rax, %r13
14229c2e7:     	movups	0xd9fec6(%rip), %xmm0   # 0x14303c1b4
14229c2ee:     	movups	%xmm0, (%rax)
14229c2f1:     	movabsq	$0x6b7261645f736b63, %rax # imm = 0x6B7261645F736B63
14229c2fb:     	movq	%rax, 0xe(%r13)
14229c2ff:     	movzbl	0x1398903(%rip), %eax   # 0x143634c09
14229c306:     	movl	$0x5, %ecx
14229c30b:     	movl	$0x1, %edx
14229c310:     	callq	0x140613c10 <.text+0x612c10>
14229c315:     	testq	%rax, %rax
14229c318:     	je	0x1422a1838 <.text+0x22a0838>
14229c31e:     	movzbl	0xe9aa32(%rip), %ecx    # 0x143136d57
14229c325:     	movb	%cl, 0x4(%rax)
14229c328:     	movl	0xe9aa25(%rip), %ecx    # 0x143136d53
14229c32e:     	movq	%rax, 0x1e8(%rbp)
14229c335:     	movl	%ecx, (%rax)
14229c337:     	movzbl	0x13988cb(%rip), %eax   # 0x143634c09
14229c33e:     	movl	$0x14, %ecx
14229c343:     	movl	$0x1, %edx
14229c348:     	callq	0x140613c10 <.text+0x612c10>
14229c34d:     	testq	%rax, %rax
14229c350:     	je	0x1422a1402 <.text+0x22a0402>
14229c356:     	movl	0xe9aa83(%rip), %ecx    # 0x143136ddf
14229c35c:     	movl	%ecx, 0x10(%rax)
14229c35f:     	movups	0xe9aa69(%rip), %xmm0   # 0x143136dcf
14229c366:     	movq	%rax, 0x1d8(%rbp)
14229c36d:     	movups	%xmm0, (%rax)
14229c370:     	movzbl	0x1398892(%rip), %eax   # 0x143634c09
14229c377:     	movl	$0x14, %ecx
14229c37c:     	movl	$0x1, %edx
14229c381:     	callq	0x140613c10 <.text+0x612c10>
14229c386:     	testq	%rax, %rax
14229c389:     	je	0x1422a1923 <.text+0x22a0923>
14229c38f:     	movq	%r15, 0x1f0(%rbp)
14229c396:     	movl	0xe9aa57(%rip), %ecx    # 0x143136df3
14229c39c:     	movl	%ecx, 0x10(%rax)
14229c39f:     	movups	0xe9aa3d(%rip), %xmm0   # 0x143136de3
14229c3a6:     	movq	%rax, 0x1c0(%rbp)
14229c3ad:     	movups	%xmm0, (%rax)
14229c3b0:     	movzbl	0x1398852(%rip), %eax   # 0x143634c09
14229c3b7:     	movl	$0x14, %ecx
14229c3bc:     	movl	$0x1, %edx
14229c3c1:     	callq	0x140613c10 <.text+0x612c10>
14229c3c6:     	testq	%rax, %rax
14229c3c9:     	je	0x1422a1355 <.text+0x22a0355>
14229c3cf:     	movl	0xe9aa32(%rip), %ecx    # 0x143136e07
14229c3d5:     	movl	%ecx, 0x10(%rax)
14229c3d8:     	movups	0xe9aa18(%rip), %xmm0   # 0x143136df7
14229c3df:     	movups	%xmm0, (%rax)
14229c3e2:     	movzbl	0x1398820(%rip), %ecx   # 0x143634c09
14229c3e9:     	movq	%rax, %r15
14229c3ec:     	movl	$0x14, %ecx
14229c3f1:     	movl	$0x1, %edx
14229c3f6:     	callq	0x140613c10 <.text+0x612c10>
14229c3fb:     	testq	%rax, %rax
14229c3fe:     	je	0x1422a126b <.text+0x22a026b>
14229c404:     	movl	0xe9aa11(%rip), %ecx    # 0x143136e1b
14229c40a:     	movl	%ecx, 0x10(%rax)
14229c40d:     	movups	0xe9a9f7(%rip), %xmm0   # 0x143136e0b
14229c414:     	movups	%xmm0, (%rax)
14229c417:     	movq	$0x17, (%rsi)
14229c41e:     	movq	%rdi, 0x8(%rsi)
14229c422:     	movq	$0x17, 0x10(%rsi)
14229c42a:     	movb	$0x3, 0x18(%rsi)
14229c42e:     	movl	$0x186a0, 0x1c(%rsi)    # imm = 0x186A0
14229c435:     	movl	$0x0, 0x24(%rsi)
14229c43c:     	movq	$0x18, 0x28(%rsi)
14229c444:     	movq	%rbx, 0x30(%rsi)
14229c448:     	movq	$0x18, 0x38(%rsi)
14229c450:     	movb	$0x3, 0x40(%rsi)
14229c454:     	movl	$0x186a1, 0x44(%rsi)    # imm = 0x186A1
14229c45b:     	movl	$0x0, 0x4c(%rsi)
14229c462:     	movq	$0x17, 0x50(%rsi)
14229c46a:     	movq	%r14, 0x58(%rsi)
14229c46e:     	movq	$0x17, 0x60(%rsi)
14229c476:     	movb	$0x3, 0x68(%rsi)
14229c47a:     	movl	$0x186a3, 0x6c(%rsi)    # imm = 0x186A3
14229c481:     	movl	$0x0, 0x74(%rsi)
14229c488:     	movq	$0x16, 0x78(%rsi)
14229c490:     	movq	%r12, 0x80(%rsi)
14229c497:     	movq	$0x16, 0x88(%rsi)
14229c4a2:     	movb	$0x3, 0x90(%rsi)
14229c4a9:     	movl	$0x186ab, 0x94(%rsi)    # imm = 0x186AB
14229c4b3:     	movl	$0x0, 0x9c(%rsi)
14229c4bd:     	movq	$0x16, 0xa0(%rsi)
14229c4c8:     	movq	%r13, 0xa8(%rsi)
14229c4cf:     	movq	$0x16, 0xb0(%rsi)
14229c4da:     	movb	$0x3, 0xb8(%rsi)
14229c4e1:     	movl	$0x186a2, 0xbc(%rsi)    # imm = 0x186A2
14229c4eb:     	movl	$0x0, 0xc4(%rsi)
14229c4f5:     	movq	$0x5, 0xc8(%rsi)
14229c500:     	movq	0x1e8(%rbp), %rcx
14229c507:     	movq	%rcx, 0xd0(%rsi)
14229c50e:     	movq	$0x5, 0xd8(%rsi)
14229c519:     	movb	$0x5, 0xe0(%rsi)
14229c520:     	movl	$0x0, 0xec(%rsi)
14229c52a:     	movq	$0x14, 0xf0(%rsi)
14229c535:     	movq	0x1d8(%rbp), %rcx
14229c53c:     	movq	%rcx, 0xf8(%rsi)
14229c543:     	movq	$0x14, 0x100(%rsi)
14229c54e:     	movw	$0xd, 0x108(%rsi)
14229c557:     	movl	$0x0, 0x114(%rsi)
14229c561:     	movq	$0x14, 0x118(%rsi)
14229c56c:     	movq	0x1c0(%rbp), %rcx
14229c573:     	movq	%rcx, 0x120(%rsi)
14229c57a:     	movq	$0x14, 0x128(%rsi)
14229c585:     	movw	$0x10d, 0x130(%rsi)     # imm = 0x10D
14229c58e:     	movl	$0x0, 0x13c(%rsi)
14229c598:     	movq	$0x14, 0x140(%rsi)
14229c5a3:     	movq	%r15, 0x148(%rsi)
14229c5aa:     	movq	$0x14, 0x150(%rsi)
14229c5b5:     	movw	$0x20d, 0x158(%rsi)     # imm = 0x20D
14229c5be:     	movl	$0x0, 0x164(%rsi)
14229c5c8:     	movq	$0x14, 0x168(%rsi)
14229c5d3:     	movq	%rax, 0x170(%rsi)
14229c5da:     	movq	$0x14, 0x178(%rsi)
14229c5e5:     	movw	$0x30d, 0x180(%rsi)     # imm = 0x30D
14229c5ee:     	movl	$0x0, 0x18c(%rsi)
14229c5f8:     	movq	$0xa, 0x110(%rbp)
14229c603:     	movq	%rsi, 0x118(%rbp)
14229c60a:     	movq	$0xa, 0x120(%rbp)
14229c615:     	movq	0x1f0(%rbp), %r15
14229c61c:     	jmp	0x14229e9e0 <.text+0x229d9e0>
14229c621:     	xorl	%edi, %edi
14229c623:     	movzbl	0x13985df(%rip), %eax   # 0x143634c09
14229c62a:     	movl	$0xc8, %ecx
14229c62f:     	movl	$0x8, %edx
14229c634:     	callq	0x140613c10 <.text+0x612c10>
14229c639:     	testq	%rax, %rax
14229c63c:     	je	0x1422a1ab9 <.text+0x22a0ab9>
14229c642:     	movq	%rax, %r14
14229c645:     	movl	%r13d, 0x1e8(%rbp)
14229c64c:     	movzbl	0x13985b6(%rip), %eax   # 0x143634c09
14229c653:     	movl	$0x17, %ecx
14229c658:     	movl	$0x1, %edx
14229c65d:     	callq	0x140613c10 <.text+0x612c10>
14229c662:     	testq	%rax, %rax
14229c665:     	je	0x1422a1a97 <.text+0x22a0a97>
14229c66b:     	movq	%rax, %r12
14229c66e:     	movups	0xd9fae3(%rip), %xmm0   # 0x14303c158
14229c675:     	movups	%xmm0, (%rax)
14229c678:     	movabsq	$0x65676965625f736b, %rax # imm = 0x65676965625F736B
14229c682:     	movq	%rax, 0xf(%r12)
14229c687:     	movzbl	0x139857b(%rip), %eax   # 0x143634c09
14229c68e:     	movl	$0x18, %ecx
14229c693:     	movl	$0x1, %edx
14229c698:     	callq	0x140613c10 <.text+0x612c10>
14229c69d:     	testq	%rax, %rax
14229c6a0:     	je	0x1422a1b28 <.text+0x22a0b28>
14229c6a6:     	movq	%rax, %r13
14229c6a9:     	movups	0xd9fabf(%rip), %xmm0   # 0x14303c16f
14229c6b0:     	movups	%xmm0, (%rax)
14229c6b3:     	movabsq	$0x776f6c6c65795f73, %rax # imm = 0x776F6C6C65795F73
14229c6bd:     	movq	%rax, 0x10(%r13)
14229c6c1:     	movzbl	0x1398541(%rip), %eax   # 0x143634c09
14229c6c8:     	movl	$0x17, %ecx
14229c6cd:     	movl	$0x1, %edx
14229c6d2:     	callq	0x140613c10 <.text+0x612c10>
14229c6d7:     	testq	%rax, %rax
14229c6da:     	je	0x1422a1ac8 <.text+0x22a0ac8>
14229c6e0:     	movq	%rax, %rcx
14229c6e3:     	movups	0xd9fa9d(%rip), %xmm0   # 0x14303c187
14229c6ea:     	movups	%xmm0, (%rax)
14229c6ed:     	movabsq	$0x746867696c5f736b, %rax # imm = 0x746867696C5F736B
14229c6f7:     	movq	%rcx, 0x1f0(%rbp)
14229c6fe:     	movq	%rax, 0xf(%rcx)
14229c702:     	movzbl	0x1398500(%rip), %eax   # 0x143634c09
14229c709:     	movl	$0x16, %ecx
14229c70e:     	movl	$0x1, %edx
14229c713:     	callq	0x140613c10 <.text+0x612c10>
14229c718:     	testq	%rax, %rax
14229c71b:     	je	0x1422a1af8 <.text+0x22a0af8>
14229c721:     	movq	%rax, %rsi
14229c724:     	movups	0xd9fa73(%rip), %xmm0   # 0x14303c19e
14229c72b:     	movups	%xmm0, (%rax)
14229c72e:     	movabsq	$0x796572675f736b63, %rax # imm = 0x796572675F736B63
14229c738:     	movq	%rax, 0xe(%rsi)
14229c73c:     	movzbl	0x13984c6(%rip), %eax   # 0x143634c09
14229c743:     	movl	$0x16, %ecx
14229c748:     	movl	$0x1, %edx
14229c74d:     	callq	0x140613c10 <.text+0x612c10>
14229c752:     	testq	%rax, %rax
14229c755:     	je	0x1422a1b51 <.text+0x22a0b51>
14229c75b:     	movups	0xd9fa52(%rip), %xmm0   # 0x14303c1b4
14229c762:     	movups	%xmm0, (%rax)
14229c765:     	movabsq	$0x6b7261645f736b63, %rcx # imm = 0x6B7261645F736B63
14229c76f:     	movq	%rcx, 0xe(%rax)
14229c773:     	movq	$0x17, (%r14)
14229c77a:     	movq	%r12, 0x8(%r14)
14229c77e:     	movq	$0x17, 0x10(%r14)
14229c786:     	movw	$0x2, 0x18(%r14)
14229c78d:     	movl	$0x0, 0x1c(%r14)
14229c795:     	movl	%edi, 0x20(%r14)
14229c799:     	movl	$0x0, 0x24(%r14)
14229c7a1:     	movq	$0x18, 0x28(%r14)
14229c7a9:     	movq	%r13, 0x30(%r14)
14229c7ad:     	movq	$0x18, 0x38(%r14)
14229c7b5:     	movw	$0x2, 0x40(%r14)
14229c7bc:     	movl	$0x1, 0x44(%r14)
14229c7c4:     	movl	%edi, 0x48(%r14)
14229c7c8:     	movl	$0x0, 0x4c(%r14)
14229c7d0:     	movq	$0x17, 0x50(%r14)
14229c7d8:     	movq	0x1f0(%rbp), %rcx
14229c7df:     	movq	%rcx, 0x58(%r14)
14229c7e3:     	movq	$0x17, 0x60(%r14)
14229c7eb:     	movw	$0x2, 0x68(%r14)
14229c7f2:     	movl	$0x3, 0x6c(%r14)
14229c7fa:     	movl	%edi, 0x70(%r14)
14229c7fe:     	movl	$0x0, 0x74(%r14)
14229c806:     	movq	$0x16, 0x78(%r14)
14229c80e:     	movq	%rsi, 0x80(%r14)
14229c815:     	movq	$0x16, 0x88(%r14)
14229c820:     	movw	$0x2, 0x90(%r14)
14229c82a:     	movl	$0xb, 0x94(%r14)
14229c835:     	movl	%edi, 0x98(%r14)
14229c83c:     	movl	$0x0, 0x9c(%r14)
14229c847:     	movq	$0x16, 0xa0(%r14)
14229c852:     	movq	%rax, 0xa8(%r14)
14229c859:     	movq	$0x16, 0xb0(%r14)
14229c864:     	movw	$0x2, 0xb8(%r14)
14229c86e:     	movl	$0x2, 0xbc(%r14)
14229c879:     	movl	%edi, 0xc0(%r14)
14229c880:     	movq	%r14, %rax
14229c883:     	addq	$0xc4, %rax
14229c889:     	movl	$0x5, %ecx
14229c88e:     	movl	%ebx, %esi
14229c890:     	movl	$0x0, (%rax)
14229c896:     	movq	%rcx, 0xd8(%rbp)
14229c89d:     	movq	%r14, 0xe0(%rbp)
14229c8a4:     	movq	%rcx, 0xe8(%rbp)
14229c8ab:     	movzbl	0x1398357(%rip), %eax   # 0x143634c09
14229c8b2:     	movl	$0x118, %ecx            # imm = 0x118
14229c8b7:     	movl	$0x8, %edx
14229c8bc:     	callq	0x140613c10 <.text+0x612c10>
14229c8c1:     	testq	%rax, %rax
14229c8c4:     	je	0x1422a15d6 <.text+0x22a05d6>
14229c8ca:     	movq	%rax, %rbx
14229c8cd:     	movzbl	0x1398335(%rip), %eax   # 0x143634c09
14229c8d4:     	movl	$0x5, %ecx
14229c8d9:     	movl	$0x1, %edx
14229c8de:     	callq	0x140613c10 <.text+0x612c10>
14229c8e3:     	testq	%rax, %rax
14229c8e6:     	je	0x1422a15ea <.text+0x22a05ea>
14229c8ec:     	movq	%rax, %r14
14229c8ef:     	movzbl	0xe9a461(%rip), %eax    # 0x143136d57
14229c8f6:     	movb	%al, 0x4(%r14)
14229c8fa:     	movl	0xe9a453(%rip), %eax    # 0x143136d53
14229c900:     	movl	%eax, (%r14)
14229c903:     	movzbl	0x13982ff(%rip), %eax   # 0x143634c09
14229c90a:     	movl	$0x10, %ecx
14229c90f:     	movl	$0x1, %edx
14229c914:     	callq	0x140613c10 <.text+0x612c10>
14229c919:     	testq	%rax, %rax
14229c91c:     	je	0x1422a160c <.text+0x22a060c>
14229c922:     	movq	%rax, %r12
14229c925:     	movups	0xd9fb00(%rip), %xmm0   # 0x14303c42c
14229c92c:     	movups	%xmm0, (%rax)
14229c92f:     	movzbl	0x13982d3(%rip), %eax   # 0x143634c09
14229c936:     	movl	$0x12, %ecx
14229c93b:     	movl	$0x1, %edx
14229c940:     	callq	0x140613c10 <.text+0x612c10>
14229c945:     	testq	%rax, %rax
14229c948:     	je	0x1422a1635 <.text+0x22a0635>
14229c94e:     	movq	%rax, %r13
14229c951:     	movups	0xd9fae4(%rip), %xmm0   # 0x14303c43c
14229c958:     	movups	%xmm0, (%rax)
14229c95b:     	movw	$0x6e6f, 0x10(%rax)     # imm = 0x6E6F
14229c961:     	movzbl	0x13982a1(%rip), %eax   # 0x143634c09
14229c968:     	movl	$0x12, %ecx
14229c96d:     	movl	$0x1, %edx
14229c972:     	callq	0x140613c10 <.text+0x612c10>
14229c977:     	testq	%rax, %rax
14229c97a:     	je	0x1422a1665 <.text+0x22a0665>
14229c980:     	movq	%r15, 0x1f0(%rbp)
14229c987:     	movups	0xd9fac0(%rip), %xmm0   # 0x14303c44e
14229c98e:     	movups	%xmm0, (%rax)
14229c991:     	movq	%rax, 0x1d8(%rbp)
14229c998:     	movw	$0x6e6f, 0x10(%rax)     # imm = 0x6E6F
14229c99e:     	movzbl	0x1398264(%rip), %eax   # 0x143634c09
14229c9a5:     	movl	$0x11, %ecx
14229c9aa:     	movl	$0x1, %edx
14229c9af:     	callq	0x140613c10 <.text+0x612c10>
14229c9b4:     	testq	%rax, %rax
14229c9b7:     	je	0x1422a169c <.text+0x22a069c>
14229c9bd:     	movl	%esi, %r15d
14229c9c0:     	movl	0x1e8(%rbp), %edi
14229c9c6:     	movups	0xd9fa93(%rip), %xmm0   # 0x14303c460
14229c9cd:     	movups	%xmm0, (%rax)
14229c9d0:     	movq	%rax, 0x1c0(%rbp)
14229c9d7:     	movb	$0x6e, 0x10(%rax)
14229c9db:     	movzbl	0x1398227(%rip), %eax   # 0x143634c09
14229c9e2:     	movl	$0x13, %ecx
14229c9e7:     	movl	$0x1, %edx
14229c9ec:     	callq	0x140613c10 <.text+0x612c10>
14229c9f1:     	testq	%rax, %rax
14229c9f4:     	je	0x1422a1447 <.text+0x22a0447>
14229c9fa:     	movups	0xd9fa70(%rip), %xmm0   # 0x14303c471
14229ca01:     	movups	%xmm0, (%rax)
14229ca04:     	movl	$0x6e6f6369, 0xf(%rax)  # imm = 0x6E6F6369
14229ca0b:     	movq	%rax, %rsi
14229ca0e:     	movzbl	0x13981f4(%rip), %eax   # 0x143634c09
14229ca15:     	movl	$0x13, %ecx
14229ca1a:     	movl	$0x1, %edx
14229ca1f:     	callq	0x140613c10 <.text+0x612c10>
14229ca24:     	testq	%rax, %rax
14229ca27:     	je	0x1422a18c3 <.text+0x22a08c3>
14229ca2d:     	movups	0xd9fa50(%rip), %xmm0   # 0x14303c484
14229ca34:     	movups	%xmm0, (%rax)
14229ca37:     	movl	$0x6e6f6369, 0xf(%rax)  # imm = 0x6E6F6369
14229ca3e:     	movq	$0x5, (%rbx)
14229ca45:     	movq	%r14, 0x8(%rbx)
14229ca49:     	movq	$0x5, 0x10(%rbx)
14229ca51:     	movb	$0x5, 0x18(%rbx)
14229ca55:     	movl	$0x0, 0x24(%rbx)
14229ca5c:     	movq	$0x10, 0x28(%rbx)
14229ca64:     	movq	%r12, 0x30(%rbx)
14229ca68:     	movq	$0x10, 0x38(%rbx)
14229ca70:     	movw	$0x2, 0x40(%rbx)
14229ca76:     	movl	%edi, 0x44(%rbx)
14229ca79:     	movq	$0x0, 0x48(%rbx)
14229ca81:     	movq	$0x12, 0x50(%rbx)
14229ca89:     	movq	%r13, 0x58(%rbx)
14229ca8d:     	movq	$0x12, 0x60(%rbx)
14229ca95:     	movw	$0x2, 0x68(%rbx)
14229ca9b:     	movl	%edi, 0x6c(%rbx)
14229ca9e:     	movq	$0x1, 0x70(%rbx)
14229caa6:     	movq	$0x12, 0x78(%rbx)
14229caae:     	movq	0x1d8(%rbp), %rcx
14229cab5:     	movq	%rcx, 0x80(%rbx)
14229cabc:     	movq	$0x12, 0x88(%rbx)
14229cac7:     	movw	$0x2, 0x90(%rbx)
14229cad0:     	movl	%edi, 0x94(%rbx)
14229cad6:     	movq	$0x2, 0x98(%rbx)
14229cae1:     	movq	$0x11, 0xa0(%rbx)
14229caec:     	movq	0x1c0(%rbp), %rcx
14229caf3:     	movq	%rcx, 0xa8(%rbx)
14229cafa:     	movq	$0x11, 0xb0(%rbx)
14229cb05:     	movw	$0x102, 0xb8(%rbx)      # imm = 0x102
14229cb0e:     	movl	%r15d, 0xbc(%rbx)
14229cb15:     	movq	$0x0, 0xc0(%rbx)
14229cb20:     	movq	$0x13, 0xc8(%rbx)
14229cb2b:     	movq	%rsi, 0xd0(%rbx)
14229cb32:     	movq	$0x13, 0xd8(%rbx)
14229cb3d:     	movw	$0x102, 0xe0(%rbx)      # imm = 0x102
14229cb46:     	movl	%r15d, 0xe4(%rbx)
14229cb4d:     	movq	$0x2, 0xe8(%rbx)
14229cb58:     	movq	$0x13, 0xf0(%rbx)
14229cb63:     	movq	%rax, 0xf8(%rbx)
14229cb6a:     	movq	$0x13, 0x100(%rbx)
14229cb75:     	movw	$0x102, 0x108(%rbx)     # imm = 0x102
14229cb7e:     	movl	%r15d, 0x10c(%rbx)
14229cb85:     	movq	$0x1, 0x110(%rbx)
14229cb90:     	movq	%rbx, %rax
14229cb93:     	addq	$0x118, %rax            # imm = 0x118
14229cb99:     	movq	%rbx, 0x180(%rbp)
14229cba0:     	movq	$0x7, 0x190(%rbp)
14229cbab:     	movq	%rbx, 0x188(%rbp)
14229cbb2:     	movq	%rax, 0x198(%rbp)
14229cbb9:     	movq	0xd8(%rbp), %rax
14229cbc0:     	movq	0xe8(%rbp), %rdi
14229cbc7:     	subq	%rdi, %rax
14229cbca:     	cmpq	$0x7, %rax
14229cbce:     	jb	0x1422a0c8c <.text+0x229fc8c>
14229cbd4:     	movq	0x1f0(%rbp), %r15
14229cbdb:     	leaq	(%rdi,%rdi,4), %rcx
14229cbdf:     	shlq	$0x3, %rcx
14229cbe3:     	addq	0xe0(%rbp), %rcx
14229cbea:     	movl	$0x118, %r8d            # imm = 0x118
14229cbf0:     	movq	%rbx, %rdx
14229cbf3:     	callq	0x142923c00 <.text+0x2922c00>
14229cbf8:     	addq	$0x7, %rdi
14229cbfc:     	movq	%rdi, 0xe8(%rbp)
14229cc03:     	movl	$0x118, %edx            # imm = 0x118
14229cc08:     	movl	$0x8, %r8d
14229cc0e:     	movq	%rbx, %rcx
14229cc11:     	callq	0x140613c20 <.text+0x612c20>
14229cc16:     	movups	0xd8(%rbp), %xmm0
14229cc1d:     	movaps	%xmm0, 0x110(%rbp)
14229cc24:     	movq	0xe8(%rbp), %rax
14229cc2b:     	movq	%rax, 0x120(%rbp)
14229cc32:     	jmp	0x14229e9e0 <.text+0x229d9e0>
14229cc37:     	movq	0x1f8(%rbp), %rcx
14229cc3e:     	leaq	0xd9f4fb(%rip), %r8     # 0x14303c140
14229cc45:     	callq	0x140a91ef0 <.text+0xa90ef0>
14229cc4a:     	testq	%rax, %rax
14229cc4d:     	je	0x1422a0db7 <.text+0x229fdb7>
14229cc53:     	movq	%rax, %r12
14229cc56:     	movzbl	0x1397fac(%rip), %eax   # 0x143634c09
14229cc5d:     	movl	$0x208, %ecx            # imm = 0x208
14229cc62:     	movl	$0x8, %edx
14229cc67:     	callq	0x140613c10 <.text+0x612c10>
14229cc6c:     	testq	%rax, %rax
14229cc6f:     	je	0x1422a27a3 <.text+0x22a17a3>
14229cc75:     	movq	%rax, %rbx
14229cc78:     	movzbl	0x1397f8a(%rip), %eax   # 0x143634c09
14229cc7f:     	movl	$0x17, %ecx
14229cc84:     	movl	$0x1, %edx
14229cc89:     	callq	0x140613c10 <.text+0x612c10>
14229cc8e:     	testq	%rax, %rax
14229cc91:     	je	0x1422a27b2 <.text+0x22a17b2>
14229cc97:     	movq	%rax, %r14
14229cc9a:     	movups	0xd9f4b7(%rip), %xmm0   # 0x14303c158
14229cca1:     	movups	%xmm0, (%rax)
14229cca4:     	movabsq	$0x65676965625f736b, %rax # imm = 0x65676965625F736B
14229ccae:     	movq	%rax, 0xf(%r14)
14229ccb2:     	movzbl	0x1397f50(%rip), %eax   # 0x143634c09
14229ccb9:     	movl	$0x18, %ecx
14229ccbe:     	movl	$0x1, %edx
14229ccc3:     	callq	0x140613c10 <.text+0x612c10>
14229ccc8:     	testq	%rax, %rax
14229cccb:     	je	0x1422a27d4 <.text+0x22a17d4>
14229ccd1:     	movq	%rax, %r13
14229ccd4:     	movups	0xd9f494(%rip), %xmm0   # 0x14303c16f
14229ccdb:     	movups	%xmm0, (%rax)
14229ccde:     	movabsq	$0x776f6c6c65795f73, %rax # imm = 0x776F6C6C65795F73
14229cce8:     	movq	%rax, 0x10(%r13)
14229ccec:     	movzbl	0x1397f16(%rip), %eax   # 0x143634c09
14229ccf3:     	movl	$0x17, %ecx
14229ccf8:     	movl	$0x1, %edx
14229ccfd:     	callq	0x140613c10 <.text+0x612c10>
14229cd02:     	testq	%rax, %rax
14229cd05:     	je	0x1422a27fd <.text+0x22a17fd>
14229cd0b:     	movq	%rax, %rsi
14229cd0e:     	movups	0xd9f472(%rip), %xmm0   # 0x14303c187
14229cd15:     	movups	%xmm0, (%rax)
14229cd18:     	movabsq	$0x746867696c5f736b, %rax # imm = 0x746867696C5F736B
14229cd22:     	movq	%rax, 0xf(%rsi)
14229cd26:     	movzbl	0x1397edc(%rip), %eax   # 0x143634c09
14229cd2d:     	movl	$0x16, %ecx
14229cd32:     	movl	$0x1, %edx
14229cd37:     	callq	0x140613c10 <.text+0x612c10>
14229cd3c:     	testq	%rax, %rax
14229cd3f:     	movq	%rsi, 0x1f0(%rbp)
14229cd46:     	je	0x1422a282d <.text+0x22a182d>
14229cd4c:     	movq	%rax, %rcx
14229cd4f:     	movups	0xd9f448(%rip), %xmm0   # 0x14303c19e
14229cd56:     	movups	%xmm0, (%rax)
14229cd59:     	movabsq	$0x796572675f736b63, %rax # imm = 0x796572675F736B63
14229cd63:     	movq	%rcx, 0x1e8(%rbp)
14229cd6a:     	movq	%rax, 0xe(%rcx)
14229cd6e:     	movzbl	0x1397e94(%rip), %eax   # 0x143634c09
14229cd75:     	movl	$0x16, %ecx
14229cd7a:     	movl	$0x1, %edx
14229cd7f:     	callq	0x140613c10 <.text+0x612c10>
14229cd84:     	testq	%rax, %rax
14229cd87:     	je	0x1422a275f <.text+0x22a175f>
14229cd8d:     	movq	%rax, %rcx
14229cd90:     	movups	0xd9f41d(%rip), %xmm0   # 0x14303c1b4
14229cd97:     	movups	%xmm0, (%rax)
14229cd9a:     	movabsq	$0x6b7261645f736b63, %rax # imm = 0x6B7261645F736B63
14229cda4:     	movq	%rcx, 0x1d8(%rbp)
14229cdab:     	movq	%rax, 0xe(%rcx)
14229cdaf:     	movzbl	0x1397e53(%rip), %eax   # 0x143634c09
14229cdb6:     	movl	$0xe, %ecx
14229cdbb:     	movl	$0x1, %edx
14229cdc0:     	callq	0x140613c10 <.text+0x612c10>
14229cdc5:     	testq	%rax, %rax
14229cdc8:     	je	0x1422a26cf <.text+0x22a16cf>
14229cdce:     	movq	%rdi, 0x6(%rax)
14229cdd2:     	movabsq	$0x5f72657473616c70, %rsi # imm = 0x5F72657473616C70
14229cddc:     	movq	%rax, 0x1c0(%rbp)
14229cde3:     	movq	%rsi, (%rax)
14229cde6:     	movzbl	0x1397e1c(%rip), %eax   # 0x143634c09
14229cded:     	movl	$0xe, %ecx
14229cdf2:     	movl	$0x1, %edx
14229cdf7:     	callq	0x140613c10 <.text+0x612c10>
14229cdfc:     	testq	%rax, %rax
14229cdff:     	je	0x1422a2924 <.text+0x22a1924>
14229ce05:     	movq	%rdi, 0x6(%rax)
14229ce09:     	movq	%rax, 0x150(%rbp)
14229ce10:     	movq	%rsi, (%rax)
14229ce13:     	movzbl	0x1397def(%rip), %eax   # 0x143634c09
14229ce1a:     	movl	$0xe, %ecx
14229ce1f:     	movl	$0x1, %edx
14229ce24:     	callq	0x140613c10 <.text+0x612c10>
14229ce29:     	testq	%rax, %rax
14229ce2c:     	je	0x1422a28bd <.text+0x22a18bd>
14229ce32:     	movq	%rdi, 0x6(%rax)
14229ce36:     	movq	%rax, 0x148(%rbp)
14229ce3d:     	movq	%rsi, (%rax)
14229ce40:     	movzbl	0x1397dc2(%rip), %eax   # 0x143634c09
14229ce47:     	movl	$0xe, %ecx
14229ce4c:     	movl	$0x1, %edx
14229ce51:     	callq	0x140613c10 <.text+0x612c10>
14229ce56:     	testq	%rax, %rax
14229ce59:     	je	0x1422a288d <.text+0x22a188d>
14229ce5f:     	movq	%rdi, 0x6(%rax)
14229ce63:     	movq	%rax, 0x100(%rbp)
14229ce6a:     	movq	%rsi, (%rax)
14229ce6d:     	movzbl	0x1397d95(%rip), %eax   # 0x143634c09
14229ce74:     	movl	$0xe, %ecx
14229ce79:     	movl	$0x1, %edx
14229ce7e:     	callq	0x140613c10 <.text+0x612c10>
14229ce83:     	testq	%rax, %rax
14229ce86:     	je	0x1422a285d <.text+0x22a185d>
14229ce8c:     	movq	%rdi, 0x6(%rax)
14229ce90:     	movq	%rax, 0xd0(%rbp)
14229ce97:     	movq	%rsi, (%rax)
14229ce9a:     	movzbl	0x1397d68(%rip), %eax   # 0x143634c09
14229cea1:     	movl	$0xe, %ecx
14229cea6:     	movl	$0x1, %edx
14229ceab:     	callq	0x140613c10 <.text+0x612c10>
14229ceb0:     	testq	%rax, %rax
14229ceb3:     	je	0x1422a272f <.text+0x22a172f>
14229ceb9:     	movq	%r12, 0xa8(%rbp)
14229cec0:     	movq	%rdi, 0x6(%rax)
14229cec4:     	movq	%rax, 0xc8(%rbp)
14229cecb:     	movq	%rsi, (%rax)
14229cece:     	movzbl	0x1397d34(%rip), %eax   # 0x143634c09
14229ced5:     	movl	$0xe, %ecx
14229ceda:     	movl	$0x1, %edx
14229cedf:     	callq	0x140613c10 <.text+0x612c10>
14229cee4:     	testq	%rax, %rax
14229cee7:     	je	0x1422a26ff <.text+0x22a16ff>
14229ceed:     	movq	%rdi, 0x6(%rax)
14229cef1:     	movq	%rsi, (%rax)
14229cef4:     	movq	%rax, %r12
14229cef7:     	movzbl	0x1397d0b(%rip), %eax   # 0x143634c09
14229cefe:     	movl	$0xe, %ecx
14229cf03:     	movl	$0x1, %edx
14229cf08:     	callq	0x140613c10 <.text+0x612c10>
14229cf0d:     	testq	%rax, %rax
14229cf10:     	je	0x1422a28ed <.text+0x22a18ed>
14229cf16:     	movq	$0x17, (%rbx)
14229cf1d:     	movq	%r14, 0x8(%rbx)
14229cf21:     	movq	$0x17, 0x10(%rbx)
14229cf29:     	movb	$0x3, 0x18(%rbx)
14229cf2d:     	movl	$0x0, 0x1c(%rbx)
14229cf34:     	movl	$0x0, 0x24(%rbx)
14229cf3b:     	movq	$0x18, 0x28(%rbx)
14229cf43:     	movq	%r13, 0x30(%rbx)
14229cf47:     	movq	$0x18, 0x38(%rbx)
14229cf4f:     	movb	$0x3, 0x40(%rbx)
14229cf53:     	movl	$0x1, 0x44(%rbx)
14229cf5a:     	movl	$0x0, 0x4c(%rbx)
14229cf61:     	movq	$0x17, 0x50(%rbx)
14229cf69:     	movq	0x1f0(%rbp), %rcx
14229cf70:     	movq	%rcx, 0x58(%rbx)
14229cf74:     	movq	$0x17, 0x60(%rbx)
14229cf7c:     	movb	$0x3, 0x68(%rbx)
14229cf80:     	movl	$0x3, 0x6c(%rbx)
14229cf87:     	movl	$0x0, 0x74(%rbx)
14229cf8e:     	movq	$0x16, 0x78(%rbx)
14229cf96:     	movq	0x1e8(%rbp), %rcx
14229cf9d:     	movq	%rcx, 0x80(%rbx)
14229cfa4:     	movq	$0x16, 0x88(%rbx)
14229cfaf:     	movb	$0x3, 0x90(%rbx)
14229cfb6:     	movl	$0xb, 0x94(%rbx)
14229cfc0:     	movl	$0x0, 0x9c(%rbx)
14229cfca:     	movq	$0x16, 0xa0(%rbx)
14229cfd5:     	movq	0x1d8(%rbp), %rcx
14229cfdc:     	movq	%rcx, 0xa8(%rbx)
14229cfe3:     	movq	$0x16, 0xb0(%rbx)
14229cfee:     	movb	$0x3, 0xb8(%rbx)
14229cff5:     	movl	$0x2, 0xbc(%rbx)
14229cfff:     	movl	$0x0, 0xc4(%rbx)
14229d009:     	movq	$0xe, 0xc8(%rbx)
14229d014:     	movq	0x1c0(%rbp), %rcx
14229d01b:     	movq	%rcx, 0xd0(%rbx)
14229d022:     	movq	$0xe, 0xd8(%rbx)
14229d02d:     	movb	$0x3, 0xe0(%rbx)
14229d034:     	movl	$0x4, 0xe4(%rbx)
14229d03e:     	movl	$0x0, 0xec(%rbx)
14229d048:     	movq	$0xe, 0xf0(%rbx)
14229d053:     	movq	0x150(%rbp), %rcx
14229d05a:     	movq	%rcx, 0xf8(%rbx)
14229d061:     	movq	$0xe, 0x100(%rbx)
14229d06c:     	movb	$0x3, 0x108(%rbx)
14229d073:     	movl	$0xc, 0x10c(%rbx)
14229d07d:     	movl	$0x7, 0x114(%rbx)
14229d087:     	movq	$0xe, 0x118(%rbx)
14229d092:     	movq	0x148(%rbp), %rcx
14229d099:     	movq	%rcx, 0x120(%rbx)
14229d0a0:     	movq	$0xe, 0x128(%rbx)
14229d0ab:     	movb	$0x3, 0x130(%rbx)
14229d0b2:     	movl	$0x5, 0x134(%rbx)
14229d0bc:     	movl	$0x1, 0x13c(%rbx)
14229d0c6:     	movq	$0xe, 0x140(%rbx)
14229d0d1:     	movq	0x100(%rbp), %rcx
14229d0d8:     	movq	%rcx, 0x148(%rbx)
14229d0df:     	movq	$0xe, 0x150(%rbx)
14229d0ea:     	movb	$0x3, 0x158(%rbx)
14229d0f1:     	movl	$0xa, 0x15c(%rbx)
14229d0fb:     	movl	$0x6, 0x164(%rbx)
14229d105:     	movq	$0xe, 0x168(%rbx)
14229d110:     	movq	0xd0(%rbp), %rcx
14229d117:     	movq	%rcx, 0x170(%rbx)
14229d11e:     	movq	$0xe, 0x178(%rbx)
14229d129:     	movb	$0x3, 0x180(%rbx)
14229d130:     	movl	$0x8, 0x184(%rbx)
14229d13a:     	movl	$0x4, 0x18c(%rbx)
14229d144:     	movq	$0xe, 0x190(%rbx)
14229d14f:     	movq	0xc8(%rbp), %rcx
14229d156:     	movq	%rcx, 0x198(%rbx)
14229d15d:     	movq	$0xe, 0x1a0(%rbx)
14229d168:     	movb	$0x3, 0x1a8(%rbx)
14229d16f:     	movl	$0x9, 0x1ac(%rbx)
14229d179:     	movl	$0x5, 0x1b4(%rbx)
14229d183:     	movq	$0xe, 0x1b8(%rbx)
14229d18e:     	movq	%r12, 0x1c0(%rbx)
14229d195:     	movq	%rdi, 0x6(%rax)
14229d199:     	movq	%rsi, (%rax)
14229d19c:     	movq	$0xe, 0x1c8(%rbx)
14229d1a7:     	movb	$0x3, 0x1d0(%rbx)
14229d1ae:     	movl	$0x6, 0x1d4(%rbx)
14229d1b8:     	movl	$0x2, 0x1dc(%rbx)
14229d1c2:     	movq	$0xe, 0x1e0(%rbx)
14229d1cd:     	movq	%rax, 0x1e8(%rbx)
14229d1d4:     	movq	$0xe, 0x1f0(%rbx)
14229d1df:     	movb	$0x3, 0x1f8(%rbx)
14229d1e6:     	movl	$0x7, 0x1fc(%rbx)
14229d1f0:     	movl	$0x3, 0x204(%rbx)
14229d1fa:     	movq	$0xd, 0x158(%rbp)
14229d205:     	movq	%rbx, 0x160(%rbp)
14229d20c:     	movq	$0xd, 0x168(%rbp)
14229d217:     	leaq	0x40(%rbp), %rcx
14229d21b:     	callq	0x1400885d0 <.text+0x875d0>
14229d220:     	movq	0x178(%rbp), %rcx
14229d227:     	movq	0x10(%rcx), %rdx
14229d22b:     	cmpq	%rdx, %rax
14229d22e:     	movq	0xa8(%rbp), %r14
14229d235:     	jae	0x1422a278f <.text+0x22a178f>
14229d23b:     	movq	0x8(%rcx), %rcx
14229d23f:     	movl	(%rcx,%rax,4), %eax
14229d242:     	leal	-0x4(%rax), %ecx
14229d245:     	cmpl	$0x7, %ecx
14229d248:     	jae	0x1422a0b3f <.text+0x229fb3f>
14229d24e:     	movzbl	0x13979b4(%rip), %eax   # 0x143634c09
14229d255:     	movl	$0xa0, %ecx
14229d25a:     	movl	$0x8, %edx
14229d25f:     	callq	0x140613c10 <.text+0x612c10>
14229d264:     	testq	%rax, %rax
14229d267:     	je	0x1422a2a06 <.text+0x22a1a06>
14229d26d:     	movq	%rax, %rsi
14229d270:     	movzbl	0x1397992(%rip), %eax   # 0x143634c09
14229d277:     	movl	$0x5, %ecx
14229d27c:     	movl	$0x1, %edx
14229d281:     	callq	0x140613c10 <.text+0x612c10>
14229d286:     	testq	%rax, %rax
14229d289:     	je	0x1422a29e4 <.text+0x22a19e4>
14229d28f:     	movq	%rax, %rdi
14229d292:     	movzbl	0xe99abe(%rip), %eax    # 0x143136d57
14229d299:     	movb	%al, 0x4(%rdi)
14229d29c:     	movl	0xe99ab1(%rip), %eax    # 0x143136d53
14229d2a2:     	movl	%eax, (%rdi)
14229d2a4:     	movzbl	0x139795e(%rip), %eax   # 0x143634c09
14229d2ab:     	movl	$0x1b, %ecx
14229d2b0:     	movl	$0x1, %edx
14229d2b5:     	callq	0x140613c10 <.text+0x612c10>
14229d2ba:     	testq	%rax, %rax
14229d2bd:     	je	0x1422a298b <.text+0x22a198b>
14229d2c3:     	movq	%rax, %rbx
14229d2c6:     	movups	0xd9ef26(%rip), %xmm0   # 0x14303c1f3
14229d2cd:     	movups	%xmm0, 0xb(%rax)
14229d2d1:     	movups	0xd9ef10(%rip), %xmm0   # 0x14303c1e8
14229d2d8:     	movups	%xmm0, (%rax)
14229d2db:     	movzbl	0x1397927(%rip), %eax   # 0x143634c09
14229d2e2:     	movl	$0x1e, %ecx
14229d2e7:     	movl	$0x1, %edx
14229d2ec:     	callq	0x140613c10 <.text+0x612c10>
14229d2f1:     	testq	%rax, %rax
14229d2f4:     	je	0x1422a29b4 <.text+0x22a19b4>
14229d2fa:     	movq	%rax, %r12
14229d2fd:     	movups	0xd9ef0d(%rip), %xmm0   # 0x14303c211
14229d304:     	movups	%xmm0, 0xe(%rax)
14229d308:     	movups	0xd9eef4(%rip), %xmm0   # 0x14303c203
14229d30f:     	movups	%xmm0, (%rax)
14229d312:     	movzbl	0x13978f0(%rip), %eax   # 0x143634c09
14229d319:     	movl	$0x1c, %ecx
14229d31e:     	movl	$0x1, %edx
14229d323:     	callq	0x140613c10 <.text+0x612c10>
14229d328:     	testq	%rax, %rax
14229d32b:     	je	0x1422a2954 <.text+0x22a1954>
14229d331:     	movups	0xd9eef5(%rip), %xmm0   # 0x14303c22d
14229d338:     	movups	%xmm0, 0xc(%rax)
14229d33c:     	movups	0xd9eede(%rip), %xmm0   # 0x14303c221
14229d343:     	movups	%xmm0, (%rax)
14229d346:     	movq	$0x5, (%rsi)
14229d34d:     	movq	%rdi, 0x8(%rsi)
14229d351:     	movq	$0x5, 0x10(%rsi)
14229d359:     	movb	$0x5, 0x18(%rsi)
14229d35d:     	movl	$0x0, 0x24(%rsi)
14229d364:     	movq	$0x1b, 0x28(%rsi)
14229d36c:     	movq	%rbx, 0x30(%rsi)
14229d370:     	movq	$0x1b, 0x38(%rsi)
14229d378:     	movb	$0xc, 0x40(%rsi)
14229d37c:     	movl	$0x0, 0x44(%rsi)
14229d383:     	movl	$0x0, 0x4c(%rsi)
14229d38a:     	movq	$0x1e, 0x50(%rsi)
14229d392:     	movq	%r12, 0x58(%rsi)
14229d396:     	movq	$0x1e, 0x60(%rsi)
14229d39e:     	movb	$0xc, 0x68(%rsi)
14229d3a2:     	movl	$0x3f000000, 0x6c(%rsi) # imm = 0x3F000000
14229d3a9:     	movl	$0x0, 0x74(%rsi)
14229d3b0:     	movq	$0x1c, 0x78(%rsi)
14229d3b8:     	movq	%rax, 0x80(%rsi)
14229d3bf:     	movq	$0x1c, 0x88(%rsi)
14229d3ca:     	movb	$0xc, 0x90(%rsi)
14229d3d1:     	movl	$0x3f800000, 0x94(%rsi) # imm = 0x3F800000
14229d3db:     	movl	$0x0, 0x9c(%rsi)
14229d3e5:     	movq	%rsi, %rax
14229d3e8:     	addq	$0xa0, %rax
14229d3ee:     	movq	%rsi, 0x180(%rbp)
14229d3f5:     	movq	$0x4, 0x190(%rbp)
14229d400:     	movq	%rsi, 0x188(%rbp)
14229d407:     	movq	%rax, 0x198(%rbp)
14229d40e:     	movq	0x158(%rbp), %rax
14229d415:     	movq	0x168(%rbp), %rdi
14229d41c:     	subq	%rdi, %rax
14229d41f:     	cmpq	$0x4, %rax
14229d423:     	jb	0x1422a0d27 <.text+0x229fd27>
14229d429:     	leaq	(%rdi,%rdi,4), %rcx
14229d42d:     	shlq	$0x3, %rcx
14229d431:     	addq	0x160(%rbp), %rcx
14229d438:     	movl	$0xa0, %r8d
14229d43e:     	movq	%rsi, %rdx
14229d441:     	callq	0x142923c00 <.text+0x2922c00>
14229d446:     	addq	$0x4, %rdi
14229d44a:     	movq	%rdi, 0x168(%rbp)
14229d451:     	movl	$0xa0, %edx
14229d456:     	movl	$0x8, %r8d
14229d45c:     	movq	%rsi, %rcx
14229d45f:     	callq	0x140613c20 <.text+0x612c20>
14229d464:     	cmpl	$0x2, 0x258(%r14)
14229d46c:     	movzbl	0x1397796(%rip), %eax   # 0x143634c09
14229d473:     	jae	0x14229e525 <.text+0x229d525>
14229d479:     	movl	$0xa0, %ecx
14229d47e:     	movl	$0x8, %edx
14229d483:     	callq	0x140613c10 <.text+0x612c10>
14229d488:     	testq	%rax, %rax
14229d48b:     	je	0x1422a2b91 <.text+0x22a1b91>
14229d491:     	movq	%rax, %rsi
14229d494:     	movzbl	0x139776e(%rip), %eax   # 0x143634c09
14229d49b:     	movl	$0x5, %ecx
14229d4a0:     	movl	$0x1, %edx
14229d4a5:     	callq	0x140613c10 <.text+0x612c10>
14229d4aa:     	testq	%rax, %rax
14229d4ad:     	je	0x1422a2b6f <.text+0x22a1b6f>
14229d4b3:     	movq	%rax, %rdi
14229d4b6:     	movzbl	0xe9989a(%rip), %eax    # 0x143136d57
14229d4bd:     	movb	%al, 0x4(%rdi)
14229d4c0:     	movl	0xe9988d(%rip), %eax    # 0x143136d53
14229d4c6:     	movl	%eax, (%rdi)
14229d4c8:     	movzbl	0x139773a(%rip), %eax   # 0x143634c09
14229d4cf:     	movl	$0x15, %ecx
14229d4d4:     	movl	$0x1, %edx
14229d4d9:     	callq	0x140613c10 <.text+0x612c10>
14229d4de:     	testq	%rax, %rax
14229d4e1:     	je	0x1422a2b46 <.text+0x22a1b46>
14229d4e7:     	movq	%rax, %rbx
14229d4ea:     	movups	0xd9ed6f(%rip), %xmm0   # 0x14303c260
14229d4f1:     	movups	%xmm0, (%rax)
14229d4f4:     	movabsq	$0x7265626d6974666c, %rax # imm = 0x7265626D6974666C
14229d4fe:     	movq	%rax, 0xd(%rbx)
14229d502:     	movzbl	0x1397700(%rip), %eax   # 0x143634c09
14229d509:     	movl	$0x10, %ecx
14229d50e:     	movl	$0x1, %edx
14229d513:     	callq	0x140613c10 <.text+0x612c10>
14229d518:     	testq	%rax, %rax
14229d51b:     	je	0x1422a2bf7 <.text+0x22a1bf7>
14229d521:     	movq	%rax, %r14
14229d524:     	movups	0xd9ed12(%rip), %xmm0   # 0x14303c23d
14229d52b:     	movups	%xmm0, (%rax)
14229d52e:     	movzbl	0x13976d4(%rip), %eax   # 0x143634c09
14229d535:     	movl	$0x13, %ecx
14229d53a:     	movl	$0x1, %edx
14229d53f:     	callq	0x140613c10 <.text+0x612c10>
14229d544:     	testq	%rax, %rax
14229d547:     	je	0x1422a2ba5 <.text+0x22a1ba5>
14229d54d:     	movups	0xd9ecf9(%rip), %xmm0   # 0x14303c24d
14229d554:     	movups	%xmm0, (%rax)
14229d557:     	movl	$0x7472696b, 0xf(%rax)  # imm = 0x7472696B
14229d55e:     	movq	$0x5, (%rsi)
14229d565:     	movq	%rdi, 0x8(%rsi)
14229d569:     	movq	$0x5, 0x10(%rsi)
14229d571:     	movb	$0x5, 0x18(%rsi)
14229d575:     	movl	$0x0, 0x24(%rsi)
14229d57c:     	movq	$0x15, 0x28(%rsi)
14229d584:     	movq	%rbx, 0x30(%rsi)
14229d588:     	movq	$0x15, 0x38(%rsi)
14229d590:     	movw	$0x204, 0x40(%rsi)      # imm = 0x204
14229d596:     	movl	$0x0, 0x4c(%rsi)
14229d59d:     	movq	$0x10, 0x50(%rsi)
14229d5a5:     	movq	%r14, 0x58(%rsi)
14229d5a9:     	movq	$0x10, 0x60(%rsi)
14229d5b1:     	movw	$0x104, 0x68(%rsi)      # imm = 0x104
14229d5b7:     	movl	$0x0, 0x74(%rsi)
14229d5be:     	movq	$0x13, 0x78(%rsi)
14229d5c6:     	movq	%rax, 0x80(%rsi)
14229d5cd:     	movq	$0x13, 0x88(%rsi)
14229d5d8:     	movw	$0x4, 0x90(%rsi)
14229d5e1:     	movl	$0x0, 0x9c(%rsi)
14229d5eb:     	movq	%rsi, %rax
14229d5ee:     	addq	$0xa0, %rax
14229d5f4:     	movq	%rsi, 0x180(%rbp)
14229d5fb:     	movq	$0x4, 0x190(%rbp)
14229d606:     	movq	%rsi, 0x188(%rbp)
14229d60d:     	movq	%rax, 0x198(%rbp)
14229d614:     	movq	0x158(%rbp), %rax
14229d61b:     	movq	0x168(%rbp), %rdi
14229d622:     	subq	%rdi, %rax
14229d625:     	cmpq	$0x4, %rax
14229d629:     	jb	0x1422a0d57 <.text+0x229fd57>
14229d62f:     	leaq	(%rdi,%rdi,4), %rcx
14229d633:     	shlq	$0x3, %rcx
14229d637:     	addq	0x160(%rbp), %rcx
14229d63e:     	movl	$0xa0, %r8d
14229d644:     	movq	%rsi, %rdx
14229d647:     	callq	0x142923c00 <.text+0x2922c00>
14229d64c:     	addq	$0x4, %rdi
14229d650:     	movq	%rdi, 0x168(%rbp)
14229d657:     	movl	$0xa0, %edx
14229d65c:     	movl	$0x8, %r8d
14229d662:     	movq	%rsi, %rcx
14229d665:     	callq	0x140613c20 <.text+0x612c20>
14229d66a:     	movq	0x40(%rbp), %rdx
14229d66e:     	movq	0x1f8(%rbp), %rcx
14229d675:     	callq	0x140a922b0 <.text+0xa912b0>
14229d67a:     	movq	%rax, %rcx
14229d67d:     	shrq	$0x20, %rcx
14229d681:     	je	0x14229e9c1 <.text+0x229d9c1>
14229d687:     	movl	%eax, %eax
14229d689:     	movq	0x128(%rbp), %rdx
14229d690:     	cmpq	0x10(%rdx), %rax
14229d694:     	jae	0x14229e9c1 <.text+0x229d9c1>
14229d69a:     	movq	0x128(%rbp), %rdx
14229d6a1:     	movq	0x8(%rdx), %rdx
14229d6a5:     	leaq	(%rax,%rax,4), %rax
14229d6a9:     	cmpl	%ecx, (%rdx,%rax,4)
14229d6ac:     	jne	0x14229e9c1 <.text+0x229d9c1>
14229d6b2:     	leaq	(%rdx,%rax,4), %rax
14229d6b6:     	movl	0x4(%rax), %ecx
14229d6b9:     	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
14229d6be:     	cmpq	%rdx, %rcx
14229d6c1:     	je	0x14229e9c1 <.text+0x229d9c1>
14229d6c7:     	movq	0x108(%rbp), %rdx
14229d6ce:     	cmpq	%rcx, 0x38(%rdx)
14229d6d2:     	jbe	0x14229e9c1 <.text+0x229d9c1>
14229d6d8:     	movq	0x108(%rbp), %rdx
14229d6df:     	movq	0x28(%rdx), %rdx
14229d6e3:     	movl	%ecx, %r8d
14229d6e6:     	shrl	$0x6, %r8d
14229d6ea:     	movq	(%rdx,%r8,8), %rdx
14229d6ee:     	btq	%rcx, %rdx
14229d6f2:     	jae	0x14229e9c1 <.text+0x229d9c1>
14229d6f8:     	movl	0xc(%rax), %ecx
14229d6fb:     	movq	0x128(%rbp), %rdx
14229d702:     	movq	0x1a8(%rdx), %rdx
14229d709:     	leaq	(%rcx,%rcx,8), %r8
14229d70d:     	movq	0x108(%rbp), %rcx
14229d714:     	movq	0x110(%rcx), %rcx
14229d71b:     	cmpq	%rcx, 0x40(%rdx,%r8,8)
14229d720:     	jbe	0x1422a0ba1 <.text+0x229fba1>
14229d726:     	leaq	(%rdx,%r8,8), %rdx
14229d72a:     	movq	0x38(%rdx), %r8
14229d72e:     	movq	(%r8,%rcx,8), %rcx
14229d732:     	testq	%rcx, %rcx
14229d735:     	je	0x1422a0ba1 <.text+0x229fba1>
14229d73b:     	movq	0x18(%rdx), %rdx
14229d73f:     	notq	%rcx
14229d742:     	leaq	(%rcx,%rcx,2), %rcx
14229d746:     	shlq	$0x4, %rcx
14229d74a:     	movq	0x10(%rdx,%rcx), %rcx
14229d74f:     	jmp	0x1422a0ba3 <.text+0x229fba3>
14229d754:     	movl	$0x1b8, %ecx            # imm = 0x1B8
14229d759:     	movl	$0x8, %edx
14229d75e:     	callq	0x140613c10 <.text+0x612c10>
14229d763:     	testq	%rax, %rax
14229d766:     	je	0x1422a2316 <.text+0x22a1316>
14229d76c:     	movq	%rax, %rsi
14229d76f:     	movzbl	0x1397493(%rip), %eax   # 0x143634c09
14229d776:     	movl	$0x15, %ecx
14229d77b:     	movl	$0x1, %edx
14229d780:     	callq	0x140613c10 <.text+0x612c10>
14229d785:     	testq	%rax, %rax
14229d788:     	je	0x1422a2325 <.text+0x22a1325>
14229d78e:     	movq	%rax, %rdi
14229d791:     	movups	0xd9eb09(%rip), %xmm0   # 0x14303c2a1
14229d798:     	movups	%xmm0, (%rax)
14229d79b:     	movabsq	$0x657268636f5f666f, %rax # imm = 0x657268636F5F666F
14229d7a5:     	movq	%rax, 0xd(%rdi)
14229d7a9:     	movzbl	0x1397459(%rip), %eax   # 0x143634c09
14229d7b0:     	movl	$0x13, %ecx
14229d7b5:     	movl	$0x1, %edx
14229d7ba:     	callq	0x140613c10 <.text+0x612c10>
14229d7bf:     	testq	%rax, %rax
14229d7c2:     	je	0x1422a24e0 <.text+0x22a14e0>
14229d7c8:     	movq	%rax, %rbx
14229d7cb:     	movups	0xd9eae4(%rip), %xmm0   # 0x14303c2b6
14229d7d2:     	movups	%xmm0, (%rax)
14229d7d5:     	movl	$0x6465725f, 0xf(%rax)  # imm = 0x6465725F
14229d7dc:     	movzbl	0x1397426(%rip), %eax   # 0x143634c09
14229d7e3:     	movl	$0x16, %ecx
14229d7e8:     	movl	$0x1, %edx
14229d7ed:     	callq	0x140613c10 <.text+0x612c10>
14229d7f2:     	testq	%rax, %rax
14229d7f5:     	je	0x1422a22a1 <.text+0x22a12a1>
14229d7fb:     	movq	%rax, %r14
14229d7fe:     	movups	0xd9eac4(%rip), %xmm0   # 0x14303c2c9
14229d805:     	movups	%xmm0, (%rax)
14229d808:     	movabsq	$0x656c707275705f66, %rax # imm = 0x656C707275705F66
14229d812:     	movq	%rax, 0xe(%r14)
14229d816:     	movzbl	0x13973ec(%rip), %eax   # 0x143634c09
14229d81d:     	movl	$0x14, %ecx
14229d822:     	movl	$0x1, %edx
14229d827:     	callq	0x140613c10 <.text+0x612c10>
14229d82c:     	testq	%rax, %rax
14229d82f:     	je	0x1422a1ec8 <.text+0x22a0ec8>
14229d835:     	movq	%rax, %r12
14229d838:     	movups	0xd9eaa0(%rip), %xmm0   # 0x14303c2df
14229d83f:     	movups	%xmm0, (%rax)
14229d842:     	movl	$0x65756c62, 0x10(%rax) # imm = 0x65756C62
14229d849:     	movzbl	0x13973b9(%rip), %eax   # 0x143634c09
14229d850:     	movl	$0x1a, %ecx
14229d855:     	movl	$0x1, %edx
14229d85a:     	callq	0x140613c10 <.text+0x612c10>
14229d85f:     	testq	%rax, %rax
14229d862:     	je	0x1422a1cd1 <.text+0x22a0cd1>
14229d868:     	movq	%rax, %r13
14229d86b:     	movups	0xd9ea8b(%rip), %xmm0   # 0x14303c2fd
14229d872:     	movups	%xmm0, 0xa(%rax)
14229d876:     	movups	0xd9ea76(%rip), %xmm0   # 0x14303c2f3
14229d87d:     	movups	%xmm0, (%rax)
14229d880:     	movzbl	0x1397382(%rip), %eax   # 0x143634c09
14229d887:     	movl	$0x19, %ecx
14229d88c:     	movl	$0x1, %edx
14229d891:     	callq	0x140613c10 <.text+0x612c10>
14229d896:     	testq	%rax, %rax
14229d899:     	je	0x1422a2615 <.text+0x22a1615>
14229d89f:     	movups	0xd9ea70(%rip), %xmm0   # 0x14303c316
14229d8a6:     	movups	%xmm0, 0x9(%rax)
14229d8aa:     	movups	0xd9ea5c(%rip), %xmm0   # 0x14303c30d
14229d8b1:     	movq	%rax, 0x1f0(%rbp)
14229d8b8:     	movups	%xmm0, (%rax)
14229d8bb:     	movzbl	0x1397347(%rip), %eax   # 0x143634c09
14229d8c2:     	movl	$0x1b, %ecx
14229d8c7:     	movl	$0x1, %edx
14229d8cc:     	callq	0x140613c10 <.text+0x612c10>
14229d8d1:     	testq	%rax, %rax
14229d8d4:     	je	0x1422a2532 <.text+0x22a1532>
14229d8da:     	movups	0xd9ea50(%rip), %xmm0   # 0x14303c331
14229d8e1:     	movups	%xmm0, 0xb(%rax)
14229d8e5:     	movups	0xd9ea3a(%rip), %xmm0   # 0x14303c326
14229d8ec:     	movq	%rax, 0x1e8(%rbp)
14229d8f3:     	movups	%xmm0, (%rax)
14229d8f6:     	movzbl	0x139730c(%rip), %eax   # 0x143634c09
14229d8fd:     	movl	$0x15, %ecx
14229d902:     	movl	$0x1, %edx
14229d907:     	callq	0x140613c10 <.text+0x612c10>
14229d90c:     	testq	%rax, %rax
14229d90f:     	je	0x1422a2418 <.text+0x22a1418>
14229d915:     	movq	%rax, %rcx
14229d918:     	movups	0xd9ea22(%rip), %xmm0   # 0x14303c341
14229d91f:     	movups	%xmm0, (%rax)
14229d922:     	movabsq	$0x6e656572675f666f, %rax # imm = 0x6E656572675F666F
14229d92c:     	movq	%rcx, 0x1d8(%rbp)
14229d933:     	movq	%rax, 0xd(%rcx)
14229d937:     	movzbl	0x13972cb(%rip), %eax   # 0x143634c09
14229d93e:     	movl	$0x19, %ecx
14229d943:     	movl	$0x1, %edx
14229d948:     	callq	0x140613c10 <.text+0x612c10>
14229d94d:     	testq	%rax, %rax
14229d950:     	je	0x1422a225c <.text+0x22a125c>
14229d956:     	movups	0xd9ea02(%rip), %xmm0   # 0x14303c35f
14229d95d:     	movups	%xmm0, 0x9(%rax)
14229d961:     	movups	0xd9e9ee(%rip), %xmm0   # 0x14303c356
14229d968:     	movq	%rax, 0x1c0(%rbp)
14229d96f:     	movups	%xmm0, (%rax)
14229d972:     	movzbl	0x1397290(%rip), %eax   # 0x143634c09
14229d979:     	movl	$0x14, %ecx
14229d97e:     	movl	$0x1, %edx
14229d983:     	callq	0x140613c10 <.text+0x612c10>
14229d988:     	testq	%rax, %rax
14229d98b:     	je	0x1422a1e3e <.text+0x22a0e3e>
14229d991:     	movups	0xd9e9d7(%rip), %xmm0   # 0x14303c36f
14229d998:     	movups	%xmm0, (%rax)
14229d99b:     	movl	$0x6b726164, 0x10(%rax) # imm = 0x6B726164
14229d9a2:     	movq	%rax, 0x150(%rbp)
14229d9a9:     	movzbl	0x1397259(%rip), %eax   # 0x143634c09
14229d9b0:     	movl	$0x14, %ecx
14229d9b5:     	movl	$0x1, %edx
14229d9ba:     	callq	0x140613c10 <.text+0x612c10>
14229d9bf:     	testq	%rax, %rax
14229d9c2:     	je	0x1422a1d38 <.text+0x22a0d38>
14229d9c8:     	movups	0xd9e9b4(%rip), %xmm0   # 0x14303c383
14229d9cf:     	movups	%xmm0, (%rax)
14229d9d2:     	movl	$0x79657267, 0x10(%rax) # imm = 0x79657267
14229d9d9:     	movq	$0x15, (%rsi)
14229d9e0:     	movq	%rdi, 0x8(%rsi)
14229d9e4:     	movq	$0x15, 0x10(%rsi)
14229d9ec:     	movb	$0x3, 0x18(%rsi)
14229d9f0:     	movl	$0x8, 0x1c(%rsi)
14229d9f7:     	movl	$0x0, 0x24(%rsi)
14229d9fe:     	movq	$0x13, 0x28(%rsi)
14229da06:     	movq	%rbx, 0x30(%rsi)
14229da0a:     	movq	$0x13, 0x38(%rsi)
14229da12:     	movb	$0x3, 0x40(%rsi)
14229da16:     	movl	$0x0, 0x44(%rsi)
14229da1d:     	movl	$0x0, 0x4c(%rsi)
14229da24:     	movq	$0x16, 0x50(%rsi)
14229da2c:     	movq	%r14, 0x58(%rsi)
14229da30:     	movq	$0x16, 0x60(%rsi)
14229da38:     	movb	$0x3, 0x68(%rsi)
14229da3c:     	movl	$0xa, 0x6c(%rsi)
14229da43:     	movl	$0x0, 0x74(%rsi)
14229da4a:     	movq	$0x14, 0x78(%rsi)
14229da52:     	movq	%r12, 0x80(%rsi)
14229da59:     	movq	$0x14, 0x88(%rsi)
14229da64:     	movb	$0x3, 0x90(%rsi)
14229da6b:     	movl	$0x2, 0x94(%rsi)
14229da75:     	movl	$0x0, 0x9c(%rsi)
14229da7f:     	movq	$0x1a, 0xa0(%rsi)
14229da8a:     	movq	%r13, 0xa8(%rsi)
14229da91:     	movq	$0x1a, 0xb0(%rsi)
14229da9c:     	movb	$0x3, 0xb8(%rsi)
14229daa3:     	movl	$0x9, 0xbc(%rsi)
14229daad:     	movl	$0x0, 0xc4(%rsi)
14229dab7:     	movq	$0x19, 0xc8(%rsi)
14229dac2:     	movq	0x1f0(%rbp), %rcx
14229dac9:     	movq	%rcx, 0xd0(%rsi)
14229dad0:     	movq	$0x19, 0xd8(%rsi)
14229dadb:     	movb	$0x3, 0xe0(%rsi)
14229dae2:     	movl	$0x3, 0xe4(%rsi)
14229daec:     	movl	$0x0, 0xec(%rsi)
14229daf6:     	movq	$0x1b, 0xf0(%rsi)
14229db01:     	movq	0x1e8(%rbp), %rcx
14229db08:     	movq	%rcx, 0xf8(%rsi)
14229db0f:     	movq	$0x1b, 0x100(%rsi)
14229db1a:     	movb	$0x3, 0x108(%rsi)
14229db21:     	movl	$0x6, 0x10c(%rsi)
14229db2b:     	movl	$0x0, 0x114(%rsi)
14229db35:     	movq	$0x15, 0x118(%rsi)
14229db40:     	movq	0x1d8(%rbp), %rcx
14229db47:     	movq	%rcx, 0x120(%rsi)
14229db4e:     	movq	$0x15, 0x128(%rsi)
14229db59:     	movb	$0x3, 0x130(%rsi)
14229db60:     	movl	$0x4, 0x134(%rsi)
14229db6a:     	movl	$0x0, 0x13c(%rsi)
14229db74:     	movq	$0x19, 0x140(%rsi)
14229db7f:     	movq	0x1c0(%rbp), %rcx
14229db86:     	movq	%rcx, 0x148(%rsi)
14229db8d:     	movq	$0x19, 0x150(%rsi)
14229db98:     	movb	$0x3, 0x158(%rsi)
14229db9f:     	movl	$0x1, 0x15c(%rsi)
14229dba9:     	movl	$0x0, 0x164(%rsi)
14229dbb3:     	movq	$0x14, 0x168(%rsi)
14229dbbe:     	movq	0x150(%rbp), %rcx
14229dbc5:     	movq	%rcx, 0x170(%rsi)
14229dbcc:     	movq	$0x14, 0x178(%rsi)
14229dbd7:     	movb	$0x3, 0x180(%rsi)
14229dbde:     	movl	$0x5, 0x184(%rsi)
14229dbe8:     	movl	$0x0, 0x18c(%rsi)
14229dbf2:     	movq	$0x14, 0x190(%rsi)
14229dbfd:     	movq	%rax, 0x198(%rsi)
14229dc04:     	movq	$0x14, 0x1a0(%rsi)
14229dc0f:     	movb	$0x3, 0x1a8(%rsi)
14229dc16:     	movl	$0x7, 0x1ac(%rsi)
14229dc20:     	movl	$0x0, 0x1b4(%rsi)
14229dc2a:     	movq	$0xb, 0x110(%rbp)
14229dc35:     	movq	%rsi, 0x118(%rbp)
14229dc3c:     	movq	$0xb, 0x120(%rbp)
14229dc47:     	jmp	0x14229e9e0 <.text+0x229d9e0>
14229dc4c:     	movb	$0x1, %sil
14229dc4f:     	movzbl	0x1396fb3(%rip), %eax   # 0x143634c09
14229dc56:     	cmpb	$0x14, %bl
14229dc59:     	je	0x14229aee1 <.text+0x2299ee1>
14229dc5f:     	movl	$0x118, %ecx            # imm = 0x118
14229dc64:     	movl	$0x8, %edx
14229dc69:     	callq	0x140613c10 <.text+0x612c10>
14229dc6e:     	testq	%rax, %rax
14229dc71:     	je	0x1422a2347 <.text+0x22a1347>
14229dc77:     	movq	%rax, %rdi
14229dc7a:     	movzbl	0x1396f88(%rip), %eax   # 0x143634c09
14229dc81:     	movl	$0x17, %ecx
14229dc86:     	movl	$0x1, %edx
14229dc8b:     	callq	0x140613c10 <.text+0x612c10>
14229dc90:     	testq	%rax, %rax
14229dc93:     	je	0x1422a21be <.text+0x22a11be>
14229dc99:     	movq	%rax, %rbx
14229dc9c:     	movups	0xd9e4b5(%rip), %xmm0   # 0x14303c158
14229dca3:     	movups	%xmm0, (%rax)
14229dca6:     	movabsq	$0x65676965625f736b, %rax # imm = 0x65676965625F736B
14229dcb0:     	movq	%rax, 0xf(%rbx)
14229dcb4:     	movzbl	0x1396f4e(%rip), %eax   # 0x143634c09
14229dcbb:     	movl	$0x18, %ecx
14229dcc0:     	movl	$0x1, %edx
14229dcc5:     	callq	0x140613c10 <.text+0x612c10>
14229dcca:     	testq	%rax, %rax
14229dccd:     	je	0x1422a2112 <.text+0x22a1112>
14229dcd3:     	movq	%rax, %r14
14229dcd6:     	movups	0xd9e492(%rip), %xmm0   # 0x14303c16f
14229dcdd:     	movups	%xmm0, (%rax)
14229dce0:     	movabsq	$0x776f6c6c65795f73, %rax # imm = 0x776F6C6C65795F73
14229dcea:     	movq	%rax, 0x10(%r14)
14229dcee:     	movzbl	0x1396f14(%rip), %eax   # 0x143634c09
14229dcf5:     	movl	$0x17, %ecx
14229dcfa:     	movl	$0x1, %edx
14229dcff:     	callq	0x140613c10 <.text+0x612c10>
14229dd04:     	testq	%rax, %rax
14229dd07:     	je	0x1422a2013 <.text+0x22a1013>
14229dd0d:     	movq	%rax, %r12
14229dd10:     	movups	0xd9e470(%rip), %xmm0   # 0x14303c187
14229dd17:     	movups	%xmm0, (%rax)
14229dd1a:     	movabsq	$0x746867696c5f736b, %rax # imm = 0x746867696C5F736B
14229dd24:     	movq	%rax, 0xf(%r12)
14229dd29:     	movzbl	0x1396ed9(%rip), %eax   # 0x143634c09
14229dd30:     	movl	$0x16, %ecx
14229dd35:     	movl	$0x1, %edx
14229dd3a:     	callq	0x140613c10 <.text+0x612c10>
14229dd3f:     	testq	%rax, %rax
14229dd42:     	je	0x1422a1dc2 <.text+0x22a0dc2>
14229dd48:     	movq	%rax, %r13
14229dd4b:     	movups	0xd9e44c(%rip), %xmm0   # 0x14303c19e
14229dd52:     	movups	%xmm0, (%rax)
14229dd55:     	movabsq	$0x796572675f736b63, %rax # imm = 0x796572675F736B63
14229dd5f:     	movq	%rax, 0xe(%r13)
14229dd63:     	movzbl	0x1396e9f(%rip), %eax   # 0x143634c09
14229dd6a:     	movl	$0x16, %ecx
14229dd6f:     	movl	$0x1, %edx
14229dd74:     	callq	0x140613c10 <.text+0x612c10>
14229dd79:     	testq	%rax, %rax
14229dd7c:     	je	0x1422a2386 <.text+0x22a1386>
14229dd82:     	movq	%rax, %rcx
14229dd85:     	movups	0xd9e428(%rip), %xmm0   # 0x14303c1b4
14229dd8c:     	movups	%xmm0, (%rax)
14229dd8f:     	movabsq	$0x6b7261645f736b63, %rax # imm = 0x6B7261645F736B63
14229dd99:     	movq	%rcx, 0x1f0(%rbp)
14229dda0:     	movq	%rax, 0xe(%rcx)
14229dda4:     	movzbl	0x1396e5e(%rip), %eax   # 0x143634c09
14229ddab:     	movl	$0x5, %ecx
14229ddb0:     	movl	$0x1, %edx
14229ddb5:     	callq	0x140613c10 <.text+0x612c10>
14229ddba:     	testq	%rax, %rax
14229ddbd:     	je	0x1422a1eff <.text+0x22a0eff>
14229ddc3:     	movzbl	0xe98f8d(%rip), %ecx    # 0x143136d57
14229ddca:     	movb	%cl, 0x4(%rax)
14229ddcd:     	movl	0xe98f80(%rip), %ecx    # 0x143136d53
14229ddd3:     	movq	%rax, 0x1e8(%rbp)
14229ddda:     	movl	%ecx, (%rax)
14229dddc:     	movzbl	0x1396e26(%rip), %eax   # 0x143634c09
14229dde3:     	testb	%sil, %sil
14229dde6:     	je	0x14229e385 <.text+0x229d385>
14229ddec:     	movl	$0x1f, %ecx
14229ddf1:     	movl	$0x1, %edx
14229ddf6:     	callq	0x140613c10 <.text+0x612c10>
14229ddfb:     	testq	%rax, %rax
14229ddfe:     	je	0x1422a2b08 <.text+0x22a1b08>
14229de04:     	movups	0xe98fb4(%rip), %xmm0   # 0x143136dbf
14229de0b:     	movups	%xmm0, 0xf(%rax)
14229de0f:     	movups	0xe98f9a(%rip), %xmm0   # 0x143136db0
14229de16:     	movups	%xmm0, (%rax)
14229de19:     	movl	$0x1f, %ecx
14229de1e:     	jmp	0x14229e3c1 <.text+0x229d3c1>
14229de23:     	xorl	%esi, %esi
14229de25:     	leaq	0x180(%rbp), %rdi
14229de2c:     	movzbl	0x1396dd6(%rip), %eax   # 0x143634c09
14229de33:     	movl	$0x190, %ecx            # imm = 0x190
14229de38:     	movl	$0x8, %edx
14229de3d:     	callq	0x140613c10 <.text+0x612c10>
14229de42:     	testq	%rax, %rax
14229de45:     	je	0x1422a0f87 <.text+0x229ff87>
14229de4b:     	movq	%rax, %rbx
14229de4e:     	movzbl	0x1396db4(%rip), %eax   # 0x143634c09
14229de55:     	movl	$0x14, %ecx
14229de5a:     	movl	$0x1, %edx
14229de5f:     	callq	0x140613c10 <.text+0x612c10>
14229de64:     	testq	%rax, %rax
14229de67:     	je	0x1422a1901 <.text+0x22a0901>
14229de6d:     	movq	%rax, %r14
14229de70:     	movups	0xd9e659(%rip), %xmm0   # 0x14303c4d0
14229de77:     	movups	%xmm0, (%rax)
14229de7a:     	movl	$0x6465722f, 0x10(%rax) # imm = 0x6465722F
14229de81:     	movzbl	0x1396d81(%rip), %eax   # 0x143634c09
14229de88:     	movl	$0x15, %ecx
14229de8d:     	movl	$0x1, %edx
14229de92:     	callq	0x140613c10 <.text+0x612c10>
14229de97:     	testq	%rax, %rax
14229de9a:     	je	0x1422a132c <.text+0x22a032c>
14229dea0:     	movq	%rax, %r13
14229dea3:     	movups	0xd9e63a(%rip), %xmm0   # 0x14303c4e4
14229deaa:     	movups	%xmm0, (%rax)
14229dead:     	movabsq	$0x6b6e69702f736761, %rax # imm = 0x6B6E69702F736761
14229deb7:     	movq	%rax, 0xd(%r13)
14229debb:     	movzbl	0x1396d47(%rip), %eax   # 0x143634c09
14229dec2:     	movl	$0x17, %ecx
14229dec7:     	movl	$0x1, %edx
14229decc:     	callq	0x140613c10 <.text+0x612c10>
14229ded1:     	testq	%rax, %rax
14229ded4:     	je	0x1422a16d3 <.text+0x22a06d3>
14229deda:     	movq	%rax, %rcx
14229dedd:     	movups	0xd9e615(%rip), %xmm0   # 0x14303c4f9
14229dee4:     	movups	%xmm0, (%rax)
14229dee7:     	movabsq	$0x776f6c6c65792f73, %rax # imm = 0x776F6C6C65792F73
14229def1:     	movq	%rcx, 0x1f0(%rbp)
14229def8:     	movq	%rax, 0xf(%rcx)
14229defc:     	movzbl	0x1396d06(%rip), %eax   # 0x143634c09
14229df03:     	movl	$0x16, %ecx
14229df08:     	movl	$0x1, %edx
14229df0d:     	callq	0x140613c10 <.text+0x612c10>
14229df12:     	testq	%rax, %rax
14229df15:     	je	0x1422a1703 <.text+0x22a0703>
14229df1b:     	movq	%rax, %rcx
14229df1e:     	movups	0xd9e5eb(%rip), %xmm0   # 0x14303c510
14229df25:     	movups	%xmm0, (%rax)
14229df28:     	movabsq	$0x6e656572672f7367, %rax # imm = 0x6E656572672F7367
14229df32:     	movq	%rcx, 0x1e8(%rbp)
14229df39:     	movq	%rax, 0xe(%rcx)
14229df3d:     	movzbl	0x1396cc5(%rip), %eax   # 0x143634c09
14229df44:     	movl	$0x19, %ecx
14229df49:     	movl	$0x1, %edx
14229df4e:     	callq	0x140613c10 <.text+0x612c10>
14229df53:     	testq	%rax, %rax
14229df56:     	je	0x1422a1733 <.text+0x22a0733>
14229df5c:     	movups	0xd9e5cc(%rip), %xmm0   # 0x14303c52f
14229df63:     	movups	%xmm0, 0x9(%rax)
14229df67:     	movups	0xd9e5b8(%rip), %xmm0   # 0x14303c526
14229df6e:     	movq	%rax, 0x1d8(%rbp)
14229df75:     	movups	%xmm0, (%rax)
14229df78:     	movzbl	0x1396c8a(%rip), %eax   # 0x143634c09
14229df7f:     	movl	$0x15, %ecx
14229df84:     	movl	$0x1, %edx
14229df89:     	callq	0x140613c10 <.text+0x612c10>
14229df8e:     	testq	%rax, %rax
14229df91:     	je	0x1422a1763 <.text+0x22a0763>
14229df97:     	movq	%rax, %rcx
14229df9a:     	movups	0xd9e59e(%rip), %xmm0   # 0x14303c53f
14229dfa1:     	movups	%xmm0, (%rax)
14229dfa4:     	movabsq	$0x65756c622f736761, %rax # imm = 0x65756C622F736761
14229dfae:     	movq	%rcx, 0x1c0(%rbp)
14229dfb5:     	movq	%rax, 0xd(%rcx)
14229dfb9:     	movzbl	0x1396c49(%rip), %eax   # 0x143634c09
14229dfc0:     	movl	$0x1d, %ecx
14229dfc5:     	movl	$0x1, %edx
14229dfca:     	callq	0x140613c10 <.text+0x612c10>
14229dfcf:     	testq	%rax, %rax
14229dfd2:     	je	0x1422a1793 <.text+0x22a0793>
14229dfd8:     	movl	%r12d, 0x100(%rbp)
14229dfdf:     	movups	0xd9e57b(%rip), %xmm0   # 0x14303c561
14229dfe6:     	movups	%xmm0, 0xd(%rax)
14229dfea:     	movups	0xd9e563(%rip), %xmm0   # 0x14303c554
14229dff1:     	movq	%rax, 0x150(%rbp)
14229dff8:     	movups	%xmm0, (%rax)
14229dffb:     	movzbl	0x1396c07(%rip), %eax   # 0x143634c09
14229e002:     	movl	$0x17, %ecx
14229e007:     	movl	$0x1, %edx
14229e00c:     	callq	0x140613c10 <.text+0x612c10>
14229e011:     	testq	%rax, %rax
14229e014:     	je	0x1422a17c3 <.text+0x22a07c3>
14229e01a:     	movq	%rax, %rcx
14229e01d:     	movq	%rdi, %r12
14229e020:     	movups	0xd9e54a(%rip), %xmm0   # 0x14303c571
14229e027:     	movups	%xmm0, (%rax)
14229e02a:     	movabsq	$0x656c707275702f73, %rax # imm = 0x656C707275702F73
14229e034:     	movq	%rcx, 0x148(%rbp)
14229e03b:     	movq	%rax, 0xf(%rcx)
14229e03f:     	movzbl	0x1396bc3(%rip), %eax   # 0x143634c09
14229e046:     	movl	$0x1d, %ecx
14229e04b:     	movl	$0x1, %edx
14229e050:     	callq	0x140613c10 <.text+0x612c10>
14229e055:     	testq	%rax, %rax
14229e058:     	je	0x1422a19f8 <.text+0x22a09f8>
14229e05e:     	movups	0xd9e530(%rip), %xmm0   # 0x14303c595
14229e065:     	movups	%xmm0, 0xd(%rax)
14229e069:     	movups	0xd9e518(%rip), %xmm0   # 0x14303c588
14229e070:     	movups	%xmm0, (%rax)
14229e073:     	movq	%rax, %rdi
14229e076:     	movzbl	0x1396b8c(%rip), %eax   # 0x143634c09
14229e07d:     	movl	$0x15, %ecx
14229e082:     	movl	$0x1, %edx
14229e087:     	callq	0x140613c10 <.text+0x612c10>
14229e08c:     	testq	%rax, %rax
14229e08f:     	je	0x1422a13cb <.text+0x22a03cb>
14229e095:     	movups	0xd9e509(%rip), %xmm0   # 0x14303c5a5
14229e09c:     	movups	%xmm0, (%rax)
14229e09f:     	movabsq	$0x6b7261642f736761, %rcx # imm = 0x6B7261642F736761
14229e0a9:     	movq	%rcx, 0xd(%rax)
14229e0ad:     	movq	$0x14, (%rbx)
14229e0b4:     	movq	%r14, 0x8(%rbx)
14229e0b8:     	movq	$0x14, 0x10(%rbx)
14229e0c0:     	movb	$0x3, 0x18(%rbx)
14229e0c4:     	movl	$0x1, 0x1c(%rbx)
14229e0cb:     	movl	$0x0, 0x24(%rbx)
14229e0d2:     	movq	$0x15, 0x28(%rbx)
14229e0da:     	movq	%r13, 0x30(%rbx)
14229e0de:     	movq	$0x15, 0x38(%rbx)
14229e0e6:     	movb	$0x3, 0x40(%rbx)
14229e0ea:     	movl	$0x7, 0x44(%rbx)
14229e0f1:     	movl	$0x0, 0x4c(%rbx)
14229e0f8:     	movq	$0x17, 0x50(%rbx)
14229e100:     	movq	0x1f0(%rbp), %rcx
14229e107:     	movq	%rcx, 0x58(%rbx)
14229e10b:     	movq	$0x17, 0x60(%rbx)
14229e113:     	movb	$0x3, 0x68(%rbx)
14229e117:     	movl	$0x3, 0x6c(%rbx)
14229e11e:     	movl	$0x0, 0x74(%rbx)
14229e125:     	movq	$0x16, 0x78(%rbx)
14229e12d:     	movq	0x1e8(%rbp), %rcx
14229e134:     	movq	%rcx, 0x80(%rbx)
14229e13b:     	movq	$0x16, 0x88(%rbx)
14229e146:     	movb	$0x3, 0x90(%rbx)
14229e14d:     	movl	$0x2, 0x94(%rbx)
14229e157:     	movl	$0x0, 0x9c(%rbx)
14229e161:     	movq	$0x19, 0xa0(%rbx)
14229e16c:     	movq	0x1d8(%rbp), %rcx
14229e173:     	movq	%rcx, 0xa8(%rbx)
14229e17a:     	movq	$0x19, 0xb0(%rbx)
14229e185:     	movb	$0x3, 0xb8(%rbx)
14229e18c:     	movl	$0x9, 0xbc(%rbx)
14229e196:     	movl	$0x0, 0xc4(%rbx)
14229e1a0:     	movq	$0x15, 0xc8(%rbx)
14229e1ab:     	movq	0x1c0(%rbp), %rcx
14229e1b2:     	movq	%rcx, 0xd0(%rbx)
14229e1b9:     	movq	$0x15, 0xd8(%rbx)
14229e1c4:     	movb	$0x3, 0xe0(%rbx)
14229e1cb:     	movl	$0x0, 0xe4(%rbx)
14229e1d5:     	movl	$0x0, 0xec(%rbx)
14229e1df:     	movq	$0x1d, 0xf0(%rbx)
14229e1ea:     	movq	0x150(%rbp), %rcx
14229e1f1:     	movq	%rcx, 0xf8(%rbx)
14229e1f8:     	movq	$0x1d, 0x100(%rbx)
14229e203:     	movb	$0x3, 0x108(%rbx)
14229e20a:     	movl	$0x6, 0x10c(%rbx)
14229e214:     	movl	$0x0, 0x114(%rbx)
14229e21e:     	movq	$0x17, 0x118(%rbx)
14229e229:     	movq	0x148(%rbp), %rcx
14229e230:     	movq	%rcx, 0x120(%rbx)
14229e237:     	movq	$0x17, 0x128(%rbx)
14229e242:     	movb	$0x3, 0x130(%rbx)
14229e249:     	movl	$0x4, 0x134(%rbx)
14229e253:     	movl	$0x0, 0x13c(%rbx)
14229e25d:     	movq	$0x1d, 0x140(%rbx)
14229e268:     	movq	%rdi, 0x148(%rbx)
14229e26f:     	movq	$0x1d, 0x150(%rbx)
14229e27a:     	movb	$0x3, 0x158(%rbx)
14229e281:     	movl	$0x8, 0x15c(%rbx)
14229e28b:     	movl	$0x0, 0x164(%rbx)
14229e295:     	movq	$0x15, 0x168(%rbx)
14229e2a0:     	movq	%rax, 0x170(%rbx)
14229e2a7:     	movq	$0x15, 0x178(%rbx)
14229e2b2:     	movb	$0x3, 0x180(%rbx)
14229e2b9:     	movl	$0x5, 0x184(%rbx)
14229e2c3:     	movl	$0x0, 0x18c(%rbx)
14229e2cd:     	movq	$0xa, 0x180(%rbp)
14229e2d8:     	movq	%rbx, 0x188(%rbp)
14229e2df:     	movq	$0xa, 0x190(%rbp)
14229e2ea:     	testb	%sil, %sil
14229e2ed:     	je	0x14229e80d <.text+0x229d80d>
14229e2f3:     	movq	0x200(%rbp), %rcx
14229e2fa:     	movl	0x100(%rbp), %edx
14229e300:     	callq	0x140a1c930 <.text+0xa1b930>
14229e305:     	testq	%rax, %rax
14229e308:     	je	0x14229e67b <.text+0x229d67b>
14229e30e:     	addq	$0x30, %rax
14229e312:     	movq	%rax, %rcx
14229e315:     	leaq	0xd9e6c4(%rip), %rdx    # 0x14303c9e0
14229e31c:     	callq	0x140839340 <.text+0x838340>
14229e321:     	movzbl	(%rax), %ebx
14229e324:     	jmp	0x14229e67d <.text+0x229d67d>
14229e329:     	movl	$0x21, %ecx
14229e32e:     	movl	$0x1, %edx
14229e333:     	callq	0x140613c10 <.text+0x612c10>
14229e338:     	testq	%rax, %rax
14229e33b:     	je	0x1422a2a50 <.text+0x22a1a50>
14229e341:     	movzbl	0xe98a67(%rip), %ecx    # 0x143136daf
14229e348:     	movb	%cl, 0x20(%rax)
14229e34b:     	movups	0xe98a4d(%rip), %xmm0   # 0x143136d9f
14229e352:     	movups	%xmm0, 0x10(%rax)
14229e356:     	movups	0xe98a32(%rip), %xmm0   # 0x143136d8f
14229e35d:     	movups	%xmm0, (%rax)
14229e360:     	movl	$0x21, %ecx
14229e365:     	movq	%rcx, (%rdi)
14229e368:     	movq	%rax, 0x8(%rdi)
14229e36c:     	movq	%rcx, 0x10(%rdi)
14229e370:     	movb	$0xa, 0x18(%rdi)
14229e374:     	movq	%rdi, %rax
14229e377:     	addq	$0x24, %rax
14229e37b:     	movl	$0x1, %ecx
14229e380:     	jmp	0x14229e505 <.text+0x229d505>
14229e385:     	movl	$0x21, %ecx
14229e38a:     	movl	$0x1, %edx
14229e38f:     	callq	0x140613c10 <.text+0x612c10>
14229e394:     	testq	%rax, %rax
14229e397:     	je	0x1422a2a94 <.text+0x22a1a94>
14229e39d:     	movzbl	0xe98a0b(%rip), %ecx    # 0x143136daf
14229e3a4:     	movb	%cl, 0x20(%rax)
14229e3a7:     	movups	0xe989f1(%rip), %xmm0   # 0x143136d9f
14229e3ae:     	movups	%xmm0, 0x10(%rax)
14229e3b2:     	movups	0xe989d6(%rip), %xmm0   # 0x143136d8f
14229e3b9:     	movups	%xmm0, (%rax)
14229e3bc:     	movl	$0x21, %ecx
14229e3c1:     	movq	0x1e8(%rbp), %r8
14229e3c8:     	movq	$0x17, (%rdi)
14229e3cf:     	movq	%rbx, 0x8(%rdi)
14229e3d3:     	movq	$0x17, 0x10(%rdi)
14229e3db:     	movb	$0x3, 0x18(%rdi)
14229e3df:     	movl	$0x186a0, 0x1c(%rdi)    # imm = 0x186A0
14229e3e6:     	movl	$0x0, 0x24(%rdi)
14229e3ed:     	movq	$0x18, 0x28(%rdi)
14229e3f5:     	movq	%r14, 0x30(%rdi)
14229e3f9:     	movq	$0x18, 0x38(%rdi)
14229e401:     	movb	$0x3, 0x40(%rdi)
14229e405:     	movl	$0x186a1, 0x44(%rdi)    # imm = 0x186A1
14229e40c:     	movl	$0x0, 0x4c(%rdi)
14229e413:     	movq	$0x17, 0x50(%rdi)
14229e41b:     	movq	%r12, 0x58(%rdi)
14229e41f:     	movq	$0x17, 0x60(%rdi)
14229e427:     	movb	$0x3, 0x68(%rdi)
14229e42b:     	movl	$0x186a3, 0x6c(%rdi)    # imm = 0x186A3
14229e432:     	movl	$0x0, 0x74(%rdi)
14229e439:     	movq	$0x16, 0x78(%rdi)
14229e441:     	movq	%r13, 0x80(%rdi)
14229e448:     	movq	$0x16, 0x88(%rdi)
14229e453:     	movb	$0x3, 0x90(%rdi)
14229e45a:     	movl	$0x186ab, 0x94(%rdi)    # imm = 0x186AB
14229e464:     	movl	$0x0, 0x9c(%rdi)
14229e46e:     	movq	$0x16, 0xa0(%rdi)
14229e479:     	movq	0x1f0(%rbp), %rdx
14229e480:     	movq	%rdx, 0xa8(%rdi)
14229e487:     	movq	$0x16, 0xb0(%rdi)
14229e492:     	movb	$0x3, 0xb8(%rdi)
14229e499:     	movl	$0x186a2, 0xbc(%rdi)    # imm = 0x186A2
14229e4a3:     	movl	$0x0, 0xc4(%rdi)
14229e4ad:     	movq	$0x5, 0xc8(%rdi)
14229e4b8:     	movq	%r8, 0xd0(%rdi)
14229e4bf:     	movq	$0x5, 0xd8(%rdi)
14229e4ca:     	movb	$0x5, 0xe0(%rdi)
14229e4d1:     	movl	$0x0, 0xec(%rdi)
14229e4db:     	movq	%rcx, 0xf0(%rdi)
14229e4e2:     	movq	%rax, 0xf8(%rdi)
14229e4e9:     	movq	%rcx, 0x100(%rdi)
14229e4f0:     	movb	$0xa, 0x108(%rdi)
14229e4f7:     	movq	%rdi, %rax
14229e4fa:     	addq	$0x114, %rax            # imm = 0x114
14229e500:     	movl	$0x7, %ecx
14229e505:     	movl	$0x0, (%rax)
14229e50b:     	movq	%rcx, 0x110(%rbp)
14229e512:     	movq	%rdi, 0x118(%rbp)
14229e519:     	movq	%rcx, 0x120(%rbp)
14229e520:     	jmp	0x14229e9e0 <.text+0x229d9e0>
14229e525:     	movl	$0xc8, %ecx
14229e52a:     	movl	$0x8, %edx
14229e52f:     	callq	0x140613c10 <.text+0x612c10>
14229e534:     	testq	%rax, %rax
14229e537:     	leaq	0x180(%rbp), %rdi
14229e53e:     	je	0x1422a2c5d <.text+0x22a1c5d>
14229e544:     	movq	%rax, %rsi
14229e547:     	movzbl	0x13966bb(%rip), %eax   # 0x143634c09
14229e54e:     	movl	$0x5, %ecx
14229e553:     	movl	$0x1, %edx
14229e558:     	callq	0x140613c10 <.text+0x612c10>
14229e55d:     	testq	%rax, %rax
14229e560:     	movq	%rsi, 0x1f0(%rbp)
14229e567:     	je	0x1422a2c27 <.text+0x22a1c27>
14229e56d:     	movzbl	0xe987e3(%rip), %ebx    # 0x143136d57
14229e574:     	movb	%bl, 0x4(%rax)
14229e577:     	movl	0xe987d6(%rip), %esi    # 0x143136d53
14229e57d:     	movq	%rax, 0x1e8(%rbp)
14229e584:     	movl	%esi, (%rax)
14229e586:     	movzbl	0x139667c(%rip), %eax   # 0x143634c09
14229e58d:     	movl	$0x10, %ecx
14229e592:     	movl	$0x1, %edx
14229e597:     	callq	0x140613c10 <.text+0x612c10>
14229e59c:     	testq	%rax, %rax
14229e59f:     	je	0x1422a2bdc <.text+0x22a1bdc>
14229e5a5:     	movups	0xd9dc91(%rip), %xmm0   # 0x14303c23d
14229e5ac:     	movq	%rax, 0x1d8(%rbp)
14229e5b3:     	movups	%xmm0, (%rax)
14229e5b6:     	movzbl	0x139664c(%rip), %eax   # 0x143634c09
14229e5bd:     	movl	$0x13, %ecx
14229e5c2:     	movl	$0x1, %edx
14229e5c7:     	callq	0x140613c10 <.text+0x612c10>
14229e5cc:     	testq	%rax, %rax
14229e5cf:     	je	0x1422a2c71 <.text+0x22a1c71>
14229e5d5:     	movups	0xd9dc71(%rip), %xmm0   # 0x14303c24d
14229e5dc:     	movups	%xmm0, (%rax)
14229e5df:     	movq	%rax, 0x1c0(%rbp)
14229e5e6:     	movl	$0x7472696b, 0xf(%rax)  # imm = 0x7472696B
14229e5ed:     	movzbl	0x1396615(%rip), %eax   # 0x143634c09
14229e5f4:     	movl	$0x5, %ecx
14229e5f9:     	movl	$0x1, %edx
14229e5fe:     	callq	0x140613c10 <.text+0x612c10>
14229e603:     	testq	%rax, %rax
14229e606:     	je	0x1422a2c42 <.text+0x22a1c42>
14229e60c:     	movb	%bl, 0x4(%rax)
14229e60f:     	movq	%rax, 0x150(%rbp)
14229e616:     	movl	%esi, (%rax)
14229e618:     	movzbl	0x295(%r14), %eax
14229e620:     	movb	%al, 0x180(%rbp)
14229e626:     	movq	%rdi, %rcx
14229e629:     	callq	0x140833230 <.text+0x832230>
14229e62e:     	movzbl	0x13965d4(%rip), %ecx   # 0x143634c09
14229e635:     	testb	%al, %al
14229e637:     	je	0x14229e83c <.text+0x229d83c>
14229e63d:     	movl	$0x1e, %ecx
14229e642:     	movl	$0x1, %edx
14229e647:     	callq	0x140613c10 <.text+0x612c10>
14229e64c:     	testq	%rax, %rax
14229e64f:     	movq	0x1f0(%rbp), %rsi
14229e656:     	je	0x1422a2ca7 <.text+0x22a1ca7>
14229e65c:     	movups	0xe987e9(%rip), %xmm0   # 0x143136e4c
14229e663:     	movups	%xmm0, 0xe(%rax)
14229e667:     	movups	0xe987d0(%rip), %xmm0   # 0x143136e3e
14229e66e:     	movups	%xmm0, (%rax)
14229e671:     	movl	$0x1e, %ecx
14229e676:     	jmp	0x14229e875 <.text+0x229d875>
14229e67b:     	movb	$0x1, %bl
14229e67d:     	movzbl	0x1396585(%rip), %eax   # 0x143634c09
14229e684:     	movl	$0x5, %ecx
14229e689:     	movl	$0x1, %edx
14229e68e:     	callq	0x140613c10 <.text+0x612c10>
14229e693:     	testq	%rax, %rax
14229e696:     	je	0x1422a1f3d <.text+0x22a0f3d>
14229e69c:     	movzbl	0xe986b4(%rip), %ecx    # 0x143136d57
14229e6a3:     	movb	%cl, 0x4(%rax)
14229e6a6:     	movl	0xe986a7(%rip), %ecx    # 0x143136d53
14229e6ac:     	movq	%rax, 0x1f0(%rbp)
14229e6b3:     	movl	%ecx, (%rax)
14229e6b5:     	movq	0x190(%rbp), %rsi
14229e6bc:     	cmpq	0x180(%rbp), %rsi
14229e6c3:     	jne	0x14229e6d4 <.text+0x229d6d4>
14229e6c5:     	movq	%r12, %rcx
14229e6c8:     	leaq	0xd9def1(%rip), %rdx    # 0x14303c5c0
14229e6cf:     	callq	0x140562ab0 <.text+0x561ab0>
14229e6d4:     	movq	0x188(%rbp), %rax
14229e6db:     	leaq	(%rsi,%rsi,4), %rcx
14229e6df:     	movq	$0x5, (%rax,%rcx,8)
14229e6e7:     	movq	0x1f0(%rbp), %rdx
14229e6ee:     	movq	%rdx, 0x8(%rax,%rcx,8)
14229e6f3:     	movq	$0x5, 0x10(%rax,%rcx,8)
14229e6fc:     	movb	$0x5, 0x18(%rax,%rcx,8)
14229e701:     	movl	$0x0, 0x24(%rax,%rcx,8)
14229e709:     	incq	%rsi
14229e70c:     	movq	%rsi, 0x190(%rbp)
14229e713:     	movzbl	0x13964ef(%rip), %eax   # 0x143634c09
14229e71a:     	testb	%bl, %bl
14229e71c:     	je	0x14229e770 <.text+0x229d770>
14229e71e:     	movl	$0x1a, %ecx
14229e723:     	movl	$0x1, %edx
14229e728:     	callq	0x140613c10 <.text+0x612c10>
14229e72d:     	testq	%rax, %rax
14229e730:     	je	0x1422a2aed <.text+0x22a1aed>
14229e736:     	movups	0xe98642(%rip), %xmm0   # 0x143136d7f
14229e73d:     	movups	%xmm0, 0xa(%rax)
14229e741:     	movups	0xe9862d(%rip), %xmm0   # 0x143136d75
14229e748:     	movq	%rax, 0x1f0(%rbp)
14229e74f:     	movups	%xmm0, (%rax)
14229e752:     	movl	$0x1a, %eax
14229e757:     	movq	%rax, 0x1e8(%rbp)
14229e75e:     	movq	0x190(%rbp), %rsi
14229e765:     	cmpq	0x180(%rbp), %rsi
14229e76c:     	je	0x14229e7c0 <.text+0x229d7c0>
14229e76e:     	jmp	0x14229e7cf <.text+0x229d7cf>
14229e770:     	movl	$0x1d, %ecx
14229e775:     	movl	$0x1, %edx
14229e77a:     	callq	0x140613c10 <.text+0x612c10>
14229e77f:     	testq	%rax, %rax
14229e782:     	je	0x1422a2ad2 <.text+0x22a1ad2>
14229e788:     	movups	0xe985d6(%rip), %xmm0   # 0x143136d65
14229e78f:     	movups	%xmm0, 0xd(%rax)
14229e793:     	movups	0xe985be(%rip), %xmm0   # 0x143136d58
14229e79a:     	movq	%rax, 0x1f0(%rbp)
14229e7a1:     	movups	%xmm0, (%rax)
14229e7a4:     	movl	$0x1d, %eax
14229e7a9:     	movq	%rax, 0x1e8(%rbp)
14229e7b0:     	movq	0x190(%rbp), %rsi
14229e7b7:     	cmpq	0x180(%rbp), %rsi
14229e7be:     	jne	0x14229e7cf <.text+0x229d7cf>
14229e7c0:     	movq	%r12, %rcx
14229e7c3:     	leaq	0xd9de0e(%rip), %rdx    # 0x14303c5d8
14229e7ca:     	callq	0x140562ab0 <.text+0x561ab0>
14229e7cf:     	movq	0x188(%rbp), %rax
14229e7d6:     	leaq	(%rsi,%rsi,4), %rcx
14229e7da:     	movq	0x1e8(%rbp), %r8
14229e7e1:     	movq	%r8, (%rax,%rcx,8)
14229e7e5:     	movq	0x1f0(%rbp), %rdx
14229e7ec:     	movq	%rdx, 0x8(%rax,%rcx,8)
14229e7f1:     	movq	%r8, 0x10(%rax,%rcx,8)
14229e7f6:     	movb	$0x6, 0x18(%rax,%rcx,8)
14229e7fb:     	movl	$0x0, 0x24(%rax,%rcx,8)
14229e803:     	incq	%rsi
14229e806:     	movq	%rsi, 0x190(%rbp)
14229e80d:     	movq	0x190(%rbp), %rax
14229e814:     	movq	%rax, 0x120(%rbp)
14229e81b:     	movq	0x180(%rbp), %rax
14229e822:     	movq	%rax, 0x110(%rbp)
14229e829:     	movq	0x188(%rbp), %rax
14229e830:     	movq	%rax, 0x118(%rbp)
14229e837:     	jmp	0x14229e9e0 <.text+0x229d9e0>
14229e83c:     	movl	$0x1f, %ecx
14229e841:     	movl	$0x1, %edx
14229e846:     	callq	0x140613c10 <.text+0x612c10>
14229e84b:     	testq	%rax, %rax
14229e84e:     	movq	0x1f0(%rbp), %rsi
14229e855:     	je	0x1422a2c8c <.text+0x22a1c8c>
14229e85b:     	movups	0xe985cc(%rip), %xmm0   # 0x143136e2e
14229e862:     	movups	%xmm0, 0xf(%rax)
14229e866:     	movups	0xe985b2(%rip), %xmm0   # 0x143136e1f
14229e86d:     	movups	%xmm0, (%rax)
14229e870:     	movl	$0x1f, %ecx
14229e875:     	movq	$0x5, (%rsi)
14229e87c:     	movq	0x1e8(%rbp), %rdx
14229e883:     	movq	%rdx, 0x8(%rsi)
14229e887:     	movq	$0x5, 0x10(%rsi)
14229e88f:     	movb	$0x5, 0x18(%rsi)
14229e893:     	movl	$0x0, 0x24(%rsi)
14229e89a:     	movq	$0x10, 0x28(%rsi)
14229e8a2:     	movq	0x1d8(%rbp), %rdx
14229e8a9:     	movq	%rdx, 0x30(%rsi)
14229e8ad:     	movq	$0x10, 0x38(%rsi)
14229e8b5:     	movw	$0x104, 0x40(%rsi)      # imm = 0x104
14229e8bb:     	movl	$0x0, 0x4c(%rsi)
14229e8c2:     	movq	$0x13, 0x50(%rsi)
14229e8ca:     	movq	0x1c0(%rbp), %rdx
14229e8d1:     	movq	%rdx, 0x58(%rsi)
14229e8d5:     	movq	$0x13, 0x60(%rsi)
14229e8dd:     	movw	$0x4, 0x68(%rsi)
14229e8e3:     	movl	$0x0, 0x74(%rsi)
14229e8ea:     	movq	$0x5, 0x78(%rsi)
14229e8f2:     	movq	0x150(%rbp), %rdx
14229e8f9:     	movq	%rdx, 0x80(%rsi)
14229e900:     	movq	$0x5, 0x88(%rsi)
14229e90b:     	movb	$0x5, 0x90(%rsi)
14229e912:     	movl	$0x0, 0x9c(%rsi)
14229e91c:     	movq	%rcx, 0xa0(%rsi)
14229e923:     	movq	%rax, 0xa8(%rsi)
14229e92a:     	movq	%rcx, 0xb0(%rsi)
14229e931:     	movb	$0x7, 0xb8(%rsi)
14229e938:     	movl	$0x0, 0xc4(%rsi)
14229e942:     	movq	%rsi, %rax
14229e945:     	addq	$0xc8, %rax
14229e94b:     	movq	%rsi, 0x180(%rbp)
14229e952:     	movq	$0x5, 0x190(%rbp)
14229e95d:     	movq	%rsi, 0x188(%rbp)
14229e964:     	movq	%rax, 0x198(%rbp)
14229e96b:     	movq	0x158(%rbp), %rax
14229e972:     	movq	0x168(%rbp), %rdi
14229e979:     	subq	%rdi, %rax
14229e97c:     	cmpq	$0x5, %rax
14229e980:     	jb	0x1422a0d87 <.text+0x229fd87>
14229e986:     	leaq	(%rdi,%rdi,4), %rcx
14229e98a:     	shlq	$0x3, %rcx
14229e98e:     	addq	0x160(%rbp), %rcx
14229e995:     	movl	$0xc8, %r8d
14229e99b:     	movq	%rsi, %rdx
14229e99e:     	callq	0x142923c00 <.text+0x2922c00>
14229e9a3:     	addq	$0x5, %rdi
14229e9a7:     	movq	%rdi, 0x168(%rbp)
14229e9ae:     	movl	$0xc8, %edx
14229e9b3:     	movl	$0x8, %r8d
14229e9b9:     	movq	%rsi, %rcx
14229e9bc:     	callq	0x140613c20 <.text+0x612c20>
14229e9c1:     	movq	0x168(%rbp), %rax
14229e9c8:     	movq	%rax, 0x120(%rbp)
14229e9cf:     	movups	0x158(%rbp), %xmm0
14229e9d6:     	movaps	%xmm0, 0x110(%rbp)
14229e9dd:     	nopl	(%rax)
14229e9e0:     	movq	(%rbp), %rax
14229e9e4:     	movss	0xe8(%rax), %xmm15
14229e9ed:     	movaps	%xmm15, %xmm10
14229e9f1:     	mulss	0x72e8da(%rip), %xmm15  # 0x1429cd2d4
14229e9fa:     	xorl	%edi, %edi
14229e9fc:     	cmpb	$0x0, (%r15)
14229ea00:     	sete	%dil
14229ea04:     	shll	$0x2, %edi
14229ea07:     	leaq	0xd9d5d2(%rip), %rax    # 0x14303bfe0
14229ea0e:     	mulss	(%rdi,%rax), %xmm15
14229ea14:     	xorps	%xmm0, %xmm0
14229ea17:     	cvtsi2ssq	0x120(%rbp), %xmm0
14229ea20:     	mulss	0x778417(%rip), %xmm10  # 0x142a16e40
14229ea29:     	movaps	%xmm10, %xmm6
14229ea2d:     	addss	%xmm15, %xmm6
14229ea32:     	mulss	%xmm0, %xmm6
14229ea36:     	divss	0xd9d5aa(%rip), %xmm6   # 0x14303bfe8
14229ea3e:     	maxss	%xmm13, %xmm6
14229ea43:     	movq	$0x0, 0x130(%rbp)
14229ea4e:     	movq	$0x8, 0x138(%rbp)
14229ea59:     	movq	$0x0, 0x140(%rbp)
14229ea64:     	movzbl	(%r15), %eax
14229ea68:     	leaq	0xd9e035(%rip), %rcx    # 0x14303caa4
14229ea6f:     	movslq	(%rcx,%rax,4), %rax
14229ea73:     	addq	%rcx, %rax
14229ea76:     	movq	%r15, %r14
14229ea79:     	jmpq	*%rax
14229ea7b:     	movl	0x4(%r15), %edx
14229ea7f:     	movq	0x200(%rbp), %rcx
14229ea86:     	callq	0x140a1c930 <.text+0xa1b930>
14229ea8b:     	testq	%rax, %rax
14229ea8e:     	je	0x1422a0eac <.text+0x229feac>
14229ea94:     	movl	0x28(%rax), %eax
14229ea97:     	jmp	0x14229f058 <.text+0x229e058>
14229ea9c:     	movq	0x8(%r15), %rax
14229eaa0:     	movq	%rax, 0x180(%rbp)
14229eaa7:     	leaq	0x180(%rbp), %rcx
14229eaae:     	callq	0x1400885d0 <.text+0x875d0>
14229eab3:     	movq	0x90(%rbp), %rcx
14229eaba:     	movq	0x10(%rcx), %rdx
14229eabe:     	cmpq	%rdx, %rax
14229eac1:     	jb	0x14229f051 <.text+0x229e051>
14229eac7:     	jmp	0x1422a1a28 <.text+0x22a0a28>
14229eacc:     	movq	0x8(%r15), %rdx
14229ead0:     	movq	0x1f8(%rbp), %rcx
14229ead7:     	callq	0x140a922b0 <.text+0xa912b0>
14229eadc:     	movq	%rax, %rcx
14229eadf:     	shrq	$0x20, %rcx
14229eae3:     	je	0x1422a1a3c <.text+0x22a0a3c>
14229eae9:     	movl	%eax, %r8d
14229eaec:     	movq	0x1b0(%rbp), %rdx
14229eaf3:     	cmpq	0x10(%rdx), %r8
14229eaf7:     	jae	0x1422a0f33 <.text+0x229ff33>
14229eafd:     	movq	0x8(%rdx), %rdx
14229eb01:     	movq	%rax, %rcx
14229eb04:     	shrq	$0x20, %rcx
14229eb08:     	leaq	(%r8,%r8,4), %r9
14229eb0c:     	cmpl	%ecx, (%rdx,%r9,4)
14229eb10:     	jne	0x1422a0f33 <.text+0x229ff33>
14229eb16:     	leaq	(%rdx,%r9,4), %r9
14229eb1a:     	movl	0x4(%r9), %edx
14229eb1e:     	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14229eb24:     	cmpq	%r10, %rdx
14229eb27:     	je	0x1422a0f33 <.text+0x229ff33>
14229eb2d:     	xorl	%r8d, %r8d
14229eb30:     	movq	0x88(%rbp), %r11
14229eb37:     	cmpq	%rdx, 0x38(%r11)
14229eb3b:     	jbe	0x1422a0f43 <.text+0x229ff43>
14229eb41:     	movq	0x28(%r11), %rcx
14229eb45:     	movl	%edx, %r10d
14229eb48:     	shrl	$0x6, %r10d
14229eb4c:     	movq	(%rcx,%r10,8), %rcx
14229eb50:     	btq	%rdx, %rcx
14229eb54:     	jb	0x14229ec1d <.text+0x229dc1d>
14229eb5a:     	jmp	0x1422a0f43 <.text+0x229ff43>
14229eb5f:     	movq	0x8(%r15), %rax
14229eb63:     	movq	%rax, 0x180(%rbp)
14229eb6a:     	leaq	0x180(%rbp), %rcx
14229eb71:     	callq	0x1400885d0 <.text+0x875d0>
14229eb76:     	movq	0x178(%rbp), %rcx
14229eb7d:     	movq	0x10(%rcx), %rdx
14229eb81:     	cmpq	%rdx, %rax
14229eb84:     	jb	0x14229f051 <.text+0x229e051>
14229eb8a:     	jmp	0x1422a1a4d <.text+0x22a0a4d>
14229eb8f:     	movq	0x8(%r15), %rdx
14229eb93:     	movq	0x1f8(%rbp), %rcx
14229eb9a:     	callq	0x140a922b0 <.text+0xa912b0>
14229eb9f:     	movq	%rax, %rcx
14229eba2:     	shrq	$0x20, %rcx
14229eba6:     	je	0x1422a1a61 <.text+0x22a0a61>
14229ebac:     	movl	%eax, %r8d
14229ebaf:     	movq	0x1b0(%rbp), %rdx
14229ebb6:     	cmpq	0x10(%rdx), %r8
14229ebba:     	jae	0x1422a0edf <.text+0x229fedf>
14229ebc0:     	movq	0x8(%rdx), %rdx
14229ebc4:     	movq	%rax, %rcx
14229ebc7:     	shrq	$0x20, %rcx
14229ebcb:     	leaq	(%r8,%r8,4), %r9
14229ebcf:     	cmpl	%ecx, (%rdx,%r9,4)
14229ebd3:     	jne	0x1422a0edf <.text+0x229fedf>
14229ebd9:     	leaq	(%rdx,%r9,4), %r9
14229ebdd:     	movl	0x4(%r9), %edx
14229ebe1:     	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14229ebe7:     	cmpq	%r10, %rdx
14229ebea:     	je	0x1422a0edf <.text+0x229fedf>
14229ebf0:     	xorl	%r8d, %r8d
14229ebf3:     	movq	0x88(%rbp), %r11
14229ebfa:     	cmpq	%rdx, 0x38(%r11)
14229ebfe:     	jbe	0x1422a0eef <.text+0x229feef>
14229ec04:     	movq	0x28(%r11), %rcx
14229ec08:     	movl	%edx, %r10d
14229ec0b:     	shrl	$0x6, %r10d
14229ec0f:     	movq	(%rcx,%r10,8), %rcx
14229ec13:     	btq	%rdx, %rcx
14229ec17:     	jae	0x1422a0eef <.text+0x229feef>
14229ec1d:     	movq	%r14, %rsi
14229ec20:     	movl	0xc(%r9), %eax
14229ec24:     	movl	0x10(%r9), %ecx
14229ec28:     	movq	0x1b0(%rbp), %rdx
14229ec2f:     	movq	0x1a8(%rdx), %rdx
14229ec36:     	leaq	(%rax,%rax,8), %rax
14229ec3a:     	movq	0x110(%r11), %r8
14229ec41:     	movq	0x18(%rdx,%rax,8), %r9
14229ec46:     	movq	0x38(%rdx,%rax,8), %rax
14229ec4b:     	movq	(%rax,%r8,8), %rax
14229ec4f:     	notq	%rax
14229ec52:     	leaq	(%rax,%rax,2), %rax
14229ec56:     	shlq	$0x4, %rax
14229ec5a:     	movq	0x10(%r9,%rax), %rax
14229ec5f:     	imulq	$0x58, %rcx, %rcx
14229ec63:     	movl	0x50(%rax,%rcx), %eax
14229ec67:     	movl	%eax, 0x1d8(%rbp)
14229ec6d:     	movzbl	(%rsi), %eax
14229ec70:     	xorps	%xmm9, %xmm9
14229ec74:     	testb	%al, %al
14229ec76:     	jne	0x14229f070 <.text+0x229e070>
14229ec7c:     	nopl	(%rax)
14229ec80:     	movq	0x8(%rsi), %rdx
14229ec84:     	movq	0x1f8(%rbp), %rcx
14229ec8b:     	leaq	0xd9db26(%rip), %r8     # 0x14303c7b8
14229ec92:     	callq	0x140a91ef0 <.text+0xa90ef0>
14229ec97:     	testq	%rax, %rax
14229ec9a:     	je	0x1422a0ebd <.text+0x229febd>
14229eca0:     	movzbl	0x299(%rax), %eax
14229eca7:     	movl	%eax, 0x1c0(%rbp)
14229ecad:     	movq	%r14, %rsi
14229ecb0:     	movzbl	(%r14), %eax
14229ecb4:     	testb	%al, %al
14229ecb6:     	je	0x14229eed8 <.text+0x229ded8>
14229ecbc:     	movl	$0x0, 0x1f0(%rbp)
14229ecc6:     	movl	$0x0, 0x1e8(%rbp)
14229ecd0:     	cmpb	$0xb, %al
14229ecd2:     	jne	0x14229f096 <.text+0x229e096>
14229ecd8:     	movl	0x4(%rsi), %edx
14229ecdb:     	movq	0x200(%rbp), %rcx
14229ece2:     	callq	0x140a1c930 <.text+0xa1b930>
14229ece7:     	testq	%rax, %rax
14229ecea:     	je	0x14229ed20 <.text+0x229dd20>
14229ecec:     	addq	$0x30, %rax
14229ecf0:     	movq	%rax, %rcx
14229ecf3:     	callq	0x140839490 <.text+0x838490>
14229ecf8:     	testq	%rax, %rax
14229ecfb:     	je	0x14229ed20 <.text+0x229dd20>
14229ecfd:     	movzbl	0x10(%rax), %eax
14229ed01:     	movl	%eax, 0x1e8(%rbp)
14229ed07:     	movq	%r14, %rsi
14229ed0a:     	movzbl	(%rsi), %eax
14229ed0d:     	cmpb	$0xb, %al
14229ed0f:     	jne	0x14229ed40 <.text+0x229dd40>
14229ed11:     	jmp	0x14229f0a1 <.text+0x229e0a1>
14229ed16:     	nopw	%cs:(%rax,%rax)
14229ed20:     	movl	$0x0, 0x1e8(%rbp)
14229ed2a:     	movq	%r14, %rsi
14229ed2d:     	movzbl	(%rsi), %eax
14229ed30:     	cmpb	$0xb, %al
14229ed32:     	je	0x14229f0a1 <.text+0x229e0a1>
14229ed38:     	nopl	(%rax,%rax)
14229ed40:     	cmpb	$0x2, %al
14229ed42:     	jne	0x14229f0e8 <.text+0x229e0e8>
14229ed48:     	movq	0x8(%rsi), %rdx
14229ed4c:     	movq	0x1f8(%rbp), %rcx
14229ed53:     	callq	0x140a922b0 <.text+0xa912b0>
14229ed58:     	movq	%rax, %rcx
14229ed5b:     	shrq	$0x20, %rcx
14229ed5f:     	je	0x1422a0e9b <.text+0x229fe9b>
14229ed65:     	movl	%eax, %r8d
14229ed68:     	movq	0x1b0(%rbp), %rdx
14229ed6f:     	cmpq	0x10(%rdx), %r8
14229ed73:     	jae	0x1422a0e47 <.text+0x229fe47>
14229ed79:     	movq	0x8(%rdx), %rdx
14229ed7d:     	movq	%rax, %rcx
14229ed80:     	shrq	$0x20, %rcx
14229ed84:     	leaq	(%r8,%r8,4), %r9
14229ed88:     	cmpl	%ecx, (%rdx,%r9,4)
14229ed8c:     	jne	0x1422a0e47 <.text+0x229fe47>
14229ed92:     	leaq	(%rdx,%r9,4), %r9
14229ed96:     	movl	0x4(%r9), %edx
14229ed9a:     	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14229eda0:     	cmpq	%r10, %rdx
14229eda3:     	je	0x1422a0e47 <.text+0x229fe47>
14229eda9:     	xorl	%r8d, %r8d
14229edac:     	movq	0x88(%rbp), %r11
14229edb3:     	cmpq	%rdx, 0x38(%r11)
14229edb7:     	jbe	0x1422a0e57 <.text+0x229fe57>
14229edbd:     	movq	0x28(%r11), %rcx
14229edc1:     	movl	%edx, %r10d
14229edc4:     	shrl	$0x6, %r10d
14229edc8:     	movq	(%rcx,%r10,8), %rcx
14229edcc:     	btq	%rdx, %rcx
14229edd0:     	jae	0x1422a0e57 <.text+0x229fe57>
14229edd6:     	movq	%r14, %rsi
14229edd9:     	movl	0xc(%r9), %eax
14229eddd:     	movl	0x10(%r9), %ecx
14229ede1:     	movq	0x1b0(%rbp), %rdx
14229ede8:     	movq	0x1a8(%rdx), %rdx
14229edef:     	leaq	(%rax,%rax,8), %rax
14229edf3:     	movq	0x110(%r11), %r8
14229edfa:     	movq	0x18(%rdx,%rax,8), %r9
14229edff:     	movq	0x38(%rdx,%rax,8), %rax
14229ee04:     	movq	(%rax,%r8,8), %rax
14229ee08:     	notq	%rax
14229ee0b:     	leaq	(%rax,%rax,2), %rax
14229ee0f:     	shlq	$0x4, %rax
14229ee13:     	movq	0x10(%r9,%rax), %rax
14229ee18:     	imulq	$0x58, %rcx, %rcx
14229ee1c:     	movzbl	0x40(%rax,%rcx), %ebx
14229ee21:     	movl	0x48(%rax,%rcx), %edx
14229ee25:     	movl	%edx, 0x37(%rbp)
14229ee28:     	movq	0x41(%rax,%rcx), %rax
14229ee2d:     	movq	%rax, 0x30(%rbp)
14229ee31:     	movzbl	(%r14), %eax
14229ee35:     	cmpb	$0x6, %al
14229ee37:     	je	0x14229f0f2 <.text+0x229e0f2>
14229ee3d:     	xorl	%r12d, %r12d
14229ee40:     	cmpb	$0xa, %al
14229ee42:     	jne	0x14229f140 <.text+0x229e140>
14229ee48:     	movl	0x4(%rsi), %edx
14229ee4b:     	movq	0x200(%rbp), %rcx
14229ee52:     	callq	0x140a1c930 <.text+0xa1b930>
14229ee57:     	testq	%rax, %rax
14229ee5a:     	je	0x14229eec5 <.text+0x229dec5>
14229ee5c:     	addq	$0x30, %rax
14229ee60:     	movq	%rax, %rcx
14229ee63:     	leaq	0xd9dbbe(%rip), %rdx    # 0x14303ca28
14229ee6a:     	callq	0x1408392d0 <.text+0x8382d0>
14229ee6f:     	movzbl	0x4(%rax), %r15d
14229ee74:     	movq	%r14, %rsi
14229ee77:     	movzbl	(%rsi), %eax
14229ee7a:     	testb	%al, %al
14229ee7c:     	jne	0x14229f14e <.text+0x229e14e>
14229ee82:     	movq	0x8(%rsi), %rdx
14229ee86:     	movq	0x1f8(%rbp), %rcx
14229ee8d:     	leaq	0xd9d9b4(%rip), %r8     # 0x14303c848
14229ee94:     	callq	0x140a91ef0 <.text+0xa90ef0>
14229ee99:     	testq	%rax, %rax
14229ee9c:     	je	0x1422a0ece <.text+0x229fece>
14229eea2:     	movzbl	0x295(%rax), %r13d
14229eeaa:     	movzbl	(%r14), %eax
14229eeae:     	testb	%al, %al
14229eeb0:     	je	0x14229ef29 <.text+0x229df29>
14229eeb2:     	xorl	%esi, %esi
14229eeb4:     	movaps	%xmm14, %xmm11
14229eeb8:     	cmpb	$0x9, %al
14229eeba:     	je	0x14229f15b <.text+0x229e15b>
14229eec0:     	jmp	0x14229f18c <.text+0x229e18c>
14229eec5:     	movb	$0x1, %r15b
14229eec8:     	movq	%r14, %rsi
14229eecb:     	movzbl	(%rsi), %eax
14229eece:     	testb	%al, %al
14229eed0:     	jne	0x14229f14e <.text+0x229e14e>
14229eed6:     	jmp	0x14229ee82 <.text+0x229de82>
14229eed8:     	movq	0x8(%rsi), %rdx
14229eedc:     	movq	0x1f8(%rbp), %rcx
14229eee3:     	leaq	0xd9d8fe(%rip), %r8     # 0x14303c7e8
14229eeea:     	callq	0x140a91ef0 <.text+0xa90ef0>
14229eeef:     	testq	%rax, %rax
14229eef2:     	je	0x1422a0fcb <.text+0x229ffcb>
14229eef8:     	movss	0x290(%rax), %xmm9
14229ef01:     	movq	%r14, %rsi
14229ef04:     	movzbl	(%r14), %eax
14229ef08:     	movl	$0x0, 0x1f0(%rbp)
14229ef12:     	movl	$0x0, 0x1e8(%rbp)
14229ef1c:     	cmpb	$0xb, %al
14229ef1e:     	je	0x14229ecd8 <.text+0x229dcd8>
14229ef24:     	jmp	0x14229f096 <.text+0x229e096>
14229ef29:     	movq	0x8(%r14), %rdx
14229ef2d:     	movq	0x1f8(%rbp), %rcx
14229ef34:     	callq	0x140a922b0 <.text+0xa912b0>
14229ef39:     	movq	%rax, %rcx
14229ef3c:     	shrq	$0x20, %rcx
14229ef40:     	je	0x14229efe1 <.text+0x229dfe1>
14229ef46:     	movl	%eax, %eax
14229ef48:     	movq	0x128(%rbp), %rdx
14229ef4f:     	cmpq	0x10(%rdx), %rax
14229ef53:     	jae	0x14229efe1 <.text+0x229dfe1>
14229ef59:     	movq	0x8(%rdx), %rdx
14229ef5d:     	leaq	(%rax,%rax,4), %rax
14229ef61:     	cmpl	%ecx, (%rdx,%rax,4)
14229ef64:     	jne	0x14229efe1 <.text+0x229dfe1>
14229ef66:     	leaq	(%rdx,%rax,4), %rax
14229ef6a:     	movl	0x4(%rax), %ecx
14229ef6d:     	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
14229ef72:     	cmpq	%rdx, %rcx
14229ef75:     	je	0x14229efe1 <.text+0x229dfe1>
14229ef77:     	movq	0x108(%rbp), %r9
14229ef7e:     	cmpq	%rcx, 0x38(%r9)
14229ef82:     	jbe	0x14229efe1 <.text+0x229dfe1>
14229ef84:     	movq	0x28(%r9), %rdx
14229ef88:     	movl	%ecx, %r8d
14229ef8b:     	shrl	$0x6, %r8d
14229ef8f:     	movq	(%rdx,%r8,8), %rdx
14229ef93:     	btq	%rcx, %rdx
14229ef97:     	jae	0x14229efe1 <.text+0x229dfe1>
14229ef99:     	movl	0xc(%rax), %ecx
14229ef9c:     	movl	0x10(%rax), %eax
14229ef9f:     	movq	0x128(%rbp), %rdx
14229efa6:     	movq	0x1a8(%rdx), %rdx
14229efad:     	leaq	(%rcx,%rcx,8), %rcx
14229efb1:     	movq	0x110(%r9), %r8
14229efb8:     	movq	0x18(%rdx,%rcx,8), %r9
14229efbd:     	movq	0x38(%rdx,%rcx,8), %rcx
14229efc2:     	movq	(%rcx,%r8,8), %rcx
14229efc6:     	notq	%rcx
14229efc9:     	leaq	(%rcx,%rcx,2), %rcx
14229efcd:     	shlq	$0x4, %rcx
14229efd1:     	movq	0x10(%r9,%rcx), %rcx
14229efd6:     	imulq	$0x58, %rax, %rax
14229efda:     	movzbl	0x3d(%rcx,%rax), %esi
14229efdf:     	jmp	0x14229efe3 <.text+0x229dfe3>
14229efe1:     	xorl	%esi, %esi
14229efe3:     	movzbl	(%r14), %eax
14229efe7:     	movaps	%xmm14, %xmm11
14229efeb:     	cmpb	$0x9, %al
14229efed:     	je	0x14229f15b <.text+0x229e15b>
14229eff3:     	jmp	0x14229f18c <.text+0x229e18c>
14229eff8:     	movl	0x4(%r15), %edx
14229effc:     	movq	0x200(%rbp), %rcx
14229f003:     	callq	0x140a1c930 <.text+0xa1b930>
14229f008:     	testq	%rax, %rax
14229f00b:     	je	0x1422a1a72 <.text+0x22a0a72>
14229f011:     	addq	$0x30, %rax
14229f015:     	movq	%rax, %rcx
14229f018:     	leaq	0xd9d781(%rip), %rdx    # 0x14303c7a0
14229f01f:     	callq	0x1408394b0 <.text+0x8384b0>
14229f024:     	movl	0x8(%rax), %eax
14229f027:     	jmp	0x14229f058 <.text+0x229e058>
14229f029:     	movq	0x8(%r15), %rax
14229f02d:     	movq	%rax, 0x180(%rbp)
14229f034:     	leaq	0x180(%rbp), %rcx
14229f03b:     	callq	0x1400885d0 <.text+0x875d0>
14229f040:     	movq	0x28(%rbp), %rcx
14229f044:     	movq	0x10(%rcx), %rdx
14229f048:     	cmpq	%rdx, %rax
14229f04b:     	jae	0x1422a1a83 <.text+0x22a0a83>
14229f051:     	movq	0x8(%rcx), %rcx
14229f055:     	movl	(%rcx,%rax,4), %eax
14229f058:     	movl	%eax, 0x1d8(%rbp)
14229f05e:     	movq	%r14, %rsi
14229f061:     	movzbl	(%rsi), %eax
14229f064:     	xorps	%xmm9, %xmm9
14229f068:     	testb	%al, %al
14229f06a:     	je	0x14229ec80 <.text+0x229dc80>
14229f070:     	movl	$0x0, 0x1c0(%rbp)
14229f07a:     	movl	$0x0, 0x1f0(%rbp)
14229f084:     	movl	$0x0, 0x1e8(%rbp)
14229f08e:     	cmpb	$0xb, %al
14229f090:     	je	0x14229ecd8 <.text+0x229dcd8>
14229f096:     	movzbl	(%rsi), %eax
14229f099:     	cmpb	$0xb, %al
14229f09b:     	jne	0x14229ed40 <.text+0x229dd40>
14229f0a1:     	movl	0x4(%rsi), %edx
14229f0a4:     	movq	0x200(%rbp), %rcx
14229f0ab:     	callq	0x140a1c930 <.text+0xa1b930>
14229f0b0:     	movl	$0x0, 0x1f0(%rbp)
14229f0ba:     	testq	%rax, %rax
14229f0bd:     	je	0x14229f0d9 <.text+0x229e0d9>
14229f0bf:     	addq	$0x30, %rax
14229f0c3:     	movq	%rax, %rcx
14229f0c6:     	callq	0x140839490 <.text+0x838490>
14229f0cb:     	testq	%rax, %rax
14229f0ce:     	je	0x14229f0d9 <.text+0x229e0d9>
14229f0d0:     	movl	0xc(%rax), %eax
14229f0d3:     	movl	%eax, 0x1f0(%rbp)
14229f0d9:     	movq	%r14, %rsi
14229f0dc:     	movzbl	(%r14), %eax
14229f0e0:     	cmpb	$0x2, %al
14229f0e2:     	je	0x14229ed48 <.text+0x229dd48>
14229f0e8:     	xorl	%ebx, %ebx
14229f0ea:     	cmpb	$0x6, %al
14229f0ec:     	jne	0x14229ee3d <.text+0x229de3d>
14229f0f2:     	movl	0x4(%rsi), %edx
14229f0f5:     	movq	0x200(%rbp), %rcx
14229f0fc:     	callq	0x140a1c930 <.text+0xa1b930>
14229f101:     	testq	%rax, %rax
14229f104:     	je	0x14229f12e <.text+0x229e12e>
14229f106:     	addq	$0x30, %rax
14229f10a:     	movq	%rax, %rcx
14229f10d:     	leaq	0xd9d8fc(%rip), %rdx    # 0x14303ca10
14229f114:     	callq	0x140839340 <.text+0x838340>
14229f119:     	movzbl	(%rax), %r12d
14229f11d:     	movq	%r14, %rsi
14229f120:     	movzbl	(%r14), %eax
14229f124:     	cmpb	$0xa, %al
14229f126:     	je	0x14229ee48 <.text+0x229de48>
14229f12c:     	jmp	0x14229f140 <.text+0x229e140>
14229f12e:     	movb	$0x1, %r12b
14229f131:     	movq	%r14, %rsi
14229f134:     	movzbl	(%r14), %eax
14229f138:     	cmpb	$0xa, %al
14229f13a:     	je	0x14229ee48 <.text+0x229de48>
14229f140:     	xorl	%r15d, %r15d
14229f143:     	movzbl	(%rsi), %eax
14229f146:     	testb	%al, %al
14229f148:     	je	0x14229ee82 <.text+0x229de82>
14229f14e:     	xorl	%r13d, %r13d
14229f151:     	xorl	%esi, %esi
14229f153:     	movaps	%xmm14, %xmm11
14229f157:     	cmpb	$0x9, %al
14229f159:     	jne	0x14229f18c <.text+0x229e18c>
14229f15b:     	movl	0x4(%r14), %edx
14229f15f:     	movq	0x200(%rbp), %rcx
14229f166:     	callq	0x140a1c930 <.text+0xa1b930>
14229f16b:     	movaps	%xmm14, %xmm11
14229f16f:     	testq	%rax, %rax
14229f172:     	je	0x14229f18c <.text+0x229e18c>
14229f174:     	addq	$0x30, %rax
14229f178:     	movq	%rax, %rcx
14229f17b:     	leaq	0xd9d8be(%rip), %rdx    # 0x14303ca40
14229f182:     	callq	0x140839420 <.text+0x838420>
14229f187:     	movss	(%rax), %xmm11
14229f18c:     	movss	0x10(%r14), %xmm7
14229f192:     	movss	0x14(%r14), %xmm8
14229f198:     	leaq	0x158(%rbp), %rcx
14229f19f:     	leaq	0x110(%rbp), %rdx
14229f1a6:     	leaq	0xd9d6cb(%rip), %r8     # 0x14303c878
14229f1ad:     	callq	0x141c5a430 <.text+0x1c59430>
14229f1b2:     	movups	(%r14), %xmm0
14229f1b6:     	movaps	%xmm0, 0xb0(%rbp)
14229f1bd:     	movl	0x37(%rbp), %eax
14229f1c0:     	leaq	0x191(%rbp), %rcx
14229f1c7:     	movl	%eax, 0x7(%rcx)
14229f1ca:     	movq	0x30(%rbp), %rax
14229f1ce:     	movq	%rax, (%rcx)
14229f1d1:     	movl	0x1d8(%rbp), %eax
14229f1d7:     	movl	%eax, 0x180(%rbp)
14229f1dd:     	movl	0x1c0(%rbp), %eax
14229f1e3:     	movb	%al, 0x19d(%rbp)
14229f1e9:     	movb	%r12b, 0x19e(%rbp)
14229f1f0:     	movb	%r13b, 0x19f(%rbp)
14229f1f7:     	movb	%sil, 0x1a0(%rbp)
14229f1fe:     	movss	%xmm11, 0x184(%rbp)
14229f207:     	movb	%r15b, 0x1a1(%rbp)
14229f20e:     	movb	%bl, 0x190(%rbp)
14229f214:     	movss	%xmm9, 0x188(%rbp)
14229f21d:     	movl	0x1e8(%rbp), %eax
14229f223:     	movb	%al, 0x19c(%rbp)
14229f229:     	movl	0x1f0(%rbp), %eax
14229f22f:     	movl	%eax, 0x18c(%rbp)
14229f235:     	movb	$0x1, 0x177(%rbp)
14229f23c:     	leaq	0x98(%rbp), %rsi
14229f243:     	movq	%rsi, %rcx
14229f246:     	movq	0x1c8(%rbp), %rdx
14229f24d:     	callq	0x140911dd0 <.text+0x910dd0>
14229f252:     	leaq	0xd9cd97(%rip), %rax    # 0x14303bff0
14229f259:     	movss	(%rdi,%rax), %xmm11
14229f25f:     	movb	$0x0, 0x177(%rbp)
14229f266:     	movq	0x370(%rbp), %rax
14229f26d:     	movq	%rax, 0x60(%rsp)
14229f272:     	movq	%rsi, 0x58(%rsp)
14229f277:     	leaq	0x180(%rbp), %rax
14229f27e:     	movq	%rax, 0x50(%rsp)
14229f283:     	leaq	0xb0(%rbp), %rax
14229f28a:     	movq	%rax, 0x48(%rsp)
14229f28f:     	leaq	0x158(%rbp), %rax
14229f296:     	movq	%rax, 0x40(%rsp)
14229f29b:     	movss	%xmm11, 0x38(%rsp)
14229f2a2:     	movss	%xmm15, 0x30(%rsp)
14229f2a9:     	movss	%xmm10, 0x28(%rsp)
14229f2b0:     	movl	$0x0, 0x20(%rsp)
14229f2b8:     	leaq	0xd8(%rbp), %rcx
14229f2bf:     	movaps	%xmm7, %xmm1
14229f2c2:     	movaps	%xmm8, %xmm2
14229f2c6:     	movaps	%xmm6, %xmm3
14229f2c9:     	callq	0x1425f27c0 <.text+0x25f17c0>
14229f2ce:     	movq	0x140(%rbp), %rsi
14229f2d5:     	cmpq	0x130(%rbp), %rsi
14229f2dc:     	jne	0x14229f2f1 <.text+0x229e2f1>
14229f2de:     	leaq	0x130(%rbp), %rcx
14229f2e5:     	leaq	0xd9d5a4(%rip), %rdx    # 0x14303c890
14229f2ec:     	callq	0x140562c30 <.text+0x561c30>
14229f2f1:     	movq	0x138(%rbp), %rax
14229f2f8:     	movq	%rsi, %rcx
14229f2fb:     	shlq	$0x5, %rcx
14229f2ff:     	movdqu	0xd8(%rbp), %xmm0
14229f307:     	movups	0xe8(%rbp), %xmm1
14229f30e:     	movups	%xmm1, 0x10(%rax,%rcx)
14229f313:     	movdqu	%xmm0, (%rax,%rcx)
14229f318:     	incq	%rsi
14229f31b:     	movq	%rsi, 0x140(%rbp)
14229f322:     	movq	%r14, %rbx
14229f325:     	movzbl	(%r14), %eax
14229f329:     	testb	%al, %al
14229f32b:     	je	0x14229f3b0 <.text+0x229e3b0>
14229f331:     	cmpb	$0x3, %al
14229f333:     	movq	%rbx, 0x1f0(%rbp)
14229f33a:     	jne	0x14229f6b0 <.text+0x229e6b0>
14229f340:     	movl	0x4(%rbx), %r12d
14229f344:     	movq	0x358(%rbp), %rcx
14229f34b:     	callq	0x140b26a70 <.text+0xb25a70>
14229f350:     	movq	%rax, 0x98(%rbp)
14229f357:     	movq	%rdx, 0xa0(%rbp)
14229f35e:     	leaq	0x98(%rbp), %rcx
14229f365:     	movl	%r12d, %edx
14229f368:     	callq	0x140b26970 <.text+0xb25970>
14229f36d:     	cmpb	$0xa, %al
14229f36f:     	movq	0x1f0(%rbp), %rbx
14229f376:     	ja	0x14229f814 <.text+0x229e814>
14229f37c:     	movzbl	%al, %eax
14229f37f:     	movl	$0x630, %ecx            # imm = 0x630
14229f384:     	btl	%eax, %ecx
14229f387:     	jae	0x14229f814 <.text+0x229e814>
14229f38d:     	movzbl	(%rbx), %eax
14229f390:     	cmpb	$0xc, %al
14229f392:     	je	0x14229f6b8 <.text+0x229e6b8>
14229f398:     	cmpb	$0xb, %al
14229f39a:     	je	0x1422a0082 <.text+0x229f082>
14229f3a0:     	jmp	0x1422a0660 <.text+0x229f660>
14229f3a5:     	nopw	%cs:(%rax,%rax)
14229f3b0:     	movq	0x8(%rbx), %rdx
14229f3b4:     	movq	%rdx, 0x78(%rbp)
14229f3b8:     	movq	0x1f8(%rbp), %rcx
14229f3bf:     	leaq	0xd9d4e2(%rip), %r8     # 0x14303c8a8
14229f3c6:     	callq	0x140a91ef0 <.text+0xa90ef0>
14229f3cb:     	testq	%rax, %rax
14229f3ce:     	je	0x14229f4e0 <.text+0x229e4e0>
14229f3d4:     	addq	$0x299, %rax            # imm = 0x299
14229f3da:     	movq	%rax, %rcx
14229f3dd:     	callq	0x1408c5670 <.text+0x8c4670>
14229f3e2:     	testb	%al, %al
14229f3e4:     	je	0x14229f4e0 <.text+0x229e4e0>
14229f3ea:     	movq	0x78(%rbp), %rdx
14229f3ee:     	movq	0x1f8(%rbp), %rcx
14229f3f5:     	callq	0x140a922b0 <.text+0xa912b0>
14229f3fa:     	movq	%rax, %rcx
14229f3fd:     	shrq	$0x20, %rcx
14229f401:     	je	0x14229f4fa <.text+0x229e4fa>
14229f407:     	movl	%eax, %eax
14229f409:     	movq	0x128(%rbp), %rdx
14229f410:     	cmpq	0x10(%rdx), %rax
14229f414:     	jae	0x14229f4fa <.text+0x229e4fa>
14229f41a:     	movq	0x128(%rbp), %rdx
14229f421:     	movq	0x8(%rdx), %rdx
14229f425:     	leaq	(%rax,%rax,4), %rax
14229f429:     	cmpl	%ecx, (%rdx,%rax,4)
14229f42c:     	jne	0x14229f4fa <.text+0x229e4fa>
14229f432:     	leaq	(%rdx,%rax,4), %rax
14229f436:     	movl	0x4(%rax), %ecx
14229f439:     	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
14229f43e:     	cmpq	%rdx, %rcx
14229f441:     	je	0x14229f4fa <.text+0x229e4fa>
14229f447:     	movq	0x108(%rbp), %rdx
14229f44e:     	cmpq	%rcx, 0x38(%rdx)
14229f452:     	jbe	0x14229f4fa <.text+0x229e4fa>
14229f458:     	movq	0x108(%rbp), %rdx
14229f45f:     	movq	0x28(%rdx), %rdx
14229f463:     	movl	%ecx, %r8d
14229f466:     	shrl	$0x6, %r8d
14229f46a:     	movq	(%rdx,%r8,8), %rdx
14229f46e:     	btq	%rcx, %rdx
14229f472:     	jae	0x14229f4fa <.text+0x229e4fa>
14229f478:     	movl	0xc(%rax), %ecx
14229f47b:     	movl	0x10(%rax), %eax
14229f47e:     	movq	0x128(%rbp), %rdx
14229f485:     	movq	0x1a8(%rdx), %rdx
14229f48c:     	leaq	(%rcx,%rcx,8), %rcx
14229f490:     	movq	0x108(%rbp), %r8
14229f497:     	movq	0x110(%r8), %r8
14229f49e:     	movq	0x18(%rdx,%rcx,8), %r9
14229f4a3:     	movq	0x38(%rdx,%rcx,8), %rcx
14229f4a8:     	movq	(%rcx,%r8,8), %rcx
14229f4ac:     	notq	%rcx
14229f4af:     	leaq	(%rcx,%rcx,2), %rdx
14229f4b3:     	shlq	$0x4, %rdx
14229f4b7:     	imulq	$0x58, %rax, %rcx
14229f4bb:     	addq	0x10(%r9,%rdx), %rcx
14229f4c0:     	callq	0x1408e3380 <.text+0x8e2380>
14229f4c5:     	movsd	0x10(%r14), %xmm7
14229f4cb:     	testb	%al, %al
14229f4cd:     	je	0x14229f500 <.text+0x229e500>
14229f4cf:     	movss	0xd9cb24(%rip), %xmm9   # 0x14303bffc
14229f4d8:     	jmp	0x14229f509 <.text+0x229e509>
14229f4da:     	nopw	(%rax,%rax)
14229f4e0:     	movq	%r14, %rbx
14229f4e3:     	movzbl	(%rbx), %eax
14229f4e6:     	cmpb	$0x3, %al
14229f4e8:     	movq	%rbx, 0x1f0(%rbp)
14229f4ef:     	jne	0x14229f6b0 <.text+0x229e6b0>
14229f4f5:     	jmp	0x14229f340 <.text+0x229e340>
14229f4fa:     	movsd	0x10(%r14), %xmm7
14229f500:     	movss	0xd9caef(%rip), %xmm9   # 0x14303bff8
14229f509:     	movaps	%xmm10, %xmm8
14229f50d:     	mulss	0x7e9cfa(%rip), %xmm8   # 0x142a89210
14229f516:     	movaps	%xmm10, %xmm12
14229f51a:     	mulss	0x777795(%rip), %xmm12  # 0x142a16cb8
14229f523:     	leaq	0x158(%rbp), %rcx
14229f52a:     	callq	0x1422acce0 <.text+0x22abce0>
14229f52f:     	movq	0x78(%rbp), %rax
14229f533:     	movq	%rax, 0xb8(%rbp)
14229f53a:     	movb	$0x7, 0xb0(%rbp)
14229f541:     	movb	$0x1, 0x1bf(%rbp)
14229f548:     	leaq	0x78(%rbp), %rcx
14229f54c:     	callq	0x1400885d0 <.text+0x875d0>
14229f551:     	movq	0x90(%rbp), %rcx
14229f558:     	leaq	0x180(%rbp), %rdi
14229f55f:     	movq	0x10(%rcx), %rdx
14229f563:     	cmpq	%rdx, %rax
14229f566:     	jae	0x1422a0feb <.text+0x229ffeb>
14229f56c:     	movq	%r14, %rbx
14229f56f:     	movq	0x8(%rcx), %rcx
14229f573:     	movl	(%rcx,%rax,4), %esi
14229f576:     	movb	$0x1, 0x1bf(%rbp)
14229f57d:     	callq	0x140b06780 <.text+0xb05780>
14229f582:     	movl	%esi, 0x180(%rbp)
14229f588:     	movl	$0x0, 0x19d(%rbp)
14229f592:     	movd	%xmm0, 0x184(%rbp)
14229f59a:     	movb	$0x0, 0x1a1(%rbp)
14229f5a1:     	movb	$0x0, 0x19c(%rbp)
14229f5a8:     	leaq	0x191(%rbp), %rax
14229f5af:     	movb	$0x0, -0x1(%rax)
14229f5b3:     	movq	$0x0, -0x9(%rax)
14229f5bb:     	movb	$0x1, 0x1bf(%rbp)
14229f5c2:     	leaq	0x98(%rbp), %rcx
14229f5c9:     	movq	0x1c8(%rbp), %rdx
14229f5d0:     	callq	0x140911dd0 <.text+0x910dd0>
14229f5d5:     	addss	%xmm6, %xmm8
14229f5da:     	movshdup	%xmm7, %xmm2            # xmm2 = xmm7[1,1,3,3]
14229f5de:     	movb	$0x0, 0x1bf(%rbp)
14229f5e5:     	movq	0x370(%rbp), %rax
14229f5ec:     	movq	%rax, 0x60(%rsp)
14229f5f1:     	leaq	0x98(%rbp), %rax
14229f5f8:     	movq	%rax, 0x58(%rsp)
14229f5fd:     	movq	%rdi, 0x50(%rsp)
14229f602:     	leaq	0xb0(%rbp), %rax
14229f609:     	movq	%rax, 0x48(%rsp)
14229f60e:     	leaq	0x158(%rbp), %rax
14229f615:     	movq	%rax, 0x40(%rsp)
14229f61a:     	movss	%xmm11, 0x38(%rsp)
14229f621:     	movss	%xmm15, 0x30(%rsp)
14229f628:     	movss	%xmm12, 0x28(%rsp)
14229f62f:     	movss	%xmm9, 0x20(%rsp)
14229f636:     	leaq	0xd8(%rbp), %rcx
14229f63d:     	movaps	%xmm7, %xmm1
14229f640:     	movaps	%xmm8, %xmm3
14229f644:     	callq	0x1425f27c0 <.text+0x25f17c0>
14229f649:     	movq	0x140(%rbp), %rsi
14229f650:     	cmpq	0x130(%rbp), %rsi
14229f657:     	jne	0x14229f66c <.text+0x229e66c>
14229f659:     	leaq	0x130(%rbp), %rcx
14229f660:     	leaq	0xd9d271(%rip), %rdx    # 0x14303c8d8
14229f667:     	callq	0x140562c30 <.text+0x561c30>
14229f66c:     	movq	0x138(%rbp), %rax
14229f673:     	movq	%rsi, %rcx
14229f676:     	shlq	$0x5, %rcx
14229f67a:     	movdqu	0xd8(%rbp), %xmm0
14229f682:     	movups	0xe8(%rbp), %xmm1
14229f689:     	movups	%xmm1, 0x10(%rax,%rcx)
14229f68e:     	movdqu	%xmm0, (%rax,%rcx)
14229f693:     	incq	%rsi
14229f696:     	movq	%rsi, 0x140(%rbp)
14229f69d:     	movzbl	(%rbx), %eax
14229f6a0:     	cmpb	$0x3, %al
14229f6a2:     	movq	%rbx, 0x1f0(%rbp)
14229f6a9:     	je	0x14229f340 <.text+0x229e340>
14229f6af:     	nop
14229f6b0:     	cmpb	$0xc, %al
14229f6b2:     	jne	0x14229f398 <.text+0x229e398>
14229f6b8:     	movl	0x4(%rbx), %esi
14229f6bb:     	movq	0x200(%rbp), %rcx
14229f6c2:     	movl	%esi, %edx
14229f6c4:     	callq	0x140a1c930 <.text+0xa1b930>
14229f6c9:     	movq	%rax, %rdi
14229f6cc:     	testq	%rax, %rax
14229f6cf:     	je	0x1422a0070 <.text+0x229f070>
14229f6d5:     	leaq	0x30(%rdi), %rcx
14229f6d9:     	callq	0x140839490 <.text+0x838490>
14229f6de:     	testq	%rax, %rax
14229f6e1:     	je	0x1422a0070 <.text+0x229f070>
14229f6e7:     	movl	0x28(%rdi), %ecx
14229f6ea:     	movl	%ecx, 0x150(%rbp)
14229f6f0:     	movl	0xc(%rax), %edi
14229f6f3:     	movzbl	0x10(%rax), %eax
14229f6f7:     	movb	%al, 0x1c0(%rbp)
14229f6fd:     	leaq	0x180(%rbp), %rcx
14229f704:     	movq	0x58(%rbp), %rdx
14229f708:     	movl	%esi, %r8d
14229f70b:     	callq	0x1408fe8c0 <.text+0x8fd8c0>
14229f710:     	movq	0x188(%rbp), %rcx
14229f717:     	movq	0x190(%rbp), %rdx
14229f71e:     	testq	%rdx, %rdx
14229f721:     	movl	%esi, 0x1d8(%rbp)
14229f727:     	movl	%edi, 0x148(%rbp)
14229f72d:     	je	0x14229fc06 <.text+0x229ec06>
14229f733:     	movq	0x60(%rbp), %r8
14229f737:     	movq	0x18(%r8), %rax
14229f73b:     	testq	%rax, %rax
14229f73e:     	je	0x14229fbf1 <.text+0x229ebf1>
14229f744:     	leaq	(%rcx,%rdx,4), %rdx
14229f748:     	movq	(%r8), %rbx
14229f74b:     	movdqa	(%rbx), %xmm0
14229f74f:     	pmovmskb	%xmm0, %r9d
14229f754:     	notl	%r9d
14229f757:     	leaq	0x10(%rbx), %rsi
14229f75b:     	movl	$0x0, 0x1e8(%rbp)
14229f765:     	movq	%rcx, %r11
14229f768:     	jmp	0x14229f783 <.text+0x229e783>
14229f76a:     	nopw	(%rax,%rax)
14229f770:     	movb	$0x1, %r8b
14229f773:     	movl	%r8d, 0x1e8(%rbp)
14229f77a:     	cmpq	%rdx, %r11
14229f77d:     	je	0x14229fc10 <.text+0x229ec10>
14229f783:     	movl	(%r11), %r15d
14229f786:     	addq	$0x4, %r11
14229f78a:     	movq	%rax, %r13
14229f78d:     	movl	%r9d, %edi
14229f790:     	movq	%rsi, %r10
14229f793:     	movq	%rbx, %r8
14229f796:     	jmp	0x14229f7ae <.text+0x229e7ae>
14229f798:     	nopl	(%rax,%rax)
14229f7a0:     	cmpb	$0x0, -0x80(%rdi)
14229f7a4:     	jne	0x14229f770 <.text+0x229e770>
14229f7a6:     	movl	%r12d, %edi
14229f7a9:     	decq	%r13
14229f7ac:     	je	0x14229f77a <.text+0x229e77a>
14229f7ae:     	testw	%di, %di
14229f7b1:     	je	0x14229f7c0 <.text+0x229e7c0>
14229f7b3:     	leal	-0x1(%rdi), %r12d
14229f7b7:     	andl	%edi, %r12d
14229f7ba:     	jmp	0x14229f7ed <.text+0x229e7ed>
14229f7bc:     	nopl	(%rax)
14229f7c0:     	movdqa	(%r10), %xmm0
14229f7c5:     	pmovmskb	%xmm0, %edi
14229f7c9:     	addq	$-0x1100, %r8           # imm = 0xEF00
14229f7d0:     	addq	$0x10, %r10
14229f7d4:     	cmpl	$0xffff, %edi           # imm = 0xFFFF
14229f7da:     	je	0x14229f7c0 <.text+0x229e7c0>
14229f7dc:     	movl	$0xfffffffe, %r14d      # imm = 0xFFFFFFFE
14229f7e2:     	subl	%edi, %r14d
14229f7e5:     	notl	%edi
14229f7e7:     	movl	%edi, %r12d
14229f7ea:     	andl	%r14d, %r12d
14229f7ed:     	tzcntl	%edi, %edi
14229f7f1:     	negq	%rdi
14229f7f4:     	imulq	$0x110, %rdi, %r14      # imm = 0x110
14229f7fb:     	leaq	(%r8,%r14), %rdi
14229f7ff:     	cmpl	%r15d, -0x110(%r8,%r14)
14229f807:     	je	0x14229f7a0 <.text+0x229e7a0>
14229f809:     	cmpl	%r15d, -0x10c(%rdi)
14229f810:     	je	0x14229f7a0 <.text+0x229e7a0>
14229f812:     	jmp	0x14229f7a6 <.text+0x229e7a6>
14229f814:     	movss	0x10(%rbx), %xmm7
14229f819:     	movss	0x14(%rbx), %xmm8
14229f81f:     	movaps	%xmm10, %xmm9
14229f823:     	mulss	0x7e99e4(%rip), %xmm9   # 0x142a89210
14229f82c:     	movaps	%xmm10, %xmm12
14229f830:     	mulss	0x77747f(%rip), %xmm12  # 0x142a16cb8
14229f839:     	movzbl	0x13953c9(%rip), %eax   # 0x143634c09
14229f840:     	movl	$0xc8, %ecx
14229f845:     	movl	$0x8, %edx
14229f84a:     	callq	0x140613c10 <.text+0x612c10>
14229f84f:     	testq	%rax, %rax
14229f852:     	je	0x1422a1006 <.text+0x22a0006>
14229f858:     	movq	%rax, %rsi
14229f85b:     	movzbl	0x13953a7(%rip), %eax   # 0x143634c09
14229f862:     	movl	$0x16, %ecx
14229f867:     	movl	$0x1, %edx
14229f86c:     	callq	0x140613c10 <.text+0x612c10>
14229f871:     	testq	%rax, %rax
14229f874:     	je	0x1422a10d7 <.text+0x22a00d7>
14229f87a:     	movq	%rax, %r14
14229f87d:     	movups	0xd9cdac(%rip), %xmm0   # 0x14303c630
14229f884:     	movups	%xmm0, (%rax)
14229f887:     	movabsq	$0x776f6c6c65795f67, %rax # imm = 0x776F6C6C65795F67
14229f891:     	movq	%rax, 0xe(%r14)
14229f895:     	movzbl	0x139536d(%rip), %eax   # 0x143634c09
14229f89c:     	movl	$0x16, %ecx
14229f8a1:     	movl	$0x1, %edx
14229f8a6:     	callq	0x140613c10 <.text+0x612c10>
14229f8ab:     	testq	%rax, %rax
14229f8ae:     	je	0x1422a10f9 <.text+0x22a00f9>
14229f8b4:     	movq	%rax, %r15
14229f8b7:     	movups	0xd9cd88(%rip), %xmm0   # 0x14303c646
14229f8be:     	movups	%xmm0, (%rax)
14229f8c1:     	movabsq	$0x6465722f73776f64, %rax # imm = 0x6465722F73776F64
14229f8cb:     	movq	%rax, 0xe(%r15)
14229f8cf:     	movzbl	0x1395333(%rip), %eax   # 0x143634c09
14229f8d6:     	movl	$0x19, %ecx
14229f8db:     	movl	$0x1, %edx
14229f8e0:     	callq	0x140613c10 <.text+0x612c10>
14229f8e5:     	testq	%rax, %rax
14229f8e8:     	je	0x1422a1032 <.text+0x22a0032>
14229f8ee:     	movq	%rax, %r13
14229f8f1:     	movups	0xd9cb24(%rip), %xmm0   # 0x14303c41c
14229f8f8:     	movups	%xmm0, 0x9(%rax)
14229f8fc:     	movups	0xd9cb10(%rip), %xmm0   # 0x14303c413
14229f903:     	movups	%xmm0, (%rax)
14229f906:     	movzbl	0x13952fc(%rip), %eax   # 0x143634c09
14229f90d:     	movl	$0x17, %ecx
14229f912:     	movl	$0x1, %edx
14229f917:     	callq	0x140613c10 <.text+0x612c10>
14229f91c:     	testq	%rax, %rax
14229f91f:     	je	0x1422a1062 <.text+0x22a0062>
14229f925:     	movq	%rax, %rdi
14229f928:     	movups	0xd9cd2d(%rip), %xmm0   # 0x14303c65c
14229f92f:     	movups	%xmm0, (%rax)
14229f932:     	movabsq	$0x65756c622f73776f, %rax # imm = 0x65756C622F73776F
14229f93c:     	movq	%rax, 0xf(%rdi)
14229f940:     	movzbl	0x13952c2(%rip), %eax   # 0x143634c09
14229f947:     	movl	$0x1b, %ecx
14229f94c:     	movl	$0x1, %edx
14229f951:     	callq	0x140613c10 <.text+0x612c10>
14229f956:     	testq	%rax, %rax
14229f959:     	je	0x1422a1099 <.text+0x22a0099>
14229f95f:     	movups	0xd9c9cb(%rip), %xmm0   # 0x14303c331
14229f966:     	movups	%xmm0, 0xb(%rax)
14229f96a:     	movups	0xd9c9b5(%rip), %xmm0   # 0x14303c326
14229f971:     	movups	%xmm0, (%rax)
14229f974:     	movq	$0x16, (%rsi)
14229f97b:     	movq	%r14, 0x8(%rsi)
14229f97f:     	movq	$0x16, 0x10(%rsi)
14229f987:     	movb	$0x3, 0x18(%rsi)
14229f98b:     	movl	$0x0, 0x1c(%rsi)
14229f992:     	movl	$0x0, 0x24(%rsi)
14229f999:     	movq	$0x16, 0x28(%rsi)
14229f9a1:     	movq	%r15, 0x30(%rsi)
14229f9a5:     	movq	$0x16, 0x38(%rsi)
14229f9ad:     	movb	$0x3, 0x40(%rsi)
14229f9b1:     	movl	$0x4, 0x44(%rsi)
14229f9b8:     	movl	$0x0, 0x4c(%rsi)
14229f9bf:     	movq	$0x19, 0x50(%rsi)
14229f9c7:     	movq	%r13, 0x58(%rsi)
14229f9cb:     	movq	$0x19, 0x60(%rsi)
14229f9d3:     	movb	$0x3, 0x68(%rsi)
14229f9d7:     	movl	$0x3, 0x6c(%rsi)
14229f9de:     	movl	$0x0, 0x74(%rsi)
14229f9e5:     	movq	$0x17, 0x78(%rsi)
14229f9ed:     	movq	%rdi, 0x80(%rsi)
14229f9f4:     	movq	$0x17, 0x88(%rsi)
14229f9ff:     	movb	$0x3, 0x90(%rsi)
14229fa06:     	movl	$0x1, 0x94(%rsi)
14229fa10:     	movl	$0x0, 0x9c(%rsi)
14229fa1a:     	movq	$0x1b, 0xa0(%rsi)
14229fa25:     	movq	%rax, 0xa8(%rsi)
14229fa2c:     	movq	$0x1b, 0xb0(%rsi)
14229fa37:     	movb	$0x3, 0xb8(%rsi)
14229fa3e:     	movl	$0x2, 0xbc(%rsi)
14229fa48:     	movl	$0x0, 0xc4(%rsi)
14229fa52:     	movq	$0x5, 0x158(%rbp)
14229fa5d:     	movq	%rsi, 0x160(%rbp)
14229fa64:     	movq	$0x5, 0x168(%rbp)
14229fa6f:     	movl	%r12d, 0xb4(%rbp)
14229fa76:     	movb	$0x4, 0xb0(%rbp)
14229fa7d:     	movb	$0x1, 0x1d7(%rbp)
14229fa84:     	movq	0x200(%rbp), %rcx
14229fa8b:     	movl	%r12d, %edx
14229fa8e:     	callq	0x140a1c930 <.text+0xa1b930>
14229fa93:     	testq	%rax, %rax
14229fa96:     	je	0x1422a101a <.text+0x22a001a>
14229fa9c:     	addq	$0x30, %rax
14229faa0:     	movb	$0x1, 0x1d7(%rbp)
14229faa7:     	movq	%rax, %rcx
14229faaa:     	leaq	0xd9ce57(%rip), %rdx    # 0x14303c908
14229fab1:     	callq	0x1408393b0 <.text+0x8383b0>
14229fab6:     	movl	(%rax), %esi
14229fab8:     	movb	$0x1, 0x1d7(%rbp)
14229fabf:     	callq	0x140b06780 <.text+0xb05780>
14229fac4:     	movl	%esi, 0x180(%rbp)
14229faca:     	movl	$0x0, 0x19d(%rbp)
14229fad4:     	movss	%xmm0, 0x184(%rbp)
14229fadc:     	movb	$0x0, 0x1a1(%rbp)
14229fae3:     	movb	$0x0, 0x19c(%rbp)
14229faea:     	leaq	0x191(%rbp), %rax
14229faf1:     	movb	$0x0, -0x1(%rax)
14229faf5:     	movq	$0x0, -0x9(%rax)
14229fafd:     	movb	$0x1, 0x1d7(%rbp)
14229fb04:     	leaq	-0xc(%rbp), %rcx
14229fb08:     	movq	0x1c8(%rbp), %rdx
14229fb0f:     	callq	0x140911dd0 <.text+0x910dd0>
14229fb14:     	addss	%xmm6, %xmm9
14229fb19:     	movb	$0x0, 0x1d7(%rbp)
14229fb20:     	movq	0x370(%rbp), %rax
14229fb27:     	movq	%rax, 0x60(%rsp)
14229fb2c:     	leaq	-0xc(%rbp), %rax
14229fb30:     	movq	%rax, 0x58(%rsp)
14229fb35:     	leaq	0x180(%rbp), %rax
14229fb3c:     	movq	%rax, 0x50(%rsp)
14229fb41:     	leaq	0xb0(%rbp), %rax
14229fb48:     	movq	%rax, 0x48(%rsp)
14229fb4d:     	leaq	0x158(%rbp), %rax
14229fb54:     	movq	%rax, 0x40(%rsp)
14229fb59:     	movss	%xmm11, 0x38(%rsp)
14229fb60:     	movss	%xmm15, 0x30(%rsp)
14229fb67:     	movss	%xmm12, 0x28(%rsp)
14229fb6e:     	movl	$0x3ec51eb7, 0x20(%rsp) # imm = 0x3EC51EB7
14229fb76:     	leaq	0xd8(%rbp), %rcx
14229fb7d:     	movaps	%xmm7, %xmm1
14229fb80:     	movaps	%xmm8, %xmm2
14229fb84:     	movaps	%xmm9, %xmm3
14229fb88:     	callq	0x1425f27c0 <.text+0x25f17c0>
14229fb8d:     	movq	0x140(%rbp), %rsi
14229fb94:     	cmpq	0x130(%rbp), %rsi
14229fb9b:     	jne	0x14229fbb0 <.text+0x229ebb0>
14229fb9d:     	leaq	0x130(%rbp), %rcx
14229fba4:     	leaq	0xd9cd75(%rip), %rdx    # 0x14303c920
14229fbab:     	callq	0x140562c30 <.text+0x561c30>
14229fbb0:     	movq	0x138(%rbp), %rax
14229fbb7:     	movq	%rsi, %rcx
14229fbba:     	shlq	$0x5, %rcx
14229fbbe:     	movdqu	0xd8(%rbp), %xmm0
14229fbc6:     	movups	0xe8(%rbp), %xmm1
14229fbcd:     	movups	%xmm1, 0x10(%rax,%rcx)
14229fbd2:     	movdqu	%xmm0, (%rax,%rcx)
14229fbd7:     	incq	%rsi
14229fbda:     	movq	%rsi, 0x140(%rbp)
14229fbe1:     	movzbl	(%rbx), %eax
14229fbe4:     	cmpb	$0xc, %al
14229fbe6:     	jne	0x14229f398 <.text+0x229e398>
14229fbec:     	jmp	0x14229f6b8 <.text+0x229e6b8>
14229fbf1:     	shlq	$0x2, %rdx
14229fbf5:     	nopw	%cs:(%rax,%rax)
14229fc00:     	addq	$-0x4, %rdx
14229fc04:     	jne	0x14229fc00 <.text+0x229ec00>
14229fc06:     	movl	$0x0, 0x1e8(%rbp)
14229fc10:     	movq	0x180(%rbp), %rdx
14229fc17:     	testq	%rdx, %rdx
14229fc1a:     	je	0x14229fc2b <.text+0x229ec2b>
14229fc1c:     	shlq	$0x2, %rdx
14229fc20:     	movl	$0x4, %r8d
14229fc26:     	callq	0x140613c20 <.text+0x612c20>
14229fc2b:     	testb	$0x1, 0x1e8(%rbp)
14229fc32:     	movl	0x1d8(%rbp), %edi
14229fc38:     	je	0x1422a0070 <.text+0x229f070>
14229fc3e:     	movq	0x1f0(%rbp), %rax
14229fc45:     	movss	0x10(%rax), %xmm7
14229fc4a:     	movss	0x14(%rax), %xmm8
14229fc50:     	movaps	%xmm10, %xmm9
14229fc54:     	mulss	0x7e95b3(%rip), %xmm9   # 0x142a89210
14229fc5d:     	movaps	%xmm10, %xmm12
14229fc61:     	mulss	0x77704e(%rip), %xmm12  # 0x142a16cb8
14229fc6a:     	movzbl	0x1394f98(%rip), %eax   # 0x143634c09
14229fc71:     	movl	$0xf0, %ecx
14229fc76:     	movl	$0x8, %edx
14229fc7b:     	callq	0x140613c10 <.text+0x612c10>
14229fc80:     	testq	%rax, %rax
14229fc83:     	je	0x1422a119e <.text+0x22a019e>
14229fc89:     	movq	%rax, %rbx
14229fc8c:     	movzbl	0x1394f76(%rip), %eax   # 0x143634c09
14229fc93:     	movl	$0x1b, %ecx
14229fc98:     	movl	$0x1, %edx
14229fc9d:     	callq	0x140613c10 <.text+0x612c10>
14229fca2:     	testq	%rax, %rax
14229fca5:     	je	0x1422a1249 <.text+0x22a0249>
14229fcab:     	movq	%rax, %r14
14229fcae:     	movups	0xd9c53e(%rip), %xmm0   # 0x14303c1f3
14229fcb5:     	movups	%xmm0, 0xb(%rax)
14229fcb9:     	movdqu	0xd9c527(%rip), %xmm0   # 0x14303c1e8
14229fcc1:     	movdqu	%xmm0, (%rax)
14229fcc5:     	movzbl	0x1394f3d(%rip), %eax   # 0x143634c09
14229fccc:     	movl	$0x17, %ecx
14229fcd1:     	movl	$0x1, %edx
14229fcd6:     	callq	0x140613c10 <.text+0x612c10>
14229fcdb:     	testq	%rax, %rax
14229fcde:     	je	0x1422a1220 <.text+0x22a0220>
14229fce4:     	movq	%rax, %r15
14229fce7:     	movdqu	0xd9c96d(%rip), %xmm0   # 0x14303c65c
14229fcef:     	movdqu	%xmm0, (%rax)
14229fcf3:     	movabsq	$0x65756c622f73776f, %rax # imm = 0x65756C622F73776F
14229fcfd:     	movq	%rax, 0xf(%r15)
14229fd01:     	movzbl	0x1394f01(%rip), %eax   # 0x143634c09
14229fd08:     	movl	$0x19, %ecx
14229fd0d:     	movl	$0x1, %edx
14229fd12:     	callq	0x140613c10 <.text+0x612c10>
14229fd17:     	testq	%rax, %rax
14229fd1a:     	je	0x1422a11f0 <.text+0x22a01f0>
14229fd20:     	movq	%rax, %r12
14229fd23:     	movups	0xd9c6f2(%rip), %xmm0   # 0x14303c41c
14229fd2a:     	movups	%xmm0, 0x9(%rax)
14229fd2e:     	movdqu	0xd9c6dd(%rip), %xmm0   # 0x14303c413
14229fd36:     	movdqu	%xmm0, (%rax)
14229fd3a:     	movzbl	0x1394ec8(%rip), %eax   # 0x143634c09
14229fd41:     	movl	$0x15, %ecx
14229fd46:     	movl	$0x1, %edx
14229fd4b:     	callq	0x140613c10 <.text+0x612c10>
14229fd50:     	testq	%rax, %rax
14229fd53:     	je	0x1422a1122 <.text+0x22a0122>
14229fd59:     	movq	%rax, %r13
14229fd5c:     	movdqu	0xd9c53d(%rip), %xmm0   # 0x14303c2a1
14229fd64:     	movdqu	%xmm0, (%rax)
14229fd68:     	movabsq	$0x657268636f5f666f, %rax # imm = 0x657268636F5F666F
14229fd72:     	movq	%rax, 0xd(%r13)
14229fd76:     	movzbl	0x1394e8c(%rip), %eax   # 0x143634c09
14229fd7d:     	movl	$0x1b, %ecx
14229fd82:     	movl	$0x1, %edx
14229fd87:     	callq	0x140613c10 <.text+0x612c10>
14229fd8c:     	testq	%rax, %rax
14229fd8f:     	je	0x1422a11b2 <.text+0x22a01b2>
14229fd95:     	movups	0xd9c595(%rip), %xmm0   # 0x14303c331
14229fd9c:     	movups	%xmm0, 0xb(%rax)
14229fda0:     	movdqu	0xd9c57e(%rip), %xmm0   # 0x14303c326
14229fda8:     	movdqu	%xmm0, (%rax)
14229fdac:     	movq	%rax, %rsi
14229fdaf:     	movzbl	0x1394e53(%rip), %eax   # 0x143634c09
14229fdb6:     	movl	$0x14, %ecx
14229fdbb:     	movl	$0x1, %edx
14229fdc0:     	callq	0x140613c10 <.text+0x612c10>
14229fdc5:     	testq	%rax, %rax
14229fdc8:     	je	0x1422a1159 <.text+0x22a0159>
14229fdce:     	movups	0xd9c59a(%rip), %xmm0   # 0x14303c36f
14229fdd5:     	movups	%xmm0, (%rax)
14229fdd8:     	movl	$0x6b726164, 0x10(%rax) # imm = 0x6B726164
14229fddf:     	movq	$0x1b, (%rbx)
14229fde6:     	movq	%r14, 0x8(%rbx)
14229fdea:     	movq	$0x1b, 0x10(%rbx)
14229fdf2:     	movb	$0xe, 0x18(%rbx)
14229fdf6:     	movl	$0x0, 0x1c(%rbx)
14229fdfd:     	movl	$0x0, 0x24(%rbx)
14229fe04:     	movq	$0x17, 0x28(%rbx)
14229fe0c:     	movq	%r15, 0x30(%rbx)
14229fe10:     	movq	$0x17, 0x38(%rbx)
14229fe18:     	movb	$0xe, 0x40(%rbx)
14229fe1c:     	movl	$0x1, 0x44(%rbx)
14229fe23:     	movl	$0x0, 0x4c(%rbx)
14229fe2a:     	movq	$0x19, 0x50(%rbx)
14229fe32:     	movq	%r12, 0x58(%rbx)
14229fe36:     	movq	$0x19, 0x60(%rbx)
14229fe3e:     	movb	$0xe, 0x68(%rbx)
14229fe42:     	movl	$0x2, 0x6c(%rbx)
14229fe49:     	movl	$0x0, 0x74(%rbx)
14229fe50:     	movq	$0x15, 0x78(%rbx)
14229fe58:     	movq	%r13, 0x80(%rbx)
14229fe5f:     	movq	$0x15, 0x88(%rbx)
14229fe6a:     	movb	$0xe, 0x90(%rbx)
14229fe71:     	movl	$0x3, 0x94(%rbx)
14229fe7b:     	movl	$0x0, 0x9c(%rbx)
14229fe85:     	movq	$0x1b, 0xa0(%rbx)
14229fe90:     	movq	%rsi, 0xa8(%rbx)
14229fe97:     	movq	$0x1b, 0xb0(%rbx)
14229fea2:     	movb	$0xe, 0xb8(%rbx)
14229fea9:     	movl	$0x4, 0xbc(%rbx)
14229feb3:     	movl	$0x0, 0xc4(%rbx)
14229febd:     	movq	$0x14, 0xc8(%rbx)
14229fec8:     	movq	%rax, 0xd0(%rbx)
14229fecf:     	movq	$0x14, 0xd8(%rbx)
14229feda:     	movb	$0xe, 0xe0(%rbx)
14229fee1:     	movl	$0x5, 0xe4(%rbx)
14229feeb:     	movl	$0x0, 0xec(%rbx)
14229fef5:     	movq	$0x6, 0x158(%rbp)
14229ff00:     	movq	%rbx, 0x160(%rbp)
14229ff07:     	movq	$0x6, 0x168(%rbp)
14229ff12:     	movl	%edi, 0xb4(%rbp)
14229ff18:     	movb	$0xb, 0xb0(%rbp)
14229ff1f:     	movb	$0x1, 0x1af(%rbp)
14229ff26:     	callq	0x140b06780 <.text+0xb05780>
14229ff2b:     	movl	0x150(%rbp), %eax
14229ff31:     	movl	%eax, 0x180(%rbp)
14229ff37:     	movl	$0x0, 0x19d(%rbp)
14229ff41:     	movss	%xmm0, 0x184(%rbp)
14229ff49:     	movb	$0x0, 0x1a1(%rbp)
14229ff50:     	movb	$0x0, 0x190(%rbp)
14229ff57:     	movl	$0x0, 0x188(%rbp)
14229ff61:     	movzbl	0x1c0(%rbp), %eax
14229ff68:     	movb	%al, 0x19c(%rbp)
14229ff6e:     	movl	0x148(%rbp), %eax
14229ff74:     	movl	%eax, 0x18c(%rbp)
14229ff7a:     	movb	$0x1, 0x1af(%rbp)
14229ff81:     	leaq	0x98(%rbp), %rcx
14229ff88:     	movq	0x1c8(%rbp), %rdx
14229ff8f:     	callq	0x140911dd0 <.text+0x910dd0>
14229ff94:     	addss	%xmm6, %xmm9
14229ff99:     	movb	$0x0, 0x1af(%rbp)
14229ffa0:     	movq	0x370(%rbp), %rax
14229ffa7:     	movq	%rax, 0x60(%rsp)
14229ffac:     	leaq	0x98(%rbp), %rax
14229ffb3:     	movq	%rax, 0x58(%rsp)
14229ffb8:     	leaq	0x180(%rbp), %rax
14229ffbf:     	movq	%rax, 0x50(%rsp)
14229ffc4:     	leaq	0xb0(%rbp), %rax
14229ffcb:     	movq	%rax, 0x48(%rsp)
14229ffd0:     	leaq	0x158(%rbp), %rax
14229ffd7:     	movq	%rax, 0x40(%rsp)
14229ffdc:     	movss	%xmm11, 0x38(%rsp)
14229ffe3:     	movss	%xmm15, 0x30(%rsp)
14229ffea:     	movss	%xmm12, 0x28(%rsp)
14229fff1:     	movl	$0x3ec51eb7, 0x20(%rsp) # imm = 0x3EC51EB7
14229fff9:     	leaq	0xd8(%rbp), %rcx
1422a0000:     	movaps	%xmm7, %xmm1
1422a0003:     	movaps	%xmm8, %xmm2
1422a0007:     	movaps	%xmm9, %xmm3
1422a000b:     	callq	0x1425f27c0 <.text+0x25f17c0>
1422a0010:     	movq	0x140(%rbp), %rsi
1422a0017:     	cmpq	0x130(%rbp), %rsi
1422a001e:     	jne	0x1422a0033 <.text+0x229f033>
1422a0020:     	leaq	0x130(%rbp), %rcx
1422a0027:     	leaq	0xd9c90a(%rip), %rdx    # 0x14303c938
1422a002e:     	callq	0x140562c30 <.text+0x561c30>
1422a0033:     	movq	0x138(%rbp), %rax
1422a003a:     	movq	%rsi, %rcx
1422a003d:     	shlq	$0x5, %rcx
1422a0041:     	movdqu	0xd8(%rbp), %xmm0
1422a0049:     	movups	0xe8(%rbp), %xmm1
1422a0050:     	movups	%xmm1, 0x10(%rax,%rcx)
1422a0055:     	movdqu	%xmm0, (%rax,%rcx)
1422a005a:     	incq	%rsi
1422a005d:     	movq	%rsi, 0x140(%rbp)
1422a0064:     	nopw	%cs:(%rax,%rax)
1422a0070:     	movq	0x1f0(%rbp), %rbx
1422a0077:     	movzbl	(%rbx), %eax
1422a007a:     	cmpb	$0xb, %al
1422a007c:     	jne	0x1422a0660 <.text+0x229f660>
1422a0082:     	movl	0x4(%rbx), %esi
1422a0085:     	movq	0x200(%rbp), %rcx
1422a008c:     	movl	%esi, %edx
1422a008e:     	callq	0x140a1c930 <.text+0xa1b930>
1422a0093:     	testq	%rax, %rax
1422a0096:     	je	0x1422a0660 <.text+0x229f660>
1422a009c:     	addq	$0x30, %rax
1422a00a0:     	movq	%rax, %rcx
1422a00a3:     	callq	0x140839490 <.text+0x838490>
1422a00a8:     	testq	%rax, %rax
1422a00ab:     	je	0x1422a0660 <.text+0x229f660>
1422a00b1:     	movl	0x8(%rax), %ebx
1422a00b4:     	movl	0xc(%rax), %edi
1422a00b7:     	movb	0x10(%rax), %r14b
1422a00bb:     	leaq	0x180(%rbp), %rcx
1422a00c2:     	movq	0x58(%rbp), %rdx
1422a00c6:     	movl	%esi, %r8d
1422a00c9:     	callq	0x1408fe8c0 <.text+0x8fd8c0>
1422a00ce:     	movq	0x188(%rbp), %rcx
1422a00d5:     	movq	0x190(%rbp), %rdx
1422a00dc:     	testq	%rdx, %rdx
1422a00df:     	movl	%esi, 0x1d8(%rbp)
1422a00e5:     	movl	%edi, 0x150(%rbp)
1422a00eb:     	movl	%ebx, 0x148(%rbp)
1422a00f1:     	movb	%r14b, 0x1c0(%rbp)
1422a00f8:     	je	0x1422a01f6 <.text+0x229f1f6>
1422a00fe:     	movq	0x60(%rbp), %r8
1422a0102:     	movq	0x18(%r8), %rax
1422a0106:     	testq	%rax, %rax
1422a0109:     	je	0x1422a01e1 <.text+0x229f1e1>
1422a010f:     	leaq	(%rcx,%rdx,4), %rdx
1422a0113:     	movq	(%r8), %r8
1422a0116:     	movdqa	(%r8), %xmm0
1422a011b:     	pmovmskb	%xmm0, %r9d
1422a0120:     	notl	%r9d
1422a0123:     	leaq	0x10(%r8), %r10
1422a0127:     	movl	$0x0, 0x1e8(%rbp)
1422a0131:     	movq	%rcx, %r11
1422a0134:     	jmp	0x1422a0152 <.text+0x229f152>
1422a0136:     	nopw	%cs:(%rax,%rax)
1422a0140:     	movb	$0x1, %sil
1422a0143:     	movl	%esi, 0x1e8(%rbp)
1422a0149:     	cmpq	%rdx, %r11
1422a014c:     	je	0x1422a0200 <.text+0x229f200>
1422a0152:     	movl	(%r11), %edi
1422a0155:     	addq	$0x4, %r11
1422a0159:     	movq	%rax, %r14
1422a015c:     	movl	%r9d, %ebx
1422a015f:     	movq	%r10, %r15
1422a0162:     	movq	%r8, %r13
1422a0165:     	jmp	0x1422a017e <.text+0x229f17e>
1422a0167:     	nopw	(%rax,%rax)
1422a0170:     	cmpb	$0x0, -0x80(%rbx)
1422a0174:     	jne	0x1422a0140 <.text+0x229f140>
1422a0176:     	movl	%r12d, %ebx
1422a0179:     	decq	%r14
1422a017c:     	je	0x1422a0149 <.text+0x229f149>
1422a017e:     	testw	%bx, %bx
1422a0181:     	je	0x1422a0190 <.text+0x229f190>
1422a0183:     	leal	-0x1(%rbx), %r12d
1422a0187:     	andl	%ebx, %r12d
1422a018a:     	jmp	0x1422a01bb <.text+0x229f1bb>
1422a018c:     	nopl	(%rax)
1422a0190:     	movdqa	(%r15), %xmm0
1422a0195:     	pmovmskb	%xmm0, %ebx
1422a0199:     	addq	$-0x1100, %r13          # imm = 0xEF00
1422a01a0:     	addq	$0x10, %r15
1422a01a4:     	cmpl	$0xffff, %ebx           # imm = 0xFFFF
1422a01aa:     	je	0x1422a0190 <.text+0x229f190>
1422a01ac:     	movl	$0xfffffffe, %esi       # imm = 0xFFFFFFFE
1422a01b1:     	subl	%ebx, %esi
1422a01b3:     	notl	%ebx
1422a01b5:     	movl	%ebx, %r12d
1422a01b8:     	andl	%esi, %r12d
1422a01bb:     	tzcntl	%ebx, %esi
1422a01bf:     	negq	%rsi
1422a01c2:     	imulq	$0x110, %rsi, %rsi      # imm = 0x110
1422a01c9:     	leaq	(%rsi,%r13), %rbx
1422a01cd:     	cmpl	%edi, -0x110(%r13,%rsi)
1422a01d5:     	je	0x1422a0170 <.text+0x229f170>
1422a01d7:     	cmpl	%edi, -0x10c(%rbx)
1422a01dd:     	je	0x1422a0170 <.text+0x229f170>
1422a01df:     	jmp	0x1422a0176 <.text+0x229f176>
1422a01e1:     	shlq	$0x2, %rdx
1422a01e5:     	nopw	%cs:(%rax,%rax)
1422a01f0:     	addq	$-0x4, %rdx
1422a01f4:     	jne	0x1422a01f0 <.text+0x229f1f0>
1422a01f6:     	movl	$0x0, 0x1e8(%rbp)
1422a0200:     	movq	0x180(%rbp), %rdx
1422a0207:     	testq	%rdx, %rdx
1422a020a:     	je	0x1422a021b <.text+0x229f21b>
1422a020c:     	shlq	$0x2, %rdx
1422a0210:     	movl	$0x4, %r8d
1422a0216:     	callq	0x140613c20 <.text+0x612c20>
1422a021b:     	movzbl	0x1c0(%rbp), %ebx
1422a0222:     	cmpb	$0x3, %bl
1422a0225:     	setne	%al
1422a0228:     	movl	0x1e8(%rbp), %edx
1422a022e:     	movl	%edx, %ecx
1422a0230:     	xorb	$0x1, %cl
1422a0233:     	testb	%cl, %al
1422a0235:     	movq	0x1f0(%rbp), %rax
1422a023c:     	movl	0x1d8(%rbp), %r14d
1422a0243:     	jne	0x1422a0660 <.text+0x229f660>
1422a0249:     	movss	0x10(%rax), %xmm7
1422a024e:     	movss	0x14(%rax), %xmm8
1422a0254:     	movaps	%xmm10, %xmm9
1422a0258:     	mulss	0x7e8faf(%rip), %xmm9   # 0x142a89210
1422a0261:     	mulss	0x776a4e(%rip), %xmm10  # 0x142a16cb8
1422a026a:     	movq	$0x0, 0x158(%rbp)
1422a0275:     	movq	$0x8, 0x160(%rbp)
1422a0280:     	movq	$0x0, 0x168(%rbp)
1422a028b:     	testb	$0x1, %dl
1422a028e:     	je	0x1422a091b <.text+0x229f91b>
1422a0294:     	movzbl	0x139496e(%rip), %eax   # 0x143634c09
1422a029b:     	movl	$0xf0, %ecx
1422a02a0:     	movl	$0x8, %edx
1422a02a5:     	callq	0x140613c10 <.text+0x612c10>
1422a02aa:     	testq	%rax, %rax
1422a02ad:     	je	0x1422a1c9b <.text+0x22a0c9b>
1422a02b3:     	movq	%rax, %rsi
1422a02b6:     	movzbl	0x139494c(%rip), %eax   # 0x143634c09
1422a02bd:     	movl	$0x1b, %ecx
1422a02c2:     	movl	$0x1, %edx
1422a02c7:     	callq	0x140613c10 <.text+0x612c10>
1422a02cc:     	testq	%rax, %rax
1422a02cf:     	je	0x1422a1caf <.text+0x22a0caf>
1422a02d5:     	movq	%rax, %rdi
1422a02d8:     	movups	0xd9bf14(%rip), %xmm0   # 0x14303c1f3
1422a02df:     	movups	%xmm0, 0xb(%rax)
1422a02e3:     	movdqu	0xd9befd(%rip), %xmm0   # 0x14303c1e8
1422a02eb:     	movdqu	%xmm0, (%rax)
1422a02ef:     	movzbl	0x1394913(%rip), %eax   # 0x143634c09
1422a02f6:     	movl	$0x17, %ecx
1422a02fb:     	movl	$0x1, %edx
1422a0300:     	callq	0x140613c10 <.text+0x612c10>
1422a0305:     	testq	%rax, %rax
1422a0308:     	je	0x1422a1b88 <.text+0x22a0b88>
1422a030e:     	movq	%rax, %r15
1422a0311:     	movdqu	0xd9c343(%rip), %xmm0   # 0x14303c65c
1422a0319:     	movdqu	%xmm0, (%rax)
1422a031d:     	movabsq	$0x65756c622f73776f, %rax # imm = 0x65756C622F73776F
1422a0327:     	movq	%rax, 0xf(%r15)
1422a032b:     	movzbl	0x13948d7(%rip), %eax   # 0x143634c09
1422a0332:     	movl	$0x19, %ecx
1422a0337:     	movl	$0x1, %edx
1422a033c:     	callq	0x140613c10 <.text+0x612c10>
1422a0341:     	testq	%rax, %rax
1422a0344:     	je	0x1422a1bb1 <.text+0x22a0bb1>
1422a034a:     	movq	%rax, %r12
1422a034d:     	movups	0xd9c0c8(%rip), %xmm0   # 0x14303c41c
1422a0354:     	movups	%xmm0, 0x9(%rax)
1422a0358:     	movdqu	0xd9c0b3(%rip), %xmm0   # 0x14303c413
1422a0360:     	movdqu	%xmm0, (%rax)
1422a0364:     	movzbl	0x139489e(%rip), %eax   # 0x143634c09
1422a036b:     	movl	$0x15, %ecx
1422a0370:     	movl	$0x1, %edx
1422a0375:     	callq	0x140613c10 <.text+0x612c10>
1422a037a:     	testq	%rax, %rax
1422a037d:     	je	0x1422a1be1 <.text+0x22a0be1>
1422a0383:     	movq	%rax, %r13
1422a0386:     	movdqu	0xd9bf13(%rip), %xmm0   # 0x14303c2a1
1422a038e:     	movdqu	%xmm0, (%rax)
1422a0392:     	movabsq	$0x657268636f5f666f, %rax # imm = 0x657268636F5F666F
1422a039c:     	movq	%rax, 0xd(%r13)
1422a03a0:     	movzbl	0x1394862(%rip), %eax   # 0x143634c09
1422a03a7:     	movl	$0x1b, %ecx
1422a03ac:     	movl	$0x1, %edx
1422a03b1:     	callq	0x140613c10 <.text+0x612c10>
1422a03b6:     	testq	%rax, %rax
1422a03b9:     	je	0x1422a1c18 <.text+0x22a0c18>
1422a03bf:     	movups	0xd9bf6b(%rip), %xmm0   # 0x14303c331
1422a03c6:     	movups	%xmm0, 0xb(%rax)
1422a03ca:     	movdqu	0xd9bf54(%rip), %xmm0   # 0x14303c326
1422a03d2:     	movdqu	%xmm0, (%rax)
1422a03d6:     	movq	%rax, %r14
1422a03d9:     	movzbl	0x1394829(%rip), %eax   # 0x143634c09
1422a03e0:     	movl	$0x14, %ecx
1422a03e5:     	movl	$0x1, %edx
1422a03ea:     	callq	0x140613c10 <.text+0x612c10>
1422a03ef:     	testq	%rax, %rax
1422a03f2:     	je	0x1422a1c56 <.text+0x22a0c56>
1422a03f8:     	movdqu	0xd9bf6f(%rip), %xmm0   # 0x14303c36f
1422a0400:     	movdqu	%xmm0, (%rax)
1422a0404:     	movl	$0x6b726164, 0x10(%rax) # imm = 0x6B726164
1422a040b:     	movq	$0x1b, (%rsi)
1422a0412:     	movq	%rdi, 0x8(%rsi)
1422a0416:     	movq	$0x1b, 0x10(%rsi)
1422a041e:     	movb	$0xe, 0x18(%rsi)
1422a0422:     	movl	$0x0, 0x1c(%rsi)
1422a0429:     	movl	$0x0, 0x24(%rsi)
1422a0430:     	movq	$0x17, 0x28(%rsi)
1422a0438:     	movq	%r15, 0x30(%rsi)
1422a043c:     	movq	$0x17, 0x38(%rsi)
1422a0444:     	movb	$0xe, 0x40(%rsi)
1422a0448:     	movl	$0x1, 0x44(%rsi)
1422a044f:     	movl	$0x0, 0x4c(%rsi)
1422a0456:     	movq	$0x19, 0x50(%rsi)
1422a045e:     	movq	%r12, 0x58(%rsi)
1422a0462:     	movq	$0x19, 0x60(%rsi)
1422a046a:     	movb	$0xe, 0x68(%rsi)
1422a046e:     	movl	$0x2, 0x6c(%rsi)
1422a0475:     	movl	$0x0, 0x74(%rsi)
1422a047c:     	movq	$0x15, 0x78(%rsi)
1422a0484:     	movq	%r13, 0x80(%rsi)
1422a048b:     	movq	$0x15, 0x88(%rsi)
1422a0496:     	movb	$0xe, 0x90(%rsi)
1422a049d:     	movl	$0x3, 0x94(%rsi)
1422a04a7:     	movl	$0x0, 0x9c(%rsi)
1422a04b1:     	movq	$0x1b, 0xa0(%rsi)
1422a04bc:     	movq	%r14, 0xa8(%rsi)
1422a04c3:     	movq	$0x1b, 0xb0(%rsi)
1422a04ce:     	movb	$0xe, 0xb8(%rsi)
1422a04d5:     	movl	$0x4, 0xbc(%rsi)
1422a04df:     	movl	$0x0, 0xc4(%rsi)
1422a04e9:     	movq	$0x14, 0xc8(%rsi)
1422a04f4:     	movq	%rax, 0xd0(%rsi)
1422a04fb:     	movq	$0x14, 0xd8(%rsi)
1422a0506:     	movb	$0xe, 0xe0(%rsi)
1422a050d:     	movl	$0x5, 0xe4(%rsi)
1422a0517:     	movl	$0x0, 0xec(%rsi)
1422a0521:     	movq	%rsi, %rax
1422a0524:     	addq	$0xf0, %rax
1422a052a:     	movq	%rsi, 0x180(%rbp)
1422a0531:     	movq	$0x6, 0x190(%rbp)
1422a053c:     	movq	%rsi, 0x188(%rbp)
1422a0543:     	movq	%rax, 0x198(%rbp)
1422a054a:     	movq	0x158(%rbp), %rax
1422a0551:     	movq	0x168(%rbp), %rdi
1422a0558:     	subq	%rdi, %rax
1422a055b:     	cmpq	$0x6, %rax
1422a055f:     	jb	0x1422a0cf0 <.text+0x229fcf0>
1422a0565:     	movl	0x1d8(%rbp), %r14d
1422a056c:     	leaq	(%rdi,%rdi,4), %rcx
1422a0570:     	shlq	$0x3, %rcx
1422a0574:     	addq	0x160(%rbp), %rcx
1422a057b:     	movl	$0xf0, %r8d
1422a0581:     	movq	%rsi, %rdx
1422a0584:     	callq	0x142923c00 <.text+0x2922c00>
1422a0589:     	addq	$0x6, %rdi
1422a058d:     	movq	%rdi, 0x168(%rbp)
1422a0594:     	movl	$0xf0, %edx
1422a0599:     	movl	$0x8, %r8d
1422a059f:     	movq	%rsi, %rcx
1422a05a2:     	callq	0x140613c20 <.text+0x612c20>
1422a05a7:     	cmpb	$0x3, %bl
1422a05aa:     	leaq	0x180(%rbp), %rsi
1422a05b1:     	jne	0x1422a09d8 <.text+0x229f9d8>
1422a05b7:     	movzbl	0x139464b(%rip), %eax   # 0x143634c09
1422a05be:     	movl	$0x5, %ecx
1422a05c3:     	movl	$0x1, %edx
1422a05c8:     	callq	0x140613c10 <.text+0x612c10>
1422a05cd:     	testq	%rax, %rax
1422a05d0:     	je	0x1422a2a1a <.text+0x22a1a1a>
1422a05d6:     	movzbl	0xe9677a(%rip), %ecx    # 0x143136d57
1422a05dd:     	movb	%cl, 0x4(%rax)
1422a05e0:     	movl	0xe9676d(%rip), %ecx    # 0x143136d53
1422a05e6:     	movq	%rax, 0x1f0(%rbp)
1422a05ed:     	movl	%ecx, (%rax)
1422a05ef:     	movq	0x168(%rbp), %rsi
1422a05f6:     	cmpq	0x158(%rbp), %rsi
1422a05fd:     	jne	0x1422a0612 <.text+0x229f612>
1422a05ff:     	leaq	0x158(%rbp), %rcx
1422a0606:     	leaq	0xd9c343(%rip), %rdx    # 0x14303c950
1422a060d:     	callq	0x140562ab0 <.text+0x561ab0>
1422a0612:     	movq	0x160(%rbp), %rax
1422a0619:     	leaq	(%rsi,%rsi,4), %rcx
1422a061d:     	movq	$0x5, (%rax,%rcx,8)
1422a0625:     	movq	0x1f0(%rbp), %rdx
1422a062c:     	movq	%rdx, 0x8(%rax,%rcx,8)
1422a0631:     	movq	$0x5, 0x10(%rax,%rcx,8)
1422a063a:     	movb	$0x5, 0x18(%rax,%rcx,8)
1422a063f:     	movl	$0x0, 0x24(%rax,%rcx,8)
1422a0647:     	incq	%rsi
1422a064a:     	movq	%rsi, 0x168(%rbp)
1422a0651:     	jmp	0x1422a092b <.text+0x229f92b>
1422a0656:     	nopw	%cs:(%rax,%rax)
1422a0660:     	movq	0x138(%rbp), %rbx
1422a0667:     	movq	0x140(%rbp), %rsi
1422a066e:     	movq	%rsi, %rax
1422a0671:     	shlq	$0x5, %rax
1422a0675:     	addq	%rbx, %rax
1422a0678:     	movq	%rax, 0x1f0(%rbp)
1422a067f:     	leaq	-0x1(%rsi), %rax
1422a0683:     	movq	%rax, 0x1e8(%rbp)
1422a068a:     	xorl	%ecx, %ecx
1422a068c:     	movq	%rsi, 0x1d8(%rbp)
1422a0693:     	cmpq	0x1f0(%rbp), %rbx
1422a069a:     	je	0x1422a0810 <.text+0x229f810>
1422a06a0:     	leaq	0x1(%rcx), %r12
1422a06a4:     	movq	0x18(%rbx), %rdi
1422a06a8:     	movq	0x370(%rbp), %rax
1422a06af:     	movq	0x8(%rax), %r8
1422a06b3:     	movq	0x10(%rax), %r15
1422a06b7:     	movq	%r12, %rax
1422a06ba:     	orq	%rsi, %rax
1422a06bd:     	shrq	$0x20, %rax
1422a06c1:     	je	0x1422a06e0 <.text+0x229f6e0>
1422a06c3:     	movq	%r12, %rax
1422a06c6:     	xorl	%edx, %edx
1422a06c8:     	divq	%rsi
1422a06cb:     	movq	%rdx, %r10
1422a06ce:     	movq	0x140(%rbp), %r9
1422a06d5:     	cmpq	%r9, %r10
1422a06d8:     	jb	0x1422a06fa <.text+0x229f6fa>
1422a06da:     	jmp	0x1422a0e30 <.text+0x229fe30>
1422a06df:     	nop
1422a06e0:     	movl	%r12d, %eax
1422a06e3:     	xorl	%edx, %edx
1422a06e5:     	divl	%esi
1422a06e7:     	movl	%edx, %r10d
1422a06ea:     	movq	0x140(%rbp), %r9
1422a06f1:     	cmpq	%r9, %r10
1422a06f4:     	jae	0x1422a0e30 <.text+0x229fe30>
1422a06fa:     	movq	0x138(%rbp), %r11
1422a0701:     	addq	0x1e8(%rbp), %rcx
1422a0708:     	movq	%rcx, %rax
1422a070b:     	orq	%rsi, %rax
1422a070e:     	shrq	$0x20, %rax
1422a0712:     	je	0x1422a0730 <.text+0x229f730>
1422a0714:     	movq	%rcx, %rax
1422a0717:     	xorl	%edx, %edx
1422a0719:     	divq	%rsi
1422a071c:     	cmpq	%r9, %rdx
1422a071f:     	jb	0x1422a073f <.text+0x229f73f>
1422a0721:     	jmp	0x1422a0e19 <.text+0x229fe19>
1422a0726:     	nopw	%cs:(%rax,%rax)
1422a0730:     	movl	%ecx, %eax
1422a0732:     	xorl	%edx, %edx
1422a0734:     	divl	%esi
1422a0736:     	cmpq	%r9, %rdx
1422a0739:     	jae	0x1422a0e19 <.text+0x229fe19>
1422a073f:     	movq	%rdi, %rsi
1422a0742:     	shlq	$0x5, %r10
1422a0746:     	shlq	$0x5, %rdx
1422a074a:     	testq	%r8, %r8
1422a074d:     	movq	0x18(%r11,%r10), %r13
1422a0752:     	movq	0x18(%r11,%rdx), %r14
1422a0757:     	cmovneq	%r8, %r15
1422a075b:     	movq	(%r15), %rax
1422a075e:     	movq	0x10(%r15), %rdi
1422a0762:     	subq	%rdi, %rax
1422a0765:     	cmpq	$0x5f, %rax
1422a0769:     	jbe	0x1422a07e3 <.text+0x229f7e3>
1422a076b:     	addq	$0x20, %rbx
1422a076f:     	movq	0x8(%r15), %rax
1422a0773:     	leaq	0x1dad6(%rip), %rcx     # 0x1422be250 <.text+0x22bd250>
1422a077a:     	movq	%rcx, (%rax,%rdi)
1422a077e:     	movq	$0x2, 0x8(%rax,%rdi)
1422a0787:     	movq	$0x2, 0x18(%rax,%rdi)
1422a0790:     	movq	$0x3, 0x28(%rax,%rdi)
1422a0799:     	movq	%r13, 0x30(%rax,%rdi)
1422a079e:     	movq	$0x3, 0x38(%rax,%rdi)
1422a07a7:     	movq	%r14, 0x40(%rax,%rdi)
1422a07ac:     	movb	$0x0, 0x48(%rax,%rdi)
1422a07b1:     	movq	%rsi, 0x50(%rax,%rdi)
1422a07b6:     	leaq	0x1f213(%rip), %rcx     # 0x1422bf9d0 <.text+0x22be9d0>
1422a07bd:     	movq	%rcx, 0x58(%rax,%rdi)
1422a07c2:     	addq	$0x60, %rdi
1422a07c6:     	movq	%rdi, 0x10(%r15)
1422a07ca:     	movq	%r12, %rcx
1422a07cd:     	movq	0x1d8(%rbp), %rsi
1422a07d4:     	cmpq	0x1f0(%rbp), %rbx
1422a07db:     	jne	0x1422a06a0 <.text+0x229f6a0>
1422a07e1:     	jmp	0x1422a0810 <.text+0x229f810>
1422a07e3:     	movq	$0x1, 0x20(%rsp)
1422a07ec:     	movl	$0x60, %r8d
1422a07f2:     	movl	$0x1, %r9d
1422a07f8:     	movq	%r15, %rcx
1422a07fb:     	movq	%rdi, %rdx
1422a07fe:     	callq	0x1429107f0 <.text+0x290f7f0>
1422a0803:     	jmp	0x1422a076b <.text+0x229f76b>
1422a0808:     	nopl	(%rax,%rax)
1422a0810:     	movq	0x140(%rbp), %rsi
1422a0817:     	testq	%rsi, %rsi
1422a081a:     	je	0x1422a0880 <.text+0x229f880>
1422a081c:     	movq	0x138(%rbp), %r15
1422a0823:     	movq	0x18(%r15), %rax
1422a0827:     	movl	0x74(%rbp), %ecx
1422a082a:     	movq	0x18(%rbp), %rdx
1422a082e:     	movl	%ecx, (%rdx)
1422a0830:     	movq	0x10(%rbp), %rcx
1422a0834:     	movq	$0x1, 0x10(%rcx)
1422a083c:     	movq	%rax, 0x18(%rcx)
1422a0840:     	leaq	0x8(%r15), %rdi
1422a0844:     	jmp	0x1422a0859 <.text+0x229f859>
1422a0846:     	nopw	%cs:(%rax,%rax)
1422a0850:     	addq	$0x20, %rdi
1422a0854:     	decq	%rsi
1422a0857:     	je	0x1422a0887 <.text+0x229f887>
1422a0859:     	movq	-0x8(%rdi), %rdx
1422a085d:     	testq	%rdx, %rdx
1422a0860:     	je	0x1422a0850 <.text+0x229f850>
1422a0862:     	movq	(%rdi), %rcx
1422a0865:     	shlq	$0x3, %rdx
1422a0869:     	movl	$0x8, %r8d
1422a086f:     	callq	0x140613c20 <.text+0x612c20>
1422a0874:     	jmp	0x1422a0850 <.text+0x229f850>
1422a0876:     	nopw	%cs:(%rax,%rax)
1422a0880:     	movq	0x138(%rbp), %r15
1422a0887:     	movq	0x130(%rbp), %rdx
1422a088e:     	testq	%rdx, %rdx
1422a0891:     	je	0x1422a08a5 <.text+0x229f8a5>
1422a0893:     	shlq	$0x5, %rdx
1422a0897:     	movl	$0x8, %r8d
1422a089d:     	movq	%r15, %rcx
1422a08a0:     	callq	0x140613c20 <.text+0x612c20>
1422a08a5:     	movq	0x118(%rbp), %r15
1422a08ac:     	movq	0x120(%rbp), %rsi
1422a08b3:     	testq	%rsi, %rsi
1422a08b6:     	je	0x1422a08f0 <.text+0x229f8f0>
1422a08b8:     	leaq	0x8(%r15), %rdi
1422a08bc:     	jmp	0x1422a08c9 <.text+0x229f8c9>
1422a08be:     	nop
1422a08c0:     	addq	$0x28, %rdi
1422a08c4:     	decq	%rsi
1422a08c7:     	je	0x1422a08f0 <.text+0x229f8f0>
1422a08c9:     	movq	-0x8(%rdi), %rdx
1422a08cd:     	testq	%rdx, %rdx
1422a08d0:     	je	0x1422a08c0 <.text+0x229f8c0>
1422a08d2:     	movq	(%rdi), %rcx
1422a08d5:     	movl	$0x1, %r8d
1422a08db:     	callq	0x140613c20 <.text+0x612c20>
1422a08e0:     	jmp	0x1422a08c0 <.text+0x229f8c0>
1422a08e2:     	nopw	%cs:(%rax,%rax)
1422a08f0:     	movq	0x110(%rbp), %rax
1422a08f7:     	testq	%rax, %rax
1422a08fa:     	je	0x14229a9e0 <.text+0x22999e0>
1422a0900:     	shlq	$0x3, %rax
1422a0904:     	leaq	(%rax,%rax,4), %rdx
1422a0908:     	movl	$0x8, %r8d
1422a090e:     	movq	%r15, %rcx
1422a0911:     	callq	0x140613c20 <.text+0x612c20>
1422a0916:     	jmp	0x14229a9e0 <.text+0x22999e0>
1422a091b:     	cmpb	$0x3, %bl
1422a091e:     	leaq	0x180(%rbp), %rsi
1422a0925:     	jne	0x1422a09d8 <.text+0x229f9d8>
1422a092b:     	leaq	0xd8(%rbp), %rcx
1422a0932:     	callq	0x1422acce0 <.text+0x22abce0>
1422a0937:     	movq	0xd8(%rbp), %rsi
1422a093e:     	movq	0xe0(%rbp), %r15
1422a0945:     	movq	0xe8(%rbp), %r13
1422a094c:     	leaq	(,%r13,8), %rax
1422a0954:     	leaq	(%rax,%rax,4), %rdi
1422a0958:     	leaq	(%r15,%rdi), %rax
1422a095c:     	movq	%r15, 0x180(%rbp)
1422a0963:     	movq	%rsi, 0x190(%rbp)
1422a096a:     	movq	%r15, 0x188(%rbp)
1422a0971:     	movq	%rax, 0x198(%rbp)
1422a0978:     	movq	0x158(%rbp), %rax
1422a097f:     	movq	0x168(%rbp), %rbx
1422a0986:     	subq	%rbx, %rax
1422a0989:     	cmpq	%r13, %rax
1422a098c:     	jb	0x1422a0cc3 <.text+0x229fcc3>
1422a0992:     	leaq	(%rbx,%rbx,4), %rcx
1422a0996:     	shlq	$0x3, %rcx
1422a099a:     	addq	0x160(%rbp), %rcx
1422a09a1:     	movq	%r15, %rdx
1422a09a4:     	movq	%rdi, %r8
1422a09a7:     	callq	0x142923c00 <.text+0x2922c00>
1422a09ac:     	addq	%rbx, %r13
1422a09af:     	movq	%r13, 0x168(%rbp)
1422a09b6:     	testq	%rsi, %rsi
1422a09b9:     	je	0x1422a09d1 <.text+0x229f9d1>
1422a09bb:     	shlq	$0x3, %rsi
1422a09bf:     	leaq	(%rsi,%rsi,4), %rdx
1422a09c3:     	movl	$0x8, %r8d
1422a09c9:     	movq	%r15, %rcx
1422a09cc:     	callq	0x140613c20 <.text+0x612c20>
1422a09d1:     	leaq	0x180(%rbp), %rsi
1422a09d8:     	movq	0x168(%rbp), %rax
1422a09df:     	movq	%rax, 0xc0(%rbp)
1422a09e6:     	movups	0x158(%rbp), %xmm0
1422a09ed:     	movaps	%xmm0, 0xb0(%rbp)
1422a09f4:     	movl	%r14d, 0x15c(%rbp)
1422a09fb:     	movb	$0xc, 0x158(%rbp)
1422a0a02:     	movb	$0x1, 0x1ae(%rbp)
1422a0a09:     	callq	0x140b06780 <.text+0xb05780>
1422a0a0e:     	movl	0x148(%rbp), %eax
1422a0a14:     	movl	%eax, 0x180(%rbp)
1422a0a1a:     	movl	$0x0, 0x19d(%rbp)
1422a0a24:     	movss	%xmm0, 0x184(%rbp)
1422a0a2c:     	movb	$0x0, 0x1a1(%rbp)
1422a0a33:     	movb	$0x0, 0x190(%rbp)
1422a0a3a:     	movl	$0x0, 0x188(%rbp)
1422a0a44:     	movb	$0x0, 0x19c(%rbp)
1422a0a4b:     	movl	0x150(%rbp), %eax
1422a0a51:     	movl	%eax, 0x18c(%rbp)
1422a0a57:     	movb	$0x1, 0x1ae(%rbp)
1422a0a5e:     	leaq	0x98(%rbp), %rcx
1422a0a65:     	movq	0x1c8(%rbp), %rdx
1422a0a6c:     	callq	0x140911dd0 <.text+0x910dd0>
1422a0a71:     	addss	%xmm6, %xmm9
1422a0a76:     	movb	$0x0, 0x1ae(%rbp)
1422a0a7d:     	movq	0x370(%rbp), %rax
1422a0a84:     	movq	%rax, 0x60(%rsp)
1422a0a89:     	leaq	0x98(%rbp), %rax
1422a0a90:     	movq	%rax, 0x58(%rsp)
1422a0a95:     	movq	%rsi, 0x50(%rsp)
1422a0a9a:     	leaq	0x158(%rbp), %rax
1422a0aa1:     	movq	%rax, 0x48(%rsp)
1422a0aa6:     	leaq	0xb0(%rbp), %rax
1422a0aad:     	movq	%rax, 0x40(%rsp)
1422a0ab2:     	movss	%xmm11, 0x38(%rsp)
1422a0ab9:     	movss	%xmm15, 0x30(%rsp)
1422a0ac0:     	movss	%xmm10, 0x28(%rsp)
1422a0ac7:     	movl	$0x3ec51eb7, 0x20(%rsp) # imm = 0x3EC51EB7
1422a0acf:     	leaq	0xd8(%rbp), %rcx
1422a0ad6:     	movaps	%xmm7, %xmm1
1422a0ad9:     	movaps	%xmm8, %xmm2
1422a0add:     	movaps	%xmm9, %xmm3
1422a0ae1:     	callq	0x1425f27c0 <.text+0x25f17c0>
1422a0ae6:     	movq	0x140(%rbp), %rsi
1422a0aed:     	cmpq	0x130(%rbp), %rsi
1422a0af4:     	jne	0x1422a0b09 <.text+0x229fb09>
1422a0af6:     	leaq	0x130(%rbp), %rcx
1422a0afd:     	leaq	0xd9be64(%rip), %rdx    # 0x14303c968
1422a0b04:     	callq	0x140562c30 <.text+0x561c30>
1422a0b09:     	movq	0x138(%rbp), %rbx
1422a0b10:     	movq	%rsi, %rax
1422a0b13:     	shlq	$0x5, %rax
1422a0b17:     	movdqu	0xd8(%rbp), %xmm0
1422a0b1f:     	movups	0xe8(%rbp), %xmm1
1422a0b26:     	movups	%xmm1, 0x10(%rbx,%rax)
1422a0b2b:     	movdqu	%xmm0, (%rbx,%rax)
1422a0b30:     	incq	%rsi
1422a0b33:     	movq	%rsi, 0x140(%rbp)
1422a0b3a:     	jmp	0x1422a066e <.text+0x229f66e>
1422a0b3f:     	cmpl	$0xc, %eax
1422a0b42:     	je	0x14229d24e <.text+0x229c24e>
1422a0b48:     	jmp	0x14229d464 <.text+0x229c464>
1422a0b4d:     	movq	0x80(%rbp), %rdx
1422a0b54:     	movq	0x1f8(%rbp), %rcx
1422a0b5b:     	leaq	0xd9be4e(%rip), %r8     # 0x14303c9b0
1422a0b62:     	callq	0x140a91ef0 <.text+0xa90ef0>
1422a0b67:     	testq	%rax, %rax
1422a0b6a:     	je	0x1422a2dfd <.text+0x22a1dfd>
1422a0b70:     	addq	$0x299, %rax            # imm = 0x299
1422a0b76:     	movq	%rax, %rcx
1422a0b79:     	callq	0x1408c5670 <.text+0x8c4670>
1422a0b7e:     	testb	%al, %al
1422a0b80:     	leaq	0x70(%rbp), %rcx
1422a0b84:     	leaq	0x6c(%rbp), %rdx
1422a0b88:     	cmovneq	%rdx, %rcx
1422a0b8c:     	xorl	%edi, %edi
1422a0b8e:     	movl	(%rcx), %r12d
1422a0b91:     	testb	%al, %al
1422a0b93:     	jne	0x14229b0ab <.text+0x229a0ab>
1422a0b99:     	movl	%r12d, %r13d
1422a0b9c:     	jmp	0x14229c623 <.text+0x229b623>
1422a0ba1:     	xorl	%ecx, %ecx
1422a0ba3:     	movl	0x10(%rax), %eax
1422a0ba6:     	imulq	$0x58, %rax, %rdi
1422a0baa:     	addq	%rcx, %rdi
1422a0bad:     	movq	%rdi, %rcx
1422a0bb0:     	callq	0x1408e3380 <.text+0x8e2380>
1422a0bb5:     	testb	%al, %al
1422a0bb7:     	je	0x14229e9c1 <.text+0x229d9c1>
1422a0bbd:     	movzbl	0x1394045(%rip), %eax   # 0x143634c09
1422a0bc4:     	movl	$0x50, %ecx
1422a0bc9:     	movl	$0x8, %edx
1422a0bce:     	callq	0x140613c10 <.text+0x612c10>
1422a0bd3:     	movq	%rax, 0x1f0(%rbp)
1422a0bda:     	testq	%rax, %rax
1422a0bdd:     	je	0x1422a2e97 <.text+0x22a1e97>
1422a0be3:     	leaq	0xd8(%rbp), %rcx
1422a0bea:     	callq	0x1425f2680 <.text+0x25f1680>
1422a0bef:     	movzbl	0x3d(%rdi), %edx
1422a0bf3:     	leaq	0x180(%rbp), %rsi
1422a0bfa:     	movq	%rsi, %rcx
1422a0bfd:     	callq	0x1425f26f0 <.text+0x25f16f0>
1422a0c02:     	movq	0xf8(%rbp), %rax
1422a0c09:     	movq	0x1f0(%rbp), %rcx
1422a0c10:     	movq	%rax, 0x20(%rcx)
1422a0c14:     	movups	0xd8(%rbp), %xmm0
1422a0c1b:     	movups	0xe8(%rbp), %xmm1
1422a0c22:     	movups	%xmm1, 0x10(%rcx)
1422a0c26:     	movups	%xmm0, (%rcx)
1422a0c29:     	movups	0x180(%rbp), %xmm0
1422a0c30:     	movups	0x190(%rbp), %xmm1
1422a0c37:     	movups	%xmm0, 0x28(%rcx)
1422a0c3b:     	movups	%xmm1, 0x38(%rcx)
1422a0c3f:     	movq	0x1a0(%rbp), %rax
1422a0c46:     	movq	%rax, 0x48(%rcx)
1422a0c4a:     	movq	%rcx, %rax
1422a0c4d:     	addq	$0x50, %rax
1422a0c51:     	movq	%rcx, 0x180(%rbp)
1422a0c58:     	movq	$0x2, 0x190(%rbp)
1422a0c63:     	movq	%rcx, 0x188(%rbp)
1422a0c6a:     	movq	%rax, 0x198(%rbp)
1422a0c71:     	leaq	0x158(%rbp), %rcx
1422a0c78:     	movq	%rsi, %rdx
1422a0c7b:     	leaq	0xd9b5f6(%rip), %r8     # 0x14303c278
1422a0c82:     	callq	0x141c5a140 <.text+0x1c59140>
1422a0c87:     	jmp	0x14229e9c1 <.text+0x229d9c1>
1422a0c8c:     	movq	$0x28, 0x20(%rsp)
1422a0c95:     	movl	$0x7, %r8d
1422a0c9b:     	movl	$0x8, %r9d
1422a0ca1:     	leaq	0xd8(%rbp), %rcx
1422a0ca8:     	movq	%rdi, %rdx
1422a0cab:     	callq	0x1429107f0 <.text+0x290f7f0>
1422a0cb0:     	movq	0x1f0(%rbp), %r15
1422a0cb7:     	movq	0xe8(%rbp), %rdi
1422a0cbe:     	jmp	0x14229cbdb <.text+0x229bbdb>
1422a0cc3:     	movq	$0x28, 0x20(%rsp)
1422a0ccc:     	movl	$0x8, %r9d
1422a0cd2:     	leaq	0x158(%rbp), %rcx
1422a0cd9:     	movq	%rbx, %rdx
1422a0cdc:     	movq	%r13, %r8
1422a0cdf:     	callq	0x1429107f0 <.text+0x290f7f0>
1422a0ce4:     	movq	0x168(%rbp), %rbx
1422a0ceb:     	jmp	0x1422a0992 <.text+0x229f992>
1422a0cf0:     	movq	$0x28, 0x20(%rsp)
1422a0cf9:     	movl	$0x6, %r8d
1422a0cff:     	movl	$0x8, %r9d
1422a0d05:     	leaq	0x158(%rbp), %rcx
1422a0d0c:     	movq	%rdi, %rdx
1422a0d0f:     	callq	0x1429107f0 <.text+0x290f7f0>
1422a0d14:     	movl	0x1d8(%rbp), %r14d
1422a0d1b:     	movq	0x168(%rbp), %rdi
1422a0d22:     	jmp	0x1422a056c <.text+0x229f56c>
1422a0d27:     	movq	$0x28, 0x20(%rsp)
1422a0d30:     	movl	$0x4, %r8d
1422a0d36:     	movl	$0x8, %r9d
1422a0d3c:     	leaq	0x158(%rbp), %rcx
1422a0d43:     	movq	%rdi, %rdx
1422a0d46:     	callq	0x1429107f0 <.text+0x290f7f0>
1422a0d4b:     	movq	0x168(%rbp), %rdi
1422a0d52:     	jmp	0x14229d429 <.text+0x229c429>
1422a0d57:     	movq	$0x28, 0x20(%rsp)
1422a0d60:     	movl	$0x4, %r8d
1422a0d66:     	movl	$0x8, %r9d
1422a0d6c:     	leaq	0x158(%rbp), %rcx
1422a0d73:     	movq	%rdi, %rdx
1422a0d76:     	callq	0x1429107f0 <.text+0x290f7f0>
1422a0d7b:     	movq	0x168(%rbp), %rdi
1422a0d82:     	jmp	0x14229d62f <.text+0x229c62f>
1422a0d87:     	movq	$0x28, 0x20(%rsp)
1422a0d90:     	movl	$0x5, %r8d
1422a0d96:     	movl	$0x8, %r9d
1422a0d9c:     	leaq	0x158(%rbp), %rcx
1422a0da3:     	movq	%rdi, %rdx
1422a0da6:     	callq	0x1429107f0 <.text+0x290f7f0>
1422a0dab:     	movq	0x168(%rbp), %rdi
1422a0db2:     	jmp	0x14229e986 <.text+0x229d986>
1422a0db7:     	movaps	0x210(%rbp), %xmm6
1422a0dbe:     	movaps	0x220(%rbp), %xmm7
1422a0dc5:     	movaps	0x230(%rbp), %xmm8
1422a0dcd:     	movaps	0x240(%rbp), %xmm9
1422a0dd5:     	movaps	0x250(%rbp), %xmm10
1422a0ddd:     	movaps	0x260(%rbp), %xmm11
1422a0de5:     	movaps	0x270(%rbp), %xmm12
1422a0ded:     	movaps	0x280(%rbp), %xmm13
1422a0df5:     	movaps	0x290(%rbp), %xmm14
1422a0dfd:     	movaps	0x2a0(%rbp), %xmm15
1422a0e05:     	addq	$0x338, %rsp            # imm = 0x338
1422a0e0c:     	popq	%rbx
1422a0e0d:     	popq	%rdi
1422a0e0e:     	popq	%rsi
1422a0e0f:     	popq	%r12
1422a0e11:     	popq	%r13
1422a0e13:     	popq	%r14
1422a0e15:     	popq	%r15
1422a0e17:     	popq	%rbp
1422a0e18:     	retq
1422a0e19:     	leaq	0xd9bb78(%rip), %r8     # 0x14303c998
1422a0e20:     	movq	%rdx, %rcx
1422a0e23:     	movq	%r9, %rdx
1422a0e26:     	callq	0x1428d9518 <.text+0x28d8518>
1422a0e2b:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a0e30:     	leaq	0xd9bb49(%rip), %r8     # 0x14303c980
1422a0e37:     	movq	%r10, %rcx
1422a0e3a:     	movq	%r9, %rdx
1422a0e3d:     	callq	0x1428d9518 <.text+0x28d8518>
1422a0e42:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a0e47:     	shlq	$0x20, %rcx
1422a0e4b:     	orq	%r8, %rcx
1422a0e4e:     	movl	$0x1, %r8d
1422a0e54:     	movq	%rcx, %rax
1422a0e57:     	movl	%r8d, 0x180(%rbp)
1422a0e5e:     	movl	%edx, 0x184(%rbp)
1422a0e64:     	movq	%rax, 0x188(%rbp)
1422a0e6b:     	leaq	0xd9b9be(%rip), %rax    # 0x14303c830
1422a0e72:     	movq	%rax, 0x20(%rsp)
1422a0e77:     	leaq	0xd9b1a2(%rip), %rcx    # 0x14303c020
1422a0e7e:     	leaq	0xd9b17b(%rip), %r9     # 0x14303c000
1422a0e85:     	leaq	0x180(%rbp), %r8
1422a0e8c:     	movl	$0x2b, %edx
1422a0e91:     	callq	0x1428d9760 <.text+0x28d8760>
1422a0e96:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a0e9b:     	leaq	0xd9b976(%rip), %rcx    # 0x14303c818
1422a0ea2:     	callq	0x1428d9310 <.text+0x28d8310>
1422a0ea7:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a0eac:     	leaq	0xd9b8bd(%rip), %rcx    # 0x14303c770
1422a0eb3:     	callq	0x1428d9310 <.text+0x28d8310>
1422a0eb8:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a0ebd:     	leaq	0xd9b90c(%rip), %rcx    # 0x14303c7d0
1422a0ec4:     	callq	0x1428d9310 <.text+0x28d8310>
1422a0ec9:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a0ece:     	leaq	0xd9b98b(%rip), %rcx    # 0x14303c860
1422a0ed5:     	callq	0x1428d9310 <.text+0x28d8310>
1422a0eda:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a0edf:     	shlq	$0x20, %rcx
1422a0ee3:     	orq	%r8, %rcx
1422a0ee6:     	movl	$0x1, %r8d
1422a0eec:     	movq	%rcx, %rax
1422a0eef:     	movl	%r8d, 0x180(%rbp)
1422a0ef6:     	movl	%edx, 0x184(%rbp)
1422a0efc:     	movq	%rax, 0x188(%rbp)
1422a0f03:     	leaq	0xd9b7d6(%rip), %rax    # 0x14303c6e0
1422a0f0a:     	movq	%rax, 0x20(%rsp)
1422a0f0f:     	leaq	0xd9b10a(%rip), %rcx    # 0x14303c020
1422a0f16:     	leaq	0xd9b0e3(%rip), %r9     # 0x14303c000
1422a0f1d:     	leaq	0x180(%rbp), %r8
1422a0f24:     	movl	$0x2b, %edx
1422a0f29:     	callq	0x1428d9760 <.text+0x28d8760>
1422a0f2e:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a0f33:     	shlq	$0x20, %rcx
1422a0f37:     	orq	%r8, %rcx
1422a0f3a:     	movl	$0x1, %r8d
1422a0f40:     	movq	%rcx, %rax
1422a0f43:     	movl	%r8d, 0x180(%rbp)
1422a0f4a:     	movl	%edx, 0x184(%rbp)
1422a0f50:     	movq	%rax, 0x188(%rbp)
1422a0f57:     	leaq	0xd9b7b2(%rip), %rax    # 0x14303c710
1422a0f5e:     	movq	%rax, 0x20(%rsp)
1422a0f63:     	leaq	0xd9b0b6(%rip), %rcx    # 0x14303c020
1422a0f6a:     	leaq	0xd9b08f(%rip), %r9     # 0x14303c000
1422a0f71:     	leaq	0x180(%rbp), %r8
1422a0f78:     	movl	$0x2b, %edx
1422a0f7d:     	callq	0x1428d9760 <.text+0x28d8760>
1422a0f82:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a0f87:     	movl	$0x8, %ecx
1422a0f8c:     	movl	$0x190, %edx            # imm = 0x190
1422a0f91:     	callq	0x1428d9000 <.text+0x28d8000>
1422a0f96:     	leaq	0xd9b653(%rip), %rcx    # 0x14303c5f0
1422a0f9d:     	leaq	0xd9b674(%rip), %r8     # 0x14303c618
1422a0fa4:     	movl	$0x28, %edx
1422a0fa9:     	callq	0x1428d9430 <.text+0x28d8430>
1422a0fae:     	leaq	0xd9b63b(%rip), %rcx    # 0x14303c5f0
1422a0fb5:     	leaq	0xd9b784(%rip), %r8     # 0x14303c740
1422a0fbc:     	movl	$0x28, %edx
1422a0fc1:     	callq	0x1428d9430 <.text+0x28d8430>
1422a0fc6:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a0fcb:     	leaq	0xd9b82e(%rip), %rcx    # 0x14303c800
1422a0fd2:     	callq	0x1428d9310 <.text+0x28d8310>
1422a0fd7:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a0fdc:     	movl	$0x8, %ecx
1422a0fe1:     	movl	$0x168, %edx            # imm = 0x168
1422a0fe6:     	callq	0x1428d9000 <.text+0x28d8000>
1422a0feb:     	movb	$0x1, 0x1bf(%rbp)
1422a0ff2:     	leaq	0xd9b8c7(%rip), %r8     # 0x14303c8c0
1422a0ff9:     	movq	%rax, %rcx
1422a0ffc:     	callq	0x1428d9518 <.text+0x28d8518>
1422a1001:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1006:     	movl	$0x8, %ecx
1422a100b:     	movl	$0xc8, %edx
1422a1010:     	callq	0x1428d9000 <.text+0x28d8000>
1422a1015:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a101a:     	movb	$0x1, 0x1d7(%rbp)
1422a1021:     	leaq	0xd9b8c8(%rip), %rcx    # 0x14303c8f0
1422a1028:     	callq	0x1428d9310 <.text+0x28d8310>
1422a102d:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1032:     	movq	%r15, 0x1d8(%rbp)
1422a1039:     	movq	%r14, 0x1e8(%rbp)
1422a1040:     	movq	%rsi, 0x1f0(%rbp)
1422a1047:     	leaq	0xe95c1a(%rip), %r8     # 0x143136c68
1422a104e:     	movl	$0x1, %ecx
1422a1053:     	movl	$0x19, %edx
1422a1058:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a105d:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1062:     	movq	%r13, 0x200(%rbp)
1422a1069:     	movq	%r15, 0x1d8(%rbp)
1422a1070:     	movq	%r14, 0x1e8(%rbp)
1422a1077:     	movq	%rsi, 0x1f0(%rbp)
1422a107e:     	leaq	0xe95be3(%rip), %r8     # 0x143136c68
1422a1085:     	movl	$0x1, %ecx
1422a108a:     	movl	$0x17, %edx
1422a108f:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1094:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1099:     	movq	%rdi, 0x1f8(%rbp)
1422a10a0:     	movq	%r13, 0x200(%rbp)
1422a10a7:     	movq	%r15, 0x1d8(%rbp)
1422a10ae:     	movq	%r14, 0x1e8(%rbp)
1422a10b5:     	movq	%rsi, 0x1f0(%rbp)
1422a10bc:     	leaq	0xe95ba5(%rip), %r8     # 0x143136c68
1422a10c3:     	movl	$0x1, %ecx
1422a10c8:     	movl	$0x1b, %edx
1422a10cd:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a10d2:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a10d7:     	movq	%rsi, 0x1f0(%rbp)
1422a10de:     	leaq	0xe95b83(%rip), %r8     # 0x143136c68
1422a10e5:     	movl	$0x1, %ecx
1422a10ea:     	movl	$0x16, %edx
1422a10ef:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a10f4:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a10f9:     	movq	%r14, 0x1e8(%rbp)
1422a1100:     	movq	%rsi, 0x1f0(%rbp)
1422a1107:     	leaq	0xe95b5a(%rip), %r8     # 0x143136c68
1422a110e:     	movl	$0x1, %ecx
1422a1113:     	movl	$0x16, %edx
1422a1118:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a111d:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1122:     	movq	%r12, 0x200(%rbp)
1422a1129:     	movq	%r15, 0x1d8(%rbp)
1422a1130:     	movq	%r14, 0x1e8(%rbp)
1422a1137:     	movq	%rbx, 0x1f0(%rbp)
1422a113e:     	leaq	0xe95b23(%rip), %r8     # 0x143136c68
1422a1145:     	movl	$0x1, %ecx
1422a114a:     	movl	$0x15, %edx
1422a114f:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1154:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1159:     	movq	%rsi, 0x1e0(%rbp)
1422a1160:     	movq	%r13, 0x1f8(%rbp)
1422a1167:     	movq	%r12, 0x200(%rbp)
1422a116e:     	movq	%r15, 0x1d8(%rbp)
1422a1175:     	movq	%r14, 0x1e8(%rbp)
1422a117c:     	movq	%rbx, 0x1f0(%rbp)
1422a1183:     	leaq	0xe95ade(%rip), %r8     # 0x143136c68
1422a118a:     	movl	$0x1, %ecx
1422a118f:     	movl	$0x14, %edx
1422a1194:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1199:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a119e:     	movl	$0x8, %ecx
1422a11a3:     	movl	$0xf0, %edx
1422a11a8:     	callq	0x1428d9000 <.text+0x28d8000>
1422a11ad:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a11b2:     	movq	%r13, 0x1f8(%rbp)
1422a11b9:     	movq	%r12, 0x200(%rbp)
1422a11c0:     	movq	%r15, 0x1d8(%rbp)
1422a11c7:     	movq	%r14, 0x1e8(%rbp)
1422a11ce:     	movq	%rbx, 0x1f0(%rbp)
1422a11d5:     	leaq	0xe95a8c(%rip), %r8     # 0x143136c68
1422a11dc:     	movl	$0x1, %ecx
1422a11e1:     	movl	$0x1b, %edx
1422a11e6:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a11eb:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a11f0:     	movq	%r15, 0x1d8(%rbp)
1422a11f7:     	movq	%r14, 0x1e8(%rbp)
1422a11fe:     	movq	%rbx, 0x1f0(%rbp)
1422a1205:     	leaq	0xe95a5c(%rip), %r8     # 0x143136c68
1422a120c:     	movl	$0x1, %ecx
1422a1211:     	movl	$0x19, %edx
1422a1216:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a121b:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1220:     	movq	%r14, 0x1e8(%rbp)
1422a1227:     	movq	%rbx, 0x1f0(%rbp)
1422a122e:     	leaq	0xe95a33(%rip), %r8     # 0x143136c68
1422a1235:     	movl	$0x1, %ecx
1422a123a:     	movl	$0x17, %edx
1422a123f:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1244:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1249:     	movq	%rbx, 0x1f0(%rbp)
1422a1250:     	leaq	0xe95a11(%rip), %r8     # 0x143136c68
1422a1257:     	movl	$0x1, %ecx
1422a125c:     	movl	$0x1b, %edx
1422a1261:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1266:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a126b:     	movq	%r15, 0x178(%rbp)
1422a1272:     	movq	%r13, 0x1b0(%rbp)
1422a1279:     	movq	%r12, 0x1c8(%rbp)
1422a1280:     	movq	%r14, 0x1e0(%rbp)
1422a1287:     	movq	%rbx, 0x1f8(%rbp)
1422a128e:     	movq	%rdi, 0x200(%rbp)
1422a1295:     	movq	%rsi, 0x1f0(%rbp)
1422a129c:     	leaq	0xe959c5(%rip), %r8     # 0x143136c68
1422a12a3:     	movl	$0x1, %ecx
1422a12a8:     	movl	$0x14, %edx
1422a12ad:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a12b2:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a12b7:     	movq	%r15, 0x1b0(%rbp)
1422a12be:     	movq	%r13, 0x1c8(%rbp)
1422a12c5:     	movq	%r12, 0x1c0(%rbp)
1422a12cc:     	movq	%r14, 0x1e0(%rbp)
1422a12d3:     	movq	%rbx, 0x1f8(%rbp)
1422a12da:     	movq	%rdi, 0x200(%rbp)
1422a12e1:     	movq	%rsi, 0x1f0(%rbp)
1422a12e8:     	leaq	0xe95979(%rip), %r8     # 0x143136c68
1422a12ef:     	movl	$0x1, %ecx
1422a12f4:     	movl	$0x12, %edx
1422a12f9:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a12fe:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1303:     	movq	%rdi, 0x200(%rbp)
1422a130a:     	movq	%rsi, 0x1f0(%rbp)
1422a1311:     	leaq	0xe95950(%rip), %r8     # 0x143136c68
1422a1318:     	movl	$0x1, %ecx
1422a131d:     	movl	$0x18, %edx
1422a1322:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1327:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a132c:     	movq	%r14, 0x1f8(%rbp)
1422a1333:     	movq	%rbx, 0x200(%rbp)
1422a133a:     	leaq	0xe95927(%rip), %r8     # 0x143136c68
1422a1341:     	movl	$0x1, %ecx
1422a1346:     	movl	$0x15, %edx
1422a134b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1350:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1355:     	movq	%r13, 0x1b0(%rbp)
1422a135c:     	movq	%r12, 0x1c8(%rbp)
1422a1363:     	movq	%r14, 0x1e0(%rbp)
1422a136a:     	movq	%rbx, 0x1f8(%rbp)
1422a1371:     	movq	%rdi, 0x200(%rbp)
1422a1378:     	movq	%rsi, 0x1f0(%rbp)
1422a137f:     	leaq	0xe958e2(%rip), %r8     # 0x143136c68
1422a1386:     	movl	$0x1, %ecx
1422a138b:     	movl	$0x14, %edx
1422a1390:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1395:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a139a:     	leaq	0xd9b00f(%rip), %r8     # 0x14303c3b0
1422a13a1:     	movq	%rax, %rcx
1422a13a4:     	callq	0x1428d9518 <.text+0x28d8518>
1422a13a9:     	movq	%rsi, 0x1f0(%rbp)
1422a13b0:     	leaq	0xe958b1(%rip), %r8     # 0x143136c68
1422a13b7:     	movl	$0x1, %ecx
1422a13bc:     	movl	$0x17, %edx
1422a13c1:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a13c6:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a13cb:     	movq	%rdi, 0x1c8(%rbp)
1422a13d2:     	movq	%r13, 0x1e0(%rbp)
1422a13d9:     	movq	%r14, 0x1f8(%rbp)
1422a13e0:     	movq	%rbx, 0x200(%rbp)
1422a13e7:     	leaq	0xe9587a(%rip), %r8     # 0x143136c68
1422a13ee:     	movl	$0x1, %ecx
1422a13f3:     	movl	$0x15, %edx
1422a13f8:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a13fd:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1402:     	movq	%r13, 0x1b0(%rbp)
1422a1409:     	movq	%r12, 0x1c8(%rbp)
1422a1410:     	movq	%r14, 0x1e0(%rbp)
1422a1417:     	movq	%rbx, 0x1f8(%rbp)
1422a141e:     	movq	%rdi, 0x200(%rbp)
1422a1425:     	movq	%rsi, 0x1f0(%rbp)
1422a142c:     	leaq	0xe95835(%rip), %r8     # 0x143136c68
1422a1433:     	movl	$0x1, %ecx
1422a1438:     	movl	$0x14, %edx
1422a143d:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1442:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1447:     	movq	%r13, 0x1f8(%rbp)
1422a144e:     	movq	%r12, 0x200(%rbp)
1422a1455:     	movq	%r14, 0x1e8(%rbp)
1422a145c:     	movq	%rbx, 0x1f0(%rbp)
1422a1463:     	leaq	0xe957fe(%rip), %r8     # 0x143136c68
1422a146a:     	movl	$0x1, %ecx
1422a146f:     	movl	$0x13, %edx
1422a1474:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1479:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a147e:     	movq	%rdi, 0x200(%rbp)
1422a1485:     	movq	%rsi, 0x1f0(%rbp)
1422a148c:     	leaq	0xe957d5(%rip), %r8     # 0x143136c68
1422a1493:     	movl	$0x1, %ecx
1422a1498:     	movl	$0x16, %edx
1422a149d:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a14a2:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a14a7:     	movq	%rbx, 0x1f8(%rbp)
1422a14ae:     	movq	%rdi, 0x200(%rbp)
1422a14b5:     	movq	%rsi, 0x1f0(%rbp)
1422a14bc:     	leaq	0xe957a5(%rip), %r8     # 0x143136c68
1422a14c3:     	movl	$0x1, %ecx
1422a14c8:     	movl	$0x19, %edx
1422a14cd:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a14d2:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a14d7:     	movq	%r14, 0x1e0(%rbp)
1422a14de:     	movq	%rbx, 0x1f8(%rbp)
1422a14e5:     	movq	%rdi, 0x200(%rbp)
1422a14ec:     	movq	%rsi, 0x1f0(%rbp)
1422a14f3:     	leaq	0xe9576e(%rip), %r8     # 0x143136c68
1422a14fa:     	movl	$0x1, %ecx
1422a14ff:     	movl	$0x17, %edx
1422a1504:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1509:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a150e:     	movq	%r12, 0x1c0(%rbp)
1422a1515:     	movq	%r14, 0x1e0(%rbp)
1422a151c:     	movq	%rbx, 0x1f8(%rbp)
1422a1523:     	movq	%rdi, 0x200(%rbp)
1422a152a:     	movq	%rsi, 0x1f0(%rbp)
1422a1531:     	leaq	0xe95730(%rip), %r8     # 0x143136c68
1422a1538:     	movl	$0x1, %ecx
1422a153d:     	movl	$0x1b, %edx
1422a1542:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1547:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a154c:     	movq	%r13, 0x1c8(%rbp)
1422a1553:     	movq	%r12, 0x1c0(%rbp)
1422a155a:     	movq	%r14, 0x1e0(%rbp)
1422a1561:     	movq	%rbx, 0x1f8(%rbp)
1422a1568:     	movq	%rdi, 0x200(%rbp)
1422a156f:     	movq	%rsi, 0x1f0(%rbp)
1422a1576:     	leaq	0xe956eb(%rip), %r8     # 0x143136c68
1422a157d:     	movl	$0x1, %ecx
1422a1582:     	movl	$0x5, %edx
1422a1587:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a158c:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1591:     	movq	%r13, 0x1c8(%rbp)
1422a1598:     	movq	%r12, 0x1c0(%rbp)
1422a159f:     	movq	%r14, 0x1e0(%rbp)
1422a15a6:     	movq	%rbx, 0x1f8(%rbp)
1422a15ad:     	movq	%rdi, 0x200(%rbp)
1422a15b4:     	movq	%rsi, 0x1f0(%rbp)
1422a15bb:     	leaq	0xe956a6(%rip), %r8     # 0x143136c68
1422a15c2:     	movl	$0x1, %ecx
1422a15c7:     	movl	$0x12, %edx
1422a15cc:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a15d1:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a15d6:     	movl	$0x8, %ecx
1422a15db:     	movl	$0x118, %edx            # imm = 0x118
1422a15e0:     	callq	0x1428d9000 <.text+0x28d8000>
1422a15e5:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a15ea:     	movq	%rbx, 0x1f0(%rbp)
1422a15f1:     	leaq	0xe95670(%rip), %r8     # 0x143136c68
1422a15f8:     	movl	$0x1, %ecx
1422a15fd:     	movl	$0x5, %edx
1422a1602:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1607:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a160c:     	movq	%r14, 0x1e8(%rbp)
1422a1613:     	movq	%rbx, 0x1f0(%rbp)
1422a161a:     	leaq	0xe95647(%rip), %r8     # 0x143136c68
1422a1621:     	movl	$0x1, %ecx
1422a1626:     	movl	$0x10, %edx
1422a162b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1630:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1635:     	movq	%r12, 0x200(%rbp)
1422a163c:     	movq	%r14, 0x1e8(%rbp)
1422a1643:     	movq	%rbx, 0x1f0(%rbp)
1422a164a:     	leaq	0xe95617(%rip), %r8     # 0x143136c68
1422a1651:     	movl	$0x1, %ecx
1422a1656:     	movl	$0x12, %edx
1422a165b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1660:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1665:     	movq	%r13, 0x1f8(%rbp)
1422a166c:     	movq	%r12, 0x200(%rbp)
1422a1673:     	movq	%r14, 0x1e8(%rbp)
1422a167a:     	movq	%rbx, 0x1f0(%rbp)
1422a1681:     	leaq	0xe955e0(%rip), %r8     # 0x143136c68
1422a1688:     	movl	$0x1, %ecx
1422a168d:     	movl	$0x12, %edx
1422a1692:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1697:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a169c:     	movq	%r13, 0x1f8(%rbp)
1422a16a3:     	movq	%r12, 0x200(%rbp)
1422a16aa:     	movq	%r14, 0x1e8(%rbp)
1422a16b1:     	movq	%rbx, 0x1f0(%rbp)
1422a16b8:     	leaq	0xe955a9(%rip), %r8     # 0x143136c68
1422a16bf:     	movl	$0x1, %ecx
1422a16c4:     	movl	$0x11, %edx
1422a16c9:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a16ce:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a16d3:     	movq	%r13, 0x1e0(%rbp)
1422a16da:     	movq	%r14, 0x1f8(%rbp)
1422a16e1:     	movq	%rbx, 0x200(%rbp)
1422a16e8:     	leaq	0xe95579(%rip), %r8     # 0x143136c68
1422a16ef:     	movl	$0x1, %ecx
1422a16f4:     	movl	$0x17, %edx
1422a16f9:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a16fe:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1703:     	movq	%r13, 0x1e0(%rbp)
1422a170a:     	movq	%r14, 0x1f8(%rbp)
1422a1711:     	movq	%rbx, 0x200(%rbp)
1422a1718:     	leaq	0xe95549(%rip), %r8     # 0x143136c68
1422a171f:     	movl	$0x1, %ecx
1422a1724:     	movl	$0x16, %edx
1422a1729:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a172e:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1733:     	movq	%r13, 0x1e0(%rbp)
1422a173a:     	movq	%r14, 0x1f8(%rbp)
1422a1741:     	movq	%rbx, 0x200(%rbp)
1422a1748:     	leaq	0xe95519(%rip), %r8     # 0x143136c68
1422a174f:     	movl	$0x1, %ecx
1422a1754:     	movl	$0x19, %edx
1422a1759:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a175e:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1763:     	movq	%r13, 0x1e0(%rbp)
1422a176a:     	movq	%r14, 0x1f8(%rbp)
1422a1771:     	movq	%rbx, 0x200(%rbp)
1422a1778:     	leaq	0xe954e9(%rip), %r8     # 0x143136c68
1422a177f:     	movl	$0x1, %ecx
1422a1784:     	movl	$0x15, %edx
1422a1789:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a178e:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1793:     	movq	%r13, 0x1e0(%rbp)
1422a179a:     	movq	%r14, 0x1f8(%rbp)
1422a17a1:     	movq	%rbx, 0x200(%rbp)
1422a17a8:     	leaq	0xe954b9(%rip), %r8     # 0x143136c68
1422a17af:     	movl	$0x1, %ecx
1422a17b4:     	movl	$0x1d, %edx
1422a17b9:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a17be:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a17c3:     	movq	%r13, 0x1e0(%rbp)
1422a17ca:     	movq	%r14, 0x1f8(%rbp)
1422a17d1:     	movq	%rbx, 0x200(%rbp)
1422a17d8:     	leaq	0xe95489(%rip), %r8     # 0x143136c68
1422a17df:     	movl	$0x1, %ecx
1422a17e4:     	movl	$0x17, %edx
1422a17e9:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a17ee:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a17f3:     	movq	%r13, 0x1c8(%rbp)
1422a17fa:     	movq	%r12, 0x1c0(%rbp)
1422a1801:     	movq	%r14, 0x1e0(%rbp)
1422a1808:     	movq	%rbx, 0x1f8(%rbp)
1422a180f:     	movq	%rdi, 0x200(%rbp)
1422a1816:     	movq	%rsi, 0x1f0(%rbp)
1422a181d:     	leaq	0xe95444(%rip), %r8     # 0x143136c68
1422a1824:     	movl	$0x1, %ecx
1422a1829:     	movl	$0x12, %edx
1422a182e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1833:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1838:     	movq	%r13, 0x1b0(%rbp)
1422a183f:     	movq	%r12, 0x1c8(%rbp)
1422a1846:     	movq	%r14, 0x1e0(%rbp)
1422a184d:     	movq	%rbx, 0x1f8(%rbp)
1422a1854:     	movq	%rdi, 0x200(%rbp)
1422a185b:     	movq	%rsi, 0x1f0(%rbp)
1422a1862:     	leaq	0xe953ff(%rip), %r8     # 0x143136c68
1422a1869:     	movl	$0x1, %ecx
1422a186e:     	movl	$0x5, %edx
1422a1873:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1878:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a187d:     	movq	%r14, 0x1e0(%rbp)
1422a1884:     	movq	%rbx, 0x1f8(%rbp)
1422a188b:     	movq	%rdi, 0x200(%rbp)
1422a1892:     	movq	%rsi, 0x1f0(%rbp)
1422a1899:     	leaq	0xe953c8(%rip), %r8     # 0x143136c68
1422a18a0:     	movl	$0x1, %ecx
1422a18a5:     	movl	$0x16, %edx
1422a18aa:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a18af:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a18b4:     	leaq	0xd9aadd(%rip), %r8     # 0x14303c398
1422a18bb:     	movq	%rax, %rcx
1422a18be:     	callq	0x1428d9518 <.text+0x28d8518>
1422a18c3:     	movq	%rsi, 0x1e0(%rbp)
1422a18ca:     	movq	%r13, 0x1f8(%rbp)
1422a18d1:     	movq	%r12, 0x200(%rbp)
1422a18d8:     	movq	%r14, 0x1e8(%rbp)
1422a18df:     	movq	%rbx, 0x1f0(%rbp)
1422a18e6:     	leaq	0xe9537b(%rip), %r8     # 0x143136c68
1422a18ed:     	movl	$0x1, %ecx
1422a18f2:     	movl	$0x13, %edx
1422a18f7:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a18fc:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1901:     	movq	%rbx, 0x200(%rbp)
1422a1908:     	leaq	0xe95359(%rip), %r8     # 0x143136c68
1422a190f:     	movl	$0x1, %ecx
1422a1914:     	movl	$0x14, %edx
1422a1919:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a191e:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1923:     	movq	%r13, 0x1b0(%rbp)
1422a192a:     	movq	%r12, 0x1c8(%rbp)
1422a1931:     	movq	%r14, 0x1e0(%rbp)
1422a1938:     	movq	%rbx, 0x1f8(%rbp)
1422a193f:     	movq	%rdi, 0x200(%rbp)
1422a1946:     	movq	%rsi, 0x1f0(%rbp)
1422a194d:     	leaq	0xe95314(%rip), %r8     # 0x143136c68
1422a1954:     	movl	$0x1, %ecx
1422a1959:     	movl	$0x14, %edx
1422a195e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1963:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1968:     	movq	%rbx, 0x1f8(%rbp)
1422a196f:     	movq	%rdi, 0x200(%rbp)
1422a1976:     	movq	%rsi, 0x1f0(%rbp)
1422a197d:     	leaq	0xe952e4(%rip), %r8     # 0x143136c68
1422a1984:     	movl	$0x1, %ecx
1422a1989:     	movl	$0x17, %edx
1422a198e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1993:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1998:     	movq	%rsi, 0x1f0(%rbp)
1422a199f:     	leaq	0xe952c2(%rip), %r8     # 0x143136c68
1422a19a6:     	movl	$0x1, %ecx
1422a19ab:     	movl	$0x16, %edx
1422a19b0:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a19b5:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a19ba:     	movq	%r12, 0x1c8(%rbp)
1422a19c1:     	movq	%r14, 0x1e0(%rbp)
1422a19c8:     	movq	%rbx, 0x1f8(%rbp)
1422a19cf:     	movq	%rdi, 0x200(%rbp)
1422a19d6:     	movq	%rsi, 0x1f0(%rbp)
1422a19dd:     	leaq	0xe95284(%rip), %r8     # 0x143136c68
1422a19e4:     	movl	$0x1, %ecx
1422a19e9:     	movl	$0x16, %edx
1422a19ee:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a19f3:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a19f8:     	movq	%r13, 0x1e0(%rbp)
1422a19ff:     	movq	%r14, 0x1f8(%rbp)
1422a1a06:     	movq	%rbx, 0x200(%rbp)
1422a1a0d:     	leaq	0xe95254(%rip), %r8     # 0x143136c68
1422a1a14:     	movl	$0x1, %ecx
1422a1a19:     	movl	$0x1d, %edx
1422a1a1e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1a23:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1a28:     	leaq	0xd9ad29(%rip), %r8     # 0x14303c758
1422a1a2f:     	movq	%rax, %rcx
1422a1a32:     	callq	0x1428d9518 <.text+0x28d8518>
1422a1a37:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1a3c:     	leaq	0xd9acb5(%rip), %rcx    # 0x14303c6f8
1422a1a43:     	callq	0x1428d9310 <.text+0x28d8310>
1422a1a48:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1a4d:     	leaq	0xd9ac5c(%rip), %r8     # 0x14303c6b0
1422a1a54:     	movq	%rax, %rcx
1422a1a57:     	callq	0x1428d9518 <.text+0x28d8518>
1422a1a5c:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1a61:     	leaq	0xd9ac60(%rip), %rcx    # 0x14303c6c8
1422a1a68:     	callq	0x1428d9310 <.text+0x28d8310>
1422a1a6d:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1a72:     	leaq	0xd9ad0f(%rip), %rcx    # 0x14303c788
1422a1a79:     	callq	0x1428d9310 <.text+0x28d8310>
1422a1a7e:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1a83:     	leaq	0xd9ac9e(%rip), %r8     # 0x14303c728
1422a1a8a:     	movq	%rax, %rcx
1422a1a8d:     	callq	0x1428d9518 <.text+0x28d8518>
1422a1a92:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1a97:     	movq	%r14, 0x1e8(%rbp)
1422a1a9e:     	leaq	0xe951c3(%rip), %r8     # 0x143136c68
1422a1aa5:     	movl	$0x1, %ecx
1422a1aaa:     	movl	$0x17, %edx
1422a1aaf:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1ab4:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1ab9:     	movl	$0x8, %ecx
1422a1abe:     	movl	$0xc8, %edx
1422a1ac3:     	callq	0x1428d9000 <.text+0x28d8000>
1422a1ac8:     	movq	%r13, 0x200(%rbp)
1422a1acf:     	movq	%r12, 0x1d8(%rbp)
1422a1ad6:     	movq	%r14, 0x1e8(%rbp)
1422a1add:     	leaq	0xe95184(%rip), %r8     # 0x143136c68
1422a1ae4:     	movl	$0x1, %ecx
1422a1ae9:     	movl	$0x17, %edx
1422a1aee:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1af3:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1af8:     	movq	%r13, 0x200(%rbp)
1422a1aff:     	movq	%r12, 0x1d8(%rbp)
1422a1b06:     	movq	%r14, 0x1e8(%rbp)
1422a1b0d:     	leaq	0xe95154(%rip), %r8     # 0x143136c68
1422a1b14:     	movl	$0x1, %ecx
1422a1b19:     	movl	$0x16, %edx
1422a1b1e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1b23:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1b28:     	movq	%r12, 0x1d8(%rbp)
1422a1b2f:     	movq	%r14, 0x1e8(%rbp)
1422a1b36:     	leaq	0xe9512b(%rip), %r8     # 0x143136c68
1422a1b3d:     	movl	$0x1, %ecx
1422a1b42:     	movl	$0x18, %edx
1422a1b47:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1b4c:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1b51:     	movq	%rsi, 0x1f8(%rbp)
1422a1b58:     	movq	%r13, 0x200(%rbp)
1422a1b5f:     	movq	%r12, 0x1d8(%rbp)
1422a1b66:     	movq	%r14, 0x1e8(%rbp)
1422a1b6d:     	leaq	0xe950f4(%rip), %r8     # 0x143136c68
1422a1b74:     	movl	$0x1, %ecx
1422a1b79:     	movl	$0x16, %edx
1422a1b7e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1b83:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1b88:     	movq	%rdi, 0x1e8(%rbp)
1422a1b8f:     	movq	%rsi, 0x1f0(%rbp)
1422a1b96:     	leaq	0xe950cb(%rip), %r8     # 0x143136c68
1422a1b9d:     	movl	$0x1, %ecx
1422a1ba2:     	movl	$0x17, %edx
1422a1ba7:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1bac:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1bb1:     	movq	%r15, 0x1d8(%rbp)
1422a1bb8:     	movq	%rdi, 0x1e8(%rbp)
1422a1bbf:     	movq	%rsi, 0x1f0(%rbp)
1422a1bc6:     	leaq	0xe9509b(%rip), %r8     # 0x143136c68
1422a1bcd:     	movl	$0x1, %ecx
1422a1bd2:     	movl	$0x19, %edx
1422a1bd7:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1bdc:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1be1:     	movq	%r12, 0x200(%rbp)
1422a1be8:     	movq	%r15, 0x1d8(%rbp)
1422a1bef:     	movq	%rdi, 0x1e8(%rbp)
1422a1bf6:     	movq	%rsi, 0x1f0(%rbp)
1422a1bfd:     	leaq	0xe95064(%rip), %r8     # 0x143136c68
1422a1c04:     	movl	$0x1, %ecx
1422a1c09:     	movl	$0x15, %edx
1422a1c0e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1c13:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1c18:     	movq	%r13, 0x1f8(%rbp)
1422a1c1f:     	movq	%r12, 0x200(%rbp)
1422a1c26:     	movq	%r15, 0x1d8(%rbp)
1422a1c2d:     	movq	%rdi, 0x1e8(%rbp)
1422a1c34:     	movq	%rsi, 0x1f0(%rbp)
1422a1c3b:     	leaq	0xe95026(%rip), %r8     # 0x143136c68
1422a1c42:     	movl	$0x1, %ecx
1422a1c47:     	movl	$0x1b, %edx
1422a1c4c:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1c51:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1c56:     	movq	%r14, 0x1e0(%rbp)
1422a1c5d:     	movq	%r13, 0x1f8(%rbp)
1422a1c64:     	movq	%r12, 0x200(%rbp)
1422a1c6b:     	movq	%r15, 0x1d8(%rbp)
1422a1c72:     	movq	%rdi, 0x1e8(%rbp)
1422a1c79:     	movq	%rsi, 0x1f0(%rbp)
1422a1c80:     	leaq	0xe94fe1(%rip), %r8     # 0x143136c68
1422a1c87:     	movl	$0x1, %ecx
1422a1c8c:     	movl	$0x14, %edx
1422a1c91:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1c96:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1c9b:     	movl	$0x8, %ecx
1422a1ca0:     	movl	$0xf0, %edx
1422a1ca5:     	callq	0x1428d9000 <.text+0x28d8000>
1422a1caa:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1caf:     	movq	%rsi, 0x1f0(%rbp)
1422a1cb6:     	leaq	0xe94fab(%rip), %r8     # 0x143136c68
1422a1cbd:     	movl	$0x1, %ecx
1422a1cc2:     	movl	$0x1b, %edx
1422a1cc7:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1ccc:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1cd1:     	movq	%r12, 0x1b0(%rbp)
1422a1cd8:     	movq	%r14, 0x1c8(%rbp)
1422a1cdf:     	movq	%rbx, 0x1e0(%rbp)
1422a1ce6:     	movq	%rdi, 0x1f8(%rbp)
1422a1ced:     	movq	%rsi, 0x200(%rbp)
1422a1cf4:     	leaq	0xe94f6d(%rip), %r8     # 0x143136c68
1422a1cfb:     	movl	$0x1, %ecx
1422a1d00:     	movl	$0x1a, %edx
1422a1d05:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1d0a:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1d0f:     	movq	%r14, 0x1f8(%rbp)
1422a1d16:     	movq	%rbx, 0x200(%rbp)
1422a1d1d:     	leaq	0xe94f44(%rip), %r8     # 0x143136c68
1422a1d24:     	movl	$0x1, %ecx
1422a1d29:     	movl	$0xe, %edx
1422a1d2e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1d33:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1d38:     	movq	%r13, 0x178(%rbp)
1422a1d3f:     	movq	%r12, 0x1b0(%rbp)
1422a1d46:     	movq	%r14, 0x1c8(%rbp)
1422a1d4d:     	movq	%rbx, 0x1e0(%rbp)
1422a1d54:     	movq	%rdi, 0x1f8(%rbp)
1422a1d5b:     	movq	%rsi, 0x200(%rbp)
1422a1d62:     	leaq	0xe94eff(%rip), %r8     # 0x143136c68
1422a1d69:     	movl	$0x1, %ecx
1422a1d6e:     	movl	$0x14, %edx
1422a1d73:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1d78:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1d7d:     	movq	%rdi, 0x178(%rbp)
1422a1d84:     	movq	%rsi, 0x1b0(%rbp)
1422a1d8b:     	movq	%r13, 0x1c8(%rbp)
1422a1d92:     	movq	%r12, 0x1e0(%rbp)
1422a1d99:     	movq	%r14, 0x1f8(%rbp)
1422a1da0:     	movq	%rbx, 0x200(%rbp)
1422a1da7:     	leaq	0xe94eba(%rip), %r8     # 0x143136c68
1422a1dae:     	movl	$0x1, %ecx
1422a1db3:     	movl	$0x11, %edx
1422a1db8:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1dbd:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1dc2:     	movq	%r12, 0x1e0(%rbp)
1422a1dc9:     	movq	%r14, 0x1f8(%rbp)
1422a1dd0:     	movq	%rbx, 0x200(%rbp)
1422a1dd7:     	movq	%rdi, 0x1d8(%rbp)
1422a1dde:     	leaq	0xe94e83(%rip), %r8     # 0x143136c68
1422a1de5:     	movl	$0x1, %ecx
1422a1dea:     	movl	$0x16, %edx
1422a1def:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1df4:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1df9:     	movq	%r13, 0x1b0(%rbp)
1422a1e00:     	movq	%r12, 0x1c8(%rbp)
1422a1e07:     	movq	%r14, 0x1c0(%rbp)
1422a1e0e:     	movq	%rbx, 0x1e0(%rbp)
1422a1e15:     	movq	%rdi, 0x1f8(%rbp)
1422a1e1c:     	movq	%rsi, 0x200(%rbp)
1422a1e23:     	leaq	0xe94e3e(%rip), %r8     # 0x143136c68
1422a1e2a:     	movl	$0x1, %ecx
1422a1e2f:     	movl	$0x11, %edx
1422a1e34:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1e39:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1e3e:     	movq	%r13, 0x178(%rbp)
1422a1e45:     	movq	%r12, 0x1b0(%rbp)
1422a1e4c:     	movq	%r14, 0x1c8(%rbp)
1422a1e53:     	movq	%rbx, 0x1e0(%rbp)
1422a1e5a:     	movq	%rdi, 0x1f8(%rbp)
1422a1e61:     	movq	%rsi, 0x200(%rbp)
1422a1e68:     	leaq	0xe94df9(%rip), %r8     # 0x143136c68
1422a1e6f:     	movl	$0x1, %ecx
1422a1e74:     	movl	$0x14, %edx
1422a1e79:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1e7e:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1e83:     	movq	%rdi, 0x178(%rbp)
1422a1e8a:     	movq	%rsi, 0x1b0(%rbp)
1422a1e91:     	movq	%r13, 0x1c8(%rbp)
1422a1e98:     	movq	%r12, 0x1e0(%rbp)
1422a1e9f:     	movq	%r14, 0x1f8(%rbp)
1422a1ea6:     	movq	%rbx, 0x200(%rbp)
1422a1ead:     	leaq	0xe94db4(%rip), %r8     # 0x143136c68
1422a1eb4:     	movl	$0x1, %ecx
1422a1eb9:     	movl	$0x11, %edx
1422a1ebe:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1ec3:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1ec8:     	movq	%r14, 0x1c8(%rbp)
1422a1ecf:     	movq	%rbx, 0x1e0(%rbp)
1422a1ed6:     	movq	%rdi, 0x1f8(%rbp)
1422a1edd:     	movq	%rsi, 0x200(%rbp)
1422a1ee4:     	leaq	0xe94d7d(%rip), %r8     # 0x143136c68
1422a1eeb:     	movl	$0x1, %ecx
1422a1ef0:     	movl	$0x14, %edx
1422a1ef5:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1efa:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1eff:     	movq	%r13, 0x1c0(%rbp)
1422a1f06:     	movq	%r12, 0x1e0(%rbp)
1422a1f0d:     	movq	%r14, 0x1f8(%rbp)
1422a1f14:     	movq	%rbx, 0x200(%rbp)
1422a1f1b:     	movq	%rdi, 0x1d8(%rbp)
1422a1f22:     	leaq	0xe94d3f(%rip), %r8     # 0x143136c68
1422a1f29:     	movl	$0x1, %ecx
1422a1f2e:     	movl	$0x5, %edx
1422a1f33:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1f38:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1f3d:     	leaq	0xe94d24(%rip), %r8     # 0x143136c68
1422a1f44:     	movl	$0x1, %ecx
1422a1f49:     	movl	$0x5, %edx
1422a1f4e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1f53:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1f58:     	movq	%rbx, 0x200(%rbp)
1422a1f5f:     	leaq	0xe94d02(%rip), %r8     # 0x143136c68
1422a1f66:     	movl	$0x1, %ecx
1422a1f6b:     	movl	$0x16, %edx
1422a1f70:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1f75:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1f7a:     	movl	$0x8, %ecx
1422a1f7f:     	movl	$0x28, %edx
1422a1f84:     	callq	0x1428d9000 <.text+0x28d8000>
1422a1f89:     	movq	%r13, 0x1b0(%rbp)
1422a1f90:     	movq	%r12, 0x1c8(%rbp)
1422a1f97:     	movq	%r14, 0x1c0(%rbp)
1422a1f9e:     	movq	%rbx, 0x1e0(%rbp)
1422a1fa5:     	movq	%rdi, 0x1f8(%rbp)
1422a1fac:     	movq	%rsi, 0x200(%rbp)
1422a1fb3:     	leaq	0xe94cae(%rip), %r8     # 0x143136c68
1422a1fba:     	movl	$0x1, %ecx
1422a1fbf:     	movl	$0x11, %edx
1422a1fc4:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a1fc9:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a1fce:     	movq	%rdi, 0x178(%rbp)
1422a1fd5:     	movq	%rsi, 0x1b0(%rbp)
1422a1fdc:     	movq	%r13, 0x1c8(%rbp)
1422a1fe3:     	movq	%r12, 0x1e0(%rbp)
1422a1fea:     	movq	%r14, 0x1f8(%rbp)
1422a1ff1:     	movq	%rbx, 0x200(%rbp)
1422a1ff8:     	leaq	0xe94c69(%rip), %r8     # 0x143136c68
1422a1fff:     	movl	$0x1, %ecx
1422a2004:     	movl	$0x13, %edx
1422a2009:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a200e:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2013:     	movq	%r14, 0x1f8(%rbp)
1422a201a:     	movq	%rbx, 0x200(%rbp)
1422a2021:     	movq	%rdi, 0x1d8(%rbp)
1422a2028:     	leaq	0xe94c39(%rip), %r8     # 0x143136c68
1422a202f:     	movl	$0x1, %ecx
1422a2034:     	movl	$0x17, %edx
1422a2039:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a203e:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2043:     	movq	%r13, 0x1b0(%rbp)
1422a204a:     	movq	%r12, 0x1c8(%rbp)
1422a2051:     	movq	%r14, 0x1c0(%rbp)
1422a2058:     	movq	%rbx, 0x1e0(%rbp)
1422a205f:     	movq	%rdi, 0x1f8(%rbp)
1422a2066:     	movq	%rsi, 0x200(%rbp)
1422a206d:     	leaq	0xe94bf4(%rip), %r8     # 0x143136c68
1422a2074:     	movl	$0x1, %ecx
1422a2079:     	movl	$0x11, %edx
1422a207e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2083:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2088:     	movq	%rdi, 0x178(%rbp)
1422a208f:     	movq	%rsi, 0x1b0(%rbp)
1422a2096:     	movq	%r13, 0x1c8(%rbp)
1422a209d:     	movq	%r12, 0x1e0(%rbp)
1422a20a4:     	movq	%r14, 0x1f8(%rbp)
1422a20ab:     	movq	%rbx, 0x200(%rbp)
1422a20b2:     	leaq	0xe94baf(%rip), %r8     # 0x143136c68
1422a20b9:     	movl	$0x1, %ecx
1422a20be:     	movl	$0x11, %edx
1422a20c3:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a20c8:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a20cd:     	movq	%rdi, 0x178(%rbp)
1422a20d4:     	movq	%rsi, 0x1b0(%rbp)
1422a20db:     	movq	%r13, 0x1c8(%rbp)
1422a20e2:     	movq	%r12, 0x1e0(%rbp)
1422a20e9:     	movq	%r14, 0x1f8(%rbp)
1422a20f0:     	movq	%rbx, 0x200(%rbp)
1422a20f7:     	leaq	0xe94b6a(%rip), %r8     # 0x143136c68
1422a20fe:     	movl	$0x1, %ecx
1422a2103:     	movl	$0x10, %edx
1422a2108:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a210d:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2112:     	movq	%rbx, 0x200(%rbp)
1422a2119:     	movq	%rdi, 0x1d8(%rbp)
1422a2120:     	leaq	0xe94b41(%rip), %r8     # 0x143136c68
1422a2127:     	movl	$0x1, %ecx
1422a212c:     	movl	$0x18, %edx
1422a2131:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2136:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a213b:     	movq	%r12, 0x1c8(%rbp)
1422a2142:     	movq	%r14, 0x1c0(%rbp)
1422a2149:     	movq	%rbx, 0x1e0(%rbp)
1422a2150:     	movq	%rdi, 0x1f8(%rbp)
1422a2157:     	movq	%rsi, 0x200(%rbp)
1422a215e:     	leaq	0xe94b03(%rip), %r8     # 0x143136c68
1422a2165:     	movl	$0x1, %ecx
1422a216a:     	movl	$0x11, %edx
1422a216f:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2174:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2179:     	movq	%rdi, 0x178(%rbp)
1422a2180:     	movq	%rsi, 0x1b0(%rbp)
1422a2187:     	movq	%r13, 0x1c8(%rbp)
1422a218e:     	movq	%r12, 0x1e0(%rbp)
1422a2195:     	movq	%r14, 0x1f8(%rbp)
1422a219c:     	movq	%rbx, 0x200(%rbp)
1422a21a3:     	leaq	0xe94abe(%rip), %r8     # 0x143136c68
1422a21aa:     	movl	$0x1, %ecx
1422a21af:     	movl	$0x11, %edx
1422a21b4:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a21b9:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a21be:     	movq	%rdi, 0x1d8(%rbp)
1422a21c5:     	leaq	0xe94a9c(%rip), %r8     # 0x143136c68
1422a21cc:     	movl	$0x1, %ecx
1422a21d1:     	movl	$0x17, %edx
1422a21d6:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a21db:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a21e0:     	movq	%r14, 0x1c0(%rbp)
1422a21e7:     	movq	%rbx, 0x1e0(%rbp)
1422a21ee:     	movq	%rdi, 0x1f8(%rbp)
1422a21f5:     	movq	%rsi, 0x200(%rbp)
1422a21fc:     	leaq	0xe94a65(%rip), %r8     # 0x143136c68
1422a2203:     	movl	$0x1, %ecx
1422a2208:     	movl	$0x11, %edx
1422a220d:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2212:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2217:     	movq	%rdi, 0x178(%rbp)
1422a221e:     	movq	%rsi, 0x1b0(%rbp)
1422a2225:     	movq	%r13, 0x1c8(%rbp)
1422a222c:     	movq	%r12, 0x1e0(%rbp)
1422a2233:     	movq	%r14, 0x1f8(%rbp)
1422a223a:     	movq	%rbx, 0x200(%rbp)
1422a2241:     	leaq	0xe94a20(%rip), %r8     # 0x143136c68
1422a2248:     	movl	$0x1, %ecx
1422a224d:     	movl	$0x5, %edx
1422a2252:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2257:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a225c:     	movq	%r13, 0x178(%rbp)
1422a2263:     	movq	%r12, 0x1b0(%rbp)
1422a226a:     	movq	%r14, 0x1c8(%rbp)
1422a2271:     	movq	%rbx, 0x1e0(%rbp)
1422a2278:     	movq	%rdi, 0x1f8(%rbp)
1422a227f:     	movq	%rsi, 0x200(%rbp)
1422a2286:     	leaq	0xe949db(%rip), %r8     # 0x143136c68
1422a228d:     	movl	$0x1, %ecx
1422a2292:     	movl	$0x19, %edx
1422a2297:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a229c:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a22a1:     	movq	%rbx, 0x1e0(%rbp)
1422a22a8:     	movq	%rdi, 0x1f8(%rbp)
1422a22af:     	movq	%rsi, 0x200(%rbp)
1422a22b6:     	leaq	0xe949ab(%rip), %r8     # 0x143136c68
1422a22bd:     	movl	$0x1, %ecx
1422a22c2:     	movl	$0x16, %edx
1422a22c7:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a22cc:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a22d1:     	movq	%rdi, 0x178(%rbp)
1422a22d8:     	movq	%rsi, 0x1b0(%rbp)
1422a22df:     	movq	%r13, 0x1c8(%rbp)
1422a22e6:     	movq	%r12, 0x1e0(%rbp)
1422a22ed:     	movq	%r14, 0x1f8(%rbp)
1422a22f4:     	movq	%rbx, 0x200(%rbp)
1422a22fb:     	leaq	0xe94966(%rip), %r8     # 0x143136c68
1422a2302:     	movl	$0x1, %ecx
1422a2307:     	movl	$0x11, %edx
1422a230c:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2311:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2316:     	movl	$0x8, %ecx
1422a231b:     	movl	$0x1b8, %edx            # imm = 0x1B8
1422a2320:     	callq	0x1428d9000 <.text+0x28d8000>
1422a2325:     	movq	%rsi, 0x200(%rbp)
1422a232c:     	leaq	0xe94935(%rip), %r8     # 0x143136c68
1422a2333:     	movl	$0x1, %ecx
1422a2338:     	movl	$0x15, %edx
1422a233d:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2342:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2347:     	movl	$0x8, %ecx
1422a234c:     	movl	$0x118, %edx            # imm = 0x118
1422a2351:     	callq	0x1428d9000 <.text+0x28d8000>
1422a2356:     	movq	%rbx, 0x1e0(%rbp)
1422a235d:     	movq	%rdi, 0x1f8(%rbp)
1422a2364:     	movq	%rsi, 0x200(%rbp)
1422a236b:     	leaq	0xe948f6(%rip), %r8     # 0x143136c68
1422a2372:     	movl	$0x1, %ecx
1422a2377:     	movl	$0x11, %edx
1422a237c:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2381:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2386:     	movq	%r13, 0x1c0(%rbp)
1422a238d:     	movq	%r12, 0x1e0(%rbp)
1422a2394:     	movq	%r14, 0x1f8(%rbp)
1422a239b:     	movq	%rbx, 0x200(%rbp)
1422a23a2:     	movq	%rdi, 0x1d8(%rbp)
1422a23a9:     	leaq	0xe948b8(%rip), %r8     # 0x143136c68
1422a23b0:     	movl	$0x1, %ecx
1422a23b5:     	movl	$0x16, %edx
1422a23ba:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a23bf:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a23c4:     	movq	%r13, 0x1b0(%rbp)
1422a23cb:     	movq	%r12, 0x1c8(%rbp)
1422a23d2:     	movq	%r14, 0x1c0(%rbp)
1422a23d9:     	movq	%rbx, 0x1e0(%rbp)
1422a23e0:     	movq	%rdi, 0x1f8(%rbp)
1422a23e7:     	movq	%rsi, 0x200(%rbp)
1422a23ee:     	leaq	0xe94873(%rip), %r8     # 0x143136c68
1422a23f5:     	movl	$0x1, %ecx
1422a23fa:     	movl	$0x11, %edx
1422a23ff:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2404:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2409:     	movl	$0x8, %ecx
1422a240e:     	movl	$0x280, %edx            # imm = 0x280
1422a2413:     	callq	0x1428d9000 <.text+0x28d8000>
1422a2418:     	movq	%r13, 0x178(%rbp)
1422a241f:     	movq	%r12, 0x1b0(%rbp)
1422a2426:     	movq	%r14, 0x1c8(%rbp)
1422a242d:     	movq	%rbx, 0x1e0(%rbp)
1422a2434:     	movq	%rdi, 0x1f8(%rbp)
1422a243b:     	movq	%rsi, 0x200(%rbp)
1422a2442:     	leaq	0xe9481f(%rip), %r8     # 0x143136c68
1422a2449:     	movl	$0x1, %ecx
1422a244e:     	movl	$0x15, %edx
1422a2453:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2458:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a245d:     	movq	%rsi, 0x1b0(%rbp)
1422a2464:     	movq	%r13, 0x1c8(%rbp)
1422a246b:     	movq	%r12, 0x1e0(%rbp)
1422a2472:     	movq	%r14, 0x1f8(%rbp)
1422a2479:     	movq	%rbx, 0x200(%rbp)
1422a2480:     	leaq	0xe947e1(%rip), %r8     # 0x143136c68
1422a2487:     	movl	$0x1, %ecx
1422a248c:     	movl	$0x11, %edx
1422a2491:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2496:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a249b:     	movq	%rdi, 0x178(%rbp)
1422a24a2:     	movq	%rsi, 0x1b0(%rbp)
1422a24a9:     	movq	%r13, 0x1c8(%rbp)
1422a24b0:     	movq	%r12, 0x1e0(%rbp)
1422a24b7:     	movq	%r14, 0x1f8(%rbp)
1422a24be:     	movq	%rbx, 0x200(%rbp)
1422a24c5:     	leaq	0xe9479c(%rip), %r8     # 0x143136c68
1422a24cc:     	movl	$0x1, %ecx
1422a24d1:     	movl	$0x11, %edx
1422a24d6:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a24db:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a24e0:     	movq	%rdi, 0x1f8(%rbp)
1422a24e7:     	movq	%rsi, 0x200(%rbp)
1422a24ee:     	leaq	0xe94773(%rip), %r8     # 0x143136c68
1422a24f5:     	movl	$0x1, %ecx
1422a24fa:     	movl	$0x13, %edx
1422a24ff:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2504:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2509:     	movq	%rdi, 0x1f8(%rbp)
1422a2510:     	movq	%rsi, 0x200(%rbp)
1422a2517:     	leaq	0xe9474a(%rip), %r8     # 0x143136c68
1422a251e:     	movl	$0x1, %ecx
1422a2523:     	movl	$0x15, %edx
1422a2528:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a252d:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2532:     	movq	%r13, 0x178(%rbp)
1422a2539:     	movq	%r12, 0x1b0(%rbp)
1422a2540:     	movq	%r14, 0x1c8(%rbp)
1422a2547:     	movq	%rbx, 0x1e0(%rbp)
1422a254e:     	movq	%rdi, 0x1f8(%rbp)
1422a2555:     	movq	%rsi, 0x200(%rbp)
1422a255c:     	leaq	0xe94705(%rip), %r8     # 0x143136c68
1422a2563:     	movl	$0x1, %ecx
1422a2568:     	movl	$0x1b, %edx
1422a256d:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2572:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2577:     	movq	%r13, 0x1c8(%rbp)
1422a257e:     	movq	%r12, 0x1e0(%rbp)
1422a2585:     	movq	%r14, 0x1f8(%rbp)
1422a258c:     	movq	%rbx, 0x200(%rbp)
1422a2593:     	leaq	0xe946ce(%rip), %r8     # 0x143136c68
1422a259a:     	movl	$0x1, %ecx
1422a259f:     	movl	$0x11, %edx
1422a25a4:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a25a9:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a25ae:     	movq	%rdi, 0x178(%rbp)
1422a25b5:     	movq	%rsi, 0x1b0(%rbp)
1422a25bc:     	movq	%r13, 0x1c8(%rbp)
1422a25c3:     	movq	%r12, 0x1e0(%rbp)
1422a25ca:     	movq	%r14, 0x1f8(%rbp)
1422a25d1:     	movq	%rbx, 0x200(%rbp)
1422a25d8:     	leaq	0xe94689(%rip), %r8     # 0x143136c68
1422a25df:     	movl	$0x1, %ecx
1422a25e4:     	movl	$0x11, %edx
1422a25e9:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a25ee:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a25f3:     	movq	%rsi, 0x200(%rbp)
1422a25fa:     	leaq	0xe94667(%rip), %r8     # 0x143136c68
1422a2601:     	movl	$0x1, %ecx
1422a2606:     	movl	$0x13, %edx
1422a260b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2610:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2615:     	movq	%r13, 0x178(%rbp)
1422a261c:     	movq	%r12, 0x1b0(%rbp)
1422a2623:     	movq	%r14, 0x1c8(%rbp)
1422a262a:     	movq	%rbx, 0x1e0(%rbp)
1422a2631:     	movq	%rdi, 0x1f8(%rbp)
1422a2638:     	movq	%rsi, 0x200(%rbp)
1422a263f:     	leaq	0xe94622(%rip), %r8     # 0x143136c68
1422a2646:     	movl	$0x1, %ecx
1422a264b:     	movl	$0x19, %edx
1422a2650:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2655:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a265a:     	movq	%r12, 0x1e0(%rbp)
1422a2661:     	movq	%r14, 0x1f8(%rbp)
1422a2668:     	movq	%rbx, 0x200(%rbp)
1422a266f:     	leaq	0xe945f2(%rip), %r8     # 0x143136c68
1422a2676:     	movl	$0x1, %ecx
1422a267b:     	movl	$0xe, %edx
1422a2680:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2685:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a268a:     	movq	%rdi, 0x178(%rbp)
1422a2691:     	movq	%rsi, 0x1b0(%rbp)
1422a2698:     	movq	%r13, 0x1c8(%rbp)
1422a269f:     	movq	%r12, 0x1e0(%rbp)
1422a26a6:     	movq	%r14, 0x1f8(%rbp)
1422a26ad:     	movq	%rbx, 0x200(%rbp)
1422a26b4:     	leaq	0xe945ad(%rip), %r8     # 0x143136c68
1422a26bb:     	movl	$0x1, %ecx
1422a26c0:     	movl	$0x11, %edx
1422a26c5:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a26ca:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a26cf:     	movq	%r13, 0x1e0(%rbp)
1422a26d6:     	movq	%r14, 0x1f8(%rbp)
1422a26dd:     	movq	%rbx, 0x200(%rbp)
1422a26e4:     	leaq	0xe9457d(%rip), %r8     # 0x143136c68
1422a26eb:     	movl	$0x1, %ecx
1422a26f0:     	movl	$0xe, %edx
1422a26f5:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a26fa:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a26ff:     	movq	%r13, 0x1e0(%rbp)
1422a2706:     	movq	%r14, 0x1f8(%rbp)
1422a270d:     	movq	%rbx, 0x200(%rbp)
1422a2714:     	leaq	0xe9454d(%rip), %r8     # 0x143136c68
1422a271b:     	movl	$0x1, %ecx
1422a2720:     	movl	$0xe, %edx
1422a2725:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a272a:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a272f:     	movq	%r13, 0x1e0(%rbp)
1422a2736:     	movq	%r14, 0x1f8(%rbp)
1422a273d:     	movq	%rbx, 0x200(%rbp)
1422a2744:     	leaq	0xe9451d(%rip), %r8     # 0x143136c68
1422a274b:     	movl	$0x1, %ecx
1422a2750:     	movl	$0xe, %edx
1422a2755:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a275a:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a275f:     	movq	%r13, 0x1e0(%rbp)
1422a2766:     	movq	%r14, 0x1f8(%rbp)
1422a276d:     	movq	%rbx, 0x200(%rbp)
1422a2774:     	leaq	0xe944ed(%rip), %r8     # 0x143136c68
1422a277b:     	movl	$0x1, %ecx
1422a2780:     	movl	$0x16, %edx
1422a2785:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a278a:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a278f:     	leaq	0xd99a3a(%rip), %r8     # 0x14303c1d0
1422a2796:     	movq	%rax, %rcx
1422a2799:     	callq	0x1428d9518 <.text+0x28d8518>
1422a279e:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a27a3:     	movl	$0x8, %ecx
1422a27a8:     	movl	$0x208, %edx            # imm = 0x208
1422a27ad:     	callq	0x1428d9000 <.text+0x28d8000>
1422a27b2:     	movq	%rbx, 0x200(%rbp)
1422a27b9:     	leaq	0xe944a8(%rip), %r8     # 0x143136c68
1422a27c0:     	movl	$0x1, %ecx
1422a27c5:     	movl	$0x17, %edx
1422a27ca:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a27cf:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a27d4:     	movq	%r14, 0x1f8(%rbp)
1422a27db:     	movq	%rbx, 0x200(%rbp)
1422a27e2:     	leaq	0xe9447f(%rip), %r8     # 0x143136c68
1422a27e9:     	movl	$0x1, %ecx
1422a27ee:     	movl	$0x18, %edx
1422a27f3:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a27f8:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a27fd:     	movq	%r13, 0x1e0(%rbp)
1422a2804:     	movq	%r14, 0x1f8(%rbp)
1422a280b:     	movq	%rbx, 0x200(%rbp)
1422a2812:     	leaq	0xe9444f(%rip), %r8     # 0x143136c68
1422a2819:     	movl	$0x1, %ecx
1422a281e:     	movl	$0x17, %edx
1422a2823:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2828:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a282d:     	movq	%r13, 0x1e0(%rbp)
1422a2834:     	movq	%r14, 0x1f8(%rbp)
1422a283b:     	movq	%rbx, 0x200(%rbp)
1422a2842:     	leaq	0xe9441f(%rip), %r8     # 0x143136c68
1422a2849:     	movl	$0x1, %ecx
1422a284e:     	movl	$0x16, %edx
1422a2853:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2858:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a285d:     	movq	%r13, 0x1e0(%rbp)
1422a2864:     	movq	%r14, 0x1f8(%rbp)
1422a286b:     	movq	%rbx, 0x200(%rbp)
1422a2872:     	leaq	0xe943ef(%rip), %r8     # 0x143136c68
1422a2879:     	movl	$0x1, %ecx
1422a287e:     	movl	$0xe, %edx
1422a2883:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2888:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a288d:     	movq	%r13, 0x1e0(%rbp)
1422a2894:     	movq	%r14, 0x1f8(%rbp)
1422a289b:     	movq	%rbx, 0x200(%rbp)
1422a28a2:     	leaq	0xe943bf(%rip), %r8     # 0x143136c68
1422a28a9:     	movl	$0x1, %ecx
1422a28ae:     	movl	$0xe, %edx
1422a28b3:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a28b8:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a28bd:     	movq	%r13, 0x1e0(%rbp)
1422a28c4:     	movq	%r14, 0x1f8(%rbp)
1422a28cb:     	movq	%rbx, 0x200(%rbp)
1422a28d2:     	leaq	0xe9438f(%rip), %r8     # 0x143136c68
1422a28d9:     	movl	$0x1, %ecx
1422a28de:     	movl	$0xe, %edx
1422a28e3:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a28e8:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a28ed:     	movq	%r12, 0x1c8(%rbp)
1422a28f4:     	movq	%r13, 0x1e0(%rbp)
1422a28fb:     	movq	%r14, 0x1f8(%rbp)
1422a2902:     	movq	%rbx, 0x200(%rbp)
1422a2909:     	leaq	0xe94358(%rip), %r8     # 0x143136c68
1422a2910:     	movl	$0x1, %ecx
1422a2915:     	movl	$0xe, %edx
1422a291a:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a291f:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2924:     	movq	%r13, 0x1e0(%rbp)
1422a292b:     	movq	%r14, 0x1f8(%rbp)
1422a2932:     	movq	%rbx, 0x200(%rbp)
1422a2939:     	leaq	0xe94328(%rip), %r8     # 0x143136c68
1422a2940:     	movl	$0x1, %ecx
1422a2945:     	movl	$0xe, %edx
1422a294a:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a294f:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2954:     	movq	%r12, 0x200(%rbp)
1422a295b:     	movq	%rbx, 0x1d8(%rbp)
1422a2962:     	movq	%rdi, 0x1e8(%rbp)
1422a2969:     	movq	%rsi, 0x1f0(%rbp)
1422a2970:     	leaq	0xe942f1(%rip), %r8     # 0x143136c68
1422a2977:     	movl	$0x1, %ecx
1422a297c:     	movl	$0x1c, %edx
1422a2981:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2986:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a298b:     	movq	%rdi, 0x1e8(%rbp)
1422a2992:     	movq	%rsi, 0x1f0(%rbp)
1422a2999:     	leaq	0xe942c8(%rip), %r8     # 0x143136c68
1422a29a0:     	movl	$0x1, %ecx
1422a29a5:     	movl	$0x1b, %edx
1422a29aa:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a29af:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a29b4:     	movq	%rbx, 0x1d8(%rbp)
1422a29bb:     	movq	%rdi, 0x1e8(%rbp)
1422a29c2:     	movq	%rsi, 0x1f0(%rbp)
1422a29c9:     	leaq	0xe94298(%rip), %r8     # 0x143136c68
1422a29d0:     	movl	$0x1, %ecx
1422a29d5:     	movl	$0x1e, %edx
1422a29da:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a29df:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a29e4:     	movq	%rsi, 0x1f0(%rbp)
1422a29eb:     	leaq	0xe94276(%rip), %r8     # 0x143136c68
1422a29f2:     	movl	$0x1, %ecx
1422a29f7:     	movl	$0x5, %edx
1422a29fc:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2a01:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2a06:     	movl	$0x8, %ecx
1422a2a0b:     	movl	$0xa0, %edx
1422a2a10:     	callq	0x1428d9000 <.text+0x28d8000>
1422a2a15:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2a1a:     	leaq	0xe94247(%rip), %r8     # 0x143136c68
1422a2a21:     	movl	$0x1, %ecx
1422a2a26:     	movl	$0x5, %edx
1422a2a2b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2a30:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2a35:     	leaq	0xe9422c(%rip), %r8     # 0x143136c68
1422a2a3c:     	movl	$0x1, %ecx
1422a2a41:     	movl	$0x1b, %edx
1422a2a46:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2a4b:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2a50:     	movq	%rdi, 0x1d8(%rbp)
1422a2a57:     	leaq	0xe9420a(%rip), %r8     # 0x143136c68
1422a2a5e:     	movl	$0x1, %ecx
1422a2a63:     	movl	$0x21, %edx
1422a2a68:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2a6d:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2a72:     	movq	%rdi, 0x1d8(%rbp)
1422a2a79:     	leaq	0xe941e8(%rip), %r8     # 0x143136c68
1422a2a80:     	movl	$0x1, %ecx
1422a2a85:     	movl	$0x1f, %edx
1422a2a8a:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2a8f:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2a94:     	movq	%r13, 0x1c0(%rbp)
1422a2a9b:     	movq	%r12, 0x1e0(%rbp)
1422a2aa2:     	movq	%r14, 0x1f8(%rbp)
1422a2aa9:     	movq	%rbx, 0x200(%rbp)
1422a2ab0:     	movq	%rdi, 0x1d8(%rbp)
1422a2ab7:     	leaq	0xe941aa(%rip), %r8     # 0x143136c68
1422a2abe:     	movl	$0x1, %ecx
1422a2ac3:     	movl	$0x21, %edx
1422a2ac8:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2acd:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2ad2:     	leaq	0xe9418f(%rip), %r8     # 0x143136c68
1422a2ad9:     	movl	$0x1, %ecx
1422a2ade:     	movl	$0x1d, %edx
1422a2ae3:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2ae8:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2aed:     	leaq	0xe94174(%rip), %r8     # 0x143136c68
1422a2af4:     	movl	$0x1, %ecx
1422a2af9:     	movl	$0x1a, %edx
1422a2afe:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2b03:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2b08:     	movq	%r13, 0x1c0(%rbp)
1422a2b0f:     	movq	%r12, 0x1e0(%rbp)
1422a2b16:     	movq	%r14, 0x1f8(%rbp)
1422a2b1d:     	movq	%rbx, 0x200(%rbp)
1422a2b24:     	movq	%rdi, 0x1d8(%rbp)
1422a2b2b:     	leaq	0xe94136(%rip), %r8     # 0x143136c68
1422a2b32:     	movl	$0x1, %ecx
1422a2b37:     	movl	$0x1f, %edx
1422a2b3c:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2b41:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2b46:     	movq	%rdi, 0x1e8(%rbp)
1422a2b4d:     	movq	%rsi, 0x1f0(%rbp)
1422a2b54:     	leaq	0xe9410d(%rip), %r8     # 0x143136c68
1422a2b5b:     	movl	$0x1, %ecx
1422a2b60:     	movl	$0x15, %edx
1422a2b65:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2b6a:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2b6f:     	movq	%rsi, 0x1f0(%rbp)
1422a2b76:     	leaq	0xe940eb(%rip), %r8     # 0x143136c68
1422a2b7d:     	movl	$0x1, %ecx
1422a2b82:     	movl	$0x5, %edx
1422a2b87:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2b8c:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2b91:     	movl	$0x8, %ecx
1422a2b96:     	movl	$0xa0, %edx
1422a2b9b:     	callq	0x1428d9000 <.text+0x28d8000>
1422a2ba0:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2ba5:     	movq	%r14, 0x200(%rbp)
1422a2bac:     	movq	%rbx, 0x1d8(%rbp)
1422a2bb3:     	movq	%rdi, 0x1e8(%rbp)
1422a2bba:     	movq	%rsi, 0x1f0(%rbp)
1422a2bc1:     	leaq	0xe940a0(%rip), %r8     # 0x143136c68
1422a2bc8:     	movl	$0x1, %ecx
1422a2bcd:     	movl	$0x13, %edx
1422a2bd2:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2bd7:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2bdc:     	leaq	0xe94085(%rip), %r8     # 0x143136c68
1422a2be3:     	movl	$0x1, %ecx
1422a2be8:     	movl	$0x10, %edx
1422a2bed:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2bf2:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2bf7:     	movq	%rbx, 0x1d8(%rbp)
1422a2bfe:     	movq	%rdi, 0x1e8(%rbp)
1422a2c05:     	movq	%rsi, 0x1f0(%rbp)
1422a2c0c:     	leaq	0xe94055(%rip), %r8     # 0x143136c68
1422a2c13:     	movl	$0x1, %ecx
1422a2c18:     	movl	$0x10, %edx
1422a2c1d:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2c22:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2c27:     	leaq	0xe9403a(%rip), %r8     # 0x143136c68
1422a2c2e:     	movl	$0x1, %ecx
1422a2c33:     	movl	$0x5, %edx
1422a2c38:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2c3d:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2c42:     	leaq	0xe9401f(%rip), %r8     # 0x143136c68
1422a2c49:     	movl	$0x1, %ecx
1422a2c4e:     	movl	$0x5, %edx
1422a2c53:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2c58:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2c5d:     	movl	$0x8, %ecx
1422a2c62:     	movl	$0xc8, %edx
1422a2c67:     	callq	0x1428d9000 <.text+0x28d8000>
1422a2c6c:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2c71:     	leaq	0xe93ff0(%rip), %r8     # 0x143136c68
1422a2c78:     	movl	$0x1, %ecx
1422a2c7d:     	movl	$0x13, %edx
1422a2c82:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2c87:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2c8c:     	leaq	0xe93fd5(%rip), %r8     # 0x143136c68
1422a2c93:     	movl	$0x1, %ecx
1422a2c98:     	movl	$0x1f, %edx
1422a2c9d:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2ca2:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2ca7:     	leaq	0xe93fba(%rip), %r8     # 0x143136c68
1422a2cae:     	movl	$0x1, %ecx
1422a2cb3:     	movl	$0x1e, %edx
1422a2cb8:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2cbd:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2cc2:     	movq	%r13, 0x1f8(%rbp)
1422a2cc9:     	movq	%rsi, 0x200(%rbp)
1422a2cd0:     	movq	%r14, 0x1e8(%rbp)
1422a2cd7:     	leaq	0xe93f8a(%rip), %r8     # 0x143136c68
1422a2cde:     	movl	$0x1, %ecx
1422a2ce3:     	movl	$0x19, %edx
1422a2ce8:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2ced:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2cf2:     	movq	%r13, 0x1f8(%rbp)
1422a2cf9:     	movq	%rsi, 0x200(%rbp)
1422a2d00:     	movq	%r14, 0x1e8(%rbp)
1422a2d07:     	leaq	0xe93f5a(%rip), %r8     # 0x143136c68
1422a2d0e:     	movl	$0x1, %ecx
1422a2d13:     	movl	$0xf, %edx
1422a2d18:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2d1d:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2d22:     	movq	%r13, 0x1f8(%rbp)
1422a2d29:     	movq	%rsi, 0x200(%rbp)
1422a2d30:     	movq	%r14, 0x1e8(%rbp)
1422a2d37:     	leaq	0xe93f2a(%rip), %r8     # 0x143136c68
1422a2d3e:     	movl	$0x1, %ecx
1422a2d43:     	movl	$0x13, %edx
1422a2d48:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2d4d:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2d52:     	movq	%r13, 0x1f8(%rbp)
1422a2d59:     	movq	%rsi, 0x200(%rbp)
1422a2d60:     	movq	%r14, 0x1e8(%rbp)
1422a2d67:     	leaq	0xe93efa(%rip), %r8     # 0x143136c68
1422a2d6e:     	movl	$0x1, %ecx
1422a2d73:     	movl	$0x14, %edx
1422a2d78:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2d7d:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2d82:     	movq	%r14, 0x1e8(%rbp)
1422a2d89:     	leaq	0xe93ed8(%rip), %r8     # 0x143136c68
1422a2d90:     	movl	$0x1, %ecx
1422a2d95:     	movl	$0x18, %edx
1422a2d9a:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2d9f:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2da4:     	movq	%rsi, 0x200(%rbp)
1422a2dab:     	movq	%r14, 0x1e8(%rbp)
1422a2db2:     	leaq	0xe93eaf(%rip), %r8     # 0x143136c68
1422a2db9:     	movl	$0x1, %ecx
1422a2dbe:     	movl	$0x17, %edx
1422a2dc3:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2dc8:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2dcd:     	movq	%r13, 0x1f8(%rbp)
1422a2dd4:     	movq	%rsi, 0x200(%rbp)
1422a2ddb:     	movq	%r14, 0x1e8(%rbp)
1422a2de2:     	leaq	0xe93e7f(%rip), %r8     # 0x143136c68
1422a2de9:     	movl	$0x1, %ecx
1422a2dee:     	movl	$0x1c, %edx
1422a2df3:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2df8:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2dfd:     	leaq	0xd99bc4(%rip), %rcx    # 0x14303c9c8
1422a2e04:     	callq	0x1428d9310 <.text+0x28d8310>
1422a2e09:     	movq	%r13, 0x1f8(%rbp)
1422a2e10:     	movq	%rsi, 0x200(%rbp)
1422a2e17:     	movq	%r14, 0x1e8(%rbp)
1422a2e1e:     	leaq	0xe93e43(%rip), %r8     # 0x143136c68
1422a2e25:     	movl	$0x1, %ecx
1422a2e2a:     	movl	$0x14, %edx
1422a2e2f:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2e34:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2e36:     	movq	%r13, 0x1f8(%rbp)
1422a2e3d:     	movq	%rsi, 0x200(%rbp)
1422a2e44:     	movq	%r14, 0x1e8(%rbp)
1422a2e4b:     	leaq	0xe93e16(%rip), %r8     # 0x143136c68
1422a2e52:     	movl	$0x1, %ecx
1422a2e57:     	movl	$0x1b, %edx
1422a2e5c:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2e61:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2e63:     	movq	%r12, 0x1e0(%rbp)
1422a2e6a:     	movq	%r13, 0x1f8(%rbp)
1422a2e71:     	movq	%rsi, 0x200(%rbp)
1422a2e78:     	movq	%r14, 0x1e8(%rbp)
1422a2e7f:     	leaq	0xe93de2(%rip), %r8     # 0x143136c68
1422a2e86:     	movl	$0x1, %ecx
1422a2e8b:     	movl	$0x15, %edx
1422a2e90:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422a2e95:     	jmp	0x1422a2ea6 <.text+0x22a1ea6>
1422a2e97:     	movl	$0x8, %ecx
1422a2e9c:     	movl	$0x50, %edx
1422a2ea1:     	callq	0x1428d9000 <.text+0x28d8000>
1422a2ea6:     	ud2
1422a2ea8:     	nopl	(%rax,%rax)
1422a2eb0:     	movq	%rdx, 0x10(%rsp)
1422a2eb5:     	pushq	%rbp
1422a2eb6:     	pushq	%r15
1422a2eb8:     	pushq	%r14
1422a2eba:     	pushq	%r13
1422a2ebc:     	pushq	%r12
1422a2ebe:     	pushq	%rsi
1422a2ebf:     	pushq	%rdi
1422a2ec0:     	pushq	%rbx
1422a2ec1:     	subq	$0x108, %rsp            # imm = 0x108
1422a2ec8:     	leaq	0x80(%rdx), %rbp
1422a2ecf:     	movaps	%xmm15, 0x60(%rsp)
1422a2ed5:     	movaps	%xmm14, 0x70(%rsp)
1422a2edb:     	movaps	%xmm13, 0x80(%rsp)
1422a2ee4:     	movaps	%xmm12, 0x90(%rsp)
1422a2eed:     	movaps	%xmm11, 0xa0(%rsp)
1422a2ef6:     	movaps	%xmm10, 0xb0(%rsp)
1422a2eff:     	movaps	%xmm9, 0xc0(%rsp)
1422a2f08:     	movaps	%xmm8, 0xd0(%rsp)
1422a2f11:     	movaps	%xmm7, 0xe0(%rsp)
1422a2f19:     	movaps	%xmm6, 0xf0(%rsp)
1422a2f21:     	leaq	0x180(%rbp), %rcx
1422a2f28:     	callq	0x14087b030 <.text+0x87a030>
1422a2f2d:     	movaps	0xf0(%rsp), %xmm6
1422a2f35:     	movaps	0xe0(%rsp), %xmm7
1422a2f3d:     	movaps	0xd0(%rsp), %xmm8
1422a2f46:     	movaps	0xc0(%rsp), %xmm9
1422a2f4f:     	movaps	0xb0(%rsp), %xmm10
1422a2f58:     	movaps	0xa0(%rsp), %xmm11
1422a2f61:     	movaps	0x90(%rsp), %xmm12
1422a2f6a:     	movaps	0x80(%rsp), %xmm13
1422a2f73:     	movaps	0x70(%rsp), %xmm14
1422a2f79:     	movaps	0x60(%rsp), %xmm15
1422a2f7f:     	addq	$0x108, %rsp            # imm = 0x108
1422a2f86:     	popq	%rbx
1422a2f87:     	popq	%rdi
1422a2f88:     	popq	%rsi
1422a2f89:     	popq	%r12
1422a2f8b:     	popq	%r13
1422a2f8d:     	popq	%r14
1422a2f8f:     	popq	%r15
1422a2f91:     	popq	%rbp
1422a2f92:     	retq
1422a2f93:     	nopw	%cs:(%rax,%rax)
1422a2fa0:     	movq	%rdx, 0x10(%rsp)
1422a2fa5:     	pushq	%rbp
1422a2fa6:     	pushq	%r15
1422a2fa8:     	pushq	%r14
1422a2faa:     	pushq	%r13
1422a2fac:     	pushq	%r12
1422a2fae:     	pushq	%rsi
1422a2faf:     	pushq	%rdi
1422a2fb0:     	pushq	%rbx
1422a2fb1:     	subq	$0x108, %rsp            # imm = 0x108
1422a2fb8:     	leaq	0x80(%rdx), %rbp
1422a2fbf:     	movaps	%xmm15, 0x60(%rsp)
1422a2fc5:     	movaps	%xmm14, 0x70(%rsp)
1422a2fcb:     	movaps	%xmm13, 0x80(%rsp)
1422a2fd4:     	movaps	%xmm12, 0x90(%rsp)
1422a2fdd:     	movaps	%xmm11, 0xa0(%rsp)
1422a2fe6:     	movaps	%xmm10, 0xb0(%rsp)
1422a2fef:     	movaps	%xmm9, 0xc0(%rsp)
1422a2ff8:     	movaps	%xmm8, 0xd0(%rsp)
1422a3001:     	movaps	%xmm7, 0xe0(%rsp)
1422a3009:     	movaps	%xmm6, 0xf0(%rsp)
1422a3011:     	leaq	0x180(%rbp), %rcx
1422a3018:     	callq	0x14087b030 <.text+0x87a030>
1422a301d:     	movaps	0xf0(%rsp), %xmm6
1422a3025:     	movaps	0xe0(%rsp), %xmm7
1422a302d:     	movaps	0xd0(%rsp), %xmm8
1422a3036:     	movaps	0xc0(%rsp), %xmm9
1422a303f:     	movaps	0xb0(%rsp), %xmm10
1422a3048:     	movaps	0xa0(%rsp), %xmm11
1422a3051:     	movaps	0x90(%rsp), %xmm12
1422a305a:     	movaps	0x80(%rsp), %xmm13
1422a3063:     	movaps	0x70(%rsp), %xmm14
1422a3069:     	movaps	0x60(%rsp), %xmm15
1422a306f:     	addq	$0x108, %rsp            # imm = 0x108
1422a3076:     	popq	%rbx
1422a3077:     	popq	%rdi
1422a3078:     	popq	%rsi
1422a3079:     	popq	%r12
1422a307b:     	popq	%r13
1422a307d:     	popq	%r14
1422a307f:     	popq	%r15
1422a3081:     	popq	%rbp
1422a3082:     	retq
1422a3083:     	nopw	%cs:(%rax,%rax)
1422a3090:     	movq	%rdx, 0x10(%rsp)
1422a3095:     	pushq	%rbp
1422a3096:     	pushq	%r15
1422a3098:     	pushq	%r14
1422a309a:     	pushq	%r13
1422a309c:     	pushq	%r12
1422a309e:     	pushq	%rsi
1422a309f:     	pushq	%rdi
1422a30a0:     	pushq	%rbx
1422a30a1:     	subq	$0x108, %rsp            # imm = 0x108
1422a30a8:     	leaq	0x80(%rdx), %rbp
1422a30af:     	movaps	%xmm15, 0x60(%rsp)
1422a30b5:     	movaps	%xmm14, 0x70(%rsp)
1422a30bb:     	movaps	%xmm13, 0x80(%rsp)
1422a30c4:     	movaps	%xmm12, 0x90(%rsp)
1422a30cd:     	movaps	%xmm11, 0xa0(%rsp)
1422a30d6:     	movaps	%xmm10, 0xb0(%rsp)
1422a30df:     	movaps	%xmm9, 0xc0(%rsp)
1422a30e8:     	movaps	%xmm8, 0xd0(%rsp)
1422a30f1:     	movaps	%xmm7, 0xe0(%rsp)
1422a30f9:     	movaps	%xmm6, 0xf0(%rsp)
1422a3101:     	leaq	0x180(%rbp), %rcx
1422a3108:     	callq	0x14087b030 <.text+0x87a030>
1422a310d:     	movaps	0xf0(%rsp), %xmm6
1422a3115:     	movaps	0xe0(%rsp), %xmm7
1422a311d:     	movaps	0xd0(%rsp), %xmm8
1422a3126:     	movaps	0xc0(%rsp), %xmm9
1422a312f:     	movaps	0xb0(%rsp), %xmm10
1422a3138:     	movaps	0xa0(%rsp), %xmm11
1422a3141:     	movaps	0x90(%rsp), %xmm12
1422a314a:     	movaps	0x80(%rsp), %xmm13
1422a3153:     	movaps	0x70(%rsp), %xmm14
1422a3159:     	movaps	0x60(%rsp), %xmm15
1422a315f:     	addq	$0x108, %rsp            # imm = 0x108
1422a3166:     	popq	%rbx
1422a3167:     	popq	%rdi
1422a3168:     	popq	%rsi
1422a3169:     	popq	%r12
1422a316b:     	popq	%r13
1422a316d:     	popq	%r14
1422a316f:     	popq	%r15
1422a3171:     	popq	%rbp
1422a3172:     	retq
1422a3173:     	nopw	%cs:(%rax,%rax)
1422a3180:     	movq	%rdx, 0x10(%rsp)
1422a3185:     	pushq	%rbp
1422a3186:     	pushq	%r15
1422a3188:     	pushq	%r14
1422a318a:     	pushq	%r13
1422a318c:     	pushq	%r12
1422a318e:     	pushq	%rsi
1422a318f:     	pushq	%rdi
1422a3190:     	pushq	%rbx
1422a3191:     	subq	$0x108, %rsp            # imm = 0x108
1422a3198:     	leaq	0x80(%rdx), %rbp
1422a319f:     	movaps	%xmm15, 0x60(%rsp)
1422a31a5:     	movaps	%xmm14, 0x70(%rsp)
1422a31ab:     	movaps	%xmm13, 0x80(%rsp)
1422a31b4:     	movaps	%xmm12, 0x90(%rsp)
1422a31bd:     	movaps	%xmm11, 0xa0(%rsp)
1422a31c6:     	movaps	%xmm10, 0xb0(%rsp)
1422a31cf:     	movaps	%xmm9, 0xc0(%rsp)
1422a31d8:     	movaps	%xmm8, 0xd0(%rsp)
1422a31e1:     	movaps	%xmm7, 0xe0(%rsp)
1422a31e9:     	movaps	%xmm6, 0xf0(%rsp)
1422a31f1:     	leaq	0x180(%rbp), %rcx
1422a31f8:     	callq	0x14087b030 <.text+0x87a030>
1422a31fd:     	movaps	0xf0(%rsp), %xmm6
1422a3205:     	movaps	0xe0(%rsp), %xmm7
1422a320d:     	movaps	0xd0(%rsp), %xmm8
1422a3216:     	movaps	0xc0(%rsp), %xmm9
1422a321f:     	movaps	0xb0(%rsp), %xmm10
1422a3228:     	movaps	0xa0(%rsp), %xmm11
1422a3231:     	movaps	0x90(%rsp), %xmm12
1422a323a:     	movaps	0x80(%rsp), %xmm13
1422a3243:     	movaps	0x70(%rsp), %xmm14
1422a3249:     	movaps	0x60(%rsp), %xmm15
1422a324f:     	addq	$0x108, %rsp            # imm = 0x108
1422a3256:     	popq	%rbx
1422a3257:     	popq	%rdi
1422a3258:     	popq	%rsi
1422a3259:     	popq	%r12
1422a325b:     	popq	%r13
1422a325d:     	popq	%r14
1422a325f:     	popq	%r15
1422a3261:     	popq	%rbp
1422a3262:     	retq
1422a3263:     	nopw	%cs:(%rax,%rax)
1422a3270:     	movq	%rdx, 0x10(%rsp)
1422a3275:     	pushq	%rbp
1422a3276:     	pushq	%r15
1422a3278:     	pushq	%r14
1422a327a:     	pushq	%r13
1422a327c:     	pushq	%r12
1422a327e:     	pushq	%rsi
1422a327f:     	pushq	%rdi
1422a3280:     	pushq	%rbx
1422a3281:     	subq	$0x108, %rsp            # imm = 0x108
1422a3288:     	leaq	0x80(%rdx), %rbp
1422a328f:     	movaps	%xmm15, 0x60(%rsp)
1422a3295:     	movaps	%xmm14, 0x70(%rsp)
1422a329b:     	movaps	%xmm13, 0x80(%rsp)
1422a32a4:     	movaps	%xmm12, 0x90(%rsp)
1422a32ad:     	movaps	%xmm11, 0xa0(%rsp)
1422a32b6:     	movaps	%xmm10, 0xb0(%rsp)
1422a32bf:     	movaps	%xmm9, 0xc0(%rsp)
1422a32c8:     	movaps	%xmm8, 0xd0(%rsp)
1422a32d1:     	movaps	%xmm7, 0xe0(%rsp)
1422a32d9:     	movaps	%xmm6, 0xf0(%rsp)
1422a32e1:     	leaq	0x180(%rbp), %rcx
1422a32e8:     	callq	0x14087b030 <.text+0x87a030>
1422a32ed:     	movaps	0xf0(%rsp), %xmm6
1422a32f5:     	movaps	0xe0(%rsp), %xmm7
1422a32fd:     	movaps	0xd0(%rsp), %xmm8
1422a3306:     	movaps	0xc0(%rsp), %xmm9
1422a330f:     	movaps	0xb0(%rsp), %xmm10
1422a3318:     	movaps	0xa0(%rsp), %xmm11
1422a3321:     	movaps	0x90(%rsp), %xmm12
1422a332a:     	movaps	0x80(%rsp), %xmm13
1422a3333:     	movaps	0x70(%rsp), %xmm14
1422a3339:     	movaps	0x60(%rsp), %xmm15
1422a333f:     	addq	$0x108, %rsp            # imm = 0x108
1422a3346:     	popq	%rbx
1422a3347:     	popq	%rdi
1422a3348:     	popq	%rsi
1422a3349:     	popq	%r12
1422a334b:     	popq	%r13
1422a334d:     	popq	%r14
1422a334f:     	popq	%r15
1422a3351:     	popq	%rbp
1422a3352:     	retq
1422a3353:     	nopw	%cs:(%rax,%rax)
1422a3360:     	movq	%rdx, 0x10(%rsp)
1422a3365:     	pushq	%rbp
1422a3366:     	pushq	%r15
1422a3368:     	pushq	%r14
1422a336a:     	pushq	%r13
1422a336c:     	pushq	%r12
1422a336e:     	pushq	%rsi
1422a336f:     	pushq	%rdi
1422a3370:     	pushq	%rbx
1422a3371:     	subq	$0x108, %rsp            # imm = 0x108
1422a3378:     	leaq	0x80(%rdx), %rbp
1422a337f:     	movaps	%xmm15, 0x60(%rsp)
1422a3385:     	movaps	%xmm14, 0x70(%rsp)
1422a338b:     	movaps	%xmm13, 0x80(%rsp)
1422a3394:     	movaps	%xmm12, 0x90(%rsp)
1422a339d:     	movaps	%xmm11, 0xa0(%rsp)
1422a33a6:     	movaps	%xmm10, 0xb0(%rsp)
1422a33af:     	movaps	%xmm9, 0xc0(%rsp)
1422a33b8:     	movaps	%xmm8, 0xd0(%rsp)
1422a33c1:     	movaps	%xmm7, 0xe0(%rsp)
1422a33c9:     	movaps	%xmm6, 0xf0(%rsp)
1422a33d1:     	leaq	0x180(%rbp), %rcx
1422a33d8:     	callq	0x14087b030 <.text+0x87a030>
1422a33dd:     	movaps	0xf0(%rsp), %xmm6
1422a33e5:     	movaps	0xe0(%rsp), %xmm7
1422a33ed:     	movaps	0xd0(%rsp), %xmm8
1422a33f6:     	movaps	0xc0(%rsp), %xmm9
1422a33ff:     	movaps	0xb0(%rsp), %xmm10
1422a3408:     	movaps	0xa0(%rsp), %xmm11
1422a3411:     	movaps	0x90(%rsp), %xmm12
1422a341a:     	movaps	0x80(%rsp), %xmm13
1422a3423:     	movaps	0x70(%rsp), %xmm14
1422a3429:     	movaps	0x60(%rsp), %xmm15
1422a342f:     	addq	$0x108, %rsp            # imm = 0x108
1422a3436:     	popq	%rbx
1422a3437:     	popq	%rdi
1422a3438:     	popq	%rsi
1422a3439:     	popq	%r12
1422a343b:     	popq	%r13
1422a343d:     	popq	%r14
1422a343f:     	popq	%r15
1422a3441:     	popq	%rbp
1422a3442:     	retq
1422a3443:     	nopw	%cs:(%rax,%rax)
1422a3450:     	movq	%rdx, 0x10(%rsp)
1422a3455:     	pushq	%rbp
1422a3456:     	pushq	%r15
1422a3458:     	pushq	%r14
1422a345a:     	pushq	%r13
1422a345c:     	pushq	%r12
1422a345e:     	pushq	%rsi
1422a345f:     	pushq	%rdi
1422a3460:     	pushq	%rbx
1422a3461:     	subq	$0x108, %rsp            # imm = 0x108
1422a3468:     	leaq	0x80(%rdx), %rbp
1422a346f:     	movaps	%xmm15, 0x60(%rsp)
1422a3475:     	movaps	%xmm14, 0x70(%rsp)
1422a347b:     	movaps	%xmm13, 0x80(%rsp)
1422a3484:     	movaps	%xmm12, 0x90(%rsp)
1422a348d:     	movaps	%xmm11, 0xa0(%rsp)
1422a3496:     	movaps	%xmm10, 0xb0(%rsp)
1422a349f:     	movaps	%xmm9, 0xc0(%rsp)
1422a34a8:     	movaps	%xmm8, 0xd0(%rsp)
1422a34b1:     	movaps	%xmm7, 0xe0(%rsp)
1422a34b9:     	movaps	%xmm6, 0xf0(%rsp)
1422a34c1:     	movq	0xd8(%rbp), %rdx
1422a34c8:     	testq	%rdx, %rdx
1422a34cb:     	je	0x1422a34df <.text+0x22a24df>
1422a34cd:     	movq	0xe0(%rbp), %rcx
1422a34d4:     	movl	$0x1, %r8d
1422a34da:     	callq	0x140613c20 <.text+0x612c20>
1422a34df:     	movaps	0xf0(%rsp), %xmm6
1422a34e7:     	movaps	0xe0(%rsp), %xmm7
1422a34ef:     	movaps	0xd0(%rsp), %xmm8
1422a34f8:     	movaps	0xc0(%rsp), %xmm9
1422a3501:     	movaps	0xb0(%rsp), %xmm10
1422a350a:     	movaps	0xa0(%rsp), %xmm11
1422a3513:     	movaps	0x90(%rsp), %xmm12
1422a351c:     	movaps	0x80(%rsp), %xmm13
1422a3525:     	movaps	0x70(%rsp), %xmm14
1422a352b:     	movaps	0x60(%rsp), %xmm15
1422a3531:     	addq	$0x108, %rsp            # imm = 0x108
1422a3538:     	popq	%rbx
1422a3539:     	popq	%rdi
1422a353a:     	popq	%rsi
1422a353b:     	popq	%r12
1422a353d:     	popq	%r13
1422a353f:     	popq	%r14
1422a3541:     	popq	%r15
1422a3543:     	popq	%rbp
1422a3544:     	retq
1422a3545:     	nopw	%cs:(%rax,%rax)
1422a3550:     	movq	%rdx, 0x10(%rsp)
1422a3555:     	pushq	%rbp
1422a3556:     	pushq	%r15
1422a3558:     	pushq	%r14
1422a355a:     	pushq	%r13
1422a355c:     	pushq	%r12
1422a355e:     	pushq	%rsi
1422a355f:     	pushq	%rdi
1422a3560:     	pushq	%rbx
1422a3561:     	subq	$0x108, %rsp            # imm = 0x108
1422a3568:     	leaq	0x80(%rdx), %rbp
1422a356f:     	movaps	%xmm15, 0x60(%rsp)
1422a3575:     	movaps	%xmm14, 0x70(%rsp)
1422a357b:     	movaps	%xmm13, 0x80(%rsp)
1422a3584:     	movaps	%xmm12, 0x90(%rsp)
1422a358d:     	movaps	%xmm11, 0xa0(%rsp)
1422a3596:     	movaps	%xmm10, 0xb0(%rsp)
1422a359f:     	movaps	%xmm9, 0xc0(%rsp)
1422a35a8:     	movaps	%xmm8, 0xd0(%rsp)
1422a35b1:     	movaps	%xmm7, 0xe0(%rsp)
1422a35b9:     	movaps	%xmm6, 0xf0(%rsp)
1422a35c1:     	movl	$0x50, %edx
1422a35c6:     	movl	$0x8, %r8d
1422a35cc:     	movq	0x1f0(%rbp), %rcx
1422a35d3:     	callq	0x140613c20 <.text+0x612c20>
1422a35d8:     	movaps	0xf0(%rsp), %xmm6
1422a35e0:     	movaps	0xe0(%rsp), %xmm7
1422a35e8:     	movaps	0xd0(%rsp), %xmm8
1422a35f1:     	movaps	0xc0(%rsp), %xmm9
1422a35fa:     	movaps	0xb0(%rsp), %xmm10
1422a3603:     	movaps	0xa0(%rsp), %xmm11
1422a360c:     	movaps	0x90(%rsp), %xmm12
1422a3615:     	movaps	0x80(%rsp), %xmm13
1422a361e:     	movaps	0x70(%rsp), %xmm14
1422a3624:     	movaps	0x60(%rsp), %xmm15
1422a362a:     	addq	$0x108, %rsp            # imm = 0x108
1422a3631:     	popq	%rbx
1422a3632:     	popq	%rdi
1422a3633:     	popq	%rsi
1422a3634:     	popq	%r12
1422a3636:     	popq	%r13
1422a3638:     	popq	%r14
1422a363a:     	popq	%r15
1422a363c:     	popq	%rbp
1422a363d:     	retq
1422a363e:     	nop
1422a3640:     	movq	%rdx, 0x10(%rsp)
1422a3645:     	pushq	%rbp
1422a3646:     	pushq	%r15
1422a3648:     	pushq	%r14
1422a364a:     	pushq	%r13
1422a364c:     	pushq	%r12
1422a364e:     	pushq	%rsi
1422a364f:     	pushq	%rdi
1422a3650:     	pushq	%rbx
1422a3651:     	subq	$0x108, %rsp            # imm = 0x108
1422a3658:     	leaq	0x80(%rdx), %rbp
1422a365f:     	movaps	%xmm15, 0x60(%rsp)
1422a3665:     	movaps	%xmm14, 0x70(%rsp)
1422a366b:     	movaps	%xmm13, 0x80(%rsp)
1422a3674:     	movaps	%xmm12, 0x90(%rsp)
1422a367d:     	movaps	%xmm11, 0xa0(%rsp)
1422a3686:     	movaps	%xmm10, 0xb0(%rsp)
1422a368f:     	movaps	%xmm9, 0xc0(%rsp)
1422a3698:     	movaps	%xmm8, 0xd0(%rsp)
1422a36a1:     	movaps	%xmm7, 0xe0(%rsp)
1422a36a9:     	movaps	%xmm6, 0xf0(%rsp)
1422a36b1:     	movl	$0x1b, %edx
1422a36b6:     	movl	$0x1, %r8d
1422a36bc:     	movq	0x1e0(%rbp), %rcx
1422a36c3:     	callq	0x140613c20 <.text+0x612c20>
1422a36c8:     	movaps	0xf0(%rsp), %xmm6
1422a36d0:     	movaps	0xe0(%rsp), %xmm7
1422a36d8:     	movaps	0xd0(%rsp), %xmm8
1422a36e1:     	movaps	0xc0(%rsp), %xmm9
1422a36ea:     	movaps	0xb0(%rsp), %xmm10
1422a36f3:     	movaps	0xa0(%rsp), %xmm11
1422a36fc:     	movaps	0x90(%rsp), %xmm12
1422a3705:     	movaps	0x80(%rsp), %xmm13
1422a370e:     	movaps	0x70(%rsp), %xmm14
1422a3714:     	movaps	0x60(%rsp), %xmm15
1422a371a:     	addq	$0x108, %rsp            # imm = 0x108
1422a3721:     	popq	%rbx
1422a3722:     	popq	%rdi
1422a3723:     	popq	%rsi
1422a3724:     	popq	%r12
1422a3726:     	popq	%r13
1422a3728:     	popq	%r14
1422a372a:     	popq	%r15
1422a372c:     	popq	%rbp
1422a372d:     	retq
1422a372e:     	nop
1422a3730:     	movq	%rdx, 0x10(%rsp)
1422a3735:     	pushq	%rbp
1422a3736:     	pushq	%r15
1422a3738:     	pushq	%r14
1422a373a:     	pushq	%r13
1422a373c:     	pushq	%r12
1422a373e:     	pushq	%rsi
1422a373f:     	pushq	%rdi
1422a3740:     	pushq	%rbx
1422a3741:     	subq	$0x108, %rsp            # imm = 0x108
1422a3748:     	leaq	0x80(%rdx), %rbp
1422a374f:     	movaps	%xmm15, 0x60(%rsp)
1422a3755:     	movaps	%xmm14, 0x70(%rsp)
1422a375b:     	movaps	%xmm13, 0x80(%rsp)
1422a3764:     	movaps	%xmm12, 0x90(%rsp)
1422a376d:     	movaps	%xmm11, 0xa0(%rsp)
1422a3776:     	movaps	%xmm10, 0xb0(%rsp)
1422a377f:     	movaps	%xmm9, 0xc0(%rsp)
1422a3788:     	movaps	%xmm8, 0xd0(%rsp)
1422a3791:     	movaps	%xmm7, 0xe0(%rsp)
1422a3799:     	movaps	%xmm6, 0xf0(%rsp)
1422a37a1:     	movl	$0x14, %edx
1422a37a6:     	movl	$0x1, %r8d
1422a37ac:     	movq	0x100(%rbp), %rcx
1422a37b3:     	callq	0x140613c20 <.text+0x612c20>
1422a37b8:     	movaps	0xf0(%rsp), %xmm6
1422a37c0:     	movaps	0xe0(%rsp), %xmm7
1422a37c8:     	movaps	0xd0(%rsp), %xmm8
1422a37d1:     	movaps	0xc0(%rsp), %xmm9
1422a37da:     	movaps	0xb0(%rsp), %xmm10
1422a37e3:     	movaps	0xa0(%rsp), %xmm11
1422a37ec:     	movaps	0x90(%rsp), %xmm12
1422a37f5:     	movaps	0x80(%rsp), %xmm13
1422a37fe:     	movaps	0x70(%rsp), %xmm14
1422a3804:     	movaps	0x60(%rsp), %xmm15
1422a380a:     	addq	$0x108, %rsp            # imm = 0x108
1422a3811:     	popq	%rbx
1422a3812:     	popq	%rdi
1422a3813:     	popq	%rsi
1422a3814:     	popq	%r12
1422a3816:     	popq	%r13
1422a3818:     	popq	%r14
1422a381a:     	popq	%r15
1422a381c:     	popq	%rbp
1422a381d:     	retq
1422a381e:     	nop
1422a3820:     	movq	%rdx, 0x10(%rsp)
1422a3825:     	pushq	%rbp
1422a3826:     	pushq	%r15
1422a3828:     	pushq	%r14
1422a382a:     	pushq	%r13
1422a382c:     	pushq	%r12
1422a382e:     	pushq	%rsi
1422a382f:     	pushq	%rdi
1422a3830:     	pushq	%rbx
1422a3831:     	subq	$0x108, %rsp            # imm = 0x108
1422a3838:     	leaq	0x80(%rdx), %rbp
1422a383f:     	movaps	%xmm15, 0x60(%rsp)
1422a3845:     	movaps	%xmm14, 0x70(%rsp)
1422a384b:     	movaps	%xmm13, 0x80(%rsp)
1422a3854:     	movaps	%xmm12, 0x90(%rsp)
1422a385d:     	movaps	%xmm11, 0xa0(%rsp)
1422a3866:     	movaps	%xmm10, 0xb0(%rsp)
1422a386f:     	movaps	%xmm9, 0xc0(%rsp)
1422a3878:     	movaps	%xmm8, 0xd0(%rsp)
1422a3881:     	movaps	%xmm7, 0xe0(%rsp)
1422a3889:     	movaps	%xmm6, 0xf0(%rsp)
1422a3891:     	movl	$0x19, %edx
1422a3896:     	movl	$0x1, %r8d
1422a389c:     	movq	0x148(%rbp), %rcx
1422a38a3:     	callq	0x140613c20 <.text+0x612c20>
1422a38a8:     	movaps	0xf0(%rsp), %xmm6
1422a38b0:     	movaps	0xe0(%rsp), %xmm7
1422a38b8:     	movaps	0xd0(%rsp), %xmm8
1422a38c1:     	movaps	0xc0(%rsp), %xmm9
1422a38ca:     	movaps	0xb0(%rsp), %xmm10
1422a38d3:     	movaps	0xa0(%rsp), %xmm11
1422a38dc:     	movaps	0x90(%rsp), %xmm12
1422a38e5:     	movaps	0x80(%rsp), %xmm13
1422a38ee:     	movaps	0x70(%rsp), %xmm14
1422a38f4:     	movaps	0x60(%rsp), %xmm15
1422a38fa:     	addq	$0x108, %rsp            # imm = 0x108
1422a3901:     	popq	%rbx
1422a3902:     	popq	%rdi
1422a3903:     	popq	%rsi
1422a3904:     	popq	%r12
1422a3906:     	popq	%r13
1422a3908:     	popq	%r14
1422a390a:     	popq	%r15
1422a390c:     	popq	%rbp
1422a390d:     	retq
1422a390e:     	nop
1422a3910:     	movq	%rdx, 0x10(%rsp)
1422a3915:     	pushq	%rbp
1422a3916:     	pushq	%r15
1422a3918:     	pushq	%r14
1422a391a:     	pushq	%r13
1422a391c:     	pushq	%r12
1422a391e:     	pushq	%rsi
1422a391f:     	pushq	%rdi
1422a3920:     	pushq	%rbx
1422a3921:     	subq	$0x108, %rsp            # imm = 0x108
1422a3928:     	leaq	0x80(%rdx), %rbp
1422a392f:     	movaps	%xmm15, 0x60(%rsp)
1422a3935:     	movaps	%xmm14, 0x70(%rsp)
1422a393b:     	movaps	%xmm13, 0x80(%rsp)
1422a3944:     	movaps	%xmm12, 0x90(%rsp)
1422a394d:     	movaps	%xmm11, 0xa0(%rsp)
1422a3956:     	movaps	%xmm10, 0xb0(%rsp)
1422a395f:     	movaps	%xmm9, 0xc0(%rsp)
1422a3968:     	movaps	%xmm8, 0xd0(%rsp)
1422a3971:     	movaps	%xmm7, 0xe0(%rsp)
1422a3979:     	movaps	%xmm6, 0xf0(%rsp)
1422a3981:     	movl	$0xf, %edx
1422a3986:     	movl	$0x1, %r8d
1422a398c:     	movq	0x150(%rbp), %rcx
1422a3993:     	callq	0x140613c20 <.text+0x612c20>
1422a3998:     	movaps	0xf0(%rsp), %xmm6
1422a39a0:     	movaps	0xe0(%rsp), %xmm7
1422a39a8:     	movaps	0xd0(%rsp), %xmm8
1422a39b1:     	movaps	0xc0(%rsp), %xmm9
1422a39ba:     	movaps	0xb0(%rsp), %xmm10
1422a39c3:     	movaps	0xa0(%rsp), %xmm11
1422a39cc:     	movaps	0x90(%rsp), %xmm12
1422a39d5:     	movaps	0x80(%rsp), %xmm13
1422a39de:     	movaps	0x70(%rsp), %xmm14
1422a39e4:     	movaps	0x60(%rsp), %xmm15
1422a39ea:     	addq	$0x108, %rsp            # imm = 0x108
1422a39f1:     	popq	%rbx
1422a39f2:     	popq	%rdi
1422a39f3:     	popq	%rsi
1422a39f4:     	popq	%r12
1422a39f6:     	popq	%r13
1422a39f8:     	popq	%r14
1422a39fa:     	popq	%r15
1422a39fc:     	popq	%rbp
1422a39fd:     	retq
1422a39fe:     	nop
1422a3a00:     	movq	%rdx, 0x10(%rsp)
1422a3a05:     	pushq	%rbp
1422a3a06:     	pushq	%r15
1422a3a08:     	pushq	%r14
1422a3a0a:     	pushq	%r13
1422a3a0c:     	pushq	%r12
1422a3a0e:     	pushq	%rsi
1422a3a0f:     	pushq	%rdi
1422a3a10:     	pushq	%rbx
1422a3a11:     	subq	$0x108, %rsp            # imm = 0x108
1422a3a18:     	leaq	0x80(%rdx), %rbp
1422a3a1f:     	movaps	%xmm15, 0x60(%rsp)
1422a3a25:     	movaps	%xmm14, 0x70(%rsp)
1422a3a2b:     	movaps	%xmm13, 0x80(%rsp)
1422a3a34:     	movaps	%xmm12, 0x90(%rsp)
1422a3a3d:     	movaps	%xmm11, 0xa0(%rsp)
1422a3a46:     	movaps	%xmm10, 0xb0(%rsp)
1422a3a4f:     	movaps	%xmm9, 0xc0(%rsp)
1422a3a58:     	movaps	%xmm8, 0xd0(%rsp)
1422a3a61:     	movaps	%xmm7, 0xe0(%rsp)
1422a3a69:     	movaps	%xmm6, 0xf0(%rsp)
1422a3a71:     	movl	$0x13, %edx
1422a3a76:     	movl	$0x1, %r8d
1422a3a7c:     	movq	0x1c0(%rbp), %rcx
1422a3a83:     	callq	0x140613c20 <.text+0x612c20>
1422a3a88:     	movaps	0xf0(%rsp), %xmm6
1422a3a90:     	movaps	0xe0(%rsp), %xmm7
1422a3a98:     	movaps	0xd0(%rsp), %xmm8
1422a3aa1:     	movaps	0xc0(%rsp), %xmm9
1422a3aaa:     	movaps	0xb0(%rsp), %xmm10
1422a3ab3:     	movaps	0xa0(%rsp), %xmm11
1422a3abc:     	movaps	0x90(%rsp), %xmm12
1422a3ac5:     	movaps	0x80(%rsp), %xmm13
1422a3ace:     	movaps	0x70(%rsp), %xmm14
1422a3ad4:     	movaps	0x60(%rsp), %xmm15
1422a3ada:     	addq	$0x108, %rsp            # imm = 0x108
1422a3ae1:     	popq	%rbx
1422a3ae2:     	popq	%rdi
1422a3ae3:     	popq	%rsi
1422a3ae4:     	popq	%r12
1422a3ae6:     	popq	%r13
1422a3ae8:     	popq	%r14
1422a3aea:     	popq	%r15
1422a3aec:     	popq	%rbp
1422a3aed:     	retq
1422a3aee:     	nop
1422a3af0:     	movq	%rdx, 0x10(%rsp)
1422a3af5:     	pushq	%rbp
1422a3af6:     	pushq	%r15
1422a3af8:     	pushq	%r14
1422a3afa:     	pushq	%r13
1422a3afc:     	pushq	%r12
1422a3afe:     	pushq	%rsi
1422a3aff:     	pushq	%rdi
1422a3b00:     	pushq	%rbx
1422a3b01:     	subq	$0x108, %rsp            # imm = 0x108
1422a3b08:     	leaq	0x80(%rdx), %rbp
1422a3b0f:     	movaps	%xmm15, 0x60(%rsp)
1422a3b15:     	movaps	%xmm14, 0x70(%rsp)
1422a3b1b:     	movaps	%xmm13, 0x80(%rsp)
1422a3b24:     	movaps	%xmm12, 0x90(%rsp)
1422a3b2d:     	movaps	%xmm11, 0xa0(%rsp)
1422a3b36:     	movaps	%xmm10, 0xb0(%rsp)
1422a3b3f:     	movaps	%xmm9, 0xc0(%rsp)
1422a3b48:     	movaps	%xmm8, 0xd0(%rsp)
1422a3b51:     	movaps	%xmm7, 0xe0(%rsp)
1422a3b59:     	movaps	%xmm6, 0xf0(%rsp)
1422a3b61:     	movl	$0x14, %edx
1422a3b66:     	movl	$0x1, %r8d
1422a3b6c:     	movq	0x1d8(%rbp), %rcx
1422a3b73:     	callq	0x140613c20 <.text+0x612c20>
1422a3b78:     	movaps	0xf0(%rsp), %xmm6
1422a3b80:     	movaps	0xe0(%rsp), %xmm7
1422a3b88:     	movaps	0xd0(%rsp), %xmm8
1422a3b91:     	movaps	0xc0(%rsp), %xmm9
1422a3b9a:     	movaps	0xb0(%rsp), %xmm10
1422a3ba3:     	movaps	0xa0(%rsp), %xmm11
1422a3bac:     	movaps	0x90(%rsp), %xmm12
1422a3bb5:     	movaps	0x80(%rsp), %xmm13
1422a3bbe:     	movaps	0x70(%rsp), %xmm14
1422a3bc4:     	movaps	0x60(%rsp), %xmm15
1422a3bca:     	addq	$0x108, %rsp            # imm = 0x108
1422a3bd1:     	popq	%rbx
1422a3bd2:     	popq	%rdi
1422a3bd3:     	popq	%rsi
1422a3bd4:     	popq	%r12
1422a3bd6:     	popq	%r13
1422a3bd8:     	popq	%r14
1422a3bda:     	popq	%r15
1422a3bdc:     	popq	%rbp
1422a3bdd:     	retq
1422a3bde:     	nop
1422a3be0:     	movq	%rdx, 0x10(%rsp)
1422a3be5:     	pushq	%rbp
1422a3be6:     	pushq	%r15
1422a3be8:     	pushq	%r14
1422a3bea:     	pushq	%r13
1422a3bec:     	pushq	%r12
1422a3bee:     	pushq	%rsi
1422a3bef:     	pushq	%rdi
1422a3bf0:     	pushq	%rbx
1422a3bf1:     	subq	$0x108, %rsp            # imm = 0x108
1422a3bf8:     	leaq	0x80(%rdx), %rbp
1422a3bff:     	movaps	%xmm15, 0x60(%rsp)
1422a3c05:     	movaps	%xmm14, 0x70(%rsp)
1422a3c0b:     	movaps	%xmm13, 0x80(%rsp)
1422a3c14:     	movaps	%xmm12, 0x90(%rsp)
1422a3c1d:     	movaps	%xmm11, 0xa0(%rsp)
1422a3c26:     	movaps	%xmm10, 0xb0(%rsp)
1422a3c2f:     	movaps	%xmm9, 0xc0(%rsp)
1422a3c38:     	movaps	%xmm8, 0xd0(%rsp)
1422a3c41:     	movaps	%xmm7, 0xe0(%rsp)
1422a3c49:     	movaps	%xmm6, 0xf0(%rsp)
1422a3c51:     	movl	$0x1c, %edx
1422a3c56:     	movl	$0x1, %r8d
1422a3c5c:     	movq	0x1f0(%rbp), %rcx
1422a3c63:     	callq	0x140613c20 <.text+0x612c20>
1422a3c68:     	movaps	0xf0(%rsp), %xmm6
1422a3c70:     	movaps	0xe0(%rsp), %xmm7
1422a3c78:     	movaps	0xd0(%rsp), %xmm8
1422a3c81:     	movaps	0xc0(%rsp), %xmm9
1422a3c8a:     	movaps	0xb0(%rsp), %xmm10
1422a3c93:     	movaps	0xa0(%rsp), %xmm11
1422a3c9c:     	movaps	0x90(%rsp), %xmm12
1422a3ca5:     	movaps	0x80(%rsp), %xmm13
1422a3cae:     	movaps	0x70(%rsp), %xmm14
1422a3cb4:     	movaps	0x60(%rsp), %xmm15
1422a3cba:     	addq	$0x108, %rsp            # imm = 0x108
1422a3cc1:     	popq	%rbx
1422a3cc2:     	popq	%rdi
1422a3cc3:     	popq	%rsi
1422a3cc4:     	popq	%r12
1422a3cc6:     	popq	%r13
1422a3cc8:     	popq	%r14
1422a3cca:     	popq	%r15
1422a3ccc:     	popq	%rbp
1422a3ccd:     	retq
1422a3cce:     	nop
1422a3cd0:     	movq	%rdx, 0x10(%rsp)
1422a3cd5:     	pushq	%rbp
1422a3cd6:     	pushq	%r15
1422a3cd8:     	pushq	%r14
1422a3cda:     	pushq	%r13
1422a3cdc:     	pushq	%r12
1422a3cde:     	pushq	%rsi
1422a3cdf:     	pushq	%rdi
1422a3ce0:     	pushq	%rbx
1422a3ce1:     	subq	$0x108, %rsp            # imm = 0x108
1422a3ce8:     	leaq	0x80(%rdx), %rbp
1422a3cef:     	movaps	%xmm15, 0x60(%rsp)
1422a3cf5:     	movaps	%xmm14, 0x70(%rsp)
1422a3cfb:     	movaps	%xmm13, 0x80(%rsp)
1422a3d04:     	movaps	%xmm12, 0x90(%rsp)
1422a3d0d:     	movaps	%xmm11, 0xa0(%rsp)
1422a3d16:     	movaps	%xmm10, 0xb0(%rsp)
1422a3d1f:     	movaps	%xmm9, 0xc0(%rsp)
1422a3d28:     	movaps	%xmm8, 0xd0(%rsp)
1422a3d31:     	movaps	%xmm7, 0xe0(%rsp)
1422a3d39:     	movaps	%xmm6, 0xf0(%rsp)
1422a3d41:     	movl	$0x17, %edx
1422a3d46:     	movl	$0x1, %r8d
1422a3d4c:     	movq	0x1f8(%rbp), %rcx
1422a3d53:     	callq	0x140613c20 <.text+0x612c20>
1422a3d58:     	movaps	0xf0(%rsp), %xmm6
1422a3d60:     	movaps	0xe0(%rsp), %xmm7
1422a3d68:     	movaps	0xd0(%rsp), %xmm8
1422a3d71:     	movaps	0xc0(%rsp), %xmm9
1422a3d7a:     	movaps	0xb0(%rsp), %xmm10
1422a3d83:     	movaps	0xa0(%rsp), %xmm11
1422a3d8c:     	movaps	0x90(%rsp), %xmm12
1422a3d95:     	movaps	0x80(%rsp), %xmm13
1422a3d9e:     	movaps	0x70(%rsp), %xmm14
1422a3da4:     	movaps	0x60(%rsp), %xmm15
1422a3daa:     	addq	$0x108, %rsp            # imm = 0x108
1422a3db1:     	popq	%rbx
1422a3db2:     	popq	%rdi
1422a3db3:     	popq	%rsi
1422a3db4:     	popq	%r12
1422a3db6:     	popq	%r13
1422a3db8:     	popq	%r14
1422a3dba:     	popq	%r15
1422a3dbc:     	popq	%rbp
1422a3dbd:     	retq
1422a3dbe:     	nop
1422a3dc0:     	movq	%rdx, 0x10(%rsp)
1422a3dc5:     	pushq	%rbp
1422a3dc6:     	pushq	%r15
1422a3dc8:     	pushq	%r14
1422a3dca:     	pushq	%r13
1422a3dcc:     	pushq	%r12
1422a3dce:     	pushq	%rsi
1422a3dcf:     	pushq	%rdi
1422a3dd0:     	pushq	%rbx
1422a3dd1:     	subq	$0x108, %rsp            # imm = 0x108
1422a3dd8:     	leaq	0x80(%rdx), %rbp
1422a3ddf:     	movaps	%xmm15, 0x60(%rsp)
1422a3de5:     	movaps	%xmm14, 0x70(%rsp)
1422a3deb:     	movaps	%xmm13, 0x80(%rsp)
1422a3df4:     	movaps	%xmm12, 0x90(%rsp)
1422a3dfd:     	movaps	%xmm11, 0xa0(%rsp)
1422a3e06:     	movaps	%xmm10, 0xb0(%rsp)
1422a3e0f:     	movaps	%xmm9, 0xc0(%rsp)
1422a3e18:     	movaps	%xmm8, 0xd0(%rsp)
1422a3e21:     	movaps	%xmm7, 0xe0(%rsp)
1422a3e29:     	movaps	%xmm6, 0xf0(%rsp)
1422a3e31:     	movl	$0x18, %edx
1422a3e36:     	movl	$0x1, %r8d
1422a3e3c:     	movq	0x200(%rbp), %rcx
1422a3e43:     	callq	0x140613c20 <.text+0x612c20>
1422a3e48:     	movaps	0xf0(%rsp), %xmm6
1422a3e50:     	movaps	0xe0(%rsp), %xmm7
1422a3e58:     	movaps	0xd0(%rsp), %xmm8
1422a3e61:     	movaps	0xc0(%rsp), %xmm9
1422a3e6a:     	movaps	0xb0(%rsp), %xmm10
1422a3e73:     	movaps	0xa0(%rsp), %xmm11
1422a3e7c:     	movaps	0x90(%rsp), %xmm12
1422a3e85:     	movaps	0x80(%rsp), %xmm13
1422a3e8e:     	movaps	0x70(%rsp), %xmm14
1422a3e94:     	movaps	0x60(%rsp), %xmm15
1422a3e9a:     	addq	$0x108, %rsp            # imm = 0x108
1422a3ea1:     	popq	%rbx
1422a3ea2:     	popq	%rdi
1422a3ea3:     	popq	%rsi
1422a3ea4:     	popq	%r12
1422a3ea6:     	popq	%r13
1422a3ea8:     	popq	%r14
1422a3eaa:     	popq	%r15
1422a3eac:     	popq	%rbp
1422a3ead:     	retq
1422a3eae:     	nop
1422a3eb0:     	movq	%rdx, 0x10(%rsp)
1422a3eb5:     	pushq	%rbp
1422a3eb6:     	pushq	%r15
1422a3eb8:     	pushq	%r14
1422a3eba:     	pushq	%r13
1422a3ebc:     	pushq	%r12
1422a3ebe:     	pushq	%rsi
1422a3ebf:     	pushq	%rdi
1422a3ec0:     	pushq	%rbx
1422a3ec1:     	subq	$0x108, %rsp            # imm = 0x108
1422a3ec8:     	leaq	0x80(%rdx), %rbp
1422a3ecf:     	movaps	%xmm15, 0x60(%rsp)
1422a3ed5:     	movaps	%xmm14, 0x70(%rsp)
1422a3edb:     	movaps	%xmm13, 0x80(%rsp)
1422a3ee4:     	movaps	%xmm12, 0x90(%rsp)
1422a3eed:     	movaps	%xmm11, 0xa0(%rsp)
1422a3ef6:     	movaps	%xmm10, 0xb0(%rsp)
1422a3eff:     	movaps	%xmm9, 0xc0(%rsp)
1422a3f08:     	movaps	%xmm8, 0xd0(%rsp)
1422a3f11:     	movaps	%xmm7, 0xe0(%rsp)
1422a3f19:     	movaps	%xmm6, 0xf0(%rsp)
1422a3f21:     	movl	$0x190, %edx            # imm = 0x190
1422a3f26:     	movl	$0x8, %r8d
1422a3f2c:     	movq	0x1e8(%rbp), %rcx
1422a3f33:     	callq	0x140613c20 <.text+0x612c20>
1422a3f38:     	movaps	0xf0(%rsp), %xmm6
1422a3f40:     	movaps	0xe0(%rsp), %xmm7
1422a3f48:     	movaps	0xd0(%rsp), %xmm8
1422a3f51:     	movaps	0xc0(%rsp), %xmm9
1422a3f5a:     	movaps	0xb0(%rsp), %xmm10
1422a3f63:     	movaps	0xa0(%rsp), %xmm11
1422a3f6c:     	movaps	0x90(%rsp), %xmm12
1422a3f75:     	movaps	0x80(%rsp), %xmm13
1422a3f7e:     	movaps	0x70(%rsp), %xmm14
1422a3f84:     	movaps	0x60(%rsp), %xmm15
1422a3f8a:     	addq	$0x108, %rsp            # imm = 0x108
1422a3f91:     	popq	%rbx
1422a3f92:     	popq	%rdi
1422a3f93:     	popq	%rsi
1422a3f94:     	popq	%r12
1422a3f96:     	popq	%r13
1422a3f98:     	popq	%r14
1422a3f9a:     	popq	%r15
1422a3f9c:     	popq	%rbp
1422a3f9d:     	retq
1422a3f9e:     	nop
1422a3fa0:     	movq	%rdx, 0x10(%rsp)
1422a3fa5:     	pushq	%rbp
1422a3fa6:     	pushq	%r15
1422a3fa8:     	pushq	%r14
1422a3faa:     	pushq	%r13
1422a3fac:     	pushq	%r12
1422a3fae:     	pushq	%rsi
1422a3faf:     	pushq	%rdi
1422a3fb0:     	pushq	%rbx
1422a3fb1:     	subq	$0x108, %rsp            # imm = 0x108
1422a3fb8:     	leaq	0x80(%rdx), %rbp
1422a3fbf:     	movaps	%xmm15, 0x60(%rsp)
1422a3fc5:     	movaps	%xmm14, 0x70(%rsp)
1422a3fcb:     	movaps	%xmm13, 0x80(%rsp)
1422a3fd4:     	movaps	%xmm12, 0x90(%rsp)
1422a3fdd:     	movaps	%xmm11, 0xa0(%rsp)
1422a3fe6:     	movaps	%xmm10, 0xb0(%rsp)
1422a3fef:     	movaps	%xmm9, 0xc0(%rsp)
1422a3ff8:     	movaps	%xmm8, 0xd0(%rsp)
1422a4001:     	movaps	%xmm7, 0xe0(%rsp)
1422a4009:     	movaps	%xmm6, 0xf0(%rsp)
1422a4011:     	movl	$0x1b, %edx
1422a4016:     	movl	$0x1, %r8d
1422a401c:     	movq	0x1f0(%rbp), %rcx
1422a4023:     	callq	0x140613c20 <.text+0x612c20>
1422a4028:     	movaps	0xf0(%rsp), %xmm6
1422a4030:     	movaps	0xe0(%rsp), %xmm7
1422a4038:     	movaps	0xd0(%rsp), %xmm8
1422a4041:     	movaps	0xc0(%rsp), %xmm9
1422a404a:     	movaps	0xb0(%rsp), %xmm10
1422a4053:     	movaps	0xa0(%rsp), %xmm11
1422a405c:     	movaps	0x90(%rsp), %xmm12
1422a4065:     	movaps	0x80(%rsp), %xmm13
1422a406e:     	movaps	0x70(%rsp), %xmm14
1422a4074:     	movaps	0x60(%rsp), %xmm15
1422a407a:     	addq	$0x108, %rsp            # imm = 0x108
1422a4081:     	popq	%rbx
1422a4082:     	popq	%rdi
1422a4083:     	popq	%rsi
1422a4084:     	popq	%r12
1422a4086:     	popq	%r13
1422a4088:     	popq	%r14
1422a408a:     	popq	%r15
1422a408c:     	popq	%rbp
1422a408d:     	retq
1422a408e:     	nop
1422a4090:     	movq	%rdx, 0x10(%rsp)
1422a4095:     	pushq	%rbp
1422a4096:     	pushq	%r15
1422a4098:     	pushq	%r14
1422a409a:     	pushq	%r13
1422a409c:     	pushq	%r12
1422a409e:     	pushq	%rsi
1422a409f:     	pushq	%rdi
1422a40a0:     	pushq	%rbx
1422a40a1:     	subq	$0x108, %rsp            # imm = 0x108
1422a40a8:     	leaq	0x80(%rdx), %rbp
1422a40af:     	movaps	%xmm15, 0x60(%rsp)
1422a40b5:     	movaps	%xmm14, 0x70(%rsp)
1422a40bb:     	movaps	%xmm13, 0x80(%rsp)
1422a40c4:     	movaps	%xmm12, 0x90(%rsp)
1422a40cd:     	movaps	%xmm11, 0xa0(%rsp)
1422a40d6:     	movaps	%xmm10, 0xb0(%rsp)
1422a40df:     	movaps	%xmm9, 0xc0(%rsp)
1422a40e8:     	movaps	%xmm8, 0xd0(%rsp)
1422a40f1:     	movaps	%xmm7, 0xe0(%rsp)
1422a40f9:     	movaps	%xmm6, 0xf0(%rsp)
1422a4101:     	movl	$0x5, %edx
1422a4106:     	movl	$0x1, %r8d
1422a410c:     	movq	0x1f0(%rbp), %rcx
1422a4113:     	callq	0x140613c20 <.text+0x612c20>
1422a4118:     	movaps	0xf0(%rsp), %xmm6
1422a4120:     	movaps	0xe0(%rsp), %xmm7
1422a4128:     	movaps	0xd0(%rsp), %xmm8
1422a4131:     	movaps	0xc0(%rsp), %xmm9
1422a413a:     	movaps	0xb0(%rsp), %xmm10
1422a4143:     	movaps	0xa0(%rsp), %xmm11
1422a414c:     	movaps	0x90(%rsp), %xmm12
1422a4155:     	movaps	0x80(%rsp), %xmm13
1422a415e:     	movaps	0x70(%rsp), %xmm14
1422a4164:     	movaps	0x60(%rsp), %xmm15
1422a416a:     	addq	$0x108, %rsp            # imm = 0x108
1422a4171:     	popq	%rbx
1422a4172:     	popq	%rdi
1422a4173:     	popq	%rsi
1422a4174:     	popq	%r12
1422a4176:     	popq	%r13
1422a4178:     	popq	%r14
1422a417a:     	popq	%r15
1422a417c:     	popq	%rbp
1422a417d:     	retq
1422a417e:     	nop
1422a4180:     	movq	%rdx, 0x10(%rsp)
1422a4185:     	pushq	%rbp
1422a4186:     	pushq	%r15
1422a4188:     	pushq	%r14
1422a418a:     	pushq	%r13
1422a418c:     	pushq	%r12
1422a418e:     	pushq	%rsi
1422a418f:     	pushq	%rdi
1422a4190:     	pushq	%rbx
1422a4191:     	subq	$0x108, %rsp            # imm = 0x108
1422a4198:     	leaq	0x80(%rdx), %rbp
1422a419f:     	movaps	%xmm15, 0x60(%rsp)
1422a41a5:     	movaps	%xmm14, 0x70(%rsp)
1422a41ab:     	movaps	%xmm13, 0x80(%rsp)
1422a41b4:     	movaps	%xmm12, 0x90(%rsp)
1422a41bd:     	movaps	%xmm11, 0xa0(%rsp)
1422a41c6:     	movaps	%xmm10, 0xb0(%rsp)
1422a41cf:     	movaps	%xmm9, 0xc0(%rsp)
1422a41d8:     	movaps	%xmm8, 0xd0(%rsp)
1422a41e1:     	movaps	%xmm7, 0xe0(%rsp)
1422a41e9:     	movaps	%xmm6, 0xf0(%rsp)
1422a41f1:     	movl	$0x10, %edx
1422a41f6:     	movl	$0x1, %r8d
1422a41fc:     	movq	0x200(%rbp), %rcx
1422a4203:     	callq	0x140613c20 <.text+0x612c20>
1422a4208:     	movaps	0xf0(%rsp), %xmm6
1422a4210:     	movaps	0xe0(%rsp), %xmm7
1422a4218:     	movaps	0xd0(%rsp), %xmm8
1422a4221:     	movaps	0xc0(%rsp), %xmm9
1422a422a:     	movaps	0xb0(%rsp), %xmm10
1422a4233:     	movaps	0xa0(%rsp), %xmm11
1422a423c:     	movaps	0x90(%rsp), %xmm12
1422a4245:     	movaps	0x80(%rsp), %xmm13
1422a424e:     	movaps	0x70(%rsp), %xmm14
1422a4254:     	movaps	0x60(%rsp), %xmm15
1422a425a:     	addq	$0x108, %rsp            # imm = 0x108
1422a4261:     	popq	%rbx
1422a4262:     	popq	%rdi
1422a4263:     	popq	%rsi
1422a4264:     	popq	%r12
1422a4266:     	popq	%r13
1422a4268:     	popq	%r14
1422a426a:     	popq	%r15
1422a426c:     	popq	%rbp
1422a426d:     	retq
1422a426e:     	nop
1422a4270:     	movq	%rdx, 0x10(%rsp)
1422a4275:     	pushq	%rbp
1422a4276:     	pushq	%r15
1422a4278:     	pushq	%r14
1422a427a:     	pushq	%r13
1422a427c:     	pushq	%r12
1422a427e:     	pushq	%rsi
1422a427f:     	pushq	%rdi
1422a4280:     	pushq	%rbx
1422a4281:     	subq	$0x108, %rsp            # imm = 0x108
1422a4288:     	leaq	0x80(%rdx), %rbp
1422a428f:     	movaps	%xmm15, 0x60(%rsp)
1422a4295:     	movaps	%xmm14, 0x70(%rsp)
1422a429b:     	movaps	%xmm13, 0x80(%rsp)
1422a42a4:     	movaps	%xmm12, 0x90(%rsp)
1422a42ad:     	movaps	%xmm11, 0xa0(%rsp)
1422a42b6:     	movaps	%xmm10, 0xb0(%rsp)
1422a42bf:     	movaps	%xmm9, 0xc0(%rsp)
1422a42c8:     	movaps	%xmm8, 0xd0(%rsp)
1422a42d1:     	movaps	%xmm7, 0xe0(%rsp)
1422a42d9:     	movaps	%xmm6, 0xf0(%rsp)
1422a42e1:     	movl	$0x1, %r8d
1422a42e7:     	movq	0x1f0(%rbp), %rcx
1422a42ee:     	movq	0x1e8(%rbp), %rdx
1422a42f5:     	callq	0x140613c20 <.text+0x612c20>
1422a42fa:     	movaps	0xf0(%rsp), %xmm6
1422a4302:     	movaps	0xe0(%rsp), %xmm7
1422a430a:     	movaps	0xd0(%rsp), %xmm8
1422a4313:     	movaps	0xc0(%rsp), %xmm9
1422a431c:     	movaps	0xb0(%rsp), %xmm10
1422a4325:     	movaps	0xa0(%rsp), %xmm11
1422a432e:     	movaps	0x90(%rsp), %xmm12
1422a4337:     	movaps	0x80(%rsp), %xmm13
1422a4340:     	movaps	0x70(%rsp), %xmm14
1422a4346:     	movaps	0x60(%rsp), %xmm15
1422a434c:     	addq	$0x108, %rsp            # imm = 0x108
1422a4353:     	popq	%rbx
1422a4354:     	popq	%rdi
1422a4355:     	popq	%rsi
1422a4356:     	popq	%r12
1422a4358:     	popq	%r13
1422a435a:     	popq	%r14
1422a435c:     	popq	%r15
1422a435e:     	popq	%rbp
1422a435f:     	retq
1422a4360:     	movq	%rdx, 0x10(%rsp)
1422a4365:     	pushq	%rbp
1422a4366:     	pushq	%r15
1422a4368:     	pushq	%r14
1422a436a:     	pushq	%r13
1422a436c:     	pushq	%r12
1422a436e:     	pushq	%rsi
1422a436f:     	pushq	%rdi
1422a4370:     	pushq	%rbx
1422a4371:     	subq	$0x108, %rsp            # imm = 0x108
1422a4378:     	leaq	0x80(%rdx), %rbp
1422a437f:     	movaps	%xmm15, 0x60(%rsp)
1422a4385:     	movaps	%xmm14, 0x70(%rsp)
1422a438b:     	movaps	%xmm13, 0x80(%rsp)
1422a4394:     	movaps	%xmm12, 0x90(%rsp)
1422a439d:     	movaps	%xmm11, 0xa0(%rsp)
1422a43a6:     	movaps	%xmm10, 0xb0(%rsp)
1422a43af:     	movaps	%xmm9, 0xc0(%rsp)
1422a43b8:     	movaps	%xmm8, 0xd0(%rsp)
1422a43c1:     	movaps	%xmm7, 0xe0(%rsp)
1422a43c9:     	movaps	%xmm6, 0xf0(%rsp)
1422a43d1:     	movl	$0x5, %edx
1422a43d6:     	movl	$0x1, %r8d
1422a43dc:     	movq	0x1f0(%rbp), %rcx
1422a43e3:     	callq	0x140613c20 <.text+0x612c20>
1422a43e8:     	movaps	0xf0(%rsp), %xmm6
1422a43f0:     	movaps	0xe0(%rsp), %xmm7
1422a43f8:     	movaps	0xd0(%rsp), %xmm8
1422a4401:     	movaps	0xc0(%rsp), %xmm9
1422a440a:     	movaps	0xb0(%rsp), %xmm10
1422a4413:     	movaps	0xa0(%rsp), %xmm11
1422a441c:     	movaps	0x90(%rsp), %xmm12
1422a4425:     	movaps	0x80(%rsp), %xmm13
1422a442e:     	movaps	0x70(%rsp), %xmm14
1422a4434:     	movaps	0x60(%rsp), %xmm15
1422a443a:     	addq	$0x108, %rsp            # imm = 0x108
1422a4441:     	popq	%rbx
1422a4442:     	popq	%rdi
1422a4443:     	popq	%rsi
1422a4444:     	popq	%r12
1422a4446:     	popq	%r13
1422a4448:     	popq	%r14
1422a444a:     	popq	%r15
1422a444c:     	popq	%rbp
1422a444d:     	retq
1422a444e:     	nop
1422a4450:     	movq	%rdx, 0x10(%rsp)
1422a4455:     	pushq	%rbp
1422a4456:     	pushq	%r15
1422a4458:     	pushq	%r14
1422a445a:     	pushq	%r13
1422a445c:     	pushq	%r12
1422a445e:     	pushq	%rsi
1422a445f:     	pushq	%rdi
1422a4460:     	pushq	%rbx
1422a4461:     	subq	$0x108, %rsp            # imm = 0x108
1422a4468:     	leaq	0x80(%rdx), %rbp
1422a446f:     	movaps	%xmm15, 0x60(%rsp)
1422a4475:     	movaps	%xmm14, 0x70(%rsp)
1422a447b:     	movaps	%xmm13, 0x80(%rsp)
1422a4484:     	movaps	%xmm12, 0x90(%rsp)
1422a448d:     	movaps	%xmm11, 0xa0(%rsp)
1422a4496:     	movaps	%xmm10, 0xb0(%rsp)
1422a449f:     	movaps	%xmm9, 0xc0(%rsp)
1422a44a8:     	movaps	%xmm8, 0xd0(%rsp)
1422a44b1:     	movaps	%xmm7, 0xe0(%rsp)
1422a44b9:     	movaps	%xmm6, 0xf0(%rsp)
1422a44c1:     	movl	$0x1e, %edx
1422a44c6:     	movl	$0x1, %r8d
1422a44cc:     	movq	0x200(%rbp), %rcx
1422a44d3:     	callq	0x140613c20 <.text+0x612c20>
1422a44d8:     	movaps	0xf0(%rsp), %xmm6
1422a44e0:     	movaps	0xe0(%rsp), %xmm7
1422a44e8:     	movaps	0xd0(%rsp), %xmm8
1422a44f1:     	movaps	0xc0(%rsp), %xmm9
1422a44fa:     	movaps	0xb0(%rsp), %xmm10
1422a4503:     	movaps	0xa0(%rsp), %xmm11
1422a450c:     	movaps	0x90(%rsp), %xmm12
1422a4515:     	movaps	0x80(%rsp), %xmm13
1422a451e:     	movaps	0x70(%rsp), %xmm14
1422a4524:     	movaps	0x60(%rsp), %xmm15
1422a452a:     	addq	$0x108, %rsp            # imm = 0x108
1422a4531:     	popq	%rbx
1422a4532:     	popq	%rdi
1422a4533:     	popq	%rsi
1422a4534:     	popq	%r12
1422a4536:     	popq	%r13
1422a4538:     	popq	%r14
1422a453a:     	popq	%r15
1422a453c:     	popq	%rbp
1422a453d:     	retq
1422a453e:     	nop
1422a4540:     	movq	%rdx, 0x10(%rsp)
1422a4545:     	pushq	%rbp
1422a4546:     	pushq	%r15
1422a4548:     	pushq	%r14
1422a454a:     	pushq	%r13
1422a454c:     	pushq	%r12
1422a454e:     	pushq	%rsi
1422a454f:     	pushq	%rdi
1422a4550:     	pushq	%rbx
1422a4551:     	subq	$0x108, %rsp            # imm = 0x108
1422a4558:     	leaq	0x80(%rdx), %rbp
1422a455f:     	movaps	%xmm15, 0x60(%rsp)
1422a4565:     	movaps	%xmm14, 0x70(%rsp)
1422a456b:     	movaps	%xmm13, 0x80(%rsp)
1422a4574:     	movaps	%xmm12, 0x90(%rsp)
1422a457d:     	movaps	%xmm11, 0xa0(%rsp)
1422a4586:     	movaps	%xmm10, 0xb0(%rsp)
1422a458f:     	movaps	%xmm9, 0xc0(%rsp)
1422a4598:     	movaps	%xmm8, 0xd0(%rsp)
1422a45a1:     	movaps	%xmm7, 0xe0(%rsp)
1422a45a9:     	movaps	%xmm6, 0xf0(%rsp)
1422a45b1:     	movl	$0x5, %edx
1422a45b6:     	movl	$0x1, %r8d
1422a45bc:     	movq	0x150(%rbp), %rcx
1422a45c3:     	callq	0x140613c20 <.text+0x612c20>
1422a45c8:     	movaps	0xf0(%rsp), %xmm6
1422a45d0:     	movaps	0xe0(%rsp), %xmm7
1422a45d8:     	movaps	0xd0(%rsp), %xmm8
1422a45e1:     	movaps	0xc0(%rsp), %xmm9
1422a45ea:     	movaps	0xb0(%rsp), %xmm10
1422a45f3:     	movaps	0xa0(%rsp), %xmm11
1422a45fc:     	movaps	0x90(%rsp), %xmm12
1422a4605:     	movaps	0x80(%rsp), %xmm13
1422a460e:     	movaps	0x70(%rsp), %xmm14
1422a4614:     	movaps	0x60(%rsp), %xmm15
1422a461a:     	addq	$0x108, %rsp            # imm = 0x108
1422a4621:     	popq	%rbx
1422a4622:     	popq	%rdi
1422a4623:     	popq	%rsi
1422a4624:     	popq	%r12
1422a4626:     	popq	%r13
1422a4628:     	popq	%r14
1422a462a:     	popq	%r15
1422a462c:     	popq	%rbp
1422a462d:     	retq
1422a462e:     	nop
1422a4630:     	movq	%rdx, 0x10(%rsp)
1422a4635:     	pushq	%rbp
1422a4636:     	pushq	%r15
1422a4638:     	pushq	%r14
1422a463a:     	pushq	%r13
1422a463c:     	pushq	%r12
1422a463e:     	pushq	%rsi
1422a463f:     	pushq	%rdi
1422a4640:     	pushq	%rbx
1422a4641:     	subq	$0x108, %rsp            # imm = 0x108
1422a4648:     	leaq	0x80(%rdx), %rbp
1422a464f:     	movaps	%xmm15, 0x60(%rsp)
1422a4655:     	movaps	%xmm14, 0x70(%rsp)
1422a465b:     	movaps	%xmm13, 0x80(%rsp)
1422a4664:     	movaps	%xmm12, 0x90(%rsp)
1422a466d:     	movaps	%xmm11, 0xa0(%rsp)
1422a4676:     	movaps	%xmm10, 0xb0(%rsp)
1422a467f:     	movaps	%xmm9, 0xc0(%rsp)
1422a4688:     	movaps	%xmm8, 0xd0(%rsp)
1422a4691:     	movaps	%xmm7, 0xe0(%rsp)
1422a4699:     	movaps	%xmm6, 0xf0(%rsp)
1422a46a1:     	movl	$0x15, %edx
1422a46a6:     	movl	$0x1, %r8d
1422a46ac:     	movq	0x1d8(%rbp), %rcx
1422a46b3:     	callq	0x140613c20 <.text+0x612c20>
1422a46b8:     	movaps	0xf0(%rsp), %xmm6
1422a46c0:     	movaps	0xe0(%rsp), %xmm7
1422a46c8:     	movaps	0xd0(%rsp), %xmm8
1422a46d1:     	movaps	0xc0(%rsp), %xmm9
1422a46da:     	movaps	0xb0(%rsp), %xmm10
1422a46e3:     	movaps	0xa0(%rsp), %xmm11
1422a46ec:     	movaps	0x90(%rsp), %xmm12
1422a46f5:     	movaps	0x80(%rsp), %xmm13
1422a46fe:     	movaps	0x70(%rsp), %xmm14
1422a4704:     	movaps	0x60(%rsp), %xmm15
1422a470a:     	addq	$0x108, %rsp            # imm = 0x108
1422a4711:     	popq	%rbx
1422a4712:     	popq	%rdi
1422a4713:     	popq	%rsi
1422a4714:     	popq	%r12
1422a4716:     	popq	%r13
1422a4718:     	popq	%r14
1422a471a:     	popq	%r15
1422a471c:     	popq	%rbp
1422a471d:     	retq
1422a471e:     	nop
1422a4720:     	movq	%rdx, 0x10(%rsp)
1422a4725:     	pushq	%rbp
1422a4726:     	pushq	%r15
1422a4728:     	pushq	%r14
1422a472a:     	pushq	%r13
1422a472c:     	pushq	%r12
1422a472e:     	pushq	%rsi
1422a472f:     	pushq	%rdi
1422a4730:     	pushq	%rbx
1422a4731:     	subq	$0x108, %rsp            # imm = 0x108
1422a4738:     	leaq	0x80(%rdx), %rbp
1422a473f:     	movaps	%xmm15, 0x60(%rsp)
1422a4745:     	movaps	%xmm14, 0x70(%rsp)
1422a474b:     	movaps	%xmm13, 0x80(%rsp)
1422a4754:     	movaps	%xmm12, 0x90(%rsp)
1422a475d:     	movaps	%xmm11, 0xa0(%rsp)
1422a4766:     	movaps	%xmm10, 0xb0(%rsp)
1422a476f:     	movaps	%xmm9, 0xc0(%rsp)
1422a4778:     	movaps	%xmm8, 0xd0(%rsp)
1422a4781:     	movaps	%xmm7, 0xe0(%rsp)
1422a4789:     	movaps	%xmm6, 0xf0(%rsp)
1422a4791:     	movl	$0xe, %edx
1422a4796:     	movl	$0x1, %r8d
1422a479c:     	movq	0x1c8(%rbp), %rcx
1422a47a3:     	callq	0x140613c20 <.text+0x612c20>
1422a47a8:     	movaps	0xf0(%rsp), %xmm6
1422a47b0:     	movaps	0xe0(%rsp), %xmm7
1422a47b8:     	movaps	0xd0(%rsp), %xmm8
1422a47c1:     	movaps	0xc0(%rsp), %xmm9
1422a47ca:     	movaps	0xb0(%rsp), %xmm10
1422a47d3:     	movaps	0xa0(%rsp), %xmm11
1422a47dc:     	movaps	0x90(%rsp), %xmm12
1422a47e5:     	movaps	0x80(%rsp), %xmm13
1422a47ee:     	movaps	0x70(%rsp), %xmm14
1422a47f4:     	movaps	0x60(%rsp), %xmm15
1422a47fa:     	addq	$0x108, %rsp            # imm = 0x108
1422a4801:     	popq	%rbx
1422a4802:     	popq	%rdi
1422a4803:     	popq	%rsi
1422a4804:     	popq	%r12
1422a4806:     	popq	%r13
1422a4808:     	popq	%r14
1422a480a:     	popq	%r15
1422a480c:     	popq	%rbp
1422a480d:     	retq
1422a480e:     	nop
1422a4810:     	movq	%rdx, 0x10(%rsp)
1422a4815:     	pushq	%rbp
1422a4816:     	pushq	%r15
1422a4818:     	pushq	%r14
1422a481a:     	pushq	%r13
1422a481c:     	pushq	%r12
1422a481e:     	pushq	%rsi
1422a481f:     	pushq	%rdi
1422a4820:     	pushq	%rbx
1422a4821:     	subq	$0x108, %rsp            # imm = 0x108
1422a4828:     	leaq	0x80(%rdx), %rbp
1422a482f:     	movaps	%xmm15, 0x60(%rsp)
1422a4835:     	movaps	%xmm14, 0x70(%rsp)
1422a483b:     	movaps	%xmm13, 0x80(%rsp)
1422a4844:     	movaps	%xmm12, 0x90(%rsp)
1422a484d:     	movaps	%xmm11, 0xa0(%rsp)
1422a4856:     	movaps	%xmm10, 0xb0(%rsp)
1422a485f:     	movaps	%xmm9, 0xc0(%rsp)
1422a4868:     	movaps	%xmm8, 0xd0(%rsp)
1422a4871:     	movaps	%xmm7, 0xe0(%rsp)
1422a4879:     	movaps	%xmm6, 0xf0(%rsp)
1422a4881:     	movl	$0x5, %edx
1422a4886:     	movl	$0x1, %r8d
1422a488c:     	movq	0x1e8(%rbp), %rcx
1422a4893:     	callq	0x140613c20 <.text+0x612c20>
1422a4898:     	movaps	0xf0(%rsp), %xmm6
1422a48a0:     	movaps	0xe0(%rsp), %xmm7
1422a48a8:     	movaps	0xd0(%rsp), %xmm8
1422a48b1:     	movaps	0xc0(%rsp), %xmm9
1422a48ba:     	movaps	0xb0(%rsp), %xmm10
1422a48c3:     	movaps	0xa0(%rsp), %xmm11
1422a48cc:     	movaps	0x90(%rsp), %xmm12
1422a48d5:     	movaps	0x80(%rsp), %xmm13
1422a48de:     	movaps	0x70(%rsp), %xmm14
1422a48e4:     	movaps	0x60(%rsp), %xmm15
1422a48ea:     	addq	$0x108, %rsp            # imm = 0x108
1422a48f1:     	popq	%rbx
1422a48f2:     	popq	%rdi
1422a48f3:     	popq	%rsi
1422a48f4:     	popq	%r12
1422a48f6:     	popq	%r13
1422a48f8:     	popq	%r14
1422a48fa:     	popq	%r15
1422a48fc:     	popq	%rbp
1422a48fd:     	retq
1422a48fe:     	nop
1422a4900:     	movq	%rdx, 0x10(%rsp)
1422a4905:     	pushq	%rbp
1422a4906:     	pushq	%r15
1422a4908:     	pushq	%r14
1422a490a:     	pushq	%r13
1422a490c:     	pushq	%r12
1422a490e:     	pushq	%rsi
1422a490f:     	pushq	%rdi
1422a4910:     	pushq	%rbx
1422a4911:     	subq	$0x108, %rsp            # imm = 0x108
1422a4918:     	leaq	0x80(%rdx), %rbp
1422a491f:     	movaps	%xmm15, 0x60(%rsp)
1422a4925:     	movaps	%xmm14, 0x70(%rsp)
1422a492b:     	movaps	%xmm13, 0x80(%rsp)
1422a4934:     	movaps	%xmm12, 0x90(%rsp)
1422a493d:     	movaps	%xmm11, 0xa0(%rsp)
1422a4946:     	movaps	%xmm10, 0xb0(%rsp)
1422a494f:     	movaps	%xmm9, 0xc0(%rsp)
1422a4958:     	movaps	%xmm8, 0xd0(%rsp)
1422a4961:     	movaps	%xmm7, 0xe0(%rsp)
1422a4969:     	movaps	%xmm6, 0xf0(%rsp)
1422a4971:     	movl	$0x10, %edx
1422a4976:     	movl	$0x1, %r8d
1422a497c:     	movq	0xa8(%rbp), %rcx
1422a4983:     	callq	0x140613c20 <.text+0x612c20>
1422a4988:     	movaps	0xf0(%rsp), %xmm6
1422a4990:     	movaps	0xe0(%rsp), %xmm7
1422a4998:     	movaps	0xd0(%rsp), %xmm8
1422a49a1:     	movaps	0xc0(%rsp), %xmm9
1422a49aa:     	movaps	0xb0(%rsp), %xmm10
1422a49b3:     	movaps	0xa0(%rsp), %xmm11
1422a49bc:     	movaps	0x90(%rsp), %xmm12
1422a49c5:     	movaps	0x80(%rsp), %xmm13
1422a49ce:     	movaps	0x70(%rsp), %xmm14
1422a49d4:     	movaps	0x60(%rsp), %xmm15
1422a49da:     	addq	$0x108, %rsp            # imm = 0x108
1422a49e1:     	popq	%rbx
1422a49e2:     	popq	%rdi
1422a49e3:     	popq	%rsi
1422a49e4:     	popq	%r12
1422a49e6:     	popq	%r13
1422a49e8:     	popq	%r14
1422a49ea:     	popq	%r15
1422a49ec:     	popq	%rbp
1422a49ed:     	retq
1422a49ee:     	nop
1422a49f0:     	movq	%rdx, 0x10(%rsp)
1422a49f5:     	pushq	%rbp
1422a49f6:     	pushq	%r15
1422a49f8:     	pushq	%r14
1422a49fa:     	pushq	%r13
1422a49fc:     	pushq	%r12
1422a49fe:     	pushq	%rsi
1422a49ff:     	pushq	%rdi
1422a4a00:     	pushq	%rbx
1422a4a01:     	subq	$0x108, %rsp            # imm = 0x108
1422a4a08:     	leaq	0x80(%rdx), %rbp
1422a4a0f:     	movaps	%xmm15, 0x60(%rsp)
1422a4a15:     	movaps	%xmm14, 0x70(%rsp)
1422a4a1b:     	movaps	%xmm13, 0x80(%rsp)
1422a4a24:     	movaps	%xmm12, 0x90(%rsp)
1422a4a2d:     	movaps	%xmm11, 0xa0(%rsp)
1422a4a36:     	movaps	%xmm10, 0xb0(%rsp)
1422a4a3f:     	movaps	%xmm9, 0xc0(%rsp)
1422a4a48:     	movaps	%xmm8, 0xd0(%rsp)
1422a4a51:     	movaps	%xmm7, 0xe0(%rsp)
1422a4a59:     	movaps	%xmm6, 0xf0(%rsp)
1422a4a61:     	movl	$0x11, %edx
1422a4a66:     	movl	$0x1, %r8d
1422a4a6c:     	movq	0x1d8(%rbp), %rcx
1422a4a73:     	callq	0x140613c20 <.text+0x612c20>
1422a4a78:     	movaps	0xf0(%rsp), %xmm6
1422a4a80:     	movaps	0xe0(%rsp), %xmm7
1422a4a88:     	movaps	0xd0(%rsp), %xmm8
1422a4a91:     	movaps	0xc0(%rsp), %xmm9
1422a4a9a:     	movaps	0xb0(%rsp), %xmm10
1422a4aa3:     	movaps	0xa0(%rsp), %xmm11
1422a4aac:     	movaps	0x90(%rsp), %xmm12
1422a4ab5:     	movaps	0x80(%rsp), %xmm13
1422a4abe:     	movaps	0x70(%rsp), %xmm14
1422a4ac4:     	movaps	0x60(%rsp), %xmm15
1422a4aca:     	addq	$0x108, %rsp            # imm = 0x108
1422a4ad1:     	popq	%rbx
1422a4ad2:     	popq	%rdi
1422a4ad3:     	popq	%rsi
1422a4ad4:     	popq	%r12
1422a4ad6:     	popq	%r13
1422a4ad8:     	popq	%r14
1422a4ada:     	popq	%r15
1422a4adc:     	popq	%rbp
1422a4add:     	retq
1422a4ade:     	nop
1422a4ae0:     	movq	%rdx, 0x10(%rsp)
1422a4ae5:     	pushq	%rbp
1422a4ae6:     	pushq	%r15
1422a4ae8:     	pushq	%r14
1422a4aea:     	pushq	%r13
1422a4aec:     	pushq	%r12
1422a4aee:     	pushq	%rsi
1422a4aef:     	pushq	%rdi
1422a4af0:     	pushq	%rbx
1422a4af1:     	subq	$0x108, %rsp            # imm = 0x108
1422a4af8:     	leaq	0x80(%rdx), %rbp
1422a4aff:     	movaps	%xmm15, 0x60(%rsp)
1422a4b05:     	movaps	%xmm14, 0x70(%rsp)
1422a4b0b:     	movaps	%xmm13, 0x80(%rsp)
1422a4b14:     	movaps	%xmm12, 0x90(%rsp)
1422a4b1d:     	movaps	%xmm11, 0xa0(%rsp)
1422a4b26:     	movaps	%xmm10, 0xb0(%rsp)
1422a4b2f:     	movaps	%xmm9, 0xc0(%rsp)
1422a4b38:     	movaps	%xmm8, 0xd0(%rsp)
1422a4b41:     	movaps	%xmm7, 0xe0(%rsp)
1422a4b49:     	movaps	%xmm6, 0xf0(%rsp)
1422a4b51:     	movl	$0x14, %edx
1422a4b56:     	movl	$0x1, %r8d
1422a4b5c:     	movq	0x150(%rbp), %rcx
1422a4b63:     	callq	0x140613c20 <.text+0x612c20>
1422a4b68:     	movaps	0xf0(%rsp), %xmm6
1422a4b70:     	movaps	0xe0(%rsp), %xmm7
1422a4b78:     	movaps	0xd0(%rsp), %xmm8
1422a4b81:     	movaps	0xc0(%rsp), %xmm9
1422a4b8a:     	movaps	0xb0(%rsp), %xmm10
1422a4b93:     	movaps	0xa0(%rsp), %xmm11
1422a4b9c:     	movaps	0x90(%rsp), %xmm12
1422a4ba5:     	movaps	0x80(%rsp), %xmm13
1422a4bae:     	movaps	0x70(%rsp), %xmm14
1422a4bb4:     	movaps	0x60(%rsp), %xmm15
1422a4bba:     	addq	$0x108, %rsp            # imm = 0x108
1422a4bc1:     	popq	%rbx
1422a4bc2:     	popq	%rdi
1422a4bc3:     	popq	%rsi
1422a4bc4:     	popq	%r12
1422a4bc6:     	popq	%r13
1422a4bc8:     	popq	%r14
1422a4bca:     	popq	%r15
1422a4bcc:     	popq	%rbp
1422a4bcd:     	retq
1422a4bce:     	nop
1422a4bd0:     	movq	%rdx, 0x10(%rsp)
1422a4bd5:     	pushq	%rbp
1422a4bd6:     	pushq	%r15
1422a4bd8:     	pushq	%r14
1422a4bda:     	pushq	%r13
1422a4bdc:     	pushq	%r12
1422a4bde:     	pushq	%rsi
1422a4bdf:     	pushq	%rdi
1422a4be0:     	pushq	%rbx
1422a4be1:     	subq	$0x108, %rsp            # imm = 0x108
1422a4be8:     	leaq	0x80(%rdx), %rbp
1422a4bef:     	movaps	%xmm15, 0x60(%rsp)
1422a4bf5:     	movaps	%xmm14, 0x70(%rsp)
1422a4bfb:     	movaps	%xmm13, 0x80(%rsp)
1422a4c04:     	movaps	%xmm12, 0x90(%rsp)
1422a4c0d:     	movaps	%xmm11, 0xa0(%rsp)
1422a4c16:     	movaps	%xmm10, 0xb0(%rsp)
1422a4c1f:     	movaps	%xmm9, 0xc0(%rsp)
1422a4c28:     	movaps	%xmm8, 0xd0(%rsp)
1422a4c31:     	movaps	%xmm7, 0xe0(%rsp)
1422a4c39:     	movaps	%xmm6, 0xf0(%rsp)
1422a4c41:     	movl	$0x28, %edx
1422a4c46:     	movl	$0x8, %r8d
1422a4c4c:     	movq	0x1d8(%rbp), %rcx
1422a4c53:     	callq	0x140613c20 <.text+0x612c20>
1422a4c58:     	movaps	0xf0(%rsp), %xmm6
1422a4c60:     	movaps	0xe0(%rsp), %xmm7
1422a4c68:     	movaps	0xd0(%rsp), %xmm8
1422a4c71:     	movaps	0xc0(%rsp), %xmm9
1422a4c7a:     	movaps	0xb0(%rsp), %xmm10
1422a4c83:     	movaps	0xa0(%rsp), %xmm11
1422a4c8c:     	movaps	0x90(%rsp), %xmm12
1422a4c95:     	movaps	0x80(%rsp), %xmm13
1422a4c9e:     	movaps	0x70(%rsp), %xmm14
1422a4ca4:     	movaps	0x60(%rsp), %xmm15
1422a4caa:     	addq	$0x108, %rsp            # imm = 0x108
1422a4cb1:     	popq	%rbx
1422a4cb2:     	popq	%rdi
1422a4cb3:     	popq	%rsi
1422a4cb4:     	popq	%r12
1422a4cb6:     	popq	%r13
1422a4cb8:     	popq	%r14
1422a4cba:     	popq	%r15
1422a4cbc:     	popq	%rbp
1422a4cbd:     	retq
1422a4cbe:     	nop
1422a4cc0:     	movq	%rdx, 0x10(%rsp)
1422a4cc5:     	pushq	%rbp
1422a4cc6:     	pushq	%r15
1422a4cc8:     	pushq	%r14
1422a4cca:     	pushq	%r13
1422a4ccc:     	pushq	%r12
1422a4cce:     	pushq	%rsi
1422a4ccf:     	pushq	%rdi
1422a4cd0:     	pushq	%rbx
1422a4cd1:     	subq	$0x108, %rsp            # imm = 0x108
1422a4cd8:     	leaq	0x80(%rdx), %rbp
1422a4cdf:     	movaps	%xmm15, 0x60(%rsp)
1422a4ce5:     	movaps	%xmm14, 0x70(%rsp)
1422a4ceb:     	movaps	%xmm13, 0x80(%rsp)
1422a4cf4:     	movaps	%xmm12, 0x90(%rsp)
1422a4cfd:     	movaps	%xmm11, 0xa0(%rsp)
1422a4d06:     	movaps	%xmm10, 0xb0(%rsp)
1422a4d0f:     	movaps	%xmm9, 0xc0(%rsp)
1422a4d18:     	movaps	%xmm8, 0xd0(%rsp)
1422a4d21:     	movaps	%xmm7, 0xe0(%rsp)
1422a4d29:     	movaps	%xmm6, 0xf0(%rsp)
1422a4d31:     	movl	$0x1b, %edx
1422a4d36:     	movl	$0x1, %r8d
1422a4d3c:     	movq	0x1e0(%rbp), %rcx
1422a4d43:     	callq	0x140613c20 <.text+0x612c20>
1422a4d48:     	movaps	0xf0(%rsp), %xmm6
1422a4d50:     	movaps	0xe0(%rsp), %xmm7
1422a4d58:     	movaps	0xd0(%rsp), %xmm8
1422a4d61:     	movaps	0xc0(%rsp), %xmm9
1422a4d6a:     	movaps	0xb0(%rsp), %xmm10
1422a4d73:     	movaps	0xa0(%rsp), %xmm11
1422a4d7c:     	movaps	0x90(%rsp), %xmm12
1422a4d85:     	movaps	0x80(%rsp), %xmm13
1422a4d8e:     	movaps	0x70(%rsp), %xmm14
1422a4d94:     	movaps	0x60(%rsp), %xmm15
1422a4d9a:     	addq	$0x108, %rsp            # imm = 0x108
1422a4da1:     	popq	%rbx
1422a4da2:     	popq	%rdi
1422a4da3:     	popq	%rsi
1422a4da4:     	popq	%r12
1422a4da6:     	popq	%r13
1422a4da8:     	popq	%r14
1422a4daa:     	popq	%r15
1422a4dac:     	popq	%rbp
1422a4dad:     	retq
1422a4dae:     	nop
1422a4db0:     	movq	%rdx, 0x10(%rsp)
1422a4db5:     	pushq	%rbp
1422a4db6:     	pushq	%r15
1422a4db8:     	pushq	%r14
1422a4dba:     	pushq	%r13
1422a4dbc:     	pushq	%r12
1422a4dbe:     	pushq	%rsi
1422a4dbf:     	pushq	%rdi
1422a4dc0:     	pushq	%rbx
1422a4dc1:     	subq	$0x108, %rsp            # imm = 0x108
1422a4dc8:     	leaq	0x80(%rdx), %rbp
1422a4dcf:     	movaps	%xmm15, 0x60(%rsp)
1422a4dd5:     	movaps	%xmm14, 0x70(%rsp)
1422a4ddb:     	movaps	%xmm13, 0x80(%rsp)
1422a4de4:     	movaps	%xmm12, 0x90(%rsp)
1422a4ded:     	movaps	%xmm11, 0xa0(%rsp)
1422a4df6:     	movaps	%xmm10, 0xb0(%rsp)
1422a4dff:     	movaps	%xmm9, 0xc0(%rsp)
1422a4e08:     	movaps	%xmm8, 0xd0(%rsp)
1422a4e11:     	movaps	%xmm7, 0xe0(%rsp)
1422a4e19:     	movaps	%xmm6, 0xf0(%rsp)
1422a4e21:     	movq	0xd8(%rbp), %rdx
1422a4e28:     	testq	%rdx, %rdx
1422a4e2b:     	je	0x1422a4e43 <.text+0x22a3e43>
1422a4e2d:     	movq	0xe0(%rbp), %rcx
1422a4e34:     	shlq	$0x3, %rdx
1422a4e38:     	movl	$0x8, %r8d
1422a4e3e:     	callq	0x140613c20 <.text+0x612c20>
1422a4e43:     	movaps	0xf0(%rsp), %xmm6
1422a4e4b:     	movaps	0xe0(%rsp), %xmm7
1422a4e53:     	movaps	0xd0(%rsp), %xmm8
1422a4e5c:     	movaps	0xc0(%rsp), %xmm9
1422a4e65:     	movaps	0xb0(%rsp), %xmm10
1422a4e6e:     	movaps	0xa0(%rsp), %xmm11
1422a4e77:     	movaps	0x90(%rsp), %xmm12
1422a4e80:     	movaps	0x80(%rsp), %xmm13
1422a4e89:     	movaps	0x70(%rsp), %xmm14
1422a4e8f:     	movaps	0x60(%rsp), %xmm15
1422a4e95:     	addq	$0x108, %rsp            # imm = 0x108
1422a4e9c:     	popq	%rbx
1422a4e9d:     	popq	%rdi
1422a4e9e:     	popq	%rsi
1422a4e9f:     	popq	%r12
1422a4ea1:     	popq	%r13
1422a4ea3:     	popq	%r14
1422a4ea5:     	popq	%r15
1422a4ea7:     	popq	%rbp
1422a4ea8:     	retq
1422a4ea9:     	nopl	(%rax)
1422a4eb0:     	movq	%rdx, 0x10(%rsp)
1422a4eb5:     	pushq	%rbp
1422a4eb6:     	pushq	%r15
1422a4eb8:     	pushq	%r14
1422a4eba:     	pushq	%r13
1422a4ebc:     	pushq	%r12
1422a4ebe:     	pushq	%rsi
1422a4ebf:     	pushq	%rdi
1422a4ec0:     	pushq	%rbx
1422a4ec1:     	subq	$0x108, %rsp            # imm = 0x108
1422a4ec8:     	leaq	0x80(%rdx), %rbp
1422a4ecf:     	movaps	%xmm15, 0x60(%rsp)
1422a4ed5:     	movaps	%xmm14, 0x70(%rsp)
1422a4edb:     	movaps	%xmm13, 0x80(%rsp)
1422a4ee4:     	movaps	%xmm12, 0x90(%rsp)
1422a4eed:     	movaps	%xmm11, 0xa0(%rsp)
1422a4ef6:     	movaps	%xmm10, 0xb0(%rsp)
1422a4eff:     	movaps	%xmm9, 0xc0(%rsp)
1422a4f08:     	movaps	%xmm8, 0xd0(%rsp)
1422a4f11:     	movaps	%xmm7, 0xe0(%rsp)
1422a4f19:     	movaps	%xmm6, 0xf0(%rsp)
1422a4f21:     	movq	0xd8(%rbp), %rdx
1422a4f28:     	testq	%rdx, %rdx
1422a4f2b:     	je	0x1422a4f43 <.text+0x22a3f43>
1422a4f2d:     	movq	0xe0(%rbp), %rcx
1422a4f34:     	shlq	$0x3, %rdx
1422a4f38:     	movl	$0x8, %r8d
1422a4f3e:     	callq	0x140613c20 <.text+0x612c20>
1422a4f43:     	movaps	0xf0(%rsp), %xmm6
1422a4f4b:     	movaps	0xe0(%rsp), %xmm7
1422a4f53:     	movaps	0xd0(%rsp), %xmm8
1422a4f5c:     	movaps	0xc0(%rsp), %xmm9
1422a4f65:     	movaps	0xb0(%rsp), %xmm10
1422a4f6e:     	movaps	0xa0(%rsp), %xmm11
1422a4f77:     	movaps	0x90(%rsp), %xmm12
1422a4f80:     	movaps	0x80(%rsp), %xmm13
1422a4f89:     	movaps	0x70(%rsp), %xmm14
1422a4f8f:     	movaps	0x60(%rsp), %xmm15
1422a4f95:     	addq	$0x108, %rsp            # imm = 0x108
1422a4f9c:     	popq	%rbx
1422a4f9d:     	popq	%rdi
1422a4f9e:     	popq	%rsi
1422a4f9f:     	popq	%r12
1422a4fa1:     	popq	%r13
1422a4fa3:     	popq	%r14
1422a4fa5:     	popq	%r15
1422a4fa7:     	popq	%rbp
1422a4fa8:     	retq
1422a4fa9:     	nopl	(%rax)
1422a4fb0:     	movq	%rdx, 0x10(%rsp)
1422a4fb5:     	pushq	%rbp
1422a4fb6:     	pushq	%r15
1422a4fb8:     	pushq	%r14
1422a4fba:     	pushq	%r13
1422a4fbc:     	pushq	%r12
1422a4fbe:     	pushq	%rsi
1422a4fbf:     	pushq	%rdi
1422a4fc0:     	pushq	%rbx
1422a4fc1:     	subq	$0x108, %rsp            # imm = 0x108
1422a4fc8:     	leaq	0x80(%rdx), %rbp
1422a4fcf:     	movaps	%xmm15, 0x60(%rsp)
1422a4fd5:     	movaps	%xmm14, 0x70(%rsp)
1422a4fdb:     	movaps	%xmm13, 0x80(%rsp)
1422a4fe4:     	movaps	%xmm12, 0x90(%rsp)
1422a4fed:     	movaps	%xmm11, 0xa0(%rsp)
1422a4ff6:     	movaps	%xmm10, 0xb0(%rsp)
1422a4fff:     	movaps	%xmm9, 0xc0(%rsp)
1422a5008:     	movaps	%xmm8, 0xd0(%rsp)
1422a5011:     	movaps	%xmm7, 0xe0(%rsp)
1422a5019:     	movaps	%xmm6, 0xf0(%rsp)
1422a5021:     	movq	0xd8(%rbp), %rdx
1422a5028:     	testq	%rdx, %rdx
1422a502b:     	je	0x1422a5043 <.text+0x22a4043>
1422a502d:     	movq	0xe0(%rbp), %rcx
1422a5034:     	shlq	$0x3, %rdx
1422a5038:     	movl	$0x8, %r8d
1422a503e:     	callq	0x140613c20 <.text+0x612c20>
1422a5043:     	movaps	0xf0(%rsp), %xmm6
1422a504b:     	movaps	0xe0(%rsp), %xmm7
1422a5053:     	movaps	0xd0(%rsp), %xmm8
1422a505c:     	movaps	0xc0(%rsp), %xmm9
1422a5065:     	movaps	0xb0(%rsp), %xmm10
1422a506e:     	movaps	0xa0(%rsp), %xmm11
1422a5077:     	movaps	0x90(%rsp), %xmm12
1422a5080:     	movaps	0x80(%rsp), %xmm13
1422a5089:     	movaps	0x70(%rsp), %xmm14
1422a508f:     	movaps	0x60(%rsp), %xmm15
1422a5095:     	addq	$0x108, %rsp            # imm = 0x108
1422a509c:     	popq	%rbx
1422a509d:     	popq	%rdi
1422a509e:     	popq	%rsi
1422a509f:     	popq	%r12
1422a50a1:     	popq	%r13
1422a50a3:     	popq	%r14
1422a50a5:     	popq	%r15
1422a50a7:     	popq	%rbp
1422a50a8:     	retq
1422a50a9:     	nopl	(%rax)
1422a50b0:     	movq	%rdx, 0x10(%rsp)
1422a50b5:     	pushq	%rbp
1422a50b6:     	pushq	%r15
1422a50b8:     	pushq	%r14
1422a50ba:     	pushq	%r13
1422a50bc:     	pushq	%r12
1422a50be:     	pushq	%rsi
1422a50bf:     	pushq	%rdi
1422a50c0:     	pushq	%rbx
1422a50c1:     	subq	$0x108, %rsp            # imm = 0x108
1422a50c8:     	leaq	0x80(%rdx), %rbp
1422a50cf:     	movaps	%xmm15, 0x60(%rsp)
1422a50d5:     	movaps	%xmm14, 0x70(%rsp)
1422a50db:     	movaps	%xmm13, 0x80(%rsp)
1422a50e4:     	movaps	%xmm12, 0x90(%rsp)
1422a50ed:     	movaps	%xmm11, 0xa0(%rsp)
1422a50f6:     	movaps	%xmm10, 0xb0(%rsp)
1422a50ff:     	movaps	%xmm9, 0xc0(%rsp)
1422a5108:     	movaps	%xmm8, 0xd0(%rsp)
1422a5111:     	movaps	%xmm7, 0xe0(%rsp)
1422a5119:     	movaps	%xmm6, 0xf0(%rsp)
1422a5121:     	movq	0xd8(%rbp), %rdx
1422a5128:     	testq	%rdx, %rdx
1422a512b:     	je	0x1422a5143 <.text+0x22a4143>
1422a512d:     	movq	0xe0(%rbp), %rcx
1422a5134:     	shlq	$0x3, %rdx
1422a5138:     	movl	$0x8, %r8d
1422a513e:     	callq	0x140613c20 <.text+0x612c20>
1422a5143:     	movaps	0xf0(%rsp), %xmm6
1422a514b:     	movaps	0xe0(%rsp), %xmm7
1422a5153:     	movaps	0xd0(%rsp), %xmm8
1422a515c:     	movaps	0xc0(%rsp), %xmm9
1422a5165:     	movaps	0xb0(%rsp), %xmm10
1422a516e:     	movaps	0xa0(%rsp), %xmm11
1422a5177:     	movaps	0x90(%rsp), %xmm12
1422a5180:     	movaps	0x80(%rsp), %xmm13
1422a5189:     	movaps	0x70(%rsp), %xmm14
1422a518f:     	movaps	0x60(%rsp), %xmm15
1422a5195:     	addq	$0x108, %rsp            # imm = 0x108
1422a519c:     	popq	%rbx
1422a519d:     	popq	%rdi
1422a519e:     	popq	%rsi
1422a519f:     	popq	%r12
1422a51a1:     	popq	%r13
1422a51a3:     	popq	%r14
1422a51a5:     	popq	%r15
1422a51a7:     	popq	%rbp
1422a51a8:     	retq
1422a51a9:     	nopl	(%rax)
1422a51b0:     	movq	%rdx, 0x10(%rsp)
1422a51b5:     	pushq	%rbp
1422a51b6:     	pushq	%r15
1422a51b8:     	pushq	%r14
1422a51ba:     	pushq	%r13
1422a51bc:     	pushq	%r12
1422a51be:     	pushq	%rsi
1422a51bf:     	pushq	%rdi
1422a51c0:     	pushq	%rbx
1422a51c1:     	subq	$0x108, %rsp            # imm = 0x108
1422a51c8:     	leaq	0x80(%rdx), %rbp
1422a51cf:     	movaps	%xmm15, 0x60(%rsp)
1422a51d5:     	movaps	%xmm14, 0x70(%rsp)
1422a51db:     	movaps	%xmm13, 0x80(%rsp)
1422a51e4:     	movaps	%xmm12, 0x90(%rsp)
1422a51ed:     	movaps	%xmm11, 0xa0(%rsp)
1422a51f6:     	movaps	%xmm10, 0xb0(%rsp)
1422a51ff:     	movaps	%xmm9, 0xc0(%rsp)
1422a5208:     	movaps	%xmm8, 0xd0(%rsp)
1422a5211:     	movaps	%xmm7, 0xe0(%rsp)
1422a5219:     	movaps	%xmm6, 0xf0(%rsp)
1422a5221:     	movl	$0x13, %edx
1422a5226:     	movl	$0x1, %r8d
1422a522c:     	movq	0x1c0(%rbp), %rcx
1422a5233:     	callq	0x140613c20 <.text+0x612c20>
1422a5238:     	movaps	0xf0(%rsp), %xmm6
1422a5240:     	movaps	0xe0(%rsp), %xmm7
1422a5248:     	movaps	0xd0(%rsp), %xmm8
1422a5251:     	movaps	0xc0(%rsp), %xmm9
1422a525a:     	movaps	0xb0(%rsp), %xmm10
1422a5263:     	movaps	0xa0(%rsp), %xmm11
1422a526c:     	movaps	0x90(%rsp), %xmm12
1422a5275:     	movaps	0x80(%rsp), %xmm13
1422a527e:     	movaps	0x70(%rsp), %xmm14
1422a5284:     	movaps	0x60(%rsp), %xmm15
1422a528a:     	addq	$0x108, %rsp            # imm = 0x108
1422a5291:     	popq	%rbx
1422a5292:     	popq	%rdi
1422a5293:     	popq	%rsi
1422a5294:     	popq	%r12
1422a5296:     	popq	%r13
1422a5298:     	popq	%r14
1422a529a:     	popq	%r15
1422a529c:     	popq	%rbp
1422a529d:     	retq
1422a529e:     	nop
1422a52a0:     	movq	%rdx, 0x10(%rsp)
1422a52a5:     	pushq	%rbp
1422a52a6:     	pushq	%r15
1422a52a8:     	pushq	%r14
1422a52aa:     	pushq	%r13
1422a52ac:     	pushq	%r12
1422a52ae:     	pushq	%rsi
1422a52af:     	pushq	%rdi
1422a52b0:     	pushq	%rbx
1422a52b1:     	subq	$0x108, %rsp            # imm = 0x108
1422a52b8:     	leaq	0x80(%rdx), %rbp
1422a52bf:     	movaps	%xmm15, 0x60(%rsp)
1422a52c5:     	movaps	%xmm14, 0x70(%rsp)
1422a52cb:     	movaps	%xmm13, 0x80(%rsp)
1422a52d4:     	movaps	%xmm12, 0x90(%rsp)
1422a52dd:     	movaps	%xmm11, 0xa0(%rsp)
1422a52e6:     	movaps	%xmm10, 0xb0(%rsp)
1422a52ef:     	movaps	%xmm9, 0xc0(%rsp)
1422a52f8:     	movaps	%xmm8, 0xd0(%rsp)
1422a5301:     	movaps	%xmm7, 0xe0(%rsp)
1422a5309:     	movaps	%xmm6, 0xf0(%rsp)
1422a5311:     	movl	$0x5, %edx
1422a5316:     	movl	$0x1, %r8d
1422a531c:     	movq	0x1e8(%rbp), %rcx
1422a5323:     	callq	0x140613c20 <.text+0x612c20>
1422a5328:     	movaps	0xf0(%rsp), %xmm6
1422a5330:     	movaps	0xe0(%rsp), %xmm7
1422a5338:     	movaps	0xd0(%rsp), %xmm8
1422a5341:     	movaps	0xc0(%rsp), %xmm9
1422a534a:     	movaps	0xb0(%rsp), %xmm10
1422a5353:     	movaps	0xa0(%rsp), %xmm11
1422a535c:     	movaps	0x90(%rsp), %xmm12
1422a5365:     	movaps	0x80(%rsp), %xmm13
1422a536e:     	movaps	0x70(%rsp), %xmm14
1422a5374:     	movaps	0x60(%rsp), %xmm15
1422a537a:     	addq	$0x108, %rsp            # imm = 0x108
1422a5381:     	popq	%rbx
1422a5382:     	popq	%rdi
1422a5383:     	popq	%rsi
1422a5384:     	popq	%r12
1422a5386:     	popq	%r13
1422a5388:     	popq	%r14
1422a538a:     	popq	%r15
1422a538c:     	popq	%rbp
1422a538d:     	retq
1422a538e:     	nop
1422a5390:     	movq	%rdx, 0x10(%rsp)
1422a5395:     	pushq	%rbp
1422a5396:     	pushq	%r15
1422a5398:     	pushq	%r14
1422a539a:     	pushq	%r13
1422a539c:     	pushq	%r12
1422a539e:     	pushq	%rsi
1422a539f:     	pushq	%rdi
1422a53a0:     	pushq	%rbx
1422a53a1:     	subq	$0x108, %rsp            # imm = 0x108
1422a53a8:     	leaq	0x80(%rdx), %rbp
1422a53af:     	movaps	%xmm15, 0x60(%rsp)
1422a53b5:     	movaps	%xmm14, 0x70(%rsp)
1422a53bb:     	movaps	%xmm13, 0x80(%rsp)
1422a53c4:     	movaps	%xmm12, 0x90(%rsp)
1422a53cd:     	movaps	%xmm11, 0xa0(%rsp)
1422a53d6:     	movaps	%xmm10, 0xb0(%rsp)
1422a53df:     	movaps	%xmm9, 0xc0(%rsp)
1422a53e8:     	movaps	%xmm8, 0xd0(%rsp)
1422a53f1:     	movaps	%xmm7, 0xe0(%rsp)
1422a53f9:     	movaps	%xmm6, 0xf0(%rsp)
1422a5401:     	movl	$0x1b, %edx
1422a5406:     	movl	$0x1, %r8d
1422a540c:     	movq	0x1d8(%rbp), %rcx
1422a5413:     	callq	0x140613c20 <.text+0x612c20>
1422a5418:     	movaps	0xf0(%rsp), %xmm6
1422a5420:     	movaps	0xe0(%rsp), %xmm7
1422a5428:     	movaps	0xd0(%rsp), %xmm8
1422a5431:     	movaps	0xc0(%rsp), %xmm9
1422a543a:     	movaps	0xb0(%rsp), %xmm10
1422a5443:     	movaps	0xa0(%rsp), %xmm11
1422a544c:     	movaps	0x90(%rsp), %xmm12
1422a5455:     	movaps	0x80(%rsp), %xmm13
1422a545e:     	movaps	0x70(%rsp), %xmm14
1422a5464:     	movaps	0x60(%rsp), %xmm15
1422a546a:     	addq	$0x108, %rsp            # imm = 0x108
1422a5471:     	popq	%rbx
1422a5472:     	popq	%rdi
1422a5473:     	popq	%rsi
1422a5474:     	popq	%r12
1422a5476:     	popq	%r13
1422a5478:     	popq	%r14
1422a547a:     	popq	%r15
1422a547c:     	popq	%rbp
1422a547d:     	retq
1422a547e:     	nop
1422a5480:     	movq	%rdx, 0x10(%rsp)
1422a5485:     	pushq	%rbp
1422a5486:     	pushq	%r15
1422a5488:     	pushq	%r14
1422a548a:     	pushq	%r13
1422a548c:     	pushq	%r12
1422a548e:     	pushq	%rsi
1422a548f:     	pushq	%rdi
1422a5490:     	pushq	%rbx
1422a5491:     	subq	$0x108, %rsp            # imm = 0x108
1422a5498:     	leaq	0x80(%rdx), %rbp
1422a549f:     	movaps	%xmm15, 0x60(%rsp)
1422a54a5:     	movaps	%xmm14, 0x70(%rsp)
1422a54ab:     	movaps	%xmm13, 0x80(%rsp)
1422a54b4:     	movaps	%xmm12, 0x90(%rsp)
1422a54bd:     	movaps	%xmm11, 0xa0(%rsp)
1422a54c6:     	movaps	%xmm10, 0xb0(%rsp)
1422a54cf:     	movaps	%xmm9, 0xc0(%rsp)
1422a54d8:     	movaps	%xmm8, 0xd0(%rsp)
1422a54e1:     	movaps	%xmm7, 0xe0(%rsp)
1422a54e9:     	movaps	%xmm6, 0xf0(%rsp)
1422a54f1:     	movl	$0x10, %edx
1422a54f6:     	movl	$0x1, %r8d
1422a54fc:     	movq	0x1d8(%rbp), %rcx
1422a5503:     	callq	0x140613c20 <.text+0x612c20>
1422a5508:     	movaps	0xf0(%rsp), %xmm6
1422a5510:     	movaps	0xe0(%rsp), %xmm7
1422a5518:     	movaps	0xd0(%rsp), %xmm8
1422a5521:     	movaps	0xc0(%rsp), %xmm9
1422a552a:     	movaps	0xb0(%rsp), %xmm10
1422a5533:     	movaps	0xa0(%rsp), %xmm11
1422a553c:     	movaps	0x90(%rsp), %xmm12
1422a5545:     	movaps	0x80(%rsp), %xmm13
1422a554e:     	movaps	0x70(%rsp), %xmm14
1422a5554:     	movaps	0x60(%rsp), %xmm15
1422a555a:     	addq	$0x108, %rsp            # imm = 0x108
1422a5561:     	popq	%rbx
1422a5562:     	popq	%rdi
1422a5563:     	popq	%rsi
1422a5564:     	popq	%r12
1422a5566:     	popq	%r13
1422a5568:     	popq	%r14
1422a556a:     	popq	%r15
1422a556c:     	popq	%rbp
1422a556d:     	retq
1422a556e:     	nop
1422a5570:     	movq	%rdx, 0x10(%rsp)
1422a5575:     	pushq	%rbp
1422a5576:     	pushq	%r15
1422a5578:     	pushq	%r14
1422a557a:     	pushq	%r13
1422a557c:     	pushq	%r12
1422a557e:     	pushq	%rsi
1422a557f:     	pushq	%rdi
1422a5580:     	pushq	%rbx
1422a5581:     	subq	$0x108, %rsp            # imm = 0x108
1422a5588:     	leaq	0x80(%rdx), %rbp
1422a558f:     	movaps	%xmm15, 0x60(%rsp)
1422a5595:     	movaps	%xmm14, 0x70(%rsp)
1422a559b:     	movaps	%xmm13, 0x80(%rsp)
1422a55a4:     	movaps	%xmm12, 0x90(%rsp)
1422a55ad:     	movaps	%xmm11, 0xa0(%rsp)
1422a55b6:     	movaps	%xmm10, 0xb0(%rsp)
1422a55bf:     	movaps	%xmm9, 0xc0(%rsp)
1422a55c8:     	movaps	%xmm8, 0xd0(%rsp)
1422a55d1:     	movaps	%xmm7, 0xe0(%rsp)
1422a55d9:     	movaps	%xmm6, 0xf0(%rsp)
1422a55e1:     	movl	$0xa0, %edx
1422a55e6:     	movl	$0x8, %r8d
1422a55ec:     	movq	0x1f0(%rbp), %rcx
1422a55f3:     	callq	0x140613c20 <.text+0x612c20>
1422a55f8:     	movaps	0xf0(%rsp), %xmm6
1422a5600:     	movaps	0xe0(%rsp), %xmm7
1422a5608:     	movaps	0xd0(%rsp), %xmm8
1422a5611:     	movaps	0xc0(%rsp), %xmm9
1422a561a:     	movaps	0xb0(%rsp), %xmm10
1422a5623:     	movaps	0xa0(%rsp), %xmm11
1422a562c:     	movaps	0x90(%rsp), %xmm12
1422a5635:     	movaps	0x80(%rsp), %xmm13
1422a563e:     	movaps	0x70(%rsp), %xmm14
1422a5644:     	movaps	0x60(%rsp), %xmm15
1422a564a:     	addq	$0x108, %rsp            # imm = 0x108
1422a5651:     	popq	%rbx
1422a5652:     	popq	%rdi
1422a5653:     	popq	%rsi
1422a5654:     	popq	%r12
1422a5656:     	popq	%r13
1422a5658:     	popq	%r14
1422a565a:     	popq	%r15
1422a565c:     	popq	%rbp
1422a565d:     	retq
1422a565e:     	nop
1422a5660:     	movq	%rdx, 0x10(%rsp)
1422a5665:     	pushq	%rbp
1422a5666:     	pushq	%r15
1422a5668:     	pushq	%r14
1422a566a:     	pushq	%r13
1422a566c:     	pushq	%r12
1422a566e:     	pushq	%rsi
1422a566f:     	pushq	%rdi
1422a5670:     	pushq	%rbx
1422a5671:     	subq	$0x108, %rsp            # imm = 0x108
1422a5678:     	leaq	0x80(%rdx), %rbp
1422a567f:     	movaps	%xmm15, 0x60(%rsp)
1422a5685:     	movaps	%xmm14, 0x70(%rsp)
1422a568b:     	movaps	%xmm13, 0x80(%rsp)
1422a5694:     	movaps	%xmm12, 0x90(%rsp)
1422a569d:     	movaps	%xmm11, 0xa0(%rsp)
1422a56a6:     	movaps	%xmm10, 0xb0(%rsp)
1422a56af:     	movaps	%xmm9, 0xc0(%rsp)
1422a56b8:     	movaps	%xmm8, 0xd0(%rsp)
1422a56c1:     	movaps	%xmm7, 0xe0(%rsp)
1422a56c9:     	movaps	%xmm6, 0xf0(%rsp)
1422a56d1:     	movl	$0xe, %edx
1422a56d6:     	movl	$0x1, %r8d
1422a56dc:     	movq	0xc8(%rbp), %rcx
1422a56e3:     	callq	0x140613c20 <.text+0x612c20>
1422a56e8:     	movaps	0xf0(%rsp), %xmm6
1422a56f0:     	movaps	0xe0(%rsp), %xmm7
1422a56f8:     	movaps	0xd0(%rsp), %xmm8
1422a5701:     	movaps	0xc0(%rsp), %xmm9
1422a570a:     	movaps	0xb0(%rsp), %xmm10
1422a5713:     	movaps	0xa0(%rsp), %xmm11
1422a571c:     	movaps	0x90(%rsp), %xmm12
1422a5725:     	movaps	0x80(%rsp), %xmm13
1422a572e:     	movaps	0x70(%rsp), %xmm14
1422a5734:     	movaps	0x60(%rsp), %xmm15
1422a573a:     	addq	$0x108, %rsp            # imm = 0x108
1422a5741:     	popq	%rbx
1422a5742:     	popq	%rdi
1422a5743:     	popq	%rsi
1422a5744:     	popq	%r12
1422a5746:     	popq	%r13
1422a5748:     	popq	%r14
1422a574a:     	popq	%r15
1422a574c:     	popq	%rbp
1422a574d:     	retq
1422a574e:     	nop
1422a5750:     	movq	%rdx, 0x10(%rsp)
1422a5755:     	pushq	%rbp
1422a5756:     	pushq	%r15
1422a5758:     	pushq	%r14
1422a575a:     	pushq	%r13
1422a575c:     	pushq	%r12
1422a575e:     	pushq	%rsi
1422a575f:     	pushq	%rdi
1422a5760:     	pushq	%rbx
1422a5761:     	subq	$0x108, %rsp            # imm = 0x108
1422a5768:     	leaq	0x80(%rdx), %rbp
1422a576f:     	movaps	%xmm15, 0x60(%rsp)
1422a5775:     	movaps	%xmm14, 0x70(%rsp)
1422a577b:     	movaps	%xmm13, 0x80(%rsp)
1422a5784:     	movaps	%xmm12, 0x90(%rsp)
1422a578d:     	movaps	%xmm11, 0xa0(%rsp)
1422a5796:     	movaps	%xmm10, 0xb0(%rsp)
1422a579f:     	movaps	%xmm9, 0xc0(%rsp)
1422a57a8:     	movaps	%xmm8, 0xd0(%rsp)
1422a57b1:     	movaps	%xmm7, 0xe0(%rsp)
1422a57b9:     	movaps	%xmm6, 0xf0(%rsp)
1422a57c1:     	movl	$0x16, %edx
1422a57c6:     	movl	$0x1, %r8d
1422a57cc:     	movq	0x1f8(%rbp), %rcx
1422a57d3:     	callq	0x140613c20 <.text+0x612c20>
1422a57d8:     	movaps	0xf0(%rsp), %xmm6
1422a57e0:     	movaps	0xe0(%rsp), %xmm7
1422a57e8:     	movaps	0xd0(%rsp), %xmm8
1422a57f1:     	movaps	0xc0(%rsp), %xmm9
1422a57fa:     	movaps	0xb0(%rsp), %xmm10
1422a5803:     	movaps	0xa0(%rsp), %xmm11
1422a580c:     	movaps	0x90(%rsp), %xmm12
1422a5815:     	movaps	0x80(%rsp), %xmm13
1422a581e:     	movaps	0x70(%rsp), %xmm14
1422a5824:     	movaps	0x60(%rsp), %xmm15
1422a582a:     	addq	$0x108, %rsp            # imm = 0x108
1422a5831:     	popq	%rbx
1422a5832:     	popq	%rdi
1422a5833:     	popq	%rsi
1422a5834:     	popq	%r12
1422a5836:     	popq	%r13
1422a5838:     	popq	%r14
1422a583a:     	popq	%r15
1422a583c:     	popq	%rbp
1422a583d:     	retq
1422a583e:     	nop
1422a5840:     	movq	%rdx, 0x10(%rsp)
1422a5845:     	pushq	%rbp
1422a5846:     	pushq	%r15
1422a5848:     	pushq	%r14
1422a584a:     	pushq	%r13
1422a584c:     	pushq	%r12
1422a584e:     	pushq	%rsi
1422a584f:     	pushq	%rdi
1422a5850:     	pushq	%rbx
1422a5851:     	subq	$0x108, %rsp            # imm = 0x108
1422a5858:     	leaq	0x80(%rdx), %rbp
1422a585f:     	movaps	%xmm15, 0x60(%rsp)
1422a5865:     	movaps	%xmm14, 0x70(%rsp)
1422a586b:     	movaps	%xmm13, 0x80(%rsp)
1422a5874:     	movaps	%xmm12, 0x90(%rsp)
1422a587d:     	movaps	%xmm11, 0xa0(%rsp)
1422a5886:     	movaps	%xmm10, 0xb0(%rsp)
1422a588f:     	movaps	%xmm9, 0xc0(%rsp)
1422a5898:     	movaps	%xmm8, 0xd0(%rsp)
1422a58a1:     	movaps	%xmm7, 0xe0(%rsp)
1422a58a9:     	movaps	%xmm6, 0xf0(%rsp)
1422a58b1:     	movl	$0x16, %edx
1422a58b6:     	movl	$0x1, %r8d
1422a58bc:     	movq	0x1f0(%rbp), %rcx
1422a58c3:     	callq	0x140613c20 <.text+0x612c20>
1422a58c8:     	movaps	0xf0(%rsp), %xmm6
1422a58d0:     	movaps	0xe0(%rsp), %xmm7
1422a58d8:     	movaps	0xd0(%rsp), %xmm8
1422a58e1:     	movaps	0xc0(%rsp), %xmm9
1422a58ea:     	movaps	0xb0(%rsp), %xmm10
1422a58f3:     	movaps	0xa0(%rsp), %xmm11
1422a58fc:     	movaps	0x90(%rsp), %xmm12
1422a5905:     	movaps	0x80(%rsp), %xmm13
1422a590e:     	movaps	0x70(%rsp), %xmm14
1422a5914:     	movaps	0x60(%rsp), %xmm15
1422a591a:     	addq	$0x108, %rsp            # imm = 0x108
1422a5921:     	popq	%rbx
1422a5922:     	popq	%rdi
1422a5923:     	popq	%rsi
1422a5924:     	popq	%r12
1422a5926:     	popq	%r13
1422a5928:     	popq	%r14
1422a592a:     	popq	%r15
1422a592c:     	popq	%rbp
1422a592d:     	retq
1422a592e:     	nop
1422a5930:     	movq	%rdx, 0x10(%rsp)
1422a5935:     	pushq	%rbp
1422a5936:     	pushq	%r15
1422a5938:     	pushq	%r14
1422a593a:     	pushq	%r13
1422a593c:     	pushq	%r12
1422a593e:     	pushq	%rsi
1422a593f:     	pushq	%rdi
1422a5940:     	pushq	%rbx
1422a5941:     	subq	$0x108, %rsp            # imm = 0x108
1422a5948:     	leaq	0x80(%rdx), %rbp
1422a594f:     	movaps	%xmm15, 0x60(%rsp)
1422a5955:     	movaps	%xmm14, 0x70(%rsp)
1422a595b:     	movaps	%xmm13, 0x80(%rsp)
1422a5964:     	movaps	%xmm12, 0x90(%rsp)
1422a596d:     	movaps	%xmm11, 0xa0(%rsp)
1422a5976:     	movaps	%xmm10, 0xb0(%rsp)
1422a597f:     	movaps	%xmm9, 0xc0(%rsp)
1422a5988:     	movaps	%xmm8, 0xd0(%rsp)
1422a5991:     	movaps	%xmm7, 0xe0(%rsp)
1422a5999:     	movaps	%xmm6, 0xf0(%rsp)
1422a59a1:     	movl	$0x11, %edx
1422a59a6:     	movl	$0x1, %r8d
1422a59ac:     	movq	0x1e8(%rbp), %rcx
1422a59b3:     	callq	0x140613c20 <.text+0x612c20>
1422a59b8:     	movaps	0xf0(%rsp), %xmm6
1422a59c0:     	movaps	0xe0(%rsp), %xmm7
1422a59c8:     	movaps	0xd0(%rsp), %xmm8
1422a59d1:     	movaps	0xc0(%rsp), %xmm9
1422a59da:     	movaps	0xb0(%rsp), %xmm10
1422a59e3:     	movaps	0xa0(%rsp), %xmm11
1422a59ec:     	movaps	0x90(%rsp), %xmm12
1422a59f5:     	movaps	0x80(%rsp), %xmm13
1422a59fe:     	movaps	0x70(%rsp), %xmm14
1422a5a04:     	movaps	0x60(%rsp), %xmm15
1422a5a0a:     	addq	$0x108, %rsp            # imm = 0x108
1422a5a11:     	popq	%rbx
1422a5a12:     	popq	%rdi
1422a5a13:     	popq	%rsi
1422a5a14:     	popq	%r12
1422a5a16:     	popq	%r13
1422a5a18:     	popq	%r14
1422a5a1a:     	popq	%r15
1422a5a1c:     	popq	%rbp
1422a5a1d:     	retq
1422a5a1e:     	nop
1422a5a20:     	movq	%rdx, 0x10(%rsp)
1422a5a25:     	pushq	%rbp
1422a5a26:     	pushq	%r15
1422a5a28:     	pushq	%r14
1422a5a2a:     	pushq	%r13
1422a5a2c:     	pushq	%r12
1422a5a2e:     	pushq	%rsi
1422a5a2f:     	pushq	%rdi
1422a5a30:     	pushq	%rbx
1422a5a31:     	subq	$0x108, %rsp            # imm = 0x108
1422a5a38:     	leaq	0x80(%rdx), %rbp
1422a5a3f:     	movaps	%xmm15, 0x60(%rsp)
1422a5a45:     	movaps	%xmm14, 0x70(%rsp)
1422a5a4b:     	movaps	%xmm13, 0x80(%rsp)
1422a5a54:     	movaps	%xmm12, 0x90(%rsp)
1422a5a5d:     	movaps	%xmm11, 0xa0(%rsp)
1422a5a66:     	movaps	%xmm10, 0xb0(%rsp)
1422a5a6f:     	movaps	%xmm9, 0xc0(%rsp)
1422a5a78:     	movaps	%xmm8, 0xd0(%rsp)
1422a5a81:     	movaps	%xmm7, 0xe0(%rsp)
1422a5a89:     	movaps	%xmm6, 0xf0(%rsp)
1422a5a91:     	movl	$0x11, %edx
1422a5a96:     	movl	$0x1, %r8d
1422a5a9c:     	movq	0xc8(%rbp), %rcx
1422a5aa3:     	callq	0x140613c20 <.text+0x612c20>
1422a5aa8:     	movaps	0xf0(%rsp), %xmm6
1422a5ab0:     	movaps	0xe0(%rsp), %xmm7
1422a5ab8:     	movaps	0xd0(%rsp), %xmm8
1422a5ac1:     	movaps	0xc0(%rsp), %xmm9
1422a5aca:     	movaps	0xb0(%rsp), %xmm10
1422a5ad3:     	movaps	0xa0(%rsp), %xmm11
1422a5adc:     	movaps	0x90(%rsp), %xmm12
1422a5ae5:     	movaps	0x80(%rsp), %xmm13
1422a5aee:     	movaps	0x70(%rsp), %xmm14
1422a5af4:     	movaps	0x60(%rsp), %xmm15
1422a5afa:     	addq	$0x108, %rsp            # imm = 0x108
1422a5b01:     	popq	%rbx
1422a5b02:     	popq	%rdi
1422a5b03:     	popq	%rsi
1422a5b04:     	popq	%r12
1422a5b06:     	popq	%r13
1422a5b08:     	popq	%r14
1422a5b0a:     	popq	%r15
1422a5b0c:     	popq	%rbp
1422a5b0d:     	retq
1422a5b0e:     	nop
1422a5b10:     	movq	%rdx, 0x10(%rsp)
1422a5b15:     	pushq	%rbp
1422a5b16:     	pushq	%r15
1422a5b18:     	pushq	%r14
1422a5b1a:     	pushq	%r13
1422a5b1c:     	pushq	%r12
1422a5b1e:     	pushq	%rsi
1422a5b1f:     	pushq	%rdi
1422a5b20:     	pushq	%rbx
1422a5b21:     	subq	$0x108, %rsp            # imm = 0x108
1422a5b28:     	leaq	0x80(%rdx), %rbp
1422a5b2f:     	movaps	%xmm15, 0x60(%rsp)
1422a5b35:     	movaps	%xmm14, 0x70(%rsp)
1422a5b3b:     	movaps	%xmm13, 0x80(%rsp)
1422a5b44:     	movaps	%xmm12, 0x90(%rsp)
1422a5b4d:     	movaps	%xmm11, 0xa0(%rsp)
1422a5b56:     	movaps	%xmm10, 0xb0(%rsp)
1422a5b5f:     	movaps	%xmm9, 0xc0(%rsp)
1422a5b68:     	movaps	%xmm8, 0xd0(%rsp)
1422a5b71:     	movaps	%xmm7, 0xe0(%rsp)
1422a5b79:     	movaps	%xmm6, 0xf0(%rsp)
1422a5b81:     	movl	$0x19, %edx
1422a5b86:     	movl	$0x1, %r8d
1422a5b8c:     	movq	0x1c0(%rbp), %rcx
1422a5b93:     	callq	0x140613c20 <.text+0x612c20>
1422a5b98:     	movaps	0xf0(%rsp), %xmm6
1422a5ba0:     	movaps	0xe0(%rsp), %xmm7
1422a5ba8:     	movaps	0xd0(%rsp), %xmm8
1422a5bb1:     	movaps	0xc0(%rsp), %xmm9
1422a5bba:     	movaps	0xb0(%rsp), %xmm10
1422a5bc3:     	movaps	0xa0(%rsp), %xmm11
1422a5bcc:     	movaps	0x90(%rsp), %xmm12
1422a5bd5:     	movaps	0x80(%rsp), %xmm13
1422a5bde:     	movaps	0x70(%rsp), %xmm14
1422a5be4:     	movaps	0x60(%rsp), %xmm15
1422a5bea:     	addq	$0x108, %rsp            # imm = 0x108
1422a5bf1:     	popq	%rbx
1422a5bf2:     	popq	%rdi
1422a5bf3:     	popq	%rsi
1422a5bf4:     	popq	%r12
1422a5bf6:     	popq	%r13
1422a5bf8:     	popq	%r14
1422a5bfa:     	popq	%r15
1422a5bfc:     	popq	%rbp
1422a5bfd:     	retq
1422a5bfe:     	nop
1422a5c00:     	movq	%rdx, 0x10(%rsp)
1422a5c05:     	pushq	%rbp
1422a5c06:     	pushq	%r15
1422a5c08:     	pushq	%r14
1422a5c0a:     	pushq	%r13
1422a5c0c:     	pushq	%r12
1422a5c0e:     	pushq	%rsi
1422a5c0f:     	pushq	%rdi
1422a5c10:     	pushq	%rbx
1422a5c11:     	subq	$0x108, %rsp            # imm = 0x108
1422a5c18:     	leaq	0x80(%rdx), %rbp
1422a5c1f:     	movaps	%xmm15, 0x60(%rsp)
1422a5c25:     	movaps	%xmm14, 0x70(%rsp)
1422a5c2b:     	movaps	%xmm13, 0x80(%rsp)
1422a5c34:     	movaps	%xmm12, 0x90(%rsp)
1422a5c3d:     	movaps	%xmm11, 0xa0(%rsp)
1422a5c46:     	movaps	%xmm10, 0xb0(%rsp)
1422a5c4f:     	movaps	%xmm9, 0xc0(%rsp)
1422a5c58:     	movaps	%xmm8, 0xd0(%rsp)
1422a5c61:     	movaps	%xmm7, 0xe0(%rsp)
1422a5c69:     	movaps	%xmm6, 0xf0(%rsp)
1422a5c71:     	movl	$0x13, %edx
1422a5c76:     	movl	$0x1, %r8d
1422a5c7c:     	movq	0x1e0(%rbp), %rcx
1422a5c83:     	callq	0x140613c20 <.text+0x612c20>
1422a5c88:     	movaps	0xf0(%rsp), %xmm6
1422a5c90:     	movaps	0xe0(%rsp), %xmm7
1422a5c98:     	movaps	0xd0(%rsp), %xmm8
1422a5ca1:     	movaps	0xc0(%rsp), %xmm9
1422a5caa:     	movaps	0xb0(%rsp), %xmm10
1422a5cb3:     	movaps	0xa0(%rsp), %xmm11
1422a5cbc:     	movaps	0x90(%rsp), %xmm12
1422a5cc5:     	movaps	0x80(%rsp), %xmm13
1422a5cce:     	movaps	0x70(%rsp), %xmm14
1422a5cd4:     	movaps	0x60(%rsp), %xmm15
1422a5cda:     	addq	$0x108, %rsp            # imm = 0x108
1422a5ce1:     	popq	%rbx
1422a5ce2:     	popq	%rdi
1422a5ce3:     	popq	%rsi
1422a5ce4:     	popq	%r12
1422a5ce6:     	popq	%r13
1422a5ce8:     	popq	%r14
1422a5cea:     	popq	%r15
1422a5cec:     	popq	%rbp
1422a5ced:     	retq
1422a5cee:     	nop
1422a5cf0:     	movq	%rdx, 0x10(%rsp)
1422a5cf5:     	pushq	%rbp
1422a5cf6:     	pushq	%r15
1422a5cf8:     	pushq	%r14
1422a5cfa:     	pushq	%r13
1422a5cfc:     	pushq	%r12
1422a5cfe:     	pushq	%rsi
1422a5cff:     	pushq	%rdi
1422a5d00:     	pushq	%rbx
1422a5d01:     	subq	$0x108, %rsp            # imm = 0x108
1422a5d08:     	leaq	0x80(%rdx), %rbp
1422a5d0f:     	movaps	%xmm15, 0x60(%rsp)
1422a5d15:     	movaps	%xmm14, 0x70(%rsp)
1422a5d1b:     	movaps	%xmm13, 0x80(%rsp)
1422a5d24:     	movaps	%xmm12, 0x90(%rsp)
1422a5d2d:     	movaps	%xmm11, 0xa0(%rsp)
1422a5d36:     	movaps	%xmm10, 0xb0(%rsp)
1422a5d3f:     	movaps	%xmm9, 0xc0(%rsp)
1422a5d48:     	movaps	%xmm8, 0xd0(%rsp)
1422a5d51:     	movaps	%xmm7, 0xe0(%rsp)
1422a5d59:     	movaps	%xmm6, 0xf0(%rsp)
1422a5d61:     	movl	$0x1d, %edx
1422a5d66:     	movl	$0x1, %r8d
1422a5d6c:     	movq	0x1c8(%rbp), %rcx
1422a5d73:     	callq	0x140613c20 <.text+0x612c20>
1422a5d78:     	movaps	0xf0(%rsp), %xmm6
1422a5d80:     	movaps	0xe0(%rsp), %xmm7
1422a5d88:     	movaps	0xd0(%rsp), %xmm8
1422a5d91:     	movaps	0xc0(%rsp), %xmm9
1422a5d9a:     	movaps	0xb0(%rsp), %xmm10
1422a5da3:     	movaps	0xa0(%rsp), %xmm11
1422a5dac:     	movaps	0x90(%rsp), %xmm12
1422a5db5:     	movaps	0x80(%rsp), %xmm13
1422a5dbe:     	movaps	0x70(%rsp), %xmm14
1422a5dc4:     	movaps	0x60(%rsp), %xmm15
1422a5dca:     	addq	$0x108, %rsp            # imm = 0x108
1422a5dd1:     	popq	%rbx
1422a5dd2:     	popq	%rdi
1422a5dd3:     	popq	%rsi
1422a5dd4:     	popq	%r12
1422a5dd6:     	popq	%r13
1422a5dd8:     	popq	%r14
1422a5dda:     	popq	%r15
1422a5ddc:     	popq	%rbp
1422a5ddd:     	retq
1422a5dde:     	nop
1422a5de0:     	movq	%rdx, 0x10(%rsp)
1422a5de5:     	pushq	%rbp
1422a5de6:     	pushq	%r15
1422a5de8:     	pushq	%r14
1422a5dea:     	pushq	%r13
1422a5dec:     	pushq	%r12
1422a5dee:     	pushq	%rsi
1422a5def:     	pushq	%rdi
1422a5df0:     	pushq	%rbx
1422a5df1:     	subq	$0x108, %rsp            # imm = 0x108
1422a5df8:     	leaq	0x80(%rdx), %rbp
1422a5dff:     	movaps	%xmm15, 0x60(%rsp)
1422a5e05:     	movaps	%xmm14, 0x70(%rsp)
1422a5e0b:     	movaps	%xmm13, 0x80(%rsp)
1422a5e14:     	movaps	%xmm12, 0x90(%rsp)
1422a5e1d:     	movaps	%xmm11, 0xa0(%rsp)
1422a5e26:     	movaps	%xmm10, 0xb0(%rsp)
1422a5e2f:     	movaps	%xmm9, 0xc0(%rsp)
1422a5e38:     	movaps	%xmm8, 0xd0(%rsp)
1422a5e41:     	movaps	%xmm7, 0xe0(%rsp)
1422a5e49:     	movaps	%xmm6, 0xf0(%rsp)
1422a5e51:     	movl	$0x12, %edx
1422a5e56:     	movl	$0x1, %r8d
1422a5e5c:     	movq	0x1b0(%rbp), %rcx
1422a5e63:     	callq	0x140613c20 <.text+0x612c20>
1422a5e68:     	movaps	0xf0(%rsp), %xmm6
1422a5e70:     	movaps	0xe0(%rsp), %xmm7
1422a5e78:     	movaps	0xd0(%rsp), %xmm8
1422a5e81:     	movaps	0xc0(%rsp), %xmm9
1422a5e8a:     	movaps	0xb0(%rsp), %xmm10
1422a5e93:     	movaps	0xa0(%rsp), %xmm11
1422a5e9c:     	movaps	0x90(%rsp), %xmm12
1422a5ea5:     	movaps	0x80(%rsp), %xmm13
1422a5eae:     	movaps	0x70(%rsp), %xmm14
1422a5eb4:     	movaps	0x60(%rsp), %xmm15
1422a5eba:     	addq	$0x108, %rsp            # imm = 0x108
1422a5ec1:     	popq	%rbx
1422a5ec2:     	popq	%rdi
1422a5ec3:     	popq	%rsi
1422a5ec4:     	popq	%r12
1422a5ec6:     	popq	%r13
1422a5ec8:     	popq	%r14
1422a5eca:     	popq	%r15
1422a5ecc:     	popq	%rbp
1422a5ecd:     	retq
1422a5ece:     	nop
1422a5ed0:     	movq	%rdx, 0x10(%rsp)
1422a5ed5:     	pushq	%rbp
1422a5ed6:     	pushq	%r15
1422a5ed8:     	pushq	%r14
1422a5eda:     	pushq	%r13
1422a5edc:     	pushq	%r12
1422a5ede:     	pushq	%rsi
1422a5edf:     	pushq	%rdi
1422a5ee0:     	pushq	%rbx
1422a5ee1:     	subq	$0x108, %rsp            # imm = 0x108
1422a5ee8:     	leaq	0x80(%rdx), %rbp
1422a5eef:     	movaps	%xmm15, 0x60(%rsp)
1422a5ef5:     	movaps	%xmm14, 0x70(%rsp)
1422a5efb:     	movaps	%xmm13, 0x80(%rsp)
1422a5f04:     	movaps	%xmm12, 0x90(%rsp)
1422a5f0d:     	movaps	%xmm11, 0xa0(%rsp)
1422a5f16:     	movaps	%xmm10, 0xb0(%rsp)
1422a5f1f:     	movaps	%xmm9, 0xc0(%rsp)
1422a5f28:     	movaps	%xmm8, 0xd0(%rsp)
1422a5f31:     	movaps	%xmm7, 0xe0(%rsp)
1422a5f39:     	movaps	%xmm6, 0xf0(%rsp)
1422a5f41:     	movl	$0x14, %edx
1422a5f46:     	movl	$0x1, %r8d
1422a5f4c:     	movq	0x178(%rbp), %rcx
1422a5f53:     	callq	0x140613c20 <.text+0x612c20>
1422a5f58:     	movaps	0xf0(%rsp), %xmm6
1422a5f60:     	movaps	0xe0(%rsp), %xmm7
1422a5f68:     	movaps	0xd0(%rsp), %xmm8
1422a5f71:     	movaps	0xc0(%rsp), %xmm9
1422a5f7a:     	movaps	0xb0(%rsp), %xmm10
1422a5f83:     	movaps	0xa0(%rsp), %xmm11
1422a5f8c:     	movaps	0x90(%rsp), %xmm12
1422a5f95:     	movaps	0x80(%rsp), %xmm13
1422a5f9e:     	movaps	0x70(%rsp), %xmm14
1422a5fa4:     	movaps	0x60(%rsp), %xmm15
1422a5faa:     	addq	$0x108, %rsp            # imm = 0x108
1422a5fb1:     	popq	%rbx
1422a5fb2:     	popq	%rdi
1422a5fb3:     	popq	%rsi
1422a5fb4:     	popq	%r12
1422a5fb6:     	popq	%r13
1422a5fb8:     	popq	%r14
1422a5fba:     	popq	%r15
1422a5fbc:     	popq	%rbp
1422a5fbd:     	retq
1422a5fbe:     	nop
1422a5fc0:     	movq	%rdx, 0x10(%rsp)
1422a5fc5:     	pushq	%rbp
1422a5fc6:     	pushq	%r15
1422a5fc8:     	pushq	%r14
1422a5fca:     	pushq	%r13
1422a5fcc:     	pushq	%r12
1422a5fce:     	pushq	%rsi
1422a5fcf:     	pushq	%rdi
1422a5fd0:     	pushq	%rbx
1422a5fd1:     	subq	$0x108, %rsp            # imm = 0x108
1422a5fd8:     	leaq	0x80(%rdx), %rbp
1422a5fdf:     	movaps	%xmm15, 0x60(%rsp)
1422a5fe5:     	movaps	%xmm14, 0x70(%rsp)
1422a5feb:     	movaps	%xmm13, 0x80(%rsp)
1422a5ff4:     	movaps	%xmm12, 0x90(%rsp)
1422a5ffd:     	movaps	%xmm11, 0xa0(%rsp)
1422a6006:     	movaps	%xmm10, 0xb0(%rsp)
1422a600f:     	movaps	%xmm9, 0xc0(%rsp)
1422a6018:     	movaps	%xmm8, 0xd0(%rsp)
1422a6021:     	movaps	%xmm7, 0xe0(%rsp)
1422a6029:     	movaps	%xmm6, 0xf0(%rsp)
1422a6031:     	movl	$0x15, %edx
1422a6036:     	movl	$0x1, %r8d
1422a603c:     	movq	0x1f8(%rbp), %rcx
1422a6043:     	callq	0x140613c20 <.text+0x612c20>
1422a6048:     	movaps	0xf0(%rsp), %xmm6
1422a6050:     	movaps	0xe0(%rsp), %xmm7
1422a6058:     	movaps	0xd0(%rsp), %xmm8
1422a6061:     	movaps	0xc0(%rsp), %xmm9
1422a606a:     	movaps	0xb0(%rsp), %xmm10
1422a6073:     	movaps	0xa0(%rsp), %xmm11
1422a607c:     	movaps	0x90(%rsp), %xmm12
1422a6085:     	movaps	0x80(%rsp), %xmm13
1422a608e:     	movaps	0x70(%rsp), %xmm14
1422a6094:     	movaps	0x60(%rsp), %xmm15
1422a609a:     	addq	$0x108, %rsp            # imm = 0x108
1422a60a1:     	popq	%rbx
1422a60a2:     	popq	%rdi
1422a60a3:     	popq	%rsi
1422a60a4:     	popq	%r12
1422a60a6:     	popq	%r13
1422a60a8:     	popq	%r14
1422a60aa:     	popq	%r15
1422a60ac:     	popq	%rbp
1422a60ad:     	retq
1422a60ae:     	nop
1422a60b0:     	movq	%rdx, 0x10(%rsp)
1422a60b5:     	pushq	%rbp
1422a60b6:     	pushq	%r15
1422a60b8:     	pushq	%r14
1422a60ba:     	pushq	%r13
1422a60bc:     	pushq	%r12
1422a60be:     	pushq	%rsi
1422a60bf:     	pushq	%rdi
1422a60c0:     	pushq	%rbx
1422a60c1:     	subq	$0x108, %rsp            # imm = 0x108
1422a60c8:     	leaq	0x80(%rdx), %rbp
1422a60cf:     	movaps	%xmm15, 0x60(%rsp)
1422a60d5:     	movaps	%xmm14, 0x70(%rsp)
1422a60db:     	movaps	%xmm13, 0x80(%rsp)
1422a60e4:     	movaps	%xmm12, 0x90(%rsp)
1422a60ed:     	movaps	%xmm11, 0xa0(%rsp)
1422a60f6:     	movaps	%xmm10, 0xb0(%rsp)
1422a60ff:     	movaps	%xmm9, 0xc0(%rsp)
1422a6108:     	movaps	%xmm8, 0xd0(%rsp)
1422a6111:     	movaps	%xmm7, 0xe0(%rsp)
1422a6119:     	movaps	%xmm6, 0xf0(%rsp)
1422a6121:     	movl	$0x1b, %edx
1422a6126:     	movl	$0x1, %r8d
1422a612c:     	movq	0x1e0(%rbp), %rcx
1422a6133:     	callq	0x140613c20 <.text+0x612c20>
1422a6138:     	movaps	0xf0(%rsp), %xmm6
1422a6140:     	movaps	0xe0(%rsp), %xmm7
1422a6148:     	movaps	0xd0(%rsp), %xmm8
1422a6151:     	movaps	0xc0(%rsp), %xmm9
1422a615a:     	movaps	0xb0(%rsp), %xmm10
1422a6163:     	movaps	0xa0(%rsp), %xmm11
1422a616c:     	movaps	0x90(%rsp), %xmm12
1422a6175:     	movaps	0x80(%rsp), %xmm13
1422a617e:     	movaps	0x70(%rsp), %xmm14
1422a6184:     	movaps	0x60(%rsp), %xmm15
1422a618a:     	addq	$0x108, %rsp            # imm = 0x108
1422a6191:     	popq	%rbx
1422a6192:     	popq	%rdi
1422a6193:     	popq	%rsi
1422a6194:     	popq	%r12
1422a6196:     	popq	%r13
1422a6198:     	popq	%r14
1422a619a:     	popq	%r15
1422a619c:     	popq	%rbp
1422a619d:     	retq
1422a619e:     	nop
1422a61a0:     	movq	%rdx, 0x10(%rsp)
1422a61a5:     	pushq	%rbp
1422a61a6:     	pushq	%r15
1422a61a8:     	pushq	%r14
1422a61aa:     	pushq	%r13
1422a61ac:     	pushq	%r12
1422a61ae:     	pushq	%rsi
1422a61af:     	pushq	%rdi
1422a61b0:     	pushq	%rbx
1422a61b1:     	subq	$0x108, %rsp            # imm = 0x108
1422a61b8:     	leaq	0x80(%rdx), %rbp
1422a61bf:     	movaps	%xmm15, 0x60(%rsp)
1422a61c5:     	movaps	%xmm14, 0x70(%rsp)
1422a61cb:     	movaps	%xmm13, 0x80(%rsp)
1422a61d4:     	movaps	%xmm12, 0x90(%rsp)
1422a61dd:     	movaps	%xmm11, 0xa0(%rsp)
1422a61e6:     	movaps	%xmm10, 0xb0(%rsp)
1422a61ef:     	movaps	%xmm9, 0xc0(%rsp)
1422a61f8:     	movaps	%xmm8, 0xd0(%rsp)
1422a6201:     	movaps	%xmm7, 0xe0(%rsp)
1422a6209:     	movaps	%xmm6, 0xf0(%rsp)
1422a6211:     	movl	$0x17, %edx
1422a6216:     	movl	$0x1, %r8d
1422a621c:     	movq	0x1f8(%rbp), %rcx
1422a6223:     	callq	0x140613c20 <.text+0x612c20>
1422a6228:     	movaps	0xf0(%rsp), %xmm6
1422a6230:     	movaps	0xe0(%rsp), %xmm7
1422a6238:     	movaps	0xd0(%rsp), %xmm8
1422a6241:     	movaps	0xc0(%rsp), %xmm9
1422a624a:     	movaps	0xb0(%rsp), %xmm10
1422a6253:     	movaps	0xa0(%rsp), %xmm11
1422a625c:     	movaps	0x90(%rsp), %xmm12
1422a6265:     	movaps	0x80(%rsp), %xmm13
1422a626e:     	movaps	0x70(%rsp), %xmm14
1422a6274:     	movaps	0x60(%rsp), %xmm15
1422a627a:     	addq	$0x108, %rsp            # imm = 0x108
1422a6281:     	popq	%rbx
1422a6282:     	popq	%rdi
1422a6283:     	popq	%rsi
1422a6284:     	popq	%r12
1422a6286:     	popq	%r13
1422a6288:     	popq	%r14
1422a628a:     	popq	%r15
1422a628c:     	popq	%rbp
1422a628d:     	retq
1422a628e:     	nop
1422a6290:     	movq	%rdx, 0x10(%rsp)
1422a6295:     	pushq	%rbp
1422a6296:     	pushq	%r15
1422a6298:     	pushq	%r14
1422a629a:     	pushq	%r13
1422a629c:     	pushq	%r12
1422a629e:     	pushq	%rsi
1422a629f:     	pushq	%rdi
1422a62a0:     	pushq	%rbx
1422a62a1:     	subq	$0x108, %rsp            # imm = 0x108
1422a62a8:     	leaq	0x80(%rdx), %rbp
1422a62af:     	movaps	%xmm15, 0x60(%rsp)
1422a62b5:     	movaps	%xmm14, 0x70(%rsp)
1422a62bb:     	movaps	%xmm13, 0x80(%rsp)
1422a62c4:     	movaps	%xmm12, 0x90(%rsp)
1422a62cd:     	movaps	%xmm11, 0xa0(%rsp)
1422a62d6:     	movaps	%xmm10, 0xb0(%rsp)
1422a62df:     	movaps	%xmm9, 0xc0(%rsp)
1422a62e8:     	movaps	%xmm8, 0xd0(%rsp)
1422a62f1:     	movaps	%xmm7, 0xe0(%rsp)
1422a62f9:     	movaps	%xmm6, 0xf0(%rsp)
1422a6301:     	movl	$0x5, %edx
1422a6306:     	movl	$0x1, %r8d
1422a630c:     	movq	0x1e8(%rbp), %rcx
1422a6313:     	callq	0x140613c20 <.text+0x612c20>
1422a6318:     	movaps	0xf0(%rsp), %xmm6
1422a6320:     	movaps	0xe0(%rsp), %xmm7
1422a6328:     	movaps	0xd0(%rsp), %xmm8
1422a6331:     	movaps	0xc0(%rsp), %xmm9
1422a633a:     	movaps	0xb0(%rsp), %xmm10
1422a6343:     	movaps	0xa0(%rsp), %xmm11
1422a634c:     	movaps	0x90(%rsp), %xmm12
1422a6355:     	movaps	0x80(%rsp), %xmm13
1422a635e:     	movaps	0x70(%rsp), %xmm14
1422a6364:     	movaps	0x60(%rsp), %xmm15
1422a636a:     	addq	$0x108, %rsp            # imm = 0x108
1422a6371:     	popq	%rbx
1422a6372:     	popq	%rdi
1422a6373:     	popq	%rsi
1422a6374:     	popq	%r12
1422a6376:     	popq	%r13
1422a6378:     	popq	%r14
1422a637a:     	popq	%r15
1422a637c:     	popq	%rbp
1422a637d:     	retq
1422a637e:     	nop
1422a6380:     	movq	%rdx, 0x10(%rsp)
1422a6385:     	pushq	%rbp
1422a6386:     	pushq	%r15
1422a6388:     	pushq	%r14
1422a638a:     	pushq	%r13
1422a638c:     	pushq	%r12
1422a638e:     	pushq	%rsi
1422a638f:     	pushq	%rdi
1422a6390:     	pushq	%rbx
1422a6391:     	subq	$0x108, %rsp            # imm = 0x108
1422a6398:     	leaq	0x80(%rdx), %rbp
1422a639f:     	movaps	%xmm15, 0x60(%rsp)
1422a63a5:     	movaps	%xmm14, 0x70(%rsp)
1422a63ab:     	movaps	%xmm13, 0x80(%rsp)
1422a63b4:     	movaps	%xmm12, 0x90(%rsp)
1422a63bd:     	movaps	%xmm11, 0xa0(%rsp)
1422a63c6:     	movaps	%xmm10, 0xb0(%rsp)
1422a63cf:     	movaps	%xmm9, 0xc0(%rsp)
1422a63d8:     	movaps	%xmm8, 0xd0(%rsp)
1422a63e1:     	movaps	%xmm7, 0xe0(%rsp)
1422a63e9:     	movaps	%xmm6, 0xf0(%rsp)
1422a63f1:     	movl	$0x5, %edx
1422a63f6:     	movl	$0x1, %r8d
1422a63fc:     	movq	0x1e8(%rbp), %rcx
1422a6403:     	callq	0x140613c20 <.text+0x612c20>
1422a6408:     	movaps	0xf0(%rsp), %xmm6
1422a6410:     	movaps	0xe0(%rsp), %xmm7
1422a6418:     	movaps	0xd0(%rsp), %xmm8
1422a6421:     	movaps	0xc0(%rsp), %xmm9
1422a642a:     	movaps	0xb0(%rsp), %xmm10
1422a6433:     	movaps	0xa0(%rsp), %xmm11
1422a643c:     	movaps	0x90(%rsp), %xmm12
1422a6445:     	movaps	0x80(%rsp), %xmm13
1422a644e:     	movaps	0x70(%rsp), %xmm14
1422a6454:     	movaps	0x60(%rsp), %xmm15
1422a645a:     	addq	$0x108, %rsp            # imm = 0x108
1422a6461:     	popq	%rbx
1422a6462:     	popq	%rdi
1422a6463:     	popq	%rsi
1422a6464:     	popq	%r12
1422a6466:     	popq	%r13
1422a6468:     	popq	%r14
1422a646a:     	popq	%r15
1422a646c:     	popq	%rbp
1422a646d:     	retq
1422a646e:     	nop
1422a6470:     	movq	%rdx, 0x10(%rsp)
1422a6475:     	pushq	%rbp
1422a6476:     	pushq	%r15
1422a6478:     	pushq	%r14
1422a647a:     	pushq	%r13
1422a647c:     	pushq	%r12
1422a647e:     	pushq	%rsi
1422a647f:     	pushq	%rdi
1422a6480:     	pushq	%rbx
1422a6481:     	subq	$0x108, %rsp            # imm = 0x108
1422a6488:     	leaq	0x80(%rdx), %rbp
1422a648f:     	movaps	%xmm15, 0x60(%rsp)
1422a6495:     	movaps	%xmm14, 0x70(%rsp)
1422a649b:     	movaps	%xmm13, 0x80(%rsp)
1422a64a4:     	movaps	%xmm12, 0x90(%rsp)
1422a64ad:     	movaps	%xmm11, 0xa0(%rsp)
1422a64b6:     	movaps	%xmm10, 0xb0(%rsp)
1422a64bf:     	movaps	%xmm9, 0xc0(%rsp)
1422a64c8:     	movaps	%xmm8, 0xd0(%rsp)
1422a64d1:     	movaps	%xmm7, 0xe0(%rsp)
1422a64d9:     	movaps	%xmm6, 0xf0(%rsp)
1422a64e1:     	leaq	0x180(%rbp), %rcx
1422a64e8:     	callq	0x14014a860 <.text+0x149860>
1422a64ed:     	movaps	0xf0(%rsp), %xmm6
1422a64f5:     	movaps	0xe0(%rsp), %xmm7
1422a64fd:     	movaps	0xd0(%rsp), %xmm8
1422a6506:     	movaps	0xc0(%rsp), %xmm9
1422a650f:     	movaps	0xb0(%rsp), %xmm10
1422a6518:     	movaps	0xa0(%rsp), %xmm11
1422a6521:     	movaps	0x90(%rsp), %xmm12
1422a652a:     	movaps	0x80(%rsp), %xmm13
1422a6533:     	movaps	0x70(%rsp), %xmm14
1422a6539:     	movaps	0x60(%rsp), %xmm15
1422a653f:     	addq	$0x108, %rsp            # imm = 0x108
1422a6546:     	popq	%rbx
1422a6547:     	popq	%rdi
1422a6548:     	popq	%rsi
1422a6549:     	popq	%r12
1422a654b:     	popq	%r13
1422a654d:     	popq	%r14
1422a654f:     	popq	%r15
1422a6551:     	popq	%rbp
1422a6552:     	retq
1422a6553:     	nopw	%cs:(%rax,%rax)
1422a6560:     	movq	%rdx, 0x10(%rsp)
1422a6565:     	pushq	%rbp
1422a6566:     	pushq	%r15
1422a6568:     	pushq	%r14
1422a656a:     	pushq	%r13
1422a656c:     	pushq	%r12
1422a656e:     	pushq	%rsi
1422a656f:     	pushq	%rdi
1422a6570:     	pushq	%rbx
1422a6571:     	subq	$0x108, %rsp            # imm = 0x108
1422a6578:     	leaq	0x80(%rdx), %rbp
1422a657f:     	movaps	%xmm15, 0x60(%rsp)
1422a6585:     	movaps	%xmm14, 0x70(%rsp)
1422a658b:     	movaps	%xmm13, 0x80(%rsp)
1422a6594:     	movaps	%xmm12, 0x90(%rsp)
1422a659d:     	movaps	%xmm11, 0xa0(%rsp)
1422a65a6:     	movaps	%xmm10, 0xb0(%rsp)
1422a65af:     	movaps	%xmm9, 0xc0(%rsp)
1422a65b8:     	movaps	%xmm8, 0xd0(%rsp)
1422a65c1:     	movaps	%xmm7, 0xe0(%rsp)
1422a65c9:     	movaps	%xmm6, 0xf0(%rsp)
1422a65d1:     	movl	$0xc8, %edx
1422a65d6:     	movl	$0x8, %r8d
1422a65dc:     	movq	0x1f0(%rbp), %rcx
1422a65e3:     	callq	0x140613c20 <.text+0x612c20>
1422a65e8:     	movaps	0xf0(%rsp), %xmm6
1422a65f0:     	movaps	0xe0(%rsp), %xmm7
1422a65f8:     	movaps	0xd0(%rsp), %xmm8
1422a6601:     	movaps	0xc0(%rsp), %xmm9
1422a660a:     	movaps	0xb0(%rsp), %xmm10
1422a6613:     	movaps	0xa0(%rsp), %xmm11
1422a661c:     	movaps	0x90(%rsp), %xmm12
1422a6625:     	movaps	0x80(%rsp), %xmm13
1422a662e:     	movaps	0x70(%rsp), %xmm14
1422a6634:     	movaps	0x60(%rsp), %xmm15
1422a663a:     	addq	$0x108, %rsp            # imm = 0x108
1422a6641:     	popq	%rbx
1422a6642:     	popq	%rdi
1422a6643:     	popq	%rsi
1422a6644:     	popq	%r12
1422a6646:     	popq	%r13
1422a6648:     	popq	%r14
1422a664a:     	popq	%r15
1422a664c:     	popq	%rbp
1422a664d:     	retq
1422a664e:     	nop
1422a6650:     	movq	%rdx, 0x10(%rsp)
1422a6655:     	pushq	%rbp
1422a6656:     	pushq	%r15
1422a6658:     	pushq	%r14
1422a665a:     	pushq	%r13
1422a665c:     	pushq	%r12
1422a665e:     	pushq	%rsi
1422a665f:     	pushq	%rdi
1422a6660:     	pushq	%rbx
1422a6661:     	subq	$0x108, %rsp            # imm = 0x108
1422a6668:     	leaq	0x80(%rdx), %rbp
1422a666f:     	movaps	%xmm15, 0x60(%rsp)
1422a6675:     	movaps	%xmm14, 0x70(%rsp)
1422a667b:     	movaps	%xmm13, 0x80(%rsp)
1422a6684:     	movaps	%xmm12, 0x90(%rsp)
1422a668d:     	movaps	%xmm11, 0xa0(%rsp)
1422a6696:     	movaps	%xmm10, 0xb0(%rsp)
1422a669f:     	movaps	%xmm9, 0xc0(%rsp)
1422a66a8:     	movaps	%xmm8, 0xd0(%rsp)
1422a66b1:     	movaps	%xmm7, 0xe0(%rsp)
1422a66b9:     	movaps	%xmm6, 0xf0(%rsp)
1422a66c1:     	movl	$0xe, %edx
1422a66c6:     	movl	$0x1, %r8d
1422a66cc:     	movq	0xd0(%rbp), %rcx
1422a66d3:     	callq	0x140613c20 <.text+0x612c20>
1422a66d8:     	movaps	0xf0(%rsp), %xmm6
1422a66e0:     	movaps	0xe0(%rsp), %xmm7
1422a66e8:     	movaps	0xd0(%rsp), %xmm8
1422a66f1:     	movaps	0xc0(%rsp), %xmm9
1422a66fa:     	movaps	0xb0(%rsp), %xmm10
1422a6703:     	movaps	0xa0(%rsp), %xmm11
1422a670c:     	movaps	0x90(%rsp), %xmm12
1422a6715:     	movaps	0x80(%rsp), %xmm13
1422a671e:     	movaps	0x70(%rsp), %xmm14
1422a6724:     	movaps	0x60(%rsp), %xmm15
1422a672a:     	addq	$0x108, %rsp            # imm = 0x108
1422a6731:     	popq	%rbx
1422a6732:     	popq	%rdi
1422a6733:     	popq	%rsi
1422a6734:     	popq	%r12
1422a6736:     	popq	%r13
1422a6738:     	popq	%r14
1422a673a:     	popq	%r15
1422a673c:     	popq	%rbp
1422a673d:     	retq
1422a673e:     	nop
1422a6740:     	movq	%rdx, 0x10(%rsp)
1422a6745:     	pushq	%rbp
1422a6746:     	pushq	%r15
1422a6748:     	pushq	%r14
1422a674a:     	pushq	%r13
1422a674c:     	pushq	%r12
1422a674e:     	pushq	%rsi
1422a674f:     	pushq	%rdi
1422a6750:     	pushq	%rbx
1422a6751:     	subq	$0x108, %rsp            # imm = 0x108
1422a6758:     	leaq	0x80(%rdx), %rbp
1422a675f:     	movaps	%xmm15, 0x60(%rsp)
1422a6765:     	movaps	%xmm14, 0x70(%rsp)
1422a676b:     	movaps	%xmm13, 0x80(%rsp)
1422a6774:     	movaps	%xmm12, 0x90(%rsp)
1422a677d:     	movaps	%xmm11, 0xa0(%rsp)
1422a6786:     	movaps	%xmm10, 0xb0(%rsp)
1422a678f:     	movaps	%xmm9, 0xc0(%rsp)
1422a6798:     	movaps	%xmm8, 0xd0(%rsp)
1422a67a1:     	movaps	%xmm7, 0xe0(%rsp)
1422a67a9:     	movaps	%xmm6, 0xf0(%rsp)
1422a67b1:     	movl	$0x16, %edx
1422a67b6:     	movl	$0x1, %r8d
1422a67bc:     	movq	0x1c0(%rbp), %rcx
1422a67c3:     	callq	0x140613c20 <.text+0x612c20>
1422a67c8:     	movaps	0xf0(%rsp), %xmm6
1422a67d0:     	movaps	0xe0(%rsp), %xmm7
1422a67d8:     	movaps	0xd0(%rsp), %xmm8
1422a67e1:     	movaps	0xc0(%rsp), %xmm9
1422a67ea:     	movaps	0xb0(%rsp), %xmm10
1422a67f3:     	movaps	0xa0(%rsp), %xmm11
1422a67fc:     	movaps	0x90(%rsp), %xmm12
1422a6805:     	movaps	0x80(%rsp), %xmm13
1422a680e:     	movaps	0x70(%rsp), %xmm14
1422a6814:     	movaps	0x60(%rsp), %xmm15
1422a681a:     	addq	$0x108, %rsp            # imm = 0x108
1422a6821:     	popq	%rbx
1422a6822:     	popq	%rdi
1422a6823:     	popq	%rsi
1422a6824:     	popq	%r12
1422a6826:     	popq	%r13
1422a6828:     	popq	%r14
1422a682a:     	popq	%r15
1422a682c:     	popq	%rbp
1422a682d:     	retq
1422a682e:     	nop
1422a6830:     	movq	%rdx, 0x10(%rsp)
1422a6835:     	pushq	%rbp
1422a6836:     	pushq	%r15
1422a6838:     	pushq	%r14
1422a683a:     	pushq	%r13
1422a683c:     	pushq	%r12
1422a683e:     	pushq	%rsi
1422a683f:     	pushq	%rdi
1422a6840:     	pushq	%rbx
1422a6841:     	subq	$0x108, %rsp            # imm = 0x108
1422a6848:     	leaq	0x80(%rdx), %rbp
1422a684f:     	movaps	%xmm15, 0x60(%rsp)
1422a6855:     	movaps	%xmm14, 0x70(%rsp)
1422a685b:     	movaps	%xmm13, 0x80(%rsp)
1422a6864:     	movaps	%xmm12, 0x90(%rsp)
1422a686d:     	movaps	%xmm11, 0xa0(%rsp)
1422a6876:     	movaps	%xmm10, 0xb0(%rsp)
1422a687f:     	movaps	%xmm9, 0xc0(%rsp)
1422a6888:     	movaps	%xmm8, 0xd0(%rsp)
1422a6891:     	movaps	%xmm7, 0xe0(%rsp)
1422a6899:     	movaps	%xmm6, 0xf0(%rsp)
1422a68a1:     	movl	$0x11, %edx
1422a68a6:     	movl	$0x1, %r8d
1422a68ac:     	movq	0x1f0(%rbp), %rcx
1422a68b3:     	callq	0x140613c20 <.text+0x612c20>
1422a68b8:     	movaps	0xf0(%rsp), %xmm6
1422a68c0:     	movaps	0xe0(%rsp), %xmm7
1422a68c8:     	movaps	0xd0(%rsp), %xmm8
1422a68d1:     	movaps	0xc0(%rsp), %xmm9
1422a68da:     	movaps	0xb0(%rsp), %xmm10
1422a68e3:     	movaps	0xa0(%rsp), %xmm11
1422a68ec:     	movaps	0x90(%rsp), %xmm12
1422a68f5:     	movaps	0x80(%rsp), %xmm13
1422a68fe:     	movaps	0x70(%rsp), %xmm14
1422a6904:     	movaps	0x60(%rsp), %xmm15
1422a690a:     	addq	$0x108, %rsp            # imm = 0x108
1422a6911:     	popq	%rbx
1422a6912:     	popq	%rdi
1422a6913:     	popq	%rsi
1422a6914:     	popq	%r12
1422a6916:     	popq	%r13
1422a6918:     	popq	%r14
1422a691a:     	popq	%r15
1422a691c:     	popq	%rbp
1422a691d:     	retq
1422a691e:     	nop
1422a6920:     	movq	%rdx, 0x10(%rsp)
1422a6925:     	pushq	%rbp
1422a6926:     	pushq	%r15
1422a6928:     	pushq	%r14
1422a692a:     	pushq	%r13
1422a692c:     	pushq	%r12
1422a692e:     	pushq	%rsi
1422a692f:     	pushq	%rdi
1422a6930:     	pushq	%rbx
1422a6931:     	subq	$0x108, %rsp            # imm = 0x108
1422a6938:     	leaq	0x80(%rdx), %rbp
1422a693f:     	movaps	%xmm15, 0x60(%rsp)
1422a6945:     	movaps	%xmm14, 0x70(%rsp)
1422a694b:     	movaps	%xmm13, 0x80(%rsp)
1422a6954:     	movaps	%xmm12, 0x90(%rsp)
1422a695d:     	movaps	%xmm11, 0xa0(%rsp)
1422a6966:     	movaps	%xmm10, 0xb0(%rsp)
1422a696f:     	movaps	%xmm9, 0xc0(%rsp)
1422a6978:     	movaps	%xmm8, 0xd0(%rsp)
1422a6981:     	movaps	%xmm7, 0xe0(%rsp)
1422a6989:     	movaps	%xmm6, 0xf0(%rsp)
1422a6991:     	movl	$0x5, %edx
1422a6996:     	movl	$0x1, %r8d
1422a699c:     	movq	0xd0(%rbp), %rcx
1422a69a3:     	callq	0x140613c20 <.text+0x612c20>
1422a69a8:     	movaps	0xf0(%rsp), %xmm6
1422a69b0:     	movaps	0xe0(%rsp), %xmm7
1422a69b8:     	movaps	0xd0(%rsp), %xmm8
1422a69c1:     	movaps	0xc0(%rsp), %xmm9
1422a69ca:     	movaps	0xb0(%rsp), %xmm10
1422a69d3:     	movaps	0xa0(%rsp), %xmm11
1422a69dc:     	movaps	0x90(%rsp), %xmm12
1422a69e5:     	movaps	0x80(%rsp), %xmm13
1422a69ee:     	movaps	0x70(%rsp), %xmm14
1422a69f4:     	movaps	0x60(%rsp), %xmm15
1422a69fa:     	addq	$0x108, %rsp            # imm = 0x108
1422a6a01:     	popq	%rbx
1422a6a02:     	popq	%rdi
1422a6a03:     	popq	%rsi
1422a6a04:     	popq	%r12
1422a6a06:     	popq	%r13
1422a6a08:     	popq	%r14
1422a6a0a:     	popq	%r15
1422a6a0c:     	popq	%rbp
1422a6a0d:     	retq
1422a6a0e:     	nop
1422a6a10:     	movq	%rdx, 0x10(%rsp)
1422a6a15:     	pushq	%rbp
1422a6a16:     	pushq	%r15
1422a6a18:     	pushq	%r14
1422a6a1a:     	pushq	%r13
1422a6a1c:     	pushq	%r12
1422a6a1e:     	pushq	%rsi
1422a6a1f:     	pushq	%rdi
1422a6a20:     	pushq	%rbx
1422a6a21:     	subq	$0x108, %rsp            # imm = 0x108
1422a6a28:     	leaq	0x80(%rdx), %rbp
1422a6a2f:     	movaps	%xmm15, 0x60(%rsp)
1422a6a35:     	movaps	%xmm14, 0x70(%rsp)
1422a6a3b:     	movaps	%xmm13, 0x80(%rsp)
1422a6a44:     	movaps	%xmm12, 0x90(%rsp)
1422a6a4d:     	movaps	%xmm11, 0xa0(%rsp)
1422a6a56:     	movaps	%xmm10, 0xb0(%rsp)
1422a6a5f:     	movaps	%xmm9, 0xc0(%rsp)
1422a6a68:     	movaps	%xmm8, 0xd0(%rsp)
1422a6a71:     	movaps	%xmm7, 0xe0(%rsp)
1422a6a79:     	movaps	%xmm6, 0xf0(%rsp)
1422a6a81:     	movl	$0x15, %edx
1422a6a86:     	movl	$0x1, %r8d
1422a6a8c:     	movq	0x1d8(%rbp), %rcx
1422a6a93:     	callq	0x140613c20 <.text+0x612c20>
1422a6a98:     	movaps	0xf0(%rsp), %xmm6
1422a6aa0:     	movaps	0xe0(%rsp), %xmm7
1422a6aa8:     	movaps	0xd0(%rsp), %xmm8
1422a6ab1:     	movaps	0xc0(%rsp), %xmm9
1422a6aba:     	movaps	0xb0(%rsp), %xmm10
1422a6ac3:     	movaps	0xa0(%rsp), %xmm11
1422a6acc:     	movaps	0x90(%rsp), %xmm12
1422a6ad5:     	movaps	0x80(%rsp), %xmm13
1422a6ade:     	movaps	0x70(%rsp), %xmm14
1422a6ae4:     	movaps	0x60(%rsp), %xmm15
1422a6aea:     	addq	$0x108, %rsp            # imm = 0x108
1422a6af1:     	popq	%rbx
1422a6af2:     	popq	%rdi
1422a6af3:     	popq	%rsi
1422a6af4:     	popq	%r12
1422a6af6:     	popq	%r13
1422a6af8:     	popq	%r14
1422a6afa:     	popq	%r15
1422a6afc:     	popq	%rbp
1422a6afd:     	retq
1422a6afe:     	nop
1422a6b00:     	movq	%rdx, 0x10(%rsp)
1422a6b05:     	pushq	%rbp
1422a6b06:     	pushq	%r15
1422a6b08:     	pushq	%r14
1422a6b0a:     	pushq	%r13
1422a6b0c:     	pushq	%r12
1422a6b0e:     	pushq	%rsi
1422a6b0f:     	pushq	%rdi
1422a6b10:     	pushq	%rbx
1422a6b11:     	subq	$0x108, %rsp            # imm = 0x108
1422a6b18:     	leaq	0x80(%rdx), %rbp
1422a6b1f:     	movaps	%xmm15, 0x60(%rsp)
1422a6b25:     	movaps	%xmm14, 0x70(%rsp)
1422a6b2b:     	movaps	%xmm13, 0x80(%rsp)
1422a6b34:     	movaps	%xmm12, 0x90(%rsp)
1422a6b3d:     	movaps	%xmm11, 0xa0(%rsp)
1422a6b46:     	movaps	%xmm10, 0xb0(%rsp)
1422a6b4f:     	movaps	%xmm9, 0xc0(%rsp)
1422a6b58:     	movaps	%xmm8, 0xd0(%rsp)
1422a6b61:     	movaps	%xmm7, 0xe0(%rsp)
1422a6b69:     	movaps	%xmm6, 0xf0(%rsp)
1422a6b71:     	movl	$0x19, %edx
1422a6b76:     	movl	$0x1, %r8d
1422a6b7c:     	movq	0x200(%rbp), %rcx
1422a6b83:     	callq	0x140613c20 <.text+0x612c20>
1422a6b88:     	movaps	0xf0(%rsp), %xmm6
1422a6b90:     	movaps	0xe0(%rsp), %xmm7
1422a6b98:     	movaps	0xd0(%rsp), %xmm8
1422a6ba1:     	movaps	0xc0(%rsp), %xmm9
1422a6baa:     	movaps	0xb0(%rsp), %xmm10
1422a6bb3:     	movaps	0xa0(%rsp), %xmm11
1422a6bbc:     	movaps	0x90(%rsp), %xmm12
1422a6bc5:     	movaps	0x80(%rsp), %xmm13
1422a6bce:     	movaps	0x70(%rsp), %xmm14
1422a6bd4:     	movaps	0x60(%rsp), %xmm15
1422a6bda:     	addq	$0x108, %rsp            # imm = 0x108
1422a6be1:     	popq	%rbx
1422a6be2:     	popq	%rdi
1422a6be3:     	popq	%rsi
1422a6be4:     	popq	%r12
1422a6be6:     	popq	%r13
1422a6be8:     	popq	%r14
1422a6bea:     	popq	%r15
1422a6bec:     	popq	%rbp
1422a6bed:     	retq
1422a6bee:     	nop
1422a6bf0:     	movq	%rdx, 0x10(%rsp)
1422a6bf5:     	pushq	%rbp
1422a6bf6:     	pushq	%r15
1422a6bf8:     	pushq	%r14
1422a6bfa:     	pushq	%r13
1422a6bfc:     	pushq	%r12
1422a6bfe:     	pushq	%rsi
1422a6bff:     	pushq	%rdi
1422a6c00:     	pushq	%rbx
1422a6c01:     	subq	$0x108, %rsp            # imm = 0x108
1422a6c08:     	leaq	0x80(%rdx), %rbp
1422a6c0f:     	movaps	%xmm15, 0x60(%rsp)
1422a6c15:     	movaps	%xmm14, 0x70(%rsp)
1422a6c1b:     	movaps	%xmm13, 0x80(%rsp)
1422a6c24:     	movaps	%xmm12, 0x90(%rsp)
1422a6c2d:     	movaps	%xmm11, 0xa0(%rsp)
1422a6c36:     	movaps	%xmm10, 0xb0(%rsp)
1422a6c3f:     	movaps	%xmm9, 0xc0(%rsp)
1422a6c48:     	movaps	%xmm8, 0xd0(%rsp)
1422a6c51:     	movaps	%xmm7, 0xe0(%rsp)
1422a6c59:     	movaps	%xmm6, 0xf0(%rsp)
1422a6c61:     	movl	$0xa0, %edx
1422a6c66:     	movl	$0x8, %r8d
1422a6c6c:     	movq	0x1f0(%rbp), %rcx
1422a6c73:     	callq	0x140613c20 <.text+0x612c20>
1422a6c78:     	movaps	0xf0(%rsp), %xmm6
1422a6c80:     	movaps	0xe0(%rsp), %xmm7
1422a6c88:     	movaps	0xd0(%rsp), %xmm8
1422a6c91:     	movaps	0xc0(%rsp), %xmm9
1422a6c9a:     	movaps	0xb0(%rsp), %xmm10
1422a6ca3:     	movaps	0xa0(%rsp), %xmm11
1422a6cac:     	movaps	0x90(%rsp), %xmm12
1422a6cb5:     	movaps	0x80(%rsp), %xmm13
1422a6cbe:     	movaps	0x70(%rsp), %xmm14
1422a6cc4:     	movaps	0x60(%rsp), %xmm15
1422a6cca:     	addq	$0x108, %rsp            # imm = 0x108
1422a6cd1:     	popq	%rbx
1422a6cd2:     	popq	%rdi
1422a6cd3:     	popq	%rsi
1422a6cd4:     	popq	%r12
1422a6cd6:     	popq	%r13
1422a6cd8:     	popq	%r14
1422a6cda:     	popq	%r15
1422a6cdc:     	popq	%rbp
1422a6cdd:     	retq
1422a6cde:     	nop
1422a6ce0:     	movq	%rdx, 0x10(%rsp)
1422a6ce5:     	pushq	%rbp
1422a6ce6:     	pushq	%r15
1422a6ce8:     	pushq	%r14
1422a6cea:     	pushq	%r13
1422a6cec:     	pushq	%r12
1422a6cee:     	pushq	%rsi
1422a6cef:     	pushq	%rdi
1422a6cf0:     	pushq	%rbx
1422a6cf1:     	subq	$0x108, %rsp            # imm = 0x108
1422a6cf8:     	leaq	0x80(%rdx), %rbp
1422a6cff:     	movaps	%xmm15, 0x60(%rsp)
1422a6d05:     	movaps	%xmm14, 0x70(%rsp)
1422a6d0b:     	movaps	%xmm13, 0x80(%rsp)
1422a6d14:     	movaps	%xmm12, 0x90(%rsp)
1422a6d1d:     	movaps	%xmm11, 0xa0(%rsp)
1422a6d26:     	movaps	%xmm10, 0xb0(%rsp)
1422a6d2f:     	movaps	%xmm9, 0xc0(%rsp)
1422a6d38:     	movaps	%xmm8, 0xd0(%rsp)
1422a6d41:     	movaps	%xmm7, 0xe0(%rsp)
1422a6d49:     	movaps	%xmm6, 0xf0(%rsp)
1422a6d51:     	movl	$0xe, %edx
1422a6d56:     	movl	$0x1, %r8d
1422a6d5c:     	movq	0x100(%rbp), %rcx
1422a6d63:     	callq	0x140613c20 <.text+0x612c20>
1422a6d68:     	movaps	0xf0(%rsp), %xmm6
1422a6d70:     	movaps	0xe0(%rsp), %xmm7
1422a6d78:     	movaps	0xd0(%rsp), %xmm8
1422a6d81:     	movaps	0xc0(%rsp), %xmm9
1422a6d8a:     	movaps	0xb0(%rsp), %xmm10
1422a6d93:     	movaps	0xa0(%rsp), %xmm11
1422a6d9c:     	movaps	0x90(%rsp), %xmm12
1422a6da5:     	movaps	0x80(%rsp), %xmm13
1422a6dae:     	movaps	0x70(%rsp), %xmm14
1422a6db4:     	movaps	0x60(%rsp), %xmm15
1422a6dba:     	addq	$0x108, %rsp            # imm = 0x108
1422a6dc1:     	popq	%rbx
1422a6dc2:     	popq	%rdi
1422a6dc3:     	popq	%rsi
1422a6dc4:     	popq	%r12
1422a6dc6:     	popq	%r13
1422a6dc8:     	popq	%r14
1422a6dca:     	popq	%r15
1422a6dcc:     	popq	%rbp
1422a6dcd:     	retq
1422a6dce:     	nop
1422a6dd0:     	movq	%rdx, 0x10(%rsp)
1422a6dd5:     	pushq	%rbp
1422a6dd6:     	pushq	%r15
1422a6dd8:     	pushq	%r14
1422a6dda:     	pushq	%r13
1422a6ddc:     	pushq	%r12
1422a6dde:     	pushq	%rsi
1422a6ddf:     	pushq	%rdi
1422a6de0:     	pushq	%rbx
1422a6de1:     	subq	$0x108, %rsp            # imm = 0x108
1422a6de8:     	leaq	0x80(%rdx), %rbp
1422a6def:     	movaps	%xmm15, 0x60(%rsp)
1422a6df5:     	movaps	%xmm14, 0x70(%rsp)
1422a6dfb:     	movaps	%xmm13, 0x80(%rsp)
1422a6e04:     	movaps	%xmm12, 0x90(%rsp)
1422a6e0d:     	movaps	%xmm11, 0xa0(%rsp)
1422a6e16:     	movaps	%xmm10, 0xb0(%rsp)
1422a6e1f:     	movaps	%xmm9, 0xc0(%rsp)
1422a6e28:     	movaps	%xmm8, 0xd0(%rsp)
1422a6e31:     	movaps	%xmm7, 0xe0(%rsp)
1422a6e39:     	movaps	%xmm6, 0xf0(%rsp)
1422a6e41:     	movl	$0x17, %edx
1422a6e46:     	movl	$0x1, %r8d
1422a6e4c:     	movq	0x1f0(%rbp), %rcx
1422a6e53:     	callq	0x140613c20 <.text+0x612c20>
1422a6e58:     	movaps	0xf0(%rsp), %xmm6
1422a6e60:     	movaps	0xe0(%rsp), %xmm7
1422a6e68:     	movaps	0xd0(%rsp), %xmm8
1422a6e71:     	movaps	0xc0(%rsp), %xmm9
1422a6e7a:     	movaps	0xb0(%rsp), %xmm10
1422a6e83:     	movaps	0xa0(%rsp), %xmm11
1422a6e8c:     	movaps	0x90(%rsp), %xmm12
1422a6e95:     	movaps	0x80(%rsp), %xmm13
1422a6e9e:     	movaps	0x70(%rsp), %xmm14
1422a6ea4:     	movaps	0x60(%rsp), %xmm15
1422a6eaa:     	addq	$0x108, %rsp            # imm = 0x108
1422a6eb1:     	popq	%rbx
1422a6eb2:     	popq	%rdi
1422a6eb3:     	popq	%rsi
1422a6eb4:     	popq	%r12
1422a6eb6:     	popq	%r13
1422a6eb8:     	popq	%r14
1422a6eba:     	popq	%r15
1422a6ebc:     	popq	%rbp
1422a6ebd:     	retq
1422a6ebe:     	nop
1422a6ec0:     	movq	%rdx, 0x10(%rsp)
1422a6ec5:     	pushq	%rbp
1422a6ec6:     	pushq	%r15
1422a6ec8:     	pushq	%r14
1422a6eca:     	pushq	%r13
1422a6ecc:     	pushq	%r12
1422a6ece:     	pushq	%rsi
1422a6ecf:     	pushq	%rdi
1422a6ed0:     	pushq	%rbx
1422a6ed1:     	subq	$0x108, %rsp            # imm = 0x108
1422a6ed8:     	leaq	0x80(%rdx), %rbp
1422a6edf:     	movaps	%xmm15, 0x60(%rsp)
1422a6ee5:     	movaps	%xmm14, 0x70(%rsp)
1422a6eeb:     	movaps	%xmm13, 0x80(%rsp)
1422a6ef4:     	movaps	%xmm12, 0x90(%rsp)
1422a6efd:     	movaps	%xmm11, 0xa0(%rsp)
1422a6f06:     	movaps	%xmm10, 0xb0(%rsp)
1422a6f0f:     	movaps	%xmm9, 0xc0(%rsp)
1422a6f18:     	movaps	%xmm8, 0xd0(%rsp)
1422a6f21:     	movaps	%xmm7, 0xe0(%rsp)
1422a6f29:     	movaps	%xmm6, 0xf0(%rsp)
1422a6f31:     	movl	$0x17, %edx
1422a6f36:     	movl	$0x1, %r8d
1422a6f3c:     	movq	0x1e0(%rbp), %rcx
1422a6f43:     	callq	0x140613c20 <.text+0x612c20>
1422a6f48:     	movaps	0xf0(%rsp), %xmm6
1422a6f50:     	movaps	0xe0(%rsp), %xmm7
1422a6f58:     	movaps	0xd0(%rsp), %xmm8
1422a6f61:     	movaps	0xc0(%rsp), %xmm9
1422a6f6a:     	movaps	0xb0(%rsp), %xmm10
1422a6f73:     	movaps	0xa0(%rsp), %xmm11
1422a6f7c:     	movaps	0x90(%rsp), %xmm12
1422a6f85:     	movaps	0x80(%rsp), %xmm13
1422a6f8e:     	movaps	0x70(%rsp), %xmm14
1422a6f94:     	movaps	0x60(%rsp), %xmm15
1422a6f9a:     	addq	$0x108, %rsp            # imm = 0x108
1422a6fa1:     	popq	%rbx
1422a6fa2:     	popq	%rdi
1422a6fa3:     	popq	%rsi
1422a6fa4:     	popq	%r12
1422a6fa6:     	popq	%r13
1422a6fa8:     	popq	%r14
1422a6faa:     	popq	%r15
1422a6fac:     	popq	%rbp
1422a6fad:     	retq
1422a6fae:     	nop
1422a6fb0:     	movq	%rdx, 0x10(%rsp)
1422a6fb5:     	pushq	%rbp
1422a6fb6:     	pushq	%r15
1422a6fb8:     	pushq	%r14
1422a6fba:     	pushq	%r13
1422a6fbc:     	pushq	%r12
1422a6fbe:     	pushq	%rsi
1422a6fbf:     	pushq	%rdi
1422a6fc0:     	pushq	%rbx
1422a6fc1:     	subq	$0x108, %rsp            # imm = 0x108
1422a6fc8:     	leaq	0x80(%rdx), %rbp
1422a6fcf:     	movaps	%xmm15, 0x60(%rsp)
1422a6fd5:     	movaps	%xmm14, 0x70(%rsp)
1422a6fdb:     	movaps	%xmm13, 0x80(%rsp)
1422a6fe4:     	movaps	%xmm12, 0x90(%rsp)
1422a6fed:     	movaps	%xmm11, 0xa0(%rsp)
1422a6ff6:     	movaps	%xmm10, 0xb0(%rsp)
1422a6fff:     	movaps	%xmm9, 0xc0(%rsp)
1422a7008:     	movaps	%xmm8, 0xd0(%rsp)
1422a7011:     	movaps	%xmm7, 0xe0(%rsp)
1422a7019:     	movaps	%xmm6, 0xf0(%rsp)
1422a7021:     	movl	$0x11, %edx
1422a7026:     	movl	$0x1, %r8d
1422a702c:     	movq	0x100(%rbp), %rcx
1422a7033:     	callq	0x140613c20 <.text+0x612c20>
1422a7038:     	movaps	0xf0(%rsp), %xmm6
1422a7040:     	movaps	0xe0(%rsp), %xmm7
1422a7048:     	movaps	0xd0(%rsp), %xmm8
1422a7051:     	movaps	0xc0(%rsp), %xmm9
1422a705a:     	movaps	0xb0(%rsp), %xmm10
1422a7063:     	movaps	0xa0(%rsp), %xmm11
1422a706c:     	movaps	0x90(%rsp), %xmm12
1422a7075:     	movaps	0x80(%rsp), %xmm13
1422a707e:     	movaps	0x70(%rsp), %xmm14
1422a7084:     	movaps	0x60(%rsp), %xmm15
1422a708a:     	addq	$0x108, %rsp            # imm = 0x108
1422a7091:     	popq	%rbx
1422a7092:     	popq	%rdi
1422a7093:     	popq	%rsi
1422a7094:     	popq	%r12
1422a7096:     	popq	%r13
1422a7098:     	popq	%r14
1422a709a:     	popq	%r15
1422a709c:     	popq	%rbp
1422a709d:     	retq
1422a709e:     	nop
1422a70a0:     	movq	%rdx, 0x10(%rsp)
1422a70a5:     	pushq	%rbp
1422a70a6:     	pushq	%r15
1422a70a8:     	pushq	%r14
1422a70aa:     	pushq	%r13
1422a70ac:     	pushq	%r12
1422a70ae:     	pushq	%rsi
1422a70af:     	pushq	%rdi
1422a70b0:     	pushq	%rbx
1422a70b1:     	subq	$0x108, %rsp            # imm = 0x108
1422a70b8:     	leaq	0x80(%rdx), %rbp
1422a70bf:     	movaps	%xmm15, 0x60(%rsp)
1422a70c5:     	movaps	%xmm14, 0x70(%rsp)
1422a70cb:     	movaps	%xmm13, 0x80(%rsp)
1422a70d4:     	movaps	%xmm12, 0x90(%rsp)
1422a70dd:     	movaps	%xmm11, 0xa0(%rsp)
1422a70e6:     	movaps	%xmm10, 0xb0(%rsp)
1422a70ef:     	movaps	%xmm9, 0xc0(%rsp)
1422a70f8:     	movaps	%xmm8, 0xd0(%rsp)
1422a7101:     	movaps	%xmm7, 0xe0(%rsp)
1422a7109:     	movaps	%xmm6, 0xf0(%rsp)
1422a7111:     	movl	$0x1b, %edx
1422a7116:     	movl	$0x1, %r8d
1422a711c:     	movq	0x1e8(%rbp), %rcx
1422a7123:     	callq	0x140613c20 <.text+0x612c20>
1422a7128:     	movaps	0xf0(%rsp), %xmm6
1422a7130:     	movaps	0xe0(%rsp), %xmm7
1422a7138:     	movaps	0xd0(%rsp), %xmm8
1422a7141:     	movaps	0xc0(%rsp), %xmm9
1422a714a:     	movaps	0xb0(%rsp), %xmm10
1422a7153:     	movaps	0xa0(%rsp), %xmm11
1422a715c:     	movaps	0x90(%rsp), %xmm12
1422a7165:     	movaps	0x80(%rsp), %xmm13
1422a716e:     	movaps	0x70(%rsp), %xmm14
1422a7174:     	movaps	0x60(%rsp), %xmm15
1422a717a:     	addq	$0x108, %rsp            # imm = 0x108
1422a7181:     	popq	%rbx
1422a7182:     	popq	%rdi
1422a7183:     	popq	%rsi
1422a7184:     	popq	%r12
1422a7186:     	popq	%r13
1422a7188:     	popq	%r14
1422a718a:     	popq	%r15
1422a718c:     	popq	%rbp
1422a718d:     	retq
1422a718e:     	nop
1422a7190:     	movq	%rdx, 0x10(%rsp)
1422a7195:     	pushq	%rbp
1422a7196:     	pushq	%r15
1422a7198:     	pushq	%r14
1422a719a:     	pushq	%r13
1422a719c:     	pushq	%r12
1422a719e:     	pushq	%rsi
1422a719f:     	pushq	%rdi
1422a71a0:     	pushq	%rbx
1422a71a1:     	subq	$0x108, %rsp            # imm = 0x108
1422a71a8:     	leaq	0x80(%rdx), %rbp
1422a71af:     	movaps	%xmm15, 0x60(%rsp)
1422a71b5:     	movaps	%xmm14, 0x70(%rsp)
1422a71bb:     	movaps	%xmm13, 0x80(%rsp)
1422a71c4:     	movaps	%xmm12, 0x90(%rsp)
1422a71cd:     	movaps	%xmm11, 0xa0(%rsp)
1422a71d6:     	movaps	%xmm10, 0xb0(%rsp)
1422a71df:     	movaps	%xmm9, 0xc0(%rsp)
1422a71e8:     	movaps	%xmm8, 0xd0(%rsp)
1422a71f1:     	movaps	%xmm7, 0xe0(%rsp)
1422a71f9:     	movaps	%xmm6, 0xf0(%rsp)
1422a7201:     	movl	$0x11, %edx
1422a7206:     	movl	$0x1, %r8d
1422a720c:     	movq	0x1b0(%rbp), %rcx
1422a7213:     	callq	0x140613c20 <.text+0x612c20>
1422a7218:     	movaps	0xf0(%rsp), %xmm6
1422a7220:     	movaps	0xe0(%rsp), %xmm7
1422a7228:     	movaps	0xd0(%rsp), %xmm8
1422a7231:     	movaps	0xc0(%rsp), %xmm9
1422a723a:     	movaps	0xb0(%rsp), %xmm10
1422a7243:     	movaps	0xa0(%rsp), %xmm11
1422a724c:     	movaps	0x90(%rsp), %xmm12
1422a7255:     	movaps	0x80(%rsp), %xmm13
1422a725e:     	movaps	0x70(%rsp), %xmm14
1422a7264:     	movaps	0x60(%rsp), %xmm15
1422a726a:     	addq	$0x108, %rsp            # imm = 0x108
1422a7271:     	popq	%rbx
1422a7272:     	popq	%rdi
1422a7273:     	popq	%rsi
1422a7274:     	popq	%r12
1422a7276:     	popq	%r13
1422a7278:     	popq	%r14
1422a727a:     	popq	%r15
1422a727c:     	popq	%rbp
1422a727d:     	retq
1422a727e:     	nop
1422a7280:     	movq	%rdx, 0x10(%rsp)
1422a7285:     	pushq	%rbp
1422a7286:     	pushq	%r15
1422a7288:     	pushq	%r14
1422a728a:     	pushq	%r13
1422a728c:     	pushq	%r12
1422a728e:     	pushq	%rsi
1422a728f:     	pushq	%rdi
1422a7290:     	pushq	%rbx
1422a7291:     	subq	$0x108, %rsp            # imm = 0x108
1422a7298:     	leaq	0x80(%rdx), %rbp
1422a729f:     	movaps	%xmm15, 0x60(%rsp)
1422a72a5:     	movaps	%xmm14, 0x70(%rsp)
1422a72ab:     	movaps	%xmm13, 0x80(%rsp)
1422a72b4:     	movaps	%xmm12, 0x90(%rsp)
1422a72bd:     	movaps	%xmm11, 0xa0(%rsp)
1422a72c6:     	movaps	%xmm10, 0xb0(%rsp)
1422a72cf:     	movaps	%xmm9, 0xc0(%rsp)
1422a72d8:     	movaps	%xmm8, 0xd0(%rsp)
1422a72e1:     	movaps	%xmm7, 0xe0(%rsp)
1422a72e9:     	movaps	%xmm6, 0xf0(%rsp)
1422a72f1:     	movl	$0x17, %edx
1422a72f6:     	movl	$0x1, %r8d
1422a72fc:     	movq	0x148(%rbp), %rcx
1422a7303:     	callq	0x140613c20 <.text+0x612c20>
1422a7308:     	movaps	0xf0(%rsp), %xmm6
1422a7310:     	movaps	0xe0(%rsp), %xmm7
1422a7318:     	movaps	0xd0(%rsp), %xmm8
1422a7321:     	movaps	0xc0(%rsp), %xmm9
1422a732a:     	movaps	0xb0(%rsp), %xmm10
1422a7333:     	movaps	0xa0(%rsp), %xmm11
1422a733c:     	movaps	0x90(%rsp), %xmm12
1422a7345:     	movaps	0x80(%rsp), %xmm13
1422a734e:     	movaps	0x70(%rsp), %xmm14
1422a7354:     	movaps	0x60(%rsp), %xmm15
1422a735a:     	addq	$0x108, %rsp            # imm = 0x108
1422a7361:     	popq	%rbx
1422a7362:     	popq	%rdi
1422a7363:     	popq	%rsi
1422a7364:     	popq	%r12
1422a7366:     	popq	%r13
1422a7368:     	popq	%r14
1422a736a:     	popq	%r15
1422a736c:     	popq	%rbp
1422a736d:     	retq
1422a736e:     	nop
1422a7370:     	movq	%rdx, 0x10(%rsp)
1422a7375:     	pushq	%rbp
1422a7376:     	pushq	%r15
1422a7378:     	pushq	%r14
1422a737a:     	pushq	%r13
1422a737c:     	pushq	%r12
1422a737e:     	pushq	%rsi
1422a737f:     	pushq	%rdi
1422a7380:     	pushq	%rbx
1422a7381:     	subq	$0x108, %rsp            # imm = 0x108
1422a7388:     	leaq	0x80(%rdx), %rbp
1422a738f:     	movaps	%xmm15, 0x60(%rsp)
1422a7395:     	movaps	%xmm14, 0x70(%rsp)
1422a739b:     	movaps	%xmm13, 0x80(%rsp)
1422a73a4:     	movaps	%xmm12, 0x90(%rsp)
1422a73ad:     	movaps	%xmm11, 0xa0(%rsp)
1422a73b6:     	movaps	%xmm10, 0xb0(%rsp)
1422a73bf:     	movaps	%xmm9, 0xc0(%rsp)
1422a73c8:     	movaps	%xmm8, 0xd0(%rsp)
1422a73d1:     	movaps	%xmm7, 0xe0(%rsp)
1422a73d9:     	movaps	%xmm6, 0xf0(%rsp)
1422a73e1:     	movl	$0x11, %edx
1422a73e6:     	movl	$0x1, %r8d
1422a73ec:     	movq	0x1c0(%rbp), %rcx
1422a73f3:     	callq	0x140613c20 <.text+0x612c20>
1422a73f8:     	movaps	0xf0(%rsp), %xmm6
1422a7400:     	movaps	0xe0(%rsp), %xmm7
1422a7408:     	movaps	0xd0(%rsp), %xmm8
1422a7411:     	movaps	0xc0(%rsp), %xmm9
1422a741a:     	movaps	0xb0(%rsp), %xmm10
1422a7423:     	movaps	0xa0(%rsp), %xmm11
1422a742c:     	movaps	0x90(%rsp), %xmm12
1422a7435:     	movaps	0x80(%rsp), %xmm13
1422a743e:     	movaps	0x70(%rsp), %xmm14
1422a7444:     	movaps	0x60(%rsp), %xmm15
1422a744a:     	addq	$0x108, %rsp            # imm = 0x108
1422a7451:     	popq	%rbx
1422a7452:     	popq	%rdi
1422a7453:     	popq	%rsi
1422a7454:     	popq	%r12
1422a7456:     	popq	%r13
1422a7458:     	popq	%r14
1422a745a:     	popq	%r15
1422a745c:     	popq	%rbp
1422a745d:     	retq
1422a745e:     	nop
1422a7460:     	movq	%rdx, 0x10(%rsp)
1422a7465:     	pushq	%rbp
1422a7466:     	pushq	%r15
1422a7468:     	pushq	%r14
1422a746a:     	pushq	%r13
1422a746c:     	pushq	%r12
1422a746e:     	pushq	%rsi
1422a746f:     	pushq	%rdi
1422a7470:     	pushq	%rbx
1422a7471:     	subq	$0x108, %rsp            # imm = 0x108
1422a7478:     	leaq	0x80(%rdx), %rbp
1422a747f:     	movaps	%xmm15, 0x60(%rsp)
1422a7485:     	movaps	%xmm14, 0x70(%rsp)
1422a748b:     	movaps	%xmm13, 0x80(%rsp)
1422a7494:     	movaps	%xmm12, 0x90(%rsp)
1422a749d:     	movaps	%xmm11, 0xa0(%rsp)
1422a74a6:     	movaps	%xmm10, 0xb0(%rsp)
1422a74af:     	movaps	%xmm9, 0xc0(%rsp)
1422a74b8:     	movaps	%xmm8, 0xd0(%rsp)
1422a74c1:     	movaps	%xmm7, 0xe0(%rsp)
1422a74c9:     	movaps	%xmm6, 0xf0(%rsp)
1422a74d1:     	movl	$0x12, %edx
1422a74d6:     	movl	$0x1, %r8d
1422a74dc:     	movq	0x1d8(%rbp), %rcx
1422a74e3:     	callq	0x140613c20 <.text+0x612c20>
1422a74e8:     	movaps	0xf0(%rsp), %xmm6
1422a74f0:     	movaps	0xe0(%rsp), %xmm7
1422a74f8:     	movaps	0xd0(%rsp), %xmm8
1422a7501:     	movaps	0xc0(%rsp), %xmm9
1422a750a:     	movaps	0xb0(%rsp), %xmm10
1422a7513:     	movaps	0xa0(%rsp), %xmm11
1422a751c:     	movaps	0x90(%rsp), %xmm12
1422a7525:     	movaps	0x80(%rsp), %xmm13
1422a752e:     	movaps	0x70(%rsp), %xmm14
1422a7534:     	movaps	0x60(%rsp), %xmm15
1422a753a:     	addq	$0x108, %rsp            # imm = 0x108
1422a7541:     	popq	%rbx
1422a7542:     	popq	%rdi
1422a7543:     	popq	%rsi
1422a7544:     	popq	%r12
1422a7546:     	popq	%r13
1422a7548:     	popq	%r14
1422a754a:     	popq	%r15
1422a754c:     	popq	%rbp
1422a754d:     	retq
1422a754e:     	nop
1422a7550:     	movq	%rdx, 0x10(%rsp)
1422a7555:     	pushq	%rbp
1422a7556:     	pushq	%r15
1422a7558:     	pushq	%r14
1422a755a:     	pushq	%r13
1422a755c:     	pushq	%r12
1422a755e:     	pushq	%rsi
1422a755f:     	pushq	%rdi
1422a7560:     	pushq	%rbx
1422a7561:     	subq	$0x108, %rsp            # imm = 0x108
1422a7568:     	leaq	0x80(%rdx), %rbp
1422a756f:     	movaps	%xmm15, 0x60(%rsp)
1422a7575:     	movaps	%xmm14, 0x70(%rsp)
1422a757b:     	movaps	%xmm13, 0x80(%rsp)
1422a7584:     	movaps	%xmm12, 0x90(%rsp)
1422a758d:     	movaps	%xmm11, 0xa0(%rsp)
1422a7596:     	movaps	%xmm10, 0xb0(%rsp)
1422a759f:     	movaps	%xmm9, 0xc0(%rsp)
1422a75a8:     	movaps	%xmm8, 0xd0(%rsp)
1422a75b1:     	movaps	%xmm7, 0xe0(%rsp)
1422a75b9:     	movaps	%xmm6, 0xf0(%rsp)
1422a75c1:     	movl	$0x14, %edx
1422a75c6:     	movl	$0x1, %r8d
1422a75cc:     	movq	0x1c0(%rbp), %rcx
1422a75d3:     	callq	0x140613c20 <.text+0x612c20>
1422a75d8:     	movaps	0xf0(%rsp), %xmm6
1422a75e0:     	movaps	0xe0(%rsp), %xmm7
1422a75e8:     	movaps	0xd0(%rsp), %xmm8
1422a75f1:     	movaps	0xc0(%rsp), %xmm9
1422a75fa:     	movaps	0xb0(%rsp), %xmm10
1422a7603:     	movaps	0xa0(%rsp), %xmm11
1422a760c:     	movaps	0x90(%rsp), %xmm12
1422a7615:     	movaps	0x80(%rsp), %xmm13
1422a761e:     	movaps	0x70(%rsp), %xmm14
1422a7624:     	movaps	0x60(%rsp), %xmm15
1422a762a:     	addq	$0x108, %rsp            # imm = 0x108
1422a7631:     	popq	%rbx
1422a7632:     	popq	%rdi
1422a7633:     	popq	%rsi
1422a7634:     	popq	%r12
1422a7636:     	popq	%r13
1422a7638:     	popq	%r14
1422a763a:     	popq	%r15
1422a763c:     	popq	%rbp
1422a763d:     	retq
1422a763e:     	nop
1422a7640:     	movq	%rdx, 0x10(%rsp)
1422a7645:     	pushq	%rbp
1422a7646:     	pushq	%r15
1422a7648:     	pushq	%r14
1422a764a:     	pushq	%r13
1422a764c:     	pushq	%r12
1422a764e:     	pushq	%rsi
1422a764f:     	pushq	%rdi
1422a7650:     	pushq	%rbx
1422a7651:     	subq	$0x108, %rsp            # imm = 0x108
1422a7658:     	leaq	0x80(%rdx), %rbp
1422a765f:     	movaps	%xmm15, 0x60(%rsp)
1422a7665:     	movaps	%xmm14, 0x70(%rsp)
1422a766b:     	movaps	%xmm13, 0x80(%rsp)
1422a7674:     	movaps	%xmm12, 0x90(%rsp)
1422a767d:     	movaps	%xmm11, 0xa0(%rsp)
1422a7686:     	movaps	%xmm10, 0xb0(%rsp)
1422a768f:     	movaps	%xmm9, 0xc0(%rsp)
1422a7698:     	movaps	%xmm8, 0xd0(%rsp)
1422a76a1:     	movaps	%xmm7, 0xe0(%rsp)
1422a76a9:     	movaps	%xmm6, 0xf0(%rsp)
1422a76b1:     	movl	$0x17, %edx
1422a76b6:     	movl	$0x1, %r8d
1422a76bc:     	movq	0x1d8(%rbp), %rcx
1422a76c3:     	callq	0x140613c20 <.text+0x612c20>
1422a76c8:     	movaps	0xf0(%rsp), %xmm6
1422a76d0:     	movaps	0xe0(%rsp), %xmm7
1422a76d8:     	movaps	0xd0(%rsp), %xmm8
1422a76e1:     	movaps	0xc0(%rsp), %xmm9
1422a76ea:     	movaps	0xb0(%rsp), %xmm10
1422a76f3:     	movaps	0xa0(%rsp), %xmm11
1422a76fc:     	movaps	0x90(%rsp), %xmm12
1422a7705:     	movaps	0x80(%rsp), %xmm13
1422a770e:     	movaps	0x70(%rsp), %xmm14
1422a7714:     	movaps	0x60(%rsp), %xmm15
1422a771a:     	addq	$0x108, %rsp            # imm = 0x108
1422a7721:     	popq	%rbx
1422a7722:     	popq	%rdi
1422a7723:     	popq	%rsi
1422a7724:     	popq	%r12
1422a7726:     	popq	%r13
1422a7728:     	popq	%r14
1422a772a:     	popq	%r15
1422a772c:     	popq	%rbp
1422a772d:     	retq
1422a772e:     	nop
1422a7730:     	movq	%rdx, 0x10(%rsp)
1422a7735:     	pushq	%rbp
1422a7736:     	pushq	%r15
1422a7738:     	pushq	%r14
1422a773a:     	pushq	%r13
1422a773c:     	pushq	%r12
1422a773e:     	pushq	%rsi
1422a773f:     	pushq	%rdi
1422a7740:     	pushq	%rbx
1422a7741:     	subq	$0x108, %rsp            # imm = 0x108
1422a7748:     	leaq	0x80(%rdx), %rbp
1422a774f:     	movaps	%xmm15, 0x60(%rsp)
1422a7755:     	movaps	%xmm14, 0x70(%rsp)
1422a775b:     	movaps	%xmm13, 0x80(%rsp)
1422a7764:     	movaps	%xmm12, 0x90(%rsp)
1422a776d:     	movaps	%xmm11, 0xa0(%rsp)
1422a7776:     	movaps	%xmm10, 0xb0(%rsp)
1422a777f:     	movaps	%xmm9, 0xc0(%rsp)
1422a7788:     	movaps	%xmm8, 0xd0(%rsp)
1422a7791:     	movaps	%xmm7, 0xe0(%rsp)
1422a7799:     	movaps	%xmm6, 0xf0(%rsp)
1422a77a1:     	movl	$0x15, %edx
1422a77a6:     	movl	$0x1, %r8d
1422a77ac:     	movq	0x1f8(%rbp), %rcx
1422a77b3:     	callq	0x140613c20 <.text+0x612c20>
1422a77b8:     	movaps	0xf0(%rsp), %xmm6
1422a77c0:     	movaps	0xe0(%rsp), %xmm7
1422a77c8:     	movaps	0xd0(%rsp), %xmm8
1422a77d1:     	movaps	0xc0(%rsp), %xmm9
1422a77da:     	movaps	0xb0(%rsp), %xmm10
1422a77e3:     	movaps	0xa0(%rsp), %xmm11
1422a77ec:     	movaps	0x90(%rsp), %xmm12
1422a77f5:     	movaps	0x80(%rsp), %xmm13
1422a77fe:     	movaps	0x70(%rsp), %xmm14
1422a7804:     	movaps	0x60(%rsp), %xmm15
1422a780a:     	addq	$0x108, %rsp            # imm = 0x108
1422a7811:     	popq	%rbx
1422a7812:     	popq	%rdi
1422a7813:     	popq	%rsi
1422a7814:     	popq	%r12
1422a7816:     	popq	%r13
1422a7818:     	popq	%r14
1422a781a:     	popq	%r15
1422a781c:     	popq	%rbp
1422a781d:     	retq
1422a781e:     	nop
1422a7820:     	movq	%rdx, 0x10(%rsp)
1422a7825:     	pushq	%rbp
1422a7826:     	pushq	%r15
1422a7828:     	pushq	%r14
1422a782a:     	pushq	%r13
1422a782c:     	pushq	%r12
1422a782e:     	pushq	%rsi
1422a782f:     	pushq	%rdi
1422a7830:     	pushq	%rbx
1422a7831:     	subq	$0x108, %rsp            # imm = 0x108
1422a7838:     	leaq	0x80(%rdx), %rbp
1422a783f:     	movaps	%xmm15, 0x60(%rsp)
1422a7845:     	movaps	%xmm14, 0x70(%rsp)
1422a784b:     	movaps	%xmm13, 0x80(%rsp)
1422a7854:     	movaps	%xmm12, 0x90(%rsp)
1422a785d:     	movaps	%xmm11, 0xa0(%rsp)
1422a7866:     	movaps	%xmm10, 0xb0(%rsp)
1422a786f:     	movaps	%xmm9, 0xc0(%rsp)
1422a7878:     	movaps	%xmm8, 0xd0(%rsp)
1422a7881:     	movaps	%xmm7, 0xe0(%rsp)
1422a7889:     	movaps	%xmm6, 0xf0(%rsp)
1422a7891:     	movl	$0x19, %edx
1422a7896:     	movl	$0x1, %r8d
1422a789c:     	movq	0x200(%rbp), %rcx
1422a78a3:     	callq	0x140613c20 <.text+0x612c20>
1422a78a8:     	movaps	0xf0(%rsp), %xmm6
1422a78b0:     	movaps	0xe0(%rsp), %xmm7
1422a78b8:     	movaps	0xd0(%rsp), %xmm8
1422a78c1:     	movaps	0xc0(%rsp), %xmm9
1422a78ca:     	movaps	0xb0(%rsp), %xmm10
1422a78d3:     	movaps	0xa0(%rsp), %xmm11
1422a78dc:     	movaps	0x90(%rsp), %xmm12
1422a78e5:     	movaps	0x80(%rsp), %xmm13
1422a78ee:     	movaps	0x70(%rsp), %xmm14
1422a78f4:     	movaps	0x60(%rsp), %xmm15
1422a78fa:     	addq	$0x108, %rsp            # imm = 0x108
1422a7901:     	popq	%rbx
1422a7902:     	popq	%rdi
1422a7903:     	popq	%rsi
1422a7904:     	popq	%r12
1422a7906:     	popq	%r13
1422a7908:     	popq	%r14
1422a790a:     	popq	%r15
1422a790c:     	popq	%rbp
1422a790d:     	retq
1422a790e:     	nop
1422a7910:     	movq	%rdx, 0x10(%rsp)
1422a7915:     	pushq	%rbp
1422a7916:     	pushq	%r15
1422a7918:     	pushq	%r14
1422a791a:     	pushq	%r13
1422a791c:     	pushq	%r12
1422a791e:     	pushq	%rsi
1422a791f:     	pushq	%rdi
1422a7920:     	pushq	%rbx
1422a7921:     	subq	$0x108, %rsp            # imm = 0x108
1422a7928:     	leaq	0x80(%rdx), %rbp
1422a792f:     	movaps	%xmm15, 0x60(%rsp)
1422a7935:     	movaps	%xmm14, 0x70(%rsp)
1422a793b:     	movaps	%xmm13, 0x80(%rsp)
1422a7944:     	movaps	%xmm12, 0x90(%rsp)
1422a794d:     	movaps	%xmm11, 0xa0(%rsp)
1422a7956:     	movaps	%xmm10, 0xb0(%rsp)
1422a795f:     	movaps	%xmm9, 0xc0(%rsp)
1422a7968:     	movaps	%xmm8, 0xd0(%rsp)
1422a7971:     	movaps	%xmm7, 0xe0(%rsp)
1422a7979:     	movaps	%xmm6, 0xf0(%rsp)
1422a7981:     	leaq	0x180(%rbp), %rcx
1422a7988:     	callq	0x14014a860 <.text+0x149860>
1422a798d:     	movaps	0xf0(%rsp), %xmm6
1422a7995:     	movaps	0xe0(%rsp), %xmm7
1422a799d:     	movaps	0xd0(%rsp), %xmm8
1422a79a6:     	movaps	0xc0(%rsp), %xmm9
1422a79af:     	movaps	0xb0(%rsp), %xmm10
1422a79b8:     	movaps	0xa0(%rsp), %xmm11
1422a79c1:     	movaps	0x90(%rsp), %xmm12
1422a79ca:     	movaps	0x80(%rsp), %xmm13
1422a79d3:     	movaps	0x70(%rsp), %xmm14
1422a79d9:     	movaps	0x60(%rsp), %xmm15
1422a79df:     	addq	$0x108, %rsp            # imm = 0x108
1422a79e6:     	popq	%rbx
1422a79e7:     	popq	%rdi
1422a79e8:     	popq	%rsi
1422a79e9:     	popq	%r12
1422a79eb:     	popq	%r13
1422a79ed:     	popq	%r14
1422a79ef:     	popq	%r15
1422a79f1:     	popq	%rbp
1422a79f2:     	retq
1422a79f3:     	nopw	%cs:(%rax,%rax)
1422a7a00:     	movq	%rdx, 0x10(%rsp)
1422a7a05:     	pushq	%rbp
1422a7a06:     	pushq	%r15
1422a7a08:     	pushq	%r14
1422a7a0a:     	pushq	%r13
1422a7a0c:     	pushq	%r12
1422a7a0e:     	pushq	%rsi
1422a7a0f:     	pushq	%rdi
1422a7a10:     	pushq	%rbx
1422a7a11:     	subq	$0x108, %rsp            # imm = 0x108
1422a7a18:     	leaq	0x80(%rdx), %rbp
1422a7a1f:     	movaps	%xmm15, 0x60(%rsp)
1422a7a25:     	movaps	%xmm14, 0x70(%rsp)
1422a7a2b:     	movaps	%xmm13, 0x80(%rsp)
1422a7a34:     	movaps	%xmm12, 0x90(%rsp)
1422a7a3d:     	movaps	%xmm11, 0xa0(%rsp)
1422a7a46:     	movaps	%xmm10, 0xb0(%rsp)
1422a7a4f:     	movaps	%xmm9, 0xc0(%rsp)
1422a7a58:     	movaps	%xmm8, 0xd0(%rsp)
1422a7a61:     	movaps	%xmm7, 0xe0(%rsp)
1422a7a69:     	movaps	%xmm6, 0xf0(%rsp)
1422a7a71:     	movl	$0xe, %edx
1422a7a76:     	movl	$0x1, %r8d
1422a7a7c:     	movq	0x148(%rbp), %rcx
1422a7a83:     	callq	0x140613c20 <.text+0x612c20>
1422a7a88:     	movaps	0xf0(%rsp), %xmm6
1422a7a90:     	movaps	0xe0(%rsp), %xmm7
1422a7a98:     	movaps	0xd0(%rsp), %xmm8
1422a7aa1:     	movaps	0xc0(%rsp), %xmm9
1422a7aaa:     	movaps	0xb0(%rsp), %xmm10
1422a7ab3:     	movaps	0xa0(%rsp), %xmm11
1422a7abc:     	movaps	0x90(%rsp), %xmm12
1422a7ac5:     	movaps	0x80(%rsp), %xmm13
1422a7ace:     	movaps	0x70(%rsp), %xmm14
1422a7ad4:     	movaps	0x60(%rsp), %xmm15
1422a7ada:     	addq	$0x108, %rsp            # imm = 0x108
1422a7ae1:     	popq	%rbx
1422a7ae2:     	popq	%rdi
1422a7ae3:     	popq	%rsi
1422a7ae4:     	popq	%r12
1422a7ae6:     	popq	%r13
1422a7ae8:     	popq	%r14
1422a7aea:     	popq	%r15
1422a7aec:     	popq	%rbp
1422a7aed:     	retq
1422a7aee:     	nop
1422a7af0:     	movq	%rdx, 0x10(%rsp)
1422a7af5:     	pushq	%rbp
1422a7af6:     	pushq	%r15
1422a7af8:     	pushq	%r14
1422a7afa:     	pushq	%r13
1422a7afc:     	pushq	%r12
1422a7afe:     	pushq	%rsi
1422a7aff:     	pushq	%rdi
1422a7b00:     	pushq	%rbx
1422a7b01:     	subq	$0x108, %rsp            # imm = 0x108
1422a7b08:     	leaq	0x80(%rdx), %rbp
1422a7b0f:     	movaps	%xmm15, 0x60(%rsp)
1422a7b15:     	movaps	%xmm14, 0x70(%rsp)
1422a7b1b:     	movaps	%xmm13, 0x80(%rsp)
1422a7b24:     	movaps	%xmm12, 0x90(%rsp)
1422a7b2d:     	movaps	%xmm11, 0xa0(%rsp)
1422a7b36:     	movaps	%xmm10, 0xb0(%rsp)
1422a7b3f:     	movaps	%xmm9, 0xc0(%rsp)
1422a7b48:     	movaps	%xmm8, 0xd0(%rsp)
1422a7b51:     	movaps	%xmm7, 0xe0(%rsp)
1422a7b59:     	movaps	%xmm6, 0xf0(%rsp)
1422a7b61:     	movl	$0x19, %edx
1422a7b66:     	movl	$0x1, %r8d
1422a7b6c:     	movq	0x1f0(%rbp), %rcx
1422a7b73:     	callq	0x140613c20 <.text+0x612c20>
1422a7b78:     	movaps	0xf0(%rsp), %xmm6
1422a7b80:     	movaps	0xe0(%rsp), %xmm7
1422a7b88:     	movaps	0xd0(%rsp), %xmm8
1422a7b91:     	movaps	0xc0(%rsp), %xmm9
1422a7b9a:     	movaps	0xb0(%rsp), %xmm10
1422a7ba3:     	movaps	0xa0(%rsp), %xmm11
1422a7bac:     	movaps	0x90(%rsp), %xmm12
1422a7bb5:     	movaps	0x80(%rsp), %xmm13
1422a7bbe:     	movaps	0x70(%rsp), %xmm14
1422a7bc4:     	movaps	0x60(%rsp), %xmm15
1422a7bca:     	addq	$0x108, %rsp            # imm = 0x108
1422a7bd1:     	popq	%rbx
1422a7bd2:     	popq	%rdi
1422a7bd3:     	popq	%rsi
1422a7bd4:     	popq	%r12
1422a7bd6:     	popq	%r13
1422a7bd8:     	popq	%r14
1422a7bda:     	popq	%r15
1422a7bdc:     	popq	%rbp
1422a7bdd:     	retq
1422a7bde:     	nop
1422a7be0:     	movq	%rdx, 0x10(%rsp)
1422a7be5:     	pushq	%rbp
1422a7be6:     	pushq	%r15
1422a7be8:     	pushq	%r14
1422a7bea:     	pushq	%r13
1422a7bec:     	pushq	%r12
1422a7bee:     	pushq	%rsi
1422a7bef:     	pushq	%rdi
1422a7bf0:     	pushq	%rbx
1422a7bf1:     	subq	$0x108, %rsp            # imm = 0x108
1422a7bf8:     	leaq	0x80(%rdx), %rbp
1422a7bff:     	movaps	%xmm15, 0x60(%rsp)
1422a7c05:     	movaps	%xmm14, 0x70(%rsp)
1422a7c0b:     	movaps	%xmm13, 0x80(%rsp)
1422a7c14:     	movaps	%xmm12, 0x90(%rsp)
1422a7c1d:     	movaps	%xmm11, 0xa0(%rsp)
1422a7c26:     	movaps	%xmm10, 0xb0(%rsp)
1422a7c2f:     	movaps	%xmm9, 0xc0(%rsp)
1422a7c38:     	movaps	%xmm8, 0xd0(%rsp)
1422a7c41:     	movaps	%xmm7, 0xe0(%rsp)
1422a7c49:     	movaps	%xmm6, 0xf0(%rsp)
1422a7c51:     	movl	$0x11, %edx
1422a7c56:     	movl	$0x1, %r8d
1422a7c5c:     	movq	0x148(%rbp), %rcx
1422a7c63:     	callq	0x140613c20 <.text+0x612c20>
1422a7c68:     	movaps	0xf0(%rsp), %xmm6
1422a7c70:     	movaps	0xe0(%rsp), %xmm7
1422a7c78:     	movaps	0xd0(%rsp), %xmm8
1422a7c81:     	movaps	0xc0(%rsp), %xmm9
1422a7c8a:     	movaps	0xb0(%rsp), %xmm10
1422a7c93:     	movaps	0xa0(%rsp), %xmm11
1422a7c9c:     	movaps	0x90(%rsp), %xmm12
1422a7ca5:     	movaps	0x80(%rsp), %xmm13
1422a7cae:     	movaps	0x70(%rsp), %xmm14
1422a7cb4:     	movaps	0x60(%rsp), %xmm15
1422a7cba:     	addq	$0x108, %rsp            # imm = 0x108
1422a7cc1:     	popq	%rbx
1422a7cc2:     	popq	%rdi
1422a7cc3:     	popq	%rsi
1422a7cc4:     	popq	%r12
1422a7cc6:     	popq	%r13
1422a7cc8:     	popq	%r14
1422a7cca:     	popq	%r15
1422a7ccc:     	popq	%rbp
1422a7ccd:     	retq
1422a7cce:     	nop
1422a7cd0:     	movq	%rdx, 0x10(%rsp)
1422a7cd5:     	pushq	%rbp
1422a7cd6:     	pushq	%r15
1422a7cd8:     	pushq	%r14
1422a7cda:     	pushq	%r13
1422a7cdc:     	pushq	%r12
1422a7cde:     	pushq	%rsi
1422a7cdf:     	pushq	%rdi
1422a7ce0:     	pushq	%rbx
1422a7ce1:     	subq	$0x108, %rsp            # imm = 0x108
1422a7ce8:     	leaq	0x80(%rdx), %rbp
1422a7cef:     	movaps	%xmm15, 0x60(%rsp)
1422a7cf5:     	movaps	%xmm14, 0x70(%rsp)
1422a7cfb:     	movaps	%xmm13, 0x80(%rsp)
1422a7d04:     	movaps	%xmm12, 0x90(%rsp)
1422a7d0d:     	movaps	%xmm11, 0xa0(%rsp)
1422a7d16:     	movaps	%xmm10, 0xb0(%rsp)
1422a7d1f:     	movaps	%xmm9, 0xc0(%rsp)
1422a7d28:     	movaps	%xmm8, 0xd0(%rsp)
1422a7d31:     	movaps	%xmm7, 0xe0(%rsp)
1422a7d39:     	movaps	%xmm6, 0xf0(%rsp)
1422a7d41:     	movl	$0x18, %edx
1422a7d46:     	movl	$0x1, %r8d
1422a7d4c:     	movq	0x1f8(%rbp), %rcx
1422a7d53:     	callq	0x140613c20 <.text+0x612c20>
1422a7d58:     	movaps	0xf0(%rsp), %xmm6
1422a7d60:     	movaps	0xe0(%rsp), %xmm7
1422a7d68:     	movaps	0xd0(%rsp), %xmm8
1422a7d71:     	movaps	0xc0(%rsp), %xmm9
1422a7d7a:     	movaps	0xb0(%rsp), %xmm10
1422a7d83:     	movaps	0xa0(%rsp), %xmm11
1422a7d8c:     	movaps	0x90(%rsp), %xmm12
1422a7d95:     	movaps	0x80(%rsp), %xmm13
1422a7d9e:     	movaps	0x70(%rsp), %xmm14
1422a7da4:     	movaps	0x60(%rsp), %xmm15
1422a7daa:     	addq	$0x108, %rsp            # imm = 0x108
1422a7db1:     	popq	%rbx
1422a7db2:     	popq	%rdi
1422a7db3:     	popq	%rsi
1422a7db4:     	popq	%r12
1422a7db6:     	popq	%r13
1422a7db8:     	popq	%r14
1422a7dba:     	popq	%r15
1422a7dbc:     	popq	%rbp
1422a7dbd:     	retq
1422a7dbe:     	nop
1422a7dc0:     	movq	%rdx, 0x10(%rsp)
1422a7dc5:     	pushq	%rbp
1422a7dc6:     	pushq	%r15
1422a7dc8:     	pushq	%r14
1422a7dca:     	pushq	%r13
1422a7dcc:     	pushq	%r12
1422a7dce:     	pushq	%rsi
1422a7dcf:     	pushq	%rdi
1422a7dd0:     	pushq	%rbx
1422a7dd1:     	subq	$0x108, %rsp            # imm = 0x108
1422a7dd8:     	leaq	0x80(%rdx), %rbp
1422a7ddf:     	movaps	%xmm15, 0x60(%rsp)
1422a7de5:     	movaps	%xmm14, 0x70(%rsp)
1422a7deb:     	movaps	%xmm13, 0x80(%rsp)
1422a7df4:     	movaps	%xmm12, 0x90(%rsp)
1422a7dfd:     	movaps	%xmm11, 0xa0(%rsp)
1422a7e06:     	movaps	%xmm10, 0xb0(%rsp)
1422a7e0f:     	movaps	%xmm9, 0xc0(%rsp)
1422a7e18:     	movaps	%xmm8, 0xd0(%rsp)
1422a7e21:     	movaps	%xmm7, 0xe0(%rsp)
1422a7e29:     	movaps	%xmm6, 0xf0(%rsp)
1422a7e31:     	movl	$0x11, %edx
1422a7e36:     	movl	$0x1, %r8d
1422a7e3c:     	movq	0x1c8(%rbp), %rcx
1422a7e43:     	callq	0x140613c20 <.text+0x612c20>
1422a7e48:     	movaps	0xf0(%rsp), %xmm6
1422a7e50:     	movaps	0xe0(%rsp), %xmm7
1422a7e58:     	movaps	0xd0(%rsp), %xmm8
1422a7e61:     	movaps	0xc0(%rsp), %xmm9
1422a7e6a:     	movaps	0xb0(%rsp), %xmm10
1422a7e73:     	movaps	0xa0(%rsp), %xmm11
1422a7e7c:     	movaps	0x90(%rsp), %xmm12
1422a7e85:     	movaps	0x80(%rsp), %xmm13
1422a7e8e:     	movaps	0x70(%rsp), %xmm14
1422a7e94:     	movaps	0x60(%rsp), %xmm15
1422a7e9a:     	addq	$0x108, %rsp            # imm = 0x108
1422a7ea1:     	popq	%rbx
1422a7ea2:     	popq	%rdi
1422a7ea3:     	popq	%rsi
1422a7ea4:     	popq	%r12
1422a7ea6:     	popq	%r13
1422a7ea8:     	popq	%r14
1422a7eaa:     	popq	%r15
1422a7eac:     	popq	%rbp
1422a7ead:     	retq
1422a7eae:     	nop
1422a7eb0:     	movq	%rdx, 0x10(%rsp)
1422a7eb5:     	pushq	%rbp
1422a7eb6:     	pushq	%r15
1422a7eb8:     	pushq	%r14
1422a7eba:     	pushq	%r13
1422a7ebc:     	pushq	%r12
1422a7ebe:     	pushq	%rsi
1422a7ebf:     	pushq	%rdi
1422a7ec0:     	pushq	%rbx
1422a7ec1:     	subq	$0x108, %rsp            # imm = 0x108
1422a7ec8:     	leaq	0x80(%rdx), %rbp
1422a7ecf:     	movaps	%xmm15, 0x60(%rsp)
1422a7ed5:     	movaps	%xmm14, 0x70(%rsp)
1422a7edb:     	movaps	%xmm13, 0x80(%rsp)
1422a7ee4:     	movaps	%xmm12, 0x90(%rsp)
1422a7eed:     	movaps	%xmm11, 0xa0(%rsp)
1422a7ef6:     	movaps	%xmm10, 0xb0(%rsp)
1422a7eff:     	movaps	%xmm9, 0xc0(%rsp)
1422a7f08:     	movaps	%xmm8, 0xd0(%rsp)
1422a7f11:     	movaps	%xmm7, 0xe0(%rsp)
1422a7f19:     	movaps	%xmm6, 0xf0(%rsp)
1422a7f21:     	movl	$0x1b, %edx
1422a7f26:     	movl	$0x1, %r8d
1422a7f2c:     	movq	0x1e8(%rbp), %rcx
1422a7f33:     	callq	0x140613c20 <.text+0x612c20>
1422a7f38:     	movaps	0xf0(%rsp), %xmm6
1422a7f40:     	movaps	0xe0(%rsp), %xmm7
1422a7f48:     	movaps	0xd0(%rsp), %xmm8
1422a7f51:     	movaps	0xc0(%rsp), %xmm9
1422a7f5a:     	movaps	0xb0(%rsp), %xmm10
1422a7f63:     	movaps	0xa0(%rsp), %xmm11
1422a7f6c:     	movaps	0x90(%rsp), %xmm12
1422a7f75:     	movaps	0x80(%rsp), %xmm13
1422a7f7e:     	movaps	0x70(%rsp), %xmm14
1422a7f84:     	movaps	0x60(%rsp), %xmm15
1422a7f8a:     	addq	$0x108, %rsp            # imm = 0x108
1422a7f91:     	popq	%rbx
1422a7f92:     	popq	%rdi
1422a7f93:     	popq	%rsi
1422a7f94:     	popq	%r12
1422a7f96:     	popq	%r13
1422a7f98:     	popq	%r14
1422a7f9a:     	popq	%r15
1422a7f9c:     	popq	%rbp
1422a7f9d:     	retq
1422a7f9e:     	nop
1422a7fa0:     	movq	%rdx, 0x10(%rsp)
1422a7fa5:     	pushq	%rbp
1422a7fa6:     	pushq	%r15
1422a7fa8:     	pushq	%r14
1422a7faa:     	pushq	%r13
1422a7fac:     	pushq	%r12
1422a7fae:     	pushq	%rsi
1422a7faf:     	pushq	%rdi
1422a7fb0:     	pushq	%rbx
1422a7fb1:     	subq	$0x108, %rsp            # imm = 0x108
1422a7fb8:     	leaq	0x80(%rdx), %rbp
1422a7fbf:     	movaps	%xmm15, 0x60(%rsp)
1422a7fc5:     	movaps	%xmm14, 0x70(%rsp)
1422a7fcb:     	movaps	%xmm13, 0x80(%rsp)
1422a7fd4:     	movaps	%xmm12, 0x90(%rsp)
1422a7fdd:     	movaps	%xmm11, 0xa0(%rsp)
1422a7fe6:     	movaps	%xmm10, 0xb0(%rsp)
1422a7fef:     	movaps	%xmm9, 0xc0(%rsp)
1422a7ff8:     	movaps	%xmm8, 0xd0(%rsp)
1422a8001:     	movaps	%xmm7, 0xe0(%rsp)
1422a8009:     	movaps	%xmm6, 0xf0(%rsp)
1422a8011:     	movl	$0xe, %edx
1422a8016:     	movl	$0x1, %r8d
1422a801c:     	movq	0x150(%rbp), %rcx
1422a8023:     	callq	0x140613c20 <.text+0x612c20>
1422a8028:     	movaps	0xf0(%rsp), %xmm6
1422a8030:     	movaps	0xe0(%rsp), %xmm7
1422a8038:     	movaps	0xd0(%rsp), %xmm8
1422a8041:     	movaps	0xc0(%rsp), %xmm9
1422a804a:     	movaps	0xb0(%rsp), %xmm10
1422a8053:     	movaps	0xa0(%rsp), %xmm11
1422a805c:     	movaps	0x90(%rsp), %xmm12
1422a8065:     	movaps	0x80(%rsp), %xmm13
1422a806e:     	movaps	0x70(%rsp), %xmm14
1422a8074:     	movaps	0x60(%rsp), %xmm15
1422a807a:     	addq	$0x108, %rsp            # imm = 0x108
1422a8081:     	popq	%rbx
1422a8082:     	popq	%rdi
1422a8083:     	popq	%rsi
1422a8084:     	popq	%r12
1422a8086:     	popq	%r13
1422a8088:     	popq	%r14
1422a808a:     	popq	%r15
1422a808c:     	popq	%rbp
1422a808d:     	retq
1422a808e:     	nop
1422a8090:     	movq	%rdx, 0x10(%rsp)
1422a8095:     	pushq	%rbp
1422a8096:     	pushq	%r15
1422a8098:     	pushq	%r14
1422a809a:     	pushq	%r13
1422a809c:     	pushq	%r12
1422a809e:     	pushq	%rsi
1422a809f:     	pushq	%rdi
1422a80a0:     	pushq	%rbx
1422a80a1:     	subq	$0x108, %rsp            # imm = 0x108
1422a80a8:     	leaq	0x80(%rdx), %rbp
1422a80af:     	movaps	%xmm15, 0x60(%rsp)
1422a80b5:     	movaps	%xmm14, 0x70(%rsp)
1422a80bb:     	movaps	%xmm13, 0x80(%rsp)
1422a80c4:     	movaps	%xmm12, 0x90(%rsp)
1422a80cd:     	movaps	%xmm11, 0xa0(%rsp)
1422a80d6:     	movaps	%xmm10, 0xb0(%rsp)
1422a80df:     	movaps	%xmm9, 0xc0(%rsp)
1422a80e8:     	movaps	%xmm8, 0xd0(%rsp)
1422a80f1:     	movaps	%xmm7, 0xe0(%rsp)
1422a80f9:     	movaps	%xmm6, 0xf0(%rsp)
1422a8101:     	movl	$0x18, %edx
1422a8106:     	movl	$0x1, %r8d
1422a810c:     	movq	0x200(%rbp), %rcx
1422a8113:     	callq	0x140613c20 <.text+0x612c20>
1422a8118:     	movaps	0xf0(%rsp), %xmm6
1422a8120:     	movaps	0xe0(%rsp), %xmm7
1422a8128:     	movaps	0xd0(%rsp), %xmm8
1422a8131:     	movaps	0xc0(%rsp), %xmm9
1422a813a:     	movaps	0xb0(%rsp), %xmm10
1422a8143:     	movaps	0xa0(%rsp), %xmm11
1422a814c:     	movaps	0x90(%rsp), %xmm12
1422a8155:     	movaps	0x80(%rsp), %xmm13
1422a815e:     	movaps	0x70(%rsp), %xmm14
1422a8164:     	movaps	0x60(%rsp), %xmm15
1422a816a:     	addq	$0x108, %rsp            # imm = 0x108
1422a8171:     	popq	%rbx
1422a8172:     	popq	%rdi
1422a8173:     	popq	%rsi
1422a8174:     	popq	%r12
1422a8176:     	popq	%r13
1422a8178:     	popq	%r14
1422a817a:     	popq	%r15
1422a817c:     	popq	%rbp
1422a817d:     	retq
1422a817e:     	nop
1422a8180:     	movq	%rdx, 0x10(%rsp)
1422a8185:     	pushq	%rbp
1422a8186:     	pushq	%r15
1422a8188:     	pushq	%r14
1422a818a:     	pushq	%r13
1422a818c:     	pushq	%r12
1422a818e:     	pushq	%rsi
1422a818f:     	pushq	%rdi
1422a8190:     	pushq	%rbx
1422a8191:     	subq	$0x108, %rsp            # imm = 0x108
1422a8198:     	leaq	0x80(%rdx), %rbp
1422a819f:     	movaps	%xmm15, 0x60(%rsp)
1422a81a5:     	movaps	%xmm14, 0x70(%rsp)
1422a81ab:     	movaps	%xmm13, 0x80(%rsp)
1422a81b4:     	movaps	%xmm12, 0x90(%rsp)
1422a81bd:     	movaps	%xmm11, 0xa0(%rsp)
1422a81c6:     	movaps	%xmm10, 0xb0(%rsp)
1422a81cf:     	movaps	%xmm9, 0xc0(%rsp)
1422a81d8:     	movaps	%xmm8, 0xd0(%rsp)
1422a81e1:     	movaps	%xmm7, 0xe0(%rsp)
1422a81e9:     	movaps	%xmm6, 0xf0(%rsp)
1422a81f1:     	movl	$0x1a, %edx
1422a81f6:     	movl	$0x1, %r8d
1422a81fc:     	movq	0x178(%rbp), %rcx
1422a8203:     	callq	0x140613c20 <.text+0x612c20>
1422a8208:     	movaps	0xf0(%rsp), %xmm6
1422a8210:     	movaps	0xe0(%rsp), %xmm7
1422a8218:     	movaps	0xd0(%rsp), %xmm8
1422a8221:     	movaps	0xc0(%rsp), %xmm9
1422a822a:     	movaps	0xb0(%rsp), %xmm10
1422a8233:     	movaps	0xa0(%rsp), %xmm11
1422a823c:     	movaps	0x90(%rsp), %xmm12
1422a8245:     	movaps	0x80(%rsp), %xmm13
1422a824e:     	movaps	0x70(%rsp), %xmm14
1422a8254:     	movaps	0x60(%rsp), %xmm15
1422a825a:     	addq	$0x108, %rsp            # imm = 0x108
1422a8261:     	popq	%rbx
1422a8262:     	popq	%rdi
1422a8263:     	popq	%rsi
1422a8264:     	popq	%r12
1422a8266:     	popq	%r13
1422a8268:     	popq	%r14
1422a826a:     	popq	%r15
1422a826c:     	popq	%rbp
1422a826d:     	retq
1422a826e:     	nop
1422a8270:     	movq	%rdx, 0x10(%rsp)
1422a8275:     	pushq	%rbp
1422a8276:     	pushq	%r15
1422a8278:     	pushq	%r14
1422a827a:     	pushq	%r13
1422a827c:     	pushq	%r12
1422a827e:     	pushq	%rsi
1422a827f:     	pushq	%rdi
1422a8280:     	pushq	%rbx
1422a8281:     	subq	$0x108, %rsp            # imm = 0x108
1422a8288:     	leaq	0x80(%rdx), %rbp
1422a828f:     	movaps	%xmm15, 0x60(%rsp)
1422a8295:     	movaps	%xmm14, 0x70(%rsp)
1422a829b:     	movaps	%xmm13, 0x80(%rsp)
1422a82a4:     	movaps	%xmm12, 0x90(%rsp)
1422a82ad:     	movaps	%xmm11, 0xa0(%rsp)
1422a82b6:     	movaps	%xmm10, 0xb0(%rsp)
1422a82bf:     	movaps	%xmm9, 0xc0(%rsp)
1422a82c8:     	movaps	%xmm8, 0xd0(%rsp)
1422a82d1:     	movaps	%xmm7, 0xe0(%rsp)
1422a82d9:     	movaps	%xmm6, 0xf0(%rsp)
1422a82e1:     	movl	$0x11, %edx
1422a82e6:     	movl	$0x1, %r8d
1422a82ec:     	movq	0x150(%rbp), %rcx
1422a82f3:     	callq	0x140613c20 <.text+0x612c20>
1422a82f8:     	movaps	0xf0(%rsp), %xmm6
1422a8300:     	movaps	0xe0(%rsp), %xmm7
1422a8308:     	movaps	0xd0(%rsp), %xmm8
1422a8311:     	movaps	0xc0(%rsp), %xmm9
1422a831a:     	movaps	0xb0(%rsp), %xmm10
1422a8323:     	movaps	0xa0(%rsp), %xmm11
1422a832c:     	movaps	0x90(%rsp), %xmm12
1422a8335:     	movaps	0x80(%rsp), %xmm13
1422a833e:     	movaps	0x70(%rsp), %xmm14
1422a8344:     	movaps	0x60(%rsp), %xmm15
1422a834a:     	addq	$0x108, %rsp            # imm = 0x108
1422a8351:     	popq	%rbx
1422a8352:     	popq	%rdi
1422a8353:     	popq	%rsi
1422a8354:     	popq	%r12
1422a8356:     	popq	%r13
1422a8358:     	popq	%r14
1422a835a:     	popq	%r15
1422a835c:     	popq	%rbp
1422a835d:     	retq
1422a835e:     	nop
1422a8360:     	movq	%rdx, 0x10(%rsp)
1422a8365:     	pushq	%rbp
1422a8366:     	pushq	%r15
1422a8368:     	pushq	%r14
1422a836a:     	pushq	%r13
1422a836c:     	pushq	%r12
1422a836e:     	pushq	%rsi
1422a836f:     	pushq	%rdi
1422a8370:     	pushq	%rbx
1422a8371:     	subq	$0x108, %rsp            # imm = 0x108
1422a8378:     	leaq	0x80(%rdx), %rbp
1422a837f:     	movaps	%xmm15, 0x60(%rsp)
1422a8385:     	movaps	%xmm14, 0x70(%rsp)
1422a838b:     	movaps	%xmm13, 0x80(%rsp)
1422a8394:     	movaps	%xmm12, 0x90(%rsp)
1422a839d:     	movaps	%xmm11, 0xa0(%rsp)
1422a83a6:     	movaps	%xmm10, 0xb0(%rsp)
1422a83af:     	movaps	%xmm9, 0xc0(%rsp)
1422a83b8:     	movaps	%xmm8, 0xd0(%rsp)
1422a83c1:     	movaps	%xmm7, 0xe0(%rsp)
1422a83c9:     	movaps	%xmm6, 0xf0(%rsp)
1422a83d1:     	movl	$0x17, %edx
1422a83d6:     	movl	$0x1, %r8d
1422a83dc:     	movq	0x200(%rbp), %rcx
1422a83e3:     	callq	0x140613c20 <.text+0x612c20>
1422a83e8:     	movaps	0xf0(%rsp), %xmm6
1422a83f0:     	movaps	0xe0(%rsp), %xmm7
1422a83f8:     	movaps	0xd0(%rsp), %xmm8
1422a8401:     	movaps	0xc0(%rsp), %xmm9
1422a840a:     	movaps	0xb0(%rsp), %xmm10
1422a8413:     	movaps	0xa0(%rsp), %xmm11
1422a841c:     	movaps	0x90(%rsp), %xmm12
1422a8425:     	movaps	0x80(%rsp), %xmm13
1422a842e:     	movaps	0x70(%rsp), %xmm14
1422a8434:     	movaps	0x60(%rsp), %xmm15
1422a843a:     	addq	$0x108, %rsp            # imm = 0x108
1422a8441:     	popq	%rbx
1422a8442:     	popq	%rdi
1422a8443:     	popq	%rsi
1422a8444:     	popq	%r12
1422a8446:     	popq	%r13
1422a8448:     	popq	%r14
1422a844a:     	popq	%r15
1422a844c:     	popq	%rbp
1422a844d:     	retq
1422a844e:     	nop
1422a8450:     	movq	%rdx, 0x10(%rsp)
1422a8455:     	pushq	%rbp
1422a8456:     	pushq	%r15
1422a8458:     	pushq	%r14
1422a845a:     	pushq	%r13
1422a845c:     	pushq	%r12
1422a845e:     	pushq	%rsi
1422a845f:     	pushq	%rdi
1422a8460:     	pushq	%rbx
1422a8461:     	subq	$0x108, %rsp            # imm = 0x108
1422a8468:     	leaq	0x80(%rdx), %rbp
1422a846f:     	movaps	%xmm15, 0x60(%rsp)
1422a8475:     	movaps	%xmm14, 0x70(%rsp)
1422a847b:     	movaps	%xmm13, 0x80(%rsp)
1422a8484:     	movaps	%xmm12, 0x90(%rsp)
1422a848d:     	movaps	%xmm11, 0xa0(%rsp)
1422a8496:     	movaps	%xmm10, 0xb0(%rsp)
1422a849f:     	movaps	%xmm9, 0xc0(%rsp)
1422a84a8:     	movaps	%xmm8, 0xd0(%rsp)
1422a84b1:     	movaps	%xmm7, 0xe0(%rsp)
1422a84b9:     	movaps	%xmm6, 0xf0(%rsp)
1422a84c1:     	movl	$0x11, %edx
1422a84c6:     	movl	$0x1, %r8d
1422a84cc:     	movq	0x1c0(%rbp), %rcx
1422a84d3:     	callq	0x140613c20 <.text+0x612c20>
1422a84d8:     	movaps	0xf0(%rsp), %xmm6
1422a84e0:     	movaps	0xe0(%rsp), %xmm7
1422a84e8:     	movaps	0xd0(%rsp), %xmm8
1422a84f1:     	movaps	0xc0(%rsp), %xmm9
1422a84fa:     	movaps	0xb0(%rsp), %xmm10
1422a8503:     	movaps	0xa0(%rsp), %xmm11
1422a850c:     	movaps	0x90(%rsp), %xmm12
1422a8515:     	movaps	0x80(%rsp), %xmm13
1422a851e:     	movaps	0x70(%rsp), %xmm14
1422a8524:     	movaps	0x60(%rsp), %xmm15
1422a852a:     	addq	$0x108, %rsp            # imm = 0x108
1422a8531:     	popq	%rbx
1422a8532:     	popq	%rdi
1422a8533:     	popq	%rsi
1422a8534:     	popq	%r12
1422a8536:     	popq	%r13
1422a8538:     	popq	%r14
1422a853a:     	popq	%r15
1422a853c:     	popq	%rbp
1422a853d:     	retq
1422a853e:     	nop
1422a8540:     	movq	%rdx, 0x10(%rsp)
1422a8545:     	pushq	%rbp
1422a8546:     	pushq	%r15
1422a8548:     	pushq	%r14
1422a854a:     	pushq	%r13
1422a854c:     	pushq	%r12
1422a854e:     	pushq	%rsi
1422a854f:     	pushq	%rdi
1422a8550:     	pushq	%rbx
1422a8551:     	subq	$0x108, %rsp            # imm = 0x108
1422a8558:     	leaq	0x80(%rdx), %rbp
1422a855f:     	movaps	%xmm15, 0x60(%rsp)
1422a8565:     	movaps	%xmm14, 0x70(%rsp)
1422a856b:     	movaps	%xmm13, 0x80(%rsp)
1422a8574:     	movaps	%xmm12, 0x90(%rsp)
1422a857d:     	movaps	%xmm11, 0xa0(%rsp)
1422a8586:     	movaps	%xmm10, 0xb0(%rsp)
1422a858f:     	movaps	%xmm9, 0xc0(%rsp)
1422a8598:     	movaps	%xmm8, 0xd0(%rsp)
1422a85a1:     	movaps	%xmm7, 0xe0(%rsp)
1422a85a9:     	movaps	%xmm6, 0xf0(%rsp)
1422a85b1:     	movl	$0x1d, %edx
1422a85b6:     	movl	$0x1, %r8d
1422a85bc:     	movq	0x150(%rbp), %rcx
1422a85c3:     	callq	0x140613c20 <.text+0x612c20>
1422a85c8:     	movaps	0xf0(%rsp), %xmm6
1422a85d0:     	movaps	0xe0(%rsp), %xmm7
1422a85d8:     	movaps	0xd0(%rsp), %xmm8
1422a85e1:     	movaps	0xc0(%rsp), %xmm9
1422a85ea:     	movaps	0xb0(%rsp), %xmm10
1422a85f3:     	movaps	0xa0(%rsp), %xmm11
1422a85fc:     	movaps	0x90(%rsp), %xmm12
1422a8605:     	movaps	0x80(%rsp), %xmm13
1422a860e:     	movaps	0x70(%rsp), %xmm14
1422a8614:     	movaps	0x60(%rsp), %xmm15
1422a861a:     	addq	$0x108, %rsp            # imm = 0x108
1422a8621:     	popq	%rbx
1422a8622:     	popq	%rdi
1422a8623:     	popq	%rsi
1422a8624:     	popq	%r12
1422a8626:     	popq	%r13
1422a8628:     	popq	%r14
1422a862a:     	popq	%r15
1422a862c:     	popq	%rbp
1422a862d:     	retq
1422a862e:     	nop
1422a8630:     	movq	%rdx, 0x10(%rsp)
1422a8635:     	pushq	%rbp
1422a8636:     	pushq	%r15
1422a8638:     	pushq	%r14
1422a863a:     	pushq	%r13
1422a863c:     	pushq	%r12
1422a863e:     	pushq	%rsi
1422a863f:     	pushq	%rdi
1422a8640:     	pushq	%rbx
1422a8641:     	subq	$0x108, %rsp            # imm = 0x108
1422a8648:     	leaq	0x80(%rdx), %rbp
1422a864f:     	movaps	%xmm15, 0x60(%rsp)
1422a8655:     	movaps	%xmm14, 0x70(%rsp)
1422a865b:     	movaps	%xmm13, 0x80(%rsp)
1422a8664:     	movaps	%xmm12, 0x90(%rsp)
1422a866d:     	movaps	%xmm11, 0xa0(%rsp)
1422a8676:     	movaps	%xmm10, 0xb0(%rsp)
1422a867f:     	movaps	%xmm9, 0xc0(%rsp)
1422a8688:     	movaps	%xmm8, 0xd0(%rsp)
1422a8691:     	movaps	%xmm7, 0xe0(%rsp)
1422a8699:     	movaps	%xmm6, 0xf0(%rsp)
1422a86a1:     	movl	$0x12, %edx
1422a86a6:     	movl	$0x1, %r8d
1422a86ac:     	movq	0x1d8(%rbp), %rcx
1422a86b3:     	callq	0x140613c20 <.text+0x612c20>
1422a86b8:     	movaps	0xf0(%rsp), %xmm6
1422a86c0:     	movaps	0xe0(%rsp), %xmm7
1422a86c8:     	movaps	0xd0(%rsp), %xmm8
1422a86d1:     	movaps	0xc0(%rsp), %xmm9
1422a86da:     	movaps	0xb0(%rsp), %xmm10
1422a86e3:     	movaps	0xa0(%rsp), %xmm11
1422a86ec:     	movaps	0x90(%rsp), %xmm12
1422a86f5:     	movaps	0x80(%rsp), %xmm13
1422a86fe:     	movaps	0x70(%rsp), %xmm14
1422a8704:     	movaps	0x60(%rsp), %xmm15
1422a870a:     	addq	$0x108, %rsp            # imm = 0x108
1422a8711:     	popq	%rbx
1422a8712:     	popq	%rdi
1422a8713:     	popq	%rsi
1422a8714:     	popq	%r12
1422a8716:     	popq	%r13
1422a8718:     	popq	%r14
1422a871a:     	popq	%r15
1422a871c:     	popq	%rbp
1422a871d:     	retq
1422a871e:     	nop
1422a8720:     	movq	%rdx, 0x10(%rsp)
1422a8725:     	pushq	%rbp
1422a8726:     	pushq	%r15
1422a8728:     	pushq	%r14
1422a872a:     	pushq	%r13
1422a872c:     	pushq	%r12
1422a872e:     	pushq	%rsi
1422a872f:     	pushq	%rdi
1422a8730:     	pushq	%rbx
1422a8731:     	subq	$0x108, %rsp            # imm = 0x108
1422a8738:     	leaq	0x80(%rdx), %rbp
1422a873f:     	movaps	%xmm15, 0x60(%rsp)
1422a8745:     	movaps	%xmm14, 0x70(%rsp)
1422a874b:     	movaps	%xmm13, 0x80(%rsp)
1422a8754:     	movaps	%xmm12, 0x90(%rsp)
1422a875d:     	movaps	%xmm11, 0xa0(%rsp)
1422a8766:     	movaps	%xmm10, 0xb0(%rsp)
1422a876f:     	movaps	%xmm9, 0xc0(%rsp)
1422a8778:     	movaps	%xmm8, 0xd0(%rsp)
1422a8781:     	movaps	%xmm7, 0xe0(%rsp)
1422a8789:     	movaps	%xmm6, 0xf0(%rsp)
1422a8791:     	movl	$0x14, %edx
1422a8796:     	movl	$0x1, %r8d
1422a879c:     	movq	0x1d8(%rbp), %rcx
1422a87a3:     	callq	0x140613c20 <.text+0x612c20>
1422a87a8:     	movaps	0xf0(%rsp), %xmm6
1422a87b0:     	movaps	0xe0(%rsp), %xmm7
1422a87b8:     	movaps	0xd0(%rsp), %xmm8
1422a87c1:     	movaps	0xc0(%rsp), %xmm9
1422a87ca:     	movaps	0xb0(%rsp), %xmm10
1422a87d3:     	movaps	0xa0(%rsp), %xmm11
1422a87dc:     	movaps	0x90(%rsp), %xmm12
1422a87e5:     	movaps	0x80(%rsp), %xmm13
1422a87ee:     	movaps	0x70(%rsp), %xmm14
1422a87f4:     	movaps	0x60(%rsp), %xmm15
1422a87fa:     	addq	$0x108, %rsp            # imm = 0x108
1422a8801:     	popq	%rbx
1422a8802:     	popq	%rdi
1422a8803:     	popq	%rsi
1422a8804:     	popq	%r12
1422a8806:     	popq	%r13
1422a8808:     	popq	%r14
1422a880a:     	popq	%r15
1422a880c:     	popq	%rbp
1422a880d:     	retq
1422a880e:     	nop
1422a8810:     	movq	%rdx, 0x10(%rsp)
1422a8815:     	pushq	%rbp
1422a8816:     	pushq	%r15
1422a8818:     	pushq	%r14
1422a881a:     	pushq	%r13
1422a881c:     	pushq	%r12
1422a881e:     	pushq	%rsi
1422a881f:     	pushq	%rdi
1422a8820:     	pushq	%rbx
1422a8821:     	subq	$0x108, %rsp            # imm = 0x108
1422a8828:     	leaq	0x80(%rdx), %rbp
1422a882f:     	movaps	%xmm15, 0x60(%rsp)
1422a8835:     	movaps	%xmm14, 0x70(%rsp)
1422a883b:     	movaps	%xmm13, 0x80(%rsp)
1422a8844:     	movaps	%xmm12, 0x90(%rsp)
1422a884d:     	movaps	%xmm11, 0xa0(%rsp)
1422a8856:     	movaps	%xmm10, 0xb0(%rsp)
1422a885f:     	movaps	%xmm9, 0xc0(%rsp)
1422a8868:     	movaps	%xmm8, 0xd0(%rsp)
1422a8871:     	movaps	%xmm7, 0xe0(%rsp)
1422a8879:     	movaps	%xmm6, 0xf0(%rsp)
1422a8881:     	movl	$0x5, %edx
1422a8886:     	movl	$0x1, %r8d
1422a888c:     	movq	0x1e8(%rbp), %rcx
1422a8893:     	callq	0x140613c20 <.text+0x612c20>
1422a8898:     	movaps	0xf0(%rsp), %xmm6
1422a88a0:     	movaps	0xe0(%rsp), %xmm7
1422a88a8:     	movaps	0xd0(%rsp), %xmm8
1422a88b1:     	movaps	0xc0(%rsp), %xmm9
1422a88ba:     	movaps	0xb0(%rsp), %xmm10
1422a88c3:     	movaps	0xa0(%rsp), %xmm11
1422a88cc:     	movaps	0x90(%rsp), %xmm12
1422a88d5:     	movaps	0x80(%rsp), %xmm13
1422a88de:     	movaps	0x70(%rsp), %xmm14
1422a88e4:     	movaps	0x60(%rsp), %xmm15
1422a88ea:     	addq	$0x108, %rsp            # imm = 0x108
1422a88f1:     	popq	%rbx
1422a88f2:     	popq	%rdi
1422a88f3:     	popq	%rsi
1422a88f4:     	popq	%r12
1422a88f6:     	popq	%r13
1422a88f8:     	popq	%r14
1422a88fa:     	popq	%r15
1422a88fc:     	popq	%rbp
1422a88fd:     	retq
1422a88fe:     	nop
1422a8900:     	movq	%rdx, 0x10(%rsp)
1422a8905:     	pushq	%rbp
1422a8906:     	pushq	%r15
1422a8908:     	pushq	%r14
1422a890a:     	pushq	%r13
1422a890c:     	pushq	%r12
1422a890e:     	pushq	%rsi
1422a890f:     	pushq	%rdi
1422a8910:     	pushq	%rbx
1422a8911:     	subq	$0x108, %rsp            # imm = 0x108
1422a8918:     	leaq	0x80(%rdx), %rbp
1422a891f:     	movaps	%xmm15, 0x60(%rsp)
1422a8925:     	movaps	%xmm14, 0x70(%rsp)
1422a892b:     	movaps	%xmm13, 0x80(%rsp)
1422a8934:     	movaps	%xmm12, 0x90(%rsp)
1422a893d:     	movaps	%xmm11, 0xa0(%rsp)
1422a8946:     	movaps	%xmm10, 0xb0(%rsp)
1422a894f:     	movaps	%xmm9, 0xc0(%rsp)
1422a8958:     	movaps	%xmm8, 0xd0(%rsp)
1422a8961:     	movaps	%xmm7, 0xe0(%rsp)
1422a8969:     	movaps	%xmm6, 0xf0(%rsp)
1422a8971:     	movl	$0xf0, %edx
1422a8976:     	movl	$0x8, %r8d
1422a897c:     	movq	0x1f0(%rbp), %rcx
1422a8983:     	callq	0x140613c20 <.text+0x612c20>
1422a8988:     	movaps	0xf0(%rsp), %xmm6
1422a8990:     	movaps	0xe0(%rsp), %xmm7
1422a8998:     	movaps	0xd0(%rsp), %xmm8
1422a89a1:     	movaps	0xc0(%rsp), %xmm9
1422a89aa:     	movaps	0xb0(%rsp), %xmm10
1422a89b3:     	movaps	0xa0(%rsp), %xmm11
1422a89bc:     	movaps	0x90(%rsp), %xmm12
1422a89c5:     	movaps	0x80(%rsp), %xmm13
1422a89ce:     	movaps	0x70(%rsp), %xmm14
1422a89d4:     	movaps	0x60(%rsp), %xmm15
1422a89da:     	addq	$0x108, %rsp            # imm = 0x108
1422a89e1:     	popq	%rbx
1422a89e2:     	popq	%rdi
1422a89e3:     	popq	%rsi
1422a89e4:     	popq	%r12
1422a89e6:     	popq	%r13
1422a89e8:     	popq	%r14
1422a89ea:     	popq	%r15
1422a89ec:     	popq	%rbp
1422a89ed:     	retq
1422a89ee:     	nop
1422a89f0:     	movq	%rdx, 0x10(%rsp)
1422a89f5:     	pushq	%rbp
1422a89f6:     	pushq	%r15
1422a89f8:     	pushq	%r14
1422a89fa:     	pushq	%r13
1422a89fc:     	pushq	%r12
1422a89fe:     	pushq	%rsi
1422a89ff:     	pushq	%rdi
1422a8a00:     	pushq	%rbx
1422a8a01:     	subq	$0x108, %rsp            # imm = 0x108
1422a8a08:     	leaq	0x80(%rdx), %rbp
1422a8a0f:     	movaps	%xmm15, 0x60(%rsp)
1422a8a15:     	movaps	%xmm14, 0x70(%rsp)
1422a8a1b:     	movaps	%xmm13, 0x80(%rsp)
1422a8a24:     	movaps	%xmm12, 0x90(%rsp)
1422a8a2d:     	movaps	%xmm11, 0xa0(%rsp)
1422a8a36:     	movaps	%xmm10, 0xb0(%rsp)
1422a8a3f:     	movaps	%xmm9, 0xc0(%rsp)
1422a8a48:     	movaps	%xmm8, 0xd0(%rsp)
1422a8a51:     	movaps	%xmm7, 0xe0(%rsp)
1422a8a59:     	movaps	%xmm6, 0xf0(%rsp)
1422a8a61:     	cmpb	$0x0, 0x1af(%rbp)
1422a8a68:     	je	0x1422a8adc <.text+0x22a7adc>
1422a8a6a:     	leaq	0x158(%rbp), %rcx
1422a8a71:     	callq	0x14014a860 <.text+0x149860>
1422a8a76:     	movaps	0xf0(%rsp), %xmm6
1422a8a7e:     	movaps	0xe0(%rsp), %xmm7
1422a8a86:     	movaps	0xd0(%rsp), %xmm8
1422a8a8f:     	movaps	0xc0(%rsp), %xmm9
1422a8a98:     	movaps	0xb0(%rsp), %xmm10
1422a8aa1:     	movaps	0xa0(%rsp), %xmm11
1422a8aaa:     	movaps	0x90(%rsp), %xmm12
1422a8ab3:     	movaps	0x80(%rsp), %xmm13
1422a8abc:     	movaps	0x70(%rsp), %xmm14
1422a8ac2:     	movaps	0x60(%rsp), %xmm15
1422a8ac8:     	addq	$0x108, %rsp            # imm = 0x108
1422a8acf:     	popq	%rbx
1422a8ad0:     	popq	%rdi
1422a8ad1:     	popq	%rsi
1422a8ad2:     	popq	%r12
1422a8ad4:     	popq	%r13
1422a8ad6:     	popq	%r14
1422a8ad8:     	popq	%r15
1422a8ada:     	popq	%rbp
1422a8adb:     	retq
1422a8adc:     	movaps	0xf0(%rsp), %xmm6
1422a8ae4:     	movaps	0xe0(%rsp), %xmm7
1422a8aec:     	movaps	0xd0(%rsp), %xmm8
1422a8af5:     	movaps	0xc0(%rsp), %xmm9
1422a8afe:     	movaps	0xb0(%rsp), %xmm10
1422a8b07:     	movaps	0xa0(%rsp), %xmm11
1422a8b10:     	movaps	0x90(%rsp), %xmm12
1422a8b19:     	movaps	0x80(%rsp), %xmm13
1422a8b22:     	movaps	0x70(%rsp), %xmm14
1422a8b28:     	movaps	0x60(%rsp), %xmm15
1422a8b2e:     	addq	$0x108, %rsp            # imm = 0x108
1422a8b35:     	popq	%rbx
1422a8b36:     	popq	%rdi
1422a8b37:     	popq	%rsi
1422a8b38:     	popq	%r12
1422a8b3a:     	popq	%r13
1422a8b3c:     	popq	%r14
1422a8b3e:     	popq	%r15
1422a8b40:     	popq	%rbp
1422a8b41:     	retq
1422a8b42:     	nopw	%cs:(%rax,%rax)
1422a8b50:     	movq	%rdx, 0x10(%rsp)
1422a8b55:     	pushq	%rbp
1422a8b56:     	pushq	%r15
1422a8b58:     	pushq	%r14
1422a8b5a:     	pushq	%r13
1422a8b5c:     	pushq	%r12
1422a8b5e:     	pushq	%rsi
1422a8b5f:     	pushq	%rdi
1422a8b60:     	pushq	%rbx
1422a8b61:     	subq	$0x108, %rsp            # imm = 0x108
1422a8b68:     	leaq	0x80(%rdx), %rbp
1422a8b6f:     	movaps	%xmm15, 0x60(%rsp)
1422a8b75:     	movaps	%xmm14, 0x70(%rsp)
1422a8b7b:     	movaps	%xmm13, 0x80(%rsp)
1422a8b84:     	movaps	%xmm12, 0x90(%rsp)
1422a8b8d:     	movaps	%xmm11, 0xa0(%rsp)
1422a8b96:     	movaps	%xmm10, 0xb0(%rsp)
1422a8b9f:     	movaps	%xmm9, 0xc0(%rsp)
1422a8ba8:     	movaps	%xmm8, 0xd0(%rsp)
1422a8bb1:     	movaps	%xmm7, 0xe0(%rsp)
1422a8bb9:     	movaps	%xmm6, 0xf0(%rsp)
1422a8bc1:     	cmpb	$0x0, 0x1ae(%rbp)
1422a8bc8:     	je	0x1422a8c3c <.text+0x22a7c3c>
1422a8bca:     	leaq	0xb0(%rbp), %rcx
1422a8bd1:     	callq	0x14014a860 <.text+0x149860>
1422a8bd6:     	movaps	0xf0(%rsp), %xmm6
1422a8bde:     	movaps	0xe0(%rsp), %xmm7
1422a8be6:     	movaps	0xd0(%rsp), %xmm8
1422a8bef:     	movaps	0xc0(%rsp), %xmm9
1422a8bf8:     	movaps	0xb0(%rsp), %xmm10
1422a8c01:     	movaps	0xa0(%rsp), %xmm11
1422a8c0a:     	movaps	0x90(%rsp), %xmm12
1422a8c13:     	movaps	0x80(%rsp), %xmm13
1422a8c1c:     	movaps	0x70(%rsp), %xmm14
1422a8c22:     	movaps	0x60(%rsp), %xmm15
1422a8c28:     	addq	$0x108, %rsp            # imm = 0x108
1422a8c2f:     	popq	%rbx
1422a8c30:     	popq	%rdi
1422a8c31:     	popq	%rsi
1422a8c32:     	popq	%r12
1422a8c34:     	popq	%r13
1422a8c36:     	popq	%r14
1422a8c38:     	popq	%r15
1422a8c3a:     	popq	%rbp
1422a8c3b:     	retq
1422a8c3c:     	movaps	0xf0(%rsp), %xmm6
1422a8c44:     	movaps	0xe0(%rsp), %xmm7
1422a8c4c:     	movaps	0xd0(%rsp), %xmm8
1422a8c55:     	movaps	0xc0(%rsp), %xmm9
1422a8c5e:     	movaps	0xb0(%rsp), %xmm10
1422a8c67:     	movaps	0xa0(%rsp), %xmm11
1422a8c70:     	movaps	0x90(%rsp), %xmm12
1422a8c79:     	movaps	0x80(%rsp), %xmm13
1422a8c82:     	movaps	0x70(%rsp), %xmm14
1422a8c88:     	movaps	0x60(%rsp), %xmm15
1422a8c8e:     	addq	$0x108, %rsp            # imm = 0x108
1422a8c95:     	popq	%rbx
1422a8c96:     	popq	%rdi
1422a8c97:     	popq	%rsi
1422a8c98:     	popq	%r12
1422a8c9a:     	popq	%r13
1422a8c9c:     	popq	%r14
1422a8c9e:     	popq	%r15
1422a8ca0:     	popq	%rbp
1422a8ca1:     	retq
1422a8ca2:     	nopw	%cs:(%rax,%rax)
1422a8cb0:     	movq	%rdx, 0x10(%rsp)
1422a8cb5:     	pushq	%rbp
1422a8cb6:     	pushq	%r15
1422a8cb8:     	pushq	%r14
1422a8cba:     	pushq	%r13
1422a8cbc:     	pushq	%r12
1422a8cbe:     	pushq	%rsi
1422a8cbf:     	pushq	%rdi
1422a8cc0:     	pushq	%rbx
1422a8cc1:     	subq	$0x108, %rsp            # imm = 0x108
1422a8cc8:     	leaq	0x80(%rdx), %rbp
1422a8ccf:     	movaps	%xmm15, 0x60(%rsp)
1422a8cd5:     	movaps	%xmm14, 0x70(%rsp)
1422a8cdb:     	movaps	%xmm13, 0x80(%rsp)
1422a8ce4:     	movaps	%xmm12, 0x90(%rsp)
1422a8ced:     	movaps	%xmm11, 0xa0(%rsp)
1422a8cf6:     	movaps	%xmm10, 0xb0(%rsp)
1422a8cff:     	movaps	%xmm9, 0xc0(%rsp)
1422a8d08:     	movaps	%xmm8, 0xd0(%rsp)
1422a8d11:     	movaps	%xmm7, 0xe0(%rsp)
1422a8d19:     	movaps	%xmm6, 0xf0(%rsp)
1422a8d21:     	movl	$0x19, %edx
1422a8d26:     	movl	$0x1, %r8d
1422a8d2c:     	movq	0x200(%rbp), %rcx
1422a8d33:     	callq	0x140613c20 <.text+0x612c20>
1422a8d38:     	movaps	0xf0(%rsp), %xmm6
1422a8d40:     	movaps	0xe0(%rsp), %xmm7
1422a8d48:     	movaps	0xd0(%rsp), %xmm8
1422a8d51:     	movaps	0xc0(%rsp), %xmm9
1422a8d5a:     	movaps	0xb0(%rsp), %xmm10
1422a8d63:     	movaps	0xa0(%rsp), %xmm11
1422a8d6c:     	movaps	0x90(%rsp), %xmm12
1422a8d75:     	movaps	0x80(%rsp), %xmm13
1422a8d7e:     	movaps	0x70(%rsp), %xmm14
1422a8d84:     	movaps	0x60(%rsp), %xmm15
1422a8d8a:     	addq	$0x108, %rsp            # imm = 0x108
1422a8d91:     	popq	%rbx
1422a8d92:     	popq	%rdi
1422a8d93:     	popq	%rsi
1422a8d94:     	popq	%r12
1422a8d96:     	popq	%r13
1422a8d98:     	popq	%r14
1422a8d9a:     	popq	%r15
1422a8d9c:     	popq	%rbp
1422a8d9d:     	retq
1422a8d9e:     	nop
1422a8da0:     	movq	%rdx, 0x10(%rsp)
1422a8da5:     	pushq	%rbp
1422a8da6:     	pushq	%r15
1422a8da8:     	pushq	%r14
1422a8daa:     	pushq	%r13
1422a8dac:     	pushq	%r12
1422a8dae:     	pushq	%rsi
1422a8daf:     	pushq	%rdi
1422a8db0:     	pushq	%rbx
1422a8db1:     	subq	$0x108, %rsp            # imm = 0x108
1422a8db8:     	leaq	0x80(%rdx), %rbp
1422a8dbf:     	movaps	%xmm15, 0x60(%rsp)
1422a8dc5:     	movaps	%xmm14, 0x70(%rsp)
1422a8dcb:     	movaps	%xmm13, 0x80(%rsp)
1422a8dd4:     	movaps	%xmm12, 0x90(%rsp)
1422a8ddd:     	movaps	%xmm11, 0xa0(%rsp)
1422a8de6:     	movaps	%xmm10, 0xb0(%rsp)
1422a8def:     	movaps	%xmm9, 0xc0(%rsp)
1422a8df8:     	movaps	%xmm8, 0xd0(%rsp)
1422a8e01:     	movaps	%xmm7, 0xe0(%rsp)
1422a8e09:     	movaps	%xmm6, 0xf0(%rsp)
1422a8e11:     	movl	$0x16, %edx
1422a8e16:     	movl	$0x1, %r8d
1422a8e1c:     	movq	0x1d8(%rbp), %rcx
1422a8e23:     	callq	0x140613c20 <.text+0x612c20>
1422a8e28:     	movaps	0xf0(%rsp), %xmm6
1422a8e30:     	movaps	0xe0(%rsp), %xmm7
1422a8e38:     	movaps	0xd0(%rsp), %xmm8
1422a8e41:     	movaps	0xc0(%rsp), %xmm9
1422a8e4a:     	movaps	0xb0(%rsp), %xmm10
1422a8e53:     	movaps	0xa0(%rsp), %xmm11
1422a8e5c:     	movaps	0x90(%rsp), %xmm12
1422a8e65:     	movaps	0x80(%rsp), %xmm13
1422a8e6e:     	movaps	0x70(%rsp), %xmm14
1422a8e74:     	movaps	0x60(%rsp), %xmm15
1422a8e7a:     	addq	$0x108, %rsp            # imm = 0x108
1422a8e81:     	popq	%rbx
1422a8e82:     	popq	%rdi
1422a8e83:     	popq	%rsi
1422a8e84:     	popq	%r12
1422a8e86:     	popq	%r13
1422a8e88:     	popq	%r14
1422a8e8a:     	popq	%r15
1422a8e8c:     	popq	%rbp
1422a8e8d:     	retq
1422a8e8e:     	nop
1422a8e90:     	movq	%rdx, 0x10(%rsp)
1422a8e95:     	pushq	%rbp
1422a8e96:     	pushq	%r15
1422a8e98:     	pushq	%r14
1422a8e9a:     	pushq	%r13
1422a8e9c:     	pushq	%r12
1422a8e9e:     	pushq	%rsi
1422a8e9f:     	pushq	%rdi
1422a8ea0:     	pushq	%rbx
1422a8ea1:     	subq	$0x108, %rsp            # imm = 0x108
1422a8ea8:     	leaq	0x80(%rdx), %rbp
1422a8eaf:     	movaps	%xmm15, 0x60(%rsp)
1422a8eb5:     	movaps	%xmm14, 0x70(%rsp)
1422a8ebb:     	movaps	%xmm13, 0x80(%rsp)
1422a8ec4:     	movaps	%xmm12, 0x90(%rsp)
1422a8ecd:     	movaps	%xmm11, 0xa0(%rsp)
1422a8ed6:     	movaps	%xmm10, 0xb0(%rsp)
1422a8edf:     	movaps	%xmm9, 0xc0(%rsp)
1422a8ee8:     	movaps	%xmm8, 0xd0(%rsp)
1422a8ef1:     	movaps	%xmm7, 0xe0(%rsp)
1422a8ef9:     	movaps	%xmm6, 0xf0(%rsp)
1422a8f01:     	movl	$0xe, %edx
1422a8f06:     	movl	$0x1, %r8d
1422a8f0c:     	movq	0x1c0(%rbp), %rcx
1422a8f13:     	callq	0x140613c20 <.text+0x612c20>
1422a8f18:     	movaps	0xf0(%rsp), %xmm6
1422a8f20:     	movaps	0xe0(%rsp), %xmm7
1422a8f28:     	movaps	0xd0(%rsp), %xmm8
1422a8f31:     	movaps	0xc0(%rsp), %xmm9
1422a8f3a:     	movaps	0xb0(%rsp), %xmm10
1422a8f43:     	movaps	0xa0(%rsp), %xmm11
1422a8f4c:     	movaps	0x90(%rsp), %xmm12
1422a8f55:     	movaps	0x80(%rsp), %xmm13
1422a8f5e:     	movaps	0x70(%rsp), %xmm14
1422a8f64:     	movaps	0x60(%rsp), %xmm15
1422a8f6a:     	addq	$0x108, %rsp            # imm = 0x108
1422a8f71:     	popq	%rbx
1422a8f72:     	popq	%rdi
1422a8f73:     	popq	%rsi
1422a8f74:     	popq	%r12
1422a8f76:     	popq	%r13
1422a8f78:     	popq	%r14
1422a8f7a:     	popq	%r15
1422a8f7c:     	popq	%rbp
1422a8f7d:     	retq
1422a8f7e:     	nop
1422a8f80:     	movq	%rdx, 0x10(%rsp)
1422a8f85:     	pushq	%rbp
1422a8f86:     	pushq	%r15
1422a8f88:     	pushq	%r14
1422a8f8a:     	pushq	%r13
1422a8f8c:     	pushq	%r12
1422a8f8e:     	pushq	%rsi
1422a8f8f:     	pushq	%rdi
1422a8f90:     	pushq	%rbx
1422a8f91:     	subq	$0x108, %rsp            # imm = 0x108
1422a8f98:     	leaq	0x80(%rdx), %rbp
1422a8f9f:     	movaps	%xmm15, 0x60(%rsp)
1422a8fa5:     	movaps	%xmm14, 0x70(%rsp)
1422a8fab:     	movaps	%xmm13, 0x80(%rsp)
1422a8fb4:     	movaps	%xmm12, 0x90(%rsp)
1422a8fbd:     	movaps	%xmm11, 0xa0(%rsp)
1422a8fc6:     	movaps	%xmm10, 0xb0(%rsp)
1422a8fcf:     	movaps	%xmm9, 0xc0(%rsp)
1422a8fd8:     	movaps	%xmm8, 0xd0(%rsp)
1422a8fe1:     	movaps	%xmm7, 0xe0(%rsp)
1422a8fe9:     	movaps	%xmm6, 0xf0(%rsp)
1422a8ff1:     	movl	$0x11, %edx
1422a8ff6:     	movl	$0x1, %r8d
1422a8ffc:     	movq	0x1c0(%rbp), %rcx
1422a9003:     	callq	0x140613c20 <.text+0x612c20>
1422a9008:     	movaps	0xf0(%rsp), %xmm6
1422a9010:     	movaps	0xe0(%rsp), %xmm7
1422a9018:     	movaps	0xd0(%rsp), %xmm8
1422a9021:     	movaps	0xc0(%rsp), %xmm9
1422a902a:     	movaps	0xb0(%rsp), %xmm10
1422a9033:     	movaps	0xa0(%rsp), %xmm11
1422a903c:     	movaps	0x90(%rsp), %xmm12
1422a9045:     	movaps	0x80(%rsp), %xmm13
1422a904e:     	movaps	0x70(%rsp), %xmm14
1422a9054:     	movaps	0x60(%rsp), %xmm15
1422a905a:     	addq	$0x108, %rsp            # imm = 0x108
1422a9061:     	popq	%rbx
1422a9062:     	popq	%rdi
1422a9063:     	popq	%rsi
1422a9064:     	popq	%r12
1422a9066:     	popq	%r13
1422a9068:     	popq	%r14
1422a906a:     	popq	%r15
1422a906c:     	popq	%rbp
1422a906d:     	retq
1422a906e:     	nop
1422a9070:     	movq	%rdx, 0x10(%rsp)
1422a9075:     	pushq	%rbp
1422a9076:     	pushq	%r15
1422a9078:     	pushq	%r14
1422a907a:     	pushq	%r13
1422a907c:     	pushq	%r12
1422a907e:     	pushq	%rsi
1422a907f:     	pushq	%rdi
1422a9080:     	pushq	%rbx
1422a9081:     	subq	$0x108, %rsp            # imm = 0x108
1422a9088:     	leaq	0x80(%rdx), %rbp
1422a908f:     	movaps	%xmm15, 0x60(%rsp)
1422a9095:     	movaps	%xmm14, 0x70(%rsp)
1422a909b:     	movaps	%xmm13, 0x80(%rsp)
1422a90a4:     	movaps	%xmm12, 0x90(%rsp)
1422a90ad:     	movaps	%xmm11, 0xa0(%rsp)
1422a90b6:     	movaps	%xmm10, 0xb0(%rsp)
1422a90bf:     	movaps	%xmm9, 0xc0(%rsp)
1422a90c8:     	movaps	%xmm8, 0xd0(%rsp)
1422a90d1:     	movaps	%xmm7, 0xe0(%rsp)
1422a90d9:     	movaps	%xmm6, 0xf0(%rsp)
1422a90e1:     	movl	$0x14, %edx
1422a90e6:     	movl	$0x1, %r8d
1422a90ec:     	movq	0x1b0(%rbp), %rcx
1422a90f3:     	callq	0x140613c20 <.text+0x612c20>
1422a90f8:     	movaps	0xf0(%rsp), %xmm6
1422a9100:     	movaps	0xe0(%rsp), %xmm7
1422a9108:     	movaps	0xd0(%rsp), %xmm8
1422a9111:     	movaps	0xc0(%rsp), %xmm9
1422a911a:     	movaps	0xb0(%rsp), %xmm10
1422a9123:     	movaps	0xa0(%rsp), %xmm11
1422a912c:     	movaps	0x90(%rsp), %xmm12
1422a9135:     	movaps	0x80(%rsp), %xmm13
1422a913e:     	movaps	0x70(%rsp), %xmm14
1422a9144:     	movaps	0x60(%rsp), %xmm15
1422a914a:     	addq	$0x108, %rsp            # imm = 0x108
1422a9151:     	popq	%rbx
1422a9152:     	popq	%rdi
1422a9153:     	popq	%rsi
1422a9154:     	popq	%r12
1422a9156:     	popq	%r13
1422a9158:     	popq	%r14
1422a915a:     	popq	%r15
1422a915c:     	popq	%rbp
1422a915d:     	retq
1422a915e:     	nop
1422a9160:     	movq	%rdx, 0x10(%rsp)
1422a9165:     	pushq	%rbp
1422a9166:     	pushq	%r15
1422a9168:     	pushq	%r14
1422a916a:     	pushq	%r13
1422a916c:     	pushq	%r12
1422a916e:     	pushq	%rsi
1422a916f:     	pushq	%rdi
1422a9170:     	pushq	%rbx
1422a9171:     	subq	$0x108, %rsp            # imm = 0x108
1422a9178:     	leaq	0x80(%rdx), %rbp
1422a917f:     	movaps	%xmm15, 0x60(%rsp)
1422a9185:     	movaps	%xmm14, 0x70(%rsp)
1422a918b:     	movaps	%xmm13, 0x80(%rsp)
1422a9194:     	movaps	%xmm12, 0x90(%rsp)
1422a919d:     	movaps	%xmm11, 0xa0(%rsp)
1422a91a6:     	movaps	%xmm10, 0xb0(%rsp)
1422a91af:     	movaps	%xmm9, 0xc0(%rsp)
1422a91b8:     	movaps	%xmm8, 0xd0(%rsp)
1422a91c1:     	movaps	%xmm7, 0xe0(%rsp)
1422a91c9:     	movaps	%xmm6, 0xf0(%rsp)
1422a91d1:     	movl	$0x118, %edx            # imm = 0x118
1422a91d6:     	movl	$0x8, %r8d
1422a91dc:     	movq	0x1d8(%rbp), %rcx
1422a91e3:     	callq	0x140613c20 <.text+0x612c20>
1422a91e8:     	movaps	0xf0(%rsp), %xmm6
1422a91f0:     	movaps	0xe0(%rsp), %xmm7
1422a91f8:     	movaps	0xd0(%rsp), %xmm8
1422a9201:     	movaps	0xc0(%rsp), %xmm9
1422a920a:     	movaps	0xb0(%rsp), %xmm10
1422a9213:     	movaps	0xa0(%rsp), %xmm11
1422a921c:     	movaps	0x90(%rsp), %xmm12
1422a9225:     	movaps	0x80(%rsp), %xmm13
1422a922e:     	movaps	0x70(%rsp), %xmm14
1422a9234:     	movaps	0x60(%rsp), %xmm15
1422a923a:     	addq	$0x108, %rsp            # imm = 0x108
1422a9241:     	popq	%rbx
1422a9242:     	popq	%rdi
1422a9243:     	popq	%rsi
1422a9244:     	popq	%r12
1422a9246:     	popq	%r13
1422a9248:     	popq	%r14
1422a924a:     	popq	%r15
1422a924c:     	popq	%rbp
1422a924d:     	retq
1422a924e:     	nop
1422a9250:     	movq	%rdx, 0x10(%rsp)
1422a9255:     	pushq	%rbp
1422a9256:     	pushq	%r15
1422a9258:     	pushq	%r14
1422a925a:     	pushq	%r13
1422a925c:     	pushq	%r12
1422a925e:     	pushq	%rsi
1422a925f:     	pushq	%rdi
1422a9260:     	pushq	%rbx
1422a9261:     	subq	$0x108, %rsp            # imm = 0x108
1422a9268:     	leaq	0x80(%rdx), %rbp
1422a926f:     	movaps	%xmm15, 0x60(%rsp)
1422a9275:     	movaps	%xmm14, 0x70(%rsp)
1422a927b:     	movaps	%xmm13, 0x80(%rsp)
1422a9284:     	movaps	%xmm12, 0x90(%rsp)
1422a928d:     	movaps	%xmm11, 0xa0(%rsp)
1422a9296:     	movaps	%xmm10, 0xb0(%rsp)
1422a929f:     	movaps	%xmm9, 0xc0(%rsp)
1422a92a8:     	movaps	%xmm8, 0xd0(%rsp)
1422a92b1:     	movaps	%xmm7, 0xe0(%rsp)
1422a92b9:     	movaps	%xmm6, 0xf0(%rsp)
1422a92c1:     	movl	$0x15, %edx
1422a92c6:     	movl	$0x1, %r8d
1422a92cc:     	movq	0x1e0(%rbp), %rcx
1422a92d3:     	callq	0x140613c20 <.text+0x612c20>
1422a92d8:     	movaps	0xf0(%rsp), %xmm6
1422a92e0:     	movaps	0xe0(%rsp), %xmm7
1422a92e8:     	movaps	0xd0(%rsp), %xmm8
1422a92f1:     	movaps	0xc0(%rsp), %xmm9
1422a92fa:     	movaps	0xb0(%rsp), %xmm10
1422a9303:     	movaps	0xa0(%rsp), %xmm11
1422a930c:     	movaps	0x90(%rsp), %xmm12
1422a9315:     	movaps	0x80(%rsp), %xmm13
1422a931e:     	movaps	0x70(%rsp), %xmm14
1422a9324:     	movaps	0x60(%rsp), %xmm15
1422a932a:     	addq	$0x108, %rsp            # imm = 0x108
1422a9331:     	popq	%rbx
1422a9332:     	popq	%rdi
1422a9333:     	popq	%rsi
1422a9334:     	popq	%r12
1422a9336:     	popq	%r13
1422a9338:     	popq	%r14
1422a933a:     	popq	%r15
1422a933c:     	popq	%rbp
1422a933d:     	retq
1422a933e:     	nop
1422a9340:     	movq	%rdx, 0x10(%rsp)
1422a9345:     	pushq	%rbp
1422a9346:     	pushq	%r15
1422a9348:     	pushq	%r14
1422a934a:     	pushq	%r13
1422a934c:     	pushq	%r12
1422a934e:     	pushq	%rsi
1422a934f:     	pushq	%rdi
1422a9350:     	pushq	%rbx
1422a9351:     	subq	$0x108, %rsp            # imm = 0x108
1422a9358:     	leaq	0x80(%rdx), %rbp
1422a935f:     	movaps	%xmm15, 0x60(%rsp)
1422a9365:     	movaps	%xmm14, 0x70(%rsp)
1422a936b:     	movaps	%xmm13, 0x80(%rsp)
1422a9374:     	movaps	%xmm12, 0x90(%rsp)
1422a937d:     	movaps	%xmm11, 0xa0(%rsp)
1422a9386:     	movaps	%xmm10, 0xb0(%rsp)
1422a938f:     	movaps	%xmm9, 0xc0(%rsp)
1422a9398:     	movaps	%xmm8, 0xd0(%rsp)
1422a93a1:     	movaps	%xmm7, 0xe0(%rsp)
1422a93a9:     	movaps	%xmm6, 0xf0(%rsp)
1422a93b1:     	movl	$0x16, %edx
1422a93b6:     	movl	$0x1, %r8d
1422a93bc:     	movq	0x1d8(%rbp), %rcx
1422a93c3:     	callq	0x140613c20 <.text+0x612c20>
1422a93c8:     	movaps	0xf0(%rsp), %xmm6
1422a93d0:     	movaps	0xe0(%rsp), %xmm7
1422a93d8:     	movaps	0xd0(%rsp), %xmm8
1422a93e1:     	movaps	0xc0(%rsp), %xmm9
1422a93ea:     	movaps	0xb0(%rsp), %xmm10
1422a93f3:     	movaps	0xa0(%rsp), %xmm11
1422a93fc:     	movaps	0x90(%rsp), %xmm12
1422a9405:     	movaps	0x80(%rsp), %xmm13
1422a940e:     	movaps	0x70(%rsp), %xmm14
1422a9414:     	movaps	0x60(%rsp), %xmm15
1422a941a:     	addq	$0x108, %rsp            # imm = 0x108
1422a9421:     	popq	%rbx
1422a9422:     	popq	%rdi
1422a9423:     	popq	%rsi
1422a9424:     	popq	%r12
1422a9426:     	popq	%r13
1422a9428:     	popq	%r14
1422a942a:     	popq	%r15
1422a942c:     	popq	%rbp
1422a942d:     	retq
1422a942e:     	nop
1422a9430:     	movq	%rdx, 0x10(%rsp)
1422a9435:     	pushq	%rbp
1422a9436:     	pushq	%r15
1422a9438:     	pushq	%r14
1422a943a:     	pushq	%r13
1422a943c:     	pushq	%r12
1422a943e:     	pushq	%rsi
1422a943f:     	pushq	%rdi
1422a9440:     	pushq	%rbx
1422a9441:     	subq	$0x108, %rsp            # imm = 0x108
1422a9448:     	leaq	0x80(%rdx), %rbp
1422a944f:     	movaps	%xmm15, 0x60(%rsp)
1422a9455:     	movaps	%xmm14, 0x70(%rsp)
1422a945b:     	movaps	%xmm13, 0x80(%rsp)
1422a9464:     	movaps	%xmm12, 0x90(%rsp)
1422a946d:     	movaps	%xmm11, 0xa0(%rsp)
1422a9476:     	movaps	%xmm10, 0xb0(%rsp)
1422a947f:     	movaps	%xmm9, 0xc0(%rsp)
1422a9488:     	movaps	%xmm8, 0xd0(%rsp)
1422a9491:     	movaps	%xmm7, 0xe0(%rsp)
1422a9499:     	movaps	%xmm6, 0xf0(%rsp)
1422a94a1:     	movl	$0x17, %edx
1422a94a6:     	movl	$0x1, %r8d
1422a94ac:     	movq	0x1d8(%rbp), %rcx
1422a94b3:     	callq	0x140613c20 <.text+0x612c20>
1422a94b8:     	movaps	0xf0(%rsp), %xmm6
1422a94c0:     	movaps	0xe0(%rsp), %xmm7
1422a94c8:     	movaps	0xd0(%rsp), %xmm8
1422a94d1:     	movaps	0xc0(%rsp), %xmm9
1422a94da:     	movaps	0xb0(%rsp), %xmm10
1422a94e3:     	movaps	0xa0(%rsp), %xmm11
1422a94ec:     	movaps	0x90(%rsp), %xmm12
1422a94f5:     	movaps	0x80(%rsp), %xmm13
1422a94fe:     	movaps	0x70(%rsp), %xmm14
1422a9504:     	movaps	0x60(%rsp), %xmm15
1422a950a:     	addq	$0x108, %rsp            # imm = 0x108
1422a9511:     	popq	%rbx
1422a9512:     	popq	%rdi
1422a9513:     	popq	%rsi
1422a9514:     	popq	%r12
1422a9516:     	popq	%r13
1422a9518:     	popq	%r14
1422a951a:     	popq	%r15
1422a951c:     	popq	%rbp
1422a951d:     	retq
1422a951e:     	nop
1422a9520:     	movq	%rdx, 0x10(%rsp)
1422a9525:     	pushq	%rbp
1422a9526:     	pushq	%r15
1422a9528:     	pushq	%r14
1422a952a:     	pushq	%r13
1422a952c:     	pushq	%r12
1422a952e:     	pushq	%rsi
1422a952f:     	pushq	%rdi
1422a9530:     	pushq	%rbx
1422a9531:     	subq	$0x108, %rsp            # imm = 0x108
1422a9538:     	leaq	0x80(%rdx), %rbp
1422a953f:     	movaps	%xmm15, 0x60(%rsp)
1422a9545:     	movaps	%xmm14, 0x70(%rsp)
1422a954b:     	movaps	%xmm13, 0x80(%rsp)
1422a9554:     	movaps	%xmm12, 0x90(%rsp)
1422a955d:     	movaps	%xmm11, 0xa0(%rsp)
1422a9566:     	movaps	%xmm10, 0xb0(%rsp)
1422a956f:     	movaps	%xmm9, 0xc0(%rsp)
1422a9578:     	movaps	%xmm8, 0xd0(%rsp)
1422a9581:     	movaps	%xmm7, 0xe0(%rsp)
1422a9589:     	movaps	%xmm6, 0xf0(%rsp)
1422a9591:     	movl	$0x11, %edx
1422a9596:     	movl	$0x1, %r8d
1422a959c:     	movq	0x1d8(%rbp), %rcx
1422a95a3:     	callq	0x140613c20 <.text+0x612c20>
1422a95a8:     	movaps	0xf0(%rsp), %xmm6
1422a95b0:     	movaps	0xe0(%rsp), %xmm7
1422a95b8:     	movaps	0xd0(%rsp), %xmm8
1422a95c1:     	movaps	0xc0(%rsp), %xmm9
1422a95ca:     	movaps	0xb0(%rsp), %xmm10
1422a95d3:     	movaps	0xa0(%rsp), %xmm11
1422a95dc:     	movaps	0x90(%rsp), %xmm12
1422a95e5:     	movaps	0x80(%rsp), %xmm13
1422a95ee:     	movaps	0x70(%rsp), %xmm14
1422a95f4:     	movaps	0x60(%rsp), %xmm15
1422a95fa:     	addq	$0x108, %rsp            # imm = 0x108
1422a9601:     	popq	%rbx
1422a9602:     	popq	%rdi
1422a9603:     	popq	%rsi
1422a9604:     	popq	%r12
1422a9606:     	popq	%r13
1422a9608:     	popq	%r14
1422a960a:     	popq	%r15
1422a960c:     	popq	%rbp
1422a960d:     	retq
1422a960e:     	nop
1422a9610:     	movq	%rdx, 0x10(%rsp)
1422a9615:     	pushq	%rbp
1422a9616:     	pushq	%r15
1422a9618:     	pushq	%r14
1422a961a:     	pushq	%r13
1422a961c:     	pushq	%r12
1422a961e:     	pushq	%rsi
1422a961f:     	pushq	%rdi
1422a9620:     	pushq	%rbx
1422a9621:     	subq	$0x108, %rsp            # imm = 0x108
1422a9628:     	leaq	0x80(%rdx), %rbp
1422a962f:     	movaps	%xmm15, 0x60(%rsp)
1422a9635:     	movaps	%xmm14, 0x70(%rsp)
1422a963b:     	movaps	%xmm13, 0x80(%rsp)
1422a9644:     	movaps	%xmm12, 0x90(%rsp)
1422a964d:     	movaps	%xmm11, 0xa0(%rsp)
1422a9656:     	movaps	%xmm10, 0xb0(%rsp)
1422a965f:     	movaps	%xmm9, 0xc0(%rsp)
1422a9668:     	movaps	%xmm8, 0xd0(%rsp)
1422a9671:     	movaps	%xmm7, 0xe0(%rsp)
1422a9679:     	movaps	%xmm6, 0xf0(%rsp)
1422a9681:     	movl	$0x13, %edx
1422a9686:     	movl	$0x1, %r8d
1422a968c:     	movq	0x1f8(%rbp), %rcx
1422a9693:     	callq	0x140613c20 <.text+0x612c20>
1422a9698:     	movaps	0xf0(%rsp), %xmm6
1422a96a0:     	movaps	0xe0(%rsp), %xmm7
1422a96a8:     	movaps	0xd0(%rsp), %xmm8
1422a96b1:     	movaps	0xc0(%rsp), %xmm9
1422a96ba:     	movaps	0xb0(%rsp), %xmm10
1422a96c3:     	movaps	0xa0(%rsp), %xmm11
1422a96cc:     	movaps	0x90(%rsp), %xmm12
1422a96d5:     	movaps	0x80(%rsp), %xmm13
1422a96de:     	movaps	0x70(%rsp), %xmm14
1422a96e4:     	movaps	0x60(%rsp), %xmm15
1422a96ea:     	addq	$0x108, %rsp            # imm = 0x108
1422a96f1:     	popq	%rbx
1422a96f2:     	popq	%rdi
1422a96f3:     	popq	%rsi
1422a96f4:     	popq	%r12
1422a96f6:     	popq	%r13
1422a96f8:     	popq	%r14
1422a96fa:     	popq	%r15
1422a96fc:     	popq	%rbp
1422a96fd:     	retq
1422a96fe:     	nop
1422a9700:     	movq	%rdx, 0x10(%rsp)
1422a9705:     	pushq	%rbp
1422a9706:     	pushq	%r15
1422a9708:     	pushq	%r14
1422a970a:     	pushq	%r13
1422a970c:     	pushq	%r12
1422a970e:     	pushq	%rsi
1422a970f:     	pushq	%rdi
1422a9710:     	pushq	%rbx
1422a9711:     	subq	$0x108, %rsp            # imm = 0x108
1422a9718:     	leaq	0x80(%rdx), %rbp
1422a971f:     	movaps	%xmm15, 0x60(%rsp)
1422a9725:     	movaps	%xmm14, 0x70(%rsp)
1422a972b:     	movaps	%xmm13, 0x80(%rsp)
1422a9734:     	movaps	%xmm12, 0x90(%rsp)
1422a973d:     	movaps	%xmm11, 0xa0(%rsp)
1422a9746:     	movaps	%xmm10, 0xb0(%rsp)
1422a974f:     	movaps	%xmm9, 0xc0(%rsp)
1422a9758:     	movaps	%xmm8, 0xd0(%rsp)
1422a9761:     	movaps	%xmm7, 0xe0(%rsp)
1422a9769:     	movaps	%xmm6, 0xf0(%rsp)
1422a9771:     	movl	$0x16, %edx
1422a9776:     	movl	$0x1, %r8d
1422a977c:     	movq	0x1c8(%rbp), %rcx
1422a9783:     	callq	0x140613c20 <.text+0x612c20>
1422a9788:     	movaps	0xf0(%rsp), %xmm6
1422a9790:     	movaps	0xe0(%rsp), %xmm7
1422a9798:     	movaps	0xd0(%rsp), %xmm8
1422a97a1:     	movaps	0xc0(%rsp), %xmm9
1422a97aa:     	movaps	0xb0(%rsp), %xmm10
1422a97b3:     	movaps	0xa0(%rsp), %xmm11
1422a97bc:     	movaps	0x90(%rsp), %xmm12
1422a97c5:     	movaps	0x80(%rsp), %xmm13
1422a97ce:     	movaps	0x70(%rsp), %xmm14
1422a97d4:     	movaps	0x60(%rsp), %xmm15
1422a97da:     	addq	$0x108, %rsp            # imm = 0x108
1422a97e1:     	popq	%rbx
1422a97e2:     	popq	%rdi
1422a97e3:     	popq	%rsi
1422a97e4:     	popq	%r12
1422a97e6:     	popq	%r13
1422a97e8:     	popq	%r14
1422a97ea:     	popq	%r15
1422a97ec:     	popq	%rbp
1422a97ed:     	retq
1422a97ee:     	nop
1422a97f0:     	movq	%rdx, 0x10(%rsp)
1422a97f5:     	pushq	%rbp
1422a97f6:     	pushq	%r15
1422a97f8:     	pushq	%r14
1422a97fa:     	pushq	%r13
1422a97fc:     	pushq	%r12
1422a97fe:     	pushq	%rsi
1422a97ff:     	pushq	%rdi
1422a9800:     	pushq	%rbx
1422a9801:     	subq	$0x108, %rsp            # imm = 0x108
1422a9808:     	leaq	0x80(%rdx), %rbp
1422a980f:     	movaps	%xmm15, 0x60(%rsp)
1422a9815:     	movaps	%xmm14, 0x70(%rsp)
1422a981b:     	movaps	%xmm13, 0x80(%rsp)
1422a9824:     	movaps	%xmm12, 0x90(%rsp)
1422a982d:     	movaps	%xmm11, 0xa0(%rsp)
1422a9836:     	movaps	%xmm10, 0xb0(%rsp)
1422a983f:     	movaps	%xmm9, 0xc0(%rsp)
1422a9848:     	movaps	%xmm8, 0xd0(%rsp)
1422a9851:     	movaps	%xmm7, 0xe0(%rsp)
1422a9859:     	movaps	%xmm6, 0xf0(%rsp)
1422a9861:     	movl	$0x12, %edx
1422a9866:     	movl	$0x1, %r8d
1422a986c:     	movq	0x1f8(%rbp), %rcx
1422a9873:     	callq	0x140613c20 <.text+0x612c20>
1422a9878:     	movaps	0xf0(%rsp), %xmm6
1422a9880:     	movaps	0xe0(%rsp), %xmm7
1422a9888:     	movaps	0xd0(%rsp), %xmm8
1422a9891:     	movaps	0xc0(%rsp), %xmm9
1422a989a:     	movaps	0xb0(%rsp), %xmm10
1422a98a3:     	movaps	0xa0(%rsp), %xmm11
1422a98ac:     	movaps	0x90(%rsp), %xmm12
1422a98b5:     	movaps	0x80(%rsp), %xmm13
1422a98be:     	movaps	0x70(%rsp), %xmm14
1422a98c4:     	movaps	0x60(%rsp), %xmm15
1422a98ca:     	addq	$0x108, %rsp            # imm = 0x108
1422a98d1:     	popq	%rbx
1422a98d2:     	popq	%rdi
1422a98d3:     	popq	%rsi
1422a98d4:     	popq	%r12
1422a98d6:     	popq	%r13
1422a98d8:     	popq	%r14
1422a98da:     	popq	%r15
1422a98dc:     	popq	%rbp
1422a98dd:     	retq
1422a98de:     	nop
1422a98e0:     	movq	%rdx, 0x10(%rsp)
1422a98e5:     	pushq	%rbp
1422a98e6:     	pushq	%r15
1422a98e8:     	pushq	%r14
1422a98ea:     	pushq	%r13
1422a98ec:     	pushq	%r12
1422a98ee:     	pushq	%rsi
1422a98ef:     	pushq	%rdi
1422a98f0:     	pushq	%rbx
1422a98f1:     	subq	$0x108, %rsp            # imm = 0x108
1422a98f8:     	leaq	0x80(%rdx), %rbp
1422a98ff:     	movaps	%xmm15, 0x60(%rsp)
1422a9905:     	movaps	%xmm14, 0x70(%rsp)
1422a990b:     	movaps	%xmm13, 0x80(%rsp)
1422a9914:     	movaps	%xmm12, 0x90(%rsp)
1422a991d:     	movaps	%xmm11, 0xa0(%rsp)
1422a9926:     	movaps	%xmm10, 0xb0(%rsp)
1422a992f:     	movaps	%xmm9, 0xc0(%rsp)
1422a9938:     	movaps	%xmm8, 0xd0(%rsp)
1422a9941:     	movaps	%xmm7, 0xe0(%rsp)
1422a9949:     	movaps	%xmm6, 0xf0(%rsp)
1422a9951:     	movl	$0x15, %edx
1422a9956:     	movl	$0x1, %r8d
1422a995c:     	movq	0x1c0(%rbp), %rcx
1422a9963:     	callq	0x140613c20 <.text+0x612c20>
1422a9968:     	movaps	0xf0(%rsp), %xmm6
1422a9970:     	movaps	0xe0(%rsp), %xmm7
1422a9978:     	movaps	0xd0(%rsp), %xmm8
1422a9981:     	movaps	0xc0(%rsp), %xmm9
1422a998a:     	movaps	0xb0(%rsp), %xmm10
1422a9993:     	movaps	0xa0(%rsp), %xmm11
1422a999c:     	movaps	0x90(%rsp), %xmm12
1422a99a5:     	movaps	0x80(%rsp), %xmm13
1422a99ae:     	movaps	0x70(%rsp), %xmm14
1422a99b4:     	movaps	0x60(%rsp), %xmm15
1422a99ba:     	addq	$0x108, %rsp            # imm = 0x108
1422a99c1:     	popq	%rbx
1422a99c2:     	popq	%rdi
1422a99c3:     	popq	%rsi
1422a99c4:     	popq	%r12
1422a99c6:     	popq	%r13
1422a99c8:     	popq	%r14
1422a99ca:     	popq	%r15
1422a99cc:     	popq	%rbp
1422a99cd:     	retq
1422a99ce:     	nop
1422a99d0:     	movq	%rdx, 0x10(%rsp)
1422a99d5:     	pushq	%rbp
1422a99d6:     	pushq	%r15
1422a99d8:     	pushq	%r14
1422a99da:     	pushq	%r13
1422a99dc:     	pushq	%r12
1422a99de:     	pushq	%rsi
1422a99df:     	pushq	%rdi
1422a99e0:     	pushq	%rbx
1422a99e1:     	subq	$0x108, %rsp            # imm = 0x108
1422a99e8:     	leaq	0x80(%rdx), %rbp
1422a99ef:     	movaps	%xmm15, 0x60(%rsp)
1422a99f5:     	movaps	%xmm14, 0x70(%rsp)
1422a99fb:     	movaps	%xmm13, 0x80(%rsp)
1422a9a04:     	movaps	%xmm12, 0x90(%rsp)
1422a9a0d:     	movaps	%xmm11, 0xa0(%rsp)
1422a9a16:     	movaps	%xmm10, 0xb0(%rsp)
1422a9a1f:     	movaps	%xmm9, 0xc0(%rsp)
1422a9a28:     	movaps	%xmm8, 0xd0(%rsp)
1422a9a31:     	movaps	%xmm7, 0xe0(%rsp)
1422a9a39:     	movaps	%xmm6, 0xf0(%rsp)
1422a9a41:     	movl	$0x1b, %edx
1422a9a46:     	movl	$0x1, %r8d
1422a9a4c:     	movq	0x1c8(%rbp), %rcx
1422a9a53:     	callq	0x140613c20 <.text+0x612c20>
1422a9a58:     	movaps	0xf0(%rsp), %xmm6
1422a9a60:     	movaps	0xe0(%rsp), %xmm7
1422a9a68:     	movaps	0xd0(%rsp), %xmm8
1422a9a71:     	movaps	0xc0(%rsp), %xmm9
1422a9a7a:     	movaps	0xb0(%rsp), %xmm10
1422a9a83:     	movaps	0xa0(%rsp), %xmm11
1422a9a8c:     	movaps	0x90(%rsp), %xmm12
1422a9a95:     	movaps	0x80(%rsp), %xmm13
1422a9a9e:     	movaps	0x70(%rsp), %xmm14
1422a9aa4:     	movaps	0x60(%rsp), %xmm15
1422a9aaa:     	addq	$0x108, %rsp            # imm = 0x108
1422a9ab1:     	popq	%rbx
1422a9ab2:     	popq	%rdi
1422a9ab3:     	popq	%rsi
1422a9ab4:     	popq	%r12
1422a9ab6:     	popq	%r13
1422a9ab8:     	popq	%r14
1422a9aba:     	popq	%r15
1422a9abc:     	popq	%rbp
1422a9abd:     	retq
1422a9abe:     	nop
1422a9ac0:     	movq	%rdx, 0x10(%rsp)
1422a9ac5:     	pushq	%rbp
1422a9ac6:     	pushq	%r15
1422a9ac8:     	pushq	%r14
1422a9aca:     	pushq	%r13
1422a9acc:     	pushq	%r12
1422a9ace:     	pushq	%rsi
1422a9acf:     	pushq	%rdi
1422a9ad0:     	pushq	%rbx
1422a9ad1:     	subq	$0x108, %rsp            # imm = 0x108
1422a9ad8:     	leaq	0x80(%rdx), %rbp
1422a9adf:     	movaps	%xmm15, 0x60(%rsp)
1422a9ae5:     	movaps	%xmm14, 0x70(%rsp)
1422a9aeb:     	movaps	%xmm13, 0x80(%rsp)
1422a9af4:     	movaps	%xmm12, 0x90(%rsp)
1422a9afd:     	movaps	%xmm11, 0xa0(%rsp)
1422a9b06:     	movaps	%xmm10, 0xb0(%rsp)
1422a9b0f:     	movaps	%xmm9, 0xc0(%rsp)
1422a9b18:     	movaps	%xmm8, 0xd0(%rsp)
1422a9b21:     	movaps	%xmm7, 0xe0(%rsp)
1422a9b29:     	movaps	%xmm6, 0xf0(%rsp)
1422a9b31:     	movl	$0x5, %edx
1422a9b36:     	movl	$0x1, %r8d
1422a9b3c:     	movq	0x1e8(%rbp), %rcx
1422a9b43:     	callq	0x140613c20 <.text+0x612c20>
1422a9b48:     	movaps	0xf0(%rsp), %xmm6
1422a9b50:     	movaps	0xe0(%rsp), %xmm7
1422a9b58:     	movaps	0xd0(%rsp), %xmm8
1422a9b61:     	movaps	0xc0(%rsp), %xmm9
1422a9b6a:     	movaps	0xb0(%rsp), %xmm10
1422a9b73:     	movaps	0xa0(%rsp), %xmm11
1422a9b7c:     	movaps	0x90(%rsp), %xmm12
1422a9b85:     	movaps	0x80(%rsp), %xmm13
1422a9b8e:     	movaps	0x70(%rsp), %xmm14
1422a9b94:     	movaps	0x60(%rsp), %xmm15
1422a9b9a:     	addq	$0x108, %rsp            # imm = 0x108
1422a9ba1:     	popq	%rbx
1422a9ba2:     	popq	%rdi
1422a9ba3:     	popq	%rsi
1422a9ba4:     	popq	%r12
1422a9ba6:     	popq	%r13
1422a9ba8:     	popq	%r14
1422a9baa:     	popq	%r15
1422a9bac:     	popq	%rbp
1422a9bad:     	retq
1422a9bae:     	nop
1422a9bb0:     	movq	%rdx, 0x10(%rsp)
1422a9bb5:     	pushq	%rbp
1422a9bb6:     	pushq	%r15
1422a9bb8:     	pushq	%r14
1422a9bba:     	pushq	%r13
1422a9bbc:     	pushq	%r12
1422a9bbe:     	pushq	%rsi
1422a9bbf:     	pushq	%rdi
1422a9bc0:     	pushq	%rbx
1422a9bc1:     	subq	$0x108, %rsp            # imm = 0x108
1422a9bc8:     	leaq	0x80(%rdx), %rbp
1422a9bcf:     	movaps	%xmm15, 0x60(%rsp)
1422a9bd5:     	movaps	%xmm14, 0x70(%rsp)
1422a9bdb:     	movaps	%xmm13, 0x80(%rsp)
1422a9be4:     	movaps	%xmm12, 0x90(%rsp)
1422a9bed:     	movaps	%xmm11, 0xa0(%rsp)
1422a9bf6:     	movaps	%xmm10, 0xb0(%rsp)
1422a9bff:     	movaps	%xmm9, 0xc0(%rsp)
1422a9c08:     	movaps	%xmm8, 0xd0(%rsp)
1422a9c11:     	movaps	%xmm7, 0xe0(%rsp)
1422a9c19:     	movaps	%xmm6, 0xf0(%rsp)
1422a9c21:     	movl	$0x17, %edx
1422a9c26:     	movl	$0x1, %r8d
1422a9c2c:     	movq	0x1d8(%rbp), %rcx
1422a9c33:     	callq	0x140613c20 <.text+0x612c20>
1422a9c38:     	movaps	0xf0(%rsp), %xmm6
1422a9c40:     	movaps	0xe0(%rsp), %xmm7
1422a9c48:     	movaps	0xd0(%rsp), %xmm8
1422a9c51:     	movaps	0xc0(%rsp), %xmm9
1422a9c5a:     	movaps	0xb0(%rsp), %xmm10
1422a9c63:     	movaps	0xa0(%rsp), %xmm11
1422a9c6c:     	movaps	0x90(%rsp), %xmm12
1422a9c75:     	movaps	0x80(%rsp), %xmm13
1422a9c7e:     	movaps	0x70(%rsp), %xmm14
1422a9c84:     	movaps	0x60(%rsp), %xmm15
1422a9c8a:     	addq	$0x108, %rsp            # imm = 0x108
1422a9c91:     	popq	%rbx
1422a9c92:     	popq	%rdi
1422a9c93:     	popq	%rsi
1422a9c94:     	popq	%r12
1422a9c96:     	popq	%r13
1422a9c98:     	popq	%r14
1422a9c9a:     	popq	%r15
1422a9c9c:     	popq	%rbp
1422a9c9d:     	retq
1422a9c9e:     	nop
1422a9ca0:     	movq	%rdx, 0x10(%rsp)
1422a9ca5:     	pushq	%rbp
1422a9ca6:     	pushq	%r15
1422a9ca8:     	pushq	%r14
1422a9caa:     	pushq	%r13
1422a9cac:     	pushq	%r12
1422a9cae:     	pushq	%rsi
1422a9caf:     	pushq	%rdi
1422a9cb0:     	pushq	%rbx
1422a9cb1:     	subq	$0x108, %rsp            # imm = 0x108
1422a9cb8:     	leaq	0x80(%rdx), %rbp
1422a9cbf:     	movaps	%xmm15, 0x60(%rsp)
1422a9cc5:     	movaps	%xmm14, 0x70(%rsp)
1422a9ccb:     	movaps	%xmm13, 0x80(%rsp)
1422a9cd4:     	movaps	%xmm12, 0x90(%rsp)
1422a9cdd:     	movaps	%xmm11, 0xa0(%rsp)
1422a9ce6:     	movaps	%xmm10, 0xb0(%rsp)
1422a9cef:     	movaps	%xmm9, 0xc0(%rsp)
1422a9cf8:     	movaps	%xmm8, 0xd0(%rsp)
1422a9d01:     	movaps	%xmm7, 0xe0(%rsp)
1422a9d09:     	movaps	%xmm6, 0xf0(%rsp)
1422a9d11:     	movl	$0x16, %edx
1422a9d16:     	movl	$0x1, %r8d
1422a9d1c:     	movq	0x1e8(%rbp), %rcx
1422a9d23:     	callq	0x140613c20 <.text+0x612c20>
1422a9d28:     	movaps	0xf0(%rsp), %xmm6
1422a9d30:     	movaps	0xe0(%rsp), %xmm7
1422a9d38:     	movaps	0xd0(%rsp), %xmm8
1422a9d41:     	movaps	0xc0(%rsp), %xmm9
1422a9d4a:     	movaps	0xb0(%rsp), %xmm10
1422a9d53:     	movaps	0xa0(%rsp), %xmm11
1422a9d5c:     	movaps	0x90(%rsp), %xmm12
1422a9d65:     	movaps	0x80(%rsp), %xmm13
1422a9d6e:     	movaps	0x70(%rsp), %xmm14
1422a9d74:     	movaps	0x60(%rsp), %xmm15
1422a9d7a:     	addq	$0x108, %rsp            # imm = 0x108
1422a9d81:     	popq	%rbx
1422a9d82:     	popq	%rdi
1422a9d83:     	popq	%rsi
1422a9d84:     	popq	%r12
1422a9d86:     	popq	%r13
1422a9d88:     	popq	%r14
1422a9d8a:     	popq	%r15
1422a9d8c:     	popq	%rbp
1422a9d8d:     	retq
1422a9d8e:     	nop
1422a9d90:     	movq	%rdx, 0x10(%rsp)
1422a9d95:     	pushq	%rbp
1422a9d96:     	pushq	%r15
1422a9d98:     	pushq	%r14
1422a9d9a:     	pushq	%r13
1422a9d9c:     	pushq	%r12
1422a9d9e:     	pushq	%rsi
1422a9d9f:     	pushq	%rdi
1422a9da0:     	pushq	%rbx
1422a9da1:     	subq	$0x108, %rsp            # imm = 0x108
1422a9da8:     	leaq	0x80(%rdx), %rbp
1422a9daf:     	movaps	%xmm15, 0x60(%rsp)
1422a9db5:     	movaps	%xmm14, 0x70(%rsp)
1422a9dbb:     	movaps	%xmm13, 0x80(%rsp)
1422a9dc4:     	movaps	%xmm12, 0x90(%rsp)
1422a9dcd:     	movaps	%xmm11, 0xa0(%rsp)
1422a9dd6:     	movaps	%xmm10, 0xb0(%rsp)
1422a9ddf:     	movaps	%xmm9, 0xc0(%rsp)
1422a9de8:     	movaps	%xmm8, 0xd0(%rsp)
1422a9df1:     	movaps	%xmm7, 0xe0(%rsp)
1422a9df9:     	movaps	%xmm6, 0xf0(%rsp)
1422a9e01:     	movl	$0x16, %edx
1422a9e06:     	movl	$0x1, %r8d
1422a9e0c:     	movq	0x1e8(%rbp), %rcx
1422a9e13:     	callq	0x140613c20 <.text+0x612c20>
1422a9e18:     	movaps	0xf0(%rsp), %xmm6
1422a9e20:     	movaps	0xe0(%rsp), %xmm7
1422a9e28:     	movaps	0xd0(%rsp), %xmm8
1422a9e31:     	movaps	0xc0(%rsp), %xmm9
1422a9e3a:     	movaps	0xb0(%rsp), %xmm10
1422a9e43:     	movaps	0xa0(%rsp), %xmm11
1422a9e4c:     	movaps	0x90(%rsp), %xmm12
1422a9e55:     	movaps	0x80(%rsp), %xmm13
1422a9e5e:     	movaps	0x70(%rsp), %xmm14
1422a9e64:     	movaps	0x60(%rsp), %xmm15
1422a9e6a:     	addq	$0x108, %rsp            # imm = 0x108
1422a9e71:     	popq	%rbx
1422a9e72:     	popq	%rdi
1422a9e73:     	popq	%rsi
1422a9e74:     	popq	%r12
1422a9e76:     	popq	%r13
1422a9e78:     	popq	%r14
1422a9e7a:     	popq	%r15
1422a9e7c:     	popq	%rbp
1422a9e7d:     	retq
1422a9e7e:     	nop
1422a9e80:     	movq	%rdx, 0x10(%rsp)
1422a9e85:     	pushq	%rbp
1422a9e86:     	pushq	%r15
1422a9e88:     	pushq	%r14
1422a9e8a:     	pushq	%r13
1422a9e8c:     	pushq	%r12
1422a9e8e:     	pushq	%rsi
1422a9e8f:     	pushq	%rdi
1422a9e90:     	pushq	%rbx
1422a9e91:     	subq	$0x108, %rsp            # imm = 0x108
1422a9e98:     	leaq	0x80(%rdx), %rbp
1422a9e9f:     	movaps	%xmm15, 0x60(%rsp)
1422a9ea5:     	movaps	%xmm14, 0x70(%rsp)
1422a9eab:     	movaps	%xmm13, 0x80(%rsp)
1422a9eb4:     	movaps	%xmm12, 0x90(%rsp)
1422a9ebd:     	movaps	%xmm11, 0xa0(%rsp)
1422a9ec6:     	movaps	%xmm10, 0xb0(%rsp)
1422a9ecf:     	movaps	%xmm9, 0xc0(%rsp)
1422a9ed8:     	movaps	%xmm8, 0xd0(%rsp)
1422a9ee1:     	movaps	%xmm7, 0xe0(%rsp)
1422a9ee9:     	movaps	%xmm6, 0xf0(%rsp)
1422a9ef1:     	movl	$0x11, %edx
1422a9ef6:     	movl	$0x1, %r8d
1422a9efc:     	movq	0x1e8(%rbp), %rcx
1422a9f03:     	callq	0x140613c20 <.text+0x612c20>
1422a9f08:     	movaps	0xf0(%rsp), %xmm6
1422a9f10:     	movaps	0xe0(%rsp), %xmm7
1422a9f18:     	movaps	0xd0(%rsp), %xmm8
1422a9f21:     	movaps	0xc0(%rsp), %xmm9
1422a9f2a:     	movaps	0xb0(%rsp), %xmm10
1422a9f33:     	movaps	0xa0(%rsp), %xmm11
1422a9f3c:     	movaps	0x90(%rsp), %xmm12
1422a9f45:     	movaps	0x80(%rsp), %xmm13
1422a9f4e:     	movaps	0x70(%rsp), %xmm14
1422a9f54:     	movaps	0x60(%rsp), %xmm15
1422a9f5a:     	addq	$0x108, %rsp            # imm = 0x108
1422a9f61:     	popq	%rbx
1422a9f62:     	popq	%rdi
1422a9f63:     	popq	%rsi
1422a9f64:     	popq	%r12
1422a9f66:     	popq	%r13
1422a9f68:     	popq	%r14
1422a9f6a:     	popq	%r15
1422a9f6c:     	popq	%rbp
1422a9f6d:     	retq
1422a9f6e:     	nop
1422a9f70:     	movq	%rdx, 0x10(%rsp)
1422a9f75:     	pushq	%rbp
1422a9f76:     	pushq	%r15
1422a9f78:     	pushq	%r14
1422a9f7a:     	pushq	%r13
1422a9f7c:     	pushq	%r12
1422a9f7e:     	pushq	%rsi
1422a9f7f:     	pushq	%rdi
1422a9f80:     	pushq	%rbx
1422a9f81:     	subq	$0x108, %rsp            # imm = 0x108
1422a9f88:     	leaq	0x80(%rdx), %rbp
1422a9f8f:     	movaps	%xmm15, 0x60(%rsp)
1422a9f95:     	movaps	%xmm14, 0x70(%rsp)
1422a9f9b:     	movaps	%xmm13, 0x80(%rsp)
1422a9fa4:     	movaps	%xmm12, 0x90(%rsp)
1422a9fad:     	movaps	%xmm11, 0xa0(%rsp)
1422a9fb6:     	movaps	%xmm10, 0xb0(%rsp)
1422a9fbf:     	movaps	%xmm9, 0xc0(%rsp)
1422a9fc8:     	movaps	%xmm8, 0xd0(%rsp)
1422a9fd1:     	movaps	%xmm7, 0xe0(%rsp)
1422a9fd9:     	movaps	%xmm6, 0xf0(%rsp)
1422a9fe1:     	movl	$0x168, %edx            # imm = 0x168
1422a9fe6:     	movl	$0x8, %r8d
1422a9fec:     	movq	0x200(%rbp), %rcx
1422a9ff3:     	callq	0x140613c20 <.text+0x612c20>
1422a9ff8:     	movaps	0xf0(%rsp), %xmm6
1422aa000:     	movaps	0xe0(%rsp), %xmm7
1422aa008:     	movaps	0xd0(%rsp), %xmm8
1422aa011:     	movaps	0xc0(%rsp), %xmm9
1422aa01a:     	movaps	0xb0(%rsp), %xmm10
1422aa023:     	movaps	0xa0(%rsp), %xmm11
1422aa02c:     	movaps	0x90(%rsp), %xmm12
1422aa035:     	movaps	0x80(%rsp), %xmm13
1422aa03e:     	movaps	0x70(%rsp), %xmm14
1422aa044:     	movaps	0x60(%rsp), %xmm15
1422aa04a:     	addq	$0x108, %rsp            # imm = 0x108
1422aa051:     	popq	%rbx
1422aa052:     	popq	%rdi
1422aa053:     	popq	%rsi
1422aa054:     	popq	%r12
1422aa056:     	popq	%r13
1422aa058:     	popq	%r14
1422aa05a:     	popq	%r15
1422aa05c:     	popq	%rbp
1422aa05d:     	retq
1422aa05e:     	nop
1422aa060:     	movq	%rdx, 0x10(%rsp)
1422aa065:     	pushq	%rbp
1422aa066:     	pushq	%r15
1422aa068:     	pushq	%r14
1422aa06a:     	pushq	%r13
1422aa06c:     	pushq	%r12
1422aa06e:     	pushq	%rsi
1422aa06f:     	pushq	%rdi
1422aa070:     	pushq	%rbx
1422aa071:     	subq	$0x108, %rsp            # imm = 0x108
1422aa078:     	leaq	0x80(%rdx), %rbp
1422aa07f:     	movaps	%xmm15, 0x60(%rsp)
1422aa085:     	movaps	%xmm14, 0x70(%rsp)
1422aa08b:     	movaps	%xmm13, 0x80(%rsp)
1422aa094:     	movaps	%xmm12, 0x90(%rsp)
1422aa09d:     	movaps	%xmm11, 0xa0(%rsp)
1422aa0a6:     	movaps	%xmm10, 0xb0(%rsp)
1422aa0af:     	movaps	%xmm9, 0xc0(%rsp)
1422aa0b8:     	movaps	%xmm8, 0xd0(%rsp)
1422aa0c1:     	movaps	%xmm7, 0xe0(%rsp)
1422aa0c9:     	movaps	%xmm6, 0xf0(%rsp)
1422aa0d1:     	movl	$0x13, %edx
1422aa0d6:     	movl	$0x1, %r8d
1422aa0dc:     	movq	0x1e0(%rbp), %rcx
1422aa0e3:     	callq	0x140613c20 <.text+0x612c20>
1422aa0e8:     	movaps	0xf0(%rsp), %xmm6
1422aa0f0:     	movaps	0xe0(%rsp), %xmm7
1422aa0f8:     	movaps	0xd0(%rsp), %xmm8
1422aa101:     	movaps	0xc0(%rsp), %xmm9
1422aa10a:     	movaps	0xb0(%rsp), %xmm10
1422aa113:     	movaps	0xa0(%rsp), %xmm11
1422aa11c:     	movaps	0x90(%rsp), %xmm12
1422aa125:     	movaps	0x80(%rsp), %xmm13
1422aa12e:     	movaps	0x70(%rsp), %xmm14
1422aa134:     	movaps	0x60(%rsp), %xmm15
1422aa13a:     	addq	$0x108, %rsp            # imm = 0x108
1422aa141:     	popq	%rbx
1422aa142:     	popq	%rdi
1422aa143:     	popq	%rsi
1422aa144:     	popq	%r12
1422aa146:     	popq	%r13
1422aa148:     	popq	%r14
1422aa14a:     	popq	%r15
1422aa14c:     	popq	%rbp
1422aa14d:     	retq
1422aa14e:     	nop
1422aa150:     	movq	%rdx, 0x10(%rsp)
1422aa155:     	pushq	%rbp
1422aa156:     	pushq	%r15
1422aa158:     	pushq	%r14
1422aa15a:     	pushq	%r13
1422aa15c:     	pushq	%r12
1422aa15e:     	pushq	%rsi
1422aa15f:     	pushq	%rdi
1422aa160:     	pushq	%rbx
1422aa161:     	subq	$0x108, %rsp            # imm = 0x108
1422aa168:     	leaq	0x80(%rdx), %rbp
1422aa16f:     	movaps	%xmm15, 0x60(%rsp)
1422aa175:     	movaps	%xmm14, 0x70(%rsp)
1422aa17b:     	movaps	%xmm13, 0x80(%rsp)
1422aa184:     	movaps	%xmm12, 0x90(%rsp)
1422aa18d:     	movaps	%xmm11, 0xa0(%rsp)
1422aa196:     	movaps	%xmm10, 0xb0(%rsp)
1422aa19f:     	movaps	%xmm9, 0xc0(%rsp)
1422aa1a8:     	movaps	%xmm8, 0xd0(%rsp)
1422aa1b1:     	movaps	%xmm7, 0xe0(%rsp)
1422aa1b9:     	movaps	%xmm6, 0xf0(%rsp)
1422aa1c1:     	leaq	0x158(%rbp), %rcx
1422aa1c8:     	callq	0x14014a860 <.text+0x149860>
1422aa1cd:     	movaps	0xf0(%rsp), %xmm6
1422aa1d5:     	movaps	0xe0(%rsp), %xmm7
1422aa1dd:     	movaps	0xd0(%rsp), %xmm8
1422aa1e6:     	movaps	0xc0(%rsp), %xmm9
1422aa1ef:     	movaps	0xb0(%rsp), %xmm10
1422aa1f8:     	movaps	0xa0(%rsp), %xmm11
1422aa201:     	movaps	0x90(%rsp), %xmm12
1422aa20a:     	movaps	0x80(%rsp), %xmm13
1422aa213:     	movaps	0x70(%rsp), %xmm14
1422aa219:     	movaps	0x60(%rsp), %xmm15
1422aa21f:     	addq	$0x108, %rsp            # imm = 0x108
1422aa226:     	popq	%rbx
1422aa227:     	popq	%rdi
1422aa228:     	popq	%rsi
1422aa229:     	popq	%r12
1422aa22b:     	popq	%r13
1422aa22d:     	popq	%r14
1422aa22f:     	popq	%r15
1422aa231:     	popq	%rbp
1422aa232:     	retq
1422aa233:     	nopw	%cs:(%rax,%rax)
1422aa240:     	movq	%rdx, 0x10(%rsp)
1422aa245:     	pushq	%rbp
1422aa246:     	pushq	%r15
1422aa248:     	pushq	%r14
1422aa24a:     	pushq	%r13
1422aa24c:     	pushq	%r12
1422aa24e:     	pushq	%rsi
1422aa24f:     	pushq	%rdi
1422aa250:     	pushq	%rbx
1422aa251:     	subq	$0x108, %rsp            # imm = 0x108
1422aa258:     	leaq	0x80(%rdx), %rbp
1422aa25f:     	movaps	%xmm15, 0x60(%rsp)
1422aa265:     	movaps	%xmm14, 0x70(%rsp)
1422aa26b:     	movaps	%xmm13, 0x80(%rsp)
1422aa274:     	movaps	%xmm12, 0x90(%rsp)
1422aa27d:     	movaps	%xmm11, 0xa0(%rsp)
1422aa286:     	movaps	%xmm10, 0xb0(%rsp)
1422aa28f:     	movaps	%xmm9, 0xc0(%rsp)
1422aa298:     	movaps	%xmm8, 0xd0(%rsp)
1422aa2a1:     	movaps	%xmm7, 0xe0(%rsp)
1422aa2a9:     	movaps	%xmm6, 0xf0(%rsp)
1422aa2b1:     	movl	$0x17, %edx
1422aa2b6:     	movl	$0x1, %r8d
1422aa2bc:     	movq	0x1f0(%rbp), %rcx
1422aa2c3:     	callq	0x140613c20 <.text+0x612c20>
1422aa2c8:     	movaps	0xf0(%rsp), %xmm6
1422aa2d0:     	movaps	0xe0(%rsp), %xmm7
1422aa2d8:     	movaps	0xd0(%rsp), %xmm8
1422aa2e1:     	movaps	0xc0(%rsp), %xmm9
1422aa2ea:     	movaps	0xb0(%rsp), %xmm10
1422aa2f3:     	movaps	0xa0(%rsp), %xmm11
1422aa2fc:     	movaps	0x90(%rsp), %xmm12
1422aa305:     	movaps	0x80(%rsp), %xmm13
1422aa30e:     	movaps	0x70(%rsp), %xmm14
1422aa314:     	movaps	0x60(%rsp), %xmm15
1422aa31a:     	addq	$0x108, %rsp            # imm = 0x108
1422aa321:     	popq	%rbx
1422aa322:     	popq	%rdi
1422aa323:     	popq	%rsi
1422aa324:     	popq	%r12
1422aa326:     	popq	%r13
1422aa328:     	popq	%r14
1422aa32a:     	popq	%r15
1422aa32c:     	popq	%rbp
1422aa32d:     	retq
1422aa32e:     	nop
1422aa330:     	movq	%rdx, 0x10(%rsp)
1422aa335:     	pushq	%rbp
1422aa336:     	pushq	%r15
1422aa338:     	pushq	%r14
1422aa33a:     	pushq	%r13
1422aa33c:     	pushq	%r12
1422aa33e:     	pushq	%rsi
1422aa33f:     	pushq	%rdi
1422aa340:     	pushq	%rbx
1422aa341:     	subq	$0x108, %rsp            # imm = 0x108
1422aa348:     	leaq	0x80(%rdx), %rbp
1422aa34f:     	movaps	%xmm15, 0x60(%rsp)
1422aa355:     	movaps	%xmm14, 0x70(%rsp)
1422aa35b:     	movaps	%xmm13, 0x80(%rsp)
1422aa364:     	movaps	%xmm12, 0x90(%rsp)
1422aa36d:     	movaps	%xmm11, 0xa0(%rsp)
1422aa376:     	movaps	%xmm10, 0xb0(%rsp)
1422aa37f:     	movaps	%xmm9, 0xc0(%rsp)
1422aa388:     	movaps	%xmm8, 0xd0(%rsp)
1422aa391:     	movaps	%xmm7, 0xe0(%rsp)
1422aa399:     	movaps	%xmm6, 0xf0(%rsp)
1422aa3a1:     	movl	$0xc8, %edx
1422aa3a6:     	movl	$0x8, %r8d
1422aa3ac:     	movq	0x1e8(%rbp), %rcx
1422aa3b3:     	callq	0x140613c20 <.text+0x612c20>
1422aa3b8:     	movaps	0xf0(%rsp), %xmm6
1422aa3c0:     	movaps	0xe0(%rsp), %xmm7
1422aa3c8:     	movaps	0xd0(%rsp), %xmm8
1422aa3d1:     	movaps	0xc0(%rsp), %xmm9
1422aa3da:     	movaps	0xb0(%rsp), %xmm10
1422aa3e3:     	movaps	0xa0(%rsp), %xmm11
1422aa3ec:     	movaps	0x90(%rsp), %xmm12
1422aa3f5:     	movaps	0x80(%rsp), %xmm13
1422aa3fe:     	movaps	0x70(%rsp), %xmm14
1422aa404:     	movaps	0x60(%rsp), %xmm15
1422aa40a:     	addq	$0x108, %rsp            # imm = 0x108
1422aa411:     	popq	%rbx
1422aa412:     	popq	%rdi
1422aa413:     	popq	%rsi
1422aa414:     	popq	%r12
1422aa416:     	popq	%r13
1422aa418:     	popq	%r14
1422aa41a:     	popq	%r15
1422aa41c:     	popq	%rbp
1422aa41d:     	retq
1422aa41e:     	nop
1422aa420:     	movq	%rdx, 0x10(%rsp)
1422aa425:     	pushq	%rbp
1422aa426:     	pushq	%r15
1422aa428:     	pushq	%r14
1422aa42a:     	pushq	%r13
1422aa42c:     	pushq	%r12
1422aa42e:     	pushq	%rsi
1422aa42f:     	pushq	%rdi
1422aa430:     	pushq	%rbx
1422aa431:     	subq	$0x108, %rsp            # imm = 0x108
1422aa438:     	leaq	0x80(%rdx), %rbp
1422aa43f:     	movaps	%xmm15, 0x60(%rsp)
1422aa445:     	movaps	%xmm14, 0x70(%rsp)
1422aa44b:     	movaps	%xmm13, 0x80(%rsp)
1422aa454:     	movaps	%xmm12, 0x90(%rsp)
1422aa45d:     	movaps	%xmm11, 0xa0(%rsp)
1422aa466:     	movaps	%xmm10, 0xb0(%rsp)
1422aa46f:     	movaps	%xmm9, 0xc0(%rsp)
1422aa478:     	movaps	%xmm8, 0xd0(%rsp)
1422aa481:     	movaps	%xmm7, 0xe0(%rsp)
1422aa489:     	movaps	%xmm6, 0xf0(%rsp)
1422aa491:     	movl	$0x11, %edx
1422aa496:     	movl	$0x1, %r8d
1422aa49c:     	movq	0x1f0(%rbp), %rcx
1422aa4a3:     	callq	0x140613c20 <.text+0x612c20>
1422aa4a8:     	movaps	0xf0(%rsp), %xmm6
1422aa4b0:     	movaps	0xe0(%rsp), %xmm7
1422aa4b8:     	movaps	0xd0(%rsp), %xmm8
1422aa4c1:     	movaps	0xc0(%rsp), %xmm9
1422aa4ca:     	movaps	0xb0(%rsp), %xmm10
1422aa4d3:     	movaps	0xa0(%rsp), %xmm11
1422aa4dc:     	movaps	0x90(%rsp), %xmm12
1422aa4e5:     	movaps	0x80(%rsp), %xmm13
1422aa4ee:     	movaps	0x70(%rsp), %xmm14
1422aa4f4:     	movaps	0x60(%rsp), %xmm15
1422aa4fa:     	addq	$0x108, %rsp            # imm = 0x108
1422aa501:     	popq	%rbx
1422aa502:     	popq	%rdi
1422aa503:     	popq	%rsi
1422aa504:     	popq	%r12
1422aa506:     	popq	%r13
1422aa508:     	popq	%r14
1422aa50a:     	popq	%r15
1422aa50c:     	popq	%rbp
1422aa50d:     	retq
1422aa50e:     	nop
1422aa510:     	movq	%rdx, 0x10(%rsp)
1422aa515:     	pushq	%rbp
1422aa516:     	pushq	%r15
1422aa518:     	pushq	%r14
1422aa51a:     	pushq	%r13
1422aa51c:     	pushq	%r12
1422aa51e:     	pushq	%rsi
1422aa51f:     	pushq	%rdi
1422aa520:     	pushq	%rbx
1422aa521:     	subq	$0x108, %rsp            # imm = 0x108
1422aa528:     	leaq	0x80(%rdx), %rbp
1422aa52f:     	movaps	%xmm15, 0x60(%rsp)
1422aa535:     	movaps	%xmm14, 0x70(%rsp)
1422aa53b:     	movaps	%xmm13, 0x80(%rsp)
1422aa544:     	movaps	%xmm12, 0x90(%rsp)
1422aa54d:     	movaps	%xmm11, 0xa0(%rsp)
1422aa556:     	movaps	%xmm10, 0xb0(%rsp)
1422aa55f:     	movaps	%xmm9, 0xc0(%rsp)
1422aa568:     	movaps	%xmm8, 0xd0(%rsp)
1422aa571:     	movaps	%xmm7, 0xe0(%rsp)
1422aa579:     	movaps	%xmm6, 0xf0(%rsp)
1422aa581:     	movl	$0x15, %edx
1422aa586:     	movl	$0x1, %r8d
1422aa58c:     	movq	0x1f8(%rbp), %rcx
1422aa593:     	callq	0x140613c20 <.text+0x612c20>
1422aa598:     	movaps	0xf0(%rsp), %xmm6
1422aa5a0:     	movaps	0xe0(%rsp), %xmm7
1422aa5a8:     	movaps	0xd0(%rsp), %xmm8
1422aa5b1:     	movaps	0xc0(%rsp), %xmm9
1422aa5ba:     	movaps	0xb0(%rsp), %xmm10
1422aa5c3:     	movaps	0xa0(%rsp), %xmm11
1422aa5cc:     	movaps	0x90(%rsp), %xmm12
1422aa5d5:     	movaps	0x80(%rsp), %xmm13
1422aa5de:     	movaps	0x70(%rsp), %xmm14
1422aa5e4:     	movaps	0x60(%rsp), %xmm15
1422aa5ea:     	addq	$0x108, %rsp            # imm = 0x108
1422aa5f1:     	popq	%rbx
1422aa5f2:     	popq	%rdi
1422aa5f3:     	popq	%rsi
1422aa5f4:     	popq	%r12
1422aa5f6:     	popq	%r13
1422aa5f8:     	popq	%r14
1422aa5fa:     	popq	%r15
1422aa5fc:     	popq	%rbp
1422aa5fd:     	retq
1422aa5fe:     	nop
1422aa600:     	movq	%rdx, 0x10(%rsp)
1422aa605:     	pushq	%rbp
1422aa606:     	pushq	%r15
1422aa608:     	pushq	%r14
1422aa60a:     	pushq	%r13
1422aa60c:     	pushq	%r12
1422aa60e:     	pushq	%rsi
1422aa60f:     	pushq	%rdi
1422aa610:     	pushq	%rbx
1422aa611:     	subq	$0x108, %rsp            # imm = 0x108
1422aa618:     	leaq	0x80(%rdx), %rbp
1422aa61f:     	movaps	%xmm15, 0x60(%rsp)
1422aa625:     	movaps	%xmm14, 0x70(%rsp)
1422aa62b:     	movaps	%xmm13, 0x80(%rsp)
1422aa634:     	movaps	%xmm12, 0x90(%rsp)
1422aa63d:     	movaps	%xmm11, 0xa0(%rsp)
1422aa646:     	movaps	%xmm10, 0xb0(%rsp)
1422aa64f:     	movaps	%xmm9, 0xc0(%rsp)
1422aa658:     	movaps	%xmm8, 0xd0(%rsp)
1422aa661:     	movaps	%xmm7, 0xe0(%rsp)
1422aa669:     	movaps	%xmm6, 0xf0(%rsp)
1422aa671:     	movl	$0x19, %edx
1422aa676:     	movl	$0x1, %r8d
1422aa67c:     	movq	0x1d8(%rbp), %rcx
1422aa683:     	callq	0x140613c20 <.text+0x612c20>
1422aa688:     	movaps	0xf0(%rsp), %xmm6
1422aa690:     	movaps	0xe0(%rsp), %xmm7
1422aa698:     	movaps	0xd0(%rsp), %xmm8
1422aa6a1:     	movaps	0xc0(%rsp), %xmm9
1422aa6aa:     	movaps	0xb0(%rsp), %xmm10
1422aa6b3:     	movaps	0xa0(%rsp), %xmm11
1422aa6bc:     	movaps	0x90(%rsp), %xmm12
1422aa6c5:     	movaps	0x80(%rsp), %xmm13
1422aa6ce:     	movaps	0x70(%rsp), %xmm14
1422aa6d4:     	movaps	0x60(%rsp), %xmm15
1422aa6da:     	addq	$0x108, %rsp            # imm = 0x108
1422aa6e1:     	popq	%rbx
1422aa6e2:     	popq	%rdi
1422aa6e3:     	popq	%rsi
1422aa6e4:     	popq	%r12
1422aa6e6:     	popq	%r13
1422aa6e8:     	popq	%r14
1422aa6ea:     	popq	%r15
1422aa6ec:     	popq	%rbp
1422aa6ed:     	retq
1422aa6ee:     	nop
1422aa6f0:     	movq	%rdx, 0x10(%rsp)
1422aa6f5:     	pushq	%rbp
1422aa6f6:     	pushq	%r15
1422aa6f8:     	pushq	%r14
1422aa6fa:     	pushq	%r13
1422aa6fc:     	pushq	%r12
1422aa6fe:     	pushq	%rsi
1422aa6ff:     	pushq	%rdi
1422aa700:     	pushq	%rbx
1422aa701:     	subq	$0x108, %rsp            # imm = 0x108
1422aa708:     	leaq	0x80(%rdx), %rbp
1422aa70f:     	movaps	%xmm15, 0x60(%rsp)
1422aa715:     	movaps	%xmm14, 0x70(%rsp)
1422aa71b:     	movaps	%xmm13, 0x80(%rsp)
1422aa724:     	movaps	%xmm12, 0x90(%rsp)
1422aa72d:     	movaps	%xmm11, 0xa0(%rsp)
1422aa736:     	movaps	%xmm10, 0xb0(%rsp)
1422aa73f:     	movaps	%xmm9, 0xc0(%rsp)
1422aa748:     	movaps	%xmm8, 0xd0(%rsp)
1422aa751:     	movaps	%xmm7, 0xe0(%rsp)
1422aa759:     	movaps	%xmm6, 0xf0(%rsp)
1422aa761:     	movl	$0x10, %edx
1422aa766:     	movl	$0x1, %r8d
1422aa76c:     	movq	0x200(%rbp), %rcx
1422aa773:     	callq	0x140613c20 <.text+0x612c20>
1422aa778:     	movaps	0xf0(%rsp), %xmm6
1422aa780:     	movaps	0xe0(%rsp), %xmm7
1422aa788:     	movaps	0xd0(%rsp), %xmm8
1422aa791:     	movaps	0xc0(%rsp), %xmm9
1422aa79a:     	movaps	0xb0(%rsp), %xmm10
1422aa7a3:     	movaps	0xa0(%rsp), %xmm11
1422aa7ac:     	movaps	0x90(%rsp), %xmm12
1422aa7b5:     	movaps	0x80(%rsp), %xmm13
1422aa7be:     	movaps	0x70(%rsp), %xmm14
1422aa7c4:     	movaps	0x60(%rsp), %xmm15
1422aa7ca:     	addq	$0x108, %rsp            # imm = 0x108
1422aa7d1:     	popq	%rbx
1422aa7d2:     	popq	%rdi
1422aa7d3:     	popq	%rsi
1422aa7d4:     	popq	%r12
1422aa7d6:     	popq	%r13
1422aa7d8:     	popq	%r14
1422aa7da:     	popq	%r15
1422aa7dc:     	popq	%rbp
1422aa7dd:     	retq
1422aa7de:     	nop
1422aa7e0:     	movq	%rdx, 0x10(%rsp)
1422aa7e5:     	pushq	%rbp
1422aa7e6:     	pushq	%r15
1422aa7e8:     	pushq	%r14
1422aa7ea:     	pushq	%r13
1422aa7ec:     	pushq	%r12
1422aa7ee:     	pushq	%rsi
1422aa7ef:     	pushq	%rdi
1422aa7f0:     	pushq	%rbx
1422aa7f1:     	subq	$0x108, %rsp            # imm = 0x108
1422aa7f8:     	leaq	0x80(%rdx), %rbp
1422aa7ff:     	movaps	%xmm15, 0x60(%rsp)
1422aa805:     	movaps	%xmm14, 0x70(%rsp)
1422aa80b:     	movaps	%xmm13, 0x80(%rsp)
1422aa814:     	movaps	%xmm12, 0x90(%rsp)
1422aa81d:     	movaps	%xmm11, 0xa0(%rsp)
1422aa826:     	movaps	%xmm10, 0xb0(%rsp)
1422aa82f:     	movaps	%xmm9, 0xc0(%rsp)
1422aa838:     	movaps	%xmm8, 0xd0(%rsp)
1422aa841:     	movaps	%xmm7, 0xe0(%rsp)
1422aa849:     	movaps	%xmm6, 0xf0(%rsp)
1422aa851:     	movl	$0x16, %edx
1422aa856:     	movl	$0x1, %r8d
1422aa85c:     	movq	0x1b0(%rbp), %rcx
1422aa863:     	callq	0x140613c20 <.text+0x612c20>
1422aa868:     	movaps	0xf0(%rsp), %xmm6
1422aa870:     	movaps	0xe0(%rsp), %xmm7
1422aa878:     	movaps	0xd0(%rsp), %xmm8
1422aa881:     	movaps	0xc0(%rsp), %xmm9
1422aa88a:     	movaps	0xb0(%rsp), %xmm10
1422aa893:     	movaps	0xa0(%rsp), %xmm11
1422aa89c:     	movaps	0x90(%rsp), %xmm12
1422aa8a5:     	movaps	0x80(%rsp), %xmm13
1422aa8ae:     	movaps	0x70(%rsp), %xmm14
1422aa8b4:     	movaps	0x60(%rsp), %xmm15
1422aa8ba:     	addq	$0x108, %rsp            # imm = 0x108
1422aa8c1:     	popq	%rbx
1422aa8c2:     	popq	%rdi
1422aa8c3:     	popq	%rsi
1422aa8c4:     	popq	%r12
1422aa8c6:     	popq	%r13
1422aa8c8:     	popq	%r14
1422aa8ca:     	popq	%r15
1422aa8cc:     	popq	%rbp
1422aa8cd:     	retq
1422aa8ce:     	nop
1422aa8d0:     	movq	%rdx, 0x10(%rsp)
1422aa8d5:     	pushq	%rbp
1422aa8d6:     	pushq	%r15
1422aa8d8:     	pushq	%r14
1422aa8da:     	pushq	%r13
1422aa8dc:     	pushq	%r12
1422aa8de:     	pushq	%rsi
1422aa8df:     	pushq	%rdi
1422aa8e0:     	pushq	%rbx
1422aa8e1:     	subq	$0x108, %rsp            # imm = 0x108
1422aa8e8:     	leaq	0x80(%rdx), %rbp
1422aa8ef:     	movaps	%xmm15, 0x60(%rsp)
1422aa8f5:     	movaps	%xmm14, 0x70(%rsp)
1422aa8fb:     	movaps	%xmm13, 0x80(%rsp)
1422aa904:     	movaps	%xmm12, 0x90(%rsp)
1422aa90d:     	movaps	%xmm11, 0xa0(%rsp)
1422aa916:     	movaps	%xmm10, 0xb0(%rsp)
1422aa91f:     	movaps	%xmm9, 0xc0(%rsp)
1422aa928:     	movaps	%xmm8, 0xd0(%rsp)
1422aa931:     	movaps	%xmm7, 0xe0(%rsp)
1422aa939:     	movaps	%xmm6, 0xf0(%rsp)
1422aa941:     	movl	$0x17, %edx
1422aa946:     	movl	$0x1, %r8d
1422aa94c:     	movq	0x1c0(%rbp), %rcx
1422aa953:     	callq	0x140613c20 <.text+0x612c20>
1422aa958:     	movaps	0xf0(%rsp), %xmm6
1422aa960:     	movaps	0xe0(%rsp), %xmm7
1422aa968:     	movaps	0xd0(%rsp), %xmm8
1422aa971:     	movaps	0xc0(%rsp), %xmm9
1422aa97a:     	movaps	0xb0(%rsp), %xmm10
1422aa983:     	movaps	0xa0(%rsp), %xmm11
1422aa98c:     	movaps	0x90(%rsp), %xmm12
1422aa995:     	movaps	0x80(%rsp), %xmm13
1422aa99e:     	movaps	0x70(%rsp), %xmm14
1422aa9a4:     	movaps	0x60(%rsp), %xmm15
1422aa9aa:     	addq	$0x108, %rsp            # imm = 0x108
1422aa9b1:     	popq	%rbx
1422aa9b2:     	popq	%rdi
1422aa9b3:     	popq	%rsi
1422aa9b4:     	popq	%r12
1422aa9b6:     	popq	%r13
1422aa9b8:     	popq	%r14
1422aa9ba:     	popq	%r15
1422aa9bc:     	popq	%rbp
1422aa9bd:     	retq
1422aa9be:     	nop
1422aa9c0:     	movq	%rdx, 0x10(%rsp)
1422aa9c5:     	pushq	%rbp
1422aa9c6:     	pushq	%r15
1422aa9c8:     	pushq	%r14
1422aa9ca:     	pushq	%r13
1422aa9cc:     	pushq	%r12
1422aa9ce:     	pushq	%rsi
1422aa9cf:     	pushq	%rdi
1422aa9d0:     	pushq	%rbx
1422aa9d1:     	subq	$0x108, %rsp            # imm = 0x108
1422aa9d8:     	leaq	0x80(%rdx), %rbp
1422aa9df:     	movaps	%xmm15, 0x60(%rsp)
1422aa9e5:     	movaps	%xmm14, 0x70(%rsp)
1422aa9eb:     	movaps	%xmm13, 0x80(%rsp)
1422aa9f4:     	movaps	%xmm12, 0x90(%rsp)
1422aa9fd:     	movaps	%xmm11, 0xa0(%rsp)
1422aaa06:     	movaps	%xmm10, 0xb0(%rsp)
1422aaa0f:     	movaps	%xmm9, 0xc0(%rsp)
1422aaa18:     	movaps	%xmm8, 0xd0(%rsp)
1422aaa21:     	movaps	%xmm7, 0xe0(%rsp)
1422aaa29:     	movaps	%xmm6, 0xf0(%rsp)
1422aaa31:     	movl	$0x18, %edx
1422aaa36:     	movl	$0x1, %r8d
1422aaa3c:     	movq	0x1e0(%rbp), %rcx
1422aaa43:     	callq	0x140613c20 <.text+0x612c20>
1422aaa48:     	movaps	0xf0(%rsp), %xmm6
1422aaa50:     	movaps	0xe0(%rsp), %xmm7
1422aaa58:     	movaps	0xd0(%rsp), %xmm8
1422aaa61:     	movaps	0xc0(%rsp), %xmm9
1422aaa6a:     	movaps	0xb0(%rsp), %xmm10
1422aaa73:     	movaps	0xa0(%rsp), %xmm11
1422aaa7c:     	movaps	0x90(%rsp), %xmm12
1422aaa85:     	movaps	0x80(%rsp), %xmm13
1422aaa8e:     	movaps	0x70(%rsp), %xmm14
1422aaa94:     	movaps	0x60(%rsp), %xmm15
1422aaa9a:     	addq	$0x108, %rsp            # imm = 0x108
1422aaaa1:     	popq	%rbx
1422aaaa2:     	popq	%rdi
1422aaaa3:     	popq	%rsi
1422aaaa4:     	popq	%r12
1422aaaa6:     	popq	%r13
1422aaaa8:     	popq	%r14
1422aaaaa:     	popq	%r15
1422aaaac:     	popq	%rbp
1422aaaad:     	retq
1422aaaae:     	nop
1422aaab0:     	movq	%rdx, 0x10(%rsp)
1422aaab5:     	pushq	%rbp
1422aaab6:     	pushq	%r15
1422aaab8:     	pushq	%r14
1422aaaba:     	pushq	%r13
1422aaabc:     	pushq	%r12
1422aaabe:     	pushq	%rsi
1422aaabf:     	pushq	%rdi
1422aaac0:     	pushq	%rbx
1422aaac1:     	subq	$0x108, %rsp            # imm = 0x108
1422aaac8:     	leaq	0x80(%rdx), %rbp
1422aaacf:     	movaps	%xmm15, 0x60(%rsp)
1422aaad5:     	movaps	%xmm14, 0x70(%rsp)
1422aaadb:     	movaps	%xmm13, 0x80(%rsp)
1422aaae4:     	movaps	%xmm12, 0x90(%rsp)
1422aaaed:     	movaps	%xmm11, 0xa0(%rsp)
1422aaaf6:     	movaps	%xmm10, 0xb0(%rsp)
1422aaaff:     	movaps	%xmm9, 0xc0(%rsp)
1422aab08:     	movaps	%xmm8, 0xd0(%rsp)
1422aab11:     	movaps	%xmm7, 0xe0(%rsp)
1422aab19:     	movaps	%xmm6, 0xf0(%rsp)
1422aab21:     	movl	$0x1b, %edx
1422aab26:     	movl	$0x1, %r8d
1422aab2c:     	movq	0x1e8(%rbp), %rcx
1422aab33:     	callq	0x140613c20 <.text+0x612c20>
1422aab38:     	movaps	0xf0(%rsp), %xmm6
1422aab40:     	movaps	0xe0(%rsp), %xmm7
1422aab48:     	movaps	0xd0(%rsp), %xmm8
1422aab51:     	movaps	0xc0(%rsp), %xmm9
1422aab5a:     	movaps	0xb0(%rsp), %xmm10
1422aab63:     	movaps	0xa0(%rsp), %xmm11
1422aab6c:     	movaps	0x90(%rsp), %xmm12
1422aab75:     	movaps	0x80(%rsp), %xmm13
1422aab7e:     	movaps	0x70(%rsp), %xmm14
1422aab84:     	movaps	0x60(%rsp), %xmm15
1422aab8a:     	addq	$0x108, %rsp            # imm = 0x108
1422aab91:     	popq	%rbx
1422aab92:     	popq	%rdi
1422aab93:     	popq	%rsi
1422aab94:     	popq	%r12
1422aab96:     	popq	%r13
1422aab98:     	popq	%r14
1422aab9a:     	popq	%r15
1422aab9c:     	popq	%rbp
1422aab9d:     	retq
1422aab9e:     	nop
1422aaba0:     	movq	%rdx, 0x10(%rsp)
1422aaba5:     	pushq	%rbp
1422aaba6:     	pushq	%r15
1422aaba8:     	pushq	%r14
1422aabaa:     	pushq	%r13
1422aabac:     	pushq	%r12
1422aabae:     	pushq	%rsi
1422aabaf:     	pushq	%rdi
1422aabb0:     	pushq	%rbx
1422aabb1:     	subq	$0x108, %rsp            # imm = 0x108
1422aabb8:     	leaq	0x80(%rdx), %rbp
1422aabbf:     	movaps	%xmm15, 0x60(%rsp)
1422aabc5:     	movaps	%xmm14, 0x70(%rsp)
1422aabcb:     	movaps	%xmm13, 0x80(%rsp)
1422aabd4:     	movaps	%xmm12, 0x90(%rsp)
1422aabdd:     	movaps	%xmm11, 0xa0(%rsp)
1422aabe6:     	movaps	%xmm10, 0xb0(%rsp)
1422aabef:     	movaps	%xmm9, 0xc0(%rsp)
1422aabf8:     	movaps	%xmm8, 0xd0(%rsp)
1422aac01:     	movaps	%xmm7, 0xe0(%rsp)
1422aac09:     	movaps	%xmm6, 0xf0(%rsp)
1422aac11:     	movq	0xd8(%rbp), %rdx
1422aac18:     	testq	%rdx, %rdx
1422aac1b:     	je	0x1422aac33 <.text+0x22a9c33>
1422aac1d:     	movq	0xe0(%rbp), %rcx
1422aac24:     	shlq	$0x3, %rdx
1422aac28:     	movl	$0x8, %r8d
1422aac2e:     	callq	0x140613c20 <.text+0x612c20>
1422aac33:     	movaps	0xf0(%rsp), %xmm6
1422aac3b:     	movaps	0xe0(%rsp), %xmm7
1422aac43:     	movaps	0xd0(%rsp), %xmm8
1422aac4c:     	movaps	0xc0(%rsp), %xmm9
1422aac55:     	movaps	0xb0(%rsp), %xmm10
1422aac5e:     	movaps	0xa0(%rsp), %xmm11
1422aac67:     	movaps	0x90(%rsp), %xmm12
1422aac70:     	movaps	0x80(%rsp), %xmm13
1422aac79:     	movaps	0x70(%rsp), %xmm14
1422aac7f:     	movaps	0x60(%rsp), %xmm15
1422aac85:     	addq	$0x108, %rsp            # imm = 0x108
1422aac8c:     	popq	%rbx
1422aac8d:     	popq	%rdi
1422aac8e:     	popq	%rsi
1422aac8f:     	popq	%r12
1422aac91:     	popq	%r13
1422aac93:     	popq	%r14
1422aac95:     	popq	%r15
1422aac97:     	popq	%rbp
1422aac98:     	retq
1422aac99:     	nopl	(%rax)
1422aaca0:     	movq	%rdx, 0x10(%rsp)
1422aaca5:     	pushq	%rbp
1422aaca6:     	pushq	%r15
1422aaca8:     	pushq	%r14
1422aacaa:     	pushq	%r13
1422aacac:     	pushq	%r12
1422aacae:     	pushq	%rsi
1422aacaf:     	pushq	%rdi
1422aacb0:     	pushq	%rbx
1422aacb1:     	subq	$0x108, %rsp            # imm = 0x108
1422aacb8:     	leaq	0x80(%rdx), %rbp
1422aacbf:     	movaps	%xmm15, 0x60(%rsp)
1422aacc5:     	movaps	%xmm14, 0x70(%rsp)
1422aaccb:     	movaps	%xmm13, 0x80(%rsp)
1422aacd4:     	movaps	%xmm12, 0x90(%rsp)
1422aacdd:     	movaps	%xmm11, 0xa0(%rsp)
1422aace6:     	movaps	%xmm10, 0xb0(%rsp)
1422aacef:     	movaps	%xmm9, 0xc0(%rsp)
1422aacf8:     	movaps	%xmm8, 0xd0(%rsp)
1422aad01:     	movaps	%xmm7, 0xe0(%rsp)
1422aad09:     	movaps	%xmm6, 0xf0(%rsp)
1422aad11:     	movl	$0xc8, %edx
1422aad16:     	movl	$0x8, %r8d
1422aad1c:     	movq	0x1f0(%rbp), %rcx
1422aad23:     	callq	0x140613c20 <.text+0x612c20>
1422aad28:     	movaps	0xf0(%rsp), %xmm6
1422aad30:     	movaps	0xe0(%rsp), %xmm7
1422aad38:     	movaps	0xd0(%rsp), %xmm8
1422aad41:     	movaps	0xc0(%rsp), %xmm9
1422aad4a:     	movaps	0xb0(%rsp), %xmm10
1422aad53:     	movaps	0xa0(%rsp), %xmm11
1422aad5c:     	movaps	0x90(%rsp), %xmm12
1422aad65:     	movaps	0x80(%rsp), %xmm13
1422aad6e:     	movaps	0x70(%rsp), %xmm14
1422aad74:     	movaps	0x60(%rsp), %xmm15
1422aad7a:     	addq	$0x108, %rsp            # imm = 0x108
1422aad81:     	popq	%rbx
1422aad82:     	popq	%rdi
1422aad83:     	popq	%rsi
1422aad84:     	popq	%r12
1422aad86:     	popq	%r13
1422aad88:     	popq	%r14
1422aad8a:     	popq	%r15
1422aad8c:     	popq	%rbp
1422aad8d:     	retq
1422aad8e:     	nop
1422aad90:     	movq	%rdx, 0x10(%rsp)
1422aad95:     	pushq	%rbp
1422aad96:     	pushq	%r15
1422aad98:     	pushq	%r14
1422aad9a:     	pushq	%r13
1422aad9c:     	pushq	%r12
1422aad9e:     	pushq	%rsi
1422aad9f:     	pushq	%rdi
1422aada0:     	pushq	%rbx
1422aada1:     	subq	$0x108, %rsp            # imm = 0x108
1422aada8:     	leaq	0x80(%rdx), %rbp
1422aadaf:     	movaps	%xmm15, 0x60(%rsp)
1422aadb5:     	movaps	%xmm14, 0x70(%rsp)
1422aadbb:     	movaps	%xmm13, 0x80(%rsp)
1422aadc4:     	movaps	%xmm12, 0x90(%rsp)
1422aadcd:     	movaps	%xmm11, 0xa0(%rsp)
1422aadd6:     	movaps	%xmm10, 0xb0(%rsp)
1422aaddf:     	movaps	%xmm9, 0xc0(%rsp)
1422aade8:     	movaps	%xmm8, 0xd0(%rsp)
1422aadf1:     	movaps	%xmm7, 0xe0(%rsp)
1422aadf9:     	movaps	%xmm6, 0xf0(%rsp)
1422aae01:     	movl	$0x11, %edx
1422aae06:     	movl	$0x1, %r8d
1422aae0c:     	movq	0x178(%rbp), %rcx
1422aae13:     	callq	0x140613c20 <.text+0x612c20>
1422aae18:     	movaps	0xf0(%rsp), %xmm6
1422aae20:     	movaps	0xe0(%rsp), %xmm7
1422aae28:     	movaps	0xd0(%rsp), %xmm8
1422aae31:     	movaps	0xc0(%rsp), %xmm9
1422aae3a:     	movaps	0xb0(%rsp), %xmm10
1422aae43:     	movaps	0xa0(%rsp), %xmm11
1422aae4c:     	movaps	0x90(%rsp), %xmm12
1422aae55:     	movaps	0x80(%rsp), %xmm13
1422aae5e:     	movaps	0x70(%rsp), %xmm14
1422aae64:     	movaps	0x60(%rsp), %xmm15
1422aae6a:     	addq	$0x108, %rsp            # imm = 0x108
1422aae71:     	popq	%rbx
1422aae72:     	popq	%rdi
1422aae73:     	popq	%rsi
1422aae74:     	popq	%r12
1422aae76:     	popq	%r13
1422aae78:     	popq	%r14
1422aae7a:     	popq	%r15
1422aae7c:     	popq	%rbp
1422aae7d:     	retq
1422aae7e:     	nop
1422aae80:     	movq	%rdx, 0x10(%rsp)
1422aae85:     	pushq	%rbp
1422aae86:     	pushq	%r15
1422aae88:     	pushq	%r14
1422aae8a:     	pushq	%r13
1422aae8c:     	pushq	%r12
1422aae8e:     	pushq	%rsi
1422aae8f:     	pushq	%rdi
1422aae90:     	pushq	%rbx
1422aae91:     	subq	$0x108, %rsp            # imm = 0x108
1422aae98:     	leaq	0x80(%rdx), %rbp
1422aae9f:     	movaps	%xmm15, 0x60(%rsp)
1422aaea5:     	movaps	%xmm14, 0x70(%rsp)
1422aaeab:     	movaps	%xmm13, 0x80(%rsp)
1422aaeb4:     	movaps	%xmm12, 0x90(%rsp)
1422aaebd:     	movaps	%xmm11, 0xa0(%rsp)
1422aaec6:     	movaps	%xmm10, 0xb0(%rsp)
1422aaecf:     	movaps	%xmm9, 0xc0(%rsp)
1422aaed8:     	movaps	%xmm8, 0xd0(%rsp)
1422aaee1:     	movaps	%xmm7, 0xe0(%rsp)
1422aaee9:     	movaps	%xmm6, 0xf0(%rsp)
1422aaef1:     	movl	$0x1b8, %edx            # imm = 0x1B8
1422aaef6:     	movl	$0x8, %r8d
1422aaefc:     	movq	0x200(%rbp), %rcx
1422aaf03:     	callq	0x140613c20 <.text+0x612c20>
1422aaf08:     	movaps	0xf0(%rsp), %xmm6
1422aaf10:     	movaps	0xe0(%rsp), %xmm7
1422aaf18:     	movaps	0xd0(%rsp), %xmm8
1422aaf21:     	movaps	0xc0(%rsp), %xmm9
1422aaf2a:     	movaps	0xb0(%rsp), %xmm10
1422aaf33:     	movaps	0xa0(%rsp), %xmm11
1422aaf3c:     	movaps	0x90(%rsp), %xmm12
1422aaf45:     	movaps	0x80(%rsp), %xmm13
1422aaf4e:     	movaps	0x70(%rsp), %xmm14
1422aaf54:     	movaps	0x60(%rsp), %xmm15
1422aaf5a:     	addq	$0x108, %rsp            # imm = 0x108
1422aaf61:     	popq	%rbx
1422aaf62:     	popq	%rdi
1422aaf63:     	popq	%rsi
1422aaf64:     	popq	%r12
1422aaf66:     	popq	%r13
1422aaf68:     	popq	%r14
1422aaf6a:     	popq	%r15
1422aaf6c:     	popq	%rbp
1422aaf6d:     	retq
1422aaf6e:     	nop
1422aaf70:     	movq	%rdx, 0x10(%rsp)
1422aaf75:     	pushq	%rbp
1422aaf76:     	pushq	%r15
1422aaf78:     	pushq	%r14
1422aaf7a:     	pushq	%r13
1422aaf7c:     	pushq	%r12
1422aaf7e:     	pushq	%rsi
1422aaf7f:     	pushq	%rdi
1422aaf80:     	pushq	%rbx
1422aaf81:     	subq	$0x108, %rsp            # imm = 0x108
1422aaf88:     	leaq	0x80(%rdx), %rbp
1422aaf8f:     	movaps	%xmm15, 0x60(%rsp)
1422aaf95:     	movaps	%xmm14, 0x70(%rsp)
1422aaf9b:     	movaps	%xmm13, 0x80(%rsp)
1422aafa4:     	movaps	%xmm12, 0x90(%rsp)
1422aafad:     	movaps	%xmm11, 0xa0(%rsp)
1422aafb6:     	movaps	%xmm10, 0xb0(%rsp)
1422aafbf:     	movaps	%xmm9, 0xc0(%rsp)
1422aafc8:     	movaps	%xmm8, 0xd0(%rsp)
1422aafd1:     	movaps	%xmm7, 0xe0(%rsp)
1422aafd9:     	movaps	%xmm6, 0xf0(%rsp)
1422aafe1:     	movl	$0x17, %edx
1422aafe6:     	movl	$0x1, %r8d
1422aafec:     	movq	0x1f8(%rbp), %rcx
1422aaff3:     	callq	0x140613c20 <.text+0x612c20>
1422aaff8:     	movaps	0xf0(%rsp), %xmm6
1422ab000:     	movaps	0xe0(%rsp), %xmm7
1422ab008:     	movaps	0xd0(%rsp), %xmm8
1422ab011:     	movaps	0xc0(%rsp), %xmm9
1422ab01a:     	movaps	0xb0(%rsp), %xmm10
1422ab023:     	movaps	0xa0(%rsp), %xmm11
1422ab02c:     	movaps	0x90(%rsp), %xmm12
1422ab035:     	movaps	0x80(%rsp), %xmm13
1422ab03e:     	movaps	0x70(%rsp), %xmm14
1422ab044:     	movaps	0x60(%rsp), %xmm15
1422ab04a:     	addq	$0x108, %rsp            # imm = 0x108
1422ab051:     	popq	%rbx
1422ab052:     	popq	%rdi
1422ab053:     	popq	%rsi
1422ab054:     	popq	%r12
1422ab056:     	popq	%r13
1422ab058:     	popq	%r14
1422ab05a:     	popq	%r15
1422ab05c:     	popq	%rbp
1422ab05d:     	retq
1422ab05e:     	nop
1422ab060:     	movq	%rdx, 0x10(%rsp)
1422ab065:     	pushq	%rbp
1422ab066:     	pushq	%r15
1422ab068:     	pushq	%r14
1422ab06a:     	pushq	%r13
1422ab06c:     	pushq	%r12
1422ab06e:     	pushq	%rsi
1422ab06f:     	pushq	%rdi
1422ab070:     	pushq	%rbx
1422ab071:     	subq	$0x108, %rsp            # imm = 0x108
1422ab078:     	leaq	0x80(%rdx), %rbp
1422ab07f:     	movaps	%xmm15, 0x60(%rsp)
1422ab085:     	movaps	%xmm14, 0x70(%rsp)
1422ab08b:     	movaps	%xmm13, 0x80(%rsp)
1422ab094:     	movaps	%xmm12, 0x90(%rsp)
1422ab09d:     	movaps	%xmm11, 0xa0(%rsp)
1422ab0a6:     	movaps	%xmm10, 0xb0(%rsp)
1422ab0af:     	movaps	%xmm9, 0xc0(%rsp)
1422ab0b8:     	movaps	%xmm8, 0xd0(%rsp)
1422ab0c1:     	movaps	%xmm7, 0xe0(%rsp)
1422ab0c9:     	movaps	%xmm6, 0xf0(%rsp)
1422ab0d1:     	movl	$0x11, %edx
1422ab0d6:     	movl	$0x1, %r8d
1422ab0dc:     	movq	0x1b0(%rbp), %rcx
1422ab0e3:     	callq	0x140613c20 <.text+0x612c20>
1422ab0e8:     	movaps	0xf0(%rsp), %xmm6
1422ab0f0:     	movaps	0xe0(%rsp), %xmm7
1422ab0f8:     	movaps	0xd0(%rsp), %xmm8
1422ab101:     	movaps	0xc0(%rsp), %xmm9
1422ab10a:     	movaps	0xb0(%rsp), %xmm10
1422ab113:     	movaps	0xa0(%rsp), %xmm11
1422ab11c:     	movaps	0x90(%rsp), %xmm12
1422ab125:     	movaps	0x80(%rsp), %xmm13
1422ab12e:     	movaps	0x70(%rsp), %xmm14
1422ab134:     	movaps	0x60(%rsp), %xmm15
1422ab13a:     	addq	$0x108, %rsp            # imm = 0x108
1422ab141:     	popq	%rbx
1422ab142:     	popq	%rdi
1422ab143:     	popq	%rsi
1422ab144:     	popq	%r12
1422ab146:     	popq	%r13
1422ab148:     	popq	%r14
1422ab14a:     	popq	%r15
1422ab14c:     	popq	%rbp
1422ab14d:     	retq
1422ab14e:     	nop
1422ab150:     	movq	%rdx, 0x10(%rsp)
1422ab155:     	pushq	%rbp
1422ab156:     	pushq	%r15
1422ab158:     	pushq	%r14
1422ab15a:     	pushq	%r13
1422ab15c:     	pushq	%r12
1422ab15e:     	pushq	%rsi
1422ab15f:     	pushq	%rdi
1422ab160:     	pushq	%rbx
1422ab161:     	subq	$0x108, %rsp            # imm = 0x108
1422ab168:     	leaq	0x80(%rdx), %rbp
1422ab16f:     	movaps	%xmm15, 0x60(%rsp)
1422ab175:     	movaps	%xmm14, 0x70(%rsp)
1422ab17b:     	movaps	%xmm13, 0x80(%rsp)
1422ab184:     	movaps	%xmm12, 0x90(%rsp)
1422ab18d:     	movaps	%xmm11, 0xa0(%rsp)
1422ab196:     	movaps	%xmm10, 0xb0(%rsp)
1422ab19f:     	movaps	%xmm9, 0xc0(%rsp)
1422ab1a8:     	movaps	%xmm8, 0xd0(%rsp)
1422ab1b1:     	movaps	%xmm7, 0xe0(%rsp)
1422ab1b9:     	movaps	%xmm6, 0xf0(%rsp)
1422ab1c1:     	movl	$0x5, %edx
1422ab1c6:     	movl	$0x1, %r8d
1422ab1cc:     	movq	0x1e8(%rbp), %rcx
1422ab1d3:     	callq	0x140613c20 <.text+0x612c20>
1422ab1d8:     	movaps	0xf0(%rsp), %xmm6
1422ab1e0:     	movaps	0xe0(%rsp), %xmm7
1422ab1e8:     	movaps	0xd0(%rsp), %xmm8
1422ab1f1:     	movaps	0xc0(%rsp), %xmm9
1422ab1fa:     	movaps	0xb0(%rsp), %xmm10
1422ab203:     	movaps	0xa0(%rsp), %xmm11
1422ab20c:     	movaps	0x90(%rsp), %xmm12
1422ab215:     	movaps	0x80(%rsp), %xmm13
1422ab21e:     	movaps	0x70(%rsp), %xmm14
1422ab224:     	movaps	0x60(%rsp), %xmm15
1422ab22a:     	addq	$0x108, %rsp            # imm = 0x108
1422ab231:     	popq	%rbx
1422ab232:     	popq	%rdi
1422ab233:     	popq	%rsi
1422ab234:     	popq	%r12
1422ab236:     	popq	%r13
1422ab238:     	popq	%r14
1422ab23a:     	popq	%r15
1422ab23c:     	popq	%rbp
1422ab23d:     	retq
1422ab23e:     	nop
1422ab240:     	movq	%rdx, 0x10(%rsp)
1422ab245:     	pushq	%rbp
1422ab246:     	pushq	%r15
1422ab248:     	pushq	%r14
1422ab24a:     	pushq	%r13
1422ab24c:     	pushq	%r12
1422ab24e:     	pushq	%rsi
1422ab24f:     	pushq	%rdi
1422ab250:     	pushq	%rbx
1422ab251:     	subq	$0x108, %rsp            # imm = 0x108
1422ab258:     	leaq	0x80(%rdx), %rbp
1422ab25f:     	movaps	%xmm15, 0x60(%rsp)
1422ab265:     	movaps	%xmm14, 0x70(%rsp)
1422ab26b:     	movaps	%xmm13, 0x80(%rsp)
1422ab274:     	movaps	%xmm12, 0x90(%rsp)
1422ab27d:     	movaps	%xmm11, 0xa0(%rsp)
1422ab286:     	movaps	%xmm10, 0xb0(%rsp)
1422ab28f:     	movaps	%xmm9, 0xc0(%rsp)
1422ab298:     	movaps	%xmm8, 0xd0(%rsp)
1422ab2a1:     	movaps	%xmm7, 0xe0(%rsp)
1422ab2a9:     	movaps	%xmm6, 0xf0(%rsp)
1422ab2b1:     	movl	$0x16, %edx
1422ab2b6:     	movl	$0x1, %r8d
1422ab2bc:     	movq	0x1e8(%rbp), %rcx
1422ab2c3:     	callq	0x140613c20 <.text+0x612c20>
1422ab2c8:     	movaps	0xf0(%rsp), %xmm6
1422ab2d0:     	movaps	0xe0(%rsp), %xmm7
1422ab2d8:     	movaps	0xd0(%rsp), %xmm8
1422ab2e1:     	movaps	0xc0(%rsp), %xmm9
1422ab2ea:     	movaps	0xb0(%rsp), %xmm10
1422ab2f3:     	movaps	0xa0(%rsp), %xmm11
1422ab2fc:     	movaps	0x90(%rsp), %xmm12
1422ab305:     	movaps	0x80(%rsp), %xmm13
1422ab30e:     	movaps	0x70(%rsp), %xmm14
1422ab314:     	movaps	0x60(%rsp), %xmm15
1422ab31a:     	addq	$0x108, %rsp            # imm = 0x108
1422ab321:     	popq	%rbx
1422ab322:     	popq	%rdi
1422ab323:     	popq	%rsi
1422ab324:     	popq	%r12
1422ab326:     	popq	%r13
1422ab328:     	popq	%r14
1422ab32a:     	popq	%r15
1422ab32c:     	popq	%rbp
1422ab32d:     	retq
1422ab32e:     	nop
1422ab330:     	movq	%rdx, 0x10(%rsp)
1422ab335:     	pushq	%rbp
1422ab336:     	pushq	%r15
1422ab338:     	pushq	%r14
1422ab33a:     	pushq	%r13
1422ab33c:     	pushq	%r12
1422ab33e:     	pushq	%rsi
1422ab33f:     	pushq	%rdi
1422ab340:     	pushq	%rbx
1422ab341:     	subq	$0x108, %rsp            # imm = 0x108
1422ab348:     	leaq	0x80(%rdx), %rbp
1422ab34f:     	movaps	%xmm15, 0x60(%rsp)
1422ab355:     	movaps	%xmm14, 0x70(%rsp)
1422ab35b:     	movaps	%xmm13, 0x80(%rsp)
1422ab364:     	movaps	%xmm12, 0x90(%rsp)
1422ab36d:     	movaps	%xmm11, 0xa0(%rsp)
1422ab376:     	movaps	%xmm10, 0xb0(%rsp)
1422ab37f:     	movaps	%xmm9, 0xc0(%rsp)
1422ab388:     	movaps	%xmm8, 0xd0(%rsp)
1422ab391:     	movaps	%xmm7, 0xe0(%rsp)
1422ab399:     	movaps	%xmm6, 0xf0(%rsp)
1422ab3a1:     	movl	$0x16, %edx
1422ab3a6:     	movl	$0x1, %r8d
1422ab3ac:     	movq	0x1c8(%rbp), %rcx
1422ab3b3:     	callq	0x140613c20 <.text+0x612c20>
1422ab3b8:     	movaps	0xf0(%rsp), %xmm6
1422ab3c0:     	movaps	0xe0(%rsp), %xmm7
1422ab3c8:     	movaps	0xd0(%rsp), %xmm8
1422ab3d1:     	movaps	0xc0(%rsp), %xmm9
1422ab3da:     	movaps	0xb0(%rsp), %xmm10
1422ab3e3:     	movaps	0xa0(%rsp), %xmm11
1422ab3ec:     	movaps	0x90(%rsp), %xmm12
1422ab3f5:     	movaps	0x80(%rsp), %xmm13
1422ab3fe:     	movaps	0x70(%rsp), %xmm14
1422ab404:     	movaps	0x60(%rsp), %xmm15
1422ab40a:     	addq	$0x108, %rsp            # imm = 0x108
1422ab411:     	popq	%rbx
1422ab412:     	popq	%rdi
1422ab413:     	popq	%rsi
1422ab414:     	popq	%r12
1422ab416:     	popq	%r13
1422ab418:     	popq	%r14
1422ab41a:     	popq	%r15
1422ab41c:     	popq	%rbp
1422ab41d:     	retq
1422ab41e:     	nop
1422ab420:     	movq	%rdx, 0x10(%rsp)
1422ab425:     	pushq	%rbp
1422ab426:     	pushq	%r15
1422ab428:     	pushq	%r14
1422ab42a:     	pushq	%r13
1422ab42c:     	pushq	%r12
1422ab42e:     	pushq	%rsi
1422ab42f:     	pushq	%rdi
1422ab430:     	pushq	%rbx
1422ab431:     	subq	$0x108, %rsp            # imm = 0x108
1422ab438:     	leaq	0x80(%rdx), %rbp
1422ab43f:     	movaps	%xmm15, 0x60(%rsp)
1422ab445:     	movaps	%xmm14, 0x70(%rsp)
1422ab44b:     	movaps	%xmm13, 0x80(%rsp)
1422ab454:     	movaps	%xmm12, 0x90(%rsp)
1422ab45d:     	movaps	%xmm11, 0xa0(%rsp)
1422ab466:     	movaps	%xmm10, 0xb0(%rsp)
1422ab46f:     	movaps	%xmm9, 0xc0(%rsp)
1422ab478:     	movaps	%xmm8, 0xd0(%rsp)
1422ab481:     	movaps	%xmm7, 0xe0(%rsp)
1422ab489:     	movaps	%xmm6, 0xf0(%rsp)
1422ab491:     	movl	$0x19, %edx
1422ab496:     	movl	$0x1, %r8d
1422ab49c:     	movq	0x1e0(%rbp), %rcx
1422ab4a3:     	callq	0x140613c20 <.text+0x612c20>
1422ab4a8:     	movaps	0xf0(%rsp), %xmm6
1422ab4b0:     	movaps	0xe0(%rsp), %xmm7
1422ab4b8:     	movaps	0xd0(%rsp), %xmm8
1422ab4c1:     	movaps	0xc0(%rsp), %xmm9
1422ab4ca:     	movaps	0xb0(%rsp), %xmm10
1422ab4d3:     	movaps	0xa0(%rsp), %xmm11
1422ab4dc:     	movaps	0x90(%rsp), %xmm12
1422ab4e5:     	movaps	0x80(%rsp), %xmm13
1422ab4ee:     	movaps	0x70(%rsp), %xmm14
1422ab4f4:     	movaps	0x60(%rsp), %xmm15
1422ab4fa:     	addq	$0x108, %rsp            # imm = 0x108
1422ab501:     	popq	%rbx
1422ab502:     	popq	%rdi
1422ab503:     	popq	%rsi
1422ab504:     	popq	%r12
1422ab506:     	popq	%r13
1422ab508:     	popq	%r14
1422ab50a:     	popq	%r15
1422ab50c:     	popq	%rbp
1422ab50d:     	retq
1422ab50e:     	nop
1422ab510:     	movq	%rdx, 0x10(%rsp)
1422ab515:     	pushq	%rbp
1422ab516:     	pushq	%r15
1422ab518:     	pushq	%r14
1422ab51a:     	pushq	%r13
1422ab51c:     	pushq	%r12
1422ab51e:     	pushq	%rsi
1422ab51f:     	pushq	%rdi
1422ab520:     	pushq	%rbx
1422ab521:     	subq	$0x108, %rsp            # imm = 0x108
1422ab528:     	leaq	0x80(%rdx), %rbp
1422ab52f:     	movaps	%xmm15, 0x60(%rsp)
1422ab535:     	movaps	%xmm14, 0x70(%rsp)
1422ab53b:     	movaps	%xmm13, 0x80(%rsp)
1422ab544:     	movaps	%xmm12, 0x90(%rsp)
1422ab54d:     	movaps	%xmm11, 0xa0(%rsp)
1422ab556:     	movaps	%xmm10, 0xb0(%rsp)
1422ab55f:     	movaps	%xmm9, 0xc0(%rsp)
1422ab568:     	movaps	%xmm8, 0xd0(%rsp)
1422ab571:     	movaps	%xmm7, 0xe0(%rsp)
1422ab579:     	movaps	%xmm6, 0xf0(%rsp)
1422ab581:     	leaq	0x158(%rbp), %rcx
1422ab588:     	callq	0x14014a860 <.text+0x149860>
1422ab58d:     	movaps	0xf0(%rsp), %xmm6
1422ab595:     	movaps	0xe0(%rsp), %xmm7
1422ab59d:     	movaps	0xd0(%rsp), %xmm8
1422ab5a6:     	movaps	0xc0(%rsp), %xmm9
1422ab5af:     	movaps	0xb0(%rsp), %xmm10
1422ab5b8:     	movaps	0xa0(%rsp), %xmm11
1422ab5c1:     	movaps	0x90(%rsp), %xmm12
1422ab5ca:     	movaps	0x80(%rsp), %xmm13
1422ab5d3:     	movaps	0x70(%rsp), %xmm14
1422ab5d9:     	movaps	0x60(%rsp), %xmm15
1422ab5df:     	addq	$0x108, %rsp            # imm = 0x108
1422ab5e6:     	popq	%rbx
1422ab5e7:     	popq	%rdi
1422ab5e8:     	popq	%rsi
1422ab5e9:     	popq	%r12
1422ab5eb:     	popq	%r13
1422ab5ed:     	popq	%r14
1422ab5ef:     	popq	%r15
1422ab5f1:     	popq	%rbp
1422ab5f2:     	retq
1422ab5f3:     	nopw	%cs:(%rax,%rax)
1422ab600:     	movq	%rdx, 0x10(%rsp)
1422ab605:     	pushq	%rbp
1422ab606:     	pushq	%r15
1422ab608:     	pushq	%r14
1422ab60a:     	pushq	%r13
1422ab60c:     	pushq	%r12
1422ab60e:     	pushq	%rsi
1422ab60f:     	pushq	%rdi
1422ab610:     	pushq	%rbx
1422ab611:     	subq	$0x108, %rsp            # imm = 0x108
1422ab618:     	leaq	0x80(%rdx), %rbp
1422ab61f:     	movaps	%xmm15, 0x60(%rsp)
1422ab625:     	movaps	%xmm14, 0x70(%rsp)
1422ab62b:     	movaps	%xmm13, 0x80(%rsp)
1422ab634:     	movaps	%xmm12, 0x90(%rsp)
1422ab63d:     	movaps	%xmm11, 0xa0(%rsp)
1422ab646:     	movaps	%xmm10, 0xb0(%rsp)
1422ab64f:     	movaps	%xmm9, 0xc0(%rsp)
1422ab658:     	movaps	%xmm8, 0xd0(%rsp)
1422ab661:     	movaps	%xmm7, 0xe0(%rsp)
1422ab669:     	movaps	%xmm6, 0xf0(%rsp)
1422ab671:     	movl	$0x208, %edx            # imm = 0x208
1422ab676:     	movl	$0x8, %r8d
1422ab67c:     	movq	0x200(%rbp), %rcx
1422ab683:     	callq	0x140613c20 <.text+0x612c20>
1422ab688:     	movaps	0xf0(%rsp), %xmm6
1422ab690:     	movaps	0xe0(%rsp), %xmm7
1422ab698:     	movaps	0xd0(%rsp), %xmm8
1422ab6a1:     	movaps	0xc0(%rsp), %xmm9
1422ab6aa:     	movaps	0xb0(%rsp), %xmm10
1422ab6b3:     	movaps	0xa0(%rsp), %xmm11
1422ab6bc:     	movaps	0x90(%rsp), %xmm12
1422ab6c5:     	movaps	0x80(%rsp), %xmm13
1422ab6ce:     	movaps	0x70(%rsp), %xmm14
1422ab6d4:     	movaps	0x60(%rsp), %xmm15
1422ab6da:     	addq	$0x108, %rsp            # imm = 0x108
1422ab6e1:     	popq	%rbx
1422ab6e2:     	popq	%rdi
1422ab6e3:     	popq	%rsi
1422ab6e4:     	popq	%r12
1422ab6e6:     	popq	%r13
1422ab6e8:     	popq	%r14
1422ab6ea:     	popq	%r15
1422ab6ec:     	popq	%rbp
1422ab6ed:     	retq
1422ab6ee:     	nop
1422ab6f0:     	movq	%rdx, 0x10(%rsp)
1422ab6f5:     	pushq	%rbp
1422ab6f6:     	pushq	%r15
1422ab6f8:     	pushq	%r14
1422ab6fa:     	pushq	%r13
1422ab6fc:     	pushq	%r12
1422ab6fe:     	pushq	%rsi
1422ab6ff:     	pushq	%rdi
1422ab700:     	pushq	%rbx
1422ab701:     	subq	$0x108, %rsp            # imm = 0x108
1422ab708:     	leaq	0x80(%rdx), %rbp
1422ab70f:     	movaps	%xmm15, 0x60(%rsp)
1422ab715:     	movaps	%xmm14, 0x70(%rsp)
1422ab71b:     	movaps	%xmm13, 0x80(%rsp)
1422ab724:     	movaps	%xmm12, 0x90(%rsp)
1422ab72d:     	movaps	%xmm11, 0xa0(%rsp)
1422ab736:     	movaps	%xmm10, 0xb0(%rsp)
1422ab73f:     	movaps	%xmm9, 0xc0(%rsp)
1422ab748:     	movaps	%xmm8, 0xd0(%rsp)
1422ab751:     	movaps	%xmm7, 0xe0(%rsp)
1422ab759:     	movaps	%xmm6, 0xf0(%rsp)
1422ab761:     	cmpb	$0x0, 0x1bf(%rbp)
1422ab768:     	je	0x1422ab7dc <.text+0x22aa7dc>
1422ab76a:     	leaq	0x158(%rbp), %rcx
1422ab771:     	callq	0x14014a860 <.text+0x149860>
1422ab776:     	movaps	0xf0(%rsp), %xmm6
1422ab77e:     	movaps	0xe0(%rsp), %xmm7
1422ab786:     	movaps	0xd0(%rsp), %xmm8
1422ab78f:     	movaps	0xc0(%rsp), %xmm9
1422ab798:     	movaps	0xb0(%rsp), %xmm10
1422ab7a1:     	movaps	0xa0(%rsp), %xmm11
1422ab7aa:     	movaps	0x90(%rsp), %xmm12
1422ab7b3:     	movaps	0x80(%rsp), %xmm13
1422ab7bc:     	movaps	0x70(%rsp), %xmm14
1422ab7c2:     	movaps	0x60(%rsp), %xmm15
1422ab7c8:     	addq	$0x108, %rsp            # imm = 0x108
1422ab7cf:     	popq	%rbx
1422ab7d0:     	popq	%rdi
1422ab7d1:     	popq	%rsi
1422ab7d2:     	popq	%r12
1422ab7d4:     	popq	%r13
1422ab7d6:     	popq	%r14
1422ab7d8:     	popq	%r15
1422ab7da:     	popq	%rbp
1422ab7db:     	retq
1422ab7dc:     	movaps	0xf0(%rsp), %xmm6
1422ab7e4:     	movaps	0xe0(%rsp), %xmm7
1422ab7ec:     	movaps	0xd0(%rsp), %xmm8
1422ab7f5:     	movaps	0xc0(%rsp), %xmm9
1422ab7fe:     	movaps	0xb0(%rsp), %xmm10
1422ab807:     	movaps	0xa0(%rsp), %xmm11
1422ab810:     	movaps	0x90(%rsp), %xmm12
1422ab819:     	movaps	0x80(%rsp), %xmm13
1422ab822:     	movaps	0x70(%rsp), %xmm14
1422ab828:     	movaps	0x60(%rsp), %xmm15
1422ab82e:     	addq	$0x108, %rsp            # imm = 0x108
1422ab835:     	popq	%rbx
1422ab836:     	popq	%rdi
1422ab837:     	popq	%rsi
1422ab838:     	popq	%r12
1422ab83a:     	popq	%r13
1422ab83c:     	popq	%r14
1422ab83e:     	popq	%r15
1422ab840:     	popq	%rbp
1422ab841:     	retq
1422ab842:     	nopw	%cs:(%rax,%rax)
1422ab850:     	movq	%rdx, 0x10(%rsp)
1422ab855:     	pushq	%rbp
1422ab856:     	pushq	%r15
1422ab858:     	pushq	%r14
1422ab85a:     	pushq	%r13
1422ab85c:     	pushq	%r12
1422ab85e:     	pushq	%rsi
1422ab85f:     	pushq	%rdi
1422ab860:     	pushq	%rbx
1422ab861:     	subq	$0x108, %rsp            # imm = 0x108
1422ab868:     	leaq	0x80(%rdx), %rbp
1422ab86f:     	movaps	%xmm15, 0x60(%rsp)
1422ab875:     	movaps	%xmm14, 0x70(%rsp)
1422ab87b:     	movaps	%xmm13, 0x80(%rsp)
1422ab884:     	movaps	%xmm12, 0x90(%rsp)
1422ab88d:     	movaps	%xmm11, 0xa0(%rsp)
1422ab896:     	movaps	%xmm10, 0xb0(%rsp)
1422ab89f:     	movaps	%xmm9, 0xc0(%rsp)
1422ab8a8:     	movaps	%xmm8, 0xd0(%rsp)
1422ab8b1:     	movaps	%xmm7, 0xe0(%rsp)
1422ab8b9:     	movaps	%xmm6, 0xf0(%rsp)
1422ab8c1:     	movl	$0xf0, %edx
1422ab8c6:     	movl	$0x8, %r8d
1422ab8cc:     	movq	0x1f0(%rbp), %rcx
1422ab8d3:     	callq	0x140613c20 <.text+0x612c20>
1422ab8d8:     	movaps	0xf0(%rsp), %xmm6
1422ab8e0:     	movaps	0xe0(%rsp), %xmm7
1422ab8e8:     	movaps	0xd0(%rsp), %xmm8
1422ab8f1:     	movaps	0xc0(%rsp), %xmm9
1422ab8fa:     	movaps	0xb0(%rsp), %xmm10
1422ab903:     	movaps	0xa0(%rsp), %xmm11
1422ab90c:     	movaps	0x90(%rsp), %xmm12
1422ab915:     	movaps	0x80(%rsp), %xmm13
1422ab91e:     	movaps	0x70(%rsp), %xmm14
1422ab924:     	movaps	0x60(%rsp), %xmm15
1422ab92a:     	addq	$0x108, %rsp            # imm = 0x108
1422ab931:     	popq	%rbx
1422ab932:     	popq	%rdi
1422ab933:     	popq	%rsi
1422ab934:     	popq	%r12
1422ab936:     	popq	%r13
1422ab938:     	popq	%r14
1422ab93a:     	popq	%r15
1422ab93c:     	popq	%rbp
1422ab93d:     	retq
1422ab93e:     	nop
1422ab940:     	movq	%rdx, 0x10(%rsp)
1422ab945:     	pushq	%rbp
1422ab946:     	pushq	%r15
1422ab948:     	pushq	%r14
1422ab94a:     	pushq	%r13
1422ab94c:     	pushq	%r12
1422ab94e:     	pushq	%rsi
1422ab94f:     	pushq	%rdi
1422ab950:     	pushq	%rbx
1422ab951:     	subq	$0x108, %rsp            # imm = 0x108
1422ab958:     	leaq	0x80(%rdx), %rbp
1422ab95f:     	movaps	%xmm15, 0x60(%rsp)
1422ab965:     	movaps	%xmm14, 0x70(%rsp)
1422ab96b:     	movaps	%xmm13, 0x80(%rsp)
1422ab974:     	movaps	%xmm12, 0x90(%rsp)
1422ab97d:     	movaps	%xmm11, 0xa0(%rsp)
1422ab986:     	movaps	%xmm10, 0xb0(%rsp)
1422ab98f:     	movaps	%xmm9, 0xc0(%rsp)
1422ab998:     	movaps	%xmm8, 0xd0(%rsp)
1422ab9a1:     	movaps	%xmm7, 0xe0(%rsp)
1422ab9a9:     	movaps	%xmm6, 0xf0(%rsp)
1422ab9b1:     	movl	$0xe, %edx
1422ab9b6:     	movl	$0x1, %r8d
1422ab9bc:     	movq	0x1c8(%rbp), %rcx
1422ab9c3:     	callq	0x140613c20 <.text+0x612c20>
1422ab9c8:     	movaps	0xf0(%rsp), %xmm6
1422ab9d0:     	movaps	0xe0(%rsp), %xmm7
1422ab9d8:     	movaps	0xd0(%rsp), %xmm8
1422ab9e1:     	movaps	0xc0(%rsp), %xmm9
1422ab9ea:     	movaps	0xb0(%rsp), %xmm10
1422ab9f3:     	movaps	0xa0(%rsp), %xmm11
1422ab9fc:     	movaps	0x90(%rsp), %xmm12
1422aba05:     	movaps	0x80(%rsp), %xmm13
1422aba0e:     	movaps	0x70(%rsp), %xmm14
1422aba14:     	movaps	0x60(%rsp), %xmm15
1422aba1a:     	addq	$0x108, %rsp            # imm = 0x108
1422aba21:     	popq	%rbx
1422aba22:     	popq	%rdi
1422aba23:     	popq	%rsi
1422aba24:     	popq	%r12
1422aba26:     	popq	%r13
1422aba28:     	popq	%r14
1422aba2a:     	popq	%r15
1422aba2c:     	popq	%rbp
1422aba2d:     	retq
1422aba2e:     	nop
1422aba30:     	movq	%rdx, 0x10(%rsp)
1422aba35:     	pushq	%rbp
1422aba36:     	pushq	%r15
1422aba38:     	pushq	%r14
1422aba3a:     	pushq	%r13
1422aba3c:     	pushq	%r12
1422aba3e:     	pushq	%rsi
1422aba3f:     	pushq	%rdi
1422aba40:     	pushq	%rbx
1422aba41:     	subq	$0x108, %rsp            # imm = 0x108
1422aba48:     	leaq	0x80(%rdx), %rbp
1422aba4f:     	movaps	%xmm15, 0x60(%rsp)
1422aba55:     	movaps	%xmm14, 0x70(%rsp)
1422aba5b:     	movaps	%xmm13, 0x80(%rsp)
1422aba64:     	movaps	%xmm12, 0x90(%rsp)
1422aba6d:     	movaps	%xmm11, 0xa0(%rsp)
1422aba76:     	movaps	%xmm10, 0xb0(%rsp)
1422aba7f:     	movaps	%xmm9, 0xc0(%rsp)
1422aba88:     	movaps	%xmm8, 0xd0(%rsp)
1422aba91:     	movaps	%xmm7, 0xe0(%rsp)
1422aba99:     	movaps	%xmm6, 0xf0(%rsp)
1422abaa1:     	cmpb	$0x0, 0x1d7(%rbp)
1422abaa8:     	je	0x1422abb1c <.text+0x22aab1c>
1422abaaa:     	leaq	0x158(%rbp), %rcx
1422abab1:     	callq	0x14014a860 <.text+0x149860>
1422abab6:     	movaps	0xf0(%rsp), %xmm6
1422ababe:     	movaps	0xe0(%rsp), %xmm7
1422abac6:     	movaps	0xd0(%rsp), %xmm8
1422abacf:     	movaps	0xc0(%rsp), %xmm9
1422abad8:     	movaps	0xb0(%rsp), %xmm10
1422abae1:     	movaps	0xa0(%rsp), %xmm11
1422abaea:     	movaps	0x90(%rsp), %xmm12
1422abaf3:     	movaps	0x80(%rsp), %xmm13
1422abafc:     	movaps	0x70(%rsp), %xmm14
1422abb02:     	movaps	0x60(%rsp), %xmm15
1422abb08:     	addq	$0x108, %rsp            # imm = 0x108
1422abb0f:     	popq	%rbx
1422abb10:     	popq	%rdi
1422abb11:     	popq	%rsi
1422abb12:     	popq	%r12
1422abb14:     	popq	%r13
1422abb16:     	popq	%r14
1422abb18:     	popq	%r15
1422abb1a:     	popq	%rbp
1422abb1b:     	retq
1422abb1c:     	movaps	0xf0(%rsp), %xmm6
1422abb24:     	movaps	0xe0(%rsp), %xmm7
1422abb2c:     	movaps	0xd0(%rsp), %xmm8
1422abb35:     	movaps	0xc0(%rsp), %xmm9
1422abb3e:     	movaps	0xb0(%rsp), %xmm10
1422abb47:     	movaps	0xa0(%rsp), %xmm11
1422abb50:     	movaps	0x90(%rsp), %xmm12
1422abb59:     	movaps	0x80(%rsp), %xmm13
1422abb62:     	movaps	0x70(%rsp), %xmm14
1422abb68:     	movaps	0x60(%rsp), %xmm15
1422abb6e:     	addq	$0x108, %rsp            # imm = 0x108
1422abb75:     	popq	%rbx
1422abb76:     	popq	%rdi
1422abb77:     	popq	%rsi
1422abb78:     	popq	%r12
1422abb7a:     	popq	%r13
1422abb7c:     	popq	%r14
1422abb7e:     	popq	%r15
1422abb80:     	popq	%rbp
1422abb81:     	retq
1422abb82:     	nopw	%cs:(%rax,%rax)
1422abb90:     	movq	%rdx, 0x10(%rsp)
1422abb95:     	pushq	%rbp
1422abb96:     	pushq	%r15
1422abb98:     	pushq	%r14
1422abb9a:     	pushq	%r13
1422abb9c:     	pushq	%r12
1422abb9e:     	pushq	%rsi
1422abb9f:     	pushq	%rdi
1422abba0:     	pushq	%rbx
1422abba1:     	subq	$0x108, %rsp            # imm = 0x108
1422abba8:     	leaq	0x80(%rdx), %rbp
1422abbaf:     	movaps	%xmm15, 0x60(%rsp)
1422abbb5:     	movaps	%xmm14, 0x70(%rsp)
1422abbbb:     	movaps	%xmm13, 0x80(%rsp)
1422abbc4:     	movaps	%xmm12, 0x90(%rsp)
1422abbcd:     	movaps	%xmm11, 0xa0(%rsp)
1422abbd6:     	movaps	%xmm10, 0xb0(%rsp)
1422abbdf:     	movaps	%xmm9, 0xc0(%rsp)
1422abbe8:     	movaps	%xmm8, 0xd0(%rsp)
1422abbf1:     	movaps	%xmm7, 0xe0(%rsp)
1422abbf9:     	movaps	%xmm6, 0xf0(%rsp)
1422abc01:     	movl	$0xe, %edx
1422abc06:     	movl	$0x1, %r8d
1422abc0c:     	movq	0x1e0(%rbp), %rcx
1422abc13:     	callq	0x140613c20 <.text+0x612c20>
1422abc18:     	movaps	0xf0(%rsp), %xmm6
1422abc20:     	movaps	0xe0(%rsp), %xmm7
1422abc28:     	movaps	0xd0(%rsp), %xmm8
1422abc31:     	movaps	0xc0(%rsp), %xmm9
1422abc3a:     	movaps	0xb0(%rsp), %xmm10
1422abc43:     	movaps	0xa0(%rsp), %xmm11
1422abc4c:     	movaps	0x90(%rsp), %xmm12
1422abc55:     	movaps	0x80(%rsp), %xmm13
1422abc5e:     	movaps	0x70(%rsp), %xmm14
1422abc64:     	movaps	0x60(%rsp), %xmm15
1422abc6a:     	addq	$0x108, %rsp            # imm = 0x108
1422abc71:     	popq	%rbx
1422abc72:     	popq	%rdi
1422abc73:     	popq	%rsi
1422abc74:     	popq	%r12
1422abc76:     	popq	%r13
1422abc78:     	popq	%r14
1422abc7a:     	popq	%r15
1422abc7c:     	popq	%rbp
1422abc7d:     	retq
1422abc7e:     	nop
1422abc80:     	movq	%rdx, 0x10(%rsp)
1422abc85:     	pushq	%rbp
1422abc86:     	pushq	%r15
1422abc88:     	pushq	%r14
1422abc8a:     	pushq	%r13
1422abc8c:     	pushq	%r12
1422abc8e:     	pushq	%rsi
1422abc8f:     	pushq	%rdi
1422abc90:     	pushq	%rbx
1422abc91:     	subq	$0x108, %rsp            # imm = 0x108
1422abc98:     	leaq	0x80(%rdx), %rbp
1422abc9f:     	movaps	%xmm15, 0x60(%rsp)
1422abca5:     	movaps	%xmm14, 0x70(%rsp)
1422abcab:     	movaps	%xmm13, 0x80(%rsp)
1422abcb4:     	movaps	%xmm12, 0x90(%rsp)
1422abcbd:     	movaps	%xmm11, 0xa0(%rsp)
1422abcc6:     	movaps	%xmm10, 0xb0(%rsp)
1422abccf:     	movaps	%xmm9, 0xc0(%rsp)
1422abcd8:     	movaps	%xmm8, 0xd0(%rsp)
1422abce1:     	movaps	%xmm7, 0xe0(%rsp)
1422abce9:     	movaps	%xmm6, 0xf0(%rsp)
1422abcf1:     	movl	$0x17, %edx
1422abcf6:     	movl	$0x1, %r8d
1422abcfc:     	movq	0x1e0(%rbp), %rcx
1422abd03:     	callq	0x140613c20 <.text+0x612c20>
1422abd08:     	movaps	0xf0(%rsp), %xmm6
1422abd10:     	movaps	0xe0(%rsp), %xmm7
1422abd18:     	movaps	0xd0(%rsp), %xmm8
1422abd21:     	movaps	0xc0(%rsp), %xmm9
1422abd2a:     	movaps	0xb0(%rsp), %xmm10
1422abd33:     	movaps	0xa0(%rsp), %xmm11
1422abd3c:     	movaps	0x90(%rsp), %xmm12
1422abd45:     	movaps	0x80(%rsp), %xmm13
1422abd4e:     	movaps	0x70(%rsp), %xmm14
1422abd54:     	movaps	0x60(%rsp), %xmm15
1422abd5a:     	addq	$0x108, %rsp            # imm = 0x108
1422abd61:     	popq	%rbx
1422abd62:     	popq	%rdi
1422abd63:     	popq	%rsi
1422abd64:     	popq	%r12
1422abd66:     	popq	%r13
1422abd68:     	popq	%r14
1422abd6a:     	popq	%r15
1422abd6c:     	popq	%rbp
1422abd6d:     	retq
1422abd6e:     	nop
1422abd70:     	movq	%rdx, 0x10(%rsp)
1422abd75:     	pushq	%rbp
1422abd76:     	pushq	%r15
1422abd78:     	pushq	%r14
1422abd7a:     	pushq	%r13
1422abd7c:     	pushq	%r12
1422abd7e:     	pushq	%rsi
1422abd7f:     	pushq	%rdi
1422abd80:     	pushq	%rbx
1422abd81:     	subq	$0x108, %rsp            # imm = 0x108
1422abd88:     	leaq	0x80(%rdx), %rbp
1422abd8f:     	movaps	%xmm15, 0x60(%rsp)
1422abd95:     	movaps	%xmm14, 0x70(%rsp)
1422abd9b:     	movaps	%xmm13, 0x80(%rsp)
1422abda4:     	movaps	%xmm12, 0x90(%rsp)
1422abdad:     	movaps	%xmm11, 0xa0(%rsp)
1422abdb6:     	movaps	%xmm10, 0xb0(%rsp)
1422abdbf:     	movaps	%xmm9, 0xc0(%rsp)
1422abdc8:     	movaps	%xmm8, 0xd0(%rsp)
1422abdd1:     	movaps	%xmm7, 0xe0(%rsp)
1422abdd9:     	movaps	%xmm6, 0xf0(%rsp)
1422abde1:     	movl	$0x17, %edx
1422abde6:     	movl	$0x1, %r8d
1422abdec:     	movq	0x1f0(%rbp), %rcx
1422abdf3:     	callq	0x140613c20 <.text+0x612c20>
1422abdf8:     	movaps	0xf0(%rsp), %xmm6
1422abe00:     	movaps	0xe0(%rsp), %xmm7
1422abe08:     	movaps	0xd0(%rsp), %xmm8
1422abe11:     	movaps	0xc0(%rsp), %xmm9
1422abe1a:     	movaps	0xb0(%rsp), %xmm10
1422abe23:     	movaps	0xa0(%rsp), %xmm11
1422abe2c:     	movaps	0x90(%rsp), %xmm12
1422abe35:     	movaps	0x80(%rsp), %xmm13
1422abe3e:     	movaps	0x70(%rsp), %xmm14
1422abe44:     	movaps	0x60(%rsp), %xmm15
1422abe4a:     	addq	$0x108, %rsp            # imm = 0x108
1422abe51:     	popq	%rbx
1422abe52:     	popq	%rdi
1422abe53:     	popq	%rsi
1422abe54:     	popq	%r12
1422abe56:     	popq	%r13
1422abe58:     	popq	%r14
1422abe5a:     	popq	%r15
1422abe5c:     	popq	%rbp
1422abe5d:     	retq
1422abe5e:     	nop
1422abe60:     	movq	%rdx, 0x10(%rsp)
1422abe65:     	pushq	%rbp
1422abe66:     	pushq	%r15
1422abe68:     	pushq	%r14
1422abe6a:     	pushq	%r13
1422abe6c:     	pushq	%r12
1422abe6e:     	pushq	%rsi
1422abe6f:     	pushq	%rdi
1422abe70:     	pushq	%rbx
1422abe71:     	subq	$0x108, %rsp            # imm = 0x108
1422abe78:     	leaq	0x80(%rdx), %rbp
1422abe7f:     	movaps	%xmm15, 0x60(%rsp)
1422abe85:     	movaps	%xmm14, 0x70(%rsp)
1422abe8b:     	movaps	%xmm13, 0x80(%rsp)
1422abe94:     	movaps	%xmm12, 0x90(%rsp)
1422abe9d:     	movaps	%xmm11, 0xa0(%rsp)
1422abea6:     	movaps	%xmm10, 0xb0(%rsp)
1422abeaf:     	movaps	%xmm9, 0xc0(%rsp)
1422abeb8:     	movaps	%xmm8, 0xd0(%rsp)
1422abec1:     	movaps	%xmm7, 0xe0(%rsp)
1422abec9:     	movaps	%xmm6, 0xf0(%rsp)
1422abed1:     	movl	$0x118, %edx            # imm = 0x118
1422abed6:     	movl	$0x8, %r8d
1422abedc:     	movq	0x1f0(%rbp), %rcx
1422abee3:     	callq	0x140613c20 <.text+0x612c20>
1422abee8:     	movaps	0xf0(%rsp), %xmm6
1422abef0:     	movaps	0xe0(%rsp), %xmm7
1422abef8:     	movaps	0xd0(%rsp), %xmm8
1422abf01:     	movaps	0xc0(%rsp), %xmm9
1422abf0a:     	movaps	0xb0(%rsp), %xmm10
1422abf13:     	movaps	0xa0(%rsp), %xmm11
1422abf1c:     	movaps	0x90(%rsp), %xmm12
1422abf25:     	movaps	0x80(%rsp), %xmm13
1422abf2e:     	movaps	0x70(%rsp), %xmm14
1422abf34:     	movaps	0x60(%rsp), %xmm15
1422abf3a:     	addq	$0x108, %rsp            # imm = 0x108
1422abf41:     	popq	%rbx
1422abf42:     	popq	%rdi
1422abf43:     	popq	%rsi
1422abf44:     	popq	%r12
1422abf46:     	popq	%r13
1422abf48:     	popq	%r14
1422abf4a:     	popq	%r15
1422abf4c:     	popq	%rbp
1422abf4d:     	retq
1422abf4e:     	nop
1422abf50:     	movq	%rdx, 0x10(%rsp)
1422abf55:     	pushq	%rbp
1422abf56:     	pushq	%r15
1422abf58:     	pushq	%r14
1422abf5a:     	pushq	%r13
1422abf5c:     	pushq	%r12
1422abf5e:     	pushq	%rsi
1422abf5f:     	pushq	%rdi
1422abf60:     	pushq	%rbx
1422abf61:     	subq	$0x108, %rsp            # imm = 0x108
1422abf68:     	leaq	0x80(%rdx), %rbp
1422abf6f:     	movaps	%xmm15, 0x60(%rsp)
1422abf75:     	movaps	%xmm14, 0x70(%rsp)
1422abf7b:     	movaps	%xmm13, 0x80(%rsp)
1422abf84:     	movaps	%xmm12, 0x90(%rsp)
1422abf8d:     	movaps	%xmm11, 0xa0(%rsp)
1422abf96:     	movaps	%xmm10, 0xb0(%rsp)
1422abf9f:     	movaps	%xmm9, 0xc0(%rsp)
1422abfa8:     	movaps	%xmm8, 0xd0(%rsp)
1422abfb1:     	movaps	%xmm7, 0xe0(%rsp)
1422abfb9:     	movaps	%xmm6, 0xf0(%rsp)
1422abfc1:     	movl	$0x16, %edx
1422abfc6:     	movl	$0x1, %r8d
1422abfcc:     	movq	0x1f8(%rbp), %rcx
1422abfd3:     	callq	0x140613c20 <.text+0x612c20>
1422abfd8:     	movaps	0xf0(%rsp), %xmm6
1422abfe0:     	movaps	0xe0(%rsp), %xmm7
1422abfe8:     	movaps	0xd0(%rsp), %xmm8
1422abff1:     	movaps	0xc0(%rsp), %xmm9
1422abffa:     	movaps	0xb0(%rsp), %xmm10
1422ac003:     	movaps	0xa0(%rsp), %xmm11
1422ac00c:     	movaps	0x90(%rsp), %xmm12
1422ac015:     	movaps	0x80(%rsp), %xmm13
1422ac01e:     	movaps	0x70(%rsp), %xmm14
1422ac024:     	movaps	0x60(%rsp), %xmm15
1422ac02a:     	addq	$0x108, %rsp            # imm = 0x108
1422ac031:     	popq	%rbx
1422ac032:     	popq	%rdi
1422ac033:     	popq	%rsi
1422ac034:     	popq	%r12
1422ac036:     	popq	%r13
1422ac038:     	popq	%r14
1422ac03a:     	popq	%r15
1422ac03c:     	popq	%rbp
1422ac03d:     	retq
1422ac03e:     	nop
1422ac040:     	movq	%rdx, 0x10(%rsp)
1422ac045:     	pushq	%rbp
1422ac046:     	pushq	%r15
1422ac048:     	pushq	%r14
1422ac04a:     	pushq	%r13
1422ac04c:     	pushq	%r12
1422ac04e:     	pushq	%rsi
1422ac04f:     	pushq	%rdi
1422ac050:     	pushq	%rbx
1422ac051:     	subq	$0x108, %rsp            # imm = 0x108
1422ac058:     	leaq	0x80(%rdx), %rbp
1422ac05f:     	movaps	%xmm15, 0x60(%rsp)
1422ac065:     	movaps	%xmm14, 0x70(%rsp)
1422ac06b:     	movaps	%xmm13, 0x80(%rsp)
1422ac074:     	movaps	%xmm12, 0x90(%rsp)
1422ac07d:     	movaps	%xmm11, 0xa0(%rsp)
1422ac086:     	movaps	%xmm10, 0xb0(%rsp)
1422ac08f:     	movaps	%xmm9, 0xc0(%rsp)
1422ac098:     	movaps	%xmm8, 0xd0(%rsp)
1422ac0a1:     	movaps	%xmm7, 0xe0(%rsp)
1422ac0a9:     	movaps	%xmm6, 0xf0(%rsp)
1422ac0b1:     	movl	$0x16, %edx
1422ac0b6:     	movl	$0x1, %r8d
1422ac0bc:     	movq	0x1f8(%rbp), %rcx
1422ac0c3:     	callq	0x140613c20 <.text+0x612c20>
1422ac0c8:     	movaps	0xf0(%rsp), %xmm6
1422ac0d0:     	movaps	0xe0(%rsp), %xmm7
1422ac0d8:     	movaps	0xd0(%rsp), %xmm8
1422ac0e1:     	movaps	0xc0(%rsp), %xmm9
1422ac0ea:     	movaps	0xb0(%rsp), %xmm10
1422ac0f3:     	movaps	0xa0(%rsp), %xmm11
1422ac0fc:     	movaps	0x90(%rsp), %xmm12
1422ac105:     	movaps	0x80(%rsp), %xmm13
1422ac10e:     	movaps	0x70(%rsp), %xmm14
1422ac114:     	movaps	0x60(%rsp), %xmm15
1422ac11a:     	addq	$0x108, %rsp            # imm = 0x108
1422ac121:     	popq	%rbx
1422ac122:     	popq	%rdi
1422ac123:     	popq	%rsi
1422ac124:     	popq	%r12
1422ac126:     	popq	%r13
1422ac128:     	popq	%r14
1422ac12a:     	popq	%r15
1422ac12c:     	popq	%rbp
1422ac12d:     	retq
1422ac12e:     	nop
1422ac130:     	movq	%rdx, 0x10(%rsp)
1422ac135:     	pushq	%rbp
1422ac136:     	pushq	%r15
1422ac138:     	pushq	%r14
1422ac13a:     	pushq	%r13
1422ac13c:     	pushq	%r12
1422ac13e:     	pushq	%rsi
1422ac13f:     	pushq	%rdi
1422ac140:     	pushq	%rbx
1422ac141:     	subq	$0x108, %rsp            # imm = 0x108
1422ac148:     	leaq	0x80(%rdx), %rbp
1422ac14f:     	movaps	%xmm15, 0x60(%rsp)
1422ac155:     	movaps	%xmm14, 0x70(%rsp)
1422ac15b:     	movaps	%xmm13, 0x80(%rsp)
1422ac164:     	movaps	%xmm12, 0x90(%rsp)
1422ac16d:     	movaps	%xmm11, 0xa0(%rsp)
1422ac176:     	movaps	%xmm10, 0xb0(%rsp)
1422ac17f:     	movaps	%xmm9, 0xc0(%rsp)
1422ac188:     	movaps	%xmm8, 0xd0(%rsp)
1422ac191:     	movaps	%xmm7, 0xe0(%rsp)
1422ac199:     	movaps	%xmm6, 0xf0(%rsp)
1422ac1a1:     	movl	$0x280, %edx            # imm = 0x280
1422ac1a6:     	movl	$0x8, %r8d
1422ac1ac:     	movq	0x200(%rbp), %rcx
1422ac1b3:     	callq	0x140613c20 <.text+0x612c20>
1422ac1b8:     	movaps	0xf0(%rsp), %xmm6
1422ac1c0:     	movaps	0xe0(%rsp), %xmm7
1422ac1c8:     	movaps	0xd0(%rsp), %xmm8
1422ac1d1:     	movaps	0xc0(%rsp), %xmm9
1422ac1da:     	movaps	0xb0(%rsp), %xmm10
1422ac1e3:     	movaps	0xa0(%rsp), %xmm11
1422ac1ec:     	movaps	0x90(%rsp), %xmm12
1422ac1f5:     	movaps	0x80(%rsp), %xmm13
1422ac1fe:     	movaps	0x70(%rsp), %xmm14
1422ac204:     	movaps	0x60(%rsp), %xmm15
1422ac20a:     	addq	$0x108, %rsp            # imm = 0x108
1422ac211:     	popq	%rbx
1422ac212:     	popq	%rdi
1422ac213:     	popq	%rsi
1422ac214:     	popq	%r12
1422ac216:     	popq	%r13
1422ac218:     	popq	%r14
1422ac21a:     	popq	%r15
1422ac21c:     	popq	%rbp
1422ac21d:     	retq
1422ac21e:     	nop
1422ac220:     	movq	%rdx, 0x10(%rsp)
1422ac225:     	pushq	%rbp
1422ac226:     	pushq	%r15
1422ac228:     	pushq	%r14
1422ac22a:     	pushq	%r13
1422ac22c:     	pushq	%r12
1422ac22e:     	pushq	%rsi
1422ac22f:     	pushq	%rdi
1422ac230:     	pushq	%rbx
1422ac231:     	subq	$0x108, %rsp            # imm = 0x108
1422ac238:     	leaq	0x80(%rdx), %rbp
1422ac23f:     	movaps	%xmm15, 0x60(%rsp)
1422ac245:     	movaps	%xmm14, 0x70(%rsp)
1422ac24b:     	movaps	%xmm13, 0x80(%rsp)
1422ac254:     	movaps	%xmm12, 0x90(%rsp)
1422ac25d:     	movaps	%xmm11, 0xa0(%rsp)
1422ac266:     	movaps	%xmm10, 0xb0(%rsp)
1422ac26f:     	movaps	%xmm9, 0xc0(%rsp)
1422ac278:     	movaps	%xmm8, 0xd0(%rsp)
1422ac281:     	movaps	%xmm7, 0xe0(%rsp)
1422ac289:     	movaps	%xmm6, 0xf0(%rsp)
1422ac291:     	movl	$0x18, %edx
1422ac296:     	movl	$0x1, %r8d
1422ac29c:     	movq	0x1f8(%rbp), %rcx
1422ac2a3:     	callq	0x140613c20 <.text+0x612c20>
1422ac2a8:     	movaps	0xf0(%rsp), %xmm6
1422ac2b0:     	movaps	0xe0(%rsp), %xmm7
1422ac2b8:     	movaps	0xd0(%rsp), %xmm8
1422ac2c1:     	movaps	0xc0(%rsp), %xmm9
1422ac2ca:     	movaps	0xb0(%rsp), %xmm10
1422ac2d3:     	movaps	0xa0(%rsp), %xmm11
1422ac2dc:     	movaps	0x90(%rsp), %xmm12
1422ac2e5:     	movaps	0x80(%rsp), %xmm13
1422ac2ee:     	movaps	0x70(%rsp), %xmm14
1422ac2f4:     	movaps	0x60(%rsp), %xmm15
1422ac2fa:     	addq	$0x108, %rsp            # imm = 0x108
1422ac301:     	popq	%rbx
1422ac302:     	popq	%rdi
1422ac303:     	popq	%rsi
1422ac304:     	popq	%r12
1422ac306:     	popq	%r13
1422ac308:     	popq	%r14
1422ac30a:     	popq	%r15
1422ac30c:     	popq	%rbp
1422ac30d:     	retq
1422ac30e:     	nop
1422ac310:     	movq	%rdx, 0x10(%rsp)
1422ac315:     	pushq	%rbp
1422ac316:     	pushq	%r15
1422ac318:     	pushq	%r14
1422ac31a:     	pushq	%r13
1422ac31c:     	pushq	%r12
1422ac31e:     	pushq	%rsi
1422ac31f:     	pushq	%rdi
1422ac320:     	pushq	%rbx
1422ac321:     	subq	$0x108, %rsp            # imm = 0x108
1422ac328:     	leaq	0x80(%rdx), %rbp
1422ac32f:     	movaps	%xmm15, 0x60(%rsp)
1422ac335:     	movaps	%xmm14, 0x70(%rsp)
1422ac33b:     	movaps	%xmm13, 0x80(%rsp)
1422ac344:     	movaps	%xmm12, 0x90(%rsp)
1422ac34d:     	movaps	%xmm11, 0xa0(%rsp)
1422ac356:     	movaps	%xmm10, 0xb0(%rsp)
1422ac35f:     	movaps	%xmm9, 0xc0(%rsp)
1422ac368:     	movaps	%xmm8, 0xd0(%rsp)
1422ac371:     	movaps	%xmm7, 0xe0(%rsp)
1422ac379:     	movaps	%xmm6, 0xf0(%rsp)
1422ac381:     	movl	$0x15, %edx
1422ac386:     	movl	$0x1, %r8d
1422ac38c:     	movq	0x1e0(%rbp), %rcx
1422ac393:     	callq	0x140613c20 <.text+0x612c20>
1422ac398:     	movaps	0xf0(%rsp), %xmm6
1422ac3a0:     	movaps	0xe0(%rsp), %xmm7
1422ac3a8:     	movaps	0xd0(%rsp), %xmm8
1422ac3b1:     	movaps	0xc0(%rsp), %xmm9
1422ac3ba:     	movaps	0xb0(%rsp), %xmm10
1422ac3c3:     	movaps	0xa0(%rsp), %xmm11
1422ac3cc:     	movaps	0x90(%rsp), %xmm12
1422ac3d5:     	movaps	0x80(%rsp), %xmm13
1422ac3de:     	movaps	0x70(%rsp), %xmm14
1422ac3e4:     	movaps	0x60(%rsp), %xmm15
1422ac3ea:     	addq	$0x108, %rsp            # imm = 0x108
1422ac3f1:     	popq	%rbx
1422ac3f2:     	popq	%rdi
1422ac3f3:     	popq	%rsi
1422ac3f4:     	popq	%r12
1422ac3f6:     	popq	%r13
1422ac3f8:     	popq	%r14
1422ac3fa:     	popq	%r15
1422ac3fc:     	popq	%rbp
1422ac3fd:     	retq
1422ac3fe:     	nop
1422ac400:     	movq	%rdx, 0x10(%rsp)
1422ac405:     	pushq	%rbp
1422ac406:     	pushq	%r15
1422ac408:     	pushq	%r14
1422ac40a:     	pushq	%r13
1422ac40c:     	pushq	%r12
1422ac40e:     	pushq	%rsi
1422ac40f:     	pushq	%rdi
1422ac410:     	pushq	%rbx
1422ac411:     	subq	$0x108, %rsp            # imm = 0x108
1422ac418:     	leaq	0x80(%rdx), %rbp
1422ac41f:     	movaps	%xmm15, 0x60(%rsp)
1422ac425:     	movaps	%xmm14, 0x70(%rsp)
1422ac42b:     	movaps	%xmm13, 0x80(%rsp)
1422ac434:     	movaps	%xmm12, 0x90(%rsp)
1422ac43d:     	movaps	%xmm11, 0xa0(%rsp)
1422ac446:     	movaps	%xmm10, 0xb0(%rsp)
1422ac44f:     	movaps	%xmm9, 0xc0(%rsp)
1422ac458:     	movaps	%xmm8, 0xd0(%rsp)
1422ac461:     	movaps	%xmm7, 0xe0(%rsp)
1422ac469:     	movaps	%xmm6, 0xf0(%rsp)
1422ac471:     	movl	$0x16, %edx
1422ac476:     	movl	$0x1, %r8d
1422ac47c:     	movq	0x200(%rbp), %rcx
1422ac483:     	callq	0x140613c20 <.text+0x612c20>
1422ac488:     	movaps	0xf0(%rsp), %xmm6
1422ac490:     	movaps	0xe0(%rsp), %xmm7
1422ac498:     	movaps	0xd0(%rsp), %xmm8
1422ac4a1:     	movaps	0xc0(%rsp), %xmm9
1422ac4aa:     	movaps	0xb0(%rsp), %xmm10
1422ac4b3:     	movaps	0xa0(%rsp), %xmm11
1422ac4bc:     	movaps	0x90(%rsp), %xmm12
1422ac4c5:     	movaps	0x80(%rsp), %xmm13
1422ac4ce:     	movaps	0x70(%rsp), %xmm14
1422ac4d4:     	movaps	0x60(%rsp), %xmm15
1422ac4da:     	addq	$0x108, %rsp            # imm = 0x108
1422ac4e1:     	popq	%rbx
1422ac4e2:     	popq	%rdi
1422ac4e3:     	popq	%rsi
1422ac4e4:     	popq	%r12
1422ac4e6:     	popq	%r13
1422ac4e8:     	popq	%r14
1422ac4ea:     	popq	%r15
1422ac4ec:     	popq	%rbp
1422ac4ed:     	retq
1422ac4ee:     	nop
1422ac4f0:     	movq	%rdx, 0x10(%rsp)
1422ac4f5:     	pushq	%rbp
1422ac4f6:     	pushq	%r15
1422ac4f8:     	pushq	%r14
1422ac4fa:     	pushq	%r13
1422ac4fc:     	pushq	%r12
1422ac4fe:     	pushq	%rsi
1422ac4ff:     	pushq	%rdi
1422ac500:     	pushq	%rbx
1422ac501:     	subq	$0x108, %rsp            # imm = 0x108
1422ac508:     	leaq	0x80(%rdx), %rbp
1422ac50f:     	movaps	%xmm15, 0x60(%rsp)
1422ac515:     	movaps	%xmm14, 0x70(%rsp)
1422ac51b:     	movaps	%xmm13, 0x80(%rsp)
1422ac524:     	movaps	%xmm12, 0x90(%rsp)
1422ac52d:     	movaps	%xmm11, 0xa0(%rsp)
1422ac536:     	movaps	%xmm10, 0xb0(%rsp)
1422ac53f:     	movaps	%xmm9, 0xc0(%rsp)
1422ac548:     	movaps	%xmm8, 0xd0(%rsp)
1422ac551:     	movaps	%xmm7, 0xe0(%rsp)
1422ac559:     	movaps	%xmm6, 0xf0(%rsp)
1422ac561:     	leaq	0xd8(%rbp), %rcx
1422ac568:     	callq	0x14014a860 <.text+0x149860>
1422ac56d:     	movaps	0xf0(%rsp), %xmm6
1422ac575:     	movaps	0xe0(%rsp), %xmm7
1422ac57d:     	movaps	0xd0(%rsp), %xmm8
1422ac586:     	movaps	0xc0(%rsp), %xmm9
1422ac58f:     	movaps	0xb0(%rsp), %xmm10
1422ac598:     	movaps	0xa0(%rsp), %xmm11
1422ac5a1:     	movaps	0x90(%rsp), %xmm12
1422ac5aa:     	movaps	0x80(%rsp), %xmm13
1422ac5b3:     	movaps	0x70(%rsp), %xmm14
1422ac5b9:     	movaps	0x60(%rsp), %xmm15
1422ac5bf:     	addq	$0x108, %rsp            # imm = 0x108
1422ac5c6:     	popq	%rbx
1422ac5c7:     	popq	%rdi
1422ac5c8:     	popq	%rsi
1422ac5c9:     	popq	%r12
1422ac5cb:     	popq	%r13
1422ac5cd:     	popq	%r14
1422ac5cf:     	popq	%r15
1422ac5d1:     	popq	%rbp
1422ac5d2:     	retq
1422ac5d3:     	nopw	%cs:(%rax,%rax)
1422ac5e0:     	movq	%rdx, 0x10(%rsp)
1422ac5e5:     	pushq	%rbp
1422ac5e6:     	pushq	%r15
1422ac5e8:     	pushq	%r14
1422ac5ea:     	pushq	%r13
1422ac5ec:     	pushq	%r12
1422ac5ee:     	pushq	%rsi
1422ac5ef:     	pushq	%rdi
1422ac5f0:     	pushq	%rbx
1422ac5f1:     	subq	$0x108, %rsp            # imm = 0x108
1422ac5f8:     	leaq	0x80(%rdx), %rbp
1422ac5ff:     	movaps	%xmm15, 0x60(%rsp)
1422ac605:     	movaps	%xmm14, 0x70(%rsp)
1422ac60b:     	movaps	%xmm13, 0x80(%rsp)
1422ac614:     	movaps	%xmm12, 0x90(%rsp)
1422ac61d:     	movaps	%xmm11, 0xa0(%rsp)
1422ac626:     	movaps	%xmm10, 0xb0(%rsp)
1422ac62f:     	movaps	%xmm9, 0xc0(%rsp)
1422ac638:     	movaps	%xmm8, 0xd0(%rsp)
1422ac641:     	movaps	%xmm7, 0xe0(%rsp)
1422ac649:     	movaps	%xmm6, 0xf0(%rsp)
1422ac651:     	movl	$0x17, %edx
1422ac656:     	movl	$0x1, %r8d
1422ac65c:     	movq	0x200(%rbp), %rcx
1422ac663:     	callq	0x140613c20 <.text+0x612c20>
1422ac668:     	movaps	0xf0(%rsp), %xmm6
1422ac670:     	movaps	0xe0(%rsp), %xmm7
1422ac678:     	movaps	0xd0(%rsp), %xmm8
1422ac681:     	movaps	0xc0(%rsp), %xmm9
1422ac68a:     	movaps	0xb0(%rsp), %xmm10
1422ac693:     	movaps	0xa0(%rsp), %xmm11
1422ac69c:     	movaps	0x90(%rsp), %xmm12
1422ac6a5:     	movaps	0x80(%rsp), %xmm13
1422ac6ae:     	movaps	0x70(%rsp), %xmm14
1422ac6b4:     	movaps	0x60(%rsp), %xmm15
1422ac6ba:     	addq	$0x108, %rsp            # imm = 0x108
1422ac6c1:     	popq	%rbx
1422ac6c2:     	popq	%rdi
1422ac6c3:     	popq	%rsi
1422ac6c4:     	popq	%r12
1422ac6c6:     	popq	%r13
1422ac6c8:     	popq	%r14
1422ac6ca:     	popq	%r15
1422ac6cc:     	popq	%rbp
1422ac6cd:     	retq
1422ac6ce:     	nop
1422ac6d0:     	movq	%rdx, 0x10(%rsp)
1422ac6d5:     	pushq	%rbp
1422ac6d6:     	pushq	%r15
1422ac6d8:     	pushq	%r14
1422ac6da:     	pushq	%r13
1422ac6dc:     	pushq	%r12
1422ac6de:     	pushq	%rsi
1422ac6df:     	pushq	%rdi
1422ac6e0:     	pushq	%rbx
1422ac6e1:     	subq	$0x108, %rsp            # imm = 0x108
1422ac6e8:     	leaq	0x80(%rdx), %rbp
1422ac6ef:     	movaps	%xmm15, 0x60(%rsp)
1422ac6f5:     	movaps	%xmm14, 0x70(%rsp)
1422ac6fb:     	movaps	%xmm13, 0x80(%rsp)
1422ac704:     	movaps	%xmm12, 0x90(%rsp)
1422ac70d:     	movaps	%xmm11, 0xa0(%rsp)
1422ac716:     	movaps	%xmm10, 0xb0(%rsp)
1422ac71f:     	movaps	%xmm9, 0xc0(%rsp)
1422ac728:     	movaps	%xmm8, 0xd0(%rsp)
1422ac731:     	movaps	%xmm7, 0xe0(%rsp)
1422ac739:     	movaps	%xmm6, 0xf0(%rsp)
1422ac741:     	movl	$0x168, %edx            # imm = 0x168
1422ac746:     	movl	$0x8, %r8d
1422ac74c:     	movq	0x1f0(%rbp), %rcx
1422ac753:     	callq	0x140613c20 <.text+0x612c20>
1422ac758:     	movaps	0xf0(%rsp), %xmm6
1422ac760:     	movaps	0xe0(%rsp), %xmm7
1422ac768:     	movaps	0xd0(%rsp), %xmm8
1422ac771:     	movaps	0xc0(%rsp), %xmm9
1422ac77a:     	movaps	0xb0(%rsp), %xmm10
1422ac783:     	movaps	0xa0(%rsp), %xmm11
1422ac78c:     	movaps	0x90(%rsp), %xmm12
1422ac795:     	movaps	0x80(%rsp), %xmm13
1422ac79e:     	movaps	0x70(%rsp), %xmm14
1422ac7a4:     	movaps	0x60(%rsp), %xmm15
1422ac7aa:     	addq	$0x108, %rsp            # imm = 0x108
1422ac7b1:     	popq	%rbx
1422ac7b2:     	popq	%rdi
1422ac7b3:     	popq	%rsi
1422ac7b4:     	popq	%r12
1422ac7b6:     	popq	%r13
1422ac7b8:     	popq	%r14
1422ac7ba:     	popq	%r15
1422ac7bc:     	popq	%rbp
1422ac7bd:     	retq
1422ac7be:     	nop
1422ac7c0:     	movq	%rdx, 0x10(%rsp)
1422ac7c5:     	pushq	%rbp
1422ac7c6:     	pushq	%r15
1422ac7c8:     	pushq	%r14
1422ac7ca:     	pushq	%r13
1422ac7cc:     	pushq	%r12
1422ac7ce:     	pushq	%rsi
1422ac7cf:     	pushq	%rdi
1422ac7d0:     	pushq	%rbx
1422ac7d1:     	subq	$0x108, %rsp            # imm = 0x108
1422ac7d8:     	leaq	0x80(%rdx), %rbp
1422ac7df:     	movaps	%xmm15, 0x60(%rsp)
1422ac7e5:     	movaps	%xmm14, 0x70(%rsp)
1422ac7eb:     	movaps	%xmm13, 0x80(%rsp)
1422ac7f4:     	movaps	%xmm12, 0x90(%rsp)
1422ac7fd:     	movaps	%xmm11, 0xa0(%rsp)
1422ac806:     	movaps	%xmm10, 0xb0(%rsp)
1422ac80f:     	movaps	%xmm9, 0xc0(%rsp)
1422ac818:     	movaps	%xmm8, 0xd0(%rsp)
1422ac821:     	movaps	%xmm7, 0xe0(%rsp)
1422ac829:     	movaps	%xmm6, 0xf0(%rsp)
1422ac831:     	movl	$0x14, %edx
1422ac836:     	movl	$0x1, %r8d
1422ac83c:     	movq	0x1f8(%rbp), %rcx
1422ac843:     	callq	0x140613c20 <.text+0x612c20>
1422ac848:     	movaps	0xf0(%rsp), %xmm6
1422ac850:     	movaps	0xe0(%rsp), %xmm7
1422ac858:     	movaps	0xd0(%rsp), %xmm8
1422ac861:     	movaps	0xc0(%rsp), %xmm9
1422ac86a:     	movaps	0xb0(%rsp), %xmm10
1422ac873:     	movaps	0xa0(%rsp), %xmm11
1422ac87c:     	movaps	0x90(%rsp), %xmm12
1422ac885:     	movaps	0x80(%rsp), %xmm13
1422ac88e:     	movaps	0x70(%rsp), %xmm14
1422ac894:     	movaps	0x60(%rsp), %xmm15
1422ac89a:     	addq	$0x108, %rsp            # imm = 0x108
1422ac8a1:     	popq	%rbx
1422ac8a2:     	popq	%rdi
1422ac8a3:     	popq	%rsi
1422ac8a4:     	popq	%r12
1422ac8a6:     	popq	%r13
1422ac8a8:     	popq	%r14
1422ac8aa:     	popq	%r15
1422ac8ac:     	popq	%rbp
1422ac8ad:     	retq
1422ac8ae:     	nop
1422ac8b0:     	movq	%rdx, 0x10(%rsp)
1422ac8b5:     	pushq	%rbp
1422ac8b6:     	pushq	%r15
1422ac8b8:     	pushq	%r14
1422ac8ba:     	pushq	%r13
1422ac8bc:     	pushq	%r12
1422ac8be:     	pushq	%rsi
1422ac8bf:     	pushq	%rdi
1422ac8c0:     	pushq	%rbx
1422ac8c1:     	subq	$0x108, %rsp            # imm = 0x108
1422ac8c8:     	leaq	0x80(%rdx), %rbp
1422ac8cf:     	movaps	%xmm15, 0x60(%rsp)
1422ac8d5:     	movaps	%xmm14, 0x70(%rsp)
1422ac8db:     	movaps	%xmm13, 0x80(%rsp)
1422ac8e4:     	movaps	%xmm12, 0x90(%rsp)
1422ac8ed:     	movaps	%xmm11, 0xa0(%rsp)
1422ac8f6:     	movaps	%xmm10, 0xb0(%rsp)
1422ac8ff:     	movaps	%xmm9, 0xc0(%rsp)
1422ac908:     	movaps	%xmm8, 0xd0(%rsp)
1422ac911:     	movaps	%xmm7, 0xe0(%rsp)
1422ac919:     	movaps	%xmm6, 0xf0(%rsp)
1422ac921:     	movl	$0x190, %edx            # imm = 0x190
1422ac926:     	movl	$0x8, %r8d
1422ac92c:     	movq	0x200(%rbp), %rcx
1422ac933:     	callq	0x140613c20 <.text+0x612c20>
1422ac938:     	movaps	0xf0(%rsp), %xmm6
1422ac940:     	movaps	0xe0(%rsp), %xmm7
1422ac948:     	movaps	0xd0(%rsp), %xmm8
1422ac951:     	movaps	0xc0(%rsp), %xmm9
1422ac95a:     	movaps	0xb0(%rsp), %xmm10
1422ac963:     	movaps	0xa0(%rsp), %xmm11
1422ac96c:     	movaps	0x90(%rsp), %xmm12
1422ac975:     	movaps	0x80(%rsp), %xmm13
1422ac97e:     	movaps	0x70(%rsp), %xmm14
1422ac984:     	movaps	0x60(%rsp), %xmm15
1422ac98a:     	addq	$0x108, %rsp            # imm = 0x108
1422ac991:     	popq	%rbx
1422ac992:     	popq	%rdi
1422ac993:     	popq	%rsi
1422ac994:     	popq	%r12
1422ac996:     	popq	%r13
1422ac998:     	popq	%r14
1422ac99a:     	popq	%r15
1422ac99c:     	popq	%rbp
1422ac99d:     	retq
1422ac99e:     	nop
1422ac9a0:     	movq	%rdx, 0x10(%rsp)
1422ac9a5:     	pushq	%rbp
1422ac9a6:     	pushq	%r15
1422ac9a8:     	pushq	%r14
1422ac9aa:     	pushq	%r13
1422ac9ac:     	pushq	%r12
1422ac9ae:     	pushq	%rsi
1422ac9af:     	pushq	%rdi
1422ac9b0:     	pushq	%rbx
1422ac9b1:     	subq	$0x108, %rsp            # imm = 0x108
1422ac9b8:     	leaq	0x80(%rdx), %rbp
1422ac9bf:     	movaps	%xmm15, 0x60(%rsp)
1422ac9c5:     	movaps	%xmm14, 0x70(%rsp)
1422ac9cb:     	movaps	%xmm13, 0x80(%rsp)
1422ac9d4:     	movaps	%xmm12, 0x90(%rsp)
1422ac9dd:     	movaps	%xmm11, 0xa0(%rsp)
1422ac9e6:     	movaps	%xmm10, 0xb0(%rsp)
1422ac9ef:     	movaps	%xmm9, 0xc0(%rsp)
1422ac9f8:     	movaps	%xmm8, 0xd0(%rsp)
1422aca01:     	movaps	%xmm7, 0xe0(%rsp)
1422aca09:     	movaps	%xmm6, 0xf0(%rsp)
1422aca11:     	movl	$0x190, %edx            # imm = 0x190
1422aca16:     	movl	$0x8, %r8d
1422aca1c:     	movq	0x1f0(%rbp), %rcx
1422aca23:     	callq	0x140613c20 <.text+0x612c20>
1422aca28:     	movaps	0xf0(%rsp), %xmm6
1422aca30:     	movaps	0xe0(%rsp), %xmm7
1422aca38:     	movaps	0xd0(%rsp), %xmm8
1422aca41:     	movaps	0xc0(%rsp), %xmm9
1422aca4a:     	movaps	0xb0(%rsp), %xmm10
1422aca53:     	movaps	0xa0(%rsp), %xmm11
1422aca5c:     	movaps	0x90(%rsp), %xmm12
1422aca65:     	movaps	0x80(%rsp), %xmm13
1422aca6e:     	movaps	0x70(%rsp), %xmm14
1422aca74:     	movaps	0x60(%rsp), %xmm15
1422aca7a:     	addq	$0x108, %rsp            # imm = 0x108
1422aca81:     	popq	%rbx
1422aca82:     	popq	%rdi
1422aca83:     	popq	%rsi
1422aca84:     	popq	%r12
1422aca86:     	popq	%r13
1422aca88:     	popq	%r14
1422aca8a:     	popq	%r15
1422aca8c:     	popq	%rbp
1422aca8d:     	retq
1422aca8e:     	nop
1422aca90:     	movq	%rdx, 0x10(%rsp)
1422aca95:     	pushq	%rbp
1422aca96:     	pushq	%r15
1422aca98:     	pushq	%r14
1422aca9a:     	pushq	%r13
1422aca9c:     	pushq	%r12
1422aca9e:     	pushq	%rsi
1422aca9f:     	pushq	%rdi
1422acaa0:     	pushq	%rbx
1422acaa1:     	subq	$0x108, %rsp            # imm = 0x108
1422acaa8:     	leaq	0x80(%rdx), %rbp
1422acaaf:     	movaps	%xmm15, 0x60(%rsp)
1422acab5:     	movaps	%xmm14, 0x70(%rsp)
1422acabb:     	movaps	%xmm13, 0x80(%rsp)
1422acac4:     	movaps	%xmm12, 0x90(%rsp)
1422acacd:     	movaps	%xmm11, 0xa0(%rsp)
1422acad6:     	movaps	%xmm10, 0xb0(%rsp)
1422acadf:     	movaps	%xmm9, 0xc0(%rsp)
1422acae8:     	movaps	%xmm8, 0xd0(%rsp)
1422acaf1:     	movaps	%xmm7, 0xe0(%rsp)
1422acaf9:     	movaps	%xmm6, 0xf0(%rsp)
1422acb01:     	cmpb	$0x0, 0x177(%rbp)
1422acb08:     	je	0x1422acb7c <.text+0x22abb7c>
1422acb0a:     	leaq	0x158(%rbp), %rcx
1422acb11:     	callq	0x14014a860 <.text+0x149860>
1422acb16:     	movaps	0xf0(%rsp), %xmm6
1422acb1e:     	movaps	0xe0(%rsp), %xmm7
1422acb26:     	movaps	0xd0(%rsp), %xmm8
1422acb2f:     	movaps	0xc0(%rsp), %xmm9
1422acb38:     	movaps	0xb0(%rsp), %xmm10
1422acb41:     	movaps	0xa0(%rsp), %xmm11
1422acb4a:     	movaps	0x90(%rsp), %xmm12
1422acb53:     	movaps	0x80(%rsp), %xmm13
1422acb5c:     	movaps	0x70(%rsp), %xmm14
1422acb62:     	movaps	0x60(%rsp), %xmm15
1422acb68:     	addq	$0x108, %rsp            # imm = 0x108
1422acb6f:     	popq	%rbx
1422acb70:     	popq	%rdi
1422acb71:     	popq	%rsi
1422acb72:     	popq	%r12
1422acb74:     	popq	%r13
1422acb76:     	popq	%r14
1422acb78:     	popq	%r15
1422acb7a:     	popq	%rbp
1422acb7b:     	retq
1422acb7c:     	movaps	0xf0(%rsp), %xmm6
1422acb84:     	movaps	0xe0(%rsp), %xmm7
1422acb8c:     	movaps	0xd0(%rsp), %xmm8
1422acb95:     	movaps	0xc0(%rsp), %xmm9
1422acb9e:     	movaps	0xb0(%rsp), %xmm10
1422acba7:     	movaps	0xa0(%rsp), %xmm11
1422acbb0:     	movaps	0x90(%rsp), %xmm12
1422acbb9:     	movaps	0x80(%rsp), %xmm13
1422acbc2:     	movaps	0x70(%rsp), %xmm14
1422acbc8:     	movaps	0x60(%rsp), %xmm15
1422acbce:     	addq	$0x108, %rsp            # imm = 0x108
1422acbd5:     	popq	%rbx
1422acbd6:     	popq	%rdi
1422acbd7:     	popq	%rsi
1422acbd8:     	popq	%r12
1422acbda:     	popq	%r13
1422acbdc:     	popq	%r14
1422acbde:     	popq	%r15
1422acbe0:     	popq	%rbp
1422acbe1:     	retq
1422acbe2:     	nopw	%cs:(%rax,%rax)
1422acbf0:     	movq	%rdx, 0x10(%rsp)
1422acbf5:     	pushq	%rbp
1422acbf6:     	pushq	%r15
1422acbf8:     	pushq	%r14
1422acbfa:     	pushq	%r13
1422acbfc:     	pushq	%r12
1422acbfe:     	pushq	%rsi
1422acbff:     	pushq	%rdi
1422acc00:     	pushq	%rbx
1422acc01:     	subq	$0x108, %rsp            # imm = 0x108
1422acc08:     	leaq	0x80(%rdx), %rbp
1422acc0f:     	movaps	%xmm15, 0x60(%rsp)
1422acc15:     	movaps	%xmm14, 0x70(%rsp)
1422acc1b:     	movaps	%xmm13, 0x80(%rsp)
1422acc24:     	movaps	%xmm12, 0x90(%rsp)
1422acc2d:     	movaps	%xmm11, 0xa0(%rsp)
1422acc36:     	movaps	%xmm10, 0xb0(%rsp)
1422acc3f:     	movaps	%xmm9, 0xc0(%rsp)
1422acc48:     	movaps	%xmm8, 0xd0(%rsp)
1422acc51:     	movaps	%xmm7, 0xe0(%rsp)
1422acc59:     	movaps	%xmm6, 0xf0(%rsp)
1422acc61:     	leaq	0x130(%rbp), %rcx
1422acc68:     	callq	0x140ec60f0 <.text+0xec50f0>
1422acc6d:     	leaq	0x110(%rbp), %rcx
1422acc74:     	callq	0x14014a860 <.text+0x149860>
1422acc79:     	movaps	0xf0(%rsp), %xmm6
1422acc81:     	movaps	0xe0(%rsp), %xmm7
1422acc89:     	movaps	0xd0(%rsp), %xmm8
1422acc92:     	movaps	0xc0(%rsp), %xmm9
1422acc9b:     	movaps	0xb0(%rsp), %xmm10
1422acca4:     	movaps	0xa0(%rsp), %xmm11
1422accad:     	movaps	0x90(%rsp), %xmm12
1422accb6:     	movaps	0x80(%rsp), %xmm13
1422accbf:     	movaps	0x70(%rsp), %xmm14
1422accc5:     	movaps	0x60(%rsp), %xmm15
1422acccb:     	addq	$0x108, %rsp            # imm = 0x108
1422accd2:     	popq	%rbx
1422accd3:     	popq	%rdi
1422accd4:     	popq	%rsi
1422accd5:     	popq	%r12
1422accd7:     	popq	%r13
1422accd9:     	popq	%r14
1422accdb:     	popq	%r15
1422accdd:     	popq	%rbp
1422accde:     	retq
