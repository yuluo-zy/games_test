
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1426246e0:     	pushq	%rbp
1426246e1:     	pushq	%r15
1426246e3:     	pushq	%r14
1426246e5:     	pushq	%r13
1426246e7:     	pushq	%r12
1426246e9:     	pushq	%rsi
1426246ea:     	pushq	%rdi
1426246eb:     	pushq	%rbx
1426246ec:     	subq	$0x2f8, %rsp            # imm = 0x2F8
1426246f3:     	leaq	0x80(%rsp), %rbp
1426246fb:     	movaps	%xmm7, 0x260(%rbp)
142624702:     	movaps	%xmm6, 0x250(%rbp)
142624709:     	movq	$-0x2, 0x248(%rbp)
142624714:     	movq	%r9, %rdi
142624717:     	movq	%r8, 0x238(%rbp)
14262471e:     	movq	%rdx, %r14
142624721:     	movq	%rcx, %rbx
142624724:     	leaq	-0x30(%rbp), %rcx
142624728:     	callq	0x141e4ccb0 <.text+0x1e4bcb0>
14262472d:     	movzbl	0x10104d5(%rip), %eax   # 0x143634c09
142624734:     	movl	$0x210, %ecx            # imm = 0x210
142624739:     	movl	$0x8, %edx
14262473e:     	callq	0x140613c10 <.text+0x612c10>
142624743:     	testq	%rax, %rax
142624746:     	je	0x142625c54 <.text+0x2624c54>
14262474c:     	movq	%rax, %rsi
14262474f:     	leaq	0xb25126(%rip), %rax    # 0x14314987c
142624756:     	movq	%rax, 0x120(%rbp)
14262475d:     	movq	$0x18, 0x128(%rbp)
142624768:     	movb	$0xc, 0x130(%rbp)
14262476f:     	movq	$0x0, 0x138(%rbp)
14262477a:     	movq	$0x8, 0x140(%rbp)
142624785:     	movq	$0x0, 0x148(%rbp)
142624790:     	movzbl	0x1010472(%rip), %eax   # 0x143634c09
142624797:     	movl	$0x40, %ecx
14262479c:     	movl	$0x8, %edx
1426247a1:     	callq	0x140613c10 <.text+0x612c10>
1426247a6:     	testq	%rax, %rax
1426247a9:     	movq	%rsi, 0x158(%rbp)
1426247b0:     	je	0x142625c68 <.text+0x2624c68>
1426247b6:     	movq	%rax, %r12
1426247b9:     	movzbl	0x1010449(%rip), %eax   # 0x143634c09
1426247c0:     	movl	$0x100, %ecx            # imm = 0x100
1426247c5:     	movl	$0x8, %edx
1426247ca:     	callq	0x140613c10 <.text+0x612c10>
1426247cf:     	testq	%rax, %rax
1426247d2:     	je	0x142625c7c <.text+0x2624c7c>
1426247d8:     	movq	%rax, %rsi
1426247db:     	movzbl	0x1010427(%rip), %eax   # 0x143634c09
1426247e2:     	movl	$0x1, %ecx
1426247e7:     	movl	$0x1, %edx
1426247ec:     	callq	0x140613c10 <.text+0x612c10>
1426247f1:     	testq	%rax, %rax
1426247f4:     	je	0x142625c97 <.text+0x2624c97>
1426247fa:     	movb	$0x0, (%rax)
1426247fd:     	leaq	0xb25090(%rip), %rcx    # 0x143149894
142624804:     	movq	%rcx, 0x180(%rbp)
14262480b:     	movq	$0x22, 0x188(%rbp)
142624816:     	movsd	0x309e92(%rip), %xmm6   # 0x14292e6b0
14262481e:     	movsd	%xmm6, 0x190(%rbp)
142624826:     	movb	$0x0, 0x160(%rbp)
14262482d:     	movq	%rax, 0x168(%rbp)
142624834:     	leaq	0xb24f1d(%rip), %r15    # 0x143149758
14262483b:     	movq	%r15, 0x170(%rbp)
142624842:     	movb	$0x0, 0x198(%rbp)
142624849:     	movzbl	0x10103b9(%rip), %eax   # 0x143634c09
142624850:     	movl	$0x1, %ecx
142624855:     	movl	$0x1, %edx
14262485a:     	callq	0x140613c10 <.text+0x612c10>
14262485f:     	testq	%rax, %rax
142624862:     	je	0x142625cb9 <.text+0x2624cb9>
142624868:     	movb	$0x1, (%rax)
14262486b:     	leaq	0xb25044(%rip), %rcx    # 0x1431498b6
142624872:     	movq	%rcx, 0x1c0(%rbp)
142624879:     	movq	$0x1d, 0x1c8(%rbp)
142624884:     	movsd	%xmm6, 0x1d0(%rbp)
14262488c:     	movb	$0x0, 0x1a0(%rbp)
142624893:     	movq	%rax, 0x1a8(%rbp)
14262489a:     	movq	%r15, 0x1b0(%rbp)
1426248a1:     	movb	$0x0, 0x1d8(%rbp)
1426248a8:     	movzbl	0x101035a(%rip), %eax   # 0x143634c09
1426248af:     	movl	$0x1, %ecx
1426248b4:     	movl	$0x1, %edx
1426248b9:     	callq	0x140613c10 <.text+0x612c10>
1426248be:     	testq	%rax, %rax
1426248c1:     	je	0x142625cdb <.text+0x2624cdb>
1426248c7:     	movb	$0x2, (%rax)
1426248ca:     	leaq	0xb25002(%rip), %rcx    # 0x1431498d3
1426248d1:     	movq	%rcx, 0x208(%rbp)
1426248d8:     	movq	$0x1d, 0x210(%rbp)
1426248e3:     	movsd	%xmm6, 0x218(%rbp)
1426248eb:     	movb	$0x0, 0x1e8(%rbp)
1426248f2:     	movq	%rax, 0x1f0(%rbp)
1426248f9:     	movq	%r15, 0x1f8(%rbp)
142624900:     	movb	$0x0, 0x220(%rbp)
142624907:     	movq	0x100778a(%rip), %rax   # 0x14362c098
14262490e:     	cmpb	$0x0, (%rax)
142624911:     	je	0x142624924 <.text+0x2623924>
142624913:     	movl	$0x25, %ecx
142624918:     	leaq	0xb24ff1(%rip), %rdx    # 0x143149910
14262491f:     	movb	$0x7, %r8b
142624922:     	jmp	0x142624955 <.text+0x2623955>
142624924:     	movzbl	0x10102de(%rip), %eax   # 0x143634c09
14262492b:     	movl	$0x1, %ecx
142624930:     	movl	$0x1, %edx
142624935:     	callq	0x140613c10 <.text+0x612c10>
14262493a:     	testq	%rax, %rax
14262493d:     	je	0x142625dda <.text+0x2624dda>
142624943:     	movb	$0x3, (%rax)
142624946:     	movl	$0x20, %ecx
14262494b:     	leaq	0xb24f9e(%rip), %rdx    # 0x1431498f0
142624952:     	xorl	%r8d, %r8d
142624955:     	movups	0x160(%rbp), %xmm0
14262495c:     	movups	0x170(%rbp), %xmm1
142624963:     	movups	0x180(%rbp), %xmm2
14262496a:     	movups	0x190(%rbp), %xmm3
142624971:     	movups	%xmm3, 0x30(%rsi)
142624975:     	movups	%xmm2, 0x20(%rsi)
142624979:     	movups	%xmm1, 0x10(%rsi)
14262497d:     	movups	%xmm0, (%rsi)
142624980:     	movups	0x1a0(%rbp), %xmm0
142624987:     	movups	0x1b0(%rbp), %xmm1
14262498e:     	movups	0x1c0(%rbp), %xmm2
142624995:     	movups	0x1d0(%rbp), %xmm3
14262499c:     	movups	%xmm0, 0x40(%rsi)
1426249a0:     	movups	%xmm1, 0x50(%rsi)
1426249a4:     	movups	%xmm2, 0x60(%rsi)
1426249a8:     	movups	%xmm3, 0x70(%rsi)
1426249ac:     	movups	0x1e8(%rbp), %xmm0
1426249b3:     	movups	0x1f8(%rbp), %xmm1
1426249ba:     	movups	0x208(%rbp), %xmm2
1426249c1:     	movups	0x218(%rbp), %xmm3
1426249c8:     	movups	%xmm2, 0xa0(%rsi)
1426249cf:     	movups	%xmm1, 0x90(%rsi)
1426249d6:     	movups	%xmm0, 0x80(%rsi)
1426249dd:     	movups	%xmm3, 0xb0(%rsi)
1426249e4:     	movb	%r8b, 0xc0(%rsi)
1426249eb:     	movq	%rax, 0xc8(%rsi)
1426249f2:     	movq	%r15, 0xd0(%rsi)
1426249f9:     	movq	%rdx, 0xe0(%rsi)
142624a00:     	movq	%rcx, 0xe8(%rsi)
142624a07:     	movsd	%xmm6, 0xf0(%rsi)
142624a0f:     	movb	$0x0, 0xf8(%rsi)
142624a16:     	movb	$0x4, (%r12)
142624a1b:     	movl	0x1e8(%rbp), %eax
142624a21:     	movl	0x1eb(%rbp), %ecx
142624a27:     	movl	%eax, 0x1(%r12)
142624a2c:     	movl	%ecx, 0x4(%r12)
142624a31:     	movq	$0x4, 0x8(%r12)
142624a3a:     	movq	%rsi, 0x10(%r12)
142624a3f:     	movq	$0x4, 0x18(%r12)
142624a48:     	movq	$0x1, 0x20(%r12)
142624a51:     	movq	$0x0, 0x28(%r12)
142624a5a:     	movsd	%xmm6, 0x30(%r12)
142624a61:     	movb	$0x0, 0x38(%r12)
142624a67:     	movl	0x1a0(%rbp), %eax
142624a6d:     	movl	0x1a3(%rbp), %ecx
142624a73:     	movl	%ecx, 0x3c(%r12)
142624a78:     	movl	%eax, 0x39(%r12)
142624a7d:     	leaq	0xb24eb1(%rip), %rax    # 0x143149935
142624a84:     	movq	%rax, 0xf0(%rbp)
142624a8b:     	movq	$0x1d, 0xf8(%rbp)
142624a96:     	movb	$0xd, 0x100(%rbp)
142624a9d:     	movq	$0x1, 0x108(%rbp)
142624aa8:     	movq	%r12, 0x110(%rbp)
142624aaf:     	movq	$0x1, 0x118(%rbp)
142624aba:     	movzbl	0x1010148(%rip), %eax   # 0x143634c09
142624ac1:     	movl	$0xc0, %ecx
142624ac6:     	movl	$0x8, %edx
142624acb:     	callq	0x140613c10 <.text+0x612c10>
142624ad0:     	testq	%rax, %rax
142624ad3:     	je	0x142625cfd <.text+0x2624cfd>
142624ad9:     	movq	%rbx, 0x228(%rbp)
142624ae0:     	movb	$0x6, (%rax)
142624ae3:     	leaq	0xb24e68(%rip), %rcx    # 0x143149952
142624aea:     	movq	%rcx, 0x20(%rax)
142624aee:     	movq	$0x1c, 0x28(%rax)
142624af6:     	movsd	%xmm6, 0x30(%rax)
142624afb:     	movb	$0x1, 0x38(%rax)
142624aff:     	movw	$0x402, 0x40(%rax)      # imm = 0x402
142624b05:     	leaq	0xb24e62(%rip), %r13    # 0x14314996e
142624b0c:     	movq	%r13, 0x60(%rax)
142624b10:     	movq	$0x11, 0x68(%rax)
142624b18:     	movsd	0x96aac0(%rip), %xmm7   # 0x142f8f5e0
142624b20:     	movsd	%xmm7, 0x70(%rax)
142624b25:     	movb	$0x1, 0x78(%rax)
142624b29:     	movw	$0x403, 0x80(%rax)      # imm = 0x403
142624b32:     	leaq	0xb24e46(%rip), %r12    # 0x14314997f
142624b39:     	movq	%r12, 0xa0(%rax)
142624b40:     	movq	$0xd, 0xa8(%rax)
142624b4b:     	movsd	%xmm6, 0xb0(%rax)
142624b53:     	movb	$0x1, 0xb8(%rax)
142624b5a:     	movq	$0x1, 0x1c0(%rbp)
142624b65:     	movq	$0x0, 0x1c8(%rbp)
142624b70:     	movsd	%xmm6, 0x1d0(%rbp)
142624b78:     	movb	$0x4, 0x1a0(%rbp)
142624b7f:     	movq	$0x3, 0x1a8(%rbp)
142624b8a:     	movq	%rax, 0x1b0(%rbp)
142624b91:     	movq	$0x3, 0x1b8(%rbp)
142624b9c:     	movb	$0x0, 0x1d8(%rbp)
142624ba3:     	movzbl	0x101005f(%rip), %eax   # 0x143634c09
142624baa:     	movl	$0xc0, %ecx
142624baf:     	movl	$0x8, %edx
142624bb4:     	callq	0x140613c10 <.text+0x612c10>
142624bb9:     	testq	%rax, %rax
142624bbc:     	je	0x142625d11 <.text+0x2624d11>
142624bc2:     	movb	$0x6, (%rax)
142624bc5:     	leaq	0xb24dc0(%rip), %rsi    # 0x14314998c
142624bcc:     	movq	%rsi, 0x20(%rax)
142624bd0:     	movq	$0x15, 0x28(%rax)
142624bd8:     	movsd	%xmm6, 0x30(%rax)
142624bdd:     	movb	$0x0, 0x38(%rax)
142624be1:     	movw	$0x602, 0x40(%rax)      # imm = 0x602
142624be7:     	movq	%r13, 0x60(%rax)
142624beb:     	movq	$0x11, 0x68(%rax)
142624bf3:     	movsd	%xmm7, 0x70(%rax)
142624bf8:     	movb	$0x0, 0x78(%rax)
142624bfc:     	movw	$0x603, 0x80(%rax)      # imm = 0x603
142624c05:     	movq	%r12, 0xa0(%rax)
142624c0c:     	movq	$0xd, 0xa8(%rax)
142624c17:     	movsd	%xmm6, 0xb0(%rax)
142624c1f:     	movb	$0x0, 0xb8(%rax)
142624c26:     	movq	$0x1, 0x208(%rbp)
142624c31:     	movq	$0x0, 0x210(%rbp)
142624c3c:     	movsd	%xmm6, 0x218(%rbp)
142624c44:     	movb	$0x4, 0x1e8(%rbp)
142624c4b:     	movq	$0x3, 0x1f0(%rbp)
142624c56:     	movq	%rax, 0x1f8(%rbp)
142624c5d:     	movq	$0x3, 0x200(%rbp)
142624c68:     	movb	$0x0, 0x220(%rbp)
142624c6f:     	movzbl	0x100ff93(%rip), %eax   # 0x143634c09
142624c76:     	movq	0x100741b(%rip), %rax   # 0x14362c098
142624c7d:     	movzbl	(%rax), %ebx
142624c80:     	movl	$0x40, %ecx
142624c85:     	movl	$0x8, %edx
142624c8a:     	callq	0x140613c10 <.text+0x612c10>
142624c8f:     	testb	%bl, %bl
142624c91:     	je	0x142624cdb <.text+0x2623cdb>
142624c93:     	testq	%rax, %rax
142624c96:     	je	0x142625dfc <.text+0x2624dfc>
142624c9c:     	movq	%rax, %rbx
142624c9f:     	movups	0x1e8(%rbp), %xmm0
142624ca6:     	movups	0x1f8(%rbp), %xmm1
142624cad:     	movups	0x208(%rbp), %xmm2
142624cb4:     	movups	0x218(%rbp), %xmm3
142624cbb:     	movups	%xmm3, 0x30(%rax)
142624cbf:     	movups	%xmm2, 0x20(%rax)
142624cc3:     	movups	%xmm1, 0x10(%rax)
142624cc7:     	movups	%xmm0, (%rax)
142624cca:     	leaq	0x1a0(%rbp), %rcx
142624cd1:     	callq	0x140faa6e0 <.text+0xfa96e0>
142624cd6:     	jmp	0x142624d8d <.text+0x2623d8d>
142624cdb:     	testq	%rax, %rax
142624cde:     	je	0x142625e10 <.text+0x2624e10>
142624ce4:     	movq	%rax, %rbx
142624ce7:     	movzbl	0x100ff1b(%rip), %eax   # 0x143634c09
142624cee:     	movl	$0x80, %ecx
142624cf3:     	movl	$0x8, %edx
142624cf8:     	callq	0x140613c10 <.text+0x612c10>
142624cfd:     	testq	%rax, %rax
142624d00:     	je	0x142625e24 <.text+0x2624e24>
142624d06:     	movups	0x1a0(%rbp), %xmm0
142624d0d:     	movups	0x1b0(%rbp), %xmm1
142624d14:     	movups	0x1c0(%rbp), %xmm2
142624d1b:     	movups	0x1d0(%rbp), %xmm3
142624d22:     	movups	%xmm3, 0x30(%rax)
142624d26:     	movups	%xmm2, 0x20(%rax)
142624d2a:     	movups	%xmm1, 0x10(%rax)
142624d2e:     	movups	%xmm0, (%rax)
142624d31:     	movups	0x1e8(%rbp), %xmm0
142624d38:     	movups	0x1f8(%rbp), %xmm1
142624d3f:     	movups	0x208(%rbp), %xmm2
142624d46:     	movups	0x218(%rbp), %xmm3
142624d4d:     	movups	%xmm0, 0x40(%rax)
142624d51:     	movups	%xmm1, 0x50(%rax)
142624d55:     	movups	%xmm2, 0x60(%rax)
142624d59:     	movups	%xmm3, 0x70(%rax)
142624d5d:     	movb	$0x5, (%rbx)
142624d60:     	movq	$0x2, 0x8(%rbx)
142624d68:     	movq	%rax, 0x10(%rbx)
142624d6c:     	movq	$0x2, 0x18(%rbx)
142624d74:     	movq	$0x1, 0x20(%rbx)
142624d7c:     	movq	$0x0, 0x28(%rbx)
142624d84:     	movsd	%xmm6, 0x30(%rbx)
142624d89:     	movb	$0x0, 0x38(%rbx)
142624d8d:     	movq	%rsi, 0xc0(%rbp)
142624d94:     	movq	$0x15, 0xc8(%rbp)
142624d9f:     	movb	$0xe, 0xd0(%rbp)
142624da6:     	movq	$0x1, 0xd8(%rbp)
142624db1:     	movq	%rbx, 0xe0(%rbp)
142624db8:     	movq	$0x1, 0xe8(%rbp)
142624dc3:     	movzbl	0x100fe3f(%rip), %eax   # 0x143634c09
142624dca:     	movl	$0xc0, %ecx
142624dcf:     	movl	$0x8, %edx
142624dd4:     	callq	0x140613c10 <.text+0x612c10>
142624dd9:     	testq	%rax, %rax
142624ddc:     	je	0x142625d25 <.text+0x2624d25>
142624de2:     	movb	$0x6, (%rax)
142624de5:     	leaq	0xb24bb5(%rip), %rcx    # 0x1431499a1
142624dec:     	movq	%rcx, 0x20(%rax)
142624df0:     	movq	$0x1a, 0x28(%rax)
142624df8:     	movsd	%xmm6, 0x30(%rax)
142624dfd:     	movb	$0x0, 0x38(%rax)
142624e01:     	movw	$0x302, 0x40(%rax)      # imm = 0x302
142624e07:     	movq	%r13, 0x60(%rax)
142624e0b:     	movq	$0x11, 0x68(%rax)
142624e13:     	movsd	%xmm7, 0x70(%rax)
142624e18:     	movb	$0x0, 0x78(%rax)
142624e1c:     	movw	$0x303, 0x80(%rax)      # imm = 0x303
142624e25:     	movq	%r12, 0xa0(%rax)
142624e2c:     	movq	$0xd, 0xa8(%rax)
142624e37:     	movsd	%xmm6, 0xb0(%rax)
142624e3f:     	movb	$0x0, 0xb8(%rax)
142624e46:     	movq	$0x1, 0x1c0(%rbp)
142624e51:     	movq	$0x0, 0x1c8(%rbp)
142624e5c:     	movsd	%xmm6, 0x1d0(%rbp)
142624e64:     	movb	$0x4, 0x1a0(%rbp)
142624e6b:     	movq	$0x3, 0x1a8(%rbp)
142624e76:     	movq	%rax, 0x1b0(%rbp)
142624e7d:     	movq	$0x3, 0x1b8(%rbp)
142624e88:     	movb	$0x0, 0x1d8(%rbp)
142624e8f:     	movzbl	0x100fd73(%rip), %eax   # 0x143634c09
142624e96:     	movl	$0xc0, %ecx
142624e9b:     	movl	$0x8, %edx
142624ea0:     	callq	0x140613c10 <.text+0x612c10>
142624ea5:     	testq	%rax, %rax
142624ea8:     	je	0x142625d39 <.text+0x2624d39>
142624eae:     	movb	$0x6, (%rax)
142624eb1:     	leaq	0xb24b03(%rip), %rsi    # 0x1431499bb
142624eb8:     	movq	%rsi, 0x20(%rax)
142624ebc:     	movq	$0x15, 0x28(%rax)
142624ec4:     	movsd	%xmm6, 0x30(%rax)
142624ec9:     	movb	$0x0, 0x38(%rax)
142624ecd:     	movw	$0x802, 0x40(%rax)      # imm = 0x802
142624ed3:     	movq	%r13, 0x60(%rax)
142624ed7:     	movq	$0x11, 0x68(%rax)
142624edf:     	movsd	%xmm7, 0x70(%rax)
142624ee4:     	movb	$0x0, 0x78(%rax)
142624ee8:     	movw	$0x803, 0x80(%rax)      # imm = 0x803
142624ef1:     	movq	%r12, 0xa0(%rax)
142624ef8:     	movq	$0xd, 0xa8(%rax)
142624f03:     	movsd	%xmm6, 0xb0(%rax)
142624f0b:     	movb	$0x0, 0xb8(%rax)
142624f12:     	movq	$0x1, 0x208(%rbp)
142624f1d:     	movq	$0x0, 0x210(%rbp)
142624f28:     	movsd	%xmm6, 0x218(%rbp)
142624f30:     	movb	$0x4, 0x1e8(%rbp)
142624f37:     	movq	$0x3, 0x1f0(%rbp)
142624f42:     	movq	%rax, 0x1f8(%rbp)
142624f49:     	movq	$0x3, 0x200(%rbp)
142624f54:     	movb	$0x0, 0x220(%rbp)
142624f5b:     	movzbl	0x100fca7(%rip), %eax   # 0x143634c09
142624f62:     	movq	0x100712f(%rip), %rax   # 0x14362c098
142624f69:     	movzbl	(%rax), %ebx
142624f6c:     	movl	$0x40, %ecx
142624f71:     	movl	$0x8, %edx
142624f76:     	callq	0x140613c10 <.text+0x612c10>
142624f7b:     	testb	%bl, %bl
142624f7d:     	je	0x142624fc7 <.text+0x2623fc7>
142624f7f:     	testq	%rax, %rax
142624f82:     	je	0x142625e3f <.text+0x2624e3f>
142624f88:     	movq	%rax, %rbx
142624f8b:     	movups	0x1e8(%rbp), %xmm0
142624f92:     	movups	0x1f8(%rbp), %xmm1
142624f99:     	movups	0x208(%rbp), %xmm2
142624fa0:     	movups	0x218(%rbp), %xmm3
142624fa7:     	movups	%xmm3, 0x30(%rax)
142624fab:     	movups	%xmm2, 0x20(%rax)
142624faf:     	movups	%xmm1, 0x10(%rax)
142624fb3:     	movups	%xmm0, (%rax)
142624fb6:     	leaq	0x1a0(%rbp), %rcx
142624fbd:     	callq	0x140faa6e0 <.text+0xfa96e0>
142624fc2:     	jmp	0x142625079 <.text+0x2624079>
142624fc7:     	testq	%rax, %rax
142624fca:     	je	0x142625e53 <.text+0x2624e53>
142624fd0:     	movq	%rax, %rbx
142624fd3:     	movzbl	0x100fc2f(%rip), %eax   # 0x143634c09
142624fda:     	movl	$0x80, %ecx
142624fdf:     	movl	$0x8, %edx
142624fe4:     	callq	0x140613c10 <.text+0x612c10>
142624fe9:     	testq	%rax, %rax
142624fec:     	je	0x142625e67 <.text+0x2624e67>
142624ff2:     	movups	0x1a0(%rbp), %xmm0
142624ff9:     	movups	0x1b0(%rbp), %xmm1
142625000:     	movups	0x1c0(%rbp), %xmm2
142625007:     	movups	0x1d0(%rbp), %xmm3
14262500e:     	movups	%xmm3, 0x30(%rax)
142625012:     	movups	%xmm2, 0x20(%rax)
142625016:     	movups	%xmm1, 0x10(%rax)
14262501a:     	movups	%xmm0, (%rax)
14262501d:     	movups	0x1e8(%rbp), %xmm0
142625024:     	movups	0x1f8(%rbp), %xmm1
14262502b:     	movups	0x208(%rbp), %xmm2
142625032:     	movups	0x218(%rbp), %xmm3
142625039:     	movups	%xmm0, 0x40(%rax)
14262503d:     	movups	%xmm1, 0x50(%rax)
142625041:     	movups	%xmm2, 0x60(%rax)
142625045:     	movups	%xmm3, 0x70(%rax)
142625049:     	movb	$0x5, (%rbx)
14262504c:     	movq	$0x2, 0x8(%rbx)
142625054:     	movq	%rax, 0x10(%rbx)
142625058:     	movq	$0x2, 0x18(%rbx)
142625060:     	movq	$0x1, 0x20(%rbx)
142625068:     	movq	$0x0, 0x28(%rbx)
142625070:     	movsd	%xmm6, 0x30(%rbx)
142625075:     	movb	$0x0, 0x38(%rbx)
142625079:     	movq	%rsi, 0x90(%rbp)
142625080:     	movq	$0x15, 0x98(%rbp)
14262508b:     	movb	$0x10, 0xa0(%rbp)
142625092:     	movq	$0x1, 0xa8(%rbp)
14262509d:     	movq	%rbx, 0xb0(%rbp)
1426250a4:     	movq	$0x1, 0xb8(%rbp)
1426250af:     	movzbl	0x100fb53(%rip), %eax   # 0x143634c09
1426250b6:     	movl	$0x40, %ecx
1426250bb:     	movl	$0x8, %edx
1426250c0:     	callq	0x140613c10 <.text+0x612c10>
1426250c5:     	testq	%rax, %rax
1426250c8:     	je	0x142625d4d <.text+0x2624d4d>
1426250ce:     	movq	%rax, %rsi
1426250d1:     	movzbl	0x100fb31(%rip), %eax   # 0x143634c09
1426250d8:     	movl	$0xc0, %ecx
1426250dd:     	movl	$0x8, %edx
1426250e2:     	callq	0x140613c10 <.text+0x612c10>
1426250e7:     	testq	%rax, %rax
1426250ea:     	je	0x142625d61 <.text+0x2624d61>
1426250f0:     	movq	%rax, %rbx
1426250f3:     	movzbl	0x100fb0f(%rip), %eax   # 0x143634c09
1426250fa:     	movl	$0x1, %ecx
1426250ff:     	movl	$0x1, %edx
142625104:     	callq	0x140613c10 <.text+0x612c10>
142625109:     	testq	%rax, %rax
14262510c:     	je	0x142625d7c <.text+0x2624d7c>
142625112:     	movb	$0x10, (%rax)
142625115:     	movb	$0x0, (%rbx)
142625118:     	movq	%rax, 0x8(%rbx)
14262511c:     	movq	%r15, 0x10(%rbx)
142625120:     	leaq	0xb248a9(%rip), %rax    # 0x1431499d0
142625127:     	movq	%rax, 0x20(%rbx)
14262512b:     	movq	$0x15, 0x28(%rbx)
142625133:     	movsd	%xmm6, 0x30(%rbx)
142625138:     	movb	$0x0, 0x38(%rbx)
14262513c:     	movw	$0x702, 0x40(%rbx)      # imm = 0x702
142625142:     	movq	%r13, 0x60(%rbx)
142625146:     	movq	$0x11, 0x68(%rbx)
14262514e:     	movsd	%xmm7, 0x70(%rbx)
142625153:     	movb	$0x0, 0x78(%rbx)
142625157:     	movw	$0x703, 0x80(%rbx)      # imm = 0x703
142625160:     	movq	%r12, 0xa0(%rbx)
142625167:     	movq	$0xd, 0xa8(%rbx)
142625172:     	movsd	%xmm6, 0xb0(%rbx)
14262517a:     	movb	$0x0, 0xb8(%rbx)
142625181:     	movb	$0x4, (%rsi)
142625184:     	movq	$0x3, 0x8(%rsi)
14262518c:     	movq	%rbx, 0x10(%rsi)
142625190:     	movq	$0x3, 0x18(%rsi)
142625198:     	movq	$0x1, 0x20(%rsi)
1426251a0:     	movq	$0x0, 0x28(%rsi)
1426251a8:     	movsd	%xmm6, 0x30(%rsi)
1426251ad:     	movb	$0x0, 0x38(%rsi)
1426251b1:     	leaq	0xb2482d(%rip), %rax    # 0x1431499e5
1426251b8:     	movq	%rax, 0x60(%rbp)
1426251bc:     	movq	$0x17, 0x68(%rbp)
1426251c4:     	movb	$0xf, 0x70(%rbp)
1426251c8:     	movq	$0x1, 0x78(%rbp)
1426251d0:     	movq	%rsi, 0x80(%rbp)
1426251d7:     	movq	$0x1, 0x88(%rbp)
1426251e2:     	leaq	0xb24813(%rip), %rax    # 0x1431499fc
1426251e9:     	movq	%rax, 0x30(%rbp)
1426251ed:     	movq	$0x1e, 0x38(%rbp)
1426251f5:     	movb	$0x11, 0x40(%rbp)
1426251f9:     	movq	$0x0, 0x48(%rbp)
142625201:     	movq	$0x8, 0x50(%rbp)
142625209:     	movq	$0x0, 0x58(%rbp)
142625211:     	movzbl	0x100f9f1(%rip), %eax   # 0x143634c09
142625218:     	movl	$0xc0, %ecx
14262521d:     	movl	$0x8, %edx
142625222:     	callq	0x140613c10 <.text+0x612c10>
142625227:     	testq	%rax, %rax
14262522a:     	je	0x142625d9e <.text+0x2624d9e>
142625230:     	movb	$0x6, (%rax)
142625233:     	leaq	0xb247e0(%rip), %rcx    # 0x143149a1a
14262523a:     	movq	%rcx, 0x20(%rax)
14262523e:     	movq	$0x1c, 0x28(%rax)
142625246:     	movsd	%xmm6, 0x30(%rax)
14262524b:     	movb	$0x0, 0x38(%rax)
14262524f:     	movw	$0x202, 0x40(%rax)      # imm = 0x202
142625255:     	movq	%r13, 0x60(%rax)
142625259:     	movq	$0x11, 0x68(%rax)
142625261:     	movsd	%xmm7, 0x70(%rax)
142625266:     	movb	$0x0, 0x78(%rax)
14262526a:     	movw	$0x203, 0x80(%rax)      # imm = 0x203
142625273:     	movq	%r12, 0xa0(%rax)
14262527a:     	movq	$0xd, 0xa8(%rax)
142625285:     	movsd	%xmm6, 0xb0(%rax)
14262528d:     	movb	$0x0, 0xb8(%rax)
142625294:     	movq	$0x1, 0x180(%rbp)
14262529f:     	movq	$0x0, 0x188(%rbp)
1426252aa:     	movsd	%xmm6, 0x190(%rbp)
1426252b2:     	movb	$0x4, 0x160(%rbp)
1426252b9:     	movq	$0x3, 0x168(%rbp)
1426252c4:     	movq	%rax, 0x170(%rbp)
1426252cb:     	movq	$0x3, 0x178(%rbp)
1426252d6:     	movb	$0x0, 0x198(%rbp)
1426252dd:     	movzbl	0x100f925(%rip), %eax   # 0x143634c09
1426252e4:     	movl	$0xc0, %ecx
1426252e9:     	movl	$0x8, %edx
1426252ee:     	callq	0x140613c10 <.text+0x612c10>
1426252f3:     	testq	%rax, %rax
1426252f6:     	je	0x142625db2 <.text+0x2624db2>
1426252fc:     	movb	$0x6, (%rax)
1426252ff:     	leaq	0xb24730(%rip), %rcx    # 0x143149a36
142625306:     	movq	%rcx, 0x20(%rax)
14262530a:     	movq	$0x1a, 0x28(%rax)
142625312:     	movsd	%xmm6, 0x30(%rax)
142625317:     	movb	$0x0, 0x38(%rax)
14262531b:     	movw	$0x102, 0x40(%rax)      # imm = 0x102
142625321:     	movq	%r13, 0x60(%rax)
142625325:     	movq	$0x11, 0x68(%rax)
14262532d:     	movsd	%xmm7, 0x70(%rax)
142625332:     	movb	$0x0, 0x78(%rax)
142625336:     	movw	$0x103, 0x80(%rax)      # imm = 0x103
14262533f:     	movq	%r12, 0xa0(%rax)
142625346:     	movq	$0xd, 0xa8(%rax)
142625351:     	movsd	%xmm6, 0xb0(%rax)
142625359:     	movb	$0x0, 0xb8(%rax)
142625360:     	movq	$0x1, 0x1c0(%rbp)
14262536b:     	movq	$0x0, 0x1c8(%rbp)
142625376:     	movsd	%xmm6, 0x1d0(%rbp)
14262537e:     	movb	$0x4, 0x1a0(%rbp)
142625385:     	movq	$0x3, 0x1a8(%rbp)
142625390:     	movq	%rax, 0x1b0(%rbp)
142625397:     	movq	$0x3, 0x1b8(%rbp)
1426253a2:     	movb	$0x0, 0x1d8(%rbp)
1426253a9:     	movzbl	0x100f859(%rip), %eax   # 0x143634c09
1426253b0:     	movl	$0xc0, %ecx
1426253b5:     	movl	$0x8, %edx
1426253ba:     	callq	0x140613c10 <.text+0x612c10>
1426253bf:     	testq	%rax, %rax
1426253c2:     	je	0x142625dc6 <.text+0x2624dc6>
1426253c8:     	movb	$0x6, (%rax)
1426253cb:     	leaq	0xb2467e(%rip), %rcx    # 0x143149a50
1426253d2:     	movq	%rcx, 0x20(%rax)
1426253d6:     	movq	$0x14, 0x28(%rax)
1426253de:     	movsd	%xmm6, 0x30(%rax)
1426253e3:     	movb	$0x0, 0x38(%rax)
1426253e7:     	movw	$0x2, 0x40(%rax)
1426253ed:     	movq	%r13, 0x60(%rax)
1426253f1:     	movq	$0x11, 0x68(%rax)
1426253f9:     	movsd	%xmm7, 0x70(%rax)
1426253fe:     	movb	$0x0, 0x78(%rax)
142625402:     	movw	$0x3, 0x80(%rax)
14262540b:     	movq	%r12, 0xa0(%rax)
142625412:     	movq	$0xd, 0xa8(%rax)
14262541d:     	movsd	%xmm6, 0xb0(%rax)
142625425:     	movb	$0x0, 0xb8(%rax)
14262542c:     	movq	$0x1, 0x208(%rbp)
142625437:     	movq	$0x0, 0x210(%rbp)
142625442:     	movsd	%xmm6, 0x218(%rbp)
14262544a:     	movb	$0x4, 0x1e8(%rbp)
142625451:     	movq	$0x3, 0x1f0(%rbp)
14262545c:     	movq	%rax, 0x1f8(%rbp)
142625463:     	movq	$0x3, 0x200(%rbp)
14262546e:     	movb	$0x0, 0x220(%rbp)
142625475:     	movzbl	0x100f78d(%rip), %eax   # 0x143634c09
14262547c:     	movq	0x1006c15(%rip), %rax   # 0x14362c098
142625483:     	movzbl	(%rax), %ebx
142625486:     	movl	$0x40, %ecx
14262548b:     	movl	$0x8, %edx
142625490:     	callq	0x140613c10 <.text+0x612c10>
142625495:     	movq	%rax, %rsi
142625498:     	testb	%bl, %bl
14262549a:     	je	0x1426254ed <.text+0x26244ed>
14262549c:     	testq	%rsi, %rsi
14262549f:     	je	0x142625e82 <.text+0x2624e82>
1426254a5:     	movups	0x1e8(%rbp), %xmm0
1426254ac:     	movups	0x1f8(%rbp), %xmm1
1426254b3:     	movups	0x208(%rbp), %xmm2
1426254ba:     	movups	0x218(%rbp), %xmm3
1426254c1:     	movups	%xmm3, 0x30(%rsi)
1426254c5:     	movups	%xmm2, 0x20(%rsi)
1426254c9:     	movups	%xmm1, 0x10(%rsi)
1426254cd:     	movups	%xmm0, (%rsi)
1426254d0:     	leaq	0x1a0(%rbp), %rcx
1426254d7:     	callq	0x140faa6e0 <.text+0xfa96e0>
1426254dc:     	leaq	0x160(%rbp), %rcx
1426254e3:     	callq	0x140faa6e0 <.text+0xfa96e0>
1426254e8:     	jmp	0x1426255d4 <.text+0x26245d4>
1426254ed:     	testq	%rsi, %rsi
1426254f0:     	je	0x142625e96 <.text+0x2624e96>
1426254f6:     	movzbl	0x100f70c(%rip), %eax   # 0x143634c09
1426254fd:     	movl	$0xc0, %ecx
142625502:     	movl	$0x8, %edx
142625507:     	callq	0x140613c10 <.text+0x612c10>
14262550c:     	testq	%rax, %rax
14262550f:     	je	0x142625eaa <.text+0x2624eaa>
142625515:     	movups	0x160(%rbp), %xmm0
14262551c:     	movups	0x170(%rbp), %xmm1
142625523:     	movups	0x180(%rbp), %xmm2
14262552a:     	movups	0x190(%rbp), %xmm3
142625531:     	movups	%xmm3, 0x30(%rax)
142625535:     	movups	%xmm2, 0x20(%rax)
142625539:     	movups	%xmm1, 0x10(%rax)
14262553d:     	movups	%xmm0, (%rax)
142625540:     	movups	0x1a0(%rbp), %xmm0
142625547:     	movups	0x1b0(%rbp), %xmm1
14262554e:     	movups	0x1c0(%rbp), %xmm2
142625555:     	movups	0x1d0(%rbp), %xmm3
14262555c:     	movups	%xmm0, 0x40(%rax)
142625560:     	movups	%xmm1, 0x50(%rax)
142625564:     	movups	%xmm2, 0x60(%rax)
142625568:     	movups	%xmm3, 0x70(%rax)
14262556c:     	movups	0x1e8(%rbp), %xmm0
142625573:     	movups	0x1f8(%rbp), %xmm1
14262557a:     	movups	0x208(%rbp), %xmm2
142625581:     	movups	0x218(%rbp), %xmm3
142625588:     	movups	%xmm0, 0x80(%rax)
14262558f:     	movups	%xmm1, 0x90(%rax)
142625596:     	movups	%xmm2, 0xa0(%rax)
14262559d:     	movups	%xmm3, 0xb0(%rax)
1426255a4:     	movb	$0x5, (%rsi)
1426255a7:     	movq	$0x3, 0x8(%rsi)
1426255af:     	movq	%rax, 0x10(%rsi)
1426255b3:     	movq	$0x3, 0x18(%rsi)
1426255bb:     	movq	$0x1, 0x20(%rsi)
1426255c3:     	movq	$0x0, 0x28(%rsi)
1426255cb:     	movsd	%xmm6, 0x30(%rsi)
1426255d0:     	movb	$0x0, 0x38(%rsi)
1426255d4:     	leaq	0xb24489(%rip), %rax    # 0x143149a64
1426255db:     	movq	%rax, (%rbp)
1426255df:     	movq	$0x1a, 0x8(%rbp)
1426255e7:     	movb	$0x12, 0x10(%rbp)
1426255eb:     	movq	$0x1, 0x18(%rbp)
1426255f3:     	movq	%rsi, 0x20(%rbp)
1426255f7:     	movq	$0x1, 0x28(%rbp)
1426255ff:     	movq	0x1006a92(%rip), %rax   # 0x14362c098
142625606:     	cmpb	$0x0, (%rax)
142625609:     	je	0x14262566c <.text+0x262466c>
14262560b:     	leaq	0xb24482(%rip), %rax    # 0x143149a94
142625612:     	movq	%rax, 0x160(%rbp)
142625619:     	movq	$0x1b, 0x168(%rbp)
142625624:     	movb	$0x13, 0x170(%rbp)
14262562b:     	movq	$0x0, 0x178(%rbp)
142625636:     	movq	$0x8, 0x180(%rbp)
142625641:     	movq	$0x0, 0x188(%rbp)
14262564c:     	leaq	0xb244cd(%rip), %rax    # 0x143149b20
142625653:     	movl	$0x1f, %ecx
142625658:     	movl	$0x8, %esi
14262565d:     	xorl	%r12d, %r12d
142625660:     	movq	0x158(%rbp), %r13
142625667:     	jmp	0x1426258ae <.text+0x26248ae>
14262566c:     	leaq	0xb2440b(%rip), %rax    # 0x143149a7e
142625673:     	movq	%rax, 0x160(%rbp)
14262567a:     	movq	$0x16, 0x168(%rbp)
142625685:     	movb	$0x13, 0x170(%rbp)
14262568c:     	movq	$0x0, 0x178(%rbp)
142625697:     	movq	$0x8, 0x180(%rbp)
1426256a2:     	movq	$0x0, 0x188(%rbp)
1426256ad:     	movzbl	0x100f555(%rip), %eax   # 0x143634c09
1426256b4:     	movl	$0x40, %ecx
1426256b9:     	movl	$0x8, %edx
1426256be:     	callq	0x140613c10 <.text+0x612c10>
1426256c3:     	testq	%rax, %rax
1426256c6:     	movq	0x158(%rbp), %r13
1426256cd:     	je	0x142625ec5 <.text+0x2624ec5>
1426256d3:     	movq	%rax, %rsi
1426256d6:     	movzbl	0x100f52c(%rip), %eax   # 0x143634c09
1426256dd:     	movl	$0xc0, %ecx
1426256e2:     	movl	$0x8, %edx
1426256e7:     	callq	0x140613c10 <.text+0x612c10>
1426256ec:     	testq	%rax, %rax
1426256ef:     	je	0x142625ed6 <.text+0x2624ed6>
1426256f5:     	movq	%rax, %rbx
1426256f8:     	movzbl	0x100f50a(%rip), %eax   # 0x143634c09
1426256ff:     	movl	$0x1, %ecx
142625704:     	movl	$0x1, %edx
142625709:     	callq	0x140613c10 <.text+0x612c10>
14262570e:     	testq	%rax, %rax
142625711:     	je	0x142625eee <.text+0x2624eee>
142625717:     	movb	$0xc, (%rax)
14262571a:     	leaq	0xb2438e(%rip), %rcx    # 0x143149aaf
142625721:     	movq	%rcx, 0x1c0(%rbp)
142625728:     	movq	$0x1b, 0x1c8(%rbp)
142625733:     	movsd	%xmm6, 0x1d0(%rbp)
14262573b:     	movb	$0x0, 0x1a0(%rbp)
142625742:     	movq	%rax, 0x1a8(%rbp)
142625749:     	movq	%r15, 0x1b0(%rbp)
142625750:     	movb	$0x0, 0x1d8(%rbp)
142625757:     	movzbl	0x100f4ab(%rip), %eax   # 0x143634c09
14262575e:     	movl	$0x1, %ecx
142625763:     	movl	$0x1, %edx
142625768:     	callq	0x140613c10 <.text+0x612c10>
14262576d:     	testq	%rax, %rax
142625770:     	je	0x142625f0d <.text+0x2624f0d>
142625776:     	movb	$0xd, (%rax)
142625779:     	leaq	0xb2434a(%rip), %rcx    # 0x143149aca
142625780:     	movq	%rcx, 0x208(%rbp)
142625787:     	movq	$0x1b, 0x210(%rbp)
142625792:     	movsd	%xmm6, 0x218(%rbp)
14262579a:     	movb	$0x0, 0x1e8(%rbp)
1426257a1:     	movq	%rax, 0x1f0(%rbp)
1426257a8:     	movq	%r15, 0x1f8(%rbp)
1426257af:     	movb	$0x0, 0x220(%rbp)
1426257b6:     	movzbl	0x100f44c(%rip), %eax   # 0x143634c09
1426257bd:     	movl	$0x1, %r12d
1426257c3:     	movl	$0x1, %ecx
1426257c8:     	movl	$0x1, %edx
1426257cd:     	callq	0x140613c10 <.text+0x612c10>
1426257d2:     	testq	%rax, %rax
1426257d5:     	je	0x142625f2c <.text+0x2624f2c>
1426257db:     	movb	$0xe, (%rax)
1426257de:     	movups	0x1a0(%rbp), %xmm0
1426257e5:     	movups	0x1b0(%rbp), %xmm1
1426257ec:     	movups	0x1c0(%rbp), %xmm2
1426257f3:     	movups	0x1d0(%rbp), %xmm3
1426257fa:     	movups	%xmm3, 0x30(%rbx)
1426257fe:     	movups	%xmm2, 0x20(%rbx)
142625802:     	movups	%xmm1, 0x10(%rbx)
142625806:     	movups	%xmm0, (%rbx)
142625809:     	movups	0x1e8(%rbp), %xmm0
142625810:     	movups	0x1f8(%rbp), %xmm1
142625817:     	movups	0x208(%rbp), %xmm2
14262581e:     	movups	0x218(%rbp), %xmm3
142625825:     	movups	%xmm0, 0x40(%rbx)
142625829:     	movups	%xmm1, 0x50(%rbx)
14262582d:     	movups	%xmm2, 0x60(%rbx)
142625831:     	movups	%xmm3, 0x70(%rbx)
142625835:     	movb	$0x0, 0x80(%rbx)
14262583c:     	movq	%rax, 0x88(%rbx)
142625843:     	movq	%r15, 0x90(%rbx)
14262584a:     	leaq	0xb24294(%rip), %rax    # 0x143149ae5
142625851:     	movq	%rax, 0xa0(%rbx)
142625858:     	movq	$0x21, 0xa8(%rbx)
142625863:     	movsd	%xmm6, 0xb0(%rbx)
14262586b:     	movb	$0x0, 0xb8(%rbx)
142625872:     	movb	$0x4, (%rsi)
142625875:     	movq	$0x3, 0x8(%rsi)
14262587d:     	movq	%rbx, 0x10(%rsi)
142625881:     	movq	$0x3, 0x18(%rsi)
142625889:     	movq	$0x1, 0x20(%rsi)
142625891:     	movq	$0x0, 0x28(%rsi)
142625899:     	movsd	%xmm6, 0x30(%rsi)
14262589e:     	movb	$0x0, 0x38(%rsi)
1426258a2:     	leaq	0xb2425d(%rip), %rax    # 0x143149b06
1426258a9:     	movl	$0x1a, %ecx
1426258ae:     	movups	0x120(%rbp), %xmm0
1426258b5:     	movups	0x130(%rbp), %xmm1
1426258bc:     	movups	0x140(%rbp), %xmm2
1426258c3:     	movups	%xmm2, 0x20(%r13)
1426258c8:     	movups	%xmm1, 0x10(%r13)
1426258cd:     	movups	%xmm0, (%r13)
1426258d2:     	movups	0xf0(%rbp), %xmm0
1426258d9:     	movups	0x100(%rbp), %xmm1
1426258e0:     	movups	0x110(%rbp), %xmm2
1426258e7:     	movups	%xmm2, 0x50(%r13)
1426258ec:     	movups	%xmm1, 0x40(%r13)
1426258f1:     	movups	%xmm0, 0x30(%r13)
1426258f6:     	movups	0xc0(%rbp), %xmm0
1426258fd:     	movups	0xd0(%rbp), %xmm1
142625904:     	movups	0xe0(%rbp), %xmm2
14262590b:     	movups	%xmm2, 0x80(%r13)
142625913:     	movups	%xmm1, 0x70(%r13)
142625918:     	movups	%xmm0, 0x60(%r13)
14262591d:     	movups	0x90(%rbp), %xmm0
142625924:     	movups	0xa0(%rbp), %xmm1
14262592b:     	movups	0xb0(%rbp), %xmm2
142625932:     	movups	%xmm0, 0x90(%r13)
14262593a:     	movups	%xmm1, 0xa0(%r13)
142625942:     	movups	%xmm2, 0xb0(%r13)
14262594a:     	movups	0x60(%rbp), %xmm0
14262594e:     	movups	0x70(%rbp), %xmm1
142625952:     	movups	0x80(%rbp), %xmm2
142625959:     	movups	%xmm1, 0xd0(%r13)
142625961:     	movups	%xmm2, 0xe0(%r13)
142625969:     	movups	%xmm0, 0xc0(%r13)
142625971:     	movups	0x30(%rbp), %xmm0
142625975:     	movups	0x40(%rbp), %xmm1
142625979:     	movups	0x50(%rbp), %xmm2
14262597d:     	movups	%xmm0, 0xf0(%r13)
142625985:     	movups	%xmm1, 0x100(%r13)
14262598d:     	movups	%xmm2, 0x110(%r13)
142625995:     	movups	(%rbp), %xmm0
142625999:     	movups	0x10(%rbp), %xmm1
14262599d:     	movups	0x20(%rbp), %xmm2
1426259a1:     	movups	%xmm0, 0x120(%r13)
1426259a9:     	movups	%xmm1, 0x130(%r13)
1426259b1:     	movups	%xmm2, 0x140(%r13)
1426259b9:     	movups	0x160(%rbp), %xmm0
1426259c0:     	movups	0x170(%rbp), %xmm1
1426259c7:     	movups	0x180(%rbp), %xmm2
1426259ce:     	movups	%xmm0, 0x150(%r13)
1426259d6:     	movups	%xmm2, 0x170(%r13)
1426259de:     	movups	%xmm1, 0x160(%r13)
1426259e6:     	movq	%rax, 0x180(%r13)
1426259ed:     	movq	%rcx, 0x188(%r13)
1426259f4:     	movb	$0x14, 0x190(%r13)
1426259fc:     	movl	0x151(%rbp), %eax
142625a02:     	movl	0x154(%rbp), %ecx
142625a08:     	movl	%ecx, 0x194(%r13)
142625a0f:     	movl	%eax, 0x191(%r13)
142625a16:     	movq	%r12, 0x198(%r13)
142625a1d:     	movq	%rsi, 0x1a0(%r13)
142625a24:     	movq	%r12, 0x1a8(%r13)
142625a2b:     	leaq	0xb2410d(%rip), %rax    # 0x143149b3f
142625a32:     	movq	%rax, 0x1b0(%r13)
142625a39:     	movq	$0x15, 0x1b8(%r13)
142625a44:     	movb	$0xb, 0x1c0(%r13)
142625a4c:     	movq	$0x0, 0x1c8(%r13)
142625a57:     	movq	$0x8, 0x1d0(%r13)
142625a62:     	movq	$0x0, 0x1d8(%r13)
142625a6d:     	leaq	0xb240e0(%rip), %rax    # 0x143149b54
142625a74:     	movq	%rax, 0x1e0(%r13)
142625a7b:     	movq	$0x16, 0x1e8(%r13)
142625a86:     	movb	$0x15, 0x1f0(%r13)
142625a8e:     	movq	$0x0, 0x1f8(%r13)
142625a99:     	movq	$0x8, 0x200(%r13)
142625aa4:     	movq	$0x0, 0x208(%r13)
142625aaf:     	movq	%r13, %rax
142625ab2:     	addq	$0x210, %rax            # imm = 0x210
142625ab8:     	movq	%r13, 0x1e8(%rbp)
142625abf:     	movq	%r13, 0x1f0(%rbp)
142625ac6:     	movq	$0xb, 0x1f8(%rbp)
142625ad1:     	movq	%rax, 0x200(%rbp)
142625ad8:     	leaq	0x168(%rbp), %rsi
142625adf:     	addq	$0x8, 0x238(%rbp)
142625ae7:     	leaq	0x236(%rbp), %rbx
142625aee:     	leaq	-0x30(%rbp), %r15
142625af2:     	leaq	0x1a0(%rbp), %r12
142625af9:     	cmpq	%rax, %r13
142625afc:     	je	0x142625bee <.text+0x2624bee>
142625b02:     	nopw	%cs:(%rax,%rax)
142625b10:     	leaq	0x30(%r13), %rax
142625b14:     	movq	%rax, 0x1f0(%rbp)
142625b1b:     	movq	0x18(%r13), %rax
142625b1f:     	movq	%rax, %rcx
142625b22:     	negq	%rcx
142625b25:     	jo	0x142625bee <.text+0x2624bee>
142625b2b:     	movq	%r13, %rdx
142625b2e:     	movb	0x10(%r13), %r13b
142625b32:     	movq	(%rdx), %r9
142625b35:     	movq	0x8(%rdx), %rcx
142625b39:     	movb	%r13b, 0x236(%rbp)
142625b40:     	movq	%rax, 0x160(%rbp)
142625b47:     	movups	0x20(%rdx), %xmm0
142625b4b:     	movups	%xmm0, (%rsi)
142625b4e:     	xorl	%eax, %eax
142625b50:     	cmpb	$0x11, %r13b
142625b54:     	sete	%al
142625b57:     	movb	$0x1, 0x247(%rbp)
142625b5e:     	movl	%eax, 0x38(%rsp)
142625b62:     	movq	%rbx, 0x30(%rsp)
142625b67:     	movq	%rdi, 0x28(%rsp)
142625b6c:     	movq	%rcx, 0x20(%rsp)
142625b71:     	movl	$0x3fa00000, 0x40(%rsp) # imm = 0x3FA00000
142625b79:     	movq	%r15, %rcx
142625b7c:     	movq	%r14, %rdx
142625b7f:     	movl	%r13d, %r8d
142625b82:     	callq	0x142623cb0 <.text+0x2622cb0>
142625b87:     	movq	0x170(%rbp), %rax
142625b8e:     	movq	%rax, 0x1b0(%rbp)
142625b95:     	movups	0x160(%rbp), %xmm0
142625b9c:     	movaps	%xmm0, 0x1a0(%rbp)
142625ba3:     	movb	$0x1, 0x237(%rbp)
142625baa:     	movq	0x238(%rbp), %rcx
142625bb1:     	callq	0x140b0e070 <.text+0xb0d070>
142625bb6:     	movb	$0x0, 0x237(%rbp)
142625bbd:     	movq	%rdi, 0x28(%rsp)
142625bc2:     	movb	%al, 0x20(%rsp)
142625bc6:     	movq	%r15, %rcx
142625bc9:     	movq	%r14, %rdx
142625bcc:     	movl	%r13d, %r8d
142625bcf:     	movq	%r12, %r9
142625bd2:     	callq	0x141e4d1e0 <.text+0x1e4c1e0>
142625bd7:     	movq	0x1f0(%rbp), %r13
142625bde:     	movq	0x200(%rbp), %rax
142625be5:     	cmpq	%rax, %r13
142625be8:     	jne	0x142625b10 <.text+0x2624b10>
142625bee:     	leaq	0x1e8(%rbp), %rcx
142625bf5:     	callq	0x140fa3670 <.text+0xfa2670>
142625bfa:     	movq	0x238(%rbp), %rcx
142625c01:     	callq	0x140b0e070 <.text+0xb0d070>
142625c06:     	leaq	-0x30(%rbp), %rcx
142625c0a:     	movl	%eax, %edx
142625c0c:     	movq	%rdi, %r8
142625c0f:     	callq	0x141e4d880 <.text+0x1e4c880>
142625c14:     	movups	-0x30(%rbp), %xmm0
142625c18:     	movups	-0x20(%rbp), %xmm1
142625c1c:     	movups	-0x10(%rbp), %xmm2
142625c20:     	movq	0x228(%rbp), %rax
142625c27:     	movups	%xmm2, 0x20(%rax)
142625c2b:     	movups	%xmm1, 0x10(%rax)
142625c2f:     	movups	%xmm0, (%rax)
142625c32:     	movaps	0x250(%rbp), %xmm6
142625c39:     	movaps	0x260(%rbp), %xmm7
142625c40:     	addq	$0x2f8, %rsp            # imm = 0x2F8
142625c47:     	popq	%rbx
142625c48:     	popq	%rdi
142625c49:     	popq	%rsi
142625c4a:     	popq	%r12
142625c4c:     	popq	%r13
142625c4e:     	popq	%r14
142625c50:     	popq	%r15
142625c52:     	popq	%rbp
142625c53:     	retq
142625c54:     	movl	$0x8, %ecx
142625c59:     	movl	$0x210, %edx            # imm = 0x210
142625c5e:     	callq	0x1428d9000 <.text+0x28d8000>
142625c63:     	jmp	0x142625f49 <.text+0x2624f49>
142625c68:     	movl	$0x8, %ecx
142625c6d:     	movl	$0x40, %edx
142625c72:     	callq	0x1428d9000 <.text+0x28d8000>
142625c77:     	jmp	0x142625f49 <.text+0x2624f49>
142625c7c:     	movq	%r12, 0x238(%rbp)
142625c83:     	movl	$0x8, %ecx
142625c88:     	movl	$0x100, %edx            # imm = 0x100
142625c8d:     	callq	0x1428d9000 <.text+0x28d8000>
142625c92:     	jmp	0x142625f49 <.text+0x2624f49>
142625c97:     	movq	%rsi, 0x228(%rbp)
142625c9e:     	movq	%r12, 0x238(%rbp)
142625ca5:     	movl	$0x1, %ecx
142625caa:     	movl	$0x1, %edx
142625caf:     	callq	0x1428d9000 <.text+0x28d8000>
142625cb4:     	jmp	0x142625f49 <.text+0x2624f49>
142625cb9:     	movq	%rsi, 0x228(%rbp)
142625cc0:     	movq	%r12, 0x238(%rbp)
142625cc7:     	movl	$0x1, %ecx
142625ccc:     	movl	$0x1, %edx
142625cd1:     	callq	0x1428d9000 <.text+0x28d8000>
142625cd6:     	jmp	0x142625f49 <.text+0x2624f49>
142625cdb:     	movq	%rsi, 0x228(%rbp)
142625ce2:     	movq	%r12, 0x238(%rbp)
142625ce9:     	movl	$0x1, %ecx
142625cee:     	movl	$0x1, %edx
142625cf3:     	callq	0x1428d9000 <.text+0x28d8000>
142625cf8:     	jmp	0x142625f49 <.text+0x2624f49>
142625cfd:     	movl	$0x8, %ecx
142625d02:     	movl	$0xc0, %edx
142625d07:     	callq	0x1428d9000 <.text+0x28d8000>
142625d0c:     	jmp	0x142625f49 <.text+0x2624f49>
142625d11:     	movl	$0x8, %ecx
142625d16:     	movl	$0xc0, %edx
142625d1b:     	callq	0x1428d9000 <.text+0x28d8000>
142625d20:     	jmp	0x142625f49 <.text+0x2624f49>
142625d25:     	movl	$0x8, %ecx
142625d2a:     	movl	$0xc0, %edx
142625d2f:     	callq	0x1428d9000 <.text+0x28d8000>
142625d34:     	jmp	0x142625f49 <.text+0x2624f49>
142625d39:     	movl	$0x8, %ecx
142625d3e:     	movl	$0xc0, %edx
142625d43:     	callq	0x1428d9000 <.text+0x28d8000>
142625d48:     	jmp	0x142625f49 <.text+0x2624f49>
142625d4d:     	movl	$0x8, %ecx
142625d52:     	movl	$0x40, %edx
142625d57:     	callq	0x1428d9000 <.text+0x28d8000>
142625d5c:     	jmp	0x142625f49 <.text+0x2624f49>
142625d61:     	movq	%rsi, 0x238(%rbp)
142625d68:     	movl	$0x8, %ecx
142625d6d:     	movl	$0xc0, %edx
142625d72:     	callq	0x1428d9000 <.text+0x28d8000>
142625d77:     	jmp	0x142625f49 <.text+0x2624f49>
142625d7c:     	movq	%rbx, 0x228(%rbp)
142625d83:     	movq	%rsi, 0x238(%rbp)
142625d8a:     	movl	$0x1, %ecx
142625d8f:     	movl	$0x1, %edx
142625d94:     	callq	0x1428d9000 <.text+0x28d8000>
142625d99:     	jmp	0x142625f49 <.text+0x2624f49>
142625d9e:     	movl	$0x8, %ecx
142625da3:     	movl	$0xc0, %edx
142625da8:     	callq	0x1428d9000 <.text+0x28d8000>
142625dad:     	jmp	0x142625f49 <.text+0x2624f49>
142625db2:     	movl	$0x8, %ecx
142625db7:     	movl	$0xc0, %edx
142625dbc:     	callq	0x1428d9000 <.text+0x28d8000>
142625dc1:     	jmp	0x142625f49 <.text+0x2624f49>
142625dc6:     	movl	$0x8, %ecx
142625dcb:     	movl	$0xc0, %edx
142625dd0:     	callq	0x1428d9000 <.text+0x28d8000>
142625dd5:     	jmp	0x142625f49 <.text+0x2624f49>
142625dda:     	movq	%rsi, 0x228(%rbp)
142625de1:     	movq	%r12, 0x238(%rbp)
142625de8:     	movl	$0x1, %ecx
142625ded:     	movl	$0x1, %edx
142625df2:     	callq	0x1428d9000 <.text+0x28d8000>
142625df7:     	jmp	0x142625f49 <.text+0x2624f49>
142625dfc:     	movl	$0x8, %ecx
142625e01:     	movl	$0x40, %edx
142625e06:     	callq	0x1428d9000 <.text+0x28d8000>
142625e0b:     	jmp	0x142625f49 <.text+0x2624f49>
142625e10:     	movl	$0x8, %ecx
142625e15:     	movl	$0x40, %edx
142625e1a:     	callq	0x1428d9000 <.text+0x28d8000>
142625e1f:     	jmp	0x142625f49 <.text+0x2624f49>
142625e24:     	movq	%rbx, 0x238(%rbp)
142625e2b:     	movl	$0x8, %ecx
142625e30:     	movl	$0x80, %edx
142625e35:     	callq	0x1428d9000 <.text+0x28d8000>
142625e3a:     	jmp	0x142625f49 <.text+0x2624f49>
142625e3f:     	movl	$0x8, %ecx
142625e44:     	movl	$0x40, %edx
142625e49:     	callq	0x1428d9000 <.text+0x28d8000>
142625e4e:     	jmp	0x142625f49 <.text+0x2624f49>
142625e53:     	movl	$0x8, %ecx
142625e58:     	movl	$0x40, %edx
142625e5d:     	callq	0x1428d9000 <.text+0x28d8000>
142625e62:     	jmp	0x142625f49 <.text+0x2624f49>
142625e67:     	movq	%rbx, 0x238(%rbp)
142625e6e:     	movl	$0x8, %ecx
142625e73:     	movl	$0x80, %edx
142625e78:     	callq	0x1428d9000 <.text+0x28d8000>
142625e7d:     	jmp	0x142625f49 <.text+0x2624f49>
142625e82:     	movl	$0x8, %ecx
142625e87:     	movl	$0x40, %edx
142625e8c:     	callq	0x1428d9000 <.text+0x28d8000>
142625e91:     	jmp	0x142625f49 <.text+0x2624f49>
142625e96:     	movl	$0x8, %ecx
142625e9b:     	movl	$0x40, %edx
142625ea0:     	callq	0x1428d9000 <.text+0x28d8000>
142625ea5:     	jmp	0x142625f49 <.text+0x2624f49>
142625eaa:     	movq	%rsi, 0x238(%rbp)
142625eb1:     	movl	$0x8, %ecx
142625eb6:     	movl	$0xc0, %edx
142625ebb:     	callq	0x1428d9000 <.text+0x28d8000>
142625ec0:     	jmp	0x142625f49 <.text+0x2624f49>
142625ec5:     	movl	$0x8, %ecx
142625eca:     	movl	$0x40, %edx
142625ecf:     	callq	0x1428d9000 <.text+0x28d8000>
142625ed4:     	jmp	0x142625f49 <.text+0x2624f49>
142625ed6:     	movq	%rsi, 0x238(%rbp)
142625edd:     	movl	$0x8, %ecx
142625ee2:     	movl	$0xc0, %edx
142625ee7:     	callq	0x1428d9000 <.text+0x28d8000>
142625eec:     	jmp	0x142625f49 <.text+0x2624f49>
142625eee:     	movq	%rbx, 0x228(%rbp)
142625ef5:     	movq	%rsi, 0x238(%rbp)
142625efc:     	movl	$0x1, %ecx
142625f01:     	movl	$0x1, %edx
142625f06:     	callq	0x1428d9000 <.text+0x28d8000>
142625f0b:     	jmp	0x142625f49 <.text+0x2624f49>
142625f0d:     	movq	%rbx, 0x228(%rbp)
142625f14:     	movq	%rsi, 0x238(%rbp)
142625f1b:     	movl	$0x1, %ecx
142625f20:     	movl	$0x1, %edx
142625f25:     	callq	0x1428d9000 <.text+0x28d8000>
142625f2a:     	jmp	0x142625f49 <.text+0x2624f49>
142625f2c:     	movq	%rbx, 0x228(%rbp)
142625f33:     	movq	%rsi, 0x238(%rbp)
142625f3a:     	movl	$0x1, %ecx
142625f3f:     	movl	$0x1, %edx
142625f44:     	callq	0x1428d9000 <.text+0x28d8000>
142625f49:     	ud2
142625f4b:     	nopl	(%rax,%rax)
142625f50:     	movq	%rdx, 0x10(%rsp)
142625f55:     	pushq	%rbp
142625f56:     	pushq	%r15
142625f58:     	pushq	%r14
142625f5a:     	pushq	%r13
142625f5c:     	pushq	%r12
142625f5e:     	pushq	%rsi
142625f5f:     	pushq	%rdi
142625f60:     	pushq	%rbx
142625f61:     	subq	$0x68, %rsp
142625f65:     	leaq	0x80(%rdx), %rbp
142625f6c:     	movaps	%xmm7, 0x40(%rsp)
142625f71:     	movaps	%xmm6, 0x50(%rsp)
142625f76:     	movq	-0x30(%rbp), %rdx
142625f7a:     	testq	%rdx, %rdx
142625f7d:     	je	0x142625f92 <.text+0x2624f92>
142625f7f:     	movq	-0x28(%rbp), %rcx
142625f83:     	shlq	$0x5, %rdx
142625f87:     	movl	$0x8, %r8d
142625f8d:     	callq	0x140613c20 <.text+0x612c20>
142625f92:     	movaps	0x50(%rsp), %xmm6
142625f97:     	movaps	0x40(%rsp), %xmm7
142625f9c:     	addq	$0x68, %rsp
142625fa0:     	popq	%rbx
142625fa1:     	popq	%rdi
142625fa2:     	popq	%rsi
142625fa3:     	popq	%r12
142625fa5:     	popq	%r13
142625fa7:     	popq	%r14
142625fa9:     	popq	%r15
142625fab:     	popq	%rbp
142625fac:     	retq
142625fad:     	nopl	(%rax)
142625fb0:     	movq	%rdx, 0x10(%rsp)
142625fb5:     	pushq	%rbp
142625fb6:     	pushq	%r15
142625fb8:     	pushq	%r14
142625fba:     	pushq	%r13
142625fbc:     	pushq	%r12
142625fbe:     	pushq	%rsi
142625fbf:     	pushq	%rdi
142625fc0:     	pushq	%rbx
142625fc1:     	subq	$0x68, %rsp
142625fc5:     	leaq	0x80(%rdx), %rbp
142625fcc:     	movaps	%xmm7, 0x40(%rsp)
142625fd1:     	movaps	%xmm6, 0x50(%rsp)
142625fd6:     	leaq	0x138(%rbp), %rcx
142625fdd:     	callq	0x140fa19f0 <.text+0xfa09f0>
142625fe2:     	movl	$0x210, %edx            # imm = 0x210
142625fe7:     	movl	$0x8, %r8d
142625fed:     	movq	0x158(%rbp), %rcx
142625ff4:     	callq	0x140613c20 <.text+0x612c20>
142625ff9:     	movaps	0x50(%rsp), %xmm6
142625ffe:     	movaps	0x40(%rsp), %xmm7
142626003:     	addq	$0x68, %rsp
142626007:     	popq	%rbx
142626008:     	popq	%rdi
142626009:     	popq	%rsi
14262600a:     	popq	%r12
14262600c:     	popq	%r13
14262600e:     	popq	%r14
142626010:     	popq	%r15
142626012:     	popq	%rbp
142626013:     	retq
142626014:     	nopw	%cs:(%rax,%rax)
142626020:     	movq	%rdx, 0x10(%rsp)
142626025:     	pushq	%rbp
142626026:     	pushq	%r15
142626028:     	pushq	%r14
14262602a:     	pushq	%r13
14262602c:     	pushq	%r12
14262602e:     	pushq	%rsi
14262602f:     	pushq	%rdi
142626030:     	pushq	%rbx
142626031:     	subq	$0x68, %rsp
142626035:     	leaq	0x80(%rdx), %rbp
14262603c:     	movaps	%xmm7, 0x40(%rsp)
142626041:     	movaps	%xmm6, 0x50(%rsp)
142626046:     	movl	$0x40, %edx
14262604b:     	movl	$0x8, %r8d
142626051:     	movq	0x238(%rbp), %rcx
142626058:     	callq	0x140613c20 <.text+0x612c20>
14262605d:     	movaps	0x50(%rsp), %xmm6
142626062:     	movaps	0x40(%rsp), %xmm7
142626067:     	addq	$0x68, %rsp
14262606b:     	popq	%rbx
14262606c:     	popq	%rdi
14262606d:     	popq	%rsi
14262606e:     	popq	%r12
142626070:     	popq	%r13
142626072:     	popq	%r14
142626074:     	popq	%r15
142626076:     	popq	%rbp
142626077:     	retq
142626078:     	nopl	(%rax,%rax)
142626080:     	movq	%rdx, 0x10(%rsp)
142626085:     	pushq	%rbp
142626086:     	pushq	%r15
142626088:     	pushq	%r14
14262608a:     	pushq	%r13
14262608c:     	pushq	%r12
14262608e:     	pushq	%rsi
14262608f:     	pushq	%rdi
142626090:     	pushq	%rbx
142626091:     	subq	$0x68, %rsp
142626095:     	leaq	0x80(%rdx), %rbp
14262609c:     	movaps	%xmm7, 0x40(%rsp)
1426260a1:     	movaps	%xmm6, 0x50(%rsp)
1426260a6:     	movl	$0x100, %edx            # imm = 0x100
1426260ab:     	movl	$0x8, %r8d
1426260b1:     	movq	0x228(%rbp), %rcx
1426260b8:     	callq	0x140613c20 <.text+0x612c20>
1426260bd:     	movaps	0x50(%rsp), %xmm6
1426260c2:     	movaps	0x40(%rsp), %xmm7
1426260c7:     	addq	$0x68, %rsp
1426260cb:     	popq	%rbx
1426260cc:     	popq	%rdi
1426260cd:     	popq	%rsi
1426260ce:     	popq	%r12
1426260d0:     	popq	%r13
1426260d2:     	popq	%r14
1426260d4:     	popq	%r15
1426260d6:     	popq	%rbp
1426260d7:     	retq
1426260d8:     	nopl	(%rax,%rax)
1426260e0:     	movq	%rdx, 0x10(%rsp)
1426260e5:     	pushq	%rbp
1426260e6:     	pushq	%r15
1426260e8:     	pushq	%r14
1426260ea:     	pushq	%r13
1426260ec:     	pushq	%r12
1426260ee:     	pushq	%rsi
1426260ef:     	pushq	%rdi
1426260f0:     	pushq	%rbx
1426260f1:     	subq	$0x68, %rsp
1426260f5:     	leaq	0x80(%rdx), %rbp
1426260fc:     	movaps	%xmm7, 0x40(%rsp)
142626101:     	movaps	%xmm6, 0x50(%rsp)
142626106:     	leaq	0x160(%rbp), %rcx
14262610d:     	callq	0x140faa6e0 <.text+0xfa96e0>
142626112:     	movaps	0x50(%rsp), %xmm6
142626117:     	movaps	0x40(%rsp), %xmm7
14262611c:     	addq	$0x68, %rsp
142626120:     	popq	%rbx
142626121:     	popq	%rdi
142626122:     	popq	%rsi
142626123:     	popq	%r12
142626125:     	popq	%r13
142626127:     	popq	%r14
142626129:     	popq	%r15
14262612b:     	popq	%rbp
14262612c:     	retq
14262612d:     	nopl	(%rax)
142626130:     	movq	%rdx, 0x10(%rsp)
142626135:     	pushq	%rbp
142626136:     	pushq	%r15
142626138:     	pushq	%r14
14262613a:     	pushq	%r13
14262613c:     	pushq	%r12
14262613e:     	pushq	%rsi
14262613f:     	pushq	%rdi
142626140:     	pushq	%rbx
142626141:     	subq	$0x68, %rsp
142626145:     	leaq	0x80(%rdx), %rbp
14262614c:     	movaps	%xmm7, 0x40(%rsp)
142626151:     	movaps	%xmm6, 0x50(%rsp)
142626156:     	leaq	0x1a0(%rbp), %rcx
14262615d:     	callq	0x140faa6e0 <.text+0xfa96e0>
142626162:     	movaps	0x50(%rsp), %xmm6
142626167:     	movaps	0x40(%rsp), %xmm7
14262616c:     	addq	$0x68, %rsp
142626170:     	popq	%rbx
142626171:     	popq	%rdi
142626172:     	popq	%rsi
142626173:     	popq	%r12
142626175:     	popq	%r13
142626177:     	popq	%r14
142626179:     	popq	%r15
14262617b:     	popq	%rbp
14262617c:     	retq
14262617d:     	nopl	(%rax)
142626180:     	movq	%rdx, 0x10(%rsp)
142626185:     	pushq	%rbp
142626186:     	pushq	%r15
142626188:     	pushq	%r14
14262618a:     	pushq	%r13
14262618c:     	pushq	%r12
14262618e:     	pushq	%rsi
14262618f:     	pushq	%rdi
142626190:     	pushq	%rbx
142626191:     	subq	$0x68, %rsp
142626195:     	leaq	0x80(%rdx), %rbp
14262619c:     	movaps	%xmm7, 0x40(%rsp)
1426261a1:     	movaps	%xmm6, 0x50(%rsp)
1426261a6:     	leaq	0x1e8(%rbp), %rcx
1426261ad:     	callq	0x140faa6e0 <.text+0xfa96e0>
1426261b2:     	movaps	0x50(%rsp), %xmm6
1426261b7:     	movaps	0x40(%rsp), %xmm7
1426261bc:     	addq	$0x68, %rsp
1426261c0:     	popq	%rbx
1426261c1:     	popq	%rdi
1426261c2:     	popq	%rsi
1426261c3:     	popq	%r12
1426261c5:     	popq	%r13
1426261c7:     	popq	%r14
1426261c9:     	popq	%r15
1426261cb:     	popq	%rbp
1426261cc:     	retq
1426261cd:     	nopl	(%rax)
1426261d0:     	movq	%rdx, 0x10(%rsp)
1426261d5:     	pushq	%rbp
1426261d6:     	pushq	%r15
1426261d8:     	pushq	%r14
1426261da:     	pushq	%r13
1426261dc:     	pushq	%r12
1426261de:     	pushq	%rsi
1426261df:     	pushq	%rdi
1426261e0:     	pushq	%rbx
1426261e1:     	subq	$0x68, %rsp
1426261e5:     	leaq	0x80(%rdx), %rbp
1426261ec:     	movaps	%xmm7, 0x40(%rsp)
1426261f1:     	movaps	%xmm6, 0x50(%rsp)
1426261f6:     	leaq	0x108(%rbp), %rcx
1426261fd:     	callq	0x140fa19f0 <.text+0xfa09f0>
142626202:     	movaps	0x50(%rsp), %xmm6
142626207:     	movaps	0x40(%rsp), %xmm7
14262620c:     	addq	$0x68, %rsp
142626210:     	popq	%rbx
142626211:     	popq	%rdi
142626212:     	popq	%rsi
142626213:     	popq	%r12
142626215:     	popq	%r13
142626217:     	popq	%r14
142626219:     	popq	%r15
14262621b:     	popq	%rbp
14262621c:     	retq
14262621d:     	nopl	(%rax)
142626220:     	movq	%rdx, 0x10(%rsp)
142626225:     	pushq	%rbp
142626226:     	pushq	%r15
142626228:     	pushq	%r14
14262622a:     	pushq	%r13
14262622c:     	pushq	%r12
14262622e:     	pushq	%rsi
14262622f:     	pushq	%rdi
142626230:     	pushq	%rbx
142626231:     	subq	$0x68, %rsp
142626235:     	leaq	0x80(%rdx), %rbp
14262623c:     	movaps	%xmm7, 0x40(%rsp)
142626241:     	movaps	%xmm6, 0x50(%rsp)
142626246:     	leaq	0x1a0(%rbp), %rcx
14262624d:     	callq	0x140faa6e0 <.text+0xfa96e0>
142626252:     	movaps	0x50(%rsp), %xmm6
142626257:     	movaps	0x40(%rsp), %xmm7
14262625c:     	addq	$0x68, %rsp
142626260:     	popq	%rbx
142626261:     	popq	%rdi
142626262:     	popq	%rsi
142626263:     	popq	%r12
142626265:     	popq	%r13
142626267:     	popq	%r14
142626269:     	popq	%r15
14262626b:     	popq	%rbp
14262626c:     	retq
14262626d:     	nopl	(%rax)
142626270:     	movq	%rdx, 0x10(%rsp)
142626275:     	pushq	%rbp
142626276:     	pushq	%r15
142626278:     	pushq	%r14
14262627a:     	pushq	%r13
14262627c:     	pushq	%r12
14262627e:     	pushq	%rsi
14262627f:     	pushq	%rdi
142626280:     	pushq	%rbx
142626281:     	subq	$0x68, %rsp
142626285:     	leaq	0x80(%rdx), %rbp
14262628c:     	movaps	%xmm7, 0x40(%rsp)
142626291:     	movaps	%xmm6, 0x50(%rsp)
142626296:     	leaq	0x1e8(%rbp), %rcx
14262629d:     	callq	0x140faa6e0 <.text+0xfa96e0>
1426262a2:     	movaps	0x50(%rsp), %xmm6
1426262a7:     	movaps	0x40(%rsp), %xmm7
1426262ac:     	addq	$0x68, %rsp
1426262b0:     	popq	%rbx
1426262b1:     	popq	%rdi
1426262b2:     	popq	%rsi
1426262b3:     	popq	%r12
1426262b5:     	popq	%r13
1426262b7:     	popq	%r14
1426262b9:     	popq	%r15
1426262bb:     	popq	%rbp
1426262bc:     	retq
1426262bd:     	nopl	(%rax)
1426262c0:     	movq	%rdx, 0x10(%rsp)
1426262c5:     	pushq	%rbp
1426262c6:     	pushq	%r15
1426262c8:     	pushq	%r14
1426262ca:     	pushq	%r13
1426262cc:     	pushq	%r12
1426262ce:     	pushq	%rsi
1426262cf:     	pushq	%rdi
1426262d0:     	pushq	%rbx
1426262d1:     	subq	$0x68, %rsp
1426262d5:     	leaq	0x80(%rdx), %rbp
1426262dc:     	movaps	%xmm7, 0x40(%rsp)
1426262e1:     	movaps	%xmm6, 0x50(%rsp)
1426262e6:     	movl	$0x40, %edx
1426262eb:     	movl	$0x8, %r8d
1426262f1:     	movq	0x238(%rbp), %rcx
1426262f8:     	callq	0x140613c20 <.text+0x612c20>
1426262fd:     	movaps	0x50(%rsp), %xmm6
142626302:     	movaps	0x40(%rsp), %xmm7
142626307:     	addq	$0x68, %rsp
14262630b:     	popq	%rbx
14262630c:     	popq	%rdi
14262630d:     	popq	%rsi
14262630e:     	popq	%r12
142626310:     	popq	%r13
142626312:     	popq	%r14
142626314:     	popq	%r15
142626316:     	popq	%rbp
142626317:     	retq
142626318:     	nopl	(%rax,%rax)
142626320:     	movq	%rdx, 0x10(%rsp)
142626325:     	pushq	%rbp
142626326:     	pushq	%r15
142626328:     	pushq	%r14
14262632a:     	pushq	%r13
14262632c:     	pushq	%r12
14262632e:     	pushq	%rsi
14262632f:     	pushq	%rdi
142626330:     	pushq	%rbx
142626331:     	subq	$0x68, %rsp
142626335:     	leaq	0x80(%rdx), %rbp
14262633c:     	movaps	%xmm7, 0x40(%rsp)
142626341:     	movaps	%xmm6, 0x50(%rsp)
142626346:     	leaq	0xd8(%rbp), %rcx
14262634d:     	callq	0x140fa19f0 <.text+0xfa09f0>
142626352:     	movaps	0x50(%rsp), %xmm6
142626357:     	movaps	0x40(%rsp), %xmm7
14262635c:     	addq	$0x68, %rsp
142626360:     	popq	%rbx
142626361:     	popq	%rdi
142626362:     	popq	%rsi
142626363:     	popq	%r12
142626365:     	popq	%r13
142626367:     	popq	%r14
142626369:     	popq	%r15
14262636b:     	popq	%rbp
14262636c:     	retq
14262636d:     	nopl	(%rax)
142626370:     	movq	%rdx, 0x10(%rsp)
142626375:     	pushq	%rbp
142626376:     	pushq	%r15
142626378:     	pushq	%r14
14262637a:     	pushq	%r13
14262637c:     	pushq	%r12
14262637e:     	pushq	%rsi
14262637f:     	pushq	%rdi
142626380:     	pushq	%rbx
142626381:     	subq	$0x68, %rsp
142626385:     	leaq	0x80(%rdx), %rbp
14262638c:     	movaps	%xmm7, 0x40(%rsp)
142626391:     	movaps	%xmm6, 0x50(%rsp)
142626396:     	leaq	0x1a0(%rbp), %rcx
14262639d:     	callq	0x140faa6e0 <.text+0xfa96e0>
1426263a2:     	movaps	0x50(%rsp), %xmm6
1426263a7:     	movaps	0x40(%rsp), %xmm7
1426263ac:     	addq	$0x68, %rsp
1426263b0:     	popq	%rbx
1426263b1:     	popq	%rdi
1426263b2:     	popq	%rsi
1426263b3:     	popq	%r12
1426263b5:     	popq	%r13
1426263b7:     	popq	%r14
1426263b9:     	popq	%r15
1426263bb:     	popq	%rbp
1426263bc:     	retq
1426263bd:     	nopl	(%rax)
1426263c0:     	movq	%rdx, 0x10(%rsp)
1426263c5:     	pushq	%rbp
1426263c6:     	pushq	%r15
1426263c8:     	pushq	%r14
1426263ca:     	pushq	%r13
1426263cc:     	pushq	%r12
1426263ce:     	pushq	%rsi
1426263cf:     	pushq	%rdi
1426263d0:     	pushq	%rbx
1426263d1:     	subq	$0x68, %rsp
1426263d5:     	leaq	0x80(%rdx), %rbp
1426263dc:     	movaps	%xmm7, 0x40(%rsp)
1426263e1:     	movaps	%xmm6, 0x50(%rsp)
1426263e6:     	leaq	0x1e8(%rbp), %rcx
1426263ed:     	callq	0x140faa6e0 <.text+0xfa96e0>
1426263f2:     	movaps	0x50(%rsp), %xmm6
1426263f7:     	movaps	0x40(%rsp), %xmm7
1426263fc:     	addq	$0x68, %rsp
142626400:     	popq	%rbx
142626401:     	popq	%rdi
142626402:     	popq	%rsi
142626403:     	popq	%r12
142626405:     	popq	%r13
142626407:     	popq	%r14
142626409:     	popq	%r15
14262640b:     	popq	%rbp
14262640c:     	retq
14262640d:     	nopl	(%rax)
142626410:     	movq	%rdx, 0x10(%rsp)
142626415:     	pushq	%rbp
142626416:     	pushq	%r15
142626418:     	pushq	%r14
14262641a:     	pushq	%r13
14262641c:     	pushq	%r12
14262641e:     	pushq	%rsi
14262641f:     	pushq	%rdi
142626420:     	pushq	%rbx
142626421:     	subq	$0x68, %rsp
142626425:     	leaq	0x80(%rdx), %rbp
14262642c:     	movaps	%xmm7, 0x40(%rsp)
142626431:     	movaps	%xmm6, 0x50(%rsp)
142626436:     	movl	$0x40, %edx
14262643b:     	movl	$0x8, %r8d
142626441:     	movq	0x238(%rbp), %rcx
142626448:     	callq	0x140613c20 <.text+0x612c20>
14262644d:     	movaps	0x50(%rsp), %xmm6
142626452:     	movaps	0x40(%rsp), %xmm7
142626457:     	addq	$0x68, %rsp
14262645b:     	popq	%rbx
14262645c:     	popq	%rdi
14262645d:     	popq	%rsi
14262645e:     	popq	%r12
142626460:     	popq	%r13
142626462:     	popq	%r14
142626464:     	popq	%r15
142626466:     	popq	%rbp
142626467:     	retq
142626468:     	nopl	(%rax,%rax)
142626470:     	movq	%rdx, 0x10(%rsp)
142626475:     	pushq	%rbp
142626476:     	pushq	%r15
142626478:     	pushq	%r14
14262647a:     	pushq	%r13
14262647c:     	pushq	%r12
14262647e:     	pushq	%rsi
14262647f:     	pushq	%rdi
142626480:     	pushq	%rbx
142626481:     	subq	$0x68, %rsp
142626485:     	leaq	0x80(%rdx), %rbp
14262648c:     	movaps	%xmm7, 0x40(%rsp)
142626491:     	movaps	%xmm6, 0x50(%rsp)
142626496:     	leaq	0xa8(%rbp), %rcx
14262649d:     	callq	0x140fa19f0 <.text+0xfa09f0>
1426264a2:     	movaps	0x50(%rsp), %xmm6
1426264a7:     	movaps	0x40(%rsp), %xmm7
1426264ac:     	addq	$0x68, %rsp
1426264b0:     	popq	%rbx
1426264b1:     	popq	%rdi
1426264b2:     	popq	%rsi
1426264b3:     	popq	%r12
1426264b5:     	popq	%r13
1426264b7:     	popq	%r14
1426264b9:     	popq	%r15
1426264bb:     	popq	%rbp
1426264bc:     	retq
1426264bd:     	nopl	(%rax)
1426264c0:     	movq	%rdx, 0x10(%rsp)
1426264c5:     	pushq	%rbp
1426264c6:     	pushq	%r15
1426264c8:     	pushq	%r14
1426264ca:     	pushq	%r13
1426264cc:     	pushq	%r12
1426264ce:     	pushq	%rsi
1426264cf:     	pushq	%rdi
1426264d0:     	pushq	%rbx
1426264d1:     	subq	$0x68, %rsp
1426264d5:     	leaq	0x80(%rdx), %rbp
1426264dc:     	movaps	%xmm7, 0x40(%rsp)
1426264e1:     	movaps	%xmm6, 0x50(%rsp)
1426264e6:     	movl	$0x40, %edx
1426264eb:     	movl	$0x8, %r8d
1426264f1:     	movq	0x238(%rbp), %rcx
1426264f8:     	callq	0x140613c20 <.text+0x612c20>
1426264fd:     	movaps	0x50(%rsp), %xmm6
142626502:     	movaps	0x40(%rsp), %xmm7
142626507:     	addq	$0x68, %rsp
14262650b:     	popq	%rbx
14262650c:     	popq	%rdi
14262650d:     	popq	%rsi
14262650e:     	popq	%r12
142626510:     	popq	%r13
142626512:     	popq	%r14
142626514:     	popq	%r15
142626516:     	popq	%rbp
142626517:     	retq
142626518:     	nopl	(%rax,%rax)
142626520:     	movq	%rdx, 0x10(%rsp)
142626525:     	pushq	%rbp
142626526:     	pushq	%r15
142626528:     	pushq	%r14
14262652a:     	pushq	%r13
14262652c:     	pushq	%r12
14262652e:     	pushq	%rsi
14262652f:     	pushq	%rdi
142626530:     	pushq	%rbx
142626531:     	subq	$0x68, %rsp
142626535:     	leaq	0x80(%rdx), %rbp
14262653c:     	movaps	%xmm7, 0x40(%rsp)
142626541:     	movaps	%xmm6, 0x50(%rsp)
142626546:     	movl	$0xc0, %edx
14262654b:     	movl	$0x8, %r8d
142626551:     	movq	0x228(%rbp), %rcx
142626558:     	callq	0x140613c20 <.text+0x612c20>
14262655d:     	movaps	0x50(%rsp), %xmm6
142626562:     	movaps	0x40(%rsp), %xmm7
142626567:     	addq	$0x68, %rsp
14262656b:     	popq	%rbx
14262656c:     	popq	%rdi
14262656d:     	popq	%rsi
14262656e:     	popq	%r12
142626570:     	popq	%r13
142626572:     	popq	%r14
142626574:     	popq	%r15
142626576:     	popq	%rbp
142626577:     	retq
142626578:     	nopl	(%rax,%rax)
142626580:     	movq	%rdx, 0x10(%rsp)
142626585:     	pushq	%rbp
142626586:     	pushq	%r15
142626588:     	pushq	%r14
14262658a:     	pushq	%r13
14262658c:     	pushq	%r12
14262658e:     	pushq	%rsi
14262658f:     	pushq	%rdi
142626590:     	pushq	%rbx
142626591:     	subq	$0x68, %rsp
142626595:     	leaq	0x80(%rdx), %rbp
14262659c:     	movaps	%xmm7, 0x40(%rsp)
1426265a1:     	movaps	%xmm6, 0x50(%rsp)
1426265a6:     	leaq	0x48(%rbp), %rcx
1426265aa:     	callq	0x140fa19f0 <.text+0xfa09f0>
1426265af:     	leaq	0x78(%rbp), %rcx
1426265b3:     	callq	0x140fa19f0 <.text+0xfa09f0>
1426265b8:     	movaps	0x50(%rsp), %xmm6
1426265bd:     	movaps	0x40(%rsp), %xmm7
1426265c2:     	addq	$0x68, %rsp
1426265c6:     	popq	%rbx
1426265c7:     	popq	%rdi
1426265c8:     	popq	%rsi
1426265c9:     	popq	%r12
1426265cb:     	popq	%r13
1426265cd:     	popq	%r14
1426265cf:     	popq	%r15
1426265d1:     	popq	%rbp
1426265d2:     	retq
1426265d3:     	nopw	%cs:(%rax,%rax)
1426265e0:     	movq	%rdx, 0x10(%rsp)
1426265e5:     	pushq	%rbp
1426265e6:     	pushq	%r15
1426265e8:     	pushq	%r14
1426265ea:     	pushq	%r13
1426265ec:     	pushq	%r12
1426265ee:     	pushq	%rsi
1426265ef:     	pushq	%rdi
1426265f0:     	pushq	%rbx
1426265f1:     	subq	$0x68, %rsp
1426265f5:     	leaq	0x80(%rdx), %rbp
1426265fc:     	movaps	%xmm7, 0x40(%rsp)
142626601:     	movaps	%xmm6, 0x50(%rsp)
142626606:     	leaq	0x160(%rbp), %rcx
14262660d:     	callq	0x140faa6e0 <.text+0xfa96e0>
142626612:     	movaps	0x50(%rsp), %xmm6
142626617:     	movaps	0x40(%rsp), %xmm7
14262661c:     	addq	$0x68, %rsp
142626620:     	popq	%rbx
142626621:     	popq	%rdi
142626622:     	popq	%rsi
142626623:     	popq	%r12
142626625:     	popq	%r13
142626627:     	popq	%r14
142626629:     	popq	%r15
14262662b:     	popq	%rbp
14262662c:     	retq
14262662d:     	nopl	(%rax)
142626630:     	movq	%rdx, 0x10(%rsp)
142626635:     	pushq	%rbp
142626636:     	pushq	%r15
142626638:     	pushq	%r14
14262663a:     	pushq	%r13
14262663c:     	pushq	%r12
14262663e:     	pushq	%rsi
14262663f:     	pushq	%rdi
142626640:     	pushq	%rbx
142626641:     	subq	$0x68, %rsp
142626645:     	leaq	0x80(%rdx), %rbp
14262664c:     	movaps	%xmm7, 0x40(%rsp)
142626651:     	movaps	%xmm6, 0x50(%rsp)
142626656:     	leaq	0x1a0(%rbp), %rcx
14262665d:     	callq	0x140faa6e0 <.text+0xfa96e0>
142626662:     	movaps	0x50(%rsp), %xmm6
142626667:     	movaps	0x40(%rsp), %xmm7
14262666c:     	addq	$0x68, %rsp
142626670:     	popq	%rbx
142626671:     	popq	%rdi
142626672:     	popq	%rsi
142626673:     	popq	%r12
142626675:     	popq	%r13
142626677:     	popq	%r14
142626679:     	popq	%r15
14262667b:     	popq	%rbp
14262667c:     	retq
14262667d:     	nopl	(%rax)
142626680:     	movq	%rdx, 0x10(%rsp)
142626685:     	pushq	%rbp
142626686:     	pushq	%r15
142626688:     	pushq	%r14
14262668a:     	pushq	%r13
14262668c:     	pushq	%r12
14262668e:     	pushq	%rsi
14262668f:     	pushq	%rdi
142626690:     	pushq	%rbx
142626691:     	subq	$0x68, %rsp
142626695:     	leaq	0x80(%rdx), %rbp
14262669c:     	movaps	%xmm7, 0x40(%rsp)
1426266a1:     	movaps	%xmm6, 0x50(%rsp)
1426266a6:     	leaq	0x1e8(%rbp), %rcx
1426266ad:     	callq	0x140faa6e0 <.text+0xfa96e0>
1426266b2:     	movaps	0x50(%rsp), %xmm6
1426266b7:     	movaps	0x40(%rsp), %xmm7
1426266bc:     	addq	$0x68, %rsp
1426266c0:     	popq	%rbx
1426266c1:     	popq	%rdi
1426266c2:     	popq	%rsi
1426266c3:     	popq	%r12
1426266c5:     	popq	%r13
1426266c7:     	popq	%r14
1426266c9:     	popq	%r15
1426266cb:     	popq	%rbp
1426266cc:     	retq
1426266cd:     	nopl	(%rax)
1426266d0:     	movq	%rdx, 0x10(%rsp)
1426266d5:     	pushq	%rbp
1426266d6:     	pushq	%r15
1426266d8:     	pushq	%r14
1426266da:     	pushq	%r13
1426266dc:     	pushq	%r12
1426266de:     	pushq	%rsi
1426266df:     	pushq	%rdi
1426266e0:     	pushq	%rbx
1426266e1:     	subq	$0x68, %rsp
1426266e5:     	leaq	0x80(%rdx), %rbp
1426266ec:     	movaps	%xmm7, 0x40(%rsp)
1426266f1:     	movaps	%xmm6, 0x50(%rsp)
1426266f6:     	movl	$0x40, %edx
1426266fb:     	movl	$0x8, %r8d
142626701:     	movq	0x238(%rbp), %rcx
142626708:     	callq	0x140613c20 <.text+0x612c20>
14262670d:     	movaps	0x50(%rsp), %xmm6
142626712:     	movaps	0x40(%rsp), %xmm7
142626717:     	addq	$0x68, %rsp
14262671b:     	popq	%rbx
14262671c:     	popq	%rdi
14262671d:     	popq	%rsi
14262671e:     	popq	%r12
142626720:     	popq	%r13
142626722:     	popq	%r14
142626724:     	popq	%r15
142626726:     	popq	%rbp
142626727:     	retq
142626728:     	nopl	(%rax,%rax)
142626730:     	movq	%rdx, 0x10(%rsp)
142626735:     	pushq	%rbp
142626736:     	pushq	%r15
142626738:     	pushq	%r14
14262673a:     	pushq	%r13
14262673c:     	pushq	%r12
14262673e:     	pushq	%rsi
14262673f:     	pushq	%rdi
142626740:     	pushq	%rbx
142626741:     	subq	$0x68, %rsp
142626745:     	leaq	0x80(%rdx), %rbp
14262674c:     	movaps	%xmm7, 0x40(%rsp)
142626751:     	movaps	%xmm6, 0x50(%rsp)
142626756:     	leaq	0x178(%rbp), %rcx
14262675d:     	callq	0x140fa19f0 <.text+0xfa09f0>
142626762:     	leaq	0x18(%rbp), %rcx
142626766:     	callq	0x140fa19f0 <.text+0xfa09f0>
14262676b:     	movaps	0x50(%rsp), %xmm6
142626770:     	movaps	0x40(%rsp), %xmm7
142626775:     	addq	$0x68, %rsp
142626779:     	popq	%rbx
14262677a:     	popq	%rdi
14262677b:     	popq	%rsi
14262677c:     	popq	%r12
14262677e:     	popq	%r13
142626780:     	popq	%r14
142626782:     	popq	%r15
142626784:     	popq	%rbp
142626785:     	retq
142626786:     	nopw	%cs:(%rax,%rax)
142626790:     	movq	%rdx, 0x10(%rsp)
142626795:     	pushq	%rbp
142626796:     	pushq	%r15
142626798:     	pushq	%r14
14262679a:     	pushq	%r13
14262679c:     	pushq	%r12
14262679e:     	pushq	%rsi
14262679f:     	pushq	%rdi
1426267a0:     	pushq	%rbx
1426267a1:     	subq	$0x68, %rsp
1426267a5:     	leaq	0x80(%rdx), %rbp
1426267ac:     	movaps	%xmm7, 0x40(%rsp)
1426267b1:     	movaps	%xmm6, 0x50(%rsp)
1426267b6:     	movl	$0x40, %edx
1426267bb:     	movl	$0x8, %r8d
1426267c1:     	movq	0x238(%rbp), %rcx
1426267c8:     	callq	0x140613c20 <.text+0x612c20>
1426267cd:     	movaps	0x50(%rsp), %xmm6
1426267d2:     	movaps	0x40(%rsp), %xmm7
1426267d7:     	addq	$0x68, %rsp
1426267db:     	popq	%rbx
1426267dc:     	popq	%rdi
1426267dd:     	popq	%rsi
1426267de:     	popq	%r12
1426267e0:     	popq	%r13
1426267e2:     	popq	%r14
1426267e4:     	popq	%r15
1426267e6:     	popq	%rbp
1426267e7:     	retq
1426267e8:     	nopl	(%rax,%rax)
1426267f0:     	movq	%rdx, 0x10(%rsp)
1426267f5:     	pushq	%rbp
1426267f6:     	pushq	%r15
1426267f8:     	pushq	%r14
1426267fa:     	pushq	%r13
1426267fc:     	pushq	%r12
1426267fe:     	pushq	%rsi
1426267ff:     	pushq	%rdi
142626800:     	pushq	%rbx
142626801:     	subq	$0x68, %rsp
142626805:     	leaq	0x80(%rdx), %rbp
14262680c:     	movaps	%xmm7, 0x40(%rsp)
142626811:     	movaps	%xmm6, 0x50(%rsp)
142626816:     	movl	$0xc0, %edx
14262681b:     	movl	$0x8, %r8d
142626821:     	movq	0x228(%rbp), %rcx
142626828:     	callq	0x140613c20 <.text+0x612c20>
14262682d:     	movaps	0x50(%rsp), %xmm6
142626832:     	movaps	0x40(%rsp), %xmm7
142626837:     	addq	$0x68, %rsp
14262683b:     	popq	%rbx
14262683c:     	popq	%rdi
14262683d:     	popq	%rsi
14262683e:     	popq	%r12
142626840:     	popq	%r13
142626842:     	popq	%r14
142626844:     	popq	%r15
142626846:     	popq	%rbp
142626847:     	retq
142626848:     	nopl	(%rax,%rax)
142626850:     	movq	%rdx, 0x10(%rsp)
142626855:     	pushq	%rbp
142626856:     	pushq	%r15
142626858:     	pushq	%r14
14262685a:     	pushq	%r13
14262685c:     	pushq	%r12
14262685e:     	pushq	%rsi
14262685f:     	pushq	%rdi
142626860:     	pushq	%rbx
142626861:     	subq	$0x68, %rsp
142626865:     	leaq	0x80(%rdx), %rbp
14262686c:     	movaps	%xmm7, 0x40(%rsp)
142626871:     	movaps	%xmm6, 0x50(%rsp)
142626876:     	leaq	0x1a0(%rbp), %rcx
14262687d:     	callq	0x140faa6e0 <.text+0xfa96e0>
142626882:     	movaps	0x50(%rsp), %xmm6
142626887:     	movaps	0x40(%rsp), %xmm7
14262688c:     	addq	$0x68, %rsp
142626890:     	popq	%rbx
142626891:     	popq	%rdi
142626892:     	popq	%rsi
142626893:     	popq	%r12
142626895:     	popq	%r13
142626897:     	popq	%r14
142626899:     	popq	%r15
14262689b:     	popq	%rbp
14262689c:     	retq
14262689d:     	nopl	(%rax)
1426268a0:     	movq	%rdx, 0x10(%rsp)
1426268a5:     	pushq	%rbp
1426268a6:     	pushq	%r15
1426268a8:     	pushq	%r14
1426268aa:     	pushq	%r13
1426268ac:     	pushq	%r12
1426268ae:     	pushq	%rsi
1426268af:     	pushq	%rdi
1426268b0:     	pushq	%rbx
1426268b1:     	subq	$0x68, %rsp
1426268b5:     	leaq	0x80(%rdx), %rbp
1426268bc:     	movaps	%xmm7, 0x40(%rsp)
1426268c1:     	movaps	%xmm6, 0x50(%rsp)
1426268c6:     	leaq	0x1e8(%rbp), %rcx
1426268cd:     	callq	0x140faa6e0 <.text+0xfa96e0>
1426268d2:     	movaps	0x50(%rsp), %xmm6
1426268d7:     	movaps	0x40(%rsp), %xmm7
1426268dc:     	addq	$0x68, %rsp
1426268e0:     	popq	%rbx
1426268e1:     	popq	%rdi
1426268e2:     	popq	%rsi
1426268e3:     	popq	%r12
1426268e5:     	popq	%r13
1426268e7:     	popq	%r14
1426268e9:     	popq	%r15
1426268eb:     	popq	%rbp
1426268ec:     	retq
1426268ed:     	nopl	(%rax)
1426268f0:     	movq	%rdx, 0x10(%rsp)
1426268f5:     	pushq	%rbp
1426268f6:     	pushq	%r15
1426268f8:     	pushq	%r14
1426268fa:     	pushq	%r13
1426268fc:     	pushq	%r12
1426268fe:     	pushq	%rsi
1426268ff:     	pushq	%rdi
142626900:     	pushq	%rbx
142626901:     	subq	$0x68, %rsp
142626905:     	leaq	0x80(%rdx), %rbp
14262690c:     	movaps	%xmm7, 0x40(%rsp)
142626911:     	movaps	%xmm6, 0x50(%rsp)
142626916:     	cmpb	$0x0, 0x237(%rbp)
14262691d:     	je	0x14262694d <.text+0x262594d>
14262691f:     	leaq	0x1a0(%rbp), %rcx
142626926:     	callq	0x140fa19f0 <.text+0xfa09f0>
14262692b:     	movb	$0x0, 0x247(%rbp)
142626932:     	movaps	0x50(%rsp), %xmm6
142626937:     	movaps	0x40(%rsp), %xmm7
14262693c:     	addq	$0x68, %rsp
142626940:     	popq	%rbx
142626941:     	popq	%rdi
142626942:     	popq	%rsi
142626943:     	popq	%r12
142626945:     	popq	%r13
142626947:     	popq	%r14
142626949:     	popq	%r15
14262694b:     	popq	%rbp
14262694c:     	retq
14262694d:     	movb	$0x0, 0x247(%rbp)
142626954:     	movaps	0x50(%rsp), %xmm6
142626959:     	movaps	0x40(%rsp), %xmm7
14262695e:     	addq	$0x68, %rsp
142626962:     	popq	%rbx
142626963:     	popq	%rdi
142626964:     	popq	%rsi
142626965:     	popq	%r12
142626967:     	popq	%r13
142626969:     	popq	%r14
14262696b:     	popq	%r15
14262696d:     	popq	%rbp
14262696e:     	retq
14262696f:     	nop
142626970:     	movq	%rdx, 0x10(%rsp)
142626975:     	pushq	%rbp
142626976:     	pushq	%r15
142626978:     	pushq	%r14
14262697a:     	pushq	%r13
14262697c:     	pushq	%r12
14262697e:     	pushq	%rsi
14262697f:     	pushq	%rdi
142626980:     	pushq	%rbx
142626981:     	subq	$0x68, %rsp
142626985:     	leaq	0x80(%rdx), %rbp
14262698c:     	movaps	%xmm7, 0x40(%rsp)
142626991:     	movaps	%xmm6, 0x50(%rsp)
142626996:     	cmpb	$0x0, 0x247(%rbp)
14262699d:     	je	0x1426269c6 <.text+0x26259c6>
14262699f:     	leaq	0x160(%rbp), %rcx
1426269a6:     	callq	0x140fa19f0 <.text+0xfa09f0>
1426269ab:     	movaps	0x50(%rsp), %xmm6
1426269b0:     	movaps	0x40(%rsp), %xmm7
1426269b5:     	addq	$0x68, %rsp
1426269b9:     	popq	%rbx
1426269ba:     	popq	%rdi
1426269bb:     	popq	%rsi
1426269bc:     	popq	%r12
1426269be:     	popq	%r13
1426269c0:     	popq	%r14
1426269c2:     	popq	%r15
1426269c4:     	popq	%rbp
1426269c5:     	retq
1426269c6:     	movaps	0x50(%rsp), %xmm6
1426269cb:     	movaps	0x40(%rsp), %xmm7
1426269d0:     	addq	$0x68, %rsp
1426269d4:     	popq	%rbx
1426269d5:     	popq	%rdi
1426269d6:     	popq	%rsi
1426269d7:     	popq	%r12
1426269d9:     	popq	%r13
1426269db:     	popq	%r14
1426269dd:     	popq	%r15
1426269df:     	popq	%rbp
1426269e0:     	retq
1426269e1:     	nopw	%cs:(%rax,%rax)
1426269f0:     	movq	%rdx, 0x10(%rsp)
1426269f5:     	pushq	%rbp
1426269f6:     	pushq	%r15
1426269f8:     	pushq	%r14
1426269fa:     	pushq	%r13
1426269fc:     	pushq	%r12
1426269fe:     	pushq	%rsi
1426269ff:     	pushq	%rdi
142626a00:     	pushq	%rbx
142626a01:     	subq	$0x68, %rsp
142626a05:     	leaq	0x80(%rdx), %rbp
142626a0c:     	movaps	%xmm7, 0x40(%rsp)
142626a11:     	movaps	%xmm6, 0x50(%rsp)
142626a16:     	leaq	0x1e8(%rbp), %rcx
142626a1d:     	callq	0x140fa3670 <.text+0xfa2670>
142626a22:     	movaps	0x50(%rsp), %xmm6
142626a27:     	movaps	0x40(%rsp), %xmm7
142626a2c:     	addq	$0x68, %rsp
142626a30:     	popq	%rbx
142626a31:     	popq	%rdi
142626a32:     	popq	%rsi
142626a33:     	popq	%r12
142626a35:     	popq	%r13
142626a37:     	popq	%r14
142626a39:     	popq	%r15
142626a3b:     	popq	%rbp
142626a3c:     	retq
