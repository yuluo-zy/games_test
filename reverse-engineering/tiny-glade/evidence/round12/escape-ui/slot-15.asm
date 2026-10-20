
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404ff7e0:     	pushq	%r15
1404ff7e2:     	pushq	%r14
1404ff7e4:     	pushq	%r13
1404ff7e6:     	pushq	%r12
1404ff7e8:     	pushq	%rsi
1404ff7e9:     	pushq	%rdi
1404ff7ea:     	pushq	%rbx
1404ff7eb:     	subq	$0xf0, %rsp
1404ff7f2:     	movq	%rcx, %rdi
1404ff7f5:     	movq	%rdx, 0x60(%rsp)
1404ff7fa:     	movq	%r8, 0x68(%rsp)
1404ff7ff:     	leaq	0x10(%rcx), %rsi
1404ff803:     	movq	%rsi, %rcx
1404ff806:     	callq	0x1406868b0 <.text+0x6858b0>
1404ff80b:     	movq	%rax, %r15
1404ff80e:     	leaq	0x60(%rsp), %r14
1404ff813:     	movq	%r14, %rcx
1404ff816:     	callq	0x140683c10 <.text+0x682c10>
1404ff81b:     	movq	%rax, 0x40(%rsp)
1404ff820:     	leaq	0x40(%rsp), %rbx
1404ff825:     	movq	%rbx, %rcx
1404ff828:     	callq	0x1400885d0 <.text+0x875d0>
1404ff82d:     	movq	%rax, 0x70(%rsp)
1404ff832:     	movabsq	$0x41223169ff28813b, %r12 # imm = 0x41223169FF28813B
1404ff83c:     	movq	%r12, 0x78(%rsp)
1404ff841:     	movabsq	$-0x58648d975d569727, %r13 # imm = 0xA79B7268A2A968D9
1404ff84b:     	movq	%r13, 0x80(%rsp)
1404ff853:     	leaq	0x70(%rsp), %rcx
1404ff858:     	callq	0x1405bdf20 <.text+0x5bcf20>
1404ff85d:     	movq	%r15, 0x70(%rsp)
1404ff862:     	movq	%rax, 0x78(%rsp)
1404ff867:     	leaq	0x70(%rsp), %rcx
1404ff86c:     	callq	0x1405bd910 <.text+0x5bc910>
1404ff871:     	movq	%rax, %r15
1404ff874:     	movq	%rsi, %rcx
1404ff877:     	movq	%rax, %rdx
1404ff87a:     	callq	0x140686450 <.text+0x685450>
1404ff87f:     	testb	%al, %al
1404ff881:     	je	0x1404ff89b <.text+0x4fe89b>
1404ff883:     	leaq	0x40(%rsp), %rdi
1404ff888:     	movq	%rdi, %rcx
1404ff88b:     	movq	%rsi, %rdx
1404ff88e:     	movq	%r15, %r8
1404ff891:     	callq	0x140686530 <.text+0x685530>
1404ff896:     	jmp	0x1404ff9e7 <.text+0x4fe9e7>
1404ff89b:     	movq	0x312cb06(%rip), %rax   # 0x14362c3a8
1404ff8a2:     	movq	(%rax), %rax
1404ff8a5:     	testq	%rax, %rax
1404ff8a8:     	je	0x1404ff98c <.text+0x4fe98c>
1404ff8ae:     	movq	%rdi, 0x40(%rsp)
1404ff8b3:     	leaq	0x33fe6(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
1404ff8ba:     	movq	%rax, 0x48(%rsp)
1404ff8bf:     	movq	%r14, 0x50(%rsp)
1404ff8c4:     	movq	%rax, 0x58(%rsp)
1404ff8c9:     	leaq	0x24ee488(%rip), %rcx   # 0x1429edd58
1404ff8d0:     	callq	0x14067dd80 <.text+0x67cd80>
1404ff8d5:     	movups	(%rax), %xmm0
1404ff8d8:     	movl	0x10(%rax), %eax
1404ff8db:     	movq	$0x1, 0xa0(%rsp)
1404ff8e7:     	leaq	0x24ee43a(%rip), %rcx   # 0x1429edd28
1404ff8ee:     	movq	%rcx, 0xa8(%rsp)
1404ff8f6:     	movq	$0x14, 0xb0(%rsp)
1404ff902:     	leaq	0x24ee38f(%rip), %rdx   # 0x1429edc98
1404ff909:     	movq	%rdx, 0xc0(%rsp)
1404ff911:     	movq	$0x2, 0xc8(%rsp)
1404ff91d:     	movq	%rbx, 0xd0(%rsp)
1404ff925:     	movq	$0x2, 0xd8(%rsp)
1404ff931:     	movq	$0x0, 0xe0(%rsp)
1404ff93d:     	movq	$0x0, 0x70(%rsp)
1404ff946:     	movq	%rcx, 0x78(%rsp)
1404ff94b:     	movq	$0x14, 0x80(%rsp)
1404ff957:     	movq	$0x0, 0x88(%rsp)
1404ff963:     	movups	%xmm0, 0x90(%rsp)
1404ff96b:     	movl	$0x1, 0xb8(%rsp)
1404ff976:     	movl	%eax, 0xbc(%rsp)
1404ff97d:     	leaq	0x3f(%rsp), %rcx
1404ff982:     	leaq	0x70(%rsp), %rdx
1404ff987:     	callq	0x140d4a330 <.text+0xd49330>
1404ff98c:     	movups	0x60(%rsp), %xmm0
1404ff991:     	movq	0x18(%rdi), %rdi
1404ff995:     	movups	%xmm0, 0x40(%rsp)
1404ff99a:     	leaq	0x40(%rsp), %rcx
1404ff99f:     	callq	0x140683c10 <.text+0x682c10>
1404ff9a4:     	movq	%rax, 0x70(%rsp)
1404ff9a9:     	movq	%r12, 0x78(%rsp)
1404ff9ae:     	movq	%r13, 0x80(%rsp)
1404ff9b6:     	leaq	0x70(%rsp), %rcx
1404ff9bb:     	callq	0x1405bdf20 <.text+0x5bcf20>
1404ff9c0:     	movq	%rdi, 0x70(%rsp)
1404ff9c5:     	movq	%rax, 0x78(%rsp)
1404ff9ca:     	leaq	0x70(%rsp), %rcx
1404ff9cf:     	callq	0x1405bd910 <.text+0x5bc910>
1404ff9d4:     	leaq	0x40(%rsp), %rdi
1404ff9d9:     	movq	%rdi, %rcx
1404ff9dc:     	movq	%rsi, %rdx
1404ff9df:     	movq	%rax, %r8
1404ff9e2:     	callq	0x1406861b0 <.text+0x6851b0>
1404ff9e7:     	leaq	0x24ee4b1(%rip), %rax   # 0x1429ede9f
1404ff9ee:     	movq	%rax, 0x20(%rsp)
1404ff9f3:     	movl	$0x43960000, 0x30(%rsp) # imm = 0x43960000
1404ff9fb:     	movq	$0x14, 0x28(%rsp)
1404ffa04:     	leaq	0x24ee4c1(%rip), %r8    # 0x1429edecc
1404ffa0b:     	leaq	0x70(%rsp), %rcx
1404ffa10:     	movl	$0x8, %r9d
1404ffa16:     	movq	%rdi, %rdx
1404ffa19:     	callq	0x14032e0b0 <.text+0x32d0b0>
1404ffa1e:     	nop
1404ffa1f:     	addq	$0xf0, %rsp
1404ffa26:     	popq	%rbx
1404ffa27:     	popq	%rdi
1404ffa28:     	popq	%rsi
1404ffa29:     	popq	%r12
1404ffa2b:     	popq	%r13
1404ffa2d:     	popq	%r14
1404ffa2f:     	popq	%r15
1404ffa31:     	retq
