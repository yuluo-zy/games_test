
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14033def0:     	pushq	%rbp
14033def1:     	pushq	%rsi
14033def2:     	pushq	%rdi
14033def3:     	pushq	%rbx
14033def4:     	subq	$0x588, %rsp            # imm = 0x588
14033defb:     	leaq	0x80(%rsp), %rbp
14033df03:     	movq	$-0x2, 0x500(%rbp)
14033df0e:     	movq	%rcx, %rsi
14033df11:     	movzbl	0x32f6cf1(%rip), %eax   # 0x143634c09
14033df18:     	movl	$0x3f0, %ecx            # imm = 0x3F0
14033df1d:     	movl	$0x8, %edx
14033df22:     	callq	0x140613c10 <.text+0x612c10>
14033df27:     	testq	%rax, %rax
14033df2a:     	je	0x14033e686 <.text+0x33d686>
14033df30:     	movq	%rax, 0x4f8(%rbp)
14033df37:     	xorps	%xmm0, %xmm0
14033df3a:     	movups	%xmm0, 0x340(%rbp)
14033df41:     	movups	%xmm0, 0x358(%rbp)
14033df48:     	movups	%xmm0, 0x370(%rbp)
14033df4f:     	movups	%xmm0, 0x388(%rbp)
14033df56:     	movups	%xmm0, 0x3a0(%rbp)
14033df5d:     	movl	$0x0, 0x3b0(%rbp)
14033df67:     	movups	%xmm0, 0x3d8(%rbp)
14033df6e:     	movups	%xmm0, 0x3f0(%rbp)
14033df75:     	movups	%xmm0, 0x408(%rbp)
14033df7c:     	movups	%xmm0, 0x420(%rbp)
14033df83:     	movups	%xmm0, 0x438(%rbp)
14033df8a:     	movl	$0x0, 0x448(%rbp)
14033df94:     	movq	$0x0, 0x320(%rbp)
14033df9f:     	movq	$0x8, 0x328(%rbp)
14033dfaa:     	movq	$0x0, 0x330(%rbp)
14033dfb5:     	movq	$0x10, 0x338(%rbp)
14033dfc0:     	movq	$0x10, 0x350(%rbp)
14033dfcb:     	movq	$0x10, 0x368(%rbp)
14033dfd6:     	movq	$0x10, 0x380(%rbp)
14033dfe1:     	movq	$0x10, 0x398(%rbp)
14033dfec:     	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
14033dff6:     	movq	%rbx, 0x3b8(%rbp)
14033dffd:     	leaq	0x2683cb3(%rip), %rax   # 0x1429c1cb7
14033e004:     	movq	%rax, 0x3c0(%rbp)
14033e00b:     	movq	$0x47, 0x3c8(%rbp)
14033e016:     	movq	$0x10, 0x3d0(%rbp)
14033e021:     	movq	$0x10, 0x3e8(%rbp)
14033e02c:     	movq	$0x10, 0x400(%rbp)
14033e037:     	movq	$0x10, 0x418(%rbp)
14033e042:     	movq	$0x10, 0x430(%rbp)
14033e04d:     	movl	$0x0, 0x450(%rbp)
14033e057:     	movw	$0x1, 0x454(%rbp)
14033e060:     	movq	%rbx, 0x4b8(%rbp)
14033e067:     	movl	$0x0, 0x4f0(%rbp)
14033e071:     	movzbl	0x32f6b91(%rip), %eax   # 0x143634c09
14033e078:     	movl	$0x1d8, %ecx            # imm = 0x1D8
14033e07d:     	movl	$0x8, %edx
14033e082:     	callq	0x140613c10 <.text+0x612c10>
14033e087:     	testq	%rax, %rax
14033e08a:     	je	0x14033e695 <.text+0x33d695>
14033e090:     	movq	%rax, %rdi
14033e093:     	leaq	0x320(%rbp), %rdx
14033e09a:     	movl	$0x1d8, %r8d            # imm = 0x1D8
14033e0a0:     	movq	%rax, %rcx
14033e0a3:     	callq	0x142923c00 <.text+0x2922c00>
14033e0a8:     	leaq	0x2683af1(%rip), %r8    # 0x1429c1ba0
14033e0af:     	leaq	0x240(%rbp), %rcx
14033e0b6:     	movq	%rdi, %rdx
14033e0b9:     	callq	0x140ce4b10 <.text+0xce3b10>
14033e0be:     	xorps	%xmm0, %xmm0
14033e0c1:     	movups	%xmm0, 0x340(%rbp)
14033e0c8:     	movups	%xmm0, 0x358(%rbp)
14033e0cf:     	movups	%xmm0, 0x370(%rbp)
14033e0d6:     	movups	%xmm0, 0x388(%rbp)
14033e0dd:     	movups	%xmm0, 0x3a0(%rbp)
14033e0e4:     	movl	$0x0, 0x3b0(%rbp)
14033e0ee:     	movups	%xmm0, 0x3d8(%rbp)
14033e0f5:     	movups	%xmm0, 0x3f0(%rbp)
14033e0fc:     	movups	%xmm0, 0x408(%rbp)
14033e103:     	movups	%xmm0, 0x420(%rbp)
14033e10a:     	movups	%xmm0, 0x438(%rbp)
14033e111:     	movl	$0x0, 0x448(%rbp)
14033e11b:     	movq	$0x0, 0x320(%rbp)
14033e126:     	movq	$0x8, 0x328(%rbp)
14033e131:     	movq	$0x0, 0x330(%rbp)
14033e13c:     	movq	$0x10, 0x338(%rbp)
14033e147:     	movq	$0x10, 0x350(%rbp)
14033e152:     	movq	$0x10, 0x368(%rbp)
14033e15d:     	movq	$0x10, 0x380(%rbp)
14033e168:     	movq	$0x10, 0x398(%rbp)
14033e173:     	movq	%rbx, 0x3b8(%rbp)
14033e17a:     	leaq	0x2683af4(%rip), %rax   # 0x1429c1c75
14033e181:     	movq	%rax, 0x3c0(%rbp)
14033e188:     	movq	$0x42, 0x3c8(%rbp)
14033e193:     	movq	$0x10, 0x3d0(%rbp)
14033e19e:     	movq	$0x10, 0x3e8(%rbp)
14033e1a9:     	movq	$0x10, 0x400(%rbp)
14033e1b4:     	movq	$0x10, 0x418(%rbp)
14033e1bf:     	movq	$0x10, 0x430(%rbp)
14033e1ca:     	movl	$0x0, 0x450(%rbp)
14033e1d4:     	movw	$0x1, 0x454(%rbp)
14033e1dd:     	movq	%rbx, 0x458(%rbp)
14033e1e4:     	movl	$0x0, 0x4e0(%rbp)
14033e1ee:     	movzbl	0x32f6a14(%rip), %eax   # 0x143634c09
14033e1f5:     	movl	$0x1c8, %ecx            # imm = 0x1C8
14033e1fa:     	movl	$0x8, %edx
14033e1ff:     	callq	0x140613c10 <.text+0x612c10>
14033e204:     	testq	%rax, %rax
14033e207:     	je	0x14033e6a6 <.text+0x33d6a6>
14033e20d:     	movq	%rax, %rdi
14033e210:     	leaq	0x320(%rbp), %rdx
14033e217:     	movl	$0x1c8, %r8d            # imm = 0x1C8
14033e21d:     	movq	%rax, %rcx
14033e220:     	callq	0x142923c00 <.text+0x2922c00>
14033e225:     	leaq	0x26838bc(%rip), %r8    # 0x1429c1ae8
14033e22c:     	leaq	0x1d0(%rbp), %rcx
14033e233:     	movq	%rdi, %rdx
14033e236:     	callq	0x140ce4b10 <.text+0xce3b10>
14033e23b:     	leaq	0x160(%rbp), %rcx
14033e242:     	callq	0x14033a430 <.text+0x339430>
14033e247:     	leaq	0xf0(%rbp), %rcx
14033e24e:     	callq	0x140342790 <.text+0x341790>
14033e253:     	leaq	0x80(%rbp), %rcx
14033e25a:     	callq	0x14033a260 <.text+0x339260>
14033e25f:     	leaq	0x10(%rbp), %rcx
14033e263:     	callq	0x140002c90 <.text+0x1c90>
14033e268:     	leaq	-0x60(%rbp), %rcx
14033e26c:     	callq	0x14031a8a0 <.text+0x3198a0>
14033e271:     	leaq	0x2b0(%rbp), %rcx
14033e278:     	callq	0x1402ad500 <.text+0x2ac500>
14033e27d:     	leaq	0x320(%rbp), %rcx
14033e284:     	callq	0x14050f480 <.text+0x50e480>
14033e289:     	movups	0x2a0(%rbp), %xmm0
14033e290:     	movq	0x4f8(%rbp), %rax
14033e297:     	movups	%xmm0, 0x60(%rax)
14033e29b:     	movups	0x290(%rbp), %xmm0
14033e2a2:     	movups	%xmm0, 0x50(%rax)
14033e2a6:     	movups	0x280(%rbp), %xmm0
14033e2ad:     	movups	%xmm0, 0x40(%rax)
14033e2b1:     	movups	0x240(%rbp), %xmm0
14033e2b8:     	movups	0x250(%rbp), %xmm1
14033e2bf:     	movups	0x260(%rbp), %xmm2
