
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140ae3740:     	pushq	%rbp
140ae3741:     	pushq	%r14
140ae3743:     	pushq	%rsi
140ae3744:     	pushq	%rdi
140ae3745:     	pushq	%rbx
140ae3746:     	movl	$0x1dd0, %eax           # imm = 0x1DD0
140ae374b:     	callq	0x14289aa40 <.text+0x2899a40>
140ae3750:     	subq	%rax, %rsp
140ae3753:     	leaq	0x80(%rsp), %rbp
140ae375b:     	movq	$-0x2, 0x1d48(%rbp)
140ae3766:     	movl	%edx, %ebx
140ae3768:     	movq	%rcx, %rsi
140ae376b:     	movzbl	0x2b51497(%rip), %eax   # 0x143634c09
140ae3772:     	movl	$0x460, %ecx            # imm = 0x460
140ae3777:     	movl	$0x8, %edx
140ae377c:     	callq	0x140613c10 <.text+0x612c10>
140ae3781:     	testq	%rax, %rax
140ae3784:     	je	0x140ae42a4 <.text+0xae32a4>
140ae378a:     	movq	%rax, 0x1d40(%rbp)
140ae3791:     	xorps	%xmm0, %xmm0
140ae3794:     	movups	%xmm0, -0x38(%rbp)
140ae3798:     	movups	%xmm0, -0x20(%rbp)
140ae379c:     	movups	%xmm0, -0x8(%rbp)
140ae37a0:     	movups	%xmm0, 0x10(%rbp)
140ae37a4:     	movups	%xmm0, 0x28(%rbp)
140ae37a8:     	movl	$0x0, 0x38(%rbp)
140ae37af:     	movups	%xmm0, 0x60(%rbp)
140ae37b3:     	movups	%xmm0, 0x78(%rbp)
140ae37b7:     	movups	%xmm0, 0x90(%rbp)
140ae37be:     	movups	%xmm0, 0xa8(%rbp)
140ae37c5:     	movups	%xmm0, 0xc0(%rbp)
140ae37cc:     	movl	$0x0, 0xd0(%rbp)
140ae37d6:     	movq	$0x0, -0x58(%rbp)
140ae37de:     	movq	$0x8, -0x50(%rbp)
140ae37e6:     	movq	$0x0, -0x48(%rbp)
140ae37ee:     	movq	$0x10, -0x40(%rbp)
140ae37f6:     	movq	$0x10, -0x28(%rbp)
140ae37fe:     	movq	$0x10, -0x10(%rbp)
140ae3806:     	movq	$0x10, 0x8(%rbp)
140ae380e:     	movq	$0x10, 0x20(%rbp)
140ae3816:     	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
140ae3820:     	movq	%r14, 0x40(%rbp)
140ae3824:     	leaq	0x1fff566(%rip), %rax   # 0x142ae2d91
140ae382b:     	movq	%rax, 0x48(%rbp)
140ae382f:     	movq	$0x75, 0x50(%rbp)
140ae3837:     	movq	$0x10, 0x58(%rbp)
140ae383f:     	movq	$0x10, 0x70(%rbp)
140ae3847:     	movq	$0x10, 0x88(%rbp)
140ae3852:     	movq	$0x10, 0xa0(%rbp)
140ae385d:     	movq	$0x10, 0xb8(%rbp)
140ae3868:     	movl	$0x0, 0xd8(%rbp)
140ae3872:     	movw	$0x1, 0xdc(%rbp)
140ae387b:     	movq	%r14, 0x1910(%rbp)
140ae3882:     	movl	$0x0, 0x1948(%rbp)
140ae388c:     	movzbl	0x2b51376(%rip), %eax   # 0x143634c09
140ae3893:     	movl	$0x19a8, %ecx           # imm = 0x19A8
140ae3898:     	movl	$0x8, %edx
140ae389d:     	callq	0x140613c10 <.text+0x612c10>
140ae38a2:     	testq	%rax, %rax
140ae38a5:     	je	0x140ae42b3 <.text+0xae32b3>
140ae38ab:     	movq	%rax, %rdi
140ae38ae:     	leaq	-0x58(%rbp), %rdx
140ae38b2:     	movl	$0x19a8, %r8d           # imm = 0x19A8
140ae38b8:     	movq	%rax, %rcx
140ae38bb:     	callq	0x142923c00 <.text+0x2922c00>
140ae38c0:     	leaq	0x1ffefd1(%rip), %r8    # 0x142ae2898
140ae38c7:     	leaq	0x1c60(%rbp), %rcx
140ae38ce:     	movq	%rdi, %rdx
140ae38d1:     	callq	0x140ce4b10 <.text+0xce3b10>
140ae38d6:     	xorps	%xmm0, %xmm0
140ae38d9:     	movups	%xmm0, -0x38(%rbp)
140ae38dd:     	movups	%xmm0, -0x20(%rbp)
140ae38e1:     	movups	%xmm0, -0x8(%rbp)
140ae38e5:     	movups	%xmm0, 0x10(%rbp)
140ae38e9:     	movups	%xmm0, 0x28(%rbp)
140ae38ed:     	movl	$0x0, 0x38(%rbp)
140ae38f4:     	movups	%xmm0, 0x60(%rbp)
140ae38f8:     	movups	%xmm0, 0x78(%rbp)
140ae38fc:     	movups	%xmm0, 0x90(%rbp)
140ae3903:     	movups	%xmm0, 0xa8(%rbp)
140ae390a:     	movups	%xmm0, 0xc0(%rbp)
140ae3911:     	movl	$0x0, 0xd0(%rbp)
140ae391b:     	movq	$0x0, -0x58(%rbp)
140ae3923:     	movq	$0x8, -0x50(%rbp)
140ae392b:     	movq	$0x0, -0x48(%rbp)
140ae3933:     	movq	$0x10, -0x40(%rbp)
140ae393b:     	movq	$0x10, -0x28(%rbp)
140ae3943:     	movq	$0x10, -0x10(%rbp)
140ae394b:     	movq	$0x10, 0x8(%rbp)
140ae3953:     	movq	$0x10, 0x20(%rbp)
140ae395b:     	movq	%r14, 0x40(%rbp)
140ae395f:     	leaq	0x1fff331(%rip), %rax   # 0x142ae2c97
140ae3966:     	movq	%rax, 0x48(%rbp)
140ae396a:     	movq	$0x7d, 0x50(%rbp)
140ae3972:     	movq	$0x10, 0x58(%rbp)
140ae397a:     	movq	$0x10, 0x70(%rbp)
140ae3982:     	movq	$0x10, 0x88(%rbp)
140ae398d:     	movq	$0x10, 0xa0(%rbp)
140ae3998:     	movq	$0x10, 0xb8(%rbp)
140ae39a3:     	movl	$0x0, 0xd8(%rbp)
140ae39ad:     	movw	$0x1, 0xdc(%rbp)
140ae39b6:     	movq	%r14, 0x1910(%rbp)
140ae39bd:     	movl	$0x0, 0x1948(%rbp)
140ae39c7:     	movzbl	0x2b5123b(%rip), %eax   # 0x143634c09
140ae39ce:     	movl	$0x19a8, %ecx           # imm = 0x19A8
140ae39d3:     	movl	$0x8, %edx
140ae39d8:     	callq	0x140613c10 <.text+0x612c10>
140ae39dd:     	testq	%rax, %rax
140ae39e0:     	je	0x140ae42c4 <.text+0xae32c4>
140ae39e6:     	movq	%rax, %rdi
140ae39e9:     	leaq	-0x58(%rbp), %rdx
140ae39ed:     	movl	$0x19a8, %r8d           # imm = 0x19A8
140ae39f3:     	movq	%rax, %rcx
140ae39f6:     	callq	0x142923c00 <.text+0x2922c00>
140ae39fb:     	leaq	0x1ffed26(%rip), %r8    # 0x142ae2728
140ae3a02:     	leaq	0x1bf0(%rbp), %rcx
140ae3a09:     	movq	%rdi, %rdx
140ae3a0c:     	callq	0x140ce4b10 <.text+0xce3b10>
140ae3a11:     	xorps	%xmm0, %xmm0
140ae3a14:     	movups	%xmm0, -0x38(%rbp)
140ae3a18:     	movups	%xmm0, -0x20(%rbp)
140ae3a1c:     	movups	%xmm0, -0x8(%rbp)
140ae3a20:     	movups	%xmm0, 0x10(%rbp)
140ae3a24:     	movups	%xmm0, 0x28(%rbp)
140ae3a28:     	movl	$0x0, 0x38(%rbp)
140ae3a2f:     	movups	%xmm0, 0x60(%rbp)
140ae3a33:     	movups	%xmm0, 0x78(%rbp)
140ae3a37:     	movups	%xmm0, 0x90(%rbp)
140ae3a3e:     	movups	%xmm0, 0xa8(%rbp)
140ae3a45:     	movups	%xmm0, 0xc0(%rbp)
140ae3a4c:     	movl	$0x0, 0xd0(%rbp)
140ae3a56:     	movq	$0x0, -0x58(%rbp)
140ae3a5e:     	movq	$0x8, -0x50(%rbp)
140ae3a66:     	movq	$0x0, -0x48(%rbp)
140ae3a6e:     	movq	$0x10, -0x40(%rbp)
140ae3a76:     	movq	$0x10, -0x28(%rbp)
140ae3a7e:     	movq	$0x10, -0x10(%rbp)
140ae3a86:     	movq	$0x10, 0x8(%rbp)
140ae3a8e:     	movq	$0x10, 0x20(%rbp)
140ae3a96:     	movq	%r14, 0x40(%rbp)
140ae3a9a:     	leaq	0x1fff179(%rip), %rax   # 0x142ae2c1a
140ae3aa1:     	movq	%rax, 0x48(%rbp)
140ae3aa5:     	movq	$0x7d, 0x50(%rbp)
140ae3aad:     	movq	$0x10, 0x58(%rbp)
140ae3ab5:     	movq	$0x10, 0x70(%rbp)
140ae3abd:     	movq	$0x10, 0x88(%rbp)
140ae3ac8:     	movq	$0x10, 0xa0(%rbp)
140ae3ad3:     	movq	$0x10, 0xb8(%rbp)
140ae3ade:     	movl	$0x0, 0xd8(%rbp)
140ae3ae8:     	movw	$0x1, 0xdc(%rbp)
140ae3af1:     	movq	%r14, 0x1910(%rbp)
140ae3af8:     	movl	$0x0, 0x1948(%rbp)
140ae3b02:     	movzbl	0x2b51100(%rip), %eax   # 0x143634c09
140ae3b09:     	movl	$0x19a8, %ecx           # imm = 0x19A8
140ae3b0e:     	movl	$0x8, %edx
140ae3b13:     	callq	0x140613c10 <.text+0x612c10>
140ae3b18:     	testq	%rax, %rax
140ae3b1b:     	je	0x140ae42d5 <.text+0xae32d5>
140ae3b21:     	movq	%rax, %rdi
140ae3b24:     	leaq	-0x58(%rbp), %rdx
140ae3b28:     	movl	$0x19a8, %r8d           # imm = 0x19A8
140ae3b2e:     	movq	%rax, %rcx
140ae3b31:     	callq	0x142923c00 <.text+0x2922c00>
140ae3b36:     	leaq	0x1ffef83(%rip), %r8    # 0x142ae2ac0
140ae3b3d:     	leaq	0x1b80(%rbp), %rcx
140ae3b44:     	movq	%rdi, %rdx
140ae3b47:     	callq	0x140ce4b10 <.text+0xce3b10>
140ae3b4c:     	xorps	%xmm0, %xmm0
140ae3b4f:     	movups	%xmm0, -0x38(%rbp)
140ae3b53:     	movups	%xmm0, -0x20(%rbp)
140ae3b57:     	movups	%xmm0, -0x8(%rbp)
140ae3b5b:     	movups	%xmm0, 0x10(%rbp)
140ae3b5f:     	movups	%xmm0, 0x28(%rbp)
140ae3b63:     	movl	$0x0, 0x38(%rbp)
140ae3b6a:     	movups	%xmm0, 0x60(%rbp)
140ae3b6e:     	movups	%xmm0, 0x78(%rbp)
140ae3b72:     	movups	%xmm0, 0x90(%rbp)
140ae3b79:     	movups	%xmm0, 0xa8(%rbp)
140ae3b80:     	movups	%xmm0, 0xc0(%rbp)
140ae3b87:     	movl	$0x0, 0xd0(%rbp)
140ae3b91:     	movq	$0x0, -0x58(%rbp)
140ae3b99:     	movq	$0x8, -0x50(%rbp)
140ae3ba1:     	movq	$0x0, -0x48(%rbp)
140ae3ba9:     	movq	$0x10, -0x40(%rbp)
140ae3bb1:     	movq	$0x10, -0x28(%rbp)
140ae3bb9:     	movq	$0x10, -0x10(%rbp)
140ae3bc1:     	movq	$0x10, 0x8(%rbp)
140ae3bc9:     	movq	$0x10, 0x20(%rbp)
140ae3bd1:     	movq	%r14, 0x40(%rbp)
140ae3bd5:     	leaq	0x1fff138(%rip), %rax   # 0x142ae2d14
140ae3bdc:     	movq	%rax, 0x48(%rbp)
140ae3be0:     	movq	$0x7d, 0x50(%rbp)
140ae3be8:     	movq	$0x10, 0x58(%rbp)
140ae3bf0:     	movq	$0x10, 0x70(%rbp)
140ae3bf8:     	movq	$0x10, 0x88(%rbp)
140ae3c03:     	movq	$0x10, 0xa0(%rbp)
140ae3c0e:     	movq	$0x10, 0xb8(%rbp)
140ae3c19:     	movl	$0x0, 0xd8(%rbp)
140ae3c23:     	movw	$0x1, 0xdc(%rbp)
140ae3c2c:     	movq	%r14, 0x1910(%rbp)
140ae3c33:     	movl	$0x0, 0x1948(%rbp)
140ae3c3d:     	movzbl	0x2b50fc5(%rip), %eax   # 0x143634c09
140ae3c44:     	movl	$0x19a8, %ecx           # imm = 0x19A8
140ae3c49:     	movl	$0x8, %edx
140ae3c4e:     	callq	0x140613c10 <.text+0x612c10>
140ae3c53:     	testq	%rax, %rax
140ae3c56:     	je	0x140ae42e6 <.text+0xae32e6>
140ae3c5c:     	movq	%rax, %rdi
140ae3c5f:     	leaq	-0x58(%rbp), %rdx
140ae3c63:     	movl	$0x19a8, %r8d           # imm = 0x19A8
140ae3c69:     	movq	%rax, %rcx
140ae3c6c:     	callq	0x142923c00 <.text+0x2922c00>
140ae3c71:     	leaq	0x1ffeb68(%rip), %r8    # 0x142ae27e0
140ae3c78:     	leaq	0x1b10(%rbp), %rcx
140ae3c7f:     	movq	%rdi, %rdx
140ae3c82:     	callq	0x140ce4b10 <.text+0xce3b10>
140ae3c87:     	xorps	%xmm0, %xmm0
140ae3c8a:     	movups	%xmm0, 0x1818(%rbp)
140ae3c91:     	movups	%xmm0, 0x1830(%rbp)
140ae3c98:     	movups	%xmm0, 0x1848(%rbp)
140ae3c9f:     	movups	%xmm0, 0x1860(%rbp)
140ae3ca6:     	movups	%xmm0, 0x1878(%rbp)
140ae3cad:     	movl	$0x0, 0x1888(%rbp)
140ae3cb7:     	movups	%xmm0, 0x18b0(%rbp)
140ae3cbe:     	movups	%xmm0, 0x18c8(%rbp)
140ae3cc5:     	movups	%xmm0, 0x18e0(%rbp)
140ae3ccc:     	movups	%xmm0, 0x18f8(%rbp)
140ae3cd3:     	movups	%xmm0, 0x1910(%rbp)
140ae3cda:     	movl	$0x0, 0x1920(%rbp)
140ae3ce4:     	movq	$0x0, -0x58(%rbp)
140ae3cec:     	movq	$0x0, 0x17f8(%rbp)
140ae3cf7:     	movq	$0x8, 0x1800(%rbp)
140ae3d02:     	movq	$0x0, 0x1808(%rbp)
140ae3d0d:     	movq	$0x10, 0x1810(%rbp)
140ae3d18:     	movq	$0x10, 0x1828(%rbp)
140ae3d23:     	movq	$0x10, 0x1840(%rbp)
140ae3d2e:     	movq	$0x10, 0x1858(%rbp)
140ae3d39:     	movq	$0x10, 0x1870(%rbp)
140ae3d44:     	movq	%r14, 0x1890(%rbp)
140ae3d4b:     	leaq	0x1ffee8e(%rip), %rax   # 0x142ae2be0
140ae3d52:     	movq	%rax, 0x1898(%rbp)
140ae3d59:     	movq	$0x3a, 0x18a0(%rbp)
140ae3d64:     	movq	$0x10, 0x18a8(%rbp)
140ae3d6f:     	movq	$0x10, 0x18c0(%rbp)
140ae3d7a:     	movq	$0x10, 0x18d8(%rbp)
140ae3d85:     	movq	$0x10, 0x18f0(%rbp)
140ae3d90:     	movq	$0x10, 0x1908(%rbp)
140ae3d9b:     	movl	$0x0, 0x1928(%rbp)
140ae3da5:     	movw	$0x1, 0x192c(%rbp)
140ae3dae:     	movl	$0x0, 0x1930(%rbp)
140ae3db8:     	movzbl	0x2b50e4a(%rip), %eax   # 0x143634c09
140ae3dbf:     	movl	$0x1990, %ecx           # imm = 0x1990
140ae3dc4:     	movl	$0x8, %edx
140ae3dc9:     	callq	0x140613c10 <.text+0x612c10>
140ae3dce:     	testq	%rax, %rax
140ae3dd1:     	je	0x140ae42f7 <.text+0xae32f7>
140ae3dd7:     	movq	%rax, %rdi
140ae3dda:     	leaq	-0x58(%rbp), %rdx
140ae3dde:     	movl	$0x1990, %r8d           # imm = 0x1990
140ae3de4:     	movq	%rax, %rcx
140ae3de7:     	callq	0x142923c00 <.text+0x2922c00>
140ae3dec:     	leaq	0x1ffeb5d(%rip), %r8    # 0x142ae2950
140ae3df3:     	leaq	0x1aa0(%rbp), %rcx
140ae3dfa:     	movq	%rdi, %rdx
140ae3dfd:     	callq	0x140ce4b10 <.text+0xce3b10>
140ae3e02:     	leaq	0x1a30(%rbp), %rcx
140ae3e09:     	callq	0x14176a170 <.text+0x1769170>
140ae3e0e:     	leaq	0x19c0(%rbp), %rcx
140ae3e15:     	callq	0x1408c4f20 <.text+0x8c3f20>
140ae3e1a:     	leaq	0x1950(%rbp), %rcx
140ae3e21:     	callq	0x140abf2c0 <.text+0xabe2c0>
140ae3e26:     	leaq	0x1cd0(%rbp), %rcx
140ae3e2d:     	callq	0x140a9eec0 <.text+0xa9dec0>
140ae3e32:     	leaq	-0x58(%rbp), %rcx
140ae3e36:     	callq	0x140abf490 <.text+0xabe490>
140ae3e3b:     	movups	0x1cc0(%rbp), %xmm0
140ae3e42:     	movq	0x1d40(%rbp), %rax
140ae3e49:     	movups	%xmm0, 0x60(%rax)
140ae3e4d:     	movups	0x1cb0(%rbp), %xmm0
140ae3e54:     	movups	%xmm0, 0x50(%rax)
140ae3e58:     	movups	0x1ca0(%rbp), %xmm0
140ae3e5f:     	movups	%xmm0, 0x40(%rax)
140ae3e63:     	movups	0x1c60(%rbp), %xmm0
140ae3e6a:     	movups	0x1c70(%rbp), %xmm1
140ae3e71:     	movups	0x1c80(%rbp), %xmm2
140ae3e78:     	movups	0x1c90(%rbp), %xmm3
140ae3e7f:     	movups	%xmm3, 0x30(%rax)
140ae3e83:     	movups	%xmm2, 0x20(%rax)
140ae3e87:     	movups	%xmm1, 0x10(%rax)
140ae3e8b:     	movups	%xmm0, (%rax)
140ae3e8e:     	movups	0x1bf0(%rbp), %xmm0
140ae3e95:     	movups	0x1c00(%rbp), %xmm1
140ae3e9c:     	movups	0x1c10(%rbp), %xmm2
140ae3ea3:     	movups	0x1c20(%rbp), %xmm3
140ae3eaa:     	movups	%xmm0, 0x70(%rax)
140ae3eae:     	movups	%xmm1, 0x80(%rax)
140ae3eb5:     	movups	%xmm2, 0x90(%rax)
140ae3ebc:     	movups	%xmm3, 0xa0(%rax)
140ae3ec3:     	movups	0x1c30(%rbp), %xmm0
140ae3eca:     	movups	%xmm0, 0xb0(%rax)
140ae3ed1:     	movups	0x1c40(%rbp), %xmm0
140ae3ed8:     	movups	%xmm0, 0xc0(%rax)
140ae3edf:     	movups	0x1c50(%rbp), %xmm0
140ae3ee6:     	movups	%xmm0, 0xd0(%rax)
140ae3eed:     	movups	0x1be0(%rbp), %xmm0
140ae3ef4:     	movups	%xmm0, 0x140(%rax)
140ae3efb:     	movups	0x1bd0(%rbp), %xmm0
140ae3f02:     	movups	%xmm0, 0x130(%rax)
140ae3f09:     	movups	0x1bc0(%rbp), %xmm0
140ae3f10:     	movups	%xmm0, 0x120(%rax)
140ae3f17:     	movups	0x1b80(%rbp), %xmm0
140ae3f1e:     	movups	0x1b90(%rbp), %xmm1
140ae3f25:     	movups	0x1ba0(%rbp), %xmm2
140ae3f2c:     	movups	0x1bb0(%rbp), %xmm3
140ae3f33:     	movups	%xmm3, 0x110(%rax)
140ae3f3a:     	movups	%xmm2, 0x100(%rax)
140ae3f41:     	movups	%xmm1, 0xf0(%rax)
140ae3f48:     	movups	%xmm0, 0xe0(%rax)
140ae3f4f:     	movups	0x1b10(%rbp), %xmm0
140ae3f56:     	movups	0x1b20(%rbp), %xmm1
140ae3f5d:     	movups	0x1b30(%rbp), %xmm2
140ae3f64:     	movups	0x1b40(%rbp), %xmm3
140ae3f6b:     	movups	%xmm0, 0x150(%rax)
140ae3f72:     	movups	%xmm1, 0x160(%rax)
140ae3f79:     	movups	%xmm2, 0x170(%rax)
140ae3f80:     	movups	%xmm3, 0x180(%rax)
140ae3f87:     	movups	0x1b50(%rbp), %xmm0
140ae3f8e:     	movups	%xmm0, 0x190(%rax)
140ae3f95:     	movups	0x1b60(%rbp), %xmm0
140ae3f9c:     	movups	%xmm0, 0x1a0(%rax)
140ae3fa3:     	movups	0x1b70(%rbp), %xmm0
140ae3faa:     	movups	%xmm0, 0x1b0(%rax)
140ae3fb1:     	movups	0x1aa0(%rbp), %xmm0
140ae3fb8:     	movups	0x1ab0(%rbp), %xmm1
140ae3fbf:     	movups	0x1ac0(%rbp), %xmm2
140ae3fc6:     	movups	0x1ad0(%rbp), %xmm3
140ae3fcd:     	movups	%xmm0, 0x1c0(%rax)
140ae3fd4:     	movups	%xmm1, 0x1d0(%rax)
140ae3fdb:     	movups	%xmm2, 0x1e0(%rax)
140ae3fe2:     	movups	%xmm3, 0x1f0(%rax)
140ae3fe9:     	movups	0x1ae0(%rbp), %xmm0
140ae3ff0:     	movups	%xmm0, 0x200(%rax)
140ae3ff7:     	movups	0x1b00(%rbp), %xmm0
140ae3ffe:     	movups	%xmm0, 0x220(%rax)
140ae4005:     	movups	0x1af0(%rbp), %xmm0
140ae400c:     	movups	%xmm0, 0x210(%rax)
140ae4013:     	movups	0x1a90(%rbp), %xmm0
140ae401a:     	movups	%xmm0, 0x290(%rax)
140ae4021:     	movups	0x1a80(%rbp), %xmm0
140ae4028:     	movups	%xmm0, 0x280(%rax)
140ae402f:     	movups	0x1a70(%rbp), %xmm0
140ae4036:     	movups	%xmm0, 0x270(%rax)
140ae403d:     	movups	0x1a30(%rbp), %xmm0
140ae4044:     	movups	0x1a40(%rbp), %xmm1
140ae404b:     	movups	0x1a50(%rbp), %xmm2
140ae4052:     	movups	0x1a60(%rbp), %xmm3
140ae4059:     	movups	%xmm3, 0x260(%rax)
140ae4060:     	movups	%xmm2, 0x250(%rax)
140ae4067:     	movups	%xmm1, 0x240(%rax)
140ae406e:     	movups	%xmm0, 0x230(%rax)
140ae4075:     	movups	0x19c0(%rbp), %xmm0
140ae407c:     	movups	0x19d0(%rbp), %xmm1
140ae4083:     	movups	0x19e0(%rbp), %xmm2
140ae408a:     	movups	0x19f0(%rbp), %xmm3
140ae4091:     	movups	%xmm0, 0x2a0(%rax)
140ae4098:     	movups	%xmm1, 0x2b0(%rax)
140ae409f:     	movups	%xmm2, 0x2c0(%rax)
140ae40a6:     	movups	%xmm3, 0x2d0(%rax)
140ae40ad:     	movups	0x1a00(%rbp), %xmm0
140ae40b4:     	movups	%xmm0, 0x2e0(%rax)
140ae40bb:     	movups	0x1a10(%rbp), %xmm0
140ae40c2:     	movups	%xmm0, 0x2f0(%rax)
140ae40c9:     	movups	0x1a20(%rbp), %xmm0
140ae40d0:     	movups	%xmm0, 0x300(%rax)
140ae40d7:     	movups	0x1950(%rbp), %xmm0
140ae40de:     	movups	0x1960(%rbp), %xmm1
140ae40e5:     	movups	0x1970(%rbp), %xmm2
140ae40ec:     	movups	0x1980(%rbp), %xmm3
140ae40f3:     	movups	%xmm0, 0x310(%rax)
140ae40fa:     	movups	%xmm1, 0x320(%rax)
140ae4101:     	movups	%xmm2, 0x330(%rax)
140ae4108:     	movups	%xmm3, 0x340(%rax)
140ae410f:     	movups	0x1990(%rbp), %xmm0
140ae4116:     	movups	%xmm0, 0x350(%rax)
140ae411d:     	movups	0x19a0(%rbp), %xmm0
140ae4124:     	movups	%xmm0, 0x360(%rax)
140ae412b:     	movups	0x19b0(%rbp), %xmm0
140ae4132:     	movups	%xmm0, 0x370(%rax)
140ae4139:     	movups	0x1cd0(%rbp), %xmm0
140ae4140:     	movups	0x1ce0(%rbp), %xmm1
140ae4147:     	movups	0x1cf0(%rbp), %xmm2
140ae414e:     	movups	0x1d00(%rbp), %xmm3
140ae4155:     	movups	%xmm0, 0x380(%rax)
140ae415c:     	movups	%xmm1, 0x390(%rax)
140ae4163:     	movups	%xmm2, 0x3a0(%rax)
140ae416a:     	movups	%xmm3, 0x3b0(%rax)
140ae4171:     	movups	0x1d10(%rbp), %xmm0
140ae4178:     	movups	%xmm0, 0x3c0(%rax)
140ae417f:     	movups	0x1d20(%rbp), %xmm0
140ae4186:     	movups	%xmm0, 0x3d0(%rax)
140ae418d:     	movups	0x1d30(%rbp), %xmm0
140ae4194:     	movups	%xmm0, 0x3e0(%rax)
140ae419b:     	movups	-0x58(%rbp), %xmm0
140ae419f:     	movups	-0x48(%rbp), %xmm1
140ae41a3:     	movups	-0x38(%rbp), %xmm2
140ae41a7:     	movups	-0x28(%rbp), %xmm3
140ae41ab:     	movups	%xmm0, 0x3f0(%rax)
140ae41b2:     	movups	%xmm1, 0x400(%rax)
140ae41b9:     	movups	%xmm2, 0x410(%rax)
140ae41c0:     	movups	%xmm3, 0x420(%rax)
140ae41c7:     	movups	-0x18(%rbp), %xmm0
140ae41cb:     	movups	%xmm0, 0x430(%rax)
140ae41d2:     	movups	0x8(%rbp), %xmm0
140ae41d6:     	movups	%xmm0, 0x450(%rax)
140ae41dd:     	movups	-0x8(%rbp), %xmm0
140ae41e1:     	movups	%xmm0, 0x440(%rax)
140ae41e8:     	movq	$0xa, -0x50(%rbp)
140ae41f0:     	movq	%rax, -0x48(%rbp)
140ae41f4:     	movq	$0xa, -0x40(%rbp)
140ae41fc:     	movq	$0x0, -0x38(%rbp)
140ae4204:     	movq	$0x8, -0x30(%rbp)
140ae420c:     	xorps	%xmm0, %xmm0
140ae420f:     	movups	%xmm0, -0x28(%rbp)
140ae4213:     	movq	0x1ce0(%rbp), %rax
140ae421a:     	movq	%rax, -0x8(%rbp)
140ae421e:     	movups	0x1cd0(%rbp), %xmm0
140ae4225:     	movups	%xmm0, -0x18(%rbp)
140ae4229:     	movq	%r14, -0x58(%rbp)
140ae422d:     	movb	%bl, 0x1cd0(%rbp)
140ae4233:     	movq	0x2b48136(%rip), %rcx   # 0x14362c370
140ae423a:     	leaq	0x217bb1f(%rip), %r8    # 0x142c5fd60
140ae4241:     	leaq	0x1cd0(%rbp), %rdx
140ae4248:     	callq	0x140b218a0 <.text+0xb208a0>
140ae424d:     	movq	%rdx, %r8
140ae4250:     	leaq	-0x58(%rbp), %rcx
140ae4254:     	movq	%rax, %rdx
140ae4257:     	callq	0x141453000 <.text+0x1452000>
140ae425c:     	movups	0x8(%rbp), %xmm0
140ae4260:     	movups	%xmm0, 0x60(%rsi)
140ae4264:     	movups	-0x8(%rbp), %xmm0
140ae4268:     	movups	%xmm0, 0x50(%rsi)
140ae426c:     	movups	-0x18(%rbp), %xmm0
140ae4270:     	movups	%xmm0, 0x40(%rsi)
140ae4274:     	movups	-0x58(%rbp), %xmm0
140ae4278:     	movups	-0x48(%rbp), %xmm1
140ae427c:     	movups	-0x38(%rbp), %xmm2
140ae4280:     	movups	-0x28(%rbp), %xmm3
140ae4284:     	movups	%xmm3, 0x30(%rsi)
140ae4288:     	movups	%xmm2, 0x20(%rsi)
140ae428c:     	movups	%xmm1, 0x10(%rsi)
140ae4290:     	movups	%xmm0, (%rsi)
140ae4293:     	movq	%rsi, %rax
140ae4296:     	addq	$0x1dd0, %rsp           # imm = 0x1DD0
140ae429d:     	popq	%rbx
140ae429e:     	popq	%rdi
140ae429f:     	popq	%rsi
140ae42a0:     	popq	%r14
140ae42a2:     	popq	%rbp
140ae42a3:     	retq
140ae42a4:     	movl	$0x8, %ecx
140ae42a9:     	movl	$0x460, %edx            # imm = 0x460
140ae42ae:     	callq	0x1428d9000 <.text+0x28d8000>
140ae42b3:     	movl	$0x8, %ecx
140ae42b8:     	movl	$0x19a8, %edx           # imm = 0x19A8
140ae42bd:     	callq	0x1428d9000 <.text+0x28d8000>
140ae42c2:     	jmp	0x140ae4306 <.text+0xae3306>
140ae42c4:     	movl	$0x8, %ecx
140ae42c9:     	movl	$0x19a8, %edx           # imm = 0x19A8
140ae42ce:     	callq	0x1428d9000 <.text+0x28d8000>
140ae42d3:     	jmp	0x140ae4306 <.text+0xae3306>
140ae42d5:     	movl	$0x8, %ecx
140ae42da:     	movl	$0x19a8, %edx           # imm = 0x19A8
140ae42df:     	callq	0x1428d9000 <.text+0x28d8000>
140ae42e4:     	jmp	0x140ae4306 <.text+0xae3306>
140ae42e6:     	movl	$0x8, %ecx
140ae42eb:     	movl	$0x19a8, %edx           # imm = 0x19A8
140ae42f0:     	callq	0x1428d9000 <.text+0x28d8000>
140ae42f5:     	jmp	0x140ae4306 <.text+0xae3306>
140ae42f7:     	movl	$0x8, %ecx
140ae42fc:     	movl	$0x1990, %edx           # imm = 0x1990
140ae4301:     	callq	0x1428d9000 <.text+0x28d8000>
140ae4306:     	ud2
